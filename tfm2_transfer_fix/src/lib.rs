// tfm2_transfer_fix — 0.7.0 이적·연봉 산식의 계산 오류 3가지를 바로잡는다
// =====================================================================================
// 근거 = 정적 RE(게임 0.7.0 · exe sha256 D678E012… · 2026-11-01)
//   REPORT\tfm2_transfer_tweak\RE\2026-11-01_0.7.0_*  (A·B·C·D·E·F·G · 실측·설계판정)
//   이 모드 문서 = REPORT\tfm2_transfer_fix\01_구조.md · 02_구현정보.md
//
// ① AI 판매 호가 하한 — `0x1bdbdb0`(transfer\planner, ask.rs 인접)
//    s = c + c·tanh((max(r,c) − c)/c), c = 0.95 ⟹ s ≥ 0.95 ⟹ 호가 = 기준가 × e^s ≥ ×2.586(0.6.5 엔 없던 배수).
//    tanh 부분은 r>c 를 부드럽게 눌러 2c 로 수렴시키는 「소프트 상한」 모양인데, 앞의 max(r,c) 때문에
//    보통 경우(r<c)가 전부 c 로 끌려 올라간다(실수로 판단 · 확신 중간).
//    수정: `maxsd xmm0,xmm6`(4B)를 NOP → s = c + c·tanh((r−c)/c)  (r=0 → 0.238c, r=c → c, r→∞ → 2c).
//          비교 표본이 없을 때의 기본값 0.95 도 cfg 값으로. c 상수(0.95 / −0.95) 로드 2곳과 기본값 로드 3곳은
//          rip-rel disp32 를 우리 상수 칸으로 재지향한다 — 원 상수(.rdata)는 다른 코드도 읽을 수 있어 건드리지 않는다.
//    파급: 같은 s 가 셀러 바닥가(base·e^{s·t})와 재계약 동료시장 견인 상한(e^s)에도 쓰여 함께 낮아진다.
//
// ② 10분위 연봉 보정 첫 생성 이중 적용 — `0x150c860`(SalaryBasis 갱신, 연도 전환 때 1회)
//    pay_corrections 가 비어 있으면(실측: 0.7.0 첫 시즌 세이브 전부) 「방금 갱신한」 전체 pay_correction 으로
//    10칸을 채운 뒤 같은 해 분위 인자를 또 곱한다 ⟹ 같은 해 인자가 두 번 곱해진다(확실한 실수).
//    수정: 초기값 로드 `movsd xmm6,[rbx+0xa0]`(8B)를 `xorpd xmm6,xmm6 + nop4` 로 →
//          바로 뒤 원본 코드의 (값 ≤ 0 이면 1.0) 분기가 초기값을 1.0 으로 만든다.
//
// ③ 연봉·선수가치 배율 P 상·하한 — `0x150ca20(rcx=SalaryBasis*, xmm1=능력값) → xmm0=P`, 호출자 13곳
//    P 는 해마다 상한 없이 누적 곱해지고 백분위 누적최대를 거친다 ⟹ 유저 팀 굿즈 수입이 크면 상위 1% 가 9~18배.
//    수정: 함수 입구 5B(push r14/rsi/rdi/rbx)를 jmp rel32 로 → 스텁이 원본을 부른 뒤 xmm0 을 max 로 자르고,
//          min > 0 일 때만 min 으로 올린다. 스텁은 실측 통계(호출 수 · 원래 P 최솟값/최댓값 · 상·하한에 걸린 횟수)를 남긴다.
//    ★이건 계산 실수 수정이 아니라 게임에 없던 안전장치다(설계 의도 = 리그 재정 되먹임, 조율 실패).
//      그래서 기본값 = 상한 5.0 · 하한 없음 — 평소 게임 계산은 그대로 두고 폭주할 때만 자른다(유저 결정 2026-11-02).
//      ~~v0.1.0 기본 0.8~2.0~~: ② 수정과 겹쳐 하한 0.8 이 게임 식의 낮은 값(0.67 등)을 끌어올렸다(실측 164만 회).
//
// 안전 규칙
//   · 모든 패치는 ①원본 바이트 ②(재지향이면) 원 대상 RVA ③원 상수 값을 대조하고, 하나라도 다르면 그 패치만 건너뛰고 로그.
//   · 이미 우리 패치 모양이면 그대로 둔다(멱등). 게임 패치로 RVA 가 어긋나면 전부 스킵 = 바닐라 동작(재핀 필요).
//   · 입구가 이미 다른 훅(e9/48 b8)이면 체인하지 않고 건너뛴다(이 함수를 거는 다른 모드는 없다 — 2026-11-01).
// =====================================================================================

