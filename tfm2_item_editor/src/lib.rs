//! tfm2_item_editor — 아이템 정보(특히 가격/돈) 런타임 변경 모드
//! ===========================================================================
//! 동작 원리:
//!   - 게임이 메모리에 올려둔 ItemSetting 레지스트리(바닐라 30 + 모드 아이템)를
//!     직접 찾아, 각 아이템 element 의 price(돈) 필드를 런타임에 덮어쓴다.
//!   - 외부 JSON/번들 디코딩 0. 전부 "로딩된 메모리 영역"에서 동적으로 읽고 쓴다.
//!   - price 필드의 정확한 바이트 오프셋은 바닐라 배열의 구조적 시그니처
//!     (값이 tier(i%5)에만 의존 · >=100 · 단조증가)로 자가보정 탐지 → 게임 업데이트로
//!     구체 값이 바뀌어도 구조만 같으면 동작.
//!
//! Stage 1 (이 파일): 서버확장에서 db 잡고 → 레지스트리 탐색 → price 오프셋 자동탐지
//!   → item_overrides.txt(key=price) 적용. 진단 = item_editor_probe.txt.
//!   ※ 입력(config)은 임시. Stage 2 에서 인게임 UI 편집기로 교체 예정.
//!
//! 안전: 모든 메모리 접근은 SEH(VEH) 로 보호 → 틀린 주소 만져도 게임 안 멈춤.
//! ===========================================================================
#![allow(dead_code, unused_imports, unused_variables)]
use mod_api::*;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use std::rc::Rc;

#[path = "C:/tfm2mods/ui_kit/ui_kit.rs"]
mod ui_kit; // 인게임 UI 편집기 (클릭 라우팅/가시성/라벨)

const MOD_ID: &str = "tfm2_item_editor";

// Database 내 champion_patch_statistics 필드 오프셋 (확정 RE). db_base = 필드주소 − 이 값.
const O_CPS: usize = 0x16698;
// 바닐라 아이템 배열 시작 (db 상대). 실패 시 스캔 폴백.
const O_VANILLA: usize = 0x12d50;
// element stride 후보 (ModItemEntry/BaseItemInfo). 자동탐지.
const ITEM_STRIDES: [usize; 3] = [0x1a8, 0x198, 0x1b0];

// ───────────────────────── WinAPI ─────────────────────────
type HMODULE = isize; type DWORD = u32; type BOOL = i32;
const PAGE_READWRITE: u32 = 0x04;
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleExW(f: DWORD, name: *const u16, h: *mut HMODULE) -> BOOL;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, n: DWORD) -> DWORD;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old_protect: *mut u32) -> BOOL;
    fn AddVectoredExceptionHandler(first: u32, handler: VehHandler) -> usize;
    fn GetCurrentThreadId() -> u32;
    fn GetModuleHandleW(name: *const u16) -> usize; // null=exe base (RVA 산출)
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize; // 트램폴린 스텁용
    fn FlushInstructionCache(proc: usize, addr: usize, size: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
}
#[link(name = "user32")]
extern "system" { fn GetAsyncKeyState(vk: i32) -> i16; }
#[repr(C)]
#[derive(Default)]
struct MemBasicInfo {
    base: usize, alloc_base: usize, alloc_protect: u32, _pad0: u32,
    region_size: usize, state: u32, protect: u32, mtype: u32, _pad1: u32,
}

// ───────────────────────── SEH 안전 r/w (scrim 검증본) ─────────────────────────
//  SEH[]: 0=active 1=tid 2=land_rip 3=land_rsp 4=land_rbp 5=code_lo 6=code_hi 7=faults
#[repr(C)]
struct ExceptionRecord { code: u32, flags: u32, rec: usize, addr: usize, nparams: u32, _p: u32, params: [usize; 15] }
#[repr(C)]
struct ExceptionPointers { rec: *mut ExceptionRecord, ctx: *mut core::ffi::c_void }
type VehHandler = extern "system" fn(*mut ExceptionPointers) -> i32;

static mut SEH: [u64; 8] = [0u64; 8];
static SEH_INSTALLED: AtomicBool = AtomicBool::new(false);
static SEH_BUSY: AtomicBool = AtomicBool::new(false);

// ── 전투 스탯 파이프라인 후킹 (FUN_141b860d0 최종계산) — 런타임 엔티티 이펙트엔트리 편집=세이브 무오염. ──
//   현재 로깅모드: entity+0x2c8 이펙트리스트 덤프 → 아이템 키 엔트리·스탯(+0x58) 레이아웃 런타임 확인.
const STAT_FN_RVA: usize = 0x1fc12b0;          // 0.5.0_3(구0.5.0_2=0x1d04700, UNIQUE PROL-OK) 유효스탯 최종계산 (rcx=entity, rdx=int)
static STAT_TRAMP: AtomicUsize = AtomicUsize::new(0);
static STAT_HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
static EFFECT_DUMPS: AtomicUsize = AtomicUsize::new(0);
static INJECT_LOGS: AtomicUsize = AtomicUsize::new(0); // 주입 실제 쓰기 로그 카운트(편집아이템만 → 캡 안 먹음)
static STATHOOK_LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());
static STAT_FLUSH_LEN: AtomicUsize = AtomicUsize::new(0);

extern "system" fn seh_veh(p: *mut ExceptionPointers) -> i32 {
    const CONTINUE_EXECUTION: i32 = -1;
    const CONTINUE_SEARCH: i32 = 0;
    unsafe {
        if p.is_null() { return CONTINUE_SEARCH; }
        let rec = (*p).rec;
        if rec.is_null() || (*rec).code != 0xC0000005 { return CONTINUE_SEARCH; }
        let g = core::ptr::addr_of!(SEH) as *const u64;
        if *g.add(0) == 0 { return CONTINUE_SEARCH; }
        if *g.add(1) != GetCurrentThreadId() as u64 { return CONTINUE_SEARCH; }
        let ctx = (*p).ctx as usize;
        if ctx == 0 { return CONTINUE_SEARCH; }
        let rip = *((ctx + 0xF8) as *const u64);
        if rip < *g.add(5) || rip >= *g.add(6) { return CONTINUE_SEARCH; }
        *((ctx + 0xF8) as *mut u64) = *g.add(2);
        *((ctx + 0x98) as *mut u64) = *g.add(3);
        *((ctx + 0xA0) as *mut u64) = *g.add(4);
        let gm = core::ptr::addr_of_mut!(SEH) as *mut u64;
        *gm.add(7) += 1;
        CONTINUE_EXECUTION
    }
}
fn seh_install() {
    if SEH_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    unsafe { AddVectoredExceptionHandler(1, seh_veh); }
}
#[inline(never)]
unsafe fn safe_copy(dst: *mut u8, src: *const u8, len: usize) -> bool {
    if !SEH_INSTALLED.load(Ordering::Relaxed) { return false; }
    while SEH_BUSY.swap(true, Ordering::Acquire) { core::hint::spin_loop(); }
    let g = core::ptr::addr_of_mut!(SEH) as *mut u64;
    *g.add(1) = GetCurrentThreadId() as u64;
    let ok: u64;
    core::arch::asm!(
        "lea rax, [rip + 200f]",
        "mov [{g} + 40], rax",
        "lea rax, [rip + 201f]",
        "mov [{g} + 48], rax",
        "lea rax, [rip + 202f]",
        "mov [{g} + 16], rax",
        "mov [{g} + 24], rsp",
        "mov [{g} + 32], rbp",
        "mov qword ptr [{g} + 0], 1",
        "cld",
        "200:",
        "rep movsb",
        "201:",
        "mov {ok}, 1",
        "jmp 203f",
        "202:",
        "mov {ok}, 0",
        "203:",
        "mov qword ptr [{g} + 0], 0",
        g = in(reg) g,
        ok = out(reg) ok,
        inout("rcx") len => _,
        inout("rdi") dst => _,
        inout("rsi") src => _,
        out("rax") _,
    );
    SEH_BUSY.store(false, Ordering::Release);
    ok != 0
}
unsafe fn safe_read_u64(addr: usize) -> Option<u64> {
    let mut b = [0u8; 8];
    if safe_copy(b.as_mut_ptr(), addr as *const u8, 8) { Some(u64::from_le_bytes(b)) } else { None }
}
unsafe fn safe_read_i32(addr: usize) -> Option<i32> {
    let mut b = [0u8; 4];
    if safe_copy(b.as_mut_ptr(), addr as *const u8, 4) { Some(i32::from_le_bytes(b)) } else { None }
}
unsafe fn safe_read_f32(addr: usize) -> Option<f32> {
    let mut b = [0u8; 4];
    if safe_copy(b.as_mut_ptr(), addr as *const u8, 4) { Some(f32::from_le_bytes(b)) } else { None }
}
unsafe fn safe_read_bytes(addr: usize, len: usize, out: &mut Vec<u8>) -> bool {
    if len == 0 || len > 4096 { return false; }
    out.clear(); out.resize(len, 0);
    safe_copy(out.as_mut_ptr(), addr as *const u8, len)
}
// 게임 메모리에 data 쓰기. 페이지를 RW 로 보장한 뒤 safe_copy. 폴트나면 false.
unsafe fn safe_write_bytes(addr: usize, data: &[u8]) -> bool {
    if data.is_empty() || data.len() > 4096 || addr < 0x10000 { return false; }
    let mut old = 0u32;
    let changed = VirtualProtect(addr, data.len(), PAGE_READWRITE, &mut old) != 0;
    let ok = safe_copy(addr as *mut u8, data.as_ptr(), data.len());
    if changed { let mut o2 = 0u32; VirtualProtect(addr, data.len(), old, &mut o2); }
    ok
}
unsafe fn safe_write_i32(addr: usize, v: i32) -> bool { safe_write_bytes(addr, &v.to_le_bytes()) }

