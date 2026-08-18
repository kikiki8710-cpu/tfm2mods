//! tfm2_html_overlay — 인게임 HTML 오버레이 (WebView2)
//! ===========================================================================
//! 목표: 게임 창 위에 항상 떠 있는 HTML 패널. 어느 게임 화면이든 표시되고,
//!       안 쓸 때는 [—] 버튼으로 작은 바 하나로 접어 구석에 둘 수 있다.
//!
//! 방식(순수 SDK — 게임함수 후킹/RVA 0개, 패치 무관):
//!   · post_update 첫 프레임에 전용 스레드 1개 생성(게임 스레드는 건드리지 않음).
//!   · 그 스레드에서 CoInitializeEx(STA) → 게임 메인 창을 **오너**로 하는
//!     WS_POPUP 창 생성(오너 관계 = 항상 게임 창 위 z-order, TOPMOST 불필요)
//!     → WebView2Loader.dll(모드 폴더 동봉)로 WebView2 부착.
//!   · COM은 외부 크레이트 없이 raw vtable FFI (빌드가 rustc 단독이라 crates 불가).
//!     사용하는 vtable 슬롯을 최소화(Env#3, Controller#4/#6/#23/#25, WebView#5)해
//!     레이아웃 오판 리스크를 줄임.
//!   · 상단 30px 스트립 = 드래그 이동(HTCAPTION) + [↻]리로드 + [—]접기 버튼.
//!     접기 = 창을 150px 바로 축소 + webview 숨김. Ctrl+F10 핫키 동일 토글.
//!   · 게임 창 이동/최소화 추적 = 200ms 타이머(창 이동 델타만큼 따라감.
//!     오너 최소화 시 owned 창은 OS가 자동 숨김 — 타이머는 보조).
//!
//! 파일(게임 exe 기준 동적 도출 — 설치경로 하드코딩 금지):
//!   <게임>\mods\tfm2_html_overlay\WebView2Loader.dat   — 필수 동봉(없으면 로그 남기고 비활성)
//!     (실체는 WebView2Loader.dll — .dll 이면 게임 모드 로더가 모드로 오인해 강제비활성됨)
//!   <게임>\mods\tfm2_html_overlay\html_overlay.cfg     — url/크기 설정(없으면 자동 생성)
//!   <게임>\mods\tfm2_html_overlay\index.html           — 기본 페이지(없으면 자동 생성)
//!   <게임>\mods\tfm2_html_overlay\html_overlay_err.txt — 오류 로그(항상)
//!   <게임>\mods\tfm2_html_overlay\html_overlay_log.txt — 상세 로그(LOG_ENABLED)
//!   웹뷰 프로필 데이터 = %LOCALAPPDATA%\tfm2_html_overlay\wv2_data (게임 폴더 오염 방지)
//!
//! ⚠ 알려진 한계: 게임이 **전용(exclusive) 전체화면**이면 별도 창이라 안 보인다.
//!   창모드/테두리없는 전체화면에서 사용할 것.
//! ===========================================================================
#![allow(dead_code, non_snake_case, clippy::missing_safety_doc)]
use mod_api::*;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicIsize, AtomicU32, AtomicUsize, Ordering};
use std::sync::Mutex;

const MOD_ID: &str = "tfm2_html_overlay";
const LOG_ENABLED: bool = true; // 배포 시 OFF (오류 로그는 별도로 항상 남음)

// build_inj.ps1 신원검증(dll 안에 소스 절대경로 존재)용.
// ⚠#[used] static 만으론 부족 — rust-lld 가 미참조 섹션을 GC 해 경로 문자열이
//   통째로 사라졌다(이 모드는 자체 패닉 위치 문자열도 없어 우연 통과도 안 됨).
//   #[no_mangle] pub = cdylib export ⟹ 링커가 못 지움.
#[used]
#[no_mangle]
pub static TFM2_HTML_OVERLAY_SRC: &str = file!();

const STRIP_H: i32 = 30; // 상단 스트립(px)
const BTN_W: i32 = 26; // 버튼 폭
const MAX_PRESETS: usize = 6;

// ── 상태 (오버레이 스레드 + wndproc 동일 스레드에서만 만짐. 원자형은 습관적 안전) ──
static STARTED: AtomicBool = AtomicBool::new(false);
static GAME_HWND: AtomicUsize = AtomicUsize::new(0);
static MY_HWND: AtomicUsize = AtomicUsize::new(0);
static COLLAPSED: AtomicBool = AtomicBool::new(false);
static ENV_PTR: AtomicUsize = AtomicUsize::new(0);
// 접기 전 창 rect(l,t,w,h)
static SAVED_RECT: Mutex<(i32, i32, i32, i32)> = Mutex::new((0, 0, 480, 620));
// 게임 창 마지막 rect(따라가기용)
static LAST_GAME: Mutex<Option<(i32, i32, i32, i32)>> = Mutex::new(None);
static NAV_URL: Mutex<String> = Mutex::new(String::new());
// 프리셋 — cfg presetN=이름|url[|가로x세로]. 버튼 클릭으로 전환(+창 크기 적용).
#[derive(Clone)]
struct Preset {
    name: String,
    url: String,
    w: i32, // 0 = 크기 미지정(현재 크기 유지)
    h: i32,
    keep: bool, // true = 탭 전환해도 안 끄고 유지(전용 webview 상주)
}
static PRESETS: Mutex<Vec<Preset>> = Mutex::new(Vec::new());
// 탭별 webview 슬롯: keep 탭 = 프리셋 index 태그로 상주, 비-keep 탭 = TAG_TRANSIENT 공용 1개.
const TAG_TRANSIENT: i64 = -1;
struct CtlSlot {
    tag: i64,
    ctl: usize, // ICoreWebView2Controller*
    wv: usize,  // ICoreWebView2*
}
static CTLS: Mutex<Vec<CtlSlot>> = Mutex::new(Vec::new());
static PENDING_TAGS: Mutex<Vec<i64>> = Mutex::new(Vec::new()); // 생성 요청 중(중복 방지)
// 탭별 마지막 사용자 지정 크기 (이름, w, h) — html_overlay_state.txt 로 영속.
// 우선순위: 기억된 크기 > cfg 의 |WxH.
static SIZE_MEMO: Mutex<Vec<(String, i32, i32)>> = Mutex::new(Vec::new());
// 현재 활성 프리셋 index (usize::MAX = 없음/커스텀 url)
static ACTIVE_PRESET: AtomicUsize = AtomicUsize::new(usize::MAX);

