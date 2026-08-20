// ═══════════════════════════════════════════════════════════════════════════
// tfm2_flow_capture v0.1 — 경기 흐름 캡처 (2026-08-10 · 0.5.5 마이그 2026-08-13)
// ★0.5.5 오프셋/RVA 재핀 = version-migrator 2026-08-13(각 상수 주석에 근거). 구조체 4대역 §6 = MIGRATION §7.5.
// ⚠서버측 RM_OFF/SGR_STRIDE만 미확정(§6 밖·가드로 크래시無, 세트번호 표시 한정).
//
// 목적: 리플레이(다시보기)/라이브/조합테스트의 sim 전 구간을 30틱(~1초) 간격으로
//   샘플링해 파일로 남긴다 → 오프라인 분석기가 골드 곡선·킬 타임라인·스노우볼을
//   재구성(경기 "흐름" 피드백의 데이터 원천).
//
// 훅: run_tick 0x13b3150 (0.5.4, 프롤로그 12B push8 — comptest_unlock 프로브가
//   크래시 0으로 실증한 함수). ⚠comptest_unlock이 같은 함수를 후킹할 수 있어
//   **체인 후킹**(진입부가 이미 movabs+jmp면 그 스텁으로 체인) + 늦은 설치(120프레임)
//   + 1회 설치 확정. 매프레임 재체인 금지(CLAUDE.md §3).
//
// 핫패스 규율(sim 스레드, 매치당 3만+회): alloc/lock/format!/파일IO 절대 금지 —
//   원시 read + 원자 store만. readable()(VirtualQuery) 가드. 파일 flush는 UI 스레드
//   (post_update)에서만.
//
// 내 경기 식별: **시드 키 슬롯**(MEM\DONE.md — 순번/최초등장 방식 금지). 배경 리그
//   sim도 상시 돌므로 전부 슬롯별로 캡처하고, 오프라인에서 세이브의
//   MatchReplayData.seed 와 조인해 "유저가 본 경기"를 고른다.
//   완주 판정 = 틱 정지(스코어 아님 — DONE.md 규칙).
//
// 오프셋 근거(전부 0.5.4 확정): REPORT\tfm2_comptest_unlock\RE\
//   2026-08-08_실시간_킬스코어_판정.md — ctrl+0x1dc0=game, game+0xeb28=seed,
//   +0xeb30=tick, +0xeb38/+0xeb40=scores, +0x840/+0x848=엔티티 슬라이스(stride 0x8c0),
//   MIGRATION §7.4 — athlete_id +0x800 / team +0x810 / gold +0x878.
// ═══════════════════════════════════════════════════════════════════════════

use mod_api::*;
use std::fs;
use std::io::Write as _;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "tfm2_flow_capture";
const LOG_ENABLED: bool = true; // 검증 단계. 릴리스 시 false (flow 파일 출력과는 별개 경로)

#[path = "../../ui_kit/ui_kit.rs"]
mod ui_kit; // 런타임 Node 조작(라벨·visible·클릭) — 공용 모듈 import(CLAUDE.md §1)
mod ui_replay; // 다시보기 편집 팝업 UI 주입 (레이아웃 로더 체인훅)

// ── win32 ──────────────────────────────────────────────────────────────────
type HMODULE = isize;
type DWORD = u32;
#[repr(C)]
#[derive(Default)]
struct MemBasicInfo {
    base: usize,
    alloc_base: usize,
    alloc_protect: u32,
    _pad0: u32,
    region_size: usize,
    state: u32,
    protect: u32,
    mtype: u32,
    _pad1: u32,
}
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleHandleExW(flags: DWORD, addr: *const u16, out: *mut HMODULE) -> i32;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, n: DWORD) -> DWORD;
    fn VirtualQuery(addr: *const core::ffi::c_void, mbi: *mut MemBasicInfo, len: usize) -> usize;
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, protect: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(h: usize, addr: usize, size: usize) -> i32;
    fn GetCurrentProcess() -> usize;
    fn GetCurrentThreadId() -> u32;
}

#[inline]
unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || addr >= 1usize << 48 || len == 0 {
        return false;
    }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 {
        return false;
    }
    const COMMIT: u32 = 0x1000;
    const RD: u32 = 0x02 | 0x04 | 0x20 | 0x40;
    const GUARD: u32 = 0x01 | 0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 {
        return false;
    }
    addr.wrapping_add(len) <= mbi.base.wrapping_add(mbi.region_size)
}

fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}

fn mod_dir() -> Option<PathBuf> {
    unsafe {
        let mut h: HMODULE = 0;
        if GetModuleHandleExW(0x4 | 0x2, mod_dir as *const () as *const u16, &mut h) == 0 || h == 0 {
            return None;
        }
        let mut b = [0u16; 4096];
        let n = GetModuleFileNameW(h, b.as_mut_ptr(), b.len() as DWORD);
        if n == 0 {
            return None;
        }
        let mut p = PathBuf::from(String::from_utf16_lossy(&b[..n as usize]));
        p.pop();
        Some(p)
    }
}

fn log(s: &str) {
    if !LOG_ENABLED {
        return;
    }
    if let Some(mut p) = mod_dir() {
        p.push("flow_capture_log.txt");
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) {
            let _ = write!(f, "[{}ms] {}\n", now_ms(), s);
        }
    }
}

// ── 0.5.5 오프셋 (2026-08-13 마이그 · 근거 = 파일 상단 헤더 + 아래 각 주석) ──────
//   ⚠구조체 4대역 비균일 시프트 정본 = MIGRATION §7.5 §6. 이 모드 재핀 근거는 각 상수 주석.
//   (0.5.4 값은 취소선으로 정정형 보존.)
const RUN_TICK_RVA: usize = 0x14db7e0; // 0.5.6(구0.5.5=0x14aa160). exe2exe skel UNIQUE·BYTE=SAME·size 5417 동일·프롤로그12 동일. // 0.5.5(구0.5.4=0x13b3150). disp-masked 구조skel UNIQUE + comptest ORACLE 교차 + 프롤로그14/size 0x1529 동일
const HOOK_PROLOGUE12: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53]; // 불변(run_tick·ctor 첫12B 동일)
const HOOK_PROLOGUE12_ALT: [u8; 12] = [0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53];
const G_OFF: usize = 0x1dc0; // ctrl(rdx) → game — 불변(§6 Game +0x1dc0/+0x1dc8, run_tick 정렬 확인)
// provider 대역 +0x168 (§6 균일 · SEED 직접확정 · twin(0x13edef0) 정렬 재확인)
const G_SEED: usize = 0xec90; // 0.5.5(구0xeb28, +0x168)
const G_TICK: usize = 0xec98; // 0.5.5(구0xeb30, +0x168)
const G_SCORE0: usize = 0xeca0; // 0.5.5(구0xeb38, +0x168)
const G_SCORE1: usize = 0xeca8; // 0.5.5(구0xeb40, +0x168)
const ENT_PTR: usize = 0x858; // 엔티티(선수) dense 슬라이스 — 0.5.5(구0x840, +0x18 §6 World W_PLAYER_DENSE)
const ENT_LEN: usize = 0x860; // 0.5.5(구0x848, +0x18)
const ENT_STRIDE: usize = 0x9e0; // 0.5.5(구0x8c0, §6 athlete stride · item_tactics 확정)
// athlete 레코드 필드 = 대역분할(⚠비균일): [0x408,0x6b0) 대역 +0x60 / [0x800,…) 대역 +0x120 (§6)
const A_ID: usize = 0x920; // 0.5.5(구0x800, +0x120 §6·item_tactics·twin)
const A_TEAM: usize = 0x930; // 0.5.5(구0x810, +0x120 · twin·DEAL_WRITE 정렬 확정)
const A_GOLD: usize = 0x998; // 0.5.5(구0x878, +0x120 athlete 고대역 · 0x810/0x8a0 확정 사이 내삽)
const A_DEAL: usize = 0x610; // 누적 딜량(i64) — 0.5.5(구0x5b0, +0x60 · DEAL_WRITE fn 0x13d1670→0x102e160 정렬 확정)
const A_TANK: usize = 0x618; // 누적 탱킹(i64) — 0.5.5(구0x5b8, +0x60 · TANK_WRITE fn 0x13ce8e0→0x102b1b0 정렬 확정)
// athlete champion_name String — R↔E 브릿지. 0.5.5 = cap+0x468/ptr+0x470/len+0x478
//   (구0.5.4 +0x408/+0x410/+0x418, +0x60 mid대역 · item_tactics 0.5.5 인게임검증본 확정).
const A_NAME_PTR: usize = 0x470; // 0.5.5(구0x410, +0x60)
const A_NAME_LEN: usize = 0x478; // 0.5.5(구0x418, +0x60)
const A_POS: usize = 0x9c0; // position/role u32 0~4 — 0.5.5(구0x8a0, +0x120 §6·item_tactics·twin)
// 팀 전술: World(game)+0xb3b0 = [Strategy;2], 팀당 24B(stride 0x18). 12필드 disc(u8).
//   ★0.5.5 = 0xb3b0(구0.5.4 0xb248, +0x168). getter/setter fn(0x13e5760/0x13fefa0 → 0x14ef2f0/0x150be90)
//   `[base+idx*0x18+0xb248]`→`+0xb3b0` 정렬 직접확정 · stride 0x18 불변(lea rdx*3). ⚠kill logs(0xb2xx)는 +0x18인데
//   Strategy(0xb248)만 +0x168 = World 저대역/고대역 경계가 (0xb210,0xb248) 사이(비균일 함정).
const STRAT_OFF: usize = 0xb3b0;
const STRAT_STRIDE: usize = 0x18;

// ── 밴픽 실시간 탐지: BanpickScene scene_step 훅 ─────────────────────────────
//   RE 2026-08-11(ghidra-re, exe 디스어셈 실증): scene_step(&BanpickScene)->u8
//   RVA 0x1dad900(0.5.4 재핀·클라 콜러 23=23 완전일치), rcx=&BanpickScene, 리프함수.
//   4 Vec<String>(챔프 내부이름, {cap@+0x138,ptr@+0x140,len@+0x148} 계열): t1밴 ptr+0x140/len+0x148,
//   t2밴 +0x158/+0x160, t1픽 +0x170/+0x178, t2픽 +0x188/+0x190. 각 String{cap@0,ptr@8,len@0x10} stride 0x18.
//   game_rule u8@+0xce(0=2v2..3=5v5, 팀당 목표픽=rule+2). ban_count u64@+0x3c0. blue측 팀id@+0x3d0.
//   ⚠pass-through 캡처 훅이라 14B 재배치 필요(12B는 2번째 mov 중간 절단). 두 명령 모두
//    mov reg,[rcx+disp32](rip-rel 없음)이라 그대로 재배치 가능. 패치=12B(movabs+jmp)가 14B 안에 듦.
const SCENE_STEP_RVA: usize = 0x24d1dc0; // 0.5.6(구0.5.5=0x196c2c0). 마스크시그 UNIQUE(pdata 없는 리프)·movzx r9d,[rcx+0xce]@0x24d1de2 시그 재확인·프롤로그14 동일. // 0.5.5(구0.5.4=0x1dad900). banpick_order PHASE_SCENE 마이그 동일 함수 + movzx[rcx+0xce]@0x196c2e2 시그 확인
// BanpickScene 구조 = 0.5.5 전건 불변(banpick_order 0.5.5 확정: PROLOGUE_SCENE·O_SC 오프셋·+0x3d0 유지)
const BP_PROLOGUE14: [u8; 14] = [0x48, 0x8b, 0x81, 0x60, 0x01, 0x00, 0x00, 0x48, 0x8b, 0x91, 0x78, 0x01, 0x00, 0x00]; // 불변
const BP_BAN1: usize = 0x140; // t1 밴 Vec ptr(len=+8) — 불변
const BP_BAN2: usize = 0x158;
const BP_PICK1: usize = 0x170;
const BP_PICK2: usize = 0x188;
const BP_RULE: usize = 0xce; // u8 game_rule
const BP_BLUE_TID: usize = 0x3d0;
const BP_STR_STRIDE: usize = 0x18;

// ── 게임 전술 추천 훅: recommend_strategy_inner ────────────────────────────
//   RE 2026-08-11(ghidra-re): inner=0x1455980(결정본체). 프롤로그 12B=8push+sub rsp,0x248(rip-rel·chkstk 없음).
//   Win64: rcx=Strategy* sret, rdx=ctx, r8=my_comp, r9=my_len, [rsp+0x28]=opp_comp,[+0x30]=opp_len,[+0x38]=flag,[+0x40]=diff,[+0x48]=override.
//   sret 24B = 캐논: bld u32@0, mor u32@4, mor_pos u32@8, bat@12, twr@13, def@14, fin@15, wav@16, foc@17, jng@18, srp@19, srt@20, end@21.
//   캡처 = 트램폴린-후-덮기(원본 실행 후 sret 읽기). 배경 sim(rayon) 필터 = 메인 스레드(위임버튼)만.
// ★훅 대상 = 공개진입 0x14a0240 (라이브 위임 전용, 배경 sim은 콜그래프상 여기 못 닿음 → 필터 불요).
//   RE 2026-08-11: 배경=0x20d5bf0이 inner 직접(override=0), 라이브=0x237c030→0x14a0240→inner(override≠0).
//   프롤로그 12B=8push(41 57..53)+sub rsp,0x408. sret rcx 24B=캐논 Strategy. 호출당 양팀 2회(최신 채택).
const RECO_INNER_RVA: usize = 0x2ce38f0; // 0.5.6(구0.5.5=0x12ae860). 마스크시그 UNIQUE(pdata 없는 리프)·프롤로그12 동일. // 0.5.5(구0.5.4=0x14a0240). skel UNIQUE(disp 포함, delta -0x1f19e0) + 프롤로그12 동일 = 본문 무변경
const RECO_PROLOGUE12: [u8; 12] = [0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53]; // 불변(NEW 프롤로그 동일 확인)

// 밴픽 피어리스 불가목록 = 씬(X) 고정오프셋 직접 읽기(extract_fearless). contains 훅은 폐기(공용 헬퍼라 분리 불가, RE 2026-08-11).

// 전투 엔티티(좌표·HP) — game-atlas 2026-08-10 회신:
//   배열 = game+0x720(ptr)/+0x728(count), stride 0x6a8 (0.5.0_3 채록 +0x38 시프트 가족 —
//   같은 가족 +0x840/+0x848이 0.5.4 실측 유효 ⟹ 유효 추정·런타임 sanity 게이트로 방어).
//   좌표 = +0x648(x)/+0x650(y) **정수 world 단위**(셀=32000, 30×30 그리드, 0~960,000 —
//   CASE-불변 확정·재조사 금지 등재). 챔피언 판별 = kind(+0x68)==0xd. team=+0x8.
//   HP = +0x658(cur)/+0x610(max). champion_name String cap/ptr/len = +0x248/+0x250/+0x258.
// 전투 엔티티(0.5.5): World 배열 ptr/cnt = +0x18(§6 World), entity 내부 ≥0x5a8 = +0x18, ≤0x258 = 불변(§6).
const CE_PTR: usize = 0x738; // 0.5.5(구0x720, +0x18 §6 W_CHAMP_DENSE)
const CE_CNT: usize = 0x730; // 0.5.5(구0x728, +0x18)
const CE_STRIDE: usize = 0x6c0; // 0.5.5(구0x6a8, §6 CHAMP_STRIDE)
const CE_KIND: usize = 0x68; // 불변(§6 ≤0x258)
const CE_KIND_CHAMP: u32 = 0xd;
// 미니언(라인 병사) — RE 2026-08-11. 팀=+0x8, 레인 선택자=+0x11a(byte)·좌표=+0x660/+0x668(0.5.5).
const CE_MIN_KIND: u32 = 1;
const CE_LANE: usize = 0x11a; // 불변(§6 저대역 ≤0x258 — band 파생, 미직접확인)
const NMINCOL: usize = 8; // 미니언 카운트 = 2팀 × 4버킷(레인 0/1/2/기타). '기타'>0이면 레인 인코딩 재확인.
const CE_TEAM: usize = 0x8; // 불변(entity 0xc94b50 정렬 확인)
const CE_X: usize = 0x660; // 0.5.5(구0x648, +0x18 · entity fn 0xc94b50→0xc97bf0 정렬 확정)
const CE_Y: usize = 0x668; // 0.5.5(구0x650, +0x18 · 동)
const CE_HP: usize = 0x670; // 0.5.5(구0x658, +0x18 · §6 CUR_HP + entity fn 확정)
const CE_HPMAX: usize = 0x628; // 0.5.5(구0x610, +0x18 · §6 EXEC_MAXHP + entity fn 확정)
const CE_LEVEL: usize = 0x5c8; // entity level(qword) — 0.5.5(구0x5b0, +0x18 · entity ≥0x5a8 밴드: 0x5a8→0x5c0·0x610→0x628 브래킷)
const CE_NAME_PTR: usize = 0x250; // 불변(§6 explicit)
const CE_NAME_LEN: usize = 0x258; // 불변(§6 explicit)
const CE_SCAN_MAX: usize = 512; // 엔티티 배열 스캔 상한(후반 미니언 다수 대비 — 챔피언 누락 방지)

