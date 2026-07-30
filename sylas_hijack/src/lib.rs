// sylas_hijack — W31: 데이터챔프 master record 강탈 (clone캡처 + factory rcx-swap, shadow-call 없음)
// ★W30 실측 확정: FUN_141c8ad20(rcx=out 임시버퍼, rdx=src=영속 master record 0x748, +0x4c8 궁 descriptor)
//   = 경기 로드 시 전체 데이터챔프 21개(cirno 포함) 일괄 clone. src=챔프별 고유·영속. src+0x4c8=0x3(궁 descriptor 첫필드, -1 아님=factory-valid).
// ★W31: clone서 (id→src) 캡처 → 사일러스 궁 factory 시점 rcx를 타깃 master로 스왑(W4 메커니즘).
//   전부 VEH-safe 읽기 + Rust HashMap. 게임 map shadow-call 없음. +0x4c8 검증 후에만 스왑.
// ★검증: cirno가 경기에 없어도 사일러스가 cirno 궁 시전 = replica 접근(경기 밖 정의 훔치기) 확정.

#![allow(non_snake_case, dead_code)]
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;
use std::collections::HashMap;
use std::time::Duration;
use mod_api::*;

const CLONE_RVA: usize = 0x1c8ad20;
const CLONE_SIG: [u8; 12] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];
const CLONE_LEN: usize = 12;
const FACTORY_RVA: usize = 0x1c279d0;
const FACTORY_SIG: [u8; 17] = [0x55,0x56,0x48,0x81,0xec,0x98,0x01,0x00,0x00,0x48,0x8d,0xac,0x24,0x80,0x00,0x00,0x00];
const FACTORY_LEN: usize = 17;
const B_IDPTR: usize = 0x8;
const B_IDLEN: usize = 0x10;
const E_ULT_DESC: usize = 0x4c8;
const DEFAULT_TARGET: &str = "pythoness_r"; // W32a: 우리가 저작한 바닐라 replica
// ── 5번째 길(W36): 네이티브 apply 리다이렉트 (datachamp apply → gambler 네이티브 apply) ──
//   datachamp apply(0x1c90c20) 진입점서 사일러스 궁이면 gambler apply(0x1d10190)를 같은 살아있는
//   인자(rdx=caster_handle,r8=sim_state,r9=target,casting_ctx)로 직접 CALL → 네이티브 gambler 궁 실행.
const DATAPPLY_RVA: usize = 0x1c90c20;
const DATAPPLY_SIG: [u8; 12] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];
const DATAPPLY_LEN: usize = 12;
const GAMBLER_APPLY_RVA: usize = 0x1d10190;
// gambler descriptor 0x50 POD (덤프 실측 10워드; apply가 값만 읽음=vtable 불필요).
static mut GAMBLER_DESC: [u64; 10] = [50, 60, 36, 100_000, 35_000, 2_000, 3_600, 28, 20, 0];
static NATIVE_MODE: AtomicBool = AtomicBool::new(false);
static NAT_LOG: AtomicU32 = AtomicU32::new(0);
type ApplyFn = unsafe extern "C" fn(usize, usize, usize, usize, usize) -> u64;
const HANDLED_BIT: u64 = 1u64 << 32;
// ── effect 트리 덤프 프로브(cfg dump=1) ── 챔프 궁 시전 시 실행 effect 구조 덤프 ──
const CASTDUMP_RVA: usize = 0x1e3e550; // cast-apply, rcx=caster entity
const CASTDUMP_SIG: [u8; 12] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];
const CASTDUMP_LEN: usize = 12;
const E_NAMEPTR: usize = 0x250;
const E_NAMELEN: usize = 0x258;
const E_STATE: usize = 0x68;

fn rva_of(a: u64) -> u64 { let b = exe_base() as u64; if a >= b && a < b + 0x8000000 { a - b } else { a } }

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
    fn QueryPerformanceFrequency(f: *mut i64) -> BOOL;
}
// ★전투 게이트: sim tick(0x1e8c640) 진입 시각을 TLS에 기록 → factory는 최근 틱 있을 때만 rcx-swap.
//   로드/드래프트/스폰(틱 밖) 시엔 스왑 안 함 → 복잡한 replica 궁을 로드때 반복빌드하다 행나는 것 방지.
const TICK_RVA: usize = 0x1e8c640;
const TICK_SIG: [u8; 12] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];
const TICK_LEN: usize = 12;
static QPF: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[inline] fn qpc_now() -> i64 { let mut c=0i64; unsafe { QueryPerformanceCounter(&mut c); } c }
thread_local! { static TICK_TS: core::cell::Cell<i64> = core::cell::Cell::new(0); }
// 사일러스 궁 descriptor 캐시 → datachamp apply서 rcx가 정확히 이거일 때만 리다이렉트(궁만, 평타/스킬 배제).
thread_local! { static SYLAS_ULT_TS: core::cell::Cell<i64> = core::cell::Cell::new(0); }
thread_local! { static SYLAS_ULT_DESC: core::cell::Cell<usize> = core::cell::Cell::new(0); }
fn in_combat_tick() -> bool {
    let f = QPF.load(Ordering::Relaxed);
    if f == 0 { return false; }
    let ts = TICK_TS.with(|c| c.get());
    if ts == 0 { return false; }
    let age_us = (qpc_now() - ts).saturating_mul(1_000_000) / f as i64;
    age_us >= 0 && age_us < 50_000 // 최근 50ms 내 틱 = 전투 중
}
unsafe fn tick_common(saved: usize) {
    let _ = saved; // rcx=GameData(미사용, 존재만 확인) — 시각만 기록
    TICK_TS.with(|c| c.set(qpc_now()));
}
#[repr(C)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_prot: u32, _pad0: u32,
    region: usize, state: u32, protect: u32, typ: u32 }
