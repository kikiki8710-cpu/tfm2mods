// plan_probe.rs v2 — plan_v2 런타임 캡처 (§K). 후킹 2개: retreat_engage(엔티티) + dispatch(subplan)
// 빌드: build_mod.bat "C:\tfm2mods\plan_probe\src\plan_probe.rs"  → plan_probe.dll
// 로그: mods\plan_probe\plan_probe.txt , 정적테이블: tables.txt , 선수: athlete.txt
//
// 키: F8=엔티티/컨테이너 캡처(12건) | F7=정적테이블 1회 | F9=선수목록 | F10=subplan 전환추적 토글 | F11=(위험)vtable호출
// v2 변경: ①+0x3e4/+0x3e8/+0x4a8/+0x458 = i32로 읽음 ②챔피언 필터 speed>0 ③zone_cells +8 보정
//          ④dispatch(0x1c08770) 후킹: rdx=plan_state+0x500 → subplan disc 직접캡처 ⑤전환표(§K15)

use mod_api::*;
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "plan_probe";
const RVA_RETREAT:  usize = 0x1fa9cc0;   // FUN_141fa9cc0 retreat_engage (#5 disc4) prologue 12B
const RVA_DISPATCH: usize = 0x1c08770;   // plan_judge_dispatch prologue 17B; rdx=plan_state+0x500

const RVA_ZONE_GRID:  usize = 0x3573f10;
const RVA_ZONE_CELLS: usize = 0x3563138; // v2: +8 보정 (entry0 헤더 스킵)
const RVA_ROUTE_SERP: usize = 0x3565d18;
const RVA_ROUTE_EPIC: usize = 0x3567a60;
const EPIC_XY: i64 = 288000;
const SERP_XY: i64 = 672000;

type BOOL = i32; type DWORD = u32; type HMODULE = usize;
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleHandleExW(flags: u32, addr: *const u16, h: *mut HMODULE) -> BOOL;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, sz: DWORD) -> DWORD;
    fn VirtualAlloc(addr: usize, sz: usize, typ: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old_protect: *mut u32) -> BOOL;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, sz: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
}
#[link(name = "user32")]
extern "system" { fn GetAsyncKeyState(vkey: i32) -> i16; }
const VK_F7: i32 = 0x76; const VK_F8: i32 = 0x77; const VK_F9: i32 = 0x78;
const VK_F10: i32 = 0x79; const VK_F11: i32 = 0x7A;

#[repr(C)] #[derive(Default)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32,
    region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32 }
unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const RD: u32 = 0x02|0x04|0x20|0x40; const GUARD: u32 = 0x01|0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}
unsafe fn rd_u64(a: usize) -> Option<u64> { if readable(a,8){Some(std::ptr::read_unaligned(a as *const u64))}else{None} }
unsafe fn rd_i64(a: usize) -> Option<i64> { if readable(a,8){Some(std::ptr::read_unaligned(a as *const i64))}else{None} }
unsafe fn rd_i32(a: usize) -> Option<i32> { if readable(a,4){Some(std::ptr::read_unaligned(a as *const i32))}else{None} }

fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }
fn log_path() -> Option<PathBuf> {
    unsafe {
        let addr = log_path as *const () as usize;
        let mut h: HMODULE = 0;
        if GetModuleHandleExW(0x4|0x2, addr as *const u16, &mut h) == 0 || h == 0 { return None; }
        let mut buf = [0u16; 4096];
        let len = GetModuleFileNameW(h, buf.as_mut_ptr(), buf.len() as DWORD);
        if len == 0 { return None; }
        let s = String::from_utf16_lossy(&buf[..len as usize]);
        let mut p = PathBuf::from(s); p.pop(); p.push("plan_probe.txt"); Some(p)
    }
}
fn append_log(line: &str) {
    if let Some(p) = log_path() {
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = write!(f,"{}",line); }
    }
}
fn write_log_named(name: &str, content: &str) {
    if let Some(mut p) = log_path() { p.pop(); p.push(name); let _ = fs::write(p, content); }
}
fn subplan_name(d: u64) -> &'static str {
    match d {
        2=>"ForcePassive",3=>"PassiveLine",4=>"PassiveJungle",5=>"ActiveRecall",6=>"BattleSupport",
        7=>"LineGanker",8=>"LineGankCover",9=>"EpicHuntAndPoke",10=>"EpicHuntAndBattle",
        11=>"SerpenHuntAndPoke",12=>"SerpenHuntAndBattle",13=>"AttackNexus",14=>"DefenseNexus",
        0=>"None/Init?",1=>"None/Init?",_=>"?",
    }
}