// kill_logs — RE 2026-08-10 확정(REPORT\tfm2_flow_capture\RE\2026-08-10_Game흐름필드-*.md):
//   Vec<KillLog> @game+0xb200, ★Vec 레이아웃 = {cap@+0, ptr@+8, len@+0x10}(cap-선두).
//   KillLog(0x30B) = assist Vec<Position>@+0x00 · tick u64@+0x18 · killer_team u64@+0x20 ·
//   killer_position u32@+0x28 · killed_position u32@+0x2c. Position={0 Top,1 Jg,2 Mid,3 Bot,4 Sup}.
//   MobaGame·SingleLaneGame 공통. sim 스레드 자신이 detour 안에서 읽으므로 동시변경 없음.
// ★0.5.5: kill_logs Vec는 World 저대역 = +0x18 (Strategy 0xb248의 +0x168과 다른 밴드!).
//   twin(kill_logs 순회 fn 0x13edef0→0x14f7c60) 정렬 직접확정: ptr 0xb208→0xb220·len 0xb210→0xb228,
//   stride 0x30·KillLog 내부오프셋(tick 0x18/team 0x20/kpos 0x28/…)·assist 전부 불변.
const KL_PTR: usize = 0xb220; // 0.5.5(구0xb208, +0x18)
const KL_LEN: usize = 0xb228; // 0.5.5(구0xb210, +0x18)
const KL_STRIDE: usize = 0x30; // 불변(twin: add r11,-0x30)
const KLE_ASSIST_PTR: usize = 0x08;
const KLE_ASSIST_LEN: usize = 0x10;
const KLE_TICK: usize = 0x18;
const KLE_TEAM: usize = 0x20;
const KLE_KPOS: usize = 0x28;
const KLE_DPOS: usize = 0x2c;

// 오브젝트 처치 로그 — RE 2026-08-10 확정(REPORT\...\RE\2026-08-10_오브젝트로그-*.md, 0.5.4).
//   세 로그 Vec {cap@+0,ptr@+8,len@+0x10} 연속. team u64@+0, tick u64@+8 공통.
//   tower만 stride 0x18(+line u8@+0x10), epic(모르가드)/serpen stride 0x10.
// ★0.5.5: 오브젝트 로그 Vec cap 오프셋은 provider 대역 +0x168 (serpen 0xed88→0xeef0 = §6 KILLS Vec 직접확정,
//   tower/epic 동일밴드 내삽). 원소 stride(0x18/0x10)·내부오프셋(OE_*)은 불변.
const OBJ_LOGS: [(usize, usize, u32); 3] = [
    (0xeec0, 0x18, 0), // tower_destroy_logs, type 0 — 0.5.5(구0xed58, +0x168)
    (0xeed8, 0x10, 1), // epic_logs(모르가드), type 1 — 0.5.5(구0xed70, +0x168)
    (0xeef0, 0x10, 2), // serpen_logs, type 2 — 0.5.5(구0xed88, +0x168 · §6 확정)
];
const OE_TEAM: usize = 0x00;
const OE_TICK: usize = 0x08;
const OE_LINE: usize = 0x10; // tower만
const OMAX: usize = 64;

// ── 캡처 슬롯 (시드 키, lock-free) ──────────────────────────────────────────
// SLOTS 16 = 배경 리그 sim(동시 8+)과 경쟁해도 유저 경기가 슬롯을 초반부터 잡도록.
const SLOTS: usize = 16;
const SMAX: usize = 4096; // 샘플 상한. 30틱(0.5초@60tps)×4096 = ~34분. ★2048=17분서 장기전 잘림(19분경기 실측) → 4096
const NATH: usize = 10;
const KMAX: usize = 256;
const SAMPLE_EVERY: u64 = 30;

// ★유저 팀 선수 aid 필터 + game 포인터 슬롯(2026-08-10): 같은 seed의 여러 세트 game이
//   섞이는 문제 → 슬롯 키를 game_ptr로, 유저 선수(MY_ATH) 든 game만 캡처(배경 리그 제외).
static MY_ATH: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8];
static MY_ATH_N: AtomicU64 = AtomicU64::new(0);
static MY_ATH_SRC: AtomicU64 = AtomicU64::new(0); // 0=미확정 1=SDK자동 2=cfg폴백
static SL_SEED: [AtomicU64; SLOTS] = [const { AtomicU64::new(0) }; SLOTS]; // 이제 game_ptr 키로 사용
static SL_REALSEED: [AtomicU64; SLOTS] = [const { AtomicU64::new(0) }; SLOTS]; // 파일명용 실제 seed
static SL_OVR: [AtomicU32; SLOTS] = [const { AtomicU32::new(0) }; SLOTS]; // 이 슬롯 = 전술 오버라이드 주입된 재생
static INJECT_FIRED: AtomicU64 = AtomicU64::new(0); // 진단: inject_override 발화 수
static SL_N: [AtomicU32; SLOTS] = [const { AtomicU32::new(0) }; SLOTS]; // 샘플 수 (Release)
static SL_LASTTICK: [AtomicU64; SLOTS] = [const { AtomicU64::new(0) }; SLOTS];
static SL_LASTBUCKET: [AtomicU64; SLOTS] = [const { AtomicU64::new(u64::MAX) }; SLOTS];
static SL_ROSTER_OK: [AtomicU32; SLOTS] = [const { AtomicU32::new(0) }; SLOTS];
static SL_AID: [[AtomicU64; NATH]; SLOTS] = [const { [const { AtomicU64::new(0) }; NATH] }; SLOTS];
static SL_TEAM: [[AtomicU32; NATH]; SLOTS] = [const { [const { AtomicU32::new(0) }; NATH] }; SLOTS];
// athlete 챔피언명 16B(2×u64) — R행에 넣어 뷰어가 (team,champ)로 E순과 브릿지(딜/탱 정렬).
static SL_ACHAMP: [[AtomicU64; 2 * NATH]; SLOTS] = [const { [const { AtomicU64::new(0) }; 2 * NATH] }; SLOTS];
// 팀 전술 24B×2팀 = 48바이트(각 byte를 u32로). game+0xb248 [Strategy;2] 1회 채록.
static SL_STRAT_OK: [AtomicU32; SLOTS] = [const { AtomicU32::new(0) }; SLOTS];
static SL_STRAT: [[AtomicU32; 48]; SLOTS] = [const { [const { AtomicU32::new(0) }; 48] }; SLOTS];
// 세트 번호/시리즈 스코어 — 로스터 채록 시 서버가 게시한 값 스냅샷(경기 시작 기준).
static SL_SETNO: [AtomicU32; SLOTS] = [const { AtomicU32::new(0) }; SLOTS];
static SL_SERIES_U: [AtomicU32; SLOTS] = [const { AtomicU32::new(0) }; SLOTS];
static SL_SERIES_O: [AtomicU32; SLOTS] = [const { AtomicU32::new(0) }; SLOTS];
static SL_T0MS: [AtomicU64; SLOTS] = [const { AtomicU64::new(0) }; SLOTS];
// 샘플 본체: tick / score0 / score1 / gold×10
static S_TICK: [[AtomicU32; SMAX]; SLOTS] = [const { [const { AtomicU32::new(0) }; SMAX] }; SLOTS];
static S_SC0: [[AtomicU32; SMAX]; SLOTS] = [const { [const { AtomicU32::new(0) }; SMAX] }; SLOTS];
static S_SC1: [[AtomicU32; SMAX]; SLOTS] = [const { [const { AtomicU32::new(0) }; SMAX] }; SLOTS];
static S_GOLD: [[[AtomicU32; NATH]; SMAX]; SLOTS] =
    [const { [const { [const { AtomicU32::new(0) }; NATH] }; SMAX] }; SLOTS];
// 전투 엔티티(챔피언 10) 인덱스 맵 — 슬롯당 1회 구축
static CE_N: [AtomicU32; SLOTS] = [const { AtomicU32::new(0) }; SLOTS]; // 찾은 챔피언 수
static CE_TEAMS: [[AtomicU32; NATH]; SLOTS] = [const { [const { AtomicU32::new(0) }; NATH] }; SLOTS];
static CE_NAME: [[AtomicU64; 2 * NATH]; SLOTS] = [const { [const { AtomicU64::new(0) }; 2 * NATH] }; SLOTS];
// 샘플 확장: 좌표·HP (전투 엔티티 순서 = CE_IDX 순서)
static S_X: [[[AtomicU32; NATH]; SMAX]; SLOTS] =
    [const { [const { [const { AtomicU32::new(0) }; NATH] }; SMAX] }; SLOTS];
static S_Y: [[[AtomicU32; NATH]; SMAX]; SLOTS] =
    [const { [const { [const { AtomicU32::new(0) }; NATH] }; SMAX] }; SLOTS];
static S_HP: [[[AtomicU32; NATH]; SMAX]; SLOTS] =
    [const { [const { [const { AtomicU32::new(0) }; NATH] }; SMAX] }; SLOTS];
static S_LV: [[[AtomicU32; NATH]; SMAX]; SLOTS] =
    [const { [const { [const { AtomicU32::new(0) }; NATH] }; SMAX] }; SLOTS];
// 누적 딜량(athlete+0x5b0)·탱킹(+0x5b8) — R순(athlete 배열 순서). 한타 구간 diff로 한타 피해 산출.
//   RE 2026-08-11: athlete 레코드(stride 0x8c0)에 i64 단조증가. 값은 u32로 저장(경기 총딜<4B).
static S_DEAL: [[[AtomicU32; NATH]; SMAX]; SLOTS] =
    [const { [const { [const { AtomicU32::new(0) }; NATH] }; SMAX] }; SLOTS];
static S_TANK: [[[AtomicU32; NATH]; SMAX]; SLOTS] =
    [const { [const { [const { AtomicU32::new(0) }; NATH] }; SMAX] }; SLOTS];
// 미니언(kind==1) 팀×레인 카운트 — wav(웨이브 관리) 지표용. 샘플당 2팀×4버킷.
static S_MIN: [[[AtomicU32; NMINCOL]; SMAX]; SLOTS] =
    [const { [const { [const { AtomicU32::new(0) }; NMINCOL] }; SMAX] }; SLOTS];
// 미니언 웨이브 중심점(뷰어 렌더용): 6버킷(2팀×3레인) 좌표 합. flush서 count로 나눠 평균.
static S_MINX: [[[AtomicU32; 6]; SMAX]; SLOTS] =
    [const { [const { [const { AtomicU32::new(0) }; 6] }; SMAX] }; SLOTS];
static S_MINY: [[[AtomicU32; 6]; SMAX]; SLOTS] =
    [const { [const { [const { AtomicU32::new(0) }; 6] }; SMAX] }; SLOTS];
// 정글 캠프 생존 비트마스크(뷰어 몹 죽음/스폰용): bit i = SL_N* 캠프 i에 생존 몹 존재.
static S_JMASK: [[AtomicU32; SMAX]; SLOTS] =
    [const { [const { AtomicU32::new(0) }; SMAX] }; SLOTS];
// 중립 몹(정글 kind4·곰 kind9) 스폰 좌표 1회 채록 — mapgeo 캠프 좌표가 부정확해 실측으로 대체.
//   camps는 스폰 후 어그로 전까지 정지 → 초반 1회 캡처 = 캠프 위치.
const MAXNEUT: usize = 24;
static SL_NEUT_OK: [AtomicU32; SLOTS] = [const { AtomicU32::new(0) }; SLOTS];
static SL_NEUT_N: [AtomicU32; SLOTS] = [const { AtomicU32::new(0) }; SLOTS];
static SL_NKIND: [[AtomicU32; MAXNEUT]; SLOTS] = [const { [const { AtomicU32::new(0) }; MAXNEUT] }; SLOTS];
static SL_NTEAM: [[AtomicU32; MAXNEUT]; SLOTS] = [const { [const { AtomicU32::new(0) }; MAXNEUT] }; SLOTS];
static SL_NX: [[AtomicU32; MAXNEUT]; SLOTS] = [const { [const { AtomicU32::new(0) }; MAXNEUT] }; SLOTS];
static SL_NY: [[AtomicU32; MAXNEUT]; SLOTS] = [const { [const { AtomicU32::new(0) }; MAXNEUT] }; SLOTS];
static SL_NNAME: [[AtomicU64; 2 * MAXNEUT]; SLOTS] = [const { [const { AtomicU64::new(0) }; 2 * MAXNEUT] }; SLOTS];
// 오브젝트 처치 이벤트(tower/epic/serpen 병합) — 로그별 seen으로 증분 캡처
static O_N: [AtomicU32; SLOTS] = [const { AtomicU32::new(0) }; SLOTS];
static O_SEEN: [[AtomicU32; 3]; SLOTS] = [const { [const { AtomicU32::new(0) }; 3] }; SLOTS];
static O_TICK: [[AtomicU32; OMAX]; SLOTS] = [const { [const { AtomicU32::new(0) }; OMAX] }; SLOTS];
static O_TEAM: [[AtomicU32; OMAX]; SLOTS] = [const { [const { AtomicU32::new(0) }; OMAX] }; SLOTS];
static O_TYPE: [[AtomicU32; OMAX]; SLOTS] = [const { [const { AtomicU32::new(0) }; OMAX] }; SLOTS];
static O_LINE: [[AtomicU32; OMAX]; SLOTS] = [const { [const { AtomicU32::new(255) }; OMAX] }; SLOTS];
// kill_logs 원시 사본
static K_N: [AtomicU32; SLOTS] = [const { AtomicU32::new(0) }; SLOTS];
static K_RAW: [[[AtomicU32; 16]; KMAX]; SLOTS] =
    [const { [const { [const { AtomicU32::new(0) }; 16] }; KMAX] }; SLOTS];

// UI 스레드 상태
static SL_IDLE: [AtomicU32; SLOTS] = [const { AtomicU32::new(0) }; SLOTS];
static SL_UILAST: [AtomicU64; SLOTS] = [const { AtomicU64::new(0) }; SLOTS];
static FILES_WRITTEN: AtomicU64 = AtomicU64::new(0);
static DROPPED_FULL: AtomicU64 = AtomicU64::new(0);

// ── 훅 상태 ────────────────────────────────────────────────────────────────
static TRAMP: AtomicUsize = AtomicUsize::new(0);
static INSTALLED: AtomicU32 = AtomicU32::new(0); // 0=미설치 1=성공 2=실패
static INSTALL_MSG: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
// SDK로 해석한 이름(post_update 클라 스레드서 채움, flush서 읽음). athlete_id→선수명 / 유저 팀명.
static PLAYER_NAMES: std::sync::Mutex<std::collections::BTreeMap<u64, String>> =
    std::sync::Mutex::new(std::collections::BTreeMap::new());
static USER_TNAME: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
static OPP_TNAME: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
// ★seed→(blue명,red명) 조인표: dump_replays(db)가 30초마다 갱신, flush가 슬롯 seed로 조회해
//   경기별 정확한 상대팀명을 얻는다(OPP_TNAME 전역 하나로 고정되던 문제 해결). Vec=const init 가능.
//   단 라이브 경기는 seed가 아직 replay에 없어 조인 실패 → AID_TEAM로 폴백.
static SEED_TEAMS: std::sync::Mutex<Vec<(u64, String, String)>> = std::sync::Mutex::new(Vec::new());
// ★aid→팀명 역조회표(현재 로스터): collect_my_team이 db.team_ids()×last_starting로 갱신.
//   라이브 경기(리플레이 미생성)의 상대측 선수 aid로 상대팀명을 경기별 정확하게 해석.
static AID_TEAM: std::sync::Mutex<Vec<(u64, String)>> = std::sync::Mutex::new(Vec::new());
// 세트 번호/시리즈: 서버 확장이 running_matches에서 유저 경기 set_game_results를 읽어 게시.
static PLAYER_TID: AtomicU64 = AtomicU64::new(u64::MAX); // 클라가 게시(유저 팀 id)
static SERVER_STATE: AtomicUsize = AtomicUsize::new(0);
static MATCH_COMPLETED: AtomicU32 = AtomicU32::new(0); // 완료 세트 수(=현재 세트-1)
static MATCH_SERIES_U: AtomicU32 = AtomicU32::new(0);  // 시리즈 유저 승
static MATCH_SERIES_O: AtomicU32 = AtomicU32::new(0);  // 시리즈 상대 승
// 피어리스 불가 챔프(서버 running_matches의 RMI.set_info에서 walk가 추출, write_banpick_json이 읽음).
static FEARLESS_LOCKED: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
// 다시보기 편집기 주입: replay_override.json(시드+전술 24B×2) → 그 시드 재생 시 game+0xb248 덮어쓰기.
static OVERRIDE_ON: AtomicBool = AtomicBool::new(false);
static OVERRIDE_SEED: AtomicU64 = AtomicU64::new(0);
static OVERRIDE_STRAT: [AtomicU64; 6] = [const { AtomicU64::new(0) }; 6]; // blue[0..3]+red[3..6] (각 24B=3u64)
static OVERRIDE_MTIME: AtomicU64 = AtomicU64::new(0);
// 마지막 관전(화면출력) 리플레이의 seed — 관전 클라 game(score 0:0) 감지 시 래치. 편집 패널이 이 세트를 자동 로드.
static LAST_WATCHED_SEED: AtomicU64 = AtomicU64::new(0);
// 패널이 override를 소유하면(시작 눌러 무장) 파일기반 read_override가 덮어쓰지 않게.
static OVERRIDE_FROM_PANEL: AtomicBool = AtomicBool::new(false);
static HITS: AtomicU64 = AtomicU64::new(0);
static FRAMES: AtomicU64 = AtomicU64::new(0);
// 밴픽 실시간: scene_step detour가 매 진입 &BanpickScene(rcx)를 덮어씀(씬 재생성 대응).
//   draft 밖에선 발화 멈춰 stale → BP_HITS 진행 여부로 "draft 중" 게이트.
static BP_SCENE: AtomicUsize = AtomicUsize::new(0);
static BP_HITS: AtomicU64 = AtomicU64::new(0);
static BP_TRAMP: AtomicUsize = AtomicUsize::new(0);
static BP_INSTALLED: AtomicU32 = AtomicU32::new(0);
static BP_LAST_HITS: AtomicU64 = AtomicU64::new(0);
static BP_STALE_WRITTEN: AtomicBool = AtomicBool::new(true);
// 게임 전술 추천: inner detour가 메인 스레드(위임버튼) 호출 시 sret 24B(=3 u64)를 캡처.
static RECO_TRAMP: AtomicUsize = AtomicUsize::new(0);
static RECO_INSTALLED: AtomicU32 = AtomicU32::new(0);
static MAIN_TID: AtomicU32 = AtomicU32::new(0); // post_update(클라 스레드) 첫 프레임에 게시
static RECO_SEQ: AtomicU64 = AtomicU64::new(0);
static RECO_S0: AtomicU64 = AtomicU64::new(0);
static RECO_S1: AtomicU64 = AtomicU64::new(0);
static RECO_S2: AtomicU64 = AtomicU64::new(0);
static RECO_LAST_SEQ: AtomicU64 = AtomicU64::new(0);
// 진단: reco 발화 카운터(전체/메인) + 스레드 ID·comp 링.
static RECO_ANY: AtomicU64 = AtomicU64::new(0);
static RECO_MAIN: AtomicU64 = AtomicU64::new(0);
static RECO_TID_RING: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8];
static RECO_LEN_RING: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8];
static RECO_RING_IDX: AtomicU64 = AtomicU64::new(0);

