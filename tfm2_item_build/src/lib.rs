//! tfm2_item_build — 아이템 빌드 적용 계층 (스테이블 ABI, 재구현 1차 = 측정판)
//!
//! 역할 분담
//!   `tfm2_item_tactics`(legacy ABI, 유지) : 전술화면 UI + 모드아이템 열거 + 선택 저장
//!                                           → `item_tactics_sel.txt` (`@<b|r>:<champ> <slot> <item_key>`)
//!   `tfm2_item_build`  (이 모드, stable)   : 그 파일을 읽어 **실제 빌드에 적용**
//!
//! 왜 나눴나: 한 DLL 은 legacy/stable ABI 를 겸할 수 없다
//! (`tfm2_mod_entry_stable` 이 있으면 로더가 legacy 경로를 쓰지 않는다 — SDK entry.rs 주석).
//!
//! ── 1차(이 파일) = **측정만 한다. 아무것도 쓰지 않는다.** ──────────────────
//!   M1 `SimCtxV1{size,sim,frame,state}` 실값 + `state` 가 가리키는 것의 정체
//!      → `GamePlayer(+0x4e0 item_builds / +0x920 athlete_id / +0x930 team)` 까지
//!        오프셋만으로 뚫리는지(=RVA 0 으로 끝나는지) 판정하기 위한 재료
//!   M2 `sim_origin` 게이트 실증 — ClientMatchView(2) vs ServerPresim(1) 짝
//!   M3 스테이블 API 로 본 플레이어 상태(id/team/lane/champion/보유템)와 M1 덤프의 교차대조
//!   M4 `decide_build` 가 화면경기(kind=2)에 대해 따로 도는지 (06 문서 미확정 ①)
//!   M5 `item_tactics_sel.txt` 파싱 검증
//!
//! 안전: 모든 원시 읽기는 `VirtualQuery` 로 커밋·읽기가능 확인 후에만 한다(AV 회피).
//!       콜백 본문은 전부 `catch_unwind`. `on_match_tick` 은 무동작(매 틱 호출).
//!
//! 로그: `<게임설치>\mods\tfm2_item_build\item_build.log` (cfg `log = 1` 일 때만)

use std::ffi::c_void;
use std::io::Write as _;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

use mod_api_stable::{
    entry_shim, resolve_required_level, DraftDecisionKindV1, HostApiV1, ItemBuildCtxV1,
    ItemBuildHookRegV1, ItemBuildHookVtableV1, LogLevel, MatchHookRegV1, MatchHookVtableV1,
    ModExportV1, SimCtxV1, SimOriginKindV1, StableClient, StableExtension, StableHost,
    StableItemBuildContext, StableMod, StableSim, StrV1,
};

const MOD_ID: &str = "tfm2_item_build";
/// 선택 파일을 만드는 legacy 모드.
const SEL_MOD_ID: &str = "tfm2_item_tactics";
const DETAIL_LIMIT: usize = 4000;

// ---------------------------------------------------------------------------
// 경로 — ★게임 exe 기준 동적 도출 (`/deploy` §4: install 경로 하드코딩 금지)
//   하드코딩하면 다른 드라이브·설치위치 유저에게서 **조용히 실패**한다(크래시 없음).
// ---------------------------------------------------------------------------

fn game_root() -> Option<PathBuf> {
    let mut buf = [0u16; 1024];
    let n = unsafe {
        GetModuleFileNameW(
            GetModuleHandleW(core::ptr::null()),
            buf.as_mut_ptr(),
            buf.len() as u32,
        )
    };
    if n == 0 {
        return None;
    }
    PathBuf::from(String::from_utf16_lossy(&buf[..n as usize]))
        .parent()
        .map(|p| p.to_path_buf())
}

/// (로그, sel, cfg) — 1회 도출 후 캐시.
static PATHS: OnceLock<Option<(PathBuf, PathBuf, PathBuf)>> = OnceLock::new();
fn paths() -> Option<&'static (PathBuf, PathBuf, PathBuf)> {
    PATHS
        .get_or_init(|| {
            let m = game_root()?.join("mods");
            Some((
                m.join(MOD_ID).join("item_build.log"),
                m.join(SEL_MOD_ID).join("item_tactics_sel.txt"),
                m.join(MOD_ID).join("item_build.cfg"),
            ))
        })
        .as_ref()
}
fn sel_path_str() -> String {
    paths()
        .map(|p| p.1.display().to_string())
        .unwrap_or_else(|| "<경로 도출 실패>".into())
}

// ---------------------------------------------------------------------------
// 로깅
// ---------------------------------------------------------------------------

static LOG_LOCK: Mutex<()> = Mutex::new(());
static START: OnceLock<Instant> = OnceLock::new();

fn ms() -> u128 {
    START.get_or_init(Instant::now).elapsed().as_millis()
}

/// 진단 로그 스위치 — cfg `log = 1` 일 때만 켠다. **프로덕션 기본 OFF.**
static LOG_ON: AtomicBool = AtomicBool::new(false);

fn logline(s: &str) {
    if !LOG_ON.load(Ordering::Relaxed) {
        return;
    }
    let Some(pp) = paths() else { return };
    let _g = LOG_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&pp.0) {
        let _ = writeln!(f, "{s}");
        let _ = f.flush();
    }
}

fn log_once(flag: &AtomicBool, s: &str) {
    if !flag.swap(true, Ordering::Relaxed) {
        logline(s);
    }
}

// ---------------------------------------------------------------------------
// 안전한 원시 메모리 읽기 (VirtualQuery 검증 → AV 없이 실패 반환)
// ---------------------------------------------------------------------------

#[repr(C)]
struct MemoryBasicInformation {
    base_address: *mut c_void,
    allocation_base: *mut c_void,
    allocation_protect: u32,
    __alignment1: u32,
    region_size: usize,
    state: u32,
    protect: u32,
    type_: u32,
    __alignment2: u32,
}

#[link(name = "kernel32")]
extern "system" {
    fn VirtualQuery(
        address: *const c_void,
        buffer: *mut MemoryBasicInformation,
        length: usize,
    ) -> usize;
    fn GetProcessHeap() -> *mut c_void;
    fn GetModuleFileNameW(module: *mut c_void, buf: *mut u16, size: u32) -> u32;
    fn GetModuleHandleW(name: *const u16) -> *mut c_void;
    /// 유효한 블록이면 크기, 아니면 usize::MAX
    fn HeapSize(heap: *mut c_void, flags: u32, mem: *const c_void) -> usize;
    fn HeapValidate(heap: *mut c_void, flags: u32, mem: *const c_void) -> i32;
}

/// ★할당자 호환성 측정 — 게임의 Vec 버퍼가 프로세스 힙에서 왔는지.
/// Rust 의 Windows System 할당자는 HeapAlloc(GetProcessHeap()) 이므로,
/// 게임 버퍼가 프로세스 힙 블록이면 우리가 만든 Vec 을 게임이 HeapFree 해도 안전하다.
fn heap_probe(ptr: usize) -> (i64, bool) {
    if ptr < 0x10000 {
        return (-1, false);
    }
    unsafe {
        let h = GetProcessHeap();
        if h.is_null() {
            return (-1, false);
        }
        let valid = HeapValidate(h, 0, ptr as *const c_void) != 0;
        let sz = HeapSize(h, 0, ptr as *const c_void);
        (if sz == usize::MAX { -1 } else { sz as i64 }, valid)
    }
}

const MEM_COMMIT: u32 = 0x1000;
const MEM_PRIVATE: u32 = 0x20000;
const PAGE_GUARD: u32 = 0x100;
const PAGE_NOACCESS: u32 = 0x01;
const READABLE: u32 = 0x02 /*R*/ | 0x04 /*RW*/ | 0x08 /*WC*/ | 0x20 /*ER*/ | 0x40 /*ERW*/ | 0x80 /*ERWC*/;

/// `addr..addr+len` 이 전부 커밋·읽기가능한 한 리전 안에 있는지 검사.
fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 || addr.checked_add(len).is_none() {
        return false;
    }
    let mut mbi: MemoryBasicInformation = unsafe { std::mem::zeroed() };
    let n = unsafe {
        VirtualQuery(
            addr as *const c_void,
            &mut mbi,
            std::mem::size_of::<MemoryBasicInformation>(),
        )
    };
    if n == 0 || mbi.state != MEM_COMMIT {
        return false;
    }
    if mbi.protect & (PAGE_GUARD | PAGE_NOACCESS) != 0 || mbi.protect & READABLE == 0 {
        return false;
    }
    let end = mbi.base_address as usize + mbi.region_size;
    addr + len <= end
}

fn read_usize(addr: usize) -> Option<usize> {
    if !readable(addr, 8) {
        return None;
    }
    Some(unsafe { std::ptr::read_unaligned(addr as *const usize) })
}

/// exe 모듈 [base, base+size) — 포인터가 코드/정적 영역인지 분류용.
fn exe_range() -> (usize, usize) {
    static R: OnceLock<(usize, usize)> = OnceLock::new();
    *R.get_or_init(|| {
        let base = unsafe { GetModuleHandleW(std::ptr::null()) } as usize;
        if base == 0 {
            return (0, 0);
        }
        // PE: e_lfanew@0x3c → NT headers → OptionalHeader.SizeOfImage @ +0x50
        let size = read_usize(base + 0x3c)
            .map(|v| (v & 0xffff_ffff) as usize)
            .and_then(|lfanew| read_usize(base + lfanew + 0x50))
            .map(|v| (v & 0xffff_ffff) as usize)
            .unwrap_or(0);
        (base, size)
    })
}

