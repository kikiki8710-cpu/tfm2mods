//! tfm2_stat_exp — 판단력(judgement)/오더(order) 강제 덮어쓰기 통제실험 모드 (0.5.5 전용)
//! ===========================================================================
//! 목적: 다시보기(및 경기)에서 지정 선수의 판단력·오더를 독립적으로 강제값으로 덮어
//!       행동 변화를 관찰하는 통제실험. cfg 값이 -1 이면 그 스탯은 안 건드림(무개입).
//!
//! ★안전 최우선(CLAUDE.md §3):
//!   - Game 캡처 = 런처 체인 후킹(item_tactics install_launcher_hook 이식). 최소 디투어(rcx→Game 저장만).
//!   - 실제 poke 는 post_update 콜백에서. 모든 raw read/write = VEH 경유 safe_read/safe_write.
//!   - detour·post_update 본문 = catch_unwind. 포인터 산술 = wrapping_add + 범위체크.
//!   - cfg 전부 -1(기본) = 어떤 write 도 안 함(안전 기본값).
//!   - ★오프셋 체인이 ghidra-re 로 확정되기 전엔 CHAIN_CONFIRMED=false → write 원천차단.
//! ===========================================================================
#![allow(dead_code, unused_imports, unused_variables)]
use mod_api::*;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "tfm2_stat_exp";

// ───────────────────────────────────────────────────────────────────────────
//  오프셋 체인 (0.5.5)  — ★확정 전엔 CHAIN_CONFIRMED=false 로 write 차단
// ───────────────────────────────────────────────────────────────────────────
// 런처 훅(체인) — item_tactics 와 동일 RVA/프롤로그(0.5.5). rcx(arg1)=Game.
const CL_LAUNCHER_RVA: usize = 0x14ac3e0;
const CL_LAUNCHER_ORIG_LEN: usize = 12; // 8push(12B). chkstk call 제외 재배치(item_tactics 동일).
const CL_LAUNCHER_PROLOGUE: [u8; 17] = [0x55, 0x41,0x57, 0x41,0x56, 0x41,0x55, 0x41,0x54, 0x56, 0x57, 0x53, 0xb8, 0x38, 0x54, 0x02, 0x00];

// ★런처가 캡처한 Game 에서 참가자 레코드까지의 오프셋 체인 (0.5.5, ghidra-re 명령레벨 확증 2026-08-13).
//   RE 정본 = REPORT\tfm2_stat_exp\RE\2026-08-13_Game-참가자-판단력오더-체인-0.5.5.md
//   World      = *(Game + 0x1dc0)          // 엔진 dyn fat-ptr data 슬롯 (Game 임베드 아님 = 포인터 역참조)
//   ath_base   = *(World + 0x858)          // 참가자 레코드 배열 base 포인터 (0.5.4 0x840→0.5.5 +0x18 시프트)
//   ath_count  = *(World + 0x860) u64      // 배열 원소 수 상한
//   record[i]  = ath_base + i*0x9e0        // 원소 stride 0x9e0
//   판단력=record+0x450 / 오더=record+0x458 / flag=record+0x9c8(==1) / team(side)=record+0x930
//   ★안전순회: unit-slot 리졸버 경유(매치 참가자만) — 직접 배열순회는 글로벌 athlete 풀 가능성(미확정) 회피.
//     for s in 0..*(World+0x878): e=*(World+0x870)+s*0x10; gen=*(e)==1; aidx=*(e+8)<ath_count; …
const O_GAME_WORLD: usize = 0x1dc0;    // Game→World (포인터 역참조). CONFIRMED (run_one_tick 0x14aa160)
const O_ATH_ARR: usize   = 0x858;      // World→참가자배열 base 포인터. CONFIRMED (0x14f11b0 MOV RAX,[RCX+0x858])
const O_ATH_COUNT: usize = 0x860;      // World→참가자 수. CONFIRMED (CMP RDX,[RCX+0x860])
const ARR_STRIDE: usize  = 0x9e0;      // 레코드 stride. CONFIRMED (imul 0x9e0)
// ★2026-08-13 정정: 판단력=record+0x218, 오더=record+0x220 (AthleteStat 임베드 base+0x1e0의 +0x38/+0x40).
//   ~~+0x450=C계수(런타임 objective base ‰~1000)·+0x458=오더 콜 payload(연출)~~ = 둘 다 능력치 아님·덮으면 안 됨.
//   근거(ghidra 게임함수 디스어셈): retreat_engage `[RAX+0x220]`+clamp(order,100)×6+400(오더콜 확률식) /
//   objective calc `[R13+0x218]`=판단력 A. old값이 능치범위(~0~100대)로 나오면 정합, 1000/큰값이면 아직 틀린 것.
const O_JUDGE: usize     = 0x218;      // 판단력(정적 능치). ghidra 게임함수 디스어셈 확정
const O_ORDER: usize     = 0x220;      // 오더(정적 능치). ghidra 게임함수 디스어셈 확정
const O_REC_FLAG: usize  = 0x9c8;      // 레코드 유효 flag(==1). CONFIRMED
const O_SIDE: usize      = 0x930;      // team(side) u64 — blue/red 구분. CONFIRMED (run_one_tick 읽음)
// ★2026-08-14 정정: 액션상태는 참가자 record+0x517(항상 0)이 아니라 **유닛객체 U+0xA47**(Entity+0x517).
//   참가자(athlete)엔 U 포인터 없음(별개 객체) → behavior 훅(FUN_140eafed0, RCX=U·R8=athlete)에서 함께 받아 집계.
const O_U_STATE: usize   = 0xA47;      // U(유닛객체)+0xA47 = action_state byte. 0x3존·0x5교전·0xb후퇴
// behavior 공통 진입 훅 — 매 틱 유닛마다 호출(hot·멀티스레드). RCX=U, R8=athlete. RE 확정.
//   RE 정본 = REPORT\tfm2_stat_exp\RE\2026-08-14_액션상태-U0xA47-behavior훅.md
const STATE_HOOK_RVA: usize = 0xeafed0;
const STATE_HOOK_ORIG_LEN: usize = 12; // 8push(run_one_tick·seed-ctor 동일 프롤로그)
const STATE_HOOK_PROLOGUE: [u8; 12] = [0x55, 0x41,0x57, 0x41,0x56, 0x41,0x55, 0x41,0x54, 0x56, 0x57, 0x53];

// ★오더 콜큐 (U-상대) — Entity+0x7c8/7d0/7d8 = U+0xCF8(cap)/0xD00(ptr)/0xD08(len). 엔트리 stride 0x18 {tag@0,data@1,q@8}.
const O_Q_PTR: usize = 0xD00;  // U+0xD00 = 큐 데이터 포인터
const O_Q_LEN: usize = 0xD08;  // U+0xD08 = 큐 길이(usize). ★워치 타깃(U 상주 = 안정 주소)
// unit-slot 리졸버(매치 참가자 슬롯) — CONFIRMED (0x14f11b0)
const O_UNIT_ARR: usize   = 0x870;     // World→unit-slot 배열 base 포인터
const O_UNIT_COUNT: usize = 0x878;     // World→unit-slot 수
const UNIT_STRIDE: usize  = 0x10;      // 슬롯 stride: gen@+0(==1) / aidx@+8

// ★2026-08-13: run_one_tick 훅 — 매 틱 sim 함수 인자(rdx=Game)에서 현재 World를 직접 캡처.
//   런처 캡처 Game 은 다시보기서 stale(객체 relocate)이라 신뢰불가 → run_one_tick 인자 우선.
//   ghidra 확정(0x1414aa160): rdx=Game, World=*(Game+0x1dc0). 프롤로그 12B=8push(seed-ctor 동일).
//   RE 정본 = REPORT\tfm2_stat_exp\RE\2026-08-13_run_one_tick-인자-World-훅.md
const RUN_TICK_RVA: usize = 0x14aa160;
const RUN_TICK_ORIG_LEN: usize = 12;
const RUN_TICK_PROLOGUE: [u8; 12] = [0x55, 0x41,0x57, 0x41,0x56, 0x41,0x55, 0x41,0x54, 0x56, 0x57, 0x53];

// ★★오프셋 체인 확정 게이트. ghidra-re 명령레벨 확증 완료 → true.
//   ⚠단 (1)+0x458 "오더" 시맨틱·(2)blue/red team 값 매핑은 인게임 검증 필요(아래 poke 가드가 보호).
//   실제 write 는 여기 true + cfg 값≥0 + old 타당성 통과 3중 조건 모두 충족해야 발생.
const CHAIN_CONFIRMED: bool = true;

// poke 하기 전 old 값 타당성 상한 — 판단력/오더는 작은 정수. 이보다 크면(=포인터/쓰레기)
//   오프셋 오식별로 보고 write 를 건너뛴다(잘못된 주소에 쓰는 사고 방지).
const VAL_SANITY_MAX: u64 = 0x0010_0000; // 약 100만. 판단력 스탯(원본 0~100대)엔 충분히 넉넉.

const MAX_P: usize = 20; // 로그/추적용 참가자 상한

// ───────────────────────────────────────────────────────────────────────────
//  cfg 상태
// ───────────────────────────────────────────────────────────────────────────
// SIDE: 0=blue 1=red 2=all.  SLOT: 0..4, 255=all.  JUDGE/ORDER: -1=무개입.
static CFG_SIDE: AtomicU64 = AtomicU64::new(0);
static CFG_SLOT: AtomicU64 = AtomicU64::new(0);
static CFG_JUDGE: AtomicI64 = AtomicI64::new(-1);
static CFG_ORDER: AtomicI64 = AtomicI64::new(-1);
static CFG_HIST: AtomicBool = AtomicBool::new(true); // 액션상태 히스토그램 on/off(freeze 시 끌 수단)
static CFG_LOADED: AtomicBool = AtomicBool::new(false);

