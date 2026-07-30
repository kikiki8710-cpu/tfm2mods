// tfm2_aim_lead.rs — 스킬샷 조준 "리드(이동예측)" 모드  [Phase 3: linear + line_range]
// =====================================================================================
// 게임 AI는 방향형 스킬샷을 적의 "현재 위치"에만 쏜다(리드 없음) → 움직이는 적을 빗나감.
// 이 모드는 발사 직전 조준점을 "적의 미래 위치(pos + vel*t_hit)"로 바꿔치기해 적중률을 올린다.
//
// ★스킬 종류마다 spawn 함수/경로가 다름(런타임 실측으로 확정):
//  ① LINEAR (gambler류 traveling 직진투사체) = FUN_1418df080 case[0] 미드훅 @0x18df118(14B).
//       rax=target, r15=caster, rcx=caster_x, r10=caster_y, rsi=projdef. r11=tx→led_x, r8=ty→led_y.
//  ② LINE_RANGE (magic_knight skill1 "일직선 마력폭발") = FUN_1420f9ab0 case[0] 미드훅 @0x20f9b68(14B).
//       rax=target, rcx=caster_x, rdx=caster_y, r13=projdef. r8=tx→led_x, r11=ty→led_y. 복귀 0x20f9b76.
//       (caster entity가 직전 mov r15,rcx로 덮여 없음 → 챔프게이트 불가 → 모든 line_range에 리드)
//  ③ SAMPLER = FUN_1418d0190 이동적용(per-entity per-tick), r8=entity. 매틱 위치→velocity 캐시.
//
// velocity(틱당)=이번틱pos−직전틱pos.  t_hit(틱)=caster→target거리/투사체speed.  단위 일치.
//
// cfg(mods\tfm2_aim_lead\tfm2_aim_lead.cfg, 경기 시작 전 설정):
//    log=1       진단 로그
//    aim_lead=0  0=원본동일(리드끔·샘플러idle) / 1=리드적용
//    mk_only=1   LINEAR 훅만 magic_knight 한정(LINE_RANGE 훅은 항상 적용=magic_knight 경로)
//    lead_pct=100 리드 강도 %% (0~400)
//
// 빌드: powershell -File C:\tfm2mods\build_inj.ps1 -Src C:\tfm2mods\tfm2_aim_lead\src\tfm2_aim_lead.rs -ModId tfm2_aim_lead
// =====================================================================================

use mod_api::*;
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU8, AtomicU64, AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "tfm2_aim_lead";

// ── ① LINEAR 후킹 (FUN_1418df080 case[0] target read) ──
const RVA_AIM_PATCH: usize = 0x18df118;
const AIM_ORIG_LEN: usize  = 14;
const AIM_EXPECT: [u8; 14] = [0x4C,0x8B,0x98,0x48,0x06,0x00,0x00, 0x4C,0x8B,0x80,0x50,0x06,0x00,0x00];
// AIM: r11=[rax+648]=tx, r8=[rax+650]=ty → led_x→r11슬롯, led_y→r8슬롯

// ── ② LINE_RANGE 후킹 (FUN_1420f9ab0 case[0] target read) = magic_knight skill1 ──
const RVA_LR_PATCH: usize = 0x20f9b68;
const LR_ORIG_LEN: usize  = 14;
const LR_EXPECT: [u8; 14] = [0x4C,0x8B,0x80,0x48,0x06,0x00,0x00, 0x4C,0x8B,0x98,0x50,0x06,0x00,0x00];
// LR: r8=[rax+648]=tx, r11=[rax+650]=ty → led_x→r8슬롯, led_y→r11슬롯

// ── ③ SAMPLER 후킹 (이동적용 FUN_1418d0190 진입, r8=entity) ──
const RVA_MOVE: usize = 0x18d0190;
const MOVE_ORIG_LEN: usize = 12;
const MOVE_EXPECT: [u8; 12] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];

// ── 진단: line_range apply(FUN_1420f9ab0) 진입훅 = variant/dx/dy/caller RVA 로깅 ──
const RVA_LR_APPLY: usize = 0x20f9ab0;
const LR_APPLY_LEN: usize = 12;
const LR_APPLY_EXPECT: [u8; 12] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];

// ── ★ DISPATCH 후킹 (FUN_141e41800 진입, rcx=caster) = casting block 리드 주입 ──
//   caster casting block: +0xa8=variant(i32, 2=line_range 절대좌표), +0xb0=aim_x(i64), +0xb8=aim_y(i64).
//   variant==2면 aim좌표(=타깃 현위치)를 velocity캐시 역탐색→미래위치로 바꿔써서 apply가 그 좌표를 복사→리드.
const RVA_DISPATCH: usize = 0x1e41800;
const DISP_LEN: usize = 12;
const DISP_EXPECT: [u8; 12] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];
const CB_VARIANT: usize = 0xa8;
const CB_DX: usize = 0xb0;
const CB_DY: usize = 0xb8;
const MATCH_THR: i64 = 12_000;   // aim↔타깃현위치 매칭 임계(역탐색)

// ── 오프셋/상수 ──
const E_POSX: usize = 0x648;
const E_POSY: usize = 0x650;
const E_SPEED: usize = 0x628;
const E_HP: usize = 0x658;
const E_NAME: usize = 0x250;
// 로스터(plan_base + team*0x228 + 0x1e0 + i*8 = 전투엔티티). plan_reimpl 검증.
const ROSTER_BASE: usize = 0x1e0;
const ROSTER_STRIDE: usize = 0x228;
const ROSTER_N: usize = 5;
const PROJDEF_SPEED: usize = 0x80;
const VEL_MAX_STEP: i64 = 12_000;   // enum간 변위 상한(초과=텔포/리콜/respawn 이상치 → velocity 무효). 정상 이동만 통과
const LEAD_CAP: i64 = 300_000;      // 리드 오프셋 상한(안전). 시전딜레이(~41틱) 대비 크게. lead_ticks(cfg)가 주 튜닝
const POS_MIN: i64 = 0;
const POS_MAX: i64 = 2_000_000;
const VEL_CACHE_CAP: usize = 4096;

// ── cfg 상태 ──
static LOG_ON: AtomicBool = AtomicBool::new(true);
static AIM_LEAD: AtomicU8 = AtomicU8::new(0);
static MK_ONLY: AtomicBool = AtomicBool::new(true);
static LEAD_PCT: AtomicI64 = AtomicI64::new(100);
static LEAD_TICKS: AtomicI64 = AtomicI64::new(8);    // 조준 미래위치 = aim + vel*lead_ticks*(lead_pct/100). 주 튜닝레버
static MATCH_THR_A: AtomicI64 = AtomicI64::new(80000);  // aim↔타깃현위치 역탐색 매칭 임계(cfg match_thr)
static VEL_SMOOTH: AtomicI64 = AtomicI64::new(35);     // velocity EMA 계수 %(α). 낮을수록 부드럽(노이즈↓)·반응느림
static REAIM: AtomicBool = AtomicBool::new(true);      // 1=사거리 밖 타깃이면 닿는 적으로 재조준(헛방→명중)
// ── 하드웨어 watchpoint(시전 setter 발견용, cfg wp_find=1) ──
static WP_FIND: AtomicBool = AtomicBool::new(false);   // 1=발견 스레드 가동(평소 0=완전 비활성)
static WP_VEH_INSTALLED: AtomicBool = AtomicBool::new(false);
static WP_FOUND: AtomicBool = AtomicBool::new(false);
static WP_ADDR: AtomicUsize = AtomicUsize::new(0);     // 현재 감시중인 caster+off 절대주소
static WP_WATCH_OFF: AtomicUsize = AtomicUsize::new(0xb0);  // 감시 필드 오프셋(0xb0=aim_x / 0x68=state tag)
static WP_CAST_TARGET: AtomicUsize = AtomicUsize::new(0);  // apply서 잡은 실제 magic_knight caster 엔티티
// 캡처된 distinct (RIP, 값) 쌍. +0x68 모드서 상태값별 writer 구분용(이동값에 안 묻히게).
const WP_CAP: usize = 24;
static WP_NHITS: AtomicUsize = AtomicUsize::new(0);
static WP_HIT_RIP: [AtomicU64; WP_CAP] = [const { AtomicU64::new(0) }; WP_CAP];
static WP_HIT_VAL: [AtomicU64; WP_CAP] = [const { AtomicU64::new(0) }; WP_CAP];
const CAST_AIMX: usize = 0xb0;                          // casting block aim_x (감시 대상)
// ★magic_knight skill1 projdef 버프(시그니처 +0x98==41 && +0xa8==300). 원본 range 113000/width 5000.
static RANGE_MULT: AtomicI64 = AtomicI64::new(160);   // 사정거리 % (113000*x/100). 적 escape 보정
static WIDTH_MULT: AtomicI64 = AtomicI64::new(100);   // 폭 % (5000*x/100)
const MK_S1_RANGE: i64 = 113_000;
const MK_S1_WIDTH: i64 = 5_000;
const MK_S1_RANGE_OFF: usize = 0x88;
const MK_S1_WIDTH_OFF: usize = 0x90;
const MK_S1_SIG_DELAY: usize = 0x98;   // ==41
const MK_S1_SIG_A8: usize = 0xa8;      // ==300
static CALLS: AtomicU64 = AtomicU64::new(0);
static MK_CALLS: AtomicU64 = AtomicU64::new(0);
static LR_CALLS: AtomicU64 = AtomicU64::new(0);
static APPLY_CALLS: AtomicU64 = AtomicU64::new(0);
static DISP_LED: AtomicU64 = AtomicU64::new(0);
static SAMP_CALLS: AtomicU64 = AtomicU64::new(0);   // 샘플러 총 호출수
static SAMP_INS: AtomicU64 = AtomicU64::new(0);     // 샘플러 캐시 insert 성공수
static PDEF_LOG: AtomicU64 = AtomicU64::new(0);     // projdef 필드 프로브 로그수
static CFG_LOADED: AtomicBool = AtomicBool::new(false);
const LOG_LIMIT: u64 = 250;
const APPLY_LOG_LIMIT: u64 = 3000;   // 데모(메뉴)가 소진 못하게 크게 → 스크림 apply도 로깅