// ── 핫패스: run_tick detour (sim 스레드 — alloc/lock/format! 금지) ─────────
extern "win64" fn tick_detour(a1: usize, a2: usize, a3: usize, a4: usize, a5: usize, a6: usize) -> usize {
    HITS.fetch_add(1, Ordering::Relaxed);
    unsafe {
        capture(a2);
    }
    let stub = TRAMP.load(Ordering::Relaxed);
    if stub == 0 {
        return 0;
    }
    let f: extern "win64" fn(usize, usize, usize, usize, usize, usize) -> usize =
        unsafe { core::mem::transmute(stub) };
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(a1, a2, a3, a4, a5, a6))).unwrap_or(0)
}

// ── 핫 리프: scene_step detour (밴픽 draft 클라 스레드 — store만, alloc/lock 금지) ──
//   rcx=&BanpickScene를 매 진입 덮어씀. draft마다 씬 재할당 가능 → 항상 최신 포인터 유지.
extern "win64" fn scene_step_detour(scene: usize) -> u8 {
    BP_SCENE.store(scene, Ordering::Relaxed);
    BP_HITS.fetch_add(1, Ordering::Relaxed);
    let stub = BP_TRAMP.load(Ordering::Relaxed);
    if stub == 0 {
        return 0;
    }
    let f: extern "win64" fn(usize) -> u8 = unsafe { core::mem::transmute(stub) };
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(scene))).unwrap_or(0)
}

// ── recommend_strategy_inner detour: 원본 실행 후, 메인 스레드(위임버튼)면 sret 24B 캡처 ──
//   배경 리그 sim(rayon 워커)은 스레드 id 불일치로 캡처 제외(원본은 전 스레드 정상 실행).
extern "win64" fn reco_detour(a1: usize, a2: usize, a3: usize, a4: usize, a5: usize, a6: usize, a7: usize, a8: usize, a9: usize) -> usize {
    let stub = RECO_TRAMP.load(Ordering::Relaxed);
    let ret = if stub != 0 {
        let f: extern "win64" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
            unsafe { core::mem::transmute(stub) };
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(a1, a2, a3, a4, a5, a6, a7, a8, a9))).unwrap_or(0)
    } else {
        0
    };
    let tid = unsafe { GetCurrentThreadId() };
    RECO_ANY.fetch_add(1, Ordering::Relaxed); // 진단: 공개진입은 위임 시에만 발화해야(배경 0)
    let ri = (RECO_RING_IDX.fetch_add(1, Ordering::Relaxed) as usize) % 8;
    RECO_TID_RING[ri].store(tid as u64, Ordering::Relaxed);
    RECO_LEN_RING[ri].store(a4 as u64, Ordering::Relaxed);
    // ★공개진입 0x14a0240 = 라이브 위임 전용(배경 sim 콜그래프 미도달) → 필터 없이 sret 24B 캡처.
    unsafe {
        if readable(a1, 24) {
            RECO_S0.store(core::ptr::read_unaligned(a1 as *const u64), Ordering::Relaxed);
            RECO_S1.store(core::ptr::read_unaligned((a1 + 8) as *const u64), Ordering::Relaxed);
            RECO_S2.store(core::ptr::read_unaligned((a1 + 16) as *const u64), Ordering::Relaxed);
            RECO_SEQ.fetch_add(1, Ordering::Relaxed);
            RECO_MAIN.fetch_add(1, Ordering::Relaxed);
        }
    }
    ret
}


// game의 선수(관리 레코드) 배열에서 MY_ATH(유저 선발 aid) 매칭 수를 센다.
#[inline]
unsafe fn count_my_ath(g: usize, n: usize) -> usize {
    if !readable(g.wrapping_add(ENT_PTR), 16) {
        return 0;
    }
    let ep = core::ptr::read_unaligned((g + ENT_PTR) as *const u64) as usize;
    let el = core::ptr::read_unaligned((g + ENT_LEN) as *const u64) as usize;
    let cnt = el.min(NATH);
    if ep < 0x10000 || cnt == 0 || !readable(ep, cnt * ENT_STRIDE) {
        return 0;
    }
    let mut m = 0usize;
    for i in 0..cnt {
        let aid = core::ptr::read_unaligned((ep + i * ENT_STRIDE + A_ID) as *const u64);
        for k in 0..n {
            if MY_ATH[k].load(Ordering::Relaxed) == aid {
                m += 1;
                break;
            }
        }
    }
    m
}

