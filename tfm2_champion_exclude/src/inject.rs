//! inject.rs — 환경설정 게임플레이 탭에 '추가 챔피언 설정' 행 + 팝업 주입.
//! ===========================================================================
//! 방식 = tfm2_champ_pos_lock/inject.rs 참조구현(로더 체인 훅 + 게임 PARSER + child append).
//! - 행: option 템플릿(마커=current_database_edit)의 게임플레이 탭 contents 에 append.
//!   ★유저 요구 "젤 아래" — 포지션 제한 행(pos_lock_row, 타 모드)이 같은 parent 에 있으면
//!   내 행이 **그 뒤(아래)** 로 가도록: 주입 후에도 순서를 감시해 pos_lock_row 보다 앞이면
//!   NodeTemplate 엔트리(0x90B)를 swap 한다(템플릿 재로드 대응·멱등).
//! - 팝업: #tabs 의 부모(inner option)에 append — pos_lock_popup 과 동일 앵커.
//! - 팝업 .ui 는 런타임 생성(build_popup_ui — 셀 120개 반복이라 정적 파일 대신 format!).
//! ===========================================================================
#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

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

// 공용 4심볼(0.5.6, MIGRATION §7.6 §2 — pos_lock/inject.rs 와 동일 값).
const LOADER_RVA: usize = 0x2ea930; // 에셋 게터 FUN(am, path, len) -> NodeTemplate*
const PARSER_RVA: usize = 0x1ab310; // .ui 텍스트 → NodeTemplate
const ALLOC_RVA: usize = 0x2ab4010; // 게임 alloc impl(_, flags, size) -> ptr
const NT_SIZE: usize = 0x90; // NodeTemplate stride
// 템플릿 오프셋: id ptr+0x08 / id len+0x10 / child cap+0x48 / child ptr+0x50 / child len+0x58

const ROW_UI: &str = include_str!("../assets/champ_excl_row.ui");

type LoaderFn = extern "win64" fn(usize, *const u8, usize) -> usize;
type ParserFn = extern "win64" fn(*mut u8, *const u8, usize);
type AllocFn = extern "win64" fn(usize, usize, usize) -> usize;

static BASE: AtomicUsize = AtomicUsize::new(0);
static TRAMP: AtomicUsize = AtomicUsize::new(0);
static INSTALLED: AtomicBool = AtomicBool::new(false);
pub static INJECTED_ROW: AtomicBool = AtomicBool::new(false);
pub static INJECTED_POPUP: AtomicBool = AtomicBool::new(false);
/// pos_lock_row 대비 순서 확정 여부(내 행이 뒤에 있음을 확인했거나 pos_lock 부재 판정).
static ROW_ORDERED: AtomicBool = AtomicBool::new(false);

// ── 로더 detour(체인) ───────────────────────────────────────────────────────
extern "win64" fn detour(am: usize, path: *const u8, len: usize) -> usize {
    let ta = TRAMP.load(Ordering::Relaxed);
    if ta == 0 {
        return 0;
    }
    let tramp: LoaderFn = unsafe { core::mem::transmute(ta) };
    let r = tramp(am, path, len);
    // 현 씬 Assets 기록 — 관리 씬이면 챔프 로드된 AssetServer(아이콘 공식 렌더용).
    crate::note_assets(am);
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

/// 로드된 템플릿에 마커가 있으면 행/팝업 append(멱등) + 행 순서 유지.
unsafe fn try_inject(r: usize) {
    let need = !INJECTED_ROW.load(Ordering::Relaxed)
        || !INJECTED_POPUP.load(Ordering::Relaxed)
        || !ROW_ORDERED.load(Ordering::Relaxed);
    if !need {
        return; // 전부 끝났으면 스캔조차 안 함(시작 로딩 지연 방지 — pos_lock 08-23 교훈)
    }
    let is_option = find_id(r, b"current_database_edit", 0) != 0;
    if !is_option {
        return;
    }
    // ── 행 ──
    if !INJECTED_ROW.load(Ordering::Relaxed) {
        if find_id(r, b"champ_excl_row", 0) != 0 {
            INJECTED_ROW.store(true, Ordering::Relaxed);
        } else {
            let parent = find_parent_of_child(r, b"current_database_edit", 0);
            if parent != 0 && append_child(parent, ROW_UI) {
                INJECTED_ROW.store(true, Ordering::Relaxed);
                crate::log("champ_excl_row injected (gameplay tab)");
            }
        }
    }
    // ── 행 순서: pos_lock_row(타 모드)가 있으면 내 행을 그 뒤로 ──
    if INJECTED_ROW.load(Ordering::Relaxed) && !ROW_ORDERED.load(Ordering::Relaxed) {
        let parent = find_parent_of_child(r, b"champ_excl_row", 0);
        if parent != 0 {
            let cptr = *((parent + 0x50) as *const usize);
            let clen = *((parent + 0x58) as *const usize);
            if cptr > 0x10000 && clen < 2000 {
                let mut mine = usize::MAX;
                let mut theirs = usize::MAX;
                for i in 0..clen {
                    let c = cptr + i * NT_SIZE;
                    let idp = *((c + 0x08) as *const usize);
                    let idl = *((c + 0x10) as *const usize);
                    if slice_eq(idp, idl, b"champ_excl_row") {
                        mine = i;
                    } else if slice_eq(idp, idl, b"pos_lock_row") {
                        theirs = i;
                    }
                }
                if mine != usize::MAX && theirs != usize::MAX {
                    if mine < theirs {
                        // 0x90B 엔트리 통swap(NodeTemplate move = bitwise 안전 — append_child 와 동근거)
                        let a = (cptr + mine * NT_SIZE) as *mut u8;
                        let b = (cptr + theirs * NT_SIZE) as *mut u8;
                        for k in 0..NT_SIZE {
                            core::ptr::swap(a.add(k), b.add(k));
                        }
                        crate::log("row order adjusted: champ_excl_row below pos_lock_row");
                    }
                    ROW_ORDERED.store(true, Ordering::Relaxed);
                }
                // pos_lock 미설치 환경: 팝업까지 끝났으면 더 기다리지 않고 종결
                //   (pos_lock 이 나중에 주입하면 그쪽이 아래로 가지만 — 기능 무영향·수용).
                if theirs == usize::MAX && INJECTED_POPUP.load(Ordering::Relaxed) {
                    ROW_ORDERED.store(true, Ordering::Relaxed);
                }
            }
        }
    }
    // ── 팝업 ──
    if !INJECTED_POPUP.load(Ordering::Relaxed) {
        if find_id(r, b"champ_excl_popup", 0) != 0 {
            INJECTED_POPUP.store(true, Ordering::Relaxed);
        } else {
            let mut parent = find_parent_of_child(r, b"tabs", 0);
            if parent == 0 {
                parent = r; // 폴백: 템플릿 루트
            }
            let ui = crate::build_popup_ui();
            if append_child(parent, &ui) {
                INJECTED_POPUP.store(true, Ordering::Relaxed);
                crate::log("champ_excl_popup injected");
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
    // ptr → cap → len 순(중간 read 안전). 옛 배열 leak = 무해(게임 free = 경합 크래시 이력).
    *((parent + 0x50) as *mut usize) = np;
    *((parent + 0x48) as *mut usize) = new_n;
    *((parent + 0x58) as *mut usize) = new_n;
    true
}

// ── 체이닝 설치 (draft_overlay install_one 방식 — pos_lock 동일) ────────────
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
        crate::log("loader hook installed (UI inject)");
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
