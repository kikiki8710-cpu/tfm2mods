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
static SWAP_DIAG: AtomicBool = AtomicBool::new(false);
static SIDE_FAIL_LOGGED: AtomicBool = AtomicBool::new(false);
static DISABLED: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);
static CNT_SPAWN_FAIL: AtomicUsize = AtomicUsize::new(0);
static REGISTERED: Mutex<Option<HashSet<String>>> = Mutex::new(None);
/// ★09-18 스왑 추적: 0.6.0 스왑 표 행(swap_slot_n)의 이름/포지션 텍스트는 stable API 로 못 읽는다(swapdiag: runner None·text None).
///   행 클릭을 등록해 게임의 조작(행 A 선택 → 행 B 클릭 = A↔B 교환 / 같은 행 재클릭 = 해제)을 따라가고, 행 k = 포지션 k(라인업 순서 탑→서폿),
///   행 k ↔ 픽 슬롯 = 같은 팀 pick_slot_n 을 y 오름차순 정렬한 k 번째. ⚠코치 위임 스왑은 클릭이 없어 추적 불가 → 게이트가 옛 배치로 판정할 수 있다(한계).
static SWAP_SEL: Mutex<Option<usize>> = Mutex::new(None);
static SWAP_ACTIVE: AtomicBool = AtomicBool::new(false);
static SWAP_CLICKS: Mutex<Vec<usize>> = Mutex::new(Vec::new());
static SWAP_LAST_CLICK: Mutex<Option<(u64, usize)>> = Mutex::new(None);
static RAW_SRC: AtomicBool = AtomicBool::new(false);
static CONFIRM_PAINT: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(u8::MAX);
static RAW_LOGGED: AtomicBool = AtomicBool::new(false);
/// 스왑 진입당 1회(스왑 화면을 떠날 때 리셋).
fn f_once_swap() -> bool { !RAW_LOGGED.swap(true, Ordering::Relaxed) }
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
    SWAP_ACTIVE.store(false, Ordering::Relaxed);
    *SWAP_SEL.lock().unwrap_or_else(|e| e.into_inner()) = None;
    *REGISTERED.lock().unwrap_or_else(|e| e.into_inner()) = None;
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
        if OVERLAYS.lock().unwrap_or_else(|e| e.into_inner()).is_some() { *OVERLAYS.lock().unwrap_or_else(|e| e.into_inner()) = None; *BLOCKED.lock().unwrap_or_else(|e| e.into_inner()) = None; *PICK_STATE.lock().unwrap_or_else(|e| e.into_inner()) = None; MY_SIDE.store(-1, Ordering::Relaxed); *DISABLED.lock().unwrap_or_else(|e| e.into_inner()) = None; uk::index_clear(); }
        MY_PICK_TURN.store(false, Ordering::Relaxed);
        return;
    }
    let f = crate::FRAME.load(Ordering::Relaxed);
    if f % TICK_EVERY != 0 { return; }
    // ── 내 진영(30프레임마다 재확인)
    if MY_SIDE.load(Ordering::Relaxed) < 0 || f.saturating_sub(SIDE_CHECK_AT.load(Ordering::Relaxed)) > 30 {
        SIDE_CHECK_AT.store(f, Ordering::Relaxed);
        let my = crate::PLAYER_TEAM.load(Ordering::Relaxed);
        if MY_SIDE.load(Ordering::Relaxed) < 0 && !SIDE_FAIL_LOGGED.load(Ordering::Relaxed) && f % 240 == 0 {
            // ★09-17 진단: side 미확정 원인(PLAYER_TEAM 미캡처 / team_name None / 하단 텍스트 불일치) 1회
            let nm = if my != u64::MAX { ctx.team_name(my as usize) } else { None };
            let b = ctx.ui_text("main.bottom.blue_side.name"); let r = ctx.ui_text("main.bottom.red_side.name");
            config::llog(&format!("side-diag: player_team={} name={:?} blue={:?} red={:?} bottom_kids={:?}", my as i64, nm, b, r, ctx.ui_child_names("main.bottom")));
            SIDE_FAIL_LOGGED.store(true, Ordering::Relaxed);
        }
        if my != u64::MAX { if let Some(name) = ctx.team_name(my as usize) {
            let b = ctx.ui_text("main.bottom.blue_side.name").unwrap_or_default();
            let r = ctx.ui_text("main.bottom.red_side.name").unwrap_or_default();
            // ★09-17: 밴픽 하단 팀명은 "Gen.G #2" 처럼 시드 접미가 붙는다 → 완전일치가 아니라 접두 일치(공백/# 앞까지).
            let eq = |ui: &str| { let ui = ui.trim(); ui == name || ui.strip_prefix(name.as_str()).map(|rest| rest.trim_start().starts_with('#') || rest.trim().is_empty()).unwrap_or(false) };
            let side = if eq(&b) { 0 } else if eq(&r) { 1 } else { -1 };
            if MY_SIDE.swap(side, Ordering::Relaxed) != side || (side < 0 && !SIDE_FAIL_LOGGED.swap(true, Ordering::Relaxed)) { config::llog(&format!("side: 내 팀 '{}' → {} (blue='{}' red='{}')", name, side, b, r)); }
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
    if vis(ctx, "main.swap") { swap_track(ctx, my_side); swap_gate(ctx, my_side); } else { SWAP_SIG.store(u64::MAX, Ordering::Relaxed); SWAP_DIAG.store(false, Ordering::Relaxed); if SWAP_ACTIVE.swap(false, Ordering::Relaxed) { *SWAP_SEL.lock().unwrap_or_else(|e| e.into_inner()) = None; } RAW_LOGGED.store(false, Ordering::Relaxed); crate::swap_confirm_hook::BLOCK.store(false, Ordering::Relaxed); CONFIRM_PAINT.store(u8::MAX, Ordering::Relaxed); }
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
                // ★0.6.0: z 속성 제거(z 가 스폰 노드 렌더를 죽임 — 팝업과 동일 증상, 09-17 유저 제보 "차단 안 보임"). 카드 마지막 자식이라 트리 순서로 위에 그려진다.
                // 밴 카드와 같은 계열(회색 #666666 + 우상단 ⊘ 아이콘) — 게임의 `.ban` 자식은 스캐너가 "밴됨" 판정에 쓰므로 건드리지 않고 덮개에 아이콘을 얹는다.
                let src = format!("{}:color_icon_button {{ width: 100%; height: 100%; rounding: Uniform {{ rounding: 12; }} btn: {{ color: #666666b8; hover: {{ color: #6e6e6ec8; }} active: {{ color: #6e6e6ec8; }} }} icon: {{ source: \"asset/base/ui/icons/ban\"; rect: {{ x: 95; y: 6; w: 18; h: 18; }} }} }}", OVERLAY);
                if !ctx.ui_spawn_source(&cp, &src) || !ctx.ui_exists(&op) { CNT_SPAWN_FAIL.fetch_add(1, Ordering::Relaxed); continue; }
                spawned.insert(lower.clone());
                let id2 = lower.clone();
                let mut rg = REGISTERED.lock().unwrap_or_else(|e| e.into_inner());
                let rs = rg.get_or_insert_with(HashSet::new);
                if !rs.contains(&op) { rs.insert(op.clone()); ctx.ui_register_click(&op, "", move |_| { let r = REASON.lock().unwrap_or_else(|e| e.into_inner()).as_ref().and_then(|m| m.get(&id2).cloned()); *TIP_TEXT.lock().unwrap_or_else(|e| e.into_inner()) = Some((crate::block_msg(&id2, r.as_deref()), crate::FRAME.load(Ordering::Relaxed))); }); }
            }
            if ctx.ui_visible(&op) != Some(want) { ctx.ui_set_visible(&op, want); }
            // ★09-17(유저 제보 "회색인데 클릭하면 선택됨"): 0.6.0 에선 덮개가 클릭을 흡수하지 못한다 → 카드 자체를 disable.
            //   (게임의 비활성 스타일도 따라온다 — 밴/피어리스 회색과 같은 계열.) 덮개는 사유 툴팁 클릭용으로 유지.
            let dis = format!("{}|disable", cp);
            let want_s = if want { "true" } else { "false" };
            let mut dc = DISABLED.lock().unwrap_or_else(|e| e.into_inner());
            let m = dc.get_or_insert_with(HashMap::new);
            if m.get(&dis).map(|v| v != want_s).unwrap_or(want) { ctx.ui_set_properties(&cp, &format!("disabled: {};", want_s)); m.insert(dis, want_s.to_string()); }
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
                let src = SWAPTIP_UI.replacen("pos_lock_swaptip:color {", &format!("{}:color {{ width: 420px;", TIP), 1);
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

/// 같은 팀 pick_slot_n 을 y 오름차순으로 정렬한 슬롯 번호 목록(행 k ↔ order[k]).
fn slot_order(ctx: &StableClient<'_>, is_blue: bool) -> Vec<usize> {
    let mut v: Vec<(usize, f32)> = (0..5).filter_map(|k| ctx.ui_node_rect(&format!("main.{}_picks.pick_slot_{}", side_name(is_blue), k)).map(|r| (k, r.1))).collect();
    v.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap_or(std::cmp::Ordering::Equal));
    v.into_iter().map(|(k, _)| k).collect()
}

/// 스왑 화면: 내 표 행 클릭 등록(진입마다) + 클릭 큐 처리(선택/교환) → PICK_STATE.slot_champ 갱신.
fn swap_track(ctx: &mut StableClient<'_>, my_side: i32) {
    if my_side < 0 { return; }
    let is_blue = my_side == 0;
    let table = format!("main.swap.{}_table", side_name(is_blue));
    if !SWAP_ACTIVE.swap(true, Ordering::Relaxed) {
        let mut ok = 0;
        for n in 0..5 { let p = format!("{}.swap_slot_{}", table, n); if ctx.ui_exists(&p) && ctx.ui_register_click(&p, "", move |_| SWAP_CLICKS.lock().unwrap_or_else(|e| e.into_inner()).push(n)) { ok += 1; } }
        *SWAP_SEL.lock().unwrap_or_else(|e| e.into_inner()) = None;
        SWAP_CLICKS.lock().unwrap_or_else(|e| e.into_inner()).clear();
        config::llog(&format!("swaptrack: 진입 — 행 클릭 등록 {}/5 ({})", ok, table));
    }
    let clicks: Vec<usize> = std::mem::take(&mut *SWAP_CLICKS.lock().unwrap_or_else(|e| e.into_inner()));
    if clicks.is_empty() { return; }
    let f = crate::FRAME.load(Ordering::Relaxed);
    for n in clicks {
        { let mut lc = SWAP_LAST_CLICK.lock().unwrap_or_else(|e| e.into_inner()); if *lc == Some((f, n)) { continue; } *lc = Some((f, n)); }
        let mut sel = SWAP_SEL.lock().unwrap_or_else(|e| e.into_inner());
        match *sel {
            Some(a) if a == n => { *sel = None; }
            Some(a) => {
                *sel = None;
                let order = slot_order(ctx, is_blue);
                if let (Some(&ka), Some(&kb)) = (order.get(a), order.get(n)) {
                    let mut g = PICK_STATE.lock().unwrap_or_else(|e| e.into_inner());
                    if let Some(st) = g.as_mut() {
                        let va = st.slot_champ.get(&(is_blue, ka)).cloned(); let vb = st.slot_champ.get(&(is_blue, kb)).cloned();
                        match (va, vb) {
                            (Some(va), Some(vb)) => { st.slot_champ.insert((is_blue, ka), vb.clone()); st.slot_champ.insert((is_blue, kb), va.clone()); config::llog(&format!("swaptrack: 행{}↔행{} = 슬롯{}({})↔슬롯{}({})", a, n, ka, va, kb, vb)); }
                            _ => config::llog(&format!("swaptrack: 슬롯 {}/{} 챔피언 미확정 — 건너뜀", ka, kb)),
                        }
                    }
                }
            }
            None => { *sel = Some(n); }
        }
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
        let rows = ctx.ui_child_names(&table);
        // ★진단(0.6.0 검증): 행 매칭 실패 원인 규명용 — 스왑 진입당 1회
        if SWAP_DIAG.swap(true, Ordering::Relaxed) == false {
            let first = rows.first().map(|r| format!("{}.{}", table, r));
            let first_kids = first.as_ref().map(|f| ctx.ui_child_names(f)).unwrap_or_default();
            let first_data = first.as_ref().map(|f| ctx.ui_child_names(&format!("{}.data", f))).unwrap_or_default();
            let bottom = ctx.ui_child_names("main.swap.bottom");
            let swap_kids = ctx.ui_child_names("main.swap");
            let f0 = rows.iter().find(|r| r.starts_with("swap_slot")).map(|r| format!("{}.{}", table, r));
            let name0 = f0.as_ref().map(|f| (ctx.ui_child_names(&format!("{}.name", f)), ctx.ui_text(&format!("{}.name", f)), ctx.ui_text(&format!("{}.name.text", f))));
            let pos0 = f0.as_ref().map(|f| (ctx.ui_child_names(&format!("{}.main_position", f)), ctx.ui_text(&format!("{}.main_position", f)), ctx.ui_text(&format!("{}.main_position.text", f))));
            let kinds: Vec<String> = f0.as_ref().map(|f| ["", ".name", ".main_position", ".name.text", ".main_position.text"].iter().map(|sfx| format!("{}: kind={:?} state={:?} text={:?} rect={:?} exists={} kids={:?}", sfx, ctx.ui_runner_name(&format!("{}{}", f, sfx)), ctx.ui_state_json(&format!("{}{}", f, sfx)).map(|j| j.chars().take(200).collect::<String>()), ctx.ui_text(&format!("{}{}", f, sfx)), ctx.ui_node_rect(&format!("{}{}", f, sfx)), ctx.ui_exists(&format!("{}{}", f, sfx)), ctx.ui_child_count(&format!("{}{}", f, sfx)))).collect()).unwrap_or_default();
            config::llog(&format!("swapdiag: table={} rows={:?} swap_kids={:?} bottom={:?} first_kids={:?} first_data={:?} name0={:?} pos0={:?} pos_texts={:?} slot_champ={:?} | kinds={:?}", table, rows, swap_kids, bottom, first_kids, first_data, name0, pos0, pos_texts, slot_champ, kinds));
        }
        // ★0.6.0(09-18 RE): 1순위 = 씬 raw(order[포지션] = 픽 인덱스 · 유저 클릭·코치 위임·상대 AI 전부 반영) / 폴백 = 행 k = 포지션 k, 챔피언 = y 순 k 번째 픽 슬롯(swap_track 클릭 추적).
        let raw = crate::draft_scene::read().filter(|st| st.is_swap());
        let lineup: Vec<Option<String>> = match &raw {
            Some(st) => st.lineup(my_side as usize),
            None => {
                let order = slot_order(ctx, my_side == 0);
                let st_slots: HashMap<(bool, usize), String> = PICK_STATE.lock().unwrap_or_else(|e| e.into_inner()).as_ref().map(|st| st.slot_champ.clone()).unwrap_or_default();
                let nrows = rows.iter().filter(|r| r.starts_with("swap_slot_")).count().min(5);
                (0..nrows).map(|p| order.get(p).and_then(|k| st_slots.get(&(my_side == 0, *k)).cloned())).collect()
            }
        };
        if RAW_SRC.swap(raw.is_some(), Ordering::Relaxed) != raw.is_some() { config::llog(&format!("swapgate: 소스 = {}", if raw.is_some() { "씬 raw(order)" } else { "클릭 추적 폴백" })); }
        if let Some(st) = &raw { if f_once_swap() { config::llog(&format!("swapraw: phase={} rule={} sent={} t1={:#x} t2={:#x} pick1={:?} pick2={:?} orderA={:?} orderB={:?} my_side={}", st.phase, st.rule, st.sent, st.t1_id, st.t2_id, st.pick1, st.pick2, st.order_a, st.order_b, my_side)); } }
        for (p, c) in lineup.iter().enumerate().take(5) {
            let Some(champ) = c else { continue };
            resolved += 1;
            let m = crate::mask_of(champ);
            if m != MASK_ALL && m & (1 << p) == 0 { violations.push(format!("{}→{}", crate::disp_name(champ), i18n::pos_name(p))); }
        }
        let _ = (&slot_champ, &pos_texts);
    }
    let bad = resolved == 5 && !violations.is_empty();
    let sig = { use std::hash::{Hash, Hasher}; let mut h = std::collections::hash_map::DefaultHasher::new(); (resolved, &violations, bad).hash(&mut h); h.finish() };
    if SWAP_SIG.swap(sig, Ordering::Relaxed) != sig { config::llog(&format!("swapgate: resolved={} bad={} {:?}", resolved, bad, violations)); }
    // ★09-18: `disable`/`disabled` 속성 set 은 게임 버튼에 아무 효과 없음(실측 2회) → primary_button 의 disabled 스타일 색(btn #20342da6 · text #a7b8b3a6)을 직접 칠한다(클릭은 detour 가 삼킴).
    {
        let want = bad as u8;
        if CONFIRM_PAINT.swap(want, Ordering::Relaxed) != want {
            let css = if bad { "btn: { color: #20342da6; } text: { color: #a7b8b3a6; } hover: { btn: { color: #20342da6; } text: { color: #a7b8b3a6; } }" } else { "btn: { color: #124f43ff; } text: { color: #f4fffcff; } hover: { btn: { color: #176f5dff; } text: { color: #f4fffcff; } }" };
            ctx.ui_set_properties(confirm, css);
        }
    }
    // ★09-18: 게임 버튼은 disable 을 안 본다(실측: 클릭 통과) → 확정 핸들러 detour 가 클릭을 삼킨다.
    crate::swap_confirm_hook::BLOCK.store(bad, Ordering::Relaxed);
    let tp = "main.pl_swaptip";
    if bad {
        if !ctx.ui_exists(tp) { let src = SWAPTIP_UI.replacen("pos_lock_swaptip:color {", "pl_swaptip:color {", 1); if !ctx.ui_spawn_source(ROOT, &src) { return; } }
        if let Some((x, y, w, _h)) = ctx.ui_node_rect(confirm) { uk::set_props_if_changed(ctx, tp, "x", &format!("{}px", x + w / 2.0 - 120.0)); uk::set_props_if_changed(ctx, tp, "y", &format!("{}px", y - 40.0)); }
        uk::set_props_if_changed(ctx, tp, "visible", "true");
    } else if ctx.ui_exists(tp) { uk::set_props_if_changed(ctx, tp, "visible", "false"); }
}