#[inline]
unsafe fn capture(ctrl: usize) {
    if !readable(ctrl.wrapping_add(G_OFF), 8) {
        return;
    }
    let g = core::ptr::read_unaligned((ctrl + G_OFF) as *const u64) as usize;
    if g < 0x10000 || !readable(g.wrapping_add(G_SCORE1), 8) {
        return;
    }
    let seed = core::ptr::read_unaligned((g + G_SEED) as *const u64);
    if seed == 0 {
        return;
    }
    // ★다시보기 편집기: 이 game의 seed가 override와 같으면 전술 주입(원본 tick 전에 = 이번 틱 반영).
    inject_override(g, seed);
    // ★관전 클라 game 배제(2026-08-10 실측): 서버 sim은 game_tick 단조·score=킬수인데,
    //   관전(화면출력) 클라 game은 game_tick 비단조·score 0:0(킬이 있어도)이라 좌표가 재생 종속.
    //   판별: kill_logs.len>2 인데 score0+score1==0 → 관전 클라 → 캡처 안 함(서버 sim만 잡음).
    if readable(g.wrapping_add(KL_LEN), 8) {
        let klen = core::ptr::read_unaligned((g + KL_LEN) as *const u64);
        let s0 = core::ptr::read_unaligned((g + G_SCORE0) as *const u64);
        let s1 = core::ptr::read_unaligned((g + G_SCORE1) as *const u64);
        if klen > 2 && s0 + s1 == 0 {
            // ★관전 클라 = 지금 보고 있는 리플레이. 그 seed를 래치(편집 패널이 이 세트 자동 로드).
            LAST_WATCHED_SEED.store(seed, Ordering::Relaxed);
            return;
        }
    }
    // ★캡처 게이트 = 화면 경기 OR 내 선수 멤버십(MY_ATH>=3). 화면 경기 신호 2가지:
    //   ① LAST_WATCHED_SEED(주력): 렌더 클라(score 0:0·klen>2)를 위 L505서 감지해 래치한
    //      "지금 화면에 보는 매치" 시드. 그 매치의 서버 sim seed와 일치 = 화면 경기 확정.
    //      리플레이/관전/내경기 전부 렌더 클라를 거치므로 launch 경로·retaddr 버전 무관하게 잡힘.
    //      (리플레이 편집기가 이미 쓰는 검증된 신호 → flow_capture 네이티브.)
    //   ② LIVE_SEED(보조): ctor 훅이 화면 retaddr에서 심은 시드(retaddr 버전 맞을 때만).
    //   ③ MY_ATH 멤버십: 하위호환(위 둘 미설정 대비).
    //   전부 실패면 배경 sim → 스킵.
    let wseed = LAST_WATCHED_SEED.load(Ordering::Relaxed);
    let lseed = LIVE_SEED.load(Ordering::Relaxed);
    let is_onscreen = (wseed != 0 && seed == wseed) || (lseed != 0 && seed == lseed);
    if !is_onscreen {
        let man = MY_ATH_N.load(Ordering::Relaxed) as usize;
        if man > 0 && count_my_ath(g, man) < 3 {
            return;
        }
    }
    // 슬롯 찾기/획득 — 키 = game_ptr(g). 같은 seed의 여러 세트 game을 분리(각자 단조 tick).
    let key = g as u64;
    let mut slot = usize::MAX;
    for i in 0..SLOTS {
        let s = SL_SEED[i].load(Ordering::Relaxed);
        if s == key {
            slot = i;
            break;
        }
        if s == 0 {
            if SL_SEED[i].compare_exchange(0, key, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
                SL_REALSEED[i].store(seed, Ordering::Relaxed);
                SL_OVR[i].store(0, Ordering::Relaxed);
                SL_N[i].store(0, Ordering::Relaxed);
                SL_ROSTER_OK[i].store(0, Ordering::Relaxed);
                SL_LASTBUCKET[i].store(u64::MAX, Ordering::Relaxed);
                K_N[i].store(0, Ordering::Relaxed);
                O_N[i].store(0, Ordering::Relaxed);
                for li in 0..3 { O_SEEN[i][li].store(0, Ordering::Relaxed); }
                CE_N[i].store(0, Ordering::Relaxed);
                SL_NEUT_OK[i].store(0, Ordering::Relaxed);
                SL_NEUT_N[i].store(0, Ordering::Relaxed);
                SL_STRAT_OK[i].store(0, Ordering::Relaxed);
                SL_T0MS[i].store(now_ms_hot(), Ordering::Relaxed);
                slot = i;
                break;
            }
            // CAS 패배 → 그 슬롯이 내 game_ptr일 수도
            if SL_SEED[i].load(Ordering::Relaxed) == key {
                slot = i;
                break;
            }
        }
    }
    if slot == usize::MAX {
        DROPPED_FULL.fetch_add(1, Ordering::Relaxed);
        return;
    }
    // 이 game이 오버라이드 주입 대상이면 슬롯 마킹(flush서 별도 파일명 + 중복제거 skip).
    if OVERRIDE_ON.load(Ordering::Relaxed) && seed == OVERRIDE_SEED.load(Ordering::Relaxed) {
        SL_OVR[slot].store(1, Ordering::Relaxed);
    }
    let tick = core::ptr::read_unaligned((g + G_TICK) as *const u64);
    SL_LASTTICK[slot].store(tick, Ordering::Relaxed);

    // 로스터 1회 캡처
    if SL_ROSTER_OK[slot].load(Ordering::Relaxed) == 0 {
        if readable(g.wrapping_add(ENT_PTR), 16) {
            let ep = core::ptr::read_unaligned((g + ENT_PTR) as *const u64) as usize;
            let el = core::ptr::read_unaligned((g + ENT_LEN) as *const u64) as usize;
            let n = el.min(NATH);
            if ep > 0x10000 && n > 0 && readable(ep, n * ENT_STRIDE) {
                for i in 0..n {
                    let e = ep + i * ENT_STRIDE;
                    let aid = core::ptr::read_unaligned((e + A_ID) as *const u64);
                    let team = core::ptr::read_unaligned((e + A_TEAM) as *const u64) as u32;
                    SL_AID[slot][i].store(aid, Ordering::Relaxed);
                    SL_TEAM[slot][i].store(team, Ordering::Relaxed);
                    // athlete 챔피언명(브릿지 키)
                    let (c0, c1) = read_str16(e, A_NAME_PTR, A_NAME_LEN);
                    SL_ACHAMP[slot][i * 2].store(c0, Ordering::Relaxed);
                    SL_ACHAMP[slot][i * 2 + 1].store(c1, Ordering::Relaxed);
                }
                SL_ROSTER_OK[slot].store(n as u32, Ordering::Release);
                // 세트 번호 = 완료 세트 수 + 1(현재 세트). 서버 확장이 게시한 값(경기 시작 기준).
                SL_SETNO[slot].store(MATCH_COMPLETED.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
                SL_SERIES_U[slot].store(MATCH_SERIES_U.load(Ordering::Relaxed), Ordering::Relaxed);
                SL_SERIES_O[slot].store(MATCH_SERIES_O.load(Ordering::Relaxed), Ordering::Relaxed);
            }
        }
    }

    // 팀 전술(game+0xb248 [Strategy;2], 24B×2) 1회 채록.
    if SL_STRAT_OK[slot].load(Ordering::Relaxed) == 0 && readable(g.wrapping_add(STRAT_OFF), 48) {
        for tm in 0..2 {
            let base = g + STRAT_OFF + tm * STRAT_STRIDE;
            for b in 0..24 {
                let v = core::ptr::read_unaligned((base + b) as *const u8) as u32;
                SL_STRAT[slot][tm * 24 + b].store(v, Ordering::Relaxed);
            }
        }
        SL_STRAT_OK[slot].store(1, Ordering::Release);
    }

    // 전투 엔티티(챔피언 10) 인덱스 맵 — 10명 전원 발견 시에만 확정(그 전엔 샘플마다 재시도)
    if CE_N[slot].load(Ordering::Relaxed) == 0 {
        ce_index_build(slot, g);
    }
    // 중립 몹(정글·곰) 위치 1회 채록(캠프 좌표 실측)
    if SL_NEUT_OK[slot].load(Ordering::Relaxed) == 0 {
        neut_capture(slot, g);
    }

    // 30틱 버킷당 1샘플
    let bucket = tick / SAMPLE_EVERY;
    if SL_LASTBUCKET[slot].swap(bucket, Ordering::Relaxed) == bucket {
        kill_capture(slot, g);
        return;
    }
    let idx = SL_N[slot].load(Ordering::Relaxed) as usize;
    if idx < SMAX {
        // 슬롯 재사용 시 이전 경기 잔여값 방지 — 행을 먼저 0으로
        for i in 0..NATH {
            S_GOLD[slot][idx][i].store(0, Ordering::Relaxed);
            S_X[slot][idx][i].store(0, Ordering::Relaxed);
            S_Y[slot][idx][i].store(0, Ordering::Relaxed);
            S_HP[slot][idx][i].store(0, Ordering::Relaxed);
            S_LV[slot][idx][i].store(0, Ordering::Relaxed);
            S_DEAL[slot][idx][i].store(0, Ordering::Relaxed);
            S_TANK[slot][idx][i].store(0, Ordering::Relaxed);
        }
        for c in 0..NMINCOL {
            S_MIN[slot][idx][c].store(0, Ordering::Relaxed);
        }
        for c in 0..6 {
            S_MINX[slot][idx][c].store(0, Ordering::Relaxed);
            S_MINY[slot][idx][c].store(0, Ordering::Relaxed);
        }
        S_JMASK[slot][idx].store(0, Ordering::Relaxed);
        S_TICK[slot][idx].store(tick as u32, Ordering::Relaxed);
        S_SC0[slot][idx].store(core::ptr::read_unaligned((g + G_SCORE0) as *const u64) as u32, Ordering::Relaxed);
        S_SC1[slot][idx].store(core::ptr::read_unaligned((g + G_SCORE1) as *const u64) as u32, Ordering::Relaxed);
        if readable(g.wrapping_add(ENT_PTR), 16) {
            let ep = core::ptr::read_unaligned((g + ENT_PTR) as *const u64) as usize;
            let el = core::ptr::read_unaligned((g + ENT_LEN) as *const u64) as usize;
            let n = el.min(NATH);
            if ep > 0x10000 && n > 0 && readable(ep, n * ENT_STRIDE) {
                for i in 0..n {
                    let rec = ep + i * ENT_STRIDE;
                    let gold = core::ptr::read_unaligned((rec + A_GOLD) as *const u64) as u32;
                    S_GOLD[slot][idx][i].store(gold, Ordering::Relaxed);
                    // 누적 딜량/탱킹(i64) — 같은 athlete 레코드. 한타 diff 재료.
                    S_DEAL[slot][idx][i].store(core::ptr::read_unaligned((rec + A_DEAL) as *const u64) as u32, Ordering::Relaxed);
                    S_TANK[slot][idx][i].store(core::ptr::read_unaligned((rec + A_TANK) as *const u64) as u32, Ordering::Relaxed);
                }
            }
        }
        // 좌표·HP·레벨 (전투 엔티티) — ★고정 인덱스 금지: 배열이 dense라 죽음/부활/미니언
        //   생성으로 인덱스가 재편된다. 매 샘플 배열을 재스캔해 (team, champion_name)로 선수를
        //   재식별한다. 이름 조회는 kind==0xd(챔피언 ~10개)에만 하므로 비용 낮음. 배열에서 빠진
        //   (죽은) 선수는 매칭 실패 → 그 슬롯 0 유지(= 미표시).
        if CE_N[slot].load(Ordering::Relaxed) as usize == NATH && readable(g.wrapping_add(CE_PTR), 16) {
            let base = core::ptr::read_unaligned((g + CE_PTR) as *const u64) as usize;
            let cnt = core::ptr::read_unaligned((g + CE_CNT) as *const u64) as usize;
            let scan = cnt.min(CE_SCAN_MAX);
            if base > 0x10000 && scan >= NATH && readable(base, scan * CE_STRIDE) {
                for i in 0..scan {
                    let e = base + i * CE_STRIDE;
                    let kind = core::ptr::read_unaligned((e + CE_KIND) as *const u32);
                    if kind == CE_MIN_KIND {
                        // 미니언: 팀×레인 카운트(wav) + 웨이브 중심점 좌표합(뷰어).
                        let mt = core::ptr::read_unaligned((e + CE_TEAM) as *const u32);
                        if mt < 2 {
                            let lane = core::ptr::read_unaligned((e + CE_LANE) as *const u8) as usize;
                            let col = (mt as usize) * 4 + if lane < 3 { lane } else { 3 };
                            let c = S_MIN[slot][idx][col].load(Ordering::Relaxed);
                            S_MIN[slot][idx][col].store(c + 1, Ordering::Relaxed);
                            if lane < 3 {
                                let cb = (mt as usize) * 3 + lane;
                                let mx = core::ptr::read_unaligned((e + CE_X) as *const u64) as u32;
                                let my = core::ptr::read_unaligned((e + CE_Y) as *const u64) as u32;
                                S_MINX[slot][idx][cb].store(S_MINX[slot][idx][cb].load(Ordering::Relaxed).wrapping_add(mx), Ordering::Relaxed);
                                S_MINY[slot][idx][cb].store(S_MINY[slot][idx][cb].load(Ordering::Relaxed).wrapping_add(my), Ordering::Relaxed);
                            }
                        }
                        continue;
                    }
                    if kind == 4 || kind == 9 {
                        // 정글/곰: 생존(hp>0)이면 최근접 캠프 비트 셋(뷰어 몹 죽음/스폰).
                        let hp = core::ptr::read_unaligned((e + CE_HP) as *const u64);
                        if hp > 0 {
                            let mx = core::ptr::read_unaligned((e + CE_X) as *const u64) as i64;
                            let my = core::ptr::read_unaligned((e + CE_Y) as *const u64) as i64;
                            let nn = (SL_NEUT_N[slot].load(Ordering::Relaxed) as usize).min(MAXNEUT).min(32);
                            for ci in 0..nn {
                                let cx = SL_NX[slot][ci].load(Ordering::Relaxed) as i64;
                                let cy = SL_NY[slot][ci].load(Ordering::Relaxed) as i64;
                                if (mx - cx).abs() < 24000 && (my - cy).abs() < 24000 {
                                    let m = S_JMASK[slot][idx].load(Ordering::Relaxed);
                                    S_JMASK[slot][idx].store(m | (1u32 << ci), Ordering::Relaxed);
                                    break;
                                }
                            }
                        }
                        continue;
                    }
                    if kind != CE_KIND_CHAMP {
                        continue;
                    }
                    let team = core::ptr::read_unaligned((e + CE_TEAM) as *const u32);
                    let (n0, n1) = read_name16(e);
                    // CE 슬롯(team, name)과 매칭
                    for j in 0..NATH {
                        if CE_TEAMS[slot][j].load(Ordering::Relaxed) == team
                            && CE_NAME[slot][j * 2].load(Ordering::Relaxed) == n0
                            && CE_NAME[slot][j * 2 + 1].load(Ordering::Relaxed) == n1
                        {
                            S_X[slot][idx][j].store(core::ptr::read_unaligned((e + CE_X) as *const u64) as u32, Ordering::Relaxed);
                            S_Y[slot][idx][j].store(core::ptr::read_unaligned((e + CE_Y) as *const u64) as u32, Ordering::Relaxed);
                            S_HP[slot][idx][j].store(core::ptr::read_unaligned((e + CE_HP) as *const u64) as u32, Ordering::Relaxed);
                            S_LV[slot][idx][j].store(core::ptr::read_unaligned((e + CE_LEVEL) as *const u64) as u32, Ordering::Relaxed);
                            break;
                        }
                    }
                }
            }
        }
        SL_N[slot].store(idx as u32 + 1, Ordering::Release);
    }
    kill_capture(slot, g);
    obj_capture(slot, g);
}

// 전투 엔티티 champion_name String(ptr@0x250/len@0x258) → 앞 16B를 (u64,u64)로.
//   ⚠핫패스 — 패닉 가능 경로(try_into().unwrap() 등) 금지, 포인터 read로만 조립.
#[inline]
unsafe fn read_name16(e: usize) -> (u64, u64) {
    read_str16(e, CE_NAME_PTR, CE_NAME_LEN)
}
// String(ptr@ptr_off, len@len_off) 앞 16B를 2×u64로. 안전가드 내장.
unsafe fn read_str16(e: usize, ptr_off: usize, len_off: usize) -> (u64, u64) {
    let mut nb = [0u8; 16];
    let np = core::ptr::read_unaligned((e + ptr_off) as *const u64) as usize;
    let nl = core::ptr::read_unaligned((e + len_off) as *const u64) as usize;
    if np > 0x10000 && nl > 0 && nl < 4096 && readable(np, nl.min(16)) {
        core::ptr::copy_nonoverlapping(np as *const u8, nb.as_mut_ptr(), nl.min(16));
    }
    (
        core::ptr::read_unaligned(nb.as_ptr() as *const u64),
        core::ptr::read_unaligned(nb.as_ptr().add(8) as *const u64),
    )
}

// 전투 엔티티 배열에서 챔피언(kind==0xd) 10명의 팀·이름(16B)을 1회 채록(재식별 테이블).
// 10명 전원 발견 시에만 커밋 — 부분 발견은 다음 샘플에서 재시도(경기 초기 생성 순서 대응).
// ⚠이름이 초기화 전(빈 String)이면 매칭 실패하므로, name이 비어있지 않은 10명 확보 시에만 커밋.
#[inline]
unsafe fn ce_index_build(slot: usize, g: usize) {
    if !readable(g.wrapping_add(CE_PTR), 16) {
        return;
    }
    let base = core::ptr::read_unaligned((g + CE_PTR) as *const u64) as usize;
    let cnt = core::ptr::read_unaligned((g + CE_CNT) as *const u64) as usize;
    if base < 0x10000 || cnt < NATH || cnt > 100_000 {
        return;
    }
    let scan = cnt.min(CE_SCAN_MAX);
    if !readable(base, scan * CE_STRIDE) {
        return;
    }
    let mut found = 0usize;
    for i in 0..scan {
        let e = base + i * CE_STRIDE;
        if core::ptr::read_unaligned((e + CE_KIND) as *const u32) != CE_KIND_CHAMP {
            continue;
        }
        let (n0, n1) = read_name16(e);
        if n0 == 0 {
            return; // 이름 미초기화 = 아직 이르다 → 다음 샘플 재시도
        }
        CE_TEAMS[slot][found].store(core::ptr::read_unaligned((e + CE_TEAM) as *const u32), Ordering::Relaxed);
        CE_NAME[slot][found * 2].store(n0, Ordering::Relaxed);
        CE_NAME[slot][found * 2 + 1].store(n1, Ordering::Relaxed);
        found += 1;
        if found == NATH {
            break;
        }
    }
    if found == NATH {
        CE_N[slot].store(found as u32, Ordering::Release);
    }
}

// 중립 몹(정글 kind4·곰 kind9) 위치 1회 채록. 스폰 직후 정지상태에서 잡아 캠프 좌표로 사용.
//   epic(5)/serpen(6)은 스폰이 늦고 좌표가 이미 확정(mapgeo)이라 제외.
#[inline]
unsafe fn neut_capture(slot: usize, g: usize) {
    if !readable(g.wrapping_add(CE_PTR), 16) {
        return;
    }
    let base = core::ptr::read_unaligned((g + CE_PTR) as *const u64) as usize;
    let cnt = core::ptr::read_unaligned((g + CE_CNT) as *const u64) as usize;
    if base < 0x10000 || cnt < NATH || cnt > 100_000 {
        return;
    }
    let scan = cnt.min(CE_SCAN_MAX);
    if !readable(base, scan * CE_STRIDE) {
        return;
    }
    let mut found = 0usize;
    for i in 0..scan {
        if found >= MAXNEUT {
            break;
        }
        let e = base + i * CE_STRIDE;
        let kind = core::ptr::read_unaligned((e + CE_KIND) as *const u32);
        if kind != 4 && kind != 9 {
            continue; // 정글·곰만
        }
        let (n0, n1) = read_name16(e);
        if n0 == 0 {
            continue; // 이름 미초기화 = 아직 이르다(이번 스캔서 이 개체만 건너뜀)
        }
        let x = core::ptr::read_unaligned((e + CE_X) as *const u64) as u32;
        let y = core::ptr::read_unaligned((e + CE_Y) as *const u64) as u32;
        SL_NKIND[slot][found].store(kind, Ordering::Relaxed);
        SL_NTEAM[slot][found].store(core::ptr::read_unaligned((e + CE_TEAM) as *const u32), Ordering::Relaxed);
        SL_NX[slot][found].store(x, Ordering::Relaxed);
        SL_NY[slot][found].store(y, Ordering::Relaxed);
        SL_NNAME[slot][found * 2].store(n0, Ordering::Relaxed);
        SL_NNAME[slot][found * 2 + 1].store(n1, Ordering::Relaxed);
        found += 1;
    }
    // 정글 캠프(측당 4 = 최소 8) 이상 확보 시에만 확정 — 부분 스폰 중 조기 커밋 방지.
    if found >= 8 {
        SL_NEUT_N[slot].store(found as u32, Ordering::Relaxed);
        SL_NEUT_OK[slot].store(1, Ordering::Release);
    }
}

// kill_logs 증분 사본 — 원소를 디코드해 저장:
//   w0/w1=tick(lo/hi) w2=killer_team w3=killer_pos w4=killed_pos w5=assist_len w6..w9=assist roles(≤4)
#[inline]
unsafe fn kill_capture(slot: usize, g: usize) {
    if !readable(g.wrapping_add(KL_PTR), 16) {
        return;
    }
    let ptr = core::ptr::read_unaligned((g + KL_PTR) as *const u64) as usize;
    let len = core::ptr::read_unaligned((g + KL_LEN) as *const u64) as usize;
    if ptr < 0x10000 || len == 0 || len > 4096 {
        return;
    }
    let seen = K_N[slot].load(Ordering::Relaxed) as usize;
    if len <= seen {
        return;
    }
    let upto = len.min(KMAX);
    if upto > seen && !readable(ptr + seen * KL_STRIDE, (upto - seen) * KL_STRIDE) {
        return;
    }
    for k in seen..upto {
        let e = ptr + k * KL_STRIDE;
        let tick = core::ptr::read_unaligned((e + KLE_TICK) as *const u64);
        K_RAW[slot][k][0].store(tick as u32, Ordering::Relaxed);
        K_RAW[slot][k][1].store((tick >> 32) as u32, Ordering::Relaxed);
        K_RAW[slot][k][2].store(core::ptr::read_unaligned((e + KLE_TEAM) as *const u64) as u32, Ordering::Relaxed);
        K_RAW[slot][k][3].store(core::ptr::read_unaligned((e + KLE_KPOS) as *const u32), Ordering::Relaxed);
        K_RAW[slot][k][4].store(core::ptr::read_unaligned((e + KLE_DPOS) as *const u32), Ordering::Relaxed);
        let ap = core::ptr::read_unaligned((e + KLE_ASSIST_PTR) as *const u64) as usize;
        let an = core::ptr::read_unaligned((e + KLE_ASSIST_LEN) as *const u64) as usize;
        K_RAW[slot][k][5].store(an.min(255) as u32, Ordering::Relaxed);
        for j in 0..4 {
            K_RAW[slot][k][6 + j].store(u32::MAX, Ordering::Relaxed); // 슬롯 재사용 잔여값 방지(MAX=없음)
        }
        let take = an.min(4);
        if take > 0 && ap > 0x10000 && readable(ap, take * 4) {
            for j in 0..take {
                K_RAW[slot][k][6 + j].store(
                    core::ptr::read_unaligned((ap + j * 4) as *const u32),
                    Ordering::Relaxed,
                );
            }
        }
    }
    K_N[slot].store(upto as u32, Ordering::Release);
}

// 오브젝트 처치 로그(tower/epic/serpen) 증분 캡처 → O 이벤트 병합
#[inline]
unsafe fn obj_capture(slot: usize, g: usize) {
    for (li, &(cap, stride, otype)) in OBJ_LOGS.iter().enumerate() {
        if !readable(g.wrapping_add(cap), 24) {
            continue;
        }
        let ptr = core::ptr::read_unaligned((g + cap + 8) as *const u64) as usize;
        let len = core::ptr::read_unaligned((g + cap + 16) as *const u64) as usize;
        if ptr < 0x10000 || len > 4096 {
            continue;
        }
        let seen = O_SEEN[slot][li].load(Ordering::Relaxed) as usize;
        if len <= seen {
            continue;
        }
        for i in seen..len {
            let e = ptr + i * stride;
            if !readable(e, stride) {
                break;
            }
            let n = O_N[slot].load(Ordering::Relaxed) as usize;
            if n >= OMAX {
                break;
            }
            O_TICK[slot][n].store(core::ptr::read_unaligned((e + OE_TICK) as *const u64) as u32, Ordering::Relaxed);
            O_TEAM[slot][n].store(core::ptr::read_unaligned((e + OE_TEAM) as *const u64) as u32, Ordering::Relaxed);
            O_TYPE[slot][n].store(otype, Ordering::Relaxed);
            O_LINE[slot][n].store(if otype == 0 { core::ptr::read_unaligned((e + OE_LINE) as *const u8) as u32 } else { 255 }, Ordering::Relaxed);
            O_N[slot].store(n as u32 + 1, Ordering::Release);
        }
        O_SEEN[slot][li].store(len as u32, Ordering::Release);
    }
}

// 핫패스용 시각 (SystemTime 호출은 syscall 1회 — 슬롯 신규 획득 시에만 사용)
#[inline]
fn now_ms_hot() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}

// ── 훅 설치 (체인 인지·1회) ────────────────────────────────────────────────
unsafe fn install_tick_hook() -> Result<String, String> {
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 {
        return Err("module 0".into());
    }
    let fn_addr = base + RUN_TICK_RVA;
    if !readable(fn_addr, 12) {
        return Err(format!("unreadable @0x{:x}", fn_addr));
    }
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    // 체인: 진입부가 이미 외부 모드 훅(movabs rax,tgt; jmp rax)이면 그 스텁으로 체인
    let chained = cur[0] == 0x48 && cur[1] == 0xb8 && cur[10] == 0xff && cur[11] == 0xe0;
    if !chained && cur != HOOK_PROLOGUE12 && cur != HOOK_PROLOGUE12_ALT {
        return Err(format!("prologue mismatch cur={:02x?}", cur));
    }
    let stub = VirtualAlloc(0, 32, 0x1000 | 0x2000, 0x40);
    if stub == 0 {
        return Err("VirtualAlloc".into());
    }
    let mut s: Vec<u8> = Vec::with_capacity(24);
    s.extend_from_slice(&cur); // 체인: 외부 스텁으로 jmp / 비체인: 원본 프롤로그 12B
    if !chained {
        s.extend_from_slice(&[0x48, 0xb8]);
        s.extend_from_slice(&(fn_addr + 12).to_le_bytes());
        s.extend_from_slice(&[0xff, 0xe0]);
    }
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    TRAMP.store(stub, Ordering::Release);
    let mut patch = [0u8; 12];
    patch[0] = 0x48;
    patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&(tick_detour as usize).to_le_bytes());
    patch[10] = 0xff;
    patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, 0x40, &mut old) == 0 {
        return Err("VirtualProtect".into());
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    Ok(format!("ok @0x{:x} stub=0x{:x} chained={}", fn_addr, stub, chained))
}

