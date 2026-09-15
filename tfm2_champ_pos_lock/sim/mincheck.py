"""mincheck.py — 최소 선택 인원수 공식 검증. ①홀(Hall) 기반 정확 필요치 계산기 ②적대적 상대(내 최약 라인 밴·픽)로 강제 위반 실측
python mincheck.py [--grid] [--seeds 60]

need(S) = |S| + 2b + Opp(S) + Lock(S),  N(S)=S 라인 후보 합집합, L=N(S) 챔프들이 갈 수 있는 라인 집합
  Opp(S)  = min(5, |L|)                          (이번 세트 상대 픽이 N(S) 에서 가져갈 수 있는 최대)
  Lock(S) = (SERIES-1) × { 0 | min(5,|L|) | 2·min(5,|L|) }  (클래식 | 피어리스 | 하드)
안전 ⇔ ∀S: |N(S)| ≥ need(S).  현행 공식 = 단일 라인 S={p} 근사(피어리스는 상대 픽 1 누락).
"""
import argparse, itertools, random, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bpsim
from bpsim import Scenario, Runner, PARAM_SETS, CLASSIC, FEARLESS, HARD
from poslock_math import MASK_ALL, max_match

SERIES = 5


def bits(m):
    return [p for p in range(5) if m & (1 << p)]


def exact_need(allowed, style, b, series=SERIES):
    """allowed: 5개 리스트. 반환 (min_slack, worst_S, table[S]=(|N|, need))"""
    champ_lines = {}
    for p in range(5):
        for c in allowed[p]:
            champ_lines.setdefault(c, set()).add(p)
    table = {}
    worst = (10**9, None)
    for r in range(1, 6):
        for S in itertools.combinations(range(5), r):
            N = set()
            for p in S:
                N |= set(allowed[p])
            L = set()
            for c in N:
                L |= champ_lines[c]
            opp = min(5, len(L))
            lock = (series - 1) * {0: 0, 1: min(5, len(L)), 2: 2 * min(5, len(L))}[style]
            need = len(S) + 2 * b + opp + lock
            table[S] = (len(N), need)
            slack = len(N) - need
            if slack < worst[0]:
                worst = (slack, S)
    return worst[0], worst[1], table


def current_formula(style, b):
    return {0: 2 + 2 * b, 1: 5 + 2 * b, 2: 10 + 2 * b}[style]


def corrected_formula(style, b, k):
    """단일/복수 라인 근사: 상대 픽이 N({p}) 에서 가져갈 수 있는 수 = k==1 이면 1, 아니면 5(후보가 5라인을 덮는 경우)."""
    opp = 1 if k == 1 else 5
    lock = (SERIES - 1) * {0: 0, 1: opp, 2: 2 * opp}[style]
    return 1 + 2 * b + opp + lock


# ── 적대적 상대: 내 최약 라인 후보를 밴·픽 ──
def install_adversary():
    orig_score = bpsim.base_score
    orig_human = bpsim.Runner.human_decide

    def weakest_line(runner, my_team):
        s = runner.match.sets[-1]
        own, _ = runner.match.locked_for(my_team)
        taken = s.taken() | set(own)
        best = None
        for p in range(5):
            n = sum(1 for c in runner.allowed[p] if c not in taken)
            if best is None or n < best[0]:
                best = (n, p)
        return best[1]

    def adv_score(seed, champ, team, turn_no, picks):
        r = bpsim._CUR
        if r is None or team == r.sc.user_team:
            return orig_score(seed, champ, team, turn_no, picks)
        p = weakest_line(r, r.sc.user_team)
        bonus = 1000.0 if champ in r.allowed[p] else 0.0
        return bonus + orig_score(seed, champ, team, turn_no, picks)
    bpsim.base_score = adv_score

    def human(self, team):
        # 공식이 밴 2b 전부를 적대로 치므로 내 밴도 최악(내 최약 라인 후보)으로 · 픽 = 클릭 가능 중 무작위(게이트가 안전 경로 유지)
        s = self.match.sets[-1]
        is_ban, side = s.turn()
        name = orig_human(self, team)
        if is_ban and name is not None:
            p = weakest_line(self, team)
            hurt = [c for c in self.allowed[p] if c not in s.taken()]
            if hurt:
                name = self.rng.choice(hurt)
        return name
    bpsim.Runner.human_decide = human
    orig_init = bpsim.Runner.__init__

    def init(self, sc, params):
        orig_init(self, sc, params)
        bpsim._CUR = self
    bpsim.Runner.__init__ = init
    bpsim._CUR = None


