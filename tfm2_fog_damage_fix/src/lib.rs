// tfm2_fog_damage_fix — 시야 밖 착탄 노데미지 완화 (라스트씬까지 인정)
// =====================================================================================
// 현상: 투사체(석궁병 화살 등)가 착탄하는 순간 대상이 캐스터 팀 시야에 "지금" 보이지
//   않으면(state!=0) 이펙트 적용 루프가 통째로 스킵 → 데미지 미발생. 반복형 투사체(태그7)는
//   재장전 블록이 적용 경로 안에만 있어 시야 이탈 착탄 1회로 반복 체인(DoT)이 영구 종료.
//
// 게이트 실체 (0.5.2 buildid 24310934, 재핀 2026-07-22 / 최초 RE 0.5.1 2026-07-21):
//   착탄 해석 함수 0x21ff390 (projectile.rs, 구 0.5.1 0x1b7b770) 내 raw 시야체크 2곳.
//   visible = ( *(target + 0x38 + casterSide*0x18) == 0 )   // 0=지금 보임 / 1=라스트씬 / ≥2=미인지
//   - 사이트 A (태그6, 단발 타겟형):  cmp @0x22022ca `49 83 7C C7 38 00` → sete r14b @0x22022d0
//   - 사이트 B (태그7, 반복·재타겟형): cmp @0x22022f4 `48 83 7C C3 38 00` → sete dil  @0x22022fa
//     ⚠0.5.2에서 B의 sete 목적 레지스터만 r8b(`41 0F 94 C0`) → dil(`40 0F 94 C7`)로 바뀜
//       (레지스터 할당 차이·로직 동일) ⟹ fixed 바이트도 `41 B0 01`(mov r8b,1) → `40 B7 01`(mov dil,1).
//
// 패치: "지금 보임만 인정" → "라스트씬까지 인정"(AI 판단 게이트 0x222bd90과 동일 기준).
//   cmp imm8 0→2  +  sete(0F 94)→setb(0F 92)  ⟹  visible = (state < 2) = state∈{0,1}
//   완전 미인지(state≥2) 대상은 원본대로 미적용. 함수 중간부 imm/opcode 바이트만 교체 —
//   분기 구조·RIP-rel 무관, detour 없음 = AV 위험 없음.
//
// ⚠ 시드 재시뮬레이션 게임이라 이 패치 적용 시 sim 결과가 바닐라와 달라짐(재현 검증 시 주의).
// ⚠ byte mismatch면 RVA stale(패치 옴) → 조용히 스킵+로그만. /migrate 후 재핀.
//
// 빌드: powershell -File C:\tfm2mods\build_inj.ps1 -Src C:\tfm2mods\tfm2_fog_damage_fix\src\lib.rs -ModId tfm2_fog_damage_fix
// =====================================================================================

use mod_api::*;
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "tfm2_fog_damage_fix";

