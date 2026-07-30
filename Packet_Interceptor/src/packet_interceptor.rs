// packet_interceptor.rs — ClientPacket 로컬 채널 MITM (가로채기 + 값 변조/통과)
// 빌드: build_mod.bat "C:\tfm2mods\Packet_Interceptor\src\packet_interceptor.rs" → packet_interceptor.dll
// 배치: 게임\mods\packet_interceptor\ 에 dll + mod.mod_info + rules.txt 복사, 전체 재시작.
// 로그: mods\packet_interceptor\packet_interceptor.txt
//
// 구조: 클라이언트 → (crossbeam-channel) → 로컬서버 packet_handler → 우리 훅 → 원본처리
//   훅 지점: FUN_140ee4b80 (RVA 0xee4b80, game-view/src/logic/server/packet_handler.rs)
//   진입 레지스터 규약(x64):
//     R8  = ClientPacket discriminant (패킷 종류) ← 끝의 점프테이블(0x1433d963c)이 이 값으로 분기
//     RCX = param1, RDX = param2, R9 = param4, [rsp+0x28]=param5, [rsp+0x30]=param6  (페이로드)
//   프롤로그 12B 전부 PUSH(55 41 57 41 56 41 55 41 54 56 57 53) → 훅 안전(plan_probe와 동일).
//
// ★ 변조 원리: 트램폴린 스텁이 rcx/rdx/r8/r9 를 스택에 push → 우리 함수에 그 포인터(saved_ptr) 전달 →
//   함수 복귀 후 stub 이 다시 pop 하여 레지스터 복원. 즉 saved_ptr 의 슬롯을 덮어쓰면
//   "변조된 인자"로 원본 packet_handler 가 실행됨. 안 건드리면 그대로 통과(pass-through).
//
// 키: (plan_probe가 F7~F11을 쓰므로 충돌 피해 F3~F5 사용)
//   F3  = 로깅 무장(다음 N개 패킷 덤프; disc + 6슬롯 + 포인터 메모리 hex/ascii). 어떤 disc가 무슨 행동인지·필드 위치 관찰용.
//   F4  = rules.txt 리로드(변조 규칙 다시 읽기)
//   F5  = 변조 ON/OFF 토글(기본 OFF = 전부 통과). ON이면 rules.txt 규칙대로 슬롯/메모리 덮어씀.
//
// rules.txt 형식(각 줄):
//   when <disc>                     이후 규칙이 적용될 패킷 discriminant (10진 또는 0x..)
//   set <slot> <hexval>             슬롯(레지스터/스택값) 8바이트 덮어쓰기
//   setmem <slot> <off> <sz> <hex>  *(슬롯)+off 위치에 sz(1/2/4/8)바이트 쓰기 (포인터가 가리키는 구조체 필드)
//   slot ∈ rcx rdx r8 r9 p5 p6  (r8=discriminant — 바꾸면 위험)
//   '#' 주석, 빈 줄 무시. 예시는 동봉 rules.txt 참고.
//
// ⚠ 주의: r8(discriminant) 변경이나 잘못된 setmem 은 sim 불일치/크래시 유발 가능. 먼저 F8로 관찰 후 좁게 규칙 작성.

use mod_api::*;
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "packet_interceptor";
const PKT_RVA: usize = 0x144faa0;                    // 0.4.14 FUN_14144faa0 packet_handler dispatch (구 0.4.12 0xee4b80). disc jmptbl=0x1434b288c
const PROLOGUE: [u8; 12] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];

// saved_ptr(스텁이 push한 레지스터들) 기준 슬롯 오프셋
const OFF_R9:  usize = 0x10;
const OFF_R8:  usize = 0x18;   // discriminant
const OFF_RDX: usize = 0x20;
const OFF_RCX: usize = 0x28;
// entry_rsp 기준 스택 인자(파라미터 5/6)
const OFF_P5: usize = 0x28;
const OFF_P6: usize = 0x30;

const LOG_LIMIT: u64 = 256;                          // F3 무장 시 덤프할 최대 패킷 수

