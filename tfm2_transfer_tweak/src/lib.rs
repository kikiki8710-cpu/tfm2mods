// tfm2_transfer_tweak — 이적 협상 문턱 완화 + "이적 생각 없음" 절대 게이트에 초고액 오버라이드
// =====================================================================================
// 근거 RE (0.5.2 buildid 24310934, 2026-07-23, 정본 = ANA discovered §9a-1):
//   선수측 수락 판정 = FUN_141d15e90(게이트+배율 문턱 동일 함수, bool 반환).
//   ratio = 제안연봉(연) ÷ 현재연봉(주급[Athlete+0x570]×52).
//   문턱표: 행 = 전투 8스탯 평균(Athlete+0x98..0xd0) / 열 = 나이(베테랑·27~28·≤26):
//       70+   : 1.45 / 1.85 / 2.25
//       65~70 : 1.35 / 1.65 / 1.95
//       <65   : 1.20 / 1.45 / 1.45   (1.45는 70+베테랑과 rdata 슬롯 공유)
//   + 원하는 지위(argmax Athlete+0xf8..0x118)보다 낮은 제안이면 +0.25, 하위지위 가드 +0.3.
//   이적 의향 false → 사유9 거절(돈 무관). 콜사이트 3곳(주간판정/오퍼생성 사전체크/평가) 전부
//   이 함수 경유 ⟹ 함수 detour 하나로 전부 커버.
//
// 하는 일:
//   [P1] 문턱 4종(2.25/1.85/1.95/1.65) = 전용 rdata 테이블 0x3835560 직접 패치 (xref 1, 안전).
//   [P2] 공유 상수 5종(1.2/1.45/1.35/+0.25/+0.3) = 게이트 내 로드 명령의 rip-rel disp 4B만
//        모드 소유 f64 슬롯으로 재지향(다른 코드 사용처는 무영향). 사이트/opcode/원본타깃 검증 후 적용.
//   [H1] FUN_141d15e90 detour: 원본(패치된 문턱 반영) 호출 → false(의향 게이트/문턱 불합격)면
//        "현재연봉 대비 (자기 문턱 + unwilling_surcharge)배 이상 제안"일 때만 true로 오버라이드.
//        ⟹ 잔류 의지가 절대 방어막이 아니라 아주 비싼 프리미엄이 됨. 셀러팀 판정(에이스 등)은 불변.
//
// 설정: dll 옆 transfer_tweak.cfg (BOM 없는 UTF-8, key=value). 없으면 기본값으로 자동 생성.
//   음수 unwilling_surcharge = 오버라이드 OFF(원본 절대 게이트 유지).
//
// ⚠ 결정론 유지: 난수 없음. 같은 주 재시도는 여전히 같은 결과.
// ⚠ AI 팀도 같은 규칙을 쓰므로(오퍼 생성 사전체크가 같은 함수) AI의 영입 판단에도 동일 완화 적용됨.
// ⚠ byte/opcode mismatch = RVA stale(패치 옴) → 해당 항목 조용히 스킵+로그. /migrate 후 재핀.
//
// 빌드: powershell -File C:\tfm2mods\build_inj.ps1 -Src C:\tfm2mods\tfm2_transfer_tweak\src\lib.rs -ModId tfm2_transfer_tweak
// =====================================================================================

use mod_api::*;
use std::fs;
use std::io::Write;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "tfm2_transfer_tweak";

// ── RVA (0.5.2 buildid 24310934, image_base 0x140000000) ──
const RVA_GATE: usize = 0x1d15e90;      // 선수 수락 판정(의향 게이트 + 배율 문턱)
const RVA_TBL: usize = 0x3835560;       // 문턱 테이블 [2.25, 1.85, 1.95, 1.65] (전용, xref 1)
// 게이트 프롤로그 12B = push r15,r14,r13,r12,rsi,rdi,rbp,rbx (rip-rel 없음, 명령 경계 정확)
const GATE_PROLOGUE: [u8; 12] = [0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53];
const TBL_ORIG: [f64; 4] = [2.25, 1.85, 1.95, 1.65];

