"""run27D_ec.py — o27D_ec.exe 를 케이스당 프로세스 1개로 돌려 Flexible MATCH/DIFF 집계(+Stable/Aggressive 관측). 산출 = oracle/o27D_ec.log"""
import subprocess, random, sys, io, os, collections
from concurrent.futures import ThreadPoolExecutor
EXE = os.path.expandvars(r"%LOCALAPPDATA%\Temp\tfm2_spanprobe\o27D_ec.exe")
HERE = os.path.dirname(os.path.abspath(__file__))
LOG = os.path.join(HERE, "o27D_ec.log")

def run(case):
    args = [EXE] + ["%s=%s" % kv for kv in case.items()]
    try:
        out = subprocess.run(args, capture_output=True, text=True, timeout=120, encoding='utf-8', errors='replace').stdout
    except Exception as e:
        out = "EXC %s" % e
    return case, out

cases = []
# 특수: 타워 생존 / 한 기만 생존 / disable 로 타워 검사 생략 / 적 0 / twin
cases.append(dict(strat=1))
cases.append(dict(strat=1, towers=0, twin=0, ek=0, gather=1000, ahp=1000))
cases.append(dict(strat=1, t1only=1, twin=0, ek=0, gather=1000, ahp=1000))            # tower1 만 살아있음 → false
cases.append(dict(strat=1, disable=2000, tick=3000, ek=0, gather=1000, ahp=1000))      # tick>disable → 타워·twin 무시
cases.append(dict(strat=1, disable=2000, tick=2000, ek=0, gather=1000, ahp=1000))      # tick==disable → 검사함
cases.append(dict(strat=1, towers=0, twin=1, ek=0, gather=1000, ahp=1000))             # 적 0 이면 twin 무관 true
cases.append(dict(strat=1, towers=0, twin=1, ek=2, gather=1000, ahp=1000))             # twin 살아있고 적 2 → false
cases.append(dict(strat=1, towers=0, twin=0, ek=2, gather=1000, ahp=1000, far=0, fard=1300000))   # 아군 흩어짐 → false
cases.append(dict(strat=1, towers=0, twin=0, ek=2, gather=1000, ahp=1000, far=0, fard=1190000))   # 경계 안
cases.append(dict(strat=1, towers=0, twin=0, ek=2, gather=1000, ahp=1000, far=0, fard=1200000))   # 정확히 1,200,000 (≤ 허용)
cases.append(dict(strat=1, towers=0, twin=0, ek=2, gather=1000, ahp=1000, far=0, fard=1200001))   # 1,200,001 → 초과
cases.append(dict(strat=1, towers=0, twin=0, ek=2, gather=1000, ahp=1000, ebuff=1199))  # epic < 1200 경로
cases.append(dict(strat=1, towers=0, twin=0, ek=2, gather=1000, ahp=1000, ebuff=1200, mc=6))  # epic >= 1200 · mc>5 · ok_ally 5>=1 → true
cases.append(dict(strat=1, towers=0, twin=0, ek=2, gather=1000, ahp=1000, ebuff=1200, mc=5))  # mc 5 → 넥서스 경로
cases.append(dict(strat=1, towers=0, twin=0, ek=2, gather=1000, ahp=400, ebuff=5000, mc=6))   # hp 40% → ok (>39)
cases.append(dict(strat=1, towers=0, twin=0, ek=2, gather=1000, ahp=390, ebuff=5000, mc=6))   # hp 39% → not ok → ok_ally 0 < 1 → 넥서스 경로
cases.append(dict(strat=1, towers=0, twin=0, ek=1, gather=1000, ahp=390, ebuff=5000, mc=6))   # live_enemy 1 → sat_sub 0 → ok_ally 0>=0 true
cases.append(dict(strat=1, towers=0, twin=0, ek=2, nex=100000, ahp=1000))          # 전원 넥서스 근접 5 >= 3+2 → true
cases.append(dict(strat=1, towers=0, twin=0, ek=3, nex=100000, ahp=1000))          # 5 >= 3+3 false
cases.append(dict(strat=1, towers=0, twin=0, ek=3, nex=100000, ahp=1000, tut=5))   # MidBottom pc3 → edge 2 → 5>=5 true
cases.append(dict(strat=1, towers=0, twin=0, ek=3, nex=120000, ahp=1000, tut=3))   # Bottom pc2 → edge 1 → 4개(120k*5=600k 경계)
cases.append(dict(strat=1, towers=0, twin=0, ek=2, nex=120000, ahp=1000, tut=7))   # Line pc4 → edge 3 → near 4(600k 제외) >= 5? false
cases.append(dict(strat=1, towers=0, twin=0, ek=0, ahp=1000, cnone=1))             # champ None → unwrap 패닉 기대(관측)
for st in (0, 2):
    cases.append(dict(strat=st))
    cases.append(dict(strat=st, towers=0, twin=0, ek=0, gather=1000, ahp=1000))
    cases.append(dict(strat=st, towers=0, twin=0, ek=2, nex=100000, ahp=1000))
random.seed(255)
grid = dict(strat=[1], towers=[0, 0, 1], twin=[0, 0, 1], ek=[0, 1, 2, 3, 5], gather=[1000, 200000], ahp=[1000, 400, 390],
            ebuff=[0, 1199, 1200, 5000], mc=[0, 5, 6, 9], tut=[0, 3, 5, 7, 8], line=[0, 1, 2], tick=[3000], disable=[9999999, 2000])
keys = list(grid)
for _ in range(220):
    c = {k: random.choice(grid[k]) for k in keys}
    r = random.random()
    if r < 0.3: c['nex'] = random.choice([50000, 100000, 120000, 130000])
    elif r < 0.45: c['far'] = random.choice([0, 1, 4]); c['fard'] = random.choice([1190000, 1200000, 1200001, 2000000])
    elif r < 0.6: c['lowk'] = random.choice([0, 3]); c['lowhp'] = random.choice([390, 400, 100])
    elif r < 0.7: c['nexk'] = random.choice([0, 1]); c['nexkd'] = random.choice([100000, 600000, 600001])
    cases.append(c)

res = []
with ThreadPoolExecutor(max_workers=6) as ex:
    for case, out in ex.map(run, cases):
        res.append((case, out))
n_match = n_diff = n_obs = n_panic = 0; cov = collections.Counter()
with io.open(LOG, 'w', encoding='utf-8') as f:
    for case, out in res:
        s = out.rstrip()
        if s.endswith('MATCH'): v = 'MATCH'; n_match += 1
        elif s.endswith('OBS'): v = 'OBS'; n_obs += 1
        elif 'panicked' in out or not s: v = 'PANIC'; n_panic += 1
        else: v = 'DIFF'; n_diff += 1
        g = [l for l in out.splitlines() if l.startswith('got')]
        key = (v, g[0].split('\t')[1] if g else '?', (g[0].split('\t')[5].split(' ')[-2] if g and v == 'MATCH' and len(g[0].split('\t')) > 5 else ''))
        cov[key] += 1
        f.write("### %s\n%s\n%s\n" % (" ".join("%s=%s" % kv for kv in case.items()), v, s))
print("cases=%d MATCH=%d DIFF=%d OBS=%d PANIC=%d" % (len(res), n_match, n_diff, n_obs, n_panic))
for k, v in sorted(cov.items(), key=lambda x: str(x)): print("  %s : %d" % (k, v))
for case, out in res:
    s = out.rstrip()
    if not (s.endswith('MATCH') or s.endswith('OBS')):
        print("NON-MATCH:", " ".join("%s=%s" % kv for kv in case.items())); print(s[-500:])
print("->", LOG)