fn looks_heap(v: u64) -> bool { v & 0x7 == 0 && v >= 0x10000 && v < 0x0000_8000_0000_0000 && (v & 0xffff) != 0 }
// 아이템이 아닌 스탯/필드 이름이 key 로 잡히는 구조 제외 (예: "attack","defence" 라는 키의 stat 구조).
fn is_stat_name(k: &str) -> bool {
    matches!(k, "attack" | "defence" | "hp" | "magic_power" | "magic_resistance" | "move_speed"
        | "hp_regen" | "stack" | "crit_chance" | "attack_mult" | "magic_power_mult" | "defence_mult"
        | "hp_mult" | "vamp" | "range" | "toughness" | "duration" | "name" | "attack_speed_mult"
        | "move_speed_mult" | "skill_cooldown_mult" | "ult_cooldown_mult" | "adaptive_force")
}

// ───────────────────────── 파일/로그 ─────────────────────────
fn dll_path() -> Option<PathBuf> {
    unsafe {
        let addr = dll_path as *const () as usize;
        let mut h: HMODULE = 0;
        // FROM_ADDRESS(0x4) | UNCHANGED_REFCOUNT(0x2)
        if GetModuleHandleExW(0x4 | 0x2, addr as *const u16, &mut h) == 0 || h == 0 { return None; }
        let mut buf = [0u16; 4096];
        let n = GetModuleFileNameW(h, buf.as_mut_ptr(), buf.len() as DWORD);
        if n == 0 { return None; }
        Some(PathBuf::from(String::from_utf16_lossy(&buf[..n as usize])))
    }
}
fn mod_dir() -> Option<PathBuf> { dll_path()?.parent().map(|p| p.to_path_buf()) }
const LOG_ENABLED: bool = false; // ★ 배포=false: 디버그 파일출력 전부 끔. 디버그시 true 로 재빌드.
fn write_log(name: &str, content: &str) {
    if !LOG_ENABLED { return; }
    if let Some(p) = mod_dir().map(|d| d.join(name)) { let _ = fs::write(p, content); }
}
fn read_text(name: &str) -> Option<String> {
    fs::read_to_string(mod_dir()?.join(name)).ok()
}
fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }

// ───────────────────────── 아이템 레지스트리 탐색 ─────────────────────────
// element 의 key (String len-field 정확 읽기). 아이템 key 가 아니면 None.
unsafe fn key_of_elem(elem: usize) -> Option<String> {
    let a = safe_read_u64(elem)? as usize;
    let ptr = safe_read_u64(elem + 8)? as usize;
    let c = safe_read_u64(elem + 0x10)? as usize;
    let len = a.min(c);
    if ptr <= 0x10000 || len < 2 || len > 48 { return None; }
    let mut b = Vec::new();
    if !safe_read_bytes(ptr, len, &mut b) { return None; }
    if b.iter().all(|&x| x == b'_' || x.is_ascii_alphanumeric()) && (b[0] as char).is_ascii_alphabetic() {
        String::from_utf8(b).ok()
    } else { None }
}
const VANILLA_FIRST3: [&str; 3] = ["ironsword", "soldiers_longsword", "ruinous_blade"];
// 바닐라 배열 (base, stride) 탐지: ironsword→soldiers_longsword→ruinous_blade 연속.
unsafe fn find_vanilla(db: usize) -> Option<(usize, usize)> {
    let try_base = |base: usize| -> Option<usize> {
        for &st in ITEM_STRIDES.iter() {
            if (0..3).all(|i| key_of_elem(base + i * st).as_deref() == Some(VANILLA_FIRST3[i])) {
                return Some(st);
            }
        }
        None
    };
    if let Some(st) = try_base(db + O_VANILLA) { return Some((db + O_VANILLA, st)); }
    let mut o = 0usize;
    while o < 0x60000 {
        if let Some(st) = try_base(db + o) { return Some((db + o, st)); }
        o += 8;
    }
    None
}
// 아이템 element 판별: 키 + tier∈0..=4 + price∈[1,2_000_000]. athlete(선수)/비아이템 배제.
//  (price@+0x180, tier@+0x188 = vanilla/mod 동일 → VPRICE_OFF/VTIER_OFF 사용)
unsafe fn is_item_elem(addr: usize) -> bool {
    if key_of_elem(addr).is_none() { return false; }
    let voff = VPRICE_OFF.load(Ordering::Relaxed);
    let toff = VTIER_OFF.load(Ordering::Relaxed);
    if voff == 0 || toff == 0 { return true; } // 오프셋 미탐지 폴백(키만 — 모달 열릴땐 이미 탐지됨)
    let t = safe_read_i32(addr + toff).unwrap_or(-1);
    let p = safe_read_i32(addr + voff).unwrap_or(-1);
    (0..=4).contains(&t) && (1..=2_000_000).contains(&p)
}

// mod_items 버퍼 (buf, stride, cnt) 탐지: db+0..0x60000 의 (ptr,cnt) 트리플에서
//  ptr=힙 & 비바닐라 item-struct 배열(연속 유효 item element)인 곳.
unsafe fn find_mod_items(db: usize) -> Option<(usize, usize, usize)> {
    let is_vanilla = |k: &str| VANILLA_FIRST3.contains(&k) || k == "iron_blade";
    let detect_stride = |buf: usize| -> usize {
        for &st in ITEM_STRIDES.iter() {
            let k: Vec<Option<String>> = (0..4).map(|i| key_of_elem(buf + i * st)).collect();
            if k.iter().all(|x| x.is_some()) && k[0] != k[1] && k[1] != k[2] && k[2] != k[3] { return st; }
        }
        0
    };
    let mut o = 0usize;
    while o + 0x18 <= 0x60000 {
        let a = db + o; o += 8;
        let (Some(q0), Some(q1), Some(q2)) = (safe_read_u64(a), safe_read_u64(a + 8), safe_read_u64(a + 0x10)) else { continue; };
        for &(p, c) in [(q1, q0), (q1, q2), (q0, q2), (q0, q1)].iter() {
            let (p, c) = (p as usize, c as usize);
            if !looks_heap(p as u64) || c < 3 || c > 2000 { continue; }
            let Some(k0) = key_of_elem(p) else { continue; };
            if is_vanilla(&k0) { continue; }
            if !is_item_elem(p) { continue; } // ★ athlete(선수) Vec 오탐 배제: 첫 원소가 아이템(tier/price)이어야
            let st = detect_stride(p);
            if st == 0 { continue; }
            let probe = c.min(48);
            let valid = (0..probe).filter(|&i| is_item_elem(p + i * st)).count(); // ★ key 만이 아니라 아이템성 검증
            if valid * 10 < probe * 8 || valid < 3 { continue; }
            // 실제 개수 = idx0부터 연속 유효 아이템 element
            let mut cnt = 0usize;
            while cnt < c.max(1) && cnt < 500 { if is_item_elem(p + cnt * st) { cnt += 1; } else { break; } }
            return Some((p, st, cnt));
        }
    }
    None
}
// 전체 레지스트리: (key, elem_addr, is_vanilla). 바닐라(영역스캔 전부) + 모드 아이템(Vec).
unsafe fn build_registry(db: usize) -> Vec<(String, usize, bool)> {
    let mut out: Vec<(String, usize, bool)> = Vec::new();
    for (k, addr) in find_all_vanilla(db) { out.push((k, addr, true)); }
    if let Some((buf, st, cnt)) = find_mod_items(db) {
        let poff = price_off_for(false); // mod 가격 오프셋 (폴백 포함)
        for i in 0..cnt {
            let addr = buf + i * st;
            if let Some(k) = key_of_elem(addr) {
                // 더미/테스트 아이템 제외: 센티넬 가격(예: spectator_chat_probe=99999). 정상템 ≤ ~2000.
                let p = if poff != 0 { safe_read_i32(addr + poff).unwrap_or(-1) } else { -1 };
                if p >= 50000 { continue; }
                out.push((k, addr, false));
            }
        }
    }
    out
}