// ── F6 예산 세팅 (DB 직접 쓰기) ── Team 구조체(PDB 확정, 값은 f64):
//   AdjustBudget 패킷은 전체잔고를 이적/연봉으로 "나누기"만 함 → 돈 증가엔 부적합.
//   ClientData.db_mut()로 내 팀 Team 의 잔고/예산 필드를 직접 기록하는 게 정확.
const OFF_TOTAL_BALANCE:   usize = 0x3c0;   // total_balance: f64 (전체 잔고 = 실제 돈)
const OFF_TRANSFER_BUDGET: usize = 0x3c8;   // transfer_budget: f64 (이적 예산)
const OFF_SALARY_BUDGET:   usize = 0x3d0;   // salary_budget: f64 (연봉 예산)
const BUDGET_VALUE: f64 = 6_666_666_666.0;  // F6으로 세팅할 값

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
const VK_F4: i32 = 0x73;
const VK_F5: i32 = 0x74;
const VK_F6: i32 = 0x75;

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
unsafe fn writable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mbi = match region(addr) { Some(m) => m, None => return false };
    const COMMIT: u32 = 0x1000;
    const WR: u32 = 0x04|0x08|0x40|0x80;     // READWRITE | WRITECOPY | EXECUTE_READWRITE | EXECUTE_WRITECOPY
    const GUARD: u32 = 0x01|0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & WR == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}
unsafe fn rd_u64(a: usize) -> Option<u64> { if readable(a,8){Some(std::ptr::read_unaligned(a as *const u64))}else{None} }
unsafe fn rd_u8(a: usize) -> Option<u8> { if readable(a,1){Some(std::ptr::read_unaligned(a as *const u8))}else{None} }

// sz(1/2/4/8) 바이트를 addr 에 기록. 스택 슬롯/게임 힙 모두 RW 라 대개 가능. 안전 위해 writable 확인.
unsafe fn wr_sized(addr: usize, sz: usize, val: u64) -> bool {
    if !writable(addr, sz) { return false; }
    match sz {
        1 => std::ptr::write_unaligned(addr as *mut u8,  val as u8),
        2 => std::ptr::write_unaligned(addr as *mut u16, val as u16),
        4 => std::ptr::write_unaligned(addr as *mut u32, val as u32),
        8 => std::ptr::write_unaligned(addr as *mut u64, val),
        _ => return false,
    }
    true
}

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
        p.push("packet_interceptor.txt");
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = write!(f,"{}",line); }
    }
}

// ── 변조 규칙 ──
#[derive(Clone, Copy)]
enum Slot { Rcx, Rdx, R8, R9, P5, P6 }
impl Slot {
    fn parse(s: &str) -> Option<Slot> {
        match s.to_ascii_lowercase().as_str() {
            "rcx"=>Some(Slot::Rcx), "rdx"=>Some(Slot::Rdx), "r8"=>Some(Slot::R8),
            "r9"=>Some(Slot::R9), "p5"=>Some(Slot::P5), "p6"=>Some(Slot::P6), _=>None,
        }
    }
    // 이 슬롯의 "값이 저장된 주소"
    fn addr(self, saved_ptr: usize, entry_rsp: usize) -> usize {
        match self {
            Slot::Rcx=>saved_ptr+OFF_RCX, Slot::Rdx=>saved_ptr+OFF_RDX, Slot::R8=>saved_ptr+OFF_R8,
            Slot::R9=>saved_ptr+OFF_R9, Slot::P5=>entry_rsp+OFF_P5, Slot::P6=>entry_rsp+OFF_P6,
        }
    }
}
#[derive(Clone, Copy)]
enum Op {
    SetSlot { slot: Slot, val: u64 },                       // 슬롯값(레지스터) 8B 덮어쓰기
    SetMem  { slot: Slot, off: usize, sz: usize, val: u64 },// *(슬롯)+off 에 sz바이트 쓰기
}
#[derive(Clone, Copy)]
struct Rule { disc: u64, op: Op }

static RULES: Mutex<Vec<Rule>> = Mutex::new(Vec::new());

