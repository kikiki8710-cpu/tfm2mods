//! inject.rs — 게임 UI 템플릿에 내 노드 조각을 주입(로더 훅 + 게임 파서).
//! ===========================================================================
//! 방식 = tfm2_draft_overlay 참조구현. 에셋 게터(로더)를 체인 후킹 → 로드된 템플릿에
//! 마커 노드가 있으면 내 .ui 조각을 게임 PARSER 로 파싱해 그 템플릿의 child 배열에 append.
//! .ui 문자열은 컴파일타임 임베드 → 에셋 등록·override 불요.
//!
//! Stage 1: 환경설정 게임플레이 탭(contents, 마커=current_database_edit)에 '포지션 제한' 행.
//! ===========================================================================
#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::config;

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(n: *const u16) -> usize;
    fn VirtualAlloc(addr: usize, size: usize, ty: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, prot: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(h: usize, addr: usize, size: usize) -> i32;
    fn GetCurrentProcess() -> usize;
}
const MEM_CR: u32 = 0x1000 | 0x2000;
const RWX: u32 = 0x40;

// 공용 4심볼(0.5.6, MIGRATION §7.6 §2). RVA=abs−base.
const LOADER_RVA: usize = 0x2ea930; // 에셋 게터 FUN(am, path, len) -> NodeTemplate*
const PARSER_RVA: usize = 0x1ab310; // .ui 텍스트 → NodeTemplate
const ALLOC_RVA: usize = 0x2ab4010; // 게임 alloc impl(_, flags, size) -> ptr
const NT_SIZE: usize = 0x90; // NodeTemplate stride
// 템플릿 오프셋: id ptr+0x08 / id len+0x10 / child cap+0x48 / child ptr+0x50 / child len+0x58

const ROW_UI: &str = include_str!("../assets/pos_lock_row.ui");
const POPUP_UI: &str = include_str!("../assets/pos_lock_popup.ui");
/// 스왑 확정 버튼 비활성 사유 툴팁 — **밴픽 레이아웃 루트의 마지막 자식**으로 주입한다.
///   왜 루트 마지막인가: 게임의 `#coach_dialogue_button_tooltip` 은 `.ui` 상 `#swap` 보다
///   **먼저** 선언돼 있어 스왑 패널 뒤에 깔린다(z 순서 = 자식 배열 순서). 재사용 불가.
const SWAPTIP_UI: &str = include_str!("../assets/pos_lock_swaptip.ui");
/// 팝업 주입 스위치(문제 격리용). 검정화면 원인 규명 중 false 로 끌 수 있음.
const POPUP_INJECT: bool = true;

type LoaderFn = extern "win64" fn(usize, *const u8, usize) -> usize;
type ParserFn = extern "win64" fn(*mut u8, *const u8, usize);
type AllocFn = extern "win64" fn(usize, usize, usize) -> usize;

static BASE: AtomicUsize = AtomicUsize::new(0);
static TRAMP: AtomicUsize = AtomicUsize::new(0);
static INSTALLED: AtomicBool = AtomicBool::new(false);
pub static INJECTED_ROW: AtomicBool = AtomicBool::new(false);
pub static INJECTED_POPUP: AtomicBool = AtomicBool::new(false);
pub static INJECTED_SWAPTIP: AtomicBool = AtomicBool::new(false);
/// 시작 지연 계측 — try_inject 호출 수 / 누적 마이크로초 / 총계 1회 보고 여부.
static SCAN_N: AtomicUsize = AtomicUsize::new(0);
static SCAN_US: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static SCAN_DONE: AtomicBool = AtomicBool::new(false);

// ── 로더 detour(체인) ───────────────────────────────────────────────────────
extern "win64" fn detour(am: usize, path: *const u8, len: usize) -> usize {
    let ta = TRAMP.load(Ordering::Relaxed);
    if ta == 0 {
        return 0;
    }
    let tramp: LoaderFn = unsafe { core::mem::transmute(ta) };
    let r = tramp(am, path, len);
    // 현 씬 Assets(am)을 기록 — 관리 씬이면 챔프 로드된 AssetServer(아이콘 공식 렌더용).
    crate::hooks::note_assets(am);
    if r > 0x10000 {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            try_inject(r);
        }));
    }
    r
}

unsafe fn slice_eq(idp: usize, idl: usize, target: &[u8]) -> bool {
    idl == target.len()
        && idp > 0x10000
        && core::slice::from_raw_parts(idp as *const u8, idl) == target
}

