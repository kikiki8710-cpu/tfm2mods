//! Scrim_Probe — 내부 스크림(커스텀 매치) 본기능
//! ===========================================================================
//! 검증된 기반(당신 완성형 lib.rs 그대로 사용):
//!   - 클릭 감지 = ui.filter_handler 에 (filter, handler) push, UIEvent::Click{path} 매칭
//!   - 라인업 주입 = ARM 동안 매 프레임 match_replays[key] 슬롯에 athlete_id/champion 강제
//!   - 진입 시 백업 / 종료 시 복원, champion 은 leak 버퍼 repoint
//!   - replay_popup 소환 = pause_stack 에 32바이트 ReplayPopup(MatchType) raw push
//!
//! 이 파일이 추가/해결한 것:
//!   - 하드코딩(key=0, 팀115/117, 라인업 상수, SUMMON_MATCH_ID=1176) 전부 제거
//!   - 경기 리스트에서 "맨 처음 완료된 연습경기"를 찾아:
//!       popup match_id = 그 연습경기 match_id (Practice)
//!       주입 key       = 그 연습경기 match_info.replays[0]  ← match_replays 의 key
//!     => popup 과 주입이 같은 replay 를 가리켜 §2 매핑문제 해소.
//!   - 연습경기 없으면 / 로스터 10명 미만이면 안내 문구
//!   - 설정 모달: 선수10 + 챔프10 슬롯(클릭=다음 후보, 중복 자동 제외), 다 차면 시작 활성
//! ===========================================================================
#![allow(dead_code, unused_imports, unused_variables)]
use mod_api::*;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU8, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "Scrim_Probe";

// MatchReplayData / MatchReplayAthlete 오프셋 (소환성공 §8)
const O_BLUE_TEAM_ID: usize = 672;
const O_RED_TEAM_ID: usize = 680;
const O_SEED: usize = 688;
const O_BLUE_TEAM: usize = 48; // Vec: ptr@+8, len@+16
const O_RED_TEAM: usize = 72;
const O_BLUE_BAN: usize = 0;  // Vec<String> (cap@+0, ptr@+8, len@+16)
const O_RED_BAN: usize = 24;  // Vec<String>
const O_BLUE_STRAT: usize = 0x78; // Strategy(24B, POD 추정)
const O_RED_STRAT: usize = 0x90;
const ATH_STRIDE: usize = 544;
const AO_CHAMPION: usize = 232; // String(cap, ptr, len)
const AO_ATHLETE_ID: usize = 520;
const AO_POSITION: usize = 536; // Position enum: Top0 Jungle1 Mid2 Bottom3 Support4
const AO_ITEMS: usize = 256; // Vec<아이템ID u64> (cap@256, ptr@264, len@272) = 산 아이템(결과)
const TEXT_OFFSET: usize = 352; // LabelRunner.text
const MT_PRACTICE: u64 = 2; // MatchType disc: Tutorial0 Normal1 Practice2 SoloRank3

// 테스트 스위치: 0=자동(맨 처음 연습경기). 0이 아니면 그 match_id(예: 1176)를 강제로 사용.
// 매핑 검증용. 자동 선택이 미덥지 않을 때 1176 으로 고정해 엔진부터 확인.
const FORCE_PRACTICE_MATCH_ID: u64 = 0;

// ── 상태 ──
static BOOTED: AtomicBool = AtomicBool::new(false);
static HANDLER_INSTALLED: AtomicBool = AtomicBool::new(false);
static SCRIM_MODAL_VISIBLE: AtomicBool = AtomicBool::new(false);
static SCRIM_ARMED: AtomicBool = AtomicBool::new(false);
static INJECTED_ONCE: AtomicBool = AtomicBool::new(false); // 1회 주입 플래그 (매프레임 대신)
static SUMMON_REQUESTED: AtomicBool = AtomicBool::new(false);
static CAPTURED: AtomicBool = AtomicBool::new(false);
static POPUP_ACTIVE: AtomicBool = AtomicBool::new(false); // 다시보기 팝업 떠있는 동안
static PREV_POPUP_PRESENT: AtomicBool = AtomicBool::new(false); // 직전 프레임 팝업 존재 여부
static PREV_REPLAY_SOME: AtomicBool = AtomicBool::new(false);
static REPLAY_LOG_TICK: AtomicU64 = AtomicU64::new(0); // 다시보기 로그 간격 카운터
static CONFIG_READY: AtomicBool = AtomicBool::new(false); // 게이트 통과(설정 가능)
static SUMMON_MID: AtomicU64 = AtomicU64::new(u64::MAX); // popup 용 Practice match_id
static INJECT_KEY: AtomicI64 = AtomicI64::new(-1); // 주입 대상 match_replays key

// ── 밸런스 검증(읽기전용) ──
static THINK_CALLS: AtomicUsize = AtomicUsize::new(0);
static BALANCE_LOGGED: AtomicBool = AtomicBool::new(false); // 재생 1회당 현재스탯 1회 로그
static OUR_ATHLETES: Mutex<Vec<usize>> = Mutex::new(Vec::new()); // 우리 경기 선수 athlete_id (think 필터)
// ── pre_patch_data 백업/복원 (스크림만 현재밸런스) ──
static PRE_PATCH_BAK: Mutex<Option<HashMap<String, GamePatchState>>> = Mutex::new(None);
static REPLAY_STARTED: AtomicBool = AtomicBool::new(false);   // think 가 우리경기 감지 = 재생 시작
static PRE_PATCH_RESTORE_PENDING: AtomicBool = AtomicBool::new(false);
static STRAT_APPLIED_LOGGED: AtomicBool = AtomicBool::new(false); // 주입 24B 덤프 1회 제한(재생마다 리셋)

// 슬롯 선택값
static PLAYER_SLOTS: Mutex<[Option<usize>; 10]> = Mutex::new([None; 10]);
static CHAMP_SLOTS: Mutex<[Option<String>; 10]> =
    Mutex::new([None, None, None, None, None, None, None, None, None, None]);
// 후보(모달 열 때 1회 채움)
static ROSTER: Mutex<Vec<(usize, String)>> = Mutex::new(Vec::new());
static CHAMPS: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());
// filter_handler → post_update 로 넘기는 클릭 큐 (DB 접근이 filter 안에선 불가하므로)
//   kind: 0=open, 1=player슬롯, 2=champ슬롯, 3=start, 4=close, 5=드롭다운행, 6=드롭다운닫기
static CLICK_QUEUE: Mutex<Vec<(u8, usize)>> = Mutex::new(Vec::new());

// ── 전술(팀)/아이템(개인) 설정 상태 ──
// kind 추가: 10=open_strat 11=open_items 12=strat_ok 13=strat_cancel 14=items_ok 15=items_cancel
//            16=strat_box(slot=team*12+field) 17=item_box(slot=slot*3+s)
static STRAT_OPEN: AtomicBool = AtomicBool::new(false);
static ITEMS_OPEN: AtomicBool = AtomicBool::new(false);
static STRAT_COMMITTED: Mutex<[[u8; 12]; 2]> = Mutex::new([[0; 12]; 2]); // [team][field]
static STRAT_WORKING: Mutex<[[u8; 12]; 2]> = Mutex::new([[0; 12]; 2]);
static ITEMS_COMMITTED: Mutex<[[u8; 3]; 10]> = Mutex::new([[0; 3]; 10]); // [slot][itemslot]
static ITEMS_WORKING: Mutex<[[u8; 3]; 10]> = Mutex::new([[0; 3]; 10]);
// 스플릿 담당 포지션 [team] = (bld담당, mor첫담당, mor둘째담당). 기본 탑(0). (담당선택 UI는 추후)
static STRAT_SPLIT_POS: Mutex<[(u8, u8, u8); 2]> = Mutex::new([(0, 0, 0); 2]);
// 전술 모달에서 지금 보고 있는 팀 (0=블루, 1=레드). 토글로 전환.
static STRAT_VIEW_TEAM: AtomicU8 = AtomicU8::new(0);
// 전술 12필드 키(노드 id 접미사) + 변형 라벨(순서=disc 0,1,2…)
const SKEYS: [&str; 12] = ["foc","jng","srp","srt","bld","bat","mor","twr","def","fin","wav","end"];
const STRAT_OPTS: [&[&str]; 12] = [
    &["탑/미드 집중", "미드/바텀 집중", "구분 없이 전부"],
    &["성장/커버 위주", "라인 개입 위주", "카운터 정글 위주"],
    &["무조건 시도", "유연하게 판단", "되도록 포기"],
    &["반드시 합류", "유연하게 판단", "합류하지 않음"],
    &["최대한 모이기", "유연하게 판단", "스플릿"],
    &["거리 유지 견제", "빠른 이니시에이팅"],
    &["모두 모이기", "1-4 스플릿", "1-3-1 스플릿"],
    &["다이브", "거리 유지 견제"],
    &["밀리는 라인 방어", "교전 유도"],
    &["처치 우선", "전투 우선"],
    &["웨이브 우선", "합류 우선"],
    &["안정적", "유연하게", "공격적"],
];
const ITEM_OPTS: [&str; 7] = ["자동", "공격력", "주문력", "공격속도", "방어력", "마저", "체력"];
// 전술 24B 필드→바이트 매핑. SKEYS: foc jng srp srt bld bat mor twr def fin wav end
// 분석(26076샘플): byte0/4=스플릿내장필드(빌드업/모르가드,0~6), byte8=Position(담당),
//   byte12~16=2변형 5개(bat/twr/def/fin/wav), byte17~21=3변형 5개(foc/jng/srp/srt/end).
//   ★그룹은 근거강함, 그룹내 순서는 UI테스트로 확정.
const STRAT_OFFS: [usize; 12] = [
    17, // foc (3변형)
    18, // jng (3변형)
    19, // srp (3변형)
    20, // srt (3변형)
    0,  // bld (스플릿내장: 0모이기 1유연 2~6스플릿@pos)
    12, // bat (2변형)
    4,  // mor (스플릿내장)
    13, // twr (2변형)
    14, // def (2변형)
    15, // fin (2변형)
    16, // wav (2변형)
    21, // end (3변형)
];

// 드롭다운(풀다운) 상태
const PP_ROWS: usize = 14; // 선수창: 1열 × 14행/페이지
const CC_ROWS: usize = 42; // 챔피언창: 3열 × 14행 = 42/페이지
static DD_OPEN: AtomicBool = AtomicBool::new(false);
static DD_KIND: AtomicUsize = AtomicUsize::new(0); // 0=선수, 1=챔피언
static DD_SLOT: AtomicUsize = AtomicUsize::new(0); // 어느 슬롯을 채우는 중인지
static DD_PAGE: AtomicUsize = AtomicUsize::new(0); // 현재 페이지
// 현재 펼친 목록: (선수id, 챔프key, 표시라벨)
static DD_ITEMS: Mutex<Vec<(Option<usize>, Option<String>, String)>> = Mutex::new(Vec::new());

fn dd_rows() -> usize { if DD_KIND.load(Ordering::Relaxed) == 0 { PP_ROWS } else { CC_ROWS } }
fn dd_prefix() -> &'static str { if DD_KIND.load(Ordering::Relaxed) == 0 { "scrim_ddp" } else { "scrim_ddc" } }
fn dd_pages() -> usize {
    let rows = dd_rows();
    let len = DD_ITEMS.lock().unwrap().len();
    ((len + rows - 1) / rows).max(1)
}


// 원본 백업
struct SlotBak { aid: u64, cap: u64, ptr: u64, len: u64, pos: u64, item_len: u64, items: [u64; 3] }
struct VecBak { cap: u64, ptr: u64, len: u64 }
struct Backup { key: usize, seed: u64, slots: Vec<SlotBak>, blue_ban: VecBak, red_ban: VecBak, blue_tid: u64, red_tid: u64, blue_strat: [u8; 24], red_strat: [u8; 24] }
static BACKUP: Mutex<Option<Backup>> = Mutex::new(None);
static MY_TEAM_ID: AtomicU64 = AtomicU64::new(u64::MAX); // 내 팀 id (모달 열 때 저장)
static MY_TEAM_NAME: Mutex<String> = Mutex::new(String::new());
static MY_TEAM_LOGO: Mutex<String> = Mutex::new(String::new());
static LOGO_DIAG_DONE: AtomicBool = AtomicBool::new(false);
static POPUP_DUMPED: AtomicBool = AtomicBool::new(false);

// ===========================================================================
//  WinAPI 로그
// ===========================================================================
type HMODULE = isize; type DWORD = u32; type BOOL = i32;
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleExW(f: DWORD, name: *const u16, h: *mut HMODULE) -> BOOL;
    fn GetModuleHandleW(name: *const u16) -> HMODULE;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, n: DWORD) -> DWORD;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old_protect: *mut u32) -> BOOL;
    fn FlushInstructionCache(proc: usize, addr: usize, size: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
}
#[repr(C)]
#[derive(Default)]
struct MemBasicInfo {
    base: usize, alloc_base: usize, alloc_protect: u32, _pad0: u32,
    region_size: usize, state: u32, protect: u32, mtype: u32, _pad1: u32,
}
// addr 부터 len 바이트가 "읽기 가능"한 커밋 메모리인지 (크래시 방지용)
unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    let n = VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>());
    if n == 0 { return false; }
    const MEM_COMMIT: u32 = 0x1000;
    const READABLE: u32 = 0x02 | 0x04 | 0x20 | 0x40; // R, RW, ER, ERW
    const NOACCESS_GUARD: u32 = 0x01 | 0x100; // PAGE_NOACCESS | PAGE_GUARD
    if mbi.state != MEM_COMMIT { return false; }
    if mbi.protect & NOACCESS_GUARD != 0 { return false; }
    if mbi.protect & READABLE == 0 { return false; }
    // 요청 범위가 이 영역 안에 들어오는지
    addr + len <= mbi.base + mbi.region_size
}
// addr 부터 len 바이트가 "쓰기 가능"한 커밋 메모리인지
unsafe fn writable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    let n = VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>());
    if n == 0 { return false; }
    const MEM_COMMIT: u32 = 0x1000;
    const WRITABLE: u32 = 0x04 | 0x08 | 0x40 | 0x80; // RW, WRITECOPY, ERW, EWC
    const GUARD: u32 = 0x100;
    if mbi.state != MEM_COMMIT { return false; }
    if mbi.protect & GUARD != 0 { return false; }
    if mbi.protect & WRITABLE == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}

// ===========================================================================
//  트램폴린 후킹 — FUN_1419a4c00 진입시 param_2(ctx) 캡처 (champ id 확인용)
// ===========================================================================
static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
static CAPTURE_DONE: AtomicBool = AtomicBool::new(false);
static CAPTURE_SEQ: AtomicU64 = AtomicU64::new(0); // 캡처 횟수 (새 캡처 감지)
static ITEMNET_CALL_TOTAL: AtomicU64 = AtomicU64::new(0); // ★ 신경망 forward 총 호출 횟수 (중복 포함)
static CAPTURED_CTX: Mutex<[u64; 32]> = Mutex::new([0u64; 32]);
static CAPTURED_STACK: Mutex<[u64; 24]> = Mutex::new([0u64; 24]); // 진입시점 스택 덤프
static LAST_CAP_KEY: AtomicU64 = AtomicU64::new(0); // 직전 캡처 식별 (중복 방지)
static CAPTURED_ARGS: Mutex<[u64; 8]> = Mutex::new([0u64; 8]); // [item_net,ctx,cands,n,flag,cand0,cand1,cand2]
static CAPTURED_CANDS: Mutex<[u64; 32]> = Mutex::new([0u64; 32]); // ★ 캡처순간 cands 전체(최대32)
static CAPTURED_CANDS_N: AtomicU64 = AtomicU64::new(0); // cands 실제 개수

// 트램폴린이 호출하는 캡처 함수.
//   saved_ptr = 저장된 레지스터 영역. 레이아웃(낮은→높은 주소):
//     [+0x00]=r11 [+0x08]=r10(entry_rsp) [+0x10]=r9(n) [+0x18]=r8(cands)
//     [+0x20]=rdx(ctx) [+0x28]=rcx(item_net)
//   entry_rsp = 진입 시점 rsp (호출처 스택). flag(param_5)는 여기 [+0x28]에 있음
//     (x64: 인자 5번째 = [entry_rsp + 0x28], shadow 0x20 + 리턴 8)
unsafe extern "C" fn itemnet_capture(saved_ptr: usize, entry_rsp: usize) {
    ITEMNET_CALL_TOTAL.fetch_add(1, Ordering::Relaxed); // ★ 모든 호출 카운트 (중복 포함)
    if saved_ptr < 0x10000 { return; }
    let item_net = *((saved_ptr + 0x28) as *const u64); // rcx
    let ctx_ptr = *((saved_ptr + 0x20) as *const u64) as usize; // rdx
    let cands = *((saved_ptr + 0x18) as *const u64); // r8
    let n = *((saved_ptr + 0x10) as *const u64); // r9
    // flag = 5번째 인자 = entry_rsp + 0x28 (리턴주소8 + shadow 0x20)
    let flag = if entry_rsp > 0x10000 && readable(entry_rsp + 0x28, 8) { *((entry_rsp + 0x28) as *const u64) } else { 0 };
    if ctx_ptr < 0x10000 || !readable(ctx_ptr, 0x58) { return; }
    let key = {
        let mut k = 0u64;
        for i in 0..5 { k = k.wrapping_mul(61).wrapping_add(*((ctx_ptr + i*8) as *const u64)); }
        k
    };
    if LAST_CAP_KEY.load(Ordering::Relaxed) == key { return; }
    let mut buf = [0u64; 32];
    for i in 0..32 {
        if readable(ctx_ptr + i*8, 8) { buf[i] = *((ctx_ptr + i*8) as *const u64); }
    }
    *CAPTURED_CTX.lock().unwrap() = buf;
    // 호출 인자 저장 + cands 배열 내용 (n개, 최대 3개)
    let mut args = [0u64; 8];
    args[0] = item_net; args[1] = ctx_ptr as u64; args[2] = cands; args[3] = n; args[4] = flag;
    if cands > 0x10000 {
        for i in 0..3.min(n as usize) {
            if readable(cands as usize + i*8, 8) { args[5+i] = *((cands as usize + i*8) as *const u64); }
        }
    }
    *CAPTURED_ARGS.lock().unwrap() = args;
    // ★ cands 전체를 캡처 순간에 복사 (나중에 읽으면 메모리 재사용돼 쓰레기됨)
    {
        let mut cc = [0u64; 32];
        let nn = (n as usize).min(32);
        if cands > 0x10000 {
            for i in 0..nn {
                if readable(cands as usize + i*8, 8) { cc[i] = *((cands as usize + i*8) as *const u64); }
            }
        }
        *CAPTURED_CANDS.lock().unwrap() = cc;
        CAPTURED_CANDS_N.store(n, Ordering::Relaxed);
    }
    // 진입시점 스택 덤프 (rsp[0]=리턴주소, 위로 호출처 지역변수)
    let mut st = [0u64; 24];
    if entry_rsp > 0x10000 {
        for i in 0..24 {
            if readable(entry_rsp + i*8, 8) { st[i] = *((entry_rsp + i*8) as *const u64); }
        }
    }
    *CAPTURED_STACK.lock().unwrap() = st;
    LAST_CAP_KEY.store(key, Ordering::Relaxed);
    CAPTURE_SEQ.fetch_add(1, Ordering::Relaxed);
    CAPTURE_DONE.store(true, Ordering::Relaxed);
}