fn parse_num(s: &str) -> Option<u64> {
    let s = s.trim();
    if let Some(h) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u64::from_str_radix(h, 16).ok()
    } else { s.parse::<u64>().ok() }
}
fn load_rules() -> (usize, String) {
    let mut path = match mod_dir() { Some(p)=>p, None=>return (0, "mod_dir 실패".into()) };
    path.push("rules.txt");
    let text = match fs::read_to_string(&path) { Ok(t)=>t, Err(e)=>return (0, format!("rules.txt 읽기 실패: {}", e)) };
    let mut out: Vec<Rule> = Vec::new();
    let mut cur: Option<u64> = None;
    let mut errs = String::new();
    for (ln, raw) in text.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() { continue; }
        let tok: Vec<&str> = line.split_whitespace().collect();
        match tok[0].to_ascii_lowercase().as_str() {
            "when" => {
                if tok.len()>=2 { if let Some(d)=parse_num(tok[1]) { cur=Some(d); continue; } }
                errs.push_str(&format!("  L{}: when 파싱 실패\n", ln+1));
            }
            "set" => {
                if let (Some(d), true) = (cur, tok.len()>=3) {
                    if let (Some(sl), Some(v)) = (Slot::parse(tok[1]), parse_num(tok[2])) {
                        out.push(Rule{disc:d, op:Op::SetSlot{slot:sl, val:v}}); continue;
                    }
                }
                errs.push_str(&format!("  L{}: set 파싱 실패(when 먼저?)\n", ln+1));
            }
            "setmem" => {
                if let (Some(d), true) = (cur, tok.len()>=5) {
                    if let (Some(sl), Some(off), Some(sz), Some(v)) =
                        (Slot::parse(tok[1]), parse_num(tok[2]).map(|x|x as usize),
                         parse_num(tok[3]).map(|x|x as usize), parse_num(tok[4])) {
                        if matches!(sz,1|2|4|8) {
                            out.push(Rule{disc:d, op:Op::SetMem{slot:sl, off, sz, val:v}}); continue;
                        }
                    }
                }
                errs.push_str(&format!("  L{}: setmem 파싱 실패(sz는 1/2/4/8)\n", ln+1));
            }
            _ => errs.push_str(&format!("  L{}: 알 수 없는 키워드 '{}'\n", ln+1, tok[0])),
        }
    }
    let n = out.len();
    if let Ok(mut g) = RULES.lock() { *g = out; }
    (n, errs)
}

static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
static HIT_TOTAL: AtomicU64 = AtomicU64::new(0);     // 진단: 훅이 불린 총 횟수
static MUTATE_ON: AtomicBool = AtomicBool::new(false);
static LOG_ARMED: AtomicBool = AtomicBool::new(false);
static LOG_COUNT: AtomicU64 = AtomicU64::new(0);
static LAST_KEY: AtomicU64 = AtomicU64::new(0);
static BOOTED: AtomicBool = AtomicBool::new(false);
static PREV_F3: AtomicBool = AtomicBool::new(false);
static PREV_F4: AtomicBool = AtomicBool::new(false);
static PREV_F5: AtomicBool = AtomicBool::new(false);
static PREV_F6: AtomicBool = AtomicBool::new(false);

// 포인터가 가리키는 메모리 첫 len 바이트 hex+ascii
unsafe fn dump_mem(tag: &str, ptr: usize, len: usize, s: &mut String) {
    if ptr <= 0x10000 || !readable(ptr, 16) { s.push_str(&format!("     {}=0x{:x} (역참조 불가)\n", tag, ptr)); return; }
    let n = len.min(0x40);
    let mut hex = String::new(); let mut asc = String::new();
    for i in 0..n {
        match rd_u8(ptr+i) {
            Some(b) => { hex.push_str(&format!("{:02x} ", b));
                asc.push(if (0x20..0x7f).contains(&b) { b as char } else { '.' }); }
            None => { hex.push_str("?? "); asc.push('.'); }
        }
    }
    s.push_str(&format!("     {}=0x{:x}: {}\n        |{}|\n", tag, ptr, hex.trim_end(), asc));
}

