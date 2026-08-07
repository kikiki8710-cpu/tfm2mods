# 가설 검증: "cid 가 늘어나는 건 죽고 부활할 때마다 엔티티가 새로 생겨서다"
#  → 참이면 ①기본 10개는 틱1부터 ②나머지는 나중에 등장 ③등장 시점이 다른 cid 의 소멸 시점과 맞물림
import io, os, re, sys, collections
sys.stdout.reconfigure(encoding="utf-8")

MD = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_ai_adjust\match_log"
L = re.compile(r't\s*(-?\d+)\s+team(-?\d+)\s+cid(-?\d+)\s*(★?)\s*disc(-?\d+)\(([^)]*)\)')

def load(f):
    span = {}                     # (team,cid) -> [first, last, n, mine]
    for ln in io.open(os.path.join(MD, f), encoding="utf-8", errors="replace"):
        m = L.match(ln.strip())
        if not m: continue
        t, team, cid, star = int(m.group(1)), int(m.group(2)), int(m.group(3)), bool(m.group(4))
        k = (team, cid)
        if k not in span: span[k] = [t, t, 0, star]
        span[k][1] = max(span[k][1], t); span[k][2] += 1
    return span

for f in ["match_00_테스트A.txt", "match_00_테스트B.txt"]:
    span = load(f)
    print("\n" + "=" * 78)
    print(f, " — 엔티티 %d개" % len(span))
    start1 = [k for k, v in span.items() if v[0] <= 5]
    later  = sorted([k for k, v in span.items() if v[0] > 5], key=lambda k: span[k][0])
    print("  틱 1부터 존재: %d개  %s" % (len(start1), sorted(start1)))
    print("  나중에 등장  : %d개" % len(later))
    byteam = collections.Counter(k[0] for k in later)
    print("  나중 등장 팀별: team0=%d  team1=%d" % (byteam[0], byteam[1]))

    # 등장 시점이 '같은 팀의 다른 엔티티 소멸'과 맞물리는지
    ends = sorted([(v[1], k) for k, v in span.items()])
    matched = 0
    for k in later:
        st = span[k][0]
        # 같은 팀에서, 이 등장보다 앞서 끝난 엔티티가 있는가 (부활 = 이전 몸의 마지막 판단 이후)
        if any(e <= st and kk[0] == k[0] and kk != k for e, kk in ends): matched += 1
    print("  등장 전에 같은 팀 엔티티가 끝난 사례: %d/%d" % (matched, len(later)))

    # 부활 간격 분포 (등장 시각 차)
    st = sorted(span[k][0] for k in later)
    if len(st) > 1:
        gaps = [st[i] - st[i - 1] for i in range(1, len(st))]
        gaps.sort()
        print("  등장 간격(틱) 중앙 %d · 최소 %d · 최대 %d" % (gaps[len(gaps)//2], gaps[0], gaps[-1]))

    # 한 시점에 살아있는 엔티티 수 = 항상 10 근처인가
    evts = []
    for k, v in span.items(): evts.append((v[0], 1)); evts.append((v[1], -1))
    evts.sort()
    cur = 0; hist = collections.Counter()
    for _, d in evts:
        cur += d; hist[cur] += 1
    print("  동시 활성 엔티티 수 분포(상위):", dict(hist.most_common(6)))

    # 줄 수 상위/하위
    tops = sorted(span.items(), key=lambda kv: -kv[1][2])[:3]
    lows = sorted(span.items(), key=lambda kv: kv[1][2])[:3]
    print("  줄 많은 엔티티:", [(k, v[2], "t%d~%d" % (v[0], v[1])) for k, v in tops])
    print("  줄 적은 엔티티:", [(k, v[2], "t%d~%d" % (v[0], v[1])) for k, v in lows])
