//! tfm2_mod_scroll_fix — 모드 활성/비활성 토글 시 모드 목록 스크롤 초기화 방지
//! ===========================================================================
//! 문제: 타이틀 화면 모드 관리 팝업(title.ui #mods_popup)에서 모드의 활성/비활성
//!   토글을 누르면 게임이 목록을 재빌드하면서 스크롤이 맨 위로 초기화됨.
//!   (실측: 러너 인스턴스는 유지, ratio 만 한 프레임에 0.0 으로 스냅)
//! 해법 (순수 SDK 모드 — 게임 함수 후킹/RVA 0개, 패치 무관):
//!   - 매 프레임 mods_popup 하위의 목록 scroll_view(#contents, ScrollViewRunner)의
//!     ratio(+1740, f32 0~1)를 추적.
//!   - 클릭 감지 2중화:
//!     (1) filter_handler 비소비 필터 — Rc 포인터 동일성으로 존재 확인 후 재등록
//!         (len 비교는 게임이 더 긴 벡터로 갈아끼우면 교체를 놓침 — 실측 실패 원인)
//!     (2) WinAPI 폴백 — LMB 눌림 에지 + 커서가 토글 컬럼 영역(가상좌표) 안이면 기록
//!   - 클릭 후 1초(60프레임) 안에 리셋(ratio 0 스냅 / 러너 교체 / 팝업 재생성)이
//!     관측되면 직전 ratio 를 3프레임 연속 복원.
//!   - 클릭 게이트 덕분에 유저가 휠/드래그로 맨 위로 올리는 정상 조작은 안 건드림.
//! ===========================================================================
#![allow(dead_code, unused_imports)]
use mod_api::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};

#[path = "C:/tfm2mods/ui_kit/ui_kit.rs"]
mod ui_kit;

const MOD_ID: &str = "tfm2_mod_scroll_fix";
const LOG_ENABLED: bool = false; // 배포 전 false (배포 체크리스트)

// 복원 파라미터
const RESTORE_FRAMES: usize = 3; // 리셋 감지 후 연속 복원 프레임 수
const CLICK_WINDOW: usize = 60;  // 클릭 → 리셋 감지 허용 프레임 창 (~1초)
const MIN_KEEP: f32 = 0.005;     // 이 미만 스크롤은 복원 무의미(사실상 맨 위)

// 토글 컬럼 히트영역 (게임 가상좌표 1920x1080; 팝업은 anchor 0.5 중앙 고정이라 상수)
// popup(1532x886) 좌상 = (194,97), left_panel +(24,68), scroll_view +y56,
// 행 toggle x=532 w=214 → x [752,966], 목록 y [221,867]. 여유 마진 포함.
const TG_X1: f32 = 745.0;
const TG_X2: f32 = 975.0;
const TG_Y1: f32 = 215.0;
const TG_Y2: f32 = 875.0;

// ── 상태 ──
static FRAME: AtomicUsize = AtomicUsize::new(0);        // post_update 프레임 카운터
static SAVED: AtomicU32 = AtomicU32::new(0);            // 저장된 ratio (f32 bits)
static LAST_BASE: AtomicUsize = AtomicUsize::new(0);    // 지난 프레임 러너 인스턴스 ptr
static RESTORE_LEFT: AtomicUsize = AtomicUsize::new(0); // 남은 강제복원 프레임
static CLICK_FRAME: AtomicUsize = AtomicUsize::new(usize::MAX); // 마지막 토글영역 클릭 프레임
static POPUP_OPEN: AtomicBool = AtomicBool::new(false);
static BOOT_LOGGED: AtomicBool = AtomicBool::new(false);
static PREV_LMB: AtomicBool = AtomicBool::new(false);

// ── WinAPI ──
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleFileNameW(h: usize, buf: *mut u16, n: u32) -> u32;
}
#[link(name = "user32")]
extern "system" {
    fn GetAsyncKeyState(vk: i32) -> i16;
}
const VK_LBUTTON: i32 = 0x01;