#[derive(Clone, Copy)]
struct Sample { x: i64, y: i64, vx: i64, vy: i64, has_vel: bool }
static VEL_CACHE: Mutex<Option<HashMap<usize, Sample>>> = Mutex::new(None);
// 재리드 방지: caster→내가 마지막에 쓴 led좌표. 다음틱 aim이 그대로면(게임 미갱신) skip.
static LED_STATE: Mutex<Option<HashMap<usize, (i64, i64)>>> = Mutex::new(None);
// ── 로스터 기반 velocity: plan_base 탐지 + 후보포인터 캡처 + 직전위치 ──
static PLAN_BASE: AtomicUsize = AtomicUsize::new(0);
static PB_TRIES: AtomicU64 = AtomicU64::new(0);
static CAP_SIM: AtomicUsize = AtomicUsize::new(0);       // apply param_4(sim_state)
static CAP_CCTX: AtomicUsize = AtomicUsize::new(0);      // apply param_3(caster_ctx)
static CAP_SAMP: AtomicUsize = AtomicUsize::new(0);      // sampler r8(고정 ctx)
static ROSTER_PREV: Mutex<Option<HashMap<usize, (i64, i64)>>> = Mutex::new(None);  // 직전 enum 위치
static SEEN_PDEF: Mutex<Vec<usize>> = Mutex::new(Vec::new());  // pdef 프로브 dedup(고유 projdef)

// ── Win32 ──
type BOOL = i32; type DWORD = u32; type HMODULE = usize;
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleHandleExW(flags: u32, addr: *const u16, h: *mut HMODULE) -> BOOL;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, sz: DWORD) -> DWORD;
    fn VirtualAlloc(addr: usize, sz: usize, typ: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old: *mut u32) -> BOOL;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, sz: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
}
#[repr(C)] #[derive(Default)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32,
    region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32 }

static EXE_BASE: AtomicU64 = AtomicU64::new(0);
#[inline] unsafe fn exe_base() -> usize {
    let v = EXE_BASE.load(Ordering::Relaxed) as usize;
    if v != 0 { return v; }
    let b = GetModuleHandleW(core::ptr::null());
    EXE_BASE.store(b as u64, Ordering::Relaxed); b
}
#[inline] unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const RD: u32 = 0x02|0x04|0x20|0x40; const GUARD: u32 = 0x01|0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}
#[inline] unsafe fn rd_i64(a: usize) -> Option<i64> { if readable(a, 8) { Some(core::ptr::read_unaligned(a as *const i64)) } else { None } }
#[inline] unsafe fn rd_i32(a: usize) -> Option<i32> { if readable(a, 4) { Some(core::ptr::read_unaligned(a as *const i32)) } else { None } }
#[inline] unsafe fn rd_u8(a: usize) -> Option<u8> { if readable(a, 1) { Some(core::ptr::read_unaligned(a as *const u8)) } else { None } }
#[inline] unsafe fn writable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const WR: u32 = 0x04|0x08|0x40|0x80; const GUARD: u32 = 0x01|0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & WR == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}
#[inline] unsafe fn wr_i64(a: usize, v: i64) -> bool { if writable(a, 8) { core::ptr::write_unaligned(a as *mut i64, v); true } else { false } }
#[inline] unsafe fn rd_u64(a: usize) -> Option<u64> { if readable(a, 8) { Some(core::ptr::read_unaligned(a as *const u64)) } else { None } }
#[inline] fn ptr_ok(a: usize) -> bool { a >= 0x10000 && a < (1usize << 48) }

// ── 로스터 시그니처: 후보 c가 plan_base인지 점수(유효 챔프 수 0..10) [plan_reimpl 검증본] ──
unsafe fn roster_sig(c: usize) -> usize {
    if !ptr_ok(c) { return 0; }
    let mut cnt = 0;
    for team in 0..2usize {
        let base = c + team*ROSTER_STRIDE + ROSTER_BASE;
        if !readable(base, ROSTER_N*8) { continue; }
        for i in 0..ROSTER_N {
            let e = rd_u64(base + i*8).unwrap_or(0) as usize;
            if e <= 0x10000 || !readable(e, 0x740) { continue; }
            let sp = rd_i64(e + E_SPEED).unwrap_or(0);
            let x  = rd_i64(e + E_POSX).unwrap_or(-1);
            let hp = rd_i64(e + E_HP).unwrap_or(-1);
            // ★챔피언만 카운트(미니언/타워/몬스터 배열 false-positive 차단) — 기본체크 통과시에만 이름검사
            if sp > 0 && x >= 0 && x < 2_000_000 && hp > 0 && hp < 1_000_000 && is_champion(e) { cnt += 1; }
        }
    }
    cnt
}
// ── plan_base 탐지: 캡처 포인터들 + 그 필드(+0..0x800)를 roster_sig로 스캔, score>=6 채택 ──
unsafe fn try_find_plan_base() {
    // (sampler가 현재 plan_base 약할 때만 호출) 최신 시드로 더 나은 후보 탐색→교체
    let seeds = [CAP_SIM.load(Ordering::Relaxed), CAP_CCTX.load(Ordering::Relaxed), CAP_SAMP.load(Ordering::Relaxed)];
    let mut best = (0usize, 0usize);  // (score, addr)
    for &s in seeds.iter() {
        if !ptr_ok(s) { continue; }
        let sc = roster_sig(s);
        if sc > best.0 { best = (sc, s); }
        if !readable(s, 0x1008) { continue; }
        let mut o = 0usize;
        while o < 0x1000 {
            if let Some(p) = rd_u64(s + o) {
                let c = p as usize;
                if ptr_ok(c) { let sc = roster_sig(c); if sc > best.0 { best = (sc, c); } }
            }
            o += 8;
        }
    }
    if best.0 < 5 { return; }
    // near-miss 정밀화: best 주변 ±0x100(8B step)서 최고점 = 진짜 plan_base
    let mut rb = best;
    for k in -32i64..=32 {
        let c = (best.1 as i64 + k*8) as usize;
        if ptr_ok(c) { let sc = roster_sig(c); if sc > rb.0 { rb = (sc, c); } }
    }
    if rb.0 >= 8 {
        PLAN_BASE.store(rb.1, Ordering::Relaxed);   // 약한 현재 plan_base 교체
        append_log(&format!("[planbase] SET 0x{:x} score={} (raw 0x{:x} sc{})\n", rb.1, rb.0, best.1, best.0));
    }
}
// ── 로스터 열거→velocity 갱신: VEL_CACHE를 챔피언 현위치+(현-직전) 속도로 채움 ──
unsafe fn update_roster_velocities() {
    let pb = PLAN_BASE.load(Ordering::Relaxed);
    if !ptr_ok(pb) { return; }
    let mut cur: Vec<(usize, i64, i64)> = Vec::new();
    for team in 0..2usize {
        let base = pb + team*ROSTER_STRIDE + ROSTER_BASE;
        if !readable(base, ROSTER_N*8) { continue; }
        for i in 0..ROSTER_N {
            let e = rd_u64(base + i*8).unwrap_or(0) as usize;
            if e <= 0x10000 || !readable(e, 0x740) { continue; }
            let x = rd_i64(e + E_POSX).unwrap_or(-1);
            let y = rd_i64(e + E_POSY).unwrap_or(-1);
            if pos_ok(x, y) { cur.push((e, x, y)); }
        }
    }
    if cur.is_empty() { return; }
    let alpha = VEL_SMOOTH.load(Ordering::Relaxed) as f64 / 100.0;
    let mut pg = ROSTER_PREV.lock().unwrap_or_else(|e| e.into_inner());
    let prev = pg.get_or_insert_with(HashMap::new);
    let mut vg = VEL_CACHE.lock().unwrap_or_else(|e| e.into_inner());
    let vmap = vg.get_or_insert_with(HashMap::new);
    for (e, x, y) in &cur {
        // 이번 틱 raw 변위(이상치=텔포/리콜은 무효)
        let raw = prev.get(e).and_then(|(px, py)| {
            let dx = x - px; let dy = y - py;
            if dx.abs() <= VEL_MAX_STEP && dy.abs() <= VEL_MAX_STEP { Some((dx, dy)) } else { None }
        });
        let old = vmap.get(e).map(|s| (s.vx, s.vy, s.has_vel)).unwrap_or((0, 0, false));
        // EMA: smoothed = α*raw + (1-α)*old. 새 샘플 없으면 기존 유지.
        let (vx, vy, has) = match raw {
            Some((rx, ry)) => (
                (alpha * rx as f64 + (1.0 - alpha) * old.0 as f64) as i64,
                (alpha * ry as f64 + (1.0 - alpha) * old.1 as f64) as i64,
                true),
            None => (old.0, old.1, old.2),
        };
        vmap.insert(*e, Sample { x: *x, y: *y, vx, vy, has_vel: has });
        prev.insert(*e, (*x, *y));
    }
    // 현재 로스터에 없는 stale 엔티티(경기전환 등) 제거
    let curset: std::collections::HashSet<usize> = cur.iter().map(|(e, _, _)| *e).collect();
    vmap.retain(|k, _| curset.contains(k));
    prev.retain(|k, _| curset.contains(k));
}
unsafe fn caster_name(e: usize) -> String {
    let p = match rd_i64(e + E_NAME) { Some(p) => p as usize, None => return String::new() };
    if p < 0x10000 { return String::new(); }
    let mut bytes = Vec::with_capacity(24);
    for i in 0..40usize { match rd_u8(p + i) { Some(0) | None => break, Some(b) => bytes.push(b) } }
    String::from_utf8_lossy(&bytes).into_owned()
}
#[inline] unsafe fn pos_ok(x: i64, y: i64) -> bool { x > POS_MIN && x < POS_MAX && y > POS_MIN && y < POS_MAX }
// 챔피언 판별(이름에 minion/tower/monster/nexus/projectile 없음) — plan_base를 챔프 로스터로 안정화
unsafe fn is_champion(e: usize) -> bool {
    let nm = caster_name(e);
    if nm.len() < 3 { return false; }
    !(nm.contains("minion") || nm.contains("tower") || nm.contains("monster")
        || nm.contains("nexus") || nm.contains("projectile") || nm.contains("ward"))
}

