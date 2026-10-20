//! 임계점 트레이서 (v0.4.0). 분석 아틀라스가 "이 값을 쓴다"고 기록한 비교 명령(cmp/test)에
//! INT3 관측점을 걸어, 실제 게임에서 그 비교에 들어온 값이 임계점의 아래/같음/위 중 어디였는지 센다.
//!
//! 게임 판단은 바꾸지 않는다: 관측점에 걸린 스레드는 원래 명령 한 개를 그대로 복사한 스텁
//! (원명령 + 다음 명령으로 절대 점프)으로 보내진다. 명령을 흉내 내지(에뮬레이션) 않는다.
//! VEH 처리기에는 할당/락/대기/게임 CALL 이 없다. 카운터는 전부 원자 변수다.
//! 지점 표(SITES)는 tools/gen_sites.py 가 exe 0.6.4 에서 디스어셈블해 만든다.
//! 실게임 미검증(v0.4.0 작성 시점).
use crate::platform;
use crate::sites_generated::SITES;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU32, AtomicU64, AtomicU8, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

pub const MAX_SITES: usize = 6144;
const SAMPLES: usize = 8;
const STUB_SLOT: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind { CmpImm, CmpDyn, Test, Value }
#[derive(Clone, Copy, Debug)]
pub enum Op {
    None,
    /// reg = CONTEXT 정수 레지스터 번호(Rax=0..R15=15), shift=8 이면 ah/ch/dh/bh
    Reg { reg: u8, shift: u8, size: u8 },
    Imm { value: u64, size: u8 },
    /// base/index = 레지스터 번호, -1 = 없음, 16 = RIP(v0.4.0 표에는 없음)
    Mem { base: i8, index: i8, scale: u8, disp: i64, size: u8 },
}
pub struct Site {
    pub rva: u32, pub len: u8, pub bytes: [u8; 16], pub kind: Kind,
    pub lhs: Op, pub rhs: Op,
    /// 0=unsigned 1=signed 2=소비 조건 없음/같음비교(unsigned 로 분류)
    pub signed: u8,
    /// 소비 조건: 0=없음 1:< 2:>= 3:<= 4:> 5:== 6:!=  (lhs ? rhs 가 참이면 조건 성립)
    pub rel: u8,
}

// 상태: 0=미무장 1=무장 2=예산초과(해제 대기) 3=예산으로 쉬는 중 4=거절
const S_IDLE: u8 = 0; const S_ARMED: u8 = 1; const S_OVER: u8 = 2; const S_REST: u8 = 3; const S_REFUSED: u8 = 4;

struct Cls {
    hits: AtomicU64, below: AtomicU64, equal: AtomicU64, above: AtomicU64,
    tm1: AtomicU64, tp1: AtomicU64, cond_true: AtomicU64, read_fail: AtomicU64,
    /// 최솟값은 0 초기화(.bss)를 위해 반전값(!key)의 최댓값으로 저장한다.
    min_key: AtomicU64, max_key: AtomicU64,
}
impl Cls {
    const fn new() -> Self {
        Self { hits: AtomicU64::new(0), below: AtomicU64::new(0), equal: AtomicU64::new(0), above: AtomicU64::new(0),
            tm1: AtomicU64::new(0), tp1: AtomicU64::new(0), cond_true: AtomicU64::new(0), read_fail: AtomicU64::new(0),
            min_key: AtomicU64::new(0), max_key: AtomicU64::new(0) }
    }
}
struct SiteState {
    cls: [Cls; 3], window: AtomicU32, state: AtomicU8, stub: AtomicUsize,
    rhs_min: AtomicU64, rhs_max: AtomicU64,
    samples: [AtomicU64; SAMPLES], sample_i: AtomicU32,
}
impl SiteState {
    const fn new() -> Self {
        Self { cls: [Cls::new(), Cls::new(), Cls::new()], window: AtomicU32::new(0), state: AtomicU8::new(S_IDLE),
            stub: AtomicUsize::new(0), rhs_min: AtomicU64::new(0), rhs_max: AtomicU64::new(0),
            samples: [const { AtomicU64::new(0) }; SAMPLES], sample_i: AtomicU32::new(0) }
    }
}
static STATE: [SiteState; MAX_SITES] = [const { SiteState::new() }; MAX_SITES];

