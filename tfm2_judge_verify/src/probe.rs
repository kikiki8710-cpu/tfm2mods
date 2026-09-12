//! probe — **발화수(호출 횟수) 전용 프로브**. 1단계의 전부.
//! ===========================================================================
//! 이관 원본 = `tfm2_ai_adjust\src\judge\probe.rs` (표 타입 `Probe`→`P20`, 실패사유 집계 추가).
//!
//! 왜 1단계가 발화수인가: 대조(sweep)를 아무리 정교하게 짜도 **그 함수가 안 뜨면 표본이 0**이다.
//!   프로젝트 규율 = 「DIFF=0 은 표본 수와 함께」·「발화 0 = 검증 표본 불성립」.
//!   선례 = 명세 `07` 은 포팅까지 끝내 놓고 **발화 0 = 사장** 판정이 났다. 그 판정을 먼저 내리는 것이 1단계.
//!
//! 안전성(= 인자·반환형을 몰라도 되는 이유): 스텁이 **레지스터·스택을 일절 건드리지 않는다.**
//!   stub+0        : u64 카운터
//!   stub+8        : F0 48 FF 05 F0FFFFFF   lock inc qword [rip-16]  ← 오직 EFLAGS 만 변경(x64 ABI 상 비보존)
//!   stub+16       : 원본 프롤로그 len 바이트
//!   stub+16+len   : FF 25 00000000 / dq (fn+len)   원본 len 바이트 뒤로 복귀
//!   진입부는 `48 b8 <stub+8> ff e0` 12B(나머지 NOP) 로 교체.
//! ⚠ rax 를 쓰지 않는다 — 진입부의 movabs 가 rax 를 깨지만 그건 함수 첫 명령 **이전**이라 무해하고,
//!   스텁 본문이 rax 를 안 건드리므로 프롤로그가 rax 를 세팅하는 함수도 안전하다.
//! ⚠ 프롤로그를 **옮겨 실행**하므로 그 안에 분기/call/rip-상대가 있으면 안 된다 —
//!   `MIG\probe20.py` 가 capstone(`aiprobe.prolog_of`)으로 판정해 통과분만 `PROBES20` 에 넣었고,
//!   탈락분은 `MISSING20` 에 사유와 함께 남는다(그래서 표를 손으로 고치면 안 된다).
//!
//! ★★프로브 종류 ②= **호출부 프로브**(`CALLSITES20`, 2026-09-12 추가 · 1호 적용 = #13)
//!   진입부 12B 를 못 빼는 함수(#13 `target_bush_v30` = `+9` 의 `je rel32` 가 잘린다)를 위한 방식.
//!   **원 함수의 명령을 하나도 건드리지 않는다** — `call rel32`(5B)의 **rel32 4바이트만** 고쳐
//!   호출을 스텁으로 우회시키고, 스텁이 세고 나서 원 함수로 절대점프한다.
//!     stub+0  : u64 카운터
//!     stub+8  : F0 48 FF 05 F0FFFFFF   lock inc qword [rip-16]   ← EFLAGS 만 변경
//!     stub+16 : FF 25 00000000 / dq (mbase+target_rva)           원 함수로 절대점프
//!   호출부 5B 는 `E8 <(stub+8) − (site+5)>` 로 재작성. `call` 이 스택에 push 한 리턴주소는
//!   그대로이므로 원 함수는 **원래 호출자에게** 리턴한다. 레지스터·스택·인자 전부 무변.
//!   ⚠제약 ㉠ `E8 rel32` 는 **±2GB** — 스텁을 exe 근처에 잡아야 한다(`alloc_near`).
//!      **근접 검산 실패 시 설치하지 않는다**(범위 밖 rel32 = 엉뚱한 주소로 점프 = 즉사).
//!   ⚠제약 ㉡ 호출부를 **전수** 덮어야 발화수가 맞다 — `MIG\probe20.py` 가 `.text` 전역 스캔 +
//!      절대주소 8B 리터럴 검사(간접호출 유무)로 전수성을 검산하고 통과분만 표에 넣는다.
//!   ⚠#13 의 사이트 `0xcafc27` 은 `tfm2_ai_adjust` 가 **진입부를 체인 후킹하는 함수**
//!      `sub_plan`(0xcaf9f0) **안**이다(= 그 함수 +0x237). 충돌하지 않는 근거:
//!      ①ai_adjust 도, 우리 진입부 프로브도 건드리는 것은 **진입부 +0~13** 뿐이고
//!      ②우리가 쓰는 바이트는 **+0x237..+0x23c** 다 ⟹ 구간이 겹치지 않는다.
//!      ③진입부 훅은 프롤로그를 옮겨 실행한 뒤 `fn+len` 으로 복귀하므로 본문 +0x237 은 **그대로 실행**된다.
//!
//! ★★진입부 12B 를 패치하는 계측은 **셋**이다(probe/sweep/abiprobe) — 전부 공존 불가.
//!   우선순위 = **sweep > abiprobe > probe** 이고, 이 파일은 가장 늦게 설치되면서 앞선 둘을 보고
//!   양보한다(`SWEPT` + `F_SWEEP`/`F_ABI`). 배타의 3중 강제 = `abiprobe.rs` 헤더 ㉠㉡㉢.
//!
//! ★2단계(sweep = game==mine 비트동일 대조)에는 `extern crate game_ai;` 가 필요하다
//!   (= 640함수 링크사본 = 재현 정본). 1단계에 넣지 않은 이유 = dll 이 수 MB 로 불어
//!   `build_inj.ps1` 의 사이즈가드를 넘기고, 1단계는 「발화수」만 보면 되기 때문이다.
#![allow(dead_code)]
use crate::probe20_tbl::{C20, CALLSITES20, P20, PROBES20};
use crate::{
    exe_base, readable, stub_reg, FlushInstructionCache, GetCurrentProcess, VirtualAlloc,
    VirtualFree, VirtualProtect,
};

