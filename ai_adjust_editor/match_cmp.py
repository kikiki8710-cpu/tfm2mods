# 같은 다시보기를 서로 다른 설정으로 재생한 판단 로그 2개를 비교한다.
#  한 줄 형식: t{tick} team{N} cid{N}[★] disc{N}({이름}) 명령{N} 플랜{N} pos(x,y) hp{N}%
import io, os, re, sys, collections
sys.stdout.reconfigure(encoding="utf-8")

MD = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_ai_adjust\match_log"
# ⚠cid 는 `{:<4}` 좌측정렬이라 뒤에 공백이 붙고, 그다음 칸이 ★ 또는 공백이다.
#   구 정규식은 그 사이 공백을 못 흡수해 **★(관리팀) 줄을 통째로 놓쳤다**(전체의 절반).
LINE = re.compile(
    r't\s*(-?\d+)\s+team(-?\d+)\s+cid(-?\d+)\s*(★?)\s*disc(-?\d+)\(([^)]*)\)\s*'
    r'\S+?(-?\d+)\s+\S+?(-?\d+)\s+pos\((-?\d+),(-?\d+)\)\s+hp(-?\d+)%')

def parse(path):
    rows = []
    for ln in io.open(path, encoding="utf-8", errors="replace"):
        m = LINE.match(ln.strip())
        if not m: continue
        t, team, cid, star, disc, name, cmd, plan, x, y, hp = m.groups()
        rows.append(dict(t=int(t), team=int(team), cid=int(cid), mine=(star == "★"),
                         disc=int(disc), name=name.strip(), cmd=int(cmd),
                         plan=int(plan), x=int(x), y=int(y), hp=int(hp)))
    return rows

def summarize(tag, rows):
    print("\n" + "=" * 74)
    print("[%s]  줄 %d · 틱 %d~%d · 선수 %d명 · 관리팀 줄 %d"
          % (tag, len(rows), min(r["t"] for r in rows), max(r["t"] for r in rows),
             len(set((r["team"], r["cid"]) for r in rows)),
             sum(1 for r in rows if r["mine"])))
    d = collections.Counter(r["name"] for r in rows)
    tot = sum(d.values())
    print("\n 판단별 발화(전체)")
    for k, v in d.most_common(12):
        print("   %-16s %7d  %5.1f%%" % (k, v, v * 100 / tot))
    return d

def per_player_switch(rows):
    """선수별 판단 전환 횟수 = 우왕좌왕 지표."""
    seq = collections.defaultdict(list)
    for r in sorted(rows, key=lambda r: r["t"]):
        seq[(r["team"], r["cid"])].append((r["t"], r["disc"], r["hp"]))
    out = {}
    for k, v in seq.items():
        sw = sum(1 for i in range(1, len(v)) if v[i][1] != v[i - 1][1])
        span = v[-1][0] - v[0][0] or 1
        out[k] = (sw, len(v), span, sw * 1000.0 / span)   # 1000틱당 전환
    return out

def retreat_hp(rows, names):
    hp = [r["hp"] for r in rows if r["name"] in names]
    return hp

A = parse(os.path.join(MD, "match_00_테스트A.txt"))
B = parse(os.path.join(MD, "match_00_테스트B.txt"))
dA = summarize("테스트A", A)
dB = summarize("테스트B", B)

print("\n" + "=" * 74)
print("판단 발화 비교 (A vs B, 전체 대비 %)")
allk = sorted(set(dA) | set(dB), key=lambda k: -(dA.get(k, 0) + dB.get(k, 0)))
ta, tb = sum(dA.values()), sum(dB.values())
print("  %-18s %10s %10s %10s" % ("판단", "A", "B", "차이(%p)"))
for k in allk[:15]:
    pa = dA.get(k, 0) * 100 / ta; pb = dB.get(k, 0) * 100 / tb
    print("  %-18s %9.1f%% %9.1f%% %+9.1f" % (k, pa, pb, pb - pa))

print("\n" + "=" * 74)
print("판단 전환 빈도 (1000틱당) — 낮을수록 뚝심 있음")
sa, sb = per_player_switch(A), per_player_switch(B)
common = sorted(set(sa) & set(sb))
print("  %-12s %12s %12s" % ("선수", "A", "B"))
for k in common:
    print("  team%d cid%-5d %11.1f %11.1f" % (k[0], k[1], sa[k][3], sb[k][3]))
if common:
    ma = sum(sa[k][3] for k in common) / len(common)
    mb = sum(sb[k][3] for k in common) / len(common)
    print("  %-12s %11.1f %11.1f   (B−A = %+.1f)" % ("평균", ma, mb, mb - ma))

print("\n" + "=" * 74)
print("체력 분포 — 판단이 뜬 순간의 hp%")
for tag, rows in [("A", A), ("B", B)]:
    hp = [r["hp"] for r in rows if 0 <= r["hp"] <= 100]
    hp.sort()
    if not hp: continue
    q = lambda p: hp[int(len(hp) * p)]
    print("  %s  최저 %3d · 25%% %3d · 중앙 %3d · 75%% %3d  (평균 %.1f)"
          % (tag, hp[0], q(.25), q(.5), q(.75), sum(hp) / len(hp)))

print("\n" + "=" * 74)
print("귀환/후퇴 계열이 뜬 순간의 체력")
RET = [k for k in allk if any(w in k for w in ("귀환", "후퇴", "복귀", "도주"))]
print("  대상 판단:", ", ".join(RET) if RET else "(이름에서 못 찾음)")
for tag, rows in [("A", A), ("B", B)]:
    hp = [r["hp"] for r in rows if r["name"] in RET and 0 <= r["hp"] <= 100]
    if hp: print("  %s  n=%-6d 평균 hp %.1f%%  중앙 %d%%" % (tag, len(hp), sum(hp) / len(hp), sorted(hp)[len(hp) // 2]))
    else:  print("  %s  해당 없음" % tag)
