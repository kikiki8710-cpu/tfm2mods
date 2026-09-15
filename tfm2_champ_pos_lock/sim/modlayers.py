"""modlayers.py — tfm2_champ_pos_lock 개입 층의 정적 재현. 근거 = RE\\2026-09-15_모드훅-시뮬레이터명세.md (§1~§8) + lib.rs/hooks.rs.
파라미터로 현행(v0.6.0 3차)과 수정안(v0.6.1: 모델 공간 마스크 · finalize 자체 합법 판정)을 같은 엔진에서 비교한다.

인덱스 공간
  MODEL  : AI 모델 챔피언 목록(finalize champion_list / DraftScoreContext 슬라이스 / Hook A rdx) — 크기 model_n
  NAMES  : 모드 로스터(db.available_champions 순서) — 크기 avail_n ≤ model_n
  현행 모드는 NAMES 인덱스 테이블을 MODEL 인덱스로 조회한다(index_space='buggy'); 수정안은 이름으로 조인(index_space='correct').
"""
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Set
from poslock_math import MASK_ALL, feasible, helps, free_left, max_match

NEG = -1.0e9


# ───────────────────────── config.rs ─────────────────────────
def min_required(style, ban_count):
    """구 공식(config.rs 09-16 이전): 클래식 2+2b / 피어리스 5+2b / 하드 10+2b"""
    if style == 0:
        return 2 + ban_count * 2
    if style == 1:
        return 5 + ban_count * 2
    return 10 + ban_count * 2


@dataclass
class PosState:
    allowed: List[List[str]]            # 5개 포지션별 허용 챔프(소문자)
    roster: Set[str]                    # NAMES 집합
    style: int = 0
    ban_count: int = 2

    def named_count(self, p):
        return sum(1 for c in self.allowed[p] if c in self.roster)

    def live_count(self, p):
        named = self.named_count(p)
        if named == 0:
            return 0
        unassigned = sum(1 for c in self.roster if not any(c in self.allowed[q] for q in range(5)))
        return named + unassigned

    gate: str = 'exact'   # 'old'(live_count >= min_required) | 'exact'(홀 정리 부분집합식, 2026-09-16)

    def _safety(self):
        if getattr(self, '_saf', None) is not None:
            return self._saf
        des = {}
        for c in self.roster:
            m = 0
            for p in range(5):
                if c in self.allowed[p]:
                    m |= 1 << p
            des[c] = m
        named = sum(1 << p for p in range(5) if self.named_count(p) > 0)

        def table(a):
            free = MASK_ALL & ~a
            eff = [((d & a) if (d & a) else (free if free else MASK_ALL)) for d in des.values()]
            have = [0] * 32; need = [0] * 32
            for S in range(1, 32):
                if S & a != S:
                    continue
                n = 0; l = 0
                for m in eff:
                    if m & S:
                        n += 1; l |= m
                opp = min(5, bin(l & a).count('1'))
                lock = 4 * {0: 0, 1: opp, 2: 2 * opp}[self.style]
                have[S] = n; need[S] = bin(S).count('1') + 2 * self.ban_count + opp + lock
            return have, need

        excl = [sum(1 for d in des.values() if d == 1 << p) for p in range(5)]
        best = None
        for a in range(32):
            if a & ~named:
                continue
            have, need = table(a)
            mn = min([have[S] - need[S] for S in range(1, 32) if S & a == S] or [0])
            if a and mn < 0:
                continue
            key = (bin(a).count('1'), sum(excl[p] for p in range(5) if a & (1 << p)), mn)
            if best is None or key > best[0]:
                best = (key, a, have, need)
        _, a, have, need = best
        self._saf = ([bool(a & (1 << p)) for p in range(5)], have, need)
        return self._saf

    def pos_active(self, p):
        if self.gate == 'old':
            n = self.live_count(p)
            return n != 0 and n >= min_required(self.style, self.ban_count)
        return self._safety()[0][p]

    def mask_of(self, lower):
        designated = 0
        free = 0
        for p in range(5):
            if not self.pos_active(p):
                free |= 1 << p
            elif lower in self.allowed[p]:
                designated |= 1 << p
        if designated:
            return designated
        if free:
            return free
        return MASK_ALL

    def any_restricted(self):
        return any(self.pos_active(p) for p in range(5))