/// 슬롯 상한. ★표 길이에 의존하지 않는다 — `#5/#13/#17` RVA 가 확정돼 표가 20행으로
/// 재생성돼도 코드는 그대로 `PROBES20.len()` 으로 돈다. 여유를 크게 둔다.
pub const CAP: usize = 128; // ★09-13 64→128: r7 잎 20 편입으로 진입부 35 가 CS_SLOT0(32)를 넘쳐 3개(i=37·38·AUX20) 미설치 — probe20.txt 경고로 적발

static mut SLOTS: [usize; CAP] = [0; CAP]; // 프로브 i 의 스텁 주소(= 카운터 주소). 0 = 미설치
static mut FAILS: [&'static str; CAP] = [""; CAP]; // 미설치 사유
static mut BASE: [u64; CAP] = [0; CAP]; // 직전 「판 종료」 시점 누적치(판별 델타 계산용)
static mut NPROBE: usize = 0;
/// ★★**sweep(2단계)이 대신 잡은** 프로브 슬롯. 둘 다 진입부 12B 를 패치하므로 **공존 불가**다 —
///   sweep 이 설치에 성공한 함수는 여기 표시하고 프로브를 **걸지 않는다**(실패가 아니다).
///   ⟹ 리포트에서 「[sweep] 로 대체」 목록으로 따로 나가고, 설치 실패 집계에 섞이지 않는다.
static mut SWEPT: [bool; CAP] = [false; CAP];
/// ★진입부 표가 `CS_SLOT0` 을 넘쳐 **측정에서 빠진 개수**. 0 이 아니면 리포트가 크게 경고한다.
///   (조용한 절단을 만들지 않기 위한 칸 — 넘친 사실 자체가 결함이다.)
static mut OVERFLOW: usize = 0;

// ── 호출부 프로브 = **같은 배열을 공유**한다(한 표로 읽히게) ──
//   슬롯 인덱스 = `CS_SLOT0 + j`. 진입부는 `0..NPROBE` 를 쓴다.
//   ★고정 오프셋인 이유: `PROBES20.len()` 은 static 이라 const 문맥에서 못 읽는다
//     (constants cannot refer to statics) ⟹ 상수로 칸을 나누고 install 시 상한을 검사한다.
pub const CS_SLOT0: usize = 96; // 진입부 프로브는 96개까지(09-13 32→96 · 현재 35 · 서브트리 107 대비)
pub const CS_CAP: usize = 8; // 호출부 프로브 함수 수 상한(현재 1)
pub const MAXSITE: usize = 8; // 함수당 호출부 수 상한(현재 2)

static mut NCS: usize = 0;
/// 사이트별 결과. `""` = 설치 OK · 그 외 = 사유(집계·리포트용). `F_NOTRIED` = 아직 시도 안 함.
static mut CS_WHY: [[&'static str; MAXSITE]; CS_CAP] = [[F_NOTRIED; MAXSITE]; CS_CAP];
/// 사이트별로 써 넣은 새 rel32(리포트에 그대로 찍어 검산 가능하게).
static mut CS_REL: [[i64; MAXSITE]; CS_CAP] = [[0; MAXSITE]; CS_CAP];

// ── 실패 사유 문자열(집계 키로 쓰므로 상수로 고정) ──
const F_LEN: &str = "len 범위 밖(표 이상)";
const F_UNREAD: &str = "읽기불가(RVA 가 코드가 아님)";
const F_HOOKED: &str = "이미 훅됨(다른 모드가 선점)";
const F_PROLOG: &str = "프롤로그 불일치(RVA 가 틀렸다)";
const F_ALLOC: &str = "VirtualAlloc 실패";
const F_PROT: &str = "VirtualProtect 실패";
// ── 호출부 프로브 전용 사유 ──
const F_NOTRIED: &str = "미시도";
const F_NOCALL: &str = "호출부가 E8 아님(누가 이미 고쳤다)";
const F_CTGT: &str = "call 목표 불일치(RVA 가 틀렸다)";
const F_NEAR: &str = "근접 할당 실패(±2GB 밖) — 설치 안 함";
const F_SITES: &str = "사이트 수 이상(표 이상)";
const F_NOSITE: &str = "쓸 수 있는 사이트 0곳";
/// ★실패가 아니다 — 2단계 sweep 이 같은 진입부를 잡았다(그쪽이 발화수까지 같이 센다).
const F_SWEEP: &str = "sweep(2단계)이 대체 — 발화수는 sweep20.txt 의 호출수 열";
/// ★실패가 아니다 — 3단계 abiprobe 가 같은 진입부를 잡았다(관측 전용 · 발화수는 안 센다).
///   ⚠sweep 대체와 달리 **발화수 대체물이 없다**(abiprobe 는 N회 관측 후 손을 뗀다) ⟹
///   그 함수의 이번 판 발화수는 **측정 안 됨**으로 읽어야 한다(0 이 아니다).
const F_ABI: &str = "abiprobe(3단계)가 대체 — ★발화수 측정 안 됨(관측 전용 · abiprobe.txt 참조)";
const REASONS: [&str; 11] = [
    F_HOOKED, F_PROLOG, F_UNREAD, F_LEN, F_ALLOC, F_PROT, F_NOCALL, F_CTGT, F_NEAR, F_SITES,
    F_NOSITE,
];

/// 프로브 설치. 반환 = (설치 성공 수, 시도 수).
/// ⚠ 실패해도 **그 함수만 건너뛰고 계속**한다. 사유는 `FAILS` 에 남아 `report()` 가 찍는다
///   (조용히 넘기면 「RVA 가 틀렸다」를 아무도 모르게 된다 — 1단계에서 가장 비싼 실수).
pub unsafe fn install_all() -> (usize, usize) {
    let mbase = exe_base();
    if mbase == 0 {
        return (0, 0);
    }
    // ★★진입부 슬롯 상한은 `CAP`(64) 이 아니라 **`CS_SLOT0`(32)** 이다 —
    //   32 부터는 호출부 프로브가 쓰는 칸이라 넘치면 **카운터가 조용히 겹친다.**
    //   ⚠초판이 `.min(CAP)` 이었다. 위 주석이 「install 시 상한을 검사한다」고 적어 놨는데
    //     실제 검사는 64 였다 — **「규칙을 적는 것과 기계가 강제하는 것은 다르다」**(11차 교훈).
    //   ⚠그리고 넘친 것을 **조용히 잘라서도** 안 된다. `mkspec3` 의 `rs[:3]` 절단이
    //     「임의의 세 개에 정본 도장」을 찍던 것과 같은 형태다(12차에 고쳤다) ⟹ 사유를 남긴다.
    let n = PROBES20.len().min(CS_SLOT0);
    if PROBES20.len() > CS_SLOT0 {
        OVERFLOW = PROBES20.len() - CS_SLOT0;
    }
    let mut ok = 0usize;
    let mut swept = 0usize;
    for (i, p) in PROBES20.iter().take(n).enumerate() {
        // ★★sweep(2단계)·abiprobe(3단계)가 이미 그 진입부를 잡았으면 **건너뛴다**(셋 다 진입부 12B
        //   패치 = 공존 불가 — 위 SWEPT 주석). ⚠순서가 load-bearing 이다: `lib.rs::do_install` 이
        //   **sweep → abiprobe → probe** 순으로 설치하므로, 가장 늦은 probe 가 둘을 다 보고 양보한다.
        //   (반대 순서면 probe 가 먼저 12B 를 덮어 둘 다 「이미 훅됨」으로 전부 미설치된다.)
        if crate::sweep20::is_installed_spec(p.idx) || crate::abiprobe::is_installed_spec(p.idx) {
            SLOTS[i] = 0;
            FAILS[i] = if crate::sweep20::is_installed_spec(p.idx) { F_SWEEP } else { F_ABI };
            SWEPT[i] = true;
            swept += 1;
            continue;
        }
        match install_one(mbase, p) {
            Ok(stub) => {
                SLOTS[i] = stub;
                FAILS[i] = "";
                ok += 1;
            }
            Err(e) => {
                SLOTS[i] = 0;
                FAILS[i] = e;
            }
        }
    }
    NPROBE = n;
    (ok, n - swept)
}

unsafe fn install_one(mbase: usize, p: &P20) -> Result<usize, &'static str> {
    let len = p.len as usize;
    if len < 12 || len > 24 || len > p.prolog.len() {
        return Err(F_LEN);
    }
    let fn_addr = mbase.wrapping_add(p.rva);
    if fn_addr < 0x10000 || fn_addr >= (1usize << 48) {
        return Err(F_UNREAD);
    }
    if !readable(fn_addr, len + 4) {
        return Err(F_UNREAD);
    }
    // ★이미 누가 훅한 함수는 **절대** 건드리지 않는다. 체인을 만들 이유도 없다
    //   (이미 훅됐다 = 누군가 그 함수를 쓰고 있다 = 사실상 "뜬다"가 확인된 함수).
    //   07-18 에 두 모드가 서로를 재체인해 게임이 먹통이 된 실사고가 있다(CLAUDE.md §3).
    if *(fn_addr as *const u8) == 0x48 && *((fn_addr + 1) as *const u8) == 0xb8 {
        return Err(F_HOOKED);
    }
    for i in 0..len {
        if *((fn_addr + i) as *const u8) != p.prolog[i] {
            return Err(F_PROLOG);
        }
    }
    const MEM_CR: u32 = 0x1000 | 0x2000; // MEM_COMMIT|MEM_RESERVE
    const RWX: u32 = 0x40; // PAGE_EXECUTE_READWRITE
    let stub = stub_reg(VirtualAlloc(0, 128, MEM_CR, RWX), 128, p.rva);
    if stub == 0 {
        return Err(F_ALLOC);
    }
    let mut s: Vec<u8> = Vec::with_capacity(64);
    s.extend_from_slice(&0u64.to_le_bytes()); // +0  카운터
    s.extend_from_slice(&[0xf0, 0x48, 0xff, 0x05, 0xf0, 0xff, 0xff, 0xff]); // +8  lock inc qword [rip-16]
    s.extend_from_slice(&p.prolog[..len]); // +16 원본 프롤로그
    s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]); //     jmp [rip+0]
    s.extend_from_slice(&(fn_addr + len).to_le_bytes()); //     dq fn+len
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());

    let entry = stub + 8; // 코드 시작(카운터 8B 뒤)
    let mut patch = vec![0x90u8; len];
    patch[0] = 0x48;
    patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&entry.to_le_bytes());
    patch[10] = 0xff;
    patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, len, RWX, &mut old) == 0 {
        return Err(F_PROT);
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, len);
    VirtualProtect(fn_addr, len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, len);
    Ok(stub)
}