// ───────────────────────── price 오프셋 자가보정 탐지 ─────────────────────────
// 바닐라 배열로 구조 시그니처 탐지. 바닐라 ID 규약: 카테고리마다 5티어, idx i 의 tier = i%5.
//  - tier 필드: i32 값이 i%5 와 정확히 일치(0..4).
//  - price 필드: i32 값이 tier(i%5)에만 의존(같은 tier=같은 값) · 전부 >=100 · 단조증가 ·
//                최고 >=1000 · tier 따라 "변함"(상수 70000 류 오탐 배제).
//  ※ n 은 base 에서 연속으로 유효한 element 개수(이 게임에선 한 base 에 15개만 연속) 만큼만 사용.
//  ※ vanilla(0x198) 와 mod(0x1a8) 는 크기 다른 구조체 → price 오프셋 별도(VPRICE/MPRICE).
static VPRICE_OFF: AtomicUsize = AtomicUsize::new(0);
static VTIER_OFF: AtomicUsize = AtomicUsize::new(0);
static MPRICE_OFF: AtomicUsize = AtomicUsize::new(0);
// 스탯 오프셋 (공격력/주문력/방어력/체력/마저). element 기준, vanilla=mod 공유. 0=미검출.
static STAT_OFFS: Mutex<[usize; 5]> = Mutex::new([0; 5]);
// 스탯 탐지 정의: (라벨, 바닐라 5키, 기대값 5). 알려진 바닐라 스탯값으로 오프셋 식별.
fn stat_defs() -> [(&'static str, [&'static str; 5], [i32; 5]); 5] {
    [
        ("공격력", ["ironsword", "soldiers_longsword", "ruinous_blade", "conquerors_greatsword", "warlords_final_judgement"], [15, 30, 50, 75, 100]),
        ("주문력", ["arcane_crystal", "spirit_crystal", "staff_of_rapture", "angels_fang", "prophet_of_the_abyss"], [30, 60, 100, 150, 200]),
        ("방어력", ["steel_armor", "gatekeepers_armor", "black_knights_heavy_plate", "eternal_iron_plate", "impregnable_fortress"], [20, 40, 70, 100, 130]),
        ("체력", ["vital_orb", "hardened_heart", "ring_of_reincarnation", "hourglass_of_eternity", "giants_horn_shard"], [100, 200, 300, 500, 800]),
        ("마저", ["mystic_cloak", "night_hood", "dusk_raven", "souls_edge", "veil_of_annihilation"], [30, 60, 100, 150, 200]),
    ]
}
// 각 스탯의 element 오프셋 탐지: 5개 바닐라키 주소에서 기대값이 모두 일치하는 i32 오프셋.
unsafe fn detect_stat_offsets(db: usize) -> String {
    let reg = build_registry(db);
    let addr_of = |k: &str| reg.iter().find(|(kk, _, _)| kk == k).map(|(_, a, _)| *a);
    let mut offs = [0usize; 5];
    let mut diag = String::from("=== 스탯 오프셋 탐지 ===\n");
    for (si, (name, keys, vals)) in stat_defs().iter().enumerate() {
        let addrs: Vec<Option<usize>> = keys.iter().map(|k| addr_of(k)).collect();
        if addrs.iter().any(|a| a.is_none()) { diag.push_str(&format!("  {} : 키주소 일부 없음\n", name)); continue; }
        let addrs: Vec<usize> = addrs.into_iter().map(|a| a.unwrap()).collect();
        let mut off = 0x40usize;
        while off + 4 <= 0x180 {
            if (0..5).all(|i| safe_read_i32(addrs[i] + off) == Some(vals[i])) { offs[si] = off; break; }
            off += 4;
        }
        diag.push_str(&format!("  {} → +{:#x}\n", name, offs[si]));
    }
    // ★ 등간격 복원: 스탯은 8바이트 등간격(attack,magic,def,hp,mr = base+i*span).
    //   일부 아이템을 편집하면 그 카테고리 지문이 깨져 탐지 실패함 → 탐지된 2개+로 base/span 복원해 실패분 채움.
    let det: Vec<(usize, usize)> = offs.iter().enumerate().filter(|(_, &o)| o != 0).map(|(i, &o)| (i, o)).collect();
    if det.len() >= 2 {
        let (i0, o0) = det[0];
        let (i1, o1) = det[det.len() - 1];
        if i1 != i0 {
            let span = (o1 as isize - o0 as isize) / (i1 as isize - i0 as isize);
            if span > 0 {
                let base = o0 as isize - i0 as isize * span;
                for i in 0..5 {
                    if offs[i] == 0 {
                        let cand = base + i as isize * span;
                        if cand > 0 { offs[i] = cand as usize; diag.push_str(&format!("  [복원] stat[{}] → +{:#x} (base={:#x} span={})\n", i, cand, base, span)); }
                    }
                }
            }
        }
    }
    *STAT_OFFS.lock().unwrap() = offs;
    diag
}
// base 에서 연속 유효 element 개수 (key_of_elem 성공이 끊길 때까지, 최대 30)
unsafe fn contiguous_run(base: usize, st: usize, cap: usize) -> usize {
    let mut r = 0usize;
    while r < cap { if key_of_elem(base + r * st).is_some() { r += 1; } else { break; } }
    r
}
unsafe fn detect_vanilla_offsets(db: usize) -> (usize, usize, String) {
    let mut diag = String::new();
    let Some((base, st)) = find_vanilla(db) else {
        return (0, 0, "바닐라 배열 못 찾음\n".into());
    };
    let n = contiguous_run(base, st, 30).max(5).min(30);
    diag.push_str(&format!("바닐라 base={:#x} stride={:#x} 연속 element={}\n", base, st, n));
    let mut price_off = 0usize;
    let mut tier_off = 0usize;
    let mut off = 0x18usize; // key(0x0) 이후부터
    while off + 4 <= st {
        let vals: Vec<Option<i32>> = (0..n).map(|i| safe_read_i32(base + i * st + off)).collect();
        if vals.iter().all(|v| v.is_some()) {
            let v: Vec<i32> = vals.iter().map(|o| o.unwrap()).collect();
            let is_tier = (0..n).all(|i| v[i] == (i % 5) as i32);
            // tier(i%5)에만 의존?
            let mut by_tier = [i32::MIN; 5];
            let mut dep_only_tier = true;
            for i in 0..n {
                let t = i % 5;
                if by_tier[t] == i32::MIN { by_tier[t] = v[i]; }
                else if by_tier[t] != v[i] { dep_only_tier = false; break; }
            }
            let seen: Vec<i32> = by_tier.iter().copied().filter(|&x| x != i32::MIN).collect();
            let distinct = { let mut s = seen.clone(); s.sort_unstable(); s.dedup(); s.len() };
            let is_price = dep_only_tier
                && seen.iter().all(|&x| x >= 100)
                && seen.windows(2).all(|w| w[0] <= w[1])
                && *seen.iter().max().unwrap_or(&0) >= 1000
                && distinct >= 2; // tier 따라 변해야 함 (상수 배제)
            if is_tier && tier_off == 0 { tier_off = off; }
            if is_price && price_off == 0 {
                price_off = off;
                diag.push_str(&format!("price 후보 @+{:#x}: tier별 값 {:?}\n", off, by_tier));
            }
        }
        off += 4;
    }
    // 폴백: price 미검출 & tier 검출 시 → 구조상 price = tier-8 (0x180 vs 0x188 인접 i32) 앵커.
    //   (카테고리별 가격이 달라 dep_only_tier 가 깨져도 동작. tier 검출은 i%5 라 안정적.)
    if price_off == 0 && tier_off >= 8 {
        let cand = tier_off - 8;
        let vals: Vec<i32> = (0..n).map(|i| safe_read_i32(base + i * st + cand).unwrap_or(-999999)).collect();
        let good = vals.iter().filter(|&&x| (1..=2_000_000).contains(&x)).count();
        diag.push_str(&format!("price 폴백후보 @+{:#x} good={}/{} vals={:?}\n", cand, good, n, &vals[..n.min(15)]));
        if good * 10 >= n * 7 { // 70%+ sane 이면 채택 (read 1~2개 실패해도)
            price_off = cand;
        }
    }
    diag.push_str(&format!("→ 탐지결과: VPRICE_OFF=+{:#x}  VTIER_OFF=+{:#x}\n", price_off, tier_off));
    (price_off, tier_off, diag)
}
// 모드 배열에서 price 오프셋 탐지 (mod 구조체는 0x1a8 로 vanilla 와 다름).
//  시그니처: 앞 m개 mod 아이템에서 값이 전부 [1,2_000_000] & 5의 배수 & 서로다른 값 3+.
//  vanilla 와 같은 오프셋이면 즉시 채택(이 게임에선 둘 다 +0x180).
unsafe fn detect_mod_price(db: usize) -> usize {
    let Some((buf, st, cnt)) = find_mod_items(db) else { return 0; };
    let m = cnt.min(16).max(1);
    let vprice = VPRICE_OFF.load(Ordering::Relaxed);
    let mut best = 0usize;
    let mut off = 0x18usize;
    while off + 4 <= st {
        let nums: Vec<i32> = (0..m).filter_map(|i| safe_read_i32(buf + i * st + off)).collect();
        if nums.len() == m && nums.iter().all(|&x| (1..=2_000_000).contains(&x) && x % 5 == 0) {
            let mut d = nums.clone(); d.sort_unstable(); d.dedup();
            if d.len() >= 3 {
                if off == vprice { return off; }
                if best == 0 { best = off; }
            }
        }
        off += 4;
    }
    best
}
// 바닐라 30개를 영역스캔으로 전부 찾기 (한 base 에 연속 15개뿐이라 stride 가정 못 함).
//  db+0x12000..0x16800 을 8바이트씩: key_of_elem 성공 & tier∈0..=4 & price∈[1,2_000_000] 이면 아이템 element.
unsafe fn find_all_vanilla(db: usize) -> Vec<(String, usize)> {
    let voff = VPRICE_OFF.load(Ordering::Relaxed);
    let toff = VTIER_OFF.load(Ordering::Relaxed);
    let mut out: Vec<(String, usize)> = Vec::new();
    if voff == 0 || toff == 0 { return out; }
    let mut a = db + 0x12000;
    let end = db + 0x16800;
    while a < end {
        if let Some(k) = key_of_elem(a) {
            let t = safe_read_i32(a + toff).unwrap_or(-1);
            let p = safe_read_i32(a + voff).unwrap_or(-1);
            // 티어라벨(icon) "t<digit>_<digit>" 형식 제외 (실제 아이템 아님; 예 t5_2)
            let is_tier_label = k.len() <= 5 && k.as_bytes()[0] == b't'
                && k.as_bytes().get(1).map_or(false, |c| c.is_ascii_digit());
            if !is_tier_label && !is_stat_name(&k) && (0..=4).contains(&t) && (1..=2_000_000).contains(&p) {
                if !out.iter().any(|(ek, _)| *ek == k) { out.push((k, a)); }
            }
        }
        a += 8;
    }
    out
}

// ───────────────────────── 영속 편집 저장/적용 ─────────────────────────
// item_edits.txt: "key field value" 줄들. field ∈ price/attack/magic_power/defence/hp/magic_resistance.
//   에디터 확인 시 여기 기록 → 로드(서버시작/관리틱)마다 재적용 → 게임 재시작 후에도 편집 유지.
static EDITS: Mutex<Vec<(String, String, i32)>> = Mutex::new(Vec::new());
static EDITS_LOADED: AtomicBool = AtomicBool::new(false);

