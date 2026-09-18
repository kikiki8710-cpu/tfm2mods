//! ui_popup — 환경설정 게임플레이 탭 '포지션 제한' 행 + 설정 팝업 (stable: 스폰 API + 경로 기반 조작).
//! · 행 = `<contents>.pos_lock_row`(옵션 화면 `contents` 끝에 스폰) — 게임플레이 탭 행(`always_delegate_to_staff`) 가시성을 따라감.
//! · 팝업 = 옵션 화면 루트(contents 의 조부모)에 `pos_lock_popup` 스폰(z 2000). 셀 120개 = 클래식 .ui 그대로.
//! · 아이콘 = `ui_set_champion_icon`(공식 렌더 — 클래식 icon_data.rs 번들 UV 로더 불요).
//! · 클래스 필터 = 드롭다운 대신 선택탭 6개(stable 엔 드롭다운 옵션 주입 API 없음).
//! · 클릭 = 경로에 1회 등록(영구) — 팝업이 재스폰돼도 같은 경로면 유지.
use crate::{config, i18n, uk};
use mod_api_stable::StableClient;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;

const ROW_UI: &str = include_str!("../assets/pos_lock_row.ui");
const POPUP_UI: &str = include_str!("../assets/pos_lock_popup.ui");
pub const NCELLS: usize = 120;
const TAB_IDS: [&str; 5] = ["tab_top", "tab_jungle", "tab_mid", "tab_bottom", "tab_support"];
const CLASS_IDS: [&str; 6] = ["class_all", "class_melee", "class_range", "class_magician", "class_util", "class_assassin"];
const GAMEPLAY_ROW: &str = "difficulty"; // ★0.6.0: 게임플레이 탭에서 always_delegate_to_staff 행이 사라짐(09-17 실측) → 같은 탭의 난이도 행을 가시성 앵커로

static POPUP_OPEN: AtomicBool = AtomicBool::new(false);
static DD_OPEN: AtomicBool = AtomicBool::new(false);
static TAB_PAINT: Mutex<Option<std::collections::HashMap<String, bool>>> = Mutex::new(None);
/// 탭 버튼 선택색 직접 칠하기(값이 바뀔 때만 set_properties).
fn paint_tab(ctx: &mut StableClient<'_>, path: &str, on: bool) {
    { let mut g = TAB_PAINT.lock().unwrap_or_else(|e| e.into_inner()); let m = g.get_or_insert_with(std::collections::HashMap::new); if m.get(path) == Some(&on) { return; } m.insert(path.to_string(), on); }
    let css = if on { "btn: { color: #ecfbf8ff; back_color: #ecfbf8ff; } text: { color: #161721ff; } hover: { btn: { color: #ffffffff; back_color: #ffffffff; } text: { color: #161721ff; } }" } else { "btn: { color: #6f7788ff; back_color: #20232dff; } text: { color: #d7dbe4ff; } hover: { btn: { color: #dfe5efff; back_color: #2a2f3cff; } text: { color: #ffffffff; } }" };
    ctx.ui_set_properties(path, css);
}
static ROW_DIAG: AtomicBool = AtomicBool::new(false);
static RESPAWN_TRIES: AtomicUsize = AtomicUsize::new(0);
static SEL_POS: AtomicUsize = AtomicUsize::new(0);
static CLASS_SEL: AtomicUsize = AtomicUsize::new(0);
static SEARCH_CLEAR: AtomicBool = AtomicBool::new(false);
static GRID_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
static SEARCH_TXT: Mutex<String> = Mutex::new(String::new());
static VISIBLE: Mutex<Vec<String>> = Mutex::new(Vec::new());
static REGISTERED: Mutex<Option<HashSet<String>>> = Mutex::new(None);
static POPUP_PATH: Mutex<Option<String>> = Mutex::new(None);
pub static CNT_ROW_CLICK: AtomicU64 = AtomicU64::new(0);

/// 클릭 등록 — ★09-18: 스폰마다 재등록(재로드로 등록이 사라지는 경우 대응) + 같은 프레임 중복 발화 무시.
fn reg(ctx: &mut StableClient<'_>, path: &str, f: impl Fn() + Send + Sync + 'static) {
    let key = path.to_string();
    let ok = ctx.ui_register_click(path, "", move |_| {
        let fr = crate::FRAME.load(Ordering::Relaxed);
        { let mut g = FIRED.lock().unwrap_or_else(|e| e.into_inner()); let m = g.get_or_insert_with(std::collections::HashMap::new); if m.get(&key) == Some(&fr) { return; } m.insert(key.clone(), fr); }
        f()
    });
    if !ok { config::dlog(&format!("클릭 등록 실패 {}", path)); }
}
static FIRED: Mutex<Option<std::collections::HashMap<String, u64>>> = Mutex::new(None);