// rip-rel disp 재지향 사이트 (전부 게이트 함수 내부, opcode 4B + disp32 = 8B 명령)
struct DispSite { name: &'static str, rva: usize, opcode: [u8; 4], orig_target_rva: usize, orig_val: f64, slot: usize }
const DISP_SITES: &[DispSite] = &[
    DispSite { name: "thr_1_20", rva: 0x1d1626b, opcode: [0xf2, 0x0f, 0x10, 0x05], orig_target_rva: 0x374a4e8, orig_val: 1.20, slot: 0 },
    DispSite { name: "thr_1_45", rva: 0x1d162db, opcode: [0xf2, 0x0f, 0x10, 0x05], orig_target_rva: 0x374a498, orig_val: 1.45, slot: 1 },
    DispSite { name: "thr_1_35", rva: 0x1d162e9, opcode: [0xf2, 0x0f, 0x10, 0x05], orig_target_rva: 0x374a4b0, orig_val: 1.35, slot: 2 },
    DispSite { name: "pen_0_25", rva: 0x1d16340, opcode: [0xf2, 0x0f, 0x58, 0x05], orig_target_rva: 0x2cab698, orig_val: 0.25, slot: 3 },
    DispSite { name: "gate_0_30", rva: 0x1d162ab, opcode: [0xf2, 0x0f, 0x58, 0x05], orig_target_rva: 0x372a650, orig_val: 0.30, slot: 4 },
];

// ── Athlete 오프셋 (0.5.2 실측, §9a-1) ──
const O_WEEKLY_SALARY: usize = 0x570;   // f64 주급
const O_AGE: usize = 0x708;             // u64 나이
const O_STATS8: usize = 0x98;           // u64×8 전투 핵심 스탯 (평균 0~100)
const O_STATUS5: usize = 0xf8;          // u64×5 지위별 점수 (argmax = 원하는 지위)

// ── 설정 (AtomicU64 = f64 bits, detour에서 lock-free 읽기) ──
// 기본값 = 1.2~2.25 → 1.1~1.8 선형 축소, 페널티 비례 축소, 의향 오버라이드 가산 +0.8
static CFG_T225: AtomicU64 = AtomicU64::new(0); // ≤26세·70+      (orig 2.25 → 1.80)
static CFG_T185: AtomicU64 = AtomicU64::new(0); // 27~28세·70+    (orig 1.85 → 1.53)
static CFG_T195: AtomicU64 = AtomicU64::new(0); // ≤26세·65~70    (orig 1.95 → 1.60)
static CFG_T165: AtomicU64 = AtomicU64::new(0); // 27~28세·65~70  (orig 1.65 → 1.40)
static CFG_T145: AtomicU64 = AtomicU64::new(0); // 베테랑·70+ & <65·비베테랑 공유 (orig 1.45 → 1.27)
static CFG_T135: AtomicU64 = AtomicU64::new(0); // 베테랑·65~70   (orig 1.35 → 1.20)
static CFG_T120: AtomicU64 = AtomicU64::new(0); // 베테랑·<65     (orig 1.20 → 1.10)
static CFG_PEN: AtomicU64 = AtomicU64::new(0);  // 지위 페널티     (orig 0.25 → 0.17)
static CFG_GATE03: AtomicU64 = AtomicU64::new(0); // 하위지위 가드 (orig 0.30 → 0.20)
static CFG_SUR: AtomicU64 = AtomicU64::new(0);  // 의향 게이트 오버라이드 가산 (+0.8, 음수=OFF)

static TRAMPOLINE: AtomicUsize = AtomicUsize::new(0);
static EXE_BASE: AtomicU64 = AtomicU64::new(0);

#[inline] fn f(a: &AtomicU64) -> f64 { f64::from_bits(a.load(Ordering::Relaxed)) }
#[inline] fn setf(a: &AtomicU64, v: f64) { a.store(v.to_bits(), Ordering::Relaxed); }

// ── Win32 ──
type BOOL = i32; type DWORD = u32; type HMODULE = usize;
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleHandleExW(flags: u32, addr: *const u16, h: *mut HMODULE) -> BOOL;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, sz: DWORD) -> DWORD;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old: *mut u32) -> BOOL;
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, sz: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
}
#[repr(C)] #[derive(Default)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32,
    region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32 }

#[inline] unsafe fn exe_base() -> usize {
    let v = EXE_BASE.load(Ordering::Relaxed) as usize;
    if v != 0 { return v; }
    let b = GetModuleHandleW(core::ptr::null());
    EXE_BASE.store(b as u64, Ordering::Relaxed); b
}
#[inline] unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || addr >= 1usize << 48 || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const RD: u32 = 0x02 | 0x04 | 0x20 | 0x40; const GUARD: u32 = 0x01 | 0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}

fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }
fn dir() -> Option<PathBuf> { unsafe {
    let mut h: HMODULE = 0;
    if GetModuleHandleExW(0x4 | 0x2, dir as *const () as *const u16, &mut h) == 0 || h == 0 { return None; }
    let mut b = [0u16; 4096];
    let n = GetModuleFileNameW(h, b.as_mut_ptr(), b.len() as DWORD);
    if n == 0 { return None; }
    let mut p = PathBuf::from(String::from_utf16_lossy(&b[..n as usize])); p.pop(); Some(p)
}}
fn log(s: &str) {
    if let Some(mut p) = dir() {
        p.push("tfm2_transfer_tweak.txt");
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = write!(f, "{}", s); let _ = f.flush(); }
    }
}

// ── 설정 파일 ──
const CFG_DEFAULT: &str = "\
# tfm2_transfer_tweak 설정 — 이적 협상 문턱 (게임 재시작 시 반영)\n\
# 문턱 = \"제안 연봉이 선수 현재 연봉의 몇 배여야 수락하나\". 행=전투 8스탯 평균, 열=나이.\n\
# 바닐라 값은 [ ] 안. 베테랑 = 31세+ (또는 29~30세 & 계약잔여 2년 이하).\n\
t_young_hi=1.80\n\
# ↑ 26세 이하 · 스탯 70+ [2.25]\n\
t_mid_hi=1.53\n\
# ↑ 27~28세 · 스탯 70+ [1.85]\n\
t_young_md=1.60\n\
# ↑ 26세 이하 · 스탯 65~70 [1.95]\n\
t_mid_md=1.40\n\
# ↑ 27~28세 · 스탯 65~70 [1.65]\n\
t_shared=1.27\n\
# ↑ 베테랑 · 스탯 70+ 및 비베테랑 · 스탯 65미만 (게임이 같은 상수를 공유) [1.45]\n\
t_vet_md=1.20\n\
# ↑ 베테랑 · 스탯 65~70 [1.35]\n\
t_vet_lo=1.10\n\
# ↑ 베테랑 · 스탯 65미만 [1.20]\n\
pos_penalty=0.17\n\
# ↑ 원하는 지위보다 낮은 지위 제안 시 가산 [0.25]\n\
low_status_gate=0.20\n\
# ↑ 하위 지위 제안 선거절 가드 가산 [0.30]\n\
unwilling_surcharge=0.80\n\
# ↑ \"이적 생각 없음\"(바닐라 = 돈 무관 무조건 거절) 선수를 돈으로 데려올 때 문턱에 추가되는 가산.\n\
#   예: 스탯 70+ · 26세 이하가 이적 생각 없으면 1.80+0.80 = 현재 연봉의 2.6배 이상 제안 시 수락.\n\
#   음수(-1 등)로 두면 바닐라처럼 무조건 거절로 복귀.\n";

