//! tfm2_champion_exclude — 인게임 패치 신챔프 추가 대상에서 특정 챔피언 영구 제외 (게임 0.5.6)
//! ===========================================================================
//! 배경: 시즌 진행 중 패치 데이(ScheduleType Minor/Major/SeasonPatch)에 게임이
//!   "챔피언 registry(바닐라+모드챔프 전부) − available_champions" 를 후보로 뽑아
//!   셔플 후 앞 N개(N=db+0x708, champion_add 설정)를 available_champions 에 push
//!   = 신챔피언 출시. (RE 정본 = REPORT\tfm2_champion_exclude\RE\
//!   2026-08-19_신챔프추가-메커니즘-0.5.5.md)
//! 콜체인(0.5.6 재핀 2026-08-20): day-proceed 0x2109760 → 패치데이 0x2371820
//!   → 신챔프추가 0x2363ee0(콜사이트 0x236406e)
//!   → ★후보 Vec 생성 0x1894610(콜러 0x2363ee0 한 곳뿐 — 0.5.6 실측).
//!   (0.5.5 = 0x1e34a00 → 0x203acc0 → 0x202c440(@0x202c5f2) → 0x186e150)
//! 해법: 0x186e150 진입 트램폴린 detour — 원본 호출로 후보 Vec<String> 을 받은 뒤
//!   cfg(champion_exclude.cfg)에 적힌 챔피언 id 를 swap_remove 로 걸러낸다.
//!   후보에서 빠지면 셔플/선택/available push/액션 등록/팀 티어 반영/뉴스까지
//!   전부 자연히 배제 = "영원히 추가 안 됨". 이미 출시된 챔피언은 건드리지 않는다.
//!   '*' 한 줄이면 후보 전량 제거 = 신챔프 추가 전면 차단.
//! 적용 범위 주의: 이 모드는 "시즌 중 패치로 추가"만 막는다. 신규 게임 시작 시
//!   초기 풀 포함(0.5.5 0xc2d980 경로 — 0.5.6 재핀 NO MATCH·미사용이라 미추적)은
//!   범위 외 — 바닐라 게임 생성 옵션(custom_champions)
//!   으로 제어 가능하고, 필요 시 추가 RE 후 별도 버전에서 다룬다.
//! 안전:
//!   - detour 본문 = catch_unwind(AssertUnwindSafe) 격리(§3 — detour 패닉은 UB).
//!   - 프롤로그 17B 실측 검증 후에만 설치(불일치 = skip + 실측 바이트 로그).
//!     기존 외부 훅(48 B8 .. FF E0)이면 체인 훅(그 타깃으로 점프하는 트램폴린).
//!   - 제거된 String 힙 버퍼는 의도적으로 leak(FREE_REMOVED=false 기본).
//!     게임 Rust 할당자는 프로세스 힙(HeapAlloc) 계열로 보이나(디컴 근거) 확증
//!     전이므로 cross-allocator free 크래시 위험 > 패치데이당 수백 B leak.
//!   - 포인터는 범위체크 + wrapping_add. exe base 는 GetModuleHandleW(null) 동적.
//! cfg: mods\tfm2_champion_exclude\champion_exclude.cfg — 한 줄에 챔피언 id 하나,
//!   '#' 주석, 대소문자 무시, UTF-8(BOM 허용). detour 발화 시마다 재독(핫 편집 가능).
//! 진단: mods\tfm2_champion_exclude\champion_exclude.txt — 설치 결과, 발화마다
//!   후보 전체 덤프(모드챔프 포함 여부 확증용)·제거 목록·before/after 수.
//! ===========================================================================
#![allow(dead_code)]
use mod_api::*;
use std::io::Write as _;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "tfm2_champion_exclude";

// ── 훅 사이트 (0.5.6 재핀 2026-08-20, 스켈레톤해시 UNIQUE — 0.5.5 = 0x186e150, RE 정본 2026-08-19) ──
// 신챔프 추가 후보 Vec<String> 생성 (rcx=out, rdx=iter_ctx, ret rax=out) — 0.5.6 본문 무변경(UNIQUE)
const HOOK_RVA: usize = 0x1894610;
// 프롤로그: push rbp; push r15; push r14; push r12; push rsi; push rdi; push rbx;
//           sub rsp,0xA0  (10B + 7B = 17B, rip-rel 없음 — 0.5.6 실측 바이트 0.5.5와 완전 동일)
const HOOK_ORIG: [u8; 17] = [
    0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x54, 0x56, 0x57, 0x53,
    0x48, 0x81, 0xEC, 0xA0, 0x00, 0x00, 0x00,
];
const ORIG_LEN: usize = 17; // 12B jmp 훅 시 명령 경계
// 제거한 String 의 힙 버퍼를 HeapFree 할지 — 기본 false(의도적 leak, 상단 주석 참조)
const FREE_REMOVED: bool = false;
// 후보 Vec 새니티 상한(이보다 크면 오독으로 보고 필터 skip)
const MAX_CANDIDATES: usize = 4096;
const MAX_NAME_LEN: usize = 256;