pub fn hidden() { POPUP_OPEN.store(false, Ordering::Relaxed); DD_OPEN.store(false, Ordering::Relaxed); }
/// 세이브 밖으로 나가면 클릭 등록이 사라진다(09-18 실측) → 등록 기억·경로 캐시 초기화.
pub fn reset_registrations() { *REGISTERED.lock().unwrap_or_else(|e| e.into_inner()) = None; *POPUP_PATH.lock().unwrap_or_else(|e| e.into_inner()) = None; }
pub fn is_open() -> bool { POPUP_OPEN.load(Ordering::Relaxed) }

/// 매 프레임(옵션 화면 contents 경로가 있을 때).
pub fn tick(ctx: &mut StableClient<'_>, contents: &str) {
    // ── 행 스폰/가시성
    let row = format!("{}.pos_lock_row", contents);
    // ★09-17(유저 제보): 설정창이 열린 채 설정을 바꾸면 contents 재구성으로 행이 빠짐 → 60프레임마다 자식 목록 대조, 없으면 재스폰.
    let f = crate::FRAME.load(Ordering::Relaxed);
    if f % 60 == 0 && ctx.ui_exists(&row) && !ctx.ui_child_names(contents).iter().any(|c| c == "pos_lock_row") {
        config::dlog("행이 contents 에서 떨어짐(옵션 재구성) — 재스폰");
        ctx.ui_remove_node(&row);
    }
    if !ctx.ui_exists(&row) {
        let ok = ctx.ui_spawn_source(contents, ROW_UI);
        config::dlog(&format!("행 스폰 {} ok={}", row, ok));
        if !ok { return; }
        // ★09-17 근본원인(유저 제보 "다시 열면 없음"): uk 속성 캐시(경로|키)에 이전 인스턴스의 visible=true 가 남아
        //   새 행(visible:false)에 set 을 건너뜀 → 스폰마다 캐시 초기화.
        uk::index_clear();
        let btn = format!("{}.pos_lock_configure", row);
        reg(ctx, &btn, || {
            CNT_ROW_CLICK.fetch_add(1, Ordering::Relaxed);
            POPUP_OPEN.store(true, Ordering::Relaxed);
            GRID_SIG.store(u64::MAX, Ordering::Relaxed);
            i18n::poll_now();
            crate::ROSTER_DIRTY.store(true, Ordering::Relaxed);
            config::dlog("포지션 제한 버튼 클릭됨");
        });
    }
    let gp_vis = ctx.ui_visible(&format!("{}.{}", contents, GAMEPLAY_ROW)).unwrap_or(false);
    // 가시성은 실제 노드 상태를 읽어 다르면 직접 set(캐시 무관 — 새 인스턴스에도 확실히 적용).
    if ctx.ui_visible(&row) != Some(gp_vis) { ctx.ui_set_visible(&row, gp_vis); }
    // ★09-17 진단+자가복구(유저 제보 "설정창 다시 열면 행 없음"): 120프레임마다 rect 로그, 앵커 배치됐는데 행 rect 0 이면 재스폰(최대 3회).
    if f % 120 == 0 {
        let rr = ctx.ui_node_rect(&row); let ar = ctx.ui_node_rect(&format!("{}.{}", contents, GAMEPLAY_ROW));
        config::dlog(&format!("행 진단: gp_vis={} row_vis={:?} row_rect={:?} anchor_rect={:?} kids_tail={:?}", gp_vis, ctx.ui_visible(&row), rr, ar, ctx.ui_child_names(contents).iter().rev().take(4).collect::<Vec<_>>()));
        let anchor_laid = ar.map(|r| r.2 > 0.0).unwrap_or(false);
        let row_zero = rr.map(|r| r.2 == 0.0 && r.3 == 0.0).unwrap_or(true);
        if gp_vis && anchor_laid && row_zero && RESPAWN_TRIES.fetch_add(1, Ordering::Relaxed) < 3 {
            config::dlog("행 미배치 — 제거 후 재스폰");
            ctx.ui_remove_node(&row);
            return;
        }
    }
    // ★0.6.0 진단(1회): 앵커 행 rect / 스폰된 행 rect — 겹침·배치 확인용
    if gp_vis && !ROW_DIAG.swap(true, Ordering::Relaxed) {
        let a = ctx.ui_node_rect(&format!("{}.{}", contents, GAMEPLAY_ROW)); let r = ctx.ui_node_rect(&row); let d = ctx.ui_node_rect(&format!("{}.current_database_edit", contents));
        config::dlog(&format!("행 배치 진단: anchor({})={:?} row={:?} dbedit={:?} contents_kids={:?}", GAMEPLAY_ROW, a, r, d, ctx.ui_child_names(contents)));
    }
    // ── 팝업 스폰(옵션 루트 = contents 의 조부모)
    let root = match contents.rfind('.') { Some(i) => { let p = &contents[..i]; match p.rfind('.') { Some(j) => p[..j].to_string(), None => p.to_string() } } None => String::new() };
    let pop = if root.is_empty() { "pos_lock_popup".to_string() } else { format!("{}.pos_lock_popup", root) };
    if !ctx.ui_exists(&pop) {
        if !POPUP_OPEN.load(Ordering::Relaxed) { return; } // 열 때만 스폰(시작 로딩 경합 회피)
        let ok = ctx.ui_spawn_source(&root, POPUP_UI);
        config::dlog(&format!("팝업 스폰 {} ok={}", pop, ok));
        if !ok || !ctx.ui_exists(&pop) { POPUP_OPEN.store(false, Ordering::Relaxed); return; }
        uk::index_clear();
        GRID_SIG.store(u64::MAX, Ordering::Relaxed);
        *TAB_PAINT.lock().unwrap_or_else(|e| e.into_inner()) = None; DD_OPEN.store(false, Ordering::Relaxed);
        register_popup_clicks(ctx, &pop);
    }
    *POPUP_PATH.lock().unwrap_or_else(|e| e.into_inner()) = Some(pop.clone());
    let open = POPUP_OPEN.load(Ordering::Relaxed);
    uk::set_props_if_changed(ctx, &pop, "visible", if open { "true" } else { "false" });
    if !open { return; }
    // ── 필터 위젯
    if SEARCH_CLEAR.swap(false, Ordering::Relaxed) { ctx.ui_set_text_edit_text(&format!("{}.filter_bar.champ_search", pop), ""); }
    let cur = ctx.ui_text_edit_text(&format!("{}.filter_bar.champ_search", pop)).unwrap_or_default().trim().to_lowercase();
    { let mut g = SEARCH_TXT.lock().unwrap_or_else(|e| e.into_inner()); if *g != cur { *g = cur; GRID_SIG.store(u64::MAX, Ordering::Relaxed); } }
    fill_grid(ctx, &pop);
}

