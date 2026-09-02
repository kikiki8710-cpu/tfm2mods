// class_micro.rs — 바이트패치 노브를 **클래스별 값**으로 여는 마이크로 디투어.
//
// ★왜 필요한가 (배경)
//   클래스별 값(`{key}_class_<cls>`)은 `CUR_CLASS` 가 세팅된 판단 문맥에서 `tune()` 이 불릴 때만 동작한다.
//   `apply_*_imm()` 안에서만 읽히는 노브(0.5.4 기준 340여 개)는 exe 기계어 상수 한 자리를 고치는 방식이라
//   **선수마다 다른 값을 가질 수 없다** — 유저가 원본 테스트C cfg 에 넣은 `_class_` 20줄이 전부 무시된 이유다.
//
// ★이 파일이 하는 것
//   상수가 박힌 그 자리(사이트)를 `E9 rel32` 로 우리 스텁에 보내고, 스텁이
//   ①그 시점 레지스터에 살아 있는 **self 엔티티**로 클래스를 판정 → ②`tune()` 으로 값을 뽑아
//   ③원본 명령과 **의미가 같은 명령**을 그 값으로 실행한 뒤 ④제자리로 돌아온다.
//   재현(폐포 전체 Rust 이식)이 아니라 **상수 한 자리만 동적화**하는 것이라 비용이 극히 작다.
//   근거·설계 = `REPORT\tfm2_ai_adjust\RE\2026-08-07_클래스별노브_확장가능성.md` 부록 B.
//
// ⚠안전 규칙 (이 파일을 고칠 때 반드시 지킬 것)
//   ① self 판정은 **포인터인지 스칼라인지** 확인된 사이트만 등록한다(부록 D 오탐 사고: `add rX,0x78` 의
//      rX 는 엔티티가 아니라 "마지막 목격 틱" 스칼라였다). 등록 근거 없는 사이트 추가 금지.
//   ② 창(window)은 원본 바이트 전수 대조 후에만 덮어쓴다. 한 바이트라도 다르면 설치하지 않는다(패치 버전 방어).
//   ③ 창 안으로 **뛰어드는 분기 타깃이 없어야** 한다(RE 로 확인한 사이트만 등록).
//   ④ 스텁은 volatile 정수 8종 + xmm0~5 + (필요시)플래그를 전부 보존한다. 게임은 사이트 시점에
//      그 레지스터들에 살아있는 값을 갖고 있을 수 있다.
//   ⑤ 설치는 **1회 확정**. 해제·재설치를 매프레임 하면 실행 중 스레드와 경쟁한다(§3 상호 체인 사고와 같은 계열).
//      값 자체는 콜백이 매번 `tune()` 으로 읽으므로 cfg 를 고치면 재설치 없이 반영된다.

// ── 레지스터 코드 (x86-64) ──
#[allow(dead_code)] mod reg {
    pub const RAX: u8 = 0; pub const RCX: u8 = 1; pub const RDX: u8 = 2; pub const RBX: u8 = 3;
    pub const RSP: u8 = 4; pub const RBP: u8 = 5; pub const RSI: u8 = 6; pub const RDI: u8 = 7;
    pub const R8: u8 = 8;  pub const R9: u8 = 9;  pub const R10: u8 = 10; pub const R11: u8 = 11;
    pub const R12: u8 = 12; pub const R13: u8 = 13; pub const R14: u8 = 14; pub const R15: u8 = 15;
}

/// 원본 명령을 "값을 스택에서 읽는 같은 의미의 명령"으로 바꾸는 방식.
/// ★플래그: 원본이 플래그를 **만드는** 명령(Add/Cmp/Imul)이면 우리 명령도 똑같이 만들므로 보존 불필요.
///   원본이 플래그를 **안 건드리는** 명령(Mov/Lea)이면 우리가 pushfq/popfq 로 그대로 넘겨줘야 한다.
#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub(crate) enum MOp {
    /// `mov r32, imm32` → `mov r32, [rsp]`
    MovR32 { dst: u8 },
    /// `add r64, imm` → `add r64, [rsp]`
    AddR64 { dst: u8 },
    /// `cmp r64, imm32` → `cmp r64, [rsp]`
    CmpR64 { dst: u8 },
    /// `imul r64, r64, imm32` → (`mov dst,src` +) `imul dst, [rsp]`
    ImulR64 { dst: u8, src: u8 },
    /// `lea dst,[...+disp32]` → pre(=disp 뺀 lea) + `add dst, [rsp]`  ※원본이 플래그를 안 만드므로 보존 필요
    LeaAdd { dst: u8 },
}

impl MOp {
    /// 원본이 플래그를 만들지 않는 명령인가(= pushfq/popfq 로 넘겨줘야 하는가).
    fn preserve_flags(&self) -> bool {
        matches!(self, MOp::MovR32 { .. } | MOp::LeaAdd { .. })
    }
}

/// 사이트 시점에 값을 어디서 읽어올지. **스텁이 게임 레지스터를 건드리기 전에** 읽는다.
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(crate) enum Src {
    /// 없음(두 번째 인자를 안 쓸 때).
    None,
    /// 레지스터 값 그대로.
    Reg(u8),
    /// `[reg + disp]` 메모리 읽기.
    Mem(u8, i32),
    /// `[사이트 진입 시점 rsp + disp]` — 스텁이 쌓은 만큼을 자동 보정한다.
    Stack(i32),
}

/// 인자 두 개(a, b)로 self 엔티티를 어떻게 만드는가.
#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub(crate) enum Resolve {
    /// a 가 곧 self 엔티티.
    Direct,
    /// a = athlete/판단 컨텍스트(side=`[a+0x810]`, role=`[a+0x8a0]`), b = champions 홀더.
    /// self = `[b + side*0x28 + role*8 + 0x1e0]` — `slot_in_world` 와 같은 로스터 배치.
    Champions,
    /// ★사이트에서 self 가 안 잡히는 자리용 — **함수 진입부 훅**이 미리 담아둔 값을 쓴다(`ENTRY_SELF[n]`).
    ///   사이트에 도달했다는 것 자체가 "그 함수 안"이라는 뜻이므로, 그 스레드의 최근 진입값이 곧 이 판단의 self 다.
    ///   ⚠전제 = 그 함수가 **재귀·재진입하지 않을 것**(안쪽 프레임이 끝난 뒤 바깥 사이트가 안쪽 값을 볼 수 있다).
    ///   등록 전 RE 로 재귀 여부를 확인한다.
    FromEntry(usize),
}

/// 마이크로 디투어 사이트 1개.
pub(crate) struct MicroSite {
    /// cfg 키(= 이 상수의 노브 이름). `{key}_class_<cls>` 가 있을 때만 설치한다.
    pub key: &'static str,
    /// 게임 원본 값(클래스 오버라이드가 없을 때 그대로 돌려줄 값 = 동작 불변 보장).
    pub orig: i64,
    /// 사이트 RVA(창의 시작).
    pub rva: usize,
    /// 창 전체의 원본 바이트. 길이 = 창 길이(≥5). **전수 일치할 때만 설치**.
    pub win: &'static [u8],
    /// 값 op 앞에 그대로 실행할 바이트(위치 무관 명령만. 예: disp 를 뺀 lea).
    pub pre: &'static [u8],
    /// 값 op 뒤에 그대로 실행할 바이트(창이 원본 명령보다 길 때의 나머지. 위치 무관 명령만).
    pub tail: &'static [u8],
    /// 값 op.
    pub op: MOp,
    /// 창 안에서 **상수(imm/disp) 필드가 시작하는 오프셋**과 폭.
    /// ★원본 대조에서 이 구간은 **건너뛴다** — 기존 imm 패치가 먼저 값을 써 놓았을 수 있는데,
    ///   우리는 명령 전체를 대체하며 값을 직접 공급하므로 그 자리의 현재 값은 아무 상관이 없다.
    ///   (이걸 안 건너뛰면 "전역값을 튜닝한 유저"에게만 설치가 조용히 실패한다.)
    pub imm_off: usize,
    pub imm_w: usize,
    /// self 를 만들 첫 번째 재료(RE 로 생존 확인된 것만).
    pub a: Src,
    /// 두 번째 재료(`Resolve::Champions` 일 때 champions 홀더).
    pub b: Src,
    /// a·b 를 self 로 바꾸는 방법.
    pub resolve: Resolve,
    /// 사람이 읽는 근거 한 줄(RE 문서의 어느 판정인지).
    pub note: &'static str,
}

