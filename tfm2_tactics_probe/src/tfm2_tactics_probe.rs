// tfm2_tactics_probe.rs — 신규전술(상황별 공격성) 학습훅 프로브 (독립 진단 모드)
// 목적: RunningMatchInfo::apply_to_match(&self, db) @ RVA 0x1d9ba30 에 detour 걸어 경기완료마다
//   (a)이 훅이 백그라운드(비관전) 경기 포함 계속 발화하나 (b)team_id@self+0x140/+0x148 이 작은정수(team_id)인가
//   큰값(Team*포인터)인가 (c)승패(self+8 세트결과Vec 집계)가 맞나 를 실측. AI/결과 무영향(순수 캡처).
// 근거: apply_to_match = worker.rs "apply_completed_match_results" 완료경기당1회·단일서버스레드(非rayon).
//   프롤로그 = 8-push(55 41 57 41 56 41 55 41 54 56 57 53) = 정확히 12B 위치독립·rip-rel無 → install_8push 안전.
// 빌드: powershell -File C:\tfm2mods\build_inj.ps1 -Src <이 파일> -ModId tfm2_tactics_probe
// 로그: mods\tfm2_tactics_probe\tactics_probe.txt   (F3=카운터 리셋 → 더 캡처)
// 트램폴린/세이프리드 = sim_probe 검증코드 그대로.

use mod_api::*;
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "tfm2_tactics_probe";
const APPLY_RVA: usize = 0x1d9ba30;   // RunningMatchInfo::apply_to_match (0.4.14 핫픽스, 8-push 프롤로그)
const PROLOGUE: [u8; 12] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];
// ── 경기시작 캡처 2종 (마일스톤1: config 0x740서 team_id 오프셋 + 챔프로스터 실측) ──
// 0x1e38ce0 = 매치 등록(큐+HashMap): rcx=게임상태, rdx=매치config(0x740). 프롤로그=6push+mov eax,0x7568=13B 위치독립
//   (chkstk call은 off+13=복사영역 밖 → reloc 불필요·안전. 0x1e2f1c0는 e8이 안쪽이라 크래시 전력=사용금지.)
const REG_RVA: usize = 0x1e38ce0;
const REG_PROLOGUE: [u8; 13] = [0x55,0x41,0x57,0x41,0x56,0x56,0x57,0x53,0xb8,0x68,0x75,0x00,0x00];
// 0x204f810 = sim tick 처리기(8-push): param2(rdx)=GameData, param4(r9)=매치config → config가 sim까지 전달되는지 확인
const TICK_RVA: usize = 0x204f810;

// saved_ptr(스텁이 push한 레지스터: rcx rdx r8 r9 r10 r11) 기준 슬롯 오프셋
const OFF_R9:  usize = 0x10;
const OFF_R8:  usize = 0x18;   // r8  = Database*
const OFF_RDX: usize = 0x20;   // rdx = self(RunningMatchInfo*)
const OFF_RCX: usize = 0x28;   // rcx = out(sret 24B)

const CAP_MAX: u64 = 300;      // 스팸방지: 처음 300경기만 로깅 (F3로 리셋)

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
const VK_F3: i32 = 0x72;

#[repr(C)] #[derive(Default)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32,
    region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32 }

unsafe fn region(addr: usize) -> Option<MemBasicInfo> {
    if addr < 0x10000 { return None; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return None; }
    Some(mbi)
}
unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mbi = match region(addr) { Some(m) => m, None => return false };
    const COMMIT: u32 = 0x1000;
    const RD: u32 = 0x02|0x04|0x20|0x40;
    const GUARD: u32 = 0x01|0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}
unsafe fn rd_u64(a: usize) -> Option<u64> { if readable(a,8){Some(std::ptr::read_unaligned(a as *const u64))}else{None} }
unsafe fn rd_u8(a: usize) -> Option<u8> { if readable(a,1){Some(std::ptr::read_unaligned(a as *const u8))}else{None} }

fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }
fn mod_dir() -> Option<PathBuf> {
    unsafe {
        let addr = mod_dir as *const () as usize;
        let mut h: HMODULE = 0;
        if GetModuleHandleExW(0x4|0x2, addr as *const u16, &mut h) == 0 || h == 0 { return None; }
        let mut buf = [0u16; 4096];
        let len = GetModuleFileNameW(h, buf.as_mut_ptr(), buf.len() as DWORD);
        if len == 0 { return None; }
        let s = String::from_utf16_lossy(&buf[..len as usize]);
        let mut p = PathBuf::from(s); p.pop(); Some(p)
    }
}
fn append_log(line: &str) {
    if let Some(mut p) = mod_dir() {
        p.push("tactics_probe.txt");
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = write!(f,"{}",line); }
    }
}

static BASE: AtomicUsize = AtomicUsize::new(0);
static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
static LOGGED: AtomicU64 = AtomicU64::new(0);
static HIT_TOTAL: AtomicU64 = AtomicU64::new(0);
static BOOTED: AtomicBool = AtomicBool::new(false);
static PREV_F3: AtomicBool = AtomicBool::new(false);

// 유효 포인터처럼 보이는가
fn looks_ptr(v: u64) -> bool { v >= 0x10000 && v < 0x7fff_ffff_ffff }

// 포인터가 가리키는 메모리 첫 len 바이트 hex+ascii (16B/줄)
unsafe fn dump_mem(tag: &str, ptr: usize, len: usize, s: &mut String) {
    if ptr <= 0x10000 || !readable(ptr, 16) { s.push_str(&format!("     {} = 0x{:x} (역참조 불가)\n", tag, ptr)); return; }
    s.push_str(&format!("     {} = 0x{:x}:\n", tag, ptr));
    let n = len.min(0x200);
    let mut i = 0;
    while i < n {
        let mut hex = String::new(); let mut asc = String::new();
        for j in 0..16 {
            if i+j >= n { break; }
            match rd_u8(ptr+i+j) {
                Some(b) => { hex.push_str(&format!("{:02x} ", b));
                    asc.push(if (0x20..0x7f).contains(&b) { b as char } else { '.' }); }
                None => { hex.push_str("?? "); asc.push('.'); }
            }
        }
        s.push_str(&format!("       +{:03x}  {:<48} |{}|\n", i, hex.trim_end(), asc));
        i += 16;
    }
}

// 후보 포인터가 RunningMatchInfo 인지 진단: +0x140/+0x148(team_id 후보), +8/+0x10(세트 Vec ptr/len 후보) 표시.
#[allow(dead_code)]
unsafe fn probe_candidate(tag: &str, c: u64, s: &mut String) {
    if !looks_ptr(c) || !readable(c as usize, 0x18) { s.push_str(&format!("   {} = 0x{:x} (포인터 아님/역참조 불가)\n", tag, c)); return; }
    let cu = c as usize;
    let v8 = rd_u64(cu + 0x08).unwrap_or(0);   // Vec ptr 후보
    let v10 = rd_u64(cu + 0x10).unwrap_or(0);  // Vec len 후보
    let t140 = rd_u64(cu + 0x140).unwrap_or(0);
    let t148 = rd_u64(cu + 0x148).unwrap_or(0);
    s.push_str(&format!("   {} = 0x{:x}  | +8={:#x}(rd={}) +0x10={} | +0x140={:#x} +0x148={:#x}\n",
        tag, c, v8, readable(v8 as usize, 16), v10 as i64, t140, t148));
}