// 트램폴린 설치: FUN 첫 12B 백업 → stub 생성 → FUN 첫 12B를 jmp stub 으로 패치
unsafe fn install_itemnet_hook() -> Result<usize, &'static str> {
    if HOOK_INSTALLED.swap(true, Ordering::Relaxed) { return Err("이미 설치됨"); }
    let mbase = GetModuleHandleW(core::ptr::null()) as usize;
    if mbase == 0 { return Err("module base 0"); }
    let fn_addr = mbase + 0x19a4c00;
    if !readable(fn_addr, 16) { return Err("fn not readable"); }
    // 시작 바이트 검증
    let expect = [0x55u8,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];
    for i in 0..12 { if *((fn_addr+i) as *const u8) != expect[i] { return Err("바이트 불일치"); } }

    // stub 메모리 (RWX) 확보
    const MEM_COMMIT_RESERVE: u32 = 0x1000 | 0x2000;
    const PAGE_EXECUTE_READWRITE: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_COMMIT_RESERVE, PAGE_EXECUTE_READWRITE);
    if stub == 0 { return Err("VirtualAlloc 실패"); }

    let cap_fn = itemnet_capture as usize;
    let ret_addr = fn_addr + 12; // 원본 push 12B 다음

    // stub 바이트 작성
    let mut s: Vec<u8> = Vec::new();
    // 진입 시점 rsp 를 r10 에 저장 (push 하기 전 = 호출처 리턴주소 위치)
    s.extend_from_slice(&[0x49,0x89,0xe2]);    // mov r10, rsp
    // 레지스터 보존 (push 순서: rcx,rdx,r8,r9,r10,r11 → 스택에 역순 저장)
    s.extend_from_slice(&[0x51]);             // push rcx  (item_net)
    s.extend_from_slice(&[0x52]);             // push rdx  (ctx)
    s.extend_from_slice(&[0x41,0x50]);        // push r8   (cands)
    s.extend_from_slice(&[0x41,0x51]);        // push r9   (n)
    s.extend_from_slice(&[0x41,0x52]);        // push r10  (entry_rsp)
    s.extend_from_slice(&[0x41,0x53]);        // push r11
    // 이 시점 rsp = 저장된 레지스터들 [r11,r10,r9,r8,rdx,rcx] (낮은주소→높은주소)
    //   [rsp+0]=r11 [rsp+8]=r10(entry_rsp) [rsp+0x10]=r9(n) [rsp+0x18]=r8(cands)
    //   [rsp+0x20]=rdx(ctx) [rsp+0x28]=rcx(item_net)
    // 캡처 함수(saved_ptr, entry_rsp) 호출: rcx=현재rsp, rdx=r10(entry_rsp)
    s.extend_from_slice(&[0x48,0x89,0xe1]);    // mov rcx, rsp   (저장영역 포인터)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);    // mov rdx, r10   (진입시점 rsp)
    // sub rsp, 0x28 (shadow space + 정렬)
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);
    // mov rax, cap_fn ; call rax
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff,0xd0]);
    // add rsp, 0x28
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);
    // 레지스터 복원
    s.extend_from_slice(&[0x41,0x5b]);        // pop r11
    s.extend_from_slice(&[0x41,0x5a]);        // pop r10
    s.extend_from_slice(&[0x41,0x59]);        // pop r9
    s.extend_from_slice(&[0x41,0x58]);        // pop r8
    s.extend_from_slice(&[0x5a]);             // pop rdx
    s.extend_from_slice(&[0x59]);             // pop rcx
    // 원본 12바이트 재현 (push 8개)
    s.extend_from_slice(&expect);
    // mov rax, ret_addr ; jmp rax
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes());
    s.extend_from_slice(&[0xff,0xe0]);
    // stub 메모리에 복사
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());

    // FUN 첫 12B 를 jmp stub 으로 패치: mov rax, stub ; jmp rax
    let mut patch = [0u8; 12];
    patch[0] = 0x48; patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&stub.to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;
    // 코드영역 쓰기 가능하게
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, PAGE_EXECUTE_READWRITE, &mut old) == 0 { return Err("VirtualProtect 실패"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    Ok(stub)
}

fn dll_path() -> Option<PathBuf> {
    unsafe {
        let addr = dll_path as *const () as usize;
        let mut h: HMODULE = 0;
        if GetModuleHandleExW(0x4 | 0x2, addr as *const u16, &mut h) == 0 || h == 0 { return None; }
        let mut buf = [0u16; 4096];
        let len = GetModuleFileNameW(h, buf.as_mut_ptr(), buf.len() as DWORD);
        if len == 0 { return None; }
        Some(PathBuf::from(String::from_utf16_lossy(&buf[..len as usize])))
    }
}
fn write_log(name: &str, content: &str) {
    let Some(p) = dll_path().and_then(|p| p.parent().map(|d| d.join(name))) else { return; };
    let _ = fs::write(p, content);
}
fn append_log(name: &str, line: &str) {
    use std::io::Write;
    let Some(p) = dll_path().and_then(|p| p.parent().map(|d| d.join(name))) else { return; };
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(p) {
        let _ = writeln!(f, "{}", line);
    }
}
// ★ UI_Dump/ 하위에 순번 파일(ui_dump_N.txt)로 저장. 디렉터리 없으면 생성. (DLL 폴더 기준)
fn dump_seq_write(seq: u64, content: &str) {
    let Some(dir) = dll_path().and_then(|p| p.parent().map(|d| d.join("UI_Dump"))) else { return; };
    let _ = fs::create_dir_all(&dir);
    let _ = fs::write(dir.join(format!("ui_dump_{}.txt", seq)), content);
}
fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }
fn read_text(name: &str) -> Option<String> {
    let p = dll_path().and_then(|p| p.parent().map(|d| d.join(name)))?;
    fs::read_to_string(p).ok()
}
// 선택(선수10+챔프10)을 파일로 저장 → 창 닫거나 게임 재시작해도 유지
fn save_selection() {
    let p = PLAYER_SLOTS.lock().unwrap();
    let c = CHAMP_SLOTS.lock().unwrap();
    let ps: Vec<String> = p.iter().map(|o| o.map(|x| x.to_string()).unwrap_or_else(|| "-".into())).collect();
    let cs: Vec<String> = c.iter().map(|o| o.clone().unwrap_or_else(|| "-".into())).collect();
    write_log("scrim_selection.txt", &format!("players:{}\nchamps:{}\n", ps.join(","), cs.join(",")));
}
// 파일에서 선택 복원 (세션 첫 오픈 시)
fn load_selection() {
    let Some(txt) = read_text("scrim_selection.txt") else { return; };
    for line in txt.lines() {
        if let Some(v) = line.strip_prefix("players:") {
            let mut p = PLAYER_SLOTS.lock().unwrap();
            for (i, tok) in v.split(',').enumerate().take(10) {
                p[i] = tok.trim().parse::<usize>().ok();
            }
        } else if let Some(v) = line.strip_prefix("champs:") {
            let mut c = CHAMP_SLOTS.lock().unwrap();
            for (i, tok) in v.split(',').enumerate().take(10) {
                let t = tok.trim();
                c[i] = if t == "-" || t.is_empty() { None } else { Some(t.to_string()) };
            }
        }
    }
}
// 현재 로스터/챔프 목록에 없는 선택은 비움 (트레이드/다른 세이브 대비). 중복도 정리.
fn validate_selection() {
    let valid_p: Vec<usize> = ROSTER.lock().unwrap().iter().map(|(id, _)| *id).collect();
    let valid_c: Vec<String> = CHAMPS.lock().unwrap().iter().map(|(k, _)| k.clone()).collect();
    let mut p = PLAYER_SLOTS.lock().unwrap();
    let mut seen_p: Vec<usize> = Vec::new();
    for s in p.iter_mut() {
        if let Some(id) = *s {
            if !valid_p.contains(&id) || seen_p.contains(&id) { *s = None; } else { seen_p.push(id); }
        }
    }
    let mut c = CHAMP_SLOTS.lock().unwrap();
    let mut seen_c: Vec<String> = Vec::new();
    for s in c.iter_mut() {
        if let Some(k) = s.clone() {
            if !valid_c.contains(&k) || seen_c.contains(&k) { *s = None; } else { seen_c.push(k); }
        }
    }
}

#[inline] unsafe fn rd_u64(p: usize) -> u64 { std::ptr::read_unaligned(p as *const u64) }
#[inline] unsafe fn wr_u64(p: usize, v: u64) { std::ptr::write_unaligned(p as *mut u64, v); }
#[inline] fn addr_of<T>(r: &T) -> usize { r as *const T as usize }
fn looks_heap(v: u64) -> bool { v & 0x7 == 0 && v >= 0x10000 && v < 0x0000_8000_0000_0000 && (v & 0xffff) != 0 }
unsafe fn read_str(ptr: u64, n: usize) -> String {
    let mut v = Vec::new();
    if looks_heap(ptr) {
        for i in 0..n {
            let b = *((ptr as usize + i) as *const u8);
            if (0x20..0x7f).contains(&b) { v.push(b); } else { break; }
        }
    }
    String::from_utf8_lossy(&v).into_owned()
}
unsafe fn champ_repoint(slot: usize, off: usize, s: &str) {
    let boxed: Box<[u8]> = s.as_bytes().to_vec().into_boxed_slice();
    let len = boxed.len() as u64;
    let ptr = Box::leak(boxed).as_ptr() as u64;
    wr_u64(slot + off, len);
    wr_u64(slot + off + 8, ptr);
    wr_u64(slot + off + 16, len);
}