fn register_popup_clicks(ctx: &mut StableClient<'_>, pop: &str) {
    let close = || POPUP_OPEN.store(false, Ordering::Relaxed);
    reg(ctx, &format!("{}.close", pop), close);
    reg(ctx, &format!("{}.cancel", pop), close);
    reg(ctx, &format!("{}.ok", pop), || {
        let body = config::state_text(true);
        *crate::PENDING_SAVE.lock().unwrap_or_else(|e| e.into_inner()) = Some(body.clone());
        POPUP_OPEN.store(false, Ordering::Relaxed);
        config::slog(&format!("확인: {}B 기록 대기 (이 세이브)", body.len()));
    });
    for (i, t) in TAB_IDS.iter().enumerate() { reg(ctx, &format!("{}.pos_tabs.{}", pop, t), move || { SEL_POS.store(i, Ordering::Relaxed); GRID_SIG.store(u64::MAX, Ordering::Relaxed); }); }
    // ★09-18: 클래스 필터 = 풀다운(버튼 class_dd + 목록 패널 class_list, 루트 마지막 자식) — stable 엔 드롭다운 항목 주입이 없다.
    reg(ctx, &format!("{}.filter_bar.class_dd", pop), || { DD_OPEN.fetch_xor(true, Ordering::Relaxed); GRID_SIG.store(u64::MAX, Ordering::Relaxed); });
    for (i, t) in CLASS_IDS.iter().enumerate() { reg(ctx, &format!("{}.class_list.{}", pop, t), move || { CLASS_SEL.store(i, Ordering::Relaxed); DD_OPEN.store(false, Ordering::Relaxed); GRID_SIG.store(u64::MAX, Ordering::Relaxed); }); }
    reg(ctx, &format!("{}.filter_bar.search_clear", pop), || SEARCH_CLEAR.store(true, Ordering::Relaxed));
    reg(ctx, &format!("{}.right.clear_pos", pop), || config::clear_pos(SEL_POS.load(Ordering::Relaxed)));
    reg(ctx, &format!("{}.right.select_all_pos", pop), || { if let Some(r) = crate::roster() { config::set_pos(SEL_POS.load(Ordering::Relaxed), r.ids.clone()); } });
    for k in 0..NCELLS {
        reg(ctx, &format!("{}.left.scroll.contents.cell{}", pop, k), move || {
            let v = VISIBLE.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(c) = v.get(k) { config::toggle(SEL_POS.load(Ordering::Relaxed), c); }
        });
    }
}