// ── 로그 (게임 exe 기준 동적 경로 — 설치위치 하드코딩 금지) ──
fn mod_dir() -> Option<String> {
    let mut buf = [0u16; 512];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), 512) } as usize;
    if n == 0 || n >= 512 { return None; }
    let exe = String::from_utf16_lossy(&buf[..n]);
    let dir = std::path::Path::new(&exe).parent()?;
    Some(format!("{}\\mods\\{}", dir.display(), MOD_ID))
}
fn log(msg: &str) {
    if !LOG_ENABLED { return; }
    if let Some(d) = mod_dir() {
        use std::io::Write;
        let _ = std::fs::OpenOptions::new().append(true).create(true)
            .open(format!("{d}\\scrollfix_log.txt"))
            .and_then(|mut f| writeln!(f, "[f{}] {}", FRAME.load(Ordering::Relaxed), msg));
    }
}

// ── 목록 scroll_view 찾기: mods_popup 서브트리에서 id=="contents" && ScrollViewRunner ──
fn find_list_scroll(n: &Node) -> Option<usize> {
    if n.id == "contents" {
        if let Some(b) = ui_kit::runner_base(n, "ScrollViewRunner") {
            return Some(b);
        }
    }
    for c in n.child.iter() {
        if let Some(b) = find_list_scroll(c) {
            return Some(b);
        }
    }
    None
}

// ── 클릭 감지 (1): filter_handler 비소비 필터 — Rc 동일성으로 존재 확인 ──
// post_update 는 메인 스레드 단일 → thread_local 로 내 필터 Rc 보관.
thread_local! {
    static MY_FILTER: RefCell<Option<Rc<dyn Fn(&UIEvent) -> bool>>> = RefCell::new(None);
}
fn ensure_click_recorder(ui: &mut GameUI) {
    MY_FILTER.with(|slot| {
        let mut slot = slot.borrow_mut();
        let present = slot.as_ref().map_or(false, |mine| {
            ui.filter_handler.iter().any(|(f, _)| Rc::ptr_eq(f, mine))
        });
        if present { return; }
        let filter: Rc<dyn Fn(&UIEvent) -> bool> = Rc::new(|e: &UIEvent| {
            if let UIEvent::Click { path, .. } = e {
                if POPUP_OPEN.load(Ordering::Relaxed) {
                    log(&format!("CLICK(fh) path='{path}'"));
                    if path.ends_with(".enabled") || path.ends_with(".disabled") {
                        CLICK_FRAME.store(FRAME.load(Ordering::Relaxed), Ordering::Relaxed);
                    }
                }
            }
            false // 절대 소비하지 않음 — 관찰만
        });
        let handler: Rc<dyn Fn(&mut UIEventHandlerContext<(), UIOutEvent>)> = Rc::new(|_| {});
        ui.filter_handler.push((filter.clone(), handler));
        *slot = Some(filter);
        log(&format!("click filter (re)registered (fh len {})", ui.filter_handler.len()));
    });
}

// ── 클릭 감지 (2): WinAPI 폴백 — LMB 에지 + 커서가 토글 컬럼 안 ──
fn poll_mouse_toggle_click() {
    let btn = unsafe { (GetAsyncKeyState(VK_LBUTTON) as u16) & 0x8000 != 0 };
    let was = PREV_LMB.swap(btn, Ordering::Relaxed);
    if btn && !was {
        let (gx, gy) = ui_kit::cursor_to_game();
        if gx >= TG_X1 && gx <= TG_X2 && gy >= TG_Y1 && gy <= TG_Y2 {
            CLICK_FRAME.store(FRAME.load(Ordering::Relaxed), Ordering::Relaxed);
            log(&format!("CLICK(mouse) gx={gx:.0} gy={gy:.0} — toggle zone"));
        }
    }
}