// ── 다시보기 편집기 Step1: World 생성자(0x13b53d0) 호출자 RVA 캡처 ────────────
//   목적: 9개 매치런처 중 "리플레이 런처"를 런타임 특정(RE 2026-08-12 권장경로).
//   캡처스텁이 진입 시 [rsp](=호출자 리턴주소)를 CTOR_RET에 저장 → ctor_detour가 RVA 기록.
//   다시보기 클릭 직후(DABO_CLICK_MS 기준 3초) 발화한 호출자 = 리플레이 런처.
const CTOR_RVA: usize = 0x14dda60; // 0.5.6(구0.5.5=0x14ac3e0). head-UNIQUE + 마스크시그 UNIQUE + launcher 콜사이트 9/9 전단사(item_tactics/serpen 교차)·첫12B=HOOK_PROLOGUE12 불변(chkstk imm 0x25438→0x25438 프롤로그14 동일). // 0.5.5(구0.5.4=0x13b53d0). 구조skel LCP 342(압도) + item_tactics/serpen LAUNCHER 교차 + 프롤로그[13] 0x68→0x38(chkstk imm 0x25168→0x25438). 첫12B=HOOK_PROLOGUE12 불변(설치체크 통과)
static CTOR_INSTALLED: AtomicU32 = AtomicU32::new(0);
static CTOR_TRAMP: AtomicUsize = AtomicUsize::new(0);
static CTOR_RET: AtomicUsize = AtomicUsize::new(0); // 캡처스텁이 [rsp] 저장
static CTOR_BASE: AtomicUsize = AtomicUsize::new(0);
static CTOR_R_RVA: [AtomicU64; 24] = [const { AtomicU64::new(0) }; 24]; // 호출자 RVA
static CTOR_R_CNT: [AtomicU64; 24] = [const { AtomicU64::new(0) }; 24]; // 총 호출수
static CTOR_R_PC: [AtomicU64; 24] = [const { AtomicU64::new(0) }; 24]; // 다시보기 클릭 직후 호출수
pub(crate) static DABO_CLICK_MS: AtomicU64 = AtomicU64::new(0); // 다시보기 클릭 시각(ui_replay 인터셉터가 설정)
// ★화면 경기 식별(item_tactics 이식): 같은 0x14ac3e0 launcher 진입 시 retaddr가 화면 경로면
//   r8(=ctor_detour a3)=seed 를 LIVE_SEED 에 게시. capture()가 seed==LIVE_SEED 로 화면 경기 확정
//   (배경 리그 sim은 화면 retaddr 아님 → 무매칭). sim 계층엔 team id가 없어 경기 아이덴티티(seed)로 특정.
//   ⚠0.5.5 화면 retaddr — 패치 때 재핀(item_tactics lib.rs L2216 참조):
//     관전 0x763329 · 내경기 0x76829b · 조테본경기 0x1aed292 · 조테기록 0x1aa88ce.
//     배경(state.rs/solo_rank/worker.rs)은 절대 넣지 말 것.
const ONSCREEN_RETADDRS: [u64; 4] = [0x763329, 0x76829b, 0x1aed292, 0x1aa88ce];
static LIVE_SEED: AtomicU64 = AtomicU64::new(0);   // 화면 경기 시드(ctor 훅이 화면 retaddr에서 게시)
static LIVE_SEED_N: AtomicU64 = AtomicU64::new(0); // 화면 판정 발화수(진단)
static CTOR_R_SEED: [AtomicU64; 24] = [const { AtomicU64::new(0) }; 24]; // 진단: retaddr별 마지막 seed(r8)
static LAST_HANDLED_CLICK: AtomicU64 = AtomicU64::new(0); // 다시보기 클릭 처리 완료 시각(첫 ctor만 잡기)

extern "win64" fn ctor_detour(
    a1: usize, a2: usize, a3: usize, a4: usize, a5: usize, a6: usize, a7: usize, a8: usize,
) -> usize {
    let ret = CTOR_RET.load(Ordering::Relaxed);
    let base = CTOR_BASE.load(Ordering::Relaxed);
    if base != 0 && ret > base && ret - base < (1usize << 32) {
        let rva = (ret - base) as u64;
        // ★화면 경기 식별: retaddr가 화면 경로면 a3(=r8=seed)를 LIVE_SEED에 게시(capture 게이트가 사용).
        //   a3 = win64 3번째 인자 = r8 = seed (capstub이 rcx/rdx/r8/r9 안 건드리고 진입 → a3=r8).
        if a3 != 0 && ONSCREEN_RETADDRS.contains(&rva) {
            LIVE_SEED.store(a3 as u64, Ordering::Relaxed);
            LIVE_SEED_N.fetch_add(1, Ordering::Relaxed);
        }
        let click = DABO_CLICK_MS.load(Ordering::Relaxed);
        let pc = click != 0 && now_ms_hot().saturating_sub(click) < 3000;
        // ★다시보기 클릭 직후 "첫" ctor 호출 = 리플레이 런처(클릭이 즉시 런치 유발 → 배경 sim보다 먼저).
        //   그 a3(=r8=seed)=지금 보는 리플레이 시드 → LIVE_SEED. 새 클릭당 1회만(배경 오염 방지).
        //   retaddr 버전 무관하게 화면 경기(다시보기)를 특정.
        if pc && a3 != 0 {
            let handled = LAST_HANDLED_CLICK.load(Ordering::Relaxed);
            if handled != click
                && LAST_HANDLED_CLICK
                    .compare_exchange(handled, click, Ordering::AcqRel, Ordering::Relaxed)
                    .is_ok()
            {
                LIVE_SEED.store(a3 as u64, Ordering::Relaxed);
                LIVE_SEED_N.fetch_add(1, Ordering::Relaxed);
            }
        }
        for i in 0..24 {
            let curv = CTOR_R_RVA[i].load(Ordering::Relaxed);
            if curv == rva {
                CTOR_R_CNT[i].fetch_add(1, Ordering::Relaxed);
                CTOR_R_SEED[i].store(a3 as u64, Ordering::Relaxed);
                if pc { CTOR_R_PC[i].fetch_add(1, Ordering::Relaxed); }
                break;
            }
            if curv == 0
                && CTOR_R_RVA[i]
                    .compare_exchange(0, rva, Ordering::AcqRel, Ordering::Relaxed)
                    .is_ok()
            {
                CTOR_R_CNT[i].fetch_add(1, Ordering::Relaxed);
                CTOR_R_SEED[i].store(a3 as u64, Ordering::Relaxed);
                if pc { CTOR_R_PC[i].fetch_add(1, Ordering::Relaxed); }
                break;
            }
        }
    }
    let stub = CTOR_TRAMP.load(Ordering::Relaxed);
    if stub == 0 {
        return 0;
    }
    let f: extern "win64" fn(usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        unsafe { core::mem::transmute(stub) };
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(a1, a2, a3, a4, a5, a6, a7, a8)))
        .unwrap_or(0)
}

unsafe fn install_ctor_hook() -> Result<String, String> {
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 {
        return Err("module 0".into());
    }
    CTOR_BASE.store(base, Ordering::Relaxed);
    let fn_addr = base + CTOR_RVA;
    if !readable(fn_addr, 12) {
        return Err(format!("unreadable @0x{:x}", fn_addr));
    }
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    let chained = cur[0] == 0x48 && cur[1] == 0xb8 && cur[10] == 0xff && cur[11] == 0xe0;
    if !chained && cur != HOOK_PROLOGUE12 && cur != HOOK_PROLOGUE12_ALT {
        return Err(format!("prologue mismatch {:02x?}", cur));
    }
    // 트램폴린: 원본 12B + jmp fn+12 (비체인) / 체인이면 그 스텁으로.
    let tramp = VirtualAlloc(0, 32, 0x1000 | 0x2000, 0x40);
    if tramp == 0 {
        return Err("valloc tramp".into());
    }
    let mut ts: Vec<u8> = Vec::new();
    ts.extend_from_slice(&cur);
    if !chained {
        ts.extend_from_slice(&[0x48, 0xb8]);
        ts.extend_from_slice(&(fn_addr + 12).to_le_bytes());
        ts.extend_from_slice(&[0xff, 0xe0]);
    }
    core::ptr::copy_nonoverlapping(ts.as_ptr(), tramp as *mut u8, ts.len());
    CTOR_TRAMP.store(tramp, Ordering::Release);
    // 캡처 스텁: mov rax,[rsp]; mov [CTOR_RET],rax; movabs rax,ctor_detour; jmp rax
    let capstub = VirtualAlloc(0, 64, 0x1000 | 0x2000, 0x40);
    if capstub == 0 {
        return Err("valloc capstub".into());
    }
    let ret_addr = &CTOR_RET as *const AtomicUsize as usize;
    let mut cs: Vec<u8> = Vec::new();
    cs.extend_from_slice(&[0x48, 0x8b, 0x04, 0x24]); // mov rax,[rsp]
    cs.extend_from_slice(&[0x48, 0xa3]); // mov [moffs64],rax
    cs.extend_from_slice(&ret_addr.to_le_bytes());
    cs.extend_from_slice(&[0x48, 0xb8]); // movabs rax,ctor_detour
    cs.extend_from_slice(&(ctor_detour as usize).to_le_bytes());
    cs.extend_from_slice(&[0xff, 0xe0]); // jmp rax
    core::ptr::copy_nonoverlapping(cs.as_ptr(), capstub as *mut u8, cs.len());
    // 패치: fn 12B → movabs rax,capstub; jmp rax
    let mut patch = [0u8; 12];
    patch[0] = 0x48;
    patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&capstub.to_le_bytes());
    patch[10] = 0xff;
    patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, 0x40, &mut old) == 0 {
        return Err("vprotect".into());
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    Ok(format!("ctor hook @0x{:x} capstub=0x{:x} tramp=0x{:x}", fn_addr, capstub, tramp))
}

// 생성자 호출자 RVA 로그 파일.
fn write_ctor_debug() {
    let Some(mut p) = mod_dir() else { return };
    p.push("ctor_callers.txt");
    let mut s = String::from("caller_rva  total  post_click(다시보기직후)  last_seed\n");
    for i in 0..24 {
        let rva = CTOR_R_RVA[i].load(Ordering::Relaxed);
        if rva == 0 {
            continue;
        }
        s.push_str(&format!(
            "0x{:x}  {}  {}  0x{:x}\n",
            rva,
            CTOR_R_CNT[i].load(Ordering::Relaxed),
            CTOR_R_PC[i].load(Ordering::Relaxed),
            CTOR_R_SEED[i].load(Ordering::Relaxed)
        ));
    }
    let _ = fs::write(&p, s);
}

// ── 밴픽 씬 훅 설치 (14B 재배치·체인 인지·1회) ─────────────────────────────
unsafe fn install_scene_hook() -> Result<String, String> {
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 {
        return Err("module 0".into());
    }
    let fn_addr = base + SCENE_STEP_RVA;
    if !readable(fn_addr, 14) {
        return Err(format!("unreadable @0x{:x}", fn_addr));
    }
    let mut cur = [0u8; 14];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 14);
    // 체인: 진입부가 이미 외부 훅(movabs rax,tgt; jmp rax = banpick_order 등)이면 그 스텁으로 체인
    let chained = cur[0] == 0x48 && cur[1] == 0xb8 && cur[10] == 0xff && cur[11] == 0xe0;
    if !chained && cur != BP_PROLOGUE14 {
        return Err(format!("prologue mismatch cur={:02x?}", cur));
    }
    let stub = VirtualAlloc(0, 48, 0x1000 | 0x2000, 0x40);
    if stub == 0 {
        return Err("VirtualAlloc".into());
    }
    let mut s: Vec<u8> = Vec::with_capacity(32);
    if chained {
        // 외부 스텁 jmp(12B) 그대로 — 내 detour가 store 후 그리로 점프(외부 모드 detour 순차 발화)
        s.extend_from_slice(&cur[..12]);
    } else {
        // 원본 프롤로그 14B(두 mov reg,[rcx+disp32] — rip-rel 없음) 재배치 + 본문(fn+14)로 복귀
        s.extend_from_slice(&cur[..14]);
        s.extend_from_slice(&[0x48, 0xb8]);
        s.extend_from_slice(&(fn_addr + 14).to_le_bytes());
        s.extend_from_slice(&[0xff, 0xe0]);
    }
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    BP_TRAMP.store(stub, Ordering::Release);
    let mut patch = [0u8; 12];
    patch[0] = 0x48;
    patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&(scene_step_detour as usize).to_le_bytes());
    patch[10] = 0xff;
    patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, 0x40, &mut old) == 0 {
        return Err("VirtualProtect".into());
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    Ok(format!("ok @0x{:x} stub=0x{:x} chained={}", fn_addr, stub, chained))
}

// Rust String{cap@0,ptr@8,len@0x10} 1개를 안전하게 읽음(챔프 내부이름, ~≤64자).
unsafe fn read_rust_string(sp: usize) -> Option<String> {
    if !readable(sp, 0x18) {
        return None;
    }
    let dptr = core::ptr::read_unaligned((sp + 8) as *const u64) as usize;
    let dlen = core::ptr::read_unaligned((sp + 0x10) as *const u64) as usize;
    if dptr < 0x10000 || dlen == 0 || dlen > 128 || !readable(dptr, dlen) {
        return None;
    }
    let mut buf = vec![0u8; dlen];
    core::ptr::copy_nonoverlapping(dptr as *const u8, buf.as_mut_ptr(), dlen);
    String::from_utf8(buf).ok()
}

// BanpickScene의 Vec<String>(ptr@vec_off/len@vec_off+8)에서 챔프 이름들을 읽음.
unsafe fn read_champ_vec(scene: usize, vec_off: usize) -> Vec<String> {
    let mut out = Vec::new();
    if !readable(scene + vec_off, 16) {
        return out;
    }
    let ptr = core::ptr::read_unaligned((scene + vec_off) as *const u64) as usize;
    let len = core::ptr::read_unaligned((scene + vec_off + 8) as *const u64) as usize;
    if ptr < 0x10000 || len == 0 || len > 20 || !readable(ptr, len * BP_STR_STRIDE) {
        return out;
    }
    for i in 0..len {
        if let Some(name) = read_rust_string(ptr + i * BP_STR_STRIDE) {
            out.push(name);
        }
    }
    out
}

fn json_arr(v: &[String]) -> String {
    let items: Vec<String> = v.iter().map(|s| format!("\"{}\"", s.replace('\\', "").replace('"', ""))).collect();
    format!("[{}]", items.join(","))
}

// draft 중이면(active=true) 현재 밴/픽을, 종료면(active=false) 비활성 상태를 banpick_live.json에 기록.
fn write_banpick_out(body: &str) {
    // .json(도구·검사용) + .js(file:// 페이지가 <script>로 로드) 둘 다 기록.
    if let Some(mut p) = mod_dir() {
        p.push("banpick_live.json");
        let _ = fs::write(&p, body);
    }
    if let Some(mut p) = mod_dir() {
        p.push("banpick_live.js");
        let _ = fs::write(&p, format!("window.__BP_LIVE={};", body));
    }
}

// ── recommend_strategy_inner 훅 설치 (12B=8push 재배치·1회) ────────────────
unsafe fn install_reco_hook() -> Result<String, String> {
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 {
        return Err("module 0".into());
    }
    let fn_addr = base + RECO_INNER_RVA;
    if !readable(fn_addr, 12) {
        return Err(format!("unreadable @0x{:x}", fn_addr));
    }
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    let chained = cur[0] == 0x48 && cur[1] == 0xb8 && cur[10] == 0xff && cur[11] == 0xe0;
    if !chained && cur != RECO_PROLOGUE12 {
        return Err(format!("prologue mismatch cur={:02x?}", cur));
    }
    let stub = VirtualAlloc(0, 32, 0x1000 | 0x2000, 0x40);
    if stub == 0 {
        return Err("VirtualAlloc".into());
    }
    let mut s: Vec<u8> = Vec::with_capacity(24);
    s.extend_from_slice(&cur); // 8push(pos-independent) / 체인이면 외부 스텁 jmp
    if !chained {
        s.extend_from_slice(&[0x48, 0xb8]);
        s.extend_from_slice(&(fn_addr + 12).to_le_bytes());
        s.extend_from_slice(&[0xff, 0xe0]);
    }
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    RECO_TRAMP.store(stub, Ordering::Release);
    let mut patch = [0u8; 12];
    patch[0] = 0x48;
    patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&(reco_detour as usize).to_le_bytes());
    patch[10] = 0xff;
    patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, 0x40, &mut old) == 0 {
        return Err("VirtualProtect".into());
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    Ok(format!("ok @0x{:x} stub=0x{:x} chained={}", fn_addr, stub, chained))
}

// 이름이 챔프 내부이름 형태([a-z0-9_], 3~32)인가.
fn looks_champ(s: &str) -> bool {
    let n = s.len();
    n >= 3 && n <= 32 && s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
}
// MatchSetInfo의 Vec<String> 픽 필드(ptr@field) 읽기 — len이 +8 또는 +0x10 중 "전원 챔프명"인 해석 채택.
unsafe fn read_pick_vec(field: usize, out: &mut Vec<String>) {
    let ptr = rd(field).unwrap_or(0);
    if ptr < 0x10000 {
        return;
    }
    for lo in [field + 8, field + 0x10] {
        let len = rd(lo).unwrap_or(0);
        if len == 0 || len > 80 || !readable(ptr, len * BP_STR_STRIDE) {
            continue;
        }
        let mut names: Vec<String> = Vec::new();
        let mut ok = true;
        for k in 0..len {
            match read_rust_string(ptr + k * BP_STR_STRIDE) {
                Some(nm) if looks_champ(&nm) => names.push(nm),
                _ => { ok = false; break; }
            }
        }
        if ok && !names.is_empty() {
            for nm in names {
                if !out.contains(&nm) {
                    out.push(nm);
                }
            }
            return;
        }
    }
}
// 피어리스 불가목록 = RunningMatchInfo.set_info[*]의 team1_picks(+0x68)+team2_picks(+0x80) 합집합(RE 2026-08-11).
//   RMI+0x151=banpick_style(0=classic→빈). set_info ptr@RMI+0x08/len@+0x10, MatchSetInfo stride 0x100.
unsafe fn extract_fearless_rmi(rmi: usize) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    if rdu8(rmi + 0x151) == Some(0) {
        return out; // classic
    }
    let sp = rd(rmi + 0x08).unwrap_or(0);
    let sn = rd(rmi + 0x10).unwrap_or(0);
    if sp < 0x10000 || sn == 0 || sn > 20 || !readable(sp, sn * 0x100) {
        return out;
    }
    for i in 0..sn {
        let msi = sp + i * 0x100;
        read_pick_vec(msi + 0x68, &mut out);
        read_pick_vec(msi + 0x80, &mut out);
    }
    out
}