// VEH 가 읽는 게시 값. TABLE/BASE/STUB 이 먼저 게시된 뒤에만 INT3 를 쓴다.
static TABLE: AtomicPtr<Site> = AtomicPtr::new(std::ptr::null_mut());
static TABLE_LEN: AtomicUsize = AtomicUsize::new(0);
static BASE: AtomicUsize = AtomicUsize::new(0);
static CAP: AtomicU32 = AtomicU32::new(64);
static SCENE: AtomicUsize = AtomicUsize::new(usize::MAX);
static MAIN_TID: AtomicU32 = AtomicU32::new(0);
static TOTAL_HITS: AtomicU64 = AtomicU64::new(0);
static UNSUPPORTED_HITS: AtomicU64 = AtomicU64::new(0);

#[link(name = "kernel32")]
extern "system" { fn GetCurrentThreadId() -> u32; }

fn table() -> &'static [Site] {
    let p = TABLE.load(Ordering::Acquire);
    if p.is_null() { return &[]; }
    unsafe { std::slice::from_raw_parts(p, TABLE_LEN.load(Ordering::Acquire)) }
}

#[inline(always)]
fn mask(size: u8) -> u64 { if size >= 8 { u64::MAX } else { (1u64 << (size as u32 * 8)) - 1 } }
#[inline(always)]
fn sext(v: u64, size: u8) -> i64 {
    if size >= 8 { v as i64 } else { let s = 64 - size as u32 * 8; ((v << s) as i64) >> s }
}
#[inline(always)]
unsafe fn ctx_reg(ctx: *const u8, reg: u8) -> u64 {
    std::ptr::read_unaligned(ctx.add(0x78 + 8 * reg as usize) as *const u64)
}

/// 피연산자 값(폭으로 자른 원값). 메모리는 게임 명령이 곧 읽을 같은 주소를 읽는다.
unsafe fn read_op(ctx: *const u8, op: &Op, next_ip: usize) -> Option<u64> {
    match *op {
        Op::None => None,
        Op::Imm { value, size } => Some(value & mask(size)),
        Op::Reg { reg, shift, size } => {
            if reg > 15 { return None; }
            Some((ctx_reg(ctx, reg) >> shift) & mask(size))
        }
        Op::Mem { base, index, scale, disp, size } => {
            let mut ea: u64 = disp as u64;
            if base == 16 { ea = ea.wrapping_add(next_ip as u64); }
            else if base >= 0 { if base > 15 { return None; } ea = ea.wrapping_add(ctx_reg(ctx, base as u8)); }
            if index >= 0 { if index > 15 { return None; } ea = ea.wrapping_add(ctx_reg(ctx, index as u8).wrapping_mul(scale as u64)); }
            platform::veh_read(ea as usize, size).map(|v| v & mask(size))
        }
    }
}
fn op_size(op: &Op) -> u8 {
    match *op { Op::Reg { size, .. } | Op::Imm { size, .. } | Op::Mem { size, .. } => size, Op::None => 8 }
}

/// 비교 분류. 반환 (순서 -1/0/1, a-b 가 정확히 -1/+1 인지, 조건 성립 여부, lhs 정렬키)
pub(crate) fn classify(a: u64, b: u64, size: u8, signed: u8, rel: u8) -> (i8, i8, Option<bool>, u64) {
    let (ord, diff, key) = if signed == 1 {
        let (x, y) = (sext(a, size), sext(b, size));
        let d = x as i128 - y as i128;
        (x.cmp(&y) as i8, d, (x as u64) ^ (1u64 << 63))
    } else {
        let (x, y) = (a & mask(size), b & mask(size));
        (x.cmp(&y) as i8, x as i128 - y as i128, x)
    };
    let near = if diff == -1 { -1 } else if diff == 1 { 1 } else { 0 };
    let cond = match rel {
        1 => Some(ord < 0), 2 => Some(ord >= 0), 3 => Some(ord <= 0), 4 => Some(ord > 0),
        5 => Some(ord == 0), 6 => Some(ord != 0), _ => None,
    };
    (ord, near, cond, key)
}

