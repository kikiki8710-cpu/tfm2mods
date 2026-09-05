# -*- coding: utf-8 -*-
"""스캐폴딩 md의 ```c 블록을 Ghidra HTTP(8081) 디컴 결과로 채운다."""
import io, os, re, sys, json, math, urllib.request, urllib.parse

BASE = 0x140000000
ROOT = r"C:\tfm2mods\MIG\decomp\0.5.8"
HOST = "http://127.0.0.1:8081"
EXE  = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"

PLACEHOLDER = "// <본문> — 디컴 결과를 여기에 채운다"

OFFS = {
    0x1e0:  "로스터 슬롯 베이스(+0x1e0 + side*0x28 + role*8)",
    0x438:  "히트박스",
    0x470:  "사거리%보정",
    0x490:  "어빌 슬롯 베이스(+0x490 + k*0x38)",
    0x4c0:  "슬롯 id(-1=없음)",
    0x4f8:  "슬롯 id(-1=없음)",
    0x5c0:  "핸들",
    0x5c8:  "레벨",
    0x628:  "최대HP",
    0x660:  "x",
    0x668:  "y",
    0x670:  "현재HP",
    0x680:  "기본사거리",
    0x930:  "side",
    0x9c0:  "role",
    0x12f8: "tick/sec",
}

def http(path, **params):
    url = HOST + path
    if params:
        url += "?" + urllib.parse.urlencode(params)
    last = None
    for _try in range(3):
        try:
            with urllib.request.urlopen(url, timeout=900) as r:
                return r.read().decode("utf-8", "replace")
        except Exception as e:
            last = e
            import time as _t; _t.sleep(3)
    return "!!ERR!! %s" % last

# ---------- capstone fallback ----------
_pe = None
def pe():
    global _pe
    if _pe is None:
        import pefile
        _pe = pefile.PE(EXE, fast_load=True)
    return _pe

def read_rva(rva, size):
    p = pe()
    return p.get_data(rva, size)

def jumptable_targets(start_rva, end_rva):
    """lea rX,[rip+imm] / movsxd rY,[rX+rZ*4] / add rY,rX / jmp rY 패턴의 점프테이블 타깃 나열."""
    from capstone import Cs, CS_ARCH_X86, CS_MODE_64
    data = read_rva(start_rva, end_rva - start_rva)
    md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = False
    ins = list(md.disasm(data, BASE + start_rva))
    res = []
    for n, i in enumerate(ins):
        if i.mnemonic != "lea": continue
        m = re.match(r"(\w+), \[rip \+ (0x[0-9a-f]+)\]", i.op_str)
        if not m: continue
        base_reg, disp = m.group(1), int(m.group(2), 16)
        tbl = i.address + i.size + disp - BASE
        # 뒤 6개 안에 movsxd [base+..*4] 와 jmp 가 있나
        win = ins[n+1:n+7]
        if not any(x.mnemonic == "movsxd" and (base_reg + " +") in x.op_str for x in win): continue
        if not any(x.mnemonic == "jmp" and not x.op_str.startswith("0x") for x in win): continue
        ents = []
        try:
            raw = read_rva(tbl, 4*96)
        except Exception:
            continue
        for k in range(96):
            v = int.from_bytes(raw[k*4:k*4+4], "little", signed=True)
            t = tbl + v
            if not (start_rva <= t < end_rva): break
            ents.append((k, t))
        if ents:
            res.append((i.address - BASE, tbl, ents))
    return res

def lin_disasm(start_rva, end_rva):
    from capstone import Cs, CS_ARCH_X86, CS_MODE_64
    data = read_rva(start_rva, end_rva - start_rva)
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    md.detail = False
    out = []
    for i in md.disasm(data, BASE + start_rva):
        out.append("%08x  %-9s %s" % (i.address - BASE, i.mnemonic, i.op_str))
    txt = annotate("\n".join(out))
    try:
        jts = jumptable_targets(start_rva, end_rva)
    except Exception:
        jts = []
    pre = []
    for at, tbl, ents in jts:
        pre.append("// 점프테이블 @%#x (dispatch %#x): " % (tbl, at) +
                   ", ".join("[%d]->%#x" % (k, t) for k, t in ents))
    if pre:
        txt = "\n".join(pre) + "\n" + txt
    return txt

# ---------- annotation ----------
HEXRE = re.compile(r"0x[0-9a-fA-F]+")
VTRE  = re.compile(r"\(code \*\*?\)\([^()]*?\+\s*(0x[0-9a-fA-F]+)\)")

def issq(v):
    if v < 100000: return None
    r = math.isqrt(v)
    return r if r*r == v else None

