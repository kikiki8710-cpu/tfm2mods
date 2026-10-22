"""상세전술 지점 표 생성기 — targets/knobs_<버전>.json + targets/subjects_<버전>.json + 게임 exe → src/sites.rs

하는 일(지점마다):
  1. exe 바이트에서 지점 명령을 디스어셈해 표의 asm·즉치(=기본값 변환)와 대조. 다르면 중단.
  2. 창 = 지점 명령부터 5바이트 이상이 될 때까지의 명령들(E9 rel32 가 들어갈 자리).
     창 안 명령 제약: RIP 상대 메모리·call·ret 금지. 상대 분기(jcc/jmp)는 허용(변형 코드에서 절대 분기로 다시 짠다).
  3. 창 안(첫 바이트 제외)으로 뛰어드는 분기가 없어야 한다:
     ① 포함 함수(.pdata) 전체 선형 디스어셈의 직접 분기 ② 그 함수가 lea 로 가리키는 점프 표(LLVM 상대형·RVA 형 둘 다)
     ③ .text 전체의 rel32 분기 바이트 패턴(E9 / 0F 8x) — 우연 일치도 거절(보수적).
  4. 판단 주체 위치(subjects)를 붙인다. unsupported 지점이 하나라도 있는 전술은 통째로 뺀다(전술 단위 all-or-nothing).

사용: python -I tools/gen_sites.py --exe <TeamfightManager2.exe> [--ver 0.6.5]
패치 대응: 새 버전 knobs/subjects 를 만든 뒤 이 스크립트만 다시 돌린다(소스에 RVA 리터럴을 직접 쓰지 않는다).
"""
import argparse, hashlib, json, re, struct, sys
from pathlib import Path

import numpy as np
from capstone import Cs, CS_ARCH_X86, CS_MODE_64, CS_GRP_JUMP, CS_GRP_CALL, CS_GRP_RET
from capstone.x86 import X86_OP_IMM, X86_OP_MEM, X86_REG_RIP

