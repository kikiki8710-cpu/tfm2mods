# -*- coding: utf-8 -*-
"""
enumdiff.py — 두 exe 의 .rdata 에서 Rust `#[derive(Debug)]` 가 남기는 변형 이름 연접 문자열(예: "RunAwayRecallAround…")을 뽑아
CamelCase 토큰열로 쪼개고, 구/신 exe 를 대조해 「어느 enum 이 변형을 잃었나/얻었나」 를 exe 만으로 본다.
방법: .rdata 전체에서 [A-Za-z0-9_]{12,} 런 → CamelCase 분절(대문자 경계) → 3토큰 이상인 런만 → 구/신 집합 대조.
     같은 enum 은 토큰열의 LCS 유사도(≥0.6)로 짝짓고 차집합을 출력.
사용: python enumdiff.py [--min 3]   → _next\enumdiff.md
"""
import re, sys, io, os, difflib, collections
sys.stdout.reconfigure(encoding="utf-8")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import mig060 as M
HERE = M.HERE
def runs(path):
    E = M.Exe(path, "x"); va, sz = E.secs[".rdata"]; buf = bytes(E.img[va:va + sz])
    out = {}
    for m in re.finditer(rb"[A-Za-z][A-Za-z0-9_]{11,}", buf):
        s = m.group(0).decode()
        toks = re.findall(r"[A-Z][a-z0-9_]*|[A-Z]+(?![a-z])|[a-z0-9_]+", s)
        # CamelCase 연접(각 토큰이 대문자 시작)만 · 소문자 시작 식별자(함수명·경로) 제외
        if len(toks) >= 3 and all(t[0].isupper() for t in toks) and len(set(toks)) >= 3:
            out[s] = toks
    return out
def main():
    mn = int(sys.argv[sys.argv.index("--min") + 1]) if "--min" in sys.argv else 3
    RO, RN = runs(M.OLD), runs(M.NEW)
    same = set(RO) & set(RN)
    oo = {k: v for k, v in RO.items() if k not in same}; nn = {k: v for k, v in RN.items() if k not in same}
    L = [u"# enumdiff — Debug 변형 이름 연접 문자열 대조 (0.5.8 ↔ 0.6.0 · .rdata)", u"",
         u"연접 런: 구 %d · 신 %d · 완전 동일 %d · 구에만 %d · 신에만 %d" % (len(RO), len(RN), len(same), len(oo), len(nn)), u"",
         u"| 구 토큰열 | 신 토큰열 | 구에만 | 신에만 | 유사도 |", u"|---|---|---|---|---|"]
    used = set(); pairs = []
    for k, v in sorted(oo.items(), key=lambda kv: -len(kv[1])):
        best = None
        for k2, v2 in nn.items():
            if k2 in used: continue
            r = difflib.SequenceMatcher(None, v, v2, autojunk=False).ratio()
            if r >= 0.6 and (best is None or r > best[0]): best = (r, k2, v2)
        if best:
            used.add(best[1]); r, k2, v2 = best
            pairs.append((v, v2, r))
            L.append(u"| %s | %s | **%s** | **%s** | %.2f |" % (u"·".join(v)[:120], u"·".join(v2)[:120], u" ".join(t for t in v if t not in v2), u" ".join(t for t in v2 if t not in v), r))
        else:
            L.append(u"| %s | — | (통째로 사라짐) | | |" % u"·".join(v)[:160])
    for k2, v2 in nn.items():
        if k2 not in used and len(v2) >= mn: L.append(u"| — | %s | | (신규) | |" % u"·".join(v2)[:160])
    io.open(os.path.join(HERE, "_next", "enumdiff.md"), "w", encoding="utf-8").write(u"\n".join(L))
    print(u"\n".join(L[:3])); print(u"짝 %d · 구에만 남음 %d · 신에만 %d" % (len(pairs), len(oo) - len(pairs), len(nn) - len(used)))
    for v, v2, r in pairs:
        d1 = [t for t in v if t not in v2]; d2 = [t for t in v2 if t not in v]
        if d1 or d2: print(u"  %-70s −%s +%s" % (u"·".join(v)[:70], u" ".join(d1), u" ".join(d2)))
if __name__ == "__main__": main()