// ── 로그/경로/cfg ──
fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }
fn dir() -> Option<PathBuf> { unsafe {
    let mut h: HMODULE = 0;
    if GetModuleHandleExW(0x4|0x2, dir as *const () as *const u16, &mut h) == 0 || h == 0 { return None; }
    let mut b = [0u16; 4096];
    let n = GetModuleFileNameW(h, b.as_mut_ptr(), b.len() as DWORD);
    if n == 0 { return None; }
    let mut p = PathBuf::from(String::from_utf16_lossy(&b[..n as usize])); p.pop(); Some(p)
}}
fn pth(name: &str) -> Option<PathBuf> { dir().map(|mut p| { p.push(name); p }) }
fn append_log(s: &str) {
    if !LOG_ON.load(Ordering::Relaxed) { return; }
    if let Some(p) = pth("tfm2_aim_lead.txt") {
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = write!(f, "{}", s); }
    }
}
fn load_cfg() {
    let p = match pth("tfm2_aim_lead.cfg") { Some(p) => p, None => return };
    let txt = match fs::read_to_string(&p) { Ok(t) => t, Err(_) => { let _=fs::write(&p,"log=1\naim_lead=1\nmk_only=1\nlead_pct=100\n"); return; } };
    for line in txt.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') { continue; }
        if let Some((k, v)) = t.split_once('=') {
            let k = k.trim(); let v = v.split('#').next().unwrap_or("").trim();
            let n = v.parse::<i64>().ok();
            match k {
                "log"      => LOG_ON.store(v == "1" || v.eq_ignore_ascii_case("true"), Ordering::Relaxed),
                "aim_lead" => AIM_LEAD.store(n.unwrap_or(0).clamp(0, 2) as u8, Ordering::Relaxed),
                "mk_only"  => MK_ONLY.store(v == "1" || v.eq_ignore_ascii_case("true"), Ordering::Relaxed),
                "lead_pct" => LEAD_PCT.store(n.unwrap_or(100).clamp(0, 400), Ordering::Relaxed),
                "lead_ticks" => LEAD_TICKS.store(n.unwrap_or(20).clamp(0, 120), Ordering::Relaxed),
                "match_thr"  => MATCH_THR_A.store(n.unwrap_or(80000).clamp(1000, 1_000_000), Ordering::Relaxed),
                "range_mult" => RANGE_MULT.store(n.unwrap_or(160).clamp(50, 400), Ordering::Relaxed),
                "width_mult" => WIDTH_MULT.store(n.unwrap_or(100).clamp(50, 600), Ordering::Relaxed),
                "vel_smooth" => VEL_SMOOTH.store(n.unwrap_or(35).clamp(5, 100), Ordering::Relaxed),
                "reaim"    => REAIM.store(v == "1" || v.eq_ignore_ascii_case("true"), Ordering::Relaxed),
                "wp_find"  => WP_FIND.store(v == "1" || v.eq_ignore_ascii_case("true"), Ordering::Relaxed),
                "wp_find2" => if v == "1" || v.eq_ignore_ascii_case("true") {  // +0x68(state tag) 추적 모드
                    WP_FIND.store(true, Ordering::Relaxed); WP_WATCH_OFF.store(0x68, Ordering::Relaxed);
                },
                "wp_find3" => if v == "1" || v.eq_ignore_ascii_case("true") {  // +0x88(range) memcpy의 caller 체인 추적
                    WP_FIND.store(true, Ordering::Relaxed); WP_WATCH_OFF.store(0x88, Ordering::Relaxed);
                },
                "wp_find4" => if v == "1" || v.eq_ignore_ascii_case("true") {  // +0x2b8(effect-list count) push의 caller 체인(skill1 필터)
                    WP_FIND.store(true, Ordering::Relaxed); WP_WATCH_OFF.store(0x2b8, Ordering::Relaxed);
                },
                _ => {}
            }
        }
    }
    CFG_LOADED.store(true, Ordering::Relaxed);
}
#[inline] fn maybe_reload() {
    let n = CALLS.fetch_add(1, Ordering::Relaxed);
    if n % 256 == 0 || !CFG_LOADED.load(Ordering::Relaxed) { load_cfg(); }
}

