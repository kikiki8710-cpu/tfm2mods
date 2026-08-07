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

/// 스텁이 호출하는 콜백. `idx`=사이트 번호, `a`·`b`=self 를 만들 재료(사이트별 규약).
/// 반환 = 그 자리에 쓸 값(클래스 오버라이드 없으면 원본값 = 동작 불변).
unsafe extern "C" fn class_micro_value(idx: u64, a: usize, b: usize) -> u64 {
    let i = idx as usize;
    if i >= MICRO_MAX || i >= MICRO_SITES.len() { return 0; }
    let site = &MICRO_SITES[i];
    MICRO_HITS[i].fetch_add(1, Ordering::Relaxed);
    // self 산출. ★전부 fault-safe read — 스테일 포인터면 -1(전역 폴백)이 되고 크래시하지 않는다.
    let selfe = match site.resolve {
        Resolve::Direct => a,
        Resolve::Champions => {
            if !ptr_ok(a) || !ptr_ok(b) { 0 } else {
                let side = rd_i64(a + 0x810).unwrap_or(-1);
                let role = rd_i64(a + 0x8a0).unwrap_or(-1) & 0xffff_ffff;   // dword 필드
                if !(0..2).contains(&side) || !(0..5).contains(&role) { 0 }
                else { rd_u64(b + side as usize * 0x28 + role as usize * 8 + 0x1e0).unwrap_or(0) as usize }
            }
        }
    };
    // ★패닉이 게임 콜스택으로 새면 UB(§3). 어떤 실패든 원본값으로 조용히 폴백한다.
    let v = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
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
        if c != i64::MIN { return c; }
        // 캐시 미스: CUR_CLASS 를 세워 tune() 이 클래스별 값을 보게 한다(판단 문맥 재현).
        let prev = cur_class();
        set_cur_class(cls);
        let v = tune(site.key, site.orig);
        set_cur_class(prev);
        if v != site.orig { MICRO_OVHIT[i].fetch_add(1, Ordering::Relaxed); }
        MICRO_VAL[i][slot].store(v, Ordering::Relaxed);
        v
    })).unwrap_or(site.orig);
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
    if MICRO_DONE.load(Ordering::Relaxed) { return; }
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let base = exe_base();
    if base == 0 { return; }
    // cfg 가 아직 안 올라왔으면 다음 기회에(오버라이드 판정을 빈 테이블로 하면 영구 미설치가 된다).
    if TUNE_PTR.load(Ordering::Acquire).is_null() { return; }
    MICRO_DONE.store(true, Ordering::Relaxed);

    let cb = class_micro_value as *const () as usize;
    let mut report = String::from("=== 클래스별 마이크로 디투어 설치 결과 ===\n\
        # 설치된 사이트만 클래스별 값이 먹는다. 값 변경은 재설치 없이 반영되지만,\n\
        # 설치/해제 자체는 게임 재시작이 필요하다(실행 중 코드 교체 경쟁 방지).\n");
    for (i, site) in MICRO_SITES.iter().enumerate() {
        if i >= MICRO_MAX { break; }
        if !has_class_override(site.key) {
            report.push_str(&format!("skip  {:<22} 클래스 오버라이드 없음(기존 바이트패치 유지)\n", site.key));
            continue;
        }
        MICRO_TRIED.fetch_add(1, Ordering::Relaxed);
        let addr = base + site.rva;
        let n = site.win.len();
        if n < 5 {
            report.push_str(&format!("FAIL  {:<22} 창이 5바이트 미만({})\n", site.key, n));
            continue;
        }
        if !readable(addr, n) {
            report.push_str(&format!("FAIL  {:<22} rva={:#x} 읽기 불가\n", site.key, site.rva));
            continue;
        }
        // ★원본 전수 대조 — 한 바이트라도 다르면 설치하지 않는다(게임 패치 방어).
        let mut ok = true;
        for (k, &b) in site.win.iter().enumerate() { if rd_u8(addr + k) != b { ok = false; break; } }
        if !ok {
            report.push_str(&format!("BLOCK {:<22} rva={:#x} 원본 바이트 불일치 — 이 자리는 내가 아는 그 자리가 아니다\n",
                                     site.key, site.rva));
            continue;
        }
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
        // 창 = E9 rel32 + 나머지 NOP
        let mut patch = vec![0x90u8; n];
        patch[0] = 0xe9;
        patch[1..5].copy_from_slice(&(rel as i32).to_le_bytes());
        let mut old: u32 = 0;
        if VirtualProtect(addr, n, 0x40, &mut old) == 0 {
            report.push_str(&format!("FAIL  {:<22} VirtualProtect 실패\n", site.key));
            continue;
        }
        core::ptr::copy_nonoverlapping(patch.as_ptr(), addr as *mut u8, n);
        VirtualProtect(addr, n, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), addr, n);
        MICRO_TAKEN[i].store(true, Ordering::Relaxed);
        MICRO_INSTALLED.fetch_add(1, Ordering::Relaxed);
        report.push_str(&format!("OK    {:<22} rva={:#x} 창={}B 스텁={:#x}  {}\n",
                                 site.key, site.rva, n, stub, site.note));
    }
    report.push_str(&format!("\n설치 {}/{} (시도 {})\n",
        MICRO_INSTALLED.load(Ordering::Relaxed), MICRO_SITES.len(), MICRO_TRIED.load(Ordering::Relaxed)));
    if let Some(p) = pth("class_micro.txt") { let _ = fs::write(p, report); }
}

