//! abiprobe — 3단계 = **관측 전용 ABI 프로브**(2026-09-12 신설 · 게임 0.5.8).
//! ===========================================================================
//! 왜 이 단계가 생겼나 (= 크래시 1건의 값)
//!   2단계 sweep 에서 `#04 handle_line_defense`(bit3)가 `0xc0000005` 로 게임을 죽였다.
//!   원인 = exe 의 그 함수는 **LTO 로 ABI 가 바뀌어** IR `define` 의 인자 순서와 다른데,
//!   `sweep20.rs` 의 래퍼가 **IR 순서대로 받았다고 가정**하고 그 값을 내 링크사본(`my_4`)에
//!   넘겼다 ⟹ 엉뚱한 값을 포인터로 역참조.
//!   `MIG\abiagree.py` 가 이걸 정적으로 재판정했고 런타임과 정확히 일치했다:
//!     ✅모순없음 = `#09`·`#01`·`#08`(셋 다 런타임 DIFF=0)  /  ⛔ABI 불일치 = `#04`·`#16`
//!   (`#16` 은 아직 안 켰지만 켰으면 같은 방식으로 죽었을 것이다.)
//!
//! ★그래서 이 단계의 계약은 **「아무 데도 넘기지 않는다」** 하나다.
//!   ①**인자를 어디로도 전달하지 않는다** — `my_*`(내 링크사본) **절대 호출 금지**.
//!   ②**상태를 바꾸지 않는다** — 레지스터·스택·플래그·xmm 전부 무변(아래 「무손상 계약」).
//!   ③**읽기만** 한다 — 그것도 `readable()` 이 참일 때만(인자가 포인터가 아닐 수 있다.
//!     그게 바로 알아내려는 것이다 ⟹ **포인터라고 가정하는 순간 이 단계도 죽는다**).
//!   ⟹ 정적 도출(ghidra/abiagree)이 틀렸어도 **게임은 죽지 않는다**. 번역 래퍼는 이 관측 뒤에 짠다.
//!
//! ───────────────────────── 무손상 계약(레지스터·스택·플래그) ─────────────────────────
//! 스텁은 `tfm2_ai_adjust\src\class_micro.rs::build_entry_stub` 의 **검증된 레이아웃을 그대로**
//! 쓴다(그쪽은 프로덕션에서 도는 진입 훅이다). 진입 rsp 를 `R`(관례상 `R%16==8`)이라 할 때:
//!   `pushfq`                                    → R-8    플래그 보존 ★
//!   `push rax rcx rdx r8 r9 r10 r11 rbp`        → R-72   volatile 정수 8종(+비volatile rbp=앵커)
//!   `lea rsp,[rsp-0x60]` + `movups [rsp+k*16],xmm{k}` (k=0..5) → R-168  volatile xmm 6종 ★
//!   `mov rbp,rsp`  ⟹ **rbp = R − 0xa8**(앵커. rbp 는 비volatile 이라 콜백이 보존해 준다)
//!   `lea rsp,[rsp-0x40]` + `and rsp,-16`         호출규약 16정렬 + shadow 0x20 + 스택인자 3칸
//!   콜백 인자는 **전부 앵커 상대(rbp+disp)로 읽는다** — 게임 레지스터를 하나도 안 깬다:
//!     rbp+0x90=원본 rcx · +0x88=rdx · +0x80=r8 · +0x78=r9 · +0xa8=리턴주소 · **+0xd0=[R+0x28]** · **+0xd8=[R+0x30]**
//!   `mov ecx,slot` `movabs rax,cb` `call rax`    관측(읽기 전용)
//!   `mov rsp,rbp` → xmm 복원 → `lea rsp,[rsp+0x60]` → pop 8종 → **`popfq`**  ⟹ 전부 원상복구
//!   그 다음 **원본 프롤로그 len 바이트를 그대로 실행** → `jmp [rip+0]` → `fn+len` 복귀.
//! ⟹ 함수 본문은 **진입 시와 비트동일한 레지스터·스택·플래그**로 시작한다. 관측만 남는다.
//! ★★`#16` 의 프롤로그엔 `48 8d 6c 24 30`(=`lea rbp,[rsp+0x30]`)이 있다 — **rsp 상대**다.
//!   그래서 「rsp 를 정확히 R 로 되돌리는 것」이 장식이 아니라 **정확성 조건**이다(1바이트 어긋나면
//!   rbp 가 틀어져 함수 전체가 엉뚱한 스택을 본다). `mov rsp,rbp`+`lea +0x60`+pop 8종 이 그걸 보장한다.
//! ⚠rip-상대·분기가 프롤로그 안에 있으면 옮겨 실행이 불가하지만, `#04`/`#16` 의 프롤로그는
//!   `MIG\probe20.py`(capstone)·`gensweep20.py` 가 이미 통과시킨 바이트열이다(= 1·2단계와 동일 바이트).
//!
//! ───────────────────────── 오버헤드 0 복귀(N회 뒤) ─────────────────────────
//! `#16` 은 1단계 실측 **105,791,090회**/판 발화한다. N회 관측 뒤엔 즉시 손을 떼야 한다.
//! 스텁 레이아웃(RWX 512B 한 칸):
//!   `+0x00` **DISPATCH**(u64, 8정렬) ← 이 칸만 갈아 끼운다
//!   `+0x10` ENTRY  : `ff 25 ea ff ff ff` = `jmp qword ptr [rip-0x16]` ⟹ `[+0x00]` 으로 점프
//!   `+0x16` SLOW   : 위 보존 스텁 본문 → `jmp rel32` → FAST
//!   `+K`    FAST   : 원본 프롤로그 + `jmp [rip+0]` + `.quad fn+len`
//! N회를 채우면 콜백이 `DISPATCH = FAST` 로 **8바이트 정렬 원자 store** 한다.
//!   ⟹ 레이스가 나도 어느 스레드든 **SLOW 또는 FAST = 둘 다 올바른 코드**를 본다(찢어질 값이 없다).
//!   ⟹ 이후 비용 = 간접 jmp 1회 + 프롤로그 + 간접 jmp 1회 = 1·2단계 트램폴린과 동급.
//!   ⛔진입부 12B 를 원본으로 되돌리는 방식은 **쓰지 않는다**(다른 스레드가 그 구간을 실행 중이면
//!     명령이 찢어진다 — 12B 를 원자적으로 못 쓴다).
//!
//! ───────────────────────── 배타(코드가 강제) ─────────────────────────
//! `probe`(1단계 진입부) · `sweep`(2단계) · `abiprobe`(3단계)는 **셋 다 같은 진입부 12B** 를 패치한다
//! ⟹ 공존 불가. 우선순위 = **sweep > abiprobe > probe** 를 3중으로 강제한다:
//!   ㉠ 설치 순서(`lib.rs::do_install`) = sweep → abiprobe → probe. 늦게 오는 쪽이 앞을 본다.
//!   ㉡ **게이트 레벨 교차 검사** — `sweep20_on.txt` 가 같은 함수를 요청했으면 abiprobe 는
//!      **설치 성공 여부와 무관하게** 건너뛴다(`install(.., sweep_mask, ..)`). 그리고 probe 는
//!      `abiprobe::is_installed_spec(idx)` 를 보고 건너뛴다.
//!   ㉢ 최후의 기계 가드 = 진입부 `48 b8` 선점 검사(누가 먼저 패치했으면 **미설치**).
//!      ㉠㉡ 가 논리 오류여도 ㉢ 이 이중 패치를 물리적으로 막는다.
//! ⚠㉡ 를 「sweep 이 실제 설치에 성공했을 때만」으로 좁히지 않은 이유: sweep 설치가 실패하면
//!   (예: 다른 모드 선점) abiprobe 가 슬그머니 대신 들어가 **유저가 요청한 적 없는 패치**가 된다.
//!   게이트가 충돌하면 **둘 다 자기 주장을 리포트에 적고 abiprobe 가 양보**하는 쪽이 안전하다.
//!
//! 켜는 법: `<게임>\mods\tfm2_judge_verify\abiprobe_on.txt` (**없으면 미설치** = 기본 OFF)
//!   · 비트마스크 줄(선택) `0x3` / `3`  — bit0=#04 · bit1=#16. 마스크 줄이 없으면 **전 슬롯**.
//!   · `n=<관측횟수>`(선택, 1..=64, 기본 8)
//! 산출 = `abiprobe.txt` (★`probe20.txt`/`sweep20.txt` 와 **별도 파일** — 섞으면 판정이 흐려진다)
//! ===========================================================================
#![allow(dead_code)]
use crate::{
    exe_base, readable, stub_reg, FlushInstructionCache, GetCurrentProcess, VirtualAlloc,
    VirtualProtect,
};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;