pub(crate) const MICRO_MAX: usize = 24;
pub(crate) const ENTRY_MAX: usize = 8;

// ════════════════════════════════════════════════════════════════════════════
//  함수 진입부 훅 — "사이트에서 self 가 안 잡히는" 자리를 여는 두 번째 열쇠
//
//  ★왜 필요한가: 상수가 박힌 자리에 판단 주체가 항상 살아 있는 건 아니다(레지스터가 이미 덮였거나,
//    self 로드가 그 자리보다 뒤에 있거나). 그런 자리는 사이트만 봐서는 "누구의 값이냐"를 못 정한다.
//    그러나 **함수에 들어올 때** self 를 알 수 있다면, 그 함수 안 어느 자리든 답이 정해진다.
//    모드가 이미 판단 진입부에서 `CUR_CLASS` 를 세우는 것(`pos_enter_*`)과 같은 발상이다.
//
//  ★수명: 스레드별 슬롯에 담고 **복원하지 않는다**. 사이트에 도달했다는 건 그 함수 안이라는 뜻이고,
//    그 함수에 들어오려면 반드시 이 훅을 지나므로 최근값이 곧 정답이다(리턴 훅이 필요 없다).
//    ⚠단 **재귀 함수면 이 논리가 깨진다** — 등록 전 RE 로 확인한다.
// ════════════════════════════════════════════════════════════════════════════

/// 함수 진입부 훅 1개.
pub(crate) struct ClassEntrySite {
    /// 진단용 이름(보통 이 함수가 담당하는 판단).
    pub name: &'static str,
    /// 함수 진입 RVA.
    pub rva: usize,
    /// 옮겨 실행할 프롤로그 바이트(≥12, rip-상대·분기 없음). **전수 일치할 때만 설치**.
    pub prologue: &'static [u8],
    /// self 재료(진입 시점 레지스터/스택 기준. `Src::Stack` 은 5번째 인자 = `[rsp+0x28]`).
    pub a: Src,
    pub b: Src,
    pub resolve: Resolve,
    /// 이 함수 안에서 열리는 노브들 — 그중 하나라도 `_class_` 오버라이드가 있어야 설치한다.
    pub knobs: &'static [&'static str],
    pub note: &'static str,
}

// 스레드별 "지금 이 함수가 처리 중인 self 엔티티". Cell = 소멸자 없음 ⟹ 디투어에서 접근 안전(§3).
thread_local! {
    static ENTRY_SELF: [std::cell::Cell<usize>; ENTRY_MAX] =
        [const { std::cell::Cell::new(0) }; ENTRY_MAX];
}
#[inline] fn entry_self_get(i: usize) -> usize {
    if i >= ENTRY_MAX { return 0; }
    ENTRY_SELF.with(|s| s[i].get())
}
#[inline] fn entry_self_set(i: usize, v: usize) {
    if i < ENTRY_MAX { ENTRY_SELF.with(|s| s[i].set(v)); }
}

static ENTRY_TAKEN: [AtomicBool; ENTRY_MAX] = [const { AtomicBool::new(false) }; ENTRY_MAX];
static ENTRY_HITS: [AtomicU64; ENTRY_MAX] = [const { AtomicU64::new(0) }; ENTRY_MAX];
static ENTRY_RESOLVED: [AtomicU64; ENTRY_MAX] = [const { AtomicU64::new(0) }; ENTRY_MAX];   // self 를 실제로 구한 횟수

/// 진입 훅 스텁이 부르는 콜백. self 를 계산해 스레드 슬롯에 담아둔다.
unsafe extern "C" fn class_entry_set(idx: u64, a: usize, b: usize) {
    let i = idx as usize;
    if i >= ENTRY_MAX || i >= ENTRY_SITES.len() { return; }
    ENTRY_HITS[i].fetch_add(1, Ordering::Relaxed);
    // ★패닉이 게임 콜스택으로 새면 UB(§3). 실패하면 0(=미상)으로 두면 전역값 폴백이라 동작 불변.
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let selfe = resolve_self(ENTRY_SITES[i].resolve, a, b);
        entry_self_set(i, selfe);
        if selfe != 0 { ENTRY_RESOLVED[i].fetch_add(1, Ordering::Relaxed); }
    }));
}

/// 재료 → self 엔티티. ★전부 fault-safe read — 스테일 포인터여도 0(전역 폴백)이지 크래시가 아니다.
unsafe fn resolve_self(r: Resolve, a: usize, b: usize) -> usize {
    match r {
        Resolve::Direct => a,
        Resolve::FromEntry(n) => entry_self_get(n),
        Resolve::Champions => {
            if !ptr_ok(a) || !ptr_ok(b) { return 0; }
            let side = rd_i64(a + 0x930).unwrap_or(-1);
            let role = rd_i64(a + 0x9c0).unwrap_or(-1) & 0xffff_ffff;   // dword 필드
            if !(0..2).contains(&side) || !(0..5).contains(&role) { return 0; }
            rd_u64(b + side as usize * 0x28 + role as usize * 8 + 0x1e0).unwrap_or(0) as usize
        }
    }
}

// ── 설치 상태 / 진단 ──
static MICRO_INSTALLED: AtomicUsize = AtomicUsize::new(0);      // 설치 성공 사이트 수
static MICRO_TRIED: AtomicUsize = AtomicUsize::new(0);          // 설치 시도 수
static MICRO_DONE: AtomicBool = AtomicBool::new(false);         // 1회 확정(§ 안전규칙 ⑤)
static MICRO_HITS: [AtomicU64; MICRO_MAX] = [const { AtomicU64::new(0) }; MICRO_MAX];   // 콜백 발화 수
static MICRO_OVHIT: [AtomicU64; MICRO_MAX] = [const { AtomicU64::new(0) }; MICRO_MAX];  // 클래스 전용값이 실제 적용된 수
// 값 캐시: [사이트][클래스 0..4 / 5=미상] — cfg 세대가 바뀌면 통째로 무효화.
static MICRO_VAL: [[AtomicI64; 6]; MICRO_MAX] =
    [const { [const { AtomicI64::new(i64::MIN) }; 6] }; MICRO_MAX];
static MICRO_VAL_GEN: AtomicU64 = AtomicU64::new(u64::MAX);
// 그 값이 `_class_` 키에서 온 것인가(= 클래스 전용값). 값 자체가 원본과 같아도 참일 수 있고,
// 전역 튜닝으로 원본과 달라도 거짓이다 — 이 구분이 없으면 지표가 "전역 튜닝했다"를 "클래스별로 먹는다"로 오독한다.
static MICRO_CLSHIT: [[AtomicBool; 6]; MICRO_MAX] =
    [const { [const { AtomicBool::new(false) }; 6] }; MICRO_MAX];