static CAPTURE_ARMED: AtomicBool = AtomicBool::new(false);
static TRANS_ARMED: AtomicBool = AtomicBool::new(false);
static CALL_VTABLE: AtomicBool = AtomicBool::new(false);
static LAST_KEY: AtomicU64 = AtomicU64::new(0);
static DUMP_COUNT: AtomicU64 = AtomicU64::new(0);
static DISP_LAST: AtomicU64 = AtomicU64::new(0);
static DISP_COUNT: AtomicU64 = AtomicU64::new(0);
static PREV_F7: AtomicBool = AtomicBool::new(false);
static PREV_F8: AtomicBool = AtomicBool::new(false);
static PREV_F9: AtomicBool = AtomicBool::new(false);
static PREV_F10: AtomicBool = AtomicBool::new(false);
static PREV_F11: AtomicBool = AtomicBool::new(false);
static BOOTED: AtomicBool = AtomicBool::new(false);
// 전환 추적: plan_state 해시 256슬롯에 마지막 subplan 저장 (0xff=미설정)
static TRANS_LAST: [AtomicU8; 256] = [const { AtomicU8::new(0xff) }; 256];

// ── 엔티티 결정필드 덤프 (v2: i32 정정) ──
unsafe fn dump_entity_full(tag: &str, e: usize, exe_base: usize, s: &mut String) {
    if e <= 0x10000 || !readable(e, 0x740) { return; }
    let gi64 = |off: usize| rd_i64(e+off).unwrap_or(i64::MIN);
    let gi32 = |off: usize| rd_i32(e+off).unwrap_or(i32::MIN);
    let x = gi64(0x648); let y = gi64(0x650);
    let hpn = gi64(0x658); let hpd = gi64(0x610);
    let hp_pct = if hpd>0 { hpn*100/hpd } else { -1 };
    let team = gi32(0x6a8);
    let ridx = gi32(0x738);
    s.push_str(&format!("  [{}] e=0x{:x} team={} ridx={} pos=({},{}) hp={}/{}({}%) alive(+0x4a8,i32)={}\n",
        tag, e, team, ridx, x, y, hpn, hpd, hp_pct, gi32(0x4a8)));
    s.push_str(&format!("     speed(+0x628)={} 사거리(+0x488)={} reach[420={} 490={} 5b0={} 458(i32)={} 45c(i32)={} 668={}]\n",
        gi64(0x628), gi64(0x488), gi64(0x420), gi64(0x490), gi64(0x5b0), gi32(0x458), gi32(0x45c), gi64(0x668)));
    s.push_str(&format!("     rate(+0x3e4,i32)={} flag(+0x3e8,i32)={} f6(+0x3e6,u8 i32)={} dtype(+0x4a4,i32)={} tgtsel(+0x6a0)={}\n",
        gi32(0x3e4), gi32(0x3e8), gi32(0x3e6) & 0xff, gi32(0x4a4), gi64(0x6a0)));
    let d2 = |ox: i64, oy: i64| { let dx=x-ox; let dy=y-oy; dx*dx+dy*dy };
    s.push_str(&format!("     dist²→epic={} serpent={} (커밋²=22500000000)\n", d2(EPIC_XY,EPIC_XY), d2(SERP_XY,SERP_XY)));
    let _ = exe_base;
    // ★인라인 스탯 스캔: entity +0x80~+0x260 을 i32로 (방어/마저/공력/레벨 = 값으로 식별).
    //   stat 후보(0<v<2000)만 오프셋과 함께 출력.
    let mut sc = String::from("     [stat스캔 i32]");
    let mut k = 0;
    while k < 0x1e0 {
        let off = 0x80 + k;
        let v = rd_i32(e+off).unwrap_or(i32::MIN);
        if v > 0 && v < 2000 { sc.push_str(&format!(" +{:x}={}", off, v)); }
        k += 4;
    }
    s.push_str(&sc); s.push('\n');
}