// ── 리드 계산 (공통). 반환 Some((led_x,led_y,t_hit)) = 리드적용, None = passthrough ──
unsafe fn compute_led(target_e: usize, cx: i64, cy: i64, projdef: usize, tx: i64, ty: i64) -> Option<(i64, i64, f64)> {
    if AIM_LEAD.load(Ordering::Relaxed) == 0 { return None; }
    let speed = rd_i64(projdef + PROJDEF_SPEED).unwrap_or(0);
    if speed < 1000 || speed > 5_000_000 { return None; }       // sanity(잘못된 projdef 방어)
    let (vx, vy) = target_velocity(target_e)?;
    let dx = (tx - cx) as f64; let dy = (ty - cy) as f64;
    let dist = (dx*dx + dy*dy).sqrt();
    let thit = dist / (speed as f64);
    let k = LEAD_PCT.load(Ordering::Relaxed) as f64 / 100.0;
    let ox = ((vx as f64 * thit * k) as i64).clamp(-LEAD_CAP, LEAD_CAP);
    let oy = ((vy as f64 * thit * k) as i64).clamp(-LEAD_CAP, LEAD_CAP);
    let (lx, ly) = (tx + ox, ty + oy);
    if !pos_ok(lx, ly) { return None; }
    Some((lx, ly, thit))
}
unsafe fn target_velocity(target_e: usize) -> Option<(i64, i64)> {
    let g = VEL_CACHE.lock().unwrap_or_else(|e| e.into_inner());
    let s = g.as_ref()?.get(&target_e)?;
    if s.has_vel { Some((s.vx, s.vy)) } else { None }
}
// aim좌표(=타깃 현위치)에 가장 가까운 캐시 엔티티 역탐색 → (tx,ty,vx,vy). caster 제외, thr 이내, has_vel.
unsafe fn find_closest_target(ax: i64, ay: i64, thr: i64, exclude: usize) -> Option<(i64, i64, i64, i64)> {
    let g = VEL_CACHE.lock().unwrap_or_else(|e| e.into_inner());
    let map = g.as_ref()?;
    let thr2 = (thr as i128) * (thr as i128);
    let mut best: Option<(i128, i64, i64, i64, i64)> = None;
    for (&ptr, s) in map.iter() {
        if ptr == exclude || !s.has_vel { continue; }
        let dx = (s.x - ax) as i128; let dy = (s.y - ay) as i128;
        let d2 = dx*dx + dy*dy;
        if d2 > thr2 { continue; }
        if best.map_or(true, |b| d2 < b.0) { best = Some((d2, s.x, s.y, s.vx, s.vy)); }
    }
    best.map(|b| (b.1, b.2, b.3, b.4))
}
// 진단용: 캐시크기 + 최근접 엔티티(임계무관). (cache_len, Option<(d2, tx,ty,vx,vy)>)
unsafe fn closest_in_cache(ax: i64, ay: i64, exclude: usize) -> (usize, Option<(i128, i64, i64, i64, i64)>) {
    let g = VEL_CACHE.lock().unwrap_or_else(|e| e.into_inner());
    let map = match g.as_ref() { Some(m) => m, None => return (0, None) };
    let len = map.len();
    let mut best: Option<(i128, i64, i64, i64, i64)> = None;
    for (&ptr, s) in map.iter() {
        if ptr == exclude || !s.has_vel { continue; }
        let dx = (s.x - ax) as i128; let dy = (s.y - ay) as i128; let d2 = dx*dx + dy*dy;
        if best.map_or(true, |b| d2 < b.0) { best = Some((d2, s.x, s.y, s.vx, s.vy)); }
    }
    (len, best)
}
#[inline] fn isqrt128(v: i128) -> i64 { if v <= 0 { return 0; } let mut x = (v as f64).sqrt() as i128; while x*x > v { x -= 1; } while (x+1)*(x+1) <= v { x += 1; } x as i64 }

// ── ★ DISPATCH cap: rcx=후보 엔티티. casting block(+0xa8 variant/+0xb0 aim_x/+0xb8 aim_y) 탐색+리드주입 ──
//   진단우선: +0xb0/+0xb8가 유효좌표(casting block 채워짐)인 엔티티를 로깅 → rcx가 맞는지 확인.
#[no_mangle]
pub extern "C" fn dispatch_cap(saved: *mut u64, _entry_rsp: usize) {
    if AIM_LEAD.load(Ordering::Relaxed) == 0 { return; }
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        if saved.is_null() { return; }
        let caster = *saved.add(0) as usize;          // rcx
        if caster < 0x10000 { return; }
        // rcx가 엔티티인지(자기 좌표 유효)
        let cx = match rd_i64(caster + E_POSX) { Some(v) => v, None => return };
        let cy = match rd_i64(caster + E_POSY) { Some(v) => v, None => return };
        if !pos_ok(cx, cy) { return; }
        // casting block 후보: +0xb0/+0xb8가 유효 절대좌표?
        let ax = rd_i64(caster + CB_DX).unwrap_or(-1);
        let ay = rd_i64(caster + CB_DY).unwrap_or(-1);
        if !pos_ok(ax, ay) { return; }                // 채워진 aim 블록만
        let variant = rd_i32(caster + CB_VARIANT).unwrap_or(-99);
        // 재리드 방지
        {
            let g = LED_STATE.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(m) = g.as_ref() { if m.get(&caster) == Some(&(ax, ay)) { return; } }
        }
        let tgt = find_closest_target(ax, ay, MATCH_THR, caster);
        // 리드 적용: variant==2(line_range 절대) + 타깃매칭
        let mut led = (ax, ay); let mut ox = 0i64; let mut oy = 0i64; let mut wrote = false;
        if variant == 2 {
            if let Some((_tx, _ty, vx, vy)) = tgt {
                let ticks = LEAD_TICKS.load(Ordering::Relaxed) as f64;
                let k = LEAD_PCT.load(Ordering::Relaxed) as f64 / 100.0;
                ox = ((vx as f64 * ticks * k) as i64).clamp(-LEAD_CAP, LEAD_CAP);
                oy = ((vy as f64 * ticks * k) as i64).clamp(-LEAD_CAP, LEAD_CAP);
                let (lx, ly) = (ax + ox, ay + oy);
                if pos_ok(lx, ly) {
                    wrote = wr_i64(caster + CB_DX, lx) && wr_i64(caster + CB_DY, ly);
                    led = (lx, ly);
                    if wrote {
                        let mut g = LED_STATE.lock().unwrap_or_else(|e| e.into_inner());
                        let m = g.get_or_insert_with(HashMap::new);
                        if m.len() > VEL_CACHE_CAP { m.clear(); }
                        m.insert(caster, led);
                    }
                }
            }
        }
        let n = DISP_LED.fetch_add(1, Ordering::Relaxed);
        if LOG_ON.load(Ordering::Relaxed) && n < LOG_LIMIT {
            let (tdesc) = match tgt { Some((tx,ty,vx,vy)) => format!("tgt=({},{}) vel=({},{})", tx,ty,vx,vy), None => "tgt=NONE".to_string() };
            append_log(&format!("[disp {}] caster=0x{:x} self=({},{}) variant={} aim=({},{}) {} led=({},{}) off=({},{}) wrote={}\n",
                n, caster, cx, cy, variant, ax, ay, tdesc, led.0, led.1, ox, oy, wrote));
        }
    }));
}

// ── ③ SAMPLER: saved[2]=r8=entity. 매틱 위치 기록→velocity. aim_lead=0이면 idle ──
#[no_mangle]
pub extern "C" fn sampler_cap(saved: *mut u64, _entry_rsp: usize) {
    let c = SAMP_CALLS.fetch_add(1, Ordering::Relaxed);
    if AIM_LEAD.load(Ordering::Relaxed) == 0 { return; }
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        if saved.is_null() { return; }
        CAP_SAMP.store(*saved.add(2) as usize, Ordering::Relaxed);   // 시드 항상 최신
        if c % 1500 != 0 { return; }   // 무거운 작업은 스로틀
        let pb = PLAN_BASE.load(Ordering::Relaxed);
        let sig = if ptr_ok(pb) { roster_sig(pb) } else { 0 };
        if sig >= 4 {
            // 유효: 로스터 velocity 갱신
            update_roster_velocities();
            SAMP_INS.fetch_add(1, Ordering::Relaxed);
        } else if c % 30000 == 0 {
            // 약함/없음: 최신 시드로 재탐지(더 무거움, 덜 자주)
            PB_TRIES.fetch_add(1, Ordering::Relaxed);
            try_find_plan_base();
        }
    }));
}