/// 스텁이 호출하는 콜백. `idx`=사이트 번호, `a`·`b`=self 를 만들 재료(사이트별 규약).
/// 반환 = 그 자리에 쓸 값(클래스 오버라이드 없으면 원본값 = 동작 불변).
unsafe extern "C" fn class_micro_value(idx: u64, a: usize, b: usize) -> u64 {
    let i = idx as usize;
    if i >= MICRO_MAX || i >= MICRO_SITES.len() { return 0; }
    let site = &MICRO_SITES[i];
    MICRO_HITS[i].fetch_add(1, Ordering::Relaxed);
    // self 산출. ★전부 fault-safe read — 스테일 포인터면 0(전역 폴백)이 되고 크래시하지 않는다.
    let selfe = resolve_self(site.resolve, a, b);
    // ★패닉이 게임 콜스택으로 새면 UB(§3). 어떤 실패든 원본값으로 조용히 폴백한다.
    let (v, slot) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // cfg 세대가 바뀌었으면 캐시 무효화(한 스레드만 수행).
        let gen = CFG_GEN.load(Ordering::Relaxed);
        if MICRO_VAL_GEN.load(Ordering::Relaxed) != gen {
            if MICRO_VAL_GEN.swap(gen, Ordering::Relaxed) != gen {
                for s in MICRO_VAL.iter() { for c in s.iter() { c.store(i64::MIN, Ordering::Relaxed); } }
            }
        }
        let cls = class_from_entity(selfe);          // -1 = 미상(전역값 폴백)
        let slot = if (0..5).contains(&cls) { cls as usize } else { 5 };
        let c = MICRO_VAL[i][slot].load(Ordering::Relaxed);
        if c != i64::MIN { return (c, slot); }   // ★캐시 히트. 적용 카운트는 이 클로저 **밖**에서 센다
        // 캐시 미스: CUR_CLASS 를 세워 tune() 이 클래스별 값을 보게 한다(판단 문맥 재현).
        let prev = cur_class();
        set_cur_class(cls);
        // ★"이 값이 `_class_` 키에서 온 것인가"를 따로 본다. `값 != 원본` 으로 세면 **전역 튜닝까지**
        //   클래스 적용으로 잡힌다(첫 검증에서 `ex_think_min` 이 전역 300 때문에 100%로 찍혔다).
        //   지표가 묻는 질문은 "클래스별 값이 실제로 쓰이는가" 이므로 출처를 봐야 한다.
        let cls_specific = tune_class_lookup(site.key).is_some();
        let raw = tune(site.key, site.orig);
        set_cur_class(prev);
        // ★프로젝트 공통 규약 "-1 = 원본 유지"([[tfm2-knob-default-minus1-rule]]).
        //   기존 바이트패치는 `b1/b4(v, orig)` 가 이 변환을 해줬는데, 콜백은 tune() 을 직접 읽으므로
        //   여기서 같은 변환을 해야 한다. 빠뜨리면 `-1` 이 그대로 상수로 박혀 동작이 망가진다
        //   (예: `cs_lead_attack = -1` 은 "원본 30" 인데 사거리 판정에 0xFFFFFFFF 가 들어간다).
        // ★상한 클램프 — 이 값들 중 일부는 게임에서 **틱에 더해진다**(`마지막목격틱 + 120`).
        //   터무니없이 큰 값을 넣으면 덧셈이 넘쳐 부호가 뒤집히고 판정이 **정반대**가 된다.
        //   원본 imm8/imm32 자리는 애초에 그럴 수 없었지만, 마이크로 디투어는 64비트 슬롯이라 가능해졌다
        //   — 표현력이 늘면 넣을 수 있는 잘못된 값도 는다.
        let v = if raw < 0 { site.orig } else { raw.min(1_000_000_000) };
        MICRO_CLSHIT[i][slot].store(cls_specific, Ordering::Relaxed);
        MICRO_VAL[i][slot].store(v, Ordering::Relaxed);
        (v, slot)
    })).unwrap_or((site.orig, 5));
    // ★적용 카운트는 **매 호출** 센다. 캐시 미스 때만 세면 클래스당 1회씩(사이트당 최대 5)만 잡혀
    //   "이 값이 실제로 쓰이고 있는가"를 답하지 못한다 — 08-06 사고의 교훈이 정확히 그것이었다
    //   ("설정했다 ≠ 먹는다"를 말해주는 지표가 없었다).
    if MICRO_CLSHIT[i][slot].load(Ordering::Relaxed) { MICRO_OVHIT[i].fetch_add(1, Ordering::Relaxed); }
    v as u64
}

// ── 명령 인코더(값은 항상 `[rsp]` 에서 읽는다) ──
#[inline] fn rex(w: bool, r: u8, b: u8) -> Option<u8> {
    let v = 0x40 | ((w as u8) << 3) | (((r >= 8) as u8) << 2) | ((b >= 8) as u8);
    if v == 0x40 { None } else { Some(v) }
}
/// modrm(mod=00, reg, rm=100) + SIB(0x24) = `[rsp]` 메모리 오퍼랜드.
#[inline] fn mem_rsp(reg: u8) -> [u8; 2] { [(reg & 7) << 3 | 0x04, 0x24] }

/// self 재료 하나를 `dst` 레지스터로 읽어온다. `delta` = 이 시점까지 스텁이 rsp 를 내린 양
/// (`Src::Stack` 이 "사이트 진입 시점 rsp" 기준이므로 보정해야 한다).
fn emit_load(out: &mut Vec<u8>, dst: u8, src: Src, delta: i32) {
    match src {
        Src::None => {                                  // xor dst,dst  (0 = 안 씀)
            out.push(rex(true, dst, dst).unwrap());
            out.push(0x31); out.push(0xc0 | ((dst & 7) << 3) | (dst & 7));
        }
        Src::Reg(r) => {                                // mov dst, r
            out.push(rex(true, r, dst).unwrap());
            out.push(0x89); out.push(0xc0 | ((r & 7) << 3) | (dst & 7));
        }
        Src::Mem(r, disp) => emit_mem_load(out, dst, r, disp),
        // rsp 상대는 SIB 가 필수라 같은 경로로 처리(base=rsp).
        Src::Stack(disp) => emit_mem_load(out, dst, reg::RSP, disp + delta),
    }
}

/// `mov dst, [base + disp32]` (mod=10). base 가 rsp/r12 면 SIB 필요.
fn emit_mem_load(out: &mut Vec<u8>, dst: u8, base: u8, disp: i32) {
    out.push(rex(true, dst, base).unwrap());
    out.push(0x8b);
    out.push(0x80 | ((dst & 7) << 3) | (base & 7));      // mod=10, rm=base
    if base & 7 == 4 { out.push(0x24); }                 // SIB: scale=0 index=none base=rm
    out.extend_from_slice(&disp.to_le_bytes());
}

fn emit_value_op(out: &mut Vec<u8>, op: MOp) {
    match op {
        MOp::MovR32 { dst } => {                       // mov r32, [rsp]
            if let Some(r) = rex(false, dst, 0) { out.push(r); }
            out.push(0x8b); out.extend_from_slice(&mem_rsp(dst));
        }
        MOp::AddR64 { dst } => {                       // add r64, [rsp]
            out.push(rex(true, dst, 0).unwrap());
            out.push(0x03); out.extend_from_slice(&mem_rsp(dst));
        }
        MOp::CmpR64 { dst } => {                       // cmp r64, [rsp]
            out.push(rex(true, dst, 0).unwrap());
            out.push(0x3b); out.extend_from_slice(&mem_rsp(dst));
        }
        MOp::ImulR64 { dst, src } => {
            if dst != src {                            // mov dst, src
                out.push(rex(true, src, dst).unwrap());
                out.push(0x89); out.push(0xc0 | ((src & 7) << 3) | (dst & 7));
            }
            out.push(rex(true, dst, 0).unwrap());      // imul dst, [rsp]
            out.push(0x0f); out.push(0xaf); out.extend_from_slice(&mem_rsp(dst));
        }
        MOp::LeaAdd { dst } => {                       // add dst, [rsp]  (pre 에서 disp 뺀 lea 를 이미 실행)
            out.push(rex(true, dst, 0).unwrap());
            out.push(0x03); out.extend_from_slice(&mem_rsp(dst));
        }
    }
}