// ─────────────────── 호출부 프로브(진입부를 안 건드리는 방식) ───────────────────

const MEM_CR: u32 = 0x1000 | 0x2000; // MEM_COMMIT|MEM_RESERVE
const RWX: u32 = 0x40; // PAGE_EXECUTE_READWRITE
const MEM_RELEASE: u32 = 0x8000;
/// `E8 rel32` 사거리. i32 상한(0x7FFFFFFF)에서 여유를 크게 뺀 값 —
/// 스텁 한 칸(128B) 과 명령 길이 따위로 경계에 걸리는 일을 아예 없앤다.
const REACH: i64 = 0x7FFF_0000;

/// ★근접 할당 — `E8 rel32` 가 닿는 곳에 RWX 128B 를 잡는다.
/// `VirtualAlloc(0, ..)` 은 임의 주소를 주므로 **힌트를 주고 64KB(할당 granularity) 단위로 스캔**한다.
/// exe 베이스 −128MB 에서 시작해 위로 훑으며(총 256MB = 4096칸) 첫 성공 주소를 쓴다.
/// ⛔성공한 주소라도 **모든 사이트와의 rel32 가 REACH 안인지 검산**하고, 실패하면
///   그 블록을 해제하고 계속 찾는다. 끝까지 못 찾으면 **0 을 반환 = 설치하지 않는다**
///   (범위 밖 rel32 를 쓰면 엉뚱한 주소로 점프 = 즉시 크래시).
pub(crate) unsafe fn alloc_near(sites_abs: &[usize], size: usize, tag: usize) -> usize {
    let mbase = exe_base();
    if mbase == 0 || sites_abs.is_empty() {
        return 0;
    }
    const GRAN: usize = 0x10000;
    let start = (mbase.saturating_sub(0x0800_0000) & !(GRAN - 1)).max(GRAN);
    for k in 0..4096usize {
        let hint = start.wrapping_add(k * GRAN);
        let p = VirtualAlloc(hint, size, MEM_CR, RWX);
        if p == 0 {
            continue; // 그 주소는 이미 누가 쓴다(exe 이미지·힙 등) — 다음 칸
        }
        // ★검산: 스텁 코드 시작(p+8)까지의 rel32 가 전 사이트에서 사거리 안인가
        let mut ok = true;
        for &s in sites_abs.iter() {
            let d = (p as i64 + 8) - (s as i64 + 5);
            if d > REACH || d < -REACH {
                ok = false;
                break;
            }
        }
        if ok {
            return stub_reg(p, size, tag);
        }
        VirtualFree(p, 0, MEM_RELEASE); // 검산 실패분은 즉시 반납(누수 금지)
    }
    0
}