// ── 훅 본체: 패킷 1건당 호출 ──
unsafe extern "C" fn pkt_intercept(saved_ptr: usize, entry_rsp: usize) {
    if saved_ptr < 0x10000 || entry_rsp < 0x10000 { return; }
    let disc = rd_u64(saved_ptr+OFF_R8).unwrap_or(u64::MAX);
    let rcx  = rd_u64(saved_ptr+OFF_RCX).unwrap_or(0);
    let rdx  = rd_u64(saved_ptr+OFF_RDX).unwrap_or(0);
    let r9   = rd_u64(saved_ptr+OFF_R9).unwrap_or(0);
    let p5   = rd_u64(entry_rsp+OFF_P5).unwrap_or(0);
    let p6   = rd_u64(entry_rsp+OFF_P6).unwrap_or(0);

    // ── 0) 진단: 훅이 불리는지 무조건 확인 (F3 무관, 첫 8회만) ──
    let hit = HIT_TOTAL.fetch_add(1, Ordering::Relaxed);
    if hit < 8 {
        append_log(&format!("[hit #{} {}ms] disc={} (0x{:x}) rcx=0x{:x} rdx=0x{:x} r9=0x{:x} p5=0x{:x} p6=0x{:x}\n",
            hit, now_ms(), disc as i64, disc, rcx, rdx, r9, p5, p6));
    }

    // ── 1) 변조 (ON 일 때만, disc 일치 규칙 적용) ──
    let mut applied: Vec<String> = Vec::new();
    if MUTATE_ON.load(Ordering::Relaxed) && disc != u64::MAX {
        if let Ok(g) = RULES.try_lock() {
            for r in g.iter().filter(|r| r.disc == disc) {
                match r.op {
                    Op::SetSlot{slot, val} => {
                        let a = slot.addr(saved_ptr, entry_rsp);
                        let ok = wr_sized(a, 8, val);
                        applied.push(format!("set@{:#x}={:#x}({})", a, val, if ok {"OK"} else {"실패"}));
                    }
                    Op::SetMem{slot, off, sz, val} => {
                        let base = rd_u64(slot.addr(saved_ptr, entry_rsp)).unwrap_or(0) as usize;
                        let a = base.wrapping_add(off);
                        let ok = base>0x10000 && wr_sized(a, sz, val);
                        applied.push(format!("setmem@{:#x}({}B)={:#x}({})", a, sz, val, if ok {"OK"} else {"실패"}));
                    }
                }
            }
        }
    }

    // ── 2) 로깅 (무장 시) ──
    if LOG_ARMED.load(Ordering::Relaxed) {
        // 중복 제거: 같은 (disc,rcx,rdx,r9) 연속 무시 (틱 반복/no-op 패킷 스팸 방지)
        let key = disc ^ rcx.rotate_left(7) ^ rdx.rotate_left(17) ^ r9.rotate_left(29);
        if LAST_KEY.swap(key, Ordering::Relaxed) != key {
            let c = LOG_COUNT.fetch_add(1, Ordering::Relaxed);
            if c < LOG_LIMIT {
                let mut s = format!("\n[pkt #{} {}ms] disc={} (0x{:x})  rcx=0x{:x} rdx=0x{:x} r9=0x{:x} p5=0x{:x} p6=0x{:x}\n",
                    c, now_ms(), disc as i64, disc, rcx, rdx, r9, p5, p6);
                if !applied.is_empty() { s.push_str(&format!("     ★변조적용: {}\n", applied.join(", "))); }
                dump_mem("*rcx", rcx as usize, 0x40, &mut s);
                dump_mem("*rdx", rdx as usize, 0x40, &mut s);
                dump_mem("*r9 ", r9 as usize, 0x40, &mut s);
                append_log(&s);
                if c+1 == LOG_LIMIT {
                    LOG_ARMED.store(false, Ordering::Relaxed);
                    append_log(&format!("[{}ms] (로그 {}건 도달 — 무장 해제)\n", now_ms(), LOG_LIMIT));
                }
            }
        }
    } else if !applied.is_empty() {
        // 비무장이어도 변조는 1줄 기록
        append_log(&format!("[{}ms] 변조 disc={}: {}\n", now_ms(), disc as i64, applied.join(", ")));
    }
}

unsafe fn install_hook() -> Result<(), &'static str> {
    if HOOK_INSTALLED.swap(true, Ordering::Relaxed) { return Err("이미 설치됨"); }
    let mbase = GetModuleHandleW(core::ptr::null());
    if mbase == 0 { return Err("module base 0"); }
    let fn_addr = mbase + PKT_RVA;
    if !readable(fn_addr, 16) { return Err("fn not readable"); }
    for i in 0..12 { if *((fn_addr+i) as *const u8) != PROLOGUE[i] { return Err("프롤로그 바이트 불일치(게임 업데이트?)"); } }

    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc 실패"); }
    let cap_fn = pkt_intercept as usize;
    let ret_addr = fn_addr + 12;
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
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58]);// pop r11 r10 r9 r8 (변조값 복원)
    s.extend_from_slice(&[0x5a,0x59]);                             // pop rdx; pop rcx
    s.extend_from_slice(&PROLOGUE);                                 // 원본 가로챈 12B
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes()); // mov rax, fn+12
    s.extend_from_slice(&[0xff,0xe0]);                              // jmp rax
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