/// 한 qword 를 사람이 읽을 수 있게 분류한다.
fn classify(v: usize) -> String {
    let (b, s) = exe_range();
    if b != 0 && s != 0 && v >= b && v < b + s {
        return format!("0x{v:016x}  exe+0x{:x}", v - b);
    }
    if v >= 0x10000 && readable(v, 8) {
        return format!("0x{v:016x}  ptr(읽기가능)");
    }
    if v < 0x100000 {
        return format!("0x{v:016x}  int={v}");
    }
    format!("0x{v:016x}")
}

/// `addr` 부터 `n` qword 를 분류해 로그로 남긴다.
fn dump_qwords(tag: &str, addr: usize, n: usize) {
    if !readable(addr, 8) {
        logline(&format!("  {tag} @0x{addr:016x} — 읽기 불가"));
        return;
    }
    logline(&format!("  {tag} @0x{addr:016x}"));
    for i in 0..n {
        match read_usize(addr + i * 8) {
            Some(v) => logline(&format!("    +0x{:03x}  {}", i * 8, classify(v))),
            None => {
                logline(&format!("    +0x{:03x}  (읽기 불가 — 중단)", i * 8));
                break;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// GamePlayer 배열 자기검증 스캔
//   `state+0x00` = 게임 객체(&mut dyn ... 팻포인터의 data). 그 안에서 players Vec 을 찾는다.
//   추측으로 고르지 않고 **team 값 지문**(0 다섯 · 1 다섯)으로 (base, stride) 를 증명한다.
//   기대 오프셋(distruct GamePlayer, 0.5.8):
//     +0x4e0 item_builds Vec{cap@0,ptr@8,len@16}  +0x920 athlete_id  +0x930 team  +0x9c0 position
// ---------------------------------------------------------------------------

const OFF_STATE_CACHE: usize = 0x00;      // SimCtxV1.state → AbstractGameWithCache
const OFF_CACHE_GAME: usize = 0x00;       // AbstractGameWithCache.game (&dyn) 의 data ptr
const OFF_WORLD_PLAYERS: usize = 0x850;   // Game(=World at +0).players: Container<PlayerState>.data(Vec)
const OFF_SIM_ORIGIN: usize = 0xecc8;     // Game.sim_origin
const OFF_SIMORIGIN_KIND: usize = 0x30;   // SimOrigin.kind (u8)
const STRIDE_PLAYERSTATE: usize = 0x9e0;  // PlayerState 크기 2528
// PlayerState.info: GamePlayer 안쪽
const OFF_CHAMPION_NAME: usize = 0x468; // GamePlayer.champion_name: String{cap,ptr,len}
const OFF_OWNED_ITEMS: usize = 0x498; // GamePlayer.items: Vec<Box<dyn ItemInfo>> (보유템)
const OFF_ITEM_BUILDS: usize = 0x4e0;
const OFF_ATHLETE_ID: usize = 0x920;
const OFF_TEAM: usize = 0x930;
const OFF_POSITION: usize = 0x9c0;

/// DWARF 로 확정한 경로를 그대로 탄다(추측 스캔 아님).
///   state+0x00 → AbstractGameWithCache
///   +0x00      → &dyn AbstractGame 팻포인터 data = Game
///   Game+0x850 → World.players = Container<PlayerState> { data: Vec{cap,ptr,len} }
///   Game+0xecc8+0x30 = sim_origin.kind  ← 스테이블 API 값과 대조해 경로를 자기검증
fn resolve_players(state: usize, expect_kind: u32, expect_len: usize) -> Option<(usize, usize, usize, usize)> {
    // 후보: ①state+0x00 이 곧 Game  ②한 겹 더(래퍼가 낀 경우)
    //   어느 쪽이 맞는지는 Game.sim_origin.kind 가 스테이블 API 값과 같은지로 가른다.
    let mut cands: Vec<(usize, &str)> = Vec::new();
    if let Some(g) = read_usize(state + 0x00) {
        if g >= 0x10000 {
            cands.push((g, "state+0x00"));
            if let Some(g2) = read_usize(g + 0x00) {
                if g2 >= 0x10000 {
                    cands.push((g2, "state+0x00 → +0x00"));
                }
            }
        }
    }
    if cands.is_empty() {
        logline("  [M6] state+0x00 읽기 실패");
        return None;
    }
    let mut game = 0usize;
    for (g, how) in &cands {
        if !readable(*g, OFF_SIM_ORIGIN + 0x40) {
            logline(&format!("  [M6] 후보 {how}=0x{g:012x} — 범위 부족"));
            continue;
        }
        let kind = read_usize(*g + OFF_SIM_ORIGIN + OFF_SIMORIGIN_KIND).map(|v| (v & 0xff) as u32);
        logline(&format!(
            "  [M6] 후보 {how}=0x{g:012x} sim_origin.kind={:?} (기대 {expect_kind})",
            kind
        ));
        if kind == Some(expect_kind) {
            game = *g;
            break;
        }
    }
    if game == 0 {
        logline("  [M6] ★어느 후보도 sim_origin.kind 검증을 통과하지 못했다");
        return None;
    }
    let cap = read_usize(game + OFF_WORLD_PLAYERS)?;
    let ptr = read_usize(game + OFF_WORLD_PLAYERS + 8)?;
    let len = read_usize(game + OFF_WORLD_PLAYERS + 16)?;
    if ptr < 0x10000 || len == 0 || len > 64 || cap < len || !readable(ptr, len * STRIDE_PLAYERSTATE) {
        logline(&format!(
            "  [M6] players Vec 이상: cap={cap} ptr=0x{ptr:012x} len={len}"
        ));
        return None;
    }
    if len != expect_len {
        logline(&format!("  [M6] ⚠len={len} ≠ sim.player_count()={expect_len} (계속 진행)"));
    }
    logline(&format!(
        "  [M6] ★경로 검증 통과: game=0x{game:012x} kind={expect_kind} | players cap={cap} ptr=0x{ptr:012x} len={len} stride=0x{STRIDE_PLAYERSTATE:x}"
    ));
    Some((game, ptr, len, cap))
}

/// 찾은 배열을 사람이 읽을 수 있게 찍는다.
fn log_players(base: usize, len: usize) {
    for i in 0..len.min(16) {
        let p = base + i * STRIDE_PLAYERSTATE;
        let team = read_usize(p + OFF_TEAM).map(|v| v as i64).unwrap_or(-1);
        let ath = read_usize(p + OFF_ATHLETE_ID).map(|v| v as i64).unwrap_or(-1);
        let pos = read_usize(p + OFF_POSITION).map(|v| (v & 0xffff_ffff) as i64).unwrap_or(-1);
        let cap = read_usize(p + OFF_ITEM_BUILDS).unwrap_or(0);
        let bp = read_usize(p + OFF_ITEM_BUILDS + 8).unwrap_or(0);
        let blen = read_usize(p + OFF_ITEM_BUILDS + 16).unwrap_or(0);
        let mut items = Vec::new();
        for k in 0..blen.min(8) {
            if let Some(v) = read_usize(bp + k * 8) {
                items.push(v);
            }
        }
        let (hsz, hvalid) = heap_probe(bp);
        logline(&format!(
            "   [GP{i}] @0x{p:012x} team={team} athlete_id={ath} pos={pos} item_builds(cap={cap},len={blen})={items:?} buf=0x{bp:012x} heap_size={hsz} valid={hvalid}"
        ));
    }
}

// ---------------------------------------------------------------------------
// item_tactics_sel.txt 파싱  (`@<b|r>:<champion_key> <slot> <item_key>`)
// ---------------------------------------------------------------------------

/// 바닐라 카테고리(ItemBuildOverride) → 최종템 key. 0=Auto 는 "지정 없음".
const VANILLA_CAT_KEY: [&str; 7] = [
    "",                          // 0 Auto
    "warlords_final_judgement",  // 1 AD
    "prophet_of_the_abyss",      // 2 Magic
    "storm_sovereign",           // 3 AttackSpeed
    "impregnable_fortress",      // 4 Defense
    "veil_of_annihilation",      // 5 MagicResistance
    "giants_horn_shard",         // 6 Hp
];

/// `item_tactics_sel.txt` 포맷: `<champ> <slot 0..3> <token>`
///   · 접두 없음      = 리그/관전/배경 (★우리가 쓰는 것)
///   · `@b:` / `@r:`  = 조합테스트 진영 스코프 (여기선 무시)
///   · token = 아이템 key 문자열, 또는 바닐라 카테고리 숫자 0~6
/// 스코프: 0=plain(리그/관전/배경) · 1=조합테스트 블루(`@b:`) · 2=조합테스트 레드(`@r:`)
/// ★값이 Vec 인 이유(2026-09-13): 같은 (스코프,챔프,슬롯)에 행이 **여러 개** 있을 수 있다 —
///   legacy 가 해석 못 한 옛 지정(예: 롤아이템모드 OFF 후의 `radiant_*`)을 pending 원문으로 보존한 채
///   유저의 새 지정을 같은 칸에 또 쓰던 실사고. 구 코드는 HashMap 덮어쓰기라 **뒤 행이 이겨** 유저 지정이
///   조용히 무시됐다. 이제 후보를 전부 담고, 적용 시 **해석되는 첫 토큰**을 쓴다(파일 순서 유지).
fn load_selections() -> std::collections::HashMap<(u8, String, u8), Vec<String>> {
    let mut out: std::collections::HashMap<(u8, String, u8), Vec<String>> =
        std::collections::HashMap::new();
    let Some(pp) = paths() else { return out };
    let Ok(text) = std::fs::read_to_string(&pp.1) else {
        return out;
    };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut it = line.split_whitespace();
        let (Some(head), Some(slot), Some(token)) = (it.next(), it.next(), it.next()) else {
            continue;
        };
        let (scope, champ) = if let Some(r) = head.strip_prefix("@b:") {
            (1u8, r)
        } else if let Some(r) = head.strip_prefix("@r:") {
            (2u8, r)
        } else if head.starts_with('@') {
            continue; // 미지의 스코프
        } else {
            (0u8, head)
        };
        let Ok(slot) = slot.parse::<u8>() else { continue };
        if slot >= 4 || champ.is_empty() {
            continue;
        }
        let e = out.entry((scope, champ.to_string(), slot)).or_default();
        if !e.iter().any(|t| t == token) {
            e.push(token.to_string());
        }
    }
    out
}

/// token → 실제 아이템 key. 숫자면 바닐라 카테고리, 아니면 그대로.
fn token_to_key(token: &str) -> Option<&str> {
    if let Ok(n) = token.parse::<usize>() {
        let k = VANILLA_CAT_KEY.get(n).copied().unwrap_or("");
        return if k.is_empty() { None } else { Some(k) };
    }
    Some(token)
}

/// `p` 에 있는 Rust String{cap,ptr,len} 을 읽는다.
fn read_string_at(p: usize) -> Option<String> {
    let ptr = read_usize(p + 8)?;
    let len = read_usize(p + 16)?;
    if len == 0 || len > 256 || ptr < 0x10000 || !readable(ptr, len) {
        return None;
    }
    let mut b = Vec::with_capacity(len);
    for i in 0..len {
        b.push(unsafe { std::ptr::read_unaligned((ptr + i) as *const u8) });
    }
    String::from_utf8(b).ok()
}

/// 내 팀 선발 5명 athlete_id. 클라 확장이 갱신한다.
static MY_ATHLETES: Mutex<Vec<usize>> = Mutex::new(Vec::new());
/// 전역 아이템 key 목록(엔진 순서). `ItemBuildCtxV1.item_keys` 에서 캐시한다 —
/// decide_build 는 배경 경기 때문에 상시 돌아서 화면 경기 전에 반드시 채워진다.
static ITEM_KEYS: Mutex<Vec<String>> = Mutex::new(Vec::new());
/// 화면 경기의 players 배열 (base, len). check_match_end 샘플링용.
static LIVE_PLAYERS: Mutex<Option<(usize, usize)>> = Mutex::new(None);
static TICK_SAMPLES: AtomicUsize = AtomicUsize::new(0);
/// write 를 실제로 수행할지. `item_build.cfg` 의 `write = 1` 로 켠다(기본 드라이런).
static WRITE_ENABLED: AtomicBool = AtomicBool::new(false);

/// `item_build.cfg` 를 읽어 `(write, log)` 를 돌려주고 `LOG_ON` 을 갱신한다.
/// 매 경기 재읽기 — cfg 만 고치면 게임 재시작 없이 토글된다.
fn load_cfg() -> (bool, bool) {
    let Some(pp) = paths() else { return (false, false) };
    let Ok(t) = std::fs::read_to_string(&pp.2) else { return (false, false) };
    let (mut w, mut l) = (false, false);
    for line in t.lines() {
        let s = line.trim();
        if s.is_empty() || s.starts_with('#') {
            continue;
        }
        let Some((k, v)) = s.split_once('=') else { continue };
        match k.trim() {
            "write" => w = v.trim().starts_with('1'),
            "log" => l = v.trim().starts_with('1'),
            _ => {}
        }
    }
    LOG_ON.store(l, Ordering::Relaxed);
    (w, l)
}

fn load_write_flag() -> bool {
    load_cfg().0
}

// ---------------------------------------------------------------------------
// 카운터
// ---------------------------------------------------------------------------

static DECIDE_CALLS: AtomicUsize = AtomicUsize::new(0);
static MATCH_STARTS: AtomicUsize = AtomicUsize::new(0);
static DUMPED: AtomicUsize = AtomicUsize::new(0);
static IB_ID_LOGGED: AtomicBool = AtomicBool::new(false);
static CAPS: Mutex<Vec<usize>> = Mutex::new(Vec::new());
/// 최근 ClientMatchView 발생 시각 — decide_build 가 그 근처에 도는지 보려고.
static LAST_CMV_MS: AtomicUsize = AtomicUsize::new(0);

// ---------------------------------------------------------------------------
// raw MatchHook vtable — ★`*mut SimCtxV1` 을 그대로 받기 위해 직접 구현
//   (안전 래퍼 `StableMatchHook` 은 SimCtxV1 을 감춰서 `state` 에 접근할 수 없다)
// ---------------------------------------------------------------------------

unsafe extern "C" fn mh_destroy(_userdata: *mut c_void) {
    logline("[MH] destroy");
}

unsafe extern "C" fn mh_on_match_start(_userdata: *mut c_void, ctx: *mut SimCtxV1) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        let n = MATCH_STARTS.fetch_add(1, Ordering::Relaxed) + 1;
        if ctx.is_null() {
            logline(&format!("[MS] #{n} ctx=null"));
            return;
        }
        let raw = &*ctx;
        let Some(sim) = StableSim::from_raw(ctx) else {
            logline(&format!("[MS] #{n} StableSim::from_raw 실패 (size={})", raw.size));
            return;
        };
        let origin = sim.sim_origin();
        let (kind, mid, sidx) = match origin {
            Some(o) => (o.kind, o.match_id, o.set_index),
            None => (u32::MAX, u64::MAX, u64::MAX),
        };
        let f = |v: u64| if v == u64::MAX { "-".to_string() } else { v.to_string() };
        // ★게이트: 화면에 보이는 경기 = ClientMatchView(내 팀) | ClientSpectate(남의 경기).
        //   Spectate 는 내 선수가 없어 팀 게이트에서 자연히 걸러지므로 넓혀도 안전하다.
        let is_view = kind == SimOriginKindV1::ClientMatchView as u32
            || kind == SimOriginKindV1::ClientSpectate as u32;
        // ★조합 테스트 = sim_origin 이 안 붙는다(kind=Unknown, 실측). 밴픽 롤아웃도 Unknown 이라
        //   추가 지문이 필요하다: **양 팀 전원이 내 선수**(롤아웃은 athlete_id 가 0/1 미기입).
        let is_unknown = kind == SimOriginKindV1::Unknown as u32;
        let is_cmv = is_view || is_unknown; // 실제 판정은 resolve 뒤 comptest 검사로 좁힌다
        if is_cmv {
            LAST_CMV_MS.store(ms() as usize, Ordering::Relaxed);
        }

        // ── M2: sim_origin 게이트 ──
        logline(&format!(
            "[MS] #{n} t={}ms kind={kind}{} match_id={} set_index={} players={} tick={}",
            ms(),
            match kind {
                0 => " Unknown",
                1 => " ServerPresim",
                2 => " ★ClientMatchView",
                3 => " ★ClientSpectate",
                4 => " ClientReplay",
                5 => " Tool",
                _ => " ?",
            },
            f(mid),
            f(sidx),
            sim.player_count(),
            sim.tick(),
        ));

        // ── M3: 스테이블 API 로 본 플레이어 상태 (화면경기 + 앞 2건만) ──
        let want_detail = is_cmv || n <= 2;
        if want_detail {
            for i in 0..sim.player_count() {
                if let Some(p) = sim.player_at(i) {
                    let champ = p
                        .champion()
                        .and_then(|e| e.name())
                        .unwrap_or_else(|| "?".to_string());
                    logline(&format!(
                        "   [P{i}] id={} team={} lane={:?} champ={} gold={} items={:?}",
                        p.id(),
                        p.team(),
                        p.lane(),
                        champ,
                        p.gold(),
                        p.item_keys(),
                    ));
                }
            }
        }

        // ── M6/M8: players 해석 → 지정 계획 산출 → (드라이런 | 쓰기) ──
        if is_cmv || n <= 3 {
            if let Some((_game, base, plen, pcap)) =
                resolve_players(raw.state as usize, kind, sim.player_count())
            {
                log_players(base, plen);
                if is_cmv {
                    // ★스캔은 "실제로 쓸 경기"에서만 — 밴픽 롤아웃·메뉴 배경에서 돌리면
                    //   세이브 로드 전이라 신경망이 없어 헛스캔으로 시도만 소모한다.
                    let real = is_view || looks_like_comptest(base, plen, pcap);
                    let net = if real { scan_for_net(base) } else { None };
                    let na = ACTIVE_FINALS.lock().unwrap_or_else(|e| e.into_inner()).len();
                    logline(&format!(
                        "  [NET] net={:x?} fn_ok={} | 활성 최종템 후보 {}개 | fwd호출 {} / stale {}",
                        net, itemnet_fn_ok(), na,
                        FWD_CALLS.load(Ordering::Relaxed), NET_STALE.load(Ordering::Relaxed)
                    ));
                    apply_or_dryrun(base, plen, net.unwrap_or(0), is_unknown, pcap);
                }
            }
        }

        // ── M1: SimCtxV1 + state 정체 (최초 2회, 그리고 첫 ClientMatchView 1회) ──
        let d = DUMPED.load(Ordering::Relaxed);
        if d < 2 || (is_cmv && d < 3) {
            DUMPED.fetch_add(1, Ordering::Relaxed);
            let (eb, es) = exe_range();
            logline(&format!(
                "  [M1] exe base=0x{eb:016x} size=0x{es:x} | SimCtxV1 @0x{:016x} size={} sim={} frame={} state={}",
                ctx as usize,
                raw.size,
                classify(raw.sim as usize),
                classify(raw.frame as usize),
                classify(raw.state as usize),
            ));
            dump_qwords("[M1] *state", raw.state as usize, 40);
            if let Some(p0) = read_usize(raw.state as usize) {
                if p0 >= 0x10000 && readable(p0, 8) {
                    dump_qwords("[M1] **state[0] (게임객체)", p0, 64);
                }
            }
        }
    }));
}