use mod_api_stable::{declare_stable_mod, LogLevel, StableHost, StableMod};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "tfm2_transfer_fix";
const CFG_NAME: &str = "tfm2_transfer_fix.cfg";
const LOG_NAME: &str = "tfm2_transfer_fix.txt";
const GAME_VER: &str = "0.7.1";

// ── ① s 식 (0.7.1 · c 레지스터 xmm6→xmm7) ──
const RVA_S_MAX: usize = 0x140de7b;                               // maxsd xmm0, xmm7  (s 하한)
const ORIG_S_MAX: [u8; 4] = [0xf2, 0x0f, 0x5f, 0xc7];
const NOP4: [u8; 4] = [0x0f, 0x1f, 0x40, 0x00];                   // nop dword [rax+0]

/// rip-rel 상수 로드 재지향 지점. 명령 = prefix(4B) + disp32, 길이 8B.
struct RipSite { name: &'static str, rva: usize, prefix: [u8; 4], old_tgt: usize, old_val: f64, slot: usize }
const RIP_SITES: [RipSite; 5] = [
    // movsd xmm7,[c]          — s 식의 c (0.95)
    RipSite { name: "s.c",    rva: 0x140de73, prefix: [0xf2, 0x0f, 0x10, 0x3d], old_tgt: 0x4104d90, old_val: 0.95,  slot: SLOT_C },
    // addsd xmm0,[-c]         — s 식의 −c (−0.95)
    RipSite { name: "s.-c",   rva: 0x140de7f, prefix: [0xf2, 0x0f, 0x58, 0x05], old_tgt: 0x4104d98, old_val: -0.95, slot: SLOT_NEGC },
    // movsd xmm0,[0.95]       — 비교 표본 없음 기본값 ①
    RipSite { name: "s.def1", rva: 0x140ddda, prefix: [0xf2, 0x0f, 0x10, 0x05], old_tgt: 0x4104d90, old_val: 0.95,  slot: SLOT_DEF },
    // movsd xmm0,[0.95]       — 비교 표본 없음 기본값 ②
    RipSite { name: "s.def2", rva: 0x140de09, prefix: [0xf2, 0x0f, 0x10, 0x05], old_tgt: 0x4104d90, old_val: 0.95,  slot: SLOT_DEF },
    // movapd xmm0,[{0.95,0}]  — 기본값 ③ (16B 정렬 읽기 → 슬롯도 16B 정렬)
    RipSite { name: "s.def3", rva: 0x140dfdb, prefix: [0x66, 0x0f, 0x28, 0x05], old_tgt: 0x4104da0, old_val: 0.95,  slot: SLOT_DEF },
];

// ── ② 10분위 보정 초기값 (0.7.1 — 게임이 호출 순서를 고쳐 초기값이 이미 1.0 · 패치는 결과 불변 · RE 2026-11-04_0.7.1-재핀판정-로직대조) ──
const RVA_PCS_INIT: usize = 0x1ca4a7a;
const ORIG_PCS_INIT: [u8; 8] = [0xf2, 0x0f, 0x10, 0xb3, 0xa0, 0x00, 0x00, 0x00]; // movsd xmm6,[rbx+0xa0]
const NEW_PCS_INIT: [u8; 8] = [0x66, 0x0f, 0x57, 0xf6, 0x0f, 0x1f, 0x40, 0x00];  // xorpd xmm6,xmm6 ; nop4

// ── ③ 배율 함수 입구 (0.7.1) ──
const RVA_P_FN: usize = 0x1ca4c00;
const ORIG_P_FN: [u8; 9] = [0x41, 0x56, 0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x48]; // push r14/rsi/rdi/rbx ; sub rsp,0x48
const STOLEN: usize = 5;                                                           // 위 push 4개 = 위치 무관 명령

// ── 우리 페이지(엑스 이미지 ±2GB 안 RWX 4KB) 배치 ──
const SLOT_C: usize = 0x00;      // f64 c
const SLOT_NEGC: usize = 0x08;   // f64 −c
const SLOT_DEF: usize = 0x10;    // f64 기본 s, +0x18 = 0.0 (movapd 용 16B)
const SLOT_PLO: usize = 0x20;    // f64 P 하한
const SLOT_PHI: usize = 0x28;    // f64 P 상한
const SLOT_RAWMIN: usize = 0x30; // f64 원래 P 최솟값(통계)
const SLOT_RAWMAX: usize = 0x38; // f64 원래 P 최댓값(통계)
const SLOT_CALLS: usize = 0x40;  // u64 배율 함수 호출 수
const SLOT_HI: usize = 0x48;     // u64 상한에 걸린 횟수
const SLOT_LO: usize = 0x50;     // u64 하한에 걸린 횟수
const STUB_OFF: usize = 0x100;   // 스텁 코드
const PAGE: usize = 0x1000;

static PAGE_BASE: AtomicUsize = AtomicUsize::new(0);

type BOOL = i32; type DWORD = u32; type HMODULE = usize;
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleHandleExW(flags: u32, addr: *const u16, h: *mut HMODULE) -> BOOL;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, sz: DWORD) -> DWORD;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old: *mut u32) -> BOOL;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, sz: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
}
#[repr(C)] #[derive(Default)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32,
    region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32 }