# ───────────────────────── 공통 컨텍스트 ─────────────────────────
@dataclass
class ModParams:
    enabled: bool = True
    ai_pick_gate: bool = True
    user_pick_block: bool = True
    swap_force: bool = True
    index_space: str = 'buggy'          # 'buggy'(현행: NAMES 테이블을 MODEL idx 로 조회) | 'correct'(이름 조인)
    cover_gate: bool = True             # 3차: cover_veto 를 MY_PICK_TURN 일 때만
    finalize_ver: str = 'v3'            # 'v2'(order 보존+점수 대체) | 'v3'(=v2) | 'v4'(팀별 자체 합법 판정) | 'v1'(구: cand 전체·allowed[0]) | 'off'
    cprod: bool = True                  # tag6 이름 교체 층
    frame_lag: int = 1                  # 픽 뒤 UI 게이트가 갱신되기까지 워커 결정 수(0=즉시 반영)
    score_pick_on: bool = True
    ui_fearless_aware: bool = False     # v0.6.1: UI 게이트 taken 에 피어리스 own lock(FEARLESS_SEEN) 포함


class ModState:
    """프로세스 전역(모드 static) 상태."""
    def __init__(self, names: List[str], model: List[str], pos: PosState, params: ModParams):
        self.names = list(names)                       # NAMES(로스터 순서)
        self.model = list(model)                       # MODEL 이름 목록(finalize champion_list)
        self.pos = pos
        self.p = params
        self.masks_names = [pos.mask_of(n) for n in names]
        self.mask_by_name = {n: m for n, m in zip(names, self.masks_names)}
        # 게시 상태
        self.blocklist: Optional[Set[str]] = None
        self.my_pick_turn = False
        self.ui_turn = 0
        self.scores_tls: List[tuple] = []              # note_score 링 (cand_idx, score)
        self.model_captured = False                    # v4: finalize 에서 champion_list 캡처 여부
        self.counters = {k: 0 for k in ['veto', 'cover', 'cover_skip', 'failopen', 'fz_seen', 'fz_filt', 'fz_scored', 'fz_noscore',
                                        'fz_v4_filt', 'cp_swap', 'cp_fail', 'wrongmask']}

    # 마스크 조회 — 인덱스 공간 정책
    def mask_of_model_idx(self, i):
        if self.p.index_space == 'buggy':
            if i < len(self.masks_names):
                m = self.masks_names[i]
                if self.model[i] != self.names[i]:
                    self.counters['wrongmask'] += 1
                return m
            return MASK_ALL
        # correct: 이름으로
        if self.p.finalize_ver == 'v4' and not self.model_captured:
            return None  # 캡처 전 = 판정 불가(fail-open)
        return self.mask_by_name.get(self.model[i], MASK_ALL)

    def mask_of_name(self, n):
        return self.mask_by_name.get(n, MASK_ALL)

    def name_of_model_idx(self, i):
        if self.p.index_space == 'buggy':
            return self.names[i] if i < len(self.names) else '?'
        return self.model[i]