/// 사이트 1개의 스텁 코드를 만든다. 스택 배치(높은 주소 → 낮은 주소):
///   `[플래그(선택)] [값 슬롯 8B] [rax rcx rdx r8 r9 r10 r11 rbp = 64B] [xmm0~5 = 96B]`
/// rbp = xmm 영역의 바닥(앵커). 값 슬롯 = `rbp + 0xA0`.
fn build_micro_stub(idx: usize, site: &MicroSite, ret_addr: usize, cb: usize) -> Vec<u8> {
    let mut s: Vec<u8> = Vec::with_capacity(192);
    let pf = site.op.preserve_flags();
    if pf { s.push(0x9c); }                                        // pushfq
    s.extend_from_slice(&[0x48, 0x8d, 0x64, 0x24, 0xf8]);          // lea rsp,[rsp-8]   (값 슬롯. LEA=플래그 무영향)
    // volatile 정수 8종 보존
    for b in [0x50u8, 0x51, 0x52, 0x41, 0x50, 0x41, 0x51, 0x41, 0x52, 0x41, 0x53, 0x55] { s.push(b); }
    //         push rax  rcx   rdx   push r8     push r9     push r10    push r11    push rbp
    s.extend_from_slice(&[0x48, 0x8d, 0xa4, 0x24, 0xa0, 0xff, 0xff, 0xff]);   // lea rsp,[rsp-0x60]
    for k in 0..6u8 {                                              // movups [rsp+k*16], xmm{k}
        s.extend_from_slice(&[0x0f, 0x11, 0x44 | ((k & 7) << 3), 0x24, (k * 16)]);
    }
    // ★self 재료를 **rbp 클로버 전에** 읽는다(재료가 rbp 이거나 rbp 상대인 경우가 실제로 있다).
    //   이 시점 rsp = 진입시 rsp − delta 이므로 Stack(disp) 는 그만큼 보정한다.
    let delta = 0xa8i32 + if pf { 8 } else { 0 };
    emit_load(&mut s, reg::RDX, site.a, delta);
    emit_load(&mut s, reg::R8, site.b, delta);
    s.extend_from_slice(&[0x48, 0x89, 0xe5]);                      // mov rbp, rsp   (앵커)
    s.extend_from_slice(&[0x48, 0x8d, 0x64, 0x24, 0xd0]);          // lea rsp,[rsp-0x30]
    s.extend_from_slice(&[0x48, 0x83, 0xe4, 0xf0]);                // and rsp,-16    (호출 규약 16정렬)
    s.push(0xb9); s.extend_from_slice(&(idx as u32).to_le_bytes()); // mov ecx, idx
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&cb.to_le_bytes()); // movabs rax, cb
    s.extend_from_slice(&[0xff, 0xd0]);                            // call rax → rax = 값
    s.extend_from_slice(&[0x48, 0x89, 0x85]); s.extend_from_slice(&0xa0u32.to_le_bytes()); // mov [rbp+0xa0], rax
    s.extend_from_slice(&[0x48, 0x89, 0xec]);                      // mov rsp, rbp
    for k in 0..6u8 {                                              // movups xmm{k}, [rsp+k*16]
        s.extend_from_slice(&[0x0f, 0x10, 0x44 | ((k & 7) << 3), 0x24, (k * 16)]);
    }
    s.extend_from_slice(&[0x48, 0x8d, 0x64, 0x24, 0x60]);          // lea rsp,[rsp+0x60]
    for b in [0x5du8, 0x41, 0x5b, 0x41, 0x5a, 0x41, 0x59, 0x41, 0x58, 0x5a, 0x59, 0x58] { s.push(b); }
    //         pop rbp   pop r11     pop r10     pop r9      pop r8      rdx   rcx   rax
    // 여기서 rsp = 값 슬롯.
    s.extend_from_slice(site.pre);
    emit_value_op(&mut s, site.op);
    s.extend_from_slice(site.tail);
    s.extend_from_slice(&[0x48, 0x8d, 0x64, 0x24, 0x08]);          // lea rsp,[rsp+8]   (값 슬롯 반환. 플래그 무영향)
    if pf { s.push(0x9d); }                                        // popfq  (원본이 안 건드리던 플래그 복원)
    s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]);    // jmp [rip+0]
    s.extend_from_slice(&ret_addr.to_le_bytes());                  // .quad 복귀주소
    s
}

/// 진입 훅 스텁: self 를 슬롯에 담고 → 원본 프롤로그 실행 → 함수 본문으로 복귀.
/// 스택 배치는 사이트 스텁과 같되 **값 슬롯이 없다**(반환값을 쓰지 않으므로).
fn build_entry_stub(idx: usize, site: &ClassEntrySite, ret_addr: usize, cb: usize) -> Vec<u8> {
    let mut s: Vec<u8> = Vec::with_capacity(224);
    // 함수 진입 직후의 플래그는 규약상 죽은 값이지만, 프롤로그가 무엇을 하든 원본과 같게 보이도록 보존한다.
    s.push(0x9c);                                                  // pushfq
    for b in [0x50u8, 0x51, 0x52, 0x41, 0x50, 0x41, 0x51, 0x41, 0x52, 0x41, 0x53, 0x55] { s.push(b); }
    //         push rax  rcx   rdx   r8          r9          r10         r11         rbp
    s.extend_from_slice(&[0x48, 0x8d, 0xa4, 0x24, 0xa0, 0xff, 0xff, 0xff]);   // lea rsp,[rsp-0x60]
    for k in 0..6u8 { s.extend_from_slice(&[0x0f, 0x11, 0x44 | ((k & 7) << 3), 0x24, k * 16]); }
    // ★재료는 게임 레지스터를 건드리기 전에 읽는다. 이 시점 rsp = 진입 rsp − delta.
    let delta = 8i32 + 64 + 0x60;                                  // 플래그 + 8푸시 + xmm
    emit_load(&mut s, reg::RDX, site.a, delta);
    emit_load(&mut s, reg::R8, site.b, delta);
    s.extend_from_slice(&[0x48, 0x89, 0xe5]);                      // mov rbp, rsp   (앵커)
    s.extend_from_slice(&[0x48, 0x8d, 0x64, 0x24, 0xd0]);          // lea rsp,[rsp-0x30]
    s.extend_from_slice(&[0x48, 0x83, 0xe4, 0xf0]);                // and rsp,-16
    s.push(0xb9); s.extend_from_slice(&(idx as u32).to_le_bytes()); // mov ecx, idx
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&cb.to_le_bytes());
    s.extend_from_slice(&[0xff, 0xd0]);                            // call rax
    s.extend_from_slice(&[0x48, 0x89, 0xec]);                      // mov rsp, rbp
    for k in 0..6u8 { s.extend_from_slice(&[0x0f, 0x10, 0x44 | ((k & 7) << 3), 0x24, k * 16]); }
    s.extend_from_slice(&[0x48, 0x8d, 0x64, 0x24, 0x60]);          // lea rsp,[rsp+0x60]
    for b in [0x5du8, 0x41, 0x5b, 0x41, 0x5a, 0x41, 0x59, 0x41, 0x58, 0x5a, 0x59, 0x58] { s.push(b); }
    s.push(0x9d);                                                  // popfq
    s.extend_from_slice(site.prologue);                            // 옮겨온 원본 프롤로그
    s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]);    // jmp [rip+0]
    s.extend_from_slice(&ret_addr.to_le_bytes());
    s
}

/// 진입 훅 설치. 프롤로그 전수 대조 후 `E9 rel32` 로 스텁에 보낸다.
unsafe fn install_class_entries(base: usize, cb: usize, report: &mut String) {
    if ENTRY_SITES.is_empty() { return; }
    report.push_str("\n[함수 진입부 훅] — 사이트에서 self 가 안 잡히는 자리를 여는 열쇠\n");
    for (i, site) in ENTRY_SITES.iter().enumerate() {
        if i >= ENTRY_MAX { break; }
        if !site.knobs.iter().any(|k| has_class_override(k)) {
            report.push_str(&format!("  skip  {:<20} 이 함수의 노브에 클래스 오버라이드 없음\n", site.name));
            continue;
        }
        let addr = base + site.rva;
        let n = site.prologue.len();
        if n < 5 { report.push_str(&format!("  FAIL  {:<20} 프롤로그가 5바이트 미만\n", site.name)); continue; }
        if !readable(addr, n) { report.push_str(&format!("  FAIL  {:<20} 읽기 불가\n", site.name)); continue; }
        let mut ok = true;
        for (k, &b) in site.prologue.iter().enumerate() { if rd_u8(addr + k) != b { ok = false; break; } }
        if !ok {
            report.push_str(&format!("  BLOCK {:<20} rva={:#x} 프롤로그 불일치 — 게임이 바뀌었다\n", site.name, site.rva));
            continue;
        }
        let code = build_entry_stub(i, site, addr + n, cb);
        let stub = micro_alloc(addr, code.len());
        if stub == 0 { report.push_str(&format!("  FAIL  {:<20} 스텁 할당 실패\n", site.name)); continue; }
        let rel = stub as i64 - (addr as i64 + 5);
        if !(-0x7fff_0000..=0x7fff_0000).contains(&rel) {
            report.push_str(&format!("  FAIL  {:<20} rel32 범위 밖\n", site.name)); continue;
        }
        core::ptr::copy_nonoverlapping(code.as_ptr(), stub as *mut u8, code.len());
        FlushInstructionCache(GetCurrentProcess(), stub, code.len());
        let mut e9 = [0x90u8; 5];
        e9[0] = 0xe9; e9[1..5].copy_from_slice(&(rel as i32).to_le_bytes());
        let mut old: u32 = 0;
        if VirtualProtect(addr, n, 0x40, &mut old) == 0 {
            report.push_str(&format!("  FAIL  {:<20} VirtualProtect 실패\n", site.name)); continue;
        }
        core::ptr::copy_nonoverlapping(e9.as_ptr(), addr as *mut u8, 5);
        VirtualProtect(addr, n, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), addr, n);
        ENTRY_TAKEN[i].store(true, Ordering::Relaxed);
        report.push_str(&format!("  OK    {:<20} rva={:#x} 프롤로그 {}B 스텁={:#x}  {}\n",
                                 site.name, site.rva, n, stub, site.note));
    }
}

/// cfg 에 이 노브의 클래스 오버라이드가 하나라도 있는가.
fn has_class_override(key: &str) -> bool {
    let p = TUNE_PTR.load(Ordering::Acquire);
    if p.is_null() { return false; }
    let m: &TuneMap = unsafe { &*p };
    CLASS_NAMES.iter().any(|n| m.contains_key(&format!("{}_class_{}", key, n)))
}