#[link(name = "kernel32")]
extern "system" {
    fn GetCurrentThreadId() -> u32;
}

const MEM_CR: u32 = 0x1000 | 0x2000; // MEM_COMMIT|MEM_RESERVE
const RWX: u32 = 0x40; // PAGE_EXECUTE_READWRITE
/// 스텁 한 칸. 실측 필요량 ≈ 250B(SLOW 약 200 + FAST 약 30 + 헤더 0x16) — 넉넉히 잡고
/// `build_stub` 이 **초과 시 설치를 포기**한다(조용한 절단 금지).
const STUB_SZ: usize = 512;
/// 스텁 내부 고정 오프셋.
const OFF_DISPATCH: usize = 0x00;
const OFF_ENTRY: usize = 0x10;
const OFF_SLOW: usize = 0x16;

// ───────────────────── IR 기대형(= `define` 의 인자 순서) ─────────────────────
/// 실측을 **무엇과 대조하는가**. 이 값이 리포트의 「IR 기대」 열이 되고, 여기서 ⛔가 나오면
/// 그것이 곧 「exe ABI ≠ IR ABI」의 런타임 증거다(abiagree.py 의 정적 판정과 같은 명제).
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Exp {
    /// `usize` — **판별력 없음**(어떤 비트열이든 양립한다). 🟡로만 나간다.
    Usize,
    StdRng,
    PState,
    OpData,
    LineTy,
    DbgFr,
    Entity,
    /// IR 상 인자가 아닌 위치(예: 3인자 함수의 a3..a5 = 호출자 스크래치).
    NotArg,
}
impl Exp {
    fn name(self) -> &'static str {
        match self {
            Exp::Usize => "usize",
            Exp::StdRng => "&mut StdRng(320B/al16)",
            Exp::PState => "&PlayerState(2528B)",
            Exp::OpData => "&OperationData(24B)",
            Exp::LineTy => "LineType(i8 0/1/2)",
            Exp::DbgFr => "&mut DebugFrameData(224B)",
            Exp::Entity => "&Entity",
            Exp::NotArg => "(IR 인자 아님)",
        }
    }
    fn cand(self) -> Option<usize> {
        match self {
            Exp::StdRng => Some(C_RNG),
            Exp::PState => Some(C_PS),
            Exp::OpData => Some(C_OD),
            Exp::LineTy => Some(C_LT),
            Exp::DbgFr => Some(C_DF),
            Exp::Entity => Some(C_ENT),
            _ => None,
        }
    }
}

// ───────────────────── 정합 후보(기계 판정의 어휘) ─────────────────────
// ★단순 hex 덤프만 남기면 사람이 또 눈으로 추측한다 ⟹ **후보별 비트**로 세고, 종합표에서
//   「전 관측 일관」만 판정으로 승격한다(1회성 우연 정합을 결론으로 쓰지 않기 위해).
pub const C_NULL: usize = 0; // 0
pub const C_SMALL: usize = 1; // < 0x10000 = 포인터 아님
pub const C_LT: usize = 2; // <= 2  (LineType 0/1/2)
pub const C_RD8: usize = 3; // readable(v,8) = 유효 포인터
pub const C_AL16: usize = 4; // 16정렬
pub const C_EXE: usize = 5; // exe 이미지 안(정적데이터/vtable 권역)
pub const C_PS: usize = 6; // &PlayerState 정합(강)
pub const C_RNG: usize = 7; // &StdRng 양립(약 — 크기·정렬만)
pub const C_OD: usize = 8; // &OperationData 양립(약)
pub const C_DF: usize = 9; // &DebugFrameData 양립(약)
pub const C_ENT: usize = 10; // &Entity 정합(휴리스틱)
pub const NCAND: usize = 11;
const CNAME: [&str; NCAND] = [
    "널",
    "소정수",
    "LineType(0..2)",
    "읽기가능",
    "16정렬",
    "exe안",
    "&PlayerState",
    "&StdRng",
    "&OperationData",
    "&DebugFrameData",
    "&Entity",
];
/// 종합표의 「역배치」에 쓸 **구조체 후보만**(소정수·정렬 따위 속성은 제외).
const STRUCT_CANDS: [usize; 5] = [C_PS, C_RNG, C_OD, C_DF, C_ENT];

