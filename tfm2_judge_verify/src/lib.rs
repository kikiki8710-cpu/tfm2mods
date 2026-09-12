//! tfm2_judge_verify — AI 판단함수 20종 검증 모드. **1단계 = 발화수(호출 횟수) 측정 전용.**
//! ===========================================================================
//! 왜 새 모드인가: `tfm2_ai_adjust`(7.7MB · 여러 버전 누적 · T1 프로덕션)에 검증을 얹으면
//!   ① `DIFF≠0` 의 원인 귀속이 안 되고 ② 위험 cfg 비트(기록된 게임 즉사 3종)를 프로덕션 dll 에서
//!   켜야 한다. ⟹ 검증은 **깨끗한 새 모드**에서 한다(유저 결정 2026-09-12).
//!
//! 1단계 범위 = **발화수만**. 대조(sweep)는 2단계다. 이 순서는 프로젝트 규율이다 —
//!   `DONE.md` 「DIFF=0 은 **표본 수와 함께**」 · 교훈 「**발화 0 = 검증 표본 불성립**」.
//!   (실제로 명세 `07` 은 포팅까지 해 놓고 **발화 0 = 사장**으로 판정된 선례가 있다.)
//!
//! 구성
//!   src\probe20_tbl.rs  ★자동생성(`MIG\probe20.py`) — 손대지 말 것.
//!                       PROBES20(진입부) + **CALLSITES20(호출부)** + MISSING20(제외·사유).
//!   src\probe.rs        카운트 스텁 설치 + 집계(이관 원본 = ai_adjust\src\judge\probe.rs).
//!                       종류 2가지 = **진입부 프로브**(12B 스틸) · **호출부 프로브**(`call rel32`
//!                       의 rel32 만 우회 — 진입부를 못 빼는 #13 용. 원 함수 명령 무손상).
//!   src\lib.rs          이 파일 — 생명주기 배선 · WinAPI · 경로 도출 · 덤프 타이밍.
//!
//! 출력(게임 exe/dll 기준 **동적 도출** — install 경로 하드코딩 금지)
//!   <게임>\mods\tfm2_judge_verify\probe20.txt        발화수 표(중간 스냅샷 + 판 종료 확정치로 덮어씀)
//!   <게임>\mods\tfm2_judge_verify\probe20_stubs.txt  RWX 스텁 인벤토리(크래시 `module=unknown` 역해석용)
//!   <게임>\mods\tfm2_judge_verify\probe20_off.txt    ★이 파일이 있으면 설치하지 않는다(재빌드 없는 비활성)
//!
//! ★★2단계 = **sweep**(2026-09-12 추가) — `src\sweep20.rs`(자동생성 `MIG\gensweep20.py`)
//!   게임 원본 함수와 **내 dll 안 링크사본**(아래 `extern crate game_ai;` = SDK rlib 640함수 =
//!   재현 정본)을 **같은 인자로 각각 호출해 반환을 비트동일 대조**한다.
//!   · 출력 = `<게임>\mods\tfm2_judge_verify\sweep20.txt` (★1단계 `probe20.txt` 와 **별도 파일**)
//!   · 게이트 = `sweep20_on.txt` 의 비트마스크. **파일 없음 = 기본 OFF**(한 곳도 안 건다).
//!   · ★★sweep 과 진입부 프로브는 **같은 함수에 공존 불가**(둘 다 진입부 12B 패치) ⟹ sweep 이
//!     설치에 성공한 함수는 프로브를 **안 건다**(`probe.rs` 의 `SWEPT`). 그래서 **sweep 을 먼저**
//!     설치한다. 표기 = `[sweep]` / `[진입부]` / `[호출부]`.
//!   · 빌드 = `-NoDeploy -MaxSize 8000000`. ⚠**실측 정정**: 링크사본 때문에 dll 이 수 MB 가 될 거라는
//!     1단계의 예고는 틀렸다 — `208,384 → 299,520 B`(**+91,136**) 뿐이다. 비-LTO rlib 링크는
//!     **참조된 오브젝트 멤버만** 끌어오므로 5함수와 그 전이 의존 CGU 만 들어온다(가드 1.3MB 도 통과).
//!
//! ★★★3단계 = **abiprobe**(2026-09-12 추가) — `src\abiprobe.rs`(손작성)
//!   2단계 sweep 의 `#04`(bit3)가 `0xc0000005` 로 게임을 죽였다. 원인 = exe 의 그 함수는 **LTO 로
//!   ABI 가 바뀌어** IR `define` 의 인자 순서와 다른데 래퍼가 IR 순서를 가정하고 내 링크사본에
//!   넘겼다(= 엉뚱한 값을 포인터로 역참조). `MIG\abiagree.py` 의 정적 재판정이 런타임과 일치했다:
//!   ✅`#09`·`#01`·`#08`(DIFF=0 확보) / ⛔**`#04`·`#16` = ABI 불일치**.
//!   ⟹ 번역 래퍼를 짜기 **전에** 「인자를 아무 데도 넘기지 않고 레지스터만 보는」 단계를 넣는다.
//!   · 대상 = `#04`(0xd3e4b0) · `#16`(0xc809d0) — abiagree 가 ⛔로 찍은 둘뿐.
//!   · 출력 = `<게임>\mods\tfm2_judge_verify\abiprobe.txt` (★probe20/sweep20 과 **별도 파일**)
//!   · 게이트 = `abiprobe_on.txt` **존재 = 설치**(없으면 미설치). 선택 줄 = 비트마스크 / `n=<관측수>`.
//!   · ⛔**`sweep20_on.txt` 와 동시 사용 금지**(같은 진입부 12B) — 코드가 3중으로 막는다(abiprobe.rs 헤더 ㉠㉡㉢).
//!   · `my_*`(링크사본) **호출 없음** · 레지스터·스택·플래그 무변 · 모든 역참조 전 `readable()`.
//! ===========================================================================
#![allow(dead_code)]
/// ★2단계 sweep 의 「내 사본」. 이게 있어야 `sweep20.rs` 의 `#[link_name]` 이 rlib 안 구현으로
///   해석된다(= 손포팅이 아니라 **재현 정본** 링크. `project-judge-port-all-at-once` 가 손포팅을 폐지했다).
extern crate game_ai;
use mod_api::*;
use std::fs;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};

