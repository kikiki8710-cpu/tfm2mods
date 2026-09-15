"""bpsim.py — 밴픽 정적 시뮬레이터: 게임 프로토콜(draft.py) + AI 결정 경로(resolve 3갈래) + 모드 층(modlayers.py)을 한 엔진에서 돌려
시나리오 대량 실행·불변식 검사. python bpsim.py [--n 2000] [--seed 1] [--params buggy|correct|v4|all] [--quick]

불변식(세트마다):
  I1 활성 풀 밖(디폴트 챔프) 픽 0          I2 커밋 거부(정지) 0          I3 중복/피어리스 위반 0(게임 보장, 확인만)
  I4 최종 5픽 라인 적합 수 = 그 픽 조합의 최대 매칭(모드가 더 나쁘게 만들지 않음) + "정배치 가능했는데 못 함" 카운트
  I5 사람 차례에 합법 후보가 있는데 전부 회색 0     I6 resolve 폴백(avail0/scan) 발생 시 결과가 활성 풀 안인지
"""
import argparse, itertools, json, random, sys, os
from dataclasses import dataclass, field, asdict
from typing import List, Optional, Set
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from draft import Match, SetInfo, CLASSIC, FEARLESS, HARD, PICKS_PER_RULE, resolve_banpick_candidate, is_legal_banpick_candidate
from modlayers import ModParams, ModState, PosState, score_pick, recompute_blocklist, finalize_hook, cprod_swap, NEG
from poslock_math import MASK_ALL, max_match, feasible, helps, free_left

POS = ['top', 'jungle', 'mid', 'bottom', 'support']


# ───────────────────────── 시나리오 ─────────────────────────
@dataclass
class Scenario:
    seed: int
    model_n: int = 95            # AI 모델 챔프 수
    avail_n: int = 31            # 활성 풀 크기(≤ model_n)
    avail_mode: str = 'subset'   # 'subset'(모델 앞부분 아님·무작위 부분집합) | 'all'(활성=모델)
    lines_per_champ: int = 1     # 챔프당 지정 라인 수(0=미지정 섞기)
    per_pos: List[int] = field(default_factory=lambda: [7, 6, 6, 6, 6])
    unassigned: int = 0          # 활성 풀 중 미지정 수
    empty_pos: List[int] = field(default_factory=list)   # 목록 비운 포지션
    style: int = CLASSIC
    ban_count: int = 2
    rule: int = 3
    sets: int = 1                # 세트 수(Bo1=1, Bo3 최대 3, Bo5 최대 5 — 여기선 정확히 n세트 진행)
    user_team: int = 1           # 1|2, 0=사람 없음(AI vs AI)
    delegate: bool = False
    p_stale: float = 0.0         # recommend 가 낡은 계획(이미 taken 일 수 있음)을 내는 확률
    p_random: float = 0.0        # 랜덤픽 플래그 경로 확률
    p_ambiguous: float = 0.0     # UI 차례 판정 모호 확률
    frame_lag: int = 1
    timeout: bool = True         # 타임아웃 타이머 존재(정지 대신 랜덤)
    difficulty: int = 0          # 0/1/2 → top-K 6/4/2
    user_policy: str = 'random'  # 'random'|'adversarial'(내 목록에 있는 챔프를 상대가 원하게 유도)


def make_world(sc: Scenario, rng: random.Random):
    model = [f"c{i:02d}" for i in range(sc.model_n)]
    if sc.avail_mode == 'all':
        avail = list(model)
    else:
        idx = sorted(rng.sample(range(sc.model_n), sc.avail_n))
        avail = [model[i] for i in idx]
    names = list(avail)  # NAMES = db.available_champions 순서(=모델 순서의 부분열)
    # 포지션 목록
    allowed = [[] for _ in range(5)]
    pool = list(avail)
    rng.shuffle(pool)
    un = pool[:sc.unassigned]
    rest = pool[sc.unassigned:]
    if sc.lines_per_champ >= 1:
        # 각 챔프에 lines_per_champ 개 라인 배정하되 per_pos 상한 근사
        cap = list(sc.per_pos)
        for c in rest:
            ps = [p for p in range(5) if cap[p] > 0 and p not in sc.empty_pos]
            rng.shuffle(ps)
            take = ps[:sc.lines_per_champ]
            if not take:
                take = [p for p in range(5) if p not in sc.empty_pos][:1]
            for p in take:
                allowed[p].append(c)
                cap[p] -= 1
    for p in sc.empty_pos:
        allowed[p] = []
    return model, names, allowed