// ===========================================================================
// WinAPI FFI
// ===========================================================================
type HWND = usize;
type HRESULT = i32;

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct RECT {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}
#[repr(C)]
struct MSG {
    hwnd: HWND,
    message: u32,
    wparam: usize,
    lparam: isize,
    time: u32,
    pt_x: i32,
    pt_y: i32,
    _priv: u32,
}
#[repr(C)]
struct WNDCLASSW {
    style: u32,
    lpfnWndProc: usize,
    cbClsExtra: i32,
    cbWndExtra: i32,
    hInstance: usize,
    hIcon: usize,
    hCursor: usize,
    hbrBackground: usize,
    lpszMenuName: *const u16,
    lpszClassName: *const u16,
}
#[repr(C)]
struct PAINTSTRUCT {
    hdc: usize,
    fErase: i32,
    rcPaint: RECT,
    fRestore: i32,
    fIncUpdate: i32,
    rgbReserved: [u8; 32],
}
#[repr(C)]
struct MINMAXINFO {
    ptReserved: [i32; 2],
    ptMaxSize: [i32; 2],
    ptMaxPosition: [i32; 2],
    ptMinTrackSize: [i32; 2],
    ptMaxTrackSize: [i32; 2],
}

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleFileNameW(h: usize, buf: *mut u16, n: u32) -> u32;
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn LoadLibraryW(name: *const u16) -> usize;
    fn GetProcAddress(h: usize, name: *const u8) -> usize;
    fn GetCurrentProcessId() -> u32;
    fn GetLastError() -> u32;
}
#[link(name = "user32")]
extern "system" {
    fn RegisterClassW(wc: *const WNDCLASSW) -> u16;
    fn CreateWindowExW(
        exstyle: u32,
        class: *const u16,
        title: *const u16,
        style: u32,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        parent: HWND,
        menu: usize,
        hinst: usize,
        param: usize,
    ) -> HWND;
    fn DefWindowProcW(hwnd: HWND, msg: u32, w: usize, l: isize) -> isize;
    fn GetMessageW(msg: *mut MSG, hwnd: HWND, min: u32, max: u32) -> i32;
    fn TranslateMessage(msg: *const MSG) -> i32;
    fn DispatchMessageW(msg: *const MSG) -> isize;
    fn PostQuitMessage(code: i32);
    fn SetTimer(hwnd: HWND, id: usize, ms: u32, cb: usize) -> usize;
    fn ShowWindow(hwnd: HWND, cmd: i32) -> i32;
    fn IsWindow(hwnd: HWND) -> i32;
    fn IsWindowVisible(hwnd: HWND) -> i32;
    fn IsIconic(hwnd: HWND) -> i32;
    fn GetWindowRect(hwnd: HWND, r: *mut RECT) -> i32;
    fn GetClientRect(hwnd: HWND, r: *mut RECT) -> i32;
    fn SetWindowPos(hwnd: HWND, after: HWND, x: i32, y: i32, w: i32, h: i32, flags: u32) -> i32;
    fn EnumWindows(cb: extern "system" fn(HWND, isize) -> i32, l: isize) -> i32;
    fn GetWindowThreadProcessId(hwnd: HWND, pid: *mut u32) -> u32;
    fn GetWindow(hwnd: HWND, cmd: u32) -> HWND;
    fn GetWindowTextLengthW(hwnd: HWND) -> i32;
    fn BeginPaint(hwnd: HWND, ps: *mut PAINTSTRUCT) -> usize;
    fn EndPaint(hwnd: HWND, ps: *const PAINTSTRUCT) -> i32;
    fn InvalidateRect(hwnd: HWND, r: *const RECT, erase: i32) -> i32;
    fn LoadCursorW(hinst: usize, name: usize) -> usize;
    fn RegisterHotKey(hwnd: HWND, id: i32, mods: u32, vk: u32) -> i32;
    fn ScreenToClient(hwnd: HWND, pt: *mut [i32; 2]) -> i32;
    fn FillRect(hdc: usize, r: *const RECT, brush: usize) -> i32;
    fn PostMessageW(hwnd: HWND, msg: u32, w: usize, l: isize) -> i32;
    fn GetForegroundWindow() -> HWND;
    fn GetAncestor(hwnd: HWND, flags: u32) -> HWND;
}
#[link(name = "comdlg32")]
extern "system" {
    fn GetOpenFileNameW(ofn: *mut OPENFILENAMEW) -> i32;
}
#[repr(C)]
struct OPENFILENAMEW {
    lStructSize: u32,
    hwndOwner: usize,
    hInstance: usize,
    lpstrFilter: *const u16,
    lpstrCustomFilter: *mut u16,
    nMaxCustFilter: u32,
    nFilterIndex: u32,
    lpstrFile: *mut u16,
    nMaxFile: u32,
    lpstrFileTitle: *mut u16,
    nMaxFileTitle: u32,
    lpstrInitialDir: *const u16,
    lpstrTitle: *const u16,
    Flags: u32,
    nFileOffset: u16,
    nFileExtension: u16,
    lpstrDefExt: *const u16,
    lCustData: usize,
    lpfnHook: usize,
    lpTemplateName: *const u16,
    pvReserved: usize,
    dwReserved: u32,
    FlagsEx: u32,
}
#[link(name = "gdi32")]
extern "system" {
    fn CreateSolidBrush(color: u32) -> usize;
    fn SetBkMode(hdc: usize, mode: i32) -> i32;
    fn SetTextColor(hdc: usize, color: u32) -> u32;
    fn TextOutW(hdc: usize, x: i32, y: i32, s: *const u16, n: i32) -> i32;
    fn SelectObject(hdc: usize, obj: usize) -> usize;
    fn CreateFontW(
        height: i32,
        width: i32,
        escapement: i32,
        orientation: i32,
        weight: i32,
        italic: u32,
        underline: u32,
        strikeout: u32,
        charset: u32,
        out_prec: u32,
        clip_prec: u32,
        quality: u32,
        pitch_family: u32,
        face: *const u16,
    ) -> usize;
}

/// 스트립용 폰트 2종 캐시 — ⚠GDI 기본 stock 폰트(System 비트맵)는 ↻/□ 등
/// 기호 글리프가 없어 깨진 네모로 그려진다(0.5.4 실사고) → 명시 생성 필수.
fn strip_fonts() -> (usize, usize) {
    static FONT_TEXT: AtomicUsize = AtomicUsize::new(0);
    static FONT_SYM: AtomicUsize = AtomicUsize::new(0);
    let mut ft = FONT_TEXT.load(Ordering::Relaxed);
    if ft == 0 {
        ft = unsafe {
            CreateFontW(-14, 0, 0, 0, 400, 0, 0, 0, 1 /*DEFAULT_CHARSET*/, 0, 0,
                5 /*CLEARTYPE*/, 0, wstr("Malgun Gothic").as_ptr())
        };
        FONT_TEXT.store(ft, Ordering::Relaxed);
    }
    let mut fs = FONT_SYM.load(Ordering::Relaxed);
    if fs == 0 {
        fs = unsafe {
            CreateFontW(-15, 0, 0, 0, 400, 0, 0, 0, 1, 0, 0, 5, 0,
                wstr("Segoe UI Symbol").as_ptr())
        };
        FONT_SYM.store(fs, Ordering::Relaxed);
    }
    (ft, fs)
}
#[link(name = "ole32")]
extern "system" {
    fn CoInitializeEx(reserved: usize, model: u32) -> HRESULT;
}

const WS_POPUP: u32 = 0x8000_0000;
const WS_THICKFRAME: u32 = 0x0004_0000;
const WS_CLIPCHILDREN: u32 = 0x0200_0000;
const WS_EX_TOOLWINDOW: u32 = 0x0000_0080;
const SW_HIDE: i32 = 0;
const SW_SHOWNOACTIVATE: i32 = 4;
const SWP_NOSIZE: u32 = 0x1;
const SWP_NOMOVE: u32 = 0x2;
const SWP_NOZORDER: u32 = 0x4;
const SWP_NOACTIVATE: u32 = 0x10;
const GW_OWNER: u32 = 4;
const WM_CREATE: u32 = 0x0001;
const WM_DESTROY: u32 = 0x0002;
const WM_MOVE: u32 = 0x0003;
const WM_SIZE: u32 = 0x0005;
const WM_PAINT: u32 = 0x000F;
const WM_ERASEBKGND: u32 = 0x0014;
const WM_GETMINMAXINFO: u32 = 0x0024;
const WM_NCHITTEST: u32 = 0x0084;
const WM_TIMER: u32 = 0x0113;
const WM_HOTKEY: u32 = 0x0312;
const WM_LBUTTONUP: u32 = 0x0202;
const WM_EXITSIZEMOVE: u32 = 0x0232;
const HTCLIENT: isize = 1;
const HTCAPTION: isize = 2;
const TRANSPARENT_BK: i32 = 1;
const MOD_CONTROL: u32 = 0x2;
const VK_F10: u32 = 0x79;
const TIMER_FOLLOW: usize = 1;
const HOTKEY_COLLAPSE: usize = 1;
const HOTKEY_OPEN_SAVE: usize = 2;
const WM_APP_REFRESH_DONE: u32 = 0x8001; // WM_APP+1: 세이브 갱신 파이프라인 완료
// 세이브 갱신 파이프라인 실행 중(스트립에 표시)
static REFRESHING: AtomicBool = AtomicBool::new(false);

// ===========================================================================
// 로그
// ===========================================================================
fn mod_dir() -> Option<String> {
    let mut buf = [0u16; 512];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), 512) } as usize;
    if n == 0 || n >= 512 {
        return None;
    }
    let exe = String::from_utf16_lossy(&buf[..n]);
    let dir = std::path::Path::new(&exe).parent()?;
    Some(format!("{}\\mods\\{}", dir.display(), MOD_ID))
}
fn write_line(file: &str, msg: &str) {
    if let Some(d) = mod_dir() {
        use std::io::Write;
        let _ = std::fs::create_dir_all(&d);
        let _ = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("{d}\\{file}"))
            .and_then(|mut f| writeln!(f, "{msg}"));
    }
}
fn log(msg: &str) {
    if LOG_ENABLED {
        write_line("html_overlay_log.txt", msg);
    }
}
/// 오류는 LOG_ENABLED 무관하게 항상 남긴다(조용한 비활성 방지).
fn elog(msg: &str) {
    write_line("html_overlay_err.txt", msg);
    log(&format!("ERR: {msg}"));
}