// ===========================================================================
//  Node 헬퍼
// ===========================================================================
fn set_visible_by_id(n: &mut Node, target: &str, vis: bool) -> bool {
    if n.id.as_str() == target { n.visible = vis; return true; }
    for c in n.child.iter_mut() { if set_visible_by_id(c, target, vis) { return true; } }
    false
}
fn set_disabled_by_id(n: &mut Node, target: &str, dis: bool) -> bool {
    if n.id.as_str() == target { n.disabled = dis; return true; }
    for c in n.child.iter_mut() { if set_disabled_by_id(c, target, dis) { return true; } }
    false
}
fn has_id(n: &Node, target: &str) -> bool {
    if n.id.as_str() == target { return true; }
    n.child.iter().any(|c| has_id(c, target))
}
unsafe fn set_label_node(n: &mut Node, text: &str) -> bool {
    if !n.runner.type_name().contains("LabelRunner") { return false; }
    let any: &dyn std::any::Any = n.runner.as_any();
    let parts: [usize; 2] =
        std::mem::transmute::<*const dyn std::any::Any, [usize; 2]>(any as *const dyn std::any::Any);
    *(((parts[0] as *mut u8).add(TEXT_OFFSET)) as *mut String) = text.to_string();
    true
}
fn set_label_by_id(n: &mut Node, target: &str, text: &str) -> bool {
    if n.id.as_str() == target { unsafe { return set_label_node(n, text); } }
    for c in n.child.iter_mut() { if set_label_by_id(c, target, text) { return true; } }
    false
}
unsafe fn read_label_node(n: &Node) -> Option<String> {
    if !n.runner.type_name().contains("LabelRunner") { return None; }
    let any: &dyn std::any::Any = n.runner.as_any();
    let parts: [usize; 2] =
        std::mem::transmute::<*const dyn std::any::Any, [usize; 2]>(any as *const dyn std::any::Any);
    Some((*((parts[0] as *const u8).add(TEXT_OFFSET) as *const String)).clone())
}
// runner 인스턴스 베이스 포인터
unsafe fn runner_base(n: &Node) -> usize {
    let any: &dyn std::any::Any = n.runner.as_any();
    let parts: [usize; 2] =
        std::mem::transmute::<*const dyn std::any::Any, [usize; 2]>(any as *const dyn std::any::Any);
    parts[0]
}
// 안전 probe: VirtualQuery 로 읽기 가능 확인 후에만 문자열 후보를 읽음 → 크래시 없음
unsafe fn probe_strings_safe(n: &Node, out: &mut String) {
    let base = runner_base(n);
    out.push_str(&format!("=== id={} {} @ {:#x} ===\n", n.id.as_str(), n.runner.type_name(), base));
    if !readable(base, 16) { out.push_str("  (base unreadable)\n"); return; }
    let rd = |a: usize| -> Option<usize> {
        if readable(a, 8) { Some(std::ptr::read_unaligned(a as *const usize)) } else { None }
    };
    let try_str = |ptr: usize, len: usize| -> Option<String> {
        if ptr > 0x10000 && len >= 1 && len <= 128 && readable(ptr, len) {
            let sl = std::slice::from_raw_parts(ptr as *const u8, len);
            if let Ok(s) = std::str::from_utf8(sl) {
                if s.chars().all(|c| c == ' ' || !c.is_control()) { return Some(s.to_string()); }
            }
        }
        None
    };
    let mut off = 0usize;
    while off < 1024 {
        if !readable(base + off, 16) { off += 8; continue; }
        let w0 = rd(base + off).unwrap_or(0);
        let w1 = rd(base + off + 8).unwrap_or(0);
        if let Some(s) = try_str(w0, w1) { out.push_str(&format!("  +{:>3} [ptr,len] {:?}\n", off, s)); }
        else if let Some(s) = try_str(w1, w0) { out.push_str(&format!("  +{:>3} [len,ptr] {:?}\n", off, s)); }
        off += 8;
    }
}
fn probe_safe_by_id(n: &Node, id: &str, out: &mut String) -> bool {
    if n.id.as_str() == id { unsafe { probe_strings_safe(n, out); } return true; }
    for c in n.child.iter() { if probe_safe_by_id(c, id, out) { return true; } }
    false
}
// 워드(hex) + 문자열 후보를 같이 덤프 (로고 인덱스/태그 같은 정수 필드까지 보기 위함)
unsafe fn probe_words_safe(n: &Node, out: &mut String) {
    let base = runner_base(n);
    out.push_str(&format!("=== id={} {} @ {:#x} ===\n", n.id.as_str(), n.runner.type_name(), base));
    if !readable(base, 16) { out.push_str("  (unreadable)\n"); return; }
    let try_str = |ptr: usize, len: usize| -> Option<String> {
        if ptr > 0x10000 && len >= 1 && len <= 64 && readable(ptr, len) {
            std::str::from_utf8(std::slice::from_raw_parts(ptr as *const u8, len)).ok()
                .filter(|s| s.chars().all(|c| c == ' ' || !c.is_control())).map(|s| s.to_string())
        } else { None }
    };
    let mut off = 0usize;
    while off < 256 {
        if !readable(base + off, 8) { off += 8; continue; }
        let w = std::ptr::read_unaligned((base + off) as *const usize);
        let w1 = if readable(base + off + 8, 8) { std::ptr::read_unaligned((base + off + 8) as *const usize) } else { 0 };
        let s = try_str(w, w1).or_else(|| try_str(w1, w)).map(|x| format!("  str={:?}", x)).unwrap_or_default();
        out.push_str(&format!("  +{:>3}: {:#018x} ({}){}\n", off, w, w as i64, s));
        off += 8;
    }
}
fn probe_words_by_id(n: &Node, id: &str, out: &mut String) -> bool {
    if n.id.as_str() == id { unsafe { probe_words_safe(n, out); } return true; }
    for c in n.child.iter() { if probe_words_by_id(c, id, out) { return true; } }
    false
}
// ★ 노드 1개 종합 덤프: runner 인스턴스를 u32/f32/바이트로 다 해석 (색·크기·플래그 탐색용)
unsafe fn probe_full_safe(n: &Node, out: &mut String) {
    let base = runner_base(n);
    out.push_str(&format!("\n=== id={} type={} @ {:#x} ===\n",
        n.id.as_str(), n.runner.type_name(), base));
    out.push_str(&format!("  [Node] visible={} disabled={} child={}\n",
        n.visible, n.disabled, n.child.len()));
    if !readable(base, 16) { out.push_str("  (runner base unreadable)\n"); return; }
    out.push_str("  off | u64                | u32a     u32b     | f32a     f32b     | bytes\n");
    let mut off = 0usize;
    while off < 512 { // ★ 256→512 확장 (hover 색 등 깊은 오프셋까지 캡처)
        if !readable(base + off, 8) { off += 8; continue; }
        let w = std::ptr::read_unaligned((base + off) as *const u64);
        let lo = (w & 0xffff_ffff) as u32;
        let hi = (w >> 32) as u32;
        let fa = f32::from_bits(lo);
        let fb = f32::from_bits(hi);
        let bytes: [u8; 8] = w.to_le_bytes();
        let bstr: String = bytes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ");
        out.push_str(&format!("  +{:>3} | {:#018x} | {:08x} {:08x} | {:8.3} {:8.3} | {}\n",
            off, w, lo, hi, fa, fb, bstr));
        off += 8;
    }
}
fn probe_full_by_id(n: &Node, id: &str, out: &mut String) -> bool {
    if n.id.as_str() == id { unsafe { probe_full_safe(n, out); } return true; }
    for c in n.child.iter() { if probe_full_by_id(c, id, out) { return true; } }
    false
}
// ★ 색 오프셋 프로브: COLOR_PROBE_SET 의 각 오프셋에 구분색 f32 RGBA 를 매 프레임 써넣음.
//    인게임에서 버튼 색을 보고 어느 오프셋이 배경/아이콘/텍스트인지 식별. (writable 일 때만)
fn color_probe(root: &mut Node) {
    fn walk(n: &mut Node, id: &str) -> bool {
        if n.id.as_str() == id {
            unsafe {
                let base = runner_base(n);
                for &(off, r, g, b, a) in COLOR_PROBE_SET.iter() {
                    if writable(base + off, 16) {
                        std::ptr::write_unaligned((base + off) as *mut f32, r);
                        std::ptr::write_unaligned((base + off + 4) as *mut f32, g);
                        std::ptr::write_unaligned((base + off + 8) as *mut f32, b);
                        std::ptr::write_unaligned((base + off + 12) as *mut f32, a);
                    }
                }
            }
            return true;
        }
        for c in n.child.iter_mut() { if walk(c, id) { return true; } }
        false
    }
    walk(root, UIDUMP_TARGET);
}
// ★ scrim_btn 평소 색을 다른 메뉴 회색과 맞춤 (매 프레임 +144 RGBA 덮어쓰기). 호버 슬롯은 안 건드림.
fn fix_scrim_btn_idle(root: &mut Node) {
    fn walk(n: &mut Node, id: &str) -> bool {
        if n.id.as_str() == id {
            unsafe {
                let base = runner_base(n);
                let (r, g, b, a) = SCRIM_BTN_IDLE_RGBA;
                for &off in SCRIM_BTN_IDLE_OFFS.iter() {
                    if writable(base + off, 16) {
                        std::ptr::write_unaligned((base + off) as *mut f32, r);
                        std::ptr::write_unaligned((base + off + 4) as *mut f32, g);
                        std::ptr::write_unaligned((base + off + 8) as *mut f32, b);
                        std::ptr::write_unaligned((base + off + 12) as *mut f32, a);
                    }
                }
            }
            return true;
        }
        for c in n.child.iter_mut() { if walk(c, id) { return true; } }
        false
    }
    walk(root, UIDUMP_TARGET);
}
// ★ 진단: scrim_btn 텍스트 키(+408/+1008) 문자열 + 색 후보 f32(0~1) 덤프 → 텍스트 색 출처 식별
unsafe fn dump_scrim_textinfo(n: &Node, out: &mut String) {
    let base = runner_base(n);
    out.push_str(&format!("scrim_btn runner @ {:#x}\n", base));
    for &off in &[408usize, 1008usize] {
        if !readable(base + off, 16) { out.push_str(&format!("  +{}: unreadable\n", off)); continue; }
        let ptr = std::ptr::read_unaligned((base + off) as *const usize);
        let len = std::ptr::read_unaligned((base + off + 8) as *const usize);
        out.push_str(&format!("  text@+{}: ptr={:#x} len={}\n", off, ptr, len));
        if ptr > 0x10000 && len > 0 && len <= 256 && readable(ptr, len) {
            let sl = std::slice::from_raw_parts(ptr as *const u8, len);
            out.push_str(&format!("    utf8: {:?}\n", String::from_utf8_lossy(sl)));
            let hex: String = sl.iter().map(|b| format!("{:02x} ", b)).collect();
            out.push_str(&format!("    hex : {}\n", hex));
        }
    }
    out.push_str("  [0.0~1.0 사이 f32 (색 후보) @off]:\n");
    let mut off = 0usize;
    while off < 1100 {
        if readable(base + off, 4) {
            let f = f32::from_bits(std::ptr::read_unaligned((base + off) as *const u32));
            if f > 0.0 && f <= 1.0 {
                out.push_str(&format!("    +{:>4} = {:.4}\n", off, f));
            }
        }
        off += 4;
    }
}
fn dump_scrim_textinfo_by_id(n: &Node, id: &str, out: &mut String) -> bool {
    if n.id.as_str() == id { unsafe { dump_scrim_textinfo(n, out); } return true; }
    for c in n.child.iter() { if dump_scrim_textinfo_by_id(c, id, out) { return true; } }
    false
}
// ★ Node 구조체 자체 메모리 덤프 (rect=x,y,w,h 위치 찾기용). &Node 주소에서 raw 읽기.
unsafe fn probe_node_struct(n: &Node, out: &mut String) {
    let base = n as *const Node as usize;
    out.push_str(&format!("\n=== Node struct id={} @ {:#x} (child={}) ===\n",
        n.id.as_str(), base, n.child.len()));
    out.push_str("  off | u64                | u32a     u32b     | f32a     f32b\n");
    let mut off = 0usize;
    while off < 256 {
        if !readable(base + off, 8) { off += 8; continue; }
        let w = std::ptr::read_unaligned((base + off) as *const u64);
        let lo = (w & 0xffff_ffff) as u32;
        let hi = (w >> 32) as u32;
        out.push_str(&format!("  +{:>3} | {:#018x} | {:08x} {:08x} | {:8.2} {:8.2}\n",
            off, w, lo, hi, f32::from_bits(lo), f32::from_bits(hi)));
        off += 8;
    }
}
fn probe_node_struct_by_id(n: &Node, id: &str, out: &mut String) -> bool {
    if n.id.as_str() == id { unsafe { probe_node_struct(n, out); } return true; }
    for c in n.child.iter() { if probe_node_struct_by_id(c, id, out) { return true; } }
    false
}
// ★ Node rect 변경. w=+116, h=+124, x=+132, y=+140 (f32). 복사본 +244/+252.
//   (덤프로 확정. scrim_msg 의 x=30,y=64 가 main.ui 와 일치 확인.)
const RECT_W_OFF: usize = 116;
const RECT_H_OFF: usize = 124;
const RECT_X_OFF: usize = 132;
const RECT_Y_OFF: usize = 140;
const RECT_W2_OFF: usize = 244;
const RECT_H2_OFF: usize = 252;
unsafe fn set_rect_size_node(n: &Node, w: f32, h: f32) {
    let base = n as *const Node as usize;
    if readable(base + RECT_H_OFF, 4) {
        std::ptr::write_unaligned((base + RECT_W_OFF) as *mut f32, w);
        std::ptr::write_unaligned((base + RECT_H_OFF) as *mut f32, h);
    }
    if readable(base + RECT_H2_OFF, 4) {
        std::ptr::write_unaligned((base + RECT_W2_OFF) as *mut f32, w);
        std::ptr::write_unaligned((base + RECT_H2_OFF) as *mut f32, h);
    }
}
fn set_rect_size_by_id(n: &Node, id: &str, w: f32, h: f32) -> bool {
    if n.id.as_str() == id { unsafe { set_rect_size_node(n, w, h); } return true; }
    for c in n.child.iter() { if set_rect_size_by_id(c, id, w, h) { return true; } }
    false
}
// rect 위치(x,y) 변경
unsafe fn set_rect_pos_node(n: &Node, x: f32, y: f32) {
    let base = n as *const Node as usize;
    if readable(base + RECT_Y_OFF, 4) {
        std::ptr::write_unaligned((base + RECT_X_OFF) as *mut f32, x);
        std::ptr::write_unaligned((base + RECT_Y_OFF) as *mut f32, y);
    }
}
fn set_rect_pos_by_id(n: &Node, id: &str, x: f32, y: f32) -> bool {
    if n.id.as_str() == id { unsafe { set_rect_pos_node(n, x, y); } return true; }
    for c in n.child.iter() { if set_rect_pos_by_id(c, id, x, y) { return true; } }
    false
}
// ★ ColorRunner 의 back_color 변경. RGBA(f32) 가 +24(normal)/+120(hover)/+216(active) 3벌.
//   set_back_color 로 3벌 다 같은 색 쓰면 hover 시에도 유지됨.
const COLOR_BACK_OFFS: [usize; 3] = [24, 120, 216]; // normal, hover, active
unsafe fn set_back_color_node(n: &mut Node, r: f32, g: f32, b: f32, a: f32) -> bool {
    if !n.runner.type_name().contains("ColorRunner") { return false; }
    let base = runner_base(n);
    for &o in COLOR_BACK_OFFS.iter() {
        if !readable(base + o, 16) { continue; }
        std::ptr::write_unaligned((base + o + 0) as *mut f32, r);
        std::ptr::write_unaligned((base + o + 4) as *mut f32, g);
        std::ptr::write_unaligned((base + o + 8) as *mut f32, b);
        std::ptr::write_unaligned((base + o + 12) as *mut f32, a);
    }
    true
}
fn set_back_color_by_id(n: &mut Node, id: &str, r: f32, g: f32, b: f32, a: f32) -> bool {
    if n.id.as_str() == id { unsafe { return set_back_color_node(n, r, g, b, a); } }
    for c in n.child.iter_mut() { if set_back_color_by_id(c, id, r, g, b, a) { return true; } }
    false
}
// runner 의 특정 오프셋 String(layout: len@+0 ptr@+8) 을 새 값으로 교체 (leak repoint)
unsafe fn write_runner_str(n: &mut Node, off: usize, s: &str) {
    let base = runner_base(n);
    if !readable(base + off, 16) { return; }
    let buf: &'static [u8] = s.as_bytes().to_vec().leak();
    std::ptr::write_unaligned((base + off) as *mut u64, buf.len() as u64);
    std::ptr::write_unaligned((base + off + 8) as *mut u64, buf.as_ptr() as u64);
}
fn write_runner_str_by_id(n: &mut Node, id: &str, off: usize, s: &str) -> bool {
    if n.id.as_str() == id { unsafe { write_runner_str(n, off, s); } return true; }
    for c in n.child.iter_mut() { if write_runner_str_by_id(c, id, off, s) { return true; } }
    false
}
// ColorIconButton 텍스트 키는 두 곳: +408(표시 복사본), +1008(원본 — 호버 시 +408로 재복사).
// 둘 다 같은 (ptr@off, len@off+8) 레이아웃. 기존 버퍼에 "연습 시작" 제자리 덮어쓰기 + len 갱신.
unsafe fn try_blank_view(n: &mut Node) {
    const OFFS: [usize; 2] = [408, 1008];
    const NEW: &str = "연습 시작";
    let base = runner_base(n);
    for &off_ptr in OFFS.iter() {
        let off_len = off_ptr + 8;
        if !readable(base + off_ptr, 16) { continue; }
        let ptr = std::ptr::read_unaligned((base + off_ptr) as *const usize);
        let len = std::ptr::read_unaligned((base + off_len) as *const usize);
        if ptr <= 0x10000 || len < 1 || len > 256 || !readable(ptr, len) { continue; }
        let cur = match std::str::from_utf8(std::slice::from_raw_parts(ptr as *const u8, len)) {
            Ok(s) => s, Err(_) => continue,
        };
        if cur == NEW { continue; }                  // 이미 적용됨
        if !cur.contains("replay") { continue; }     // 키도 아니고 우리가 바꾼 것도 아니면 skip
        let nb = NEW.as_bytes();
        if nb.len() <= len && writable(ptr, nb.len()) {
            std::ptr::copy_nonoverlapping(nb.as_ptr(), ptr as *mut u8, nb.len());
            std::ptr::write_unaligned((base + off_len) as *mut u64, nb.len() as u64);
        }
    }
}
fn blank_view_by_id(n: &mut Node, id: &str) -> bool {
    if n.id.as_str() == id { unsafe { try_blank_view(n); } return true; }
    for c in n.child.iter_mut() { if blank_view_by_id(c, id) { return true; } }
    false
}
// ── ImageRunner source 경로 (인스턴스 +0=len, +8=ptr) ──
unsafe fn read_img_source(n: &Node) -> Option<String> {
    if !n.runner.type_name().contains("ImageRunner") { return None; }
    let any: &dyn std::any::Any = n.runner.as_any();
    let parts: [usize; 2] =
        std::mem::transmute::<*const dyn std::any::Any, [usize; 2]>(any as *const dyn std::any::Any);
    let dp = parts[0];
    let len = std::ptr::read_unaligned(dp as *const u64) as usize;
    let ptr = std::ptr::read_unaligned((dp + 8) as *const u64) as *const u8;
    if ptr.is_null() || len == 0 || len > 512 { return None; }
    Some(String::from_utf8_lossy(std::slice::from_raw_parts(ptr, len)).to_string())
}
unsafe fn write_img_source(n: &mut Node, s: &str) {
    if !n.runner.type_name().contains("ImageRunner") { return; }
    let any: &dyn std::any::Any = n.runner.as_any();
    let parts: [usize; 2] =
        std::mem::transmute::<*const dyn std::any::Any, [usize; 2]>(any as *const dyn std::any::Any);
    let dp = parts[0];
    let buf: &'static [u8] = s.as_bytes().to_vec().leak(); // leak → UAF 방지
    std::ptr::write_unaligned((dp + 8) as *mut u64, buf.as_ptr() as u64);
    std::ptr::write_unaligned(dp as *mut u64, buf.len() as u64);
}
fn read_img_source_by_id(n: &Node, id: &str) -> Option<String> {
    if n.id.as_str() == id { return unsafe { read_img_source(n) }; }
    for c in n.child.iter() { if let Some(s) = read_img_source_by_id(c, id) { return Some(s); } }
    None
}
fn write_img_source_by_id(n: &mut Node, id: &str, s: &str) -> bool {
    if n.id.as_str() == id { unsafe { write_img_source(n, s); } return true; }
    for c in n.child.iter_mut() { if write_img_source_by_id(c, id, s) { return true; } }
    false
}
// 로고 태그(team_logo 시트 내 프레임명, 예 "7_11")는 ImageRunner 안 +24/+32 와 복사본 +232/+240.
// 두 곳 모두 내 팀 태그로 제자리 덮어쓰기(같은/짧은 길이일 때, 쓰기 가능할 때).
unsafe fn set_logo_tag(n: &mut Node, tag: &str) {
    if !n.runner.type_name().contains("ImageRunner") { return; }
    let base = runner_base(n);
    let nb = tag.as_bytes();
    for &(ol, op, oc) in &[(24usize, 32usize, 40usize), (232usize, 240usize, 248usize)] {
        if !readable(base + ol, 24) { continue; }
        let ptr = std::ptr::read_unaligned((base + op) as *const usize);
        let len = std::ptr::read_unaligned((base + ol) as *const usize);
        let cap = std::ptr::read_unaligned((base + oc) as *const usize);
        if ptr <= 0x10000 || len == 0 || len > 16 || !readable(ptr, len) { continue; }
        let cur = match std::str::from_utf8(std::slice::from_raw_parts(ptr as *const u8, len)) {
            Ok(s) => s, Err(_) => continue,
        };
        if cur == tag { continue; }              // 이미 내 팀 태그
        if !cur.contains('_') { continue; }      // 태그 형식("N_M") 확인 → 엉뚱한 필드 보호
        if nb.len() <= cap && writable(ptr, nb.len()) {
            std::ptr::copy_nonoverlapping(nb.as_ptr(), ptr as *mut u8, nb.len());
            std::ptr::write_unaligned((base + ol) as *mut u64, nb.len() as u64);
        }
    }
}
fn set_logo_tag_by_id(n: &mut Node, id: &str, tag: &str) -> bool {
    if n.id.as_str() == id { unsafe { set_logo_tag(n, tag); } return true; }
    for c in n.child.iter_mut() { if set_logo_tag_by_id(c, id, tag) { return true; } }
    false
}
// 팝업 양쪽 팀 로고를 내 팀 태그로 통일 (내 팀이 좌/우 어디든 둘 다 내 팀 로고)
fn fix_logos(root: &mut Node, my_logo: &str) {
    if my_logo.is_empty() { return; }
    set_logo_tag_by_id(root, "team1_logo", my_logo);
    set_logo_tag_by_id(root, "team2_logo", my_logo);
}
// 텍스트에 from 이 포함된 LabelRunner 를 모두 to 로 교체 (반환: 교체 수)
fn rename_labels(n: &mut Node, from: &str, to: &str) -> usize {
    let mut cnt = 0;
    unsafe {
        if let Some(t) = read_label_node(n) {
            if t.contains(from) && t != to { if set_label_node(n, to) { cnt += 1; } }
        }
    }
    for c in n.child.iter_mut() { cnt += rename_labels(c, from, to); }
    cnt
}
// 텍스트가 정확히 from 인 LabelRunner 만 to 로 (배경의 "승점" 등 오인 방지)
fn rename_exact(n: &mut Node, from: &str, to: &str) -> usize {
    let mut cnt = 0;
    unsafe {
        if let Some(t) = read_label_node(n) {
            if t.trim() == from { if set_label_node(n, to) { cnt += 1; } }
        }
    }
    for c in n.child.iter_mut() { cnt += rename_exact(c, from, to); }
    cnt
}
// 팝업의 모든 라벨(id + 텍스트) 1회 덤프 → 점수/색 정밀 타겟용
fn dump_labels(n: &Node, depth: usize, out: &mut String) {
    unsafe {
        if let Some(t) = read_label_node(n) {
            out.push_str(&format!("{}id={:<24} text={:?}\n", "  ".repeat(depth), n.id.as_str(), t));
        }
    }
    for c in n.child.iter() { dump_labels(c, depth + 1, out); }
}
// 모든 노드(id + runner 종류) 덤프 → 아이콘/버튼(화살표 등) 특정용
fn dump_nodes(n: &Node, depth: usize, out: &mut String) {
    let ty = n.runner.type_name();
    let short = ty.rsplit(':').next().unwrap_or(ty);
    out.push_str(&format!("{}id={:<22} {}\n", "  ".repeat(depth.min(16)), n.id.as_str(), short));
    for c in n.child.iter() { dump_nodes(c, depth + 1, out); }
}
// 다시보기 팝업 정밀 수정: 제목 제거 / 양팀명=내팀(흰색) / 점수 제거 / 승·패 제거
fn fix_replay_popup(n: &mut Node, myname: &str) {
    let id = n.id.as_str().to_owned();
    let cur = unsafe { read_label_node(n) };
    if let Some(t) = cur {
        let new: Option<String> =
            if t.contains("replay.header") { Some(String::new()) }                       // 좌상단 "리플레이"
            else if id == "blue_result" || id == "red_result" { Some(String::new()) }     // 승/패
            else if id == "blue_name" || id == "red_name" { Some(format!("<#ffffffff>{}<>", myname)) }
            else if id == "text" && t.contains(" vs ") && t.contains("set/bold") {         // 매치업+점수 줄
                Some(format!("<#ffffffff>{}  vs  {}<>", myname, myname))
            } else { None };
        if let Some(s) = new { unsafe { set_label_node(n, &s); } }
    }
    for c in n.child.iter_mut() { fix_replay_popup(c, myname); }
}

// ===========================================================================
//  데이터: 첫 연습경기 / 로스터 / 사용가능 챔피언
// ===========================================================================
// 완료된(replays 채워진) 연습경기 중 match_id 최소 → (popup match_id, 주입 key)
fn find_first_practice(r: &ClientDatabase) -> Option<(u64, usize)> {
    let mut cands: Vec<(u64, usize)> = Vec::new();
    for (mt, mi) in r.matches.iter() {
        let mid = match mt { MatchType::Practice { match_id } => *match_id, _ => continue };
        if mi.replays.is_empty() { continue; } // 완료된 연습경기만
        cands.push((mid as u64, mi.replays[0]));
    }
    cands.sort_by_key(|(mid, _)| *mid);
    cands.into_iter().next()
}

// FORCE 우선. 0이면 자동(맨 처음). → (popup match_id, 주입 key=replays[0])
//   ★ 핵심 매핑: practice_match(match_id).replays[0] == match_replays 의 key
//     (popup 은 match_id 로 띄우고, 주입은 그 key 로 한다 = 같은 경기를 가리킴)
fn resolve_practice(r: &ClientDatabase) -> Option<(u64, usize)> {
    if FORCE_PRACTICE_MATCH_ID != 0 {
        let mi = r.practice_match(FORCE_PRACTICE_MATCH_ID as usize)?;
        if mi.replays.is_empty() { return None; }
        return Some((FORCE_PRACTICE_MATCH_ID, mi.replays[0]));
    }
    find_first_practice(r)
}

// 검증 덤프: 모든 Practice 경기의 match_id / replays(key들) / 양팀.
//   기대: 우리 경기 match_id=1176, replays=[0], team1/team2 에 115·117.
fn dump_practice_matches(r: &ClientDatabase) {
    let mut s = format!("[{}ms] db.matches Practice 덤프 (match_id / replays(=match_replays key) / team1 / team2):\n", now_ms());
    let mut rows: Vec<(u64, String)> = Vec::new();
    for (mt, mi) in r.matches.iter() {
        let mid = match mt { MatchType::Practice { match_id } => *match_id as u64, _ => continue };
        let t1 = format!("{:?}", mi.team1);
        let t2 = format!("{:?}", mi.team2);
        rows.push((mid, format!(
            "  match_id={:<8} replays={:?}  team1={}  team2={}",
            mid, mi.replays, t1, t2
        )));
    }
    rows.sort_by_key(|(m, _)| *m);
    for (_, line) in rows { s.push_str(&line); s.push('\n'); }
    // 추가로 현재 선택될 (popup, key) 도 기록
    if let Some((mid, key)) = resolve_practice(r) {
        s.push_str(&format!("=> 선택됨: popup match_id={}  주입 key={}\n", mid, key));
    } else {
        s.push_str("=> 선택 가능한 (완료된) 연습경기 없음\n");
    }
    write_log("scrim_practice_dump.txt", &s);
}

fn parse_usize_field(s: &str, field: &str) -> usize {
    let key = format!("{}: ", field);
    let Some(start) = s.find(&key) else { return usize::MAX; };
    let rest = &s[start + key.len()..];
    let end = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
    rest[..end].parse().unwrap_or(usize::MAX)
}

fn read_roster(r: &ClientDatabase) -> Vec<(usize, String)> {
    let my = r.player_team_id();
    let mut out: Vec<(usize, String)> = Vec::new();
    for (&aid, ath) in r.athletes.iter() {
        let cs = format!("{:?}", ath.contract);
        if !cs.contains("InContract") { continue; }
        if parse_usize_field(&cs, "team_id") != my { continue; }
        out.push((aid, ath.name.clone()));
    }
    out.sort_by(|a, b| a.1.cmp(&b.1));
    out
}

// 런타임 챔피언 표시 이름. ※ChampionInfo 엔 이름 메서드 없음(빌드로 확인) → 진짜 이름 소스
//   (SDK ChampionInfo 메서드 or i18n asset/base/text/champion) 확정 후 교체 예정. 지금은 champ_kr→key.
fn champ_name_dyn(r: &ClientDatabase, key: &str) -> String {
    let _ = r;
    champ_kr(key)
}
// ★ 챔프 이름 = 게임 i18n 참조 문자열. 라벨에 이걸 넣으면 LabelRunner 가 렌더 시점에
//   현지화 이름(모드 챔프 포함)으로 자동 해석함. (게임 자기 name 라벨도 이 형식으로 저장돼 있음)
//   라벨 덤프에서 확인: id=name text="#asset/base/text/champion?description.<key>.name"
const CHAMP_NAME_I18N: bool = true; // true=게임 i18n 자동해석(하드코딩 불필요) / false=champ_kr 폴백
const HIDE_DEAD_CHAMPS: bool = true; // 모드 꺼져 ChampionInfo 없는 챔프(이름 안 풀림)를 후보에서 숨김
fn champ_i18n(key: &str) -> String { format!("#asset/base/text/champion?description.{}.name", key) }
// CHAMPS 캐시에서 표시 이름 조회 (r 없는 곳용 — read_champs 가 채워둠)
fn champ_display(key: &str) -> String {
    let champs = CHAMPS.lock().unwrap();
    champs.iter().find(|(k, _)| k == key).map(|(_, n)| n.clone())
        .unwrap_or_else(|| key.to_string())
}

fn read_champs(r: &ClientDatabase) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    // 1회 로그: available_champions 전체 + champ_kr 가 모르는(=영문 key 그대로) 챔프 표시 → 모드 챔프 식별
    let log = !CHAMP_NAME_DUMP_DONE.swap(true, Ordering::Relaxed);
    let mut s = String::new();
    for key in r.available_champions.iter() {
        let kr = champ_name_dyn(r, key);                 // 정렬·로그용 한글(모르면 key)
        let disp = if CHAMP_NAME_I18N { champ_i18n(key) } else { kr.clone() }; // 표시용
        // ★ 모드 꺼지면 그 챔프의 ChampionInfo 가 사라짐(None) → i18n 도 안 풀려 "description.x.name" 날것 표시.
        //   champion_info 가 살아있는 챔프만 후보에 남김(바닐라는 항상 Some). HIDE_DEAD_CHAMPS 로 토글.
        let alive = r.champion_info(key).is_some();
        if log {
            let unknown = kr == *key; // champ_kr 가 모름 = 하드코딩에 없는 모드 챔프 후보
            s.push_str(&format!("  {:<24} alive={} kr={}  disp={}{}\n", key, alive, kr, disp,
                if unknown { "  ← 하드코딩없음(모드?)" } else { "" }));
        }
        if HIDE_DEAD_CHAMPS && !alive { continue; } // 죽은 모드 챔프(이름 안 풀림) 숨김
        out.push((key.clone(), disp));
    }
    if log {
        write_log("scrim_champ_names.txt",
            &format!("=== available_champions (총 {}개, 표시 {}개) — I18N={} HIDE_DEAD={} ===\n{}",
                r.available_champions.len(), out.len(), CHAMP_NAME_I18N, HIDE_DEAD_CHAMPS, s));
    }
    // 정렬: 표시값이 i18n 참조면 모두 #asset… 으로 시작하므로 한글(champ_kr) 기준으로 정렬
    out.sort_by(|a, b| champ_kr(&a.0).cmp(&champ_kr(&b.0)));
    out
}