unsafe fn dump_static_tables() {
    let base = GetModuleHandleW(core::ptr::null());
    let mut o = format!("[{}ms] === 정적 테이블 (base=0x{:x}) ===\n", now_ms(), base);
    o.push_str("\n[zone_grid_29x29 @0x3573f10] (i32, row*0x1e+col)\n");
    let zg = base + RVA_ZONE_GRID;
    for row in 0..30usize {
        let mut line = format!("r{:02}:", row);
        for col in 0..30usize { line.push_str(&format!(" {:>3}", rd_i32(zg+(row*0x1e+col)*4).unwrap_or(-999))); }
        o.push_str(&line); o.push('\n');
    }
    o.push_str("\n[zone_cells_70 @0x3563138 (+8보정)] (col,row) i64쌍\n");
    let zc = base + RVA_ZONE_CELLS;
    for i in 0..70usize {
        let c = rd_i64(zc+i*0x10).unwrap_or(i64::MIN);
        let r = rd_i64(zc+i*0x10+8).unwrap_or(i64::MIN);
        if c==i64::MIN { break; }
        o.push_str(&format!("  [{:02}] col={} row={} (world {},{})\n", i, c, r, c*32000+16000, r*32000+16000));
    }
    for (nm, rva) in [("route_epic@0x3567a60", RVA_ROUTE_EPIC), ("route_serpent@0x3565d18", RVA_ROUTE_SERP)] {
        o.push_str(&format!("\n[{}] (i64, col*8+row*0xf0)\n", nm));
        let g = base + rva;
        for row in 0..30usize {
            let mut line = format!("r{:02}:", row);
            for col in 0..30usize { line.push_str(&format!(" {:>2}", rd_i64(g+col*8+row*0xf0).unwrap_or(-9))); }
            o.push_str(&line); o.push('\n');
        }
    }
    write_log_named("tables.txt", &o);
    append_log(&format!("[{}ms] ★F7 — tables.txt\n", now_ms()));
}