// ── apply_to_match 진입 캡처 ── self=rcx 확정판. team_id@self+0x140/+0x148(정수), 세트Vec@self+8.
unsafe extern "C" fn apply_capture(saved_ptr: usize, entry_rsp: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved_ptr < 0x10000 || entry_rsp < 0x10000 { return; }
        let hit = HIT_TOTAL.fetch_add(1, Ordering::Relaxed);
        let logged = LOGGED.load(Ordering::Relaxed);
        if logged >= CAP_MAX { return; }
        let base = BASE.load(Ordering::Relaxed);
        let caller = (rd_u64(entry_rsp).unwrap_or(0)).wrapping_sub(base as u64);
        let self_ptr = rd_u64(saved_ptr + OFF_RCX).unwrap_or(0) as usize;   // rcx = RunningMatchInfo*
        if self_ptr < 0x10000 || !readable(self_ptr, 0x150) { return; }
        LOGGED.fetch_add(1, Ordering::Relaxed);
        let t0 = rd_u64(self_ptr + 0x140).unwrap_or(0);   // team_id[0] (정수)
        let t1 = rd_u64(self_ptr + 0x148).unwrap_or(0);   // team_id[1] (정수)
        // 세트결과 Vec: self+8=ptr, self+0x10=len, self+0x18=cap. 원소 stride 0x100, +0xc0=상태(0xd=완료), +0xe8=win bool.
        let vptr = rd_u64(self_ptr + 0x08).unwrap_or(0) as usize;
        let vlen = rd_u64(self_ptr + 0x10).unwrap_or(0) as usize;
        let mut wins_a = 0u32; let mut wins_b = 0u32; let mut sets = String::new();
        if vptr >= 0x10000 && vlen <= 16 {
            for i in 0..vlen {
                let el = vptr + i * 0x100;
                let st = (rd_u64(el + 0xc0).unwrap_or(0) & 0xffff_ffff) as u32;
                let wn = rd_u8(el + 0xe8).unwrap_or(0xff);
                sets.push_str(&format!("(st={} win={}) ", st, wn));
                if st == 0xd { if wn == 0 { wins_a += 1; } else { wins_b += 1; } }
            }
        }
        let winner = if wins_b < wins_a { "team0" } else if wins_a < wins_b { "team1" } else { "?동수" };
        let mut s = format!(
            "[#{} hit{} {}ms caller=RVA0x{:x}] self={:#x}  team0={} team1={}  sets(len={}): {} → winner={} ({}-{})\n",
            logged, hit, now_ms(), caller, self_ptr, t0, t1, vlen, sets, winner, wins_a, wins_b);
        // 첫 8건: 세트원소 구조 해석 출력 — Vec 트리플릿({cap,ptr,len}) 레이아웃 (v5 덤프로 규명):
        //   +0x00/+0x18=팀0/1 선수ID Vec<u64>(5) · +0x30/+0x48=밴 Vec<String>(2) ·
        //   +0x60/+0x78=★챔피언 Vec<String>(5) · +0x90/+0xa8=포지션 Vec<u64>(0-4 순열)
        if logged < 8 && vptr >= 0x10000 && vlen >= 1 {
            for si in 0..vlen.min(3) {
                let el = vptr + si * 0x100;
                let champs0 = read_str_vec(el + 0x60);
                let champs1 = read_str_vec(el + 0x78);
                let bans0 = read_str_vec(el + 0x30);
                let bans1 = read_str_vec(el + 0x48);
                let pos0 = read_u64_vec(el + 0x90);
                let pos1 = read_u64_vec(el + 0xa8);
                let ath0 = read_u64_vec(el + 0x00);
                let ath1 = read_u64_vec(el + 0x18);
                s.push_str(&format!("    set[{}] 팀0챔프={:?} 팀1챔프={:?}\n           밴0={:?} 밴1={:?} pos0={:?} pos1={:?} ath0={:?} ath1={:?}\n",
                    si, champs0, champs1, bans0, bans1, pos0, pos1, ath0, ath1));
            }
        }
        append_log(&s);
    }));
}

// Vec 트리플릿 {cap@+0, ptr@+8, len@+0x10} 리더 (v5 덤프로 레이아웃 확정)
unsafe fn read_u64_vec(at: usize) -> Vec<u64> {
    let ptr = rd_u64(at + 8).unwrap_or(0) as usize;
    let len = rd_u64(at + 0x10).unwrap_or(0) as usize;
    let mut v = Vec::new();
    if ptr >= 0x10000 && len <= 16 {
        for i in 0..len { v.push(rd_u64(ptr + i * 8).unwrap_or(u64::MAX)); }
    }
    v
}
// Vec<String> 리더: 원소=24B String{cap,ptr,len}
unsafe fn read_str_vec(at: usize) -> Vec<String> {
    let ptr = rd_u64(at + 8).unwrap_or(0) as usize;
    let len = rd_u64(at + 0x10).unwrap_or(0) as usize;
    let mut v = Vec::new();
    if ptr >= 0x10000 && len <= 16 {
        for i in 0..len {
            let el = ptr + i * 0x18;
            let sp = rd_u64(el + 8).unwrap_or(0) as usize;
            let sl = (rd_u64(el + 0x10).unwrap_or(0) as usize).min(24);
            let mut name = String::new();
            if sp >= 0x10000 {
                for j in 0..sl { match rd_u8(sp + j) { Some(b) if (0x20..0x7f).contains(&b) => name.push(b as char), _ => break } }
            }
            v.push(name);
        }
    }
    v
}

