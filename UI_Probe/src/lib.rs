// UI_Probe v2 — 화면에 보이는 UI 트리 전체 덤프 probe
// ===========================================================================
// 트리거:
//   F8  → 현재 화면(ui.root)의 노드 트리를 통째로 ui_dump.txt 에 append.
//   F9  → 위와 동일 + 각 노드 runner 인스턴스의 type_name 전체, 자동 스캔으로
//         찾은 모든 (offset, 문자열) 목록, raw hex 까지.
//
// v2 변경: LabelRunner +352 같은 "고정 오프셋 가설"을 버림. SDK 버전마다 깨지므로,
//   runner 인스턴스 메모리(0~SCAN_OFF)를 8바이트씩 훑어 힙 포인터를 찾고, 그 너머가
//   문자열이면 (offset, 내용) 으로 자동 추출. 오프셋을 몰라도 텍스트/에셋경로가 잡힘.
//   문자열은 null/제어문자에서 끊어 끝 쓰레기 제거 (len 필드 안 믿음). 한글(UTF-8) 통과.
//
// 출력 파일: 이 DLL 과 같은 폴더의 ui_dump.txt (회차/타임스탬프 구분선으로 append).
//
// 안전: 읽기 전용. 코드로 노드 push 안 함. 키 폴링이라 filter_handler 교체와 무관.
//       모든 메모리 역참조는 looks_heap + IsBadReadPtr 로 가드 → 못 멈춤.
//
// 빌드: nightly-2026-06-11, mod_api API_VERSION (0,7). UI_Probe\src\lib.rs.
//       역할 끝나면 게임에서 비활성화/삭제.
// ===========================================================================

use mod_api::*;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "UI_Probe";

// 폭주 방지 한도
const MAX_DEPTH: usize = 60;
const MAX_NODES: usize = 12000;
const STR_CAP: usize = 256; // 문자열 최대 읽기 길이
const SCAN_OFF: usize = 2048; // runner 인스턴스 문자열 자동 스캔 범위(DropdownRunner 큼)
const MIN_STR: usize = 3; // 이 길이 이상만 의미있는 문자열로 채택
const HEX_BYTES: usize = 96; // F9 일 때 runner 당 hex 덤프 바이트 수

// 트리거 키 (Virtual-Key Code)
const VK_F8: i32 = 0x77;
const VK_F9: i32 = 0x78;

static PREV_F8: AtomicBool = AtomicBool::new(false);
static PREV_F9: AtomicBool = AtomicBool::new(false);
static DUMP_COUNT: AtomicU32 = AtomicU32::new(0);
static BOOTED: AtomicBool = AtomicBool::new(false);

// ----------------------------- WinAPI -------------------------------------
type HMODULE = isize;
type DWORD = u32;
type BOOL = i32;

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleExW(f: DWORD, name: *const u16, h: *mut HMODULE) -> BOOL;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, n: DWORD) -> DWORD;
    fn IsBadReadPtr(lp: usize, ucb: usize) -> BOOL;
}
#[link(name = "user32")]
extern "system" {
    fn GetAsyncKeyState(v: i32) -> i16;
}

#[inline]
fn key_down(vk: i32) -> bool {
    (unsafe { GetAsyncKeyState(vk) } as u16 & 0x8000) != 0
}

/// rising edge (안 눌림 → 눌림 순간에만 true)
fn key_pressed(vk: i32, prev: &AtomicBool) -> bool {
    let now = key_down(vk);
    let was = prev.swap(now, Ordering::Relaxed);
    now && !was
}

// --------------------------- 파일/로그 유틸 --------------------------------
fn dll_path() -> Option<PathBuf> {
    unsafe {
        let addr = dll_path as *const () as usize;
        let mut h: HMODULE = 0;
        if GetModuleHandleExW(0x4 | 0x2, addr as *const u16, &mut h) == 0 || h == 0 {
            return None;
        }
        let mut buf = [0u16; 4096];
        let len = GetModuleFileNameW(h, buf.as_mut_ptr(), buf.len() as DWORD);
        if len == 0 {
            return None;
        }
        Some(PathBuf::from(String::from_utf16_lossy(&buf[..len as usize])))
    }
}