const ARGN: [&str; 6] = ["a0", "a1", "a2", "a3", "a4", "a5"];
const REGN: [&str; 6] = ["rcx", "rdx", "r8", "r9", "[rsp+0x28]", "[rsp+0x30]"];

// ───────────────────────── 슬롯 표 ─────────────────────────
pub struct ASlot {
    pub bit: u8,
    pub idx: u8,
    pub name: &'static str,
    pub rva: usize,
    /// ★1·2단계와 **같은 바이트열**(probe20_tbl.rs / sweep20.rs 실측분). 완전일치만 패치한다.
    pub prolog: &'static [u8],
    /// IR `define` 의 인자 수(a3..a5 가 인자인지 구분해 리포트에 적는다).
    pub nargs: u8,
    pub exp: [Exp; 6],
    pub ir: &'static str,
    pub stub: AtomicUsize,
    pub slow: AtomicUsize,
    pub fast: AtomicUsize,
    /// 스텁이 콜백을 부른 횟수(관측 상한 초과분 포함). 상한 뒤엔 DISPATCH=FAST 라 더 늘지 않는다.
    pub seen: AtomicU32,
    /// 실제로 기록한 관측 수(= 종합표의 분모).
    pub rec: AtomicU32,
    /// 설치 결과/미설치 사유(사람이 읽는 문장).
    pub why: Mutex<String>,
    pub fit: [[AtomicU32; NCAND]; 6],
}

macro_rules! asl {
    ($b:expr, $i:expr, $n:expr, $r:expr, $p:expr, $na:expr, $e:expr, $ir:expr) => {
        ASlot {
            bit: $b, idx: $i, name: $n, rva: $r, prolog: $p, nargs: $na, exp: $e, ir: $ir,
            stub: AtomicUsize::new(0), slow: AtomicUsize::new(0), fast: AtomicUsize::new(0),
            seen: AtomicU32::new(0), rec: AtomicU32::new(0),
            why: Mutex::new(String::new()),
            // ⚠외곽 반복(`; 6`)도 `const {}` 로 감싸야 한다 — 안 감싸면 내부 배열에 `Copy` 를
            //   요구해 `Atomic<u32>: Copy` 로 컴파일 에러가 난다(실측 2026-09-12).
            fit: [const { [const { AtomicU32::new(0) }; NCAND] }; 6],
        }
    };
}

/// ★대상 = `abiagree.py` 가 **⛔ABI 불일치**로 판정한 2함수뿐이다.
///   `#09`·`#01`·`#08` 은 이미 런타임 DIFF=0 을 확보했으므로 여기 넣지 않는다(건드릴 이유가 없다).
pub static S: [ASlot; 2] = [
    asl!(
        0, 4, "handle_line_defense", 0xd3e4b0,
        &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x54, 0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x50],
        6,
        [Exp::Usize, Exp::StdRng, Exp::PState, Exp::OpData, Exp::LineTy, Exp::DbgFr],
        "fn(usize, &mut StdRng, &PlayerState, &OperationData, LineType, &mut DebugFrameData) -> bool"
    ),
    asl!(
        1, 16, "max_range_nearly_can_use", 0xc809d0,
        &[0x55, 0x56, 0x57, 0x48, 0x83, 0xec, 0x30, 0x48, 0x8d, 0x6c, 0x24, 0x30],
        3,
        [Exp::Entity, Exp::Entity, Exp::Usize, Exp::NotArg, Exp::NotArg, Exp::NotArg],
        "fn(&Entity, &Entity, usize) -> u64"
    ),
];

/// 관측 상한(슬롯당). 게이트 `n=` 으로 1..=64 조절.
static NOBS: AtomicU32 = AtomicU32::new(8);
#[inline]
fn nobs() -> u32 {
    NOBS.load(Ordering::Relaxed)
}
/// 관측 본문 로그(슬롯 인덱스, 텍스트). detour 문맥에서 잡으므로 poison-safe.
static OBS: Mutex<Vec<(usize, String)>> = Mutex::new(Vec::new());
const OBS_MAX: usize = 160;

pub fn is_installed_spec(idx: u8) -> bool {
    S.iter().any(|s| s.idx == idx && s.stub.load(Ordering::Relaxed) != 0)
}
pub fn any_installed() -> bool {
    S.iter().any(|s| s.stub.load(Ordering::Relaxed) != 0)
}
/// 「판이 돌고 있나」 판정 합에 얹는 값(상한까지만 는다 — 그래도 단조라 무해하다).
pub fn total_seen() -> u64 {
    S.iter().map(|s| s.seen.load(Ordering::Relaxed) as u64).sum()
}

// ───────────────────── exe 이미지 범위(PE 헤더 실측) ─────────────────────
static EXE_SZ: AtomicUsize = AtomicUsize::new(usize::MAX); // MAX = 아직 안 읽음
/// (base, SizeOfImage). 값이 안 잡히면 size=0 → 「exe 안」 판정을 아예 하지 않는다(거짓 단정 금지).
unsafe fn exe_span() -> (usize, usize) {
    let b = exe_base();
    let c = EXE_SZ.load(Ordering::Relaxed);
    if c != usize::MAX {
        return (b, c);
    }
    let mut sz = 0usize;
    if b != 0 && readable(b + 0x3c, 4) {
        let e = core::ptr::read_unaligned((b + 0x3c) as *const u32) as usize;
        // NT 헤더: sig(4) + FileHeader(20) = OptionalHeader @ e+0x18 · PE32+ SizeOfImage @ +0x38
        if e > 0 && e < 0x1000 && readable(b + e + 0x50, 4) {
            sz = core::ptr::read_unaligned((b + e + 0x50) as *const u32) as usize;
        }
    }
    EXE_SZ.store(sz, Ordering::Relaxed);
    (b, sz)
}

