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
//!   - 연습경기 없으면 / 로스터 5명 미만이면 안내 문구
//!   - 설정 모달: 선수10 + 챔프10 슬롯(클릭=후보 선택, 선수·챔프 중복 허용), 다 차면 시작 활성
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

#[path = "C:/tfm2mods/ui_kit/ui_kit.rs"]
mod ui_kit; // cursor_to_game(마우스→게임좌표) 용 — 풀다운 위치 계산

const MOD_ID: &str = "tfm2_scrim";

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
const LEN_F32_OFF: usize = 4;         // Length enum 내 f32 위치 (디버그 덤프용; 레이아웃 쓰기는 ui_kit::set_layout_*)
#[link(name = "user32")]
extern "system" { fn GetAsyncKeyState(vk: i32) -> i16; }
#[link(name = "kernel32")]
extern "system" { fn IsBadReadPtr(lp: usize, ucb: usize) -> i32; }
const VK_F2: i32 = 0x71;
const VK_UP: i32 = 0x26;
const VK_DOWN: i32 = 0x28;

// 네이티브 DropdownRunner 옵션 set 함수 (RE 확인). RVA = abs - image_base(0x140000000).
const FN_DD_SETOPT_RVA: usize = 0x2416070; // 0.5.0_3(구0.5.0_2=0x2418cf0, UNIQUE PROL-OK 555657/4883ec70). 0.4.14 06-24 핫픽스 재탐색(ghidra-re 2026-06-28): 프롤로그 5556574883ec70 + mov[rcx+0x1788],rdx + 옵션Vec[+0x1528] 확정, 마스크시그 유일매치. ⚠구값 0x21381f0/0x213fbd0/0x2112800 등은 전부 미검증 오류(컨테이너 함수 중간 주소).

// dropdown 옵션 1개 = 0x28(40바이트): 색상 16B + 텍스트 String 24B
#[repr(C)]
struct DdOpt {
    color:  u64,   // +0  색상(RGBA 등)
    color2: u32,   // +8  색 변형
    alpha:  f32,   // +12 = 1.0
    s_len:  usize, // +16 String.len  (게임 String = {len, ptr, cap})
    s_ptr:  usize, // +24 String.ptr
    s_cap:  usize, // +32 String.cap
}
const MT_PRACTICE: u64 = 2; // MatchType disc: Tutorial0 Normal1 Practice2 SoloRank3

// 테스트 스위치: 0=자동(맨 처음 연습경기). 0이 아니면 그 match_id(예: 1176)를 강제로 사용.
// 매핑 검증용. 자동 선택이 미덥지 않을 때 1176 으로 고정해 엔진부터 확인.
const FORCE_PRACTICE_MATCH_ID: u64 = 0;

// ── 상태 ──
static BOOTED: AtomicBool = AtomicBool::new(false);
static SCRIM_FH_LEN: AtomicUsize = AtomicUsize::new(usize::MAX); // filter_handler len 추적(공존 재등록용)
static SCRIM_MODAL_VISIBLE: AtomicBool = AtomicBool::new(false);
static NAT_DD: ui_kit::NativeDropdown = ui_kit::NativeDropdown::new("scrim_nat_dd");
static NAT_ROSTER: Mutex<Vec<String>> = Mutex::new(Vec::new()); // 가상 스크롤 전체 명단
const NAT_WINDOW: usize = 5; // dropdown 에 한번에 보일 개수
static SCRIM_ARMED: AtomicBool = AtomicBool::new(false);
static INJECTED_ONCE: AtomicBool = AtomicBool::new(false); // 1회 주입 플래그 (매프레임 대신)
// ── [아이템칸 검증] 라이브 보유 아이템(GamePlayerStateData.items) 선수별 최대개수 추적 ──
static ITEMLOG_POS: AtomicUsize = AtomicUsize::new(0); // 이벤트 프레임 증분 스캔 위치
static ITEMLOG_MAX: Mutex<Option<HashMap<String, usize>>> = Mutex::new(None); // 선수명 → 최대 보유개수
static COUNT_BY_SLOT: Mutex<[usize; 10]> = Mutex::new([0; 10]);   // slot → 현재 아이템수(골드로그용)
static GOLD_MS_BY_SLOT: Mutex<[i64; 10]> = Mutex::new([0; 10]);   // slot → 마지막 기록 골드마일스톤(2000단위)
static SUMMON_REQUESTED: AtomicBool = AtomicBool::new(false);
static CAPTURED: AtomicBool = AtomicBool::new(false);
static POPUP_ACTIVE: AtomicBool = AtomicBool::new(false); // 다시보기 팝업 떠있는 동안
static PREV_POPUP_PRESENT: AtomicBool = AtomicBool::new(false); // 직전 프레임 팝업 존재 여부
static PREV_REPLAY_SOME: AtomicBool = AtomicBool::new(false);
static REPLAY_LOG_TICK: AtomicU64 = AtomicU64::new(0); // 다시보기 로그 간격 카운터
static CONFIG_READY: AtomicBool = AtomicBool::new(false); // 게이트 통과(설정 가능)
static SUMMON_MID: AtomicU64 = AtomicU64::new(u64::MAX); // popup 용 Practice match_id
static INJECT_KEY: AtomicI64 = AtomicI64::new(-1); // 주입 대상 match_replays key
static CREATED_ENTRIES: Mutex<Option<(u64, usize)>> = Mutex::new(None); // ★create-new: 삽입한 (match_id, key) — 종료 시 제거용
static FRAMESCAN_TICK: AtomicUsize = AtomicUsize::new(0); // ★M4 프레임스캔 검증용 throttle 카운터
static LAST_BRIEF: Mutex<Option<([SlotStat; 10], MatchAgg)>> = Mutex::new(None); // ★M4 최근 성공 스캔(스탯+집계)
static HAD_GAMEVIEW: AtomicBool = AtomicBool::new(false); // 이번 세션 game_view 본 적 있나
static BRIEF_FINALIZED: AtomicBool = AtomicBool::new(false); // brief 1회 확정/기록 가드
static RESULT_READY: AtomicBool = AtomicBool::new(false); // brief 확정됨 → 팝업닫힘시 결과창 표시 대기
static SCRIM_RESULT_VISIBLE: AtomicBool = AtomicBool::new(false); // 결과창(scrim_result) 표시중
static RESDIAG_DONE: AtomicBool = AtomicBool::new(false); // 주입검사 1회 로그 가드
static AUTO_CLOSE_ARMED: AtomicBool = AtomicBool::new(false); // 재생후 중간 ReplayPopup 자동닫기 대기
static SCRIM_DBLOG_DONE: AtomicBool = AtomicBool::new(false); // ClientDatabase 베이스 1회 로깅 가드

// ── 밸런스 검증(읽기전용) ──
static THINK_CALLS: AtomicUsize = AtomicUsize::new(0);
static THINK_ALL: AtomicUsize = AtomicUsize::new(0); // [캡추적2] ours 무관 think 호출 카운터(entity 프로브 트리거)
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
const ITEM_OPTS: [&str; 25] = ["자동", "공격력", "주문력", "공격속도", "방어력", "마저", "체력",
    "★Blackfire Torch", "★Blade of Ruined King", "★Deathblade", "★Experimental Hexplate", "★Frozen Mallet",
    "★Guinsoo's Rageblade", "★Infinity Edge", "★Jak'sho Protean", "★Mirage Blade", "★Mortal Reminder",
    "★Nashor's Tooth", "★Protector's Vow", "★Protoplasm Harness", "★Rabadon's Deathcap", "★Riftmaker",
    "★Spirit Visage", "★Terminus", "★Unending Despair"]; // 7~24 = 모드 최종템 '등록순' (라이브 ID 동적바인딩, 박힌번호 아님)
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
const PP_ROWS: usize = 40; // 공용 풀다운 행 노드 수(선수/전술/아이템). 아이템 옵션(자동+카테고리+모드최종템)이 많을 수 있어 넉넉히.
const CC_ROWS: usize = 42; // 챔피언창: 3열 × 14행 = 42/페이지
static DD_OPEN: AtomicBool = AtomicBool::new(false);
static DD_KIND: AtomicUsize = AtomicUsize::new(0); // 0=선수, 1=챔피언
static DD_SLOT: AtomicUsize = AtomicUsize::new(0); // 어느 슬롯을 채우는 중인지
static DD_PAGE: AtomicUsize = AtomicUsize::new(0); // 현재 페이지
// 현재 펼친 목록: (선수id, 챔프key, 표시라벨)
static DD_ITEMS: Mutex<Vec<(Option<usize>, Option<String>, String)>> = Mutex::new(Vec::new());
// 전술/아이템 드롭다운에서 현재 값 인덱스 (행 selected 하이라이트용)
static DD_CUR: AtomicUsize = AtomicUsize::new(0);
// 전술 12필드 → (왼쪽열?, 열내 행) — 풀다운을 해당 박스 위에 띄우기 위함 (SKEYS 인덱스 순)
const STRAT_POS: [(bool, usize); 12] = [
    (true, 0),  // foc
    (true, 1),  // jng
    (true, 2),  // srp
    (true, 3),  // srt
    (true, 5),  // bld
    (true, 4),  // bat
    (false, 5), // mor
    (false, 0), // twr
    (false, 1), // def
    (false, 2), // fin
    (false, 3), // wav
    (false, 4), // end
];
// 스플릿 담당 포지션 이름 (0=탑 … 4=서포터)
const POS_NAMES: [&str; 5] = ["탑", "정글", "미드", "바텀", "서포터"];

// 챔프(1)만 그리드(scrim_ddc), 나머지(선수0/전술2/아이템3)는 스크롤 풀다운(scrim_ddp) 재사용
fn dd_rows() -> usize { if DD_KIND.load(Ordering::Relaxed) == 1 { CC_ROWS } else { PP_ROWS } }
fn dd_prefix() -> &'static str {
    match DD_KIND.load(Ordering::Relaxed) {
        1 => "scrim_ddc",     // 챔프 그리드
        2 | 4 => "scrim_dds",  // 팀 전술 / 스플릿 담당 (폭 304 = 버튼 폭)
        _ => "scrim_ddp",     // 선수 / 아이템 (폭 240)
    }
}
fn dd_pages() -> usize {
    let rows = dd_rows();
    let len = DD_ITEMS.lock().unwrap().len();
    ((len + rows - 1) / rows).max(1)
}


// 원본 백업
struct SlotBak { aid: u64, cap: u64, ptr: u64, len: u64, pos: u64, item_len: u64, items: [u64; 3], item_cap: u64, item_ptr: u64 }
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
//  SEH 안전읽기 — VEH 로 접근위반(0xC0000005)을 가로채 크래시 대신 실패 반환.
//  목적: 힙 자료구조 탐색 중 틀린 주소를 만져도 게임이 안 멈추게 하는 범용 도구.
//  SEH[]: 0=active 1=tid 2=land_rip 3=land_rsp 4=land_rbp 5=code_lo 6=code_hi 7=faults
// ===========================================================================
#[link(name = "kernel32")]
extern "system" {
    fn AddVectoredExceptionHandler(first: u32, handler: VehHandler) -> usize;
    fn GetCurrentThreadId() -> u32;
}
#[repr(C)]
struct ExceptionRecord { code: u32, flags: u32, rec: usize, addr: usize, nparams: u32, _p: u32, params: [usize; 15] }
#[repr(C)]
struct ExceptionPointers { rec: *mut ExceptionRecord, ctx: *mut core::ffi::c_void }
type VehHandler = extern "system" fn(*mut ExceptionPointers) -> i32;

static mut SEH: [u64; 8] = [0u64; 8];
static SEH_INSTALLED: AtomicBool = AtomicBool::new(false);
static SEH_BUSY: AtomicBool = AtomicBool::new(false); // 한 번에 한 스레드만 (전역 SEH 보호)
// ── [캡추적] 가드페이지 프로브: items 버퍼 읽는 코드의 RIP 포착 ──
static GUARD_BASE: AtomicU64 = AtomicU64::new(0);     // 가드 페이지 시작(0=비활성)
static GUARD_END: AtomicU64 = AtomicU64::new(0);
static GUARD_HITS: AtomicU64 = AtomicU64::new(0);     // 총 폴트수(재무장 상한용)
static GUARD_LAST_RIP: AtomicU64 = AtomicU64::new(0); // 마지막 폴트 RIP
static GUARD_LAST_OFF: AtomicU64 = AtomicU64::new(0); // 접근 오프셋(0=item0,8,16,24=4th)
static GUARD_LAST_TID: AtomicU64 = AtomicU64::new(0);
static GUARD_PENDING: AtomicBool = AtomicBool::new(false); // 미기록 폴트 있음(post_update가 기록)
static GUARD_RSP: AtomicU64 = AtomicU64::new(0);
static GUARD_STK: [AtomicU64; 16] = [const { AtomicU64::new(0) }; 16]; // 폴트시 스택 16qword (게임exe 복귀주소 탐색)
static GUARD_REGS: [AtomicU64; 4] = [const { AtomicU64::new(0) }; 4];  // rcx,rdx,r8,r9 (memcpy 길이 후보)
const ITEMLOG_ON: bool = false; // ★false=정상(꺼짐). 재생중 GamePlayerStateData.items/골드 로깅(scrim_item_count.txt)

// CONTEXT x64 오프셋: Rsp@0x98, Rbp@0xA0, Rip@0xF8
extern "system" fn seh_veh(p: *mut ExceptionPointers) -> i32 {
    const CONTINUE_EXECUTION: i32 = -1;
    const CONTINUE_SEARCH: i32 = 0;
    unsafe {
        if p.is_null() { return CONTINUE_SEARCH; }
        let rec = (*p).rec;
        if rec.is_null() { return CONTINUE_SEARCH; }
        // ── [캡추적] 가드페이지 폴트(STATUS_GUARD_PAGE_VIOLATION=0x80000001) ──
        //   우리 items 버퍼 페이지를 읽은 명령어(RIP)+오프셋 포착. 가드는 OS가 자동해제 →
        //   CONTINUE_EXECUTION 으로 그 읽기는 정상 진행. post_update 가 매프레임 재무장.
        if (*rec).code == 0x80000001 {
            let acc = if (*rec).nparams >= 2 { (*rec).params[1] } else { 0 };
            let gb = GUARD_BASE.load(Ordering::Relaxed) as usize;
            let ge = GUARD_END.load(Ordering::Relaxed) as usize;
            if gb != 0 && acc >= gb && acc < ge {
                GUARD_HITS.fetch_add(1, Ordering::Relaxed);
                if !GUARD_PENDING.swap(true, Ordering::Relaxed) {
                    GUARD_LAST_RIP.store((*rec).addr as u64, Ordering::Relaxed);
                    GUARD_LAST_OFF.store((acc - gb) as u64, Ordering::Relaxed);
                    GUARD_LAST_TID.store(GetCurrentThreadId() as u64, Ordering::Relaxed);
                    let ctx = (*p).ctx as usize;
                    if ctx != 0 {
                        GUARD_REGS[0].store(*((ctx + 0x80) as *const u64), Ordering::Relaxed); // rcx
                        GUARD_REGS[1].store(*((ctx + 0x88) as *const u64), Ordering::Relaxed); // rdx
                        GUARD_REGS[2].store(*((ctx + 0xB8) as *const u64), Ordering::Relaxed); // r8
                        GUARD_REGS[3].store(*((ctx + 0xC0) as *const u64), Ordering::Relaxed); // r9
                        let rsp = *((ctx + 0x98) as *const u64) as usize;
                        GUARD_RSP.store(rsp as u64, Ordering::Relaxed);
                        if rsp > 0x10000 {
                            for k in 0..16 { GUARD_STK[k].store(*((rsp + k * 8) as *const u64), Ordering::Relaxed); }
                        }
                    }
                }
                return CONTINUE_EXECUTION;
            }
            return CONTINUE_SEARCH; // 우리 페이지 아님
        }
        if (*rec).code != 0xC0000005 { return CONTINUE_SEARCH; }
        let g = core::ptr::addr_of!(SEH) as *const u64;
        if *g.add(0) == 0 { return CONTINUE_SEARCH; }          // active?
        if *g.add(1) != GetCurrentThreadId() as u64 { return CONTINUE_SEARCH; } // 우리 스레드?
        let ctx = (*p).ctx as usize;
        if ctx == 0 { return CONTINUE_SEARCH; }
        let rip = *((ctx + 0xF8) as *const u64);
        if rip < *g.add(5) || rip >= *g.add(6) { return CONTINUE_SEARCH; } // 우리 보호구간?
        // 폴트 → 착륙지점으로 리디렉트 (rsp/rbp 복원)
        *((ctx + 0xF8) as *mut u64) = *g.add(2); // Rip = land_rip
        *((ctx + 0x98) as *mut u64) = *g.add(3); // Rsp = land_rsp
        *((ctx + 0xA0) as *mut u64) = *g.add(4); // Rbp = land_rbp
        let gm = core::ptr::addr_of_mut!(SEH) as *mut u64;
        *gm.add(7) += 1; // faults++
        CONTINUE_EXECUTION
    }
}

fn seh_install() {
    if SEH_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    unsafe { AddVectoredExceptionHandler(1, seh_veh); }
}

// dst 로 src 에서 len 바이트 복사. 폴트나면 false (게임 안 멈춤). 동시호출은 직렬화.
#[inline(never)]
unsafe fn safe_copy(dst: *mut u8, src: *const u8, len: usize) -> bool {
    if !SEH_INSTALLED.load(Ordering::Relaxed) { return false; }
    // 전역 SEH 상태 보호: 한 번에 하나만
    while SEH_BUSY.swap(true, Ordering::Acquire) { core::hint::spin_loop(); }
    let g = core::ptr::addr_of_mut!(SEH) as *mut u64;
    *g.add(1) = GetCurrentThreadId() as u64; // tid
    let mut ok: u64;
    core::arch::asm!(
        "lea rax, [rip + 200f]",
        "mov [{g} + 40], rax",        // code_lo
        "lea rax, [rip + 201f]",
        "mov [{g} + 48], rax",        // code_hi
        "lea rax, [rip + 202f]",
        "mov [{g} + 16], rax",        // land_rip
        "mov [{g} + 24], rsp",        // land_rsp
        "mov [{g} + 32], rbp",        // land_rbp
        "mov qword ptr [{g} + 0], 1", // active = 1
        "cld",
        "200:",
        "rep movsb",                  // [rdi]<-[rsi], rcx bytes
        "201:",
        "mov {ok}, 1",
        "jmp 203f",
        "202:",                       // VEH 착륙 (rsp/rbp 복원됨)
        "mov {ok}, 0",
        "203:",
        "mov qword ptr [{g} + 0], 0", // active = 0
        g = in(reg) g,
        ok = out(reg) ok,
        inout("rcx") len => _,
        inout("rdi") dst => _,
        inout("rsi") src => _,
        out("rax") _,
    );
    SEH_BUSY.store(false, Ordering::Release);
    ok != 0
}

// 안전 읽기 헬퍼들
unsafe fn safe_read_u64(addr: usize) -> Option<u64> {
    let mut b = [0u8; 8];
    if safe_copy(b.as_mut_ptr(), addr as *const u8, 8) { Some(u64::from_le_bytes(b)) } else { None }
}
unsafe fn safe_read_bytes(addr: usize, len: usize, out: &mut Vec<u8>) -> bool {
    if len == 0 || len > 4096 { return false; }
    out.clear(); out.resize(len, 0);
    safe_copy(out.as_mut_ptr(), addr as *const u8, len)
}

// 셀프테스트: 유효주소=성공, 잘못된주소=크래시없이 실패 여야 정상.
const SEH_TEST: bool = true;
static SEH_TESTED: AtomicBool = AtomicBool::new(false);
fn seh_selftest() {
    if !SEH_TEST || SEH_TESTED.swap(true, Ordering::Relaxed) { return; }
    seh_install();
    unsafe {
        let probe: u64 = 0x1122_3344_5566_7788;
        let valid = &probe as *const u64 as usize;
        let ok_valid = safe_read_u64(valid);
        let ok_bad1 = safe_read_u64(0x1);
        let ok_bad2 = safe_read_u64(0xdead_0000_0000);
        let ok_bad3 = safe_read_u64(0);
        let faults = { let g = core::ptr::addr_of!(SEH) as *const u64; *g.add(7) };
        write_log("scrim_seh.txt", &format!(
            "[{}ms] SEH 셀프테스트\n  설치됨={}\n  유효읽기 = {:?} (기대 Some({:#x}))\n  잘못된0x1 = {:?} (기대 None)\n  잘못된0xdead.. = {:?} (기대 None)\n  잘못된0x0 = {:?} (기대 None)\n  총 폴트삼킴 = {}\n  → 게임이 안 멈추고 이 로그가 보이면 SEH 작동!\n",
            now_ms(), SEH_INSTALLED.load(Ordering::Relaxed),
            ok_valid, probe, ok_bad1, ok_bad2, ok_bad3, faults));
    }
}

// ===========================================================================
//  (다) safe_read 탐색 덤프 — 이제 프리징 안 나니 자유롭게 구경.
//   1) db..db+0x20000 을 8바이트씩 String{cap,ptr,len}로 프로브 → ASCII key 로그
//      (아이템 key 들이 뭉쳐 나오는 지점 = ItemSetting 마스터 컬렉션 위치)
//   2) item_net / cps 주변 raw qword 덤프
// ===========================================================================
const EXPLORE_DUMP: bool = false; // ★ 배포=false (1회 백그라운드 진단스캔 끔)
static EXPLORE_DONE: AtomicBool = AtomicBool::new(false);
unsafe fn safe_explore(db: usize, item_net: usize, cps: usize) {
    if !EXPLORE_DUMP || EXPLORE_DONE.swap(true, Ordering::Relaxed) { return; }
    seh_install();
    // 위치 p 가 String{cap@0, ptr@+8, len@+0x10} 이면 그 문자열 반환
    let str_at = |p: usize| -> Option<String> {
        let ptr = safe_read_u64(p + 8)? as usize;
        let len = safe_read_u64(p + 16)? as usize;
        if ptr <= 0x10000 || len < 3 || len > 40 { return None; }
        let mut buf = Vec::new();
        if !safe_read_bytes(ptr, len, &mut buf) { return None; }
        if buf.iter().all(|&b| b == b'_' || b.is_ascii_alphanumeric()) {
            return String::from_utf8(buf).ok();
        }
        None
    };
    let mut s = format!("[{}ms] (다) safe_read 탐색\n  db={:#x} item_net={:#x} cps={:#x}\n", now_ms(), db, item_net, cps);

    // 1) 문자열 프로브 스캔
    s.push_str("=== 문자열 프로브 (db+off) ===\n");
    let mut hits = 0u32;
    let mut off = 0usize;
    while off < 0x20000 {
        if let Some(st) = str_at(db + off) {
            s.push_str(&format!("  db+{:#07x}: '{}'\n", off, st));
            hits += 1;
            if hits > 500 { s.push_str("  ...(500개 초과, 끊음)\n"); break; }
        }
        off += 8;
    }
    s.push_str(&format!("  문자열 후보 총 {}개\n", hits));

    // 2) item_net 주변 raw qword 덤프 (-0x40 ~ +0x80)
    s.push_str("=== item_net 주변 raw ===\n");
    let base = item_net.wrapping_sub(0x40);
    for i in 0..48 {
        let a = base + i * 8;
        match safe_read_u64(a) {
            Some(v) => s.push_str(&format!("  [{:+#06x}] {:#018x}\n", (a as isize) - (item_net as isize), v)),
            None => s.push_str(&format!("  [{:+#06x}] <unreadable>\n", (a as isize) - (item_net as isize))),
        }
    }
    write_log("scrim_explore.txt", &s);

    // ── 3) 바닐라 배열 실제 stride 측정 + 모드템 key 위치 추적 ──
    let read_str_at = |p: usize| -> Option<String> {
        let ptr = safe_read_u64(p + 8)? as usize;
        let len = safe_read_u64(p + 16)? as usize;
        if ptr <= 0x10000 || len == 0 || len > 60 { return None; }
        let mut buf = Vec::new();
        if !safe_read_bytes(ptr, len, &mut buf) { return None; }
        if buf.iter().all(|&b| b == b'_' || b.is_ascii_alphanumeric()) {
            String::from_utf8(buf).ok()
        } else { None }
    };
    let mut a = format!("[{}ms] 아이템 배열 stride 측정 + 모드템 추적\n", now_ms());

    // (3a) db+0x12000 ~ db+0x40000 을 8바이트씩 프로브 → 모든 key 주소 수집 (범위 확대)
    //      바닐라 key(ironsword 등)와 모드템 key(deathblade/radiant_* 등) 위치를 절대주소로 기록.
    let vanilla_first = ["ironsword","dagger","steel_armor","mystic_cloak","arcane_crystal","vital_orb"];
    let mod_baits = ["deathblade","infinity_edge","radiant_infinity_edge","bf_sword",
                     "executioners_calling","radiant_deathblade","blackfire_torch","unending_despair"];
    a.push_str("=== key 주소 스캔 (db+0x12000 ~ +0x60000) ===\n");
    let mut prev_addr: usize = 0;
    let mut count = 0u32;
    let mut off = 0x12000usize;
    while off < 0x60000 {
        let p = db + off;
        if let Some(k) = read_str_at(p) {
            // 아이템 key 처럼 보이는 것만 (라벨/스킬 제외 위해 길이 4+ 이고 't1_0' 류 아님)
            let is_label = k.len() <= 4 && k.as_bytes().get(0) == Some(&b't');
            if !is_label {
                let gap = if prev_addr != 0 { p as isize - prev_addr as isize } else { 0 };
                let mark = if mod_baits.contains(&k.as_str()) { " ★MOD" }
                           else if vanilla_first.contains(&k.as_str()) { " (바닐라카테고리시작)" }
                           else { "" };
                a.push_str(&format!("  db+{:#07x} ({:#x})  gap={:#x}  '{}'{}\n", off, p, gap, k, mark));
                prev_addr = p;
                count += 1;
                if count > 400 { a.push_str("  ...(400 초과, 끊음)\n"); break; }
            }
        }
        off += 8;
    }
    a.push_str(&format!("  총 {}개\n", count));
    write_log("scrim_itemarray.txt", &a);

    // ── 4) 범용 메모리 스캐너: 모드템 key 바이트 찾기(phase1) → 그걸 가리키는 포인터 찾기(phase2) ──
    scan_mod_items(db);
    // ── 5) ★ P3: ItemSetting.mod_items Vec 직독 → idx→ID(30+idx)→key 덤프 (scrim_moditems.txt) ──
    dump_mod_items(db);
}