// 진단 카운터
static LIVE_GAME: AtomicU64 = AtomicU64::new(0);   // 런처 캡처 Game(화면 경기)
static LAUNCH_N: AtomicU64 = AtomicU64::new(0);    // 런처 발화 수
static LAUNCH_RENDER_N: AtomicU64 = AtomicU64::new(0); // 화면경기로 판정된 캡처 수
static LAUNCH_RENDER_RA: AtomicU64 = AtomicU64::new(0); // 마지막 화면경기 retaddr rva
static LAUNCH_RVAS: [AtomicU64; 24] = [const { AtomicU64::new(0) }; 24]; // 관측된 고유 콜러 rva
static CLAUNCH_INSTALLED: AtomicU64 = AtomicU64::new(0);
static CLAUNCH_STUB: AtomicU64 = AtomicU64::new(0);
static LAUNCH_WAIT: AtomicU64 = AtomicU64::new(0);
// ★run_one_tick 훅 — 현재 World Game 캡처(다시보기 신뢰소스)
static RT_N: AtomicU64 = AtomicU64::new(0);            // run_one_tick 발화 수
static RT_GAME: AtomicU64 = AtomicU64::new(0);         // 최신 rdx=Game
static RTHOOK_INSTALLED: AtomicU64 = AtomicU64::new(0);
static RTHOOK_STUB: AtomicU64 = AtomicU64::new(0);
static RT_WAIT: AtomicU64 = AtomicU64::new(0);
const RT_TAB_N: usize = 8;                             // 동시 관측 Game 후보 테이블(배경 sim 다수)
static RT_TAB_G: [AtomicU64; RT_TAB_N] = [const { AtomicU64::new(0) }; RT_TAB_N];
static RT_TAB_C: [AtomicU64; RT_TAB_N] = [const { AtomicU64::new(0) }; RT_TAB_N];
static CHOSEN_WORLD: AtomicU64 = AtomicU64::new(0);    // 선택된 World(진단)
static CHOSEN_GAME: AtomicU64 = AtomicU64::new(0);     // ★캐시된 Game — 유효하면 매프레임 전체 재스캔 회피(fault 폭풍 방지)
static CHOSEN_SRC: AtomicU64 = AtomicU64::new(0);      // 0=없음 1=run_tick 2=launcher폴백
static CAND_SEEN: AtomicU64 = AtomicU64::new(0);       // 이번 선택서 검사한 후보 수
static CAND_VALID: AtomicU64 = AtomicU64::new(0);      // 유효 매치배열 후보 수
static CAND_LOG: Mutex<String> = Mutex::new(String::new()); // 후보별 (game·world·유효참가자) 요약
// poke 진단
static PK_FRAMES: AtomicU64 = AtomicU64::new(0);     // post_update 진입 수
static PK_WORLD_OK: AtomicU64 = AtomicU64::new(0);   // World 읽기 성공
static PK_ARR_OK: AtomicU64 = AtomicU64::new(0);     // 배열 base 읽기 성공
static PK_COUNT: AtomicU64 = AtomicU64::new(u64::MAX); // 마지막 관측 count
static PK_REC_OK: AtomicU64 = AtomicU64::new(0);     // 레코드 read 성공
static PK_WROTE: AtomicU64 = AtomicU64::new(0);      // 실제 write 수
static PK_SKIP_SANITY: AtomicU64 = AtomicU64::new(0);// old 값 타당성 실패로 skip
static SIDE_A: AtomicU64 = AtomicU64::new(u64::MAX); // 관측된 blue side 값(작은쪽)
static SIDE_B: AtomicU64 = AtomicU64::new(u64::MAX); // 관측된 red side 값(큰쪽)
// ★2026-08-13 진단강화: "마지막값=0"에 속지 않게 최대값·유효프레임·unit 경로 분리 계측.
static MAX_ATH_COUNT: AtomicU64 = AtomicU64::new(0);  // 관측된 최대 World+0x860(athlete count)
static MAX_UNIT_COUNT: AtomicU64 = AtomicU64::new(0); // 관측된 최대 World+0x878(unit-slot count)
static FRAMES_WITH_ATH: AtomicU64 = AtomicU64::new(0);// ath_count>0 이었던 프레임 수
static DBG_UNIT_COUNT: AtomicU64 = AtomicU64::new(u64::MAX); // 마지막 unit_count(ath_count==0 이어도 읽음)
static DBG_UNIT_BASE_OK: AtomicU64 = AtomicU64::new(0);     // unit_base 유효성(0=미확인 1=유효 2=무효/읽기실패)
static DBG_ATH_BASE_OK: AtomicU64 = AtomicU64::new(0);      // ath_base 유효성(0=미확인 1=유효 2=무효/읽기실패)
static DBG_WORLD: AtomicU64 = AtomicU64::new(0);            // 마지막 World 주소
static RAW_SAMPLED: AtomicBool = AtomicBool::new(false);    // raw 샘플 최초 1회 게이트
static RAW_SAMPLE_N: AtomicU64 = AtomicU64::new(0);         // raw 샘플 횟수(상한)
static RAW_SAMPLE: Mutex<String> = Mutex::new(String::new());
// ★2026-08-14 오더 다운스트림 측정: action_state(+0x517 byte) side별 히스토그램(관찰 전용·read only).
//   [side][state 0..15] 유닛-프레임 누적. state>=16 은 HIST_OTHER. 핵심 = 0xb(후퇴) 비율.
static HIST_STATE: [[AtomicU64; 16]; 2] = [const { [const { AtomicU64::new(0) }; 16] }; 2];
static HIST_OTHER: [AtomicU64; 2] = [const { AtomicU64::new(0) }; 2];
static HIST_FRAMES: AtomicU64 = AtomicU64::new(0); // 액션상태 관측 프레임 수(구·미사용)
// behavior 훅(FUN_140eafed0) 상태 — U+0xA47 액션상태 집계원
static SHOOK_INSTALLED: AtomicU64 = AtomicU64::new(0);
static SHOOK_STUB: AtomicU64 = AtomicU64::new(0);
static SHOOK_WAIT: AtomicU64 = AtomicU64::new(0);
static SHOOK_N: AtomicU64 = AtomicU64::new(0);        // detour 발화 수(=유닛-틱 관측)
static SHOOK_SIDE_OOB: AtomicU64 = AtomicU64::new(0); // side 값이 0/1 아님(스킵)
// ★HW 워치포인트(+0x7c8 오더 콜큐 소비자 포착) — 기본 OFF(cfg watchpoint=1로만 켬). 고위험.
static CFG_WP: AtomicBool = AtomicBool::new(false);   // cfg watchpoint 게이트(기본 0)
const WP_NT: usize = 4;                               // 동시 워치 수(DR0~DR3)
static WP_TARGETS: [AtomicU64; WP_NT] = [const { AtomicU64::new(0) }; WP_NT]; // 서로 다른 U 4개의 +0xD08(len) 주소
static WP_U: AtomicU64 = AtomicU64::new(0);           // 첫 워치 대상 U(로그용)
static WP_ARMED: AtomicBool = AtomicBool::new(false); // DR0 무장 상태
static WP_THREADS: AtomicU64 = AtomicU64::new(0);     // 마지막 apply서 DR 설정한 스레드 수
static WP_APPLY_N: AtomicU64 = AtomicU64::new(0);     // apply 호출 수
static WP_HIT_TOTAL: AtomicU64 = AtomicU64::new(0);   // 총 히트 수
static WP_UNIQUE: AtomicU64 = AtomicU64::new(0);      // 유니크 RIP 수
static WP_DISARMED: AtomicU64 = AtomicU64::new(0);    // 0=활성 1=상한자동해제 2=cfg off해제 3=미설치실패
const WP_SLOTS: usize = 64;
static WP_HITS_OFF: [AtomicU64; WP_SLOTS] = [const { AtomicU64::new(0) }; WP_SLOTS]; // RIP exe-상대 offset
static WP_HITS_CNT: [AtomicU64; WP_SLOTS] = [const { AtomicU64::new(0) }; WP_SLOTS];
const WP_HIT_CAP: u64 = 5000;   // 총 히트 상한 → 자동해제
const WP_UNIQUE_CAP: u64 = 60;  // 유니크 RIP 상한(슬롯 64) → 자동해제
// 마지막 관측 old 값(로그 스팸 방지) — [idx*2 + field]
static LAST_OLD: [AtomicU64; MAX_P * 2] = [const { AtomicU64::new(u64::MAX) }; MAX_P * 2];
static LOG_BUF: Mutex<String> = Mutex::new(String::new());

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
    // ── HW 워치포인트(디버그 레지스터)용 — 스레드 열거·컨텍스트 조작 ──
    fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> isize;
    fn Thread32First(snap: isize, te: *mut ThreadEntry32) -> BOOL;
    fn Thread32Next(snap: isize, te: *mut ThreadEntry32) -> BOOL;
    fn OpenThread(access: u32, inherit: BOOL, tid: u32) -> isize;
    fn SuspendThread(h: isize) -> u32;
    fn ResumeThread(h: isize) -> u32;
    fn CloseHandle(h: isize) -> BOOL;
    fn GetThreadContext(h: isize, ctx: *mut u8) -> BOOL;
    fn SetThreadContext(h: isize, ctx: *const u8) -> BOOL;
    fn GetCurrentThreadId() -> u32;
    fn GetCurrentProcessId() -> u32;
}
#[repr(C)]
#[derive(Default)]
struct MemBasicInfo {
    base: usize, alloc_base: usize, alloc_protect: u32, _pad0: u32,
    region_size: usize, state: u32, protect: u32, mtype: u32, _pad1: u32,
}
#[repr(C)]
#[derive(Default)]
struct ThreadEntry32 {
    dw_size: u32, cnt_usage: u32, th32_thread_id: u32,
    th32_owner_pid: u32, tp_base_pri: i32, tp_delta_pri: i32, dw_flags: u32,
}
// x64 CONTEXT 버퍼(16정렬 필수). 관심 필드 오프셋: ContextFlags@0x30, Dr0@0x48, Dr6@0x68, Dr7@0x70, Rip@0xF8.
#[repr(C, align(16))]
struct CtxBuf([u8; 1256]);
impl CtxBuf { fn new() -> Self { CtxBuf([0u8; 1256]) } }
const CONTEXT_DEBUG_REGS: u32 = 0x0010_0010; // CONTEXT_AMD64 | CONTEXT_DEBUG_REGISTERS