fn append_log(name: &str, content: &str) {
    use std::io::Write;
    let Some(p) = dll_path().and_then(|p| p.parent().map(|d| d.join(name))) else {
        return;
    };
    if let Some(parent) = p.parent() {
        let _ = fs::create_dir_all(Path::new(parent));
    }
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(p) {
        let _ = f.write_all(content.as_bytes());
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

// --------------------------- 메모리 안전 읽기 ------------------------------
#[inline]
unsafe fn rd_u64(p: usize) -> u64 {
    std::ptr::read_unaligned(p as *const u64)
}

/// 힙 포인터다움 + 실제 읽기 가능 여부
fn readable(p: usize, n: usize) -> bool {
    p >= 0x10000
        && (p as u64) < 0x0000_8000_0000_0000
        && unsafe { IsBadReadPtr(p, n) } == 0
}

fn looks_heap(v: u64) -> bool {
    v & 0x7 == 0
        && v >= 0x10000
        && v < 0x0000_8000_0000_0000
        && (v & 0xffff) != 0
        && (v & 0xffff_ffff) != 0xffff_ffff
}

/// ptr 에서 문자열을 안전하게 읽음. max 길이까지 읽되 null/제어문자에서 끊음.
/// 0x80+ 바이트는 UTF-8(한글 등)로 보고 통과. STR_CAP 절대 상한.
unsafe fn read_cstr(ptr: u64, max: usize) -> String {
    if !looks_heap(ptr) || !readable(ptr as usize, 1) {
        return String::new();
    }
    let base = ptr as usize;
    let cap = max.min(STR_CAP);
    let mut v: Vec<u8> = Vec::new();
    for i in 0..cap {
        if !readable(base + i, 1) {
            break;
        }
        let b = *((base + i) as *const u8);
        if b == 0 {
            break; // null 종료
        }
        if b < 0x20 && b != b'\n' && b != b'\t' {
            break; // 제어문자 종료
        }
        v.push(b);
    }
    String::from_utf8_lossy(&v).into_owned()
}

/// 문자열다움 판정: 출력가능 문자 비율이 높은가 (잡음 포인터 걸러내기)
fn looks_textlike(s: &str) -> bool {
    if s.len() < MIN_STR {
        return false;
    }
    let printable = s
        .chars()
        .filter(|c| c.is_alphanumeric() || "/_.-#: ".contains(*c) || (*c as u32) > 0x7f)
        .count();
    printable * 100 >= s.len() * 70
}

/// runner 인스턴스 메모리(dp..dp+SCAN_OFF)를 8바이트씩 훑어, 힙 포인터가 가리키는
/// 곳이 문자열이면 (offset, 문자열). 포인터 ±8 에 있는 길이 필드로 정확히 자름
/// (인터닝으로 null 없이 붙은 문자열의 끝 쓰레기 제거).
unsafe fn scan_strings(dp: usize) -> Vec<(usize, String)> {
    let mut out: Vec<(usize, String)> = Vec::new();
    if dp == 0 {
        return out;
    }
    let mut off = 0usize;
    while off + 8 <= SCAN_OFF {
        if readable(dp + off, 8) {
            let w = rd_u64(dp + off);
            if looks_heap(w) {
                // 길이 힌트: 포인터 뒤(+8) 우선, 없으면 앞(-8). 1..=STR_CAP 범위만 신뢰.
                let mut hint = STR_CAP;
                if readable(dp + off + 8, 8) {
                    let h = rd_u64(dp + off + 8);
                    if h >= 1 && h <= STR_CAP as u64 {
                        hint = h as usize;
                    }
                }
                if hint == STR_CAP && off >= 8 && readable(dp + off - 8, 8) {
                    let h = rd_u64(dp + off - 8);
                    if h >= 1 && h <= STR_CAP as u64 {
                        hint = h as usize;
                    }
                }
                let s = read_cstr(w, hint);
                if looks_textlike(&s) {
                    out.push((off, s));
                }
            }
        }
        off += 8;
    }
    out
}

// --------------------------- runner 식별/읽기 ------------------------------
/// runner 인스턴스 데이터 시작 포인터 (transmute 로 추출)
unsafe fn runner_data_ptr(n: &Node) -> usize {
    use std::any::Any;
    let any: &dyn Any = n.runner.as_any();
    let parts: [usize; 2] = std::mem::transmute::<*const dyn Any, [usize; 2]>(any as *const dyn Any);
    parts[0]
}

/// type_name 을 짧은 라벨로
fn runner_kind(n: &Node) -> &'static str {
    let t = n.runner.type_name();
    if t.contains("LabelRunner") {
        "Label"
    } else if t.contains("ImageRunner") {
        "Image"
    } else if t.contains("ColorRunner") {
        "Color"
    } else if t.contains("EmptyRunner") {
        "Empty"
    } else {
        "?"
    }
}

/// runner 인스턴스 raw hex (미지 오프셋 탐색용)
unsafe fn runner_hex(dp: usize, count: usize) -> String {
    if !readable(dp, count) {
        return "<unreadable>".into();
    }
    let mut s = String::with_capacity(count * 3);
    for i in 0..count {
        let b = *((dp + i) as *const u8);
        let _ = write!(s, "{:02x}", b);
        if (i + 1) % 8 == 0 {
            s.push(' ');
        }
    }
    s
}

// ------------------------------ 트리 순회 ----------------------------------
struct Stats {
    nodes: usize,
    label: usize,
    image: usize,
    color: usize,
    empty: usize,
    other: usize,
    visible: usize,
    clickable: Vec<String>, // visible leaf 들의 full path
}

impl Stats {
    fn new() -> Self {
        Stats {
            nodes: 0,
            label: 0,
            image: 0,
            color: 0,
            empty: 0,
            other: 0,
            visible: 0,
            clickable: Vec::new(),
        }
    }
    fn bump_kind(&mut self, kind: &str) {
        match kind {
            "Label" => self.label += 1,
            "Image" => self.image += 1,
            "Color" => self.color += 1,
            "Empty" => self.empty += 1,
            _ => self.other += 1,
        }
    }
}

/// 한 노드를 한 줄로 + 자식 재귀
fn walk(n: &Node, depth: usize, path: &str, with_hex: bool, out: &mut String, st: &mut Stats) {
    if st.nodes >= MAX_NODES || depth > MAX_DEPTH {
        return;
    }
    st.nodes += 1;

    let id = n.id.as_str();
    let full = if path.is_empty() {
        id.to_string()
    } else {
        format!("{path}.{id}")
    };

    let kind = runner_kind(n);
    st.bump_kind(kind);
    let vis = n.visible;
    if vis {
        st.visible += 1;
    }

    // 플래그/사각형
    let flags = format!(
        "{}{}",
        if vis { "V" } else { "-" },
        if n.disabled { "D" } else { "-" }
    );
    let r = &n.rect;
    let rect = format!("({:.0},{:.0} {:.0}x{:.0})", r.x, r.y, r.w, r.h);

    // ? runner 는 type_name 끝 토큰을 붙여 정체 노출 (예: ?Dropdown, ?Button)
    let kind_disp = if kind == "?" {
        let t = n.runner.type_name();
        let tail = t.rsplit("::").next().unwrap_or(t);
        format!("?{}", tail.trim_end_matches("Runner"))
    } else {
        kind.to_string()
    };

    // 내용: 고정 오프셋 가설 없이 인스턴스 메모리 자동 스캔 → (off, str) 들
    let dp = unsafe { runner_data_ptr(n) };
    let found = unsafe { scan_strings(dp) };
    // inline 대표 문자열: asset 경로 우선, 없으면 가장 긴 것
    let rep = found
        .iter()
        .find(|(_, s)| s.contains("asset/") || s.contains('?'))
        .or_else(|| found.iter().max_by_key(|(_, s)| s.len()))
        .map(|(off, s)| format!(" str[+{off}]=\"{}\"", s.replace('\n', "\\n")))
        .unwrap_or_default();

    let indent = "  ".repeat(depth);
    let child_n = n.child.len();
    let _ = writeln!(
        out,
        "{indent}{flags} {rect} {kind_disp}#{child_n} <{id}>{rep}   @{full}"
    );

    // F9: type_name 전체 + 스캔된 모든 문자열 + raw hex
    if with_hex {
        let _ = writeln!(out, "{indent}     type: {}", n.runner.type_name());
        for (off, s) in &found {
            let _ = writeln!(out, "{indent}     +{off} \"{}\"", s.replace('\n', "\\n"));
        }
        let hx = unsafe { runner_hex(dp, HEX_BYTES) };
        let _ = writeln!(out, "{indent}     hex@{dp:#x}: {hx}");
    }

    // 클릭 후보: 보이고 자식 없는(말단) 노드 = 버튼/라벨일 확률 높음
    if vis && child_n == 0 {
        st.clickable.push(full.clone());
    }

    for c in n.child.iter() {
        walk(c, depth + 1, &full, with_hex, out, st);
    }
}

/// 전체 트리 → ui_dump.txt 에 append
fn dump(ui: &GameUI, scene_name: &str, with_hex: bool) {
    let seq = DUMP_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    let ms = now_ms();

    let mut body = String::with_capacity(64 * 1024);
    let mut st = Stats::new();
    walk(&ui.root, 0, "", with_hex, &mut body, &mut st);

    let mut head = String::new();
    let _ = writeln!(head, "\n{}", "=".repeat(78));
    let _ = writeln!(
        head,
        "# DUMP #{seq}  t={ms}ms  scene={scene_name}  hex={}",
        with_hex
    );
    let _ = writeln!(
        head,
        "# nodes={} visible={} | Label={} Image={} Color={} Empty={} ?={}",
        st.nodes, st.visible, st.label, st.image, st.color, st.empty, st.other
    );
    let _ = writeln!(head, "{}", "=".repeat(78));

    // 클릭 후보 섹션 (filter_handler path 매칭에 그대로 사용)
    let mut tail = String::new();
    let _ = writeln!(tail, "\n--- CLICK TARGETS (visible leaves, {} 개) ---", st.clickable.len());
    for p in &st.clickable {
        let _ = writeln!(tail, "  {p}");
    }

    append_log("ui_dump.txt", &head);
    append_log("ui_dump.txt", &body);
    append_log("ui_dump.txt", &tail);
}

// ------------------------------ 모드 본체 ----------------------------------
struct UiProbe;

impl ModExtension for UiProbe {
    fn post_update(&self, scene: &mut Scene, ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        if !BOOTED.swap(true, Ordering::Relaxed) {
            append_log(
                "ui_dump.txt",
                &format!("\n[UI_Probe booted t={}ms]  F8=dump  F9=dump+hex\n", now_ms()),
            );
        }

        // Scene 은 #[non_exhaustive] → InGame 만 이름 식별, 나머지는 표시만
        let scene_name = match scene {
            Scene::InGame { .. } => "InGame",
            _ => "other",
        };

        if key_pressed(VK_F8, &PREV_F8) {
            dump(ui, scene_name, false);
        }
        if key_pressed(VK_F9, &PREV_F9) {
            dump(ui, scene_name, true);
        }
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(UiProbe);
    reg
}

declare_mod!(init);