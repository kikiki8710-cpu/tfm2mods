//! tfm2_active_probe — "현재 활성 아이템 리스트" 진단 전용 모드
//! ===========================================================================
//! 목적: 게임에 지금 '활성'으로 병합돼 있는 아이템 전체(바닐라 + 활성 모드 아이템)를
//!       로그 파일로 뽑는다. 게임값은 절대 바꾸지 않는다(읽기 전용).
//!
//! 활성 판정 원리 (RE 규명, 2026-07-05):
//!   게임은 아이템 element 에 enabled 플래그를 두지 않는다. 대신 모드 로드 시
//!   "활성 모드만" Database 에 아이템을 병합(FUN_1408e3350 → …→ FUN_141d90190)한다.
//!   ⟹ 비활성 모드 아이템은 애초에 Database 통합 컬렉션에 안 들어간다.
//!   ⟹ "Database 의 바닐라 배열 + mod_items Vec 에 존재 = 활성" 이 게임과 동일한 판정.
//!   (i18n text/item 은 '표시명' 전용이라 지워도 아이템은 그대로 뜬다 — 유저 실측 확증.)
//!
//! 그래서 이 모드는 그 통합 컬렉션(바닐라 영역 스캔 + mod_items Vec)을 직접 순회해 덤프한다.
//! 외부 파일/enabled_mods/i18n 교차 불필요 — 게임 메모리가 곧 정답.
//!
//! 안전: 모든 메모리 접근은 SEH(VEH) 로 보호 → 틀린 주소 만져도 게임 안 멈춤. 쓰기 0.
//! 빌드: powershell -File C:\tfm2mods\build_inj.ps1 -Src C:\tfm2mods\tfm2_active_probe\src\lib.rs -ModId tfm2_active_probe
//! ===========================================================================
#![allow(dead_code, unused_imports, unused_variables)]
use mod_api::*;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "tfm2_active_probe";

// Database 내 champion_patch_statistics 필드 오프셋 (확정 RE). db_base = 필드주소 − 이 값.
const O_CPS: usize = 0x16698;
// 바닐라 아이템 배열 시작 (db 상대). 실패 시 스캔 폴백.
const O_VANILLA: usize = 0x12d50;
// element stride 후보 (ModItemEntry/BaseItemInfo). 자동탐지.
const ITEM_STRIDES: [usize; 3] = [0x1a8, 0x198, 0x1b0];
// 현재 적용된 활성 모드 서명 Vec (RE 2026-07-05: FUN_1408e3350 이 관리). ptr@db+0x16690.
const O_ACTIVE_SIG: usize = 0x16690;

// ───────────────────────── WinAPI ─────────────────────────
type HMODULE = isize; type DWORD = u32; type BOOL = i32;
const PAGE_READWRITE: u32 = 0x04;
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleExW(f: DWORD, name: *const u16, h: *mut HMODULE) -> BOOL;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, n: DWORD) -> DWORD;
    fn AddVectoredExceptionHandler(first: u32, handler: VehHandler) -> usize;
    fn GetCurrentThreadId() -> u32;
}

// ───────────────────────── SEH 안전 read (item_editor 검증본) ─────────────────────────
//  SEH[]: 0=active 1=tid 2=land_rip 3=land_rsp 4=land_rbp 5=code_lo 6=code_hi 7=faults
#[repr(C)]
struct ExceptionRecord { code: u32, flags: u32, rec: usize, addr: usize, nparams: u32, _p: u32, params: [usize; 15] }
#[repr(C)]
struct ExceptionPointers { rec: *mut ExceptionRecord, ctx: *mut core::ffi::c_void }
type VehHandler = extern "system" fn(*mut ExceptionPointers) -> i32;

static mut SEH: [u64; 8] = [0u64; 8];
static SEH_INSTALLED: AtomicBool = AtomicBool::new(false);
static SEH_BUSY: AtomicBool = AtomicBool::new(false);

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
unsafe fn safe_read_bytes(addr: usize, len: usize, out: &mut Vec<u8>) -> bool {
    if len == 0 || len > 4096 { return false; }
    out.clear(); out.resize(len, 0);
    safe_copy(out.as_mut_ptr(), addr as *const u8, len)
}
fn looks_heap(v: u64) -> bool { v & 0x7 == 0 && v >= 0x10000 && v < 0x0000_8000_0000_0000 && (v & 0xffff) != 0 }
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
        if GetModuleHandleExW(0x4 | 0x2, addr as *const u16, &mut h) == 0 || h == 0 { return None; }
        let mut buf = [0u16; 4096];
        let n = GetModuleFileNameW(h, buf.as_mut_ptr(), buf.len() as DWORD);
        if n == 0 { return None; }
        Some(PathBuf::from(String::from_utf16_lossy(&buf[..n as usize])))
    }
}
fn mod_dir() -> Option<PathBuf> { dll_path()?.parent().map(|p| p.to_path_buf()) }
fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }

// ───────────────────────── 아이템 레지스트리 탐색 (item_editor 검증본) ─────────────────────────
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
unsafe fn is_item_elem(addr: usize) -> bool {
    if key_of_elem(addr).is_none() { return false; }
    let voff = VPRICE_OFF.load(Ordering::Relaxed);
    let toff = VTIER_OFF.load(Ordering::Relaxed);
    if voff == 0 || toff == 0 { return true; } // 오프셋 미탐지 폴백(키만)
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
            if !is_item_elem(p) { continue; }
            let st = detect_stride(p);
            if st == 0 { continue; }
            let probe = c.min(48);
            let valid = (0..probe).filter(|&i| is_item_elem(p + i * st)).count();
            if valid * 10 < probe * 8 || valid < 3 { continue; }
            let mut cnt = 0usize;
            while cnt < c.max(1) && cnt < 500 { if is_item_elem(p + cnt * st) { cnt += 1; } else { break; } }
            return Some((p, st, cnt));
        }
    }
    None
}
// 바닐라 30개 영역스캔 (한 base 에 연속 15개뿐이라 stride 가정 못 함).
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
// 전체 활성 레지스트리: (key, elem_addr, is_vanilla). 바닐라(영역스캔) + 모드 아이템(Vec).
unsafe fn build_registry(db: usize) -> Vec<(String, usize, bool)> {
    let mut out: Vec<(String, usize, bool)> = Vec::new();
    for (k, addr) in find_all_vanilla(db) { out.push((k, addr, true)); }
    if let Some((buf, st, cnt)) = find_mod_items(db) {
        let poff = mod_price_off();
        for i in 0..cnt {
            let addr = buf + i * st;
            if let Some(k) = key_of_elem(addr) {
                // 더미/probe 센티넬(예: 99999) 제외. 정상템 ≤ ~2000.
                let p = if poff != 0 { safe_read_i32(addr + poff).unwrap_or(-1) } else { -1 };
                if p >= 50000 { continue; }
                out.push((k, addr, false));
            }
        }
    }
    out
}

// ───────────────────────── 최종빌드 판정 (next_tier, scrim 검증본) ─────────────────────────
// element+o = next_tier Vec<String>{len@o, ptr@o+8, cap@o+0x10}, 원소 String stride 0x18.
//   next_tier = "이 아이템으로 조합되는 상위 아이템들". 비어있음 = 더 못 올라감 = 최종빌드.
//   len==0 → 빈 Vec(최종). len>8 이거나 원소가 레지스트리 키 아니면 next_tier 아님 → None.
unsafe fn read_next_tier(elem: usize, o: usize) -> Option<Vec<String>> {
    let len = safe_read_u64(elem + o)? as usize;
    if len == 0 { return Some(Vec::new()); }
    if len > 8 { return None; }
    let ptr = safe_read_u64(elem + o + 8)? as usize;
    let cap = safe_read_u64(elem + o + 0x10)? as usize;
    if ptr <= 0x10000 || cap < len { return None; }
    let mut out = Vec::new();
    for j in 0..len { out.push(key_of_elem(ptr + j * 0x18)?); }
    Some(out)
}
// next_tier 오프셋 탐지: 각 후보 o 에 대해 "비어있지않고 전부 레지스트리키인 Vec" element 수 투표.
//   최다 득표 o = next_tier 오프셋. (tags 같은 다른 Vec<String> 은 키셋에 없어 배제됨.)
unsafe fn detect_nt_offset(addrs: &[usize], keyset: &[String], max_off: usize) -> (usize, u32) {
    let in_reg = |k: &str| keyset.iter().any(|x| x == k);
    let mut best_off = 0usize;
    let mut best_votes = 0u32;
    let mut o = 0x18usize;
    while o + 0x18 <= max_off {
        let mut votes = 0u32;
        for &e in addrs {
            if let Some(v) = read_next_tier(e, o) {
                if !v.is_empty() && v.iter().all(|k| in_reg(k)) { votes += 1; }
            }
        }
        if votes > best_votes { best_votes = votes; best_off = o; }
        o += 8;
    }
    (best_off, best_votes)
}