#[repr(C)] struct ExceptionRecord { code: u32, flags: u32, _rec: usize, addr: usize, _np: u32, _pad: u32 }
#[repr(C)] struct ExceptionPointers { rec: *mut ExceptionRecord, ctx: usize }
type VehHandler = extern "system" fn(*mut ExceptionPointers) -> i32;

fn exe_base() -> usize { unsafe { GetModuleHandleW(core::ptr::null()) } }

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
    ".globl shj_rd8", ".globl shj_rd8_f", ".globl shj_rd8_l",
    ".globl shj_rd1", ".globl shj_rd1_f", ".globl shj_rd1_l",
    "shj_rd8:", "shj_rd8_f:", "mov rax, qword ptr [rcx]", "mov qword ptr [rdx], rax", "mov eax, 1", "ret",
    "shj_rd8_l:", "xor eax, eax", "ret",
    "shj_rd1:", "shj_rd1_f:", "movzx eax, byte ptr [rcx]", "mov byte ptr [rdx], al", "mov eax, 1", "ret",
    "shj_rd1_l:", "xor eax, eax", "ret",
);
extern "C" {
    fn shj_rd8(addr: usize, out: *mut u64) -> u32;
    fn shj_rd1(addr: usize, out: *mut u8) -> u32;
    static shj_rd8_f: u8; static shj_rd8_l: u8;
    static shj_rd1_f: u8; static shj_rd1_l: u8;
}
#[inline] unsafe fn rd_u64(a: usize) -> Option<u64> { if a < 0x10000 { return None; } let mut o=0u64; if shj_rd8(a,&mut o)!=0 {Some(o)} else {None} }
#[inline] unsafe fn rd_u8(a: usize)  -> Option<u8>  { if a < 0x10000 { return None; } let mut o=0u8;  if shj_rd1(a,&mut o)!=0 {Some(o)} else {None} }
unsafe fn in_exe(a: u64) -> bool { let b = exe_base() as u64; a >= b && a < b + 0x4000000 }

static VEH_INSTALLED: AtomicBool = AtomicBool::new(false);
extern "system" fn shj_veh(p: *mut ExceptionPointers) -> i32 {
    const CONTINUE_EXECUTION: i32 = -1; const CONTINUE_SEARCH: i32 = 0;
    unsafe {
        if p.is_null() { return CONTINUE_SEARCH; }
        let rec = (*p).rec;
        if rec.is_null() || (*rec).code != 0xC0000005 { return CONTINUE_SEARCH; }
        let ctx = (*p).ctx;
        if ctx == 0 { return CONTINUE_SEARCH; }
        let rip = *((ctx + 0xF8) as *const u64) as usize;
        let land = if rip == core::ptr::addr_of!(shj_rd8_f) as usize { core::ptr::addr_of!(shj_rd8_l) as usize }
                   else if rip == core::ptr::addr_of!(shj_rd1_f) as usize { core::ptr::addr_of!(shj_rd1_l) as usize }
                   else { 0 };
        if land != 0 { *((ctx + 0xF8) as *mut u64) = land as u64; return CONTINUE_EXECUTION; }
        CONTINUE_SEARCH
    }
}
fn veh_install() { if VEH_INSTALLED.swap(true, Ordering::Relaxed) { return; } unsafe { AddVectoredExceptionHandler(1, shj_veh); } }

unsafe fn read_str_len(ptr: usize, len: usize) -> Option<String> {
    if ptr < 0x10000 || len < 2 || len > 32 { return None; }
    let mut buf = Vec::with_capacity(len);
    for i in 0..len {
        let b = rd_u8(ptr + i)?;
        if !(b == b'_' || b.is_ascii_alphanumeric()) { return None; }
        buf.push(b);
    }
    String::from_utf8(buf).ok()
}
unsafe fn read_id(rec: usize) -> Option<String> {
    let idptr = rd_u64(rec + B_IDPTR)? as usize;
    let idlen = rd_u64(rec + B_IDLEN)? as usize;
    read_str_len(idptr, idlen)
}

static WRITE_LOCK: Mutex<()> = Mutex::new(());
fn write_log(s: &str) {
    use std::io::Write;
    let _g = WRITE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true)
        .open("C:\\tfm2mods\\sylas_hijack\\hijack_log.txt") {
        let _ = f.write_all(s.as_bytes());
    }
}

