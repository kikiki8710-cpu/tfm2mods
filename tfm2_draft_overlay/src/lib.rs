//! tfm2_draft_overlay — 밴픽(드래프트) 화면에 TFM2.gg 메타 분석을 표시하는 오버레이.
//! ───────────────────────────────────────────────────────────────────────────
//! ★0.6.0 (2026-09-16) stable ABI 재작성. 클래식 원본 = `src/lib_classic_058.rs.bak`(0.5.8 까지).
//!   왜: 0.6.0 부터 클래식 SDK 미배포 + 로더가 클래식 dll 거부. `mod_api::Node` 위에 짜인 ui_kit 을
//!   stable UI API(경로 기반) 위의 `ui_kit_stable.rs` 로 갈아탔다.
//!   달라진 것:
//!     · 팝업 주입 = 로더 훅(LOADER/PARSER/ALLOC RVA)이 아니라 `ui_spawn_source("body", POPUP_UI)`.
//!       ⟹ 하드코딩 RVA 0개. 게임 패치에도 재빌드 불요(ABI 계약만 유지되면).
//!     · 얼굴 = `ui_set_champion_icon`(게임 자체 아이콘 계산, 모드챔프 포함) — RT 스캔/UV 코드 제거.
//!     · 전신 이미지 = `source: "…/champions/<id>#sheet"; rect_tag: "idle_0"`(⬜인게임 확인).
//!     · raw 러너 쓰기(드래그 밴드·raise_to_top·툴팁 숨김·스크롤 리셋)는 stable 에 없어 **제거**(⬜동작 확인).
//!   데이터 = TFM2.gg 가 write 한 overlay_data.txt(미리 포맷된 행) → 고정슬롯 row0..row99 에 fill.
//!   (분석/포맷은 TFM2.gg, 표시는 여기.)
#![allow(dead_code)]
use mod_api_stable::{declare_stable_mod, LogLevel, StableClient, StableExtension, StableHost, StableMod};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

#[path = r"C:\tfm2mods\ui_kit\ui_kit_stable.rs"]
mod uk;
use uk::Rgba;

const MOD_ID: &str = "tfm2_draft_overlay";
const MAX_ROWS: usize = 100;
const NUM_TABS: usize = 5; // 메타통계/챔피언정보/메타해석/밴픽코치/모의밴픽
const BPP_H: f32 = 200.0;  // 모의밴픽 컨트롤 패널 높이(gen_popup.py BPP_H 와 일치)
const NGRID: usize = 72;
const GCOLS: usize = 12;
const GC_H: f32 = 52.0;
const POPUP_UI: &str = include_str!("../ui_inject/draft_popup.ui");
const ROOT_ID: &str = "draft_root";
const CHAMP_SEP: char = '\u{1f}';
const SCOPE_IDS: [&str; 3] = ["overall", "tournament", "solo"];
const DSLOT_KEYS: [&str; 4] = ["enemyBans", "myBans", "enemyPicks", "myPicks"];
const DBG: bool = false; // 배포 전 false. 첫 스폰·트리 덤프·씬 이름을 overlay_debug.txt 에 기록.

static LAST_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
static SEL_TAB: AtomicUsize = AtomicUsize::new(0);
static SEL_ROLE: AtomicUsize = AtomicUsize::new(0);
static SEL_SCOPE: AtomicUsize = AtomicUsize::new(0);
static SEL_DSLOT: AtomicUsize = AtomicUsize::new(0);
static SEL_CHAMPSUB: AtomicUsize = AtomicUsize::new(0);
static LAST_CHAMP_ID: Mutex<String> = Mutex::new(String::new());
static ROW_CHAMP: Mutex<Vec<String>> = Mutex::new(Vec::new());
static GRID_CHAMP: Mutex<Vec<String>> = Mutex::new(Vec::new());
static LAST_RENDER: AtomicU64 = AtomicU64::new(u64::MAX);
static LAST_NAV: AtomicU64 = AtomicU64::new(u64::MAX);
static SECTIONS: Mutex<Vec<(String, Vec<String>)>> = Mutex::new(Vec::new());
static IMG_HAS_CHAMP: AtomicBool = AtomicBool::new(false);
static SHOW_OVERLAY: AtomicBool = AtomicBool::new(true);
static CTRL_SEQ: AtomicUsize = AtomicUsize::new(1);
static ROUTES_DONE: AtomicBool = AtomicBool::new(false);
static SPAWN_LOGGED: AtomicBool = AtomicBool::new(false);
static BANPICK_ROOT: Mutex<Option<String>> = Mutex::new(None); // "body.<banpick 루트 id>" 캐시
static FRAME: AtomicU64 = AtomicU64::new(0);

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleFileNameW(module: usize, buf: *mut u16, size: u32) -> u32;
}