// ===========================================================================
//  중복 없는 순환 선택 (클릭 = 다음 사용가능 후보)
// ===========================================================================
fn cycle_player(slot: usize) {
    let roster = ROSTER.lock().unwrap();
    if roster.is_empty() { return; }
    let mut slots = PLAYER_SLOTS.lock().unwrap();
    let taken: Vec<usize> = slots.iter().enumerate()
        .filter(|(i, _)| *i != slot).filter_map(|(_, v)| *v).collect();
    let cur = slots[slot].and_then(|aid| roster.iter().position(|(id, _)| *id == aid))
        .map(|i| i as i64).unwrap_or(-1);
    let n = roster.len() as i64;
    for step in 1..=n {
        let idx = (cur + step).rem_euclid(n) as usize;
        if !taken.contains(&roster[idx].0) { slots[slot] = Some(roster[idx].0); return; }
    }
}
fn cycle_champ(slot: usize) {
    let champs = CHAMPS.lock().unwrap();
    if champs.is_empty() { return; }
    let mut slots = CHAMP_SLOTS.lock().unwrap();
    let taken: Vec<String> = slots.iter().enumerate()
        .filter(|(i, _)| *i != slot).filter_map(|(_, v)| v.clone()).collect();
    let cur = slots[slot].as_ref().and_then(|k| champs.iter().position(|(key, _)| key == k))
        .map(|i| i as i64).unwrap_or(-1);
    let n = champs.len() as i64;
    for step in 1..=n {
        let idx = (cur + step).rem_euclid(n) as usize;
        if !taken.contains(&champs[idx].0) { slots[slot] = Some(champs[idx].0.clone()); return; }
    }
}
fn all_filled() -> bool {
    PLAYER_SLOTS.lock().unwrap().iter().all(|x| x.is_some())
        && CHAMP_SLOTS.lock().unwrap().iter().all(|x| x.is_some())
}

// ===========================================================================
//  드롭다운(풀다운): 클릭한 슬롯에 맞춰 목록을 채우고 펼침
// ===========================================================================
fn open_dropdown(root: &mut Node, kind: usize, slot: usize) {
    DD_KIND.store(kind, Ordering::Relaxed);
    DD_SLOT.store(slot, Ordering::Relaxed);
    DD_PAGE.store(0, Ordering::Relaxed);
    // 후보 구성 (★이미 고른 값도 계속 표시. 중복 선택 시 select_dropdown 에서 기존 슬롯을 비움)
    let mut items: Vec<(Option<usize>, Option<String>, String)> = Vec::new();
    if kind == 0 {
        let roster = ROSTER.lock().unwrap();
        for (aid, name) in roster.iter() {
            items.push((Some(*aid), None, name.clone()));
        }
        set_label_by_id(root, "scrim_ddp_head", "선수 선택");
    } else {
        let champs = CHAMPS.lock().unwrap();
        for (key, kr) in champs.iter() {
            items.push((None, Some(key.clone()), kr.clone()));
        }
        set_label_by_id(root, "scrim_ddc_head", "챔피언 선택");
    }
    *DD_ITEMS.lock().unwrap() = items;
    DD_OPEN.store(true, Ordering::Relaxed);
    fill_page(root);
}

// 현재 페이지의 항목으로 행 라벨 채우기 + 남는 행/페이지 버튼 처리
fn fill_page(root: &mut Node) {
    let prefix = dd_prefix();
    let rows = dd_rows();
    let page = DD_PAGE.load(Ordering::Relaxed);
    let items = DD_ITEMS.lock().unwrap();
    let len = items.len();
    let pages = ((len + rows - 1) / rows).max(1);
    // 이미 어느 슬롯에든 선택된 선수/챔프 집합 (마커 표시용)
    let kind = DD_KIND.load(Ordering::Relaxed);
    let taken_p: Vec<usize> = if kind == 0 {
        PLAYER_SLOTS.lock().unwrap().iter().filter_map(|x| *x).collect()
    } else { Vec::new() };
    let taken_c: Vec<String> = if kind == 1 {
        CHAMP_SLOTS.lock().unwrap().iter().filter_map(|x| x.clone()).collect()
    } else { Vec::new() };
    for i in 0..rows {
        let idx = page * rows + i;
        if idx < len {
            let (pid, ckey, name) = &items[idx];
            // 이미 선택된 항목인지
            let chosen = match (pid, ckey) {
                (Some(p), _) => taken_p.contains(p),
                (_, Some(k)) => taken_c.contains(k),
                _ => false,
            };
            // 색으로만 선택 구분 (마커 제거)
            let box_id = format!("{}_{:02}_box", prefix, i);
            set_label_by_id(root, &format!("{}_{:02}_t", prefix, i), name);
            if chosen {
                // 선택됨: 파란빛 강조 (#2e5d8c 정도)
                set_back_color_by_id(root, &box_id, 0.18, 0.36, 0.55, 1.0);
            } else {
                // 기본색 복원 (#1d1f2c)
                set_back_color_by_id(root, &box_id, 0.114, 0.122, 0.173, 1.0);
            }
            set_visible_by_id(root, &box_id, true);
        } else {
            set_visible_by_id(root, &format!("{}_{:02}_box", prefix, i), false);
        }
    }
    set_visible_by_id(root, &format!("{}_up", prefix), page > 0);
    set_visible_by_id(root, &format!("{}_dn", prefix), page + 1 < pages);
    set_label_by_id(root, &format!("{}_pg", prefix), &format!("{} / {}", page + 1, pages));
}

fn dd_page_delta(d: i64) {
    let pages = dd_pages() as i64;
    let p = (DD_PAGE.load(Ordering::Relaxed) as i64 + d).clamp(0, pages - 1);
    DD_PAGE.store(p as usize, Ordering::Relaxed);
}

// 드롭다운 행 선택 (gi = 화면상 행 인덱스) → 활성 슬롯에 적용
fn select_dropdown(gi: usize) {
    let rows = dd_rows();
    let idx = DD_PAGE.load(Ordering::Relaxed) * rows + gi;
    let items = DD_ITEMS.lock().unwrap();
    if let Some((pid, ckey, _)) = items.get(idx) {
        let slot = DD_SLOT.load(Ordering::Relaxed);
        if DD_KIND.load(Ordering::Relaxed) == 0 {
            if let Some(p) = pid {
                let mut ps = PLAYER_SLOTS.lock().unwrap();
                // ★ 같은 선수가 다른 슬롯에 있으면 그 슬롯을 공백으로 (한 선수 = 한 슬롯)
                for j in 0..10 { if j != slot && ps[j] == Some(*p) { ps[j] = None; } }
                ps[slot] = Some(*p);
            }
        } else if let Some(k) = ckey {
            let mut cs = CHAMP_SLOTS.lock().unwrap();
            // ★ 같은 챔프가 다른 슬롯에 있으면 그 슬롯을 공백으로
            for j in 0..10 { if j != slot && cs[j].as_deref() == Some(k.as_str()) { cs[j] = None; } }
            cs[slot] = Some(k.clone());
        }
    }
    drop(items);
    save_selection(); // 선택 즉시 저장 → 창 닫아도/재시작해도 유지
    DD_OPEN.store(false, Ordering::Relaxed);
}

// ===========================================================================
//  라벨 갱신
// ===========================================================================
// 전술 모달 값 라벨 갱신 (working 기준)
fn repaint_strat(root: &mut Node) {
    let w = *STRAT_WORKING.lock().unwrap();
    let team = STRAT_VIEW_TEAM.load(Ordering::Relaxed) as usize;
    // 현재 보는 팀의 12필드를 공용 박스(scrim_st_<key>)에 표시
    for f in 0..12 {
        let opts = STRAT_OPTS[f];
        let idx = (w[team][f] as usize).min(opts.len() - 1);
        set_label_by_id(root, &format!("scrim_st_{}_t", SKEYS[f]), opts[idx]);
    }
    // 팀 토글 버튼: 텍스트 + 색 박스(파랑/빨강) visible 전환
    if team == 0 {
        set_label_by_id(root, "scrim_st_team_t", "블루팀 ▼");
        set_visible_by_id(root, "scrim_st_team_blue", true);
        set_visible_by_id(root, "scrim_st_team_red", false);
    } else {
        set_label_by_id(root, "scrim_st_team_t", "레드팀 ▼");
        set_visible_by_id(root, "scrim_st_team_blue", false);
        set_visible_by_id(root, "scrim_st_team_red", true);
    }
    // 스플릿 담당 행 표시/숨김 + 담당 포지션 라벨
    const POS_KR: [&str; 5] = ["탑", "정글", "미드", "바텀", "서포터"];
    let sp = STRAT_SPLIT_POS.lock().unwrap();
    let (bld_pos, mor_a, mor_b) = sp[team];
    drop(sp);
    // bld(SKEYS[4]): working idx 2 = 스플릿 → 담당1행 표시
    let bld_split = w[team][4] == 2;
    set_visible_by_id(root, "scrim_sp_bld_1_row", bld_split);
    if bld_split {
        set_label_by_id(root, "scrim_sp_bld_1_t", POS_KR[(bld_pos as usize).min(4)]);
    }
    // mor(SKEYS[6]): working idx 1=1-4(담당1줄), 2=1-3-1(담당2줄)
    let mor_v = w[team][6];
    // 1-3-1인데 두 담당이 같으면 둘째를 다른 포지션으로 보정
    if mor_v == 2 && mor_a == mor_b {
        let mut sp2 = STRAT_SPLIT_POS.lock().unwrap();
        sp2[team].2 = (mor_a + 1) % 5;
        drop(sp2);
    }
    let sp = STRAT_SPLIT_POS.lock().unwrap();
    let (_, mor_a, mor_b) = sp[team];
    drop(sp);
    set_visible_by_id(root, "scrim_sp_mor_1_row", mor_v >= 1);
    set_visible_by_id(root, "scrim_sp_mor_2_row", mor_v == 2);
    if mor_v >= 1 {
        set_label_by_id(root, "scrim_sp_mor_1_t", POS_KR[(mor_a as usize).min(4)]);
    }
    if mor_v == 2 {
        set_label_by_id(root, "scrim_sp_mor_2_t", POS_KR[(mor_b as usize).min(4)]);
    }
}
// 아이템 모달 값/이름 라벨 갱신 (working 기준 + 라인업 챔프명)
fn repaint_items(root: &mut Node) {
    let w = *ITEMS_WORKING.lock().unwrap();
    let cs = CHAMP_SLOTS.lock().unwrap();
    for slot in 0..10 {
        let pre = if slot < 5 { "scrim_ib_" } else { "scrim_ir_" };
        for s in 0..3 {
            let idx = (w[slot][s] as usize).min(ITEM_OPTS.len() - 1);
            set_label_by_id(root, &format!("{}{}_{}_t", pre, slot, s), ITEM_OPTS[idx]);
        }
        let nm = cs[slot].as_ref().map(|k| champ_display(k)).unwrap_or_else(|| "(선택)".to_string());
        set_label_by_id(root, &format!("{}{}_name", pre, slot), &nm);
    }
}
fn refresh_labels(root: &mut Node) {
    let roster = ROSTER.lock().unwrap();
    let champs = CHAMPS.lock().unwrap();
    let pslots = PLAYER_SLOTS.lock().unwrap();
    let cslots = CHAMP_SLOTS.lock().unwrap();
    for i in 0..10 {
        let ptxt = pslots[i]
            .and_then(|aid| roster.iter().find(|(id, _)| *id == aid).map(|(_, nm)| nm.clone()))
            .unwrap_or_else(|| "(선택)".to_string());
        let ctxt = cslots[i].as_ref()
            .and_then(|k| champs.iter().find(|(key, _)| key == k).map(|(_, nm)| nm.clone()))
            .unwrap_or_else(|| "(선택)".to_string());
        // 클릭 버튼은 scrim_pN/scrim_cN, 값 표시 라벨은 scrim_pN_t/scrim_cN_t (이름만)
        set_label_by_id(root, &format!("scrim_p{}_t", i), &ptxt);
        set_label_by_id(root, &format!("scrim_c{}_t", i), &ctxt);
    }
    let ready = pslots.iter().all(|x| x.is_some()) && cslots.iter().all(|x| x.is_some());
    set_disabled_by_id(root, "scrim_start", !ready);
}

// ===========================================================================
//  주입: match_replays[key] 슬롯 base
// ===========================================================================
unsafe fn scrim_slots(r: &ClientDatabase, key: usize) -> Option<(usize, [usize; 10])> {
    let rep = r.match_replays.get(&key)?;
    let base = addr_of(rep);
    let bp = rd_u64(base + O_BLUE_TEAM + 8);
    let bl = rd_u64(base + O_BLUE_TEAM + 16);
    let rp = rd_u64(base + O_RED_TEAM + 8);
    let rl = rd_u64(base + O_RED_TEAM + 16);
    if bl != 5 || rl != 5 || !looks_heap(bp) || !looks_heap(rp) { return None; }
    let mut slots = [0usize; 10];
    for i in 0..5 { slots[i] = bp as usize + i * ATH_STRIDE; }
    for i in 0..5 { slots[5 + i] = rp as usize + i * ATH_STRIDE; }
    Some((base, slots))
}
unsafe fn capture_backup(key: usize, base: usize, slots: &[usize; 10]) {
    // 이미 백업돼 있으면(경기재생 시 1회 떠놨으면) 덮어쓰지 않음 = 항상 원본 보존
    if BACKUP.lock().unwrap().is_some() { return; }
    let mut bak = Backup {
        key, seed: rd_u64(base + O_SEED), slots: Vec::with_capacity(10),
        blue_ban: VecBak { cap: rd_u64(base + O_BLUE_BAN), ptr: rd_u64(base + O_BLUE_BAN + 8), len: rd_u64(base + O_BLUE_BAN + 16) },
        red_ban:  VecBak { cap: rd_u64(base + O_RED_BAN),  ptr: rd_u64(base + O_RED_BAN + 8),  len: rd_u64(base + O_RED_BAN + 16) },
        blue_tid: rd_u64(base + O_BLUE_TEAM_ID),
        red_tid:  rd_u64(base + O_RED_TEAM_ID),
        blue_strat: { let mut a = [0u8; 24]; for i in 0..24 { a[i] = *((base + O_BLUE_STRAT + i) as *const u8); } a },
        red_strat:  { let mut a = [0u8; 24]; for i in 0..24 { a[i] = *((base + O_RED_STRAT + i) as *const u8); } a },
    };
    for &slot in slots.iter() {
        let iptr = rd_u64(slot + AO_ITEMS + 8) as usize;
        let ilen = rd_u64(slot + AO_ITEMS + 16);
        let mut items = [0u64; 3];
        if iptr != 0 && readable(iptr, 24) { for s in 0..3 { items[s] = rd_u64(iptr + s*8); } }
        bak.slots.push(SlotBak {
            aid: rd_u64(slot + AO_ATHLETE_ID),
            cap: rd_u64(slot + AO_CHAMPION),
            ptr: rd_u64(slot + AO_CHAMPION + 8),
            len: rd_u64(slot + AO_CHAMPION + 16),
            pos: rd_u64(slot + AO_POSITION),
            item_len: ilen,
            items,
        });
    }
    *BACKUP.lock().unwrap() = Some(bak);
    // 전술 24B 덤프(레이아웃 확정용, 1회)
    let hex = |off: usize| (0..24).map(|i| format!("{:02x}", *((base + off + i) as *const u8)))
        .collect::<Vec<_>>().join(" ");
    append_log("scrim_strategy.txt", &format!(
        "[{}ms] blue@+0x78: {}\n        red@+0x90: {}\n", now_ms(), hex(O_BLUE_STRAT), hex(O_RED_STRAT)));
    STRAT_APPLIED_LOGGED.store(false, Ordering::Relaxed); // 주입 덤프 1회 허용
}
// ===========================================================================
//  B방식 신경망 직접 호출 — 수동칸 고려해서 자동칸 채우기
// ===========================================================================
// 신경망 forward: 빌드(아이템 조합) 전체의 점수 반환 (높을수록 좋음)
//   net=item_network, ctx=11개u64 컨텍스트, build=아이템id 배열, flag=0(결정론적)
// 신경망 forward 호출. 게임 함수 FUN_1419a4c00 을 그대로 호출 (재구현 아님).
//   ★ 함수 주소 = module_base + ITEMNET_FORWARD_RVA. item_network 데이터주소(net) 아님!
//     (과거 크래시 원인: net을 함수로 transmute해 데이터를 코드로 실행했음.)
//   시그니처: fn(item_net, ctx_ptr, build_ptr, build_len, flag) -> f32 (시그모이드 0~1)
//   flag=0 → 결정론적(노이즈 없음). ctx = [0..5]팀champ, [5..10]상대(9999=없음), [10]포지션.
type ItemNetFn = unsafe extern "C" fn(usize, usize, *const u64, u64, u8) -> f32;
const ITEMNET_FORWARD_RVA: usize = 0x19a4c00; // FUN_1419a4c00 - image_base(0x140000000)
unsafe fn itemnet_forward(net: usize, ctx: &[u64; 11], build: &[u64]) -> f32 {
    let mbase = GetModuleHandleW(core::ptr::null()) as usize;
    let func: ItemNetFn = core::mem::transmute(mbase + ITEMNET_FORWARD_RVA);
    func(net, ctx.as_ptr() as usize, build.as_ptr(), build.len() as u64, 0)
}
// ════════════════════════════════════════════════════════════════
//  ItemSetting JSON 동적 카탈로그  (하드코딩 cat_to_id/id_to_cat/cands/item_name 대체)
//  - DLL 옆 "item_setting.json"(없으면 "item_setting.item_setting") 을 read_text 로 읽어 파싱.
//  - ID = 파일 등장 순서 (= JSON 0~29: AD,AttackSpeed,Defense,MagicResistance,Magic,Hp 각 5티어).
//  - 최종템(next_tier 빈것) ID 를 카테고리별로 뽑아 UI 순서로 정렬 → cands/cat_to_id.
//  - 파일 없거나 파싱 실패 시 전부 기존 하드코딩 값으로 폴백 → 바닐라 출력/동작 불변.
// ════════════════════════════════════════════════════════════════
#[derive(Clone)]
struct ItemMeta { id: usize, key: String, category: String, tier: i64, is_final: bool }
static ITEM_CATALOG: Mutex<Option<Vec<ItemMeta>>> = Mutex::new(None);
static CATALOG_TRIED: AtomicBool = AtomicBool::new(false);