// ── arm 게이트 ───────────────────────────────────────────────────────────────
static ARMED: AtomicBool = AtomicBool::new(false);
static DUMP_MODE: AtomicBool = AtomicBool::new(false);
// cfg `target=<id>` 로 시전할 replica를 런타임 지정(rebuild 없이 60종 테스트). 미지정 시 DEFAULT_TARGET.
static TARGET: Mutex<Option<String>> = Mutex::new(None);
fn cur_target() -> String {
    TARGET.lock().unwrap_or_else(|e| e.into_inner())
        .clone().filter(|s| !s.is_empty())
        .unwrap_or_else(|| DEFAULT_TARGET.to_string())
}
fn arm_refresher() {
    std::thread::spawn(|| loop {
        let s = std::fs::read_to_string("C:\\tfm2mods\\sylas_hijack\\sylas_hijack.cfg").unwrap_or_default();
        ARMED.store(s.contains("arm=1"), Ordering::Relaxed);
        DUMP_MODE.store(s.contains("dump=1"), Ordering::Relaxed);
        NATIVE_MODE.store(s.contains("native=1"), Ordering::Relaxed); // 5번째 길: 네이티브 gambler 강탈
        // target=<id> 파싱(공백/CR 제거)
        let t = s.lines().find_map(|l| l.trim().strip_prefix("target=").map(|v| v.trim().to_string()));
        *TARGET.lock().unwrap_or_else(|e| e.into_inner()) = t;
        std::thread::sleep(Duration::from_secs(2));
    });
}