def annotate(code):
    out = []
    for ln in code.split("\n"):
        if "//" in ln or ln.strip().startswith("/*"):
            out.append(ln); continue
        notes = []
        for m in VTRE.finditer(ln):
            o = int(m.group(1), 16)
            if o <= 0x28:
                notes.append("vtbl+%#x(WorldOps +0x20/+0x28 불변)" % o)
            elif o < 0x38:
                notes.append("vtbl+%#x(WorldOps계열 Δ+0x10)" % o)
            else:
                notes.append("vtbl+%#x(WorldOps계열 Δ+0x10 / 대형 dyn-desc Δ+0x8)" % o)
        seen = set()
        for m in HEXRE.finditer(ln):
            try: v = int(m.group(0), 16)
            except: continue
            if v in seen: continue
            seen.add(v)
            if v in OFFS:
                notes.append("%s=%s" % (m.group(0), OFFS[v]))
            else:
                s = issq(v)
                if s is not None and s % 100 == 0:
                    notes.append("%s = %d² (거리 제곱)" % (m.group(0), s))
        for m in re.finditer(r"(?<![\w.])(\d{6,})(?![\w.])", ln):
            v = int(m.group(1))
            s = issq(v)
            if s is not None and s % 100 == 0:
                notes.append("%d = %d² (거리 제곱)" % (v, s))
        if notes:
            # 중복 제거 순서보존
            u, sn = [], set()
            for n in notes:
                if n not in sn: sn.add(n); u.append(n)
            ln = ln + "  // " + " · ".join(u)
        out.append(ln)
    return "\n".join(out)

# ---------- main ----------
HDR = re.compile(r"^## `0x([0-9a-fA-F]+)`\s+—\s+원본 행 (\d+)~(\d+)\s*$")
RNG = re.compile(r"^\| RVA \| `0x([0-9a-fA-F]+)` ~ `0x([0-9a-fA-F]+)`")

def process(relpath, force=False):
    path = os.path.join(ROOT, relpath)
    src = io.open(path, encoding="utf-8").read()
    lines = src.split("\n")
    module = relpath.replace("/", "\\").replace(".md", ".rs")
    n_fill = 0; fails = []
    i = 0
    while i < len(lines):
        m = HDR.match(lines[i])
        if not m:
            i += 1; continue
        rva = int(m.group(1), 16); line0 = m.group(2)
        # 범위
        end = None
        for j in range(i, min(i+12, len(lines))):
            r = RNG.match(lines[j])
            if r: end = int(r.group(2), 16) + 1; break
        # ```c 블록 찾기
        k = i+1
        while k < len(lines) and lines[k] != "```c":
            if HDR.match(lines[k]): break
            k += 1
        if k >= len(lines) or lines[k] != "```c":
            i += 1; continue
        e = k+1
        while e < len(lines) and lines[e] != "```":
            e += 1
        body = "\n".join(lines[k+1:e])
        if PLACEHOLDER not in body and not force:
            i = e+1; continue   # 이미 채워짐
        abs_addr = "0x%x" % (rva + BASE)
        dec = http("/decompile_function_by_address", address=abs_addr)
        info = http("/get_function_by_address", address=abs_addr)
        newb = ["// fn @ %s:%s" % (module, line0)]
        ok = dec and not dec.startswith("!!ERR!!") and "unction not found" not in dec[:80] and "No function found" not in dec[:80] \
             and "Decompilation failed" not in dec[:80] and dec.strip() != ""
        if not ok:
            fails.append((relpath, abs_addr, (dec or "")[:80]))
            newb.append("// DECOMP FAILED: %s" % (dec or "empty").strip().replace("\n"," ")[:200])
            try:
                newb.append("// --- capstone 선형 디스어셈 (%#x ~ %#x) ---" % (rva, end))
                newb.append(lin_disasm(rva, end))
            except Exception as ex:
                newb.append("// disasm 실패: %s" % ex)
        else:
            newb.append(annotate(dec.strip().rstrip()))
            # 점프테이블로 본문이 잘렸는지 확인
            bm = re.search(r"Body: ([0-9a-f]+) - ([0-9a-f]+)", info or "")
            if bm and end:
                gend = int(bm.group(2), 16) - BASE + 1
                if gend + 16 < end:
                    newb.append("")
                    newb.append("// !! Ghidra 함수 본문이 %#x 에서 끊김(점프테이블 미복구). 전체 범위 선형 디스어셈:" % gend)
                    try:
                        newb.append("// --- capstone %#x ~ %#x ---" % (rva, end))
                        newb.append(lin_disasm(rva, end))
                    except Exception as ex:
                        newb.append("// disasm 실패: %s" % ex)
                    fails.append((relpath, abs_addr, "jumptable-truncated (body ends %#x, expect %#x)" % (gend, end)))
        lines[k+1:e] = newb
        n_fill += 1
        i = k + 1 + len(newb) + 1
    io.open(path, "w", encoding="utf-8", newline="\n").write("\n".join(lines))
    return n_fill, fails

if __name__ == "__main__":
    tot = 0; allf = []
    for rel in sys.argv[1:]:
        n, f = process(rel)
        tot += n; allf += f
        print("[OK] %-45s %d개 채움 %s" % (rel, n, ("/ 주의 %d" % len(f)) if f else ""))
        sys.stdout.flush()
    print("=== 총 %d 함수" % tot)
    for f in allf:
        print("  ! %s %s : %s" % f)