// 필드명 → kind (usize::MAX=price(van별 오프셋), else STAT_OFFS 인덱스 0~4)
fn field_kind(name: &str) -> Option<usize> {
    match name {
        "price" => Some(usize::MAX),
        "attack" => Some(0), "magic_power" => Some(1), "defence" => Some(2),
        "hp" => Some(3), "magic_resistance" => Some(4),
        _ => None,
    }
}
fn load_edits() {
    if EDITS_LOADED.swap(true, Ordering::Relaxed) { return; }
    let Some(txt) = read_text("item_edits.txt") else { return; };
    let mut v = EDITS.lock().unwrap();
    for line in txt.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let p: Vec<&str> = line.split_whitespace().collect();
        if p.len() == 3 { if let Ok(val) = p[2].parse::<i32>() { v.push((p[0].to_string(), p[1].to_string(), val)); } }
    }
}
fn save_edits() {
    let v = EDITS.lock().unwrap_or_else(|e| e.into_inner());
    let mut s = String::from("# tfm2_item_editor 저장 편집 (key field value). 로드시 자동 재적용. 지우면 원복.\n");
    for (k, f, val) in v.iter() { s.push_str(&format!("{} {} {}\n", k, f, val)); }
    // ★ 기능 영속(편집 저장) — write_log(디버그 게이트)와 분리해 직접 쓴다. LOG_ENABLED 무관.
    if let Some(p) = mod_dir().map(|d| d.join("item_edits.txt")) { let _ = fs::write(p, s); }
}
fn upsert_edit(key: &str, field: &str, value: i32) {
    let mut v = EDITS.lock().unwrap();
    if let Some(e) = v.iter_mut().find(|(k, f, _)| k == key && f == field) { e.2 = value; }
    else { v.push((key.to_string(), field.to_string(), value)); }
}
// van 무관 오프셋 (vanilla·mod 둘 다 price=0x180, stat=STAT_OFFS 동일).
fn field_off_nv(kind: usize) -> usize {
    if kind == usize::MAX {
        let v = VPRICE_OFF.load(Ordering::Relaxed);
        if v != 0 { v } else { MPRICE_OFF.load(Ordering::Relaxed) }
    } else { STAT_OFFS.lock().unwrap().get(kind).copied().unwrap_or(0) }
}
// db 영역(+0x10000..0x60000) 의 모든 아이템 element 수집 (copy 포함). tier·price sane 으로 판별.
unsafe fn find_all_item_addrs(db: usize) -> Vec<(String, usize)> {
    let vt = VTIER_OFF.load(Ordering::Relaxed);
    let vp = field_off_nv(usize::MAX);
    let mut out = Vec::new();
    if vt == 0 || vp == 0 { return out; }
    let mut a = db + 0x10000;
    let end = db + 0x60000;
    while a < end {
        if let Some(k) = key_of_elem(a) {
            let t = safe_read_i32(a + vt).unwrap_or(-1);
            let p = safe_read_i32(a + vp).unwrap_or(-1);
            if (0..=6).contains(&t) && (1..=2_000_000).contains(&p) {
                // 티어라벨 t\d 제외
                let lbl = k.len() <= 5 && k.as_bytes()[0] == b't' && k.as_bytes().get(1).map_or(false, |c| c.is_ascii_digit());
                if !lbl && !is_stat_name(&k) { out.push((k, a)); }
            }
        }
        a += 8;
    }
    out
}
// 저장된 편집 전부 메모리에 재적용 — db영역 모든 copy + mod Vec 에 다 씀.
unsafe fn apply_edits(db: usize) -> String {
    load_edits();
    let edits = { EDITS.lock().unwrap().clone() };
    if edits.is_empty() { return "  편집 없음 (item_edits.txt)\n".into(); }
    // 모든 copy 주소 수집
    let mut copies = find_all_item_addrs(db); // db영역 vanilla+copy
    if let Some((buf, st, cnt)) = find_mod_items(db) {
        for i in 0..cnt { if let Some(k) = key_of_elem(buf + i * st) { copies.push((k, buf + i * st)); } }
    }
    let mut s = format!("  copy 스캔: db영역 아이템 element {}개\n", copies.len());
    for (key, field, val) in &edits {
        // ★ 세이브 무오염: 스탯은 Database 에 안 씀(이펙트 런타임 주입=inject_effects 가 전담). 가격만 Database(경제).
        if field != "price" { continue; }
        let Some(kind) = field_kind(field) else { continue; };
        let off = field_off_nv(kind);
        if off == 0 { s.push_str(&format!("  {} {} ✗ 오프셋미확정\n", key, field)); continue; }
        let mut n = 0;
        for (k, addr) in copies.iter() {
            if k == key { if safe_write_i32(addr + off, *val) { n += 1; } }
        }
        s.push_str(&format!("  {} {} = {} → {}개 copy 적용\n", key, field, val, n));
    }
    s
}

// ───────────────────────── 진단 덤프 (1회) ─────────────────────────
static PROBE_DONE: AtomicBool = AtomicBool::new(false);
unsafe fn probe_once(db: usize) {
    if PROBE_DONE.load(Ordering::Relaxed) { return; }
    // 레지스트리(바닐라 배열)가 아직 메모리에 안 올라왔으면 done 처리하지 말고 다음 틱 재시도.
    if find_vanilla(db).is_none() { return; }
    PROBE_DONE.store(true, Ordering::Relaxed);
    let (vprice, vtier, ddiag) = detect_vanilla_offsets(db);
    VPRICE_OFF.store(vprice, Ordering::Relaxed);
    VTIER_OFF.store(vtier, Ordering::Relaxed);
    let mprice = detect_mod_price(db);
    MPRICE_OFF.store(mprice, Ordering::Relaxed);
    let stat_diag = detect_stat_offsets(db);

    let mut s = format!("[{}ms] tfm2_item_editor 진단 v2\n  db={:#x}\n", now_ms(), db);
    s.push_str(&ddiag);
    s.push_str(&format!("→ MPRICE_OFF=+{:#x} (mod 배열 탐지)\n", mprice));
    s.push_str(&stat_diag);
    s.push('\n');

    // 레지스트리 + 현재가격 표
    let reg = build_registry(db);
    let nv = reg.iter().filter(|(_, _, v)| *v).count();
    let nm = reg.len() - nv;
    s.push_str(&format!("=== 레지스트리 {}개 (바닐라 {} + 모드 {}) ===\n", reg.len(), nv, nm));
    s.push_str("  idx | key | Vprice@0x180 | tier | V/M\n");
    for (i, (k, elem, van)) in reg.iter().enumerate() {
        let p = if vprice != 0 { safe_read_i32(elem + vprice).unwrap_or(-1) } else { -1 };
        let t = if vtier != 0 { safe_read_i32(elem + vtier).unwrap_or(-1) } else { -1 };
        s.push_str(&format!("  {:>3} | {:32} | {:>6} | {:>2} | {}\n", i, k, p, t, if *van { "V" } else { "M" }));
    }

    // 모드 배열 앞 16개 i32 슬롯 덤프 (mod price 오프셋 식별용 — vanilla 와 구조 다름)
    if let Some((buf, st, cnt)) = find_mod_items(db) {
        let m = cnt.min(16);
        s.push_str(&format!("\n=== 모드 배열 앞{} i32 슬롯 (mod price 식별; buf={:#x} stride={:#x} cnt={}) ===\n", m, buf, st, cnt));
        let keys: Vec<String> = (0..m).map(|i| key_of_elem(buf + i * st).unwrap_or("?".into())).collect();
        s.push_str(&format!("  대상: {}\n", keys.iter().enumerate().map(|(i, k)| format!("{}:{}", i, k)).collect::<Vec<_>>().join(" ")));
        s.push_str("  off    | i32 (앞16 아이템)\n");
        let mut off = 0usize;
        while off + 4 <= st {
            let vi: Vec<String> = (0..m).map(|i| safe_read_i32(buf + i * st + off).map(|v| v.to_string()).unwrap_or("?".into())).collect();
            // mod 에서도 price 후보 자동 태그: 전부 [1,2_000_000] & 서로다른 값 3+ & 모두 5의 배수
            let nums: Vec<i32> = (0..m).filter_map(|i| safe_read_i32(buf + i * st + off)).collect();
            let mut tag = "";
            if nums.len() == m && nums.iter().all(|&x| (1..=2_000_000).contains(&x) && x % 5 == 0) {
                let mut d = nums.clone(); d.sort_unstable(); d.dedup();
                if d.len() >= 3 { tag = " ★mod-price?"; }
            }
            s.push_str(&format!("  +{:#05x} | {}{}\n", off, vi.join(","), tag));
            off += 4;
        }
    } else {
        s.push_str("\n(모드 배열 못 찾음 — 모드 아이템 미적용?)\n");
    }
    write_log("item_editor_probe.txt", &s);
}

// ───────────────────────── 서버 확장 ─────────────────────────
fn db_base(ctx: &mut ServerModContext) -> usize {
    let cps = &ctx.database.champion_patch_statistics as *const _ as usize;
    cps.wrapping_sub(O_CPS)
}
fn run(ctx: &mut ServerModContext, where_: &str) {
    seh_install();
    let db = db_base(ctx);
    DB_BASE.store(db, Ordering::Relaxed); // 클라이언트 UI 가 쓸 db base 공유
    unsafe {
        probe_once(db);
        let log = apply_edits(db); // find_all_item_addrs(0x10000..0x60000) = 아이템 copy 전부 커버(검증: ironsword 4copy 다 적용)
        write_log("item_editor_apply.txt", &format!("[{}ms] apply @{}\n  VPRICE=+{:#x} MPRICE=+{:#x}\n{}",
            now_ms(), where_, VPRICE_OFF.load(Ordering::Relaxed), MPRICE_OFF.load(Ordering::Relaxed), log));
    }
}
struct ItemEditorServerExt;
impl ModServerExtension for ItemEditorServerExt {
    fn on_server_start(&self, ctx: &mut ServerModContext) { run(ctx, "on_server_start"); }
    fn before_management_tick(&self, ctx: &mut ServerModContext) { run(ctx, "before_management_tick"); }
}