// ===========================================================================
//  포인터 검증 / 메모리 안전
// ===========================================================================
#[inline]
fn ptr_ok(v: usize) -> bool { v >= 0x10000 && v < 0x0000_8000_0000_0000 }

static EXE_BASE_CACHE: AtomicUsize = AtomicUsize::new(0);
fn exe_base_addr() -> usize {
    let b = EXE_BASE_CACHE.load(Ordering::Relaxed);
    if b != 0 { return b; }
    let v = unsafe { GetModuleHandleW(core::ptr::null()) as usize };
    EXE_BASE_CACHE.store(v, Ordering::Relaxed);
    v
}

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

// ── VEH 안전 read/write (접근위반 0xC0000005 을 가로채 크래시 대신 실패 반환) ──
//   TLS 상태(락 없음). 핸들러 안엔 할당/락/패닉 경로 없음(§3 준수).
#[repr(C)]
struct ExceptionRecord { code: u32, flags: u32, rec: usize, addr: usize, nparams: u32, _p: u32, params: [usize; 15] }
#[repr(C)]
struct ExceptionPointers { rec: *mut ExceptionRecord, ctx: *mut core::ffi::c_void }
type VehHandler = extern "system" fn(*mut ExceptionPointers) -> i32;

#[repr(C)]
struct SehTls { v: [core::cell::Cell<u64>; 8] }
thread_local! {
    static SEH_T: SehTls = const { SehTls { v: [const { core::cell::Cell::new(0) }; 8] } };
}
static SEH_INSTALLED: AtomicBool = AtomicBool::new(false);

extern "system" fn seh_veh(p: *mut ExceptionPointers) -> i32 {
    const CONTINUE_EXECUTION: i32 = -1;
    const CONTINUE_SEARCH: i32 = 0;
    unsafe {
        if p.is_null() { return CONTINUE_SEARCH; }
        let rec = (*p).rec;
        if rec.is_null() { return CONTINUE_SEARCH; }
        // ── ★HW 워치포인트(DR0 데이터 브레이크포인트) 히트: STATUS_SINGLE_STEP ──
        //   데이터 bp는 trap(명령 실행 후 보고). RIP 기록·DR6 클리어·계속. VEH 안전: 원자기록만.
        if (*rec).code == 0x80000004 {
            if !WP_ARMED.load(Ordering::Relaxed) { return CONTINUE_SEARCH; }
            let ctx = (*p).ctx as usize;
            if ctx == 0 { return CONTINUE_SEARCH; }
            let dr6 = *((ctx + 0x68) as *const u64);
            if dr6 & 0xF != 0 { // DR0~DR3 중 하나 히트
                let rip = *((ctx + 0xF8) as *const u64);
                let base = EXE_BASE_CACHE.load(Ordering::Relaxed) as u64;
                let off = if base != 0 && rip >= base { rip - base } else { rip };
                wp_record(off);
                *((ctx + 0x68) as *mut u64) = 0; // DR6 클리어
                return CONTINUE_EXECUTION;
            }
            return CONTINUE_SEARCH;
        }
        if (*rec).code != 0xC0000005 { return CONTINUE_SEARCH; }
        let Ok(g) = SEH_T.try_with(|s| s.v.as_ptr() as *mut u64) else { return CONTINUE_SEARCH; };
        if *g.add(0) == 0 { return CONTINUE_SEARCH; }
        let ctx = (*p).ctx as usize;
        if ctx == 0 { return CONTINUE_SEARCH; }
        let rip = *((ctx + 0xF8) as *const u64);
        if rip < *g.add(5) || rip >= *g.add(6) { return CONTINUE_SEARCH; }
        *((ctx + 0xF8) as *mut u64) = *g.add(2); // Rip = land_rip
        *((ctx + 0x98) as *mut u64) = *g.add(3); // Rsp = land_rsp
        *((ctx + 0xA0) as *mut u64) = *g.add(4); // Rbp = land_rbp
        *g.add(7) += 1;
        CONTINUE_EXECUTION
    }
}
fn seh_install() {
    if SEH_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    unsafe { AddVectoredExceptionHandler(1, seh_veh); }
}
#[inline(always)]
fn seh_ptr() -> *mut u64 { SEH_T.with(|s| s.v.as_ptr() as *mut u64) }

// ★VEH 안에서 호출 — 원자연산만(alloc/lock/format 금지). RIP offset 유니크 집계.
fn wp_record(off: u64) {
    WP_HIT_TOTAL.fetch_add(1, Ordering::Relaxed);
    for k in 0..WP_SLOTS {
        let cur = WP_HITS_OFF[k].load(Ordering::Relaxed);
        if cur == off { WP_HITS_CNT[k].fetch_add(1, Ordering::Relaxed); return; }
        if cur == 0 && WP_HITS_OFF[k].compare_exchange(0, off, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
            WP_HITS_CNT[k].fetch_add(1, Ordering::Relaxed);
            WP_UNIQUE.fetch_add(1, Ordering::Relaxed);
            return;
        }
    }
    // 슬롯 만석 = 유니크 상한 도달(밖에서 자동해제 판단)
}

// ★전 스레드에 DR0~DR3 데이터 워치포인트 설정/해제(메인스레드서 호출). enable=false면 클리어.
//   WP_TARGETS[0..4] 의 non-zero 주소를 Dr0~Dr3 에 각각. 반환 = DR 설정 성공 스레드 수.
//   DR7 per-reg: Li=bit(2i) · RWi=bits(16+4i)=11(read/write) · LENi=bits(18+4i)=00(1byte).
unsafe fn wp_apply_all(enable: bool) -> u64 {
    const TH32CS_SNAPTHREAD: u32 = 0x4;
    const THREAD_ACCESS: u32 = 0x0002 | 0x0008 | 0x0010; // SUSPEND_RESUME|GET_CONTEXT|SET_CONTEXT
    // Dr0~Dr3 컨텍스트 오프셋
    const DR_OFF: [usize; 4] = [0x48, 0x50, 0x58, 0x60];
    let mut addrs = [0u64; 4];
    let mut dr7 = 0u64;
    if enable {
        for i in 0..WP_NT {
            let a = WP_TARGETS[i].load(Ordering::Relaxed);
            addrs[i] = a;
            if a != 0 { dr7 |= (1u64 << (i * 2)) | (0b11u64 << (16 + i * 4)); } // Li + RWi=11 + LENi=00
        }
    }
    if enable && dr7 == 0 { return 0; } // 설정할 타깃 없음
    let snap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
    if snap == -1 { return 0; }
    let cur_pid = GetCurrentProcessId();
    let cur_tid = GetCurrentThreadId();
    let mut te = ThreadEntry32::default();
    te.dw_size = core::mem::size_of::<ThreadEntry32>() as u32;
    let mut n = 0u64;
    if Thread32First(snap, &mut te) != 0 {
        loop {
            if te.th32_owner_pid == cur_pid && te.th32_thread_id != cur_tid {
                let h = OpenThread(THREAD_ACCESS, 0, te.th32_thread_id);
                if h != 0 && h != -1 {
                    SuspendThread(h);
                    let mut ctx = CtxBuf::new();
                    let cp = ctx.0.as_mut_ptr();
                    *((cp.add(0x30)) as *mut u32) = CONTEXT_DEBUG_REGS;
                    if GetThreadContext(h, cp) != 0 {
                        if enable {
                            for i in 0..4 { *((cp.add(DR_OFF[i])) as *mut u64) = addrs[i]; }
                            *((cp.add(0x70)) as *mut u64) = dr7; // Dr7
                        } else {
                            for i in 0..4 { *((cp.add(DR_OFF[i])) as *mut u64) = 0; }
                            *((cp.add(0x68)) as *mut u64) = 0; // Dr6
                            *((cp.add(0x70)) as *mut u64) = 0; // Dr7
                        }
                        *((cp.add(0x30)) as *mut u32) = CONTEXT_DEBUG_REGS;
                        if SetThreadContext(h, cp) != 0 { n += 1; }
                    }
                    ResumeThread(h);
                    CloseHandle(h);
                }
            }
            if Thread32Next(snap, &mut te) == 0 { break; }
        }
    }
    CloseHandle(snap);
    n
}
fn wp_target_count() -> u64 { WP_TARGETS.iter().filter(|t| t.load(Ordering::Relaxed) != 0).count() as u64 }

// 워치포인트 유지관리(메인스레드 post_update). 무장/재적용/자동해제.
fn wp_maintain() {
    if !CFG_WP.load(Ordering::Relaxed) {
        // cfg off인데 무장돼 있으면 해제
        if WP_ARMED.swap(false, Ordering::Relaxed) {
            unsafe { wp_apply_all(false); }
            WP_DISARMED.store(2, Ordering::Relaxed);
        }
        return;
    }
    // 자동해제 조건(상한): 부하·행 방지 — 목적은 "누가 읽나" 열거지 상시 감시 아님.
    if WP_ARMED.load(Ordering::Relaxed) {
        let disarm = WP_HIT_TOTAL.load(Ordering::Relaxed) >= WP_HIT_CAP
                  || WP_UNIQUE.load(Ordering::Relaxed) >= WP_UNIQUE_CAP;
        if disarm {
            unsafe { wp_apply_all(false); }
            WP_ARMED.store(false, Ordering::Relaxed);
            WP_DISARMED.store(1, Ordering::Relaxed);
            return;
        }
        // 주기적 재적용(rayon 신규 스레드 포섭 + cap_state가 타깃 더 잡았으면 반영) — 60프레임마다.
        static REAPPLY: AtomicU64 = AtomicU64::new(0);
        if REAPPLY.fetch_add(1, Ordering::Relaxed) % 60 == 0 {
            let n = unsafe { wp_apply_all(true) };
            WP_THREADS.store(n, Ordering::Relaxed); WP_APPLY_N.fetch_add(1, Ordering::Relaxed);
        }
        return;
    }
    // 미무장: WP_TARGETS(cap_state가 잡음)에 하나라도 있으면 무장.
    if wp_target_count() == 0 { return; } // 아직 유효 U 못 잡음
    let n = unsafe { wp_apply_all(true) };
    WP_THREADS.store(n, Ordering::Relaxed);
    WP_APPLY_N.fetch_add(1, Ordering::Relaxed);
    if n > 0 { WP_ARMED.store(true, Ordering::Relaxed); WP_DISARMED.store(0, Ordering::Relaxed); }
    else { WP_DISARMED.store(3, Ordering::Relaxed); } // 설정 실패
}

