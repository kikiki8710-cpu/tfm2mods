//! catalog.rs — 모드 아이템 카탈로그 + 최종템 판정 + 선택 저장 (item_tactics 엔진 이식)
//! ============================================================================
//! tfm2_4items 통합용. item_tactics `src/lib.rs`(scrim 유래)의 카탈로그·선택 로직을
//! 자족 모듈로 이식하되, **최종템 판정 버그(핸드오프 §3)를 수정**해 반영.
//!   - dump_mod_items: Database mod_items 배열 스캔 → MOD_REGISTRY/MOD_FINALS.
//!   - 최종템 = next_tier `Some(empty)` AND built_set 포함. (★None=판정불가는 제외 — 수정점)
//!   - mod_final_opts/item_opt_label: 활성모드 필터 + i18n 라벨.
//!   - SEL_BY_CHAMP: (챔프,slot)→옵션idx 영속(4items_sel.txt).
//! 주입경로(c6 등)와 독립 — 드롭다운 옵션·선택 저장만 담당.
#![allow(dead_code, unused_imports, unused_variables)]
use mod_api::*;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub const MOD_ID: &str = "tfm2_4items";
pub const LOG_ENABLED: bool = true; // 이식검증용 로그(프로덕션서 false)

// 바닐라 7옵션 라벨 (idx 0~6). 게임 personal_tactics ItemBuildOverride 와 1:1.
pub const VANILLA_OPTS: [&str; 7] = ["선수에게 맡김", "공격력", "주문력", "공격 속도", "방어력", "마법 저항력", "체력"];
// 바닐라 카테고리(1~6) → 최종템 게임 ID (게임 c6 JT 변환과 동일, 0.4.14).
pub const VANILLA_FINAL: [u64; 6] = [4, 24, 9, 14, 19, 29];

// ============================================================================
//  WinAPI FFI
// ============================================================================
type HMODULE = isize;
type VehHandler = extern "system" fn(*mut ExceptionPointers) -> i32;
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> HMODULE;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, n: u32) -> u32;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn AddVectoredExceptionHandler(first: u32, handler: VehHandler) -> usize;
    fn GetCurrentThreadId() -> u32;
}
#[repr(C)]
#[derive(Default)]
struct MemBasicInfo {
    base: usize, alloc_base: usize, alloc_protect: u32, _pad0: u32,
    region_size: usize, state: u32, protect: u32, mtype: u32, _pad1: u32,
}

pub unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const MEM_COMMIT: u32 = 0x1000;
    const READABLE: u32 = 0x02 | 0x04 | 0x20 | 0x40;
    const NOACCESS_GUARD: u32 = 0x01 | 0x100;
    if mbi.state != MEM_COMMIT { return false; }
    if mbi.protect & NOACCESS_GUARD != 0 { return false; }
    if mbi.protect & READABLE == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}
fn looks_heap(v: u64) -> bool { v & 0x7 == 0 && v >= 0x10000 && v < 0x0000_8000_0000_0000 && (v & 0xffff) != 0 }

// ============================================================================
//  SEH 안전읽기 — VEH 로 0xC0000005 가로채 크래시 대신 실패반환.
//  SEH[]: 0=active 1=tid 2=land_rip 3=land_rsp 4=land_rbp 5=code_lo 6=code_hi 7=faults
// ============================================================================
#[repr(C)]
struct ExceptionRecord { code: u32, flags: u32, rec: usize, addr: usize, nparams: u32, _p: u32, params: [usize; 15] }
#[repr(C)]
struct ExceptionPointers { rec: *mut ExceptionRecord, ctx: *mut core::ffi::c_void }

static mut SEH: [u64; 8] = [0u64; 8];
static SEH_INSTALLED: AtomicBool = AtomicBool::new(false);
static SEH_BUSY: AtomicBool = AtomicBool::new(false);