/// 마이크로 디투어 설치(1회 확정). 클래스 오버라이드가 걸린 사이트만 설치한다.
/// ★기존 imm 패치와 **상호배타**: 설치된 사이트는 `micro_taken()` 이 참을 돌려주어 apply_* 가 건드리지 않는다.
pub(crate) unsafe fn install_class_micro() {
    // ★[08-07] 게이트 계측 — 첫 인게임 확인에서 `class_micro.txt` 가 **아예 안 생겨서** 어느 관문에서
    //   되돌아갔는지 알 수 없었다. 조용한 조기반환은 진단이 불가능하다(08-06 교훈 "조용한 무시가 가장 비싸다").
    //   ⚠단 이 함수는 **retreat 훅 = 핫패스**에서 불린다(프레임당 여러 번). 매번 쓰면 핫패스 파일 IO 금지
    //   규칙 위반이다(08-07 `ct_hunt` 가 정확히 그래서 한 판에 20MB 를 썼다) ⟹ **사유가 바뀔 때만** 쓴다.
    let ready = READY_TICKS.load(Ordering::Relaxed);
    let base = exe_base();
    let tune_null = TUNE_PTR.load(Ordering::Acquire).is_null();
    let done = MICRO_DONE.load(Ordering::Relaxed);
    macro_rules! bail { ($code:expr, $why:expr) => {{
        if MICRO_BAIL_CODE.swap($code, Ordering::Relaxed) != $code {   // 사유가 바뀐 첫 1회만 기록
            if let Some(p) = pth("class_micro.txt") {
                let _ = fs::write(p, format!(
                    "=== 클래스별 마이크로 디투어 — 설치 보류 ===\n\
                     사유: {}\n\n\
                     게이트 값: ready_ticks={} (필요 {}) · exe_base={:#x} · tune_table={} · 설치완료플래그={}\n\
                     사이트 표에 등록된 노브 {}개.\n\
                     ※ 이 파일이 '보류'로 남아 있으면 클래스별 값은 안 먹고 기존 바이트패치가 그대로 걸린다.\n\
                     ※ ready_ticks 는 게임 프레임마다 1 오른다 — 아직 작으면 그냥 더 기다리면 된다.\n",
                    $why, ready, READY_MIN, base,
                    if tune_null { "아직 없음" } else { "있음" }, done, MICRO_SITES.len()));
            }
        }
        return;
    }}; }
    if done { return; }                       // 이미 처리됨 — 덮어쓰지 않는다(설치 결과 보존)
    // ★준비 대기는 **횟수로 재지 않는다.** 08-07 첫 수정에서 "재시도 600회 상한"을 뒀다가,
    //   이 함수가 프레임당 여러 번 불리는 바람에 `ready_ticks=23`(필요 200)에서 상한을 다 써버려
    //   설치가 또 통째로 누락됐다. `READY_TICKS` 는 프레임마다 단조 증가하므로 **그냥 기다리면 열린다**
    //   — 호출 횟수는 경과 시간의 척도가 아니다.
    if ready < READY_MIN { bail!(1, "게임이 아직 준비 전 — 프레임이 더 지나면 자동으로 열린다"); }
    if base == 0 { bail!(2, "exe base 를 못 구했다"); }
    // ★cfg 가 아직 안 올라왔으면 **다음 기회에**. 오버라이드 판정을 빈 테이블로 하면 "오버라이드 없음"으로
    //   오판해 영구 미설치가 된다. 이 경로는 실제로 밟혔다(첫 인게임 확인에서 설치가 통째로 누락) —
    //   `CFG_GEN` 은 cfg **파싱 시작**에 올라가는데 `tune_publish` 는 파싱 **끝**이라, 그 사이에
    //   체인이 돌면 여기서 튕긴다. 그래서 `micro_settled()` 로 체인이 다시 오게 만든다(아래).
    if tune_null {
        // ★여기만 상한을 둔다 — READY 가 이미 열렸는데도 테이블이 안 올라오는 건 정상 상황이 아니다
        //   (cfg 파일이 없거나 파싱이 계속 실패). 그때까지 체인을 영원히 재실행하지 않도록 끊는다.
        //   READY 이후로만 세므로 위 사고(준비 전에 상한 소진)는 재발하지 않는다.
        if MICRO_ATTEMPTS.fetch_add(1, Ordering::Relaxed) > 20_000 {
            MICRO_DONE.store(true, Ordering::Relaxed);
            bail!(4, "준비 후에도 cfg 튜닝 테이블이 끝내 게시되지 않았다(상한 초과)");
        }
        bail!(3, "cfg 튜닝 테이블이 아직 게시 전 — 곧 열린다");
    }
    MICRO_DONE.store(true, Ordering::Relaxed);

    let cb = class_micro_value as *const () as usize;
    let mut report = String::from("=== 클래스별 마이크로 디투어 설치 결과 ===\n\
        # 설치된 사이트만 클래스별 값이 먹는다. 값 변경은 재설치 없이 반영되지만,\n\
        # 설치/해제 자체는 게임 재시작이 필요하다(실행 중 코드 교체 경쟁 방지).\n");

    // ── 1차: 사이트별 "설치 가능한가" 판정 (아직 아무것도 쓰지 않는다) ──
    let mut feasible = [false; MICRO_MAX];
    let mut why: [&'static str; MICRO_MAX] = [""; MICRO_MAX];
    for (i, site) in MICRO_SITES.iter().enumerate() {
        if i >= MICRO_MAX { break; }
        if !has_class_override(site.key) { why[i] = "클래스 오버라이드 없음(기존 바이트패치 유지)"; continue; }
        let addr = base + site.rva;
        let n = site.win.len();
        if n < 5 { why[i] = "창이 5바이트 미만"; continue; }
        if !readable(addr, n) { why[i] = "읽기 불가"; continue; }
        // ★원본 대조(게임 패치 방어) — 단 **상수 필드는 건너뛴다**(위 imm_off/imm_w 주석 참조).
        //   명령의 골격(opcode·ModRM·레지스터)이 내가 아는 그대로일 때만 설치한다.
        let mut ok = true;
        for (k, &b) in site.win.iter().enumerate() {
            if k >= site.imm_off && k < site.imm_off + site.imm_w { continue; }   // 상수 자리 = 무시
            if rd_u8(addr + k) != b { ok = false; break; }
        }
        if !ok { why[i] = "명령 골격 불일치 — 이 자리는 내가 아는 그 자리가 아니다"; continue; }
        feasible[i] = true;
    }

    // ── 2차: **키 단위 all-or-nothing** ──
    // ★한 노브가 여러 사이트에 흩어져 있으면(`bt_vision_mem` = 11곳) 일부만 설치하는 순간
    //   **반쪽 노브**가 된다 — 어떤 경로는 클래스별, 어떤 경로는 전역. 그건 값이 안 먹는 것보다 나쁘다
    //   (증상이 경로에 따라 갈려 진단이 거의 불가능해진다). 전부 되거나, 하나도 안 하거나.
    let mut go = [false; MICRO_MAX];
    for (i, site) in MICRO_SITES.iter().enumerate() {
        if i >= MICRO_MAX || !feasible[i] { continue; }
        let all = MICRO_SITES.iter().enumerate()
            .filter(|(j, s)| *j < MICRO_MAX && s.key == site.key)
            .all(|(j, _)| feasible[j]);
        if all { go[i] = true; } else { why[i] = "같은 노브의 다른 사이트가 불가 — 반쪽 설치 방지로 전체 보류"; }
    }

    for (i, site) in MICRO_SITES.iter().enumerate() {
        if i >= MICRO_MAX { break; }
        if !go[i] {
            let tag = if feasible[i] || why[i].starts_with("클래스") { "skip " } else { "BLOCK" };
            report.push_str(&format!("{} {:<22} rva={:#x} {}\n", tag, site.key, site.rva, why[i]));
            continue;
        }
        MICRO_TRIED.fetch_add(1, Ordering::Relaxed);
        let addr = base + site.rva;
        let n = site.win.len();
        let stub_code = build_micro_stub(i, site, addr + n, cb);
        let stub = micro_alloc(addr, stub_code.len());
        if stub == 0 {
            report.push_str(&format!("FAIL  {:<22} 스텁 할당 실패(±2GB 내 여유 없음)\n", site.key));
            continue;
        }
        let rel = stub as i64 - (addr as i64 + 5);
        if !(-0x7fff_0000..=0x7fff_0000).contains(&rel) {
            report.push_str(&format!("FAIL  {:<22} 스텁이 rel32 범위 밖\n", site.key));
            continue;
        }
        core::ptr::copy_nonoverlapping(stub_code.as_ptr(), stub as *mut u8, stub_code.len());
        FlushInstructionCache(GetCurrentProcess(), stub, stub_code.len());
        // ★쓰는 것은 **앞 5바이트(E9 rel32)뿐**이다. 나머지(창이 5보다 길면 그 명령의 남은 오퍼랜드
        //   바이트)는 E9 가 자리잡는 순간 도달 불가가 되므로 NOP 로 덮을 필요가 없다.
        //   ⚠왜 굳이 안 덮나 = **실행 중인 스레드와의 경쟁을 줄이려고.** 게임은 이 함수들을 rayon 워커에서
        //   동시에 돌린다. 두 번 쓰면 그 사이에 낀 스레드가 "원본 첫 명령 + 반쯤 덮인 둘째 명령"을 실행할
        //   수 있다. 한 번만 쓰면, E9 직전에 원본 명령을 이미 페치한 스레드는 **원본 시퀀스를 그대로 완주**한다
        //   (뒷바이트가 손대지 않은 원본 그대로이기 때문). 남는 경쟁은 5바이트 단일 store 하나뿐이다.
        //   ⚠전제 = 창 안으로 뛰어드는 분기 타깃이 없어야 한다(사이트 등록 시 RE 로 확인).
        let mut e9 = [0x90u8; 5];
        e9[0] = 0xe9;
        e9[1..5].copy_from_slice(&(rel as i32).to_le_bytes());
        let mut old: u32 = 0;
        if VirtualProtect(addr, n, 0x40, &mut old) == 0 {
            report.push_str(&format!("FAIL  {:<22} VirtualProtect 실패\n", site.key));
            continue;
        }
        core::ptr::copy_nonoverlapping(e9.as_ptr(), addr as *mut u8, 5);
        VirtualProtect(addr, n, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), addr, n);
        MICRO_TAKEN[i].store(true, Ordering::Relaxed);
        MICRO_INSTALLED.fetch_add(1, Ordering::Relaxed);
        report.push_str(&format!("OK    {:<22} rva={:#x} 창={}B 스텁={:#x}  {}\n",
                                 site.key, site.rva, n, stub, site.note));
    }
    report.push_str(&format!("\n설치 {}/{} (시도 {})\n",
        MICRO_INSTALLED.load(Ordering::Relaxed), MICRO_SITES.len(), MICRO_TRIED.load(Ordering::Relaxed)));
    // ★진입부 훅은 사이트 설치 **뒤에** 건다 — 순서 자체는 무관하지만(다른 주소), 보고서를 한 파일에 모은다.
    install_class_entries(base, class_entry_set as *const () as usize, &mut report);
    if let Some(p) = pth("class_micro.txt") { let _ = fs::write(p, report); }
}