#[inline(never)]
unsafe fn safe_copy(dst: *mut u8, src: *const u8, len: usize) -> bool {
    if !SEH_INSTALLED.load(Ordering::Relaxed) { return false; }
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
unsafe fn safe_write_u64(addr: usize, val: u64) -> bool {
    let b = val.to_le_bytes();
    safe_copy(addr as *mut u8, b.as_ptr(), 8)
}

// ===========================================================================
//  경로 / 로깅
// ===========================================================================
fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }
fn exe_path() -> Option<PathBuf> {
    let mut buf = [0u16; 1024];
    let n = unsafe { GetModuleFileNameW(GetModuleHandleW(core::ptr::null()), buf.as_mut_ptr(), buf.len() as u32) };
    if n == 0 { return None; }
    Some(PathBuf::from(String::from_utf16_lossy(&buf[..n as usize])))
}
fn game_root() -> Option<PathBuf> { exe_path()?.parent().map(|p| p.to_path_buf()) }
fn mod_dir() -> Option<PathBuf> { Some(game_root()?.join("mods").join(MOD_ID)) }

fn log_line(s: &str) {
    let mut g = LOG_BUF.lock().unwrap_or_else(|e| e.into_inner());
    if g.len() > 200_000 { return; } // 비대 방지
    g.push_str(s); g.push('\n');
}

// ===========================================================================
//  cfg 로드
// ===========================================================================
fn load_cfg() {
    // side/slot/judgement/order 파싱. '#' 주석 제거. 못 읽으면 전부 -1(무개입) 유지.
    let mut side = 0u64; let mut slot = 0u64; let mut judge = -1i64; let mut order = -1i64;
    let mut hist = true;
    let mut wp = false; // 워치포인트 기본 OFF(고위험 — 명시적으로 켤 때만)
    let mut diag = String::new();
    match mod_dir() {
        None => diag.push_str("mod_dir()=None → 기본값(전부 무개입)\n"),
        Some(d) => {
            let p = d.join("stat_exp.cfg");
            diag.push_str(&format!("cfg 경로 = {} (존재={})\n", p.display(), p.exists()));
            match fs::read_to_string(&p) {
                Err(e) => diag.push_str(&format!("cfg 읽기 실패({}) → 무개입 유지\n", e)),
                Ok(s) => {
                    for raw in s.lines() {
                        let line = match raw.find('#') { Some(i) => &raw[..i], None => raw };
                        let line = line.trim();
                        if line.is_empty() { continue; }
                        let Some(eq) = line.find('=') else { continue; };
                        let key = line[..eq].trim().to_ascii_lowercase();
                        let val = line[eq + 1..].trim();
                        match key.as_str() {
                            "side" => side = match val.to_ascii_lowercase().as_str() {
                                "blue" => 0, "red" => 1, "all" => 2, _ => side,
                            },
                            "slot" => slot = if val.eq_ignore_ascii_case("all") { 255 }
                                              else { val.parse::<u64>().ok().filter(|v| *v < 5).unwrap_or(slot) },
                            "judgement" | "judge" => judge = val.parse::<i64>().unwrap_or(judge),
                            "order" => order = val.parse::<i64>().unwrap_or(order),
                            "histogram" | "hist" => hist = !(val == "0" || val.eq_ignore_ascii_case("off") || val.eq_ignore_ascii_case("false")),
                            "watchpoint" | "wp" => wp = val == "1" || val.eq_ignore_ascii_case("on") || val.eq_ignore_ascii_case("true"),
                            _ => {}
                        }
                    }
                    diag.push_str("cfg 읽기 OK\n");
                }
            }
        }
    }
    CFG_SIDE.store(side, Ordering::Relaxed);
    CFG_SLOT.store(slot, Ordering::Relaxed);
    CFG_JUDGE.store(judge, Ordering::Relaxed);
    CFG_ORDER.store(order, Ordering::Relaxed);
    CFG_HIST.store(hist, Ordering::Relaxed);
    CFG_WP.store(wp, Ordering::Relaxed);
    CFG_LOADED.store(true, Ordering::Relaxed);
    let side_s = ["blue", "red", "all"].get(side as usize).unwrap_or(&"?");
    let slot_s = if slot == 255 { "all".to_string() } else { slot.to_string() };
    diag.push_str(&format!("★적용: side={} slot={} judgement={} order={} histogram={} watchpoint={}\n", side_s, slot_s, judge, order, hist, wp));
    diag.push_str(&format!("CHAIN_CONFIRMED={} (false 면 로그 관측만·write 안 함)\n", CHAIN_CONFIRMED));
    if let Some(d) = mod_dir() { let _ = fs::create_dir_all(&d); let _ = fs::write(d.join("stat_exp_cfg.txt"), &diag); }
}

fn poke_active() -> bool {
    // 개입 대상이 하나라도 있으면 true(로그 목적). 실제 write 는 추가로 CHAIN_CONFIRMED 필요.
    CFG_JUDGE.load(Ordering::Relaxed) >= 0 || CFG_ORDER.load(Ordering::Relaxed) >= 0
}

// ===========================================================================
//  런처 체인 후킹 (item_tactics install_launcher_hook / install_detour_generic 이식)
// ===========================================================================
// 진입부가 이미 외부 훅(48 b8 <tgt> ff e0)이면 그 12B 를 내 트램폴린에 담아 원본 대신 tgt 로 점프(체인).
unsafe fn install_detour_generic(rva: usize, orig_len: usize, cap_fn: usize, prologue: &[u8]) -> Result<usize, &'static str> {
    let base = GetModuleHandleW(core::ptr::null()) as usize;
    if base == 0 { return Err("module 0"); }
    let fn_addr = base + rva;
    if !readable(fn_addr, orig_len + 4) { return Err("unreadable"); }
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    let foreign_tgt: usize = if cur[0] == 0x48 && cur[1] == 0xb8 && cur[10] == 0xff && cur[11] == 0xe0 {
        usize::from_le_bytes(cur[2..10].try_into().unwrap())
    } else { 0 };
    let chained = foreign_tgt >= 0x10000;
    if !chained {
        for i in 0..prologue.len() { if *((fn_addr + i) as *const u8) != prologue[i] { return Err("prologue mismatch"); } }
    }
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    // push r12 rsi rdi rbx r11 r10 r9 r8 rdx rcx  (rcx 마지막=saved+0; r10/r9 원본 보존)
    s.extend_from_slice(&[0x41, 0x54, 0x56, 0x57, 0x53, 0x41, 0x53, 0x41, 0x52, 0x41, 0x51, 0x41, 0x50, 0x52, 0x51]);
    s.extend_from_slice(&[0x48, 0x89, 0xe1]);             // mov rcx, rsp (saved=arg1)
    s.extend_from_slice(&[0x48, 0x89, 0xe3]);             // mov rbx, rsp (정렬복원 홀더)
    s.extend_from_slice(&[0x48, 0x83, 0xe4, 0xf0]);       // and rsp, -16
    s.extend_from_slice(&[0x48, 0x83, 0xec, 0x20]);       // sub rsp, 0x20 (shadow)
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes()); // movabs rax, cap_fn
    s.extend_from_slice(&[0xff, 0xd0]);                   // call rax
    s.extend_from_slice(&[0x48, 0x89, 0xdc]);             // mov rsp, rbx
    // pop rcx rdx r8 r9 r10 r11 rbx rdi rsi r12
    s.extend_from_slice(&[0x59, 0x5a, 0x41, 0x58, 0x41, 0x59, 0x41, 0x5a, 0x41, 0x5b, 0x5b, 0x5f, 0x5e, 0x41, 0x5c]);
    if chained {
        s.extend_from_slice(&cur); // = 48 b8 <foreign_tgt> ff e0
    } else {
        let mut orig = vec![0u8; orig_len];
        core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
        s.extend_from_slice(&orig);
        s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes()); // movabs rax, ret_addr
        s.extend_from_slice(&[0xff, 0xe0]);               // jmp rax
    }
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

// ⚠최소 디투어: 런처는 대형 chkstk 프레임 + 배경 다수경기 발화 → format!/fs/락/catch_unwind 금지.
//   본문 = raw read(safe_read) + 원자연산만(패닉 원천 없음).
unsafe extern "C" fn cap_launcher(saved: *mut u64, _e: usize) -> u64 {
    if saved.is_null() { return 0; }
    let game_arg = *saved;        // rcx = arg1 = Game
    let retaddr = *saved.add(10); // 스텁 push 10개 위 = 콜사이트 retaddr
    let base = exe_base_addr() as u64;
    if base == 0 || retaddr < base { return 0; }
    let rva = retaddr - base;
    LAUNCH_N.fetch_add(1, Ordering::Relaxed);
    // 고유 콜러 rva 수집(리플레이 retaddr 등 인게임 식별용)
    for k in 0..24 {
        let s = LAUNCH_RVAS[k].load(Ordering::Relaxed);
        if s == rva { break; }
        if s == 0 && LAUNCH_RVAS[k].compare_exchange(0, rva, Ordering::Relaxed, Ordering::Relaxed).is_ok() { break; }
    }
    // ★화면 경기(관전/내경기/조합테스트/다시보기) retaddr 만 Game 캡처 = 배경 sim 오염 배제.
    //   0.5.5 값(item_tactics 정본): 관전 0x763329 · 내경기 0x76829b · 조테본경기 0x1aed292 · 조테기록 0x1aa88ce.
    //   다시보기(리플레이) 0x1da7d54 = elemental_serpen 0.5.5 RET_C 교차확인 + 인게임 로그 실측(2026-08-13).
    //   ⛔나머지 후보(0x1c777d8 대회워커·0x2228db·0x1a7191c 배경sim·heap ra) 추가 금지 = 비가시 배경 sim 오염.
    let is_screen = rva == 0x768a99 || rva == 0x76829b || rva == 0x1aed292 || rva == 0x1aa88ce
        || rva == 0x1da7d54;
    if is_screen && ptr_ok(game_arg as usize) {
        LIVE_GAME.store(game_arg, Ordering::Relaxed);
        LAUNCH_RENDER_N.fetch_add(1, Ordering::Relaxed);
        LAUNCH_RENDER_RA.store(rva, Ordering::Relaxed);
    }
    0
}