// 구조체 덤프 + 8B 슬롯 중 유효 포인터 1단계 추적(팀/챔프 중첩 노출 — 챔프이름 ASCII 찾기용)
unsafe fn dump_entry(tag: &str, ptr: usize, len: usize, scan: usize, s: &mut String) {
    dump_mem(tag, ptr, len, s);
    if ptr <= 0x10000 || !readable(ptr, 8) { return; }
    let mut off = 0;
    let mut shown = 0;
    while off + 8 <= scan && shown < 40 {
        if let Some(v) = rd_u64(ptr + off) {
            if looks_ptr(v) && readable(v as usize, 16) {
                let sub = format!("{}+{:03x}->", tag.trim(), off);
                dump_mem(&sub, v as usize, 0x60, s);
                shown += 1;
            }
        }
        off += 8;
    }
}

// ── 경기시작 캡처 A: 매치 등록(0x1e38ce0) — config 0x740 전체 덤프(team_id/챔프로스터 오프셋 실측) ──
static REG_LOGGED: AtomicU64 = AtomicU64::new(0);
static REG_HITS: AtomicU64 = AtomicU64::new(0);
const REG_CAP_MAX: u64 = 3;
unsafe extern "C" fn reg_capture(saved_ptr: usize, entry_rsp: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved_ptr < 0x10000 || entry_rsp < 0x10000 { return; }
        let hit = REG_HITS.fetch_add(1, Ordering::Relaxed);
        let n = REG_LOGGED.load(Ordering::Relaxed);
        if n >= REG_CAP_MAX { return; }
        REG_LOGGED.fetch_add(1, Ordering::Relaxed);
        let base = BASE.load(Ordering::Relaxed);
        let caller = (rd_u64(entry_rsp).unwrap_or(0)).wrapping_sub(base as u64);
        let rcx = rd_u64(saved_ptr + OFF_RCX).unwrap_or(0);   // 게임상태
        let rdx = rd_u64(saved_ptr + OFF_RDX).unwrap_or(0);   // 매치config(0x740)
        let mut s = format!(
            "\n[★REG(경기등록) #{} hit{} {}ms caller=RVA0x{:x}] 게임상태={:#x} config={:#x}\n",
            n, hit, now_ms(), caller, rcx, rdx);
        if looks_ptr(rdx) {
            // config 0x740 전체 + 포인터 1단계 추적 → team_id(작은정수쌍)·챔프이름(ASCII) 위치 실측
            dump_entry("config", rdx as usize, 0x740, 0x740, &mut s);
        }
        append_log(&s);
    }));
}