static MICRO_TAKEN: [AtomicBool; MICRO_MAX] = [const { AtomicBool::new(false) }; MICRO_MAX];
static MICRO_ATTEMPTS: AtomicU64 = AtomicU64::new(0);
static MICRO_BAIL_CODE: AtomicU8 = AtomicU8::new(0);   // 마지막으로 기록한 보류 사유(0=없음) — 같은 사유 반복 기록 차단

/// 설치 시도가 **끝났는가**(성공이든 실패든). 거짓이면 아직 준비 관문에서 튕기는 중이다.
/// ★apply 체인이 이걸 보고 "이번 세대 완료" 마킹을 미룬다 — 안 그러면 한 번 튕긴 설치가
///   `APPLY_GEN` 이 저장되는 순간 **영영 재시도되지 않는다**(첫 인게임 확인에서 실제로 그렇게 누락됐다).
#[inline] pub(crate) fn micro_settled() -> bool { MICRO_DONE.load(Ordering::Relaxed) }

// ── 스텁 전용 블록 할당기 ──
// `alloc_near` 를 사이트마다 부르면 사이트당 64KB(VirtualAlloc 예약 단위)를 잡고 스텁 인벤토리(STUB_MAX=24)도
// 그만큼 먹는다. 한 블록을 잡아 나눠 쓴다. 설치는 단일 스레드(APPLY_LOCK 아래)라 잠금 불요.
static MICRO_BLK: AtomicUsize = AtomicUsize::new(0);
static MICRO_BLK_USED: AtomicUsize = AtomicUsize::new(0);
const MICRO_BLK_SZ: usize = 0x1000;

unsafe fn micro_alloc(near: usize, size: usize) -> usize {
    let size = (size + 15) & !15;
    let mut blk = MICRO_BLK.load(Ordering::Relaxed);
    let used = MICRO_BLK_USED.load(Ordering::Relaxed);
    if blk == 0 || used + size > MICRO_BLK_SZ {
        blk = alloc_near(near, MICRO_BLK_SZ);
        if blk == 0 { return 0; }
        MICRO_BLK.store(blk, Ordering::Relaxed);
        MICRO_BLK_USED.store(0, Ordering::Relaxed);
    }
    let used = MICRO_BLK_USED.load(Ordering::Relaxed);
    MICRO_BLK_USED.store(used + size, Ordering::Relaxed);
    blk + used
}

/// 이 노브가 마이크로 디투어 표에 있는가(설치 성공 여부와 무관).
/// ★쓰임: cfg 로드 시점에 "이 클래스 오버라이드가 판단 재현을 필요로 하는가"를 판정한다.
///   마이크로 디투어는 게임 원본 코드 위에서 상수만 바꾸므로 **재현이 전혀 필요 없다** ⟹
///   `skip_untuned` 최적화를 끌 이유가 없다. 이 구분을 빼먹으면 08-06 재생 멈춤이 그대로 재발한다
///   (효과 없는 키 하나가 최적화를 통째로 끄던 그 사고 — `03_시행착오.md` 08-07 항목).
#[inline] pub(crate) fn is_micro_knob(key: &str) -> bool {
    MICRO_SITES.iter().any(|s| s.key == key)
}

/// 이 노브가 마이크로 디투어로 넘어갔는가 = 기존 imm 패치를 **하면 안 되는가**.
/// ⚠상호배타가 깨지면 imm 패치가 우리 `E9` 를 덮어써 게임이 엉뚱한 주소로 점프한다.
#[inline] pub(crate) fn micro_taken(key: &str) -> bool {
    for (i, s) in MICRO_SITES.iter().enumerate() {
        if i < MICRO_MAX && s.key == key { return MICRO_TAKEN[i].load(Ordering::Relaxed); }
    }
    false
}

/// 런타임 확인용 요약(class_verify=1 일 때 class_verify.txt 에 덧붙는다).
pub(crate) fn micro_summary() -> String {
    let mut s = String::from("\n[마이크로 디투어] 사이트별 발화 / 그중 클래스 전용값이 쓰인 횟수\n");
    for (i, site) in MICRO_SITES.iter().enumerate() {
        if i >= MICRO_MAX { break; }
        if !MICRO_TAKEN[i].load(Ordering::Relaxed) { continue; }
        let hit = MICRO_HITS[i].load(Ordering::Relaxed);
        let ov = MICRO_OVHIT[i].load(Ordering::Relaxed);
        let pct = if hit > 0 { ov * 100 / hit } else { 0 };
        s.push_str(&format!("  {:<22} 발화={:<10} 클래스전용값={:<10} ({}%)\n", site.key, hit, ov, pct));
    }
    s.push_str("  ※ '클래스전용값' = 그 판단이 cfg 의 `키_class_<클래스>` 값을 실제로 읽은 횟수.\n\
                \x20    전역값(클래스 구분 없는 값)만 튜닝한 경우는 여기 안 잡힌다 — 이 칸이 묻는 건\n\
                \x20    '클래스별로 갈라졌는가' 뿐이다. 0 이면 그 클래스 챔프가 안 나왔거나 키가 없는 것.\n");
    let mut any = false;
    for (i, e) in ENTRY_SITES.iter().enumerate() {
        if i >= ENTRY_MAX || !ENTRY_TAKEN[i].load(Ordering::Relaxed) { continue; }
        if !any { s.push_str("\n[함수 진입부 훅] 진입 / 그중 self 를 실제로 구한 횟수\n"); any = true; }
        let hit = ENTRY_HITS[i].load(Ordering::Relaxed);
        let ok = ENTRY_RESOLVED[i].load(Ordering::Relaxed);
        let pct = if hit > 0 { ok * 100 / hit } else { 0 };
        s.push_str(&format!("  {:<20} 진입={:<10} self확보={:<10} ({}%)\n", e.name, hit, ok, pct));
    }
    if any {
        s.push_str("  ※ self확보 0% = 재료 해석이 틀린 것(그 함수의 사이트는 전역값으로 폴백된다).\n");
    }
    s
}

