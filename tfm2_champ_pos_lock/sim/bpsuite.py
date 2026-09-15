"""bpsuite.py — 정적 테스트 스위트 B~E (bpsim 엔진 위). python bpsuite.py [--part B|C|D|E|all] [--out report.md]
 B 층간 일관성: 내 픽 차례(신선 프레임)에서 UI 회색 집합 == finalize v4 컷 집합
 C 결정적 격자: 룰×밴수×스타일×풀×라인×사람/AI×위임 전수(1시드) · v3/v4
 D 유저 설정 고정(31·[7,6,6,6,6]·클래식·밴2) 500시드 × 위임 × 프레임지연 · v3/v4
 E 스트레스: 낡은 계획 100% / 랜덤픽 100% / 차례 모호 100% / 타이머 없음 / 난이도 2
"""
import argparse, itertools, random, sys, os, time, io
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bpsim
from bpsim import Scenario, Runner, PARAM_SETS, CLASSIC, FEARLESS, HARD, PICKS_PER_RULE
from modlayers import ModParams
from poslock_math import MASK_ALL, helps, free_left

KEYS = ['turns', 'outside_pool', 'stall', 'user_allgrey', 'viol_avoidable', 'fit_loss', 'topk_has_veto', 'timeout_pick']


def run_one(sc, pname):
    r = Runner(sc, PARAM_SETS[pname])
    st = r.run()
    st['m_wrongmask'] = r.ms.counters['wrongmask']
    st['m_cp'] = r.ms.counters['cp_swap']
    st['incons'] = getattr(r, 'incons', 0)
    st['incons_checked'] = getattr(r, 'incons_checked', 0)
    return st


