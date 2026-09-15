"""draft.py — TFM2 밴픽 드래프트 상태기계(정적 재현). 근거 = REPORT\\tfm2_champ_pos_lock\\RE\\2026-09-15_밴픽-프로토콜-상태기계-IR독해.md
 · 턴 = MatchSetInfo::banpick_phase (match_info.rs:449)  · 커밋 = RunningMatchInfo::select_champion (match_info.rs:903)
 · 피어리스 lock = fearless_locked (match_info.rs:845)     · 타임아웃 = server.rs run L4220 (합법 후보 균등 랜덤)
게임 로직만 담는다(모드 층은 modlayers.py). 이름은 소문자 챔피언 id.
"""
from dataclasses import dataclass, field
from typing import List, Optional, Tuple

PICK_TBL = [[0, 1, 0, 1], [0, 1, 1, 0, 0, 1], [0, 1, 1, 0, 1, 0, 0, 1], [0, 1, 1, 0, 0, 1, 1, 0, 0, 1]]
PICKS_PER_RULE = [4, 6, 8, 10]
CLASSIC, FEARLESS, HARD = 0, 1, 2


@dataclass
class SetInfo:
    ban_count: int
    rule: int = 3                     # 0=2v2 1=3v3 2=4v4 3=5v5
    side: bool = True                 # team1 이 블루
    blue_ban: List[str] = field(default_factory=list)
    red_ban: List[str] = field(default_factory=list)
    blue_pick: List[str] = field(default_factory=list)
    red_pick: List[str] = field(default_factory=list)

    def total(self):
        return len(self.blue_ban) + len(self.red_ban) + len(self.blue_pick) + len(self.red_pick)

    def turn(self) -> Optional[Tuple[bool, int]]:
        """None=종료 / (is_ban, side) side 0=블루."""
        total = self.total()
        base = 2 * self.ban_count
        if total >= base + PICKS_PER_RULE[self.rule]:
            return None
        if total < base:
            return (True, total & 1)
        return (False, PICK_TBL[self.rule][total - base])

    def buckets(self):
        return self.blue_ban, self.red_ban, self.blue_pick, self.red_pick

    def taken(self):
        return set(self.blue_ban) | set(self.red_ban) | set(self.blue_pick) | set(self.red_pick)


@dataclass
class Match:
    team1: int
    team2: int
    style: int = CLASSIC
    rule: int = 3
    sets: List[SetInfo] = field(default_factory=list)

    # ── 진영/팀 ──
    def team1_in_set(self):
        s = self.sets[-1]
        return self.team1 if s.side else self.team2

    def team2_in_set(self):
        s = self.sets[-1]
        return self.team2 if s.side else self.team1

    def next_banpick_team(self):
        t = self.sets[-1].turn()
        if t is None:
            return None
        return self.team1_in_set() if t[1] == 0 else self.team2_in_set()

    # ── 피어리스 ──
    def fearless_locked(self):
        """(match team1 locked, match team2 locked) — 이전 세트들의 픽만(밴 제외)."""
        if self.style == CLASSIC:
            return [], []
        t1, t2 = [], []
        for s in self.sets[:-1]:
            if self.style == HARD:
                allp = list(s.blue_pick) + list(s.red_pick)
                t1 += allp
                t2 += list(allp)
            else:
                if s.side:
                    t1 += s.blue_pick
                    t2 += s.red_pick
                else:
                    t1 += s.red_pick
                    t2 += s.blue_pick
        return t1, t2

    def fearless_locked_in_set(self):
        t1, t2 = self.fearless_locked()
        return (t1, t2) if self.sets[-1].side else (t2, t1)

    def locked_for(self, team_id):
        """(own_locked, enemy_locked) — team 기준."""
        b, r = self.fearless_locked_in_set()
        return (b, r) if team_id == self.team1_in_set() else (r, b)

    # ── 커밋 (select_champion) ──
    def select_champion(self, team_id, champ) -> bool:
        s = self.sets[-1]
        blue_locked, red_locked = self.fearless_locked_in_set()
        if champ in s.taken():
            return False
        t = s.turn()
        if t is None:
            return False
        is_ban, side = t
        limit = s.ban_count if is_ban else s.rule + 2
        if side == 0:
            if team_id != self.team1_in_set():
                return False
            bucket = s.blue_ban if is_ban else s.blue_pick
            lock = red_locked if is_ban else blue_locked
        else:
            if team_id != self.team2_in_set():
                return False
            bucket = s.red_ban if is_ban else s.red_pick
            lock = blue_locked if is_ban else red_locked
        if len(bucket) >= limit or champ in lock:
            return False
        bucket.append(champ)
        return True

    # ── 타임아웃 후보 (server.rs L4248~) ──
    def timeout_candidates(self, team_id, selectable):
        s = self.sets[-1]
        own, enemy = self.locked_for(team_id)
        t = s.turn()
        if t is None:
            return []
        lock = enemy if t[0] else own
        tk = s.taken()
        return [c for c in selectable if c not in tk and c not in lock]

    # ── 세트 전환 (§7.2) ──
    def new_set(self, loser_wants_blue: Optional[bool], loser_is_team1: Optional[bool]):
        last = self.sets[-1]
        if loser_is_team1 is None or loser_wants_blue is None:
            side = True
        else:
            # 패배팀이 블루를 원하면 그 팀이 블루: team1 이 블루 ⇔ (loser==team1) == wants_blue
            side = (loser_is_team1 == loser_wants_blue)
        self.sets.append(SetInfo(ban_count=last.ban_count, rule=last.rule, side=side))


def is_legal_banpick_candidate(cand, s: SetInfo, own_locked, enemy_locked, is_ban) -> bool:
    """worker.rs:866 — 4버킷 밖 + (픽: own_locked 밖 / 밴: enemy_locked 밖)."""
    if cand in s.taken():
        return False
    return cand not in (enemy_locked if is_ban else own_locked)


def resolve_banpick_candidate(champion_list, available_idx, select, s: SetInfo, own_locked, enemy_locked, is_ban):
    """worker.rs:881 — 게임의 최종 이름 확정(3갈래 폴백). champion_list = 모델 이름 목록(전체), available_idx = 활성 인덱스.
    return (name|None, path)  path ∈ {'pref','avail0','scan','none'}"""
    if select in available_idx:
        idx = select
        path = 'pref'
    elif available_idx:
        idx = available_idx[0]
        path = 'avail0'
    else:
        idx = None
        path = 'none'
    if idx is not None and idx < len(champion_list):
        c = champion_list[idx]
        if is_legal_banpick_candidate(c, s, own_locked, enemy_locked, is_ban):
            return c, path
    for c in champion_list:
        if is_legal_banpick_candidate(c, s, own_locked, enemy_locked, is_ban):
            return c, 'scan'
    return None, 'none'