/// VEH 진입점. 등록 지점의 INT3 면 기록 후 RIP 를 스텁으로 옮기고 true.
pub(crate) unsafe fn on_breakpoint(ctx: *mut u8, address: usize) -> bool {
    let base = BASE.load(Ordering::Acquire);
    if base == 0 || address < base { return false; }
    let rva = address - base;
    if rva > u32::MAX as usize { return false; }
    let t = table();
    let Ok(id) = t.binary_search_by_key(&(rva as u32), |s| s.rva) else { return false; };
    if id >= MAX_SITES { return false; }
    let st = &STATE[id];
    let stub = st.stub.load(Ordering::Acquire);
    if stub == 0 { return false; }
    let rip = std::ptr::read_unaligned(ctx.add(0xf8) as *const usize);
    if rip != address && address.checked_add(1) != Some(rip) { return false; }
    let site = &t[id];
    record(ctx, site, st, address + site.len as usize);
    // CONTEXT.Rip 만 바꾼다. 레지스터/플래그/스택은 그대로 → 스텁에서 원래 명령이 실행된다.
    std::ptr::write_unaligned(ctx.add(0xf8) as *mut usize, stub);
    true
}

unsafe fn record(ctx: *const u8, site: &Site, st: &SiteState, next_ip: usize) {
    TOTAL_HITS.fetch_add(1, Ordering::Relaxed);
    let scene = SCENE.load(Ordering::Relaxed);
    let c = if matches!(scene, 4 | 5) { if GetCurrentThreadId() == MAIN_TID.load(Ordering::Relaxed) { 0 } else { 1 } } else { 2 };
    let cls = &st.cls[c];
    cls.hits.fetch_add(1, Ordering::Relaxed);
    let n = st.window.fetch_add(1, Ordering::Relaxed).wrapping_add(1);
    if n >= CAP.load(Ordering::Relaxed) {
        let _ = st.state.compare_exchange(S_ARMED, S_OVER, Ordering::AcqRel, Ordering::Relaxed);
    }
    let size = op_size(&site.lhs);
    let (Some(a), Some(b)) = (read_op(ctx, &site.lhs, next_ip), read_op(ctx, &site.rhs, next_ip)) else {
        cls.read_fail.fetch_add(1, Ordering::Relaxed); return;
    };
    match site.kind {
        Kind::CmpImm | Kind::CmpDyn => {
            let (ord, near, cond, key) = classify(a, b, size, site.signed, site.rel);
            match ord { -1 => &cls.below, 0 => &cls.equal, _ => &cls.above }.fetch_add(1, Ordering::Relaxed);
            if near == -1 { cls.tm1.fetch_add(1, Ordering::Relaxed); }
            if near == 1 { cls.tp1.fetch_add(1, Ordering::Relaxed); }
            if cond == Some(true) { cls.cond_true.fetch_add(1, Ordering::Relaxed); }
            cls.min_key.fetch_max(!key, Ordering::Relaxed);
            cls.max_key.fetch_max(key, Ordering::Relaxed);
            if site.kind == Kind::CmpDyn {
                let rk = if site.signed == 1 { (sext(b, size) as u64) ^ (1u64 << 63) } else { b & mask(size) };
                st.rhs_min.fetch_max(!rk, Ordering::Relaxed);
                st.rhs_max.fetch_max(rk, Ordering::Relaxed);
            }
            if c != 2 {
                let i = st.sample_i.fetch_add(1, Ordering::Relaxed) as usize % SAMPLES;
                st.samples[i].store(a & mask(size), Ordering::Relaxed);
            }
        }
        Kind::Test => {
            let zero = (a & b & mask(size)) == 0;
            if zero { cls.equal.fetch_add(1, Ordering::Relaxed); } else { cls.above.fetch_add(1, Ordering::Relaxed); }
            let cond = match site.rel { 5 => Some(zero), 6 => Some(!zero), _ => None };
            if cond == Some(true) { cls.cond_true.fetch_add(1, Ordering::Relaxed); }
            let key = a & mask(size);
            cls.min_key.fetch_max(!key, Ordering::Relaxed);
            cls.max_key.fetch_max(key, Ordering::Relaxed);
        }
        Kind::Value => { UNSUPPORTED_HITS.fetch_add(1, Ordering::Relaxed); }
    }
}