// ── effect 덤프: 챔프 궁 시전 시 casting-block 실행 effect 구조를 로깅 ──────────
unsafe fn champ_name(ent: usize) -> Option<String> {
    let nptr = rd_u64(ent + E_NAMEPTR)? as usize;
    let nlen = rd_u64(ent + E_NAMELEN)? as usize;
    read_str_len(nptr, nlen)
}
// (last_ult_ptr, ult재덤프 횟수). 궁 슬롯은 시전마다 재빌드되므로 포인터 변화 = 신선한 궁 트리.
static DUMP_SEEN: Mutex<Option<std::collections::HashMap<String, (u64, u32)>>> = Mutex::new(None);
fn dump_log(s: &str) {
    use std::io::Write;
    let _g = WRITE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true)
        .open("C:\\tfm2mods\\sylas_hijack\\effect_dump.txt") {
        let _ = f.write_all(s.as_bytes());
    }
}
#[inline] fn is_heap(x: u64) -> bool { x >= 0x10000000000 && x < 0x800000000000 }
// ── 방법 B: 실행 effect-list 캡처(바닐라 궁 실제 트리) ────────────────────────
// castapply(FUN_141e3e550) 규명: effect-list = entity+0x290 Vec<CastedEffect stride 0x38>
//   (cap@+0x290 / ptr@+0x298 / len@+0x2a0). record: +0x00=effect ArcInner ptr(strong@0/weak@8/data@0x10)
//   / +0x08=dyn vtable / +0x10=range. 네이티브 apply가 시전 중 push = 바닐라 궁의 실제 effect(descriptor엔 없음).
//   캐스트 진행하며 len 증가 → 최대 len 스냅샷이 가장 완전한 트리.
static EXEC_SEEN: Mutex<Option<std::collections::HashMap<String, (u64, u64)>>> = Mutex::new(None); // name->(ult_ptr,max_len)
unsafe fn dump_exec_list(ent: usize, name: &str) {
    let ult_ptr  = rd_u64(ent + 0x588).unwrap_or(0);       // 현재 궁 descriptor(캐스트 식별)
    let exec_len = rd_u64(ent + 0x2a0).unwrap_or(0);
    let exec_ptr = rd_u64(ent + 0x298).unwrap_or(0) as usize;
    if exec_len == 0 || exec_len > 64 || exec_ptr < 0x10000 { return; }
    // 같은 궁(ult_ptr)에서 len이 이전 최대 이하면 skip = 성장 스냅샷만 기록(마지막이 가장 완전).
    let do_dump = {
        let mut g = EXEC_SEEN.lock().unwrap_or_else(|e| e.into_inner());
        if g.is_none() { *g = Some(std::collections::HashMap::new()); }
        let m = g.as_mut().unwrap();
        match m.get(name).copied() {
            Some((last, maxlen)) if last == ult_ptr && exec_len <= maxlen => false,
            _ => { m.insert(name.to_string(), (ult_ptr, exec_len)); true }
        }
    };
    if !do_dump { return; }
    let mut s = format!("\n===== {} EXEC ent={:#x} ult={:#x} len={} =====\n", name, ent, ult_ptr, exec_len);
    let _ = &s;
    // (본체는 아래 계속)
    dump_exec_body(ent, name, ult_ptr, exec_ptr, exec_len, s);
}
// 현재 시전 effect(entity+0x478/+0x480) 캡처 = 궁의 실제 effect(단발/반복 무관).
// ★궁 필터: ult descriptor(entity+0x588)는 궁 시전 때만 재빌드(ptr 변화). 그 변화 직후 윈도우
//   (ttl call) 동안만 +0x478 캡처 → 기본공격 effect 배제. 챔프별 descriptor ptr dedup(cap 16).
static CUR_SEEN: Mutex<Option<std::collections::HashMap<String, (u64, u32, Vec<usize>)>>> = Mutex::new(None);
unsafe fn dump_cur_effect(ent: usize, name: &str) {
    let ult_ptr = rd_u64(ent + 0x588).unwrap_or(0);       // 궁 descriptor(시전마다 재빌드)
    let data    = rd_u64(ent + 0x478).unwrap_or(0) as usize; // 현재 effect ArcInner
    let vt      = rd_u64(ent + 0x480).unwrap_or(0);          // dyn vtable
    let capture = {
        let mut g = CUR_SEEN.lock().unwrap_or_else(|e| e.into_inner());
        if g.is_none() { *g = Some(std::collections::HashMap::new()); }
        let m = g.as_mut().unwrap();
        let e = m.entry(name.to_string()).or_insert((0, 0, Vec::new()));
        if e.0 != ult_ptr && ult_ptr != 0 { e.0 = ult_ptr; e.1 = 18; } // 새 궁 → 캡처 윈도우 오픈(18 call)
        if e.1 == 0 { false }
        else {
            e.1 -= 1;
            if data < 0x10000 || !in_exe(vt) || e.2.contains(&data) || e.2.len() >= 16 { false }
            else { e.2.push(data); true }
        }
    };
    if !capture { return; }
    let state = rd_u64(ent + 0x68).unwrap_or(0);
    let mut s = format!("\n===== {} CUR ent={:#x} ult={:#x} state={:#x} =====\n[cur] arc={:#x} vt=RVA:{:#x}\n",
                        name, ent, ult_ptr, state, data, rva_of(vt));
    let mut seen: Vec<usize> = Vec::new();
    let mut budget: u32 = 60;
    walk_node(data + 0x10, vt, 1, &mut s, &mut seen, &mut budget); // effect struct = ArcInner+0x10
    dump_log(&s);
}
unsafe fn dump_exec_body(_ent: usize, _name: &str, _ult: u64, exec_ptr: usize, exec_len: u64, mut s: String) {
    for i in 0..(exec_len as usize).min(32) {
        let rec  = exec_ptr + i * 0x38;
        let arc  = rd_u64(rec + 0x00).unwrap_or(0) as usize; // ArcInner ptr
        let vt   = rd_u64(rec + 0x08).unwrap_or(0);          // dyn vtable
        let rng  = rd_u64(rec + 0x10).unwrap_or(0);
        // record 원본 7워드(0x38)도 남김(해석 안전망)
        let mut rw = String::new();
        for k in 0..7 { rw.push_str(&format!("{:#x} ", rd_u64(rec + k * 8).unwrap_or(0))); }
        s.push_str(&format!("[#{}] arc={:#x} vt=RVA:{:#x} rng={:#x} rec=[{}]\n", i, arc, rva_of(vt), rng, rw.trim_end()));
        // effect struct = ArcInner+0x10
        let data = arc + 0x10;
        if arc >= 0x10000 && in_exe(vt) {
            let mut seen: Vec<usize> = Vec::new();
            let mut budget: u32 = 40;
            walk_node(data, vt, 1, &mut s, &mut seen, &mut budget);
        }
    }
    dump_log(&s);
}
// v2: 힙 버퍼를 ASCII 문자열로 안전 판독(에셋 이름/경로 디코드용). 전부 인쇄가능문자일 때만.
unsafe fn read_ascii(ptr: usize, len: usize) -> Option<String> {
    if ptr < 0x10000 || len < 2 || len > 96 { return None; }
    let mut buf = vec![0u8; len];
    for i in 0..len { buf[i] = rd_u8(ptr + i)?; }
    if buf.iter().all(|&b| (0x20..0x7f).contains(&b)) {
        Some(String::from_utf8_lossy(&buf).into_owned())
    } else { None }
}
// effect 노드 재귀 순회: 노드의 워드 덤프 + 자식 fat-ptr(인접 heap,exe) / Vec<Arc<dyn>>(heap,len) 따라감.
// v2: ①노드 실제 sz 내로만 읽음(인접 힙 오염 제거) ②String{ptr,cap,len}/(ptr,len) 필드 문자열 디코드.
unsafe fn walk_node(data: usize, vt: u64, depth: usize, out: &mut String, seen: &mut Vec<usize>, budget: &mut u32) {
    if depth > 4 || *budget == 0 || data < 0x10000 { return; }
    if seen.contains(&data) { return; }
    seen.push(data); *budget -= 1;
    let ind = "  ".repeat(depth + 1);
    let size = if in_exe(vt) { rd_u64(vt as usize + 8).unwrap_or(0) } else { 0 };
    let nw: usize = if size >= 8 && size <= 0x800 { (((size as usize) + 7) / 8).min(40) } else { 14 };
    let mut w = vec![0u64; nw];
    for i in 0..nw { w[i] = rd_u64(data + i * 8).unwrap_or(0); }
    let mut ws = String::new();
    for x in &w { ws.push_str(&format!("{:#x} ", x)); }
    out.push_str(&format!("{}vt=RVA:{:#x} sz={:#x} d={:#x} [{}]\n", ind, rva_of(vt), size, data, ws.trim_end()));
    // 문자열 필드 디코드: String{ptr,cap,len} 우선, 아니면 (ptr,len)
    let mut si = 0usize;
    while si < nw {
        let p = w[si];
        if is_heap(p) {
            if si + 2 < nw {
                let (cap, len) = (w[si + 1], w[si + 2]);
                if len >= 2 && len <= 96 && cap >= len && cap <= 0x1000 {
                    if let Some(st) = read_ascii(p as usize, len as usize) {
                        out.push_str(&format!("{}  s@+{:#x}=\"{}\"\n", ind, si * 8, st));
                        si += 3; continue;
                    }
                }
            }
            if si + 1 < nw {
                let len = w[si + 1];
                if len >= 2 && len <= 96 {
                    if let Some(st) = read_ascii(p as usize, len as usize) {
                        out.push_str(&format!("{}  s@+{:#x}=\"{}\"\n", ind, si * 8, st));
                        si += 2; continue;
                    }
                }
            }
        }
        si += 1;
    }
    // 자식 탐색
    for i in 0..nw.saturating_sub(1) {
        let (a, b) = (w[i], w[i + 1]);
        // 노이즈 억제: b가 effect vtable이려면 size(b+8)이 [8,0x800] (코드/쓰레기 포인터 추종 방지)
        let vt_ok = |v: u64| -> bool { let sz = rd_u64(v as usize + 8).unwrap_or(0); sz >= 8 && sz <= 0x800 };
        if is_heap(a) && in_exe(b) && vt_ok(b) {
            // 직접 자식 fat-ptr {data=a, vt=b}
            walk_node(a as usize, b, depth + 1, out, seen, budget);
        } else if is_heap(a) && b >= 1 && b <= 32 {
            // Vec<fat-ptr or Arc> {ptr=a, ?, len=b}. element stride 0x10 = {p, vt}.
            for e in 0..(b as usize).min(16) {
                let ep = rd_u64(a as usize + e * 0x10).unwrap_or(0);
                let evt = rd_u64(a as usize + e * 0x10 + 8).unwrap_or(0);
                if ep >= 0x10000 && in_exe(evt) {
                    // {data=ep, vt=evt} 직접이거나, Arc면 payload=ep+0x10
                    let d2 = if in_exe(rd_u64(ep as usize + 8).unwrap_or(0)) { ep } else { ep + 0x10 };
                    let v2 = if in_exe(rd_u64(ep as usize + 8).unwrap_or(0)) { rd_u64(ep as usize + 8).unwrap_or(evt) } else { evt };
                    if in_exe(v2) && vt_ok(v2) {
                        walk_node(d2 as usize, v2, depth + 1, out, seen, budget);
                    }
                }
                if *budget == 0 { break; }
            }
        }
        if *budget == 0 { break; }
    }
}
// 사일러스 시전 계측: cast-apply의 rdx(action ptr 후보)를 4개 슬롯 포인터와 대조 → 어느 액션이 실행되는지 직접 판정
static SYL_CAST_N: AtomicU32 = AtomicU32::new(0);
unsafe fn dump_common(saved: usize) {
    let ent = *((saved + 0x28) as *const u64) as usize; // rcx=caster
    let name = match champ_name(ent) { Some(v) => v, None => return };
    if name.len() < 3 { return; }
    // ★네이티브 강탈 판별: 사일러스 시전 시 그의 궁 descriptor(entity+0x588 data)를 캐시.
    //   datachamp apply서 rcx가 정확히 이거일 때만 리다이렉트 → 평타/스킬 배제.
    if NATIVE_MODE.load(Ordering::Relaxed) && name == "sylas" {
        if let Some(d) = rd_u64(ent + 0x588) {
            if d >= 0x10000 {
                SYLAS_ULT_DESC.with(|c| c.set(d as usize));
                SYLAS_ULT_TS.with(|c| c.set(qpc_now()));
            }
        }
    }
    if !DUMP_MODE.load(Ordering::Relaxed) { return; }
    if name == "sylas" {
        let n = SYL_CAST_N.fetch_add(1, Ordering::Relaxed);
        if n < 300 {
            let rdx = *((saved + 0x20) as *const u64);
            let r8  = *((saved + 0x18) as *const u64);
            let p = [rd_u64(ent+0x558).unwrap_or(0), rd_u64(ent+0x568).unwrap_or(0),
                     rd_u64(ent+0x578).unwrap_or(0), rd_u64(ent+0x588).unwrap_or(0)];
            let which = if rdx==p[0] {"ATK"} else if rdx==p[1] {"SK1"} else if rdx==p[2] {"SK2"}
                        else if rdx==p[3] {"ULT★"} else {"?"};
            write_log(&format!("[캐스트#{}] sylas {} rdx={:#x} r8={:#x} slots=[{:#x} {:#x} {:#x} {:#x}]\n",
                n, which, rdx, r8, p[0], p[1], p[2], p[3]));
        }
    }
    // 방법 B: 실행 effect-list(entity+0x290, 반복궁) + 현재 effect(entity+0x478, 전 궁) 캡처
    dump_exec_list(ent, &name);
    dump_cur_effect(ent, &name);
    // 슬롯 정정(W35): +0x578=skill2, +0x588=ult (실측: sylas +0x588에 gambler_r.ult 디스크립터)
    let ult_ptr = rd_u64(ent + 0x588).unwrap_or(0);
    // 신규 챔프=풀덤프 / 궁슬롯 포인터 변화(=궁 방금 빌드됨)=궁만 재덤프(≤3회) / 그외=skip
    let ult_redump: u32 = {
        let mut g = DUMP_SEEN.lock().unwrap_or_else(|e| e.into_inner());
        if g.is_none() { *g = Some(std::collections::HashMap::new()); }
        let m = g.as_mut().unwrap();
        match m.get(&name).copied() {
            None => { m.insert(name.clone(), (ult_ptr, 0)); 0 } // 0 = 풀덤프
            Some((last, n)) => {
                if ult_ptr != 0 && ult_ptr != last && n < 3 {
                    m.insert(name.clone(), (ult_ptr, n + 1)); n + 1
                } else { return; }
            }
        }
    };
    if ult_redump > 0 {
        // 궁 전용 재덤프: 신선한 궁 트리(방금 빌드) 깊게 — W35: 진짜 궁 슬롯 +0x588
        let mut s = format!("\n===== {} ULT#{} ent={:#x} =====\n[exec.ult] slot+0x588:\n", name, ult_redump, ent);
        let vt = rd_u64(ent + 0x590).unwrap_or(0);
        if ult_ptr >= 0x10000 && in_exe(vt) {
            let mut seen: Vec<usize> = Vec::new();
            let mut budget: u32 = 120;
            walk_node(ult_ptr as usize, vt, 0, &mut s, &mut seen, &mut budget);
        }
        dump_log(&s);
        return;
    }
    let mut s = format!("\n===== {} ent={:#x} =====\n", name, ent);
    // 실행 레이어: casting-block 4쌍(값). W35 슬롯 정정: 578=skill2 / 588=ult
    for (lbl, off) in [("attack",0x558usize),("skill",0x568),("skill2",0x578),("ult",0x588)] {
        let data = rd_u64(ent + off).unwrap_or(0) as usize;
        let vt = rd_u64(ent + off + 8).unwrap_or(0);
        s.push_str(&format!("[exec.{}] slot+{:#x}:\n", lbl, off));
        if data >= 0x10000 && in_exe(vt) {
            let mut seen: Vec<usize> = Vec::new();
            let mut budget: u32 = 50;
            walk_node(data, vt, 0, &mut s, &mut seen, &mut budget);
        }
    }
    // 레시피 레이어: ability-def(+0x4e8, Arc<Vec<ability>> → 깔끔한 disc 트리). 넓게 순회.
    {
        let data = rd_u64(ent + 0x4e8).unwrap_or(0) as usize;
        let vt = rd_u64(ent + 0x4f0).unwrap_or(0);
        s.push_str(&format!("[adef] +0x4e8: data={:#x} vt=RVA:{:#x}\n", data, rva_of(vt)));
        if data >= 0x10000 {
            let mut seen: Vec<usize> = Vec::new();
            let mut budget: u32 = 80;
            walk_node(data, vt, 0, &mut s, &mut seen, &mut budget);
        }
    }
    dump_log(&s);
}