fn mods_dir() -> Option<String> {
    let mut buf = [0u16; 520];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n == 0 || n >= buf.len() { return None; }
    let exe = String::from_utf16_lossy(&buf[..n]);
    exe.rfind(|c| c == '\\' || c == '/').map(|i| format!(r"{}\mods\{}", &exe[..i], MOD_ID))
}
fn data_path() -> String { mods_dir().map(|d| format!(r"{}\overlay_data.txt", d)).unwrap_or_default() }
fn dbg_write(s: &str) {
    if !DBG { return; }
    if let Some(d) = mods_dir() {
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(format!(r"{}\overlay_debug.txt", d)) {
            let _ = writeln!(f, "[f{}] {}", FRAME.load(Ordering::Relaxed), s);
        }
    }
}
fn write_control(s: &str) {
    if let Some(d) = mods_dir() {
        let seq = CTRL_SEQ.fetch_add(1, Ordering::Relaxed);
        let _ = std::fs::write(format!(r"{}\overlay_control.txt", d), format!("{}\n_seq={}\n", s, seq));
    }
}
fn write_control_state() {
    let role = SEL_ROLE.load(Ordering::Relaxed);
    let scope = SCOPE_IDS[SEL_SCOPE.load(Ordering::Relaxed).min(2)];
    let champ = LAST_CHAMP_ID.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let mut s = format!("role={}\nscope={}", role, scope);
    if !champ.is_empty() { s.push_str(&format!("\nchamp={}", champ)); }
    write_control(&s);
}
fn parse_sections(txt: &str) -> Vec<(String, Vec<String>)> {
    let mut out: Vec<(String, Vec<String>)> = Vec::new();
    for line in txt.lines() {
        if let Some(name) = line.strip_prefix("#TAB ") {
            out.push((name.trim().to_string(), Vec::new()));
        } else if let Some(last) = out.last_mut() {
            last.1.push(line.to_string());
        }
    }
    out
}

// 티어 → 뱃지 배경색 (대시보드 styles.css 와 동일: OP=빨강, 그 외=파랑).
fn tier_color(t: &str) -> Rgba { if t == "OP" { Rgba::hex(0xef4b5fff) } else { Rgba::hex(0x5162bbff) } }
// 승률 라벨: 대시보드가 붙인 접두 마커(\x02=초록/\x03=빨강/\x04=흐릿)를 읽어 색 적용 후 마커 제거.
fn set_wr_label(ctx: &mut StableClient<'_>, id: &str, text: &str) {
    let (color, body) = match text.as_bytes().first() {
        Some(&2) => (Rgba::hex(0x5ac77aff), &text[1..]),
        Some(&3) => (Rgba::hex(0xef6b6bff), &text[1..]),
        Some(&4) => (Rgba::hex(0x6f7686ff), &text[1..]),
        _ => (Rgba::hex(0xe8e8e8ff), text),
    };
    uk::label_set(ctx, id, body);
    uk::label_set_color(ctx, id, color);
}
/// 전신 이미지: 시트 소스 + idle 첫 프레임 rect_tag. (⬜0.6.0 인게임 확인 — 실패 시 숨김)
fn set_champ_image(ctx: &mut StableClient<'_>, id: &str, champ_id: &str) -> bool {
    let src = format!("asset/base/aseprite_resources/champions/{}#sheet", champ_id);
    uk::image_set_source(ctx, id, &src, Some("idle_0"))
}

/// 밴픽 화면인가 = body 직속 자식 중 "banpick" 으로 시작하는 노드가 있는가(이름 캐시).
fn banpick_root(ctx: &StableClient<'_>) -> Option<String> {
    {
        let g = BANPICK_ROOT.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(p) = g.as_ref() { if ctx.ui_exists(p) { return Some(p.clone()); } }
    }
    for name in ctx.ui_child_names("body") {
        if name.starts_with("banpick") {
            let p = format!("body.{}", name);
            *BANPICK_ROOT.lock().unwrap_or_else(|e| e.into_inner()) = Some(p.clone());
            return Some(p);
        }
    }
    None
}