#[inline] unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const RD: u32 = 0x02|0x04|0x20|0x40; const GUARD: u32 = 0x01|0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}

fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }
fn dir() -> Option<PathBuf> { unsafe {
    let mut h: HMODULE = 0;
    if GetModuleHandleExW(0x4|0x2, dir as *const () as *const u16, &mut h) == 0 || h == 0 { return None; }
    let mut b = [0u16; 4096];
    let n = GetModuleFileNameW(h, b.as_mut_ptr(), b.len() as DWORD);
    if n == 0 { return None; }
    let mut p = PathBuf::from(String::from_utf16_lossy(&b[..n as usize])); p.pop(); Some(p)
}}
fn log(s: &str) {
    if let Some(mut p) = dir() {
        p.push(LOG_NAME);
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) {
            let _ = write!(f, "{}", s); let _ = f.flush();
        }
    }
}

// ───────────────────────────── 설정 ─────────────────────────────

struct Cfg {
    spread_floor_fix: bool, spread_c: f64, spread_default: f64,
    double_apply_fix: bool,
    mult_clamp: bool, mult_min: f64, mult_max: f64,
}
impl Default for Cfg {
    fn default() -> Self {
        Cfg { spread_floor_fix: true, spread_c: 0.95, spread_default: 0.23,
              double_apply_fix: true,
              mult_clamp: true, mult_min: 0.0, mult_max: 5.0 }   // mult_min 0 = 하한 없음
    }
}

const CFG_TEMPLATE: &str = "\
# tfm2_transfer_fix 설정 (게임 0.7.1)
# 형식: 키 = 값   (# 로 시작하는 줄은 무시. 고친 뒤 게임을 다시 켜야 적용)

# ① AI 판매 호가 하한 제거 (1 = 켬, 0 = 끔)
#    바닐라: AI 가 선수를 팔 때 항상 기준가의 2.59배 이상을 부른다.
#    켜면: 호가 배수가 시장 상황에 따라 약 1.25배 ~ (드물게) 6.7배 사이로 움직인다.
spread_floor_fix = 1
# 호가 폭 기준값 c (바닐라 0.95). 배수 범위 = e^(0.238c) ~ e^(2c)
spread_c = 0.95
# 비교할 시장 표본이 없을 때 쓰는 호가 폭 (바닐라 0.95 = 2.59배 · 0.23 = 1.26배)
spread_default = 0.23