// ── WinAPI ──
type BOOL = i32;
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleFileNameW(h: usize, buf: *mut u16, sz: u32) -> u32;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old: *mut u32) -> BOOL;
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, sz: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
    fn GetProcessHeap() -> usize;
    fn HeapFree(heap: usize, flags: u32, ptr: usize) -> BOOL;
}
#[repr(C)]
#[derive(Default)]
struct MemBasicInfo {
    base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32,
    region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32,
}

#[inline]
fn ptr_sane(addr: usize) -> bool {
    addr >= 0x10000 && addr < (1usize << 48)
}

#[inline]
unsafe fn readable(addr: usize, len: usize) -> bool {
    if !ptr_sane(addr) || len == 0 { return false; }
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
    // KST(UTC+9) HH:MM:SS.mmm
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
    p.pop();
    p.push("mods");
    p.push(MOD_ID);
    Some(p)
}
fn log(msg: &str) {
    if let Some(d) = mod_dir() {
        let _ = std::fs::create_dir_all(&d);
        let p = d.join("champion_exclude.txt");
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&p) {
            let _ = writeln!(f, "[{}] {}", ts(), msg);
        }
    }
}

// ── cfg: 제외 챔피언 목록 (발화마다 재독 — 패치데이는 드물어서 비용 무시 가능) ──
// 반환: (소문자 정규화된 제외 id 목록, 전면차단 여부)
fn load_exclude_cfg() -> (Vec<String>, bool) {
    let mut list = Vec::new();
    let mut block_all = false;
    let Some(d) = mod_dir() else { return (list, false) };
    let p = d.join("champion_exclude.cfg");
    let Ok(bytes) = std::fs::read(&p) else {
        // 첫 실행 편의: 템플릿 cfg 생성
        let _ = std::fs::create_dir_all(&d);
        let _ = std::fs::write(&p,
            "# tfm2_champion_exclude — 인게임 패치로 절대 추가되지 않을 챔피언 id 목록\n\
             # 한 줄에 하나. '#' 뒤는 주석. 대소문자 무시.\n\
             # 챔피언 id 는 진단 로그(champion_exclude.txt)의 후보 덤프에서 확인 가능.\n\
             # '*' 한 줄만 적으면 신규 챔피언 추가를 전면 차단.\n\
             # 예)\n\
             # nightmare\n\
             # sand_mage\n");
        return (list, false);
    };
    let text = String::from_utf8_lossy(&bytes);
    let text = text.trim_start_matches('\u{feff}'); // BOM 허용
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() { continue; }
        if line == "*" { block_all = true; continue; }
        list.push(line.to_ascii_lowercase());
    }
    (list, block_all)
}

// ── detour ──
// 원본 계약(0.5.5 RE): rcx=out(*mut Vec<String>), rdx=iter_ctx, 반환 rax=out.
// Vec{cap@0, ptr@8, len@0x10} / 요소 String{cap@0, ptr@8, len@0x10} stride 0x18.
type HookFn = extern "C" fn(usize, usize, usize, usize) -> usize;
static TRAMPOLINE: AtomicUsize = AtomicUsize::new(0);
static FIRE_COUNT: AtomicUsize = AtomicUsize::new(0);

extern "C" fn detour_candidates(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    let tramp = TRAMPOLINE.load(Ordering::Acquire);
    if tramp == 0 {
        // 설치 전 발화는 불가능하지만 방어적으로: 원본 주소 자체를 잃은 상태 = 어쩔 수 없이 0 리턴 불가
        // → 트램폴린 미확보 시 detour 를 설치하지 않으므로 도달 불가.
        return rcx;
    }
    let orig: HookFn = unsafe { core::mem::transmute(tramp) };
    let ret = orig(rcx, rdx, r8, r9);
    // 필터는 전부 격리 — 무슨 일이 있어도 게임 콜스택으로 패닉 전파 금지
    let _ = catch_unwind(AssertUnwindSafe(|| filter_candidates(ret)));
    ret
}

