//! tfm2_champ_pos_lock — 특정 챔피언을 특정 포지션에서만 쓰게 제한.
//! ===========================================================================
//! 설정은 인게임 UI(환경설정 → 게임플레이 → '포지션 제한' 버튼)로 한다.
//!   UI 가 포지션별 허용 챔피언 화이트리스트를 champ_pos_lock_state.txt 에 쓰고,
//!   이 파일이 그걸 읽어 챔피언별 허용 포지션 5비트 마스크로 환산해 게이트가 소비.
//!   (포지션 화이트리스트가 비면 그 포지션은 전 챔피언 허용.)
//!
//! 축1 AI 픽 게이트(ModDraftScoreHook.score_pick): 매칭(홀 조건) 깨는 후보 Replace(-1e9).
//! 축2 AI 배정(hookA)·유저 픽 차단(hookC/D): hooks.rs.
//! ===========================================================================

use mod_api::*;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU64, Ordering};
use std::sync::OnceLock;

mod config;
mod hooks;
mod inject;

#[path = r"C:\tfm2mods\ui_kit\ui_kit.rs"]
mod ui_kit;

use config::{MASK_ALL, POS_NAMES};
use std::rc::Rc;

pub(crate) const MOD_ID: &str = "tfm2_champ_pos_lock";

// build_inj.ps1 신원 검증용 — dll 안에 lib.rs 절대경로 문자열 필요(stale/타모드 차단).
#[no_mangle]
pub extern "C" fn tfm2_champ_pos_lock_src_id() -> *const u8 {
    concat!(file!(), "\0").as_bytes().as_ptr()
}

// ── 게임 exe 기준 모드 폴더 (설치위치 무관 — 경로 하드코딩 금지 규칙) ──
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleFileNameW(h: usize, buf: *mut u16, n: u32) -> u32;
}

pub(crate) fn mod_dir() -> Option<String> {
    let mut buf = [0u16; 512];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), 512) } as usize;
    if n == 0 || n >= 512 {
        return None;
    }
    let exe = String::from_utf16_lossy(&buf[..n]);
    let dir = std::path::Path::new(&exe).parent()?;
    Some(format!("{}\\mods\\{}", dir.display(), MOD_ID))
}

// ── 챔피언 인덱스 → 이름/마스크 ─────────────────────────────────────────────
// ctx candidate 인덱스 = db.available_champions 순서(모드 챔프는 그 뒤) 전제.
// NAMES 는 1회 캡처(경기 중 불변), MASKS 는 상태(UI 편집) 버전이 바뀔 때 재계산.
static NAMES: OnceLock<Vec<String>> = OnceLock::new(); // 원본 표기(UI/덤프용)
static IDX_MASK: AtomicPtr<Vec<u8>> = AtomicPtr::new(core::ptr::null_mut());
static APPLIED_VER: AtomicU64 = AtomicU64::new(u64::MAX);

/// 인덱스별 마스크 스냅샷(락 없음 — 버전 변경 시 통째 교체·이전 것 leak=드묾).
pub fn masks() -> Option<&'static Vec<u8>> {
    let p = IDX_MASK.load(Ordering::Acquire);
    if p.is_null() {
        None
    } else {
        Some(unsafe { &*p })
    }
}
fn publish_masks(v: Vec<u8>) {
    let p = Box::into_raw(Box::new(v));
    IDX_MASK.store(p, Ordering::Release);
}
/// 사용 가능 챔피언 목록(NAMES) 노출 — UI 가 그리드를 채울 때 사용.
pub fn champ_names() -> Option<&'static Vec<String>> {
    NAMES.get()
}

static CNT_VETO: AtomicU64 = AtomicU64::new(0);
static CNT_SEEN: AtomicU64 = AtomicU64::new(0);
static CNT_FAILOPEN: AtomicU64 = AtomicU64::new(0);
static FLUSH_TICK: AtomicU64 = AtomicU64::new(0);