// ── 경기시작 캡처 B: sim tick(0x204f810) — (GameData, config) 쌍 확인(브리지: sim서 config로 team_id 직독 가능?) ──
static TICK_LAST_CFG: AtomicU64 = AtomicU64::new(0);
static TICK_LOGGED: AtomicU64 = AtomicU64::new(0);
static TICK_DUMPED: AtomicU64 = AtomicU64::new(0);
const TICK_CAP_MAX: u64 = 30;
unsafe extern "C" fn tick_capture(saved_ptr: usize, entry_rsp: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved_ptr < 0x10000 || entry_rsp < 0x10000 { return; }
        let r9 = rd_u64(saved_ptr + OFF_R9).unwrap_or(0);     // param4 = 매치config 후보
        // dedup: 같은 config 연속 틱은 스킵(핫패스 최소화)
        if TICK_LAST_CFG.swap(r9, Ordering::Relaxed) == r9 { return; }
        let n = TICK_LOGGED.load(Ordering::Relaxed);
        if n >= TICK_CAP_MAX { return; }
        TICK_LOGGED.fetch_add(1, Ordering::Relaxed);
        let rdx = rd_u64(saved_ptr + OFF_RDX).unwrap_or(0);   // param2 = GameData 후보
        let r8  = rd_u64(saved_ptr + OFF_R8).unwrap_or(0);
        let p5  = rd_u64(entry_rsp + 0x28).unwrap_or(0);
        // config+0x6a8 = team index(0/1) — ghidra-re 확인 필드로 config 정체 검증
        let tidx = if looks_ptr(r9) { rd_u64(r9 as usize + 0x6a8).unwrap_or(u64::MAX) } else { u64::MAX };
        let mut s = format!(
            "[TICK #{} {}ms] GameData(rdx)={:#x} r8={:#x} config(r9)={:#x} p5={:#x} | config+0x6a8(team idx?)={:#x}\n",
            n, now_ms(), rdx, r8, r9, p5, tidx);
        // 첫 6개 config: (a)작은정수(<0x200) 전수 스캔표 → apply의 team_id쌍과 대조해 config내 team_id 오프셋 확정
        //   (b)+0x80/+0x398 로스터 이름(stride 0x10 인라인 8B) → apply 세트 챔프와 매치 대조(같은 경기 식별)
        if looks_ptr(r9) && TICK_DUMPED.load(Ordering::Relaxed) < 6 {
            TICK_DUMPED.fetch_add(1, Ordering::Relaxed);
            let c = r9 as usize;
            s.push_str("    smallints:");
            let mut off = 0usize;
            while off < 0x740 {
                if let Some(v) = rd_u64(c + off) { if v > 0 && v < 0x200 { s.push_str(&format!(" +{:x}={}", off, v)); } }
                off += 8;
            }
            s.push('\n');
            for (tag, roff) in [("로스터A(+0x80)", 0x80usize), ("로스터B(+0x398)", 0x398usize)] {
                let vp = rd_u64(c + roff + 8).unwrap_or(0) as usize;
                let vl = (rd_u64(c + roff + 0x10).unwrap_or(0) as usize).min(8);
                let mut names = Vec::new();
                if vp >= 0x10000 {
                    for i in 0..vl {
                        let mut nm = String::new();
                        for j in 0..8 { match rd_u8(vp + i*0x10 + j) { Some(b) if (0x20..0x7f).contains(&b) => nm.push(b as char), _ => break } }
                        names.push(nm);
                    }
                }
                s.push_str(&format!("    {}: {:?}\n", tag, names));
            }
        }
        append_log(&s);
    }));
}

// 일반화 트램폴린(sim_probe 검증): copy_len 위치독립 바이트 가로채 cap_fn 호출 후 원본실행→jmp fn+copy_len.
unsafe fn install_hook_at(rva: usize, copy_len: usize, cap_fn: usize) -> Result<(), &'static str> {
    let mbase = GetModuleHandleW(core::ptr::null());
    if mbase == 0 { return Err("module base 0"); }
    BASE.store(mbase, Ordering::Relaxed);
    let fn_addr = mbase + rva;
    if !readable(fn_addr, copy_len.max(12)) { return Err("fn not readable"); }
    let mut orig = [0u8; 16];
    for i in 0..copy_len { orig[i] = *((fn_addr+i) as *const u8); }
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc 실패"); }
    let ret_addr = fn_addr + copy_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);                          // mov r10, rsp  (= entry_rsp)
    s.extend_from_slice(&[0x51,0x52]);                              // push rcx; push rdx
    s.extend_from_slice(&[0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]);// push r8 r9 r10 r11
    s.extend_from_slice(&[0x48,0x89,0xe1]);                         // mov rcx, rsp  (= saved_ptr)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);                         // mov rdx, r10  (= entry_rsp)
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);                    // sub rsp, 0x28 (shadow+align)
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes()); // mov rax, cap_fn
    s.extend_from_slice(&[0xff,0xd0]);                              // call rax
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);                    // add rsp, 0x28
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58]);// pop r11 r10 r9 r8
    s.extend_from_slice(&[0x5a,0x59]);                             // pop rdx; pop rcx
    s.extend_from_slice(&orig[..copy_len]);                        // 원본 가로챈 copy_len 바이트
    // ★점프백 = rax-보존형 jmp qword [rip+0] (movabs rax는 원본 prologue가 세팅한 eax를 파괴 —
    //   0x1e38ce0처럼 "mov eax,프레임크기" 직후 chkstk 함수면 eax=주소값으로 스택프로브 → 즉사. sim_probe SEED 크래시 동일원인)
    s.extend_from_slice(&[0xff,0x25,0x00,0x00,0x00,0x00]);          // jmp qword [rip+0]
    s.extend_from_slice(&ret_addr.to_le_bytes());                   // 8B 절대 복귀주소
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = [0u8; 12];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); // mov rax, stub
    patch[10]=0xff; patch[11]=0xe0;                                                  // jmp rax
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, RWX, &mut old) == 0 { return Err("VirtualProtect 실패"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    Ok(())
}

unsafe fn install_hook() -> Result<(), &'static str> {
    if HOOK_INSTALLED.swap(true, Ordering::Relaxed) { return Err("이미 설치됨"); }
    let mbase = GetModuleHandleW(core::ptr::null());
    let fn_addr = mbase + APPLY_RVA;
    if !readable(fn_addr, 16) { return Err("fn not readable"); }
    for i in 0..12 { if *((fn_addr+i) as *const u8) != PROLOGUE[i] { return Err("프롤로그 불일치(8-push 아님, RVA 확인)"); } }
    install_hook_at(APPLY_RVA, 12, apply_capture as usize)
}

