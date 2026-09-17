//! draft — AI/코치 픽 게이트 (`StableDraftHook`).
//! 규칙(클래식과 동일): 후보 마스크 cand 가 "내 팀이 이미 픽한 마스크(pinned) + cand" 로도 서로 다른 포지션 배정이 되면 허용,
//!   아니면 (자유 슬롯이 남아 있으면 허용) 아니면 차단. 어떤 후보도 합법이 아니면 fail-open(개입 없음).
//! 개입 방식: `score_pick` 은 점수를 캡처만 하고(Pass), `decide_pick` 에서 게임이 고를 후보(점수 최대)가 불법일 때만
//!   **합법 후보 중 게임 점수 최대**를 돌려준다. (클래식 09-13 2차 수정과 같은 원칙 — 정렬 첫 챔프 폴백 금지.)
//!   ⚠`score_pick` 의 Replace(-1e9) 는 top-K 랜덤 때문에 배제가 아니라 확률 감소일 뿐(RE 09-15) → decide 가 정본.
//! 인덱스 = ctx 의 챔피언 id(모델 공간) → `champion_name(id)` 로 이름을 얻어 마스크 조회(클래식 인덱스 공간 불일치 문제 원천 제거).
//! 밴 단계는 관여하지 않는다.
use crate::assign;
use crate::config::{self, MASK_ALL};
use mod_api_stable::{DraftPhaseV1, StableDraftContext, StableDraftDecision, StableDraftHook};
use std::cell::RefCell;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

pub static CNT_SCORE: AtomicU64 = AtomicU64::new(0);
pub static CNT_DECIDE: AtomicU64 = AtomicU64::new(0);
pub static CNT_REPLACE: AtomicU64 = AtomicU64::new(0);
pub static CNT_FAILOPEN: AtomicU64 = AtomicU64::new(0);
static LOGS: Mutex<Vec<String>> = Mutex::new(Vec::new());
fn qlog(s: String) { let mut g = LOGS.lock().unwrap_or_else(|e| e.into_inner()); if g.len() < 200 { g.push(s); } }
/// 메인 스레드에서 호출 — 워커 스레드가 쌓은 로그를 파일로.
pub fn drain_logs() {
    let v: Vec<String> = std::mem::take(&mut *LOGS.lock().unwrap_or_else(|e| e.into_inner()));
    for l in v { config::dlog(&l); }
}

thread_local! {
    /// 이번 결정의 (후보 id, 게임 점수) — score_pick 이 채우고 decide_pick 이 소비. 스레드로컬 = 같은 스레드에서 연속 호출.
    static SCORES: RefCell<Vec<(usize, f32)>> = RefCell::new(Vec::new());
    static SCORE_KEY: RefCell<u64> = RefCell::new(0);
}
fn ctx_key(ctx: &StableDraftContext<'_>) -> u64 {
    // 같은 결정인지 식별: 픽/밴 수 + 가용 수 (결정마다 바뀜)
    ((ctx.ally_picks().len() as u64) << 48) | ((ctx.enemy_picks().len() as u64) << 40) | ((ctx.ally_bans().len() as u64) << 32) | (ctx.available_champions().len() as u64)
}

fn mask_of_id(ctx: &StableDraftContext<'_>, id: usize) -> u8 {
    ctx.champion_name(id).map(|n| crate::mask_of(&n.to_ascii_lowercase())).unwrap_or(MASK_ALL)
}

/// 모델 인덱스 공간 이름표(champion_briefs 순서) → legacy_assign(hookA). 길이가 바뀔 때만 게시.
fn publish_model(ctx: &StableDraftContext<'_>) {
    let briefs = ctx.champion_briefs();
    if briefs.is_empty() || briefs.len() == crate::legacy_assign::model_len() { return; }
    let names: Vec<String> = briefs.iter().map(|b| unsafe { b.name.as_str() }.to_ascii_lowercase()).collect();
    crate::legacy_assign::publish_model_names(names);
}

pub struct PosLockDraft;
impl StableDraftHook for PosLockDraft {
    fn id(&self) -> String { "tfm2_champ_pos_lock.pick_gate".into() }
    fn priority(&self) -> i32 { 100 }
    fn score_pick(&self, ctx: &StableDraftContext<'_>, candidate: usize, base_score: f32) -> StableDraftDecision {
        CNT_SCORE.fetch_add(1, Ordering::Relaxed);
        publish_model(ctx);
        let key = ctx_key(ctx);
        SCORE_KEY.with(|k| { let mut k = k.borrow_mut(); if *k != key { *k = key; SCORES.with(|s| s.borrow_mut().clear()); } });
        SCORES.with(|s| { let mut s = s.borrow_mut(); if let Some(e) = s.iter_mut().find(|e| e.0 == candidate) { e.1 = base_score; } else if s.len() < 512 { s.push((candidate, base_score)); } });
        StableDraftDecision::Pass
    }
    fn decide_pick(&self, ctx: &StableDraftContext<'_>) -> Option<usize> {
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| decide(ctx)));
        match r { Ok(v) => v, Err(_) => { qlog("decide_pick 패닉".into()); None } }
    }
}

fn decide(ctx: &StableDraftContext<'_>) -> Option<usize> {
    CNT_DECIDE.fetch_add(1, Ordering::Relaxed);
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_pick_gate || !config::any_restricted() { return None; }
    if ctx.phase() != Some(DraftPhaseV1::Pick) { return None; }
    let avail = ctx.available_champions();
    if avail.is_empty() { return None; }
    let pinned_all: Vec<u8> = ctx.ally_picks().iter().map(|&i| mask_of_id(ctx, i)).collect();
    let team = 5usize;
    if pinned_all.len() >= team { return None; }
    let pool: Vec<u8> = avail.iter().map(|&i| mask_of_id(ctx, i)).collect();
    let key = ctx_key(ctx);
    let scores: Vec<(usize, f32)> = SCORE_KEY.with(|k| if *k.borrow() == key { SCORES.with(|s| s.borrow().clone()) } else { Vec::new() });
    // 게임이 고를 후보(점수 최대). 점수를 못 봤으면 개입 근거가 없다 → 그대로.
    let Some(&(top, top_score)) = scores.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)) else { return None };
    let legal = |id: usize| assign::pick_allowed(&pinned_all, &pool, team, mask_of_id(ctx, id)).0;
    if legal(top) { return None; }
    // 합법 후보 중 점수 최대(점수표에 있는 것 우선, 없으면 가용 순서 첫 합법)
    let mut best: Option<(usize, f32)> = None;
    for &(id, sc) in &scores { if avail.contains(&id) && legal(id) && best.map(|b| sc > b.1).unwrap_or(true) { best = Some((id, sc)); } }
    if best.is_none() { if let Some(&id) = avail.iter().find(|&&id| legal(id)) { best = Some((id, f32::NEG_INFINITY)); } }
    match best {
        Some((id, sc)) => {
            CNT_REPLACE.fetch_add(1, Ordering::Relaxed);
            if cfg.debug { qlog(format!("decide: {}({:.3}) 불법 → {}({:.3}) | ally={:?}", ctx.champion_name(top).unwrap_or("?"), top_score, ctx.champion_name(id).unwrap_or("?"), sc, ctx.ally_picks().iter().map(|&i| ctx.champion_name(i).unwrap_or("?")).collect::<Vec<_>>())); }
            if cfg.ai_observe_only { None } else { Some(id) }
        }
        None => { CNT_FAILOPEN.fetch_add(1, Ordering::Relaxed); None } // 합법 후보 0 = fail-open
    }
}
