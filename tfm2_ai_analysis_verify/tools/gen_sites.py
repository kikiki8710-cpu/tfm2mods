"""임계점 트레이서 표 생성기 (v0.4.0).

함수 아틀라스(0.6.4)의 수치 기록마다 붙은 "현행 소비 위치" 명령을 실제 exe 에서 디스어셈블해
 1) 정적 대조: 기록된 값이 그 명령(또는 그 명령이 읽는 .rdata 칸)에 정말 들어 있는가
 2) 트레이스 지점: 런타임에 INT3 를 걸 명령(비교 명령)과 비교 방향(부호/포함 여부)
을 만든다. 게임/모드 파일은 바꾸지 않는다. 출력:
  tools/sites/static_audit.json   기록×소비위치 정적 대조 전수
  tools/sites/site_catalog.json   트레이스 지점 전수(지점 id ↔ 기록 id)
  src/sites_generated.rs          모드에 들어가는 지점 표(선택분)
사용: python -B -X utf8 tools/gen_sites.py [--atlas DIR] [--exe PATH] [--select all|semantic|threshold]
"""
import argparse, csv, hashlib, json, os, re, sys, collections
import pefile, capstone
from capstone import x86_const as X

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
DEF_ATLAS = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_ai_adjust\function_atlas_064_20261018"
DEF_EXE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
BASELINE_SHA = "a1d05045405a12e2bed182daeafcad8c8a40cf29ab68ba6ea5bfd30257439582"
ENTRY_GUARD = 16          # 진입 훅(12B) 영역과 겹치지 않게 함수 첫 16B 는 트레이스 금지
MAX_SITES = 6144          # 모드 표 상한 (site_trace.rs 의 MAX_SITES 와 같아야 함)

# CONTEXT(AMD64) 정수 레지스터 번호 = Rax..R15 순서(0..15). Rip 는 16.
GPR = {}
for i, names in enumerate([
    ("rax", "eax", "ax", "al"), ("rcx", "ecx", "cx", "cl"), ("rdx", "edx", "dx", "dl"), ("rbx", "ebx", "bx", "bl"),
    ("rsp", "esp", "sp", "spl"), ("rbp", "ebp", "bp", "bpl"), ("rsi", "esi", "si", "sil"), ("rdi", "edi", "di", "dil"),
    ("r8", "r8d", "r8w", "r8b"), ("r9", "r9d", "r9w", "r9b"), ("r10", "r10d", "r10w", "r10b"), ("r11", "r11d", "r11w", "r11b"),
    ("r12", "r12d", "r12w", "r12b"), ("r13", "r13d", "r13w", "r13b"), ("r14", "r14d", "r14w", "r14b"), ("r15", "r15d", "r15w", "r15b")]):
    for n in names:
        GPR[n] = i
HIGH8 = {"ah": 0, "ch": 1, "dh": 2, "bh": 3}  # 상위 8비트 레지스터 (shift 8)

# 플래그 소비 명령의 조건 → (부호, 의미). 의미는 "lhs ? rhs" 가 참일 때 조건 성립.
CC = {
    "b": ("u", "<"), "c": ("u", "<"), "nae": ("u", "<"), "ae": ("u", ">="), "nb": ("u", ">="), "nc": ("u", ">="),
    "be": ("u", "<="), "na": ("u", "<="), "a": ("u", ">"), "nbe": ("u", ">"),
    "l": ("s", "<"), "nge": ("s", "<"), "ge": ("s", ">="), "nl": ("s", ">="),
    "le": ("s", "<="), "ng": ("s", "<="), "g": ("s", ">"), "nle": ("s", ">"),
    "e": ("x", "=="), "z": ("x", "=="), "ne": ("x", "!="), "nz": ("x", "!="),
    "s": ("x", "sign"), "ns": ("x", "nosign"), "o": ("x", "ovf"), "no": ("x", "noovf"),
    "p": ("x", "par"), "np": ("x", "nopar"), "pe": ("x", "par"), "po": ("x", "nopar"),
}
OPS = {"<": 1, ">=": 2, "<=": 3, ">": 4, "==": 5, "!=": 6}
FLAG_PRODUCERS = {"cmp", "test", "sub", "and", "or", "xor", "add", "dec", "inc", "neg", "bt", "comiss", "ucomiss", "comisd", "ucomisd"}
FLAG_NEUTRAL = {"mov", "movzx", "movsx", "movsxd", "lea", "nop", "movabs", "movaps", "movups", "movdqa", "movdqu", "movq", "movd"}
CONTROL = {"jmp", "call", "ret", "retn", "int3", "int", "syscall", "ud2", "hlt", "loop", "jrcxz", "jecxz"}