fn install_launcher_hook() {
    if CLAUNCH_INSTALLED.load(Ordering::Relaxed) == 1 {
        // 60프레임 주기 self-heal 재검증(매프레임 재체인 금지 — 상호 사이클 방지)
        static TICK: AtomicU64 = AtomicU64::new(0);
        if TICK.fetch_add(1, Ordering::Relaxed) % 60 != 0 { return; }
    }
    let base = exe_base_addr();
    if base == 0 { return; }
    let fn_addr = base + CL_LAUNCHER_RVA;
    let Some(w0) = (unsafe { safe_read_u64(fn_addr) }) else { return; };
    let b0 = (w0 & 0xff) as u8;
    let b1 = ((w0 >> 8) & 0xff) as u8;
    let cur_tgt: usize = if b0 == 0x48 && b1 == 0xb8 {
        match unsafe { safe_read_u64(fn_addr + 2) } { Some(t) => t as usize, None => return }
    } else { 0 };
    let our = CLAUNCH_STUB.load(Ordering::Relaxed) as usize;
    if our != 0 && cur_tgt == our { CLAUNCH_INSTALLED.store(1, Ordering::Relaxed); return; } // 진입부=우리 스텁 → 정상
    let is_foreign = b0 == 0x48 && cur_tgt >= 0x10000 && cur_tgt != our;
    let waited = LAUNCH_WAIT.fetch_add(1, Ordering::Relaxed);
    // 원본프롤로그 & 대기중 → 외부훅(다른 모드) 설치를 기다림(늦게 설치 = 늦게 체인).
    if !is_foreign && b0 != 0x48 && waited < 240 { return; }
    let r = unsafe { install_detour_generic(CL_LAUNCHER_RVA, CL_LAUNCHER_ORIG_LEN, cap_launcher as usize, &CL_LAUNCHER_PROLOGUE) };
    match r {
        Ok(stub) => { CLAUNCH_STUB.store(stub as u64, Ordering::Relaxed); CLAUNCH_INSTALLED.store(1, Ordering::Relaxed); }
        Err(_) => { CLAUNCH_INSTALLED.store(2, Ordering::Relaxed); }
    }
}

// ⚠최소 디투어(멀티스레드 동시호출) — rdx=Game 을 원자 테이블에 기록만. format!/fs/락/catch_unwind 금지.
unsafe extern "C" fn cap_run_tick(saved: *mut u64, _e: usize) -> u64 {
    if saved.is_null() { return 0; }
    let game = *saved.add(1); // saved[1] = rdx = arg2 = Game
    if game < 0x10000 || game >= 0x0000_8000_0000_0000 { return 0; }
    RT_N.fetch_add(1, Ordering::Relaxed);
    RT_GAME.store(game, Ordering::Relaxed);
    // 8슬롯 테이블: 같은 Game 이면 카운트++, 없으면 빈 슬롯 점유(전부 원자연산 = 스레드안전).
    for k in 0..RT_TAB_N {
        let g = RT_TAB_G[k].load(Ordering::Relaxed);
        if g == game { RT_TAB_C[k].fetch_add(1, Ordering::Relaxed); return 0; }
        if g == 0 && RT_TAB_G[k].compare_exchange(0, game, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
            RT_TAB_C[k].fetch_add(1, Ordering::Relaxed); return 0;
        }
    }
    0
}

fn install_run_tick_hook() {
    if RTHOOK_INSTALLED.load(Ordering::Relaxed) == 1 {
        static TICK: AtomicU64 = AtomicU64::new(0);
        if TICK.fetch_add(1, Ordering::Relaxed) % 60 != 0 { return; } // 60프레임 self-heal
    }
    let base = exe_base_addr();
    if base == 0 { return; }
    let fn_addr = base + RUN_TICK_RVA;
    let Some(w0) = (unsafe { safe_read_u64(fn_addr) }) else { return; };
    let b0 = (w0 & 0xff) as u8;
    let b1 = ((w0 >> 8) & 0xff) as u8;
    let cur_tgt: usize = if b0 == 0x48 && b1 == 0xb8 {
        match unsafe { safe_read_u64(fn_addr + 2) } { Some(t) => t as usize, None => return }
    } else { 0 };
    let our = RTHOOK_STUB.load(Ordering::Relaxed) as usize;
    if our != 0 && cur_tgt == our { RTHOOK_INSTALLED.store(1, Ordering::Relaxed); return; }
    let is_foreign = b0 == 0x48 && cur_tgt >= 0x10000 && cur_tgt != our;
    let waited = RT_WAIT.fetch_add(1, Ordering::Relaxed);
    if !is_foreign && b0 != 0x48 && waited < 240 { return; } // 외부훅(타 모드) 설치 대기 후 체인
    let r = unsafe { install_detour_generic(RUN_TICK_RVA, RUN_TICK_ORIG_LEN, cap_run_tick as usize, &RUN_TICK_PROLOGUE) };
    match r {
        Ok(stub) => { RTHOOK_STUB.store(stub as u64, Ordering::Relaxed); RTHOOK_INSTALLED.store(1, Ordering::Relaxed); }
        Err(_) => { RTHOOK_INSTALLED.store(2, Ordering::Relaxed); }
    }
}

// ═══ 액션상태 집계 훅 (FUN_140eafed0, RCX=U·R8=athlete) — U+0xA47 액션상태 side별 히스토그램 ═══
// ⚠hot·멀티스레드(rayon) — detour 본문 극소화: 3 read + 원자 increment + 원본호출. write 절대 없음.
//   catch_unwind + 명시 가드(side≤1·st<16)로 패닉(인덱스 OOB) 원천 차단(트램폴린 asm로 unwind = UB 방지).
unsafe extern "C" fn cap_state(saved: *mut u64, _e: usize) -> u64 {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved.is_null() { return; }
        let u = *saved as usize;          // rcx = U(유닛객체)
        let athlete = *saved.add(2) as usize; // r8 = athlete(참가자 record)
        if !ptr_ok(u) || !ptr_ok(athlete) { return; }
        SHOOK_N.fetch_add(1, Ordering::Relaxed);
        // ── 히스토그램(cfg histogram=1) ──
        if CFG_HIST.load(Ordering::Relaxed) {
            if let Some(side) = safe_read_u64(athlete.wrapping_add(O_SIDE)) {
                if side <= 1 {
                    if let Some(sw) = safe_read_u64(u.wrapping_add(O_U_STATE)) {
                        let st = (sw & 0xff) as usize; let si = side as usize;
                        if st < 16 { HIST_STATE[si][st].fetch_add(1, Ordering::Relaxed); }
                        else { HIST_OTHER[si].fetch_add(1, Ordering::Relaxed); }
                    }
                } else { SHOOK_SIDE_OOB.fetch_add(1, Ordering::Relaxed); }
            }
        }
        // ── 워치포인트 타깃 캡처(cfg watchpoint=1) — 서로 다른 U 4개의 len 필드(U+0xD08) 주소 ──
        //   ★큐가 transient(qlen 순간 0)라 qlen>0 요구는 못 잡음 → qlen 무관, 유효 U(큐 포인터 존재)면 채택.
        //   미래의 push(len++ write)·consume(len read)를 잡는 게 목적. len 필드는 U 상주=안정 주소.
        if CFG_WP.load(Ordering::Relaxed) {
            let addr = (u + O_Q_LEN) as u64;
            // 이미 등록됐거나(같은 addr) 빈 슬롯 없으면 skip. qptr 존재만 요구(큐 필드 유효성).
            if WP_TARGETS.iter().all(|t| t.load(Ordering::Relaxed) != addr) {
                if let Some(qptr) = safe_read_u64(u.wrapping_add(O_Q_PTR)) {
                    if ptr_ok(qptr as usize) {
                        for t in WP_TARGETS.iter() {
                            if t.compare_exchange(0, addr, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
                                if WP_U.load(Ordering::Relaxed) == 0 { WP_U.store(u as u64, Ordering::Relaxed); }
                                break;
                            }
                        }
                    }
                }
            }
        }
    }));
    0
}