// ── HOOK A: retreat_engage. p5=[rsp+0x28] p6=[rsp+0x30] ──
unsafe extern "C" fn retreat_capture(_saved: usize, entry_rsp: usize) {
    if entry_rsp < 0x10000 || !CAPTURE_ARMED.load(Ordering::Relaxed) { return; }
    let p5 = rd_u64(entry_rsp+0x28).unwrap_or(0) as usize;
    let p6 = rd_u64(entry_rsp+0x30).unwrap_or(0) as usize;
    let plan_base = if p6>0x10000 && readable(p6+0x18,8) { rd_u64(p6+0x18).unwrap_or(0) as usize } else { 0 };
    if plan_base <= 0x10000000 || !readable(plan_base+0x1e0, 0x28) { return; }
    let key = plan_base as u64;
    if LAST_KEY.load(Ordering::Relaxed) == key { return; }
    LAST_KEY.store(key, Ordering::Relaxed);
    let dc = DUMP_COUNT.fetch_add(1, Ordering::Relaxed);
    if dc >= 12 { CAPTURE_ARMED.store(false, Ordering::Relaxed); return; }
    let exe_base = GetModuleHandleW(core::ptr::null());
    let ret = rd_u64(entry_rsp).unwrap_or(0) as usize;
    let mut s = format!("\n[cap #{}] retreat_engage | 호출자RVA=0x{:x} p5=0x{:x} plan_base=0x{:x}\n",
        dc, ret.wrapping_sub(exe_base), p5, plan_base);
    // ★trait vtable 자동탐지: 후보들의 +8을 vtable로 검증(exe범위 & vtable[0]→.text), 첫 유효본 덤프.
    let img = 0x4000000usize; // ~exe image size
    let is_vt = |vt: usize| -> bool {
        vt > 0x10000 && readable(vt, 0x178) && {
            let m0 = rd_u64(vt).unwrap_or(0) as usize;
            m0 > exe_base && m0 < exe_base + img
        }
    };
    let d = |x: usize| rd_u64(x).unwrap_or(0) as usize; // deref
    let cands: [(&str, usize); 10] = [
        ("p5", p5), ("p6", p6), ("*p5", d(p5)), ("*p6", d(p6)),
        ("rsp+38", d(entry_rsp+0x38)), ("rsp+40", d(entry_rsp+0x40)),
        ("rsp+48", d(entry_rsp+0x48)), ("rsp+50", d(entry_rsp+0x50)),
        ("rsp+58", d(entry_rsp+0x58)), ("rsp+60", d(entry_rsp+0x60)),
    ];
    let mut found = false;
    for (nm, c) in cands {
        if c <= 0x10000 || !readable(c+8, 8) { continue; }
        let vt = rd_u64(c+8).unwrap_or(0) as usize;
        if is_vt(vt) {
            let rva = |o: usize| rd_u64(vt+o).map(|p|(p as usize).wrapping_sub(exe_base)).unwrap_or(0);
            s.push_str(&format!("   ★vtable found via {}: obj=0x{:x} vtable=0x{:x}(RVA {:x})\n", nm, c, vt, vt.wrapping_sub(exe_base)));
            s.push_str(&format!("   getter RVA: 0={:x} 8={:x} 18={:x} 20={:x} 28={:x} 30={:x} 38={:x} 48={:x} 50={:x} 90={:x} a8={:x} 128={:x} 140={:x} 168={:x}\n",
                rva(0),rva(8),rva(0x18),rva(0x20),rva(0x28),rva(0x30),rva(0x38),rva(0x48),rva(0x50),rva(0x90),rva(0xa8),rva(0x128),rva(0x140),rva(0x168)));
            found = true; break;
        }
    }
    if !found { s.push_str("   (vtable 후보 못찾음 — 더 넓은 슬롯/deref 필요)\n"); }
    // subplan 후보: plan_base / p5 의 +0x500
    for (nm, base) in [("plan_base", plan_base), ("p5", p5)] {
        if let Some(sp) = rd_u64(base+0x500) { if sp<=14 { s.push_str(&format!("   ?subplan@{}+0x500={}({})\n", nm, sp, subplan_name(sp))); } }
    }
    if let Some(c1) = rd_u64(plan_base+8) { let c1=c1 as usize;
        if c1>0x10000 && readable(c1,0x1300) {
            s.push_str(&format!("   container[1]=0x{:x} +0x12c0={} +0x12f8={}\n", c1, rd_i64(c1+0x12c0).unwrap_or(-1), rd_i64(c1+0x12f8).unwrap_or(-1)));
        }
    }
    for team in 0..2usize {
        let roster = plan_base + team*0x228 + 0x1e0;
        if !readable(roster, 0x28) { continue; }
        for i in 0..5usize {
            let mem = rd_u64(roster+i*8).unwrap_or(0) as usize;
            if mem<=0x10000 || !readable(mem,0x740) { continue; }
            if rd_i64(mem+0x628).unwrap_or(0) <= 0 { continue; }   // v2: speed>0 = 챔피언
            dump_entity_full(&format!("t{}#{}", team, i), mem, exe_base, &mut s);
        }
    }
    append_log(&s);
}

