//! ui — 환경설정 게임플레이 탭 '추가 챔피언 설정' 행 + 미출시 챔피언 그리드 팝업 (stable 스폰 API, 경로 기반 조작).
//! · 옵션 contents 경로 = `pause_ui.option.option.contents`(0.6.0 메인 씬 실측 09-17) / 타이틀 `body.option.option.contents`.
//! · 행 = `<contents>.champ_excl_row` — 게임플레이 탭 행(`difficulty`) 가시성을 따라감.
//! · 팝업 = 옵션 루트(contents 의 조부모)에 `champ_excl_popup` 스폰(★z 속성 없음 — 0.6.0 은 z 가 자식 렌더를 죽임). 셀 120.
//! · 클릭 = 경로에 1회 등록(영구). 클릭 콜백엔 ctx 가 없어 상태만 바꾸고 다음 프레임 tick 이 반영.
use crate::{cands, dd, uk, I18N};
use mod_api_stable::StableClient;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;

const ROW_UI: &str = include_str!("../assets/champ_excl_row.ui");
const POPUP_UI: &str = include_str!("../assets/champ_excl_popup.ui");
pub const NCELLS: usize = 120;
const CLASS_IDS: [&str; 6] = ["class_all", "class_melee", "class_range", "class_magician", "class_util", "class_assassin"];
const GAMEPLAY_ROW: &str = "difficulty";

static OPT_PATH: Mutex<Option<String>> = Mutex::new(None);
static OPT_SCAN_AT: AtomicU64 = AtomicU64::new(0);
static OPT_FAIL_LOGGED: AtomicBool = AtomicBool::new(false);
static OPT_FAIL_SIG: AtomicU64 = AtomicU64::new(0);
static POPUP_OPEN: AtomicBool = AtomicBool::new(false);
static DD_OPEN: AtomicBool = AtomicBool::new(false);
static TAB_PAINT: Mutex<Option<std::collections::HashMap<String, bool>>> = Mutex::new(None);
fn paint_tab(ctx: &mut StableClient<'_>, path: &str, on: bool) {
    { let mut g = TAB_PAINT.lock().unwrap_or_else(|e| e.into_inner()); let m = g.get_or_insert_with(std::collections::HashMap::new); if m.get(path) == Some(&on) { return; } m.insert(path.to_string(), on); }
    ctx.ui_set_properties(path, if on { "btn: { color: #ecfbf8ff; back_color: #ecfbf8ff; } text: { color: #161721ff; } hover: { btn: { color: #ffffffff; back_color: #ffffffff; } text: { color: #161721ff; } }" } else { "btn: { color: #6f7788ff; back_color: #20232dff; } text: { color: #d7dbe4ff; } hover: { btn: { color: #dfe5efff; back_color: #2a2f3cff; } text: { color: #ffffffff; } }" });
}
static LOAD_SEL_REQ: AtomicBool = AtomicBool::new(false);
static GRID_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
static SEL_VER: AtomicU64 = AtomicU64::new(0);
static CLASS_SEL: AtomicUsize = AtomicUsize::new(0);
static SEARCH_CLEAR: AtomicBool = AtomicBool::new(false);
static SEARCH_TXT: Mutex<String> = Mutex::new(String::new());
static VISIBLE: Mutex<Vec<String>> = Mutex::new(Vec::new());
static SEL: Mutex<Option<HashSet<String>>> = Mutex::new(None);
static HAD_STAR: AtomicBool = AtomicBool::new(false);
static REGISTERED: Mutex<Option<HashSet<String>>> = Mutex::new(None);
static SAVE_REQ: AtomicBool = AtomicBool::new(false);
static RESPAWN_TRIES: AtomicUsize = AtomicUsize::new(0);

pub fn hidden() { POPUP_OPEN.store(false, Ordering::Relaxed); DD_OPEN.store(false, Ordering::Relaxed); }
/// 세이브 밖(타이틀)으로 나가면 UI 트리가 통째로 재생성되어 클릭 등록이 사라진다(09-18 실측: 재로드 후 버튼 무반응) → 등록 기억을 비워 재등록.
pub fn reset_registrations() { *REGISTERED.lock().unwrap_or_else(|e| e.into_inner()) = None; *OPT_PATH.lock().unwrap_or_else(|e| e.into_inner()) = None; }
pub fn grid_dirty() { GRID_SIG.store(u64::MAX, Ordering::Relaxed); }