fn set_label(ctx: &mut StableClient<'_>, path: &str, s: &str) { if ctx.ui_text(path).as_deref() != Some(s) { ctx.ui_set_text(path, s); } }

fn fill_grid(ctx: &mut StableClient<'_>, pop: &str) {
    let pos = SEL_POS.load(Ordering::Relaxed);
    let ver = config::state_version();
    let Some(r) = crate::roster() else { return };
    let class_sel = CLASS_SEL.load(Ordering::Relaxed);
    let search = SEARCH_TXT.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let champs: Vec<String> = r.sorted.iter().filter(|id| {
        if class_sel > 0 { if r.cats.get(*id).copied() != Some((class_sel - 1) as u8) { return false; } }
        if !search.is_empty() { let n = r.names.get(*id).cloned().unwrap_or_default().to_lowercase(); if !n.contains(&search) && !id.contains(&search) { return false; } }
        true
    }).cloned().collect();
    let (style, ban_opt) = config::cur_rule();
    let ban_count = ban_opt.unwrap_or(0);
    let sig = {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        (pos, ver, &champs, class_sel, &search, style, ban_opt, r.sig, i18n::current_lang(), DD_OPEN.load(Ordering::Relaxed)).hash(&mut h);
        h.finish()
    };
    if GRID_SIG.swap(sig, Ordering::Relaxed) == sig { return; }
    *VISIBLE.lock().unwrap_or_else(|e| e.into_inner()) = champs.clone();
    // ★09-18: color_selectable 의 selected 는 0.6.0 stable 로 못 쓴다(state json = {}) → 버튼 탭에 색을 직접 칠한다.
    for (i, t) in TAB_IDS.iter().enumerate() { paint_tab(ctx, &format!("{}.pos_tabs.{}", pop, t), i == pos); }
    let dd_open = DD_OPEN.load(Ordering::Relaxed);
    let lst = format!("{}.class_list", pop);
    if ctx.ui_visible(&lst) != Some(dd_open) { ctx.ui_set_visible(&lst, dd_open); }
    for (i, t) in CLASS_IDS.iter().enumerate() { paint_tab(ctx, &format!("{}.{}", lst, t), i == class_sel); }
    set_label(ctx, &format!("{}.filter_bar.class_dd.label", pop), &format!("#asset/base/text/ui?pos_lock.{}", CLASS_IDS[class_sel]));
    // ── 우측 요약 라벨(클래식 fill_grid 그대로)
    let cnt = config::pos_count(pos);
    let (_pool1, base_need, worst_bits, worst_have, worst_need) = config::pos_safety(pos);
    let comp_size = worst_bits.count_ones() as usize;
    let worst_label: String = (0..5).filter(|q| worst_bits & (1 << q) != 0).map(i18n::pos_name).collect::<Vec<_>>().join("/");
    let right = format!("{}.right", pop);
    set_label(ctx, &format!("{}.summary", right), &i18n::trf("summary_fmt", &[("pos", &i18n::pos_name(pos))]));
    let rule_name = i18n::tr(match style { 2 => "rule_fearless_hard", 1 => "rule_fearless", _ => "rule_classic" });
    set_label(ctx, &format!("{}.rule_label", right), &i18n::trf("rule_label", &[("name", &rule_name)]));
    set_label(ctx, &format!("{}.ban_label", right), &if ban_opt.is_none() { i18n::tr("ban_reading") } else { i18n::trf("ban_label", &[("n", &ban_count.to_string())]) });
    let min_s = if ban_opt.is_none() { i18n::tr("min_unknown") }
        else if comp_size > 1 { i18n::trf("min_shared", &[("need", &base_need.to_string()), ("count", &comp_size.to_string()), ("have", &worst_have.to_string()), ("want", &worst_need.to_string())]) }
        else { i18n::trf("min_label", &[("need", &base_need.to_string())]) };
    set_label(ctx, &format!("{}.min_label", right), &min_s);
    let pool = config::pos_pool(pos);
    let count_s = if cnt == 0 { i18n::tr("count_zero") } else if pool > cnt { i18n::trf("count_label_pool", &[("n", &cnt.to_string()), ("pool", &pool.to_string())]) } else { i18n::trf("count_label", &[("n", &cnt.to_string())]) };
    set_label(ctx, &format!("{}.count_label", right), &count_s);
    {
        let live = config::pos_pool(pos);
        let active = config::pos_active_of(pos);
        let s = if ban_opt.is_none() { i18n::tr("warn_ban_unknown") }
            else if cnt == 0 { String::new() }
            else if !active {
                let stale = config::pos_stale(pos);
                let tail = if stale > 0 { i18n::trf("warn_min_tail_stale", &[("n", &stale.to_string())]) } else { String::new() };
                if worst_bits != (1u8 << pos) || live >= base_need {
                    i18n::trf("warn_subset", &[("lines", &worst_label), ("have", &worst_have.to_string()), ("need", &worst_need.to_string()), ("more", &worst_need.saturating_sub(worst_have).to_string())])
                } else {
                    i18n::trf("warn_min", &[("need", &base_need.to_string()), ("pool", &live.to_string()), ("more", &base_need.saturating_sub(live).to_string()), ("tail", &tail)])
                }
            } else { i18n::trf("status_active", &[("pool", &live.to_string()), ("need", &base_need.to_string())]) };
        let p = format!("{}.warning_min", right);
        set_label(ctx, &p, &s);
        let warn = !s.is_empty() && !active;
        uk::set_props_if_changed(ctx, &p, "color", if s.is_empty() || warn { "#ff4a4aff" } else { "#37d5b3ff" });
    }
    {
        let s = if ban_opt.is_none() { String::new() } else {
            let mut items: Vec<String> = Vec::new();
            for p in 0..5 {
                if p == pos || config::pos_count(p) == 0 || config::pos_active_of(p) { continue; }
                let (pool1, need1, wbits, whave, wneed) = config::pos_safety(p);
                items.push(if wbits == (1u8 << p) { format!("{} {}/{}", i18n::pos_name(p), pool1, need1) } else {
                    let lines: String = (0..5).filter(|q| wbits & (1 << q) != 0).map(i18n::pos_name).collect::<Vec<_>>().join("/");
                    format!("{}({} {}/{})", i18n::pos_name(p), lines, whave, wneed)
                });
            }
            if items.is_empty() { String::new() } else { i18n::trf("warn_others", &[("list", &items.join(" · "))]) }
        };
        set_label(ctx, &format!("{}.warning_others", right), &s);
    }
    // ── 셀
    let contents = format!("{}.left.scroll.contents", pop);
    let mut icon_ok = 0usize;
    for k in 0..NCELLS {
        let cell = format!("{}.cell{}", contents, k);
        match champs.get(k) {
            Some(id) => {
                let listed = config::is_listed(pos, id);
                set_label(ctx, &format!("{}.name", cell), &format!("#asset/base/text/champion?description.{}.name", id));
                uk::set_props_if_changed(ctx, &format!("{}.sel", cell), "visible", if listed { "true" } else { "false" });
                if ctx.ui_set_champion_icon(&format!("{}.icon", cell), id, 84.0, 84.0, 2.0) { icon_ok += 1; }
                uk::set_props_if_changed(ctx, &cell, "visible", "true");
            }
            None => { uk::set_props_if_changed(ctx, &cell, "visible", "false"); }
        }
    }
    let n = champs.len().min(NCELLS);
    let rows = n.div_ceil(7);
    let h = (rows as f32) * (171.0 + 15.0) + 16.0;
    uk::set_props_if_changed(ctx, &contents, "height", &format!("{}px", h));
    let total = r.sorted.len();
    set_label(ctx, &format!("{}.filter_bar.filter_count", pop), &if n == total { String::new() } else { format!("{}/{}", n, total) });
    config::dlog(&format!("grid fill: pos={} n={} icons={} h={}", pos, n, icon_ok, h));
}