ROOT = Path(__file__).resolve().parent.parent
REGS = ["rax", "rcx", "rdx", "rbx", "rsp", "rbp", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"]
JCC = {"jo": 0, "jno": 1, "jb": 2, "jae": 3, "je": 4, "jne": 5, "jbe": 6, "ja": 7,
       "js": 8, "jns": 9, "jp": 10, "jnp": 11, "jl": 12, "jge": 13, "jle": 14, "jg": 15}


class PE:
    def __init__(self, path):
        self.data = Path(path).read_bytes()
        d = self.data
        pe = struct.unpack_from("<I", d, 0x3c)[0]
        nsec = struct.unpack_from("<H", d, pe + 6)[0]
        opt = pe + 24
        optsz = struct.unpack_from("<H", d, pe + 20)[0]
        self.secs = []
        for i in range(nsec):
            o = opt + optsz + i * 40
            name = d[o:o + 8].rstrip(b"\0").decode()
            vsz, va, rsz, raw = struct.unpack_from("<IIII", d, o + 8)
            self.secs.append((name, va, max(vsz, rsz), raw, rsz))
        # data directory 3 = exception(.pdata)
        ddir = opt + 112
        self.pdata_rva, self.pdata_size = struct.unpack_from("<II", d, ddir + 3 * 8)

    def off(self, rva):
        for _, va, sz, raw, rsz in self.secs:
            if va <= rva < va + sz:
                if rva - va >= rsz:
                    return None
                return raw + rva - va
        return None

    def read(self, rva, n):
        o = self.off(rva)
        return self.data[o:o + n]

    def sec(self, name):
        return next(s for s in self.secs if s[0] == name)

    def functions(self):
        o = self.off(self.pdata_rva)
        n = self.pdata_size // 12
        arr = np.frombuffer(self.data, dtype="<u4", count=n * 3, offset=o).reshape(n, 3)
        return arr[:, 0].astype(np.int64), arr[:, 1].astype(np.int64)


def find_func(starts, ends, rva):
    i = int(np.searchsorted(starts, rva, side="right")) - 1
    if i < 0 or not (starts[i] <= rva < ends[i]):
        return None
    return int(starts[i]), int(ends[i])


def text_rel32_hits(pe, targets):
    """.text 안 E9 rel32 / 0F 8x rel32 패턴 중 target 집합에 닿는 것(우연 일치 포함)."""
    name, va, sz, raw, rsz = pe.sec(".text")
    buf = np.frombuffer(pe.data, dtype=np.uint8, count=rsz, offset=raw)
    tset = np.array(sorted(targets), dtype=np.int64)
    hits = []
    for oplen, mask in ((1, buf == 0xE9), (2, None)):
        if oplen == 1:
            pos = np.nonzero(mask[:-5])[0]
        else:
            pos = np.nonzero((buf[:-6] == 0x0F) & ((buf[1:-5] & 0xF0) == 0x80))[0]
        if pos.size == 0:
            continue
        b = [buf[pos + oplen + k].astype(np.int64) for k in range(4)]
        rel = b[0] | (b[1] << 8) | (b[2] << 16) | (b[3] << 24)
        rel = np.where(rel >= 1 << 31, rel - (1 << 32), rel)
        tgt = va + pos + oplen + 4 + rel
        sel = np.isin(tgt, tset)
        for p, t in zip(pos[sel], tgt[sel]):
            hits.append((int(va + p), int(t)))
    return hits


def parse_tactics_order():
    src = (ROOT / "src" / "tactics.rs").read_text(encoding="utf-8")
    order = re.findall(r'^\s*[vt]\("([a-z0-9_]+)",\s*"([a-z0-9_]+)"', src, re.M)
    return {knob: i for i, (_id, knob) in enumerate(order)}, [k for _i, k in order]


def parse_src(s):
    if s is None:
        return "Src::None"
    if isinstance(s, dict) and s.get("kind") == "cache_player":
        return f"Src::CachePlayer {{ operation: {REGS.index(s['operation'])}, entity: {REGS.index(s['entity'])}, team_stack: {int(s['team_stack'],16):#x} }}"
    if isinstance(s, dict) and s.get("kind") == "caller":
        cases = []
        for case in s["cases"]:
            if isinstance(case["src"], dict):
                raise SystemExit("caller 주체 중첩 거절")
            cases.append(f"Caller {{ ret_rva: {int(case['ret_rva'], 16):#x}, src: {parse_src(case['src'])} }}")
        return f"Src::Caller({int(s['return_off'], 16):#x}, &[{', '.join(cases)}])"
    p = s.split(":")
    if p[0] == "reg":
        return f"Src::Reg({REGS.index(p[1])})"
    if p[0] == "stack":
        return f"Src::Stack({int(p[1], 16):#x})"
    if p[0] == "mem":
        return f"Src::Mem({REGS.index(p[1])}, {int(p[2], 16):#x})"
    if p[0] == "stackmem":
        return f"Src::StackMem({int(p[1], 16):#x}, {int(p[2], 16):#x})"
    raise SystemExit(f"알 수 없는 src 형식: {s}")


def imm_limit(insn, imm_at, width):
    """즉치 자리에 넣을 수 있는 최대값(부호 확장 고려)."""
    b = insn.bytes
    op = b[1] if b[0] in (0x48, 0x49, 0x4c, 0x4d) else b[0]
    if width == 1:
        # 83 /x ib · 6B ib · disp8(lea) = 부호 확장 → 0..127
        if op in (0x83, 0x6B, 0x8D) or insn.mnemonic in ("cmp", "imul", "add", "lea"):
            return 127
        return 255
    if width == 4:
        return (1 << 31) - 1 if (b[0] & 0xF8) == 0x48 else (1 << 32) - 1
    return (1 << 63) - 1


def tf_of(s):
    # "'v', 0" / "'sq1'," / "'toggle', ..."
    s = s.strip()
    if s.startswith("'sq1'"):
        return ("sq1", 0)
    m = re.match(r"'v',\s*(-?\d+)", s)
    if m:
        return ("v", int(m.group(1)))
    return ("other", s)


def dynamic_instruction(ins):
    """r11에 읽은 동적 즉치로 첫 명령을 실행. r11/플래그/원래 rsp 보존."""
    from capstone.x86 import X86_OP_REG
    def regno(op):
        name = ins.reg_name(op.reg)
        aliases = {"eax": "rax", "ecx": "rcx", "edx": "rdx"}
        return REGS.index(aliases.get(name, name))
    ops = ins.operands
    if any(o.type == X86_OP_REG and regno(o) == 11 for o in ops):
        raise ValueError("r11 operand refused")
    # push r11; mov r11,[rsp-8] (진입 rsp-16의 콜백 결과)
    code = bytearray.fromhex("41 53 4c 8b 5c 24 f8")
    def rr(opcode, dst, src=11, wide=True):
        code.extend([0x40 | (8 if wide else 0) | (4 if src >= 8 else 0) | (1 if dst >= 8 else 0),
                     opcode, 0xc0 | ((src & 7) << 3) | (dst & 7)])
    if ins.mnemonic in ("cmp", "add") and ops[0].type == X86_OP_REG:
        rr(0x39 if ins.mnemonic == "cmp" else 0x01, regno(ops[0]))
    elif ins.mnemonic == "cmp" and ops[0].type == X86_OP_MEM:
        m = ops[0].mem
        if m.index or ins.reg_name(m.base) not in ("rbp", "rsp"):
            raise ValueError("unsupported cmp memory")
        base = ins.reg_name(m.base)
        code.extend([0x4c, 0x39, 0x9d if base == "rbp" else 0x9c])
        if base == "rsp": code.append(0x24)
        code.extend(struct.pack("<i", m.disp + (8 if base == "rsp" else 0)))
    elif ins.mnemonic in ("mov", "movabs"):
        rr(0x89, regno(ops[0]), wide=ops[0].size == 8)
    elif ins.mnemonic == "imul":
        dst, src = regno(ops[0]), regno(ops[1])
        if dst != src: rr(0x89, dst, src)
        code.extend([0x4d if dst >= 8 else 0x49, 0x0f, 0xaf, 0xc3 | ((dst & 7) << 3)])
    elif ins.mnemonic == "lea":
        m = ops[1].mem
        if m.index or ins.reg_name(m.base) != "rdx": raise ValueError("unsupported lea")
        code.append(0x9c)  # lea는 플래그 불변
        rr(0x89, regno(ops[0]), 2)
        rr(0x01, regno(ops[0]))
        code.append(0x9d)
    else:
        raise ValueError(f"unsupported dynamic instruction: {ins.mnemonic}")
    code.extend([0x41, 0x5b])
    return bytes(code)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--exe", required=True)
    ap.add_argument("--ver", default="0.6.5")
    ap.add_argument("--out", default=str(ROOT / "src" / "sites.rs"))
    a = ap.parse_args()
    knobs = json.loads((ROOT / "tools" / "targets" / f"knobs_{a.ver}.json").read_text(encoding="utf-8"))
    subj = json.loads((ROOT / "tools" / "targets" / f"subjects_{a.ver}.json").read_text(encoding="utf-8"))
    pe = PE(a.exe)
    sha = hashlib.sha256(pe.data).hexdigest().upper()
    if sha != knobs["exe_sha256"].upper():
        raise SystemExit(f"exe sha 불일치: {sha} != {knobs['exe_sha256']}")
    order, order_list = parse_tactics_order()
    starts, ends = pe.functions()
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    md.detail = True

    patches, unsupported, log = [], [], []
    interior_all = {}
    func_cache = {}
    for k in knobs["knobs"]:
        kid = k["id"]
        if kid not in order:
            raise SystemExit(f"tactics.rs 에 없는 노브: {kid}")
        t = order[kid]
        bad = [subj["sites"].get(s["rva"], {}).get("unsupported") for s in k["sites"]]
        missing = [s["rva"] for s in k["sites"] if s["rva"] not in subj["sites"]]
        if missing:
            raise SystemExit(f"{kid}: subjects 에 없는 지점 {missing}")
        if any(bad):
            unsupported.append((kid, next(b for b in bad if b)))
            log.append(f"[미지원] {kid}: {next(b for b in bad if b)}")
            continue
        tbv = subj.get("toggle_by_value", {}).get(kid)
        for s in k["sites"]:
            rva = int(s["rva"], 16)
            code = pe.read(rva, 64)
            insns = list(md.disasm(code, 0x140000000 + rva))
            first = insns[0]
            asm = f"{first.mnemonic} {first.op_str}".strip()
            if asm != s["asm"]:
                raise SystemExit(f"{kid} {s['rva']}: asm 불일치 '{asm}' != '{s['asm']}'")
            tf = tf_of(s["tf"])
            if tf[0] == "other":
                raise SystemExit(f"{kid} {s['rva']}: 지원하지 않는 변환 {s['tf']}")
            imm_at, width = s["imm_at"], s["width"]
            cur = int.from_bytes(first.bytes[imm_at:imm_at + width], "little")
            exp = (k["default"] + tf[1]) if tf[0] == "v" else k["default"] ** 2 + 1
            if cur != exp:
                raise SystemExit(f"{kid} {s['rva']}: 즉치 {cur} != 기본값 변환 {exp}")
            # 창
            win, ln = [], 0
            for ins in insns:
                win.append(ins)
                ln += ins.size
                if ln >= 5:
                    break
            branches, fields = [], []
            off = 0
            for ins in win:
                for op in ins.operands:
                    if op.type == X86_OP_MEM and op.mem.base == X86_REG_RIP:
                        raise SystemExit(f"{kid} {s['rva']}: 창 안 RIP 상대 명령 {ins.mnemonic} {ins.op_str}")
                if ins.group(CS_GRP_CALL) or ins.group(CS_GRP_RET):
                    raise SystemExit(f"{kid} {s['rva']}: 창 안 call/ret {ins.mnemonic}")
                if ins.group(CS_GRP_JUMP):
                    if ins.mnemonic not in JCC and ins.mnemonic != "jmp":
                        raise SystemExit(f"{kid} {s['rva']}: 창 안 지원 안 되는 분기 {ins.mnemonic}")
                    if not ins.operands or ins.operands[0].type != X86_OP_IMM:
                        raise SystemExit(f"{kid} {s['rva']}: 간접 분기 {ins.op_str}")
                    cc = JCC.get(ins.mnemonic)
                    branches.append((off, ins.size, cc, ins.operands[0].imm - 0x140000000))
                off += ins.size
            winbytes = b"".join(bytes(i.bytes) for i in win)
            lim = imm_limit(first, imm_at, width)
            if tf[0] == "sq1":
                tfrs = "Tf::Sq1"
            elif tbv:
                tfrs = f"Tf::Map {{ on: {tbv['on']}, off: {tbv['off']} }}"
            else:
                tfrs = f"Tf::Add({tf[1]})"
            fields.append(f"Field {{ off: {imm_at}, width: {width}, tf: {tfrs}, max: {lim} }}")
            interior = set(range(rva + 1, rva + len(winbytes)))
            fn = find_func(starts, ends, rva)
            if fn is None:
                raise SystemExit(f"{kid} {s['rva']}: .pdata 함수 없음")
            # ① 함수 안 직접 분기 ② 점프 표
            if fn not in func_cache:
                body = pe.read(fn[0], fn[1] - fn[0])
                tg = set()
                for ins in md.disasm(body, 0x140000000 + fn[0]):
                    if ins.group(CS_GRP_JUMP) or ins.group(CS_GRP_CALL):
                        if ins.operands and ins.operands[0].type == X86_OP_IMM:
                            tg.add(ins.operands[0].imm - 0x140000000)
                    if ins.mnemonic == "lea" and len(ins.operands) == 2 and ins.operands[1].type == X86_OP_MEM \
                            and ins.operands[1].mem.base == X86_REG_RIP:
                        tb = ins.address + ins.size + ins.operands[1].mem.disp - 0x140000000
                        raw = pe.read(tb, 4 * 2048) if pe.off(tb) is not None else b""
                        # 표는 연속이다 — 첫 항목으로 해석(상대형/RVA형)을 정하고, 함수 밖 항목이 나오면 끝.
                        mode = None
                        for j in range(len(raw) // 4):
                            e = struct.unpack_from("<i", raw, 4 * j)[0]
                            rel_ok, abs_ok = fn[0] <= tb + e < fn[1], fn[0] <= e < fn[1]
                            if mode is None:
                                mode = "rel" if rel_ok else ("abs" if abs_ok else None)
                                if mode is None:
                                    break
                            cand = tb + e if mode == "rel" else e
                            if not (fn[0] <= cand < fn[1]):
                                break
                            tg.add(cand)
                func_cache[fn] = tg
            hit = interior & func_cache[fn]
            if hit:
                raise SystemExit(f"{kid} {s['rva']}: 창 안으로 분기 {sorted(hex(h) for h in hit)} (함수 {fn[0]:#x})")
            for h in interior:
                interior_all[h] = (kid, s["rva"])
            sj = subj["sites"][s["rva"]]
            patches.append(dict(name=f"{kid}@{s['rva']}", tactic=t, rva=rva, win=winbytes, fields=fields,
                                branches=branches, src=parse_src(sj["src"]), chk=parse_src(sj.get("team_chk")),
                                asm=asm, func=fn[0], first_len=first.size, dynamic=dynamic_instruction(first)))
            log.append(f"[창] {kid} {s['rva']} len={len(winbytes)} {' ; '.join(f'{i.mnemonic} {i.op_str}' for i in win)}"
                       f" | 함수 {fn[0]:#x} | 주체 {sj['src']}")
    # ③ .text 전체 rel32 패턴
    hits = text_rel32_hits(pe, interior_all.keys())
    if hits:
        for src_rva, tgt in hits:
            log.append(f"[거절] rel32 패턴 {src_rva:#x} → {tgt:#x} ({interior_all[tgt]})")
        raise SystemExit("창 안으로 닿는 rel32 패턴 발견 — 로그 참조:\n" + "\n".join(l for l in log if l.startswith("[거절]")))
    # 창 겹침 검사
    spans = sorted((p["rva"], p["rva"] + len(p["win"]), p["name"]) for p in patches)
    for (a0, a1, an), (b0, b1, bn) in zip(spans, spans[1:]):
        if b0 < a1:
            raise SystemExit(f"창 겹침: {an} / {bn}")

    out = ["// 자동 생성 — tools/gen_sites.py (손으로 고치지 말 것). 원천 = tools/targets/knobs_{0}.json + subjects_{0}.json".format(a.ver),
           "#![allow(clippy::all)]",
           "use crate::micro::{Branch, Caller, Field, Patch, Src, Tf};",
           f'pub const GAME_VERSION: &str = "{a.ver}";',
           f'pub const EXE_SHA256: &str = "{sha}";',
           "pub static PATCHES: &[Patch] = &["]
    for p in patches:
        br = ", ".join(f"Branch {{ off: {o}, len: {l}, cc: {('Some(%d)' % c) if c is not None else 'None'}, target_rva: {tg:#x} }}"
                       for o, l, c, tg in p["branches"])
        out.append(f'    // {p["asm"]} (함수 {p["func"]:#x})')
        out.append(f'    Patch {{ name: "{p["name"]}", tactic: {p["tactic"]}, rva: {p["rva"]:#x}, '
                   f'win: &[{", ".join(f"0x{b:02x}" for b in p["win"])}], fields: &[{", ".join(p["fields"])}], '
                   f'branches: &[{br}], src: {p["src"]}, team_chk: {p["chk"]}, first_len: {p["first_len"]}, '
                   f'dynamic: &[{", ".join(f"0x{b:02x}" for b in p["dynamic"])}] }},')
    out.append("];")
    out.append("/// 이번 버전에서 판단 주체를 못 잡아 원본으로 두는 전술(노브 id, 이유).")
    out.append("pub static UNSUPPORTED: &[(&str, &str)] = &[")
    for kid, why in unsupported:
        out.append(f'    ("{kid}", "{why}"),')
    out.append("];")
    Path(a.out).write_text("\n".join(out) + "\n", encoding="utf-8", newline="\n")
    (ROOT / "tools" / f"gen_sites_{a.ver}.log.txt").write_text("\n".join(log) + "\n", encoding="utf-8", newline="\n")
    print(f"지점 {len(patches)} · 미지원 전술 {len(unsupported)} · 지원 전술 {len({p['tactic'] for p in patches})} → {a.out}")


if __name__ == "__main__":
    main()