unsafe extern "C" fn mh_on_match_tick(_userdata: *mut c_void, _ctx: *mut SimCtxV1, _seed: u64) {
    // 매 틱 호출 — 절대 아무것도 하지 않는다.
}

unsafe extern "C" fn mh_check_match_end(
    _userdata: *mut c_void,
    _ctx: *mut SimCtxV1,
    _out_blue_win: *mut bool,
) -> bool {
    // ★매 틱 호출 — 아주 드물게만 샘플링한다(보유템이 4개까지 가는지 확인용).
    let k = TICK_SAMPLES.fetch_add(1, Ordering::Relaxed);
    if k % 3000 == 0 {
        let _ = catch_unwind(AssertUnwindSafe(|| {
            let lp = *LIVE_PLAYERS.lock().unwrap_or_else(|e| e.into_inner());
            let Some((base, plen)) = lp else { return };
            let mine = MY_ATHLETES.lock().unwrap_or_else(|e| e.into_inner()).clone();
            let mut row = String::new();
            for i in 0..plen {
                let p = base + i * STRIDE_PLAYERSTATE;
                let Some(ath) = read_usize(p + OFF_ATHLETE_ID) else { continue };
                if !mine.contains(&ath) {
                    continue;
                }
                let owned = read_usize(p + OFF_OWNED_ITEMS + 16).unwrap_or(usize::MAX);
                let blen = read_usize(p + OFF_ITEM_BUILDS + 16).unwrap_or(usize::MAX);
                row.push_str(&format!("ath{ath}:보유{owned}/빌드{blen} "));
            }
            if !row.is_empty() {
                logline(&format!("[TK] 샘플#{} {row}", k / 3000));
            }
        }));
    }
    false // 경기 결과에 개입하지 않는다
}

