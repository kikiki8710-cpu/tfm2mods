//! tfm2_bancard_keep — 환경설정 "밴 카드 수" 리셋 방지 핫픽스 (게임 0.5.5)
//! ===========================================================================
//! 문제: 환경설정의 "밴 카드 수"(1~5장)를 설정해도 팝업을 껐다 켜면 0(자동)으로 리셋.
//! 원인(RE 정본 = REPORT\tfm2_bancard_keep\RE\2026-08-12_밴카드설정-리셋-원인-RE.md):
//!   룸 설정 병합/커밋 새니타이저 FUN_1421d0ad0(0.5.5 RVA 0x21d0ad0)의 클램프가
//!   챔피언 풀(+0x165c8) < banpick_style*20 + ban*2 + 15 이면
//!   GPO+0x720(room_practice_ban_count)을 0으로 강제(리셋 store @0x21d11d2).
//!   0.5.4에도 있던 설계 동작(회귀 아님).
//! 해법:
//!   (1) 바이트패치 1회(모드 로드 시): RVA 0x21d11d2 의
//!       `mov qword [rsi+0x720], 0` 11B → NOP×11. orig 실측 검증 후에만 패치
//!       (불일치 시 skip + 실측 바이트 로그 — 조용한 오패치 금지). 멱등(이미 NOP=OK).
//!       직전 0x21d11a7 에서 incoming ban 이 이미 [rsi+0x720]에 복사돼 있어
//!       리셋만 무력화하면 사용자 값이 유지된다.
//!   (2) 진단 로그(잔여 불확실성 (A)/(B) 판별용): mods\<MOD_ID>\bancard_keep.txt 에
//!       ①패치 성공/skip ②서버 tick 에서 ~2초 간격으로 GPO+0x720 폴링,
//!       값이 바뀌는 순간 타임스탬프와 함께 기록.
//!       (A) 팝업 커밋 직후 +0x720 이 선택값(1~5)으로 바뀌고 유지 → 클램프가 원인, 패치로 해결.
//!       (B) 팝업 커밋 후에도 +0x720 이 전혀 안 바뀜 → 커밋(영속) 누락 — 패치로 안 고쳐짐,
//!           옵션 셋터 경로 추가 RE 필요.
//! 안전: 순수 코드 바이트 write(VirtualProtect RW→원복) — detour/shadow-call 없음.
//!   exe base 는 런타임 GetModuleHandleW(null) 기준(하드코딩 금지).
//!   GPO 읽기는 SDK 가 주는 &Database 참조 기준 +0x720 read 전용
//!   (참조구현 = community_reaction_mod capture_gpo, GPO+0x729~72b 동일 패턴).
//! ===========================================================================
#![allow(dead_code)]
use mod_api::*;
use std::io::Write as _;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "tfm2_bancard_keep";

// ── 패치 사이트 (0.5.5 확정 — RE 정본 2026-08-12) ──
// FUN_1421d0ad0 내 리셋 store: mov qword ptr [rsi+0x720], 0
const PATCH_RVA: usize = 0x21d11d2;
const PATCH_ORIG: [u8; 11] = [0x48, 0xC7, 0x86, 0x20, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
const PATCH_FIXED: [u8; 11] = [0x90; 11];

// GPO(GamePlayOption) = Database+0x0. +0x720 = room_practice_ban_count(u64),
// +0x737 = banpick_style(u8) — 클램프 threshold 계산에 쓰이므로 함께 관측(진단 참고용).
const GPO_OFF_BAN_COUNT: usize = 0x720;
const GPO_OFF_BANPICK_STYLE: usize = 0x737;
const POLL_INTERVAL_MS: u64 = 2000;

// ── WinAPI ──
type BOOL = i32;
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleFileNameW(h: usize, buf: *mut u16, sz: u32) -> u32;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old: *mut u32) -> BOOL;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, sz: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
}
#[repr(C)]
#[derive(Default)]
struct MemBasicInfo {
    base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32,
    region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32,
}

#[inline]
unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000;
    const RD: u32 = 0x02 | 0x04 | 0x20 | 0x40;
    const GUARD: u32 = 0x01 | 0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr.wrapping_add(len) <= mbi.base + mbi.region_size
}

// ── 로그 (게임 exe 기준 동적 경로 — 설치위치 하드코딩 금지) ──
fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}
fn ts() -> String {
    // KST(UTC+9) HH:MM:SS.mmm — 유저 관찰("설정 5장 → 팝업 닫음") 시각과 대조용
    let ms = now_ms();
    let s = ms / 1000 + 9 * 3600;
    format!("{:02}:{:02}:{:02}.{:03}", (s / 3600) % 24, (s / 60) % 60, s % 60, ms % 1000)
}
fn mod_dir() -> Option<PathBuf> {
    let mut buf = [0u16; 1024];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), 1024) } as usize;
    if n == 0 || n >= 1024 { return None; }
    let exe = String::from_utf16_lossy(&buf[..n]);
    let mut p = PathBuf::from(exe);
    p.pop(); // exe 파일명 제거 → 게임 설치 폴더
    p.push("mods");
    p.push(MOD_ID);
    Some(p)
}
fn log(msg: &str) {
    if let Some(d) = mod_dir() {
        let _ = std::fs::create_dir_all(&d);
        let p = d.join("bancard_keep.txt");
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&p) {
            let _ = writeln!(f, "[{}] {}", ts(), msg);
        }
    }
}