# ② 첫 시즌 종료 때 연봉 보정이 두 번 곱해지는 오류 수정 (1 = 켬, 0 = 끔)
#    0.7.1 부터 게임이 직접 고쳤다. 켜 둬도 결과는 같다.
double_apply_fix = 1

# ③ 연봉·선수가치 배율 상한 (1 = 켬, 0 = 끔 · 바닐라는 제한 없음)
#    바닐라: 배율이 해마다 상한 없이 누적돼, 시즌이 쌓이면 상위 선수 연봉·이적료가 크게 치솟을 수 있다.
#    기본 상한 5.0 = 평소엔 거의 걸리지 않고 폭주할 때만 자르는 안전장치(게임 계산은 최대한 그대로).
#    ★연봉·이적료가 너무 많이 오른다 싶으면 salary_mult_max 를 낮추세요 (예: 3.0, 더 세게는 2.0).
#      참고: 0.7.0 첫 연도 전환 직후 배율은 0.67 ~ 1.85 였다(② 수정 켬 · 2026-11 실측).
salary_mult_clamp = 1
salary_mult_max = 5.0
#    하한 (0 = 없음 · 기본). 낮은 연봉이 너무 떨어진다 싶을 때만 0.05 ~ 1 사이 값을 넣으세요 (예: 0.8).
salary_mult_min = 0
";

fn load_config() -> (Cfg, Vec<String>) {
    let mut c = Cfg::default();
    let mut notes = Vec::new();
    let path = match dir() { Some(mut p) => { p.push(CFG_NAME); p }, None => {
        notes.push("cfg 경로 확인 실패 → 기본값".into()); return (c, notes); } };
    if !path.exists() {
        let _ = fs::write(&path, CFG_TEMPLATE);   // BOM 없는 UTF-8
        notes.push(format!("{} 없음 → 기본값으로 생성", CFG_NAME));
        return (c, notes);
    }
    let text = match fs::read_to_string(&path) { Ok(t) => t, Err(e) => {
        notes.push(format!("cfg 읽기 실패({}) → 기본값", e)); return (c, notes); } };
    for raw in text.trim_start_matches('\u{feff}').lines() {
        let l = raw.trim();
        if l.is_empty() || l.starts_with('#') || l.starts_with("//") { continue; }
        let Some((k, v)) = l.split_once('=') else { notes.push(format!("무시(= 없음): {}", l)); continue; };
        let (k, v) = (k.trim(), v.trim());
        let b = |v: &str| matches!(v, "1" | "true" | "on" | "yes");
        let f = |v: &str| v.parse::<f64>().ok().filter(|x| x.is_finite());
        match k {
            "spread_floor_fix" => c.spread_floor_fix = b(v),
            "double_apply_fix" => c.double_apply_fix = b(v),
            "salary_mult_clamp" => c.mult_clamp = b(v),
            "spread_c" => match f(v) { Some(x) if (0.05..=3.0).contains(&x) => c.spread_c = x,
                _ => notes.push(format!("spread_c={} 범위(0.05~3) 밖 → {}", v, c.spread_c)) },
            "spread_default" => match f(v) { Some(x) if (0.01..=3.0).contains(&x) => c.spread_default = x,
                _ => notes.push(format!("spread_default={} 범위(0.01~3) 밖 → {}", v, c.spread_default)) },
            "salary_mult_min" => match f(v) { Some(x) if x == 0.0 || (0.05..=1.0).contains(&x) => c.mult_min = x,
                _ => notes.push(format!("salary_mult_min={} 범위(0 또는 0.05~1) 밖 → {}", v, c.mult_min)) },
            "salary_mult_max" => match f(v) { Some(x) if (1.0..=100.0).contains(&x) => c.mult_max = x,
                _ => notes.push(format!("salary_mult_max={} 범위(1~100) 밖 → {}", v, c.mult_max)) },
            _ => notes.push(format!("모르는 키 무시: {}", k)),
        }
    }
    (c, notes)
}

