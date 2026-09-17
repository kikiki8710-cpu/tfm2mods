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
const GAMEPLAY_ROW: &str = "always_delegate_to_staff";

static POPUP_OPEN: AtomicBool = AtomicBool::new(false);
static SEL_POS: AtomicUsize = AtomicUsize::new(0);
static CLASS_SEL: AtomicUsize = AtomicUsize::new(0);
static SEARCH_CLEAR: AtomicBool = AtomicBool::new(false);
static GRID_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
static SEARCH_TXT: Mutex<String> = Mutex::new(String::new());
static VISIBLE: Mutex<Vec<String>> = Mutex::new(Vec::new());
static REGISTERED: Mutex<Option<HashSet<String>>> = Mutex::new(None);
static POPUP_PATH: Mutex<Option<String>> = Mutex::new(None);
pub static CNT_ROW_CLICK: AtomicU64 = AtomicU64::new(0);

fn reg(ctx: &mut StableClient<'_>, path: &str, f: impl Fn() + Send + Sync + 'static) {
    { let mut g = REGISTERED.lock().unwrap_or_else(|e| e.into_inner()); let s = g.get_or_insert_with(HashSet::new); if s.contains(path) { return; } s.insert(path.to_string()); }
    if !ctx.ui_register_click(path, "", move |_| f()) { config::dlog(&format!("클릭 등록 실패 {}", path)); }
}

pub fn hidden() { POPUP_OPEN.store(false, Ordering::Relaxed); }
pub fn is_open() -> bool { POPUP_OPEN.load(Ordering::Relaxed) }

/// 매 프레임(옵션 화면 contents 경로가 있을 때).
pub fn tick(ctx: &mut StableClient<'_>, contents: &str) {
    // ── 행 스폰/가시성
    let row = format!("{}.pos_lock_row", contents);
    if !ctx.ui_exists(&row) {
        let ok = ctx.ui_spawn_source(contents, ROW_UI);
        config::dlog(&format!("행 스폰 {} ok={}", row, ok));
        if !ok { return; }
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
    uk::set_props_if_changed(ctx, &row, "visible", if gp_vis { "true" } else { "false" });
    // ── 팝업 스폰(옵션 루트 = contents 의 조부모)
    let root = match contents.rfind('.') { Some(i) => { let p = &contents[..i]; match p.rfind('.') { Some(j) => p[..j].to_string(), None => p.to_string() } } None => String::new() };
    let pop = if root.is_empty() { "pos_lock_popup".to_string() } else { format!("{}.pos_lock_popup", root) };
    if !ctx.ui_exists(&pop) {
        if !POPUP_OPEN.load(Ordering::Relaxed) { return; } // 열 때만 스폰(시작 로딩 경합 회피)
        let ok = ctx.ui_spawn_source(&root, POPUP_UI);
        config::dlog(&format!("팝업 스폰 {} ok={}", pop, ok));
        if !ok || !ctx.ui_exists(&pop) { POPUP_OPEN.store(false, Ordering::Relaxed); return; }
        GRID_SIG.store(u64::MAX, Ordering::Relaxed);
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
    for (i, t) in CLASS_IDS.iter().enumerate() { reg(ctx, &format!("{}.filter_bar.class_tabs.{}", pop, t), move || { CLASS_SEL.store(i, Ordering::Relaxed); GRID_SIG.store(u64::MAX, Ordering::Relaxed); }); }
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
        (pos, ver, &champs, class_sel, &search, style, ban_opt, r.sig, i18n::current_lang()).hash(&mut h);
        h.finish()
    };
    if GRID_SIG.swap(sig, Ordering::Relaxed) == sig { return; }
    *VISIBLE.lock().unwrap_or_else(|e| e.into_inner()) = champs.clone();
    for (i, t) in TAB_IDS.iter().enumerate() { ctx.ui_set_selectable_selected(&format!("{}.pos_tabs.{}", pop, t), i == pos); }
    for (i, t) in CLASS_IDS.iter().enumerate() { ctx.ui_set_selectable_selected(&format!("{}.filter_bar.class_tabs.{}", pop, t), i == class_sel); }
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