// ═══════════════════════════════════════════════════════════════════════════
//  인게임 UI 편집기 (클라이언트) — Stage 2a: 사이드바 버튼 → 모달 → 로딩된 아이템 목록 표시
//  서버확장이 채운 DB_BASE 로 레지스트리를 읽어 그 사람이 로딩한 아이템 전체를 동적으로 띄움.
// ═══════════════════════════════════════════════════════════════════════════
static DB_BASE: AtomicUsize = AtomicUsize::new(0);
static MODAL_OPEN: AtomicBool = AtomicBool::new(false);
static UI_REFRESH: AtomicBool = AtomicBool::new(true);
static SCROLL_OFF: AtomicUsize = AtomicUsize::new(0);
static LAST_FH: AtomicUsize = AtomicUsize::new(usize::MAX);
const UI_ROWS: usize = 90; // .ui 선언 행 수(스크롤). 실제 표시는 realN 만.
// 캐시 행: (key, is_vanilla, elem_addr, active, vals)  vals=[가격,공격,주문,방어,체력,마저]
type ItemRow = (String, bool, usize, bool, [i32; 6]);
static UI_REG: Mutex<Vec<ItemRow>> = Mutex::new(Vec::new());
static ACTIVE_ONLY: AtomicBool = AtomicBool::new(true); // true=활성아이템만 / false=전체
static ACTIVE_I18N: Mutex<Option<String>> = Mutex::new(None); // 활성모드 item.i18n 합본(캐시)
static SELECTED: AtomicUsize = AtomicUsize::new(usize::MAX); // 선택된 레지스트리 idx
static ROW_CLICKED: AtomicUsize = AtomicUsize::new(usize::MAX); // 이번 프레임 클릭된 화면행 i
static SCROLL_RESET: AtomicBool = AtomicBool::new(false); // 열 때 스크롤 맨위로 1회
static EDIT_OPEN: AtomicBool = AtomicBool::new(false); // 가격편집 팝업 열림
static EDIT_PREFILL: AtomicBool = AtomicBool::new(false); // 팝업 열 때 입력칸에 현재가 채우기(1회)
static CONFIRM_REQ: AtomicBool = AtomicBool::new(false); // 확인버튼 눌림
// ── 주입 모달(페이징/±편집) ──
const ROWS_PP: usize = 14;                 // 페이지당 행 수
static PAGE: AtomicUsize = AtomicUsize::new(0);
static PREV_REQ: AtomicBool = AtomicBool::new(false);
static NEXT_REQ: AtomicBool = AtomicBool::new(false);
// 필드별(가격/공/주/방/체/마저) 이번 프레임 누적 증감. +/- 버튼 클릭이 더함, 프레임에서 메모리 반영.
static PENDING: [AtomicI32; 6] = [const { AtomicI32::new(0) }; 6];

unsafe fn price_off_for(van: bool) -> usize {
    if van { VPRICE_OFF.load(Ordering::Relaxed) }
    else {
        // mod 가격도 element+0x180 (vanilla/mod 동일, 진단 확인). detect_mod_price 는 사다리패턴 의존이라
        // riot 처럼 비사다리 가격이면 실패(=0) → VPRICE_OFF 로 폴백.
        let m = MPRICE_OFF.load(Ordering::Relaxed);
        if m != 0 { m } else { VPRICE_OFF.load(Ordering::Relaxed) }
    }
}
// ── 활성 아이템 판단 (스크림 로직 이식): 활성 모드(mods.json enabled)의 text/item.i18n 에
//    키가 있으면 활성. 바닐라는 항상 활성. JSON 파서 없이 `"key"` 부분문자열로 판정(실용).
fn game_root() -> Option<PathBuf> { mod_dir()?.parent()?.parent().map(|p| p.to_path_buf()) }
fn read_enabled_mods() -> Vec<String> {
    let Some(root) = game_root() else { return vec![]; };
    let Ok(txt) = fs::read_to_string(root.join("config").join("game").join("mods.json")) else { return vec![]; };
    let mut out = vec![];
    if let Some(i) = txt.find("\"enabled_mods\"") {
        if let Some(lb) = txt[i..].find('[') {
            if let Some(rb) = txt[i + lb..].find(']') {
                for part in txt[i + lb + 1..i + lb + rb].split(',') {
                    let s = part.trim().trim_matches('"').trim();
                    if !s.is_empty() { out.push(s.to_string()); }
                }
            }
        }
    }
    out
}
fn extract_mod_id(info: &str) -> Option<String> {
    // ⚠ "dependencies":[{"mod_id":"base",..}] 안의 mod_id 를 잡으면 안 됨(첫 "mod_id"=의존성).
    //   dependencies 배열 블록을 잘라낸 뒤 최상위 mod_id 추출. (riot_items_tfm2 가 base 뒤에 옴)
    let cleaned: String = match info.find("\"dependencies\"") {
        Some(ds) => match info[ds..].find('[').and_then(|lb| info[ds + lb..].find(']').map(|rb| (lb, rb))) {
            Some((lb, rb)) => format!("{}{}", &info[..ds], &info[ds + lb + rb + 1..]),
            None => info.to_string(),
        },
        None => info.to_string(),
    };
    let i = cleaned.find("\"mod_id\"")?;
    let after = &cleaned[i + 8..];
    let c = after.find(':')?;
    let q1 = after[c..].find('"')? + c;
    let q2 = after[q1 + 1..].find('"')? + q1 + 1;
    Some(after[q1 + 1..q2].to_string())
}
fn build_active_i18n() -> String {
    let mut concat = String::new();
    let Some(root) = game_root() else { return concat; };
    let enabled = read_enabled_mods();
    if enabled.is_empty() { return concat; }
    let mut dirs: Vec<PathBuf> = vec![];
    if let Ok(rd) = fs::read_dir(root.join("mods")) { for e in rd.flatten() { dirs.push(e.path()); } }
    if let Some(ws) = root.parent().and_then(|p| p.parent()).map(|p| p.join("workshop").join("content").join("3009300")) {
        if let Ok(rd) = fs::read_dir(&ws) { for e in rd.flatten() { dirs.push(e.path()); } }
    }
    for d in dirs {
        let Ok(info) = fs::read_to_string(d.join("mod.mod_info")) else { continue; };
        let Some(mid) = extract_mod_id(&info) else { continue; };
        if !enabled.iter().any(|e| e == &mid) { continue; }
        if let Ok(i18n) = fs::read_to_string(d.join("text").join("item.i18n")) { concat.push_str(&i18n); concat.push('\n'); }
    }
    concat
}
fn active_i18n() -> String {
    { if let Some(s) = ACTIVE_I18N.lock().unwrap().as_ref() { return s.clone(); } }
    let s = build_active_i18n();
    *ACTIVE_I18N.lock().unwrap() = Some(s.clone());
    s
}
fn is_item_active(key: &str, van: bool, i18n: &str) -> bool {
    van || i18n.contains(&format!("\"{}\"", key))
}

// 아이템 이름 = 게임 i18n 참조 문자열. 라벨에 통째로 넣으면 LabelRunner 가 렌더시 현지화.
//   바닐라 + 활성 모드아이템은 asset/base/text/item 네임스페이스로 병합돼 해석됨(scrim 확인).
//   비활성 모드아이템은 안 풀려 '#asset…' 날것이라 호출측에서 빈 문자열로 폴백.
fn item_name_ref(key: &str) -> String { format!("#asset/base/text/item?{}.name", key) }
// 표시용 이름: 활성(van||active)=엔진 i18n 참조(현지화). 비활성=「[비활성] 코드명」 폴백(마커 겸 코드명).
//   ⚠ 활성 케이스는 반드시 순수 참조 1줄이어야 LabelRunner 가 현지화함(태그 섞으면 안 풀림).
fn name_label(key: &str, van: bool, active: bool) -> String {
    if van || active { item_name_ref(key) } else { format!("[비활성] {}", key) }
}
// 표 값칸: 음수=? / 0=빈칸 / 그외=숫자
fn cell_num(v: i32) -> String {
    if v < 0 { "?".into() } else if v == 0 { String::new() } else { v.to_string() }
}

// 스탯칸 인덱스(0..4) → item_edits.txt 필드명 (가격은 별도). vals[1+j] 순서와 일치.
const STAT_FIELD_NAMES: [&str; 5] = ["attack", "magic_power", "defence", "hp", "magic_resistance"];

// item_edits.txt(EDITS)에서 (key,field) 편집값 조회. 스탯은 Database 에 안 쓰므로 이게 표시/프리필 진실.
fn edit_val(key: &str, field: &str) -> Option<i32> {
    load_edits();
    EDITS.lock().unwrap().iter().find(|(k, f, _)| k == key && f == field).map(|(_, _, v)| *v)
}