fn load_cfg() {
    // 기본값 먼저
    setf(&CFG_T225, 1.80); setf(&CFG_T185, 1.53); setf(&CFG_T195, 1.60); setf(&CFG_T165, 1.40);
    setf(&CFG_T145, 1.27); setf(&CFG_T135, 1.20); setf(&CFG_T120, 1.10);
    setf(&CFG_PEN, 0.17); setf(&CFG_GATE03, 0.20); setf(&CFG_SUR, 0.80);
    let Some(mut p) = dir() else { return };
    p.push("transfer_tweak.cfg");
    let text = match fs::read_to_string(&p) {
        Ok(t) => t,
        Err(_) => {
            // BOM 없는 UTF-8 생성 (fs::write는 BOM을 쓰지 않음)
            let _ = fs::write(&p, CFG_DEFAULT.as_bytes());
            log(&format!("[cfg] 기본 설정 생성: {}\n", p.display()));
            return;
        }
    };
    let mut n = 0;
    for line in text.lines() {
        let line = line.trim_start_matches('\u{feff}').trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let Some((k, v)) = line.split_once('=') else { continue };
        let Ok(v) = v.trim().parse::<f64>() else { continue };
        let slot = match k.trim() {
            "t_young_hi" => &CFG_T225, "t_mid_hi" => &CFG_T185,
            "t_young_md" => &CFG_T195, "t_mid_md" => &CFG_T165,
            "t_shared" => &CFG_T145, "t_vet_md" => &CFG_T135, "t_vet_lo" => &CFG_T120,
            "pos_penalty" => &CFG_PEN, "low_status_gate" => &CFG_GATE03,
            "unwilling_surcharge" => &CFG_SUR,
            _ => continue,
        };
        // 가산 2종 외 문턱값은 0.5~10 범위 가드 (0/음수 문턱 = 전원 즉시수락 사고 방지)
        let is_thr = !matches!(k.trim(), "pos_penalty" | "low_status_gate" | "unwilling_surcharge");
        if is_thr && !(0.5..=10.0).contains(&v) { log(&format!("[cfg] {}={} 범위 밖(0.5~10) — 무시\n", k, v)); continue; }
        setf(slot, v); n += 1;
    }
    log(&format!("[cfg] 로드 {}건: 표=[{}/{}/{} | {}/{}/{} | {}/{}/{}] pen={} gate={} sur={}\n", n,
        f(&CFG_T145), f(&CFG_T185), f(&CFG_T225),
        f(&CFG_T135), f(&CFG_T165), f(&CFG_T195),
        f(&CFG_T120), f(&CFG_T145), f(&CFG_T145),
        f(&CFG_PEN), f(&CFG_GATE03), f(&CFG_SUR)));
}

// detour 폴백용 문턱표 조회 (row: 0=70+ 1=65~70 2=<65 / col: 0=베테랑 1=27~28 2=≤26)
fn table_val(row: usize, col: usize) -> f64 {
    match (row, col) {
        (0, 0) => f(&CFG_T145), (0, 1) => f(&CFG_T185), (0, 2) => f(&CFG_T225),
        (1, 0) => f(&CFG_T135), (1, 1) => f(&CFG_T165), (1, 2) => f(&CFG_T195),
        (2, 0) => f(&CFG_T120), _ => f(&CFG_T145), // <65 비베테랑 = 공유 1.45 슬롯
    }
}

// ── [P1] rdata 문턱 테이블 패치 ──
unsafe fn patch_table() -> Result<String, String> {
    let base = exe_base(); if base == 0 { return Err("module 0".into()); }
    let addr = base.wrapping_add(RVA_TBL);
    if !readable(addr, 32) { return Err("table unreadable".into()); }
    let newv = [f(&CFG_T225), f(&CFG_T185), f(&CFG_T195), f(&CFG_T165)];
    let cur: [f64; 4] = core::ptr::read(addr as *const [f64; 4]);
    if cur == newv { return Ok("already".into()); }
    if cur != TBL_ORIG { return Err(format!("table mismatch cur={:?} (RVA stale?)", cur)); }
    const RW: u32 = 0x04;
    let mut old: u32 = 0;
    if VirtualProtect(addr, 32, RW, &mut old) == 0 { return Err("VirtualProtect".into()); }
    core::ptr::write(addr as *mut [f64; 4], newv);
    VirtualProtect(addr, 32, old, &mut old);
    let landed: [f64; 4] = core::ptr::read(addr as *const [f64; 4]);
    if landed == newv { Ok(format!("{:?} -> {:?}", TBL_ORIG, newv)) } else { Err("write 미반영".into()) }
}

// ── [P2] rip-rel disp 재지향 ──
// exe ±2GB 안에 f64 슬롯 페이지 확보 (disp32 도달 필수)
unsafe fn alloc_near_exe() -> usize {
    let base = exe_base(); if base == 0 { return 0; }
    const MEM_CR: u32 = 0x1000 | 0x2000; const RW: u32 = 0x04;
    // 이미지 끝 위쪽부터 탐색 (SizeOfImage = PE 헤더 in-memory)
    let nt = base + core::ptr::read((base + 0x3c) as *const u32) as usize;
    let img_size = core::ptr::read((nt + 0x50) as *const u32) as usize;
    let start = (base + img_size + 0xFFFF) & !0xFFFF;
    for i in 0..2048usize {
        let try_addr = start + i * 0x10000;
        let p = VirtualAlloc(try_addr, 0x1000, MEM_CR, RW);
        if p != 0 { return p; }
    }
    // 폴백: 베이스 아래쪽
    for i in 1..2048usize {
        let try_addr = base.wrapping_sub(i * 0x10000);
        if try_addr < 0x10000 { break; }
        let p = VirtualAlloc(try_addr, 0x1000, MEM_CR, RW);
        if p != 0 { return p; }
    }
    0
}

