# -*- coding: utf-8 -*-
# bytepatch_053.py — ai_adjust 의 patch_imm_bytes 사이트 전수를 0.5.3 에서 재핀.
#   원리: 각 사이트는 (prefix opcode + 원본 imm) 형태이고, 코드가 prefix 를 검증한 뒤에만 imm 을 쓴다.
#         ⟹ "prefix + 원본 imm" 을 시그로 삼아 NEW 컨테이너 안을 스캔하면 오프셋 변화와 무관하게 재도출된다.
#   컨테이너: OLD 사이트를 품은 .pdata 함수 → 그 함수의 NEW 대응(_MIGRATE_053.md / 확정 매핑표).
#   유일하지 않으면 주변 명령 시퀀스로 좁히고, 그래도 애매하면 미해결로 남긴다(패치 skip = fail-safe).
import struct, sys, io, re, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)
SRC = r"C:\tfm2mods\tfm2_ai_adjust\src\detour.rs"


def load(p):
    d = open(p, "rb").read()
    pe = struct.unpack_from("<I", d, 0x3c)[0]
    nsec = struct.unpack_from("<H", d, pe + 6)[0]
    opt = pe + 24
    ib = struct.unpack_from("<Q", d, opt + 24)[0]
    sectab = opt + struct.unpack_from("<H", d, pe + 20)[0]
    secs = []
    for i in range(nsec):
        o = sectab + i * 40
        nm = d[o:o + 8].rstrip(b"\0").decode(errors="replace")
        vsz, va, rsz, rraw = struct.unpack_from("<IIII", d, o + 8)
        secs.append((nm, va, vsz, rraw, rsz))
    magic = struct.unpack_from("<H", d, opt)[0]
    ddir = opt + (112 if magic == 0x20b else 96)
    ex, ez = struct.unpack_from("<II", d, ddir + 3 * 8)
    return d, ib, secs, ex, ez


def roff(secs, rva):
    for nm, va, vsz, rraw, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            o = rva - va
            return rraw + o if o < rsz else None
    return None