// 진단: reco 훅 발화 현황 → reco_debug.txt (어느 스레드서 얼마나, 메인 매칭 여부).
fn write_reco_debug() {
    let mut tids = String::new();
    for i in 0..8 {
        tids.push_str(&format!("(tid={},len={}) ", RECO_TID_RING[i].load(Ordering::Relaxed), RECO_LEN_RING[i].load(Ordering::Relaxed)));
    }
    let body = format!(
        "MAIN_TID={} ANY={} MAIN={} SEQ={}\nring: {}\n",
        MAIN_TID.load(Ordering::Relaxed),
        RECO_ANY.load(Ordering::Relaxed),
        RECO_MAIN.load(Ordering::Relaxed),
        RECO_SEQ.load(Ordering::Relaxed),
        tids
    );
    if let Some(mut p) = mod_dir() {
        p.push("reco_debug.txt");
        let _ = fs::write(&p, body);
    }
}

// 캡처한 추천 Strategy 24B를 디코드해 game_reco.json/.js에 기록(위임버튼 눌렀을 때).
fn write_game_reco() {
    let mut b = [0u8; 24];
    b[0..8].copy_from_slice(&RECO_S0.load(Ordering::Relaxed).to_le_bytes());
    b[8..16].copy_from_slice(&RECO_S1.load(Ordering::Relaxed).to_le_bytes());
    b[16..24].copy_from_slice(&RECO_S2.load(Ordering::Relaxed).to_le_bytes());
    let u32at = |o: usize| u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
    let body = format!(
        "{{\"ts\":{},\"seq\":{},\"strat\":{{\"bld\":{},\"mor\":{},\"mor_pos\":{},\"bat\":{},\"twr\":{},\"def\":{},\
\"fin\":{},\"wav\":{},\"foc\":{},\"jng\":{},\"srp\":{},\"srt\":{},\"end\":{}}}}}",
        now_ms(), RECO_SEQ.load(Ordering::Relaxed),
        u32at(0), u32at(4), u32at(8), b[12], b[13], b[14], b[15], b[16], b[17], b[18], b[19], b[20], b[21]
    );
    if let Some(mut p) = mod_dir() {
        p.push("game_reco.json");
        let _ = fs::write(&p, &body);
    }
    if let Some(mut p) = mod_dir() {
        p.push("game_reco.js");
        let _ = fs::write(&p, format!("window.__GAME_RECO={};", body));
    }
}

fn write_banpick_json(active: bool) {
    let my_tid = PLAYER_TID.load(Ordering::Relaxed);
    let my_tid_s = if my_tid == u64::MAX { "-1".to_string() } else { my_tid.to_string() };
    if !active {
        write_banpick_out(&format!("{{\"active\":false,\"ts\":{},\"my_tid\":{}}}", now_ms(), my_tid_s));
        return;
    }
    let scene = BP_SCENE.load(Ordering::Relaxed);
    if scene == 0 {
        return;
    }
    // 씬 sanity: 마지막 Vec len 필드까지 읽히나
    let (b1, b2, p1, p2, rule, blue) = unsafe {
        if !readable(scene + BP_PICK2 + 8, 8) {
            return;
        }
        (
            read_champ_vec(scene, BP_BAN1),
            read_champ_vec(scene, BP_BAN2),
            read_champ_vec(scene, BP_PICK1),
            read_champ_vec(scene, BP_PICK2),
            rdu8(scene + BP_RULE).unwrap_or(255),
            rd(scene + BP_BLUE_TID).unwrap_or(0),
        )
    };
    let picks_per_team = if rule <= 3 { (rule as u32) + 2 } else { 0 };
    // 내 사이드: team1=blue 확정(인게임 실증 2026-08-11). my_tid==blue_tid → team1, 아니면 team2.
    let my_side = if my_tid != u64::MAX && (my_tid as usize) == blue { 1 } else { 2 };
    // 피어리스 불가 챔프 = 서버 walk가 RMI.set_info에서 추출해 게시한 값(비어있으면 classic/미도달).
    let _ = scene;
    let locked = FEARLESS_LOCKED.lock().map(|g| g.clone()).unwrap_or_default();
    let body = format!(
        "{{\"active\":true,\"ts\":{},\"rule\":{},\"picks_per_team\":{},\"my_tid\":{},\"blue_tid\":{},\"my_side\":{},\
\"team1\":{{\"bans\":{},\"picks\":{}}},\"team2\":{{\"bans\":{},\"picks\":{}}},\"locked\":{}}}",
        now_ms(), rule, picks_per_team, my_tid_s, blue, my_side,
        json_arr(&b1), json_arr(&p1), json_arr(&b2), json_arr(&p2), json_arr(&locked)
    );
    write_banpick_out(&body);
}

// ── UI 스레드: 완주 감지 → 파일 flush → 슬롯 해제 ─────────────────────────
const IDLE_FRAMES: u32 = 90; // post_update ~90프레임(≈1.5초) 틱 정지 = 완주