/// 스텁 = 원래 명령 바이트 + jmp [rip+0] + 다음 명령 절대주소. RIP 상대 명령은 지원하지 않는다.
pub(crate) fn build_stub(site: &Site, next_abs: usize) -> Result<[u8; STUB_SLOT], &'static str> {
    let len = site.len as usize;
    if len == 0 || len > 15 || len + 14 > STUB_SLOT { return Err("instruction length unsupported"); }
    if matches!(site.lhs, Op::Mem { base: 16, .. }) || matches!(site.rhs, Op::Mem { base: 16, .. }) {
        return Err("rip-relative operand unsupported");
    }
    let mut out = [0xccu8; STUB_SLOT];
    out[..len].copy_from_slice(&site.bytes[..len]);
    out[len..len + 6].copy_from_slice(&[0xff, 0x25, 0, 0, 0, 0]);
    out[len + 6..len + 14].copy_from_slice(&(next_abs as u64).to_le_bytes());
    Ok(out)
}

// ───────────────────────── 설정 ─────────────────────────
#[derive(Clone, Debug)]
pub struct Config {
    pub trace: bool,
    pub functions: Vec<u32>,
    pub window_ms: u64,
    pub cap_per_window: u32,
    pub snapshot_ms: u64,
    pub install_delay_frames: u64,
}
impl Default for Config {
    fn default() -> Self {
        Self { trace: false, functions: Vec::new(), window_ms: 10_000, cap_per_window: 64, snapshot_ms: 15_000, install_delay_frames: 360 }
    }
}
pub fn parse(text: &str) -> Result<Config, String> {
    let mut out = Config::default();
    let mut seen = std::collections::BTreeSet::new();
    for (line, raw) in text.lines().enumerate() {
        let value = raw.split('#').next().unwrap_or("").trim();
        if value.is_empty() { continue; }
        let (key, value) = value.split_once('=').ok_or_else(|| format!("sites.cfg {}행: key=value 필요", line + 1))?;
        let (key, value) = (key.trim(), value.trim());
        if !seen.insert(key.to_string()) { return Err(format!("sites.cfg 중복 설정: {key}")); }
        let num = |lo: u64, hi: u64| -> Result<u64, String> {
            let n = value.parse::<u64>().map_err(|_| format!("sites.cfg {key}: 정수 필요"))?;
            if !(lo..=hi).contains(&n) { return Err(format!("sites.cfg {key}: 범위 {lo}..{hi} 밖")); }
            Ok(n)
        };
        match key {
            "trace" => out.trace = match value { "true" => true, "false" => false, _ => return Err("sites.cfg trace 는 true/false".into()) },
            "functions" => for item in value.split(',').map(str::trim).filter(|s| !s.is_empty()) {
                let rva = u32::from_str_radix(item.trim_start_matches("0x"), 16).map_err(|_| format!("sites.cfg 잘못된 함수 RVA: {item}"))?;
                out.functions.push(rva);
            },
            "window_ms" => out.window_ms = num(1000, 600_000)?,
            "cap_per_window" => out.cap_per_window = num(1, 1_000_000)? as u32,
            "snapshot_ms" => out.snapshot_ms = num(1000, 600_000)?,
            "install_delay_frames" => out.install_delay_frames = num(60, 36_000)?,
            _ => return Err(format!("sites.cfg 알 수 없는 설정: {key}")),
        }
    }
    Ok(out)
}

// ───────────────────────── 실행 상태 ─────────────────────────
struct Runtime {
    dir: PathBuf, run_id: String, config: Config, identity: bool,
    frames: AtomicU64, started: AtomicBool, stop: AtomicBool,
    func_of: Vec<u32>, // 지점별 소속 함수(선택 필터용). 표와 같은 순서.
    status: Mutex<String>,
}
static RT: OnceLock<Runtime> = OnceLock::new();