unsafe fn patch_disp_sites() {
    let base = exe_base(); if base == 0 { log("[disp] module 0\n"); return; }
    let slot_page = alloc_near_exe();
    if slot_page == 0 { log("[disp] near-alloc 실패 — 공유상수 5종 미적용(테이블 4종·detour는 유효)\n"); return; }
    let slot_vals = [f(&CFG_T120), f(&CFG_T145), f(&CFG_T135), f(&CFG_PEN), f(&CFG_GATE03)];
    for (i, v) in slot_vals.iter().enumerate() {
        core::ptr::write((slot_page + i * 8) as *mut f64, *v);
    }
    log(&format!("[disp] slot page @0x{:x} vals={:?}\n", slot_page, slot_vals));
    for s in DISP_SITES {
        let site = base.wrapping_add(s.rva);
        let r = (|| -> Result<String, String> {
            if !readable(site, 8) { return Err("unreadable".into()); }
            let mut b = [0u8; 8];
            core::ptr::copy_nonoverlapping(site as *const u8, b.as_mut_ptr(), 8);
            if b[..4] != s.opcode { return Err(format!("opcode mismatch {:02x?} (RVA stale?)", &b[..4])); }
            let disp = i32::from_le_bytes(b[4..8].try_into().unwrap());
            let tgt = site.wrapping_add(8).wrapping_add(disp as isize as usize);
            let slot_addr = slot_page + s.slot * 8;
            if tgt == slot_addr { return Ok("already".into()); }
            if tgt != base.wrapping_add(s.orig_target_rva) {
                return Err(format!("disp target 0x{:x} != 기대 0x{:x} (RVA stale?)", tgt, base + s.orig_target_rva));
            }
            // 원본 상수 값도 확증
            let ov = core::ptr::read(tgt as *const f64);
            if (ov - s.orig_val).abs() > 1e-12 { return Err(format!("const {} != {}", ov, s.orig_val)); }
            let delta = (slot_addr as i128) - ((site + 8) as i128);
            if delta > i32::MAX as i128 || delta < i32::MIN as i128 { return Err("slot >±2GB".into()); }
            const RWX: u32 = 0x40;
            let mut old: u32 = 0;
            if VirtualProtect(site, 8, RWX, &mut old) == 0 { return Err("VirtualProtect".into()); }
            core::ptr::write_unaligned((site + 4) as *mut i32, delta as i32);
            VirtualProtect(site, 8, old, &mut old);
            FlushInstructionCache(GetCurrentProcess(), site, 8);
            Ok(format!("-> slot[{}]={} @0x{:x}", s.slot, slot_vals[s.slot], slot_addr))
        })();
        match r {
            Ok(st) => log(&format!("[disp] {} @0x{:x} {}\n", s.name, s.rva, st)),
            Err(e) => log(&format!("[disp] {} @0x{:x} 실패: {}\n", s.name, s.rva, e)),
        }
    }
}

// ── [H1] 게이트 detour ──
// 원본 시그니처(실측): rcx=game, rdx=ctx, r8=Athlete*, r9=제안팀 id,
//   [rsp+0x20]=제안 지위 byte, [rsp+0x28]=제안 연봉(연) f64 → bool(al).
// 미확인 상위 스택 인자 대비 a7~a10 over-forward(읽기·전달 모두 무해).
type GateFn = unsafe extern "system" fn(usize, usize, usize, u64, u64, u64, u64, u64, u64, u64) -> u8;