unsafe fn rebuild_ui_reg() {
    let db = DB_BASE.load(Ordering::Relaxed);
    let i18n = active_i18n();
    let soff = *STAT_OFFS.lock().unwrap(); // [공격,주문,방어,체력,마저]
    load_edits();
    let edits = EDITS.lock().unwrap().clone(); // 스탯은 Database 에 안 쓰므로 EDITS 가 표시 진실(오버레이)
    let edit_of = |key: &str, field: &str| edits.iter().find(|(k, f, _)| k == key && f == field).map(|(_, _, v)| *v);
    let mut out: Vec<ItemRow> = Vec::new();
    if db != 0 {
        for (k, addr, van) in build_registry(db) {
            let mut vals = [0i32; 6]; // [가격,공격,주문,방어,체력,마저]
            let poff = price_off_for(van);
            vals[0] = if poff != 0 { safe_read_i32(addr + poff).unwrap_or(-1) } else { -1 };
            for j in 0..5 {
                let base = if soff[j] != 0 { safe_read_i32(addr + soff[j]).unwrap_or(0) } else { 0 };
                vals[1 + j] = edit_of(&k, STAT_FIELD_NAMES[j]).unwrap_or(base); // 편집값 우선
            }
            let active = is_item_active(&k, van, &i18n);
            out.push((k, van, addr, active, vals));
        }
    }
    *UI_REG.lock().unwrap() = out;
}

// 편집 팝업 6필드: (text_edit id, field명, stat_kind). field명 = item_edits.txt 저장키.
fn edit_fields() -> [(&'static str, &'static str, usize); 6] {
    [("ie_in_price", "price", usize::MAX), ("ie_in_atk", "attack", 0), ("ie_in_mag", "magic_power", 1),
     ("ie_in_def", "defence", 2), ("ie_in_hp", "hp", 3), ("ie_in_mr", "magic_resistance", 4)]
}
fn field_offset(kind: usize, van: bool) -> usize {
    if kind == usize::MAX { unsafe { price_off_for(van) } }
    else { STAT_OFFS.lock().unwrap().get(kind).copied().unwrap_or(0) }
}

// item_editor 편집필드 → 이펙트엔트리 +0x58 i32 인덱스. (런타임 확정: 공격=0, 주문력=2)
//   방어/체력/마저 인덱스 미확정 → None(주입 스킵). 확정되면 추가.
fn effect_stat_index(field: &str) -> Option<usize> {
    // ★확정(오름차순 편집 111/222/333/444/555 의 게임복사본 슬롯, i32 8바이트 간격):
    //   공격=0, 주문=2, 방어=4, 체력=6, 마저=8 (각 +0x58 + idx*4, 짝수 i32만 = i64 스탯)
    match field {
        "attack" => Some(0),
        "magic_power" => Some(2),
        "defence" => Some(4),
        "hp" => Some(6),
        "magic_resistance" => Some(8),
        _ => None,
    }
}
// entity+0x2c8 이펙트 엔트리들을 순회 → 편집한 아이템 키와 일치하면 +0x58[idx] 를 편집값으로 덮음.
//   원본 합산(FUN_141b860d0) 전에 호출 → 합산이 편집값을 봄. Database 무손상 = 세이브 무오염.
unsafe fn inject_effects(entity: usize) -> bool {
    let edits = { let g = EDITS.lock().unwrap_or_else(|e| e.into_inner()); if g.is_empty() { return false; } g.clone() }; // poison-safe(sim스레드)
    let mut wrote = false;
    let mut hdr = [0u64; 2];
    if !safe_copy(hdr.as_mut_ptr() as *mut u8, (entity + 0x2c8) as *const u8, 16) { return false; }
    let lptr = hdr[0] as usize; let cnt = hdr[1] as usize;
    if lptr <= 0x10000 || cnt == 0 || cnt > 256 { return false; }
    for i in 0..cnt {
        let e = lptr + i * 0x120;
        let mut nb = [0u8; 0x40];
        if !safe_copy(nb.as_mut_ptr(), e as *const u8, 0x40) { continue; }
        let nlen = u32::from_le_bytes([nb[0], nb[1], nb[2], nb[3]]) as usize;
        if nlen == 0 || nlen > 0x3c { continue; }
        let name = match core::str::from_utf8(&nb[4..4 + nlen]) { Ok(s) => s, Err(_) => continue };
        for (k, field, val) in edits.iter() {
            if k == name {
                if let Some(idx) = effect_stat_index(field) {
                    let a = e + 0x58 + idx * 4;
                    let old = safe_read_i32(a).unwrap_or(i32::MIN);
                    if safe_write_i32(a, *val) { // ★SEH 보호(stale 엔트리 포인터 세그폴트 방지). 과거 직접 *mut 쓰기였음.
                        wrote = true;
                        if old != *val && INJECT_LOGS.load(Ordering::Relaxed) < 30 {
                            INJECT_LOGS.fetch_add(1, Ordering::Relaxed);
                            STATHOOK_LOG.lock().unwrap_or_else(|e| e.into_inner()).push(format!("[{}ms] INJECT ent={:#x} '{}' {}[idx{}] {} → {}", now_ms(), entity, name, field, idx, old, *val));
                        }
                    }
                }
            }
        }
    }
    wrote
}
// ★ FUN_141ce0860 의 +0x3b0 아이템합산을 우리 5스탯만 재현 (inject 된 entry+0x58 반영).
//   매프레임 도는 최종계산(FUN_141ce05b0) 직전에 호출 → 전챔프 +0x3b0 가 편집값 반영 → +0x600 반영.
//   매핑(FUN_141ce0860 디컴 확인): entry+off → entity+0x3b0off, 합산은 effect 엔트리 전부.
unsafe fn recompute_sums(entity: usize) {
    let mut hdr = [0u64; 2];
    if !safe_copy(hdr.as_mut_ptr() as *mut u8, (entity + 0x2c8) as *const u8, 16) { return; }
    let lptr = hdr[0] as usize; let cnt = hdr[1] as usize;
    if lptr <= 0x10000 || cnt == 0 || cnt > 256 { return; }
    const MAP: [(usize, usize); 5] = [(0x58, 0x3b0), (0x60, 0x3b8), (0x68, 0x3c0), (0x70, 0x3c8), (0x78, 0x3d0)];
    for &(eo, po) in MAP.iter() {
        let mut sum: i32 = 0;
        for i in 0..cnt { if let Some(v) = safe_read_i32(lptr + i * 0x120 + eo) { sum = sum.wrapping_add(v); } }
        let _ = safe_write_i32(entity + po, sum);
    }
}
// FUN_141b860d0 detour: 원본 합산 전에 편집값 주입 → 전투가 편집값 봄. (+ 제한 로깅)
type StatFn = extern "win64" fn(usize, usize);
extern "win64" fn stat_detour(entity: usize, arg: usize) {
    // ★ 최종계산(FUN_141ce05b0)은 +0x3b0(아이템합 캐시)을 읽음 → 매프레임 여기서 inject + 재합산해야 전챔프 반영.
    // catch_unwind: 우리 코드의 어떤 패닉(락 poison unwrap/OOB 등)도 게임 콜스택(extern win64)으로 unwind=UB 되지 않게 차단.
    if entity > 0x10000 {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { if inject_effects(entity) { recompute_sums(entity); } }));
    }
    let t = STAT_TRAMP.load(Ordering::Relaxed);
    if t != 0 { let orig: StatFn = unsafe { core::mem::transmute(t) }; orig(entity, arg); }
    if LOG_ENABLED && entity > 0x10000 && EFFECT_DUMPS.load(Ordering::Relaxed) < 24 {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { dump_effect_list(entity); }));
    }
}
// entity+0x2c8 이펙트 리스트(엔트리 0x120B: 이름len@+0x0, 이름@+0x4 인라인, 스탯@+0x58) 안전 덤프.
unsafe fn dump_effect_list(entity: usize) {
    let mut hdr = [0u64; 4];
    if !safe_copy(hdr.as_mut_ptr() as *mut u8, (entity + 0x2c8) as *const u8, 32) { return; }
    let lptr = hdr[0] as usize; let cnt = hdr[1] as usize;
    if lptr <= 0x10000 || cnt == 0 || cnt > 256 { return; }
    if EFFECT_DUMPS.fetch_add(1, Ordering::Relaxed) >= 24 { return; }
    let mut s = format!("[{}ms] entity={:#x} list ptr={:#x} cnt={} hdr2={:#x} hdr3={:#x}", now_ms(), entity, lptr, cnt, hdr[2], hdr[3]);
    for i in 0..cnt.min(14) {
        let mut buf = [0u8; 0x120];
        if !safe_copy(buf.as_mut_ptr(), (lptr + i * 0x120) as *const u8, 0x120) { continue; }
        let nlen = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
        let name = if nlen <= 0x3c { String::from_utf8_lossy(&buf[4..4 + nlen]).into_owned() } else { format!("<len{}>", nlen) };
        let mut st = String::new();
        for k in 0..24 { let o = 0x58 + k * 4; st.push_str(&format!("{} ", i32::from_le_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]]))); }
        s.push_str(&format!("\n  [{}] '{}' +0x58 i32[0..24]=[{}]", i, name, st.trim()));
    }
    STATHOOK_LOG.lock().unwrap().push(s);
}
fn stathook_flush() {
    let v = STATHOOK_LOG.lock().unwrap();
    let n = v.len();
    if n != 0 && n != STAT_FLUSH_LEN.load(Ordering::Relaxed) {
        STAT_FLUSH_LEN.store(n, Ordering::Relaxed);
        write_log("item_editor_stathook.txt", &v.join("\n\n"));
    }
}
// ───────── 추가 후킹: FUN_141b85c50 (아이템합산 → entity+0x3b0) ─────────
//  ★ FUN_141b860d0(최종계산)은 +0x600=(+0x3b0 + base) 로 **+0x3b0(아이템합 캐시)를 읽음** — +0x58 안 봄.
//    그래서 거기 inject 는 무효(1명만 우연히 됨). 진짜 합산하는 FUN_141b85c50(+0x58→+0x3b0) 의 합산 전에 주입해야
//    +0x3b0=편집값 → +0x600 반영. equip/recalc 마다 호출이라 전챔프 커버.
const PER_ITEM_RVA: usize = 0x1fc0960; // FUN_141b85c50 0.5.0_3(구0.5.0_2=0x1d03db0, UNIQUE PROL-OK). f(rcx=entity, rdx=keyptr, r8=keylen)
static PER_TRAMP: AtomicUsize = AtomicUsize::new(0);
static PER_HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
type PerFn = extern "win64" fn(usize, usize, usize) -> usize;
extern "win64" fn per_detour(entity: usize, kp: usize, kl: usize) -> usize {
    if entity > 0x10000 { let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { inject_effects(entity); })); } // 합산 전 +0x58 주입
    let t = PER_TRAMP.load(Ordering::Relaxed);
    if t != 0 { let orig: PerFn = unsafe { core::mem::transmute(t) }; orig(entity, kp, kl) } else { 0 }
}
fn install_peritem_hook() {
    if PER_HOOK_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    unsafe {
        let base = GetModuleHandleW(core::ptr::null());
        if base == 0 { PER_HOOK_INSTALLED.store(false, Ordering::Relaxed); return; }
        let fn_addr = base + PER_ITEM_RVA;
        let expect: [u8; 12] = [0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53];
        for i in 0..12 {
            if *((fn_addr + i) as *const u8) != expect[i] {
                STATHOOK_LOG.lock().unwrap().push(format!("[{}ms] peritem prologue mismatch @+{}: {:#x} (RVA {:#x})", now_ms(), i, *((fn_addr + i) as *const u8), PER_ITEM_RVA));
                return;
            }
        }
        let stub = VirtualAlloc(0, 64, 0x3000, 0x40);
        if stub == 0 { return; }
        let mut sb: Vec<u8> = Vec::new();
        sb.extend_from_slice(&expect);
        sb.extend_from_slice(&[0x48, 0xb8]); sb.extend_from_slice(&((fn_addr + 0xc) as u64).to_le_bytes());
        sb.extend_from_slice(&[0xff, 0xe0]);
        core::ptr::copy_nonoverlapping(sb.as_ptr(), stub as *mut u8, sb.len());
        PER_TRAMP.store(stub, Ordering::Relaxed);
        let d = per_detour as usize;
        let mut patch = [0u8; 12]; patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&d.to_le_bytes()); patch[10] = 0xff; patch[11] = 0xe0;
        let mut old = 0u32;
        if VirtualProtect(fn_addr, 12, 0x40, &mut old) == 0 { return; }
        core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
        VirtualProtect(fn_addr, 12, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
        STATHOOK_LOG.lock().unwrap().push(format!("[{}ms] peritem hook installed fn={:#x} stub={:#x}", now_ms(), fn_addr, stub));
    }
}
// ───────── 추가 후킹: FUN_141b86380 (단순합산 → entity+0x3b0) ─────────
//  ★ 매치는 대부분 챔프를 이 "단순합산"으로 +0x3b0 계산하는 듯(FUN_141b85c50 후킹해도 1명만 → 그건 다른 경로).
//    합산 직전 +0x58 주입 → +0x3b0=편집값. 프롤로그 56 48 83 ec 70 66 44 0f 7f 64 24 60 (push rsi+sub rsp+movdqa, 12B 깔끔).
const SUM_RVA: usize = 0x1fc1560; // FUN_141b86380 0.5.0_3(구0.5.0_2=0x1d049b0, UNIQUE PROL-OK)
static SUM_TRAMP: AtomicUsize = AtomicUsize::new(0);
static SUM_HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
type SumFn = extern "win64" fn(usize, usize, usize, usize) -> usize;
extern "win64" fn sum_detour(entity: usize, a: usize, b: usize, c: usize) -> usize {
    if entity > 0x10000 { let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { inject_effects(entity); })); } // 합산 전 주입
    let t = SUM_TRAMP.load(Ordering::Relaxed);
    if t != 0 { let orig: SumFn = unsafe { core::mem::transmute(t) }; orig(entity, a, b, c) } else { 0 }
}
fn install_sumstat_hook() {
    if SUM_HOOK_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    unsafe {
        let base = GetModuleHandleW(core::ptr::null());
        if base == 0 { SUM_HOOK_INSTALLED.store(false, Ordering::Relaxed); return; }
        let fn_addr = base + SUM_RVA;
        let expect: [u8; 12] = [0x56, 0x48, 0x83, 0xec, 0x70, 0x66, 0x44, 0x0f, 0x7f, 0x64, 0x24, 0x60];
        for i in 0..12 {
            if *((fn_addr + i) as *const u8) != expect[i] {
                STATHOOK_LOG.lock().unwrap().push(format!("[{}ms] sum prologue mismatch @+{}: {:#x} (RVA {:#x})", now_ms(), i, *((fn_addr + i) as *const u8), SUM_RVA));
                return;
            }
        }
        let stub = VirtualAlloc(0, 64, 0x3000, 0x40);
        if stub == 0 { return; }
        let mut sb: Vec<u8> = Vec::new();
        sb.extend_from_slice(&expect);
        sb.extend_from_slice(&[0x48, 0xb8]); sb.extend_from_slice(&((fn_addr + 0xc) as u64).to_le_bytes());
        sb.extend_from_slice(&[0xff, 0xe0]);
        core::ptr::copy_nonoverlapping(sb.as_ptr(), stub as *mut u8, sb.len());
        SUM_TRAMP.store(stub, Ordering::Relaxed);
        let d = sum_detour as usize;
        let mut patch = [0u8; 12]; patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&d.to_le_bytes()); patch[10] = 0xff; patch[11] = 0xe0;
        let mut old = 0u32;
        if VirtualProtect(fn_addr, 12, 0x40, &mut old) == 0 { return; }
        core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
        VirtualProtect(fn_addr, 12, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
        STATHOOK_LOG.lock().unwrap().push(format!("[{}ms] sumstat hook installed fn={:#x} stub={:#x}", now_ms(), fn_addr, stub));
    }
}
fn install_stat_hook() {
    if STAT_HOOK_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    unsafe {
        let base = GetModuleHandleW(core::ptr::null());
        if base == 0 { STAT_HOOK_INSTALLED.store(false, Ordering::Relaxed); return; }
        let fn_addr = base + STAT_FN_RVA;
        let expect: [u8; 12] = [0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53];
        for i in 0..12 {
            if *((fn_addr + i) as *const u8) != expect[i] {
                STATHOOK_LOG.lock().unwrap().push(format!("[{}ms] prologue mismatch @+{}: {:#x} (RVA {:#x}) — 훅 미설치", now_ms(), i, *((fn_addr + i) as *const u8), STAT_FN_RVA));
                return;
            }
        }
        let stub = VirtualAlloc(0, 64, 0x3000, 0x40); // MEM_COMMIT|RESERVE, PAGE_EXECUTE_READWRITE
        if stub == 0 { return; }
        let mut sb: Vec<u8> = Vec::new();
        sb.extend_from_slice(&expect);
        sb.extend_from_slice(&[0x48, 0xb8]); sb.extend_from_slice(&((fn_addr + 0xc) as u64).to_le_bytes());
        sb.extend_from_slice(&[0xff, 0xe0]);
        core::ptr::copy_nonoverlapping(sb.as_ptr(), stub as *mut u8, sb.len());
        STAT_TRAMP.store(stub, Ordering::Relaxed);
        let d = stat_detour as usize;
        let mut patch = [0u8; 12]; patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&d.to_le_bytes()); patch[10] = 0xff; patch[11] = 0xe0;
        let mut old = 0u32;
        if VirtualProtect(fn_addr, 12, 0x40, &mut old) == 0 { return; }
        core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
        VirtualProtect(fn_addr, 12, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
        STATHOOK_LOG.lock().unwrap().push(format!("[{}ms] stat hook installed fn={:#x} stub={:#x} → 경기 진행하면 이펙트리스트 덤프", now_ms(), fn_addr, stub));
    }
}