# ───────────────────────── 점수(블랙박스) ─────────────────────────
def base_score(rng_seed, champ, team, turn_no, picks):
    h = hash((rng_seed, champ, team, turn_no % 3, len(picks))) & 0xffffffff
    return (h % 10000) / 100.0


# ───────────────────────── 한 세트 실행 ─────────────────────────
class Runner:
    def __init__(self, sc: Scenario, params: ModParams):
        self.sc = sc
        self.p = params
        self.rng = random.Random(sc.seed)
        self.model, self.names, self.allowed = make_world(sc, self.rng)
        self.pos = PosState(self.allowed, set(self.names), sc.style, sc.ban_count)
        self.ms = ModState(self.names, self.model, self.pos, params)
        self.model_idx = {n: i for i, n in enumerate(self.model)}
        self.match = Match(team1=1, team2=2, style=sc.style, rule=sc.rule)
        self.match.sets.append(SetInfo(ban_count=sc.ban_count, rule=sc.rule, side=True))
        self.stats = {k: 0 for k in ['turns', 'stall', 'outside_pool', 'path_pref', 'path_avail0', 'path_scan', 'path_none',
                                     'timeout_pick', 'user_allgrey', 'fit_loss', 'fit_free', 'dup', 'fearless_viol', 'stale_hit', 'topk_has_veto', 'viol_avoidable']}
        self.plan = {}         # team -> 낡은 계획 인덱스
        self.hold_decisions = 0
        self.lineups = []

    # ── UI 프레임 ──
    def ui_frame(self, scene_ok=True):
        s = self.match.sets[-1]
        t = s.turn()
        if not scene_ok or t is None:
            return recompute_blocklist(self.ms, {'scene_ok': False})
        is_ban, side = t
        my_team = self.sc.user_team
        if my_team == 0:
            return recompute_blocklist(self.ms, {'scene_ok': False})
        turn_team = self.match.team1_in_set() if side == 0 else self.match.team2_in_set()
        my_is_blue = (my_team == self.match.team1_in_set())
        my_picks = s.blue_pick if my_is_blue else s.red_pick
        if self.rng.random() < self.sc.p_ambiguous:
            known = None
        else:
            known = (turn_team == my_team)
        own_locked, _ = self.match.locked_for(my_team)
        return recompute_blocklist(self.ms, {'scene_ok': True, 'is_ban': is_ban, 'my_turn_known': known,
                                             'my_picks': list(my_picks), 'taken': s.taken(), 'own_locked': own_locked,
                                             'picks_n': PICKS_PER_RULE[s.rule], 'total': s.total()})

    # ── AI 결정(코치·상대·위임 공통) ──
    def ai_decide(self, team):
        s = self.match.sets[-1]
        is_ban, side = s.turn()
        own, enemy = self.match.locked_for(team)
        lock = enemy if is_ban else own
        taken = s.taken()
        my_picks = s.blue_pick if team == self.match.team1_in_set() else s.red_pick
        # available(order) = 활성 ∖ taken ∖ lock  (모델 인덱스)
        available_idx = [self.model_idx[n] for n in self.names if n not in taken and n not in lock]
        ally_idx = [self.model_idx[n] for n in my_picks]
        ctx = {'available': available_idx, 'ally_pick': ally_idx}
        turn_no = s.total()
        scored = None
        select = -1
        if available_idx and self.rng.random() < self.sc.p_random:
            select = self.rng.choice(available_idx)          # 랜덤픽 플래그 경로(recommend 생략)
            runnerup = []
        else:
            scored = []
            for c in available_idx:
                b = base_score(self.sc.seed, self.model[c], team, turn_no, my_picks)
                r = score_pick(self.ms, ctx, c, b) if not is_ban else None
                scored.append((c, b if r is None else NEG, b))
            # choose_scored_candidate(composite.rs): 내림차순(동점 idx 오름차순) → K=max(1,min(n,6-2*난이도)) → 균등 1개
            ranked = sorted(scored, key=lambda x: (-x[1], x[0]))
            if ranked:
                K = max(1, min(len(ranked), 6 - 2 * self.sc.difficulty))
                select = self.rng.choice(ranked[:K])[0]
                if any(x[1] <= NEG for x in ranked[:K]):
                    self.stats['topk_has_veto'] += 1
            runnerup = [self.model[c] for c, _, _ in ranked]
            # 낡은 계획: 이전에 세운 계획을 고집(이미 taken 일 수 있음)
            if not is_ban and self.rng.random() < self.sc.p_stale:
                if team in self.plan:
                    select = self.plan[team]
                    self.stats['stale_hit'] += 1
                elif scored:
                    self.plan[team] = select
        # 모드 finalize pre-call
        rep = finalize_hook(self.ms, self.model, available_idx, select, taken, list(my_picks), is_ban)
        if rep is not None:
            available_idx2, select2 = rep
        else:
            available_idx2, select2 = available_idx, select
        name, path = resolve_banpick_candidate(self.model, available_idx2, select2, s, own, enemy, is_ban)
        self.stats['path_' + path] += 1
        if name is None:
            return None
        # cprod tag6 이름 교체(모드)
        name2 = cprod_swap(self.ms, name, list(my_picks), taken | set(lock), False, is_ban, runnerup)
        # 회피 가능한 라인 위반 픽(불변식 I4'): 새 라인을 못 채우는데 자유 슬롯도 없고, 채울 수 있는 합법 후보가 있었다
        if not is_ban and name2 in self.names:
            pinned = [self.ms.mask_of_name(x) for x in my_picks if self.ms.mask_of_name(x) != MASK_ALL]
            pinned_all = [self.ms.mask_of_name(x) for x in my_picks]
            legal = [self.model[c] for c in available_idx]
            pool = [self.ms.mask_of_name(n) for n in legal]
            m = self.ms.mask_of_name(name2)
            if m != MASK_ALL and not helps(pinned, m) and free_left(pinned_all, pool, 5) == 0:
                if any(helps(pinned, self.ms.mask_of_name(n)) for n in legal):
                    self.stats['viol_avoidable'] += 1
        return name2

    # ── 사람 픽 ──
    def human_decide(self, team):
        s = self.match.sets[-1]
        is_ban, side = s.turn()
        own, enemy = self.match.locked_for(team)
        lock = enemy if is_ban else own
        taken = s.taken()
        legal = [n for n in self.names if n not in taken and n not in lock]
        if not legal:
            return None
        grey = self.ms.blocklist or set()
        clickable = [n for n in legal if is_ban or n not in grey]
        if not clickable:
            self.stats['user_allgrey'] += 1
            clickable = legal  # 실제론 클릭 불가 = 정지. 검사용으로 기록만 하고 진행.
        return self.rng.choice(clickable)

    def commit(self, team, name, human):
        s = self.match.sets[-1]
        if name not in self.names:
            self.stats['outside_pool'] += 1
        ok = self.match.select_champion(team, name)
        if not ok:
            self.stats['stall'] += 1
            if self.sc.timeout:
                own, enemy = self.match.locked_for(team)
                cands = self.match.timeout_candidates(team, self.names)
                if cands:
                    self.stats['timeout_pick'] += 1
                    self.match.select_champion(team, self.rng.choice(cands))
                    return True
            return False
        return True

    def run_set(self):
        s = self.match.sets[-1]
        self.ms.blocklist = None
        self.ms.my_pick_turn = False
        self.ui_frame(True)
        pending_hold = 0
        guard = 0
        while s.turn() is not None and guard < 100:
            guard += 1
            is_ban, side = s.turn()
            team = self.match.team1_in_set() if side == 0 else self.match.team2_in_set()
            human = (team == self.sc.user_team) and not self.sc.delegate and self.sc.user_team != 0
            # UI 프레임: 직전 픽 뒤 frame_lag 개의 **AI 결정** 동안은 hold(전환 창). 사람은 전환 연출이 끝난 뒤에야
            #   클릭할 수 있으므로 사람 차례는 항상 신선한 프레임(hold 는 워커 결정에만 영향).
            if pending_hold > 0 and not human:
                self.ui_frame(scene_ok=False)
                pending_hold -= 1
            else:
                self.ui_frame(True)
                pending_hold = 0
            if human:
                name = self.human_decide(team)
                if name is None:
                    break
                self.commit(team, name, True)
                pending_hold = self.sc.frame_lag
            else:
                name = self.ai_decide(team)
                self.stats['turns'] += 1
                if name is None:
                    self.stats['stall'] += 1
                    break
                self.commit(team, name, False)
                # 위임/연속 AI 결정도 UI 갱신 전에 나갈 수 있음
                if team == self.sc.user_team and self.sc.frame_lag > 0:
                    pending_hold = self.sc.frame_lag
        # 세트 결과 검사
        for picks in (s.blue_pick, s.red_pick):
            masks = [self.ms.mask_of_name(n) for n in picks]
            fit = max_match(masks)
            self.lineups.append((list(picks), fit))
            if fit < len(picks):
                # 정배치가 가능했는가? (풀에서 이 팀이 고를 수 있었던 조합 존재 여부는 별도 — 여기선 자유슬롯 여부만)
                self.stats['fit_loss'] += len(picks) - fit
        # 중복/피어리스 확인(게임 보장)
        allp = s.blue_ban + s.red_ban + s.blue_pick + s.red_pick
        if len(set(allp)) != len(allp):
            self.stats['dup'] += 1
        b, r = self.match.fearless_locked_in_set()
        if any(x in b for x in s.blue_pick) or any(x in r for x in s.red_pick):
            self.stats['fearless_viol'] += 1

    def run(self):
        for k in range(self.sc.sets):
            if k > 0:
                loser_is_t1 = self.rng.random() < 0.5
                self.match.new_set(self.rng.random() < 0.5, loser_is_t1)
            self.run_set()
        return self.stats