// ── 패치 테이블 (0.5.2 buildid 24310934, image_base 0x140000000) ──
// 0.5.2 마이그(2026-07-22): 5사이트 전부 **로직 무변경·주소 이동만**(exe↔exe 바이트 시그 대조 + ghidra-re
//   panic-Location(소스경로:행) 대조로 함수 대응 확정). 시야배열 레이아웃(target+0x38+side*0x18)·
//   게이트 함수내 오프셋(+0xc4/+0xc4/+0xc5)·jne rel32(+0x1df/+0x1df/+0x19f) 전부 보존.
//   구(0.5.1) → 신(0.5.2): A 0x1b7e3a7→0x22022ca / B 0x1b7e3d1→0x22022f4 /
//   native 0x1dbb364→0x201c274 / data 0x1dba014→0x2019aa4 / v3 0x1db4865→0x2005085.
//   ★코어 native↔data는 바이트 쌍둥이라 시그 대조로 구분 불가 → attack.rs 행번호(141 vs 459)로 판정,
//     0.5.1 나열순 대비 **앞 두 개가 서로 스왑**됐음에 주의(패치 내용은 동일해 결과는 무관).
// ⚠0.5.2 신설 4번째 시야게이트 0x2367c3f(함수 0x2367c20)는 **의도적으로 패치하지 않음** —
//   데미지 경로가 아니라 plan_legacy engage 핸들러의 **AI 타겟 후보 필터**(0.5.1엔 없던 신규).
//   무력화하면 AI가 안 보이는 적까지 교전대상으로 삼는 전지적 AI가 됨 = 이 모드 목적 밖.
// v0.3: "라스트씬까지 인정"(state<2)으로는 여전히 미적용(인게임 확인 — 시야 이탈 시 state가
//   곧바로 ≥2로 넘어가는 것으로 추정) → **시야 완전 무시**로 전환. orig는 항상 게임 원본 바이트.
struct Patch { name: &'static str, rva: usize, orig: &'static [u8], fixed: &'static [u8] }
const PATCHES: &[Patch] = &[
    // 사이트 A (태그6, 단발 타겟형): cmp qword [r15+rax*8+0x38],0 ; sete r14b
    //   → sete를 mov r14b,1 + nop으로 대체 = 항상 visible 취급 (cmp는 무해하게 유지)
    Patch { name: "impact_vision_gate_A", rva: 0x22022ca,
            orig:  &[0x49, 0x83, 0x7c, 0xc7, 0x38, 0x00, 0x41, 0x0f, 0x94, 0xc6],
            fixed: &[0x49, 0x83, 0x7c, 0xc7, 0x38, 0x00, 0x41, 0xb6, 0x01, 0x90] },
    // 사이트 B (태그7, 반복·재타겟형): cmp qword [rbx+rax*8+0x38],0 ; sete dil → mov dil,1 + nop
    //   (0.5.1은 sete r8b `41 0F 94 C0` → 0.5.2 sete dil `40 0F 94 C7`. REX 0x40 + B0+7 = mov dil,imm8)
    Patch { name: "impact_vision_gate_B", rva: 0x22022f4,
            orig:  &[0x48, 0x83, 0x7c, 0xc3, 0x38, 0x00, 0x40, 0x0f, 0x94, 0xc7],
            fixed: &[0x48, 0x83, 0x7c, 0xc3, 0x38, 0x00, 0x40, 0xb7, 0x01, 0x90] },
    // ── 데미지 코어 내부의 독립 시야 게이트 3곳 (RE 2026-07-21 2차) ──
    // 착탄 게이트와 별개로, 데미지 코어가 `cmp qword[target+0x38+side*0x18],0 ; jne skip`으로
    //   재차 차단(state≠0 → 데미지 스킵). jne 6B를 NOP = 시야 무관 데미지 진행.
    //   패치 단위 = cmp imm8 + jne 6B = 7바이트 (rel32 포함 orig 검증).
    // 코어1: 0x201c1b0 (구 0.5.1 FUN_141dbb2a0) = 네이티브 AttackEffect(석궁병 ult/skill1/skill2 등)
    //        데미지 코어 — `effect\type\attack.rs:141`
    Patch { name: "dmgcore_native_gate", rva: 0x201c274,
            orig:  &[0x00, 0x0f, 0x85, 0xdf, 0x01, 0x00, 0x00],
            fixed: &[0x00, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90] },
    // 코어2: 0x20199e0 (구 0.5.1 FUN_141db9f50) = 데이터 Attack(평타 TargetProjectile) 데미지 코어
    //        — `effect\type\attack.rs:459`
    Patch { name: "dmgcore_data_gate", rva: 0x2019aa4,
            orig:  &[0x00, 0x0f, 0x85, 0xdf, 0x01, 0x00, 0x00],
            fixed: &[0x00, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90] },
    // 코어3: 0x2004fc0 (구 0.5.1 FUN_141db47a0) = 제3 변형 데미지 코어
    Patch { name: "dmgcore_v3_gate", rva: 0x2005085,
            orig:  &[0x00, 0x0f, 0x85, 0x9f, 0x01, 0x00, 0x00],
            fixed: &[0x00, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90] },
];

type BOOL = i32; type DWORD = u32; type HMODULE = usize;
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleHandleExW(flags: u32, addr: *const u16, h: *mut HMODULE) -> BOOL;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, sz: DWORD) -> DWORD;
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
        p.push("tfm2_fog_damage_fix.txt");
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = write!(f, "{}", s); let _ = f.flush(); }
    }
}

unsafe fn apply_one(p: &Patch) -> Result<String, String> {
    let base = exe_base(); if base == 0 { return Err("module 0".into()); }
    let n = p.orig.len();
    let addr = base.wrapping_add(p.rva);
    if !readable(addr, n) { return Err(format!("addr unreadable @abs=0x{:x} base=0x{:x}", addr, base)); }
    let mut buf = [0u8; 16];
    core::ptr::copy_nonoverlapping(addr as *const u8, buf.as_mut_ptr(), n);
    let cur = &buf[..n];
    if cur == p.fixed { return Ok(format!("already @abs=0x{:x}", addr)); }   // 멱등
    if cur != p.orig { return Err(format!("byte mismatch @abs=0x{:x} cur={:02x?} want_orig={:02x?} (RVA stale?)", addr, cur, p.orig)); }
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(addr, n, RWX, &mut old) == 0 { return Err("VirtualProtect".into()); }
    core::ptr::copy_nonoverlapping(p.fixed.as_ptr(), addr as *mut u8, n);
    VirtualProtect(addr, n, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, n);
    // write 후 재read 검증
    let mut vbuf = [0u8; 16];
    core::ptr::copy_nonoverlapping(addr as *const u8, vbuf.as_mut_ptr(), n);
    let landed = &vbuf[..n];
    if landed == p.fixed {
        Ok(format!("patched+VERIFIED @abs=0x{:x}", addr))
    } else {
        Err(format!("write 미반영 @abs=0x{:x} landed={:02x?}", addr, landed))
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    log(&format!("[{}ms] === {} INIT ({} patches, 0.5.2 buildid 24310934) ===\n", now_ms(), MOD_ID, PATCHES.len()));
    unsafe {
        for p in PATCHES {
            match apply_one(p) {
                Ok(st) => log(&format!("[patch] {} @0x{:x} {}\n", p.name, p.rva, st)),
                Err(e) => log(&format!("[patch] {} @0x{:x} 실패: {}\n", p.name, p.rva, e)),
            }
        }
    }
    ModRegistration::new(MOD_ID)
}
declare_mod!(init);