fn filter_candidates(out: usize) {
    let n = FIRE_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    if !ptr_sane(out) || unsafe { !readable(out, 0x18) } {
        log(&format!("fire#{}: out 포인터 이상(0x{:x}) — 필터 skip", n, out));
        return;
    }
    let vec_ptr = unsafe { *((out + 8) as *const usize) };
    let mut len = unsafe { *((out + 0x10) as *const usize) };
    if len == 0 {
        log(&format!("fire#{}: 후보 0개(추가 설정 off 또는 전원 출시) — 조치 없음", n));
        return;
    }
    if len > MAX_CANDIDATES || !ptr_sane(vec_ptr) || unsafe { !readable(vec_ptr, len * 0x18) } {
        log(&format!("fire#{}: 후보 Vec 이상(ptr=0x{:x} len={}) — 필터 skip(오독 방지)", n, vec_ptr, len));
        return;
    }

    let read_name = |idx: usize| -> Option<String> {
        let elem = vec_ptr + idx * 0x18;
        let sptr = unsafe { *((elem + 8) as *const usize) };
        let slen = unsafe { *((elem + 0x10) as *const usize) };
        if slen == 0 || slen > MAX_NAME_LEN || !ptr_sane(sptr) || unsafe { !readable(sptr, slen) } {
            return None;
        }
        let bytes = unsafe { core::slice::from_raw_parts(sptr as *const u8, slen) };
        std::str::from_utf8(bytes).ok().map(|s| s.to_string())
    };

    // 후보 전체 덤프(모드챔프 포함 여부 확증용 진단 — 패치데이당 1~2회라 저비용)
    let mut names: Vec<String> = Vec::with_capacity(len);
    for i in 0..len {
        names.push(read_name(i).unwrap_or_else(|| "<판독불가>".into()));
    }
    let (exclude, block_all) = load_exclude_cfg();
    log(&format!("fire#{}: 후보 {}개 = [{}] / 제외목록 {}개{}",
        n, len, names.join(", "), exclude.len(), if block_all { " + 전면차단(*)" } else { "" }));

    if exclude.is_empty() && !block_all {
        return;
    }

    let mut removed: Vec<String> = Vec::new();
    let mut i = 0usize;
    while i < len {
        let name = read_name(i);
        let hit = match (&name, block_all) {
            (_, true) => true,
            (Some(nm), false) => exclude.iter().any(|e| e == &nm.to_ascii_lowercase()),
            (None, false) => false, // 판독불가 요소는 건드리지 않음
        };
        if hit {
            unsafe {
                let elem = vec_ptr + i * 0x18;
                if FREE_REMOVED {
                    let scap = *(elem as *const usize);
                    let sptr = *((elem + 8) as *const usize);
                    if scap > 0 && ptr_sane(sptr) {
                        HeapFree(GetProcessHeap(), 0, sptr);
                    }
                }
                // swap_remove: 마지막 요소(0x18B)를 이 자리에 복사 후 len−1
                let last = vec_ptr + (len - 1) * 0x18;
                if last != elem {
                    core::ptr::copy_nonoverlapping(last as *const u8, elem as *mut u8, 0x18);
                }
                len -= 1;
                *((out + 0x10) as *mut usize) = len;
            }
            removed.push(name.unwrap_or_else(|| "<판독불가>".into()));
            // i 는 그대로(방금 당겨온 요소 재검사)
        } else {
            i += 1;
        }
    }
    if removed.is_empty() {
        log(&format!("fire#{}: 매치 없음 — 후보 {}개 유지", n, len));
    } else {
        log(&format!("fire#{}: {}개 제거 = [{}] → 후보 {}개 남음",
            n, removed.len(), removed.join(", "), len));
    }
}