// UI: '포지션 제한' 버튼 클릭 라우팅 + 팝업 열림 상태.
use std::sync::atomic::AtomicUsize;
static CLICK_LAST: AtomicUsize = AtomicUsize::new(usize::MAX);
static POPUP_OPEN: AtomicBool = AtomicBool::new(false);
static CNT_ROW_CLICK: AtomicU64 = AtomicU64::new(0);
/// 현재 선택된 포지션 탭: 0=탑 1=정글 2=미드 3=원딜 4=서폿 (전체 탭 없음 — 유저 지시)
static SEL_POS: AtomicUsize = AtomicUsize::new(0);
const NCELLS: usize = 120;
const TAB_IDS: [&str; 5] = ["tab_top", "tab_jungle", "tab_mid", "tab_bottom", "tab_support"];
static GRID_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
static mut CELL_BUF: [[u8; 96]; NCELLS] = [[0u8; 96]; NCELLS];

// 챔피언 스프라이트 에셋 키 + UV 테이블(CHAMP_UV/CHAMP_KEY).
include!(r"C:\tfm2mods\tfm2_champ_pos_lock\assets\champ_uv.rs");
fn champ_key(id: &str) -> Option<String> {
    CHAMP_KEY.iter().find(|e| e.0 == id).map(|e| e.1.to_string())
}
/// ImageRunner 커스텀 UV: +0xa4 flag=1, +0xa8..0xb4 = 4개 f32.
unsafe fn set_img_uv(dp: usize, a: f32, b: f32, c: f32, d: f32) {
    *((dp + 0xa4) as *mut u8) = 1;
    *((dp + 0xa8) as *mut f32) = a;
    *((dp + 0xac) as *mut f32) = b;
    *((dp + 0xb0) as *mut f32) = c;
    *((dp + 0xb4) as *mut f32) = d;
}
/// 노드 크기(복사본 6곳).
fn set_node_wh(node: &Node, w: f32, h: f32) {
    let na = node as *const Node as usize;
    for off in [0x74usize, 0xf4, 0x174, 0x1f4, 0x248, 0x258] {
        ui_kit::runner_wr_f32(na, off, w);
    }
    for off in [0x7cusize, 0xfc, 0x17c, 0x1fc, 0x24c, 0x25c] {
        ui_kit::runner_wr_f32(na, off, h);
    }
}

/// 셀 아이콘 = 챔피언 전신 스프라이트(#sheet) + UV 크롭(CHAMP_UV) + 종횡비 리사이즈.
unsafe fn set_cell_icon(icon: &mut Node, k: usize, lower: &str) {
    let Some(dp) = ui_kit::runner_base(icon, "ImageRunner") else {
        return;
    };
    if k >= NCELLS {
        return;
    }
    let uv = CHAMP_UV
        .iter()
        .find(|e| e.0 == lower)
        .map(|e| (e.1, e.2, e.3 - e.1, e.4 - e.2, e.5, e.6));
    let Some(key) = champ_key(lower) else {
        *((dp + 0xa4) as *mut u8) = 0;
        core::ptr::write_unaligned((dp + 0x10) as *mut u64, 0u64);
        return;
    };
    let full = format!("{key}#sheet");
    let kb = full.as_bytes();
    if kb.len() > 96 {
        return;
    }
    let buf = core::ptr::addr_of_mut!(CELL_BUF[k]) as *mut u8;
    core::ptr::copy_nonoverlapping(kb.as_ptr(), buf, kb.len());
    let gate: i64 = if uv.is_some() { -1 } else { 0 };
    core::ptr::write_unaligned(dp as *mut u64, 0u64);
    core::ptr::write_unaligned((dp + 0x08) as *mut u64, buf as u64);
    core::ptr::write_unaligned((dp + 0x10) as *mut u64, kb.len() as u64);
    core::ptr::write_unaligned((dp + 0x18) as *mut i64, gate);
    if let Some((x, y, w, h, fw, fh)) = uv {
        set_img_uv(dp, x, y, w, h);
        if fw > 0.0 && fh > 0.0 {
            let (bw, bh) = (84.0f32, 84.0f32);
            let ar = fw / fh;
            let (mut w2, mut h2) = (bh * ar, bh);
            if w2 > bw {
                w2 = bw;
                h2 = bw / ar;
            }
            set_node_wh(icon, w2, h2);
        }
    } else {
        *((dp + 0xa4) as *mut u8) = 0;
    }
}

/// 노드 높이(복사본 6곳) — 스크롤 컨텐츠 높이 지정용.
fn set_node_h(node: &Node, h: f32) {
    let na = node as *const Node as usize;
    for off in [0x7cusize, 0xfc, 0x17c, 0x1fc, 0x24c, 0x25c] {
        ui_kit::runner_wr_f32(na, off, h);
    }
}