#[path = "probe20_tbl.rs"]
mod probe20_tbl;
#[path = "probe.rs"]
mod probe;
#[path = "hookw.rs"]
mod hookw;
#[path = "sweep20.rs"]
mod sweep20;
#[path = "abiprobe.rs"]
mod abiprobe;
/// `#02` midpin sweep(2026-09-13) — 게이트 mask bit19(0x80000).
mod pin02;
/// `#02` midpin 전용 게이트 비트 = 0x4000000000000000 (gensweep20 자동배정 0..~60 밖 · 09-13).
pub const PIN02_BIT: u32 = 62;

const MOD_ID: &str = "tfm2_judge_verify";

// build_inj.ps1 신원검증(dll 안에 이번에 컴파일한 소스의 절대경로가 박혀 있어야 한다)용.
// #[used] 로 강제 유지 — 로그가 전부 최적화돼도 경로 문자열이 남는다.
#[used]
static SRC_PATH: &str = file!();

// ───────────────────────── 설치·덤프 타이밍 상수 ─────────────────────────
/// 설치 지연 프레임. ★**늦게 설치**하는 것이 의도다(CLAUDE.md §3).
///   `tfm2_ai_adjust` 는 자기 `init()`(= 모드 로드 시점)에서 judge 훅을 박는다.
///   우리가 먼저 박으면 ai_adjust 의 프롤로그 검증이 깨져 **프로덕션 모드의 훅이 미설치**된다.
///   ⟹ post_update 까지 기다려 상대가 먼저 박게 하고, 우리는 `48 b8` 을 보고 스킵한다.
///   (스킵은 손실이지만 **간섭은 사고**다. 스킵 개수는 probe20.txt 에 사유와 함께 남는다.)
const INSTALL_AT_FRAME: u64 = 60;
/// 폴링 주기(초). 카운터 총합의 변화로 「판이 돌고 있나」를 판정한다.
const POLL_SEC: f32 = 2.0;
/// 활동이 멈춘 뒤 「판 종료」로 확정할 연속 무변화 폴 수(= POLL_SEC * 이 값 만큼 조용하면 종료).
const IDLE_POLLS_TO_END: u32 = 3;