/// 호출부 프로브 설치. 반환 = (설치 성공 함수 수, 시도 함수 수).
/// 진입부 프로브와 **카운터 배열(`SLOTS`/`BASE`/`FAILS`)을 공유**한다 — 리포트가 한 표다.
pub unsafe fn install_callsites() -> (usize, usize) {
    let mbase = exe_base();
    if mbase == 0 {
        return (0, 0);
    }
    let n = CALLSITES20.len().min(CS_CAP);
    let mut ok = 0usize;
    for (j, c) in CALLSITES20.iter().take(n).enumerate() {
        let slot = CS_SLOT0 + j;
        if slot >= CAP {
            continue;
        }
        match install_one_cs(mbase, j, c) {
            Ok(stub) => {
                SLOTS[slot] = stub;
                FAILS[slot] = "";
                ok += 1;
            }
            Err(e) => {
                SLOTS[slot] = 0;
                FAILS[slot] = e;
            }
        }
    }
    NCS = n;
    (ok, n)
}

unsafe fn install_one_cs(mbase: usize, j: usize, c: &C20) -> Result<usize, &'static str> {
    for k in 0..MAXSITE {
        CS_WHY[j][k] = F_NOTRIED;
        CS_REL[j][k] = 0;
    }
    if c.sites.is_empty() || c.sites.len() > MAXSITE {
        return Err(F_SITES);
    }
    let fn_addr = mbase.wrapping_add(c.target_rva);
    if fn_addr < 0x10000 || fn_addr >= (1usize << 48) || !readable(fn_addr, 16) {
        return Err(F_UNREAD);
    }

    // ── ① 사이트 사전검증 — 실제로 `E8 <우리가 아는 rel32>` 인가 ──
    //   ⚠E8 이 아니면 **누가 이미 건드렸다**는 뜻이므로 그 사이트는 스킵한다
    //     (진입부 프로브의 「이미 훅됨(48 b8) 스킵」 규율을 호출부에 맞게 옮긴 것).
    //     call 목표가 우리가 아는 함수와 다르면 RVA 가 틀렸다는 뜻이니 역시 스킵.
    let mut cand: [usize; MAXSITE] = [0; MAXSITE]; // c.sites 의 인덱스
    let mut sabs: [usize; MAXSITE] = [0; MAXSITE]; // 그 사이트의 절대주소
    let mut nc = 0usize;
    for (k, &srva) in c.sites.iter().enumerate() {
        let s = mbase.wrapping_add(srva);
        if s < 0x10000 || !readable(s, 5) {
            CS_WHY[j][k] = F_UNREAD;
            continue;
        }
        if *(s as *const u8) != 0xE8 {
            CS_WHY[j][k] = F_NOCALL;
            continue;
        }
        let rel = core::ptr::read_unaligned((s + 1) as *const i32) as i64;
        if (s as i64).wrapping_add(5).wrapping_add(rel) != fn_addr as i64 {
            CS_WHY[j][k] = F_CTGT;
            continue;
        }
        cand[nc] = k;
        sabs[nc] = s;
        nc += 1;
    }
    if nc == 0 {
        return Err(F_NOSITE);
    }

    // ── ② 근접 스텁 할당(⛔검산 실패 = 설치 안 함. 여기서 반환하면 .text 는 무손상) ──
    let stub = alloc_near(&sabs[..nc], 128, c.target_rva);
    if stub == 0 {
        return Err(F_NEAR);
    }
    let mut s: Vec<u8> = Vec::with_capacity(32);
    s.extend_from_slice(&0u64.to_le_bytes()); // +0  카운터
    s.extend_from_slice(&[0xf0, 0x48, 0xff, 0x05, 0xf0, 0xff, 0xff, 0xff]); // +8  lock inc qword [rip-16]
    s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]); // +16 jmp [rip+0]
    s.extend_from_slice(&fn_addr.to_le_bytes()); // +22 dq 원 함수(진입부 그대로)
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let entry = stub + 8;

    // ── ③ 사이트 패치 — rel32 4바이트만. `E8` 은 그대로 둔다 ──
    let mut done = 0usize;
    for i in 0..nc {
        let k = cand[i];
        let site = sabs[i];
        let nrel = (entry as i64) - (site as i64 + 5);
        if nrel > REACH || nrel < -REACH {
            CS_WHY[j][k] = F_NEAR; // 이중 검산(alloc_near 가 이미 걸렀어야 한다)
            continue;
        }
        let mut old: u32 = 0;
        if VirtualProtect(site, 5, RWX, &mut old) == 0 {
            CS_WHY[j][k] = F_PROT;
            continue;
        }
        core::ptr::write_unaligned((site + 1) as *mut i32, nrel as i32);
        VirtualProtect(site, 5, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), site, 5);
        CS_WHY[j][k] = ""; // 설치 OK
        CS_REL[j][k] = nrel;
        done += 1;
    }
    if done == 0 {
        return Err(F_PROT);
    }
    Ok(stub)
}