// ── 진단: line_range apply 진입. entry_rsp[0]=caller ret, entry_rsp[0x38]=param_7 ptr.
//     param_7[0]=variant(i32), [+8]=dx, [+0x10]=dy. magic_knight skill1 variant/방향/caller 실측. ──
// ★ line_range 리드 주입: apply 진입에서 param_7(=복사된 casting block) 잡아 variant==2면 dx/dy(절대 aim좌표)를
//   미래위치로 바꿔씀 → case[2]가 그 좌표로 발사. 타깃=velocity캐시 역탐색(aim≈타깃현위치).
#[no_mangle]
pub extern "C" fn lr_apply_cap(saved: *mut u64, entry_rsp: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        // ★entry-detour saved 레이아웃: [0]=r11 [1]=r10 [2]=r9(param_4 sim) [3]=r8(param_3 cctx) [4]=rdx [5]=rcx(param_1 projdef)
        let projdef = if !saved.is_null() {
            CAP_CCTX.store(*saved.add(3) as usize, Ordering::Relaxed);  // r8=caster_ctx
            CAP_SIM.store(*saved.add(2) as usize, Ordering::Relaxed);   // r9=sim_state
            *saved.add(5) as usize   // rcx = param_1 = projdef (effect/self)
        } else { 0 };
        let p7 = rd_i64(entry_rsp + 0x38).unwrap_or(0) as usize;   // param_7 ptr
        if p7 < 0x10000 { return; }
        let variant = rd_i32(p7).unwrap_or(-99);
        let ax = rd_i64(p7 + 8).unwrap_or(-1);
        let ay = rd_i64(p7 + 0x10).unwrap_or(-1);
        // ★진단: projdef 필드 프로브 — 고유 projdef마다 1회(dedup, 데모 무관). 캐스트수도 기록.
        if variant == 2 && projdef >= 0x10000 && LOG_ON.load(Ordering::Relaxed) {
            let cnt = PDEF_LOG.fetch_add(1, Ordering::Relaxed);
            let mut g = SEEN_PDEF.lock().unwrap_or_else(|e| e.into_inner());
            if !g.contains(&projdef) && g.len() < 16 {
                g.push(projdef);
                let mut s = format!("[pdef #{} cast{}] projdef=0x{:x}", g.len(), cnt, projdef);
                for o in (0x60usize..0x140).step_by(8) {
                    s.push_str(&format!(" +{:x}={}", o, rd_i64(projdef + o).unwrap_or(0)));
                }
                s.push('\n'); append_log(&s);
            }
        }
        // ★magic_knight skill1 range/width 버프: 시그니처(+0x98==41 && +0xa8==300) 매칭시 공유 effect def 덮어씀.
        //   +0x98/+0xa8은 안 건드림(시그니처 유지=idempotent·튜닝 재적용 가능). 적 escape 보정.
        if AIM_LEAD.load(Ordering::Relaxed) != 0 && projdef >= 0x10000
            && rd_i64(projdef + MK_S1_SIG_DELAY) == Some(41) && rd_i64(projdef + MK_S1_SIG_A8) == Some(300) {
            let rm = RANGE_MULT.load(Ordering::Relaxed);
            if rm != 100 { wr_i64(projdef + MK_S1_RANGE_OFF, MK_S1_RANGE * rm / 100); }
            let wm = WIDTH_MULT.load(Ordering::Relaxed);
            if wm != 100 { wr_i64(projdef + MK_S1_WIDTH_OFF, MK_S1_WIDTH * wm / 100); }
            let bn = PDEF_LOG.load(Ordering::Relaxed);
            if LOG_ON.load(Ordering::Relaxed) && bn < 60 && bn % 20 == 0 {
                append_log(&format!("[rangebuff] projdef=0x{:x} range {}->{} width {}->{}\n",
                    projdef, MK_S1_RANGE, rd_i64(projdef+MK_S1_RANGE_OFF).unwrap_or(0),
                    MK_S1_WIDTH, rd_i64(projdef+MK_S1_WIDTH_OFF).unwrap_or(0)));
            }
        }
        // ★watchpoint 발견: 첫 magic_knight caster에 PIN(마검사 5명 오실레이션 방지). 좌표 유효+이름 검증.
        //   pin 후엔 그 caster만 감시 → 그가 시전할 때 시전 상태값 writer 포착.
        if WP_FIND.load(Ordering::Relaxed) && WP_CAST_TARGET.load(Ordering::Relaxed) == 0
            && variant == 2 && projdef >= 0x10000 && pos_ok(ax, ay)
            && rd_i64(projdef + MK_S1_SIG_DELAY) == Some(41) && rd_i64(projdef + MK_S1_SIG_A8) == Some(300) {
            let pb = PLAN_BASE.load(Ordering::Relaxed);
            if let Some((caster, _team)) = find_caster_by_castblock(pb, ax, ay) {
                let nm = caster_name(caster);
                if nm.contains("magic") || nm.contains("knight") || nm.contains("마검") {
                    WP_CAST_TARGET.store(caster, Ordering::Relaxed);
                }
            }
        }
        let mut led = (ax, ay); let mut ox = 0i64; let mut oy = 0i64; let mut wrote = false;
        let mut tdesc = String::from("tgt=NONE");
        let mut clen = 0usize; let mut cdist = -1i64; let mut reaimed = false;
        if AIM_LEAD.load(Ordering::Relaxed) != 0 && variant == 2 && pos_ok(ax, ay) {
            let ticks_i = LEAD_TICKS.load(Ordering::Relaxed);
            let ticks = ticks_i as f64;
            let k = LEAD_PCT.load(Ordering::Relaxed) as f64 / 100.0;
            let pb = PLAN_BASE.load(Ordering::Relaxed);
            let is_mk_s1 = rd_i64(projdef + MK_S1_SIG_DELAY) == Some(41) && rd_i64(projdef + MK_S1_SIG_A8) == Some(300);
            let (len, best) = closest_in_cache(ax, ay, 0);
            clen = len;
            let thr = MATCH_THR_A.load(Ordering::Relaxed);
            let mut chosen: Option<(i64, i64)> = None;
            if let Some((d2, tx, ty, vx, vy)) = best {
                cdist = isqrt128(d2);
                tdesc = format!("tgt=({},{}) vel=({},{})", tx, ty, vx, vy);
                if cdist <= thr {
                    // ★조준 기준 = 타깃 현재위치(un-stale) + 비행 리드
                    let lvx = ((vx as f64 * ticks * k) as i64).clamp(-LEAD_CAP, LEAD_CAP);
                    let lvy = ((vy as f64 * ticks * k) as i64).clamp(-LEAD_CAP, LEAD_CAP);
                    let (lx, ly) = (tx + lvx, ty + lvy);
                    // ★재조준: magic_knight skill1 + reaim 켜짐 + 의도타깃이 사거리 밖이면 → 닿는 적으로
                    if REAIM.load(Ordering::Relaxed) && is_mk_s1 {
                        if let Some((caster, team)) = find_caster_by_castblock(pb, ax, ay) {
                            let cx = rd_i64(caster + E_POSX).unwrap_or(0);
                            let cy = rd_i64(caster + E_POSY).unwrap_or(0);
                            let range = rd_i64(projdef + MK_S1_RANGE_OFF).unwrap_or(MK_S1_RANGE);
                            let dx = (lx - cx) as i128; let dy = (ly - cy) as i128;
                            if dx*dx + dy*dy <= (range as i128) * (range as i128) {
                                chosen = Some((lx, ly));               // 사거리 안 → 예측타깃 리드
                            } else if let Some((px, py)) = nearest_reachable_enemy(pb, 1 - team, cx, cy, range, ticks_i) {
                                chosen = Some((px, py)); reaimed = true; // 사거리 밖 → 닿는 적 재조준
                            } else {
                                chosen = Some((lx, ly));               // 닿는 적 없음 → 리드(헛방 감수)
                            }
                        } else { chosen = Some((lx, ly)); }            // caster 못찾음 → 리드만
                    } else { chosen = Some((lx, ly)); }                // reaim off → 기존 리드
                }
            }
            if let Some((fx, fy)) = chosen {
                if pos_ok(fx, fy) {
                    wrote = wr_i64(p7 + 8, fx) && wr_i64(p7 + 0x10, fy);
                    led = (fx, fy); ox = fx - ax; oy = fy - ay;
                }
            }
        }
        let m = APPLY_CALLS.fetch_add(1, Ordering::Relaxed);
        if LOG_ON.load(Ordering::Relaxed) && m < APPLY_LOG_LIMIT {
            append_log(&format!("[apply {}] variant={} aim=({},{}) pb=0x{:x} pbtry={} cache={} cdist={} updates={} {} led=({},{}) off=({},{}) reaim={} wrote={}\n",
                m, variant, ax, ay, PLAN_BASE.load(Ordering::Relaxed), PB_TRIES.load(Ordering::Relaxed), clen, cdist, SAMP_INS.load(Ordering::Relaxed), tdesc, led.0, led.1, ox, oy, reaimed, wrote));
        }
    }));
}