// ───────────────────────────── 패치 도구 ─────────────────────────────

/// rel32 사거리(±2GB) 안에 RWX 페이지를 잡는다 — 대상 주변을 64KB 단위로 왕복 스캔.
unsafe fn alloc_near(target: usize, size: usize) -> usize {
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let base = target & !0xFFFF;
    for step in 1..0x7F00usize {
        for d in [1i64, -1] {
            let addr = (base as i64 + d * (step as i64) * 0x10000) as usize;
            if addr < 0x10000 { continue; }
            let p = VirtualAlloc(addr, size, MEM_CR, RWX);
            if p != 0 {
                let dist = (p as i64) - (target as i64);
                if dist.abs() < 0x7000_0000 { return p; }
            }
        }
    }
    0
}

unsafe fn read_bytes<const N: usize>(addr: usize) -> [u8; N] {
    let mut b = [0u8; N];
    core::ptr::copy_nonoverlapping(addr as *const u8, b.as_mut_ptr(), N);
    b
}

unsafe fn write_code(addr: usize, bytes: &[u8]) -> Result<(), String> {
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(addr, bytes.len(), RWX, &mut old) == 0 { return Err("VirtualProtect 실패".into()); }
    core::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, bytes.len());
    VirtualProtect(addr, bytes.len(), old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, bytes.len());
    let mut back = vec![0u8; bytes.len()];
    core::ptr::copy_nonoverlapping(addr as *const u8, back.as_mut_ptr(), bytes.len());
    if back != bytes { return Err(format!("쓰기 미반영 landed={:02x?}", back)); }
    Ok(())
}

/// 고정 길이 바이트 교체. 이미 new 모양이면 그대로(멱등), orig 와 다르면 스킵.
unsafe fn patch_fixed(base: usize, rva: usize, orig: &[u8], new: &[u8]) -> Result<String, String> {
    let site = base.wrapping_add(rva);
    if !readable(site, orig.len()) { return Err(format!("읽기 불가 @0x{:x}", rva)); }
    let mut cur = vec![0u8; orig.len()];
    core::ptr::copy_nonoverlapping(site as *const u8, cur.as_mut_ptr(), orig.len());
    if cur == new { return Ok("이미 적용됨".into()); }
    if cur != orig { return Err(format!("바이트 불일치 @0x{:x} cur={:02x?} want={:02x?} (RVA stale?)", rva, cur, orig)); }
    write_code(site, new)?;
    Ok("적용".into())
}

/// rip-rel 상수 로드의 disp32 를 우리 슬롯으로 돌린다. 3중 대조(prefix · 원 대상 RVA · 원 상수 값).
unsafe fn redirect(base: usize, page: usize, s: &RipSite) -> Result<String, String> {
    let site = base.wrapping_add(s.rva);
    if !readable(site, 8) { return Err(format!("읽기 불가 @0x{:x}", s.rva)); }
    let cur: [u8; 8] = read_bytes(site);
    if cur[..4] != s.prefix { return Err(format!("명령 불일치 @0x{:x} cur={:02x?} want prefix={:02x?}", s.rva, cur, s.prefix)); }
    let disp = i32::from_le_bytes([cur[4], cur[5], cur[6], cur[7]]) as i64;
    let tgt = (site as i64 + 8 + disp) as usize;
    let want = page + s.slot;
    if tgt == want { return Ok("이미 적용됨".into()); }
    if tgt != base.wrapping_add(s.old_tgt) {
        return Err(format!("원 대상 불일치 @0x{:x} tgt_rva=0x{:x} want=0x{:x}", s.rva, tgt.wrapping_sub(base), s.old_tgt));
    }
    if !readable(tgt, 8) { return Err("원 상수 읽기 불가".into()); }
    let v = core::ptr::read_unaligned(tgt as *const f64);
    if v != s.old_val { return Err(format!("원 상수 값 불일치 {} (want {})", v, s.old_val)); }
    let nd = want as i64 - (site as i64 + 8);
    if nd < i32::MIN as i64 || nd > i32::MAX as i64 { return Err("disp32 범위 초과".into()); }
    write_code(site + 4, &(nd as i32).to_le_bytes())?;
    Ok(format!("적용 → 슬롯 +0x{:x}", s.slot))
}