enum JsonValue {
    Null, Bool(bool), Num(f64), Str(String),
    Arr(Vec<JsonValue>), Obj(Vec<(String, JsonValue)>),
}
impl JsonValue {
    fn as_obj(&self) -> Option<&Vec<(String, JsonValue)>> {
        if let JsonValue::Obj(o) = self { Some(o) } else { None }
    }
    fn get<'b>(&'b self, key: &str) -> Option<&'b JsonValue> {
        self.as_obj()?.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
    fn as_str(&self) -> Option<&str> {
        if let JsonValue::Str(s) = self { Some(s.as_str()) } else { None }
    }
    fn as_i64(&self) -> Option<i64> {
        if let JsonValue::Num(n) = self { Some(*n as i64) } else { None }
    }
    fn arr_len(&self) -> Option<usize> {
        if let JsonValue::Arr(a) = self { Some(a.len()) } else { None }
    }
}
struct JsonParser<'a> { b: &'a [u8], i: usize }
impl<'a> JsonParser<'a> {
    fn new(s: &'a str) -> Self { JsonParser { b: s.as_bytes(), i: 0 } }
    fn skip_ws(&mut self) {
        while self.i < self.b.len() {
            match self.b[self.i] {
                b' ' | b'\t' | b'\r' | b'\n' | b',' => self.i += 1,
                _ => break,
            }
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
                        b'n' => out.push(b'\n'),
                        b't' => out.push(b'\t'),
                        b'r' => out.push(b'\r'),
                        b'b' => out.push(0x08),
                        b'f' => out.push(0x0c),
                        b'"' => out.push(b'"'),
                        b'\\' => out.push(b'\\'),
                        b'/' => out.push(b'/'),
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
            match self.b[self.i] {
                b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E' => self.i += 1,
                _ => break,
            }
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
fn item_meta_from(id: usize, fallback_key: Option<&str>, v: &JsonValue) -> Option<ItemMeta> {
    v.as_obj()?;
    let key = v.get("key").and_then(|x| x.as_str()).map(|s| s.to_string())
        .or_else(|| fallback_key.map(|s| s.to_string()))?;
    let category = v.get("category").and_then(|x| x.as_str()).unwrap_or("").to_string();
    let tier = v.get("tier").and_then(|x| x.as_i64()).unwrap_or(-1);
    let is_final = v.get("next_tier").and_then(|x| x.arr_len()).map(|n| n == 0).unwrap_or(true);
    Some(ItemMeta { id, key, category, tier, is_final })
}
fn load_item_catalog() -> Option<Vec<ItemMeta>> {
    let txt = read_text("item_setting.json").or_else(|| read_text("item_setting.item_setting"))?;
    let txt = txt.trim_start_matches('\u{feff}');
    let root = JsonParser::new(txt).parse_value()?;
    let obj = root.as_obj()?;
    let mut out: Vec<ItemMeta> = Vec::new();
    let mut id = 0usize;
    for (k, v) in obj {
        if k == "mod_items" {
            if let JsonValue::Arr(items) = v {
                for it in items {
                    if let Some(meta) = item_meta_from(id, None, it) { out.push(meta); id += 1; }
                }
            }
            continue;
        }
        if let Some(meta) = item_meta_from(id, Some(k), v) { out.push(meta); id += 1; }
    }
    if out.is_empty() { None } else { Some(out) }
}
fn ensure_catalog() {
    if CATALOG_TRIED.swap(true, Ordering::Relaxed) { return; }
    if let Some(c) = load_item_catalog() { *ITEM_CATALOG.lock().unwrap() = Some(c); }
}
fn cat_kor(cat: &str) -> &'static str {
    match cat {
        "AD" => "공격력", "AttackSpeed" => "공속", "Defense" => "방어력",
        "MagicResistance" => "마저", "Magic" => "주문력", "Hp" => "체력", _ => "",
    }
}
// UI 카테고리(ITEM_OPTS: 1=공격력 2=주문력 3=공속 4=방어력 5=마저 6=체력) → JSON 카테고리
const UI_CAT_TO_JSON: [&str; 6] = ["AD", "Magic", "AttackSpeed", "Defense", "MagicResistance", "Hp"];
// UI 순서 6개 카테고리의 '최종템(next_tier 빈것)' ID. 카탈로그 없으면 하드코딩 폴백.
fn final_ids() -> [u64; 6] {
    let fb: [u64; 6] = [4, 24, 9, 14, 19, 29];
    ensure_catalog();
    let guard = ITEM_CATALOG.lock().unwrap();
    let Some(cat) = guard.as_ref() else { return fb; };
    let mut out = fb;
    for (i, jc) in UI_CAT_TO_JSON.iter().enumerate() {
        if let Some(m) = cat.iter().find(|m| m.is_final && m.category == *jc) {
            out[i] = m.id as u64;
        }
    }
    out
}

// 카테고리(1~6) → 5티어 아이템 ID
fn cat_to_id(cat: u8) -> Option<u64> {
    if cat == 0 || cat > 6 { return None; }
    Some(final_ids()[(cat - 1) as usize])
}
// 아이템 ID → 카테고리 (역변환, self_item 빌드 구성용)
#[allow(dead_code)]
fn id_to_cat(id: u64) -> u8 {
    let f = final_ids();
    for (i, &fid) in f.iter().enumerate() { if fid == id { return (i + 1) as u8; } }
    0
}
// ★ B방식: 수동칸 고정 + 자동칸을 신경망이 (수동칸 고려해서) 채움.
//   commit[3] = 각 칸 카테고리 (0=자동, 1~6=수동). champ_id = 우리 챔프 시트인덱스.
//   team_ids[5] = 우리팀 5포지션 champ id, pos = 이 슬롯 포지션(0~4).
//   반환: 최종 3칸 아이템 ID (수동=지정, 자동=신경망 최선).
// ★ B방식: 자동칸을 forward 전체조합 완전탐색으로 채움 (수동칸 고려).
//   commit[3] = 각 칸 카테고리 (0=자동, 1~6=수동). team_ids[5]=우리팀 champ, counters[5]=상대.
//   각 칸 후보(수동=1개, 자동=6개)의 모든 조합을 forward 점수 → 최고점 빌드 반환.
unsafe fn compute_build_b(net: usize, team_ids: &[u64; 5], counters: &[u64; 5], pos: usize, commit: &[u8; 3]) -> Option<[u64; 3]> {
    if net == 0 || !readable(net, 0x20) { return None; }
    // ctx 구성: [0..5]=팀 champ id, [5..10]=카운터, [10]=현재 포지션
    let mut ctx = [0u64; 11];
    for i in 0..5 { ctx[i] = team_ids[i]; ctx[5 + i] = counters[i]; }
    ctx[10] = pos as u64;
    let cands: [u64; 6] = final_ids(); // 공격,주문,공속,방어,마저,체력 5티어 (JSON 동적, 없으면 폴백)
    // 각 칸의 후보 목록: 수동칸 = 그 1개, 자동칸 = 6개 전부
    let slot_cands: [Vec<u64>; 3] = [
        if commit[0] != 0 { vec![cat_to_id(commit[0]).unwrap_or(4)] } else { cands.to_vec() },
        if commit[1] != 0 { vec![cat_to_id(commit[1]).unwrap_or(4)] } else { cands.to_vec() },
        if commit[2] != 0 { vec![cat_to_id(commit[2]).unwrap_or(4)] } else { cands.to_vec() },
    ];
    // ★ 전체조합 완전탐색: 모든 (c0,c1,c2) 조합을 forward 로 평가 → 최고점 빌드 채택
    let mut best_build: [u64; 3] = [0, 0, 0];
    let mut best_score = f32::MIN;
    for &c0 in slot_cands[0].iter() {
        for &c1 in slot_cands[1].iter() {
            for &c2 in slot_cands[2].iter() {
                let trial: [u64; 3] = [c0, c1, c2];
                let score = itemnet_forward(net, &ctx, &trial);
                if score > best_score { best_score = score; best_build = trial; }
            }
        }
    }
    Some(best_build)
}
// 라인업(athlete_id/champion) 주입 + 블루/레드 밴은 빈 값으로(len=0). seed 는 원본 유지.
unsafe fn apply_lineup(base: usize, slots: &[usize; 10]) {
    let pslots = PLAYER_SLOTS.lock().unwrap();
    let cslots = CHAMP_SLOTS.lock().unwrap();
    // ── 자가진단(1회): ITEM_NET_ADDR 가 유효한 신경망 객체인지 확인 ──
    //    item_network 구조 시그니처: +0x00=16384(cap), +0x10=16384(len), +0x18=1(fdim).
    //    게임 업데이트로 오프셋(0xda0)이나 구조가 바뀌면 여기서 "무효"로 찍힘 → 재탐색 신호.
    let _net = ITEM_NET_ADDR.load(Ordering::Relaxed);
    if !ITEMNET_VERIFY_LOGGED.swap(true, Ordering::Relaxed) {
        let mut s = format!("[{}ms] ITEM_NET_ADDR 자가진단\n  ITEM_NET_ADDR = {:#x}\n", now_ms(), _net);
        if _net != 0 && readable(_net, 0x20) {
            let (c0, len, fdim) = (rd_u64(_net), rd_u64(_net + 0x10), rd_u64(_net + 0x18));
            s.push_str(&format!("  cap={} len={} fdim={}\n", c0, len, fdim));
            s.push_str(if c0 == 16384 && len == 16384 && fdim == 1 {
                "  ★유효 (신경망 구조 정상)\n"
            } else {
                "  ✗무효 — 구조 불일치. 업데이트로 오프셋 바뀐듯 → 문서의 '재탐색' 절차 참고\n"
            });
        } else {
            s.push_str("  ✗ NOT readable — 주소 무효 → 재탐색 필요\n");
        }
        append_log("scrim_itemnet_client_verify.txt", &s);
    }
    let mut champ_ids = [0u64; 10];
    for i in 0..10 {
        champ_ids[i] = cslots[i].as_ref().and_then(|k| champ_id(k)).map(|x| x as u64).unwrap_or(0);
    }
    let commits = *ITEMS_COMMITTED.lock().unwrap();
    // ★ 캐시 키 = 라인업(champ id) + 커밋(수동설정) 의 해시. 바뀔 때만 재계산.
    let cache_key = {
        let mut k = 0xcbf29ce484222325u64; // FNV offset
        for &c in champ_ids.iter() { k = (k ^ c).wrapping_mul(0x100000001b3); }
        for row in commits.iter() { for &b in row.iter() { k = (k ^ b as u64).wrapping_mul(0x100000001b3); } }
        k
    };
    // 키가 바뀌었으면 = 라인업/설정 변경 → 빌드 1회 재계산
    if BUILD_CACHE_KEY.load(Ordering::Relaxed) != cache_key {
        let mut cache = BUILD_CACHE.lock().unwrap();
        let net = ITEM_NET_ADDR.load(Ordering::Relaxed);
        for i in 0..10 {
            let ib = commits[i];
            if ib.iter().all(|&b| b == 0) {
                // 전부 자동 = forward 로 3칸 다 계산 (수동 없음)
            } else if !ib.iter().any(|&b| b == 0) {
                // 전부 수동 = 그대로 5티어ID
                let mut a = [0u64; 3];
                for s in 0..3 { if let Some(id) = cat_to_id(ib[s]) { a[s] = id; } }
                cache[i] = a;
                continue;
            }
            // ★ B방식: 자동칸을 forward 최선으로 채움 (수동칸 고려, 순차)
            //   team_ids = 우리팀 5명, counters = 상대팀 5명, pos = i%5
            //   슬롯 0~4 = 블루, 5~9 = 레드. 상대 = 반대팀.
            let pos = i % 5;
            let (my_off, opp_off) = if i < 5 { (0usize, 5usize) } else { (5usize, 0usize) };
            let mut team_ids = [0u64; 5];
            let mut counters = [0u64; 5];
            for p in 0..5 {
                team_ids[p] = champ_ids[my_off + p];
                // 상대 champ id. 0(빈슬롯)이면 9999(없음)로
                let c = champ_ids[opp_off + p];
                counters[p] = if c == 0 { 9999 } else { c };
            }
            if net != 0 {
                if let Some(b) = compute_build_b(net, &team_ids, &counters, pos, &ib) {
                    cache[i] = b;
                } else {
                    // 실패시 수동칸만
                    let mut a = [0u64; 3];
                    for s in 0..3 { if let Some(id) = cat_to_id(ib[s]) { a[s] = id; } }
                    cache[i] = a;
                }
            } else {
                let mut a = [0u64; 3];
                for s in 0..3 { if let Some(id) = cat_to_id(ib[s]) { a[s] = id; } }
                cache[i] = a;
            }
        }
        drop(cache);
        BUILD_CACHE_KEY.store(cache_key, Ordering::Relaxed);
    }
    let build_cache = *BUILD_CACHE.lock().unwrap();
    for i in 0..10 {
        if let Some(aid) = pslots[i] { wr_u64(slots[i] + AO_ATHLETE_ID, aid as u64); }
        if let Some(key) = cslots[i].as_ref() {
            let cur = read_str(rd_u64(slots[i] + AO_CHAMPION + 8), 24);
            if &cur != key { champ_repoint(slots[i], AO_CHAMPION, key); } // 매 프레임 leak 방지
        }
        // 포지션 강제: 슬롯 0~4 = Top,Jungle,Mid,Bottom,Support (블루/레드 동일)
        wr_u64(slots[i] + AO_POSITION, (i % 5) as u64);
        // ★ B방식: 자동칸도 forward 로 채운 값이 cache 에 있음. 3칸 다 써넣기.
        //   len 안 줄임 (벡터제약 없음 = 가운데 자동도 OK).
        let mut b = build_cache[i];
        // ★★ 임시테스트3: 슬롯마다 모드영역 연속ID. slot0:[30,31,32] slot1:[33,34,35] ... slot9:[57,58,59].
        //   재생화면서 각 선수 아이템 보면 어느 ID 가 유효(모드템)/무효(0)인지 한눈에.
        const FORCE_MOD_TEST: bool = false;
        if FORCE_MOD_TEST {
            let g = 30u64 + (i as u64)*3;   // slot0:30,31,32  slot1:33,34,35 ...
            b = [g, g+1, g+2];
        }
        if b.iter().any(|&x| x != 0) {
            let iptr = rd_u64(slots[i] + AO_ITEMS + 8) as usize;
            let ilen = rd_u64(slots[i] + AO_ITEMS + 16) as usize;
            if iptr != 0 && ilen >= 3 && writable(iptr, ilen * 8) {
                for s in 0..3 { if b[s] != 0 { wr_u64(iptr + s * 8, b[s]); } }
            }
        }
    }
    // 밴 비우기: Vec len 만 0 으로 (cap/ptr 보존 → 복원 시 그대로 되살림, free 안 함)
    wr_u64(base + O_BLUE_BAN + 16, 0);
    wr_u64(base + O_RED_BAN + 16, 0);
    // 양팀 id 를 내 팀으로 → 팝업에 양쪽 다 내 팀명 표시 (내전)
    let my = MY_TEAM_ID.load(Ordering::Relaxed);
    if my != u64::MAX {
        wr_u64(base + O_BLUE_TEAM_ID, my);
        wr_u64(base + O_RED_TEAM_ID, my);
    }
    // ★전술 주입: UI committed 값을 24B 에 씀(blue=team0, red=team1).
    //   확정 매핑(2026-06-13): 단순필드 10개는 disc 그대로. bld/mor 는 평탄화.
    //   bld(byte0): UI[0모이기,1유연,2스플릿] → byte[5모이기,6유연,스플릿=담당pos0~4]
    //   mor(byte4): UI[0모이기,1=1-4,2=1-3-1] → byte4[5모이기,6=1-4,1-3-1=첫담당] + byte8=둘째담당
    //   ※담당 포지션 선택 UI 아직 없음 → 스플릿이면 담당 기본=탑(0) 으로 주입.
    {
        let sc = STRAT_COMMITTED.lock().unwrap();
        let sp = STRAT_SPLIT_POS.lock().unwrap(); // [team] = (bld담당, mor첫담당, mor둘째담당)
        for team in 0..2 {
            for f in 0..12 {
                if f == 4 || f == 6 { continue; } // bld, mor 는 아래서 따로
                *((base + (if team==0 {O_BLUE_STRAT} else {O_RED_STRAT}) + STRAT_OFFS[f]) as *mut u8) = sc[team][f];
            }
            let so = if team == 0 { O_BLUE_STRAT } else { O_RED_STRAT };
            // bld (byte0)
            let bld = sc[team][4];
            let (bld_pos, mor_a, mor_b) = sp[team];
            let bld_byte = match bld { 0 => 5u8, 1 => 6u8, _ => bld_pos.min(4) }; // 스플릿→담당pos
            *((base + so + 0) as *mut u8) = bld_byte;
            // mor (byte4 + byte8)
            let mor = sc[team][6];
            match mor {
                0 => { *((base + so + 4) as *mut u8) = 5; *((base + so + 8) as *mut u8) = 0; } // 모이기
                1 => { *((base + so + 4) as *mut u8) = 6; *((base + so + 8) as *mut u8) = mor_a.min(4); } // 1-4@담당
                _ => { *((base + so + 4) as *mut u8) = mor_a.min(4); *((base + so + 8) as *mut u8) = mor_b.min(4); } // 1-3-1
            }
        }
    }
    // 주입된 24B 1회 덤프(매핑 확정용; capture 때 플래그 리셋됨)
    if !STRAT_APPLIED_LOGGED.swap(true, Ordering::Relaxed) {
        let hex = |off: usize| (0..24).map(|i| *((base + off + i) as *const u8))
            .map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ");
        append_log("scrim_strategy_applied.txt", &format!(
            "[{}ms] blue: {}\n        red:  {}\n", now_ms(), hex(O_BLUE_STRAT), hex(O_RED_STRAT)));
    }
}
unsafe fn restore_backup(r: &ClientDatabase) {
    let Some(bak) = BACKUP.lock().unwrap().take() else { return; };
    let Some((base, slots)) = scrim_slots(r, bak.key) else { return; };
    wr_u64(base + O_SEED, bak.seed);
    wr_u64(base + O_BLUE_BAN, bak.blue_ban.cap);
    wr_u64(base + O_BLUE_BAN + 8, bak.blue_ban.ptr);
    wr_u64(base + O_BLUE_BAN + 16, bak.blue_ban.len);
    wr_u64(base + O_RED_BAN, bak.red_ban.cap);
    wr_u64(base + O_RED_BAN + 8, bak.red_ban.ptr);
    wr_u64(base + O_RED_BAN + 16, bak.red_ban.len);
    wr_u64(base + O_BLUE_TEAM_ID, bak.blue_tid);
    wr_u64(base + O_RED_TEAM_ID, bak.red_tid);
    for i in 0..24 { *((base + O_BLUE_STRAT + i) as *mut u8) = bak.blue_strat[i]; }
    for i in 0..24 { *((base + O_RED_STRAT + i) as *mut u8) = bak.red_strat[i]; }
    for (i, &slot) in slots.iter().enumerate() {
        if let Some(sb) = bak.slots.get(i) {
            wr_u64(slot + AO_ATHLETE_ID, sb.aid);
            wr_u64(slot + AO_CHAMPION, sb.cap);
            wr_u64(slot + AO_CHAMPION + 8, sb.ptr);
            wr_u64(slot + AO_CHAMPION + 16, sb.len);
            wr_u64(slot + AO_POSITION, sb.pos);
            // items 복원: 원본 값 3개 + len 되돌림 (오염 방지)
            let iptr = rd_u64(slot + AO_ITEMS + 8) as usize;
            if iptr != 0 && writable(iptr, 24) { for s in 0..3 { wr_u64(iptr + s*8, sb.items[s]); } }
            wr_u64(slot + AO_ITEMS + 16, sb.item_len);
        }
    }
}
// pause_stack 안에 ReplayPopup(disc=6) 항목이 있는지 = 다시보기 팝업이 떠 있는지
unsafe fn replay_popup_present(r: &ClientDatabase) -> bool {
    let v = &r.pause_stack;
    let p = v.as_ptr() as *const u8;
    for i in 0..v.len() {
        if std::ptr::read_unaligned(p.add(i * 32) as *const u32) == 6 { return true; }
    }
    false
}

// ===========================================================================
//  filter_handler 설치 (클릭 → 큐). DB 접근은 post_update 에서.
// ===========================================================================
fn ensure_handler(ui: &mut GameUI) {
    if HANDLER_INSTALLED.swap(true, Ordering::Relaxed) {
        if !ui.filter_handler.is_empty() { return; }
        HANDLER_INSTALLED.store(false, Ordering::Relaxed);
    }
    let filter: Rc<dyn Fn(&UIEvent) -> bool> = Rc::new(|e: &UIEvent| {
        if let UIEvent::Click { path, .. } = e {
            let ps = path.to_string();
            let hit = |kind: u8, slot: usize| {
                CLICK_QUEUE.lock().unwrap().push((kind, slot));
            };
            // 드롭다운 우선 (열려 있을 때 그 클릭부터 처리)
            if ps.contains("scrim_ddp_close") || ps.contains("scrim_ddc_close")
                || ps.contains("scrim_ddp_xbtn") || ps.contains("scrim_ddc_xbtn") { hit(6, 0); return true; }
            if ps.contains("scrim_ddp_up") || ps.contains("scrim_ddc_up") { hit(7, 0); return true; }
            if ps.contains("scrim_ddp_dn") || ps.contains("scrim_ddc_dn") { hit(8, 0); return true; }
            for i in 0..PP_ROWS { if ps.contains(&format!("scrim_ddp_{:02}", i)) { hit(5, i); return true; } }
            for i in 0..CC_ROWS { if ps.contains(&format!("scrim_ddc_{:02}", i)) { hit(9, i); return true; } }
            if ps.contains("scrim_start") { hit(3, 0); return true; }
            if ps.contains("scrim_confirm") { hit(20, 0); return true; }
            if ps.contains("scrim_close") { hit(4, 0); return true; }
            if ps.contains("scrim_btn") { hit(0, 0); return true; }
            // ── 전술/아이템 모달 ──
            if ps.contains("scrim_open_strat") { hit(10, 0); return true; }
            if ps.contains("scrim_open_items") { hit(11, 0); return true; }
            if ps.contains("scrim_strat_ok") { hit(12, 0); return true; }
            if ps.contains("scrim_strat_cancel") || ps.contains("scrim_strat_x") || ps.contains("scrim_strat_bg") { hit(13, 0); return true; }
            if ps.contains("scrim_items_ok") { hit(14, 0); return true; }
            if ps.contains("scrim_items_cancel") || ps.contains("scrim_items_x") || ps.contains("scrim_items_bg") { hit(15, 0); return true; }
            // 전술 팀 토글 (박스 감지보다 먼저! scrim_st_ 로 시작하므로)
            if ps.contains("scrim_st_team") { hit(18, 0); return true; }
            // 스플릿 담당 박스 (scrim_sp_bld_1 / mor_1 / mor_2) — slot: 0=bld1, 1=mor1, 2=mor2
            if ps.contains("scrim_sp_bld_1") { hit(19, 0); return true; }
            if ps.contains("scrim_sp_mor_1") { hit(19, 1); return true; }
            if ps.contains("scrim_sp_mor_2") { hit(19, 2); return true; }
            // 전술 박스: scrim_st_<key> (각 키를 직접 검사 — 컨테이너 scrim_st_lcol 등과 혼동 방지)
            {
                let mut matched = false;
                for (idx, k) in SKEYS.iter().enumerate() {
                    if ps.contains(&format!("scrim_st_{}", k)) {
                        hit(16, idx); matched = true; break;
                    }
                }
                if matched { return true; }
            }
            // 아이템 박스: path 가 ".scrim_ib_<sl>_<s>" 로 끝남 (부모 _box/_blk/_brow 제외)
            {
                let mut matched = false;
                'outer: for sl in 0..10 {
                    let pre = if sl < 5 { "scrim_ib_" } else { "scrim_ir_" };
                    for s in 0..3 {
                        let id = format!("{}{}_{}", pre, sl, s); // 예: scrim_ib_0_0
                        // 클릭된 버튼이면 path 가 이 id 로 끝남 (마지막 세그먼트)
                        if ps.ends_with(&id) {
                            hit(17, sl * 3 + s); matched = true; break 'outer;
                        }
                    }
                }
                if matched { return true; }
            }
            for i in 0..10 {
                if ps.contains(&format!("scrim_p{}", i)) { hit(1, i); return true; }
                if ps.contains(&format!("scrim_c{}", i)) { hit(2, i); return true; }
            }
        }
        false
    });
    let handler: Rc<dyn Fn(&mut UIEventHandlerContext<(), UIOutEvent>)> = Rc::new(|_ctx| {});
    ui.filter_handler.push((filter, handler));
    HANDLER_INSTALLED.store(true, Ordering::Relaxed);
    write_log("scrim_handler.txt", &format!("handler 설치 @{}ms fh_len={}\n", now_ms(), ui.filter_handler.len()));
}

// ===========================================================================
//  메인
// ===========================================================================
struct ScrimExt;
impl ModExtension for ScrimExt {
    fn post_update(&self, scene: &mut Scene, ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        let Scene::InGame { data } = scene else { return; };
        if !BOOTED.swap(true, Ordering::Relaxed) {
            write_log("scrim_version.txt", &format!("build=SCRIM_MAIN booted @{}ms\n", now_ms()));
        }
        ensure_handler(ui);

        // ★ 사이드바 버튼 비교 덤프 (1회): scrim_btn 이 실제 존재할 때만 (UI 완성 후)
        if has_id(&ui.root, "scrim_btn") && !NODE_DUMP_DONE.swap(true, Ordering::Relaxed) {
            let mut s = String::from("=== 사이드바 버튼 비교 (scrim_btn vs 정상) ===\n");
            fn tree(n: &Node, depth: usize, out: &mut String) {
                let pad = "  ".repeat(depth);
                out.push_str(&format!("{}{} [{}] vis={} dis={}\n",
                    pad, n.id.as_str(), n.runner.type_name().rsplit(':').next().unwrap_or(""), n.visible, n.disabled));
                if depth < 6 { for c in n.child.iter() { tree(c, depth + 1, out); } }
            }
            fn find_and_tree(n: &Node, id: &str, out: &mut String) -> bool {
                if n.id.as_str() == id { tree(n, 0, out); return true; }
                for c in n.child.iter() { if find_and_tree(c, id, out) { return true; } }
                false
            }
            s.push_str("\n##### 사이드바(left) 트리 #####\n");
            if !find_and_tree(&ui.root, "left", &mut s) {
                s.push_str("(left 못 찾음 — root 1단계)\n");
                for c in ui.root.child.iter() {
                    s.push_str(&format!("  {} [{}]\n", c.id.as_str(), c.runner.type_name().rsplit(':').next().unwrap_or("")));
                }
            }
            s.push_str("\n##### scrim_btn 상세 #####\n");
            probe_full_by_id(&ui.root, "scrim_btn", &mut s);
            write_log("scrim_node_dump.txt", &s);
        }

        // ★ scrim_btn 텍스트/색 진단 (1회): 텍스트 키 문자열 + 색 후보 f32 → scrim_textkey.txt
        if has_id(&ui.root, "scrim_btn") && !TEXTKEY_DUMP_DONE.swap(true, Ordering::Relaxed) {
            let mut s = String::from("=== scrim_btn 텍스트/색 진단 ===\n");
            dump_scrim_textinfo_by_id(&ui.root, "scrim_btn", &mut s);
            write_log("scrim_textkey.txt", &s);
        }

        // ★ UI 연속 덤프(hover 색 추적용): UIDUMP_TARGET 존재 시 UIDUMP_PERIOD 프레임마다 1장.
        //    UI_Dump/ui_dump_N.txt 로 순번 저장 → 평소 프레임 1장 + hover 중 1장을 diff 하면
        //    바뀐 바이트 = hover 색/플래그. UIDUMP_MAX 장 찍으면 자동 정지(디스크 보호).
        //    사용법: 대상 버튼 위에 마우스 올렸다 뗐다 하며 몇 초 두면 평소/hover 스냅샷이 섞여 찍힘.
        if UIDUMP_ENABLED && has_id(&ui.root, UIDUMP_TARGET) {
            let tick = UIDUMP_TICK.fetch_add(1, Ordering::Relaxed);
            if tick % UIDUMP_PERIOD == 0 {
                let seq = UIDUMP_SEQ.load(Ordering::Relaxed);
                if seq < UIDUMP_MAX {
                    let mut s = format!(
                        "=== UI dump seq={} tick={} [{}ms] target={} ===\n",
                        seq, tick, now_ms(), UIDUMP_TARGET);
                    probe_full_by_id(&ui.root, UIDUMP_TARGET, &mut s); // 512B 확장본
                    dump_seq_write(seq, &s);
                    UIDUMP_SEQ.fetch_add(1, Ordering::Relaxed);
                    if seq + 1 == UIDUMP_MAX {
                        append_log("scrim_apply.txt", &format!(
                            "[{}ms] UI 연속덤프 {}장 완료 → 정지 (폴더: UI_Dump/)", now_ms(), UIDUMP_MAX));
                    }
                }
            }
        }

        // ★ 화면 라벨 연속 덤프: 현재 화면의 모든 LabelRunner(id + 게임이 그린 텍스트)를
        //   LBL_Dump/lbl_N.txt 로. 챔피언 이름이 보이는 화면으로 이동하면 스냅샷에 잡힘.
        //   → 모드 챔프 포함 실제 한글 이름이 어느 노드/형식으로 출력되는지 그대로 확인.
        if LBLDUMP_ENABLED {
            let tick = LBLDUMP_TICK.fetch_add(1, Ordering::Relaxed);
            if tick % LBLDUMP_PERIOD == 0 {
                let seq = LBLDUMP_SEQ.load(Ordering::Relaxed);
                if seq < LBLDUMP_MAX {
                    let mut s = format!("=== labels seq={} tick={} [{}ms] ===\n", seq, tick, now_ms());
                    dump_labels(&ui.root, 0, &mut s);
                    if let Some(dir) = dll_path().and_then(|p| p.parent().map(|d| d.join("LBL_Dump"))) {
                        let _ = fs::create_dir_all(&dir);
                        let _ = fs::write(dir.join(format!("lbl_{}.txt", seq)), &s);
                    }
                    LBLDUMP_SEQ.fetch_add(1, Ordering::Relaxed);
                    if seq + 1 == LBLDUMP_MAX {
                        append_log("scrim_apply.txt", &format!(
                            "[{}ms] 라벨 덤프 {}장 완료 → 정지 (폴더: LBL_Dump/)", now_ms(), LBLDUMP_MAX));
                    }
                }
            }
        }

        // ★ 색 오프셋 프로브: 후보 오프셋들을 구분색(빨/초/파)으로 칠해 배경/아이콘/텍스트 식별
        if COLOR_PROBE_ENABLED { color_probe(&mut ui.root); }
        // ★ scrim_btn 평소 색을 다른 메뉴 회색과 맞춤 (호버는 별도 슬롯이라 영향 없음)
        if SCRIM_BTN_IDLE_FIX { fix_scrim_btn_idle(&mut ui.root); }

        // ── 밸런스 검증(읽기전용): 재생 무장 후 1회, 선택 챔프들의 "현재(active)" 스탯 로그 ──
        //    재생은 historical 시트(당시값)를 쓰고, 여기서 읽는 건 현재값 → 둘을 비교해 목표값 확정.
        if SCRIM_ARMED.load(Ordering::Relaxed) && !BALANCE_LOGGED.load(Ordering::Relaxed) {
            let champs: Vec<String> =
                CHAMP_SLOTS.lock().unwrap().iter().filter_map(|c| c.clone()).collect();
            if !champs.is_empty() {
                let db = data.db();
                let r: &ClientDatabase = &*db;
                let mut out = format!("[{}ms] ── 현재(active) 챔피언 스탯 champion_info() ──\n", now_ms());
                for name in &champs {
                    match r.champion_info(name) {
                        Some(ci) => {
                            let s = ci.stat();
                            let g = ci.growth();
                            out.push_str(&format!(
                                "  {}: base_hp={} growth_hp={} atk={} def={}\n",
                                name, s.hp, g.hp, s.attack, s.defence));
                        }
                        None => out.push_str(&format!("  {}: champion_info=None\n", name)),
                    }
                }
                write_log("scrim_balance.txt", &out);
                BALANCE_LOGGED.store(true, Ordering::Relaxed);
            }
        }

        // ── 클릭 큐 처리 ──
        let clicks: Vec<(u8, usize)> = std::mem::take(&mut *CLICK_QUEUE.lock().unwrap());
        let mut changed = false;
        for (kind, slot) in clicks {
            match kind {
                0 => {
                    // 내부 스크림 열기: 게이트 검사
                    let db = data.db();
                    let r: &ClientDatabase = &*db;
                    dump_practice_matches(r); // ★ 매핑 검증 로그 (scrim_practice_dump.txt)
                    let practice = resolve_practice(r);
                    let roster = read_roster(r);
                    SCRIM_MODAL_VISIBLE.store(true, Ordering::Relaxed);
                    let practice_ok = practice.is_some();
                    let roster_ok = roster.len() >= 10;
                    if !practice_ok || !roster_ok {
                        CONFIG_READY.store(false, Ordering::Relaxed);
                        // 미달 조건을 모두 나열
                        let mut lines: Vec<String> = Vec::new();
                        if !practice_ok {
                            lines.push("• 연습경기 기록이 없습니다.".to_string());
                            lines.push("   연습경기를 최소 1회 실시하여야 합니다.".to_string());
                        }
                        if !roster_ok {
                            lines.push(format!("• 선수가 최소 10명 이상 필요합니다. (현재 {}명)", roster.len()));
                            lines.push("   선수가 10명 이상인데도 목록이 안 보이면,".to_string());
                            lines.push("   선수탭을 한번 열어 선수 리스트를 읽어와 주세요.".to_string());
                        }
                        set_label_by_id(&mut ui.root, "scrim_msg", &lines.join("\n\n"));
                    } else {
                        let (mid, key) = practice.unwrap();
                        SUMMON_MID.store(mid, Ordering::Relaxed);
                        INJECT_KEY.store(key as i64, Ordering::Relaxed);
                        MY_TEAM_ID.store(r.player_team_id() as u64, Ordering::Relaxed);
                        *MY_TEAM_NAME.lock().unwrap() = r.team(r.player_team_id())
                            .map(|t| t.name.clone()).unwrap_or_else(|| "MY TEAM".into());
                        *MY_TEAM_LOGO.lock().unwrap() = r.team(r.player_team_id())
                            .map(|t| t.logo.clone()).unwrap_or_default();
                        *ROSTER.lock().unwrap() = roster;
                        *CHAMPS.lock().unwrap() = read_champs(r);
                        // 선택 유지: 세션 첫 오픈이면 파일에서 복원, 그 후 현재 로스터/챔프로 검증
                        let empty = PLAYER_SLOTS.lock().unwrap().iter().all(|s| s.is_none())
                            && CHAMP_SLOTS.lock().unwrap().iter().all(|s| s.is_none());
                        if empty { load_selection(); }
                        validate_selection();
                        save_selection(); // 검증으로 정리된 상태 반영
                        CONFIG_READY.store(true, Ordering::Relaxed);
                        set_label_by_id(&mut ui.root, "scrim_msg", "선수와 챔피언을 모두 선택하세요");
                        changed = true;
                        append_log("scrim_apply.txt", &format!(
                            "[{}ms] open: practice match_id={} inject_key={}", now_ms(), mid, key));
                    }
                }
                1 => if CONFIG_READY.load(Ordering::Relaxed) { open_dropdown(&mut ui.root, 0, slot); },
                2 => if CONFIG_READY.load(Ordering::Relaxed) { open_dropdown(&mut ui.root, 1, slot); },
                5 => { select_dropdown(slot); changed = true; }
                9 => { select_dropdown(slot); changed = true; }
                7 => { dd_page_delta(-1); fill_page(&mut ui.root); }
                8 => { dd_page_delta(1); fill_page(&mut ui.root); }
                6 => { DD_OPEN.store(false, Ordering::Relaxed); }
                3 => {
                    if CONFIG_READY.load(Ordering::Relaxed) && all_filled() {
                        DD_OPEN.store(false, Ordering::Relaxed);
                        // ① 경기 재생 누른 순간 1회 주입 (백업 후)
                        {
                            let db = data.db();
                            let r: &ClientDatabase = &*db;
                            let key = INJECT_KEY.load(Ordering::Relaxed);
                            if key >= 0 {
                                unsafe {
                                    if let Some((base, slots)) = scrim_slots(r, key as usize) {
                                        capture_backup(key as usize, base, &slots);
                                        // 진단: apply 전 items 상태
                                        let ic = *ITEMS_COMMITTED.lock().unwrap();
                                        let mut diag = format!("=== [{}ms] items 진단 (apply 전) ===\n", now_ms());
                                        for i in 0..10 {
                                            let iptr = rd_u64(slots[i] + AO_ITEMS + 8) as usize;
                                            let ilen = rd_u64(slots[i] + AO_ITEMS + 16);
                                            let cur: Vec<u64> = if iptr != 0 && readable(iptr, 24) { (0..3).map(|s| rd_u64(iptr + s*8)).collect() } else { vec![] };
                                            diag.push_str(&format!("  slot{} ib={:?} ilen={} cur={:?}\n", i, ic[i], ilen, cur));
                                        }
                                        apply_lineup(base, &slots);
                                        // 진단: apply 후 items 상태
                                        diag.push_str("--- apply 후 ---\n");
                                        for i in 0..10 {
                                            let iptr = rd_u64(slots[i] + AO_ITEMS + 8) as usize;
                                            let ilen = rd_u64(slots[i] + AO_ITEMS + 16);
                                            let cur: Vec<u64> = if iptr != 0 && readable(iptr, 24) { (0..3).map(|s| rd_u64(iptr + s*8)).collect() } else { vec![] };
                                            diag.push_str(&format!("  slot{} ilen={} cur={:?}\n", i, ilen, cur));
                                        }
                                        append_log("scrim_items_diag.txt", &diag);
                                        append_log("scrim_apply.txt", &format!("[{}ms] 경기재생 클릭 → 1회 주입(key={})", now_ms(), key));
                                    }
                                }
                            }
                        }
                        // 스크림만 현재밸런스: pre_patch_data 비워 유지(팝업 닫힐 때 복원).
                        // 백업은 1회만(이미 있으면 유지) — 재진입에도 원본 보존.
                        {
                            let mut dbm = data.db_mut();
                            if PRE_PATCH_BAK.lock().unwrap().is_none() {
                                let taken = std::mem::take(&mut dbm.pre_patch_data);
                                let nlen = taken.len();
                                *PRE_PATCH_BAK.lock().unwrap() = Some(taken);
                                append_log("scrim_patch.txt", &format!(
                                    "[{}ms] ★pre_patch_data 백업+비움 (len={})\n", now_ms(), nlen));
                            } else {
                                dbm.pre_patch_data.clear(); // 이미 백업됨 → 그냥 비움 유지
                            }
                        }
                        SCRIM_ARMED.store(true, Ordering::Relaxed);
                        SUMMON_REQUESTED.store(true, Ordering::Relaxed);
                        SCRIM_MODAL_VISIBLE.store(false, Ordering::Relaxed);
                        // 밸런스 검증: 우리 경기 선수 athlete_id 스냅샷(think 필터용) + 로그 1회 재무장
                        *OUR_ATHLETES.lock().unwrap() =
                            PLAYER_SLOTS.lock().unwrap().iter().filter_map(|x| *x).collect();
                        BALANCE_LOGGED.store(false, Ordering::Relaxed);
                        append_log("scrim_apply.txt", &format!("[{}ms] ★ARMED + 소환요청", now_ms()));
                    }
                }
                4 => { SCRIM_MODAL_VISIBLE.store(false, Ordering::Relaxed); DD_OPEN.store(false, Ordering::Relaxed); }
                20 => { SCRIM_MODAL_VISIBLE.store(false, Ordering::Relaxed); DD_OPEN.store(false, Ordering::Relaxed); } // 확인 버튼 = 닫기
                10 => { // 팀 전술 열기: working=committed 복사, 블루부터 보기
                    *STRAT_WORKING.lock().unwrap() = *STRAT_COMMITTED.lock().unwrap();
                    STRAT_VIEW_TEAM.store(0, Ordering::Relaxed);
                    STRAT_OPEN.store(true, Ordering::Relaxed);
                    repaint_strat(&mut ui.root);
                }
                11 => { // 개인 전술(아이템) 열기
                    *ITEMS_WORKING.lock().unwrap() = *ITEMS_COMMITTED.lock().unwrap();
                    ITEMS_OPEN.store(true, Ordering::Relaxed);
                    repaint_items(&mut ui.root);
                }
                12 => { *STRAT_COMMITTED.lock().unwrap() = *STRAT_WORKING.lock().unwrap(); STRAT_OPEN.store(false, Ordering::Relaxed); }
                13 => { STRAT_OPEN.store(false, Ordering::Relaxed); } // 취소: working 버림
                14 => { *ITEMS_COMMITTED.lock().unwrap() = *ITEMS_WORKING.lock().unwrap(); ITEMS_OPEN.store(false, Ordering::Relaxed); }
                15 => { ITEMS_OPEN.store(false, Ordering::Relaxed); }
                16 => { // 전술 박스 cycle (현재 보는 팀 대상)
                    let team = STRAT_VIEW_TEAM.load(Ordering::Relaxed) as usize;
                    let f = slot;
                    if team < 2 && f < 12 {
                        let mut w = STRAT_WORKING.lock().unwrap();
                        let n = STRAT_OPTS[f].len() as u8;
                        w[team][f] = (w[team][f] + 1) % n;
                        drop(w);
                        repaint_strat(&mut ui.root);
                    }
                }
                18 => { // 전술 팀 토글 (버튼 1개 — 누를 때마다 0↔1 전환)
                    let cur = STRAT_VIEW_TEAM.load(Ordering::Relaxed);
                    STRAT_VIEW_TEAM.store(if cur == 0 { 1 } else { 0 }, Ordering::Relaxed);
                    repaint_strat(&mut ui.root);
                }
                19 => { // 스플릿 담당 cycle (slot: 0=bld1, 1=mor1, 2=mor2). 포지션 0~4 순환, 1-3-1 중복 방지
                    let team = STRAT_VIEW_TEAM.load(Ordering::Relaxed) as usize;
                    let mut sp = STRAT_SPLIT_POS.lock().unwrap();
                    let (bld, mor_a, mor_b) = sp[team];
                    let next = |cur: u8, skip: Option<u8>| -> u8 {
                        let mut n = (cur + 1) % 5;
                        if let Some(s) = skip { if n == s { n = (n + 1) % 5; } }
                        n
                    };
                    match slot {
                        0 => { sp[team].0 = next(bld, None); }          // bld 담당
                        1 => { // mor 첫째 담당. 1-3-1(둘째 존재)일 때만 둘째와 중복 방지
                            let mor_v = STRAT_WORKING.lock().unwrap()[team][6];
                            let skip = if mor_v == 2 { Some(mor_b) } else { None };
                            sp[team].1 = next(mor_a, skip);
                        }
                        2 => { // mor 둘째 담당. 첫째와 겹치면 건너뜀
                            let nn = next(mor_b, Some(sp[team].1));
                            sp[team].2 = nn;
                        }
                        _ => {}
                    }
                    drop(sp);
                    repaint_strat(&mut ui.root);
                }
                17 => { // 아이템 박스 cycle
                    let sl = slot / 3; let s = slot % 3;
                    if sl < 10 && s < 3 {
                        let mut w = ITEMS_WORKING.lock().unwrap();
                        w[sl][s] = (w[sl][s] + 1) % (ITEM_OPTS.len() as u8);
                        drop(w);
                        repaint_items(&mut ui.root);
                    }
                }
                _ => {}
            }
        }

        // 드롭다운 표시 (선수창/챔프창 분리)
        let dd_open = DD_OPEN.load(Ordering::Relaxed);
        let dd_kind = DD_KIND.load(Ordering::Relaxed);
        set_visible_by_id(&mut ui.root, "scrim_dd_p", dd_open && dd_kind == 0);
        set_visible_by_id(&mut ui.root, "scrim_dd_c", dd_open && dd_kind == 1);
        set_visible_by_id(&mut ui.root, "scrim_strat_modal", STRAT_OPEN.load(Ordering::Relaxed));
        set_visible_by_id(&mut ui.root, "scrim_items_modal", ITEMS_OPEN.load(Ordering::Relaxed));

        // 모달 표시 + 라벨
        let modal_vis = SCRIM_MODAL_VISIBLE.load(Ordering::Relaxed);
        set_visible_by_id(&mut ui.root, "scrim_modal", modal_vis);
        if modal_vis {
            let ready = CONFIG_READY.load(Ordering::Relaxed);
            // 조건 충족 시: 선수/챔프 슬롯 + 헤더 + 전술바 + 시작버튼 보임, 확인 숨김.
            // 조건 미달 시: 위 전부 숨기고 확인 버튼만 (가운데가 비므로 안내+확인만 노출).
            set_visible_by_id(&mut ui.root, "scrim_head", ready);
            for i in 0..5 { set_visible_by_id(&mut ui.root, &format!("scrim_row{}", i), ready); }
            set_visible_by_id(&mut ui.root, "scrim_openbar", ready);
            set_visible_by_id(&mut ui.root, "scrim_start", ready);
            set_visible_by_id(&mut ui.root, "scrim_confirm", !ready);
            // ★ panel 높이: 조건충족=원래(640), 미달=작게(280). 매프레임(layout 덮음 대비).
            //   미달 시 문구(scrim_msg)를 헤더 아래로 내려 겹침 방지 (y 64→100).
            if ready {
                set_rect_size_by_id(&ui.root, "panel", 1120.0, 640.0);
                set_rect_pos_by_id(&ui.root, "scrim_msg", 30.0, 64.0);
            } else {
                set_rect_size_by_id(&ui.root, "panel", 1120.0, 380.0);
                set_rect_pos_by_id(&ui.root, "header", 30.0, 24.0);
                set_rect_pos_by_id(&ui.root, "scrim_msg", 40.0, 140.0);
            }
            if ready { refresh_labels(&mut ui.root); }
        }

        // ── pre_patch_data 복원: 팝업 닫힘 후 1회 (think 아님 → 토글 없음) ──
        if PRE_PATCH_RESTORE_PENDING.swap(false, Ordering::Relaxed) {
            if let Some(bak) = PRE_PATCH_BAK.lock().unwrap().take() {
                let nlen = bak.len();
                data.db_mut().pre_patch_data = bak;
                append_log("scrim_patch.txt", &format!(
                    "[{}ms] 팝업닫힘 → pre_patch_data 복원 (len={})\n", now_ms(), nlen));
            }
        }
        // ── SCRIM 세션(팝업) 동안 pre_patch_data 를 계속 비워 유지 → 매 재생이 항상 현재밸런스 ──
        //    (백업은 arm 때 1회. 복원은 팝업 닫힐 때.) db borrow 전에 처리.
        if SCRIM_ARMED.load(Ordering::Relaxed) && PRE_PATCH_BAK.lock().unwrap().is_some() {
            let mut dbm = data.db_mut();
            if !dbm.pre_patch_data.is_empty() { dbm.pre_patch_data.clear(); }
        }

        let db = data.db();
        let r: &ClientDatabase = &*db;
        let replay_some = r.replay_view.is_some();
        // 다시보기 진입(None→Some) 시 + 진행중 600프레임마다: items 역산 로그 (경기 진행 추적)
        let replay_tick = if replay_some { REPLAY_LOG_TICK.fetch_add(1, Ordering::Relaxed) } else { REPLAY_LOG_TICK.store(0, Ordering::Relaxed); 0 };
        let do_replay_log = replay_some && (!PREV_REPLAY_SOME.swap(true, Ordering::Relaxed) || replay_tick % 60 == 0);
        if do_replay_log {
            if let Some(mt) = r.replay_view.as_ref() {
                let mid = match mt {
                    MatchType::Normal { match_id } => *match_id as i64,
                    MatchType::Practice { match_id } => *match_id as i64,
                    _ => -1,
                };
                if let Some(mi) = r.matches.get(mt) {
                    let mut out = format!("[{}ms] match_id={} (세트 {}개)\n", now_ms(), mid, mi.replays.len());
                    for (si, &key) in mi.replays.iter().enumerate() {
                        if let Some((base, _)) = unsafe { scrim_slots(r, key) } {
                            let hex = |off: usize| (0..24).map(|i| unsafe { *((base + off + i) as *const u8) })
                                .map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ");
                            out.push_str(&format!(
                                "  [세트{} key={}] blue: {}\n              red:  {}\n",
                                si + 1, key, hex(O_BLUE_STRAT), hex(O_RED_STRAT)));
                        }
                    }
                    append_log("scrim_strategy_view.txt", &out);
                    // [추적] Nightstar 선수만 4종 비교 로그
                    if let Some(&key0) = mi.replays.first() {
                        if let Some((_, slots)) = unsafe { scrim_slots(r, key0) } {
                            let dir = ["AD","Magic","AttackSpeed","Defense","MagicResist","Hp","Auto"];
                            let roster = ROSTER.lock().unwrap().clone();
                            // Nightstar athlete_id 찾기
                            let night_aid = roster.iter().find(|(_, n)| n == "Nightstar").map(|(id, _)| *id as u64);
                            let mut a = format!("=== Nightstar 추적 [{}ms] match_id={} key={} tick={} (night_aid={:?}) ===\n", now_ms(), mid, key0, replay_tick, night_aid);
                            let my = MY_TEAM_ID.load(Ordering::Relaxed);
                            for (si, &s) in slots.iter().enumerate() {
                                unsafe {
                                    let aid = rd_u64(s + 520);
                                    if Some(aid) != night_aid { continue; } // Nightstar 만
                                    let champ = read_str(rd_u64(s + 240), rd_u64(s + 248) as usize);
                                    // [4] 실제 보유 아이템 (items @256)
                                    let iptr = rd_u64(s + 256 + 8) as usize;
                                    let ilen = rd_u64(s + 256 + 16);
                                    let mut items_str = String::new();
                                    if readable(iptr, (ilen as usize).min(20)*8) {
                                        for k in 0..ilen.min(20) { items_str.push_str(&format!("{} ", item_name(rd_u64(iptr + k as usize*8)))); }
                                    }
                                    // [1][2] champion_personal_tactics (현 player team) — 챔프별
                                    let pt = |team_id: usize| -> String {
                                        if let Some(t) = r.team(team_id) {
                                            if let Some(arr) = t.champion_personal_tactics.get(champ.as_str()) {
                                                let addr = arr.as_ptr() as usize;
                                                if readable(addr, 3) {
                                                    let b = [*(addr as *const u8), *((addr+1) as *const u8), *((addr+2) as *const u8)];
                                                    // 원시 숫자 + (우리 추정 이름)
                                                    let nm = b.iter().map(|&x| if (x as usize)<7 { dir[x as usize] } else {"?"}).collect::<Vec<_>>().join(",");
                                                    return format!("원시[{},{},{}] 추정({})", b[0], b[1], b[2], nm);
                                                }
                                            }
                                        }
                                        "없음".into()
                                    };
                                    // [3] 우리 스크림 설정 (ITEMS_COMMITTED[그 슬롯])
                                    let ic = ITEMS_COMMITTED.lock().unwrap()[si];
                                    let uidir = ["자동","공격","주문","공속","방어","마저","체력"];
                                    let ic_str = ic.iter().map(|&x| if (x as usize)<7 { uidir[x as usize] } else {"?"}).collect::<Vec<_>>().join(",");
                                    a.push_str(&format!(
                                        "slot{} champ={} aid={}\n  [1/2] champion_personal_tactics(myteam={}) = {}\n  [3] 스크림설정 ITEMS_COMMITTED = {}\n  [4] 실제보유 items({}) = {}\n",
                                        si, champ, aid, my, pt(my as usize), ic_str, ilen, items_str));
                                    // athlete 구조 빌드후보 영역 (items 앞뒤, 0~6 작은값 탐색)
                                    a.push_str("  [athlete 빌드후보 0~6 값들]: ");
                                    for off in (0..544).step_by(8) {
                                        let v = rd_u64(s + off);
                                        if v <= 6 { a.push_str(&format!("@{}={} ", off, v)); }
                                    }
                                    a.push('\n');
                                    // items Vec 앞뒤(@280~320) 도 (빌드지시 Vec 가 따로?)
                                    a.push_str("  [@272~320]: ");
                                    for off in (272..320).step_by(8) { a.push_str(&format!("@{}={} ", off, rd_u64(s + off))); }
                                    a.push('\n');
                                    // ★ champ id 후보값(4,10,21,74,200) athlete slot 전체에서 위치 찾기
                                    a.push_str("  [champ후보값 위치 4/10/21/74/200]: ");
                                    let cand: [u64;5] = [4,10,21,74,200];
                                    for off in (0..544).step_by(4) { // 4바이트 단위로도
                                        if readable(s+off, 8) {
                                            let v8 = rd_u64(s + off);
                                            let v4 = (v8 & 0xffffffff) as u64;
                                            if cand.contains(&v8) { a.push_str(&format!("@{}(u64)={} ", off, v8)); }
                                            else if cand.contains(&v4) && v4 != 4 { a.push_str(&format!("@{}(u32)={} ", off, v4)); }
                                        }
                                    }
                                    a.push('\n');
                                    // entity 도 스캔 (전투개체에 champ id 있을수)
                                    let ent = rd_u64(s + 0) ; // athlete 첫 필드가 entity? 아닐수 있음
                                    a.push_str(&format!("  [entity ptr 후보 @0={:#x}]\n", ent));
                                }
                            }
                            append_log("scrim_nightstar.txt", &a);
                        }
                    }
                }
            }
        } else if !replay_some {
            PREV_REPLAY_SOME.store(false, Ordering::Relaxed);
        }

        // ── replay_popup 소환 (pause_stack raw push) ──
        if SUMMON_REQUESTED.swap(false, Ordering::Relaxed) {
            let mid = SUMMON_MID.load(Ordering::Relaxed);
            drop(db);
            let mut dbm = data.db_mut();
            let v = &mut dbm.pause_stack;
            let old = v.len();
            let mut item = [0u8; 32];
            item[0..4].copy_from_slice(&6u32.to_le_bytes());        // ReplayPopup
            item[8..16].copy_from_slice(&MT_PRACTICE.to_le_bytes()); // Practice
            item[16..24].copy_from_slice(&mid.to_le_bytes());        // match_id
            unsafe {
                v.reserve(1);
                let p = v.as_mut_ptr() as *mut u8;
                core::ptr::copy_nonoverlapping(item.as_ptr(), p.add(old * 32), 32);
                v.set_len(old + 1);
            }
            POPUP_ACTIVE.store(true, Ordering::Relaxed);
            PREV_POPUP_PRESENT.store(true, Ordering::Relaxed);
            append_log("scrim_apply.txt", &format!(
                "[{}ms] ★ pause_stack push ReplayPopup Practice match_id={} (len {}→{})",
                now_ms(), mid, old, dbm.pause_stack.len()));
            return;
        }

        // ── 팝업 존재(실제 pause_stack 기준) ──
        let popup_now = unsafe { replay_popup_present(r) };
        let prev_popup = PREV_POPUP_PRESENT.swap(popup_now, Ordering::Relaxed);

        // ── 스크림 세션 동안: 주입(1회 시도) + 팝업 떠있으면 표시 정리 ──
        if SCRIM_ARMED.load(Ordering::Relaxed) {
            let key = INJECT_KEY.load(Ordering::Relaxed);
            if key >= 0 && !INJECTED_ONCE.load(Ordering::Relaxed) {
                unsafe {
                    if let Some((base, slots)) = scrim_slots(r, key as usize) {
                        capture_backup(key as usize, base, &slots); // 최초 1회만(idempotent)
                        apply_lineup(base, &slots);                  // ★1회만 주입 (안되면 매프레임으로 되돌릴것)
                        INJECTED_ONCE.store(true, Ordering::Relaxed);
                    }
                }
            }
            // 팝업이 보일 때(재생 중 아님)만 표시 정리 — 재생 끝나고 다시 떠도 계속 적용
            if popup_now && !replay_some {
                let myname = MY_TEAM_NAME.lock().unwrap().clone();
                let mylogo = MY_TEAM_LOGO.lock().unwrap().clone();
                fix_replay_popup(&mut ui.root, &myname);
                fix_logos(&mut ui.root, &mylogo);              // 내 팀 쪽 로고로 양쪽 통일
                set_visible_by_id(&mut ui.root, "prev", false); // ◀ 숨김
                set_visible_by_id(&mut ui.root, "next", false); // ▶ 숨김
                blank_view_by_id(&mut ui.root, "view");         // "다시보기" → "연습 시작" (+408,+1008)
            }
            // 팝업이 닫히면(있다가 사라짐, 재생중 아님) 복원 + 세션 해제
            if prev_popup && !popup_now && !replay_some {
                unsafe { restore_backup(r); }
                CAPTURED.store(false, Ordering::Relaxed);
                SCRIM_ARMED.store(false, Ordering::Relaxed);
                INJECTED_ONCE.store(false, Ordering::Relaxed); // 다음 세션 위해 리셋
                POPUP_ACTIVE.store(false, Ordering::Relaxed);
                LOGO_DIAG_DONE.store(false, Ordering::Relaxed);
                // 재생 없이 닫힌 경우 등: 백업 남아있으면 복원 예약(안전망)
                if PRE_PATCH_BAK.lock().unwrap().is_some() {
                    PRE_PATCH_RESTORE_PENDING.store(true, Ordering::Relaxed);
                }
                append_log("scrim_apply.txt", &format!("[{}ms] 팝업 닫힘 → 복원 + 해제", now_ms()));
            }
        }
    }
}

// ★ 읽기전용 시뮬 훅: 재생 중 우리 경기 엔티티의 실제 stat 측정 (동작 영향 0).
//   think 는 시뮬 워커 스레드에서 동기 호출됨. 우리 경기 필터 = athlete_id ∈ OUR_ATHLETES.
#[derive(Clone, Debug)]
struct ProbeAi;
impl ModPlayerInputAi for ProbeAi {
    fn clone_box(&self) -> Box<dyn ModPlayerInputAi> { Box::new(self.clone()) }
    fn id(&self) -> &str { "scrim_probe_ai" }
    fn think(&mut self, ctx: &mut PlayerAiContext<'_, '_, '_>, base_input: Option<Input>) -> PlayerInputDecision {
        let aid = ctx.athlete_id();
        let ours = { OUR_ATHLETES.lock().unwrap().contains(&aid) };
        if ours {
            let n = THINK_CALLS.fetch_add(1, Ordering::Relaxed);
            if n % 1000 == 0 {
                let patched = SHEET_PATCH_LOGGED.load(Ordering::Relaxed);
                append_log("scrim_stat.txt", &format!(
                    "[{}ms] (patched={}) ★재생 champ={} max_hp={:?} hp={:?} tick={} athlete={} team={}\n",
                    now_ms(), patched, ctx.champion_name(), ctx.max_hp(), ctx.hp(), ctx.tick(), aid, ctx.team()));
            }
            // [테스트] 광전사 entity 스탯+아이템 영역 일정간격 출력
        }
        match base_input {
            Some(i) => PlayerInputDecision::Replace(i),
            None => PlayerInputDecision::Pass,
        }
    }
}

// ★ 서버 확장(authoritative): 재생이 읽는 original_info_sheet 를 현재 champion_info_sheet 로 덮는다.
//   → 재생 시뮬이 "현재 밸런스"(hp/공격/주문력/스킬계수/쿨타임/성장 전부)로 재구성됨.
//   on_server_start(로드 1회) + before_management_tick(시간진행 시) 에 적용. 멱등.
struct ScrimServerExt;
impl ModServerExtension for ScrimServerExt {
    fn on_server_start(&self, ctx: &mut ServerModContext) {
        server_patch_work(ctx, "on_server_start");
        patch_original_sheet(ctx, "on_server_start");
    }
    fn before_management_tick(&self, ctx: &mut ServerModContext) {
        server_patch_work(ctx, "before_management_tick");
        if SHEET_PATCH_ENABLED.load(Ordering::Relaxed) {
            patch_original_sheet(ctx, "before_management_tick");
        }
    }
}
static SHEET_PATCH_ENABLED: AtomicBool = AtomicBool::new(false); // (무효 확인) 시트 덮기 — 기본 끔
static SHEET_PATCH_LOGGED: AtomicBool = AtomicBool::new(false);
static PRE_PATCH_CLEAR: AtomicBool = AtomicBool::new(false);  // 서버쪽 무조건 비우기 — 끔
static PATCH_DIAG_LOGGED: AtomicBool = AtomicBool::new(false);
static DB_PROBED: AtomicBool = AtomicBool::new(false); // item_network 주소 검증 1회
static DUMP_FIRST_MS: AtomicU64 = AtomicU64::new(0); // ★ 첫 호출 시각 (30초 지연 게이트용)
const ITEMSET_DUMP: bool = true; // ★모드템 위치찾기 (그래프 len + 이름 메모리 스캔)
static ITEM_NET_ADDR: AtomicUsize = AtomicUsize::new(0); // item_network 주소 (B방식 신경망 호출용)
static BUILD_CACHE: Mutex<[[u64; 3]; 10]> = Mutex::new([[0; 3]; 10]); // B방식 계산결과 캐시
static BUILD_CACHE_KEY: AtomicU64 = AtomicU64::new(u64::MAX); // 캐시 유효성 키 (라인업+커밋 해시)
static ITEMNET_VERIFY_LOGGED: AtomicBool = AtomicBool::new(false); // client item_net 검증 1회
static NODE_DUMP_DONE: AtomicBool = AtomicBool::new(false); // 노드 종합 덤프 1회
static TEXTKEY_DUMP_DONE: AtomicBool = AtomicBool::new(false); // scrim_btn 텍스트/색 진단 1회
static CHAMP_NAME_DUMP_DONE: AtomicBool = AtomicBool::new(false); // 챔피언 이름 자동탐색 검증 1회
// ★ UI 연속 덤프 설정 (hover 색 추적용)
const UIDUMP_ENABLED: bool = false;       // 연속 덤프 on/off (17장 확보 → 끔)
const UIDUMP_TARGET: &str = "scrim_btn";  // 덤프 대상 노드 id (정상버튼 비교하려면 "roaster" 등으로)
const UIDUMP_PERIOD: u64 = 30;            // 몇 프레임마다 1장 (≈0.5초@60fps)
const UIDUMP_MAX: u64 = 60;               // 최대 파일 수 (디스크 보호, ≈30초 분량)
static UIDUMP_TICK: AtomicU64 = AtomicU64::new(0); // 프레임 카운터
static UIDUMP_SEQ: AtomicU64 = AtomicU64::new(0);  // 다음 파일 순번
// ★ 화면 라벨 연속 덤프: 전체 UI 트리의 LabelRunner(노드 id + 게임이 해석해 그린 텍스트)를
//   LBL_Dump/lbl_N.txt 로. 챔피언 이름이 보이는 화면(챔프 정보/밴픽/툴팁 등)으로 이동하면
//   스냅샷에 잡힘 → 모드 챔프 포함 한글 이름이 어느 노드에 어떻게 출력되는지 그대로 확인.
const LBLDUMP_ENABLED: bool = false;      // 라벨 덤프 on/off (이름 경로 확인 완료 → 끔)
const LBLDUMP_PERIOD: u64 = 60;           // 몇 프레임마다 1장 (≈1초@60fps)
const LBLDUMP_MAX: u64 = 30;              // 최대 파일 수 (≈30초 분량 네비)
static LBLDUMP_TICK: AtomicU64 = AtomicU64::new(0);
static LBLDUMP_SEQ: AtomicU64 = AtomicU64::new(0);
// ★ 색 오프셋 프로브 (어느 f32 오프셋이 scrim_btn 의 배경/아이콘/텍스트 색인지 식별)
const COLOR_PROBE_ENABLED: bool = false;  // 안전상태 복귀 — 블록B 쓰기는 메모리 손상(렌더 얼룩) 유발
// (오프셋, R, G, B, A) — 블록A 만. 블록B(744/944/1048)는 색이 아니라 쓰면 화면 깨짐 → 제거
const COLOR_PROBE_SET: [(usize, f32, f32, f32, f32); 3] = [
    (144, 1.0, 0.0, 0.0, 1.0), // 빨강
    (344, 0.0, 1.0, 0.0, 1.0), // 초록
    (448, 0.0, 0.0, 1.0, 1.0), // 파랑
];
// ★ scrim_btn 평소(비호버) 색 보정: +144 를 다른 메뉴와 같은 회색으로. 호버색은 별도 슬롯이라 영향 없음.
//   인게임 보고 미세조정: 너무 밝으면 숫자 낮추고, 너무 어두우면 올리면 됨.
const SCRIM_BTN_IDLE_FIX: bool = true;    // 비호버 아이콘(+144)+텍스트(+448) 회색. 호버(+744/+1048)는 안 건드림→흰색 유지
const SCRIM_BTN_IDLE_OFFS: [usize; 2] = [144, 448]; // +144=아이콘, +448=텍스트 (둘 다 비호버 색)
const SCRIM_BTN_IDLE_RGBA: (f32, f32, f32, f32) = (0.70, 0.71, 0.74, 1.0); // ≈ 다른 메뉴 회색
static ITEMNET_LAST_SEQ: AtomicU64 = AtomicU64::new(0); // 마지막 로그한 캡처 seq
fn server_patch_work(ctx: &mut ServerModContext, where_: &str) {
    // ★ Database 주소 잡기 + item_network(+0xda0) 검증
    if !DB_PROBED.swap(true, Ordering::Relaxed) {
        unsafe {
            // &ctx.database 는 Database 시작이 아님 (ServerModContext 구조).
            // 알려진 필드(champion_patch_statistics @ Database+0x16698)의 절대주소로 역산.
            let cps = &ctx.database.champion_patch_statistics as *const _ as usize;
            let db = cps - 0x16698;       // 진짜 Database 시작
            let item_net = db + 0xda0;    // LogisticSGDAgent
            ITEM_NET_ADDR.store(item_net, Ordering::Relaxed); // B방식 호출용 저장
            let mut s = format!("[{}ms] ({}) DB 주소 검증 (역산)\n", now_ms(), where_);
            s.push_str(&format!("  champion_patch_statistics = {:#x}\n", cps));
            s.push_str(&format!("  db(역산) = {:#x}\n", db));
            s.push_str(&format!("  item_network(db+0xda0) = {:#x}\n", item_net));
            if readable(item_net, 0x20) {
                let w0 = rd_u64(item_net + 0x00);
                let wptr = rd_u64(item_net + 0x08);
                let wcnt = rd_u64(item_net + 0x10);
                let fdim = rd_u64(item_net + 0x18);
                s.push_str(&format!("  +0x00={:#x} +0x08(wptr)={:#x} +0x10(wcnt)={} +0x18(fdim)={}\n", w0, wptr, wcnt, fdim));
                if wptr > 0x10000 && readable(wptr as usize, 32) {
                    let fs: Vec<f32> = (0..6).map(|i| f32::from_bits(rd_u64(wptr as usize + i*4) as u32)).collect();
                    s.push_str(&format!("  weights[0..6] = {:?}\n", fs));
                }
                // ★ 모듈 베이스 + FUN_1419a4c00 함수 주소 검증
                //   메인 exe(게임) 모듈 베이스 = GetModuleHandleW(NULL)
                let h = GetModuleHandleW(core::ptr::null());
                s.push_str(&format!("  GetModuleHandleW(NULL) = {:#x}\n", h as usize));
                if h != 0 {
                    let mbase = h as usize;
                    let fn_addr = mbase + 0x19a4c00; // 0x1419a4c00 - 0x140000000
                    s.push_str(&format!("  module_base = {:#x}\n", mbase));
                    s.push_str(&format!("  FUN_1419a4c00 주소 = {:#x}\n", fn_addr));
                    if readable(fn_addr, 16) {
                        let bytes: Vec<u8> = (0..16).map(|i| *((fn_addr + i) as *const u8)).collect();
                        let hex: String = bytes.iter().map(|b| format!("{:02x} ", b)).collect();
                        s.push_str(&format!("  함수 시작 바이트: {}\n", hex));
                        s.push_str("  (기대값: 55 41 57 41 56 41 55 41 54 56 57 53 48 81 ec)\n");
                        // 바이트 일치하면 트램폴린 후킹 설치
                        let expect = [0x55u8,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53,0x48,0x81,0xec];
                        if bytes[..15] == expect {
                            s.push_str("  ★바이트 일치 → 트램폴린 후킹 설치 시도\n");
                            match install_itemnet_hook() {
                                Ok(stub) => s.push_str(&format!("  후킹 설치 성공! stub={:#x}\n", stub)),
                                Err(e) => s.push_str(&format!("  후킹 설치 실패: {}\n", e)),
                            }
                        }
                    } else {
                        s.push_str("  함수 주소 NOT readable\n");
                    }
                }
            } else {
                s.push_str("  item_network NOT readable!\n");
            }
            write_log("scrim_itemnet_probe.txt", &s);

            // ★ STEP1(정정): 디컴 추적으로 ItemSetting 데이터 시작 = db+0x12d58 (파서 FUN_1405739b0 인자).
            //   기존 +0x12d50 은 8바이트 어긋나 Vec 오판→쓰레기 len 으로 멈춤 유발. 안전하게 재작성.
            if ITEMSET_DUMP {
                // ★ 주기 덤프: 백그라운드 스레드로 5초마다 (로딩 전/후 변화 추적). db 를 move.
                let db_for_thread = db;
                std::thread::spawn(move || {
                    let db = db_for_thread;
                    let mut iter: u32 = 0;
                    loop {
                        std::thread::sleep(std::time::Duration::from_secs(5));
                        iter += 1;
                        unsafe {
                let mut d = format!("[{}ms] (iter {}) 모드템 위치찾기2\n", now_ms(), iter);
                // (A) riot_items_tfm2.dll 로드 주소 찾기
                let modh = GetModuleHandleW(
                    "riot_items_tfm2.dll".encode_utf16().chain([0]).collect::<Vec<u16>>().as_ptr());
                d.push_str(&format!("riot_items_tfm2.dll base = {:#x}\n", modh as usize));
                let scrimh = GetModuleHandleW(core::ptr::null());
                d.push_str(&format!("game exe base = {:#x}\n", scrimh as usize));
                // (B) 모드템 이름 변형들 (.name 포함, 대소문자)
                let baits = ["infinity_edge","rabadons_deathcap","deathblade","nashors_tooth",
                             "infinity_edge.name","executioners_calling","InfinityEdge","bf_sword"];
                let find_str_anywhere = |p: usize| -> String {
                    if p<=0x10000 || !readable(p, 24) { return String::new(); }
                    let sp=rd_u64(p+8) as usize; let sl=rd_u64(p+16) as usize;
                    if sp>0x10000 && sl>=3 && sl<=60 && readable(sp,sl) {
                        let by=std::slice::from_raw_parts(sp as *const u8, sl);
                        if let Ok(s)=std::str::from_utf8(by){ return s.to_string(); }
                    }
                    String::new()
                };
                // (C) 모드 DLL 메모리 영역에서 직접 문자열 스캔 (DLL base ~ +0x100000)
                if modh as usize > 0x10000 {
                    let mb = modh as usize;
                    d.push_str("\n[모드DLL 내 아이템이름 문자열 직접탐색]\n");
                    let mut hits = 0;
                    for off in (0..0x80000).step_by(1) {
                        let a = mb + off;
                        if !readable(a, 16) { continue; }
                        // 직접 ascii 문자열인지 (infinity_edge 등)
                        let by = std::slice::from_raw_parts(a as *const u8, 24.min(0x80000-off));
                        if by[0].is_ascii_lowercase() {
                            let e = by.iter().position(|&c| c==0 || !(c.is_ascii_lowercase()||c==b'_'||c.is_ascii_digit())).unwrap_or(0);
                            if e>=8 && e<=40 {
                                if let Ok(s)=std::str::from_utf8(&by[..e]){
                                    if s.contains('_') && (s.contains("edge")||s.contains("blade")||s.contains("deathcap")||s.contains("tooth")||s.contains("radiant")||s.contains("infinity")) {
                                        d.push_str(&format!("  DLL+{:#x}: {:?}\n", off, s));
                                        hits += 1;
                                        if hits > 50 { break; }
                                    }
                                }
                            }
                        }
                    }
                    d.push_str(&format!("  (총 {}개 히트)\n", hits));
                }
                write_log("scrim_itemsetting_dump.txt", &d);
                        } // unsafe
                        if iter >= 2 { break; }
                    } // loop
                }); // thread
            }
        }
    }
    // ★ 트램폴린이 ctx 캡처했으면 누적 로그 (CAPTURE_SEQ 바뀔때마다)
    {
        let seq = CAPTURE_SEQ.load(Ordering::Relaxed);
        let last = ITEMNET_LAST_SEQ.load(Ordering::Relaxed);
        if seq != last {
            ITEMNET_LAST_SEQ.store(seq, Ordering::Relaxed);
            let buf = *CAPTURED_CTX.lock().unwrap();
            let names = ["swordman","monk","mod_champions","fighter","knight","archer","soldier","priest","pythoness","pyromancer","ice_mage","ninja","magic_knight","berserker","executioner","lancer","ogre","dual_blader","cavalry_knight","gunner","pole_warrior","jiangshi","gambler","hammerer","demon","vampire","spirit_caller","boomerang_hunter","inquisitor","shield_bearer","whip_master","werewolf","dokkaebi","necromancer","bard","barrier_magician","chef","clown","dancer","dark_mage","exorcist","ghost","illusionist","lightning_mage","plague_doctor","poison_dart_hunter","shadowmancer","taoist","siege_breaker","android","druid","prisoner","bomber","voodoo_shaman","white_mage","wind_mage","enchanter","hitman","guardian_spirit","hunter","circus_blade"];
            let nm = |v: u64| -> String { if (v as usize) < names.len() { names[v as usize].to_string() } else { format!("?{}", v) } };
            let mut s = format!("\n=== 캡처 #{} [{}ms] ({}) ===\n", seq, now_ms(), where_);
            // ★ 신경망에 넘어온 아이템 ID 후보(cands) — 모드템 ID 여기 찍힘!
            {
                let a = *CAPTURED_ARGS.lock().unwrap();
                let cc = *CAPTURED_CANDS.lock().unwrap();
                let ncap = CAPTURED_CANDS_N.load(Ordering::Relaxed) as usize;
                s.push_str(&format!("  ★cands: n={} flag={} (item_net={:#x})\n", a[3], a[4], a[0]));
                // ★ 캡처순간 저장된 cands (포인터 재읽기 X = 정확)
                let n = ncap.min(32);
                let ids: Vec<u64> = (0..n).map(|i| cc[i]).collect();
                s.push_str(&format!("  ★cands 전체({}개): {:?}\n", ids.len(), ids));
                let big: Vec<u64> = ids.iter().cloned().filter(|&v| v >= 30).collect();
                if !big.is_empty() { s.push_str(&format!("  ★★모드ID후보(30+): {:?}\n", big)); }
            }
            s.push_str(&format!("  champ id [0..5] = {:?}\n", &buf[0..5]));
            s.push_str(&format!("  → 시트이름: {}\n", (0..5).map(|i| nm(buf[i])).collect::<Vec<_>>().join(", ")));
            s.push_str(&format!("  카운터 [5..10] = {:?} → {}\n", &buf[5..10], (5..10).map(|i| nm(buf[i])).collect::<Vec<_>>().join(", ")));
            // 우리 스크림 라인업 (CHAMP_SLOTS) 도 같이 — 대조용
            let cs = CHAMP_SLOTS.lock().unwrap();
            let our: Vec<String> = (0..10).map(|i| cs[i].clone().unwrap_or_else(|| "-".into())).collect();
            drop(cs);
            s.push_str(&format!("  [우리 스크림 라인업 CHAMP_SLOTS] = {:?}\n", our));
            s.push_str("    (캡처 시트이름이 우리 라인업과 같으면 = 시트인덱스 확정!)\n");
            // 스택 덤프 (리턴주소 = 호출처, 힙포인터 = athlete/팀 구조 탐색)
            let st = *CAPTURED_STACK.lock().unwrap();
            let mbase = unsafe { GetModuleHandleW(core::ptr::null()) as u64 };
            s.push_str("  [진입시점 스택 (리턴주소+호출처 지역변수)]:\n");
            unsafe {
                for i in 0..24 {
                    let v = st[i];
                    if v == 0 { continue; }
                    let mut tag = String::new();
                    // 모듈 주소 = 코드 (리턴주소/호출처)
                    if v >= mbase && v < mbase + 0x4000000 {
                        tag = format!(" [코드 RVA={:#x}]", v - mbase);
                    } else if v > 0x10000 && readable(v as usize, 24) {
                        // 힙 포인터 = 구조체. String(ptr@8,len@16) 또는 직접 문자열 탐색
                        let sp = rd_u64(v as usize + 8);
                        let sl = rd_u64(v as usize + 16);
                        if sp > 0x10000 && sl > 0 && sl < 64 && readable(sp as usize, sl as usize) {
                            let txt = read_str(sp, sl as usize);
                            if txt.len() >= 3 && txt.chars().all(|c| c.is_ascii_graphic()) {
                                tag = format!(" [String?=\"{}\"]", txt);
                            }
                        }
                        if tag.is_empty() {
                            let txt = read_str(v, 20);
                            if txt.len() >= 3 && txt.chars().take(10).all(|c| c.is_ascii_graphic()||c=='_') {
                                tag = format!(" [str?=\"{}\"]", &txt[..txt.len().min(18)]);
                            }
                        }
                    }
                    s.push_str(&format!("    sp[{}] @{:#x} = {:#x}{}\n", i, i*8, v, tag));
                }
            }
            append_log("scrim_champ_id.txt", &s);
        }
    }
    // 읽기전용 진단: 패치 맵 크기 (1회)
    if !PATCH_DIAG_LOGGED.swap(true, Ordering::Relaxed) {
        write_log("scrim_patch.txt", &format!(
            "[{}ms] ({}) pre_patch_data.len={} champion_patch_statistics.len={}\n",
            now_ms(), where_,
            ctx.database.pre_patch_data.len(),
            ctx.database.champion_patch_statistics.len()));
    }
    if PRE_PATCH_CLEAR.load(Ordering::Relaxed) {
        ctx.database.pre_patch_data.clear();
    }
}
fn patch_original_sheet(ctx: &mut ServerModContext, where_: &str) {
    if !SHEET_PATCH_ENABLED.load(Ordering::Relaxed) { return; }
    // 현재(active) 시트를 복제해 원본 시트에 덮어쓰기 → 재생이 현재 밸런스로 돌게
    let cur = ctx.database.champion_info_sheet.clone();
    ctx.database.original_info_sheet = cur;
    if !SHEET_PATCH_LOGGED.swap(true, Ordering::Relaxed) {
        write_log("scrim_sheet.txt",
            &format!("[{}ms] original_info_sheet ← champion_info_sheet ({})\n", now_ms(), where_));
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(ScrimExt);
    reg.set_server_extension(ScrimServerExt); // 서버측 authoritative — 시트 덮기
    reg.add_player_input_ai(ProbeAi); // 읽기전용 시뮬 훅(재생 stat 측정)
    reg
}
declare_mod!(init);

// ===========================================================================
//  챔피언 영문 key → 한글명
// ===========================================================================
// 아이템 ID → 카테고리/티어 (0~4공격력 5~9공속 10~14방어력 15~19마저 20~24주문력 25~29체력)
fn item_name(id: u64) -> String {
    ensure_catalog();
    {
        let guard = ITEM_CATALOG.lock().unwrap();
        if let Some(cat) = guard.as_ref() {
            if let Some(m) = cat.iter().find(|m| m.id as u64 == id) {
                let base = cat_kor(&m.category);
                if !base.is_empty() && m.tier >= 0 { return format!("{}{}", base, m.tier + 1); }
                return m.key.clone(); // 모드템 등 카테고리 한글 매핑 없으면 key
            }
        }
    }
    // 폴백: 카탈로그 못 읽음 → 기존 하드코딩 공식
    let cat = ["공격력","공속","방어력","마저","주문력","체력"];
    if id < 30 { format!("{}{}", cat[(id/5) as usize], id%5+1) }
    else { format!("?{}", id) }
}
// ===========================================================================
//  champion 문자열 ↔ champ id 변환 (= ChampionInfoSheet 시트 인덱스)
//  트램폴린 캡처로 검증됨: siege_breaker=48, whip_master=30 등 (실제 신경망 ctx 값과 일치)
//  exe 의 "struct ChampionInfoSheet with 61 elements" 직전 문자열 순서.
// ===========================================================================
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
// champion 문자열 → champ id (시트 인덱스). 없으면 None.
fn champ_id(name: &str) -> Option<usize> {
    CHAMP_SHEET.iter().position(|&c| c == name)
}
// champ id → champion 문자열. 범위 밖이면 None.
fn champ_name(id: usize) -> Option<&'static str> {
    CHAMP_SHEET.get(id).copied()
}
fn champ_kr(key: &str) -> String {
    let name = match key {
        "acrobat" => "봉술사", "android" => "안드로이드", "archer" => "궁수", "bard" => "음유시인",
        "barrier_magician" => "결계사", "berserker" => "광전사", "bomber" => "폭탄병",
        "boomerang_hunter" => "부메랑 헌터", "cavalry_knight" => "기병", "chef" => "요리사",
        "circus_blade" => "곡예사", "clown" => "광대", "dancer" => "무희", "dark_mage" => "흑마술사",
        "demon" => "악마", "dokkaebi" => "도깨비", "druid" => "드루이드", "dual_blader" => "듀얼 블레이더",
        "enchanter" => "인챈터", "executioner" => "처형인", "exorcist" => "엑소시스트", "fighter" => "격투가",
        "gambler" => "도박사", "ghost" => "유령", "guardian_spirit" => "수호령", "gunner" => "총잡이",
        "hammerer" => "중보병", "hitman" => "히트맨", "hunter" => "사냥꾼", "ice_mage" => "얼음술사",
        "illusionist" => "환영술사", "inquisitor" => "이단심문관", "jiangshi" => "강시", "knight" => "기사",
        "lancer" => "창술사", "lightning_mage" => "번개술사", "magic_knight" => "마검사", "monk" => "몽크",
        "necromancer" => "네크로맨서", "ninja" => "닌자", "ogre" => "오우거", "plague_doctor" => "역병의사",
        "poison_dart_hunter" => "독침술사", "pole_warrior" => "봉술사", "priest" => "성직자",
        "prisoner" => "죄수", "pyromancer" => "화염술사", "pythoness" => "무녀", "shadowmancer" => "그림자술사",
        "shield_bearer" => "방패병", "siege_breaker" => "공성병", "sniper" => "소총수", "soldier" => "중보병",
        "spirit_caller" => "정령사", "swordman" => "검사", "taoist" => "도사", "vampire" => "흡혈귀",
        "voodoo" => "부두술사", "voodoo_shaman" => "부두술사", "whip_master" => "채찍술사",
        "white_mage" => "백마법사", "wind_mage" => "바람술사", "werewolf" => "늑대인간",
        _ => return key.to_string(),
    };
    name.to_string()
}