//! 드래프트 상태 — 내팀/상대팀 밴·픽 슬롯 + 진행 + undo.
//! TFM2 = 팀당 밴 2 + 픽 5. 정확한 인터리브 순서는 게임마다/모드마다 다를 수 있어
//! 사용자가 직접 차례를 고르는 자유형 + 휴리스틱 자동진행으로 둔다.
use std::collections::HashSet;

pub const BANS_PER_TEAM: usize = 2;
pub const PICKS_PER_TEAM: usize = 5;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {
    My,
    Enemy,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Ban,
    Pick,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Step {
    pub side: Side,
    pub kind: Kind,
}

#[derive(Default)]
pub struct Draft {
    pub my_bans: Vec<String>,
    pub my_picks: Vec<String>,
    pub en_bans: Vec<String>,
    pub en_picks: Vec<String>,
    pub history: Vec<(Side, Kind, String)>,
}

impl Draft {
    pub fn used(&self) -> HashSet<String> {
        let mut s = HashSet::new();
        for v in [&self.my_bans, &self.my_picks, &self.en_bans, &self.en_picks] {
            for c in v {
                s.insert(c.clone());
            }
        }
        s
    }

    fn slot(&mut self, step: Step) -> &mut Vec<String> {
        match (step.side, step.kind) {
            (Side::My, Kind::Ban) => &mut self.my_bans,
            (Side::My, Kind::Pick) => &mut self.my_picks,
            (Side::Enemy, Kind::Ban) => &mut self.en_bans,
            (Side::Enemy, Kind::Pick) => &mut self.en_picks,
        }
    }

    fn cap(kind: Kind) -> usize {
        match kind {
            Kind::Ban => BANS_PER_TEAM,
            Kind::Pick => PICKS_PER_TEAM,
        }
    }

    /// step 슬롯이 꽉 찼는지.
    pub fn full(&self, step: Step) -> bool {
        let len = match (step.side, step.kind) {
            (Side::My, Kind::Ban) => self.my_bans.len(),
            (Side::My, Kind::Pick) => self.my_picks.len(),
            (Side::Enemy, Kind::Ban) => self.en_bans.len(),
            (Side::Enemy, Kind::Pick) => self.en_picks.len(),
        };
        len >= Self::cap(step.kind)
    }

    /// champ 을 step 슬롯에 배정 (꽉 찼으면 무시). 성공 시 true.
    pub fn assign(&mut self, step: Step, champ: &str) -> bool {
        if self.full(step) || self.used().contains(champ) {
            return false;
        }
        self.slot(step).push(champ.to_string());
        self.history.push((step.side, step.kind, champ.to_string()));
        true
    }

    pub fn undo(&mut self) {
        if let Some((side, kind, _)) = self.history.pop() {
            let step = Step { side, kind };
            self.slot(step).pop();
        }
    }

    pub fn reset(&mut self) {
        *self = Draft::default();
    }

    /// 다음 차례 휴리스틱: 밴 안 끝났으면 밴 번갈아, 끝났으면 픽 스네이크-ish.
    /// 단순 토글(사용자가 버튼으로 override 가능).
    pub fn suggest_next(&self, cur: Step) -> Step {
        // 밴 단계: 양 팀 밴이 다 안 찼으면 밴 유지하며 반대편으로
        let bans_done = self.my_bans.len() >= BANS_PER_TEAM && self.en_bans.len() >= BANS_PER_TEAM;
        if !bans_done {
            let other = match cur.side {
                Side::My => Side::Enemy,
                Side::Enemy => Side::My,
            };
            let cand = Step { side: other, kind: Kind::Ban };
            if !self.full(cand) {
                return cand;
            }
            let same = Step { side: cur.side, kind: Kind::Ban };
            if !self.full(same) {
                return same;
            }
            // 밴 다 찼으면 픽으로
            return Step { side: Side::My, kind: Kind::Pick };
        }
        // 픽 단계: 반대편으로 토글, 꽉 찼으면 같은편
        let other = match cur.side {
            Side::My => Side::Enemy,
            Side::Enemy => Side::My,
        };
        for s in [other, cur.side] {
            let cand = Step { side: s, kind: Kind::Pick };
            if !self.full(cand) {
                return cand;
            }
        }
        cur // 드래프트 종료
    }

    pub fn complete(&self) -> bool {
        self.my_bans.len() >= BANS_PER_TEAM
            && self.en_bans.len() >= BANS_PER_TEAM
            && self.my_picks.len() >= PICKS_PER_TEAM
            && self.en_picks.len() >= PICKS_PER_TEAM
    }
}