fn emit_mov_rax(s: &mut Vec<u8>, imm: u64) { s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&imm.to_le_bytes()); }

/// ③ 배율 함수 스텁 + 트램폴린을 page+STUB_OFF 에 조립. 반환 = (스텁 주소, 트램폴린 주소).
///   스텁: sub rsp,0x28 → call tramp(원본) → add rsp,0x28 → 통계 → [상한] → [하한] → ret
///   진입 rsp ≡ 8 (mod 16) → sub 0x28 후 call 하면 원본 진입 정렬이 원래와 같다. 원본의 홈 영역 = 우리가 비운 0x28.
///   하한을 끄면(min = 0) 하한 코드를 아예 넣지 않는다 — NaN 도 바닐라처럼 그대로 통과.
unsafe fn build_p_stub(page: usize, fn_abs: usize, clamp_hi: bool, clamp_lo: bool) -> (usize, usize) {
    let stub = page + STUB_OFF;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x48, 0x83, 0xec, 0x28]);                 // sub rsp,0x28
    let call_at = s.len(); s.extend_from_slice(&[0xe8, 0, 0, 0, 0]); // call tramp (나중에 채움)
    s.extend_from_slice(&[0x48, 0x83, 0xc4, 0x28]);                 // add rsp,0x28
    // 통계: 호출 수
    emit_mov_rax(&mut s, (page + SLOT_CALLS) as u64);
    s.extend_from_slice(&[0xf0, 0x48, 0xff, 0x00]);                 // lock inc qword [rax]
    // 통계: 원래 P 최댓값 / 최솟값 (경쟁 쓰기 허용 — 진단용)
    emit_mov_rax(&mut s, (page + SLOT_RAWMAX) as u64);
    s.extend_from_slice(&[0xf2, 0x0f, 0x10, 0x08]);                 // movsd xmm1,[rax]
    s.extend_from_slice(&[0xf2, 0x0f, 0x5f, 0xc8]);                 // maxsd xmm1,xmm0
    s.extend_from_slice(&[0xf2, 0x0f, 0x11, 0x08]);                 // movsd [rax],xmm1
    emit_mov_rax(&mut s, (page + SLOT_RAWMIN) as u64);
    s.extend_from_slice(&[0xf2, 0x0f, 0x10, 0x08]);                 // movsd xmm1,[rax]
    s.extend_from_slice(&[0xf2, 0x0f, 0x5d, 0xc8]);                 // minsd xmm1,xmm0
    s.extend_from_slice(&[0xf2, 0x0f, 0x11, 0x08]);                 // movsd [rax],xmm1
    if clamp_hi {
        // 상한: xmm0 > hi 이면 hi (NaN 이면 건너뜀)
        emit_mov_rax(&mut s, (page + SLOT_PHI) as u64);
        s.extend_from_slice(&[0xf2, 0x0f, 0x10, 0x08]);             // movsd xmm1,[rax]
        s.extend_from_slice(&[0x66, 0x0f, 0x2e, 0xc1]);             // ucomisd xmm0,xmm1
        let jbe_at = s.len(); s.extend_from_slice(&[0x76, 0x00]);   // jbe no_hi
        emit_mov_rax(&mut s, (page + SLOT_HI) as u64);
        s.extend_from_slice(&[0xf0, 0x48, 0xff, 0x00]);             // lock inc qword [rax]
        s.extend_from_slice(&[0x66, 0x0f, 0x28, 0xc1]);             // movapd xmm0,xmm1
        s[jbe_at + 1] = (s.len() - (jbe_at + 2)) as u8;
    }
    if clamp_lo {
        // 하한: xmm0 < lo 또는 NaN 이면 lo
        emit_mov_rax(&mut s, (page + SLOT_PLO) as u64);
        s.extend_from_slice(&[0xf2, 0x0f, 0x10, 0x08]);             // movsd xmm1,[rax]
        s.extend_from_slice(&[0x66, 0x0f, 0x2e, 0xc1]);             // ucomisd xmm0,xmm1
        let jae_at = s.len(); s.extend_from_slice(&[0x73, 0x00]);   // jae no_lo
        emit_mov_rax(&mut s, (page + SLOT_LO) as u64);
        s.extend_from_slice(&[0xf0, 0x48, 0xff, 0x00]);             // lock inc qword [rax]
        s.extend_from_slice(&[0x66, 0x0f, 0x28, 0xc1]);             // movapd xmm0,xmm1
        s[jae_at + 1] = (s.len() - (jae_at + 2)) as u8;
    }
    s.push(0xc3);                                                   // ret
    // 트램폴린: 떼어 낸 원본 5B + jmp 원본+5
    while s.len() % 16 != 0 { s.push(0xcc); }
    let tramp_off = s.len();
    s.extend_from_slice(&ORIG_P_FN[..STOLEN]);
    let back = (fn_abs + STOLEN) as i64 - (stub + s.len() + 5) as i64;
    s.push(0xe9); s.extend_from_slice(&(back as i32).to_le_bytes());
    let rel = tramp_off as i64 - (call_at + 5) as i64;
    s[call_at + 1..call_at + 5].copy_from_slice(&(rel as i32).to_le_bytes());

    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());
    (stub, stub + tramp_off)
}