/// 모드 초기화 때 한 번. sites.cfg 가 없거나 trace=false 면 아무것도 걸지 않는다.
pub fn initialize(dir: PathBuf, identity: bool) -> Result<String, String> {
    let config = match std::fs::read_to_string(dir.join("sites.cfg")) {
        Ok(text) => parse(&text)?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Config::default(),
        Err(e) => return Err(format!("sites.cfg 읽기 실패: {e}")),
    };
    if SITES.len() > MAX_SITES { return Err("지점 표가 MAX_SITES 초과".into()); }
    if SITES.windows(2).any(|w| w[0].rva >= w[1].rva) { return Err("지점 표가 RVA 오름차순이 아님".into()); }
    let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e| e.to_string())?.as_nanos();
    let run_id = format!("{}_{stamp}", std::process::id());
    let msg = if !config.trace { "임계점 트레이서 꺼짐(sites.cfg trace=false)".to_string() }
        else if !identity { "임계점 트레이서 거절: exe SHA 불일치".to_string() }
        else { format!("임계점 트레이서 준비: 지점 {} · run {run_id}", SITES.len()) };
    let func_of = crate::sites_generated::SITE_FUNCS.to_vec();
    RT.set(Runtime { dir, run_id, config, identity, frames: AtomicU64::new(0), started: AtomicBool::new(false),
        stop: AtomicBool::new(false), func_of, status: Mutex::new(msg.clone()) }).map_err(|_| "site_trace 중복 초기화".to_string())?;
    Ok(msg)
}

/// 매 프레임(메인 스레드). 장면과 메인 스레드 ID를 갱신하고, 지연 뒤 한 번 작업자를 띄운다.
pub fn post_update(scene: usize) {
    SCENE.store(scene, Ordering::Relaxed);
    MAIN_TID.store(unsafe { GetCurrentThreadId() }, Ordering::Relaxed);
    let Some(rt) = RT.get() else { return; };
    if !rt.config.trace || !rt.identity { return; }
    let frame = rt.frames.fetch_add(1, Ordering::Relaxed) + 1;
    if frame < rt.config.install_delay_frames || rt.started.swap(true, Ordering::AcqRel) { return; }
    if platform::debugger_present() { set_status(rt, "임계점 트레이서 거절: 디버거 연결됨(INT3 충돌)".into()); return; }
    if let Err(e) = platform::init_safe_reader() { set_status(rt, format!("임계점 트레이서 거절: {e}")); return; }
    let spawned = std::thread::Builder::new().name("tfm2_site_trace".into()).spawn(move || {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| worker(rt)));
    });
    if let Err(e) = spawned { set_status(rt, format!("임계점 트레이서 작업자 시작 실패: {e}")); }
}

pub fn shutdown() {
    if let Some(rt) = RT.get() { rt.stop.store(true, Ordering::Release); }
}

fn set_status(rt: &Runtime, text: String) { *rt.status.lock().unwrap_or_else(|e| e.into_inner()) = text; }

