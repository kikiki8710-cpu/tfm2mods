//! tactics — 상세전술 32개 표 · 단계 선택 · 단계 값 · 내 팀 판별. (2026-10-22, 0.6.5)
//!
//! 다른 모듈은 여기만 본다:
//!   · UI/저장  → `set_level` / `level` / `state_text` / `apply_state_text`
//!   · 설정 파일 → `set_value`
//!   · 적용 장치 → `value_for(tactic, athlete_id, pos)` — None 이면 원본 그대로.
//! 모든 상태는 원자 변수라 어느 스레드(시뮬 스레드 포함)에서 읽어도 된다. 락·할당 없음.
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU32, AtomicU64, AtomicU8, Ordering::Relaxed};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Scope {
    /// 포지션(탑·정글·미드·원딜·서폿)마다 따로 고른다.
    Player,
    /// 팀 전체에 하나.
    Team,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Kind {
    /// 숫자 문턱 — 단계: 기본 / 낮게 / 높게.
    Value,
    /// 켬·끔 — 단계: 기본(켬) / 끔.
    Toggle,
}

pub struct TacticDef {
    /// 설정 파일 [섹션] 이름 · 저장 키.
    pub id: &'static str,
    /// tactics_verify 노브 id(지점 표 `knobs32_065.json` 의 id).
    pub knob: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
    pub scope: Scope,
    pub kind: Kind,
    /// 게임 원본값(표시용).
    pub default: i64,
    /// 설정 파일이 없을 때 쓸 낮게/높게 기본값.
    pub low: i64,
    pub high: i64,
    /// 허용 범위(즉치 폭 한계 — tactics_verify KNOBS min/max).
    pub min: i64,
    pub max: i64,
}

const fn v(id: &'static str, knob: &'static str, name: &'static str, desc: &'static str, scope: Scope,
           default: i64, low: i64, high: i64, min: i64, max: i64) -> TacticDef {
    TacticDef { id, knob, name, desc, scope, kind: Kind::Value, default, low, high, min, max }
}
const fn t(id: &'static str, knob: &'static str, name: &'static str, desc: &'static str, scope: Scope) -> TacticDef {
    TacticDef { id, knob, name, desc, scope, kind: Kind::Toggle, default: 1, low: 0, high: 1, min: 0, max: 1 }
}

use Scope::{Player as P, Team as T};

/// 표 순서 = 전술 번호(저장·적용 장치가 이 번호를 쓴다 — 바꾸지 말고 뒤에 추가할 것).
pub static TACTICS: &[TacticDef] = &[
    // ── 후퇴·교전 회피
    v("combat_retreat_hp", "combat_retreat_hp_35", "전투 중 후퇴 체력(%)", "교전 중 체력이 이 값보다 낮으면 빠진다. 높을수록 일찍 빠진다", P, 35, 20, 50, 0, 127),
    v("line_chase_retreat_hp", "line_chase_retreat_hp_45", "라인·추적 중 즉시 후퇴 체력(%)", "라인전·추적 중 체력이 이 값 이하면 바로 후퇴", P, 45, 30, 60, 0, 127),
    v("line_chase_target_guard_hp", "line_chase_target_guard_hp_65", "추적 표적 유지 체력(%)", "체력이 이 값 이하면 쫓던 표적을 놓는다", P, 65, 50, 80, 0, 127),
    v("lane_retreat_final", "lane_retreat_final_50", "라인 후퇴 점수 문턱", "라인 후퇴 최종 점수가 이 값을 넘으면 후퇴. 낮을수록 잘 빠진다", P, 50, 35, 65, 0, 127),
    v("crowd_retreat_weight", "crowd_retreat_weight_plus2_300", "수적 열세 후퇴 가중치", "적이 2명 이상 많을 때 후퇴 쪽에 주는 무게", P, 300, 150, 600, 0, 100000),
    v("combat_avoid_hp", "combat_avoid_hp_41", "교전 회피 체력(%)", "체력이 이 값보다 낮으면 싸움을 피한다", P, 41, 25, 60, 0, 127),
    v("combat_avoid_count_margin", "combat_avoid_count_margin_2", "교전 회피 인원 여유(명)", "적이 아군보다 이만큼 많으면 피한다. 작을수록 조심", P, 2, 1, 3, 0, 127),
    v("self_risk_ignore_hp", "self_risk_ignore_hp_35", "위험 무시 체력(%)", "체력이 이 값 이상이면 주변 위험을 무시하고 행동", P, 35, 20, 55, 0, 127),
    // ── 갱킹·정글
    v("jg_gank_hp_start", "jg_gank_hp_start_50", "갱 시작 체력(%)", "정글러가 이 체력 이상일 때 갱을 시작", P, 50, 35, 70, 0, 127),
    v("jg_gank_hp_floor", "jg_gank_hp_floor_40", "갱 최소 체력(%)", "갱 중 이 체력 아래면 그만둔다", P, 40, 25, 60, 0, 127),
    v("gank_abort_hp", "gank_abort_hp_41", "갱 포기 체력(%)", "갱 진행 중 체력이 이 값보다 낮으면 포기", P, 41, 25, 60, 0, 127),
    t("gank_abort_tower", "gank_abort_tower_reason9", "갱 중 적 포탑 교전 시 포기", "끄면 적 포탑 근처에서도 갱을 계속한다", P),
    v("gank_call_accept_hp", "gank_call_accept_hp_60", "갱 콜 수락 최소 체력(%)", "라이너가 갱 콜을 받을 때 필요한 체력", P, 60, 40, 80, 1, 127),
    v("jungle_safe_radius", "jungle_safe_radius_150000", "정글 캠프 안전 반경", "노리는 캠프 이 거리 안에 적이 있으면 위험으로 본다", P, 150000, 100000, 250000, 1000, 2000000),
    v("jungle_stop_hp", "jungle_stop_hp_41", "정글링·로밍 중단 체력(%)", "체력이 이 값 아래면 정글링(로밍 중 포함)을 멈춘다", P, 41, 31, 55, 30, 127),
    v("minion_damage_ratio", "minion_damage_ratio_49", "미니언 피해 민감도(%)", "미니언 예상 피해율이 이 값을 넘으면 위험으로 본다. 낮을수록 조심", P, 49, 39, 65, 39, 127),
    // ── 라인 운영·로밍
    v("lane_assign_hp", "lane_assign_hp_30", "라인 배정 체력(%)", "이 체력 미만인 팀원은 라인 배정 후보에서 뺀다", P, 30, 15, 50, 0, 127),
    v("lane_opportunity_own_min", "lane_opportunity_own_min_2", "라인 기회 — 건강 아군 최소(명)", "라인 영역에 건강한 아군이 이만큼 있어야 기회로 본다", P, 2, 1, 3, 0, 127),
    t("support_minion_block", "support_minion_block_role_4", "서폿 라인 미니언 처리 금지", "끄면 서폿도 라인 미니언을 처리한다", P),
    t("bot_duo_partner", "bot_duo_partner_required", "원딜·서폿 짝 동행 확인", "끄면 원딜·서폿이 짝 없이도 혼자 움직인다", P),
    v("liner_roam_self_hp", "liner_roam_self_hp_41", "로밍 최소 자기 체력(%)", "라이너가 이 체력 이상일 때만 로밍", P, 41, 25, 60, 0, 127),
    v("liner_roam_lane_healthy_hp", "liner_roam_lane_healthy_hp_41", "로밍 대상 라인 — 건강 아군 기준(%)", "갈 라인을 고를 때 이 체력 이상인 아군을 건강하다고 센다", P, 41, 25, 60, 0, 127),
    v("liner_roam_call_chance", "liner_roam_call_chance_700", "로밍 팀 콜 확률(/1000)", "로밍할 때 팀 콜을 보낼 확률", P, 700, 400, 900, 0, 1001),
    v("hazard_avoid_padding", "hazard_avoid_padding_6000", "위험 구역 비켜 걷기 여유", "위험 구역을 이만큼 넓게 피해 걷는다", P, 6000, 3000, 12000, 0, 1000000),
    // ── 푸시·타워
    v("push_sustain_min_healthy", "push_sustain_min_healthy_3", "푸시 유지 최소 인원(명)", "진행 중인 푸시를 이어가려면 건강한 아군이 이만큼 필요", T, 3, 2, 4, 0, 127),
    v("push_sustain_margin", "push_sustain_margin_2", "푸시 유지 인원 여유(명)", "아군이 적보다 이만큼 많아야 푸시를 이어간다", T, 2, 1, 3, 0, 126),
    v("tower_chase_own_hp", "tower_chase_own_hp_35", "타워 아래 추격 내 체력(%)", "적 타워 아래까지 쫓으려면 필요한 내 체력(25/35/45 단계를 함께 이동)", P, 35, 25, 50, 10, 117),
    // ── 기지 방어
    v("base_defense_min_enemies", "base_defense_min_enemies_6", "기지 방어 출동 기준 적 수", "기지 근처 적(미니언 포함 추정)이 이만큼 이상이면 방어하러 간다", T, 6, 4, 8, 1, 127),
    v("base_defense_hold_retreat_hp", "base_defense_hold_retreat_hp_45", "기지 방어 중 후퇴 체력(%)", "기지 방어 중 체력이 이 값 아래면 물러난다", P, 45, 30, 60, 0, 127),
    v("base_defense_hold_nexus_hp", "base_defense_hold_nexus_hp_50", "넥서스 위험 체력(%)", "넥서스 체력이 이 값 이하면 치명 피해만 아니면 버틴다", T, 50, 35, 70, 0, 127),
    v("base_defense_hold_dmg_ratio", "base_defense_hold_dmg_ratio_49", "기지 방어 피해율 문턱(%)", "받는 피해율이 이 값을 넘으면 체력과 무관하게 위험으로 본다", P, 49, 30, 70, 0, 127),
    // ── 궁극기
    v("lowhp_ult_bonus_low", "lowhp_ult_bonus_low_14", "저체력 궁극기 가산점", "체력 45% 미만일 때 궁극기 사용 점수에 더하는 값", P, 14, 5, 40, 0, 127),
];

pub const N: usize = 32;
const _: () = assert!(TACTICS.len() == N);

/// 단계: 0 기본(원본) · 1 낮게(켬·끔이면 끔) · 2 높게.
pub const LV_BASE: u8 = 0;
pub const LV_LOW: u8 = 1;
pub const LV_HIGH: u8 = 2;
/// 칸: 0..4 = 포지션(탑·정글·미드·원딜·서폿), 5 = 팀.
pub const COL_TEAM: usize = 5;
pub const POS_NAMES: [&str; 5] = ["탑", "정글", "미드", "원딜", "서폿"];

static LEVEL: [[AtomicU8; 6]; N] = [const { [const { AtomicU8::new(0) }; 6] }; N];
/// [전술][0 = 낮게, 1 = 높게]. i64::MIN = 아직 안 읽음(정의 기본값 사용).
static VALUE: [[AtomicI64; 2]; N] = [const { [const { AtomicI64::new(i64::MIN) }; 2] }; N];
static STATE_VER: AtomicU32 = AtomicU32::new(0);
/// 단계 값(설정 파일)이 바뀔 때마다 증가 — 적용 장치가 값 반영 시점을 안다.
static VALUES_VER: AtomicU32 = AtomicU32::new(0);

/// 전술별 적용 상태(적용 장치가 쓴다 · UI 가 표시).
pub const ST_UNKNOWN: u8 = 0;
pub const ST_ACTIVE: u8 = 1;
pub const ST_UNSUPPORTED: u8 = 2;
pub const ST_FAILED: u8 = 3;
static STATUS: [AtomicU8; N] = [const { AtomicU8::new(ST_UNKNOWN) }; N];
pub fn set_status(t: usize, st: u8) { if t < N { STATUS[t].store(st, Relaxed); } }
pub fn status(t: usize) -> u8 { if t < N { STATUS[t].load(Relaxed) } else { ST_UNKNOWN } }
pub fn index_of_knob(knob: &str) -> Option<usize> { TACTICS.iter().position(|d| d.knob == knob) }

/// 계약 명단 athlete_id(최대 32). ID 0도 유효하므로 빈 슬롯은 u64::MAX.
static MY_ATHLETES: [AtomicU64; 32] = [const { AtomicU64::new(u64::MAX) }; 32];
/// 적용 스위치 — 세이브 설정 로드 + 내 팀 확보 뒤 켠다.
static ARMED: AtomicBool = AtomicBool::new(false);

pub fn index_of(id: &str) -> Option<usize> { TACTICS.iter().position(|d| d.id == id) }

pub fn level(t: usize, col: usize) -> u8 { if t < N && col < 6 { LEVEL[t][col].load(Relaxed) } else { 0 } }
pub fn set_level(t: usize, col: usize, lv: u8) {
    if t >= N || col >= 6 { return; }
    let max = if TACTICS[t].kind == Kind::Toggle { LV_LOW } else { LV_HIGH };
    LEVEL[t][col].store(lv.min(max), Relaxed);
    STATE_VER.fetch_add(1, Relaxed);
}
pub fn state_version() -> u32 { STATE_VER.load(Relaxed) }
pub fn clear_levels() { for row in LEVEL.iter() { for c in row { c.store(0, Relaxed); } } STATE_VER.fetch_add(1, Relaxed); }

/// 단계 값(낮게/높게). 범위 밖이면 거절(false).
pub fn set_value(t: usize, lv: u8, value: i64) -> bool {
    if t >= N || !(lv == LV_LOW || lv == LV_HIGH) { return false; }
    let d = &TACTICS[t];
    if value < d.min || value > d.max { return false; }
    VALUE[t][(lv - 1) as usize].store(value, Relaxed);
    VALUES_VER.fetch_add(1, Relaxed);
    true
}
pub fn values_version() -> u32 { VALUES_VER.load(Relaxed) }
/// 설정 파일 다시 읽기 전 — 전부 정의 기본값으로.
pub fn reset_values() {
    for row in VALUE.iter() { for c in row { c.store(i64::MIN, Relaxed); } }
    VALUES_VER.fetch_add(1, Relaxed);
}
/// 단계의 실제 값(기본 = 원본값).
pub fn value_of(t: usize, lv: u8) -> i64 {
    let d = &TACTICS[t];
    match lv {
        LV_LOW | LV_HIGH => {
            if d.kind == Kind::Toggle { return if lv == LV_LOW { 0 } else { 1 }; }
            let v = VALUE[t][(lv - 1) as usize].load(Relaxed);
            if v == i64::MIN { if lv == LV_LOW { d.low } else { d.high } } else { v }
        }
        _ => d.default,
    }
}

pub fn set_my_athletes(ids: &[u64]) {
    for (i, slot) in MY_ATHLETES.iter().enumerate() { slot.store(ids.get(i).copied().unwrap_or(u64::MAX), Relaxed); }
}
pub fn my_athletes() -> Vec<u64> { MY_ATHLETES.iter().map(|a| a.load(Relaxed)).filter(|a| *a != u64::MAX).collect() }
#[inline]
pub fn is_my_athlete(ath: u64) -> bool { ath != u64::MAX && MY_ATHLETES.iter().any(|a| a.load(Relaxed) == ath) }
pub fn set_armed(on: bool) { ARMED.store(on, Relaxed); }
pub fn armed() -> bool { ARMED.load(Relaxed) }

/// ★적용 장치용 단일 진입점. 판단 중인 선수(athlete_id, 포지션 0..4)에게 쓸 값.
/// None = 원본 그대로(내 팀이 아님 · 단계 기본 · 비무장 · 범위 밖 입력).
#[inline]
pub fn value_for(t: usize, athlete_id: u64, pos: usize) -> Option<i64> {
    let lv = level_for(t, athlete_id, pos);
    if lv == LV_BASE { None } else { Some(value_of(t, lv)) }
}
/// 같은 판정의 단계 번호판(0 = 원본 그대로). 단계별로 미리 만든 코드를 고르는 쪽이 쓴다.
#[inline]
pub fn level_for(t: usize, athlete_id: u64, pos: usize) -> u8 {
    if t >= N || !armed() || !is_my_athlete(athlete_id) || pos > 4 { return LV_BASE; }
    let col = if TACTICS[t].scope == Scope::Team { COL_TEAM } else { pos };
    LEVEL[t][col].load(Relaxed)
}

// ───────── 세이브 저장 형식: 줄마다 "<전술 id> <칸6개>" (칸 = 0/1/2, 순서 탑 정글 미드 원딜 서폿 팀) ─────────
pub fn state_text() -> String {
    let mut s = String::from("v1\n");
    for (t, d) in TACTICS.iter().enumerate() {
        let cols: Vec<String> = (0..6).map(|c| level(t, c).to_string()).collect();
        if cols.iter().all(|c| c == "0") { continue; }
        s.push_str(&format!("{} {}\n", d.id, cols.join(" ")));
    }
    s
}
/// 반환 = 읽은 전술 수. 모르는 id·잘못된 줄은 건너뛴다(다음 버전 호환).
pub fn apply_state_text(txt: &str) -> usize {
    clear_levels();
    let mut n = 0;
    for line in txt.lines() {
        let mut it = line.split_whitespace();
        let Some(id) = it.next() else { continue };
        let Some(t) = index_of(id) else { continue };
        let Ok(cols): Result<Vec<u8>,_> = it.map(|x| x.parse::<u8>()).collect() else { continue; };
        if cols.len() != 6 { continue; }
        for (c, lv) in cols.iter().enumerate() { set_level(t, c, *lv); }
        n += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn table_is_consistent() {
        for d in TACTICS {
            assert!(d.min <= d.default && d.default <= d.max, "{}", d.id);
            assert!(d.min <= d.low && d.low <= d.max && d.min <= d.high && d.high <= d.max, "{}", d.id);
        }
        let mut ids: Vec<&str> = TACTICS.iter().map(|d| d.id).collect();
        ids.sort(); ids.dedup(); assert_eq!(ids.len(), N);
    }
    #[test]
    fn malformed_saved_line_is_rejected_as_a_whole() {
        assert_eq!(apply_state_text("combat_retreat_hp 1 0 0 0 0 0 extra\n"),0);
        assert_eq!(apply_state_text("combat_retreat_hp 1 0 0 0 0 0\n"),1);
        assert_eq!(level(0,0),1);
        clear_levels();
    }
    #[test]
    fn value_for_respects_team_level_and_armed() {
        clear_levels();
        set_my_athletes(&[101, 102]);
        let t = index_of("combat_retreat_hp").unwrap();
        set_level(t, 0, LV_HIGH);
        set_armed(false);
        assert_eq!(value_for(t, 101, 0), None);
        set_armed(true);
        assert_eq!(value_for(t, 101, 0), Some(50));
        assert_eq!(value_for(t, 101, 1), None); // 정글 칸은 기본
        assert_eq!(value_for(t, 999, 0), None); // 남의 팀
        set_my_athletes(&[0,101,102]); // 실제 세이브의 첫 선수 ID도 0이다.
        assert_eq!(value_for(t,0,0),Some(50));
        assert_eq!(my_athletes(),vec![0,101,102]);
        assert!(set_value(t, LV_HIGH, 60));
        assert_eq!(value_for(t, 102, 0), Some(60));
        assert!(!set_value(t, LV_HIGH, 500)); // 범위 밖
        let team = index_of("push_sustain_min_healthy").unwrap();
        set_level(team, COL_TEAM, LV_LOW);
        assert_eq!(value_for(team, 101, 3), Some(2));
        let tog = index_of("bot_duo_partner").unwrap();
        set_level(tog, 4, LV_HIGH); // 켬·끔은 낮게(끔)까지만
        assert_eq!(level(tog, 4), LV_LOW);
        assert_eq!(value_for(tog, 101, 4), Some(0));
        let txt = state_text();
        set_level(t, 0, LV_BASE);
        assert_eq!(apply_state_text(&txt), 3);
        assert_eq!(level(t, 0), LV_HIGH);
        set_armed(false);
        clear_levels();
    }
}