/// 옵션 화면 contents 경로(캐시·120프레임 재탐색). ★가드를 별도 문장으로 먼저 해제(if-let 가드 수명 데드락 방지, 09-17 comptest 실사고).
pub fn option_contents(ctx: &StableClient<'_>, f: u64) -> Option<String> {
    let cached = OPT_PATH.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if let Some(p) = cached {
        if ctx.ui_exists(&p) { return Some(p); }
        *OPT_PATH.lock().unwrap_or_else(|e| e.into_inner()) = None;
    }
    if f.saturating_sub(OPT_SCAN_AT.load(Ordering::Relaxed)) < 120 && f > 120 { return None; }
    OPT_SCAN_AT.store(f, Ordering::Relaxed);
    let found = ["pause_ui.option.option.contents", "body.option.option.contents", "option.option.contents", "main.pause_ui.option.option.contents"].iter()
        .find(|p| ctx.ui_exists(&format!("{}.current_database_edit", p))).map(|p| p.to_string());
    if let Some(p) = &found { crate::log(&format!("option contents path = {}", p)); }
    else if ctx.ui_exists("pause_ui") || ctx.ui_exists("pause_ui.option") {
        // ★진단: pause 오버레이는 있는데 옵션 contents 경로가 후보에 없다 → 자식 이름 덤프(상태 변화 시 1회)
        let dump = format!("pause_ui={:?} option={:?} option.option={:?} scene={:?} client={:?}", ctx.ui_child_names("pause_ui"), ctx.ui_child_names("pause_ui.option"), ctx.ui_child_names("pause_ui.option.option"), ctx.scene_kind(), ctx.client_scene_kind());
        let h = { use std::hash::{Hash, Hasher}; let mut hs = std::collections::hash_map::DefaultHasher::new(); dump.hash(&mut hs); hs.finish() };
        if OPT_FAIL_SIG.swap(h, Ordering::Relaxed) != h { crate::log(&format!("option path NOT found while pause_ui exists: {}", dump)); }
    }
    *OPT_PATH.lock().unwrap_or_else(|e| e.into_inner()) = found.clone();
    found
}

