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

use config::{MASK_ALL, POS_NAMES};

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
static DUMPED_OPTION: AtomicBool = AtomicBool::new(false);
static DUMPED_POPUP: AtomicBool = AtomicBool::new(false);
fn maybe_dump_ui(root: &Node) {
    // 환경설정(option) 화면이 떠 있으면 1회 덤프 (게임플레이 탭 컨테이너 id 확보용).
    let opt = node_has_id(root, "option") || node_has_id(root, "gameplay");
    if opt {
        if !DUMPED_OPTION.swap(true, Ordering::Relaxed) {
            let mut s = String::from("=== option/settings 노드 트리 ===\n");
            dump_tree(root, 0, &mut s);
            if let Some(d) = mod_dir() {
                let _ = std::fs::write(format!("{d}\\ui_tree_option.txt"), s);
            }
            config::dlog("option 트리 덤프 완료 → ui_tree_option.txt");
        }
    } else {
        DUMPED_OPTION.store(false, Ordering::Relaxed);
    }
    // 커스텀 챔피언 팝업(시작 챔피언 선택)이 떠 있으면 1회 덤프 (복제 구조 파악용).
    let pop = node_has_id(root, "custom_champion");
    if pop {
        if !DUMPED_POPUP.swap(true, Ordering::Relaxed) {
            let mut s = String::from("=== custom_champion_popup 노드 트리 ===\n");
            dump_tree(root, 0, &mut s);
            if let Some(d) = mod_dir() {
                let _ = std::fs::write(format!("{d}\\ui_tree_popup.txt"), s);
            }
            config::dlog("popup 트리 덤프 완료 → ui_tree_popup.txt");
        }
    } else {
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