// ── clone 캡처: (id → 영속 master record src) ────────────────────────────────
static RECORDS: Mutex<Option<HashMap<String, usize>>> = Mutex::new(None);
static CL_LOG: AtomicU32 = AtomicU32::new(0);
unsafe fn clone_common(saved: usize) {
    let src = *((saved + 0x20) as *const u64) as usize; // rdx = 영속 master record
    let id = match read_id(src) { Some(v) => v, None => return };
    // src+0x4c8 궁 descriptor 유효(readable, -1 아님)일 때만 캡처
    let desc0 = match rd_u64(src + E_ULT_DESC) { Some(v) => v, None => return };
    if desc0 == u64::MAX { return; } // -1 = 궁 없음
    if !readable(src + E_ULT_DESC, 0x170) { return; }
    let is_new = {
        let mut g = RECORDS.lock().unwrap_or_else(|e| e.into_inner());
        if g.is_none() { *g = Some(HashMap::new()); }
        g.as_mut().unwrap().insert(id.clone(), src) != Some(src)
    };
    if is_new && CL_LOG.fetch_add(1, Ordering::Relaxed) < 40 {
        write_log(&format!("[캡처] {} → master {:#x} (+0x4c8 첫필드={:#x})\n", id, src, desc0));
    }
}

// 타깃 record 조회 + 재검증(스왑 직전 stale/무효 방지)
unsafe fn lookup_valid(target: &str) -> Option<usize> {
    let rec = {
        let g = RECORDS.lock().unwrap_or_else(|e| e.into_inner());
        *g.as_ref()?.get(target)?
    };
    let desc0 = rd_u64(rec + E_ULT_DESC)?;
    if desc0 == u64::MAX { return None; }
    if !readable(rec + E_ULT_DESC, 0x170) { return None; }
    if !readable(rec + B_IDPTR, 0x10) { return None; }
    // id 재확인(주소 재사용 방지)
    if read_id(rec).as_deref() != Some(target) { return None; }
    Some(rec)
}