def consumer_cc(insn):
    m = insn.mnemonic
    if m in ("sbb", "adc"):
        return "b"  # CF 소비 = unsigned below
    for pre in ("set", "cmov"):
        if m.startswith(pre):
            return m[len(pre):]
    if m.startswith("j") and m not in ("jmp", "jrcxz", "jecxz"):
        return m[1:]
    return None


def parse_values(text):
    """기록값 문자열 → 숫자 목록. 동적/서술형이면 []."""
    t = (text or "").strip()
    if not t:
        return []
    try:
        obj = json.loads(t)
    except Exception:
        obj = None
        if re.fullmatch(r"-?0x[0-9a-fA-F]+", t):
            return [int(t, 16)]
        if re.fullmatch(r"-?\d+", t):
            return [int(t)]
        if re.fullmatch(r"-?\d+\.\d*(e-?\d+)?", t):
            return [float(t)]
        return []
    out = []
    def walk(o):
        if isinstance(o, bool):
            return
        if isinstance(o, (int, float)):
            out.append(o)
        elif isinstance(o, str):
            out.extend(parse_values(o) if re.fullmatch(r"-?(0x)?[0-9a-fA-F.]+", o) else [])
        elif isinstance(o, list):
            for x in o:
                walk(x)
        elif isinstance(o, dict):
            for x in o.values():
                walk(x)
    walk(obj)
    return out


class Image:
    def __init__(self, path):
        self.raw = open(path, "rb").read()
        self.sha = hashlib.sha256(self.raw).hexdigest()
        self.pe = pefile.PE(data=self.raw, fast_load=True)
        self.md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
        self.md.detail = True
        self.md.skipdata = True

    def read(self, rva, n):
        try:
            return self.pe.get_data(rva, n)
        except Exception:
            return None

    def disasm(self, start, end):
        data = self.read(start, end - start)
        return list(self.md.disasm(data, start)) if data else []


def mem_desc(insn, op):
    m = op.mem
    base = insn.reg_name(m.base) if m.base else None
    index = insn.reg_name(m.index) if m.index else None
    if (base and base not in GPR and base != "rip") or (index and index not in GPR):
        return None
    if m.segment:
        return None
    return {"base": "rip" if base == "rip" else (GPR[base] if base else -1),
            "index": GPR[index] if index else -1, "scale": m.scale, "disp": m.disp, "size": op.size}


def reg_desc(insn, op):
    name = insn.reg_name(op.reg)
    if name in GPR:
        return {"reg": GPR[name], "shift": 0, "size": op.size}
    if name in HIGH8:
        return {"reg": HIGH8[name], "shift": 8, "size": 1}
    return None


def operand(insn, op):
    if op.type == X.X86_OP_REG:
        d = reg_desc(insn, op)
        return ("reg", d) if d else None
    if op.type == X.X86_OP_MEM:
        d = mem_desc(insn, op)
        return ("mem", d) if d else None
    if op.type == X.X86_OP_IMM:
        return ("imm", {"value": op.imm, "size": op.size})
    return None


def rdata_value(img, insn, op, size):
    """[rip+disp] 피연산자의 파일 값(정수, f32, f64)."""
    if op.type != X.X86_OP_MEM or op.mem.base != X.X86_REG_RIP:
        return None
    rva = insn.address + insn.size + op.mem.disp
    b = img.read(rva, size)
    if not b or len(b) < size:
        return None
    import struct
    v = {"rva": rva, "int": int.from_bytes(b[:size], "little", signed=True)}
    if size == 4:
        v["f32"] = struct.unpack("<f", b[:4])[0]
    if size == 8:
        v["f64"] = struct.unpack("<d", b[:8])[0]
    return v