struct Ext;
impl StableExtension for Ext {
    fn post_update(&self, ctx: &mut StableClient<'_>, _dt: u64) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            FRAME.fetch_add(1, Ordering::Relaxed);
            uk::frame_begin();
            let root = format!("body.{}", ROOT_ID);
            let in_banpick = banpick_root(ctx).is_some();
            let have = ctx.ui_exists(&root);
            if !in_banpick {
                if have { uk::remove(ctx, &root); ROUTES_DONE.store(false, Ordering::Relaxed); dbg_write("밴픽 이탈 → 팝업 제거"); }
                return;
            }
            if !have {
                match uk::spawn_source(ctx, "body", ROOT_ID, POPUP_UI) {
                    Some(p) => {
                        ROUTES_DONE.store(false, Ordering::Relaxed);
                        LAST_RENDER.store(u64::MAX, Ordering::Relaxed);
                        if !SPAWN_LOGGED.swap(true, Ordering::Relaxed) {
                            let mut s = format!("팝업 스폰 OK {} (body 자식: {:?})\n", p, ctx.ui_child_names("body"));
                            uk::dump_tree(ctx, &p, 0, &mut s, 3);
                            dbg_write(&s);
                        }
                    }
                    None => { dbg_write("★팝업 스폰 실패(ui_spawn_source false — .ui 문법/호스트 ABI 확인)"); return; }
                }
            }
            frame_body(ctx);
        }));
    }
}