/// 클릭 등록. ★09-18: 세이브 재로드(인게임 불러오기 포함)로 노드가 재생성되면 등록이 사라지는 경우가 있어
///   스폰할 때마다 다시 등록한다. 등록이 살아있는 경우 중복 발화가 되므로 같은 프레임 두 번째 발화는 무시(FIRED).
fn reg(ctx: &mut StableClient<'_>, path: &str, f: impl Fn() + Send + Sync + 'static) {
    let key = path.to_string();
    let ok = ctx.ui_register_click(path, "", move |_| {
        let fr = crate::FRAME.load(Ordering::Relaxed);
        { let mut g = FIRED.lock().unwrap_or_else(|e| e.into_inner()); let m = g.get_or_insert_with(std::collections::HashMap::new); if m.get(&key) == Some(&fr) { return; } m.insert(key.clone(), fr); }
        f()
    });
    if !ok { crate::log(&format!("click register failed {}", path)); }
}
static FIRED: Mutex<Option<std::collections::HashMap<String, u64>>> = Mutex::new(None);
fn set_label(ctx: &mut StableClient<'_>, path: &str, s: &str) { if ctx.ui_text(path).as_deref() != Some(s) { ctx.ui_set_text(path, s); } }

/// 매 프레임(옵션 contents 경로가 있을 때).
pub fn tick(ctx: &mut StableClient<'_>, contents: &str) {
    // ── 행
    let row = format!("{}.champ_excl_row", contents);
    // ★09-17(유저 제보): 설정창이 열린 채 설정을 바꾸면 게임이 contents 를 재구성해 스폰한 행이 빠진다(닫았다 열면 복구).
    //   → 60프레임마다 행이 contents 자식 목록에 실제로 있는지 확인, 없으면 잔존 노드 제거 후 재스폰.
    let f = crate::FRAME.load(Ordering::Relaxed);
    if f % 60 == 0 && ctx.ui_exists(&row) && !ctx.ui_child_names(contents).iter().any(|c| c == "champ_excl_row") {
        crate::log("row detached from contents (option rebuilt) - respawn");
        ctx.ui_remove_node(&row);
    }
    if !ctx.ui_exists(&row) {
        let ok = ctx.ui_spawn_source(contents, ROW_UI);
        crate::log(&format!("row spawn {} ok={}", row, ok));
        if !ok { return; }
        // ★09-17 근본원인(유저 제보 "다시 열면 없음"): uk 속성 캐시가 경로 기준이라 새로 스폰된 행(visible:false)에
        //   이전 인스턴스의 "visible=true" 가 남아 set 을 건너뛰었다 → 스폰마다 캐시 초기화.
        uk::index_clear();
        reg(ctx, &format!("{}.champ_excl_configure", row), || {
            POPUP_OPEN.store(true, Ordering::Relaxed);
            crate::CAND_FORCE.store(true, Ordering::Relaxed);
            LOAD_SEL_REQ.store(true, Ordering::Relaxed);
            GRID_SIG.store(u64::MAX, Ordering::Relaxed);
            crate::log("champion addition settings button clicked");
        });
    }
    let gp_vis = ctx.ui_visible(&format!("{}.{}", contents, GAMEPLAY_ROW)).unwrap_or(false);
    // 가시성은 실제 노드 상태를 읽어 다르면 직접 set(캐시 무관 — 새 인스턴스에도 확실히 적용).
    if ctx.ui_visible(&row) != Some(gp_vis) { ctx.ui_set_visible(&row, gp_vis); }
    // ★09-17 진단+자가복구(유저 제보 "설정창 다시 열면 행 없음"): 120프레임마다 행/앵커 rect·가시성 로그.
    //   앵커(difficulty) 는 배치됐는데 행 rect 가 0 이면 레이아웃 누락 → 행 제거 후 재스폰(최대 3회).
    if f % 120 == 0 {
        let rr = ctx.ui_node_rect(&row); let ar = ctx.ui_node_rect(&format!("{}.{}", contents, GAMEPLAY_ROW)); let dr = ctx.ui_node_rect(&format!("{}.current_database_edit", contents));
        let rv = ctx.ui_visible(&row);
        crate::log(&format!("row diag: gp_vis={} row_vis={:?} row_rect={:?} anchor_rect={:?} dbedit_rect={:?} kids_tail={:?}", gp_vis, rv, rr, ar, dr, ctx.ui_child_names(contents).iter().rev().take(4).collect::<Vec<_>>()));
        let anchor_laid = ar.map(|r| r.2 > 0.0).unwrap_or(false);
        let row_zero = rr.map(|r| r.2 == 0.0 && r.3 == 0.0).unwrap_or(true);
        if gp_vis && anchor_laid && row_zero && RESPAWN_TRIES.fetch_add(1, Ordering::Relaxed) < 3 {
            crate::log("row not laid out - remove & respawn");
            ctx.ui_remove_node(&row);
            return;
        }
    }
    // ── 팝업(옵션 루트 = contents 의 조부모)
    let root = match contents.rfind('.') { Some(i) => { let p = &contents[..i]; match p.rfind('.') { Some(j) => p[..j].to_string(), None => p.to_string() } } None => String::new() };
    let pop = if root.is_empty() { "champ_excl_popup".to_string() } else { format!("{}.champ_excl_popup", root) };
    if !ctx.ui_exists(&pop) {
        if !POPUP_OPEN.load(Ordering::Relaxed) { return; }
        let ok = ctx.ui_spawn_source(&root, &popup_source());
        crate::log(&format!("popup spawn {} ok={}", pop, ok));
        if !ok || !ctx.ui_exists(&pop) { POPUP_OPEN.store(false, Ordering::Relaxed); return; }
        uk::index_clear();
        GRID_SIG.store(u64::MAX, Ordering::Relaxed);
        *TAB_PAINT.lock().unwrap_or_else(|e| e.into_inner()) = None; DD_OPEN.store(false, Ordering::Relaxed);
        dd::reset_cache();
        register_popup_clicks(ctx, &pop);
    }
    let open = POPUP_OPEN.load(Ordering::Relaxed);
    uk::set_props_if_changed(ctx, &pop, "visible", if open { "true" } else { "false" });
    if !open { return; }
    if SAVE_REQ.swap(false, Ordering::Relaxed) { save_selection(); POPUP_OPEN.store(false, Ordering::Relaxed); uk::set_props_if_changed(ctx, &pop, "visible", "false"); return; }
    if SEARCH_CLEAR.swap(false, Ordering::Relaxed) { ctx.ui_set_text_edit_text(&format!("{}.filter_bar.champ_search", pop), ""); }
    let cur = ctx.ui_text_edit_text(&format!("{}.filter_bar.champ_search", pop)).unwrap_or_default().trim().to_lowercase();
    { let mut g = SEARCH_TXT.lock().unwrap_or_else(|e| e.into_inner()); if *g != cur { *g = cur; GRID_SIG.store(u64::MAX, Ordering::Relaxed); } }
    if LOAD_SEL_REQ.swap(false, Ordering::Relaxed) { load_selection(); }
    fill_grid(ctx, &pop);
}

fn register_popup_clicks(ctx: &mut StableClient<'_>, pop: &str) {
    let close = || POPUP_OPEN.store(false, Ordering::Relaxed);
    reg(ctx, &format!("{}.close", pop), close);
    reg(ctx, &format!("{}.cancel", pop), close);
    reg(ctx, &format!("{}.ok", pop), || SAVE_REQ.store(true, Ordering::Relaxed));
    reg(ctx, &format!("{}.right.sel_all", pop), || { if let Some(c) = cands() { *SEL.lock().unwrap_or_else(|e| e.into_inner()) = Some(c.ids.iter().cloned().collect()); SEL_VER.fetch_add(1, Ordering::Relaxed); } });
    reg(ctx, &format!("{}.right.sel_none", pop), || { *SEL.lock().unwrap_or_else(|e| e.into_inner()) = Some(HashSet::new()); SEL_VER.fetch_add(1, Ordering::Relaxed); });
    reg(ctx, &format!("{}.filter_bar.search_clear", pop), || SEARCH_CLEAR.store(true, Ordering::Relaxed));
    reg(ctx, &format!("{}.filter_bar.class_dd", pop), || { DD_OPEN.fetch_xor(true, Ordering::Relaxed); GRID_SIG.store(u64::MAX, Ordering::Relaxed); });
    for (i, t) in CLASS_IDS.iter().enumerate() { reg(ctx, &format!("{}.class_list.{}", pop, t), move || { CLASS_SEL.store(i, Ordering::Relaxed); DD_OPEN.store(false, Ordering::Relaxed); GRID_SIG.store(u64::MAX, Ordering::Relaxed); }); }
    for k in 0..NCELLS {
        reg(ctx, &format!("{}.left.scroll.contents.cell{}", pop, k), move || {
            // ★필터된 목록(VISIBLE)을 본다 — 그리드와 같은 출처(필터 중 엉뚱한 챔프 토글 방지).
            let id = VISIBLE.lock().unwrap_or_else(|e| e.into_inner()).get(k).cloned();
            if let Some(id) = id { toggle_sel(&id); }
        });
    }
}

fn toggle_sel(id: &str) {
    let mut g = SEL.lock().unwrap_or_else(|e| e.into_inner());
    let set = g.get_or_insert_with(HashSet::new);
    if !set.remove(id) { set.insert(id.to_string()); }
    SEL_VER.fetch_add(1, Ordering::Relaxed);
}

/// 현재 세이브 설정 → 선택 상태(팝업 열릴 때).
fn load_selection() {
    let (list, star, src) = crate::effective_exclusion();
    crate::log(&format!("selection loaded: {} (source={}{})", list.len(), src, if star { ",*" } else { "" }));
    HAD_STAR.store(star, Ordering::Relaxed);
    let cand: Vec<String> = cands().map(|c| c.ids.clone()).unwrap_or_default();
    let sel: HashSet<String> = if star { cand.iter().cloned().collect() } else { let cs: HashSet<&String> = cand.iter().collect(); list.into_iter().filter(|e| cs.contains(e)).collect() };
    *SEL.lock().unwrap_or_else(|e| e.into_inner()) = Some(sel);
    SEL_VER.fetch_add(1, Ordering::Relaxed);
}

/// 선택 상태 → 세이브 기록 본문(PENDING_SAVE 이월). 후보 밖 기존 항목 보존·'*' 는 원래 있었고 전부 선택일 때만 유지.
fn save_selection() {
    let cand: Vec<String> = cands().map(|c| c.ids.clone()).unwrap_or_default();
    let Some(sel) = SEL.lock().unwrap_or_else(|e| e.into_inner()).clone() else { return };
    let (prev, _, _) = crate::effective_exclusion();
    let cand_set: HashSet<&String> = cand.iter().collect();
    let mut foreign: Vec<String> = prev.into_iter().filter(|e| !cand_set.contains(e)).collect();
    foreign.sort(); foreign.dedup();
    let all_selected = !cand.is_empty() && sel.len() == cand.len();
    let keep_star = HAD_STAR.load(Ordering::Relaxed) && all_selected;
    let mut out = String::new();
    if keep_star { out.push_str("*\n"); } else { for c in &cand { if sel.contains(c) { out.push_str(c); out.push('\n'); } } }
    for f in &foreign { out.push_str(f); out.push('\n'); }
    crate::log(&format!("save requested: {} selected{}{}", sel.len(), if keep_star { " ('*' kept)" } else { "" }, if foreign.is_empty() { String::new() } else { format!(" + {} preserved", foreign.len()) }));
    *crate::PENDING_SAVE.lock().unwrap_or_else(|e| e.into_inner()) = Some(out);
}

/// 팝업 .ui 의 `<<CLASS_DD>>`/`<<CLASS_LIST>>` 를 게임 dropdown 규격 소스로 치환(단일 정본 = ui_kit\dropdown_stable).
fn popup_source() -> String {
    let items: Vec<(&str, String)> = CLASS_IDS.iter().map(|k| (*k, format!("{}{}", I18N, k))).collect();
    let items_ref: Vec<(&str, &str)> = items.iter().map(|(a, b)| (*a, b.as_str())).collect();
    POPUP_UI.replace("<<CLASS_DD>>", &dd::button_source("class_dd", 0, 0, 150, 40, &format!("{}class_all", I18N), 16))
        .replace("<<CLASS_LIST>>", &dd::list_source("class_list", 32, 81 + 40 + dd::LIST_GAP as i32, 150, 40, 16, &items_ref))
}

fn fill_grid(ctx: &mut StableClient<'_>, pop: &str) {
    let Some(c) = cands() else { return };
    let selected: HashSet<String> = SEL.lock().unwrap_or_else(|e| e.into_inner()).clone().unwrap_or_default();
    let class_sel = CLASS_SEL.load(Ordering::Relaxed);
    // ★09-18 v2(매프레임 — sig 조기반환 전): 게임 dropdown 규격 재현(공용 ui_kit\dropdown_stable) — 바깥 클릭 닫힘·hover·체크.
    let edge = dd::click_edge();
    dd::tick(ctx, &format!("{}.filter_bar.class_dd", pop), &format!("{}.class_list", pop), &CLASS_IDS, &DD_OPEN, class_sel, &format!("{}{}", I18N, CLASS_IDS[class_sel]), edge);
    let search = SEARCH_TXT.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let visible: Vec<String> = c.ids.iter().filter(|id| {
        if class_sel > 0 && c.cats.get(*id).copied() != Some((class_sel - 1) as u8) { return false; }
        if !search.is_empty() { let n = c.names.get(*id).cloned().unwrap_or_default().to_lowercase(); if !n.contains(&search) && !id.contains(&search) { return false; } }
        true
    }).cloned().collect();
    let sig = { use std::hash::{Hash, Hasher}; let mut h = std::collections::hash_map::DefaultHasher::new(); (SEL_VER.load(Ordering::Relaxed), c.sig, &visible, class_sel, &search, crate::has_save_setting(), DD_OPEN.load(Ordering::Relaxed)).hash(&mut h); h.finish() };
    if GRID_SIG.swap(sig, Ordering::Relaxed) == sig { return; }
    *VISIBLE.lock().unwrap_or_else(|e| e.into_inner()) = visible.clone();
    let right = format!("{}.right", pop);
    set_label(ctx, &format!("{}.cnt_total_v", right), &c.ids.len().to_string());
    set_label(ctx, &format!("{}.cnt_sel_v", right), &selected.len().to_string());
    let src = if !c.avail_ok { "src_avail_unknown" } else if crate::has_save_setting() { "src_save" } else { "src_none" };
    set_label(ctx, &format!("{}.src", right), &format!("{}{}", I18N, src));
    let note = if c.ids.is_empty() { format!("{}note_none", I18N) } else if selected.len() == c.ids.len() { format!("{}note_all", I18N) } else { String::new() };
    set_label(ctx, &format!("{}.note_all", right), &note);
    set_label(ctx, &format!("{}.filter_bar.filter_count", pop), &if visible.len() == c.ids.len() { String::new() } else { format!("{} / {}", visible.len(), c.ids.len()) });
    let contents = format!("{}.left.scroll.contents", pop);
    let mut icon_ok = 0usize;
    for k in 0..NCELLS {
        let cell = format!("{}.cell{}", contents, k);
        match visible.get(k) {
            Some(id) => {
                set_label(ctx, &format!("{}.name", cell), &format!("#asset/base/text/champion?description.{}.name", id));
                uk::set_props_if_changed(ctx, &format!("{}.sel", cell), "visible", if selected.contains(id) { "true" } else { "false" });
                if ctx.ui_set_champion_icon(&format!("{}.icon", cell), id, 84.0, 84.0, 2.0) { icon_ok += 1; }
                uk::set_props_if_changed(ctx, &cell, "visible", "true");
            }
            None => { uk::set_props_if_changed(ctx, &cell, "visible", "false"); }
        }
    }
    let n = visible.len().min(NCELLS);
    let rows = n.div_ceil(7);
    let h = (rows as f32) * (171.0 + 15.0) + 16.0;
    uk::set_props_if_changed(ctx, &contents, "height", &format!("{}px", h));
    crate::log(&format!("grid fill: n={} icons={} sel={} class={} search='{}'", n, icon_ok, selected.len(), class_sel, search));
}