// ════════════════════════════════════════════════════════════════════════════
//  사이트 표 — ★RE 로 self 생존이 확인된 자리만 넣는다(안전규칙 ①).
//  근거 = REPORT\tfm2_ai_adjust\RE\2026-08-07_클래스별노브_확장가능성.md 및 후속 RE.
// ════════════════════════════════════════════════════════════════════════════
//  ⛔불가 판정(재시도 전 RE 문서를 먼저 볼 것 — 범위 한정 판정이다):
//    · `ex_order_hold` 0xd7f963 — 교체창 4B(`add rax,0xa`) + 값이 **팀 오더** 단위(조건② 불만족).
//    · `sf_margin`     0xdb3f1b — 사이트 시점 self 미도달(self 로드가 뒤쪽 분기, role 소스 없음).
//      ★"**A 방식(사이트에서 self 즉시 복원) 한정** 불가" — self 신원 역산은 별도 심층 RE 로 열릴 수 있다.
//    · `bt_vision_mem` 11곳    — 로드값이 `[목격틱테이블 + 적side*0x2e8 + 대상role*8 + 0x1e0]` = **전역 목격
//      테이블의 스칼라**이지 self 가 아니다(부록 D 오탐 정정과 정합). 교체창도 4B.
macro_rules! bt_vis {
    ($rva:expr, $win:expr, $dst:expr, $tail:expr) => {
        MicroSite {
            key: "bt_vision_mem", orig: 120, rva: $rva,
            win: $win, pre: &[], tail: $tail,
            op: MOp::AddR64 { dst: $dst },
            imm_off: 3, imm_w: 1,
            a: Src::Mem(reg::RBP, 0x610), b: Src::Mem(reg::RBP, 0x560),
            resolve: Resolve::Champions,
            note: "self=champions[[rbp+0x560]][side,role of [rbp+ 0x628]]",
        }
    };
}