// ───────────────────────── price/tier 오프셋 자동탐지 (item_editor 검증본) ─────────────────────────
static VPRICE_OFF: AtomicUsize = AtomicUsize::new(0);
static VTIER_OFF: AtomicUsize = AtomicUsize::new(0);
static MPRICE_OFF: AtomicUsize = AtomicUsize::new(0);
fn mod_price_off() -> usize {
    let m = MPRICE_OFF.load(Ordering::Relaxed);
    if m != 0 { m } else { VPRICE_OFF.load(Ordering::Relaxed) }
}
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
    let mut off = 0x18usize;
    while off + 4 <= st {
        let vals: Vec<Option<i32>> = (0..n).map(|i| safe_read_i32(base + i * st + off)).collect();
        if vals.iter().all(|v| v.is_some()) {
            let v: Vec<i32> = vals.iter().map(|o| o.unwrap()).collect();
            let is_tier = (0..n).all(|i| v[i] == (i % 5) as i32);
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
                && distinct >= 2;
            if is_tier && tier_off == 0 { tier_off = off; }
            if is_price && price_off == 0 {
                price_off = off;
                diag.push_str(&format!("price 후보 @+{:#x}: tier별 값 {:?}\n", off, by_tier));
            }
        }
        off += 4;
    }
    if price_off == 0 && tier_off >= 8 {
        let cand = tier_off - 8;
        let vals: Vec<i32> = (0..n).map(|i| safe_read_i32(base + i * st + cand).unwrap_or(-999999)).collect();
        let good = vals.iter().filter(|&&x| (1..=2_000_000).contains(&x)).count();
        if good * 10 >= n * 7 { price_off = cand; }
    }
    diag.push_str(&format!("→ 탐지결과: VPRICE_OFF=+{:#x}  VTIER_OFF=+{:#x}\n", price_off, tier_off));
    (price_off, tier_off, diag)
}
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

// ───────────────────────── 활성 모드 서명 Vec 덤프 (보너스, RE 2026-07-05) ─────────────────────────
// Database+0x16690 = 현재 적용된 활성 모드 서명 Vec (stride 0x30 추정). 각 엔트리에서
// String 형태(len,ptr,cap)를 시도해 모드 id/서명 문자열을 뽑는다. 실패해도 무해(읽기전용).
unsafe fn dump_active_signatures(db: usize) -> String {
    let mut s = String::from("=== 활성 모드 서명 Vec (Database+0x16690, 실험적) ===\n");
    let ptr = safe_read_u64(db + O_ACTIVE_SIG).unwrap_or(0) as usize;
    // len 후보: +0x16698(=cps, 겹침 주의) 대신 +0x166a0 을 우선 시도.
    let len_a = safe_read_u64(db + O_ACTIVE_SIG + 0x10).unwrap_or(0) as usize;
    let len_b = safe_read_u64(db + O_ACTIVE_SIG + 8).unwrap_or(0) as usize;
    s.push_str(&format!("  ptr={:#x} len후보(+0x10)={} (+0x8)={}\n", ptr, len_a, len_b));
    if !looks_heap(ptr as u64) { s.push_str("  (ptr 비힙 — 활성 모드 0개이거나 구조 상이)\n"); return s; }
    let len = if (1..=64).contains(&len_a) { len_a } else if (1..=64).contains(&len_b) { len_b } else { 8 };
    // 각 엔트리를 stride 0x30 으로, 오프셋 0/0x8 에서 String{len,ptr} 을 읽어본다.
    'outer: for i in 0..len.min(32) {
        let e = ptr + i * 0x30;
        let mut got = false;
        for eo in [0usize, 8, 0x10, 0x18] {
            // String 레이아웃 두 가지(cap,ptr,len / len,ptr,cap) 모두 시도
            if let Some(k) = key_of_elem(e + eo) {
                s.push_str(&format!("  [{}] +{:#x}: \"{}\"\n", i, eo, k));
                got = true;
                break;
            }
        }
        if !got {
            let w0 = safe_read_u64(e).unwrap_or(0);
            if w0 == 0 && i > 0 { break 'outer; }
            s.push_str(&format!("  [{}] (문자열 미검출) w0={:#x}\n", i, w0));
        }
    }
    s
}