// ── ① LINEAR cap: blk[0]=rcx(cx) [4]=r10(cy) [10]=rsi(projdef) [11]=r15(caster) [12]=rax(target)
//     led_x→[5](r11) led_y→[2](r8). mk_only 게이트(magic_knight 한정). ──
#[no_mangle]
pub extern "C" fn aim_cap(saved: *mut u64) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        if saved.is_null() { return; }
        let target_e = *saved.add(12) as usize;
        let tx = rd_i64(target_e + E_POSX).unwrap_or(0);
        let ty = rd_i64(target_e + E_POSY).unwrap_or(0);
        *saved.add(5) = tx as u64;   // r11 = led_x (passthrough)
        *saved.add(2) = ty as u64;   // r8  = led_y (passthrough)
        maybe_reload();
        let cx = *saved.add(0) as i64;
        let cy = *saved.add(4) as i64;
        let projdef = *saved.add(10) as usize;
        let caster_e = *saved.add(11) as usize;
        let nm = caster_name(caster_e);
        let is_mk = nm.starts_with("magic_knight");
        let gate = if MK_ONLY.load(Ordering::Relaxed) { is_mk } else { true };
        let mut applied = false; let mut lx = tx; let mut ly = ty;
        if gate {
            if let Some((px, py, _t)) = compute_led(target_e, cx, cy, projdef, tx, ty) {
                lx = px; ly = py; *saved.add(5) = lx as u64; *saved.add(2) = ly as u64; applied = true;
            }
        }
        if is_mk {
            let m = MK_CALLS.fetch_add(1, Ordering::Relaxed);
            if LOG_ON.load(Ordering::Relaxed) && m < LOG_LIMIT {
                append_log(&format!("[lin {}] c=({},{}) t=({},{}) led=({},{}) off=({},{}) applied={}\n",
                    m, cx, cy, tx, ty, lx, ly, lx-tx, ly-ty, applied));
            }
        }
    }));
}

// ── ② LINE_RANGE cap (magic_knight skill1): blk[0]=rcx(cx) [1]=rdx(cy) [9]=r13(projdef) [12]=rax(target)
//     led_x→[2](r8) led_y→[5](r11). caster entity 없음→챔프게이트X(모든 line_range에 리드). ──
#[no_mangle]
pub extern "C" fn aim_cap_lr(saved: *mut u64) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        if saved.is_null() { return; }
        let target_e = *saved.add(12) as usize;
        let tx = rd_i64(target_e + E_POSX).unwrap_or(0);
        let ty = rd_i64(target_e + E_POSY).unwrap_or(0);
        *saved.add(2) = tx as u64;   // r8  = led_x (passthrough)
        *saved.add(5) = ty as u64;   // r11 = led_y (passthrough)
        maybe_reload();
        let cx = *saved.add(0) as i64;
        let cy = *saved.add(1) as i64;
        let projdef = *saved.add(9) as usize;
        let mut applied = false; let mut lx = tx; let mut ly = ty; let mut thit = 0.0;
        let mut vfound = false;
        if target_velocity(target_e).is_some() { vfound = true; }
        if let Some((px, py, t)) = compute_led(target_e, cx, cy, projdef, tx, ty) {
            lx = px; ly = py; thit = t; *saved.add(2) = lx as u64; *saved.add(5) = ly as u64; applied = true;
        }
        let m = LR_CALLS.fetch_add(1, Ordering::Relaxed);
        if LOG_ON.load(Ordering::Relaxed) && m < LOG_LIMIT {
            let speed = rd_i64(projdef + PROJDEF_SPEED).unwrap_or(0);
            append_log(&format!("[lr {}] c=({},{}) t=({},{}) speed={} vel={} t_hit={:.2} led=({},{}) off=({},{}) applied={}\n",
                m, cx, cy, tx, ty, speed, vfound, thit, lx, ly, lx-tx, ly-ty, applied));
        }
    }));
}

// =====================================================================================
// 설치
// =====================================================================================
// 미드훅(14B): rax 라이브 → jmp qword[rip] 무클로버 패치. 13push 컨텍스트블록(정렬보정). LINEAR/LINE_RANGE 공용.
unsafe fn install_mid_hook(patch_rva: usize, orig_len: usize, expect: &[u8], cap: usize) -> Result<(), &'static str> {
    let base = exe_base(); if base == 0 { return Err("module 0"); }
    let patch_addr = base + patch_rva;
    if !readable(patch_addr, orig_len + 4) { return Err("addr unreadable"); }
    let mut cur = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(patch_addr as *const u8, cur.as_mut_ptr(), orig_len);
    if cur.as_slice() != expect { return Err("byte mismatch (RVA stale?)"); }
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX); if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = patch_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    // push rax r15 rsi r13 r12 rbp rbx r11 r10 r9 r8 rdx rcx  (rcx last=blk+0)
    s.extend_from_slice(&[0x50, 0x41,0x57, 0x56, 0x41,0x55, 0x41,0x54, 0x55, 0x53,
                          0x41,0x53, 0x41,0x52, 0x41,0x51, 0x41,0x50, 0x52, 0x51]);
    s.extend_from_slice(&[0x48,0x89,0xe1]);        // mov rcx, rsp
    s.extend_from_slice(&[0x48,0x89,0xe3]);        // mov rbx, rsp
    s.extend_from_slice(&[0x48,0x83,0xe4,0xf0]);   // and rsp,-16
    s.extend_from_slice(&[0x48,0x83,0xec,0x20]);   // sub rsp,0x20
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap.to_le_bytes());
    s.extend_from_slice(&[0xff,0xd0]);             // call rax
    s.extend_from_slice(&[0x48,0x89,0xdc]);        // mov rsp, rbx
    // pop rcx rdx r8 r9 r10 r11 rbx rbp r12 r13 rsi r15 rax
    s.extend_from_slice(&[0x59, 0x5a, 0x41,0x58, 0x41,0x59, 0x41,0x5a, 0x41,0x5b, 0x5b, 0x5d,
                          0x41,0x5c, 0x41,0x5d, 0x5e, 0x41,0x5f, 0x58]);
    s.extend_from_slice(&[0xff,0x25,0x00,0x00,0x00,0x00]); s.extend_from_slice(&ret_addr.to_le_bytes());
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0u8; orig_len];
    patch[0] = 0xff; patch[1] = 0x25; patch[6..14].copy_from_slice(&stub.to_le_bytes());
    let mut old: u32 = 0;
    if VirtualProtect(patch_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), patch_addr as *mut u8, orig_len);
    VirtualProtect(patch_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), patch_addr, orig_len);
    Ok(())
}

// 진입훅(표준 install_detour, rax 클로버 OK = 진입). cap(saved, entry_rsp). SAMPLER/진단 공용.
unsafe fn install_entry_detour(rva: usize, orig_len: usize, expect: &[u8], cap: usize) -> Result<(), &'static str> {
    let base = exe_base(); if base == 0 { return Err("module 0"); }
    let fn_addr = base + rva;
    if !readable(fn_addr, orig_len + 4) { return Err("unreadable"); }
    let mut cur = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), orig_len);
    if cur.as_slice() != expect { return Err("byte mismatch"); }
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX); if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);                                              // mov r10, rsp
    s.extend_from_slice(&[0x51,0x52,0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]);           // push rcx rdx r8 r9 r10 r11
    s.extend_from_slice(&[0x48,0x89,0xe1]);                                              // mov rcx, rsp
    s.extend_from_slice(&[0x4c,0x89,0xd2]);                                              // mov rdx, r10
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);                                         // sub rsp,0x28
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap.to_le_bytes());          // movabs rax, cap
    s.extend_from_slice(&[0xff,0xd0]);                                                   // call rax
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);                                         // add rsp,0x28
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]);           // pop r11 r10 r9 r8 rdx rcx
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes());     // movabs rax, fn+len
    s.extend_from_slice(&[0xff,0xe0]);                                                   // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(())
}

