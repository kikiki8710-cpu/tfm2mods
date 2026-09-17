//! ui_block — 밴픽 화면: 유저 픽 차단(회색 오버레이 + 클릭 흡수) / 스왑 화면: 확정 게이트 (전부 stable UI 경로 읽기·스폰).
//! 씬 raw 읽기(클래식 O_BAN1/O_PICK1…)를 **UI 노드 상태**로 대체:
//!   · 카드 `main.champions.contents.<champ>`: `.ban`(밴됨)·`.blue`/`.red`(픽됨)·`.fearless_icon`(이전 세트 잠금) visible
//!   · 턴: `main.{blue,red}_picks.pick_slot_n.in_turn` visible → 픽 턴(진영) / `main.bottom.{blue,red}_side.bans.*.in_turn` → 밴 턴
//!   · 내 진영: `main.bottom.{blue,red}_side.name` 텍스트 == 내 팀 이름
//!   · 픽 슬롯 ↔ 챔피언: 카드 색 오버레이 신규 ↔ 새 `done` 슬롯 순서 짝짓기(banpick_view_plus 검증 기법) → `done.name`(선수명) 으로 스왑 표와 연결
//! 차단 규칙 = 클래식 `recompute_blocklist` 그대로(자유 슬롯 있으면 차단 없음 / helps 실패 = 차단 / 합법 0 = fail-open).
use crate::assign;
use crate::config::{self, MASK_ALL};
use crate::{i18n, uk};
use mod_api_stable::StableClient;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;

const ROOT: &str = "main";
const CARDS: &str = "main.champions.contents";
const OVERLAY: &str = "pl_block";
const TIP: &str = "pl_tip";
const SWAPTIP_UI: &str = include_str!("../assets/pos_lock_swaptip.ui");

static MY_SIDE: AtomicI32 = AtomicI32::new(-1); // 0 blue 1 red -1 미상
static SIDE_CHECK_AT: AtomicU64 = AtomicU64::new(0);
static OVERLAYS: Mutex<Option<HashSet<String>>> = Mutex::new(None); // 오버레이 스폰된 카드 id
static BLOCKED: Mutex<Option<HashSet<String>>> = Mutex::new(None);   // 현재 차단 집합(표시 상태)
static REASON: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);
static TIP_TEXT: Mutex<Option<(String, u64)>> = Mutex::new(None); // (본문, 표시 시작 프레임)
static GATE_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
static SWAP_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
static CNT_SPAWN_FAIL: AtomicUsize = AtomicUsize::new(0);
static REGISTERED: Mutex<Option<HashSet<String>>> = Mutex::new(None);
pub static MY_PICK_TURN: AtomicBool = AtomicBool::new(false);
pub static BAN_CNT_SEEN: AtomicUsize = AtomicUsize::new(0);

#[derive(Default)]
struct PickState { slot_champ: HashMap<(bool, usize), String>, known: Vec<(bool, String)> }
static PICK_STATE: Mutex<Option<PickState>> = Mutex::new(None);
// ★성능(2026-09-17): 카드 전수 스캔(카드당 ui_visible 4회)은 내 픽 차례이거나 done 슬롯이 바뀐 직후·120프레임마다만. 그 외엔 캐시.
static SCAN_CACHE: Mutex<Option<(HashSet<String>, Vec<(bool, String)>, usize)>> = Mutex::new(None); // (taken, picked, ban_n)
static LAST_DONE: Mutex<Vec<(bool, usize)>> = Mutex::new(Vec::new());
static DONE_CHANGED_AT: AtomicU64 = AtomicU64::new(0);
const TICK_EVERY: u64 = 4;

pub fn reset() {
    MY_SIDE.store(-1, Ordering::Relaxed);
    *PICK_STATE.lock().unwrap_or_else(|e| e.into_inner()) = None;
    *BLOCKED.lock().unwrap_or_else(|e| e.into_inner()) = None;
    MY_PICK_TURN.store(false, Ordering::Relaxed);
}