// ── HOOK B: dispatch. saved_ptr[+0x20]=rdx=plan_state+0x500 → *rdx=subplan ──
unsafe extern "C" fn dispatch_capture(saved: usize, _entry_rsp: usize) {
    // 핫패스 — 둘 다 비무장이면 즉시 리턴 (atomic load 2회만)
    if !CAPTURE_ARMED.load(Ordering::Relaxed) && !TRANS_ARMED.load(Ordering::Relaxed) { return; }
    let rdx = rd_u64(saved+0x20).unwrap_or(0) as usize;
    if rdx < 0x10500 || !readable(rdx,8) { return; }
    let subplan = rd_u64(rdx).unwrap_or(999);
    if subplan > 14 { return; }
    let plan_state = rdx - 0x500;
    // §K15 전환 추적 (항상 갱신, 변하면 로그-armed시)
    let slot = ((plan_state >> 4) & 0xff) as usize;
    let cur = (subplan & 0xff) as u8;
    let prev = TRANS_LAST[slot].swap(cur, Ordering::Relaxed);
    if TRANS_ARMED.load(Ordering::Relaxed) && prev != cur && prev != 0xff {
        append_log(&format!("[{}ms] ★전환 plan_state=0x{:x}: {}({}) → {}({})\n",
            now_ms(), plan_state, prev, subplan_name(prev as u64), subplan, subplan_name(subplan as u64)));
    }
    // §K9 F8: distinct plan_state 별 현재 subplan 로그
    if CAPTURE_ARMED.load(Ordering::Relaxed) {
        let dc = DISP_COUNT.load(Ordering::Relaxed);
        if dc < 24 {
            let last = DISP_LAST.load(Ordering::Relaxed);
            if last != plan_state as u64 {
                DISP_LAST.store(plan_state as u64, Ordering::Relaxed);
                DISP_COUNT.fetch_add(1, Ordering::Relaxed);
                append_log(&format!("[disp #{}] plan_state=0x{:x} subplan={}({})\n", dc, plan_state, subplan, subplan_name(subplan)));
            }
        }
    }
}

// 범용 디투어: orig_len 바이트(완전한 명령경계, ≥12)를 스텁에 복제 후 jmp fn+orig_len
unsafe fn install_detour(rva: usize, orig_len: usize, cap_fn: usize) -> Result<(), &'static str> {
    let mbase = GetModuleHandleW(core::ptr::null());
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, orig_len+4) { return Err("fn unreadable"); }
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);        // mov r10, rsp (entry_rsp)
    s.extend_from_slice(&[0x51,0x52,0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]); // push rcx rdx r8 r9 r10 r11
    s.extend_from_slice(&[0x48,0x89,0xe1]);        // mov rcx, rsp (saved)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);        // mov rdx, r10 (entry_rsp)
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);   // sub rsp,0x28
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff,0xd0]);             // call rax
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);   // add rsp,0x28
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]); // pop r11 r10 r9 r8 rdx rcx
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len); // 원본 명령 복제(위치독립 전제)
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes());
    s.extend_from_slice(&[0xff,0xe0]);             // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());

    let mut patch = vec![0x90u8; orig_len];        // nop 패딩
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(())
}

fn key_down(vk: i32) -> bool { (unsafe { GetAsyncKeyState(vk) } as u16 & 0x8000) != 0 }