// ═════════════════════════════════════════════════════════════════════════
//  하드웨어 write-watchpoint — 시전 setter 발견 (cfg wp_find=1)
//  magic_knight entity+0xb0(aim_x)에 DR0 write-BP → 게임이 쓰는 순간 #DB →
//  VEH가 RIP(=시전 setter 명령) 캡처. 정적으로 못 찾는 setter를 런타임 특정.
// ═════════════════════════════════════════════════════════════════════════
#[repr(C)] struct TE32 { dw_size: u32, cnt: u32, tid: u32, owner_pid: u32, base_pri: i32, delta_pri: i32, flags: u32 }
#[repr(C)] struct ExcRecord { code: u32 }           // EXCEPTION_RECORD: code@0만 읽음
#[repr(C)] struct ExcPointers { rec: *mut ExcRecord, ctx: *mut u8 }
type PVEH = extern "system" fn(*mut ExcPointers) -> i32;
#[repr(C, align(16))] struct CtxBuf([u8; 1232]);    // CONTEXT(x64): Dr0@0x48 Dr6@0x68 Dr7@0x70 Rip@0xF8 Flags@0x30
extern "system" {
    fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> usize;
    fn Thread32First(snap: usize, te: *mut TE32) -> i32;
    fn Thread32Next(snap: usize, te: *mut TE32) -> i32;
    fn OpenThread(access: u32, inherit: i32, tid: u32) -> usize;
    fn GetThreadContext(h: usize, ctx: *mut u8) -> i32;
    fn SetThreadContext(h: usize, ctx: *const u8) -> i32;
    fn SuspendThread(h: usize) -> u32;
    fn ResumeThread(h: usize) -> u32;
    fn CloseHandle(h: usize) -> i32;
    fn GetCurrentProcessId() -> u32;
    fn GetCurrentThreadId() -> u32;
    fn AddVectoredExceptionHandler(first: u32, h: PVEH) -> usize;
}
// VEH: #DB(0x80000004) + DR0-3 status → RIP 1회 캡처. ⚠최소 작업만(락/fs/format 금지).
extern "system" fn wp_veh(p: *mut ExcPointers) -> i32 {
    const CONT_EXEC: i32 = -1; const CONT_SEARCH: i32 = 0;
    unsafe {
        if p.is_null() { return CONT_SEARCH; }
        let rec = (*p).rec; let ctx = (*p).ctx;
        if rec.is_null() || ctx.is_null() { return CONT_SEARCH; }
        if (*rec).code != 0x80000004 { return CONT_SEARCH; }  // not single-step/data-BP
        let dr6 = core::ptr::read_unaligned(ctx.add(0x68) as *const u64);
        if dr6 & 0xf == 0 { return CONT_SEARCH; }              // DR0-3 미발화 → 우리 것 아님
        let rip = core::ptr::read_unaligned(ctx.add(0xf8) as *const u64) as u64;
        let base = EXE_BASE.load(Ordering::Relaxed);
        let wa = WP_ADDR.load(Ordering::Relaxed);
        let val = if wa != 0 { core::ptr::read_unaligned(wa as *const u64) } else { 0 };
        // ── caller-capture 모드 (+0x88=skill1 setup / +0x2b8=effect push): 스택 복귀체인 1회 캡처 ──
        let watch_off = WP_WATCH_OFF.load(Ordering::Relaxed);
        if watch_off == 0x88 || watch_off == 0x2b8 {
            if !WP_FOUND.load(Ordering::Relaxed) {
                let hit = if watch_off == 0x88 { val == 113000 }                       // skill1 range setup
                          else { pushed_is_skill1(wa.wrapping_sub(0x2b8)) };           // skill1 effect push
                if hit {
                    let rsp = core::ptr::read_unaligned(ctx.add(0x98) as *const u64) as usize;  // CONTEXT.Rsp
                    let mut n = 0usize; let mut o = 0usize;
                    while o < 0x600 && n < WP_CAP {
                        let v = core::ptr::read_unaligned((rsp + o) as *const u64);
                        if base != 0 && v >= base && v < base + 0x4000000 {  // game-exe 복귀주소
                            let mut seen = false; let mut j = 0; while j < n { if WP_HIT_RIP[j].load(Ordering::Relaxed) == v { seen = true; break; } j += 1; }
                            if !seen { WP_HIT_RIP[n].store(v, Ordering::Relaxed); WP_HIT_VAL[n].store(o as u64, Ordering::Relaxed); n += 1; }
                        }
                        o += 8;
                    }
                    WP_NHITS.store(n, Ordering::Relaxed);
                    WP_FOUND.store(true, Ordering::Relaxed);
                }
            }
            core::ptr::write_unaligned(ctx.add(0x68) as *mut u64, dr6 & !0xf);
            return CONT_EXEC;
        }
        // ── 기존 (+0x68/+0xb0) 모드: 게임 exe 밖 노이즈 필터 + distinct (RIP,값) 캡처 ──
        if base != 0 && (rip < base || rip >= base + 0x4000000) {
            core::ptr::write_unaligned(ctx.add(0x68) as *mut u64, dr6 & !0xf);
            return CONT_EXEC;
        }
        let n = WP_NHITS.load(Ordering::Relaxed);
        let mut seen = false;
        let mut i = 0; while i < n && i < WP_CAP {
            if WP_HIT_RIP[i].load(Ordering::Relaxed) == rip && WP_HIT_VAL[i].load(Ordering::Relaxed) == val { seen = true; break; }
            i += 1;
        }
        if !seen && n < WP_CAP {
            WP_HIT_RIP[n].store(rip, Ordering::Relaxed);
            WP_HIT_VAL[n].store(val, Ordering::Relaxed);
            WP_NHITS.store(n + 1, Ordering::Relaxed);
            WP_FOUND.store(true, Ordering::Relaxed);
        }
        core::ptr::write_unaligned(ctx.add(0x68) as *mut u64, dr6 & !0xf);  // DR6 상태비트 클리어
        CONT_EXEC
    }
}
// 모든 (현재) 스레드의 DR0=addr, DR7=write-watch-8byte 설정. 호출자=별도 발견스레드(sim스레드 미스킵).
unsafe fn wp_install(addr: usize) {
    let _ = exe_base();  // VEH의 게임-exe 필터가 EXE_BASE 필요 → 무장 전 보장
    if !WP_VEH_INSTALLED.swap(true, Ordering::Relaxed) { AddVectoredExceptionHandler(1, wp_veh); }
    WP_ADDR.store(addr, Ordering::Relaxed);
    const TH32CS_SNAPTHREAD: u32 = 0x4; const THREAD_ALL: u32 = 0x1fffff;
    let snap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
    if snap == 0 || snap == usize::MAX { return; }
    let pid = GetCurrentProcessId(); let cur = GetCurrentThreadId();
    let mut te: TE32 = core::mem::zeroed(); te.dw_size = core::mem::size_of::<TE32>() as u32;
    let (mut ok, mut set) = (Thread32First(snap, &mut te), 0i32);
    while ok != 0 {
        if te.owner_pid == pid && te.tid != cur {
            let h = OpenThread(THREAD_ALL, 0, te.tid);
            if h != 0 {
                SuspendThread(h);
                let mut cb = CtxBuf([0u8; 1232]); let c = cb.0.as_mut_ptr();
                core::ptr::write_unaligned(c.add(0x30) as *mut u32, 0x100010);  // CONTEXT_DEBUG_REGISTERS
                if GetThreadContext(h, c) != 0 {
                    core::ptr::write_unaligned(c.add(0x48) as *mut u64, addr as u64);  // Dr0
                    core::ptr::write_unaligned(c.add(0x70) as *mut u64, 0x90001u64);   // Dr7: L0|RW0=write|LEN0=8B
                    core::ptr::write_unaligned(c.add(0x30) as *mut u32, 0x100010);
                    if SetThreadContext(h, c) != 0 { set += 1; }
                }
                ResumeThread(h); CloseHandle(h);
            }
        }
        ok = Thread32Next(snap, &mut te);
    }
    CloseHandle(snap);
    append_log(&format!("[wp] install addr=0x{:x} threads_set={}\n", addr, set));
}
// 로스터에서 첫 magic_knight 엔티티(이름 매칭, 폴백용)
unsafe fn find_magic_knight(pb: usize) -> Option<usize> {
    for team in 0..2usize {
        let base = pb + team*ROSTER_STRIDE + ROSTER_BASE;
        if !readable(base, ROSTER_N*8) { continue; }
        for i in 0..ROSTER_N {
            let e = rd_u64(base + i*8).unwrap_or(0) as usize;
            if e <= 0x10000 || !readable(e, 0x740) { continue; }
            let nm = caster_name(e);
            if nm.contains("magic") || nm.contains("knight") || nm.contains("마검") { return Some(e); }
        }
    }
    None
}
// caster effect-list(+0x2b0 ptr, +0x2b8 count, stride 0x28)의 막 push된 effect가 skill1인지(시그니처).
// VEH서 호출 — rd_u64/rd_i64(VirtualQuery 기반 safe-read)만 사용, 패닉/락/alloc 없음.
unsafe fn pushed_is_skill1(caster: usize) -> bool {
    let vec_ptr = match rd_u64(caster + 0x2b0) { Some(v) => v as usize, None => return false };
    let count = match rd_u64(caster + 0x2b8) { Some(v) => v as usize, None => return false };
    if count == 0 || count > 64 || vec_ptr < 0x10000 { return false; }
    // 막 push된 요소(인라인, stride 0x28) 주소
    let elem = vec_ptr + (count - 1) * 0x28;
    // 요소의 각 qword를 projdef 후보로(=[+0x98]==41 && [+0xa8]==300 = skill1), 그리고 1단계 deref도 시도
    for k in 0..5usize {
        if let Some(p) = rd_u64(elem + k * 8) {
            let p = p as usize;
            if p >= 0x10000 {
                if rd_i64(p + 0x98) == Some(41) && rd_i64(p + 0xa8) == Some(300) { return true; }
                // 요소가 effect 포인터인 경우: deref한 effect 안의 projdef 후보
                if let Some(pp) = rd_u64(p + 0x10) { let pp = pp as usize;
                    if pp >= 0x10000 && rd_i64(pp + 0x98) == Some(41) && rd_i64(pp + 0xa8) == Some(300) { return true; }
                }
            }
        }
    }
    false
}
// ★실제 caster: casting block(+0xb0/+0xb8)이 apply의 aim과 일치하는 로스터 엔티티(이름 무관, robust). (caster, team) 반환.
unsafe fn find_caster_by_castblock(pb: usize, ax: i64, ay: i64) -> Option<(usize, usize)> {
    if !ptr_ok(pb) { return None; }
    for team in 0..2usize {
        let base = pb + team*ROSTER_STRIDE + ROSTER_BASE;
        if !readable(base, ROSTER_N*8) { continue; }
        for i in 0..ROSTER_N {
            let e = rd_u64(base + i*8).unwrap_or(0) as usize;
            if e <= 0x10000 || !readable(e, 0x740) { continue; }
            if rd_i64(e + CB_DX) == Some(ax) && rd_i64(e + CB_DY) == Some(ay) { return Some((e, team)); }
        }
    }
    None
}
// 적팀(enemy_team) 챔프 중 caster에서 (예측위치가) 사거리 안인 가장 가까운 적의 예측좌표. 재조준 대상.
unsafe fn nearest_reachable_enemy(pb: usize, enemy_team: usize, cx: i64, cy: i64, range: i64, lead: i64) -> Option<(i64, i64)> {
    if !ptr_ok(pb) || enemy_team >= 2 { return None; }
    let base = pb + enemy_team*ROSTER_STRIDE + ROSTER_BASE;
    if !readable(base, ROSTER_N*8) { return None; }
    let vg = VEL_CACHE.lock().unwrap_or_else(|e| e.into_inner());
    let r2 = (range as i128) * (range as i128);
    let mut best: Option<(i128, i64, i64)> = None;
    for i in 0..ROSTER_N {
        let e = rd_u64(base + i*8).unwrap_or(0) as usize;
        if e <= 0x10000 || !readable(e, 0x740) { continue; }
        let hp = rd_i64(e + E_HP).unwrap_or(0);
        let ex = rd_i64(e + E_POSX).unwrap_or(-1);
        let ey = rd_i64(e + E_POSY).unwrap_or(-1);
        if hp <= 0 || !pos_ok(ex, ey) { continue; }
        let (vx, vy) = vg.as_ref().and_then(|m| m.get(&e)).map(|s| (s.vx, s.vy)).unwrap_or((0, 0));
        let (px, py) = (ex + vx*lead, ey + vy*lead);   // 41틱후 예측 아닌 비행리드(escape는 이미 발생)
        let dx = (px - cx) as i128; let dy = (py - cy) as i128;
        let d2 = dx*dx + dy*dy;
        if d2 <= r2 && (best.is_none() || d2 < best.unwrap().0) { best = Some((d2, px, py)); }
    }
    best.map(|(_, px, py)| (px, py))
}
// 발견 스레드: plan_base+magic_knight 폴링 → watchpoint(엔티티 바뀌면 재설치) → 적중시 RVA 로그
fn wp_spawn_discovery() {
    std::thread::spawn(|| {
        let mut last_n = 0usize;
        let mut dumped_names = false;
        loop {
            std::thread::sleep(std::time::Duration::from_millis(400));
            unsafe {
                // 진단: 로스터 이름 1회 덤프(이름매칭 실패 원인 파악)
                if !dumped_names {
                    let pb = PLAN_BASE.load(Ordering::Relaxed);
                    if ptr_ok(pb) {
                        let mut s = String::from("[wp] roster names:");
                        for team in 0..2usize { for i in 0..ROSTER_N {
                            let e = rd_u64(pb + team*ROSTER_STRIDE + ROSTER_BASE + i*8).unwrap_or(0) as usize;
                            if e > 0x10000 && readable(e, 0x740) { s.push_str(&format!(" [{},{}]={}", team, i, caster_name(e))); }
                        }}
                        append_log(&format!("{}\n", s));
                        dumped_names = true;
                    }
                }
                // caster 잡히면(엔티티 바뀌면 재설치) caster+watch_off에 watchpoint. WP_FOUND 후에도 더 많은
                // distinct writer 캡처 위해 설치 유지(재설치만 멈춤).
                {
                    let tgt = WP_CAST_TARGET.load(Ordering::Relaxed);
                    let mk = if ptr_ok(tgt) { Some(tgt) } else {
                        let pb = PLAN_BASE.load(Ordering::Relaxed);
                        if ptr_ok(pb) { find_magic_knight(pb) } else { None }
                    };
                    if let Some(mk) = mk {
                        let addr = mk + WP_WATCH_OFF.load(Ordering::Relaxed);
                        if addr != WP_ADDR.load(Ordering::Relaxed) { wp_install(addr); }
                    }
                }
                // 새 distinct writer가 늘면 전체 목록 재덤프(RVA + 그때 필드값 + VM안/밖 분류).
                let n = WP_NHITS.load(Ordering::Relaxed);
                if n > last_n {
                    let base = exe_base();
                    let off = WP_WATCH_OFF.load(Ordering::Relaxed);
                    let mut s = format!("[wp] ★★★ +0x{:x} writers ({} distinct):", off, n);
                    let mut i = 0; while i < n && i < WP_CAP {
                        let rip = WP_HIT_RIP[i].load(Ordering::Relaxed) as usize;
                        let val = WP_HIT_VAL[i].load(Ordering::Relaxed);
                        let rva = rip.wrapping_sub(base);
                        let invm = rva >= 0x1e41800 && rva < 0x1e48340;
                        s.push_str(&format!("  [rva=0x{:x} val={} {}]", rva, val, if invm { "VM" } else { "★EXT" }));
                        i += 1;
                    }
                    append_log(&format!("{}\n", s));
                    last_n = n;
                }
            }
        }
    });
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    load_cfg();
    if let Some(p) = pth("tfm2_aim_lead.txt") {
        let _ = fs::write(&p, format!("[{}ms] === tfm2_aim_lead INIT (Phase4: casting-block lead) ===\n", now_ms()));
    }
    unsafe {
        match install_mid_hook(RVA_AIM_PATCH, AIM_ORIG_LEN, &AIM_EXPECT, aim_cap as *const () as usize) {
            Ok(())  => append_log("[hook] LINEAR @0x18df118 (14B) OK\n"),
            Err(e)  => append_log(&format!("[hook] LINEAR 실패: {}\n", e)),
        }
        match install_mid_hook(RVA_LR_PATCH, LR_ORIG_LEN, &LR_EXPECT, aim_cap_lr as *const () as usize) {
            Ok(())  => append_log("[hook] LINE_RANGE @0x20f9b68 (14B) OK = magic_knight skill1\n"),
            Err(e)  => append_log(&format!("[hook] LINE_RANGE 실패: {}\n", e)),
        }
        match install_entry_detour(RVA_MOVE, MOVE_ORIG_LEN, &MOVE_EXPECT, sampler_cap as *const () as usize) {
            Ok(())  => append_log("[hook] SAMPLER @0x18d0190 (12B, r8=entity) OK\n"),
            Err(e)  => append_log(&format!("[hook] SAMPLER 실패: {}\n", e)),
        }
        match install_entry_detour(RVA_LR_APPLY, LR_APPLY_LEN, &LR_APPLY_EXPECT, lr_apply_cap as *const () as usize) {
            Ok(())  => append_log("[hook] LR_APPLY diag @0x20f9ab0 (12B) OK\n"),
            Err(e)  => append_log(&format!("[hook] LR_APPLY diag 실패: {}\n", e)),
        }
        // DISPATCH 훅(0x1e41800)은 rcx≠casting엔티티로 판명 → 미설치. 리드는 apply(param_7 주입)서 처리.
        let _ = (RVA_DISPATCH, DISP_LEN, &DISP_EXPECT, dispatch_cap as *const () as usize);
    }
    if WP_FIND.load(Ordering::Relaxed) {
        append_log("[wp] wp_find=1 → 시전 setter 발견 스레드 가동(magic_knight +0xb0 watchpoint)\n");
        wp_spawn_discovery();
    }
    ModRegistration::new(MOD_ID)
}
declare_mod!(init);