fn key_down(vk: i32) -> bool { (unsafe { GetAsyncKeyState(vk) } as u16 & 0x8000) != 0 }

struct ProbeExt;
impl ModExtension for ProbeExt {
    fn post_update(&self, _scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        if !BOOTED.swap(true, Ordering::Relaxed) {
            append_log(&format!("[{}ms] [ext] 폴링 시작 — 경기 완료(일정넘김/관전)마다 자동 로깅. F3=카운터 리셋(더 캡처).\n", now_ms()));
        }
        let f3 = key_down(VK_F3);
        if f3 && !PREV_F3.swap(true, Ordering::Relaxed) {
            LOGGED.store(0, Ordering::Relaxed);
            REG_LOGGED.store(0, Ordering::Relaxed);
            TICK_LOGGED.store(0, Ordering::Relaxed);
            TICK_DUMPED.store(0, Ordering::Relaxed);
            append_log(&format!("\n[{}ms] ===== F3: 카운터 리셋 (누적 apply {} / reg {}) =====\n",
                now_ms(), HIT_TOTAL.load(Ordering::Relaxed), REG_HITS.load(Ordering::Relaxed)));
        } else if !f3 { PREV_F3.store(false, Ordering::Relaxed); }
    }
}

// REG(0x1e38ce0, 13B 위치독립) 프롤로그 검증 후 설치
unsafe fn install_reg_hook() -> Result<(), &'static str> {
    let mbase = GetModuleHandleW(core::ptr::null());
    let fn_addr = mbase + REG_RVA;
    if !readable(fn_addr, 16) { return Err("REG fn not readable"); }
    for i in 0..13 { if *((fn_addr+i) as *const u8) != REG_PROLOGUE[i] { return Err("REG 프롤로그 불일치(6push+mov 아님)"); } }
    install_hook_at(REG_RVA, 13, reg_capture as usize)
}
// TICK(0x204f810, 8-push 12B) 프롤로그 검증 후 설치
unsafe fn install_tick_hook() -> Result<(), &'static str> {
    let mbase = GetModuleHandleW(core::ptr::null());
    let fn_addr = mbase + TICK_RVA;
    if !readable(fn_addr, 16) { return Err("TICK fn not readable"); }
    for i in 0..12 { if *((fn_addr+i) as *const u8) != PROLOGUE[i] { return Err("TICK 프롤로그 불일치(8-push 아님)"); } }
    install_hook_at(TICK_RVA, 12, tick_capture as usize)
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    append_log(&format!("[{}ms] === tfm2_tactics_probe INIT (apply 0x{:x} / reg 0x{:x} / tick 0x{:x}) ===\n", now_ms(), APPLY_RVA, REG_RVA, TICK_RVA));
    unsafe {
        match install_hook() {
            Ok(()) => append_log("[hook] apply_to_match(0x1d9ba30) detour OK (8-push)\n"),
            Err(e) => append_log(&format!("[hook] apply 설치 실패: {}\n", e)),
        }
        match install_reg_hook() {
            Ok(()) => append_log("[hook] REG 경기등록(0x1e38ce0) detour OK (13B, chkstk 밖)\n"),
            Err(e) => append_log(&format!("[hook] REG 설치 실패: {}\n", e)),
        }
        match install_tick_hook() {
            Ok(()) => append_log("[hook] TICK sim tick(0x204f810) detour OK (8-push)\n"),
            Err(e) => append_log(&format!("[hook] TICK 설치 실패: {}\n", e)),
        }
    }
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(ProbeExt);
    reg
}
declare_mod!(init);