# ───────────────────────── 시나리오 생성 ─────────────────────────
def gen_scenarios(n, seed):
    rng = random.Random(seed)
    out = []
    # 1) 결정적 엣지 케이스
    edge = [
        dict(model_n=95, avail_n=31, per_pos=[7, 6, 6, 6, 6], lines_per_champ=1, style=CLASSIC, ban_count=2, user_team=1, delegate=False),
        dict(model_n=95, avail_n=31, per_pos=[7, 6, 6, 6, 6], lines_per_champ=1, style=CLASSIC, ban_count=2, user_team=2, delegate=True),
        dict(model_n=95, avail_n=31, per_pos=[13, 12, 12, 12, 13], lines_per_champ=2, style=CLASSIC, ban_count=2, user_team=1),
        dict(model_n=95, avail_n=95, avail_mode='all', per_pos=[21, 21, 21, 21, 16], lines_per_champ=3, style=HARD, ban_count=3, sets=3, user_team=1, delegate=True),
        dict(model_n=95, avail_n=31, per_pos=[7, 6, 6, 6, 6], lines_per_champ=1, style=HARD, ban_count=3, sets=3, user_team=1),
        dict(model_n=95, avail_n=31, per_pos=[7, 6, 6, 6, 6], lines_per_champ=1, style=FEARLESS, ban_count=5, sets=5, user_team=1, delegate=True),
        dict(model_n=95, avail_n=20, per_pos=[4, 4, 4, 4, 4], lines_per_champ=1, style=CLASSIC, ban_count=5, user_team=1),   # 극단 고갈
        dict(model_n=95, avail_n=12, per_pos=[3, 3, 2, 2, 2], lines_per_champ=1, style=CLASSIC, ban_count=1, user_team=1),   # 12개·최소만
        dict(model_n=95, avail_n=31, per_pos=[31, 0, 0, 0, 0], lines_per_champ=1, empty_pos=[1, 2, 3, 4], style=CLASSIC, ban_count=2, user_team=1),  # 탑만 제한
        dict(model_n=95, avail_n=31, per_pos=[7, 6, 6, 6, 6], lines_per_champ=1, unassigned=10, style=CLASSIC, ban_count=2, user_team=1),  # 미지정 섞임
        dict(model_n=95, avail_n=31, per_pos=[7, 6, 6, 6, 6], lines_per_champ=1, style=CLASSIC, ban_count=2, user_team=0),   # AI vs AI(백그라운드)
        dict(model_n=95, avail_n=31, per_pos=[7, 6, 6, 6, 6], lines_per_champ=1, style=CLASSIC, ban_count=2, user_team=1, p_stale=0.5, p_random=0.2, p_ambiguous=0.2, frame_lag=2),
        dict(model_n=95, avail_n=31, per_pos=[7, 6, 6, 6, 6], lines_per_champ=1, style=CLASSIC, ban_count=2, user_team=1, timeout=False, p_stale=0.5),
        dict(model_n=40, avail_n=40, avail_mode='all', per_pos=[8, 8, 8, 8, 8], lines_per_champ=2, style=FEARLESS, ban_count=3, sets=3, user_team=2),
        dict(model_n=95, avail_n=31, per_pos=[7, 6, 6, 6, 6], lines_per_champ=1, rule=1, style=CLASSIC, ban_count=2, user_team=1),  # 3v3
    ]
    for i, e in enumerate(edge):
        out.append(Scenario(seed=1000 + i, **e))
    # 2) 무작위
    for i in range(n):
        model_n = rng.choice([95, 95, 119, 60])
        mode = rng.choice(['subset', 'subset', 'all'])
        avail_n = model_n if mode == 'all' else rng.choice([12, 20, 31, 31, 45, 60])
        avail_n = min(avail_n, model_n)
        lines = rng.choice([1, 1, 2, 2, 3])
        base = max(avail_n * lines // 5, 1)
        per_pos = [max(base + rng.randint(-2, 2), 0) for _ in range(5)]
        empty = [p for p in range(5) if rng.random() < 0.08]
        sc = Scenario(seed=rng.randint(1, 10**9), model_n=model_n, avail_n=avail_n, avail_mode=mode, lines_per_champ=lines,
                      per_pos=per_pos, unassigned=rng.choice([0, 0, 0, 3, 8]), empty_pos=empty,
                      style=rng.choice([CLASSIC, CLASSIC, FEARLESS, HARD]), ban_count=rng.choice([1, 2, 2, 3, 5]),
                      rule=rng.choice([3, 3, 3, 1, 2]), sets=rng.choice([1, 1, 3, 5]), user_team=rng.choice([0, 1, 2]),
                      delegate=rng.random() < 0.4, p_stale=rng.choice([0, 0, 0.3, 0.6]), p_random=rng.choice([0, 0, 0.1]),
                      p_ambiguous=rng.choice([0, 0, 0.1]), frame_lag=rng.choice([0, 1, 1, 2]), timeout=rng.random() < 0.85,
                      difficulty=rng.choice([0, 0, 1, 2]))
        out.append(sc)
    return out


PARAM_SETS = {
    'v1_pre':  ModParams(index_space='buggy', cover_gate=False, finalize_ver='v1'),     # 09-13 이전(원 버그)
    'v3_now':  ModParams(index_space='buggy', cover_gate=True, finalize_ver='v3'),      # 현행 배포본
    'v4_fix':  ModParams(index_space='correct', cover_gate=True, finalize_ver='v4', ui_fearless_aware=True),    # 수정안
    'off':     ModParams(enabled=False),
}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--n', type=int, default=600)
    ap.add_argument('--seed', type=int, default=1)
    ap.add_argument('--params', default='all')
    ap.add_argument('--json', default=None)
    a = ap.parse_args()
    scs = gen_scenarios(a.n, a.seed)
    sets = PARAM_SETS if a.params == 'all' else {a.params: PARAM_SETS[a.params]}
    report = {}
    for pname, pr in sets.items():
        agg = {}
        per = []
        for sc in scs:
            r = Runner(sc, ModParams(**asdict(pr)) if hasattr(pr, '__dataclass_fields__') else pr)
            st = r.run()
            st.update({'m_' + k: v for k, v in r.ms.counters.items()})
            for k, v in st.items():
                agg[k] = agg.get(k, 0) + v
            per.append((sc, st))
        report[pname] = {'agg': agg, 'scenarios': len(scs)}
        bad = [(sc, st) for sc, st in per if st['outside_pool'] or st['stall'] or st['user_allgrey']]
        print(f"=== {pname}: {len(scs)} 시나리오 · AI결정 {agg['turns']} · 풀밖픽 {agg['outside_pool']} · 정지 {agg['stall']}(타임아웃구제 {agg['timeout_pick']}) · 전부회색 {agg['user_allgrey']} · fit손실 {agg['fit_loss']} · 회피가능위반픽 {agg['viol_avoidable']} · topK내veto {agg['topk_has_veto']} · resolve(pref/avail0/scan/none)={agg['path_pref']}/{agg['path_avail0']}/{agg['path_scan']}/{agg['path_none']} · veto {agg['m_veto']} cover {agg['m_cover']} skip {agg['m_cover_skip']} wrongmask {agg['m_wrongmask']} fz(filt/v4/scored/noscore)={agg['m_fz_filt']}/{agg['m_fz_v4_filt']}/{agg['m_fz_scored']}/{agg['m_fz_noscore']} cp {agg['m_cp_swap']}/{agg['m_cp_fail']}")
        for sc, st in bad[:8]:
            print("   !! seed=%d avail=%d/%d lines=%d per_pos=%s style=%d ban=%d sets=%d user=%d deleg=%s stale=%.1f rnd=%.1f lag=%d → %s" % (
                sc.seed, sc.avail_n, sc.model_n, sc.lines_per_champ, sc.per_pos, sc.style, sc.ban_count, sc.sets, sc.user_team, sc.delegate,
                sc.p_stale, sc.p_random, sc.frame_lag, {k: v for k, v in st.items() if v and k in ('outside_pool', 'stall', 'user_allgrey', 'timeout_pick', 'path_scan', 'path_avail0')}))
    if a.json:
        json.dump(report, open(a.json, 'w'), indent=1)


if __name__ == '__main__':
    main()
