//! tfm2_judge_verify060 — 0.6.0 judge 검증 하네스(stable 껍데기 + 네이티브 캡처 detour).
//!
//! 0.5.8 하네스(`tfm2_judge_verify`)는 클래식 SDK dll 이었는데 0.6.0 로더가 거부하므로, stable 모드 dll 안에서
//! 같은 진입부 12B 캡처 detour(tfm2_ver_probe 로 09-17 실증)를 설치한다.
//!
//! ★1단계(이 파일) = 진입부 프로브: 명세 함수(`probe_tbl.rs` · 260) 마다 발화수 + 콜러 리턴 RVA(≤8) 를 센다.
//!   켜는 법(기본 OFF): `<게임>\mods\tfm2_judge_verify060\probe060_on.txt` 한 줄에
//!     `all` / `idx,idx,…` / `name:<부분문자열>` / `rva:<hex>,…` (`#` 주석) — 게임 재시작.
//!   출력: 같은 폴더 `probe060.txt` — INIT · 설치 결과(프롤로그 불일치는 skip) · 5초마다 변화분 · 콜러 RVA.
//! ★2단계(sweep · game==mine) 는 재현체(명세에서 직접 쓴 Rust) 가 생기면 `cap_fn` 자리에 「원본 실행 → 내 재현 실행 → 비교」
//!   를 얹는다(0.5.8 sweep20.rs 의 SRET_LIVE/ARG_SNAP 기구 이식 대상 · 이 파일 범위 밖).
//! ⚠ 프로브와 sweep 은 같은 진입부를 패치하므로 한 함수에 동시에 걸 수 없다(0.5.8 과 동일 원칙).
use mod_api_stable::{declare_stable_mod, LogLevel, StableClient, StableExtension, StableHost, StableMod};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
mod probe_tbl;
use probe_tbl::{Probe, PROBES, N};

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleFileNameW(module: usize, buf: *mut u16, size: u32) -> u32;
    fn VirtualAlloc(addr: usize, size: usize, ty: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, prot: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(h: usize, addr: usize, size: usize) -> i32;
    fn GetCurrentProcess() -> usize;
}

fn mod_dir() -> Option<String> {
    let mut buf = [0u16; 520];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n == 0 || n >= buf.len() { return None; }
    let exe = String::from_utf16_lossy(&buf[..n]);
    exe.rfind(|c| c == '\\' || c == '/').map(|i| format!(r"{}\mods\tfm2_judge_verify060", &exe[..i]))
}
fn w(s: &str) {
    if let Some(d) = mod_dir() {
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(format!(r"{}\probe060.txt", d)) { let _ = writeln!(f, "{}", s); }
    }
}

// ── 카운터(캡처 안에서는 원자만 · format!/alloc 금지) ──
const RET_SLOTS: usize = 8;
static HITS: [AtomicU64; N] = [const { AtomicU64::new(0) }; N];
static RETS: [AtomicUsize; N * RET_SLOTS] = [const { AtomicUsize::new(0) }; N * RET_SLOTS];
static RET_OVF: [AtomicU64; N] = [const { AtomicU64::new(0) }; N];
static INSTALLED: [AtomicUsize; N] = [const { AtomicUsize::new(0) }; N]; // 0 미설치 · 1 설치 · 2 프롤로그 불일치 · 3 실패
static BASE: AtomicUsize = AtomicUsize::new(0);
static DONE: AtomicUsize = AtomicUsize::new(0);
static FRAME: AtomicU64 = AtomicU64::new(0);
static LAST: Mutex<Vec<u64>> = Mutex::new(Vec::new());

/// 스텁 레이아웃(push r12 rsi rdi rbx r11 r10 r9 r8 rdx rcx): saved+0 rcx … · 원본 rsp = saved+0x50 · [원본 rsp] = 리턴 주소
unsafe extern "C" fn cap(saved: *const usize, idx: u32) {
    let i = idx as usize;
    if i >= N { return; }
    HITS[i].fetch_add(1, Ordering::Relaxed);
    let ret = *((saved as usize + 0x50) as *const usize);
    let rva = ret.wrapping_sub(BASE.load(Ordering::Relaxed));
    let base = i * RET_SLOTS;
    for k in 0..RET_SLOTS {
        let cur = RETS[base + k].load(Ordering::Relaxed);
        if cur == rva { return; }
        if cur == 0 { if RETS[base + k].compare_exchange(0, rva, Ordering::Relaxed, Ordering::Relaxed).is_ok() { return; } }
    }
    RET_OVF[i].fetch_add(1, Ordering::Relaxed);
}

