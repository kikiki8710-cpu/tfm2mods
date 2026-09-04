# -*- coding: utf-8 -*-
"""fncheck.py — 함수 진입 RVA 상수가 **현행 exe 에서도 그 함수인지** 검증하고 재핀 후보를 낸다.

왜 필요한가 (2026-09-02 사고):
    `RVA_AUCTION = 0xe65b10` 은 0.5.6 에서 재핀된 값인데 0.5.7·0.5.8 에서 재핀되지 않았다.
    0.5.7 에선 그 자리가 **명령 중간**이라 훅이 무의미했지만 우연히 무해했고,
    0.5.8 에선 하필 **다른 함수의 진입부**가 되어 12B 트램폴린이 박히면서 게임이 즉사했다.
    ⟹ "값이 살아 있다"와 "그 함수가 맞다"는 전혀 다르다. 후자를 기계로 확인해야 한다.

판정
    OK       현행 exe 의 그 주소가 **함수 프롤로그**이고, 원본 exe 의 같은 주소와 프롤로그가 일치
    MOVED    현행 그 주소는 다른 것. 원본 프롤로그가 현행 어딘가에서 **유일하게** 발견됨 → 그 주소로 재핀
    DANGER   현행 그 주소가 **다른 함수의 진입부** (= 엉뚱한 함수를 후킹 중. AUCTION 사고 유형)
    MIDINS   현행 그 주소가 명령 중간/쓰레기 (훅이 무의미하거나 코드 파괴)
    AMBIG    원본 프롤로그가 현행에서 여러 곳에 매치 (수동 판정 필요)

사용
  python MIG\fncheck.py --ref <구exe> --cur <현exe> --rva 0xd2f180 0xcaf0d0 ...
  python MIG\fncheck.py --ref ... --cur ... --from-file <이름=RVA 목록>
"""
import argparse, struct, sys, os

sys.stdout.reconfigure(encoding="utf-8", errors="replace")

PROLOGUE_STARTS = (
    b"\x55",                      # push rbp
    b"\x40\x55", b"\x41\x54", b"\x41\x55", b"\x41\x56", b"\x41\x57",
    b"\x53", b"\x56", b"\x57",    # push rbx/rsi/rdi
    b"\x48\x83\xec", b"\x48\x81\xec",   # sub rsp, imm
    b"\x48\x89\x5c\x24", b"\x48\x89\x4c\x24", b"\x4c\x8b\xdc", b"\x48\x8b\xc4",
)


def load(p):
    d = open(p, "rb").read()
    pe = struct.unpack_from("<I", d, 0x3C)[0]
    nsec = struct.unpack_from("<H", d, pe + 6)[0]
    optsz = struct.unpack_from("<H", d, pe + 20)[0]
    s = pe + 24 + optsz
    secs = []
    for i in range(nsec):
        o = s + i * 40
        n = d[o:o + 8].rstrip(b"\0").decode("ascii", "replace")
        vsz, va, rsz, ptr = struct.unpack_from("<IIII", d, o + 8)
        secs.append((n, va, vsz, ptr, rsz))
    return d, secs


def off(secs, rva):
    for n, va, vsz, ptr, rsz in secs:
        if va <= rva < va + min(vsz, rsz):
            return ptr + (rva - va)
    return None


def text(secs):
    for n, va, vsz, ptr, rsz in secs:
        if n == ".text":
            return va, vsz, ptr, rsz
    return None


def looks_prologue(b):
    return any(b.startswith(x) for x in PROLOGUE_STARTS)


def find_all(hay, needle, limit=8):
    out, i = [], 0
    while len(out) < limit:
        i = hay.find(needle, i)
        if i < 0:
            break
        out.append(i)
        i += 1
    return out


def check(dr, sr, dc, sc, rva, siglen):
    o_ref, o_cur = off(sr, rva), off(sc, rva)
    if o_ref is None:
        return ("NO_REF", "원본 exe 범위 밖", None)
    sig = dr[o_ref:o_ref + siglen]
    cur = dc[o_cur:o_cur + siglen] if o_cur is not None else b""
    if cur == sig:
        return ("OK", "현행 같은 주소에 동일 프롤로그", rva)
    # 원본 프롤로그를 현행 .text 에서 검색
    va, vsz, ptr, rsz = text(sc)
    blob = dc[ptr:ptr + rsz]
    hits = [va + h for h in find_all(blob, sig)]
    cur_is_pro = looks_prologue(cur) if cur else False
    if len(hits) == 1:
        st = "MOVED"
        note = "현행 %s / 새 주소 유일" % ("다른 함수 진입부 ★DANGER" if cur_is_pro else "명령중간·쓰레기")
        return (st, note, hits[0])
    if len(hits) == 0:
        return ("DANGER" if cur_is_pro else "MIDINS",
                "원본 프롤로그가 현행에 없음(함수 자체가 바뀜). 현행 그 자리 = %s"
                % ("다른 함수 진입부" if cur_is_pro else "명령중간·쓰레기"), None)
    return ("AMBIG", "현행 매치 %d곳" % len(hits), hits)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--ref", required=True, help="상수가 확정된 구 exe")
    ap.add_argument("--cur", required=True, help="현행 exe")
    ap.add_argument("--rva", nargs="*", default=[])
    ap.add_argument("--from-file")
    ap.add_argument("--siglen", type=int, default=24, help="프롤로그 지문 길이(기본 24B)")
    a = ap.parse_args()

    items = []
    for r in a.rva:
        items.append((r, int(r, 16)))
    if a.from_file:
        for line in open(a.from_file, encoding="utf-8"):
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            if "=" in line:
                n, v = line.split("=", 1)
                items.append((n.strip(), int(v.strip(), 16)))
            else:
                items.append((line, int(line, 16)))

    dr, sr = load(a.ref)
    dc, sc = load(a.cur)
    print("ref = %s" % os.path.basename(os.path.dirname(a.ref)))
    print("cur = %s" % os.path.basename(os.path.dirname(a.cur)))
    print()
    print("%-26s %-12s %-8s %-14s %s" % ("NAME", "RVA", "판정", "재핀후보", "비고"))
    print("-" * 108)
    bad = 0
    for name, rva in items:
        st, note, new = check(dr, sr, dc, sc, rva, a.siglen)
        cand = ""
        if isinstance(new, int) and new != rva:
            cand = "0x%x" % new
        elif isinstance(new, list):
            cand = ",".join("0x%x" % x for x in new[:3])
        if st != "OK":
            bad += 1
        print("%-26s 0x%-10x %-8s %-14s %s" % (name[:26], rva, st, cand, note))
    print()
    print("문제 있는 상수 = %d / %d" % (bad, len(items)))


if __name__ == "__main__":
    main()