fn key_down(vk: i32) -> bool { (unsafe { GetAsyncKeyState(vk) } as u16 & 0x8000) != 0 }

struct InterceptExt;
impl ModExtension for InterceptExt {
    fn post_update(&self, scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        if !BOOTED.swap(true, Ordering::Relaxed) {
            let (n, _) = load_rules();
            append_log(&format!("[{}ms] [ext] 폴링 시작 — F3=로깅무장 F4=규칙리로드 F5=변조토글 F6=예산세팅 | 초기 규칙 {}건\n", now_ms(), n));
        }
        // F6 = 내 팀 예산을 BUDGET_VALUE 로 직접 세팅 (관리화면에서)
        let f6 = key_down(VK_F6); if f6 && !PREV_F6.swap(f6, Ordering::Relaxed) {
            if let Scene::InGame { data } = scene {
                let tid = data.player_team_id();
                let mut db = data.db_mut();
                if let Some(team) = db.teams.get_mut(&tid) {
                    let base = team as *mut Team as usize;
                    unsafe {
                        *((base + OFF_TOTAL_BALANCE)   as *mut f64) = BUDGET_VALUE;
                        *((base + OFF_TRANSFER_BUDGET) as *mut f64) = BUDGET_VALUE;
                        *((base + OFF_SALARY_BUDGET)   as *mut f64) = BUDGET_VALUE;
                    }
                    append_log(&format!("[{}ms] ★F6 — team {} 예산 → {} (total/transfer/salary)\n", now_ms(), tid, BUDGET_VALUE));
                } else {
                    append_log(&format!("[{}ms] F6 — team {} 못 찾음\n", now_ms(), tid));
                }
            } else {
                append_log(&format!("[{}ms] F6 — InGame 아님(관리 화면에서 눌러야 함)\n", now_ms()));
            }
        } else if !f6 { PREV_F6.store(false, Ordering::Relaxed); }
        let f3 = key_down(VK_F3); if f3 && !PREV_F3.swap(f3, Ordering::Relaxed) {
            LOG_COUNT.store(0, Ordering::Relaxed); LAST_KEY.store(0, Ordering::Relaxed);
            LOG_ARMED.store(true, Ordering::Relaxed);
            append_log(&format!("\n[{}ms] ===== F3 로깅 무장 (다음 {}건) | 누적 훅히트 {}회 =====\n",
                now_ms(), LOG_LIMIT, HIT_TOTAL.load(Ordering::Relaxed)));
        } else if !f3 { PREV_F3.store(false, Ordering::Relaxed); }

        let f4 = key_down(VK_F4); if f4 && !PREV_F4.swap(f4, Ordering::Relaxed) {
            let (n, errs) = load_rules();
            append_log(&format!("[{}ms] F4 규칙 리로드: {}건{}\n", now_ms(), n,
                if errs.is_empty() { String::new() } else { format!("\n{}", errs) }));
        } else if !f4 { PREV_F4.store(false, Ordering::Relaxed); }

        let f5 = key_down(VK_F5); if f5 && !PREV_F5.swap(f5, Ordering::Relaxed) {
            let v = !MUTATE_ON.load(Ordering::Relaxed); MUTATE_ON.store(v, Ordering::Relaxed);
            append_log(&format!("[{}ms] F5 변조 {}\n", now_ms(), if v {"ON ★"} else {"OFF(통과)"}));
        } else if !f5 { PREV_F5.store(false, Ordering::Relaxed); }
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    append_log(&format!("[{}ms] === packet_interceptor INIT ===\n", now_ms()));
    unsafe {
        match install_hook() {
            Ok(()) => append_log("[hook] packet_handler(0xee4b80) 설치 성공\n"),
            Err(e) => append_log(&format!("[hook] 설치 실패: {}\n", e)),
        }
    }
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(InterceptExt);
    reg
}
declare_mod!(init);
