"""run27D.py — o27D_v30.exe 를 케이스당 프로세스 1개로 돌려 MATCH/DIFF 집계 + (tag,obs) 커버리지. 산출 = oracle/o27D_v30.log"""
import subprocess, random, itertools, sys, io, os, collections
from concurrent.futures import ThreadPoolExecutor
EXE = os.path.expandvars(r"%LOCALAPPDATA%\Temp\tfm2_spanprobe\o27D_v30.exe")
HERE = os.path.dirname(os.path.abspath(__file__))
LOG = os.path.join(HERE, "o27D_v30.log")

def run(case):
    args = [EXE] + ["%s=%s" % kv for kv in case.items()]
    try:
        out = subprocess.run(args, capture_output=True, text=True, timeout=120, encoding='utf-8', errors='replace').stdout
    except Exception as e:
        out = "EXC %s" % e
    return case, out

cases = []
# 특수 경로
cases.append(dict(ftm=1))                       # 같은 팀 → None · 미기록
cases.append(dict(mr=0))                        # max_range 0 → None · 미기록
cases.append(dict(mr=30000, fdx=100000, invis=1))   # 비가시(Invisible) → None · 미기록
cases.append(dict(mr=30000, fdx=100000, invis=2))   # Unknown → None · 미기록
cases.append(dict(mr=30000, fdx=100000))            # 가시 · 사거리 밖 → stance 투영
cases.append(dict(mr=30000, fdx=10000))             # 사거리 안 · 미니언 없음 → obs 1
cases.append(dict(ticks=0, nearm=0))                # 미니언 0 (nearm 실패) → obs 1
cases.append(dict(nearm=0, chp=500, cmhp=1000, fdx=10000, cms=1000, fms=900, version=1))  # version 1 경로
cases.append(dict(nearm=0, chp=100, cmhp=1000, fdx=60000, mr=30000, cms=1000, fms=900, version=1, inr=0))
random.seed(27)
grid = dict(chp=[20, 40, 60, 90, 120, 200, 300, 500, 1000], cmhp=[1000], fdx=[3000, 10000, 40000, 60000, 120000, 200000],
            mr=[30000, 100000], inr=[0, 1], cr=[0, 1], rto=[0, 1], oe=[0, 1], fdt=[5, 20, 100, 400], mdt=[5, 40, 100, 400],
            thr=[10, 25, 26, 50], cms=[600, 1000], fms=[900], mdx=[0, 15000, 30000], nearm=[0, 4, 8])
keys = list(grid)
for _ in range(360):
    c = {k: random.choice(grid[k]) for k in keys}
    cases.append(c)

res = []
with ThreadPoolExecutor(max_workers=6) as ex:
    for case, out in ex.map(run, cases):
        res.append((case, out))

cov = collections.Counter(); n_match = n_diff = 0
with io.open(LOG, 'w', encoding='utf-8') as f:
    for case, out in res:
        got = [l for l in out.splitlines() if l.startswith('got')]
        pred = [l for l in out.splitlines() if l.startswith('pred')]
        verdict = 'MATCH' if '\nMATCH' in out or out.endswith('MATCH') or out.rstrip().endswith('MATCH') else 'DIFF'
        if verdict == 'MATCH': n_match += 1
        else: n_diff += 1
        tag = obs = '?'
        if got:
            parts = dict(p.split('=') for p in got[0].split('\t')[1:])
            tag, obs = parts.get('tag'), parts.get('obs')
        cov[(tag, obs)] += 1
        f.write("### %s\n%s\n%s\n" % (" ".join("%s=%s" % kv for kv in case.items()), verdict, out.strip()))
print("cases=%d MATCH=%d DIFF=%d" % (len(res), n_match, n_diff))
for k, v in sorted(cov.items(), key=lambda x: str(x)): print("  tag=%s obs=%s : %d" % (k[0], k[1], v))
for case, out in res:
    if not out.rstrip().endswith('MATCH'):
        print("DIFF:", " ".join("%s=%s" % kv for kv in case.items()))
        print(out.strip()[-600:])
print("->", LOG)