fn install_state_hook() {
    // 히스토그램 또는 워치포인트 중 하나라도 켜져 있으면 설치(cap_state가 U 획득에 필요). 둘 다 OFF = 미설치.
    if !CFG_HIST.load(Ordering::Relaxed) && !CFG_WP.load(Ordering::Relaxed) { return; }
    if SHOOK_INSTALLED.load(Ordering::Relaxed) == 1 {
        static TICK: AtomicU64 = AtomicU64::new(0);
        if TICK.fetch_add(1, Ordering::Relaxed) % 60 != 0 { return; }
    }
    let base = exe_base_addr();
    if base == 0 { return; }
    let fn_addr = base + STATE_HOOK_RVA;
    let Some(w0) = (unsafe { safe_read_u64(fn_addr) }) else { return; };
    let b0 = (w0 & 0xff) as u8;
    let b1 = ((w0 >> 8) & 0xff) as u8;
    let cur_tgt: usize = if b0 == 0x48 && b1 == 0xb8 {
        match unsafe { safe_read_u64(fn_addr + 2) } { Some(t) => t as usize, None => return }
    } else { 0 };
    let our = SHOOK_STUB.load(Ordering::Relaxed) as usize;
    if our != 0 && cur_tgt == our { SHOOK_INSTALLED.store(1, Ordering::Relaxed); return; }
    let is_foreign = b0 == 0x48 && cur_tgt >= 0x10000 && cur_tgt != our;
    let waited = SHOOK_WAIT.fetch_add(1, Ordering::Relaxed);
    if !is_foreign && b0 != 0x48 && waited < 240 { return; }
    let r = unsafe { install_detour_generic(STATE_HOOK_RVA, STATE_HOOK_ORIG_LEN, cap_state as usize, &STATE_HOOK_PROLOGUE) };
    match r {
        Ok(stub) => { SHOOK_STUB.store(stub as u64, Ordering::Relaxed); SHOOK_INSTALLED.store(1, Ordering::Relaxed); }
        Err(_) => { SHOOK_INSTALLED.store(2, Ordering::Relaxed); }
    }
}

// ===========================================================================
//  poke 적용 (post_update — 메인스레드)
// ===========================================================================
// unit-slot 리졸버 경유로 매치 참가자 레코드를 순회한다(글로벌 athlete 풀 직접순회 회피).
//   진영은 team 필드(record+0x930)로 구분: 서로 다른 값 두 개 중 작은쪽=blue, 큰쪽=red.
// 레코드 하나의 관심 오프셋들을 write 무관하게 읽어 로그(오프셋 정합 최종 판정용). gate 로 최초 1회.
unsafe fn raw_sample_record(gate: &AtomicBool, rec: usize, label: &str) {
    if gate.swap(true, Ordering::Relaxed) { return; } // 최초 1회만
    let mut s = format!("[{}ms] RAW 샘플 {} @rec={:#x}\n", now_ms(), label, rec);
    // AthleteStat 임베드 base(+0x1e0) 및 판단력/오더 인접값 + side/flag
    let offs: [(usize, &str); 8] = [
        (0x1e0, "stat_embed_base"), (0x210, "+0x210"), (0x218, "판단력(+0x218)"), (0x220, "오더(+0x220)"),
        (0x228, "+0x228"), (0x450, "구판단력후보(+0x450)"), (0x930, "side/team(+0x930)"), (0x9c8, "flag(+0x9c8)"),
    ];
    for (o, nm) in offs.iter() {
        match safe_read_u64(rec.wrapping_add(*o)) {
            Some(v) => s.push_str(&format!("   {:<20} {:#018x}  (dec {})\n", nm, v, v as i64)),
            None => s.push_str(&format!("   {:<20} <읽기실패>\n", nm)),
        }
    }
    let mut g = RAW_SAMPLE.lock().unwrap_or_else(|e| e.into_inner());
    g.push_str(&s);
    RAW_SAMPLE_N.fetch_add(1, Ordering::Relaxed);
}
static RAW_SAMPLED_UNIT: AtomicBool = AtomicBool::new(false); // unit-slot 첫 유효 참가자 샘플 게이트

// 후보 World 의 유효 매치 참가자 수(검증 전용, poke 없음). run_one_tick Game 다수 중 선별용.
unsafe fn world_valid_participants(world: usize) -> u64 {
    let Some(ath_base) = safe_read_u64(world.wrapping_add(O_ATH_ARR)) else { return 0; };
    let ath_base = ath_base as usize;
    if !ptr_ok(ath_base) { return 0; }
    let ath_count = safe_read_u64(world.wrapping_add(O_ATH_COUNT)).unwrap_or(0);
    if ath_count == 0 || ath_count > 100_000 { return 0; }
    if ath_count > MAX_ATH_COUNT.load(Ordering::Relaxed) { MAX_ATH_COUNT.store(ath_count, Ordering::Relaxed); }
    let Some(unit_base) = safe_read_u64(world.wrapping_add(O_UNIT_ARR)) else { return 0; };
    let unit_base = unit_base as usize;
    if !ptr_ok(unit_base) { return 0; }
    let unit_count = safe_read_u64(world.wrapping_add(O_UNIT_COUNT)).unwrap_or(0);
    if unit_count == 0 || unit_count > 64 { return 0; }
    if unit_count > MAX_UNIT_COUNT.load(Ordering::Relaxed) { MAX_UNIT_COUNT.store(unit_count, Ordering::Relaxed); }
    let mut n = 0u64;
    for s in 0..unit_count as usize {
        let e = unit_base.wrapping_add(s.wrapping_mul(UNIT_STRIDE));
        if !ptr_ok(e) { continue; }
        let Some(gen) = safe_read_u64(e) else { continue; };
        if gen != 1 { continue; }
        let Some(aidx) = safe_read_u64(e.wrapping_add(8)) else { continue; };
        if aidx >= ath_count { continue; }
        let rec = ath_base.wrapping_add((aidx as usize).wrapping_mul(ARR_STRIDE));
        if !ptr_ok(rec) { continue; }
        let Some(flag) = safe_read_u64(rec.wrapping_add(O_REC_FLAG)) else { continue; };
        if flag != 1 { continue; }
        n += 1;
    }
    n
}

// ★freeze 방지 핵심: 캐시된 Game 이 유효하면 매 프레임 World 프로브 1회로 끝낸다.
//   전투 전환 시 stale 후보 다수를 매 프레임 재프로브 → VEH fault 폭풍 → 메인스레드 스톨(=freeze)이 원인.
//   캐시가 깨지면 재스캔하되 스로틀(전투 전환 몇 프레임 동안 폭풍 방지).
static RESCAN_TICK: AtomicU64 = AtomicU64::new(0);
fn apply_poke() {
    PK_FRAMES.fetch_add(1, Ordering::Relaxed);
    unsafe {
        // 1) 캐시 Game 우선 — 유효하면 World 프로브 1회로 끝(fault 폭풍 방지).
        let cached = CHOSEN_GAME.load(Ordering::Relaxed) as usize;
        if cached != 0 && ptr_ok(cached) {
            if let Some(w) = safe_read_u64(cached + O_GAME_WORLD) {
                let w = w as usize;
                if ptr_ok(w) && world_valid_participants(w) > 0 {
                    CHOSEN_WORLD.store(w as u64, Ordering::Relaxed);
                    CHOSEN_SRC.store(1, Ordering::Relaxed);
                    PK_WORLD_OK.fetch_add(1, Ordering::Relaxed);
                    poke_world(w);
                    return;
                }
            }
            // 캐시 깨짐 → 재스캔을 스로틀(전투 전환 중 매프레임 폭풍 금지).
            if RESCAN_TICK.fetch_add(1, Ordering::Relaxed) % 15 != 0 { return; }
        }
        // 2) 후보 재스캔(캐시 없음/깨짐). fault 상한으로 폭풍 차단.
        let mut cands: Vec<(u64, u64)> = Vec::new(); // (game, src)
        for k in 0..RT_TAB_N {
            let g = RT_TAB_G[k].load(Ordering::Relaxed);
            if g != 0 && !cands.iter().any(|c| c.0 == g) { cands.push((g, 1)); }
        }
        { let g = RT_GAME.load(Ordering::Relaxed); if g != 0 && !cands.iter().any(|c| c.0 == g) { cands.push((g, 1)); } }
        { let g = LIVE_GAME.load(Ordering::Relaxed); if g != 0 && !cands.iter().any(|c| c.0 == g) { cands.push((g, 2)); } }
        if cands.is_empty() { return; }

        let mut seen = 0u64; let mut valid = 0u64;
        let mut best: Option<(u64, usize, u64)> = None; // (game, world, src)·최다 유효참가자
        let mut best_vc = 0u64;
        let mut clog = String::new();
        let f0 = *seh_ptr().add(7); // fault 스냅샷 — 스캔 중 폭주하면 중단
        for &(game, src) in cands.iter() {
            if (*seh_ptr().add(7)).wrapping_sub(f0) > 2000 { clog.push_str("  ⚠fault 상한 초과 → 스캔 중단\n"); break; }
            let g = game as usize;
            if !ptr_ok(g) { continue; }
            seen += 1;
            let Some(w) = safe_read_u64(g + O_GAME_WORLD) else {
                clog.push_str(&format!("  game={:#x} src{} World읽기실패\n", game, src)); continue; };
            let w = w as usize;
            if !ptr_ok(w) { clog.push_str(&format!("  game={:#x} src{} World무효\n", game, src)); continue; }
            let vc = world_valid_participants(w);
            clog.push_str(&format!("  game={:#x} src{} world={:#x} 유효참가자={}\n", game, src, w, vc));
            if vc == 0 { continue; }
            valid += 1;
            let better = match best { None => true, Some((_, _, bsrc)) => src < bsrc || (src == bsrc && vc > best_vc) };
            if better { best = Some((game, w, src)); best_vc = vc; }
        }
        CAND_SEEN.store(seen, Ordering::Relaxed);
        CAND_VALID.store(valid, Ordering::Relaxed);
        *CAND_LOG.lock().unwrap_or_else(|e| e.into_inner()) = clog;
        let Some((game, world, src)) = best else {
            CHOSEN_GAME.store(0, Ordering::Relaxed);
            CHOSEN_WORLD.store(0, Ordering::Relaxed); CHOSEN_SRC.store(0, Ordering::Relaxed); return; };
        CHOSEN_GAME.store(game, Ordering::Relaxed); // ★캐시 — 다음 프레임부터 이 Game 만 프로브
        CHOSEN_WORLD.store(world as u64, Ordering::Relaxed);
        CHOSEN_SRC.store(src, Ordering::Relaxed);
        PK_WORLD_OK.fetch_add(1, Ordering::Relaxed);
        poke_world(world);
    }
}