// ───────────────────── 안전 읽기(⛔역참조 전 반드시 readable) ─────────────────────
#[inline]
unsafe fn rd_u8(a: usize) -> Option<u8> {
    if a >= 0x10000 && readable(a, 1) {
        Some(*(a as *const u8))
    } else {
        None
    }
}
#[inline]
unsafe fn rd_u32(a: usize) -> Option<u32> {
    if a >= 0x10000 && readable(a, 4) {
        Some(core::ptr::read_unaligned(a as *const u32))
    } else {
        None
    }
}

unsafe fn hexdump(a: usize) -> String {
    let mut s = String::new();
    for row in 0..4usize {
        let base = a.wrapping_add(row * 16);
        if !readable(base, 16) {
            s.push_str(&format!("           +0x{:02x}: (읽기불가 — 여기서 중단)\n", row * 16));
            break;
        }
        let mut h = String::new();
        for j in 0..16usize {
            h.push_str(&format!("{:02x} ", *(base.wrapping_add(j) as *const u8)));
        }
        s.push_str(&format!("           +0x{:02x}: {}\n", row * 16, h.trim_end()));
    }
    s
}

// ───────────────────────── 관측(콜백 본체) ─────────────────────────
/// 한 인자 위치의 판정 문장을 만든다. 반환 = (정합 비트, 텍스트).
/// ⛔여기서 하는 일은 **읽기뿐**이다. 값을 어디에도 넘기지 않는다.
unsafe fn arg_text(sl: &ASlot, k: usize, v: u64, all: &[u64; 6]) -> (u32, String) {
    let mut b = 0u32;
    let a = v as usize;
    let mut s = String::new();
    let exp = sl.exp[k];
    s.push_str(&format!(
        "      {} {:<11} = {:#018x}   IR기대={}",
        ARGN[k], REGN[k], v, exp.name()
    ));
    let dup: Vec<&str> = (0..6)
        .filter(|&j| j != k && all[j] == v && v != 0)
        .map(|j| ARGN[j])
        .collect();
    if !dup.is_empty() {
        s.push_str(&format!("  (= {} 와 같은 값)", dup.join(",")));
    }
    s.push('\n');

    if v == 0 {
        b |= 1 << C_NULL;
        s.push_str("           판정: 널(0) ⟹ 포인터 아님 · 정수 0 과 양립\n");
        return (b, s);
    }
    if v < 0x10000 {
        b |= 1 << C_SMALL;
    }
    if v <= 2 {
        b |= 1 << C_LT;
    }
    if a % 16 == 0 {
        b |= 1 << C_AL16;
    }
    let pt = v >= 0x10000 && v < (1u64 << 48) && readable(a, 8);
    if pt {
        b |= 1 << C_RD8;
    }
    let (eb, esz) = exe_span();
    let in_exe = pt && eb != 0 && esz != 0 && a >= eb && a < eb.wrapping_add(esz);
    if in_exe {
        b |= 1 << C_EXE;
    }

    // ── ① 값 자체의 성격 ──
    if b & (1 << C_SMALL) != 0 {
        s.push_str(&format!(
            "           판정: 소정수({}) ⟹ **포인터 아님**(readable 검사조차 불필요){}\n",
            v,
            if v <= 2 { " · LineType(0/1/2) 범위 안 ✓" } else { " · LineType 범위 밖" }
        ));
        return (b, s);
    }
    if v >= (1u64 << 48) {
        s.push_str("           판정: 48비트 범위 밖 ⟹ **포인터 아님**(정수/비트필드/부동소수 추정)\n");
        return (b, s);
    }
    if !pt {
        s.push_str("           판정: readable(v,8)=false ⟹ **유효 포인터 아님**(비커밋/가드/PAGE_NOACCESS)\n");
        return (b, s);
    }
    s.push_str(&format!(
        "           판정: 읽기가능 ✓ · {} · {}\n",
        if in_exe {
            format!("exe+{:#x}(정적데이터/vtable 권역)", a - eb)
        } else {
            "exe 이미지 밖(힙/스택/타모듈)".to_string()
        },
        if b & (1 << C_AL16) != 0 { "16정렬 ✓" } else { "16정렬 ✗" }
    ));

    // ── ② 구조체 정합(★판정 문장) ──
    let mut ok: Vec<String> = Vec::new();
    let mut no: Vec<String> = Vec::new();

    // &PlayerState = 2528B · +0x930 info.team ∈{0,1} · +0x9c0 info.position ∈0..=4
    //   ⟹ 이 모드에서 **가장 판별력이 높은 검사**(두 필드가 동시에 좁은 범위).
    let t8 = rd_u8(a.wrapping_add(0x930));
    let p8 = rd_u8(a.wrapping_add(0x9c0));
    let t32 = rd_u32(a.wrapping_add(0x930));
    let p32 = rd_u32(a.wrapping_add(0x9c0));
    let ps_sz = readable(a, 2528);
    if ps_sz && matches!(t8, Some(0) | Some(1)) && matches!(p8, Some(x) if x <= 4) {
        b |= 1 << C_PS;
        ok.push(format!(
            "&PlayerState ✓ (2528B 읽기가능 · [+0x930] u8={} u32={:#x} ∈{{0,1}} · [+0x9c0] u8={} u32={:#x} ∈0..4)",
            t8.unwrap_or(255),
            t32.unwrap_or(0),
            p8.unwrap_or(255),
            p32.unwrap_or(0)
        ));
    } else {
        no.push(format!(
            "&PlayerState ✗ (2528B={} · [+0x930]={:?} · [+0x9c0]={:?})",
            ps_sz, t8, p8
        ));
    }
    // &mut StdRng = 320B · align 16 ⟹ **16정렬 위반이면 배제 가능**(내용 판별은 불가 = 약한 정합)
    if readable(a, 320) && (b & (1 << C_AL16)) != 0 {
        b |= 1 << C_RNG;
        ok.push("&StdRng △ (320B 읽기가능 + 16정렬 — 배제 못 함. 확정 아님)".into());
    } else {
        no.push(format!(
            "&StdRng ✗ ({})",
            if b & (1 << C_AL16) == 0 { "16정렬 아님 = align16 위반" } else { "320B 읽기불가" }
        ));
    }
    if readable(a, 24) {
        b |= 1 << C_OD;
        ok.push("&OperationData △ (24B — 크기가 작아 판별력 거의 없음)".into());
    } else {
        no.push("&OperationData ✗ (24B 읽기불가)".into());
    }
    if readable(a, 224) {
        b |= 1 << C_DF;
        ok.push("&DebugFrameData △ (224B)".into());
    } else {
        no.push("&DebugFrameData ✗ (224B 읽기불가)".into());
    }
    // &Entity = +0x0 team 태그 · +0x68 ty 판별자
    //   ⚠값 범위는 유저 제공 사실에 없다 ⟹ **휴리스틱**(team u8∈{0,1} · ty u32<16)으로만 판정하고
    //     리포트에 휴리스틱이라고 못 박는다(사실과 추정을 섞지 않는다 — CLAUDE.md §10).
    let e0 = rd_u8(a);
    let e68 = rd_u32(a.wrapping_add(0x68));
    if readable(a, 0x70) && matches!(e0, Some(0) | Some(1)) && matches!(e68, Some(x) if x < 16) {
        b |= 1 << C_ENT;
        ok.push(format!(
            "&Entity ✓휴리스틱 ([+0x0] team u8={} ∈{{0,1}} · [+0x68] ty u32={} <16)",
            e0.unwrap_or(255),
            e68.unwrap_or(9999)
        ));
    } else {
        no.push(format!("&Entity ✗휴리스틱 ([+0x0]={:?} · [+0x68]={:?})", e0, e68));
    }

    s.push_str(&format!("           정합: {}\n", ok.join("  |  ")));
    s.push_str(&format!("           배제: {}\n", no.join("  |  ")));
    s.push_str(&hexdump(a));
    (b, s)
}