/// 로드된 템플릿에 마커가 있으면 내 행/팝업을 append(각각 멱등).
/// ★option 템플릿 확정 = current_database_edit 마커 보유(new_game.ui 오주입 방지).
unsafe fn try_inject(r: usize) {
    // ★★2026-08-23 로딩 최적화: 주입은 **각각 1회**면 끝인데, 구버전은 주입이 끝난 뒤에도
    //   **모든 에셋 템플릿 로드마다** 트리를 깊이 14까지 재귀 스캔했다(마커 2종 × 템플릿 수백 개).
    //   시작 시 로딩(검정화면)이 그만큼 길어진다 ⟹ 완료 플래그로 조기 탈출한다.
    // 계측: 주입이 끝날 때까지 스캔한 템플릿 수/누적 시간(시작 지연 원인 규명용).
    let t0 = std::time::Instant::now();
    SCAN_N.fetch_add(1, Ordering::Relaxed);
    let need_tip = !INJECTED_SWAPTIP.load(Ordering::Relaxed);
    let need_opt = !INJECTED_ROW.load(Ordering::Relaxed)
        || (POPUP_INJECT && !INJECTED_POPUP.load(Ordering::Relaxed));
    if !need_tip && !need_opt {
        // 완료 후 첫 1회만 총계를 남긴다.
        if !SCAN_DONE.swap(true, Ordering::Relaxed) {
            config::llog(&format!(
                "inject: 주입 완료 — 스캔한 템플릿 {}개, 누적 {}ms (이후 스캔 안 함)",
                SCAN_N.load(Ordering::Relaxed),
                SCAN_US.load(Ordering::Relaxed) / 1000
            ));
        }
        return; // 할 일 없음 — 스캔조차 하지 않는다
    }
    // ── 밴픽 레이아웃(마커=swap_waiting): 스왑 툴팁을 **루트 마지막 자식**으로 append ──
    if need_tip && find_id(r, b"swap_waiting", 0) != 0 {
        if find_id(r, b"pos_lock_swaptip", 0) != 0 {
            INJECTED_SWAPTIP.store(true, Ordering::Relaxed);
        } else if append_child(r, SWAPTIP_UI) {
            INJECTED_SWAPTIP.store(true, Ordering::Relaxed);
            config::llog("pos_lock_swaptip 주입 OK (밴픽 레이아웃)");
        }
        return;
    }
    if !need_opt {
        SCAN_US.fetch_add(t0.elapsed().as_micros() as u64, Ordering::Relaxed);
        return;
    }
    let is_option = find_id(r, b"current_database_edit", 0) != 0;
    if !is_option {
        SCAN_US.fetch_add(t0.elapsed().as_micros() as u64, Ordering::Relaxed);
        return; // option 템플릿이 아니면 아무것도 하지 않음(custom_champion_popup 은 new_game 에도 있음)
    }
    // 행: 게임플레이 탭 contents 에 append.
    if !INJECTED_ROW.load(Ordering::Relaxed) {
        if find_id(r, b"pos_lock_row", 0) != 0 {
            INJECTED_ROW.store(true, Ordering::Relaxed);
        } else {
            let parent = find_parent_of_child(r, b"current_database_edit", 0);
            if parent != 0 && append_child(parent, ROW_UI) {
                INJECTED_ROW.store(true, Ordering::Relaxed);
                config::dlog("pos_lock_row 주입 OK (게임플레이 탭)");
            }
        }
    }
    // 팝업: option 템플릿 안쪽 노드(#tabs 의 부모 = header/tabs/contents 를 담는 inner option)에
    //   append → 중앙 정렬·최상단. custom_champion_popup 은 런타임 조합이라 정적 트리에 없어
    //   마커로 못 쓴다. #tabs 는 option.ui 에 인라인이라 안전한 앵커.
    if POPUP_INJECT && !INJECTED_POPUP.load(Ordering::Relaxed) {
        if find_id(r, b"pos_lock_popup", 0) != 0 {
            INJECTED_POPUP.store(true, Ordering::Relaxed);
        } else {
            let mut parent = find_parent_of_child(r, b"tabs", 0);
            if parent == 0 {
                parent = r; // 폴백: 템플릿 루트
            }
            if append_child(parent, POPUP_UI) {
                INJECTED_POPUP.store(true, Ordering::Relaxed);
                config::dlog("pos_lock_popup 주입 OK");
            }
        }
    }
}