fn wstr(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

// ===========================================================================
// 설정
// ===========================================================================
struct Cfg {
    url: String,
    width: i32,
    height: i32,
    start_collapsed: bool,
    presets: Vec<Preset>,
}
fn default_index_html() -> &'static str {
    r#"<!doctype html>
<meta charset="utf-8">
<title>TFM2 HTML 오버레이</title>
<style>
  body{margin:0;font:14px/1.6 'Malgun Gothic',sans-serif;background:#14161c;color:#dfe3ec;padding:16px}
  h1{font-size:16px;margin:0 0 10px;color:#ffd23e}
  code{background:#232735;padding:1px 5px;border-radius:4px}
  .clock{font-size:28px;color:#7ec8ff;margin:12px 0}
  li{margin:4px 0}
</style>
<h1>TFM2 HTML 오버레이</h1>
<div class="clock" id="clk"></div>
<ul>
  <li>이 패널은 <b>어느 게임 화면에서든</b> 항상 떠 있습니다.</li>
  <li>위 스트립을 <b>드래그</b>하면 이동, 가장자리를 끌면 크기 조절.</li>
  <li><b>[—]</b> 버튼(또는 Ctrl+F10) = 작은 바로 접기/펴기.</li>
  <li><b>[↻]</b> 버튼 = 설정(<code>html_overlay.cfg</code>) 다시 읽고 새로고침.</li>
  <li>표시할 페이지 변경 = <code>html_overlay.cfg</code>의 <code>url=</code> 수정
      (로컬 html 경로 또는 https:// 주소).</li>
</ul>
<script>
  const f=()=>{document.getElementById('clk').textContent=new Date().toLocaleTimeString('ko-KR');};
  f(); setInterval(f,1000);
</script>
"#
}
fn default_cfg_text() -> &'static str {
    "# tfm2_html_overlay 설정\r\n\
     # url = 시작 페이지. https:// 주소, file:/// URL, 또는 로컬 파일 경로.\r\n\
     #       {mod} = 이 모드 폴더, {mods} = 게임 mods 폴더로 치환됩니다.\r\n\
     url={mod}\\index.html\r\n\
     # presetN = 스트립 버튼. 형식: 이름|url|가로x세로|keep (N=1~6)\r\n\
     #   keep = 탭을 바꿔도 페이지를 안 끄고 유지(다시 눌러도 이어서 봄). 크기·keep 생략 가능.\r\n\
     preset1=홈|{mod}\\index.html|480x620\r\n\
     # 패널 크기(px)\r\n\
     width=480\r\n\
     height=620\r\n\
     # 1 = 시작 시 접힌 상태\r\n\
     start_collapsed=0\r\n"
}
/// cfg 로드(없으면 기본 파일 생성). index.html 도 없으면 생성.
fn load_cfg() -> Cfg {
    let mut cfg = Cfg {
        url: String::new(),
        width: 480,
        height: 620,
        start_collapsed: false,
        presets: Vec::new(),
    };
    let Some(d) = mod_dir() else { return cfg };
    let _ = std::fs::create_dir_all(&d);
    let cfg_path = format!("{d}\\html_overlay.cfg");
    if !std::path::Path::new(&cfg_path).exists() {
        let _ = std::fs::write(&cfg_path, default_cfg_text());
    }
    let idx_path = format!("{d}\\index.html");
    if !std::path::Path::new(&idx_path).exists() {
        let _ = std::fs::write(&idx_path, default_index_html());
    }
    // {mods} = 게임 mods 루트 (= mod_dir 의 부모)
    let mods_root = std::path::Path::new(&d)
        .parent()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| d.clone());
    let expand = |v: &str| v.replace("{mod}", &d).replace("{mods}", &mods_root);
    let to_url = |raw: &str| -> String {
        if raw.starts_with("http://") || raw.starts_with("https://") || raw.starts_with("file://") {
            raw.to_string()
        } else {
            path_to_file_url(raw)
        }
    };
    let Ok(s) = std::fs::read_to_string(&cfg_path) else { return cfg };
    let mut raw_url = String::new();
    let mut presets: [Option<Preset>; MAX_PRESETS] = Default::default();
    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((k, v)) = line.split_once('=') else { continue };
        let (k, v) = (k.trim(), v.trim());
        match k {
            "url" => raw_url = v.to_string(),
            "width" => cfg.width = v.parse().unwrap_or(480),
            "height" => cfg.height = v.parse().unwrap_or(620),
            "start_collapsed" => cfg.start_collapsed = v == "1" || v.eq_ignore_ascii_case("true"),
            _ => {
                // presetN=이름|url[|WxH]
                if let Some(nstr) = k.strip_prefix("preset") {
                    if let Ok(n) = nstr.parse::<usize>() {
                        if (1..=MAX_PRESETS).contains(&n) {
                            let mut it = v.splitn(4, '|');
                            let name = it.next().unwrap_or("").trim();
                            let url = it.next().unwrap_or("").trim();
                            let size = it.next().unwrap_or("").trim();
                            let keep_s = it.next().unwrap_or("").trim();
                            let keep = keep_s.eq_ignore_ascii_case("keep")
                                || keep_s == "1"
                                || keep_s == "유지";
                            let (mut w, mut h) = (0, 0);
                            if let Some((ws, hs)) =
                                size.split_once(['x', 'X', '*'].as_ref())
                            {
                                w = ws.trim().parse().unwrap_or(0);
                                h = hs.trim().parse().unwrap_or(0);
                                if w > 0 {
                                    w = w.clamp(200, 3800);
                                }
                                if h > 0 {
                                    h = h.clamp(STRIP_H + 60, 2400);
                                }
                            }
                            if !name.is_empty() && !url.is_empty() {
                                presets[n - 1] = Some(Preset {
                                    name: name.to_string(),
                                    url: to_url(&expand(url)),
                                    w,
                                    h,
                                    keep,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    cfg.presets = presets.into_iter().flatten().collect();
    apply_size_memo(&mut cfg.presets); // 기억된 탭 크기 > cfg 크기
    let raw_url = expand(&raw_url);
    cfg.url = if raw_url.is_empty() {
        cfg.presets
            .first()
            .map(|p| p.url.clone())
            .unwrap_or_else(|| path_to_file_url(&idx_path))
    } else {
        to_url(&raw_url)
    };
    cfg.width = cfg.width.clamp(200, 3000);
    cfg.height = cfg.height.clamp(STRIP_H + 60, 3000);
    cfg
}
fn path_to_file_url(p: &str) -> String {
    format!("file:///{}", p.replace('\\', "/"))
}

// ── 탭별 마지막 창 크기 영속 (html_overlay_state.txt: "이름=WxH" 행) ──
fn state_path() -> Option<String> {
    mod_dir().map(|d| format!("{d}\\html_overlay_state.txt"))
}
fn load_size_memo() {
    let Some(p) = state_path() else { return };
    let Ok(s) = std::fs::read_to_string(&p) else { return };
    let mut memo = Vec::new();
    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((name, size)) = line.split_once('=') {
            if let Some((ws, hs)) = size.split_once('x') {
                if let (Ok(w), Ok(h)) = (ws.trim().parse::<i32>(), hs.trim().parse::<i32>()) {
                    if w >= 150 && h >= STRIP_H {
                        memo.push((name.trim().to_string(), w, h));
                    }
                }
            }
        }
    }
    *SIZE_MEMO.lock().unwrap_or_else(|e| e.into_inner()) = memo;
}
fn save_size_memo() {
    let Some(p) = state_path() else { return };
    let memo = SIZE_MEMO.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let mut out = String::from("# tfm2_html_overlay — 탭별 마지막 창 크기 (자동 저장, 삭제하면 cfg 크기로 복귀)\r\n");
    for (name, w, h) in &memo {
        out.push_str(&format!("{name}={w}x{h}\r\n"));
    }
    let _ = std::fs::write(&p, out);
}
/// 활성 탭의 크기를 기억(메모 갱신 + PRESETS 반영 + 파일 저장).
fn memo_size_for_active(w: i32, h: i32) {
    let active = ACTIVE_PRESET.load(Ordering::Relaxed);
    let name = {
        let mut presets = PRESETS.lock().unwrap_or_else(|e| e.into_inner());
        let Some(p) = presets.get_mut(active) else { return };
        p.w = w;
        p.h = h;
        p.name.clone()
    };
    {
        let mut memo = SIZE_MEMO.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(e) = memo.iter_mut().find(|(n, _, _)| *n == name) {
            e.1 = w;
            e.2 = h;
        } else {
            memo.push((name, w, h));
        }
    }
    save_size_memo();
}
/// 기억된 크기를 프리셋 목록에 덮어쓴다(기억 > cfg).
fn apply_size_memo(presets: &mut [Preset]) {
    let memo = SIZE_MEMO.lock().unwrap_or_else(|e| e.into_inner()).clone();
    for p in presets.iter_mut() {
        if let Some((_, w, h)) = memo.iter().find(|(n, _, _)| *n == p.name) {
            p.w = *w;
            p.h = *h;
        }
    }
}

// ===========================================================================
// COM — 최소 vtable FFI
// ===========================================================================
// 함수 시그니처(사용 슬롯만):
type FnCreateEnv =
    unsafe extern "system" fn(*const u16, *const u16, *mut c_void, *mut Handler) -> HRESULT;
type FnHr1 = unsafe extern "system" fn(*mut c_void) -> HRESULT; // (this)
type FnHrU32 = unsafe extern "system" fn(*mut c_void) -> u32; // AddRef/Release
type FnHrPtr = unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> HRESULT;
type FnHrBool = unsafe extern "system" fn(*mut c_void, i32) -> HRESULT;
type FnHrRect = unsafe extern "system" fn(*mut c_void, RECT) -> HRESULT;
type FnHrWstr = unsafe extern "system" fn(*mut c_void, *const u16) -> HRESULT;
type FnCreateCtl = unsafe extern "system" fn(*mut c_void, HWND, *mut Handler) -> HRESULT;

/// COM 객체의 idx 번째 vtable 슬롯을 T 로 캐스팅해 가져온다.
unsafe fn vslot<T: Copy>(obj: *mut c_void, idx: usize) -> T {
    let vtbl = *(obj as *const *const usize);
    let f = *vtbl.add(idx);
    std::mem::transmute_copy::<usize, T>(&f)
}
unsafe fn com_addref(obj: *mut c_void) -> u32 {
    vslot::<FnHrU32>(obj, 1)(obj)
}
unsafe fn com_release(obj: *mut c_void) -> u32 {
    vslot::<FnHrU32>(obj, 2)(obj)
}

// 우리가 구현하는 콜백 핸들러(IUnknown + Invoke(HRESULT, ptr)) — env/controller 완료 공용.
#[repr(C)]
struct Handler {
    vtbl: *const HandlerVtbl,
    refs: AtomicU32,
    kind: u32, // 0 = env 완료, 1 = controller 완료
    tag: i64,  // controller 완료용: 프리셋 index(keep) 또는 TAG_TRANSIENT
}
#[repr(C)]
struct HandlerVtbl {
    qi: unsafe extern "system" fn(*mut Handler, *const u8, *mut *mut c_void) -> HRESULT,
    addref: unsafe extern "system" fn(*mut Handler) -> u32,
    release: unsafe extern "system" fn(*mut Handler) -> u32,
    invoke: unsafe extern "system" fn(*mut Handler, HRESULT, *mut c_void) -> HRESULT,
}
static HANDLER_VTBL: HandlerVtbl =
    HandlerVtbl { qi: h_qi, addref: h_addref, release: h_release, invoke: h_invoke };

unsafe extern "system" fn h_qi(this: *mut Handler, _iid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    // 요청 IID 무관 자기 자신 반환(핸들러에 오는 QI 는 핸들러 IID/IAgileObject 뿐).
    if ppv.is_null() {
        return -2147467261; // E_POINTER
    }
    (*this).refs.fetch_add(1, Ordering::Relaxed);
    *ppv = this as *mut c_void;
    0
}
unsafe extern "system" fn h_addref(this: *mut Handler) -> u32 {
    (*this).refs.fetch_add(1, Ordering::Relaxed) + 1
}
unsafe extern "system" fn h_release(this: *mut Handler) -> u32 {
    let n = (*this).refs.fetch_sub(1, Ordering::Relaxed) - 1;
    if n == 0 {
        drop(Box::from_raw(this));
    }
    n
}
fn new_handler(kind: u32, tag: i64) -> *mut Handler {
    Box::into_raw(Box::new(Handler {
        vtbl: &HANDLER_VTBL,
        refs: AtomicU32::new(1),
        kind,
        tag,
    }))
}

unsafe extern "system" fn h_invoke(this: *mut Handler, hr: HRESULT, obj: *mut c_void) -> HRESULT {
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if hr < 0 || obj.is_null() {
            elog(&format!("webview 콜백 실패 kind={} hr={hr:#x}", (*this).kind));
            return;
        }
        match (*this).kind {
            0 => on_env_created(obj),
            1 => on_controller_created(obj, (*this).tag),
            _ => {}
        }
    }));
    if r.is_err() {
        elog("h_invoke panic (무시)");
    }
    0
}

/// env 생성 완료 → 시작 탭의 controller 생성 요청.
unsafe fn on_env_created(env: *mut c_void) {
    com_addref(env);
    ENV_PTR.store(env as usize, Ordering::Relaxed);
    let url = NAV_URL.lock().unwrap_or_else(|e| e.into_inner()).clone();
    show_tag(current_tag(), &url, true);
}

/// 현재 활성 탭의 태그: 활성 프리셋이 keep 이면 그 index, 아니면 공용(transient).
fn current_tag() -> i64 {
    let a = ACTIVE_PRESET.load(Ordering::Relaxed);
    let presets = PRESETS.lock().unwrap_or_else(|e| e.into_inner());
    match presets.get(a) {
        Some(p) if p.keep => a as i64,
        _ => TAG_TRANSIENT,
    }
}

fn client_bounds() -> RECT {
    let hwnd = MY_HWND.load(Ordering::Relaxed);
    let mut rc = RECT::default();
    if hwnd != 0 {
        unsafe { GetClientRect(hwnd, &mut rc) };
    }
    RECT { left: 0, top: STRIP_H, right: rc.right, bottom: rc.bottom }
}
unsafe fn ctl_set_bounds(ctl: usize, b: RECT) {
    if ctl != 0 {
        let f: FnHrRect = vslot(ctl as *mut c_void, 6); // put_Bounds
        let _ = f(ctl as *mut c_void, b);
    }
}
unsafe fn ctl_set_visible(ctl: usize, v: bool) {
    if ctl != 0 {
        let f: FnHrBool = vslot(ctl as *mut c_void, 4); // put_IsVisible
        let _ = f(ctl as *mut c_void, if v { 1 } else { 0 });
    }
}
unsafe fn navigate_wv(wv: usize, url: &str) {
    if wv == 0 || url.is_empty() {
        return;
    }
    let f: FnHrWstr = vslot(wv as *mut c_void, 5); // Navigate
    let w = wstr(url);
    let hr = f(wv as *mut c_void, w.as_ptr());
    log(&format!("Navigate('{url}') hr={hr:#x}"));
    if hr < 0 {
        elog(&format!("Navigate 실패 hr={hr:#x} url={url}"));
    }
}

/// 탭 표시의 단일 진입점: tag 슬롯을 보이게 하고 나머지는 숨긴다.
/// 슬롯이 없으면 비동기 생성 요청(완료 시 on_controller_created 가 이어받음).
/// force_nav = 이미 로드된 keep 탭도 다시 navigate (리로드/비-keep 탭 전환용).
fn show_tag(tag: i64, url: &str, force_nav: bool) {
    let found = {
        let ctls = CTLS.lock().unwrap_or_else(|e| e.into_inner());
        for s in ctls.iter() {
            if s.tag != tag {
                unsafe { ctl_set_visible(s.ctl, false) };
            }
        }
        ctls.iter().find(|s| s.tag == tag).map(|s| (s.ctl, s.wv))
    };
    if let Some((ctl, wv)) = found {
        unsafe {
            ctl_set_bounds(ctl, client_bounds());
            if !COLLAPSED.load(Ordering::Relaxed) {
                ctl_set_visible(ctl, true);
            }
            if force_nav {
                navigate_wv(wv, url);
            }
        }
        return;
    }
    // 신규 생성 (중복 요청 방지)
    let env = ENV_PTR.load(Ordering::Relaxed);
    if env == 0 {
        return; // env 준비 전 — on_env_created 가 시작 탭을 처리
    }
    {
        let mut pend = PENDING_TAGS.lock().unwrap_or_else(|e| e.into_inner());
        if pend.contains(&tag) {
            return;
        }
        pend.push(tag);
    }
    let hwnd = MY_HWND.load(Ordering::Relaxed);
    unsafe {
        // ICoreWebView2Environment vtable: 3 = CreateCoreWebView2Controller(HWND, handler)
        let f: FnCreateCtl = vslot(env as *mut c_void, 3);
        let hr = f(env as *mut c_void, hwnd, new_handler(1, tag));
        log(&format!("controller 생성 요청 tag={tag} hr={hr:#x}"));
        if hr < 0 {
            elog(&format!("CreateCoreWebView2Controller 실패 tag={tag} hr={hr:#x}"));
            PENDING_TAGS.lock().unwrap_or_else(|e| e.into_inner()).retain(|t| *t != tag);
        }
    }
}

/// controller 생성 완료 → 슬롯 등록·bounds·(활성이면 표시)·navigate.
unsafe fn on_controller_created(ctl: *mut c_void, tag: i64) {
    com_addref(ctl); // Invoke 인자는 빌린 참조 — 보관하려면 AddRef
    PENDING_TAGS.lock().unwrap_or_else(|e| e.into_inner()).retain(|t| *t != tag);
    // ICoreWebView2Controller vtable: 25 = get_CoreWebView2(out) (out 참조는 callee 가 AddRef)
    let mut wv: *mut c_void = std::ptr::null_mut();
    let f: FnHrPtr = vslot(ctl, 25);
    let hr = f(ctl, &mut wv);
    if hr < 0 || wv.is_null() {
        elog(&format!("get_CoreWebView2 실패 tag={tag} hr={hr:#x}"));
        return;
    }
    let url = if tag >= 0 {
        let presets = PRESETS.lock().unwrap_or_else(|e| e.into_inner());
        presets.get(tag as usize).map(|p| p.url.clone()).unwrap_or_default()
    } else {
        NAV_URL.lock().unwrap_or_else(|e| e.into_inner()).clone()
    };
    ctl_set_bounds(ctl as usize, client_bounds());
    let active = current_tag() == tag && !COLLAPSED.load(Ordering::Relaxed);
    ctl_set_visible(ctl as usize, active);
    CTLS.lock()
        .unwrap_or_else(|e| e.into_inner())
        .push(CtlSlot { tag, ctl: ctl as usize, wv: wv as usize });
    navigate_wv(wv as usize, &url);
    log(&format!("webview 준비 완료 tag={tag}"));
}

/// 창 클라이언트 크기에 맞춰 현재 탭 webview bounds 갱신(스트립 아래 전체).
fn update_webview_bounds() {
    let tag = current_tag();
    let b = client_bounds();
    let ctls = CTLS.lock().unwrap_or_else(|e| e.into_inner());
    for s in ctls.iter() {
        if s.tag == tag {
            unsafe { ctl_set_bounds(s.ctl, b) };
        }
    }
}
/// 현재 탭 webview 표시/숨김 (접기용).
fn webview_set_visible(v: bool) {
    let tag = current_tag();
    let ctls = CTLS.lock().unwrap_or_else(|e| e.into_inner());
    for s in ctls.iter() {
        if s.tag == tag {
            unsafe { ctl_set_visible(s.ctl, v) };
        }
    }
}
fn notify_moved() {
    let ctls = CTLS.lock().unwrap_or_else(|e| e.into_inner());
    for s in ctls.iter() {
        unsafe {
            // ICoreWebView2Controller vtable: 23 = NotifyParentWindowPositionChanged()
            let f: FnHr1 = vslot(s.ctl as *mut c_void, 23);
            let _ = f(s.ctl as *mut c_void);
        }
    }
}

// ===========================================================================
// 게임 메인 창 찾기 (이 프로세스의 최대 크기 가시 최상위 창)
// ===========================================================================
static ENUM_BEST: AtomicUsize = AtomicUsize::new(0);
static ENUM_BEST_AREA: AtomicIsize = AtomicIsize::new(0);
extern "system" fn enum_cb(hwnd: HWND, _l: isize) -> i32 {
    unsafe {
        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, &mut pid);
        if pid != GetCurrentProcessId() {
            return 1;
        }
        if IsWindowVisible(hwnd) == 0 || GetWindow(hwnd, GW_OWNER) != 0 {
            return 1;
        }
        if GetWindowTextLengthW(hwnd) == 0 {
            return 1;
        }
        let mut r = RECT::default();
        if GetWindowRect(hwnd, &mut r) == 0 {
            return 1;
        }
        let area = ((r.right - r.left) as isize) * ((r.bottom - r.top) as isize);
        if area > ENUM_BEST_AREA.load(Ordering::Relaxed) {
            ENUM_BEST_AREA.store(area, Ordering::Relaxed);
            ENUM_BEST.store(hwnd, Ordering::Relaxed);
        }
    }
    1
}
fn find_game_window() -> Option<HWND> {
    ENUM_BEST.store(0, Ordering::Relaxed);
    ENUM_BEST_AREA.store(0, Ordering::Relaxed);
    unsafe { EnumWindows(enum_cb, 0) };
    let h = ENUM_BEST.load(Ordering::Relaxed);
    let area = ENUM_BEST_AREA.load(Ordering::Relaxed);
    // 스플래시 오탐 방지: 최소 640x360 상당
    if h != 0 && area >= 640 * 360 {
        Some(h)
    } else {
        None
    }
}

// ===========================================================================
// 오버레이 창
// ===========================================================================
fn strip_buttons(client_w: i32) -> (RECT, RECT) {
    // (reload, collapse) — 스트립 오른쪽 끝에서부터 [↻][—]
    let collapse = RECT { left: client_w - BTN_W - 4, top: 2, right: client_w - 4, bottom: STRIP_H - 2 };
    let reload = RECT {
        left: client_w - BTN_W * 2 - 8,
        top: 2,
        right: client_w - BTN_W - 8,
        bottom: STRIP_H - 2,
    };
    (reload, collapse)
}
/// 프리셋 버튼 폭(대략적 글자폭 추정: 한글 14px/ASCII 8px + 좌우 여백)
fn preset_btn_w(name: &str) -> i32 {
    14 + name.chars().map(|c| if (c as u32) > 0x7F { 14 } else { 8 }).sum::<i32>()
}
/// 프리셋 버튼 rect 목록 — "≡" 마커(드래그 힌트) 오른쪽부터 나열.
fn preset_rects(presets: &[Preset]) -> Vec<RECT> {
    let mut x = 24;
    let mut out = Vec::with_capacity(presets.len());
    for p in presets {
        let w = preset_btn_w(&p.name);
        out.push(RECT { left: x, top: 2, right: x + w, bottom: STRIP_H - 2 });
        x += w + 4;
    }
    out
}
/// 접었을 때 바 너비 = 프리셋 전부 + 접기 버튼이 들어가는 폭.
fn collapsed_width() -> i32 {
    let presets = PRESETS.lock().unwrap_or_else(|e| e.into_inner());
    let pw: i32 = presets.iter().map(|p| preset_btn_w(&p.name) + 4).sum();
    (24 + pw + BTN_W + 12).max(150)
}
fn pt_in(r: &RECT, x: i32, y: i32) -> bool {
    x >= r.left && x < r.right && y >= r.top && y < r.bottom
}

fn toggle_collapse() {
    let hwnd = MY_HWND.load(Ordering::Relaxed);
    if hwnd == 0 {
        return;
    }
    let was = COLLAPSED.load(Ordering::Relaxed);
    unsafe {
        let mut wr = RECT::default();
        GetWindowRect(hwnd, &mut wr);
        if !was {
            // 접기: 현재 rect 저장 → ★좌상단 고정 작은 바(버튼 위치 유지)
            *SAVED_RECT.lock().unwrap_or_else(|e| e.into_inner()) =
                (wr.left, wr.top, wr.right - wr.left, wr.bottom - wr.top);
            COLLAPSED.store(true, Ordering::Relaxed);
            webview_set_visible(false);
            let cw = collapsed_width();
            SetWindowPos(
                hwnd,
                0,
                wr.left,
                wr.top,
                cw,
                STRIP_H + 12,
                SWP_NOZORDER | SWP_NOACTIVATE,
            );
        } else {
            let (_, _, w, h) = *SAVED_RECT.lock().unwrap_or_else(|e| e.into_inner());
            COLLAPSED.store(false, Ordering::Relaxed);
            // 현재(접힌) 바의 ★좌상단을 기준으로 펼침(사용자가 바를 옮겼을 수 있음)
            SetWindowPos(hwnd, 0, wr.left, wr.top, w, h, SWP_NOZORDER | SWP_NOACTIVATE);
            webview_set_visible(true);
        }
        InvalidateRect(hwnd, std::ptr::null(), 1);
    }
}

fn do_reload() {
    // cfg 다시 읽고 재-navigate(url/프리셋 변경 즉시 반영)
    let cfg = load_cfg();
    *PRESETS.lock().unwrap_or_else(|e| e.into_inner()) = cfg.presets.clone();
    // 활성 프리셋이 여전히 있으면 그 페이지를, 없으면 시작 url 을 리로드
    let active = ACTIVE_PRESET.load(Ordering::Relaxed);
    let url = cfg
        .presets
        .get(active)
        .map(|p| p.url.clone())
        .unwrap_or_else(|| cfg.url.clone());
    *NAV_URL.lock().unwrap_or_else(|e| e.into_inner()) = url.clone();
    show_tag(current_tag(), &url, true); // 리로드는 keep 탭도 강제 재-navigate
    let hwnd = MY_HWND.load(Ordering::Relaxed);
    if hwnd != 0 {
        unsafe { InvalidateRect(hwnd, std::ptr::null(), 1) };
    }
}

/// 프리셋 버튼 클릭: 해당 url 로 전환 + 프리셋 지정 크기로 리사이즈. 접힌 상태면 펼친다.
fn click_preset(idx: usize) {
    let preset = {
        let p = PRESETS.lock().unwrap_or_else(|e| e.into_inner());
        p.get(idx).cloned()
    };
    let Some(preset) = preset else { return };
    ACTIVE_PRESET.store(idx, Ordering::Relaxed);
    if COLLAPSED.load(Ordering::Relaxed) {
        toggle_collapse(); // 펼치면서 webview 재표시
    }
    let hwnd = MY_HWND.load(Ordering::Relaxed);
    // 프리셋 크기 적용 — ★좌상단 모서리 고정(우상단 고정은 탭 버튼 위치가 매번 이동해
    //   불편하다는 유저 피드백 08-11 — 프리셋 버튼이 왼쪽 정렬이라 좌상단 고정이 맞음)
    if hwnd != 0 && preset.w > 0 && preset.h > 0 {
        unsafe {
            let mut wr = RECT::default();
            if GetWindowRect(hwnd, &mut wr) != 0 {
                SetWindowPos(
                    hwnd,
                    0,
                    wr.left,
                    wr.top,
                    preset.w,
                    preset.h,
                    SWP_NOZORDER | SWP_NOACTIVATE,
                );
            }
        }
    }
    *NAV_URL.lock().unwrap_or_else(|e| e.into_inner()) = preset.url.clone();
    // keep 탭 = 상주 슬롯 재표시(재로딩 없음, 첫 진입만 로드) / 비-keep = 공용 슬롯에 새로 navigate
    let tag = if preset.keep { idx as i64 } else { TAG_TRANSIENT };
    show_tag(tag, &preset.url, !preset.keep);
    if hwnd != 0 {
        unsafe { InvalidateRect(hwnd, std::ptr::null(), 1) };
    }
}

// ===========================================================================
// Ctrl+O — 세이브 선택 → 대시보드 데이터 갱신 (refresh_meta_dashboard.ps1)
// ===========================================================================
/// 세이브 폴더 = %APPDATA%\TeamSamoyed\TeamfightManager2\data
fn save_data_dir() -> Option<String> {
    std::env::var("APPDATA").ok().map(|p| format!("{p}\\TeamSamoyed\\TeamfightManager2\\data"))
}
/// 파일 선택 대화상자(우리 STA 스레드 모달 — 게임 스레드는 무관).
fn pick_save_file() -> Option<String> {
    let hwnd = MY_HWND.load(Ordering::Relaxed);
    let filter = wstr("세이브 파일 (*.data)\u{0}*.data\u{0}모든 파일\u{0}*.*\u{0}");
    let title = wstr("대시보드에 쓸 세이브 파일 선택 (수정시각 최신 = 현재 진행 세이브)");
    let init_dir = save_data_dir().map(|d| wstr(&d));
    let mut buf = [0u16; 1024];
    let mut ofn: OPENFILENAMEW = unsafe { std::mem::zeroed() };
    ofn.lStructSize = std::mem::size_of::<OPENFILENAMEW>() as u32;
    ofn.hwndOwner = hwnd;
    ofn.lpstrFilter = filter.as_ptr();
    ofn.nFilterIndex = 1;
    ofn.lpstrFile = buf.as_mut_ptr();
    ofn.nMaxFile = buf.len() as u32;
    ofn.lpstrInitialDir = init_dir.as_ref().map(|d| d.as_ptr()).unwrap_or(std::ptr::null());
    ofn.lpstrTitle = title.as_ptr();
    // OFN_EXPLORER | OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST | OFN_NOCHANGEDIR
    ofn.Flags = 0x0008_0000 | 0x1000 | 0x800 | 0x8;
    let ok = unsafe { GetOpenFileNameW(&mut ofn) };
    if ok == 0 {
        return None; // 취소
    }
    let len = buf.iter().position(|&c| c == 0).unwrap_or(0);
    if len == 0 {
        return None;
    }
    Some(String::from_utf16_lossy(&buf[..len]))
}
/// 선택한 세이브로 갱신 파이프라인 실행(백그라운드 스레드) → 완료 시 WM_APP_REFRESH_DONE.
fn run_dashboard_refresh(save_path: String) {
    let Some(d) = mod_dir() else { return };
    let mods_root = std::path::Path::new(&d)
        .parent()
        .map(|p| p.display().to_string())
        .unwrap_or(d);
    let ps1 = format!("{mods_root}\\TFM2_Meta_Dashboard\\refresh_meta_dashboard.ps1");
    if !std::path::Path::new(&ps1).exists() {
        elog(&format!("갱신 스크립트 없음: {ps1}"));
        return;
    }
    if REFRESHING.swap(true, Ordering::Relaxed) {
        log("갱신 이미 진행 중 — 무시");
        return;
    }
    let hwnd = MY_HWND.load(Ordering::Relaxed);
    unsafe { InvalidateRect(hwnd, std::ptr::null(), 1) };
    log(&format!("세이브 갱신 시작: {save_path}"));
    std::thread::spawn(move || {
        let out = std::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-File",
                &ps1,
                "-SavePath",
                &save_path,
                "-NoPrompt",
                "-SkipLiveExporter",
            ])
            .output();
        let ok = match &out {
            Ok(o) if o.status.success() => true,
            Ok(o) => {
                let tail: String = String::from_utf8_lossy(&o.stdout)
                    .chars()
                    .rev()
                    .take(600)
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect();
                elog(&format!("갱신 실패 exit={:?} tail:\n{tail}", o.status.code()));
                false
            }
            Err(e) => {
                elog(&format!("갱신 프로세스 실행 실패: {e}"));
                false
            }
        };
        unsafe { PostMessageW(hwnd, WM_APP_REFRESH_DONE, ok as usize, 0) };
    });
}
/// 갱신 완료 → 대시보드 래퍼를 쓰는 keep 슬롯을 조용히 재-navigate(탭 전환 없음).
fn reload_dashboard_slots() {
    let presets = PRESETS.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let ctls: Vec<(i64, usize)> = CTLS
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .map(|s| (s.tag, s.wv))
        .collect();
    for (i, p) in presets.iter().enumerate() {
        if p.url.contains("TFM2_Meta_Dashboard") {
            for (tag, wv) in &ctls {
                if *tag == i as i64 {
                    unsafe { navigate_wv(*wv, &p.url) };
                }
            }
        }
    }
}
/// Ctrl+O 발화 시 포커스 게이트: 게임 창 또는 오버레이(웹뷰 포함)가 전면일 때만.
fn hotkey_scope_ok() -> bool {
    unsafe {
        let fore = GetForegroundWindow();
        if fore == 0 {
            return false;
        }
        let root = GetAncestor(fore, 2 /* GA_ROOT */);
        root == GAME_HWND.load(Ordering::Relaxed) || root == MY_HWND.load(Ordering::Relaxed)
    }
}

/// 게임 창 따라가기 + 게임 창 소멸 감지.
fn follow_game() {
    let game = GAME_HWND.load(Ordering::Relaxed);
    let hwnd = MY_HWND.load(Ordering::Relaxed);
    if game == 0 || hwnd == 0 {
        return;
    }
    unsafe {
        if IsWindow(game) == 0 {
            PostQuitMessage(0);
            return;
        }
        if IsIconic(game) != 0 {
            return; // owned 창은 OS 가 자동 숨김 — 위치 갱신만 쉼
        }
        let mut gr = RECT::default();
        if GetWindowRect(game, &mut gr) == 0 {
            return;
        }
        let cur = (gr.left, gr.top, gr.right, gr.bottom);
        let mut last = LAST_GAME.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(prev) = *last {
            let (dl, dt) = (cur.0 - prev.0, cur.1 - prev.1);
            if dl != 0 || dt != 0 {
                let mut mr = RECT::default();
                GetWindowRect(hwnd, &mut mr);
                SetWindowPos(
                    hwnd,
                    0,
                    mr.left + dl,
                    mr.top + dt,
                    0,
                    0,
                    SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
                );
            }
        }
        *last = Some(cur);
    }
}

unsafe fn paint_strip(hwnd: HWND) {
    let mut ps: PAINTSTRUCT = std::mem::zeroed();
    let hdc = BeginPaint(hwnd, &mut ps);
    if hdc != 0 {
        let mut rc = RECT::default();
        GetClientRect(hwnd, &mut rc);
        let strip = RECT { left: 0, top: 0, right: rc.right, bottom: STRIP_H.min(rc.bottom) };
        // 정적 브러시 캐시(GDI 누수 방지)
        static BRUSH: AtomicUsize = AtomicUsize::new(0);
        let mut b = BRUSH.load(Ordering::Relaxed);
        if b == 0 {
            b = CreateSolidBrush(0x0026_201E); // rgb(30,32,38)
            BRUSH.store(b, Ordering::Relaxed);
        }
        FillRect(hdc, &strip, b);
        SetBkMode(hdc, TRANSPARENT_BK);
        let (f_text, f_sym) = strip_fonts();
        let old_font = SelectObject(hdc, f_text);
        // 드래그 힌트 마커
        SetTextColor(hdc, 0x0080_8080);
        let title = wstr("≡");
        TextOutW(hdc, 8, 6, title.as_ptr(), (title.len() - 1) as i32);
        // 프리셋 버튼들 (활성 = 노란색)
        static BTN_BRUSH: AtomicUsize = AtomicUsize::new(0);
        let mut bb = BTN_BRUSH.load(Ordering::Relaxed);
        if bb == 0 {
            bb = CreateSolidBrush(0x003A_322E); // rgb(46,50,58) 살짝 밝은 버튼 배경
            BTN_BRUSH.store(bb, Ordering::Relaxed);
        }
        let presets = PRESETS.lock().unwrap_or_else(|e| e.into_inner()).clone();
        let rects = preset_rects(&presets);
        let active = ACTIVE_PRESET.load(Ordering::Relaxed);
        for (i, (p, r)) in presets.iter().zip(rects.iter()).enumerate() {
            FillRect(hdc, r, bb);
            SetTextColor(hdc, if i == active { 0x003E_D2FF } else { 0x00C8_C8C8 });
            let t = wstr(&p.name);
            TextOutW(hdc, r.left + 7, 6, t.as_ptr(), (t.len() - 1) as i32);
        }
        if REFRESHING.load(Ordering::Relaxed) {
            let x_end = rects.last().map(|r| r.right).unwrap_or(24) + 10;
            SetTextColor(hdc, 0x003E_D2FF);
            let t = wstr("세이브 갱신중…");
            TextOutW(hdc, x_end, 6, t.as_ptr(), (t.len() - 1) as i32);
        }
        let (r_re, r_co) = strip_buttons(rc.right);
        SetTextColor(hdc, 0x003E_D2FF);
        SelectObject(hdc, f_sym); // 기호는 Segoe UI Symbol (기본 폰트엔 글리프 없음)
        if !COLLAPSED.load(Ordering::Relaxed) {
            let re = wstr("↻");
            TextOutW(hdc, r_re.left + 7, 5, re.as_ptr(), (re.len() - 1) as i32);
        }
        let co = if COLLAPSED.load(Ordering::Relaxed) { wstr("❐") } else { wstr("—") };
        TextOutW(hdc, r_co.left + 7, 5, co.as_ptr(), (co.len() - 1) as i32);
        SelectObject(hdc, old_font);
        EndPaint(hwnd, &ps);
    } else {
        EndPaint(hwnd, &ps);
    }
}

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, w: usize, l: isize) -> isize {
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| wnd_proc_inner(hwnd, msg, w, l)));
    match r {
        Ok(v) => v,
        Err(_) => {
            elog(&format!("wnd_proc panic msg={msg:#x} (기본 처리로 폴백)"));
            unsafe { DefWindowProcW(hwnd, msg, w, l) }
        }
    }
}
fn wnd_proc_inner(hwnd: HWND, msg: u32, w: usize, l: isize) -> isize {
    unsafe {
        match msg {
            WM_CREATE => {
                SetTimer(hwnd, TIMER_FOLLOW, 200, 0);
                0
            }
            WM_NCHITTEST => {
                let hit = DefWindowProcW(hwnd, msg, w, l);
                if hit == HTCLIENT {
                    let x = (l & 0xFFFF) as i16 as i32;
                    let y = ((l >> 16) & 0xFFFF) as i16 as i32;
                    let mut pt = [x, y];
                    ScreenToClient(hwnd, &mut pt);
                    if pt[1] < STRIP_H {
                        let mut rc = RECT::default();
                        GetClientRect(hwnd, &mut rc);
                        let (r_re, r_co) = strip_buttons(rc.right);
                        if pt_in(&r_re, pt[0], pt[1]) || pt_in(&r_co, pt[0], pt[1]) {
                            return HTCLIENT; // 버튼은 클릭 수신
                        }
                        let presets = PRESETS.lock().unwrap_or_else(|e| e.into_inner()).clone();
                        if preset_rects(&presets).iter().any(|r| pt_in(r, pt[0], pt[1])) {
                            return HTCLIENT;
                        }
                        return HTCAPTION; // 나머지 스트립 = 드래그
                    }
                }
                hit
            }
            WM_LBUTTONUP => {
                let x = (l & 0xFFFF) as i16 as i32;
                let y = ((l >> 16) & 0xFFFF) as i16 as i32;
                let mut rc = RECT::default();
                GetClientRect(hwnd, &mut rc);
                let (r_re, r_co) = strip_buttons(rc.right);
                if pt_in(&r_co, x, y) {
                    toggle_collapse();
                } else if !COLLAPSED.load(Ordering::Relaxed) && pt_in(&r_re, x, y) {
                    do_reload();
                } else {
                    let presets = PRESETS.lock().unwrap_or_else(|e| e.into_inner()).clone();
                    if let Some(i) =
                        preset_rects(&presets).iter().position(|r| pt_in(r, x, y))
                    {
                        click_preset(i);
                    }
                }
                0
            }
            WM_HOTKEY => {
                match w {
                    HOTKEY_COLLAPSE => toggle_collapse(),
                    HOTKEY_OPEN_SAVE => {
                        // Ctrl+O 는 시스템 전역 등록이라 게임/오버레이 전면일 때만 반응
                        if hotkey_scope_ok() && !REFRESHING.load(Ordering::Relaxed) {
                            if let Some(p) = pick_save_file() {
                                run_dashboard_refresh(p);
                            }
                        }
                    }
                    _ => {}
                }
                0
            }
            WM_APP_REFRESH_DONE => {
                REFRESHING.store(false, Ordering::Relaxed);
                if w == 1 {
                    log("세이브 갱신 완료 — 대시보드 탭 재로드");
                    reload_dashboard_slots();
                }
                InvalidateRect(hwnd, std::ptr::null(), 1);
                0
            }
            WM_SIZE => {
                if !COLLAPSED.load(Ordering::Relaxed) {
                    update_webview_bounds();
                }
                InvalidateRect(hwnd, std::ptr::null(), 1);
                0
            }
            WM_MOVE => {
                notify_moved();
                0
            }
            WM_EXITSIZEMOVE => {
                // 사용자 드래그/리사이즈 종료 → 활성 탭에 현재 크기 기억
                if !COLLAPSED.load(Ordering::Relaxed) {
                    let mut wr = RECT::default();
                    if GetWindowRect(hwnd, &mut wr) != 0 {
                        let (w, h) = (wr.right - wr.left, wr.bottom - wr.top);
                        if w >= 150 && h > STRIP_H + 12 {
                            memo_size_for_active(w, h);
                        }
                    }
                }
                0
            }
            WM_TIMER => {
                if w == TIMER_FOLLOW {
                    follow_game();
                }
                0
            }
            WM_PAINT => {
                paint_strip(hwnd);
                0
            }
            WM_ERASEBKGND => 1,
            WM_GETMINMAXINFO => {
                let mmi = l as *mut MINMAXINFO;
                if !mmi.is_null() {
                    (*mmi).ptMinTrackSize = [150, STRIP_H + 10];
                }
                0
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcW(hwnd, msg, w, l),
        }
    }
}

// ===========================================================================
// 오버레이 스레드 본문
// ===========================================================================
fn overlay_thread() {
    unsafe {
        let hr = CoInitializeEx(0, 0x2 /* STA */);
        if hr < 0 {
            elog(&format!("CoInitializeEx 실패 hr={hr:#x}"));
            return;
        }
    }
    // 게임 메인 창 대기(부팅 직후엔 아직 없을 수 있음)
    let game = loop {
        if let Some(h) = find_game_window() {
            break h;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    };
    GAME_HWND.store(game, Ordering::Relaxed);
    log(&format!("게임 창 발견 hwnd={game:#x}"));

    load_size_memo();
    let cfg = load_cfg();
    *NAV_URL.lock().unwrap_or_else(|e| e.into_inner()) = cfg.url.clone();
    *PRESETS.lock().unwrap_or_else(|e| e.into_inner()) = cfg.presets.clone();
    ACTIVE_PRESET.store(
        cfg.presets.iter().position(|p| p.url == cfg.url).unwrap_or(usize::MAX),
        Ordering::Relaxed,
    );

    // WebView2Loader.dll 로드 (모드 폴더 동봉본)
    let Some(d) = mod_dir() else {
        elog("mod_dir 도출 실패");
        return;
    };
    // ⚠파일명이 .dll 이면 게임 모드 로더가 이것도 모드로 간주해 로드 시도
    //   → "API version symbol not found" 에러 + 모드 강제 비활성(0.5.4 실사고).
    //   .dat 로 개명해 로더 스캔을 피한다(LoadLibraryW 는 전체경로면 확장자 무관).
    let loader_path = format!("{d}\\WebView2Loader.dat");
    let hlib = unsafe { LoadLibraryW(wstr(&loader_path).as_ptr()) };
    if hlib == 0 {
        elog(&format!(
            "WebView2Loader.dll 로드 실패 (GetLastError={}) — {loader_path} 존재 확인",
            unsafe { GetLastError() }
        ));
        return;
    }
    let create_env: FnCreateEnv = unsafe {
        let p = GetProcAddress(hlib, b"CreateCoreWebView2EnvironmentWithOptions\0".as_ptr());
        if p == 0 {
            elog("CreateCoreWebView2EnvironmentWithOptions export 없음");
            return;
        }
        std::mem::transmute(p)
    };

    // 창 생성 (게임 창 오너 = 항상 게임 창 위)
    let hinst = unsafe { GetModuleHandleW(std::ptr::null()) };
    let cls = wstr("tfm2_html_overlay_wnd");
    let wc = WNDCLASSW {
        style: 0x0003, // CS_HREDRAW|CS_VREDRAW
        lpfnWndProc: wnd_proc as usize,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: hinst,
        hIcon: 0,
        hCursor: unsafe { LoadCursorW(0, 32512) },
        hbrBackground: 0,
        lpszMenuName: std::ptr::null(),
        lpszClassName: cls.as_ptr(),
    };
    unsafe { RegisterClassW(&wc) };

    // 초기 위치 = 게임 창 우상단 안쪽. 시작 페이지가 프리셋이면 그 프리셋 크기 우선.
    let (mut init_w, mut init_h) = (cfg.width, cfg.height);
    if let Some(p) = cfg.presets.iter().find(|p| p.url == cfg.url) {
        if p.w > 0 && p.h > 0 {
            init_w = p.w;
            init_h = p.h;
        }
    }
    let mut gr = RECT::default();
    unsafe { GetWindowRect(game, &mut gr) };
    let x = gr.right - init_w - 24;
    let y = gr.top + 60;
    let hwnd = unsafe {
        CreateWindowExW(
            WS_EX_TOOLWINDOW,
            cls.as_ptr(),
            wstr("HTML Overlay").as_ptr(),
            WS_POPUP | WS_THICKFRAME | WS_CLIPCHILDREN,
            x,
            y,
            init_w,
            init_h,
            game,
            0,
            hinst,
            0,
        )
    };
    if hwnd == 0 {
        elog(&format!("CreateWindowExW 실패 (GetLastError={})", unsafe { GetLastError() }));
        return;
    }
    MY_HWND.store(hwnd, Ordering::Relaxed);
    unsafe {
        ShowWindow(hwnd, SW_SHOWNOACTIVATE);
        RegisterHotKey(hwnd, HOTKEY_COLLAPSE as i32, MOD_CONTROL, VK_F10);
        RegisterHotKey(hwnd, HOTKEY_OPEN_SAVE as i32, MOD_CONTROL, 0x4F /* O */);
    }

    // WebView2 환경 생성 (프로필 데이터 = %LOCALAPPDATA% — 게임 폴더 오염 방지)
    let data_dir = std::env::var("LOCALAPPDATA")
        .map(|p| format!("{p}\\tfm2_html_overlay\\wv2_data"))
        .unwrap_or_else(|_| format!("{d}\\wv2_data"));
    let _ = std::fs::create_dir_all(&data_dir);
    let hr = unsafe {
        create_env(std::ptr::null(), wstr(&data_dir).as_ptr(), std::ptr::null_mut(), new_handler(0, 0))
    };
    if hr < 0 {
        elog(&format!("CreateCoreWebView2EnvironmentWithOptions 실패 hr={hr:#x} (런타임 미설치?)"));
        // 창은 살려둔다(스트립에 상태라도 보이게)
    } else {
        log(&format!("env 요청 OK, data={data_dir}"));
    }

    if cfg.start_collapsed {
        toggle_collapse();
    }

    // 메시지 루프
    let mut msg: MSG = unsafe { std::mem::zeroed() };
    loop {
        let r = unsafe { GetMessageW(&mut msg, 0, 0, 0) };
        if r <= 0 {
            break;
        }
        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    log("메시지 루프 종료");
}

// ===========================================================================
// 모드 등록
// ===========================================================================
struct HtmlOverlayExt;
impl ModExtension for HtmlOverlayExt {
    fn post_update(&self, _scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        if !STARTED.swap(true, Ordering::Relaxed) {
            std::thread::Builder::new()
                .name("tfm2_html_overlay".into())
                .spawn(|| {
                    let r = std::panic::catch_unwind(overlay_thread);
                    if r.is_err() {
                        elog("overlay_thread panic — 오버레이 비활성");
                    }
                })
                .ok();
        }
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(HtmlOverlayExt);
    reg
}
declare_mod!(init);
