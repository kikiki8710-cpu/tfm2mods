# -*- coding: utf-8 -*-
# repin_053.py — 프롤로그 바이트 정확일치 필터 + 니모닉 코사인으로 특정 함수를 0.5.3에서 재핀.
#   _MIGRATE_053.md 의 통계 매칭이 프롤로그 실측에서 탈락한 건(MOVEPRI 등)을 다시 찾는다.
#   install_replace_detour 계열은 신원검증이 없으므로(오후킹=즉시 크래시) 프롤로그 일치를 필수 조건으로 둔다.
import pickle, sys, io, math, struct
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

O = pickle.load(open(r"C:\tfm2mods\_fnidx_052.pkl", "rb"))
N = pickle.load(open(r"C:\tfm2mods\_fnidx_053.pkl", "rb"))


def load(p):
    d = open(p, "rb").read()
    pe = struct.unpack_from("<I", d, 0x3c)[0]
    nsec = struct.unpack_from("<H", d, pe + 6)[0]
    opt = pe + 24
    sectab = opt + struct.unpack_from("<H", d, pe + 20)[0]
    secs = []
    for i in range(nsec):
        o = sectab + i * 40
        nm = d[o:o + 8].rstrip(b"\0").decode(errors="replace")
        vsz, va, rsz, rraw = struct.unpack_from("<IIII", d, o + 8)
        secs.append((nm, va, vsz, rraw, rsz))
    return d, secs


def rd(d, secs, rva, n):
    for nm, va, vsz, rraw, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            off = rva - va
            if off >= rsz:
                return b""
            return d[rraw + off: rraw + off + n]
    return b""


DO, SO = load(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe")
DN, SN = load(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe")


def cos(a, b):
    ks = set(a) | set(b)
    da = math.sqrt(sum(v * v for v in a.values()))
    db = math.sqrt(sum(v * v for v in b.values()))
    if not da or not db:
        return 0.0
    return sum(a.get(k, 0) * b.get(k, 0) for k in ks) / (da * db)


def repin(name, old_rva, pro_len, topn=8, size_tol=(0.7, 1.5), require_pro=True):
    o = O["idx"].get(old_rva)
    if not o:
        print(f"[{name}] 0.5.2 인덱스에 함수시작 없음 (0x{old_rva:x})")
        return []
    pro = rd(DO, SO, old_rva, pro_len)
    print(f"[{name}] OLD 0x{old_rva:x} size={o['size']} ninsn={o['ninsn']} pro={pro.hex(' ')}")
    hits = []
    for rva, v in N["idx"].items():
        if require_pro and rd(DN, SN, rva, pro_len) != pro:
            continue
        r = v["size"] / o["size"] if o["size"] else 0
        if not (size_tol[0] <= r <= size_tol[1]):
            continue
        c = cos(o["mnem"], v["mnem"])
        hits.append((c, rva, v["size"], r, v["skel"] == o["skel"], v["head"] == o["head"]))
    hits.sort(reverse=True)
    print(f"    프롤로그{pro_len}B 일치 & 크기비 내 후보 {len(hits)}건")
    for c, rva, sz, r, sk, hd in hits[:topn]:
        tag = "  ★SKEL동일" if sk else ("  ☆HEAD동일" if hd else "")
        print(f"      0x{rva:<9x} cos={c:.6f} size={sz} 비={r:.3f}{tag}")
    return hits


if __name__ == "__main__":
    print("=" * 70)
    # MOVEPRI: push r15,r14,r12,rsi,rdi,rbx + sub rsp,0x48 = 13B
    h = repin("RVA_MOVEPRI", 0x2134240, 13)
    print()
    # 참고: CONDGATE 확정값 0xc550b0 과의 인접성 확인
    print("  ↑ CONDGATE(0.5.3 확정) = 0xc550b0 / 0.5.2 간격 = 0x970")
    for c, rva, sz, r, sk, hd in h[:20]:
        print(f"      0x{rva:x} : CONDGATE와 간격 {rva-0xc550b0:+#x}")