// ── 바이트패치 (1회, 모드 로드 시) ──
unsafe fn apply_patch() -> Result<String, String> {
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { return Err("GetModuleHandleW(null)=0".into()); }
    let addr = base + PATCH_RVA;
    let n = PATCH_ORIG.len();
    if !readable(addr, n) {
        return Err(format!("addr unreadable @abs=0x{:x} (base=0x{:x} rva=0x{:x})", addr, base, PATCH_RVA));
    }
    let mut cur = [0u8; 11];
    core::ptr::copy_nonoverlapping(addr as *const u8, cur.as_mut_ptr(), n);
    if cur == PATCH_FIXED {
        return Ok(format!("already patched @abs=0x{:x}", addr)); // 멱등
    }
    if cur != PATCH_ORIG {
        // orig 불일치 → 조용한 오패치 금지: skip + 실측 바이트 기록
        return Err(format!(
            "orig 불일치 → 패치 SKIP @abs=0x{:x} 실측={:02x?} 기대={:02x?} (패치버전 확인 필요)",
            addr, cur, PATCH_ORIG
        ));
    }
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(addr, n, RWX, &mut old) == 0 { return Err("VirtualProtect 실패".into()); }
    core::ptr::copy_nonoverlapping(PATCH_FIXED.as_ptr(), addr as *mut u8, n);
    let mut old2: u32 = 0;
    VirtualProtect(addr, n, old, &mut old2);
    FlushInstructionCache(GetCurrentProcess(), addr, n);
    // write 후 재read 검증
    let mut landed = [0u8; 11];
    core::ptr::copy_nonoverlapping(addr as *const u8, landed.as_mut_ptr(), n);
    if landed == PATCH_FIXED {
        Ok(format!("patched+VERIFIED @abs=0x{:x} (base=0x{:x} rva=0x{:x}) NOP×11", addr, base, PATCH_RVA))
    } else {
        Err(format!("write 미반영 @abs=0x{:x} landed={:02x?}", addr, landed))
    }
}

// ── GPO+0x720 진단 폴링 ((A)/(B) 판별용) ──
static LAST_BAN: AtomicU64 = AtomicU64::new(u64::MAX); // sentinel = 미관측
static LAST_POLL_MS: AtomicU64 = AtomicU64::new(0);

fn poll_gpo(ctx: &ServerModContext) {
    let now = now_ms();
    let last = LAST_POLL_MS.load(Ordering::Relaxed);
    if now.wrapping_sub(last) < POLL_INTERVAL_MS { return; }
    LAST_POLL_MS.store(now, Ordering::Relaxed);
    unsafe {
        let base = (&*ctx.database) as *const _ as usize;
        if base < 0x10000 || base >= (1usize << 48) { return; }
        let ban = *((base + GPO_OFF_BAN_COUNT) as *const u64);
        let style = *((base + GPO_OFF_BANPICK_STYLE) as *const u8);
        let prev = LAST_BAN.swap(ban, Ordering::Relaxed);
        if prev == u64::MAX {
            log(&format!("GPO 관측 시작: ban_count(+0x720)={} banpick_style(+0x737)={}", ban, style));
        } else if prev != ban {
            log(&format!("GPO+0x720 ban_count 변화: {} -> {} (banpick_style={})", prev, ban, style));
        }
    }
}

struct BancardServerExt;
impl ModServerExtension for BancardServerExt {
    fn on_server_start(&self, ctx: &mut ServerModContext) {
        LAST_BAN.store(u64::MAX, Ordering::Relaxed); // 세이브 (재)로드마다 관측 재시작
        LAST_POLL_MS.store(0, Ordering::Relaxed);
        poll_gpo(ctx);
    }
    fn before_management_tick(&self, ctx: &mut ServerModContext) { poll_gpo(ctx); }
    fn after_management_tick(&self, ctx: &mut ServerModContext) { poll_gpo(ctx); }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    // src=file!() — build_inj.ps1 신원 검증(dll 내 소스 절대경로 문자열)이 요구.
    // 이 모드는 패닉 위치 문자열이 최적화로 빠질 만큼 작아 명시적으로 박는다.
    log(&format!("mod init v0.1.0 (src={})", file!()));
    // 패치 1회 — init 패닉은 로더로 전파되면 안 되므로 catch_unwind 격리
    let r = catch_unwind(AssertUnwindSafe(|| unsafe { apply_patch() }));
    match r {
        Ok(Ok(m)) => log(&format!("PATCH OK: {}", m)),
        Ok(Err(e)) => log(&format!("PATCH FAIL: {}", e)),
        Err(_) => log("PATCH FAIL: panic in apply_patch"),
    }
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_server_extension(BancardServerExt);
    reg
}
declare_mod!(init);