static MICRO_TAKEN: [AtomicBool; MICRO_MAX] = [const { AtomicBool::new(false) }; MICRO_MAX];

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
    let mut s = String::from("\n[마이크로 디투어] 사이트별 발화/클래스적용\n");
    for (i, site) in MICRO_SITES.iter().enumerate() {
        if i >= MICRO_MAX { break; }
        if !MICRO_TAKEN[i].load(Ordering::Relaxed) { continue; }
        s.push_str(&format!("  {:<22} 발화={} 클래스적용={}\n", site.key,
            MICRO_HITS[i].load(Ordering::Relaxed), MICRO_OVHIT[i].load(Ordering::Relaxed)));
    }
    s
}

// ════════════════════════════════════════════════════════════════════════════
//  사이트 표 — ★RE 로 self 생존이 확인된 자리만 넣는다(안전규칙 ①).
//  근거 = REPORT\tfm2_ai_adjust\RE\2026-08-07_클래스별노브_확장가능성.md 및 후속 RE.
// ════════════════════════════════════════════════════════════════════════════
//  ⛔불가 판정(재시도 전 RE 문서를 먼저 볼 것 — 범위 한정 판정이다):
//    · `ex_order_hold` 0xe747e3 — 교체창 4B(`add rax,0xa`) + 값이 **팀 오더** 단위(조건② 불만족).
//    · `sf_margin`     0xdb3f1b — 사이트 시점 self 미도달(self 로드가 뒤쪽 분기, role 소스 없음).
//      ★"**A 방식(사이트에서 self 즉시 복원) 한정** 불가" — self 신원 역산은 별도 심층 RE 로 열릴 수 있다.
//    · `bt_vision_mem` 11곳    — 로드값이 `[목격틱테이블 + 적side*0x2e8 + 대상role*8 + 0x1e0]` = **전역 목격
//      테이블의 스칼라**이지 self 가 아니다(부록 D 오탐 정정과 정합). 교체창도 4B.
pub(crate) static MICRO_SITES: &[MicroSite] = &[
    // ① cs_lead_attack — 평타 선행예측 틱. self 가 r14 에 그대로 살아 있는 가장 깨끗한 자리.
    //    0xdb867b `mov r14,[rbx+rax*8]` = champions[myside][myrole] 이후 사이트까지 r14 write 없음.
    MicroSite {
        key: "cs_lead_attack", orig: 30, rva: 0xdb869a,
        win: &[0xb8, 0x1e, 0x00, 0x00, 0x00],            // mov eax,0x1e
        pre: &[], tail: &[],
        op: MOp::MovR32 { dst: reg::RAX },
        a: Src::Reg(reg::R14), b: Src::None, resolve: Resolve::Direct,
        note: "self=r14 생존(0xdb867b 로드)",
    },
    // ② mv2_avoid_coef — 회피 반경 계수. fn 0xe587f0 의 5번째 인자(=champions[side][role])가 rbp 에 들어있다.
    MicroSite {
        key: "mv2_avoid_coef", orig: 400, rva: 0xe58cf1,
        win: &[0x48, 0x69, 0xc1, 0x90, 0x01, 0x00, 0x00],  // imul rax,rcx,0x190
        pre: &[], tail: &[],
        op: MOp::ImulR64 { dst: reg::RAX, src: reg::RCX },
        a: Src::Reg(reg::RBP), b: Src::None, resolve: Resolve::Direct,
        note: "self=rbp 생존(0xe58816 [rsp+0x180] 로드)",
    },
    // ③ mv2_avoid_margin — 회피 여유 상수. ②와 같은 경로·같은 rbp.
    MicroSite {
        key: "mv2_avoid_margin", orig: 6000, rva: 0xe58d45,
        win: &[0x48, 0x05, 0x70, 0x17, 0x00, 0x00],        // add rax,0x1770
        pre: &[], tail: &[],
        op: MOp::AddR64 { dst: reg::RAX },
        a: Src::Reg(reg::RBP), b: Src::None, resolve: Resolve::Direct,
        note: "self=rbp 생존(②와 동일 경로)",
    },
    // ④ mv2_avoid_bias — 편향 확정 임계. ★사이트 전에 rbp 가 덮여서(0xe58dc9·0xe58de9) self 가 아니다.
    //    5번째 인자 원본 슬롯 `[rsp+0x180]` 은 함수 내 write 가 없어 재로드로 복원한다.
    //    ⚠사이트가 `cmp` 라 **뒤 `ja` 가 플래그를 소비**한다 — 값 op(cmp)가 스텁의 마지막 플래그 생산자여야 한다.
    MicroSite {
        key: "mv2_avoid_bias", orig: 1500, rva: 0xe5919f,
        win: &[0x48, 0x3d, 0xdc, 0x05, 0x00, 0x00],        // cmp rax,0x5dc
        pre: &[], tail: &[],
        op: MOp::CmpR64 { dst: reg::RAX },
        a: Src::Stack(0x180), b: Src::None, resolve: Resolve::Direct,
        note: "self=[진입rsp+0x180] 재로드(rbp 는 덮임)",
    },
    // ⑤ ex_think_min — 재판단 간격 하한. 사이트엔 self 포인터가 없고 **재료 두 개**로 만든다:
    //    r12(=진입 rdx, 판단 컨텍스트)에서 side/role, champions 홀더는 [rbp+0x2c8](진입부 0xe76c4f 세팅).
    //    원본 = `lea rcx,[rax+rax*2+400]` ⟹ pre 로 `lea rcx,[rax+rax*2]` 실행 후 값을 더한다.
    //    ⚠원본 lea 는 플래그를 안 건드리므로 pushfq/popfq 로 넘겨준다(MOp::LeaAdd 가 그렇게 한다).
    MicroSite {
        key: "ex_think_min", orig: 400, rva: 0xe76cb0,
        win: &[0x48, 0x8d, 0x8c, 0x40, 0x90, 0x01, 0x00, 0x00],   // lea rcx,[rax+rax*2+0x190]
        pre: &[0x48, 0x8d, 0x0c, 0x40],                            // lea rcx,[rax+rax*2]
        tail: &[],
        op: MOp::LeaAdd { dst: reg::RCX },
        a: Src::Reg(reg::R12), b: Src::Mem(reg::RBP, 0x2c8), resolve: Resolve::Champions,
        note: "self=champions[[rbp+0x2c8]][side,role of r12]",
    },
];