// 특정 주소범위 [start, start+len) 만 needle 검색.
unsafe fn scan_range<F: FnMut(usize) -> bool>(start: usize, len: usize, needle: &[u8], max_hits: usize, mut found: F) -> u32 {
    let mut hits = 0u32; let mut buf: Vec<u8> = Vec::new();
    let end = start + len; let mut p = start;
    let step_back = needle.len().saturating_sub(1);
    while p < end && (hits as usize) < max_hits {
        let want = (end - p).min(4096);
        if safe_read_bytes(p, want, &mut buf) {
            let n0 = needle[0]; let lim = buf.len().saturating_sub(needle.len()); let mut i = 0usize;
            while i <= lim {
                if buf[i] == n0 && &buf[i..i + needle.len()] == needle {
                    hits += 1;
                    if found(p + i) { return hits; }
                    if (hits as usize) >= max_hits { break; }
                }
                i += 1;
            }
        }
        if want <= step_back { break; }
        p += want - step_back; // 경계 겹침
    }
    hits
}

// 모든 커밋된 읽기영역 순회 스캔 (byte-cap 없음 → 높은 주소까지 도달). progress 파일로 진행상황.
unsafe fn scan_all<F: FnMut(usize, u32, u32) -> bool>(needle: &[u8], max_hits: usize, label: &str, mut found: F) -> u32 {
    let mut addr: usize = 0x10000; let mut hits = 0u32; let mut buf: Vec<u8> = Vec::new();
    let mut scanned: u64 = 0; let mut next_prog: u64 = 0x400_0000; // 64MB 마다 진행로그
    let step_back = needle.len().saturating_sub(1);
    const MEM_COMMIT: u32 = 0x1000;
    const READABLE: u32 = 0x02 | 0x04 | 0x20 | 0x40;
    const BAD: u32 = 0x01 | 0x100;
    while addr < 0x7fff_fff0_0000 && (hits as usize) < max_hits && scanned < 0x8000_0000 {
        let mut mbi = MemBasicInfo::default();
        let n = VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>());
        if n == 0 { break; }
        let region_end = mbi.base.wrapping_add(mbi.region_size);
        let okp = mbi.state == MEM_COMMIT && (mbi.protect & READABLE) != 0 && (mbi.protect & BAD) == 0;
        // 거대 region(≥64MB)은 스킵 — 모드 HashMap 엔트리는 일반 힙(작은 region)에 있음.
        if okp && mbi.region_size > 0 && mbi.region_size < 0x400_0000 {
            let mut p = mbi.base.max(addr);
            while p < region_end && (hits as usize) < max_hits {
                let want = (region_end - p).min(4096);
                if safe_read_bytes(p, want, &mut buf) {
                    let n0 = needle[0]; let lim = buf.len().saturating_sub(needle.len()); let mut i = 0usize;
                    while i <= lim {
                        if buf[i] == n0 && &buf[i..i + needle.len()] == needle {
                            hits += 1;
                            if found(p + i, mbi.protect, mbi.mtype) { return hits; }
                            if (hits as usize) >= max_hits { break; }
                        }
                        i += 1;
                    }
                }
                scanned += want as u64;
                if scanned >= next_prog {
                    write_log("scrim_scan_progress.txt", &format!("[{}ms] {} 진행: {}MB 읽음, 현재 {:#x}, 히트 {}\n",
                        now_ms(), label, scanned / 0x100000, p, hits));
                    next_prog += 0x400_0000;
                }
                if want <= step_back { break; }
                p += want - step_back;
            }
        }
        if region_end <= addr { break; }
        addr = region_end;
    }
    write_log("scrim_scan_progress.txt", &format!("[{}ms] {} 완료: 총 {}MB, 히트 {}\n", now_ms(), label, scanned / 0x100000, hits));
    hits
}

const SCAN_MOD: bool = true;
static SCAN_DONE: AtomicBool = AtomicBool::new(false);
unsafe fn scan_mod_items(_db: usize) {
    if !SCAN_MOD || SCAN_DONE.swap(true, Ordering::Relaxed) { return; }
    seh_install();
    let mut s = format!("[{}ms] 모드템 메모리 스캔\n", now_ms());
    let modh = GetModuleHandleW("riot_items_tfm2.dll".encode_utf16().chain([0]).collect::<Vec<u16>>().as_ptr()) as usize;
    s.push_str(&format!("  riot_items_tfm2.dll base = {:#x}\n", modh));
    if modh == 0 { s.push_str("  모듈 로드 안됨 (riot 모드 꺼짐?)\n"); write_log("scrim_modscan.txt", &s); return; }

    let targets = ["radiant_infinity_edge", "deathblade", "executioners_calling"];
    // Phase1: 세 key 모두 DLL 모듈 범위만 스캔 (즉시) → 리터럴 주소
    let mut first_lit: Option<usize> = None;
    for t in targets {
        s.push_str(&format!("=== '{}' (P1) ===\n", t));
        let mut found_any = false;
        scan_range(modh, 0x60000, t.as_bytes(), 4, |a| {
            s.push_str(&format!("  [P1] DLL내 @ {:#x} (RVA {:#x})\n", a, a - modh));
            if first_lit.is_none() { first_lit = Some(a); }
            found_any = true; false
        });
        if !found_any { s.push_str("  [P1] DLL 범위에서 못 찾음\n"); }
    }
    // Phase2: 첫 리터럴(radiant_infinity_edge) 하나만 전메모리 포인터 검색 (1회) → HashMap 엔트리
    if let Some(la) = first_lit {
        let needle = (la as u64).to_le_bytes();
        s.push_str(&format!("=== [P2] {:#x} 가리키는 포인터 전메모리 검색 (1회) ===\n", la));
        scan_all(&needle, 5, "P2", |pp, prot, mty| {
            let entry = pp.wrapping_sub(8);
            s.push_str(&format!("  [P2] ptr@ {:#x} (prot={:#x} type={:#x}) 엔트리 {:#x}\n", pp, prot, mty, entry));
            // (ptr@off, len@off+8) 이 유효 key &str 인지
            let key_at = |pa: usize| -> Option<String> {
                let ptr = safe_read_u64(pa)? as usize;
                let len = safe_read_u64(pa + 8)? as usize;
                if ptr <= 0x10000 || len < 3 || len > 40 { return None; }
                let mut b = Vec::new();
                if safe_read_bytes(ptr, len, &mut b)
                    && b.iter().all(|&x| x == b'_' || x.is_ascii_alphanumeric()) && b[0].is_ascii_alphabetic() {
                    std::str::from_utf8(&b).ok().map(|x| x.to_string())
                } else { None }
            };
            // 엔트리 -0x10 ~ +0x800 에서, &str 쌍이 연속 3개+ 이어지는 "배열" 시작점 찾기
            let mut best_start: Option<usize> = None;
            let mut off = -0x10isize;
            while off < 0x800 {
                let a0 = (entry as isize + off) as usize;
                if key_at(a0).is_some() && key_at(a0 + 0x10).is_some() && key_at(a0 + 0x20).is_some() {
                    best_start = Some(a0); break;
                }
                off += 8;
            }
            match best_start {
                None => s.push_str("    (연속 &str 배열 없음)\n"),
                Some(start0) => {
                    // 시작점에서 뒤로 확장 (앞에 더 있으면 포함)
                    let mut start = start0;
                    let mut back = 0u32;
                    while start > entry.saturating_sub(0x100) && back < 64 {
                        if key_at(start - 0x10).is_some() { start -= 0x10; back += 1; } else { break; }
                    }
                    s.push_str(&format!("    ▼ (ptr,len) &str 배열 @ {:#x} (엔트리+{:#x}) 끊길때까지:\n", start, start.wrapping_sub(entry)));
                    let mut a = start; let mut idx = 0u32; let mut blanks = 0u32;
                    while idx < 200 {
                        match key_at(a) {
                            Some(k) => { s.push_str(&format!("      [{:3}] {:#x} '{}'\n", idx, a, k)); blanks = 0; }
                            None => { s.push_str(&format!("      [{:3}] {:#x} (빈칸)\n", idx, a)); blanks += 1; if blanks >= 4 { s.push_str("      (연속 4빈칸 → 끝)\n"); break; } }
                        }
                        a += 0x10; idx += 1;
                    }
                }
            }
            false
        });
    }
    write_log("scrim_modscan.txt", &s);
}

const SCAN_MODITEMS: bool = true;
static MODITEMS_DONE: AtomicBool = AtomicBool::new(false);
// ★ 메모리 레지스트리 캐시: idx i → mod_items[i].key (게임 ID = 30+i). dump_mod_items 가 서버시작때 1회 채움.
//   commit_to_id/mod_final_ids 가 SEEN(관리틱 필요) 대신 이걸 우선 사용 → 일정 없이 즉시.
static MOD_REGISTRY: Mutex<Vec<String>> = Mutex::new(Vec::new());
// ★ 동적 최종템 ID 목록 (next_tier 빈 모드템). dump_mod_items 가 트리분석으로 채움. radiant 하드코딩 대체.
static MOD_FINALS: Mutex<Vec<u64>> = Mutex::new(Vec::new());
// ★ ModItemEntry 내 next_tier(Vec<String>) 바이트 오프셋 (런타임 프로브로 확정, 0=미확정).
static NT_OFFSET: AtomicUsize = AtomicUsize::new(0);
// ★ P3 v3: mod_items Vec 실덤프 → idx→ID(30+idx)→key (일정/SEEN 없이 즉시).
//   확정사실(P3v2 scrim_moditems): mod_items 헤더 = db+0x15d78 형태 [len=39, ptr=buf, cap=39],
//     buf 의 element 는 key String 을 element+0x8 (ptr) 에 보유, firstkey='executioners_calling'(ID30).
//     비최종21 + 최종18(radiant_*) = 39. 헤더는 힙이동 대비 런타임 재탐색.
//   알고리즘:
//     1) 헤더 탐색: db+0..0x60000 에서 (ptr,cnt) 중 cnt∈35..45 & key_at(ptr+8)=비바닐라 lowercase.
//     2) stride 자동탐지: 후보 stride 들 중 "전부(≥37) 유효 & radiant_ 16~20개" 시그니처 만족.
//     3) 39개 idx→ID(30+idx)→key 덤프. 실패 시 stride 점수표 + 버퍼 raw 진단.
//   ※ 반드시 safe_read_* 만 사용 (과거 rd_u64 비보호 직독이 프리징 원인 → 폐기됨).
//   (헤더 firstkey 검증은 기존 const VANILLA_KEYS[] 재사용 — executioners_calling 은 비포함→비바닐라로 판정)
unsafe fn dump_mod_items(db: usize) {
    if !SCAN_MODITEMS || MODITEMS_DONE.swap(true, Ordering::Relaxed) { return; }
    seh_install();
    let mut s = format!("[{}ms] ★ P3v9 mod_items (walk+정확key+트리분석) (db={:#x})\n", now_ms(), db);

    // pa 에 String(ptr@pa) 가정 → ptr 따라가 ascii item-key 자체측정.
    let key_at = |pa: usize| -> Option<String> {
        let ptr = safe_read_u64(pa)? as usize;
        if ptr <= 0x10000 { return None; }
        for &m in &[64usize, 32, 16, 8] {
            let mut b = Vec::new();
            if !safe_read_bytes(ptr, m, &mut b) { continue; }
            let mut v = Vec::new();
            for &c in b.iter() {
                if c == b'_' || c.is_ascii_alphanumeric() { v.push(c); } else { break; }
            }
            // 키 첫글자는 영문(대/소문자) — more_item 신규템 키 'A_1_1' 등 대문자시작 허용.
            if v.len() >= 3 && (v[0] as char).is_ascii_alphabetic() { return String::from_utf8(v).ok(); }
        }
        None
    };

    // 바닐라 배열 판별 (firstkey). 실제 메모리 key[0]="ironsword" (serde명 "iron_blade"와 다름) → 둘 다 제외.
    let is_vanilla = |k: &str| k == "ironsword" || VANILLA_KEYS.contains(&k);
    // ★ ModItemEntry/BaseItemInfo 구조체 크기군. 어떤 아이템 모드든 동일 stride(게임 고정구조) →
    //   모드 개수·radiant 비율과 무관하게 탐지. 챔피언 등 다른 크기 배열은 자동 배제.
    let item_strides: [usize; 3] = [0x1a8, 0x198, 0x1b0];

    // 버퍼가 item-struct 배열인지: item_strides 중 entry0..3 가 유효+서로다른 key → 그 stride, 아니면 0.
    let detect_stride = |buf: usize| -> usize {
        for &st in item_strides.iter() {
            let k: Vec<Option<String>> = (0..4).map(|i| key_at(buf + i * st + 0x8)).collect();
            if k.iter().all(|x| x.is_some()) && k[0] != k[1] && k[1] != k[2] && k[2] != k[3] {
                return st;
            }
        }
        0
    };

    // ── 1) 헤더 전수 스캔: 모든 비바닐라 item-struct 배열 수집 (buf 기준 dedup, 별도 Vec 다중모드 탐지) ──
    let mut found: Vec<(usize, usize, usize, usize)> = Vec::new(); // (buf, cnt, stride, hdr)
    let mut o = 0usize;
    while o + 0x18 <= 0x60000 && found.len() < 16 {
        let a = db + o; o += 8;
        let (Some(q0), Some(q1), Some(q2)) = (safe_read_u64(a), safe_read_u64(a + 8), safe_read_u64(a + 0x10)) else { continue; };
        // 레이아웃 [len,ptr,cap]→(q1,q0); [ptr,cap,len]→(q0,q2); [ptr,len,cap]→(q0,q1)
        for &(p, c) in [(q1, q0), (q1, q2), (q0, q2), (q0, q1)].iter() {
            let (p, c) = (p as usize, c as usize);
            if !looks_heap(p as u64) || c < 3 || c > 2000 { continue; }
            let Some(k0) = key_at(p + 0x8) else { continue; };
            if is_vanilla(&k0) { continue; }
            let cst = detect_stride(p);
            if cst == 0 { continue; }
            // 유효율 검증 (앞 min(cnt,48)개의 80%+ 가 유효 key)
            let probe = c.min(48);
            let valid = (0..probe).filter(|&i| key_at(p + i * cst + 0x8).is_some()).count();
            if valid * 10 < probe * 8 || valid < 3 { continue; }
            if found.iter().any(|&(b, _, _, _)| b == p) { continue; } // 복사본/중복 dedup
            found.push((p, c, cst, a));
        }
    }
    if found.is_empty() {
        s.push_str("  ✗ 비바닐라 item-struct 배열(stride 0x1a8/0x198) 못 찾음. (모드 아이템 미적용? 구조변경?)\n");
        write_log("scrim_moditems.txt", &s); return;
    }

    // ── 2) 발견한 모든 배열 요약 (별도 Vec = 분리형 다중모드 단서) ──
    found.sort_by(|x, y| y.1.cmp(&x.1)); // cnt 내림차순
    s.push_str(&format!("=== 발견한 비바닐라 item-배열 {}개 (서로 다른 buf=별개 모드 가능) ===\n", found.len()));
    for &(b, c, cst, h) in found.iter() {
        let rad = (0..c).filter(|&i| key_at(b + i * cst + 0x8).map_or(false, |k| k.starts_with("radiant"))).count();
        let k0 = key_at(b + 0x8).unwrap_or_default();
        let kl = key_at(b + (c - 1) * cst + 0x8).unwrap_or_default();
        s.push_str(&format!("  hdr=db+{:#x} buf={:#x} cnt={} stride={:#x} radiant={} first='{}' last='{}'\n", h - db, b, c, cst, rad, k0, kl));
    }

    // ── 3) 최대 배열 채택 → 덤프 + 레지스트리 (분리형이면 추후 병합 로직 필요) ──
    // element = key String {len@+0, ptr@+8, cap@+0x10}. 길이필드로 정확히 읽어 꼬리글자 제거.
    let key_of_elem = |elem: usize| -> Option<String> {
        let a = safe_read_u64(elem)? as usize;
        let ptr = safe_read_u64(elem + 8)? as usize;
        let c = safe_read_u64(elem + 0x10)? as usize;
        let len = a.min(c); // len <= cap
        if ptr <= 0x10000 || len < 2 || len > 48 { return None; }
        let mut b = Vec::new();
        if !safe_read_bytes(ptr, len, &mut b) { return None; }
        if b.iter().all(|&x| x == b'_' || x.is_ascii_alphanumeric()) && (b[0] as char).is_ascii_alphabetic() {
            String::from_utf8(b).ok()
        } else { None }
    };
    let (buf, hdr_cnt, st, hdr) = found[0];
    // 개수 = idx0부터 유효 key 연속개수 walk (헤더 cnt 는 cap/오염 가능). merge형 클린배열 정확대응.
    let mut cnt = 0usize;
    while cnt < hdr_cnt.max(1) && cnt < 500 {
        if key_of_elem(buf + cnt * st).is_some() { cnt += 1; } else { break; }
    }
    let radiant = (0..cnt).filter(|&i| key_of_elem(buf + i * st).map_or(false, |k| k.starts_with("radiant"))).count();
    s.push_str(&format!("  [채택] 헤더=db+{:#x} buf={:#x} hdr_cnt={} real={} stride={:#x} radiant={}\n", hdr - db, buf, hdr_cnt, cnt, st, radiant));
    s.push_str("  idx | ID | key\n");
    let mut reg = MOD_REGISTRY.lock().unwrap();
    reg.clear();
    for i in 0..cnt {
        let k = key_of_elem(buf + i * st).unwrap_or_default();
        let tag = if k.starts_with("radiant") { " ←최종(radiant)" } else { "" };
        s.push_str(&format!("  {:>3} | {:>3} | {}{}\n", i, 30 + i, k, tag));
        reg.push(k);
    }
    let keys: Vec<String> = reg.clone();
    drop(reg);
    s.push_str(&format!("  → MOD_REGISTRY {}개 캐시됨 (commit_to_id 즉시구동). radiant={}개\n", cnt, radiant));
    write_log("scrim_moditems.txt", &s);

    // ── 4) next_tier 오프셋 탐지 + 최종템 동적판별 + 트리 덤프 (radiant 하드코딩 제거 근거) ──
    //   element 내 elem+o = Vec<String>{len@o, ptr@o+8, cap@o+0x10}. 비어있지않고 전부 레지스트리키 =next_tier.
    //   tags 도 Vec<String> 이나 'AD'/'HP' 등 비-키라 레지스트리 불포함 → 구분됨.
    let in_reg = |k: &str| keys.iter().any(|x| x.as_str() == k);
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
    // 오프셋 합의: 비어있지않고 전부 레지스트리키인 Vec<String> 가 가장 많은 offset = next_tier
    let mut best_off = 0usize; let mut best_votes = 0u32;
    let mut o = 0x18usize;
    while o + 0x18 <= st {
        let mut votes = 0u32;
        for i in 0..cnt {
            if let Some(v) = read_nt(buf + i * st, o) {
                if !v.is_empty() && v.iter().all(|k| in_reg(k)) { votes += 1; }
            }
        }
        if votes > best_votes { best_votes = votes; best_off = o; }
        o += 8;
    }
    let mut tree = format!("[{}ms] ★ 아이템 트리 분석 (next_tier 동적탐지, db={:#x})\n", now_ms(), db);
    if best_off == 0 || best_votes < 3 {
        tree.push_str(&format!("  ✗ next_tier 오프셋 미확정 (best_off={:#x} votes={})\n", best_off, best_votes));
        write_log("scrim_itemtree.txt", &tree);
        return;
    }
    NT_OFFSET.store(best_off, Ordering::Relaxed);
    tree.push_str(&format!("  next_tier offset = +{:#x} (votes={}/{}). 최종템 = next_tier 빈 것.\n", best_off, best_votes, cnt));
    tree.push_str("  idx | ID | key | next_tier\n");
    let mut finals: Vec<u64> = Vec::new();
    for i in 0..cnt {
        let elem = buf + i * st;
        let k = key_of_elem(elem).unwrap_or_default();
        let nt = read_nt(elem, best_off).unwrap_or_default();
        if nt.is_empty() {
            finals.push(30 + i as u64);
            tree.push_str(&format!("  {:>3} | {:>3} | {} | ★최종\n", i, 30 + i, k));
        } else {
            tree.push_str(&format!("  {:>3} | {:>3} | {} | → {}\n", i, 30 + i, k, nt.join(", ")));
        }
    }
    tree.push_str(&format!("  → 동적 최종템 {}개 (ID): {:?}\n", finals.len(), finals));
    *MOD_FINALS.lock().unwrap() = finals;
    write_log("scrim_itemtree.txt", &tree);
}