fn flush_slot(slot: usize) {
    let key = SL_SEED[slot].load(Ordering::Acquire); // game_ptr
    let seed = SL_REALSEED[slot].load(Ordering::Relaxed); // 파일명용 실제 seed
    let n = SL_N[slot].load(Ordering::Acquire) as usize;
    if key == 0 {
        return;
    }
    if n >= 3 {
        // 3샘플(90틱) 미만 = 쓰레기(초기화 직후 중단) — 파일 안 남김
        let Some(mut dirp) = mod_dir() else { return };
        dirp.push("flow");
        dirp.push(format!("{:016x}", seed)); // ★같은 경기(seed) = 한 폴더 (원본+주입변형 묶음)
        let _ = fs::create_dir_all(&dirp);
        let nr = SL_ROSTER_OK[slot].load(Ordering::Relaxed) as usize;
        let cen = CE_N[slot].load(Ordering::Acquire) as usize;
        let mut out = String::with_capacity(n * 200 + 1024);
        out.push_str(&format!(
            "# tfm2_flow_capture v0.16 seed=0x{:x} samples={} kills={} obj={} ce={} captured_t0_ms={} game=0.5.5\n",
            seed,
            n,
            K_N[slot].load(Ordering::Relaxed),
            O_N[slot].load(Ordering::Relaxed),
            cen,
            SL_T0MS[slot].load(Ordering::Relaxed)
        ));
        out.push_str(concat!(
            "# T,team0_name,team1_name — 팀 이름(SDK db.team.name. 상대팀 미상시 '상대')\n",
            "# R,idx,athlete_id,team,champion,player_name  (골드·딜·탱 컬럼순. champion=브릿지키, player_name=SDK)\n",
            "# E,idx,team,champion  (좌표/HP/레벨 컬럼 순서)\n",
            "# S,tick,score0,score1,gold_x10(R순),(x,y,hp,lv)_x10(E순, 셀=32000),deal_x10(R순),tank_x10(R순, 누적),min_x8(미니언 카운트: t0[레인0,1,2,기타],t1[…]),wave중심점_x12((cx,cy)×6버킷=t0[레인0,1,2],t1[…]),jmask(정글캠프 생존 비트마스크: bit i=J라인 i번째 캠프)\n",
            "# K,tick,killer_team,killer_role,killed_role,assist_n,assist_roles_x4(4294967295=없음) — role: 0Top 1Jg 2Mid 3Bot 4Sup\n",
            "# O,tick,team,type(tower/morgard/serpen),detail(타워라인) — 오브젝트 처치/파괴\n",
            "# J,kind(4정글/9곰),team,x,y,name — 중립 몹 스폰 좌표(캠프 위치 실측)\n",
            "# P,team,b0..b23 — 팀 전술 Strategy 24B(game+0xb248). 12필드 disc는 뷰어서 디코드\n",
            "# U,user_team,user_aids(;구분) — 유저 팀(피드백 대상). 자동인식 MY_ATH 기준\n",
            "# M,set_no,series_user_wins,series_opp_wins — 세트 번호+시리즈 스코어(서버 running_matches)\n"
        ));
        // ★U 라인: MY_ATH(자동인식 선발)이 어느 팀에 있는지 판정 → 뷰어/피드백의 "아군" 기준.
        //   같은 세트를 team0/team1 중 유저가 어느 쪽인지 파일 자체에 박아 자립화(cfg·조인 불요).
        {
            let man = MY_ATH_N.load(Ordering::Relaxed) as usize;
            if man > 0 {
                let mut cnt = [0u32; 2];
                let mut uaids: Vec<u64> = Vec::new();
                for i in 0..nr.min(NATH) {
                    let aid = SL_AID[slot][i].load(Ordering::Relaxed);
                    for k in 0..man.min(8) {
                        if MY_ATH[k].load(Ordering::Relaxed) == aid {
                            let tm = SL_TEAM[slot][i].load(Ordering::Relaxed);
                            if (tm as usize) < 2 { cnt[tm as usize] += 1; }
                            uaids.push(aid);
                            break;
                        }
                    }
                }
                // ★uaids가 비면(이 game에 내 선수 없음) U/T라인을 안 쓴다 — 배경 리그 sim이
                //   MY_ATH 전역 때문에 "내 경기"로 위장되던 누출 차단(빈 U라인 방지).
                if !uaids.is_empty() {
                let uteam = if cnt[1] > cnt[0] { 1 } else { 0 };
                let aids_s: Vec<String> = uaids.iter().map(|a| a.to_string()).collect();
                out.push_str(&format!("U,{},{}\n", uteam, aids_s.join(";")));
                // T 라인: 유저 쪽 = SDK 팀명(정확). 반대 쪽 = ★seed별 상대팀명(조인표에서 내 팀 아닌 쪽).
                //   OPP_TNAME 전역은 첫 상대 하나로 고정돼 부정확 → seed→(blue,red) 조인으로 경기별 해석.
                let raw_utn = USER_TNAME.lock().map(|g| g.clone()).unwrap_or_default();
                let mut opp = String::new();
                if !raw_utn.trim().is_empty() {
                    if let Ok(st) = SEED_TEAMS.lock() {
                        for (s, b, r) in st.iter() {
                            if *s == seed {
                                if *b == raw_utn { opp = r.clone(); }
                                else if *r == raw_utn { opp = b.clone(); }
                                break;
                            }
                        }
                    }
                }
                if opp.trim().is_empty() {
                    // ★라이브 경기(seed가 아직 replay에 없음): 상대측 선수(SL_TEAM=1-uteam) aid를
                    //   현재 로스터 역조회표(AID_TEAM)로 매칭 → 경기별 정확한 상대팀명.
                    if let Ok(at) = AID_TEAM.lock() {
                        'op: for i in 0..nr.min(NATH) {
                            if SL_TEAM[slot][i].load(Ordering::Relaxed) as i32 != 1 - uteam {
                                continue;
                            }
                            let aid = SL_AID[slot][i].load(Ordering::Relaxed);
                            for (a, name) in at.iter() {
                                if *a == aid && !name.is_empty() {
                                    opp = name.clone();
                                    break 'op;
                                }
                            }
                        }
                    }
                }
                if opp.trim().is_empty() {
                    // 최종 폴백: 전역 OPP_TNAME(둘 다 실패 시)
                    opp = OPP_TNAME.lock().map(|g| g.clone()).unwrap_or_default();
                }
                let utn = if raw_utn.trim().is_empty() { "아군".to_string() } else { raw_utn.replace(',', " ") };
                let otn = if opp.trim().is_empty() { "상대".to_string() } else { opp.replace(',', " ") };
                let (t0n, t1n) = if uteam == 0 { (utn, otn) } else { (otn, utn) };
                out.push_str(&format!("T,{},{}\n", t0n, t1n));
                }
            }
        }
        // M 라인: 세트 번호 + 시리즈 스코어(유저:상대). 서버가 running_matches서 게시(로스터 시 스냅샷).
        {
            let sn = SL_SETNO[slot].load(Ordering::Relaxed).max(1);
            let su = SL_SERIES_U[slot].load(Ordering::Relaxed);
            let so = SL_SERIES_O[slot].load(Ordering::Relaxed);
            out.push_str(&format!("M,{},{},{}\n", sn, su, so));
        }
        // 선수 이름 맵 1회 잠금(R 루프 전) — aid→이름
        let pmap = PLAYER_NAMES.lock().ok().map(|g| g.clone()).unwrap_or_default();
        for i in 0..nr.min(NATH) {
            // athlete 챔피언명 디코드(R↔E 브릿지 키). deal/tank(R순)↔pos(E순) 매칭용.
            let mut cb = [0u8; 16];
            cb[0..8].copy_from_slice(&SL_ACHAMP[slot][i * 2].load(Ordering::Relaxed).to_le_bytes());
            cb[8..16].copy_from_slice(&SL_ACHAMP[slot][i * 2 + 1].load(Ordering::Relaxed).to_le_bytes());
            let champ: String = cb.iter().take_while(|&&b| b != 0)
                .map(|&b| if b.is_ascii_graphic() { b as char } else { '?' })
                .collect();
            let aid = SL_AID[slot][i].load(Ordering::Relaxed);
            let pname = pmap.get(&aid).map(|s| s.replace(',', " ")).unwrap_or_default();
            out.push_str(&format!(
                "R,{},{},{},{},{}\n",
                i,
                aid,
                SL_TEAM[slot][i].load(Ordering::Relaxed),
                champ,
                pname
            ));
        }
        for j in 0..cen.min(NATH) {
            let mut nb = [0u8; 16];
            nb[0..8].copy_from_slice(&CE_NAME[slot][j * 2].load(Ordering::Relaxed).to_le_bytes());
            nb[8..16].copy_from_slice(&CE_NAME[slot][j * 2 + 1].load(Ordering::Relaxed).to_le_bytes());
            let name: String = nb.iter().take_while(|&&b| b != 0)
                .map(|&b| if b.is_ascii_graphic() { b as char } else { '?' })
                .collect();
            out.push_str(&format!(
                "E,{},{},{}\n",
                j,
                CE_TEAMS[slot][j].load(Ordering::Relaxed),
                name
            ));
        }
        for k in 0..n {
            out.push_str(&format!(
                "S,{},{},{}",
                S_TICK[slot][k].load(Ordering::Relaxed),
                S_SC0[slot][k].load(Ordering::Relaxed),
                S_SC1[slot][k].load(Ordering::Relaxed)
            ));
            for i in 0..NATH {
                out.push_str(&format!(",{}", S_GOLD[slot][k][i].load(Ordering::Relaxed)));
            }
            for i in 0..NATH {
                out.push_str(&format!(
                    ",{},{},{},{}",
                    S_X[slot][k][i].load(Ordering::Relaxed),
                    S_Y[slot][k][i].load(Ordering::Relaxed),
                    S_HP[slot][k][i].load(Ordering::Relaxed),
                    S_LV[slot][k][i].load(Ordering::Relaxed)
                ));
            }
            // 누적 딜량×10(R순), 이어서 누적 탱킹×10(R순). 뒤에 append=구 파서 무해.
            for i in 0..NATH {
                out.push_str(&format!(",{}", S_DEAL[slot][k][i].load(Ordering::Relaxed)));
            }
            for i in 0..NATH {
                out.push_str(&format!(",{}", S_TANK[slot][k][i].load(Ordering::Relaxed)));
            }
            // 미니언 카운트 2팀×4버킷(t0[레인0,1,2,기타], t1[…]). 뒤에 append=구 파서 무해.
            for c in 0..NMINCOL {
                out.push_str(&format!(",{}", S_MIN[slot][k][c].load(Ordering::Relaxed)));
            }
            // 웨이브 중심점 6버킷(cx,cy) = 좌표합/카운트. 이어서 정글 생존 비트마스크. (뷰어 렌더)
            for c in 0..6usize {
                let cnt = S_MIN[slot][k][(c / 3) * 4 + (c % 3)].load(Ordering::Relaxed);
                let (cx, cy) = if cnt > 0 {
                    (S_MINX[slot][k][c].load(Ordering::Relaxed) / cnt, S_MINY[slot][k][c].load(Ordering::Relaxed) / cnt)
                } else {
                    (0, 0)
                };
                out.push_str(&format!(",{},{}", cx, cy));
            }
            out.push_str(&format!(",{}", S_JMASK[slot][k].load(Ordering::Relaxed)));
            out.push('\n');
        }
        let kn = K_N[slot].load(Ordering::Acquire) as usize;
        for k in 0..kn.min(KMAX) {
            let tick = K_RAW[slot][k][0].load(Ordering::Relaxed) as u64
                | (K_RAW[slot][k][1].load(Ordering::Relaxed) as u64) << 32;
            out.push_str(&format!("K,{}", tick));
            for w in 2..10 {
                out.push_str(&format!(",{}", K_RAW[slot][k][w].load(Ordering::Relaxed)));
            }
            out.push('\n');
        }
        // 오브젝트 처치 이벤트: O,tick,team,type,detail
        let on = O_N[slot].load(Ordering::Acquire) as usize;
        for k in 0..on.min(OMAX) {
            let ty = match O_TYPE[slot][k].load(Ordering::Relaxed) {
                0 => "tower",
                1 => "morgard",
                2 => "serpen",
                _ => "?",
            };
            let line = O_LINE[slot][k].load(Ordering::Relaxed);
            let detail = if line < 5 {
                ["탑", "정글", "미드", "바텀", "서폿"][line as usize]
            } else {
                ""
            };
            out.push_str(&format!(
                "O,{},{},{},{}\n",
                O_TICK[slot][k].load(Ordering::Relaxed),
                O_TEAM[slot][k].load(Ordering::Relaxed),
                ty,
                detail
            ));
        }
        // 팀 전술: P,team,b0..b23 (Strategy 24B 원바이트). 뷰어가 12필드 디코드.
        if SL_STRAT_OK[slot].load(Ordering::Relaxed) != 0 {
            for tm in 0..2 {
                let mut bs: Vec<String> = Vec::with_capacity(24);
                for b in 0..24 {
                    bs.push(SL_STRAT[slot][tm * 24 + b].load(Ordering::Relaxed).to_string());
                }
                out.push_str(&format!("P,{},{}\n", tm, bs.join(",")));
            }
        }
        // 중립 몹(정글/곰) 위치: J,kind(4정글/9곰),team,x,y,name — 캠프 좌표 실측
        let nn = SL_NEUT_N[slot].load(Ordering::Relaxed) as usize;
        for k in 0..nn.min(MAXNEUT) {
            let mut nb = [0u8; 16];
            nb[0..8].copy_from_slice(&SL_NNAME[slot][k * 2].load(Ordering::Relaxed).to_le_bytes());
            nb[8..16].copy_from_slice(&SL_NNAME[slot][k * 2 + 1].load(Ordering::Relaxed).to_le_bytes());
            let name: String = nb.iter().take_while(|&&b| b != 0)
                .map(|&b| if b.is_ascii_graphic() { b as char } else { '?' })
                .collect();
            out.push_str(&format!(
                "J,{},{},{},{},{}\n",
                SL_NKIND[slot][k].load(Ordering::Relaxed),
                SL_NTEAM[slot][k].load(Ordering::Relaxed),
                SL_NX[slot][k].load(Ordering::Relaxed),
                SL_NY[slot][k].load(Ordering::Relaxed),
                name
            ));
        }
        // ★세트당 1파일(2026-08-10): 같은 seed 로 게임 객체가 여러 개(화면출력용·결과확인용)
        //   동시에 돌아 세트 종료 시 슬롯마다 중복 flush 된다(내용 동일). 같은 seed 파일이
        //   이미 있으면 더 완전한(바이트 큰) 것 **하나만** 남긴다 — 분석기 중복처리 방지.
        // 오버라이드 주입 재생은 원본과 별도 저장(변형끼리만 중복제거) — 비교용.
        let ovr = SL_OVR[slot].load(Ordering::Relaxed) == 1;
        let prefix = format!("f_{:016x}_", seed);
        let mut skip = false;
        let mut dups: Vec<PathBuf> = Vec::new();
        if let Ok(rd) = fs::read_dir(&dirp) {
            for e in rd.flatten() {
                let fname = e.file_name().to_string_lossy().to_string();
                if !fname.starts_with(&prefix) {
                    continue;
                }
                if fname.contains("_ovr") != ovr {
                    continue; // 원본 vs 주입은 서로 중복제거 안 함
                }
                let sz = e.metadata().map(|m| m.len()).unwrap_or(0);
                if sz >= out.len() as u64 {
                    skip = true;
                } else {
                    dups.push(e.path());
                }
            }
        }
        if !skip {
            for d in dups {
                let _ = fs::remove_file(&d);
            }
            let mut p = dirp.clone();
            p.push(format!("{}{}{}.txt", prefix, if ovr { "ovr" } else { "" }, now_ms()));
            if fs::write(&p, &out).is_ok() {
                FILES_WRITTEN.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
    // 슬롯 해제 (seed=0 이 마지막 — 핫패스가 재획득 가능해짐)
    SL_N[slot].store(0, Ordering::Relaxed);
    SL_IDLE[slot].store(0, Ordering::Relaxed);
    SL_UILAST[slot].store(0, Ordering::Relaxed);
    SL_SEED[slot].store(0, Ordering::Release);
}

fn ui_scan() {
    for i in 0..SLOTS {
        if SL_SEED[i].load(Ordering::Relaxed) == 0 {
            continue;
        }
        let t = SL_LASTTICK[i].load(Ordering::Relaxed);
        if t != SL_UILAST[i].swap(t, Ordering::Relaxed) {
            SL_IDLE[i].store(0, Ordering::Relaxed);
            continue;
        }
        if SL_IDLE[i].fetch_add(1, Ordering::Relaxed) + 1 >= IDLE_FRAMES {
            flush_slot(i);
        }
    }
}

// 자동정리: 배경 리그 sim이 대량 파일을 만들어 유저 경기를 밀어내는 것 방지.
//   상한 2000(하루치 여유) + 초과 시 **작은 파일(미완주 배경 sim)부터** 삭제해 완주 경기 보존.
const PRUNE_CAP: usize = 2000;
const PRUNE_TRIGGER: usize = 2500;
fn prune_flow_dir() {
    let Some(mut p) = mod_dir() else { return };
    p.push("flow");
    let Ok(rd) = fs::read_dir(&p) else { return };
    // flow/<seed>/*.txt — 하위폴더 순회.
    let mut files: Vec<(PathBuf, u64)> = Vec::new();
    for sub in rd.flatten() {
        let sp = sub.path();
        if sp.is_dir() {
            if let Ok(rd2) = fs::read_dir(&sp) {
                for e in rd2.flatten() {
                    let pp = e.path();
                    let sz = fs::metadata(&pp).map(|m| m.len()).unwrap_or(0);
                    files.push((pp, sz));
                }
            }
        }
    }
    if files.len() <= PRUNE_TRIGGER {
        return;
    }
    files.sort_by_key(|(_, sz)| *sz); // 작은 것 먼저 = 완주(큰) 경기 보존
    let drop = files.len() - PRUNE_CAP;
    for (f, _) in files.iter().take(drop) {
        let _ = fs::remove_file(f);
    }
}

fn write_status() {
    let Some(mut p) = mod_dir() else { return };
    p.push("flow_status.txt");
    let inst = INSTALLED.load(Ordering::Relaxed);
    let msg = INSTALL_MSG.lock().map(|g| g.clone()).unwrap_or_default();
    let mut active = String::new();
    for i in 0..SLOTS {
        let s = SL_SEED[i].load(Ordering::Relaxed);
        if s != 0 {
            active.push_str(&format!(
                " [{}] seed=0x{:x} n={} tick={}\n",
                i,
                s,
                SL_N[i].load(Ordering::Relaxed),
                SL_LASTTICK[i].load(Ordering::Relaxed)
            ));
        }
    }
    // 자동 인식된 내 팀 로스터(MY_ATH) — 필터가 이 aid 3명 이상 든 game만 캡처.
    let man = MY_ATH_N.load(Ordering::Relaxed) as usize;
    let mut myteam = String::new();
    for k in 0..man.min(8) {
        let a = MY_ATH[k].load(Ordering::Relaxed);
        if a != 0 {
            myteam.push_str(&format!("{} ", a));
        }
    }
    let s = format!(
        "tfm2_flow_capture v0.7 상태 [{}ms]\n훅 설치: {} ({})\n내 팀 인식(MY_ATH): n={} src={} [{}]\nAID_TEAM 로스터맵: {}명\n화면경기: WATCHED={:#x} LIVE_SEED={:#x} 판정발화={}\nrun_tick 발화: {}\n파일 저장: {} · 슬롯만석 드롭: {}\n활성 슬롯:\n{}",
        now_ms(),
        match inst {
            1 => "OK",
            2 => "FAIL",
            _ => "대기",
        },
        msg,
        man,
        match MY_ATH_SRC.load(Ordering::Relaxed) {
            1 => "SDK자동",
            2 => "cfg폴백",
            3 => "파일복원",
            _ => "미확정",
        },
        myteam.trim(),
        AID_TEAM.lock().map(|m| m.len()).unwrap_or(0),
        LAST_WATCHED_SEED.load(Ordering::Relaxed),
        LIVE_SEED.load(Ordering::Relaxed),
        LIVE_SEED_N.load(Ordering::Relaxed),
        HITS.load(Ordering::Relaxed),
        FILES_WRITTEN.load(Ordering::Relaxed),
        DROPPED_FULL.load(Ordering::Relaxed),
        active
    );
    let _ = fs::write(p, s);
}

// ── SDK 라이프사이클 ───────────────────────────────────────────────────────
struct FlowCaptureExt;

impl ModExtension for FlowCaptureExt {
    fn post_update(&self, scene: &mut Scene, ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        let f = FRAMES.fetch_add(1, Ordering::Relaxed);
        // 메인(클라) 스레드 id 게시 — reco detour가 배경 sim(rayon) 발화를 걸러내는 기준.
        if f == 0 {
            MAIN_TID.store(unsafe { GetCurrentThreadId() }, Ordering::Relaxed);
            load_my_team(); // 저장된 유저 팀 복원(있으면) — 리플레이만 보는 세션도 배경 sim 필터.
        }
        // 늦은 설치(120프레임 ≈ 2초): 부분초기화 상태 회피 + 타 모드(comptest 등) 훅이 먼저
        // 설치되게 해 체인 순서 안정화. 1회 설치 확정 — 재검증/재체인 금지.
        if f == 120 && INSTALLED.load(Ordering::Relaxed) == 0 {
            let r = unsafe { install_tick_hook() };
            match r {
                Ok(m) => {
                    INSTALLED.store(1, Ordering::Relaxed);
                    if let Ok(mut g) = INSTALL_MSG.lock() {
                        *g = m.clone();
                    }
                    log(&format!("install {}", m));
                }
                Err(m) => {
                    INSTALLED.store(2, Ordering::Relaxed);
                    if let Ok(mut g) = INSTALL_MSG.lock() {
                        *g = m.clone();
                    }
                    log(&format!("install FAIL {}", m));
                }
            }
        }
        // 밴픽 씬 훅 설치(독립·1회) — draft 실시간 탐지용
        if f == 120 && BP_INSTALLED.load(Ordering::Relaxed) == 0 {
            match unsafe { install_scene_hook() } {
                Ok(m) => {
                    BP_INSTALLED.store(1, Ordering::Relaxed);
                    log(&format!("bp install {}", m));
                }
                Err(m) => {
                    BP_INSTALLED.store(2, Ordering::Relaxed);
                    log(&format!("bp install FAIL {}", m));
                }
            }
        }
        // (피어리스 불가목록 = contains 훅 폐기, 씬 고정오프셋 직접읽기로 대체 — RE 2026-08-11)
        // 게임 전술 추천 훅 설치(독립·1회) — recommend_strategy_inner
        if f == 120 && RECO_INSTALLED.load(Ordering::Relaxed) == 0 {
            match unsafe { install_reco_hook() } {
                Ok(m) => {
                    RECO_INSTALLED.store(1, Ordering::Relaxed);
                    log(&format!("reco install {}", m));
                }
                Err(m) => {
                    RECO_INSTALLED.store(2, Ordering::Relaxed);
                    log(&format!("reco install FAIL {}", m));
                }
            }
        }
        // 다시보기 편집 UI 로더 훅 설치(독립·1회, 체인). 다른 UI모드(comptest) 뒤에 얹힘.
        if f == 120 {
            let ok = unsafe { ui_replay::install() };
            log(&format!("ui_replay install ok={}", ok));
        }
        // 생성자 호출자 RVA 캡처 훅(Step1, 독립·1회) — 리플레이 런처 특정용.
        if f == 120 && CTOR_INSTALLED.load(Ordering::Relaxed) == 0 {
            match unsafe { install_ctor_hook() } {
                Ok(m) => {
                    CTOR_INSTALLED.store(1, Ordering::Relaxed);
                    log(&format!("ctor {}", m));
                }
                Err(m) => {
                    CTOR_INSTALLED.store(2, Ordering::Relaxed);
                    log(&format!("ctor FAIL {}", m));
                }
            }
        }
        if f % 120 == 100 {
            write_ctor_debug();
        }
        // 다시보기 UI 진단 파일 flush(2초마다) — 경로/컨테이너/주입 상태.
        if f % 120 == 90 {
            ui_replay::write_diag();
        }
        // 다시보기 편집 패널 pump(매프레임) — 클릭 라우트 등록 + 상태 반영.
        ui_replay::pump(ui);
        // 전술 추천 캡처됨(위임버튼 등, 메인 스레드) → game_reco 기록.
        let rseq = RECO_SEQ.load(Ordering::Relaxed);
        if rseq != RECO_LAST_SEQ.swap(rseq, Ordering::Relaxed) {
            write_game_reco();
        }
        if f % 120 == 60 {
            write_reco_debug(); // 진단
        }
        // 밴픽 실시간: scene_step 최근 발화(BP_HITS 증가)했으면 draft 중 → 상태 기록.
        //   멈추면(draft 종료) inactive 1회 기록. 15프레임마다 판정(과다 write 방지).
        if f % 15 == 7 {
            let h = BP_HITS.load(Ordering::Relaxed);
            let last = BP_LAST_HITS.swap(h, Ordering::Relaxed);
            if h > last {
                write_banpick_json(true);
                BP_STALE_WRITTEN.store(false, Ordering::Relaxed);
            } else if !BP_STALE_WRITTEN.swap(true, Ordering::Relaxed) {
                write_banpick_json(false);
            }
        }
        // ★유저 팀 자동 인식 (cfg 완전 제거): InGame일 때 db.player_team_id() → team.last_starting.
        //   격자: 초반 자주(60·120·…) + 이후 저빈도(30초). pid 변동/이적 자동 반영.
        if f < 600 || f % 1800 == 1799 {
            collect_my_team(scene);
        }
        // 선수 이름 해석(SDK db.athlete) — 캡처된 로스터(SL_AID)의 aid를 이름으로. 클라 스레드서 안전.
        if f < 600 || f % 600 == 599 {
            resolve_player_names(scene);
        }
        // 다시보기 편집기: 리플레이 파라미터 덤프(30초마다) + 오버라이드 파일 읽기(1초).
        if f % 1800 == 900 {
            dump_replays(scene);
        }
        if f % 60 == 30 {
            read_override();
            if let Some(mut p) = mod_dir() {
                p.push("inject_debug.txt");
                let _ = fs::write(&p, format!(
                    "override_on={} override_seed={} inject_fired={}\n",
                    OVERRIDE_ON.load(Ordering::Relaxed),
                    OVERRIDE_SEED.load(Ordering::Relaxed),
                    INJECT_FIRED.load(Ordering::Relaxed)
                ));
            }
        }
        ui_scan();
        if f % 300 == 299 {
            write_status();
        }
        if f % 18000 == 17999 {
            prune_flow_dir();
        }
    }
}

// ★유저 팀 선발 로스터 자동 인식 (item_tactics와 동일 경로).
//   Scene::InGame일 때 db.player_team_id()로 내 팀 id를 얻고, db.team(pid).last_starting
//   (선발 5명 athlete_id)을 MY_ATH에 게시 → 캡처 필터(count_my_ath>=3)가 배경 리그를 걸러냄.
//   ⚠ pid=0 함정: 조합테스트/비경기 컨텍스트서 player_team_id()=0 → team(0).last_starting=
//     [0,1,2,3,4](배경 sim aid)를 내 팀으로 오게시할 수 있다. 방어:
//       ① pid 유효범위(1~9999)만 사용, u64::MAX(미확정) 제외.
//       ② aid는 실제값(>10)만 수집 → [0,1,2,3,4] 같은 센티널 로스터는 자연히 걸러짐.
//       ③ 유효 로스터(>=3명) 확보 시에만 store, 실패 시 마지막 good 유지(후퇴 금지).
fn collect_my_team(scene: &Scene) {
    let Scene::InGame { data } = scene else { return };
    let db = data.db();
    let pid = db.player_team_id();
    // 유효 pid만: 0은 미확정/조합테스트 오염원이라 배제, u64::MAX(-1)도 배제.
    if pid == 0 || (pid as u64) >= 10000 {
        return;
    }
    PLAYER_TID.store(pid as u64, Ordering::Relaxed); // 서버 확장이 running_matches 매칭에 사용
    let Some(team) = db.team(pid) else { return };
    // 유저 팀 이름(SDK) — 뷰어 라벨용. 1회 채움.
    if let Ok(mut g) = USER_TNAME.lock() {
        if g.is_empty() && !team.name.is_empty() {
            *g = team.name.clone();
        }
    }
    // (AID_TEAM 역조회표 채우기는 dump_replays로 이동 — 30초 스로틀·pid 무관·db 정착 상태.
    //  collect_my_team은 f<600 매 프레임 도라 여기서 db 전체 순회는 낭비/위험.)
    // 상대 팀 이름: 리플레이 매치들 중 유저팀(pid)이 든 매치의 반대편 팀 → 이름. 1회 채움.
    if OPP_TNAME.lock().map(|g| g.is_empty()).unwrap_or(false) {
        for rid in db.match_replay_ids() {
            let Some(mi) = db.match_info(mod_api::MatchType::Normal { match_id: rid }) else { continue };
            let t1 = if let game_core::MatchTeamType::Normal(x) = mi.team1 { x } else { continue };
            let t2 = if let game_core::MatchTeamType::Normal(x) = mi.team2 { x } else { continue };
            if t1 == pid || t2 == pid {
                let opp = if t1 == pid { t2 } else { t1 };
                if let Some(ot) = db.team(opp) {
                    if !ot.name.is_empty() {
                        if let Ok(mut g) = OPP_TNAME.lock() { *g = ot.name.clone(); }
                    }
                }
                break;
            }
        }
    }
    let mut ids: Vec<u64> = Vec::new();
    for slot in team.last_starting.iter() {
        if let Some(aid) = slot {
            let a = *aid as u64;
            // 실제 athlete_id만(>10) — team(0)의 [0,1,2,3,4] 센티널 배제.
            if a > 10 && ids.len() < 8 && !ids.contains(&a) {
                ids.push(a);
            }
        }
    }
    // 유효 로스터(3명 이상) 확보 시에만 갱신. 그 미만이면 마지막 good 유지.
    if ids.len() >= 3 {
        for k in 0..8 {
            MY_ATH[k].store(ids.get(k).copied().unwrap_or(0), Ordering::Relaxed);
        }
        MY_ATH_N.store(ids.len() as u64, Ordering::Relaxed);
        MY_ATH_SRC.store(1, Ordering::Relaxed); // SDK 자동 인식
        // ★영속화: 감지된 팀을 파일에 저장 → 다음 세션(리플레이만 봐도) 팀 필터 유지(하드코딩 0).
        if let Some(mut p) = mod_dir() {
            p.push("my_team.dat");
            let tn = USER_TNAME.lock().map(|g| g.clone()).unwrap_or_default();
            let aids_s: Vec<String> = ids.iter().map(|a| a.to_string()).collect();
            let _ = fs::write(&p, format!("{}\n{}\n", tn.replace('\n', " "), aids_s.join(";")));
        }
    }
}

// 저장된 유저 팀(my_team.dat) 로드 → MY_ATH 미설정 시 복원(리플레이만 보는 세션 대비).
//   자기 커리어 경기를 한 번이라도 뜨면 collect_my_team이 파일을 남기고, 이후 세션은 이걸로 필터.
fn load_my_team() {
    if MY_ATH_N.load(Ordering::Relaxed) > 0 {
        return; // 이미 라이브 감지됨(더 신선)
    }
    let Some(mut p) = mod_dir() else { return };
    p.push("my_team.dat");
    let Ok(txt) = fs::read_to_string(&p) else { return };
    let mut lines = txt.lines();
    let tn = lines.next().unwrap_or("").trim().to_string();
    let ids: Vec<u64> = lines
        .next()
        .unwrap_or("")
        .split(';')
        .filter_map(|s| s.trim().parse::<u64>().ok())
        .filter(|&a| a > 10)
        .take(8)
        .collect();
    if ids.len() >= 3 {
        for k in 0..8 {
            MY_ATH[k].store(ids.get(k).copied().unwrap_or(0), Ordering::Relaxed);
        }
        MY_ATH_N.store(ids.len() as u64, Ordering::Relaxed);
        MY_ATH_SRC.store(3, Ordering::Relaxed); // 3=파일 복원
        if !tn.is_empty() {
            if let Ok(mut g) = USER_TNAME.lock() {
                if g.is_empty() {
                    *g = tn;
                }
            }
        }
    }
}

// 캡처된 로스터(SL_AID)의 athlete_id를 SDK db.athlete(id).name으로 해석해 PLAYER_NAMES에 채움.
//   post_update(클라 스레드)서만 호출 — SDK는 안전. flush(sim 스레드)는 이 맵을 읽기만.
fn resolve_player_names(scene: &Scene) {
    let Scene::InGame { data } = scene else { return };
    let db = data.db();
    let Ok(mut map) = PLAYER_NAMES.lock() else { return };
    for s in 0..SLOTS {
        if SL_ROSTER_OK[s].load(Ordering::Relaxed) == 0 {
            continue;
        }
        for i in 0..NATH {
            let aid = SL_AID[s][i].load(Ordering::Relaxed);
            if aid == 0 || aid >= 1_000_000 || map.contains_key(&aid) {
                continue;
            }
            if let Some(a) = db.athlete(aid as usize) {
                if !a.name.is_empty() {
                    map.insert(aid, a.name.clone());
                }
            }
        }
    }
}

// ── 다시보기 편집기 step1: 리플레이 파라미터 덤프(SDK) → replay_params.json ──
fn js_esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    o.push('"');
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' | '\r' => o.push(' '),
            _ => o.push(c),
        }
    }
    o.push('"');
    o
}
// Debug 덤프에서 "label: 12345" 숫자 추출.
fn grab_num(dbg: &str, label: &str) -> String {
    if let Some(i) = dbg.find(label) {
        let rest = &dbg[i + label.len()..];
        let t: String = rest.trim_start().chars().take_while(|c| c.is_ascii_digit()).collect();
        if !t.is_empty() {
            return t;
        }
    }
    "0".into()
}
// Debug 덤프에서 "label: Foo { ... }"의 {…} 균형블록 추출(전술 원본).
fn grab_block(dbg: &str, label: &str) -> String {
    if let Some(i) = dbg.find(label) {
        let after = i + label.len();
        if let Some(b) = dbg[after..].find('{') {
            let start = after + b;
            let bytes = dbg.as_bytes();
            let mut depth = 0i32;
            let mut j = start;
            while j < dbg.len() {
                match bytes[j] {
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            return dbg[start..=j].to_string();
                        }
                    }
                    _ => {}
                }
                j += 1;
            }
        }
    }
    String::new()
}
fn dump_replays(scene: &Scene) {
    let Scene::InGame { data } = scene else { return };
    let db = data.db();
    ui_replay::clear_replays();
    // ★seed→양팀명 조인표 재구축(flush의 per-game 상대팀 해석용). 매번 새로 채움.
    if let Ok(mut st) = SEED_TEAMS.lock() { st.clear(); }
    // ★aid→팀명 역조회표 재구축(라이브/관전 경기 상대팀 해석용) — pid 무관, 30초 스로틀.
    //   dump_replays는 이미 db를 무겁게(11011 리플레이) 순회하며 행 없이 도는 게 실증됨 → db 안전지대.
    //   각 팀(db.team_ids) 선발(last_starting) aid를 팀명에 매핑. flush가 상대측 선수 aid로 역조회.
    if let Ok(mut at) = AID_TEAM.lock() {
        at.clear();
        for tid in db.team_ids() {
            if let Some(t) = db.team(tid) {
                if t.name.is_empty() { continue; }
                for slot in t.last_starting.iter() {
                    if let Some(aid) = slot {
                        let a = *aid as u64;
                        if a > 10 { at.push((a, t.name.clone())); }
                    }
                }
            }
        }
    }
    let mut items: Vec<String> = Vec::new();
    let mut match_group: u64 = 0;
    for mid in db.match_replay_ids() {
        let Some(mi) = db.match_info(mod_api::MatchType::Normal { match_id: mid }) else { continue };
        match_group += 1; // 같은 경기의 세트들을 묶는 그룹 id(mid 타입 무관)
        for (set_idx, &rid) in mi.replays.iter().enumerate() {
            let Some(rep) = db.match_replays.get(&rid) else { continue };
            let dbg = format!("{:?}", rep);
            let seed = grab_num(&dbg, "seed:");
            let bstrat = grab_block(&dbg, "blue_strategy:");
            let rstrat = grab_block(&dbg, "red_strategy:");
            let mut blue = String::new();
            for a in rep.blue_team.iter() {
                if !blue.is_empty() {
                    blue.push(',');
                }
                let name = db.athletes.get(&a.athlete_id).map(|x| x.name.clone()).unwrap_or_default();
                blue.push_str(&format!(
                    "{{\"aid\":{},\"name\":{},\"pos\":\"{:?}\",\"champ\":{},\"items\":{:?}}}",
                    a.athlete_id, js_esc(&name), a.position, js_esc(&a.champion), a.items
                ));
            }
            let mut red = String::new();
            for a in rep.red_team.iter() {
                if !red.is_empty() {
                    red.push(',');
                }
                let name = db.athletes.get(&a.athlete_id).map(|x| x.name.clone()).unwrap_or_default();
                red.push_str(&format!(
                    "{{\"aid\":{},\"name\":{},\"pos\":\"{:?}\",\"champ\":{},\"items\":{:?}}}",
                    a.athlete_id, js_esc(&name), a.position, js_esc(&a.champion), a.items
                ));
            }
            let bt = db.teams.get(&rep.blue_team_id).map(|t| t.name.clone()).unwrap_or_default();
            let rt = db.teams.get(&rep.red_team_id).map(|t| t.name.clone()).unwrap_or_default();
            // 편집 패널용: seed → 파싱된 전술 등록(현재값 자동 로드).
            if let Ok(sd) = seed.parse::<u64>() {
                // ★flush의 per-game 상대팀 해석용 조인표에도 등록(seed→양팀명).
                if let Ok(mut st) = SEED_TEAMS.lock() { st.push((sd, bt.clone(), rt.clone())); }
                let blue_win = dbg
                    .split("blue_team_win:")
                    .nth(1)
                    .map(|s| s.trim_start().starts_with("true"))
                    .unwrap_or(false);
                ui_replay::ingest(sd, &bt, &rt, &bstrat, &rstrat, set_idx as u32, match_group, blue_win);
            }
            items.push(format!(
                "{{\"replay_id\":{},\"seed\":{},\"blue_team\":{},\"red_team\":{},\"blue_strategy\":{},\"red_strategy\":{},\"blue\":[{}],\"red\":[{}]}}",
                rid, seed, js_esc(&bt), js_esc(&rt), js_esc(&bstrat), js_esc(&rstrat), blue, red
            ));
        }
    }
    if let Some(mut p) = mod_dir() {
        p.push("replay_params.json");
        let _ = fs::write(&p, format!("[{}]", items.join(",")));
    }
}