// ★ ie_open 버튼 평소(비호버) 글자/아이콘 색을 다른 메뉴 회색과 맞춤 (scrim fix_scrim_btn_idle 동일).
//   color_icon_button 러너의 +144(아이콘)/+448(텍스트) = 비호버 색 슬롯. 호버 슬롯은 안 건드려 .ui 밝은색(#e8e8e8) 유지.
//   → idle=회색, hover=밝음 (스크림 사이드버튼과 룩 일치). 매프레임 호출.
unsafe fn runner_base(n: &Node) -> usize {
    let any: &dyn std::any::Any = n.runner.as_any();
    let parts: [usize; 2] = std::mem::transmute::<*const dyn std::any::Any, [usize; 2]>(any as *const dyn std::any::Any);
    parts[0]
}
fn fix_ie_btn_idle(root: &mut Node) {
    fn walk(n: &mut Node, id: &str) -> bool {
        if n.id.as_str() == id {
            unsafe {
                let base = runner_base(n);
                let (r, g, b, a) = (0.70f32, 0.71f32, 0.74f32, 1.0f32); // ≈다른 메뉴 회색 (scrim 과 동일)
                for &off in &[144usize, 448usize] { // +144 아이콘 / +448 텍스트 (비호버 슬롯만)
                    safe_write_i32(base + off, r.to_bits() as i32);
                    safe_write_i32(base + off + 4, g.to_bits() as i32);
                    safe_write_i32(base + off + 8, b.to_bits() as i32);
                    safe_write_i32(base + off + 12, a.to_bits() as i32);
                }
            }
            return true;
        }
        for c in n.child.iter_mut() { if walk(c, id) { return true; } }
        false
    }
    walk(root, "ie_open");
}