static MH_VTABLE: MatchHookVtableV1 = MatchHookVtableV1 {
    size: std::mem::size_of::<MatchHookVtableV1>(),
    destroy: Some(mh_destroy),
    on_match_start: Some(mh_on_match_start),
    on_match_tick: Some(mh_on_match_tick),
    check_match_end: Some(mh_check_match_end),
};

// ---------------------------------------------------------------------------
// 아이템 신경망 (A안) — net 포인터 확보 + 활성 최종템 후보 수집
//   forward(net, ctx: &[u64;11], build_ptr, build_len, flag=0) -> f32
//   ctx[0..5]=아군 champ id / ctx[5..10]=적 / ctx[10]=포지션(0~4)
//   RVA 0x11e1b10 = 0.5.8 유효(repin_058: bytes_same/fn_start_new, 프롤로그 push8)
// ---------------------------------------------------------------------------

const ITEMNET_FORWARD_RVA: usize = 0x11e1b10;
const NET_PROLOGUE: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];

static ITEMNET: AtomicUsize = AtomicUsize::new(0);
/// 힙 스캔 시도 횟수. ★일회성이면 안 된다 —
/// 첫 시도는 게임 시작 직후(메뉴 배경 시뮬)에 걸리는데 그때는 세이브를 불러오기 전이라
/// 아이템 신경망이 아직 메모리에 없다(실측 2026-09-11: 리전 1739 / 2.7GB 훑어 후보 0개,
/// 그 뒤 1350회 전부 net=None, AUTO4 발동 0회).
static NET_SCAN_TRIES: AtomicUsize = AtomicUsize::new(0);
/// 스캔은 2초 넘게 걸려 경기 시작을 지연시킨다 ⟹ 시도 횟수를 묶는다.
const NET_SCAN_MAX_TRIES: usize = 6;
/// 엔진이 score_item 후보로 넘겨준 인덱스 = **활성 tier>=4 최종템**. is_active 대용.
static ACTIVE_FINALS: Mutex<Vec<usize>> = Mutex::new(Vec::new());

/// forward 함수 진입부가 기대 프롤로그인지.
fn itemnet_fn_ok() -> bool {
    let (b, _) = exe_range();
    if b == 0 { return false; }
    let a = b + ITEMNET_FORWARD_RVA;
    if !readable(a, 12) { return false; }
    for i in 0..12 {
        if unsafe { std::ptr::read_unaligned((a + i) as *const u8) } != NET_PROLOGUE[i] {
            return false;
        }
    }
    true
}

/// net 시그니처: [0]=16384, [2]=16384, [3]=1, [1]=가중치 ptr(16384*4 읽기가능)
fn net_sig_at(a: usize) -> bool {
    if !readable(a, 0x20) { return false; }
    let q = |i: usize| read_usize(a + i * 8).unwrap_or(0);
    if q(0) != 16384 || q(2) != 16384 || q(3) != 1 { return false; }
    let w = q(1);
    w >= 0x10000 && readable(w, 16384 * 4)
}

/// ★2026-09-13: 스캔을 **경기 시작 경로에서 뺀다**. 실측 3.9GB/5.8초(성공 시)·실패 시 최대 20초×6회가
///   `on_match_start` 에서 동기로 돌아 "경기 시작 누르면 한참 뒤에 시작"(유저 제보)의 원인이었다.
///   이제 스캔은 백그라운드 스레드가 하고(세이브 로드 후 메뉴에서 미리, post_update 가 띄움),
///   결과 후보(NET_HITS)만 저장한다. 경기 시작 시엔 후보에서 heap_hint 로 고르기만 한다(µs).
static NET_HITS: Mutex<Vec<usize>> = Mutex::new(Vec::new());
static NET_SCAN_BUSY: AtomicBool = AtomicBool::new(false);
static NET_SCAN_LAST_MS: AtomicUsize = AtomicUsize::new(0);
/// 백그라운드 재시도 간격(세이브 로드 전엔 net 이 없어 헛스캔이므로 띄엄띄엄).
const NET_SCAN_COOLDOWN_MS: usize = 20_000;

/// 백그라운드 스캔 1회 기동(이미 찾았거나·돌고 있거나·시도 상한이면 무동작).
fn spawn_net_scan(reason: &str) {
    if ITEMNET.load(Ordering::Relaxed) != 0 { return; }
    if NET_SCAN_TRIES.load(Ordering::Relaxed) >= NET_SCAN_MAX_TRIES { return; }
    if NET_SCAN_BUSY.swap(true, Ordering::AcqRel) { return; }
    NET_SCAN_TRIES.fetch_add(1, Ordering::Relaxed);
    NET_SCAN_LAST_MS.store(ms() as usize, Ordering::Relaxed);
    let why = reason.to_string();
    let _ = std::thread::Builder::new()
        .name("tfm2_item_build_netscan".into())
        .spawn(move || {
            let _ = catch_unwind(AssertUnwindSafe(|| {
                let hits = scan_heap_for_hits(&why);
                if !hits.is_empty() {
                    *NET_HITS.lock().unwrap_or_else(|e| e.into_inner()) = hits;
                }
            }));
            NET_SCAN_BUSY.store(false, Ordering::Release);
        });
}

/// 저장된 후보 중에서 고른다(경기 시작 경로 — 스캔 안 함). 없으면 백그라운드 스캔을 띄우고 None.
fn scan_for_net(heap_hint: usize) -> Option<usize> {
    let cur = ITEMNET.load(Ordering::Relaxed);
    if cur != 0 { return Some(cur); }
    let hits: Vec<usize> = NET_HITS.lock().unwrap_or_else(|e| e.into_inner()).clone();
    // ★후보 고르기: 스택에도 우연히 시그니처가 맞는 곳이 생긴다(실측 6/7 이 스택 대역).
    //   게임 힙 객체는 players 배열과 같은 대역에 있으므로, heap_hint 와 상위 24비트가
    //   같은 후보를 우선한다. 없으면 heap_hint 와 가장 가까운 후보. 고르기 전에 시그니처 재검증(스택 사본은 사라진다).
    let live: Vec<usize> = hits.iter().copied().filter(|&a| net_sig_at(a)).collect();
    let found = live
        .iter()
        .copied()
        .find(|a| a >> 40 == heap_hint >> 40)
        .or_else(|| live.iter().copied().min_by_key(|a| a.abs_diff(heap_hint)));
    logline(&format!(
        "  [NET] 후보 {}개(생존 {}개) heap_hint=0x{heap_hint:x} → 선택 {:x?} (상위24비트 일치 우선)",
        hits.len(), live.len(), found
    ));
    match found {
        Some(a) => { ITEMNET.store(a, Ordering::Relaxed); Some(a) }
        None => { spawn_net_scan("match_start(후보 없음)"); None }
    }
}