fn frame_body(ctx: &mut StableClient<'_>) {
    // 0) 표시/숨김
    let show = SHOW_OVERLAY.load(Ordering::Relaxed);
    uk::set_visible(ctx, "draft_overlay", show);

    // 1) 파일 변경 시 재파싱 → SECTIONS 캐시. 변경감지 = mtime ^ len.
    let sig = std::fs::metadata(data_path()).map(|m| {
        let t = m.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_nanos() as u64).unwrap_or(0);
        t ^ m.len().rotate_left(17)
    }).unwrap_or(0);
    if LAST_SIG.swap(sig, Ordering::Relaxed) != sig {
        let txt = std::fs::read_to_string(data_path()).unwrap_or_default();
        *SECTIONS.lock().unwrap_or_else(|e| e.into_inner()) = parse_sections(&txt);
    }

    // 2) 클릭 라우트(스폰 후 1회 등록 — 호스트 등록은 영구).
    if !ROUTES_DONE.load(Ordering::Relaxed) {
        let n = register_routes(ctx);
        dbg_write(&format!("클릭 라우트 등록 {}건", n));
        ROUTES_DONE.store(true, Ordering::Relaxed);
    }

    let sel = SEL_TAB.load(Ordering::Relaxed).min(NUM_TABS - 1);
    let role_sel = SEL_ROLE.load(Ordering::Relaxed);
    let csub_sel = SEL_CHAMPSUB.load(Ordering::Relaxed);
    let scope_sel = SEL_SCOPE.load(Ordering::Relaxed);
    let dslot_sel = SEL_DSLOT.load(Ordering::Relaxed);

    // 챔프 전신 이미지: 챔프탭 + 스크롤 최상단에서만 표시(루트 절대노드라 클리핑 불가).
    {
        let at_top = uk::scroll_ratio(ctx, "scroll").unwrap_or(0.0) <= 0.01;
        let show_img = sel == 1 && IMG_HAS_CHAMP.load(Ordering::Relaxed) && at_top;
        uk::set_visible(ctx, "champ_img", show_img);
    }

    // 3) 렌더(파일/탭/역할/서브탭 변경 시만). 첫 진입이면 강제 1회.
    let key = sig.wrapping_shl(12) ^ ((sel as u64) << 8) ^ ((role_sel as u64) << 4) ^ (csub_sel as u64) ^ ((scope_sel as u64) << 20) ^ ((dslot_sel as u64) << 24);
    if LAST_RENDER.swap(key, Ordering::Relaxed) == key { return; }

    // 새 뷰 진입 → (클래식: 스크롤 최상단) — stable 엔 스크롤 쓰기 API 가 없어 생략(⬜).
    {
        let champ_hash = LAST_CHAMP_ID.lock().unwrap_or_else(|e| e.into_inner()).bytes()
            .fold(0xcbf29ce484222325u64, |h, b| (h ^ b as u64).wrapping_mul(0x100000001b3));
        let nav = ((sel as u64) << 56) ^ ((role_sel as u64) << 48) ^ ((csub_sel as u64) << 40) ^ ((dslot_sel as u64) << 32) ^ champ_hash;
        let _ = LAST_NAV.swap(nav, Ordering::Relaxed);
    }

    let teal = Rgba::hex(0x37d5b3ff);
    let dim = Rgba::hex(0xa3a9b6ff);
    uk::set_visible(ctx, "rolebar", sel == 0);
    uk::set_visible(ctx, "csub", sel == 1);
    uk::set_visible(ctx, "dslot", sel == 3);
    uk::set_visible(ctx, "bppanel", sel == 4);
    let hl = |ctx: &mut StableClient<'_>, prefix: &str, n: usize, cur: usize| {
        for i in 0..n { uk::label_set_color(ctx, &format!("{}{}l", prefix, i), if i == cur { teal } else { dim }); }
    };
    hl(ctx, "tab", NUM_TABS, sel); hl(ctx, "role", 6, role_sel); hl(ctx, "csub", 4, csub_sel);
    hl(ctx, "scope", 3, scope_sel); hl(ctx, "dslot", 4, dslot_sel);

    let (title, mut lines): (String, Vec<String>) = {
        let secs = SECTIONS.lock().unwrap_or_else(|e| e.into_inner());
        if secs.is_empty() { ("데이터 대기 중… (TFM2.gg 를 켜세요)".to_string(), Vec::new()) }
        else { let idx = sel.min(secs.len() - 1); (format!("{}  ·  TFM2.gg 연동됨", secs[idx].0), secs[idx].1.clone()) }
    };

    // 챔피언정보 탭: #IMG / #SUB 분해.
    if sel == 1 {
        let mut champ_id = String::new();
        let mut header: Vec<String> = Vec::new();
        let mut subs: Vec<Vec<String>> = Vec::new();
        for ln in &lines {
            if let Some(id) = ln.strip_prefix("#IMG ") { champ_id = id.trim().to_string(); }
            else if ln.starts_with("#SUB ") { subs.push(Vec::new()); }
            else if subs.is_empty() { header.push(ln.clone()); }
            else if let Some(last) = subs.last_mut() { last.push(ln.clone()); }
        }
        if champ_id.is_empty() { IMG_HAS_CHAMP.store(false, Ordering::Relaxed); }
        else { let ok = set_champ_image(ctx, "champ_img", &champ_id); IMG_HAS_CHAMP.store(ok, Ordering::Relaxed); }
        let cs = csub_sel.min(subs.len().saturating_sub(1));
        let mut out = header;
        if let Some(rows) = subs.get(cs) { out.extend(rows.iter().cloned()); }
        lines = out;
    } else {
        IMG_HAS_CHAMP.store(false, Ordering::Relaxed);
    }

    // 모의밴픽(탭4) 상태줄.
    if sel == 4 {
        if let Some(pos) = lines.iter().position(|l| l.starts_with("#BPSTATE ")) {
            let st = lines.remove(pos);
            let body = st["#BPSTATE ".len()..].to_string();
            let getv = |key: &str| -> Option<String> {
                body.split_whitespace().find_map(|tok| tok.split_once('=').and_then(|(k, v)| if k == key { Some(v.to_string()) } else { None }))
            };
            let seti: usize = getv("set").and_then(|v| v.parse().ok()).unwrap_or(0);
            let rule = getv("rule").unwrap_or_else(|| "classic".into());
            let ruleidx = ["classic", "fearless", "hardFearless"].iter().position(|r| *r == rule).unwrap_or(0);
            let bans: u8 = getv("bans").and_then(|v| v.parse().ok()).unwrap_or(2);
            hl(ctx, "bpset", 5, seti); hl(ctx, "bprule", 3, ruleidx);
            for (i, n) in [(0usize, 2u8), (1usize, 3u8)] { uk::label_set_color(ctx, &format!("bpban{}l", i), if n == bans { teal } else { dim }); }
            uk::label_set(ctx, "bpsidel", if getv("side").as_deref() == Some("red") { "레드" } else { "블루" });
            uk::label_set(ctx, "bppatchl", if getv("patch").as_deref() == Some("1") { "패치보정 ON" } else { "패치보정 OFF" });
            for k in ["stat", "meta", "game", "solo", "syn", "ctr"] { uk::label_set(ctx, &format!("bpw_{}val", k), &getv(k).unwrap_or_else(|| "0".into())); }
        }
    }

    // 챔프 그리드(밴픽코치·모의밴픽).
    let grid_on = sel == 3 || sel == 4;
    let mut grid_h = 0.0f32;
    if grid_on {
        let entries: Vec<String> = match lines.iter().position(|l| l == "#GRID") {
            Some(p) => lines.split_off(p).into_iter().skip(1).collect(),
            None => Vec::new(),
        };
        let used = entries.len().min(NGRID);
        let rows_used = (used + GCOLS - 1) / GCOLS;
        grid_h = (rows_used as f32) * GC_H + 6.0;
        let mut gchamps = vec![String::new(); NGRID];
        for k in 0..NGRID {
            let (id, state): (String, u8) = match entries.get(k) {
                Some(e) => { let (i, s) = e.split_once('\t').unwrap_or((e.as_str(), "0")); (i.to_string(), s.trim().parse::<u8>().unwrap_or(0)) }
                None => (String::new(), 0),
            };
            gchamps[k] = id.clone();
            let cell = format!("gcell{}", k);
            uk::set_visible(ctx, &cell, !id.is_empty());
            uk::set_disabled(ctx, &cell, id.is_empty());
            let c = match state { 1 => 0x37d5b3aa, 2 => 0x00000088, _ => 0x00000000 };
            uk::rect_set_color(ctx, &format!("gbg{}", k), Rgba::hex(c));
            let img = format!("gimg{}", k);
            let ok = !id.is_empty() && uk::image_set_champion_face(ctx, &img, &id, 44.0, 44.0);
            uk::set_visible(ctx, &img, ok);
        }
        *GRID_CHAMP.lock().unwrap_or_else(|e| e.into_inner()) = gchamps;
    }
    uk::set_layout_size(ctx, "pgrid", 0.0, grid_h.max(1.0));
    uk::set_visible(ctx, "pgrid", grid_on);

    // 리스트 높이 + 스크롤바.
    {
        let visible_rows = lines.len().min(MAX_ROWS);
        let mut content_h = 34.0 + (visible_rows as f32) * 31.0;
        if grid_on { content_h += grid_h; }
        if sel == 4 { content_h += BPP_H; }
        uk::set_layout_size(ctx, "list", 0.0, content_h);
        const VP: f32 = 626.0;
        let ratio = uk::scroll_ratio(ctx, "scroll").unwrap_or(0.0).clamp(0.0, 1.0);
        let need = content_h > VP + 1.0;
        uk::set_visible(ctx, "sbar_track", need);
        if need {
            let thumb_h = (VP * VP / content_h).max(24.0).min(VP);
            let thumb_y = ratio * (VP - thumb_h);
            uk::set_layout_size(ctx, "sbar_thumb", 0.0, thumb_h);
            uk::set_layout_xy(ctx, "sbar_thumb", 0.0, thumb_y);
        }
    }

    uk::set_visible(ctx, "colhead", (sel == 0 || sel == 2) && !lines.is_empty());
    uk::label_set(ctx, "ch_pr", if sel == 2 { "픽밴" } else { "픽률" });
    uk::label_set(ctx, "ch_p5", if sel == 2 { "표본" } else { "" });

    // 행 채우기. 각 행 = "표시텍스트\x1f챔프id". 표시텍스트 = \t 구분 [c0, tier, sc, wr, pr, p5, mname].
    let mut row_champs: Vec<String> = vec![String::new(); MAX_ROWS];
    uk::label_set(ctx, "status", &title);
    for i in 0..MAX_ROWS {
        let row = format!("row{}", i);
        if !uk::exists(ctx, &row) { continue; }
        let li = if sel == 4 { if i < 5 { usize::MAX } else { i - 5 } } else { i };
        if li == usize::MAX || li >= lines.len() {
            uk::set_visible(ctx, &row, false);
            uk::set_disabled(ctx, &row, true);
            continue;
        }
        uk::set_visible(ctx, &row, true);
        let (disp, cid) = match lines[li].split_once(CHAMP_SEP) { Some((d, c)) => (d.to_string(), c.to_string()), None => (lines[li].clone(), String::new()) };
        row_champs[i] = cid.clone();
        uk::set_disabled(ctx, &row, cid.is_empty());
        let parts: Vec<&str> = disp.split('\t').collect();
        let is_slot = cid.starts_with("SLOT:");
        let is_action = is_slot || cid == "CLEAR" || cid.contains('=');
        let has_champ = !cid.is_empty() && !is_action;
        let face = format!("r{}face", i);
        let ok = has_champ && uk::image_set_champion_face(ctx, &face, &cid, 28.0, 28.0);
        uk::set_visible(ctx, &face, ok);
        let (c0_text, name_text): (&str, &str) = if sel == 1 || sel == 3 || sel == 4 {
            if has_champ { ("", parts.first().copied().unwrap_or("").trim_start()) } else { (parts.first().copied().unwrap_or(""), "") }
        } else {
            (parts.first().copied().unwrap_or(""), parts.get(6).copied().unwrap_or(""))
        };
        uk::label_set(ctx, &format!("r{}c0", i), c0_text);
        let c = if is_slot {
            let active = cid.strip_prefix("SLOT:").and_then(|k| DSLOT_KEYS.iter().position(|d| *d == k)) == Some(dslot_sel);
            if active { teal } else { dim }
        } else { Rgba::hex(0xe8e8e8ff) };
        uk::label_set_color(ctx, &format!("r{}c0", i), c);
        set_wr_label(ctx, &format!("r{}mname", i), name_text);
        if sel == 1 || sel == 3 || sel == 4 {
            uk::rect_set_color(ctx, &format!("r{}bg", i), Rgba::hex(0x00000000));
            uk::label_set(ctx, &format!("r{}tt", i), "");
            for k in ["sc", "wr", "pr", "p5"] { uk::label_set(ctx, &format!("r{}{}", i, k), ""); }
            set_wr_label(ctx, &format!("r{}val", i), parts.get(4).copied().unwrap_or(""));
        } else {
            let tier = parts.get(1).copied().unwrap_or("");
            uk::rect_set_color(ctx, &format!("r{}bg", i), if tier.is_empty() { Rgba::hex(0x00000000) } else { tier_color(tier) });
            uk::label_set(ctx, &format!("r{}tt", i), tier);
            uk::label_set(ctx, &format!("r{}sc", i), parts.get(2).copied().unwrap_or(""));
            set_wr_label(ctx, &format!("r{}wr", i), parts.get(3).copied().unwrap_or(""));
            uk::label_set(ctx, &format!("r{}pr", i), parts.get(4).copied().unwrap_or(""));
            uk::label_set(ctx, &format!("r{}p5", i), parts.get(5).copied().unwrap_or(""));
            uk::label_set(ctx, &format!("r{}val", i), "");
        }
    }
    *ROW_CHAMP.lock().unwrap_or_else(|e| e.into_inner()) = row_champs;
    // 탭/바 라벨 색 재확정(행 렌더 이후).
    hl(ctx, "tab", NUM_TABS, sel); hl(ctx, "role", 6, role_sel); hl(ctx, "csub", 4, csub_sel);
    hl(ctx, "scope", 3, scope_sel); hl(ctx, "dslot", 4, dslot_sel);
}