// ───────────────────────── 덤프 (활성 아이템 리스트) ─────────────────────────
static PROBE_DONE: AtomicBool = AtomicBool::new(false);
fn dump_path() -> Option<PathBuf> { mod_dir().map(|d| d.join("tfm2_active_probe.txt")) }

unsafe fn dump_active_items(db: usize, where_: &str) {
    if PROBE_DONE.load(Ordering::Relaxed) { return; }
    // 바닐라 배열이 아직 안 올라왔으면 done 처리 말고 다음 틱 재시도.
    if find_vanilla(db).is_none() { return; }
    PROBE_DONE.store(true, Ordering::Relaxed);

    let (vprice, vtier, ddiag) = detect_vanilla_offsets(db);
    VPRICE_OFF.store(vprice, Ordering::Relaxed);
    VTIER_OFF.store(vtier, Ordering::Relaxed);
    let mprice = detect_mod_price(db);
    MPRICE_OFF.store(mprice, Ordering::Relaxed);

    let mut s = format!("[{}ms] tfm2_active_probe — 현재 활성 아이템 리스트\n", now_ms());
    s.push_str(&format!("  진입={}  db={:#x}\n", where_, db));
    s.push_str(&ddiag);
    s.push_str(&format!("  MPRICE_OFF=+{:#x}\n\n", mprice));

    let poff = mod_price_off();
    let reg = build_registry(db);
    let nv = reg.iter().filter(|(_, _, v)| *v).count();
    let nm = reg.len() - nv;

    // 모드 배열 정보
    if let Some((buf, st, cnt)) = find_mod_items(db) {
        s.push_str(&format!("모드 아이템 Vec: buf={:#x} stride={:#x} cnt={}\n", buf, st, cnt));
    } else {
        s.push_str("모드 아이템 Vec: 없음 (활성 아이템 모드 없음 → 바닐라만)\n");
    }
    s.push_str(&format!("\n=== 활성 아이템 총 {}개 (바닐라 {} + 활성 모드 {}) ===\n", reg.len(), nv, nm));

    // 바닐라
    s.push_str(&format!("\n[바닐라 {}개]\n", nv));
    s.push_str("  # | key | price | tier\n");
    let mut vi = 0;
    for (k, elem, van) in reg.iter().filter(|(_, _, v)| *v) {
        let p = if vprice != 0 { safe_read_i32(elem + vprice).unwrap_or(-1) } else { -1 };
        let t = if vtier != 0 { safe_read_i32(elem + vtier).unwrap_or(-1) } else { -1 };
        s.push_str(&format!("  {:>2} | {} | {} | t{}\n", vi, k, p, t));
        vi += 1;
    }

    // 활성 모드 아이템 (= 게임이 활성으로 병합한 것 = 이 리스트가 유저 질문의 답)
    s.push_str(&format!("\n[활성 모드 아이템 {}개]  ← 게임에 지금 켜져있는 모드의 아이템\n", nm));
    if nm == 0 {
        s.push_str("  (없음)\n");
    } else {
        s.push_str("  # | id(=30+idx) | key | price | tier\n");
        let mut mi = 0;
        for (k, elem, van) in reg.iter().filter(|(_, _, v)| !*v) {
            let p = if poff != 0 { safe_read_i32(elem + poff).unwrap_or(-1) } else { -1 };
            let t = if vtier != 0 { safe_read_i32(elem + vtier).unwrap_or(-1) } else { -1 };
            s.push_str(&format!("  {:>2} | {:>3} | {} | {} | t{}\n", mi, 30 + mi, k, p, t));
            mi += 1;
        }
    }

    // ── 최종빌드 아이템 판정 (next_tier 상위방향 + built_set) ──
    //   최종 = next_tier(상위 조합) '확실히' 비었음(Some(empty)) AND 무언가 이 아이템으로 조합됨(built).
    //   None(그 오프셋서 next_tier 아님=읽기실패)은 최종서 제외. 베이스컴포넌트(built 아님)도 제외.
    //   ★바닐라(stride 0x198)·모드(0x1a8) 구조체 오프셋이 다를 수 있어 배열별로 next_tier 오프셋 분리 탐지.
    let all_keys: Vec<String> = reg.iter().map(|(k, _, _)| k.clone()).collect();
    let van_addrs: Vec<usize> = reg.iter().filter(|(_, _, v)| *v).map(|(_, a, _)| *a).collect();
    let mod_addrs: Vec<usize> = reg.iter().filter(|(_, _, v)| !*v).map(|(_, a, _)| *a).collect();
    let (van_off, van_votes) = detect_nt_offset(&van_addrs, &all_keys, 0x198);
    let (mod_off, mod_votes) = detect_nt_offset(&mod_addrs, &all_keys, 0x1a8);
    let off_for = |van: bool| if van { van_off } else { mod_off };
    s.push_str("\n=== 최종빌드 아이템 (조합 완성품만) ===\n");
    s.push_str(&format!("  next_tier offset: 바닐라=+{:#x}(votes {}) 모드=+{:#x}(votes {})\n",
        van_off, van_votes, mod_off, mod_votes));
    // built_set = 각 아이템의 next_tier 타겟(=조합 결과물) 합집합
    let mut built: std::collections::HashSet<String> = std::collections::HashSet::new();
    for (_, e, van) in reg.iter() {
        let o = off_for(*van); if o == 0 { continue; }
        if let Some(nt) = read_next_tier(*e, o) { for t in nt { built.insert(t); } }
    }
    s.push_str(&format!("  built_targets={} (다른 아이템의 상위조합 결과로 등장하는 key 수)\n", built.len()));
    let vt = VTIER_OFF.load(Ordering::Relaxed);
    let mut finals: Vec<(String, i32, bool)> = Vec::new();
    let mut n_base = 0usize; let mut n_none = 0usize;
    for (k, e, van) in reg.iter() {
        let o = off_for(*van);
        let t = if vt != 0 { safe_read_i32(*e + vt).unwrap_or(-1) } else { -1 };
        if o == 0 { n_none += 1; continue; }
        match read_next_tier(*e, o) {
            Some(nt) if nt.is_empty() => {
                if built.contains(k) { finals.push((k.clone(), t, *van)); } else { n_base += 1; }
            }
            Some(_) => {}                 // 중간템(상위조합 존재)
            None => { n_none += 1; }       // 이 오프셋서 next_tier 아님(판정불가)
        }
    }
    let fv = finals.iter().filter(|(_, _, v)| *v).count();
    let fm = finals.len() - fv;
    s.push_str(&format!("  → 최종빌드 총 {}개 (바닐라 {} + 모드 {}) | 베이스컴포넌트제외={} 판정불가={}\n",
        finals.len(), fv, fm, n_base, n_none));
    s.push_str("  key | tier | 소속\n");
    for (k, t, van) in finals.iter() {
        s.push_str(&format!("  {} | t{} | {}\n", k, t, if *van { "바닐라" } else { "모드" }));
    }
    // 트리 상세(진단): 각 아이템의 next_tier 상태 — 오판 원인 검증용
    s.push_str("\n--- [트리 상세] key : next_tier ---\n");
    for (k, e, van) in reg.iter() {
        let o = off_for(*van);
        let st = if o == 0 { "판정불가(off=0)".to_string() } else {
            match read_next_tier(*e, o) {
                Some(nt) if nt.is_empty() => if built.contains(k) { "∅ ★최종".to_string() } else { "∅ (베이스컴포넌트)".to_string() },
                Some(nt) => format!("→ {}", nt.join(", ")),
                None => "NONE(이 오프셋서 읽기실패)".to_string(),
            }
        };
        s.push_str(&format!("  {} : {}\n", k, st));
    }

    s.push('\n');
    s.push_str(&dump_active_signatures(db));
    s.push_str("\n※ 여기 뜬 아이템 = 게임이 '활성'으로 Database 에 병합한 것과 동일.\n");
    s.push_str("  비활성 모드 아이템은 애초에 병합 안 돼 이 리스트에 없음(i18n 유무와 무관).\n");

    if let Some(p) = dump_path() { let _ = fs::write(p, &s); }
}

// ───────────────────────── 서버 확장 ─────────────────────────
fn db_base(ctx: &mut ServerModContext) -> usize {
    let cps = &ctx.database.champion_patch_statistics as *const _ as usize;
    cps.wrapping_sub(O_CPS)
}
fn run(ctx: &mut ServerModContext, where_: &str) {
    seh_install();
    let db = db_base(ctx);
    unsafe { dump_active_items(db, where_); }
}
struct ActiveProbeServerExt;
impl ModServerExtension for ActiveProbeServerExt {
    fn on_server_start(&self, ctx: &mut ServerModContext) { run(ctx, "on_server_start"); }
    fn before_management_tick(&self, ctx: &mut ServerModContext) { run(ctx, "before_management_tick"); }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_server_extension(ActiveProbeServerExt);
    reg
}
declare_mod!(init);