/// 프로세스 힙을 훑어 시그니처 후보를 모은다. ★백그라운드 스레드 전용(수 초 소요).
fn scan_heap_for_hits(reason: &str) -> Vec<usize> {
    let t0 = std::time::Instant::now();
    let mut addr: usize = 0x10000;
    let mut scanned_regions = 0usize;
    let mut scanned_bytes = 0usize;
    let mut hits: Vec<usize> = Vec::new();
    while addr < (1usize << 47) {
        let mut mbi: MemoryBasicInformation = unsafe { std::mem::zeroed() };
        let n = unsafe {
            VirtualQuery(addr as *const c_void, &mut mbi,
                         std::mem::size_of::<MemoryBasicInformation>())
        };
        if n == 0 { break; }
        let base = mbi.base_address as usize;
        let size = mbi.region_size;
        if size == 0 { break; }
        let usable = mbi.state == MEM_COMMIT
            && mbi.type_ == MEM_PRIVATE
            && mbi.protect & (PAGE_GUARD | PAGE_NOACCESS) == 0
            && mbi.protect & READABLE != 0;
        if usable && size >= 0x20 {
            scanned_regions += 1;
            scanned_bytes += size;
            let end = base + size - 0x20;
            let mut a = base;
            while a <= end {
                // 첫 qword 만 싸게 걸러낸다
                if unsafe { std::ptr::read_unaligned(a as *const usize) } == 16384 && net_sig_at(a) {
                    hits.push(a);
                    if hits.len() >= 8 { break; }
                }
                a += 8;
            }
        }
        if hits.len() >= 8 { break; }
        addr = base.wrapping_add(size);
        if addr <= base { break; }
        if t0.elapsed().as_secs() >= 20 { break; }
    }
    logline(&format!(
        "  [NET] 힙 스캔(백그라운드, {reason}): 리전 {} / {:.1}MB / {}ms → 후보 {}개 {:x?}",
        scanned_regions, scanned_bytes as f64 / 1048576.0,
        t0.elapsed().as_millis(), hits.len(), hits
    ));
    hits
}

// ---------------------------------------------------------------------------
// A안 — 신경망 자동 4번째 (legacy tfm2_item_tactics 검증본 이식)
// ---------------------------------------------------------------------------

/// 챔피언 시트 인덱스 = forward ctx 가 받는 champion id. 모드챔프는 없다(None → 자동추천 스킵).
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
/// 엔진 순서 챔피언 목록(런타임). 클라 확장이 `champion_names()` 로 채운다.
/// ★이게 있으면 **모드 챔피언도 cid 를 얻는다** — CHAMP_SHEET(61, 바닐라 전용)의 한계 해소.
static CHAMP_LIST: Mutex<Vec<String>> = Mutex::new(Vec::new());
static CHAMP_LIST_CHECKED: AtomicBool = AtomicBool::new(false);

/// ★cid = **`champion_names()`(런타임 데이터 목록) 인덱스**다.
///
/// IR 근거: `recommend_items_beam_search` 의 `champ_list: Vec<Arc<dyn ChampionInfo>>` +
///   `my_champ_id: usize` = 그 목록의 인덱스.
///
/// ★판별 실험(2026-09-11, `cid_system_probe`): 엔진의 3칸 빌드는 진짜 cid 로 신경망이 고른
///   (준)최적해다. 올바른 ctx 로 채점하면 무작위 대안보다 뚜렷이 높아야 한다.
/// ```
///   archer     sheet 백분위 29%  vs  runtime 100%
///   pythoness  sheet 백분위 88%  vs  runtime 100%
/// ```
///   ⟹ **런타임 목록이 정답.** `CHAMP_SHEET`(스프라이트 시트 순서)는 틀린 체계이며,
///   legacy `tfm2_item_tactics` 는 줄곧 이 틀린 cid 로 채점해 왔다(그래서 모드챔프도 못 풀었다).
///   런타임 목록에는 모드 챔피언도 들어 있으므로 그 한계도 함께 사라진다.
fn champ_id_of(name: &str) -> Option<usize> {
    {
        let v = CHAMP_LIST.lock().unwrap_or_else(|e| e.into_inner());
        if !v.is_empty() {
            return v.iter().position(|c| c == name);
        }
    }
    // 런타임 목록 미확보(관리화면 미방문) 시에만 폴백. 정확도가 떨어지므로 임시용.
    CHAMP_SHEET.iter().position(|&c| c == name)
}

/// 런타임 목록을 채우고, 앞부분이 CHAMP_SHEET 와 일치하는지 **1회 대조**한다.
/// 일치해야 "목록 인덱스 == forward 가 받는 cid" 라고 믿을 수 있다.
fn set_champ_list(names: Vec<String>) {
    if names.is_empty() { return; }
    let mut v = CHAMP_LIST.lock().unwrap_or_else(|e| e.into_inner());
    if *v == names { return; }
    *v = names;
    if !CHAMP_LIST_CHECKED.swap(true, Ordering::Relaxed) {
        let n = CHAMP_SHEET.len().min(v.len());
        let mism: Vec<String> = (0..n)
            .filter(|&i| v[i] != CHAMP_SHEET[i])
            .take(6)
            .map(|i| format!("[{i}] 런타임={} 시트={}", v[i], CHAMP_SHEET[i]))
            .collect();
        logline(&format!(
            "  [CHAMP] 런타임 목록 {}개 (시트 {}개) | 불일치 {}건 | 뒤 8개={:?}",
            v.len(), CHAMP_SHEET.len(), mism.len(),
            v.iter().rev().take(8).collect::<Vec<_>>()
        ));
    }
}

type ItemNetFn = unsafe extern "C" fn(usize, usize, *const u64, u64, u8) -> f32;

static NET_STALE: AtomicUsize = AtomicUsize::new(0);
static FWD_CALLS: AtomicUsize = AtomicUsize::new(0);
static CIDPROBE_N: AtomicUsize = AtomicUsize::new(0);
/// 로스터 확보 실패 진단 로그 횟수 상한용(2026-09-11).
static ROSTER_PROBE: AtomicUsize = AtomicUsize::new(0);

/// forward 호출. ★매 호출 직전 net 재검증 — legacy 가 크래시 근절용으로 넣은 방어.
/// net 내부 가중치 ptr 이 세션전환/배경sim 재로드로 stale 해지면 forward 내부에서 deref AV 가 난다.
unsafe fn itemnet_forward(net: usize, ctx: &[u64; 11], build: &[u64]) -> f32 {
    if net == 0 || !itemnet_fn_ok() || !net_sig_at(net) {
        NET_STALE.fetch_add(1, Ordering::Relaxed);
        return f32::MIN;
    }
    let (b, _) = exe_range();
    let f: ItemNetFn = core::mem::transmute(b + ITEMNET_FORWARD_RVA);
    FWD_CALLS.fetch_add(1, Ordering::Relaxed);
    f(net, ctx.as_ptr() as usize, build.as_ptr(), build.len() as u64, 0)
}

/// players 배열에서 그 경기의 진짜 라인업 ctx 를 복원한다(매치업 반영).
///   ctx[0..5]=아군 챔프 cid / ctx[5..10]=적 / ctx[10]=내 포지션(0~4)
fn build_lineup_ctx(base: usize, plen: usize, me: usize) -> Option<[u64; 11]> {
    let mp = base + me * STRIDE_PLAYERSTATE;
    let my_team = read_usize(mp + OFF_TEAM)?;
    if my_team > 1 { return None; }
    let mut ctx = [9999u64; 11];
    for i in 0..plen {
        let p = base + i * STRIDE_PLAYERSTATE;
        let team = match read_usize(p + OFF_TEAM) { Some(t) if t <= 1 => t, _ => continue };
        let lane = match read_usize(p + OFF_POSITION) { Some(v) => v & 0xffff_ffff, None => continue };
        if lane >= 5 { continue; }
        let name = match read_string_at(p + OFF_CHAMPION_NAME) { Some(n) => n, None => continue };
        let cid = match champ_id_of(&name) { Some(c) => c as u64, None => continue };
        if team == my_team { ctx[lane] = cid; } else { ctx[5 + lane] = cid; }
    }
    let pos = read_usize(mp + OFF_POSITION)? & 0xffff_ffff;
    ctx[10] = pos.min(4) as u64;
    Some(ctx)
}

/// ★cid 체계 판별 실험 (2026-09-11)
/// cid = champion_list 인덱스인데, 후보가 둘이다:
///   (A) CHAMP_SHEET  = 스프라이트 시트 순서 (legacy 가 쓰던 것, 61개)
///   (B) champion_names() = 런타임 데이터 목록 순서 (60개)
/// 판별 원리: 엔진의 3칸 빌드는 **진짜 cid** 로 신경망이 고른 (준)최적해다.
///   따라서 올바른 ctx 로 채점하면 그 빌드가 무작위 대안들보다 뚜렷이 높아야 한다.
///   틀린 cid 면 평범한 점수가 나온다. → 백분위가 높은 쪽이 진짜다.
fn cid_system_probe(net: usize, base: usize, plen: usize, me: usize, cur: &[usize]) {
    if cur.len() < 3 || net == 0 { return; }
    let cands = ACTIVE_FINALS.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if cands.len() < 12 { return; }
    let name = match read_string_at(base + me * STRIDE_PLAYERSTATE + OFF_CHAMPION_NAME) {
        Some(n) => n, None => return,
    };
    let mut out = format!("  [CIDPROBE] champ={name}");
    for (label, use_runtime) in [("sheet", false), ("runtime", true)] {
        let ctx = match build_lineup_ctx_sys(base, plen, me, use_runtime) { Some(c) => c, None => {
            out.push_str(&format!(" | {label}=ctx불가")); continue; } };
        let b: Vec<u64> = cur.iter().take(3).map(|&x| x as u64).collect();
        let s_eng = unsafe { itemnet_forward(net, &ctx, &b) };
        if s_eng == f32::MIN { out.push_str(&format!(" | {label}=stale")); continue; }
        // 결정적 대안 24개: 후보 목록에서 (i, i+7, i+13) 조합
        let n = cands.len();
        let mut better = 0usize;
        let mut total = 0usize;
        for i in 0..24 {
            let alt = [cands[(i * 3) % n] as u64,
                       cands[(i * 3 + 7) % n] as u64,
                       cands[(i * 3 + 13) % n] as u64];
            if alt[0] == alt[1] || alt[1] == alt[2] || alt[0] == alt[2] { continue; }
            let sa = unsafe { itemnet_forward(net, &ctx, &alt) };
            if sa == f32::MIN { break; }
            total += 1;
            if sa >= s_eng { better += 1; }
        }
        if total == 0 { out.push_str(&format!(" | {label}=표본0")); continue; }
        let pct = 100.0 * (total - better) as f64 / total as f64;
        out.push_str(&format!(" | {label}: 엔진빌드={s_eng:.4} 백분위={pct:.0}% (대안 {total}개 중 {better}개가 더 높음)"));
    }
    logline(&out);
}