// ===========================================================================
// ★ itemnet_capture = 순수 진단 훅(캡처→로그만; 실기능 자동빌드 compute_build_b/itemnet_forward 와 무관).
//   게임 아이템빌드 forward 함수를 후킹 → 날짜넘김 병렬 sim(rayon 워커스레드)의 매 빌드마다 발화 →
//   readable()+raw-read TOCTOU AV(동시 alloc/free 중 check→read 사이 decommit) → 그 AV는 워커스레드라
//   seh_veh 가 못 잡음(SEH는 arming tid 단일스레드만 보호) → **간헐 하드크래시**. ⟹ 배포=false(설치 안함).
const ITEMNET_CAPTURE_HOOK: bool = false;
static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
static CAPTURE_DONE: AtomicBool = AtomicBool::new(false);
static CAPTURE_SEQ: AtomicU64 = AtomicU64::new(0); // 캡처 횟수 (새 캡처 감지)
static ITEMNET_CALL_TOTAL: AtomicU64 = AtomicU64::new(0); // ★ 신경망 forward 총 호출 횟수 (중복 포함)
static CAPTURED_CTX: Mutex<[u64; 32]> = Mutex::new([0u64; 32]);
static CAPTURED_STACK: Mutex<[u64; 24]> = Mutex::new([0u64; 24]); // 진입시점 스택 덤프
static LAST_CAP_KEY: AtomicU64 = AtomicU64::new(0); // 직전 캡처 식별 (중복 방지)
static SEEN_ITEM_IDS: Mutex<Vec<u64>> = Mutex::new(Vec::new()); // ★ forward 에 등장한 모든 아이템 ID 누적(진단)
static SEEN_IDS_LOGGED_LEN: AtomicUsize = AtomicUsize::new(0);
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
    if !ITEMNET_CAPTURE_HOOK { return; } // 진단훅 비활성: 설치돼도(혹시) 즉시 반환 = 워커스레드 raw-read 안함
    ITEMNET_CALL_TOTAL.fetch_add(1, Ordering::Relaxed); // ★ 모든 호출 카운트 (중복 포함)
    if saved_ptr < 0x10000 { return; }
    let item_net = *((saved_ptr + 0x28) as *const u64); // rcx
    let ctx_ptr = *((saved_ptr + 0x20) as *const u64) as usize; // rdx
    let cands = *((saved_ptr + 0x18) as *const u64); // r8
    let n = *((saved_ptr + 0x10) as *const u64); // r9
    // ★ 진단: ctx dedup 과 무관하게 모든 forward 후보 ID 누적 (모드템 30+ 포착)
    if cands > 0x10000 {
        let nn = (n as usize).min(64);
        if let Ok(mut seen) = SEEN_ITEM_IDS.try_lock() {
            for i in 0..nn {
                if readable(cands as usize + i*8, 8) {
                    let id = *((cands as usize + i*8) as *const u64);
                    if id < 100000 && !seen.contains(&id) { seen.push(id); }
                }
            }
        }
    }
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
    *CAPTURED_CTX.lock().unwrap_or_else(|e| e.into_inner()) = buf; // poison-safe(트램폴린 콜백=panic 시 FFI UB 방지)
    // 호출 인자 저장 + cands 배열 내용 (n개, 최대 3개)
    let mut args = [0u64; 8];
    args[0] = item_net; args[1] = ctx_ptr as u64; args[2] = cands; args[3] = n; args[4] = flag;
    if cands > 0x10000 {
        for i in 0..3.min(n as usize) {
            if readable(cands as usize + i*8, 8) { args[5+i] = *((cands as usize + i*8) as *const u64); }
        }
    }
    *CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner()) = args;
    // ★ cands 전체를 캡처 순간에 복사 (나중에 읽으면 메모리 재사용돼 쓰레기됨)
    {
        let mut cc = [0u64; 32];
        let nn = (n as usize).min(32);
        if cands > 0x10000 {
            for i in 0..nn {
                if readable(cands as usize + i*8, 8) { cc[i] = *((cands as usize + i*8) as *const u64); }
            }
        }
        *CAPTURED_CANDS.lock().unwrap_or_else(|e| e.into_inner()) = cc;
        CAPTURED_CANDS_N.store(n, Ordering::Relaxed);
    }
    // 진입시점 스택 덤프 (rsp[0]=리턴주소, 위로 호출처 지역변수)
    let mut st = [0u64; 24];
    if entry_rsp > 0x10000 {
        for i in 0..24 {
            if readable(entry_rsp + i*8, 8) { st[i] = *((entry_rsp + i*8) as *const u64); }
        }
    }
    *CAPTURED_STACK.lock().unwrap_or_else(|e| e.into_inner()) = st;
    LAST_CAP_KEY.store(key, Ordering::Relaxed);
    CAPTURE_SEQ.fetch_add(1, Ordering::Relaxed);
    CAPTURE_DONE.store(true, Ordering::Relaxed);
}