/// 설치: 지점 바이트 확인 → 스텁 게시 → 표 게시 → INT3 무장. 이후 창(window)마다 예산 관리, 주기적 스냅샷.
fn worker(rt: &'static Runtime) {
    let base = match platform::module_base() { Ok(b) => b, Err(e) => { set_status(rt, format!("module_base 실패: {e}")); return; } };
    let selected: Vec<bool> = (0..SITES.len()).map(|i| rt.config.functions.is_empty() || rt.config.functions.contains(&rt.func_of[i])).collect();
    let mut refused: Vec<(usize, &'static str)> = Vec::new();
    // 1) 스텁 아레나를 한 번에 만든다(선택 지점만). 바이트가 표와 다르면 거절.
    let mut arena = vec![0xccu8; SITES.len() * STUB_SLOT];
    let mut ok = vec![false; SITES.len()];
    for (i, s) in SITES.iter().enumerate() {
        if !selected[i] { continue; }
        let addr = base + s.rva as usize;
        let mut cur = [0u8; 16];
        if platform::safe_read_bytes(addr, &mut cur[..s.len as usize]).is_err() { refused.push((i, "read failed")); continue; }
        if cur[..s.len as usize] != s.bytes[..s.len as usize] { refused.push((i, "bytes differ (patched/other mod)")); continue; }
        match build_stub(s, addr + s.len as usize) {
            Ok(stub) => { arena[i * STUB_SLOT..(i + 1) * STUB_SLOT].copy_from_slice(&stub); ok[i] = true; }
            Err(e) => refused.push((i, e)),
        }
    }
    let arena_addr = match platform::allocate_code(&arena) { Ok(a) => a, Err(e) => { set_status(rt, format!("스텁 할당 실패: {e}")); return; } };
    for (i, good) in ok.iter().enumerate() {
        if *good { STATE[i].stub.store(arena_addr + i * STUB_SLOT, Ordering::Release); }
    }
    for &(i, _) in &refused { STATE[i].state.store(S_REFUSED, Ordering::Release); }
    CAP.store(rt.config.cap_per_window, Ordering::Release);
    BASE.store(base, Ordering::Release);
    TABLE_LEN.store(SITES.len(), Ordering::Release);
    TABLE.store(SITES.as_ptr() as *mut Site, Ordering::Release);
    // 2) 무장
    let want: Vec<usize> = (0..SITES.len()).filter(|&i| ok[i]).collect();
    let armed = swap_sites(base, &want, true);
    let summary = format!("임계점 트레이서 무장: 선택 {} · 무장 {} · 거절 {} · run {}",
        selected.iter().filter(|x| **x).count(), armed, refused.len(), rt.run_id);
    set_status(rt, summary.clone());
    write_status(rt, &refused, &summary, arena_addr);
    // 3) 루프
    let start = std::time::Instant::now();
    let mut next_window = rt.config.window_ms;
    let mut next_snap = rt.config.snapshot_ms;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(20));
        let stopping = rt.stop.load(Ordering::Acquire);
        let over: Vec<usize> = (0..SITES.len()).filter(|&i| STATE[i].state.load(Ordering::Acquire) == S_OVER).collect();
        if !over.is_empty() { swap_sites(base, &over, false); }
        let now = start.elapsed().as_millis() as u64;
        if now >= next_window && !stopping {
            next_window = now + rt.config.window_ms;
            for st in STATE.iter().take(SITES.len()) { st.window.store(0, Ordering::Relaxed); }
            let rest: Vec<usize> = (0..SITES.len()).filter(|&i| STATE[i].state.load(Ordering::Acquire) == S_REST).collect();
            if !rest.is_empty() { swap_sites(base, &rest, true); }
        }
        if now >= next_snap || stopping {
            next_snap = now + rt.config.snapshot_ms;
            let _ = write_snapshot(rt, now);
        }
        if stopping { break; }
    }
}

/// arm=true: 원바이트→CC (IDLE/REST → ARMED). arm=false: CC→원바이트 (OVER → REST). 반환 = 성공 수.
fn swap_sites(base: usize, ids: &[usize], arm: bool) -> usize {
    let mut done = 0;
    let mut i = 0;
    let mut items: Vec<(usize, u8, u8)> = Vec::new();
    let mut idx: Vec<usize> = Vec::new();
    let mut res = Vec::new();
    while i < ids.len() {
        let page = (base + SITES[ids[i]].rva as usize) & !0xfff;
        items.clear(); idx.clear();
        while i < ids.len() && (base + SITES[ids[i]].rva as usize) & !0xfff == page {
            let s = &SITES[ids[i]];
            let a = base + s.rva as usize;
            items.push(if arm { (a, s.bytes[0], 0xcc) } else { (a, 0xcc, s.bytes[0]) });
            idx.push(ids[i]);
            i += 1;
        }
        let r = unsafe { platform::swap_page_bytes(&items, &mut res) };
        for (k, &id) in idx.iter().enumerate() {
            let success = r.is_ok() && res.get(k).copied().unwrap_or(false);
            if success {
                done += 1;
                STATE[id].state.store(if arm { S_ARMED } else { S_REST }, Ordering::Release);
            } else if arm {
                STATE[id].state.store(S_REFUSED, Ordering::Release);
            }
        }
    }
    done
}

