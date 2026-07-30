//! draft_ai.rs — 우리만의 밴픽 AI 보정(밴할 때 "이미 확정된 픽"을 함께 본다).
//! ===========================================================================
//! ## 왜 필요한가
//! 게임 네이티브 밴 AI는 **픽을 전혀 안 본다**(RE 확정, discovered-banpick-ai §17):
//!   · recommend_ban(0xefef00)에 픽 목록이 **인자로 전달조차 안 됨**(밴 2개만 받음)
//!   · 밴 가치함수 0x1c07880의 "내 조합/상대 조합" 배열 = **하드코딩 빈 Vec**
//!   · 모드용 확장 API(ModDraftScoreHook)의 ctx도 **밴 호출에선 픽 슬롯이 빈 슬라이스**
//! 바닐라는 밴이 항상 먼저라 픽이 없으니 자연스러운 설계였지만, 이 모드의 인터리브
//! 순서(밴↔픽 혼합)에선 "상대가 이미 뽑은 챔프"를 보고 밴해야 의미가 있다.
//!
//! ## 어떻게 픽을 얻나
//! ctx가 안 주므로 **모드가 직접 스냅샷**한다. hooks.rs의 phase 훅(0x1cd9380)은 AI턴
//! 직전에 **MatchSetInfo 레코드**(양 팀 밴/픽 4벡터 = 챔프 id 문자열)를 통째로 받으므로,
//! 거기서 4벡터를 떠 두고 score_ban에서 사용한다. ctx의 밴 개수와 스냅샷의 밴 개수가
//! 일치할 때만 적용(다른 매치의 호출이면 Pass) — 오적용 방지.
//!
//! ## 점수 기준 (v1, cfg로 가중치 조절)
//! 밴 후보 c에 대해 두 축을 본다:
//!   ① **상대 시너지**: c가 상대 조합의 빈 곳을 메우는가(딜 타입 균형·앞라인 부재).
//!      → 상대가 다음에 집으면 강해지는 카드 = 미리 자른다.
//!   ② **내 조합 위협**: c가 내 조합을 잘 때리는가(내 저항 낮은 쪽 딜 타입·물몸 저격).
//!      → 내가 앞으로 쌓을 조합의 카운터를 선제 차단.
//! 최종 = 네이티브 점수 + clamp(w1*① + w2*②, ±cap)  (DraftScoreDecision::Add)
//!
//! 챔프 능력치는 클라이언트 db의 ChampionInfoSheet에서 1회 캡처한다(경기 중 불변).
//! ===========================================================================
#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;

/// 챔프 한 마리의 판단용 능력치(Lv1 기준 + 성장 반영 대략값).
#[derive(Clone, Copy, Default, Debug)]
pub struct Attr {
    pub ad: f32,
    pub ap: f32,
    pub hp: f32,
    pub def: f32,
    pub mr: f32,
    pub range: f32,
}

impl Attr {
    /// 물리/마법 성향 (-1 = 순수 마법, +1 = 순수 물리, 0 = 혼합)
    fn dmg_axis(&self) -> f32 {
        let t = self.ad + self.ap;
        if t <= 0.0 {
            return 0.0;
        }
        (self.ad - self.ap) / t
    }
    /// 앞라인 성향(체력·방어 기반, 0~1 대략)
    fn front(&self) -> f32 {
        ((self.hp / 700.0) * 0.6 + (self.def / 40.0) * 0.4).clamp(0.0, 2.0) / 2.0
    }
    /// 화력 총량(정규화)
    fn power(&self) -> f32 {
        ((self.ad + self.ap) / 120.0).clamp(0.0, 2.0) / 2.0
    }
}

/// 챔프 테이블: 인덱스(ctx candidate id) → (id 문자열, 능력치).
static TABLE: Mutex<Vec<(String, Attr)>> = Mutex::new(Vec::new());
static TABLE_READY: AtomicBool = AtomicBool::new(false);

/// 현재 드래프트 스냅샷 — phase 훅이 매 턴 갱신.
#[derive(Clone, Default)]
pub struct Snapshot {
    /// 팀1/팀2 밴·픽 (챔프 id 문자열)
    pub t1_ban: Vec<String>,
    pub t2_ban: Vec<String>,
    pub t1_pick: Vec<String>,
    pub t2_pick: Vec<String>,
    /// 이번 턴에 행동하는 팀이 팀1인가(= seq 팀비트 0)
    pub acting_t1: bool,
    pub valid: bool,
}
static SNAP: Mutex<Option<Snapshot>> = Mutex::new(None);

pub static CNT_BAN_ADJ: AtomicU64 = AtomicU64::new(0);
pub static CNT_BAN_PASS: AtomicU64 = AtomicU64::new(0);

/// 클라이언트에서 챔프 능력치 테이블 1회 캡처(경기 중 불변).
/// sheet 순서 = ctx candidate 인덱스와 동일하다는 전제 — debug 로그로 검증 가능.
pub fn capture_table<F>(fill: F)
where
    F: FnOnce(&mut Vec<(String, Attr)>),
{
    if TABLE_READY.load(Ordering::Relaxed) {
        return;
    }
    if let Ok(mut t) = TABLE.lock() {
        if t.is_empty() {
            fill(&mut t);
            if !t.is_empty() {
                TABLE_READY.store(true, Ordering::Relaxed);
            }
        }
    }
}

pub fn table_len() -> usize {
    TABLE.lock().map(|t| t.len()).unwrap_or(0)
}

