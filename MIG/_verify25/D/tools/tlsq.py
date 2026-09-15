"""tlsq.py <fn-substring>... — 각 함수의 전이 TLS 도달(LocalKey::with 기준) + 직접 콜리 수. tlsreach.py 의 reach 를 재사용."""
import sys, io, re
sys.argv_saved = sys.argv[:]
sys.argv = [sys.argv[0]]
src = io.open(sys.argv_saved[0].replace("tlsq.py", "tlsreach.py"), encoding="utf-8").read()
src = src[:src.index("root = ")]
exec(src)
for key in sys.argv_saved[1:]:
    fns = [f for f in G if key in f]
    print("== %s : %d defines" % (key, len(fns)))
    for f in fns[:6]:
        r = reach(f)
        print("  %s\n     callees=%d tls=%s" % (nice(f)[:150], len(G[f]), ", ".join(sorted(r)) or "-"))
        for k, p in sorted(r.items()):
            print("       %s: %s" % (k, " > ".join(nice(x)[:60] for x in p[1:])))