/// cid 체계를 골라 ctx 를 만든다.
fn build_lineup_ctx_sys(base: usize, plen: usize, me: usize, use_runtime: bool) -> Option<[u64; 11]> {
    let rt = CHAMP_LIST.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if use_runtime && rt.is_empty() { return None; }
    let resolve = |n: &str| -> Option<usize> {
        if use_runtime { rt.iter().position(|c| c == n) } else { CHAMP_SHEET.iter().position(|&c| c == n) }
    };
    let mp = base + me * STRIDE_PLAYERSTATE;
    let my_team = read_usize(mp + OFF_TEAM)?;
    if my_team > 1 { return None; }
    let mut ctx = [9999u64; 11];
    for i in 0..plen {
        let p = base + i * STRIDE_PLAYERSTATE;
        let team = match read_usize(p + OFF_TEAM) { Some(t) if t <= 1 => t, _ => continue };
        let lane = match read_usize(p + OFF_POSITION) { Some(v) => v & 0xffff_ffff, None => continue };
        if lane >= 5 { continue; }
        let nm = match read_string_at(p + OFF_CHAMPION_NAME) { Some(n) => n, None => continue };
        let cid = match resolve(&nm) { Some(c) => c as u64, None => continue };
        if team == my_team { ctx[lane] = cid; } else { ctx[5 + lane] = cid; }
    }
    ctx[10] = (read_usize(mp + OFF_POSITION)? & 0xffff_ffff).min(4) as u64;
    Some(ctx)
}

/// 현재 3칸 빌드에 이어붙일 4번째를 신경망 점수로 고른다.
/// 후보 = score_item 이 넘겨준 **활성 최종템**(is_active 대용). 중복은 제외.
fn compute_auto_4th(net: usize, base: usize, plen: usize, me: usize, cur: &[usize]) -> Option<(usize, f32, f32)> {
    if cur.len() < 3 { return None; }
    let ctx = build_lineup_ctx(base, plen, me)?;
    if ctx[ctx[10] as usize] == 9999 { return None; } // 내 챔프 cid 미상(모드챔프) → 스킵
    let cands = ACTIVE_FINALS.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if cands.is_empty() { return None; }
    let (b0, b1, b2) = (cur[0] as u64, cur[1] as u64, cur[2] as u64);
    if b0 >= 0x10000 || b1 >= 0x10000 || b2 >= 0x10000 { return None; }
    let (mut best, mut best_s) = (None, f32::MIN);
    let (mut smin, mut smax) = (f32::MAX, f32::MIN);
    for &c in cands.iter() {
        let cu = c as u64;
        if cu == b0 || cu == b1 || cu == b2 { continue; }
        let sc = unsafe { itemnet_forward(net, &ctx, &[b0, b1, b2, cu]) };
        if sc == f32::MIN { return None; } // net stale → 통째 포기(부분결과 금지)
        if sc < smin { smin = sc; }
        if sc > smax { smax = sc; }
        if sc > best_s { best_s = sc; best = Some(c); }
    }
    best.map(|b| (b, smin, smax))
}

// ---------------------------------------------------------------------------
// 지정 계획 산출 + 적용 (화면 경기 전용)
// ---------------------------------------------------------------------------

/// 화면 경기(ClientMatchView)의 내 팀 선수들에 대해 `item_builds` 를 계산한다.
/// `WRITE_ENABLED` 가 꺼져 있으면 **계산 결과만 로그**하고 아무것도 쓰지 않는다.
/// 양 팀 전원이 내 선수면 조합 테스트로 본다(일반 경기는 5명뿐, 롤아웃은 athlete_id 미기입).
/// 조합 테스트 판별 — **players Vec 이 정확맞춤(cap == len)인지**로 가른다.
///
/// 실측(2026-09-11, 로그 1,140건):
/// | 경로 | players cap | item_builds |
/// |---|---|---|
/// | 리그 백그라운드 시뮬 | **16**(여유 할당) | len=3 |
/// | 메뉴 배경 롤아웃 | 10 | **len=0** |
/// | 조합 테스트 | **10 = len** | len=3 |
///
/// ⟹ `cap == len && item_builds 비어있지 않음` 이 조합테스트만 집는다(2/2 적중, 오탐 0/576).
/// ~~구: `athlete_id ∈ MY_ATHLETES` 비율~~ → 조합테스트 선수는 더미(athlete_id 9~13, 한쪽은 전원 동일)라
/// MY_ATHLETES 와 겹치지 않아 **항상 false** 였다(2026-09-11 정정).
fn looks_like_comptest(base: usize, plen: usize, pcap: usize) -> bool {
    if plen < 6 || pcap != plen {
        return false;
    }
    // 롤아웃(빌드 미할당)과 가르기 위해 빌드가 실제로 채워져 있는지 본다.
    (0..plen).any(|i| {
        read_usize(base + i * STRIDE_PLAYERSTATE + OFF_ITEM_BUILDS + 16).unwrap_or(0) > 0
    })
}