static FRAME: AtomicU64 = AtomicU64::new(0);
static INSTALLED: AtomicBool = AtomicBool::new(false);
static INSTALL_TRIED: AtomicBool = AtomicBool::new(false);
static ACC_MS: AtomicU64 = AtomicU64::new(0); // 누적 시간(ms) — 폴링 주기 판정용
static LAST_POLL_MS: AtomicU64 = AtomicU64::new(0);
static LAST_TOTAL: AtomicU64 = AtomicU64::new(0);
static ACTIVE: AtomicBool = AtomicBool::new(false); // 판이 돌고 있다(카운터가 늘고 있다)
static IDLE_POLLS: AtomicU32 = AtomicU32::new(0);
static MATCH_SEQ: AtomicUsize = AtomicUsize::new(0); // 관측한 「판 종료」 횟수
static INSTALL_OK: AtomicUsize = AtomicUsize::new(0);
static INSTALL_N: AtomicUsize = AtomicUsize::new(0);
// ── 2단계 sweep 게이트·설치 결과(리포트 헤더에 그대로 실린다) ──
static SWEEP_MASK: AtomicU64 = AtomicU64::new(0);
static SWEEP_OK: AtomicUsize = AtomicUsize::new(0);
static SWEEP_N: AtomicUsize = AtomicUsize::new(0);
static SWEEP_GATE: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
static SWEEP_LOG: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
// ── 3단계 abiprobe 게이트·설치 결과(리포트 헤더에 그대로 실린다) ──
static ABI_MASK: AtomicU64 = AtomicU64::new(0);
static ABI_NOBS: AtomicU32 = AtomicU32::new(0);
static ABI_OK: AtomicUsize = AtomicUsize::new(0);
static ABI_N: AtomicUsize = AtomicUsize::new(0);
static ABI_GATE: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
static ABI_LOG: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

// ───────────────────────── WinAPI ─────────────────────────
type BOOL = i32;
type DWORD = u32;
type HMODULE = usize;

#[repr(C)]
#[derive(Default)]
struct MemBasicInfo {
    base: usize,
    alloc_base: usize,
    alloc_protect: u32,
    _p0: u32,
    region_size: usize,
    state: u32,
    protect: u32,
    typ: u32,
    _p1: u32,
}

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleHandleExW(flags: u32, addr: *const u16, h: *mut HMODULE) -> BOOL;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, sz: DWORD) -> DWORD;
    pub fn VirtualAlloc(addr: usize, sz: usize, typ: u32, prot: u32) -> usize;
    pub fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old: *mut u32) -> BOOL;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    pub fn FlushInstructionCache(proc_: usize, addr: usize, sz: usize) -> BOOL;
    pub fn GetCurrentProcess() -> usize;
    /// 호출부 프로브의 **근접 할당**(`probe::alloc_near`)에서만 쓴다 — 힌트 스캔으로 잡은 블록이
    /// `E8 rel32` 사거리(±2GB) 검산에 떨어지면 그 자리에서 반납한다(MEM_RELEASE, size=0).
    pub fn VirtualFree(addr: usize, sz: usize, free_type: u32) -> BOOL;
}

/// ★exe 이미지 베이스. `GetModuleHandleW(null)` = exe 베이스이고 프로세스 수명 내내 고정
/// (미언로드·미재배치)이라 1회 캐시로 충분하다. **RVA = abs − base.**
/// 이관 원본 = `tfm2_ai_adjust\src\tfm2_ai_adjust.rs:1057`.
static EXE_BASE: AtomicU64 = AtomicU64::new(0);
#[inline]
pub unsafe fn exe_base() -> usize {
    let v = EXE_BASE.load(Ordering::Relaxed) as usize;
    if v != 0 {
        return v;
    }
    let b = GetModuleHandleW(core::ptr::null());
    EXE_BASE.store(b as u64, Ordering::Relaxed);
    b
}

