// sylas — 데이터 챔피언 + 강탈 네이티브 통합 모드 (★0.5.6, 2026-08-24 단일 모드로 병합).
//   구 `sylas`(데이터 챔프) + 구 `sylas_hijack`(네이티브 dll) → 모드 하나. 로더는 mods\<MOD_ID>\ 안의
//   모든 .dll을 로드하므로 에셋(champion/·icons/·text/)과 dll이 한 폴더에 공존 가능
//   (선례 = tfm2_banpick_illust: dll+illust/+ui/ · tfm2_champion_exclude: dll+text/).
// ★배선 (4개 병렬 RE + Grab apply 정밀 디컴, 2026-07-29 확정):
//   Grab effect apply(0.5.6 0x1801780 / 구 0.5.2 0x1e267b0) = 사일러스 궁 전용 = 궁 시전 트리거. ABI:
//     rcx=effect_def / rdx=미사용 / r8=world / r9=WorldOps 테이블(★0.5.6부터 하드코딩 폐지 = 런타임 r9 사용)
//     [rsp+0x30]=target_key(붙잡은 적) / [rsp+0x38]=casting_ctx{tag@0=0, key@8=sylas_key}
//   강탈: X = resolve(world, target_key) = 붙잡은 적. X의 궁 apply를 sylas caster로 CALL.
//     X_action = [X+0x148], X_vtbl = [X+0x150], X_apply = [X_vtbl+0xd0] (base 궁 apply, 0x20b0460류).
//     base apply 계약: rcx=action_data, rdx=world, r8=WorldOps(=Grab이 받은 r9), r9=target, [rsp+0x28]=casting_ctx.
//     casting_ctx = Grab의 a7 그대로 재사용({tag=0, sylas_key}) → sylas가 caster로 귀속.
//   Grab 원본은 트램폴린이 계속 실행(사일러스 대시) + X궁 강탈 추가 발화.
// ★안전: cfg arm=1 게이트(기본 OFF). shadow-call은 catch_unwind. X_action=X live원본(가짜 금지). VEH-safe read.
#![allow(non_snake_case, dead_code)]
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Duration;
use mod_api::*;

// ★0.5.6 마이그(2026-08-24) — 구값은 각 줄 주석. RVA = exe↔exe 체인 재핀(0.5.2→3→4→5→6, 전 단계 UNIQUE).
const GRAB_RVA: usize = 0x1504310; // 0.5.6 (구 0.5.5 0x1162b20 / 0.5.4 0x156c8d0 / 0.5.3 0x12ca630 / 0.5.2 0x1e267b0)
const GRAB_SIG: [u8; 12] = [0x41,0x57,0x41,0x56,0x41,0x54,0x56,0x57,0x53,0x48,0x83,0xec]; // 0.5.6 프롤로그 동일(실측)
const GRAB_LEN: usize = 13; // push6(9)+sub rsp,0x38(4)
// ★WORLDOPS_RVA(구 0.5.2 0x38c5d78) 폐지 = detour가 받는 r9를 그대로 사용(버전 독립).
//   이유: 그 테이블은 .rdata 데이터라 마스크시그(코드) 재핀 불가 + 데이터 지문도 다중매치(0.5.6 실측 429후보).
//   Grab이 r9로 WorldOps를 넘겨주므로 하드코딩할 이유가 없다 = 다음 패치에도 재핀 불요.
// ★재생 루프 = Combine effect의 apply. 자식 {data,vtable} stride 0x10 배열을 순회하며
//   각 자식의 vtable+0x20(apply)을 같은 인자 세트로 호출한다(0.5.6 디컴 FUN_141802630).
//   self 보정식 = data + ((vtable[0x10]/*align*/ - 1) & !0xf) + 0x10.
const COMBINE_RVA: usize = 0x159a430;
const COMBINE_SIG: [u8; 12] = [0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x55,0x53];
const COMBINE_LEN: usize = 19; // 8push(12) + sub rsp,0x98(7), rip-rel 없음
const EFF_STRIDE: usize = 0x10;   // 자식 항목 = {data@+0, vtable@+8}
const EFF_APPLY:  usize = 0x20;   // effect vtable + 0x20 = apply
const EFF_ALIGN:  usize = 0x10;
/// ★"자식을 순회하는" effect는 하나가 아니다. 둘 다 `[self+0x10]`=len, `[self+8]`=ptr(stride 0x10) 구조.
///   `0x1802630` = Combine apply(재생 루프) / `0x1802760` = 또 다른 Combine류(디스어셈 확인).
///   중첩을 펼치지 않으면 **껍데기만 캡처**된다(유저 지적 2026-08-24: "이펙트 2개가 진짜 끝인가").
const COMBINE_APPLYS: [usize; 2] = [0x1802630, 0x1802760];
/// ★게임 할당자. 디스어셈 계약: rcx 무시 / edx=flags / r8=size / 반환 rax=ptr.
const ALLOC_RVA: usize = 0x2ab4010;

/// ★★effect를 **fresh 인스턴스로 복제**한다.
///   effect = `Arc { strong@+0, weak@+8, payload@+0x10 }` 이고 payload 크기는 **vtable+8**에 있다.
///   바닐라 setup(0x182b9b0)이 하는 일(할당 → Arc 헤더 → payload 채우기)을 그대로 재현하는 것.
///   목적 = "남의 effect 인스턴스는 그 시전에 묶여 재사용 불가"라는 벽을, **내용은 같되 새 객체**로 우회.
unsafe fn clone_effect(data: usize, vt: usize) -> Option<usize> {
    let size = rd_u64(vt + 8)?;
    if size == 0 || size > 0x1000 { return None; }
    let total = 0x10 + size as usize;
    let mem = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        sy_call4(exe_base() + ALLOC_RVA, 0, 0, total, core::ptr::null_mut())
    })).ok()?;
    if mem < 0x10000 || !readable(mem, total) { return None; }
    if !wr_u64(mem, 1) || !wr_u64(mem + 8, 1) { return None; }   // strong=1, weak=1
    let words = (size as usize + 7) / 8;
    for i in 0..words {
        let v = rd_u64(data + 0x10 + i * 8)?;
        if !wr_u64(mem + 0x10 + i * 8, v) { return None; }
    }
    Some(mem)
}
#[inline] unsafe fn is_combine_apply(vt: usize) -> bool {
    match rd_u64(vt + EFF_APPLY) { Some(a) => COMBINE_APPLYS.contains(&(rva_of(a) as usize)), None => false }
}

/// 자식 배열을 **재귀적으로 펼쳐** 말단 effect만 모은다(중첩 Combine은 내용으로 치환).
unsafe fn flatten_children(selfp: usize, depth: u32, out: &mut Vec<(usize, usize)>) {
    if depth > 3 || out.len() >= 16 { return; }
    let ptr = rd_u64(selfp + 8).unwrap_or(0) as usize;
    let len = rd_u64(selfp + 0x10).unwrap_or(0) as usize;
    if ptr < 0x10000 || len == 0 || len > 32 { return; }
    for i in 0..len {
        if out.len() >= 16 { return; }
        let it = ptr + i * EFF_STRIDE;
        let d = rd_u64(it).unwrap_or(0) as usize;
        let v = rd_u64(it + 8).unwrap_or(0) as usize;
        if d < 0x10000 || !in_exe(v as u64) { continue; }
        if is_combine_apply(v) {
            // 중첩: 그 Combine의 self(= data + align보정 + 0x10)로 내려간다
            let al = rd_u64(v + EFF_ALIGN).unwrap_or(8) as usize;
            let inner = d.wrapping_add((al.wrapping_sub(1)) & !0xf).wrapping_add(0x10);
            if readable(inner, 24) { flatten_children(inner, depth + 1, out); }
        } else {
            out.push((d, v));
        }
    }
}   // effect vtable + 0x10 = align
// ★바닐라 궁 경로 = action-kind 점프테이블 디스패처. 챔프별 궁 apply(base 공용 0x173c780 등)가
//   내부에서 스택에 "실행 요청 구조체"를 조립해 이 함수로 넘긴다 ⟹ 모든 바닐라 action이 여기로 수렴.
//   계약: rcx=req구조체, rdx=world, r8=WorldOps, r9=target, [rsp+0x28]=casting_ctx
//   req[0x30] = action kind(JT 인덱스), req[0] = action Arc 포인터
const JT_RVA: usize = 0x161b390;
const JT_SIG: [u8; 12] = [0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x55,0x53];
const JT_LEN: usize = 16;  // 8push(12) + sub rsp,0x38(4), rip-rel 없음
// ★base 궁 apply — 본문 첫머리에 `lVar2 = [action+0x48]; if (lVar2 == -1) return 0;` **궁 게이트**가 있다.
//   ⟹ 이 함수 진입 = "궁 시전"이라 kind 판별 없이 궁만 골라낼 수 있다(JT는 모든 action이 kind=0으로 들어와 구분 불가).
//   계약(0.5.6 디컴): rcx=action_data, rdx=world, r8=WorldOps, r9=target, [rsp+0x28]=casting_ctx → u64
const BASEULT_RVA: usize = 0x1556940;
const BASEULT_SIG: [u8; 11] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57];
const BASEULT_LEN: usize = 19; // ★push×8 = 12B(rbp1+r15/r14/r13/r12 각2 +rsi/rdi/rbx 각1) + sub rsp,0x88(7B) = 19B
//   ⚠18로 잘못 잡아 sub 명령이 6B만 복사돼 게임이 먹통이 됐다(2026-08-24). hook_len은 반드시 명령 경계에 맞출 것.
const A_ULT_GATE: usize = 0x48; // clone 대상 시작점이자 게이트 필드
// ★base 궁 apply가 내부에서 쓰는 clone. 이것을 우리가 "캡처 시점"에 호출해 **우리 소유의 사본**을 만든다.
//   (공유 action 포인터를 그대로 들고 있다가 남의 문맥에 태우면 크래시 — 2026-08-24 실증)
const CLONE_RVA: usize = 0x14f43c0;
const A_F180: usize = 0x180;
const A_F188: usize = 0x188;
const A_F190: usize = 0x190;
const A_F198: usize = 0x198; // casting_type — JT 디스패처가 req[0x30]으로 읽는 인덱스
const A_F19C: usize = 0x19c;
const REQ_COPY: usize = 0x48; // 스택 조립 구조체 복사 크기(디컴상 local_a0~local_68 = 0x40 + 여유)
const WOPS_RESOLVE_SLOT: usize = 0x1e0; // 0.5.6 Grab 본문 `call [r9+0x1e0]` (구 0.5.2 0x1b8) — r9 sanity 검사용
// world SlotMap 오프셋 (resolve 재구현) — 0.5.5에서 world 저대역 +0x18 강체 시프트(정본 = MIGRATION §7.5 §6)
const W_DENSE_BASE: usize = 0x738; // 구 0x720
const W_DENSE_LEN:  usize = 0x740; // 구 0x728
const W_SLOTS:      usize = 0x750; // 구 0x738
const W_SLOT_LEN:   usize = 0x758; // 구 0x740
const W_FB_KEY:     usize = 0x618; // ⚠0.5.2 값 유지 = 0.5.6 미검증(폴백 전용. 뒤의 name=="sylas" 게이트가 오탐 차단)
const W_FB_ENT:     usize = 0x70;  // ⚠동상
const ENT_STRIDE:   usize = 0x6c0; // 구 0x6a8 (+0x18) — serpen MOBATICK `imul rax,rax,0x6c0` 실측이 정본
const E_NAME_PTR: usize = 0x250; // 불변(0.5.6 serpen 사용중)
const E_NAME_LEN: usize = 0x258; // 불변
const E_CUR_HP:   usize = 0x670; // 0.5.6 현재 HP (serpen 정본: 0.5.5에서 구 0x658 → 0x670). 재생 효과 정량 판정용
// ★★[v86] ~~`E_ALIVE = 0x468`(==0이 생존)~~ → **오독이었다**(2026-08-25 RE, `Entity::fmt` 전수).
//   `+0x468 = 0x370 + 0xf8` = **버프 합산 `cc_immune`**. Grab/Stun이 `[+0x468]==0`을 요구하는 건
//   "살아있나"가 아니라 **"CC 면역이 아닌가"**다. CC면역 버프(예: ninja_ult)가 걸린 동안엔
//   사일러스 Grab/Stun이 조용히 무발동한다 — 그걸 "죽었다"로 로그하면 오독이다.
const E_CC_IMMUNE: usize = 0x468;   // bool. !=0이면 CC 무효
const E_HP:        usize = 0x670;   // ★진짜 생존 = hp != 0
const E_BLOCK_TGT: usize = 0x6a0;   // block_target_tick == 0 이어야 타게팅 가능
const E_CAN_TGT:   usize = 0x6b9;   // can_target == 1
/// ★유닛 종류 = `*(u32*)(ent+0x68)`. 챔피언 = 13 (게임 전역 247사이트가 이 비교를 쓴다)
const E_TY:        usize = 0x68;
const TY_CHAMPION: u64   = 13;
/// ★팀 = `{disc@+0x00 (0=Player/1=Neutral), side@+0x08 (0/1)}`
const E_TEAM_DISC: usize = 0x00;
const E_TEAM_SIDE: usize = 0x08;
// entity 액션 슬롯: name(0x250) 불변 & 0x450 시프트 ⟹ 삽입점 ∈ (0x258,0x450] ⟹ 0x148/0x150은 불변 판정
const S_ULT_ACTION: usize = 0x148;
const S_ULT_VTBL:   usize = 0x150;
const V_APPLY_BASE: usize = 0xd0; // base effect-action vtable +0xd0 = apply (⬜0.5.6 미검증 — 진단로그로 실측)

type BOOL = i32;
type HMODULE = usize;
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> HMODULE;
    fn VirtualAlloc(addr: usize, sz: usize, typ: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old: *mut u32) -> BOOL;
    fn VirtualQuery(addr: usize, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, size: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
    fn AddVectoredExceptionHandler(first: u32, handler: VehHandler) -> usize;
    fn QueryPerformanceCounter(c: *mut i64) -> BOOL;
    fn GetCurrentThreadId() -> u32;
    fn GetProcessHeap() -> usize;
    fn HeapAlloc(heap: usize, flags: u32, bytes: usize) -> usize;
    fn HeapFree(heap: usize, flags: u32, mem: usize) -> BOOL;
    fn CreateFileW(name: *const u16, access: u32, share: u32, sa: usize, disp: u32, flags: u32, tmpl: usize) -> usize;
    fn WriteFile(h: usize, buf: *const u8, len: u32, written: *mut u32, ov: usize) -> BOOL;
    fn SetFilePointer(h: usize, lo: i32, hi: *mut i32, method: u32) -> u32;
    // ★★[v88] 게임 **메인 스레드** 식별용 — 창을 소유한 스레드가 메인 스레드다.
    fn EnumWindows(cb: EnumWndProc, lparam: isize) -> BOOL;
    fn GetWindowThreadProcessId(hwnd: usize, pid: *mut u32) -> u32;
    fn GetCurrentProcessId() -> u32;
    fn IsWindowVisible(hwnd: usize) -> BOOL;
}
type EnumWndProc = extern "system" fn(usize, isize) -> BOOL;
#[repr(C)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_prot: u32, _pad0: u32,
    region: usize, state: u32, protect: u32, typ: u32 }
#[repr(C)] struct ExceptionRecord { code: u32, flags: u32, _rec: usize, addr: usize, _np: u32, _pad: u32 }
#[repr(C)] struct ExceptionPointers { rec: *mut ExceptionRecord, ctx: usize }
type VehHandler = extern "system" fn(*mut ExceptionPointers) -> i32;

fn exe_base() -> usize { unsafe { GetModuleHandleW(core::ptr::null()) } }
fn rva_of(a: u64) -> u64 { let b = exe_base() as u64; if a >= b && a < b + 0x8000000 { a - b } else { a } }
unsafe fn in_exe(a: u64) -> bool { let b = exe_base() as u64; a >= b && a < b + 0x4000000 }

unsafe fn readable(a: usize, len: usize) -> bool {
    if a < 0x10000 { return false; }
    let mut mbi: MemBasicInfo = core::mem::zeroed();
    if VirtualQuery(a, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    if mbi.state != 0x1000 { return false; }
    const RPROT: u32 = 0x02|0x04|0x08|0x10|0x20|0x40|0x80;
    if mbi.protect & RPROT == 0 || mbi.protect & 0x101 != 0 { return false; }
    let end = mbi.base + mbi.region;
    a.wrapping_add(len) <= end
}

core::arch::global_asm!(
    ".globl sg_rd8", ".globl sg_rd8_f", ".globl sg_rd8_l",
    ".globl sg_rd1", ".globl sg_rd1_f", ".globl sg_rd1_l",
    "sg_rd8:", "sg_rd8_f:", "mov rax, qword ptr [rcx]", "mov qword ptr [rdx], rax", "mov eax, 1", "ret",
    "sg_rd8_l:", "xor eax, eax", "ret",
    "sg_rd1:", "sg_rd1_f:", "movzx eax, byte ptr [rcx]", "mov byte ptr [rdx], al", "mov eax, 1", "ret",
    "sg_rd1_l:", "xor eax, eax", "ret",
    ".globl sg_wr8", ".globl sg_wr8_f", ".globl sg_wr8_l",
    "sg_wr8:", "sg_wr8_f:", "mov qword ptr [rcx], rdx", "mov eax, 1", "ret",
    "sg_wr8_l:", "xor eax, eax", "ret",
);
// ★clone은 (rax, rdx) 쌍을 반환한다. win64 `extern "C"`로는 rdx를 받을 수 없고,
//   inline asm에 `in(reg)`로 함수 주소를 넘기면 컴파일러가 그것을 인자 레지스터에 배정해 충돌할 수 있다
//   (v23 실측: clone 호출 즉시 크래시). ⟹ 레지스터 배치를 우리가 고정하는 전용 스텁을 쓴다.
//   호출: sy_call4(fn, a1, a2, a3, out_rdx) → rax 반환, rdx는 *out_rdx에 기록
core::arch::global_asm!(
    ".globl sy_call4",
    "sy_call4:",
    "push rsi",
    "mov rsi, qword ptr [rsp + 0x30]",   // 5번째 인자(out_rdx) — push rsi 후 +8
    // ★★정렬: 진입 rsp≡8(mod16) → `push rsi`로 ≡0 → **shadow 0x20만** 빼야 call 직전이 ≡0이 된다.
    //   구 `sub rsp,0x28`은 call 직전을 ≡8로 만들어 **피호출자가 보는 rsp를 8바이트 어긋나게** 했다.
    //   그 상태로 진입부에서 `movaps/movdqa [rsp+N]`를 쓰는 함수를 부르면 **즉시 #GP(AV)** —
    //   2026-08-24 "생성기 0x16d8b30 직접 호출 = 로그 한 줄 없이 즉사"의 정체가 이것이다.
    //   (할당자 0x2ab1670은 정렬 SSE 저장이 없어 어긋나도 살아남았다 = 기존 성공 사례와 일관)
    "sub rsp, 0x20",                     // shadow space 0x20 (정렬 유지)
    "mov rax, rcx",                      // fn
    "mov rcx, rdx",                      // a1
    "mov rdx, r8",                       // a2
    "mov r8, r9",                        // a3
    "call rax",
    "add rsp, 0x20",
    "test rsi, rsi",
    "je 2f",
    "mov qword ptr [rsi], rdx",
    "2:",
    "pop rsi",
    "ret",
);

extern "C" {
    fn sy_call4(f: usize, a1: usize, a2: usize, a3: usize, out_rdx: *mut usize) -> usize;
    fn sg_rd8(addr: usize, out: *mut u64) -> u32;
    fn sg_rd1(addr: usize, out: *mut u8) -> u32;
    static sg_rd8_f: u8; static sg_rd8_l: u8;
    static sg_rd1_f: u8; static sg_rd1_l: u8;
    fn sg_wr8(addr: usize, val: u64) -> u32;
    static sg_wr8_f: u8; static sg_wr8_l: u8;
}
#[inline] unsafe fn wr_u64(a: usize, v: u64) -> bool { if a < 0x10000 { return false; } sg_wr8(a, v) != 0 }
#[inline] unsafe fn rd_u64(a: usize) -> Option<u64> { if a < 0x10000 { return None; } let mut o=0u64; if sg_rd8(a,&mut o)!=0 {Some(o)} else {None} }
#[inline] unsafe fn rd_u8(a: usize)  -> Option<u8>  { if a < 0x10000 { return None; } let mut o=0u8;  if sg_rd1(a,&mut o)!=0 {Some(o)} else {None} }

static VEH_INSTALLED: AtomicBool = AtomicBool::new(false);
extern "system" fn sg_veh(p: *mut ExceptionPointers) -> i32 {
    const CE: i32 = -1; const CS: i32 = 0;
    unsafe {
        if p.is_null() { return CS; }
        let rec = (*p).rec; if rec.is_null() || (*rec).code != 0xC0000005 { return CS; }
        let ctx = (*p).ctx; if ctx == 0 { return CS; }
        let rip = *((ctx + 0xF8) as *const u64) as usize;
        let land = if rip == core::ptr::addr_of!(sg_rd8_f) as usize { core::ptr::addr_of!(sg_rd8_l) as usize }
                   else if rip == core::ptr::addr_of!(sg_rd1_f) as usize { core::ptr::addr_of!(sg_rd1_l) as usize }
                   else if rip == core::ptr::addr_of!(sg_wr8_f) as usize { core::ptr::addr_of!(sg_wr8_l) as usize }
                   else { 0 };
        if land != 0 { *((ctx + 0xF8) as *mut u64) = land as u64; return CE; }
        CS
    }
}
// ────────────────────────────────────────────────────────────────────────
// ★★[v66] **크래시 브레드크럼**. "크래시됐다"만으론 원인을 못 좁힌다 —
//   폴트가 난 **명령 주소(RVA)** 를 남기면 그 함수가 곧 원인이다.
//   ⚠VEH 안에서는 절대 할당·락·포맷을 하지 않는다(그 자체가 2차 폴트를 만든다).
//   원자값에만 적어 두고, **cfg 갱신 스레드**가 주기적으로 로그로 흘린다.
static FAULT_CODE: AtomicU32 = AtomicU32::new(0);
static FAULT_RIP:  AtomicU64 = AtomicU64::new(0);
static FAULT_ADDR: AtomicU64 = AtomicU64::new(0);
static FAULT_SEQ:  AtomicU32 = AtomicU32::new(0);
static FAULT_LOGGED: AtomicU32 = AtomicU32::new(0);

/// ★★[v71] 폴트를 **VEH 안에서 즉시 파일에 쓴다.**
///   구: 원자값에 적어두고 cfg 스레드(2초 주기)가 흘림 → **프로세스가 즉사하면 영영 안 남는다**
///   (2026-08-25 실측: 유저 크래시 로그에 ☠ 0건). 크래시 진단의 핵심은 "죽기 전에 남기는 것"이다.
///   ⚠VEH 안이므로 **할당·락·format! 금지** — 미리 연 핸들 + 스택 버퍼 + 수동 16진 변환만 쓴다.
static FAULT_FH: AtomicU64 = AtomicU64::new(0);
#[inline] unsafe fn hexw(buf: &mut [u8; 192], pos: &mut usize, v: u64) {
    const D: &[u8; 16] = b"0123456789abcdef";
    if *pos + 18 >= buf.len() { return; }
    buf[*pos] = b'0'; buf[*pos + 1] = b'x'; *pos += 2;
    let mut started = false;
    for i in (0..16).rev() {
        let nib = ((v >> (i * 4)) & 0xf) as usize;
        if nib != 0 { started = true; }
        if started || i == 0 { buf[*pos] = D[nib]; *pos += 1; }
    }
}
#[inline] unsafe fn puts_raw(buf: &mut [u8; 192], pos: &mut usize, sblk: &[u8]) {
    for &b in sblk { if *pos < buf.len() { buf[*pos] = b; *pos += 1; } }
}

extern "system" fn crash_veh(p: *mut ExceptionPointers) -> i32 {
    unsafe {
        if p.is_null() { return 0; }
        let rec = (*p).rec;
        if rec.is_null() { return 0; }
        let code = (*rec).code;
        // 치명적 예외만: AV / illegal instr / int div0 / stack overflow
        if code == 0xC0000005 || code == 0xC000001D || code == 0xC0000094 || code == 0xC00000FD {
            let ctx = (*p).ctx;
            let rip = if ctx != 0 { *((ctx + 0xF8) as *const u64) } else { 0 };
            // ★우리 가드 읽기(sg_rd8/sg_rd1/sg_wr8)에서 난 것은 정상 동작이라 기록하지 않는다.
            let guard = rip == core::ptr::addr_of!(sg_rd8_f) as u64
                || rip == core::ptr::addr_of!(sg_rd1_f) as u64
                || rip == core::ptr::addr_of!(sg_wr8_f) as u64;
            if !guard {
                FAULT_CODE.store(code, Ordering::Relaxed);
                FAULT_RIP.store(rip, Ordering::Relaxed);
                FAULT_ADDR.store((*rec).addr as u64, Ordering::Relaxed);
                FAULT_SEQ.fetch_add(1, Ordering::Relaxed);
                // ★즉시 기록 — 여기서 안 쓰면 즉사 시 아무것도 안 남는다.
                let fh = FAULT_FH.load(Ordering::Relaxed);
                if fh != 0 {
                    let mut buf = [0u8; 192];
                    let mut n = 0usize;
                    puts_raw(&mut buf, &mut n, b"FAULT code=");
                    hexw(&mut buf, &mut n, code as u64);
                    puts_raw(&mut buf, &mut n, b" rip=RVA:");
                    hexw(&mut buf, &mut n, rva_of(rip));
                    puts_raw(&mut buf, &mut n, b" addr=");
                    hexw(&mut buf, &mut n, (*rec).addr as u64);
                    puts_raw(&mut buf, &mut n, b"\r\n");
                    let mut wrote: u32 = 0;
                    WriteFile(fh as usize, buf.as_ptr(), n as u32, &mut wrote, 0);
                }
            }
        }
        0   // EXCEPTION_CONTINUE_SEARCH — 흐름은 그대로 둔다
    }
}
static CRASH_VEH_INSTALLED: AtomicBool = AtomicBool::new(false);
fn crash_veh_install() {
    if CRASH_VEH_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    unsafe {
        // 폴트 전용 파일을 **미리 열어둔다**(VEH 안에서 여는 것은 위험).
        let path: Vec<u16> = "C:\\tfm2mods\\sylas\\sylas_fault.txt"
            .encode_utf16().chain(core::iter::once(0)).collect();
        // GENERIC_WRITE | OPEN_ALWAYS, FILE_APPEND 대신 끝으로 이동은 생략(줄 단위 append로 충분)
        let h = CreateFileW(path.as_ptr(), 0x4000_0000, 0x1 | 0x2, 0, 4, 0x80, 0);
        if h != usize::MAX && h != 0 {
            SetFilePointer(h, 0, core::ptr::null_mut(), 2);   // FILE_END
            FAULT_FH.store(h as u64, Ordering::Relaxed);
        }
        AddVectoredExceptionHandler(0, crash_veh);   // 0 = 마지막(우리 가드 뒤)
    }
}
/// cfg 스레드가 주기적으로 호출 — 새 폴트가 잡혔으면 로그로 흘린다.
fn fault_drain() {
    let seq = FAULT_SEQ.load(Ordering::Relaxed);
    if seq == FAULT_LOGGED.swap(seq, Ordering::Relaxed) { return; }
    let (c, rip, ad) = (FAULT_CODE.load(Ordering::Relaxed),
                        FAULT_RIP.load(Ordering::Relaxed),
                        FAULT_ADDR.load(Ordering::Relaxed));
    hlog(&format!("☠[폴트] code={:#x} rip=RVA:{:#x} addr={:#x} (누적 {}회)\n",
        c, rva_of(rip), ad, seq));
}

fn veh_install() { if VEH_INSTALLED.swap(true, Ordering::Relaxed) { return; } unsafe { AddVectoredExceptionHandler(1, sg_veh); } }

unsafe fn read_name(ptr: usize, len: usize) -> Option<String> {
    if ptr < 0x10000 || len < 2 || len > 24 { return None; }
    let mut buf = Vec::with_capacity(len);
    for i in 0..len { let b = rd_u8(ptr + i)?; if !(b == b'_' || b.is_ascii_alphanumeric()) { return None; } buf.push(b); }
    String::from_utf8(buf).ok()
}
unsafe fn ent_name(ent: usize) -> Option<String> {
    let p = rd_u64(ent + E_NAME_PTR)? as usize; let l = rd_u64(ent + E_NAME_LEN)? as usize; read_name(p, l)
}

// ★resolve 재구현 (WorldOps resolve_by_key 0x2305520 동형): world+key → entity(stride 0x6a8).
unsafe fn resolve(world: usize, key: u64) -> usize {
    if world < 0x10000 { return 0; }
    let slot_len = rd_u64(world + W_SLOT_LEN).unwrap_or(0);
    if key < slot_len {
        if let Some(slots) = rd_u64(world + W_SLOTS) {
            let entry = (slots as usize).wrapping_add(key as usize * 0x10);
            if rd_u64(entry).unwrap_or(0) as u32 == 1 {
                let di = rd_u64(entry + 8).unwrap_or(u64::MAX);
                if di < rd_u64(world + W_DENSE_LEN).unwrap_or(0) {
                    if let Some(db) = rd_u64(world + W_DENSE_BASE) {
                        let ent = (db as usize).wrapping_add(di as usize * ENT_STRIDE);
                        if readable(ent, ENT_STRIDE) { return ent; }
                    }
                }
            }
        }
    }
    if rd_u64(world + W_FB_KEY).unwrap_or(u64::MAX ^ 1) == key
        && (rd_u64(world + W_FB_ENT).unwrap_or(0) as i32) != -1 { return world + W_FB_ENT; }
    0
}

static WRITE_LOCK: Mutex<()> = Mutex::new(());
/// ★전역 로그 상한. 조합 테스트는 여러 경기를 **병렬 sim**하므로 훅 호출량이 리플레이의 몇 배다.
///   hlog는 매번 파일을 열고 닫으므로 상한이 없으면 게임이 사실상 정지한다.
static LOG_TOTAL: AtomicU32 = AtomicU32::new(0);
/// ★로그 호출 상한. 넘으면 hlog가 **조용히** 반환한다.
/// ⚠2026-08-25 실사고: 궁 집계가 1440/1500을 혼자 먹어 경기 구간이 통째로 무로그가 됐고,
///   잘린 앞부분(배경 월드)만 보고 "사일러스 없는 경기"로 오판했다.
///   ⟹ 상한을 올리고, **소진 시 그 사실을 한 번은 찍는다**(침묵으로 끝나지 않게).
const LOG_MAX: u32 = 8000;
fn hlog(s: &str) {
    use std::io::Write;
    let n = LOG_TOTAL.fetch_add(1, Ordering::Relaxed);
    if n > LOG_MAX { return; }
    let s: &str = if n == LOG_MAX {
        "
===== ⚠로그 상한 도달 — 이후 기록 없음. 이 줄 뒤의 무로그는 '사건 없음'이 아니다 =====
"
    } else { s };
    let _g = WRITE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true)
        .open("C:\\tfm2mods\\sylas\\sylas_log.txt") { let _ = f.write_all(s.as_bytes()); }
}

static ARMED: AtomicBool = AtomicBool::new(false);
/// ★1단계 최소 데모: 사일러스 궁이 나갈 때 "캡처해 둔 다른 챔프의 Combine"을 같은 인자로 재생한다.
/// 강탈 메커니즘(대상 선택·보유·원복)은 배제 — **재생 자체가 성립하는지**만 본다.
static REPLAY: AtomicBool = AtomicBool::new(false);
/// ★self 재생 = 사일러스 궁 자식(Grab 제외)을 한 번 더 실행한다.
///   목적 = **재생 메커니즘이 게임 상태에 실제로 영향을 주는가**를 이분법으로 판정.
///   데미지·스턴·버프가 2배로 나타나면 메커니즘 성립 ⟹ 남의 궁이 안 보이는 건 "고른 effect가 무효과"인 문제.
///   변화가 없으면 메커니즘 자체(인자/컨텍스트)에 문제.
static REPLAY_SELF: AtomicBool = AtomicBool::new(false);
/// ★바닐라 궁 재생 = cfg `replay_vanilla=1` + `src=<champ>`
static REPLAY_VANILLA: AtomicBool = AtomicBool::new(false);
/// ★`only_ult=1` = **궁으로 판정된 Combine만** 재료로 캡처한다.
///   눈에 보이는 궁(archer 난사 등)을 확실히 잡을 때 쓴다. 평타/소형 스킬이 재료를 덮는 것을 막는다.
static ONLY_ULT: AtomicBool = AtomicBool::new(false);
/// ★`graft=1` = **우리가 apply를 부르지 않고**, 사일러스 궁 Combine의 자식 배열 한 칸을
///   캡처한 남의 effect로 **바꿔치기**한다. detour 반환 후 **게임의 원본 재생 루프가 그것을 실행**한다.
///   지금까지 실패한 4가지(effect 직접 호출·base apply 호출·base apply 훅·JT 호출)는 전부
///   "우리가 호출"이었다. 이것은 "게임이 호출하게" 하는 유일한 미시도 경로다.
static GRAFT: AtomicBool = AtomicBool::new(false);
/// ★`nullify=1` = **음성 대조군**. 이식하지 않고 사일러스 궁 Combine의 len만 0으로 만든다.
///   화면에서 사일러스 궁이 **아무 효과도 못 내면** = 우리 쓰기가 게임 실행에 실제로 영향을 준다는 증명.
///   여전히 평소대로 궁이 나오면 = 쓰기가 무시되고 있다는 뜻이고, graft 경로 자체가 무효다.
///   (이식 결과가 화면에 안 보인다는 관찰 때문에 도입 — 2026-08-24)
static NULLIFY: AtomicBool = AtomicBool::new(false);
/// ★★`steal_cast=1` = **시전자 바꿔치기**. 바닐라 챔프가 궁을 쏘는 **그 프레임에**
///   casting_ctx의 시전자 키를 사일러스로 덮는다. effect는 방금 만들어진 fresh 인스턴스이고
///   caster만 사일러스가 되므로, "나중에 재사용"할 때의 컨텍스트 불일치가 원천적으로 없다.
static STEAL_CAST: AtomicBool = AtomicBool::new(false);
/// ★`retarget=1` = 시전자를 바꿀 때 **대상도 원래 시전자(X)로** 바꾼다.
///   시전자만 바꾸면 대상은 X가 고른 그대로라, X가 적팀이면 **사일러스가 아군을 때린다**
///   (유저 실측 2026-08-24: "혹시 타겟을 아군으로 뒀나?").
///   X 자신을 대상으로 삼으면 "뺏어서 주인에게 돌려준다"가 되어 컨셉과도 맞는다.
static RETARGET: AtomicBool = AtomicBool::new(true);
/// entity 좌표·팀 (Grab/이동 함수 디컴 + 동일오프셋 비교 58건으로 확정)
const E_POS_X: usize = 0x660;
const E_POS_Y: usize = 0x668;
// ★★[v86] ~~`E_TEAM = 0x6a8`~~ → **오독**. `+0x6a8 = nontarget_avoid_range`(거리값)이다.
//   이것이 **`nearest_enemy`가 타워를 집던 진짜 원인** — 팀 비교가 사실상 무작위였다.
//   정본 = `RE/2026-08-25_entity-필드맵-전수.md`.
/// 팀 판정: None = 중립(정글·에픽·세르펜)
#[inline] unsafe fn team_of(e: usize) -> Option<u64> {
    if rd_u64(e + E_TEAM_DISC)? == 0 { rd_u64(e + E_TEAM_SIDE) } else { None }
}
#[inline] unsafe fn is_enemy(a: usize, b: usize) -> bool {
    match (team_of(a), team_of(b)) { (Some(x), Some(y)) => x != y, _ => false }
}
#[inline] unsafe fn is_champion(e: usize) -> bool { rd_u64(e + E_TY).map(|v| v & 0xffff_ffff) == Some(TY_CHAMPION) }
#[inline] unsafe fn is_alive(e: usize) -> bool { rd_u64(e + E_HP).unwrap_or(0) != 0 }
#[inline] unsafe fn is_targetable(e: usize) -> bool {
    rd_u64(e + E_BLOCK_TGT) == Some(0) && rd_u8(e + E_CAN_TGT) == Some(1)
}

// ══════════════════════════════════════════════════════════════════════════════
// ★★★[v87] cctx 정본 개입 — `ent+0x88..0x9F`
//   RE 2026-08-25 `RE/2026-08-25_cctx-생성지점-castingtype매핑.md`:
//   **cctx는 별도 구조가 아니라 엔티티 필드다.** 시전 개시(`0x17f8d10`)가 오더의
//   CastTarget을 이 자리에 통째로 복사하고, 이후 fire-record·투사체 큐·조준 리드·
//   apply가 **전부 여기서 파생**된다.
//   ⟹ ~~apply 직전 인자 포인터 교체(v82~v86)~~ 는 **자리가 틀렸다**:
//      그 시점은 조준 리드 `FUN_1417fc7b0`가 이미 지나간 뒤라
//      ①리드가 옛 cctx로 계산을 끝냈고 ②tag가 리드 도메인(1·2) 밖이면 조용히 no-op.
//      = 강탈 궁의 **스킬샷 예측사격이 죽는다**.
//   ⚠DONE.md "cctx 게임 스택 직접 쓰기 = 폐기"는 **apply 콜러 프레임 한정 판정**이다.
//     `ent+0x88`은 엔티티 필드라 스택 파괴 위험이 없다(§11 적용범위 사례).
const E_CCTX:    usize = 0x88;   // {tag:u32@0, u32@4(죽은 필드), i64@8, i64@0x10}
const E_CAST_ST: usize = 0x70;   // == E_ACTIVE_SLOT. 6 = 슬롯3(궁) 시전 중
/// 궁 슬롯 casting_type = E_SLOT0 + 3*STRIDE + SLOT_GATE = 0x490+0xa8+0x30
const E_ULT_CT:  usize = 0x568;
const E_ULT_RNG: usize = 0x548;  // 궁 슬롯 +0x10 = 사거리 base
const E_ULT_RGR: usize = 0x550;  // 궁 슬롯 +0x18 = 레벨당 사거리 증가
const E_BASE_RNG: usize = 0x438; // entity range
const E_LEVEL:   usize = 0x5c8;  // ★스킬 수가 아니라 레벨(v86 정정)

/// ★casting_type(슬롯 gate) → cctx tag. **두 enum은 1↔2가 뒤바뀐 별개 enum이다**(RE 확정).
///   casting_type: Targeting=0 / Position=1 / Direction=2 / None=3
///   cctx tag    : Targeting=0 / Direction=1 / Position=2 / None=3
#[inline] fn ct_to_tag(ct: i32) -> Option<u64> {
    match ct { 0 => Some(0), 1 => Some(2), 2 => Some(1), 3 => Some(3), _ => None }
}

/// tag==2(좌표) 사거리 클램프 — **엔진은 시전 개시 때 1회만** 한다(`0x17f9072~0x17f911f`).
///   우리가 개시 후에 좌표를 넣으면 엔진 클램프는 이미 지나갔으므로 **직접 해야 한다.**
///   maxrange = [ent+0x438] + slot[+0x10] + (level-1)*slot[+0x18]
unsafe fn clamp_to_range(cent: usize, sx: i64, sy: i64, tx: i64, ty: i64) -> (i64, i64) {
    let base = rd_u64(cent + E_BASE_RNG).unwrap_or(0) as i64;
    let r0   = rd_u64(cent + E_ULT_RNG).unwrap_or(0) as i64;
    let rg   = rd_u64(cent + E_ULT_RGR).unwrap_or(0) as i64;
    let lv   = rd_u64(cent + E_LEVEL).unwrap_or(1) as i64;
    let maxr = base + r0 + (lv - 1).max(0) * rg;
    if maxr <= 0 { return (tx, ty); }
    let (dx, dy) = (tx - sx, ty - sy);
    let dsq = (dx as i128) * (dx as i128) + (dy as i128) * (dy as i128);
    if dsq <= (maxr as i128) * (maxr as i128) { return (tx, ty); }
    let d = (dsq as f64).sqrt();
    if d <= 0.0 { return (tx, ty); }
    let k = maxr as f64 / d;
    (sx + (dx as f64 * k) as i64, sy + (dy as f64 * k) as i64)
}

/// ★★강탈 궁의 casting_type에 맞춰 `ent+0x88`을 **제자리 교정**한다.
///   반환 = 로그 문자열(교정했을 때만).
unsafe fn fix_entity_cctx(world: usize, cent: usize, caster_key: u64) -> Option<String> {
    // 강탈 궁의 gate. 슬롯에서 직접 읽는 게 정본(HELD_GATE는 보조).
    // ★★[v99] 슬롯 게이트(사일러스=0)가 아니라 **공여자 casting_type**으로 tag를 정한다.
    //   슬롯 게이트를 1로 두면 AI가 궁 오더를 아예 안 만든다(v97 실측: 시전 0건).
    //   그래서 게이트는 0으로 남기고 cctx만 공여자 계약(장판=tag2)에 맞춘다.
    let src_ct = GRAFT_SRC_CT.load(Ordering::Relaxed);
    let slot_ct = match rd_u64(cent + E_ULT_CT) { Some(v) => v as u32, None => return None };
    let ct = if SLOT_GRAFT.load(Ordering::Relaxed) && src_ct != u32::MAX {
        // ★★[v103] **공여자와 슬롯의 casting_type이 같으면 아무것도 하지 않는다.**
        //   계약이 이미 맞는데도 덮어쓰면 AI가 고른 대상을 우리가 매 8틱마다
        //   다른 대상으로 바꿔치기하게 된다(광전사 케이스에서 실제로 그랬다).
        //   개입은 **계약이 어긋날 때만.**
        //   ★★[v132] **단 casting_target 이 어긋나면 계약이 같아도 개입해야 한다.**
        //   실측(2026-08-30 magic_knight): 공여자 ct=4(AllyOnlySelf) 를 슬롯에 심으면(graft_tgt=1)
        //   AI 가 **궁을 한 번도 결심하지 않는다**(시전 상태 6 진입 0회 / 다른 챔프는 4~8회 시전).
        //   ct=4 는 자기시전 사이트만 여는데 그건 결심 3계층의 **최후 폴백**이라 거의 안 닿는다.
        //   ⟹ 슬롯 casting_target 은 **사일러스의 7 로 두어 AI가 정상 결심**하게 하고,
        //      **대상만 공여자 규칙으로 바로잡는다**(러너·자버프가 적에게 붙는 것을 막는 게 목적이므로
        //      대상만 맞으면 된다). 이 경로가 tgt_key 를 ct 규칙으로 다시 고르므로 성립한다.
        let tgt_mismatch = {
            let g = GRAFT_SRC_TGT.load(Ordering::Relaxed);
            let slot_tgt = rd_u64(cent + E_SLOT0 + 3 * SLOT_STRIDE + 0x28).map(|v| v as u32);
            g != u32::MAX && slot_tgt.map(|s| s != g).unwrap_or(false)
        };
        if src_ct == slot_ct && !tgt_mismatch { return None; }
        src_ct as i32
    } else { slot_ct as i32 };
    let want_tag = ct_to_tag(ct)?;
    let tag0 = rd_u64(cent + E_CCTX)? as u32 as u64;
    let a0   = rd_u64(cent + E_CCTX + 8)?;
    let b0   = rd_u64(cent + E_CCTX + 0x10)?;
    let (sx, sy) = (rd_u64(cent + E_POS_X)? as i64, rd_u64(cent + E_POS_Y)? as i64);

    // ── 대상 좌표/키 확보: 게임이 tag0(대상키)으로 줬으면 그걸 우선, 아니면 최근접 적
    let mut tgt_key: Option<u64> = if tag0 == 0 { Some(a0) } else { None };
    let mut tgt_pos: Option<(i64, i64)> = None;
    if tag0 == 2 { tgt_pos = Some((a0 as i64, b0 as i64)); }
    // ★★[v128] 게임이 준 대상이 **공여자 casting_target을 만족하는지** 검사한다.
    //   사일러스 슬롯은 ct=7(EnemyChampion)이라 AI는 늘 적을 준다. 공여자 궁이 아군/자기 대상이면
    //   그 적 대상은 **틀린 값**이므로 버리고 규칙에 맞는 후보로 갈아야 한다.
    let ct_want = eff_ct(cent);
    if let Some(k) = tgt_key {
        let te = resolve(world, k);
        if te >= 0x10000 && ct_allows(ct_want, cent, te) {
            tgt_pos = Some((rd_u64(te + E_POS_X).unwrap_or(0) as i64,
                            rd_u64(te + E_POS_Y).unwrap_or(0) as i64));
        } else { tgt_key = None; tgt_pos = None; }
    }
    if tgt_key.is_none() || tgt_pos.is_none() {
        if let Some(k) = nearest_by_ct(world, cent, caster_key, ct_want) {
            let pe = resolve(world, k);
            if pe >= 0x10000 {
                if tgt_key.is_none() { tgt_key = Some(k); }
                if tgt_pos.is_none() {
                    tgt_pos = Some((rd_u64(pe + E_POS_X).unwrap_or(0) as i64,
                                    rd_u64(pe + E_POS_Y).unwrap_or(0) as i64));
                }
            }
        }
    }

    // ── want_tag별 페이로드 구성
    let (w8, w10, how): (u64, u64, String) = match want_tag {
        0 => { // Targeting = 대상 엔티티 SlotMap 키
            let k = tgt_key?;
            (k, 0, format!("대상키 {}", k as i64))
        }
        2 => { // Position = 월드 절대좌표 (+ 우리가 직접 사거리 클램프)
            let (tx, ty) = if ZONE_AT_ENEMY.load(Ordering::Relaxed) {
                tgt_pos.unwrap_or((sx, sy))
            } else { (sx, sy) };
            let (cx, cy) = clamp_to_range(cent, sx, sy, tx, ty);
            let cl = if (cx, cy) != (tx, ty) { " ★클램프됨" } else { "" };
            (cx as u64, cy as u64,
             format!("좌표 ({}, {}){}{}", cx, cy,
                     if ZONE_AT_ENEMY.load(Ordering::Relaxed) { "←대상" } else { "←self" }, cl))
        }
        1 => { // Direction = 캐스터 기준 **상대** 벡터
            let (tx, ty) = tgt_pos?;
            let (dx, dy) = (tx - sx, ty - sy);
            if dx == 0 && dy == 0 { return None; }
            (dx as u64, dy as u64, format!("방향 ({}, {})", dx, dy))
        }
        _ => return None, // None(3) = 손대지 않는다
    };

    // 이미 원하는 형태면 건드리지 않는다(엔진이 알아서 준 경우 = Targeting 대부분)
    if tag0 == want_tag && a0 == w8 && b0 == w10 { return None; }

    let ok1 = wr_u64(cent + E_CCTX, want_tag);
    let ok2 = wr_u64(cent + E_CCTX + 8, w8);
    let ok3 = wr_u64(cent + E_CCTX + 0x10, w10);
    // `+0x8c`(죽은 필드)는 위 u64 쓰기의 상위 32비트로 **자동 0**이 된다 — 별도 쓰기 불요.
    // ★투사체 경로 동기화: 개시가 pending을 탔다면 `ent+0x2a8` Vec 마지막 엔트리
    //   `+0x20..+0x37`에도 같은 cctx 사본이 있다(stride 0x38).
    let mut projn = 0usize;
    let pptr = rd_u64(cent + 0x2b0).unwrap_or(0) as usize;
    let plen = rd_u64(cent + 0x2b8).unwrap_or(0) as usize;
    if pptr >= 0x10000 && plen >= 1 && plen <= 64 {
        let it = pptr + (plen - 1) * 0x38 + 0x20;
        if wr_u64(it, want_tag) && wr_u64(it + 8, w8) && wr_u64(it + 0x10, w10) { projn = 1; }
    }
    let ctname = match ct { 0 => "Targeting", 1 => "Position", 2 => "Direction", 3 => "None", _ => "미상" };
    Some(format!("  ★★[cctx·ent+0x88] ct={}({}) ⟹ tag {} / {} | 이전=({}, {:#x}, {:#x})                  wr={}{}{} 투사체동기={}
",
        ct, ctname, want_tag, how, tag0, a0, b0,
        ok1 as u8, ok2 as u8, ok3 as u8, projn))
}

/// ★사일러스 기준 **가장 가까운 적**의 SlotMap key를 고른다("일반적으로 궁 쓰는 것처럼").
///   X 자신을 대상으로 삼으면 강탈 컨셉엔 맞지만 위치가 부자연스럽다(유저 지시 2026-08-24).
/// ★★[v128] 게임 정본 술어 `FUN_1417faeb0`(casting_target 14 arm)의 재현.
///   RE `2026-08-25_casting_target-필터-14arm-전수.md` 표를 그대로 옮겼다.
///   ⚠이게 없으면 아군/자기 대상 궁(ct 0~4)을 뺏었을 때 **적을 조준**한다 —
///     ct=4(AllyOnlySelf)면 자버프가 적에게 붙고, ct=1/3이면 힐·실드가 적에게 간다.
///   실측 공여자 값: cavalry_knight=4 · knight=1 · priest=3 · archer/berserker/crossbowman=7.
///   ⚠2(InCC)/9(RecentlyAttacked)의 부가 조건(CC 상태·최근 피격)은 **후보 선정용이라 생략**한다
///     — 게임 게이트는 슬롯에 심은 값으로 별도 판정하므로 여기서 좁힐 필요가 없다.
unsafe fn ct_allows(ct: u32, se: usize, ent: usize) -> bool {
    let champ = is_champion(ent);
    let foe = is_enemy(se, ent);
    match ct {
        0 => !foe,                                  // Ally (자신 포함)
        1 | 2 => !foe && champ,                     // AllyChampion / AllyChampionInCC
        3 => !foe && champ && ent != se,            // AllyNotSelf
        4 => ent == se,                             // AllyOnlySelf (팀 검사조차 없음)
        5 | 6 => foe,                               // Enemy / EnemyWithoutTower
        7 | 8 | 9 => foe && champ,                  // EnemyChampion 계열
        10 | 11 => true,                            // Both 계열
        12 => champ,                                // BothChampion
        _ => false,                                 // 13 None
    }
}

/// `casting_target` 규칙을 만족하는 **가장 가까운** 후보의 SlotMap 키.
/// ct=4(자기시전)는 순회하지 않고 자기 키를 돌려준다.
/// ⚠"가장 가까운"은 게임 AI의 점수식이 아니다 — 이건 게임이 준 cctx가 쓸모없을 때의 **폴백**이고,
///   `graft_tgt=1`이면 AI가 애초에 옳은 대상을 골라주므로 거의 타지 않는다.
unsafe fn nearest_by_ct(world: usize, se: usize, sk: u64, ct: u32) -> Option<u64> {
    if ct == 4 { return Some(sk); }
    let sx = rd_u64(se + E_POS_X)? as i64;
    let sy = rd_u64(se + E_POS_Y)? as i64;
    let slot_len = rd_u64(world + W_SLOT_LEN).unwrap_or(0).min(2048);
    let (mut best, mut bestd) = (None, u64::MAX);
    for k in 0..slot_len {
        let ent = resolve(world, k);
        if ent == 0 { continue; }
        if !is_champion(ent) || !is_alive(ent) || !is_targetable(ent) { continue; }
        if !ct_allows(ct, se, ent) { continue; }
        let (Some(x), Some(y)) = (rd_u64(ent + E_POS_X), rd_u64(ent + E_POS_Y)) else { continue };
        let (dx, dy) = (x as i64 - sx, y as i64 - sy);
        let dsq = (dx * dx + dy * dy) as u64;
        if dsq < bestd { bestd = dsq; best = Some(k); }
    }
    best
}

/// 지금 이식된 궁의 **유효 casting_target**. 공여자 값을 확보했으면 그것, 아니면 사일러스 슬롯 값.
unsafe fn eff_ct(cent: usize) -> u32 {
    let g = GRAFT_SRC_TGT.load(Ordering::Relaxed);
    if g != u32::MAX { return g; }
    rd_u64(cent + E_SLOT0 + 3 * SLOT_STRIDE + 0x28).map(|v| v as u32).unwrap_or(7)
}

/// ★★[v128] 이 world의 **전 챔피언 궁 메타 1회 덤프**.
///   목적 = 어느 챔프의 궁이 아군/자기 대상(ct 0~4)인지, 돌진형(vt+0x110)인지를 **실측**으로 세우는 것.
///   정적 디컴으로 챔프↔프로바이더를 잇는 건 이름 매핑이 어려워 추정이 섞인다. 여기선 엔티티에
///   이름과 슬롯이 나란히 있으므로 **추정이 0이다.**
///   ⚠레벨<5인 챔프는 궁 슬롯이 정적 더미(gate=-1)라 "궁없음"으로 찍힌다 — 경기 후반에 다시 찍힌다.
unsafe fn ult_meta_census(world: usize, live: bool) {
    if !ULT_META.load(Ordering::Relaxed) { return; }
    {
        let mut g = META_DONE.lock().unwrap_or_else(|x| x.into_inner());
        if g.contains(&world) { return; }
        if g.len() >= 12 { return; }            // 상한 — 백그라운드 world가 무한히 늘지 않게
        g.push(world);
    }
    // ★기본구현 3종을 여기서도 확보한다 — census는 첫 이식보다 먼저 돌아서
    //   `ai_mask_vt`가 아직 채우지 않았을 수 있다(비우면 ★/기본 판별이 죽는다).
    let def = {
        let mut g = SY_VT_DEF.lock().unwrap_or_else(|x| x.into_inner());
        if g.is_none() { *g = default_eff_stubs(); }
        *g
    };
    let slot_len = rd_u64(world + W_SLOT_LEN).unwrap_or(0).min(2048);
    let mut out = format!("
===== [궁메타 전수] world={:#x} ({}) =====
 champ            | ctype tgt atk |  range  growth start | cool | vt+0x50/+0x60/+0x118
", world, if live { "표시경기" } else { "배경sim" });
    let mut n = 0;
    for k in 0..slot_len {
        let e = resolve(world, k);
        if e < 0x10000 || !is_champion(e) { continue; }
        let name = match ent_name(e) { Some(v) => v, None => continue };
        let b = e + E_SLOT0 + 3 * SLOT_STRIDE;
        let w = slot_words(b);
        if w.len() != SLOT_STRIDE / 8 { continue; }
        let gate = w[6] as u32;
        if gate == u32::MAX {
            out.push_str(&format!(" {:<16} | 궁슬롯 비어있음(레벨 {} < 5)
",
                name, rd_u64(e + E_SKILL_CNT).unwrap_or(0)));
            n += 1; continue;
        }
        // ⚠쿨은 **DataActionDef(0x1a8) 구조일 때만** +0x170이 cooltime이다(네이티브는 다른 필드).
        //   게이트를 안 걸면 네이티브 챔프 행에 쓰레기 숫자가 찍혀 표 전체가 못 믿을 것이 된다.
        let pd = rd_u64(e + E_PROV_ULT).unwrap_or(0) as usize;
        let pv = rd_u64(e + E_PROV_ULTV).unwrap_or(0) as usize;
        let psz = if in_exe(pv as u64) { rd_u64(pv + 0x08).unwrap_or(0) } else { 0 };
        let raw = if pd >= 0x10000 { rd_u64(pd + PROV_COOL).unwrap_or(0) } else { 0 };
        let cool = if psz >= 0x1a8 && raw > 0 && raw < 1_000_000 { raw } else { 0 };
        // effect vtable 3슬롯이 **기본값인지 오버라이드인지** — AI마스크가 실제로 뭘 바꾸는지의 근거
        let vt = w[1] as usize;
        let mark = |i: usize, d: Option<u64>| -> String {
            match (rd_u64(vt + i * 8), d) {
                (Some(v), Some(dv)) => if v == dv { "기본".into() }
                                       else { format!("★{:#x}", rva_of(v)) },
                (Some(v), None) => format!("{:#x}", rva_of(v)),
                _ => "?".into(),
            }
        };
        let cool_s = if cool > 0 { format!("{}", cool) } else { "네이티브".to_string() };
        out.push_str(&format!(" {:<16} | {:>5} {:>3} {:>3} | {:>7} {:>6} {:>5} | {:>8} | {} {} {}
",
            name, gate, w[5] as u32, (w[5] >> 32) as u32, w[2], w[3], w[4], cool_s,
            mark(VT_I_MOVE_TICKS,  def.map(|d| d[0])),
            mark(VT_I_NOT_INSTANT, def.map(|d| d[1])),
            mark(VT_I_IS_MOVE,     def.map(|d| d[2]))));
        n += 1;
    }
    out.push_str(&format!("  ctype: 0 Targeting/1 Position/2 Direction  |  tgt: 0 Ally 1 AllyChampion 2 AllyChampInCC 3 AllyNotSelf 4 AllyOnlySelf 5 Enemy 6 EnemyNoTower 7 EnemyChampion 8 EnemyChampInCC 9 EnemyRecentAtk 10 Both 11 BothNoTower 12 BothChampion 13 None
  ★ = 기본구현이 아닌 오버라이드 = AI마스크가 실제로 동작을 바꾸는 궁. 총 {}명
", n));
    hlog(&out);
}

unsafe fn nearest_enemy(world: usize, se: usize, sk: u64) -> Option<u64> {
    let _ = sk;
    let sx = rd_u64(se + E_POS_X)? as i64;
    let sy = rd_u64(se + E_POS_Y)? as i64;
    let slot_len = rd_u64(world + W_SLOT_LEN).unwrap_or(0).min(2048);
    let (mut best, mut bestd) = (None, u64::MAX);
    for k in 0..slot_len {
        if k == sk { continue; }
        let ent = resolve(world, k);
        if ent == 0 || ent == se { continue; }
        // ★★[v86] 게임 정본 술어로 교체(RE 2026-08-25). 구 버전은 team을 +0x6a8로 잘못 읽어
        //   사실상 무작위 비교였고, 유닛 종류 필터가 없어 **타워를 집었다**.
        if !is_champion(ent) { continue; }                      // ★챔피언만(타워·미니언 제외)
        if !is_alive(ent) { continue; }                         // hp != 0
        if !is_targetable(ent) { continue; }                    // block_target_tick==0 && can_target==1
        if !is_enemy(se, ent) { continue; }                     // ★진짜 팀 비교
        let (Some(x), Some(y)) = (rd_u64(ent + E_POS_X), rd_u64(ent + E_POS_Y)) else { continue };
        let (dx, dy) = ((x as i64 - sx), (y as i64 - sy));
        let dsq = (dx * dx + dy * dy) as u64;
        if dsq < bestd { bestd = dsq; best = Some(k); }
    }
    best
}
/// ★`ult_cost=1` = 강탈을 **사일러스가 궁을 쓴 것으로 취급**한다.
///   ①사일러스 궁 쿨(entity+0xC8)이 남아 있으면 강탈하지 않는다(공짜 강탈 방지)
///   ②강탈에 성공하면 그 쿨을 소모시킨다(= 원래 시전자가 방금 리셋한 값을 사일러스에도 적용)
///   딜·킬 크레딧은 시전자 인자가 이미 사일러스라 게임이 알아서 사일러스에게 준다.
static ULT_COST: AtomicBool = AtomicBool::new(false);
/// ★`clone_graft=1` = 캡처할 때 **effect를 fresh 복제**해 보관한다(원본 포인터를 들지 않는다).
///   원본 재사용이 무효/크래시였던 문제를 "새 객체"로 우회하려는 시도.
static CLONE_GRAFT: AtomicBool = AtomicBool::new(false);
/// ★★`build_effect=1` (D안) = **사일러스 궁 시전 시점에** 캡처한 X의 action으로
///   게임의 **effect 생성기 `0x16d8b30`** 을 호출해 **fresh effect를 새로 만든다**.
///   실행은 하지 않고 게임 루프에 맡긴다(= graft). 지금까지 실패한 조합과 다른 점:
///     · effect를 옮기지 않는다(새로 만든다) → "시전에 묶임" 회피
///     · 우리가 실행하지 않는다 → 직접 호출 시 크래시하던 문제 회피
///   1단계는 **반환값 관찰만** 한다(무엇이 나오는지 확인 후 이식).
static BUILD_EFFECT: AtomicBool = AtomicBool::new(false);
/// ★`stack_trace=1` = 사일러스 궁 시전 시 **콜스택 전체**를 뜬다.
///   지금까지 effect 실행 단계만 봤다. 시전 파이프라인을 위에서부터 보려면 상위 프레임이 필요하다.
///   이전 `stack_ret_cands`는 범위도 좁고 검증이 없어 vtable 주소가 섞였다 ⟹ **리턴주소 검증** 추가:
///   진짜 리턴주소라면 그 직전이 call 명령이어야 한다(`E8 rel32` / `FF /2` 형태).
static STACK_TRACE: AtomicBool = AtomicBool::new(false);
/// ★★entity의 **대기 큐**(0x17eabc0 entity 틱 처리에서 발견).
///   매 틱 `[+0x300]`(len) 만큼 `[+0x2f8]`(ptr, stride 0x10 fatptr)을 순회하며 각 항목의
///   **`vtable+0x70`** 을 호출하고 큐를 비운다(take).
///   effect(`vt+0x20`)와 **다른 계층** — 스킬 시전이 여기 쌓이는 것으로 보인다.
///   ⟹ 여기에 X의 궁을 넣으면 게임이 정규 경로로 실행한다(미검증 가설).
const E_Q_A: usize = 0x2f0;   // ? (take 시 0으로)
const E_Q_PTR: usize = 0x2f8; // 항목 배열 ptr (take 시 8로)
const E_Q_LEN: usize = 0x300; // 항목 수 (take 시 0으로)
const Q_ITEM_VCALL: usize = 0x70; // 항목 vtable + 0x70 = 매 틱 실행
const Q_ITEM_DONE: usize = 0x78;  // 항목 vtable + 0x78 = 완료 판정(true면 큐에서 제거·drop)
                                  //   근거 = FUN_140ed4750(큐 retain 함수) 디컴
/// ★entity 틱 처리 함수. 진입 직후 큐를 take하므로, **이 훅이 큐를 볼 수 있는 유일한 시점**이다.
const ETICK_RVA: usize = 0x1583de0;
const ETICK_SIG: [u8; 11] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57];
const ETICK_LEN: usize = 19; // push×8(12) + sub rsp,0x578(7), rip-rel 없음
/// 사일러스 entity 주소(큐를 직접 읽기 위해 보관)
static SYLAS_ENT: AtomicU64 = AtomicU64::new(0);
static ETICK_DIAG: AtomicU32 = AtomicU32::new(0);
/// `queue_probe=1` = 사일러스/대상 entity의 대기 큐를 덤프한다.
static QUEUE_PROBE: AtomicBool = AtomicBool::new(false);

unsafe fn dump_queue(ent: usize, who: &str) -> String {
    let a = rd_u64(ent + E_Q_A).unwrap_or(0);
    let ptr = rd_u64(ent + E_Q_PTR).unwrap_or(0) as usize;
    let len = rd_u64(ent + E_Q_LEN).unwrap_or(0);
    let mut s = format!("  [큐] {} ent={:#x} a={:#x} ptr={:#x} len={}\n", who, ent, a, ptr, len);
    if ptr < 0x10000 || len == 0 || len > 16 { return s; }
    for i in 0..len as usize {
        let it = ptr + i * 0x10;
        let d = rd_u64(it).unwrap_or(0);
        let v = rd_u64(it + 8).unwrap_or(0) as usize;
        s.push_str(&format!("    [{}] data={:#x} vt=RVA:{:#x} tick=RVA:{:#x} done=RVA:{:#x}\n",
            i, d, rva_of(v as u64), rva_of(rd_u64(v + Q_ITEM_VCALL).unwrap_or(0)),
            rva_of(rd_u64(v + Q_ITEM_DONE).unwrap_or(0))));
    }
    s
}

/// 스택을 훑어 **검증된 리턴주소**만 수집한다.
unsafe fn walk_stack(e: usize, max: usize) -> Vec<u64> {
    let mut out: Vec<u64> = Vec::new();
    let mut off = 0usize;
    while off < 0x1200 && out.len() < max {
        if let Some(v) = rd_u64(e + off) {
            if in_exe(v) {
                let ra = v as usize;
                // call rel32 : E8 xx xx xx xx  (리턴주소 -5)
                let is_e8 = rd_u8(ra.wrapping_sub(5)) == Some(0xe8);
                // call r/m64 : FF /2 — 2~7바이트. 앞쪽에서 FF를 찾는다(REX 포함 가능)
                let is_ff = (2..=7).any(|k| rd_u8(ra.wrapping_sub(k)) == Some(0xff));
                if is_e8 || is_ff {
                    let r = rva_of(v);
                    if !out.contains(&r) { out.push(r); }
                }
            }
        }
        off += 8;
    }
    out
}
static STEAL_N: AtomicU32 = AtomicU32::new(0);
/// graft는 detour 반환 **후** 원본 루프가 실행하므로 훅 안에서 즉시 측정할 수 없다.
/// ⟹ (대상ent, 시전자ent, HP, effect수, 교체한 apply)를 보류해 두고 **다음 훅 진입 때** 대조한다.
static GRAFT_PEND: Mutex<Option<(usize, usize, u64, u64, u64, u64, u64)>> = Mutex::new(None);
/// ★재생 소스 챔프 지정 = cfg `src=<champion_id>` (예: `src=demon`).
///   비우면 자동 선택(= 그 판에서 자식 수가 가장 많은 Combine을 시전한 바닐라 챔프).
///   자동 선택은 "궁일 확률이 높다"는 휴리스틱일 뿐이라 무효과 effect를 고를 수 있다(v16 ninja 사례).
static SRC_NAME: Mutex<String> = Mutex::new(String::new());
/// ★최소 자식 수 = cfg `min_kids=N` (기본 2). 시각 효과 있는 궁을 노릴 때 올려 쓴다.
static MIN_KIDS: AtomicU32 = AtomicU32::new(2);

fn cfg_val(s: &str, key: &str) -> Option<String> {
    for line in s.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix(key) {
            if let Some(v) = rest.strip_prefix('=') { return Some(v.trim().to_string()); }
        }
    }
    None
}
/// ★cfg 불리언 플래그 = **줄 단위 정확 매칭**. `contains("graft=1")`은 `buff_graft=1`에도 걸려
///   켠 적 없는 노브를 발동시킨다(2026-08-24 실사고). 키가 접미사로 겹치면 반드시 이쪽을 쓸 것.
/// cfg 경로 해석 — **게임 exe 기준**으로 도출한다(설치 경로 하드코딩 금지, CLAUDE.md 2절).
/// 배포본 `<게임>\mods\sylas\sylas.cfg` 를 우선하고, 없으면 개발 폴더로 폴백한다.
const DEV_CFG: &str = "C:\\tfm2mods\\sylas\\sylas.cfg";
fn cfg_read() -> String {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join("mods").join("sylas").join("sylas.cfg");
            if let Ok(t) = std::fs::read_to_string(&p) { return t; }
        }
    }
    std::fs::read_to_string(DEV_CFG).unwrap_or_default()
}

fn cfg_flag(s: &str, key: &str) -> bool { cfg_val(s, key).as_deref() == Some("1") }
fn cfg_refresher() {
    // ⚠여기는 **모드가 띄운 스레드**다 — 게임 메인 스레드가 아니다(v87 실사고).
    //   판정은 GAME_TID(창 소유 스레드)로 한다.
    unsafe { MAIN_TID.store(GetCurrentThreadId(), Ordering::Relaxed); }
    hlog(&format!("[스레드] 모드 초기화 tid = {}
", unsafe { GetCurrentThreadId() }));
    crash_veh_install();
    std::thread::spawn(|| loop {
        // ★[v88] 게임 창이 뜼는 대로 메인 스레드 tid를 확정한다(창이 늦게 뜼기 때문에 재시도).
        if GAME_TID.load(Ordering::Relaxed) == 0 {
            let t = unsafe { detect_game_tid() };
            if t != 0 {
                GAME_TID.store(t, Ordering::Relaxed);
                hlog(&format!("[스레드] ★게임 메인 스레드(창 소유) tid = {}
", t));
            }
        }
        let s = cfg_read();
        ARMED.store(cfg_flag(&s, "arm"), Ordering::Relaxed);
        REPLAY.store(cfg_flag(&s, "replay"), Ordering::Relaxed);
        REPLAY_SELF.store(cfg_flag(&s, "replay_self"), Ordering::Relaxed);
        REPLAY_VANILLA.store(cfg_flag(&s, "replay_vanilla"), Ordering::Relaxed);
        ONLY_ULT.store(cfg_flag(&s, "only_ult"), Ordering::Relaxed);
        GRAFT.store(cfg_flag(&s, "graft"), Ordering::Relaxed);
        NULLIFY.store(cfg_flag(&s, "nullify"), Ordering::Relaxed);
        STEAL_CAST.store(cfg_flag(&s, "steal_cast"), Ordering::Relaxed);
        RETARGET.store(cfg_val(&s, "retarget").as_deref() != Some("0"), Ordering::Relaxed);
        ULT_COST.store(cfg_flag(&s, "ult_cost"), Ordering::Relaxed);
        CLONE_GRAFT.store(cfg_flag(&s, "clone_graft"), Ordering::Relaxed);
        BUILD_EFFECT.store(cfg_flag(&s, "build_effect"), Ordering::Relaxed);
        STACK_TRACE.store(cfg_flag(&s, "stack_trace"), Ordering::Relaxed);
        QUEUE_PROBE.store(cfg_flag(&s, "queue_probe"), Ordering::Relaxed);
        BUFF_PROBE.store(cfg_flag(&s, "buff_probe"), Ordering::Relaxed);
        BUFF_GRAFT.store(cfg_flag(&s, "buff_graft"), Ordering::Relaxed);
        BUFF_CALL.store(cfg_flag(&s, "buff_call"), Ordering::Relaxed);
        SLOT_PROBE.store(cfg_flag(&s, "slot_probe"), Ordering::Relaxed);
        SLOT_SWAP.store(cfg_flag(&s, "slot_swap"), Ordering::Relaxed);
        LIVE_ONLY.store(cfg_val(&s, "live_only").as_deref() != Some("0"), Ordering::Relaxed);
        WATCH_STEAL.store(cfg_flag(&s, "watch_steal"), Ordering::Relaxed);
        {
            let want = cfg_val(&s, "force_src").unwrap_or_default();
            let mut g = FORCE_SRC.lock().unwrap_or_else(|x| x.into_inner());
            if *g != want {
                hlog(&format!("[cfg] 강제 강탈 대상 = {:?}
", want));
                *g = want;
            }
        }
        SLOT_FULL.store(cfg_flag(&s, "slot_full"), Ordering::Relaxed);
        CCTX_FIX.store(cfg_flag(&s, "cctx_fix"), Ordering::Relaxed);
        CCTX_ENT.store(cfg_flag(&s, "cctx_ent"), Ordering::Relaxed);
        PROV_SWAP.store(cfg_flag(&s, "prov_swap"), Ordering::Relaxed);
        SLOT_GRAFT.store(cfg_flag(&s, "slot_graft"), Ordering::Relaxed);
        LEGACY_CALL.store(cfg_flag(&s, "legacy_call"), Ordering::Relaxed);
        GRAFT_CT.store(cfg_val(&s, "graft_ct").as_deref() != Some("0"), Ordering::Relaxed);
        // ★★[0.5.7 정정] 기본을 **OFF**로 바꿨다(~~기본 ON~~).
        //   근거: 0.5.7 내내 표 주소가 틀려 마스킹이 **한 번도 걸린 적 없고**, 그 상태로
        //   archer/priest/berserker/knight 인게임 검증이 전부 통과했다 ⟹ 마스킹은 불필요하다.
        //   상수를 고친 지금 기본 ON으로 두면 **한 번도 검증된 적 없는 동작이 갑자기 켜진다.**
        //   원본 충실도 관점에서도 OFF가 옳다 — 마스킹은 돌진 안전판정을 건너뛰게 만든다.
        AI_MASK.store(cfg_val(&s, "ai_mask").as_deref() == Some("1"), Ordering::Relaxed);
        // ★[v128] 원본 충실도 스위치 3종. **기본 전부 OFF** — 켜기 전 동작이 현행과 같아야
        //   기존 인게임 검증(archer/priest/berserker/knight)이 그대로 유효하다. A/B는 하나씩.
        GRAFT_TGT.store(cfg_flag(&s, "graft_tgt"), Ordering::Relaxed);
        // ★★★안전 인터록 — `graft_ct=1`(casting_type 이식)은 **`graft_tgt=1` 없이는 금지**.
        //   RE `2026-08-25_궁시전-사거리게이트-d2c850-d152c0.md` §6(C) 확정:
        //     "casting_type만 바꾸고 casting_target을 원본으로 두면 전제조건 불일치 → Order::None
        //      (접근조차 안 하고 멍때림)"
        //   ⟹ **v97의 궁 시전 0건이 정확히 이 조합이었다.** 같은 실패를 다시 만들지 않도록
        //     여기서 강제로 짝을 맞춘다. 같은 RE의 권고: "desc 7워드를 통째로 복사.
        //     필드 하나만 갈아끼우는 게 모든 실패 모드의 원인."
        if GRAFT_CT.load(Ordering::Relaxed) && !GRAFT_TGT.load(Ordering::Relaxed) {
            GRAFT_TGT.store(true, Ordering::Relaxed);
            if INTERLOCK_LOG.fetch_add(1, Ordering::Relaxed) < 3 {
                hlog("★[인터록] graft_ct=1 인데 graft_tgt=0 이다 — casting_type만 바꾸면 전제조건 불일치로 Order::None(멍때림)이 된다(v97 실패 재현). graft_tgt를 강제로 켠다.
");
            }
        }
        GRAFT_ATK.store(cfg_flag(&s, "graft_atk"), Ordering::Relaxed);
        GRAFT_COOL.store(cfg_flag(&s, "graft_cool"), Ordering::Relaxed);
        ORDER_MASK.store(cfg_flag(&s, "order_mask"), Ordering::Relaxed);
        ULT_META.store(cfg_flag(&s, "ult_meta"), Ordering::Relaxed);
        TAG_SWAP.store(cfg_val(&s, "tag_swap").as_deref() == Some("1"), Ordering::Relaxed);
        EMIT_SWAP.store(cfg_val(&s, "emit_swap").as_deref() != Some("0"), Ordering::Relaxed);
        ULT_CENSUS.store(cfg_val(&s, "ult_census").as_deref() != Some("0"), Ordering::Relaxed);
        ULT_CD_MAX.store(cfg_val(&s, "ult_cd_max").and_then(|v| v.parse::<u64>().ok()).unwrap_or(0), Ordering::Relaxed);
        PROV_KEEPCOOL.store(cfg_val(&s, "prov_keepcool").as_deref() != Some("0"), Ordering::Relaxed);
        LIVE_GATE.store(cfg_val(&s, "live_gate").as_deref() != Some("0"), Ordering::Relaxed);
        ZONE_PROBE.store(cfg_flag(&s, "zone_probe"), Ordering::Relaxed);
        VIEW_FIX.store(cfg_val(&s, "view_fix").as_deref() != Some("0"), Ordering::Relaxed);
        KEEP_STOLEN.store(cfg_flag(&s, "keep"), Ordering::Relaxed);
        AI_PROBE.store(cfg_flag(&s, "ai_probe"), Ordering::Relaxed);
        IDLE_FB.store(cfg_flag(&s, "idle_fallback"), Ordering::Relaxed);
        // ★설치는 1회. cfg로 켜진 것을 본 시점에 시도한다(끄더라도 되돌리지 않는다 — 재시도 가드가 무해).
        if IDLE_FB.load(Ordering::Relaxed) && !IDLE_INSTALLED.swap(true, Ordering::Relaxed) {
            match unsafe { install_idle_fallback() } {
                Ok(a) => hlog(&format!("[install] idle_fallback stub @{:#x} OK\n", a)),
                Err(e) => hlog(&format!("[install] idle_fallback 실패: {}\n", e)),
            }
        }
        ZONE_AT_ENEMY.store(cfg_val(&s, "zone_at").as_deref() == Some("enemy"), Ordering::Relaxed);
        {
            let want = cfg_val(&s, "src").unwrap_or_default();
            let mut g = SRC_NAME.lock().unwrap_or_else(|x| x.into_inner());
            if *g != want { hlog(&format!("[cfg] 재생 소스 챔프 = {:?}\n", if want.is_empty() { "(자동선택)".to_string() } else { want.clone() })); *g = want; }
        }
        MIN_KIDS.store(cfg_val(&s, "min_kids").and_then(|v| v.parse().ok()).unwrap_or(2), Ordering::Relaxed);
        ULT_CD_JUMP.store(cfg_val(&s, "ult_jump").and_then(|v| v.parse().ok()).unwrap_or(500), Ordering::Relaxed);
        ULT_CD_MIN.store(cfg_val(&s, "ult_min").and_then(|v| v.parse().ok()).unwrap_or(600), Ordering::Relaxed);
        fault_drain();
        {
            let (l, b) = (VIEW_LIVE_N.load(Ordering::Relaxed), VIEW_BG_N.load(Ordering::Relaxed));
            if l + b > 0 && (l + b) % 1 == 0 {
                static LAST: AtomicU32 = AtomicU32::new(0);
                if LAST.swap(l + b, Ordering::Relaxed) != l + b {
                    hlog(&format!("[뷰싱크] 표시틱 {} / 백그라운드 {} (live_gate={})
",
                        l, b, LIVE_GATE.load(Ordering::Relaxed)));
                }
            }
        }
        std::thread::sleep(Duration::from_secs(2));
    });
}

// base 궁 apply: rcx=action_data, rdx=world, r8=worldops, r9=target, [rsp+0x28]=casting_ctx. → u64.
type BaseApply = unsafe extern "C" fn(usize, usize, usize, usize, usize) -> u64;
unsafe fn call_base_apply(f_addr: usize, action: usize, world: usize, wops: usize, target: usize, cctx: usize) -> Option<u64> {
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let f: BaseApply = core::mem::transmute(f_addr);
        f(action, world, wops, target, cctx)
    }));
    r.ok()
}

static HJ_N: AtomicU32 = AtomicU32::new(0);
static LOG_N: AtomicU32 = AtomicU32::new(0);
static DIAG: AtomicU32 = AtomicU32::new(0);
static SYLAS_DUMPED: AtomicBool = AtomicBool::new(false);
// 중복 발화 방지: 최근 강탈한 (sylas_key) + 시각. 같은 궁 시전(Grab 여러틱)에 1회만.
static LAST_KEY: AtomicU64 = AtomicU64::new(0);
static LAST_TS:  AtomicU64 = AtomicU64::new(0);
#[inline] fn qpc() -> u64 { let mut c=0i64; unsafe { QueryPerformanceCounter(&mut c); } c as u64 }

unsafe fn hijack_grab(saved: usize, e: usize) {
    // ★진단은 arm 무관하게 수행(프로브 모드) — 강탈 CALL 게이트는 함수 말미로 이동(2026-08-24)
    let rcx = *((saved + 0x28) as *const u64) as usize; // a1 effect_def
    let rdx = *((saved + 0x20) as *const u64) as usize; // a2
    let world = *((saved + 0x18) as *const u64) as usize; // r8 = world(추정)
    let r9    = *((saved + 0x10) as *const u64) as usize; // r9 = WorldOps(추정)
    // ★진단: Grab apply 진입 raw + 게이트 통과 실측 (처음 40건, arm 무관 아래 게이트 전)
    let diag = DIAG.fetch_add(1, Ordering::Relaxed) < 300;
    // ★WorldOps = Grab이 받은 r9 그대로(하드코딩 폐지)
    let wops = r9;
    if diag {
        let s0 = rd_u64(e + 0x28).unwrap_or(0);
        let s1 = rd_u64(e + 0x30).unwrap_or(0);
        let s2 = rd_u64(e + 0x38).unwrap_or(0);
        let s3 = rd_u64(e + 0x40).unwrap_or(0);
        let s4 = rd_u64(e + 0xa0).unwrap_or(0);
        let rslv = rd_u64(r9 + WOPS_RESOLVE_SLOT).unwrap_or(0);
        // ★★[e+0] = return address = Grab apply를 부른 "재생 루프". 이 한 줄이 설계의 핵심 미지수를 푼다.
        let caller = rd_u64(e).unwrap_or(0);
        // 상위 프레임 개요: 스택 0x200B를 훑어 exe 코드 범위 주소를 리턴주소 후보로 수집
        let mut frames = String::new();
        let mut found = 0;
        let mut off = 8usize;
        while off < 0x200 && found < 8 {
            if let Some(v) = rd_u64(e + off) {
                if in_exe(v) && v > exe_base() as u64 + 0x1000 {
                    frames.push_str(&format!(" +{:#x}=RVA:{:#x}", off, rva_of(v)));
                    found += 1;
                }
            }
            off += 8;
        }
        hlog(&format!("[콜러] ret=RVA:{:#x}\n  stack_ret_cands:{}\n", rva_of(caller), frames));
        // ★[rsp+0x30] 키와 casting_ctx+8 키를 둘 다 resolve → 어느 쪽이 시전자인지 실측으로 확정
        let k_a = rd_u64(e + 0x30).unwrap_or(u64::MAX);              // Grab이 "좌표를 읽는" 쪽
        let cc  = rd_u64(e + 0x38).unwrap_or(0) as usize;            // casting_ctx
        let k_b = if cc >= 0x10000 { rd_u64(cc + 8).unwrap_or(u64::MAX) } else { u64::MAX };
        let n_a = if k_a != u64::MAX { let en = resolve(world, k_a); if en != 0 { ent_name(en) } else { None } } else { None };
        let n_b = if k_b != u64::MAX { let en = resolve(world, k_b); if en != 0 { ent_name(en) } else { None } } else { None };
        hlog(&format!("[진입] rcx={:#x} r8={:#x} r9=RVA:{:#x}\n  ★A[rsp+0x30]key={} name={:?}  |  B[cctx+8]key={} name={:?}  (A=좌표제공자=시전자 추정)\n  stk[+28..+40]={:#x} {:#x} {:#x} {:#x}\n",
            rcx, world, rva_of(r9 as u64), k_a as i64, n_a, k_b as i64, n_b, s0, s1, s2, s3));
        // ★사일러스 entity 궁슬롯 apply 실측 (1회, world 순회) — 사일러스 궁이 실제 부르는 apply RVA 확정용
        if !SYLAS_DUMPED.load(Ordering::Relaxed) && world >= 0x10000 {
            let db = rd_u64(world + W_DENSE_BASE).unwrap_or(0) as usize;
            let dl = rd_u64(world + W_DENSE_LEN).unwrap_or(0);
            if db >= 0x10000 && dl >= 1 && dl <= 512 {
                for i in 0..dl as usize {
                    let ent = db.wrapping_add(i * ENT_STRIDE);
                    if !readable(ent, ENT_STRIDE) { continue; }
                    if ent_name(ent).as_deref() == Some("sylas") {
                        SYLAS_DUMPED.store(true, Ordering::Relaxed);
                        let cool = rd_u64(ent + 0xC8).unwrap_or(0);
                        let mut s = format!("★[SYLAS덤프] ent={:#x} ult쿨(+0xC8)={:#x}\n", ent, cool);
                        for (lbl, off) in [("atk",0x118usize),("sk",0x128),("sk2",0x138),("ult",0x148)] {
                            let act = rd_u64(ent + off).unwrap_or(0);
                            let vt = rd_u64(ent + off + 8).unwrap_or(0);
                            let a20 = rd_u64(vt as usize + 0x20).unwrap_or(0);
                            let ad0 = rd_u64(vt as usize + 0xd0).unwrap_or(0);
                            s.push_str(&format!("  {} act={:#x} vt=RVA:{:#x} [vt+0x20]=RVA:{:#x} [vt+0xd0]=RVA:{:#x}\n",
                                lbl, act, rva_of(vt), rva_of(a20), rva_of(ad0)));
                        }
                        // 궁슬롯(+0x148) act가 Combine이면 자식 effect들도: act+8=ptr, act+0x10=count, stride 0x10 {data,vt}
                        let ua = rd_u64(ent + 0x148).unwrap_or(0) as usize;
                        if ua >= 0x10000 {
                            let cp = rd_u64(ua + 8).unwrap_or(0) as usize;
                            let cc = rd_u64(ua + 0x10).unwrap_or(0);
                            s.push_str(&format!("  ult자식 ptr={:#x} count={}\n", cp, cc));
                            if cp >= 0x10000 && cc >= 1 && cc <= 16 {
                                for j in 0..cc as usize {
                                    let cvt = rd_u64(cp + j*0x10 + 8).unwrap_or(0);
                                    let cap20 = rd_u64(cvt as usize + 0x20).unwrap_or(0);
                                    s.push_str(&format!("    child[{}] vt=RVA:{:#x} [vt+0x20]=RVA:{:#x}\n", j, rva_of(cvt), rva_of(cap20)));
                                }
                            }
                        }
                        // ★effect 리스트 후보 스캔: effect 항목 = {data,vtable,tag} stride 0x18.
                        //   entity 저대역에서 "포인터 2개 + 작은 tag" 패턴 Vec{ptr,len,cap}을 찾는다.
                        s.push_str("  [Vec후보] (ptr,len,cap) 형태 필드:\n");
                        for fo in (0x280usize..0x460).step_by(8) {
                            let (Some(p0), Some(l0), Some(c0)) = (rd_u64(ent+fo), rd_u64(ent+fo+8), rd_u64(ent+fo+0x10)) else { continue };
                            if p0 < 0x10000 || l0 == 0 || l0 > 16 || c0 < l0 || c0 > 64 { continue; }
                            // 첫 항목이 {data,vtable} 꼴인가 (vtable이 exe 내부)
                            let d0 = rd_u64(p0 as usize).unwrap_or(0);
                            let v0 = rd_u64(p0 as usize + 8).unwrap_or(0);
                            if !in_exe(v0) || d0 < 0x10000 { continue; }
                            s.push_str(&format!("    +{:#x}: ptr={:#x} len={} cap={} [0].data={:#x} [0].vt=RVA:{:#x} [0].vt+0x20=RVA:{:#x}\n",
                                fo, p0, l0, c0, d0, rva_of(v0), rva_of(rd_u64(v0 as usize + 0x20).unwrap_or(0))));
                        }
                        hlog(&s);
                        break;
                    }
                }
            }
        }
    }
    // Grab apply 정합 확인: r9가 exe 이미지 내 WorldOps 테이블이고 resolve 슬롯이 exe 코드를 가리키는가.
    //   (구 0.5.2판의 `r9 == base+WORLDOPS_RVA` 하드코딩 비교를 버전 독립 sanity로 대체)
    if !in_exe(r9 as u64) { if diag { hlog(&format!("  ✗r9={:#x} !in_exe\n", r9)); } return; }
    match rd_u64(r9 + WOPS_RESOLVE_SLOT) {
        Some(v) if in_exe(v) => {}
        other => { if diag { hlog(&format!("  ✗[r9+{:#x}]={:?} !in_exe\n", WOPS_RESOLVE_SLOT, other)); } return; }
    }
    if world < 0x10000 { if diag { hlog("  ✗world\n"); } return; }
    // 스택 인자: e=orig_rsp. a6=[e+0x30]=target_key, a7=[e+0x38]=casting_ctx
    let target_key = match rd_u64(e + 0x30) { Some(v) => v, None => return };
    let cctx = match rd_u64(e + 0x38) { Some(v) => v as usize, None => return };
    if cctx < 0x10000 { if diag { hlog(&format!("  ✗cctx={:#x}\n", cctx)); } return; }
    let sylas_key = match rd_u64(cctx + 8) { Some(v) => v, None => return };
    // casting_ctx.tag==0 (fresh) 가드
    let tag = rd_u64(cctx).unwrap_or(1) as u32;
    if tag != 0 { if diag { hlog(&format!("  ✗tag={}\n", tag)); } return; }

    // ★caster = sylas 판별 — A/B 두 후보 키 중 sylas인 쪽을 시전자로 채택(방향 실측 전까지 양방 허용).
    //   확정되면 한쪽으로 고정할 것. 로그의 "채택=A/B"가 곧 정답이다.
    let cand_a = resolve(world, target_key);   // [rsp+0x30]
    let cand_b = resolve(world, sylas_key);    // casting_ctx+8
    let na = if cand_a != 0 { ent_name(cand_a) } else { None };
    let nb = if cand_b != 0 { ent_name(cand_b) } else { None };
    let (sylas, x_key, side) =
        if na.as_deref() == Some("sylas") { (cand_a, sylas_key, "A[rsp+0x30]") }
        else if nb.as_deref() == Some("sylas") { (cand_b, target_key, "B[cctx+8]") }
        else {
            if diag { hlog(&format!("  ✗둘 다 sylas 아님: A={:?} B={:?}\n", na, nb)); }
            return;
        };
    hlog(&format!("★★[사일러스 궁 포착] 시전자측={} sylas_ent={:#x} 상대키={}\n", side, sylas, x_key as i64));
    // ★[v94] 자기 궁이 발화한 **바로 그 순간**의 슬롯3·프로바이더 상태를 찍는다.
    //   "교체했는데 자기 궁이 나간다"의 결정적 증거 — 미교체인지 되돌려짐인지가 여기서 갈린다.
    {
        let mine = { PROV_MINE.lock().unwrap_or_else(|x| x.into_inner())
            .iter().find(|q| q.2 == rd_u64(sylas + E_PROV_ULT).unwrap_or(0) as usize).map(|q| q.2) };
        let curp = rd_u64(sylas + E_PROV_ULT).map(|v| v as usize);
        hlog(&format!("  ★그순간 슬롯3 data={:?} vt=RVA:{:#x} gate={:?} | prov={:?} 심은것={:?} ==> {}
",
            rd_u64(sylas + E_SLOT0 + 3 * SLOT_STRIDE),
            rva_of(rd_u64(sylas + E_SLOT0 + 3 * SLOT_STRIDE + 8).unwrap_or(0)),
            rd_u64(sylas + E_ULT_CT), curp, mine,
            match (curp, mine) {
                (Some(a), Some(b)) if a == b => "우리 것 유지",
                (_, Some(_)) => "★되돌려짐",
                _ => "★교체된 적 없음",
            }));
    }
    if diag { hlog(&format!("  ✓sylas={:#x} tk={} → 계속\n", sylas, target_key as i64)); }

    // X = 붙잡은 적 (강탈 대상) — 시전자 반대편 키
    if x_key >= (rd_u64(world + W_SLOT_LEN).unwrap_or(0)) { if diag { hlog(&format!("  ✗xk={} ≥ slot_len\n", x_key as i64)); } return; }
    let x = resolve(world, x_key);
    if x == 0 || x == sylas { if diag { hlog(&format!("  ✗X resolve(xk={})={:#x} (0 or ==sylas)\n", x_key as i64, x)); } return; }
    let xname = ent_name(x).unwrap_or_default();
    // ★★[v86] 이 값은 생존이 아니라 **cc_immune**이다(RE 2026-08-25). 이름과 로그를 바로잡는다.
    let xcc = rd_u8(x + E_CC_IMMUNE).map(|v| v as i32).unwrap_or(-1);
    let xalive = if is_alive(x) { 0 } else { 1 };   // 0=생존(기존 코드 관용 유지)
    if diag { hlog(&format!("  ✓X={:#x} name={:?} hp={:?} cc_immune(+{:#x})={}\n",
        x, xname, rd_u64(x + E_HP), E_CC_IMMUNE, xcc)); }
    // X 생존 확인 (진짜 생존 = hp != 0)
    if xalive != 0 { if diag { hlog("  ✗X 死(hp=0)\n"); } return; }
    // ★CC 면역이면 Grab/Stun이 게임 코드에서 조용히 무발동한다 — 강탈은 되지만 사슬은 안 걸린다.
    if xcc != 0 { hlog(&format!("  ⚠X {:?}가 CC 면역 상태(+0x468={}) — Grab/Stun 무발동 예상\n", xname, xcc)); }

    // ★★★[v73] **원작형 강탈** (유저 확정 2026-08-25)
    //   사일러스 R = 사슬로 적을 끌어당기는 자기 궁이 **그대로 발동**하고, 그 순간
    //   **붙잡은 대상 X의 궁 슬롯(3번)** 을 보유한다. 다음 R에서 그 궁이 발동하고 원복된다.
    //   ⟹ "무엇을 뺏을지"가 플레이(누구를 붙잡았나)로 결정된다.
    //   ~~구: 적이 궁 쓰는 것을 목격하면 자동 강탈~~ → 사일러스 자기 궁이 아예 안 나가는 문제가 있었다.
    if SLOT_SWAP.load(Ordering::Relaxed) && ARMED.load(Ordering::Relaxed) {
        let busy = { HELD.lock().unwrap_or_else(|x| x.into_inner()).iter().any(|h| h.0 == world) };
        if busy {
            hlog(&format!("  [강탈skip] 이미 보유 중 (world {:#x})\n", world));
        } else {
            let want = { SRC_NAME.lock().unwrap_or_else(|x| x.into_inner()).clone() };
            if !(want.is_empty() || want == xname) {
                hlog(&format!("  [강탈skip] 붙잡은 대상 {:?} — src={:?} 지정과 불일치\n", xname, want));
            } else {
                // ★force_src 지정시: 붙잡은 대상 대신 **그 이름의 챔프**를 world에서 찾아 빼앗는다.
                let forced = { FORCE_SRC.lock().unwrap_or_else(|x| x.into_inner()).clone() };
                let (src_ent, src_name) = if forced.is_empty() { (x, xname.clone()) } else {
                    let slot_len = rd_u64(world + W_SLOT_LEN).unwrap_or(0).min(2048);
                    let mut found = 0usize;
                    for k in 0..slot_len {
                        let e2 = resolve(world, k);
                        if e2 < 0x10000 { continue; }
                        if ent_name(e2).as_deref() == Some(forced.as_str()) { found = e2; break; }
                    }
                    if found == 0 {
                        // ★즉시 탐색 실패(대개 그 챔프가 지금 죽어 있음) → 캐시 폴백
                        match cached_slot3(world, &forced) {
                            Some((cd, cv, cg)) => {
                                let inc = arc_incref(cd);
                                {
                                    let mut g = HELD.lock().unwrap_or_else(|x| x.into_inner());
                                    if g.len() >= HOLD_MAX { g.remove(0); }
                                    g.push((world, forced.clone(), cd, cv));
                                }
                                {
                                    let mut gg = HELD_GATE.lock().unwrap_or_else(|x| x.into_inner());
                                    gg.retain(|q| q.0 != world);
                                    gg.push((world, cg));
                                }
                                hlog(&format!("★★★[강탈·캐시] {:?}가 지금 없어 **캐시된 궁** 사용 | data={:#x} vt=RVA:{:#x} gate={} arc_inc={}
",
                                    forced, cd, rva_of(cv as u64), cg, inc));
                            }
                            None => {
                                hlog(&format!("  [강탈skip] force_src={:?} — world에도 캐시에도 없음
", forced));
                            }
                        }
                        return;
                    }
                    (found, forced.clone())
                };
                let xname = src_name;
                let x = src_ent;
                let b = x + E_SLOT0 + 3 * SLOT_STRIDE;
                let d = rd_u64(b + SLOT_DATA).unwrap_or(0) as usize;
                let v = rd_u64(b + SLOT_VT).unwrap_or(0) as usize;
                // ★대상도 슬롯3이 실제로 쓰이는 상태(skill_cnt>=5)여야 그 자리가 진짜 궁이다.
                let xskc = rd_u64(x + E_SKILL_CNT).unwrap_or(0);
                if xskc < 5 {
                    hlog(&format!("  [강탈skip] {:?} skill_cnt={} (<5) — 슬롯3이 아직 궁이 아니다
", xname, xskc));
                } else if !effect_sane(d, v) {
                    hlog(&format!("  [강탈실패] {:?}의 궁 슬롯이 무효 (data={:#x} vt=RVA:{:#x})\n",
                        xname, d, rva_of(v as u64)));
                } else {
                    let inc = arc_incref(d);
                    {
                        let mut g = HELD.lock().unwrap_or_else(|x| x.into_inner());
                        if g.len() >= HOLD_MAX { g.remove(0); }
                        g.push((world, xname.clone(), d, v));
                        drop(g);
                        let gt = rd_u64(b + SLOT_GATE).unwrap_or(0) as u32 as i32;
                        let mut gg = HELD_GATE.lock().unwrap_or_else(|x| x.into_inner());
                        gg.retain(|q| q.0 != world);
                        gg.push((world, gt));
                    }
                    hlog(&format!("★★★[강탈·원작형] 사일러스가 붙잡은 {:?}의 궁을 강탈 | data={:#x} vt=RVA:{:#x} apply=RVA:{:#x} arc_inc={}\n{}",
                        xname, d, rva_of(v as u64),
                        rva_of(rd_u64(v + EFF_APPLY).unwrap_or(0)), inc,
                        dump_children(eff_self(d, v), "강탈한 궁", &xname)));
                }
            }
        }
    }

    // ★X 궁 슬롯 전수 덤프(강탈 대상 계약 실측) — 액션 슬롯 4칸의 vt/apply를 전부 남긴다
    {
        let mut sx = format!("  [X덤프] name={:?} ent={:#x}\n", xname, x);
        for (lbl, off) in [("+0x118",0x118usize),("+0x128",0x128),("+0x138",0x138),("+0x148",0x148)] {
            let act = rd_u64(x + off).unwrap_or(0);
            let vt  = rd_u64(x + off + 8).unwrap_or(0);
            sx.push_str(&format!("    {} act={:#x} vt=RVA:{:#x} [vt+0x20]=RVA:{:#x} [vt+0xd0]=RVA:{:#x} [vt+0x10](align)={:#x}\n",
                lbl, act, rva_of(vt), rva_of(rd_u64(vt as usize + 0x20).unwrap_or(0)),
                rva_of(rd_u64(vt as usize + 0xd0).unwrap_or(0)), rd_u64(vt as usize + 0x10).unwrap_or(0)));
        }
        hlog(&sx);
    }
    // X 궁 apply + action (base effect-action: [X+0x150 vtable +0xd0] = apply)
    let x_action = match rd_u64(x + S_ULT_ACTION) { Some(v) => v as usize, None => return };
    let x_vtbl   = match rd_u64(x + S_ULT_VTBL) { Some(v) => v as usize, None => return };
    if diag { hlog(&format!("  X_action={:#x} X_vtbl=RVA:{:#x} [vt+0xd0]=RVA:{:#x} [vt+0x20]=RVA:{:#x}\n",
        x_action, rva_of(x_vtbl as u64), rva_of(rd_u64(x_vtbl + 0xd0).unwrap_or(0)), rva_of(rd_u64(x_vtbl + 0x20).unwrap_or(0)))); }
    if x_action < 0x10000 || !in_exe(x_vtbl as u64) { if diag { hlog("  ✗x_action/vtbl\n"); } return; }
    let x_apply = match rd_u64(x_vtbl + V_APPLY_BASE) { Some(v) => v as usize, None => return };
    if !in_exe(x_apply as u64) { if diag { hlog(&format!("  ✗x_apply=RVA:{:#x} !in_exe\n", rva_of(x_apply as u64))); } return; }

    // 중복 방지: 같은 sylas_key + 300ms 내 재발화 skip (Grab이 여러 틱 불림)
    let now = qpc();
    if LAST_KEY.load(Ordering::Relaxed) == sylas_key {
        let dt = now.wrapping_sub(LAST_TS.load(Ordering::Relaxed));
        if dt < 500_000_000 { return; } // QPC~10MHz 가정 넉넉히 (실제 dedup은 tick 게이트로 충분)
    }
    LAST_KEY.store(sylas_key, Ordering::Relaxed);
    LAST_TS.store(now, Ordering::Relaxed);

    // ★arm 게이트 = 여기(진단은 위에서 이미 다 찍혔다). arm=0이면 "강탈 직전까지 도달"만 기록하고 반환.
    if !ARMED.load(Ordering::Relaxed) {
        if LOG_N.fetch_add(1, Ordering::Relaxed) < 60 {
            hlog(&format!("[준비완료·arm=0] sylas={:#x} X={} x_apply=RVA:{:#x} x_action={:#x} (cfg arm=1이면 여기서 강탈)\n",
                sylas, xname, rva_of(x_apply as u64), x_action));
        }
        return;
    }
    // ★강탈: X 궁 base apply를 sylas caster로 CALL. casting_ctx=cctx(그대로={tag0,sylas_key}). target=X.
    //   r8 = Grab이 받은 WorldOps(r9=wops) 그대로 전달.
    // ★★★[v104] 구세대 "강탈 CALL" 경로 — **slot_graft로 대체된 폐기 경로**인데
    //   전용 게이트 없이 공용 `arm`에 매달려 있어, slot_swap=0인데도
    //   **사일러스가 사슬을 걸 때마다 매번 남의 궁 base apply를 shadow-CALL** 하고 있었다
    //   (2026-08-25 감사 H6). 게임 함수 shadow-call은 AV 위험 경로다 — 기본 OFF로 격리한다.
    if !LEGACY_CALL.load(Ordering::Relaxed) {
        if diag { hlog("  [legacy] 구 강탈 CALL 경로 차단(legacy_call=0) — slot_graft가 정본
"); }
        return;
    }
    let ret = call_base_apply(x_apply, x_action, world, wops, x, cctx);
    let n = HJ_N.fetch_add(1, Ordering::Relaxed);
    if LOG_N.fetch_add(1, Ordering::Relaxed) < 60 {
        hlog(&format!("[강탈#{}] sylas={:#x} ← {} 궁 apply=RVA:{:#x} action={:#x} ret={:?}\n",
            n, sylas, xname, rva_of(x_apply as u64), x_action, ret.map(|v| v & 0xffff_ffff)));
    }
}

// ────────────────────────────────────────────────────────────────────────
// ★재생 루프(Combine apply) 훅 — 관측 전용 v14
//   목적 ①사일러스 궁 Combine의 자식 구성 실측 ②바닐라 챔프가 실제로 시전한 Combine을
//   caster별로 캡처(= "하드코딩 궁을 재생"할 재료). entity 액션 슬롯(+0x118~+0x148)은
//   챔프마다 유효하지 않음이 실측돼(archer 4칸 전부 0) 그 경로를 대체한다.
static CB_DIAG: AtomicU32 = AtomicU32::new(0);
static SEEN: Mutex<Vec<String>> = Mutex::new(Vec::new());

unsafe fn dump_children(self_ptr: usize, tag: &str, who: &str) -> String {
    let ptr = rd_u64(self_ptr + 8).unwrap_or(0) as usize;
    let len = rd_u64(self_ptr + 0x10).unwrap_or(0);
    let mut s = format!("  [{}] caster={} self={:#x} children ptr={:#x} len={}\n", tag, who, self_ptr, ptr, len);
    if ptr < 0x10000 || len == 0 || len > 32 { s.push_str("    (자식 배열 무효)\n"); return s; }
    for i in 0..len as usize {
        let it = ptr + i * EFF_STRIDE;
        let data = rd_u64(it).unwrap_or(0);
        let vt   = rd_u64(it + 8).unwrap_or(0) as usize;
        let ap   = rd_u64(vt + EFF_APPLY).unwrap_or(0);
        let al   = rd_u64(vt + EFF_ALIGN).unwrap_or(0);
        s.push_str(&format!("    [{}] data={:#x} vt=RVA:{:#x} apply=RVA:{:#x} align={:#x}{}\n",
            i, data, rva_of(vt as u64), rva_of(ap), al,
            if rva_of(ap) == COMBINE_RVA as u64 { "  <<중첩 Combine>>" } else { "" }));
    }
    s
}

/// ★effect data = **Arc** 레이아웃 {strong@+0, weak@+8, payload@+0x10} (0.5.6 실측).
///   근거 ①생성부 0x182b9b0: `[rax]=1; [rax+8]=1; payload는 +0x10부터`
///        ②재생 루프 0x1802680: `self = data + ((align-1)&~0xf) + 0x10`
///   ⟹ strong count를 원자적으로 올려두면 게임이 drop해도 해제되지 않는다.
///   (v15 크래시 원인 = 이 처리를 안 해서 캡처 포인터가 재생 시점에 dangling)
unsafe fn arc_incref(data: usize) -> bool {
    if data < 0x10000 || !readable(data, 16) { return false; }
    let sc = rd_u64(data).unwrap_or(0);
    if sc == 0 || sc > 0x100000 { return false; }   // 정상 refcount 범위가 아니면 건드리지 않는다
    core::arch::asm!("lock add qword ptr [{p}], 1", p = in(reg) data, options(nostack));
    true
}

// ── 재생용 캡처: 바닐라 챔프가 시전한 Combine 중 "자식이 가장 많은 것" 1개를 보관.
//    (궁 판별은 2단계 과제. 최소 데모에선 자식 수 최대 = 궁일 확률이 높다는 휴리스틱을 쓴다.)
// ★world까지 저장 — 게임은 백그라운드 sim을 rayon으로 병렬 실행하므로 다른 world의 재료를
//   사일러스가 있는 world로 태우면 문맥이 어긋난다(2026-08-24 규명).
static CAPTURED: Mutex<Option<(String, Vec<(usize, usize)>, usize, bool)>> = Mutex::new(None); // (챔프, 자식, world, 궁여부)
static REPLAY_N: AtomicU32 = AtomicU32::new(0);
/// ★사일러스가 실제로 있는 world(= 화면에 보이는 경기). 게임은 백그라운드 sim을 rayon으로 병렬
///   실행하므로, 이 world 밖에서 캡처한 재료는 재생 시점에 문맥이 어긋난다(2026-08-24 실측:
///   캡처는 전부 백그라운드 world, 사일러스 궁은 다른 world → `world 불일치 skip` 9건, 재생 0건).
static SYLAS_WORLD: AtomicU64 = AtomicU64::new(0);
/// 사일러스의 SlotMap key(= casting_ctx+8에 들어가는 시전자 식별자)
static SYLAS_KEY: AtomicU64 = AtomicU64::new(u64::MAX);
/// ★궁 판별 = **caster의 ult 쿨(entity+0xC8) 리셋 감지**.
///   궁을 쏘면 이 값이 큰 폭으로 올라간다(사일러스 실측 0x304·0x328). 직전 값보다 크게 뛴 프레임에
///   그 caster가 시전한 Combine = 궁. `CasterViewEffect 포함` 휴리스틱은 무효였다
///   (crossbowman·illusionist 같은 소형 스킬도 연출 effect를 가진다 — 2026-08-24 실측).
const E_ULT_CD: usize = 0xC8;
/// entity의 활성 effect 리스트(버프·디버프·스턴이 여기 쌓인다). Stun apply 디컴에서 확인:
/// `[entity+0x2c8]` = ptr(stride 0x28), `[entity+0x2d0]` = len.
/// ★HP만으로는 **버프형 궁**의 효과를 못 잡으므로 이 길이 변화를 함께 본다.
const E_EFF_PTR: usize = 0x2c8;
const E_EFF_LEN: usize = 0x2d0;

// ────────────────────────────────────────────────────────────────────────
// ★★[v51] 버프 리스트 = Vec{cap@+0x2d8, ptr@+0x2e0, len@+0x2e8}, 원소 = **0x120바이트 값**.
//   근거 = AddCasterBuff apply(0x15afd30) 말미의 `FUN_1417f9390(entity, 사본)` 실바이트(2026-08-24):
//     [ent+0x6b8]=1(스탯 dirty) / len=[ent+0x2e8] / if len==[ent+0x2d8] grow(ent+0x2d8)
//     memcpy([ent+0x2e0]+len*0x120, src, 0x120) / [ent+0x2e8]=len+1 / 이후 전 버프 SIMD 재합산
//   ⚠Ghidra는 memcpy를 noreturn으로 오판해 그 뒤 CALL을 통째로 누락한다 — 원바이트 확인 필수.
//   ★이 층은 게임 자신이 **값 복사**로 옮긴다 ⟹ "effect 객체가 시전 문맥에 묶여 있어" 막혔던
//   6경로(캡처재생·graft·복제·생성기)와 구조가 다르다. 값이면 dangling도 문맥 귀속도 없다.
const E_BUF_CAP: usize = 0x2d8;
const E_BUF_PTR: usize = 0x2e0;
const E_BUF_LEN: usize = 0x2e8;
const BUF_STRIDE: usize = 0x120;
const E_STAT_DIRTY: usize = 0x6b8;   // 1바이트 필드(상위 7B는 이웃) — u8 폭으로만 건드릴 것
const ADDBUFF_RVA: usize = 0x139a5d0;   // AddCasterBuff effect의 apply
/// ★게임의 버프 push 함수. `f(rcx=entity, rdx=src_0x120)` — memcpy·len++ 뿐 아니라
///   **버프 전량 재합산 결과를 `+0x3c8~+0x489`에 기록 → `0x17ea5f0`(스탯 재계산) 호출 →
///   최대HP 변화에 맞춰 현재HP(`+0x670`) 비례 조정**까지 한다(0x17f9590~ 디스어셈).
///   ⟹ 우리 재구현(`push_buff`)은 앞 절반뿐이라 **리스트에는 남지만 스탯에 반영되지 않았다**(v55 실측).
const BUFFPUSH_RVA: usize = 0x15925c0;
// ────────────────────────────────────────────────────────────────────────
// ★★[v56] entity **스킬 슬롯 4개** — 정적 RE로 확정(2026-08-24, `0x17eabc0` 틱 함수 분해).
//   틱 함수는 `[ent+0x68]`(엔티티 종류, 20종 점프테이블)으로 갈라지고, **0xd = 데이터챔프**(사일러스).
//   그 arm에서 다시 `[ent+0x70]`(3~6) 2차 점프테이블로 갈라지며, 각 arm이 **정확히 같은 형태**로
//   슬롯 하나를 읽는다 ⟹ 슬롯 = `ent + 0x490 + n*0x38` (n=0..3).
//     state 3 → +0x490 (게이트 +0x4c0, param +0x4b0, 콜백 +0x570/+0x578)
//     state 4 → +0x4c8 (게이트 +0x4f8, param +0x4e8, 콜백 +0x580/+0x588)
//     state 5 → +0x500 (게이트 +0x530, param +0x520, 콜백 +0x590/+0x598) ※[ent+0x5c8]>=3일 때만
//     state 6 → +0x538 (게이트 +0x568, param +0x558, 콜백 +0x5a0/+0x5a8) ※[ent+0x5c8]>=5일 때만
//   그리고 틱은 슬롯의 `{data,vtable}`를 **Arc inc → 로컬 실행Vec(stride 0x38)에 push → 나중에
//   `vtable+0x20`(apply) 호출 → Arc dec** 한다(콜사이트 = 우리가 계속 보던 `RVA:0x17ef3b0`).
//   ⟹ **슬롯의 effect 페어를 바꾸면 게임이 정규 경로로 그것을 실행한다** = "보유 후 사용"의 정공 경로.
//   ⚠0.5.2 시절 "궁 슬롯 +0x148/+0x150은 재조립에 덮인다"는 판정은 **다른 오프셋을 본 것**이다.
const E_SLOT0:       usize = 0x490;
const SLOT_STRIDE:   usize = 0x38;
const SLOT_DATA:     usize = 0x00;
const SLOT_VT:       usize = 0x08;
const SLOT_PARAM:    usize = 0x20;
const SLOT_GATE:     usize = 0x30;   // u32, -1이면 그 슬롯 비활성
const E_KIND:        usize = 0x68;   // 0xd = 데이터챔프
const E_ACTIVE_SLOT: usize = 0x70;   // 3~6 = 슬롯 0~3 (그 외 = 시전 중 아님)
const E_SKILL_CNT:   usize = 0x5c8;
/// `slot_probe=1` = 스킬 슬롯 4칸 전수 덤프(어느 칸이 궁인지 확정용).
static SLOT_PROBE: AtomicBool = AtomicBool::new(false);
/// `watch_steal=1` = 구 경로(적 궁 목격 시 자동 강탈). 기본 OFF — 원작형(Grab 강탈)이 정본.
static WATCH_STEAL: AtomicBool = AtomicBool::new(false);
/// ★`force_src=<champ>` = **붙잡은 대상과 무관하게** 그 챔프의 궁을 빼앗는다(검증용).
///   같은 world에서 이름으로 찾아 그 entity의 슬롯3을 가져온다.
static FORCE_SRC: Mutex<String> = Mutex::new(String::new());
static SLOT_N: AtomicU32 = AtomicU32::new(0);

// ────────────────────────────────────────────────────────────────────────
// ★★[v57] 슬롯 교체 = "보유 → 임의 시점 사용 → 원복"의 정공 구현.
//   effect를 만들지도 옮기지도 않는다. 게임이 이미 들고 있는 Arc를 **슬롯이라는 정규 보관처에서
//   정규 보관처로** 옮길 뿐이고, 실행은 전적으로 게임이 한다(틱이 슬롯을 읽어 push→apply).
/// `slot_swap=1` = 강탈·장전·발동감지·원복 상태기계를 돌린다.
static SLOT_SWAP: AtomicBool = AtomicBool::new(false);

// =====================================================================================
// [v90] 프로바이더 교체 = 정본 개입 (RE 2026-08-25 배치 20건 결론)
//
//   왜 슬롯 교체로는 안 됐는가:
//   (1) 연출/이름은 슬롯이 아니라 **프로바이더 vt+0x78(action_name)** 에서 나온다
//       (`0x14c54c7`). 슬롯만 바꾸면 "효과는 새 궁, 애니는 사일러스" = 화면상 자기 궁.
//   (2) 쿨·충전수·시전시간도 전부 프로바이더(`vt+0x90/0xa8/0x80`)에서 온다.
//   (3) 장전 트리거가 Combine apply 안에만 있어 재장전 기회를 놓쳤다.
//
//   프로바이더는 **Arc가 아니라 Box**다 → 포인터 복사 = double-free.
//   반드시 `__clone_box`(vt+0x48)로 실복사한다. 참조구현 = 게임 자신의 Entity deep-copy.
// =====================================================================================
const E_PROV_ULT:  usize = 0x5a0;   // Box<dyn DataAction> data
const E_PROV_ULTV: usize = 0x5a8;   // 그 vtable
const PV_CLONE:    usize = 0x48;    // __clone_box(&self) -> *mut ()
const PROV_COOL:   usize = 0x170;   // cooltime
const PROV_USES:   usize = 0x178;   // cooltime_use_count
const RVA_ASSEMBLE: usize = 0x1590010;  // 슬롯 4칸 재조립(프로바이더 기준)
const RVA_BOXDROP:  usize = 0xec870;    // drop_in_place<Box<dyn>> (인자 = 16B 쌍의 주소)

static PROV_SWAP: AtomicBool = AtomicBool::new(false);
/// ★정본 경로: 슬롯 필드 이식(프로바이더는 그대로).
static SLOT_GRAFT: AtomicBool = AtomicBool::new(false);
/// 뺏은 궁의 쿨을 **사일러스 것으로** 유지할지. 0이면 피해자 쿨을 그대로 쓴다.
static PROV_KEEPCOOL: AtomicBool = AtomicBool::new(true);
/// 우리가 피해자에게서 한 번 복제해 영구 보관하는 **템플릿 Box**. (data, vtable)
static PROV_TMPL: Mutex<Option<(usize, usize)>> = Mutex::new(None);
/// 템플릿을 뜬 피해자 이름(로그·재확인용)
static PROV_TMPL_WHO: Mutex<String> = Mutex::new(String::new());
/// 우리가 심어 놓은 프로바이더 data 포인터 (ent 단위). 리스폰하면 ent가 바뀌어 자동 무효.
static PROV_MINE: Mutex<Vec<(usize /*world*/, u64 /*key*/, usize /*prov_data*/)>> = Mutex::new(Vec::new());
static PROV_N: AtomicU32 = AtomicU32::new(0);
static PROV_TICK: AtomicU32 = AtomicU32::new(0);
static PROV_DIAG: AtomicU32 = AtomicU32::new(0);
/// 궁 잔여 쿨 상한(검증용). 0 = 사용 안 함.
static ULT_CD_MAX: AtomicU64 = AtomicU64::new(0);

// =====================================================================================
// ★★★[v97] 슬롯 필드 이식 = 정본 (RE 2026-08-25 "네이티브 프로바이더 혼합" 결론)
//
//   ★진범 확정: 네이티브 챔프(knight)의 프로바이더는 `casting_target`을 **상수 1(AllyChampion)**
//   으로 하드코딩한다(vt+0xb0 = `0x1107b40` = `mov eax,1; ret`). 그런데 궁 AI는 **항상 적을**
//   target으로 넘기므로, 타깃 필터 `FUN_1417faeb0`가 매번 0을 반환 →
//   궁 오더 빌더 `0x140e5bf70`이 `Order::None(-1)`을 낸다.
//   ⟹ **궁을 못 쓰고, 그 틱엔 오더가 없어 "멍때린다". 두 증상이 한 원인이었다.**
//
//   v89의 "슬롯 0x38 전량 복사"도 같은 이유로 죽었다(casting_target까지 1로 덮음)
//   — 그래서 그때 `강탈궁 발동`이 0건이었다.
//
//   ⟹ 정답: **필드 단위 이식.**
//     공여자에서 가져올 것 = data/vt(효과) · range · growth · start_timing · **casting_type**
//     피해자(사일러스) 것으로 남길 것 = ★**casting_target(+0x28)** · attack_type(+0x2c)
//   프로바이더(`ent+0x5a0`)는 **건드리지 않는다** ⟹ 연출(action_name)·쿨·타깃 규칙이 사일러스 것으로 유지된다.
// =====================================================================================
static SLOT_TMPL: Mutex<Option<(Vec<u64>, String)>> = Mutex::new(None);
static SLOT_MINE: Mutex<Vec<(usize, u64, usize)>> = Mutex::new(Vec::new());
static GRAFT_N: AtomicU32 = AtomicU32::new(0);
static AI_DIAG: AtomicU32 = AtomicU32::new(0);
static ETICK_N: AtomicU32 = AtomicU32::new(0);
static CCTX_LOG: AtomicU32 = AtomicU32::new(0);
static GRAFT_FIRE: AtomicU32 = AtomicU32::new(0);
/// 공여자 템플릿 확보 실패 전용 카운터(공용 prov_skip 상한에 안 묻히게)
static TMPL_FAIL_N: AtomicU32 = AtomicU32::new(0);
static TMPL_FAIL_LAST: Mutex<String> = Mutex::new(String::new());
/// 폐기된 구 강탈 CALL 경로. 기본 OFF — cfg `legacy_call=1`로만 켠다.
static LEGACY_CALL: AtomicBool = AtomicBool::new(false);
/// 공여자의 casting_type을 가져올지. 0이면 사일러스 것을 유지한다(A/B 격리용).
static GRAFT_CT: AtomicBool = AtomicBool::new(true);
/// ★★[v99] **공여자의 casting_type**. 슬롯에는 사일러스 것(0=Targeting)을 남겨야 AI가
///   궁 오더를 만들지만(ct=1이면 Order::None), 이펙트는 공여자 계약(장판=tag2)을 요구한다.
///   ⟹ 슬롯 게이트는 사일러스, **cctx만 공여자 계약대로** 따로 공급한다. u32::MAX = 미설정.
static GRAFT_SRC_CT: AtomicU32 = AtomicU32::new(u32::MAX);
/// ★[v128] 공여자 궁의 casting_target(slot+0x28) / attack_type(slot+0x2c) / cooltime.
///   u32::MAX·0 = 미확보. 원본 충실도 개선(§원본충실 로드맵)의 입력.
static GRAFT_SRC_TGT:  AtomicU32 = AtomicU32::new(u32::MAX);
static GRAFT_SRC_ATK:  AtomicU32 = AtomicU32::new(u32::MAX);
static GRAFT_SRC_COOL: AtomicU64 = AtomicU64::new(0);
/// cfg `graft_tgt=1` — casting_target을 공여자 것으로 이식.
///   RE(2026-08-25 casting_target-AI후보풀-전수) = 이 필드는 **AI 후보 풀만** 정하고
///   발동 경로(시전 개시·틱 arm·effect apply) 어디서도 읽히지 않는다 ⟹ 런타임 부작용 없음.
static GRAFT_TGT:  AtomicBool = AtomicBool::new(false);
/// cfg `graft_atk=1` — attack_type을 공여자 것으로 이식.
static GRAFT_ATK:  AtomicBool = AtomicBool::new(false);
/// cfg `graft_cool=1` — 궁 쿨을 공여자 것으로(사일러스 900 → 공여자 900~3000).
///   쿨은 슬롯이 아니라 **프로바이더 +0x170**에서 온다(RE 조립기-콜러전수-쿨관리).
static GRAFT_COOL: AtomicBool = AtomicBool::new(false);
/// 사일러스 프로바이더의 원래 cooltime(복원용). 0 = 미확보.
static SY_COOL_ORIG: AtomicU64 = AtomicU64::new(0);
/// 인터록 로그 상한용.
static INTERLOCK_LOG: AtomicU32 = AtomicU32::new(0);
/// cfg `order_mask=1` — ★안 A: 오더 팩토리 술어 `+0xf0`·`+0x110`을 기본 스텁(false)으로 덮어
///   **ct=1/2 궁도 ct=0과 같은 오더 경로**를 타게 한다. 그 둘이 true면 오더 팩토리가 궁을
///   **Move 오더로 강등**해 궁이 영원히 안 나간다(v97 "시전 0건"의 유일하게 남은 설명).
///   ⚠비용: AI가 그 궁을 "범위/스킬샷"으로 안 보게 되므로 위치 선정이 원본과 달라진다.
static ORDER_MASK: AtomicBool = AtomicBool::new(false);
/// cfg `ult_meta=1` — 경기 시작 시 **그 판의 전 챔피언 궁 메타**를 1회 덤프.
///   정적 RE(디컴)의 지상검증 짝 — 어느 챔프가 아군/자기 대상 궁인지, 돌진궁인지를 실측으로 확정한다.
static ULT_META: AtomicBool = AtomicBool::new(false);
/// 이미 덤프한 world(중복 방지).
static META_DONE: Mutex<Vec<usize>> = Mutex::new(Vec::new());

/// ★★★[v105] AI 판정 3슬롯 마스킹 스위치. cfg `ai_mask`(기본 ON, "0"이면 끈다).
static AI_MASK: AtomicBool = AtomicBool::new(true);
/// "AI 판정 3슬롯"의 **기본 구현 스텁**. effect vtable 표 59종의 **최빈값**으로 뽑는다.
/// ⚠v106 버그: 사일러스 자기 궁 vtable에서 뽑았는데 그건 **Combine의 자식 집계기**였다
///   (`+0x48=0x1800c00`, `+0x58=0x18009e0`). 공여자도 Combine이라 덮어써도 **완전한 no-op**.
///   "자식이 전부 기본이라 **결과**가 None"인 것과 "**슬롯**이 기본 스텁"은 다른 얘기다.
static SY_VT_DEF: Mutex<Option<[u64; 5]>> = Mutex::new(None);
/// effect vtable 표: `EFF_VT_BASE + kind*EFF_VT_STRIDE`.
/// ★★★[0.5.7 정정 2026-08-29] ~~`0x34200d8` / 59개 / stride 0x118~~ 는 **0.5.6 값**이고
///   0.5.7에서 그 주소는 effect vtable 표가 아니다. 실측 증거 = 로그
///   `[AI마스크·포기] 슬롯 9의 최빈값이 과반 미달 1/59 — 표 주소 의심`, 성공 로그 **0건**
///   ⟹ **0.5.7 내내 AI 마스킹이 완전히 죽어 있었다**(soft-fail이라 조용히 원본을 그대로 썼다).
const EFF_VT_BASE: usize = 0x33FF9C8;
const EFF_VT_N: usize = 57;
const EFF_VT_STRIDE: usize = 0x120;

// ───────────────────────── 궁 시전 센서스 (v108) ─────────────────────────
// ★왜: "궁을 잘 안 쓴다"를 눈으로 세는 걸 반복하면 판정이 안 난다.
//   **같은 경기 안에서** 사일러스 vs 상대 광전사 vs 나머지 8명의 시전 횟수를 세면 한 판에 끝난다.
//     사일러스 > 광전사  ⟹ AI 마스킹이 효과 있음(남은 병목 = 점수 경쟁)
//     사일러스 ≈ 광전사  ⟹ 마스킹 무의미, 병목은 점수/사거리
//     둘 다 ≈ 다른 챔프  ⟹ 원래 이런 게임(우리가 고칠 문제 아님)
/// ⚠`world + 0x1e0 + team*0x28 + slot*8`(RE `궁시전-사거리게이트` §1 ③)는 **AI 컨텍스트(AiWorldCtx)**
///   기준 레이아웃이다. 우리가 든 **sim World 포인터에는 맞지 않는다**(2026-08-25 실측:
///   그 자리에 포인터와 SlotMap 키(0x1e,0x22,0x23)가 섞여 있고 kind=26/27/28).
///   ⟹ 챔프 열거는 모드의 검증된 경로(`resolve` + `is_champion`)를 쓴다.
/// 챔프 키 목록 (world, key, 이름) — 전수 스캔은 비싸므로 드물게 재구축한다.
static CENSUS_KEYS: Mutex<Vec<(usize, u64, String)>> = Mutex::new(Vec::new());
static ULT_CENSUS: AtomicBool = AtomicBool::new(true);
static CENSUS_N: AtomicU32 = AtomicU32::new(0);
static CENSUS_DIAG: AtomicU32 = AtomicU32::new(0);
static CENSUS_LAST: Mutex<String> = Mutex::new(String::new());
/// 사일러스 시전 중 관측한 rush_state 변종 분포 (변종, 횟수)
static RUSH_SEEN: Mutex<Vec<(i64, u32)>> = Mutex::new(Vec::new());
static RUSH_LOG: AtomicU32 = AtomicU32::new(0);
/// 돌진 **사건** 수 (변종, 횟수) — None→변종 전이만. 표집 수(RUSH_SEEN)와 구분할 것.
static RUSH_EVENT: Mutex<Vec<(i64, u32)>> = Mutex::new(Vec::new());
static RUSH_PREV: Mutex<Vec<(usize, u64, i64)>> = Mutex::new(Vec::new());
/// 강탈 궁 Combine 발동 로그 카운터(v114)
static SY_FIRE_LOG: AtomicU32 = AtomicU32::new(0);
/// 사일러스 Combine apply 전체 분포 (말단 자식 수, 궁시전중(state6)인가, 횟수) — 상한 표본 사각지대 방지
static SY_COMBINE_TALLY: Mutex<Vec<(usize, bool, u32)>> = Mutex::new(Vec::new());
/// (world, idx=team*5+slot, 직전 state) — 0→6 **전이**만 센다(상태 6 유지 틱을 중복 계수하지 않기 위해)
static CENSUS_PREV: Mutex<Vec<(usize, u64, u64)>> = Mutex::new(Vec::new());
/// (world, 챔프이름, 시전 횟수)
static CENSUS_CNT: Mutex<Vec<(usize, String, u32)>> = Mutex::new(Vec::new());

/// 사일러스가 있는 월드에서 **전 챔프의 궁 시전 개시 횟수**를 센다(읽기 전용).
///
/// 비용 설계: `on_etick`은 엔티티마다 호출된다(실측 1판 129만 회). 그래서
///   ①전수 슬롯 스캔(최대 2048칸)은 **50,000회에 한 번**만 해서 챔프 키 목록을 갱신하고
///   ②상태 표집은 **32회에 한 번**(≈게임 1틱)만 — 캐시된 키 10여 개를 resolve 한다.
unsafe fn ult_census() {
    let n = CENSUS_N.fetch_add(1, Ordering::Relaxed);
    if n % 4 != 0 { return; }
    // ★★[v117] 사일러스 rush_state는 **상태 무관 + 8배 조밀하게** 표집한다.
    //   ⚠v113~v116 결함: `st==6`(시전 중)일 때만, 그것도 32회에 1번만 읽었다.
    //   돌진은 발화틱(4틱째)에 걸려 ~10틱 지속인데, 표집이 성기고 조건이 좁아
    //   창을 통째로 놓쳤을 가능성을 배제하지 못했다. 여기서 배제한다.
    {
        let list: Vec<(usize, u64)> = { SY_CACHE.lock().unwrap_or_else(|x| x.into_inner()).clone() };
        for (w, k) in list {
            let e = match champ_by_key(w, k, "sylas") { Some(v) => v, None => continue };
            let rs = match rd_u64(e + 0x308) { Some(v) => v, None => continue };
            let variant: i64 = if (rs as i64) < 0 { (rs ^ 0x8000_0000_0000_0000) as i64 } else { 4 };
            if variant != 0 && RUSH_LOG.fetch_add(1, Ordering::Relaxed) < 12 {
                hlog(&format!("[돌진확인] sylas key={} rush_state={:#x} 변종={} ({}) state={:?}
",
                    k, rs, variant, match variant { 1 => "MoveTo", 2 => "MoveToTarget=광전사궁 ★성공",
                    3 => "Rush(비관통)", 4 => "Rush(관통)", _ => "None" }, rd_u64(e + E_CAST_ST)));
            }
            let mut g = RUSH_SEEN.lock().unwrap_or_else(|x| x.into_inner());
            match g.iter_mut().find(|q| q.0 == variant) { Some(q) => q.1 += 1, None => g.push((variant, 1u32)) }
            drop(g);
            // ★★[v118] **표집 횟수가 아니라 사건 횟수**를 센다.
            //   돌진 1회는 ~10틱 지속돼 수백 번 표집된다 ⟹ "2403"은 사건 수가 아니다.
            //   유저가 실제로 보는 것 = **None → 변종 전이 1회 = 돌진 1회**. 그걸 센다.
            {
                let mut pg = RUSH_PREV.lock().unwrap_or_else(|x| x.into_inner());
                let prevv = match pg.iter_mut().find(|q| q.0 == w && q.1 == k) {
                    Some(q) => { let p = q.2; q.2 = variant; p }
                    None => { pg.push((w, k, variant)); variant }
                };
                drop(pg);
                if prevv == 0 && variant != 0 {
                    let mut eg = RUSH_EVENT.lock().unwrap_or_else(|x| x.into_inner());
                    match eg.iter_mut().find(|q| q.0 == variant) { Some(q) => q.1 += 1, None => eg.push((variant, 1u32)) }
                }
            }
        }
    }
    if n % 32 != 0 { return; }
    let worlds: Vec<usize> = {
        let g = SLOT_MINE.lock().unwrap_or_else(|x| x.into_inner());
        let mut v: Vec<usize> = g.iter().map(|q| q.0).collect();
        v.sort_unstable(); v.dedup(); v
    };
    // ── 키 목록 재구축(드물게). 리스폰·교체로 키가 바뀌므로 주기적으로 다시 훑는다.
    if n % 50_000 == 0 {
        let mut keys: Vec<(usize, u64, String)> = Vec::new();
        for &w in worlds.iter() {
            let slot_len = rd_u64(w + W_SLOT_LEN).unwrap_or(0).min(2048);
            for k in 0..slot_len {
                let e = resolve(w, k);
                if e < 0x10000 || !is_champion(e) { continue; }
                keys.push((w, k, ent_name(e).unwrap_or_else(|| "?".into())));
            }
        }
        if CENSUS_DIAG.fetch_add(1, Ordering::Relaxed) < 3 {
            let names: Vec<String> = keys.iter().map(|q| format!("{}#{}", q.2, q.1)).collect();
            hlog(&format!("[궁시전집계·키목록] 월드{}개 챔프{}명 | {}\n", worlds.len(), keys.len(), names.join(" ")));
        }
        *CENSUS_KEYS.lock().unwrap_or_else(|x| x.into_inner()) = keys;
    }
    // ── 상태 표집: ≠6 → 6 전이만 계수(상태 유지 틱 중복 방지)
    let keys: Vec<(usize, u64, String)> = { CENSUS_KEYS.lock().unwrap_or_else(|x| x.into_inner()).clone() };
    let found = keys.len();
    for (w, k, nm) in keys {
        let e = match champ_by_key(w, k, &nm) { Some(v) => v, None => continue };
        let st = match rd_u64(e + E_CAST_ST) { Some(v) => v, None => continue };
        let prev = {
            let mut g = CENSUS_PREV.lock().unwrap_or_else(|x| x.into_inner());
            match g.iter_mut().find(|q| q.0 == w && q.1 == k) {
                Some(q) => { let p = q.2; q.2 = st; p }
                None => { g.push((w, k, st)); st }   // 첫 관측은 전이로 치지 않는다
            }
        };
        if !(prev != 6 && st == 6) { continue; }
        let mut g = CENSUS_CNT.lock().unwrap_or_else(|x| x.into_inner());
        match g.iter_mut().find(|q| q.0 == w && q.1 == nm) { Some(q) => q.2 += 1, None => g.push((w, nm, 1)) }
    }
    // ── 요약: 주기 + **직전 출력과 내용이 같으면 침묵**(궁 시전은 드문 사건이라 변화 시에만 찍으면 줄이 적다)
    if n % 100_000 != 0 { return; }
    if worlds.is_empty() {
        if n % 2_000_000 == 0 { hlog("[궁시전집계] 대상 월드 0개(사일러스가 있는 경기가 아님)\n"); }
        return;
    }
    let snap: Vec<(usize, String, u32)> = { CENSUS_CNT.lock().unwrap_or_else(|x| x.into_inner()).clone() };
    let mut body = String::new();
    for w in worlds.iter().copied().take(4) {
        let mut rows: Vec<(&String, u32)> = snap.iter().filter(|q| q.0 == w).map(|q| (&q.1, q.2)).collect();
        rows.sort_by(|a, b| b.1.cmp(&a.1));
        let cells = if rows.is_empty() { "(궁 시전 0회)".to_string() }
                    else { rows.iter().map(|(n, c)| format!("{}={}", n, c)).collect::<Vec<_>>().join(" ") };
        body.push_str(&format!("\n           world={:#x} | {}", w, cells));
    }
    {
        let mut last = CENSUS_LAST.lock().unwrap_or_else(|x| x.into_inner());
        if *last == body && n % 2_000_000 != 0 { return; }
        *last = body.clone();
    }
    let rush = { let g = RUSH_SEEN.lock().unwrap_or_else(|x| x.into_inner());
        let mut v: Vec<(i64, u32)> = g.clone(); v.sort_by(|a, b| b.1.cmp(&a.1));
        if v.is_empty() { "관측없음".to_string() }
        else { v.iter().map(|(k, c)| format!("{}:{}", match k { 0 => "None(돌진안함)", 1 => "MoveTo",
            2 => "MoveToTarget★", 3 => "Rush", _ => "Rush관통" }, c)).collect::<Vec<_>>().join(" ") } };
    let comb = { let g = SY_COMBINE_TALLY.lock().unwrap_or_else(|x| x.into_inner());
        let mut v = g.clone(); v.sort_by(|a, b| b.2.cmp(&a.2));
        if v.is_empty() { "없음".to_string() }
        else { v.iter().map(|(n, u, c)| format!("{}자식{}:{}", n, if *u { "(궁시전중★)" } else { "" }, c))
                .collect::<Vec<_>>().join(" ") } };
    let ev = { let g = RUSH_EVENT.lock().unwrap_or_else(|x| x.into_inner());
        let mut v: Vec<(i64, u32)> = g.clone(); v.sort_by(|a, b| b.1.cmp(&a.1));
        if v.is_empty() { "0회".to_string() }
        else { v.iter().map(|(k, c)| format!("{}:{}회", match k { 1 => "MoveTo", 2 => "MoveToTarget★광전사궁",
            3 => "Rush", _ => "Rush관통" }, c)).collect::<Vec<_>>().join(" ") } };
    hlog(&format!("[궁시전집계] 챔프{}명{}\n           ★★돌진 **사건** 수(눈에 보이는 그것): {}\n           \
(참고)rush_state 표집 분포: {}\n           (참고)Combine apply 분포: {}\n",
        found, body, ev, rush, comb));
}
/// 원본 vtable → 마스킹 사본. 사본은 leak해서 Arc보다 오래 살린다.
static VT_COPY: Mutex<Vec<(usize, usize)>> = Mutex::new(Vec::new());
/// effect vtable ~~35슬롯 0x118B(0.5.6)~~ → ★**0.5.7 = 36슬롯 0x120B**.
/// 신규 메서드 1개가 **구 `+0x28`(damage)과 구 `+0x30`(heal) 사이에 삽입**되어
/// **구 `+0x30` 이상이 전부 +8 이동**했다(2026-08-29 정적 전수, 5중 독립 일치로 확정:
/// 상수 48000 스텁 유일함수가 `+0x100`→`+0x108`, 슬롯별 오버라이드 개수 대응, 기본 스텁 사전 1:1).
/// ⚠★★**표 주소만 고치고 인덱스를 그대로 두면 더 나빠진다** — 0.5.6 인덱스는 0.5.7에서
///   `+0x48`(무관)·`+0x58`(rush)·**`+0x110`(AoE)** 를 덮어써, AoE 궁 9종
///   (demon·executioner·fighter·hammerer·jiangshi·monk·poison_dart_hunter·prisoner·spirit_caller)의
///   범위 플래그를 파괴한다.
const VT_WORDS: usize = 36;
const VT_I_MOVE_TICKS: usize = 0x50 / 8;    // 10: Option<(틱, 거리)> — "시전자를 이동시키는가"
const VT_I_NOT_INSTANT: usize = 0x60 / 8;   // 12: bool — 블링크/순간이동 포함
const VT_I_IS_MOVE: usize = 0x118 / 8;      // 35: bool — is_move_skill
/// ★[v129] 오더 팩토리 술어 2종 (RE 2026-08-29 casting_type 소비처 전수 §3 D1·D2).
///   `+0xf0` = "스킬샷/리드샷 필요" · `+0x110` = "범위(AoE)". 둘 다 **AI 판정 전용**이고
///   발동(`+0x20 apply`)·drop(`+0x00`)과 무관하다.
///   ct∈{1,2}일 때만 읽히며, true면 궁 오더가 **Move 오더로 강등**된다(= 궁 시전 0).
const VT_I_SKILLSHOT: usize = 0xf0 / 8;     // 30
const VT_I_AOE:       usize = 0x110 / 8;    // 34

/// ★★★[v105] 공여자 effect vtable의 **AI 판정 3슬롯만** 사일러스 기본 구현으로 바꾼 사본을 만든다.
///
/// 왜 필요한가 (RE 2026-08-25_이동형궁-AI결심경로-전수):
///   AI 궁 결심(battle.rs `0xd4f370`)은 슬롯의 effect vtable을 **가상호출**해 "이동형 궁인가"를
///   판정한다. 이동형이면 ①모드 3(KitingBack)/4(RunAway)에서 적 대상 후보를 통째로 스킵하고
///   ②모드 1(Protect)/2(Kiting)에선 돌진 안전판정 `0x140d4a130`을 **추가로** 요구한다.
///   사일러스 자기 궁은 이 3슬롯이 전부 기본 구현(None/false/false)이라 **어떤 관문에도 안 걸렸다**.
///   광전사 궁을 이식한 순간 관문이 3개 생겨서 궁이 안 나간 것이다.
///   실제 발동은 `+0x20`(apply)이 하므로, 이 3슬롯만 가리면 **AI에겐 평범한 즉시 궁으로 보이고
///   발동은 원본대로 돌진**한다.
///
/// 안전성: drop(`+0x00`)·size·align·Debug·apply 등 나머지 32슬롯은 원본 그대로다.
///   ⟹ Arc drop 경로가 불변이라 해제 시 크래시 위험이 없다. vtable 포인터 동일성을 비교하는
///   게임 코드는 정적 스캔에서 발견되지 않았다.
/// ★[v107] effect vtable 표 59종을 훑어 각 AI 판정 슬롯의 **최빈값 = 기본 스텁**을 뽑는다.
///
/// 왜 최빈값인가: 59종 중 대다수가 그 슬롯을 오버라이드하지 않는다(RE §4 — `+0x48`/`+0x58`은
/// 43종이 기본). 따라서 **가장 많이 나오는 함수 포인터가 곧 기본 구현**이다. RVA를 박지 않으므로
/// 패치가 와도 표 주소만 맞으면 따라간다. 과반이 아니면 마스킹을 포기한다(오판 방지).
unsafe fn default_eff_stubs() -> Option<[u64; 5]> {
    let base = exe_base() + EFF_VT_BASE;
    let mut out = [0u64; 5];
    for (k, &slot) in [VT_I_MOVE_TICKS, VT_I_NOT_INSTANT, VT_I_IS_MOVE,
                       VT_I_SKILLSHOT, VT_I_AOE].iter().enumerate() {
        let mut seen: Vec<(u64, u32)> = Vec::new();
        for i in 0..EFF_VT_N {
            let v = match rd_u64(base + i * EFF_VT_STRIDE + slot * 8) { Some(v) => v, None => continue };
            if !in_exe(v) { continue; }
            match seen.iter_mut().find(|q| q.0 == v) { Some(q) => q.1 += 1, None => seen.push((v, 1)) }
        }
        let (best, n) = match seen.iter().max_by_key(|q| q.1) { Some(&q) => q, None => {
            hlog(&format!("[AI마스크·포기] effect vtable 표 {:#x} 읽기 실패(슬롯 {})\n", base, slot));
            return None;
        }};
        if (n as usize) * 2 <= EFF_VT_N {
            hlog(&format!("[AI마스크·포기] 슬롯 {}의 최빈값이 과반 미달 {}/{} — 표 주소 의심\n", slot, n, EFF_VT_N));
            return None;
        }
        out[k] = best;
    }
    hlog(&format!("[AI마스크] 기본 스텁(최빈값·0.5.7) +0x50=RVA:{:#x} +0x60=RVA:{:#x} +0x118=RVA:{:#x} | 오더술어 +0xf0=RVA:{:#x} +0x110=RVA:{:#x}
",
        rva_of(out[0]), rva_of(out[1]), rva_of(out[2]), rva_of(out[3]), rva_of(out[4])));
    Some(out)
}

unsafe fn ai_mask_vt(orig: usize, sy_vt: usize) -> Option<usize> {
    if !AI_MASK.load(Ordering::Relaxed) && !ORDER_MASK.load(Ordering::Relaxed) { return Some(orig); }
    if orig < 0x10000 { return None; }
    // ── 사일러스 기본 구현 3개 확보(최초 1회). 아직 못 얻었으면 마스킹을 포기하고
    //    원본을 그대로 쓴다(궁이 안 나갈지언정 잘못된 함수 포인터를 심지는 않는다).
    let def = {
        let mut g = SY_VT_DEF.lock().unwrap_or_else(|x| x.into_inner());
        if g.is_none() {
            match default_eff_stubs() { Some(v) => *g = Some(v), None => return Some(orig) }
        }
        match *g { Some(v) => v, None => return Some(orig) }
    };
    let _ = sy_vt;   // v107: 더 이상 사일러스 vtable을 샘플로 쓰지 않는다(위 SY_VT_DEF 주석 참조)
    // ── 사본 캐시
    {
        let g = VT_COPY.lock().unwrap_or_else(|x| x.into_inner());
        if let Some(q) = g.iter().find(|q| q.0 == orig) { return Some(q.1); }
    }
    let mut w = [0u64; VT_WORDS];
    for i in 0..VT_WORDS {
        w[i] = match rd_u64(orig + i * 8) {
            Some(v) => v,
            None => { hlog(&format!("[AI마스크·포기] 공여자 vtable {:#x} 슬롯 {} 읽기 실패\n", orig, i)); return Some(orig); }
        };
    }
    let (o9, o11, o34) = (w[VT_I_MOVE_TICKS], w[VT_I_NOT_INSTANT], w[VT_I_IS_MOVE]);
    if AI_MASK.load(Ordering::Relaxed) {
        w[VT_I_MOVE_TICKS]  = def[0];
        w[VT_I_NOT_INSTANT] = def[1];
        w[VT_I_IS_MOVE]     = def[2];
    }
    // ★[v129] 안 A — 오더 팩토리 술어 2종. `ai_mask`와 **독립**이다(목적이 다르다).
    if ORDER_MASK.load(Ordering::Relaxed) {
        let (a, b) = (w[VT_I_SKILLSHOT], w[VT_I_AOE]);
        w[VT_I_SKILLSHOT] = def[3];
        w[VT_I_AOE]       = def[4];
        hlog(&format!("★[오더마스크] +0xf0 RVA:{:#x}->{:#x} | +0x118 RVA:{:#x}->{:#x} = ct 1/2 궁이 Move 오더로 강등되는 것을 막는다
",
            rva_of(a), rva_of(def[3]), rva_of(b), rva_of(def[4])));
    }
    // leak: 이 vtable을 가리키는 Arc가 언제 죽는지 알 수 없다. 공여자당 280B라 누수는 무의미.
    let p = Box::leak(Box::new(w)).as_ptr() as usize;
    hlog(&format!("★★★[AI마스크] effect vtable 사본 {:#x}→{:#x} ({}슬롯 중 3개 교체)\n           \
+0x50 RVA:{:#x}→{:#x} | +0x60 RVA:{:#x}→{:#x} | +0x110 RVA:{:#x}→{:#x}\n           \
= AI에겐 '이동 안 시키는 즉시 궁'으로 보이게 한다. 발동(+0x20 apply)은 원본 그대로.\n",
        orig, p, VT_WORDS, rva_of(o9), rva_of(def[0]), rva_of(o11), rva_of(def[1]), rva_of(o34), rva_of(def[2])));
    VT_COPY.lock().unwrap_or_else(|x| x.into_inner()).push((orig, p));
    Some(p)
}

// ═══════════════════════════════════════════════════════════════════════════
//  ★★★[v119] 뷰 애니 태그 접미사 — 이름 충돌 해소
// ═══════════════════════════════════════════════════════════════════════════
// 문제: 사일러스 fanim에는 `ult` 가 **하나**뿐인데 48챔프가 그 이름을 쓰고
//       사일러스 자기 궁도 쓴다. `ult_loop` 도 7챔프가 공유한다.
// 해법: **뷰 쪽에서 엔티티별로** 태그에 공여자 접미사를 붙인다 (`ult` → `ult_priest`).
//       뷰 레코드는 개체마다 따로라 공여자 원본 연출을 건드리지 않는다.
//       ⛔시뮬 쪽 문자열 수정은 금지 — 그 Arc 는 공여자 챔프가 같이 쓴다(실측 확인).
//
// 개입점: **태그 선택기 `FUN_141f94b50`** — 호출마다 새로 만들어져 호출자가 즉시 drop 하는
//   임시 String 하나만 고친다. 뷰의 영속 태그(`V+0x68`)도 안 건드린다 = 개입면 최소.
//   `fn(rcx: *mut String /*sret 24B*/, rdx: *const ViewEntity) -> rax`
//   콜사이트 5곳: 0xb49bd0(소비자 검증) / 0x1f8ff8e(렌더러) / 0x1fbcd58·0x1fbdcc1(gunner) / 0x213c355(제2 렌더러)
//
// ★안전장치 = **fanim 화이트리스트**. 우리가 반환하는 이름이 사일러스 fanim 에 실재하는 것만 허용한다.
//   ①렌더러가 못 찾으면 그 프레임 **몸체가 통째로 안 그려진다**(0x1f9061d→0x1f90730)
//   ②소비자(sub-3)는 못 찾으면 **태그를 롤백**해 시전 애니가 아예 시작 안 된다(조용한 실패)
//   ⟹ 실재하는 이름만 내보내면 두 경로 모두 안전하므로 리턴어드레스 구분이 불필요하다.
//
// 참조구현: 게임 자신이 같은 짓을 한다 — small_jiangshi `_mini`(0x1fbbaa0),
//   ghoul `berserk_`(0x1fbc160), cavalry_knight `fire_`(0x238ced0), demon `archfiend_`(0x2631900).
// (RE 2026-08-26_뷰-애니이름-setter-전수-개입점.md)

const TAGSEL_RVA: usize = 0x23964e0;
const TAGSEL_SIG: [u8; 12] = [0x41,0x56,0x56,0x57,0x53,0x48,0x81,0xEC,0x98,0x00,0x00,0x00];
const V_CHAMP_ID: usize = 0x38;      // ViewEntity 안 챔프 id String {cap,ptr,len}
static TAGSEL_TRAMP: AtomicUsize = AtomicUsize::new(0);
static TAG_SWAP: AtomicBool = AtomicBool::new(false);    // cfg tag_swap (기본 OFF — 애니 정지 버그)
static TAG_SWAP_N: AtomicU32 = AtomicU32::new(0);
static TAG_MISS_N: AtomicU32 = AtomicU32::new(0);
/// 사일러스 fanim 의 anims 키 집합 — 여기 없는 이름은 **절대** 내보내지 않는다.
static ANIM_SET: Mutex<Vec<String>> = Mutex::new(Vec::new());
/// 치환용 문자열 버퍼(이름 → leak 한 포인터). cap=0 으로 넘기므로 게임이 free 하지 않는다.
static NAME_BUF: Mutex<Vec<(String, usize)>> = Mutex::new(Vec::new());

type TagSelFn = extern "win64" fn(usize, usize) -> usize;

/// 사일러스 fanim 에서 애니 이름 목록을 읽어 화이트리스트로 삼는다.
/// 경로는 **게임 exe 기준으로 도출**한다(설치 경로 하드코딩 금지 — CLAUDE.md §2).
unsafe fn load_anim_whitelist() -> usize {
    let mut cands: Vec<std::path::PathBuf> = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            cands.push(dir.join("mods").join("sylas").join("aseprite_resources")
                          .join("champions").join("sylas#anim.fanim"));
        }
    }
    cands.push(std::path::PathBuf::from(
        "C:\\tfm2mods\\sylas\\aseprite_resources\\champions\\sylas#anim.fanim"));
    for p in cands {
        let txt = match std::fs::read_to_string(&p) { Ok(t) => t, Err(_) => continue };
        // 의존성 없이 `"anims"` 블록의 **1단계 키**만 긁는다(깊이 카운트).
        let i = match txt.find("\"anims\"") { Some(v) => v, None => continue };
        let b = &txt[i..];
        let j = match b.find('{') { Some(v) => v + 1, None => continue };
        let body = &b[j..];
        let by = body.as_bytes();
        let mut depth: i32 = 1;
        let mut names: Vec<String> = Vec::new();
        let mut k: usize = 0;
        while k < by.len() {
            match by[k] {
                b'{' => depth += 1,
                b'}' => { depth -= 1; if depth == 0 { break; } }
                b'"' if depth == 1 => {
                    if let Some(e) = body[k + 1..].find('"') {
                        let name = &body[k + 1..k + 1 + e];
                        let rest = body[k + 1 + e + 1..].trim_start();
                        if rest.starts_with(':') && !name.is_empty() {
                            names.push(name.to_string());
                        }
                        k = k + 1 + e;
                    }
                }
                _ => {}
            }
            k += 1;
        }
        if !names.is_empty() {
            let n = names.len();
            hlog(&format!("[태그접미사] fanim 화이트리스트 {}종 로드 ({})\n           {}\n",
                n, p.display(), names.join(" ")));
            *ANIM_SET.lock().unwrap_or_else(|x| x.into_inner()) = names;
            return n;
        }
    }
    hlog("[태그접미사] ✗fanim 을 못 읽었다 — 접미사 치환 비활성(안전측)\n");
    0
}

/// 이름 → 안정 포인터. cap=0 으로 넘기므로 게임이 free 하지 않는다 ⟹ leak 해도 무해.
fn stable_name_ptr(name: &str) -> usize {
    let mut g = NAME_BUF.lock().unwrap_or_else(|x| x.into_inner());
    if let Some(q) = g.iter().find(|q| q.0 == name) { return q.1; }
    let p = Box::leak(name.as_bytes().to_vec().into_boxed_slice()).as_ptr() as usize;
    g.push((name.to_string(), p));
    p
}

/// 게임 String {cap,ptr,len} 읽기
unsafe fn rd_gstr(base: usize) -> Option<(usize, usize, String)> {
    let cap = rd_u64(base)? as usize;
    let ptr = rd_u64(base + 8)? as usize;
    let len = rd_u64(base + 0x10)? as usize;
    if len == 0 || len > 128 || ptr < 0x10000 || !readable(ptr, len) { return None; }
    let s = core::slice::from_raw_parts(ptr as *const u8, len);
    Some((cap, ptr, String::from_utf8_lossy(s).into_owned()))
}

extern "win64" fn tagsel_detour(out: usize, v: usize) -> usize {
    let t = TAGSEL_TRAMP.load(Ordering::Relaxed);
    if t == 0 { return out; }
    let r = unsafe { core::mem::transmute::<usize, TagSelFn>(t)(out, v) };
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { tagsel_swap(out, v) }));
    r
}

unsafe fn tagsel_swap(out: usize, v: usize) {
    if !TAG_SWAP.load(Ordering::Relaxed) { return; }
    if out < 0x10000 || v < 0x10000 { return; }
    // ① 이 개체가 사일러스인가 — ViewEntity 안에 챔프 id 가 그대로 있다(역참조 불필요)
    let id = match rd_gstr(v + V_CHAMP_ID) { Some((_, _, s)) => s, None => return };
    if id != "sylas" { return; }
    // ② 지금 강탈 중인 공여자
    let donor = {
        let g = SLOT_TMPL.lock().unwrap_or_else(|x| x.into_inner());
        match g.as_ref() { Some(t) => t.1.clone(), None => return }
    };
    if donor.is_empty() { return; }
    // ③ 현재 태그 + 접미사가 fanim 에 실재할 때만 치환
    let (cap, ptr, tag) = match rd_gstr(out) { Some(v) => v, None => return };
    let want = format!("{}_{}", tag, donor);
    {
        let g = ANIM_SET.lock().unwrap_or_else(|x| x.into_inner());
        if g.is_empty() || !g.iter().any(|n| *n == want) {
            let n = TAG_MISS_N.fetch_add(1, Ordering::Relaxed);
            if n < 8 { hlog(&format!("[태그접미사] '{}' 없음 → 원본 '{}' 유지\n", want, tag)); }
            return;
        }
    }
    // ④ 교체. 옛 버퍼는 게임 힙에서 해제하고, 우리 것은 cap=0 으로 넘겨 게임이 free 하지 않게 한다.
    //    (게임 자신도 len==0 일 때 cap=0 상태를 정상값으로 생산한다 — RE §5)
    let np = stable_name_ptr(&want);
    if cap != 0 && ptr >= 0x10000 { HeapFree(GetProcessHeap(), 0, ptr); }
    if !(wr_u64(out, 0) && wr_u64(out + 8, np as u64) && wr_u64(out + 0x10, want.len() as u64)) { return; }
    let n = TAG_SWAP_N.fetch_add(1, Ordering::Relaxed);
    if n < 12 { hlog(&format!("[태그접미사] #{} '{}' → '{}'\n", n, tag, want)); }
}

// ─────────────────────────────────────────────────────────────────────────────
// ★★[v125] 시전 애니 태그 교체 — **이미터 큐 엔트리 스왑** (RE 2026-08-25 §3 A안)
//
// 왜 갈아탔나: v119 는 태그 선택기 `FUN_141f94b50` 을 후킹했는데, 그 함수는
//   `0xb49bd0` = **SetAnimation 커맨드 처리부 안**에서 불린다. 그 직전 `0xb49b33` 이
//   옛 태그와 함께 **`V+0x158`(애니 타이머)를 백업/리셋**하므로, 채널링 중 커맨드가 반복
//   발행되면 매번 타이머가 0으로 돌아가 **프레임 0에 고정**된다(2026-08-26 인게임 실증).
//   RE 문서의 권장안은 처음부터 이미터였다.
//
// 개입면: 이미터가 방금 push 한 **큐 엔트리 1개의 String 만** 고친다.
//   sub-tag 3 노드 생성 12곳 중 이미터(`0x14c5556`)만 궁이라 **평타·이동·스킬엔 영향 없다.**
//   동기 실행이라 경합 없고, 태그가 fanim 에 없으면 게임이 스스로 롤백한다(실패 안전).
const EMIT_RVA: usize = 0x10556d0;
// push rbp/rsi/rdi/rbx ; sub rsp,0x78 ; lea rbp,[rsp+0x70]  = 13B (마지막 lea 가 5B)
const EMIT_SIG: [u8; 13] = [0x55,0x56,0x57,0x53,0x48,0x83,0xEC,0x78,0x48,0x8D,0x6C,0x24,0x70];
static EMIT_TRAMP: AtomicUsize = AtomicUsize::new(0);
static EMIT_SWAP:  AtomicBool  = AtomicBool::new(true);   // cfg emit_swap
static EMIT_N:     AtomicU32   = AtomicU32::new(0);
static EMIT_MISS:  AtomicU32   = AtomicU32::new(0);
type EmitFn = extern "win64" fn(usize, u64, usize);

extern "win64" fn emit_detour(qpp: usize, ent_id: u64, ctx: usize) {
    let t = EMIT_TRAMP.load(Ordering::Relaxed);
    if t == 0 { return; }
    unsafe { core::mem::transmute::<usize, EmitFn>(t)(qpp, ent_id, ctx) };
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
        || unsafe { emit_swap(qpp, ent_id, ctx) }));
}

/// 방금 push 된 마지막 큐 엔트리의 태그를 `<태그>_<공여자>` 로 바꾼다.
/// 레이아웃 근거 = RE `2026-08-25_시전애니-태그교체-개입점.md` §3-1/§3-3.
unsafe fn emit_swap(qpp: usize, ent_id: u64, ctx: usize) {
    if !EMIT_SWAP.load(Ordering::Relaxed) { return; }
    if qpp < 0x10000 || ctx < 0x10000 { return; }

    // ① 시전자가 사일러스인가 — CastViewCtx{+0x00: *Entity}
    let ent = match rd_u64(ctx) { Some(v) => v as usize, None => return };
    if ent < 0x10000 || !is_champion(ent) { return; }
    if ent_name(ent).as_deref() != Some("sylas") { return; }

    // ② 지금 강탈 중인 공여자
    let donor = {
        let g = SLOT_TMPL.lock().unwrap_or_else(|x| x.into_inner());
        match g.as_ref() { Some(t) => t.1.clone(), None => return }
    };
    if donor.is_empty() { return; }

    // ③ 큐의 **마지막** 엔트리 = 방금 이 호출이 push 한 것
    let q = match rd_u64(qpp) { Some(v) => v as usize, None => return };   // [rcx]==0 = 비수집 틱
    if q < 0x10000 { return; }
    let len = match rd_u64(q + 0x10) { Some(v) => v as usize, None => return };
    if len == 0 || len > 0x10000 { return; }
    let arr = match rd_u64(q + 8) { Some(v) => v as usize, None => return };
    if arr < 0x10000 { return; }
    let e = arr.wrapping_add((len - 1) * 0x10);
    if rd_u64(e).map(|v| v & 0xffff_ffff) != Some(0) { return; }           // 엔트리 kind 0
    let node = match rd_u64(e + 8) { Some(v) => v as usize, None => return };
    if node < 0x10000 { return; }
    if rd_u64(node) != Some(3) { return; }                                 // sub-tag 3 = SetAnimation
    if rd_u64(node + 0x10) != Some(ent_id) { return; }                     // 다른 개체의 노드면 손대지 않는다
    // ★CasterAnimation 경로는 Entity 포인터를 못 받으므로, 여기서 확인된 id 를 넘겨준다
    SYLAS_EID.store(ent_id, Ordering::Relaxed);
    let pl = match rd_u64(node + 8) { Some(v) => v as usize, None => return };
    if pl < 0x10000 { return; }

    // ④ 접미사 이름이 fanim 에 실재할 때만 교체
    let (cap, ptr, tag) = match rd_gstr(pl) { Some(v) => v, None => return };
    let want = format!("{}_{}", tag, donor);
    {
        let g = ANIM_SET.lock().unwrap_or_else(|x| x.into_inner());
        if g.is_empty() || !g.iter().any(|n| *n == want) {
            let n = EMIT_MISS.fetch_add(1, Ordering::Relaxed);
            if n < 8 { hlog(&format!("[이미터태그] '{}' 없음 → 원본 '{}' 유지
", want, tag)); }
            return;
        }
    }

    // ⑤ **게임 할당자**로 새 버퍼(RE §3-3 정공법). payload 가 유일 소유자라 큐 drop 이 free 한다
    //    ⟹ cap=0 트릭을 쓰면 안 된다(큐 drop 글루의 가드 여부가 미확인).
    let n = want.len();
    if n == 0 || n > 128 { return; }
    let np = HeapAlloc(GetProcessHeap(), 0, n);
    if np < 0x10000 { return; }
    core::ptr::copy_nonoverlapping(want.as_ptr(), np as *mut u8, n);
    if !(wr_u64(pl, n as u64) && wr_u64(pl + 8, np as u64) && wr_u64(pl + 0x10, n as u64)) {
        HeapFree(GetProcessHeap(), 0, np);                                 // 쓰기 실패 시 누수 방지
        return;
    }
    if cap != 0 && ptr >= 0x10000 { HeapFree(GetProcessHeap(), 0, ptr); }  // 옛 버퍼 반납
    let k = EMIT_N.fetch_add(1, Ordering::Relaxed);
    if k < 12 { hlog(&format!("[이미터태그] #{} '{}' → '{}'
", k, tag, want)); }
}

// ─────────────────────────────────────────────────────────────────────────────
// ★★[v126] **CasterAnimation(sub-tag 0xd) 태그 교체** — 두 번째 경로
//
// 왜 필요한가: 시전 애니 태그를 만드는 경로가 **둘**이다.
//   ① 이미터 `EMIT` → sub-tag 3(SetAnimation) — 미스 시 게임이 **롤백**(안전)
//   ② 이 함수      → sub-tag 0xd(CasterAnimation) — **검증 전무 → 캐릭터 소실**
//   ②를 안 잡으면, 공여자 애니를 접미사로 통일했을 때 이 경로가 **없는 평이름**을 써넣어
//   **사일러스가 투명해진다**(2026-08-28 실사고: 성직자 `ult_idle`/`ult_heal` 평이름 제거 직후).
//
// 증거: 성직자 궁 Combine 말단의 apply RVA `0x12e7ca0` = CASTANIM_APPLY 재핀값과 일치
//       (migrate_rva 0.5.6 `0x16e4c20` → 0.5.7 `0x12e7ca0`, 런타임 로그와 교차 확인).
const CASTANIM_RVA: usize = 0x104ca00;
// push rbp/r15/r14/r12/rsi/rdi/rbx ; sub rsp,0x50  = 14B (12B 에서 sub 가 잘린다)
const CASTANIM_SIG: [u8; 14] = [0x55,0x41,0x57,0x41,0x56,0x41,0x54,0x56,0x57,0x53,
                                0x48,0x83,0xEC,0x50];
static CASTANIM_TRAMP: AtomicUsize = AtomicUsize::new(0);
static CA_N:    AtomicU32 = AtomicU32::new(0);
static CA_MISS: AtomicU32 = AtomicU32::new(0);
/// 사일러스의 뷰 entity_id. `emit_swap` 이 **이름으로 사일러스임을 확인한 뒤** 기록한다.
/// CasterAnimation 쪽엔 Entity 포인터가 인자로 안 오므로 이 값으로만 대상을 가린다.
/// 못 봤으면(=u64::MAX) 아무것도 하지 않는다 — 남의 개체를 건드리면 그쪽이 투명해진다.
static SYLAS_EID: AtomicU64 = AtomicU64::new(u64::MAX);
type CastAnimFn = extern "win64" fn(usize, usize, usize, usize);

extern "win64" fn castanim_detour(a: usize, b: usize, c: usize, d: usize) {
    let t = CASTANIM_TRAMP.load(Ordering::Relaxed);
    if t == 0 { return; }
    unsafe { core::mem::transmute::<usize, CastAnimFn>(t)(a, b, c, d) };
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
        || unsafe { castanim_swap(a) }));
}

/// 큐 레이아웃은 이미터와 동일: 엔트리 0x10 `{u32 kind, u64 node}` · 노드 `{sub_tag, payload, entity_id}`.
/// 다른 점은 **sub_tag 가 0xd** 이고 대상 식별을 `SYLAS_EID` 로 한다는 것뿐이다.
unsafe fn castanim_swap(qpp: usize) {
    if !EMIT_SWAP.load(Ordering::Relaxed) { return; }
    if qpp < 0x10000 { return; }
    let eid = SYLAS_EID.load(Ordering::Relaxed);
    if eid == u64::MAX { return; }                       // 아직 사일러스를 못 봤다

    let donor = {
        let g = SLOT_TMPL.lock().unwrap_or_else(|x| x.into_inner());
        match g.as_ref() { Some(t) => t.1.clone(), None => return }
    };
    if donor.is_empty() { return; }

    let q = match rd_u64(qpp) { Some(v) => v as usize, None => return };
    if q < 0x10000 { return; }
    let len = match rd_u64(q + 0x10) { Some(v) => v as usize, None => return };
    if len == 0 || len > 0x10000 { return; }
    let arr = match rd_u64(q + 8) { Some(v) => v as usize, None => return };
    if arr < 0x10000 { return; }
    let e = arr.wrapping_add((len - 1) * 0x10);
    if rd_u64(e).map(|v| v & 0xffff_ffff) != Some(0) { return; }
    let node = match rd_u64(e + 8) { Some(v) => v as usize, None => return };
    if node < 0x10000 { return; }
    if rd_u64(node) != Some(0xd) { return; }             // ★CasterAnimation 만
    if rd_u64(node + 0x10) != Some(eid) { return; }      // ★사일러스 것만
    let pl = match rd_u64(node + 8) { Some(v) => v as usize, None => return };
    if pl < 0x10000 { return; }

    let (cap, ptr, tag) = match rd_gstr(pl) { Some(v) => v, None => return };
    let want = {
        let g = ANIM_SET.lock().unwrap_or_else(|x| x.into_inner());
        if g.is_empty() { return; }
        let suffixed = format!("{}_{}", tag, donor);
        if g.iter().any(|n| *n == suffixed) {
            suffixed                                   // ①공여자 전용 이름이 있다 = 정상 교체
        } else if g.iter().any(|n| *n == tag) {
            return;                                    // ②원본 이름이 실재 = 손대지 않는다
        } else {
            // ★★③둘 다 없다 = 이대로 두면 **캐릭터가 사라진다**.
            //   이 경로(sub-tag 0xd)는 sub-tag 3 과 달리 **미스 롤백이 없어서**, 게임이
            //   존재하지 않는 이름을 그대로 써넣고 렌더가 프레임을 못 찾는다(2026-08-28 실사고).
            //   ⟹ 모드가 책임지고 **실재하는 이름으로 갈아끼운다**. 연출은 틀려도 몸은 보인다.
            //   공여자 아트가 아직 없는 태그(예: 광전사 `ult_pre`)에서 발생한다.
            let fb = ["ult", "idle"];
            match fb.iter().find(|c| g.iter().any(|n| n.as_str() == **c)) {
                Some(c) => {
                    let n = CA_MISS.fetch_add(1, Ordering::Relaxed);
                    if n < 8 {
                        hlog(&format!("[시전연출] ⚠'{}' 도 '{}' 도 없음 → 소실방지 '{}' 로 대체
",
                                      tag, format!("{}_{}", tag, donor), c));
                    }
                    (*c).to_string()
                }
                None => return,                        // 폴백조차 없으면 건드리지 않는다
            }
        }
    };
    let n = want.len();
    if n == 0 || n > 128 { return; }
    let np = HeapAlloc(GetProcessHeap(), 0, n);
    if np < 0x10000 { return; }
    core::ptr::copy_nonoverlapping(want.as_ptr(), np as *mut u8, n);
    if !(wr_u64(pl, n as u64) && wr_u64(pl + 8, np as u64) && wr_u64(pl + 0x10, n as u64)) {
        HeapFree(GetProcessHeap(), 0, np);
        return;
    }
    if cap != 0 && ptr >= 0x10000 { HeapFree(GetProcessHeap(), 0, ptr); }
    let k = CA_N.fetch_add(1, Ordering::Relaxed);
    if k < 12 { hlog(&format!("[시전연출] #{} '{}' → '{}'
", k, tag, want)); }
}

/// serpen 모드와 같은 12B **wrapper** 트램폴린 — 원본을 먼저 실행하고 결과를 고칠 수 있다.
/// (기존 `install_detour` 는 관찰용이라 반환값을 못 고친다)
/// 길이 일반화 wrapper 트램폴린. `install_tramp12` 와 달리 **프롤로그 길이를 지정**한다.
///
/// 왜 필요한가: 패치는 항상 12B(`movabs rax,imm64; jmp rax`)지만, 원본 프롤로그가 12B에서
/// **명령 중간에 잘리면** 스텁이 쓰레기를 실행한다. 이미터 `0x14c54a0` 이 그 경우다 —
/// `55 56 57 53 | 48 83 EC 78 | 48 8D 6C 24 70` = **13B**(마지막 lea 가 5B).
/// (RE 문서의 "12B 안전" 서술은 부정확했다. 2026-08-26 exe 직접 실측으로 정정.)
/// ⟹ 스텁에는 n바이트를 통째로 복사하고, 패치 자리의 남는 (n-12)B 는 NOP 으로 채운다.
unsafe fn install_trampn(rva: usize, sig: &[u8], detour: usize,
                         tramp: &AtomicUsize) -> Result<usize, String> {
    let n = sig.len();
    if n < 12 { return Err(format!("프롤로그 {}B < 12B", n)); }
    let base = exe_base();
    if base == 0 { return Err("module 0".into()); }
    let fa = base + rva;
    if !readable(fa, n + 8) { return Err(format!("{:#x} unreadable", fa)); }
    for i in 0..n {
        let b = *((fa + i) as *const u8);
        if b != sig[i] { return Err(format!("프롤로그 불일치 +{}: {:#x} != {:#x}", i, b, sig[i])); }
    }
    let stub = VirtualAlloc(0, 64 + n, 0x1000 | 0x2000, 0x40);
    if stub == 0 { return Err("VirtualAlloc".into()); }
    let mut st: Vec<u8> = Vec::new();
    st.extend_from_slice(sig);                                             // 원본 프롤로그 n바이트
    st.extend_from_slice(&[0x48, 0xb8]); st.extend_from_slice(&((fa + n) as u64).to_le_bytes());
    st.extend_from_slice(&[0xff, 0xe0]);                                   // jmp fn+n
    core::ptr::copy_nonoverlapping(st.as_ptr(), stub as *mut u8, st.len());
    tramp.store(stub, Ordering::Relaxed);
    let mut patch: Vec<u8> = vec![0x90; n];                                // 남는 자리는 NOP
    patch[0] = 0x48; patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&detour.to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;
    let mut old = 0u32;
    if VirtualProtect(fa, n, 0x40, &mut old) == 0 { return Err("VirtualProtect".into()); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fa as *mut u8, n);
    let mut tmp = 0u32; VirtualProtect(fa, n, old, &mut tmp);
    FlushInstructionCache(GetCurrentProcess(), fa, n);
    Ok(stub)
}

unsafe fn install_tramp12(rva: usize, sig: &[u8; 12], detour: usize,
                          tramp: &AtomicUsize) -> Result<usize, String> {
    let base = exe_base();
    if base == 0 { return Err("module 0".into()); }
    let fa = base + rva;
    if !readable(fa, 20) { return Err(format!("{:#x} unreadable", fa)); }
    for i in 0..12 {
        let b = *((fa + i) as *const u8);
        if b != sig[i] { return Err(format!("프롤로그 불일치 +{}: {:#x} != {:#x}", i, b, sig[i])); }
    }
    let stub = VirtualAlloc(0, 64, 0x1000 | 0x2000, 0x40);
    if stub == 0 { return Err("VirtualAlloc".into()); }
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(sig);                                              // 원본 프롤로그
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&((fa + 12) as u64).to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);                                    // jmp fn+12
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    tramp.store(stub, Ordering::Relaxed);
    let mut patch = [0u8; 12];
    patch[0] = 0x48; patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&detour.to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;                                    // movabs rax,detour; jmp rax
    let mut old = 0u32;
    if VirtualProtect(fa, 12, 0x40, &mut old) == 0 { return Err("VirtualProtect".into()); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fa as *mut u8, 12);
    let mut tmp = 0u32; VirtualProtect(fa, 12, old, &mut tmp);
    FlushInstructionCache(GetCurrentProcess(), fa, 12);
    Ok(stub)
}

/// Arc strong-- 후 0이면 게임 drop 호출. 슬롯을 덮어쓰기 전에 옛 것을 놓아준다.
unsafe fn arc_dec_drop(data: usize, vt: usize) {
    if data < 0x10000 || !readable(data, 16) { return; }
    let sc = rd_u64(data).unwrap_or(0);
    if sc == 0 || sc > 0x100000 { return; }
    // ★★[v104] **단일 xadd로 감소하고 그 반환값으로 판정**한다.
    //   ~~xadd(+1) 후 sub(-2)~~ 는 두 가지가 틀렸다(2026-08-25 감사):
    //     ① 두 연산 사이에 refcount가 +1로 부풀어, 그 창에서 게임이 dec하면 0을 못 보고 해제 누락.
    //     ② 0 판정이 **별개의 비원자 읽기**라, 두 스레드가 동시에 dec하면 **둘 다 0을 보고
    //        drop_slow를 두 번 호출 = 이중해제**.
    //   xadd는 이전 값을 돌려주므로 "내가 1→0으로 만든 유일한 스레드"인지 원자적으로 알 수 있다.
    let mut prev: u64 = u64::MAX;   // -1을 더한다 = 1 감소
    core::arch::asm!("lock xadd qword ptr [{p}], {v}", p = in(reg) data, v = inout(reg) prev,
                     options(nostack));
    if prev == 1 {
        let pair: [u64; 2] = [data as u64, vt as u64];
        let f: extern "system" fn(usize) = core::mem::transmute(exe_base() + 0x28b50);
        f(pair.as_ptr() as usize);
    }
}

/// ★공여자 슬롯3을 템플릿으로 확보(1회). Arc를 붙잡아 두어 공여자가 죽어도 유효하게 만든다.
/// 템플릿 실패 사유를 사유별로 남긴다(공용 prov_skip 상한에 묻히지 않게).
unsafe fn tmpl_fail(world: usize, who: &str, why: &str) {
    // ⚠단순 카운터 상한(10)은 매 스캔틱 같은 사유가 반복돼 **금방 소진**된다
    //   (2026-08-26 실사고: 레벨1 실패 10건으로 상한을 다 쓰고 그 뒤가 안 보였다).
    //   ⟹ **내용이 바뀔 때만** 찍는다. 그러면 레벨이 오르는 과정과 최종 결말이 다 남는다.
    let msg = format!("  ✗[템플릿 실패] world={:#x} 공여자 '{}' — {}
", world, who, why);
    {
        let mut g = TMPL_FAIL_LAST.lock().unwrap_or_else(|x| x.into_inner());
        if *g == msg { return; }
        *g = msg.clone();
    }
    if TMPL_FAIL_N.fetch_add(1, Ordering::Relaxed) < 40 { hlog(&msg); }
}

unsafe fn slot_tmpl_get(world: usize, who: &str) -> Option<Vec<u64>> {
    {
        let g = SLOT_TMPL.lock().unwrap_or_else(|x| x.into_inner());
        if let Some((w, n)) = g.as_ref() {
            if n == who { return Some(w.clone()); }
            hlog(&format!("[템플릿 교체] {} → {} (공여자 변경)
", n, who));
        }
    }
    // ★[v120] 실패 사유를 갈라서 찍는다. 전부 None 이면 "왜 안 됐는지"를 추론해야 한다
    //   (2026-08-26 실사고: 무녀 이식 실패를 로그로 못 가려 팀 문제인지 레벨인지 추측했다).
    let src = match find_champ(world, who) {
        Some(v) => v,
        None => { tmpl_fail(world, who, "그 챔프가 이 월드에 없다(경기에 안 나왔거나 아직 스폰 전)"); return None; }
    };
    let lv = rd_u64(src + E_SKILL_CNT).unwrap_or(0);
    let b = src + E_SLOT0 + 3 * SLOT_STRIDE;
    let w = slot_words(b);
    if w.len() != SLOT_STRIDE / 8 { tmpl_fail(world, who, "슬롯 읽기 실패"); return None; }
    let (d, v) = (w[0] as usize, w[1] as usize);
    if !effect_sane(d, v) {
        tmpl_fail(world, who, &format!("effect 무효 data={:#x} vt={:#x} (레벨 {})", d, v, lv)); return None;
    }
    if (w[6] as u32) == u32::MAX {                          // gate -1 = 궁 슬롯 비어 있음
        tmpl_fail(world, who, &format!("궁 슬롯이 비어 있다(gate=-1) — 레벨 {} (궁은 5 이상 필요)", lv)); return None;
    }
    if !arc_incref(d) { tmpl_fail(world, who, "Arc inc 실패"); return None; }  // ★템플릿이 영구 참조 1개 보유
    hlog(&format!("★★[슬롯템플릿] {} 궁 확보 data={:#x} vt=RVA:{:#x} range={} growth={}          start={} casting_type={} (casting_target={} attack_type={} 는 **가져오지 않는다**)
",
        who, d, rva_of(w[1]), w[2], w[3], w[4], w[6] as u32,
        w[5] as u32, (w[5] >> 32) as u32));
    GRAFT_SRC_CT.store(w[6] as u32, Ordering::Relaxed);
    GRAFT_SRC_TGT.store(w[5] as u32, Ordering::Relaxed);
    GRAFT_SRC_ATK.store((w[5] >> 32) as u32, Ordering::Relaxed);
    // ★공여자 궁 쿨 = 그 챔프의 **궁 프로바이더 +0x170**(슬롯엔 쿨 필드가 없다).
    //   ⚠★★**네이티브 챔프는 프로바이더 struct가 DataActionDef(0x1a8)가 아니다** — `+0x170`이
    //     cooltime이 아니라 전혀 다른 필드다. v92는 여기에 값을 써서 **힙을 깼다**(knight 크래시,
    //     증거 = 읽힌 cooltime 1308622847). 읽기도 같은 이유로 쓰레기가 나온다.
    //   ⟹ ①할당 크기(vt+0x08) ≥ 0x1a8 ②값이 상식 범위 일 때만 채택하고, 아니면 **0(미확보)**.
    {
        let pd = rd_u64(src + E_PROV_ULT).unwrap_or(0) as usize;
        let pv = rd_u64(src + E_PROV_ULTV).unwrap_or(0) as usize;
        let psz = if in_exe(pv as u64) { rd_u64(pv + 0x08).unwrap_or(0) } else { 0 };
        let raw = if pd >= 0x10000 { rd_u64(pd + PROV_COOL).unwrap_or(0) } else { 0 };
        let c = if psz >= 0x1a8 && raw > 0 && raw < 1_000_000 { raw } else {
            hlog(&format!("  [쿨 채취 불가] {} 프로바이더 size={:#x} raw={} — DataActionDef가 아니거나 값이 비상식 ⟹ graft_cool은 이 공여자에 적용하지 않는다
", who, psz, raw));
            0
        };
        GRAFT_SRC_COOL.store(c, Ordering::Relaxed);
        // ★[v129] **오더 강등 위험 진단** — 게임함수를 호출하지 않고 vtable 슬롯 **값만** 읽어
        //   기본 스텁과 대조한다(shadow-call 금지 규칙 준수, CLAUDE.md §3).
        //   `+0xf0`/`+0x110`이 기본이 아니면 그 공여자는 ct=1/2일 때 궁 오더가 Move로
        //   강등될 수 있다 ⟹ `order_mask=1`이 필요한 후보다.
        {
            let vt = w[1] as usize;
            let def = { let mut g = SY_VT_DEF.lock().unwrap_or_else(|x| x.into_inner());
                        if g.is_none() { *g = default_eff_stubs(); } *g };
            let (a, b) = (rd_u64(vt + VT_I_SKILLSHOT * 8), rd_u64(vt + VT_I_AOE * 8));
            let mk = |v: Option<u64>, d: Option<u64>| -> String {
                match (v, d) { (Some(x), Some(dv)) if x == dv => "기본".into(),
                               (Some(x), _) => format!("★RVA:{:#x}", rva_of(x)),
                               _ => "?".into() } };
            hlog(&format!("  [오더술어] {} 궁 vt+0xf0={} vt+0x110={} (casting_type={}) — ★가 있고 casting_type이 1/2면 궁 오더가 Move로 강등될 수 있다(order_mask 후보)
",
                who, mk(a, def.map(|d| d[3])), mk(b, def.map(|d| d[4])), w[6] as u32));
        }
        hlog(&format!("  [공여자 메타] casting_target={} attack_type={} cooltime={} (사일러스 = 7 / 1 / {})
",
            w[5] as u32, (w[5] >> 32) as u32, c, SY_COOL_ORIG.load(Ordering::Relaxed)));
    }
    *SLOT_TMPL.lock().unwrap_or_else(|x| x.into_inner()) = Some((w.clone(), who.to_string()));
    Some(w)
}

/// ★★사일러스 슬롯3에 공여자 궁을 **필드 단위로** 이식한다.
unsafe fn slot_install(world: usize, sy_key: u64, sy: usize) -> Option<String> {
    let st = rd_u64(sy + E_CAST_ST).unwrap_or(9);
    if st > 2 { return prov_skip(&format!("시전/행동 중 state={}", st)); }
    let b = sy + E_SLOT0 + 3 * SLOT_STRIDE;
    let cur = rd_u64(b).unwrap_or(0) as usize;
    // ★[v101] 템플릿을 **먼저** 확보한다. force_src를 바꿨을 때 재이식되게 하려면
    //   "이미 우리 것" 판정을 **현행 템플릿과 비교**해야 한다(옛 템플릿과 같으면 갈아야 한다).
    let want = { FORCE_SRC.lock().unwrap_or_else(|x| x.into_inner()).clone() };
    if want.is_empty() { return prov_skip("force_src 미설정"); }
    // ★[v120] 템플릿 실패는 **전용 카운터**로 찍는다. 공용 prov_skip(상한 40)에 섞으면
    //   "sylas 없음"이 상한을 다 먹어 이 사유가 안 보인다(2026-08-26 실사고 — 무녀 이식 실패를
    //   추론으로만 판정해야 했다). 상한 표본에는 사유별 카운터를 짝지을 것.
    let t = match slot_tmpl_get(world, &want) {
        Some(v) => v,
        None => {
            return None;   // 사유는 slot_tmpl_get 안에서 이미 찍었다
        }
    };
    let (td, tv) = (t[0] as usize, t[1] as usize);
    if !effect_sane(td, tv) { return prov_skip("템플릿 effect 무효"); }
    {
        let g = SLOT_MINE.lock().unwrap_or_else(|x| x.into_inner());
        let prev = g.iter().find(|q| q.0 == world && q.1 == sy_key).map(|q| q.2);
        drop(g);
        match prev {
            // ★현행 템플릿과 같을 때만 "이미 됨". 공여자를 바꿨으면 여기서 안 걸려 재이식된다.
            Some(p) if p == cur && cur == td && cur >= 0x10000 => {
                let cap = ULT_CD_MAX.load(Ordering::Relaxed);
                if cap > 0 { if let Some(c) = rd_u64(sy + E_ULT_CD) { if c > cap { wr_u64(sy + E_ULT_CD, cap); } } }
                // ★[v98] **AI 궁 게이트 진단** — can_use_ult(0x17f7e60)가 보는 입력을 그대로 찍는다.
                //   훅을 새로 걸지 않고도 "왜 궁을 안 쓰는가"를 좁힐 수 있다.
                //   게이트: level>=5 && ult_cd <= EFF - EFF/rank && 슬롯3 gate != -1
                //   EFF = max(1, cooltime*100 / max(1, 100 + [+0x400] + [+0x46c]))
                if AI_DIAG.fetch_add(1, Ordering::Relaxed) % 64 == 0 && AI_DIAG.load(Ordering::Relaxed) < 1600 {
                    let pd = rd_u64(sy + E_PROV_ULT).unwrap_or(0) as usize;
                    let cool = if pd >= 0x10000 { rd_u64(pd + PROV_COOL) } else { None };
                    let cdr = 100i64 + rd_u64(sy + 0x400).unwrap_or(0) as i32 as i64
                                     + rd_u64(sy + 0x46c).unwrap_or(0) as i32 as i64;
                    let cdr = if cdr < 1 { 1 } else { cdr };
                    let eff = cool.map(|c| { let v = (c as i64) * 100 / cdr; if v < 1 { 1 } else { v } });
                    let cd  = rd_u64(sy + E_ULT_CD).unwrap_or(0) as i64;
                    let lv  = rd_u64(sy + E_SKILL_CNT).unwrap_or(0);
                    let gate = rd_u64(b + 0x30).map(|v| v as u32 as i32);
                    let ctgt = rd_u64(b + 0x28).map(|v| v as u32);
                    hlog(&format!("[AI궁게이트] state={:?} level={} (>=5?{}) ult_cd={} EFF={:?}                          통과조건 cd<=EFF-EFF/rank | gate={:?}(!=-1?) casting_target={:?} range={:?}                          cooltime={:?} cdr={}
",
                        rd_u64(sy + E_CAST_ST), lv, lv >= 5, cd, eff, gate, ctgt, rd_u64(b + 0x10), cool, cdr));
                }
                return None;
            }
            Some(p) => { let _ = prov_skip(&format!("★슬롯 되돌려짐 key={} 우리={:#x} 현재={:#x}", sy_key, p, cur)); }
            None => {}
        }
    }
    let my = slot_words(b);
    let (od, ov, ogate) = (my[0] as usize, my[1] as usize, my[6] as u32);
    // ★★[v128] casting_target / attack_type 선택적 이식.
    //   ~~"공여자 값을 쓰면 AI가 궁 오더를 None으로 만든다"~~ 는 **귀속 오류**였다(RE 2026-08-25):
    //   v97/v98 A/B는 둘 다 casting_target=7 고정이었고 **casting_type만 달랐다** — 진범은 그쪽이다.
    //   casting_target은 `FUN_1417faeb0` 한 곳에서만 읽히고 발동 경로는 안 읽는다 ⟹ 이식 안전.
    let use_tgt = if GRAFT_TGT.load(Ordering::Relaxed) { t[5] as u32 } else { my[5] as u32 };
    let use_atk = if GRAFT_ATK.load(Ordering::Relaxed) { (t[5] >> 32) as u32 } else { (my[5] >> 32) as u32 };
    let keep_w5 = (use_tgt as u64) | ((use_atk as u64) << 32);

    if !arc_incref(td) { return prov_skip("템플릿 Arc inc 실패"); }
    let msg = format!("★★[슬롯 이식] world={:#x} key={} sylas ent={:#x} ← {} 궁
           data {:#x}→{:#x} | range {}→{} | growth {}→{} | start {}→{} | casting_type {}→{}
           casting_target={} attack_type={} (graft_tgt={} graft_atk={} — OFF면 사일러스 것 유지)
",
        world, sy_key, sy, want, od, td, my[2], t[2], my[3], t[3], my[4], t[4],
        ogate, if GRAFT_CT.load(Ordering::Relaxed) { t[6] as u32 } else { my[6] as u32 },
        keep_w5 as u32, (keep_w5 >> 32) as u32,
        GRAFT_TGT.load(Ordering::Relaxed), GRAFT_ATK.load(Ordering::Relaxed));
    hlog(&msg);   // ★위험한 쓰기 전에 관측을 흘려보낸다

    // ★[v98] casting_type을 공여자 것으로 쓸지 A/B로 가른다.
    //   ct=1(Position)로 바꾼 v97에서 **궁 시전이 0건**이 됐다. ct가 원인인지 격리해야 한다.
    //   ct=0(사일러스 유지)면 궁은 나가되 장판 effect가 tag!=2로 무음 리턴할 것이다
    //   — 즉 "궁은 나가는데 안 보임"이면 ct가 범인, "여전히 안 나감"이면 다른 곳이다.
    let w6 = if GRAFT_CT.load(Ordering::Relaxed) { t[6] } else { my[6] };
    // ★★★[v105] 슬롯에 심는 vtable은 **원본이 아니라 AI 판정 3슬롯을 가린 사본**이다.
    //   my[1] = 사일러스 자기 궁 vtable(= 기본 구현 샘플). 실패하면 원본을 그대로 쓴다.
    let w1 = ai_mask_vt(t[1] as usize, my[1] as usize).unwrap_or(t[1] as usize) as u64;
    let ok = wr_u64(b, t[0]) && wr_u64(b + 8, w1) && wr_u64(b + 0x10, t[2])
          && wr_u64(b + 0x18, t[3]) && wr_u64(b + 0x20, t[4])
          && wr_u64(b + 0x28, keep_w5) && wr_u64(b + 0x30, w6);
    if !ok { return Some("  ✗슬롯 기입 실패
".into()); }
    if ogate != u32::MAX && od != td { arc_dec_drop(od, ov); }   // 옛 Arc 놓아주기

    // ★★[v128] 궁 쿨 = **프로바이더 +0x170**에서 온다(슬롯엔 쿨 필드가 없다 — RE 쿨관리).
    //   이식하지 않으면 뺏은 궁이 전부 사일러스 쿨(900)로 돌아, 원본이 3000인 궁은 **3.3배** 자주 나간다.
    //   ⚠사일러스 원본 값을 최초 1회 보존한다(force_src를 바꿔도 원본을 잃지 않게).
    {
        let pd = rd_u64(sy + E_PROV_ULT).unwrap_or(0) as usize;
        if pd >= 0x10000 {
            if SY_COOL_ORIG.load(Ordering::Relaxed) == 0 {
                if let Some(c) = rd_u64(pd + PROV_COOL) { SY_COOL_ORIG.store(c, Ordering::Relaxed); }
            }
            // 공여자 쿨을 못 캤으면(네이티브 struct 등) **사일러스 원본으로 되돌린다** —
            // 직전 공여자의 값이 남아 있으면 안 된다.
            let src_c = GRAFT_SRC_COOL.load(Ordering::Relaxed);
            let want = if GRAFT_COOL.load(Ordering::Relaxed) && src_c > 0 {
                src_c
            } else { SY_COOL_ORIG.load(Ordering::Relaxed) };
            if want > 0 {
                if let Some(now) = rd_u64(pd + PROV_COOL) {
                    if now != want {
                        wr_u64(pd + PROV_COOL, want);
                        hlog(&format!("  ★[쿨 이식] 사일러스 궁 cooltime {} → {} (graft_cool={})
",
                            now, want, GRAFT_COOL.load(Ordering::Relaxed)));
                    }
                }
            }
        }
    }
    let cd = rd_u64(sy + E_ULT_CD);
    wr_u64(sy + E_ULT_CD, 0);
    {
        let mut g = SLOT_MINE.lock().unwrap_or_else(|x| x.into_inner());
        g.retain(|q| !(q.0 == world && q.1 == sy_key));
        if g.len() >= 32 { g.remove(0); }
        g.push((world, sy_key, td));
    }
    GRAFT_N.fetch_add(1, Ordering::Relaxed);
    Some(format!("  ✓이식 완료 (쿨 {:?}→0) 누적 {}회
{}",
        cd, GRAFT_N.load(Ordering::Relaxed), dump_slots(sy, "sylas(이식후)")))
}
// ★★★[v96] world별 사일러스 **SlotMap 키** 캐시.
//   ~~raw Entity* 캐시~~ 는 치명적 결함이었다(v95 실측): 미니언이 상시 스폰/사망해
//   dense Vec이 realloc·swap_remove 되므로 **엔티티 포인터는 매 프레임 무효화될 수 있다**.
//   실제로 `ent+0x5a0`이 **스택 주소(0xc9b3e98620)** 로 읽혔고, 우리는 그 죽은 주소에
//   프로바이더를 쓰고 조립기까지 호출하고 있었다. (RE가 명시적으로 경고한 함정)
//   ⟹ 키만 캐시하고 **쓸 때마다 resolve로 재도출**한다.
static SY_CACHE: Mutex<Vec<(usize, u64)>> = Mutex::new(Vec::new());

/// world에서 이름으로 챔피언의 **SlotMap 키**를 찾는다(포인터가 아니라 키를 쓴다).
unsafe fn find_champ_key(world: usize, name: &str) -> Option<u64> {
    let slot_len = rd_u64(world + W_SLOT_LEN).unwrap_or(0).min(2048);
    for k in 0..slot_len {
        let e = resolve(world, k);
        if e < 0x10000 { continue; }
        if !is_champion(e) { continue; }
        if ent_name(e).as_deref() == Some(name) { return Some(k); }
    }
    None
}

/// 키 → 엔티티. **매번 재도출**해야 stale 포인터를 피한다.
unsafe fn champ_by_key(world: usize, key: u64, name: &str) -> Option<usize> {
    let e = resolve(world, key);
    if e < 0x10000 || !is_champion(e) { return None; }
    if ent_name(e).as_deref() != Some(name) { return None; }
    Some(e)
}

/// world에서 이름으로 챔피언 엔티티를 찾는다.
unsafe fn find_champ(world: usize, name: &str) -> Option<usize> {
    let slot_len = rd_u64(world + W_SLOT_LEN).unwrap_or(0).min(2048);
    for k in 0..slot_len {
        let e = resolve(world, k);
        if e < 0x10000 { continue; }
        if !is_champion(e) { continue; }
        if ent_name(e).as_deref() == Some(name) { return Some(e); }
    }
    None
}

/// `__clone_box` 호출 — 프로바이더 Box의 **실복사**.
unsafe fn clone_box(vt: usize, data: usize) -> usize {
    let f = match rd_u64(vt + PV_CLONE) { Some(v) if in_exe(v) => v as usize, _ => return 0 };
    let f: extern "system" fn(usize) -> usize = core::mem::transmute(f);
    f(data)
}

/// ★★사일러스의 궁 프로바이더를 피해자 것으로 교체한다.
///   반환 = 로그(교체했을 때만 Some).
/// ★[v91] 건너뛴 이유를 반드시 남긴다. v90은 전부 조용한 None이라 **왜 안 됐는지 로그가 0줄**이었다.
unsafe fn prov_skip(why: &str) -> Option<String> {
    if PROV_DIAG.fetch_add(1, Ordering::Relaxed) < 40 {
        hlog(&format!("  [prov skip] {}
", why));
    }
    None
}

unsafe fn prov_install(world: usize, sy_key: u64, sy: usize) -> Option<String> {
    // 시전 중에는 손대지 않는다 — 프로바이더가 바뀌면 시전 길이(L)가 흔들려
    // 미발화/이중발화가 생긴다(RE: P3는 L_raw를 매 틱 재계산, den0는 개시 스냅샷).
    let st = rd_u64(sy + E_CAST_ST).unwrap_or(9);
    if st > 2 { return prov_skip(&format!("시전/행동 중 state={}", st)); }

    // 이미 우리 것이 심겨 있나 (ent + 정확한 포인터 일치로 판정)
    let cur = rd_u64(sy + E_PROV_ULT).unwrap_or(0) as usize;
    {
        let g = PROV_MINE.lock().unwrap_or_else(|x| x.into_inner());
        let prev = g.iter().find(|q| q.0 == world && q.1 == sy_key).map(|q| q.2);
        drop(g);
        match prev {
            Some(p) if p == cur && cur >= 0x10000 => {
                // ★[v95] 검증 편의: `ult_cd_max=N`이면 잔여 쿨을 N 이하로 눌러 궁을 자주 보게 한다.
                //   0(기본)이면 아무것도 안 한다 = 뺏은 궁의 원래 쿨 그대로.
                let cap = ULT_CD_MAX.load(Ordering::Relaxed);
                if cap > 0 {
                    if let Some(c) = rd_u64(sy + E_ULT_CD) {
                        if c > cap { wr_u64(sy + E_ULT_CD, cap); }
                    }
                }
                return None;
            }
            // ★[v94] 우리가 심었는데 값이 바뀌었다 = **되돌려졌다**. 반드시 드러낸다.
            //   (v93 실측: 같은 ent가 두 번 교체됐고 그 사이에 자기 궁이 나갔다)
            Some(p) => { let _ = prov_skip(&format!("★되돌려짐 key={} ent={:#x} 우리={:#x} 현재={:#x} — 재설치", sy_key, sy, p, cur)); }
            None => {}
        }
    }

    // 템플릿 확보 (최초 1회) — 피해자에게서 Box를 실복사해 영구 보관한다.
    let want = { FORCE_SRC.lock().unwrap_or_else(|x| x.into_inner()).clone() };
    let tmpl = {
        let mut g = PROV_TMPL.lock().unwrap_or_else(|x| x.into_inner());
        if g.is_none() {
            if want.is_empty() { return prov_skip("force_src 미설정"); }
            let src = match find_champ(world, &want) { Some(v) => v, None => return prov_skip(&format!("world에 {} 없음", want)) };
            let sd = rd_u64(src + E_PROV_ULT).unwrap_or(0) as usize;
            let sv = rd_u64(src + E_PROV_ULTV).unwrap_or(0) as usize;
            if sd < 0x10000 || !in_exe(sv as u64) { return prov_skip(&format!("{} 프로바이더 무효 d={:#x} vt={:#x}", want, sd, sv)); }
            let nd = clone_box(sv, sd);
            if nd < 0x10000 { return prov_skip(&format!("clone_box 실패 (vt+0x48={:?})", rd_u64(sv + PV_CLONE))); }
            *g = Some((nd, sv));
            *PROV_TMPL_WHO.lock().unwrap_or_else(|x| x.into_inner()) = want.clone();
            hlog(&format!("★★[템플릿] {} 궁 프로바이더 복제 보관 data={:#x} vt=RVA:{:#x}                  size={:#x} align={:?} clone=RVA:{:#x}
  (size<0x1a8 이면 **네이티브 챔프** =                  DataActionDef 아님 ⟹ +0x170/+0x178 오프셋 사용 금지)
",
                want, nd, rva_of(sv as u64), rd_u64(sv + 0x08).unwrap_or(0),
                rd_u64(sv + 0x10), rva_of(rd_u64(sv + PV_CLONE).unwrap_or(0))));
        }
        (*g)?
    };
    let (td, tv) = tmpl;

    // 사일러스 원본 쿨 값 보존
    let my_d = cur;
    let my_v = rd_u64(sy + E_PROV_ULTV).unwrap_or(0) as usize;
    if my_d < 0x10000 || !in_exe(my_v as u64) { return prov_skip(&format!("사일러스 프로바이더 무효 d={:#x} vt={:#x}", my_d, my_v)); }
    let my_cool = rd_u64(my_d + PROV_COOL);
    let my_uses = rd_u64(my_d + PROV_USES);

    // 템플릿에서 이 엔티티용 Box를 새로 복제(엔티티마다 별개 소유여야 한다)
    let nd = clone_box(tv, td);
    if nd < 0x10000 { return prov_skip("템플릿 clone_box 실패"); }
    let mut msg = format!("★★[프로바이더 교체] world={:#x} sylas ent={:#x} ← {} 궁 (new={:#x} 옛={:#x})
",
        world, sy, want, nd, my_d);
    // ★★★[v93] 쿨 패치는 **구조가 같을 때만** 한다.
    //   v92 크래시 원인: knight는 **네이티브 챔피언**이라 프로바이더 struct가 데이터챔프와 다르다.
    //   `+0x170`(데이터챔프의 cooltime)이 knight에선 전혀 다른 필드이고, 할당 크기(vt+0x08)를
    //   넘길 수도 있다. 거기에 사일러스 쿨(300)을 써 넣어 **힙을 깨뜨렸다**
    //   (실측 증거: 읽어본 "뺏은 쪽 cooltime"이 1308622847 = 명백한 쓰레기값).
    //   ⟹ ①할당 크기가 `DataActionDef`(0x1a8) 이상이고 ②읽은 값이 상식 범위일 때만 쓴다.
    let psz = rd_u64(tv + 0x08).unwrap_or(0);
    let their_cool = rd_u64(td + PROV_COOL);
    let sane = psz >= 0x1a8
        && their_cool.map(|c| c > 0 && c < 1_000_000).unwrap_or(false)
        && my_cool.map(|c| c > 0 && c < 1_000_000).unwrap_or(false);
    if PROV_KEEPCOOL.load(Ordering::Relaxed) && sane {
        if let Some(c) = my_cool { wr_u64(nd + PROV_COOL, c); }
        if let Some(u) = my_uses { wr_u64(nd + PROV_USES, u); }
        msg.push_str(&format!("  쿨=사일러스 유지(cooltime={:?} uses={:?}) / 뺏은 쪽={:?}
",
            my_cool, my_uses, their_cool));
    } else {
        msg.push_str(&format!("  쿨 패치 생략 — prov_size={:#x} 뺏은쪽cooltime={:?} 내cooltime={:?} (keepcool={} sane={})
             ⟹ 쿨은 **뺏은 궁 것**이 그대로 적용된다(구조가 달라 안전하게 못 고침)
",
            psz, their_cool, my_cool, PROV_KEEPCOOL.load(Ordering::Relaxed), sane));
    }
    // ★위험한 조작 전에 관측을 흘려보낸다
    hlog(&msg);

    // 옛 Box를 임시 쌍으로 떠 놓고 새 것을 기입
    let old: [u64; 2] = [my_d as u64, my_v as u64];
    if !(wr_u64(sy + E_PROV_ULT, nd as u64) && wr_u64(sy + E_PROV_ULTV, tv as u64)) {
        return Some("  ✗프로바이더 기입 실패
".into());
    }
    // 조립기 재호출 → 슬롯 4칸을 새 프로바이더 기준으로 재생산(옛 Arc 해제까지 게임이 처리)
    let asm: extern "system" fn(usize) = core::mem::transmute(exe_base() + RVA_ASSEMBLE);
    asm(sy);
    // 옛 프로바이더 Box 해제 (안 하면 힙 릭)
    let dropf: extern "system" fn(usize) = core::mem::transmute(exe_base() + RVA_BOXDROP);
    dropf(old.as_ptr() as usize);

    // ★★★[v95] **교체 순간 잔여 쿨 재스케일** — RE가 경고했는데 v90~v94에서 빠져 있었다.
    //   `ent+0xC8`은 "사일러스 쿨 기준으로 쌓인 절대 틱값"인데 프로바이더를 바꾸면
    //   EFF(유효 쿨)의 의미가 통째로 달라진다. 그대로 두면 게이트
    //   `ult_cd <= EFF - EFF/rank` 가 한참 동안 거짓이 되어 **궁을 거의 못 쓴다**(유저 실측).
    //   ⟹ 교체 시점에 0으로 눕혀 "지금 바로 쓸 수 있음" 상태로 만든다.
    let cd_before = rd_u64(sy + E_ULT_CD);
    wr_u64(sy + E_ULT_CD, 0);
    msg.push_str(&format!("  쿨 재스케일: ult_cd {:?} → 0 (교체로 EFF 기준이 바뀌므로 눕힌다)
", cd_before));
    {
        let mut g = PROV_MINE.lock().unwrap_or_else(|x| x.into_inner());
        g.retain(|q| !(q.0 == world && q.1 == sy_key));
        if g.len() >= 32 { g.remove(0); }
        g.push((world, sy_key, nd));
    }
    PROV_N.fetch_add(1, Ordering::Relaxed);
    Some(format!("  ✓조립기 재호출 완료. 슬롯3 gate={:?} 누적 {}회
{}",
        rd_u64(sy + E_ULT_CT), PROV_N.load(Ordering::Relaxed), dump_slots(sy, "sylas(프로바이더 교체후)")))
}
/// `slot_full=1` = 슬롯 0x38 전체를 복사(param 포함). 기본은 `{data,vtable}` 16B만.
static SLOT_FULL: AtomicBool = AtomicBool::new(false);
/// ★★[v63] 보유는 **world 단위**, 장전 이력은 **entity 단위**로 분리한다.
///   ①게임은 백그라운드 sim을 rayon으로 병렬 실행해 여러 world가 동시에 돈다
///     ⟹ A world에서 뺏어 B world에 장전하면 영원히 발동하지 않는다(v60 실측: world 불일치 15연발).
///   ②★**챔프는 죽고 부활할 때마다 새 entity가 된다**(유저 지시 2026-08-24).
///     v61에서 "장전한 ent(…f14f0) ≠ 궁 쏜 ent(…fa2b0), 차이 = ENT_STRIDE×21"로 관측된 것이
///     양 팀 사일러스가 아니라 **부활로 갈린 같은 사일러스**였다.
///     ⟹ "한 번 장전하고 끝"은 성립하지 않는다. **사일러스를 볼 때마다 장전 상태를 유지**해야 한다.
/// 보유분(world 단위): (world, 챔프명, data, vtable)
type Held = (usize, String, usize, usize);
static HELD: Mutex<Vec<Held>> = Mutex::new(Vec::new());
/// ★강탈 궁의 **gate(casting_type)** — world별. cctx tag를 무엇으로 줄지 결정하는 근거.
///   0=Targeting / 1=Position / 2=Direction / -1=None (RE 2026-08-25 확정).
static HELD_GATE: Mutex<Vec<(usize, i32)>> = Mutex::new(Vec::new());
/// ★★[v85] 챔프별 궁 슬롯 캐시 (world, 이름) → (data, vt, gate).
///   이 게임은 챔프가 죽고 부활할 때마다 새 entity가 되므로
///   "지금 이름으로 찾기"는 **간헐 실패**한다(force_src·keep 갱신이 둘 다 같은 함정).
///   ⇒ 살아 있을 때 본 슬롯3을 캐시해 두고, 즉시 탐색이 실패하면 캐시를 쓴다.
static SLOT_CACHE: Mutex<Vec<(usize, String, usize, usize, i32)>> = Mutex::new(Vec::new());
/// ★★[v86] **뷰 싱크 게이트** — 백그라운드 sim 걸러내기(RE 2026-08-25).
///   effect apply의 **10번째 인자 = `&Option<*mut TickEvents>`**(= e+0x50).
///   `*(u64*)arg10 == 0` 이면 그 틱은 **뷰 명령을 생성하지 않는다** = 화면에 안 보임.
///   유저 지적("10번 발동은 백그라운드 경기 아닌가")에 대한 직접 답. 훅 추가 0개.
///   ⚠정확히는 "틱 이벤트 수집 여부"라 하이라이트 캡처도 1이다 — 1차 필터로만 쓴다.
static LIVE_ONLY: AtomicBool = AtomicBool::new(true);
static VIEW_LIVE_N: AtomicU32 = AtomicU32::new(0);
static VIEW_BG_N: AtomicU32 = AtomicU32::new(0);
/// apply 인자에서 뷰 싱크가 살아있는지 — true면 화면에 보이는 틱(추정).
#[inline] unsafe fn view_sink_live(e: usize) -> bool {
    match rd_u64(e + 0x50) {
        Some(p) if p >= 0x10000 => rd_u64(p as usize).unwrap_or(0) != 0,
        _ => true,   // 읽기 실패면 막지 않는다(보수적)
    }
}
const SLOT_CACHE_MAX: usize = 64;
/// ★★표시 경기 판별(유저 지적 2026-08-25): 백그라운드 sim은 rayon 워커,
///   화면에 보이는 경기는 메인 스레드에서 틱한다 ⇒ world별 tid를 남겨 가린다.
static MAIN_TID: AtomicU32 = AtomicU32::new(0);
/// ★★[v88] **게임** 메인 스레드 tid. ~~MAIN_TID~~ 는 `cfg_refresher()`가 **모드가 띄운 스레드**에서
///   불려 자기 자신의 tid를 저장하고 있었다(v87 실측: 24516 = 게임 스레드 어디에도 없는 값).
///   ⟹ "표시 경기(메인 스레드)" 판정이 **항상 거짓**이었고, 로그의 25개 world가 전부
///   "백그라운드 추정"으로 찍혔다. 창을 소유한 스레드로 정확히 잡는다.
static GAME_TID: AtomicU32 = AtomicU32::new(0);
/// 표시 경기 게이트 사용 여부. cfg `live_gate=0`이면 끈다(전부 통과).
static LIVE_GATE: AtomicBool = AtomicBool::new(true);

extern "system" fn enum_wnd(hwnd: usize, lparam: isize) -> BOOL {
    unsafe {
        let mut pid = 0u32;
        let tid = GetWindowThreadProcessId(hwnd, &mut pid);
        if pid == GetCurrentProcessId() && IsWindowVisible(hwnd) != 0 && tid != 0 {
            *(lparam as *mut u32) = tid;
            return 0; // 첫 가시 창에서 중단
        }
    }
    1
}
/// 게임 창을 소유한 스레드 = 메인 스레드. 실패하면 0.
unsafe fn detect_game_tid() -> u32 {
    let mut out: u32 = 0;
    EnumWindows(enum_wnd, &mut out as *mut u32 as isize);
    out
}
/// ★이 호출이 **표시 경기 스레드**에서 일어났는가.
///   ⚠적용범위: 조합테스트 **빠른 시뮬은 전용 스레드**라 여기서 걸러진다(RE#14 경고).
///   조테까지 지원하려면 이 게이트를 조테 경로에 한해 완화해야 한다.
#[inline] unsafe fn on_game_thread() -> bool {
    let g = GAME_TID.load(Ordering::Relaxed);
    g == 0 || GetCurrentThreadId() == g
}
static TID_SEEN: Mutex<Vec<(usize, u32)>> = Mutex::new(Vec::new());

unsafe fn cache_slot3(world: usize, name: &str, ent: usize) {
    if rd_u64(ent + E_SKILL_CNT).unwrap_or(0) < 5 { return; }
    let b = ent + E_SLOT0 + 3 * SLOT_STRIDE;
    let d = rd_u64(b + SLOT_DATA).unwrap_or(0) as usize;
    let v = rd_u64(b + SLOT_VT).unwrap_or(0) as usize;
    if !effect_sane(d, v) { return; }
    let g = rd_u64(b + SLOT_GATE).unwrap_or(0) as u32 as i32;
    let mut c = SLOT_CACHE.lock().unwrap_or_else(|x| x.into_inner());
    if let Some(e) = c.iter_mut().find(|e| e.0 == world && e.1 == name) {
        e.2 = d; e.3 = v; e.4 = g;
    } else {
        if c.len() >= SLOT_CACHE_MAX { c.remove(0); }
        c.push((world, name.to_string(), d, v, g));
    }
}
unsafe fn cached_slot3(world: usize, name: &str) -> Option<(usize, usize, i32)> {
    let got = {
        let c = SLOT_CACHE.lock().unwrap_or_else(|x| x.into_inner());
        c.iter().find(|e| e.0 == world && e.1 == name).map(|e| (e.2, e.3, e.4))
    };
    let (d, v, g) = got?;
    if effect_sane(d, v) { Some((d, v, g)) } else { None }
}
/// 장전 이력(entity 단위, 원복용): (world, ent, 원본 0x38 스냅샷)
static PATCHED: Mutex<Vec<(usize, usize, Vec<u64>)>> = Mutex::new(Vec::new());
const HOLD_MAX: usize = 12;
const PATCH_MAX: usize = 64;
/// ★★[v64] **casting_ctx의 tag**가 effect 실행을 가른다 — `0x1395dd0`(기사 궁 자식) 디컴 첫 줄:
///   `if (resolve(world,target)==0 || *cctx != 2) return;` ⟹ **tag==2가 아니면 아무 일도 안 한다.**
///   tag==2일 때 읽는 것은 `cctx+8`·`cctx+0x10`의 **u64 2개**(지점 문맥 = 좌표로 추정).
///   반면 사일러스 궁은 `{tag=0, caster_key}`(엔티티 문맥)로 시전된다 ⟹ 강탈 궁이 즉시 return했다.
///   `cctx_fix=1` = 강탈 궁 발동 시 cctx를 **tag=2 + 좌표**로 교정한다.
static CCTX_FIX: AtomicBool = AtomicBool::new(false);
/// ★우리가 교정에 쓴 cctx 버퍼 주소. effect 훅에서 이 값과 같으면 **사일러스의 강탈 발동**임이 확정된다.
static CCTX_MY_PTR: AtomicU64 = AtomicU64::new(0);
/// ★★Combine은 자식에게 **cctx 사본**을 넘긴다(RE: `&cctx사본`).
///   ⇒ 포인터 비교로는 우리 발동분을 못 찾는다. **넣은 좌표값**으로 식별한다.
/// `zone_at=enemy` = 장판 좌표를 최근접 적으로(구 동작). 기본 = 사일러스 자기 위치.
static ZONE_AT_ENEMY: AtomicBool = AtomicBool::new(false);
static CCTX_MY_X: AtomicU64 = AtomicU64::new(0);
static CCTX_MY_Y: AtomicU64 = AtomicU64::new(0);
// ────────────────────────────────────────────────────────────────────────
// ★★[v67] **기사 궁 자식 effect(`0x1395dd0`) 직접 후킹** — "실행됐다"를 추측이 아니라 관측으로 만든다.
//   이 함수는 `if (resolve(world,target)==0 || *cctx != 2) return;` 로 시작한다.
//   진입 시 (cctx tag, target 해석 결과)를 찍으면 **게이트 통과 여부**가 그대로 드러난다.
//   프롤로그(0x1395dd0): push rbp/r15/r14/r13/r12/rsi/rdi/rbx(=12B) + sub rsp,0x2?? ⟹ 런타임에 길이 산출 대신
//   기존 install_detour_generic이 쓰는 방식과 동일하게 시그니처+길이를 명시한다.
const KZONE_RVA: usize = 0x138f7d0;
// ────────────────────────────────────────────────────────────────────────
// ★★[v79] **시전자 외형(CasterViewEffect) 이름 바꿔치기** — 강탈 궁을 써도 사일러스가 사라지지 않게.
//   `CasterViewEffect` apply(`0x1270980`)는 실무를 `FUN_1414bee30`에 넘긴다:
//     rdx = effect payload / `[rdx+8]` = **애니메이션 이름 문자열 ptr**, `[rdx+0x10]` = 길이
//     → 이름을 새 할당에 복사 → 0x50B 뷰 명령 조립 → kind=0xd 로 렌더 큐에 push
//   ⟹ 남의 궁을 쓰면 **사일러스에게 없는 이름**이 들어가 아무것도 안 그려진다(= 투명해짐, 유저 실측).
//   ⟹ 강탈 궁이 실행되는 동안에는 **사일러스 자기 궁의 이름**으로 바꿔 넘긴다.
//   ★게임 구조체는 건드리지 않는다 — 우리 버퍼를 만들고 **인자 rdx만** 우리 것으로 돌린다
//     (트램폴린이 rcx/rdx/r8/r9를 push→pop 하므로 `saved+0x20`(rdx) 쓰기가 실제로 반영된다).
const CVIEW_RVA: usize = 0x104f060;
const CVIEW_SIG: [u8; 11] = [0x55,0x41,0x57,0x41,0x56,0x56,0x57,0x53,0x48,0x81,0xec];
const CVIEW_LEN: usize = 15;   // push×6(8B) + sub rsp,0x88(7B), rip-rel 없음
const CVIEW_APPLY_RVA: usize = 0x1413830;
// ────────────────────────────────────────────────────────────────────────
// ★★★[v83] **idle 폴백** — 강탈 궁을 써도 사일러스가 사라지지 않게(정공법).
//   RE(2026-08-25, RE/2026-08-25_뷰소비-idle폴백지점.md) 확정:
//     뷰 명령 소비 FUN_140b43ed0 안에 **인라인 SwissTable** 로 이름→AnimData를 조회하고,
//     **실패하면 폴백 없이 다음 command로 skip**(0xb46fc8) — 시전자 스프라이트 갱신이 아예 안 일어난다.
//     이것이 "사일러스 투명"의 근본 원인. (이름 바꿔치기 방식은 연출 번들 전체가 바뀌어 폐기)
//   ⟹ **실패 exit의 `jmp rel32`(5B)만** 우리 스텁으로 돌려서, 레코드 String을 "idle"로 바꿔
//      조회 진입(0xb46dc5)으로 재점프한다. idle도 없으면 1회 재시도 후 포기(무한루프 방지).
//   ★레코드 레이아웃(확정): {cap@+0x00, name_ptr@+0x08, name_len@+0x10}, rbx=레코드.
//   ⚠**이름 포인터를 정적 문자열로 바꾸면 안 된다** — 게임이 그 String을 drop할 때 cap으로 free하므로
//     우리 정적 주소를 해제하게 된다. ⟹ **기존 버퍼에 제자리로 "idle"을 써넣고 len만 4로** 바꾼다(cap>=4일 때만).
const VIEWFAIL_RVA: usize = 0x8fe4d8;      // mov r14,[rbp+0x5c0] ; jmp 0x140b44160
const VIEWFAIL_JMP_OFF: usize = 7;         // 그 안에서 jmp rel32의 위치(+7, 5바이트)
const VIEWLOOKUP_RVA: usize = 0x8fe2d5;    // 조회 진입(재시도 대상)
const VIEWLOOP_RVA: usize = 0x8fb670;      // command 루프 선두(포기 시)
const VIEWFAIL_SIG: [u8; 12] = [0x4c,0x8b,0xb5,0xc0,0x05,0x00,0x00,0xe9,0x8c,0xd1,0xff,0xff];
/// `idle_fallback=1` = 위 패치를 설치(기본 OFF — 렌더 경로 mid-function 패치라 신중히).
static IDLE_FB: AtomicBool = AtomicBool::new(false);
static IDLE_INSTALLED: AtomicBool = AtomicBool::new(false);
/// 재시도 가드: 직전에 손댄 레코드 주소. 같은 레코드면 포기(= idle도 없는 경우).
static IDLE_LAST: AtomicU64 = AtomicU64::new(0);

/// ★rel32가 닿도록 **타깃 근처(±512MB)** 에 실행 페이지를 잡는다.
///   구 install_detour는 12B movabs+jmp라 거리 무관이지만, 여기는 5B rel32뿐이라 근처여야 한다.
unsafe fn alloc_near(target: usize, size: usize) -> usize {
    let step = 0x10000usize;
    let base = target & !(step - 1);
    for i in 1..(0x2000usize) {
        for &cand in [base.wrapping_add(i * step), base.wrapping_sub(i * step)].iter() {
            if cand < 0x10000 { continue; }
            let d = if cand > target { cand - target } else { target - cand };
            if d > 0x2000_0000 { continue; }
            let p = VirtualAlloc(cand, size, 0x1000 | 0x2000, 0x40);
            if p != 0 { return p; }
        }
    }
    0
}

unsafe fn install_idle_fallback() -> Result<usize, String> {
    let fail_at = exe_base() + VIEWFAIL_RVA;
    if !readable(fail_at, 16) { return Err("실패exit 읽기 불가".into()); }
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fail_at as *const u8, cur.as_mut_ptr(), 12);
    if cur != VIEWFAIL_SIG {
        return Err(format!("바이트 불일치 실제={:02x?}", cur));
    }
    let stub = alloc_near(fail_at, 128);
    if stub == 0 { return Err("근처 실행페이지 확보 실패(rel32 도달 불가)".into()); }
    let last_addr = core::ptr::addr_of!(IDLE_LAST) as u64;
    let mut b: Vec<u8> = Vec::new();
    b.extend_from_slice(&[0x50, 0x51, 0x52]);                       // push rax,rcx,rdx
    b.extend_from_slice(&[0x48, 0xb8]); b.extend_from_slice(&last_addr.to_le_bytes()); // movabs rax,&IDLE_LAST
    b.extend_from_slice(&[0x48, 0x8b, 0x08]);                       // mov rcx,[rax]
    b.extend_from_slice(&[0x48, 0x39, 0xd9]);                       // cmp rcx,rbx
    let je_at = b.len(); b.extend_from_slice(&[0x74, 0x00]);         // je give_up (뒤에 채움)
    b.extend_from_slice(&[0x48, 0x89, 0x18]);                       // mov [rax],rbx
    b.extend_from_slice(&[0x48, 0x8b, 0x0b]);                       // mov rcx,[rbx]      ; cap
    b.extend_from_slice(&[0x48, 0x83, 0xf9, 0x04]);                 // cmp rcx,4
    let jb_at = b.len(); b.extend_from_slice(&[0x72, 0x00]);         // jb give_up
    b.extend_from_slice(&[0x48, 0x8b, 0x53, 0x08]);                 // mov rdx,[rbx+8]    ; ptr
    b.extend_from_slice(&[0xc7, 0x02, 0x69, 0x64, 0x6c, 0x65]);     // mov dword [rdx],"idle"
    b.extend_from_slice(&[0x48, 0xc7, 0x43, 0x10, 0x04, 0x00, 0x00, 0x00]); // mov qword [rbx+0x10],4
    b.extend_from_slice(&[0x5a, 0x59, 0x58]);                       // pop rdx,rcx,rax
    b.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]);     // jmp [rip+0]
    b.extend_from_slice(&((exe_base() + VIEWLOOKUP_RVA) as u64).to_le_bytes());
    let give_up = b.len();
    b.extend_from_slice(&[0x5a, 0x59, 0x58]);                       // pop rdx,rcx,rax
    b.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]);     // jmp [rip+0]
    b.extend_from_slice(&((exe_base() + VIEWLOOP_RVA) as u64).to_le_bytes());
    // 짧은 점프 오프셋 채우기
    b[je_at + 1] = (give_up - (je_at + 2)) as u8;
    b[jb_at + 1] = (give_up - (jb_at + 2)) as u8;
    if b.len() > 128 { return Err("스텁 과대".into()); }
    core::ptr::copy_nonoverlapping(b.as_ptr(), stub as *mut u8, b.len());
    // 실패 exit의 jmp rel32(+7, 5바이트)를 우리 스텁으로
    let jmp_at = fail_at + VIEWFAIL_JMP_OFF;
    let rel = (stub as i64) - (jmp_at as i64 + 5);
    if rel < i32::MIN as i64 || rel > i32::MAX as i64 { return Err("rel32 범위 초과".into()); }
    let mut patch = [0u8; 5];
    patch[0] = 0xe9;
    patch[1..].copy_from_slice(&(rel as i32).to_le_bytes());
    let mut old: u32 = 0;
    if VirtualProtect(jmp_at, 5, 0x40, &mut old) == 0 { return Err("VirtualProtect".into()); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), jmp_at as *mut u8, 5);
    VirtualProtect(jmp_at, 5, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), jmp_at, 5);
    Ok(stub)
}
// ────────────────────────────────────────────────────────────────────────
// ★★[v80] **AI 궁 사용 판단이 강탈 궁 기준인가** — 평가 함수를 직접 후킹해 판정한다.
//   `FUN_140d4d0b0` = AI 평가 함수(이전 세션 규명: game_ai/utils.rs + splitmix64).
//   진입부 실측(0.5.6):
//     `r12 = [r8]` → `rbx = [r12]`(effect data) / `r15 = [r12+8]`(**effect vtable**)
//     `rdi = [r9+0x5c0]`  ⟹ **r9 = 평가 대상 entity**
//     이후 `call [r15+0xe8]` / `[r15+0x140]` / `[r15+0x28]` = **effect vtable의 AI 가치 평가 인터페이스**
//   ⟹ AI는 **슬롯의 {data,vtable}를 그대로 평가**한다. 우리가 바꾼 것이 정확히 그것이므로
//     구조상 "강탈 궁 기준"이어야 한다 — 이 훅으로 실측 확인한다.
const AIEVAL_RVA: usize = 0xeba9d0;
const AIEVAL_SIG: [u8; 11] = [0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x55];
const AIEVAL_LEN: usize = 16;   // push×8(12B) + sub rsp,0x28(4B), rip-rel 없음
/// `ai_probe=1` = 사일러스에 대한 AI 평가가 **어떤 effect vtable**로 들어오는지 기록.
static AI_PROBE: AtomicBool = AtomicBool::new(false);
static AI_N: AtomicU32 = AtomicU32::new(0);

unsafe fn on_aieval(saved: usize, _e: usize) {
    if !AI_PROBE.load(Ordering::Relaxed) { return; }
    let r9 = *((saved + 0x10) as *const u64) as usize;   // 평가 대상 entity
    // ★hot path — 가장 싼 필터를 최상단에. 사일러스가 아니면 즉시 반환.
    let se = SYLAS_ENT.load(Ordering::Relaxed) as usize;
    if se == 0 || r9 != se { return; }
    if AI_N.fetch_add(1, Ordering::Relaxed) >= 30 { return; }
    let r8 = *((saved + 0x18) as *const u64) as usize;
    let pair = rd_u64(r8).unwrap_or(0) as usize;
    if pair < 0x10000 { return; }
    let data = rd_u64(pair).unwrap_or(0) as usize;
    let vt = rd_u64(pair + 8).unwrap_or(0) as usize;
    let apply = rd_u64(vt + EFF_APPLY).unwrap_or(0);
    // 보유 중인 강탈 궁과 대조 — 일치하면 **AI가 강탈 궁을 평가하고 있다**는 직접 증거.
    let hit = {
        let g = HELD.lock().unwrap_or_else(|x| x.into_inner());
        g.iter().find(|h| h.2 == data && h.3 == vt).map(|h| h.1.clone())
    };
    hlog(&format!("◉[AI평가] 대상=sylas ent={:#x} | effect data={:#x} vt=RVA:{:#x} apply=RVA:{:#x} ⟹ {}\n",
        r9, data, rva_of(vt as u64), rva_of(apply),
        match hit { Some(w) => format!("★★강탈 궁({})을 평가 중 = AI 판단이 강탈 궁 기준", w),
                    None => "사일러스 자기 스킬".to_string() }));
}
unsafe extern "C" fn cap_aieval(saved: usize, e: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| on_aieval(saved, e)));
}
/// `view_fix=0` 으로만 끈다(기본 ON).
static VIEW_FIX: AtomicBool = AtomicBool::new(true);
/// 사일러스 자기 궁의 외형 이름 — **우리 버퍼에 복사해 둔다**(원본 effect가 해제돼도 안전).
static SYLAS_VIEW: Mutex<Option<Vec<u8>>> = Mutex::new(None);
static VIEW_N: AtomicU32 = AtomicU32::new(0);
/// ★`keep=1` = 한 번 빼앗으면 **원복하지 않고 계속 쓴다**(테스트·검증용).
///   강탈 궁이 매 R마다 나가므로 육안 확인이 훨욱 쉽다. 사슬 붙잡기는 그동안 나가지 않는다.
static KEEP_STOLEN: AtomicBool = AtomicBool::new(false);
thread_local! {
    /// 바꿔치기용 payload 사본 {?, str_ptr, str_len, ?} + 이름 바이트 보관소
    static VIEW_TMP: core::cell::UnsafeCell<([u64; 4], [u8; 64])> =
        core::cell::UnsafeCell::new(([0; 4], [0; 64]));
    /// 강탈 궁이 방금 실행에 들어갔다 = 다음 CasterViewEffect 호출을 우리 이름으로 돌린다.
    static STOLEN_VIEW: core::cell::Cell<u64> = core::cell::Cell::new(0);
}

unsafe fn on_cview(saved: usize, _e: usize) {
    if !VIEW_FIX.load(Ordering::Relaxed) { return; }
    // ★유효 시간 창(≈50ms) 안에서만 — 강탈 Combine 직후의 호출만 잡는다.
    let mark = STOLEN_VIEW.with(|c| c.get());
    if mark == 0 { return; }
    STOLEN_VIEW.with(|c| c.set(0));
    let name = { SYLAS_VIEW.lock().unwrap_or_else(|x| x.into_inner()).clone() };
    let Some(nm) = name else { return };
    if nm.is_empty() || nm.len() > 64 { return; }
    let rdx = *((saved + 0x20) as *const u64) as usize;   // rdx = effect payload
    if rdx < 0x10000 { return; }
    let w0 = rd_u64(rdx).unwrap_or(0);
    let old_len = rd_u64(rdx + 0x10).unwrap_or(0);
    let newp = VIEW_TMP.with(|c| {
        let b = c.get();
        let bufp: *mut ([u64; 4], [u8; 64]) = b;
        let namep: *mut u8 = core::ptr::addr_of_mut!((*bufp).1) as *mut u8;
        core::ptr::copy_nonoverlapping(nm.as_ptr(), namep, nm.len());
        let hdr: *mut u64 = core::ptr::addr_of_mut!((*bufp).0) as *mut u64;
        *hdr = w0;
        *hdr.add(1) = namep as u64;
        *hdr.add(2) = nm.len() as u64;
        *hdr.add(3) = 0;
        hdr as u64
    });
    *((saved + 0x20) as *mut u64) = newp;                 // ★rdx만 우리 것으로
    if VIEW_N.fetch_add(1, Ordering::Relaxed) < 10 {
        hlog(&format!("◈[외형교체] 강탈 궁의 시전자 외형 이름(len={}) → 사일러스 것 {:?} 로 대체\n",
            old_len, String::from_utf8_lossy(&nm)));
    }
}
unsafe extern "C" fn cap_cview(saved: usize, e: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| on_cview(saved, e)));
}
/// 사일러스 자기 궁 Combine에서 CasterViewEffect 이름을 **바이트로 복사해** 보관한다.
unsafe fn capture_sylas_view(selfp: usize) {
    if SYLAS_VIEW.lock().unwrap_or_else(|x| x.into_inner()).is_some() { return; }
    let ptr = rd_u64(selfp + 8).unwrap_or(0) as usize;
    let len = rd_u64(selfp + 0x10).unwrap_or(0) as usize;
    if ptr < 0x10000 || len == 0 || len > 32 { return; }
    for i in 0..len {
        let d = rd_u64(ptr + i * EFF_STRIDE).unwrap_or(0) as usize;
        let v = rd_u64(ptr + i * EFF_STRIDE + 8).unwrap_or(0) as usize;
        if d < 0x10000 || !in_exe(v as u64) { continue; }
        if rd_u64(v + EFF_APPLY).map(|a| rva_of(a) as usize) != Some(CVIEW_APPLY_RVA) { continue; }
        let sp = eff_self(d, v);
        let np = rd_u64(sp + 8).unwrap_or(0) as usize;
        let nl = rd_u64(sp + 0x10).unwrap_or(0) as usize;
        if np < 0x10000 || nl == 0 || nl > 60 { continue; }
        if let Some(bytes) = read_bytes(np, nl) {
            hlog(&format!("◈[외형포착] 사일러스 궁 시전자 외형 = {:?}\n", String::from_utf8_lossy(&bytes)));
            *SYLAS_VIEW.lock().unwrap_or_else(|x| x.into_inner()) = Some(bytes);
        }
        return;
    }
}
const KZONE_SIG: [u8; 11] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57];
const KZONE_LEN: usize = 19;   // push×8(12B) + sub rsp,imm32(7B)
/// `zone_probe=1` = 이 effect 진입을 로깅한다(게이트 통과 여부 확정).
static ZONE_PROBE: AtomicBool = AtomicBool::new(false);
static ZONE_N: AtomicU32 = AtomicU32::new(0);

unsafe fn on_kzone(saved: usize, e: usize) {
    if !ZONE_PROBE.load(Ordering::Relaxed) { return; }
    // 계약: rcx=self(effect payload) rdx=? r8=world r9=WorldOps / [rsp+0x28]=p5 [rsp+0x30]=p6 [rsp+0x38]=p7(cctx)
    let world = *((saved + 0x18) as *const u64) as usize;   // r8
    let sw = SYLAS_WORLD.load(Ordering::Relaxed);
    // ★★사일러스 world **에서만** 기록한다. `sw != 0 &&` 로 느슨하게 두면 사일러스를 보기 전
    //   백그라운드 sim(병렬 world)이 로그 상한을 다 써버려 정작 우리 경기가 안 남는다(2026-08-24 실측:
    //   ▣ 40건이 전부 다른 world였고 강탈은 그 뒤에 일어났다).
    if sw == 0 || sw != world as u64 { return; }            // ★hot path 최상단 필터(§안전)
    let cctx_p = rd_u64(e + 0x38).unwrap_or(0) as usize;
    let cc = read_cctx(cctx_p);
    let (mx, my) = (CCTX_MY_X.load(Ordering::Relaxed), CCTX_MY_Y.load(Ordering::Relaxed));
    // ★포인터가 아니라 **내용**으로 판별한다(Combine이 사본을 넘기기 때문).
    let mine = mx != 0 && cc.map(|x| x.0 == 2 && x.1 == mx && x.2 == my).unwrap_or(false);
    // ★우리 발동분은 상한 없이 전부 남긴다.
    if !mine {
        let n = ZONE_N.fetch_add(1, Ordering::Relaxed);
        if n >= 60 { return; }
    }
    let wops = *((saved + 0x10) as *const u64) as usize;    // r9
    let p6   = rd_u64(e + 0x30).unwrap_or(0);
    let cctx = rd_u64(e + 0x38).unwrap_or(0) as usize;
    let c = read_cctx(cctx);
    let tag = c.map(|x| x.0).unwrap_or(0xffff_ffff);
    // 게이트 ①: resolve(world, p6) != 0  ②: tag == 2
    let tgt = resolve(world, p6);
    let pass = tgt != 0 && tag == 2;
    hlog(&format!("{}[기사장판 effect] world={:#x} wops=RVA:{:#x} p6={} tgt={:#x}({}) cctx={:?} ⟹ {}\n",
        if mine { "★★★[사일러스 강탈 발동] " } else { "▣" },
        world, rva_of(wops as u64), p6 as i64, tgt,
        if tgt != 0 { ent_name(tgt).unwrap_or_default() } else { String::new() },
        c, if pass { "★게이트 통과 = 장판 생성" } else { "✗조기 반환(아무 일도 안 함)" }));
}
unsafe extern "C" fn cap_kzone(saved: usize, e: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| on_kzone(saved, e)));
}

/// 강탈 당시 X의 cctx 원본 (tag, +8, +0x10) — 형식 대조용.
static SRC_CCTX: Mutex<Option<(u32, u64, u64)>> = Mutex::new(None);

/// ★★[v65] cctx는 **호출자 스택의 지역 구조체**이고, `tag=0`일 때는 **16바이트만 유효**하다.
///   실측(2026-08-24): 사일러스 시전 시 `cctx = (0, 576, 0xd1b0be7bf0)` — `+0x10`이 **스택 주소**였다.
///   거기에 좌표를 써서 호출자의 지역변수를 덮었고 **게임이 크래시**했다.
///   ⟹ 게임 구조체에 쓰지 않는다. **우리 소유 버퍼를 채우고 인자의 포인터만 바꿔치기**한다.
///   (인자 슬롯 쓰기가 원본 호출에 반영되는 것은 steal_cast의 `e+0x30` 사례로 검증됐다.)
thread_local! {
    static CCTX_TMP: core::cell::UnsafeCell<[u64; 8]> = core::cell::UnsafeCell::new([0; 8]);
}
/// cctx를 읽어 (tag, +8, +0x10)을 돌려준다.
unsafe fn read_cctx(cctx: usize) -> Option<(u32, u64, u64)> {
    if cctx < 0x10000 { return None; }
    Some((rd_u64(cctx)? as u32, rd_u64(cctx + 8)?, rd_u64(cctx + 0x10)?))
}

/// ★★[v72] 보관 중인 강탈 effect가 **아직 쓸 수 있는 상태인가**를 검사한다.
///   2026-08-25 크래시 실측: `AV @ RVA:0x1800808` = Combine 자식 순회 루프의
///   `mov rcx,[r10+0x10]`(r10 = 자식 vtable). **자식 배열이 썩어 있었다.**
///   원인 = 경기가 끝나고 world가 재사용되는데(조합 테스트 5연전) 우리는 **지난 경기의 effect**를
///   계속 들고 다음 경기 사일러스 슬롯에 심었다. Arc로 할당은 살렸지만 그 안이 가리키는
///   자식 배열·엔티티는 끝난 경기의 것이라 무효다.
///   ⟹ **심기 직전·발화 직전에 매번 검사**하고, 썩었으면 버린다(§메모리 안전: 의심스러우면 개입하지 않는다).
unsafe fn effect_sane(data: usize, vt: usize) -> bool {
    if data < 0x10000 || !in_exe(vt as u64) { return false; }
    if !readable(data, 0x20) { return false; }
    match rd_u64(vt + EFF_APPLY) { Some(a) if in_exe(a) => {}, _ => return false }
    // ★[v123] 자식 배열 검사는 **루트가 Combine 일 때만** 한다.
    //   ⚠구버전은 무조건 `selfp+8/+0x10` 을 자식 Vec 으로 읽어, 루트가 네이티브 단일 이펙트인
    //   공여자를 전부 거부했다(2026-08-26 실사고: 무녀 귀문 궁이 레벨 1~11 내내 "effect 무효").
    //   Combine 이 아닌 루트는 그 자리에 자식 배열이 없다 — 검사 대상이 아니다.
    let root_apply = match rd_u64(vt + EFF_APPLY) { Some(a) => rva_of(a) as usize, None => return false };
    if root_apply != COMBINE_RVA { return true; }          // 네이티브 단일 이펙트 = 여기까지로 충분
    let selfp = eff_self(data, vt);
    if !readable(selfp, 0x18) { return false; }
    let ptr = match rd_u64(selfp + 8) { Some(p) => p as usize, None => return false };
    let len = match rd_u64(selfp + 0x10) { Some(l) => l, None => return false };
    if ptr < 0x10000 || len == 0 || len > 32 { return false; }
    if !readable(ptr, (len as usize) * EFF_STRIDE) { return false; }
    for i in 0..len as usize {
        let d = rd_u64(ptr + i * EFF_STRIDE).unwrap_or(0) as usize;
        let v = rd_u64(ptr + i * EFF_STRIDE + 8).unwrap_or(0) as usize;
        if d < 0x10000 || !in_exe(v as u64) { return false; }
        // ★자식 vtable의 align/apply까지 읽어본다 — 크래시가 난 바로 그 접근이다.
        if rd_u64(v + EFF_ALIGN).is_none() { return false; }
        match rd_u64(v + EFF_APPLY) { Some(a) if in_exe(a) => {}, _ => return false }
    }
    true
}
/// ★★[v75] **원복은 시전이 끝난 뒤에** 한다.
///   게임은 한 번의 시전 동안 슬롯을 **여러 틱에 걸쳐 다시 읽어** 실행 큐에 넣는다.
///   발동을 감지하자마자 원복하면 그 다음 읽기에서 **사일러스 자기 궁이 또 나간다**
///   (2026-08-25 실측: 발동 직후 `[SYLAS-ULT] len=6`이 매번 뒤따랐다 = 유저가 본 "대체 안 됨"의 정체).
///   ⟹ 발동 시엔 **게이트만 -1(비활성)** 로 눌러 추가 발동을 막고,
///      `ent+0x70`(발동 슬롯)이 6에서 벗어나면 그때 원본 0x38을 통째로 되돌린다.
static PENDING: Mutex<Vec<(usize, usize, Vec<u64>)>> = Mutex::new(Vec::new());

/// 시전이 끝난 사일러스의 슬롯3을 원본으로 되돌린다. 아무 Combine 훅에서나 자주 불린다.
unsafe fn try_restore_pending(world: usize) {
    let mut done: Vec<(usize, bool)> = Vec::new();
    {
        let mut g = PENDING.lock().unwrap_or_else(|x| x.into_inner());
        if g.is_empty() { return; }
        g.retain(|(w, ent, orig)| {
            if *w != world { return true; }
            let still_sylas = rd_u64(*ent + E_KIND) == Some(0xd)
                && ent_name(*ent).as_deref() == Some("sylas");
            if !still_sylas { done.push((*ent, false)); return false; }   // 죽었으면 그냥 버린다
            if rd_u64(*ent + E_ACTIVE_SLOT) == Some(6) { return true; }   // 아직 시전 중 — 기다린다
            // ★★[v78] 되돌릴 **원본 스냅샷도 썰을 수 있다**(경기가 끝나면 그 effect도 무효).
            //   썰은 원본을 새 경기 사일러스에게 써 넣으면 그가 궁을 쓸 때 죽는다 ⇒ 검사 후에만 되돌린다.
            let od = orig.get(0).copied().unwrap_or(0) as usize;
            let ov = orig.get(1).copied().unwrap_or(0) as usize;
            if !effect_sane(od, ov) { done.push((*ent, false)); return false; }
            let b = *ent + E_SLOT0 + 3 * SLOT_STRIDE;
            let ok = orig.iter().enumerate().all(|(k, w2)| wr_u64(b + k * 8, *w2));
            done.push((*ent, ok));
            false
        });
    }
    for (ent, ok) in done {
        hlog(&format!("  [원복] ent={:#x} 시전 종료 후 슬롯3 복구 {}\n", ent, if ok { "OK" } else { "실패/폐기" }));
    }
}

/// 이 world의 보유분을 버린다(썩었거나 경기가 바뀐 경우).
unsafe fn drop_hold(world: usize, why: &str) {
    let removed = {
        let mut g = HELD.lock().unwrap_or_else(|x| x.into_inner());
        let before = g.len();
        g.retain(|h| h.0 != world);
        before != g.len()
    };
    { PATCHED.lock().unwrap_or_else(|x| x.into_inner()).retain(|q| q.0 != world); }
    { PENDING.lock().unwrap_or_else(|x| x.into_inner()).retain(|q| q.0 != world); }
    { HELD_GATE.lock().unwrap_or_else(|x| x.into_inner()).retain(|q| q.0 != world); }
    if removed { hlog(&format!("  [보유폐기] world {:#x} — {}\n", world, why)); }
}

/// ★사일러스를 볼 때마다 호출: 이 world에 보유분이 있으면 **슬롯3이 그것인지 확인하고, 아니면 쓴다.**
///   부활로 entity가 갈려도 새 entity에 다시 장전되므로 "보유 중"이 유지된다.
unsafe fn ensure_loaded(world: usize, ent: usize) -> Option<(String, bool)> {
    let held = {
        let g = HELD.lock().unwrap_or_else(|x| x.into_inner());
        g.iter().find(|h| h.0 == world).cloned()
    };
    let (_, who0, d0, v0) = held?;
    // ★★[v81] keep=1 은 원복을 안 하므로 **한 개의 effect 인스턴스를 계속 재사용**하게 된다.
    //   effect는 그 시전에 묶인 내부 상태를 가지므로(기존 6경로 폐기의 근본 원인)
    //   경기가 바뀌거나 원주인이 다시 쓰면 썰어 자식 배열이 무효가 된다
    //   (2026-08-25 크래시: AV @ 0x1800780 = Combine 자식 순회). ⇒ keep에서는 **매번 원주인에게서 새로 뜼온다**.
    let (who, d, v) = if KEEP_STOLEN.load(Ordering::Relaxed) && !who0.is_empty() {
        let slot_len = rd_u64(world + W_SLOT_LEN).unwrap_or(0).min(2048);
        let mut fresh = None;
        for k in 0..slot_len {
            let e2 = resolve(world, k);
            if e2 < 0x10000 { continue; }
            if ent_name(e2).as_deref() != Some(who0.as_str()) { continue; }
            if rd_u64(e2 + E_SKILL_CNT).unwrap_or(0) < 5 { continue; }
            let sb = e2 + E_SLOT0 + 3 * SLOT_STRIDE;
            let nd = rd_u64(sb + SLOT_DATA).unwrap_or(0) as usize;
            let nv = rd_u64(sb + SLOT_VT).unwrap_or(0) as usize;
            if effect_sane(nd, nv) {
                fresh = Some((nd, nv));
                // ★[v89] 원본 슬롯 0x38 전량을 같이 확보한다(gate·사거리·조준 플래그 포함).
                let w = slot_words(sb);
                let mut g = HELD_SRC.lock().unwrap_or_else(|x| x.into_inner());
                match g.iter_mut().find(|q| q.0 == world) {
                    Some(q) => q.1 = w,
                    None => { if g.len() < 32 { g.push((world, w)); } }
                }
            }
            break;
        }
        match fresh {
            Some((nd, nv)) => {
                if nd != d0 || nv != v0 {
                    arc_incref(nd);
                    let mut g = HELD.lock().unwrap_or_else(|x| x.into_inner());
                    if let Some(h) = g.iter_mut().find(|h| h.0 == world) { h.2 = nd; h.3 = nv; }
                }
                (who0, nd, nv)
            }
            // ★★[v84] 갱신은 **기회주의적**이어야 한다.
            //   원주인이 죽으면 이름으로 못 찾는데, 그때 보유를 통째 폐기하니
            //   희생자가 죽는 순간 강탈이 풀려 자기 궁으로 돌아갔다(2026-08-25 유저 실측).
            //   ⇒ **못 찾으면 기존 것을 유지**하고, 기존 것이 썰았을 때만 버린다.
            None => {
                if effect_sane(d0, v0) { (who0, d0, v0) }
                else { drop_hold(world, "keep: 신선본도 없고 보유본도 무효"); return None; }
            }
        }
    } else { (who0, d0, v0) };
    // ★★[v78] 슬롯3(+0x538)은 **`[ent+0x5c8] >= 5`일 때만 게임이 쓴다**(RE 확정: state 6 arm의 cmovae).
    //   그 미만이면 게임은 정적 기본 슬롯을 읽으므로 우리가 심어봐야 발동하지 않고,
    //   **심어둔 포인터만 남아 나중에 썰은 채로 읽힌다**(2026-08-25 크래시: skill_cnt=1 상태에 심은 직후 AV).
    let skc = rd_u64(ent + E_SKILL_CNT).unwrap_or(0);
    if skc < 5 {
        drop_hold(world, &format!("skill_cnt={} (<5) — 슬롯3이 아직 게임에 안 쓰인다", skc));
        return None;
    }
    // ★★심기 전 검사 — 썩은 effect를 슬롯에 넣으면 게임이 자식 순회에서 죽는다(0x1800808 AV).
    if !effect_sane(d, v) { drop_hold(world, "강탈 effect가 무효(경기 종료·재사용 추정)"); return None; }
    let b = ent + E_SLOT0 + 3 * SLOT_STRIDE;
    if rd_u64(b + SLOT_DATA) == Some(d as u64) && rd_u64(b + SLOT_VT) == Some(v as u64) {
        return None;   // 이미 장전돼 있다
    }
    let orig = slot_snapshot(ent, 3);
    // ★원본 스냅샷은 **그 entity에 대해 처음 한 번만** 보관한다(두 번 덮으면 원복이 깨진다).
    {
        let mut p = PATCHED.lock().unwrap_or_else(|x| x.into_inner());
        if !p.iter().any(|q| q.1 == ent) {
            if p.len() >= PATCH_MAX { p.remove(0); }
            p.push((world, ent, orig));
        }
    }
    let mut ok = wr_u64(b + SLOT_DATA, d as u64) && wr_u64(b + SLOT_VT, v as u64);
    // ★★[v89] **슬롯 나머지 필드도 원본 값으로** 채운다 — 특히 gate(+0x30).
    //   data/vt만 바꾸면 casting_type이 사일러스 것으로 남아 cctx tag가 어긋나고,
    //   대상 effect가 자기 게이트에서 조용히 리턴한다(장판 무생성의 직접 원인).
    let src = { HELD_SRC.lock().unwrap_or_else(|x| x.into_inner())
        .iter().find(|q| q.0 == world).map(|q| q.1.clone()) };
    let mut full = false;
    if let Some(w) = src {
        // 스냅샷이 지금 심는 effect의 것인지 확인(다른 챔프 것을 섞으면 안 된다)
        if w.len() == SLOT_STRIDE / 8 && w[0] == d as u64 && w[1] == v as u64 {
            for k in 2..w.len() { ok &= wr_u64(b + k * 8, w[k]); }
            full = true;
        }
    }
    if !full {
        // 폴백: 최소한 gate(+0x30)만이라도 강탈 대상 값으로 맞춘다.
        let gt = { HELD_GATE.lock().unwrap_or_else(|x| x.into_inner())
            .iter().find(|q| q.0 == world).map(|q| q.1) };
        if let Some(gv) = gt { ok &= wr_u64(b + SLOT_GATE, gv as i64 as u64); }
    }
    hlog(&format!("  [장전상세] 슬롯3 ← {} (전량={} gate={:?} +0x2c={:?})
",
        who, full, rd_u64(b + SLOT_GATE), rd_u64(b + 0x28)));
    Some((who, ok))
}
static SWAP_N: AtomicU32 = AtomicU32::new(0);

/// ★지금 apply 중인 `self`가 이 entity의 몇 번 슬롯에서 나왔는지 **1:1로 확정**한다.
///   슬롯의 `{data,vtable}`로 self 보정식을 다시 계산해 현재 self와 비교 — 휴리스틱이 아니다.
unsafe fn find_slot_by_self(ent: usize, selfp: usize) -> Option<usize> {
    for n in 0..4usize {
        let b = ent + E_SLOT0 + n * SLOT_STRIDE;
        let d = rd_u64(b + SLOT_DATA).unwrap_or(0) as usize;
        let v = rd_u64(b + SLOT_VT).unwrap_or(0) as usize;
        if d < 0x10000 || !in_exe(v as u64) { continue; }
        if eff_self(d, v) == selfp { return Some(n); }
    }
    None
}
/// ★★[v89] 강탈 원본의 **슬롯 0x38 전량**(world별). ~~data/vt만 교체~~ 는 결함이었다:
///   슬롯은 `{data@0, vt@8, range@0x10, growth@0x18, start_timing@0x20,
///   attack_type@0x28, casting_target@0x2c, casting_type(gate)@0x30}` 인데
///   **gate를 사일러스 것(Targeting=0)으로 남겨두면** 시전 개시가 cctx에 tag 0을 넣고,
///   기사 장판 effect는 `*cctx != 2`면 **조용히 리턴**한다 ⟹ 장판이 안 생긴다
///   (2026-08-25 실측: 강탈 9회 성공·화면 무반응. 슬롯덤프 `[3] … gate=0`이 증거).
///   `+0x2c`(casting_target)도 **조준 리드 실행 게이트**라 원본 값이 필요하다(RE#15).
static HELD_SRC: Mutex<Vec<(usize, Vec<u64>)>> = Mutex::new(Vec::new());

/// 슬롯 베이스 주소에서 0x38 전량을 읽는다.
unsafe fn slot_words(base: usize) -> Vec<u64> {
    (0..(SLOT_STRIDE / 8)).map(|k| rd_u64(base + k * 8).unwrap_or(0)).collect()
}

unsafe fn slot_snapshot(ent: usize, n: usize) -> Vec<u64> {
    let b = ent + E_SLOT0 + n * SLOT_STRIDE;
    (0..(SLOT_STRIDE / 8)).map(|k| rd_u64(b + k * 8).unwrap_or(0)).collect()
}
/// 슬롯 쓰기. full=false면 `{data,vtable}` 16B만 바꾼다(최소 개입).
unsafe fn slot_write(ent: usize, n: usize, data: usize, vt: usize, full: &[u64]) -> bool {
    let b = ent + E_SLOT0 + n * SLOT_STRIDE;
    if !readable(b, SLOT_STRIDE) { return false; }
    if SLOT_FULL.load(Ordering::Relaxed) && full.len() == SLOT_STRIDE / 8 {
        for (k, w) in full.iter().enumerate() { if !wr_u64(b + k * 8, *w) { return false; } }
        return true;
    }
    wr_u64(b + SLOT_DATA, data as u64) && wr_u64(b + SLOT_VT, vt as u64)
}


unsafe fn dump_slots(ent: usize, who: &str) -> String {
    let kind = rd_u64(ent + E_KIND).unwrap_or(0);
    let act  = rd_u64(ent + E_ACTIVE_SLOT).unwrap_or(0);
    let cnt  = rd_u64(ent + E_SKILL_CNT).unwrap_or(0);
    let mut s = format!("  [슬롯] {} ent={:#x} kind={:#x}{} active(+0x70)={} skill_cnt(+0x5c8)={}\n",
        who, ent, kind, if kind == 0xd { "(데이터챔프)" } else { "" }, act, cnt);
    for n in 0..4usize {
        let b = ent + E_SLOT0 + n * SLOT_STRIDE;
        let data = rd_u64(b + SLOT_DATA).unwrap_or(0) as usize;
        let vt   = rd_u64(b + SLOT_VT).unwrap_or(0) as usize;
        let par  = rd_u64(b + SLOT_PARAM).unwrap_or(0);
        let gate = rd_u64(b + SLOT_GATE).unwrap_or(0) as u32 as i32;
        let ap   = if in_exe(vt as u64) { rd_u64(vt + EFF_APPLY).unwrap_or(0) } else { 0 };
        // ★궁 판별 = apply가 Combine이고 그 자식에 Grab이 있는가(사일러스 궁의 실측 특징)
        let mut tag = String::new();
        if rva_of(ap) as usize == COMBINE_RVA {
            tag.push_str(" Combine");
            let selfp = eff_self(data, vt);
            let kptr = rd_u64(selfp + 8).unwrap_or(0) as usize;
            let klen = rd_u64(selfp + 0x10).unwrap_or(0) as usize;
            if kptr >= 0x10000 && klen <= 32 {
                tag.push_str(&format!("(자식{})", klen));
                for i in 0..klen {
                    let kv = rd_u64(kptr + i * EFF_STRIDE + 8).unwrap_or(0) as usize;
                    if rd_u64(kv + EFF_APPLY).map(|a| rva_of(a) as usize) == Some(GRAB_RVA) {
                        tag.push_str(" ★Grab=궁");
                    }
                }
            }
        }
        s.push_str(&format!("    [{}] +{:#05x} data={:#x} vt=RVA:{:#x} apply=RVA:{:#x} param={:#x} gate={}{}{}\n",
            n, E_SLOT0 + n * SLOT_STRIDE, data, rva_of(vt as u64), rva_of(ap), par, gate,
            if gate == -1 { " (비활성)" } else { "" }, tag));
    }
    s
}

/// `buff_call=1` = `push_buff` 재구현 대신 **게임 함수를 그대로 호출**한다.
///   AddCasterBuff apply가 부르는 그 지점과 **동일 문맥**(Combine apply 자식 루프 안)이라 조건이 같다.
static BUFF_CALL: AtomicBool = AtomicBool::new(false);
/// `buff_probe=1` = 버프 리스트·페이로드 관찰.
static BUFF_PROBE: AtomicBool = AtomicBool::new(false);
/// `buff_graft=1` = 보유한 X 버프를 사일러스 궁 시전 시 **사일러스 버프 리스트에 push**.
static BUFF_GRAFT: AtomicBool = AtomicBool::new(false);
/// X 궁의 AddCasterBuff 페이로드 **값 사본**(0x120B) — 포인터를 들지 않는다.
static BUFF_CAP: Mutex<Option<(String, Vec<u8>, usize)>> = Mutex::new(None);
static BUFF_LOGGED: AtomicU32 = AtomicU32::new(0);

/// effect 자식의 self 보정식 = data + ((align-1)&!0xf) + 0x10 (재생 루프 0x1802680 실측).
#[inline] unsafe fn eff_self(data: usize, vt: usize) -> usize {
    let al = rd_u64(vt + EFF_ALIGN).unwrap_or(8) as usize;
    data + (al.wrapping_sub(1) & !0xf) + 0x10
}
unsafe fn read_bytes(addr: usize, n: usize) -> Option<Vec<u8>> {
    if addr < 0x10000 { return None; }
    let mut v: Vec<u8> = Vec::with_capacity(n);
    let mut o = 0usize;
    while o < n { let w = rd_u64(addr + o)?; v.extend_from_slice(&w.to_le_bytes()); o += 8; }
    v.truncate(n);
    Some(v)
}
/// 0x120 페이로드 워드 덤프. **포인터가 섞여 있는지**가 값이식 안전성의 핵심 판정이라 표시한다.
unsafe fn hex_payload(p: usize, tag: &str) -> String {
    let Some(b) = read_bytes(p, BUF_STRIDE) else { return format!("  [{}] 읽기 실패 @{:#x}\n", tag, p) };
    let mut s = format!("  [{}] payload @{:#x} (0x120B, 0인 워드는 생략)\n", tag, p);
    for i in 0..(BUF_STRIDE / 8) {
        let w = u64::from_le_bytes(b[i*8..i*8+8].try_into().unwrap());
        if w == 0 { continue; }
        let note = if in_exe(w) { format!("  <<exe RVA:{:#x}>>", rva_of(w)) }
            else if w >= 0x1000_0000_0000 && w < 0x8000_0000_0000 && readable(w as usize, 8) { "  <<힙포인터?>>".to_string() }
            else {
                let f = f32::from_bits((w & 0xffff_ffff) as u32);
                if f != 0.0 && f.abs() > 1e-4 && f.abs() < 1e9 { format!("  (f32lo={})", f) } else { String::new() }
            };
        s.push_str(&format!("    +{:#05x} {:#018x}{}\n", i*8, w, note));
    }
    s
}
/// ★버프 payload 선두 = **인라인 SSO 문자열** `{len:u32 @+0, 바이트 @+4}` (v51 실측:
///   "ninja_ult"(9) / "swordman_ult"(12) / "sylas_hijacked"(14) / "serpen_permanent_buff"(21)).
///   len 뒤 바이트는 이전 사용분 잔재라 의미 없다 — len만큼만 읽을 것.
fn buff_name(b: &[u8]) -> String {
    if b.len() < 8 { return String::new(); }
    let n = u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize;
    if n == 0 || n > b.len() - 4 || n > 64 { return String::new(); }
    String::from_utf8_lossy(&b[4..4 + n]).to_string()
}
unsafe fn dump_buff_list(ent: usize, who: &str) -> String {
    let cap = rd_u64(ent + E_BUF_CAP).unwrap_or(0);
    let ptr = rd_u64(ent + E_BUF_PTR).unwrap_or(0) as usize;
    let len = rd_u64(ent + E_BUF_LEN).unwrap_or(0);
    let mut s = format!("  [버프리스트] {} ent={:#x} cap={} ptr={:#x} len={}\n", who, ent, cap, ptr, len);
    if ptr < 0x10000 || len == 0 || len > 32 { return s; }
    for i in 0..len as usize {
        let it = ptr + i * BUF_STRIDE;
        let w: Vec<u64> = (0..6).map(|k| rd_u64(it + k*8).unwrap_or(0)).collect();
        s.push_str(&format!("    [{}] {:#x} {:#x} {:#x} {:#x} {:#x} {:#x}\n", i, w[0], w[1], w[2], w[3], w[4], w[5]));
    }
    s
}
/// ★버프 push **재구현**(게임 FUN_1417f9390 동형). 단 grow는 부르지 않는다 —
///   할당자 shadow-call은 위험도가 높고 실패하면 Vec 불변식이 깨져 게임 전체가 죽는다.
///   cap 여유가 없으면 **스킵**(§3 완전 재구현 + 메모리 안전).
/// ★스탯 캐시(`+0x618~+0x658`) — 버프 재합산 결과가 여기 쌓인다(`FUN_1417ea5f0`가 dirty 플래그로 갱신).
///   버프 이식이 **실효를 냈는지**를 육안 말고 수치로 보는 지표.
unsafe fn dump_stats(ent: usize) -> String {
    let mut s = String::from("  [스탯캐시] ");
    for k in 0..9 { s.push_str(&format!("{:#x} ", rd_u64(ent + 0x618 + k * 8).unwrap_or(0))); }
    s.push_str(&format!("| dirty={:#x}\n", rd_u64(ent + E_STAT_DIRTY).unwrap_or(0) & 0xff));
    s
}
unsafe fn push_buff(ent: usize, payload: &[u8]) -> Result<u64, String> {
    if payload.len() != BUF_STRIDE { return Err(format!("페이로드 크기 {}B", payload.len())); }
    let cap = rd_u64(ent + E_BUF_CAP).ok_or("cap 읽기 실패")?;
    let ptr = rd_u64(ent + E_BUF_PTR).ok_or("ptr 읽기 실패")? as usize;
    let len = rd_u64(ent + E_BUF_LEN).ok_or("len 읽기 실패")?;
    if ptr < 0x10000 { return Err(format!("ptr 무효 {:#x}", ptr)); }
    if cap > 0x1000 || len > cap { return Err(format!("Vec 불변식 이상 len={} cap={}", len, cap)); }
    if len >= cap { return Err(format!("cap 여유 없음 len={} cap={} (grow는 하지 않는다)", len, cap)); }
    let dst = ptr + (len as usize) * BUF_STRIDE;
    if !readable(dst, BUF_STRIDE) { return Err(format!("슬롯 접근 불가 {:#x}", dst)); }
    for i in 0..(BUF_STRIDE/8) {
        let w = u64::from_le_bytes(payload[i*8..i*8+8].try_into().unwrap());
        if !wr_u64(dst + i*8, w) { return Err(format!("쓰기 실패 @+{:#x}", i*8)); }
    }
    if !wr_u64(ent + E_BUF_LEN, len + 1) { return Err("len 증가 실패".into()); }
    // 스탯 dirty(1바이트) — 이웃 7바이트를 보존해야 하므로 read-modify-write.
    if let Some(w) = rd_u64(ent + E_STAT_DIRTY) { let _ = wr_u64(ent + E_STAT_DIRTY, (w & !0xff) | 1); }
    Ok(len + 1)
}
/// ★궁 판별 임계값. 100은 **너무 낮아 일반 스킬도 잡혔다**(악마 변신 = 궁이 아닌데 포착 — 유저 실측 2026-08-24).
///   궁 쿨은 보통 900+ (사일러스 궁 = 900). 점프 폭과 **절대값**을 함께 본다.
///   cfg `ult_jump=<N>` / `ult_min=<N>` 으로 조절.
static ULT_CD_JUMP: AtomicU64 = AtomicU64::new(500);
static ULT_CD_MIN:  AtomicU64 = AtomicU64::new(600);
static ULT_CD_SEEN: Mutex<Vec<(String, u64)>> = Mutex::new(Vec::new());
thread_local! { static IN_REPLAY: std::cell::Cell<bool> = std::cell::Cell::new(false); }

/// 게임의 재생 루프(FUN_141802630)가 자식을 부르는 방식 그대로 호출한다.
///   apply(self, p2, p3, p4, p5, p6, &cctx사본, p8, &blk사본, p10),  self = data + ((align-1)&!0xf) + 0x10
type Apply10 = unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> u64;

unsafe fn replay_children(who: &str, kids: &[(usize, usize)], saved: usize, e: usize) {
    let p2 = *((saved + 0x20) as *const u64) as usize;
    let p3 = *((saved + 0x18) as *const u64) as usize;
    let p4 = *((saved + 0x10) as *const u64) as usize;
    let (Some(p5), Some(p6), Some(p7), Some(p8), Some(p9), Some(p10)) =
        (rd_u64(e+0x28), rd_u64(e+0x30), rd_u64(e+0x38), rd_u64(e+0x40), rd_u64(e+0x48), rd_u64(e+0x50))
        else { hlog("  [재생] 스택 인자 읽기 실패\n"); return };
    // 원본 루프가 그러하듯 p7(3워드)·p9(48B)는 사본을 넘긴다(자식이 mutate해도 원본 오염 없음)
    let mut cctx = [0u64; 3];
    for i in 0..3 { cctx[i] = rd_u64(p7 as usize + i*8).unwrap_or(0); }
    let mut blk = [0u64; 6];
    for i in 0..6 { blk[i] = rd_u64(p9 as usize + i*8).unwrap_or(0); }

    // ★효과 정량 판정: 재생 전/후로 "대상"과 "시전자"의 HP를 읽는다.
    //   눈으로 데미지 2배를 가늠하는 대신, 로그만으로 "게임 상태가 실제로 바뀌었는가"를 확정한다.
    let world_p = p3 as usize;
    let tgt_key = rd_u64(p7 as usize + 8).unwrap_or(u64::MAX);   // casting_ctx+8 = 대상(v13 실측)
    let cas_key = p6 as u64;                                      // [rsp+0x30] = 시전자
    let tgt_ent = if tgt_key != u64::MAX { resolve(world_p, tgt_key) } else { 0 };
    let cas_ent = resolve(world_p, cas_key);
    let hp0_t = if tgt_ent != 0 { rd_u64(tgt_ent + E_CUR_HP) } else { None };
    let hp0_c = if cas_ent != 0 { rd_u64(cas_ent + E_CUR_HP) } else { None };
    // ★effect 리스트 길이 — 버프/스턴이 붙으면 증가한다(HP가 안 변해도 효과를 잡는다)
    let el0_t = if tgt_ent != 0 { rd_u64(tgt_ent + E_EFF_LEN) } else { None };
    let el0_c = if cas_ent != 0 { rd_u64(cas_ent + E_EFF_LEN) } else { None };

    // ★로그는 자식 호출 "직전마다" flush 한다 — v15는 루프 끝에 한 번만 써서 크래시 시 통째로 유실됐다.
    let mut ok = 0;
    hlog(&format!("★[재생] src={} kids={} 대상={:?}(hp={:?}) 시전자={:?}(hp={:?})\n",
        who, kids.len(),
        if tgt_ent != 0 { ent_name(tgt_ent) } else { None }, hp0_t,
        if cas_ent != 0 { ent_name(cas_ent) } else { None }, hp0_c));
    for (i, (data, vt)) in kids.iter().enumerate() {
        let apply = match rd_u64(vt + EFF_APPLY) { Some(v) if in_exe(v) => v as usize, _ => {
            hlog(&format!("   [{}] apply 무효 skip\n", i)); continue } };
        let align = rd_u64(vt + EFF_ALIGN).unwrap_or(8) as usize;
        let selfp = data.wrapping_add((align.wrapping_sub(1)) & !0xf).wrapping_add(0x10);
        let sc = rd_u64(*data).unwrap_or(0);   // Arc strong count — 0이면 이미 해제된 것
        if sc == 0 || sc > 0x100000 { hlog(&format!("   [{}] data={:#x} strong={:#x} 이상 skip\n", i, data, sc)); continue; }
        if !readable(selfp, 8) { hlog(&format!("   [{}] self={:#x} unreadable skip\n", i, selfp)); continue; }
        // ★이 호출 하나만의 효과를 격리 측정한다(호출 직전 ↔ 직후)
        let bt = if tgt_ent != 0 { rd_u64(tgt_ent + E_CUR_HP) } else { None };
        let bc = if cas_ent != 0 { rd_u64(cas_ent + E_CUR_HP) } else { None };
        let bet = if tgt_ent != 0 { rd_u64(tgt_ent + E_EFF_LEN) } else { None };
        let bec = if cas_ent != 0 { rd_u64(cas_ent + E_EFF_LEN) } else { None };
        // ★effect self의 첫 필드 = "caster/대상 effect 배열의 슬롯 인덱스"(Stun apply 디컴).
        //   남의 effect는 그 챔프 기준 인덱스를 들고 있어 사일러스 문맥에선 안 맞을 수 있다 — 그 값을 찍어 확인한다.
        let idx0 = rd_u64(selfp).unwrap_or(u64::MAX);
        hlog(&format!("   [{}] 호출직전 apply=RVA:{:#x} self={:#x} strong={:#x} | self[0](슬롯idx)={} 대상eff_len={:?} 시전자eff_len={:?}\n",
            i, rva_of(apply as u64), selfp, sc, idx0 as i64,
            if tgt_ent != 0 { rd_u64(tgt_ent + E_EFF_LEN) } else { None },
            if cas_ent != 0 { rd_u64(cas_ent + E_EFF_LEN) } else { None }));
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let f: Apply10 = core::mem::transmute(apply);
            f(selfp, p2 as usize, p3, p4, p5 as usize,
              p6 as usize, cctx.as_ptr() as usize, p8 as usize, blk.as_ptr() as usize, p10 as usize)
        }));
        let at = if tgt_ent != 0 { rd_u64(tgt_ent + E_CUR_HP) } else { None };
        let ac = if cas_ent != 0 { rd_u64(cas_ent + E_CUR_HP) } else { None };
        let aet = if tgt_ent != 0 { rd_u64(tgt_ent + E_EFF_LEN) } else { None };
        let aec = if cas_ent != 0 { rd_u64(cas_ent + E_EFF_LEN) } else { None };
        let dd = |x: Option<u64>, y: Option<u64>| match (x, y) { (Some(a), Some(b)) => (b as i64) - (a as i64), _ => 0 };
        let (t_hp, c_hp, t_ef, c_ef) = (dd(bt, at), dd(bc, ac), dd(bet, aet), dd(bec, aec));
        let mark = if t_hp != 0 || c_hp != 0 || t_ef != 0 || c_ef != 0 { "◀이 호출이 효과" } else { "" };
        match r {
            Ok(v) => { ok += 1; hlog(&format!("   [{}] OK ret={:#x} | 이호출Δ 대상HP{} 시전자HP{} 대상eff{} 시전자eff{} {}\n",
                        i, v & 0xffff_ffff, t_hp, c_hp, t_ef, c_ef, mark)); }
            Err(_) => hlog(&format!("   [{}] PANIC\n", i)),
        }
    }
    let hp1_t = if tgt_ent != 0 { rd_u64(tgt_ent + E_CUR_HP) } else { None };
    let hp1_c = if cas_ent != 0 { rd_u64(cas_ent + E_CUR_HP) } else { None };
    let el1_t = if tgt_ent != 0 { rd_u64(tgt_ent + E_EFF_LEN) } else { None };
    let el1_c = if cas_ent != 0 { rd_u64(cas_ent + E_EFF_LEN) } else { None };
    let d = |a: Option<u64>, b: Option<u64>| match (a, b) { (Some(x), Some(y)) => (y as i64) - (x as i64), _ => 0 };
    let (dht, dhc, det, dec) = (d(hp0_t, hp1_t), d(hp0_c, hp1_c), d(el0_t, el1_t), d(el0_c, el1_c));
    let verdict = if dht != 0 || dhc != 0 || det != 0 || dec != 0 { "★효과있음" } else { "무효과" };
    hlog(&format!("   → {}/{} 실행 | {} | HP 대상Δ{} 시전자Δ{} | effect수 대상 {:?}→{:?}(Δ{}) 시전자 {:?}→{:?}(Δ{})\n",
        ok, kids.len(), verdict, dht, dhc, el0_t, el1_t, det, el0_c, el1_c, dec));
}

unsafe fn on_combine(saved: usize, e: usize) {
    // ★직전 graft의 결과를 여기서 대조한다(원본 루프가 이미 실행을 마친 시점).
    {
        let taken = { GRAFT_PEND.lock().unwrap_or_else(|x| x.into_inner()).take() };
        if let Some((te, ce, h0t, h0c, e0t, e0c, ap)) = taken {
            let h1t = rd_u64(te + E_CUR_HP).unwrap_or(h0t);
            let h1c = rd_u64(ce + E_CUR_HP).unwrap_or(h0c);
            let e1t = rd_u64(te + E_EFF_LEN).unwrap_or(e0t);
            let e1c = rd_u64(ce + E_EFF_LEN).unwrap_or(e0c);
            let (dh, dc) = ((h1t as i64) - (h0t as i64), (h1c as i64) - (h0c as i64));
            let (de, dec) = ((e1t as i64) - (e0t as i64), (e1c as i64) - (e0c as i64));
            let v = if dh != 0 || dc != 0 || de != 0 || dec != 0 { "★효과있음" } else { "무효과" };
            hlog(&format!("   ↳[graft결과] apply=RVA:{:#x} | {} | 대상HPΔ{} 시전자HPΔ{} 대상effΔ{} 시전자effΔ{}\n",
                ap, v, dh, dc, de, dec));
        }
    }
    let selfp = *((saved + 0x28) as *const u64) as usize; // rcx = Combine self
    let world = *((saved + 0x18) as *const u64) as usize; // r8  = world
    let r9    = *((saved + 0x10) as *const u64) as usize; // r9  = WorldOps
    if !in_exe(r9 as u64) { return; }
    if world < 0x10000 || selfp < 0x10000 { return; }
    // 시전자 = [rsp+0x30] 키 (v13 실측으로 확정: A=시전자, casting_ctx+8=대상)
    let Some(caster_key) = rd_u64(e + 0x30) else { return };
    let cent = resolve(world, caster_key);
    if cent == 0 { return; }
    let Some(cname) = ent_name(cent) else { return };

    // 자식 배열 스냅샷 (직계 — graft 대상 슬롯 계산용)
    let kptr = rd_u64(selfp + 8).unwrap_or(0) as usize;
    let klen = rd_u64(selfp + 0x10).unwrap_or(0) as usize;
    let mut kids: Vec<(usize, usize)> = Vec::new();
    if kptr >= 0x10000 && klen >= 1 && klen <= 32 {
        for i in 0..klen {
            let it = kptr + i * EFF_STRIDE;
            let d = rd_u64(it).unwrap_or(0) as usize;
            let v = rd_u64(it + 8).unwrap_or(0) as usize;
            if d < 0x10000 || !in_exe(v as u64) { continue; }
            kids.push((d, v));
        }
    }
    // ★캡처용은 **중첩을 펼친** 말단 effect 목록을 쓴다
    let mut flat: Vec<(usize, usize)> = Vec::new();
    flatten_children(selfp, 0, &mut flat);

    // ★★★[v114] 강탈 궁이 **발동되긴 하는가**를 확정한다.
    //   v113 실측: 사일러스 궁 시전 10회, rush_state는 74회 표집 내내 None
    //   ⟹ ①Combine apply가 아예 안 불린다(미발화) ②불리는데 MoveToTarget이 tag 불일치로 조용히 리턴
    //      — 둘 중 어느 쪽인지 로그가 없어 못 갈랐다. 여기서 가른다.
    //   MoveToTarget::apply = RVA 0x182ea50 (RE 2026-08-25_이동effect-rush_state-전수 §1)
    // ⚠v114 실패: 조건이 "caster==sylas"뿐이라 **스킬2 Combine이 상한 10개를 다 먹었다**
    //   (사일러스 스킬2 = 자식2개 Combine, 궁보다 훨씬 자주 나간다) ⟹ 궁은 표본에 들지도 못함.
    //   ⟹ v115: **시전 상태 6(궁)일 때만** 로그 + 놓친 것이 없도록 **전체 분포를 따로 집계**.
    if cname == "sylas" {
        let st6 = rd_u64(cent + E_CAST_ST) == Some(6);
        let mut g = SY_COMBINE_TALLY.lock().unwrap_or_else(|x| x.into_inner());
        match g.iter_mut().find(|q| q.0 == flat.len() && q.1 == st6) { Some(q) => q.2 += 1,
            None => g.push((flat.len(), st6, 1u32)) }
    }
    if cname == "sylas" && rd_u64(cent + E_CAST_ST) == Some(6)
        && SY_FIRE_LOG.fetch_add(1, Ordering::Relaxed) < 10 {
        let aps: Vec<String> = flat.iter()
            .map(|(_, v)| format!("{:#x}", rva_of(rd_u64(*v + EFF_APPLY).unwrap_or(0)))).collect();
        let has_mtt = flat.iter().any(|(_, v)|
            rd_u64(*v + EFF_APPLY).map(|a| rva_of(a)) == Some(0x182ea50));
        // arg7(&cctx)의 실제 자리를 모르므로 후보 오프셋을 훑어 tag(0..3)로 식별한다
        let mut cand = String::new();
        for off in [0x20usize, 0x28, 0x30, 0x38, 0x40, 0x48] {
            let v = rd_u64(e + off).unwrap_or(0) as usize;
            let t = if v >= 0x10000 { rd_u64(v) } else { None };
            cand.push_str(&format!(" +{:#x}={:#x}{}", off, v,
                match t { Some(x) if x <= 3 => format!("(→tag {})", x), _ => String::new() }));
        }
        // ★★★[v116] 이 훅은 **원본 apply가 끝난 뒤** 도는 후크다 ⟹ 여기서 읽는 값이
        //   "MoveToTarget이 실행된 직후의 상태"다. 세 값이 원인을 가른다:
        //     ①rush_state(cent+0x308) — MIN이면 돌진이 안 걸린 것
        //     ②엔티티 cctx tag(cent+0x88) — MoveToTarget은 **tag 0 전용**, 아니면 조용히 리턴
        //     ③MoveToTarget의 speed(self+0x18) — 0이면 rush_state를 None으로 만들고 자식을 통째로 버린다
        //   (RE 2026-08-25_이동effect-rush_state-전수 §1·§2-2)
        let rs = rd_u64(cent + 0x308).unwrap_or(0);
        // ⚠+0x88은 u32 tag + u32 대상id 두 필드다(u64로 읽으면 0x1b7⚠⚠00처럼 보인다)
        let tag = rd_u64(cent + 0x88).map(|v| (v as u32, (v >> 32) as u32));
        let mtt = flat.iter().find(|(_, v)| rd_u64(*v + EFF_APPLY).map(|a| rva_of(a)) == Some(0x182ea50));
        let spd = mtt.map(|(d, v)| { let sp = eff_self(*d, *v); (sp, rd_u64(sp + 0x18), rd_u64(sp + 0x20)) });
        hlog(&format!("[강탈궁발동] sylas Combine self={:#x} 말단{}개 MoveToTarget={}\n           \
apply=[{}]\n           ★apply**직전**(진입훅) rush_state={:#x}({}) | cctx (tag,대상id)={:?}(tag 0이어야 통과) | MoveToTarget {:x?}\n           \
프레임 후보:{}\n",
            selfp, flat.len(), if has_mtt { "★있음" } else { "없음" }, aps.join(" "),
            rs, if rs == 0x8000_0000_0000_0000 { "None=돌진안걸림" } else { "★걸림" },
            tag, spd.map(|(s, sp, rg)| format!("self={:#x} speed={:?} range={:?}", s, sp, rg)),
            cand));
    }

    // ★★[v59] **궁 판별 = "이 Combine이 슬롯3에서 나왔는가"** (RE로 확정: 슬롯3 = +0x538 = 궁,
    //   전 챔프 `ent+0x70 == 6` 일치). self 보정식 역산으로 1:1 매칭하므로 휴리스틱이 아니다.
    //   ~~구: ult 쿨(+0xC8) 점프 감지(ult_jump/ult_min)~~ → 놓침이 많았다(2026-08-24 실측:
    //   knight 궁이 한 판 내내 한 번도 안 잡혔다). ~~구: 자식에 Grab 존재~~ → 사일러스 전용이라
    //   슬롯을 남의 궁으로 바꾸면 그 순간부터 판별이 죽는다.
    // ★★[v75] 시전이 끝난 사일러스가 있으면 여기서 원복한다(어떤 Combine에서나 자주 불린다).
    // ★★[v86] 백그라운드 sim은 여기서 걸러낸다(뷰 싱크 널체크).
    // ★★[v88] 표시 경기 판별을 **스레드**로 교체.
    //   ~~뷰싱크(arg10) 널체크~~ 는 v87 실측에서 **1677틱 전부 "표시"** 로 통과시켰다(백그라운드 0).
    //   RE#12가 이미 경고했다: arg10은 "화면 표시"가 아니라 "**틱 이벤트 수집**"이라
    //   배경 리그 sim도 하이라이트용으로 1을 준다 ⟹ 판별력이 없다.
    //   RE#14 확정: **표시 경기는 생성·틱이 전부 메인 스레드**, 배경 sim은 스폰 스레드.
    let live = on_game_thread();
    if live { VIEW_LIVE_N.fetch_add(1, Ordering::Relaxed); } else { VIEW_BG_N.fetch_add(1, Ordering::Relaxed); }
    // ★[v130] ~~`if live`~~ 게이트를 뗀다. 실측(2026-08-29 20:04) `[뷰싱크] 표시틱 0 / 백그라운드 576` —
    //   창-소유-스레드 판별은 신뢰할 수 없고(v91 이력) 그 뒤에 두면 census 가 **영원히 안 돈다**.
    //   궁 메타는 world 종류와 무관하게 같은 값이라 배경 sim 에서 읽어도 유효하다.
    ult_meta_census(world, live);
    // ★★★[v92] 프로바이더 교체는 **표시 경기 게이트보다 앞에서** 한다.
    //   ① v91 실측: `on_game_thread()`가 판에 따라 전부 차단(표시틱 0/백그라운드 479)했다가
    //      다른 판에선 38을 통과했다 — **창-소유-스레드 판별은 신뢰할 수 없다.**
    //      그 게이트 뒤에 두는 바람에 교체 시도가 0회였고 skip 로그조차 안 남았다.
    //   ② 애초에 이 개입엔 게이트가 **불필요하고 해롭다**: RE#14가 확정한
    //      "내 경기는 두 번 시뮬레이션된다"(서버 워커 + 클라 표시, 같은 seed) 문제는
    //      **모든 world에 똑같이 심어야** 화면과 확정 결과가 일치한다.
    //   ⟹ 배경 sim world에도 그대로 심는다. 엔티티 단위 멱등이라 중복도 무해.
    if (PROV_SWAP.load(Ordering::Relaxed) || SLOT_GRAFT.load(Ordering::Relaxed))
        && PROV_TICK.fetch_add(1, Ordering::Relaxed) % 2 == 0 {
        // ★키를 캐시하고 **쓸 때마다 resolve**한다. 포인터 캐시 금지(v95 실사고).
        let cached = { let g = SY_CACHE.lock().unwrap_or_else(|x| x.into_inner());
                       g.iter().find(|q| q.0 == world).map(|q| q.1) };
        let hit = cached.and_then(|k| champ_by_key(world, k, "sylas").map(|e| (k, e)));
        let sy = match hit {
            Some(v) => Some(v),
            None => match find_champ_key(world, "sylas") {
                Some(k) => {
                    let mut g = SY_CACHE.lock().unwrap_or_else(|x| x.into_inner());
                    g.retain(|q| q.0 != world);
                    if g.len() >= 24 { g.remove(0); }
                    g.push((world, k));
                    drop(g);
                    champ_by_key(world, k, "sylas").map(|e| (k, e))
                }
                None => None,
            },
        };
        match sy {
            Some((k, e)) => {
                let r = if PROV_SWAP.load(Ordering::Relaxed) { prov_install(world, k, e) }
                        else { slot_install(world, k, e) };
                if let Some(m) = r { hlog(&m); }
            }
            None => { let _ = prov_skip(&format!("world {:#x}에 sylas 없음", world)); }
        }
    }
    // ★★[v131] **live 게이트 앞으로 옮겼다.** 이 블록은 읽기 전용 로깅인데 게이트 뒤에 있어서,
    //   창-소유-스레드 판별이 실패해 표시틱 0 이 되면(실측 2026-08-29: 표시틱 0 / 백그라운드 2507)
    //   **궁이 실제로 나가도 로그가 한 줄도 안 남았다.** 그래서 '안 나간다'를 로그로 확인할 수 없었다.
    //   world/live 를 함께 찍어 표시경기·배경sim 을 구분한다.
    // ★★[v102] **이식한 effect가 실제로 실행됐는가**를 직접 찍는다.
    //   `[SYLAS-ULT]`은 caster 이름 역산에 의존해 놓치는 경우가 있다.
    //   여기서는 payload 주소가 우리 템플릿의 것과 같은지로 **1:1 판정**한다
    //   (Arc payload = data + 0x10, align 8 기준).
    {
        let td = { SLOT_TMPL.lock().unwrap_or_else(|x| x.into_inner())
            .as_ref().map(|(w, n)| (w[0] as usize, n.clone())) };
        if let Some((td, who)) = td {
            if selfp == td + 0x10 && GRAFT_FIRE.fetch_add(1, Ordering::Relaxed) < 40 {
                hlog(&format!("★★★[이식궁 실행!] {} 궁이 실제로 apply 됐다 — caster={:?} ent={:#x}                      world={:#x} 자식={} cctx={:?} 누적{}회 live={}
",
                    who, cname, cent, world,
                    rd_u64(selfp + 0x10).unwrap_or(0),
                    read_cctx(rd_u64(e + 0x38).unwrap_or(0) as usize),
                    GRAFT_FIRE.load(Ordering::Relaxed), live));
            }
        }
    }
    if !live && LIVE_GATE.load(Ordering::Relaxed) { return; }
    if SLOT_SWAP.load(Ordering::Relaxed) { try_restore_pending(world); }
    // ★살아 있을 때 본 궁 슬롯을 캐시(부활로 entity가 갈려도 강탈 재료를 잃지 않는다)
    if SLOT_SWAP.load(Ordering::Relaxed) { cache_slot3(world, &cname, cent); }
    // ★★[v90] 프로바이더 정본 교체 — 이 훅은 아무 챔프의 Combine에서나 발화하므로
    //   "사일러스가 뭔가를 시전할 때만" 이라는 기존 장전 트리거의 구조적 결함이 사라진다.
    //   리스폰하면 엔티티가 새로 생기고(=새 프로바이더), 여기서 곧 다시 심는다.

    {
        let tid = GetCurrentThreadId();
        let known = { TID_SEEN.lock().unwrap_or_else(|x| x.into_inner()).iter().any(|q| q.0 == world && q.1 == tid) };
        if !known {
            { let mut t = TID_SEEN.lock().unwrap_or_else(|x| x.into_inner()); if t.len() < 32 { t.push((world, tid)); } }
            let m = GAME_TID.load(Ordering::Relaxed);
            hlog(&format!("[스레드] world {:#x} ← tid {} {}
", world, tid,
                if m == 0 { "(게임 tid 미탐지 — 게이트 무효)" } else if tid == m { "★표시 경기(메인 스레드)" } else { "백그라운드 sim(차단)" }));
        }
    }
    let cast_slot = find_slot_by_self(cent, selfp);
    let from_ult_slot = cast_slot == Some(3);

    if cname == "sylas" {
        // ★사일러스가 있는 world 확정 — 이후 캡처는 이 world에서만 한다(궁이 아니어도 갱신).
        SYLAS_KEY.store(caster_key, Ordering::Relaxed);
        SYLAS_ENT.store(cent as u64, Ordering::Relaxed);
        let prev = SYLAS_WORLD.swap(world as u64, Ordering::Relaxed);
        if prev != world as u64 {
            hlog(&format!("[world] 사일러스 world = {:#x}{}\n", world,
                if prev != 0 { format!(" (이전 {:#x})", prev) } else { String::new() }));
        }
        // ★사일러스 궁 판별 = 자식 중 Grab(apply=GRAB_RVA)이 있는 Combine (실측: 궁만 Grab을 가진다)
        // ★슬롯3에서 나왔으면 궁이다(정본). Grab 자식 검사는 보조 — 슬롯을 남의 궁으로 바꾸면 Grab이 없다.
        let is_ult = from_ult_slot
            || kids.iter().any(|(_, v)| rd_u64(v + EFF_APPLY).map(|a| rva_of(a) as usize) == Some(GRAB_RVA));
        if CB_DIAG.fetch_add(1, Ordering::Relaxed) < 8 || is_ult {
            hlog(&dump_children(selfp, if is_ult { "SYLAS-ULT" } else { "SYLAS-Combine" }, &cname));
        }
        // ★★[v79] 사일러스 자기 궁의 시전자 외형 이름을 확보해 둔다(강탈 궁에 씌울 이름).
        if is_ult && VIEW_FIX.load(Ordering::Relaxed) { capture_sylas_view(selfp); }
        // ★★[v56] 스킬 슬롯 4칸 전수 덤프 — 어느 칸이 궁인지 확정한다
        if is_ult && SLOT_PROBE.load(Ordering::Relaxed) {
            hlog(&format!("★[슬롯덤프] 사일러스 궁 시점\n{}", dump_slots(cent, "sylas")));
        }
        // ★★[v63] ①발동 감지 → 원복 ②아니면 장전 상태 유지(부활로 새 entity가 돼도 다시 장전)
        if SLOT_SWAP.load(Ordering::Relaxed) && ARMED.load(Ordering::Relaxed) {
            let fired = {
                let g = HELD.lock().unwrap_or_else(|x| x.into_inner());
                g.iter().position(|h| h.0 == world && eff_self(h.2, h.3) == selfp)
            };
            // ★★[v72] 보유분이 이 world에 있는데 **썩었으면** 발화 전에 버린다.
            {
                let stale = {
                    let g = HELD.lock().unwrap_or_else(|x| x.into_inner());
                    g.iter().find(|h| h.0 == world).map(|h| (h.2, h.3))
                };
                if let Some((hd, hv)) = stale {
                    if !effect_sane(hd, hv) {
                        // 이미 슬롯에 심겨 있다면 **자식 수를 0으로** 만들어 순회를 막는다(가장 싼 무해화).
                        let mut p = PATCHED.lock().unwrap_or_else(|x| x.into_inner());
                        for q in p.iter().filter(|q| q.0 == world) {
                            let b2 = q.1 + E_SLOT0 + 3 * SLOT_STRIDE;
                            if rd_u64(b2 + SLOT_DATA) == Some(hd as u64) {
                                let _ = q.2.iter().enumerate().all(|(k, w)| wr_u64(b2 + k * 8, *w));
                            }
                        }
                        p.retain(|q| q.0 != world);
                        drop(p);
                        drop_hold(world, "발화 직전 검사에서 무효 판정 — 원복 후 폐기");
                    }
                }
            }
            if let Some(idx) = fired {
                // ★keep=1 이면 보유를 유지해 **매 R마다 강탈 궁이 나가게** 한다(검증용).
                let keep = KEEP_STOLEN.load(Ordering::Relaxed);
                let held_row = { let mut g = HELD.lock().unwrap_or_else(|x| x.into_inner()); g.remove(idx) };
                if keep { HELD.lock().unwrap_or_else(|x| x.into_inner()).push(held_row.clone()); }
                let (_, who, stolen_data, _) = held_row;
                // ★★cctx 교정 — 이 시점은 Combine apply **진입 전**이라 여기서 고치면 자식들이 그 값을 받는다.
                let cctx0 = rd_u64(e + 0x38).unwrap_or(0) as usize;
                let before = read_cctx(cctx0);
                let mut fixlog = format!("  [cctx·사일러스] {:?} | 원본표본 {:?}\n",
                    before, { SRC_CCTX.lock().unwrap_or_else(|x| x.into_inner()).clone() });
                hlog(&fixlog);   // ★먼저 흘려보낸다(이후 조작이 죽어도 관측값은 남는다)
                fixlog.clear();
                // ★★★[v82] **gate(casting_type) 기반 cctx 자동 공급** — RE 2026-08-25 두 건의 결합.
                //   슬롯 gate(+0x30) = casting_type enum: 0=Targeting / 1=Position / 2=Direction
                //   cctx tag       : 0=엔티티key   / 1=방향벡터  / 2=좌표
                //   ⟹ 대응: Targeting→tag0(게임이 이미 준다, 손대지 않음) / Position→tag2(좌표) / Direction→tag1(방향)
                //   ~~구: 자식에 지점형 effect(0x1395dd0)가 있으면 tag=2 강제~~ → **폐기**.
                //   그 방식은 아는 effect 하나에만 맞춘 땜질이라, 방향형 궁(돌진·넉백)을 뺏으면 또 무발동이었다.
                //   근거 = RE/2026-08-25_슬롯조립-게이트-쿨.md(gate 의미) + RE/2026-08-25_effect-cctx-계약분류표.md(tag 3분류).
                let gate = { HELD_GATE.lock().unwrap_or_else(|x| x.into_inner())
                    .iter().find(|g| g.0 == world).map(|g| g.1) };
                // 대상 위치: 게임 cctx가 tag0으로 준 대상key를 우선 사용, 없으면 최근접 적
                let tgt_pos = {
                    let mut p = None;
                    if before.map(|c| c.0) == Some(0) {
                        let tk = before.map(|c| c.1).unwrap_or(u64::MAX);
                        let te = if tk != u64::MAX { resolve(world, tk) } else { 0 };
                        if te >= 0x10000 {
                            p = Some((rd_u64(te + E_POS_X).unwrap_or(0), rd_u64(te + E_POS_Y).unwrap_or(0)));
                        }
                    }
                    if p.is_none() {
                        // ★[v128] 여기도 공여자 casting_target을 따른다(위 fix_entity_cctx와 동형).
                        //   cctx_ent=1이면 이 경로는 안 타지만, 두 경로가 어긋나 있으면 나중에 함정이 된다.
                        if let Some(k) = nearest_by_ct(world, cent, caster_key, eff_ct(cent)) {
                            let pe = resolve(world, k);
                            if pe >= 0x10000 {
                                p = Some((rd_u64(pe + E_POS_X).unwrap_or(0), rd_u64(pe + E_POS_Y).unwrap_or(0)));
                            }
                        }
                    }
                    p
                };
                let (sx, sy) = (rd_u64(cent + E_POS_X).unwrap_or(0), rd_u64(cent + E_POS_Y).unwrap_or(0));
                // gate → (원하는 tag, w8, w10)
                let want: Option<(u64, u64, u64, &str)> = match gate {
                    Some(0) => None,                                   // Targeting: 게임 cctx 그대로
                    Some(1) => {
                        // Position: 좌표. 기본 = 사일러스 자기 위치(교전 한복판), zone_at=enemy면 대상 위치.
                        let (px, py) = if ZONE_AT_ENEMY.load(Ordering::Relaxed) {
                            tgt_pos.unwrap_or((sx, sy))
                        } else { (sx, sy) };
                        Some((2, px, py, if ZONE_AT_ENEMY.load(Ordering::Relaxed) { "좌표←대상" } else { "좌표←self" }))
                    }
                    Some(2) => {
                        // Direction: 방향벡터(대상−시전자). effect가 정규화하므로 크기는 무관.
                        match tgt_pos {
                            Some((tx, ty)) => Some((1, (tx as i64 - sx as i64) as u64,
                                                       (ty as i64 - sy as i64) as u64, "방향←대상-self")),
                            None => None,
                        }
                    }
                    _ => None,
                };
                let gname = match gate { Some(0) => "Targeting", Some(1) => "Position",
                                         Some(2) => "Direction", Some(-1) => "None", _ => "미상" };
                fixlog.push_str(&format!("  [cctx판정] 강탈 궁 gate={:?}({}) ⟹ {}\n", gate, gname,
                    match want { Some((t, _, _, s)) => format!("tag {} 공급({})", t, s),
                                 None => "게임 cctx 유지".to_string() }));
                // ★★[v87] cctx_ent=1이면 **여기서는 손대지 않는다** — 이미 `ent+0x88`에서
                //   상류 교정이 끝났고, 여기서 또 바꾸면 조준 리드가 계산한 값을 덮어써 버린다.
                if CCTX_ENT.load(Ordering::Relaxed) {
                    fixlog.push_str("  [cctx] ent+0x88 상류 교정 사용 — apply 시점 교체 생략(정본)
");
                } else if let (Some((tag, w8, w10, srcs)), true, true) =
                    (want, CCTX_FIX.load(Ordering::Relaxed), cctx0 >= 0x10000) {
                    // ★게임 스택을 건드리지 않는다 — 우리 버퍼를 채우고 **포인터만** 교체.
                    let myptr = CCTX_TMP.with(|c| {
                        let b = c.get();
                        let hdr: *mut u64 = b as *mut u64;
                        *hdr = tag; *hdr.add(1) = w8; *hdr.add(2) = w10; *hdr.add(3) = 0;
                        hdr as usize
                    });
                    fixlog.push_str(&format!("  ★[cctx교정] 우리 버퍼 {:#x} = ({}, {:#x}, {:#x}) {}\n",
                        myptr, tag, w8, w10, srcs));
                    CCTX_MY_PTR.store(myptr as u64, Ordering::Relaxed);
                    CCTX_MY_X.store(w8, Ordering::Relaxed);
                    CCTX_MY_Y.store(w10, Ordering::Relaxed);
                    let ok = wr_u64(e + 0x38, myptr as u64);
                    fixlog.push_str(&format!("  ★[cctx교정] 인자 포인터 {:#x}→{:#x} {}\n",
                        cctx0, myptr, if ok { "OK" } else { "실패" }));
                    } else {
                }
                // ★위험한 조작 **전에** 로그를 내보낸다 — 크래시하면 아무것도 안 남는다(v64 실패).
                hlog(&fixlog);
                // 이 world에서 우리가 건드린 entity를 전부 원복한다(부활로 여러 개일 수 있다).
                let mut restored = 0usize;
                {
                    let mut p = PATCHED.lock().unwrap_or_else(|x| x.into_inner());
                    let mine: Vec<(usize, usize, Vec<u64>)> =
                        p.iter().filter(|q| q.0 == world).cloned().collect();
                    p.retain(|q| q.0 != world);
                    drop(p);
                    for (_, e2, orig) in mine {
                        // ★★[v71] 죽은/재사용된 entity에 옛 스냅샷을 쓰지 않는다.
                        //   챔프는 죽고 부활할 때마다 **새 entity**가 되고, 옛 슬롯은 다른 챔프가 재사용할 수 있다.
                        //   거기에 사일러스의 옛 effect 포인터를 써 넣으면 그 챔프가 시전할 때 죽는다(크래시 후보).
                        //   ⟹ ①아직 사일러스인가 ②슬롯3이 **우리가 넣은 강탈 궁 그대로인가** 를 확인하고서만 되돌린다.
                        let b = e2 + E_SLOT0 + 3 * SLOT_STRIDE;
                        let still_sylas = rd_u64(e2 + E_KIND) == Some(0xd)
                            && ent_name(e2).as_deref() == Some("sylas");
                        let holds_stolen = rd_u64(b + SLOT_DATA).map(|d| d == stolen_data as u64).unwrap_or(false);
                        if !still_sylas || !holds_stolen {
                            hlog(&format!("  [원복skip] ent={:#x} sylas={} 슬롯3=강탈궁={} — 건드리지 않는다\n",
                                e2, still_sylas, holds_stolen));
                            continue;
                        }
                        // ★★즉시 원복하지 않는다 — 같은 시전에서 슬롯을 다시 읽어 자기 궁이 또 나간다.
                        //   ⚠~~게이트(+0x30)를 -1로 눌러 추가 발동을 막는다~~ → **폐기**(2026-08-25 실측):
                        //   게이트가 -1이면 틱 arm이 통째로 스킵되어 시전 상태기계가 완료 신호를 못 받고
                        //   `ent+0x70`이 6에 머문다 ⟹ **시뮬레이션이 멈춘다**(크래시 아님 — 유저 실측).
                        //   ⟹ 슬롯은 손대지 않고 그대로 둔다. 같은 시전에서 두 번 읽히면 강탈 궁이 2회
                        //   발동하지만(유저 스크린샷의 "보라 깃발 2개"), 멈춤보다는 낫다. 중복 제거는 별도 과제.
                        if keep { continue; }   // ★유지형: 원복 예약하지 않는다
                        restored += 1;
                        PENDING.lock().unwrap_or_else(|x| x.into_inner()).push((world, e2, orig));
                    }
                }
                // ★다음 CasterViewEffect 호출(= 이 Combine의 자식)을 우리 이름으로 돌린다.
                if VIEW_FIX.load(Ordering::Relaxed) { STOLEN_VIEW.with(|c| c.set(1)); }
                hlog(&format!("★★[강탈궁 발동] {} 궁이 사일러스에게서 실행됨(world {:#x}) → 발화 확정 {}건(원복은 시전 종료 후)\n{}{}",
                    who, world, restored,
                    dump_children(selfp, "발동한 궁", "sylas"),
                    dump_slots(cent, "sylas(원복후)")));
            } else if let Some((who, ok)) = ensure_loaded(world, cent) {
                hlog(&format!("★★[장전] 사일러스 ent={:#x}(world {:#x}) 슬롯3 ← {} 궁 {}\n{}",
                    cent, world, who, if ok { "교체 OK" } else { "교체 실패" },
                    dump_slots(cent, "sylas(장전후)")));
            }
        }
        // ★★[v51] 사일러스 궁 시점 버프 리스트 + 자기 AddCasterBuff 페이로드(= 정상 표본)
        if is_ult && BUFF_PROBE.load(Ordering::Relaxed) {
            let mut t = String::from("★[버프덤프] 사일러스 궁 시점\n");
            t.push_str(&dump_buff_list(cent, "sylas"));
            t.push_str(&dump_stats(cent));
            for (d, v) in flat.iter() {
                if rd_u64(*v + EFF_APPLY).map(|a| rva_of(a) as usize) == Some(ADDBUFF_RVA) {
                    t.push_str(&hex_payload(eff_self(*d, *v), "sylas버프"));
                }
            }
            let held = { BUFF_CAP.lock().unwrap_or_else(|x| x.into_inner()).clone() };
            match held {
                Some((who, b, cw)) => t.push_str(&format!("  [보유] {} 버프 {:?} {}B (캡처world {:#x} / 현재 {:#x} {})\n",
                    who, buff_name(&b), b.len(), cw, world, if cw == world { "일치" } else { "불일치" })),
                None => t.push_str("  [보유] 없음(바닐라 궁의 AddCasterBuff를 아직 못 봤다)\n"),
            }
            hlog(&t);
        }
        // ★★[v51] 버프 이식 — 보유한 X 버프를 사일러스 버프 리스트에 push(값 복사).
        if is_ult && BUFF_GRAFT.load(Ordering::Relaxed) && ARMED.load(Ordering::Relaxed) {
            let held = { BUFF_CAP.lock().unwrap_or_else(|x| x.into_inner()).clone() };
            match held {
                // ★buff_call=1 = 게임 함수 직접 호출(재합산·HP조정까지 게임이 한다)
                Some((who, b, cw)) if cw == world && BUFF_CALL.load(Ordering::Relaxed) => {
                    let before = rd_u64(cent + E_BUF_LEN).unwrap_or(0);
                    let st0 = dump_stats(cent);
                    let buf = b.clone();                 // 게임이 여기서 0x120B를 읽어간다
                    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        sy_call4(exe_base() + BUFFPUSH_RVA, cent, buf.as_ptr() as usize, 0,
                                 core::ptr::null_mut())
                    }));
                    let after = rd_u64(cent + E_BUF_LEN).unwrap_or(0);
                    hlog(&format!("★[버프호출] {} 버프 {:?} → 게임 push 호출 {} | len {}→{}\n{}{}{}",
                        who, buff_name(&b), if r.is_ok() { "복귀" } else { "패닉" }, before, after,
                        dump_buff_list(cent, "sylas(호출직후)"),
                        st0.replace("[스탯캐시]", "[스탯前]"),
                        dump_stats(cent).replace("[스탯캐시]", "[스탯後]")));
                    drop(buf);
                }
                Some((who, b, cw)) if cw == world => match push_buff(cent, &b) {
                    Ok(n) => hlog(&format!("★[버프이식] {} 버프 {:?} → 사일러스 push OK (len→{})\n{}{}",
                        who, buff_name(&b), n,
                        dump_buff_list(cent, "sylas(push직후)"), dump_stats(cent))),
                    Err(er) => hlog(&format!("  [버프이식] 실패: {}\n", er)),
                },
                Some((_, _, cw)) => hlog(&format!("  [버프이식] world 불일치 {:#x}≠{:#x}\n", cw, world)),
                None => hlog("  [버프이식] 보유 버프 없음\n"),
            }
        }
        // ★entity 대기 큐 덤프 — 시전이 여기 쌓이는지 확인
        if is_ult && QUEUE_PROBE.load(Ordering::Relaxed) {
            let cctx0 = rd_u64(e + 0x38).unwrap_or(0) as usize;
            let tk = if cctx0 >= 0x10000 { rd_u64(cctx0 + 8).unwrap_or(u64::MAX) } else { u64::MAX };
            let te = if tk != u64::MAX { resolve(world, tk) } else { 0 };
            let mut q = format!("★[큐덤프] 사일러스 궁 시점\n");
            q.push_str(&dump_queue(cent, "sylas"));
            if te != 0 { q.push_str(&dump_queue(te, "대상")); }
            hlog(&q);
        }
        // ★콜스택 전체 덤프 — 시전 파이프라인을 위에서부터 보기 위한 1차 정보
        if is_ult && STACK_TRACE.load(Ordering::Relaxed) {
            let frames = walk_stack(e, 14);
            let mut t = format!("★[콜스택] 사일러스 궁 (self={:#x}, 자식 {}개)\n", selfp, kids.len());
            for (i, f) in frames.iter().enumerate() {
                t.push_str(&format!("    #{:<2} RVA:{:#x}\n", i, f));
            }
            hlog(&t);
        }
        // ★★D안 1단계: 캡처한 X action으로 effect 생성기를 호출해 **무엇이 나오는지** 관찰한다.
        if is_ult && BUILD_EFFECT.load(Ordering::Relaxed) && ARMED.load(Ordering::Relaxed) {
            let snap = { VACTION.lock().unwrap_or_else(|x| x.into_inner()).clone() };
            match snap {
                Some((who, action, f188, cw)) if cw == world => {
                    // ★생성기 입력 = action + 0x48 (원본). base apply가 하는 것과 동일한 인자.
                    let gate_ptr = action + A_ULT_GATE;
                    let req = UltReq { a00: 0, a08: 0, a10: f188, a18: 0, a20: 0, a28: 0, a30: 0, _pad: 0 };
                    let mut rdx: usize = 0;
                    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        sy_call4(exe_base() + CLONE_RVA, gate_ptr, 1, req.a10 as usize, &mut rdx)
                    }));
                    match r {
                        Ok(rax) => {
                            let vt = rd_u64(rax + 8).unwrap_or(0);
                            hlog(&format!("★[생성기] src={} in={:#x} → rax={:#x} rdx={:#x} | [rax]={:#x} [rax+8]={:#x} apply?=RVA:{:#x}\n",
                                who, gate_ptr, rax, rdx,
                                rd_u64(rax).unwrap_or(0), vt,
                                rva_of(rd_u64(vt as usize + EFF_APPLY).unwrap_or(0))));
                        }
                        Err(_) => hlog("★[생성기] PANIC\n"),
                    }
                }
                Some((_, _, _, cw)) => hlog(&format!("  [생성기] world 불일치 {:#x}≠{:#x}\n", cw, world)),
                _ => hlog("  [생성기] 캡처된 action 없음\n"),
            }
            return;
        }
        // ★음성 대조군: len만 0으로 → 사일러스 궁이 아무 것도 안 하는지 화면으로 확인
        if is_ult && NULLIFY.load(Ordering::Relaxed) && ARMED.load(Ordering::Relaxed) {
            let before = rd_u64(selfp + 0x10).unwrap_or(0);
            let ok = wr_u64(selfp + 0x10, 0);
            let after = rd_u64(selfp + 0x10).unwrap_or(u64::MAX);
            hlog(&format!("★[nullify] 사일러스 궁 len {}→{} (쓰기 {}) — 화면에서 궁이 안 나오면 graft 유효\n",
                before, after, if ok { "OK" } else { "실패" }));
            return;
        }
        // ★★graft: 자식 배열 한 칸을 남의 effect로 교체하고 게임이 실행하게 한다
        if is_ult && GRAFT.load(Ordering::Relaxed) && ARMED.load(Ordering::Relaxed) {
            let snap = { CAPTURED.lock().unwrap_or_else(|x| x.into_inner()).clone() };
            match snap {
                Some((who, ks, cw, _)) if !ks.is_empty() && cw == world => {
                    // ★★사일러스 궁을 **통째로 남의 궁으로 대체**한다(유저 지시 2026-08-24:
                    //   "원래 본인 궁은 시전 안되게"). 자식 배열 앞에서부터 남의 자식으로 전부 덮고,
                    //   Combine의 len(`self+0x10`)을 남의 자식 수로 **줄인다** ⟹ 사일러스 자기 effect는
                    //   하나도 실행되지 않는다. (len을 늘리는 것은 배열 용량 초과라 금지 — 줄이는 것만 안전.)
                    let n = ks.len().min(kids.len());
                    let mut done = 0;
                    let mut s_log = format!("★[graft·전체대체] src={} 남의자식 {}개 / 사일러스 궁 {}칸 → {}칸 사용\n",
                        who, ks.len(), kids.len(), n);
                    for j in 0..n {
                        let it = kptr + j * EFF_STRIDE;
                        let (nd, nv) = ks[j];
                        let ov = rd_u64(it + 8).unwrap_or(0);
                        if readable(it, 16) && wr_u64(it, nd as u64) && wr_u64(it + 8, nv as u64) {
                            done += 1;
                            s_log.push_str(&format!("    [{}] vt {:#x}→{:#x} apply→RVA:{:#x}\n",
                                j, rva_of(ov), rva_of(nv as u64),
                                rva_of(rd_u64(nv + EFF_APPLY).unwrap_or(0))));
                        }
                    }
                    // ★len 축소 = 사일러스 자기 궁 effect 차단
                    let len_ok = if done > 0 && done < kids.len() {
                        wr_u64(selfp + 0x10, done as u64)
                    } else { true };
                    s_log.push_str(&format!("    → {}/{} 이식, len {}→{} {}\n",
                        done, n, kids.len(), done, if len_ok { "OK" } else { "(len 쓰기 실패)" }));
                    hlog(&s_log);
                    if done > 0 {
                        let cctx0 = rd_u64(e + 0x38).unwrap_or(0) as usize;
                        let tk = if cctx0 >= 0x10000 { rd_u64(cctx0 + 8).unwrap_or(u64::MAX) } else { u64::MAX };
                        let te = if tk != u64::MAX { resolve(world, tk) } else { 0 };
                        let ce = resolve(world, caster_key);
                        if te != 0 || ce != 0 {
                            let mut g = GRAFT_PEND.lock().unwrap_or_else(|x| x.into_inner());
                            *g = Some((te, ce,
                                rd_u64(te + E_CUR_HP).unwrap_or(0), rd_u64(ce + E_CUR_HP).unwrap_or(0),
                                rd_u64(te + E_EFF_LEN).unwrap_or(0), rd_u64(ce + E_EFF_LEN).unwrap_or(0),
                                done as u64));
                        }
                    }
                }
                Some((_, _, cw, _)) => hlog(&format!("  [graft] world 불일치 {:#x}≠{:#x}\n", cw, world)),
                _ => hlog("  [graft] 캡처된 재료 없음\n"),
            }
            return; // 원본 루프가 이어서 실행한다
        }
        // ★바닐라 궁 재생: cfg `replay_vanilla=1`
        if is_ult && REPLAY_VANILLA.load(Ordering::Relaxed) && ARMED.load(Ordering::Relaxed) {
            if IN_REPLAY.with(|f| f.get()) { return; }
            IN_REPLAY.with(|f| f.set(true));
            replay_vanilla_jt(saved, e);
            IN_REPLAY.with(|f| f.set(false));
            return;
        }
        // ★self 재생(진단용): 사일러스 궁 자식 중 Grab을 뺀 나머지를 한 번 더 실행
        if is_ult && REPLAY_SELF.load(Ordering::Relaxed) && ARMED.load(Ordering::Relaxed) {
            if IN_REPLAY.with(|f| f.get()) { hlog("  [self재생] 재진입 차단\n"); return; }
            let dup: Vec<(usize, usize)> = kids.iter().cloned()
                .filter(|(_, v)| rd_u64(v + EFF_APPLY).map(|a| rva_of(a) as usize) != Some(GRAB_RVA))
                .collect();
            IN_REPLAY.with(|f| f.set(true));
            replay_children("SELF(사일러스 궁 Grab제외)", &dup, saved, e);
            IN_REPLAY.with(|f| f.set(false));
            return;
        }
        if is_ult && REPLAY.load(Ordering::Relaxed) && ARMED.load(Ordering::Relaxed) {
            let already = IN_REPLAY.with(|f| f.get());
            if already { hlog("  [재생] 재진입 차단\n"); return; }
            let snap = { CAPTURED.lock().unwrap_or_else(|x| x.into_inner()).clone() };
            match snap {
                Some((who, ks, cw, was_ult)) if !ks.is_empty() => {
                    if cw != world {
                        hlog(&format!("  [재생] world 불일치 skip: 캡처={:#x} 현재={:#x}\n", cw, world));
                    } else {
                        IN_REPLAY.with(|f| f.set(true));
                        let label = if was_ult { format!("{}(궁)", who) } else { format!("{}(비궁)", who) };
                        replay_children(&label, &ks, saved, e);
                        IN_REPLAY.with(|f| f.set(false));
                        REPLAY_N.fetch_add(1, Ordering::Relaxed);
                    }
                }
                _ => hlog("  [재생] 캡처된 Combine 없음(바닐라 챔프가 스킬을 아직 안 씀)\n"),
            }
        }
        return;
    }
    // ★바닐라 챔프: 재생 재료 보관. 자식 수 2~8만(archer의 16짜리 같은 이상치 배제 — v15 크래시 관여).
    //   ★★보관 전 각 자식의 Arc strong count를 올려 수명을 연장한다(안 하면 재생 시점에 dangling).
    // src 지정이 있으면 그 챔프만, 없으면 전체에서 자식 수 최대를 고른다.
    let want = { SRC_NAME.lock().unwrap_or_else(|x| x.into_inner()).clone() };
    let src_ok = want.is_empty() || want == cname;
    let min_k = MIN_KIDS.load(Ordering::Relaxed) as usize;
    // ★이 caster가 방금 궁을 썼는가 = ult 쿨이 크게 뛰었는가
    let cd_now = rd_u64(cent + E_ULT_CD).unwrap_or(0);
    let is_ult_cast = {
        let mut g = ULT_CD_SEEN.lock().unwrap_or_else(|x| x.into_inner());
        match g.iter_mut().find(|(n, _)| n == &cname) {
            Some((_, prev)) => {
                // ★점프 폭 + 절대값을 함께 본다(일반 스킬 쿨 오탐 방지)
                let jumped = cd_now > prev.saturating_add(ULT_CD_JUMP.load(Ordering::Relaxed))
                          && cd_now >= ULT_CD_MIN.load(Ordering::Relaxed);
                *prev = cd_now;
                jumped
            }
            None => { if g.len() < 64 { g.push((cname.clone(), cd_now)); } false }
        }
    };
    // ★바닐라 궁 시전 시 그 챔프의 큐도 덤프 — 사일러스 것과 구조를 비교한다
    if is_ult_cast && QUEUE_PROBE.load(Ordering::Relaxed) {
        hlog(&dump_queue(cent, &format!("{}(궁시전)", cname)));
    }
    // ★★[v63] X가 궁을 쏘는 프레임에 X의 궁 슬롯(3)을 self 매칭으로 확정해 **보유**한다.
    //   실제 장전은 `ensure_loaded`가 사일러스를 볼 때마다 수행한다(부활로 entity가 갈려도 유지).
    // ~~구 경로: 적이 궁 쓰는 것을 **목격**하면 자동 강탈~~ → 원작형(Grab 기반)으로 대체.
    //   `watch_steal=1` 로만 켜진다(비교·디버그용, 기본 OFF).
    if from_ult_slot && WATCH_STEAL.load(Ordering::Relaxed)
        && SLOT_SWAP.load(Ordering::Relaxed) && ARMED.load(Ordering::Relaxed) {
        let want = { SRC_NAME.lock().unwrap_or_else(|x| x.into_inner()).clone() };
        if !(want.is_empty() || want == cname) {
            if SLOT_N.fetch_add(1, Ordering::Relaxed) < 24 {
                hlog(&format!("  [강탈skip] {} 궁(슬롯3) — src={:?} 지정과 불일치\n", cname, want));
            }
        } else {
            let busy = { HELD.lock().unwrap_or_else(|x| x.into_inner()).iter().any(|h| h.0 == world) };
            if busy {
                if SLOT_N.fetch_add(1, Ordering::Relaxed) < 24 {
                    hlog(&format!("  [강탈skip] {} — world {:#x}에 이미 보유 중\n", cname, world));
                }
            } else if let Some(n) = cast_slot {
                let b = cent + E_SLOT0 + n * SLOT_STRIDE;
                let d = rd_u64(b + SLOT_DATA).unwrap_or(0) as usize;
                let v = rd_u64(b + SLOT_VT).unwrap_or(0) as usize;
                if effect_sane(d, v) {
                    // ★Arc strong++ 로 수명을 붙든다(원복 시 dec 하지 않는다 = 참조 1개 누수.
                    //   해제돼 dangling 되는 것보다 안전하고, 경기 단위 수명이라 실사용에 문제 없다).
                    let inc = arc_incref(d);
                    {
                        let mut g = HELD.lock().unwrap_or_else(|x| x.into_inner());
                        if g.len() >= HOLD_MAX { g.remove(0); }
                        g.push((world, cname.clone(), d, v));
                    }
                    // ★X가 자기 궁을 쏠 때의 cctx = **정상 표본**. 사일러스 시전 시와 대조한다.
                    let cx = read_cctx(rd_u64(e + 0x38).unwrap_or(0) as usize);
                    if let Some(c) = cx { *SRC_CCTX.lock().unwrap_or_else(|x| x.into_inner()) = Some(c); }
                    hlog(&format!("★★[강탈] {} 궁 슬롯{} 보유(world {:#x}) | data={:#x} vt=RVA:{:#x} arc_inc={}\n  [cctx·{}원본] {:?} | 시전자pos=({:#x},{:#x})\n{}",
                        cname, n, world, d, rva_of(v as u64), inc, cname, cx,
                        rd_u64(cent + E_POS_X).unwrap_or(0), rd_u64(cent + E_POS_Y).unwrap_or(0),
                        dump_children(eff_self(d, v), "강탈한 궁", &cname)));
                }
            }
        }
    }
    // ★★[v56] 바닐라 챔프가 궁을 쏠 때 그 챔프의 슬롯 4칸 — 강탈 재료가 어디 있는지 확정한다
    if from_ult_slot && SLOT_PROBE.load(Ordering::Relaxed) {
        let sw4 = SYLAS_WORLD.load(Ordering::Relaxed);
        if sw4 != 0 && sw4 == world as u64 && SLOT_N.fetch_add(1, Ordering::Relaxed) < 12 {
            hlog(&format!("★[슬롯덤프] {} 궁 시전 시점\n{}", cname, dump_slots(cent, &cname)));
        }
    }
    // ★★[v51] X 궁의 AddCasterBuff 페이로드를 **값으로** 캡처한다(포인터를 들지 않는다).
    if is_ult_cast && BUFF_PROBE.load(Ordering::Relaxed) {
        let sw3 = SYLAS_WORLD.load(Ordering::Relaxed);
        // ★cfg `src=<champ>` 지정 시 그 챔프의 버프만 캡처한다(통제된 실험 — 마지막 궁에 덮이지 않게).
        let src_want = { SRC_NAME.lock().unwrap_or_else(|x| x.into_inner()).clone() };
        if sw3 != 0 && sw3 == world as u64 && (src_want.is_empty() || src_want == cname) {
            for (d, v) in flat.iter() {
                if rd_u64(*v + EFF_APPLY).map(|a| rva_of(a) as usize) != Some(ADDBUFF_RVA) { continue; }
                let sp = eff_self(*d, *v);
                if let Some(bytes) = read_bytes(sp, BUF_STRIDE) {
                    let bn = buff_name(&bytes);
                    if BUFF_LOGGED.fetch_add(1, Ordering::Relaxed) < 4 {
                        hlog(&format!("★[버프캡처] {} 궁의 AddCasterBuff = {:?}\n{}{}", cname, bn,
                            hex_payload(sp, &format!("{}버프", cname)), dump_buff_list(cent, &cname)));
                    } else {
                        hlog(&format!("★[버프캡처] {} = {:?}\n", cname, bn));
                    }
                    *BUFF_CAP.lock().unwrap_or_else(|x| x.into_inner()) = Some((cname.clone(), bytes, world));
                }
            }
        }
    }
    // ★★steal_cast: 바닐라 챔프가 궁을 쏘는 그 프레임에 시전자를 사일러스로 바꾼다.
    if is_ult_cast && STEAL_CAST.load(Ordering::Relaxed) && ARMED.load(Ordering::Relaxed) {
        let sk = SYLAS_KEY.load(Ordering::Relaxed);
        let sw2 = SYLAS_WORLD.load(Ordering::Relaxed);
        if sk != u64::MAX && sw2 == world as u64 {
            // ★시전자는 **[rsp+0x30]**(= p6, caster_key)다. casting_ctx+8은 **대상**이다
            //   (v13 실측: A[rsp+0x30]=시전자로 수렴, B[cctx+8]=대상으로 분산).
            //   ⚠첫 시도에서 이 둘을 반대로 잡아 "사일러스를 궁 맞는 쪽"으로 만들었다 — 2026-08-24 정정.
            let old_key = caster_key;
            if old_key != sk {
                let se = resolve(world, sk);
                let xe = resolve(world, old_key);
                // ★사일러스 궁 쿨이 남아 있으면 강탈하지 않는다(= 궁을 쓸 수 있을 때만 강탈)
                let cost = ULT_COST.load(Ordering::Relaxed);
                let s_cd = if se != 0 { rd_u64(se + E_ULT_CD).unwrap_or(0) } else { 0 };
                if cost && se != 0 && s_cd > 0 {
                    if STEAL_N.load(Ordering::Relaxed) < 8 {
                        hlog(&format!("  [강탈보류] 사일러스 궁 쿨 {} 남음 ({} 궁 통과)\n", s_cd, cname));
                    }
                    return;
                }
                if wr_u64(e + 0x30, sk) {
                    // ★대상도 X 자신으로 — 안 바꾸면 X가 고른 대상(사일러스 기준 아군일 수 있음)을 때린다
                    // ★대상 = 사일러스 기준 최근접 적. 실패하면 원래 시전자(X)로 폴백.
                    let mut rt = String::from("유지");
                    if RETARGET.load(Ordering::Relaxed) {
                        let cctx1 = rd_u64(e + 0x38).unwrap_or(0) as usize;
                        if cctx1 >= 0x10000 && se != 0 {
                            let pick = nearest_enemy(world, se, sk).unwrap_or(old_key);
                            let pe = resolve(world, pick);
                            if wr_u64(cctx1 + 8, pick) {
                                rt = format!("→{}({})", pick as i64,
                                    if pe != 0 { ent_name(pe).unwrap_or_default() } else { String::new() });
                            }
                        }
                    }
                    // ★강탈 성공 = 사일러스도 궁을 쓴 것으로 처리 → 쿨 소모
                    if cost && se != 0 {
                        let _ = wr_u64(se + E_ULT_CD, cd_now);
                        hlog(&format!("  [궁소모] 사일러스 ult 쿨 {} 설정({} 궁 쿨과 동일)\n", cd_now, cname));
                    }
                    STEAL_N.fetch_add(1, Ordering::Relaxed);
                    hlog(&format!("★[시전자강탈] {} 궁 | 시전자 {}→사일러스{} | 대상 {} | 자식 {}개\n",
                        cname, old_key as i64, sk as i64, rt, flat.len()));
                    if se != 0 || xe != 0 {
                        let mut g = GRAFT_PEND.lock().unwrap_or_else(|x| x.into_inner());
                        *g = Some((xe, se,
                            rd_u64(xe + E_CUR_HP).unwrap_or(0), rd_u64(se + E_CUR_HP).unwrap_or(0),
                            rd_u64(xe + E_EFF_LEN).unwrap_or(0), rd_u64(se + E_EFF_LEN).unwrap_or(0),
                            flat.len() as u64));
                    }
                }
            }
        }
    }
    // ★★사일러스와 같은 world의 재료만 캡처한다. 아직 사일러스를 못 봤으면(=0) 보류.
    let sw = SYLAS_WORLD.load(Ordering::Relaxed);
    let world_ok = sw != 0 && sw == world as u64;
    let ult_ok = !ONLY_ULT.load(Ordering::Relaxed) || is_ult_cast;
    if world_ok && src_ok && ult_ok && flat.len() >= min_k.max(1) && flat.len() <= 16 {
        let mut c = CAPTURED.lock().unwrap_or_else(|x| x.into_inner());
        // 지정 모드에선 "가장 최근 것"으로 계속 갱신(그 챔프의 여러 스킬 중 마지막 것),
        // 자동 모드에선 자식 수 최대를 유지.
        // ★궁이 잡히면 그것을 최우선으로 보관하고, 이후 비-궁으로 덮어쓰지 않는다.
        let better = match c.as_ref() {
            None => true,
            Some((_, old, _, was_ult)) => {
                if is_ult_cast && *was_ult { flat.len() > old.len() }  // 궁끼리는 더 풍부한 쪽
                else if is_ult_cast { true }         // 비궁 → 궁이면 교체
                else if *was_ult { false }           // 이미 궁을 들고 있으면 비-궁으로 덮지 않음
                else if !want.is_empty() { true }
                else { kids.len() > old.len() }
            }
        };
        if better {
            let mut held: Vec<(usize, usize)> = Vec::new();
            let mut fail = 0;
            let use_clone = CLONE_GRAFT.load(Ordering::Relaxed);
            for (d, v) in flat.iter() {
                if use_clone {
                    // ★fresh 복제본을 보관(원본 포인터를 들지 않는다)
                    match clone_effect(*d, *v) { Some(nd) => held.push((nd, *v)), None => fail += 1 }
                } else if arc_incref(*d) { held.push((*d, *v)); } else { fail += 1; }
            }
            hlog(&format!("[캡처]{} src={} 직계={} 펼침={} 보유={} 실패={} ult_cd={:#x} world={:#x}\n",
                if is_ult_cast { "★궁" } else { "" }, cname, kids.len(), flat.len(), held.len(), fail, cd_now, world));
            // ★src 지정 모드에선 자식 구성을 함께 남긴다 — 어느 Combine이 그 챔프의 궁인지 로그로 판별하기 위함
            if !want.is_empty() { hlog(&dump_children(selfp, "캡처-자식구성", &cname)); }
            // ★궁으로 판정된 것은 자식 구성을 남긴다 — 알려진 effect apply와 대조해 판별 정확도를 검증한다
            //   (참고 실측: ApAttack=0x16fcd80 · Stun=0x150ff20 · BlockSkill=0x16ec1e0 ·
            //    AddCasterBuff=0x15afd30 · CasterViewEffect=0x1270980 · Grab=0x1801780 · Combine=0x1802630)
            if is_ult_cast { hlog(&dump_children(selfp, "궁-자식구성", &cname)); }
            if !held.is_empty() { *c = Some((cname.clone(), held, world, is_ult_cast)); }
        }
    }
    // 카탈로그(챔프당 1회)
    let mut g = SEEN.lock().unwrap_or_else(|x| x.into_inner());
    if g.iter().any(|x| x == &cname) { return; }
    if g.len() >= 24 { return; }
    g.push(cname.clone());
    drop(g);
    hlog(&dump_children(selfp, "VANILLA-Combine", &cname));
}

// ── ★바닐라 궁 캡처 (JT 디스패처 훅)
//    저장물 = req 구조체 사본(0x48B) + 그 안 action Arc의 refcount 연장.
//    재생 = 사일러스 궁 문맥의 (world, WorldOps, target, cctx)로 JT 디스패처를 다시 호출.
static VANILLA_ULT: Mutex<Option<(String, [u8; REQ_COPY], usize)>> = Mutex::new(None);
static JT_DIAG: AtomicU32 = AtomicU32::new(0);
static JT_SEEN: Mutex<Vec<(String, u32)>> = Mutex::new(Vec::new());
type Jt5 = unsafe extern "C" fn(usize, usize, usize, usize, usize) -> u64;

/// base 궁 apply가 스택에 조립해 JT 디스패처로 넘기는 실행요청.
/// ★디스어셈 실측 배치(rbp-0x58 기준):
///   `+0x00=clone rax / +0x08=clone rdx / +0x10,+0x18=[action+0x188] 16B /
///    +0x20=[action+0x180] / +0x28=pshufd([action+0x19c],0xe1) / +0x30=[action+0x198](JT 인덱스)`
#[repr(C)]
#[derive(Clone, Copy)]
struct UltReq { a00: usize, a08: usize, a10: u64, a18: u64, a20: u64, a28: u64, a30: u32, _pad: u32 }

/// 캡처한 바닐라 궁 실행요청 (챔프, req, world)
static VULT: Mutex<Option<(String, UltReq, usize)>> = Mutex::new(None);
type Jt5b = unsafe extern "C" fn(usize, usize, usize, usize, usize) -> u64;

/// ★캡처해 둔 바닐라 궁 req를 **사일러스 문맥**으로 JT 디스패처에 다시 태운다.
///   effect가 아니라 **action 레벨**이라, JT가 내부에서 effect를 새로 만들어 준다(= fresh 실행).
unsafe fn replay_vanilla_jt(saved: usize, e: usize) {
    let snap = { VULT.lock().unwrap_or_else(|x| x.into_inner()).clone() };
    let Some((who, req, cw)) = snap else { hlog("  [궁재생] 캡처된 바닐라 궁 없음\n"); return };
    let world = *((saved + 0x18) as *const u64) as usize; // r8 (Combine apply의 world)
    let wops  = *((saved + 0x10) as *const u64) as usize; // r9
    if cw != world {
        hlog(&format!("  [궁재생] world 불일치 skip: 캡처={:#x} 현재={:#x}\n", cw, world));
        return;
    }
    let sc = rd_u64(req.a00).unwrap_or(0);
    if sc == 0 || sc > 0x100000 { hlog(&format!("  [궁재생] action strong={:#x} 이상 중단\n", sc)); return; }
    let (Some(p6), Some(p7)) = (rd_u64(e + 0x30), rd_u64(e + 0x38)) else { return };
    let tgt_key = rd_u64(p7 as usize + 8).unwrap_or(u64::MAX);
    let tgt_ent = if tgt_key != u64::MAX { resolve(world, tgt_key) } else { 0 };
    let cas_ent = resolve(world, p6);
    let (h0t, h0c) = (if tgt_ent != 0 { rd_u64(tgt_ent + E_CUR_HP) } else { None },
                      if cas_ent != 0 { rd_u64(cas_ent + E_CUR_HP) } else { None });
    let (e0t, e0c) = (if tgt_ent != 0 { rd_u64(tgt_ent + E_EFF_LEN) } else { None },
                      if cas_ent != 0 { rd_u64(cas_ent + E_EFF_LEN) } else { None });
    hlog(&format!("★[궁재생·JT] src={} action={:#x} strong={:#x} ctype={} 대상={:?}(hp={:?}) 시전자hp={:?}\n",
        who, req.a00, sc, req.a30, if tgt_ent != 0 { ent_name(tgt_ent) } else { None }, h0t, h0c));
    let mut local = req;   // 스택 사본 — JT는 [req+0x30] 등을 읽는다
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let f: Jt5b = core::mem::transmute(exe_base() + JT_RVA);
        f(&mut local as *mut UltReq as usize, world, wops, p6 as usize, p7 as usize)
    }));
    let (h1t, h1c) = (if tgt_ent != 0 { rd_u64(tgt_ent + E_CUR_HP) } else { None },
                      if cas_ent != 0 { rd_u64(cas_ent + E_CUR_HP) } else { None });
    let (e1t, e1c) = (if tgt_ent != 0 { rd_u64(tgt_ent + E_EFF_LEN) } else { None },
                      if cas_ent != 0 { rd_u64(cas_ent + E_EFF_LEN) } else { None });
    let dd = |x: Option<u64>, y: Option<u64>| match (x, y) { (Some(a), Some(b)) => (b as i64) - (a as i64), _ => 0 };
    let (dh, dc, de, dec) = (dd(h0t, h1t), dd(h0c, h1c), dd(e0t, e1t), dd(e0c, e1c));
    let v = if dh != 0 || dc != 0 || de != 0 || dec != 0 { "★효과있음" } else { "무효과" };
    hlog(&format!("   → ret={:?} | {} | 대상HPΔ{} 시전자HPΔ{} 대상effΔ{} 시전자effΔ{}\n",
        r.map(|x| x & 0xffff_ffff).ok(), v, dh, dc, de, dec));
}

/// ★action **원본** 보관 (챔프, action, [action+0x188], world)
///   effect 생성기 `0x16d8b30`의 입력은 **action+0x48**이다. JT 훅에서 얻는 req[0]은 이미 생성된
///   결과물이라 생성기에 넣으면 죽는다(2026-08-24 실측) ⟹ 원본은 base apply 훅에서만 얻을 수 있다.
static VACTION: Mutex<Option<(String, usize, u64, usize)>> = Mutex::new(None);

unsafe fn on_baseult2(saved: usize, e: usize) {
    // ★world 필터를 **최상단**에 — 이 훅은 병렬 sim에서 3연속 게임을 죽였다.
    //   당시엔 필터가 없어 모든 world의 모든 action을 처리했다. JT 훅은 이 처방으로 안정화됐다.
    let world = *((saved + 0x20) as *const u64) as usize; // rdx
    let sw = SYLAS_WORLD.load(Ordering::Relaxed);
    if sw == 0 || sw != world as u64 { return; }
    if !ONLY_ULT.load(Ordering::Relaxed) { return; }      // 궁만
    let action = *((saved + 0x28) as *const u64) as usize; // rcx
    if action < 0x10000 { return; }
    // 게이트 유효성(하위 32비트 0 = 빈 action)
    let gate = match rd_u64(action + A_ULT_GATE) { Some(v) => v, None => return };
    if gate == u64::MAX || (gate & 0xffff_ffff) == 0 { return; }
    let Some(cctx) = rd_u64(e + 0x28) else { return };
    if (cctx as usize) < 0x10000 || rd_u64(cctx as usize).unwrap_or(1) as u32 != 0 { return; }
    let Some(ckey) = rd_u64(cctx as usize + 8) else { return };
    let cent = resolve(world, ckey);
    if cent == 0 { return; }
    let Some(cname) = ent_name(cent) else { return };
    if cname == "sylas" { return; }
    let want = { SRC_NAME.lock().unwrap_or_else(|x| x.into_inner()).clone() };
    if !want.is_empty() && want != cname { return; }
    let f188 = rd_u64(action + A_F188).unwrap_or(0);
    if !arc_incref(action) { return; }
    let mut g = VACTION.lock().unwrap_or_else(|x| x.into_inner());
    let first = g.is_none();
    *g = Some((cname.clone(), action, f188, world));
    drop(g);
    if first {
        hlog(&format!("★[action원본] {} action={:#x} gate={:#x} f188={:#x} world={:#x}\n",
            cname, action, gate, f188, world));
    }
}
unsafe extern "C" fn cap_baseult2(saved: usize, e: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| on_baseult2(saved, e)));
}

unsafe fn on_jt(saved: usize, e: usize) {
    // ★JT 디스패처가 받는 req[0]은 **base apply가 이미 clone한 fresh action**이다.
    //   effect trait엔 clone이 없지만(실측) action은 게임이 매 시전마다 clone해 준다 ⟹ 여기서 받아 챙긴다.
    let req   = *((saved + 0x28) as *const u64) as usize; // rcx
    let world = *((saved + 0x20) as *const u64) as usize; // rdx
    let wops  = *((saved + 0x18) as *const u64) as usize; // r8
    // world 필터를 최상단에 — JT는 hot path다(모든 action이 통과). 사일러스 world 밖이면 즉시 반환.
    let sw = SYLAS_WORLD.load(Ordering::Relaxed);
    if sw == 0 || sw != world as u64 { return; }
    if req < 0x10000 || !in_exe(wops as u64) { return; }
    let Some(cctx) = rd_u64(e + 0x28) else { return };
    if (cctx as usize) < 0x10000 || rd_u64(cctx as usize).unwrap_or(1) as u32 != 0 { return; }
    let Some(ckey) = rd_u64(cctx as usize + 8) else { return };
    let cent = resolve(world, ckey);
    if cent == 0 { return; }
    let Some(cname) = ent_name(cent) else { return };
    if cname == "sylas" { return; }

    // 궁 판별 = caster의 ult 쿨(entity+0xC8) 점프
    let cd_now = rd_u64(cent + E_ULT_CD).unwrap_or(0);
    let is_ult = {
        let mut g = ULT_CD_SEEN.lock().unwrap_or_else(|x| x.into_inner());
        match g.iter_mut().find(|(n, _)| n == &cname) {
            Some((_, prev)) => {
                let j = cd_now > prev.saturating_add(ULT_CD_JUMP.load(Ordering::Relaxed))
                     && cd_now >= ULT_CD_MIN.load(Ordering::Relaxed);
                *prev = cd_now; j
            }
            None => { if g.len() < 64 { g.push((cname.clone(), cd_now)); } false }
        }
    };
    if ONLY_ULT.load(Ordering::Relaxed) && !is_ult { return; }
    let want = { SRC_NAME.lock().unwrap_or_else(|x| x.into_inner()).clone() };
    if !want.is_empty() && want != cname { return; }

    // req를 그대로 복사(레이아웃은 base apply 디스어셈 실측 = UltReq)
    let a00 = rd_u64(req).unwrap_or(0) as usize;
    if a00 < 0x10000 || !arc_incref(a00) { return; }
    let copy = UltReq {
        a00,
        a08: rd_u64(req + 0x08).unwrap_or(0) as usize,
        a10: rd_u64(req + 0x10).unwrap_or(0),
        a18: rd_u64(req + 0x18).unwrap_or(0),
        a20: rd_u64(req + 0x20).unwrap_or(0),
        a28: rd_u64(req + 0x28).unwrap_or(0),
        a30: rd_u64(req + 0x30).unwrap_or(0) as u32,
        _pad: 0,
    };
    let mut g = VULT.lock().unwrap_or_else(|x| x.into_inner());
    let first = g.is_none();
    *g = Some((cname.clone(), copy, world));
    drop(g);
    if first || is_ult {
        hlog(&format!("★[JT캡처]{} {} action={:#x} strong={:#x} ctype={} world={:#x}\n",
            if is_ult { "궁" } else { "" }, cname, a00, rd_u64(a00).unwrap_or(0), copy.a30, world));
    }
}

unsafe extern "C" fn cap_jt(saved: usize, e: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| on_jt(saved, e)));
}

/// 저장해 둔 바닐라 궁 req를 **사일러스 궁 문맥**으로 다시 디스패치한다.
unsafe fn replay_vanilla(saved: usize, e: usize) {
    let snap = { VANILLA_ULT.lock().unwrap_or_else(|x| x.into_inner()).clone() };
    let Some((who, buf, act)) = snap else { hlog("  [바닐라재생] 캡처된 궁 없음\n"); return };
    let sc = rd_u64(act).unwrap_or(0);
    if sc == 0 || sc > 0x100000 { hlog(&format!("  [바닐라재생] action strong={:#x} 이상 중단\n", sc)); return; }
    // 사일러스 궁 Combine apply가 받은 문맥
    let world = *((saved + 0x18) as *const u64) as usize; // r8
    let wops  = *((saved + 0x10) as *const u64) as usize; // r9
    let (Some(p6), Some(p7)) = (rd_u64(e + 0x30), rd_u64(e + 0x38)) else { return };
    let tgt_key = rd_u64(p7 as usize + 8).unwrap_or(u64::MAX);
    let tgt_ent = if tgt_key != u64::MAX { resolve(world, tgt_key) } else { 0 };
    let hp0 = if tgt_ent != 0 { rd_u64(tgt_ent + E_CUR_HP) } else { None };

    let mut req = buf;  // 스택 사본 — JT가 [req+0x30] 등을 읽는다
    let f_addr = exe_base() + JT_RVA;
    hlog(&format!("★[바닐라재생] src={} action={:#x} strong={:#x} 대상hp={:?}\n", who, act, sc, hp0));
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let f: Jt5 = core::mem::transmute(f_addr);
        f(req.as_mut_ptr() as usize, world, wops, p6 as usize, p7 as usize)
    }));
    let hp1 = if tgt_ent != 0 { rd_u64(tgt_ent + E_CUR_HP) } else { None };
    let d = match (hp0, hp1) { (Some(a), Some(b)) => (b as i64) - (a as i64), _ => 0 };
    hlog(&format!("   → ret={:?} | ★HP변화 {:?}→{:?} (Δ{})\n", r.map(|v| v & 0xffff_ffff).ok(), hp0, hp1, d));
}

// ★entity 틱 훅 — take 직전에 사일러스 큐를 들여다본다(우리 다른 훅은 이미 비워진 뒤라 못 본다).
/// 직전 틱의 시전 상태(전이 감지용). u64::MAX = 미관측.
static LAST_CAST_ST: AtomicU64 = AtomicU64::new(u64::MAX);
/// cctx를 엔티티 필드에 직접 쓸지(정본). cfg `cctx_ent=1`
static CCTX_ENT: AtomicBool = AtomicBool::new(false);

unsafe fn on_etick(_saved: usize, _e: usize) {
    if ULT_CENSUS.load(Ordering::Relaxed) { ult_census(); }
    // ★★★[v87] cctx 정본 개입 — **시전 상태 6(궁) 진입 틱에 1회** `ent+0x88`을 교정한다.
    //   여기가 상류다: 이 값이 fire-record → 조준 리드 → apply 로 흘러간다.
    //   (apply 시점 교체는 리드가 지나간 뒤라 조준이 죽는다 — v82~v86 실패, RE#15)
    // ★★★[v100] cctx 공급 — **이식 등록부(SLOT_MINE)의 키에서 매번 재도출**한다.
    //   ~~SYLAS_ENT 전역 raw 포인터~~ 는 v96에서 폐기한 함정인데 이 경로에만 남아 있었고,
    //   그래서 v99에서 cctx 코드가 **한 번도 안 돌았다**(궁시전개시 0건).
    //   시전 상태 6 동안 매번 써도 멱등이다(엔진은 개시 때 1회만 쓴다).
    if CCTX_ENT.load(Ordering::Relaxed) && ETICK_N.fetch_add(1, Ordering::Relaxed) % 8 == 0 {
        let list: Vec<(usize, u64)> = { SLOT_MINE.lock().unwrap_or_else(|x| x.into_inner())
            .iter().map(|q| (q.0, q.1)).collect() };
        for (w, k) in list {
            let e = match champ_by_key(w, k, "sylas") { Some(v) => v, None => continue };
            if rd_u64(e + E_CAST_ST) != Some(6) { continue; }
            if let Some(msg) = fix_entity_cctx(w, e, k) {
                if CCTX_LOG.fetch_add(1, Ordering::Relaxed) < 24 {
                    hlog(&format!("[궁시전중] world={:#x} key={} ent={:#x}
{}", w, k, e, msg));
                }
            }
        }
    }
    if !QUEUE_PROBE.load(Ordering::Relaxed) { return; }
    let se = SYLAS_ENT.load(Ordering::Relaxed) as usize;
    if se < 0x10000 { return; }
    let len = rd_u64(se + E_Q_LEN).unwrap_or(0);
    if len == 0 || len > 16 { return; }
    if ETICK_DIAG.fetch_add(1, Ordering::Relaxed) >= 12 { return; }
    hlog(&dump_queue(se, "sylas(틱 직전)"));
}
unsafe extern "C" fn cap_etick(saved: usize, e: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| on_etick(saved, e)));
}

unsafe extern "C" fn cap_combine(saved: usize, e: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| on_combine(saved, e)));
}

unsafe fn install_detour(rva: usize, hook_len: usize, expect: &[u8], cap_fn: usize) -> Result<usize, String> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0".into()); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, hook_len + 8) { return Err(format!("fn {:#x} unreadable", fn_addr)); }
    let mut cur = [0u8; 20];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 20);
    if &cur[..expect.len()] != expect {
        return Err(format!("프롤로그 불일치 rva={:#x} 실제={:02x?}", rva, &cur[..expect.len().min(20)]));
    }
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc".into()); }
    let ret_addr = fn_addr + hook_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);                                    // mov r10, rsp (orig_rsp)
    s.extend_from_slice(&[0x51,0x52,0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]); // push rcx,rdx,r8,r9,r10,r11
    s.extend_from_slice(&[0x48,0x89,0xe1]);                                    // mov rcx, rsp (saved)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);                                    // mov rdx, r10 (e=orig_rsp)
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);                              // sub rsp, 0x28
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes()); // mov rax, cap_fn
    s.extend_from_slice(&[0xff,0xd0]);                                        // call rax
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);                              // add rsp, 0x28
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]); // pop r11..rcx
    let mut orig = vec![0u8; hook_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), hook_len);
    s.extend_from_slice(&orig);                                               // 원본 프롤로그(Grab 계속)
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes()); // mov rax, ret_addr
    s.extend_from_slice(&[0xff,0xe0]);                                        // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; hook_len];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, hook_len, RWX, &mut old) == 0 { return Err("VirtualProtect".into()); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, hook_len);
    VirtualProtect(fn_addr, hook_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, hook_len);
    Ok(stub)
}

unsafe extern "C" fn cap_grab(saved: usize, e: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| hijack_grab(saved, e)));
}

static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
fn init(_ctx: &GameCtx) -> ModRegistration {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(6));
        if HOOK_INSTALLED.swap(true, Ordering::Relaxed) { return; }
        veh_install();
        cfg_refresher();
        let base = exe_base();
        hlog(&format!("\n===== sylas 통합모드(강탈 프로브 v12) 시작 base={:#x} =====\n", base));
        // ★★★[v129] **자기점검 — effect vtable 상수가 이 exe에서 유효한가.**
        //   이번 세션의 근본 사고: 0.5.7에서 vtable이 35→36슬롯으로 바뀌었는데 상수가 0.5.6이었고,
        //   `default_eff_stubs()`가 **조용히 None을 반환하며 원본을 그대로 쓰는 soft-fail** 이라
        //   기능(AI 마스킹)이 내내 죽어 있어도 크래시도 경고도 없었다.
        //   ⚠**mig_verify의 바이트 대조로는 이걸 못 잡는다** — 그 주소에 여전히 어떤 바이트가 있으면
        //   PASS가 나온다("주소가 유효한가"만 보고 "여전히 올바른 구조인가"는 안 본다).
        //   ⟹ 구조 자체를 검사하는 자기점검을 **매 실행 첫 줄**에 둔다. 실패하면 눈에 띄게 남는다.
        unsafe {
            match default_eff_stubs() {
                Some(d) => hlog(&format!("[자기점검] effect vtable OK — base RVA:{:#x} N={} stride={:#x} (기본스텁 이동={:#x} blink={:#x} is_move={:#x} 스킬샷={:#x} AoE={:#x})
",
                    EFF_VT_BASE, EFF_VT_N, EFF_VT_STRIDE,
                    rva_of(d[0]), rva_of(d[1]), rva_of(d[2]), rva_of(d[3]), rva_of(d[4]))),
                None => hlog(&format!("★★★[자기점검 실패] effect vtable 상수가 이 exe와 안 맞는다 — base RVA:{:#x} N={} stride={:#x}
    ⟹ 게임이 패치됐을 가능성이 높다. ai_mask/order_mask는 **무동작**이 된다(크래시는 안 난다).
    재핀법 = .rdata를 stride 후보로 훑어 **한 슬롯이 전 엔트리에서 동일한** 표를 찾는다(0.5.7 기준 +0x108이 57/57 전부 mov eax,0xbb80; ret = 48000).
    정본 = REPORT/sylas/RE/2026-08-29_챔프별-궁메타-전수와-0.5.7-effect-vtable-레이아웃-변경.md
",
                    EFF_VT_BASE, EFF_VT_N, EFF_VT_STRIDE)),
            }
        }
        match unsafe { install_detour(GRAB_RVA, GRAB_LEN, &GRAB_SIG, cap_grab as *const () as usize) } {
            Ok(stub) => hlog(&format!("[install] Grab_apply @{:#x} OK stub={:#x}\n", base + GRAB_RVA, stub)),
            Err(e)   => hlog(&format!("[install] Grab_apply 실패: {}\n", e)),
        }
        // ★base_ult_apply 훅 재도입 — 단 **world 필터를 훅 최상단**에 두어 부하를 억제한다.
        //   (이전 3연속 사망 때는 필터가 없었다. JT 훅은 같은 처방으로 안정화됐다.)
        //   effect 생성기의 입력인 **action 원본**은 이 훅에서만 얻을 수 있다.
        let cfg0 = cfg_read();
        if cfg_flag(&cfg0, "hook_baseult") {
            match unsafe { install_detour(BASEULT_RVA, BASEULT_LEN, &BASEULT_SIG, cap_baseult2 as *const () as usize) } {
                Ok(stub) => hlog(&format!("[install] base_ult_apply(action 원본) @{:#x} OK stub={:#x}\n", base + BASEULT_RVA, stub)),
                Err(e)   => hlog(&format!("[install] base_ult_apply 실패: {}\n", e)),
            }
        }
        // ★JT 디스패처 훅 = 바닐라 action 캡처처. v20 단독 운용에서 크래시 0으로 안정 실증됐다
        //   (게임을 죽인 것은 base_ult_apply 훅이었다). world 필터를 훅 최상단에 둬 부하를 억제한다.
        match unsafe { install_detour(JT_RVA, JT_LEN, &JT_SIG, cap_jt as *const () as usize) } {
            Ok(stub) => hlog(&format!("[install] JT_dispatch(action 캡처) @{:#x} OK stub={:#x}\n", base + JT_RVA, stub)),
            Err(e)   => hlog(&format!("[install] JT_dispatch 실패: {}\n", e)),
        }
        // ★★[v87] cctx 정본 개입(`ent+0x88`)도 이 훅을 탄다 — 게이트를 넓힌다.
        //   ⚠`contains`는 `x_queue_probe=1` 같은 다른 줄에도 걸린다(v80 buff_graft 실사고).
        //   줄 단위 정확 일치인 cfg_flag를 쓴다.
        if cfg_flag(&cfg0, "queue_probe") || cfg_flag(&cfg0, "cctx_ent") {
            match unsafe { install_detour(ETICK_RVA, ETICK_LEN, &ETICK_SIG, cap_etick as *const () as usize) } {
                Ok(stub) => hlog(&format!("[install] entity_tick(큐 관찰) @{:#x} OK stub={:#x}\n", base + ETICK_RVA, stub)),
                Err(e)   => hlog(&format!("[install] entity_tick 실패: {}\n", e)),
            }
        }
        match unsafe { install_detour(AIEVAL_RVA, AIEVAL_LEN, &AIEVAL_SIG, cap_aieval as *const () as usize) } {
            Ok(a) => hlog(&format!("[install] ai_eval @{:#x} OK\n", a)),
            Err(e) => hlog(&format!("[install] ai_eval 실패: {}\n", e)),
        }
        match unsafe { install_detour(CVIEW_RVA, CVIEW_LEN, &CVIEW_SIG, cap_cview as *const () as usize) } {
            Ok(a) => hlog(&format!("[install] caster_view_effect @{:#x} OK\n", a)),
            Err(e) => hlog(&format!("[install] caster_view_effect 실패: {}\n", e)),
        }
        match unsafe { install_detour(KZONE_RVA, KZONE_LEN, &KZONE_SIG, cap_kzone as *const () as usize) } {
            Ok(a) => hlog(&format!("[install] knight_zone_effect @{:#x} OK\n", a)),
            Err(e) => hlog(&format!("[install] knight_zone_effect 실패: {}\n", e)),
        }
        // ★★[v125] 시전 애니 태그 = **이미터 큐 스왑**(권장안). 화이트리스트가 있어야 의미가 있다.
        if cfg_val(&cfg0, "emit_swap").as_deref() != Some("0") {
            if unsafe { load_anim_whitelist() } > 0 {
                match unsafe { install_trampn(EMIT_RVA, &EMIT_SIG,
                                              emit_detour as *const () as usize, &EMIT_TRAMP) } {
                    Ok(st) => hlog(&format!("[install] cast_anim_emitter @{:#x} OK stub={:#x}
",
                                            exe_base()+EMIT_RVA, st)),
                    Err(e) => hlog(&format!("[install] cast_anim_emitter 실패: {}
", e)),
                }
            } else {
                hlog("[install] cast_anim_emitter 생략 — fanim 화이트리스트가 비었다
");
            }
        }
        // ★★[v126] CasterAnimation(sub-tag 0xd) — 이미터가 못 잡는 두 번째 태그 경로
        if cfg_val(&cfg0, "emit_swap").as_deref() != Some("0") {
            if !ANIM_SET.lock().unwrap_or_else(|x| x.into_inner()).is_empty() {
                match unsafe { install_trampn(CASTANIM_RVA, &CASTANIM_SIG,
                                              castanim_detour as *const () as usize, &CASTANIM_TRAMP) } {
                    Ok(st) => hlog(&format!("[install] caster_animation_push @{:#x} OK stub={:#x}
",
                                            exe_base()+CASTANIM_RVA, st)),
                    Err(e) => hlog(&format!("[install] caster_animation_push 실패: {}
", e)),
                }
            }
        }
        // ⚠[v119, 기본 OFF] 태그 선택기 후킹 — SetAnimation 처리부 안이라 **애니 타이머가 매번 리셋**돼
        //   프레임 0에 고정된다(2026-08-26 실증). 진단용으로만 남긴다. emit_swap 과 동시 사용 금지.
        if cfg_val(&cfg0, "tag_swap").as_deref() == Some("1") {
            if unsafe { load_anim_whitelist() } > 0 {
                match unsafe { install_tramp12(TAGSEL_RVA, &TAGSEL_SIG,
                                               tagsel_detour as *const () as usize, &TAGSEL_TRAMP) } {
                    Ok(st) => hlog(&format!("[install] tag_selector @{:#x} OK stub={:#x}
", exe_base()+TAGSEL_RVA, st)),
                    Err(e)  => hlog(&format!("[install] tag_selector 실패: {}
", e)),
                }
            }
        }
        match unsafe { install_detour(COMBINE_RVA, COMBINE_LEN, &COMBINE_SIG, cap_combine as *const () as usize) } {
            Ok(stub) => hlog(&format!("[install] Combine_apply(재생루프) @{:#x} OK stub={:#x}\n", base + COMBINE_RVA, stub)),
            Err(e)   => hlog(&format!("[install] Combine_apply 실패: {}\n", e)),
        }
    });
    ModRegistration::new("sylas")
}

declare_mod!(init);