unsafe fn hook_p_fn(base: usize, page: usize, clamp_hi: bool, clamp_lo: bool) -> Result<String, String> {
    let site = base.wrapping_add(RVA_P_FN);
    if !readable(site, ORIG_P_FN.len()) { return Err(format!("읽기 불가 @0x{:x}", RVA_P_FN)); }
    let cur: [u8; 9] = read_bytes(site);
    if cur[0] == 0xe9 || (cur[0] == 0x48 && cur[1] == 0xb8) {
        return Err(format!("입구가 이미 다른 훅({:02x?}) — 체인 안 함, 건너뜀", &cur[..5]));
    }
    if cur != ORIG_P_FN { return Err(format!("바이트 불일치 cur={:02x?} want={:02x?} (RVA stale?)", cur, ORIG_P_FN)); }
    let (stub, tramp) = build_p_stub(page, site, clamp_hi, clamp_lo);
    let rel = stub as i64 - (site as i64 + 5);
    if rel < i32::MIN as i64 || rel > i32::MAX as i64 { return Err("rel32 범위 초과".into()); }
    let mut j = [0u8; 5]; j[0] = 0xe9; j[1..5].copy_from_slice(&(rel as i32).to_le_bytes());
    write_code(site, &j)?;
    Ok(format!("훅 @0x{:x} stub=0x{:x} tramp=0x{:x} 상한={} 하한={}", site, stub, tramp, clamp_hi, clamp_lo))
}

// ───────────────────────────── 진입 ─────────────────────────────