// 트램폴린 설치: FUN 첫 12B 백업 → stub 생성 → FUN 첫 12B를 jmp stub 으로 패치
unsafe fn install_itemnet_hook() -> Result<usize, &'static str> {
    if HOOK_INSTALLED.swap(true, Ordering::Relaxed) { return Err("이미 설치됨"); }
    let mbase = GetModuleHandleW(core::ptr::null()) as usize;
    if mbase == 0 { return Err("module base 0"); }
    let fn_addr = mbase + ITEMNET_FORWARD_RVA;
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
// ★ 배포 스위치: false 면 디버그 로그(.txt/UI_Dump) 일절 안 씀. (선택저장/설정읽기 등 기능 파일은 영향 없음)
const LOG_ENABLED: bool = false;
fn write_log(name: &str, content: &str) {
    if !LOG_ENABLED { return; }
    let Some(p) = dll_path().and_then(|p| p.parent().map(|d| d.join(name))) else { return; };
    let _ = fs::write(p, content);
}
// ★ create-new 전용 always-on 로거 (LOG_ENABLED 무관 — 프로토타입 관찰용)
fn cn_log(line: &str) {
    use std::io::Write;
    let Some(p) = dll_path().and_then(|p| p.parent().map(|d| d.join("scrim_createnew.txt"))) else { return; };
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(p) {
        let _ = writeln!(f, "[{}ms] {}", now_ms(), line);
    }
}
fn append_log(name: &str, line: &str) {
    if !LOG_ENABLED { return; }
    use std::io::Write;
    let Some(p) = dll_path().and_then(|p| p.parent().map(|d| d.join(name))) else { return; };
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(p) {
        let _ = writeln!(f, "{}", line);
    }
}
// ★ UI_Dump/ 하위에 순번 파일(ui_dump_N.txt)로 저장. 디렉터리 없으면 생성. (DLL 폴더 기준)
fn dump_seq_write(seq: u64, content: &str) {
    if !LOG_ENABLED { return; }
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
    // ★ 팀전술(STRAT_COMMITTED 2×12 + STRAT_SPLIT_POS 2×3)·아이템전술(ITEMS_COMMITTED 10×3)도 영속
    let strat = *STRAT_COMMITTED.lock().unwrap();
    let split = *STRAT_SPLIT_POS.lock().unwrap();
    let items = *ITEMS_COMMITTED.lock().unwrap();
    let strat_s: Vec<String> = strat.iter().flat_map(|row| row.iter()).map(|v| v.to_string()).collect();
    let split_s: Vec<String> = split.iter().flat_map(|t| [t.0, t.1, t.2]).map(|v| v.to_string()).collect();
    let items_s: Vec<String> = items.iter().flat_map(|row| row.iter()).map(|v| v.to_string()).collect();
    // ★ 기능 파일(설정 영속) — LOG_ENABLED 와 무관하게 항상 저장. write_log 우회.
    if let Some(path) = dll_path().and_then(|d| d.parent().map(|dir| dir.join("scrim_selection.txt"))) {
        let _ = fs::write(path, format!(
            "players:{}\nchamps:{}\nstrat:{}\nsplit:{}\nitems:{}\n",
            ps.join(","), cs.join(","), strat_s.join(","), split_s.join(","), items_s.join(",")));
    }
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
        } else if let Some(v) = line.strip_prefix("strat:") {
            let nums: Vec<u8> = v.split(',').filter_map(|t| t.trim().parse::<u8>().ok()).collect();
            if nums.len() == 24 {
                let mut s = STRAT_COMMITTED.lock().unwrap();
                for i in 0..24 { s[i / 12][i % 12] = nums[i]; }
            }
        } else if let Some(v) = line.strip_prefix("split:") {
            let nums: Vec<u8> = v.split(',').filter_map(|t| t.trim().parse::<u8>().ok()).collect();
            if nums.len() == 6 {
                let mut sp = STRAT_SPLIT_POS.lock().unwrap();
                for t in 0..2 { sp[t] = (nums[t * 3], nums[t * 3 + 1], nums[t * 3 + 2]); }
            }
        } else if let Some(v) = line.strip_prefix("items:") {
            let nums: Vec<u8> = v.split(',').filter_map(|t| t.trim().parse::<u8>().ok()).collect();
            if nums.len() == 30 {
                let mut it = ITEMS_COMMITTED.lock().unwrap();
                for i in 0..30 { it[i / 3][i % 3] = nums[i]; }
            }
        }
    }
}
// 현재 로스터/챔프 목록에 없는 선택은 비움 (트레이드/다른 세이브 대비). 중복도 정리.
fn validate_selection() {
    let valid_p: Vec<usize> = ROSTER.lock().unwrap().iter().map(|(id, _)| *id).collect();
    let valid_c: Vec<String> = CHAMPS.lock().unwrap().iter().map(|(k, _)| k.clone()).collect();
    // 선수 중복 허용 — 유효성(현재 로스터에 있는지)만 검사, 중복 dedup 안 함
    let mut p = PLAYER_SLOTS.lock().unwrap();
    for s in p.iter_mut() {
        if let Some(id) = *s {
            if !valid_p.contains(&id) { *s = None; }
        }
    }
    // 챔프는 중복 허용 — 유효성(현재 챔프목록에 있는지)만 검사, 중복 dedup 안 함
    let mut c = CHAMP_SLOTS.lock().unwrap();
    for s in c.iter_mut() {
        if let Some(k) = s.clone() {
            if !valid_c.contains(&k) { *s = None; }
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
// 풀다운(scrim_ddp) 높이 조절: contents(스크롤 내용)=전체 행, 그리고 항목 적으면 패널/리스트도 축소.
// 행 40 + 간격 6 = 46/행. 폭/행 크기는 .ui 고정(런타임 폭 override 없음) → hover 안정.
// 높이 쓰기는 ui_kit::set_layout_size_by_id(폭 0 = 유지) 공용 헬퍼 사용.
fn set_pulldown_contents(root: &mut Node, len: usize) {
    let p = dd_prefix(); // scrim_ddp(선수/아이템) 또는 scrim_dds(전술/스플릿) — 폭만 다르고 높이 로직 동일
    let stride = 46.0_f32;
    let content_h = (len as f32 * stride - 6.0).max(40.0); // 전체 행 높이(마지막 뒤 간격 제외)
    ui_kit::set_layout_size_by_id(root, &format!("{}_contents", p), 0.0, content_h);
    // 5개 이하: 내용 높이만큼 패널/리스트 축소 → 빈 공간·스크롤 영역 제거. 6개 이상: 선언값(264/284) + 스크롤.
    // (풀다운 노드 재사용이라 6개 이상 경우 선언값으로 반드시 되돌려야 이전 축소가 안 남음)
    if len <= 5 {
        ui_kit::set_layout_size_by_id(root, &format!("{}_list", p), 0.0, content_h);          // 리스트=내용(스크롤 없음)
        ui_kit::set_layout_size_by_id(root, &format!("{}_panel", p), 0.0, content_h + 20.0);  // 상하 10px 여백
    } else {
        ui_kit::set_layout_size_by_id(root, &format!("{}_list", p), 0.0, 264.0);
        ui_kit::set_layout_size_by_id(root, &format!("{}_panel", p), 0.0, 284.0);
    }
}
// 네이티브 dropdown 에 옵션 주입 (FUN_1421184e0 ABI 호출). 옵션 String 은 leak(게임이 보관).
// 네이티브 dropdown set 함수(FN_DD_SETOPT_RVA)의 프롤로그 검증·캐시 (itemnet 과 독립).
static DD_VALID: AtomicU64 = AtomicU64::new(0);
unsafe fn prologue_hex(addr: usize, n: usize) -> String {
    if !readable(addr, n) { return "UNREADABLE".to_string(); }
    (0..n).map(|i| format!("{:02x}", *((addr + i) as *const u8))).collect::<Vec<_>>().join(" ")
}
unsafe fn dd_addr_valid() -> bool {
    match DD_VALID.load(Ordering::Relaxed) { 1 => return true, 2 => return false, _ => {} }
    let fa = GetModuleHandleW(core::ptr::null()) as usize + FN_DD_SETOPT_RVA;
    let expect = [0x55u8, 0x56, 0x57, 0x48, 0x83, 0xec, 0x70]; // push rbp/rsi/rdi; sub rsp,0x70 (0.4.13 native dropdown)
    let mut ok = readable(fa, 7);
    if ok { for i in 0..7 { if *((fa + i) as *const u8) != expect[i] { ok = false; break; } } }
    DD_VALID.store(if ok { 1 } else { 2 }, Ordering::Relaxed);
    ok
}

unsafe fn nat_dd_set_options(root: &Node, target: &str, items: &[&str], sel: u64) -> bool {
    if !dd_addr_valid() { return false; } // dropdown set 함수 RVA 검증(itemnet 가드와 분리)
    fn find_rb(n: &Node, t: &str) -> Option<usize> {
        if n.id.as_str() == t { return Some(unsafe { runner_base(n) }); }
        for c in n.child.iter() { if let Some(b) = find_rb(c, t) { return Some(b); } }
        None
    }
    let Some(rb) = find_rb(root, target) else { return false; };
    let mut opts: Vec<DdOpt> = Vec::with_capacity(items.len());
    for &it in items {
        let s = it.to_string();
        opts.push(DdOpt {
            // 색상 = RGBA float 4개. 흰색: R=G=B=A=1.0 (0x3f800000)
            color: 0x3f800000_3f800000, // R@0=1.0, G@4=1.0
            color2: 0x3f800000,          // B@8=1.0
            alpha: 1.0,                  // A@12=1.0
            s_len: s.len(), s_ptr: s.as_ptr() as usize, s_cap: s.capacity(),
        });
        std::mem::forget(s); // 게임이 String 소유 → leak
    }
    // param_3 = {capacity, ptr, len} (어셈블리 확인). [0]=0 → 게임이 입력배열 free 안함(leak 안전)
    let param3: [usize; 3] = [0, opts.as_ptr() as usize, opts.len()];
    let addr = GetModuleHandleW(std::ptr::null()) as usize + FN_DD_SETOPT_RVA;
    let f: unsafe extern "system" fn(usize, u64, *const [usize; 3]) = std::mem::transmute(addr);
    f(rb, sel, &param3);
    std::mem::forget(opts);
    // 검증(디버그): 호출 후 dropdown 의 옵션 Vec(+0x1528) / 선택(+0x1788) 상태
    if LOG_ENABLED {
        let p = rb as *const u8;
        let s = format!(
            "after set_options:\n  sel_idx(+0x1788) = {}\n  opt Vec(+0x1528): ptr={:#x} cap={} len={}\n",
            *(p.add(0x1788) as *const u64),
            *(p.add(0x1528) as *const usize),
            *(p.add(0x1530) as *const usize),
            *(p.add(0x1538) as *const usize));
        write_log("scrim_nat_dd_verify.txt", &s);
    }
    true
}

// ── 인라인 선수 dropdown 상태 (모달 안 선수슬롯 10개를 네이티브 dropdown 으로) ──
static PDD_INIT: AtomicBool = AtomicBool::new(false);
static PDD_LAST: Mutex<[i64; 10]> = Mutex::new([-1i64; 10]);
// ── 인라인 아이템 dropdown 상태 (개인전술 모달 30슬롯 = 선수10×아이템3) ──
static IDD_INIT: AtomicBool = AtomicBool::new(false);
static IDD_LAST: Mutex<[i64; 30]> = Mutex::new([-1i64; 30]);
// ── 인라인 팀전술 dropdown 상태 (12필드, 블루/레드 팀전환 시 재주입) ──
static SDD_INIT: AtomicBool = AtomicBool::new(false);
static SDD_LAST: Mutex<[i64; 12]> = Mutex::new([-1i64; 12]);
static SDD_TEAM: AtomicU8 = AtomicU8::new(255); // 직전 표시팀(255=무효 → 첫 프레임 재주입)

// 네이티브 dropdown 현재 선택 인덱스(runner+0x1788). u64::MAX(미선택)면 None.
unsafe fn nat_dd_selected(root: &Node, target: &str) -> Option<usize> {
    if !dd_addr_valid() { return None; } // dropdown 함수 RVA 검증(itemnet 가드와 분리)
    fn find_rb(n: &Node, t: &str) -> Option<usize> {
        if n.id.as_str() == t { return Some(unsafe { runner_base(n) }); }
        for c in n.child.iter() { if let Some(b) = find_rb(c, t) { return Some(b); } }
        None
    }
    let rb = find_rb(root, target)?;
    let v = *((rb + 0x1788) as *const u64);
    if v == u64::MAX { None } else { Some(v as usize) }
}

// 진단: 노드 id 의 runner_base + 타입명 (DropdownRunner 인지 확인용)
fn pdd_diag_node(root: &Node, id: &str) -> Option<(usize, String)> {
    fn rec(n: &Node, id: &str) -> Option<(usize, String)> {
        if n.id.as_str() == id { return Some((unsafe { runner_base(n) }, n.runner.type_name().to_string())); }
        for c in n.child.iter() { if let Some(x) = rec(c, id) { return Some(x); } }
        None
    }
    rec(root, id)
}
// 디버그: 특정 노드의 runner 인스턴스 메모리(0..size)를 u64 + 포인터역참조(ASCII)로 덤프.
// DropdownRunner 의 옵션 Vec / 선택 idx 필드를 찾기 위함.
fn dump_runner_hex(n: &Node, type_match: &str, size: usize, out: &mut String) -> bool {
    if n.runner.type_name().contains(type_match) {
        let base = unsafe { runner_base(n) };
        out.push_str(&format!("id={} runner_base={:#x} type={}\n", n.id.as_str(), base, n.runner.type_name()));
        for off in (0..size).step_by(8) {
            let v = unsafe { *((base + off) as *const u64) };
            out.push_str(&format!("  +{:<4} {:#018x}", off, v));
            // 힙 포인터처럼 보이면 역참조해서 16바이트 ASCII (String/Vec 내용 추정)
            if v > 0x10000 && v < 0x0000_7fff_ffff_ffff && unsafe { IsBadReadPtr(v as usize, 24) } == 0 {
                let ascii: String = (0..24).map(|i| {
                    let b = unsafe { *((v as *const u8).add(i)) };
                    if (32..127).contains(&b) { b as char } else { '.' }
                }).collect();
                out.push_str(&format!("  ->[{}]", ascii));
            }
            out.push('\n');
        }
        out.push('\n');
        return true;
    }
    for c in n.child.iter() { if dump_runner_hex(c, type_match, size, out) { return true; } }
    false
}
// 디버그: 전체 노드 트리를 id/타입/visible/layout/rect 와 함께 덤프 (F8)
fn dump_tree(n: &Node, depth: usize, out: &mut String) {
    let b = n as *const Node as *const u8;
    let f = |o: usize| unsafe { *(b.add(o + LEN_F32_OFF) as *const f32) };
    let r = |o: usize| unsafe { *(b.add(576 + o) as *const f32) };
    let ty = n.runner.type_name();
    let ty = ty.rsplit("::").next().unwrap_or(ty);
    out.push_str(&format!("{}{} [{}] vis={} xy=({:.0},{:.0}) wh=({:.0},{:.0}) rect=[{:.0},{:.0},{:.0},{:.0}]\n",
        "  ".repeat(depth.min(24)), n.id.as_str(), ty, n.visible,
        f(128), f(136), f(112), f(120), r(0), r(4), r(8), r(12)));
    for c in n.child.iter() { dump_tree(c, depth + 1, out); }
}
// 디버그: 노드의 layout(x/y/w/h) + rect 를 문자열로 덤프 (풀다운 위치 보정용)
fn dump_node_geom(n: &Node, target: &str, out: &mut String) -> bool {
    if n.id.as_str() == target {
        let b = n as *const Node as *const u8;
        unsafe {
            let f = |o: usize| *(b.add(o + LEN_F32_OFF) as *const f32);
            let r = |o: usize| *(b.add(576 + o) as *const f32);
            out.push_str(&format!("{}: layout x={:.1} y={:.1} w={:.1} h={:.1} | rect=[{:.1},{:.1},{:.1},{:.1}]\n",
                target, f(128), f(136), f(112), f(120), r(0), r(4), r(8), r(12)));
        }
        return true;
    }
    for c in n.child.iter() { if dump_node_geom(c, target, out) { return true; } }
    false
}
// 가상 스크롤: 전체 명단의 off..off+NAT_WINDOW 만 dropdown 에 주입
fn inject_nat_window(root: &Node, off: usize) {
    let all = NAT_ROSTER.lock().unwrap();
    let end = (off + NAT_WINDOW).min(all.len());
    if off >= end { return; }
    let window: Vec<&str> = all[off..end].iter().map(|s| s.as_str()).collect();
    NAT_DD.set_options(root, &window, 0);
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

// ===========================================================================
//  ★ create-new: 기존 완료 Practice 리플레이를 템플릿으로 clone → 새 match_id/key.
//   반환: (new_mid, new_key, 복제·수정된 MatchReplayData, 복제·수정된 MatchInfo).
//   호출측이 db_mut().match_replays.insert(new_key, replay) + matches.insert(Practice{new_mid}, info).
//   이후 apply_lineup(new_key) 로 우리 라인업 주입 → 새 매치로 재생.
// ===========================================================================
fn build_new_practice(r: &ClientDatabase) -> Option<(u64, usize, MatchReplayData, MatchInfo)> {
    let mut max_mid: u64 = 0;
    let mut max_key: usize = 0;
    let mut tmpl_practice: Option<(MatchReplayData, MatchInfo)> = None; // Practice 우선
    let mut tmpl_any: Option<(MatchReplayData, MatchInfo)> = None;      // 폴백: 아무 매치
    for (mt, mi) in r.matches.iter() {
        if let MatchType::Practice { match_id } = mt {
            max_mid = max_mid.max(*match_id as u64);
        }
        if mi.replays.is_empty() { continue; }
        max_key = max_key.max(mi.replays[0]);
        if let Some(rep) = r.match_replays.get(&mi.replays[0]) {
            if matches!(mt, MatchType::Practice { .. }) {
                if tmpl_practice.is_none() { tmpl_practice = Some((rep.clone(), mi.clone())); }
            } else if tmpl_any.is_none() {
                tmpl_any = Some((rep.clone(), mi.clone()));
            }
        }
    }
    for (&k, _) in r.match_replays.iter() { max_key = max_key.max(k); }
    // Practice 템플릿 우선, 없으면 아무 매치(Normal 등) 복제 → 완료 연습경기 의존 제거
    let (replay, mut info) = tmpl_practice.or(tmpl_any)?;
    let new_mid = max_mid + 1;
    let new_key = max_key + 1;
    info.replays = vec![new_key];
    info.is_practice = true; // Normal서 clone한 경우 Practice로 표시 (필드 접근 가능성=컴파일 확정)
    Some((new_mid, new_key, replay, info))
}

// ★M4 진단(항상 ON): match_replays[key] 선수별 statistics 를 scrim_result_stats.txt 에 append.
//   ARM 직후(="ARM-template", 클론된 템플릿 값) vs 팝업닫힘(="popup-close", 재생 후) 비교 →
//   값이 우리 라인업/매치로 바뀌면 = 게임이 재생결과를 match_replays 에 영속 → 요약읽기 가능.
//   동일하면 = 영속 안 함 → think훅 집계 등 다른 경로 필요.
fn log_result_stats(r: &ClientDatabase, key: usize, tag: &str) {
    use std::io::Write;
    let Some(p) = dll_path().and_then(|p| p.parent().map(|d| d.join("scrim_result_stats.txt"))) else { return; };
    let mut s = String::new();
    match r.match_replays.get(&key) {
        None => s.push_str(&format!("=== [stats:{}] [{}ms] key={} → match_replays 에 없음\n", tag, now_ms(), key)),
        Some(rep) => {
            s.push_str(&format!("=== [stats:{}] [{}ms] key={} replay.id={} blue_win={:?} ===\n",
                tag, now_ms(), key, rep.id, rep.blue_team_win));
            for (side, team) in [("BLUE", &rep.blue_team), ("RED", &rep.red_team)] {
                for ath in team.iter() {
                    s.push_str(&format!("  [{}] aid={} champ={} pos={:?}\n        stats={:?}\n",
                        side, ath.athlete_id, ath.champion, ath.position, ath.statistics));
                }
            }
        }
    }
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(p) {
        let _ = f.write_all(s.as_bytes());
    }
}

// ★프레임스캔(M4): always-on 로거 + 리플레이 프레임에서 슬롯별 최종 PlayerStatistics 추출.
const FRAMESCAN_LOG: bool = false; // ★false=정상(끔). 스크림경기 관전 중 매30프레임 파일쓰기=1초주기 hitch 원인
fn fs_log(line: &str) {
    if !FRAMESCAN_LOG { return; }   // 진단 전용 — 배포시 off (관전 버벅임 제거)
    use std::io::Write;
    let Some(p) = dll_path().and_then(|p| p.parent().map(|d| d.join("scrim_framescan.txt"))) else { return; };
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(p) {
        let _ = writeln!(f, "[{}ms] {}", now_ms(), line);
    }
}

#[derive(Default, Clone, Copy)]
struct SlotStat { level: i64, kill: i64, death: i64, assist: i64, deal: i64, tank: i64, heal: i64, gold: i64, cs: i64, seen: bool }

// ★M4 매치 단위 집계(오브젝트/스코어/승자) — 프레임 이벤트서 추출.
#[derive(Default, Clone, Copy)]
struct MatchAgg {
    score: [i64; 2],   // [blue, red] 팀 킬 (Score 이벤트)
    tower: [i64; 2],   // [blue, red] 타워 점수
    serpen: [i64; 2],  // [blue, red] 서펜 처치수
    morgard: [i64; 2], // [blue, red] 모르가드(에픽) — best-effort
    winner: i8,        // -1 미정, 0 블루, 1 레드
}

// game_view(재생중 라이브) 프레임 전체 스캔 → 슬롯별(team*5+pos) 최종 PlayerStatistics. game_view 없으면 None.
//   PlayerStatistics 필드는 단수(kill/death/assist/deal/tank/heal), AthleteStatistics(저장)와 다름.
// ★증분 스캔 누적 상태: 매 호출 전체 재스캔(O(frames), 경기길수록 무거워짐=1초hitch) 대신
//   새 프레임만 처리(O(신규프레임)=상수). 새 매치(프레임 리셋)는 pos>total 로 감지해 초기화.
static SCAN_POS: AtomicUsize = AtomicUsize::new(0);
static SCAN_ACC: Mutex<Option<([SlotStat; 10], MatchAgg)>> = Mutex::new(None);
fn scan_replay_stats(r: &ClientDatabase) -> Option<([SlotStat; 10], usize, MatchAgg)> {
    let gv = r.game_view.as_ref()?;
    let frames = &gv.client.events;
    let total = frames.len();
    let mut g = SCAN_ACC.lock().unwrap_or_else(|e| e.into_inner());
    let mut pos = SCAN_POS.load(Ordering::Relaxed);
    if g.is_none() || pos > total {            // 첫 스캔 or 새 매치(프레임수 되돌아감) → 누적 초기화
        let mut a = MatchAgg::default(); a.winner = -1;
        *g = Some(([SlotStat::default(); 10], a));
        pos = 0;
    }
    let (out, agg) = g.as_mut().unwrap();
    // ★부하 분산: 호출당 최대 SCAN_BUDGET 프레임만 처리(나머지는 다음 호출). 매 프레임 호출해 따라잡음.
    //   초반 버퍼링 대량유입 시에도 프레임당 부담이 작아 끊김 없음(전체계산은 좀 더 오래 걸려도 무방).
    const SCAN_BUDGET: usize = 24;
    let end = (pos + SCAN_BUDGET).min(total);
    for fi in pos..end {                        // ★새 프레임 중 예산만큼만
        for ev in frames[fi].events.iter() {
            let s = format!("{:?}", ev);
            if s.contains("PlayerStatistics") {
                let (Some(team), Some(pi)) = (dbg_i64(&s, "team:"), pos_idx(&s)) else { continue };
                let slot = (team * 5 + pi) as usize;
                if slot >= 10 { continue; }
                let st = &mut out[slot];
                st.seen = true;
                st.level  = dbg_i64(&s, "level:").unwrap_or(st.level);
                st.kill   = dbg_i64(&s, ", kill:").unwrap_or(st.kill);   // ", kill:" = solo_kill 회피
                st.death  = dbg_i64(&s, "death:").unwrap_or(st.death);
                st.assist = dbg_i64(&s, "assist:").unwrap_or(st.assist);
                st.deal   = dbg_i64(&s, "deal:").unwrap_or(st.deal);
                st.tank   = dbg_i64(&s, "tank:").unwrap_or(st.tank);
                st.heal   = dbg_i64(&s, "heal:").unwrap_or(st.heal);     // heal 이 self_heal 보다 앞 → first match
                st.gold   = dbg_i64(&s, "gold:").unwrap_or(st.gold);
                st.cs     = dbg_i64(&s, ", cs:").unwrap_or(st.cs);       // ", cs:" = cs_jungle 회피
            } else if s.contains("Score") {
                // ScoreEventData{ scores:[b,r], tower_score:[b,r], ... } — 최신값 유지
                if let (Some(sb), Some(sr)) = (dbg_arr2(&s, "scores:"), dbg_arr2(&s, "tower_score:")) {
                    agg.score = sb; agg.tower = sr;
                }
            } else if s.contains("SerpenKill") {
                if let Some(t) = dbg_i64(&s, "team:") { if (0..2).contains(&t) { agg.serpen[t as usize] += 1; } }
            } else if s.contains("Morgard") && (s.contains("Kill") || s.contains("kill")) {
                if let Some(t) = dbg_i64(&s, "team:") { if (0..2).contains(&t) { agg.morgard[t as usize] += 1; } }
            }
            // 승자: 넥서스 처치 (KillEvent killed/target 에 "nexus")
            if s.contains("nexus") {
                if let Some(kt) = dbg_i64(&s, "killer_team:").or_else(|| dbg_i64(&s, "player_team:")) {
                    if (0..2).contains(&kt) { agg.winner = kt as i8; }
                }
            }
        }
    }
    SCAN_POS.store(end, Ordering::Relaxed);    // 처리한 데까지만 전진(예산 cap)
    // 반환본에만 승자 폴백(누적 agg엔 추측 미저장 → 이후 실제 nexus 이벤트가 갱신 가능)
    let out_copy = *out;
    let mut agg_copy = *agg;
    if agg_copy.winner < 0 {
        agg_copy.winner = if agg_copy.tower[0] != agg_copy.tower[1] {
            if agg_copy.tower[0] > agg_copy.tower[1] { 0 } else { 1 }
        } else if agg_copy.score[0] != agg_copy.score[1] {
            if agg_copy.score[0] > agg_copy.score[1] { 0 } else { 1 }
        } else { -1 };
    }
    Some((out_copy, total, agg_copy))
}

// Debug 문자열의 "key: [a, b]" → [a,b] 파싱.
fn dbg_arr2(s: &str, key: &str) -> Option<[i64; 2]> {
    let p = s.find(key)?;
    let rest = &s[p + key.len()..];
    let lb = rest.find('[')?;
    let rb = rest[lb..].find(']')? + lb;
    let inner = &rest[lb + 1..rb];
    let mut it = inner.split(',').map(|x| x.trim().parse::<i64>().ok());
    Some([it.next()??, it.next()??])
}

// ★M4 결과요약(brief) 기록: 최종 SlotStat → 스코어/팀합산/선수별. scrim_brief.txt(always-on).
fn write_brief(st: &[SlotStat; 10], agg: &MatchAgg) {
    use std::io::Write;
    let tsum = |a: usize, b: usize, f: fn(&SlotStat) -> i64| (a..b).map(|i| f(&st[i])).sum::<i64>();
    let bk = if agg.score[0] + agg.score[1] > 0 { agg.score[0] } else { tsum(0, 5, |x| x.kill) };
    let rk = if agg.score[0] + agg.score[1] > 0 { agg.score[1] } else { tsum(5, 10, |x| x.kill) };
    let verdict = match agg.winner { 0 => "블루 승리", 1 => "레드 승리", _ => "무승부" };
    let mut s = String::new();
    s.push_str(&format!("======== 빠른 시뮬 결과 ========\n블루 {} : {} 레드   ({})\n", bk, rk, verdict));
    s.push_str(&format!("오브젝트  모르가드 {}:{}  서펜 {}:{}  타워 {}:{}\n",
        agg.morgard[0], agg.morgard[1], agg.serpen[0], agg.serpen[1], agg.tower[0], agg.tower[1]));
    s.push_str(&format!("팀 딜  블루 {} / 레드 {}\n", tsum(0,5,|x|x.deal), tsum(5,10,|x|x.deal)));
    s.push_str(&format!("팀 탱  블루 {} / 레드 {}\n", tsum(0,5,|x|x.tank), tsum(5,10,|x|x.tank)));
    s.push_str(&format!("팀 힐  블루 {} / 레드 {}\n", tsum(0,5,|x|x.heal), tsum(5,10,|x|x.heal)));
    let pos = ["탑", "정글", "미드", "바텀", "서폿"];
    for (side, base) in [("블루", 0usize), ("레드", 5usize)] {
        s.push_str(&format!("-- {} --\n", side));
        for i in 0..5 {
            let x = st[base + i];
            s.push_str(&format!("  {:<4} Lv{:<2} {}/{}/{}  딜{} 탱{} 힐{} 골드{} CS{}\n",
                pos[i], x.level, x.kill, x.death, x.assist, x.deal, x.tank, x.heal, x.gold, x.cs));
        }
    }
    if let Some(p) = dll_path().and_then(|p| p.parent().map(|d| d.join("scrim_brief.txt"))) {
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(p) {
            let _ = writeln!(f, "[{}ms]\n{}", now_ms(), s);
        }
    }
}

// 천단위 콤마.
fn fmt_num(v: i64) -> String {
    let s = v.abs().to_string();
    let b = s.as_bytes();
    let mut out = String::new();
    for (i, &c) in b.iter().enumerate() {
        if i > 0 && (b.len() - i) % 3 == 0 { out.push(','); }
        out.push(c as char);
    }
    if v < 0 { format!("-{}", out) } else { out }
}

// ★M4 결과창(scrim_result) 채우기 — beta comp_test 레이아웃에 데이터 주입.
fn populate_result(root: &mut Node, st: &[SlotStat; 10], agg: &MatchAgg) {
    // 스코어(팀 킬) + 승자
    let bk = if agg.score[0] + agg.score[1] > 0 { agg.score[0] } else { (0..5).map(|i| st[i].kill).sum() };
    let rk = if agg.score[0] + agg.score[1] > 0 { agg.score[1] } else { (5..10).map(|i| st[i].kill).sum() };
    set_label_by_id(root, "res_score", &format!("<#37d5b3ff>블루 {}<>  :  <#eb3d4dff>{} 레드<>", bk, rk));
    set_label_by_id(root, "res_wintag", match agg.winner {
        0 => "<#37d5b3ff>블루 승리<>", 1 => "<#eb3d4dff>레드 승리<>", _ => "무승부" });
    set_label_by_id(root, "res_morg", &format!("블루 {} / 레드 {}", agg.morgard[0], agg.morgard[1]));
    set_label_by_id(root, "res_serp", &format!("블루 {} / 레드 {}", agg.serpen[0], agg.serpen[1]));
    set_label_by_id(root, "res_tow", &format!("블루 {} / 레드 {}", agg.tower[0], agg.tower[1]));
    set_label_by_id(root, "res_daily", "");

    // 팀 비교 바 (track 588px, 블루 좌측·레드 우측앵커)
    let team = |a: usize, b: usize, f: fn(&SlotStat) -> i64| (a..b).map(|i| f(&st[i])).sum::<i64>();
    fn set_team_bar(root: &mut Node, pre: &str, bval: i64, rval: i64) {
        set_label_by_id(root, &format!("res_{}_bv", pre), &fmt_num(bval));
        set_label_by_id(root, &format!("res_{}_rv", pre), &fmt_num(rval));
        let tot = (bval + rval).max(1) as f32;
        let bw = (588.0 * bval as f32 / tot).round().clamp(0.0, 588.0);
        set_rect_size_by_id(root, &format!("res_{}_bf", pre), bw, 12.0);
        set_rect_size_by_id(root, &format!("res_{}_rf", pre), 588.0 - bw, 12.0);
    }
    set_team_bar(root, "deal", team(0, 5, |x| x.deal), team(5, 10, |x| x.deal));
    set_team_bar(root, "tank", team(0, 5, |x| x.tank), team(5, 10, |x| x.tank));
    set_team_bar(root, "heal", team(0, 5, |x| x.heal), team(5, 10, |x| x.heal));

    // 선수 카드 (이름=선수명, KDA, 딜/탱/힐 미니바 = 전체최대 대비)
    let maxd = st.iter().map(|x| x.deal).max().unwrap_or(1).max(1) as f32;
    let maxt = st.iter().map(|x| x.tank).max().unwrap_or(1).max(1) as f32;
    let maxh = st.iter().map(|x| x.heal).max().unwrap_or(1).max(1) as f32;
    let players = *PLAYER_SLOTS.lock().unwrap();
    let roster = ROSTER.lock().unwrap().clone();
    let nameof = |aid: Option<usize>| aid
        .and_then(|id| roster.iter().find(|(i, _)| *i == id).map(|(_, n)| n.clone()))
        .unwrap_or_default();
    for i in 0..10 {
        let pre = if i < 5 { format!("b{}", i) } else { format!("r{}", i - 5) };
        let x = st[i];
        set_label_by_id(root, &format!("{}_name", pre), &nameof(players[i]));
        set_label_by_id(root, &format!("{}_kda", pre), &format!("{}/{}/{}", x.kill, x.death, x.assist));
        set_label_by_id(root, &format!("{}_dv", pre), &fmt_num(x.deal));
        set_label_by_id(root, &format!("{}_kv", pre), &fmt_num(x.tank));
        set_label_by_id(root, &format!("{}_hv", pre), &fmt_num(x.heal));
        set_rect_size_by_id(root, &format!("{}_df", pre), (86.0 * x.deal as f32 / maxd).round(), 6.0);
        set_rect_size_by_id(root, &format!("{}_kf", pre), (86.0 * x.tank as f32 / maxt).round(), 6.0);
        set_rect_size_by_id(root, &format!("{}_hf", pre), (86.0 * x.heal as f32 / maxh).round(), 6.0);
    }
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
    // 풀다운(scrim_ddp)은 선수 드롭다운과 동일하게 폭 240 고정(.ui 선언값). 화면 좌상단 위치만 트리거에 맞춤.
    // 설정/전술 패널 모두 1120x640 중앙 → 좌상단 (400,220). 아이템 패널 900x600 중앙 → (510,240).
    let (mut px, mut py) = (0.0_f32, 0.0_f32);
    match kind {
        0 => { // 선수
            let roster = ROSTER.lock().unwrap();
            let cur = PLAYER_SLOTS.lock().unwrap()[slot];
            if let Some(cur_id) = cur {
                if let Some((aid, name)) = roster.iter().find(|(id, _)| *id == cur_id) {
                    items.push((Some(*aid), None, name.clone()));
                }
            }
            for (aid, name) in roster.iter() {
                if Some(*aid) != cur { items.push((Some(*aid), None, name.clone())); }
            }
            items.push((None, None, "선택 해제".to_string()));
            let row = (slot % 5) as f32;
            let rel_x = if slot < 5 { 62.0 } else { 606.0 };
            px = 400.0 + rel_x; py = 220.0 + 156.0 + row * 54.0;
        }
        1 => { // 챔피언 (별도 그리드 scrim_ddc 사용 — 폭/위치 조정 없음)
            let champs = CHAMPS.lock().unwrap();
            for (key, kr) in champs.iter() { items.push((None, Some(key.clone()), kr.clone())); }
            set_label_by_id(root, "scrim_ddc_head", "챔피언 선택");
        }
        2 => { // 팀 전술 필드 (slot = SKEYS 인덱스)
            let team = (STRAT_VIEW_TEAM.load(Ordering::Relaxed) as usize).min(1);
            for (i, s) in STRAT_OPTS[slot].iter().enumerate() { items.push((Some(i), None, s.to_string())); }
            DD_CUR.store(STRAT_WORKING.lock().unwrap()[team][slot] as usize, Ordering::Relaxed);
            let (is_left, row) = STRAT_POS[slot];
            px = if is_left { 636.0 } else { 1156.0 };
            py = 360.0 + row as f32 * 48.0;
        }
        3 => { // 개인 전술(아이템) (slot = sl*3+s)
            let cnt = item_opt_count();
            for i in 0..cnt { items.push((Some(i), None, item_opt_label(i as u8))); }
            let (sl, s) = (slot / 3, slot % 3);
            DD_CUR.store(ITEMS_WORKING.lock().unwrap()[sl.min(9)][s.min(2)] as usize, Ordering::Relaxed);
            let blue = sl < 5; let bi = if blue { sl } else { sl - 5 };
            px = if blue { 540.0 } else { 966.0 } + s as f32 * 140.0;
            py = 384.0 + bi as f32 * 60.0;
        }
        4 => { // 스플릿 담당 포지션 (slot: 0=bld, 1=mor_a, 2=mor_b)
            let team = (STRAT_VIEW_TEAM.load(Ordering::Relaxed) as usize).min(1);
            let sp = STRAT_SPLIT_POS.lock().unwrap()[team];
            // 1-3-1(mor=2)일 때 두 담당이 겹치지 않게 상대 담당 포지션 제외
            let mor131 = STRAT_WORKING.lock().unwrap()[team][6] == 2;
            let (cur, excl) = match slot {
                0 => (sp.0, None),
                1 => (sp.1, if mor131 { Some(sp.2) } else { None }),
                _ => (sp.2, Some(sp.1)),
            };
            for p in 0u8..5 {
                if Some(p) != excl { items.push((Some(p as usize), None, POS_NAMES[p as usize].to_string())); }
            }
            DD_CUR.store(cur as usize, Ordering::Relaxed);
            let (is_left, row) = match slot { 0 => (true, 6usize), 1 => (false, 6), _ => (false, 7) };
            px = if is_left { 636.0 } else { 1156.0 };
            py = 360.0 + row as f32 * 48.0;
        }
        _ => {}
    }
    *DD_ITEMS.lock().unwrap() = items;
    DD_OPEN.store(true, Ordering::Relaxed);
    fill_page(root);
    // 챔프(1) 외에는 공용 풀다운(scrim_ddp): contents 높이(스크롤 범위) + 화면 위치만 설정. 폭/패널크기는 .ui 고정.
    if kind != 1 {
        set_pulldown_contents(root, DD_ITEMS.lock().unwrap().len());
        let pf = dd_prefix(); // 선수/아이템=scrim_ddp(240), 전술/스플릿=scrim_dds(304)
        ui_kit::set_layout_xy_by_id(root, &format!("{}_panel", pf), px, py);
        // 풀다운 노드 재사용 → 이전 스크롤 위치 잔존. 열 때마다 맨 위로 리셋(scroll_view ratio 0.0 = top).
        ui_kit::scroll_set_by_id(root, &format!("{}_list", pf), 0.0);
    }
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
        let row: Option<(String, bool)> = if idx < len {
            let (pid, ckey, name) = &items[idx];
            let chosen = match (pid, ckey) {
                (Some(p), _) if kind == 0 => taken_p.contains(p),
                (Some(p), _) => *p == DD_CUR.load(Ordering::Relaxed), // 전술/아이템: p=옵션 인덱스
                (_, Some(k)) => taken_c.contains(k),
                _ => false,
            };
            Some((name.clone(), chosen))
        } else { None };
        // 선수/챔프 공통: color_selectable + strategy_option (네이티브 룩 — hover/선택 자동)
        let row_id = format!("{}_{:02}", prefix, i);
        if let Some(n) = ui_kit::find_mut(root, &row_id) {
            match &row {
                Some((name, chosen)) => {
                    ui_kit::toggle_text_set(n, name);
                    ui_kit::toggle_set(n, *chosen);
                    n.visible = true;
                }
                None => { n.visible = false; }
            }
        }
    }
    set_visible_by_id(root, &format!("{}_up", prefix), page > 0);
    set_visible_by_id(root, &format!("{}_dn", prefix), page + 1 < pages);
    set_label_by_id(root, &format!("{}_pg", prefix), &format!("{} / {}", page + 1, pages));
    // contents 높이/폭 등 풀다운 크기는 open_dropdown 의 size_pulldown 에서 일괄 설정
}

fn dd_page_delta(d: i64) {
    let pages = dd_pages() as i64;
    let p = (DD_PAGE.load(Ordering::Relaxed) as i64 + d).clamp(0, pages - 1);
    DD_PAGE.store(p as usize, Ordering::Relaxed);
}

// 드롭다운 행 선택 (gi = 화면상 행 인덱스) → 활성 대상에 적용 (kind: 0선수 1챔프 2전술 3아이템)
fn select_dropdown(root: &mut Node, gi: usize) {
    let rows = dd_rows();
    let idx = DD_PAGE.load(Ordering::Relaxed) * rows + gi;
    let kind = DD_KIND.load(Ordering::Relaxed);
    let slot = DD_SLOT.load(Ordering::Relaxed);
    let picked = DD_ITEMS.lock().unwrap().get(idx).cloned();
    if let Some((pid, ckey, _)) = picked {
        match kind {
            0 => {
                let mut ps = PLAYER_SLOTS.lock().unwrap();
                match pid {
                    // 선수 중복 허용 — 같은 선수를 여러 슬롯(미러/같은팀 중복)에 넣을 수 있게 dedup 제거
                    Some(p) => { ps[slot] = Some(p); }
                    None => { ps[slot] = None; } // "선택 해제" 항목 → 슬롯 비움
                }
            }
            1 => if let Some(k) = ckey {
                // 챔프 중복 허용 — 같은 챔프를 여러 슬롯에(미러/같은팀 중복) 넣을 수 있게 dedup 제거
                CHAMP_SLOTS.lock().unwrap()[slot] = Some(k);
            },
            2 => { // 팀 전술: slot=필드, pid=옵션 인덱스
                if let Some(oi) = pid {
                    let team = (STRAT_VIEW_TEAM.load(Ordering::Relaxed) as usize).min(1);
                    if slot < 12 { STRAT_WORKING.lock().unwrap()[team][slot] = oi as u8; }
                }
                repaint_strat(root);
            }
            3 => { // 개인 전술(아이템): slot=sl*3+s, pid=옵션 인덱스
                if let Some(oi) = pid {
                    let (sl, s) = (slot / 3, slot % 3);
                    if sl < 10 && s < 3 { ITEMS_WORKING.lock().unwrap()[sl][s] = oi as u8; }
                }
                repaint_items(root);
            }
            4 => { // 스플릿 담당: slot=0bld/1mor_a/2mor_b, pid=포지션(0~4)
                if let Some(p) = pid {
                    let team = (STRAT_VIEW_TEAM.load(Ordering::Relaxed) as usize).min(1);
                    let mut sp = STRAT_SPLIT_POS.lock().unwrap();
                    match slot { 0 => sp[team].0 = p as u8, 1 => sp[team].1 = p as u8, _ => sp[team].2 = p as u8 }
                }
                repaint_strat(root);
            }
            _ => {}
        }
    }
    // 선수(0)/챔프(1)/스플릿(4)은 즉시 적용 → 즉시 저장. 전술(2)/아이템(3)은 working → 확인 버튼서 commit+저장.
    if kind == 0 || kind == 1 || kind == 4 { save_selection(); }
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
    // 팀 탭: 현재 보는 팀을 selected (게임 strategy_option 스타일이 색 자동 처리)
    ui_kit::toggle_set_by_id(root, "scrim_st_tab_b", team == 0);
    ui_kit::toggle_set_by_id(root, "scrim_st_tab_r", team == 1);
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
            let cnt = item_opt_count();
            let v = (w[slot][s] as usize).min(cnt.saturating_sub(1)) as u8;
            let label = item_opt_label(v);
            set_label_by_id(root, &format!("{}{}_{}_t", pre, slot, s), &label);
        }
        let nm = cs[slot].as_ref().map(|k| champ_display(k)).unwrap_or_else(|| "(선택)".to_string());
        set_label_by_id(root, &format!("{}{}_name", pre, slot), &nm);
    }
}
fn refresh_labels(root: &mut Node) {
    let champs = CHAMPS.lock().unwrap();
    let pslots = PLAYER_SLOTS.lock().unwrap(); // ready 판정용(아래 all is_some)
    let cslots = CHAMP_SLOTS.lock().unwrap();
    // 선수 슬롯은 네이티브 dropdown(scrim_pddN)이 선택값을 자체 표시 → 라벨 갱신 불필요. 챔프만.
    for i in 0..10 {
        let ctxt = cslots[i].as_ref()
            .and_then(|k| champs.iter().find(|(key, _)| key == k).map(|(_, nm)| nm.clone()))
            .unwrap_or_else(|| "(선택)".to_string());
        set_label_by_id(root, &format!("scrim_c{}_t", i), &ctxt);
    }
    // 중복 챔프 경고: 같은 챔프가 2개 이상 슬롯이면 박스=연분홍(#e6aebf)+어두운 글자(선택강조의 빨간버전)
    for i in 0..10 {
        let dup = cslots[i].as_ref().map_or(false, |k|
            cslots.iter().filter(|x| x.as_deref() == Some(k.as_str())).count() >= 2);
        let box_id = format!("scrim_c{}_box", i);
        let lbl_id = format!("scrim_c{}_t", i);
        if dup {
            set_back_color_by_id(root, &box_id, 0.902, 0.682, 0.749, 1.0); // #e6aebf
            if let Some(n) = ui_kit::find_mut(root, &lbl_id) { ui_kit::label_set_color(n, ui_kit::Rgba::new(0.36, 0.125, 0.188, 1.0)); } // 어두운 적색 글자
        } else {
            set_back_color_by_id(root, &box_id, 0.1137, 0.1216, 0.1725, 1.0); // 원래 #1d1f2c
            if let Some(n) = ui_kit::find_mut(root, &lbl_id) { ui_kit::label_set_color(n, ui_kit::Rgba::new(1.0, 1.0, 1.0, 1.0)); } // 기본 흰색
        }
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
            item_cap: rd_u64(slot + AO_ITEMS),       // [아이템칸검증] repoint 원복용
            item_ptr: iptr as u64,
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
const ITEMNET_FORWARD_RVA: usize = 0x1b78420; // 0.5.0_3(구0.5.0_2=0x1b73e50, UNIQUE push8 정상). 핫픽스4 06-18: forward. 마스크시그 8-push 유일
// ★ forward 함수 주소가 기대 프롤로그(PUSH8)인지 1회 검증·캐시. 게임 업데이트로 RVA 가 어긋나면(엉뚱한 코드)
//   그대로 호출 시 크래시 → 무효면 호출 스킵. 0=미확인 1=유효 2=무효.
static FWD_VALID: AtomicU64 = AtomicU64::new(0);
unsafe fn itemnet_addr_valid() -> bool {
    match FWD_VALID.load(Ordering::Relaxed) { 1 => return true, 2 => return false, _ => {} }
    let fa = GetModuleHandleW(core::ptr::null()) as usize + ITEMNET_FORWARD_RVA;
    let expect = [0x55u8,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];
    let mut ok = readable(fa, 12);
    if ok { for i in 0..12 { if *((fa + i) as *const u8) != expect[i] { ok = false; break; } } }
    FWD_VALID.store(if ok { 1 } else { 2 }, Ordering::Relaxed);
    if !ok {
        write_log("scrim_item_filter.txt", &format!(
            "[{}ms] ⚠ forward 프롤로그 불일치(RVA 0x{:x} 무효 — 게임 업데이트로 추정) → 자동빌드 비활성(크래시 방지). RVA 재탐색 필요.\n",
            now_ms(), ITEMNET_FORWARD_RVA));
    }
    ok
}
unsafe fn itemnet_forward(net: usize, ctx: &[u64; 11], build: &[u64]) -> f32 {
    if !itemnet_addr_valid() { return f32::MIN; } // 무효 주소 호출 방지
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
static AUTO_FILTER_LOGGED: AtomicBool = AtomicBool::new(false);
// ★ ID 목록에서 비활성 모드 아이템 제거(자동빌드 후보용). 바닐라(0-29) 항상 유지.
//   활성 판정 = DB 스캔 레지스트리(MOD_REGISTRY) 멤버십. dump_mod_items 는 활성 모드템만 잡음
//   (비활성은 Database 통합컬렉션 미병합 → 스캔 부재). 구 i18n active_item_keys 폐기.
fn filter_enabled_ids(ids: Vec<u64>) -> Vec<u64> {
    let reg = MOD_REGISTRY.lock().unwrap();
    if reg.is_empty() { return ids; } // DB 스캔 전/실패 → 안전 폴백(필터 안함)
    let before = ids.len();
    let out: Vec<u64> = ids.into_iter().filter(|&id| {
        if id < 30 { return true; } // 바닐라 = 항상 활성
        (id as usize).checked_sub(30).map_or(false, |i| i < reg.len()) // DB 스캔에 있으면 활성
    }).collect();
    if !AUTO_FILTER_LOGGED.swap(true, Ordering::Relaxed) {
        write_log("scrim_item_filter.txt", &format!(
            "[{}ms] 자동빌드 후보 활성필터(DB스캔): {}개 → {}개 (비활성 {}개 제외)\n",
            now_ms(), before, out.len(), before - out.len()));
    }
    out
}
// ★ 자동칸 후보 풀 = 게임이 forward 에 실제로 넘기는 최종템 ID 집합(SEEN_ITEM_IDS, 라이브 누적).
//   = 바닐라 6 + 모드 최종템들. ★SEEN 엔 비활성 모드템도 누적되므로 filter_enabled_ids 로 제외. 아직 안 모였으면 폴백.
fn auto_cands() -> Vec<u64> {
    let seen_v: Option<Vec<u64>> = {
        let seen = SEEN_ITEM_IDS.lock().unwrap();
        if seen.is_empty() { None } else {
            let mut v = seen.clone(); v.sort_unstable(); v.dedup(); Some(v)
        }
    };
    if let Some(v) = seen_v {
        return filter_enabled_ids(v); // ★ 비활성 모드 아이템 제외
    }
    // SEEN 미수집 시: 동적 모드 최종템(이미 picker 필터 적용됨) + 바닐라6
    let mut reg: Vec<u64> = mod_final_opts().iter().map(|(id, _)| *id).collect();
    if !reg.is_empty() {
        reg.extend_from_slice(&final_ids());
        reg.sort_unstable();
        reg.dedup();
        return reg;
    }
    final_ids().to_vec()
}

// 카테고리(1~6) → 5티어 아이템 ID
fn cat_to_id(cat: u8) -> Option<u64> {
    if cat == 0 || cat > 6 { return None; }
    Some(final_ids()[(cat - 1) as usize])
}

// 게임 루트 = DLL(.../mods/tfm2_scrim/x.dll) 에서 3단계 상위.
fn game_root() -> Option<PathBuf> {
    dll_path()?.parent()?.parent()?.parent().map(|p| p.to_path_buf())
}
// config/game/mods.json 의 enabled_mods 목록.
// ⚠ mods.json 은 accepted_save_mod_mismatch_warnings 등으로 거대·복잡해 full JSON 파싱이 실패할 수 있다.
//   → enabled_mods 배열만 텍스트에서 직접 추출(robust). (enabled_mods 는 평면 문자열 배열이라 첫 ']' 까지가 끝.)
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
// 활성 모드들이 제공하는 아이템 key 집합 (각 모드 text/item.i18n 키 합집합).
//   로컬 mods/* + 워크샵 ../../workshop/content/3009300/* 폴더의 mod.mod_info 로 mod_id 확인 → 활성이면 키수집.
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
                if let JsonValue::Obj(items) = lobj {
                    for (k, _) in items { set.insert(k); }
                }
            }
        }
    }
    set
}
static ACTIVE_KEYS: Mutex<Option<std::collections::HashSet<String>>> = Mutex::new(None);
fn active_item_keys() -> std::collections::HashSet<String> {
    {
        let g = ACTIVE_KEYS.lock().unwrap();
        if let Some(s) = g.as_ref() { return s.clone(); }
    }
    let set = build_active_item_keys();
    *ACTIVE_KEYS.lock().unwrap() = Some(set.clone());
    set
}