fn vis(ctx: &StableClient<'_>, p: &str) -> bool { ctx.ui_visible(p) == Some(true) }
fn side_name(side: bool) -> &'static str { if side { "blue" } else { "red" } }

/// 밴픽/스왑 화면 프레임 처리.
pub fn tick(ctx: &mut StableClient<'_>) {
    let cfg = config::get();
    if !ctx.ui_exists(CARDS) || !vis(ctx, "main.champions_bg") && !vis(ctx, "main.swap") {
        // 밴픽 화면 아님 → 상태 리셋(오버레이는 트리와 함께 사라짐)
        if OVERLAYS.lock().unwrap_or_else(|e| e.into_inner()).is_some() { *OVERLAYS.lock().unwrap_or_else(|e| e.into_inner()) = None; *BLOCKED.lock().unwrap_or_else(|e| e.into_inner()) = None; *PICK_STATE.lock().unwrap_or_else(|e| e.into_inner()) = None; MY_SIDE.store(-1, Ordering::Relaxed); uk::index_clear(); }
        MY_PICK_TURN.store(false, Ordering::Relaxed);
        return;
    }
    let f = crate::FRAME.load(Ordering::Relaxed);
    if f % TICK_EVERY != 0 { return; }
    // ── 내 진영(30프레임마다 재확인)
    if MY_SIDE.load(Ordering::Relaxed) < 0 || f.saturating_sub(SIDE_CHECK_AT.load(Ordering::Relaxed)) > 30 {
        SIDE_CHECK_AT.store(f, Ordering::Relaxed);
        let my = crate::PLAYER_TEAM.load(Ordering::Relaxed);
        if my != u64::MAX { if let Some(name) = ctx.team_name(my as usize) {
            let b = ctx.ui_text("main.bottom.blue_side.name").unwrap_or_default();
            let r = ctx.ui_text("main.bottom.red_side.name").unwrap_or_default();
            let side = if b == name { 0 } else if r == name { 1 } else { -1 };
            if MY_SIDE.swap(side, Ordering::Relaxed) != side { config::llog(&format!("side: 내 팀 '{}' → {} (blue='{}' red='{}')", name, side, b, r)); }
        } }
    }
    // ── 픽 슬롯 done 집합(싼 신호 10회) — 바뀐 직후에만 카드 전수 스캔
    let mut done_now: Vec<(bool, usize)> = Vec::new();
    for side in [true, false] { for n in 0..5 {
        let slot = format!("main.{}_picks.pick_slot_{}", side_name(side), n);
        if vis(ctx, &format!("{}.done", slot)) { done_now.push((side, n)); }
    } }
    { let mut ld = LAST_DONE.lock().unwrap_or_else(|e| e.into_inner()); if *ld != done_now { *ld = done_now.clone(); DONE_CHANGED_AT.store(f, Ordering::Relaxed); } }
    // ── 턴 판정(싼 신호)
    let pick_turn_side: Option<bool> = {
        let b = (0..5).any(|n| vis(ctx, &format!("main.blue_picks.pick_slot_{}.in_turn", n)));
        let r = (0..5).any(|n| vis(ctx, &format!("main.red_picks.pick_slot_{}.in_turn", n)));
        match (b, r) { (true, false) => Some(true), (false, true) => Some(false), _ => None }
    };
    let ban_turn = pick_turn_side.is_none() && ["blue", "red"].iter().any(|s| { let bans = format!("main.bottom.{}_side.bans", s); ctx.ui_child_names(&bans).iter().any(|c| vis(ctx, &format!("{}.{}.in_turn", bans, c))) });
    let my_side = MY_SIDE.load(Ordering::Relaxed);
    let my_turn = my_side >= 0 && pick_turn_side.map(|b| (b && my_side == 0) || (!b && my_side == 1)).unwrap_or(false);
    let recently_changed = f.saturating_sub(DONE_CHANGED_AT.load(Ordering::Relaxed)) <= 24;
    let need_scan = (my_turn && !ban_turn) || recently_changed || f % 120 == 0 || SCAN_CACHE.lock().unwrap_or_else(|e| e.into_inner()).is_none();
    // ── 카드 상태 수집(필요할 때만 전수 스캔, 아니면 캐시)
    let cards: Vec<String> = if need_scan { ctx.ui_child_names(CARDS) } else { Vec::new() };
    let (taken, picked, ban_n): (HashSet<String>, Vec<(bool, String)>, usize) = if need_scan {
        let mut taken: HashSet<String> = HashSet::new();
        let mut picked: Vec<(bool, String)> = Vec::new();
        let mut ban_n = 0usize;
        for id in &cards {
            let cp = format!("{}.{}", CARDS, id);
            let lower = id.to_ascii_lowercase();
            if vis(ctx, &format!("{}.ban", cp)) { taken.insert(lower.clone()); ban_n += 1; }
            if vis(ctx, &format!("{}.fearless_icon", cp)) { taken.insert(lower.clone()); }
            if vis(ctx, &format!("{}.blue", cp)) { taken.insert(lower.clone()); picked.push((true, lower.clone())); }
            if vis(ctx, &format!("{}.red", cp)) { taken.insert(lower.clone()); picked.push((false, lower.clone())); }
        }
        *SCAN_CACHE.lock().unwrap_or_else(|e| e.into_inner()) = Some((taken.clone(), picked.clone(), ban_n));
        (taken, picked, ban_n)
    } else { SCAN_CACHE.lock().unwrap_or_else(|e| e.into_inner()).clone().unwrap_or_default() };
    // ── 픽 슬롯 ↔ 챔피언 짝짓기(선수명은 짝짓기 변화 때만 읽음)
    let mut done_name: HashMap<(bool, usize), String> = HashMap::new();
    if recently_changed { for &(side, n) in &done_now { if let Some(nm) = ctx.ui_text(&format!("main.{}_picks.pick_slot_{}.done.name", side_name(side), n)) { done_name.insert((side, n), nm); } } }
    {
        let mut g = PICK_STATE.lock().unwrap_or_else(|e| e.into_inner());
        let st = g.get_or_insert_with(PickState::default);
        st.slot_champ.retain(|k, _| done_now.contains(k));
        st.known.retain(|k| picked.contains(k));
        for side in [true, false] {
            let mut new_champs: Vec<String> = picked.iter().filter(|(b, id)| *b == side && !st.known.contains(&(side, id.clone())) && !st.slot_champ.values().any(|v| v == id)).map(|(_, id)| id.clone()).collect();
            let mut new_slots: Vec<(bool, usize)> = done_now.iter().filter(|k| k.0 == side && !st.slot_champ.contains_key(k)).copied().collect();
            new_slots.sort();
            for (k, id) in new_slots.iter().zip(new_champs.drain(..)) { st.slot_champ.insert(*k, id.clone()); st.known.push((side, id)); }
        }
    }
    if pick_turn_side.is_some() && !ban_turn && need_scan { let per = ban_n / 2; if per <= 5 && BAN_CNT_SEEN.swap(per + 1, Ordering::Relaxed) != per + 1 { let (st, cb) = config::cur_rule(); if cb != Some(per) { config::set_rule(st, Some(per)); config::dlog(&format!("밴카드 관측: {}장/팀 (구 {:?})", per, cb)); } } }
    MY_PICK_TURN.store(my_turn, Ordering::Relaxed);
    // ── 스왑 확정 게이트
    if vis(ctx, "main.swap") { swap_gate(ctx, my_side); } else { SWAP_SIG.store(u64::MAX, Ordering::Relaxed); }
    // ── 차단 집합 계산
    let mut block: HashSet<String> = HashSet::new();
    let mut reason: HashMap<String, String> = HashMap::new();
    let gate_on = cfg.enabled && cfg.user_pick_block && config::any_restricted() && my_turn && !ban_turn;
    if gate_on {
        if let (Some(masks), Some(roster)) = (crate::masks(), crate::roster()) {
            let my_picks: Vec<&String> = picked.iter().filter(|(b, _)| (*b && my_side == 0) || (!*b && my_side == 1)).map(|(_, id)| id).collect();
            if my_picks.len() < 5 {
                let pinned_all: Vec<u8> = my_picks.iter().map(|id| masks.get(*id).copied().unwrap_or(MASK_ALL)).collect();
                let pinned: Vec<u8> = pinned_all.iter().copied().filter(|&m| m != MASK_ALL).collect();
                let pool: Vec<u8> = roster.ids.iter().filter(|id| !taken.contains(*id)).map(|id| masks.get(id).copied().unwrap_or(MASK_ALL)).collect();
                let free = assign::free_left(&pinned_all, &pool, 5);
                if free == 0 {
                    let mut any_feasible = false;
                    for id in &roster.ids {
                        if taken.contains(id) { continue; }
                        let m = masks.get(id).copied().unwrap_or(MASK_ALL);
                        if m == MASK_ALL { any_feasible = true; continue; }
                        if assign::helps(&pinned, m) { any_feasible = true; } else {
                            let label = (0..5).filter(|p| m & (1 << p) != 0).map(i18n::pos_name).collect::<Vec<_>>().join("/");
                            reason.insert(id.clone(), label); block.insert(id.clone());
                        }
                    }
                    if !any_feasible { block.clear(); reason.clear(); } // fail-open
                }
                let sig = { use std::hash::{Hash, Hasher}; let mut h = std::collections::hash_map::DefaultHasher::new(); (my_picks.len(), free, block.len(), my_side).hash(&mut h); h.finish() };
                if GATE_SIG.swap(sig, Ordering::Relaxed) != sig { config::llog(&format!("gate: 내픽={} 자유슬롯={} 차단={} side={}", my_picks.len(), free, block.len(), my_side)); }
            }
        }
    }
    // ── 오버레이 반영(변화가 있을 때만)
    let changed = { let g = BLOCKED.lock().unwrap_or_else(|e| e.into_inner()); g.as_ref() != Some(&block) };
    if changed {
        let cards: Vec<String> = if cards.is_empty() { ctx.ui_child_names(CARDS) } else { cards.clone() };
        let mut ov = OVERLAYS.lock().unwrap_or_else(|e| e.into_inner());
        let spawned = ov.get_or_insert_with(HashSet::new);
        for id in &cards {
            let lower = id.to_ascii_lowercase();
            let cp = format!("{}.{}", CARDS, id);
            let want = block.contains(&lower);
            let op = format!("{}.{}", cp, OVERLAY);
            if !ctx.ui_exists(&op) {
                if !want { continue; }
                let src = format!("{}:color_icon_button {{ width: 100%; height: 100%; z: 50; btn: {{ color: #0f1016c8; hover: {{ color: #0f1016d8; }} active: {{ color: #0f1016e8; }} }} }}", OVERLAY);
                if !ctx.ui_spawn_source(&cp, &src) || !ctx.ui_exists(&op) { CNT_SPAWN_FAIL.fetch_add(1, Ordering::Relaxed); continue; }
                spawned.insert(lower.clone());
                let id2 = lower.clone();
                let mut rg = REGISTERED.lock().unwrap_or_else(|e| e.into_inner());
                let rs = rg.get_or_insert_with(HashSet::new);
                if !rs.contains(&op) { rs.insert(op.clone()); ctx.ui_register_click(&op, "", move |_| { let r = REASON.lock().unwrap_or_else(|e| e.into_inner()).as_ref().and_then(|m| m.get(&id2).cloned()); *TIP_TEXT.lock().unwrap_or_else(|e| e.into_inner()) = Some((crate::block_msg(&id2, r.as_deref()), crate::FRAME.load(Ordering::Relaxed))); }); }
            }
            uk::set_props_if_changed(ctx, &op, "visible", if want { "true" } else { "false" });
        }
        *REASON.lock().unwrap_or_else(|e| e.into_inner()) = Some(reason);
        *BLOCKED.lock().unwrap_or_else(|e| e.into_inner()) = Some(block);
    }
    // ── 사유 툴팁(클릭 후 2초)
    let tip = TIP_TEXT.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let tp = format!("{}.{}", ROOT, TIP);
    match tip {
        Some((text, at)) if f.saturating_sub(at) < 120 => {
            if !ctx.ui_exists(&tp) {
                let src = SWAPTIP_UI.replacen("pos_lock_swaptip:color {", &format!("{}:color {{ z: 3000; width: 420px;", TIP), 1);
                if !ctx.ui_spawn_source(ROOT, &src) { return; }
            }
            if let Some((mx, my)) = uk::cursor_ui() { uk::set_props_if_changed(ctx, &tp, "x", &format!("{}px", (mx - 210.0).clamp(0.0, 1500.0))); uk::set_props_if_changed(ctx, &tp, "y", &format!("{}px", (my - 44.0).max(0.0))); }
            let tt = format!("{}.text", tp);
            if ctx.ui_text(&tt).as_deref() != Some(text.as_str()) { ctx.ui_set_text(&tt, &text); }
            uk::set_props_if_changed(ctx, &tp, "visible", "true");
        }
        _ => { if ctx.ui_exists(&tp) { uk::set_props_if_changed(ctx, &tp, "visible", "false"); } }
    }
}

/// 스왑 화면: 내 표의 (포지션 행, 선수) → 챔피언 → 마스크 위반 시 확정 버튼 비활성 + 툴팁.
fn swap_gate(ctx: &mut StableClient<'_>, my_side: i32) {
    let cfg = config::get();
    let confirm = "main.swap.bottom.confirm";
    if !ctx.ui_exists(confirm) { return; }
    let enforce = cfg.enabled && config::any_restricted() && my_side >= 0;
    let mut violations: Vec<String> = Vec::new();
    let mut resolved = 0usize;
    if enforce {
        let table = format!("main.swap.{}_table", if my_side == 0 { "blue" } else { "red" });
        let slot_champ: HashMap<String, String> = {
            let g = PICK_STATE.lock().unwrap_or_else(|e| e.into_inner());
            let mut m = HashMap::new();
            if let Some(st) = g.as_ref() { for ((side, n), champ) in &st.slot_champ { if (*side && my_side == 0) || (!*side && my_side == 1) { if let Some(nm) = ctx.ui_text(&format!("main.{}_picks.pick_slot_{}.done.name", side_name(*side), n)) { m.insert(nm, champ.clone()); } } } }
            m
        };
        let pos_texts: Vec<String> = ["top", "jungle", "mid", "bottom", "support"].iter().map(|p| ctx.i18n(&format!("#asset/base/text/ui?position.{}", p)).unwrap_or_default()).collect();
        for row in ctx.ui_child_names(&table) {
            let rp = format!("{}.{}", table, row);
            let Some(ath) = ctx.ui_text(&format!("{}.data.name_slot.text", rp)) else { continue };
            let Some(pt) = ctx.ui_text(&format!("{}.data.position_name", rp)) else { continue };
            let Some(p) = pos_texts.iter().position(|t| !t.is_empty() && *t == pt) else { continue };
            let Some(champ) = slot_champ.get(&ath) else { continue };
            resolved += 1;
            let m = crate::mask_of(champ);
            if m != MASK_ALL && m & (1 << p) == 0 { violations.push(format!("{}→{}", crate::disp_name(champ), i18n::pos_name(p))); }
        }
    }
    let bad = resolved == 5 && !violations.is_empty();
    let sig = { use std::hash::{Hash, Hasher}; let mut h = std::collections::hash_map::DefaultHasher::new(); (resolved, &violations, bad).hash(&mut h); h.finish() };
    if SWAP_SIG.swap(sig, Ordering::Relaxed) != sig { config::llog(&format!("swapgate: resolved={} bad={} {:?}", resolved, bad, violations)); }
    uk::set_props_if_changed(ctx, confirm, "disable", if bad { "true" } else { "false" });
    let tp = "main.pl_swaptip";
    if bad {
        if !ctx.ui_exists(tp) { let src = SWAPTIP_UI.replacen("pos_lock_swaptip:color {", "pl_swaptip:color { z: 3000;", 1); if !ctx.ui_spawn_source(ROOT, &src) { return; } }
        if let Some((x, y, w, _h)) = ctx.ui_node_rect(confirm) { uk::set_props_if_changed(ctx, tp, "x", &format!("{}px", x + w / 2.0 - 120.0)); uk::set_props_if_changed(ctx, tp, "y", &format!("{}px", y - 40.0)); }
        uk::set_props_if_changed(ctx, tp, "visible", "true");
    } else if ctx.ui_exists(tp) { uk::set_props_if_changed(ctx, tp, "visible", "false"); }
}