fn init(host: &StableHost) -> StableMod {
    let (cfg, notes) = load_config();
    log(&format!("\n[{}ms] === {} INIT (게임 {}) ===\n", now_ms(), MOD_ID, GAME_VER));
    for n in &notes { log(&format!("[cfg] {}\n", n)); }
    let lo_txt = if cfg.mult_min > 0.0 { format!("{}", cfg.mult_min) } else { "없음".to_string() };
    log(&format!("[cfg] ①호가하한제거={} c={} 기본s={} | ②이중적용수정={} | ③배율제한={} 상한={} 하한={}\n",
                 cfg.spread_floor_fix, cfg.spread_c, cfg.spread_default, cfg.double_apply_fix,
                 cfg.mult_clamp, cfg.mult_max, lo_txt));

    let mut results: Vec<(String, Result<String, String>)> = Vec::new();
    unsafe {
        let base = GetModuleHandleW(core::ptr::null());
        let page = if base != 0 { alloc_near(base.wrapping_add(RVA_P_FN), PAGE) } else { 0 };
        if base == 0 || page == 0 {
            log(&format!("[selfcheck] FAIL — base=0x{:x} page=0x{:x} (근처 메모리 확보 실패) · 아무것도 바꾸지 않음\n", base, page));
            host.log(LogLevel::Warn, "tfm2_transfer_fix: page alloc failed");
            return StableMod::new(MOD_ID);
        }
        PAGE_BASE.store(page, Ordering::Relaxed);
        // 슬롯 초기화
        let wf = |off: usize, v: f64| core::ptr::write_unaligned((page + off) as *mut f64, v);
        wf(SLOT_C, cfg.spread_c); wf(SLOT_NEGC, -cfg.spread_c);
        wf(SLOT_DEF, cfg.spread_default); wf(SLOT_DEF + 8, 0.0);
        wf(SLOT_PLO, cfg.mult_min); wf(SLOT_PHI, cfg.mult_max);
        wf(SLOT_RAWMIN, f64::INFINITY); wf(SLOT_RAWMAX, f64::NEG_INFINITY);

        if cfg.spread_floor_fix {
            results.push(("①s 하한 NOP".into(), patch_fixed(base, RVA_S_MAX, &ORIG_S_MAX, &NOP4)));
            for s in RIP_SITES.iter() {
                results.push((format!("①{}", s.name), redirect(base, page, s)));
            }
        }
        if cfg.double_apply_fix {
            results.push(("②10분위 초기값 1.0".into(), patch_fixed(base, RVA_PCS_INIT, &ORIG_PCS_INIT, &NEW_PCS_INIT)));
        }
        // ③ 훅은 통계를 위해 제한이 꺼져 있어도 설치한다(clamp 코드만 빠짐).
        results.push(("③배율 훅".into(), hook_p_fn(base, page, cfg.mult_clamp, cfg.mult_clamp && cfg.mult_min > 0.0)));
    }
    let fails: Vec<&(String, Result<String, String>)> = results.iter().filter(|(_, r)| r.is_err()).collect();
    if fails.is_empty() {
        log(&format!("[selfcheck] PASS {}/{}\n", results.len(), results.len()));
    } else {
        log(&format!("[selfcheck] FAIL {}/{} 실패 — 게임 버전이 {} 가 아니면 재핀 필요(REPORT\\tfm2_transfer_fix\\02_구현정보.md)\n",
                     fails.len(), results.len(), GAME_VER));
    }
    for (name, r) in &results {
        match r { Ok(m) => log(&format!("[patch] {} : {}\n", name, m)),
                  Err(e) => log(&format!("[patch] {} : 실패 — {}\n", name, e)) }
    }

    // 진단 스레드: 배율 함수 통계가 바뀌면 30초마다 기록(원래 P 최솟값·최댓값 = 실측).
    std::thread::spawn(|| {
        let mut last = u64::MAX;
        loop {
            std::thread::sleep(std::time::Duration::from_secs(30));
            let page = PAGE_BASE.load(Ordering::Relaxed);
            if page == 0 { continue; }
            unsafe {
                let rd = |off: usize| core::ptr::read_volatile((page + off) as *const u64);
                let rf = |off: usize| core::ptr::read_volatile((page + off) as *const f64);
                let calls = rd(SLOT_CALLS);
                if calls == 0 || calls == last { continue; }
                last = calls;
                log(&format!("[{}ms] 배율 호출={} 원래P 최소={:.4} 최대={:.4} | 상한 걸림={} 하한 걸림={}\n",
                             now_ms(), calls, rf(SLOT_RAWMIN), rf(SLOT_RAWMAX), rd(SLOT_HI), rd(SLOT_LO)));
            }
        }
    });
    host.log(LogLevel::Info, "tfm2_transfer_fix (stable shell)");
    StableMod::new(MOD_ID)
}
declare_stable_mod!(init);