// ── factory 훅: 사일러스 궁 빌드 → rcx를 타깃 master로 스왑 ──────────────────
static HIJACK_N: AtomicU32 = AtomicU32::new(0);
static HJ_LOG: AtomicU32 = AtomicU32::new(0);
unsafe fn factory_common(saved: usize) {
    let selfp = *((saved + 0x28) as *const u64) as usize; // rcx = 현재 bundle
    let id = match read_id(selfp) { Some(v) => v, None => return };
    if id != "sylas" { return; }
    if !ARMED.load(Ordering::Relaxed) { return; }
    if !in_combat_tick() { return; } // ★전투(활성 sim tick) 중에만 스왑 — 로드/드래프트 행 방지
    if NATIVE_MODE.load(Ordering::Relaxed) {
        // 네이티브 모드: factory 스왑 안 함. 사일러스가 자기 궁 시전 → CASTDUMP서 궁 descriptor 캡처,
        //   datachamp apply서 rcx==그 descriptor일 때만 gambler apply로 리다이렉트.
        return;
    }
    let target = cur_target();
    let rec = match lookup_valid(&target) { Some(v) => v, None => {
        if HJ_LOG.fetch_add(1, Ordering::Relaxed) < 10 {
            write_log(&format!("[강탈보류] 타깃 {} 미캡처/무효\n", target));
        }
        return;
    } };
    *((saved + 0x28) as *mut u64) = rec as u64; // rcx-swap → 원본 factory가 rec의 궁 빌드
    let n = HIJACK_N.fetch_add(1, Ordering::Relaxed) + 1;
    if HJ_LOG.load(Ordering::Relaxed) < 20 || n % 100 == 0 {
        HJ_LOG.fetch_add(1, Ordering::Relaxed);
        write_log(&format!("[강탈#{}] sylas {:#x} → {} master {:#x}\n", n, selfp, target, rec));
    }
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
    s.extend_from_slice(&[0x49,0x89,0xe2]);
    s.extend_from_slice(&[0x51,0x52,0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]);
    s.extend_from_slice(&[0x48,0x89,0xe1]);
    s.extend_from_slice(&[0x4c,0x89,0xd2]);
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff,0xd0]);
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]);
    let mut orig = vec![0u8; hook_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), hook_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes());
    s.extend_from_slice(&[0xff,0xe0]);
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