/// 진입부 캡처 detour: 진입 orig.len() 바이트를 `movabs rax,stub; jmp rax`(12B)+NOP 로 덮고,
/// 스텁 = push 10 regs → cap(saved, idx) → pop → 원본 바이트 → jmp fn+orig.len()
unsafe fn install(p: &Probe, stub: usize) -> usize {
    let base = BASE.load(Ordering::Relaxed); let fn_addr = base + p.rva; let n = p.orig.len();
    if n < 12 || n > 40 { return 3; }
    for k in 0..n { if *((fn_addr + k) as *const u8) != p.orig[k] { return 2; } }
    let mut s: Vec<u8> = Vec::with_capacity(160);
    s.extend_from_slice(&[0x41, 0x54, 0x56, 0x57, 0x53, 0x41, 0x53, 0x41, 0x52, 0x41, 0x51, 0x41, 0x50, 0x52, 0x51]); // push r12 rsi rdi rbx r11 r10 r9 r8 rdx rcx
    s.extend_from_slice(&[0x48, 0x89, 0xe1]);       // mov rcx, rsp
    s.extend_from_slice(&[0xba]); s.extend_from_slice(&(p.idx as u32).to_le_bytes()); // mov edx, idx
    s.extend_from_slice(&[0x48, 0x89, 0xe3]);       // mov rbx, rsp
    s.extend_from_slice(&[0x48, 0x83, 0xe4, 0xf0]); // and rsp, -16
    s.extend_from_slice(&[0x48, 0x83, 0xec, 0x20]); // sub rsp, 0x20
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&(cap as usize).to_le_bytes()); // movabs rax, cap
    s.extend_from_slice(&[0xff, 0xd0]);             // call rax
    s.extend_from_slice(&[0x48, 0x89, 0xdc]);       // mov rsp, rbx
    s.extend_from_slice(&[0x59, 0x5a, 0x41, 0x58, 0x41, 0x59, 0x41, 0x5a, 0x41, 0x5b, 0x5b, 0x5f, 0x5e, 0x41, 0x5c]); // pop 역순
    s.extend_from_slice(p.orig);
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&(fn_addr + n).to_le_bytes()); // movabs rax, fn+n
    s.extend_from_slice(&[0xff, 0xe0]);             // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; n];
    patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, n, 0x40, &mut old) == 0 { return 3; }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, n);
    VirtualProtect(fn_addr, n, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, n);
    1
}

fn selected() -> Vec<usize> {
    let Some(d) = mod_dir() else { return vec![] };
    let Ok(txt) = std::fs::read_to_string(format!(r"{}\probe060_on.txt", d)) else { return vec![] };
    let mut out = vec![];
    for line in txt.lines() {
        let line = line.trim(); if line.is_empty() || line.starts_with('#') { continue; }
        if line == "all" { return (0..N).collect(); }
        for tok in line.split(',') {
            let tok = tok.trim();
            if let Some(s) = tok.strip_prefix("name:") { out.extend(PROBES.iter().filter(|p| p.name.contains(s)).map(|p| p.idx as usize)); }
            else if let Some(s) = tok.strip_prefix("rva:") { if let Ok(v) = usize::from_str_radix(s.trim_start_matches("0x"), 16) { out.extend(PROBES.iter().filter(|p| p.rva == v).map(|p| p.idx as usize)); } }
            else if let Ok(v) = tok.parse::<usize>() { if v < N { out.push(v); } }
        }
    }
    out.sort(); out.dedup(); out
}

fn install_all() {
    if DONE.swap(1, Ordering::SeqCst) != 0 { return; }
    let _ = std::panic::catch_unwind(|| unsafe {
        let base = GetModuleHandleW(core::ptr::null()); BASE.store(base, Ordering::Relaxed);
        let sel = selected();
        if sel.is_empty() { w("[install] probe060_on.txt 없음/빈 파일 → 프로브 0 (기본 OFF)"); return; }
        let block = VirtualAlloc(0, sel.len() * 256, 0x1000 | 0x2000, 0x40);
        if block == 0 { w("[install] VirtualAlloc 실패"); return; }
        let (mut ok, mut mism, mut fail) = (0, 0, 0);
        for (k, &i) in sel.iter().enumerate() {
            let r = install(&PROBES[i], block + k * 256);
            INSTALLED[i].store(r, Ordering::Relaxed);
            match r { 1 => ok += 1, 2 => { mism += 1; w(&format!("[install] ✗ 프롤로그 불일치 idx {} {:x} {}", i, PROBES[i].rva, PROBES[i].name)); } _ => { fail += 1; w(&format!("[install] ✗ 실패 idx {} {:x} {}", i, PROBES[i].rva, PROBES[i].name)); } }
        }
        w(&format!("[install] base={:#x} 선택 {} · 설치 {} · 프롤로그 불일치 {} · 실패 {}", base, sel.len(), ok, mism, fail));
    });
}

fn snapshot(f: u64) -> Option<String> {
    let mut g = LAST.lock().unwrap_or_else(|e| e.into_inner());
    if g.len() != N { *g = vec![0; N]; }
    let mut lines = vec![];
    for i in 0..N {
        if INSTALLED[i].load(Ordering::Relaxed) != 1 { continue; }
        let h = HITS[i].load(Ordering::Relaxed);
        if h == g[i] { continue; }
        let rets: Vec<String> = (0..RET_SLOTS).map(|k| RETS[i * RET_SLOTS + k].load(Ordering::Relaxed)).filter(|&r| r != 0).map(|r| format!("{:x}", r)).collect();
        lines.push(format!("  [{}] {:x} {} hits={} (+{}) rets[{}]{}", i, PROBES[i].rva, PROBES[i].name, h, h - g[i], rets.join(" "), if RET_OVF[i].load(Ordering::Relaxed) > 0 { " +ovf" } else { "" }));
        g[i] = h;
    }
    if lines.is_empty() { return None; }
    Some(format!("[f{}]\n{}", f, lines.join("\n")))
}

struct Ext;
impl StableExtension for Ext {
    fn post_update(&self, ctx: &mut StableClient<'_>, _dt: u64) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            install_all();
            let f = FRAME.fetch_add(1, Ordering::Relaxed);
            if f % 300 != 0 { return; }
            if let Some(s) = snapshot(f) { w(&format!("{} scene={:?}", s, ctx.client_scene_kind())); }
        }));
    }
}
fn init(host: &StableHost) -> StableMod {
    let v = host.game_version();
    w(&format!("\n########## INIT game {}.{}.{} host_abi={} sdk_abi={} probes={} ##########", v.major, v.minor, v.patch, host.abi_level(), mod_api_stable::ABI_LEVEL, N));
    host.log(LogLevel::Info, "tfm2_judge_verify060");
    install_all();
    let mut d = StableMod::new("tfm2_judge_verify060");
    d.set_extension(Ext);
    d
}
declare_stable_mod!(init);