// ───────────────────────── 호출부 방식 sweep 설치 ─────────────────────────

/// ★**진입부를 못 빼는 함수를 sweep(대조)까지 끌고 가는 경로.** (2026-09-12 신설)
///
/// 1단계 호출부 프로브(`install_one_cs`)와 **같은 기구**인데, 스텁이 「세고 원본으로 점프」가 아니라
/// **내 래퍼(`wrap`)로 점프**한다는 것만 다르다. 래퍼는 호출부에서 **피호출자 자리에** 앉으므로
/// 인자(레지스터·스택)를 **그대로** 받는다 — 진입부 훅과 달리 프롤로그를 훔칠 필요가 없다.
///
/// ⟹ `#13 target_bush_v30` 처럼 「명령 경계 12B 확보 실패」로 진입부 훅이 불가한 함수가 열린다.
///
/// ★`orig` 에는 **원 함수 주소 그대로**를 넣는다(진입부 훅의 트램폴린과 다르다) — 원 함수의
///   명령을 하나도 건드리지 않았으므로 그냥 부르면 된다.
///
/// ⚠제약은 1단계와 동일: ㉠`E8 rel32` **±2GB** ⟹ 스텁을 exe 근처에(`alloc_near`) · 검산 실패 시
///   **설치하지 않는다** ㉡사이트가 `E8` 이 아니거나 목표가 다르면 **그 사이트는 건너뛴다**
///   ㉢한 사이트라도 빠지면 그 경로 호출은 **대조되지 않는다**(표본이 하한선이 된다).
pub unsafe fn install_cs_wrap(
    target_rva: usize,
    sites: &[usize],
    wrap: usize,
    orig: &core::sync::atomic::AtomicUsize,
) -> Result<usize, &'static str> {
    use core::sync::atomic::Ordering;
    let mbase = exe_base();
    if mbase == 0 {
        return Err("exe_base 0");
    }
    if sites.is_empty() || sites.len() > MAXSITE {
        return Err(F_SITES);
    }
    let fn_addr = mbase.wrapping_add(target_rva);
    if fn_addr < 0x10000 || !readable(fn_addr, 16) {
        return Err(F_UNREAD);
    }
    // ① 사이트 사전검증 — 정말 `E8 <이 함수로 가는 rel32>` 인가
    let mut sabs: [usize; MAXSITE] = [0; MAXSITE];
    let mut nc = 0usize;
    for &srva in sites.iter() {
        let s = mbase.wrapping_add(srva);
        if s < 0x10000 || !readable(s, 5) || *(s as *const u8) != 0xE8 {
            continue;
        }
        let rel = core::ptr::read_unaligned((s + 1) as *const i32) as i64;
        if (s as i64).wrapping_add(5).wrapping_add(rel) != fn_addr as i64 {
            continue;
        }
        sabs[nc] = s;
        nc += 1;
    }
    if nc == 0 {
        return Err(F_NOSITE);
    }
    // ② 근접 스텁 — `jmp [rip+0]` + 절대주소(래퍼는 내 DLL 안이라 rel32 사거리 밖일 수 있다)
    let stub = alloc_near(&sabs[..nc], 64, target_rva);
    if stub == 0 {
        return Err(F_NEAR);
    }
    let mut s: Vec<u8> = Vec::with_capacity(16);
    s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]); // jmp [rip+0]
    s.extend_from_slice(&wrap.to_le_bytes()); // dq 래퍼
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());

    // ★★`orig` 를 **패치 전에** 게시한다 — 사이트가 살아나는 순간 다른 스레드가 래퍼에 들어올 수
    //   있는데 그때 `orig` 가 0 이면 `transmute(0)` 호출 = 즉사다(6.1 절 교훈의 호출부판).
    orig.store(fn_addr, Ordering::SeqCst);

    // ③ 사이트 패치 — rel32 4바이트만
    let mut done = 0usize;
    for i in 0..nc {
        let site = sabs[i];
        let nrel = (stub as i64) - (site as i64 + 5);
        if nrel > REACH || nrel < -REACH {
            continue;
        }
        let mut old: u32 = 0;
        if VirtualProtect(site, 5, RWX, &mut old) == 0 {
            continue;
        }
        core::ptr::write_unaligned((site + 1) as *mut i32, nrel as i32);
        VirtualProtect(site, 5, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), site, 5);
        done += 1;
    }
    if done == 0 {
        orig.store(0, Ordering::SeqCst); // 게시 취소
        return Err(F_PROT);
    }
    Ok(stub)
}