// 조건부 리다이렉트 detour: cap_fn(saved, orig_rsp)->u64.
//   반환 0 = passthrough(원본 실행), (result|HANDLED_BIT) = 이미 대체실행됨→그 result를 호출자에 리턴.
unsafe fn install_redirect_detour(rva: usize, hook_len: usize, expect: &[u8], cap_fn: usize) -> Result<usize, String> {
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
    let mut orig = vec![0u8; hook_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), hook_len);
    // passthrough 블록 길이 = pop(10) + orig(hook_len) + mov rax,imm(10) + jmp rax(2)
    let pass_len = 10 + hook_len + 10 + 2;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);                                   // mov r10, rsp
    s.extend_from_slice(&[0x51,0x52,0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]);// push rcx,rdx,r8,r9,r10,r11
    s.extend_from_slice(&[0x48,0x89,0xe1]);                                   // mov rcx, rsp (saved)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);                                   // mov rdx, r10 (orig_rsp)
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);                              // sub rsp, 0x28
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes()); // mov rax, cap_fn
    s.extend_from_slice(&[0xff,0xd0]);                                        // call rax
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);                              // add rsp, 0x28
    // handled 체크: rdx=rax>>32; if(edx!=0) redirect
    s.extend_from_slice(&[0x48,0x89,0xc2]);                                   // mov rdx, rax
    s.extend_from_slice(&[0x48,0xc1,0xea,0x20]);                              // shr rdx, 0x20
    s.extend_from_slice(&[0x85,0xd2]);                                        // test edx, edx
    s.extend_from_slice(&[0x75, pass_len as u8]);                            // jnz +pass_len (→redirect)
    // passthrough:
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]);// pop r11..rcx
    s.extend_from_slice(&orig);                                               // 원본 프롤로그
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes()); // mov rax, ret_addr
    s.extend_from_slice(&[0xff,0xe0]);                                        // jmp rax
    // redirect: (rax = result|HANDLED_BIT) → eax=result, rsp=orig_rsp, ret
    s.extend_from_slice(&[0x89,0xc0]);                                        // mov eax, eax (low32=result)
    s.extend_from_slice(&[0x48,0x8b,0x64,0x24,0x08]);                         // mov rsp, [rsp+8] (=orig_rsp)
    s.extend_from_slice(&[0xc3]);                                             // ret
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

unsafe extern "C" fn cap_clone(saved: usize, _e: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| clone_common(saved)));
}
unsafe extern "C" fn cap_factory(saved: usize, _e: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| factory_common(saved)));
}