fn apply_or_dryrun(base: usize, plen: usize, net: usize, is_unknown: bool, pcap: usize) {
    *LIVE_PLAYERS.lock().unwrap_or_else(|e| e.into_inner()) = Some((base, plen));
    TICK_SAMPLES.store(0, Ordering::Relaxed);
    let sels = load_selections();
    if sels.is_empty() {
        logline("  [M8] 선택 항목 0 — 할 일 없음");
        return;
    }
    // ★스코프 결정: 조합테스트면 팀별 @b:/@r:, 아니면 접두 없음
    let comptest = is_unknown && looks_like_comptest(base, plen, pcap);
    if is_unknown && !comptest {
        return; // kind=Unknown 인데 조합테스트가 아니면(리그 백그라운드 시뮬 등) 개입하지 않는다
    }
    // ★조합테스트는 내 팀 개념이 없다(선수가 더미) ⟹ MY_ATHLETES 를 요구하지 않는다.
    //   양 팀 전원에 @b:/@r: 스코프로 적용한다.
    let mine = MY_ATHLETES.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if mine.is_empty() && !comptest {
        logline("  [M8] ⚠MY_ATHLETES 미확보(관리화면 미방문?) — 팀 게이트 불가, 중단");
        return;
    }
    let keys = ITEM_KEYS.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if keys.is_empty() {
        logline("  [M8] ⚠아이템 key 목록 미확보 — 중단");
        return;
    }
    // ★매 경기 재읽기 — cfg 만 고치면 게임 재시작 없이 토글된다
    let write = load_write_flag();
    WRITE_ENABLED.store(write, Ordering::Relaxed);
    logline(&format!(
        "  [M8] 계획 산출 시작{} (MY_ATHLETES={mine:?}, item_keys={}개, WRITE={})",
        if comptest { " ★조합테스트" } else { "" },
        keys.len(),
        if write { "ON" } else { "OFF(드라이런)" }
    ));

    for i in 0..plen {
        let p = base + i * STRIDE_PLAYERSTATE;
        let Some(ath) = read_usize(p + OFF_ATHLETE_ID) else { continue };
        if !comptest && !mine.contains(&ath) {
            continue; // ★팀 게이트: 내 팀 선발만 (조합테스트는 전원 대상)
        }
        let champ = read_string_at(p + OFF_CHAMPION_NAME).unwrap_or_default();
        let cur_len = read_usize(p + OFF_ITEM_BUILDS + 16).unwrap_or(0);
        let cur_ptr = read_usize(p + OFF_ITEM_BUILDS + 8).unwrap_or(0);
        let mut cur = Vec::new();
        for k in 0..cur_len.min(8) {
            if let Some(v) = read_usize(cur_ptr + k * 8) {
                cur.push(v);
            }
        }

        // 슬롯 0..3 계획: 지정이 있으면 그 인덱스, 없으면 기존 값 유지
        let mut plan: Vec<usize> = Vec::new();
        let mut notes: Vec<String> = Vec::new();
        let mut slot3_designated = false; // 폴백까지 포함해 4번째가 "지정됨"인지
        for slot in 0u8..4 {
            let team_i = read_usize(p + OFF_TEAM).unwrap_or(0);
            let scope: u8 = if comptest { if team_i == 0 { 1 } else { 2 } } else { 0 };
            // ★스코프 폴백(2026-09-11 유저 확정 "item tactics 에서 만드는 거 그대로 쓰기"):
            //   조합테스트 전용 지정(@b:/@r:)이 없으면 **legacy 일반 전술화면 지정**을 그대로 쓴다.
            //   riot 이 조합테스트 빌드 편집기를 자기 것으로 갈아치워 legacy 의 @b:/@r: 저장 경로가
            //   죽었기 때문에(03_시행착오 §12·§16), 폴백이 없으면 조합테스트에서 지정이 전부 비어 버린다.
            let tokens: &[String] = sels
                .get(&(scope, champ.clone(), slot))
                .or_else(|| if scope != 0 { sels.get(&(0, champ.clone(), slot)) } else { None })
                .map(|v| v.as_slice())
                .unwrap_or(&[]);
            // ★후보 토큰 중 **해석되고 활성인 첫 것**을 쓴다. 전부 해석 불가면 "지정 없음"과 같이 취급
            //   (엔진 값 유지 · 4번째면 AUTO4 대상) — 죽은 지정이 자동추천까지 막지 않도록.
            let mut picked: Option<(usize, &str)> = None;
            let mut rejects: Vec<String> = Vec::new();
            for t in tokens {
                let Some(key) = token_to_key(t) else { continue }; // 0=Auto 토큰
                match keys.iter().position(|k| k == key) {
                    Some(idx) => {
                        // ★활성 검증: score_item 이 후보로 넘긴 적 없는 아이템은 비활성일 수 있다.
                        //   비활성 아이템을 넣으면 그 선수가 아이템을 **하나도 못 산다**(riot OFF 실측).
                        let act = ACTIVE_FINALS.lock().unwrap_or_else(|e| e.into_inner());
                        let known = act.is_empty() || act.contains(&idx);
                        drop(act);
                        if known {
                            picked = Some((idx, key));
                            break;
                        }
                        rejects.push(format!("비활성:{key}"));
                    }
                    None => rejects.push(format!("키없음:{key}")),
                }
            }
            let rj = if rejects.is_empty() { String::new() } else { format!("[스킵 {}]", rejects.join(",")) };
            match picked {
                Some((idx, key)) => {
                    if slot == 3 {
                        slot3_designated = true;
                    }
                    plan.push(idx);
                    notes.push(format!("s{slot}={idx}({key}){rj}"));
                }
                None => {
                    if let Some(&v) = cur.get(slot as usize) {
                        plan.push(v);
                        notes.push(format!("s{slot}={v}(유지){rj}"));
                    } else {
                        notes.push(format!("s{slot}=없음{rj}"));
                    }
                }
            }
        }

        // cid 체계 판별 실험 (팀당 앞 2명만, 1회성 진단)
        if net != 0 && CIDPROBE_N.fetch_add(1, Ordering::Relaxed) < 2 {
            let _ = catch_unwind(AssertUnwindSafe(|| cid_system_probe(net, base, plen, i, &cur)));
        }

        // ★A안: 유저가 4번째를 지정하지 않았으면 **엔진 신경망**으로 자동 선택.
        //   기존 3칸(엔진 산출)을 그대로 두고 4번째만 붙인다 = "3개 로직 그대로 확장".
        let mut auto_note = String::new();
        if plan.len() == 3 && !slot3_designated && net != 0 {
            match compute_auto_4th(net, base, plen, i, &plan) {
                Some((pick, smin, smax)) => {
                    plan.push(pick);
                    let key = keys.get(pick).map(|s| s.as_str()).unwrap_or("?");
                    auto_note = format!(" | ★AUTO4={pick}({key}) score[{smin:.4}..{smax:.4}]");
                    if smax == smin {
                        auto_note.push_str(" ⚠전후보동점(의심)");
                    }
                }
                None => auto_note = " | AUTO4=불가(cid미상/net stale/후보없음)".to_string(),
            }
        }

        logline(&format!(
            "   [M8] ath={ath} champ={champ} 현재({})={cur:?} → 계획({})={plan:?} | {}{}",
            cur.len(),
            plan.len(),
            notes.join(" "),
            auto_note
        ));

        if !write || plan.is_empty() || plan == cur {
            continue;
        }
        // ★쓰기: 새 Vec 을 우리 DLL 에서 할당해 통째로 교체.
        //   프로세스 힙이 공유됨을 실측 확인함(HeapSize/HeapValidate 양쪽 성공)
        //   ⟹ 게임이 경기 종료 시 HeapFree 해도 안전.
        let boxed: Vec<usize> = plan.clone();
        let (np, nl, nc) = (boxed.as_ptr() as usize, boxed.len(), boxed.capacity());
        std::mem::forget(boxed); // 소유권을 게임에 넘긴다
        unsafe {
            std::ptr::write_unaligned((p + OFF_ITEM_BUILDS) as *mut usize, nc);
            std::ptr::write_unaligned((p + OFF_ITEM_BUILDS + 8) as *mut usize, np);
            std::ptr::write_unaligned((p + OFF_ITEM_BUILDS + 16) as *mut usize, nl);
        }
        let back_len = read_usize(p + OFF_ITEM_BUILDS + 16).unwrap_or(0);
        logline(&format!(
            "   [M8] ★WROTE ath={ath} cap={nc} ptr=0x{np:012x} len={nl} (재확인 len={back_len})"
        ));
    }
}

// ---------------------------------------------------------------------------
// raw ItemBuildHook — M4(화면경기 근처에서 도는지) + cap 재확인. 쓰지는 않는다.
// ---------------------------------------------------------------------------

unsafe extern "C" fn ib_destroy(userdata: *mut c_void) {
    logline("[IB] destroy");
    if !userdata.is_null() {
        drop(Box::from_raw(userdata as *mut u64));
    }
}

unsafe extern "C" fn ib_id(_userdata: *const c_void) -> StrV1 {
    log_once(&IB_ID_LOGGED, "[IB] id() 최초 호출 — 훅 등록됨");
    StrV1::from_str(MOD_ID)
}

unsafe extern "C" fn ib_priority(_userdata: *const c_void) -> i32 {
    -100_000
}

unsafe extern "C" fn ib_score_item(
    _userdata: *const c_void,
    _ctx: *const ItemBuildCtxV1,
    candidate: usize,
    _base: f32,
    _out: *mut f32,
) -> u32 {
    // ★엔진이 후보로 넘기는 인덱스 = is_active && tier>=4 인 최종템.
    //   ItemBuildCtxV1 에 is_active 가 없어서 이걸 활성 판정 대용으로 쓴다.
    //   decide_build 가 항상 0 을 반환하므로 이 경로는 계속 호출된다.
    let _ = catch_unwind(AssertUnwindSafe(|| {
        let mut v = ACTIVE_FINALS.lock().unwrap_or_else(|e| e.into_inner());
        if !v.contains(&candidate) {
            v.push(candidate);
        }
    }));
    DraftDecisionKindV1::Pass as u32
}

unsafe extern "C" fn ib_decide_build(
    _userdata: *const c_void,
    ctx: *const ItemBuildCtxV1,
    _out_build: *mut usize,
    cap: usize,
) -> usize {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        let n = DECIDE_CALLS.fetch_add(1, Ordering::Relaxed) + 1;
        {
            let mut c = CAPS.lock().unwrap_or_else(|e| e.into_inner());
            if !c.contains(&cap) {
                c.push(cap);
                logline(&format!("[IB] ★cap={cap} (호출 #{n}) 누적={:?}", c));
            }
        }
        // ★전역 아이템 목록 캐시 (한 번만)
        if let Some(c) = StableItemBuildContext::from_raw(ctx) {
            let mut ks = ITEM_KEYS.lock().unwrap_or_else(|e| e.into_inner());
            if ks.len() != c.item_count() {
                *ks = c.item_keys().into_iter().map(|s| s.to_string()).collect();
                logline(&format!("[IB] 아이템 key 목록 캐시: {}개", ks.len()));
            }
        }
        if n > DETAIL_LIMIT {
            return;
        }
        // M4: 화면경기(kind=2) 직전/직후 5초 안의 호출은 눈에 띄게 표시
        let now = ms() as usize;
        let near = LAST_CMV_MS.load(Ordering::Relaxed);
        let mark = if near != 0 && now.saturating_sub(near) < 5000 { " ★CMV근접" } else { "" };
        if let Some(c) = StableItemBuildContext::from_raw(ctx) {
            logline(&format!(
                "[IB] #{n} t={now}ms{mark} cap={cap} team={} lane={:?} champ={} base={:?} ally=[{}]",
                c.team(),
                c.lane(),
                c.champion_key(),
                c.base_build(),
                c.ally_champions().join(","),
            ));
        }
    }));
    0 // ★ 아무것도 바꾸지 않는다
}

static IB_VTABLE: ItemBuildHookVtableV1 = ItemBuildHookVtableV1 {
    size: std::mem::size_of::<ItemBuildHookVtableV1>(),
    destroy: Some(ib_destroy),
    id: Some(ib_id),
    priority: Some(ib_priority),
    score_item: Some(ib_score_item),
    decide_build: Some(ib_decide_build),
};

// ---------------------------------------------------------------------------
// 클라 확장 — 팀 식별 재료(내 팀 id / 로스터)와 선택 파일 상태
// ---------------------------------------------------------------------------

struct Ext;