unsafe fn observe(i: usize, n: u32, v: [u64; 6]) {
    let sl = &S[i];
    let mut s = String::new();
    s.push_str(&format!(
        "  ▣ 관측 #{}/{}  (#{:02} {} · tid {:#x})\n",
        n + 1,
        nobs(),
        sl.idx,
        sl.name,
        GetCurrentThreadId()
    ));
    for k in 0..6usize {
        let (bits, line) = arg_text(sl, k, v[k], &v);
        for c in 0..NCAND {
            if bits & (1 << c) != 0 {
                sl.fit[k][c].fetch_add(1, Ordering::Relaxed);
            }
        }
        s.push_str(&line);
    }
    sl.rec.fetch_add(1, Ordering::Relaxed);
    let mut g = OBS.lock().unwrap_or_else(|e| e.into_inner());
    if g.len() < OBS_MAX {
        g.push((i, s));
    }
}

/// DISPATCH 를 FAST 로 갈아 끼운다(8바이트 정렬 원자 store — 위 헤더 「오버헤드 0 복귀」).
unsafe fn flip(sl: &ASlot) {
    let stub = sl.stub.load(Ordering::Relaxed);
    let fast = sl.fast.load(Ordering::Relaxed);
    if stub != 0 && fast != 0 {
        (*((stub + OFF_DISPATCH) as *const AtomicU64)).store(fast as u64, Ordering::SeqCst);
    }
}

/// 스텁이 부르는 콜백. `extern "C"` = Windows x64 규약(스텁이 그 규약으로 인자를 놓는다).
/// ⚠detour 문맥이라 패닉이 게임 콜스택을 unwind 하면 UB ⟹ `catch_unwind` 필수(CLAUDE.md §3).
unsafe extern "C" fn abi_cb(slot: u32, a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        let i = slot as usize;
        if i >= S.len() {
            return;
        }
        let sl = &S[i];
        let n = sl.seen.fetch_add(1, Ordering::Relaxed);
        if n >= nobs() {
            flip(sl); // 레이스로 상한을 넘겨 들어온 스레드 — 여기서도 한 번 더 내린다
            return;
        }
        observe(i, n, [a0, a1, a2, a3, a4, a5]);
        if n + 1 >= nobs() {
            flip(sl);
        }
    }));
}