# ───────────────────────── score_pick (lib.rs §4) ─────────────────────────
def score_pick(ms: ModState, ctx, cand_idx, base_score):
    """ctx: dict(available=[model idx], ally_pick=[model idx]) — 반환 None(Pass) | NEG(Replace)."""
    ms.scores_tls.append((cand_idx, base_score))
    if len(ms.scores_tls) > 1024:
        del ms.scores_tls[:512]
    p = ms.p
    if not (p.enabled and p.ai_pick_gate and ms.pos.any_restricted() and p.score_pick_on):
        return None
    cand = ms.mask_of_model_idx(cand_idx)
    if cand is None or cand == MASK_ALL:
        return None
    # BLOCKLIST cover_veto
    lname = ms.name_of_model_idx(cand_idx)
    blocked = ms.blocklist is not None and lname in ms.blocklist
    if blocked and p.cover_gate and not ms.my_pick_turn:
        ms.counters['cover_skip'] += 1
    elif blocked:
        ms.counters['cover'] += 1
        return NEG
    pinned = []
    for i in ctx['ally_pick']:
        m = ms.mask_of_model_idx(i)
        if m is not None and m != MASK_ALL:
            pinned.append(m)
    if not pinned:
        return None
    if feasible(pinned + [cand]):
        return None
    if not feasible(list(pinned)):
        return None  # ST_BROKEN
    # pool_has_feasible_idx
    has = False
    for c in ctx['available']:
        if c in ctx['ally_pick']:
            continue
        m = ms.mask_of_model_idx(c)
        if m is None or m == MASK_ALL:
            has = True
            break
        if feasible(pinned + [m]):
            has = True
            break
    if not has:
        ms.counters['failopen'] += 1
        return None
    ms.counters['veto'] += 1
    return NEG