def my_fit_loss(runner):
    """내 팀 최종 픽의 라인 적합 손실(세트 합)."""
    loss = 0
    for s in runner.match.sets:
        my_blue = runner.sc.user_team == (runner.match.team1 if s.side else runner.match.team2)
        picks = s.blue_pick if my_blue else s.red_pick
        masks = [runner.ms.mask_of_name(n) for n in picks]
        loss += len(picks) - max_match(masks)
    return loss


GATE = 'exact'
def run_case(style, b, k, n_per, seeds, sets):
    """포지션당 n_per 개·챔프당 k 라인(균등 배정)·풀 = 5*n_per/k. 반환 (강제위반 시드수, 세트수, 정확슬랙, 현행판정, 보정판정)"""
    viol = 0
    slack_min = None
    for seed in range(seeds):
        rng = random.Random(seed)
        pool_n = max(5, (5 * n_per) // k)
        avail = [f"c{i:02d}" for i in range(pool_n)]
        # k 라인 균등 배정: 챔프 i → 라인 {i%5, (i+1)%5, ...}
        allowed = [[] for _ in range(5)]
        order = list(range(pool_n))
        rng.shuffle(order)
        for j, i in enumerate(order):
            for t in range(k):
                allowed[(j + t) % 5].append(avail[i])
        sl, S, _ = exact_need(allowed, style, b)
        slack_min = sl if slack_min is None else min(slack_min, sl)
        sc = Scenario(seed=seed, model_n=max(95, pool_n), avail_n=pool_n, lines_per_champ=k, per_pos=[n_per] * 5, style=style, ban_count=b,
                      sets=sets, user_team=1, delegate=False, frame_lag=0)
        r = Runner(sc, PARAM_SETS['v4_fix'])
        r.names = avail
        r.allowed = allowed
        # 로스터/마스크 재구성(균등 배정 반영)
        from modlayers import PosState, ModState, ModParams
        r.pos = PosState(allowed, set(avail), style, b)
        r.pos.gate = GATE
        r.ms = ModState(avail, r.model, r.pos, PARAM_SETS['v4_fix'])
        r.model_idx = {n: i for i, n in enumerate(r.model)}
        if not r.pos.any_restricted():
            return None  # 게이트가 제한을 아예 끔(최소 미달)
        r.ms.masks_names = [r.pos.mask_of(n) for n in avail]
        r.ms.mask_by_name = {n: m for n, m in zip(avail, r.ms.masks_names)}
        r.run()
        if my_fit_loss(r) > 0:
            viol += 1
    return viol, slack_min


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--seeds', type=int, default=40)
    ap.add_argument('--gate', default='exact')
    a = ap.parse_args()
    global GATE
    GATE = a.gate
    install_adversary()
    print("| 룰 | 밴 | 라인/챔프 | 포지션당 n | 풀 | 현행 최소 | 보정 최소 | 정확 슬랙(min) | 강제위반 시드/%d |" % a.seeds)
    print("|---|---|---|---|---|---|---|---|---|")
    for style in (CLASSIC, FEARLESS, HARD):
        sets = 1 if style == CLASSIC else 5
        for b in (1, 2, 3, 5):
            for k in (1, 2, 3):
                cur = current_formula(style, b)
                cor = corrected_formula(style, b, k)
                for n_per in sorted(set([cur - 1, cur, cor - 1, cor, cor + 2])):
                    if n_per < 1:
                        continue
                    res = run_case(style, b, k, n_per, a.seeds, sets)
                    if res is None:
                        print(f"| {style} | {b} | {k} | {n_per} | {max(5,5*n_per//k)} | {cur} | {cor} | - | (게이트 OFF) |")
                        continue
                    viol, sl = res
                    print(f"| {style} | {b} | {k} | {n_per} | {max(5,5*n_per//k)} | {cur} | {cor} | {sl} | {viol} |", flush=True)


if __name__ == '__main__':
    main()