/// 팝업 그리드를 현재 탭·상태에 맞게 채운다(변경 시에만).
fn fill_grid(root: &mut Node) {
    let pos = SEL_POS.load(Ordering::Relaxed);
    let ver = config::state_version();
    let Some(champs) = champ_names() else { return };
    let sig = (pos as u64) << 48 ^ ver ^ ((champs.len() as u64) << 32);
    if GRID_SIG.swap(sig, Ordering::Relaxed) == sig {
        return;
    }
    let Some(pop) = ui_kit::find_mut(root, "pos_lock_popup") else {
        return;
    };
    // 탭 하이라이트
    for (i, t) in TAB_IDS.iter().enumerate() {
        ui_kit::toggle_set_by_id(pop, t, i == pos);
    }
    // 요약 라벨
    let cnt = config::pos_count(pos);
    if let Some(n) = ui_kit::find_mut(pop, "summary") {
        ui_kit::label_set(n, &format!("{} 포지션", POS_NAMES_KR[pos]));
    }
    if let Some(n) = ui_kit::find_mut(pop, "count_label") {
        let s = if cnt == 0 {
            "선택 안 함 → 모든 챔피언 허용".to_string()
        } else {
            format!("선택: {cnt}개", cnt = cnt)
        };
        ui_kit::label_set(n, &s);
    }
    // 그리드 셀
    let Some(contents) = ui_kit::find_mut(pop, "contents") else {
        return;
    };
    let dbg = config::get().debug;
    let mut sample = String::new();
    for (k, cell) in contents.child.iter_mut().enumerate() {
        if let Some(champ) = champs.get(k) {
            cell.visible = true;
            let lower = champ.to_ascii_lowercase();
            let listed = config::is_listed(pos, &lower);
            for c in cell.child.iter_mut() {
                match c.id.as_str() {
                    "name" => {
                        // i18n 태그로 세팅 → 게임이 로케일 문자열로 자동 해석(전 챔프·라이브).
                        ui_kit::label_set(
                            c,
                            &format!("#asset/base/text/champion?description.{lower}.name"),
                        );
                    }
                    "sel" => c.visible = listed,
                    "icon" => unsafe {
                        set_cell_icon(c, k, &lower);
                        let _ = (dbg, &mut sample);
                    },
                    _ => {}
                }
            }
        } else {
            cell.visible = false;
        }
    }
    // 스크롤: 컨텐츠 높이를 보이는 셀 수로 지정(셀 120h + 세로간격 15, 9열 가정).
    let n = champs.len().min(NCELLS);
    let rows = n.div_ceil(9);
    let h = (rows as f32) * (120.0 + 15.0) + 16.0;
    set_node_h(contents, h);
    if dbg {
        config::dlog(&format!("grid fill: h={h} icons={sample}"));
    }
}

const POS_NAMES_KR: [&str; 5] = ["탑", "정글", "미드", "원딜", "서폿"];

fn mask_str(m: u8) -> String {
    (0..5)
        .filter(|p| m & (1 << p) != 0)
        .map(|p| POS_NAMES[p])
        .collect::<Vec<_>>()
        .join(",")
}

/// 제한 챔프 마스크들에 서로 다른 포지션을 하나씩 줄 수 있는가 (5×5 이하 백트래킹).
fn feasible(masks: &mut Vec<u8>) -> bool {
    if masks.len() > 5 {
        return false;
    }
    masks.sort_by_key(|m| m.count_ones());
    fn rec(ms: &[u8], used: u8) -> bool {
        let Some((&m, rest)) = ms.split_first() else {
            return true;
        };
        for p in 0..5u8 {
            let b = 1u8 << p;
            if m & b != 0 && used & b == 0 && rec(rest, used | b) {
                return true;
            }
        }
        false
    }
    rec(masks, 0)
}

fn taken(c: usize, ctx: &DraftScoreContext) -> bool {
    ctx.ally_ban.contains(&c)
        || ctx.enemy_ban.contains(&c)
        || ctx.ally_pick.contains(&c)
        || ctx.enemy_pick.contains(&c)
}