unsafe extern "system" fn gate_hook(a1: usize, a2: usize, ath: usize, a4: u64,
                                    a5: u64, a6: u64, a7: u64, a8: u64, a9: u64, a10: u64) -> u8 {
    let r = catch_unwind(AssertUnwindSafe(|| -> u8 {
        let t = TRAMPOLINE.load(Ordering::Relaxed);
        if t == 0 { return 0; }
        let orig: GateFn = core::mem::transmute(t);
        let base = orig(a1, a2, ath, a4, a5, a6, a7, a8, a9, a10);
        if base != 0 { return base; }
        // ── 원본 거절 → 초고액 오버라이드 검사 ──
        let sur = f(&CFG_SUR);
        if !sur.is_finite() || sur < 0.0 { return 0; } // 음수 = 완화 OFF
        if !readable(ath + O_STATS8, 0x40) || !readable(ath + O_WEEKLY_SALARY, 8) || !readable(ath + O_AGE, 8) { return 0; }
        let weekly = core::ptr::read((ath + O_WEEKLY_SALARY) as *const f64);
        if !weekly.is_finite() || weekly <= 0.0 { return 0; } // FA 등 배율 트랙 밖 → 원본 판정 유지
        let offer = f64::from_bits(a6);
        if !offer.is_finite() || offer <= 0.0 { return 0; }
        let ratio = offer / (weekly * 52.0);
        // 전투 8스탯 평균 (원본은 모드별 일부 카테고리 제외가 있으나 전체 평균 근사)
        let mut sum: u64 = 0;
        for i in 0..8 { sum = sum.wrapping_add(core::ptr::read((ath + O_STATS8 + i * 8) as *const u64)); }
        let avg = sum as f64 / 8.0;
        let age = core::ptr::read((ath + O_AGE) as *const u64);
        let row = if avg >= 70.0 { 0 } else if avg >= 65.0 { 1 } else { 2 };
        // 베테랑 열은 나이만으로 근사(29~30세 단기계약 조건 생략 = 그 케이스만 문턱이 약간 높아짐, 보수적)
        let col = if age >= 31 { 0 } else if age >= 27 { 1 } else { 2 };
        let mut thr = table_val(row, col) + sur;
        // 원하는 지위보다 낮은 제안이면 페널티 가산 (원본 로직과 동일 방향)
        if readable(ath + O_STATUS5, 0x28) {
            let mut best = 0usize; let mut bestv = 0u64;
            for i in 0..5 {
                let v = core::ptr::read((ath + O_STATUS5 + i * 8) as *const u64);
                if v > bestv { bestv = v; best = i; }
            }
            if ((a5 & 0xff) as usize) < best { thr += f(&CFG_PEN); }
        }
        if ratio >= thr { 1 } else { 0 }
    }));
    r.unwrap_or(0)
}

unsafe fn install_gate_detour() -> Result<(), String> {
    let base = exe_base(); if base == 0 { return Err("module 0".into()); }
    let fn_addr = base.wrapping_add(RVA_GATE);
    if !readable(fn_addr, 16) { return Err("unreadable".into()); }
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    if cur[0] == 0x48 && cur[1] == 0xb8 && cur[10] == 0xff && cur[11] == 0xe0 {
        return Err("이미 외부 훅 존재 — 정책 교체형이라 체인 불가, 미설치".into());
    }
    if cur != GATE_PROLOGUE { return Err(format!("prologue mismatch {:02x?} (RVA stale?)", cur)); }
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 64, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc".into()); }
    // 트램폴린: 원본 12B(순수 push 8개, 재배치 무관) + movabs rax, fn+12; jmp rax
    let mut s: Vec<u8> = Vec::with_capacity(26);
    s.extend_from_slice(&GATE_PROLOGUE);
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&(fn_addr + 12).to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    TRAMPOLINE.store(stub, Ordering::Release);
    // 진입부 패치: movabs rax, gate_hook; jmp rax (12B, rax는 이 시점 비인자 = 클로버 안전)
    let mut patch = [0u8; 12];
    patch[0] = 0x48; patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&(gate_hook as usize).to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, RWX, &mut old) == 0 { return Err("VirtualProtect".into()); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    Ok(())
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    log(&format!("[{}ms] === {} INIT (0.5.2 buildid 24310934) ===\n", now_ms(), MOD_ID));
    load_cfg();
    unsafe {
        match patch_table() {
            Ok(st) => log(&format!("[table] 0x{:x} {}\n", RVA_TBL, st)),
            Err(e) => log(&format!("[table] 0x{:x} 실패: {}\n", RVA_TBL, e)),
        }
        patch_disp_sites();
        match install_gate_detour() {
            Ok(()) => log(&format!("[detour] gate 0x{:x} 설치 완료 (surcharge={})\n", RVA_GATE, f(&CFG_SUR))),
            Err(e) => log(&format!("[detour] gate 0x{:x} 실패: {} — 오버라이드 비활성(문턱 패치는 별개)\n", RVA_GATE, e)),
        }
    }
    ModRegistration::new(MOD_ID)
}
declare_mod!(init);