extern "system" fn seh_veh(p: *mut ExceptionPointers) -> i32 {
    const CONTINUE_EXECUTION: i32 = -1;
    const CONTINUE_SEARCH: i32 = 0;
    unsafe {
        if p.is_null() { return CONTINUE_SEARCH; }
        let rec = (*p).rec;
        if rec.is_null() { return CONTINUE_SEARCH; }
        if (*rec).code != 0xC0000005 { return CONTINUE_SEARCH; }
        let g = core::ptr::addr_of!(SEH) as *const u64;
        if *g.add(0) == 0 { return CONTINUE_SEARCH; }
        if *g.add(1) != GetCurrentThreadId() as u64 { return CONTINUE_SEARCH; }
        let ctx = (*p).ctx as usize;
        if ctx == 0 { return CONTINUE_SEARCH; }
        let rip = *((ctx + 0xF8) as *const u64);
        if rip < *g.add(5) || rip >= *g.add(6) { return CONTINUE_SEARCH; }
        *((ctx + 0xF8) as *mut u64) = *g.add(2); // Rip = land_rip
        *((ctx + 0x98) as *mut u64) = *g.add(3); // Rsp = land_rsp
        *((ctx + 0xA0) as *mut u64) = *g.add(4); // Rbp = land_rbp
        let gm = core::ptr::addr_of_mut!(SEH) as *mut u64;
        *gm.add(7) += 1;
        CONTINUE_EXECUTION
    }
}
pub fn seh_install() {
    if SEH_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    unsafe { AddVectoredExceptionHandler(1, seh_veh); }
}
#[inline(never)]
unsafe fn safe_copy(dst: *mut u8, src: *const u8, len: usize) -> bool {
    if !SEH_INSTALLED.load(Ordering::Relaxed) { return false; }
    while SEH_BUSY.swap(true, Ordering::Acquire) { core::hint::spin_loop(); }
    let g = core::ptr::addr_of_mut!(SEH) as *mut u64;
    *g.add(1) = GetCurrentThreadId() as u64;
    let mut ok: u64;
    core::arch::asm!(
        "lea rax, [rip + 200f]", "mov [{g} + 40], rax",
        "lea rax, [rip + 201f]", "mov [{g} + 48], rax",
        "lea rax, [rip + 202f]", "mov [{g} + 16], rax",
        "mov [{g} + 24], rsp", "mov [{g} + 32], rbp",
        "mov qword ptr [{g} + 0], 1", "cld",
        "200:", "rep movsb", "201:", "mov {ok}, 1", "jmp 203f",
        "202:", "mov {ok}, 0", "203:", "mov qword ptr [{g} + 0], 0",
        g = in(reg) g, ok = out(reg) ok,
        inout("rcx") len => _, inout("rdi") dst => _, inout("rsi") src => _, out("rax") _,
    );
    SEH_BUSY.store(false, Ordering::Release);
    ok != 0
}
unsafe fn safe_read_u64(addr: usize) -> Option<u64> {
    let mut b = [0u8; 8];
    if safe_copy(b.as_mut_ptr(), addr as *const u8, 8) { Some(u64::from_le_bytes(b)) } else { None }
}
unsafe fn safe_read_bytes(addr: usize, len: usize, out: &mut Vec<u8>) -> bool {
    if len == 0 || len > 4096 { return false; }
    out.clear(); out.resize(len, 0);
    safe_copy(out.as_mut_ptr(), addr as *const u8, len)
}

// ============================================================================
//  경로 / 로깅
// ============================================================================
fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }
fn exe_path() -> Option<PathBuf> {
    let mut buf = vec![0u16; 1024];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n == 0 || n >= buf.len() { return None; }
    Some(PathBuf::from(String::from_utf16_lossy(&buf[..n])))
}
fn game_root() -> Option<PathBuf> { exe_path()?.parent().map(|p| p.to_path_buf()) }
fn mod_dir() -> Option<PathBuf> { Some(game_root()?.join("mods").join(MOD_ID)) }
fn write_log(name: &str, content: &str) {
    if !LOG_ENABLED { return; }
    if let Some(d) = mod_dir() { let _ = fs::write(d.join(name), content); }
}