struct ProbeExt;
impl ModExtension for ProbeExt {
    fn post_update(&self, scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        if !BOOTED.swap(true, Ordering::Relaxed) {
            append_log(&format!("[{}ms] [ext] 폴링 — F8=캡처 F7=테이블 F9=선수 F10=전환추적 F11=vtable\n", now_ms()));
        }
        let f8 = key_down(VK_F8); if f8 && !PREV_F8.swap(f8, Ordering::Relaxed) {
            DUMP_COUNT.store(0, Ordering::Relaxed); LAST_KEY.store(0, Ordering::Relaxed);
            DISP_COUNT.store(0, Ordering::Relaxed); DISP_LAST.store(0, Ordering::Relaxed);
            CAPTURE_ARMED.store(true, Ordering::Relaxed);
            append_log(&format!("\n[{}ms] ===== F8 — 캡처 무장 =====\n", now_ms()));
        } else if !f8 { PREV_F8.store(false, Ordering::Relaxed); }
        let f7 = key_down(VK_F7); if f7 && !PREV_F7.swap(f7, Ordering::Relaxed) { unsafe { dump_static_tables(); } } else if !f7 { PREV_F7.store(false, Ordering::Relaxed); }
        let f10 = key_down(VK_F10); if f10 && !PREV_F10.swap(f10, Ordering::Relaxed) {
            let n = !TRANS_ARMED.load(Ordering::Relaxed); TRANS_ARMED.store(n, Ordering::Relaxed);
            append_log(&format!("[{}ms] F10 — subplan 전환추적 {}\n", now_ms(), if n {"ON"} else {"OFF"}));
        } else if !f10 { PREV_F10.store(false, Ordering::Relaxed); }
        let f11 = key_down(VK_F11); if f11 && !PREV_F11.swap(f11, Ordering::Relaxed) {
            let n = !CALL_VTABLE.load(Ordering::Relaxed); CALL_VTABLE.store(n, Ordering::Relaxed);
            append_log(&format!("[{}ms] F11 — vtable호출 {} (위험)\n", now_ms(), if n {"ON"} else {"OFF"}));
        } else if !f11 { PREV_F11.store(false, Ordering::Relaxed); }
        let f9 = key_down(VK_F9); if f9 && !PREV_F9.swap(f9, Ordering::Relaxed) {
            if let Scene::InGame { data } = scene {
                let db = data.db(); let r: &ClientDatabase = &*db;
                let mut out = format!("[{}ms] === 선수 (id\t이름\tteam) ===\n", now_ms());
                let mut rows: Vec<(usize,String,String)> = Vec::new();
                for (&aid, ath) in r.athletes.iter() {
                    let cs = format!("{:?}", ath.contract);
                    let team = { let k="team_id: "; if let Some(st)=cs.find(k){ let rest=&cs[st+k.len()..];
                        let end=rest.find(|c:char| !c.is_ascii_digit()).unwrap_or(rest.len()); rest[..end].to_string() } else {"-".into()} };
                    rows.push((aid, ath.name.clone(), team));
                }
                rows.sort_by_key(|(id,_,_)| *id);
                for (aid,name,team) in &rows { out.push_str(&format!("{}\t{}\tteam={}\n", aid, name, team)); }
                out.push_str(&format!("총 {}명\n", rows.len()));
                write_log_named("athlete.txt", &out);
                append_log(&format!("[{}ms] ★F9 — 선수 {}명\n", now_ms(), rows.len()));
            }
        } else if !f9 { PREV_F9.store(false, Ordering::Relaxed); }
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    append_log(&format!("[{}ms] === plan_probe v5 INIT (vtable 자동탐지) ===\n", now_ms()));
    unsafe {
        match install_detour(RVA_RETREAT, 12, retreat_capture as *const () as usize) {
            Ok(())=>append_log("[hook] retreat_engage(12B) OK\n"), Err(e)=>append_log(&format!("[hook] retreat 실패: {}\n", e)) }
        match install_detour(RVA_DISPATCH, 17, dispatch_capture as *const () as usize) {
            Ok(())=>append_log("[hook] dispatch(17B) OK\n"), Err(e)=>append_log(&format!("[hook] dispatch 실패: {}\n", e)) }
    }
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(ProbeExt);
    reg
}
declare_mod!(init);