impl StableExtension for Ext {
    fn post_update(&self, ctx: &mut StableClient<'_>, _dt: u64) {
        let _ = catch_unwind(AssertUnwindSafe(|| {
            static LAST: Mutex<String> = Mutex::new(String::new());
            static CALLS: AtomicUsize = AtomicUsize::new(0);
            let pid = ctx.player_team_id();
            let key = format!("{:?}|{:?}", pid, ctx.client_scene_kind());
            let changed = {
                let mut l = LAST.lock().unwrap_or_else(|e| e.into_inner());
                let c = *l != key;
                if c {
                    *l = key;
                }
                c
            };
            // ★2026-09-11: 로스터 미확보면 dedup 을 무시하고 계속 재시도한다.
            //   구 코드는 (pid, scene) 키가 바뀔 때만 돌아 **세션당 2회**만 시도했고,
            //   그 2회가 실패하면 MY_ATHLETES 가 영영 비어 팀 게이트가 죽었다(실측 476/476 중단).
            let n = CALLS.fetch_add(1, Ordering::Relaxed);
            // ★신경망 힙 스캔을 메뉴에서 미리(백그라운드, 2026-09-13). 세이브 로드 전엔 net 이 없으므로
            //   팀이 확보된 뒤에만, 20초 간격으로 재시도한다(상한 NET_SCAN_MAX_TRIES). 매 프레임 검사는
            //   원자 로드 + 빈 Vec 락뿐이고, team_name FFI 는 쿨다운이 지났을 때만 부른다.
            if n % 60 == 0
                && ITEMNET.load(Ordering::Relaxed) == 0
                && NET_SCAN_TRIES.load(Ordering::Relaxed) < NET_SCAN_MAX_TRIES
                && NET_HITS.lock().unwrap_or_else(|e| e.into_inner()).is_empty()
            {
                let now = ms() as usize;
                let due = NET_SCAN_TRIES.load(Ordering::Relaxed) == 0
                    || now.saturating_sub(NET_SCAN_LAST_MS.load(Ordering::Relaxed)) >= NET_SCAN_COOLDOWN_MS;
                if due && pid.and_then(|t| ctx.team_name(t)).is_some() {
                    spawn_net_scan("post_update(메뉴 선행)");
                }
            }
            let need_roster = MY_ATHLETES.lock().unwrap_or_else(|e| e.into_inner()).is_empty();
            if !changed && !(need_roster && n % 60 == 0) {
                return;
            }
            // ★엔진 순서 챔피언 목록 — 모드 챔프 cid 확보용
            set_champ_list(ctx.champion_names());

            // ★내 팀 선발 5명 athlete_id — legacy 가 쓰던 것과 같은 값을 스테이블 API 로.
            //   ⚠legacy 실측(tfm2_item_tactics/src/lib.rs:2960~2975): `player_team_id()` 는
            //   조합테스트·초기 컨텍스트에서 **0 같은 가짜 값**을 돌려준다(진짜 팀 id 는 105 등).
            //   그래서 pid 하나만 믿지 않고, 실패하면 후보 id 를 훑어 진단을 남긴다.
            if let Some(tid) = pid {
                let raw = ctx.record_get_json(
                    mod_api_stable::RecordKindV1::Team,
                    tid,
                    "last_starting",
                );
                let ids: Vec<usize> = raw
                    .as_deref()
                    .unwrap_or("")
                    .split(|c: char| !c.is_ascii_digit())
                    .filter_map(|t| t.parse::<usize>().ok())
                    .collect();
                if !ids.is_empty() {
                    let mut m = MY_ATHLETES.lock().unwrap_or_else(|e| e.into_inner());
                    if *m != ids {
                        logline(&format!(
                            "[CL] ★MY_ATHLETES 갱신 team_id={tid} → {ids:?}  (raw={:?})",
                            raw.as_deref().map(|s| &s[..s.len().min(120)])
                        ));
                        *m = ids;
                    }
                } else if ROSTER_PROBE.fetch_add(1, Ordering::Relaxed) < 3 {
                    // ★진단 3회: 무엇이 없는지 직접 본다 — record 자체가 None 인지, 팀 id 가 틀린 건지.
                    logline(&format!(
                        "[CL] ⚠last_starting 비었음 tid={tid} raw={:?} team_name={:?}",
                        raw.as_deref().map(|s| &s[..s.len().min(200)]),
                        ctx.team_name(tid)
                    ));
                    let mut found = Vec::new();
                    for cand in 0usize..200 {
                        if let Some(nm) = ctx.team_name(cand) {
                            let r = ctx.record_get_json(
                                mod_api_stable::RecordKindV1::Team,
                                cand,
                                "last_starting",
                            );
                            let cnt = r
                                .as_deref()
                                .unwrap_or("")
                                .split(|c: char| !c.is_ascii_digit())
                                .filter(|t| !t.is_empty())
                                .count();
                            found.push(format!("{cand}:{nm}({cnt})"));
                            if found.len() >= 24 {
                                break;
                            }
                        }
                    }
                    logline(&format!("[CL]   team_name 스캔 0..200 → {found:?}"));
                }
            } else if ROSTER_PROBE.fetch_add(1, Ordering::Relaxed) < 3 {
                logline("[CL] ⚠player_team_id()=None — 팀 id 자체를 못 얻음");
            }
            if !changed {
                return;
            }
            logline(&format!(
                "[CL] t={}ms player_team_id={:?} team={:?} scene={:?} | MS={} IB={}",
                ms(),
                pid,
                pid.and_then(|i| ctx.team_name(i)),
                ctx.client_scene_kind(),
                MATCH_STARTS.load(Ordering::Relaxed),
                DECIDE_CALLS.load(Ordering::Relaxed),
            ));
        }));
    }

    fn on_end(&self) {
        logline(&format!(
            "[END] match_start={} decide_build={} cap={:?}",
            MATCH_STARTS.load(Ordering::Relaxed),
            DECIDE_CALLS.load(Ordering::Relaxed),
            CAPS.lock().unwrap_or_else(|e| e.into_inner()),
        ));
    }
}

// ---------------------------------------------------------------------------
// 엔트리
// ---------------------------------------------------------------------------

fn init(host: &StableHost) -> StableMod {
    let v = host.game_version();
    logline(&format!(
        "\n===== [INIT] {MOD_ID} v0.1 측정판 (게임 {}.{}.{}, host_abi={}, sdk_abi={}) =====",
        v.major,
        v.minor,
        v.patch,
        host.abi_level(),
        mod_api_stable::ABI_LEVEL
    ));

    // M5: 선택 파일 파싱 검증 (★접두 없는 리그용 항목)
    let sels = load_selections();
    let w = load_write_flag();
    WRITE_ENABLED.store(w, Ordering::Relaxed);
    let n_plain = sels.keys().filter(|k| k.0 == 0).count();
    let n_ctb = sels.keys().filter(|k| k.0 == 1).count();
    let n_ctr = sels.keys().filter(|k| k.0 == 2).count();
    logline(&format!(
        "[SEL] 총 {} 항목 (리그 {n_plain} / 조합@b {n_ctb} / 조합@r {n_ctr}) | WRITE={} ({})",
        sels.len(),
        if w { "ON" } else { "OFF(드라이런)" },
        sel_path_str()
    ));
    for ((sc, c, sl), ts) in sels.iter().take(6) {
        logline(&format!("   scope{sc} {c} slot{sl} → {ts:?}"));
    }
    let n_dup = sels.values().filter(|v| v.len() > 1).count();
    if n_dup > 0 {
        logline(&format!("[SEL] ⚠같은 칸에 후보 토큰 2개 이상 = {n_dup}칸 (해석되는 첫 토큰 사용)"));
    }

    host.log(LogLevel::Info, "tfm2_item_build v0.1 (measurement only)");
    let mut decl = StableMod::new(MOD_ID);
    decl.set_extension(Ext);
    // match_hook / item_build_hook 은 entry 에서 raw 로 배선한다.
    decl
}

#[no_mangle]
pub extern "C" fn tfm2_mod_required_abi_level() -> u32 {
    // ⚠level 8 을 요구하면 0.5.8 호스트가 엔트리를 아예 안 부른다(실측 2026-09-11:
    //   required=8 → 로그 0바이트, required=2 인 자매 프로브는 정상 로드).
    //   ABI 설계상 호스트에 없는 슬롯은 슬롯 단위로 degrade 되므로(slot! → None)
    //   낮게 선언해도 sim_origin 은 호스트가 지원하면 그대로 동작한다.
    resolve_required_level(2)
}

/// # Safety
/// 로더 계약을 `entry_shim` 에 위임하고 raw 훅 2종만 덧배선한다.
#[no_mangle]
pub unsafe extern "C" fn tfm2_mod_entry_stable(host: *const HostApiV1) -> *mut ModExportV1 {
    START.get_or_init(Instant::now);
    let export = entry_shim(host, init);
    if export.is_null() {
        logline("[INIT] ★entry_shim null — export 실패");
        return export;
    }

    // ★userdata 는 반드시 non-null (null 이면 호스트가 조용히 등록을 건너뛴다 — 실측 확인)
    (*export).match_hook = MatchHookRegV1 {
        userdata: Box::into_raw(Box::new(0xB01D_0001u64)) as *mut c_void,
        vtable: &MH_VTABLE as *const MatchHookVtableV1,
    };

    let regs: &'static mut [ItemBuildHookRegV1; 1] = Box::leak(Box::new([ItemBuildHookRegV1 {
        userdata: Box::into_raw(Box::new(0xB01D_0002u64)) as *mut c_void,
        vtable: &IB_VTABLE as *const ItemBuildHookVtableV1,
    }]));
    (*export).item_build_hooks_ptr = regs.as_ptr();
    (*export).item_build_hooks_len = regs.len();

    logline(&format!(
        "[INIT] 배선 완료: match_hook.vtable={:p} item_build_hooks_len={} export.size={} req_abi={}",
        (*export).match_hook.vtable,
        (*export).item_build_hooks_len,
        (*export).size,
        (*export).required_abi_level,
    ));
    export
}