// ───────────────────────── 스텁 생성 ─────────────────────────
/// 반환 = (SLOW 진입주소, FAST 진입주소). 실패 시 스텁 버퍼는 **미기록**.
unsafe fn build_stub(
    stub: usize,
    slot: u32,
    prolog: &[u8],
    ret_addr: usize,
) -> Result<(usize, usize), &'static str> {
    let mut s: Vec<u8> = Vec::with_capacity(STUB_SZ);
    // +0x00 DISPATCH(u64) · +0x08 예약(u64 — ENTRY 를 0x10 에 두기 위한 패딩)
    s.extend_from_slice(&0u64.to_le_bytes());
    s.extend_from_slice(&0u64.to_le_bytes());
    // +0x10 ENTRY: jmp qword ptr [rip-0x16] → [stub+0x00]
    s.extend_from_slice(&[0xff, 0x25]);
    s.extend_from_slice(&(-(OFF_SLOW as i32)).to_le_bytes());
    if s.len() != OFF_SLOW {
        return Err("스텁 헤더 오프셋 불일치(코드 버그)");
    }

    // ── SLOW: 보존 → 관측 → 복원 ──
    s.push(0x9c); // pushfq                      ★플래그 보존
    s.extend_from_slice(&[0x50, 0x51, 0x52, 0x41, 0x50, 0x41, 0x51, 0x41, 0x52, 0x41, 0x53, 0x55]);
    //                    rax   rcx   rdx   r8          r9          r10         r11         rbp
    s.extend_from_slice(&[0x48, 0x8d, 0xa4, 0x24, 0xa0, 0xff, 0xff, 0xff]); // lea rsp,[rsp-0x60]
    for k in 0..6u8 {
        s.extend_from_slice(&[0x0f, 0x11, 0x44 | ((k & 7) << 3), 0x24, k * 16]); // movups [rsp+k*16],xmm{k}
    }
    s.extend_from_slice(&[0x48, 0x89, 0xe5]); // mov rbp,rsp        ★앵커 = 진입rsp − 0xa8
    s.extend_from_slice(&[0x48, 0x8d, 0x64, 0x24, 0xc0]); // lea rsp,[rsp-0x40]  (shadow 0x20 + 스택인자 3칸)
    s.extend_from_slice(&[0x48, 0x83, 0xe4, 0xf0]); // and rsp,-16        (호출규약 16정렬)
    // ★인자는 **앵커 상대**로 읽는다 — 게임 레지스터를 하나도 깨지 않는다.
    s.extend_from_slice(&[0x48, 0x8b, 0x95]); // mov rdx,[rbp+0x90]  = 원본 rcx
    s.extend_from_slice(&0x90u32.to_le_bytes());
    s.extend_from_slice(&[0x4c, 0x8b, 0x85]); // mov r8, [rbp+0x88]  = 원본 rdx
    s.extend_from_slice(&0x88u32.to_le_bytes());
    s.extend_from_slice(&[0x4c, 0x8b, 0x8d]); // mov r9, [rbp+0x80]  = 원본 r8
    s.extend_from_slice(&0x80u32.to_le_bytes());
    s.extend_from_slice(&[0x48, 0x8b, 0x85]); // mov rax,[rbp+0x78]  = 원본 r9
    s.extend_from_slice(&0x78u32.to_le_bytes());
    s.extend_from_slice(&[0x48, 0x89, 0x44, 0x24, 0x20]); // mov [rsp+0x20],rax
    s.extend_from_slice(&[0x48, 0x8b, 0x85]); // mov rax,[rbp+0xd0]  = 진입 [rsp+0x28]
    s.extend_from_slice(&0xd0u32.to_le_bytes());
    s.extend_from_slice(&[0x48, 0x89, 0x44, 0x24, 0x28]); // mov [rsp+0x28],rax
    s.extend_from_slice(&[0x48, 0x8b, 0x85]); // mov rax,[rbp+0xd8]  = 진입 [rsp+0x30]
    s.extend_from_slice(&0xd8u32.to_le_bytes());
    s.extend_from_slice(&[0x48, 0x89, 0x44, 0x24, 0x30]); // mov [rsp+0x30],rax
    s.push(0xb9); // mov ecx, slot
    s.extend_from_slice(&slot.to_le_bytes());
    s.extend_from_slice(&[0x48, 0xb8]); // movabs rax, abi_cb
    s.extend_from_slice(&(abi_cb as usize).to_le_bytes());
    s.extend_from_slice(&[0xff, 0xd0]); // call rax
    s.extend_from_slice(&[0x48, 0x89, 0xec]); // mov rsp,rbp        (앵커 복귀 — rbp 는 비volatile)
    for k in 0..6u8 {
        s.extend_from_slice(&[0x0f, 0x10, 0x44 | ((k & 7) << 3), 0x24, k * 16]); // movups xmm{k},[rsp+k*16]
    }
    s.extend_from_slice(&[0x48, 0x8d, 0x64, 0x24, 0x60]); // lea rsp,[rsp+0x60]
    s.extend_from_slice(&[0x5d, 0x41, 0x5b, 0x41, 0x5a, 0x41, 0x59, 0x41, 0x58, 0x5a, 0x59, 0x58]);
    //                    rbp   r11         r10         r9          r8          rdx   rcx   rax
    s.push(0x9d); // popfq                      ★플래그 복원 ⟹ 여기서 진입 상태와 비트동일
    s.push(0xe9); // jmp rel32 → FAST
    let relpos = s.len();
    s.extend_from_slice(&0i32.to_le_bytes());

    // ── FAST: 원본 프롤로그 그대로 실행 → fn+len 복귀 ──
    let fast_off = s.len();
    let rel = fast_off as i64 - (relpos as i64 + 4);
    if rel < 0 || rel > 0x7000 {
        return Err("내부 rel32 이상(코드 버그)");
    }
    s[relpos..relpos + 4].copy_from_slice(&(rel as i32).to_le_bytes());
    s.extend_from_slice(prolog);
    s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]); // jmp [rip+0]
    s.extend_from_slice(&ret_addr.to_le_bytes()); // .quad fn+len

    if s.len() > STUB_SZ {
        return Err("스텁 버퍼 초과(STUB_SZ 를 늘려야 한다) — 미설치");
    }
    // DISPATCH 초기값 = SLOW
    let slow = stub + OFF_SLOW;
    let fast = stub + fast_off;
    s[OFF_DISPATCH..OFF_DISPATCH + 8].copy_from_slice(&(slow as u64).to_le_bytes());
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());
    Ok((slow, fast))
}

// ───────────────────────── 설치 ─────────────────────────
unsafe fn install_one(i: usize) -> Result<(usize, usize, usize), &'static str> {
    let sl = &S[i];
    let len = sl.prolog.len();
    if len < 12 || len > 24 {
        return Err("프롤로그 길이 범위(12..=24) 밖 — 표 이상");
    }
    if sl.stub.load(Ordering::Relaxed) != 0 {
        return Err("이미 설치됨(중복 호출)");
    }
    let base = exe_base();
    if base == 0 {
        return Err("exe_base 0");
    }
    let fn_addr = base.wrapping_add(sl.rva);
    if fn_addr < 0x10000 || fn_addr >= (1usize << 48) {
        return Err("fn 주소 범위 밖");
    }
    if !readable(fn_addr, len + 4) {
        return Err("fn 읽기불가(RVA 가 코드가 아님)");
    }
    // ㉢ 최후의 기계 가드 — 누가 먼저 훅했으면 **절대** 건드리지 않는다(체인 미지원).
    if *(fn_addr as *const u8) == 0x48 && *((fn_addr + 1) as *const u8) == 0xb8 {
        return Err("이미 훅됨(진입부 `48 b8` 선점) — 체인 미지원이라 설치 안 함");
    }
    // 프롤로그 **완전일치** 검증(스테일 RVA/패치판 차단). 불일치면 .text 는 무손상.
    for k in 0..len {
        if *((fn_addr + k) as *const u8) != sl.prolog[k] {
            return Err("프롤로그 바이트 불일치(스테일 RVA/패치판) — 미설치");
        }
    }

    let stub = stub_reg(VirtualAlloc(0, STUB_SZ, MEM_CR, RWX), STUB_SZ, sl.rva);
    if stub == 0 {
        return Err("VirtualAlloc 실패");
    }
    let (slow, fast) = build_stub(stub, i as u32, sl.prolog, fn_addr + len)?;
    // ★패치 **전에** 게시(hookw 의 교훈: 패치 직후 다른 스레드가 들어와 0 을 읽으면 flip 을 못 한다).
    sl.slow.store(slow, Ordering::SeqCst);
    sl.fast.store(fast, Ordering::SeqCst);
    sl.stub.store(stub, Ordering::SeqCst);

    let entry = stub + OFF_ENTRY;
    let mut patch = vec![0x90u8; len]; // 나머지는 NOP
    patch[0] = 0x48;
    patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&entry.to_le_bytes());
    patch[10] = 0xff;
    patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, len, RWX, &mut old) == 0 {
        sl.stub.store(0, Ordering::SeqCst); // 게시 취소 — 패치 안 됐으니 스텁은 불리지 않는다
        return Err("VirtualProtect 실패");
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, len);
    VirtualProtect(fn_addr, len, old, &mut old); // ★원래 보호속성 복원
    FlushInstructionCache(GetCurrentProcess(), fn_addr, len);
    Ok((stub, slow, fast))
}

