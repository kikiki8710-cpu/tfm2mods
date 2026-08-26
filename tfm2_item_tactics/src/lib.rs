//! tfm2_item_tactics — 네이티브 개인전술 화면 아이템 드롭다운에 모드 아이템 주입
//! ===========================================================================
//! 목표: 경기 시작 전 "전술 → 개인" 화면(strategy.ui #personal)의 선수별
//!       아이템 드롭다운(#item0/#item1/#item2)에 모드 추가 아이템을 옵션으로 노출.
//!       선택 시 라이브 매치 빌드에 강제 주입(approach B, save-safe).
//!
//! ─ Phase 1a (완료) ─ 전술화면 감지 + 네이티브 드롭다운 옵션 주입 + 선택 폴링.
//! ─ Phase 1b (현재) ─ 실제 모드 최종템 열거(dump_mod_items→MOD_REGISTRY/MOD_FINALS,
//!                     활성필터, i18n 라벨). 게임함수 후킹 없음 = 무크래시.
//! ─ Phase 2  (다음) ─ FUN_140c6c430 write 3곳 트램폴린 detour로 라이브 빌드 주입.
//!
//! 재사용 출처: C:\tfm2mods\tfm2_scrim\src\lib.rs (nat_dd_*, SEH, dump_mod_items, 아이템 머신러리).
//! ===========================================================================
#![allow(dead_code, unused_imports, unused_variables)]
use mod_api::*;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicPtr, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

#[path = "ui_inject.rs"] mod uinj; // 4번째 슬롯 UI 주입(체이닝 로더훅): item3 드롭다운 + 경기중 slot3 표시
#[path = "perf.rs"] mod perf;      // 훅별 비용 계측(부담 측정). perf::PERF_ON=false 면 호출부까지 DCE.

const MOD_ID: &str = "tfm2_item_tactics";

// 네이티브 dropdown set-options 함수 (0.4.14 핫픽스, scrim 과 동일 RVA).
//   프롤로그 55 56 57 48 83 ec 70, 옵션 Vec@+0x1528, 선택 idx@+0x1788.
//   ⚠ 패치마다 이동 → MIGRATION 시 재탐색.
// 0.5.0 확정(구 0x218a5f0). dd_addr_valid() 프롤로그 가드(55 56 57 48 83 ec 70)로 검증 후 사용.
const FN_DD_SETOPT_RVA: usize = 0x1c31f0; // 0.5.6(구0.5.5=0x1c1a30, skel UNIQUE·BYTE=SAME·size 379·프롤로그 동일). // 0.5.5(구0.5.4=0x1c1ad0, exe2exe 스켈레톤 확정). // 0.5.4(구0.5.3=0x1bfc80). 본문 니모닉 100% 동일·콜러 103개 동수·프롤로그 12B 동일. // 0.5.3(구0.5.2=0x242f250). ghidra-re 확정: 직접 콜러 103개로 구 exe와 완전 일치 + 오프셋 지문 4종(+0x1788 selected / +0x1528·0x1530·0x1538 옵션Vec / +0x1570·0x1578 콜백 / 원소 0xf8 / 입력 stride 0x28) 전부 불변. ⚠프롤로그는 변경됨(아래 dd_addr_valid expect 갱신).

const LOG_ENABLED: bool = false; // 프로덕션 OFF(진단 로깅·dump·[slot012] 로그 게이트). 주입 로직은 이 게이트 바깥이라 무영향.
// ★프로덕션 마스터 진단 게이트(07-11): 이번 세션 진단(nn_moditem·timing·liveroster·p6/channel scan·shadow-call 카탈로그이름조회) +
//   기존 진단 flush/훅(c6new·countprobe·auto4·teamgate) 전부 OFF. 팀게이트(is_live/is_player)·SLOT012 주입은 게이트 바깥 = 무영향.
const DIAG_ENABLED: bool = false;

const MAX_ROWS: usize = 5;   // 선수 5명 (#row0..#row4)
const ITEM_SLOTS: usize = 4; // 최대 칸수(배열 stride). 실제 활성 칸 = slot_count() (토글 3/4)

// ── 3/4 아이템 토글 (cfg `4items.cfg`, dll 옆) ──
//   내용에 '4'=4칸(item0/1/2/3) / '3'=3칸(바닐라 item_tactics 동작). 없으면 기본 4. 변경=재시작.
static ITEM_MODE: AtomicU64 = AtomicU64::new(4);
fn load_mode() -> u64 {
    // ★설정 로드는 반드시 흔적을 남긴다(07-21): 읽기 실패 시 조용히 기본값 4로 떨어지면
    //   유저가 3칸을 원해 cfg를 만들어도 경로가 어긋난 걸 알 수 없다(원인불명 "3칸이 안 먹음").
    //   → mod_dir 경로·읽기 성공여부·파싱된 mode 를 항상 파일로 남김(LOG_ENABLED 무관, init 1회라 비용 무시).
    let mut mode = 4u64;
    let mut diag = String::new();
    match mod_dir() {
        None => diag.push_str("⚠mod_dir()=None (game_root 도출 실패) → 기본값 4칸 사용
"),
        Some(d) => {
            let p = d.join("4items.cfg");
            diag.push_str(&format!("mod_dir = {}
cfg 경로 = {}
존재 = {}
", d.display(), p.display(), p.exists()));
            match fs::read_to_string(&p) {
                Err(e) => diag.push_str(&format!("⚠cfg 읽기 실패({}) → 기본값 4칸 사용. 3칸을 원하면 이 경로에 'slots = 3' 파일 필요
", e)),
                Ok(s) => {
                    let scan = match s.rfind('=') { Some(i) => &s[i + 1..], None => &s[..] };
                    let mut found = false;
                    for c in scan.chars() { if c == '3' { mode = 3; found = true; break; } if c == '4' { mode = 4; found = true; break; } }
                    diag.push_str(&format!("cfg 읽기 OK ({}B) · 파싱대상={:?} · 숫자발견={}
", s.len(), scan.trim(), found));
                    if !found { diag.push_str("⚠'=' 뒤에서 3/4를 못 찾음 → 기본값 4칸 사용
"); }
                }
            }
        }
    }
    diag.push_str(&format!("★최종 mode = {} (slot_count={})
", mode, if mode == 4 { 4 } else { 3 }));
    if let Some(d) = mod_dir() { let _ = fs::create_dir_all(&d); let _ = fs::write(d.join("4items_mode.txt"), &diag); }
    ITEM_MODE.store(mode, Ordering::Relaxed);
    uinj::MODE4.store(mode == 4, Ordering::Relaxed);
    uinj::IN_MATCH_UI.store(mode == 4, Ordering::Relaxed); // 경기중 4번째 슬롯 UI 켬(패치와 함께). mode=3=바닐라 3슬롯 유지.
    uinj::STRAT_INJECT.store(mode == 3 || mode == 4, Ordering::Relaxed); // 전술화면 오버레이(item0m/1m/2m)=mode 3·4 공통
    mode
}
fn slot_count() -> usize { if ITEM_MODE.load(Ordering::Relaxed) == 4 { 4 } else { 3 } }

// 바닐라 7옵션 라벨 (idx 0~6). 게임 personal_tactics ItemBuildOverride 와 1:1.
//   ★게임 i18n 에셋 참조 → 드롭다운이 게임 언어(base.json lang)로 자동 현지화(모드 아이템 vi≥7과 동일
//   방식, 검증됨). 통짜 단일 라벨이라 LabelRunner가 치환(인라인 조합만 안 됨). 하드코딩 한글 폐기.
//   키 출처: strategy.i18n(build_auto) / ui.i18n(attack·magic_power·attack_speed·defence·magic_resistance·hp).
const VANILLA_OPTS: [&str; 7] = [
    "#asset/base/text/strategy?personal.build_auto",
    "#asset/base/text/item?category.ad",
    "#asset/base/text/item?category.magic",
    "#asset/base/text/item?category.attack_speed",
    "#asset/base/text/item?category.defense",
    "#asset/base/text/item?category.magic_resistance",
    "#asset/base/text/item?category.hp",
];

// ===========================================================================
//  WinAPI FFI
// ===========================================================================
type HMODULE = isize; type DWORD = u32; type BOOL = i32;
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> HMODULE;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, n: DWORD) -> DWORD;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old_protect: *mut u32) -> BOOL;
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, size: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
    fn AddVectoredExceptionHandler(first: u32, handler: VehHandler) -> usize;
    fn GetCurrentThreadId() -> u32;
    fn GetCurrentThread() -> isize;
    fn GetThreadContext(h: isize, ctx: *mut u8) -> BOOL;
    fn SetThreadContext(h: isize, ctx: *const u8) -> BOOL;
    fn OpenThread(access: u32, inherit: BOOL, tid: u32) -> isize;
    fn SuspendThread(h: isize) -> u32;
    fn ResumeThread(h: isize) -> u32;
    fn CloseHandle(h: isize) -> BOOL;
    fn CreateThread(sa: *const u8, stack: usize, start: extern "system" fn(*mut u8) -> u32, param: *mut u8, flags: u32, tid: *mut u32) -> isize;
    fn Sleep(ms: u32);
}
#[repr(C)]
#[derive(Default)]
struct MemBasicInfo {
    base: usize, alloc_base: usize, alloc_protect: u32, _pad0: u32,
    region_size: usize, state: u32, protect: u32, mtype: u32, _pad1: u32,
}

// dropdown 옵션 1개 = 0x28(40바이트): 색상 16B + 텍스트 String 24B (게임 String = {len, ptr, cap})
#[repr(C)]
struct DdOpt {
    color:  u64,   // +0  R@0=1.0, G@4=1.0
    color2: u32,   // +8  B@8=1.0
    alpha:  f32,   // +12 A=1.0
    s_len:  usize, // +16
    s_ptr:  usize, // +24
    s_cap:  usize, // +32
}

// ===========================================================================
//  메모리 안전 헬퍼 (scrim 포팅)
// ===========================================================================
unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    let n = VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>());
    if n == 0 { return false; }
    const MEM_COMMIT: u32 = 0x1000;
    const READABLE: u32 = 0x02 | 0x04 | 0x20 | 0x40;
    const NOACCESS_GUARD: u32 = 0x01 | 0x100;
    if mbi.state != MEM_COMMIT { return false; }
    if mbi.protect & NOACCESS_GUARD != 0 { return false; }
    if mbi.protect & READABLE == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}
unsafe fn writable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    let n = VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>());
    if n == 0 { return false; }
    const MEM_COMMIT: u32 = 0x1000;
    const WRITABLE: u32 = 0x04 | 0x08 | 0x40 | 0x80;
    const GUARD: u32 = 0x100;
    if mbi.state != MEM_COMMIT { return false; }
    if mbi.protect & GUARD != 0 { return false; }
    if mbi.protect & WRITABLE == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}
// ★안정성: 함수포인터가 실제 실행가능 코드페이지를 가리키는지 검증(shadow-call 前). readable만으론
//   비실행 페이지에서 DEP AV가 남음 → PAGE_EXECUTE_* 확인. VEH가 못잡는 AV를 사전차단.
unsafe fn code_ptr_ok(p: usize) -> bool {
    if p < 0x10000 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(p as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const MEM_COMMIT: u32 = 0x1000;
    const EXEC: u32 = 0x10 | 0x20 | 0x40 | 0x80; // PAGE_EXECUTE / _READ / _READWRITE / _WRITECOPY
    const BAD: u32 = 0x100 | 0x01;               // GUARD | NOACCESS
    mbi.state == MEM_COMMIT && (mbi.protect & BAD) == 0 && (mbi.protect & EXEC) != 0
}
fn looks_heap(v: u64) -> bool { v & 0x7 == 0 && v >= 0x10000 && v < 0x0000_8000_0000_0000 && (v & 0xffff) != 0 }

// ===========================================================================
//  SEH 안전읽기 — VEH 로 접근위반(0xC0000005)을 가로채 크래시 대신 실패 반환.
//  SEH[]: 0=active 1=tid 2=land_rip 3=land_rsp 4=land_rbp 5=code_lo 6=code_hi 7=faults
// ===========================================================================
#[repr(C)]
struct ExceptionRecord { code: u32, flags: u32, rec: usize, addr: usize, nparams: u32, _p: u32, params: [usize; 15] }
#[repr(C)]
struct ExceptionPointers { rec: *mut ExceptionRecord, ctx: *mut core::ffi::c_void }
type VehHandler = extern "system" fn(*mut ExceptionPointers) -> i32;

// ★2026-07-22 전환: 전역 SEH[8] + 스핀락 → **스레드별 TLS**. (perf 계측 근거)
//   구: safe_copy 가 전역 상태 하나를 공유하느라 `while SEH_BUSY.swap(true) { spin_loop() }` 로
//   **모든 rayon 워커를 직렬화**했다. buy 조기탈출 경로가 safe_read_u64 를 매 콜 부르므로
//   (130.7s에 689만회) 워커 수만큼 스핀 경합 = 모드 최대 비용원 중 하나였음.
//   VEH 핸들러는 **폴트난 바로 그 스레드 위에서** 실행되므로 자기 TLS를 읽으면 된다
//   ⇒ 락 불필요 + tid 대조도 불필요(TLS 자체가 스레드 스코프라 구조적으로 보장).
//   ⚠VEH 안전요건 유지: Cell 배열 + `const` 초기화 + **Drop 없음** ⇒ 지연초기화 플래그도 TLS
//     소멸자 등록도 없다 = 핸들러 안에서 할당/락/패닉이 발생할 경로가 없음(§3 규칙 준수).
//   레이아웃은 구 [u64;8]과 동일(asm 오프셋 그대로). idx1(구 tid)은 미사용으로 남김.
#[repr(C)]
struct SehTls { v: [core::cell::Cell<u64>; 8] }
thread_local! {
    static SEH_T: SehTls = const { SehTls { v: [const { core::cell::Cell::new(0) }; 8] } };
}
#[inline(always)]
fn seh_ptr() -> *mut u64 {
    // Cell<u64>는 repr(transparent) → [Cell<u64>;8]과 [u64;8]은 레이아웃 동일.
    SEH_T.with(|s| s.v.as_ptr() as *mut u64)
}
static SEH_INSTALLED: AtomicBool = AtomicBool::new(false);

extern "system" fn seh_veh(p: *mut ExceptionPointers) -> i32 {
    const CONTINUE_EXECUTION: i32 = -1;
    const CONTINUE_SEARCH: i32 = 0;
    unsafe {
        if p.is_null() { return CONTINUE_SEARCH; }
        let rec = (*p).rec;
        if rec.is_null() { return CONTINUE_SEARCH; }
        if (*rec).code != 0xC0000005 { return CONTINUE_SEARCH; }
        // ★TLS 전환: 이 핸들러는 폴트난 스레드에서 도므로 자기 TLS가 곧 그 스레드의 상태
        //   (구 tid 대조는 불필요해짐). try_with = TLS 소멸중이면 조용히 패스(패닉 금지 요건).
        let Ok(g) = SEH_T.try_with(|s| s.v.as_ptr() as *mut u64) else { return CONTINUE_SEARCH; };
        if *g.add(0) == 0 { return CONTINUE_SEARCH; }
        let ctx = (*p).ctx as usize;
        if ctx == 0 { return CONTINUE_SEARCH; }
        let rip = *((ctx + 0xF8) as *const u64);
        if rip < *g.add(5) || rip >= *g.add(6) { return CONTINUE_SEARCH; }
        *((ctx + 0xF8) as *mut u64) = *g.add(2); // Rip = land_rip
        *((ctx + 0x98) as *mut u64) = *g.add(3); // Rsp = land_rsp
        *((ctx + 0xA0) as *mut u64) = *g.add(4); // Rbp = land_rbp
        *g.add(7) += 1; // 폴트 카운터(이제 스레드별)
        CONTINUE_EXECUTION
    }
}
fn seh_install() {
    if SEH_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    unsafe { AddVectoredExceptionHandler(1, seh_veh); }
}
#[inline(never)]
unsafe fn safe_copy(dst: *mut u8, src: *const u8, len: usize) -> bool {
    if !SEH_INSTALLED.load(Ordering::Relaxed) { return false; }
    // ★락 없음: 상태가 스레드별이라 워커끼리 경합하지 않는다(구 SEH_BUSY 스핀락 제거).
    let g = seh_ptr();
    let mut ok: u64;
    core::arch::asm!(
        "lea rax, [rip + 200f]",
        "mov [{g} + 40], rax",
        "lea rax, [rip + 201f]",
        "mov [{g} + 48], rax",
        "lea rax, [rip + 202f]",
        "mov [{g} + 16], rax",
        "mov [{g} + 24], rsp",
        "mov [{g} + 32], rbp",
        "mov qword ptr [{g} + 0], 1",
        "cld",
        "200:",
        "rep movsb",
        "201:",
        "mov {ok}, 1",
        "jmp 203f",
        "202:",
        "mov {ok}, 0",
        "203:",
        "mov qword ptr [{g} + 0], 0",
        g = in(reg) g,
        ok = out(reg) ok,
        inout("rcx") len => _,
        inout("rdi") dst => _,
        inout("rsi") src => _,
        out("rax") _,
    );
    ok != 0
}
unsafe fn safe_read_u64(addr: usize) -> Option<u64> {
    let mut b = [0u8; 8];
    if safe_copy(b.as_mut_ptr(), addr as *const u8, 8) { Some(u64::from_le_bytes(b)) } else { None }
}
unsafe fn safe_read_bytes(addr: usize, len: usize, out: &mut Vec<u8>) -> bool {
    if len == 0 || len > 4096 { return false; }
    out.clear(); out.resize(len, 0);
    safe_copy(out.as_mut_ptr(), addr as *const u8, len)
}

// ===========================================================================
//  로깅 / 경로
// ===========================================================================
fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }
// 게임 exe 경로 (GetModuleHandleW(NULL) = 메인 exe). 하드코딩 금지 — 경로 동적도출.
fn exe_path() -> Option<PathBuf> {
    let mut buf = [0u16; 1024];
    let n = unsafe { GetModuleFileNameW(GetModuleHandleW(core::ptr::null()), buf.as_mut_ptr(), buf.len() as u32) };
    if n == 0 { return None; }
    Some(PathBuf::from(String::from_utf16_lossy(&buf[..n as usize])))
}
// 게임 루트 = exe 폴더(...\Teamfight Manager2).
fn game_root() -> Option<PathBuf> { exe_path()?.parent().map(|p| p.to_path_buf()) }
fn mod_dir() -> Option<PathBuf> { Some(game_root()?.join("mods").join(MOD_ID)) }
fn write_log(name: &str, content: &str) {
    if !LOG_ENABLED { return; }
    if let Some(d) = mod_dir() { let _ = fs::create_dir_all(&d); let _ = fs::write(d.join(name), content); }
}
fn append_log(name: &str, line: &str) {
    if !LOG_ENABLED { return; }
    if let Some(d) = mod_dir() {
        let _ = fs::create_dir_all(&d);
        let p = d.join(name);
        let mut s = fs::read_to_string(&p).unwrap_or_default();
        s.push_str(line); s.push('\n');
        let _ = fs::write(p, s);
    }
}

// ===========================================================================
//  네이티브 dropdown 제어 (scrim 포팅)
// ===========================================================================
// ★0.5.1 재활성(07-15): DD_SETOPT(0x2450f40) = ghidra-re가 OLD 0x2416070↔NEW 라인단위 동일 확증(HIGH, 3형제 중 올바른 것·오프셋 +0x1788/+0x1528/+0x1570 stride 0x28·0xf8 전부 일치). 오식별 아님 → ON.
const DD_ENABLED: bool = true;
static DD_VALID: AtomicU64 = AtomicU64::new(0);
unsafe fn prologue_hex(addr: usize, n: usize) -> String {
    if !readable(addr, n) { return "UNREADABLE".to_string(); }
    (0..n).map(|i| format!("{:02x}", *((addr + i) as *const u8))).collect::<Vec<_>>().join(" ")
}
unsafe fn dd_addr_valid() -> bool {
    if !DD_ENABLED { return false; } // 0.5.1 오식별 완화 게이트
    match DD_VALID.load(Ordering::Relaxed) { 1 => return true, 2 => return false, _ => {} }
    let fa = GetModuleHandleW(core::ptr::null()) as usize + FN_DD_SETOPT_RVA;
    // 0.5.3: push rbp/r15/r14/rsi/rdi/rbx + sub rsp,0x88 (구 0.5.2 = 55 56 57 48 83 ec 70)
    let expect = [0x55u8, 0x41, 0x57, 0x41, 0x56, 0x56, 0x57, 0x53, 0x48, 0x81, 0xec, 0x88];
    let mut ok = readable(fa, 12);
    if ok { for i in 0..12 { if *((fa + i) as *const u8) != expect[i] { ok = false; break; } } }
    DD_VALID.store(if ok { 1 } else { 2 }, Ordering::Relaxed);
    ok
}
unsafe fn runner_base(n: &Node) -> usize {
    let any: &dyn std::any::Any = n.runner.as_any();
    let parts: [usize; 2] =
        std::mem::transmute::<*const dyn std::any::Any, [usize; 2]>(any as *const dyn std::any::Any);
    parts[0]
}
fn find_rb(n: &Node, t: &str) -> Option<usize> {
    if n.id.as_str() == t { return Some(unsafe { runner_base(n) }); }
    for c in n.child.iter() { if let Some(b) = find_rb(c, t) { return Some(b); } }
    None
}
fn find_node<'a>(n: &'a Node, t: &str) -> Option<&'a Node> {
    if n.id.as_str() == t { return Some(n); }
    for c in n.child.iter() { if let Some(x) = find_node(c, t) { return Some(x); } }
    None
}
fn type_name_of(root: &Node, id: &str) -> Option<String> {
    fn rec(n: &Node, id: &str) -> Option<String> {
        if n.id.as_str() == id { return Some(n.runner.type_name().to_string()); }
        for c in n.child.iter() { if let Some(x) = rec(c, id) { return Some(x); } }
        None
    }
    rec(root, id)
}
fn find_mut<'a>(n: &'a mut Node, id: &str) -> Option<&'a mut Node> {
    if n.id.as_str() == id { return Some(n); }
    for c in n.child.iter_mut() { if let Some(x) = find_mut(c, id) { return Some(x); } }
    None
}
// ImageRunner source(에셋키) 교체 — 게임 String {len@0, ptr@8, cap@16}. static 문자열이라 leak 없음.
//   빈 슬롯(cap=0)이면 게임이 free 안 함(safe). tfm2_fog set_img_source_ptr 검증 레이아웃.
unsafe fn set_img_src(n: &Node, s: &'static str) -> bool {
    if !n.runner.type_name().contains("ImageRunner") { return false; }
    let dp = runner_base(n);
    if dp < 0x10000 { return false; }
    std::ptr::write_unaligned(dp as *mut u64, s.len() as u64);
    std::ptr::write_unaligned((dp + 8) as *mut u64, s.as_ptr() as u64);
    true
}
// ★검증: 경기중 모든 #slot3 아이콘 source를 테스트 아이템(t5_0)으로 세팅 → 뜨는지 확인.
//   #slot3 노드가 라이브 경기 트리에 있고 source-write가 먹히면 = 이 접근 전체 검증.
const TEST_ITEM_SRC: &str = "asset/base/aseprite_resources/ingame/item_icons_18x18#t5_0";
static SLOT3_TEST_LOGGED: AtomicBool = AtomicBool::new(false);
unsafe fn runner_bytes(n: &Node) -> String {
    let dp = runner_base(n);
    let mut s = format!("rb={:#x}", dp);
    for o in (0..0x48).step_by(8) { s.push_str(&format!(" +{:#x}={:#x}", o, std::ptr::read_unaligned((dp + o) as *const u64))); }
    s
}
// ImageRunner 소스 문자열(챔프 포트레이트 경로 등). 데이터ptr: len@+0, ptr@+8.
unsafe fn read_img_source(n: &Node) -> Option<String> {
    if !n.runner.type_name().contains("ImageRunner") { return None; }
    let dp = runner_base(n);
    let len = std::ptr::read_unaligned(dp as *const u64) as usize;
    let ptr = std::ptr::read_unaligned((dp + 8) as *const u64) as *const u8;
    if ptr.is_null() || len == 0 || len > 512 || !readable(ptr as usize, len) { return None; }
    Some(String::from_utf8_lossy(std::slice::from_raw_parts(ptr, len)).to_string())
}
// LabelRunner 텍스트(text@+352, len@+352,ptr@+360).
const TEXT_OFFSET: usize = 352;
unsafe fn read_label(n: &Node) -> Option<String> {
    if !n.runner.type_name().contains("LabelRunner") { return None; }
    let dp = runner_base(n);
    let len = std::ptr::read_unaligned((dp + TEXT_OFFSET) as *const u64) as usize;
    let ptr = std::ptr::read_unaligned((dp + TEXT_OFFSET + 8) as *const u64) as *const u8;
    if ptr.is_null() || len == 0 || len > 512 || !readable(ptr as usize, len) { return None; }
    Some(String::from_utf8_lossy(std::slice::from_raw_parts(ptr, len)).to_string())
}
// 노드 서브트리를 id + (이미지소스/라벨텍스트) 로 덤프 (챔프키 위치찾기 진단).
unsafe fn dump_subtree(n: &Node, depth: usize, out: &mut String) {
    let id = n.id.as_str();
    let mut extra = String::new();
    if let Some(s) = read_img_source(n) { extra.push_str(&format!("  img='{}'", s)); }
    if let Some(s) = read_label(n) { extra.push_str(&format!("  label='{}'", s)); }
    if !id.is_empty() || !extra.is_empty() {
        out.push_str(&format!("{}{}{}\n", "  ".repeat(depth), if id.is_empty() { "(no-id)" } else { id }, extra));
    }
    for c in n.child.iter() { dump_subtree(c, depth + 1, out); }
}
// ★진단(slot3 아이콘 채우기 설계): 경기중 player_info 슬롯 아이콘 노드 덤프 —
//   채워진 slot0/1/2 vs 빈 slot3의 ImageRunner source + runner base 바이트(레이아웃 확정).
static SLOTDIAG_CNT: AtomicU64 = AtomicU64::new(0);
// ★블루 슬롯 강제 42간격 (0.5.0): 게임이 blue slot0/1/2를 바닐라(간격50)로 재설정 → 렌더가 읽는
//   authored x(node+0x84 계열)를 매프레임 덮어써, slot0 실제위치 기준 slot1/2/3를 42간격으로 강제.
//   slot0은 그대로 두고 뒤 3칸만 재배치 → base_x/해상도 무관.
//   ⚠~~"렌더가 +0x240(screen_x)을 읽어 그린다"~~ → 정정: 실제 동작 구현은 +0x84 계열을 쓴다(아래 함수 주석
//     "+0x240은 히트테스트라 무효였음"이 사후 정정본). **+0x240의 실체와 y/w/h 연속 여부는 미확정** —
//     히트박스 갱신이 필요하면 반드시 실측(구조 덤프) 후에 손댈 것.
const FORCE_BLUE_SPACING: f32 = 42.0;
// 노드 authored x = +0x84(normal). hover/press/disabled 상태블록에 +0x80 stride로 복제 → 4곳 전부 써야
// 게임 리셋/상태전환에도 유지됨(tfm2-ui-runtime-layout: 값=블록+0x14, 블록 0x70/0xf0/0x170/0x1f0).
#[inline]
unsafe fn set_node_x_all_states(node: &Node, x: f32) {
    let na = node as *const Node as usize;
    if na <= 0x10000 { return; }
    for off in [0x84usize, 0x104, 0x184, 0x204] {
        if writable(na + off, 4) { *((na + off) as *mut f32) = x; }
    }
}
// 게임이 매프레임 blue_player 슬롯/스탯 x(+0x84)를 바닐라(간격50)로 재설정 → post_update서 42간격+왼쪽정렬로 재강제.
// (아이콘 렌더는 +0x84 authored를 씀. +0x240은 히트테스트라 무효였음.)
unsafe fn force_blue_slot_spacing(n: &Node) {
    if n.id.as_str() == "blue_player" {
        // slot0의 현재 x를 기준(leftmost) — 없으면 59 폴백.
        let mut base = 59.0f32;
        if let Some(s0) = find_node(n, "slot0") {
            let na = s0 as *const Node as usize;
            if na > 0x10000 && readable(na + 0x84, 4) {
                let v = *((na + 0x84) as *const f32);
                if v.is_finite() && v > 1.0 && v < 2000.0 { base = v; }
            }
        }
        // 슬롯: base + 42*i (딱 붙게, 레드와 동일 간격)
        for i in 0..4u32 {
            if let Some(sl) = find_node(n, &format!("slot{}", i)) {
                set_node_x_all_states(sl, base + FORCE_BLUE_SPACING * i as f32);
            }
        }
        // kda/cs: 왼쪽으로 (champion 372 겹침 해소). .ui와 동일 목표값 강제(리셋 대비).
        if let Some(k) = find_node(n, "kda") { set_node_x_all_states(k, 242.0); }
        if let Some(c) = find_node(n, "cs")  { set_node_x_all_states(c, 290.0); }
    }
    for c in n.child.iter() { force_blue_slot_spacing(c); }
}
// root 서브트리의 target 노드에 items 옵션 세팅, sel 선택.
unsafe fn nat_dd_set_options(root: &Node, target: &str, items: &[&str], sel: u64) -> bool {
    if !dd_addr_valid() { return false; }
    let Some(rb) = find_rb(root, target) else { return false; };
    let mut opts: Vec<DdOpt> = Vec::with_capacity(items.len());
    for &it in items {
        let s = it.to_string();
        opts.push(DdOpt {
            color: 0x3f800000_3f800000, color2: 0x3f800000, alpha: 1.0,
            s_len: s.len(), s_ptr: s.as_ptr() as usize, s_cap: s.capacity(),
        });
        std::mem::forget(s);
    }
    let param3: [usize; 3] = [0, opts.as_ptr() as usize, opts.len()];
    let addr = GetModuleHandleW(core::ptr::null()) as usize + FN_DD_SETOPT_RVA;
    let f: unsafe extern "system" fn(usize, u64, *const [usize; 3]) = std::mem::transmute(addr);
    f(rb, sel, &param3);
    std::mem::forget(opts);
    true
}
unsafe fn nat_dd_selected(root: &Node, target: &str) -> Option<usize> {
    if !dd_addr_valid() { return None; }
    let rb = find_rb(root, target)?;
    let v = *((rb + 0x1788) as *const u64);
    if v == u64::MAX { None } else { Some(v as usize) }
}
// 펼침 목록 최대높이(px). 옵션 총높이가 넘으면 엔진이 스크롤바/클립 자동 처리.
//   정석은 .ui `max_items_height:NNN;` 이나 네이티브 strategy.ui 는 못고침 → 런타임 라이트.
//   ★0.4.14 오프셋(ghidra-re): present 플래그@runner+0x1150(u32=1) + 값@runner+0x1154(f32 px).
//   (구버전 0x1d8 은 0.4.14 폐기 → 안 먹었음.) 파서 FUN_14218cb20 가 둘 다 set, 팝업빌더
//   FUN_14218a780 가 매 호출 +0x1154 read → 런타임 라이트 viable(다음 펼침때 반영).
const MAX_ITEMS_HEIGHT: f32 = 280.0; // 게임 실측: pause.ui product_dropdown=280
unsafe fn set_dd_max_height(root: &Node, target: &str, h: f32) {
    if let Some(rb) = find_rb(root, target) {
        if writable(rb + 0x1150, 8) {
            *((rb + 0x1150) as *mut u32) = 1;   // present 플래그 (Option=Some)
            *((rb + 0x1154) as *mut f32) = h;   // max_items_height (px)
        }
    }
}

// ===========================================================================
//  JSON 파서 (mods.json / item.i18n 파싱용, scrim 포팅)
// ===========================================================================
enum JsonValue { Null, Bool(bool), Num(f64), Str(String), Arr(Vec<JsonValue>), Obj(Vec<(String, JsonValue)>) }
impl JsonValue {
    fn as_obj(&self) -> Option<&Vec<(String, JsonValue)>> { if let JsonValue::Obj(o) = self { Some(o) } else { None } }
    fn get<'b>(&'b self, key: &str) -> Option<&'b JsonValue> { self.as_obj()?.iter().find(|(k, _)| k == key).map(|(_, v)| v) }
    fn as_str(&self) -> Option<&str> { if let JsonValue::Str(s) = self { Some(s.as_str()) } else { None } }
}
struct JsonParser<'a> { b: &'a [u8], i: usize }
impl<'a> JsonParser<'a> {
    fn new(s: &'a str) -> Self { JsonParser { b: s.as_bytes(), i: 0 } }
    fn skip_ws(&mut self) {
        while self.i < self.b.len() {
            match self.b[self.i] { b' ' | b'\t' | b'\r' | b'\n' | b',' => self.i += 1, _ => break }
        }
    }
    fn parse_value(&mut self) -> Option<JsonValue> {
        self.skip_ws();
        if self.i >= self.b.len() { return None; }
        match self.b[self.i] {
            b'{' => self.parse_object(),
            b'[' => self.parse_array(),
            b'"' => self.parse_string().map(JsonValue::Str),
            b't' => { self.i += 4; Some(JsonValue::Bool(true)) }
            b'f' => { self.i += 5; Some(JsonValue::Bool(false)) }
            b'n' => { self.i += 4; Some(JsonValue::Null) }
            _ => self.parse_number(),
        }
    }
    fn parse_string(&mut self) -> Option<String> {
        if self.b.get(self.i) != Some(&b'"') { return None; }
        self.i += 1;
        let mut out: Vec<u8> = Vec::new();
        while self.i < self.b.len() {
            let c = self.b[self.i]; self.i += 1;
            match c {
                b'"' => return Some(String::from_utf8_lossy(&out).into_owned()),
                b'\\' => {
                    let e = *self.b.get(self.i)?; self.i += 1;
                    match e {
                        b'n' => out.push(b'\n'), b't' => out.push(b'\t'), b'r' => out.push(b'\r'),
                        b'b' => out.push(0x08), b'f' => out.push(0x0c), b'"' => out.push(b'"'),
                        b'\\' => out.push(b'\\'), b'/' => out.push(b'/'),
                        b'u' => {
                            if self.i + 4 <= self.b.len() {
                                if let Ok(hex) = std::str::from_utf8(&self.b[self.i..self.i + 4]) {
                                    if let Ok(cp) = u32::from_str_radix(hex, 16) {
                                        if let Some(ch) = char::from_u32(cp) {
                                            let mut buf = [0u8; 4];
                                            out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                                        }
                                    }
                                }
                                self.i += 4;
                            }
                        }
                        other => out.push(other),
                    }
                }
                _ => out.push(c),
            }
        }
        None
    }
    fn parse_number(&mut self) -> Option<JsonValue> {
        let start = self.i;
        while self.i < self.b.len() {
            match self.b[self.i] { b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E' => self.i += 1, _ => break }
        }
        let tok = std::str::from_utf8(&self.b[start..self.i]).ok()?;
        tok.parse::<f64>().ok().map(JsonValue::Num)
    }
    fn parse_array(&mut self) -> Option<JsonValue> {
        self.i += 1;
        let mut arr = Vec::new();
        loop {
            self.skip_ws();
            if self.i >= self.b.len() { return None; }
            if self.b[self.i] == b']' { self.i += 1; break; }
            arr.push(self.parse_value()?);
        }
        Some(JsonValue::Arr(arr))
    }
    fn parse_object(&mut self) -> Option<JsonValue> {
        self.i += 1;
        let mut pairs = Vec::new();
        loop {
            self.skip_ws();
            if self.i >= self.b.len() { return None; }
            if self.b[self.i] == b'}' { self.i += 1; break; }
            let k = self.parse_string()?;
            self.skip_ws();
            if self.b.get(self.i) != Some(&b':') { return None; }
            self.i += 1;
            let v = self.parse_value()?;
            pairs.push((k, v));
        }
        Some(JsonValue::Obj(pairs))
    }
}

// ===========================================================================
//  모드 아이템 레지스트리 (dump_mod_items 가 서버시작때 1회 채움, scrim 포팅)
// ===========================================================================
static MOD_REGISTRY: Mutex<Vec<String>> = Mutex::new(Vec::new()); // idx i → key (게임 ID = 30+i)
static MOD_FINALS: Mutex<Vec<u64>> = Mutex::new(Vec::new());       // next_tier 빈 모드템 ID
static MOD_BUF: AtomicU64 = AtomicU64::new(0);   // mod_items 배열 base (element = MOD_BUF + i*stride, key@element+0)
static MOD_STRIDE: AtomicU64 = AtomicU64::new(0);
static NT_OFFSET: AtomicUsize = AtomicUsize::new(0);
static MODITEMS_DONE: AtomicBool = AtomicBool::new(false);
// ★0.5.2: ModItemEntry +0x190 = 활성 플래그(!=0 활성 / ==0 비활성). idx i → 활성여부.
//   근거 = 게임 자신의 Debug impl(0x21a0c10)이 이 필드로 "ModItemEntry(<id>, active|inactive)" 문자열 분기
//   (cmp qword [rcx+0x190],0 / sete / cmove ", inactive" vs ", active"). 독립확증 = 0x1408f0870 이
//   mod_items 배열을 돌며 [rsi+0x190]==0 인 엔트리만 처리하는 루프.
//   ⚠구 규칙 "mod_items Vec 존재 = 활성"(2026-07-05 실증)은 0.5.2에서 사망 — 비활성 모드 아이템도
//   같은 Vec에 inactive 로 들어옴(게임은 도감에서 거름, 우리는 못 걸러 노출된 것이 이 필드 도입 계기).
static MOD_ACTIVE: Mutex<Vec<bool>> = Mutex::new(Vec::new());
const MODITEM_ACTIVE_OFF: usize = 0x190;

// JSON 바닐라 30개 key (순서 = ID 0..29). 메모리 마스터목록 검증용 지문.
const VANILLA_KEYS: [&str; 30] = [
    "iron_blade","soldiers_longsword","ruinous_blade","conquerors_greatsword","warlords_final_judgement",
    "dagger","wind_dagger","twin_stormblade","thunderclaw","storm_sovereign",
    "steel_armor","gatekeepers_armor","black_knights_heavy_plate","eternal_iron_plate","impregnable_fortress",
    "mystic_cloak","night_hood","dusk_raven","souls_edge","veil_of_annihilation",
    "arcane_crystal","spirit_crystal","staff_of_rapture","angels_fang","prophet_of_the_abyss",
    "vital_orb","hardened_heart","ring_of_reincarnation","hourglass_of_eternity","giants_horn_shard",
];

// Database mod_items Vec 를 메모리 스캔 → MOD_REGISTRY/MOD_FINALS 채움. (scrim dump_mod_items 포팅)
unsafe fn dump_mod_items(db: usize) {
    if MODITEMS_DONE.swap(true, Ordering::Relaxed) { return; }
    seh_install();
    let mut s = format!("[{}ms] mod_items walk (db={:#x})\n", now_ms(), db);

    let key_at = |pa: usize| -> Option<String> {
        let ptr = safe_read_u64(pa)? as usize;
        if ptr <= 0x10000 { return None; }
        for &m in &[64usize, 32, 16, 8] {
            let mut b = Vec::new();
            if !safe_read_bytes(ptr, m, &mut b) { continue; }
            let mut v = Vec::new();
            for &c in b.iter() { if c == b'_' || c.is_ascii_alphanumeric() { v.push(c); } else { break; } }
            if v.len() >= 3 && (v[0] as char).is_ascii_alphabetic() { return String::from_utf8(v).ok(); }
        }
        None
    };
    let is_vanilla = |k: &str| k == "ironsword" || VANILLA_KEYS.contains(&k);
    let item_strides: [usize; 3] = [0x1a8, 0x198, 0x1b0];
    let detect_stride = |buf: usize| -> usize {
        for &st in item_strides.iter() {
            let k: Vec<Option<String>> = (0..4).map(|i| key_at(buf + i * st + 0x8)).collect();
            if k.iter().all(|x| x.is_some()) && k[0] != k[1] && k[1] != k[2] && k[2] != k[3] { return st; }
        }
        0
    };
    let mut found: Vec<(usize, usize, usize, usize)> = Vec::new();
    let mut o = 0usize;
    while o + 0x18 <= 0x60000 && found.len() < 16 {
        let a = db + o; o += 8;
        let (Some(q0), Some(q1), Some(q2)) = (safe_read_u64(a), safe_read_u64(a + 8), safe_read_u64(a + 0x10)) else { continue; };
        for &(p, c) in [(q1, q0), (q1, q2), (q0, q2), (q0, q1)].iter() {
            let (p, c) = (p as usize, c as usize);
            if !looks_heap(p as u64) || c < 3 || c > 2000 { continue; }
            let Some(k0) = key_at(p + 0x8) else { continue; };
            if is_vanilla(&k0) { continue; }
            let cst = detect_stride(p);
            if cst == 0 { continue; }
            let probe = c.min(48);
            let valid = (0..probe).filter(|&i| key_at(p + i * cst + 0x8).is_some()).count();
            if valid * 10 < probe * 8 || valid < 3 { continue; }
            if found.iter().any(|&(b, _, _, _)| b == p) { continue; }
            found.push((p, c, cst, a));
        }
    }
    if found.is_empty() {
        s.push_str("  ✗ 비바닐라 item-struct 배열 못 찾음 (모드 아이템 미적용?)\n");
        write_log("item_tactics_moditems.txt", &s);
        // ★08-07 구멍3 — 실패했으면 재시도 가능하게 되돌린다(구: 첫 줄에서 DONE 을 세워 **세션 내내 재시도 없음**
        //   ⟹ 스캔이 한 번 어긋나면 모드템 지정이 전부 조용히 사망). 상한 10회로 비용 제한.
        MODITEMS_FAIL_WHY.store(1, Ordering::Relaxed);
        if MODITEMS_TRIES.fetch_add(1, Ordering::Relaxed) < 500 { MODITEMS_DONE.store(false, Ordering::Relaxed); }
        return;
    }
    found.sort_by(|x, y| y.1.cmp(&x.1));
    let key_of_elem = |elem: usize| -> Option<String> {
        let a = safe_read_u64(elem)? as usize;
        let ptr = safe_read_u64(elem + 8)? as usize;
        let c = safe_read_u64(elem + 0x10)? as usize;
        let len = a.min(c);
        if ptr <= 0x10000 || len < 2 || len > 48 { return None; }
        let mut b = Vec::new();
        if !safe_read_bytes(ptr, len, &mut b) { return None; }
        if b.iter().all(|&x| x == b'_' || x.is_ascii_alphanumeric()) && (b[0] as char).is_ascii_alphabetic() {
            String::from_utf8(b).ok()
        } else { None }
    };
    // read_nt: elem 의 next_tier Vec(오프셋 o) 를 key 리스트로 읽음. (아이템 트리 판별 핵심)
    let read_nt = |elem: usize, o: usize| -> Option<Vec<String>> {
        let len = safe_read_u64(elem + o)? as usize;
        if len == 0 { return Some(Vec::new()); }
        if len > 8 { return None; }
        let ptr = safe_read_u64(elem + o + 8)? as usize;
        let cap = safe_read_u64(elem + o + 0x10)? as usize;
        if ptr <= 0x10000 || cap < len { return None; }
        let mut out = Vec::new();
        for j in 0..len { out.push(key_of_elem(ptr + j * 0x18)?); }
        Some(out)
    };
    // 후보 배열의 key 리스트 추출.
    let build_keys = |buf: usize, st: usize, hdr_cnt: usize| -> Vec<String> {
        let mut keys = Vec::new();
        let mut cnt = 0usize;
        while cnt < hdr_cnt.max(1) && cnt < 500 {
            if let Some(k) = key_of_elem(buf + cnt * st) { keys.push(k); cnt += 1; } else { break; }
        }
        keys
    };
    // 후보 배열의 최적 next_tier 오프셋 + votes(아이템 트리 강도). 선수/챔피언 배열은 votes 낮음.
    let best_nt = |buf: usize, st: usize, keys: &[String]| -> (usize, u32) {
        let mut best_off = 0usize; let mut best_votes = 0u32;
        let mut o = 0x18usize;
        while o + 0x18 <= st {
            let mut votes = 0u32;
            for i in 0..keys.len() {
                if let Some(v) = read_nt(buf + i * st, o) {
                    if !v.is_empty() && v.iter().all(|k| keys.iter().any(|x| x.as_str() == k.as_str())) { votes += 1; }
                }
            }
            if votes > best_votes { best_votes = votes; best_off = o; }
            o += 8;
        }
        (best_off, best_votes)
    };
    // ★ 후보 중 next_tier(아이템 트리) 를 가진 배열을 채택 (count 최대만 고르던 버그 수정 —
    //   선수/챔피언 모드 배열이 아이템보다 커도 아이템을 정확히 선택. 2026-07-04).
    // ★채택 규칙(07-22 강화): 구 규칙은 "votes≥3인 **첫** 후보"(= found 의 cnt 내림차순 1등)라
    //   비활성 모드의 대기 배열이 활성 병합 배열보다 크면 그쪽을 집는다. → **활성 엔트리 수를
    //   1순위 기준**으로 바꿈(활성이 실제로 있는 배열이 곧 게임이 쓰는 배열). 동수면 cnt 큰 쪽.
    //   전 후보가 활성 0이면(=아이템 모드를 하나도 안 켠 정상 상태) 구 규칙대로 cnt 1등 채택.
    let mut diag = String::from("  --- 후보 스캔(전부) ---\n");
    let mut cands: Vec<(usize, usize, Vec<String>, usize, u32, usize)> = Vec::new();
    for &(fbuf, fcnt, fst, _) in &found {
        let keys = build_keys(fbuf, fst, fcnt);
        let (bo, bv) = best_nt(fbuf, fst, &keys);
        let act = (0..keys.len())
            .filter(|&i| safe_read_u64(fbuf + i * fst + MODITEM_ACTIVE_OFF).map(|v| v != 0).unwrap_or(false))
            .count();
        diag.push_str(&format!("  buf={:#x} cnt={} stride={:#x} first={:?} nt_off={:#x} votes={} 활성={}\n",
            fbuf, keys.len(), fst, keys.first(), bo, bv, act));
        if bv >= 3 { cands.push((fbuf, fst, keys, bo, bv, act)); }
    }
    // 활성수 desc → cnt desc (found 가 이미 cnt desc 라 안정정렬로 동수 시 원순서 유지)
    cands.sort_by(|a, b| b.5.cmp(&a.5));
    let chosen = cands.into_iter().next().map(|(b, st, k, o, v, _)| (b, st, k, o, v));
    let Some((buf, st, keys, best_off, best_votes)) = chosen else {
        s.push_str("  ✗ 아이템 트리(next_tier) 가진 배열 없음 → 아이템 모드 미로드/미인식 의심\n");
        s.push_str(&diag);
        write_log("item_tactics_moditems.txt", &s);
        MODITEMS_FAIL_WHY.store(2, Ordering::Relaxed);
        if MODITEMS_TRIES.fetch_add(1, Ordering::Relaxed) < 500 { MODITEMS_DONE.store(false, Ordering::Relaxed); } // ★08-07 구멍3(상한 10→500: 관리틱이 빨라 병합 전 소진)
        return;
    };
    let cnt = keys.len();
    MOD_BUF.store(buf as u64, Ordering::Relaxed); MOD_STRIDE.store(st as u64, Ordering::Relaxed);
    NT_OFFSET.store(best_off, Ordering::Relaxed);
    {
        let mut reg = MOD_REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
        reg.clear();
        for k in keys.iter() { reg.push(k.clone()); }
    }
    // ★활성 플래그 수집(+0x190). 읽기 실패한 엔트리는 true(활성)로 폴백 — 못 읽었다고 목록에서
    //   지워버리면 유저 지정이 조용히 사라지므로, 불확실할 땐 노출 쪽이 안전.
    let actives: Vec<bool> = (0..cnt)
        .map(|i| safe_read_u64(buf + i * st + MODITEM_ACTIVE_OFF).map(|v| v != 0).unwrap_or(true))
        .collect();
    let n_act = actives.iter().filter(|&&a| a).count();
    *MOD_ACTIVE.lock().unwrap_or_else(|e| e.into_inner()) = actives.clone();
    s.push_str(&format!("  [채택] buf={:#x} cnt={} stride={:#x} nt_off={:#x} votes={} 활성={}/{}\n  idx | ID | act | key\n",
        buf, cnt, st, best_off, best_votes, n_act, cnt));
    for (i, k) in keys.iter().enumerate() {
        s.push_str(&format!("  {:>3} | {:>3} | {} | {}\n", i, 30 + i,
            if actives.get(i).copied().unwrap_or(true) { "O" } else { "X" }, k));
    }
    s.push_str(&diag);
    write_log("item_tactics_moditems.txt", &s);
    // ★ 1차: 모든 next_tier 타겟 수집(built_set) — 무언가 이 아이템으로 빌드되면 = 진짜 최종후보.
    //   (needlessly_large_rod 같은 베이스 컴포넌트는 next_tier 비었지만 타겟도 아니라 제외됨.)
    let mut built: std::collections::HashSet<String> = std::collections::HashSet::new();
    for i in 0..cnt {
        if let Some(nt) = read_nt(buf + i * st, best_off) { for k in nt { built.insert(k); } }
    }
    let mut finals: Vec<u64> = Vec::new();
    let mut tree = format!("[{}ms] next_tier offset=+{:#x} votes={}/{} built_targets={}\n", now_ms(), best_off, best_votes, cnt, built.len());
    for i in 0..cnt {
        let elem = buf + i * st;
        let k = key_of_elem(elem).unwrap_or_default();
        // ★ 핸드오프 §3 수정: read_nt 를 match 로 분기. None(그 오프셋서 next_tier 판정불가)은
        //   최종에서 제외 (기존 unwrap_or_default() 는 None 을 빈Vec 으로 오인 → 최종템 오판. override 시 실발생).
        match read_nt(elem, best_off) {
            Some(nt) if nt.is_empty() => {
                if built.contains(&k) { finals.push(30 + i as u64); tree.push_str(&format!("  {:>3} {} ★최종\n", 30 + i, k)); }
                else { tree.push_str(&format!("  {:>3} {} (베이스컴포넌트-제외)\n", 30 + i, k)); }
            }
            Some(nt) => { tree.push_str(&format!("  {:>3} {} → {}\n", 30 + i, k, nt.join(", "))); }
            None => { tree.push_str(&format!("  {:>3} {} (next_tier 판정불가-제외)\n", 30 + i, k)); }
        }
    }
    tree.push_str(&format!("  → 최종템 {}개: {:?}\n", finals.len(), finals));
    *MOD_FINALS.lock().unwrap_or_else(|e| e.into_inner()) = finals;
    write_log("item_tactics_itemtree.txt", &tree);
}

// ===========================================================================
//  활성 모드 아이템 필터 (scrim 포팅) — mods.json enabled_mods × 각 모드 text/item.i18n
// ===========================================================================
fn enabled_mods() -> Vec<String> {
    let mut out = Vec::new();
    let Some(root) = game_root() else { return out; };
    let Ok(txt) = fs::read_to_string(root.join("config").join("game").join("mods.json")) else { return out; };
    if let Some(p) = txt.find("\"enabled_mods\"") {
        if let Some(lb) = txt[p..].find('[') {
            let start = p + lb + 1;
            if let Some(rb) = txt[start..].find(']') {
                for part in txt[start..start + rb].split(',') {
                    let s = part.trim().trim_matches('"').trim();
                    if !s.is_empty() { out.push(s.to_string()); }
                }
            }
        }
    }
    out
}
fn build_active_item_keys() -> std::collections::HashSet<String> {
    let mut set = std::collections::HashSet::new();
    let enabled = enabled_mods();
    if enabled.is_empty() { return set; }
    let Some(root) = game_root() else { return set; };
    let mut dirs: Vec<PathBuf> = Vec::new();
    if let Ok(rd) = fs::read_dir(root.join("mods")) { for e in rd.flatten() { dirs.push(e.path()); } }
    if let Some(ws) = root.parent().and_then(|p| p.parent()).map(|p| p.join("workshop").join("content").join("3009300")) {
        if let Ok(rd) = fs::read_dir(&ws) { for e in rd.flatten() { dirs.push(e.path()); } }
    }
    for d in dirs {
        let Ok(info) = fs::read_to_string(d.join("mod.mod_info")) else { continue; };
        let Some(iv) = JsonParser::new(&info).parse_value() else { continue; };
        let Some(mid) = iv.get("mod_id").and_then(|x| x.as_str()) else { continue; };
        if !enabled.iter().any(|e| e == mid) { continue; }
        let Ok(i18n) = fs::read_to_string(d.join("text").join("item.i18n")) else { continue; };
        if let Some(JsonValue::Obj(langs)) = JsonParser::new(&i18n).parse_value() {
            for (_, lobj) in langs {
                if let JsonValue::Obj(items) = lobj { for (k, _) in items { set.insert(k); } }
            }
        }
    }
    set
}
static ACTIVE_KEYS: Mutex<Option<std::collections::HashSet<String>>> = Mutex::new(None);
fn active_item_keys() -> std::collections::HashSet<String> {
    {
        let g = ACTIVE_KEYS.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(s) = g.as_ref() { return s.clone(); }
    }
    let set = build_active_item_keys();
    *ACTIVE_KEYS.lock().unwrap_or_else(|e| e.into_inner()) = Some(set.clone());
    set
}

// 동적 최종템 전체 = (게임ID, key). MOD_FINALS(next_tier 빈것) → MOD_REGISTRY 로 key 매핑.
fn mod_final_opts_all() -> Vec<(u64, String)> {
    let finals = MOD_FINALS.lock().unwrap_or_else(|e| e.into_inner());
    let reg = MOD_REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    finals.iter().filter_map(|&id| {
        let i = (id as usize).checked_sub(30)?;
        reg.get(i).map(|k| (id, k.clone()))
    }).collect()
}
// picker 노출 최종템 = DB 스캔 결과 중 ★활성(+0x190 != 0)만.
//   ~~구: 스캔 결과 그대로(비활성은 애초에 병합 안 됨)~~ → 0.5.2에서 무효(2026-07-22):
//   비활성 모드템도 같은 Vec에 inactive 로 들어와 드롭다운에 노출되는 증상 발생(유저 확인:
//   게임 도감에는 안 뜸 = 게임은 거르는데 우리만 못 걸렀음). 게임 Debug impl 과 동일 기준으로 미러링.
//   ⚠fail-safe = **플래그 미수집(스캔 전) 한 겹만**. 초판(07-22)에 "전부 비활성이면 판정오류 의심
//   → 무필터"를 넣었다가 **정답을 뒤집었다**: 아이템 추가 모드를 하나도 안 켠 환경에서는 활성 0개가
//   정상인데(실측: 활성모드 map_free/leefs_variety*/banpick_illust 전부 item.i18n 없음), 그걸
//   오판으로 보고 비활성 104개를 도로 노출 = 증상 그대로 재현. **활성 0 = 유효한 상태**로 취급한다.
//   (빈 목록이어도 바닐라 카테고리 7종은 항상 남으므로 드롭다운이 통째로 비지 않음.)
fn mod_final_opts() -> Vec<(u64, String)> {
    let all = mod_final_opts_all();
    let act = MOD_ACTIVE.lock().unwrap_or_else(|e| e.into_inner());
    if act.is_empty() { return all; }   // 아직 스캔 전 = 판정 불가 → 무필터
    all.iter()
        .filter(|(id, _)| (*id as usize).checked_sub(30)
            .and_then(|i| act.get(i).copied()).unwrap_or(true))
        .cloned().collect()
}
// picker 총 옵션 수 = 자동(1) + 카테고리6 + 동적 최종템
fn item_opt_count() -> usize { 7 + mod_final_opts().len() }
// picker 값 v → 라벨. 0~6=고정, 7+=모드 최종템(게임 i18n 참조 → 자동 현지화).
fn item_opt_label(v: u8) -> String {
    let vi = v as usize;
    if vi < 7 { return VANILLA_OPTS[vi].to_string(); }
    match mod_final_opts().get(vi - 7) {
        Some((_, key)) => format!("#asset/base/text/item?{}.name", key),
        None => VANILLA_OPTS[0].to_string(),
    }
}

// ===========================================================================
//  모드 상태
// ===========================================================================
static SCREEN_OPEN: AtomicBool = AtomicBool::new(false);
static OPTS_INJECTED: AtomicBool = AtomicBool::new(false);
static LAST_SEL: Mutex<[i64; MAX_ROWS * ITEM_SLOTS]> = Mutex::new([-1i64; MAX_ROWS * ITEM_SLOTS]);
// ★ (챔프키, slot) → 선택 옵션 인덱스. champ-keyed 라 매치마다 라인업 바뀌어도 챔프별 유지.
//   idx 0~6=바닐라 카테고리, 7+=모드템(mod_final_opts[idx-7]). 영속(item_tactics_sel.txt).
static SEL_BY_CHAMP: Mutex<Option<HashMap<(String, u8), u8>>> = Mutex::new(None);
static SEL_LOADED: AtomicBool = AtomicBool::new(false);
// ★ 게임 personal_tactics 스냅샷: champ → [카테고리 3바이트(0~6)]. NOP로 끊긴 바닐라 표시 복원용.
//   post_update(InGame, personal 화면)서 db().team(pid).champion_personal_tactics 로 갱신.
static PT_SNAPSHOT: Mutex<Option<HashMap<String, [u8; 3]>>> = Mutex::new(None);
static DIAG_DONE: AtomicBool = AtomicBool::new(false);

// #champion/#icon ImageRunner 소스에서 챔프키 추출
//   "asset/base/aseprite_resources/champions/{champ}#sheet" → champ
fn row_champ(row: &Node) -> Option<String> {
    let icon = find_node(row, "icon")?;
    let src = unsafe { read_img_source(icon) }?;
    let a = src.find("champions/")? + "champions/".len();
    let rest = &src[a..];
    let end = rest.find('#').unwrap_or(rest.len());
    let champ = rest[..end].trim();
    if champ.is_empty() { None } else { Some(champ.to_string()) }
}
fn sel_path() -> Option<PathBuf> { Some(mod_dir()?.join("item_tactics_sel.txt")) }

// ═══════════════════════════════════════════════════════════════════════════
//  ★조합테스트 진영 스코프 (2026-07-30 신설)
// ═══════════════════════════════════════════════════════════════════════════
//  문제: SEL 키가 (챔프, 슬롯) 뿐이라 **조합테스트에서 양 진영에 같은 챔프**를 놓으면 지정이
//    하나로 합쳐졌다(뒤에 만진 쪽이 앞을 덮고, 재시드가 두 행을 같은 값으로 동기화). 게다가
//    조합테스트 지정이 같은 저장소를 쓰므로 **리그·배경 경기의 그 챔프에도 그대로 새 나갔다.**
//  해결: SEL 키의 champ 칼럼에 **스코프 접두**를 붙인다. HashMap 타입·파일 포맷(`champ slot token`
//    공백 3칼럼)·SEL_PENDING 구조를 **전부 그대로 두고** 키 문자열만 확장하는 방식이라 파급이 없다.
//    - 일반(리그/관전/배경) = 접두 없음 ⟹ **기존 파일이 그대로 유효**(레거시 호환)
//    - 조합테스트 블루 = `@b:` / 레드 = `@r:`
//    champ 는 asset key(공백·`@` 없음)라 접두가 이름과 충돌하지 않고, 구버전 dll이 이 파일을 읽어도
//    "그런 챔프 없음"으로 무시될 뿐이다(다운그레이드 안전).
const CT_PFX_B: &str = "@b:";
const CT_PFX_R: &str = "@r:";
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Scope { Plain, CtBlue, CtRed }
fn scope_pfx(s: Scope) -> &'static str {
    match s { Scope::Plain => "", Scope::CtBlue => CT_PFX_B, Scope::CtRed => CT_PFX_R }
}
fn scoped_key(s: Scope, champ: &str) -> String {
    match s { Scope::Plain => champ.to_string(), _ => format!("{}{}", scope_pfx(s), champ) }
}
fn is_scoped(k: &str) -> bool { k.starts_with(CT_PFX_B) || k.starts_with(CT_PFX_R) }
// 스코프 접두를 벗긴 순수 챔프 이름(지정 여부 판정·side 투표는 스코프 무관해야 한다).
fn strip_scope(k: &str) -> &str {
    if let Some(r) = k.strip_prefix(CT_PFX_B) { return r; }
    if let Some(r) = k.strip_prefix(CT_PFX_R) { return r; }
    k
}
// ★명시 Auto 센티널: 스코프 키에서만 쓴다. 조합테스트에서 어떤 칸을 Auto 로 되돌렸을 때
//   엔트리를 그냥 지우면 **접두 없는(일반) 지정이 폴백으로 되살아나** 유저 눈엔 "안 바뀜"이 된다.
//   ⟹ "이 진영 이 칸은 지정 없음"을 명시 기록해 폴백을 차단한다. 파일 토큰 = `auto`.
const SEL_AUTO: u8 = 255;
const SEL_AUTO_TOKEN: &str = "auto";
// ★영속 포맷 = 아이템 "키" 기반 (2026-07-22 전환).
//   구 포맷은 드롭다운 옵션 인덱스(u8)를 그대로 저장 → 목록 구성이 바뀌면(모드 on/off, 활성필터
//   도입 등) 저장된 지정이 전부 다른 아이템으로 어긋남. 인메모리는 여전히 인덱스지만, 파일에는
//   `1`~`6`(바닐라 카테고리) 또는 모드템 key 문자열을 쓴다.
//   레거시(7 이상의 숫자) = 구 인덱스 → **무필터 목록** 기준으로 key 해석(그 지정을 만들 때의 목록과
//   동일 구성)한 뒤 현재 인덱스로 재매핑.
// ⚠아이템 레지스트리는 서버시작 dump_mod_items 후에야 채워지는데 SEL 로드는 지연(lazy)이라
//   먼저 일어날 수 있다 → 해석 못 한 항목은 버리지 않고 SEL_PENDING 에 원문 보관, 레지스트리가
//   준비되면 흡수. save 시 pending 도 원문 그대로 함께 기록 = **어떤 경우에도 유실 없음**.
static SEL_PENDING: Mutex<Vec<(String, u8, String)>> = Mutex::new(Vec::new());
static SEL_PENDING_ANY: AtomicBool = AtomicBool::new(false);
fn registry_ready() -> bool { !MOD_REGISTRY.lock().unwrap_or_else(|e| e.into_inner()).is_empty() }
// 현재 노출 목록에서 key → 옵션 인덱스(7+).
fn key_to_opt_index(key: &str) -> Option<u8> {
    mod_final_opts().iter().position(|(_, k)| k == key).map(|i| (i + 7) as u8)
}
// ★레거시 숫자 인덱스(≥7)를 가능하면 key 로 승격. 승격 못 하면 원문 유지.
//   pending 에 숫자를 그대로 두면 "그때의 목록 구성"에 의존한 채 남아, 나중에 모드 구성이 바뀐 뒤
//   해석될 때 엉뚱한 아이템을 가리킨다 → 해석 가능한 시점에 즉시 key 로 고정.
fn normalize_token(tok: &str) -> String {
    if let Ok(n) = tok.parse::<u8>() {
        if n >= 7 && registry_ready() {
            if let Some((_, k)) = mod_final_opts_all().get(n as usize - 7) { return k.clone(); }
        }
    }
    tok.to_string()
}
// 파일 토큰 → 옵션 인덱스.
fn token_to_opt_index(tok: &str) -> Option<u8> {
    if tok == SEL_AUTO_TOKEN { return Some(SEL_AUTO); } // ★명시 Auto(스코프 전용) — 레지스트리 불요
    if let Ok(n) = tok.parse::<u8>() {
        if n == 0 { return None; }
        if n < 7 { return Some(n); }                       // 바닐라 카테고리 1~6
        if !registry_ready() { return None; }              // 레거시 인덱스인데 아직 해석 불가
        let key = mod_final_opts_all().get(n as usize - 7).map(|(_, k)| k.clone())?;
        return key_to_opt_index(&key);
    }
    if !registry_ready() { return None; }
    key_to_opt_index(tok)
}
// 옵션 인덱스 → 파일 토큰.
fn opt_index_to_token(idx: u8) -> Option<String> {
    if idx == 0 { return None; }
    if idx == SEL_AUTO { return Some(SEL_AUTO_TOKEN.to_string()); } // ★명시 Auto 는 반드시 보존
    if idx < 7 { return Some(idx.to_string()); }
    mod_final_opts().get(idx as usize - 7).map(|(_, k)| k.clone())
}
fn load_sel() -> HashMap<(String, u8), u8> {
    let mut m = HashMap::new();
    let mut pend = Vec::new();
    let mut legacy = false;
    if let Some(p) = sel_path() {
        if let Ok(txt) = fs::read_to_string(&p) {
            for line in txt.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() != 3 { continue; }
                let Ok(slot) = parts[1].parse::<u8>() else { continue; };
                let tok = parts[2];
                if tok.parse::<u8>().map(|n| n >= 7).unwrap_or(false) { legacy = true; }
                let tok = normalize_token(tok); // 해석 가능하면 key 로 고정 후 보관
                match token_to_opt_index(&tok) {
                    // idx 0(맡김)=오버라이드 아님 → delegate 폴백. 저장/로드 안 함(스퓨리어스 0 제거).
                    Some(idx) if idx >= 1 => { m.insert((parts[0].to_string(), slot), idx); }
                    Some(_) => {}
                    None => pend.push((parts[0].to_string(), slot, tok)),
                }
            }
            // 구 포맷을 처음 읽었으면 원본을 1회 백업(이관이 잘못돼도 되돌릴 수 있게).
            if legacy {
                if let Some(bp) = mod_dir().map(|d| d.join("item_tactics_sel.txt.bak_idxfmt")) {
                    if !bp.exists() { let _ = fs::write(bp, &txt); }
                }
            }
        }
    }
    SEL_PENDING_ANY.store(!pend.is_empty(), Ordering::Relaxed);
    *SEL_PENDING.lock().unwrap_or_else(|e| e.into_inner()) = pend;
    m
}
// 레지스트리 준비 후 pending 흡수. with_sel 안에서만 호출(SEL 락 보유 상태).
//   ★hot path 고려: with_sel 은 자주 불리므로 pending 이 비면 원자변수 1회 읽고 즉시 반환
//   (MOD_REGISTRY 락조차 잡지 않음).
fn drain_pending(m: &mut HashMap<(String, u8), u8>) {
    if !SEL_PENDING_ANY.load(Ordering::Relaxed) { return; }
    if !registry_ready() { return; }
    let mut pend = SEL_PENDING.lock().unwrap_or_else(|e| e.into_inner());
    if pend.is_empty() { return; }
    for e in pend.iter_mut() { e.2 = normalize_token(&e.2); } // 레지스트리 준비됨 → 숫자를 key 로 고정
    pend.retain(|(champ, slot, tok)| match token_to_opt_index(tok) {
        Some(idx) if idx >= 1 => { m.insert((champ.clone(), *slot), idx); false }
        Some(_) => false,
        None => true, // 여전히 해석 불가(예: 그 모드가 비활성) → 원문 보존
    });
    SEL_PENDING_ANY.store(!pend.is_empty(), Ordering::Relaxed);
}
fn save_sel(m: &HashMap<(String, u8), u8>) {
    let mut rows: Vec<(String, u8, String)> = m.iter()
        .filter_map(|((champ, slot), &idx)| opt_index_to_token(idx).map(|t| (champ.clone(), *slot, t)))
        .collect();
    // 아직 해석 못 한 항목도 원문 그대로 유지 → 유실 방지.
    rows.extend(SEL_PENDING.lock().unwrap_or_else(|e| e.into_inner()).iter().cloned());
    rows.sort();
    let mut s = String::new();
    for (champ, slot, tok) in rows { s.push_str(&format!("{} {} {}\n", champ, slot, tok)); }
    if let Some(p) = sel_path() { let _ = fs::write(p, s); }
}


// SEL_BY_CHAMP 접근(최초 1회 파일 로드). 클로저로 락 안에서 조작.
fn with_sel<R>(f: impl FnOnce(&mut HashMap<(String, u8), u8>) -> R) -> R {
    let mut g = SEL_BY_CHAMP.lock().unwrap_or_else(|e| e.into_inner());
    if g.is_none() { *g = Some(load_sel()); SEL_LOADED.store(true, Ordering::Relaxed); }
    let m = g.as_mut().unwrap();
    drain_pending(m); // 레지스트리가 늦게 준비돼도 여기서 흡수
    f(m)
}
// ★SEL 조회 단일 창구(스코프 인식). 스코프 키가 있으면 그것이 이기고, 없으면 일반 키로 폴백한다.
//   - `SEL_AUTO` = 그 진영 그 칸을 유저가 Auto 로 명시 ⟹ 0(지정없음) 으로 확정하고 폴백하지 않는다.
//   - Scope::Plain 이면 예전과 완전히 동일한 조회(일반 키만) = 리그·관전·배경 경기 동작 무변경.
fn sel_get(scope: Scope, champ: &str, si: u8) -> u8 {
    with_sel(|m| {
        if scope != Scope::Plain {
            if let Some(&v) = m.get(&(scoped_key(scope, champ), si)) {
                return if v == SEL_AUTO { 0 } else { v };
            }
        }
        m.get(&(champ.to_string(), si)).copied().unwrap_or(0)
    })
}
// ═══ 조합테스트 진영 판정 (buy detour 에서 athlete → 블루/레드) ═══
//  조합테스트 UI(handle_comptest_screen)가 행 구성을 게시하고, buy 는 그 스냅샷으로 진영을 정한다.
//  ①챔프가 한쪽 진영에만 있으면 그 진영 확정 + 그때 관측한 athlete+0x820(side) 값을 학습.
//  ②양 진영에 같은 챔프가 있으면 ①에서 학습한 side 값으로 구분.
//  ③학습 전이거나 판정 불가면 Scope::Plain(=기존 동작) 폴백 ⟹ 어떤 경우에도 회귀는 없다.
//  ⚠side 값(0/1)이 블루/레드 중 어느 쪽인지는 **하드코딩하지 않는다**(실측 미확정). 겹치지 않는
//    챔프 1명만 사도 학습되므로, 10명 전원이 같은 챔프인 극단 케이스에서만 ③으로 떨어진다.
//  ※스냅샷은 OVERRIDE_SNAPSHOT 과 동일한 leak 패턴(병렬 detour가 읽는 중일 수 있어 free 금지).
type CtRoster = (std::collections::HashSet<String>, std::collections::HashSet<String>); // (blue, red)
static CT_ROSTER: AtomicPtr<CtRoster> = AtomicPtr::new(core::ptr::null_mut());
static CT_SIDE_B: AtomicU64 = AtomicU64::new(u64::MAX); // 블루로 학습된 athlete+0x820 값
static CT_SIDE_R: AtomicU64 = AtomicU64::new(u64::MAX); // 레드로 학습된 값
fn publish_ct_roster(blue: std::collections::HashSet<String>, red: std::collections::HashSet<String>) {
    let cur = CT_ROSTER.load(Ordering::Acquire);
    if !cur.is_null() {
        let c = unsafe { &*cur };
        if c.0 == blue && c.1 == red { return; } // 무변경 → 재게시 안 함(leak 유한)
    }
    CT_ROSTER.store(Box::into_raw(Box::new((blue, red))), Ordering::Release);
}
fn ct_scope_for(champ: &str, side: u64) -> Scope {
    let p = CT_ROSTER.load(Ordering::Acquire);
    if p.is_null() { return Scope::Plain; }
    let (blue, red) = unsafe { &*p };
    let (in_b, in_r) = (blue.contains(champ), red.contains(champ));
    match (in_b, in_r) {
        (true, false) => { if side != u64::MAX { CT_SIDE_B.store(side, Ordering::Relaxed); } Scope::CtBlue }
        (false, true) => { if side != u64::MAX { CT_SIDE_R.store(side, Ordering::Relaxed); } Scope::CtRed }
        (true, true) => { // 양 진영 동일 챔프 → 학습된 side 값으로만 구분
            if side != u64::MAX {
                if side == CT_SIDE_B.load(Ordering::Relaxed) { return Scope::CtBlue; }
                if side == CT_SIDE_R.load(Ordering::Relaxed) { return Scope::CtRed; }
            }
            Scope::Plain
        }
        (false, false) => Scope::Plain, // 조합테스트 구성에 없는 챔프(=화면 미관측) → 기존 동작
    }
}
// ★성능(0.5.1): 지정챔프 판정 = SEL 스냅샷(zero-alloc contains). 매 buy당 with_sel 락+4× champ.to_string() 할당 제거.
//   SEL은 드롭다운 변경 때만 바뀜 → SEL_DIRTY로 무효화, 스냅샷은 그때만 재빌드. 읽기=Arc 짧은 clone 후 락밖 contains.
// ★스코프 접두는 **벗겨서** 담는다(2026-07-30): 조합테스트 전용 지정만 있는 챔프도 designated 로
//   잡혀야 buy 가 조회 경로에 진입한다. 접두째로 담으면 그 챔프가 영구히 무시된다.
static DESIGNATED_SNAP: Mutex<Option<std::sync::Arc<std::collections::HashSet<String>>>> = Mutex::new(None);
static SEL_DIRTY: AtomicBool = AtomicBool::new(true);
fn designated_set() -> std::collections::HashSet<String> {
    with_sel(|m| m.keys().map(|(c, _)| strip_scope(c).to_string()).collect())
}
fn is_champ_designated(champ: &str) -> bool {
    if SEL_DIRTY.swap(false, Ordering::Relaxed) {
        *DESIGNATED_SNAP.lock().unwrap_or_else(|e| e.into_inner()) = Some(std::sync::Arc::new(designated_set()));
    }
    let snap = { DESIGNATED_SNAP.lock().unwrap_or_else(|e| e.into_inner()).clone() };
    match snap {
        Some(s) => s.contains(champ),
        None => { // 최초 레이스(스냅샷 아직 없음) → 강제 빌드
            let arc = std::sync::Arc::new(designated_set());
            let hit = arc.contains(champ);
            *DESIGNATED_SNAP.lock().unwrap_or_else(|e| e.into_inner()) = Some(arc);
            hit
        }
    }
}
static MH_DIAG_DONE: AtomicBool = AtomicBool::new(false);
// 화면 진입당 1회 계산한 옵션 라벨 캐시(per-frame 파일I/O 회피).
static OPTS_CACHE: Mutex<Vec<String>> = Mutex::new(Vec::new());

fn compute_options() -> Vec<String> {
    let n = item_opt_count();
    (0..n).map(|i| item_opt_label(i as u8)).collect()
}

// ★모드소유 아이템 드롭다운 id. slot0/1/2=item{N}m(네이티브 item0/1/2 위 오버레이, 클릭이 +0x1788 직접 커밋),
//   slot3=item3(4번째). 네이티브 item0/1/2는 클릭이 모델에만 커밋돼 모드템 불가(ghidra-re) → 모드소유로 대체.
fn slot_dd_id(si: usize) -> String {
    if si >= 3 { "item3".to_string() } else { format!("item{}m", si) }
}

// ★네이티브 item0/1/2 드롭다운 숨김(모드소유 item0m/1m/2m가 대체). 개인전술 화면 열렸을 때만.
//   오버레이만으론 네이티브 "선수에게 맡김" 텍스트가 왼쪽에 비쳐 겹쳐보임 → visible=false로 완전 숨김.
//   게임이 매프레임 visible 리셋할 수 있어 post_update마다 재적용.
fn hide_native_item_dds(root: &mut Node) {
    if !find_node(root, "personal").map(|n| n.visible).unwrap_or(false) { return; } // 개인전술 아닐 때 미개입
    for ri in 0..MAX_ROWS {
        let rid = format!("row{}", ri);
        if let Some(row) = find_mut(root, &rid) {
            for si in 0..3u8 {
                if let Some(nat) = find_mut(row, &format!("item{}", si)) { nat.visible = false; }
            }
        }
    }
}

// ===========================================================================
//  개인전술 화면 처리 (post_update 마다)
// ===========================================================================
// ═══════════════════════════════════════════════════════════════════════════
//  조합테스트(training.ui) 개인전술 — 4번째 칸 아이템 지정
//    1~3칸(item0/1/2) = 게임 네이티브 소관(및 조합테스트 제한패치 모드). 우리는 item3만 소유.
//    저장소 = SEL_BY_CHAMP(전술화면과 동일, 챔프 기준) ⟹ 여기서 지정하면 실경기·조합테스트 sim
//    양쪽에 그대로 적용된다(buy 주입이 챔프명 키라 별도 배선 불요).
//    행 = blue0..4 / red0..4 (10명). UI 4칸화는 ui_inject::inject_training이 담당.
// ═══════════════════════════════════════════════════════════════════════════
const CT_ROWS: [&str; 10] = ["blue0","blue1","blue2","blue3","blue4","red0","red1","red2","red3","red4"];
static CT_OPEN: AtomicBool = AtomicBool::new(false);
static CT_INJECTED: AtomicBool = AtomicBool::new(false);
static CT_LAST: Mutex<[i64; 40]> = Mutex::new([-1; 40]); // 10행 × 4슬롯
static CT_CHAMPS: Mutex<Vec<String>> = Mutex::new(Vec::new()); // 행별 마지막 관측 챔프(변경 감지)
// ★진단: 조합테스트 배선 단계별 (buy_report 출력)
static CTD_CALL: AtomicU64 = AtomicU64::new(0);    // 핸들러 호출수
static CTD_BUILDS: AtomicU64 = AtomicU64::new(0);  // #builds 노드 발견
static CTD_VIS: AtomicU64 = AtomicU64::new(0);     // 그게 visible
static CTD_ROW: AtomicU64 = AtomicU64::new(0);     // blue0 런타임 노드 발견
static CTD_CHAMP: AtomicU64 = AtomicU64::new(0);   // 챔프 이름 해석 성공
static CTD_SET: AtomicU64 = AtomicU64::new(0);     // it4_slot3 옵션주입 성공
// training 행의 챔프: 자식 id가 #champion_icon (전술화면의 #icon 과 다름) → 전용 조회.
fn ct_row_champ(row: &Node) -> Option<String> {
    let icon = find_node(row, "champion_icon")?;
    let src = unsafe { read_img_source(icon) }?;
    let a = src.find("champions/")? + "champions/".len();
    let rest = &src[a..];
    let end = rest.find('#').unwrap_or(rest.len());
    let champ = rest[..end].trim();
    if champ.is_empty() { None } else { Some(champ.to_string()) }
}
// ★★조합테스트 4칸 좌표 강제(07-21) — 템플릿 재작성 대신 런타임 조정(타 모드 노드 보존).
//   바닐라 item0/1/2 = x146/296/446 · w140 (586까지) → 4칸용으로 축소 재배치.
//   it4_slot3(우리가 append)은 조각에 이미 x482/w104로 들어있지만, 게임 리셋 대비 함께 강제.
//   ⚠MODE4(4칸)일 때만. 3칸 모드면 바닐라 좌표 그대로 둔다.
//   게임이 매 프레임 되돌릴 수 있어 force_blue_slot_spacing 과 동일하게 매 프레임 재적용.
// ★런타임 좌표강제 폐기(07-21): 조합테스트 4칸을 우리가 전부 선언하므로 네이티브를 옮길 이유가 없다.
//   구 force_comptest_slot_layout()은 네이티브 4상태 박스를 매프레임 write 했는데, 그 방식은
//   ①히트박스가 안 따라와 클릭 관통 ②게임 재계산과 충돌해 떨림 을 유발한다(comptest_unlock 실측).
//   ⟹ 좌표는 템플릿 선언값만 사용, 네이티브는 숨기기만 한다.
// 조합테스트 네이티브 item0/1/2 숨김 — 모드소유 드롭다운이 그 자리를 대신한다.
fn hide_comptest_native_dds(root: &mut Node) {
    for rid in CT_ROWS.iter() {
        if let Some(row) = find_mut(root, rid) {
            for si in 0..3u8 {
                if let Some(nat) = find_mut(row, &format!("item{}", si)) {
                    if nat.visible { nat.visible = false; }
                }
            }
        }
    }
}
// ═══════════════════════════════════════════════════════════════════════════
//  ★경기중 4번째 슬롯 아이콘 — **노드 직접 세팅** 방식 (2026-07-30, 게임 코드 무수정)
// ═══════════════════════════════════════════════════════════════════════════
//  게임 코드 수술(프레임 확장+배열 이전) = 경기 진입 프리즈로 실패 ⟹ 접근 전환.
//  ghidra-re 실측으로 밝힌 게임의 아이콘 채우기 계약(게임 함수 호출 0회로 재현 가능):
//    ① 노드 경로 `<side>.slotN.bg.icon` 하강(게임은 '.' split 후 재귀 탐색 0x19f170)
//    ② `Node.visible`(+0x260) 1/0
//    ③ ImageRunner(4상태 stride 208 = normal/hover/active/disabled)의
//       `source`(+0) = **아이템 공통 스프라이트시트 경로 고정**, `rect_tag`(+0x18) = Some(시트 내 태그)
//    ⟹ 아이템 구분은 source가 아니라 **rect_tag**로 한다(구 set_img_src의 "경로#태그" 방식이 아님).
//  아이콘 태그 규칙(번들 item_setting 전수 대조 실측): 바닐라 index 0..29 → `t{idx%5+1}_{idx/5}`.
//    모드 아이템(idx>=30)은 이 시트에 태그가 없어 **게임 자신도 렌더 못 함** → 숨김 처리.
//  ⚠1단계(현재): 표시 자체가 되는지 검증하려고 **고정 태그**를 넣는다. 실제 아이템 매핑은
//    "화면에 뜬 선수가 누구인가"(뷰모델 조회) 규명이 남아 2단계로 분리.
const SLOT3_ICON_ENABLED: bool = true;   // 문제 시 false = 이전 상태(아이콘 없음)로 즉시 복귀
// ═══════════════════════════════════════════════════════════════════════════
//  ★★3단계 = **뷰모델(GameView) 직독** (2026-07-30 풀 RE 확정) — 게임 코드 무패치
// ═══════════════════════════════════════════════════════════════════════════
//  ⛔폐기한 접근 2종과 그 실패 이유(재시도 금지):
//    ①게임 루프 상한 확장(프레임 확장+배열 이전) = 경기 진입 프리즈(84/84 사이트 적용에도 실패).
//    ②챔프 이름 캐시(buy 훅에서 champ→icon 캐시) = **오염 구조상 불가피**. 내 선수는 배경 pre-sim과
//      화면 경기에 동시 존재(athlete+0x810 조인이 양쪽 유효 = 정본)라, 배경에서 4개 완성한 값이
//      화면(3개 보유) 선수에게 샌다. 게다가 `blue_player` 노드를 1개로 착각해(실제 레인당 1개=5+5)
//      첫 레인에 남의 값을 쓰고 있었다 ⟹ 유저 관측 "엉뚱한 아이템"의 진짜 정체.
//  ✅정답 = 게임이 slot0~2를 그릴 때 읽는 **바로 그 데이터**를 모드도 읽는다:
//    GameView(=App+0x4a50, 프로세스 수명 내내 불변) → player_view HashMap(키=(team,position))
//    → PlayerViewInfo.items: Vec<u64>(item_list 인덱스) → item_list[idx] = (data,vtable) → vtable+0x60=icon()
//  ★items[3]은 이미 존재한다: 게임 슬롯 루프의 `cmp rbx,0x30`은 아이템 개수 제한이 아니라
//    **하드코딩된 노드명 3개("slotN") 배열의 바이트 크기**이고, 실제 아이템 순회는 `i < items.len()`
//    길이가드다(0xa6339f). 뷰 체인 전 구간에 take(3)/min(3) 없음(apply_frame 0x952170 = capless collect).
const GV_OFF_ITEMLIST_CAP: usize = 0xa8;  // -1이면 None
const GV_OFF_ITEMLIST_PTR: usize = 0xb0;
const GV_OFF_ITEMLIST_LEN: usize = 0xb8;
const GV_OFF_PV_CTRL: usize = 0x1d0;      // hashbrown RawTable ctrl
const GV_OFF_PV_MASK: usize = 0x1d8;
const GV_OFF_PV_ITEMS: usize = 0x1e8;     // 원소 수(0이면 경기 아님)
const PV_STRIDE: usize = 0x2c0;           // PlayerViewInfo. 0.5.5: 구 0.5.3~0.5.4 ~~0x260~~ → **0x2c0** (+0x60, 정정 2026-08-12 인게임결함).
//   ★근거(ghidra 명령 실측): 0.5.5 ingame_ui `imul r10,r10,0x2c0` @0xadbefd 등 4곳 + GV update 0x964350 디컴
//   그룹스킵 -0x2c0 + 메가함수 상수 0x2600→0x2C00 12곳. hashbrown 역방향 어드레싱(ctrl−(i+1)*stride)이라
//   stride가 어긋나면 team/pos가 쓰레기 → collect_slot3_icons 빈 맵 → icon.visible=false 능동 세팅
//   = 0.5.5 "slot3 효과는 적용되는데 아이콘 미표시" 결함의 근본원인. 엔트리 내부(+0 team/+8 pos/items
//   +0x50·0x58·0x60)와 GameView 헤더(+0x1d0/0x1d8/0x1e8)·item vtable(+0x58/+0x60)은 불변 실측.
const PV_OFF_TEAM: usize = 0x00;          // u64 태그: 0=blue(Team0) 1=red(Team1)
const PV_OFF_POS: usize = 0x08;           // u32: 0 top /1 jungle /2 mid /3 bottom /4 support
const PV_OFF_ITEMS_PTR: usize = 0x58;     // Vec<u64> = {cap@0x50, ptr@0x58, len@0x60}
const PV_OFF_ITEMS_LEN: usize = 0x60;
const LANES: [&str; 5] = ["top", "jungle", "mid", "bottom", "support"];
// ═══ slot3 툴팁 = **게임의 `#item_tooltip` 노드 재사용** (2026-07-30) ═══
//  게임 툴팁 코드는 `"<side>_player.item0/1/2"` 3경로만 하드코딩 순회해 **#slot3을 절대 방문하지 않는다**
//  (focus를 세팅해도 안 잡힘 = A안 불가, emit은 메가함수 프레임 로컬 강결합 = 외부 호출 불가 = B안 불가).
//  ⟹ 그러나 **툴팁 노드 자체는 `ingame.ui`에 이미 존재**(`#item_tooltip`, visible:false, z=위) ⟹
//     모드가 그 노드의 라벨/아이콘을 채우고 위치+visible만 세팅하면 **게임과 100% 동일한 모양**이 나온다.
//  노드 구조(bundle 실측): #item_tooltip(274x250) > #bg / #data > {#slot>#icon, #name, #tier, #price, #desc}
//  ⚠게임이 자기 툴팁을 띄우는 프레임(slot0~2 호버)에는 **건드리지 않는다** — 소유권 경합 방지.
//    게임이 안 쓰는 프레임에만 우리가 빌려 쓰고, 우리 호버가 끝나면 visible=false로 되돌린다.
// ★재활성(2026-07-30): 크래시 원인 = **vtable 슬롯 오해**(+0x50을 name으로 알고 역참조 / 실제는 bool)와
//   게임 show 함수 직접 호출(인자 11개 계약 불일치). 둘 다 폐기하고 **확정 슬롯 + 라벨 직접 채우기**로 전환.
//   문제 시 false 로 즉시 원복(아이콘은 그대로 동작).
// ★재활성(2026-07-30, 풀 RE 후): 크래시 원인 = **인자가 한 칸씩 밀림**(p1←arg4, 정답은 arg5).
//   빈 툴팁 원인 = 번들 경로 오류(`bundle_unpacked` — 게임엔 `_full`만) + 레이아웃 미갱신.
//   ⟹ 게임 show 함수 통짜 호출로 전환(내용·크기·위치 전부 게임이 처리). 문제 시 false 로 즉시 원복.
const TOOLTIP_ENABLED: bool = true;
const LABEL_TEXT_OFF: usize = 352;        // LabelRunner.text (ui_kit 정본, String 통째 대입)
const NODE_OFF_FOCUS: usize = 0x262;      // 1|2 = hover
const NODE_OFF_RECT: usize = 0x240;       // x,y,w,h (f32 ×4)
static TIP_SHOWN: AtomicU64 = AtomicU64::new(0);   // 우리가 띄운 프레임 수(진단)
static TIP_OWNED: AtomicBool = AtomicBool::new(false); // 지금 우리가 빌려 쓰는 중인가
// ★게임 툴팁 show 함수 = `game-view\src\ui\item_tooltip.rs` (RE 확정 0.5.3).
//   계약: (p1=asset/i18n 레지스트리, p2=텍스트계측 ctx, p3=그 vtable(상수), node=#item_tooltip,
//          item_data, item_vtable, x, y, pivot_x, pivot_y, clamp_rect{x,y,w,h})
//   ★item (data,vtable)은 **빌림만** 한다(내부에서 drop 안 함) ⟹ item_list 원본 그대로 넘겨도 안전.
const RVA_TIP_SHOW: usize = 0x1b1d500; // 0.5.6(구0.5.5=0x2587990). head-UNIQUE·콜러 3개(UImega 0xab5339 포함) 동수·size 10237→10383. // 0.5.5(구0.5.4=0x236dc00). head-지문 유일후보·콜러 3개(UImega=0xaedc09 포함) 동수·size 10010→10237·ninsn 1936→2018. // 0.5.4(구0.5.3=0x1ab52f0). item_tooltip.rs panic-location 23개 동일 + 콜러 3개(UI메가 포함) 동수 + 명령수 1936 동일.
const RVA_TIP_MEASURE_VT: usize = 0x3333ec8; // 0.5.6(구0.5.5=0x333b970). UImega tipshow call(0xab5339) 직전 lea r8,[rip+0x28979e8]@0xab5321 → 0x334cd10. // 0.5.5(구0.5.4=0x32602a0). UImega tipshow call(0xaedc09) 직전 lea r8,[rip+X]@0xaedbf1 = 4c 8d 05 78 dd 84 02 → 0x333b970. p3 = 텍스트 계측 ctx 의 vtable(상수)
static TIP_P1: AtomicUsize = AtomicUsize::new(0);
static TIP_P2: AtomicUsize = AtomicUsize::new(0);
static TIP_ROOT: AtomicUsize = AtomicUsize::new(0);
// 노드 필드 read/write (전부 VEH 보호 범위 검증 후)
unsafe fn node_focus(n: &Node) -> u8 {
    let p = (n as *const Node as usize) + NODE_OFF_FOCUS;
    if readable(p, 1) { *(p as *const u8) } else { 0 }
}
unsafe fn node_rect(n: &Node) -> Option<(f32, f32, f32, f32)> {
    let p = (n as *const Node as usize) + NODE_OFF_RECT;
    if !readable(p, 16) { return None; }
    Some((*(p as *const f32), *((p + 4) as *const f32),
          *((p + 8) as *const f32), *((p + 12) as *const f32)))
}
unsafe fn node_set_xy(n: &Node, x: f32, y: f32) {
    // 레이아웃 x/y = authored 위치(+0x84 계열이 아니라 rect 자체를 게임이 매 프레임 재계산하므로
    // 툴팁처럼 게임이 위치를 안 건드리는 노드는 rect 직접 세팅이 먹는다).
    let p = (n as *const Node as usize) + NODE_OFF_RECT;
    if writable(p, 8) { *(p as *mut f32) = x; *((p + 4) as *mut f32) = y; }
}
// item_list[idx] → (data, vtable)
// ── 아이템 vtable 접근 (2026-07-30 전수 RE 확정) ──────────────────────────
//  ✅+0x58 key(&String) / +0x60 icon(&String) / +0x68 price(u64 **값**) / +0x70 tier(u64 **값**, 0-base)
//  ⛔+0x50 = bool(self+0x190!=0) — **name 아님**. 이름은 vtable 슬롯이 없고 key로 i18n 키를 조립한다.
//     (이걸 String 포인터로 착각해 역참조 → 크래시. 재발 금지.)
const RVA_GAME_ALLOC: usize = 0x2ab4010;  // 0.5.6(구0.5.5=0x2a9bf30, skel/마스크 UNIQUE·BYTE=SAME). // 0.5.5(구0.5.4=0x29bb920). (rcx=무시, rdx=flags 0, r8=size) -> ptr
unsafe fn item_obj_at(gv: usize, idx: u64) -> Option<(usize, usize)> {
    if !readable(gv + GV_OFF_ITEMLIST_CAP, 24) { return None; }
    if rd_u64(gv + GV_OFF_ITEMLIST_CAP) == u64::MAX { return None; }
    let ptr = rd_u64(gv + GV_OFF_ITEMLIST_PTR) as usize;
    let len = rd_u64(gv + GV_OFF_ITEMLIST_LEN);
    if idx >= len || ptr < 0x10000 { return None; }
    let e = ptr + (idx as usize) * 0x10;
    if !readable(e, 16) { return None; }
    let (d, v) = (rd_u64(e) as usize, rd_u64(e + 8) as usize);
    if d < 0x10000 || v < 0x10000 { None } else { Some((d, v)) }
}
// GameView 포인터(읽기전용 캡처). game.rs update(0x960df0)의 rcx = GameView. 값 불변이라 1회만 잡으면 된다.
static GAME_VIEW: AtomicUsize = AtomicUsize::new(0);
const RVA_GV_UPDATE: usize = 0x90a090; // 0.5.6(구0.5.5=0x964350, skel UNIQUE·BYTE=SAME·size 4575·프롤로그 동일). // 0.5.5(구0.5.4=0xaa06c0, exe2exe 스켈레톤 확정). // 0.5.4(구0.5.3=0x960df0). 본문 니모닉 100% 동일(오프셋 시퀀스까지)·콜러 22개 동수.
const GV_UPDATE_PROLOGUE: [u8; 12] = [0x55, 0x41,0x57, 0x41,0x56, 0x41,0x55, 0x41,0x54, 0x56, 0x57, 0x53];
static GV_HOOK_INSTALLED: AtomicU64 = AtomicU64::new(0);
static GV_HITS: AtomicU64 = AtomicU64::new(0);
// ⚠최소 detour: 매 프레임 발화하는 UI 경로라 원자 store만(락·할당·파일IO 금지).
unsafe extern "C" fn cap_game_view(saved: *mut u64, _e: usize) -> u64 {
    if saved.is_null() { return 0; }
    let gv = *saved as usize;              // rcx = &mut GameView
    if gv >= 0x10000 && gv < 0x0000_8000_0000_0000 {
        GAME_VIEW.store(gv, Ordering::Relaxed);
        GV_HITS.fetch_add(1, Ordering::Relaxed);
    }
    // ★게임 툴팁 show 함수 인자 캡처 (2026-07-30 풀 RE 확정 — ⛔이전 인덱스는 **한 칸씩 밀려** 크래시했다)
    //   콜체인: 0x960df0(game.rs update) → 0xa5c1e0(ingame_ui) → 0x1ab52f0(tooltip show)
    //   메가함수는 자기 arg1/arg2 를 툴팁 p1/p2 로 넘기고, arg4 를 노드 탐색 루트로 쓴다.
    //   그리고 0x960df0 콜사이트가 넘기는 값 = rcx←[rbp+0x140]=진입rsp+0x28=**arg5**, rdx←[rbp+0x148]=**arg6**, r9←**arg4**.
    //   ⟹ p1 = arg5 / p2 = arg6 / root = arg4(r9).
    //   ⛔구 구현: p1←r9(arg4), p2←arg5, root←arg7 ⟹ **UI 루트 노드를 레지스트리로 넘겨** 해시조회에서 즉사.
    //   스텁 레이아웃: push r12,rsi,rdi,rbx,r11,r10,r9,r8,rdx,rcx → r9=saved+3, 진입rsp=saved+10.
    let root = *saved.add(3) as usize;                             // arg4 (r9)
    let sp = saved.add(10) as usize;                               // 진입 rsp
    let p1 = safe_read_u64(sp + 0x28).unwrap_or(0) as usize;       // arg5 = 애셋/설정 레지스트리
    let p2 = safe_read_u64(sp + 0x30).unwrap_or(0) as usize;       // arg6 = 텍스트 계측 ctx
    if p1 >= 0x10000 { TIP_P1.store(p1, Ordering::Relaxed); }
    if p2 >= 0x10000 { TIP_P2.store(p2, Ordering::Relaxed); }
    if root >= 0x10000 { TIP_ROOT.store(root, Ordering::Relaxed); }
    0
}
fn install_game_view_hook() {
    if GV_HOOK_INSTALLED.load(Ordering::Relaxed) == 1 { return; }
    let r = unsafe { install_detour_generic(RVA_GV_UPDATE, 12, cap_game_view as usize, &GV_UPDATE_PROLOGUE) };
    GV_HOOK_INSTALLED.store(if r.is_ok() { 1 } else { 2 }, Ordering::Relaxed);
}
// item_list[idx] 의 아이콘 문자열(vtable +0x60 = icon()). 게임의 set_item_icon(0x97b540)과 동일 경로.
//   ⚠shadow-call이라 code_ptr_ok 가드 + 반환 String 범위검증 필수.
unsafe fn item_icon_by_index(gv: usize, idx: u64) -> Option<String> {
    if !readable(gv + GV_OFF_ITEMLIST_CAP, 24) { return None; }
    if rd_u64(gv + GV_OFF_ITEMLIST_CAP) == u64::MAX { return None; } // None sentinel
    let ptr = rd_u64(gv + GV_OFF_ITEMLIST_PTR) as usize;
    let len = rd_u64(gv + GV_OFF_ITEMLIST_LEN);
    if idx >= len || ptr < 0x10000 { return None; }
    let e = ptr + (idx as usize) * 0x10;
    if !readable(e, 16) { return None; }
    let data = rd_u64(e) as usize;
    let vt = rd_u64(e + 8) as usize;
    if data < 0x10000 || vt < 0x10000 || !readable(vt + 0x60, 8) { return None; }
    let f = rd_u64(vt + 0x60) as usize;
    if !code_ptr_ok(f) { return None; }
    let g: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(f);
    let s = g(data);
    if s < 0x10000 || !readable(s, 0x18) { return None; }
    let sp = rd_u64(s + 8) as usize;   // String = {cap@0, ptr@8, len@0x10}
    let sl = rd_u64(s + 0x10) as usize;
    if sp < 0x10000 || sl == 0 || sl > 64 || !readable(sp, sl) { return None; }
    Some(String::from_utf8_lossy(std::slice::from_raw_parts(sp as *const u8, sl)).into_owned())
}
// player_view 해시맵 전수 순회 → (team, position) → items[3] 아이콘. 해시 계산 불요(버킷 선형 스캔).
//   hashbrown: ctrl 바이트 최상위비트 0 = FULL, 엔트리는 ctrl 기준 **역방향** (ctrl - (i+1)*stride).
unsafe fn collect_slot3_icons(gv: usize) -> HashMap<(u64, u32), String> {
    let mut out = HashMap::new();
    if !readable(gv + GV_OFF_PV_CTRL, 32) { return out; }
    let ctrl = rd_u64(gv + GV_OFF_PV_CTRL) as usize;
    let mask = rd_u64(gv + GV_OFF_PV_MASK) as usize;
    let nitems = rd_u64(gv + GV_OFF_PV_ITEMS);
    if ctrl < 0x10000 || nitems == 0 || nitems > 64 || mask > 0x1000 { return out; }
    for i in 0..=mask {
        if !readable(ctrl + i, 1) { break; }
        if *((ctrl + i) as *const u8) & 0x80 != 0 { continue; } // FULL 아님
        let e = ctrl.wrapping_sub((i + 1) * PV_STRIDE);
        if e < 0x10000 || !readable(e, PV_STRIDE) { continue; }
        let team = rd_u64(e + PV_OFF_TEAM);
        let pos = (rd_u64(e + PV_OFF_POS) & 0xffff_ffff) as u32;
        if team > 1 || pos > 4 { continue; }
        let it_ptr = rd_u64(e + PV_OFF_ITEMS_PTR) as usize;
        let it_len = rd_u64(e + PV_OFF_ITEMS_LEN);
        if it_len < 4 || it_ptr < 0x10000 || !readable(it_ptr + 3 * 8, 8) { continue; } // 4번째 미보유
        let idx = rd_u64(it_ptr + 3 * 8);
        if let Some(tag) = item_icon_by_index(gv, idx) { out.insert((team, pos), tag); }
    }
    out
}
const ICON_SHEET: &str = "asset/base/aseprite_resources/ingame/item_icons_18x18";
const IMG_STATE_OFF: [usize; 4] = [0, 208, 416, 624]; // normal/hover/active/disabled
const IMG_OFF_SOURCE: usize = 0;
const IMG_OFF_RECT_TAG: usize = 24;
const NODE_OFF_VISIBLE: usize = 0x260;
static SLOT3_ICON_N: AtomicU64 = AtomicU64::new(0);   // 세팅 성공 노드 수(누계)
static SLOT3_ICON_MISS: AtomicU64 = AtomicU64::new(0); // 노드/러너 불일치로 스킵
// ImageRunner 데이터부에 시트+태그를 4상태 전부 기록. String 통째 대입(부분 필드 write 금지 —
//   구 set_img_src 가 {len@0,ptr@8}로 잘못 써서 cap 오염 → teardown 시 static ptr을 HeapFree 하는
//   잠복 버그였다. 실제 배치는 {cap@0, ptr@8, len@0x10}). 게임·모드 모두 프로세스 힙(GetProcessHeap)이라
//   모드가 만든 String을 게임이 drop 해도 안전.
unsafe fn set_icon_rect_tag(n: &Node, tag: &str) -> bool {
    if !n.runner.type_name().contains("ImageRunner") { return false; }
    let base = runner_base(n);
    if base < 0x10000 || !readable(base, 848) { return false; }
    for st in IMG_STATE_OFF {
        let sp = base + st + IMG_OFF_SOURCE;
        let tp = base + st + IMG_OFF_RECT_TAG;
        if !writable(sp, 24) || !writable(tp, 24) { return false; }
        *(sp as *mut String) = ICON_SHEET.to_string();
        *(tp as *mut Option<String>) = Some(tag.to_string());
    }
    true
}
unsafe fn node_set_visible(n: &Node, v: bool) {
    let p = (n as *const Node as usize) + NODE_OFF_VISIBLE;
    if writable(p, 1) { *(p as *mut u8) = if v { 1 } else { 0 }; }
}
// 현재 rect_tag 를 읽어 같은 값이면 재기록 생략(프레임당 String alloc 회피).
unsafe fn icon_tag_is(n: &Node, tag: &str) -> bool {
    let base = runner_base(n);
    if base < 0x10000 || !readable(base + IMG_OFF_RECT_TAG, 24) { return false; }
    // Option<String> niche 최적화: ptr==0 이면 None
    let ptr = rd_u64(base + IMG_OFF_RECT_TAG + 8) as usize;
    let len = rd_u64(base + IMG_OFF_RECT_TAG + 0x10) as usize;
    if ptr < 0x10000 || len != tag.len() || !readable(ptr, len) { return false; }
    std::slice::from_raw_parts(ptr as *const u8, len) == tag.as_bytes()
}
// ★2단계 준비 진단: 경기중 player_info 서브트리에서 "선수 식별 단서 + slot0~2 실제 태그"를 1회 덤프.
//   목적 = 4번째 아이템을 어떤 경로로 알아낼지 확정(노드의 이름/챔프 라벨 ↔ 모드가 아는 athlete 매칭).
//   slot0~2 태그를 역산(t{a}_{b} → idx = b*5 + (a-1))하면 그 선수의 items[0..2]를 알 수 있다.
static SLOT3_DUMPED: AtomicBool = AtomicBool::new(false);
unsafe fn read_icon_tag(n: &Node) -> Option<String> {
    if !n.runner.type_name().contains("ImageRunner") { return None; }
    let base = runner_base(n);
    if base < 0x10000 || !readable(base + IMG_OFF_RECT_TAG, 24) { return None; }
    let ptr = rd_u64(base + IMG_OFF_RECT_TAG + 8) as usize;
    let len = rd_u64(base + IMG_OFF_RECT_TAG + 0x10) as usize;
    if ptr < 0x10000 || len == 0 || len > 64 || !readable(ptr, len) { return None; }
    Some(String::from_utf8_lossy(std::slice::from_raw_parts(ptr as *const u8, len)).into_owned())
}
fn dump_slot3_context(ui: &GameUI) {
    if SLOT3_DUMPED.load(Ordering::Relaxed) { return; }
    // ⚠타이밍: post_update는 경기 화면 진입 **전**에도 돌기 때문에, 1회성 플래그를 무조건 소진하면
    //   빈 트리를 덤프하고 끝난다(첫 시도가 정확히 그렇게 실패했다). 대상 노드가 실제로 나타난
    //   프레임에만 덤프한다.
    if find_node(&ui.root, "blue_player").is_none() && find_node(&ui.root, "red_player").is_none() { return; }
    // ⚠2차 타이밍 함정: 경기 **시작 직후**엔 아무도 아이템이 없어서 slot0~2가 전부 비어 있다
    //   (첫 시도가 kda=0/0/0 시점에 찍혀 전부 tag=None 이었다). 게임이 slot0~2 중 하나라도
    //   실제로 아이콘을 채운 뒤에 덤프해야 "게임이 모드 아이템을 어떻게 그리는가"를 볼 수 있다.
    let mut armed = false;
    for side in ["blue_player", "red_player"].iter() {
        let Some(sp) = find_node(&ui.root, side) else { continue };
        for k in 0..3 {
            let Some(sl) = find_node(sp, &format!("slot{}", k)) else { continue };
            let Some(bg) = find_node(sl, "bg") else { continue };
            let Some(ic) = find_node(bg, "icon") else { continue };
            if unsafe { read_icon_tag(ic) }.is_some() || unsafe { read_img_source(ic) }.is_some() { armed = true; }
        }
    }
    if !armed { return; }
    SLOT3_DUMPED.store(true, Ordering::Relaxed);
    let mut s = String::from("[slot3 2단계 조사] 경기중 player_info 서브트리\n");
    for side in ["blue_player", "red_player"].iter() {
        s.push_str(&format!("\n=== {} ===\n", side));
        let Some(sp) = find_node(&ui.root, side) else { s.push_str("  (노드 없음)\n"); continue };
        // 선수 식별 단서 후보: 라벨류 전부
        for cid in ["name", "champion_icon", "champion", "level", "kda", "gold", "player_name"].iter() {
            if let Some(n) = find_node(sp, cid) {
                let lbl = unsafe { read_label(n) };
                let src = unsafe { read_img_source(n) };
                let tag = unsafe { read_icon_tag(n) };
                s.push_str(&format!("  #{:<14} runner={:<22} label={:?} src={:?} tag={:?}\n",
                    cid, n.runner.type_name(), lbl, src, tag));
            }
        }
        // slot0~3의 icon 태그(= 현재 표시중인 아이템) — 역산해 items[0..2] 파악
        for k in 0..4 {
            let sid = format!("slot{}", k);
            let Some(sl) = find_node(sp, &sid) else { s.push_str(&format!("  {} : 없음\n", sid)); continue };
            let icon = find_node(sl, "bg").and_then(|bg| find_node(bg, "icon"));
            match icon {
                None => s.push_str(&format!("  {} : bg.icon 없음 (visible={})\n", sid, sl.visible)),
                Some(ic) => {
                    let tag = unsafe { read_icon_tag(ic) };
                    let idx = tag.as_deref().and_then(tag_to_idx);
                    // ★모드 아이템 케이스 규명용: source(시트 경로)도 같이 본다.
                    //   게임 기본 시트에는 모드 아이템 태그가 없으므로, 모드템이 걸린 슬롯의 source가
                    //   다른 경로(모드 제공 에셋)로 바뀌는지 / 아니면 태그만 다른지가 여기서 갈린다.
                    let src = unsafe { read_img_source(ic) };
                    s.push_str(&format!("  {} : visible={} icon.visible={} tag={:?} → idx={:?}\n      src={:?}\n",
                        sid, sl.visible, ic.visible, tag, idx, src));
                }
            }
        }
        // 서브트리 얕은 덤프(자식 id만) — 못 찾은 식별 노드가 어디 있는지 파악용
        s.push_str("  [직계 자식] ");
        for c in sp.child.iter() { s.push_str(&format!("{} ", c.id.as_str())); }
        s.push('\n');
        // ★선수 식별 경로 탐색: champion 서브트리(챔프 아이콘 source에 챔프명이 들어있을 가능성)
        //   1차 덤프에서 #name 류 라벨이 없고 champion=ColorRunner 로 나왔다 ⟹ 한 단계 더 내려가야 한다.
        if let Some(ch) = find_node(sp, "champion") {
            s.push_str("  [champion 서브트리]\n");
            unsafe { dump_subtree(ch, 4, &mut s); }
        }
    }
    if let Some(d) = mod_dir() { let _ = fs::write(d.join("slot3_probe.txt"), s); }
}
// 태그 → 카탈로그 인덱스 역산 (t{a}_{b} → b*5 + (a-1))
fn tag_to_idx(t: &str) -> Option<usize> {
    let rest = t.strip_prefix('t')?;
    let (a, b) = rest.split_once('_')?;
    let a: usize = a.parse().ok()?;
    let b: usize = b.parse().ok()?;
    if a == 0 || a > 5 { return None; }
    Some(b * 5 + (a - 1))
}
static SLOT3_PV_N: AtomicU64 = AtomicU64::new(0); // 뷰모델에서 4번째 보유로 잡힌 선수 수
fn handle_ingame_slot3(ui: &GameUI) {
    if !SLOT3_ICON_ENABLED || slot_count() != 4 { return; }
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| dump_slot3_context(ui)));
    let gv = GAME_VIEW.load(Ordering::Relaxed);
    if gv < 0x10000 { return; }                    // 아직 캡처 전(경기 화면 진입 전)
    // ★게임이 slot0~2를 그릴 때 읽는 그 데이터를 그대로 읽는다 = 캐시·챔프매칭·is_live 전부 불요.
    let icons = unsafe { collect_slot3_icons(gv) };
    SLOT3_PV_N.store(icons.len() as u64, Ordering::Relaxed);
    // ★노드 경로 = player_info.<lane>.{blue_player|red_player}.slot3.bg.icon (일반)
    //             + wide_data.player_info.<lane>....                        (와이드)
    //   ⚠blue_player/red_player 는 **레인당 1개씩(5+5)**, 레이아웃까지 치면 최대 20개다.
    //     구 코드가 find_node(root,"blue_player") 로 **첫 매치 1개만** 처리한 것이 오표시의 진짜 원인.
    let roots: [Option<&Node>; 2] = [
        find_node(&ui.root, "player_info"),
        find_node(&ui.root, "wide_data").and_then(|w| find_node(w, "player_info")),
    ];
    let mut hover: Option<(u64, u32, f32, f32, f32, f32)> = None; // (team,pos,x,y,w,h)
    for root in roots.iter().flatten() {
        for (pos, lane) in LANES.iter().enumerate() {
            let Some(ln) = find_node(root, lane) else { continue };
            for (team, side) in ["blue_player", "red_player"].iter().enumerate() {
                let Some(sp) = find_node(ln, side) else { continue };
                let Some(slot3) = find_node(sp, "slot3") else { continue };
                let Some(bg) = find_node(slot3, "bg") else { continue };
                let Some(icon) = find_node(bg, "icon") else {
                    SLOT3_ICON_MISS.fetch_add(1, Ordering::Relaxed); continue };
                let tag = icons.get(&(team as u64, pos as u32));
                unsafe {
                    match tag {
                        // 4번째 미보유 = 게임의 빈 슬롯 처리와 동일(visible=false만, 이미지 필드는 안 건드림)
                        None => node_set_visible(icon, false),
                        Some(t) => {
                            // ★호버 감지: 슬롯 노드(또는 bg)의 focus ∈ {1,2}. 게임 hit-test가 세팅한다.
                            if TOOLTIP_ENABLED && hover.is_none()
                                && (node_focus(slot3) == 1 || node_focus(slot3) == 2
                                    || node_focus(bg) == 1 || node_focus(bg) == 2) {
                                if let Some((x, y, w, h)) = node_rect(slot3) {
                                    hover = Some((team as u64, pos as u32, x, y, w, h));
                                }
                            }
                            if icon_tag_is(icon, t) { node_set_visible(icon, true); continue; } // 동일값 = 재기록 생략
                            if set_icon_rect_tag(icon, t) {
                                node_set_visible(icon, true);
                                node_set_visible(slot3, true);
                                SLOT3_ICON_N.fetch_add(1, Ordering::Relaxed);
                            } else {
                                SLOT3_ICON_MISS.fetch_add(1, Ordering::Relaxed);
                            }
                        }
                    }
                }
            }
        }
    }
    if TOOLTIP_ENABLED {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            unsafe { drive_slot3_tooltip(ui, gv, hover); }
        }));
    }
}

// ★게임의 `#item_tooltip` 노드를 빌려 slot3 툴팁을 띄운다(모드가 새로 그리지 않음 = 모양 100% 동일).
//   ⚠소유권 규칙: 게임이 자기 툴팁을 쓰는 프레임(slot0~2 호버 → 게임이 visible=true로 세팅)에는
//     절대 손대지 않는다. 우리는 **게임이 안 쓰는 프레임에만** 빌려 쓰고, 우리 호버가 끝나면 되돌린다.
unsafe fn drive_slot3_tooltip(ui: &GameUI, gv: usize, hover: Option<(u64, u32, f32, f32, f32, f32)>) {
    let Some(tip) = find_node(&ui.root, "item_tooltip") else { return };
    let Some((team, pos, sx, sy, sw, sh)) = hover else {
        // 우리 호버 종료 → 우리가 띄운 것만 내린다(게임이 띄운 건 건드리지 않음)
        if TIP_OWNED.swap(false, Ordering::Relaxed) { node_set_visible(tip, false); }
        return;
    };
    // ⛔구 버그(2026-07-30 유저 제보 "마지막으로 호버했던 아이템 툴팁이 그대로 뜬다"):
    //   `if !TIP_OWNED && tip.visible { return; }` 로 양보했더니, 게임이 slot0~2 호버 종료 후
    //   **툴팁을 곧바로 내리지 않는 프레임**(다음 프레임에 처리)에 slot3으로 마우스를 옮기면
    //   visible=true 인 남은 툴팁을 보고 return → **내용을 안 채운 채 이전 아이템 툴팁이 그대로 보였다**.
    //   ⟹ 양보 판단을 "내용을 채우기 전"이 아니라 **"게임이 이번 프레임에 실제로 자기 슬롯을 호버 중인가"**로 바꾼다.
    //   slot0~2 중 하나라도 호버 중이면 그건 게임 차례이므로 우리는 손대지 않는다.
    if game_hovering_own_slot(ui) {
        if TIP_OWNED.swap(false, Ordering::Relaxed) { /* 우리 소유였으면 놓아준다(게임이 덮어씀) */ }
        return;
    }
    // 해당 선수의 items[3] → 아이템 객체
    let Some(pv) = find_player_view(gv, team, pos) else { return };
    let it_ptr = rd_u64(pv + PV_OFF_ITEMS_PTR) as usize;
    let it_len = rd_u64(pv + PV_OFF_ITEMS_LEN);
    if it_len < 4 || it_ptr < 0x10000 || !readable(it_ptr + 24, 8) { return; }
    let Some((data, vt)) = item_obj_at(gv, rd_u64(it_ptr + 3 * 8)) else { return };
    // ★★게임의 툴팁 show 함수를 그대로 호출한다 (2026-07-30 풀 RE 확정 계약).
    //   이름·티어·가격·스탯·효과설명·i18n·크기·위치·클램프를 **전부 게임이 처리** ⟹ 모드 아이템도 자동 정확.
    //   ⛔폐기한 시도 3종(재시도 금지):
    //     ①vtable +0x50 을 name(&String)으로 착각해 역참조 → 크래시(실제는 bool).
    //     ②같은 함수를 부르되 **인자가 한 칸씩 밀림**(p1←arg4) → 즉사. 정답은 p1=arg5/p2=arg6/root=arg4.
    //     ③라벨 직접 write + 번들 파일 파싱 → 텍스트 공백(경로가 `bundle_unpacked`인데 게임엔 `_full`만 존재)
    //       + 크기·위치 미갱신(게임은 authored 4블록 + rect 를 함께 쓴다).
    let (p1, p2) = (TIP_P1.load(Ordering::Relaxed), TIP_P2.load(Ordering::Relaxed));
    if p1 < 0x10000 || p2 < 0x10000 { return; }
    // ⚠전제: 자식 8개가 전부 있어야 한다(하나라도 없으면 게임이 unwrap 패닉 → abort).
    let Some(d) = find_node(tip, "data") else { return };
    let ok = find_node(tip, "bg").is_some()
        && find_node(d, "name").is_some() && find_node(d, "tier").is_some()
        && find_node(d, "price").is_some() && find_node(d, "desc").is_some()
        && find_node(d, "bar").is_some()
        && find_node(d, "slot").and_then(|s| find_node(s, "icon")).is_some();
    if !ok { return; }
    let base = exe_base_addr();
    if base == 0 { return; }
    let f = base + RVA_TIP_SHOW;
    if !code_ptr_ok(f) { return; }
    // 앵커(게임 규칙): 블루 = 슬롯 우측 정렬·아래 12px / 레드 = 슬롯 좌측·위 12px.
    //   authored w/h = tip+0x74 / tip+0x7c.
    let tn = tip as *const Node as usize;
    let aw = if readable(tn + 0x74, 4) { *((tn + 0x74) as *const f32) } else { 274.0 };
    let ah = if readable(tn + 0x7c, 4) { *((tn + 0x7c) as *const f32) } else { 250.0 };
    let (ax, ay) = if team == 0 { (sx + sw - aw, sy + sh + 12.0) } else { (sx, sy - ah - 12.0) };
    let clamp: [f32; 4] = [0.0, 0.0, 1920.0, 1080.0];
    type TipShow = unsafe extern "win64" fn(
        usize, usize, usize, usize,   // p1, p2, p3(계측 vtable 상수), node
        usize, usize,                 // item_data, item_vtable  (빌림만 — drop 안 함)
        f32, f32, f32, f32,           // x, y, pivot_x, pivot_y
        *const [f32; 4]);             // clamp rect
    let g: TipShow = core::mem::transmute(f);
    g(p1, p2, base + RVA_TIP_MEASURE_VT, tn, data, vt, ax, ay, 0.0, 0.0, &clamp);
    // visible=1 은 함수가 세팅한다.
    TIP_OWNED.store(true, Ordering::Relaxed);
    TIP_SHOWN.fetch_add(1, Ordering::Relaxed);
}
// 게임이 이번 프레임에 자기 슬롯(slot0~2)을 호버 중인가. 참이면 툴팁은 게임 차례다.
//   ★툴팁 노드의 visible 로 판단하면 안 된다 — 게임은 호버가 끝나도 그 프레임에 바로 내리지 않아서
//     "이전 아이템 툴팁이 그대로 남아 보이는" 버그가 났다(위 drive_slot3_tooltip 주석 참조).
unsafe fn game_hovering_own_slot(ui: &GameUI) -> bool {
    let roots: [Option<&Node>; 2] = [
        find_node(&ui.root, "player_info"),
        find_node(&ui.root, "wide_data").and_then(|w| find_node(w, "player_info")),
    ];
    for root in roots.iter().flatten() {
        for lane in LANES.iter() {
            let Some(ln) = find_node(root, lane) else { continue };
            for side in ["blue_player", "red_player"].iter() {
                let Some(sp) = find_node(ln, side) else { continue };
                for k in 0..3 {
                    let Some(sl) = find_node(sp, &format!("slot{}", k)) else { continue };
                    if node_focus(sl) == 1 || node_focus(sl) == 2 { return true; }
                    if let Some(bg) = find_node(sl, "bg") {
                        if node_focus(bg) == 1 || node_focus(bg) == 2 { return true; }
                    }
                }
            }
        }
    }
    false
}
// player_view 해시맵에서 (team,pos) 엔트리 주소를 찾는다(버킷 선형 스캔).
unsafe fn find_player_view(gv: usize, team: u64, pos: u32) -> Option<usize> {
    if !readable(gv + GV_OFF_PV_CTRL, 32) { return None; }
    let ctrl = rd_u64(gv + GV_OFF_PV_CTRL) as usize;
    let mask = rd_u64(gv + GV_OFF_PV_MASK) as usize;
    if ctrl < 0x10000 || mask > 0x1000 { return None; }
    for i in 0..=mask {
        if !readable(ctrl + i, 1) { break; }
        if *((ctrl + i) as *const u8) & 0x80 != 0 { continue; }
        let e = ctrl.wrapping_sub((i + 1) * PV_STRIDE);
        if e < 0x10000 || !readable(e, PV_STRIDE) { continue; }
        if rd_u64(e + PV_OFF_TEAM) == team && (rd_u64(e + PV_OFF_POS) & 0xffff_ffff) as u32 == pos {
            return Some(e);
        }
    }
    None
}

fn handle_comptest_screen(ui: &GameUI) {
    // ★07-21 전환: 조합테스트 개인전술을 3칸/4칸 모두 우리가 전면 관리(유저 확정).
    //   3칸이면 바닐라 좌표(146/296/446 w140)에 3개, 4칸이면 압축(146/258/370/482 w104)에 4개를
    //   ui_inject 가 선언하고, 여기서 옵션·선택을 배선한다. 네이티브는 숨긴다.
    CTD_CALL.fetch_add(1, Ordering::Relaxed);
    let bnode = find_node(&ui.root, "builds");
    if bnode.is_some() { CTD_BUILDS.fetch_add(1, Ordering::Relaxed); }
    if find_node(&ui.root, "blue0").is_some() { CTD_ROW.fetch_add(1, Ordering::Relaxed); }
    // 개인전술 탭 활성 판정: 행 컨테이너(#builds) 가시성.
    let active = bnode.map(|n| n.visible).unwrap_or(false);
    if active { CTD_VIS.fetch_add(1, Ordering::Relaxed); }
    if !active {
        if CT_OPEN.swap(false, Ordering::Relaxed) { CT_INJECTED.store(false, Ordering::Relaxed); }
        return;
    }
    CT_OPEN.store(true, Ordering::Relaxed);
    // ★옵션 라벨은 화면당 1회 계산(파일 I/O 캐시).
    if !CT_INJECTED.swap(true, Ordering::Relaxed) {
        let o = compute_options();
        *OPTS_CACHE.lock().unwrap_or_else(|e| e.into_inner()) = o;
    }
    let opts = OPTS_CACHE.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if opts.is_empty() { return; }
    let refs: Vec<&str> = opts.iter().map(|s| s.as_str()).collect();
    // ★★타이밍(07-21 실측): 화면 진입 첫 프레임엔 게임이 아직 챔프를 안 채운다(champion_icon=None).
    //   진입 1회 주입 방식이면 전부 Auto로 깔리고 챔프도 못 읽어 저장이 안 됨.
    //   → 행별로 "관측된 챔프가 바뀌면 그 행만 재시드" 하는 방식으로 전환(최초 등장·교체 모두 커버).
    let mut champs = CT_CHAMPS.lock().unwrap_or_else(|e| e.into_inner());
    if champs.len() < 10 { champs.resize(10, String::new()); }
    let mut last = CT_LAST.lock().unwrap_or_else(|e| e.into_inner());
    let mut changed = false;
    // ★진영별 챔프 구성을 모아 buy detour 로 게시(ct_scope_for 가 이걸로 진영을 판정).
    let mut ct_blue: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut ct_red: std::collections::HashSet<String> = std::collections::HashSet::new();
    for (ri, rid) in CT_ROWS.iter().enumerate() {
        let Some(row) = find_node(&ui.root, rid) else { continue; };
        let Some(c) = ct_row_champ(row) else { champs[ri].clear(); continue; }; // 챔프 미배치 = 관여 안 함
        CTD_CHAMP.fetch_add(1, Ordering::Relaxed);
        // ★행 0~4 = blue / 5~9 = red (CT_ROWS 정의 순서). 이 스코프로 지정을 읽고 쓴다 ⟹
        //   양 진영에 같은 챔프를 놓아도 각각 독립으로 지정된다(구: 하나로 합쳐졌음).
        let scope = if ri < 5 { Scope::CtBlue } else { Scope::CtRed };
        if ri < 5 { ct_blue.insert(c.clone()); } else { ct_red.insert(c.clone()); }
        let ns = slot_count();
        if champs[ri] != c {
            // 챔프 최초 등장 또는 교체 → 그 챔프의 저장값으로 전 슬롯 재시드. SEL=0(맡김)은 Auto 표시.
            //   조회는 스코프 우선 → 없으면 일반(개인전술) 지정 폴백 = 기존 지정을 그대로 물려받는다.
            let mut ok = 0;
            for si in 0..ns {
                let sel = sel_get(scope, &c, si as u8);
                let sel = (sel as usize).min(opts.len().saturating_sub(1)) as u64;
                if unsafe { nat_dd_set_options(row, uinj::CT_DD_IDS[si], &refs, sel) } {
                    unsafe { set_dd_max_height(row, uinj::CT_DD_IDS[si], MAX_ITEMS_HEIGHT); }
                    last[ri * ITEM_SLOTS + si] = sel as i64;
                    ok += 1;
                }
            }
            if ok > 0 { champs[ri] = c; CTD_SET.fetch_add(ok, Ordering::Relaxed); }
            continue; // 재시드한 프레임은 폴링 skip(자기 write를 유저선택으로 오인 방지)
        }
        // 선택 폴링 → SEL_BY_CHAMP 갱신 (전 슬롯)
        for si in 0..ns {
            if let Some(cur) = unsafe { nat_dd_selected(row, uinj::CT_DD_IDS[si]) } {
                let k = ri * ITEM_SLOTS + si;
                if cur as i64 != last[k] {
                    last[k] = cur as i64;
                    // ★스코프 키에 저장 ⟹ 이 진영에만 적용되고 일반 경기로 새 나가지 않는다.
                    //   Auto(0) 로 되돌린 경우: 일반 키에 값이 남아 있으면 폴백으로 되살아나므로
                    //   **명시 Auto(SEL_AUTO)** 를 기록해 폴백을 끊는다(일반 키는 건드리지 않음).
                    with_sel(|m| {
                        let k = (scoped_key(scope, &c), si as u8);
                        if cur == 0 {
                            if m.contains_key(&(c.clone(), si as u8)) { m.insert(k, SEL_AUTO); } else { m.remove(&k); }
                        } else { m.insert(k, cur as u8); }
                    });
                    SEL_DIRTY.store(true, Ordering::Relaxed);
                    changed = true;
                    let label = opts.get(cur).cloned().unwrap_or_else(|| format!("idx{}", cur));
                    let sidetag = if scope == Scope::CtBlue { "블루" } else { "레드" };
                    append_log("item_tactics.txt", &format!("[{}ms] (조합테스트/{}) {} slot{} → [{}] {}", now_ms(), sidetag, c, si, cur, label));
                }
            }
        }
    }
    publish_ct_roster(ct_blue, ct_red);
    if changed { drop(last); drop(champs); with_sel(|m| save_sel(m)); update_override_snapshot(); }
}
fn handle_tactics_screen(ui: &GameUI) {
    let personal = find_node(&ui.root, "personal");
    let active = personal.map(|n| n.visible).unwrap_or(false);

    if !active {
        if SCREEN_OPEN.swap(false, Ordering::Relaxed) {
            OPTS_INJECTED.store(false, Ordering::Relaxed);
        }
        return;
    }
    SCREEN_OPEN.store(true, Ordering::Relaxed);

    // ★ VEH 등록(safe_read/safe_write 공용) — 1회 멱등.
    seh_install();

    // ★ max_items_height 매 프레임 재적용(타이밍 무관 보장) + 1회 진단(쓰기 적용 확인 + 주변 덤프).
    for ri in 0..MAX_ROWS {
        let Some(row) = find_node(&ui.root, &format!("row{}", ri)) else { continue; };
        for si in 0..slot_count() {
            unsafe { set_dd_max_height(row, &slot_dd_id(si), MAX_ITEMS_HEIGHT); }
        }
    }
    if !MH_DIAG_DONE.swap(true, Ordering::Relaxed) {
        let mut s = format!("[{}ms] max_height 진단 (row0/item0)\n", now_ms());
        if let Some(row) = find_node(&ui.root, "row0") {
            if let Some(rb) = find_rb(row, "item0") {
                unsafe {
                    s.push_str(&format!("  runner_base = {:#x}\n", rb));
                    s.push_str(&format!("  writable(rb+0x1150,8) = {}\n", writable(rb + 0x1150, 8)));
                    s.push_str(&format!("  [+0x1150](present u32) = {}\n", *((rb + 0x1150) as *const u32)));
                    s.push_str(&format!("  [+0x1154](height f32) = {}\n", *((rb + 0x1154) as *const f32)));
                    s.push_str("  region +0x1130..+0x1180 (u32 hex / f32):\n");
                    let mut off = 0x1130usize;
                    while off < 0x1180 {
                        let u = *((rb + off) as *const u32);
                        let f = *((rb + off) as *const f32);
                        s.push_str(&format!("    +{:#06x}: {:#010x}  ({})\n", off, u, f));
                        off += 4;
                    }
                }
            } else { s.push_str("  item0 runner_base 못찾음\n"); }
        }
        write_log("item_tactics_maxh.txt", &s);
    }

    if !DIAG_DONE.swap(true, Ordering::Relaxed) {
        let mut s = format!("[{}ms] 개인전술 화면 감지\n", now_ms());
        s.push_str(&format!("  dd_addr_valid = {}\n", unsafe { dd_addr_valid() }));
        // ★ 비활성 모드 필터 진단: enabled_mods(mods.json) × active_keys 로 필터 판정 확인.
        let em = enabled_mods();
        let ak = active_item_keys();
        let all = mod_final_opts_all();
        let act = mod_final_opts();
        s.push_str(&format!("  enabled_mods(mods.json) = {:?}\n", em));
        s.push_str(&format!("  active_item_keys = {}개\n", ak.len()));
        s.push_str(&format!("  mod_final_opts_all(전체) = {}개: {:?}\n", all.len(), all.iter().map(|(_,k)| k.as_str()).collect::<Vec<_>>()));
        s.push_str(&format!("  mod_final_opts(활성필터후) = {}개: {:?}\n", act.len(), act.iter().map(|(_,k)| k.as_str()).collect::<Vec<_>>()));
        let filtered_out: Vec<&str> = all.iter().filter(|(_,k)| !act.iter().any(|(_,ak)| ak==k)).map(|(_,k)| k.as_str()).collect();
        s.push_str(&format!("  ★필터로 제외됨(비활성) = {:?}\n", filtered_out));
        s.push_str(&format!("  MOD_REGISTRY {}개, MOD_FINALS {}개\n",
            MOD_REGISTRY.lock().unwrap_or_else(|e| e.into_inner()).len(),
            MOD_FINALS.lock().unwrap_or_else(|e| e.into_inner()).len()));
        for ri in 0..MAX_ROWS {
            if let Some(row) = find_node(&ui.root, &format!("row{}", ri)) {
                s.push_str(&format!("  --- row{} 서브트리 (챔프키 위치찾기) ---\n", ri));
                unsafe { dump_subtree(row, 2, &mut s); }
            }
        }
        write_log("item_tactics_diag.txt", &s);
    }

    // 옵션 주입 (화면 진입당 1회). 옵션 라벨은 진입 시 1회만 계산(파일I/O 캐시).
    //   각 칸 초기 표시 = SEL_BY_CHAMP[(그 행 챔프, slot)] (champ-keyed 영속). 없으면 0(자동).
    if !OPTS_INJECTED.swap(true, Ordering::Relaxed) {
        let opts = compute_options();
        *OPTS_CACHE.lock().unwrap_or_else(|e| e.into_inner()) = opts.clone();
        let refs: Vec<&str> = opts.iter().map(|s| s.as_str()).collect();
        let mut last = LAST_SEL.lock().unwrap_or_else(|e| e.into_inner());
        let mut injected = 0;
        let pt_sz = PT_SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner()).as_ref().map(|m| m.len()).unwrap_or(0);
        let mut diag = format!("[{}ms] 옵션주입(행→챔프 매핑) PT_SNAPSHOT={}개 opts={}종\n", now_ms(), pt_sz, opts.len());
        for ri in 0..MAX_ROWS {
            let Some(row) = find_node(&ui.root, &format!("row{}", ri)) else { continue; };
            let champ = row_champ(row);
            diag.push_str(&format!("  row{} champ={:?}\n", ri, champ));
            for si in 0..slot_count() {
                let iid = slot_dd_id(si);
                // 표시 우선순위: 사용자선택(SEL_BY_CHAMP) > 게임 personal_tactics(PT_SNAPSHOT 바닐라) > Auto.
                //   → NOP로 끊긴 바닐라 표시도 personal_tactics 에서 정확히 복원.
                let (sel_v, pt_v) = if let Some(c) = champ.as_ref() {
                    (with_sel(|m| m.get(&(c.clone(), si as u8)).copied()),
                     PT_SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner()).as_ref().and_then(|m| m.get(c)).and_then(|b| b.get(si).copied()))
                } else { (None, None) };
                // ★ SEL=0(맡김/스퓨리어스)은 오버라이드 아님 → delegate(PT) 값 표시. SEL≥1 만 유저픽으로 이김.
                //   SEL_AUTO(=조합테스트 전용 명시 Auto)는 일반 키에 올 수 없지만 방어적으로 제외.
                let cur = sel_v.filter(|&v| v >= 1 && v != SEL_AUTO).or(pt_v).unwrap_or(0);
                diag.push_str(&format!("    slot{}: SEL={:?} PT={:?} → cur={}\n", si, sel_v, pt_v, cur));
                let cur = (cur as usize).min(opts.len().saturating_sub(1)) as u64;
                if unsafe { nat_dd_set_options(row, &iid, &refs, cur) } {
                    injected += 1;
                    last[ri * ITEM_SLOTS + si] = cur as i64;
                    unsafe { set_dd_max_height(row, &iid, MAX_ITEMS_HEIGHT); }
                }
            }
        }
        append_log("item_tactics.txt", &format!("[{}ms] 옵션 주입 {}칸 (옵션 {}종, 모드템 {}개)",
            now_ms(), injected, refs.len(), refs.len().saturating_sub(7)));
        write_log("item_tactics_rowchamp.txt", &diag);
        update_override_snapshot(); // 화면 진입 시 주입 스냅샷 최신화
        log_override();
    } else {
        // 선택 폴링: 변경된 칸만 SEL_BY_CHAMP(champ-keyed) 갱신 + 영속 + 로그.
        let opts = OPTS_CACHE.lock().unwrap_or_else(|e| e.into_inner()).clone();
        let mut last = LAST_SEL.lock().unwrap_or_else(|e| e.into_inner());
        let mut changed = false;
        for ri in 0..MAX_ROWS {
            let Some(row) = find_node(&ui.root, &format!("row{}", ri)) else { continue; };
            let Some(champ) = row_champ(row) else { continue; };
            for si in 0..slot_count() {
                if let Some(cur) = unsafe { nat_dd_selected(row, &slot_dd_id(si)) } {
                    let k = ri * ITEM_SLOTS + si;
                    if cur as i64 != last[k] {
                        last[k] = cur as i64;
                        // cur 0(맡김) = 엔트리 제거 → delegate 폴백. ≥1 = 유저 오버라이드 저장.
                        with_sel(|m| { if cur == 0 { m.remove(&(champ.clone(), si as u8)); } else { m.insert((champ.clone(), si as u8), cur as u8); } });
                        SEL_DIRTY.store(true, Ordering::Relaxed); // ★지정챔프 스냅샷 무효화(다음 buy에서 재빌드)
                        changed = true;
                        let label = opts.get(cur).cloned().unwrap_or_else(|| format!("idx{}", cur));
                        let modtag = if cur >= 7 {
                            mod_final_opts().get(cur - 7).map(|(id, k)| format!(" [모드템 id={} {}]", id, k)).unwrap_or_default()
                        } else { String::new() };
                        append_log("item_tactics.txt", &format!("[{}ms] {} slot{} → [{}] {}{}", now_ms(), champ, si, cur, label, modtag));
                    }
                }
            }
        }
        if changed { with_sel(|m| save_sel(m)); update_override_snapshot(); log_override(); }
    }
}
// 현재 OVERRIDE(주입대상) 맵 로그 — 검증용.
fn log_override() {
    let map = build_override_map();
    let mut s = format!("[{}ms] OVERRIDE (champ,slot)→mod_id  ({}건)\n", now_ms(), map.len());
    let mut v: Vec<_> = map.iter().collect();
    v.sort_by(|a, b| a.0.cmp(b.0));
    for ((c, slot), id) in v { s.push_str(&format!("  {} slot{} → id {}\n", c, slot, id)); }
    write_log("item_tactics_override.txt", &s);
}



// (챔프키, slot) → 주입값. c6 detour 가 소비.
//   ★2026-07-04 확장: 바닐라 픽도 포함(게임의 드롭다운→personal_tactics 커밋이 모드 환경서
//   안 일어나는 정황 → 모드가 전 픽을 직접 강제).
//   값 인코딩: 0=맡김(Auto), 1~6=바닐라 카테고리(tactics 바이트 강제→게임 JT가 처리),
//              30+=모드템 게임 ID(build 버퍼 write + tactics zero).
fn build_override_map() -> HashMap<(String, u8), u64> {
    let finals = mod_final_opts(); // (id, key) 순서 = 옵션 idx-7
    let mut out = HashMap::new();
    // ── ① delegate(팀파매gg "아이템 자동선택") 베이스라인 병합 ──────────────────────
    //   tfm2_meta_item_delegate 가 champion_personal_tactics(Team+0x348) 에 쓴 카테고리
    //   방향(1~6)을 slot0/1/2 에 깔음. PT_SNAPSHOT = 그 맵의 최신 캡처.
    //   sim c6c430 이 Team+0x348 를 직접 읽지만(ghidra-re case a), 서버측 Team 복사본/
    //   타이밍에 무관하게 확실히 적용되도록 c6 주입에도 폴드. 값 1~6 → c6 가 게임 대표템
    //   (VANILLA_FINAL {4,24,9,14,19,29})로 변환 = 게임 JT 원래 동작과 비트동일(멱등·무해).
    //   ⚠ delegate 는 slot0~2(3칸)만. slot3(4번째)=compute_auto_4th_id 자동 유지.
    if let Some(snap) = PT_SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner()).as_ref() {
        for (champ, bytes) in snap.iter() {
            for slot in 0u8..3 {
                let b = bytes[slot as usize];
                if (1..=6).contains(&b) {
                    out.insert((champ.clone(), slot), b as u64);
                }
            }
        }
    }
    // ── ② 사용자 지정(SEL_BY_CHAMP) 이 delegate 베이스라인을 덮음 ──────────────────
    //   idx 0(맡김) = 명시 덮어쓰기 안 함 → delegate/auto 로 폴백(폴링 스퓨리어스 0 클로버 방지).
    with_sel(|m| {
        for ((champ, slot), &idx) in m.iter() {
            // ★조합테스트 진영 스코프 키(@b:/@r:)는 이 맵의 대상이 아니다(2026-07-30):
            //   이 override(c6/personal_tactics 경로)는 챔프 단위라 진영을 표현할 수 없고,
            //   조합테스트 주입은 buy 경로가 스코프째로 처리한다. 넣으면 없는 챔프 엔트리만 늘어난다.
            if is_scoped(champ) { continue; }
            if idx >= 7 {
                if let Some((id, _)) = finals.get(idx as usize - 7) {
                    out.insert((champ.clone(), *slot), *id); // 모드템 게임 ID(30+)
                }
            } else if idx >= 1 {
                out.insert((champ.clone(), *slot), idx as u64); // 1~6 = 바닐라 카테고리
            }
            // idx 0(맡김/Auto) = 유저가 이 슬롯을 명시 덮지 않음 → ①의 delegate 베이스라인 유지
            //   (delegate 값 없으면 out 에 없음 = c6 미개입 = 게임 신경망 자율).
        }
    });
    out
}

// 바닐라 카테고리(1~6) → 최종템 게임 ID. 게임 c6 JT 변환과 동일(cat1=AD..cat6=Hp).
//   ⚠ churn: 게임 아이템 트리 바뀌면 이동 가능(현재 0.4.14). 게임 JT(0x143441cf4 등)의 상수와 일치.
const VANILLA_FINAL: [u64; 6] = [4, 24, 9, 14, 19, 29];

// ===========================================================================
//  Phase 2c — 라이브 매치 빌드 주입 (FUN_140c6c430 후보루프 mid-func detour)
// ===========================================================================
// detour 가 읽는 lock-free 스냅샷: (champ bytes, slot, mod_id). 선택 변경 시 새로 leak·교체.
//   sim 은 병렬(rayon) → 락 회피 위해 AtomicPtr 스냅샷(불변, 절대 free 안함=UAF 없음).
type OvEntry = (Vec<u8>, u8, u64);
static OVERRIDE_SNAPSHOT: AtomicPtr<Vec<OvEntry>> = AtomicPtr::new(core::ptr::null_mut());
static SNAP_SIG: AtomicU64 = AtomicU64::new(u64::MAX); // 직전 스냅샷 시그니처(변경감지)
fn update_override_snapshot() {
    let map = build_override_map();
    let mut v: Vec<OvEntry> = map.into_iter().map(|((c, s), id)| (c.into_bytes(), s, id)).collect();
    v.sort(); // 결정적 순서 → 시그니처 안정
    // ★ 매 프레임 호출 안전용: 내용 무변경이면 재빌드/leak 스킵(delegate 는 매프레임 쓰지만 대개 불변).
    let mut sig: u64 = 0xcbf29ce484222325;
    for (c, s, id) in &v {
        for &b in c { sig = (sig ^ b as u64).wrapping_mul(0x100000001b3); }
        sig = (sig ^ *s as u64).wrapping_mul(0x100000001b3);
        sig = (sig ^ *id).wrapping_mul(0x100000001b3);
    }
    if sig == SNAP_SIG.swap(sig, Ordering::Relaxed) { return; }
    let boxed = Box::into_raw(Box::new(v));
    // 기존 스냅샷은 leak(detour 가 다른 스레드서 읽는 중일 수 있음 → free 금지). 변경시만 발생=유한.
    OVERRIDE_SNAPSHOT.store(boxed, Ordering::Release);
}


static PLAYER_TEAM_ID: AtomicU64 = AtomicU64::new(u64::MAX); // u64::MAX=미캡처(스코프 미적용=폴백)
// ═══════════════════════════════════════════════════════════════════════════
//  ★★내 팀 판정 v15 (07-19, ai_adjust team_gate 패턴 이식) — scene tag9 불필요.
//    db.player_team_id() → db.team(tid).last_starting(선발 5명 athlete_id) → HashSet 게시.
//    sim 쪽에선 athlete+0x810(athlete_id)을 읽어 멤버십 대조 = 내 팀.
//    ⟹ 스폰(SelectLineup, tag9 前)에도 성립 → v14 스폰 커밋훅의 '내팀=0' 병목 해소.
//    ⚠A2 정적확정: sim 계층엔 team_id 자체가 부재(provider vtable 78슬롯 전수 게터 0개).
//      sim서 team_id/match_id 스캔 = 막다른길(재시도 금지). athlete_id 멤버십이 유일 경로.
//    ⚠오프셋: 0.5.1 = +0x810. (구 0x698은 0.4.x STALE — ai_adjust 소스에 아직 남아있으나 별건 TODO.
//      0x6a8도 athlete_id 아님(실측 전부 0, 라벨 오답). 0x810만 사용할 것.)
// ═══════════════════════════════════════════════════════════════════════════
// ★0.5.3 검증완(2026-07-29): athlete ctor 0x22cb050→**0xed32b0**의 3연속 스토어 관용구가 명령 단위로 동일
//   (`mov [rsi+0x810],reg` / `+0x818,0` / `+0x820,rax`, reg←rdx=arg2) ⟹ **+0x810 유지**.
//   교차검증 = 로스터 순회 0x1740380 `add rbx,0x8d0` → `mov r12,[rbx+0x810]` / VIEW 시그 0xee9070
//   (`[rcx+0x840]` 배열·`[rcx+0x848]` count·`imul rcx,r9,0x8d0`) ⟹ **stride 0x8d0도 유지**.
//   athlete 레이아웃 전체 불변 확인: champ String +0x418/0x420/0x428 · items Vec +0x448/0x450/0x458
//   · build Vec +0x490/0x498/0x4a0 · id +0x810 · team +0x820 · gold +0x888 · position(dword) +0x8b0 · 사본 0x8b8.
const O_ATHLETE_ID: usize = 0x920; // 0.5.5(구0.5.4=0x800, +0x120 시프트 — athlete ctor 0x102bf00 store 정렬 확정). id 0x800→0x920 / team 0x810→0x930 / pos 0x8a0→0x9c0 / stride 0x8c0→0x9e0. 0x408~0x520 구간 필드는 +0x60(champ String·owned·build Vec).
static MY_ATHLETES: AtomicPtr<std::collections::HashSet<u64>> = AtomicPtr::new(core::ptr::null_mut());
static MY_ATH_PREV: AtomicPtr<std::collections::HashSet<u64>> = AtomicPtr::new(core::ptr::null_mut());
static MY_ATH_N: AtomicU64 = AtomicU64::new(0); // 게시된 선발 인원수(0=미확보)
static ROSTER_TICK: AtomicU64 = AtomicU64::new(0);
// ★★2026-08-06 신설(유저 제보 "가끔 지정 빌드를 안 따른다" 실측 대응) ─────────────────────
//   실측 근거 = `buy_report.txt` 의 `MY_ATHLETES 게시 보류 : 5회`.
//   이 세이브는 pid=0(비0 관측 0회)이라 `trust` 가 `PID_ZERO_CLEAN >= 600` 에만 의존했고,
//   `PID_ZERO_CLEAN` 은 **InGame 프레임에서만** 증가한다 ⟹ 게임을 켜고 처음 들어간 경기의
//   **첫 600프레임(≈10초) 동안 팀 게이트가 통째로 닫혀** 지정이 하나도 주입되지 않았다.
//   게시 시도 자체도 120프레임 격자에서만 일어나 최대 2초가 더 붙는다(보류 5회 = 0/120/…/480).
//   그 사이에 산 슬롯은 `if owned > si { continue }`(buy_replace_ctx) 때문에 **그 경기 내내 복구 불가**다.
static CT_EVER_SEEN: AtomicU64 = AtomicU64::new(0); // 이 세션에서 조합테스트 컨텍스트를 한 번이라도 봤나
static ROSTER_FORCE: AtomicBool = AtomicBool::new(false); // 다음 기회에 격자 무시하고 재게시(pid 정정 시)
static ROSTER_SIG: AtomicU64 = AtomicU64::new(0);   // 게시된 선발 집합 지문(무변경 재게시 억제)
// 내 팀 로스터를 게시해도 되는가. `known != 0` 이면 무조건 OK.
// pid=0 은 07-30 에 "조합테스트 컨텍스트가 만들어낸 가짜 0"일 수 있다는 이유로 600틱 관측을 요구했는데,
// **그 오염원(조합테스트)을 이 세션에서 한 번도 안 봤다면 기다릴 이유가 없다.** 방어의 의도는 보존하고
// 대기만 없앤다 ⟹ 조합테스트를 먼저 한 세션에서는 기존대로 600틱 규칙이 그대로 걸린다.
fn roster_trust(known: u64) -> bool {
    known != 0
        || CT_EVER_SEEN.load(Ordering::Relaxed) == 0
        || PID_ZERO_CLEAN.load(Ordering::Relaxed) >= 600
}
// 선발 집합 지문(정렬 FNV) — 무변경이면 재게시하지 않는다(publish_my_athletes 는 Box leak 세대교체 패턴).
fn roster_sig(my: &std::collections::HashSet<u64>) -> u64 {
    let mut v: Vec<u64> = my.iter().copied().collect();
    v.sort_unstable();
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for id in v { h = (h ^ id).wrapping_mul(0x100000001b3); }
    h
}
// 내 팀 선발 로스터 게시(swap 후 직전본 지연해제 — sim 스레드가 읽는 중일 수 있어 즉시 free 금지).
// ★진단(2026-07-30): 게시된 내 팀 선발 athlete_id 실값. "왜 이 선수가 내 팀으로 잡히나"를
//   추측 없이 판정하기 위한 것(특히 aid=0 같은 센티널이 섞여 광범위 false positive 가 되는 경우).
static MY_ATH_IDS: [AtomicU64; 8] = [const { AtomicU64::new(u64::MAX) }; 8];
fn publish_my_athletes(set: std::collections::HashSet<u64>) {
    { let mut v: Vec<u64> = set.iter().copied().collect(); v.sort_unstable();
      for k in 0..8 { MY_ATH_IDS[k].store(v.get(k).copied().unwrap_or(u64::MAX), Ordering::Relaxed); } }
    MY_ATH_N.store(set.len() as u64, Ordering::Relaxed);
    let boxed = Box::into_raw(Box::new(set));
    let old = MY_ATHLETES.swap(boxed, Ordering::AcqRel);
    let stale = MY_ATH_PREV.swap(old, Ordering::AcqRel);
    if !stale.is_null() { unsafe { drop(Box::from_raw(stale)); } }
}
// athlete_id가 내 팀 선발인가. 로스터 미확보(관리화면 방문 전)면 None = 판정보류(호출측이 결정).
#[inline]
unsafe fn is_my_athlete(athlete: usize) -> Option<bool> {
    let p = MY_ATHLETES.load(Ordering::Acquire);
    if p.is_null() || (*p).is_empty() { return None; }
    let aid = safe_read_u64(athlete + O_ATHLETE_ID)?;
    // ⛔**`aid==0` 차단은 넣었다가 제거했다(2026-07-30)** — 경위를 남긴다:
    //   ①`pid=0` / `MY_ATHLETES=[0,1,2,3,4]` 를 "db 오보고"로 의심해 aid=0 매칭을 막았으나,
    //   ②실측으로 **팀 id 0 · 선수 id 0 이 실존하는 세이브**가 확인됐다(일반 경기까지 돌려도 비0 pid
    //     관측 0회 / 배경 buy 에서 aid 1~4 가 경기마다 다른 챔프로 등장 = 진짜 내 팀 선수).
    //   ⟹ 차단을 유지하면 **선발 5명 중 1명(20%)의 지정이 조용히 미적용**되는 순수 손실이다.
    //   미기입 athlete(+0x810=0) 방어라는 원래 목적은, athlete_id 0 이 실존하는 세이브에서는
    //   **애초에 구분이 불가능**하므로 이 지점에서 막을 문제가 아니다(진짜 원인이었던 pid 오판은
    //   조합테스트 컨텍스트 0 무시 + 팀0 인정 규칙으로 해결됨 = `02_구현정보.md §12`).
    Some((*p).contains(&aid))
}
// ★buy-path 팀게이트: sim athlete엔 전역 team_id 경로 없고 side(+0x820, 0/1)만 있음(ghidra-re). player가 어느
//   side인지 = 유저 지정/PT 챔프가 많은 side로 다수결 판정. 매치마다 리셋(before_management_tick). 적팀=지정 스킵.
static PLAYER_SIDE: AtomicU64 = AtomicU64::new(u64::MAX);        // 0/1, u64::MAX=미판정(폴백=적용)
static D_WROTE: AtomicU64 = AtomicU64::new(0);    // 실제 build[si] write 발생


fn is_skill_key(k: &str) -> bool {
    k.contains("_skill") || k.contains("_passive") || k.contains("_ult") || k.contains("_slow")
        || k.contains("_stack") || k.contains("_buff") || k.contains("_curse") || k.contains("_road")
        || k.contains("move_speed") || k.contains("_aura") || k.contains("_mark")
}


const RVA_REALLOC: usize = 0x2a9fb50; // 0.5.6(구0.5.5=0x2a87a70, skel UNIQUE·BYTE=SAME·size 174). // 0.5.5(구0.5.4=0x29a7640, exe2exe 스켈레톤 확정). // 0.5.4(구0.5.3=0x28e3b10). 본문 100% 동일·콜러 3개 동수. // 0.5.3(구0.5.2=0x25c4dd0). __rust_realloc 실함수. (rcx=ptr,rdx=old,r8=align,r9=new)->rax. 구 exe 진입 112B 마스크시그 → 신 exe 유일 1히트 + 본문 명령 대 명령 동형(mov rdi,r9 / mov rsi,rcx / cmp r8,0x11 / jae).
type ReallocFn = unsafe extern "win64" fn(usize, usize, usize, usize) -> usize;
static EXE_BASE_CACHE: AtomicUsize = AtomicUsize::new(0);
fn exe_base_addr() -> usize {
    let b = EXE_BASE_CACHE.load(Ordering::Relaxed);
    if b != 0 { return b; }
    let v = unsafe { GetModuleHandleW(core::ptr::null()) as usize };
    EXE_BASE_CACHE.store(v, Ordering::Relaxed);
    v
}

// ═══ 파이프라인 발화 지도 (count-only 진입 프로브) — 관전경기서 어느 셋업단계가 도는지 실측 ═══
//  0=score-many(0x100d150) 1=megafunc/team게이트(0x1447850) 2=로스터젠(0x11b77a0). 프롤로그 동일(8push 55..).
static FIRE: [AtomicU64; 4] = [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];
static NN_ID_NAME: Mutex<Option<HashMap<u64, String>>> = Mutex::new(None);

// ═══ 경기시작 launcher 훅(0.5.1 RE) — 렌더 경기 시드 결정적 캡처 ═══
//   launcher 0x20588a0(out=rcx, flag=dl, seed=r8, r9) ← 클라 렌더 씬빌더 0x722ca0가 호출(콜러=렌더 판별).
//   retaddr rva ∈ [0x722ca0, 0x732ca0)면 렌더 경기 → LIVE_SEED=seed(r8). buy훅 sim_seed==LIVE_SEED 게이트.
const CL_LAUNCHER_RVA: usize = 0x106dd60; // 0.5.6(구0.5.5=0x14ac3e0). head-UNIQUE·마스크시그 UNIQUE·launcher 콜사이트 9/9 전단사(retaddr 재핀 근거)·size 4401→4398·프롤로그14 동일(chkstk imm 0x25438 불변=CL_LAUNCHER_PROLOGUE 무수정). // 0.5.5(구0.5.4=0x13b53d0). head 유일후보·콜러 9개 동수(지문 1:1)·내부 mov[rsi+0x1dc0]/[rsi+0x1dc8] 스토어 존재·size 4432→4401. ⚠프롤로그 chkstk imm 0x25168→0x25438(아래 배열 index13 변경). // 0.5.4(구0.5.3=0xeb8810). 크기 0x1150·명령수 787·콜러 9개 전부 동일 + Game+0x1dc0/0x1dc8 스토어 동일 + 콜러 컨테이너 8개 전부 소스파일 지문 일치. // 0.5.3(구0.5.2=0x1d96870). 확정 근거: ①프롤로그 관용구 동형(8push+mov eax,frame+call chkstk+lea rbp,[rsp+0x80]+xmm 스필+[rbp+X]=-2) ②콜러 **9곳 = 구 exe와 동수** ③렌더 씬빌더(0x997740)가 2회 호출 ④내부에서 seedctor(0x12b9ab0)를 rdx=저장된 r8(seed)로 호출 = 구 exe 라인대응. 진입 시 r8=seed 계약 유지(mov r12,r8).
const CL_LAUNCHER_PROLOGUE: [u8; 17] = [0x55, 0x41,0x57, 0x41,0x56, 0x41,0x55, 0x41,0x54, 0x56, 0x57, 0x53, 0xb8, 0x18, 0x54, 0x02, 0x00]; // 0.5.5: 8push+mov eax,0x25438 (chkstk 프레임 0x25168→0x25438, index13 0x68→0x38). // 0.5.3: 8push+mov eax,0x25108 (구 0.5.2=0x165c8) — chkstk 프레임만 확대
static CLAUNCH_INSTALLED: AtomicU64 = AtomicU64::new(0);
static LAUNCH_N: AtomicU64 = AtomicU64::new(0);
static LAUNCH_RENDER_N: AtomicU64 = AtomicU64::new(0);
static LAUNCH_RVAS: [AtomicU64; 24] = [const { AtomicU64::new(0) }; 24]; // 진단: 고유 콜러 rva(format/파일쓰기 없이=거대프레임 오버플로 방지)
static LAUNCH_RENDER_RA: AtomicU64 = AtomicU64::new(0); // 렌더로 판정된 retaddr rva
// ★현재 경기가 조합테스트인가. 조합테스트는 유저가 블루/레드 양쪽을 직접 구성하는 샌드박스라
//   "내 팀"이라는 개념이 없다 → 팀 게이트를 우회하고 지정 챔프면 양 진영 모두 적용.
static COMPTEST_MATCH: AtomicBool = AtomicBool::new(false);
static LAUNCH_ERR_N: AtomicU64 = AtomicU64::new(0);     // launcher 설치실패 로깅 카운트(≤3)
static CLAUNCH_STUB: AtomicU64 = AtomicU64::new(0);     // 우리 launcher 스텁 주소(진입부 재검증용)
static LAUNCH_WAIT: AtomicU64 = AtomicU64::new(0);      // serpen 설치 대기 프레임 카운트
// ⚠최소 detour: launcher는 91KB chkstk 프레임 + 배경 30~40경기마다 발화 → format!/fs/락/catch_unwind 금지(스택오버플로).
//   본문은 raw read + 원자연산만(패닉원천 없음 → catch_unwind 불요).
unsafe extern "C" fn cap_launcher(saved: *mut u64, _e: usize) -> u64 {
    // ⚠최소 디투어 제약 유지 — 프로브도 rdtsc + 전역 원자연산만(rec_tl=TLS 지연초기화 경로 금지).
    let __lt = perf::tsc();
    if saved.is_null() { perf::rec(perf::S_LAUNCHER, __lt); return 0; }
    let seed = *saved.add(2);      // r8 = arg3 = seed
    // ★★2026-08-07 신설 — **rcx = Game 객체**(지금까지 버리고 있던 인자).
    //   근거(0.5.4 exe): launcher 가 이 포인터(→rsi)에 쓰는 필드가 spawn·buy 가 읽는 Game 필드와 정확히 일치 —
    //   `[rsi+0x1dc0]`/`+0x1dc8`/`+0x1dd0`/`+0x1dd8`(provider 계열) · `+0x1fc8`/`+0x1fd8`(카탈로그 Vec) · `+0x2060`.
    //   ⚠**이 포인터가 매치 내내 유효한지는 미확정** — 콜사이트가 `lea rcx,[rbp+0x35fb0]` = **호출자 스택**이라
    //     launcher 가 거기 Game 을 "생성"하는 것이고, 이후 영속 위치로 옮겨질 수 있다(Rust move).
    //     ⟹ 지금은 **관측만** 한다. buy 시점 Game 과 대조해 stale 여부를 실측한 뒤에 쓸 것.
    let game_arg = *saved;         // rcx = arg1 = Game (검증중)
    let retaddr = *saved.add(10);  // 콜사이트 retaddr(스텁 push 10개 위)
    let base = GetModuleHandleW(core::ptr::null()) as u64;
    if base == 0 || retaddr < base { perf::rec(perf::S_LAUNCHER, __lt); return 0; }
    let rva = retaddr - base;
    LAUNCH_N.fetch_add(1, Ordering::Relaxed);
    // ★콜러=클라 렌더 씬빌더 0x722ca0 범위 → 렌더 경기 시드
    // ★serpen 정본(CURRENT_MATCH_DETECT.md, 인게임검증): 화면경기 콜사이트 = 정확히 0x72f507(경로A)·0x733e9f(경로B). 0x2061132=배경.
    // ★조합테스트(comp_test) 추가(07-21 ghidra-re 확정): retaddr 0xc884fa(콜사이트 0xc884f5, 함수 0xc831b0).
    //   도달 경로 단일(디스패치 arm 31 → 0x75fe90 → 0xc831b0)이라 배경과 안 섞임. 관측된 나머지 3개는 전부 배경:
    //   0x13dd5a0=solo_rank / 0x1659d55=server::worker / 0x2061137=틱드라이버 → 절대 넣지 말 것.
    //   r8=seed 전달 형태가 일반 관전과 동일해 캡처 로직 그대로 재사용.
    // ★0.5.2 재매핑(exe2exe 콜사이트 재열거 2026-07-22): 렌더 씬빌더 컨테이너 0x722ca0→0x74d510(니모닉 0.9928)
    //   의 launcher 콜 2개 retaddr = 0x72f507→0x759c36 / 0x733e9f→0x75e5cf.
    //   comptest 컨테이너 0xc831b0→0xd405c0(launcher 콜러 9/9 bijection의 잔여 1쌍·단일콜러 0x75fe90→0x78a5c0 동형)
    //   retaddr 0xc884fa→0xd40a63. ⬜comptest만 잠정(컨테이너 크기 0x5b8f→0xce1 축소=리팩터, ghidra-re 확인 권장).
    // ★0.5.3 재매핑(2026-07-29, launcher 0xeb8810 콜사이트 실측 재열거):
    //   렌더 씬빌더 컨테이너 0x74d510→0x997740(콜러수·크기 지문 일치)의 launcher 콜 2개
    //   retaddr = 0x759c36→**0x9a3287** / 0x75e5cf→**0x9a7b03**(둘 다 컨테이너 내부 실측).
    //   comptest 컨테이너 0xd405c0→0x1925ab0(크기 0xce1→0xf5a·단일 콜러 지문 일치) retaddr 0xd40a63→**0x1925f12**.
    //   ⬜comptest는 0.5.2 때와 마찬가지로 잠정(단일콜러 체인 매칭까지만, 인게임 미확인).
    // ★launcher 콜러 9곳 성격 = 전수 확정(2026-07-30 풀 RE, panic Location 파일·라인 + 패킷 디스패치 arm 도달성):
    //   0x9a3287  = 관전(arm75 SpectateGameStart)   ★화면
    //   0x9a7b03  = 내 경기(arm30 GameStart)        ★화면
    //   0x1925f12 = 조합테스트 본경기(arm31 CompTestStarted, data.rs:1545)  ★화면·양진영 유저구성
    //   0x18f718e = 조합테스트 **기록 다시보기**(training_ui.rs:4351)        ★화면·양진영 유저구성
    //   0x229ad94 = 리플레이(pause_ui.rs:2332 — serpen 이 쓰는 값)
    //   0x220acb(state.rs 앱 상태머신) / 0x195c5be(server\worker.rs) / 0x20dac9c(solo_rank.rs)
    //   0x2256a6d(solo_rank_ui.rs) = **배경 sim → 절대 넣지 말 것**
    // 0.5.5 재핀(구0.5.4→0.5.5): 관전 0x9e2079→0x763329 · 내경기 0x9e6feb→0x76829b · 조테본경기 0x235c382→0x1aed292 · 조테기록 0x2323ffe→0x1aa88ce. (전부 e8 call+5, 컨테이너 지문 1:1)
    // 0.5.6 재핀(구0.5.5→0.5.6, launcher 콜사이트 9/9 전단사·retaddr=콜+5):
    //   관전 0x763329→**0x8404e1**(콜 0x8404dc) · 내경기 0x76829b→**0x84544b**(콜 0x845446)
    //   · 조테본경기 0x1aed292→**0x1af18a2**(콜 0x1af189d) · 조테기록 0x1aa88ce→**0x1ac1b2e**(콜 0x1ac1b29).
    let is_comptest = rva == 0x1af18a2 || rva == 0x1ac1b2e;
    if (rva == 0x8404e1 || rva == 0x84544b || is_comptest) && seed != 0 {
        let prev = LIVE_SEED.swap(seed, Ordering::Relaxed);
        if prev != seed { RENDER_PROVIDER.store(0, Ordering::Relaxed); } // 새 경기 시드 → 직후 ctor가 provider 재캡처
        COMPTEST_MATCH.store(is_comptest, Ordering::Relaxed);
        LAUNCH_RENDER_N.fetch_add(1, Ordering::Relaxed);
        LAUNCH_RENDER_RA.store(rva, Ordering::Relaxed);
        LIVE_GAME.store(game_arg, Ordering::Relaxed); // ★08-07: 화면 경기 Game 후보(유효성 검증중)
        BUY_GAME.store(0, Ordering::Relaxed);         // 새 매치 → buy 측 관측 리셋
        GAME_PROBE_DONE.store(false, Ordering::Relaxed);
    }
    // ★08-08 대회(리그) 배경 경기 — 디스크립터 보험 캡처(아래 tourn_capture 참조. 최소 디투어 제약 준수).
    //   ★v2.9.5(08-14): 프레임 스캔 단독으로 단순화 — 레지스터 캡처(TNR post-call 훅)는 launcher 시점
    //   레지스터에 P가 없어 커버리지 3% 한계·구조상 100% 불가(RE\2026-08-14) → 제거. 프레임 스캔이 이미
    //   2469/2469=100% 커버(교차검증 v2.9.4 NG=0 완료). caller_rbp 프레임슬롯이 팀정보의 유일 100% 소스.
    if TN_ENABLED && rva == RA_TOURN_055 { tourn_capture(saved, seed); }
    // 진단: 고유 콜러 rva 수집(원자 CAS, 24슬롯)
    for k in 0..24 {
        let s = LAUNCH_RVAS[k].load(Ordering::Relaxed);
        if s == rva { break; }
        if s == 0 && LAUNCH_RVAS[k].compare_exchange(0, rva, Ordering::Relaxed, Ordering::Relaxed).is_ok() { break; }
    }
    perf::rec(perf::S_LAUNCHER, __lt);
    0
}

// ═══ ★2026-08-08 대회(리그) 배경 경기 디스크립터 보험 ═══
//   정적 RE 정본 = REPORT\tfm2_item_tactics\RE\2026-08-08_대회배경-실행레코드-팀id-레시피.md (0.5.4).
//   worker.rs 콜러(0x2392ed0, launcher ret rva 0x239f242)의 프레임에서 실행 레코드
//   (cfg+0x2a0/+0x2d0 hashbrown 맵, 엔트리 0x160=키8+값0x158)를 역스캔해 두 팀 id(+0x140/+0x148)를
//   경기 "생성 시점"에 획득한다 — 로스터(is_my_athlete) 판정의 두 번째 보험(유저 지시 08-08).
//   ⚠launcher r8 시드는 레코드에 없다(worker가 콜 직전 TLS PRNG 즉석 생성 = 시드 대조 자기검증 불가·DONE.md)
//   ⟹ 3중 자기검증: ①[rbp+0x1cde8]==LIVE_DB ②[rbp+0x1cce0](set_end)이 레코드 세트Vec 범위 내·0x100 정렬
//     ③rec+0x151==dl(맵 바이트)·rec+0x138==엔트리키·rec+0x150==0(미완료). 스캔 miss = 대회 아님 폴백(안전).
//   ⛔팀키 프레임 슬롯([rbp+0x1cdc8]/[rbp+0x1cdd8]) 직독 금지 — launcher 시점 클로버(DONE.md).
//   ⬜런타임 실측 2건(registry [대회 디스크립터 보험] 블록으로 판독):
//     ①스캔 성공률(TN_MISS_SCAN>0 = set_end 슬롯이 무효인 경로 존재) ②+0x140/+0x148 의 side0(blue) 대응
//       — 내 팀 매치의 buy 에서 (레코드 슬롯, 세트 side 바이트) ↔ 실제 athlete side(+0x810) 투표로 확정.
//   ~~현 단계 = 관측 전용~~ → v2.8.0(08-08)에서 게이트 연결됨(tn_my_side, 추가 승인 전용·차단 미사용).
const TN_ENABLED: bool = true;
const RA_TOURN_055: u64 = 0x1da7a65; // 0.5.6(구0.5.5=0x1c777d8). worker 0x1f16ea0 내 대회 배경 launcher retaddr(콜 0x1f24063, e8+5·launcher 9/9 전단사로 확정). // 0.5.5(구0.5.4=0x239f242). worker.rs:36 대회 배경 launcher retaddr(콜 0x1c777d3, e8+5)
// ★★0.5.5 프레임 슬롯 재핀(2026-08-13): worker 0x1c6a530(구 0x2392ed0)의 프레임이 0x1ceb8→0x22cc8로
//   확대되며 슬롯 전면 시프트 — 0.5.5 인게임 실측(registry 08-13: 런처발화 120·스캔성공 0·db관측 0x1388
//   =비포인터)으로 TN 전멸을 확인하고 ghidra-re 재핀. caller_rbp = 진입rsp+0x88 공식과 cfg 맵·레코드
//   레이아웃(+0x2a0/+0x2d0·0x160·+0x140/+0x148·세트 0x100/+0xf8)은 전부 불변.
//   근거 = RE\2026-08-13_대회레코드-프레임오프셋-0.5.5재핀.md (기록/판독 사이트 명령 단위 대응).
//   ⚠교훈: 콜러 "프레임 오프셋" 레시피는 migrate_rva.py(RVA 마이그)가 못 잡는다 — 버전업 때 별도 재핀 필수.
// ★★0.5.6 프레임 슬롯 재핀(2026-08-20): worker 0x1c6a530→0x1f16ea0(정렬 ratio 0.956) 프레임 확대
//   (chkstk 0x22cc8→0x23ce8)로 슬롯 전면 시프트. caller_rbp=진입rsp+0x88 공식·레코드/cfg맵 레이아웃 불변.
//   판독/기록 사이트 명령 단위 대응: DB [rbp+0x23c18]@0x1f23fd0 · CFG [rbp+0x23c00]@0x1f16f59 ·
//   SETEND [rbp+0x23b20]@0x1f23f7c(콜사이트 판독). ⚠프레임 오프셋은 migrate_rva가 못 잡는 별개 축.
const TN_FR_DB: usize = 0x23e48;     // 0.5.6(구0.5.5=0x22bf8). db 슬롯 — 판독 0x1f23fd0(콜 직전 생존)
const TN_FR_CFG: usize = 0x23e28;    // 0.5.6(구0.5.5=0x22bd8). cfg 슬롯 — 기록 0x1f16f59(진입 rdx)
const TN_FR_SETEND: usize = 0x23d68; // 0.5.6(구0.5.5=0x22a40). set_end 슬롯 — 콜사이트 판독 0x1f23f7c
static TN_SEEN: AtomicU64 = AtomicU64::new(0);       // 대회 retaddr 런처 발화 수
static TN_HIT: AtomicU64 = AtomicU64::new(0);        // 레코드 스캔 성공
static TN_MISS_FRAME: AtomicU64 = AtomicU64::new(0); // 프레임 슬롯 읽기 실패/포인터 무효
static TN_MISS_DB: AtomicU64 = AtomicU64::new(0);    // 검증① db 불일치(LIVE_DB·DB_DIRECT 둘 다 아님 — ★08-08 관측 강등)
static TN_DB_SEEN: AtomicU64 = AtomicU64::new(0);    // ★관측: [rbp+0x1cde8] 마지막 값(정체 규명용)
static TN_DB_EQ_LIVE: AtomicU64 = AtomicU64::new(0);   // 관측: seen == LIVE_DB(scene db)
static TN_DB_EQ_DIRECT: AtomicU64 = AtomicU64::new(0); // 관측: seen == DB_DIRECT(addr_of!(*ctx.database))
static TN_MISS_SCAN: AtomicU64 = AtomicU64::new(0);  // 두 맵 전체 스캔 miss(검증항목 ①의 핵심 지표)
static TN_V3_NG: AtomicU64 = AtomicU64::new(0);      // 검증③ 탈락(키불일치/완료플래그/맵바이트)
static TN_LAST_A: AtomicU64 = AtomicU64::new(0);     // 마지막 히트 팀A id(rec+0x140)
static TN_LAST_B: AtomicU64 = AtomicU64::new(0);     // 마지막 히트 팀B id(rec+0x148)
static TN_LAST_KEY: AtomicU64 = AtomicU64::new(0);   // 마지막 히트 매치키
static TN_LAST_MAP: AtomicU64 = AtomicU64::new(0);   // 마지막 히트 맵 오프셋(0x2a0/0x2d0)
static TN_LAST_SB: AtomicU64 = AtomicU64::new(0);    // 마지막 히트 set 꼬리 2바이트(+0xf8 side·+0xf9 세트번호)
static TN_MY_N: AtomicU64 = AtomicU64::new(0);       // 내 팀(PLAYER_TEAM_ID) 매치 히트 수
static TN_MY_SLOT: AtomicU64 = AtomicU64::new(0);    // 내 팀 슬롯(0=+0x140/1=+0x148)
static TN_MY_SB: AtomicU64 = AtomicU64::new(0);      // 그 세트의 side 바이트
static TN_MY_PRED: AtomicU64 = AtomicU64::new(0);    // ★v2.7.6: 매핑식 예측 내 팀 side = slot^sb^1
static TN_MY_SEED: AtomicU64 = AtomicU64::new(0);    // ★buy측 상관 키(마지막 store = 게시 완료 표식)
static TN_VOTE: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8]; // [slot*4+sb*2+ath_side] 투표
static TN_VOTE_OK: AtomicU64 = AtomicU64::new(0);    // ★v2.7.6: 실측 side == 예측(TN_MY_PRED)
static TN_VOTE_NG: AtomicU64 = AtomicU64::new(0);    // 실측 side != 예측 (0이어야 매핑식 확정 유지)
static TN_LAST_S0: AtomicU64 = AtomicU64::new(0);    // ★v2.7.6 매핑 적용: side0(blue) 팀 id = rec+0x140+(sb^1)*8
static TN_LAST_S1: AtomicU64 = AtomicU64::new(0);    // side1(red) 팀 id = rec+0x140+sb*8
// ═══ ★v2.8.0 TN 게이트(유저 지시 08-08 "여러 군데 중 하나만 걸려도 주입") ═══
//   launcher 캡처마다 (seed → side0팀, side1팀)을 링 테이블에 게시하고, buy 시점에 provider seed 로
//   조회해 "이 sim 에서 내 팀 side"를 확정한다. 기존 aid 멤버십(is_my_athlete)과 **OR 결합** —
//   멤버십이 놓친 내 선수(교체 출전·aid 미기입)도 TN 이 걸리면 주입. TN 은 **추가 승인만** 하고
//   차단에는 쓰지 않는다(음성 오판 리스크 0 — 기존 동작의 순수 상위집합).
//   발행 순서: seed=0 → s0/s1 → seed=real(Release) ⟹ 독자는 완전 발행된 엔트리만 매치(반쯤 쓴 엔트리 무해).
const TN_GATE: bool = true;
// ★링 16→64 확장(2026-08-13): 일정넘김은 배경 경기 30~40판을 rayon으로 한꺼번에 발화시킨다 —
//   런처(게시)들이 해당 sim의 buy(조회)보다 앞서 몰리면 16슬롯은 조회 전에 퇴거될 수 있다(무카운터 침묵 miss).
//   64면 한 배치 전체가 상주. miss 경로 비용 = 원자 로드 64회(수십 ns)로 무시 가능.
const TN_TAB_N: usize = 64;
static TN_TAB_SEED: [AtomicU64; TN_TAB_N] = [const { AtomicU64::new(0) }; TN_TAB_N];
static TN_TAB_S0: [AtomicU64; TN_TAB_N] = [const { AtomicU64::new(0) }; TN_TAB_N];
static TN_TAB_S1: [AtomicU64; TN_TAB_N] = [const { AtomicU64::new(0) }; TN_TAB_N];
static TN_TAB_W: AtomicU64 = AtomicU64::new(0);       // 링 쓰기 커서
static TN_TAB_ANY: AtomicBool = AtomicBool::new(false); // 테이블 비면 핫패스 즉시 스킵(원자 1로드)
static TN_GATE_EARLY: AtomicU64 = AtomicU64::new(0);  // 조기탈출에서 TN 구제 수
static TN_GATE_HIT: AtomicU64 = AtomicU64::new(0);    // is_player 판정에서 TN 승인 수
static TN_GATE_NEG: AtomicU64 = AtomicU64::new(0);    // TN이 "내 매치 아님" 확정한 조회 수(관측·차단엔 미사용)
// buy 핫패스용 — 원자 로드 + VEH 읽기 1회 + 16슬롯 스캔뿐(락/할당/커널호출 없음).
unsafe fn tn_my_side(provider: usize) -> Option<u64> {
    if !TN_GATE || !TN_TAB_ANY.load(Ordering::Relaxed) { return None; }
    let pid = PLAYER_TEAM_ID.load(Ordering::Relaxed);
    if pid == u64::MAX { return None; }
    if provider < 0x10000 || provider >= 0x0000_8000_0000_0000 { return None; }
    let seed = safe_read_u64(provider.wrapping_add(O_PROVIDER_SEED))?;
    if seed == 0 { return None; }
    for k in 0..TN_TAB_N {
        if TN_TAB_SEED[k].load(Ordering::Acquire) == seed {
            let s0 = TN_TAB_S0[k].load(Ordering::Relaxed);
            let s1 = TN_TAB_S1[k].load(Ordering::Relaxed);
            if s0 == pid { return Some(0); }
            if s1 == pid { return Some(1); }
            TN_GATE_NEG.fetch_add(1, Ordering::Relaxed);
            return None;
        }
    }
    None
}
// ═══ ★대회(리그) 배경 경기 팀 게시 공통부 (프레임 스캔 tourn_capture가 호출) ═══
//   ⚠cap_launcher 문맥에서 호출됨: 원자연산만.
//   ★이력: v2.9.2~v2.9.4에 worker 레코드를 레지스터로 캡처(TNR post-call 훅)해 프레임 의존을 줄이려 했으나,
//   launcher 시점 레지스터에 레코드 base P가 없어(RE\2026-08-14_launcher-레코드-레지스터-부재-확정)
//   커버리지 3% 한계·100% 구조상 불가로 확정 → v2.9.5(08-14)에서 TNR 훅 제거. 프레임 스캔이 유일 100% 경로.
fn tn_publish(seed: u64, ta: u64, tb: u64, sb: u64) {
    // ★v2.7.6 매핑식(RE\2026-08-08_세트side-팀슬롯-매핑.md): side0(blue)=rec+0x140+(sb^1)*8.
    let (s0, s1) = if sb == 1 { (ta, tb) } else { (tb, ta) };
    TN_LAST_S0.store(s0, Ordering::Relaxed);
    TN_LAST_S1.store(s1, Ordering::Relaxed);
    if TN_GATE && seed != 0 {
        let k = (TN_TAB_W.fetch_add(1, Ordering::Relaxed) as usize) % TN_TAB_N;
        TN_TAB_SEED[k].store(0, Ordering::Release);
        TN_TAB_S0[k].store(s0, Ordering::Relaxed);
        TN_TAB_S1[k].store(s1, Ordering::Relaxed);
        TN_TAB_SEED[k].store(seed, Ordering::Release);
        TN_TAB_ANY.store(true, Ordering::Relaxed);
    }
    let pid = PLAYER_TEAM_ID.load(Ordering::Relaxed);
    if pid != u64::MAX && (ta == pid || tb == pid) {
        let slot = u64::from(ta != pid);
        TN_MY_N.fetch_add(1, Ordering::Relaxed);
        TN_MY_SLOT.store(slot, Ordering::Relaxed);
        TN_MY_SB.store(sb, Ordering::Relaxed);
        TN_MY_PRED.store(slot ^ sb ^ 1, Ordering::Relaxed);
        TN_MY_SEED.store(seed, Ordering::Relaxed); // 마지막 store = buy측 투표 게이트 오픈
    }
}

// ⚠cap_launcher 최소 디투어 제약 상속 — VEH 보호 읽기 + 원자연산만(format!/fs/락/할당/catch_unwind 금지).
//   대회 retaddr 에서만 호출(경기 시작당 1회)·스캔 상한 mask<0x1000 ⟹ 핫패스 아님.
//   ★caller_rbp 프레임슬롯(db/cfg/set_end)에서 hashbrown 맵을 역스캔해 레코드 특정 → tn_publish 게시.
//     팀정보의 유일 100% 소스(레지스터엔 launcher 시점 P 부재 — RE\2026-08-14).
unsafe fn tourn_capture(saved: *mut u64, seed: u64) {
    TN_SEEN.fetch_add(1, Ordering::Relaxed);
    let entry_rsp = saved.add(10) as usize;      // = launcher 진입 rsp([rsp]=retaddr, 스텁 push 10개 위)
    let rbp = entry_rsp.wrapping_add(0x88);      // worker 프롤로그: 8push + sub rsp,0x22cc8(0.5.5) + rbp=rsp+0x80
    let (Some(db), Some(cfg), Some(set_end)) = (
        safe_read_u64(rbp.wrapping_add(TN_FR_DB)),     // 배경 sim db(0.5.5 판독 0x1c77740)
        safe_read_u64(rbp.wrapping_add(TN_FR_CFG)),    // cfg(배경 sim 상태 구조체)
        safe_read_u64(rbp.wrapping_add(TN_FR_SETEND)), // 현재 세트블록 끝 포인터
    ) else { TN_MISS_FRAME.fetch_add(1, Ordering::Relaxed); return; };
    // 검증① — db 포인터 대조. 경위: v2.7.4가 LIVE_DB(scene db)와 대조해 전건 오탐 차단(47/47) → v2.7.5 관측
    //   강등 → 2판째 실측 **seen==DB_DIRECT 28/28**(=addr_of!(*ctx.database), 서버 db)로 정체 확정 →
    //   ★v2.7.6 ①복원: 대조 대상 = DB_DIRECT. 미게시(0)면 대조 생략(②③이 남는다). 03_시행착오 08-08 참조.
    TN_DB_SEEN.store(db, Ordering::Relaxed);
    let kdb = LIVE_DB.load(Ordering::Relaxed);
    let ddb = DB_DIRECT.load(Ordering::Relaxed);
    if kdb != 0 && db == kdb { TN_DB_EQ_LIVE.fetch_add(1, Ordering::Relaxed); }
    else if ddb != 0 && db == ddb { TN_DB_EQ_DIRECT.fetch_add(1, Ordering::Relaxed); }
    else { TN_MISS_DB.fetch_add(1, Ordering::Relaxed); }
    if ddb != 0 && db != ddb { return; } // ★복원된 하드 검증①(DB_DIRECT 대조)
    let (cfg, set_end) = (cfg as usize, set_end as usize);
    if cfg < 0x10000 || set_end < 0x10000 { TN_MISS_FRAME.fetch_add(1, Ordering::Relaxed); return; }
    let map_b = *saved.add(1) & 0xff;            // dl = launcher arg2 = 맵 바이트
    // 0.5.6 재핀(2026-08-20): cfg hashbrown 맵 2개 ~~0x2a0/0x2d0~~ → **0x320/0x350**(+0x80 이동 —
    //  ctrl/mask 쌍 0x328/0x358 동반. 근거 = worker 본문 disp 센서스 OLD 9/9/9/8회→NEW 동수 이동
    //  + 해시 관용구 문맥 동형 @0x1f170fc. 미재핀 시 TN 프레임 스캔 전실패(6269/6269) 실사고 —
    //  ghidra-re RE = REPORT\tfm2_item_tactics\RE\2026-08-20_0.5.6-TN맵오프셋-BUY오독정정.md).
    //  엔트리 stride 0x160·세트원소 0x100·rec+0x140/0x148/0x150 은 불변 확증 = 수정 불요.
    for map_off in [0x320usize, 0x350] {
        let Some(ctrl) = safe_read_u64(cfg + map_off) else { continue };
        let Some(mask) = safe_read_u64(cfg + map_off + 8) else { continue };
        let ctrl = ctrl as usize;
        if ctrl < 0x10000 || mask >= 0x1000 { continue; } // hashbrown: ctrl 배열 포인터·bucket_mask
        let n = mask as usize + 1;
        let mut g = 0usize;
        while g < n {
            let Some(w) = safe_read_u64(ctrl + g) else { break }; // ctrl 바이트 8개 묶음
            let lim = (n - g).min(8);
            for j in 0..lim {
                if (w >> (j * 8)) & 0x80 != 0 { continue; }       // empty/deleted 버킷
                let ent = ctrl.wrapping_sub((g + j + 1) * 0x160); // 엔트리 = ctrl 아래로 (i+1)*0x160
                if ent < 0x10000 { continue; }
                let Some(key) = safe_read_u64(ent) else { continue };
                let rec = ent + 8;
                let (Some(sptr), Some(slen)) = (safe_read_u64(rec + 8), safe_read_u64(rec + 0x10)) else { continue };
                let (sptr, slen) = (sptr as usize, slen as usize);
                if sptr < 0x10000 || slen == 0 || slen > 64 { continue; }
                // 검증② — set_end 가 이 레코드의 세트 Vec(원소 0x100) 범위 내·정렬 일치
                if !(set_end > sptr && set_end <= sptr + slen * 0x100 && (set_end - sptr) % 0x100 == 0) { continue; }
                // 검증③ — 매치키 자기일치·미완료·맵 바이트 (u64 하나로 +0x150/+0x151 동시 커버)
                let (Some(k2), Some(w150)) = (safe_read_u64(rec + 0x138), safe_read_u64(rec + 0x150)) else { continue };
                if k2 != key || w150 & 0xff != 0 || (w150 >> 8) & 0xff != map_b {
                    TN_V3_NG.fetch_add(1, Ordering::Relaxed); continue;
                }
                let (Some(ta), Some(tb)) = (safe_read_u64(rec + 0x140), safe_read_u64(rec + 0x148)) else { continue };
                let Some(wtail) = safe_read_u64(set_end - 8) else { continue }; // set+0xf8 side·+0xf9 세트번호
                TN_HIT.fetch_add(1, Ordering::Relaxed);
                TN_LAST_A.store(ta, Ordering::Relaxed);
                TN_LAST_B.store(tb, Ordering::Relaxed);
                TN_LAST_KEY.store(key, Ordering::Relaxed);
                TN_LAST_MAP.store(map_off as u64, Ordering::Relaxed);
                TN_LAST_SB.store(wtail & 0xffff, Ordering::Relaxed);
                // ★v2.7.6 매핑식은 tn_publish로 게시(side0/링/내팀 매치 상태).
                let sb = wtail & 1;
                tn_publish(seed, ta, tb, sb);
                return;
            }
            g += 8;
        }
    }
    TN_MISS_SCAN.fetch_add(1, Ordering::Relaxed);
}
// ★훅 설치 경로 카운터(2026-07-22 진단): "훅 재시도"가 프레임당 189µs = 47만 사이클로 관측 —
//   early return 경로라기엔 과대. 매프레임 실제 재설치(VirtualAlloc+VirtualProtect ×N) 여부를 가린다.
//   실제 install 이 프레임당 1회면 = 스텁 누수 + serpen 상호 재체인 사이클(draft_overlay hang 전례).
static HK_L_CALLS: AtomicU64 = AtomicU64::new(0);   // install_launcher_hook 호출수
static HK_L_OURS: AtomicU64 = AtomicU64::new(0);    // 진입부=내 스텁 확인 후 즉시 return(정상 경로)
static HK_L_WAIT: AtomicU64 = AtomicU64::new(0);    // serpen 대기 중 return
static HK_L_INSTALL: AtomicU64 = AtomicU64::new(0); // ★실제 install_detour_generic 진입
static HK_L_B0: AtomicU64 = AtomicU64::new(0);      // 마지막 관측 진입부 첫 바이트
static HK_L_TGT: AtomicU64 = AtomicU64::new(0);     // 마지막 관측 movabs 타깃
static HK_S_INSTALL: AtomicU64 = AtomicU64::new(0); // seed-ctor 실제 install 진입
static HK_L_TICK: AtomicU64 = AtomicU64::new(0);
static HK_L_SKIP: AtomicU64 = AtomicU64::new(0);   // 스로틀로 건너뛴 프레임
fn install_launcher_hook() {
    HK_L_CALLS.fetch_add(1, Ordering::Relaxed);
    // ★비용 최적화(2026-07-22 perf 계측 — 이 함수가 매 프레임 **최소 106µs** = 메인스레드 실비용
    //   최대 항목이었다. 평균이 아니라 최소가 106µs라 선점 노이즈가 아닌 실제 작업):
    //   ① `GetModuleHandleW`(로더 락) 매 프레임 직접 호출 → **캐시된 `exe_base_addr()`**
    //      (다른 경로는 전부 캐시본을 쓰는데 여기만 원시 API를 부르고 있었다)
    //   ② `readable()` = **VirtualQuery**(주소공간 락) 제거 → 진입부 읽기를 VEH 보호 `safe_read_u64`로.
    //      설치 시 이미 검증된 주소이고, 폴트나면 VEH가 잡으므로 이중 검증이 불필요했다.
    //   ③ 설치 완료 후의 재검증(=타 모드가 우리 훅을 덮었는지 self-heal)은 매 프레임일 이유가 없어
    //      **60프레임(≈1초) 주기**로. 덮이더라도 1초 내 자가복구면 충분(경기 시작 시점 이벤트라 여유 있음).
    if CLAUNCH_INSTALLED.load(Ordering::Relaxed) == 1 {
        if HK_L_TICK.fetch_add(1, Ordering::Relaxed) % 60 != 0 {
            HK_L_SKIP.fetch_add(1, Ordering::Relaxed);
            return;
        }
    }
    // ★serpen 공존(체인후킹): serpen이 launcher 0x20588a0을 먼저 후킹(진입부=movabs+jmp)하면 그 뒤로 체인.
    //   ⚠먼저 설치하면 serpen이 우릴 덮어 고아됨 → serpen(=외부 movabs 진입부) 나타날 때까지 대기(최대 240프레임),
    //     그 후에도 원본프롤로그면 serpen 부재로 보고 단독 설치. 매프레임 재검증(진입부≠우리스텁이면 재체인 → serpen이 나중에 덮어도 자가복구).
    let base = exe_base_addr(); // ★캐시본(구: GetModuleHandleW 매 프레임)
    if base == 0 { return; }
    let fn_addr = base + CL_LAUNCHER_RVA;
    // ★VirtualQuery 없이 VEH 보호 읽기로 진입부 확인(구: readable() 매 프레임).
    let Some(w0) = (unsafe { safe_read_u64(fn_addr) }) else { return; };
    let b0 = (w0 & 0xff) as u8;
    let b1 = ((w0 >> 8) & 0xff) as u8;
    let cur_tgt: usize = if b0 == 0x48 && b1 == 0xb8 {
        match unsafe { safe_read_u64(fn_addr + 2) } { Some(t) => t as usize, None => return } // movabs imm64 = fn+2..+10
    } else { 0 };
    let our = CLAUNCH_STUB.load(Ordering::Relaxed) as usize;
    HK_L_B0.store(b0 as u64, Ordering::Relaxed); HK_L_TGT.store(cur_tgt as u64, Ordering::Relaxed);
    if our != 0 && cur_tgt == our { CLAUNCH_INSTALLED.store(1, Ordering::Relaxed); HK_L_OURS.fetch_add(1, Ordering::Relaxed); return; } // 진입부=우리 스텁 → 정상
    let is_foreign = b0 == 0x48 && cur_tgt >= 0x10000 && cur_tgt != our; // serpen 등 외부 훅 존재
    let waited = LAUNCH_WAIT.fetch_add(1, Ordering::Relaxed);
    if !is_foreign && b0 != 0x48 && waited < 240 { HK_L_WAIT.fetch_add(1, Ordering::Relaxed); return; } // 원본프롤로그 & 대기중 → serpen 설치 기다림
    // 설치(또는 재체인). install_detour_generic이 외부훅 감지 시 자동 체인.
    HK_L_INSTALL.fetch_add(1, Ordering::Relaxed);
    let r = unsafe { install_detour_generic(CL_LAUNCHER_RVA, 12, cap_launcher as usize, &CL_LAUNCHER_PROLOGUE) };
    match r {
        Ok(stub) => { CLAUNCH_STUB.store(stub as u64, Ordering::Relaxed); CLAUNCH_INSTALLED.store(1, Ordering::Relaxed); }
        Err(e) => { CLAUNCH_INSTALLED.store(2, Ordering::Relaxed);
            if LAUNCH_ERR_N.fetch_add(1, Ordering::Relaxed) < 3 { write_log("4items_hooks.txt", &format!("[{}ms] launcher install FAIL: {}\n", now_ms(), e)); } }
    }
}

// ═══ seed-ctor 훅(0.5.1 RE) — 렌더 sim의 provider 포인터 결정적 캡처 (시드 값 대조 불가 → 포인터 아이덴티티) ═══
//   ghidra-re 확정: FUN_1421d03e0(rcx=provider(this), rdx=seed(=launcher r8, 무변환 비트동일))이 seed를 provider+0xeab8에 저장.
//   그러나 +0xeab8은 매 난수소비마다 갱신되는 RNG 러닝-스테이트 → buy 시점 값 대조는 원리적 불가.
//   대안: ctor 진입 시 rdx==LIVE_SEED(launcher가 잡은 렌더 초기시드)면 그 provider(rcx)를 RENDER_PROVIDER로 기록.
//   buy 시점: provider==RENDER_PROVIDER → 렌더 sim 확정(주소 대조, 가변필드 무관).
//   ⚠**레거시 주석 정정(0.5.3, 07-29)**: 현행 buy 게이트는 `*(game_p6+0x1dc0)`이 아니라 **r9(arg4)=provider**를 쓴다
//     (아래 buy 훅 `*saved.add(3)` 참조 — [rsp+0x30]은 buy-list 컨테이너지 provider가 아니었다는 구 RE 결론).
//     `Game+0x1dc0`을 읽는 활성 코드는 cap_spawn뿐이고 그건 게이트 OFF. Game+0x1dc0/+0x1dc8 자체는 0.5.3에서 **유지 확인**
//     (launcher 0xeb9646 `mov [rsi+0x1dc0],rax; mov [rsi+0x1dc8],rax` + vtable 슬롯 +0x20이 `mov rax,[rcx+0xeaf8]` = seed 오프셋과 정합).
//   ⬜**미검증**: "r9=provider"는 buy 본문이 arg4를 즉시 덮어써 정적 확인 불가 — 0.5.2 때처럼 **인게임 seed 매칭으로만 확정**된다.
//     0.5.3에서 어긋나면 크래시가 아니라 '관전 경기 미인식'으로 조용히 나타난다(is_live 히트 카운터로 판별).
//   launcher가 ctor를 동기 호출 → LIVE_SEED 선세팅 보장. 배경 sim은 렌더시드 아님 → 무매칭(오염 배제).
const SEEDCTOR_RVA: usize = 0x1635ae0; // 0.5.6(구0.5.5=0x14c2380). head-UNIQUE·seed 스토어 [rsi+0xec90]@0x10a4b0d 실측(provider 0xec90 불변)·프롤로그12 동일(chkstk imm 0x25438→0x256d8은 배열 밖). // 0.5.5(구0.5.4=0x14e16d0, 마스크시그 확정). seed 스토어 provider+0xeb28→+0xec90. // 0.5.4(구0.5.3=0x12b9ab0). 프롤로그 8push 동일·콜러 4개 동수·본문 sim 0.90·seed 스토어 1:1 대응. // 0.5.3(구0.5.2=0x22c1da0). 프롤로그 12B 완전동일(8push)·chkstk 프레임 0x11b58→0x11b98·launcher(0xeb8810) 내부 콜에서 rdx=저장된 r8(seed) 확인. ⚠seed 저장 오프셋은 provider+0xeab8→**+0xeaf8**로 이동(0x12ba92d 실측).
// ★0.5.3: provider 구조체에서 seed 저장 오프셋이 이동했다(0.5.2 +0xeab8 → 0.5.3 +0xeaf8).
//   실측 = seedctor 내부 `mov [reg+0xeaf8], rdx` @0x12ba92d (구 exe는 같은 자리에 0xeab8).
//   ⚠단일 상수로 묶어둔다 — 패치마다 여기만 갱신하면 is_live 게이트 전체가 따라온다.
const O_PROVIDER_SEED: usize = 0xec90; // 0.5.5(구0.5.4=0xeb28, +0x168 이동). seedctor 0x14c2380 fn+0xed0 mov[rsi+0xec90],rax + 게터썽크 0x14f7de0 mov rax,[rcx+0xec90];ret 유일히트로 확정.
const SEEDCTOR_PROLOGUE: [u8; 12] = [0x55, 0x41,0x57, 0x41,0x56, 0x41,0x55, 0x41,0x54, 0x56, 0x57, 0x53]; // ghidra-re 확정: 8push(12B)+mov eax,0x11b58+call chkstk (launcher 동일패턴)
const SEEDCTOR_ORIG_LEN: usize = 12; // 8push만 재배치(chkstk call 제외). jmp가 fn+12=mov eax에 착지→프레임 정상세팅
static SEEDCTOR_INSTALLED: AtomicU64 = AtomicU64::new(0);
static RENDER_PROVIDER: AtomicU64 = AtomicU64::new(0); // ★렌더 sim provider 포인터(is_live 주력 게이트)
// ★★08-07 신설 — "클라가 쥔 핸들을 우리도 쥔다" 검증용 3종.
static LIVE_GAME: AtomicU64 = AtomicU64::new(0);       // launcher rcx = Game (화면 경기)
static BUY_GAME: AtomicU64 = AtomicU64::new(0);        // buy 시점 Game([rsp_entry+0x30]) — 위와 같으면 유효
static GAME_PROBE_DONE: AtomicBool = AtomicBool::new(false);
static GAME_SCAN_DONE: AtomicBool = AtomicBool::new(false); // 디투어 내 1회 스캔 게이트
static GAME_SCAN_OK: AtomicU64 = AtomicU64::new(0);         // 0=미실행 1=스캔함 2=readable 실패
static GAME_HIT_N: AtomicU64 = AtomicU64::new(0);
static GAME_HIT: [AtomicU64; 4] = [const { AtomicU64::new(u64::MAX) }; 4]; // 최상위비트=u32쌍 표시
static GATE_AGREE: AtomicU64 = AtomicU64::new(0);      // scene side 판정 == is_my_athlete 판정
static GATE_DIFF: AtomicU64 = AtomicU64::new(0);       // ★갈린 횟수(0이어야 전환이 안전)
static GATE_SAMPLE: AtomicU64 = AtomicU64::new(0);     // 교차검증 샘플러(1/256)
// ★★2026-08-07 "경우의 수 닫기" — 제보자 환경 재현 불가 대응. 아래 4구멍 + 자가진단.
//   전제 정정: 08-06 에 고친 600틱 창은 `roster_trust` 의 `known != 0` 때문에 **pid≠0 세이브엔 존재하지 않는다**
//   ⟹ 그건 pid=0 세이브(개발자 본인) 전용 결함이었고 **제보자 원인은 따로 있다**.
static PID_SRC: AtomicU64 = AtomicU64::new(0);     // 0=미확보 1=InGame 2=server_state(관리틱)
static MYSIDE_TAB: [(AtomicU64, AtomicU64); 8] = [const { (AtomicU64::new(0), AtomicU64::new(u64::MAX)) }; 8];
static MYSIDE_HIT: AtomicU64 = AtomicU64::new(0);  // side 전파로 구제된 buy(=교체선수 등)
static GATE_SCENE: AtomicU64 = AtomicU64::new(0);  // 게이트 통과 경로 분포
static GATE_ROSTER: AtomicU64 = AtomicU64::new(0);
static GATE_NONE: AtomicU64 = AtomicU64::new(0);   // ★결함 지표: 지정챔프인데 **판정 불가(None)** 로 막힘
static GATE_BLOCK_OK: AtomicU64 = AtomicU64::new(0); // 정상 차단: 확정 타팀(적 지정챔프) — 결함 아님
static MODITEMS_TRIES: AtomicU64 = AtomicU64::new(0);
static MODITEMS_FAIL_WHY: AtomicU64 = AtomicU64::new(0); // 1=비바닐라 배열 못찾음 2=next_tier 없음
static SCAN_NEG_PURGE: AtomicU64 = AtomicU64::new(0);
// ★구멍2 — `last_starting` 에 없는 선수(교체·부상 출전)는 멤버십에서 빠져 영구 미주입된다.
//   해결: **그 매치 로스터에 내 선발이 한 명이라도 있으면 그 side 전체를 내 팀으로 인정**한다.
//   휴리스틱(지정챔프 다수결)이 아니라 athlete_id 멤버십 기반이라 결정적이다.
//   캐시는 provider 키의 lock-free 8칸 — rayon 병렬 디투어에서 Mutex 경합을 피한다.
// ⛔★★2026-08-07 실측으로 **기본 OFF 확정**(재활성 전 아래 경위 필독).
//   켜고 한 판 돌린 결과: `side전파 구제 = 1,676,944` > 멤버십 통과 `1,675,164`
//   = **배경 sim 의 거의 모든 buy 를 통과**시켰다 = 팀 게이트가 사실상 열렸다.
//   원인: 이 세이브의 `MY_ATHLETES = {0,1,2,3,4}` 인데 배경 sim 은 **athlete_id 미기입(0)** 이라
//   그 집합에 걸린다 ⟹ 아무 배경 매치에서나 "내 선발 발견" 판정 → 그 side 전체를 내 팀으로 인정.
//   07-30 에 잡았던 배경오염을 side 단위로 증폭시킨 꼴이다.
//   ⟹ **교체선수 구제라는 목적 자체는 유효하나, aid 기반 발견만으로는 판별력이 없다.**
//     재활성하려면 최소한 ①동일 side 에서 **서로 다른** my-aid 2개 이상 ②aid=0 은 근거에서 제외
//     ③배경오염 지표(비-내선수 주입)를 함께 계측 — 셋 다 충족해야 한다.
const SIDE_PROPAGATE: bool = false;
unsafe fn my_side_in_match(provider: u64, athlete: usize) -> Option<u64> {
    if !SIDE_PROPAGATE { return None; }
    if MY_ATH_N.load(Ordering::Relaxed) == 0 { return None; }
    let slot = (provider >> 4) as usize & 7;
    let (pk, pv) = (&MYSIDE_TAB[slot].0, &MYSIDE_TAB[slot].1);
    if pk.load(Ordering::Relaxed) == provider {
        let v = pv.load(Ordering::Relaxed);
        return if v <= 1 { Some(v) } else { None };
    }
    let mut found = u64::MAX;
    for k in -9i64..=9 {
        let a = athlete.wrapping_add((k.wrapping_mul(ATH_STRIDE as i64)) as usize);
        if matches!(is_my_athlete(a), Some(true)) {
            if let Some(s) = safe_read_u64(a + 0x930) { if s <= 1 { found = s; break; } }
        }
    }
    pv.store(found, Ordering::Relaxed);
    pk.store(provider, Ordering::Relaxed);
    if found <= 1 { Some(found) } else { None }
}
// (팀 id 탐색 키 `SCENE_T1`/`SCENE_T2` 는 아래쪽 기존 선언 재사용 — 원래 writer 가 없어 死 상태였던 것을
//  `quick_scene_side` 에서 채우도록 되살렸다.)
static LIVE_SEED: AtomicU64 = AtomicU64::new(0);      // ★내 경기 시드(launcher 훅 r8 캡처). v13 값대조 키.
static PROV_HIT: AtomicU64 = AtomicU64::new(0);       // is_live(v13 provider/seed 매칭) 발화수
static VT_OK: AtomicU64 = AtomicU64::new(0);          // 그중 seed 값대조 발화수
static INGAME_NOW: AtomicBool = AtomicBool::new(false); // post_update가 세팅하는 "지금 관전 화면" 플래그
static BUY_WROTE_FIRE: AtomicU64 = AtomicU64::new(0); // 실제 build[si] write 성공수
static SEEDCTOR_N: AtomicU64 = AtomicU64::new(0);      // ctor 총 발화수
static SEEDCTOR_MATCH_N: AtomicU64 = AtomicU64::new(0);// rdx==LIVE_SEED 적중(렌더 provider 캡처)수
unsafe extern "C" fn cap_seed_ctor(saved: *mut u64, _e: usize) -> u64 {
    let __st = perf::tsc();
    if saved.is_null() { perf::rec(perf::S_SEEDCTOR, __st); return 0; }
    let provider = *saved;         // saved+0 = rcx = arg1 = provider(this)
    let seed = *saved.add(1);      // saved+1 = rdx = arg2 = seed(=launcher r8)
    SEEDCTOR_N.fetch_add(1, Ordering::Relaxed);
    let ls = LIVE_SEED.load(Ordering::Relaxed);
    if ls != 0 && seed == ls && provider >= 0x10000 && provider < 0x0000_8000_0000_0000 {
        RENDER_PROVIDER.store(provider as u64, Ordering::Relaxed);
        SEEDCTOR_MATCH_N.fetch_add(1, Ordering::Relaxed);
    }
    perf::rec(perf::S_SEEDCTOR, __st);
    0
}
fn install_seed_ctor_hook() {
    if SEEDCTOR_INSTALLED.load(Ordering::Relaxed) == 1 { return; } // ★1=성공만 skip(0/2=재시도)
    HK_S_INSTALL.fetch_add(1, Ordering::Relaxed);
    let r = unsafe { install_detour_generic(SEEDCTOR_RVA, SEEDCTOR_ORIG_LEN, cap_seed_ctor as usize, &SEEDCTOR_PROLOGUE) };
    SEEDCTOR_INSTALLED.store(if r.is_ok() { 1 } else { 2 }, Ordering::Relaxed);
}


// ★★역검색 진단: buy athlete(관전 로스터 원소 확정)에서 db view 오프셋을 런타임 역산. 휴리스틱 아님=결정적.
// ★★스레드 정체성 게이트 검증(07-11 RE 1순위): 관전(재시뮬)=메인스레드, 배경 sim=rayon 워커 가설.
//   post_update(메인스레드) tid vs buy 훅(sim 스레드) tid 비교 → 갈리면 오프셋 없이 관전 판정 가능.
static CP_INSTALLED: AtomicU64 = AtomicU64::new(0);
const CP_PROLOGUE: [u8; 12] = [0x55, 0x41,0x57, 0x41,0x56, 0x41,0x55, 0x41,0x54, 0x56, 0x57, 0x53];

// game String 양쪽 레이아웃 시도: {len,ptr,cap} 또는 {ptr,len,cap}. ASCII 키/이름이면 반환.
unsafe fn read_str_try(addr: usize) -> Option<String> {
    if !readable(addr, 24) { return None; }
    let q0 = safe_read_u64(addr)? as usize;
    let q8 = safe_read_u64(addr + 8)? as usize;
    for &(ptr, len) in &[(q8, q0), (q0, q8)] { // (len,ptr)=len@0,ptr@8 / (ptr,len)=ptr@0,len@8
        if ptr <= 0x10000 || ptr >= (1usize << 48) || len < 2 || len > 48 { continue; }
        let mut b = Vec::new();
        if !safe_read_bytes(ptr, len, &mut b) { continue; }
        if b.iter().all(|&x| x == b'_' || x.is_ascii_alphanumeric()) && (b[0] as char).is_ascii_alphabetic() {
            return String::from_utf8(b).ok();
        }
    }
    None
}
// push 프로브서 확인된 아이템 키 판별 (표시 items Vec 식별용).
fn is_known_item_key(k: &str) -> bool {
    const ITEMS: [&str; 20] = ["dagger","ironsword","vital_orb","arcane_crystal","steel_armor","mystic_cloak",
        "soldiers_longsword","wind_dagger","spirit_crystal","hardened_heart","nashors_tooth","ring_of_reincarnation",
        "ruinous_blade","souls_edge","dusk_raven","staff_of_rapture","twin_stormblade","angels_fang","thunderclaw","spirit_visage"];
    ITEMS.contains(&k) || k.starts_with("radiant_") || k.contains("_blade") || k.contains("sword")
        || k.contains("_armor") || k.contains("_plate")
}
// champion String 위치로 로스터 원소 검증. ★0.5.4: champ String = cap +0x408 / **ptr +0x410** / len +0x418
//   (ath_champ_name 과 정합). ~~0.5.0_3~~0.5.3까지는 ptr +0x420 / len +0x428 이었다(athlete −0x10 시프트).
//   ⚠구 +0x388~0x3b0 유산 오프셋만 보면 0.5.0 athlete를 인식 못해 find_view_by_scan 실패→LIVE_ARR=0(팀게이트 붕괴).
//   ⚠★**현재 호출부 0개(dead code)**. `#![allow(dead_code)]` 때문에 컴파일 경고도 안 뜬다 —
//     그래서 0.5.4 마이그(2026-08-05)에서 이 한 줄만 구 오프셋(+0x420)으로 남아 있었다.
//     되살릴 때 오프셋을 반드시 재확인할 것. 살아 있었다면 전 athlete 에서 false → 팀게이트 붕괴였다.
unsafe fn valid_ps_elem(elem: usize) -> bool {
    if read_str_try(elem + 0x470).is_some() { return true; } // 0.5.4 champ String ptr 정위치
    let mut o = 0x388usize; // 폴백(구버전/레이아웃 변형 대비)
    while o <= 0x3b0 { if read_str_try(elem + o).is_some() { return true; } o += 8; }
    false
}
static CAP_MATCH_DONE: AtomicBool = AtomicBool::new(false);
static CAP_MPID: AtomicU64 = AtomicU64::new(0);
static CAP_MTID: AtomicU64 = AtomicU64::new(0);
static INJ_LOG: Mutex<Vec<(Vec<u8>, u8, u64)>> = Mutex::new(Vec::new());

unsafe fn install_detour_generic(rva: usize, orig_len: usize, cap_fn: usize, prologue: &[u8]) -> Result<usize, &'static str> {
    let base = GetModuleHandleW(core::ptr::null()) as usize;
    if base == 0 { return Err("module 0"); }
    let fn_addr = base + rva;
    if !readable(fn_addr, orig_len + 4) { return Err("unreadable"); }
    // ★체인후킹: 진입부가 이미 외부 모드 훅(movabs rax,tgt; jmp rax = 48 b8 .. ff e0)이면, 원본 대신 그 외부 스텁으로 체인.
    //   serpen 등이 같은 함수(예 launcher 0x20588a0)를 먼저 후킹한 경우 프롤로그가 덮여있으므로 프롤로그검증 skip.
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    let foreign_tgt: usize = if cur[0] == 0x48 && cur[1] == 0xb8 && cur[10] == 0xff && cur[11] == 0xe0 {
        usize::from_le_bytes(cur[2..10].try_into().unwrap())
    } else { 0 };
    let chained = foreign_tgt >= 0x10000;
    // 프롤로그 검증(RVA 어긋남 방지) — 체인(외부훅 존재) 시엔 원본프롤로그가 덮여 있으므로 skip.
    if !chained {
        for i in 0..prologue.len() { if *((fn_addr + i) as *const u8) != prologue[i] { return Err("prologue mismatch"); } }
    }
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    // ⚠ entry_rsp 캡처(mov r10,rsp) 금지 — 후킹 원본명령이 r10을 저장하므로 r10 보존 필수.
    //   cap_fn 2번째 인자(entry_rsp)는 미사용 → rdx 안 건드림(원본 rdx 그대로, cap_fn 무시).
    // push r12 rsi rdi rbx r11 r10 r9 r8 rdx rcx  (rcx 마지막=saved+0; r12=saved+0x48; r10/r9 원본 보존)
    //   ★ r12 추가 = cap_fn 이 r12(personal_tactics 매치 entry)에 접근(watchpoint 무장용).
    s.extend_from_slice(&[0x41, 0x54, 0x56, 0x57, 0x53, 0x41, 0x53, 0x41, 0x52, 0x41, 0x51, 0x41, 0x50, 0x52, 0x51]);
    s.extend_from_slice(&[0x48, 0x89, 0xe1]);             // mov rcx, rsp (saved=arg1)
    s.extend_from_slice(&[0x48, 0x89, 0xe3]);             // mov rbx, rsp (정렬복원 홀더, cap_fn 보존)
    s.extend_from_slice(&[0x48, 0x83, 0xe4, 0xf0]);       // and rsp, -16 (mid-func 16정렬 보정)
    s.extend_from_slice(&[0x48, 0x83, 0xec, 0x20]);       // sub rsp, 0x20 (shadow)
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes()); // movabs rax, cap_fn
    s.extend_from_slice(&[0xff, 0xd0]);                   // call rax
    s.extend_from_slice(&[0x48, 0x89, 0xdc]);             // mov rsp, rbx (정렬복원)
    // pop rcx rdx r8 r9 r10 r11 rbx rdi rsi r12  (push 역순)
    s.extend_from_slice(&[0x59, 0x5a, 0x41, 0x58, 0x41, 0x59, 0x41, 0x5a, 0x41, 0x5b, 0x5b, 0x5f, 0x5e, 0x41, 0x5c]);
    if chained {
        // ★체인: 원본프롤로그 실행 안 함 → 외부 모드 스텁(cur=movabs rax,foreign_tgt; jmp rax)으로 점프.
        //   외부 스텁이 자기 캡처+원본프롤로그+fn+0xc 복귀를 처리. rax 클로버는 원본 mov eax가 재설정(무해).
        s.extend_from_slice(&cur); // = 48 b8 <foreign_tgt> ff e0
    } else {
        let mut orig = vec![0u8; orig_len];              // 원본 명령(PI) 복사 실행
        core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
        s.extend_from_slice(&orig);
        s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes()); // movabs rax, ret_addr
        s.extend_from_slice(&[0xff, 0xe0]);               // jmp rax
    }
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    // 패치: movabs rax, stub; jmp rax (12B) + NOP 패딩
    let mut patch = vec![0x90u8; orig_len];
    patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(stub)
}


// ===========================================================================
//  SDK 라이프사이클
// ===========================================================================
struct ItemTacticsExt;
// ═══ buy 리포트 리셋 (새 관전 경기 진입 시) — buy_report.txt 엔 "마지막 본 경기"만 남김 ═══
fn buy_report_reset() {
    for a in [&BR_ENTER, &BR_NULLSAVED, &BR_BADATH, &BR_TOTAL, &BR_LIVE, &BR_DES, &BR_DES_LIVE, &BR_ISPLAYER, &BR_IDX_OK, &BR_IDX_NONE, &BR_WROTE].iter() {
        a.store(0, Ordering::Relaxed);
    }
    BR_LOG.lock().unwrap_or_else(|e| e.into_inner()).clear();
    BR_SEEN.lock().unwrap_or_else(|e| e.into_inner()).clear();
}

// ═══ buy 리포트 파일 flush (30프레임마다) — mods\tfm2_item_tactics\buy_report.txt ═══
fn buy_report_flush() {
    if !BUY_REPORT { return; }
    static BR_FRAME: AtomicU64 = AtomicU64::new(0);
    if BR_FRAME.fetch_add(1, Ordering::Relaxed) % 30 != 0 { return; }
    let ld = |a: &AtomicU64| a.load(Ordering::Relaxed);
    let mut s = String::new();
    s.push_str("=== tfm2_item_tactics — buy 아이템 주입 리포트 (마지막 관전 경기 기준) ===\n");
    s.push_str(&format!("생성(ms): {}\n\n", now_ms()));
    s.push_str("[단계별 집계] (위→아래 깔때기 — 어디서 수가 0/급감하는지가 원인)\n");
    s.push_str(&format!("  설치상태 BUY_PROBE_INSTALLED = {}   (0=미시도 / 1=직접설치OK / 2=설치실패 / 3=체인설치OK(외부훅 위))\n",
        BUY_PROBE_INSTALLED.load(Ordering::Relaxed)));
    s.push_str(&format!("  0. detour 진입(무조건)  : {}   =0 이면 훅 자체가 미발화(설치/경로 축) — 0.5.6 진단\n", ld(&BR_ENTER)));
    s.push_str(&format!("  0a. saved null 탈출     : {}\n", ld(&BR_NULLSAVED)));
    s.push_str(&format!("  0b. r8 athlete 쓰레기   : {}   >0 이면 buy 인자 계약(r8=athlete) 붕괴\n", ld(&BR_BADATH)));
    s.push_str(&format!("  1. 전체 buy 콜         : {}\n", ld(&BR_TOTAL)));
    s.push_str(&format!("  2. is_live(관전 경기)  : {}   =0 이면 관전 라이브 경기를 못 찾음(스폰훅 미발화)\n", ld(&BR_LIVE)));
    s.push_str(&format!("  3. 지정챔프 buy        : {}   =0 이면 SEL 지정 챔프가 이 경기 로스터에 없음\n", ld(&BR_DES)));
    s.push_str(&format!("  4. 지정 && is_live     : {}\n", ld(&BR_DES_LIVE)));
    s.push_str(&format!("  5. is_player(내 팀 확정): {}   =0 이면 팀/side 판정 실패(적·배경만 잡힘)\n", ld(&BR_ISPLAYER)));
    s.push_str(&format!("  6. 슬롯 목표 idx 구함   : {}\n", ld(&BR_IDX_OK)));
    s.push_str(&format!("  7. 슬롯 목표 idx 실패   : {}   >0 이면 모드템 카탈로그 스캔 실패(그 아이템이 이 환경에 없음)\n", ld(&BR_IDX_NONE)));
    s.push_str(&format!("  8. 실제 build write    : {}   <= 최종 주입 성공수. 0이면 위 2~7 중 하나에서 막힌 것\n", ld(&BR_WROTE)));

    // ── 관전 식별 진단 (v13 provider/seed 게이트 — lean) ──
    s.push_str("\n[관전 식별 진단]  launcher 시드 캡처 + seed-ctor provider 캡처 + r9 대조\n");
    s.push_str(&format!("  ★launcher: 발화={} 렌더판정={} 렌더RA={:#x} LIVE_SEED={:#x} 설치={}\n",
        ld(&LAUNCH_N), ld(&LAUNCH_RENDER_N), ld(&LAUNCH_RENDER_RA), ld(&LIVE_SEED), ld(&CLAUNCH_INSTALLED)));
    { // ★조합테스트 등 비-렌더 경로 규명: launcher 고유 콜러 rva 목록
        s.push_str("  ★launcher 고유 콜러 rva:");
        for k in 0..24 { let v = LAUNCH_RVAS[k].load(Ordering::Relaxed); if v == 0 { break; } s.push_str(&format!(" {:#x}", v)); }
        s.push_str("   (렌더필터=[0x722ca0,0x740000) — 여기 안 드는 콜러가 조합테스트 경로 후보)
");
    }
    s.push_str(&format!("  ★v13 provider매칭(r9): seed-ctor발화={} 렌더provider캡처={} RENDER_PROVIDER={:#x} | is_live={} seed값대조={} 설치={}\n",
        ld(&SEEDCTOR_N), ld(&SEEDCTOR_MATCH_N), ld(&RENDER_PROVIDER), ld(&PROV_HIT), ld(&VT_OK), ld(&SEEDCTOR_INSTALLED)));
    { // ★UI 로더 경로 실측 — training.ui가 후킹한 copy를 타는지 확정
        s.push_str(&format!("
[UI 로더 프로브] 총 로드콜={} · 'training' 포함 경로 관측={}
",
            uinj::LOADER_CALLS.load(Ordering::Relaxed), uinj::TRAIN_SEEN.load(Ordering::Relaxed)));
        let g = uinj::SEEN_PATHS.lock().unwrap_or_else(|e| e.into_inner());
        s.push_str(&format!("  관측된 distinct 경로 {}개:
", g.len()));
        for p in g.iter().take(60) { s.push_str(&format!("    {}
", p)); }
        s.push_str(&format!("  ★조합테스트 배선: 핸들러호출={} builds발견={} visible={} blue0발견={} 챔프해석={} 옵션주입={}
",
            ld(&CTD_CALL), ld(&CTD_BUILDS), ld(&CTD_VIS), ld(&CTD_ROW), ld(&CTD_CHAMP), ld(&CTD_SET)));
        s.push_str(&format!("  ★단계: 분기진입={} mode4={} 동일r스킵={} 멱등skip={} 행찾음={} 교체성공={}
",
            uinj::TR_BRANCH.load(Ordering::Relaxed), uinj::TR_MODE4.load(Ordering::Relaxed),
            uinj::TR_SKIP_SAME.load(Ordering::Relaxed), uinj::TR_IDEM.load(Ordering::Relaxed),
            uinj::TR_ROWS.load(Ordering::Relaxed), uinj::TR_REPL.load(Ordering::Relaxed)));
        if uinj::TRAIN_SEEN.load(Ordering::Relaxed) == 0 {
            s.push_str("  ⚠training 경로 미관측 = 제3의 로더 copy를 탐 → 그 RVA 훅 추가 필요
");
        }
    }
    { // ★★지정템 도달 판정(누적) — 스냅샷이 아니라 "실제 보유한 적 있는가". 미도달만 실패로 본다.
        let want = REACH_WANT.lock().unwrap_or_else(|e| e.into_inner()).clone();
        let hit = REACH_HIT.lock().unwrap_or_else(|e| e.into_inner()).clone();
        s.push_str(&format!("\n[지정템 도달 판정]  도달 {}/{} (누적·경기후반 조합 포함)\n", hit.len(), want.len()));
        for w in want.iter() {
            s.push_str(&format!("  {} {}\n", if hit.iter().any(|h| h == w) { "✅도달" } else { "❌미도달" }, w));
        }
        if want.is_empty() { s.push_str("  (모드템 지정 없음 — 바닐라 지정만 있거나 지정챔프가 이 경기에 없음)\n"); }
    }
    { let p = ld(&PLAYER_TEAM_ID); s.push_str(&format!("  player_team_id        : {} (유효경험={})\n",
        if p == u64::MAX { "미캡처".to_string() } else { p.to_string() }, ld(&PID_EVER_VALID))); }
    { let sc = ld(&SCENE_SIDE); s.push_str(&format!("  SCENE_SIDE            : {}   (미판정=관전경기 team_id↔내팀 매칭 실패 또는 관전아님)\n",
        if sc > 1 { "미판정".to_string() } else { sc.to_string() })); }
    { let lp = ld(&LIVE_PID); s.push_str(&format!("  LIVE_DB / LIVE_PID    : {:#x} / {}\n",
        ld(&LIVE_DB), if lp == u64::MAX { "미캡처".to_string() } else { lp.to_string() })); }

    // ── ★조합테스트 스코프 + 배경오염 차단 검증 (2026-07-30 2차 수정) ──────────────
    s.push_str("\n[★조합테스트 스코프 / 배경오염 차단]  (누적 — 경기별 리셋 안 함)\n");
    s.push_str(&format!("  COMPTEST_MATCH(현재)      : {}\n", COMPTEST_MATCH.load(Ordering::Relaxed)));
    s.push_str(&format!("  조합테스트 우회 발동      : {}   (= COMPTEST_MATCH && is_live)\n", ld(&BR_CT_LIVE)));
    s.push_str(&format!("  ★차단된 sticky buy        : {}   >0 = 수정 전이라면 **배경경기에 주입됐을** 호출 수\n", ld(&BR_CT_STICKY)));
    s.push_str(&format!("  ★★배경오염(비-내선수)     : {}   ★반드시 0 — >0 이면 남의 팀 선수에 주입 중(결함)\n", ld(&BR_BG_PLAYER)));
    s.push_str(&format!("  배경 buy·내 선수(정상)    : {}   FIXB 의도된 동작(관전==확정 수렴) — 0 이 아니어도 정상\n", ld(&BR_BG_MINE)));
    s.push_str(&format!("  진영 판정 분포            : 블루={} 레드={} 판정실패(Plain폴백)={}\n",
        ld(&BR_SCOPE_B), ld(&BR_SCOPE_R), ld(&BR_SCOPE_NA)));
    { let (b, r) = (CT_SIDE_B.load(Ordering::Relaxed), CT_SIDE_R.load(Ordering::Relaxed));
      let f = |v: u64| if v == u64::MAX { "미학습".to_string() } else { v.to_string() };
      s.push_str(&format!("  side 값 학습 (athlete+0x820): 블루={} 레드={}\n", f(b), f(r))); }
    { let p = CT_ROSTER.load(Ordering::Acquire);
      if p.is_null() { s.push_str("  CT_ROSTER: 미게시 (조합테스트 개인전술 탭을 아직 안 열었음 → 진영 판정 불가 = Plain 폴백)\n"); }
      else { let (bl, rd) = unsafe { &*p };
        let mut b: Vec<&String> = bl.iter().collect(); b.sort();
        let mut r: Vec<&String> = rd.iter().collect(); r.sort();
        s.push_str(&format!("  CT_ROSTER 블루({}): {}\n", b.len(), b.iter().map(|x| x.as_str()).collect::<Vec<_>>().join(" ")));
        s.push_str(&format!("  CT_ROSTER 레드({}): {}\n", r.len(), r.iter().map(|x| x.as_str()).collect::<Vec<_>>().join(" ")));
      } }
    { // SEL 안의 스코프 키 현황 = 진영별 지정이 실제로 저장됐는지
      let (mut nb, mut nr, mut na, mut np) = (0usize, 0usize, 0usize, 0usize);
      with_sel(|m| for ((c, _), &v) in m.iter() {
          if v == SEL_AUTO { na += 1; }
          if c.starts_with(CT_PFX_B) { nb += 1; } else if c.starts_with(CT_PFX_R) { nr += 1; } else { np += 1; }
      });
      s.push_str(&format!("  SEL 키 현황: 일반={} @b:={} @r:={} (그중 명시auto={})   @b/@r 이 0 이면 조합테스트 드롭다운을 만진 적 없음\n", np, nb, nr, na));
    }

    // ── ★내 팀 판정 근거 실값 (2026-07-30: "왜 이 선수가 내 팀인가" 판별용) ──────────
    s.push_str("\n[★내 팀 판정 근거]  is_player(FIXB) = athlete+0x810 ∈ MY_ATHLETES\n");
    { let p = PLAYER_TEAM_ID.load(Ordering::Relaxed);
      s.push_str(&format!("  player_team_id(게시)   : {}   (비0 유효 pid 관측={})\n",
          if p == u64::MAX { "미캡처".to_string() } else { p.to_string() },
          PID_NONZERO_SEEN.load(Ordering::Relaxed)));
      s.push_str(&format!("  대상팀 PT 엔트리 수     : {}   (내 팀은 유저 설정으로 수십 개·AI 팀은 몇 개뿐 = pid=0 검증 근거)\n", ld(&MY_PT_N)));
      s.push_str(&format!("  pid 관측: 0={}회 비0={}회 | 조합테스트컨텍스트 0 무시={}회 | 무관컨텍스트 0={}회(≥600이면 팀0 인정)\n",
          ld(&PID_OBS_ZERO), ld(&PID_OBS_NONZERO), ld(&PID_SKIP_CT), ld(&PID_ZERO_CLEAN)));
      s.push_str("     (★조합테스트는 유저 관점 백그라운드 brief-sim 이지만 SDK Scene 은 InGame 이고 그 db 는 pid=0 을 준다)\n");
      if p == 0 {
          s.push_str("  ⚠pid=0 = db 가 0 을 보고한 상태. pid 는 **경기 중(InGame)에만** 읽히고 조합테스트도 InGame 이라\n");
          s.push_str("     조합테스트만 하면 0 이 잡힌다 ⟹ **일반 경기(내 경기/관전)를 한 번 진행**해야 실 팀 id 가 잡힌다.\n");
      }
      { let sk = ld(&MY_TRUST_SKIP);
        if sk > 0 { s.push_str(&format!("  ★MY_ATHLETES 게시 보류  : {}회 (pid=0 + PT 미달 = 내 팀 미확정 → 팀 게이트를 안전측으로 닫음)\n", sk)); } }
      s.push_str(&format!("  MY_ATHLETES({}명) aid  :", ld(&MY_ATH_N)));
      let mut zero = false;
      for k in 0..8 { let v = MY_ATH_IDS[k].load(Ordering::Relaxed); if v == u64::MAX { continue; }
                      if v == 0 { zero = true; } s.push_str(&format!(" {}", v)); }
      s.push('\n');
      if zero { s.push_str("  ⚠★aid=0 이 목록에 있음 — 배경 sim 의 athlete_id 미기입(0)과 매칭돼 **광범위 false positive** 가 된다\n"); }
      s.push_str("  → 아래 buy 상세의 aid= 와 대조: 일치하면 db 가 실제로 그 선수를 내 팀 선발로 보고한 것,\n");
      s.push_str("     0 이나 이상값으로 일치하면 판정 결함(팀 게이트 무력화)이다.\n");
    }

    s.push_str("\n[지정챔프 buy 상세]  champ / owned / aid / live·player 판정 / 슬롯별 목표 / 실제 산 것\n");
    {
        let log = BR_LOG.lock().unwrap_or_else(|e| e.into_inner());
        if log.is_empty() { s.push_str("  (지정챔프 buy 기록 없음 — 위 3번이 0이면 이 경기에 지정챔프가 없는 것)\n"); }
        // 라이브 내 팀(player=true) 케이스를 맨 위로 — 봐야 할 핵심.
        let mut lines: Vec<&String> = log.iter().collect();
        lines.sort_by_key(|l| if l.contains("player=true") { 0 } else { 1 });
        let mut prev_live = true;
        for l in lines {
            let is_live_line = l.contains("player=true");
            if prev_live && !is_live_line { s.push_str("  ---- 아래는 적/배경 sim (참고) ----\n"); }
            prev_live = is_live_line;
            s.push_str("  "); s.push_str(l); s.push('\n');
        }
    }
    s.push_str("\n[현재 SEL 지정 내용 (build_override_map)]  champ slot → 주입목표값(1~6=바닐라, 30+=모드템ID)\n");
    {
        let m = build_override_map();
        if m.is_empty() { s.push_str("  (지정 없음 — SEL 파일이 비었거나 미지정)\n"); }
        let mut v: Vec<_> = m.iter().collect();
        v.sort_by(|a, b| a.0 .0.cmp(&b.0 .0).then(a.0 .1.cmp(&b.0 .1)));
        for ((c, slot), val) in v { s.push_str(&format!("  {} slot{} → {}\n", c, slot, val)); }
    }
    if let Some(d) = mod_dir() { let _ = fs::create_dir_all(&d); let _ = fs::write(d.join("buy_report.txt"), s); }
}

impl ModExtension for ItemTacticsExt {
    fn on_init(&self, _s: &mut Scene, _u: &mut GameUI, _a: &mut Assets) {}
    fn post_update(&self, scene: &mut Scene, ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        let __pt = perf::tsc();
        perf::sample_self(); // 프로브 자체 비용 샘플(프레임당 1회)
        { let t = perf::tsc();
          install_launcher_hook(); install_seed_ctor_hook(); install_game_view_hook(); // ★매프레임 재시도(멱등, 성공=1이면 즉시 return) — on_server_start 1회 실패 시 자가복구 + serpen 체인 재검증.
          perf::rec(perf::S_HOOK_RETRY, t); }
        if BUY_REPORT {
            // 새 관전 경기(InGame 진입) 엣지에서 카운터/로그 리셋 → buy_report.txt 엔 마지막 본 경기만.
            if matches!(scene, Scene::InGame { .. }) {
                if !BR_WAS_INGAME.swap(true, Ordering::Relaxed) { buy_report_reset(); }
            } else {
                BR_WAS_INGAME.store(false, Ordering::Relaxed);
            }
            buy_report_flush();
        }
        if !matches!(scene, Scene::InGame { .. }) { INGAME_NOW.store(false, Ordering::Relaxed); }
        if UI_INJECT_ENABLED { let t = perf::tsc(); unsafe { let _ = uinj::install(); } perf::rec(perf::S_POST_UINJ, t); } // 전술화면 드롭다운 주입훅(mode 3=item0m/1m/2m, mode 4=+item3/slot3). 멱등.
        // ★ 플레이어 팀 id 캡처(팀 스코프용) + personal_tactics 스냅샷(표시복원용).
        //   ⚠ 전술화면이 InGame 아닐 수 있어 #personal visible 게이트 제거 → 관리화면서 미리 채움.
        //   매 20프레임 throttle(HashMap 순회 비용 절감).
        if let Scene::InGame { data } = scene {
            let db = data.db();
            let pid = db.player_team_id();
            // ★경기 중 player_team_id()가 0/-1을 반환 → 유효 범위(1~9999)일 때만 저장, 아니면 마지막 유효값 유지.
            //   내 팀 id는 세션 중 불변이므로 관리/프리매치서 잡힌 값을 경기 중에도 사용.
            // ★pid=0도 유효(팀 id 공간이 0부터 — 실측: db.team(0)=Some, PT 5개). 무효=-1(u64::MAX)만.
            // ★★2026-07-30 결함 수정 — pid **후퇴 방지**.
            //   기존 주석은 "pid=0 도 유효(팀 id 공간이 0부터·db.team(0)=Some)"라고 판단했으나, 실측에서
            //   같은 세이브가 시점에 따라 **105 / 0** 을 오갔다(관리 화면 경유=105, 게임 시작 직후 조합테스트
            //   직행=0). 0 을 그대로 신뢰하면 team(0).last_starting=[0,1,2,3,4] 를 내 팀으로 게시해
            //   팀 게이트가 깨진다 ⟹ **0 이 아닌 유효 pid 를 한 번이라도 봤으면 0 으로 되돌리지 않는다.**
            //   (진짜 팀 id 가 0 인 세이브도 있을 수 있어 0 자체를 금지하진 않는다 — 비0 을 못 본 동안은 0 사용.)
            // ★★2026-07-30 실측 추가 — **조합테스트 경기 중에는 pid 를 갱신하지 않는다.**
            //   pid 는 `Scene::InGame` 에서만 읽히는데(관리 화면에서는 이 블록이 안 돌아 교정 기회가
            //   없다), 조합테스트도 InGame 이고 그 화면은 팀 소속 개념이 없어 `player_team_id()` 가
            //   **0 을 반환**한다. 그 0 이 게시되면 team(0).last_starting=[0,1,2,3,4] 가 내 팀이 된다.
            //   ⟹ 조합테스트 중 0 보고는 무시하고, 일반 경기에서 잡은 값을 유지한다.
            let in_comptest = COMPTEST_MATCH.load(Ordering::Relaxed);
            let pu = pid as u64;
            // ★진단: pid 관측 이력. ★★실측 확정(2026-07-30) — **조합테스트는 유저 관점 백그라운드
            //   brief-sim 이지만 SDK `Scene` 기준으로는 `InGame` 이고**(이 블록이 돌았음이 LIVE_DB≠0 으로
            //   증명됨), 그 컨텍스트의 `player_team_id()` 는 **0** 을 반환한다. 그게 pid=0 게시의 출처다.
            // ★2차 확장(2026-07-30 실측): `COMPTEST_MATCH` 는 조합테스트 **sim 이 시작된 뒤**(launcher
            //   발화 후)에만 true 라, 조합테스트 **화면 진입~sim 시작 전** 구간의 0 보고가 새어 게시됐다
            //   (실측: 0 관측 2416 중 1592 만 차단, 나머지 824 가 이 구간). ⟹ 조합테스트 팝업이 열려
            //   있는 동안(`CT_OPEN`)도 같은 컨텍스트로 보고 0 을 무시한다.
            let ct_ctx = in_comptest || CT_OPEN.load(Ordering::Relaxed);
            if ct_ctx { CT_EVER_SEEN.store(1, Ordering::Relaxed); } // ★08-06: 600틱 규칙의 발동 조건
            if pu == 0 {
                PID_OBS_ZERO.fetch_add(1, Ordering::Relaxed);
                if ct_ctx { PID_SKIP_CT.fetch_add(1, Ordering::Relaxed); }
                else { PID_ZERO_CLEAN.fetch_add(1, Ordering::Relaxed); } // 조합테스트와 무관한 0 관측
            } else if pu != u64::MAX && pu < 10000 { PID_OBS_NONZERO.fetch_add(1, Ordering::Relaxed); }
            if pu != u64::MAX && pu < 10000 && !(pu == 0 && ct_ctx) {
                PID_SRC.store(1, Ordering::Relaxed); // ★08-07: pid 출처 = InGame
                if pu != 0 {
                    // ★08-06: pid 가 실제로 바뀌면 **다음 프레임에 격자 무시하고 즉시 재게시**한다.
                    //   위 `roster_trust` 완화로 pid=0 을 일찍 믿게 됐으므로, 나중에 진짜 pid 가
                    //   나타났을 때 낡은 team(0) 로스터가 최대 120프레임 더 남는 일을 막는다.
                    if PLAYER_TEAM_ID.swap(pu, Ordering::Relaxed) != pu { ROSTER_FORCE.store(true, Ordering::Relaxed); }
                    PID_NONZERO_SEEN.store(1, Ordering::Relaxed);
                } else if PID_NONZERO_SEEN.load(Ordering::Relaxed) == 0 {
                    PLAYER_TEAM_ID.store(0, Ordering::Relaxed);
                }
                PID_EVER_VALID.store(1, Ordering::Relaxed);
            }
            // ★★v15: 내 팀 선발 로스터(athlete_id 5명) 게시 — 스폰 훅의 scene-free 팀판정 재료.
            //   ROSTER_POLL 주기 갱신(이적·선발변경 자동 반영). 로스터 확보 후엔 저빈도로 충분.
            {
                let __rt = perf::tsc();
                const ROSTER_POLL: u64 = 120; // 프레임
                let n = ROSTER_TICK.fetch_add(1, Ordering::Relaxed);
                let known = PLAYER_TEAM_ID.load(Ordering::Relaxed);
                // ★★08-06 수정 — **미게시/강제 상태에서는 120격자를 무시하고 매 프레임 시도**한다.
                //   구 코드는 `n % 120 == 0` 뿐이라, trust 가 성립한 프레임 뒤에도 최대 2초를 더 기다렸다.
                //   경기 시작 직후의 `owned==0` buy 를 놓치면 그 슬롯은 그 경기 내내 되돌릴 수 없다.
                let force = ROSTER_FORCE.swap(false, Ordering::Relaxed) || MY_ATH_N.load(Ordering::Relaxed) == 0;
                if (force || n % ROSTER_POLL == 0) && known != u64::MAX && known < 10000 {
                    let mut my: std::collections::HashSet<u64> = std::collections::HashSet::new();
                    let mut pt_n = 0usize;
                    if let Some(team) = db.team(known as _) {
                        pt_n = team.champion_personal_tactics.len();
                        for slot in team.last_starting.iter() {
                            if let Some(aid) = slot { my.insert(*aid as u64); }
                        }
                    }
                    MY_PT_N.store(pt_n as u64, Ordering::Relaxed);
                    // ⛔**PT 개수 교차검증 폐기(2026-07-30 실측 반박)**: "내 팀은 PT 수십 개·AI 팀은 몇 개뿐
                    //   (team(0) PT 5개)"이라는 과거 기록을 근거로 `pt_n >= 20` 을 썼으나, 실측에서
                    //   **team(0) PT = 95** 가 나와 임계값을 무의미하게 통과했다 ⟹ PT 개수는 판별력이 없다.
                    //   (pt_n 은 진단 표시용으로만 남긴다.)
                    // ★대체 규칙: `pid=0` 은 **미확정으로 간주해 기본 보류**한다(보류 = is_my_athlete None
                    //   = 팀 게이트를 안전측으로 닫음). 단 **조합테스트와 무관한 InGame 에서 0 을 충분히
                    //   (600틱≈10초) 관측**했다면 진짜 팀 id 가 0 인 세이브로 인정해 게시한다.
                    //   ⟹ 조합테스트만 한 세션에서는 절대 게시되지 않고, 일반 경기를 하면 실 pid 가 잡힌다.
                    // ★08-06: `trust` 판정을 `roster_trust()` 단일 창구로 이관(관리 틱과 규칙 공유).
                    //   구 식 `known != 0 || PID_ZERO_CLEAN >= 600` 에 "조합테스트 미관측 세션" 예외가 추가됐다.
                    let trust = roster_trust(known);
                    if !my.is_empty() && trust {
                        let sig = roster_sig(&my);
                        if ROSTER_SIG.swap(sig, Ordering::Relaxed) != sig || MY_ATH_N.load(Ordering::Relaxed) == 0 {
                            publish_my_athletes(my);
                        }
                    }
                    else if !trust { MY_TRUST_SKIP.fetch_add(1, Ordering::Relaxed); }
                }
                // ★08-06: itemnet 재시도(경기 중). 같은 120프레임 스로틀에 얹는다 — 후보 3개 검사뿐이라 저비용.
                if n % ROSTER_POLL == 0 {
                    itemnet_retry();
                    game_team_probe();
                    // ★08-07 구멍4 — **음성 캐시(-1)를 주기적으로 비운다.**
                    //   `scan_idx_cached` 는 실패를 영구 캐시하는데, 실패는 일시적일 수 있다
                    //   (카탈로그 미완성·레시피 미로드 시점에 조회되면 그 아이템이 그 컬렉션에서 영영 미발견).
                    //   양성 캐시는 유지 = 성능 목적 그대로, 실패만 10초마다 재도전.
                    if (n / ROSTER_POLL) % 5 == 0 {
                        let mut g = SCAN_CACHE.lock().unwrap_or_else(|e| e.into_inner());
                        if let Some(outer) = g.as_mut() {
                            let mut purged = 0u64;
                            for (_b, m) in outer.iter_mut() {
                                let before = m.len();
                                m.retain(|_k, v| *v >= 0);
                                purged += (before - m.len()) as u64;
                            }
                            if purged > 0 { SCAN_NEG_PURGE.fetch_add(purged, Ordering::Relaxed); }
                        }
                    }
                    // ★08-07: 경기 중에도 registry 갱신(구: probe_db 에서만 → 경기 전 스냅샷이 굳었다).
                    //   무변경이면 write 생략이라 비용 0.
                    let sdb = SERVER_DB.load(Ordering::Relaxed) as usize;
                    if sdb >= 0x10000 { write_registry_status(sdb); }
                }
                perf::rec(perf::S_POST_ROSTER, __rt);
            }
            // ★★lean(07-18): 관전 식별 = launcher(LIVE_SEED)+seed-ctor(RENDER_PROVIDER)+buy r9 대조(v13).
            //   구 db스캔(v10)·P6프로브·링크스캔 전부 제거. 여기선 scene side(내 팀 판정)와 LIVE_DB/PID만 유지.
            if !DIAG_BUY_OFF {
                let dbp = unsafe { (&*db as *const _) as usize };
                INGAME_NOW.store(true, Ordering::Relaxed);
                // ★스폰/즉석판정용 db·pid 저장 (buy 훅의 quick_scene_side 폴백 재료)
                LIVE_DB.store(dbp as u64, Ordering::Relaxed);
                { let pu = PLAYER_TEAM_ID.load(Ordering::Relaxed); if pu != u64::MAX && pu < 10000 { LIVE_PID.store(pu, Ordering::Relaxed); } }
                // ★scene 직독(경량): SCENE_SIDE = scene team_id ↔ 내 팀 매칭 (is_player 판정 재료)
                if SCENE_GATE_ENABLED {
                    let t = perf::tsc();
                    let pid_known = PLAYER_TEAM_ID.load(Ordering::Relaxed);
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
                        SCENE_SIDE.store(quick_scene_side(dbp, pid_known).unwrap_or(u64::MAX), Ordering::Relaxed);
                    }));
                    perf::rec(perf::S_POST_SCENESIDE, t);
                }
            }
            // ★블루 슬롯/스탯 x(+0x84 4상태) 매프레임 강제: 게임이 blue_player를 바닐라간격50으로 재설정하는 걸 42간격+왼쪽정렬로 덮어씀.
            //   ⚠compact(player_info)만 — wide_player_info(전체화면)는 리셋 없이 .ui로 정상이라(간격34·다른 kda/cs)
            //   ui.root 전체에 걸면 wide까지 compact값(42/242/290)으로 덮여 깨짐. player_info 서브트리로 한정.
            if ITEM_MODE.load(Ordering::Relaxed) == 4 {
                let t = perf::tsc();
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
                    if let Some(pi) = find_node(&ui.root, "player_info") { force_blue_slot_spacing(pi); }
                }));
                perf::rec(perf::S_POST_SPACING, t);
            }
            // ★ delegate(팀파매gg 자동선택) 베이스라인 소스 = champion_personal_tactics.
            //   매 프레임 갱신(경량 ~52엔트리) → 표시/주입 항상 최신. 로그만 20프레임 스로틀.
            //   갱신 후 OVERRIDE_SNAPSHOT 재빌드(시그니처 가드 → 무변경시 leak 없음) →
            //   전술화면 안 열어도 delegate 방향이 c6 주입에 반영됨.
            // ★스로틀(2026-07-22 perf 계측): 이 블록이 InGame 매 프레임 **최소 174µs** — 위 주석의
            //   "경량 ~52엔트리"는 오판이었다(챔프별 String clone + HashMap 재구축 + update_override_snapshot
            //   의 맵빌드/정렬/FNV까지 매 프레임). delegate(champion_personal_tactics)는 **유저 조작으로만**
            //   바뀌므로 매 프레임 재구축할 이유가 없다 → 20프레임(≈0.3s) 주기. 표시·주입 반응성엔 체감 차 없음.
            static PT_REBUILD: AtomicU64 = AtomicU64::new(0);
            if PT_REBUILD.fetch_add(1, Ordering::Relaxed) % 20 == 0 {
                if let Some(t) = db.team(pid) {
                    let __ptt = perf::tsc(); // ⚠ 여기서 `t`는 팀(위 바인딩)이라 프로브 변수명 분리
                    let mut snap = HashMap::new();
                    for (champ, arr) in t.champion_personal_tactics.iter() {
                        let p = arr.as_ptr() as *const u8;
                        snap.insert(champ.clone(), unsafe { [*p, *p.add(1), *p.add(2)] });
                    }
                    if LOG_ENABLED {
                        let mut s = format!("[{}ms] PT_SNAPSHOT 갱신 pid={} {}개\n", now_ms(), pid, snap.len());
                        for (k, b) in snap.iter().take(64) { s.push_str(&format!("  {} = [{},{},{}]\n", k, b[0], b[1], b[2])); }
                        write_log("item_tactics_pt.txt", &s);
                    }
                    *PT_SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner()) = Some(snap);
                    update_override_snapshot();
                    perf::rec(perf::S_POST_PT, __ptt);
                }
            }
        }
        { let t = perf::tsc();
          let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| { handle_tactics_screen(ui); }));
          perf::rec(perf::S_POST_TACTICS, t); }
        { let t = perf::tsc();
          let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| { handle_comptest_screen(ui); }));
          perf::rec(perf::S_POST_COMPTEST, t); }
        // ★경기중 4번째 슬롯 아이콘(노드 직접 세팅 방식 — 게임 코드 무수정)
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| { handle_ingame_slot3(ui); }));
        // ★네이티브 item0/1/2 숨김(모드소유 item0m/1m/2m 오버레이가 대체). 개인전술 화면서만. mode 3·4 공통(오버레이 양 모드 존재).
        { let t = perf::tsc();
          let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| hide_native_item_dds(&mut ui.root)));
          perf::rec(perf::S_POST_HIDE_DD, t); }
        // ★조합테스트: 모드소유 드롭다운이 대체하므로 네이티브 item0/1/2 숨김(멱등 — 이미 false면 write 없음).
        { let t = perf::tsc();
          let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
              if find_node(&ui.root, "builds").map(|n| n.visible).unwrap_or(false) {
                  hide_comptest_native_dds(&mut ui.root);
              }
          }));
          perf::rec(perf::S_POST_HIDE_CT, t); }
        perf::rec(perf::S_POST_TOTAL, __pt);
        // 주기 리포트(메인스레드에서만 파일 write).
        if perf::PERF_ON {
            static PERF_FRAMES: AtomicU64 = AtomicU64::new(0);
            let n = PERF_FRAMES.fetch_add(1, Ordering::Relaxed) + 1;
            if n % perf::REPORT_EVERY == 0 {
                let mut rep = perf::report(now_ms(), n);
                // ★훅 설치 경로 진단: "실제 install"이 프레임당 ~1이면 매프레임 재설치 = 스텁 누수 +
                //   serpen 상호 재체인 사이클(draft_overlay hang 전례) 확정. 0에 가까우면 무죄.
                rep.push_str(&format!(
                    "\n── 훅 설치 경로 카운터 ──\n  launcher: 호출={} / 내스텁확인={} / serpen대기={} / ★실제install={} / 스로틀skip={}\n  \
                     마지막 진입부 b0={:#04x} movabs_tgt={:#x} (내스텁={:#x})\n  seed_ctor: ★실제install={}\n  \
                     → 실제install ÷ 프레임 ≈ {:.3} (1에 가까우면 매프레임 재설치=누수)\n",
                    HK_L_CALLS.load(Ordering::Relaxed), HK_L_OURS.load(Ordering::Relaxed),
                    HK_L_WAIT.load(Ordering::Relaxed), HK_L_INSTALL.load(Ordering::Relaxed),
                    HK_L_SKIP.load(Ordering::Relaxed),
                    HK_L_B0.load(Ordering::Relaxed), HK_L_TGT.load(Ordering::Relaxed),
                    CLAUNCH_STUB.load(Ordering::Relaxed),
                    HK_S_INSTALL.load(Ordering::Relaxed),
                    HK_L_INSTALL.load(Ordering::Relaxed) as f64 / n.max(1) as f64));
                if let Some(p) = mod_dir() { let _ = fs::write(p.join("item_tactics_perf.txt"), rep); }
            }
        }
    }
}

// 서버측: Database 접근해 모드 아이템 레지스트리 1회 채움.
struct ItemTacticsServerExt;
impl ModServerExtension for ItemTacticsServerExt {
    fn on_server_start(&self, ctx: &mut ServerModContext) { probe_db(ctx); install_replace_4th(); install_launcher_hook(); install_seed_ctor_hook(); } // resolver=mode 3·4 공통(슬롯0/1/2 지정) + v13 식별훅(launcher 시드 + seed-ctor provider)
    fn before_management_tick(&self, ctx: &mut ServerModContext) {
        // ★매치 사이(관리화면)서 팀게이트 캐시 리셋 → 다음 경기서 로스터 재스캔(주소 재사용 대비).
        //   관리틱은 match sim 중엔 안 도므로 sim스레드의 판정과 레이스 없음.
        PLAYER_SIDE.store(u64::MAX, Ordering::Relaxed);
        SIDE_CACHE.lock().unwrap_or_else(|e| e.into_inner()).clear();
        probe_db(ctx); install_replace_4th(); // resolver=mode 3·4 공통 (멱등)
        refresh_roster_management(ctx); // ★08-06: 선발 변경을 경기 진입 전에 반영(경기 밖 유일 갱신점)
        // ★측정 전용: `dump_builds.trigger` 파일이 있을 때만 1회 실행(평소 비용 = exists() 1회).
        //   관리틱은 sim 중엔 안 돌므로 forward shadow-call 이 sim 과 레이스하지 않는다.
        unsafe { maybe_dump_builds(); }
    }
}
static NETSCAN_DONE: AtomicBool = AtomicBool::new(false);
// ★★2026-08-06 신설 — **관리 틱(경기 밖)에서도 내 팀 선발 로스터를 갱신한다.**
//   왜: 게시 사이트가 `Scene::InGame` 안에만 있어서, **선발 변경은 관리 화면에서 일어나는데 갱신은
//   경기 중에만** 됐다. 선발을 바꾸고 경기에 들어가지 않은 채 일정을 넘기면 그 배경 sim 은 **직전
//   경기 때의 낡은 선발 5명**으로 팀 판정을 해서, 새 선발의 지정이 통째로 미적용된다.
//   (기존 코드 주석 자신이 "관리 화면에서는 이 블록이 안 돌아 교정 기회가 없다"고 한계를 적고 있었다.)
//   pid 는 InGame 에서만 읽히므로 여기서는 **이미 잡아둔 `PLAYER_TEAM_ID`** 만 쓴다(새로 판정하지 않음).
//   무변경이면 재게시하지 않는다 — `publish_my_athletes` 는 Box 세대교체라 매 틱 호출하면 낭비다.
fn refresh_roster_management(ctx: &mut ServerModContext) {
    // ★★08-07 구멍1 — **`pid` 를 관리틱에서도 확보한다.**
    //   구조: `player_team_id()` 는 `Scene::InGame` 에서만 읽혔다 ⟹ **게임을 켜고 경기를 한 번도
    //   관전/진행하지 않은 채 일정만 넘기면 pid 가 영영 안 잡히고**, `MY_ATHLETES` 미게시 →
    //   배경 sim 조기탈출이 전량 발동 → **그 세션 주입 전멸**. 재현이 안 되는 제보의 1순위 후보다
    //   (관전을 자주 하는 환경에서는 절대 안 나타난다).
    //   해결: 서버측 `ctx.server_state.players` 의 team_id 를 쓴다(`tfm2_meta_item_delegate` 가 쓰는 소스).
    //   ⚠멀티플레이 대비 — **distinct team_id 가 정확히 1개일 때만** 채택한다.
    if PLAYER_TEAM_ID.load(Ordering::Relaxed) == u64::MAX {
        let mut only: Option<u64> = None;
        let mut multi = false;
        for (_k, p) in ctx.server_state.players.iter() {
            let t = p.team_id as u64;
            match only { None => only = Some(t), Some(v) if v != t => multi = true, _ => {} }
        }
        if !multi {
            if let Some(t) = only {
                if t < 10000 {
                    PLAYER_TEAM_ID.store(t, Ordering::Relaxed);
                    if t != 0 { PID_NONZERO_SEEN.store(1, Ordering::Relaxed); }
                    PID_EVER_VALID.store(1, Ordering::Relaxed);
                    PID_SRC.store(2, Ordering::Relaxed);
                    ROSTER_FORCE.store(true, Ordering::Relaxed);
                }
            }
        }
    }
    let known = PLAYER_TEAM_ID.load(Ordering::Relaxed);
    if known == u64::MAX || known >= 10000 { return; }
    if !roster_trust(known) { return; }
    let Some(team) = ctx.database.teams.get(known as usize) else { return; }; // ⚠teams=SlotMap류(핸들 인자, &키 아님)
    let mut my: std::collections::HashSet<u64> = std::collections::HashSet::new();
    for slot in team.last_starting.iter() {
        if let Some(aid) = slot { my.insert(*aid as u64); }
    }
    if my.is_empty() { return; }
    let sig = roster_sig(&my);
    if ROSTER_SIG.swap(sig, Ordering::Relaxed) == sig && MY_ATH_N.load(Ordering::Relaxed) != 0 { return; }
    publish_my_athletes(my);
}

// ★champion_patch_statistics 의 Database 구조체 내 오프셋. ~~0x16698~~ → **0x16ed8**
//   (0.5.5 실측 정정 2026-08-12: 자가치유 프로브 `실제 cps 오프셋 = 0x16ed8` — item_tactics_registry.txt,
//    구 하드코딩 0x16698 대비 +0x840. 0.5.4 실측은 0x16ec0(+0x828)이었으니 0.5.4→0.5.5 실이동은 +0x18.
//    ⚠어차피 base 는 dbase(addr_of 직접취득)가 1순위라 이 상수는 폴백·진단 표시용이다.)
const CPS_OFF: usize = core::mem::offset_of!(game_core::Database, champion_patch_statistics);   // ★★0.5.7(2026-08-26): 하드코딩 폐기 → 컴파일 타임 계산(SDK 구조체가 정본). 구값 ~~0.5.6 0x16ff8~~ / ~~0.5.5 0x16ed8~~. // 0.5.6(구0.5.5=0x16ed8, +0x120). ClientDatabase 고대역 재시프트(동일오프셋 직독 투표 39/40). ⚠어차피 dbase(addr_of 직접취득)가 1순위라 이 상수는 폴백·진단용.

fn probe_db(ctx: &mut ServerModContext) {
    // Database 시작 = champion_patch_statistics(@Database+CPS_OFF) 절대주소 − CPS_OFF.
    let cps = &ctx.database.champion_patch_statistics as *const _ as usize;
    let db = cps.wrapping_sub(CPS_OFF);
    // ★★2026-08-07 — **Database 구조체 시작 주소를 하드코딩 없이 직접 취득한다.**
    //   구 코드는 `db = cps − 0x16698` 로 역산했는데, 그 `0x16698`(= champion_patch_statistics 의
    //   구조체 내 오프셋)이 0.5.4 에서 유효한지는 **한 번도 검증된 적이 없다**. 그런데 exe 대조로
    //   net = GameData+0x1558 은 0.5.4 에서도 불변인데 런타임 `db+0x1558` 이 계속 빗나갔다
    //   ⟹ 의심할 것은 오프셋이 아니라 base 다.
    //   `ctx.database` 는 참조이므로 `addr_of!(*ctx.database)` 가 곧 구조체 base = exe 의 `r13` 후보.
    //   ⟹ 이제 `0x16698` 없이 정확한 base 를 얻는다(다음 패치에도 안 썩는다).
    let dbase = core::ptr::addr_of!(*ctx.database) as usize;
    DB_DIRECT.store(dbase as u64, Ordering::Relaxed);
    SERVER_DB.store(db as u64, Ordering::Relaxed);   // ★08-06: 경기 중 itemnet 재시도용(probe_db 가 1회뿐일 수 있음)
    SERVER_CPS.store(cps as u64, Ordering::Relaxed); // ★08-06: 구 역산 base 와의 대조용
    // ── 아이템 신경망 probe + 자가검증(16384/16384/1) ──
    //   ★0.5.0_3 실측: db+0xd30 (구 0xda0에서 -0x70 이동, netscan 진단 HIT). 후보 순차 + 윈도우 스캔 폴백(패치 견고).
    if ITEM_NET_ADDR.load(Ordering::Relaxed) == 0 {
        unsafe {
            // ★0.5.1 시그 강화(ghidra-re): 헤더(16384/16384/1)만 맞는 lookalike(db+0xd30)는 +0x8 가중치ptr이 dangling
            //   → forward 내부 +0x44a서 deref AV. 가중치ptr readable 검증 추가로 가짜 탈락, 진짜 net(db+0x1558)만 통과.
            let sig_ok = |a: usize| readable(a, 0x20) && rd_u64(a) == 16384 && rd_u64(a + 0x10) == 16384 && rd_u64(a + 0x18) == 1
                && { let w = rd_u64(a + 0x8) as usize; w >= 0x10000 && readable(w, 16384 * 4) };
            let mut found = 0usize;
            // ★08-07: **직접 취득한 base(dbase) 를 1순위**, 구 역산 base(db) 를 2순위로 시도한다.
            //   net=GameData+0x1558 은 0.5.3·0.5.4 exe 대조 확정(둘 다 beam 의 net 인자 `lea …,[r13+0x1558]`).
            'outer: for &base in &[dbase, db] {
                for &off in &[0x1558usize, 0xd30, 0xda0] {
                    if sig_ok(base + off) { found = base + off; break 'outer; }
                }
            }
            // 윈도우 자동탐색(향후 패치서 또 이동해도 자가복구).
            // ★08-06 상한 신설: `sig_ok` 는 readable()=VirtualQuery 커널호출이라 1회 스캔이 12,288 호출이다.
            //   `ITEM_NET_ADDR==0` 인 동안 **관리틱마다** 이게 돌면 순수 낭비 ⟹ 시도 횟수를 제한한다.
            //   후보 3개 직접검사는 싸므로 상한 이후에도 계속 돈다(늦게 할당되는 경우를 계속 잡는다).
            if found == 0 && NETSCAN_TRIES.fetch_add(1, Ordering::Relaxed) < 64 {
                // ⚠좁은 창(96KB)·`sig_ok`(=readable 선행) 유지 — 08-06 가드페이지 크래시의 재발 방지선.
                let mut o = 0usize;
                'w: for &base in &[dbase, db] {
                    o = 0;
                    while o < 0x18000 { let a = base + o; if sig_ok(a) { found = a; break 'w; } o += 8; }
                }
            }
            if found != 0 {
                ITEM_NET_ADDR.store(found as u64, Ordering::Relaxed);
                append_log("4items.txt", &format!("[{}ms] item_net={:#x} (db+{:#x}) ★유효 fwd_valid={}", now_ms(), found, found - db, itemnet_addr_valid()));
            } else {
                let net = db + 0xda0;
                // ★진단(LOG무관): +0xda0 실패 → cps 기준 넓은 창에서 net 시그(16384/*/16384/1) 스캔해 실제 오프셋 찾기.
                //   + forward RVA 프롤로그도 덤프(itemnet_addr_valid 실패 원인 구분). 한 번만.
                if !NETSCAN_DONE.swap(true, Ordering::Relaxed) {
                    let mut out = format!("db={:#x} cps={:#x} (champ_patch_stat off={CPS_OFF:#x})\n net@+0xda0={:#x} sig=({},{},{}) readable={}\n",
                        db, cps, net,
                        if readable(net,0x20){rd_u64(net) as i64}else{-1}, if readable(net,0x20){rd_u64(net+0x10) as i64}else{-1},
                        if readable(net,0x20){rd_u64(net+0x18) as i64}else{-1}, readable(net,0x20));
                    // db 기준 0..0x18000 스캔: rd(O)==16384 && rd(O+0x10)==16384 && rd(O+0x18)==1
                    let mut hits = 0;
                    let mut o = 0usize;
                    while o < 0x18000 && hits < 8 {
                        let a = db + o;
                        if readable(a, 0x20) && rd_u64(a) == 16384 && rd_u64(a + 0x10) == 16384 && rd_u64(a + 0x18) == 1 {
                            out.push_str(&format!(" ★HIT db+{:#x} (abs={:#x})\n", o, a)); hits += 1;
                        }
                        o += 8;
                    }
                    if hits == 0 { out.push_str(" (스캔 무결과 — db base 자체 의심 or 시그 변경)\n"); }
                    // forward RVA 프롤로그
                    let fa = exe_base_addr() + ITEMNET_FORWARD_RVA;
                    if readable(fa, 12) {
                        let pb: Vec<String> = (0..12).map(|i| format!("{:02x}", *((fa+i) as *const u8))).collect();
                        out.push_str(&format!(" fwd RVA={:#x} prologue={} (기대 55415741...)\n", ITEMNET_FORWARD_RVA, pb.join(" ")));
                    } else { out.push_str(" fwd RVA unreadable\n"); }
                    if let Some(d) = mod_dir() { let _ = fs::create_dir_all(&d); let _ = fs::write(d.join("4items_netscan.txt"), out); }
                }
            }
        }
    }
    // ★08-06 정정 — 이 write 를 `MODITEMS_DONE` early-return **앞**으로 옮긴다.
    //   ⚠구현 실수 경위: 처음엔 이 블록이 early-return 뒤에 있어 **세션 최초 probe_db 1회만** 기록됐다.
    //   그 시점은 GameData 가 아직 다 안 채워졌을 수 있는 가장 이른 순간이라, `ITEM_NET_ADDR=0` 이
    //   "끝내 실패"가 아니라 "아직 못 찾음"인데도 **★FAIL 로 단정 보고**하게 만들었다(실제로 오보했다).
    //   itemnet 은 관리틱마다 재시도되므로 **상태가 바뀌면 다시 쓴다**(무변경이면 write 생략 = 비용 0).
    if !MODITEMS_DONE.load(Ordering::Relaxed) {
        // ★08-07: 구 역산 base(`db`) 대신 **직접 base(`dbase`)** 를 쓴다.
        //   실측으로 `0x16698` 이 STALE 확정(실제 = `0x16ec0`, 두 base 차 −0x828)이므로 구 base 는
        //   구조체 시작보다 0x828 뒤에서 스캔을 시작하고 있었다. 지금까지 123개를 찾아낸 건
        //   스캔 창(0x60000)이 넓어서였을 뿐 = 운. 검증은 이 파일의 `MOD_REGISTRY` 줄로 즉시 된다.
        unsafe { dump_mod_items(dbase); }
        append_log("item_tactics.txt", &format!("[{}ms] probe_db: db={:#x} 모드템 {}개 최종 {}개", now_ms(), db,
            MOD_REGISTRY.lock().unwrap_or_else(|e| e.into_inner()).len(),
            MOD_FINALS.lock().unwrap_or_else(|e| e.into_inner()).len()));
    }
    write_registry_status(db); // ★항상 **마지막** — dump 결과가 반영된 상태를 기록해야 판독이 된다
}

// ★★08-06 신설 — 경기 중에도 itemnet 재시도.
//   경위: `probe_db` 는 `on_server_start` 1회 + `before_management_tick` 에서만 돈다. 그런데
//   **management tick 은 "일정 넘김" 때만 발생**하지 관리 화면에 머무는 동안 도는 게 아니다.
//   ⟹ 유저가 세이브 로드 → 곧장 경기로 가면 **세션 전체에서 probe_db 가 단 1회**뿐이고,
//   그 1회에 아직 할당 전이던 net 은 영영 못 잡는다(08-06 실측: 파일이 `윈도우 스캔 1/64회` 로 멈춰 있었다).
//   비용 = 후보 3개 직접검사(readable 3회)뿐 — 전범위 윈도우 스캔은 하지 않는다.
static SERVER_DB: AtomicU64 = AtomicU64::new(0);
static SERVER_CPS: AtomicU64 = AtomicU64::new(0);
static DB_DIRECT: AtomicU64 = AtomicU64::new(0); // ★08-07: addr_of!(*ctx.database) = 하드코딩 없는 진짜 base
static NETWIDE_TRIES: AtomicU64 = AtomicU64::new(0);
static NETRETRY_N: AtomicU64 = AtomicU64::new(0); // itemnet_retry 호출 수(배선이 살아있는지 증명용)
// net 시그 검사.
// ⛔★★2026-08-06 사고 — 여기서 `readable()` 을 생략하고 `safe_read_u64` 로 바로 읽었다가
//   **게임을 크래시시켰다**(`code=0x80000001` = STATUS_GUARD_PAGE_VIOLATION, 세이브 로드 중).
//   `readable()` 은 `mbi.protect & (PAGE_NOACCESS|PAGE_GUARD)` 를 걸러낸다(L153) — 즉 **가드 페이지를
//   피하는 유일한 장치가 그 VirtualQuery 였다.** VEH 는 AV 는 잡아도 가드페이지 위반을 삼키면
//   스레드 스택 자동증가 메커니즘이 깨져 프로세스가 죽는다.
//   ⟹ **임의 주소를 만지기 전 `readable()` 은 생략 불가.** "커널호출이 비싸다"는 최적화 동기로
//     이걸 빼는 것은 재시도 금지(= DONE.md 등재).
unsafe fn net_sig_at(a: usize) -> bool {
    if !readable(a, 0x20) { return false; }   // ⛔ 이 줄을 절대 지우지 말 것(위 사고 경위)
    if rd_u64(a) != 16384 || rd_u64(a + 0x10) != 16384 || rd_u64(a + 0x18) != 1 { return false; }
    let w = rd_u64(a + 8) as usize;
    // 가중치 ptr 검증(lookalike 배제 — 이게 없으면 forward 내부 +0x44a 에서 deref AV)
    w >= 0x10000 && readable(w, 16384 * 4)
}
// ★★2026-08-07 신설 — **"매치 생성 시점에 쥔 Game 을 계속 들고 있을 수 있는가 + 거기 팀 정보가 있는가"**
//   를 실측으로 답한다. 정적 RE 로는 호출자가 92KB 패킷 디스패처라 제어흐름 추적 비용이 크다.
//   ①stale 여부 = launcher 가 잡은 Game 과 buy 가 받는 Game(`[rsp_entry+0x30]`)이 같은 포인터인가.
//   ②팀 정보 = 같다면, 그 Game 안에 scene 이 아는 팀 id 쌍(t1,t2)이 **인접**해 있는가.
//     인접쌍으로 찾는 이유: team id 는 작은 정수라 단일 값 스캔은 오탐투성이다.
//   ⚠안전: `readable()` 1회로 영역 전체를 검증한 뒤 평범한 읽기만 한다(08-06 가드페이지 사고 재발 방지).
//     범위도 Game 이 실제로 쓰는 대역(+0x2090 까지 launcher 가 기록)에 맞춘 0x2100 으로 **좁게** 고정.
fn game_team_probe() {
    if GAME_PROBE_DONE.load(Ordering::Relaxed) { return; }
    let lg = LIVE_GAME.load(Ordering::Relaxed) as usize;
    let bg = BUY_GAME.load(Ordering::Relaxed) as usize;
    if lg < 0x10000 || bg < 0x10000 { return; }
    let (t1, t2) = (SCENE_T1.load(Ordering::Relaxed), SCENE_T2.load(Ordering::Relaxed));
    if t1 == u64::MAX || t2 == u64::MAX { return; }
    GAME_PROBE_DONE.store(true, Ordering::Relaxed);
    // ★08-07 2차 수정 — 스캔 대상을 **buy Game(bg)** 으로 고정한다.
    //   1차에선 `lg == bg` 일 때만 스캔했는데, 실측 결과 둘이 다르다(launcher rcx = 임시).
    //   그런데 우리가 알고 싶은 "Game 이 팀을 아는가"의 대상은 **경기 중 살아있는 Game = bg** 다.
    //   핸들 일치 여부와 팀필드 유무는 별개 질문인데 하나로 묶어 게이팅한 게 오류였다.
    let same = lg == bg;
    let mut s = format!("[{}ms] Game 핸들·팀필드 프로브\n\n", now_ms());
    s.push_str(&format!("  launcher rcx (매치 생성 시점) = {:#x}\n", lg));
    s.push_str(&format!("  buy [rsp+0x30] (경기 중)      = {:#x}\n", bg));
    s.push_str(&format!("  ★같은 핸들인가 = {}\n", if same { "예 — 매치 생성 시점에 쥐면 계속 쓸 수 있다" }
                                                   else { "아니오 — launcher rcx 는 임시(이후 이동됨) = 들고 있으면 stale" }));
    s.push_str(&format!("  scene 팀 id: t1={} t2={} | LIVE_PID={}\n\n", t1, t2, LIVE_PID.load(Ordering::Relaxed)));
    // ★스캔 자체는 buy 디투어 안에서 이미 끝났다(Game 은 스택 상주라 여기선 못 읽는다) — 여기선 결과만 출력.
    match GAME_SCAN_OK.load(Ordering::Relaxed) {
        0 => s.push_str("  [팀필드 스캔] 미실행 — 화면 경기 buy 가 아직 없었거나 scene 팀 id 미확보\n"),
        2 => s.push_str("  [팀필드 스캔] ⚠readable=false — 디투어 시점에도 Game 영역을 못 읽었다\n"),
        _ => {
            let n = GAME_HIT_N.load(Ordering::Relaxed);
            if n == 0 {
                s.push_str("  [팀필드 스캔] ★팀 id 쌍 없음 — **Game 은 팀을 모른다**\n\
                            \x20   (sim 계층 team_id 부재 확정과 정합) ⟹ 팀 판정은 scene side 가 최선.\n");
            } else {
                for k in 0..(n as usize).min(4) {
                    let v = GAME_HIT[k].load(Ordering::Relaxed);
                    let (off, kind) = (v & 0x7fff_ffff, if v & 0x8000_0000 != 0 { "u32쌍" } else { "u64쌍" });
                    s.push_str(&format!("  ★팀 id 쌍 발견({}) : Game+{:#06x}\n", kind, off));
                }
                s.push_str("  ⟹ 이 오프셋이 매치마다 재현되면 **스폰 시점(rcx=Game)에 팀 확정 가능**.\n");
            }
        }
    }
    if let Some(d) = mod_dir() { let _ = fs::create_dir_all(&d); let _ = fs::write(d.join("item_tactics_gameprobe.txt"), s); }
}

fn itemnet_retry() {
    NETRETRY_N.fetch_add(1, Ordering::Relaxed);
    if ITEM_NET_ADDR.load(Ordering::Relaxed) != 0 { return; }
    let db = SERVER_DB.load(Ordering::Relaxed) as usize;
    if db < 0x10000 { return; }
    // 재시도가 돌았다는 사실 자체를 파일에 반영(광역스캔 카운트가 sig 에 들어있어 자동 갱신된다)
    if NETRETRY_N.load(Ordering::Relaxed) <= 12 { write_registry_status(db); }
    unsafe {
        // ★08-07: 직접 base 1순위, 구 역산 base 2순위. 0x1558 = 0.5.4 exe 대조로 확인된 GameData 내 정위치.
        for &base in &[DB_DIRECT.load(Ordering::Relaxed) as usize, db] {
            if base < 0x10000 { continue; }
            for &off in &[0x1558usize, 0xd30, 0xda0] {
                let a = base + off;
                if net_sig_at(a) { ITEM_NET_ADDR.store(a as u64, Ordering::Relaxed); write_registry_status(db); return; }
            }
        }
        // ⛔★★2026-08-06 폐기 — **cps 기준 광역 메모리 스캔은 재시도 금지.**
        //   시도했던 것: cps ± 수백KB 를 8바이트 간격으로 훑어 net 시그를 찾고 db 를 역산.
        //   결과: **게임 크래시**(`code=0x80000001` STATUS_GUARD_PAGE_VIOLATION, 세이브 이어하기 중).
        //   근본원인: 넓은 창을 훑으면 언젠가 **다른 스레드의 스택 가드 페이지**를 밟는다. 가드페이지
        //   위반은 "AV 를 VEH 로 삼키면 된다" 가 통하지 않는다 — 삼키는 순간 스택 자동증가가 깨진다.
        //   ⟹ 임의 주소 광역 스캔 자체가 잘못된 접근. 다음 수단은 **beam(0.5.4 `0x145b090`) 진입부에
        //     캡처 훅을 걸어 rdx(=net) 를 그대로 받는 것** — 오프셋 가정이 0이고 스캔도 없다.
        let _ = &NETWIDE_TRIES; // (구 스캔 카운터 — 유지만, 미사용)
        #[cfg(any())]
        {
        // ↓이하 폐기된 구현(참고용, 컴파일 제외)
        //   근거: exe 대조로 net = GameData+0x1558 은 0.5.4 에서도 불변인데(0.5.3 `lea rsi,[r13+0x1558]`
        //   ↔ 0.5.4 `lea r15,[r13+0x1558]`, 둘 다 beam 의 net 인자), 런타임 `db+0x1558` 이 경기 한 판을
        //   다 돌리도록 계속 빗나갔다 ⟹ **`db`(= cps − 0x16698) 가 GameData 가 아니다 = `0x16698` 이 stale.**
        //   ⟹ 하드코딩 오프셋을 믿지 말고 **cps 로부터 net 을 직접 찾아 db 를 역산**한다.
        //   net 은 GameData 안에서 cps(+0x16698 자리)보다 **앞**에 있으므로 뒤쪽을 넓게 본다.
        //   ⚠비용: VEH 읽기만 사용(커널호출 없음). 시도 8회 상한 · 120프레임 간격.
        if NETWIDE_TRIES.fetch_add(1, Ordering::Relaxed) >= 8 { return; }
        let cps = SERVER_CPS.load(Ordering::Relaxed) as usize;
        if cps < 0x10000 { return; }
        // ★08-06 4차: 범위 확대(구 −0x40000/+0x8000 에서 무결과). VEH 읽기라 커널호출 0 = 확대 비용 낮음.
        let lo = cps.saturating_sub(0x100000);
        let hi = cps + 0x20000;
        let mut a = lo & !7usize;
        while a < hi {
            if net_sig_at(a) {
                ITEM_NET_ADDR.store(a as u64, Ordering::Relaxed);
                // db 역산: net 이 GameData+0x1558 이므로 GameData = net − 0x1558.
                let real_db = a.wrapping_sub(0x1558);
                let cps_off = cps.wrapping_sub(real_db);
                let s = format!(
                    "[{}ms] ★itemnet 광역 탐색 성공 — db base 하드코딩이 틀렸다는 증거\n\n\
                     \x20 cps(&champion_patch_statistics) = {:#x}\n\
                     \x20 net 실제 위치                    = {:#x}\n\
                     \x20 역산 GameData(= net − 0x1558)    = {:#x}\n\
                     \x20 ★역산 cps 오프셋                = {:#x}   (소스 하드코딩 = 0x16698)\n\
                     \x20 소스가 쓰던 db(= cps − 0x16698)  = {:#x}   (차이 = {}{:#x})\n\n\
                     \x20 ⟹ `probe_db` 의 `0x16698` 을 위 '역산 cps 오프셋' 으로 교체할 것.\n\
                     \x20 ⟹ 이 값은 `dump_mod_items(db)` 도 함께 쓰므로 그쪽 스캔 정확도도 같이 올라간다.\n",
                    now_ms(), cps, a, real_db, cps_off, db,
                    if real_db >= db { "+" } else { "−" },
                    if real_db >= db { real_db - db } else { db - real_db });
                if let Some(dd) = mod_dir() { let _ = fs::create_dir_all(&dd); let _ = fs::write(dd.join("item_tactics_netfind.txt"), s); }
                write_registry_status(db);
                return;
            }
            a += 8;
        }
        } // #[cfg(any())] 폐기 블록 끝
    }
}

static NETSCAN_TRIES: AtomicU64 = AtomicU64::new(0);   // 윈도우 전범위 스캔 시도 수(상한 64)
static REG_LAST_SIG: AtomicU64 = AtomicU64::new(u64::MAX); // 마지막으로 기록한 상태 지문(무변경 write 억제)
fn write_registry_status(db: usize) {
    // ★★진단 산출물(LOG_ENABLED 무관 · **상태 변화 시에만** write) — 2026-08-06 신설, 프로덕션 상주.
    //   경위: 프로덕션은 `LOG_ENABLED=false` 라 `dump_mod_items` 가 실패해도 파일이 하나도 안 남는다.
    //   그런데 이 함수는 **실패해도 `MODITEMS_DONE` 을 세워 재시도하지 않으므로**(dump_mod_items L1)
    //   이 1회 결과가 곧 그 세션의 결론이고, 레지스트리가 비면 **모드템 지정이 전부 조용히 무시**된다.
    //   같은 함정을 2026-07-25 제보("모드템이 드롭다운에 안 뜸") 때 원격진단 불가로 이미 겪었다.
    //   ⟹ 이 한 줄 요약만은 항상 남긴다. 비용 = 세션당 write 1회.
    {
        let reg = MOD_REGISTRY.lock().unwrap_or_else(|e| e.into_inner()).len();
        let fin = MOD_FINALS.lock().unwrap_or_else(|e| e.into_inner()).len();
        let (act_n, act_tot) = { let a = MOD_ACTIVE.lock().unwrap_or_else(|e| e.into_inner());
                                 (a.iter().filter(|&&x| x).count(), a.len()) };
        let net = ITEM_NET_ADDR.load(Ordering::Relaxed);
        let tries = NETSCAN_TRIES.load(Ordering::Relaxed);
        // 상태 지문 — 바뀐 게 없으면 write 하지 않는다(관리틱마다 도는 자리라 무조건 write 는 낭비).
        // ⚠★08-07 3차 수정 — **지문에 런타임 카운터를 포함**시킨다.
        //   이 파일이 `probe_db`(= on_server_start + 관리틱)에서만 쓰이는데, 경기 중에 변하는 값
        //   (게이트 카운터·Game 핸들)을 지문에 안 넣어서 **경기 시작 전 스냅샷이 그대로 굳었다.**
        //   같은 "진단이 너무 이른 시점에 고정" 실수를 이 세션에서 세 번 했다(레지스트리 0개·itemnet 미발견·이번).
        //   ⟹ 지문에 넣고, 아래 `registry_tick()` 으로 **경기 중에도 갱신**한다.
        let sig = (reg as u64) ^ ((fin as u64) << 12) ^ ((act_n as u64) << 24) ^ net.rotate_left(17)
                  ^ GATE_AGREE.load(Ordering::Relaxed).min(1).rotate_left(33)
                  ^ GATE_DIFF.load(Ordering::Relaxed).min(1).rotate_left(35)
                  ^ LIVE_GAME.load(Ordering::Relaxed).rotate_left(7)
                  ^ BUY_GAME.load(Ordering::Relaxed).rotate_left(11)
                  ^ if net == 0 { (tries.min(64) << 40) ^ (NETWIDE_TRIES.load(Ordering::Relaxed).min(9) << 52) } else { 0 }
                  // ★08-12 인게임결함 진단 추가: GV 훅 발화 여부가 slot3 표시 결함 판별의 핵심인데
                  //   어떤 파일에도 안 남고 있었다 — 발화 시작/아이콘 세팅 시작 시 재기록되도록 지문에 포함.
                  ^ GV_HITS.load(Ordering::Relaxed).min(1).rotate_left(45)
                  ^ SLOT3_ICON_N.load(Ordering::Relaxed).min(1).rotate_left(47);
        if REG_LAST_SIG.swap(sig, Ordering::Relaxed) == sig { return; }
        let mut s = format!("[{}ms] item_tactics 레지스트리/신경망 프로브 결과 (상태 변화 시 갱신)\n\n", now_ms());
        s.push_str(&format!("  db base(구·역산) = {:#x}  (= &champion_patch_statistics − {CPS_OFF:#x})\n", db));
        {   // ★08-07: 하드코딩 없이 직접 얻은 Database base 와 대조 — CPS_OFF 유효성 판정
            let dd = DB_DIRECT.load(Ordering::Relaxed) as usize;
            let cp = SERVER_CPS.load(Ordering::Relaxed) as usize;
            if dd != 0 {
                s.push_str(&format!("  db base(신·직접) = {:#x}  (= addr_of!(*ctx.database))\n", dd));
                if cp != 0 {
                    let real_off = cp.wrapping_sub(dd);
                    s.push_str(&format!("  ★실제 cps 오프셋 = {:#x}   (하드코딩 = {CPS_OFF:#x} → {})\n",
                        real_off, if real_off == CPS_OFF { "일치 = CPS_OFF 유효" } else { "★불일치 = CPS_OFF 가 STALE" }));
                }
                s.push_str(&format!("  두 base 차이     = {}{:#x}\n",
                    if dd >= db { "+" } else { "−" }, if dd >= db { dd - db } else { db - dd }));
            }
        }
        // ★08-12 인게임결함 진단(0.5.5 slot3 미표시): GV_UPDATE 훅 상태를 상시 노출.
        //   설치(1=성공/2=실패) · 발화수 · PV잡힘 · 아이콘 세팅수 — 발화=0이면 RVA_GV_UPDATE 오답이 원인.
        s.push_str(&format!("  [slot3진단] GV훅 설치={} 발화={} PV잡힘={} 아이콘세팅={} miss={}\n",
            GV_HOOK_INSTALLED.load(Ordering::Relaxed), GV_HITS.load(Ordering::Relaxed),
            SLOT3_PV_N.load(Ordering::Relaxed), SLOT3_ICON_N.load(Ordering::Relaxed),
            SLOT3_ICON_MISS.load(Ordering::Relaxed)));
        s.push_str(&format!("  MOD_REGISTRY  = {}개   {}\n", reg,
            if reg == 0 { "★FAIL — 모드템 지정이 전부 미적용된다(SEL_PENDING 행). 상세=item_tactics_moditems.txt(LOG_ENABLED 필요)" } else { "OK" }));
        s.push_str(&format!("  MOD_ACTIVE    = {}/{} 활성\n", act_n, act_tot));
        s.push_str(&format!("  MOD_FINALS    = {}개   {}\n", fin,
            if fin == 0 { "★FAIL — 드롭다운 모드템 목록이 비고 지정 해석 불가" } else { "OK" }));
        // ⚠★판독 주의(08-06 오보 방지): net 은 **관리틱마다 재시도**된다. `0x0` 은 "그 시점까지 못 찾음"이지
        //   "끝내 실패"가 아니다. 윈도우 스캔 상한(64회)을 다 쓴 뒤에도 0 이어야 비로소 FAIL 로 읽을 것.
        s.push_str(&format!("  ITEM_NET_ADDR = {:#x}  {}\n", net,
            if net != 0 { "OK".to_string() }
            else if tries < 64 { format!("아직 못 찾음(윈도우 스캔 {}/64회 — 재시도 중, FAIL 아님)", tries) }
            else { "★FAIL — 스캔 상한 소진. 미지정 슬롯의 4번째가 바닐라 FNV 폴백으로 떨어진다".to_string() }));
        // ★★08-07 신설 — **버킷 확장 감지.** 2026-08-06~07 에 이 한 줄이 없어서 반나절을 태웠다.
        //   `tfm2_itemnet_tune` 의 `itemnet_expand.py` 가 세이브의 해시 버킷을 16384 → 524288 로
        //   무손실 확장하면(`w_new[j]=w_old[j%16384]`, 세이브에 직렬화 = 커리어 영구), 헤더가 더 이상
        //   16384 가 아니라 **우리 시그 체크(`==16384`)가 조용히 실패**한다. net 은 멀쩡히 살아 있는데.
        //   → 그 상태를 "0.5.4 마이그 누락"으로 오진했다. 이제 헤더를 읽어 그 사실을 직접 말하게 한다.
        //   (정본 = `REPORT\tfm2_itemnet_tune\03_시행착오.md` 모드 상호작용 표)
        if net == 0 {
            let dd = DB_DIRECT.load(Ordering::Relaxed) as usize;
            let a = if dd != 0 { dd + 0x1558 } else { db + 0x1558 };
            if unsafe { readable(a, 0x20) } {
                let (n0, n1, n3) = unsafe { (rd_u64(a), rd_u64(a + 0x10), rd_u64(a + 0x18)) };
                if n0 == n1 && n3 == 1 && n0 != 16384 && n0 >= 1024 && n0 <= (1 << 22) {
                    s.push_str(&format!(
                        "  ⚠★버킷 확장 감지: +0x1558 헤더 = {} (바닐라 16384) — net 은 **살아 있고** 위치도 맞다.\n\
                         \x20    원인 = `tfm2_itemnet_tune` 의 버킷 확장(세이브에 직렬화 = 그 커리어 영구).\n\
                         \x20    AUTO4 는 16384 세이브에서만 동작한다 ⟹ 되살리려면 itemnet_tune 비활성 + **새 세이브**.\n", n0));
                }
            }
        }
        // ★★08-06 4차 — **추론 중단, 원시값 직접 표시.** 이 건에서 "오프셋이 죽었다"→"안 죽었다"→
        //   "base 가 틀렸다" 로 판정을 세 번 뒤집었다. 전부 간접 증거(스캔 실패)만 보고 내린 결론이었다.
        //   ⟹ 후보 자리의 **실제 메모리 내용**과 **재시도가 실제로 돌았는지**를 같이 찍어 한 번에 가른다.
        //     · 값이 그럴듯한 다른 데이터 = base 가 틀림   · 전부 0 = 망 미초기화(새 커리어)
        //     · 읽기실패 = 그 주소가 매핑조차 안 됨       · retry=0 = 재시도 자체가 안 돎(내 배선 문제)
        // ★08-07 전환 안전성: scene side 게이트 ↔ 구 athlete_id 게이트가 갈린 횟수. **DIFF 는 0이어야 한다.**
        s.push_str(&format!("\n  [팀게이트 전환] scene==athlete_id 일치={} · ★불일치={} (0이어야 정상)\n",
            GATE_AGREE.load(Ordering::Relaxed), GATE_DIFF.load(Ordering::Relaxed)));
        s.push_str(&format!("  [Game 핸들] launcher rcx={:#x} · buy Game={:#x}\n",
            LIVE_GAME.load(Ordering::Relaxed), BUY_GAME.load(Ordering::Relaxed)));
        // ★★08-07 신설 — **제보자 자가진단 블록.** 재현 못 하는 제보를 "이 파일 보내주세요"로 끝내기 위한 것.
        //   판독 규칙을 파일 자체에 박아둔다(보내는 사람도, 받는 사람도 해석이 필요 없게).
        {
            let pid = PLAYER_TEAM_ID.load(Ordering::Relaxed);
            let src = match PID_SRC.load(Ordering::Relaxed) { 1 => "InGame", 2 => "server_state(관리틱)", _ => "★미확보" };
            let n_my = MY_ATH_N.load(Ordering::Relaxed);
            let blocked = GATE_NONE.load(Ordering::Relaxed);
            s.push_str("\n  ═══ 팀 판정 자가진단 (아이템 주입 실패 제보 시 이 블록을 보내면 됩니다) ═══\n");
            s.push_str(&format!("  pid = {}  (출처 {})\n",
                if pid == u64::MAX { "미확보".to_string() } else { pid.to_string() }, src));
            s.push_str(&format!("  내 팀 선발 로스터 = {}명   {}\n", n_my,
                if n_my == 0 { "★FAIL — 게시 안 됨. 배경 sim(일정 넘김) 주입이 전량 스킵된다" } else { "OK" }));
            s.push_str(&format!("  게이트 통과: scene(화면경기)={} · roster(배경sim)={} · side전파 구제={}\n",
                GATE_SCENE.load(Ordering::Relaxed), GATE_ROSTER.load(Ordering::Relaxed), MYSIDE_HIT.load(Ordering::Relaxed)));
            s.push_str(&format!("  ★판정불가로 막힌 지정챔프 = {}   {}\n", blocked,
                if blocked == 0 { "OK — 판정 못 해 스킵된 적 없음" } else { "★이게 주입 실패의 직접 원인이다" }));
            s.push_str(&format!("  (참고) 확정 타팀이라 막힘 = {}   ← 정상 동작, 결함 아님\n",
                GATE_BLOCK_OK.load(Ordering::Relaxed)));
            s.push_str(&format!("  게이트 교차검증(1/256): 일치={} · ★불일치={}\n",
                GATE_AGREE.load(Ordering::Relaxed), GATE_DIFF.load(Ordering::Relaxed)));
            {   // ★08-08 실패 사유 — 새 세이브는 **저장 후 재시작**해야 모드템이 DB 에 병합된다(실측).
                let w = MODITEMS_FAIL_WHY.load(Ordering::Relaxed);
                if w != 0 { s.push_str(&format!("  ★레지스트리 실패 사유 = {}
",
                    if w == 1 { "비바닐라 item-struct 배열 못 찾음 — ★새 세이브는 저장 후 게임 재시작 필요" }
                    else { "next_tier 가진 배열 없음 — 아이템 모드 미로드/미인식" })); } }
            s.push_str(&format!("  레지스트리 스캔 재시도={}회 · 스캔 음성캐시 정리={}건\n",
                MODITEMS_TRIES.load(Ordering::Relaxed), SCAN_NEG_PURGE.load(Ordering::Relaxed)));
            s.push_str("  판독: pid 미확보 또는 로스터 0명 → 구멍1 / **판정불가** 막힘>0 → 구멍2 계열\n");
            s.push_str("        (참고) 줄은 적팀 정상 차단이라 값이 커도 무시할 것\n");
            s.push_str("        MOD_REGISTRY 0 → 구멍3 / 재시도>0 이면 첫 스캔이 실패했다는 뜻\n");
        }
        // ★08-08 — 대회(리그) 배경 디스크립터 보험 관측(관측 전용·게이트 미연결. RE 레시피 런타임 실측 2건용).
        if TN_ENABLED {
            s.push_str(&format!("\n  [대회 디스크립터 보험 (08-08 RE·worker ret 0x239f242·관측 전용)]\n\
                \x20   런처발화={} 스캔성공={} · miss: 프레임={} 스캔={} · 검증③탈락={}\n",
                TN_SEEN.load(Ordering::Relaxed), TN_HIT.load(Ordering::Relaxed),
                TN_MISS_FRAME.load(Ordering::Relaxed),
                TN_MISS_SCAN.load(Ordering::Relaxed), TN_V3_NG.load(Ordering::Relaxed)));
            s.push_str(&format!("\x20   db관측(검증①→관측 강등): seen={:#x} · ==LIVE_DB {}회 · ==DB_DIRECT {}회 · 둘다아님 {}회\n\
                \x20     (참조: LIVE_DB={:#x} DB_DIRECT={:#x} — seen 의 정체 확정 후 ① 복원 여부 결정)\n",
                TN_DB_SEEN.load(Ordering::Relaxed), TN_DB_EQ_LIVE.load(Ordering::Relaxed),
                TN_DB_EQ_DIRECT.load(Ordering::Relaxed), TN_MISS_DB.load(Ordering::Relaxed),
                LIVE_DB.load(Ordering::Relaxed), DB_DIRECT.load(Ordering::Relaxed)));
            s.push_str(&format!("\x20   마지막 히트: 팀A(+0x140)={} 팀B(+0x148)={} → ★side0(blue)={} side1(red)={} · key={} map=+{:#x} set꼬리={:#06x}\n",
                TN_LAST_A.load(Ordering::Relaxed), TN_LAST_B.load(Ordering::Relaxed),
                TN_LAST_S0.load(Ordering::Relaxed), TN_LAST_S1.load(Ordering::Relaxed),
                TN_LAST_KEY.load(Ordering::Relaxed), TN_LAST_MAP.load(Ordering::Relaxed),
                TN_LAST_SB.load(Ordering::Relaxed)));
            s.push_str(&format!("\x20   내팀 매치 히트={}회 (slot={} sidebyte={} 예측side={} seed={:#x}) · ★매핑검증 OK={} NG={} (NG=0 필수)\n",
                TN_MY_N.load(Ordering::Relaxed), TN_MY_SLOT.load(Ordering::Relaxed),
                TN_MY_SB.load(Ordering::Relaxed), TN_MY_PRED.load(Ordering::Relaxed),
                TN_MY_SEED.load(Ordering::Relaxed),
                TN_VOTE_OK.load(Ordering::Relaxed), TN_VOTE_NG.load(Ordering::Relaxed)));
            s.push_str("\x20   side 투표 (slot,sb)→ath_side0:side1 = ");
            for sl in 0..2usize { for sb in 0..2usize {
                s.push_str(&format!("({},{})={}:{}  ", sl, sb,
                    TN_VOTE[sl * 4 + sb * 2].load(Ordering::Relaxed),
                    TN_VOTE[sl * 4 + sb * 2 + 1].load(Ordering::Relaxed)));
            }}
            s.push_str("\n\x20   판독: ①스캔성공+miss스캔 합 = 런처발화여야 정상. ★miss스캔>0 = set_end 슬롯 무효 경로 실재\n\
                \x20         ②투표가 (slot,sb) 조합별로 한쪽 side 에 쏠리면 = +0x140/+0x148 의 side 대응 확정 근거\n");
            // ★v2.8.0 — TN 게이트(연결됨) 지표
            s.push_str(&format!("\x20   ★TN 게이트(v2.8.0 연결): 조기탈출 구제={} · is_player 승인={} · 내매치아님 확정조회={}\n\
                \x20     판독: 구제/승인 = aid 멤버십이 놓친 내 선수를 TN 이 잡아 주입한 수(0이어도 정상 — 멤버십이 다 잡으면 TN 차례가 없다)\n\
                \x20           NG(위 매핑검증)>0 이면 TN 게이트를 의심할 것. 내매치아님은 관측 전용(차단엔 미사용)\n",
                TN_GATE_EARLY.load(Ordering::Relaxed), TN_GATE_HIT.load(Ordering::Relaxed),
                TN_GATE_NEG.load(Ordering::Relaxed)));
        }
        s.push_str(&format!("\n  [원시값] retry호출={} 광역스캔={}회(상한8)\n",
            NETRETRY_N.load(Ordering::Relaxed), NETWIDE_TRIES.load(Ordering::Relaxed)));
        for (tag, base) in [("신", DB_DIRECT.load(Ordering::Relaxed) as usize), ("구", db)] {
            if base == 0 { continue; }
            for &off in &[0x1558usize, 0xd30, 0xda0] {
                let a = base + off;
                // ⛔`readable()` 선행 필수 — 이걸 빼고 바로 읽었다가 가드페이지 크래시를 냈다(net_sig_at 주석 참조).
                if !unsafe { readable(a, 0x20) } {
                    s.push_str(&format!("    [{}]base+{:#06x}: 매핑없음/보호됨(readable=false)\n", tag, off));
                    continue;
                }
                let f = |o: usize| -> String { format!("{:#x}", unsafe { rd_u64(a + o) }) };
                s.push_str(&format!("    [{}]base+{:#06x}: [0]={} [8]={} [0x10]={} [0x18]={}   (기대 16384 / ptr / 16384 / 1)\n",
                    tag, off, f(0), f(8), f(0x10), f(0x18)));
            }
        }
        s.push_str(&format!("\n  ※net 오프셋은 0.5.4 에서도 GameData+0x1558 로 불변임을 exe 로 확인했다\n\
                               \x20   (0.5.3 `lea rsi,[r13+0x1558]` @0x182d4be ↔ 0.5.4 `lea r15,[r13+0x1558]` @0x2123733,\n\
                               \x20    둘 다 beam(0.5.3 0x10591f0 / 0.5.4 0x145b090) 의 net 인자).\n\
                               \x20   따라서 0 이 지속되면 의심할 곳은 **오프셋이 아니라 db base(0x16698) 또는 프로브 시점**이다.\n"));
        if let Some(d) = mod_dir() { let _ = fs::create_dir_all(&d); let _ = fs::write(d.join("item_tactics_registry.txt"), s); }
    }
}

// ══ athlete→champion 매핑 프로브 (buy_item r8=athlete 스캔) ══════════════════
const RVA_BUY_ITEM: usize = 0xdf5490; // 0.5.6(구0.5.5=0xeb2c40, skel UNIQUE·BYTE=SAME·size 230·진입부 cmp[r8+0x4f0],0 확증=athlete build len 불변). resolver 0xebcb10을 호출. // 0.5.5(구0.5.4=0xe767e0, exe2exe 스켈레톤 확정). resolver 0xeb2d30을 직접 호출. // 0.5.4(구0.5.3=0xd0c680). 크기 0xe6·명령 67개 라인단위 동일(유일차 = athlete build len 0x4a0->0x490) + vtable 썽크 바이트형태 exe 전체 유일 1히트. // 0.5.3(구0.5.2=0x211e070). **진입 24B 바이트 완전동일**(exe 전체 유일 1히트) + 본체 명령 대 명령 동형 + 인자계약 유지(r8=athlete·[rsp_entry+0x30]=Game·Game+0x30=catalog). orig_len=19도 그대로(11B<12B → 다음 클린경계 mov rax,[rsp+0xa8] 8B). ⚠0.5.3 변화: 호출 경로가 direct call → vtable(+0x78) 썽크 0xd22340 경유로 바뀌었으나 **함수 진입부 훅이라 전 호출 포착됨**. ↓이하 0.5.2 이력. (구0.5.1=0x1f01090, exe2exe 스켈레톤 UNIQUE·프롤로그 24B 완전동일=본체 무변경, delta +0x21cfe0). ↓이하 0.5.1 이력. 함수 대개편(8push/sub0x38→5push/sub0x50, build/이름비교가 서브함수 0x1f00920로 분리)으로 mask-sig NONE이었으나 인자계약 불변(r8=athlete, p6=Game@rsp_entry+0x30, Game+0x30=catalog)로 확정. buy 드라이버 FUN_142234430(구 FUN_1420e76e0 후계)+vtable슬롯 교차검증.
const BUY_PROLOGUE: [u8; 12] = [0x41,0x57, 0x41,0x56, 0x56, 0x57, 0x53, 0x48,0x83,0xEC,0x50, 0x48]; // 0.5.1 신 프롤로그 첫12B: push r15/r14/rsi/rdi/rbx; sub rsp,0x50; (11B=클린경계) + mov(0x48…) 첫바이트. 트램폴린 재배치=19B(다음 클린경계=+mov rax,[rsp+0xa8])
static BUY_PROBE_INSTALLED: AtomicU64 = AtomicU64::new(0);
static CHAMP_SCAN: Mutex<Vec<u64>> = Mutex::new(Vec::new());
static SCAN_DIAG_DONE: AtomicBool = AtomicBool::new(false); // ★0.5.1 scan 진단 1회 게이트

// install_detour(트램폴린): saved = push rcx rdx r8 r9 r10 r11 → r8=saved.add(3). cap_fn(rcx=saved, rdx=entry_rsp).
unsafe fn install_detour(rva: usize, orig_len: usize, cap_fn: usize) -> Result<usize, &'static str> {
    let mbase = exe_base_addr();
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, orig_len + 4) { return Err("fn unreadable"); }
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);
    s.extend_from_slice(&[0x51,0x52,0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]);
    s.extend_from_slice(&[0x48,0x89,0xe1]);
    s.extend_from_slice(&[0x4c,0x89,0xd2]);
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff,0xd0]);
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]);
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes());
    s.extend_from_slice(&[0xff,0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(stub)
}

#[inline] unsafe fn rd_u64(p: usize) -> u64 { std::ptr::read_unaligned(p as *const u64) }
#[inline] unsafe fn wr_u64(p: usize, v: u64) { std::ptr::write_unaligned(p as *mut u64, v); }

// ── 아이템 신경망 forward 직접 호출 (scrim 검증본 이식) ──
//   forward(net, ctx=&[u64;11], build_ptr, build_len, flag=0) → f32 sigmoid 점수.
//   ctx: [0..5]=우리팀 champ id / [5..10]=상대 / [10]=포지션(0~4, >4면 forward 패닉).
const ITEMNET_FORWARD_RVA: usize = 0x17f09b0; // 0.5.6(구0.5.5=0x12624f0, skel UNIQUE·BYTE=SAME·size 1609·net 레이아웃 불변). // 0.5.5(구0.5.4=0x145a680, exe2exe 스켈레톤 확정). // 0.5.4(구0.5.3=0x10587e0). 피처 문자열 4종 완전일치 + 크기 0x649·명령 409 동일 + 본문 100% 동일(net 레이아웃 불변). // 0.5.3(구0.5.2=0x1b9cce0). 진입 24B 완전동일 + 피처명 문자열 5종 일치(self_item/champ_pos_build/lane_counter/synergy/global_counter) + net 레이아웃 불변(net+0x8=가중치ptr, +0x10=16384 바운드, +0x18=1) ⟹ 모드의 매호출 재검증 로직 그대로 유효. ↓이하 0.5.2 이력. (구0.5.1=0x1bc82e0, exe2exe UNIQUE·프롤로그 동일). ↓이하 0.5.1 이력: (구0.5.0_3=0x1b78420, mask-sig UNIQUE PROL-OK push8 554157415641554154565753). ⚠AUTO4_FORWARD_SCORE=false로 OFF(0.5.1서 forward 내부 +0x44a AV, 위 플래그 주석 참조). 프롤로그 매치≠내부동작 동일.
type ItemNetFn = unsafe extern "C" fn(usize, usize, *const u64, u64, u8) -> f32;
static ITEM_NET_ADDR: AtomicU64 = AtomicU64::new(0);
static ITEMNET_VALID: AtomicU64 = AtomicU64::new(0); // 0=미확인,1=유효,2=무효
unsafe fn itemnet_addr_valid() -> bool {
    match ITEMNET_VALID.load(Ordering::Relaxed) { 1 => return true, 2 => return false, _ => {} }
    let fa = exe_base_addr() + ITEMNET_FORWARD_RVA;
    let expect = [0x55u8, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53]; // push8
    let mut ok = readable(fa, 12);
    if ok { for i in 0..12 { if *((fa + i) as *const u8) != expect[i] { ok = false; break; } } }
    ITEMNET_VALID.store(if ok { 1 } else { 2 }, Ordering::Relaxed);
    ok
}
unsafe fn itemnet_forward(net: usize, ctx: &[u64; 11], build: &[u64]) -> f32 {
    // 관전경기 AUTO4 채점에서만 도달(배경 sim 아님) → 빈도 낮아 전역 원자 카운터로 충분.
    let __it = perf::tsc();
    if net == 0 || !itemnet_addr_valid() { perf::rec(perf::S_ITEMNET, __it); return f32::MIN; }
    // ★★per-call 가중치 재검증(07-17 크래시 근절): net은 detection 시 1회만 sig_ok 했으나, net 내부 가중치ptr(net+0x8)이
    //   세션전환/배경sim 재로드로 stale해지면 forward 내부(+0x81)서 그 stale ptr deref → AV(0xc0000005). 매 호출 직전
    //   헤더(16384/16384/1)+가중치ptr readable을 재확인 → stale이면 호출 스킵(f32::MIN=후보탈락→폴백). shadow-call 크래시조건 차단.
    if !(readable(net, 0x20) && rd_u64(net) == 16384 && rd_u64(net + 0x10) == 16384 && rd_u64(net + 0x18) == 1
        && { let w = rd_u64(net + 0x8) as usize; w >= 0x10000 && readable(w, 16384 * 4) }) {
        AUTO4_NET_STALE.fetch_add(1, Ordering::Relaxed);
        perf::rec(perf::S_ITEMNET, __it);
        return f32::MIN;
    }
    let func: ItemNetFn = core::mem::transmute(exe_base_addr() + ITEMNET_FORWARD_RVA);
    let out = func(net, ctx.as_ptr() as usize, build.as_ptr(), build.len() as u64, 0);
    perf::rec(perf::S_ITEMNET, __it); // ★게임함수 shadow-CALL 실비용 포함(이 사이트가 곧 그 비용)
    out
}
static AUTO4_NET_STALE: AtomicU64 = AtomicU64::new(0); // ★per-call net stale 감지(스킵)수
const CHAMP_SHEET: [&str; 61] = [
    "swordman","monk","mod_champions","fighter","knight","archer","soldier","priest","pythoness",
    "pyromancer","ice_mage","ninja","magic_knight","berserker","executioner","lancer","ogre",
    "dual_blader","cavalry_knight","gunner","pole_warrior","jiangshi","gambler","hammerer","demon",
    "vampire","spirit_caller","boomerang_hunter","inquisitor","shield_bearer","whip_master","werewolf",
    "dokkaebi","necromancer","bard","barrier_magician","chef","clown","dancer","dark_mage","exorcist",
    "ghost","illusionist","lightning_mage","plague_doctor","poison_dart_hunter","shadowmancer","taoist",
    "siege_breaker","android","druid","prisoner","bomber","voodoo_shaman","white_mage","wind_mage",
    "enchanter","hitman","guardian_spirit","hunter","circus_blade",
];
fn champ_id_of(name: &str) -> Option<usize> { CHAMP_SHEET.iter().position(|&c| c == name) }
const SHADOW_CALL_NAMES: bool = true; // ctx+0x20 element 이름 = vtable[0x50] 호출(AV위험, 게이트)
static MAX_OWNED4: AtomicU64 = AtomicU64::new(0);
static BUY4_LOGGED: AtomicBool = AtomicBool::new(false);
static CHAMP_AT3: Mutex<Vec<String>> = Mutex::new(Vec::new()); // 진단: owned==3 도달 챔프
static CHAMP_AT4: Mutex<Vec<String>> = Mutex::new(Vec::new()); // 진단: owned>=4 도달 챔프(4번째 아이템 티어/가격)
static BUILD3_AT: Mutex<Vec<String>> = Mutex::new(Vec::new()); // 진단: build[3] 목표 소스(neural/manual/vanilla) 챔프별 1회
// 아이템 게임 id → 이름 키 (0~29=바닐라, 30+=모드템). ctx+0x20 컬렉션 이름 스캔용.
fn item_id_to_key(id: u64) -> Option<String> {
    if (id as usize) < VANILLA_KEYS.len() { return Some(VANILLA_KEYS[id as usize].to_string()); }
    let reg = MOD_REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    reg.get((id as usize).checked_sub(30)?).cloned()
}
// ★ AUTO 4번째 = forward로 최고점 최종템 선택(신경망 추천). false=beam4 캡처만.
// ★0.5.0: ON — 로스터 오프셋 RE 확정(SimState+0x840 stride0x8d0, team@+0x820, pos@+0x8b0, champ@+0x420,
//   net@Database+0xda0). compute_auto_4th_id/build_lineup_ctx 재가동 = 신경망 자동 4번째 선택.
const AUTO4_FORWARD_SCORE: bool = true; // ★재활성(07-17): 크래시원인=net+0x8 가중치ptr이 detection후 stale(세션전환)인데 재검증 안 함. itemnet_forward에 매호출 net+0x8 readable 재검증 추가로 stale시 스킵→폴백(크래시無). 기능 유지 + 크래시조건 차단. ~~false(shadow-call 폐기 시도)~~
// ★0.5.0 build-extension: RVA_REALLOC(0x25a56c0 실함수) 확정 → ON. buy build Vec 3→4 실구매 재가동.
const BUILD_EXTEND_ENABLED: bool = true;
// ★0.5.0 ui_inject(#item3 드롭다운 + #slot3 노드): 로더훅 RVA(LOADER 0x4d8fb0/PARSER 0x2493b90/
//   ALLOC 0x25a5620) 확정 → ON. 전략화면 4번째 드롭다운/경기중 slot3 노드 주입 재가동.
const UI_INJECT_ENABLED: bool = true;   // ★0.5.0 수정: player_info/wide .ui를 0.5.0기반+4슬롯으로 재작성 → 재활성화(격리 테스트)
// ★성능 캐시(0.5.1): compute_auto_4th_id 결과. 키=(champ, build3, 라인업ctx). 같은 경기·챔프·build면
//   신경망 4번째 추천이 항상 동일 → owned==3 반복 buy에서 51-forward 재계산 제거(돈 적을때 부하 급증 완화).
//   ctx를 키에 포함하므로 "라인업 무시=오답" 우려 없음. 병렬 경기는 ctx가 달라 키 충돌 없음.
static AUTO4_RESULT: Mutex<Option<HashMap<(String, u64, u64, u64, [u64; 11]), Option<u64>>>> = Mutex::new(None);
static AUTO_CANDS: Mutex<Option<std::sync::Arc<Vec<u64>>>> = Mutex::new(None);
fn auto_cands() -> std::sync::Arc<Vec<u64>> {
    {
        let g = AUTO_CANDS.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(v) = g.as_ref() { return v.clone(); } // Arc clone = 참조카운트만(데이터 복사 없음)
    }
    let mut v: Vec<u64> = VANILLA_FINAL.to_vec();
    for (id, _) in mod_final_opts() { v.push(id); }
    let arc = std::sync::Arc::new(v);
    *AUTO_CANDS.lock().unwrap_or_else(|e| e.into_inner()) = Some(arc.clone());
    arc
}

// ===========================================================================
//  아이템 빌드 덤프 (측정 전용 · 파일 트리거 · 평소 완전 비활성)
// ===========================================================================
// 목적 = 학습된 신경망이 "지금" 각 챔피언에게 어떤 4칸 빌드를 추천하는지 그대로 뽑아낸다.
//   경기 기록을 집계하는 방식(표본이 적으면 안 보임)과 달리, **추천망을 직접 채점**하므로
//   시즌 1회만 돌려도 완전한 순위가 나온다. A/B 비교(모드 OFF 시즌 ↔ ON 시즌)용.
//
// 쓰는 법: 모드 폴더에 `dump_builds.trigger` 파일을 아무 내용으로 만들어 두고 **관리 화면으로
//   진입**하면(=관리틱 1회) 실행되고, 끝나면 트리거 파일을 지운다. 결과 = `item_builds_<ms>.csv`.
//   ⚠경기 중이 아니라 관리 화면에서 돌린다(관리틱은 sim 중엔 안 돈다 = 레이스 없음).
//
// ⚠★노이즈를 반드시 끄고 돌릴 것 — forward 안에는 탐색 노이즈(U[0,1)*0.2−0.1 = ±0.1)가 있어
//   그냥 돌리면 점수가 매번 흔들린다. `tfm2_itemnet_tune` 의 cfg 에서
//   `noise_range = 0` / `noise_offset = 0` 으로 두면 결정론이 된다.
//   (아래 자가진단이 같은 빌드를 두 번 채점해 흔들리면 CSV 머리말에 경고를 박는다.)
//
// ⚠모드 챔피언은 나오지 않는다 — CHAMP_SHEET(시트 인덱스 0~60)에 없어 cid 를 못 만든다.
//   (쓰레기 ctx 로 forward 하면 전원 동일 답이 나오므로 아예 제외한다.)
const DUMP_TRIGGER: &str = "dump_builds.trigger";
const DUMP_BEAM_WIDTH: usize = 32;
/// 완전탐색 총 forward 호출 상한. 넘으면 beam 으로 폴백하고 **CSV 에 그 사실을 명시**한다.
const DUMP_MAX_EXHAUSTIVE: u64 = 1_500_000;
static DUMP_RUNNING: AtomicBool = AtomicBool::new(false);

/// 아이템 라벨. 모드템은 실제 키, 바닐라는 `v<카테고리>_t<티어>`(id = cat*5 + tier).
fn dump_item_label(id: u64, modmap: &HashMap<u64, String>) -> String {
    if let Some(n) = modmap.get(&id) { return n.clone(); }
    if id < 30 { format!("v{}_t{}", id / 5, id % 5) } else { format!("id{}", id) }
}

/// 상위 3개만 유지하는 삽입 정렬.
#[inline]
fn dump_push_top3(top: &mut Vec<(f32, Vec<u64>)>, s: f32, b: Vec<u64>) {
    if top.len() == 3 && s <= top[2].0 { return; }
    let at = top.iter().position(|(t, _)| s > *t).unwrap_or(top.len());
    top.insert(at, (s, b));
    top.truncate(3);
}

/// 한 (챔프, 포지션) 에 대해 4칸 빌드 상위 3개를 구한다.
/// 후보가 적으면 완전탐색, 많으면 beam(깊이 4). 반환 = (top3, forward 호출수).
unsafe fn dump_top3_for(net: usize, ctx: &[u64; 11], cands: &[u64], exhaustive: bool)
    -> (Vec<(f32, Vec<u64>)>, u64)
{
    let n = cands.len();
    let mut top: Vec<(f32, Vec<u64>)> = Vec::with_capacity(4);
    let mut calls = 0u64;
    if exhaustive {
        for i in 0..n { for j in (i + 1)..n { for k in (j + 1)..n { for l in (k + 1)..n {
            let b = vec![cands[i], cands[j], cands[k], cands[l]];
            let s = itemnet_forward(net, ctx, &b);
            calls += 1;
            if s == f32::MIN { continue; } // net stale = 스킵
            dump_push_top3(&mut top, s, b);
        }}}}
    } else {
        // beam: 빈 빌드에서 시작해 한 칸씩 늘린다(게임 beam 과 같은 모양, 깊이만 4).
        let mut beam: Vec<Vec<u64>> = vec![Vec::new()];
        for depth in 0..4 {
            let mut next: Vec<(f32, Vec<u64>)> = Vec::new();
            for e in beam.iter() {
                for &c in cands.iter() {
                    if e.contains(&c) { continue; }
                    // 조합 중복 방지 = 오름차순으로만 확장
                    if let Some(&last) = e.last() { if c <= last { continue; } }
                    let mut b = e.clone();
                    b.push(c);
                    let s = itemnet_forward(net, ctx, &b);
                    calls += 1;
                    if s == f32::MIN { continue; }
                    next.push((s, b));
                }
            }
            next.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
            next.truncate(DUMP_BEAM_WIDTH);
            if depth == 3 { for (s, b) in next.iter() { dump_push_top3(&mut top, *s, b.clone()); } }
            beam = next.into_iter().map(|(_, b)| b).collect();
            if beam.is_empty() { break; }
        }
    }
    (top, calls)
}

/// 트리거 파일이 있으면 덤프를 1회 수행한다. 관리틱(=sim 미가동)에서만 호출할 것.
unsafe fn maybe_dump_builds() {
    let dir = match mod_dir() { Some(d) => d, None => return };
    let trig = dir.join(DUMP_TRIGGER);
    if !trig.exists() { return; }
    if DUMP_RUNNING.swap(true, Ordering::SeqCst) { return; } // 재진입 방지
    let t0 = now_ms();

    let net = ITEM_NET_ADDR.load(Ordering::Relaxed) as usize;
    let mut out = String::new();
    let mut header_warn = String::new();

    if net == 0 || !itemnet_addr_valid() {
        let _ = fs::write(dir.join(format!("item_builds_FAILED_{}.txt", t0)),
            format!("net 미확보 (net={:#x}, fwd_valid={}) — 경기를 한 번 치른 뒤 다시 시도하세요.\n",
                    net, itemnet_addr_valid()));
        let _ = fs::remove_file(&trig);
        DUMP_RUNNING.store(false, Ordering::SeqCst);
        return;
    }

    let cands_arc = auto_cands();
    let cands: Vec<u64> = cands_arc.iter().copied().collect();
    let modmap: HashMap<u64, String> = mod_final_opts().into_iter().collect();
    let n = cands.len();
    if n < 4 {
        let _ = fs::write(dir.join(format!("item_builds_FAILED_{}.txt", t0)),
            format!("최종템 후보가 {}개뿐 — 4칸 빌드를 만들 수 없습니다.\n", n));
        let _ = fs::remove_file(&trig);
        DUMP_RUNNING.store(false, Ordering::SeqCst);
        return;
    }

    // ── 노이즈 자가진단: 같은 빌드를 두 번 채점해 값이 흔들리는지 ──
    let mut pctx = [0u64; 11];
    for k in 0..10 { pctx[k] = 9999; }
    pctx[0] = 0; pctx[10] = 0;
    let pb = vec![cands[0], cands[1], cands[2], cands[3]];
    let s1 = itemnet_forward(net, &pctx, &pb);
    let s2 = itemnet_forward(net, &pctx, &pb);
    if (s1 - s2).abs() > 1e-6 {
        header_warn.push_str(&format!(
            "# ⚠ 탐색 노이즈가 켜져 있습니다(같은 빌드 2회 채점 = {} vs {}). 점수·순위가 흔들립니다.\n\
             #   tfm2_itemnet_tune cfg 에서 noise_range=0 / noise_offset=0 으로 두고 다시 뽑으세요.\n",
            s1, s2));
    }

    // ── 완전탐색 가능 여부 ──
    let champs: Vec<usize> = (0..CHAMP_SHEET.len()).filter(|&i| CHAMP_SHEET[i] != "mod_champions").collect();
    let cells = (champs.len() * 5) as u64;
    let combos = { // C(n,4)
        let nn = n as u64;
        if nn < 4 { 0 } else { nn * (nn - 1) * (nn - 2) * (nn - 3) / 24 }
    };
    let exhaustive = combos.saturating_mul(cells) <= DUMP_MAX_EXHAUSTIVE;

    out.push_str(&format!(
        "# tfm2_item_tactics 아이템 빌드 덤프 (신경망 직접 채점)\n\
         # 시각(ms)={} / 후보 최종템={}개(바닐라 {} + 모드 {}) / 챔피언={} / 포지션=5\n\
         # 탐색={} / C(n,4)={} / 셀={}\n\
         # ctx = 본인 챔프만 배치, 나머지 아군·적군 전부 9999(중립). A/B 비교용 고정 컨텍스트.\n\
         # ⚠모드 챔피언은 시트 인덱스가 없어 제외됩니다.\n{}\
         champion,position,rank,score,id0,id1,id2,id3,item0,item1,item2,item3\n",
        t0, n, VANILLA_FINAL.len(), n - VANILLA_FINAL.len(), champs.len(),
        if exhaustive { "완전탐색" } else { "beam(폭 32) — 후보가 많아 완전탐색 상한 초과" },
        combos, cells, header_warn));

    let mut total_calls = 0u64;
    let mut scored_cells = 0u64;
    for &ci in champs.iter() {
        for pos in 0..5usize {
            let mut ctx = [9999u64; 11];
            ctx[ci_slot(pos)] = ci as u64; // 본인 = 아군 슬롯 중 자기 포지션 자리
            ctx[10] = pos as u64;
            let (top, calls) = dump_top3_for(net, &ctx, &cands, exhaustive);
            total_calls += calls;
            if top.is_empty() { continue; }
            scored_cells += 1;
            for (rank, (s, b)) in top.iter().enumerate() {
                out.push_str(&format!("{},{},{},{:.6},{},{},{},{},{},{},{},{}\n",
                    CHAMP_SHEET[ci], pos, rank + 1, s,
                    b[0], b[1], b[2], b[3],
                    dump_item_label(b[0], &modmap), dump_item_label(b[1], &modmap),
                    dump_item_label(b[2], &modmap), dump_item_label(b[3], &modmap)));
            }
        }
    }
    let dt = now_ms().saturating_sub(t0);
    out.push_str(&format!("# 완료: forward {}회 / 채점된 셀 {}/{} / {}ms\n",
                          total_calls, scored_cells, cells, dt));
    let _ = fs::write(dir.join(format!("item_builds_{}.csv", t0)), out);
    let _ = fs::remove_file(&trig);
    append_log("4items.txt", &format!("[{}ms] build dump 완료: forward {}회 {}ms", now_ms(), total_calls, dt));
    DUMP_RUNNING.store(false, Ordering::SeqCst);
}

/// ctx 의 아군 5칸 중 이 포지션이 쓰는 슬롯. (ctx[0..5] = 아군 5포지션, self = ctx[position])
#[inline] fn ci_slot(pos: usize) -> usize { pos.min(4) }

// ── 로스터 배열(SimState+0x840, stride 0x8d0)에서 그 경기 진짜 라인업 ctx 복원 ──
//   athlete = 배열 원소. team=+0x820(0/1), champion name=+0x420. 병렬 경기는 각기 다른 배열이라
//   athlete 포인터가 정확히 한 경기에만 속함 = 충돌 없음(백포인터 불필요, RE 확정).
const ATH_STRIDE: usize = 0x9e0; // 0.5.5(구0.5.4=0x8c0, +0x120). imul r64,r64,imm32 전수: 0.5.4 0x8c0=143건 → 0.5.5 0x9e0=167건.
// athlete 유효성 검사 + (team, champ_id) 반환. 강한 검증(team∈{0,1} + 실챔프명)으로 배열 경계 자동 판정.
unsafe fn athlete_lineup_at(p: usize) -> Option<(u64, u64)> {
    if p < 0x10000 { return None; }
    let team = safe_read_u64(p + 0x930)?;
    if team > 1 { return None; }
    let nptr = safe_read_u64(p + 0x470)? as usize; // 0.5.0 champion name ptr (구 0x398)
    let nlen = safe_read_u64(p + 0x478)? as usize; // 0.5.0 champion name len (구 0x3a0)
    if nptr < 0x10000 || nlen == 0 || nlen > 48 { return None; }
    let mut buf = Vec::new();
    if !safe_read_bytes(nptr, nlen, &mut buf) { return None; }
    let name = String::from_utf8_lossy(&buf).into_owned();
    let cid = champ_id_of(&name)? as u64;
    Some((team, cid))
}
// ★ athlete의 champion name 읽기(**0.5.4: +0x410 ptr / +0x418 len**, ~~0.5.3까지 +0x420/+0x428~~). SEL/PT 매칭용.
unsafe fn ath_champ_name(p: usize) -> Option<String> {
    if p < 0x10000 { return None; }
    let nptr = safe_read_u64(p + 0x470)? as usize;
    let nlen = safe_read_u64(p + 0x478)? as usize;
    if nptr < 0x10000 || nlen == 0 || nlen > 48 { return None; }
    let mut buf = Vec::new();
    if !safe_read_bytes(nptr, nlen, &mut buf) { return None; }
    Some(String::from_utf8_lossy(&buf).into_owned())
}
// ★ athlete 유효성 + (side 0/1, champ name) 반환. ⚠champ_id_of·이름charset 안 씀 = **모드 챔피언 완전 포함**
//   판정 = side(+0x820)∈{0,1} + **position(+0x8b0)∈0..4(구조필터, 챔프종류 무관 = 모드챔프 포함)** + 읽히는 name(len 2~48).
//   ⚠교훈: 이름 charset(구 ascii필터)로 경계판정하면 (1)비식별자 이름 모드챔프 배제 (2)완전제거 시 side만으론
//     인접 구조메모리 오판→경계 과확장→카운트붕괴(전챔프 미주입 회귀). position<5 가 정밀경계+모드챔프 양립.
//   가짜양성은 SEL/PT 멤버십(카운트)에서 어차피 미매칭이라 무해. (build_lineup_ctx도 lane<5 사용.)
unsafe fn ath_side_champ(p: usize) -> Option<(u64, String)> {
    let side = safe_read_u64(p + 0x930)?;
    if side > 1 { return None; }
    let pos = safe_read_u64(p + 0x9c0)? & 0xffff_ffff; // 라인 0~4
    if pos >= 5 { return None; }
    let nm = ath_champ_name(p)?; // len 1..=48, readable
    if nm.len() < 2 { return None; }
    Some((side, nm))
}
// ★★ 결정적 팀게이트(전역 다수결 폐기): 이 athlete가 속한 "그 경기 로스터 배열"만 스캔해
//   player side(0/1)를 즉시·결정적으로 판정. 유저 지정(SEL) 또는 PT 챔프가 더 많은 side = player.
//   둘 다 0(=player 미참가) or 동점이면 None → 주입 안 함(적군 복사·오판 방지).
//   경기별 배열은 독립(백그라운드 병렬 경기 오염 없음, RE 확정) → 전역투표의 startup-gap/오염/반전 문제 원천 제거.
//   base 포인터로 캐시(매 buy 재스캔 방지). 리셋 = before_management_tick 에서 clear.
static SIDE_CACHE: Mutex<Vec<(usize, i8)>> = Mutex::new(Vec::new()); // (roster_base, side: 0/1, -1=none)
unsafe fn player_side_for_match(athlete: usize) -> Option<u64> {
    // 캐시 키용 base(대략 배열 시작): 뒤로 최대 9칸 walk. 캐시 히트용일 뿐, 카운트는 아래 고정윈도우가 담당.
    let mut base = athlete;
    for _ in 0..9 { let c = base.wrapping_sub(ATH_STRIDE); if ath_side_champ(c).is_some() { base = c; } else { break; } }
    // 캐시 조회 — ★결정된 side(0/1)만 캐시. None 은 캐시 안 함(재판정 = self-heal).
    {
        let g = SIDE_CACHE.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(&(_, s)) = g.iter().find(|&&(b, _)| b == base) { if s >= 0 { return Some(s as u64); } }
    }
    // ★athlete 기준 ±9칸 고정 윈도우로 side별 player 지정챔프 카운트(★SEL 전용).
    //   고정 윈도우 = 10인 로스터 전체를 어느 athlete서든 항상 커버 + walk truncation 없음.
    //   ⚠PT_SNAPSHOT은 투표에서 제외(07-10): PT 맵=팀 개인전술 전체 ~52챔프 → 양팀 선수 거의 전부 매칭
    //   → 5:5 동점 → 미판정 → champ_designated 안전망이 적군 지정챔프에도 발동 = "적이 내 전술대로" 원인.
    //   SEL(유저가 실제 지정한 챔프)만 세면 동점 확률 급감 + 적측 오귀속 방지.
    let (mut c0, mut c1) = (0u32, 0u32);
    for k in -9i64..=9 {
        let a = athlete.wrapping_add((k.wrapping_mul(ATH_STRIDE as i64)) as usize);
        if let Some((team, nm)) = ath_side_champ(a) {
            // ★스코프 접두 무시 매칭(2026-07-30): 조합테스트 전용 지정만 있는 챔프도 세야 한다.
            let is_p = with_sel(|m| m.keys().any(|(c, _)| strip_scope(c) == nm.as_str()));
            if is_p { if team == 0 { c0 += 1; } else { c1 += 1; } }
        }
    }
    let side: i8 = if c0 > c1 { 0 } else if c1 > c0 { 1 } else { -1 };
    // ★결정됐을 때만 캐시(bounded). None(미참가/일시글리치)은 캐시 안 함 → 다음 buy 재판정.
    if side >= 0 {
        let mut g = SIDE_CACHE.lock().unwrap_or_else(|e| e.into_inner());
        if !g.iter().any(|&(b, _)| b == base) {
            if g.len() >= 64 { g.remove(0); }
            g.push((base, side));
        }
    }
    if side < 0 { None } else { Some(side as u64) }
}
// buy athlete → (그 경기 진짜 ctx[11], view roster count@+0x848). 포지션 = athlete+0x8b0.
//   view = base−0x840. count==3 = 데모/타이틀 라이브sim(forward 크래시 컨텍스트) 마커.
unsafe fn build_lineup_ctx(p: usize) -> Option<([u64; 11], u64)> {
    let (my_team, _) = athlete_lineup_at(p)?;
    // 배열 경계: p 기준 앞뒤로 stride 스캔(유효 athlete인 동안, 각 방향 ≤9).
    let mut base = p;
    for _ in 0..9 { let c = base.wrapping_sub(ATH_STRIDE); if athlete_lineup_at(c).is_some() { base = c; } else { break; } }
    let mut end = p;
    for _ in 0..9 { let c = end.wrapping_add(ATH_STRIDE); if athlete_lineup_at(c).is_some() { end = c; } else { break; } }
    let mut ctx = [9999u64; 11];
    let mut a = base;
    while a <= end {
        if let Some((team, cid)) = athlete_lineup_at(a) {
            let lane = (safe_read_u64(a + 0x9c0).unwrap_or(9) & 0xffff_ffff) as usize; // 실제 포지션(0~4)
            if lane < 5 {
                if team == my_team { ctx[lane] = cid; } else { ctx[5 + lane] = cid; }
            }
        }
        a = a.wrapping_add(ATH_STRIDE);
    }
    let pos = ((safe_read_u64(p + 0x9c0).unwrap_or(0) & 0xffff_ffff) as usize).min(4);
    ctx[10] = pos as u64; // ctx[pos] = 내 챔프(자기일관)
    let vcount = safe_read_u64(base.wrapping_sub(0x840) + 0x848).unwrap_or(0);
    Some((ctx, vcount))
}
// ★진단(match id / athlete id 신호 규명): view 헤더·매치메타 포인터 deref + athlete 헤더/id 후보 폭넓게 dump.
static DEMO_DUMPED: Mutex<Vec<usize>> = Mutex::new(Vec::new());

// ★진단(LOG 무관, 07-10): auto4 경로 카운터 — 적 4번째 공격력 편향의 실제 원인 규명용.
//   [0]=netfail(포인터/프롤로그) [1]=inputfail(build ptr/값) [2]=heuristic(vcount==3 데모가드)
//   [3]=fwd_ok [4]=fwd_flat(forward 전 후보 점수 동일=ABI/ctx 의심) [5]=fallback③(신경망 None→바닐라폴백)
static AUTO4_CNT: [AtomicU64; 6] = [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];
static AUTO4_SAMPLES: Mutex<Vec<String>> = Mutex::new(Vec::new());
fn auto4_sample(s: String) {
    let mut g = AUTO4_SAMPLES.lock().unwrap_or_else(|e| e.into_inner());
    if g.len() < 12 { g.push(s); }
}
// post_update(메인스레드)서 호출: 카운터 합 변하면 파일 갱신.
fn auto4_diag_flush() {
    static LAST: AtomicU64 = AtomicU64::new(0);
    let c: Vec<u64> = AUTO4_CNT.iter().map(|a| a.load(Ordering::Relaxed)).collect();
    let sum: u64 = c.iter().sum();
    if sum == 0 || LAST.swap(sum, Ordering::Relaxed) == sum { return; }
    let samples = AUTO4_SAMPLES.lock().unwrap_or_else(|e| e.into_inner()).join("\n");
    if let Some(d) = mod_dir() { let _ = fs::create_dir_all(&d);
        let _ = fs::write(d.join("4items_auto4.txt"), format!(
            "netfail={} inputfail={} heuristic(vcount3)={} fwd_ok={} fwd_flat={} fallback3={}\n{}\n",
            c[0], c[1], c[2], c[3], c[4], c[5], samples)); }
}
// ★ AUTO 4번째(전원·전 경기 보편): buy 시점(owned==3)에 build[0..3]을 신경망 forward로 채점,
//   각 최종템 후보를 4번째로 붙였을 때 최고점 = 신경망이 고른 4번째. c6 발화와 무관(적/백그라운드 포함).
//   ctx = 로스터 배열서 복원한 그 경기 진짜 라인업(우리팀5+상대5+pos). 실패 시 간이 폴백.
unsafe fn compute_auto_4th_id(athlete: usize, champ: &str) -> Option<u64> {
    if !AUTO4_FORWARD_SCORE { return None; }
    let net = ITEM_NET_ADDR.load(Ordering::Relaxed) as usize;
    if net == 0 || !itemnet_addr_valid() { AUTO4_CNT[0].fetch_add(1, Ordering::Relaxed); return None; }
    let ptr = rd_u64(athlete + 0x4e8) as usize; // 0.5.0 build ptr (구 0x410)
    if ptr < 0x10000 || !readable(ptr, 24) { AUTO4_CNT[1].fetch_add(1, Ordering::Relaxed); return None; }
    let b0 = rd_u64(ptr); let b1 = rd_u64(ptr + 8); let b2 = rd_u64(ptr + 16);
    if b0 >= 0x10000 || b1 >= 0x10000 || b2 >= 0x10000 { AUTO4_CNT[1].fetch_add(1, Ordering::Relaxed); return None; }
    // ★모드챔프(cid 미상): 쓰레기 ctx(cid=0·상대9999)로 forward하면 전원 동일 답(116 고정) → 스킵하고
    //   버라이어티 폴백(champ-hash 분산)으로. 완전한 해결=게임 챔피언 레지스트리 name→id (후속).
    let cid = match champ_id_of(champ) {
        Some(c) => c as u64,
        None => { auto4_sample(format!("[nocid] champ={} → spread", champ)); return None; }
    };
    // ★ 로스터 배열서 그 경기 진짜 라인업 ctx 복원(상대 5명 실제 = global_counter 살아남). 실패 시 간이 폴백.
    let (ctx, real, vcount) = match build_lineup_ctx(athlete) {
        Some((c, vc)) => (c, true, vc),
        None => {
            let mut c = [0u64; 11];
            for k in 5..10 { c[k] = 9999; } // 상대 미상
            c[0] = cid; c[10] = 0;
            (c, false, 0)
        }
    };
    // ★ 데모/타이틀 라이브sim(view roster count==3)에선 게임 forward 호출이 크래시 → 휴리스틱 4번째로 폴백.
    //   실 sim(백그라운드 league 등, count≠3)에선 forward로 신경망 4번째. (DIAG_FWD_OFF=긴급 전역 휴리스틱.)
    if DIAG_FWD_OFF || vcount == 3 {
        AUTO4_CNT[2].fetch_add(1, Ordering::Relaxed);
        let pick = auto_cands().iter().copied().find(|&c| c != b0 && c != b1 && c != b2);
        auto4_sample(format!("[heur] champ={} vcount={} real={} pick={:?}", champ, vcount, real, pick));
        return pick;
    }
    // ★성능 캐시 조회: (champ,build,라인업) 동일하면 forward 스윕 재계산 불요.
    let ckey = (champ.to_string(), b0, b1, b2, ctx);
    {
        let mut g = AUTO4_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        let m = g.get_or_insert_with(HashMap::new);
        if let Some(&cached) = m.get(&ckey) { AUTO4_CNT[5].fetch_add(1, Ordering::Relaxed); return cached; }
    }
    let cands = auto_cands();
    let mut best: Option<u64> = None;
    let mut best_s = f32::MIN;
    let (mut smin, mut smax) = (f32::MAX, f32::MIN);
    for &cand in cands.iter() {
        if cand == b0 || cand == b1 || cand == b2 { continue; } // 중복 제외
        let s = itemnet_forward(net, &ctx, &[b0, b1, b2, cand]);
        if s < smin { smin = s; } if s > smax { smax = s; }
        if s > best_s { best_s = s; best = Some(cand); }
    }
    // ★캐시 저장(상한 8192, 초과시 단순 클리어 — 병렬 경기 다수 대비)
    {
        let mut g = AUTO4_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        let m = g.get_or_insert_with(HashMap::new);
        if m.len() >= 8192 { m.clear(); }
        m.insert(ckey.clone(), best);
    }
    AUTO4_CNT[3].fetch_add(1, Ordering::Relaxed);
    if best.is_some() && smax == smin { AUTO4_CNT[4].fetch_add(1, Ordering::Relaxed); } // 전 후보 동점=의심
    auto4_sample(format!("[fwd] champ={} cid={} real={} vcount={} best={:?} s=[{}..{}]", champ, cid, real, vcount, best, smin, smax));
    {
        static D: AtomicBool = AtomicBool::new(false);
        if !D.swap(true, Ordering::Relaxed) {
            write_log("4items_fwd.txt", &format!(
                "[auto4@buy] champ={} champ_id={} real_ctx={} ctx={:?} build=[{},{},{}] cands={} best={:?} best_s={}\n",
                champ, cid, real, ctx, b0, b1, b2, cands.len(), best, best_s));
        }
    }
    best
}

// 챔프의 4번째 슬롯 지정 아이템 키(SEL slot3). idx 0=자동(None), 1~6=바닐라 카테고리 최종템,
//   7+=모드템. 반환 키로 clone 소스 컬렉션(96개, 바닐라+모드 이름 다 포함)을 스캔.
// slot si(0~3) 수동지정 아이템 키. 0=자동(None), 1~6=바닐라 카테고리 최종템, 7+=모드템.
fn slotN_item_key(scope: Scope, champ: &str, si: u8) -> Option<String> {
    let idx = sel_get(scope, champ, si);
    if idx == 0 { return None; }                       // 자동 → 강제 안 함
    if idx <= 6 {                                       // 바닐라 카테고리 → 그 최종템 이름
        return VANILLA_KEYS.get(VANILLA_FINAL[(idx - 1) as usize] as usize).map(|k| k.to_string());
    }
    mod_final_opts().get(idx as usize - 7).map(|(_, k)| k.clone()) // 모드템
}
fn slot3_item_key(scope: Scope, champ: &str) -> Option<String> { slotN_item_key(scope, champ, 3) }
// slot3 수동지정의 아이템 id (build[3] 타겟용). 0=자동(None), 1~6=바닐라 카테고리 최종, 7+=모드템.
#[allow(dead_code)]
fn slot3_item_id(scope: Scope, champ: &str) -> Option<u64> {
    let idx = sel_get(scope, champ, 3);
    if idx == 0 { return None; }
    if idx <= 6 { return Some(VANILLA_FINAL[(idx - 1) as usize]); }
    mod_final_opts().get(idx as usize - 7).map(|(id, _)| *id)
}
// ★ 바닐라 지정(idx 1~6)만의 build[3] 인덱스. 바닐라는 id==catalog index라 스캔 불요(0.5.0 스캔 깨져도 동작).
//   모드템(7+)은 None 반환(id≠index → 이름스캔 필요).
fn slotN_vanilla_id(scope: Scope, champ: &str, si: u8) -> Option<u64> {
    let idx = sel_get(scope, champ, si);
    if (1..=6).contains(&idx) { Some(VANILLA_FINAL[(idx - 1) as usize]) } else { None }
}
fn slot3_vanilla_id(scope: Scope, champ: &str) -> Option<u64> { slotN_vanilla_id(scope, champ, 3) }
// ★ 4번째 획득 방식: true=build[3]에 목표만 넣고 게임이 t1부터 자연 빌드업(정가 골드). false=최종템 강제 즉시주입.
const AUTO4_NATURAL: bool = true; // ★자연 빌드업(유저 확정): build[3]에 목표만 넣고 게임이 t1부터 정가 골드로 빌드업. 고시작골드=완성률↑ 기대.

// ★ 4번째 목표 = 이름으로 catalog(ctx+0x20) 스캔해 인덱스 획득 + 레시피 검증. (모드템은 id≠인덱스라 이름스캔 필수.)
//   catalog = resolver가 인덱싱하는 배열과 동일(RE 확정). element{elem_ptr@0, vtable@8}, 이름=vtable[0x50],
//   has_recipe=vtable[0x68] 호출(≠0=레시피有). 레시피 없으면 FUN_141d5ab40 panic → 반드시 검증 후 사용.
//   반환 = 레시피 있는 유효 최종템의 catalog 인덱스(build[3]에 그대로 사용 가능). 없으면 None(바닐라 폴백).
unsafe fn scan_recipe_safe_index(ctx: usize, want: &[u8]) -> Option<u64> {
    if want.is_empty() || ctx < 0x10000 || !readable(ctx, 0x28) { return None; }
    let coll = rd_u64(ctx + 0x30) as usize; // ★0.5.0: catalog collection 오프셋 ctx+0x20→+0x30 (RE 확정, 유일 변경)
    if coll < 0x10000 || !readable(coll, 0x18) { return None; }
    let data = rd_u64(coll + 8) as usize;
    let len = rd_u64(coll + 0x10);
    scan_recipe_safe_in(data, len, want)
}
// 공통 스캔 코어: 카탈로그 배열(element{elem_ptr@0, vtable@8}, stride 0x10)에서 이름 일치 + 레시피 보유 인덱스.
unsafe fn scan_recipe_safe_in(data: usize, len: u64, want: &[u8]) -> Option<u64> {
    if data < 0x10000 || len == 0 || len > 100000 || !readable(data, (len as usize) * 16) { return None; }
    // ★0.5.1 진단(1회): want + 추출 이름 샘플 → scan 실패가 vtable slot(추출실패) 문제인지 컬렉션(radiant 없음) 문제인지 규명.
    let do_diag = LOG_ENABLED && !SCAN_DIAG_DONE.swap(true, Ordering::Relaxed);
    let mut dbg = if do_diag { format!("[{}ms] scan want='{}' data={:#x} len={}\n", now_ms(), String::from_utf8_lossy(want), data, len) } else { String::new() };
    let mut names_ok = 0u64;
    let mut i = 0u64;
    while i < len {
        let e = data + (i as usize) * 16;
        let edata = rd_u64(e) as usize;
        let evt = rd_u64(e + 8) as usize;
        if edata >= 0x10000 && evt >= 0x10000 && readable(evt, 0x78) {
            let namefn = rd_u64(evt + 0x58) as usize;
            if code_ptr_ok(namefn) {
                let f: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(namefn);
                let nobj = f(edata);
                if nobj >= 0x10000 && readable(nobj, 0x18) {
                    let chars = rd_u64(nobj + 8) as usize;
                    let nlen = rd_u64(nobj + 0x10) as usize;
                    if chars >= 0x10000 && nlen > 0 && nlen <= 64 && readable(chars, nlen) {
                        let nm = std::slice::from_raw_parts(chars as *const u8, nlen);
                        if do_diag { names_ok += 1; if names_ok <= 12 || nm.starts_with(b"radiant") { dbg.push_str(&format!("  [{}] '{}'\n", i, String::from_utf8_lossy(nm))); } }
                        if nm == want {
                            // ★ 레시피 검증: vtable[0x68] 호출 ≠0 이어야 자연 빌드업 안전(0=기초템→panic).
                            let recfn = rd_u64(evt + 0x70) as usize; // 0.5.1: next_tier/레시피 getter slot +0x68→+0x70(ghidra-re)
                            if code_ptr_ok(recfn) {
                                let rf: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(recfn);
                                if rf(edata) != 0 { return Some(i); }
                            }
                            return None; // 이름은 맞지만 레시피 없음 → 폴백
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if do_diag { dbg.push_str(&format!("  총 이름추출 성공={}/{} · want 미발견\n", names_ok, len)); append_log("scan_diag.txt", &dbg); }
    None
}

// ★ 성능: 스캔 캐시(이름→인덱스). 96원소 shadow-call 스캔을 이름당 1회로 축소. 값 -1=미발견/무레시피.
//   ★멀티컬렉션(coll base 키): 병렬 백그라운드 sim들이 서로 다른 ctx 컬렉션을 써도 thrash 안 함. 컬렉션 상한 16.
static SCAN_CACHE: Mutex<Option<HashMap<usize, HashMap<Vec<u8>, i64>>>> = Mutex::new(None);
// ★★지정템 도달 계측(07-19): "이 챔프가 지정한 아이템을 실제로 보유한 적 있는가"를 누적 기록.
//   스냅샷 라인(champ|owned 조합당 1줄)은 경기 후반 조합을 놓쳐 미도달로 오판하게 만든다 — 이 표는 누적이라 확정적.
//   key = "champ:slot:itemkey" → true(도달). 리포트에 도달/미도달로 출력.
static REACH_HIT: Mutex<Vec<String>> = Mutex::new(Vec::new());   // 지정템 실제 보유 확인된 champ:slot:item
static REACH_WANT: Mutex<Vec<String>> = Mutex::new(Vec::new());  // 지정이 걸린 champ:slot:item (모수)
unsafe fn scan_idx_cached(ctx: usize, want: &[u8]) -> Option<u64> {
    if ctx < 0x10000 || !readable(ctx, 0x28) { return None; }
    let coll = rd_u64(ctx + 0x30) as usize; // ★0.5.0: catalog collection 오프셋 ctx+0x20→+0x30 (RE 확정, 유일 변경)
    { let g = SCAN_CACHE.lock().unwrap_or_else(|e| e.into_inner());
      if let Some(outer) = g.as_ref() { if let Some(m) = outer.get(&coll) {
          if let Some(&v) = m.get(want) { return if v >= 0 { Some(v as u64) } else { None }; } } } }
    let res = scan_recipe_safe_index(ctx, want);
    { let mut g = SCAN_CACHE.lock().unwrap_or_else(|e| e.into_inner());
      if g.is_none() { *g = Some(HashMap::new()); }
      if let Some(outer) = g.as_mut() {
          if !outer.contains_key(&coll) && outer.len() >= 16 { outer.clear(); } // 컬렉션 과다 시 리셋(메모리 상한)
          let m = outer.entry(coll).or_insert_with(HashMap::new);
          if m.len() < 256 { m.insert(want.to_vec(), res.map(|i| i as i64).unwrap_or(-1)); }
      } }
    res
}
// ★ buy_item replace-detour: owned==3 + 챔프가 4번째 모드템 지정 시, clone소스 컬렉션(ctx+0x20)을
//   이름(vtable[0x50])으로 스캔 → 그 모드템 인덱스 i를 rax=1/rdx=i 반환 → run_tick_ext가 clone/push
//   → 4번째=모드템. (그외/미매칭 = passthrough=원본 정상 3구매.)
const DIAG_SCAN_OFF: bool = false; // ★진단 #4: realloc 무죄 확인됨 → 스캔 재개
const DIAG_FWD_OFF: bool = false;  // false=count!=3(실 sim)이면 forward 실행. true=긴급 전역 휴리스틱.
// ★슬롯0/1/2 라이브 주입 게이트: build[0/1/2]에 지정 인덱스 write(슬롯3과 동일 build-Vec 타겟 메커니즘).
const SLOT012_INJECT_ENABLED: bool = true;
static SLOT012_LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());

// ═══════════════════════════════════════════════════════════════════════════
//  buy 주입 진단 리포트 (BUY_REPORT) — 아이템 주입 실패 원인을 buy 시점 런타임에서 특정.
//    ①관전 위치 찾았나(is_live) ②내 팀 맞나(is_player) ③뭘 주입하려는가(슬롯 목표 idx)
//    ④실제 write ⑤게임이 실제 산 것. → mods\tfm2_item_tactics\buy_report.txt 로 출력.
//    read-only 관찰(주입 로직 무변경). ⚠프로덕션 배포 시 false 로 끌 것(진단 파일 write 방지).
// ═══════════════════════════════════════════════════════════════════════════
const BUY_REPORT: bool = false; // ★프로덕션 OFF 복귀(2026-08-20 저녁 — 0.5.6 riot 충돌 체인후킹 검증 완료 후).
                                //   08-20 진단판 실측 = 설치상태3(체인OK)·진입 369만·write 27 = 주입 정상 확증.
                                // (그 전: ★프로덕션 OFF 복귀 2026-08-06, 팀 게이트 수정 검증 완료 후.)
                                //   08-06 회차에 임시 ON 으로 원인을 확정했다 — 판정 지표는 `★MY_ATHLETES 게시 보류`
                                //   (수정 전 5회 → 수정 후 0회). 재검증이 필요하면 여기만 true 로.
                                // ↓이하 이력: ★프로덕션 OFF(2026-07-30 검증 완료 후 복귀): buy_report.txt write + per-buy
                                // 진단 전부 봉인. 주입/식별 기능은 이 게이트 바깥이라 무영향. (재검증 시 true)
// ★0.5.6 진단(2026-08-20): "전체 buy 콜=0"이 detour 미발화인지, 진입 후 무집계 조기탈출(saved null/r8 쓰레기)인지
//   가를 수 없어 진입 무조건 카운터 3종 추가. BR_ENTER=detour 진입 즉시(집계 전 return 없음) —
//   ENTER=0이면 훅 미발화(설치/경로 축), ENTER>0 & TOTAL=0이면 인자 계약 붕괴(BADATH/NULLSAVED로 세분).
static BR_ENTER: AtomicU64 = AtomicU64::new(0);     // detour 진입(무조건)
static BR_NULLSAVED: AtomicU64 = AtomicU64::new(0); // saved==null 탈출
static BR_BADATH: AtomicU64 = AtomicU64::new(0);    // r8(athlete)<0x10000 탈출
static BR_TOTAL: AtomicU64 = AtomicU64::new(0);     // 전체 buy콜
static BR_LIVE: AtomicU64 = AtomicU64::new(0);      // is_live (관전 라이브 경기 buy)
static BR_DES: AtomicU64 = AtomicU64::new(0);       // 지정챔프(SEL) buy
static BR_DES_LIVE: AtomicU64 = AtomicU64::new(0);  // 지정 && is_live
static BR_ISPLAYER: AtomicU64 = AtomicU64::new(0);  // is_player (내 팀 확정)
static BR_IDX_OK: AtomicU64 = AtomicU64::new(0);    // 슬롯 목표 idx 구함
static BR_IDX_NONE: AtomicU64 = AtomicU64::new(0);  // 슬롯 목표 idx 실패(모드템 스캔 실패)
static BR_WROTE: AtomicU64 = AtomicU64::new(0);     // 실제 build write 성공
static BR_LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());   // per-event 상세
// ★조합테스트 스코프·배경오염 진단(2026-07-30 2차 수정 검증용). 리셋 대상 아님(누적 — 오염은
//   경기 진입 엣지 사이의 일정넘김 중에 발생하므로 경기별 리셋하면 증거가 사라진다).
static BR_CT_LIVE: AtomicU64 = AtomicU64::new(0);    // 조합테스트 우회 발동(=COMPTEST_MATCH && is_live)
static BR_CT_STICKY: AtomicU64 = AtomicU64::new(0);  // ★COMPTEST_MATCH=true 인데 is_live=false → **차단된** buy
                                                    //   = 수정 전이라면 배경경기에 주입됐을 호출 수
static BR_BG_PLAYER: AtomicU64 = AtomicU64::new(0);  // ★★진짜 오염 지표: 배경 buy 에 주입됐는데 **내 선수가 아님**(0이어야 정상)
static BR_BG_MINE: AtomicU64 = AtomicU64::new(0);    // 배경 buy·내 선수 = FIXB 의도된 동작(관전==확정 수렴). 0이 아니어도 정상
static BR_SCOPE_B: AtomicU64 = AtomicU64::new(0);    // 진영 판정 = 블루
static BR_SCOPE_R: AtomicU64 = AtomicU64::new(0);    // 진영 판정 = 레드
static BR_SCOPE_NA: AtomicU64 = AtomicU64::new(0);   // 조합테스트인데 진영 판정 실패 → Plain 폴백
static BR_SEEN: Mutex<Vec<String>> = Mutex::new(Vec::new());  // dedup key
static BR_WAS_INGAME: AtomicBool = AtomicBool::new(false);    // InGame 진입 엣지 감지(새 경기마다 리셋용)

// ★★fix B(2026-07-27): 관전==확정. is_live 조기탈출 제거 → 배경에도 주입, 팀스코프=is_my_athlete(+0x810).
//   내 선수=지정템 / 나머지=신경망, 배경·관전 동일 → 수렴. id기반이라 AI끼리 경기는 my=0=지정無=통계오염0.
//   ⚠false=옛 동작(is_live 게이트·배경 무주입) 복원. 문제 시 즉시 롤백용.
const FIXB: bool = true;
// ★BG4(2026-08-11 유저 요청 "백그라운드 경기들도 4번째 아이템 구매"): 배경 리그 sim 의
//   비-내선수(타 팀 전원)도 build Vec 3→4 확장 — 조기탈출 직전에 경량 경로로만 수행하고
//   무거운 주입 경로(readable·지정템·신경망)는 안 탄다. 4번째 = 챔프명 FNV 분산 바닐라
//   최종템 폴백(챔프별 결정적 = 리플레이 안전. 신경망은 배경 rayon 대량병렬 미검증이라 안 씀).
//   mode=4 + BUILD_EXTEND_ENABLED 일 때만. false = 구 동작(배경 비-내선수 3템).
const BG4_ALL_ENABLED: bool = true;
static BG4_EXT: AtomicU64 = AtomicU64::new(0); // 진단: 배경 build 3→4 확장 횟수
static BG4_FIRST_LOGGED: AtomicBool = AtomicBool::new(false);

/// 배경 비-내선수 buy 의 경량 4칸 확장. len==3&&cap==3(선수당 1회)일 때만 무거운 검사 진입.
unsafe fn bg4_extend_build(athlete: usize) {
    // 싼 검사(VEH 읽기 2회) — 이미 확장됐으면(len==4) 즉시 복귀 = 배경 buy 핫패스 비용 최소.
    match safe_read_u64(athlete + 0x4f0) { Some(3) => {} _ => return } // build len
    match safe_read_u64(athlete + 0x4e0) { Some(3) => {} _ => return } // build cap
    if !readable(athlete, 0x4f8) { return; }
    let ptr = rd_u64(athlete + 0x4e8) as usize; // build ptr
    if ptr < 0x10000 || !readable(ptr, 24) || !writable(athlete + 0x4e0, 0x18) { return; }
    let (b0, b1, b2) = (rd_u64(ptr), rd_u64(ptr + 8), rd_u64(ptr + 16));
    // 챔프명 FNV → VANILLA_FINAL 분산 시작점(라이브 폴백과 동일 식 — 공격력 편향 방지)
    let mut h: u64 = 0xcbf29ce484222325;
    let cptr = rd_u64(athlete + 0x470) as usize;
    let clen = rd_u64(athlete + 0x478) as usize;
    let mut champ = String::new();
    if cptr >= 0x10000 && clen > 0 && clen <= 48 && readable(cptr, clen) {
        let bytes = std::slice::from_raw_parts(cptr as *const u8, clen);
        for &b in bytes { h = (h ^ b as u64).wrapping_mul(0x100000001b3); }
        if !BG4_FIRST_LOGGED.load(Ordering::Relaxed) {
            champ = String::from_utf8_lossy(bytes).into_owned();
        }
    }
    let start = (h % 6) as usize;
    let t4 = (0..6).map(|k| VANILLA_FINAL[(start + k) % 6]).find(|&v| v != b0 && v != b1 && v != b2);
    if let Some(t) = t4 {
        let realloc: ReallocFn = core::mem::transmute(exe_base_addr() + RVA_REALLOC);
        let np = realloc(ptr, 24, 8, 32);
        if np >= 0x10000 && writable(np, 32) {
            wr_u64(np + 24, t);
            wr_u64(athlete + 0x4e8, np as u64);
            wr_u64(athlete + 0x4e0, 4);
            wr_u64(athlete + 0x4f0, 4);
            let n = BG4_EXT.fetch_add(1, Ordering::Relaxed) + 1;
            // 작동 증거 1줄(최초 1회, LOG_ENABLED 무관·비용 무시) — "배경 4칸 확장이 실제 발화했다"
            if n == 1 && !BG4_FIRST_LOGGED.swap(true, Ordering::Relaxed) {
                append_log("4items_bg4.txt",
                    &format!("[bg4 첫 발화] champ={} build=[{},{},{}]+[{}]", champ, b0, b1, b2, t));
            }
        }
    }
}
unsafe extern "C" fn buy_replace_ctx(saved: *mut u64, rsp_entry: usize) -> u64 {
    // ★핫패스(rayon 워커 병렬) — 전역 원자 카운터는 캐시라인 경합으로 측정 자체가 부하가 되므로
    //   thread_local 누적(rec_tl) 사용. T_BUY_ALL = 디투어 전체(catch_unwind 포함),
    //   T_BUY_EARLY = 배경 sim 조기탈출분(ALL 에 포함되므로 중복 계상 — 해석 시 차감).
    let __bt = perf::tsc();
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> u64 {
        if BUY_REPORT { BR_ENTER.fetch_add(1, Ordering::Relaxed); } // ★진입 무조건(0.5.6 진단)
        if saved.is_null() { if BUY_REPORT { BR_NULLSAVED.fetch_add(1, Ordering::Relaxed); } return 0; } // ★mode=3도 통과(슬롯0/1/2 지정 주입). 4번째 로직만 아래서 mode=4 게이트.
        let athlete = *saved.add(2) as usize; // r8
        if athlete < 0x10000 { if BUY_REPORT { BR_BADATH.fetch_add(1, Ordering::Relaxed); } return 0; }
        // ★★★조기탈출 재정렬(2026-07-22, perf 계측으로 확정): 여기 앞에 `readable(athlete,0x4a8)`가 있었는데
        //   readable()=**VirtualQuery 커널호출**이라, 전 buy콜(130.7s에 689만회·초당 5.3만)이 매번 커널에 진입했다.
        //   ⇒ buy 조기탈출 평균 3.6µs = 모드 전체 비용의 75%(25.9 코어초). athlete 필드는 is_live 게이트를
        //   통과한 뒤에만 만지므로 **검사와 진단을 전부 게이트 뒤로 이동**(VirtualQuery 689만 → 약 8만회=1.2%).
        //   07-18 lean 주석이 "메모리 읽기 2회로 즉시 빠진다"고 했지만 실제로는 그 앞에 커널호출이 남아 의도가
        //   무효화돼 있었음. (LOG_ENABLED 진단블록도 함께 이동 — 원래 목적이 관전 경기 4번째템 티어추적이라 의미 동일.)
        // ★★★핫패스 조기탈출(07-18 lean): 관전(렌더) 경기 판정을 최우선으로 — 전 buy콜의 ~94%가 배경 리그 sim이라
        //   여기서 메모리 읽기 2회로 즉시 빠진다. (구조: 챔프명 추출+해시조회를 먼저 하던 순서가 비용의 주범이었음.)
        // ★관전 식별 v13(작동확정 07-18): buy r9(saved[3]) = provider(0xeb08 sim 객체).
        //   provider+O_PROVIDER_SEED(0.5.3=0xeaf8, 구0.5.2=0xeab8) = 경기 seed(값 불변, serpen 인게임검증) == LIVE_SEED(launcher 훅 캡처) → 화면 경기.
        //   보조: RENDER_PROVIDER(seed-ctor 훅이 rdx==LIVE_SEED 매칭으로 캡처) 포인터 동일성.
        //   ⚠[rsp+0x30]=buy-list 컨테이너(provider 아님) — 구 게이트(v5~v11)의 실패 원인. r9가 정답(RE 확정).
        let lseed = LIVE_SEED.load(Ordering::Relaxed);
        let provider_now = *saved.add(3); // r9 = param_4 = provider
        let seed_r9 = if provider_now >= 0x10000 && provider_now < 0x0000_8000_0000_0000 {
            safe_read_u64(provider_now as usize + O_PROVIDER_SEED).unwrap_or(0)
        } else { 0 };
        let seed_match_r9 = lseed != 0 && seed_r9 == lseed;
        let rp = RENDER_PROVIDER.load(Ordering::Relaxed);
        let is_live = seed_match_r9 || (rp != 0 && provider_now != 0 && provider_now == rp);
        if !is_live && !FIXB {
            // (FIXB=false 옛 동작) 배경 리그 sim = 무주입 passthrough.
            if BUY_REPORT { BR_TOTAL.fetch_add(1, Ordering::Relaxed); }
            perf::rec_tl(perf::T_BUY_EARLY, __bt);
            return 0;
        }
        // ★fix B: FIXB=true면 배경도 주입 진행(팀스코프=is_my_athlete). is_live 전용 카운터만 게이트.
        if is_live { PROV_HIT.fetch_add(1, Ordering::Relaxed); if seed_match_r9 { VT_OK.fetch_add(1, Ordering::Relaxed); } }
        // ★★fix B 성능(2026-07-27): 배경 buy는 내 선수(is_my_athlete)만 주입 대상 → 비-내선수 배경 buy는
        //   무거운 readable(=VirtualQuery 커널호출) 전에 싼 VEH읽기(+0x810)+HashSet 대조로 즉시 passthrough.
        //   07-22가 없앤 배경 조기탈출을 fix와 양립하게 복원(배경 buy ~94%가 여기서 빠짐). None(로스터 미확보)=
        //   주입안함=조기탈출(옛 동작 동일). 관전(is_live)은 항상 통과(by_scene 판정 필요).
        // ★08-07 구멍2 보강 — 멤버십에서 빠진 내 팀 선수(교체·부상 출전)를 **매치 내 side 전파**로 구제한다.
        //   구: `is_my_athlete != Some(true)` 면 무조건 조기탈출 ⟹ 선발 외 출전 선수는 영구 미주입.
        //   신: 그 매치에 내 선발이 한 명이라도 있으면 그 side 전체 인정(provider 키 lock-free 캐시).
        if FIXB && !is_live && !matches!(is_my_athlete(athlete), Some(true)) {
            let rescued = match my_side_in_match(provider_now, athlete) {
                Some(ms) => match safe_read_u64(athlete + 0x930) { Some(s) => s == ms, None => false },
                None => false,
            };
            // ★v2.8.0 TN 게이트(유저 지시 "하나만 걸려도 주입"): 대회 디스크립터가 이 sim(seed)의 내 팀
            //   side 를 확정하면, aid 멤버십이 놓친 내 선수(교체 출전·aid 미기입)도 구제. 추가 승인 전용.
            let tn_rescued = !rescued && match tn_my_side(provider_now as usize) {
                Some(ms) => matches!(safe_read_u64(athlete + 0x930), Some(s) if s == ms),
                None => false,
            };
            if !(rescued || tn_rescued) {
                // ★BG4: 조기탈출 전에 배경 비-내선수도 4칸 확장(경량 경로)만 수행.
                //   지정템 주입(SLOT012)·신경망은 그대로 안 탐 = 리그 오염 없음(fix A 거부 사유와 무관).
                if BG4_ALL_ENABLED && BUILD_EXTEND_ENABLED && slot_count() == 4 {
                    bg4_extend_build(athlete);
                }
                if BUY_REPORT { BR_TOTAL.fetch_add(1, Ordering::Relaxed); }
                perf::rec_tl(perf::T_BUY_EARLY, __bt);
                return 0;
            }
            if rescued { MYSIDE_HIT.fetch_add(1, Ordering::Relaxed); }
            if tn_rescued { TN_GATE_EARLY.fetch_add(1, Ordering::Relaxed); }
        }
        // ── 여기부터는 관전 경기 buy(전체 소수) + 배경의 내 선수 buy(5명)만 도달 ──
        // ★athlete 유효성 검사(VirtualQuery)는 여기서 1회 — 위 재정렬 주석 참조.
        if !readable(athlete, 0x4f8) { return 0; } // 0.5.0: build len@+0x4a0+8 커버
        let owned = rd_u64(athlete + 0x4a8); // 0.5.0 owned (구 0x3d0)
        if LOG_ENABLED && owned <= 8 { let p = MAX_OWNED4.load(Ordering::Relaxed); if owned > p { MAX_OWNED4.store(owned, Ordering::Relaxed); } }
        // ★진단: 4번째(owned[3]) 티어 진행 추적 — 순차(t0→t4)인지 최종템 직구인지. (프로덕션: LOG_ENABLED 게이트)
        if LOG_ENABLED && owned >= 3 && owned <= 8 {
            let cptr0 = rd_u64(athlete + 0x470) as usize; let clen0 = rd_u64(athlete + 0x478) as usize; // 0.5.0 champ name (구 0x398/0x3a0, +0x88 파생)
            if cptr0 >= 0x10000 && clen0 > 0 && clen0 <= 48 && readable(cptr0, clen0) {
                let cn = String::from_utf8_lossy(std::slice::from_raw_parts(cptr0 as *const u8, clen0)).into_owned();
                let optr = rd_u64(athlete + 0x440) as usize; // 0.5.0 item slot array (구 0x3c8)
                // owned[3] (4번째) 티어 (있으면)
                let mut t3 = -1i64;
                if owned >= 4 && optr >= 0x10000 && readable(optr, 4 * 0x10) {
                    let ep3 = rd_u64(optr + 3 * 0x10) as usize;
                    if ep3 >= 0x10000 && readable(ep3, 0x190) { t3 = (rd_u64(ep3 + 0x188) & 0xffffffff) as i64; }
                }
                let key = format!("{}:o{}:t{}", cn, owned, t3);
                let mut cl4 = CHAMP_AT4.lock().unwrap_or_else(|e| e.into_inner());
                if cl4.len() < 80 && !cl4.iter().any(|c| c == &key) {
                    cl4.push(key); drop(cl4);
                    let mut items = String::new();
                    if optr >= 0x10000 && readable(optr, (owned as usize) * 0x10) {
                        for i in 0..(owned as usize).min(6) {
                            let ep = rd_u64(optr + i * 0x10) as usize;
                            if ep >= 0x10000 && readable(ep, 0x190) {
                                let tier = rd_u64(ep + 0x188) & 0xffffffff;
                                let price = rd_u64(ep + 0x180) & 0xffffffff;
                                items.push_str(&format!("[{}:t{}/${}] ", i, tier, price));
                            } else { items.push_str(&format!("[{}:bad] ", i)); }
                        }
                    }
                    append_log("4items_buy4.txt", &format!("[owned={} 4th_tier={}] champ={} items={}", owned, t3, cn, items));
                }
            }
        }
        // ★ 타겟 챔프(지정)만 처리 — 비타겟은 passthrough(build 손 안 댐).
        let cptr = rd_u64(athlete + 0x470) as usize; // 0.5.0 champ name ptr (구 0x398, +0x88 파생)
        let clen = rd_u64(athlete + 0x478) as usize; // 0.5.0 champ name len (구 0x3a0)
        if cptr < 0x10000 || clen == 0 || clen > 48 || !readable(cptr, clen) { return 0; }
        // ★성능: Cow 차용(유효 UTF-8이면 힙 할당 없음).
        let champ_cow = String::from_utf8_lossy(std::slice::from_raw_parts(cptr as *const u8, clen));
        let champ: &str = champ_cow.as_ref();
        let champ_designated = is_champ_designated(champ); // 스냅샷 zero-alloc
        let side = if readable(athlete + 0x930, 8) { rd_u64(athlete + 0x930) } else { u64::MAX };
        // ★08-08 대회 보험 실측② — 내 팀 대회 배경경기에서 (레코드 슬롯, 세트 side 바이트) ↔ 실제 athlete
        //   side(+0x810) 투표. TN_MY_SEED(launcher 캡처 시 저장) == 이 sim 의 provider seed 면 같은 경기.
        //   여기 도달한 배경 buy 는 이미 내 선수(멤버십/구제 게이트 통과) ⟹ 저비용(원자 2로드+VEH 1읽기).
        if TN_ENABLED && !is_live && side <= 1 {
            let tseed = TN_MY_SEED.load(Ordering::Relaxed);
            if tseed != 0 && safe_read_u64(provider_now as usize + O_PROVIDER_SEED) == Some(tseed)
                && matches!(is_my_athlete(athlete), Some(true)) {
                let idx = (TN_MY_SLOT.load(Ordering::Relaxed) as usize & 1) * 4
                    + (TN_MY_SB.load(Ordering::Relaxed) as usize & 1) * 2 + (side as usize & 1);
                TN_VOTE[idx].fetch_add(1, Ordering::Relaxed);
                // ★v2.7.6: 매핑식 예측 대조(NG=0 이어야 매핑 확정 유지)
                if side == TN_MY_PRED.load(Ordering::Relaxed) { TN_VOTE_OK.fetch_add(1, Ordering::Relaxed); }
                else { TN_VOTE_NG.fetch_add(1, Ordering::Relaxed); }
            }
        }
        // ★side 판별: scene 직독(SCENE_SIDE, 메인스레드 갱신) 우선 → 미정이면 LIVE_DB로 즉석 판정(owned=0 주입창 보호).
        //   미판정 = 주입 안 함(적/배경 오염 방지 — 폴백 투표 폐기 확정).
        let scene_ps = scene_player_side().or_else(|| {
            if !is_live { return None; }
            let db = LIVE_DB.load(Ordering::Relaxed) as usize;
            let pid = LIVE_PID.load(Ordering::Relaxed);
            if db == 0 { return None; }
            let r = quick_scene_side(db, pid);
            if let Some(s) = r { SCENE_SIDE.store(s, Ordering::Relaxed); }
            r
        });
        let by_scene: bool = if is_live {
            match scene_ps {
                Some(ps) => side == ps,
                None => match player_side_for_match(athlete) { Some(ps) => side == ps, None => false },
            }
        } else { false };
        // ★조합테스트: 양 진영 다 유저 구성이므로 scene side 게이트를 우회(지정 챔프면 적용).
        // ★★fix B: 팀스코프 = athlete_id 멤버십(is_my_athlete, +0x810). 배경·관전 동일 판정 → 수렴.
        //   MY_ATHLETES 미게시(관전 전)면 None=false=신경망. 내 선수만 지정, AI끼리=my0=지정無.
        // ★★조합테스트 회귀 수정(2026-07-30 유저 제보 "조합테스트할때 아이템 주입이 안되"):
        //   FIXB(=athlete_id 멤버십) 경로가 **조합테스트 우회를 빠뜨리고 있었다**.
        //   조합테스트는 유저가 양 진영을 직접 구성하는 샌드박스라 그 선수들이 `MY_ATHLETES`
        //   (= db.team(pid).last_starting = 내 팀 선발)에 **들어있지 않다** ⟹ is_my_athlete=false
        //   ⟹ 지정 아이템 주입이 조용히 스킵됐다. (구 FIXB=false 경로에는 COMPTEST_MATCH 우회가 있었으나
        //    FIXB=true 로 전환하면서 그 조건이 사라진 것 = 마이그가 아니라 fix B 도입 시점의 누락.)
        //   ⟹ 조합테스트로 판정된 경기(launcher retaddr 실측 확인 = 0x1925f12)면 **양 진영 모두 지정 적용**.
        //     "내 팀" 개념이 없는 화면이라 팀 게이트를 우회하는 것이 원래 설계 의도였다(2409행 주석 참조).
        // ★★★2026-07-30 2차 수정 — **배경경기 오염 차단**(`&& is_live`):
        //   `COMPTEST_MATCH` 는 화면 경기 launcher 가 다시 올 때만 갱신되는 **sticky 전역 플래그**다
        //   (배경 sim 콜사이트 0x220acb·0x195c5be·0x20dac9c·0x2256a6d 는 갱신하지 않는다).
        //   ⟹ 조합테스트를 한 번 하면 그 뒤 일정넘김 배경 sim 의 buy 전부가 is_player=true 가 되어
        //     **배경 경기 양팀 전 선수에게 지정 아이템이 주입**됐다(관전/내 경기를 새로 시작할 때까지).
        //   1차 수정(우회를 최우선 분기로 올림) 때 구 조건식 `is_live && (by_scene || COMPTEST_MATCH)`
        //   의 **is_live AND 가 함께 떨어져 나간 것**이 원인. 조합테스트 본경기·기록 다시보기는 둘 다
        //   화면 경기(launcher 가 LIVE_SEED 를 심음)라 is_live 로 걸러도 기능은 그대로다.
        let is_comptest_live = COMPTEST_MATCH.load(Ordering::Relaxed) && is_live;
        // ★★★2026-08-07 — **화면 경기는 scene side 로 판정한다(로스터 의존 제거).**
        //   경위: 08-06 결함(pid=0 세이브에서 세션 첫 ~10초간 `MY_ATHLETES` 미게시 → 지정 전량 미주입)의
        //   근본 원인은 팀 판정이 **비동기 게시(로스터)** 에 의존한 것이었다. 반면 scene 은 매치 단위로
        //   두 팀 team_id + is_team1_blue 를 **상주**시킨다(`quick_scene_side`) ⟹ 게시를 기다릴 필요가 없고
        //   `LIVE_PID` 는 첫 InGame 프레임에 잡히므로 600틱 trust 게이트도 안 거친다.
        //   ★근거(08-07 실측 buy_report): 화면 경기 buy 28건 중 scene 판정 성립 20건, 그 20건 전부
        //     scene side 판정과 `is_my_athlete` 판정이 **100% 일치**(내 선수 side0 / 적 side1 양방향).
        //   ⟹ 결과는 같고 의존성만 줄어든다. scene 미성립(관전 아님·tag9 전·리그 외 형태)이면 기존 경로로 폴백.
        //   ⚠배경 sim 은 scene 이 없으므로 계속 `is_my_athlete`(FIXB) — "관전==확정 수렴" 성질 보존.
        // ★★08-07 정정 — `[rsp_entry+0x30]` 은 **Game 이 아니라 ctx** 다(1차에서 오라벨).
        //   근거: ctx 빌더가 `ctx+0x30 = &(Game+0x1fc8)`(카탈로그 Vec 의 주소)을 넣는다 ⟹ **Game = *(ctx+0x30) − 0x1fc8**.
        //   그리고 Game 은 **스택 상주**(launcher rcx·buy 쪽 둘 다 `0x7a30…` 대역)라, post_update 같은 다른
        //   프레임/스레드에서 읽으면 `readable=false` 가 난다 ⟹ **살아있는 디투어 안에서 1회만** 훑는다.
        //   비용: 1회 한정(`GAME_SCAN_DONE`), `readable()` 1회 + 평범한 읽기 1,056회. 파일 IO 없음(원자값만).
        if is_live {
            let ctxp = rd_u64(rsp_entry + 0x30) as usize;
            if ctxp >= 0x10000 && readable(ctxp, 0x38) {
                let g = rd_u64(ctxp + 0x30).wrapping_sub(0x1fc8) as usize;
                if g >= 0x10000 {
                    if BUY_GAME.load(Ordering::Relaxed) == 0 { BUY_GAME.store(g as u64, Ordering::Relaxed); }
                    let (t1, t2) = (SCENE_T1.load(Ordering::Relaxed), SCENE_T2.load(Ordering::Relaxed));
                    if t1 != u64::MAX && t2 != u64::MAX && !GAME_SCAN_DONE.swap(true, Ordering::Relaxed) {
                        if !readable(g, 0x2100) { GAME_SCAN_OK.store(2, Ordering::Relaxed); }
                        else {
                            GAME_SCAN_OK.store(1, Ordering::Relaxed);
                            let mut n = 0usize;
                            let mut o = 0usize;
                            while o + 16 <= 0x2100 && n < 4 {   // u64 인접쌍
                                let (a, b) = (rd_u64(g + o), rd_u64(g + o + 8));
                                if (a == t1 && b == t2) || (a == t2 && b == t1) {
                                    GAME_HIT[n].store(o as u64, Ordering::Relaxed); n += 1;
                                }
                                o += 8;
                            }
                            let mut o = 0usize;
                            while o + 8 <= 0x2100 && n < 4 {    // u32 인접쌍(팀 id 가 32비트인 경우)
                                let w = rd_u64(g + o);
                                let (a, b) = (w & 0xffff_ffff, w >> 32);
                                if (a == t1 && b == t2) || (a == t2 && b == t1) {
                                    GAME_HIT[n].store((o as u64) | 0x8000_0000, Ordering::Relaxed); n += 1;
                                }
                                o += 4;
                            }
                            GAME_HIT_N.store(n as u64, Ordering::Relaxed);
                        }
                    }
                }
            }
        }
        let scene_gate: Option<bool> = if is_live && side <= 1 { scene_ps.map(|ps| side == ps) } else { None };
        let is_player = if is_comptest_live {
            true                        // 조합테스트 = 양 진영 다 유저 구성 → 팀 게이트 우회
        } else if let Some(sg) = scene_gate {
            // ★화면 경기 = scene side(로스터 불요). 전환 검증 완료 = 08-07 실측 **15,624건 불일치 0**
            //   ⟹ 평시엔 `is_my_athlete` 호출 자체를 생략한다(VEH 읽기 + HashSet 조회를 buy 마다 하던 것).
            //   단 **1/256 샘플로 교차검증은 유지** — 회귀(scene 오프셋 변화 등)를 조용히 넘기지 않기 위한 트립와이어.
            if (GATE_SAMPLE.fetch_add(1, Ordering::Relaxed) & 0xff) == 0 {
                if sg == matches!(is_my_athlete(athlete), Some(true)) { GATE_AGREE.fetch_add(1, Ordering::Relaxed); }
                else { GATE_DIFF.fetch_add(1, Ordering::Relaxed); }
            }
            sg
        } else if matches!(is_my_athlete(athlete), Some(true)) {
            GATE_ROSTER.fetch_add(1, Ordering::Relaxed); true       // 배경 sim = athlete_id 멤버십
        } else {
            // ★08-07 구멍2: 멤버십 밖이어도 그 매치에서 내 side 가 확정되면 인정(교체선수 구제)
            match my_side_in_match(provider_now, athlete) {
                Some(ms) if side == ms => { GATE_ROSTER.fetch_add(1, Ordering::Relaxed); true }
                // ★v2.8.0 TN 게이트: 대회 디스크립터 확정 내 side 일치 → 승인(멤버십·side전파 다음의 3번째 소스)
                _ => match unsafe { tn_my_side(provider_now as usize) } {
                    Some(ms) if side == ms => { TN_GATE_HIT.fetch_add(1, Ordering::Relaxed); true }
                    _ => false,
                },
            }
        };
        // ★★08-07 지표 정정 — 구 정의 `champ_designated && !is_player` 는 **적팀 선수(정상 차단)까지 세서**
        //   실측 5,340 을 찍었다. 지정 챔프가 105개라 상대 로스터 대부분이 여기 걸린다 = 정상 동작인데 결함처럼 보인다.
        //   07-30 교훈("지표는 **결함일 때만** 증가하는 조건으로 정의할 것")을 그대로 반복했다.
        //   ⟹ 진짜 결함 = "**내 선수인지 판정조차 못 해서** 막힌 것" = `is_my_athlete()` 가 `None`.
        //     `Some(false)`(확정 타팀)는 정상 차단이므로 따로 센다.
        if champ_designated && !is_player {
            match is_my_athlete(athlete) {
                None => { GATE_NONE.fetch_add(1, Ordering::Relaxed); }        // ★결함: 판정 불가로 스킵
                _    => { GATE_BLOCK_OK.fetch_add(1, Ordering::Relaxed); }    // 정상: 확정 타팀
            }
        }
        if let Some(_) = scene_gate { GATE_SCENE.fetch_add(1, Ordering::Relaxed); }
        let _ = by_scene; // (구 경로 — scene_gate 로 대체됨. 진단 출력에서만 사용)
        // ★★SEL 스코프 결정(2026-07-30): 조합테스트면 그 선수의 진영(블루/레드) 스코프로 지정을 읽는다.
        //   조합테스트가 아니면 Scope::Plain = 예전과 동일한 조회 ⟹ 리그·관전·배경 동작 무변경.
        //   이걸로 "양 진영에 같은 챔프를 놓으면 지정이 하나로 합쳐지는" 문제와 "조합테스트 지정이
        //   일반 경기로 새 나가는" 문제가 동시에 사라진다(진영별 키 + 스코프별 조회).
        let scope = if is_comptest_live { ct_scope_for(champ, side) } else { Scope::Plain };
        if BUY_REPORT {
            if is_comptest_live {
                BR_CT_LIVE.fetch_add(1, Ordering::Relaxed);
                match scope {
                    Scope::CtBlue => { BR_SCOPE_B.fetch_add(1, Ordering::Relaxed); }
                    Scope::CtRed  => { BR_SCOPE_R.fetch_add(1, Ordering::Relaxed); }
                    Scope::Plain  => { BR_SCOPE_NA.fetch_add(1, Ordering::Relaxed); }
                }
            } else if COMPTEST_MATCH.load(Ordering::Relaxed) {
                BR_CT_STICKY.fetch_add(1, Ordering::Relaxed); // ★수정 전이라면 배경에 주입됐을 호출
            }
            // ★지표 정정(2026-07-30): 구 `is_player && !is_live` 는 **FIXB 의 의도된 배경 주입**
            //   (내 선수가 배경 리그 경기에서 지정 적용 = 관전==확정 수렴)까지 세어 1841 을 찍었고,
            //   그것을 오염으로 오판 보고했다. ⟹ **결함일 때만 증가하는 조건**으로 분리한다.
            if is_player && !is_live {
                if matches!(is_my_athlete(athlete), Some(true)) {
                    BR_BG_MINE.fetch_add(1, Ordering::Relaxed);    // 정상(FIXB 의도)
                } else if matches!(tn_my_side(provider_now as usize), Some(ms) if side == ms) {
                    BR_BG_MINE.fetch_add(1, Ordering::Relaxed);    // ★v2.8.0 정상: TN 게이트 구제(내 팀 확정)
                } else {
                    BR_BG_PLAYER.fetch_add(1, Ordering::Relaxed);  // ★결함: 내 선수가 아닌데 배경에 주입
                }
            }
        }
        // ═══ buy 단계별 기록 (BUY_REPORT) — read-only, 주입 로직 무변경 ═══
        if BUY_REPORT {
            BR_TOTAL.fetch_add(1, Ordering::Relaxed);
            if is_live { BR_LIVE.fetch_add(1, Ordering::Relaxed); }
            if champ_designated {
                BR_DES.fetch_add(1, Ordering::Relaxed);
                if is_live { BR_DES_LIVE.fetch_add(1, Ordering::Relaxed); }
                if is_player { BR_ISPLAYER.fetch_add(1, Ordering::Relaxed); }
                // ★★지정템 도달 계측(누적, dedupe 밖 — owned 제한 없음): 경기 후반 조합까지 잡는다.
                //   owned 배열을 훑어 지정 아이템키가 실제로 들어있으면 REACH_HIT에 1회 등록.
                //   (스냅샷 라인은 own2까지만 캡처돼 후반 도달을 놓치므로 미도달 오판의 원인이 됐음.)
                if is_player && owned > 0 && owned <= 6 {
                    let optr = rd_u64(athlete + 0x440) as usize;
                    if optr >= 0x10000 && readable(optr, (owned as usize) * 0x10) {
                        for si in 0u8..3 {
                            let Some(want) = slotN_item_key(scope, champ, si) else { continue };
                            let key = format!("{}:s{}:{}", champ, si, want);
                            { let mut w = REACH_WANT.lock().unwrap_or_else(|e| e.into_inner());
                              if w.len() < 64 && !w.iter().any(|k| k == &key) { w.push(key.clone()); } }
                            { let h = REACH_HIT.lock().unwrap_or_else(|e| e.into_inner());
                              if h.iter().any(|k| k == &key) { continue; } }
                            for i in 0..(owned as usize) {
                                let ep = rd_u64(optr + i * 0x10) as usize;
                                if ep < 0x10000 || !readable(ep, 0x20) { continue; }
                                let chars = rd_u64(ep + 8) as usize; let nlen = rd_u64(ep + 0x10) as usize;
                                if chars >= 0x10000 && nlen > 0 && nlen <= 64 && readable(chars, nlen) {
                                    let nm = std::slice::from_raw_parts(chars as *const u8, nlen);
                                    if nm == want.as_bytes() {
                                        let mut h = REACH_HIT.lock().unwrap_or_else(|e| e.into_inner());
                                        if h.len() < 64 && !h.iter().any(|k| k == &key) { h.push(key.clone()); }
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                // per-event 상세: (champ,side,owned,player) 조합 1회, owned<=3(초반 구매창)만.
                //   player 를 key 에 포함 → 라이브(내 팀)와 배경 sim 둘 다 잡힘.
                if owned <= 3 {
                    let key = format!("{}|s{}|o{}|p{}", champ, side, owned, is_player);
                    let mut seen = BR_SEEN.lock().unwrap_or_else(|e| e.into_inner());
                    if seen.len() < 500 && !seen.iter().any(|k| k == &key) {
                        seen.push(key); drop(seen);
                        let ctx012 = rd_u64(rsp_entry + 0x30) as usize;
                        // 각 슬롯의 주입 목표를 진단용으로 계산(원본 write 경로와 동일 함수 = 무엇을 주입하려는지)
                        let mut slots_s = String::new();
                        for si in 0u8..3 {
                            if owned > si as u64 { slots_s.push_str(&format!("s{}=이미삼 ", si)); continue; }
                            if let Some(vid) = slotN_vanilla_id(scope, champ, si) {
                                slots_s.push_str(&format!("s{}=바닐라#{} ", si, vid));
                                BR_IDX_OK.fetch_add(1, Ordering::Relaxed);
                            } else if let Some(mk) = slotN_item_key(scope, champ, si) {
                                if ctx012 >= 0x10000 {
                                    match scan_idx_cached(ctx012, mk.as_bytes()) {
                                        Some(t) => { slots_s.push_str(&format!("s{}=모드템'{}'#{} ", si, mk, t)); BR_IDX_OK.fetch_add(1, Ordering::Relaxed); }
                                        None => { slots_s.push_str(&format!("s{}=모드템'{}'*스캔실패 ", si, mk)); BR_IDX_NONE.fetch_add(1, Ordering::Relaxed); }
                                    }
                                } else { slots_s.push_str(&format!("s{}=모드템'{}'(ctx무효) ", si, mk)); }
                            } else {
                                slots_s.push_str(&format!("s{}=지정없음 ", si));
                            }
                        }
                        // 게임이 실제 산 아이템(athlete+0x450 배열): 우리 주입대로 갔는지 대조
                        let optr = rd_u64(athlete + 0x440) as usize;
                        let mut bought = String::new();
                        if owned > 0 && owned <= 6 && optr >= 0x10000 && readable(optr, (owned as usize) * 0x10) {
                            for i in 0..(owned as usize) {
                                let ep = rd_u64(optr + i * 0x10) as usize;
                                if ep >= 0x10000 && readable(ep, 0x190) {
                                    let tier = rd_u64(ep + 0x188) & 0xffffffff;
                                    // ★아이템 이름(key String@ep+0x0: chars@+8, len@+0x10) — 목표 도달 여부 확정용
                                    let chars = rd_u64(ep + 8) as usize; let nlen = rd_u64(ep + 0x10) as usize;
                                    let nm = if chars >= 0x10000 && nlen > 0 && nlen <= 48 && readable(chars, nlen) {
                                        String::from_utf8_lossy(std::slice::from_raw_parts(chars as *const u8, nlen)).into_owned()
                                    } else { "?".into() };
                                    bought.push_str(&format!("[{} t{}]", nm, tier));
                                }
                            }
                        }
                        let scstr = match scene_ps { Some(sp) => format!("{}", sp), None => "미판정".into() };
                        let sctag = match scope { Scope::CtBlue => "CT블루", Scope::CtRed => "CT레드", Scope::Plain => "일반" };
                        // ★aid(athlete+0x810) 동봉: is_player 판정 근거(MY_ATHLETES 멤버십)를 대조 가능하게.
                        let aid_s = match safe_read_u64(athlete + O_ATHLETE_ID) { Some(v) => v.to_string(), None => "읽기실패".into() };
                        let line = format!("{:>16} s{} own{} aid={} | live={} player={} scope={} scene_side={} | 목표: {}| 실제산것: {}",
                            champ, side, owned, aid_s, is_live, is_player, sctag, scstr,
                            slots_s, if bought.is_empty() { "(없음)" } else { &bought });
                        let mut log = BR_LOG.lock().unwrap_or_else(|e| e.into_inner());
                        if log.len() < 500 { log.push(line); }
                    }
                }
            }
        }
        // ★슬롯0/1/2 지정(모드템/바닐라) → build Vec 목표를 그 카탈로그 인덱스로 (라이브 buy 경로, 슬롯3과 동일).
        //   아직 안 산 슬롯(owned<=si)만 → 게임이 그 인덱스를 향해 자연 빌드업. 바닐라=id, 모드템=이름스캔(레시피검증).
        if SLOT012_INJECT_ENABLED && is_player {
            let ctx012 = rd_u64(rsp_entry + 0x30) as usize;
            let bptr = rd_u64(athlete + 0x4e8) as usize; // 0.5.0 build ptr
            let blen = rd_u64(athlete + 0x4f0);          // 0.5.0 build len
            if ctx012 >= 0x10000 && bptr >= 0x10000 && blen >= 1 && blen <= 8 && readable(bptr, (blen as usize) * 8) {
                for si in 0u8..3 {
                    if (si as u64) >= blen { break; }         // build에 그 슬롯 없음
                    if owned > si as u64 { continue; }         // 이미 구매된 슬롯 → 늦음
                    let idx: Option<u64> = if let Some(vid) = slotN_vanilla_id(scope, champ, si) {
                        Some(vid)                              // 바닐라: id==catalog index (스캔 불요)
                    } else if let Some(mk) = slotN_item_key(scope, champ, si) {
                        scan_idx_cached(ctx012, mk.as_bytes()) // 모드템: 이름스캔+레시피검증
                    } else { None };
                    if let Some(t) = idx {
                        // ★멱등 가드(07-19): 이미 목표값이면 write 생략. 실측 53,890회 write 중 절대다수가
                        //   같은 athlete·같은 슬롯에 같은 값 재기록이었음 → 값 비교로 ~10회로 축소(핫패스 비용 제거).
                        if rd_u64(bptr + (si as usize) * 8) == t { continue; }
                        if writable(bptr + (si as usize) * 8, 8) {
                            wr_u64(bptr + (si as usize) * 8, t);
                            D_WROTE.fetch_add(1, Ordering::Relaxed); // ★진단: 실제 write 발생
                            BUY_WROTE_FIRE.fetch_add(1, Ordering::Relaxed);
                            if BUY_REPORT { BR_WROTE.fetch_add(1, Ordering::Relaxed); }
                            if LOG_ENABLED {
                                let key = format!("{}:s{}={}", champ, si, t);
                                let mut cl = SLOT012_LOG.lock().unwrap_or_else(|e| e.into_inner());
                                if cl.len() < 100 && !cl.iter().any(|c| c == &key) { cl.push(key); drop(cl);
                                    append_log("4items_buy4.txt", &format!("[slot012] {} owned={} → build[{}]={}", champ, owned, si, t)); }
                            }
                        }
                    }
                }
            }
        }
        // 진단: owned==3 도달 챔프 상태. (프로덕션: LOG_ENABLED 게이트)
        if LOG_ENABLED && owned == 3 {
            let mut cl = CHAMP_AT3.lock().unwrap_or_else(|e| e.into_inner());
            if cl.len() < 40 && !cl.iter().any(|c| c.as_str() == champ) {
                let bl = rd_u64(athlete + 0x4f0); // 0.5.0 build len (구 0x418)
                let manual = slot3_item_key(scope, champ).is_some();
                let bp = rd_u64(athlete + 0x4e8) as usize; // 0.5.0 build ptr (구 0x410)
                let (mut b0, mut b1, mut b2, mut b3) = (0u64, 0u64, 0u64, 0u64);
                if bp >= 0x10000 && readable(bp, 32) { b0 = rd_u64(bp); b1 = rd_u64(bp + 8); b2 = rd_u64(bp + 16); b3 = rd_u64(bp + 24); }
                let mx = MAX_OWNED4.load(Ordering::Relaxed);
                cl.push(champ.to_string()); drop(cl);
                append_log("4items_buy4.txt", &format!("[owned==3] champ={} build_len={} build=[{},{},{},{}] manual={} MAX_OWNED={}", champ, bl, b0, b1, b2, b3, manual, mx));
            }
        }
        // ★mode=3: 여기까지(슬롯0/1/2 지정 주입)만. 4번째 아이템(build 확장·신경망·강제구매)은 mode=4 전용 → 3칸=바닐라 3슬롯 유지.
        if slot_count() != 4 { return 0; }
        if !SHADOW_CALL_NAMES { return 0; }
        // ★0.5.0: build-extension(아래 __rust_realloc @0x25a56c0 호출). RVA_REALLOC 확정 → BUILD_EXTEND_ENABLED=true.
        //   OFF일 때만 passthrough(원본 3구매). 현재 ON = build Vec 3→4 실구매.
        if !BUILD_EXTEND_ENABLED { return 0; }
        // build Vec 3→4 realloc + build[3]=카탈로그 인덱스. resolver가 owned==3서 build[3]을 목표로 t1부터 빌드업.
        //   ★ RE: build Vec 값 = "카탈로그 인덱스"(아이템 id 아님). build[0]=게임이 넣은 유효 인덱스+레시피 有.
        //   메커니즘 검증: build[3] = build[0] 복사(확실히 유효). 되면 owned가 4로 감. 그다음 실제 4번째 인덱스 매핑.
        let mut build_len = rd_u64(athlete + 0x4f0); // 0.5.0 build len (구 0x418)
        if build_len == 3 && rd_u64(athlete + 0x4e0) == 3 { // 0.5.0 build cap (구 0x408)
            let ptr = rd_u64(athlete + 0x4e8) as usize; // 0.5.0 build ptr (구 0x410)
            if ptr >= 0x10000 && readable(ptr, 24) && writable(athlete + 0x4e0, 0x18) {
                let (b0, b1, b2) = (rd_u64(ptr), rd_u64(ptr + 8), rd_u64(ptr + 16));
                // ★ build[3] = ① 개인전술 수동지정 → ② 신경망 추천 → ③ distinct 바닐라 폴백.
                //   ①②는 아이템 "이름"으로 catalog 스캔해 인덱스+레시피검증(모드템 id≠인덱스라 이름스캔 필수).
                //   레시피 없는(기초템) 픽은 버리고 폴백(그대로 쓰면 FUN_141d5ab40 panic).
                let ctx = rd_u64(rsp_entry + 0x30) as usize;
                // ① 수동지정(개인전술) 우선 → ② 신경망(캐시) → 각각 이름스캔(캐시)+레시피검증으로 인덱스 획득.
                // ★팀게이트: player 팀만 수동지정(바닐라/모드템) 적용. 적팀은 van=manual=None → 신경망 폴백.
                let manual = if is_player { slot3_item_key(scope, champ) } else { None };
                let van = if is_player { slot3_vanilla_id(scope, champ) } else { None };
                let picked = if let Some(vid) = van {
                    Some(vid) // ★바닐라 지정: id==catalog index → 스캔 불요(견고, 0.5.0)
                } else if let Some(mk) = manual.as_ref() {
                    scan_idx_cached(ctx, mk.as_bytes()) // 모드템: 이름 스캔(ctx+0x30 수정으로 동작)
                } else {
                    // ★ 적팀 or 미지정: 신경망 fresh 호출(우리5+상대5+포지션 ctx). 캐시 안 함(라인업 무시=오답).
                    compute_auto_4th_id(athlete, champ)
                        .and_then(item_id_to_key).and_then(|k| scan_idx_cached(ctx, k.as_bytes()))
                };
                // ③ 폴백: build[0..2]와 다른 바닐라 최종템(레시피 확실, 바닐라 id==인덱스 확정).
                //   ★공격력 편향 수정: 구현은 항상 [0]=공격력(id4)부터 스캔 → 신경망 실패 시 적 4번째가 죄다 공격력.
                //   → 시작점을 champ 이름 FNV 해시로 분산(챔프별 결정적=리플레이 안전, 카테고리 고르게 분포).
                let t4 = picked.or_else(|| {
                    AUTO4_CNT[5].fetch_add(1, Ordering::Relaxed); // 진단: 신경망 None→바닐라폴백 발동 수
                    let mut h: u64 = 0xcbf29ce484222325;
                    for &b in champ.as_bytes() { h = (h ^ b as u64).wrapping_mul(0x100000001b3); }
                    let start = (h % 6) as usize;
                    (0..6).map(|k| VANILLA_FINAL[(start + k) % 6]).find(|&v| v != b0 && v != b1 && v != b2)
                });
                if let Some(t) = t4 {
                    let realloc: ReallocFn = core::mem::transmute(exe_base_addr() + RVA_REALLOC);
                    let np = realloc(ptr, 24, 8, 32);
                    if np >= 0x10000 && writable(np, 32) {
                        wr_u64(np + 24, t); // ★ build[3] = 수동/신경망 인덱스 or 바닐라 폴백
                        wr_u64(athlete + 0x4e8, np as u64); wr_u64(athlete + 0x4e0, 4); wr_u64(athlete + 0x4f0, 4); // 0.5.0 build ptr/cap/len
                        build_len = 4;
                        if LOG_ENABLED {
                            let src = if manual.is_some() && picked.is_some() { "manual" } else if picked.is_some() { "neural" } else { "vanilla" };
                            let mut cl = BUILD3_AT.lock().unwrap_or_else(|e| e.into_inner());
                            if cl.len() < 60 && !cl.iter().any(|c| c.as_str() == champ) { cl.push(champ.to_string()); drop(cl);
                                append_log("4items_buy4.txt", &format!("[build3] champ={} target_idx={} src={} build012=[{},{},{}]", champ, t, src, b0, b1, b2)); }
                        }
                    }
                }
            }
        }
        // ★ AUTO4_NATURAL: build[3] 목표만 넣고 강제 안 함 → 게임이 컴포넌트(t1)부터 자연 빌드업(정가 골드 차감).
        if AUTO4_NATURAL { return 0; }
        if DIAG_SCAN_OFF { return 0; } // ★진단 #4: realloc만 하고 스캔/shadow-call/forward 스킵(realloc 격리)
        if owned != 3 || build_len < 4 { return 0; }
        // ── owned==3 확정 → 4번째 아이템 키 결정 ──
        // 수동 지정(바닐라/모드) 우선. 없으면 AUTO: 신경망 forward로 build[0..3] 기준 최선의 4번째(전원 보편).
        let want_key = match slot3_item_key(scope, champ) {
            Some(k) => k,
            None => match compute_auto_4th_id(athlete, champ).and_then(item_id_to_key) {
                Some(k) => k,
                None => return 0, // 지정도 없고 신경망 4번째도 없음 → passthrough(정상 3구매)
            },
        };
        // ctx(스택 인자=[rsp_entry+0x30]) → coll(+0x20) → data(+8)/len(+0x10)
        //   (rsp_entry 기준: 프롤로그후 [rsp+0xa8]=rsp_entry-0x78+0xa8=+0x30. 옛 buy_replace도 +0x30 사용.)
        let ctx = rd_u64(rsp_entry + 0x30) as usize;
        if ctx < 0x10000 || !readable(ctx, 0x28) { return 0; }
        let coll = rd_u64(ctx + 0x30) as usize; // ★0.5.0: catalog collection 오프셋 ctx+0x20→+0x30 (RE 확정, 유일 변경)
        if coll < 0x10000 || !readable(coll, 0x18) { return 0; }
        let data = rd_u64(coll + 8) as usize;
        let len = rd_u64(coll + 0x10);
        if data < 0x10000 || len == 0 || len > 100000 || !readable(data, (len as usize) * 16) { return 0; }
        // 이름 스캔: elem={data[i*16]=edata, +8=vtable}. name=vtable[0x50](edata) → {chars@+8, len@+0x10}
        let mut found: Option<u64> = None;
        let mut names_log = String::new();
        let mut i = 0u64;
        while i < len {
            let e = data + (i as usize) * 16;
            let edata = rd_u64(e) as usize;
            let evt = rd_u64(e + 8) as usize;
            if edata >= 0x10000 && evt >= 0x10000 && readable(evt, 0x60) {
                let namefn = rd_u64(evt + 0x58) as usize;
                if namefn >= 0x10000 {
                    let f: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(namefn);
                    let nobj = f(edata);
                    if nobj >= 0x10000 && readable(nobj, 0x18) {
                        let chars = rd_u64(nobj + 8) as usize;
                        let nlen = rd_u64(nobj + 0x10) as usize;
                        if chars >= 0x10000 && nlen > 0 && nlen <= 64 && readable(chars, nlen) {
                            let nm = std::slice::from_raw_parts(chars as *const u8, nlen);
                            if !BUY4_LOGGED.load(Ordering::Relaxed) && names_log.len() < 3000 {
                                names_log.push_str(&String::from_utf8_lossy(nm)); names_log.push(' ');
                            }
                            if nm == want_key.as_bytes() { found = Some(i); break; }
                        }
                    }
                }
            }
            i += 1;
        }
        if !BUY4_LOGGED.swap(true, Ordering::Relaxed) {
            write_log("4items_buy4.txt", &format!("[4th] champ={} want={} coll_len={} found={:?}\n  names: {}\n", champ, want_key, len, found, names_log));
        }
        let Some(rdx) = found else { return 0; };
        *saved.add(1) = rdx;   // rdx = 아이템 인덱스
        *saved.add(6) = 1;     // rax = 1(성공)
        1
    }));
    perf::rec_tl(perf::T_BUY_ALL, __bt);
    r.unwrap_or(0)
}

// buy_item replace-detour 설치(스텁: mov r10,rsp; push rax r11 r10 r9 r8 rdx rcx; cap_fn(rcx=saved,rdx=rsp_entry)).
// ★0.5.6 체인 후킹(2026-08-20, CLAUDE §3 규칙): riot_items_tfm2 v0.9.2(0.5.6판)가 같은 buy(0xebca20)를
//   먼저 후킹(dll 내 RVA 상수 5회 실측) → 진입부가 `48 b8 <tgt> ff e0` = 프롤로그 mismatch로 우리 설치가
//   영구 실패(BUY_PROBE_INSTALLED=2)하던 실사고. chain=Some(외부 12B)면:
//   ①PASSTHROUGH 연속부 = 원본 19B 대신 외부 12B 점프(movabs+jmp = 위치무관 재배치 안전 — riot 스텁이
//     원본 프롤로그 실행·복귀를 담당) ②게임 함수엔 12B만 덮어씀(외부 패치 잔여부 불훼손).
//   HANDLED(반환 1=완전대체)는 체인에서도 riot 핸들러를 건너뜀 = 우리 주입 우선(riot 빌드파일은 비어 있어 실손실 없음).
//   ⛔재체인 금지(§3): 설치는 1회 확정 — 매프레임 진입부 재검증 후 재설치 절대 금지(상호 체인 사이클 실사고 07-18).
unsafe fn install_replace_buy(rva: usize, orig_len: usize, cap_fn: usize, chain: Option<[u8; 12]>) -> Result<usize, &'static str> {
    let mbase = exe_base_addr();
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, orig_len + 4) { return Err("fn unreadable"); }
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);
    s.extend_from_slice(&[0x50, 0x41,0x53, 0x41,0x52, 0x41,0x51, 0x41,0x50, 0x52, 0x51]);
    s.extend_from_slice(&[0x48,0x89,0xe1]);
    s.extend_from_slice(&[0x4c,0x89,0xd2]);
    s.extend_from_slice(&[0x48,0x83,0xec,0x20]);
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff,0xd0]);
    s.extend_from_slice(&[0x48,0x83,0xc4,0x20]);
    s.extend_from_slice(&[0x48,0x85,0xc0]);
    s.extend_from_slice(&[0x74,0x0c]);
    s.extend_from_slice(&[0x59, 0x5a, 0x41,0x58, 0x41,0x59, 0x41,0x5a, 0x41,0x5b, 0x58, 0xc3]); // HANDLED: pop..ret
    s.extend_from_slice(&[0x59, 0x5a, 0x41,0x58, 0x41,0x59, 0x41,0x5a, 0x41,0x5b, 0x58]);       // PASSTHROUGH: pop
    let patch_len = match chain {
        Some(foreign12) => {
            // 체인: 외부 훅 12B(movabs rax,tgt; jmp rax)를 연속부로 재배치 — riot 스텁 → 원본 순으로 이어짐.
            s.extend_from_slice(&foreign12);
            12
        }
        None => {
            let mut orig = vec![0u8; orig_len];
            core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
            s.extend_from_slice(&orig);
            s.extend_from_slice(&[0xff,0x25,0x00,0x00,0x00,0x00]); s.extend_from_slice(&ret_addr.to_le_bytes());
            orig_len
        }
    };
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; patch_len];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, patch_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, patch_len);
    VirtualProtect(fn_addr, patch_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, patch_len);
    Ok(stub)
}
// ===========================================================================
//  ★★ scene 직독 = 결정적 팀게이트 (후킹 없음, ghidra-re + crm 앵커 0.5.0_3 확정)
//  라이브 매치 중 클라 scene(ClientScene::InGame, tag=9)의 match_info에 두 팀 team_id + is_team1_blue 상주.
//  메인스레드 post_update서 매프레임 직독 → player_team_id()와 비교 → PLAYER SIDE(0/1) 확정.
//  db 절대 오프셋(0.5.0_3, -0xA0 일괄시프트 3중정합): scene태그(u32)@+0x1338==9 /
//  team1 tag(u64,Normal=0)@+0x17A0·id@+0x17A8 / team2 tag@+0x17C0·id@+0x17C8 / is_team1_blue(u8)@+0x1900.
//  is_team1_blue는 세트별 진영스왑 반영해 갱신됨 → 읽는 시점 항상 현재 세트 기준.
//  ⚠구 GameStart 패킷 deserializer 훅(0x3217f0)=죽은길(라이브 단일프로세스선 미발화, crossbeam 직접전달) → 제거.
// ===========================================================================
const SCENE_GATE_ENABLED: bool = true; // ★v5(07-11): 라이브 확정(tid) 후 side 판별을 scene 직독으로 → ON. update_scene_side 매프레임(메인스레드)서 SCENE_SIDE 갱신.
// ★★확정(07-11 인게임): sim athlete+0x820 side는 blue=0/red=1 고정. 관전 경기(내팀 blue)서 KT Aiming=meiling이
//   sim side1(red)로 덤프됨 → side0=blue=내팀 확인. 따라서 scene player↔sim side 매핑 = blue를 side0으로.
//   ⚠이건 진영 무관 고정 매핑(상수 반전 아님) — scene team_id↔pid 매칭이 진영 바뀌어도 올바른 sim side 반환.
const SCENE_BLUE_IS_SIDE0: bool = true; // blue팀 = sim side0 (인게임 확정). update_scene_side가 (s0,s1)=(blue,red)로 pid 매칭.
static SCENE_SIDE: AtomicU64 = AtomicU64::new(u64::MAX); // 0/1=라이브매치 player side, MAX=미확정(비매치/관전아님)
static LIVE_DB: AtomicU64 = AtomicU64::new(0);   // ★v6: InGame post_update가 저장하는 db 절대주소(스폰훅 조기 side판정용)
static LIVE_PID: AtomicU64 = AtomicU64::new(u64::MAX); // ★v6: 저장된 PLAYER_TEAM_ID
// ★v6 경량 side-only 판정(스폰훅=sim스레드서 호출, VEH-safe read만, 파일I/O·락 없음).
//   scene tag9 + team_id Normal + is_team1_blue + pid 매칭 → player side(0/1). update_scene_side와 동일 오프셋.
unsafe fn quick_scene_side(db: usize, pid: u64) -> Option<u64> {
    if db < 0x10000 || pid == u64::MAX { return None; }
    if safe_read_u64(db + 0x1338).map(|v| v & 0xffff_ffff) != Some(9) { return None; }
    let t1_tag = safe_read_u64(db + 0x17A0)?;
    let t2_tag = safe_read_u64(db + 0x17C0)?;
    if t1_tag != 0 || t2_tag != 0 { return None; } // Normal(team_id)만
    let t1 = safe_read_u64(db + 0x17A8)?;
    let t2 = safe_read_u64(db + 0x17C8)?;
    SCENE_T1.store(t1, Ordering::Relaxed); // ★08-07: Game 프로브 탐색 키로 공개
    SCENE_T2.store(t2, Ordering::Relaxed);
    let blue_b = safe_read_u64(db + 0x1900)? & 0xff;
    let t1_blue = blue_b != 0;
    let (blue, red) = if t1_blue { (t1, t2) } else { (t2, t1) };
    let (s0, s1) = if SCENE_BLUE_IS_SIDE0 { (blue, red) } else { (red, blue) };
    if s0 == pid { Some(0) } else if s1 == pid { Some(1) } else { None }
}
static PID_EVER_VALID: AtomicU64 = AtomicU64::new(0); // player_team_id()가 유효(1~9999)를 한 번이라도 반환했나
// ★2026-07-30: **0 이 아닌** 유효 pid 를 본 적 있나. 1이면 이후 0 보고를 무시(pid 후퇴 방지 — 실측으로
//   같은 세이브가 시점에 따라 105/0 을 오가는 것이 확인됐고, 0 을 믿으면 팀 게이트가 깨진다).
static PID_NONZERO_SEEN: AtomicU64 = AtomicU64::new(0);
static MY_PT_N: AtomicU64 = AtomicU64::new(0);        // 게시 대상 팀의 champion_personal_tactics 엔트리 수(내팀 검증용)
static MY_TRUST_SKIP: AtomicU64 = AtomicU64::new(0);  // pid=0 + PT 미달로 MY_ATHLETES 게시를 보류한 횟수
static PID_OBS_ZERO: AtomicU64 = AtomicU64::new(0);    // player_team_id()가 0을 반환한 관측 수
static PID_OBS_NONZERO: AtomicU64 = AtomicU64::new(0); // 비0 유효값 관측 수
static PID_SKIP_CT: AtomicU64 = AtomicU64::new(0);     // 조합테스트 컨텍스트(sim 중 or 팝업 열림)에서 0 보고를 무시한 횟수
static PID_ZERO_CLEAN: AtomicU64 = AtomicU64::new(0);  // ★조합테스트와 무관한 InGame 에서 pid=0 을 관측한 횟수
                                                       //   (≥600 이면 "진짜 팀 id 가 0 인 세이브"로 인정 = MY_ATHLETES 게시 허용)
static SCENE_DIAG_LAST: AtomicU64 = AtomicU64::new(u64::MAX); // 진단 상태지문(변할 때만 파일 rewrite)
static LINK_SCAN_DONE: AtomicBool = AtomicBool::new(true); // 포인터 스캔 폐기(match_id 우연히트 확인으로 종결)
static BUY_SIMS: Mutex<Vec<(usize, String)>> = Mutex::new(Vec::new()); // buy 훅서 본 SimState(base-0x840) + champ0 (scene 판정 중)
// ═══ FLOW 진단: scene 태그 전환 + SimState 탄생/tag9 활동 타임라인 (한 경기 흐름 전체) ═══
static CUR_TAG: AtomicU64 = AtomicU64::new(u64::MAX);
static FLOW_SIMS: Mutex<Vec<(usize, u64, bool)>> = Mutex::new(Vec::new()); // (sim, first_tag, tag9_logged)
fn scene_player_side() -> Option<u64> {
    if !SCENE_GATE_ENABLED { return None; } // OFF면 로스터 폴백 사용
    match SCENE_SIDE.load(Ordering::Relaxed) { v @ 0..=1 => Some(v), _ => None }
}
// ★scene 원시값(진단 덤프용) — update_scene_side가 매프레임 갱신.
static SCENE_T1: AtomicU64 = AtomicU64::new(u64::MAX);
static SCENE_T2: AtomicU64 = AtomicU64::new(u64::MAX);
static SCENE_BLUEB: AtomicU64 = AtomicU64::new(u64::MAX);
// ★★타입드 헤지(07-11): SDK db.replay_view→match_replays→blue/red_team_id (side0=blue/side1=red 정본, MatchReplayData).
//   scene 직독(SCENE_T1/T2/BLUEB)과 크로스체크 = "두 소스가 완전히 같은 team_id를 주나" 검증용. DIAG_ENABLED 게이트.
//   ⚠MatchReplayData=완성/기록된 매치라 라이브 진행 중엔 미기록(MRD=MAX) 가능 → 비교는 매치 기록 후 유효.
const DIAG_BUY_OFF: bool = false; // buy 주입 전체 스위치(true=주입/식별 OFF)
fn install_replace_4th() {
    if DIAG_BUY_OFF { return; }
    if BUY_PROBE_INSTALLED.load(Ordering::Relaxed) != 0 { return; }
    let base = unsafe { GetModuleHandleW(core::ptr::null()) } as usize;
    if base == 0 { return; }
    let fn_addr = base + RVA_BUY_ITEM;
    if !unsafe { readable(fn_addr, 12) } { BUY_PROBE_INSTALLED.store(2, Ordering::Relaxed);
        append_log("4items.txt", &format!("[{}ms] buy_item 진입부 unreadable → replace 미설치", now_ms())); return; }
    let entry: [u8; 12] = core::array::from_fn(|i| unsafe { *((fn_addr + i) as *const u8) });
    let ok = entry == BUY_PROLOGUE;
    // ★0.5.6 체인(2026-08-20): riot_items_tfm2가 같은 buy를 먼저 후킹하면 진입부 = movabs+jmp.
    //   그 12B를 연속부로 담아 체인 설치(우리 detour → riot 스텁 → 원본). 상세 = install_replace_buy 주석.
    let foreign = !ok && entry[0] == 0x48 && entry[1] == 0xb8 && entry[10] == 0xff && entry[11] == 0xe0;
    if !ok && !foreign {
        BUY_PROBE_INSTALLED.store(2, Ordering::Relaxed);
        append_log("4items.txt", &format!("[{}ms] buy_item 프롤로그 mismatch(외부훅 형태도 아님) → replace 미설치 entry={:02x?}", now_ms(), entry)); return;
    }
    // orig_len=19: 0.5.1 신 프롤로그 5push(7)+sub rsp,0x50(4)=11B는 jmp패치(12B)를 못 덮음 → 다음 클린경계 11+mov rax,[rsp+0xa8](8)=19B로 재배치.
    let chain = if foreign { Some(entry) } else { None };
    match unsafe { install_replace_buy(RVA_BUY_ITEM, 19, buy_replace_ctx as usize, chain) } {
        Ok(_) => {
            BUY_PROBE_INSTALLED.store(if foreign { 3 } else { 1 }, Ordering::Relaxed);
            if foreign {
                let tgt = u64::from_le_bytes(entry[2..10].try_into().unwrap());
                append_log("4items.txt", &format!("[{}ms] buy_item 체인 설치 OK — 외부훅(tgt={:#x}, riot_items 추정) 위에 체인", now_ms(), tgt));
            }
        }
        Err(e) => { BUY_PROBE_INSTALLED.store(2, Ordering::Relaxed);
            append_log("4items.txt", &format!("[{}ms] buy_item replace 설치 실패: {}", now_ms(), e)); }
    }
}

// owned 3-캡 패치: run_tick_ext 내 `cmp qword[rax+0x3d0], 3`(보유>3 스탯적용 스킵)의 imm8 3→4.
//   4번째 아이템 스탯이 적용되게. (0.4.14+핫픽스 RVA, [[tfm2-item-slot-count]] 패치①)
unsafe fn patch_owned_cap() -> String {
    let base = exe_base_addr();
    // 0.5.0: cmp qword[rsi+0x458],3 = 48 83 BE 58 04 00 00 03 (구 0.4.14: cmp [rax+0x3d0],3). imm8 @ sig+7.
    // 0.5.2(2026-07-22): 컨테이너(0x2234430→0x233e9d0)가 리팩터되며 **레지스터 RSI→R15** (48 83 be → 49 83 bf).
    //   disp(struct 오프셋 0x458)·imm(3)은 불변. `cmp qword[reg+0x458],3` 형태는 신 exe 전체에서 이 1곳뿐(유일).
    // 0.5.3(2026-07-29): 레지스터가 R15→**RSI**로 회귀(49 83 bf → 48 83 be). disp 0x458·imm 3은 불변.
    //   `cmp qword[reg+0x458],3` 형태는 신 exe .text 전체에서 **유일 1건**(바이트스캔 실측) = 오식별 불가.
    let sig = base + 0x10da9a9; // 0.5.6(구0.5.5=0x15206a9). 컨테이너델타(owner 0x151db50→0x1549b20·BYTE=SAME)·orig 4883bea804000003 실측 일치(athlete owned len 0x4a8 불변). // 0.5.5(구0.5.4=0x1420b29). `cmp qword[rsi+0x4a8],3` = 신 exe .text 전체 유일 1건 + athlete owned len 0x448→0x4a8(+0x60 시프트, ctor 정렬 확증). // 0.5.4(구0.5.3=0xf24a39). `cmp qword[reg+0x448],3` = 신 exe 전체 유일 1건 + 컨테이너 mode.rs panic-loc 23개 동일 + 함수내 오프셋 0x2a59->0x2b29. // 0.5.3(구0.5.2=0x2341440). 컨테이너 0x233e9d0→0xf21fe0.
    let imm = base + 0x10da9b0; // cmp 의 imm8 (=sig+7) — 0.5.6(구0.5.5=0x15206b0)
    let expect = [0x48u8, 0x83, 0xbe, 0xa8, 0x04, 0x00, 0x00, 0x03]; // 0.5.5: cmp qword[rsi+0x4a8],3 (athlete owned len 0x448->0x4a8). ~~0.5.4: 0x448~~
    if !readable(sig, 8) { return "owned_cap: unreadable".into(); }
    for i in 0..8 { if *((sig + i) as *const u8) != expect[i] {
        return format!("owned_cap: sig mismatch @+{} = {:#04x}", i, *((sig + i) as *const u8));
    }}
    const RWX: u32 = 0x40;
    let mut old = 0u32;
    if VirtualProtect(imm, 1, RWX, &mut old) == 0 { return "owned_cap: VirtualProtect fail".into(); }
    *(imm as *mut u8) = 0x04;
    VirtualProtect(imm, 1, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), imm, 1);
    "owned_cap: patched 3->4".into()
}

// ★슬롯4 자연구매 게이트 패치: resolver FUN_142052dd0 내 owned>2 전용 has_recipe 게이트
//   `0x142052e76 jbe`(0x76) → `jmp`(0xEB). owned>2도 owned<=2(슬롯1~3)와 동일 경로 = 기초재료부터
//   자연 빌드업 허용. owned<=2엔 무영향(원래 jbe 스킵, 같은 목적지). build_len<4면 게이트①서 먼저
//   중단하니 slot=3/타모드엔 무해. (ghidra-re 확정, 0.4.14+핫픽스)
unsafe fn patch_gate3() -> String {
    let base = exe_base_addr();
    // 0.5.0: jbe @ 0x1e4bd36(구 0x2052e76). sig 시작 = jbe-9. 76→EB(JMP)로 owned>2 게이트 무력화.
    let sig = base + 0xdf57f8;          // 0.5.6(구0.5.5=0xeb2fa8). 컨테이너델타(resolver 0xeb2d30→0xebcb10·BYTE=SAME·off 0x278)·orig 48837c24600276 실측 일치(spill rsp+0x60 불변). // 0.5.5(구0.5.4=0xe76b1e). 신 resolver 0xeb2d30(buy 0xeb2c40이 호출) 내부 유일 gate, 함수내 off 0x24e→0x278, spill 슬롯 rsp+0x40→rsp+0x60. // 0.5.3(구0.5.2=0x211e428): resolver 컨테이너 0x211e150→**0xd0c770**(buy 0xd0c680이 직접 호출). 스필 슬롯이 rsp+0x78→**rsp+0x40**으로 이동했고 `cmp qword[rsp+0x40],2;jbe` 형태는 신 exe 전체 **유일 1건**(바이트스캔 실측). ↓0.5.2 이력: (구0.5.1=0x1f01448): resolver 컨테이너 0x1f01170→0x211e150(스켈레톤 UNIQUE, +0x21cfe0) 동일 오프셋 +0x2d8, 7B 시그 바이트동일(BYTE-OK). ↓0.5.1 이력: (구0.5.0_3=0x1fb8cdd, ghidra-re HIGH 재-ID). resolver 후계 FUN_141f01170 내부. owned_count가 [rsp+0x78]로 spill돼 시퀀스가 'cmp qword[rsp+0x78],2;jbe'로 재작성됨(구 'mov rsi,[rsp+0x40];jbe').
    let jbe = base + 0xdf57fe; // 0.5.6(구0.5.5=0xeb2fae, =sig+6). resolver 0xebcb10 내. // 0.5.5(구0.5.4=0xe76b24, =sig+6). resolver 0xeb2d30 내 +0x27e. // 0.5.4(구0.5.3=0xd0c9c4). resolver 0xe768d0 내 +0x24e = 구 exe와 동일 함수내 오프셋, 10B 바이트 완전동일, exe 전체 유일.          // 0.5.3 jbe 의 opcode 바이트 (=sig+6, 구0.5.2=0x211e42e). owned≤2→점프, >2 fall-through(has_recipe 추가검사).
    let expect = [0x48u8, 0x83, 0x7c, 0x24, 0x60, 0x02, 0x76]; // 0.5.5: cmp qword[rsp+0x60],2 ; jbe (spill rsp+0x40->rsp+0x60). ~~0.5.4: rsp+0x40~~
    if !readable(sig, 7) { return "gate3: unreadable".into(); }
    for i in 0..7 { if *((sig + i) as *const u8) != expect[i] {
        return format!("gate3: sig mismatch @+{} = {:#04x}", i, *((sig + i) as *const u8));
    }}
    const RWX: u32 = 0x40;
    let mut old = 0u32;
    if VirtualProtect(jbe, 1, RWX, &mut old) == 0 { return "gate3: VirtualProtect fail".into(); }
    *(jbe as *mut u8) = 0xEB;
    VirtualProtect(jbe, 1, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), jbe, 1);
    "gate3: patched jbe->jmp (owned>2 게이트 무력화)".into()
}



// ═══════════════════════════════════════════════════════════════════════════
//  ★★게임 버전 게이트 — 0.5.3 전용. 다른 버전이면 **모든 기능을 자동 비활성**.
// ═══════════════════════════════════════════════════════════════════════════
//  왜 필요한가: 이 모드는 하드코딩 RVA 12종 + 바이트패치 2곳 + 구조체 오프셋 다수에 의존한다.
//  0.5.4 로 패치되면 그 주소들이 전부 어긋나 **엉뚱한 코드를 후킹/패치**하게 된다
//  (프롤로그 검증이 있는 훅은 미설치로 끝나지만, 검증이 약한 곳은 크래시·데이터 손상 위험).
//  ⟹ 아예 init 단계에서 버전을 확인하고, 불일치면 훅·패치를 **하나도 설치하지 않는다**.
//
//  판정 2중(둘 다 통과해야 활성):
//   ①exe 파일 크기 — 0.5.3 = 74,970,624B. 버전마다 확실히 달라지는 값이고 읽기 비용이 없다.
//   ②핵심 훅 3곳의 진입부 프롤로그 실측 — 크기가 우연히 같은 리패키징이라도 코드가 다르면 걸러진다.
//  ⚠느슨한 검사(예 크기만)로 하면 핫픽스에서 오작동할 수 있어 프롤로그까지 본다.
// ⚠0.5.6 실사고(2026-08-20): 0.5.6 재핀이 RVA·구조체만 갱신하고 **이 게이트 상수를 누락** →
//   게임에서 모드 전체 자기비활성("4번째 아이템 안 나옴" 제보·version_gate.txt로 특정).
//   패치 대응 체크리스트에 "버전 게이트 상수(exe 크기)"도 재핀 축으로 포함할 것.
const GAME_EXE_SIZE_056: u64 = 77_111_808; // 0.5.6 (실측 확인). ~~0.5.5=76_957_696~~ ~~0.5.4=75_936_256~~
static VERSION_OK: AtomicBool = AtomicBool::new(false);
static VERSION_MSG: Mutex<String> = Mutex::new(String::new());
/// 0.5.6 인지 판정. init 에서 1회 호출하고 결과를 VERSION_OK 에 남긴다.
fn check_game_version() -> bool {
    let mut why = String::new();
    // ① exe 크기
    let size_ok = match exe_path().and_then(|p| fs::metadata(p).ok()) {
        Some(m) => {
            let sz = m.len();
            if sz == GAME_EXE_SIZE_056 { true }
            else { why = format!("exe 크기 불일치: {}B (0.5.6 = {}B)", sz, GAME_EXE_SIZE_056); false }
        }
        None => { why = "exe 경로/메타데이터 실패".into(); false }
    };
    // ② 핵심 훅 진입부 프롤로그 (크기가 같아도 코드가 다르면 여기서 걸린다)
    //  ⚠⚠**체인 후킹 예외 필수**(2026-07-30 실사고): launcher 등은 **다른 모드(serpen)가 먼저 후킹**해
    //    진입부가 `48 b8 <tgt> ... ff e0`(movabs+jmp)로 이미 덮여 있는 게 **정상 동작**이다.
    //    이걸 "버전 불일치"로 오판해 모드 전체가 비활성됐다(유저 제보 "4칸모드 갑자기 안 됨").
    //    ⟹ 진입부가 **외부 훅 형태면 통과**로 인정하고, 원본 프롤로그일 때만 바이트 대조한다.
    //  ⚠또한 우리 자신이 이미 설치한 경우도 같은 형태다(재init·핫리로드).
    let proto_ok = if size_ok {
        let base = exe_base_addr();
        if base == 0 { why = "module base 0".into(); false } else {
            // ★검사 대상 = **모드 간 공동 후킹이 없는 곳만**.
            //   launcher(CL_LAUNCHER_RVA)는 serpen 등과 체인 후킹하는 지점이라 진입부가 남의 훅으로
            //   덮여 있을 수 있고 init 호출 순서에 따라 상태가 달라진다 ⟹ **버전 판정 근거로 부적합**.
            //   buy/seedctor 는 이 모드 전용이라 init 시점엔 항상 원본 프롤로그다.
            let checks: [(&str, usize, &[u8]); 2] = [
                ("BUY",      RVA_BUY_ITEM, &BUY_PROLOGUE),
                ("SEEDCTOR", SEEDCTOR_RVA, &SEEDCTOR_PROLOGUE),
            ];
            let mut ok = true;
            for (nm, rva, want) in checks.iter() {
                let a = base + rva;
                if !unsafe { readable(a, want.len().max(12)) } {
                    why = format!("{} 진입부 읽기 실패 @{:#x}", nm, rva); ok = false; break;
                }
                // 이미 훅된 진입부(movabs rax,imm64 ; jmp rax) = 정상(우리 또는 타 모드) → 검사 통과
                let hooked = unsafe {
                    *(a as *const u8) == 0x48 && *((a + 1) as *const u8) == 0xb8
                        && *((a + 10) as *const u8) == 0xff && *((a + 11) as *const u8) == 0xe0
                };
                if hooked { continue; }
                let hit = (0..want.len()).all(|i| unsafe { *((a + i) as *const u8) } == want[i]);
                if !hit { why = format!("{} 프롤로그 불일치 @{:#x}", nm, rva); ok = false; break; }
            }
            ok
        }
    } else { false };
    let ok = size_ok && proto_ok;
    *VERSION_MSG.lock().unwrap_or_else(|e| e.into_inner()) =
        if ok { "0.5.6 확인 — 정상 활성".to_string() }
        else { format!("★버전 불일치 → 모드 전체 비활성 ({})", why) };
    VERSION_OK.store(ok, Ordering::Relaxed);
    ok
}
/// 게이트 통과 여부(런타임 훅/패치 진입부에서 조회).
#[inline]
fn version_ok() -> bool { VERSION_OK.load(Ordering::Relaxed) }

fn init(_ctx: &GameCtx) -> ModRegistration {
    // ★★버전 게이트: 0.5.6 이 아니면 **훅·패치를 하나도 설치하지 않고** 빈 등록만 반환한다.
    //   (하드코딩 RVA·바이트패치·구조체 오프셋 의존이라 다른 버전에선 오작동 위험)
    if !check_game_version() {
        let msg = VERSION_MSG.lock().unwrap_or_else(|e| e.into_inner()).clone();
        // LOG_ENABLED 무관하게 1회 남긴다(유저가 왜 비활성인지 알 수 있게).
        if let Some(d) = mod_dir() { let _ = fs::create_dir_all(&d);
            let _ = fs::write(d.join("version_gate.txt"),
                format!("{}

이 모드는 게임 0.5.6 전용입니다.
게임이 업데이트되면 모드 업데이트를 기다려 주세요.
", msg)); }
        // 등록만 하고 **확장(extension)·훅·패치를 하나도 붙이지 않는다** = 완전 비활성.
        return ModRegistration::new(MOD_ID);
    }
    let mode = load_mode();
    if mode == 4 {
        let r = unsafe { patch_owned_cap() };
        append_log("4items.txt", &format!("[{}ms] {}", now_ms(), r));
        let rg = unsafe { patch_gate3() }; // ★슬롯4 자연구매 게이트 무력화
        append_log("4items.txt", &format!("[{}ms] {}", now_ms(), rg));
    }
    append_log("4items.txt", &format!("[{}ms] init tfm2_4items (통합: item_tactics 엔진 + 4번째, mode={}칸)", now_ms(), mode));
    // uinj(item3/slot3 UI 주입) 로그 경로 설정. (MODE4/IN_MATCH_UI 는 load_mode 및 기본값.)
    if let Some(d) = mod_dir() {
        uinj::set_log(d.join("4items_uinj.txt").to_string_lossy().into_owned());
        uinj::DBG.store(LOG_ENABLED, Ordering::Relaxed);
    }
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(ItemTacticsExt);
    reg.set_server_extension(ItemTacticsServerExt);
    reg
}
declare_mod!(init);