// ── 트램폴린 설치 ──
unsafe fn install_hook() -> Result<String, String> {
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { return Err("GetModuleHandleW(null)=0".into()); }
    let addr = base + HOOK_RVA;
    if !readable(addr, ORIG_LEN) {
        return Err(format!("훅 지점 unreadable @abs=0x{:x} (base=0x{:x} rva=0x{:x})", addr, base, HOOK_RVA));
    }
    let mut cur = [0u8; ORIG_LEN];
    core::ptr::copy_nonoverlapping(addr as *const u8, cur.as_mut_ptr(), ORIG_LEN);

    // 트램폴린 페이지 확보 (orig 17B or 체인 12B + movabs jmp 12B)
    const MEM_COMMIT_RESERVE: u32 = 0x1000 | 0x2000;
    const PAGE_EXECUTE_READWRITE: u32 = 0x40;
    let tramp = VirtualAlloc(0, 0x1000, MEM_COMMIT_RESERVE, PAGE_EXECUTE_READWRITE);
    if tramp == 0 { return Err("VirtualAlloc 트램폴린 실패".into()); }

    let mode: &str;
    let mut tlen = 0usize;
    if cur == HOOK_ORIG {
        // 정상 프롤로그: orig 17B + jmp (addr+17)
        core::ptr::copy_nonoverlapping(cur.as_ptr(), tramp as *mut u8, ORIG_LEN);
        tlen = ORIG_LEN;
        let back = addr + ORIG_LEN;
        let jmp: [u8; 12] = jmp_abs(back);
        core::ptr::copy_nonoverlapping(jmp.as_ptr(), (tramp + tlen) as *mut u8, 12);
        tlen += 12;
        mode = "정상 프롤로그 → 트램폴린";
    } else if cur[0] == 0x48 && cur[1] == 0xB8 && cur[10] == 0xFF && cur[11] == 0xE0 {
        // 기존 외부 훅(movabs rax,tgt; jmp rax) — 체인: 트램폴린 = 그 12B 재현
        core::ptr::copy_nonoverlapping(cur.as_ptr(), tramp as *mut u8, 12);
        tlen = 12;
        let ext = usize::from_le_bytes(cur[2..10].try_into().unwrap());
        mode = "기존 외부 훅 감지 → 체인 훅";
        log(&format!("체인: 외부 훅 타깃=0x{:x}", ext));
    } else {
        return Err(format!(
            "프롤로그 불일치 → 설치 SKIP @abs=0x{:x} 실측={:02x?} 기대={:02x?} (패치버전 확인 필요)",
            addr, cur, HOOK_ORIG
        ));
    }
    let _ = tlen;
    TRAMPOLINE.store(tramp, Ordering::Release);

    // 진입부를 detour 로 교체: movabs rax, detour; jmp rax (+ 잔여 NOP)
    let mut patch = [0x90u8; ORIG_LEN];
    patch[..12].copy_from_slice(&jmp_abs(detour_candidates as usize));
    let mut old: u32 = 0;
    if VirtualProtect(addr, ORIG_LEN, PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        return Err("VirtualProtect 실패".into());
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), addr as *mut u8, ORIG_LEN);
    let mut old2: u32 = 0;
    VirtualProtect(addr, ORIG_LEN, old, &mut old2);
    FlushInstructionCache(GetCurrentProcess(), addr, ORIG_LEN);

    // write 후 재read 검증
    let mut landed = [0u8; ORIG_LEN];
    core::ptr::copy_nonoverlapping(addr as *const u8, landed.as_mut_ptr(), ORIG_LEN);
    if landed == patch {
        Ok(format!("설치+VERIFIED @abs=0x{:x} (rva=0x{:x}) 모드={} tramp=0x{:x}", addr, HOOK_RVA, mode, tramp))
    } else {
        Err(format!("write 미반영 @abs=0x{:x} landed={:02x?}", addr, landed))
    }
}

#[inline]
fn jmp_abs(target: usize) -> [u8; 12] {
    let mut b = [0u8; 12];
    b[0] = 0x48; b[1] = 0xB8; // movabs rax, imm64
    b[2..10].copy_from_slice(&target.to_le_bytes());
    b[10] = 0xFF; b[11] = 0xE0; // jmp rax
    b
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    // src=file!() — build_inj.ps1 신원 검증(dll 내 소스 절대경로 문자열) 요구
    log(&format!("mod init v0.1.0 (src={})", file!()));
    let (excl, block_all) = load_exclude_cfg();
    log(&format!("cfg: 제외 {}개 = [{}]{}", excl.len(), excl.join(", "),
        if block_all { " + 전면차단(*)" } else { "" }));
    let r = catch_unwind(AssertUnwindSafe(|| unsafe { install_hook() }));
    match r {
        Ok(Ok(m)) => log(&format!("HOOK OK: {}", m)),
        Ok(Err(e)) => log(&format!("HOOK FAIL: {}", e)),
        Err(_) => log("HOOK FAIL: panic in install_hook"),
    }
    ModRegistration::new(MOD_ID)
}
declare_mod!(init);