/// abiprobe 설치. `mask` 비트 k = `S[k]`. `sweep_mask` = 2단계 게이트(교차 배타 판정용).
/// 반환 = (성공, 시도).
pub unsafe fn install(mask: u64, obs: u32, sweep_mask: u128, log: &mut String) -> (usize, usize) {
    NOBS.store(obs.clamp(1, 64), Ordering::Relaxed);
    if mask == 0 {
        log.push_str("[abiprobe] 게이트 OFF (abiprobe_on.txt 없음/0) — 한 곳도 안 걸었다\n");
        for sl in S.iter() {
            *sl.why.lock().unwrap_or_else(|e| e.into_inner()) =
                "게이트 OFF(abiprobe_on.txt 없음/0) — 미설치".into();
        }
        return (0, 0);
    }
    let unknown = mask & !((1u64 << S.len()) - 1);
    if unknown != 0 {
        log.push_str(&format!(
            "[abiprobe] ⚠mask 의 미지 비트 {:#x} 는 슬롯이 없어 무시했다(슬롯 {}개)\n",
            unknown,
            S.len()
        ));
    }
    let (mut ok, mut tried) = (0usize, 0usize);
    for i in 0..S.len() {
        let sl = &S[i];
        if mask & (1u64 << i) == 0 {
            *sl.why.lock().unwrap_or_else(|e| e.into_inner()) =
                "게이트 미선택(mask 비트 0)".into();
            continue;
        }
        tried += 1;
        // ㉡ 게이트 레벨 교차 배타 — sweep 이 같은 함수를 **요청만 해도** 양보한다.
        let sbit = crate::sweep20::S.iter().find(|x| x.idx == sl.idx).map(|x| x.bit);
        let sweep_wants = sbit.map(|b| sweep_mask & (1u128 << b) != 0).unwrap_or(false);
        let sweep_has = crate::sweep20::is_installed_spec(sl.idx);
        if sweep_wants || sweep_has {
            let why = format!(
                "⛔건너뜀 — sweep(2단계)과 **동시 사용 금지**(같은 진입부 12B) · sweep20_on.txt bit{} 요청={} · sweep 실제설치={}",
                sbit.map(|b| b.to_string()).unwrap_or("?".into()),
                sweep_wants,
                sweep_has
            );
            log.push_str(&format!(
                "[abiprobe] bit{} #{:02} {} {}\n",
                i, sl.idx, sl.name, why
            ));
            *sl.why.lock().unwrap_or_else(|e| e.into_inner()) = why;
            continue;
        }
        match install_one(i) {
            Ok((stub, slow, fast)) => {
                ok += 1;
                let why = format!(
                    "설치 OK @{:#x} · 스텁 {:#x}(진입 {:#x} · SLOW {:#x} · FAST {:#x}) · 프롤로그 {}B 이동",
                    exe_base() + sl.rva,
                    stub,
                    stub + OFF_ENTRY,
                    slow,
                    fast,
                    sl.prolog.len()
                );
                log.push_str(&format!("[abiprobe] bit{} #{:02} {} {}\n", i, sl.idx, sl.name, why));
                *sl.why.lock().unwrap_or_else(|e| e.into_inner()) = why;
            }
            Err(e) => {
                let why = format!("⛔미설치 — {} (@{:#x} · .text 무손상)", e, exe_base() + sl.rva);
                log.push_str(&format!("[abiprobe] bit{} #{:02} {} {}\n", i, sl.idx, sl.name, why));
                *sl.why.lock().unwrap_or_else(|e| e.into_inner()) = why;
            }
        }
    }
    (ok, tried)
}

// ───────────────────────── 리포트 ─────────────────────────
fn vmark(fit: u32, rec: u32) -> &'static str {
    if rec == 0 {
        "판정불가(관측 0)"
    } else if fit == rec {
        "✅부합"
    } else if fit == 0 {
        "⛔모순"
    } else {
        "🟡일부"
    }
}