# ───────────────────────── UI 게이트 (lib.rs §5, 프레임 단위) ─────────────────────────
def recompute_blocklist(ms: ModState, view):
    """view: dict(scene_ok, is_ban, my_turn_known: True/False/None, my_picks:[names], taken:set, picks_n, total)
    E1~E6 결정 트리. 반환 없이 ms.blocklist/my_pick_turn 갱신."""
    ms.my_pick_turn = False
    p = ms.p
    if not (p.enabled and p.user_pick_block and ms.pos.any_restricted()):
        ms.blocklist = None
        return 'E1'
    if not view['scene_ok']:
        return 'E2-hold'
    if view['is_ban']:
        ms.blocklist = None
        return 'E3'
    mt = view['my_turn_known']
    if mt is True:
        ms.my_pick_turn = True
    elif mt is False:
        ms.blocklist = None
        return 'E4'
    my_picks = [x for x in view['my_picks'] if x]
    if len(my_picks) >= view['picks_n'] // 2:
        ms.blocklist = None
        return 'E5'
    pinned = [ms.mask_of_name(x) for x in my_picks if ms.mask_of_name(x) != MASK_ALL]
    pinned_all = [ms.mask_of_name(x) for x in my_picks]
    taken = set(view['taken'])
    if p.ui_fearless_aware:
        taken |= set(view.get('own_locked', []))
    pool = [ms.masks_names[i] for i, n in enumerate(ms.names) if n not in taken]
    if free_left(pinned_all, pool, view['picks_n'] // 2) > 0:
        ms.blocklist = None
        return "E5'"
    block = set()
    any_feasible = False
    for i, n in enumerate(ms.names):
        if n in taken:
            continue
        m = ms.masks_names[i]
        if m == MASK_ALL:
            any_feasible = True
            continue
        if helps(pinned, m):
            any_feasible = True
        else:
            block.add(n)
    ms.blocklist = block if any_feasible else set()
    return 'E6'


# ───────────────────────── finalize (hooks.rs §6) ─────────────────────────
def finalize_hook(ms: ModState, champion_list, available_idx, pref_idx, taken: Set[str],
                  acting_team_picks: List[str], is_ban: bool):
    """0x201da90 pre-call. 반환 (available_idx', pref_idx') — None 이면 원본 그대로.
    v1: 구 코드(cand 전체 순회·allowed[0]) / v2,v3: order 보존 + 점수 대체 / v4: 팀별 자체 합법 판정(BLOCKLIST 불용)."""
    p = ms.p
    ms.counters['fz_seen'] += 1
    if p.finalize_ver == 'off':
        return None
    if not (p.enabled and p.ai_pick_gate and ms.pos.any_restricted()):
        return None
    if p.finalize_ver == 'v4' and not ms.model_captured:
        ms.model_captured = True   # 첫 호출에 champion_list 캡처(이후 mask_of_model_idx 유효)
    scores = {}
    for c, sc in reversed(ms.scores_tls):
        scores.setdefault(c, sc)

    if p.finalize_ver == 'v1':
        if not ms.my_pick_turn or not ms.blocklist:
            return None
        allowed = [i for i, n in enumerate(champion_list) if n not in taken and n not in ms.blocklist]
        if not allowed:
            return None
        pref = pref_idx if pref_idx in allowed else allowed[0]
        ms.counters['fz_filt'] += 1
        return allowed, pref

    if p.finalize_ver in ('v2', 'v3'):
        block = ms.blocklist if (ms.my_pick_turn and ms.blocklist) else set()
        src = list(available_idx) if available_idx else list(range(len(champion_list)))
        allowed = []
        cut = 0
        for i in src:
            n = champion_list[i]
            if n in taken:
                continue
            if n in block:
                cut += 1
                continue
            allowed.append(i)
        if not allowed:
            return None
        pref_ok = pref_idx in allowed
        if pref_ok and cut == 0:
            return None
        pref = pref_idx if pref_ok else _argmax(ms, allowed, scores, champion_list)
        ms.counters['fz_filt'] += 1
        return allowed, pref

    # v4: 팀별 자체 판정 — 밴은 불개입. pinned = 이 팀의 실제 픽(제한만), 자유 슬롯 예산은 UI 와 같은 규칙.
    if is_ban:
        return None
    src = list(available_idx) if available_idx else list(range(len(champion_list)))
    cand = [i for i in src if champion_list[i] not in taken]
    if not cand:
        return None
    pinned = [ms.mask_of_name(x) for x in acting_team_picks if ms.mask_of_name(x) != MASK_ALL]
    pinned_all = [ms.mask_of_name(x) for x in acting_team_picks]
    pool = [ms.mask_of_name(champion_list[i]) for i in cand]
    team_n = 5
    if free_left(pinned_all, pool, team_n) > 0 or not pinned:
        allowed = cand
    else:
        allowed = [i for i in cand if ms.mask_of_name(champion_list[i]) == MASK_ALL or helps(pinned, ms.mask_of_name(champion_list[i]))]
        if not allowed:
            allowed = cand   # fail-open(합법 후보 0)
    if pref_idx in allowed and len(allowed) == len(cand):
        return None
    pref = pref_idx if pref_idx in allowed else _argmax(ms, allowed, scores, champion_list)
    ms.counters['fz_v4_filt'] += 1
    return allowed, pref


def _argmax(ms: ModState, allowed, scores, champion_list):
    best = None
    for i in allowed:
        sc = scores.get(i)
        if sc is None:
            continue
        if best is None or sc > best[1]:
            best = (i, sc)
    if best is None:
        ms.counters['fz_noscore'] += 1
        return allowed[0]
    ms.counters['fz_scored'] += 1
    return best[0]


# ───────────────────────── cprod_swap (hooks.rs §7.1, tag 6) ─────────────────────────
def cprod_swap(ms: ModState, cur: str, my_picks: List[str], used: Set[str], is_human_click: bool, is_ban: bool,
               runnerup: List[str]):
    """확정 픽 이름 교체. 반환 새 이름(또는 cur)."""
    p = ms.p
    if not (p.cprod and p.enabled and p.ai_pick_gate and ms.pos.any_restricted()):
        return cur
    if is_human_click or is_ban:
        return cur
    pinned = [ms.mask_of_name(x) for x in my_picks if ms.mask_of_name(x) != MASK_ALL]
    cm = ms.mask_of_name(cur)
    if cm == MASK_ALL:
        return cur
    if helps(pinned, cm):
        return cur
    pinned_all = [ms.mask_of_name(x) for x in my_picks]
    pool = [ms.masks_names[i] for i, n in enumerate(ms.names) if n not in used]
    if free_left(pinned_all, pool, 5) > 0:
        return cur
    for c in runnerup:
        if c in used:
            continue
        if helps(pinned, ms.mask_of_name(c)):
            ms.counters['cp_swap'] += 1
            return c
    for n in ms.names:
        if n in used:
            continue
        m = ms.mask_of_name(n)
        if m == MASK_ALL:
            continue
        if helps(pinned, m):
            ms.counters['cp_swap'] += 1
            return n
    ms.counters['cp_fail'] += 1
    return cur