// 선택된 World 에 대해 진단 갱신 + RAW 샘플 + 참가자 순회 poke.
unsafe fn poke_world(world: usize) {
    DBG_WORLD.store(world as u64, Ordering::Relaxed);
    let ath_base = match safe_read_u64(world.wrapping_add(O_ATH_ARR)) {
        Some(v) if ptr_ok(v as usize) => v as usize, _ => { DBG_ATH_BASE_OK.store(2, Ordering::Relaxed); return; } };
    DBG_ATH_BASE_OK.store(1, Ordering::Relaxed);
    let ath_count = safe_read_u64(world.wrapping_add(O_ATH_COUNT)).unwrap_or(0);
    PK_COUNT.store(ath_count, Ordering::Relaxed);
    if ath_count > 0 && ath_count <= 100_000 { FRAMES_WITH_ATH.fetch_add(1, Ordering::Relaxed); }
    let unit_base = match safe_read_u64(world.wrapping_add(O_UNIT_ARR)) {
        Some(v) if ptr_ok(v as usize) => v as usize, _ => { DBG_UNIT_BASE_OK.store(2, Ordering::Relaxed); return; } };
    DBG_UNIT_BASE_OK.store(1, Ordering::Relaxed);
    let unit_count = safe_read_u64(world.wrapping_add(O_UNIT_COUNT)).unwrap_or(0);
    DBG_UNIT_COUNT.store(unit_count, Ordering::Relaxed);
    if !RAW_SAMPLED.load(Ordering::Relaxed) && ath_count > 0 && ath_count <= 100_000 {
        raw_sample_record(&RAW_SAMPLED, ath_base, "athlete[0](직접배열)");
    }
    if ath_count == 0 || ath_count > 100_000 { return; }
    PK_ARR_OK.fetch_add(1, Ordering::Relaxed);
    if unit_count == 0 || unit_count > 64 { return; }

    // ── 유효 참가자 수집: (record, side_val) ──
    let mut parts: Vec<(usize, u64)> = Vec::with_capacity(16);
    for s in 0..unit_count as usize {
        let e = unit_base.wrapping_add(s.wrapping_mul(UNIT_STRIDE));
        if !ptr_ok(e) { continue; }
        let Some(gen) = safe_read_u64(e) else { continue; };
        if gen != 1 { continue; }
        let Some(aidx) = safe_read_u64(e.wrapping_add(8)) else { continue; };
        if aidx >= ath_count { continue; }
        let rec = ath_base.wrapping_add((aidx as usize).wrapping_mul(ARR_STRIDE));
        if !ptr_ok(rec) { continue; }
        let Some(flag) = safe_read_u64(rec.wrapping_add(O_REC_FLAG)) else { continue; };
        if flag != 1 { continue; }
        let side_val = safe_read_u64(rec.wrapping_add(O_SIDE)).unwrap_or(u64::MAX);
        if !RAW_SAMPLED_UNIT.load(Ordering::Relaxed) {
            raw_sample_record(&RAW_SAMPLED_UNIT, rec, "unit-slot 첫 유효참가자");
        }
        parts.push((rec, side_val));
    }
    if parts.is_empty() { return; }
    PK_REC_OK.store(parts.len() as u64, Ordering::Relaxed);

    // ── 진영 결정: 서로 다른 side 값 → 작은쪽=blue, 큰쪽=red ──
    let mut sides: Vec<u64> = parts.iter().map(|p| p.1).filter(|v| *v != u64::MAX).collect();
    sides.sort_unstable(); sides.dedup();
    let blue_val = sides.first().copied();
    let red_val = sides.get(1).copied();
    SIDE_A.store(blue_val.unwrap_or(u64::MAX), Ordering::Relaxed);
    SIDE_B.store(red_val.unwrap_or(u64::MAX), Ordering::Relaxed);
    // ※액션상태 히스토그램은 이제 behavior 훅(cap_state, U+0xA47)에서 집계 — 여기 참가자 +0x517 읽기는 제거(항상 0이었음).
    let cfg_side = CFG_SIDE.load(Ordering::Relaxed);
    let want: Vec<u64> = match cfg_side {
        0 => sides.first().copied().into_iter().collect(),
        1 => sides.get(1).copied().into_iter().collect(),
        _ => sides.clone(),
    };
    let cfg_slot = CFG_SLOT.load(Ordering::Relaxed);
    let judge = CFG_JUDGE.load(Ordering::Relaxed);
    let order = CFG_ORDER.load(Ordering::Relaxed);

    // ── side별 슬롯 인덱싱(수집 순서) 후 cfg_slot 매칭 → poke ──
    let (mut cnt_a, mut cnt_b) = (0usize, 0usize);
    let sa = SIDE_A.load(Ordering::Relaxed);
    for (gi, &(rec, side_val)) in parts.iter().enumerate() {
        let slot_in_side = if side_val == sa { let v = cnt_a; cnt_a += 1; v } else { let v = cnt_b; cnt_b += 1; v };
        if !want.contains(&side_val) { continue; }
        if cfg_slot != 255 && slot_in_side != cfg_slot as usize { continue; }
        if gi >= MAX_P { continue; }
        let Some(old_j) = safe_read_u64(rec.wrapping_add(O_JUDGE)) else { continue; };
        poke_field(gi, 0, rec.wrapping_add(O_JUDGE), old_j, judge, "판단력", side_val, slot_in_side);
        if let Some(old_o) = safe_read_u64(rec.wrapping_add(O_ORDER)) {
            poke_field(gi, 1, rec.wrapping_add(O_ORDER), old_o, order, "오더", side_val, slot_in_side);
        }
    }
}

// 필드 하나 poke. val<0 = 무개입. old 타당성 실패 = skip. old 변화 시에만 로그.
unsafe fn poke_field(idx: usize, field: usize, addr: usize, old: u64, val: i64, name: &str, side_val: u64, slot_in_side: usize) {
    if val < 0 { return; }
    let val = val as u64;
    let key = idx * 2 + field;
    // old 값 타당성 가드(오프셋 오식별로 포인터/쓰레기에 쓰는 사고 방지)
    if old > VAL_SANITY_MAX {
        PK_SKIP_SANITY.fetch_add(1, Ordering::Relaxed);
        let last = LAST_OLD[key].swap(old, Ordering::Relaxed);
        if last != old {
            log_line(&format!("[{}ms] side={:#x} slot{} {} old={:#x}=타당성초과(>{:#x}) → write SKIP(오프셋 의심)",
                now_ms(), side_val, slot_in_side, name, old, VAL_SANITY_MAX));
        }
        return;
    }
    // old 변화 시에만 로그(파일 비대 방지). 매 틱 게임이 원복해도 old 는 안정값이라 1회만 기록.
    let last = LAST_OLD[key].load(Ordering::Relaxed);
    if last != old {
        LAST_OLD[key].store(old, Ordering::Relaxed);
        log_line(&format!("[{}ms] side={:#x} slot{} {} {}→{} (write={})",
            now_ms(), side_val, slot_in_side, name, old, val, CHAIN_CONFIRMED));
    }
    if old == val { return; }        // 이미 목표값(게임이 아직 원복 안 함)
    if !CHAIN_CONFIRMED { return; }  // ★체인 미확정 = write 원천차단(관측만)
    if safe_write_u64(addr, val) {
        PK_WROTE.fetch_add(1, Ordering::Relaxed);
    }
}