pub fn report(header: &str, gate: &str, inst: &str) -> String {
    let mut s = String::new();
    s.push_str("=== tfm2_judge_verify 3단계 · abiprobe (관측 전용 ABI 프로브 — 인자를 아무 데도 넘기지 않는다) ===\n");
    s.push_str(header);
    s.push_str("\n\n");
    s.push_str(&format!("--- 게이트: {}\n--- 설치:\n{}\n", gate, inst));
    s.push_str(
        "--- 배타(★코드가 강제 · 우선순위 sweep > abiprobe > probe)\n\
        \x20   ㉠ 설치 순서 = sweep → abiprobe → probe (lib.rs::do_install — 늦게 오는 쪽이 앞을 본다)\n\
        \x20   ㉡ 게이트 교차 검사 = sweep20_on.txt 가 같은 함수를 **요청만 해도** abiprobe 양보 /\n\
        \x20      probe 진입부는 abiprobe::is_installed_spec(idx) 를 보고 자기를 건너뛴다(probe20.txt 「대체」 목록)\n\
        \x20   ㉢ 최후의 기계 가드 = 진입부 `48 b8` 선점 검사 ⟹ ㉠㉡ 가 틀려도 이중 패치는 물리적으로 불가\n\
        \x20   ⚠셋을 같은 함수에 동시 설치하는 경로는 **없다**. 하나라도 잡으면 나머지는 사유를 남기고 빠진다.\n\n",
    );
    s.push_str(
        "--- 무손상 계약: pushfq / push rax rcx rdx r8 r9 r10 r11 rbp / movups xmm0..5 로 보존 →\n\
        \x20   인자는 **앵커(rbp = 진입rsp−0xa8) 상대로만 읽고** → mov rsp,rbp → 역순 복원 → popfq →\n\
        \x20   **원본 프롤로그 그대로 실행** → jmp fn+len. ⟹ 함수 본문은 진입 시와 비트동일한 상태로 시작한다.\n\
        \x20   (#16 프롤로그의 `lea rbp,[rsp+0x30]` 은 rsp 상대라 rsp 정확복원이 정확성 조건이다)\n\n",
    );

    for (i, sl) in S.iter().enumerate() {
        let rec = sl.rec.load(Ordering::Relaxed);
        let seen = sl.seen.load(Ordering::Relaxed);
        let stub = sl.stub.load(Ordering::Relaxed);
        s.push_str(&format!(
            "═══ 슬롯 bit{} · #{:02} {} (rva {:#x}) ═══\n  상태: {} · 콜백진입 {} · 기록 {}/{} · 디스패치 {}\n  사유: {}\n  IR define: {}\n",
            i, sl.idx, sl.name, sl.rva,
            if stub != 0 { "설치됨" } else { "미설치" },
            seen, rec, nobs(),
            if stub != 0 {
                let cur = unsafe { (*((stub + OFF_DISPATCH) as *const AtomicU64)).load(Ordering::Relaxed) } as usize;
                if cur == sl.fast.load(Ordering::Relaxed) { "FAST(관측 종료 · 오버헤드 복귀)" } else { "SLOW(관측 중)" }
            } else { "-" },
            sl.why.lock().unwrap_or_else(|e| e.into_inner()).as_str(),
            sl.ir
        ));
        if stub == 0 {
            s.push('\n');
            continue;
        }
        if rec == 0 {
            s.push_str("  ⚠관측 0건 = **판정 불성립**(그 함수가 이 판에서 안 떴다). 1단계 발화수를 확인하라.\n\n");
            continue;
        }

        // ── 종합표(전 관측 일관만 판정으로 승격) ──
        s.push_str(&format!(
            "\n  ▣ 종합 ({}회 관측 · ★「전 관측 일관」만 판정으로 쓴다)\n    위치 레지스터     IR 기대                      실측 일관 정합후보                        판정\n",
            rec
        ));
        let mut contradiction = false;
        for k in 0..6usize {
            let exp = sl.exp[k];
            let consistent: Vec<&str> = (0..NCAND)
                .filter(|&c| sl.fit[k][c].load(Ordering::Relaxed) == rec)
                .map(|c| CNAME[c])
                .collect();
            let mark = match exp {
                Exp::NotArg => "—(IR 인자 아님)".to_string(),
                Exp::Usize => "🟡판별력없음(usize)".to_string(),
                _ => {
                    let c = exp.cand().unwrap();
                    let f = sl.fit[k][c].load(Ordering::Relaxed);
                    let m = vmark(f, rec);
                    if m == "⛔모순" {
                        contradiction = true;
                    }
                    format!("{}({}/{})", m, f, rec)
                }
            };
            s.push_str(&format!(
                "    {:<4} {:<12} {:<28} {:<40} {}\n",
                ARGN[k],
                REGN[k],
                exp.name(),
                if consistent.is_empty() { "(일관 정합 없음)".to_string() } else { consistent.join(",") },
                mark
            ));
        }
        // ── 역배치: 각 구조체 후보가 **전 관측 일관**으로 정합한 위치 ──
        s.push_str("\n  ▣ 역배치 (각 구조체 후보가 전 관측에서 일관되게 정합한 위치)\n");
        for &c in STRUCT_CANDS.iter() {
            let pos: Vec<String> = (0..6)
                .filter(|&k| sl.fit[k][c].load(Ordering::Relaxed) == rec)
                .map(|k| format!("{}({})", ARGN[k], REGN[k]))
                .collect();
            s.push_str(&format!(
                "    {:<18} → {}{}\n",
                CNAME[c],
                if pos.is_empty() { "(없음)".to_string() } else { pos.join(", ") },
                if pos.len() == 1 { "   [유일 ⟹ 배치 확정 후보]" } else if pos.len() > 1 { "   [다중 ⟹ 이 검사만으론 판별 불충분]" } else { "" }
            ));
        }
        s.push_str(&format!(
            "\n  ▣ 결론: {}\n",
            if contradiction {
                "⛔**IR define 의 인자 순서가 exe 실측과 다르다** — 번역 래퍼 없이 `my_*` 를 부르면 죽는다(= 2단계 크래시의 원인). 위 역배치로 실제 배치를 확정하고 래퍼를 짜라."
            } else {
                "✅이 관측 범위에서 IR 기대와 **모순 없음**. 단 △(약한 정합)만으로 부합이 난 위치는 확정이 아니다 — 강한 검사(&PlayerState·&Entity)가 걸린 위치를 근거로 삼아라."
            }
        ));
        s.push('\n');
    }

    // ── 관측 원문 ──
    let g = OBS.lock().unwrap_or_else(|e| e.into_inner());
    s.push_str(&format!(
        "─────────────────────────────────────────────────────────────\n\
         관측 원문 {}건 (상한 {}) — 종합표의 근거. hex 는 각 후보 포인터의 앞 64B\n\
         ─────────────────────────────────────────────────────────────\n",
        g.len(),
        OBS_MAX
    ));
    if g.is_empty() {
        s.push_str("  (없음)\n");
    }
    for (i, t) in g.iter() {
        let _ = i;
        s.push_str(t);
    }
    s.push_str(
        "\n★판정 어휘: ✓=구조 필드까지 맞은 강한 정합 · △=크기/정렬만 맞은 약한 정합(배제 못 한다는 뜻일 뿐) ·\n\
        \x20 ✗=배제됨 · ✓휴리스틱=유저 제공 사실에 값 범위가 없어 범위를 가정한 검사(&Entity).\n\
         ⚠**미관측**: xmm0..3(부동소수 인자) · 스택 7번째 이후 인자 · 반환값 — 이 프로브는 정수 4 + 스택 2 칸만 본다.\n\
         1·2단계 산출물은 probe20.txt / sweep20.txt (별도 파일) 다 — 섞어 읽지 말 것.\n",
    );
    s
}