// ★★[0.5.6 재핀 2026-08-20, mig056_ai3(054→055 cosine/난수정렬→056)] — ⚠이 파일은 sites.py류 파서 밖이라 **0.5.5 마이그에서 통째 누락**돼
//   0.5.4 주소 그대로 침묵 비활성(win 검증 fail-safe) 상태였다(nexus_emg 0.5.5 선례와 동형). 이번에 16/18 복구:
//   · bt_vis 11 + cs_lead + mv2×3 + sf_margin = 0.5.6 신주소·win 바이트 일치·self 재료(프레임 disp) 정합 실측 완료.
//   · **미복구 2종(0.5.4 stale 유지=미설치 inert)**: ex_order_hold(재료 [rbp+0x110] arg4 스필이 0.5.5에서 소멸→센티널화 = self 재RE 필요)
//     / ex_think_min(0.5.5부터 명령 분해 lea+add: add rcx,0x190 @0.5.6 0xebcf2c — LeaAdd 창·재료(rdi/[rbp+0x2c8]) 재설계 필요).
pub(crate) static MICRO_SITES: &[MicroSite] = &[
    // ① cs_lead_attack — 평타 선행예측 틱. self 가 r14 에 그대로 살아 있는 가장 깨끗한 자리.
    //    0xeb25cb `mov r14,[rbx+rax*8]` = champions[myside][myrole] 이후 사이트까지 r14 write 없음.
    MicroSite {
        key: "cs_lead_attack", orig: 30, rva: 0xcd066a,
        win: &[0xb8, 0x1e, 0x00, 0x00, 0x00],            // mov eax,0x1e
        pre: &[], tail: &[],
        imm_off: 1, imm_w: 4,
        op: MOp::MovR32 { dst: reg::RAX },
        a: Src::Reg(reg::R14), b: Src::None, resolve: Resolve::Direct,
        note: "self=r14 생존(0xeb25cb 로드)",
    },
    // ② mv2_avoid_coef — 회피 반경 계수. fn 0xe587f0 의 5번째 인자(=champions[side][role])가 rbp 에 들어있다.
    MicroSite {
        key: "mv2_avoid_coef", orig: 400, rva: 0xd34c57,   // ★0.5.6(was 0.5.4 0xe58cf1 — ⚠2단계 apply 누락분 정정 2026-08-22: 054호스트 스켈레톤 UNIQUE→056·win 바이트 일치 확증)
        win: &[0x48, 0x69, 0xc1, 0x90, 0x01, 0x00, 0x00],  // imul rax,rcx,0x190
        pre: &[], tail: &[],
        imm_off: 3, imm_w: 4,
        op: MOp::ImulR64 { dst: reg::RAX, src: reg::RCX },
        a: Src::Reg(reg::RBP), b: Src::None, resolve: Resolve::Direct,
        note: "self=rbp 생존(0xd0a806 [rsp+0x180] 로드)",
    },
    // ③ mv2_avoid_margin — 회피 여유 상수. ②와 같은 경로·같은 rbp.
    MicroSite {
        key: "mv2_avoid_margin", orig: 6000, rva: 0xd34c9f,   // ★0.5.6(was 0.5.4 0xe58d45 — ⚠2단계 apply 누락분 정정 2026-08-22: 054호스트 스켈레톤 UNIQUE→056·win 바이트 일치 확증)
        win: &[0x48, 0x05, 0x70, 0x17, 0x00, 0x00],        // add rax,0x1770
        pre: &[], tail: &[],
        imm_off: 2, imm_w: 4,
        op: MOp::AddR64 { dst: reg::RAX },
        a: Src::Reg(reg::RBP), b: Src::None, resolve: Resolve::Direct,
        note: "self=rbp 생존(②와 동일 경로)",
    },
    // ④ mv2_avoid_bias — 편향 확정 임계. ★사이트 전에 rbp 가 덮여서(0xe58dc9·0xe58de9) self 가 아니다.
    //    5번째 인자 원본 슬롯 `[rsp+0x180]` 은 함수 내 write 가 없어 재로드로 복원한다.
    //    ⚠사이트가 `cmp` 라 **뒤 `ja` 가 플래그를 소비**한다 — 값 op(cmp)가 스텁의 마지막 플래그 생산자여야 한다.
    MicroSite {
        key: "mv2_avoid_bias", orig: 1500, rva: 0xd350ff,   // ★0.5.6(was 0.5.4 0xe5919f — ⚠2단계 apply 누락분 정정 2026-08-22: 054호스트 스켈레톤 UNIQUE→056·win 바이트 일치 확증)
        win: &[0x48, 0x3d, 0xdc, 0x05, 0x00, 0x00],        // cmp rax,0x5dc
        pre: &[], tail: &[],
        imm_off: 2, imm_w: 4,
        op: MOp::CmpR64 { dst: reg::RAX },
        a: Src::Stack(0x180), b: Src::None, resolve: Resolve::Direct,
        note: "self=[진입rsp+0x180] 재로드(rbp 는 덮임)",
    },
    // ⑥ sf_margin — "지금 이 대상을 치러 가도 안전한가"의 안전 여유(위협 반경에 얹는 값).
    //    ★지난 판정은 "사이트에서 self 미도달 = 불가"였는데 **오판**이었다 — self 를 `rdi`(=4번째 인자
    //    = 공격 **대상**)로 오인했다. 실제 self 는 **3번째 인자 p3**이고 프롤로그가 `[rsp+0x50]` 에 보존한다
    //    (write 1회 0xdb3af3 · read 6회 · 재정의 없음 · rsp 는 프롤로그/에필로그 외 불변).
    //    확증 3중 = ①콜러 0xdbcab8 이 넘기는 값이 이미 확정된 self(r14) ②exe 가 스스로
    //    champions[myside][*] 중 자기 자신을 제외하는 코드(0xdb448c) ③피해 추정의 피해자가 p3.
    //    ⚠발화는 **경기 초반 한정**(`현재틱 < [[p1+8]+0x13f8]` 분기 안) — 후반에 발화=0 이어도 정상.
    //    근거 = RE\2026-08-07_sf_margin-재조사-self는-3번째인자.md
    MicroSite {
        key: "sf_margin", orig: 15000, rva: 0xd36dcb,   // ★0.5.6(was 0.5.4 0xdb3f1b — ⚠2단계 apply 누락분 정정 2026-08-22: 054호스트 스켈레톤 UNIQUE→056·win 바이트 일치 확증)
        win: &[0x48, 0x05, 0x98, 0x3a, 0x00, 0x00],        // add rax,0x3a98
        pre: &[], tail: &[],
        op: MOp::AddR64 { dst: reg::RAX },
        imm_off: 2, imm_w: 4,
        a: Src::Stack(0x50), b: Src::None, resolve: Resolve::Direct,
        note: "self=p3(3번째 인자)=[진입rsp+0x50] 재로드",
    },
    // ⑦ ex_order_hold — "오더를 낸 지 N틱 안이면 재고하지 않는다" = 그 챔프의 고집.
    //    ★지난 판정 2건이 다 뒤집혔다: ⓐ창 4B → 뒤 `cmp rsi,rax` 를 tail 로 흡수해 **7B**
    //    ⓑ"팀 오더 컨텍스트라 조건② 불만족" → `r14`(arg2)는 팀이 아니라 **챔프별 AI 브레인**이다
    //    (크기 0x2950 트레이트 객체 · 현재 플랜 1바이트만 보유 ⟹ 팀 공유 불가).
    //    `+0x28c0` = 그 챔프가 오더를 낸 틱(`0xe7600f` 에서 write). `ex_think_min` 이 같은 구조체의
    //    인접 필드 `+0x28b8` 을 쓰고 이미 승인된 것과 정합.
    //    근거 = RE\2026-08-07_bt_vision_mem-ex_order_hold-재조사-둘다가능.md §B
    MicroSite {
        key: "ex_order_hold", orig: 10, rva: 0xcf0e73,   // ⚠0.5.6 미복구 = 0.5.4 stale 유지(win 검증 실패로 미설치=inert — 파일 헤더 노트 참조)
        win: &[0x48, 0x83, 0xc0, 0x0a, 0x48, 0x39, 0xc6],   // add rax,0xa ; cmp rsi,rax
        pre: &[], tail: &[0x48, 0x39, 0xc6],                 // ★tail 이 최종 플래그 생산자 = 원본과 동일
        op: MOp::AddR64 { dst: reg::RAX },
        imm_off: 3, imm_w: 1,
        a: Src::Mem(reg::RBP, 0x110), b: Src::Reg(reg::R12), resolve: Resolve::Champions,
        note: "self=champions[r12][side,role of [rbp+0x110]=arg4]",
    },
    // ⑤ ex_think_min — 재판단 간격 하한. 사이트엔 self 포인터가 없고 **재료 두 개**로 만든다:
    //    r12(=진입 rdx, 판단 컨텍스트)에서 side/role, champions 홀더는 [rbp+0x2c8](진입부 0xe76c4f 세팅).
    //    원본 = `lea rcx,[rax+rax*2+400]` ⟹ pre 로 `lea rcx,[rax+rax*2]` 실행 후 값을 더한다.
    //    ⚠원본 lea 는 플래그를 안 건드리므로 pushfq/popfq 로 넘겨준다(MOp::LeaAdd 가 그렇게 한다).
    MicroSite {
        key: "ex_think_min", orig: 400, rva: 0xcf3310,   // ⚠0.5.6 미복구 = 0.5.4 stale 유지(win 검증 실패로 미설치=inert — 파일 헤더 노트 참조)
        win: &[0x48, 0x8d, 0x8c, 0x40, 0x90, 0x01, 0x00, 0x00],   // lea rcx,[rax+rax*2+0x190]
        pre: &[0x48, 0x8d, 0x0c, 0x40],                            // lea rcx,[rax+rax*2]
        tail: &[],
        imm_off: 4, imm_w: 4,
        op: MOp::LeaAdd { dst: reg::RCX },
        // ★★[08-07 오배선 수정] ~~`Src::Reg(r12)`~~ → **`rdi`**. 사이트 시점의 side/role 소스는 r12 가
        //   아니라 **rdi(=4번째 인자)** 다. 사이트 **뒤**에서 `0xe76d2f mov r12,rdi` 로 r12 가 스왑되는데,
        //   앞선 RE 가 그 뒤의 `0xe76dd6 mov r15,r12` 만 보고 "r12 = arg2" 로 읽은 것이 원인.
        //   교차 확증 = 콜리 `0xe742a0` 도 `[rbp+0x110](=arg4)` 로만 0x810/0x8a0 을 읽고,
        //   브레인 구조체(arg2)는 그 오프셋을 아예 접근하지 않는다.
        //   ⚠증상은 **크래시가 아니라 조용한 무효**였다 — `[브레인+0x810]` 을 side 로 읽어 범위 밖 →
        //   self=0 → 전역값 폴백. `class_verify.txt` 의 `self확보%` 가 0% 면 이 계열 오배선이다.
        a: Src::Reg(reg::RDI), b: Src::Mem(reg::RBP, 0x2c8), resolve: Resolve::Champions,
        note: "self=champions[[rbp+0x2c8]][side,role of rdi=4번째 인자]",
    },
   // ⑧ `bt_vision_mem` 11사이트 — "적을 마지막으로 본 지 N틱 안이면 아직 기억한다".
   //
   // ★지난 판정("전역 목격틱 테이블이라 self 귀속 아님")은 **읽는 값**과 **상수의 주인**을 혼동한 것이었다.
   //   테이블이 팀 공유인 건 맞다. 그러나 이 함수는 진입부에서 self 를 **한 명으로 확정**하고 그 한 명의
   //   SubPlan 을 평가한다(적 5명 순회는 *대상* 루프일 뿐). ⟹ "내가 놓친 적을 얼마나 오래 경계하는가"
   //   = 한 챔프의 성향 ⟹ 조건② 만족. **갈라지는 것은 임계값뿐이고 목격 기록 자체는 팀 공유 그대로다.**
   // self 재료는 11곳 공통 = `[rbp+0x610]`(arg5, 함수 전체 write 0회) + `[rbp+0x560]`(holder, write 1회).
   // 근거 = 같은 RE 문서 §A.
    // r15 ×5
    bt_vis!(0xcbce46, &[0x49,0x83,0xc7,0x78,0x49,0x39,0xc7], reg::R15, &[0x49,0x39,0xc7]),
    bt_vis!(0xcbfe7d, &[0x49,0x83,0xc7,0x78,0x49,0x39,0xc7], reg::R15, &[0x49,0x39,0xc7]),
    bt_vis!(0xcbff05, &[0x49,0x83,0xc7,0x78,0x49,0x39,0xc7], reg::R15, &[0x49,0x39,0xc7]),
    bt_vis!(0xcbff8d, &[0x49,0x83,0xc7,0x78,0x49,0x39,0xc7], reg::R15, &[0x49,0x39,0xc7]),
    bt_vis!(0xd3c62b, &[0x49,0x83,0xc7,0x78,0x49,0x39,0xc7], reg::R15, &[0x49,0x39,0xc7]),
    // rdi ×1  ★이 자리가 기존 전역 imm 패치의 prefix 목록에서 빠져 있던 곳이다(08-07 결함 수정분)
    bt_vis!(0xcbd93e, &[0x48,0x83,0xc7,0x78,0x48,0x39,0xc7], reg::RDI, &[0x48,0x39,0xc7]),
    // r14 ×1
    bt_vis!(0xcbe545, &[0x49,0x83,0xc6,0x78,0x49,0x39,0xc6], reg::R14, &[0x49,0x39,0xc6]),
    // rsi ×4
    bt_vis!(0xcbf50f, &[0x48,0x83,0xc6,0x78,0x48,0x39,0xc6], reg::RSI, &[0x48,0x39,0xc6]),
    bt_vis!(0xcbf597, &[0x48,0x83,0xc6,0x78,0x48,0x39,0xc6], reg::RSI, &[0x48,0x39,0xc6]),
    bt_vis!(0xcbf61f, &[0x48,0x83,0xc6,0x78,0x48,0x39,0xc6], reg::RSI, &[0x48,0x39,0xc6]),
    bt_vis!(0x13eacf7, &[0x48,0x83,0xc6,0x78,0x48,0x39,0xc6], reg::RSI, &[0x48,0x39,0xc6]),
];

// ════════════════════════════════════════════════════════════════════════════
//  함수 진입부 훅 표 — ★RE 로 ①진입부 self 도달 ②프롤로그 안전 ③비재귀 를 확인한 함수만 등록.
//  등록하면 그 함수 안 사이트는 `Resolve::FromEntry(인덱스)` 로 self 를 받아온다.
// ════════════════════════════════════════════════════════════════════════════
pub(crate) static ENTRY_SITES: &[ClassEntrySite] = &[];