fn register_routes(ctx: &mut StableClient<'_>) -> usize {
    let mut routes: Vec<(String, uk::ClickFn)> = Vec::new();
    let r = |routes: &mut Vec<(String, uk::ClickFn)>, id: String, f: uk::ClickFn| routes.push((id, f));
    for i in 0..NUM_TABS { r(&mut routes, format!("tab{}", i), Arc::new(move || { SEL_TAB.store(i, Ordering::Relaxed); })); }
    r(&mut routes, "refresh_btn".into(), Arc::new(|| { write_control("refresh=1"); }));
    r(&mut routes, "close_btn".into(), Arc::new(|| { SHOW_OVERLAY.store(false, Ordering::Relaxed); }));
    r(&mut routes, "draft_toggle".into(), Arc::new(|| { let v = SHOW_OVERLAY.load(Ordering::Relaxed); SHOW_OVERLAY.store(!v, Ordering::Relaxed); }));
    for i in 0..4 { r(&mut routes, format!("csub{}", i), Arc::new(move || { SEL_CHAMPSUB.store(i, Ordering::Relaxed); })); }
    for i in 0..6 { r(&mut routes, format!("role{}", i), Arc::new(move || { SEL_ROLE.store(i, Ordering::Relaxed); write_control_state(); })); }
    for i in 0..3 { r(&mut routes, format!("scope{}", i), Arc::new(move || { SEL_SCOPE.store(i, Ordering::Relaxed); write_control_state(); })); }
    for i in 0..4 { r(&mut routes, format!("dslot{}", i), Arc::new(move || { SEL_DSLOT.store(i, Ordering::Relaxed); write_control(&format!("draft_slot={}", DSLOT_KEYS[i])); })); }
    for k in 0..NGRID {
        r(&mut routes, format!("gcell{}", k), Arc::new(move || {
            let id = { let g = GRID_CHAMP.lock().unwrap_or_else(|e| e.into_inner()); g.get(k).cloned().unwrap_or_default() };
            if !id.is_empty() {
                if SEL_TAB.load(Ordering::Relaxed) == 4 { write_control(&format!("bp_assign={}", id)); }
                else { write_control(&format!("draft_toggle={}", id)); }
            }
        }));
    }
    for i in 0..5 { r(&mut routes, format!("bpset{}", i), Arc::new(move || { write_control(&format!("bp_setidx={}", i)); })); }
    const BP_RULES: [&str; 3] = ["classic", "fearless", "hardFearless"];
    for i in 0..3 { r(&mut routes, format!("bprule{}", i), Arc::new(move || { write_control(&format!("bp_rule={}", BP_RULES[i])); })); }
    for (i, n) in [(0usize, 2u8), (1usize, 3u8)] { r(&mut routes, format!("bpban{}", i), Arc::new(move || { write_control(&format!("bp_bans={}", n)); })); }
    r(&mut routes, "bpside".into(), Arc::new(|| { write_control("bp_flip=1"); }));
    r(&mut routes, "bppatch".into(), Arc::new(|| { write_control("bp_patch=1"); }));
    for k in ["stat", "meta", "game", "solo", "syn", "ctr"] {
        r(&mut routes, format!("bpwdec_{}", k), Arc::new(move || { write_control(&format!("bp_w={}:dec", k)); }));
        r(&mut routes, format!("bpwinc_{}", k), Arc::new(move || { write_control(&format!("bp_w={}:inc", k)); }));
    }
    for i in 0..MAX_ROWS {
        r(&mut routes, format!("row{}", i), Arc::new(move || {
            let id = { let g = ROW_CHAMP.lock().unwrap_or_else(|e| e.into_inner()); g.get(i).cloned().unwrap_or_default() };
            if id.is_empty() { return; }
            if id == "CLEAR" { write_control("draft_clear=1"); return; }
            if let Some((k, v)) = id.split_once('=') { write_control(&format!("{}={}", k, v)); return; }
            if let Some(key) = id.strip_prefix("SLOT:") {
                if let Some(idx) = DSLOT_KEYS.iter().position(|d| *d == key) { SEL_DSLOT.store(idx, Ordering::Relaxed); write_control(&format!("draft_slot={}", key)); }
                return;
            }
            if SEL_TAB.load(Ordering::Relaxed) == 4 { write_control(&format!("bp_assign={}", id)); return; }
            *LAST_CHAMP_ID.lock().unwrap_or_else(|e| e.into_inner()) = id;
            SEL_TAB.store(1, Ordering::Relaxed);
            write_control_state();
        }));
    }
    uk::ensure_clicks(ctx, &routes)
}

fn init(host: &StableHost) -> StableMod {
    host.log(LogLevel::Info, "tfm2_draft_overlay (stable 0.6.0)");
    let v = host.game_version();
    dbg_write(&format!("===== INIT 게임 {}.{}.{} host_abi={} sdk_abi={} =====", v.major, v.minor, v.patch, host.abi_level(), mod_api_stable::ABI_LEVEL));
    let mut decl = StableMod::new(MOD_ID);
    decl.set_extension(Ext);
    decl
}
declare_stable_mod!(init);