// ── 5번째 길: datachamp apply 진입점 리다이렉트 ──
//   반환 0 = passthrough(원본 datachamp apply 실행), (result|HANDLED_BIT) = gambler apply 실행완료.
unsafe fn datapply_redirect(saved: usize, orig_rsp: usize) -> u64 {
    if !NATIVE_MODE.load(Ordering::Relaxed) { return 0; }
    let rcx = *((saved + 0x28) as *const u64) as usize;   // descriptor being applied
    // ★정밀 판별: rcx가 사일러스 궁 descriptor(캐시)와 정확히 일치 + 신선할 때만.
    let ult_desc = SYLAS_ULT_DESC.with(|c| c.get());
    let fresh = SYLAS_ULT_TS.with(|c| {
        let ts = c.get();
        if ts == 0 { return false; }
        let f = QPF.load(Ordering::Relaxed);
        if f == 0 { return false; }
        let age_ms = (qpc_now() - ts).saturating_mul(1000) / f as i64;
        age_ms >= 0 && age_ms < 2000
    });
    let is_ult = rcx != 0 && rcx == ult_desc && fresh;
    // 진단: 초반 몇 건은 매칭 여부와 rcx/ult_desc를 로깅(일치하는지 확인)
    if NAT_LOG.load(Ordering::Relaxed) < 30 {
        NAT_LOG.fetch_add(1, Ordering::Relaxed);
        write_log(&format!("[apply진단] rcx={:#x} ult_desc={:#x} fresh={} 일치={}\n", rcx, ult_desc, fresh, is_ult));
    }
    if !is_ult { return 0; } // 궁 아님 → passthrough(원본 실행)
    // 살아있는 인자로 gambler 네이티브 apply 직접 호출
    let rdx = *((saved + 0x20) as *const u64) as usize;   // caster_handle
    let r8  = *((saved + 0x18) as *const u64) as usize;   // sim_state
    let r9  = *((saved + 0x10) as *const u64) as usize;   // target_ctx
    let casting_ctx = *((orig_rsp + 0x28) as *const u64) as usize; // 5th 인자
    let base = exe_base();
    let desc = core::ptr::addr_of_mut!(GAMBLER_DESC) as usize;
    let f: ApplyFn = core::mem::transmute(base + GAMBLER_APPLY_RVA);
    let r = f(desc, rdx, r8, r9, casting_ctx);            // ★네이티브 gambler 궁 직접 실행
    write_log(&format!("[네이티브강탈★] gambler apply 실행 rcx={:#x} rdx={:#x} ret={:#x}\n", rcx, rdx, r));
    (r & 0xffff_ffff) | HANDLED_BIT
}
unsafe extern "C" fn cap_datapply(saved: usize, orig_rsp: usize) -> u64 {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| datapply_redirect(saved, orig_rsp))) {
        Ok(v) => v,
        Err(_) => 0, // 패닉 시 passthrough(원본 실행)
    }
}
unsafe extern "C" fn cap_tick(saved: usize, _e: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tick_common(saved)));
}
unsafe extern "C" fn cap_dump(saved: usize, _e: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| dump_common(saved)));
}

static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
fn init(_ctx: &GameCtx) -> ModRegistration {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(6));
        if HOOK_INSTALLED.swap(true, Ordering::Relaxed) { return; }
        veh_install();
        arm_refresher();
        { let mut f = 0i64; unsafe { QueryPerformanceFrequency(&mut f); } QPF.store(f.max(1) as u64, Ordering::Relaxed); }
        let base = exe_base();
        write_log(&format!("\n===== sylas_hijack W37 네이티브강탈(궁 descriptor 정밀판별)+cfg타깃({}) 시작 base={:#x} =====\n", DEFAULT_TARGET, base));
        match unsafe { install_detour(CLONE_RVA, CLONE_LEN, &CLONE_SIG, cap_clone as *const () as usize) } {
            Ok(stub) => write_log(&format!("[install] clone(캡처) OK @{:#x} stub={:#x}\n", base + CLONE_RVA, stub)),
            Err(e)   => write_log(&format!("[install] clone 실패: {}\n", e)),
        }
        match unsafe { install_detour(TICK_RVA, TICK_LEN, &TICK_SIG, cap_tick as *const () as usize) } {
            Ok(stub) => write_log(&format!("[install] tick(전투게이트) OK @{:#x} stub={:#x}\n", base + TICK_RVA, stub)),
            Err(e)   => write_log(&format!("[install] tick 실패: {}\n", e)),
        }
        match unsafe { install_detour(FACTORY_RVA, FACTORY_LEN, &FACTORY_SIG, cap_factory as *const () as usize) } {
            Ok(stub) => write_log(&format!("[install] factory(rcx-swap) OK @{:#x} stub={:#x}\n", base + FACTORY_RVA, stub)),
            Err(e)   => write_log(&format!("[install] factory 실패: {}\n", e)),
        }
        match unsafe { install_detour(CASTDUMP_RVA, CASTDUMP_LEN, &CASTDUMP_SIG, cap_dump as *const () as usize) } {
            Ok(stub) => write_log(&format!("[install] castdump OK @{:#x} stub={:#x}\n", base + CASTDUMP_RVA, stub)),
            Err(e)   => write_log(&format!("[install] castdump 실패: {}\n", e)),
        }
        // 5번째 길: datachamp apply 리다이렉트 훅(native=1일 때만 발화, 게이트 내부)
        match unsafe { install_redirect_detour(DATAPPLY_RVA, DATAPPLY_LEN, &DATAPPLY_SIG, cap_datapply as *const () as usize) } {
            Ok(stub) => write_log(&format!("[install] datapply(네이티브리다이렉트) OK @{:#x} stub={:#x}\n", base + DATAPPLY_RVA, stub)),
            Err(e)   => write_log(&format!("[install] datapply 실패: {}\n", e)),
        }
    });
    ModRegistration::new("sylas_hijack")
}

declare_mod!(init);