/// ★fail-open: 남은 풀에 "픽해도 매칭 유지되는" 후보가 하나도 없으면 게이트 해제.
fn pool_has_feasible(ctx: &DraftScoreContext, masks: &[u8], pinned: &[u8]) -> bool {
    use std::cell::Cell;
    thread_local! {
        static CACHE: Cell<(u64, bool)> = const { Cell::new((0, false)) };
    }
    let mut key: u64 = 0xcbf29ce484222325;
    let mut mix = |v: u64| {
        key ^= v;
        key = key.wrapping_mul(0x100000001b3);
    };
    for &i in ctx.ally_pick {
        mix(i as u64 + 1);
    }
    mix(0x5eed ^ (ctx.ally_ban.len() as u64) << 8 ^ (ctx.enemy_ban.len() as u64));
    mix(ctx.available_champions.len() as u64 | 1 << 32);
    if let Some(hit) = CACHE.with(|c| {
        let (k, v) = c.get();
        (k == key && k != 0).then_some(v)
    }) {
        return hit;
    }
    let mut found = false;
    let mut v: Vec<u8> = Vec::with_capacity(pinned.len() + 1);
    for &c in ctx.available_champions {
        if taken(c, ctx) {
            continue;
        }
        let m = masks.get(c).copied().unwrap_or(MASK_ALL);
        v.clear();
        v.extend_from_slice(pinned);
        v.push(m);
        if feasible(&mut v) {
            found = true;
            break;
        }
    }
    CACHE.with(|c| c.set((key, found)));
    found
}

// ── AI 픽 게이트 (공식 확장점 — detour 불요) ───────────────────────────────
#[derive(Debug)]
struct PosLockDraftAi;

impl ModDraftScoreHook for PosLockDraftAi {
    fn id(&self) -> &str {
        "tfm2_champ_pos_lock.pick_gate"
    }

    fn score_pick(
        &self,
        ctx: &DraftScoreContext,
        candidate: usize,
        _base_score: f32,
    ) -> DraftScoreDecision {
        let cfg = config::get();
        if !cfg.enabled || !cfg.ai_pick_gate || !config::any_restricted() {
            return DraftScoreDecision::Pass;
        }
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let masks = masks()?;
            let cand = masks.get(candidate).copied().unwrap_or(MASK_ALL);
            CNT_SEEN.fetch_add(1, Ordering::Relaxed);
            if cand == MASK_ALL {
                return None;
            }
            let mut pinned: Vec<u8> = Vec::with_capacity(5);
            for &i in ctx.ally_pick {
                let m = masks.get(i).copied().unwrap_or(MASK_ALL);
                if m != MASK_ALL {
                    pinned.push(m);
                }
            }
            let mut pins = pinned.clone();
            pins.push(cand);
            if feasible(&mut pins) {
                return None;
            }
            if !pool_has_feasible(ctx, masks, &pinned) {
                CNT_FAILOPEN.fetch_add(1, Ordering::Relaxed);
                return None;
            }
            Some(())
        }));
        match r {
            Ok(Some(())) => {
                CNT_VETO.fetch_add(1, Ordering::Relaxed);
                if cfg.debug {
                    let name = NAMES
                        .get()
                        .and_then(|v| v.get(candidate))
                        .map(|s| s.as_str())
                        .unwrap_or("?");
                    config::dlog(&format!("pick_veto: {name} (idx={candidate})"));
                }
                DraftScoreDecision::Replace(-1.0e9)
            }
            _ => DraftScoreDecision::Pass,
        }
    }
}

// ── 마스크 재계산 (NAMES + 현재 상태) ───────────────────────────────────────
fn recompute_masks_if_needed() {
    let Some(names) = NAMES.get() else { return };
    let ver = config::state_version();
    if APPLIED_VER.load(Ordering::Relaxed) == ver {
        return;
    }
    let v: Vec<u8> = names
        .iter()
        .map(|n| config::mask_of(&n.to_ascii_lowercase()))
        .collect();
    publish_masks(v);
    APPLIED_VER.store(ver, Ordering::Relaxed);
    if config::get().debug {
        config::dlog(&format!("masks 재계산 (ver={ver})"));
    }
}

