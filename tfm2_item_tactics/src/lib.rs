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
const FN_DD_SETOPT_RVA: usize = 0x1bfc80; // 0.5.3(구0.5.2=0x242f250). ghidra-re 확정: 직접 콜러 103개로 구 exe와 완전 일치 + 오프셋 지문 4종(+0x1788 selected / +0x1528·0x1530·0x1538 옵션Vec / +0x1570·0x1578 콜백 / 원소 0xf8 / 입력 stride 0x28) 전부 불변. ⚠프롤로그는 변경됨(아래 dd_addr_valid expect 갱신).

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
// ★임시 진단(LOG_ENABLED 무관): delegate 통합 SEL/PT/cur 덤프. 원인 확정 후 false.
// 0.5.0 마이그: OFF (진단 종료 + MR_* 리플레이 오프셋 미확정 → dump_replay_item_counts 게이트오프).
const DELEGATE_DIAG: bool = false; // 프로덕션 OFF(dbg_write·C6 seam 카운터 파일 로그 게이트).
fn dbg_write(name: &str, content: &str) {
    if !DELEGATE_DIAG { return; }
    if let Some(d) = mod_dir() { let _ = fs::create_dir_all(&d); let _ = fs::write(d.join(name), content); }
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
// 1회성 검증 덤프(키/ID/플래그) — LOG_ENABLED 와 무관하게 씀.
// ★배포 OFF(2026-07-22): 양방향 실증 완료로 규칙 확정 — riot **비활성** 시 104개 전부 raw=0(X),
//   riot **활성** 시 110개 전부 raw=포인터(O). 판정 오류 가능성 소멸 → 덤프 불필요.
const ACTIVE_DUMP: bool = false;

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
        write_log("item_tactics_moditems.txt", &s); return;
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
    if ACTIVE_DUMP {
        // LOG_ENABLED 무관 1회성 검증 덤프: +0x190 원시값까지 남겨 규칙 오판을 판별 가능하게.
        let mut d = format!("[{}ms] ModItemEntry +{:#x} 활성판정 덤프  buf={:#x} stride={:#x} cnt={} 활성={}\n",
            now_ms(), MODITEM_ACTIVE_OFF, buf, st, cnt, n_act);
        d.push_str("  ID | act | raw(+0x190)        | key\n");
        for (i, k) in keys.iter().enumerate() {
            let raw = safe_read_u64(buf + i * st + MODITEM_ACTIVE_OFF);
            d.push_str(&format!("  {:>3} |  {}  | {:>18} | {}\n", 30 + i,
                if actives.get(i).copied().unwrap_or(true) { "O" } else { "X" },
                raw.map(|v| format!("{:#x}", v)).unwrap_or_else(|| "READ-FAIL".into()), k));
        }
        d.push_str(&diag); // 후보 배열 전량(활성수 포함) — 오채택 여부 판별용
        if let Some(p) = mod_dir() { let _ = fs::write(p.join("item_tactics_active.txt"), &d); }
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

// ── 대시보드 추천 빌드 1회 반영 (2026-07-22 추가) ────────────────────────────
//  용도: TFM2.gg 대시보드가 통계로 뽑은 추천 빌드를 개인전술 드롭다운의 **초기 선택값**으로 밀어넣는다.
//  왜 별도 파일인가: 유저 선택(item_tactics_sel.txt)과 섞으면 어느 쪽이 손으로 고른 값인지 알 수 없다.
//    추천은 item_tactics_recommend.txt 로 분리하고, 반영 여부는 내용 해시(.applied)로 판정한다.
//  동작: 전술화면을 열 때 추천 파일 해시가 직전에 반영한 것과 다르면 **그때 1회만** SEL 에 덮어쓴다.
//    - 해시가 같으면 아무것도 안 함 ⟹ 그 사이 유저가 손으로 바꾼 값이 그대로 살아남는다.
//    - 화면을 닫으면 OPTS_INJECTED 가 리셋되므로, 게임을 켜 둔 채 대시보드를 갱신해도
//      전술화면을 다시 열기만 하면 반영된다(게임 재시작 불필요).
//  ⚠바닐라 카테고리는 delegate 가 champion_personal_tactics 로 이미 처리한다(PT_SNAPSHOT 경로).
//    이 파일은 **PT 에 담을 수 없는 모드 아이템** 지정을 위한 것이다(바닐라 토큰 1~6 도 받긴 한다).
//  원복: RECO_ENABLED=false 로 끄거나 추천 파일을 지우면 즉시 기존 동작으로 돌아간다.
// ⚠2026-07-22 OFF: 추천 산출식이 미완(조합 단위 집계라 표본 1판짜리가 1위로 뽑힘) → 유저 수동선택을
//   덮는 사고가 실제로 발생(sel.txt 74줄→250줄). 산출식을 아이템 단위 shrinkage+lift 로 바꾼 뒤 재개.
const RECO_ENABLED: bool = false;
fn reco_path() -> Option<PathBuf> { Some(mod_dir()?.join("item_tactics_recommend.txt")) }
fn reco_stamp_path() -> Option<PathBuf> { Some(mod_dir()?.join("item_tactics_recommend.applied")) }

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

// 전술화면 진입 시 호출. 새 추천이 있으면 SEL 에 반영하고 true 를 반환한다.
fn apply_recommendations() -> bool {
    if !RECO_ENABLED { return false; }
    let Some(p) = reco_path() else { return false; };
    // 파일이 없으면 조용히 skip — 대시보드를 안 쓰는 사용자의 정상 상태다.
    let Ok(txt) = fs::read_to_string(&p) else { return false; };
    let hash = fnv1a64(txt.as_bytes()).to_string();
    let stamp = reco_stamp_path();
    let prev = stamp.as_ref().and_then(|s| fs::read_to_string(s).ok());
    if prev.as_deref().map(str::trim) == Some(hash.as_str()) {
        return false; // 이미 반영한 추천 → 유저의 이후 수동 변경을 보존한다.
    }

    // 파싱은 sel 파일과 동일 포맷(`champ slot token`)이라 기존 해석기를 그대로 쓴다.
    let mut rows: Vec<(String, u8, String)> = Vec::new();
    for line in txt.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 { continue; }
        let Ok(slot) = parts[1].parse::<u8>() else { continue; };
        if slot as usize >= ITEM_SLOTS { continue; }
        rows.push((parts[0].to_string(), slot, normalize_token(parts[2])));
    }
    if rows.is_empty() {
        // 빈 추천도 "반영됨"으로 찍어야 매 프레임 재시도하지 않는다.
        if let Some(s) = stamp { let _ = fs::write(s, &hash); }
        return false;
    }

    let mut applied = 0usize;
    with_sel(|m| {
        for (champ, slot, tok) in rows.iter() {
            match token_to_opt_index(tok) {
                // 0(자동)은 오버라이드가 아니므로 SEL 에서 빼서 delegate(PT) 값이 보이게 둔다.
                Some(idx) if idx >= 1 => { m.insert((champ.clone(), *slot), idx); applied += 1; }
                Some(_) => { m.remove(&(champ.clone(), *slot)); }
                // 아직 해석 불가(레지스트리 미준비/그 모드 비활성) → 원문 보관, 나중에 흡수.
                None => {
                    SEL_PENDING.lock().unwrap_or_else(|e| e.into_inner())
                        .push((champ.clone(), *slot, tok.clone()));
                    SEL_PENDING_ANY.store(true, Ordering::Relaxed);
                }
            }
        }
        save_sel(m);
    });
    if let Some(s) = stamp { let _ = fs::write(s, &hash); }
    update_override_snapshot();
    dbg_write("item_tactics_reco.txt",
              &format!("[reco] applied={} rows={} hash={}\n", applied, rows.len(), hash));
    true
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
static SETTER_NOPED: AtomicU64 = AtomicU64::new(0); // 0=미시도,1=성공,2=실패(RVA mismatch)

// ★ StrategyUIRunner update(FUN_140f17b40) 가 매 프레임 personal_tactics→드롭다운 +0x1788 로
//   강제 sync(revert) 하는 `call FUN_14218a230`(RVA 0xf1a74b, 5B `e8 rel32`)를 NOP 패치.
//   → revert 제거 → 사용자의 모드템(7+) 클릭이 +0x1788 에 영속 → 폴링이 캡처 가능.
//   (ghidra-re 2026-06-30: 드롭다운 클릭핸들러 자체는 7+ 거부 안함, revert 가 진범.)
//   부작용: 드롭다운 표시 자동sync 소실 → 모드가 옵션주입 시 sel 로 직접 유지.
// ★0.5.0_2 콜사이트 확정(ghidra-re 2026-07-08): StrategyUIRunner update(시작 0x140da1da0)의
//   item0/1/2 3회 루프 안 `call FUN_140d98720`(RVA 0xda42ee, e8 2d 44 ff ff). FUN_140d98720이
//   옵션리스트 재빌드+`FUN_142418cf0(runner,index,opts)` 호출→그 첫줄 `*(runner+0x1788)=index`=revert 진범.
//   ⚠마이그레이터 후보 0xf2a899/0xf2aae8은 둘 다 오답(밴픽 UI 러너=FUN_140f29840, NOP시 밴픽만 깨짐).
const SETTER_NOP_RVA: usize = 0xda42ee; // ⚠0.5.2·0.5.3 STALE(미마이그, SETTER_NOP_ENABLED=false라 무영향) // ⚠0.5.0_3 미마이그(STALE, mask-sig NONE→ghidra-re 후속). SETTER_NOP_ENABLED=false라 무영향. 0.5.0_2 StrategyUIRunner update item0/1/2 루프 내 model→+0x1788 라벨sync call.
// ★2026-07-08 ghidra-re 2차 확증: 이 NOP은 slot1/2/3 모드템 커밋 문제의 원인이 "아님". 전역에 +0x1788 revert-writer는
//   이 콜뿐인데 NOP해도 폴링에 item0/1/2가 7+로 바뀐 적 없음 = 클릭 자체가 게임 네이티브 드롭다운의 바닐라7 옵션벡터
//   기준 검증에 막혀 7+ 커밋 실패. #item3(모드소유 드롭다운)만 됨. → NOP 무익+라벨sync 부작용이라 OFF 복귀.
//   슬롯0/1/2 모드템 진짜 해결 = item0/1/2를 item3처럼 모드소유 벡터로 대체(별도 과제).
const SETTER_NOP_ENABLED: bool = false;
unsafe fn nop_revert_setter() {
    if !SETTER_NOP_ENABLED { return; }
    match SETTER_NOPED.load(Ordering::Relaxed) { 1 | 2 => return, _ => {} }
    let base = GetModuleHandleW(core::ptr::null()) as usize;
    if base == 0 { return; }
    let addr = base + SETTER_NOP_RVA;
    // 안전검증: call rel32(0xe8) 인지 확인 후에만 패치 (RVA 어긋나면 abort).
    if !readable(addr, 5) || *(addr as *const u8) != 0xe8 {
        SETTER_NOPED.store(2, Ordering::Relaxed);
        write_log("item_tactics_nop.txt", &format!("[{}ms] ✗ NOP abort: addr={:#x} byte0={:#04x} (expect 0xe8, RVA mismatch?)\n",
            now_ms(), addr, if readable(addr, 1) { *(addr as *const u8) } else { 0 }));
        return;
    }
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(addr, 5, RWX, &mut old) == 0 { SETTER_NOPED.store(2, Ordering::Relaxed); return; }
    for i in 0..5 { *((addr + i) as *mut u8) = 0x90; } // 5× NOP
    VirtualProtect(addr, 5, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, 5);
    SETTER_NOPED.store(1, Ordering::Relaxed);
    write_log("item_tactics_nop.txt", &format!("[{}ms] ★ revert setter NOP 적용 @ {:#x} (5B)\n", now_ms(), addr));
}
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
// ★+0x240 실측(07-21): 소스 내 주석이 모순(L375 "렌더 screen_x" vs L389 "히트테스트·무효")이고
//   y/w/h가 +0x244/+0x248/+0x24c로 이어지는지 미확인. 좌표를 아는 노드에서 영역을 덤프해 구조 확정.
//   히트박스 갱신은 이 실측 결과 확인 후에만 구현한다(추측 구현 금지).
const HITBOX_PROBE: bool = false; // ★배포 OFF(07-22): 0.5.2 채집 완료(hitbox_probe.txt). +0x240 구조 규명 재개 시 true.
static HITBOX_DONE: AtomicBool = AtomicBool::new(false);
const STRAT_DUMP: bool = false; // ★배포 OFF(07-22): 0.5.2 UI 주입 생존 확인 완료(유저 인게임 검증)로 역할 종료.
static STRAT_DUMP_DONE: AtomicBool = AtomicBool::new(false);
const CT_DUMP: bool = false; // 프로덕션 OFF(조합테스트 행 서브트리 파일덤프)
static CT_DUMP_N: AtomicU64 = AtomicU64::new(0); // 덤프 횟수(초기 몇 프레임은 게임이 아직 안 채웠을 수 있어 여러 번)
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
// ★1회성 진단(07-22 "4칸일 때 살짝 겹침" 제보): 조합테스트 행의 **실제** 자식 드롭다운을
//   id + authored x(+0x84) + visible 로 덤프. 우리 4칸 외에 네이티브 잔존/타 모드 주입 노드가
//   섞여 있는지(=4번째만 넓어 보이는 원인)를 좌표로 확정한다. 결과=item_tactics_ctrow.txt.
// ★배포 OFF(2026-07-22): 역할 완수 — 겹침의 정체가 comptest_unlock의 ct_i* 중복 주입임을 좌표로
//   확정했고, 그 모드에서 ITEM_DD_ENABLED=false 로 해소됨(재확인 덤프에서 ct_i* 소멸).
const CT_GEOM_DUMP: bool = false;
static CT_GEOM_DONE: AtomicBool = AtomicBool::new(false);
unsafe fn dump_ct_row_geom(root: &Node) {
    if !CT_GEOM_DUMP || CT_GEOM_DONE.swap(true, Ordering::Relaxed) { return; }
    let mut s = format!("[{}ms] 조합테스트 행 드롭다운 실측 (mode4={}, 기대=우리 4개만)\n\
        기준: 행 폭 608.5 / 네이티브 item0·1·2 = x146·296·446 폭140\n",
        now_ms(), ITEM_MODE.load(Ordering::Relaxed));
    for rid in ["blue0", "red0"].iter() {
        let Some(row) = find_node(root, rid) else { continue; };
        s.push_str(&format!("── {} 자식 ──\n", rid));
        for c in row.child.iter() {
            let id = c.id.as_str();
            if id.is_empty() { continue; }
            let na = c as *const Node as usize;
            let x = if na > 0x10000 && readable(na + 0x84, 4) { *((na + 0x84) as *const f32) } else { f32::NAN };
            s.push_str(&format!("  {:<12} x={:>8.1}  visible={}  runner={}\n",
                id, x, c.visible, c.runner.type_name()));
        }
    }
    if let Some(p) = mod_dir() { let _ = fs::write(p.join("item_tactics_ctrow.txt"), &s); }
}
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
const PV_STRIDE: usize = 0x260;           // PlayerViewInfo
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
const RVA_TIP_SHOW: usize = 0x1ab52f0;
const RVA_TIP_MEASURE_VT: usize = 0x318b4c0; // p3 = 텍스트 계측 ctx 의 vtable(상수)
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
const RVA_GAME_ALLOC: usize = 0x28f7df0;  // (rcx=무시, rdx=flags 0, r8=size) -> ptr
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
const RVA_GV_UPDATE: usize = 0x960df0;
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
    // ★1회 덤프: 행 서브트리 + champion_icon source 실측 → 챔프 해석 실패 원인 규명.
    if CT_DUMP && CT_DUMP_N.fetch_add(1, Ordering::Relaxed) % 120 == 0 && CT_DUMP_N.load(Ordering::Relaxed) < 700 {
        let mut s = format!("[{}ms] 조합테스트 행 구조 덤프
", now_ms());
        { // ★같은 id 노드가 트리에 몇 개인지(다중 인스턴스 판별)
            fn cnt(n: &Node, id: &str, acc: &mut usize) {
                if n.id.as_str() == id { *acc += 1; }
                for c in n.child.iter() { cnt(c, id, acc); }
            }
            for id in ["blue0", "builds", "training", "personal_plan"].iter() {
                let mut c = 0usize; cnt(&ui.root, id, &mut c);
                s.push_str(&format!("  [인스턴스] '{}' = {}개
", id, c));
            }
        }
        for rid in ["blue0", "blue1", "red0"].iter() {
            match find_node(&ui.root, rid) {
                None => s.push_str(&format!("--- {} : 노드없음
", rid)),
                Some(row) => {
                    s.push_str(&format!("--- {} 서브트리 ---
", rid));
                    unsafe { dump_subtree(row, 2, &mut s); }
                    // 아이콘 source + 라벨 text 둘 다 덤프 → 챔프명을 어디서 얻을지 확정
                    for cid in ["champion_icon", "name", "build", "item0"].iter() {
                        match find_node(row, cid) {
                            None => s.push_str(&format!("  #{} : 없음
", cid)),
                            Some(n) => {
                                let src = unsafe { read_img_source(n) };
                                let lbl = unsafe { read_label(n) };
                                s.push_str(&format!("  #{} : runner={} source={:?} label={:?} visible={}
",
                                    cid, n.runner.type_name(), src, lbl, n.visible));
                            }
                        }
                    }
                }
            }
        }
        // ★게이트 없이 직접 write (dbg_write=DELEGATE_DIAG 게이트라 안 나왔음)
        if let Some(d) = mod_dir() { let _ = fs::create_dir_all(&d); let _ = fs::write(d.join("comptest_rows.txt"), &s); }
    }
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

    // ★+0x240 구조 실측: 레이아웃 블록의 알려진 (x,y,w,h)와 +0x230~+0x260 영역을 나란히 덤프.
    //   레이아웃 값과 일치하는 f32가 어느 오프셋에 있는지로 히트박스 구조를 역산한다.
    if HITBOX_PROBE && !HITBOX_DONE.swap(true, Ordering::Relaxed) {
        let mut s = format!("[{}ms] +0x240 영역 구조 실측
", now_ms());
        for ri in 0..MAX_ROWS.min(2) {
            let Some(row) = find_node(&ui.root, &format!("row{}", ri)) else { continue; };
            for id in ["item0m", "item1m", "item2m"].iter() {
                let Some(n) = find_node(row, id) else { continue; };
                let na = n as *const Node as usize;
                unsafe {
                    let rd = |o: usize| -> f32 { if readable(na + o, 4) { *((na + o) as *const f32) } else { f32::NAN } };
                    // 레이아웃 블록0의 authored 값 (W+0x00/H+0x08/X+0x10/Y+0x18, 값 +4)
                    s.push_str(&format!("row{} #{}: authored w={} h={} x={} y={}
",
                        ri, id, rd(0x70+0x04), rd(0x70+0x0c), rd(0x70+0x14), rd(0x70+0x1c)));
                    s.push_str("   +0x230..+0x264 (f32): ");
                    let mut o = 0x230usize;
                    while o <= 0x264 { s.push_str(&format!("[{:#05x}]={} ", o, rd(o))); o += 4; }
                    s.push('\n');
                }
            }
        }
        s.push_str("해석: authored x/y/w/h 와 같은 값이 나타나는 오프셋 = 그 필드의 screen/hit 사본.
");
        if let Some(d) = mod_dir() { let _ = fs::write(d.join("hitbox_probe.txt"), &s); }
    }
    // ★검증: 개인전술 행 자식 id 실측 — 3칸 모드에서 item3가 섞여 있는지 확정.
    if STRAT_DUMP && !STRAT_DUMP_DONE.swap(true, Ordering::Relaxed) {
        let mut s = format!("[{}ms] 개인전술 행 자식 덤프 (ITEM_MODE={} slot_count={} MODE4={})
",
            now_ms(), ITEM_MODE.load(Ordering::Relaxed), slot_count(), uinj::MODE4.load(Ordering::Relaxed));
        if let Some(d) = mod_dir() {
            if let Ok(raw) = fs::read_to_string(d.join("4items.cfg")) { s.push_str(&format!("cfg 원문 tail: {:?}
", &raw[raw.len().saturating_sub(40)..])); }
        }
        for ri in 0..MAX_ROWS {
            match find_node(&ui.root, &format!("row{}", ri)) {
                None => s.push_str(&format!("row{}: 없음
", ri)),
                Some(row) => {
                    let ids: Vec<String> = row.child.iter().map(|c| format!("{}{}", c.id.as_str(), if c.visible {""} else {"(hidden)"})).collect();
                    s.push_str(&format!("row{} 자식({}) = {:?}
", ri, ids.len(), ids));
                }
            }
        }
        if let Some(d) = mod_dir() { let _ = fs::write(d.join("strategy_rows.txt"), &s); }
    }
    // ★ personal_tactics→드롭다운 revert(setter) 1회 NOP → 모드템(7+) 선택 영속.
    unsafe { nop_revert_setter(); }
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
        // ★옵션 계산 전에 추천을 반영해야 아래 SEL 조회가 새 값을 본다(같은 프레임에 표시됨).
        //   화면 닫힘마다 OPTS_INJECTED 가 리셋되므로 재진입 때마다 해시를 다시 확인한다.
        apply_recommendations();
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
        dbg_write("delegate_diag.txt", &diag); // ★임시: LOG_ENABLED 무관 덤프
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

const TRAMPOLINE_DEBUG_PASSTHROUGH: bool = false; // ★진단: 스텁=원본명령+복귀만(저장/호출 생략)

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
const O_ATHLETE_ID: usize = 0x810;
static MY_ATHLETES: AtomicPtr<std::collections::HashSet<u64>> = AtomicPtr::new(core::ptr::null_mut());
static MY_ATH_PREV: AtomicPtr<std::collections::HashSet<u64>> = AtomicPtr::new(core::ptr::null_mut());
static MY_ATH_N: AtomicU64 = AtomicU64::new(0); // 게시된 선발 인원수(0=미확보)
static ROSTER_TICK: AtomicU64 = AtomicU64::new(0);
static SPAWN_AID_OK: AtomicU64 = AtomicU64::new(0);   // 진단: 스폰시 athlete_id(+0x810) 유효(≠0·≠MAX)
static SPAWN_AID_ZERO: AtomicU64 = AtomicU64::new(0); // 진단: 스폰시 aid=0 (=스폰시점 미기입 → 이 경로 불가)
static SP4_NOBUILD: AtomicU64 = AtomicU64::new(0); // ④진단: build Vec(+0x498/+0x4a0) 무효
static SP4_NOCAT: AtomicU64 = AtomicU64::new(0);   // ④진단: 카탈로그(Game+0x1fc8 Vec) 무효
static SP4_NOIDX: AtomicU64 = AtomicU64::new(0);   // ④진단: 지정템 인덱스 획득 실패(스캔 None)
static SP4_RANGE: AtomicU64 = AtomicU64::new(0);   // ④진단: t >= cat_len(범위밖)
static SP4_BLEN: AtomicU64 = AtomicU64::new(0);    // ④진단: 관측된 build len 샘플
static SP4_CATLEN: AtomicU64 = AtomicU64::new(0);  // ④진단: 관측된 카탈로그 len 샘플
static SPAWN_AID_SAMPLE: [AtomicU64; 4] = [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];
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


// ── 4번째(slot3) build 버퍼 확장 진단/제어 ──
//   c6가 읽는 candidate build elem: [elem+8]=inner ptr, [elem+0x10]=len. [elem+0]=cap 추정(진단으로 확인).
//   slot3 write하려면 inner Vec len≥4 필요(추출기는 3만 만듦) → 여기서 cap 여유 시 len 4로 확장.
const EXTEND_BUILD: bool = false; // candidate build 확장은 추출기가 slot3 버려 무효 → OFF
// ★0.5.3 회귀진단(2026-07-29 — "1~3번은 사는데 4번째만 안 산다"의 원인 절단용).
//   detour(rayon 병렬 핫패스)에서는 **원자 카운터만** 만지고, 파일 출력은 post_update(메인 스레드)에서 한다.
//   [0]=4번째 경로 도달 [1]=build_len≠3 [2]=build_cap≠3 [3]=ptr/writable 실패
//   [4]=목표 인덱스 획득 실패(t4=None) [5]=realloc 실패 [6]=★성공(build[3] write)
//   원인 판정 = 도달수 대비 어느 카운터가 그 수를 먹었는가. 원인 확정 후 false로 되돌릴 것.
const BUILD_EXT_DIAG: bool = false; // ★원인 규명 완료(4번째 구매 정상·아이콘은 뷰모델 직독으로 해결) → 프로덕션 OFF
// ★구매순서 진단(2026-07-30): 내 팀 build[] 배열 스냅샷을 (champ,owned)당 1회 파일 기록.
const BUY_ORDER_DIAG: bool = false;
// ★조합테스트 주입 실패 규명용 — launcher retaddr 실측 목록을 파일로 남긴다(원인 확정 후 false).
// ★원인 규명·수정 완료(조합테스트 주입 = 팀 게이트 우회 누락, launcher retaddr 9곳 전수 확정) → 프로덕션 OFF.
//   재조사 필요 시 true = launcher_retaddr.txt 에 실측 목록이 남는다(원인 추적에 결정적이었음).
const LAUNCH_DIAG: bool = false;
static LD_TICK: AtomicU64 = AtomicU64::new(0); // ★구매순서 = 설계대로(내 팀은 목표 4개 동시 빌드업) 확인완 → 프로덕션 OFF
static BUY_ORDER_SEEN: Mutex<Option<std::collections::HashSet<String>>> = Mutex::new(None);
static BUY_ORDER_BUF: Mutex<String> = Mutex::new(String::new());
static BE_CNT: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8];
static BE_LAST: AtomicU64 = AtomicU64::new(0);   // 마지막 관측 (build_len<<32)|cap
static BE_LAST_T: AtomicU64 = AtomicU64::new(0); // 마지막 기록한 build[3] 목표 인덱스
static BE_TICK: AtomicU64 = AtomicU64::new(0);   // post_update 덤프 스로틀
static BE_MAX_OWNED: AtomicU64 = AtomicU64::new(0); // 관측된 owned(보유 아이템 수) 최댓값 = 실구매 증거
const RVA_REALLOC: usize = 0x28e3b10; // 0.5.3(구0.5.2=0x25c4dd0). __rust_realloc 실함수. (rcx=ptr,rdx=old,r8=align,r9=new)->rax. 구 exe 진입 112B 마스크시그 → 신 exe 유일 1히트 + 본문 명령 대 명령 동형(mov rdi,r9 / mov rsi,rcx / cmp r8,0x11 / jae).
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
// 카탈로그 인덱스 → 아이템 이름(evt[0x50] shadow-call). scan_recipe_safe_index 역방향.
unsafe fn catalog_name_at(ctx: usize, idx: u64) -> Option<String> {
    if ctx < 0x10000 || !readable(ctx, 0x38) { return None; }
    let coll = rd_u64(ctx + 0x30) as usize;
    if coll < 0x10000 || !readable(coll, 0x18) { return None; }
    let data = rd_u64(coll + 8) as usize;
    let len = rd_u64(coll + 0x10);
    if idx >= len || data < 0x10000 || !readable(data + (idx as usize) * 16, 16) { return None; }
    let e = data + (idx as usize) * 16;
    let edata = rd_u64(e) as usize;
    let evt = rd_u64(e + 8) as usize;
    if edata < 0x10000 || evt < 0x10000 || !readable(evt, 0x60) { return None; }
    let namefn = rd_u64(evt + 0x58) as usize;
    if !code_ptr_ok(namefn) { return None; }
    let f: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(namefn);
    let nobj = f(edata);
    if nobj < 0x10000 || !readable(nobj, 0x18) { return None; }
    let chars = rd_u64(nobj + 8) as usize;
    let nlen = rd_u64(nobj + 0x10) as usize;
    if chars < 0x10000 || nlen == 0 || nlen > 64 || !readable(chars, nlen) { return None; }
    Some(String::from_utf8_lossy(std::slice::from_raw_parts(chars as *const u8, nlen)).into_owned())
}

// ═══ 경기시작 launcher 훅(0.5.1 RE) — 렌더 경기 시드 결정적 캡처 ═══
//   launcher 0x20588a0(out=rcx, flag=dl, seed=r8, r9) ← 클라 렌더 씬빌더 0x722ca0가 호출(콜러=렌더 판별).
//   retaddr rva ∈ [0x722ca0, 0x732ca0)면 렌더 경기 → LIVE_SEED=seed(r8). buy훅 sim_seed==LIVE_SEED 게이트.
const CL_LAUNCHER_RVA: usize = 0xeb8810; // 0.5.3(구0.5.2=0x1d96870). 확정 근거: ①프롤로그 관용구 동형(8push+mov eax,frame+call chkstk+lea rbp,[rsp+0x80]+xmm 스필+[rbp+X]=-2) ②콜러 **9곳 = 구 exe와 동수** ③렌더 씬빌더(0x997740)가 2회 호출 ④내부에서 seedctor(0x12b9ab0)를 rdx=저장된 r8(seed)로 호출 = 구 exe 라인대응. 진입 시 r8=seed 계약 유지(mov r12,r8).
const CL_LAUNCHER_PROLOGUE: [u8; 17] = [0x55, 0x41,0x57, 0x41,0x56, 0x41,0x55, 0x41,0x54, 0x56, 0x57, 0x53, 0xb8, 0x08, 0x51, 0x02, 0x00]; // 0.5.3: 8push+mov eax,0x25108 (구 0.5.2=0x165c8) — chkstk 프레임만 확대
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
    let is_comptest = rva == 0x1925f12 || rva == 0x18f718e;
    if (rva == 0x9a3287 || rva == 0x9a7b03 || is_comptest) && seed != 0 {
        let prev = LIVE_SEED.swap(seed, Ordering::Relaxed);
        if prev != seed { RENDER_PROVIDER.store(0, Ordering::Relaxed); } // 새 경기 시드 → 직후 ctor가 provider 재캡처
        COMPTEST_MATCH.store(is_comptest, Ordering::Relaxed);
        LAUNCH_RENDER_N.fetch_add(1, Ordering::Relaxed);
        LAUNCH_RENDER_RA.store(rva, Ordering::Relaxed);
    }
    // 진단: 고유 콜러 rva 수집(원자 CAS, 24슬롯)
    for k in 0..24 {
        let s = LAUNCH_RVAS[k].load(Ordering::Relaxed);
        if s == rva { break; }
        if s == 0 && LAUNCH_RVAS[k].compare_exchange(0, rva, Ordering::Relaxed, Ordering::Relaxed).is_ok() { break; }
    }
    perf::rec(perf::S_LAUNCHER, __lt);
    0
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
const SEEDCTOR_RVA: usize = 0x12b9ab0; // 0.5.3(구0.5.2=0x22c1da0). 프롤로그 12B 완전동일(8push)·chkstk 프레임 0x11b58→0x11b98·launcher(0xeb8810) 내부 콜에서 rdx=저장된 r8(seed) 확인. ⚠seed 저장 오프셋은 provider+0xeab8→**+0xeaf8**로 이동(0x12ba92d 실측).
// ★0.5.3: provider 구조체에서 seed 저장 오프셋이 이동했다(0.5.2 +0xeab8 → 0.5.3 +0xeaf8).
//   실측 = seedctor 내부 `mov [reg+0xeaf8], rdx` @0x12ba92d (구 exe는 같은 자리에 0xeab8).
//   ⚠단일 상수로 묶어둔다 — 패치마다 여기만 갱신하면 is_live 게이트 전체가 따라온다.
const O_PROVIDER_SEED: usize = 0xeaf8;
const SEEDCTOR_PROLOGUE: [u8; 12] = [0x55, 0x41,0x57, 0x41,0x56, 0x41,0x55, 0x41,0x54, 0x56, 0x57, 0x53]; // ghidra-re 확정: 8push(12B)+mov eax,0x11b58+call chkstk (launcher 동일패턴)
const SEEDCTOR_ORIG_LEN: usize = 12; // 8push만 재배치(chkstk call 제외). jmp가 fn+12=mov eax에 착지→프레임 정상세팅
static SEEDCTOR_INSTALLED: AtomicU64 = AtomicU64::new(0);
static RENDER_PROVIDER: AtomicU64 = AtomicU64::new(0); // ★렌더 sim provider 포인터(is_live 주력 게이트)
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

// ═══════════════════════════════════════════════════════════════════════════
//  ★★v14 스폰 커밋 훅 (ghidra-re 0.5.1 확정) — "빌드 생성 1회 개입" 구조.
//    전 빌드생성 경로(라이브 워커 0x164f040 / 리그 0xf63f80 / 기타)가 athlete ctor→wrapper→
//    spawn FUN_142060280 단일 초크포인트로 합류한다. 여기서 build[] 목표만 심으면
//    buy resolver가 그 목표로 자연 빌드업(하위템→조합) → per-buy 개입 불필요.
//    인자: rcx=Game(→provider=*(Game+0x1dc0)), rdx=athlete(build 최종본), r8=descriptor.
//    athlete당 1회 · 단일 콜사이트 · 개인전술 적용 후 = 우리 주입이 최종 승자.
//    ⚠rdx athlete은 스택 사본(0x8b8) — 여기 쓰면 이후 memcpy로 provider Vec까지 전파(RE 확정).
// ═══════════════════════════════════════════════════════════════════════════
// ★0.5.2(2026-07-22 exe2exe): 0x2060280 = 스켈레톤 NO MATCH(=로직변경). 콜러 컨테이너 0x20565e0→0x1d94640(니모닉 1.0000)
//   의 동일 오프셋 +0x8c 콜 타깃으로 재핀 = 0x1d9e0e0. 함수 축소 0x714→0x51f, **프롤로그 push 8개→7개**(41 55=push r13 소멸).
//   ⇒ 프롤로그 상수·ORIG_LEN 갱신 + entry 패치(12B movabs+jmp)가 push블록(10B)보다 길어 mov eax까지 재배치해야 함
//     → rax 보존 tail(r11 점프)이 필요 = install_detour_r11.
//   ⚠로직변경 함수이므로 인자계약(rcx=Game, rdx=athlete) 미확인 → **게이트 OFF**(ghidra-re 재확인 후 재활성).
//     기능 손실 없음: 07-19 계측에서 build[] 주입 도달 8/8 = buy 경로 단독으로 충분.
// ★0.5.3 재핀 완료(2026-07-29, ghidra-re): 0x1d9e0e0 → **0xebfe50**(~0xec0302). 콜러 컨테이너 0x1d94640→0xeb6480(+0x91 콜 @0xeb6511).
//   본문 명령 1:1 대응 확인(`[rcx+0x1dc0]`/`[rcx+0x1dc8]`→`call [r15+0x160]`, `[rsi+0x1dd0]`/`[rsi+0x1dd8]`→`call [rax+0x30]`).
//   ⚠**재활성 시 반드시 2가지 변경 필요** — 지금은 게이트 OFF라 아래 상수를 쓰지 않는다:
//     ①프롤로그: 7push+mov eax+chkstk → **8push(12B) + sub rsp,0xf8**(chkstk 없음) ⟹ ORIG_LEN=12·rax 보존 불요(generic 가능).
//     ②인자계약: r8=&descriptor → **r8/r9 = descriptor 2워드 쌍**(콜러가 빌더를 전역 함수포인터 0x144531340 간접호출로 변경).
//        rcx=Game, rdx=athlete 스택사본(0x8b8)은 유지. 직접 콜러 15곳 = 단일 초크포인트 성격 유지.
const SPAWN_RVA: usize = 0xebfe50; // 0.5.3(구0.5.2=0x1d9e0e0, 구0.5.1=0x2060280). ⚠SPAWN_INJECT_ENABLED=false라 detour 미설치=무영향.
const SPAWN_PROLOGUE: [u8; 12] = [0x55, 0x41,0x57, 0x41,0x56, 0x41,0x55, 0x41,0x54, 0x56, 0x57, 0x53]; // 0.5.3: 8push(12B) + sub rsp,0xf8 (구0.5.2=7push+mov eax,0x4d20)
const SPAWN_ORIG_LEN: usize = 12; // 0.5.3: push8만 재배치(12B=정확히 명령경계) ⟹ 재활성 시 install_detour_r11 불요(generic으로 충분).
const SPAWN_INJECT_ENABLED: bool = false; // ★게이트 OFF 유지(0.5.3에서 **인자계약이 실제로 바뀌었음**이 확인됨 = 위 ② — 배선 전에 재검토 필수). ↓0.5.2 이력: 게이트 OFF(로직변경 미확인) — 구 0.5.1=true. ~~재개(07-19)~~ 봉인 사유 '스폰시 카탈로그 부재'가 오프셋 오류로 판명.
//   구 0x1fe8/0x1ff0 = 이웃 빈 Vec(상시 len=0) → 진짜 카탈로그 Game+0x1fd0/+0x1fd8 (ghidra-re 확정).
//   v15 팀판정(athlete_id 멤버십)은 검증완(aid유효 10/10·내팀 5/10 정확) → ④주입 완결 기대.
static SPAWN_INSTALLED: AtomicU64 = AtomicU64::new(0);
static SPAWN_N: AtomicU64 = AtomicU64::new(0);        // 훅 총 발화수
static SPAWN_LIVE_N: AtomicU64 = AtomicU64::new(0);   // 렌더 경기 athlete 판정수
static SPAWN_PLAYER_N: AtomicU64 = AtomicU64::new(0); // 그중 내 팀(주입 대상)
static SPAWN_WROTE: AtomicU64 = AtomicU64::new(0);    // 실제 build[] write 수
static SPAWN_NOSIDE: AtomicU64 = AtomicU64::new(0);   // side 미판정으로 스킵(=buy 경로가 커버)
unsafe extern "C" fn cap_spawn(saved: *mut u64, _e: usize) -> u64 {
    let __spt = perf::tsc();
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if !SPAWN_INJECT_ENABLED || saved.is_null() { return; }
        SPAWN_N.fetch_add(1, Ordering::Relaxed);
        let game = *saved as usize;            // rcx = Game
        let athlete = *saved.add(1) as usize;  // rdx = athlete(스택 사본, build 최종본)
        if game < 0x10000 || athlete < 0x10000 { return; }
        // ── ① 렌더(관전) 경기 게이트: provider = *(Game+0x1dc0) — 0.5.3에서 오프셋 유지 확인(07-29). ⚠buy 훅은 r9를 씀(별경로) ──
        let provider = match safe_read_u64(game + 0x1dc0) { Some(p) => p, None => return };
        if provider < 0x10000 || provider >= 0x0000_8000_0000_0000 { return; }
        let lseed = LIVE_SEED.load(Ordering::Relaxed);
        let seed_ok = lseed != 0 && safe_read_u64(provider as usize + O_PROVIDER_SEED) == Some(lseed);
        let rp = RENDER_PROVIDER.load(Ordering::Relaxed);
        if !(seed_ok || (rp != 0 && provider == rp)) { return; }
        SPAWN_LIVE_N.fetch_add(1, Ordering::Relaxed);
        // ★진단(v15 선결확인): 스폰 시점에 athlete_id(+0x810)가 이미 채워져 있나. 0이면 이 경로 자체가 불가.
        if readable(athlete + O_ATHLETE_ID, 8) {
            let aid = rd_u64(athlete + O_ATHLETE_ID);
            if aid == 0 || aid == u64::MAX { SPAWN_AID_ZERO.fetch_add(1, Ordering::Relaxed); }
            else { SPAWN_AID_OK.fetch_add(1, Ordering::Relaxed);
                   for k in 0..4 { if SPAWN_AID_SAMPLE[k].load(Ordering::Relaxed) == aid { break; }
                       if SPAWN_AID_SAMPLE[k].compare_exchange(0, aid, Ordering::Relaxed, Ordering::Relaxed).is_ok() { break; } } }
        }
        // ── ② 지정 챔프인가 ──
        if !readable(athlete, 0x8b8) { return; }
        let cptr = rd_u64(athlete + 0x420) as usize;
        let clen = rd_u64(athlete + 0x428) as usize;
        if cptr < 0x10000 || clen == 0 || clen > 48 || !readable(cptr, clen) { return; }
        let champ_cow = String::from_utf8_lossy(std::slice::from_raw_parts(cptr as *const u8, clen));
        let champ: &str = champ_cow.as_ref();
        if !is_champ_designated(champ) { return; }
        // ── ③ 내 팀인가 (v15): athlete_id(+0x810) ∈ 내 팀 선발 = scene tag9 불필요 → 스폰 시점에 성립.
        //     ⚠A2 정적확정으로 sim엔 team_id가 없으므로 이 멤버십이 유일한 결정적 경로.
        //     로스터 미확보(관리화면 방문 전)면 판정보류 → 주입 스킵(적팀 오염 방지 > 커버리지). buy 경로가 커버.
        //     폴백으로 scene side도 병행(로스터는 있으나 aid 미기입인 초기 프레임 대비).
        let mine = is_my_athlete(athlete);
        let ok = match mine {
            Some(true) => true,
            Some(false) => return,               // 확정 타팀 = 주입 안 함
            None => {                             // 로스터/aid 미확보 → scene 폴백(있으면)
                let side = if readable(athlete + 0x820, 8) { rd_u64(athlete + 0x820) } else { u64::MAX };
                match scene_player_side() { Some(ps) => side == ps, None => false }
            }
        };
        if !ok { SPAWN_NOSIDE.fetch_add(1, Ordering::Relaxed); return; }
        SPAWN_PLAYER_N.fetch_add(1, Ordering::Relaxed);
        // ── ④ build[] 목표 주입 ──
        // ★★카탈로그 오프셋 정정(07-19 ghidra-re 확정): 구 0x1fe8/0x1ff0은 0x18 어긋난 **이웃 빈 Vec**이었다
        //   (Game ctor가 cap=0/ptr=8(dangling)/len=0 초기화, exe 전체에 push 사이트 없음 → 상시 len=0.
        //    실측 catlen=0의 정체가 바로 이것 — 스폰 타이밍 문제가 아니었음).
        //   진짜 카탈로그 = Game+0x1fc8{cap}/+0x1fd0{ptr}/+0x1fd8{len}, stride 0x10 {elem_ptr, vtable}.
        //   ★인덱스 공간 동일: ctx 빌더 0x1420571C8이 ctx+0x30 = &(Game+0x1fc8)을 넣음 ⟹ buy가 인덱싱하는
        //    배열과 같은 힙 버퍼 = build[]에 그대로 사용 가능(대응표 불요).
        //   ★순서 보장: Game 생성(카탈로그 빌더 0x21c0750)이 스폰보다 선행(21개 wrapper 콜사이트 전부).
        let cat_base = rd_u64(game + 0x1fd0) as usize;
        let cat_len = rd_u64(game + 0x1fd8);
        let bptr = rd_u64(athlete + 0x498) as usize;
        let blen = rd_u64(athlete + 0x4a0);
        SP4_BLEN.store(blen, Ordering::Relaxed);
        SP4_CATLEN.store(cat_len, Ordering::Relaxed);
        if bptr < 0x10000 || blen == 0 || blen > 8 || !writable(bptr, (blen as usize) * 8) {
            SP4_NOBUILD.fetch_add(1, Ordering::Relaxed); return;
        }
        if cat_base < 0x10000 || cat_len == 0 || cat_len > 100000 {
            SP4_NOCAT.fetch_add(1, Ordering::Relaxed); // 바닐라 지정은 스캔 불요라 계속 진행
        }
        for si in 0u8..3 {
            if (si as u64) >= blen { break; }
            // ★스코프 = Plain 고정: 스폰 시점엔 조합테스트 진영 정보가 없다(그리고 이 훅은
            //   SPAWN_INJECT_ENABLED=false 로 봉인 상태). 조합테스트 진영별 지정은 buy 경로가 담당.
            let idx: Option<u64> = if let Some(vid) = slotN_vanilla_id(Scope::Plain, champ, si) {
                Some(vid) // 바닐라: id == catalog index
            } else if let Some(mk) = slotN_item_key(Scope::Plain, champ, si) {
                scan_catalog_index(cat_base, cat_len, mk.as_bytes()) // 모드템: 이름스캔+레시피검증
            } else { continue };
            let Some(t) = idx else { SP4_NOIDX.fetch_add(1, Ordering::Relaxed); continue };
            if cat_len > 0 && t < cat_len {
                if rd_u64(bptr + (si as usize) * 8) != t {
                    wr_u64(bptr + (si as usize) * 8, t);
                    SPAWN_WROTE.fetch_add(1, Ordering::Relaxed);
                }
            } else { SP4_RANGE.fetch_add(1, Ordering::Relaxed); }
        }
    }));
    perf::rec(perf::S_SPAWN, __spt);
    0 // install_detour_generic 스텁은 반환값 미사용(관찰/수정형 훅)
}
fn install_spawn_hook() {
    if !SPAWN_INJECT_ENABLED { return; } // ★봉인 시 detour 자체를 설치하지 않음(무동작 훅 = 순수 위험)
    if SPAWN_INSTALLED.load(Ordering::Relaxed) == 1 { return; }
    // ★0.5.2: rax 보존 tail 필수(재배치 구간에 mov eax,0x4d20 포함 → 직후 chkstk가 rax를 프레임크기로 사용).
    let r = unsafe { install_detour_r11(SPAWN_RVA, SPAWN_ORIG_LEN, cap_spawn as usize, &SPAWN_PROLOGUE) };
    SPAWN_INSTALLED.store(if r.is_ok() { 1 } else { 2 }, Ordering::Relaxed);
}

static VIEW_OK: AtomicU64 = AtomicU64::new(0);  // view 포착 성공수
static VIEWSCAN_DONE: AtomicBool = AtomicBool::new(false); // 실패 상세덤프 1회 게이트
// ★★역검색 진단: buy athlete(관전 로스터 원소 확정)에서 db view 오프셋을 런타임 역산. 휴리스틱 아님=결정적.
// ★★스레드 정체성 게이트 검증(07-11 RE 1순위): 관전(재시뮬)=메인스레드, 배경 sim=rayon 워커 가설.
//   post_update(메인스레드) tid vs buy 훅(sim 스레드) tid 비교 → 갈리면 오프셋 없이 관전 판정 가능.
static CP_INSTALLED: AtomicU64 = AtomicU64::new(0);
const CP_PROLOGUE: [u8; 12] = [0x55, 0x41,0x57, 0x41,0x56, 0x41,0x55, 0x41,0x54, 0x56, 0x57, 0x53];
const BS_INJECT_TEST: bool = false; // 입력주입=힙DB오염 크래시 → OFF. 반환후킹으로 전환 // ★build-score 채점 아이템키(스택)를 "dagger"로 덮어 실빌드 반영 판별

// ===========================================================================
//  player-state 배열 프로브 — 상단 아이템 바 표시 소스(GamePlayerState 배열)를
//  스캔으로 찾고 items Vec 오프셋 특정. (GameViewSystem+0x840 배열, stride 0x8d0)
//  champion@+0x420, team@+0x820, position@+0x8b0. items = +0x420~+0x820 사이.
// ===========================================================================
const PS_PROBE_ENABLED: bool = false; // 프로덕션: playerstate 진단 OFF
static PS_DONE: AtomicBool = AtomicBool::new(false);
static PS_ATTEMPTS: AtomicUsize = AtomicUsize::new(0);

// ★ sim 드라이버 FUN_14204f810 후킹: rdx=p2=매치 입력데이터. 미리계산 전 여기서 athlete 아이템빌드 찾기.
const SIM_PROBE_ENABLED: bool = false; // 프로덕션: sim 드라이버 진단훅 OFF
const SIM_RVA: usize = 0x223d1b0; // ⚠0.5.2·0.5.3 STALE(exe2exe NO MATCH=로직변경·SIM_PROBE_ENABLED=false라 무영향) // 0.5.0_3(구0.5.0_2=0x204f810, anchor 47-instr 일치, diff=스택슬롯 disp만=codegen churn·구조변경 아님). SIM_PROBE_ENABLED=false(OFF)
const SIM_ORIG_LEN: usize = 12; // push rbp/r15/r14/r13/r12/rsi/rdi/rbx (8개, PI)
const SIM_PROLOGUE: [u8; 12] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];
static SIM_INSTALLED: AtomicBool = AtomicBool::new(false);
static SIM_DUMPED: AtomicBool = AtomicBool::new(false);
// heap ptr v 에서 champ 이름/아이템키 같은 String 을 근방(v+0..v+0x28)서 탐색.
unsafe fn find_str_near(v: usize) -> Option<(usize, String)> {
    let mut o = 0usize;
    while o <= 0x28 { if let Some(st) = read_str_try(v + o) { if st.len() >= 3 { return Some((o, st)); } } o += 8; }
    None
}
unsafe fn dump_region(label: &str, base: usize) -> String {
    let mut s = format!("  {} = {:#x} (+0x0..+0x400):\n", label, base);
    if base <= 0x10000 { return s; }
    let mut oo = 0usize;
    while oo < 0x400 {
        let v = safe_read_u64(base + oo).unwrap_or(0);
        if looks_heap(v) {
            let mut note = String::new();
            if let Some(st) = read_str_try(v as usize) { note = format!(" →Str'{}'", st); }
            else if let Some((o2, st)) = find_str_near(v as usize) { note = format!(" →+{:#x}Str'{}'", o2, st); }
            else {
                note.push_str(" →[");
                for j in 0..4 { let e = safe_read_u64(v as usize + j * 8).unwrap_or(0); note.push_str(&format!("{:#x} ", e)); }
                note.push(']');
            }
            s.push_str(&format!("    +{:#x} = {:#018x}{}\n", oo, v, note));
        }
        oo += 8;
    }
    s
}
// ★ 표시소스 확정 테스트: 모든 챔프의 +0x410(Vec<u64>3) 을 알아볼 아이템ID로 덮어써 바가 바뀌나 관찰.
const DISPLAY_TEST: bool = false; // +0x410=빌드플랜사본, 바 아님 → OFF
const DISPLAY_TEST_ID: u64 = 29; // 알아볼 바닐라 최종템 ID (3칸 전부 이걸로)
static DTEST_LOGGED: AtomicBool = AtomicBool::new(false);
// ★ view(GameViewSystem) 포인터 캡처: FUN_1422360c0 mid-func 0x22360cc(rcx=view). view+0x840=배열/+0x848=count.
//   (0.5.0: 함수시작 0x22360c0, 구 0x1e84d50; mid 0x22360cc, 구 0x1e84d5c.)
static VIEW_PTR: AtomicU64 = AtomicU64::new(0);
// ⚠⚠ 0.5.0_3 미마이그(STALE): 0x22360cc→ mask-sig MULTI(로스터-게터 monomorphic family, 후보 0x19b77cc/787c/792c/79dc/… stride 0xb0). string-xref 불가=정적 확정 불가 → ghidra-re 후속.
//   ★위험: VIEW_PROLOGUE(14B)를 family 전원이 공유 → 사전검증이 오설치 못막음. AUTO4_FORWARD_SCORE 활성 시 잘못된 게터 후킹 가능 → ghidra-re 재핀 전엔 AUTO4 비활성 권장.
const VIEW_RVA: usize = 0x20ae1ac; // ⚠0.5.2·0.5.3 STALE(미마이그·VIEW_HOOK_ENABLED=false라 무영향) // 0.5.0_3(구0.5.0_2=0x22360cc, sig-xref UNIQUE: mov rax,[rcx+0x840];imul rcx,r9,0x8d0). VIEW_HOOK_ENABLED=false(OFF)
const VIEW_ORIG_LEN: usize = 14; // mov rax,[rcx+0x840](7) + imul rcx,r9,0x8d0(7)
// 0.5.0: mov rax,[rcx+0x840] = 48 8B 81 40 08 00 00 / imul rcx,r9,0x8d0 = 49 69 C9 D0 08 00 00
const VIEW_PROLOGUE: [u8; 14] = [0x48,0x8b,0x81,0x40,0x08,0x00,0x00, 0x49,0x69,0xc9,0xd0,0x08,0x00,0x00];
static VIEW_INSTALLED: AtomicBool = AtomicBool::new(false);
const VIEW_HOOK_ENABLED: bool = false; // ★hot 렌더함수 후킹=크래시 → OFF. 스캔으로 대체.
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
// champion String 위치로 로스터 원소 검증. ★0.5.0_3: champ name @ +0x420 (ath_champ_name과 정합).
//   ⚠구 +0x388~0x3b0 유산 오프셋만 보면 0.5.0 athlete를 인식 못해 find_view_by_scan 실패→LIVE_ARR=0(팀게이트 붕괴).
unsafe fn valid_ps_elem(elem: usize) -> bool {
    if read_str_try(elem + 0x420).is_some() { return true; } // 0.5.0_3 정위치
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
    // ★ 이분법: passthrough=true 면 레지스터저장·cap_fn 호출 전부 생략, 원본명령+복귀만.
    //   이게 무크래시면 patch/릴로케이션 OK → 저장/호출 문제. 크래시면 patch/orig 문제.
    if TRAMPOLINE_DEBUG_PASSTHROUGH {
        let mut s: Vec<u8> = Vec::new();
        let mut orig = vec![0u8; orig_len];
        core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
        s.extend_from_slice(&orig);
        s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes());
        s.extend_from_slice(&[0xff, 0xe0]);
        core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
        let mut patch = vec![0x90u8; orig_len];
        patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes());
        patch[10] = 0xff; patch[11] = 0xe0;
        let mut old: u32 = 0;
        if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
        core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
        VirtualProtect(fn_addr, orig_len, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
        return Ok(stub);
    }
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

// ★install_detour_generic 의 rax-보존 변형 (0.5.2 SPAWN 전용).
//   generic 은 tail 이 `movabs rax, ret_addr; jmp rax` 라 재배치 구간에 `mov eax,imm`(chkstk 프레임크기)이 들어가면
//   그 값을 덮어써 chkstk 폭주 → 여기선 tail 을 `movabs r11, ret_addr; jmp r11` 로 바꿔 rax 를 보존한다.
//   (r11 = x64 volatile scratch, 함수 진입 시점엔 의미값 없음 = 클로버 안전.)
//   체인후킹 분기는 미지원(SPAWN 은 타 모드 공용 훅 아님) — 외부훅 감지 시 Err.
unsafe fn install_detour_r11(rva: usize, orig_len: usize, cap_fn: usize, prologue: &[u8]) -> Result<usize, &'static str> {
    let base = GetModuleHandleW(core::ptr::null()) as usize;
    if base == 0 { return Err("module 0"); }
    if orig_len < 12 { return Err("orig_len<12"); } // entry 패치가 12B(movabs+jmp)
    let fn_addr = base + rva;
    if !readable(fn_addr, orig_len + 4) { return Err("unreadable"); }
    if *(fn_addr as *const u8) == 0x48 && *((fn_addr + 1) as *const u8) == 0xb8 { return Err("foreign hook"); }
    for i in 0..prologue.len() { if *((fn_addr + i) as *const u8) != prologue[i] { return Err("prologue mismatch"); } }
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    // push r12 rsi rdi rbx r11 r10 r9 r8 rdx rcx (generic 과 동일 레이아웃 = saved 인덱스 호환)
    s.extend_from_slice(&[0x41, 0x54, 0x56, 0x57, 0x53, 0x41, 0x53, 0x41, 0x52, 0x41, 0x51, 0x41, 0x50, 0x52, 0x51]);
    s.extend_from_slice(&[0x48, 0x89, 0xe1]);             // mov rcx, rsp
    s.extend_from_slice(&[0x48, 0x89, 0xe3]);             // mov rbx, rsp
    s.extend_from_slice(&[0x48, 0x83, 0xe4, 0xf0]);       // and rsp, -16
    s.extend_from_slice(&[0x48, 0x83, 0xec, 0x20]);       // sub rsp, 0x20
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff, 0xd0]);                   // call rax
    s.extend_from_slice(&[0x48, 0x89, 0xdc]);             // mov rsp, rbx
    s.extend_from_slice(&[0x59, 0x5a, 0x41, 0x58, 0x41, 0x59, 0x41, 0x5a, 0x41, 0x5b, 0x5b, 0x5f, 0x5e, 0x41, 0x5c]);
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);                            // 원본 명령 재실행(여기서 rax=프레임크기 세팅)
    s.extend_from_slice(&[0x49, 0xbb]); s.extend_from_slice(&ret_addr.to_le_bytes()); // movabs r11, ret_addr
    s.extend_from_slice(&[0x41, 0xff, 0xe3]);              // jmp r11  (rax 보존)
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
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
    for a in [&BR_TOTAL, &BR_LIVE, &BR_DES, &BR_DES_LIVE, &BR_ISPLAYER, &BR_IDX_OK, &BR_IDX_NONE, &BR_WROTE].iter() {
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
    s.push_str(&format!("  ★★v14 스폰커밋훅(0x2060280): 발화={} 렌더판정={} 내팀={} build write={} side미판정스킵={} 설치={}\n",
        ld(&SPAWN_N), ld(&SPAWN_LIVE_N), ld(&SPAWN_PLAYER_N), ld(&SPAWN_WROTE), ld(&SPAWN_NOSIDE), ld(&SPAWN_INSTALLED)));
    { // ★★지정템 도달 판정(누적) — 스냅샷이 아니라 "실제 보유한 적 있는가". 미도달만 실패로 본다.
        let want = REACH_WANT.lock().unwrap_or_else(|e| e.into_inner()).clone();
        let hit = REACH_HIT.lock().unwrap_or_else(|e| e.into_inner()).clone();
        s.push_str(&format!("\n[지정템 도달 판정]  도달 {}/{} (누적·경기후반 조합 포함)\n", hit.len(), want.len()));
        for w in want.iter() {
            s.push_str(&format!("  {} {}\n", if hit.iter().any(|h| h == w) { "✅도달" } else { "❌미도달" }, w));
        }
        if want.is_empty() { s.push_str("  (모드템 지정 없음 — 바닐라 지정만 있거나 지정챔프가 이 경기에 없음)\n"); }
    }
    s.push_str(&format!("     v15 팀판정: 내팀로스터={}명 | 스폰aid유효={} aid=0(미기입)={} 샘플=",
        ld(&MY_ATH_N), ld(&SPAWN_AID_OK), ld(&SPAWN_AID_ZERO)));
    for k in 0..4 { let v = SPAWN_AID_SAMPLE[k].load(Ordering::Relaxed); if v == 0 { break; } s.push_str(&format!("{} ", v)); }
    s.push('\n');
    s.push_str(&format!("     ④주입진단: build무효={} 카탈로그무효={} 인덱스실패={} 범위밖={} | blen샘플={} catlen샘플={}
",
        ld(&SP4_NOBUILD), ld(&SP4_NOCAT), ld(&SP4_NOIDX), ld(&SP4_RANGE), ld(&SP4_BLEN), ld(&SP4_CATLEN)));
    s.push_str("     (aid=0이 대부분이면 스폰시점엔 athlete_id 미기입 = v15 불가 → 다른 시임 필요)\n");
    s.push_str("     (write>0 = 스폰 1회 주입 성공 → buy 경로 없이도 목표 심김. side미판정스킵>0 = 스폰이 tag9보다 이름 → buy가 커버)\n");
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
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(fixdiag_flush)); // 검증 진단 덤프(FIXDIAG=true일 때만)
        // ★조합테스트 진단(2026-07-30): launcher retaddr 실측 목록 — 조합테스트 주입 실패 원인 규명용.
        //   모드는 retaddr 로 "화면 경기"를 판별한다. 조합테스트 값(0x1925f12)이 마이그 때 추정이라
        //   실제 발화하는 값과 다를 수 있다 ⟹ 관측된 전체 목록을 남겨 대조한다.
        if LAUNCH_DIAG {
            let n = LD_TICK.fetch_add(1, Ordering::Relaxed);
            if n % 300 == 0 {
                let mut s = String::from("[launcher retaddr 실측]
");
                s.push_str("  ※조합테스트를 실행한 뒤 이 목록에 새로 추가된 값이 조합테스트 retaddr
");
                for k in 0..24 {
                    let v = LAUNCH_RVAS[k].load(Ordering::Relaxed);
                    if v == 0 { break; }
                    let tag = match v {
                        0x9a3287 => " ← 렌더A(검증됨)",
                        0x9a7b03 => " ← 렌더B(검증됨)",
                        0x1925f12 => " ← 조합테스트 본경기(확정)",
                        0x18f718e => " ← 조합테스트 기록 다시보기(확정)",
                        0x229ad94 => " ← 리플레이",
                        _ => "",
                    };
                    s.push_str(&format!("  {:#x}{}
", v, tag));
                }
                s.push_str(&format!("
  launcher 총 발화={} / 화면경기 판정={} / LIVE_SEED={:#x}
  COMPTEST_MATCH={}
",
                    LAUNCH_N.load(Ordering::Relaxed), LAUNCH_RENDER_N.load(Ordering::Relaxed),
                    LIVE_SEED.load(Ordering::Relaxed), COMPTEST_MATCH.load(Ordering::Relaxed)));
                if let Some(d) = mod_dir() { let _ = fs::write(d.join("launcher_retaddr.txt"), s); }
            }
        }
        // ★0.5.3 회귀진단 덤프(build 확장 경로) — 300프레임(≈5초)마다 메인 스레드에서만 파일 write.
        if BUILD_EXT_DIAG {
            let n = BE_TICK.fetch_add(1, Ordering::Relaxed);
            if n % 300 == 0 {
                let c: Vec<u64> = BE_CNT.iter().map(|a| a.load(Ordering::Relaxed)).collect();
                let last = BE_LAST.load(Ordering::Relaxed);
                let s = format!(
                    "[build 확장 경로 진단]\n\
                     도달(4번째 경로 진입) = {}\n\
                     ├ build_len≠3 로 스킵   = {}\n\
                     ├ build_cap≠3 로 스킵   = {}\n\
                     ├ ptr/writable 실패     = {}\n\
                     ├ 목표 인덱스 획득 실패 = {}\n\
                     ├ realloc 실패          = {}\n\
                     └ ★성공(build[3] write) = {}\n\
                     ★실구매 판정: owned>=4 관측 = {} 회 / 관측된 owned 최댓값 = {}\n\
                       (owned>=4가 0이면 진짜로 안 사는 것 / 0이 아니면 구매는 되는데 경기중 아이콘만 안 보이는 것)\n\
                     마지막 관측: build_len={} build_cap={} / 마지막 목표 인덱스={}\n\
                     [slot3 아이콘] 세팅성공={} 스킵={} / GameView={:#x}(히트{}) 뷰모델 4번째보유={}명\n\
                     [슬롯 UI 수술] {}\n\
                     참고: mode(slot_count)={} · MY_ATHLETES={}명 · LIVE_SEED={:#x} · buy write 성공누계={}\n",
                    c[0], c[1], c[2], c[3], c[4], c[5], c[6],
                    c[7], BE_MAX_OWNED.load(Ordering::Relaxed),
                    last >> 32, last & 0xffff_ffff, BE_LAST_T.load(Ordering::Relaxed),
                    SLOT3_ICON_N.load(Ordering::Relaxed), SLOT3_ICON_MISS.load(Ordering::Relaxed),
                    GAME_VIEW.load(Ordering::Relaxed), GV_HITS.load(Ordering::Relaxed),
                    SLOT3_PV_N.load(Ordering::Relaxed),
                    SLOTUI_MSG.lock().unwrap_or_else(|e| e.into_inner()).clone().unwrap_or_else(|| "(미실행)".into()),
                    slot_count(), MY_ATH_N.load(Ordering::Relaxed),
                    LIVE_SEED.load(Ordering::Relaxed), BUY_WROTE_FIRE.load(Ordering::Relaxed));
                if let Some(d) = mod_dir() { let _ = fs::write(d.join("build_ext_diag.txt"), s); }
            }
        }
        { let t = perf::tsc();
          install_launcher_hook(); install_seed_ctor_hook(); install_spawn_hook(); install_game_view_hook(); // ★매프레임 재시도(멱등, 성공=1이면 즉시 return) — on_server_start 1회 실패 시 자가복구 + serpen 체인 재검증
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
            if pu == 0 {
                PID_OBS_ZERO.fetch_add(1, Ordering::Relaxed);
                if ct_ctx { PID_SKIP_CT.fetch_add(1, Ordering::Relaxed); }
                else { PID_ZERO_CLEAN.fetch_add(1, Ordering::Relaxed); } // 조합테스트와 무관한 0 관측
            } else if pu != u64::MAX && pu < 10000 { PID_OBS_NONZERO.fetch_add(1, Ordering::Relaxed); }
            if pu != u64::MAX && pu < 10000 && !(pu == 0 && ct_ctx) {
                if pu != 0 {
                    PLAYER_TEAM_ID.store(pu, Ordering::Relaxed);
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
                if n % ROSTER_POLL == 0 && known != u64::MAX && known < 10000 {
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
                    let trust = known != 0 || PID_ZERO_CLEAN.load(Ordering::Relaxed) >= 600;
                    if !my.is_empty() && trust { publish_my_athletes(my); }
                    else if !trust { MY_TRUST_SKIP.fetch_add(1, Ordering::Relaxed); }
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
                  unsafe { dump_ct_row_geom(&ui.root); } // 1회성 기하 진단
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
    fn on_server_start(&self, ctx: &mut ServerModContext) { probe_db(ctx); install_replace_4th(); install_launcher_hook(); install_seed_ctor_hook(); install_spawn_hook(); } // resolver=mode 3·4 공통(슬롯0/1/2 지정) + v13 식별훅(launcher 시드 + seed-ctor provider)
    fn before_management_tick(&self, ctx: &mut ServerModContext) {
        // ★매치 사이(관리화면)서 팀게이트 캐시 리셋 → 다음 경기서 로스터 재스캔(주소 재사용 대비).
        //   관리틱은 match sim 중엔 안 도므로 sim스레드의 판정과 레이스 없음.
        PLAYER_SIDE.store(u64::MAX, Ordering::Relaxed);
        SIDE_CACHE.lock().unwrap_or_else(|e| e.into_inner()).clear();
        probe_db(ctx); install_replace_4th(); // resolver=mode 3·4 공통 (멱등)
        // ★측정 전용: `dump_builds.trigger` 파일이 있을 때만 1회 실행(평소 비용 = exists() 1회).
        //   관리틱은 sim 중엔 안 돌므로 forward shadow-call 이 sim 과 레이스하지 않는다.
        unsafe { maybe_dump_builds(); }
    }
}
static NETSCAN_DONE: AtomicBool = AtomicBool::new(false);
fn probe_db(ctx: &mut ServerModContext) {
    // Database 시작 = champion_patch_statistics(@Database+0x16698) 절대주소 − 0x16698.
    let cps = &ctx.database.champion_patch_statistics as *const _ as usize;
    let db = cps.wrapping_sub(0x16698);
    // ── 아이템 신경망 probe + 자가검증(16384/16384/1) ──
    //   ★0.5.0_3 실측: db+0xd30 (구 0xda0에서 -0x70 이동, netscan 진단 HIT). 후보 순차 + 윈도우 스캔 폴백(패치 견고).
    if ITEM_NET_ADDR.load(Ordering::Relaxed) == 0 {
        unsafe {
            // ★0.5.1 시그 강화(ghidra-re): 헤더(16384/16384/1)만 맞는 lookalike(db+0xd30)는 +0x8 가중치ptr이 dangling
            //   → forward 내부 +0x44a서 deref AV. 가중치ptr readable 검증 추가로 가짜 탈락, 진짜 net(db+0x1558)만 통과.
            let sig_ok = |a: usize| readable(a, 0x20) && rd_u64(a) == 16384 && rd_u64(a + 0x10) == 16384 && rd_u64(a + 0x18) == 1
                && { let w = rd_u64(a + 0x8) as usize; w >= 0x10000 && readable(w, 16384 * 4) };
            let mut found = 0usize;
            for &off in &[0x1558usize, 0xd30, 0xda0] { // ★0.5.1: 게임 실제 net=GameData+0x1558(ghidra-re 확정, 양버전 동일) 우선. db==GameData베이스.
                if sig_ok(db + off) { found = db + off; break; }
            }
            if found == 0 { // 윈도우 자동탐색(향후 패치서 또 이동해도 자가복구)
                let mut o = 0usize;
                while o < 0x18000 { let a = db + o; if sig_ok(a) { found = a; break; } o += 8; }
            }
            if found != 0 {
                ITEM_NET_ADDR.store(found as u64, Ordering::Relaxed);
                append_log("4items.txt", &format!("[{}ms] item_net={:#x} (db+{:#x}) ★유효 fwd_valid={}", now_ms(), found, found - db, itemnet_addr_valid()));
            } else {
                let net = db + 0xda0;
                // ★진단(LOG무관): +0xda0 실패 → cps 기준 넓은 창에서 net 시그(16384/*/16384/1) 스캔해 실제 오프셋 찾기.
                //   + forward RVA 프롤로그도 덤프(itemnet_addr_valid 실패 원인 구분). 한 번만.
                if !NETSCAN_DONE.swap(true, Ordering::Relaxed) {
                    let mut out = format!("db={:#x} cps={:#x} (champ_patch_stat off=0x16698)\n net@+0xda0={:#x} sig=({},{},{}) readable={}\n",
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
    if MODITEMS_DONE.load(Ordering::Relaxed) { return; }
    unsafe { dump_mod_items(db); }
    append_log("item_tactics.txt", &format!("[{}ms] probe_db: db={:#x} 모드템 {}개 최종 {}개", now_ms(), db,
        MOD_REGISTRY.lock().unwrap_or_else(|e| e.into_inner()).len(),
        MOD_FINALS.lock().unwrap_or_else(|e| e.into_inner()).len()));
}

// ══ athlete→champion 매핑 프로브 (buy_item r8=athlete 스캔) ══════════════════
const RVA_BUY_ITEM: usize = 0xd0c680; // 0.5.3(구0.5.2=0x211e070). **진입 24B 바이트 완전동일**(exe 전체 유일 1히트) + 본체 명령 대 명령 동형 + 인자계약 유지(r8=athlete·[rsp_entry+0x30]=Game·Game+0x30=catalog). orig_len=19도 그대로(11B<12B → 다음 클린경계 mov rax,[rsp+0xa8] 8B). ⚠0.5.3 변화: 호출 경로가 direct call → vtable(+0x78) 썽크 0xd22340 경유로 바뀌었으나 **함수 진입부 훅이라 전 호출 포착됨**. ↓이하 0.5.2 이력. (구0.5.1=0x1f01090, exe2exe 스켈레톤 UNIQUE·프롤로그 24B 완전동일=본체 무변경, delta +0x21cfe0). ↓이하 0.5.1 이력. 함수 대개편(8push/sub0x38→5push/sub0x50, build/이름비교가 서브함수 0x1f00920로 분리)으로 mask-sig NONE이었으나 인자계약 불변(r8=athlete, p6=Game@rsp_entry+0x30, Game+0x30=catalog)로 확정. buy 드라이버 FUN_142234430(구 FUN_1420e76e0 후계)+vtable슬롯 교차검증.
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
const ITEMNET_FORWARD_RVA: usize = 0x10587e0; // 0.5.3(구0.5.2=0x1b9cce0). 진입 24B 완전동일 + 피처명 문자열 5종 일치(self_item/champ_pos_build/lane_counter/synergy/global_counter) + net 레이아웃 불변(net+0x8=가중치ptr, +0x10=16384 바운드, +0x18=1) ⟹ 모드의 매호출 재검증 로직 그대로 유효. ↓이하 0.5.2 이력. (구0.5.1=0x1bc82e0, exe2exe UNIQUE·프롤로그 동일). ↓이하 0.5.1 이력: (구0.5.0_3=0x1b78420, mask-sig UNIQUE PROL-OK push8 554157415641554154565753). ⚠AUTO4_FORWARD_SCORE=false로 OFF(0.5.1서 forward 내부 +0x44a AV, 위 플래그 주석 참조). 프롤로그 매치≠내부동작 동일.
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
// ★ AUTO 4번째: c6가 캡처한 beam의 4번째 아이템 id(챔프별). auto 챔프 구매 시 이걸로 강제.
static BEAM4TH: Mutex<Option<HashMap<String, u64>>> = Mutex::new(None);
fn beam4_get(champ: &str) -> Option<u64> {
    BEAM4TH.lock().unwrap_or_else(|e| e.into_inner()).as_ref().and_then(|m| m.get(champ).copied())
}
fn beam4_set(champ: String, id: u64) {
    let mut g = BEAM4TH.lock().unwrap_or_else(|e| e.into_inner());
    let m = g.get_or_insert_with(HashMap::new);
    if m.len() < 64 { m.insert(champ, id); }
}
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
// c6(개인전술 적용) 시점 forward 채점 — 폐기(적/백그라운드 미발화). AUTO는 buy 시점(compute_auto_4th_id)에서 처리.
const AUTO4_C6_SCORE: bool = false;
// ★0.5.0 build-extension: RVA_REALLOC(0x25a56c0 실함수) 확정 → ON. buy build Vec 3→4 실구매 재가동.
const BUILD_EXTEND_ENABLED: bool = true;
// ★0.5.0 ui_inject(#item3 드롭다운 + #slot3 노드): 로더훅 RVA(LOADER 0x4d8fb0/PARSER 0x2493b90/
//   ALLOC 0x25a5620) 확정 → ON. 전략화면 4번째 드롭다운/경기중 slot3 노드 주입 재가동.
const UI_INJECT_ENABLED: bool = true;   // ★0.5.0 수정: player_info/wide .ui를 0.5.0기반+4슬롯으로 재작성 → 재활성화(격리 테스트)
// ★진단: 슬롯 UI 패치(상한+헬퍼) OFF 게이트 — 타이틀복귀(데모전투) 크래시 이분탐색.
// ★0.5.0: 헬퍼 RVA_SLOT_HELPER(0xdc2390) 확정 → OFF(=patch_slot_ui 재가동). 경기중 slot3 아이콘 표시.
// ★★0.5.3(2026-07-29) 강제 OFF — 이 기능만 포팅 불가(크래시 방지). 근거 2단:
//   ① 헬퍼 함수 자체가 **소멸**: 0.5.2 RVA_SLOT_HELPER(0xc5cd80)가 0.5.3에선 UI 메가함수 0xa5c1e0
//      안으로 **완전 인라인**됐다(신 exe .text에 "blue_pla"/"red_play" movabs 0건, 콜사이트 0건).
//      인라인 블록 4곳(각 75B)이 (ptr,len) 3쌍을 rbp+0x10d20/+0x10d30/+0x10d40에 직접 스토어한다.
//   ② 상한만 늘리는 것도 **불가**: 4번째 엔트리 자리 rbp+0x10d50/+0x10d58이 0.5.3에선 **다른 지역변수로
//      이미 사용 중**(각각 40회·27회 참조 실측, 예 0xa62f9f mov [rbp+0x10d50],0 / 0xa6339f cmp rdi,[rbp+0x10d50]).
//      cmp 0x30→0x40 만 하면 루프가 그 지역변수를 문자열 (ptr,len)으로 읽어 확정 크래시.
//      프레임 여유도 없음(rbp 상한 +0x10f88, 상단은 xmm 스필).
//   ⟹ 재개하려면 인라인 블록 트램폴린 + 배열 base disp32 재배치까지 필요 = 별도 재설계 과제.
//      OFF 시 손실 = 경기중 4번째 아이템 **아이콘 표시**뿐(구매·스탯적용·AI 추천은 전부 살아있음).
// ★★재개(2026-07-30, 유저 지시): 위 ①②는 "0.5.2 방식(헬퍼 replace + 상한만 늘리기)"이 불가하다는 뜻이고,
//   **프레임 확장 + 배열 이전** 수술로는 가능하다고 판단해 재활성한다. 상세 설계·안전장치 = `patch_slot_ui` 주석.
//   문제 시 되돌리는 스위치 2개: 이 값을 true(전체 스킵) 또는 `SLOT_UI_SURGERY=false`.
// ★★2026-07-30 최종 정리 — 이 스위치는 **구 바이트패치 방식(SLOT_BOUNDS 상한 확장 + SLOT_HELPER replace)** 전용이다.
//   0.5.3에서 그 방식은 실패했고(`SLOT_UI_SURGERY=false` 참조), 4번째 아이콘은 **뷰모델 직독**
//   (`handle_ingame_slot3` — GameView→player_view→items[3]→노드 세팅)으로 **인게임 검증 완료**됐다.
//   ⟹ 이 값은 true(=구 방식 전체 스킵)로 두는 것이 정답. 아이콘 기능의 on/off는 `SLOT3_ICON_ENABLED`다.
//   ⚠혼동 주의: "DIAG_SLOT_UI_OFF=true"라고 아이콘이 꺼진 게 아니다(구 경로만 꺼짐).
const DIAG_SLOT_UI_OFF: bool = true;   // 구 바이트패치 경로 봉인 유지(실패). 아이콘 = 뷰모델 직독으로 별도 동작. ↓이하 0.5.1~0.5.2 이력: SLOT_BOUNDS 4곳(0x4b4d40/50b0/5790/5b00)·SLOT_HELPER(0xd81b30) = ghidra-re가 OLD↔NEW 바이트대조로 mask-sig 픽 전부 정확 확증(HIGH, idiom +0x8fb0 4곳·"blue_pla" movabs). 오식별 아님 → ON.
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
const ATH_STRIDE: usize = 0x8d0;
// athlete 유효성 검사 + (team, champ_id) 반환. 강한 검증(team∈{0,1} + 실챔프명)으로 배열 경계 자동 판정.
unsafe fn athlete_lineup_at(p: usize) -> Option<(u64, u64)> {
    if p < 0x10000 { return None; }
    let team = safe_read_u64(p + 0x820)?;
    if team > 1 { return None; }
    let nptr = safe_read_u64(p + 0x420)? as usize; // 0.5.0 champion name ptr (구 0x398)
    let nlen = safe_read_u64(p + 0x428)? as usize; // 0.5.0 champion name len (구 0x3a0)
    if nptr < 0x10000 || nlen == 0 || nlen > 48 { return None; }
    let mut buf = Vec::new();
    if !safe_read_bytes(nptr, nlen, &mut buf) { return None; }
    let name = String::from_utf8_lossy(&buf).into_owned();
    let cid = champ_id_of(&name)? as u64;
    Some((team, cid))
}
// ★ athlete의 champion name 읽기(+0x420 ptr / +0x428 len, 0.5.0_3 확정). SEL/PT 매칭용.
unsafe fn ath_champ_name(p: usize) -> Option<String> {
    if p < 0x10000 { return None; }
    let nptr = safe_read_u64(p + 0x420)? as usize;
    let nlen = safe_read_u64(p + 0x428)? as usize;
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
    let side = safe_read_u64(p + 0x820)?;
    if side > 1 { return None; }
    let pos = safe_read_u64(p + 0x8b0)? & 0xffff_ffff; // 라인 0~4
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
            let lane = (safe_read_u64(a + 0x8b0).unwrap_or(9) & 0xffff_ffff) as usize; // 실제 포지션(0~4)
            if lane < 5 {
                if team == my_team { ctx[lane] = cid; } else { ctx[5 + lane] = cid; }
            }
        }
        a = a.wrapping_add(ATH_STRIDE);
    }
    let pos = ((safe_read_u64(p + 0x8b0).unwrap_or(0) & 0xffff_ffff) as usize).min(4);
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
    let ptr = rd_u64(athlete + 0x498) as usize; // 0.5.0 build ptr (구 0x410)
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
// ★스폰 훅용(v14): 카탈로그 base/len을 직접 받는 스캔 (Game+0x1fd0/+0x1fd8). 캐시 = base 키.
//   buy 경로(ctx+0x30 컬렉션)와 같은 인덱스 공간 — build[] 값이 곧 이 인덱스라 resolver가 그대로 소비.
unsafe fn scan_catalog_index(base: usize, len: u64, want: &[u8]) -> Option<u64> {
    if want.is_empty() || base < 0x10000 || len == 0 || len > 100000 { return None; }
    { let g = SCAN_CACHE.lock().unwrap_or_else(|e| e.into_inner());
      if let Some(outer) = g.as_ref() { if let Some(m) = outer.get(&base) {
          if let Some(&v) = m.get(want) { return if v >= 0 { Some(v as u64) } else { None }; } } } }
    let res = scan_recipe_safe_in(base, len, want);
    { let mut g = SCAN_CACHE.lock().unwrap_or_else(|e| e.into_inner());
      if g.is_none() { *g = Some(HashMap::new()); }
      if let Some(outer) = g.as_mut() {
          if !outer.contains_key(&base) && outer.len() >= 16 { outer.clear(); }
          let m = outer.entry(base).or_insert_with(HashMap::new);
          if m.len() < 256 { m.insert(want.to_vec(), res.map(|i| i as i64).unwrap_or(-1)); }
      } }
    res
}
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
const BUY_REPORT: bool = false; // ★프로덕션 OFF(2026-07-30 검증 완료 후 복귀): buy_report.txt write + per-buy
                                // 진단 전부 봉인. 주입/식별 기능은 이 게이트 바깥이라 무영향. (재검증 시 true)
// ★★출력 덮어쓰기(ghidra-re 확정): 리졸버 출력 RDX(살 아이템 컬렉션 인덱스)를 목표로 강제 → 정확한 아이템 구매.
//   build[] 입력조작이 리졸버 스킵/RNG에 무시되던 근본문제 우회. saved[1]=RDX·saved[6]=RAX(=1) 쓰고 HANDLED 반환.
const OUTPUT_OVERRIDE: bool = false; // ★OFF 유지(07-19): build[] 목표주입으로 지정 최종템 실구매 확인됨(유저 인게임 관측).
//   ⚠07-19 오판 기록 — 리포트 own2까지만 캡처된 11줄 표본을 보고 "지정템 도달 0건"이라 단정해 ON 전환을 시도했으나,
//   실제로는 경기 후반(own3+) 조합이 기록되지 않았을 뿐이었다. 스냅샷 표본으로 미도달을 결론내지 말 것
//   (→ 아래 DESIGNATED_REACH 계측이 도달 여부를 확정적으로 판정한다).
// ★리졸버 진단(0.5.1): slot012 write 직전 build len·컬렉션 인덱스 유효성 캡처 → len게이트 vs 인덱스불일치 판별.
const RESOLVER_DIAG: bool = false;
static RESDIAG_N: AtomicU64 = AtomicU64::new(0);
static RESDIAG_BUF: Mutex<String> = Mutex::new(String::new());
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

// ═══ 검증 진단(2026-07-26, 개념a): 배경·관전이 같은 매치에서 같은 라인업·SEL-side(fix 입력)를 보는가 +
//   조합같은 다른매치가 seed로 분리되는가. read-only(주입 무변경). World+0x840 정순회(±9 스캔 아님).
//   배포시 false. ═══
const FIXDIAG: bool = false; // ★상세진단(내 선수5 id + 참가자10 id/매치/관전배경). 2026-07-27 10경기 10/10 배경my5·관전my5·✅수렴·발산0 검증완 → OFF. 재검증 시만 true.
// ★★fix B(2026-07-27): 관전==확정. is_live 조기탈출 제거 → 배경에도 주입, 팀스코프=is_my_athlete(+0x810).
//   내 선수=지정템 / 나머지=신경망, 배경·관전 동일 → 수렴. id기반이라 AI끼리 경기는 my=0=지정無=통계오염0.
//   ⚠false=옛 동작(is_live 게이트·배경 무주입) 복원. 문제 시 즉시 롤백용.
const FIXB: bool = true;
static FIXDIAG_MAP: Mutex<Option<HashMap<u64, (Option<(u64, i8, u32, u32)>, Option<(u64, i8, u32, u32)>, Option<String>)>>> = Mutex::new(None); // seed→(배경,관전,이름). 각=(lineup_fp, sel, pcount, my_hits)
static FIXDIAG_CTR: AtomicU64 = AtomicU64::new(0);
static FIXDIAG_LAST: AtomicU64 = AtomicU64::new(0);
static FIXDIAG_SEED0: AtomicU64 = AtomicU64::new(0); // provider+O_PROVIDER_SEED(0.5.3=0xeaf8)==0 (seed 없는 provider = 관전sim 후보?) buy 수
static LIVE_ATH: Mutex<Option<HashMap<u64, (String, bool)>>> = Mutex::new(None); // 관전 buy 선수: athlete+0x810 → (champ, is_my_athlete)
static MY_BUY: Mutex<Option<HashMap<u64, (String, bool, bool)>>> = Mutex::new(None); // 내선수(is_player) buy: id → (champ, 관전seen, 배경seen). 배경seen=true면 배경 주입 확정.
// World+0x840 참가자 정순회(count +0x848, stride 0x8d0) → (lineup_fp, sel_side, pcount). VEH 읽기만.
unsafe fn roster_scan(world: usize) -> Option<(u64, i8, u32, u32, String)> {
    let base = safe_read_u64(world + 0x840)? as usize;
    let count = safe_read_u64(world + 0x848)?;
    if base < 0x10000 || count == 0 || count > 32 { return None; }
    let mut pairs: Vec<(u64, u64)> = Vec::with_capacity(count as usize);
    let (mut c0, mut c1) = (0u32, 0u32);
    let mut my_hits = 0u32; // ★is_my_athlete(+0x810 멤버십) 히트 = 내 팀 선수 수
    let mut names = String::new();
    for i in 0..count as usize {
        let a = base + i * ATH_STRIDE;
        let Some(side) = safe_read_u64(a + 0x820) else { continue };
        if side > 1 { continue; }
        let aid = safe_read_u64(a + O_ATHLETE_ID).unwrap_or(0); // +0x810 athlete_id
        let mine = matches!(is_my_athlete(a), Some(true));
        if mine { my_hits += 1; }
        let nptr = safe_read_u64(a + 0x420).unwrap_or(0) as usize;
        let nlen = (safe_read_u64(a + 0x428).unwrap_or(0) as usize).min(48);
        if nptr < 0x10000 || nlen == 0 { continue; }
        let mut nb = [0u8; 48];
        let mut off = 0usize; let mut ok = true;
        while off < nlen { let Some(w) = safe_read_u64(nptr + off) else { ok = false; break };
            let n = (nlen - off).min(8); nb[off..off + n].copy_from_slice(&w.to_le_bytes()[..n]); off += 8; }
        if !ok { continue; }
        let mut h = 0xcbf29ce484222325u64;
        for &b in &nb[..nlen] { h = (h ^ b as u64).wrapping_mul(0x100000001b3); }
        pairs.push((side, h));
        if let Ok(cs) = std::str::from_utf8(&nb[..nlen]) {
            let d = is_champ_designated(cs);
            if d { if side == 0 { c0 += 1; } else { c1 += 1; } }
            if names.len() < 800 { names.push_str(&format!("{}[s{}]id={:#x}{}{} ", cs, side, aid, if mine { "✓내팀" } else { "" }, if d { "(D)" } else { "" })); }
        } else if names.len() < 800 { names.push_str(&format!("<bad>[s{}]id={:#x}{} ", side, aid, if mine { "✓내팀" } else { "" })); }
    }
    if pairs.is_empty() { return None; }
    pairs.sort_unstable();
    let mut fp = 0x9e3779b97f4a7c15u64;
    for &(s, hh) in &pairs { fp = (fp ^ ((s << 62) | (hh >> 2))).wrapping_mul(0x100000001b3); }
    let sel = if c0 > c1 { 0i8 } else if c1 > c0 { 1i8 } else { -1i8 };
    Some((fp, sel, pairs.len() as u32, my_hits, names))
}
fn fixdiag_flush() {
    if !FIXDIAG { return; }
    // ★관전 buy 선수 스냅샷(roster 우회): athlete+0x810 직독 id ↔ MY_ATHLETES 매칭.
    let live_rows: Vec<(u64, String, bool)> = {
        let lg = LIVE_ATH.lock().unwrap_or_else(|e| e.into_inner());
        match lg.as_ref() {
            Some(mm) => { let mut v: Vec<(u64, String, bool)> = mm.iter().map(|(k, (c, b))| (*k, c.clone(), *b)).collect(); v.sort_by_key(|x| x.0); v }
            None => Vec::new(),
        }
    };
    // ★내 선수 buy 배경/관전 감지 스냅샷 (표적 기록, 샘플링 없음).
    let my_buy: Vec<(u64, String, bool, bool)> = {
        let mg = MY_BUY.lock().unwrap_or_else(|e| e.into_inner());
        match mg.as_ref() {
            Some(mm) => { let mut v: Vec<(u64, String, bool, bool)> = mm.iter().map(|(k, (c, l, b))| (*k, c.clone(), *l, *b)).collect(); v.sort_by_key(|x| x.0); v }
            None => Vec::new(),
        }
    };
    let mut out = {
    let g = FIXDIAG_MAP.lock().unwrap_or_else(|e| e.into_inner());
    let empty: HashMap<u64, (Option<(u64, i8, u32, u32)>, Option<(u64, i8, u32, u32)>, Option<String>)> = HashMap::new();
    let m = g.as_ref().unwrap_or(&empty);
    let mb_bg = my_buy.iter().filter(|x| x.3).count() as u64;
    let mb_live = my_buy.iter().filter(|x| x.2).count() as u64;
    let sig = (m.len() as u64) ^ ((live_rows.len() as u64) << 40) ^ ((my_buy.len() as u64) << 20) ^ (mb_bg << 30) ^ (mb_live << 50);
    if FIXDIAG_LAST.swap(sig, Ordering::Relaxed) == sig { return; } // 변화 없으면 스킵
    let (mut both, mut fp_ok, mut fp_bad, mut sel_ok) = (0u32, 0u32, 0u32, 0u32);
    let mut fp_seeds: HashMap<u64, u32> = HashMap::new();
    for (_seed, (bg, spec, _)) in m.iter() {
        if let (Some(b), Some(s)) = (*bg, *spec) {
            both += 1;
            if b.0 == s.0 { fp_ok += 1; } else { fp_bad += 1; }
            if b.1 == s.1 { sel_ok += 1; }
        }
        if let Some(fp) = bg.map(|x| x.0).or_else(|| spec.map(|x| x.0)) { *fp_seeds.entry(fp).or_insert(0) += 1; }
    }
    let same_combo = fp_seeds.values().filter(|&&c| c >= 2).count();
    let my_ids: Vec<String> = {
        let p = MY_ATHLETES.load(Ordering::Acquire);
        if p.is_null() { Vec::new() } else { let mut v: Vec<String> = unsafe { (*p).iter().map(|x| format!("{:#x}", x)).collect() }; v.sort(); v }
    };
    let cap_live_seed = LIVE_SEED.load(Ordering::Relaxed);
    let cap_render = RENDER_PROVIDER.load(Ordering::Relaxed);
    let cap_provhit = PROV_HIT.load(Ordering::Relaxed);
    let cap_seedctor = SEEDCTOR_N.load(Ordering::Relaxed);
    let cap_seedmatch = SEEDCTOR_MATCH_N.load(Ordering::Relaxed);
    let cap_seed0 = FIXDIAG_SEED0.load(Ordering::Relaxed);
    let mut out = format!(
        "# item_tactics fix B 상세진단 (내 선수 id + 참가자 id/매치/관전배경)\n\
         # ★내 선수 5명 ID (MY_ATHLETES, {}명): [{}]\n\
         # 관측 seed = {}   배경+관전 둘다 잡힌 seed = {}\n\
         # ★lineup_fp 배경==관전 일치 = {} / 불일치 = {}\n\
         # my=배경 is_my_athlete 히트(내 경기=5, AI끼리=0). 이름줄의 ✓내팀 = MY_ATHLETES 매치.\n\
         # 조합 같은 다른 seed 그룹 = {}개\n\
         # ★관전캡처 상태: LIVE_SEED={:#x} RENDER_PROVIDER={:#x} is_live발화(PROV_HIT)={} | seedctor총={} 렌더캡처={} | seed_r9==0 buy수={}\n\
         #   (LIVE_SEED·RENDER_PROVIDER 둘다 0 = 관전훅 미발화 → is_live 항상 false. seed0 buy>0 = seed없는 provider 존재)\n\n\
         [seed(또는 0x8..=provider)]  배경(my)  관전(my)  판정\n  └ 참가자(이름[side]id=..✓내팀(D))\n",
        my_ids.len(), my_ids.join(", "), m.len(), both, fp_ok, fp_bad, same_combo,
        cap_live_seed, cap_render, cap_provhit, cap_seedctor, cap_seedmatch, cap_seed0);
    // ★배경+관전 쌍(둘다) → 내 선수 든 행 → 나머지 순으로 정렬해 상위 30줄만 출력(중요한 행이 안 잘리게).
    let mut rows: Vec<_> = m.iter().filter(|(_, v)| v.2.is_some()).collect();
    rows.sort_by_key(|(_, v)| {
        let paired = v.0.is_some() && v.1.is_some();
        let my = v.0.as_ref().map(|b| b.3).unwrap_or(0).max(v.1.as_ref().map(|s| s.3).unwrap_or(0));
        (if paired { 0u8 } else { 1u8 }, std::cmp::Reverse(my))
    });
    let mut n = 0;
    for (seed, v) in rows {
        if n >= 30 { break; }
        let (bg, spec, names) = (&v.0, &v.1, &v.2);
        let bgs = bg.as_ref().map(|b| format!("배경(my={})", b.3)).unwrap_or_else(|| "배경(미기록)".into());
        let sps = spec.as_ref().map(|s| format!("관전(my={})", s.3)).unwrap_or_else(|| "관전(미기록)".into());
        let conv = match (bg, spec) { (Some(b), Some(s)) => if b.0 == s.0 { "✅수렴" } else { "❌발산" }, _ => "(한쪽만)" };
        out.push_str(&format!("{:#018x}  {} {}  {}\n  └ {}\n", seed, bgs, sps, conv, names.as_deref().unwrap_or("")));
        n += 1;
    }
    out
    };
    // ★관전 경기에서 실제 아이템 산 선수들 — roster 스캔 우회, 구매자 athlete+0x810 직독.
    out.push_str(&format!("\n[관전 buy 선수 {}명 — roster 우회(구매자 athlete+0x810 직독)]\n\
        #  ✓매치 = 그 선수 id가 MY_ATHLETES(내 5명)에 있음 → athlete_id 조인이 관전 sim에서 유효.\n", live_rows.len()));
    for (id, champ, mine) in &live_rows {
        out.push_str(&format!("  {:>18}  id={:#x}{}\n", champ, id, if *mine { "   ✓MY_ATHLETES매치" } else { "   ✗불일치" }));
    }
    // ★★최종 판정 섹션: 내 선수(is_player)가 배경/관전 각각에서 buy·감지됐나.
    //   배경✓ = 배경 sim에서도 내 선수한테 주입됨 = 관전==확정. 배경✗ = 배경 미주입(문제).
    out.push_str(&format!("\n[★내 선수 buy 배경/관전 감지 {}명 — 표적기록(샘플링X)]\n\
        #  배경✓ 이면 그 선수가 배경 sim에서도 is_my_athlete로 잡혀 주입됨(=fix 작동). 관전✓=화면경기서 잡힘.\n", my_buy.len()));
    for (id, champ, live, bg) in &my_buy {
        out.push_str(&format!("  {:>18}  id={:#x}   관전{}  배경{}\n", champ, id,
            if *live { "✓" } else { "·" }, if *bg { "✓" } else { "·" }));
    }
    if let Some(d) = mod_dir() { let _ = fs::create_dir_all(&d); let _ = fs::write(d.join("fixdiag.txt"), out); }
}
unsafe extern "C" fn buy_replace_ctx(saved: *mut u64, rsp_entry: usize) -> u64 {
    // ★핫패스(rayon 워커 병렬) — 전역 원자 카운터는 캐시라인 경합으로 측정 자체가 부하가 되므로
    //   thread_local 누적(rec_tl) 사용. T_BUY_ALL = 디투어 전체(catch_unwind 포함),
    //   T_BUY_EARLY = 배경 sim 조기탈출분(ALL 에 포함되므로 중복 계상 — 해석 시 차감).
    let __bt = perf::tsc();
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> u64 {
        if saved.is_null() { return 0; } // ★mode=3도 통과(슬롯0/1/2 지정 주입). 4번째 로직만 아래서 mode=4 게이트.
        let athlete = *saved.add(2) as usize; // r8
        if athlete < 0x10000 { return 0; }
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
        // ★검증 진단(개념a): is_live 게이트 전에 배경·관전 양쪽 라인업·SEL 기록. 배경은 1/128 샘플(핫패스 부하억제)·관전은 항상.
        if FIXDIAG && provider_now >= 0x10000 {
            if seed_r9 == 0 { FIXDIAG_SEED0.fetch_add(1, Ordering::Relaxed); }
            // ★관전 sim 후보(seed_r9==0)도 강제 기록. 키=seed_r9 있으면 그 값, 없으면 provider 포인터(최상위비트 마킹).
            let key = if seed_r9 != 0 { seed_r9 } else { (provider_now as u64) | 0x8000_0000_0000_0000 };
            // ★내 선수가 buy하는 배경 sim은 무조건 전체 로스터 스캔·기록(1/128 우회) → 내 경기 배경 pre-sim이 배경(my=N) 행으로 확실히 잡힘.
            let sample = is_live || seed_r9 == 0 || matches!(is_my_athlete(athlete), Some(true)) || (FIXDIAG_CTR.fetch_add(1, Ordering::Relaxed) & 0x7f) == 0;
            if sample {
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    if let Some((fp, sel, pc, my, names)) = roster_scan(provider_now as usize) {
                        let mut g = FIXDIAG_MAP.lock().unwrap_or_else(|e| e.into_inner());
                        let mm = g.get_or_insert_with(HashMap::new);
                        if mm.len() < 4096 {
                            let e = mm.entry(key).or_insert((None, None, None));
                            if is_live { e.1 = Some((fp, sel, pc, my)); } else { e.0 = Some((fp, sel, pc, my)); }
                            if e.2.is_none() { e.2 = Some(names); }
                        }
                    }
                }));
            }
        }
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
        if FIXB && !is_live && !matches!(is_my_athlete(athlete), Some(true)) {
            if BUY_REPORT { BR_TOTAL.fetch_add(1, Ordering::Relaxed); }
            perf::rec_tl(perf::T_BUY_EARLY, __bt);
            return 0;
        }
        // ── 여기부터는 관전 경기 buy(전체 소수) + 배경의 내 선수 buy(5명)만 도달 ──
        // ★athlete 유효성 검사(VirtualQuery)는 여기서 1회 — 위 재정렬 주석 참조.
        if !readable(athlete, 0x4a8) { return 0; } // 0.5.0: build len@+0x4a0+8 커버
        let owned = rd_u64(athlete + 0x458); // 0.5.0 owned (구 0x3d0)
        // ★0.5.3 회귀진단 2단계: "목표를 심었다"와 "실제로 샀다"를 분리 계측.
        //   build[3] 주입은 성공(31회) 확인됨 ⟹ 남은 질문은 게임이 실제로 4번째를 보유하게 되는가.
        //   owned(=보유 아이템 수) 최댓값과 4 이상 도달 횟수를 센다. owned>=4가 0이면 진짜 미구매,
        //   0이 아니면 구매는 되는데 화면(아이콘)에만 안 보이는 것 = DIAG_SLOT_UI_OFF 봉인의 예상된 결과.
        if BUILD_EXT_DIAG {
            if owned >= 4 { BE_CNT[7].fetch_add(1, Ordering::Relaxed); }
            let mx = BE_MAX_OWNED.load(Ordering::Relaxed);
            if owned > mx && owned <= 16 { BE_MAX_OWNED.store(owned, Ordering::Relaxed); }
        }
        if LOG_ENABLED && owned <= 8 { let p = MAX_OWNED4.load(Ordering::Relaxed); if owned > p { MAX_OWNED4.store(owned, Ordering::Relaxed); } }
        // ★진단: 4번째(owned[3]) 티어 진행 추적 — 순차(t0→t4)인지 최종템 직구인지. (프로덕션: LOG_ENABLED 게이트)
        if LOG_ENABLED && owned >= 3 && owned <= 8 {
            let cptr0 = rd_u64(athlete + 0x420) as usize; let clen0 = rd_u64(athlete + 0x428) as usize; // 0.5.0 champ name (구 0x398/0x3a0, +0x88 파생)
            if cptr0 >= 0x10000 && clen0 > 0 && clen0 <= 48 && readable(cptr0, clen0) {
                let cn = String::from_utf8_lossy(std::slice::from_raw_parts(cptr0 as *const u8, clen0)).into_owned();
                let optr = rd_u64(athlete + 0x450) as usize; // 0.5.0 item slot array (구 0x3c8)
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
        let cptr = rd_u64(athlete + 0x420) as usize; // 0.5.0 champ name ptr (구 0x398, +0x88 파생)
        let clen = rd_u64(athlete + 0x428) as usize; // 0.5.0 champ name len (구 0x3a0)
        if cptr < 0x10000 || clen == 0 || clen > 48 || !readable(cptr, clen) { return 0; }
        // ★성능: Cow 차용(유효 UTF-8이면 힙 할당 없음).
        let champ_cow = String::from_utf8_lossy(std::slice::from_raw_parts(cptr as *const u8, clen));
        let champ: &str = champ_cow.as_ref();
        let champ_designated = is_champ_designated(champ); // 스냅샷 zero-alloc
        let side = if readable(athlete + 0x820, 8) { rd_u64(athlete + 0x820) } else { u64::MAX };
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
        let is_player = if is_comptest_live {
            true                        // 조합테스트 = 양 진영 다 유저 구성 → 팀 게이트 우회
        } else if FIXB {
            matches!(is_my_athlete(athlete), Some(true))
        } else {
            is_live && by_scene
        };
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
                } else {
                    BR_BG_PLAYER.fetch_add(1, Ordering::Relaxed);  // ★결함: 내 선수가 아닌데 배경에 주입
                }
            }
        }
        // ★fixdiag: 관전 buy 선수 직접 로깅(roster_scan 우회 — 관전 provider는 +0x840이 로스터가 아님).
        //   구매자 athlete+0x810 ↔ MY_ATHLETES 매칭 여부를 그 자리에서 확인 = athlete_id 조인이 관전에서 되는가.
        if FIXDIAG && is_live {
            let aid = safe_read_u64(athlete + O_ATHLETE_ID).unwrap_or(0);
            let mine = matches!(is_my_athlete(athlete), Some(true));
            let mut lg = LIVE_ATH.lock().unwrap_or_else(|e| e.into_inner());
            let mm = lg.get_or_insert_with(HashMap::new);
            if mm.len() < 64 { mm.entry(aid).or_insert_with(|| (champ.to_string(), mine)); }
        }
        // ★★fixdiag 표적: 내 선수(is_player)가 buy할 때마다 배경/관전 감지 플래그 기록(샘플링 없음).
        //   배경seen=true = 배경 sim에서도 내 선수가 is_my_athlete로 잡혀 주입됨 = 관전==확정 성립.
        if FIXDIAG && is_player {
            let aid = safe_read_u64(athlete + O_ATHLETE_ID).unwrap_or(0);
            let mut mg = MY_BUY.lock().unwrap_or_else(|e| e.into_inner());
            let mm = mg.get_or_insert_with(HashMap::new);
            if mm.len() < 32 {
                let e = mm.entry(aid).or_insert_with(|| (champ.to_string(), false, false));
                if is_live { e.1 = true; } else { e.2 = true; }
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
                    let optr = rd_u64(athlete + 0x450) as usize;
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
                        let optr = rd_u64(athlete + 0x450) as usize;
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
        // ★★출력 덮어쓰기(ghidra-re 확정, 0.5.1): 이번 buy가 채울 슬롯(=owned)이 지정됐으면 리졸버 출력 RDX를 목표 인덱스로 강제.
        //   saved[1]=RDX슬롯=목표, saved[6]=RAX슬롯=1 → 트램폴린 HANDLED가 rdx=목표·rax=1 반환 → caller가 arr[목표] 직접 구매.
        //   골드부족=caller 조용히 no-op(재시도), 크래시X. build 조작이 리졸버에 무시되던 근본문제 우회.
        if OUTPUT_OVERRIDE && is_player && owned < 4 {
            let si = owned as u8;
            let ctxo = rd_u64(rsp_entry + 0x30) as usize;
            let tgt: Option<u64> = if let Some(vid) = slotN_vanilla_id(scope, champ, si) {
                Some(vid) // 바닐라 최종 = 컬렉션 인덱스(id==index)
            } else if let Some(mk) = slotN_item_key(scope, champ, si) {
                if ctxo >= 0x10000 { scan_idx_cached(ctxo, mk.as_bytes()) } else { None }
            } else { None };
            if let Some(t) = tgt {
                let coll = if ctxo >= 0x10000 && readable(ctxo, 0x38) { rd_u64(ctxo + 0x30) as usize } else { 0 };
                let cnt = if coll >= 0x10000 && readable(coll, 0x18) { rd_u64(coll + 0x10) } else { 0 };
                if cnt > 0 && t < cnt { // 범위검증(caller 0x2238429 범위패닉 방지)
                    *saved.add(1) = t;  // RDX = 목표 컬렉션 인덱스
                    *saved.add(6) = 1;  // RAX = 1 (caller leaf 게이트 통과)
                    if BUY_REPORT { BR_WROTE.fetch_add(1, Ordering::Relaxed); }
                    return 1;           // HANDLED → 트램폴린이 saved[1]→rdx, saved[6]→rax, ret
                }
            }
        }
        // ★슬롯0/1/2 지정(모드템/바닐라) → build Vec 목표를 그 카탈로그 인덱스로 (라이브 buy 경로, 슬롯3과 동일).
        //   아직 안 산 슬롯(owned<=si)만 → 게임이 그 인덱스를 향해 자연 빌드업. 바닐라=id, 모드템=이름스캔(레시피검증).
        if SLOT012_INJECT_ENABLED && is_player {
            let ctx012 = rd_u64(rsp_entry + 0x30) as usize;
            let bptr = rd_u64(athlete + 0x498) as usize; // 0.5.0 build ptr
            let blen = rd_u64(athlete + 0x4a0);          // 0.5.0 build len
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
                            // ★리졸버 진단: write 직전 build len·컬렉션 count·인덱스 t 유효성·coll[t] 실제이름 캡처(24건).
                            if RESOLVER_DIAG {
                                let n = RESDIAG_N.fetch_add(1, Ordering::Relaxed);
                                if n < 24 {
                                    let cap = rd_u64(athlete + 0x490);
                                    let coll = rd_u64(ctx012 + 0x30) as usize;
                                    let ccount = if readable(coll, 0x18) { rd_u64(coll + 0x10) } else { u64::MAX };
                                    let nm = catalog_name_at(ctx012, t).unwrap_or_else(|| "?".into());
                                    let line = format!("{} s{} owned={} blen={} cap={} t={} count={} t<count={} coll[t]='{}'\n",
                                        champ, si, owned, blen, cap, t, ccount, t < ccount, nm);
                                    { let mut b = RESDIAG_BUF.lock().unwrap_or_else(|e| e.into_inner()); b.push_str(&line);
                                      if let Some(d) = mod_dir() { let _ = fs::write(d.join("4items_resolver.txt"), b.clone()); } }
                                }
                            }
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
                let bl = rd_u64(athlete + 0x4a0); // 0.5.0 build len (구 0x418)
                let manual = slot3_item_key(scope, champ).is_some();
                let bp = rd_u64(athlete + 0x498) as usize; // 0.5.0 build ptr (구 0x410)
                let (mut b0, mut b1, mut b2, mut b3) = (0u64, 0u64, 0u64, 0u64);
                if bp >= 0x10000 && readable(bp, 32) { b0 = rd_u64(bp); b1 = rd_u64(bp + 8); b2 = rd_u64(bp + 16); b3 = rd_u64(bp + 24); }
                let mx = MAX_OWNED4.load(Ordering::Relaxed);
                cl.push(champ.to_string()); drop(cl);
                append_log("4items_buy4.txt", &format!("[owned==3] champ={} build_len={} build=[{},{},{},{}] manual={} MAX_OWNED={}", champ, bl, b0, b1, b2, b3, manual, mx));
            }
        }
        // ★★구매 순서 진단(2026-07-30, "4번째를 먼저 산다" 규명): 내 팀 선수의 build[] 배열 스냅샷을
        //   (champ, owned) 조합당 1회 기록. 게임이 실제로 보는 목표 순서 = build[0..len]이고, 그 각 인덱스가
        //   어떤 아이템인지(카탈로그 이름) + 지금 보유 개수(owned)를 남겨 "게임이 어느 build 슬롯을 먼저
        //   완성해 가는가"를 직접 본다. build 확장·slot012 주입이 **둘 다 끝난 뒤** 시점이라 최종 배열이 보인다.
        if BUY_ORDER_DIAG && is_player {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let ctx = rd_u64(rsp_entry + 0x30) as usize;
                let bp = rd_u64(athlete + 0x498) as usize;
                let bl = rd_u64(athlete + 0x4a0);
                if ctx < 0x10000 || bp < 0x10000 || bl == 0 || bl > 8 || !readable(bp, (bl as usize)*8) { return; }
                let key = format!("{}#{}", champ, owned);
                let mut seen = BUY_ORDER_SEEN.lock().unwrap_or_else(|e| e.into_inner());
                let set = seen.get_or_insert_with(std::collections::HashSet::new);
                if set.contains(&key) || set.len() > 200 { return; }
                set.insert(key);
                let mut line = format!("{} owned={} build_len={} build=[", champ, owned, bl);
                for i in 0..bl as usize {
                    let idx = rd_u64(bp + i*8);
                    let nm = catalog_name_at(ctx, idx).unwrap_or_else(|| "?".into());
                    line.push_str(&format!("{}={} ", idx, nm));
                }
                line.push_str("]\n");
                let mut buf = BUY_ORDER_BUF.lock().unwrap_or_else(|e| e.into_inner());
                buf.push_str(&line);
                if let Some(d) = mod_dir() { let _ = fs::write(d.join("buy_order.txt"), buf.clone()); }
            }));
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
        let mut build_len = rd_u64(athlete + 0x4a0); // 0.5.0 build len (구 0x418)
        // ★0.5.3 회귀진단(2026-07-29): "4번째만 안 사진다" 원인 절단용. detour 안에서는 **카운터만**(파일IO 금지 —
        //   rayon 워커 병렬 detour에서 동기 IO = 폭주 크래시). 실제 파일 출력은 post_update(메인 스레드)에서.
        if BUILD_EXT_DIAG {
            BE_CNT[0].fetch_add(1, Ordering::Relaxed); // 4번째 경로 도달
            let cap_now = rd_u64(athlete + 0x490);
            BE_LAST.store((build_len << 32) | (cap_now & 0xffff_ffff), Ordering::Relaxed); // 마지막 관측 (len,cap)
            if build_len != 3 { BE_CNT[1].fetch_add(1, Ordering::Relaxed); }
            if cap_now != 3 { BE_CNT[2].fetch_add(1, Ordering::Relaxed); }
        }
        if build_len == 3 && rd_u64(athlete + 0x490) == 3 { // 0.5.0 build cap (구 0x408)
            let ptr = rd_u64(athlete + 0x498) as usize; // 0.5.0 build ptr (구 0x410)
            if !(ptr >= 0x10000 && readable(ptr, 24) && writable(athlete + 0x490, 0x18)) {
                if BUILD_EXT_DIAG { BE_CNT[3].fetch_add(1, Ordering::Relaxed); } // ptr/writable 실패
            }
            if ptr >= 0x10000 && readable(ptr, 24) && writable(athlete + 0x490, 0x18) {
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
                if t4.is_none() && BUILD_EXT_DIAG { BE_CNT[4].fetch_add(1, Ordering::Relaxed); } // 목표 인덱스 획득 실패
                if let Some(t) = t4 {
                    let realloc: ReallocFn = core::mem::transmute(exe_base_addr() + RVA_REALLOC);
                    let np = realloc(ptr, 24, 8, 32);
                    if BUILD_EXT_DIAG && !(np >= 0x10000 && writable(np, 32)) { BE_CNT[5].fetch_add(1, Ordering::Relaxed); } // realloc 실패
                    if np >= 0x10000 && writable(np, 32) {
                        wr_u64(np + 24, t); // ★ build[3] = 수동/신경망 인덱스 or 바닐라 폴백
                        wr_u64(athlete + 0x498, np as u64); wr_u64(athlete + 0x490, 4); wr_u64(athlete + 0x4a0, 4); // 0.5.0 build ptr/cap/len
                        build_len = 4;
                        if BUILD_EXT_DIAG { BE_CNT[6].fetch_add(1, Ordering::Relaxed); BE_LAST_T.store(t, Ordering::Relaxed); } // ★성공: build[3] write
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
unsafe fn install_replace_buy(rva: usize, orig_len: usize, cap_fn: usize) -> Result<usize, &'static str> {
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
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0xff,0x25,0x00,0x00,0x00,0x00]); s.extend_from_slice(&ret_addr.to_le_bytes());
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
static SPAWN_SCENE_OK: AtomicU64 = AtomicU64::new(0);   // 진단: 스폰훅서 조기 side 판정 성공 횟수
static SPAWN_NO_DB: AtomicU64 = AtomicU64::new(0);      // 진단: 스폰 시점 LIVE_DB 부재(InGame 전 스폰)
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
    let ok = unsafe { readable(fn_addr, 12) } && (0..12).all(|i| unsafe { *((fn_addr + i) as *const u8) } == BUY_PROLOGUE[i]);
    if !ok { BUY_PROBE_INSTALLED.store(2, Ordering::Relaxed);
        append_log("4items.txt", &format!("[{}ms] buy_item 프롤로그 mismatch → replace 미설치", now_ms())); return; }
    // orig_len=19: 0.5.1 신 프롤로그 5push(7)+sub rsp,0x50(4)=11B는 jmp패치(12B)를 못 덮음 → 다음 클린경계 11+mov rax,[rsp+0xa8](8)=19B로 재배치.
    match unsafe { install_replace_buy(RVA_BUY_ITEM, 19, buy_replace_ctx as usize) } {
        Ok(_) => BUY_PROBE_INSTALLED.store(1, Ordering::Relaxed),
        Err(_) => BUY_PROBE_INSTALLED.store(2, Ordering::Relaxed),
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
    let sig = base + 0xf24a39; // 0.5.3(구0.5.2=0x2341440). 컨테이너 0x233e9d0→0xf21fe0.
    let imm = base + 0xf24a40; // cmp 의 imm8 (=sig+7)
    let expect = [0x48u8, 0x83, 0xbe, 0x58, 0x04, 0x00, 0x00, 0x03];
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
    let sig = base + 0xd0c9be;          // 0.5.3(구0.5.2=0x211e428): resolver 컨테이너 0x211e150→**0xd0c770**(buy 0xd0c680이 직접 호출). 스필 슬롯이 rsp+0x78→**rsp+0x40**으로 이동했고 `cmp qword[rsp+0x40],2;jbe` 형태는 신 exe 전체 **유일 1건**(바이트스캔 실측). ↓0.5.2 이력: (구0.5.1=0x1f01448): resolver 컨테이너 0x1f01170→0x211e150(스켈레톤 UNIQUE, +0x21cfe0) 동일 오프셋 +0x2d8, 7B 시그 바이트동일(BYTE-OK). ↓0.5.1 이력: (구0.5.0_3=0x1fb8cdd, ghidra-re HIGH 재-ID). resolver 후계 FUN_141f01170 내부. owned_count가 [rsp+0x78]로 spill돼 시퀀스가 'cmp qword[rsp+0x78],2;jbe'로 재작성됨(구 'mov rsi,[rsp+0x40];jbe').
    let jbe = base + 0xd0c9c4;          // 0.5.3 jbe 의 opcode 바이트 (=sig+6, 구0.5.2=0x211e42e). owned≤2→점프, >2 fall-through(has_recipe 추가검사).
    let expect = [0x48u8, 0x83, 0x7c, 0x24, 0x40, 0x02, 0x76]; // 0.5.3: cmp qword[rsp+0x40],2 ; jbe (구0.5.2=rsp+0x78)
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

// ★AI 자동추천 4번째: beam depth 상한 리터럴 2→3 (0,1,2,3=4회 반복 → beam이 4-item build 계산).
//   2곳(진입가드 0x19f14a5·백엣지 0x19f1a11), 둘 다 `cmp r8d,2`(41 83 f8 02) imm8 02→03. 반드시 둘 다.
//   (extractor RE: slot write는 personal_tactics 오버라이드일뿐, 4번째=auto라 beam값 유지 → depth만 늘리면 됨.)
// ★진단(타이틀복귀 크래시 이분탐색 #2): beam_depth OFF. AUTO 4번째는 buy 시점 forward(compute_auto_4th_id)
//   로 하므로 beam_depth(2→3)는 레거시 — 끄면 beam이 4-item 빌드 안 만듦(내부버퍼 OOB 의심 제거). 4개 구매는 유지.
const AUTO4_BEAM_DEPTH: bool = false;
unsafe fn patch_beam_depth() -> String {
    let base = exe_base_addr();
    let mut msgs = Vec::new();
    // ⚠0.5.2·0.5.3 STALE: 이 두 주소는 0.5.1 시점에 이미 시그 불일치(=패치 스킵, fail-safe)·AUTO4_BEAM_DEPTH=false로 미실행.
    for (name, rva) in [("A", 0x19f14a5usize), ("B", 0x19f1a11usize)] {
        let addr = base + rva;
        if !readable(addr, 4) { msgs.push(format!("{}:unreadable", name)); continue; }
        let b = [*(addr as *const u8), *((addr + 1) as *const u8), *((addr + 2) as *const u8), *((addr + 3) as *const u8)];
        if b != [0x41, 0x83, 0xf8, 0x02] { // cmp r8d, 2
            msgs.push(format!("{}:mismatch[{:02x} {:02x} {:02x} {:02x}]", name, b[0], b[1], b[2], b[3]));
            continue;
        }
        const RWX: u32 = 0x40;
        let mut old = 0u32;
        if VirtualProtect(addr + 3, 1, RWX, &mut old) == 0 { msgs.push(format!("{}:vprot", name)); continue; }
        *((addr + 3) as *mut u8) = 0x03;
        VirtualProtect(addr + 3, 1, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), addr + 3, 1);
        msgs.push(format!("{}:OK(02→03)", name));
    }
    format!("beam_depth: {}", msgs.join(", "))
}

// ★후보 게이트 무력화: FUN_141a35490(레시피/개수 필터)를 always-true(mov al,1;ret)로 →
//   3-완성 빌드에도 4번째 후보 유지 → beam이 4-빌드 생성(신경망이 최고점 4번째 선택).
//   단일 호출처(0x142145ce0), 부작용=beam 후보확장 국한(RE 확인).
const CAND_GATE_ON: bool = false; // ★OFF: always-true가 beam 빌드 생성 깨뜨림(beam4→0 실측)
const CAND_GATE_RVA: usize = 0x1a3b280; // ⚠0.5.2·0.5.3 STALE(exe2exe NO MATCH·CAND_GATE_ON=false라 무영향) // 0.5.0_3(구0.5.0_2=0x1a35490, 모노모픽3중 anchor-region 0x1a3bxxx 확정). CAND_GATE_ON=false(OFF, prologue self-guard)
unsafe fn patch_cand_gate() -> String {
    let base = exe_base_addr();
    let addr = base + CAND_GATE_RVA;
    if !readable(addr, 3) { return "cand_gate: unreadable".into(); }
    let b0 = *(addr as *const u8);
    if b0 == 0xB0 { return "cand_gate: already".into(); }
    // 프롤로그 sanity: 함수 첫바이트가 흔한 프롤로그(push/sub/mov)인지 (0x40~0x57/0x48 등) — 아니면 abort.
    if !(b0 == 0x48 || b0 == 0x40 || b0 == 0x55 || b0 == 0x53 || b0 == 0x56 || b0 == 0x57 || (0x41..=0x41).contains(&b0)) {
        return format!("cand_gate: prologue?[{:02x}] abort", b0);
    }
    const RWX: u32 = 0x40; let mut old = 0u32;
    if VirtualProtect(addr, 3, RWX, &mut old) == 0 { return "cand_gate: vprot".into(); }
    *(addr as *mut u8) = 0xB0;       // mov al, 1
    *((addr + 1) as *mut u8) = 0x01;
    *((addr + 2) as *mut u8) = 0xC3; // ret
    VirtualProtect(addr, 3, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, 3);
    format!("cand_gate: patched always-true (was {:02x})", b0)
}

// ══ 경기중 UI 4번째 슬롯 아이콘 표시 (슬롯 루프상한 패치 + slot경로 헬퍼) ══════════
const RVA_SLOT_HELPER: usize = 0xc5cd80; // ⚠**0.5.3 대응 없음(함수 소멸=메가함수 0xa5c1e0에 인라인)**. DIAG_SLOT_UI_OFF=true라 미사용·무영향. 값은 0.5.2 것 그대로(이력 보존). 재개 조건 = 위 DIAG_SLOT_UI_OFF 주석 참조.
const BLUE_SLOTS: [&[u8]; 4] = [b"blue_player.slot0", b"blue_player.slot1", b"blue_player.slot2", b"blue_player.slot3"];
const RED_SLOTS: [&[u8]; 4]  = [b"red_player.slot0",  b"red_player.slot1",  b"red_player.slot2",  b"red_player.slot3"];
// slot 아이콘 루프 상한 4곳(blue/red × 창모드/전체화면, cmp reg,0x30→0x40, imm@+3).
// cmp 시작주소, imm(0x30→0x40) @ +3. (구 0.5.0_2=0x54b760/0x54bad0/0x54c1b0/0x54c520 = 전부 오식별이었음.)
// 0.5.0_3 확정(ghidra-re 재탐색): UI 렌더 메가함수 0x414800..0x42b4c5 내 `cmp reg,0x30`은 정확히 이 4개뿐
//   = blue/red × 창모드/전체화면 4루프. patch_slot_ui가 sig 사전검증 → 불일치면 스킵(fail-safe, 크래시 無).
// 0.5.1(구0.5.0_3=0x4186d0/0x418a40/0x419120/0x419490, 전부 mask-sig UNIQUE win=0x60).
// 0.5.2(2026-07-22 exe2exe): 컨테이너 UI 메가함수 0x4b0e70→0x4e07f0(스켈레톤 UNIQUE, +0x2f980) 동일 오프셋 재매핑
//   (+0x3ed0/+0x4240/+0x4920/+0x4c90) — 4곳 전부 신주소 바이트가 구주소와 동일 확인(BYTE-OK). 구0.5.1=0x4b4d40/0x4b50b0/0x4b5790/0x4b5b00.
// ★0.5.3 재핀 완료(2026-07-29, 컨테이너 0x4e07f0→**0xa5c1e0** 내 `cmp reg,0x30` 전수 = 정확히 4개·전부 RBX):
//   0xa63166 / 0xa638df / 0xa64486 / 0xa64c16, 전부 실측 `48 83 fb 30`. imm 위치 = +3 동일.
//   ⚠단 DIAG_SLOT_UI_OFF=true 라 **적용하지 않는다**(위 주석 ② = 4번째 엔트리 자리가 타 지역변수와 충돌 → 크래시).
//   값을 남겨두는 이유 = 재설계 착수 시 재조사 방지.
const SLOT_BOUNDS: [(usize, [u8; 4]); 4] = [
    (0xa63166, [0x48,0x83,0xfb,0x30]), // 0.5.3 blue(창)  — 구0.5.2=0x4e46c0 cmp r14
    (0xa638df, [0x48,0x83,0xfb,0x30]), // 0.5.3 red(창)   — 구0.5.2=0x4e4a30 cmp r15
    (0xa64486, [0x48,0x83,0xfb,0x30]), // 0.5.3 blue(전체)— 구0.5.2=0x4e5110 cmp r14
    (0xa64c16, [0x48,0x83,0xfb,0x30]), // 0.5.3 red(전체) — 구0.5.2=0x4e5480 cmp r14
];
unsafe extern "C" fn fill_slots(buf: *mut u64, len: u64) -> *mut u64 {
    let __ft = perf::tsc();
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if buf.is_null() { return; }
        let slots: &[&[u8]; 4] = if len == 11 { &BLUE_SLOTS } else if len == 10 { &RED_SLOTS } else { return; };
        for i in 0..4 { *buf.add(i * 2) = slots[i].as_ptr() as u64; *buf.add(i * 2 + 1) = slots[i].len() as u64; }
    }));
    perf::rec(perf::S_FILLSLOTS, __ft);
    buf
}
unsafe fn install_helper_replace(rva: usize, cap_fn: usize) -> Result<usize, &'static str> {
    let mbase = exe_base_addr();
    let fn_addr = mbase + rva;
    if !readable(fn_addr, 16) { return Err("unreadable"); }
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 64, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc"); }
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);   // sub rsp,0x28
    s.extend_from_slice(&[0x4c,0x89,0xc2]);        // mov rdx, r8 (len)
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff,0xd0]);             // call rax (rcx=buf)
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);   // add rsp,0x28
    s.extend_from_slice(&[0xc3]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = [0u8; 12];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    Ok(stub)
}
static SLOTUI_DONE: AtomicBool = AtomicBool::new(false);
// ═══════════════════════════════════════════════════════════════════════════
//  ★★0.5.3 슬롯 UI 복구 — "프레임 확장 + 배열 이전" 수술 (2026-07-30)
// ═══════════════════════════════════════════════════════════════════════════
//  왜 0.5.2 방식(헬퍼 replace + 상한 0x30→0x40)이 안 되는가:
//    ① 슬롯 이름 헬퍼(구 0xc5cd80)가 0.5.3에선 UI 메가함수 0xa5c1e0 **안으로 인라인**되어 소멸.
//    ② 4번째 엔트리 자리 rbp+0x10d50/0x10d58을 게임이 **다른 지역변수로 재사용**(전 함수 67곳 참조,
//       그중 4개 루프 **내부**에도 존재: 0xa6339f `cmp rdi,[rbp+0x10d50]` 등) ⟹ 상한만 늘리면 그 정수를
//       문자열 (ptr,len)으로 역참조 = 확정 크래시.
//  해법(이 함수): 배열 자체를 **프레임 최상단 새 영역(rbp+0x10f80, 64B)으로 이전**한다.
//    A) 프롤로그 `mov eax,0x11008` → `0x11048` 로 스택 프레임을 **+0x40 확장**(chkstk 인자·16B 정렬 유지).
//       ⟹ rbp가 0x40 낮아지므로 **프레임 내부 참조는 전부 자동으로 따라온다**(전 rbp+disp가 상대 주소).
//    B) 단 **호출자 스택**(5번째 인자 = 진입 rsp+0x28 = rbp+0x10ff0) 참조 13곳만 절대 위치라
//       `+0x11030` 으로 보정해야 한다. ★사전 실측: `[rsp+X]` 중 X≥0x100 참조가 **0건** =
//       rsp 상대는 전부 shadow space/지역이라 프레임 확장에 안전(이 검증이 수술의 전제).
//    C) 인라인 init 블록 4곳(각 75B, 3쌍만 씀)을 **스텁으로 통째 대체** → 스텁이 새 base에 **4쌍 전부** 기록.
//       스텁은 rax만 파괴하는 순수 mov 나열(호출·스택 사용 0) — init 블록 진입 시 rax는 dead
//       (블록 첫 명령이 `lea rax,..`), 복귀 지점에서도 dead(다음 정의가 `mov rax,[rbp+rbx+...]`) ⟹ 안전.
//    D) 루프 인덱싱 8곳의 disp32를 새 base로, 상한 4곳을 0x30→0x40.
//  ⚠ all-or-nothing: **전 사이트 시그를 먼저 검증**하고 하나라도 어긋나면 **아무것도 건드리지 않는다**
//    (부분 패치 = 프레임만 커지고 배열은 옛 자리 → 즉사). 되돌리기 없는 hot 렌더 함수라 이 규칙이 생명줄.
// ⛔**2026-07-30 인게임 실패 → 즉시 OFF**: 경기 시작 직후 프리즈(멈춤) 재현. 84/84 사이트가 시그 검증을
//   통과했더라도 **런타임 정합이 깨지는 요인이 남아 있다**는 뜻(하단 실패기록 참조). 원인 규명 전 재활성 금지.
const SLOT_UI_SURGERY: bool = false;        // ★0.5.3 슬롯 UI 수술 마스터 스위치 — 실패로 OFF(아이콘만 없음, 나머지 정상)
const UI_MEGA_PROLOGUE_IMM: usize = 0xa5c1ed; // `mov eax,imm32` 의 imm32 위치
const UI_FRAME_OLD: u32 = 0x11008;
const UI_FRAME_NEW: u32 = 0x11048;          // +0x40 (16B 정렬 유지)
const UI_ARG5_OLD: u32 = 0x10ff0;           // 5번째 인자 = 진입 rsp+0x28
const UI_ARG5_NEW: u32 = 0x11030;           // 프레임이 0x40 커지면 rbp가 0x40 내려가므로 +0x40
const UI_SLOT_BASE_OLD: u32 = 0x10d20;      // 옛 배열 base (게임이 다른 변수와 공유)
const UI_SLOT_BASE_NEW: u32 = 0x10f80;      // 새 배열 base (xmm15 스필 0x10f70 위, 64B 전용)
// 5번째 인자 참조 **68곳** — 전부 `48|4c 8b <modrm> f0 0f 01 00` (mov r64,[rbp+0x10ff0]), 길이 7, disp32 @ +3.
//   ⚠**전수 확인 필수**: 처음에 13곳으로 잘못 셌다(출력 절삭에 잘린 목록을 전부로 착각). 하나라도 빠뜨리면
//   프레임 확장 후 그 명령이 호출자 스택의 엉뚱한 자리를 읽어 **즉사**한다. 실측 = 이 68곳이 전부이고
//   0x10f88(프레임 상한) 이상의 다른 disp는 없다. REX가 0x48뿐 아니라 **0x4c**(r8/r11 대상)도 있으므로
//   아래 검증에서 둘 다 허용한다.
const UI_ARG5_SITES: [usize; 68] = [
    0xa5c3a6, 0xa5c57f, 0xa5ca03, 0xa5cbfc, 0xa5d416, 0xa5d44d, 0xa5d621, 0xa62821,
    0xa62ad5, 0xa62b3f, 0xa62eab, 0xa641b1, 0xa64280, 0xa65274, 0xa652f0, 0xa65355,
    0xa6550e, 0xa6638e, 0xa66430, 0xa665d4, 0xa665f7, 0xa667e4, 0xa6681e, 0xa66d06,
    0xa67367, 0xa675ee, 0xa68777, 0xa68e07, 0xa6908e, 0xa6a5eb, 0xa6abd1, 0xa6afbf,
    0xa6b3ad, 0xa6b45c, 0xa6b5d8, 0xa6c404, 0xa6c4b4, 0xa6c65f, 0xa6c682, 0xa6c860,
    0xa6c89a, 0xa6cd7b, 0xa6d3cb, 0xa6d645, 0xa6e7ce, 0xa6ee57, 0xa6f0ca, 0xa7069b,
    0xa70c91, 0xa7107f, 0xa7146d, 0xa7151c, 0xa72e71, 0xa73092, 0xa732b3, 0xa734d4,
    0xa7369d, 0xa737fe, 0xa73a34, 0xa73cda, 0xa740de, 0xa74e7f, 0xa7529d, 0xa754ef,
    0xa7558f, 0xa7562b, 0xa75a8c, 0xa77a62,
];
// 인라인 init 블록 4곳: (시작, 길이, blue인가) — blue len=0x11("blue_player.slotN"), red len=0x10
const UI_INIT_BLOCKS: [(usize, usize, bool); 4] = [
    (0xa630c2, 0x4b, true),  // blue (창모드)
    (0xa6384a, 0x4b, false), // red  (창모드)
    (0xa643e3, 0x4b, true),  // blue (전체화면)
    (0xa64b6a, 0x4b, false), // red  (전체화면)
];
// 루프 인덱싱 8곳 — `48 8b 84 1d <disp32>`(rax) / `48 8b 8c 1d <disp32>`(rcx), disp32 @ +4
const UI_LOOP_SITES: [(usize, u32); 8] = [
    (0xa63173, 0x10d20), (0xa63182, 0x10d28),
    (0xa638ec, 0x10d20), (0xa638fb, 0x10d28),
    (0xa64493, 0x10d20), (0xa644a2, 0x10d28),
    (0xa64c23, 0x10d20), (0xa64c32, 0x10d28),
];
static SLOTUI_STUBS: [AtomicUsize; 4] = [const { AtomicUsize::new(0) }; 4];
static SLOTUI_MSG: Mutex<Option<String>> = Mutex::new(None); // 수술 결과(진단 덤프에 노출 — LOG_ENABLED 무관)

// 한 사이트의 바이트 시그 대조.
unsafe fn sig_at(addr: usize, want: &[u8]) -> bool {
    if !readable(addr, want.len()) { return false; }
    (0..want.len()).all(|i| *((addr + i) as *const u8) == want[i])
}
unsafe fn write_bytes(addr: usize, data: &[u8]) -> bool {
    const RWX: u32 = 0x40; let mut old = 0u32;
    if VirtualProtect(addr, data.len(), RWX, &mut old) == 0 { return false; }
    core::ptr::copy_nonoverlapping(data.as_ptr(), addr as *mut u8, data.len());
    VirtualProtect(addr, data.len(), old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, data.len());
    true
}
// init 블록 대체 스텁: 새 base에 (ptr,len) 4쌍을 기록하고 블록 끝으로 복귀. rax만 사용.
unsafe fn build_slot_stub(blue: bool, ret_addr: usize) -> Option<usize> {
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let mem = VirtualAlloc(0, 256, MEM_CR, RWX);
    if mem == 0 { return None; }
    let names: &[&[u8]; 4] = if blue { &BLUE_SLOTS } else { &RED_SLOTS };
    let mut s: Vec<u8> = Vec::with_capacity(160);
    for i in 0..4 {
        let d_ptr = UI_SLOT_BASE_NEW + (i as u32) * 0x10;
        let d_len = d_ptr + 8;
        s.extend_from_slice(&[0x48, 0xb8]);                        // movabs rax, <str ptr>
        s.extend_from_slice(&(names[i].as_ptr() as u64).to_le_bytes());
        s.extend_from_slice(&[0x48, 0x89, 0x85]);                  // mov [rbp+d_ptr], rax
        s.extend_from_slice(&d_ptr.to_le_bytes());
        s.extend_from_slice(&[0x48, 0xc7, 0x85]);                  // mov qword [rbp+d_len], imm32
        s.extend_from_slice(&d_len.to_le_bytes());
        s.extend_from_slice(&(names[i].len() as u32).to_le_bytes());
    }
    s.extend_from_slice(&[0x48, 0xb8]);                            // movabs rax, ret
    s.extend_from_slice(&(ret_addr as u64).to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);                            // jmp rax
    if s.len() > 256 { return None; }
    core::ptr::copy_nonoverlapping(s.as_ptr(), mem as *mut u8, s.len());
    Some(mem)
}
unsafe fn patch_slot_ui() -> String {
    let r = patch_slot_ui_inner();
    *SLOTUI_MSG.lock().unwrap_or_else(|e| e.into_inner()) = Some(r.clone());
    r
}
unsafe fn patch_slot_ui_inner() -> String {
    if !SLOT_UI_SURGERY { return "slot_ui: surgery OFF".into(); }
    if SLOTUI_DONE.swap(true, Ordering::Relaxed) { return "slot_ui: already".into(); }
    let base = exe_base_addr();
    if base == 0 { return "slot_ui: no base".into(); }

    // ── ① 전 사이트 사전 검증(all-or-nothing) ──────────────────────────────
    // 프레임 imm32
    if !sig_at(base + UI_MEGA_PROLOGUE_IMM, &UI_FRAME_OLD.to_le_bytes()) {
        return "slot_ui: ABORT(frame imm mismatch) — 미적용".into();
    }
    // 5번째 인자 참조 13곳: `48 8b ?? f0 0f 01 00`
    for &r in UI_ARG5_SITES.iter() {
        let a = base + r;
        // REX = 0x48 또는 0x4c(r8/r11 대상), opcode 0x8b, disp32 @ +3
        let rex_ok = readable(a, 2) && matches!(*(a as *const u8), 0x48 | 0x4c) && *((a + 1) as *const u8) == 0x8b;
        if !(rex_ok && sig_at(a + 3, &UI_ARG5_OLD.to_le_bytes())) {
            return format!("slot_ui: ABORT(arg5 {:#x} mismatch) — 미적용", r);
        }
    }
    // 루프 인덱싱 8곳: `48 8b <modrm> 1d <disp32>` (rbp+rbx 인덱싱), disp32 @ +4
    for &(r, d) in UI_LOOP_SITES.iter() {
        let a = base + r;
        if !(sig_at(a, &[0x48, 0x8b]) && sig_at(a + 3, &[0x1d]) && sig_at(a + 4, &d.to_le_bytes())) {
            return format!("slot_ui: ABORT(loop {:#x} mismatch) — 미적용", r);
        }
    }
    // 루프 상한 4곳
    for (r, sig) in SLOT_BOUNDS.iter() {
        if !sig_at(base + r, sig) { return format!("slot_ui: ABORT(bound {:#x} mismatch) — 미적용", r); }
    }
    // init 블록 4곳: 첫 명령이 `lea rax,[rip+..]`(48 8d 05) 인지
    for &(r, _l, _b) in UI_INIT_BLOCKS.iter() {
        if !sig_at(base + r, &[0x48, 0x8d, 0x05]) {
            return format!("slot_ui: ABORT(init {:#x} mismatch) — 미적용", r);
        }
    }
    // 스텁 4개 준비(하나라도 실패하면 중단 — 아직 게임 코드는 안 건드린 상태)
    let mut stubs = [0usize; 4];
    for (i, &(r, l, blue)) in UI_INIT_BLOCKS.iter().enumerate() {
        match build_slot_stub(blue, base + r + l) {
            Some(m) => { stubs[i] = m; SLOTUI_STUBS[i].store(m, Ordering::Relaxed); }
            None => return "slot_ui: ABORT(stub alloc) — 미적용".into(),
        }
    }

    // ── ② 적용(여기서부터는 전부 성공해야 정합) ─────────────────────────────
    let mut done = 0;
    // 루프 인덱싱 → 새 base
    for &(r, d) in UI_LOOP_SITES.iter() {
        let nd = UI_SLOT_BASE_NEW + (d - UI_SLOT_BASE_OLD);
        if write_bytes(base + r + 4, &nd.to_le_bytes()) { done += 1; }
    }
    // 상한 0x30 → 0x40
    for (r, _sig) in SLOT_BOUNDS.iter() {
        if write_bytes(base + r + 3, &[0x40]) { done += 1; }
    }
    // 5번째 인자 참조 보정
    for &r in UI_ARG5_SITES.iter() {
        if write_bytes(base + r + 3, &UI_ARG5_NEW.to_le_bytes()) { done += 1; }
    }
    // init 블록 → 스텁 점프(12B) + 나머지 nop
    for (i, &(r, l, _b)) in UI_INIT_BLOCKS.iter().enumerate() {
        let mut patch = vec![0x90u8; l];
        patch[0] = 0x48; patch[1] = 0xb8;
        patch[2..10].copy_from_slice(&stubs[i].to_le_bytes());
        patch[10] = 0xff; patch[11] = 0xe0;
        if write_bytes(base + r, &patch) { done += 1; }
    }
    // ★프레임 확장은 **맨 마지막**(이게 먼저 들어가면 그 순간부터 옛 배열 참조가 전부 어긋난다)
    let frame_ok = write_bytes(base + UI_MEGA_PROLOGUE_IMM, &UI_FRAME_NEW.to_le_bytes());
    format!("slot_ui: 수술 적용 {}/84 사이트 + 프레임확장 {} (base {:#x}→{:#x})",
            done, if frame_ok { "OK" } else { "FAIL★" }, UI_SLOT_BASE_OLD, UI_SLOT_BASE_NEW)
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
const GAME_EXE_SIZE_053: u64 = 74_970_624;
static VERSION_OK: AtomicBool = AtomicBool::new(false);
static VERSION_MSG: Mutex<String> = Mutex::new(String::new());
/// 0.5.3 인지 판정. init 에서 1회 호출하고 결과를 VERSION_OK 에 남긴다.
fn check_game_version() -> bool {
    let mut why = String::new();
    // ① exe 크기
    let size_ok = match exe_path().and_then(|p| fs::metadata(p).ok()) {
        Some(m) => {
            let sz = m.len();
            if sz == GAME_EXE_SIZE_053 { true }
            else { why = format!("exe 크기 불일치: {}B (0.5.3 = {}B)", sz, GAME_EXE_SIZE_053); false }
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
        if ok { "0.5.3 확인 — 정상 활성".to_string() }
        else { format!("★버전 불일치 → 모드 전체 비활성 ({})", why) };
    VERSION_OK.store(ok, Ordering::Relaxed);
    ok
}
/// 게이트 통과 여부(런타임 훅/패치 진입부에서 조회).
#[inline]
fn version_ok() -> bool { VERSION_OK.load(Ordering::Relaxed) }

fn init(_ctx: &GameCtx) -> ModRegistration {
    // ★★버전 게이트: 0.5.3 이 아니면 **훅·패치를 하나도 설치하지 않고** 빈 등록만 반환한다.
    //   (하드코딩 RVA·바이트패치·구조체 오프셋 의존이라 다른 버전에선 오작동 위험)
    if !check_game_version() {
        let msg = VERSION_MSG.lock().unwrap_or_else(|e| e.into_inner()).clone();
        // LOG_ENABLED 무관하게 1회 남긴다(유저가 왜 비활성인지 알 수 있게).
        if let Some(d) = mod_dir() { let _ = fs::create_dir_all(&d);
            let _ = fs::write(d.join("version_gate.txt"),
                format!("{}

이 모드는 게임 0.5.3 전용입니다.
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
        // ★진단(타이틀복귀 크래시 이분탐색): 슬롯 UI 패치(상한 0x30→0x40 + 헬퍼 0xbbbd60 full-replace) OFF.
        //   크래시 사라지면 UI 패치가 원인(→헬퍼 트램폴린/컨텍스트 게이트). 여전하면 sim 쪽.
        if !DIAG_SLOT_UI_OFF {
            let rs = unsafe { patch_slot_ui() }; // 경기중 4번째 슬롯 아이콘
            append_log("4items.txt", &format!("[{}ms] {}", now_ms(), rs));
        } else {
            append_log("4items.txt", &format!("[{}ms] [진단] patch_slot_ui SKIP (UI 패치 OFF)", now_ms()));
        }
        if AUTO4_BEAM_DEPTH {
            let rb = unsafe { patch_beam_depth() };
            append_log("4items.txt", &format!("[{}ms] {}", now_ms(), rb));
            // ★cand_gate 패치는 beam 빌드 생성을 오히려 깨뜨림(beam4 0으로) → 비활성.
            if CAND_GATE_ON {
                let rc = unsafe { patch_cand_gate() };
                append_log("4items.txt", &format!("[{}ms] {}", now_ms(), rc));
            }
        }
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