fn tick(ui: &mut GameUI) {
    let frame = FRAME.fetch_add(1, Ordering::Relaxed) + 1;
    if !BOOT_LOGGED.swap(true, Ordering::Relaxed) {
        log("mod alive — first post_update");
    }
    ensure_click_recorder(ui);

    // 팝업 탐색. 없거나 닫힘 → 인스턴스 추적만 끊음 (SAVED/클릭 기억은 유지:
    // 팝업 통째 재생성 케이스에서 재등장 시 복원 가능해야 함)
    let root = &ui.root;
    let popup = ui_kit::find(root, "mods_popup");
    let visible = popup.as_ref().map(|p| p.visible).unwrap_or(false);
    if !visible {
        if POPUP_OPEN.swap(false, Ordering::Relaxed) {
            log(&format!("popup gone/hidden (found={})", popup.is_some()));
        }
        LAST_BASE.store(0, Ordering::Relaxed);
        return;
    }
    let popup = popup.unwrap();
    if !POPUP_OPEN.swap(true, Ordering::Relaxed) {
        log("popup visible");
    }
    poll_mouse_toggle_click(); // 팝업 열린 동안만 폴링

    let Some(base) = find_list_scroll(popup) else {
        log("popup visible but list scroll_view NOT found");
        return;
    };
    let Some(cur) = ui_kit::runner_rd_f32(base, ui_kit::off::SCROLL_RATIO) else {
        log(&format!("scroll base={base:#x} ratio unreadable"));
        return;
    };

    let saved = f32::from_bits(SAVED.load(Ordering::Relaxed));
    let prev_base = LAST_BASE.swap(base, Ordering::Relaxed);
    let clicked_recently = {
        let cf = CLICK_FRAME.load(Ordering::Relaxed);
        cf != usize::MAX && frame.wrapping_sub(cf) <= CLICK_WINDOW
    };

    if prev_base == 0 {
        // 팝업(재)등장 첫 프레임
        if clicked_recently && saved > MIN_KEEP && cur < saved {
            RESTORE_LEFT.store(RESTORE_FRAMES, Ordering::Relaxed);
            log(&format!("popup reappeared after click — restoring {saved:.4} (cur={cur:.4})"));
        } else {
            SAVED.store(cur.to_bits(), Ordering::Relaxed);
            log(&format!("scroll adopt base={base:#x} ratio={cur:.4}"));
        }
    } else {
        // 리셋 감지: (a) 러너 인스턴스 교체 (b) 한 프레임에 ratio 0 스냅 (실측 유형)
        let rebuilt = base != prev_base;
        let snapped = cur == 0.0 && saved > MIN_KEEP;
        if (rebuilt || snapped) && saved > MIN_KEEP {
            if clicked_recently {
                RESTORE_LEFT.store(RESTORE_FRAMES, Ordering::Relaxed);
                log(&format!(
                    "reset detected (rebuilt={rebuilt} snapped={snapped}) cur={cur:.4} — restoring {saved:.4}"
                ));
            } else if rebuilt {
                log(&format!("instance changed w/o recent click ({prev_base:#x}->{base:#x}) — adopt"));
            } else {
                log(&format!("snap to 0 w/o recent click (saved={saved:.4}) — treated as user scroll"));
            }
        }
    }

    if RESTORE_LEFT.load(Ordering::Relaxed) > 0 {
        RESTORE_LEFT.fetch_sub(1, Ordering::Relaxed);
        let ok = ui_kit::runner_wr_f32(base, ui_kit::off::SCROLL_RATIO, saved);
        log(&format!("restore write ratio={saved:.4} ok={ok} (left={})", RESTORE_LEFT.load(Ordering::Relaxed)));
    } else {
        if (cur - saved).abs() > 0.01 {
            log(&format!("ratio {saved:.4} -> {cur:.4} (user scroll)"));
        }
        SAVED.store(cur.to_bits(), Ordering::Relaxed);
    }
}

// ── 모드 등록 ──
struct ScrollFixExt;
impl ModExtension for ScrollFixExt {
    fn post_update(&self, _scene: &mut Scene, ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        tick(ui);
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(ScrollFixExt);
    reg
}
declare_mod!(init);