// ── 디버그: 노드 트리 덤프 (UI 주입점 파악용) ───────────────────────────────
fn node_has_id(n: &Node, sub: &str) -> bool {
    if n.id.contains(sub) {
        return true;
    }
    n.child.iter().any(|c| node_has_id(c, sub))
}
fn dump_tree(n: &Node, depth: usize, out: &mut String) {
    if depth > 14 {
        return;
    }
    out.push_str(&"  ".repeat(depth));
    out.push_str(&format!("{}  (child={}, vis={})\n", n.id, n.child.len(), n.visible));
    for c in &n.child {
        dump_tree(c, depth + 1, out);
    }
}
static DUMP_FRAME: AtomicU64 = AtomicU64::new(0);
static DUMPED_OPTION: AtomicBool = AtomicBool::new(false);
static DUMPED_POPUP: AtomicBool = AtomicBool::new(false);

fn maybe_dump_ui(root: &Node) {
    let f = DUMP_FRAME.fetch_add(1, Ordering::Relaxed);
    // 매 ~1초, 화면에 떠 있는 트리 전체를 통째로 덮어쓴다(무조건 — 어떤 root 를 받는지부터 확인).
    if f % 60 == 0 {
        let mut s = format!(
            "frame={f}  root.id='{}'  children={}\n",
            root.id,
            root.child.len()
        );
        s.push_str("[top-level child ids] ");
        for c in &root.child {
            s.push_str(&c.id);
            s.push(' ');
        }
        s.push_str("\n\n");
        dump_tree(root, 0, &mut s);
        if let Some(d) = mod_dir() {
            let _ = std::fs::write(format!("{d}\\ui_tree_live.txt"), &s);
        }
    }
    // 옵션/팝업이 트리에 나타나면 그 순간 스냅샷을 따로 남긴다(id 다양성 대비 넓게 매칭).
    let opt = node_has_id(root, "option") || node_has_id(root, "gameplay");
    if opt && !DUMPED_OPTION.swap(true, Ordering::Relaxed) {
        let mut s = String::from("=== option/settings ===\n");
        dump_tree(root, 0, &mut s);
        if let Some(d) = mod_dir() {
            let _ = std::fs::write(format!("{d}\\ui_tree_option.txt"), s);
        }
    }
    if !opt {
        DUMPED_OPTION.store(false, Ordering::Relaxed);
    }
    let pop = node_has_id(root, "custom_champion");
    if pop && !DUMPED_POPUP.swap(true, Ordering::Relaxed) {
        let mut s = String::from("=== custom_champion_popup ===\n");
        dump_tree(root, 0, &mut s);
        if let Some(d) = mod_dir() {
            let _ = std::fs::write(format!("{d}\\ui_tree_popup.txt"), s);
        }
    }
    if !pop {
        DUMPED_POPUP.store(false, Ordering::Relaxed);
    }
}

// ── 진입 ──────────────────────────────────────────────────────────────────
struct PosLockExt;