unsafe fn find_id(node: usize, target: &[u8], depth: usize) -> usize {
    if node <= 0x10000 || depth > 14 {
        return 0;
    }
    let idp = *((node + 0x08) as *const usize);
    let idl = *((node + 0x10) as *const usize);
    if slice_eq(idp, idl, target) {
        return node;
    }
    let cptr = *((node + 0x50) as *const usize);
    let clen = *((node + 0x58) as *const usize);
    if cptr > 0x10000 && clen < 2000 {
        for i in 0..clen {
            let f = find_id(cptr + i * NT_SIZE, target, depth + 1);
            if f != 0 {
                return f;
            }
        }
    }
    0
}

/// target id 를 가진 직접 자식이 있는 노드(=그 자식의 부모)를 반환.
unsafe fn find_parent_of_child(node: usize, target: &[u8], depth: usize) -> usize {
    if node <= 0x10000 || depth > 14 {
        return 0;
    }
    let cptr = *((node + 0x50) as *const usize);
    let clen = *((node + 0x58) as *const usize);
    if cptr > 0x10000 && clen < 2000 {
        for i in 0..clen {
            let child = cptr + i * NT_SIZE;
            let idp = *((child + 0x08) as *const usize);
            let idl = *((child + 0x10) as *const usize);
            if slice_eq(idp, idl, target) {
                return node;
            }
        }
        for i in 0..clen {
            let f = find_parent_of_child(cptr + i * NT_SIZE, target, depth + 1);
            if f != 0 {
                return f;
            }
        }
    }
    0
}

/// parent 템플릿의 child 배열 끝에 ui 조각(파싱한 NodeTemplate)을 append.
unsafe fn append_child(parent: usize, ui: &str) -> bool {
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 {
        return false;
    }
    let parser: ParserFn = core::mem::transmute(base + PARSER_RVA);
    let mut out = [0u8; 0x400];
    parser(out.as_mut_ptr(), ui.as_ptr(), ui.len());
    let my = out.as_ptr().add(0x10) as usize;
    if *(my as *const usize) == usize::MAX {
        return false;
    }
    let ptr = *((parent + 0x50) as *const usize);
    let len = *((parent + 0x58) as *const usize);
    if len > 2000 {
        return false;
    }
    let galloc: AllocFn = core::mem::transmute(base + ALLOC_RVA);
    let new_n = len + 1;
    let np = galloc(0, 0, new_n * NT_SIZE);
    if np == 0 {
        return false;
    }
    if ptr != 0 && len != 0 {
        core::ptr::copy_nonoverlapping(ptr as *const u8, np as *mut u8, len * NT_SIZE);
    }
    core::ptr::copy_nonoverlapping(my as *const u8, (np + len * NT_SIZE) as *mut u8, NT_SIZE);
    // ptr → cap → len 순(중간 read 안전). 옛 배열 leak = 무해.
    *((parent + 0x50) as *mut usize) = np;
    *((parent + 0x48) as *mut usize) = new_n;
    *((parent + 0x58) as *mut usize) = new_n;
    true
}

// ── 체이닝 설치 (draft_overlay install_one 방식) ────────────────────────────
pub fn install() {
    if INSTALLED.swap(true, Ordering::Relaxed) {
        return;
    }
    unsafe {
        let base = GetModuleHandleW(core::ptr::null());
        if base == 0 {
            INSTALLED.store(false, Ordering::Relaxed);
            return;
        }
        BASE.store(base, Ordering::Relaxed);
        if !install_one(base + LOADER_RVA, &TRAMP, detour as usize) {
            INSTALLED.store(false, Ordering::Relaxed);
            return;
        }
        config::dlog("loader hook 설치 OK");
    }
}

unsafe fn install_one(fn_addr: usize, tramp: &AtomicUsize, dfn: usize) -> bool {
    if fn_addr <= 0x10000 {
        return false;
    }
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    // 이미 내 훅이 앞단이면 no-op
    if cur[0] == 0x48 && cur[1] == 0xb8 {
        let tgt = usize::from_le_bytes(cur[2..10].try_into().unwrap());
        if tgt == dfn {
            return true;
        }
    }
    let stub = VirtualAlloc(0, 64, MEM_CR, RWX);
    if stub == 0 {
        return false;
    }
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&cur); // 현재 진입부 12B(원본 프롤로그 또는 상대 스텁)
    s.extend_from_slice(&[0x48, 0xb8]);
    s.extend_from_slice(&(fn_addr + 0xc).to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    tramp.store(stub, Ordering::Relaxed);
    let mut patch = [0u8; 12];
    patch[0] = 0x48;
    patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&dfn.to_le_bytes());
    patch[10] = 0xff;
    patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, RWX, &mut old) == 0 {
        return false;
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    true
}