def value_matches(v, insn, img):
    """기록값 v 가 명령의 즉치/변위/.rdata 값으로 존재하는지."""
    hits = []
    for op in insn.operands:
        if op.type == X.X86_OP_IMM:
            for w in (8, 16, 32, 64):
                if isinstance(v, int) and (v - op.imm) % (1 << w) == 0 and abs(v) < (1 << w):
                    hits.append("imm"); break
        elif op.type == X.X86_OP_MEM:
            if isinstance(v, int) and op.mem.base != X.X86_REG_RIP and op.mem.disp == v:
                hits.append("disp")
            rv = rdata_value(img, insn, op, op.size or 8)
            if rv:
                if isinstance(v, int) and rv["int"] == v:
                    hits.append("rdata_int")
                for k in ("f32", "f64"):
                    if k in rv and isinstance(v, (int, float)) and abs(rv[k] - v) <= 1e-6 * max(1.0, abs(v)):
                        hits.append("rdata_" + k)
    return sorted(set(hits))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--atlas", default=DEF_ATLAS)
    ap.add_argument("--exe", default=DEF_EXE)
    ap.add_argument("--select", default="semantic", choices=["semantic", "threshold", "all"])
    args = ap.parse_args()

    img = Image(args.exe)
    if img.sha != BASELINE_SHA:
        sys.exit(f"exe SHA 불일치: {img.sha} (기대 {BASELINE_SHA})")

    funcs = []
    for r in csv.DictReader(open(os.path.join(args.atlas, "functions.csv"), encoding="utf-8-sig")):
        funcs.append((int(r["RVA(0.6.4)"], 16), int(r["끝RVA"], 16), r["집합"], r["구역"]))
    funcs.sort()
    starts = [f[0] for f in funcs]
    import bisect
    def owner(rva):
        i = bisect.bisect_right(starts, rva) - 1
        if i >= 0 and funcs[i][0] <= rva <= funcs[i][1]:
            return funcs[i]
        return None

    insn_cache = {}
    def insns_of(f):
        if f[0] not in insn_cache:
            lst = img.disasm(f[0], f[1] + 1)
            insn_cache[f[0]] = (lst, {i.address: k for k, i in enumerate(lst)})
        return insn_cache[f[0]]

    params = list(csv.DictReader(open(os.path.join(args.atlas, "parameters.csv"), encoding="utf-8-sig")))
    audit = []
    trace = collections.OrderedDict()  # producer rva -> site
    for p in params:
        if p["원근거버전"] != "current064":
            continue
        kind = p["기록종류"]
        if args.select != "all" and kind != "semantic_record":
            continue
        if args.select == "threshold" and p["분류"] not in ("threshold", "coefficient"):
            continue
        values = parse_values(p["원본값(정밀도보존문자열)"])
        for s in re.findall(r"0x[0-9a-fA-F]+", p["현행 소비 위치"]):
            site = int(s, 16)
            row = {"record": p["기록ID"], "kind": kind, "class": p["분류"], "semantic": p["의미ID"],
                   "function": p["현행 함수RVA"], "site": hex(site), "values": values,
                   "meaning": p["의미"][:200]}
            f = owner(site)
            if not f:
                row["static"] = "SITE_OUTSIDE_KNOWN_FUNCTION"; audit.append(row); continue
            lst, idx = insns_of(f)
            if site not in idx:
                row["static"] = "SITE_NOT_INSTRUCTION_BOUNDARY"; audit.append(row); continue
            k = idx[site]; insn = lst[k]
            row["insn"] = f"{insn.mnemonic} {insn.op_str}"
            if not values:
                row["static"] = "NONNUMERIC_CLAIM"
            else:
                found = {str(v): value_matches(v, insn, img) for v in values}
                # 소비 명령(setcc/jcc/sbb)이면 직전 비교 명령에서도 찾는다
                prod = None
                if consumer_cc(insn):
                    for j in range(k - 1, max(-1, k - 5), -1):
                        if lst[j].mnemonic in FLAG_PRODUCERS:
                            prod = lst[j]; break
                        if lst[j].mnemonic in CONTROL or consumer_cc(lst[j]) and lst[j].mnemonic.startswith("j"):
                            break
                if prod is not None:
                    for v in values:
                        if not found[str(v)]:
                            found[str(v)] = ["producer:" + h for h in value_matches(v, prod, img)]
                row["value_hits"] = found
                n = sum(1 for v in found.values() if v)
                row["static"] = "VALUE_AT_SITE" if n == len(values) else ("PARTIAL_VALUES_AT_SITE" if n else "VALUE_NOT_AT_SITE")
            audit.append(row)

            # 트레이스 지점 결정: 소비 명령이면 직전 플래그 생산 명령, 아니면 그 명령 자체
            tk = k
            cc = consumer_cc(insn)
            if cc:
                tk = None
                for j in range(k - 1, max(-1, k - 5), -1):
                    if lst[j].mnemonic in FLAG_PRODUCERS:
                        tk = j; break
                    if lst[j].mnemonic in CONTROL or lst[j].mnemonic.startswith("j"):
                        break
                if tk is None:
                    continue
            t = lst[tk]
            if t.mnemonic in CONTROL or t.mnemonic.startswith("j") or t.address - f[0] < ENTRY_GUARD:
                continue
            if t.mnemonic in ("push", "pop") or "rsp" in t.op_str and t.mnemonic not in ("cmp", "test", "mov"):
                continue
            ent = trace.get(t.address)
            if ent is None:
                ent = {"rva": t.address, "function": hex(f[0]), "function_set": f[2], "area": f[3],
                       "insn": f"{t.mnemonic} {t.op_str}", "bytes": t.bytes.hex(), "len": t.size,
                       "records": [], "claims": []}
                # 소비 조건(바로 다음 명령)
                # 플래그를 건드리지 않는 명령(mov/lea 등)은 건너뛰고 첫 소비 명령을 찾는다
                nxt, ncc = None, None
                for j in range(tk + 1, min(len(lst), tk + 6)):
                    cand = lst[j]
                    c = consumer_cc(cand)
                    if c:
                        nxt, ncc = cand, c; break
                    if cand.mnemonic in FLAG_NEUTRAL:
                        continue
                    nxt = cand; break
                ent["consumer"] = f"{nxt.mnemonic} {nxt.op_str}" if nxt else None
                ent["cc"] = ncc
                ops = [operand(t, o) for o in t.operands]
                ent["mnemonic"] = t.mnemonic
                ent["ops"] = ops
                ent["riprel"] = any(o.type == X.X86_OP_MEM and o.mem.base == X.X86_REG_RIP for o in t.operands)
                trace[t.address] = ent
            ent["records"].append(p["기록ID"])
            ent["claims"].append({"record": p["기록ID"], "semantic": p["의미ID"], "class": p["분류"], "values": values,
                                  "site": hex(site), "static": row["static"], "meaning": p["의미"][:160]})

    # 트레이스 종류 판정
    sites = []
    for ent in trace.values():
        ops = ent["ops"]
        m = ent["mnemonic"]
        if any(o is None for o in ops) or m.startswith(("comis", "ucomis")) or any(
                o and o[0] == "reg" and o[1]["size"] > 8 for o in ops):
            ent["trace_kind"] = "UNSUPPORTED_OPERAND"
        elif m == "cmp" and len(ops) == 2:
            ent["trace_kind"] = "CMP_IMM" if ops[1][0] == "imm" else "CMP_DYN"
        elif m == "test" and len(ops) == 2:
            ent["trace_kind"] = "TEST"
        else:
            ent["trace_kind"] = "VALUE"  # 첫 번째 읽기 피연산자 값 분포만
        if ent["cc"] and ent["cc"] in CC:
            sg, rel = CC[ent["cc"]]
            ent["signed"] = sg; ent["rel"] = rel
        else:
            ent["signed"] = "x"; ent["rel"] = None
        sites.append(ent)

    out_dir = os.path.join(HERE, "sites"); os.makedirs(out_dir, exist_ok=True)
    # v0.4.0 은 비교(CMP/TEST)만 트레이스한다. VALUE(곱셈·나눗셈 계수 등)는 목록만 남긴다.
    traceable = [s for s in sites if s["trace_kind"] in ("CMP_IMM", "CMP_DYN", "TEST") and not s["riprel"]]
    value_sites = [s for s in sites if s["trace_kind"] == "VALUE"]
    traceable.sort(key=lambda s: s["rva"])
    for i, s in enumerate(traceable):
        s["id"] = i

    stat = collections.Counter(a["static"] for a in audit)
    kinds = collections.Counter(s["trace_kind"] for s in sites)
    meta = {"exe_sha256": img.sha, "atlas": args.atlas, "select": args.select,
            "records_sites": len(audit), "static": dict(stat), "trace_points": len(sites),
            "trace_kinds": dict(kinds), "traceable": len(traceable),
            "riprel": sum(1 for s in traceable if s["riprel"])}
    json.dump({"meta": meta, "audit": audit}, open(os.path.join(out_dir, "static_audit.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    json.dump({"meta": meta, "sites": traceable, "unsupported": [s for s in sites if s["trace_kind"] == "UNSUPPORTED_OPERAND"],
               "value_sites_untraced": value_sites},
              open(os.path.join(out_dir, "site_catalog.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    if len(traceable) > MAX_SITES:
        sys.exit(f"지점 {len(traceable)} > MAX_SITES {MAX_SITES}")
    write_rust(traceable, os.path.join(ROOT, "src", "sites_generated.rs"), img.sha)
    print(json.dumps(meta, ensure_ascii=False, indent=1))


def rust_op(o):
    if o is None:
        return "Op::None"
    k, d = o
    if k == "reg":
        return f"Op::Reg{{reg:{d['reg']},shift:{d['shift']},size:{d['size']}}}"
    if k == "imm":
        return f"Op::Imm{{value:{d['value']}i64 as u64,size:{d['size']}}}"
    base = 16 if d["base"] == "rip" else d["base"]  # 16 = RIP, -1 = 없음
    return f"Op::Mem{{base:{base},index:{d['index']},scale:{d['scale']},disp:{d['disp']},size:{d['size']}}}"


def write_rust(sites, path, sha):
    kind = {"CMP_IMM": "Kind::CmpImm", "CMP_DYN": "Kind::CmpDyn", "TEST": "Kind::Test", "VALUE": "Kind::Value"}
    lines = ["// 자동 생성: tools/gen_sites.py — 손으로 고치지 말 것.",
             f"// exe sha256 {sha}",
             "use crate::site_trace::{Site,Kind,Op};",
             f"pub const SITE_COUNT:usize={len(sites)};",
             "pub static SITES:[Site;SITE_COUNT]=["]
    for s in sites:
        b = bytes.fromhex(s["bytes"])
        bb = ",".join(f"0x{x:02x}" for x in b) + ",0" * (16 - len(b))
        ops = s["ops"] + [None] * (2 - len(s["ops"]))
        lhs = rust_op(ops[0]); rhs = rust_op(ops[1]) if len(ops) > 1 else "Op::None"
        sg = {"u": 0, "s": 1, "x": 2}[s["signed"]]
        rel = OPS.get(s["rel"], 0)
        lines.append(f"Site{{rva:0x{s['rva']:x},len:{s['len']},bytes:[{bb}],kind:{kind[s['trace_kind']]},lhs:{lhs},rhs:{rhs},signed:{sg},rel:{rel}}},")
    lines.append("];")
    lines.append("/// 지점별 소속 함수 RVA (sites.cfg functions= 필터용)")
    lines.append(f"pub static SITE_FUNCS:[u32;SITE_COUNT]=[{','.join(s['function'] for s in sites)}];")
    open(path, "w", encoding="utf-8", newline="\n").write("\n".join(lines) + "\n")


if __name__ == "__main__":
    main()