fn write_status(rt: &Runtime, refused: &[(usize, &'static str)], summary: &str, arena: usize) {
    let mut s = format!("{{\"run_id\":\"{}\",\"summary\":{:?},\"site_count\":{},\"window_ms\":{},\"cap_per_window\":{},\"stub_arena\":\"0x{:x}\",\"refused\":[",
        rt.run_id, summary, SITES.len(), rt.config.window_ms, rt.config.cap_per_window, arena);
    for (k, (i, why)) in refused.iter().enumerate() {
        if k > 0 { s.push(','); }
        s.push_str(&format!("{{\"id\":{i},\"rva\":\"0x{:x}\",\"reason\":{:?}}}", SITES[*i].rva, why));
    }
    s.push_str("]}\n");
    let _ = std::fs::write(rt.dir.join(format!("sites_status_{}.json", rt.run_id)), s);
}

/// 누적 카운터 스냅샷을 jsonl 한 줄로 덧붙인다. 관측이 있는 지점만 쓴다.
fn write_snapshot(rt: &Runtime, now_ms: u64) -> std::io::Result<()> {
    let mut line = String::with_capacity(64 * 1024);
    let (mut armed, mut rest, mut refused) = (0, 0, 0);
    for st in STATE.iter().take(SITES.len()) {
        match st.state.load(Ordering::Relaxed) { S_ARMED | S_OVER => armed += 1, S_REST => rest += 1, S_REFUSED => refused += 1, _ => {} }
    }
    line.push_str(&format!("{{\"run_id\":\"{}\",\"t_ms\":{},\"frames\":{},\"scene\":{},\"total_hits\":{},\"armed\":{},\"resting\":{},\"refused\":{},\"sites\":[",
        rt.run_id, now_ms, rt.frames.load(Ordering::Relaxed), SCENE.load(Ordering::Relaxed) as i64,
        TOTAL_HITS.load(Ordering::Relaxed), armed, rest, refused));
    let mut first = true;
    for (i, st) in STATE.iter().enumerate().take(SITES.len()) {
        let any = st.cls.iter().any(|c| c.hits.load(Ordering::Relaxed) != 0);
        if !any { continue; }
        if !first { line.push(','); }
        first = false;
        line.push_str(&format!("{{\"id\":{i},\"rva\":\"0x{:x}\",\"st\":{},\"c\":[", SITES[i].rva, st.state.load(Ordering::Relaxed)));
        for (k, c) in st.cls.iter().enumerate() {
            if k > 0 { line.push(','); }
            let l = |a: &AtomicU64| a.load(Ordering::Relaxed);
            line.push_str(&format!("[{},{},{},{},{},{},{},{},{},{}]",
                l(&c.hits), l(&c.below), l(&c.equal), l(&c.above), l(&c.tm1), l(&c.tp1), l(&c.cond_true), l(&c.read_fail),
                !l(&c.min_key), l(&c.max_key)));
        }
        line.push_str(&format!("],\"rmin\":{},\"rmax\":{},\"s\":[", !st.rhs_min.load(Ordering::Relaxed), st.rhs_max.load(Ordering::Relaxed)));
        let n = (st.sample_i.load(Ordering::Relaxed) as usize).min(SAMPLES);
        for k in 0..n {
            if k > 0 { line.push(','); }
            line.push_str(&st.samples[k].load(Ordering::Relaxed).to_string());
        }
        line.push_str("]}");
    }
    line.push_str("]}\n");
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(rt.dir.join(format!("sites_{}.jsonl", rt.run_id)))?;
    f.write_all(line.as_bytes())?;
    f.flush()
}

pub fn status_text() -> String {
    RT.get().map(|rt| rt.status.lock().unwrap_or_else(|e| e.into_inner()).clone()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_unsigned_signed_and_boundaries() {
        // cmp rax, 30 ; setae  → 29/30/31
        assert_eq!(classify(29, 30, 8, 0, 2), (-1, -1, Some(false), 29));
        assert_eq!(classify(30, 30, 8, 0, 2), (0, 0, Some(true), 30));
        assert_eq!(classify(31, 30, 8, 0, 2), (1, 1, Some(true), 31));
        // signed: -1 < 0 (unsigned 이면 큼)
        assert_eq!(classify(u64::MAX, 0, 8, 1, 1).0, -1);
        assert_eq!(classify(u64::MAX, 0, 8, 0, 1).0, 1);
        // 4바이트 signed: 0xffffffff = -1, imm -2000 은 하위 32비트로 비교
        let (o, _, c, _) = classify(0xffff_ffff, (-2000i64) as u64, 4, 1, 4);
        assert_eq!((o, c), (1, Some(true)));
        // 정렬키: signed 는 부호비트 뒤집기 → -5 < 3
        let (_, _, _, k1) = classify((-5i64) as u64, 0, 8, 1, 0);
        let (_, _, _, k2) = classify(3, 0, 8, 1, 0);
        assert!(k1 < k2);
    }

    #[test]
    fn config_defaults_and_errors() {
        let c = parse("").unwrap();
        assert!(!c.trace && c.functions.is_empty() && c.cap_per_window == 64);
        let c = parse("trace=true\nfunctions=fd4260,0xdc0bb0\ncap_per_window=256").unwrap();
        assert!(c.trace && c.functions == vec![0xfd4260, 0xdc0bb0] && c.cap_per_window == 256);
        for bad in ["trace=yes", "window_ms=5", "x=1", "trace=true\ntrace=false", "functions=zz"] {
            assert!(parse(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn generated_table_is_sorted_and_stubs_build() {
        assert!(SITES.len() <= MAX_SITES);
        assert!(SITES.windows(2).all(|w| w[0].rva < w[1].rva));
        assert_eq!(SITES.len(), crate::sites_generated::SITE_FUNCS.len());
        for s in SITES.iter() {
            let stub = build_stub(s, 0x1234_5678_9abc).unwrap();
            assert_eq!(&stub[..s.len as usize], &s.bytes[..s.len as usize]);
            assert_eq!(&stub[s.len as usize..s.len as usize + 2], &[0xff, 0x25]);
            assert!(s.bytes[0] != 0xcc);
        }
    }

    /// 실제 INT3 → VEH → 스텁 경로 종단 시험. 실행 페이지에 `cmp rcx, 30 ; setae al ; ret` 함수를 만들고
    /// 그 cmp 에 지점을 걸어 29/30/31 로 부른 뒤 반환값(원래 동작 보존)과 카운터를 확인한다.
    #[test]
    fn end_to_end_breakpoint_counts_and_preserves_result() {
        // 48 83 f9 1e = cmp rcx,0x1e / 0f 93 c0 = setae al / c3 = ret. 앞에 xor eax,eax (31 c0)
        let code: [u8; 10] = [0x31, 0xc0, 0x48, 0x83, 0xf9, 0x1e, 0x0f, 0x93, 0xc0, 0xc3];
        let page = platform::allocate_code(&code).unwrap();
        let mut bytes = [0u8; 16];
        bytes[..4].copy_from_slice(&code[2..6]);
        let site: &'static [Site] = Box::leak(Box::new([Site { rva: 2, len: 4, bytes, kind: Kind::CmpImm,
            lhs: Op::Reg { reg: 1, shift: 0, size: 8 }, rhs: Op::Imm { value: 30, size: 8 }, signed: 0, rel: 2 }]));
        platform::init_safe_reader().unwrap();
        let stub_code = build_stub(&site[0], page + 6).unwrap();
        let stub = platform::allocate_code(&stub_code).unwrap();
        STATE[0].stub.store(stub, Ordering::Release);
        CAP.store(1_000_000, Ordering::Release);
        BASE.store(page, Ordering::Release);
        TABLE_LEN.store(1, Ordering::Release);
        TABLE.store(site.as_ptr() as *mut Site, Ordering::Release);
        SCENE.store(4, Ordering::Release);
        MAIN_TID.store(unsafe { GetCurrentThreadId() }, Ordering::Release);
        let f: extern "C" fn(u64) -> u8 = unsafe { std::mem::transmute(page) };
        assert_eq!((f(29), f(30), f(31)), (0, 1, 1)); // 무장 전
        let mut res = Vec::new();
        unsafe { platform::swap_page_bytes(&[(page + 2, 0x48, 0xcc)], &mut res).unwrap(); }
        assert_eq!(res, vec![true]);
        STATE[0].state.store(S_ARMED, Ordering::Release);
        assert_eq!((f(29), f(30), f(31), f(1000)), (0, 1, 1, 1)); // 무장 후에도 결과 동일
        let c = &STATE[0].cls[0];
        let l = |a: &AtomicU64| a.load(Ordering::Relaxed);
        assert_eq!((l(&c.hits), l(&c.below), l(&c.equal), l(&c.above), l(&c.tm1), l(&c.tp1), l(&c.cond_true)), (4, 1, 1, 2, 1, 1, 3));
        assert_eq!((!l(&c.min_key), l(&c.max_key)), (29, 1000));
        unsafe { platform::swap_page_bytes(&[(page + 2, 0xcc, 0x48)], &mut res).unwrap(); }
        assert_eq!(f(31), 1);
        assert_eq!(l(&c.hits), 4); // 해제 뒤에는 세지 않음
        TABLE.store(std::ptr::null_mut(), Ordering::Release);
        BASE.store(0, Ordering::Release);
    }
}