# ── B: 일관성 훅 ──
def install_consistency_check():
    orig = bpsim.Runner.human_decide

    def human_decide(self, team):
        s = self.match.sets[-1]
        is_ban, side = s.turn()
        if not is_ban and self.ms.blocklist is not None and self.p.finalize_ver == 'v4' and self.ms.model_captured:
            own, enemy = self.match.locked_for(team)
            taken = s.taken()
            legal = [n for n in self.names if n not in taken and n not in own]
            my_picks = s.blue_pick if team == self.match.team1_in_set() else s.red_pick
            pinned = [self.ms.mask_of_name(x) for x in my_picks if self.ms.mask_of_name(x) != MASK_ALL]
            pinned_all = [self.ms.mask_of_name(x) for x in my_picks]
            pool = [self.ms.mask_of_name(n) for n in legal]
            if not pinned or free_left(pinned_all, pool, PICKS_PER_RULE[s.rule] // 2) > 0:
                cut = set()
            else:
                allowed = [n for n in legal if self.ms.mask_of_name(n) == MASK_ALL or helps(pinned, self.ms.mask_of_name(n))]
                cut = set(legal) - set(allowed) if allowed else set()
            grey = set(self.ms.blocklist) & set(legal)
            self.incons_checked = getattr(self, 'incons_checked', 0) + 1
            if grey != cut:
                self.incons = getattr(self, 'incons', 0) + 1
        return orig(self, team)
    bpsim.Runner.human_decide = human_decide


def part_B(out):
    install_consistency_check()
    rng = random.Random(3)
    tot = {'checked': 0, 'incons': 0, 'runs': 0}
    for i in range(300):
        sc = Scenario(seed=rng.randint(1, 10**9), model_n=95, avail_n=rng.choice([20, 31, 45]), lines_per_champ=rng.choice([1, 2, 3]),
                      per_pos=[rng.randint(5, 12) for _ in range(5)], style=rng.choice([0, 1, 2]), ban_count=rng.choice([1, 2, 3, 5]),
                      sets=rng.choice([1, 3]), user_team=rng.choice([1, 2]), delegate=False, frame_lag=0)
        st = run_one(sc, 'v4_fix')
        tot['checked'] += st['incons_checked']; tot['incons'] += st['incons']; tot['runs'] += 1
    out.append(f"## B. 층간 일관성(UI 회색 == finalize v4 컷) — {tot['runs']}판 · 검사 {tot['checked']}회 · 불일치 **{tot['incons']}**")
    return tot['incons'] == 0


def part_C(out):
    grid = list(itertools.product([3, 2, 1], [1, 2, 3, 5], [CLASSIC, FEARLESS, HARD], [12, 20, 31, 45, 95], [1, 2, 3], [0, 1, 2], [False, True]))
    res = {}
    t0 = time.time()
    for k, (rule, ban, style, avail, lines, user, deleg) in enumerate(grid):
        if user == 0 and deleg:
            continue
        base = max(avail * lines // 5, 1)
        sc = Scenario(seed=777, model_n=95, avail_n=avail, avail_mode='all' if avail == 95 else 'subset', lines_per_champ=lines,
                      per_pos=[base] * 5, style=style, ban_count=ban, rule=rule, sets=3 if style else 1, user_team=user, delegate=deleg, frame_lag=1)
        for pname in ('v3_now', 'v4_fix'):
            st = run_one(sc, pname)
            for dim, val in (('rule', rule), ('ban', ban), ('style', style), ('avail', avail), ('lines', lines), ('user', user), ('deleg', deleg)):
                key = (pname, dim, val)
                d = res.setdefault(key, {k: 0 for k in KEYS + ['runs', 'm_wrongmask']})
                d['runs'] += 1
                for kk in KEYS + ['m_wrongmask']:
                    d[kk] += st.get(kk, 0)
        if k % 200 == 0:
            print(f"  C {k}/{len(grid)} {time.time()-t0:.0f}s", flush=True)
    out.append("## C. 결정적 격자 전수 (룰3×밴4×스타일3×풀5×라인3×사람3×위임2 = 3,240조합 · 1시드 · v3/v4)")
    out.append("| 파라미터 | 차원 | 값 | 판 | 풀밖픽 | 정지 | 전부회색 | 회피가능위반 | fit손실 | wrongmask |")
    out.append("|---|---|---|---|---|---|---|---|---|---|")
    for (pname, dim, val), d in sorted(res.items(), key=lambda x: (x[0][0], x[0][1], str(x[0][2]))):
        out.append(f"| {pname} | {dim} | {val} | {d['runs']} | {d['outside_pool']} | {d['stall']} | {d['user_allgrey']} | {d['viol_avoidable']} | {d['fit_loss']} | {d['m_wrongmask']} |")
    v4tot = {k: sum(d[k] for (p, dim, _), d in res.items() if p == 'v4_fix' and dim == 'rule') for k in KEYS}
    v3tot = {k: sum(d[k] for (p, dim, _), d in res.items() if p == 'v3_now' and dim == 'rule') for k in KEYS}
    out.append(f"\n합계 v3: {v3tot}\n합계 v4: {v4tot}")
    return v4tot


def part_D(out):
    rows = []
    for deleg in (False, True):
        for lag in (0, 1, 2):
            for pname in ('v3_now', 'v4_fix'):
                agg = {k: 0 for k in KEYS + ['m_wrongmask', 'm_cp']}
                for seed in range(500):
                    sc = Scenario(seed=50000 + seed, model_n=95, avail_n=31, per_pos=[7, 6, 6, 6, 6], lines_per_champ=1, style=CLASSIC,
                                  ban_count=2, user_team=1 + (seed % 2), delegate=deleg, frame_lag=lag)
                    st = run_one(sc, pname)
                    for k in agg:
                        agg[k] += st.get(k, 0)
                rows.append((deleg, lag, pname, agg))
    out.append("## D. 유저 설정 고정(31챔프·[7,6,6,6,6]·클래식·밴2·1라인) 500시드 × 위임 × 프레임지연")
    out.append("| 위임 | lag | 파라미터 | AI결정 | 풀밖픽 | 정지 | 전부회색 | 회피가능위반 | fit손실 | topK내veto | wrongmask | cprod교체 |")
    out.append("|---|---|---|---|---|---|---|---|---|---|---|---|")
    for deleg, lag, pname, a in rows:
        out.append(f"| {deleg} | {lag} | {pname} | {a['turns']} | {a['outside_pool']} | {a['stall']} | {a['user_allgrey']} | {a['viol_avoidable']} | {a['fit_loss']} | {a['topk_has_veto']} | {a['m_wrongmask']} | {a['m_cp']} |")


def part_E(out):
    cases = {
        '낡은계획100%': dict(p_stale=1.0), '랜덤픽100%': dict(p_random=1.0), '차례모호100%': dict(p_ambiguous=1.0),
        '타이머없음+낡은계획': dict(timeout=False, p_stale=0.7), '난이도2(topK=2)': dict(difficulty=2), '난이도0(topK=6)+2라인': dict(difficulty=0, lines_per_champ=2, per_pos=[12, 12, 12, 13, 13]),
        '미지정10+빈포지션2': dict(unassigned=10, empty_pos=[3, 4], per_pos=[7, 7, 7, 0, 0]), '하드Bo5밴5(풀31)': dict(style=HARD, ban_count=5, sets=5),
        '피어리스Bo3밴3': dict(style=FEARLESS, ban_count=3, sets=3), 'AI전용(백그라운드)': dict(user_team=0), '3v3': dict(rule=1), '2v2': dict(rule=0),
    }
    out.append("## E. 스트레스 케이스(각 200시드)")
    out.append("| 케이스 | 파라미터 | AI결정 | 풀밖픽 | 정지 | 타임아웃구제 | 전부회색 | 회피가능위반 | fit손실 | wrongmask |")
    out.append("|---|---|---|---|---|---|---|---|---|---|")
    for name, kw in cases.items():
        for pname in ('v3_now', 'v4_fix', 'off'):
            agg = {k: 0 for k in KEYS + ['m_wrongmask']}
            for seed in range(200):
                base = dict(model_n=95, avail_n=31, per_pos=[7, 6, 6, 6, 6], lines_per_champ=1, style=CLASSIC, ban_count=2, user_team=1, delegate=(seed % 2 == 1), frame_lag=1)
                base.update(kw)
                sc = Scenario(seed=90000 + seed, **base)
                st = run_one(sc, pname)
                for k in agg:
                    agg[k] += st.get(k, 0)
            out.append(f"| {name} | {pname} | {agg['turns']} | {agg['outside_pool']} | {agg['stall']} | {agg['timeout_pick']} | {agg['user_allgrey']} | {agg['viol_avoidable']} | {agg['fit_loss']} | {agg['m_wrongmask']} |")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--part', default='all')
    ap.add_argument('--out', default=None)
    a = ap.parse_args()
    out = [f"# 정적 테스트 스위트 결과 ({time.strftime('%Y-%m-%d %H:%M')})\n"]
    parts = ['B', 'C', 'D', 'E'] if a.part == 'all' else [a.part]
    for p in parts:
        t0 = time.time()
        globals()['part_' + p](out)
        out.append(f"\n(part {p}: {time.time()-t0:.0f}s)\n")
        print('\n'.join(out[-3:]), flush=True)
    txt = '\n'.join(out) + '\n'
    if a.out:
        io.open(a.out, 'w', encoding='utf-8').write(txt)
    else:
        print(txt)


if __name__ == '__main__':
    main()