def pdata(d, secs, ex, ez):
    po = roff(secs, ex)
    out = [struct.unpack_from("<III", d, po + i * 12)[:2] for i in range(ez // 12)]
    out.sort()
    return out


def owner(fns, rva):
    lo, hi = 0, len(fns) - 1
    while lo <= hi:
        m = (lo + hi) // 2
        if fns[m][0] <= rva < fns[m][1]:
            return fns[m]
        if rva < fns[m][0]:
            hi = m - 1
        else:
            lo = m + 1
    return None


DO, IBO, SO, EO, ZO = load(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe")
DN, IBN, SN, EN, ZN = load(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe")
FO, FN = pdata(DO, SO, EO, ZO), pdata(DN, SN, EN, ZN)

# OLD 컨테이너 → NEW 컨테이너 (실측·매칭 확정분). 미등재 컨테이너는 미해결로 처리.
CONT = {
    0x1b92e40: 0xdec6b0, 0x1bdaaa0: 0xdf9320, 0x2376320: 0xd94d00, 0x22b2280: 0xe06c10,
    0x23ad980: 0xcdd010, 0x22dd9a0: 0xcc9d70, 0x22efed0: 0xcd4b40, 0x23a04d0: 0xc7f640,
    0x2380820: 0xdece30, 0x19b3e70: 0x25b12e0, 0x2126610: None, 0x2398240: None,
    0x23ba8d0: None, 0x22e6460: None, 0x20def40: None,
}

RE_CALL = re.compile(
    r'patch_imm_bytes\(\s*base\s*\+\s*(0x[0-9a-fA-F]+)\s*,\s*&\[([^\]]*)\]\s*,\s*(\d+)\s*,\s*(\d+)\s*,', re.S)


def parse_sites():
    src = open(SRC, encoding="utf-8", errors="replace").read()
    lines = src.split("\n")
    out = []
    for m in RE_CALL.finditer(src):
        rva = int(m.group(1), 16)
        pre = [int(x.strip(), 16) for x in m.group(2).split(",") if x.strip()]
        off, w = int(m.group(3)), int(m.group(4))
        ln = src[:m.start()].count("\n") + 1
        cmt = ""
        c = re.search(r'//\s*(.+)$', lines[ln - 1]) if ln <= len(lines) else None
        if c:
            cmt = c.group(1).strip()[:60]
        out.append(dict(rva=rva, pre=bytes(pre), off=off, w=w, line=ln, cmt=cmt))
    return out


def disq(d, secs, ib, rva, n):
    o = roff(secs, rva)
    return [(i.mnemonic, re.sub(r'0x[0-9a-f]+', 'I', i.op_str)) for i in md.disasm(d[o:o + n], ib + rva)]


def repin(s):
    rva, pre, off, w = s["rva"], s["pre"], s["off"], s["w"]
    o = roff(SO, rva)
    if o is None:
        return ("BAD", "OLD 섹션밖", None)
    actual = DO[o:o + off]
    if actual != pre:
        return ("STALE", f"OLD prefix 불일치 실제={actual.hex(' ')} 기대={pre.hex(' ')}", None)
    sig = DO[o:o + off + w]                      # prefix + 원본 imm
    imm = int.from_bytes(DO[o + off:o + off + w], "little")
    fo = owner(FO, rva)
    if fo is None:
        return ("BAD", "OLD 컨테이너 없음", None)
    if fo[0] not in CONT or CONT[fo[0]] is None:
        return ("NOCONT", f"컨테이너 0x{fo[0]:x} 미해결", imm)
    ns = CONT[fo[0]]
    fnn = owner(FN, ns)
    if fnn is None:
        return ("BAD", f"NEW 컨테이너 0x{ns:x} .pdata 없음", imm)
    no = roff(SN, fnn[0])
    blob = DN[no: no + (fnn[1] - fnn[0])]
    hits = [m.start() for m in re.finditer(re.escape(sig), blob)]
    if len(hits) == 1:
        return ("OK", fnn[0] + hits[0], imm)
    if len(hits) == 0:
        return ("MISS", f"컨테이너 0x{fnn[0]:x}(size {fnn[1]-fnn[0]})에 시그 {sig.hex(' ')} 없음", imm)
    # 다중 → OLD 사이트의 앞뒤 명령 문맥으로 좁힌다
    ctxo = disq(DO, SO, IBO, rva - 0, 24)
    best = []
    for h in hits:
        ctxn = disq(DN, SN, IBN, fnn[0] + h, 24)
        score = sum(1 for a, b in zip(ctxo, ctxn) if a == b)
        best.append((score, fnn[0] + h))
    best.sort(reverse=True)
    if len(best) > 1 and best[0][0] > best[1][0]:
        return ("OK?", best[0][1], imm)
    return ("AMBIG", f"{len(hits)}건 " + ",".join(f"0x{b:x}" for _, b in best[:4]), imm)


if __name__ == "__main__":
    sites = parse_sites()
    print(f"patch_imm_bytes 사이트 {len(sites)}건\n")
    stat = {}
    res = []
    for s in sites:
        st, val, imm = repin(s)
        stat[st] = stat.get(st, 0) + 1
        immS = f"imm={imm:#x}" if imm is not None else ""
        if st in ("OK", "OK?"):
            print(f"  [{st:5s}] L{s['line']:<5d} 0x{s['rva']:x} → **0x{val:x}**  {immS}  {s['cmt'][:42]}")
            res.append(dict(line=s["line"], old=s["rva"], new=val, grade=st, cmt=s["cmt"]))
        else:
            print(f"  [{st:5s}] L{s['line']:<5d} 0x{s['rva']:x} — {val}  {immS}")
            res.append(dict(line=s["line"], old=s["rva"], new=None, grade=st, cmt=s["cmt"]))
    print("\n요약:", stat)
    json.dump(res, open(r"C:\tfm2mods\_bytepatch_053.json", "w", encoding="utf-8"), indent=1)