struct ItemEditorClientExt;
impl ModExtension for ItemEditorClientExt {
    fn post_update(&self, scene: &mut Scene, ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        seh_install();
        install_stat_hook();    // FUN_141b860d0 (최종계산, 덤프용 — inject 는 무효지만 dump 유지)
        install_peritem_hook(); // FUN_141b85c50 (키매칭합산 → +0x3b0)
        install_sumstat_hook(); // ★ FUN_141b86380 (단순합산 → +0x3b0) — 매치 주경로 추정, 전챔프 커버 시도
        if LOG_ENABLED { stathook_flush(); } // 덤프 누적분을 파일로 (디버그시만)
        let Scene::InGame { .. } = scene else { return; };
        // (배경 호버 차단은 이제 tfm2_ui_inject 프레임워크가 modal 플래그로 자동 처리)

        // 클릭 라우트 (합친 .ui override 로 선언된 inert 노드들 → ui_kit 라우팅).
        let mut routes: Vec<(String, ui_kit::ClickFn)> = vec![
            ui_kit::route("ie_open", Rc::new(|| { MODAL_OPEN.fetch_xor(true, Ordering::Relaxed); UI_REFRESH.store(true, Ordering::Relaxed); SCROLL_RESET.store(true, Ordering::Relaxed); EDIT_OPEN.store(false, Ordering::Relaxed);
                EFFECT_DUMPS.store(0, Ordering::Relaxed); INJECT_LOGS.store(0, Ordering::Relaxed); STAT_FLUSH_LEN.store(0, Ordering::Relaxed); STATHOOK_LOG.lock().unwrap().clear(); })), // 훅 로그 리셋(편집세션마다 새로)
            ui_kit::route("ie_close", Rc::new(|| { MODAL_OPEN.store(false, Ordering::Relaxed); })),
            ui_kit::route("ie_dim", Rc::new(|| { MODAL_OPEN.store(false, Ordering::Relaxed); })),
            ui_kit::route("ie_confirm", Rc::new(|| { CONFIRM_REQ.store(true, Ordering::Relaxed); })),
            ui_kit::route("ie_cancel", Rc::new(|| { EDIT_OPEN.store(false, Ordering::Relaxed); })),
            ui_kit::route("ie_edit_dim", Rc::new(|| { EDIT_OPEN.store(false, Ordering::Relaxed); })),
        ]; // (활성만/전체 세그먼트 토글 제거 — 비활성 아이템이 사실상 안 생겨 무의미)
        for i in 0..UI_ROWS {
            let ii = i;
            routes.push(ui_kit::route(&format!("ie_row{:02}_b", i), Rc::new(move || { ROW_CLICKED.store(ii, Ordering::Relaxed); })));
        }
        ui_kit::ensure_clicks(ui, &LAST_FH, routes);
        fix_ie_btn_idle(&mut ui.root); // 사이드바 버튼 비호버 색 = 회색(호버시 밝아짐, 스크림과 동일)

        let open = MODAL_OPEN.load(Ordering::Relaxed);
        ui_kit::set_visible_by_id(&mut ui.root, "ie_modal", open);
        if !open { return; }

        if UI_REFRESH.swap(false, Ordering::Relaxed) {
            unsafe { rebuild_ui_reg(); }
            SELECTED.store(usize::MAX, Ordering::Relaxed);
        }
        // 열 때 스크롤 맨 위로 (닫았다 다시 열면 원위치)
        if SCROLL_RESET.swap(false, Ordering::Relaxed) {
            ui_kit::scroll_set_by_id(&mut ui.root, "ie_scroll", 0.0);
        }
        let reg = UI_REG.lock().unwrap();
        // 표시 목록 = 전체 (토글 제거). 더미/probe 는 build_registry 에서 이미 제외됨.
        let display: Vec<usize> = (0..reg.len()).collect();
        let n = display.len();
        // 행 클릭 → 선택 (화면행 rc → display[rc] = reg 인덱스)
        let rc = ROW_CLICKED.swap(usize::MAX, Ordering::Relaxed);
        if rc != usize::MAX && rc < n {
            SELECTED.store(display[rc], Ordering::Relaxed);
            EDIT_OPEN.store(true, Ordering::Relaxed);   // 행 클릭 → 편집 팝업
            EDIT_PREFILL.store(true, Ordering::Relaxed);
        }
        let sel = SELECTED.load(Ordering::Relaxed); // reg 인덱스
        // contents 높이 = 표시수 × 행높이(46)
        if let Some(lst) = ui_kit::find_mut(&mut ui.root, "ie_list") {
            ui_kit::set_layout_size(lst, 0.0, (n.max(1) as f32) * 46.0);
        }
        // 행 채우기 (표시 목록 기준)
        for i in 0..UI_ROWS {
            let show = i < n;
            if let Some(node) = ui_kit::find_mut(&mut ui.root, &format!("ie_row{:02}", i)) { node.visible = show; }
            if show {
                let ri = display[i];
                let (k, van, _, active, vals) = &reg[ri];
                // 이름칸(_n): 선택마크 + [비활성]태그 prefix 없이 — 이름은 순수 i18n 참조여야 현지화됨.
                //   활성/바닐라=현지화 이름, 비활성=코드명 폴백.
                if let Some(nn) = ui_kit::find_mut(&mut ui.root, &format!("ie_row{:02}_n", i)) {
                    ui_kit::text_set_deep(nn, &name_label(k, *van, *active));
                }
                // 값칸 v0..v5 = [가격,공격,주문,방어,체력,마저]
                for c in 0..6 {
                    if let Some(nd) = ui_kit::find_mut(&mut ui.root, &format!("ie_row{:02}_v{}", i, c)) {
                        ui_kit::text_set_deep(nd, &cell_num(vals[c]));
                    }
                }
            }
        }
        // 제목 (활성/전체 토글 제거됨)
        if let Some(t) = ui_kit::find_mut(&mut ui.root, "ie_title") {
            ui_kit::text_set_deep(t, &format!("아이템 편집   ({}개)", n));
        }
        // 하단 선택 표시줄: 이름(현지화/코드) + 코드명·기본/모드
        if let Some(sn) = ui_kit::find_mut(&mut ui.root, "ie_sel_n") {
            let nm = if sel < reg.len() { let r = &reg[sel]; name_label(&r.0, r.1, r.3) } else { String::new() };
            ui_kit::text_set_deep(sn, &nm);
        }
        if let Some(s) = ui_kit::find_mut(&mut ui.root, "ie_sel") {
            let txt = if sel < reg.len() {
                let r = &reg[sel];
                format!("코드명: {}   [{}]", r.0, if r.1 { "기본" } else { "모드" })
            } else { "행을 클릭해 아이템 선택".to_string() };
            ui_kit::text_set_deep(s, &txt);
        }

        // ── 편집 팝업 (가격 + 5스탯, text_edit 입력) ── (sel 은 reg 인덱스)
        let eopen = EDIT_OPEN.load(Ordering::Relaxed) && sel < reg.len();
        ui_kit::set_visible_by_id(&mut ui.root, "ie_edit", eopen);
        if eopen {
            let (k, van, elem, active) = { let r = &reg[sel]; (r.0.clone(), r.1, r.2, r.3) };
            if let Some(nd) = ui_kit::find_mut(&mut ui.root, "ie_edit_title") {
                ui_kit::text_set_deep(nd, &format!("편집 — {}  [{}]", k, if van { "기본" } else { "모드" }));
            }
            if let Some(nd) = ui_kit::find_mut(&mut ui.root, "ie_edit_name") {
                ui_kit::text_set_deep(nd, &name_label(&k, van, active));
            }
            let prefill = EDIT_PREFILL.swap(false, Ordering::Relaxed);
            let confirm = CONFIRM_REQ.swap(false, Ordering::Relaxed);
            let mut applied = String::new();
            let mut changed = false;
            for (suf, fname, kind) in edit_fields() {
                let off = field_offset(kind, van);
                let is_price = fname == "price";
                // 현재값: 가격=Database 직독 / 스탯=EDITS 오버레이(편집값 우선) → 없으면 Database 원본.
                //   (스탯은 Database 에 안 쓰므로 세이브 무오염 — 표시/프리필/변경판정 모두 EDITS 기준)
                let cur = || -> Option<i32> {
                    if is_price { if off != 0 { unsafe { safe_read_i32(elem + off) } } else { None } }
                    else { edit_val(&k, fname).or_else(|| if off != 0 { unsafe { safe_read_i32(elem + off) } } else { None }) }
                };
                if prefill {
                    let t = cur().map(|v| v.to_string()).unwrap_or_else(|| "-".into());
                    if let Some(nd) = ui_kit::find_mut(&mut ui.root, suf) { ui_kit::textedit_set(nd, &t); }
                }
                if confirm {
                    let v = ui_kit::find(&ui.root, suf).and_then(|nd| ui_kit::textedit_get(nd)).and_then(|t| {
                        let t = t.trim();
                        if t.is_empty() { Some(0) } else { t.parse::<i32>().ok() } // 빈칸 = 0 (값 비우고 확인하면 0으로)
                    });
                    if let Some(v) = v {
                        let before = cur().unwrap_or(i32::MIN);
                        if v != before {
                            // ★가격만 Database 즉시반영(경제). 스탯은 안 씀 → 주입(inject_effects)이 EDITS 읽어 전투반영.
                            if is_price && off != 0 { unsafe { safe_write_i32(elem + off, v); } }
                            upsert_edit(&k, fname, v); // 영속 저장(item_edits.txt) — 스탯 주입의 소스
                            applied.push_str(&format!("{}={} ", fname, v));
                            changed = true;
                        }
                    }
                }
            }
            if confirm {
                if changed { save_edits(); }
                write_log("item_editor_apply.txt", &format!("[{}ms] UI {} [{}]: {}\n", now_ms(), k, if van { "기본" } else { "모드" }, if applied.is_empty() { "변경없음".into() } else { applied }));
                UI_REFRESH.store(true, Ordering::Relaxed);
                EDIT_OPEN.store(false, Ordering::Relaxed);
            }
        }
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_server_extension(ItemEditorServerExt);
    reg.set_extension(ItemEditorClientExt);
    reg
}
declare_mod!(init);