/// 읽기 가능 검사(매 호출 VirtualQuery = point-in-time 실측).
/// 이관 원본 = `tfm2_ai_adjust\src\mem_safety.rs:264`, 단 **VEH 고속경로는 이식하지 않았다** —
///   1단계에서 이 함수는 설치 시 17회만 불린다(핫루프 아님). VEH 핸들러는 §3 의 최고위험 구역이라
///   이득 없이 들일 이유가 없다.
/// ⚠ 원본 주석의 교훈: VirtualQuery 결과는 **시간을 가로질러 캐시 불가**(게임이 sub-page 를
///   decommit 하면 캐시가 거짓 보증 → AV). 그래서 여기도 캐시하지 않는다.
pub unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 {
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

// ───────────────────── RWX 스텁 인벤토리(크래시 역해석) ─────────────────────
// 이관 원본 = `tfm2_ai_adjust\src\detour.rs:20`. **가져왔다** — 이 모드는 게임 .text 를 패치하고
//   (진입부 18곳 + 호출부 `call rel32` 2곳) RWX 스텁을 잡으므로, 거기서 폴트가 나면 WER/crash_log 가 `module=unknown` + 절대주소만
//   남긴다(어느 모듈에도 속하지 않는 VirtualAlloc 메모리). 그 주소를 이 표와 대조해야 **어느 함수의
//   스텁인지** 특정된다. 1단계에서 유일한 크래시 단서라 뺄 수 없다.
// ⚠크래시 문맥에서 읽으므로 고정 배열 + 원자 카운터만 쓴다(alloc/lock/format! 금지 — §3).
const STUB_MAX: usize = 32;
static STUB_N: AtomicUsize = AtomicUsize::new(0);
static mut STUB_TBL: [(usize, usize, usize); STUB_MAX] = [(0, 0, 0); STUB_MAX]; // (addr, size, tag=타깃RVA)

/// VirtualAlloc 결과를 그대로 통과시키며 표에 등록(할당 실패=0 이면 무등록).
#[inline]
pub unsafe fn stub_reg(addr: usize, size: usize, tag: usize) -> usize {
    if addr != 0 {
        let i = STUB_N.fetch_add(1, Ordering::Relaxed);
        if i < STUB_MAX {
            STUB_TBL[i] = (addr, size, tag);
        }
    }
    addr
}

unsafe fn stub_dump() {
    let n = STUB_N.load(Ordering::Relaxed).min(STUB_MAX);
    let mut s = format!(
        "=== tfm2_judge_verify RWX 스텁 인벤토리 (n={}) exe_base={:#x} ===\n\
         # 크래시 이벤트로그/crash_log 가 module=unknown 이면 그 절대주소를 아래 [addr, addr+size) 와 대조하라.\n\
         # tag = 타깃 함수 RVA (probe20_tbl.rs 의 rva 값). 스텁 레이아웃: +0 카운터 / +8 lock inc / +16 원본 프롤로그 / 복귀 jmp.\n",
        n,
        exe_base()
    );
    for i in 0..n {
        let (a, sz, tag) = STUB_TBL[i];
        let nm = PROBE_NAME(tag);
        s.push_str(&format!(
            "stub[{:2}] addr={:#x} size={:#x} end={:#x} tag(rva)={:#x}  {}\n",
            i,
            a,
            sz,
            a + sz,
            tag,
            nm
        ));
    }
    if let Some(p) = pth("probe20_stubs.txt") {
        let _ = fs::write(p, s);
    }
}

#[allow(non_snake_case)]
fn PROBE_NAME(rva: usize) -> String {
    // ★★abiprobe 스텁을 **가장 먼저** 본다 — 우선순위(sweep > abiprobe > probe)와 달리
    //   여기서는 「실제로 그 진입부를 잡은 쪽」이 유일하므로, 배타가 지켜졌다면 중복이 없다.
    //   abiprobe 가 설치됐다는 것 자체가 sweep·probe 는 그 함수에 없다는 뜻이다.
    for a in abiprobe::S.iter() {
        if a.rva == rva && a.stub.load(Ordering::Relaxed) != 0 {
            return format!("({} / {}) [abiprobe]", a.idx, a.name);
        }
    }
    // ★sweep 트램폴린도 같은 인벤토리에 등록된다(tag = 타깃 함수 RVA). 먼저 본다 — sweep 이 잡은
    //   함수는 진입부 프로브가 없으므로 이름이 [진입부] 로 나가면 크래시 역해석이 틀린 방향으로 간다.
    for s in sweep20::S.iter() {
        if s.rva == rva && s.orig.load(Ordering::Relaxed) != 0 {
            return format!("({} / {}) [sweep]", s.idx, s.name);
        }
    }
    for p in probe20_tbl::PROBES20.iter() {
        if p.rva == rva {
            return format!("({} / {}/{}) [진입부]", p.idx, p.module, p.name);
        }
    }
    // 호출부 프로브의 스텁도 같은 인벤토리에 등록된다(tag = 타깃 함수 RVA).
    for c in probe20_tbl::CALLSITES20.iter() {
        if c.target_rva == rva {
            return format!("({} / {}/{}) [호출부]", c.idx, c.module, c.name);
        }
    }
    String::new()
}

// ───────────────────────── 경로(동적 도출) ─────────────────────────
/// 이 **dll 자신의** 폴더 = `<게임설치>\mods\tfm2_judge_verify`.
/// 주소→모듈 핸들(`GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS|UNCHANGED_REFCOUNT`)로 구하므로
/// install 경로 하드코딩이 전혀 없다(CLAUDE.md §2 함정 ③).
fn dir() -> Option<PathBuf> {
    unsafe {
        let mut h: HMODULE = 0;
        if GetModuleHandleExW(0x4 | 0x2, dir as *const () as *const u16, &mut h) == 0 || h == 0 {
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

fn pth(name: &str) -> Option<PathBuf> {
    dir().map(|mut p| {
        p.push(name);
        p
    })
}

// ───────────────────────── 설치 ─────────────────────────
/// ★cfg(`tune()`)는 **가져오지 않았다.** 이유:
///   ① ai_adjust 의 `tune()` 은 TuneMap 게시/2세대 지연free/선수·클래스 오버라이드/`neutral_knobs`
///      까지 딸린 수백 줄 인프라이고, 의미도 「AI 조정 노브」다 — 이 모드엔 조정할 노브가 없다.
///   ② 게이트를 두면 **cfg 부재 → 0 으로 읽힘 → 발화 0** 이라는 오진 경로가 생긴다.
///      「발화 0 = 검증 표본 불성립」이 1단계의 판정 문장이므로, 그 값이 설치 실패로 오염되면 안 된다.
///   ⟹ 대신 **fail-safe 한 opt-out 하나만** 둔다: `probe20_off.txt` 가 있으면 설치하지 않는다
///      (파일 없음 = 설치 = 기본 동작. 파싱 없음 = 파서 실패 경로 없음).
fn probe_disabled() -> bool {
    pth("probe20_off.txt").map(|p| p.exists()).unwrap_or(false)
}

/// ★2단계 게이트 = `sweep20_on.txt` 의 비트마스크. 반환 = (mask, 사람이 읽을 사유 문장).
/// **파일 없음 = 0 = 기본 OFF.** 1단계의 opt-out(`probe20_off.txt`)과 방향이 반대인 이유:
///   1단계는 안전한 계측(레지스터 무변경)이라 켜 두는 게 기본이지만, sweep 은 **게임 함수를 한 번 더
///   실행**하고 ABI 가 틀리면 즉사한다(선례: ai_adjust `fn_bisect` 비트2 = 게임 즉사).
///   ⟹ **명시적으로 적어야만 켠다.** 파싱 실패도 0 으로 떨어진다(fail-safe).
/// 형식 = 한 줄에 `0x3` 또는 `3`. `#` 로 시작하는 줄과 빈 줄은 무시.
fn sweep_mask() -> (u64, String) {
    let p = match pth("sweep20_on.txt") {
        Some(p) => p,
        None => return (0, "경로 도출 실패 → OFF".into()),
    };
    let txt = match fs::read_to_string(&p) {
        Ok(t) => t,
        Err(_) => return (0, "sweep20_on.txt 없음 → **OFF**(기본값)".into()),
    };
    for ln in txt.lines() {
        let t = ln.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let (body, radix) = if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
            (h, 16)
        } else {
            (t, 10)
        };
        return match u64::from_str_radix(body, radix) {
            Ok(v) => (v, format!("sweep20_on.txt = \"{}\" → mask {:#x}", t, v)),
            Err(_) => (0, format!("sweep20_on.txt 파싱 실패(\"{}\") → **OFF**", t)),
        };
    }
    (0, "sweep20_on.txt 가 비어 있다 → **OFF**".into())
}

/// ★3단계 게이트 = `abiprobe_on.txt`. 반환 = (mask, 관측상한 N, 사람이 읽을 사유).
/// **파일 없음 = 미설치**(유저 지시). 파일만 있으면 **전 슬롯**(마스크 줄을 안 적어도 켜진다) —
/// 2단계와 달리 「켜 놓고 마스크를 깜빡해서 0건 관측」이라는 허탕을 만들지 않는다.
/// 형식(둘 다 선택): 비트마스크 한 줄 `0x3`/`3` · 관측 상한 `n=16`. `#` 주석·빈 줄 무시.
/// ⚠abiprobe 는 sweep 과 **동시 사용 금지**다(같은 진입부 12B) — 실제 배타는 `abiprobe::install`
///   이 sweep 마스크를 받아 강제한다. 여기서는 게이트 값만 읽는다.
fn abiprobe_gate() -> (u64, u32, String) {
    // ★슬롯 수를 하드코딩하지 않는다(표가 늘면 마스크 기본값이 조용히 낡는다).
    let all: u64 = (1u64 << abiprobe::S.len()) - 1;
    const DEF_N: u32 = 8;
    let p = match pth("abiprobe_on.txt") {
        Some(p) => p,
        None => return (0, DEF_N, "경로 도출 실패 → 미설치".into()),
    };
    let txt = match fs::read_to_string(&p) {
        Ok(t) => t,
        Err(_) => {
            return (
                0,
                DEF_N,
                "abiprobe_on.txt 없음 → **미설치**(기본값. 이 파일을 만들면 켜진다)".into(),
            )
        }
    };
    let mut mask: Option<u64> = None;
    let mut n = DEF_N;
    let mut notes = String::new();
    for ln in txt.lines() {
        let t = ln.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        if let Some(v) = t
            .strip_prefix("n=")
            .or_else(|| t.strip_prefix("N="))
            .and_then(|v| v.trim().parse::<u32>().ok())
        {
            n = v.clamp(1, 64);
            notes.push_str(&format!(" · n={} → 관측상한 {}", v, n));
            continue;
        }
        if mask.is_none() {
            let (body, radix) = if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
                (h, 16)
            } else {
                (t, 10)
            };
            match u64::from_str_radix(body, radix) {
                Ok(v) => {
                    mask = Some(v);
                    notes.push_str(&format!(" · 마스크 줄 \"{}\" → {:#x}", t, v));
                }
                Err(_) => notes.push_str(&format!(" · ⚠해석 못 한 줄 \"{}\" 무시", t)),
            }
        }
    }
    let m = mask.unwrap_or(all);
    if mask.is_none() {
        notes.push_str(&format!(" · 마스크 줄 없음 → **전 슬롯 {:#x}**", all));
    }
    (m, n, format!("abiprobe_on.txt 있음 → mask {:#x} · N={}{}", m, n, notes))
}

unsafe fn do_install() {
    if let Some(d) = dir() {
        let _ = fs::create_dir_all(&d);
    }
    if probe_disabled() {
        let _ = pth("probe20.txt").map(|p| {
            fs::write(
                p,
                "=== tfm2_judge_verify ===\nprobe20_off.txt 가 있어 프로브를 설치하지 않았다.\n\
                 이 파일을 지우고 게임을 재시작하면 측정이 켜진다.\n",
            )
        });
        return;
    }
    // ★★순서가 load-bearing: **sweep 을 먼저** 설치한다. 같은 진입부에 둘 다 걸 수 없으므로
    //   프로브 쪽이 `sweep20::is_installed_spec()` 를 보고 자기를 건너뛴다(반대 순서면 프로브가
    //   먼저 12B 를 덮어 sweep 이 「이미 훅됨」으로 전부 미설치된다).
    let (mask, gate) = sweep_mask();
    SWEEP_MASK.store(mask, Ordering::Relaxed);
    let mut slog = String::new();
    // ★bit62 = `#02` midpin(pin02.rs) — sweep20 슬롯 비트가 아니라 여기서 떼어 따로 건다.
    //   ⚠09-13 정정: 옛 bit19 는 gensweep20 이 발화수 순으로 **재배정**하는 칸이라 r7 편입 후 #24 와 충돌해
    //   #24 가 「미설치」로 빠졌다(실사고 · 판 08:40). 자동배정 범위 밖 고정 비트로 뺀다.
    let (sok, sn) = sweep20::install(mask & !(1u64 << PIN02_BIT), &mut slog);
    if mask & (1u64 << PIN02_BIT) != 0 {
        let _ = pin02::install(&mut slog);
    }
    SWEEP_OK.store(sok, Ordering::Relaxed);
    SWEEP_N.store(sn, Ordering::Relaxed);
    *SWEEP_GATE.lock().unwrap_or_else(|e| e.into_inner()) = gate;
    *SWEEP_LOG.lock().unwrap_or_else(|e| e.into_inner()) = slog;

    // ★★3단계 abiprobe = **sweep 다음 · probe 앞**. 우선순위 sweep > abiprobe > probe 를
    //   설치 순서로 강제한다(뒤에 오는 쪽이 앞선 쪽을 보고 양보한다).
    //   ⛔sweep 게이트가 같은 함수를 요청했으면 abiprobe 는 **설치 성공 여부와 무관하게** 건너뛴다
    //     (`mask` 를 그대로 넘겨 게이트 레벨에서 판정 — abiprobe.rs 헤더 ㉡).
    let (amask, anobs, agate) = abiprobe_gate();
    ABI_MASK.store(amask, Ordering::Relaxed);
    ABI_NOBS.store(anobs, Ordering::Relaxed);
    let mut alog = String::new();
    let (aok, an) = abiprobe::install(amask, anobs, mask, &mut alog);
    ABI_OK.store(aok, Ordering::Relaxed);
    ABI_N.store(an, Ordering::Relaxed);
    *ABI_GATE.lock().unwrap_or_else(|e| e.into_inner()) = agate;
    *ABI_LOG.lock().unwrap_or_else(|e| e.into_inner()) = alog;

    let (ok, n) = probe::install_all();
    // ★호출부 프로브(#13) — 진입부 프로브 **뒤에** 설치한다. 이유 = #13 의 사이트 하나가
    //   `sub_plan`(0xcaf9f0 = 진입부 프로브 #2) **본문 +0x237** 이라, 순서를 이렇게 두면
    //   「진입부 12B 검증 → 패치」가 끝난 뒤에 본문을 만지므로 두 패치의 관찰 구간이 섞이지 않는다.
    //   (구간 자체가 겹치지 않으니 어느 순서든 안전하지만, 로그를 읽을 때 원인 귀속이 쉬워진다.)
    let (cok, cn) = probe::install_callsites();
    INSTALL_OK.store(ok + cok, Ordering::Relaxed);
    INSTALL_N.store(n + cn, Ordering::Relaxed);
    INSTALLED.store(true, Ordering::Relaxed);
    stub_dump();
    probe::mark_base();
    dump("설치 직후 스냅샷(아직 판이 돌지 않았다)");
}

// ───────────────────────── 덤프 ─────────────────────────
unsafe fn dump(ctx: &str) {
    // ★세 계측의 설치 상태를 **한 헤더에** 싣는다 — 파일은 셋으로 나뉘어 있어도
    //   「지금 어느 계측이 그 진입부를 잡고 있나」는 어느 파일을 펴도 즉시 보여야 한다(배타 확인용).
    let header = format!(
        "덤프: {}\n프레임 {} · 경과 {:.1}s · 관측한 판 종료 {}회\n\
         계측 설치: probe(1단계) {}/{} · sweep(2단계) {}/{} mask {:#x} · abiprobe(3단계) {}/{} mask {:#x} N={}\nexe_base={:#x}\n소스: {}",
        ctx,
        FRAME.load(Ordering::Relaxed),
        ACC_MS.load(Ordering::Relaxed) as f64 / 1000.0,
        MATCH_SEQ.load(Ordering::Relaxed),
        INSTALL_OK.load(Ordering::Relaxed),
        INSTALL_N.load(Ordering::Relaxed),
        SWEEP_OK.load(Ordering::Relaxed),
        SWEEP_N.load(Ordering::Relaxed),
        SWEEP_MASK.load(Ordering::Relaxed),
        ABI_OK.load(Ordering::Relaxed),
        ABI_N.load(Ordering::Relaxed),
        ABI_MASK.load(Ordering::Relaxed),
        ABI_NOBS.load(Ordering::Relaxed),
        exe_base(),
        file!()
    );
    let s = probe::report(&header);
    if let Some(p) = pth("probe20.txt") {
        let _ = fs::write(p, s);
    }
    // ★2단계는 **별도 파일**(섞으면 판정이 흐려진다 — 유저 지시). 게이트 OFF 여도 한 번은 쓴다:
    //   「왜 대조가 0 건인가」(게이트 OFF · 설치 실패 · 발화 0)를 파일이 스스로 말해야 한다.
    if let Some(p) = pth("sweep20.txt") {
        let gate = SWEEP_GATE.lock().unwrap_or_else(|e| e.into_inner()).clone();
        let inst = format!(
            "성공 {}/{} (mask {:#x})\n{}",
            SWEEP_OK.load(Ordering::Relaxed),
            SWEEP_N.load(Ordering::Relaxed),
            SWEEP_MASK.load(Ordering::Relaxed),
            SWEEP_LOG.lock().unwrap_or_else(|e| e.into_inner()).as_str()
        );
        let mut rep = sweep20::report(&header, &gate, &inst);
        if SWEEP_MASK.load(Ordering::Relaxed) & (1u64 << PIN02_BIT) != 0 { rep.push_str(&pin02::report()); }
        let _ = fs::write(p, rep);
    }
    // ★3단계도 **별도 파일**. 게이트 OFF 여도 한 번은 쓴다(「왜 관측이 0 건인가」를 파일이 말하게).
    if let Some(p) = pth("abiprobe.txt") {
        let gate = ABI_GATE.lock().unwrap_or_else(|e| e.into_inner()).clone();
        let inst = format!(
            "    성공 {}/{} (mask {:#x} · 관측상한 N={}/슬롯)\n{}",
            ABI_OK.load(Ordering::Relaxed),
            ABI_N.load(Ordering::Relaxed),
            ABI_MASK.load(Ordering::Relaxed),
            ABI_NOBS.load(Ordering::Relaxed),
            ABI_LOG.lock().unwrap_or_else(|e| e.into_inner()).as_str()
        );
        let _ = fs::write(p, abiprobe::report(&header, &gate, &inst));
    }
}

// ───────────────────────── 프레임 틱 ─────────────────────────
fn tick(dt: f32) {
    let f = FRAME.fetch_add(1, Ordering::Relaxed) + 1;
    let ms = ACC_MS.fetch_add((dt.max(0.0) * 1000.0) as u64, Ordering::Relaxed);

    // ⓐ 설치 = 게임 로드 후 한 번(늦게 — 위 INSTALL_AT_FRAME 주석 참조)
    if !INSTALLED.load(Ordering::Relaxed) {
        if f >= INSTALL_AT_FRAME && !INSTALL_TRIED.swap(true, Ordering::Relaxed) {
            unsafe { do_install() };
        }
        return;
    }

    // ⓑ 폴링: 카운터 총합의 변화로 판 진행/종료를 판정한다.
    //    ★게임 API(Scene/db)에 의존하지 않는 이유 = 우리가 재려는 것이 바로 그 카운터이고,
    //      배경 sim(rayon 워커)에서 돌아가는 판까지 이 방식이면 똑같이 잡힌다.
    if ms.saturating_sub(LAST_POLL_MS.load(Ordering::Relaxed)) < (POLL_SEC * 1000.0) as u64 {
        return;
    }
    LAST_POLL_MS.store(ms, Ordering::Relaxed);

    // ★sweep 이 대체한 함수는 프로브 카운터가 0 이므로, sweep 의 호출수도 활동 판정에 더한다
    //   (안 더하면 sweep 만 켠 판에서 「조용하다」로 오판해 판 종료 확정치를 못 남긴다).
    // ⚠abiprobe 의 `seen` 은 관측상한까지만 는다(그 뒤엔 DISPATCH=FAST 로 콜백을 안 부른다) ⟹
    //   활동 판정의 **주 신호로 쓸 수 없다**. 단조이므로 더해도 무해하고, abiprobe 가 잡은 함수도
    //   판 초반엔 활동으로 잡히게 해 준다(주 신호는 여전히 probe/sweep 카운터다).
    let cur = unsafe { probe::total() }
        .wrapping_add(sweep20::S.iter().map(|s| s.calls.load(Ordering::Relaxed)).sum::<u64>())
        .wrapping_add(abiprobe::total_seen());
    let prev = LAST_TOTAL.load(Ordering::Relaxed);
    if cur != prev {
        // 판이 돌고 있다 → 중간 스냅샷(크래시로 판이 안 끝나도 수치가 남게).
        LAST_TOTAL.store(cur, Ordering::Relaxed);
        ACTIVE.store(true, Ordering::Relaxed);
        IDLE_POLLS.store(0, Ordering::Relaxed);
        unsafe { dump("중간 스냅샷(판 진행 중 — 아직 판정 근거 아님)") };
        return;
    }
    if !ACTIVE.load(Ordering::Relaxed) {
        return; // 애초에 조용했다 — 아무 일도 안 한다
    }
    let idle = IDLE_POLLS.fetch_add(1, Ordering::Relaxed) + 1;
    if idle < IDLE_POLLS_TO_END {
        return;
    }
    // ★판 종료 확정 — 프로젝트 규율 = 「판 종료 수치로만 판정」(tfm2-judge-layer).
    ACTIVE.store(false, Ordering::Relaxed);
    IDLE_POLLS.store(0, Ordering::Relaxed);
    let seq = MATCH_SEQ.fetch_add(1, Ordering::Relaxed) + 1;
    unsafe {
        dump(&format!(
            "★판 종료 #{} 확정치 ({:.0}s 무변화 = 판이 끝났다) — **판정은 이 수치로만**",
            seq,
            POLL_SEC * IDLE_POLLS_TO_END as f32
        ));
        probe::mark_base(); // 다음 판의 델타 기준선
        sweep20::mark_base();
    }
}

// ───────────────────────── 모드 등록 ─────────────────────────
// 생명주기 훅 = `post_update` 하나만 쓴다(표본 = tfm2_mod_order\src\lib.rs:601, ModExtension).
//   설치를 `init()`(= ai_adjust 가 쓰는 자리)에 두지 않은 것이 핵심 판단이다 — 위 INSTALL_AT_FRAME 주석.
struct JudgeVerifyExt;
impl ModExtension for JudgeVerifyExt {
    fn post_update(&self, _scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, dt: f32) {
        // detour/패치 본문의 패닉이 게임 콜스택을 unwind 하면 UB (CLAUDE.md §3).
        let _ = catch_unwind(AssertUnwindSafe(|| tick(dt)));
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(JudgeVerifyExt);
    reg
}

declare_mod!(init);