/// phase 훅이 매 AI턴 직전에 호출 — 현재 밴/픽 상태를 기록.
pub fn set_snapshot(s: Snapshot) {
    if let Ok(mut g) = SNAP.lock() {
        *g = Some(s);
    }
}

fn attr_of_name(name: &str) -> Option<Attr> {
    let t = TABLE.lock().ok()?;
    t.iter().find(|(n, _)| n == name).map(|(_, a)| *a)
}

fn attr_of_idx(i: usize) -> Option<(String, Attr)> {
    let t = TABLE.lock().ok()?;
    t.get(i).cloned()
}

/// 조합 요약(빈 조합이면 None).
struct Comp {
    n: f32,
    ad: f32,
    ap: f32,
    front: f32,
    def: f32,
    mr: f32,
}

fn summarize(names: &[String]) -> Option<Comp> {
    let mut c = Comp { n: 0.0, ad: 0.0, ap: 0.0, front: 0.0, def: 0.0, mr: 0.0 };
    for nm in names {
        if let Some(a) = attr_of_name(nm) {
            c.n += 1.0;
            c.ad += a.ad;
            c.ap += a.ap;
            c.front += a.front();
            c.def += a.def;
            c.mr += a.mr;
        }
    }
    if c.n <= 0.0 {
        None
    } else {
        Some(c)
    }
}

/// ★핵심 판정: 밴 후보 c에 대한 보정치.
/// enemy = 상대(=밴하는 팀의 적) 픽, ally = 내 픽.
/// 반환 = (보정치, 사유코드) — 사유는 debug 로그용.
pub fn ban_bias(cand: &Attr, enemy: &[String], ally: &[String], w_syn: f32, w_cnt: f32, cap: f32) -> (f32, f32, f32) {
    // ① 상대 시너지: 상대 조합의 빈 곳을 c가 메우는 정도.
    let mut syn = 0.0f32;
    if let Some(e) = summarize(enemy) {
        // 딜 타입 균형 — 상대가 한쪽으로 쏠려 있고 c가 반대면 그들이 원할 카드.
        let e_axis = if e.ad + e.ap > 0.0 { (e.ad - e.ap) / (e.ad + e.ap) } else { 0.0 };
        let c_axis = cand.dmg_axis();
        // 반대 부호일수록 큼(0~1)
        syn += ((-(e_axis * c_axis)).max(0.0)).min(1.0) * 0.6;
        // 앞라인 부재 — 상대에 앞라인이 없고 c가 탱키하면 그들이 원할 카드.
        let e_front = e.front / e.n;
        if e_front < 0.45 {
            syn += (cand.front() * (0.45 - e_front) / 0.45).clamp(0.0, 1.0) * 0.4;
        }
    }

    // ② 내 조합 위협: c가 내 조합을 잘 때리는 정도(내 저항이 약한 축의 딜).
    let mut cnt = 0.0f32;
    if let Some(a) = summarize(ally) {
        let avg_def = a.def / a.n;
        let avg_mr = a.mr / a.n;
        // 내 방어가 낮으면 물리 위협이 크고, 마저가 낮으면 마법 위협이 크다.
        let phys_risk = (1.0 - (avg_def / 40.0).clamp(0.0, 1.0)) * (cand.ad / 90.0).clamp(0.0, 1.5);
        let magi_risk = (1.0 - (avg_mr / 40.0).clamp(0.0, 1.0)) * (cand.ap / 90.0).clamp(0.0, 1.5);
        cnt += phys_risk.max(magi_risk).min(1.0) * 0.7;
        // 내 조합이 물몸(앞라인 부족)이면 화력 높은 챔프가 더 위협적.
        let a_front = a.front / a.n;
        if a_front < 0.45 {
            cnt += (cand.power() * (0.45 - a_front) / 0.45).clamp(0.0, 1.0) * 0.3;
        }
    }

    let raw = w_syn * syn + w_cnt * cnt;
    (raw.clamp(-cap, cap), syn, cnt)
}

/// score_ban 본체에서 쓰는 진입점: ctx의 밴 개수로 스냅샷 정합성을 검증하고 보정치를 낸다.
/// 반환 None = Pass.
pub fn compute_ban_bias(
    cand_idx: usize,
    ctx_ally_ban: usize,
    ctx_enemy_ban: usize,
    w_syn: f32,
    w_cnt: f32,
    cap: f32,
) -> Option<(f32, String, f32, f32)> {
    let (name, attr) = attr_of_idx(cand_idx)?;
    let g = SNAP.lock().ok()?;
    let s = g.as_ref()?;
    if !s.valid {
        return None;
    }
    // 행동 팀 기준으로 ally/enemy 정렬
    let (ally_ban, enemy_ban, ally_pick, enemy_pick) = if s.acting_t1 {
        (&s.t1_ban, &s.t2_ban, &s.t1_pick, &s.t2_pick)
    } else {
        (&s.t2_ban, &s.t1_ban, &s.t2_pick, &s.t1_pick)
    };
    // 정합성: ctx가 보고한 밴 개수와 스냅샷이 같아야 같은 드래프트로 간주.
    if ally_ban.len() != ctx_ally_ban || enemy_ban.len() != ctx_enemy_ban {
        return None;
    }
    // 픽이 하나도 없으면 바닐라와 동일 상황 → 보정 없음(네이티브에 맡김).
    if ally_pick.is_empty() && enemy_pick.is_empty() {
        return None;
    }
    let (bias, syn, cnt) = ban_bias(&attr, enemy_pick, ally_pick, w_syn, w_cnt, cap);
    Some((bias, name, syn, cnt))
}