// 동적 최종템 전체 = (게임ID, key). 1순위=MOD_FINALS(next_tier 빈것), 2순위=radiant_ 휴리스틱.
fn mod_final_opts_all() -> Vec<(u64, String)> {
    {
        let finals = MOD_FINALS.lock().unwrap();
        if !finals.is_empty() {
            let reg = MOD_REGISTRY.lock().unwrap();
            let v: Vec<(u64, String)> = finals.iter().filter_map(|&id| {
                let i = (id as usize).checked_sub(30)?;
                reg.get(i).map(|k| (id, k.clone()))
            }).collect();
            if !v.is_empty() { return v; }
        }
    }
    let reg = MOD_REGISTRY.lock().unwrap();
    reg.iter().enumerate()
        .filter(|(_, k)| k.starts_with("radiant"))
        .map(|(i, k)| (30 + i as u64, k.clone()))
        .collect()
}
// ★ picker 노출 최종템 = DB 스캔 결과 그대로(dump_mod_items → MOD_FINALS/MOD_REGISTRY).
//   비활성 모드템은 Database 통합 컬렉션에 병합 안 돼 DB 스캔에 애초에 안 잡힘 →
//   구 enabled_mods×i18n(active_item_keys) 활성필터 폐기. (item_tactics 동일 방식·[[tfm2-active-items-detection]])
fn mod_final_opts() -> Vec<(u64, String)> { mod_final_opts_all() }
// picker 총 옵션 수 = 자동(1) + 카테고리6 + 동적 최종템
fn item_opt_count() -> usize { 7 + mod_final_opts().len() }
// picker 값 v → 표시 라벨. 0~6=고정(자동/카테고리), 7+=모드 최종템(게임 i18n 참조 → 자동 현지화).
//  i18n 참조는 LabelRunner 가 렌더시 해석. 모드 비활성이면 안 풀려 '#asset...' 날것 = 비활성 표시 겸용.
fn item_opt_label(v: u8) -> String {
    let vi = v as usize;
    if vi < 7 { return ITEM_OPTS[vi].to_string(); }
    match mod_final_opts().get(vi - 7) {
        Some((_, key)) => format!("#asset/base/text/item?{}.name", key),
        None => ITEM_OPTS[0].to_string(),
    }
}
// 라이브 모드 최종템 ID 목록 (auto_cands/compute_build_b 용). 동적 최종 우선, 없으면 SEEN 폴백.
fn mod_final_ids() -> Vec<u64> {
    let opts = mod_final_opts();
    if !opts.is_empty() { return opts.iter().map(|(id, _)| *id).collect(); }
    let seen = SEEN_ITEM_IDS.lock().unwrap();
    let mut v: Vec<u64> = seen.iter().cloned().filter(|&x| x >= 30).collect();
    v.sort_unstable(); v.dedup();
    v
}
// ITEMS_COMMITTED 값 → 주입 ID. 0=자동(None), 1~6=카테고리(바닐라 최종), 7+=동적 최종템 i번째.
fn commit_to_id(v: u8) -> Option<u64> {
    match v {
        0 => None,
        1..=6 => cat_to_id(v),
        _ => mod_final_opts().get((v - 7) as usize).map(|(id, _)| *id),
    }
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
static CBB_LOGGED: AtomicBool = AtomicBool::new(false);
unsafe fn compute_build_b(net: usize, team_ids: &[u64; 5], counters: &[u64; 5], pos: usize, commit: &[u8; 3]) -> Option<[u64; 3]> {
    // ★진단(1회): 자동빌드가 왜 안 도는지 — net 주소·읽기가능·forward유효성 기록
    if !CBB_LOGGED.swap(true, Ordering::Relaxed) {
        write_log("scrim_item_filter.txt", &format!(
            "[{}ms] compute_build_b 진입: net={:#x} readable={} forward_valid={} (net==0 이면 ITEM_NET_ADDR 미설정=서버probe 안돔)\n",
            now_ms(), net, readable(net, 0x20), itemnet_addr_valid()));
    }
    if net == 0 || !readable(net, 0x20) { return None; }
    if !itemnet_addr_valid() { return None; } // ★ forward RVA 무효(게임 업데이트) → 자동빌드 전체 스킵(크래시 방지)
    // ctx 구성: [0..5]=팀 champ id, [5..10]=카운터, [10]=현재 포지션
    let mut ctx = [0u64; 11];
    for i in 0..5 { ctx[i] = team_ids[i]; ctx[5 + i] = counters[i]; }
    ctx[10] = pos as u64;
    // 자동칸 후보 = 게임의 라이브 최종템 풀(바닐라+모드). 수동칸 = 그 카테고리 바닐라 최종템.
    let cands: Vec<u64> = auto_cands();
    // 각 칸 후보: 자동(0) 또는 해석불가 모드템 → 라이브 풀(cands); 그 외 → 그 1개 ID.
    let sc = |c: u8| -> Vec<u64> {
        match commit_to_id(c) { Some(id) => vec![id], None => cands.clone() }
    };
    let slot_cands: [Vec<u64>; 3] = [sc(commit[0]), sc(commit[1]), sc(commit[2])];
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
    // ★진단(1회): 자동빌드 apply 도달 시점 — net 주소/forward유효/캐시변경 무조건 기록(원인 격리)
    static APPLY_LOGGED: AtomicBool = AtomicBool::new(false);
    if !APPLY_LOGGED.swap(true, Ordering::Relaxed) {
        let nd = ITEM_NET_ADDR.load(Ordering::Relaxed);
        write_log("scrim_item_filter.txt", &format!(
            "[{}ms] 자동빌드 apply 도달: net(ITEM_NET_ADDR)={:#x} readable={} forward_valid={} cache_changed={}\n  해석: net=0 → 서버 probe(ITEM_NET_ADDR) 미설정 / cache_changed=false → 라인업·아이템 동일로 재계산 스킵(라인업 바꿔보면 재계산됨)\n",
            now_ms(), nd, unsafe { readable(nd, 0x20) }, unsafe { itemnet_addr_valid() },
            BUILD_CACHE_KEY.load(Ordering::Relaxed) != cache_key));
    }
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
                for s in 0..3 { if let Some(id) = commit_to_id(ib[s]) { a[s] = id; } }
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
                    for s in 0..3 { if let Some(id) = commit_to_id(ib[s]) { a[s] = id; } }
                    cache[i] = a;
                }
            } else {
                let mut a = [0u64; 3];
                for s in 0..3 { if let Some(id) = commit_to_id(ib[s]) { a[s] = id; } }
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
        // ★★ [아이템칸검증] 4번째 아이템 강제주입: items Vec 을 4-element 버퍼로 repoint.
        //   FORCE_4TH_MASK 비트의 슬롯만(0x1f=블루팀 0~4, 레드팀 5~9는 컨트롤로 3유지 → 같은경기 A/B).
        //   첫 3칸=우리빌드 b(0이면 원본유지), 4번째=FOURTH_ID. 원본 cap/ptr/len 은 capture_backup 에 저장→restore 원복.
        //   .leak() = 게임이 보관(우리가 free 안함). champion repoint 와 동일 안전패턴.
        const FORCE_4TH_MASK: u64 = 0;      // ★0=정상(꺼짐). 0x3ff=전원, 0x1f=블루팀만 (4번째 강제주입 테스트)
        const FOURTH_ID: u64 = 29;          // 4번째 아이템 (29=HP 최종템). 1~3번째는 스크림UI 수동지정
        const PROBE_GUARD: bool = false;    // ★false=정상(꺼짐). [캡추적] 슬롯0 items 가드페이지 프로브
        if b.iter().any(|&x| x != 0) {
            let iptr = rd_u64(slots[i] + AO_ITEMS + 8) as usize;
            let ilen = rd_u64(slots[i] + AO_ITEMS + 16) as usize;
            if (FORCE_4TH_MASK >> i) & 1 == 1 && iptr != 0 && ilen >= 1 {
                let orig = |k: usize| if ilen > k && readable(iptr, (k + 1) * 8) { rd_u64(iptr + k * 8) } else { 0 };
                let four: Vec<u64> = vec![
                    if b[0] != 0 { b[0] } else { orig(0) },
                    if b[1] != 0 { b[1] } else { orig(1) },
                    if b[2] != 0 { b[2] } else { orig(2) },
                    FOURTH_ID,
                ];
                // 슬롯0(PROBE_GUARD)은 전용 페이지에, 나머지는 leak. 둘 다 restore 서 원복.
                let newptr = if PROBE_GUARD && i == 0 {
                    let pg = VirtualAlloc(0, 0x1000, 0x3000, 0x04); // MEM_COMMIT|RESERVE, PAGE_READWRITE
                    if pg != 0 {
                        for (k, &v) in four.iter().enumerate() { wr_u64(pg + k * 8, v); }
                        GUARD_HITS.store(0, Ordering::Relaxed);
                        GUARD_PENDING.store(false, Ordering::Relaxed);
                        GUARD_BASE.store(pg as u64, Ordering::Relaxed);
                        GUARD_END.store((pg + 0x1000) as u64, Ordering::Relaxed);
                        pg as u64
                    } else { four.leak().as_ptr() as u64 }
                } else {
                    four.leak().as_ptr() as u64
                };
                wr_u64(slots[i] + AO_ITEMS + 0, 4);        // cap
                wr_u64(slots[i] + AO_ITEMS + 8, newptr);   // ptr
                wr_u64(slots[i] + AO_ITEMS + 16, 4);       // len
                // [아이템칸검증] 주입 확증: items@256 readback (len/4개 id)
                let rb = rd_u64(slots[i] + AO_ITEMS + 16);
                let nm4: Vec<String> = (0..4).map(|k| item_name(rd_u64(newptr as usize + k*8))).collect();
                itemlog_write(&format!("[{}ms] [INJECT] slot{} items len={} [{}]", now_ms(), i, rb, nm4.join(",")));
            } else if iptr != 0 && ilen >= 3 && writable(iptr, ilen * 8) {
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
            // items 복원: 원본 cap/ptr/len 되돌림(repoint 원복) + 원본 값 3개(in-place 주입분)
            wr_u64(slot + AO_ITEMS + 0, sb.item_cap);
            wr_u64(slot + AO_ITEMS + 8, sb.item_ptr);
            wr_u64(slot + AO_ITEMS + 16, sb.item_len);
            let iptr = sb.item_ptr as usize;
            if iptr != 0 && writable(iptr, 24) { for s in 0..3 { wr_u64(iptr + s*8, sb.items[s]); } }
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
    // ★ 다른 모드(item_editor/ui_kit)도 filter_handler 사용 → is_empty 로 판단 불가(공존 충돌).
    //   len 이 직전보다 줄면(게임이 scene 전환때 filter_handler 통째 교체) 재등록. (ui_kit ensure_clicks 동일 패턴)
    let cur = ui.filter_handler.len();
    let last = SCRIM_FH_LEN.load(Ordering::Relaxed);
    if last == usize::MAX || cur < last {
    let filter: Rc<dyn Fn(&UIEvent) -> bool> = Rc::new(|e: &UIEvent| {
        if let UIEvent::Click { path, .. } = e {
            let ps = path.to_string();
            let hit = |kind: u8, slot: usize| {
                CLICK_QUEUE.lock().unwrap().push((kind, slot));
            };
            // 드롭다운 우선 (열려 있을 때 그 클릭부터 처리)
            if ps.contains("scrim_ddp_close") || ps.contains("scrim_ddc_close") || ps.contains("scrim_dds_close")
                || ps.contains("scrim_ddp_xbtn") || ps.contains("scrim_ddc_xbtn") { hit(6, 0); return true; }
            if ps.contains("scrim_ddp_up") || ps.contains("scrim_ddc_up") { hit(7, 0); return true; }
            if ps.contains("scrim_ddp_dn") || ps.contains("scrim_ddc_dn") { hit(8, 0); return true; }
            for i in 0..PP_ROWS { if ps.contains(&format!("scrim_ddp_{:02}", i)) { hit(5, i); return true; } }
            for i in 0..PP_ROWS { if ps.contains(&format!("scrim_dds_{:02}", i)) { hit(5, i); return true; } } // 전술/스플릿 행 (선수와 동일 select 경로)
            for i in 0..CC_ROWS { if ps.contains(&format!("scrim_ddc_{:02}", i)) { hit(9, i); return true; } }
            // ── 결과창(scrim_result) — scrim_close 보다 먼저(접두사 충돌 없으나 안전) ──
            if ps.contains("scrim_res_watch") { hit(22, 0); return true; }
            if ps.contains("scrim_res_close") || ps.contains("scrim_res_bg") { hit(21, 0); return true; }
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
            // 전술 팀 탭 (박스 감지보다 먼저! scrim_st_ 로 시작하므로) — slot 0=블루 1=레드
            if ps.contains("scrim_st_tab_b") { hit(18, 0); return true; }
            if ps.contains("scrim_st_tab_r") { hit(18, 1); return true; }
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
                // 선수 슬롯(scrim_pddN)은 네이티브 dropdown 이 클릭/펼침을 자체 처리 → 트리거 불필요
                if ps.contains(&format!("scrim_c{}", i)) { hit(2, i); return true; }
            }
        }
        false
    });
    let handler: Rc<dyn Fn(&mut UIEventHandlerContext<(), UIOutEvent>)> = Rc::new(|_ctx| {});
    ui.filter_handler.push((filter, handler));
    write_log("scrim_handler.txt", &format!("(re)install @{}ms fh_len={}\n", now_ms(), ui.filter_handler.len()));
    }
    SCRIM_FH_LEN.store(ui.filter_handler.len(), Ordering::Relaxed);
}

// ===========================================================================
//  메인
// ===========================================================================
struct ScrimExt;
// [아이템칸검증] GameEvent Debug 문자열 파서: 따옴표 필드값 / items 배열 추출
fn dbg_quoted(s: &str, key: &str) -> Option<String> {
    let p = s.find(key)?;
    let rest = &s[p + key.len()..];
    let q1 = rest.find('"')?;
    let after = &rest[q1 + 1..];
    let q2 = after.find('"')?;
    Some(after[..q2].to_string())
}
// key 뒤 정수 파싱 (예: ", id:" → 5). entity_id 회피 위해 호출측이 ", id:"/"{ id:" 앵커 사용.
fn dbg_i64(s: &str, key: &str) -> Option<i64> {
    let p = s.find(key)?;
    let r = s[p + key.len()..].trim_start();
    let end = r.find(|c: char| !(c.is_ascii_digit() || c == '-')).unwrap_or(r.len());
    if end == 0 { return None; }
    r[..end].parse().ok()
}
// position 문자열 → 인덱스 (Top0 Jungle1 Mid2 Bottom3 Support4). "Position::" 접두사도 허용.
fn pos_idx(s: &str) -> Option<i64> {
    let p = s.find("position:")?;
    let r = s[p + 9..].trim_start().trim_start_matches("Position::");
    for (i, n) in ["Top", "Jungle", "Mid", "Bottom", "Support"].iter().enumerate() {
        if r.starts_with(n) { return Some(i as i64); }
    }
    None
}
// items: ["a", "b"] → (개수, "a,b"). 빈 배열은 (0, "").
fn dbg_items(s: &str) -> Option<(usize, String)> {
    let p = s.find("items:")?;
    let rest = &s[p..];
    let lb = rest.find('[')?;
    let rb = rest[lb..].find(']')? + lb;
    let inner = &rest[lb + 1..rb];
    let cnt = inner.matches('"').count() / 2;
    Some((cnt, inner.replace('"', "").replace(", ", ",")))
}
// [아이템칸검증] 전용 로거 — LOG_ENABLED(평소 off) 무관하게 항상 기록 (개수 증가 시에만 호출 = 극소량)
fn itemlog_write(line: &str) {
    use std::io::Write;
    let Some(p) = dll_path().and_then(|p| p.parent().map(|d| d.join("scrim_item_count.txt"))) else { return; };
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(p) {
        let _ = writeln!(f, "{}", line);
    }
}
// [캡추적] 가드 폴트 RIP 기록 (메인스레드 post_update 에서 호출 = 파일I/O 안전)
fn guard_log(line: &str) {
    use std::io::Write;
    let Some(p) = dll_path().and_then(|p| p.parent().map(|d| d.join("scrim_guard.txt"))) else { return; };
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(p) {
        let _ = writeln!(f, "{}", line);
    }
}
// [캡추적] 매프레임: 펜딩 폴트 기록 + 가드 재무장 (post_update 에서 호출)
fn guard_tick() {
    let gb = GUARD_BASE.load(Ordering::Relaxed);
    if gb == 0 { return; }
    if GUARD_PENDING.load(Ordering::Relaxed) {
        let rip = GUARD_LAST_RIP.load(Ordering::Relaxed);
        let off = GUARD_LAST_OFF.load(Ordering::Relaxed);
        let tid = GUARD_LAST_TID.load(Ordering::Relaxed);
        let mbase = unsafe { GetModuleHandleW(core::ptr::null()) } as u64;
        let (lo, hi) = (mbase, mbase + 0x4000000);
        let rc = GUARD_REGS[0].load(Ordering::Relaxed); let rd = GUARD_REGS[1].load(Ordering::Relaxed);
        let r8 = GUARD_REGS[2].load(Ordering::Relaxed); let r9 = GUARD_REGS[3].load(Ordering::Relaxed);
        guard_log(&format!("[{}ms] === 폴트 rip=0x{:x} off={} tid={} hits={} | rcx=0x{:x} rdx=0x{:x} r8=0x{:x} r9=0x{:x}",
            now_ms(), rip, off, tid, GUARD_HITS.load(Ordering::Relaxed), rc, rd, r8, r9));
        for k in 0..16 {
            let v = GUARD_STK[k].load(Ordering::Relaxed);
            if v >= lo && v < hi {
                guard_log(&format!("    stk[{}] 게임exe복귀 = 0x{:x}  rva=0x{:x}", k, v, v - mbase));
            }
        }
        GUARD_PENDING.store(false, Ordering::Relaxed);
    }
    if GUARD_HITS.load(Ordering::Relaxed) < 200 {
        let mut old = 0u32;
        unsafe { VirtualProtect(gb as usize, 0x1000, 0x04 | 0x100, &mut old); } // RW | PAGE_GUARD
    }
}

impl ModExtension for ScrimExt {
    fn on_init(&self, _s: &mut Scene, _u: &mut GameUI, _a: &mut Assets) {
        // 흡수된 ui_inject 훅 백업 설치(init 에서 됐으면 guard no-op).
        let _ = unsafe { uinj::install() };
    }
    fn post_update(&self, scene: &mut Scene, ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        // ★ 흡수된 ui_inject 프레임워크: 조각 재시도 주입 + 모달 배경차단. 씬 무관(메뉴 포함)이라 가드 앞.
        uinj::tick(ui);
        let Scene::InGame { data } = scene else { return; };
        if !BOOTED.swap(true, Ordering::Relaxed) {
            write_log("scrim_version.txt", &format!("build=SCRIM_MAIN booted @{}ms\n", now_ms()));
        }
        ensure_handler(ui);

        // F2: 전체 노드 트리 덤프 (풀다운 구조 분석용) → scrim_tree_dump.txt
        {
            static F8_PREV: AtomicBool = AtomicBool::new(false);
            let f8 = unsafe { (GetAsyncKeyState(VK_F2) as u16) & 0x8000 != 0 };
            if f8 && !F8_PREV.load(Ordering::Relaxed) {
                let mut s = String::new();
                dump_tree(&ui.root, 0, &mut s);
                write_log("scrim_tree_dump.txt", &s);
                // 네이티브 dropdown 구조 분석: 첫 DropdownRunner 메모리 덤프
                let mut s2 = String::new();
                dump_runner_hex(&ui.root, "DropdownRunner", 2048, &mut s2);
                write_log("scrim_runner_dump.txt", &s2);
            }
            F8_PREV.store(f8, Ordering::Relaxed);
        }

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
                    let roster_ok = roster.len() >= 5;
                    if !practice_ok || !roster_ok {
                        CONFIG_READY.store(false, Ordering::Relaxed);
                        // 미달 조건을 모두 나열
                        let mut lines: Vec<String> = Vec::new();
                        if !practice_ok {
                            lines.push("• 연습경기 기록이 없습니다.".to_string());
                            lines.push("   연습경기를 최소 1회 실시하여야 합니다.".to_string());
                        }
                        if !roster_ok {
                            lines.push(format!("• 선수가 최소 5명 이상 필요합니다. (현재 {}명)", roster.len()));
                            lines.push("   선수가 5명 이상인데도 목록이 안 보이면,".to_string());
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
                5 => { select_dropdown(&mut ui.root, slot); changed = true; }
                9 => { select_dropdown(&mut ui.root, slot); changed = true; }
                7 => { dd_page_delta(-1); fill_page(&mut ui.root); }
                8 => { dd_page_delta(1); fill_page(&mut ui.root); }
                6 => { DD_OPEN.store(false, Ordering::Relaxed); }
                3 => {
                    if CONFIG_READY.load(Ordering::Relaxed) && all_filled() {
                        DD_OPEN.store(false, Ordering::Relaxed);
                        // ★ 덮어쓰기 방식(원복): open 시 resolve_practice 로 잡은 "맨 처음 연습경기"를
                        //   그대로 INJECT_KEY/SUMMON_MID 로 사용 → 그 경기를 덮어써 재생.
                        //   (create-new=새 Practice 매치 생성 방식은 다시보기 오염 → 폐기)
                        cn_log(&format!("kind3 ARM 진입 → 덮어쓰기 대상(맨처음 연습경기) match_id={} key={}",
                            SUMMON_MID.load(Ordering::Relaxed), INJECT_KEY.load(Ordering::Relaxed)));
                        // ★M4 진단: ARM 직후 = 덮어쓸 경기의 원본 statistics (재생 전 기준값)
                        {
                            let key = INJECT_KEY.load(Ordering::Relaxed);
                            if key >= 0 { let db = data.db(); log_result_stats(&*db, key as usize, "ARM-template"); }
                        }
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
                        FRAMESCAN_TICK.store(0, Ordering::Relaxed); // ★M4 프레임스캔 검증 카운터 리셋
                        HAD_GAMEVIEW.store(false, Ordering::Relaxed);
                        BRIEF_FINALIZED.store(false, Ordering::Relaxed);
                        RESULT_READY.store(false, Ordering::Relaxed);
                        AUTO_CLOSE_ARMED.store(false, Ordering::Relaxed);
                        SCRIM_RESULT_VISIBLE.store(false, Ordering::Relaxed);
                        *LAST_BRIEF.lock().unwrap() = None;
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
                12 => { *STRAT_COMMITTED.lock().unwrap() = *STRAT_WORKING.lock().unwrap(); STRAT_OPEN.store(false, Ordering::Relaxed); save_selection(); }
                13 => { STRAT_OPEN.store(false, Ordering::Relaxed); } // 취소: working 버림
                14 => { *ITEMS_COMMITTED.lock().unwrap() = *ITEMS_WORKING.lock().unwrap(); ITEMS_OPEN.store(false, Ordering::Relaxed); save_selection(); }
                15 => { ITEMS_OPEN.store(false, Ordering::Relaxed); }
                // 16(전술 박스 클릭)=인라인 dropdown(scrim_stdd_*)이 클릭/펼침 자체 처리 → 트리거 불필요
                18 => { // 전술 팀 탭 선택 (slot: 0=블루, 1=레드)
                    STRAT_VIEW_TEAM.store(slot as u8, Ordering::Relaxed);
                    repaint_strat(&mut ui.root);
                }
                19 => { // 스플릿 담당 박스 클릭 → 포지션 드롭다운 열기 (slot: 0=bld, 1=mor_a, 2=mor_b)
                    if slot < 3 { open_dropdown(&mut ui.root, 4, slot); }
                }
                // 17(아이템 박스 클릭)=인라인 dropdown(scrim_idd_*)이 클릭/펼침 자체 처리 → 트리거 불필요
                21 => { // 결과창 닫기
                    SCRIM_RESULT_VISIBLE.store(false, Ordering::Relaxed);
                }
                22 => { // 결과창 "다시보기": 같은 매치 재소환 (재주입 후 재생)
                    SCRIM_RESULT_VISIBLE.store(false, Ordering::Relaxed);
                    INJECTED_ONCE.store(false, Ordering::Relaxed);
                    FRAMESCAN_TICK.store(0, Ordering::Relaxed);
                    HAD_GAMEVIEW.store(false, Ordering::Relaxed);
                    BRIEF_FINALIZED.store(false, Ordering::Relaxed);
                    RESULT_READY.store(false, Ordering::Relaxed);
                    SCRIM_ARMED.store(true, Ordering::Relaxed);
                    SUMMON_REQUESTED.store(true, Ordering::Relaxed);
                }
                _ => {}
            }
        }

        // 드롭다운 표시 (선수창/챔프창 분리)
        let dd_open = DD_OPEN.load(Ordering::Relaxed);
        let dd_kind = DD_KIND.load(Ordering::Relaxed);
        set_visible_by_id(&mut ui.root, "scrim_dd_p", false); // 선수0/아이템3 둘 다 인라인 dropdown 으로 이전 → scrim_dd_p 모달 미사용
        set_visible_by_id(&mut ui.root, "scrim_dd_s", dd_open && dd_kind == 4); // 스플릿4만 (전술2는 인라인 dropdown 으로 이전)
        set_visible_by_id(&mut ui.root, "scrim_dd_c", dd_open && dd_kind == 1);
        let strat_open = STRAT_OPEN.load(Ordering::Relaxed);
        set_visible_by_id(&mut ui.root, "scrim_strat_modal", strat_open);
        if !strat_open { SDD_INIT.store(false, Ordering::Relaxed); }
        if strat_open {
            // ── 인라인 팀전술 dropdown(네이티브): 최초/팀전환 시 12필드 재주입 + 선택 폴링 ──
            let team = (STRAT_VIEW_TEAM.load(Ordering::Relaxed) as usize).min(1);
            let first = !SDD_INIT.swap(true, Ordering::Relaxed);
            let prev = SDD_TEAM.swap(team as u8, Ordering::Relaxed);
            if first || prev != team as u8 {
                {
                    let w = *STRAT_WORKING.lock().unwrap();
                    let mut last = SDD_LAST.lock().unwrap();
                    for f in 0..12 {
                        let sel = (w[team][f] as usize).min(STRAT_OPTS[f].len() - 1) as u64;
                        unsafe { nat_dd_set_options(&ui.root, &format!("scrim_stdd_{}", SKEYS[f]), STRAT_OPTS[f], sel); }
                        last[f] = sel as i64;
                    }
                }
                repaint_strat(&mut ui.root); // 스플릿 행 표시/팀탭 갱신
            } else {
                let mut changed = false;
                {
                    let mut last = SDD_LAST.lock().unwrap();
                    let mut w = STRAT_WORKING.lock().unwrap();
                    for f in 0..12 {
                        if let Some(cur) = unsafe { nat_dd_selected(&ui.root, &format!("scrim_stdd_{}", SKEYS[f])) } {
                            if cur as i64 != last[f] && cur < STRAT_OPTS[f].len() {
                                last[f] = cur as i64; w[team][f] = cur as u8; changed = true;
                            }
                        }
                    }
                }
                if changed { repaint_strat(&mut ui.root); } // 스플릿(bld=스플릿/mor=1-3-1) 행 갱신
            }
        }
        let items_open = ITEMS_OPEN.load(Ordering::Relaxed);
        set_visible_by_id(&mut ui.root, "scrim_items_modal", items_open);
        if !items_open { IDD_INIT.store(false, Ordering::Relaxed); } // 닫히면 재주입 가드 리셋
        if items_open {
            // ── 인라인 아이템 dropdown(네이티브): 옵션 1회주입 + 선택 폴링 → ITEMS_WORKING ──
            if !IDD_INIT.swap(true, Ordering::Relaxed) {
                let cnt = item_opt_count();
                let labels: Vec<String> = (0..cnt).map(|i| item_opt_label(i as u8)).collect();
                let refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
                let w = *ITEMS_WORKING.lock().unwrap();
                let mut last = IDD_LAST.lock().unwrap();
                for sl in 0..10 { for s in 0..3 {
                    let sel = w[sl][s] as u64;
                    unsafe { nat_dd_set_options(&ui.root, &format!("scrim_idd_{}_{}", sl, s), &refs, sel); }
                    last[sl * 3 + s] = sel as i64;
                }}
            } else {
                let mut last = IDD_LAST.lock().unwrap();
                let mut w = ITEMS_WORKING.lock().unwrap();
                for sl in 0..10 { for s in 0..3 {
                    if let Some(cur) = unsafe { nat_dd_selected(&ui.root, &format!("scrim_idd_{}_{}", sl, s)) } {
                        let k = sl * 3 + s;
                        if cur as i64 != last[k] { last[k] = cur as i64; w[sl][s] = cur as u8; }
                    }
                }}
            }
        }

        // 모달 표시 + 라벨
        // ★진단(1회): scrim_result 조각이 실제로 주입됐는지 확인 (파싱실패 vs 표시로직 판별)
        if !RESDIAG_DONE.load(Ordering::Relaxed) {
            let hm = has_id(&ui.root, "scrim_modal");
            let hr = has_id(&ui.root, "scrim_result");
            if hm || hr {
                RESDIAG_DONE.store(true, Ordering::Relaxed);
                fs_log(&format!("★inject check: scrim_modal={} scrim_result={}", hm, hr));
            }
        }
        let modal_vis = SCRIM_MODAL_VISIBLE.load(Ordering::Relaxed);
        set_visible_by_id(&mut ui.root, "scrim_modal", modal_vis);
        if !modal_vis { PDD_INIT.store(false, Ordering::Relaxed); } // 모달 닫히면 dropdown 재주입 가드 리셋
        set_visible_by_id(&mut ui.root, "scrim_result", SCRIM_RESULT_VISIBLE.load(Ordering::Relaxed)); // ★M4 결과창
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

            // ── 인라인 선수 dropdown(네이티브): 옵션 1회주입 + 선택 폴링 → PLAYER_SLOTS ──
            if ready {
                let roster = ROSTER.lock().unwrap();
                if !roster.is_empty() {
                    if !PDD_INIT.swap(true, Ordering::Relaxed) {
                        // 최초 1회: 라벨 [0]="(선택)" + 로스터, 현재 슬롯선택 반영해 주입
                        let mut labels: Vec<String> = Vec::with_capacity(roster.len() + 1);
                        labels.push("(선택)".to_string());
                        for (_, nm) in roster.iter() { labels.push(nm.clone()); }
                        let refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
                        let ps = PLAYER_SLOTS.lock().unwrap();
                        let mut last = PDD_LAST.lock().unwrap();
                        let mb = unsafe { GetModuleHandleW(core::ptr::null()) as usize };
                        let mut diag = format!(
                            "roster.len={} dd_valid={} itemnet_valid={}\ndd_fn(0x{:x}): {}\nitemnet_fn(0x{:x}): {}\n",
                            roster.len(), unsafe { dd_addr_valid() }, unsafe { itemnet_addr_valid() },
                            FN_DD_SETOPT_RVA, unsafe { prologue_hex(mb + FN_DD_SETOPT_RVA, 16) },
                            ITEMNET_FORWARD_RVA, unsafe { prologue_hex(mb + ITEMNET_FORWARD_RVA, 16) });
                        for i in 0..10 {
                            let sel = ps[i].and_then(|aid| roster.iter().position(|(id, _)| *id == aid))
                                .map(|p| p + 1).unwrap_or(0);
                            let id = format!("scrim_pdd{}", i);
                            let info = pdd_diag_node(&ui.root, &id);
                            let ok = unsafe { nat_dd_set_options(&ui.root, &id, &refs, sel as u64) };
                            let (vlen, selidx) = match &info {
                                Some((b, _)) => unsafe {
                                    (*((*b + 0x1538) as *const usize), *((*b + 0x1788) as *const u64))
                                },
                                None => (88888usize, 88888u64),
                            };
                            diag.push_str(&format!("{}: type={} set_ok={} optVecLen={} sel={}\n",
                                id, info.as_ref().map(|x| x.1.as_str()).unwrap_or("NOTFOUND"),
                                ok, vlen, selidx));
                            last[i] = sel as i64;
                        }
                        let _ = std::fs::write("scrim_pdd_diag.txt", &diag);
                    } else {
                        // 이후 매프레임: 선택 변화 폴링 → 슬롯 반영 (0=선택해제)
                        let mut last = PDD_LAST.lock().unwrap();
                        let mut ps = PLAYER_SLOTS.lock().unwrap();
                        for i in 0..10 {
                            if let Some(cur) = unsafe { nat_dd_selected(&ui.root, &format!("scrim_pdd{}", i)) } {
                                if cur as i64 != last[i] {
                                    last[i] = cur as i64;
                                    ps[i] = if cur == 0 { None } else { roster.get(cur - 1).map(|(aid, _)| *aid) };
                                }
                            }
                        }
                    }
                }
            }
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
        // ★게임상태 베이스 비교용: scrim의 ClientDatabase 주소 1회 로깅 (sim_probe REG rcx와 대조).
        if !SCRIM_DBLOG_DONE.swap(true, Ordering::Relaxed) {
            let dbase = r as *const ClientDatabase as usize;
            unsafe {
                let hm = dbase.wrapping_add(0xf8b0);   // 매치엔트리 HashMap(hashbrown RawTable) 후보
                let ctrl = rd_u64(hm);
                let bmask = rd_u64(hm + 8);
                let items = rd_u64(hm + 0x18);
                let q_cap = rd_u64(dbase.wrapping_add(0x800));
                let q_ptr = rd_u64(dbase.wrapping_add(0x808));
                let q_len = rd_u64(dbase.wrapping_add(0x810));
                cn_log(&format!(
                    "★ClientDatabase 베이스=0x{:x}\n   HashMap@+0xf8b0: ctrl=0x{:x}(readable={}) bucket_mask=0x{:x} items={}\n   큐@+0x800: cap={} ptr=0x{:x} len={}",
                    dbase, ctrl, readable(ctrl as usize, 8), bmask, items as i64, q_cap as i64, q_ptr, q_len as i64));
            }
        }
        // ★M4 프레임스캔 검증: game_view 프레임이 언제 다 차는지 + 슬롯별 최종 PlayerStatistics 추적.
        //   목적 = 15초 선연산 후 관전 없이도 frames 전부 차는지 확인(차면 즉시 brief 가능).
        if SCRIM_ARMED.load(Ordering::Relaxed) {
            FRAMESCAN_TICK.fetch_add(1, Ordering::Relaxed);
            // ★매 프레임 호출 — scan_replay_stats가 호출당 SCAN_BUDGET(24)프레임만 처리해 부하 분산.
            //   초반 버퍼링 대량유입도 프레임당 부담이 작아 끊김 없이 잘게 따라잡음(brief 정확도 동일).
            match scan_replay_stats(r) {
                None => {
                    // 재생 끝남(game_view 사라짐) + 데이터 있었음 → 최종 brief 1회 확정
                    if HAD_GAMEVIEW.load(Ordering::Relaxed) && !BRIEF_FINALIZED.swap(true, Ordering::Relaxed) {
                        if let Some((st, agg)) = *LAST_BRIEF.lock().unwrap() {
                            write_brief(&st, &agg);
                            RESULT_READY.store(true, Ordering::Relaxed); // 팝업 닫히면 결과창 표시
                            AUTO_CLOSE_ARMED.store(true, Ordering::Relaxed); // 재생후 중간팝업 자동닫기
                        }
                    }
                }
                Some((st, _total, agg)) => {
                    *LAST_BRIEF.lock().unwrap() = Some((st, agg));
                    HAD_GAMEVIEW.store(true, Ordering::Relaxed);
                }
            }
        }
        // ── [아이템칸 검증] 재생 중 라이브 보유 아이템 추적 (주입 무관, 베이스라인부터 동작) ──
        //    GamePlayerStateData.items = 선수 엔티티가 실제 보유한 아이템(Vec<String>, 점점 늘어남).
        //    items@256(타깃 빌드)이 아니라 buy/grow 결과. 4번째가 뜨면 = 시뮬이 4 소비 = 캡없음 증거.
        if let Some(gv) = r.game_view.as_ref().filter(|_| ITEMLOG_ON) {
            let frames = &gv.client.events;
            let total = frames.len();
            let mut start = ITEMLOG_POS.load(Ordering::Relaxed);
            if start > total { start = 0; } // 새 재생(프레임 Vec 리셋) 감지
            if start < total {
                let mut g = ITEMLOG_MAX.lock().unwrap();
                let map = g.get_or_insert_with(HashMap::new);
                for fi in start..total {
                    for ev in frames[fi].events.iter() {
                        let s = format!("{:?}", ev);
                        if s.contains("GamePlayerState") {
                            if let (Some(nm), Some((cnt, names))) = (dbg_quoted(&s, "name:"), dbg_items(&s)) {
                                let id = dbg_i64(&s, ", id:").or_else(|| dbg_i64(&s, "{ id:")).unwrap_or(-1);
                                let team = dbg_i64(&s, "team:").unwrap_or(-1);
                                if (0..10).contains(&id) { COUNT_BY_SLOT.lock().unwrap()[id as usize] = cnt; }
                                let e = map.entry(format!("{}#{}", nm, id)).or_insert(0);
                                if cnt > *e {
                                    *e = cnt;
                                    itemlog_write(&format!(
                                        "[{}ms] f{}/{} slot{} t{} {}: {}개 [{}]", now_ms(), fi, total, id, team, nm, cnt, names));
                                }
                            }
                        } else if s.contains("PlayerStatistics") {
                            // ★ 골드 마일스톤(2000단위)마다 전원(slot0~9)의 현재 아이템수+골드 기록.
                            //   골드 충분(높은마일스톤)한데도 3개에 멈추면 = 캡 확정 / 4개면 = 캡없음.
                            if let (Some(team), Some(pi), Some(gold)) = (dbg_i64(&s, "team:"), pos_idx(&s), dbg_i64(&s, "gold:")) {
                                let slot = team * 5 + pi;
                                if (0..10).contains(&slot) {
                                    let ms = gold / 2000;
                                    let mut msb = GOLD_MS_BY_SLOT.lock().unwrap();
                                    if ms > msb[slot as usize] {
                                        msb[slot as usize] = ms;
                                        let cnt = COUNT_BY_SLOT.lock().unwrap()[slot as usize];
                                        itemlog_write(&format!("[GOLD] f{}/{} slot{} {}개 gold={}", fi, total, slot, cnt, gold));
                                    }
                                }
                            }
                        }
                    }
                }
                ITEMLOG_POS.store(total, Ordering::Relaxed);
            }
        }
        guard_tick(); // [캡추적] 펜딩 폴트 기록 + 가드 재무장
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

        // ── 재생 종료 후 다시 뜨는 중간 ReplayPopup 자동닫기 (A 흐름: 바로 결과창으로) ──
        //    팝업이 떠 있고 top 이 disc6(ReplayPopup)이면 pop → popup_now=false → 결과창 표시 경로 발동.
        if AUTO_CLOSE_ARMED.load(Ordering::Relaxed) && !replay_some {
            if unsafe { replay_popup_present(r) } {
                drop(db);
                let mut dbm = data.db_mut();
                let v = &mut dbm.pause_stack;
                let n = v.len();
                if n > 0 {
                    unsafe {
                        let p = v.as_ptr() as *const u8;
                        if std::ptr::read_unaligned(p.add((n - 1) * 32) as *const u32) == 6 {
                            v.set_len(n - 1); // ReplayPopup=32B POD(disc+MatchType) → Drop 불필요
                            AUTO_CLOSE_ARMED.store(false, Ordering::Relaxed);
                            fs_log("★ 중간 ReplayPopup 자동닫기 (top disc6 pop)");
                        } else {
                            AUTO_CLOSE_ARMED.store(false, Ordering::Relaxed); // top 이 우리것 아님 → 수동닫기로
                        }
                    }
                }
                return; // 다음 프레임: popup_now=false → 결과창 표시
            }
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
                // ★M4 진단: 재생 후 statistics (복원 전에 읽어야 함). ARM-template 값과 비교 → 영속 여부.
                { let _k = INJECT_KEY.load(Ordering::Relaxed); if _k >= 0 { log_result_stats(r, _k as usize, "popup-close"); } }
                // ★M4 결과창: brief 확정됐으면 팝업 닫힌 직후 결과요약 표시
                if RESULT_READY.swap(false, Ordering::Relaxed) {
                    fs_log(&format!("popup-close finalize: result_node={} brief={}",
                        has_id(&ui.root, "scrim_result"), LAST_BRIEF.lock().unwrap().is_some()));
                    if let Some((st, agg)) = *LAST_BRIEF.lock().unwrap() {
                        populate_result(&mut ui.root, &st, &agg);
                        SCRIM_RESULT_VISIBLE.store(true, Ordering::Relaxed);
                        fs_log("populate_result 완료 → SCRIM_RESULT_VISIBLE=true");
                    }
                }
                unsafe { restore_backup(r); }
                CAPTURED.store(false, Ordering::Relaxed);
                SCRIM_ARMED.store(false, Ordering::Relaxed);
                INJECTED_ONCE.store(false, Ordering::Relaxed); // 다음 세션 위해 리셋
                ITEMLOG_POS.store(0, Ordering::Relaxed);        // [아이템칸검증] 다음 재생 위해 리셋
                *ITEMLOG_MAX.lock().unwrap() = None;
                *COUNT_BY_SLOT.lock().unwrap() = [0; 10];
                *GOLD_MS_BY_SLOT.lock().unwrap() = [0; 10];
                // [캡추적] 가드 해제(재무장 중단). 누수페이지는 그냥 둠(무해).
                if GUARD_BASE.load(Ordering::Relaxed) != 0 {
                    let mut old = 0u32;
                    unsafe { VirtualProtect(GUARD_BASE.load(Ordering::Relaxed) as usize, 0x1000, 0x04, &mut old); }
                    GUARD_BASE.store(0, Ordering::Relaxed);
                }
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
    // ★성능: 이 훅을 어느 엔티티에도 붙이지 않음 → think 미호출(매 엔티티·매 틱 FFI 제거).
    //   엔티티-stat 진단(PROBE_ENTITY, 1회 m==300 덤프)은 이미 역할 끝남. 재생 관전 버벅임 회귀 수정.
    //   (동작 영향 0: think는 passthrough였음. 재활성 필요시 true 또는 OUR_ATHLETES 필터로.)
    fn matches(&self, _ctx: &PlayerAiInitContext) -> bool { false }
    fn think(&mut self, ctx: &mut PlayerAiContext<'_, '_, '_>, base_input: Option<Input>) -> PlayerInputDecision {
        let aid = ctx.athlete_id();
        // ── [캡추적2] entity 프로브: ours 무관, 아무 선수나 1회 (think 동작확인 + entity 구조 탐색) ──
        const PROBE_ENTITY: bool = true;
        if PROBE_ENTITY {
            let m = THINK_ALL.fetch_add(1, Ordering::Relaxed);
            if m == 300 {
                let cp = ctx as *const _ as usize;
                let mut s = format!("=== ENTITY PROBE champ={} aid={} hp={:?} maxhp={:?} tick={} ctx@0x{:x} ===\n",
                    ctx.champion_name(), aid, ctx.hp(), ctx.max_hp(), ctx.tick(), cp);
                unsafe {
                    let pk = |a: usize| if readable(a, 8) { format!("{:x}", rd_u64(a)) } else { "?".into() };
                    for i in 0..40usize {
                        let a = cp + i * 8;
                        if !readable(a, 8) { continue; }
                        let v = rd_u64(a);
                        s.push_str(&format!(" ctx[+0x{:03x}]=0x{:x}", i * 8, v));
                        if v >= 0x10000 && v < (1u64 << 48) && readable(v as usize, 8) {
                            // 이 포인터를 entity 후보로: +0x610(maxhp분모)/+0x658(hp분자) + items@0x100(Vec) 후보
                            s.push_str(&format!("  →[+0:{} +8:{} +10:{}] e?[mhp@610:{} hp@658:{}] itemsVec@100?[cap:{} ptr:{} len:{}]",
                                pk(v as usize), pk(v as usize + 8), pk(v as usize + 0x10),
                                pk(v as usize + 0x610), pk(v as usize + 0x658),
                                pk(v as usize + 0x100), pk(v as usize + 0x108), pk(v as usize + 0x110)));
                        }
                        s.push('\n');
                    }
                }
                write_log("scrim_entity.txt", &s);
            }
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
const ITEMSET_DUMP: bool = false; // (구) 모드템 위치찾기 — scan_mod_items(P2)로 대체, 끔
const ITEMMASTER_DUMP: bool = false; // ★ 라이브힙 브루트스캔 → TOCTOU 프리징 위험. 비활성. (B)는 재생상관법으로.
const ITEMMASTER_SAFE: bool = false; // ★ 이것도 멈춤 → 메모리추적 전면 폐기. (B)는 재생상관법으로.
const ITEMVEC_DUMP: bool = false; // ★ 또 멈춤 → 게임 힙 직접읽기는 이 게임에서 근본적으로 불안정. 폐기. (B)는 (나) 수동매핑으로.
static ITEMVEC_DONE: AtomicBool = AtomicBool::new(false);
static ITEMVEC_TRIES: AtomicUsize = AtomicUsize::new(0);
static MASTER_SAFE_DONE: AtomicBool = AtomicBool::new(false);
static MASTER_SAFE_TRIES: AtomicUsize = AtomicUsize::new(0);
// JSON 바닐라 30개 key (순서 = ID 0..29). 메모리 마스터목록 검증용 지문.
const VANILLA_KEYS: [&str; 30] = [
    "iron_blade","soldiers_longsword","ruinous_blade","conquerors_greatsword","warlords_final_judgement",
    "dagger","wind_dagger","twin_stormblade","thunderclaw","storm_sovereign",
    "steel_armor","gatekeepers_armor","black_knights_heavy_plate","eternal_iron_plate","impregnable_fortress",
    "mystic_cloak","night_hood","dusk_raven","souls_edge","veil_of_annihilation",
    "arcane_crystal","spirit_crystal","staff_of_rapture","angels_fang","prophet_of_the_abyss",
    "vital_orb","hardened_heart","ring_of_reincarnation","hourglass_of_eternity","giants_horn_shard",
];
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
// region 단위로 needle 바이트열 위치 탐색 (byte 단위 VirtualQuery 회피)
unsafe fn scan_for_bytes(start: usize, max_end: usize, needle: &[u8], max_hits: usize, hits: &mut Vec<usize>) {
    const MEM_COMMIT: u32 = 0x1000;
    const READABLE: u32 = 0x02 | 0x04 | 0x20 | 0x40;
    const GUARD: u32 = 0x01 | 0x100;
    let nl = needle.len();
    if nl == 0 { return; }
    let mut a = start;
    while a < max_end && hits.len() < max_hits {
        let mut mbi = MemBasicInfo::default();
        let n = VirtualQuery(a as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>());
        if n == 0 { break; }
        let region_end = (mbi.base + mbi.region_size).max(a + 0x1000);
        let ok = mbi.state == MEM_COMMIT && (mbi.protect & READABLE) != 0 && (mbi.protect & GUARD) == 0;
        if ok {
            let scan_end = region_end.min(max_end);
            let mut q = a;
            while q + nl <= scan_end {
                let hay = std::slice::from_raw_parts(q as *const u8, nl);
                if hay == needle { hits.push(q); if hits.len() >= max_hits { break; } }
                q += 1;
            }
        }
        a = region_end;
    }
}
// region 단위로 8바이트값 == target 인 위치 탐색
unsafe fn scan_for_ptr(start: usize, max_end: usize, target: usize, max_hits: usize, hits: &mut Vec<usize>) {
    const MEM_COMMIT: u32 = 0x1000;
    const READABLE: u32 = 0x02 | 0x04 | 0x20 | 0x40;
    const GUARD: u32 = 0x01 | 0x100;
    let mut a = start & !7;
    while a < max_end && hits.len() < max_hits {
        let mut mbi = MemBasicInfo::default();
        let n = VirtualQuery(a as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>());
        if n == 0 { break; }
        let region_end = (mbi.base + mbi.region_size).max(a + 0x1000);
        let ok = mbi.state == MEM_COMMIT && (mbi.protect & READABLE) != 0 && (mbi.protect & GUARD) == 0;
        if ok {
            let scan_end = region_end.min(max_end);
            let mut q = (a + 7) & !7;
            while q + 8 <= scan_end {
                if std::ptr::read_unaligned(q as *const usize) == target { hits.push(q); if hits.len() >= max_hits { break; } }
                q += 8;
            }
        }
        a = region_end;
    }
}
// ★ JSON 바닐라 순서를 지문으로 메모리에서 마스터 아이템목록을 찾아 0..80 인덱스의 key 덤프.
fn dump_master_items(db: usize) {
    if !ITEMMASTER_DUMP { return; }
    unsafe {
        let mut s = format!("[{}ms] ★ 마스터 아이템목록 탐색 (db={:#x})\n", now_ms(), db);
        // 1) ItemSetting 영역 포인터 시드 수집
        let mut seeds: Vec<usize> = Vec::new();
        let mut off = 0x12d00usize;
        while off <= 0x12e00 {
            if readable(db + off, 8) {
                let v = rd_u64(db + off) as usize;
                if looks_heap(v as u64) && readable(v, 16) { seeds.push(v); }
            }
            off += 8;
        }
        seeds.push(db);
        seeds.sort_unstable(); seeds.dedup();
        s.push_str(&format!("  시드 {}개: {:?}\n", seeds.len(),
            seeds.iter().map(|x| format!("{:#x}", x)).collect::<Vec<_>>()));
        // key 읽기: pp = 포인터가 든 위치, *(pp)=버퍼주소, 거기서 ascii key 읽기
        let read_key = |pp: usize| -> Option<String> { unsafe {
            if !readable(pp, 8) { return None; }
            let buf = std::ptr::read_unaligned(pp as *const usize);
            if buf <= 0x10000 { return None; }
            let cap = if readable(buf, 48) { 48 } else if readable(buf, 8) { 8 } else { return None; };
            let mut v = Vec::new();
            for i in 0..cap {
                let c = *((buf + i) as *const u8);
                if c == b'_' || c.is_ascii_lowercase() || c.is_ascii_digit() { v.push(c); } else { break; }
            }
            if v.len() >= 3 { String::from_utf8(v).ok() } else { None }
        }};
        // 2) 'iron_blade' 버퍼 위치(B0) 찾기
        let needle0 = VANILLA_KEYS[0].as_bytes();
        let mut b0_hits: Vec<usize> = Vec::new();
        for &seed in seeds.iter() {
            scan_for_bytes(seed.saturating_sub(0x2000), seed + 0x60000, needle0, 48, &mut b0_hits);
            if b0_hits.len() >= 48 { break; }
        }
        b0_hits.sort_unstable(); b0_hits.dedup();
        s.push_str(&format!("  'iron_blade' 버퍼 {}곳: {:?}\n", b0_hits.len(),
            b0_hits.iter().take(12).map(|x| format!("{:#x}", x)).collect::<Vec<_>>()));
        // 3) 각 B0 의 포인터위치(P0) → stride 도출 + JSON 순서 검증
        let needle1 = VANILLA_KEYS[1].as_bytes();
        let mut found = false;
        'outer: for &b0 in b0_hits.iter() {
            let mut p0_hits: Vec<usize> = Vec::new();
            for &seed in seeds.iter() {
                scan_for_ptr(seed.saturating_sub(0x2000), seed + 0x60000, b0, 16, &mut p0_hits);
            }
            p0_hits.sort_unstable(); p0_hits.dedup();
            for &p0 in p0_hits.iter() {
                let mut cand_s = 8usize;
                while cand_s <= 0x800 {
                    if let Some(k1) = read_key(p0 + cand_s) {
                        if k1.as_bytes() == needle1 {
                            let stride = cand_s;
                            let mut ok = 0usize;
                            for i in 0..30 {
                                match read_key(p0 + i * stride) {
                                    Some(k) if k.as_bytes() == VANILLA_KEYS[i].as_bytes() => ok += 1,
                                    _ => break,
                                }
                            }
                            if ok >= 8 {
                                s.push_str(&format!(
                                    "\n  ★★ 마스터목록 발견! P0={:#x} stride={} (0x{:x}) 검증={}/30\n",
                                    p0, stride, stride, ok));
                                s.push_str("  idx | key | tier?(@block+0x190)\n");
                                let mut miss = 0;
                                for i in 0..80usize {
                                    match read_key(p0 + i * stride) {
                                        Some(k) => {
                                            miss = 0;
                                            let block = (p0 + i * stride).wrapping_sub(8);
                                            let tier = if readable(block + 0x190, 4) {
                                                std::ptr::read_unaligned((block + 0x190) as *const u32) as i64
                                            } else { -1 };
                                            let tag = if i >= 30 { "  ←모드?" } else { "" };
                                            s.push_str(&format!("  {:>3} | {:<30} tier?={}{}\n", i, k, tier, tag));
                                        }
                                        None => {
                                            miss += 1;
                                            s.push_str(&format!("  {:>3} | <무효>\n", i));
                                            if i > 30 && miss >= 3 { break; }
                                        }
                                    }
                                }
                                found = true;
                                break 'outer;
                            }
                        }
                    }
                    cand_s += 8;
                }
            }
        }
        if !found {
            s.push_str("\n  ✗ 마스터목록 미발견 — 지문 stride 검증 실패. (다음: b0_hits 주변 raw 덤프)\n");
        }
        write_log("scrim_master_items.txt", &s);
    }
}
// ★ 안전 마스터목록 추적: 게임스레드에서 1회 성공까지. ItemSetting 구조체(512B)에서 힙 Vec 후보를
//   찾아 element0=iron_blade 인지 확인 → 8KB 내에서 stride 도출 → JSON순서 검증 → 0..90 key 덤프.
//   브루트 힙스캔 없음. 발견 시 true.
fn dump_master_safe(db: usize, last: bool) -> bool {
    if !ITEMMASTER_SAFE { return true; }
    unsafe {
        // pp 위치에 든 포인터가 가리키는 버퍼에서 ascii key 읽기
        let read_key = |pp: usize| -> Option<String> { unsafe {
            if !readable(pp, 8) { return None; }
            let buf = rd_u64(pp) as usize;
            if buf <= 0x10000 || !readable(buf, 1) { return None; }
            let cap = if readable(buf, 40) { 40 } else if readable(buf, 8) { 8 } else { return None; };
            let mut v = Vec::new();
            for i in 0..cap {
                let c = *((buf + i) as *const u8);
                if c == b'_' || c.is_ascii_lowercase() || c.is_ascii_digit() { v.push(c); } else { break; }
            }
            if v.len() >= 3 { String::from_utf8(v).ok() } else { None }
        }};
        let i0 = VANILLA_KEYS[0].as_bytes();
        let i1 = VANILLA_KEYS[1].as_bytes();
        let mut s = format!("[{}ms] ★ 안전 마스터목록 추적 (db={:#x})\n", now_ms(), db);

        let dump_from = |s: &mut String, label: &str, off: usize, p: usize, keyoff: usize, stride: usize, model_b: bool| -> bool { unsafe {
            // model_b=false: elem_i = p + i*stride ; model_b=true: elem_i = *(p + i*8)
            let elem_base = |i: usize| -> usize { unsafe {
                if model_b { if readable(p + i*8, 8) { rd_u64(p + i*8) as usize } else { 0 } }
                else { p + i*stride }
            }};
            s.push_str(&format!("  ★★ 발견[{}]! structOff={:#x} p={:#x} keyoff={} stride={}\n", label, off, p, keyoff, if model_b {8} else {stride}));
            s.push_str("  idx | key | tier?(@elem+0x190)\n");
            let mut miss = 0;
            for i in 0..90usize {
                let eb = elem_base(i);
                match read_key(eb + keyoff) {
                    Some(k) => { miss = 0;
                        let tier = if readable(eb + 0x190, 4) { std::ptr::read_unaligned((eb + 0x190) as *const u32) as i64 } else { -1 };
                        let tag = if i >= 30 { "  ←모드" } else { "" };
                        s.push_str(&format!("  {:>3} | {:<30} tier?={}{}\n", i, k, tier, tag));
                    }
                    None => { miss += 1; s.push_str(&format!("  {:>3} | <무효>\n", i)); if i > 30 && miss >= 3 { break; } }
                }
            }
            true
        }};

        // 후보 검증: 모델별로 0..30 이 VANILLA_KEYS 순서와 일치(>=8)?
        let mut off = 0x12d00usize;
        while off < 0x12f00 {
            if readable(db + off, 8) {
                let p = rd_u64(db + off) as usize;
                if looks_heap(p as u64) && readable(p, 0x40) {
                    // ── Model A: 인라인 블록 (key at p+keyoff, stride 도출) ──
                    for keyoff in (0..0x60usize).step_by(8) {
                        if read_key(p + keyoff).map_or(false, |k| k.as_bytes() == i0) {
                            let mut stride = 0usize;
                            let mut a = p + keyoff + 8;
                            let end = p + keyoff + 0x2000;
                            while a + 8 <= end {
                                if readable(a, 8) {
                                    if read_key(a).map_or(false, |k| k.as_bytes() == i1) { stride = a - (p + keyoff); break; }
                                    a += 8;
                                } else { a = (a + 0x1000) & !0xfff; }
                            }
                            if stride >= 8 {
                                let mut ok = 0usize;
                                for i in 0..30 {
                                    match read_key(p + keyoff + i * stride) {
                                        Some(k) if k.as_bytes() == VANILLA_KEYS[i].as_bytes() => ok += 1, _ => break,
                                    }
                                }
                                if ok >= 8 { dump_from(&mut s, "A", off, p, keyoff, stride, false);
                                    write_log("scrim_master_safe.txt", &s); return true; }
                            }
                        }
                    }
                    // ── Model B: 포인터 배열 (elem_i = *(p+i*8), key at block+keyoff) ──
                    if readable(p, 8) {
                        let b0 = rd_u64(p) as usize;
                        if looks_heap(b0 as u64) && readable(b0, 0x20) {
                            for keyoff in (0..0x60usize).step_by(8) {
                                if read_key(b0 + keyoff).map_or(false, |k| k.as_bytes() == i0) {
                                    let b1 = if readable(p + 8, 8) { rd_u64(p + 8) as usize } else { 0 };
                                    let ok1 = b1 > 0x10000 && read_key(b1 + keyoff).map_or(false, |k| k.as_bytes() == i1);
                                    if ok1 {
                                        let mut ok = 0usize;
                                        for i in 0..30 {
                                            if !readable(p + i*8, 8) { break; }
                                            let bi = rd_u64(p + i*8) as usize;
                                            match read_key(bi + keyoff) {
                                                Some(k) if k.as_bytes() == VANILLA_KEYS[i].as_bytes() => ok += 1, _ => break,
                                            }
                                        }
                                        if ok >= 8 { dump_from(&mut s, "B", off, p, keyoff, 8, true);
                                            write_log("scrim_master_safe.txt", &s); return true; }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            off += 8;
        }

        if last {
            // ── 진단: 구조체 내 힙포인터들이 무엇을 가리키는지 (레이아웃 파악용) ──
            s.push_str("  ✗ 자동검출 실패. 진단 덤프 (structOff: p [raw u64들] | 인라인key(p+0/+8) | 포인터배열 *(p)+0/+8):\n");
            let mut shown = 0;
            let mut off = 0x12d00usize;
            while off < 0x12f00 && shown < 28 {
                if readable(db + off, 8) {
                    let p = rd_u64(db + off) as usize;
                    if looks_heap(p as u64) && readable(p, 0x20) {
                        shown += 1;
                        let raw: Vec<String> = (0..4).map(|i| if readable(p+i*8,8) { format!("{:#x}", rd_u64(p+i*8)) } else { "?".into() }).collect();
                        let k_inline = format!("{:?}/{:?}", read_key(p), read_key(p+8));
                        let b0 = rd_u64(p) as usize;
                        let k_ptrarr = if looks_heap(b0 as u64) && readable(b0, 0x20) {
                            format!("*(p)={:#x} k+0/+8/+16={:?}/{:?}/{:?}", b0, read_key(b0), read_key(b0+8), read_key(b0+16))
                        } else { "*(p)=비힙".into() };
                        s.push_str(&format!("  +{:#06x}: p={:#x} raw=[{}]\n        inline:{}\n        ptrarr:{}\n",
                            off, p, raw.join(", "), k_inline, k_ptrarr));
                    }
                }
                off += 8;
            }
            write_log("scrim_master_safe.txt", &s);
        }
        false
    }
}
// ★ (가) ItemSetting 의 "순서있는 key Vec" 직접 읽기.
//   drop 함수 분석: 구조체에 24바이트 stride Vec 가 {cap@voff, ptr@voff+8, len@voff+0x10} 로 있음.
//   후보 base × voff 로 Vec 삼중자를 읽고, element[0]/[1] 이 iron_blade/soldiers 인지 검증된 경우에만 0..len 덤프.
//   스캔 없음, 추측 이중역참조 없음(삼중자 sane + readable 가드 후에만 역참조). 실패 시 안전 중단.
fn dump_item_vec(db: usize, last: bool) -> bool {
    if !ITEMVEC_DUMP { return true; }
    unsafe {
        // loc 에 든 포인터(혹은 loc 자체가 버퍼) → ascii key. mode0: buf=*(loc); mode1: buf=loc.
        let read_key = |loc: usize| -> Option<String> { unsafe {
            if !readable(loc, 8) { return None; }
            let buf = rd_u64(loc) as usize;
            if buf <= 0x10000 || !readable(buf, 1) { return None; }
            let cap = if readable(buf, 40) { 40 } else if readable(buf, 8) { 8 } else { return None; };
            let mut v = Vec::new();
            for i in 0..cap {
                let c = *((buf + i) as *const u8);
                if c == b'_' || c.is_ascii_lowercase() || c.is_ascii_digit() { v.push(c); } else { break; }
            }
            if v.len() >= 3 { String::from_utf8(v).ok() } else { None }
        }};
        let i0 = "iron_blade"; let i1 = "soldiers_longsword";
        // 후보 ItemSetting base (db 기준). 인수인계 db+0x12d58(hashbrown) 에서 역산한 값들 + 직접값.
        let bases: [usize; 8] = [0x12d00, 0x12bc0, 0x12bd8, 0x12bc8, 0x12cc0, 0x12c00, 0x12d08, 0x12be0];
        let voffs: [usize; 3] = [0x108, 0x128, 0x0f0];
        let kos: [usize; 3] = [0, 8, 0x10]; // element 내 key String 시작 오프셋 후보
        let mut diag = format!("[{}ms] ★ (가) 순서 key Vec 탐색 (db={:#x})\n", now_ms(), db);
        for &boff in bases.iter() {
            let is = db + boff;
            for &voff in voffs.iter() {
                if !readable(is + voff, 0x18) { continue; }
                let cap = rd_u64(is + voff) as usize;
                let ptr = rd_u64(is + voff + 8) as usize;
                let len = rd_u64(is + voff + 0x10) as usize;
                // 삼중자 sane 검사 (역참조 전에)
                if !looks_heap(ptr as u64) || len < 20 || len > 2000 || cap < len || cap > 100000 { continue; }
                if !readable(ptr, 24) { continue; }
                diag.push_str(&format!("  후보 is={:#x}+{:#x} voff={:#x}: cap={} ptr={:#x} len={}\n", db, boff, voff, cap, ptr, len));
                for &ko in kos.iter() {
                    // element[0] 의 key == iron_blade ?
                    if read_key(ptr + 0*24 + ko).as_deref() == Some(i0)
                        && read_key(ptr + 1*24 + ko).as_deref() == Some(i1) {
                        let n = len.min(90);
                        let mut s = format!("[{}ms] ★★ 순서 key Vec 발견! is=db+{:#x} voff={:#x} ko={} ptr={:#x} len={}\n", now_ms(), boff, voff, ko, ptr, len);
                        s.push_str("  idx | key\n");
                        for i in 0..n {
                            match read_key(ptr + i*24 + ko) {
                                Some(k) => { let tag = if i >= 30 { "  ←모드" } else { "" };
                                    s.push_str(&format!("  {:>3} | {}{}\n", i, k, tag)); }
                                None => s.push_str(&format!("  {:>3} | <무효>\n", i)),
                            }
                        }
                        write_log("scrim_itemvec.txt", &s);
                        return true;
                    }
                }
                // 검증 실패한 후보: element[0] raw 진단 (레이아웃 파악용)
                let raw: Vec<String> = (0..4).map(|i| if readable(ptr+i*8,8) { format!("{:#x}", rd_u64(ptr+i*8)) } else { "?".into() }).collect();
                diag.push_str(&format!("    elem0 raw=[{}] *(elem0)key={:?} *(elem0+8)key={:?}\n",
                    raw.join(", "), read_key(ptr), read_key(ptr+8)));
            }
        }
        if last {
            diag.push_str("  ✗ 검증된 순서 key Vec 못 찾음. 위 후보 raw 로 다음 판단.\n");
            write_log("scrim_itemvec.txt", &diag);
        }
        false
    }
}
fn server_patch_work(ctx: &mut ServerModContext, where_: &str) {
    seh_selftest(); // ★ SEH 안전읽기 자기검증 (1회)
    // ★ (가) 순서 key Vec 직접읽기 (게임스레드, 검증후에만 덤프, 최대 12틱)
    if ITEMVEC_DUMP && !ITEMVEC_DONE.load(Ordering::Relaxed) {
        let t = ITEMVEC_TRIES.fetch_add(1, Ordering::Relaxed);
        if t < 12 {
            unsafe {
                let cps = &ctx.database.champion_patch_statistics as *const _ as usize;
                let db = cps.wrapping_sub(0x16698);
                if dump_item_vec(db, t == 11) { ITEMVEC_DONE.store(true, Ordering::Relaxed); }
            }
        } else { ITEMVEC_DONE.store(true, Ordering::Relaxed); }
    }
    // ★ 안전 마스터목록 추적 (게임스레드·바운디드·발견까지 재시도, 최대 60틱)
    if ITEMMASTER_SAFE && !MASTER_SAFE_DONE.load(Ordering::Relaxed) {
        let t = MASTER_SAFE_TRIES.fetch_add(1, Ordering::Relaxed);
        if t < 60 {
            unsafe {
                let cps = &ctx.database.champion_patch_statistics as *const _ as usize;
                let db = cps.wrapping_sub(0x16698);
                if dump_master_safe(db, t == 59) { MASTER_SAFE_DONE.store(true, Ordering::Relaxed); }
            }
        } else { MASTER_SAFE_DONE.store(true, Ordering::Relaxed); }
    }
    // ★ Database 주소 잡기 + item_network(+0xda0) 검증
    if !DB_PROBED.swap(true, Ordering::Relaxed) {
        unsafe {
            // &ctx.database 는 Database 시작이 아님 (ServerModContext 구조).
            // 알려진 필드(champion_patch_statistics @ Database+0x16698)의 절대주소로 역산.
            let cps = &ctx.database.champion_patch_statistics as *const _ as usize;
            let db = cps - 0x16698;       // 진짜 Database 시작
            let item_net = db + 0xda0;    // LogisticSGDAgent
            ITEM_NET_ADDR.store(item_net, Ordering::Relaxed); // B방식 호출용 저장
            // ★ MOD_REGISTRY(id-30→모드템 key) 캐시는 기능필수: 스크림 아이템리스트 활성필터(filter_enabled_ids)가 의존.
            //   dump_mod_items 가 이걸 채운다 → 진단(EXPLORE_DUMP) 와 분리해 *항상* 백그라운드 구동해야 함.
            //   (예전엔 dump_mod_items 가 EXPLORE_DUMP 게이트의 safe_explore 안에만 있어서, EXPLORE_DUMP=false 로
            //    끄면 레지스트리가 비어 모드템이 전부 필터링됐었음. 파일로그는 LOG_ENABLED=false 로 억제.
            //    MODITEMS_DONE 1회가드라 아래 safe_explore 의 dump_mod_items 호출과 중복돼도 무해.)
            {
                let edb = db;
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    seh_install();
                    unsafe { dump_mod_items(edb); }
                });
            }
            // (다) safe_read 진단 덤프: 디버그 전용 (게임스레드 안 막음, 로딩 후 실행)
            if EXPLORE_DUMP {
                let (edb, enet, ecps) = (db, item_net, cps);
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    unsafe { safe_explore(edb, enet, ecps); }
                });
            }
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
                    let fn_addr = mbase + ITEMNET_FORWARD_RVA; // 0x1419a4c00 - 0x140000000
                    s.push_str(&format!("  module_base = {:#x}\n", mbase));
                    s.push_str(&format!("  FUN_1419a4c00 주소 = {:#x}\n", fn_addr));
                    if readable(fn_addr, 16) {
                        let bytes: Vec<u8> = (0..16).map(|i| *((fn_addr + i) as *const u8)).collect();
                        let hex: String = bytes.iter().map(|b| format!("{:02x} ", b)).collect();
                        s.push_str(&format!("  함수 시작 바이트: {}\n", hex));
                        s.push_str("  (기대값: 55 41 57 41 56 41 55 41 54 56 57 53 48 81 ec)\n");
                        // 바이트 일치하면 트램폴린 후킹 설치
                        let expect = [0x55u8,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53,0x48,0x81,0xec];
                        if bytes[..15] == expect && ITEMNET_CAPTURE_HOOK {
                            s.push_str("  ★바이트 일치 → 트램폴린 후킹 설치 시도\n");
                            match install_itemnet_hook() {
                                Ok(stub) => s.push_str(&format!("  후킹 설치 성공! stub={:#x}\n", stub)),
                                Err(e) => s.push_str(&format!("  후킹 설치 실패: {}\n", e)),
                            }
                        } else if bytes[..15] == expect {
                            s.push_str("  (itemnet_capture 진단훅 비활성=배포 — 병렬sim 워커스레드 크래시 방지)\n");
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
            // ★ 마스터 아이템목록 탐색 (백그라운드: 로딩 후 6/12/18초에 시도, 마지막이 최종본)
            if ITEMMASTER_DUMP {
                let db_t = db;
                std::thread::spawn(move || {
                    for _ in 0..3 {
                        std::thread::sleep(std::time::Duration::from_secs(6));
                        dump_master_items(db_t);
                    }
                });
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
    // ★ forward 에 등장한 전체 아이템 ID 덤프 (새 ID 생길 때만 갱신)
    {
        let cur_len = SEEN_ITEM_IDS.lock().unwrap().len();
        if cur_len != SEEN_IDS_LOGGED_LEN.swap(cur_len, Ordering::Relaxed) {
            let mut ids = SEEN_ITEM_IDS.lock().unwrap().clone();
            ids.sort_unstable();
            let vanilla: Vec<u64> = ids.iter().cloned().filter(|&v| v < 30).collect();
            let mods: Vec<u64> = ids.iter().cloned().filter(|&v| v >= 30).collect();
            write_log("scrim_item_ids.txt", &format!(
                "[{}ms] forward 에 등장한 전체 아이템 ID (총 {}개)\n  바닐라(<30): {:?}\n  ★모드(>=30): {:?}\n  전체정렬: {:?}\n",
                now_ms(), ids.len(), vanilla, mods, ids));
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

// ===========================================================================
//  ★ 흡수된 UI 주입 프레임워크 (구 tfm2_ui_inject) — 2026-06-29 합병.
//    게임 에셋게터(FUN @LOADER_RVA)를 트램펄린 후킹 → main 템플릿 로드 시
//    mods/*/ui_inject.txt 매니페스트의 .ui 조각을 지정 컨테이너에 가산주입.
//    scrim 이 자체 설치하므로 별도 tfm2_ui_inject 모드 불필요(제거).
//    - install(): scrim init/on_init 에서 호출(가능한 한 일찍 — main 이 on_init 전 로드돼도 잡게).
//    - tick(ui): post_update 맨 위(씬 무관)에서 호출 — 재시도 주입 + 모달 배경차단.
//    RVA 는 구 ui_inject 와 동일(현행 0.4.14 검증값). 패치 시 함께 마이그.
// ===========================================================================
mod uinj {
    use mod_api::{Node, GameUI};
    use std::fs;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};

    const LOG: &str = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_scrim\inject_log.txt";
    const LOADER_RVA: usize = 0x51cd40;  // 0.5.0_3(구0.5.0_2=0x4d8fb0, MOVED→string-xref "asset/base/ui/layout/main" 확정 count17). 에셋 게터
    const PARSER_RVA: usize = 0x2499f30; // 0.5.0_3(구0.5.0_2=0x24960d0, UNIQUE). .ui 텍스트 → NodeTemplate 파서
    const ALLOC_RVA: usize = 0x25ab3d0;  // 0.5.0_3(구0.5.0_2=0x25a7b80, UNIQUE). 게임 alloc(size, align) -> ptr
    #[allow(dead_code)]
    const DEALLOC_RVA: usize = 0x25ab430;// 0.5.0_3(구0.5.0_2=0x25a7be0, UNIQUE). 게임 dealloc — 미사용(leak 설계: startup 경합 UAF 회피)
    type AllocFn = extern "win64" fn(usize, usize) -> usize;
    const TARGET: &[u8] = b"asset/base/ui/layout/main";
    const TARGET2: &[u8] = b"asset/base/ui/layout/strategy";   // ★개인/팀 전술화면(#row0~#row4 등) — 소비자 조각용
    const MODS_DIR: &str = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods";
    static BASE: AtomicUsize = AtomicUsize::new(0);
    static MAIN_TMPL: AtomicUsize = AtomicUsize::new(0);
    static LAST_INJECTED: AtomicUsize = AtomicUsize::new(0);
    static INJECT_ATTEMPTS: AtomicUsize = AtomicUsize::new(0);
    static STRAT_TMPL: AtomicUsize = AtomicUsize::new(0);       // ★strategy 템플릿(재로드마다 갱신)
    static LAST_INJ_STRAT: AtomicUsize = AtomicUsize::new(0);
    static INJ_ATT_STRAT: AtomicUsize = AtomicUsize::new(0);
    static DETOUR_HITS: AtomicUsize = AtomicUsize::new(0);
    static LEN25_HITS: AtomicUsize = AtomicUsize::new(0);
    static MAIN_HITS: AtomicUsize = AtomicUsize::new(0);
    static DIAG_FRAME: AtomicUsize = AtomicUsize::new(0);
    static MODAL_IDS: Mutex<Vec<String>> = Mutex::new(Vec::new());
    const BG_BLOCK: &str = "top"; // 모달 열렸을 때 입력/호버 차단할 배경 컨테이너
    type ParserFn = extern "win64" fn(*mut u8, *const u8, usize);

    fn nfind<'a>(n: &'a Node, id: &str) -> Option<&'a Node> {
        if n.id == id { return Some(n); }
        for c in n.child.iter() { if let Some(f) = nfind(c, id) { return Some(f); } }
        None
    }
    fn nfind_mut<'a>(n: &'a mut Node, id: &str) -> Option<&'a mut Node> {
        if n.id == id { return Some(n); }
        for c in n.child.iter_mut() { if let Some(f) = nfind_mut(c, id) { return Some(f); } }
        None
    }
    fn root_id_of(text: &[u8]) -> Option<String> {
        let s = std::str::from_utf8(text).ok()?;
        for line in s.lines() {
            let t = line.trim();
            if t.is_empty() || t.starts_with('#') || t.starts_with("//") { continue; }
            let id: String = t.chars().take_while(|&c| c != ':' && c != ' ' && c != '\t' && c != '{').collect();
            if !id.is_empty() { return Some(id); }
        }
        None
    }

    const LOG_ENABLED: bool = false; // 배포=false
    fn logln(s: &str) {
        if !LOG_ENABLED { return; }
        use std::io::Write;
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(LOG) { let _ = f.write_all(s.as_bytes()); let _ = f.write_all(b"\n"); }
    }

    type BOOL = i32;
    #[link(name = "kernel32")]
    extern "system" {
        fn GetModuleHandleW(name: *const u16) -> isize;
        fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
        fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old_protect: *mut u32) -> BOOL;
        fn FlushInstructionCache(proc: usize, addr: usize, size: usize) -> BOOL;
        fn GetCurrentProcess() -> usize;
        fn GetCurrentThreadId() -> u32;
        fn GetModuleFileNameW(module: usize, buf: *mut u16, size: u32) -> u32;
    }
    const MEM_COMMIT_RESERVE: u32 = 0x1000 | 0x2000;
    const PAGE_EXECUTE_READWRITE: u32 = 0x40;

    static TRAMP: AtomicUsize = AtomicUsize::new(0);
    static INSTALLED: AtomicBool = AtomicBool::new(false);
    static LOGGED: AtomicBool = AtomicBool::new(false);
    static MAIN_TID: AtomicU32 = AtomicU32::new(0);        // 메인(UI)스레드 id — 캡처 후 detour surgery 는 메인서만
    static INJECTING: AtomicBool = AtomicBool::new(false);  // do_inject 재진입/동시실행 가드
    static OFFMAIN_LOGGED: AtomicBool = AtomicBool::new(false); // ★진단(1회): 비-메인스레드 main-템플릿 로드 감지(가설 확증)

    type LoaderFn = extern "win64" fn(usize, *const u8, usize) -> usize;

    extern "win64" fn detour(am: usize, path: *const u8, len: usize) -> usize {
        let tramp_addr = TRAMP.load(Ordering::Relaxed);
        if tramp_addr == 0 { return 0; }
        let tramp: LoaderFn = unsafe { core::mem::transmute(tramp_addr) };
        let r = tramp(am, path, len); // 원본 호출 → 템플릿 ptr
        DETOUR_HITS.fetch_add(1, Ordering::Relaxed);
        if !path.is_null() && len == TARGET.len() {
            LEN25_HITS.fetch_add(1, Ordering::Relaxed);
            let s = unsafe { core::slice::from_raw_parts(path, len) };
            if s == TARGET && r > 0x10000 {
                MAIN_HITS.fetch_add(1, Ordering::Relaxed);
                MAIN_TMPL.store(r, Ordering::Relaxed); // reload→새 ptr 감지
                // ★ 템플릿 surgery 는 메인(UI)스레드에서만 — 날짜넘김 병렬 sim 워커가 게터 호출 시
                //   do_inject 가 그 스레드서 돌아 메인 인스턴스화와 레이스(ptr/cap/len 인터리브 손상)→간헐 크래시 방지.
                //   비-메인이면 주입 스킵(MAIN_TMPL 은 갱신됨)→다음 tick(메인)서 주입. MAIN_TID==0=startup 단일스레드라 허용.
                let mt = MAIN_TID.load(Ordering::Relaxed);
                let on_main = mt == 0 || unsafe { GetCurrentThreadId() } == mt;
                if !on_main && !OFFMAIN_LOGGED.swap(true, Ordering::Relaxed) {
                    let _ = fs::write(
                        format!(r"{}\tfm2_scrim\scrim_uinj_offmain.txt", game_mods_dir()),
                        format!("off-main main-template load 감지: tid={} main_tid={} → surgery 스킵(tick 위임)\n", unsafe { GetCurrentThreadId() }, mt));
                }
                // 첫화면 표시: 인스턴스화 "전"(여기) 주입. catch_unwind=게임 콜스택 unwind UB 차단.
                if on_main && r != LAST_INJECTED.load(Ordering::Relaxed) {
                    let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { do_inject(r) })).unwrap_or(false);
                    if ok { LAST_INJECTED.store(r, Ordering::Relaxed); INJECT_ATTEMPTS.store(0, Ordering::Relaxed); }
                }
                if !LOGGED.swap(true, Ordering::Relaxed) { logln(&format!("MAIN loaded: template={:#x}", r)); }
            }
        }
        // ★strategy 템플릿(전술화면) 캡처+주입 — main 과 동일 로직(다른 화면이라 별도 슬롯).
        if !path.is_null() && len == TARGET2.len() && r > 0x10000 {
            let s = unsafe { core::slice::from_raw_parts(path, len) };
            if s == TARGET2 {
                STRAT_TMPL.store(r, Ordering::Relaxed);
                let mt = MAIN_TID.load(Ordering::Relaxed);
                let on_main = mt == 0 || unsafe { GetCurrentThreadId() } == mt;
                if on_main && r != LAST_INJ_STRAT.load(Ordering::Relaxed) {
                    let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { do_inject(r) })).unwrap_or(false);
                    if ok { LAST_INJ_STRAT.store(r, Ordering::Relaxed); INJ_ATT_STRAT.store(0, Ordering::Relaxed); }
                }
            }
        }
        r
    }

    pub unsafe fn install() -> Result<(), &'static str> {
        if INSTALLED.swap(true, Ordering::Relaxed) { return Err("already"); }
        let base = GetModuleHandleW(core::ptr::null()) as usize;
        if base == 0 { return Err("module base 0"); }
        BASE.store(base, Ordering::Relaxed);
        let fn_addr = base + LOADER_RVA;
        let expect: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
        for i in 0..12 { if *((fn_addr + i) as *const u8) != expect[i] { logln(&format!("prologue mismatch @+{}: {:#x}", i, *((fn_addr + i) as *const u8))); return Err("prologue mismatch"); } }
        let stub = VirtualAlloc(0, 64, MEM_COMMIT_RESERVE, PAGE_EXECUTE_READWRITE);
        if stub == 0 { return Err("VirtualAlloc"); }
        let mut s: Vec<u8> = Vec::new();
        s.extend_from_slice(&expect);                        // 훔친 8 PUSH (12B)
        s.extend_from_slice(&[0x48, 0xb8]);                  // mov rax,
        s.extend_from_slice(&(fn_addr + 0xc).to_le_bytes()); //   fn+0xc
        s.extend_from_slice(&[0xff, 0xe0]);                  // jmp rax
        core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
        TRAMP.store(stub, Ordering::Relaxed);
        let d = detour as usize;
        let mut patch = [0u8; 12];
        patch[0] = 0x48; patch[1] = 0xb8;
        patch[2..10].copy_from_slice(&d.to_le_bytes());
        patch[10] = 0xff; patch[11] = 0xe0;
        let mut old: u32 = 0;
        if VirtualProtect(fn_addr, 12, PAGE_EXECUTE_READWRITE, &mut old) == 0 { return Err("VirtualProtect"); }
        core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
        VirtualProtect(fn_addr, 12, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
        logln(&format!("hook installed: fn={:#x} stub={:#x} detour={:#x}", fn_addr, stub, d));
        Ok(())
    }

    const NT_SIZE: usize = 0x90; // NodeTemplate 크기

    // 템플릿 트리에서 id 로 노드 찾기. id: ptr@+0x08, len@+0x10. child: ptr@+0x50, count@+0x58.
    unsafe fn find_tmpl(node: usize, target: &[u8], depth: usize) -> usize {
        if node <= 0x10000 || depth > 12 { return 0; }
        let idptr = *((node + 0x08) as *const usize);
        let idlen = *((node + 0x10) as *const usize);
        if idlen == target.len() && idptr > 0x10000 {
            if core::slice::from_raw_parts(idptr as *const u8, idlen) == target { return node; }
        }
        let cptr = *((node + 0x50) as *const usize);
        let clen = *((node + 0x58) as *const usize);
        if cptr > 0x10000 && clen < 1000 {
            for i in 0..clen {
                let found = find_tmpl(cptr + i * NT_SIZE, target, depth + 1);
                if found != 0 { return found; }
            }
        }
        0
    }

    fn module_loaded(dll_name: &str) -> bool {
        let w: Vec<u16> = dll_name.encode_utf16().chain(core::iter::once(0)).collect();
        unsafe { GetModuleHandleW(w.as_ptr()) != 0 }
    }
    // ★ mods 폴더를 게임 exe 경로에서 동적 도출. 하드코딩 절대경로(MODS_DIR)는 다른 PC(다른 드라이브/
    //   설치위치)선 read_dir 실패→조각수집 0→사이드바 버튼 안 뜸. GetModuleFileNameW(NULL)=게임 exe(항상 유효).
    fn game_mods_dir() -> String {
        let mut buf = [0u16; 520];
        let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), buf.len() as u32) } as usize;
        if n > 0 && n < buf.len() {
            let exe = String::from_utf16_lossy(&buf[..n]);
            if let Some(i) = exe.rfind(|c| c == '\\' || c == '/') {
                return format!(r"{}\mods", &exe[..i]);
            }
        }
        MODS_DIR.to_string() // 폴백(우리 머신 기본경로)
    }
    // 각 줄: "<ui상대경로> <타깃id> <위치> [modal]" (# 주석). 위치 = 숫자|after:<id>|before:<id>|end.
    fn collect_fragments() -> Vec<(String, String, String, bool)> {
        let mut out = Vec::new();
        let rd = match fs::read_dir(game_mods_dir()) { Ok(r) => r, Err(_) => return out };
        for ent in rd.flatten() {
            let dir = ent.path();
            let txt = match fs::read_to_string(dir.join("ui_inject.txt")) { Ok(t) => t, Err(_) => continue };
            let mod_id = dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
            // 그 모드 dll 이 실제 로드됐을 때만 주입 (inert 버튼 방지). 단 scrim 자신은 항상 OK.
            if mod_id != "tfm2_scrim" && !module_loaded(&format!("{}.dll", mod_id)) {
                logln(&format!("  skip '{}' — dll 미로드(비활성)", mod_id));
                continue;
            }
            for line in txt.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') { continue; }
                let p: Vec<&str> = line.split_whitespace().collect();
                if p.len() >= 2 {
                    let ui = dir.join(p[0]).to_string_lossy().into_owned();
                    let is_modal = p.iter().skip(2).any(|t| *t == "modal");
                    let pos = match p.get(2) { Some(&t) if t != "modal" => t.to_string(), _ => "end".to_string() };
                    out.push((ui, p[1].to_string(), pos, is_modal));
                }
            }
        }
        out
    }

    unsafe fn child_index(container: usize, id: &[u8]) -> Option<usize> {
        let cptr = *((container + 0x50) as *const usize);
        let clen = *((container + 0x58) as *const usize);
        if cptr <= 0x10000 || clen > 2000 { return None; }
        for i in 0..clen {
            let c = cptr + i * NT_SIZE;
            let idptr = *((c + 0x08) as *const usize);
            let idlen = *((c + 0x10) as *const usize);
            if idlen == id.len() && idptr > 0x10000 && core::slice::from_raw_parts(idptr as *const u8, idlen) == id {
                return Some(i);
            }
        }
        None
    }

    unsafe fn do_inject(r: usize) -> bool {
        // 동시/재진입 가드: 한 번에 한 스레드만 템플릿 수술 (ptr/cap/len 인터리브 손상 방지).
        if INJECTING.swap(true, Ordering::Acquire) { return false; }
        struct Guard;
        impl Drop for Guard { fn drop(&mut self) { INJECTING.store(false, Ordering::Release); } }
        let _g = Guard;
        let base = BASE.load(Ordering::Relaxed);
        if base == 0 || r <= 0x10000 { return false; }
        MODAL_IDS.lock().unwrap_or_else(|e| e.into_inner()).clear();
        let frags = collect_fragments();
        let mut all_ok = true;
        for (ui_path, target_id, pos, is_modal) in &frags {
            if !inject_one(r, base, ui_path, target_id, pos, *is_modal) { all_ok = false; }
        }
        if all_ok { logln(&format!("--- inject 완료: {} 조각, template {:#x} ---", frags.len(), r)); }
        all_ok
    }

    // 반환: true=완료(주입됨/이미있음/영구실패) / false=타깃 컨테이너 아직 없음(다음 프레임 재시도).
    unsafe fn inject_one(r: usize, base: usize, ui_path: &str, target_id: &str, pos: &str, is_modal: bool) -> bool {
        let fname = ui_path.rsplit(['\\', '/']).next().unwrap_or(ui_path).to_string();
        let text = match fs::read(ui_path) { Ok(t) => t, Err(e) => { logln(&format!("  '{}' read err: {}", fname, e)); return true; } };
        if text.is_empty() { return true; }
        if let Some(rid) = root_id_of(&text) { if find_tmpl(r, rid.as_bytes(), 0) != 0 { return true; } }
        let parser: ParserFn = core::mem::transmute(base + PARSER_RVA);
        let mut out = [0u8; 0x400];
        parser(out.as_mut_ptr(), text.as_ptr(), text.len());
        let f = out.as_ptr().add(0x10);
        if *(f as *const usize) == usize::MAX { logln(&format!("  '{}' parse ERROR", fname)); return true; }
        if is_modal {
            if let Some(rid) = root_id_of(&text) {
                let mut ids = MODAL_IDS.lock().unwrap_or_else(|e| e.into_inner());
                if !ids.contains(&rid) { logln(&format!("  modal watch: '{}'", rid)); ids.push(rid); }
            }
        }
        let target = if target_id == "root" { r } else {
            let t = find_tmpl(r, target_id.as_bytes(), 0);
            if t != 0 { t } else { return false; }
        };
        let cap_p = (target + 0x48) as *mut usize;
        let ptr_p = (target + 0x50) as *mut usize;
        let len_p = (target + 0x58) as *mut usize;
        let old_ptr = *ptr_p; let old_cap = *cap_p; let len = *len_p;
        if len > 2000 { logln("  len 비정상, 중단"); return true; }
        let idx = if let Some(a) = pos.strip_prefix("after:") {
            child_index(target, a.as_bytes()).map(|i| i + 1).unwrap_or(len)
        } else if let Some(b) = pos.strip_prefix("before:") {
            child_index(target, b.as_bytes()).unwrap_or(len)
        } else if pos == "end" {
            len
        } else {
            pos.parse::<usize>().unwrap_or(len)
        }.min(len);
        let galloc: AllocFn = core::mem::transmute(base + ALLOC_RVA);
        let new_n = len + 1;
        let new_ptr = galloc(new_n * NT_SIZE, 8);
        if new_ptr == 0 { logln("  game alloc 실패"); return true; }
        if old_ptr != 0 && idx != 0 {
            core::ptr::copy_nonoverlapping(old_ptr as *const u8, new_ptr as *mut u8, idx * NT_SIZE);
        }
        core::ptr::copy_nonoverlapping(f, (new_ptr + idx * NT_SIZE) as *mut u8, NT_SIZE);
        if old_ptr != 0 && idx < len {
            core::ptr::copy_nonoverlapping((old_ptr + idx * NT_SIZE) as *const u8, (new_ptr + (idx + 1) * NT_SIZE) as *mut u8, (len - idx) * NT_SIZE);
        }
        // ptr→cap→len 순서. 옛 배열은 free 안 함(startup 경합 UAF 회피, 누수 무시).
        *ptr_p = new_ptr; *cap_p = new_n; *len_p = new_n;
        let _ = (old_ptr, old_cap);
        logln(&format!("  OK '{}' → '{}' idx{} (child {}→{})", fname, target_id, idx, len, new_n));
        true
    }

    // post_update 등가: 재시도 주입(타깃 미준비 대비) + 모달 배경차단. 씬 무관(메뉴 포함) 매프레임.
    pub fn tick(u: &mut GameUI) {
        // 메인(UI)스레드 id 캡처(최초 1회) — detour 가 비-메인 surgery 를 차단하는 기준.
        if MAIN_TID.load(Ordering::Relaxed) == 0 { MAIN_TID.store(unsafe { GetCurrentThreadId() }, Ordering::Relaxed); }
        let _ = DIAG_FRAME.fetch_add(1, Ordering::Relaxed);
        let r = MAIN_TMPL.load(Ordering::Relaxed);
        if r > 0x10000 && r != LAST_INJECTED.load(Ordering::Relaxed) {
            let n = INJECT_ATTEMPTS.fetch_add(1, Ordering::Relaxed);
            if unsafe { do_inject(r) } || n > 120 {
                LAST_INJECTED.store(r, Ordering::Relaxed);
                INJECT_ATTEMPTS.store(0, Ordering::Relaxed);
            }
        }
        // ★strategy 템플릿도 재시도 주입(전술화면 진입/재로드 대응, 메인스레드)
        let rs = STRAT_TMPL.load(Ordering::Relaxed);
        if rs > 0x10000 && rs != LAST_INJ_STRAT.load(Ordering::Relaxed) {
            let n = INJ_ATT_STRAT.fetch_add(1, Ordering::Relaxed);
            if unsafe { do_inject(rs) } || n > 120 {
                LAST_INJ_STRAT.store(rs, Ordering::Relaxed);
                INJ_ATT_STRAT.store(0, Ordering::Relaxed);
            }
        }
        let ids = MODAL_IDS.lock().unwrap_or_else(|e| e.into_inner()).clone();
        if !ids.is_empty() {
            let any = ids.iter().any(|id| nfind(&u.root, id).map(|n| n.visible).unwrap_or(false));
            if let Some(top) = nfind_mut(&mut u.root, BG_BLOCK) { top.disabled = any; }
        }
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    // ★ 흡수된 ui_inject 프레임워크 훅을 가능한 한 일찍 설치(등록 시점).
    match unsafe { uinj::install() } {
        Ok(()) => append_log("scrim_apply.txt", "[init] uinj install ok"),
        Err(e) => append_log("scrim_apply.txt", &format!("[init] uinj install: {}", e)),
    }
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