// 다시보기 편집기 주입: replay_override.txt(on=/seed=/blue=/red= 24바이트 CSV) 읽어 정적에 게시.
fn read_override() {
    // 인게임 편집 패널이 override를 무장했으면 파일기반 읽기는 건너뜀(패널 값 보존).
    if OVERRIDE_FROM_PANEL.load(Ordering::Relaxed) {
        return;
    }
    let Some(mut p) = mod_dir() else { return };
    p.push("replay_override.txt");
    let Ok(txt) = fs::read_to_string(&p) else {
        OVERRIDE_ON.store(false, Ordering::Relaxed);
        return;
    };
    let (mut seed, mut on) = (0u64, false);
    let (mut blue, mut red) = ([0u8; 24], [0u8; 24]);
    let parse_bytes = |v: &str, out: &mut [u8; 24]| {
        for (i, t) in v.trim().split(',').enumerate() {
            if i < 24 {
                out[i] = t.trim().parse().unwrap_or(0);
            }
        }
    };
    for line in txt.lines() {
        let line = line.trim();
        if let Some(v) = line.strip_prefix("seed=") {
            seed = v.trim().parse().unwrap_or(0);
        } else if let Some(v) = line.strip_prefix("on=") {
            on = v.trim() == "1" || v.trim() == "true";
        } else if let Some(v) = line.strip_prefix("blue=") {
            parse_bytes(v, &mut blue);
        } else if let Some(v) = line.strip_prefix("red=") {
            parse_bytes(v, &mut red);
        }
    }
    for i in 0..3 {
        OVERRIDE_STRAT[i].store(u64::from_le_bytes(blue[i * 8..i * 8 + 8].try_into().unwrap()), Ordering::Relaxed);
        OVERRIDE_STRAT[3 + i].store(u64::from_le_bytes(red[i * 8..i * 8 + 8].try_into().unwrap()), Ordering::Relaxed);
    }
    OVERRIDE_SEED.store(seed, Ordering::Relaxed);
    OVERRIDE_ON.store(on && seed != 0, Ordering::Relaxed);
}
// 주입: 현재 game의 seed가 override seed와 같으면 game+0xb248[Strategy;2]를 오버라이드로 덮어씀(매틱).
#[inline]
unsafe fn inject_override(g: usize, seed: u64) {
    if !OVERRIDE_ON.load(Ordering::Relaxed) || seed != OVERRIDE_SEED.load(Ordering::Relaxed) {
        return;
    }
    if !readable(g.wrapping_add(STRAT_OFF), STRAT_STRIDE * 2) {
        return;
    }
    for i in 0..3 {
        core::ptr::write_unaligned((g + STRAT_OFF + i * 8) as *mut u64, OVERRIDE_STRAT[i].load(Ordering::Relaxed));
        core::ptr::write_unaligned((g + STRAT_OFF + STRAT_STRIDE + i * 8) as *mut u64, OVERRIDE_STRAT[3 + i].load(Ordering::Relaxed));
    }
    INJECT_FIRED.fetch_add(1, Ordering::Relaxed);
}

// (my_team.cfg 수동 선수id 지정 제거 2026-08-11 — SDK 자동 인식으로 완전 대체·유저 요청)

// ── 서버 확장: running_matches에서 유저 경기 세트 수/시리즈 스코어 읽기 ─────────
//   레시피 = setfeed_probe(RE 2026-08-10): server_state=ctx+0x18, running_matches@+0x2a0.
//   set_game_results.len(=완료 세트 수)이 검증된 값(setfeed_probe: 세트당 grow). ptr는 불확실 → 시리즈는 best-effort.
#[inline]
unsafe fn rd(addr: usize) -> Option<usize> {
    if readable(addr, 8) { Some(core::ptr::read_unaligned(addr as *const usize)) } else { None }
}
#[inline]
unsafe fn rdu8(a: usize) -> Option<u8> {
    if readable(a, 1) { Some(core::ptr::read_unaligned(a as *const u8)) } else { None }
}
// ⚠0.5.5 미확정 (ServerState/RunningMatchInfo 구조 = §6 World 대역 밖·별도 객체·정적 앵커 곤란).
//   walk_running_matches는 전 경로 readable()+power-of-2 bmask 가드 → 오프셋 stale 시 무데이터로 degrade(크래시 없음).
//   ⟹ 세트번호/시리즈스코어(부차 표시)만 영향. 런타임 setfeed 관측 또는 ghidra-re로 재확인 필요.
//   (0.5.4 값 유지 = 근거 없는 추정 시프트 금지 원칙.)
const RM_OFF: usize = 0x2a0; // ⚠미확정(0.5.4값)
const ENTRY_STRIDE: usize = 0x160; // ⚠미확정
const SGR_STRIDE: usize = 0xeea8; // ⚠미확정

unsafe fn walk_running_matches() {
    let ss = SERVER_STATE.load(Ordering::Relaxed);
    let tid = PLAYER_TID.load(Ordering::Relaxed);
    if ss == 0 || tid == u64::MAX || !readable(ss + RM_OFF, 0x30) {
        return;
    }
    let hm = ss + RM_OFF;
    let ctrl = match rd(hm) { Some(c) if c > 0x10000 => c, _ => return };
    let bmask = rd(hm + 0x8).unwrap_or(0);
    let items = rd(hm + 0x18).unwrap_or(0);
    if bmask == 0 || (bmask & (bmask + 1)) != 0 || bmask > 0xfffff {
        return;
    }
    let num_buckets = bmask + 1;
    if !readable(ctrl, num_buckets) {
        return;
    }
    let mut visited = 0usize;
    for i in 0..num_buckets.min(65536) {
        if visited >= items.min(64) {
            break;
        }
        let cb = match rdu8(ctrl + i) { Some(b) => b, None => break };
        if cb & 0x80 != 0 {
            continue;
        }
        visited += 1;
        let bucket = ctrl.wrapping_sub((i + 1) * ENTRY_STRIDE);
        if !readable(bucket, 0x158) {
            continue;
        }
        let rmi = bucket + 0x8;
        let t1 = rd(rmi + 0x140).unwrap_or(0) as u64;
        let t2 = rd(rmi + 0x148).unwrap_or(0) as u64;
        if t1 != tid && t2 != tid {
            continue; // 유저 경기만
        }
        let sgr_ptr = rd(rmi + 0x30).unwrap_or(0);
        let sgr_len = rd(rmi + 0x40).unwrap_or(0);
        if sgr_len > 20 {
            continue; // 비정상
        }
        MATCH_COMPLETED.store(sgr_len as u32, Ordering::Relaxed);
        // 시리즈 승 카운트(best-effort — ptr 불확실)
        let (mut uw, mut ow) = (0u32, 0u32);
        if sgr_ptr > 0x10000 {
            for s in 0..sgr_len {
                let elem = sgr_ptr.wrapping_add(s * SGR_STRIDE);
                if let Some(w) = rdu8(elem + 0xeea0) {
                    let is_t1_win = w == 1;
                    let user_is_t1 = t1 == tid;
                    if is_t1_win == user_is_t1 { uw += 1; } else { ow += 1; }
                }
            }
        }
        MATCH_SERIES_U.store(uw, Ordering::Relaxed);
        MATCH_SERIES_O.store(ow, Ordering::Relaxed);
        // ★피어리스 불가목록 = 이 RMI의 set_info[*] team1_picks/team2_picks 합집합.
        let fl = extract_fearless_rmi(rmi);
        if let Ok(mut g) = FEARLESS_LOCKED.lock() {
            *g = fl;
        }
        return;
    }
}

fn setno_poll_loop() {
    loop {
        std::thread::sleep(std::time::Duration::from_millis(2000));
        let _ = std::panic::catch_unwind(|| unsafe { walk_running_matches() });
    }
}

struct FlowServer;
impl ModServerExtension for FlowServer {
    fn on_server_start(&self, ctx: &mut ServerModContext) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            let ctx_addr = ctx as *const ServerModContext as usize;
            if let Some(ss) = rd(ctx_addr + 0x18) {
                SERVER_STATE.store(ss, Ordering::Relaxed);
            }
        }));
        if !SETNO_POLL.swap(true, Ordering::Relaxed) {
            std::thread::spawn(setno_poll_loop);
        }
    }
}
static SETNO_POLL: AtomicBool = AtomicBool::new(false);

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_server_extension(FlowServer);
    reg.set_extension(FlowCaptureExt);
    reg
}

declare_mod!(init);