// 진단 파일 flush (30프레임마다)
fn flush_status() {
    static F: AtomicU64 = AtomicU64::new(0);
    if F.fetch_add(1, Ordering::Relaxed) % 30 != 0 { return; }
    let ld = |a: &AtomicU64| a.load(Ordering::Relaxed);
    let mut s = String::new();
    s.push_str("=== tfm2_stat_exp — 판단력/오더 강제 덮어쓰기 상태 ===\n");
    s.push_str(&format!("생성(ms): {}\n\n", now_ms()));
    let side = ["blue", "red", "all"].get(CFG_SIDE.load(Ordering::Relaxed) as usize).unwrap_or(&"?");
    let slot = CFG_SLOT.load(Ordering::Relaxed);
    let slot_s = if slot == 255 { "all".to_string() } else { slot.to_string() };
    s.push_str(&format!("[cfg] side={} slot={} judgement={} order={}\n",
        side, slot_s, CFG_JUDGE.load(Ordering::Relaxed), CFG_ORDER.load(Ordering::Relaxed)));
    s.push_str(&format!("[안전] CHAIN_CONFIRMED={} (false=관측만·write 안 함) · poke_active={}\n\n",
        CHAIN_CONFIRMED, poke_active()));
    s.push_str("[런처/Game 캡처]\n");
    s.push_str(&format!("  발화={} 화면판정={} 화면RA={:#x} LIVE_GAME={:#x} 설치={}\n",
        ld(&LAUNCH_N), ld(&LAUNCH_RENDER_N), ld(&LAUNCH_RENDER_RA), ld(&LIVE_GAME), ld(&CLAUNCH_INSTALLED)));
    s.push_str("  고유 콜러 rva:");
    for k in 0..24 { let v = LAUNCH_RVAS[k].load(Ordering::Relaxed); if v == 0 { break; } s.push_str(&format!(" {:#x}", v)); }
    s.push_str("\n  (화면필터=관전0x763329·내경기0x76829b·조테0x1aed292/0x1aa88ce·다시보기0x1da7d54)\n\n");
    s.push_str("[run_one_tick 훅 (★현재 World 신뢰소스)]\n");
    let chsrc = ["없음", "run_tick", "launcher폴백"].get(CHOSEN_SRC.load(Ordering::Relaxed) as usize).unwrap_or(&"?");
    s.push_str(&format!("  발화={} 설치={} 최신Game={:#x} | 후보검사={} 유효후보={} 선택World={:#x} 선택소스={}\n",
        ld(&RT_N), ld(&RTHOOK_INSTALLED), ld(&RT_GAME), ld(&CAND_SEEN), ld(&CAND_VALID), ld(&CHOSEN_WORLD), chsrc));
    s.push_str(&format!("  ★캐시Game={:#x} (유효시 매프레임 World프로브 1회=freeze 방지) · 히스토그램={}\n",
        ld(&CHOSEN_GAME), if CFG_HIST.load(Ordering::Relaxed) { "ON" } else { "OFF" }));
    { s.push_str("  후보 Game 테이블(발화수):");
      for k in 0..RT_TAB_N { let g = RT_TAB_G[k].load(Ordering::Relaxed); if g == 0 { continue; }
        s.push_str(&format!(" [{:#x}×{}]", g, RT_TAB_C[k].load(Ordering::Relaxed))); }
      s.push('\n'); }
    { let g = CAND_LOG.lock().unwrap_or_else(|e| e.into_inner());
      if !g.is_empty() { s.push_str("  후보별 판정:\n"); s.push_str(&g); } }
    s.push('\n');
    s.push_str("[poke 깔때기]\n");
    let cnt = PK_COUNT.load(Ordering::Relaxed);
    let fmt = |v: u64| if v == u64::MAX { "미관측".to_string() } else { v.to_string() };
    s.push_str(&format!("  post_update={} World읽기OK={} 참가자배열OK={} athlete수(마지막)={} 유효참가자={} 실제write={} 타당성skip={}\n",
        ld(&PK_FRAMES), ld(&PK_WORLD_OK), ld(&PK_ARR_OK), fmt(cnt), ld(&PK_REC_OK), ld(&PK_WROTE), ld(&PK_SKIP_SANITY)));
    // ★진단강화: 마지막값=0에 속지 않게 최대값·유효프레임·경로 분리
    s.push_str(&format!("  [최대·누적] max_athlete수={} max_unit수={} ath>0프레임={}  ← 마지막=0이어도 max>0이면 재생중엔 있었음\n",
        ld(&MAX_ATH_COUNT), ld(&MAX_UNIT_COUNT), ld(&FRAMES_WITH_ATH)));
    let ok_s = |v: u64| match v { 1 => "유효", 2 => "무효/읽기실패", _ => "미확인" };
    s.push_str(&format!("  [경로분리] World={:#x} · ath_base={} · ath수(마지막)={} || unit_base={} · unit수(마지막)={}  ← 어느 경로가 0인지\n",
        ld(&DBG_WORLD), ok_s(ld(&DBG_ATH_BASE_OK)), fmt(cnt),
        ok_s(ld(&DBG_UNIT_BASE_OK)), fmt(DBG_UNIT_COUNT.load(Ordering::Relaxed))));
    s.push_str(&format!("  [진영 team값] blue(작은쪽)={} red(큰쪽)={}   ⚠매핑 검증필요: 실제 blue 팀이 이 값인지 인게임 확인\n",
        { let a = SIDE_A.load(Ordering::Relaxed); if a == u64::MAX { "미관측".to_string() } else { format!("{:#x}", a) } },
        { let b = SIDE_B.load(Ordering::Relaxed); if b == u64::MAX { "미관측".to_string() } else { format!("{:#x}", b) } }));
    s.push_str(&format!("  [오프셋] Game+{:#x}=World · World+{:#x}=참가자배열 · +{:#x}=수 · unit +{:#x}/+{:#x} · stride={:#x} · 판단력+{:#x} · 오더+{:#x} · side+{:#x}\n",
        O_GAME_WORLD, O_ATH_ARR, O_ATH_COUNT, O_UNIT_ARR, O_UNIT_COUNT, ARR_STRIDE, O_JUDGE, O_ORDER, O_SIDE));
    // ── ★액션상태(U+0xA47) side별 히스토그램 — behavior 훅(FUN_140eafed0) 집계 ──
    let sh = ld(&SHOOK_INSTALLED);
    let sh_s = match sh { 1 => "설치됨", 2 => "★설치실패", _ => if CFG_HIST.load(Ordering::Relaxed) { "대기중" } else { "OFF(histogram=0)" } };
    s.push_str(&format!("\n[액션상태 히스토그램]  훅(U+0xA47 behavior)={} · 발화={} · side범위밖skip={}\n",
        sh_s, ld(&SHOOK_N), ld(&SHOOK_SIDE_OOB)));
    if sh == 2 { s.push_str("  ⚠훅 설치실패 = 집계 안 됨(프롤로그 불일치/체인 대기). 액션상태 데이터 없음.\n"); }
    s.push_str("  ★핵심 지표 = red 0xb(후퇴)비율. order=1 패치 시 이 비율이 낮아지면 오더→후퇴 다운스트림 확정. (state 0xb=후퇴 5=교전 3=존)\n");
    for (si, label) in [(0usize, "blue"), (1usize, "red")].iter() {
        let mut tot = HIST_OTHER[*si].load(Ordering::Relaxed);
        for st in 0..16 { tot += HIST_STATE[*si][st].load(Ordering::Relaxed); }
        let retreat = HIST_STATE[*si][0xb].load(Ordering::Relaxed);
        let engage = HIST_STATE[*si][0x5].load(Ordering::Relaxed);
        let pct = |n: u64| if tot == 0 { 0.0 } else { n as f64 * 100.0 / tot as f64 };
        s.push_str(&format!("  {} 총관측={} | 후퇴(0xb)={} ({:.1}%) 교전(5)={} ({:.1}%) 존(3)={} 기타≥16={}\n",
            label, tot, retreat, pct(retreat), engage, pct(engage),
            HIST_STATE[*si][0x3].load(Ordering::Relaxed), HIST_OTHER[*si].load(Ordering::Relaxed)));
        s.push_str("    state별:");
        for st in 0..16 { let c = HIST_STATE[*si][st].load(Ordering::Relaxed); if c > 0 { s.push_str(&format!(" {:#x}={}", st, c)); } }
        s.push('\n');
    }
    // ── ★HW 워치포인트: +0x7c8 오더 콜큐(U+0xD08 len) 읽는 RIP 포착 ──
    if CFG_WP.load(Ordering::Relaxed) {
        let dis = ld(&WP_DISARMED);
        let dis_s = match dis { 0 => "활성", 1 => "자동해제(상한도달)", 2 => "해제(cfg off)", 3 => "★설치실패", _ => "?" };
        s.push_str(&format!("\n[워치포인트] watchpoint=1 · 무장={} · 상태={}\n", WP_ARMED.load(Ordering::Relaxed), dis_s));
        s.push_str("  워치주소(U+0xD08 len, DR0~DR3):");
        let mut nt = 0;
        for i in 0..WP_NT { let a = WP_TARGETS[i].load(Ordering::Relaxed); if a != 0 { nt += 1; s.push_str(&format!(" {:#x}", a)); } }
        s.push_str(&format!("  (타깃수={} 첫U={:#x})\n", nt, ld(&WP_U)));
        s.push_str(&format!("  전스레드설정={} · apply={} · 총히트={} 유니크={}\n",
            ld(&WP_THREADS), ld(&WP_APPLY_N), ld(&WP_HIT_TOTAL), ld(&WP_UNIQUE)));
        if nt == 0 { s.push_str("  (유효 U 미포착 — 큐 필드 있는 유닛 아직 없음. 전투 관전 필요)\n"); }
        s.push_str("  ★소비자 RIP 목록 (exe-상대 offset·count) — 생산자(push)와 신규(read=소비자)는 offset으로 판별:\n");
        // count 내림차순 유사 정렬 없이 슬롯 순 출력(비대 방지 = 상위 64).
        let mut any = false;
        for k in 0..WP_SLOTS {
            let off = WP_HITS_OFF[k].load(Ordering::Relaxed);
            if off == 0 { continue; }
            any = true;
            s.push_str(&format!("    {:#x} × {}\n", off, WP_HITS_CNT[k].load(Ordering::Relaxed)));
        }
        if !any { s.push_str("    (히트 없음 — 무장 후 전투 진행 필요)\n"); }
    }
    s.push_str(&format!("\n[RAW 참가자 샘플 (write 무관·오프셋 정합 최종판정) — 샘플수={}]\n", ld(&RAW_SAMPLE_N)));
    s.push_str("  ★판단력(+0x218)·오더(+0x220) dec 값이 능력치범위(~0~100대)면 정합. 1000/큰값/포인터면 아직 틀림.\n");
    { let g = RAW_SAMPLE.lock().unwrap_or_else(|e| e.into_inner());
      if g.is_empty() { s.push_str("  (아직 없음 — 참가자 레코드 미도달)\n"); } else { s.push_str(&g); } }
    s.push_str("\n[적용 로그 (old→new, 값변경시)]\n");
    { let g = LOG_BUF.lock().unwrap_or_else(|e| e.into_inner());
      if g.is_empty() { s.push_str("  (아직 없음 — 화면 경기 진입 + cfg 값 지정 필요)\n"); }
      else { s.push_str(&g); } }
    if let Some(d) = mod_dir() { let _ = fs::create_dir_all(&d); let _ = fs::write(d.join("stat_exp.txt"), s); }
}

// ===========================================================================
//  SDK 라이프사이클
// ===========================================================================
struct StatExpExt;
impl ModExtension for StatExpExt {
    fn on_init(&self, _s: &mut Scene, _u: &mut GameUI, _a: &mut Assets) {}
    fn post_update(&self, _scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            install_run_tick_hook(); // ★현재 World Game 캡처(다시보기 신뢰소스, 우선)
            install_state_hook();    // ★액션상태 집계 훅(U+0xA47) — histogram 또는 watchpoint 시
            install_launcher_hook(); // 폴백 + 화면판정
            apply_poke();
            wp_maintain();           // ★워치포인트 무장/재적용/자동해제(cfg watchpoint=1일 때만)
            flush_status();
        }));
    }
}
struct StatExpServerExt;
impl ModServerExtension for StatExpServerExt {
    fn on_server_start(&self, _ctx: &mut ServerModContext) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| { install_run_tick_hook(); install_state_hook(); install_launcher_hook(); }));
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    seh_install();
    load_cfg();
    install_run_tick_hook();
    install_state_hook();
    install_launcher_hook();
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(StatExpExt);
    reg.set_server_extension(StatExpServerExt);
    reg
}
declare_mod!(init);