impl ModExtension for PosLockExt {
    fn post_update(&self, scene: &mut Scene, ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let cfg = config::get();
            if !cfg.enabled {
                return;
            }
            // 챔피언 목록 1회 캡처(관리화면 프레임에서도 Scene::InGame 매치).
            if NAMES.get().is_none() {
                if let Scene::InGame { data } = scene {
                    let db = data.db();
                    if !db.available_champions.is_empty() {
                        let ids: Vec<String> = db.available_champions.clone();
                        if cfg.debug {
                            let mut s = String::from("# 현재 사용 가능한 챔피언 id 목록\n");
                            for id in &ids {
                                s.push_str(id);
                                s.push('\n');
                            }
                            if let Some(d) = mod_dir() {
                                let _ = std::fs::write(
                                    format!("{d}\\champ_pos_lock_champions.txt"),
                                    s,
                                );
                            }
                            config::dlog(&format!("캡처: 챔프 {}개", ids.len()));
                        }
                        let _ = NAMES.set(ids);
                    }
                }
            }
            // 상태(UI 편집)가 바뀌었으면 마스크 재계산.
            recompute_masks_if_needed();

            // 훅 설치 — 마스크 준비 후 1회 (late install, §3)
            if masks().is_some() {
                hooks::install_once();
                hooks::install_once_c();
            }

            // UI 주입점 덤프(개발용)
            if cfg.dump_ui {
                maybe_dump_ui(&ui.root);
            }

            // ── 인게임 UI: 환경설정 게임플레이 탭 '포지션 제한' 행 ──
            inject::install();
            // 게임플레이 탭이 보일 때만 내 행 표시(같은 탭의 banpick_style 행 visible 을 따라감).
            let gp_vis = ui_kit::find(&ui.root, "banpick_style")
                .map(|n| n.visible)
                .unwrap_or(false);
            if let Some(row) = ui_kit::find_mut(&mut ui.root, "pos_lock_row") {
                row.visible = gp_vis;
            }
            // 팝업 표시/숨김 + 그리드 채우기
            let open = POPUP_OPEN.load(Ordering::Relaxed);
            let present = ui_kit::find_mut(&mut ui.root, "pos_lock_popup").is_some();
            if !present {
                POPUP_OPEN.store(false, Ordering::Relaxed);
            } else if let Some(pop) = ui_kit::find_mut(&mut ui.root, "pos_lock_popup") {
                pop.visible = open;
            }
            if open && present {
                fill_grid(&mut ui.root);
            }

            // 클릭 라우팅
            let mut routes: Vec<(String, ui_kit::ClickFn)> = Vec::with_capacity(NCELLS + 16);
            routes.push(ui_kit::route(
                "pos_lock_configure",
                Rc::new(|| {
                    CNT_ROW_CLICK.fetch_add(1, Ordering::Relaxed);
                    POPUP_OPEN.store(true, Ordering::Relaxed);
                    GRID_SIG.store(u64::MAX, Ordering::Relaxed); // 열 때 강제 재채움
                    config::dlog("포지션 제한 버튼 클릭됨");
                }),
            ));
            let close: ui_kit::ClickFn = Rc::new(|| POPUP_OPEN.store(false, Ordering::Relaxed));
            routes.push(ui_kit::route("pos_lock_popup.close", close.clone()));
            routes.push(ui_kit::route("pos_lock_popup.cancel", close));
            routes.push(ui_kit::route(
                "pos_lock_popup.ok",
                Rc::new(|| {
                    config::save_state_to_file();
                    POPUP_OPEN.store(false, Ordering::Relaxed);
                    config::dlog("포지션 제한 저장");
                }),
            ));
            for (i, t) in TAB_IDS.iter().enumerate() {
                routes.push(ui_kit::route(
                    t,
                    Rc::new(move || SEL_POS.store(i, Ordering::Relaxed)),
                ));
            }
            routes.push(ui_kit::route(
                "clear_pos",
                Rc::new(|| config::clear_pos(SEL_POS.load(Ordering::Relaxed))),
            ));
            routes.push(ui_kit::route(
                "select_all_pos",
                Rc::new(|| {
                    if let Some(champs) = champ_names() {
                        let all: Vec<String> =
                            champs.iter().map(|c| c.to_ascii_lowercase()).collect();
                        config::set_pos(SEL_POS.load(Ordering::Relaxed), all);
                    }
                }),
            ));
            for k in 0..NCELLS {
                routes.push(ui_kit::route(
                    &format!("cell{k}"),
                    Rc::new(move || {
                        if let Some(champs) = champ_names() {
                            if let Some(c) = champs.get(k) {
                                config::toggle(
                                    SEL_POS.load(Ordering::Relaxed),
                                    &c.to_ascii_lowercase(),
                                );
                            }
                        }
                    }),
                ));
            }
            ui_kit::ensure_clicks(ui, &CLICK_LAST, routes);

            // 디버그 카운터 주기 flush
            if cfg.debug {
                let t = FLUSH_TICK.fetch_add(1, Ordering::Relaxed);
                if t % 600 == 599 {
                    config::dlog(&format!(
                        "counters: seen={} veto={} failopen={} mask_call={} mask_adj={} A={} C={} D={} ui_q={} ui_block={}",
                        CNT_SEEN.load(Ordering::Relaxed),
                        CNT_VETO.load(Ordering::Relaxed),
                        CNT_FAILOPEN.load(Ordering::Relaxed),
                        hooks::CNT_MASK_CALL.load(Ordering::Relaxed),
                        hooks::CNT_MASK_ADJ.load(Ordering::Relaxed),
                        hooks::INSTALL_STATE.load(Ordering::Relaxed),
                        hooks::INSTALL_STATE_C.load(Ordering::Relaxed),
                        hooks::INSTALL_STATE_D.load(Ordering::Relaxed),
                        hooks::CNT_UI_QUERY.load(Ordering::Relaxed),
                        hooks::CNT_UI_BLOCK.load(Ordering::Relaxed),
                    ));
                }
            }
        }));
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    config::load();
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(PosLockExt);
    reg.add_draft_score_hook(PosLockDraftAi);
    reg
}

declare_mod!(init);