// ───────────────────────── 집계 ─────────────────────────

#[inline]
unsafe fn count_at(i: usize) -> u64 {
    if i >= CAP || SLOTS[i] == 0 {
        0
    } else {
        core::ptr::read_volatile(SLOTS[i] as *const u64)
    }
}

/// 전 프로브(진입부 + 호출부) 누적 호출수의 합. 「판이 돌고 있나」를 이 값의 변화로 판정한다.
pub unsafe fn total() -> u64 {
    let mut t = 0u64;
    for i in 0..NPROBE {
        t = t.wrapping_add(count_at(i));
    }
    for j in 0..NCS {
        t = t.wrapping_add(count_at(CS_SLOT0 + j));
    }
    t
}

/// 현재 누적치를 기준선으로 고정(= 다음 판의 델타 기준). 「판 종료」 덤프 직후에 호출.
pub unsafe fn mark_base() {
    for i in 0..NPROBE {
        BASE[i] = count_at(i);
    }
    for j in 0..NCS {
        BASE[CS_SLOT0 + j] = count_at(CS_SLOT0 + j);
    }
}

/// 사람이 읽는 표. `header` = 덤프 문맥(「판 종료 #3」 등).
pub unsafe fn report(header: &str) -> String {
    let n = NPROBE;
    let ncs = NCS;
    // ★종류(`kind`)를 **마지막 필드**로 붙였다 — 아래 집계들이 쓰는 기존 인덱스가 그대로 살아 있게.
    // (델타, 누적, idx, rva, name, module, ins, kind)
    let mut hit: Vec<(u64, u64, u8, usize, &str, &str, u32, String)> = Vec::new();
    let mut zero: Vec<(u8, usize, &str, &str, u32, String)> = Vec::new();
    let mut miss: Vec<(u8, usize, &str, &str, &'static str, String)> = Vec::new();
    // ★sweep/abiprobe 가 대체한 것 = 실패도 미발화도 아니다. 섞으면 「측정 안 됨」과 구분이 안 된다.
    //   ★★어느 계측이 대체했는지(FAILS[i] = F_SWEEP / F_ABI)를 **같이 싣는다** — 둘의 의미가 다르다:
    //     sweep 대체 = 발화수가 sweep20.txt 에 **있다** / abiprobe 대체 = 발화수는 **어디에도 없다**.
    let mut swept: Vec<(u8, usize, &str, &str, &'static str)> = Vec::new();
    for (i, p) in PROBES20.iter().take(n).enumerate() {
        let kind = || "[진입부]".to_string();
        if SWEPT[i] {
            swept.push((p.idx, p.rva, p.name, p.module, FAILS[i]));
            continue;
        }
        if SLOTS[i] == 0 {
            miss.push((p.idx, p.rva, p.name, p.module, FAILS[i], kind()));
            continue;
        }
        let c = count_at(i);
        if c > 0 {
            hit.push((
                c.saturating_sub(BASE[i]),
                c,
                p.idx,
                p.rva,
                p.name,
                p.module,
                p.ins,
                kind(),
            ));
        } else {
            zero.push((p.idx, p.rva, p.name, p.module, p.ins, kind()));
        }
    }
    // ── 호출부 프로브도 **같은 표**에 싣는다(슬롯 CS_SLOT0+j) ──
    for (j, c20) in CALLSITES20.iter().take(ncs).enumerate() {
        let slot = CS_SLOT0 + j;
        let nsite = c20.sites.len().min(MAXSITE);
        let live = (0..nsite).filter(|&k| CS_WHY[j][k].is_empty()).count();
        let kind = format!("[호출부 {}/{}곳]", live, nsite);
        if SLOTS[slot] == 0 {
            miss.push((c20.idx, c20.target_rva, c20.name, c20.module, FAILS[slot], kind));
            continue;
        }
        let c = count_at(slot);
        if c > 0 {
            hit.push((
                c.saturating_sub(BASE[slot]),
                c,
                c20.idx,
                c20.target_rva,
                c20.name,
                c20.module,
                c20.ins,
                kind,
            ));
        } else {
            zero.push((c20.idx, c20.target_rva, c20.name, c20.module, c20.ins, kind));
        }
    }
    hit.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));

    let live_ins: u32 = hit.iter().map(|h| h.6).sum();
    let dead_ins: u32 = zero.iter().map(|z| z.4).sum();
    let mut s = String::new();
    s.push_str(&format!(
        "=== tfm2_judge_verify 1단계 · 발화수(호출 횟수) 프로브 ===\n{}\n\n",
        header
    ));

    // ── 설치 결과 + 실패 사유별 집계 ──
    let ntry = n - swept.len();
    s.push_str(&format!(
        "--- 설치 결과: 시도 {} (진입부 {} + 호출부 {}) · 성공 {} · 실패 {}  [+ sweep/abiprobe 대체 {}]\n",
        ntry + ncs,
        ntry,
        ncs,
        ntry + ncs - miss.len(),
        miss.len(),
        swept.len()
    ));
    // ★다른 계측이 진입부를 잡아 대체된 함수. 표기와 사유를 그대로 싣는다:
    //   [sweep]    = 발화수가 sweep20.txt 「호출수」 열에 있다(그쪽이 같이 센다)
    //   [abiprobe] = 발화수는 **어디에도 없다**(관측 전용 · N회 뒤 손을 뗀다) ⟹ 「측정 안 됨」
    for (idx, rva, nm, md, why) in swept.iter() {
        s.push_str(&format!(
            "      #{:02} {:#010x} {:<12} {}/{}  ← {}\n",
            idx,
            rva,
            if why.starts_with("abiprobe") { "[abiprobe]" } else { "[sweep]" },
            md,
            nm,
            why
        ));
    }
    let mut any = false;
    for r in REASONS.iter() {
        let c = miss.iter().filter(|m| m.4 == *r).count();
        if c > 0 {
            s.push_str(&format!("      {:<32} {}건\n", r, c));
            any = true;
        }
    }
    if !any {
        s.push_str("      (실패 없음)\n");
    }
    if miss.iter().any(|m| m.4 == F_HOOKED) {
        s.push_str(
            "    ⚠「이미 훅됨」이 있으면 그 함수는 이번 판에서 **측정 자체가 안 됐다**.\n\
             \x20     원인 = 다른 모드(대표적으로 tfm2_ai_adjust)가 같은 진입부를 선점했다.\n\
             \x20     측정을 성립시키려면 그 모드를 먼저 내리고(mods\\<MOD_ID> 폴더 비활성) 다시 한 판 돌려야 한다.\n",
        );
    }
    if miss.iter().any(|m| m.4 == F_PROLOG) {
        s.push_str(
            "    ⚠「프롤로그 불일치」 = 표의 RVA 가 이 exe 와 안 맞는다(패치로 어긋났을 가능성).\n\
             \x20     MIG\\probe20.py 재실행 + MIGRATION §7 확인이 필요하다.\n",
        );
    }
    s.push('\n');

    // ── 발화 ──
    s.push_str(&format!(
        "--- 발화 {}개 ({} 명령) — 호출수 내림차순\n",
        hit.len(),
        live_ins
    ));
    s.push_str("     idx  rva        누적호출      이번판델타   ins  종류         module/name\n");
    for (d, c, idx, rva, nm, md, ins, kd) in hit.iter() {
        s.push_str(&format!(
            "    {:>4}  {:#010x} {:>12} {:>12}  {:>5}  {:<12} {}/{}\n",
            idx, rva, c, d, ins, kd, md, nm
        ));
    }
    s.push('\n');

    // ── 미발화 = 1단계 핵심 산출물 ──
    s.push_str(&format!(
        "--- ★미발화 {}개 ({} 명령) = 이번 판에서 **죽은 코드** — 1단계의 핵심 산출물\n",
        zero.len(),
        dead_ins
    ));
    s.push_str("     (발화 0 = 검증 표본 불성립 ⟹ 2단계 sweep 대상에서 빼거나, 뜨는 판 조건을 먼저 찾아야 한다)\n");
    for (idx, rva, nm, md, ins, kd) in zero.iter() {
        s.push_str(&format!(
            "    {:>4}  {:#010x} {:>12} {:>12}  {:>5}  {:<12} {}/{}\n",
            idx, rva, ".", ".", ins, kd, md, nm
        ));
    }
    if live_ins + dead_ins > 0 {
        s.push_str(&format!(
            "    → 설치분 기준 죽은 비율 {:.0}% (명령 수 가중)\n",
            100.0 * dead_ins as f64 / (live_ins + dead_ins) as f64
        ));
    }
    s.push('\n');

    // ── 미설치(측정 불가) ──
    s.push_str(&format!("--- 미설치 {}개 = 측정 불가(발화 0 과 구분할 것)\n", miss.len()));
    for (idx, rva, nm, md, why, kd) in miss.iter() {
        s.push_str(&format!(
            "    {:>4}  {:#010x}  {:<12} {}/{}  ← {}\n",
            idx, rva, kd, md, nm, why
        ));
    }
    s.push('\n');

    // ── ★호출부 프로브 사이트 내역 ──
    //   「어느 경로로 불렸나」가 정보다(디스패처 경유 vs next_plan 경유). 카운터는 함수 단위로
    //   합산되므로 경로별 분해는 안 되지만, **어느 경로가 계측되고 있는지**는 여기서 확정된다.
    if ncs > 0 {
        s.push_str("--- ★호출부 프로브 사이트 내역 (카운터는 함수당 하나 = 사이트 합산)\n");
        for (j, c20) in CALLSITES20.iter().take(ncs).enumerate() {
            let slot = CS_SLOT0 + j;
            s.push_str(&format!(
                "    #{:02} {}/{} → {:#010x}   스텁 {:#x}   누적 {}\n         사유: {}\n",
                c20.idx,
                c20.module,
                c20.name,
                c20.target_rva,
                SLOTS[slot],
                count_at(slot),
                c20.why
            ));
            for (k, &srva) in c20.sites.iter().take(MAXSITE).enumerate() {
                let why = CS_WHY[j][k];
                if why.is_empty() {
                    s.push_str(&format!(
                        "         site {:#010x}  설치 OK  새 rel32 {:#010x}  {}\n",
                        srva,
                        CS_REL[j][k] as i32 as u32,
                        c20.labels.get(k).copied().unwrap_or("")
                    ));
                } else {
                    s.push_str(&format!(
                        "         site {:#010x}  ⛔스킵 — {}  {}\n",
                        srva,
                        why,
                        c20.labels.get(k).copied().unwrap_or("")
                    ));
                }
            }
        }
        s.push_str(
            "     ⚠사이트가 하나라도 스킵됐으면 그 경로 호출은 **안 세어진다** ⟹ 그 함수의 발화수는\n\
             \x20     하한선으로만 읽어라(「0 이면 죽었다」는 판정에 쓸 수 없다).\n",
        );
        s.push('\n');
    }

    // ── MISSING20: 표에 아예 없는 함수 ──
    s.push_str(&format!(
        "--- MISSING20 {}개 = 표에 아예 없어서 **측정 안 됨**(빠진 것을 모르는 상태를 만들지 않는다)\n",
        crate::probe20_tbl::MISSING20.len()
    ));
    for (idx, nm, why) in crate::probe20_tbl::MISSING20.iter() {
        s.push_str(&format!("    {:>4}  {:<28} ← {}\n", idx, nm, why));
    }
    // ★★슬롯 넘침은 **회계가 맞아도 얼을 못 재는** 상태다 — 회계 줄은 표 길이로 세므로 그래도 맞아 보인다.
    //   ⇒ 조용히 잔리면 「표에 있는다 왜 리포트에 없나」를 아무도 못 묻는다.
    if OVERFLOW > 0 {
        s.push_str(&format!(
            "\n★★★경고: 진입부 표가 슬롯 상한(CS_SLOT0={})을 **{}개 넘쳤다** — 그만큼은 설치조차 안 됐다. \
             `probe.rs` 의 CS_SLOT0/CS_CAP 를 늘리고 재빌드해야 한다. 아래 수치는 **불완전**하다.\n",
            CS_SLOT0, OVERFLOW
        ));
    }
    // ★AUX(명세 밖 보조 계측 · `idx >= 20`)를 **명세 회계에서 뺀다.**
    //   안 빼면 `진입부 18 + 호출부 1 + 표밖 2 = 21 / 20` 처럼 **합이 20 을 넘는다**(2026-09-12 실측).
    //   회계가 안 맞는 줄을 그대로 두면 다음 세션이 「표가 깨졌나」를 먼저 의심하느라 시간을 쓴다.
    let n_aux = PROBES20.iter().filter(|p| p.idx >= 20).count();
    let n_spec_entry = PROBES20.len() - n_aux;
    s.push_str(&format!(
        "\n명세 20함수 회계: 진입부 {} + 호출부 {} + 표밖 {} = {} / 20  \
         (설치 {} · 발화 {} · 미발화 {} · 미설치 {})  [＋명세 밖 AUX {}]\n",
        n_spec_entry,
        CALLSITES20.len(),
        crate::probe20_tbl::MISSING20.len(),
        n_spec_entry + CALLSITES20.len() + crate::probe20_tbl::MISSING20.len(),
        ntry + ncs - miss.len(),
        hit.len(),
        zero.len(),
        miss.len(),
        n_aux
    ));
    s.push_str(&format!(
        "\n2단계(대조/sweep) 산출물은 `sweep20.txt` · 3단계(관측 전용 ABI 프로브) 는 `abiprobe.txt` 다\n\
         — 셋 다 **별도 파일**이다(섞으면 판정이 흐려진다). 이 판에서 다른 계측이 대체한 함수 {}개는\n\
         위 목록에 사유와 함께 있다([sweep]=발화수 sweep20.txt 에 있음 · [abiprobe]=발화수 측정 안 됨).\n",
        swept.len()
    ));
    s
}