// ============================================================================
//  JSON 파서 (mods.json / item.i18n)
// ============================================================================
enum JsonValue { Null, Bool(bool), Num(f64), Str(String), Arr(Vec<JsonValue>), Obj(Vec<(String, JsonValue)>) }
impl JsonValue {
    fn as_obj(&self) -> Option<&Vec<(String, JsonValue)>> { if let JsonValue::Obj(o) = self { Some(o) } else { None } }
    fn get<'b>(&'b self, key: &str) -> Option<&'b JsonValue> { self.as_obj()?.iter().find(|(k, _)| k == key).map(|(_, v)| v) }
    fn as_str(&self) -> Option<&str> { if let JsonValue::Str(s) = self { Some(s.as_str()) } else { None } }
}
struct JsonParser<'a> { b: &'a [u8], i: usize }
impl<'a> JsonParser<'a> {
    fn new(s: &'a str) -> Self { JsonParser { b: s.as_bytes(), i: 0 } }
    fn skip_ws(&mut self) {
        while self.i < self.b.len() {
            match self.b[self.i] { b' ' | b'\t' | b'\r' | b'\n' | b',' => self.i += 1, _ => break }
        }
    }
    fn parse_value(&mut self) -> Option<JsonValue> {
        self.skip_ws();
        if self.i >= self.b.len() { return None; }
        match self.b[self.i] {
            b'{' => self.parse_object(),
            b'[' => self.parse_array(),
            b'"' => self.parse_string().map(JsonValue::Str),
            b't' => { self.i += 4; Some(JsonValue::Bool(true)) }
            b'f' => { self.i += 5; Some(JsonValue::Bool(false)) }
            b'n' => { self.i += 4; Some(JsonValue::Null) }
            _ => self.parse_number(),
        }
    }
    fn parse_string(&mut self) -> Option<String> {
        if self.b.get(self.i) != Some(&b'"') { return None; }
        self.i += 1;
        let mut out: Vec<u8> = Vec::new();
        while self.i < self.b.len() {
            let c = self.b[self.i]; self.i += 1;
            match c {
                b'"' => return Some(String::from_utf8_lossy(&out).into_owned()),
                b'\\' => {
                    let e = *self.b.get(self.i)?; self.i += 1;
                    match e {
                        b'n' => out.push(b'\n'), b't' => out.push(b'\t'), b'r' => out.push(b'\r'),
                        b'b' => out.push(0x08), b'f' => out.push(0x0c), b'"' => out.push(b'"'),
                        b'\\' => out.push(b'\\'), b'/' => out.push(b'/'),
                        b'u' => {
                            if self.i + 4 <= self.b.len() {
                                if let Ok(hex) = std::str::from_utf8(&self.b[self.i..self.i + 4]) {
                                    if let Ok(cp) = u32::from_str_radix(hex, 16) {
                                        if let Some(ch) = char::from_u32(cp) {
                                            let mut buf = [0u8; 4];
                                            out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                                        }
                                    }
                                }
                                self.i += 4;
                            }
                        }
                        other => out.push(other),
                    }
                }
                _ => out.push(c),
            }
        }
        None
    }
    fn parse_number(&mut self) -> Option<JsonValue> {
        let start = self.i;
        while self.i < self.b.len() {
            match self.b[self.i] { b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E' => self.i += 1, _ => break }
        }
        let tok = std::str::from_utf8(&self.b[start..self.i]).ok()?;
        tok.parse::<f64>().ok().map(JsonValue::Num)
    }
    fn parse_array(&mut self) -> Option<JsonValue> {
        self.i += 1;
        let mut arr = Vec::new();
        loop {
            self.skip_ws();
            if self.i >= self.b.len() { return None; }
            if self.b[self.i] == b']' { self.i += 1; break; }
            arr.push(self.parse_value()?);
        }
        Some(JsonValue::Arr(arr))
    }
    fn parse_object(&mut self) -> Option<JsonValue> {
        self.i += 1;
        let mut pairs = Vec::new();
        loop {
            self.skip_ws();
            if self.i >= self.b.len() { return None; }
            if self.b[self.i] == b'}' { self.i += 1; break; }
            let k = self.parse_string()?;
            self.skip_ws();
            if self.b.get(self.i) != Some(&b':') { return None; }
            self.i += 1;
            let v = self.parse_value()?;
            pairs.push((k, v));
        }
        Some(JsonValue::Obj(pairs))
    }
}

// ============================================================================
//  모드 아이템 레지스트리 (dump_mod_items 가 서버시작때 1회 채움)
// ============================================================================
static MOD_REGISTRY: Mutex<Vec<String>> = Mutex::new(Vec::new()); // idx i → key (게임 ID = 30+i)
static MOD_FINALS: Mutex<Vec<u64>> = Mutex::new(Vec::new());       // next_tier 빈 최종템 ID
static MOD_BUF: AtomicU64 = AtomicU64::new(0);
static MOD_STRIDE: AtomicU64 = AtomicU64::new(0);
static NT_OFFSET: AtomicUsize = AtomicUsize::new(0);
static MODITEMS_DONE: AtomicBool = AtomicBool::new(false);
pub fn catalog_ready() -> bool { MODITEMS_DONE.load(Ordering::Relaxed) && !MOD_REGISTRY.lock().unwrap_or_else(|e| e.into_inner()).is_empty() }

const VANILLA_KEYS: [&str; 30] = [
    "iron_blade","soldiers_longsword","ruinous_blade","conquerors_greatsword","warlords_final_judgement",
    "dagger","wind_dagger","twin_stormblade","thunderclaw","storm_sovereign",
    "steel_armor","gatekeepers_armor","black_knights_heavy_plate","eternal_iron_plate","impregnable_fortress",
    "mystic_cloak","night_hood","dusk_raven","souls_edge","veil_of_annihilation",
    "arcane_crystal","spirit_crystal","staff_of_rapture","angels_fang","prophet_of_the_abyss",
    "vital_orb","hardened_heart","ring_of_reincarnation","hourglass_of_eternity","giants_horn_shard",
];

// Database mod_items Vec 를 메모리 스캔 → MOD_REGISTRY/MOD_FINALS 채움.
pub unsafe fn dump_mod_items(db: usize) {
    if MODITEMS_DONE.swap(true, Ordering::Relaxed) { return; }
    seh_install();
    let mut s = format!("[{}ms] mod_items walk (db={:#x})\n", now_ms(), db);

    let key_at = |pa: usize| -> Option<String> {
        let ptr = safe_read_u64(pa)? as usize;
        if ptr <= 0x10000 { return None; }
        for &m in &[64usize, 32, 16, 8] {
            let mut b = Vec::new();
            if !safe_read_bytes(ptr, m, &mut b) { continue; }
            let mut v = Vec::new();
            for &c in b.iter() { if c == b'_' || c.is_ascii_alphanumeric() { v.push(c); } else { break; } }
            if v.len() >= 3 && (v[0] as char).is_ascii_alphabetic() { return String::from_utf8(v).ok(); }
        }
        None
    };
    let is_vanilla = |k: &str| k == "ironsword" || VANILLA_KEYS.contains(&k);
    let item_strides: [usize; 3] = [0x1a8, 0x198, 0x1b0];
    let detect_stride = |buf: usize| -> usize {
        for &st in item_strides.iter() {
            let k: Vec<Option<String>> = (0..4).map(|i| key_at(buf + i * st + 0x8)).collect();
            if k.iter().all(|x| x.is_some()) && k[0] != k[1] && k[1] != k[2] && k[2] != k[3] { return st; }
        }
        0
    };
    let mut found: Vec<(usize, usize, usize, usize)> = Vec::new();
    let mut o = 0usize;
    while o + 0x18 <= 0x60000 && found.len() < 16 {
        let a = db + o; o += 8;
        let (Some(q0), Some(q1), Some(q2)) = (safe_read_u64(a), safe_read_u64(a + 8), safe_read_u64(a + 0x10)) else { continue; };
        for &(p, c) in [(q1, q0), (q1, q2), (q0, q2), (q0, q1)].iter() {
            let (p, c) = (p as usize, c as usize);
            if !looks_heap(p as u64) || c < 3 || c > 2000 { continue; }
            let Some(k0) = key_at(p + 0x8) else { continue; };
            if is_vanilla(&k0) { continue; }
            let cst = detect_stride(p);
            if cst == 0 { continue; }
            let probe = c.min(48);
            let valid = (0..probe).filter(|&i| key_at(p + i * cst + 0x8).is_some()).count();
            if valid * 10 < probe * 8 || valid < 3 { continue; }
            if found.iter().any(|&(b, _, _, _)| b == p) { continue; }
            found.push((p, c, cst, a));
        }
    }
    if found.is_empty() {
        s.push_str("  ✗ 비바닐라 item-struct 배열 못 찾음 (모드 아이템 미적용?)\n");
        write_log("4items_moditems.txt", &s); return;
    }
    found.sort_by(|x, y| y.1.cmp(&x.1));
    let key_of_elem = |elem: usize| -> Option<String> {
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
    };
    // read_nt: elem 의 next_tier Vec(오프셋 o) 를 key 리스트로 읽음. None=그 오프셋서 next_tier 아님.
    let read_nt = |elem: usize, o: usize| -> Option<Vec<String>> {
        let len = safe_read_u64(elem + o)? as usize;
        if len == 0 { return Some(Vec::new()); }
        if len > 8 { return None; }
        let ptr = safe_read_u64(elem + o + 8)? as usize;
        let cap = safe_read_u64(elem + o + 0x10)? as usize;
        if ptr <= 0x10000 || cap < len { return None; }
        let mut out = Vec::new();
        for j in 0..len { out.push(key_of_elem(ptr + j * 0x18)?); }
        Some(out)
    };
    let build_keys = |buf: usize, st: usize, hdr_cnt: usize| -> Vec<String> {
        let mut keys = Vec::new();
        let mut cnt = 0usize;
        while cnt < hdr_cnt.max(1) && cnt < 500 {
            if let Some(k) = key_of_elem(buf + cnt * st) { keys.push(k); cnt += 1; } else { break; }
        }
        keys
    };
    let best_nt = |buf: usize, st: usize, keys: &[String]| -> (usize, u32) {
        let mut best_off = 0usize; let mut best_votes = 0u32;
        let mut o = 0x18usize;
        while o + 0x18 <= st {
            let mut votes = 0u32;
            for i in 0..keys.len() {
                if let Some(v) = read_nt(buf + i * st, o) {
                    if !v.is_empty() && v.iter().all(|k| keys.iter().any(|x| x.as_str() == k.as_str())) { votes += 1; }
                }
            }
            if votes > best_votes { best_votes = votes; best_off = o; }
            o += 8;
        }
        (best_off, best_votes)
    };
    let mut diag = String::from("  --- 후보 스캔(전부) ---\n");
    let mut chosen: Option<(usize, usize, Vec<String>, usize, u32)> = None;
    for &(fbuf, fcnt, fst, _) in &found {
        let keys = build_keys(fbuf, fst, fcnt);
        let (bo, bv) = best_nt(fbuf, fst, &keys);
        diag.push_str(&format!("  buf={:#x} cnt={} stride={:#x} first={:?} nt_off={:#x} votes={}\n",
            fbuf, keys.len(), fst, keys.first(), bo, bv));
        if bv >= 3 && chosen.is_none() { chosen = Some((fbuf, fst, keys, bo, bv)); }
    }
    let Some((buf, st, keys, best_off, best_votes)) = chosen else {
        s.push_str("  ✗ 아이템 트리(next_tier) 가진 배열 없음 → 아이템 모드 미로드/미인식 의심\n");
        s.push_str(&diag);
        write_log("4items_moditems.txt", &s);
        return;
    };
    let cnt = keys.len();
    MOD_BUF.store(buf as u64, Ordering::Relaxed); MOD_STRIDE.store(st as u64, Ordering::Relaxed);
    NT_OFFSET.store(best_off, Ordering::Relaxed);
    {
        let mut reg = MOD_REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
        reg.clear();
        for k in keys.iter() { reg.push(k.clone()); }
    }
    s.push_str(&format!("  [채택] buf={:#x} cnt={} stride={:#x} nt_off={:#x} votes={}\n  idx | ID | key\n", buf, cnt, st, best_off, best_votes));
    for (i, k) in keys.iter().enumerate() { s.push_str(&format!("  {:>3} | {:>3} | {}\n", i, 30 + i, k)); }
    s.push_str(&diag);
    write_log("4items_moditems.txt", &s);
    // built_set = 모든 next_tier 타겟 합집합 (베이스컴포넌트 배제용).
    let mut built: HashSet<String> = HashSet::new();
    for i in 0..cnt {
        if let Some(nt) = read_nt(buf + i * st, best_off) { for k in nt { built.insert(k); } }
    }
    let mut finals: Vec<u64> = Vec::new();
    let mut tree = format!("[{}ms] next_tier offset=+{:#x} votes={}/{} built_targets={}\n", now_ms(), best_off, best_votes, cnt, built.len());
    for i in 0..cnt {
        let elem = buf + i * st;
        let k = key_of_elem(elem).unwrap_or_default();
        // ★ 핸드오프 §3 수정: read_nt 를 match 로 분기. None(판정불가)=최종에서 제외
        //   (기존 unwrap_or_default() 는 None 을 빈Vec 으로 오인 → 최종템 오판. override 시 실제 발생.)
        match read_nt(elem, best_off) {
            Some(nt) if nt.is_empty() => {
                if built.contains(&k) { finals.push(30 + i as u64); tree.push_str(&format!("  {:>3} {} ★최종\n", 30 + i, k)); }
                else { tree.push_str(&format!("  {:>3} {} (베이스컴포넌트-제외)\n", 30 + i, k)); }
            }
            Some(nt) => { tree.push_str(&format!("  {:>3} {} → {}\n", 30 + i, k, nt.join(", "))); }
            None => { tree.push_str(&format!("  {:>3} {} (next_tier 판정불가-제외)\n", 30 + i, k)); }
        }
    }
    tree.push_str(&format!("  → 최종템 {}개: {:?}\n", finals.len(), finals));
    *MOD_FINALS.lock().unwrap_or_else(|e| e.into_inner()) = finals;
    write_log("4items_itemtree.txt", &tree);
}

// ============================================================================
//  모드 최종템 옵션 (DB 스캔 결과 그대로 — i18n 활성필터 폐기)
// ============================================================================
fn mod_final_opts_all() -> Vec<(u64, String)> {
    let finals = MOD_FINALS.lock().unwrap_or_else(|e| e.into_inner());
    let reg = MOD_REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    finals.iter().filter_map(|&id| {
        let i = (id as usize).checked_sub(30)?;
        reg.get(i).map(|k| (id, k.clone()))
    }).collect()
}
// ★ 활성 모드템 = DB 스캔 결과 그대로. (HANDOFF_최종템판정 / tfm2_active_probe 방법:
//   "게임 메모리가 곧 정답" — 비활성 모드 아이템은 Database 통합 컬렉션에 병합 안 돼
//   애초에 dump_mod_items 스캔에 안 잡힘. 구 enabled_mods×i18n 교차 = 폐기.)
pub fn mod_final_opts() -> Vec<(u64, String)> { mod_final_opts_all() }
pub fn item_opt_count() -> usize { 7 + mod_final_opts().len() }
pub fn item_opt_label(v: u8) -> String {
    let vi = v as usize;
    if vi < 7 { return VANILLA_OPTS[vi].to_string(); }
    match mod_final_opts().get(vi - 7) {
        Some((_, key)) => format!("#asset/base/text/item?{}.name", key),
        None => VANILLA_OPTS[0].to_string(),
    }
}
pub fn compute_options() -> Vec<String> {
    let n = item_opt_count();
    (0..n).map(|i| item_opt_label(i as u8)).collect()
}
// 옵션 idx → 게임 아이템 ID. 0=자동(None), 1~6=바닐라 카테고리 최종템, 7+=모드 최종템(30+).
pub fn commit_to_id(idx: u8) -> Option<u64> {
    if idx == 0 { return None; }
    if (1..=6).contains(&idx) { return Some(VANILLA_FINAL[(idx - 1) as usize]); }
    mod_final_opts().get(idx as usize - 7).map(|(id, _)| *id)
}

// ============================================================================
//  선택 저장 — (챔프키, slot) → 옵션 idx. 영속(4items_sel.txt).
// ============================================================================
static SEL_BY_CHAMP: Mutex<Option<HashMap<(String, u8), u8>>> = Mutex::new(None);
static SEL_LOADED: AtomicBool = AtomicBool::new(false);
fn sel_path() -> Option<PathBuf> { Some(mod_dir()?.join("4items_sel.txt")) }
fn load_sel() -> HashMap<(String, u8), u8> {
    let mut m = HashMap::new();
    if let Some(p) = sel_path() {
        if let Ok(txt) = fs::read_to_string(&p) {
            for line in txt.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() == 3 {
                    if let (Ok(slot), Ok(idx)) = (parts[1].parse::<u8>(), parts[2].parse::<u8>()) {
                        m.insert((parts[0].to_string(), slot), idx);
                    }
                }
            }
        }
    }
    m
}
pub fn save_sel(m: &HashMap<(String, u8), u8>) {
    let mut s = String::new();
    let mut v: Vec<_> = m.iter().collect();
    v.sort_by(|a, b| a.0.cmp(b.0));
    for ((champ, slot), idx) in v { s.push_str(&format!("{} {} {}\n", champ, slot, idx)); }
    if let Some(p) = sel_path() { let _ = fs::write(p, s); }
}
pub fn with_sel<R>(f: impl FnOnce(&mut HashMap<(String, u8), u8>) -> R) -> R {
    let mut g = SEL_BY_CHAMP.lock().unwrap_or_else(|e| e.into_inner());
    if g.is_none() { *g = Some(load_sel()); SEL_LOADED.store(true, Ordering::Relaxed); }
    f(g.as_mut().unwrap())
}

// ============================================================================
//  UI 노드 헬퍼 — 챔프키 추출(#icon ImageRunner source)
// ============================================================================
unsafe fn runner_base(n: &Node) -> usize {
    let any: &dyn Any = n.runner.as_any();
    let parts: [usize; 2] = std::mem::transmute::<*const dyn Any, [usize; 2]>(any as *const dyn Any);
    parts[0]
}
pub fn find_node<'a>(n: &'a Node, t: &str) -> Option<&'a Node> {
    if n.id.as_str() == t { return Some(n); }
    for c in n.child.iter() { if let Some(x) = find_node(c, t) { return Some(x); } }
    None
}
unsafe fn read_img_source(n: &Node) -> Option<String> {
    if !n.runner.type_name().contains("ImageRunner") { return None; }
    let dp = runner_base(n);
    let len = std::ptr::read_unaligned(dp as *const u64) as usize;
    let ptr = std::ptr::read_unaligned((dp + 8) as *const u64) as *const u8;
    if ptr.is_null() || len == 0 || len > 512 || !readable(ptr as usize, len) { return None; }
    Some(String::from_utf8_lossy(std::slice::from_raw_parts(ptr, len)).to_string())
}
// #icon ImageRunner source "asset/base/aseprite_resources/champions/{champ}#sheet" → champ
pub fn row_champ(row: &Node) -> Option<String> {
    let icon = find_node(row, "icon")?;
    let src = unsafe { read_img_source(icon) }?;
    let a = src.find("champions/")? + "champions/".len();
    let rest = &src[a..];
    let end = rest.find('#').unwrap_or(rest.len());
    let champ = rest[..end].trim();
    if champ.is_empty() { None } else { Some(champ.to_string()) }
}
