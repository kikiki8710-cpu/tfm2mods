//! tfm2_ver_probe — 0.6.0 judge 재명세용 1회성 진단 모드(stable 껍데기 + 네이티브 캡처 detour).
//! 목적(r21_pending.md §B): ①`version` 값(TeamPlan pre-update f0f080 의 rdx = agent+0x3608 유래)
//! ②ObjContest 씬 발생 여부(tp+0x3e4/+0x404 take_born 니치 ≠2) · armed +0xcc7/+0xcc2
//! ③check_kill_die_tick eda920 의 10번째 인자 `&Option<(x,y,t)>` 가 Some 인 콜러(리턴 주소 RVA).
//! 출력 = mods\tfm2_ver_probe\ver_probe.txt (5초마다 카운터가 바뀌면 한 줄). 클래식 SDK 없이 stable 로더로 적재.
use mod_api_stable::{declare_stable_mod, LogLevel, StableClient, StableExtension, StableHost, StableMod};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleFileNameW(module: usize, buf: *mut u16, size: u32) -> u32;
    fn VirtualAlloc(addr: usize, size: usize, ty: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, prot: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(h: usize, addr: usize, size: usize) -> i32;
    fn GetCurrentProcess() -> usize;
}

// ── 0.6.0 RVA(RE r21 · 진입부 12B = push 8개 · 위치독립) ──
const RVA_PRE_UPDATE: usize = 0xf0f080; // TeamPlan pre-update(rcx=tp, rdx=version) · +0xcc7 세팅처
const PRO_PRE: [u8; 12] = [0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53];
const RVA_CKDT: usize = 0xeda920; // check_kill_die_tick(version, data, judger, focus, &enemy, &towers, simple, ignore_nuke, no_noise, &Option<(x,y,t)>)
const PRO_CK: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];

fn out_path() -> Option<String> {
    let mut buf = [0u16; 520];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n == 0 || n >= buf.len() { return None; }
    let exe = String::from_utf16_lossy(&buf[..n]);
    exe.rfind(|c| c == '\\' || c == '/').map(|i| format!(r"{}\mods\tfm2_ver_probe\ver_probe.txt", &exe[..i]))
}
fn w(s: &str) {
    if let Some(p) = out_path() {
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(p) { let _ = writeln!(f, "{}", s); }
    }
}

// ── 카운터(캡처 안에서는 원자/락만 · format! 금지) ──
const NV: usize = 16;
static PRE_HITS: AtomicU64 = AtomicU64::new(0);
static PRE_VER: [AtomicU64; NV] = [const { AtomicU64::new(0) }; NV];
static PRE_VER_OTHER: AtomicU64 = AtomicU64::new(u64::MAX);
static CC7_ON: AtomicU64 = AtomicU64::new(0);
static CC2_ON: AtomicU64 = AtomicU64::new(0);
static CC0_ON: AtomicU64 = AtomicU64::new(0);
static SERPEN_SCENE: AtomicU64 = AtomicU64::new(0);
static MORGARD_SCENE: AtomicU64 = AtomicU64::new(0);
static SERPEN_PHASE: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8];
static MORGARD_PHASE: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8];
static OBJ_LEGACY: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8]; // tp+0xcd5 레거시 objective 태그
static CK_HITS: AtomicU64 = AtomicU64::new(0);
static CK_SOME: AtomicU64 = AtomicU64::new(0);
static CK_VER: [AtomicU64; NV] = [const { AtomicU64::new(0) }; NV];
static CK_BAD: AtomicU64 = AtomicU64::new(0);
static CK_SOME_RET: Mutex<BTreeMap<usize, u64>> = Mutex::new(BTreeMap::new());
static CK_NONE_RET: Mutex<BTreeMap<usize, u64>> = Mutex::new(BTreeMap::new());
static CK_SOME_SAMPLE: Mutex<Vec<(i64, i64, i64)>> = Mutex::new(Vec::new());
static BASE: AtomicUsize = AtomicUsize::new(0);
static INSTALLED: AtomicUsize = AtomicUsize::new(0);
static FRAME: AtomicU64 = AtomicU64::new(0);
static LAST: Mutex<String> = Mutex::new(String::new());

#[inline] fn ptr_ok(p: usize) -> bool { p >= 0x10000 && p < (1usize << 47) }
#[inline] fn bump(h: &[AtomicU64; NV], v: u64) { if (v as usize) < NV - 1 { h[v as usize].fetch_add(1, Ordering::Relaxed); } else { h[NV - 1].fetch_add(1, Ordering::Relaxed); } }

// saved 레이아웃(스텁 push 순서 r12 rsi rdi rbx r11 r10 r9 r8 rdx rcx): +0 rcx +8 rdx +0x10 r8 +0x18 r9 … +0x48 r12 · 원본 rsp = saved+0x50
unsafe extern "C" fn cap_pre(saved: *const usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let tp = *saved; let ver = *saved.add(1) as u64;
        PRE_HITS.fetch_add(1, Ordering::Relaxed);
        bump(&PRE_VER, ver); if ver as usize >= NV - 1 { PRE_VER_OTHER.store(ver, Ordering::Relaxed); }
        if !ptr_ok(tp) { return; }
        let b = |off: usize| -> u8 { *((tp + off) as *const u8) };
        if b(0xcc7) != 0 { CC7_ON.fetch_add(1, Ordering::Relaxed); }
        if b(0xcc2) != 0 { CC2_ON.fetch_add(1, Ordering::Relaxed); }
        if b(0xcc0) != 0 { CC0_ON.fetch_add(1, Ordering::Relaxed); }
        let sp = b(0x3e4); let mp = b(0x404);
        if sp != 2 { SERPEN_SCENE.fetch_add(1, Ordering::Relaxed); let ph = b(0x3e0) as usize; SERPEN_PHASE[ph.min(7)].fetch_add(1, Ordering::Relaxed); }
        if mp != 2 { MORGARD_SCENE.fetch_add(1, Ordering::Relaxed); let ph = b(0x400) as usize; MORGARD_PHASE[ph.min(7)].fetch_add(1, Ordering::Relaxed); }
        let ob = b(0xcd5) as usize; OBJ_LEGACY[ob.min(7)].fetch_add(1, Ordering::Relaxed);
    }));
}
unsafe extern "C" fn cap_ck(saved: *const usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let ver = *saved as u64;
        CK_HITS.fetch_add(1, Ordering::Relaxed); bump(&CK_VER, ver);
        let orig_rsp = saved as usize + 0x50;
        let ret = *(orig_rsp as *const usize);
        let base = BASE.load(Ordering::Relaxed);
        let rva = ret.wrapping_sub(base);
        let opt = *((orig_rsp + 0x50) as *const usize); // 10번째 인자
        if !ptr_ok(opt) { CK_BAD.fetch_add(1, Ordering::Relaxed); return; }
        let tag = *(opt as *const u64);
        if tag == 1 {
            CK_SOME.fetch_add(1, Ordering::Relaxed);
            let mut g = CK_SOME_RET.lock().unwrap_or_else(|e| e.into_inner());
            if g.len() < 64 || g.contains_key(&rva) { *g.entry(rva).or_insert(0) += 1; }
            let mut s = CK_SOME_SAMPLE.lock().unwrap_or_else(|e| e.into_inner());
            if s.len() < 8 { s.push((*((opt + 8) as *const i64), *((opt + 16) as *const i64), *((opt + 24) as *const i64))); }
        } else if tag == 0 {
            let mut g = CK_NONE_RET.lock().unwrap_or_else(|e| e.into_inner());
            if g.len() < 64 || g.contains_key(&rva) { *g.entry(rva).or_insert(0) += 1; }
        } else { CK_BAD.fetch_add(1, Ordering::Relaxed); }
    }));
}

/// 캡처 detour(tfm2_item_tactics install_detour_generic 축약판 · 체인 없음): 진입부 12B 를 `movabs rax,stub; jmp rax` 로 덮고
/// 스텁 = push 10 regs → cap_fn(saved) → pop → 원본 12B → jmp fn+12.
unsafe fn install_capture(rva: usize, prologue: &[u8; 12], cap_fn: usize) -> Result<usize, &'static str> {
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { return Err("module 0"); }
    BASE.store(base, Ordering::Relaxed);
    let fn_addr = base + rva;
    for i in 0..12 { if *((fn_addr + i) as *const u8) != prologue[i] { return Err("prologue mismatch"); } }
    let stub = VirtualAlloc(0, 256, 0x1000 | 0x2000, 0x40);
    if stub == 0 { return Err("VirtualAlloc"); }
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x41, 0x54, 0x56, 0x57, 0x53, 0x41, 0x53, 0x41, 0x52, 0x41, 0x51, 0x41, 0x50, 0x52, 0x51]); // push r12 rsi rdi rbx r11 r10 r9 r8 rdx rcx
    s.extend_from_slice(&[0x48, 0x89, 0xe1]);       // mov rcx, rsp
    s.extend_from_slice(&[0x48, 0x89, 0xe3]);       // mov rbx, rsp
    s.extend_from_slice(&[0x48, 0x83, 0xe4, 0xf0]); // and rsp, -16
    s.extend_from_slice(&[0x48, 0x83, 0xec, 0x20]); // sub rsp, 0x20
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes()); // movabs rax, cap_fn
    s.extend_from_slice(&[0xff, 0xd0]);             // call rax
    s.extend_from_slice(&[0x48, 0x89, 0xdc]);       // mov rsp, rbx
    s.extend_from_slice(&[0x59, 0x5a, 0x41, 0x58, 0x41, 0x59, 0x41, 0x5a, 0x41, 0x5b, 0x5b, 0x5f, 0x5e, 0x41, 0x5c]); // pop 역순
    s.extend_from_slice(prologue);                  // 원본 12B(push 들)
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&(fn_addr + 12).to_le_bytes()); // movabs rax, fn+12
    s.extend_from_slice(&[0xff, 0xe0]);             // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = [0x90u8; 12];
    patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, 0x40, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    Ok(stub)
}

fn install_all() {
    if INSTALLED.swap(1, Ordering::SeqCst) != 0 { return; }
    let _ = std::panic::catch_unwind(|| unsafe {
        let r1 = install_capture(RVA_PRE_UPDATE, &PRO_PRE, cap_pre as usize);
        let r2 = install_capture(RVA_CKDT, &PRO_CK, cap_ck as usize);
        w(&format!("[install] base={:#x} pre_update f0f080 → {:?} · ckdt eda920 → {:?}", BASE.load(Ordering::Relaxed), r1.map(|s| format!("{:#x}", s)), r2.map(|s| format!("{:#x}", s))));
    });
}

fn snapshot() -> String {
    let h = |a: &[AtomicU64; NV]| -> String {
        let mut v: Vec<String> = Vec::new();
        for (i, c) in a.iter().enumerate() { let n = c.load(Ordering::Relaxed); if n > 0 { v.push(if i == NV - 1 { format!("other({})={}", PRE_VER_OTHER.load(Ordering::Relaxed), n) } else { format!("{}={}", i, n) }); } }
        v.join(",")
    };
    let ph = |a: &[AtomicU64; 8]| -> String { a.iter().enumerate().filter(|(_, c)| c.load(Ordering::Relaxed) > 0).map(|(i, c)| format!("{}={}", i, c.load(Ordering::Relaxed))).collect::<Vec<_>>().join(",") };
    let rets = |m: &Mutex<BTreeMap<usize, u64>>| -> String { let g = m.lock().unwrap_or_else(|e| e.into_inner()); g.iter().map(|(k, v)| format!("{:x}:{}", k, v)).collect::<Vec<_>>().join(" ") };
    let sample = { let s = CK_SOME_SAMPLE.lock().unwrap_or_else(|e| e.into_inner()); format!("{:?}", *s) };
    format!("pre_update hits={} version[{}] cc0_on={} cc2_on={} cc7_on={} | serpen_scene={} phase[{}] morgard_scene={} phase[{}] legacy_obj[{}] | ckdt hits={} version[{}] some={} bad={} some_ret[{}] none_ret[{}] some_sample={}",
        PRE_HITS.load(Ordering::Relaxed), h(&PRE_VER), CC0_ON.load(Ordering::Relaxed), CC2_ON.load(Ordering::Relaxed), CC7_ON.load(Ordering::Relaxed),
        SERPEN_SCENE.load(Ordering::Relaxed), ph(&SERPEN_PHASE), MORGARD_SCENE.load(Ordering::Relaxed), ph(&MORGARD_PHASE), ph(&OBJ_LEGACY),
        CK_HITS.load(Ordering::Relaxed), h(&CK_VER), CK_SOME.load(Ordering::Relaxed), CK_BAD.load(Ordering::Relaxed), rets(&CK_SOME_RET), rets(&CK_NONE_RET), sample)
}

struct Ext;
impl StableExtension for Ext {
    fn post_update(&self, ctx: &mut StableClient<'_>, _dt: u64) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            install_all();
            let f = FRAME.fetch_add(1, Ordering::Relaxed);
            if f % 300 != 0 { return; }
            let s = snapshot();
            let mut g = LAST.lock().unwrap_or_else(|e| e.into_inner());
            if *g == s { return; }
            *g = s.clone();
            w(&format!("[f{} scene={:?}] {}", f, ctx.client_scene_kind(), s));
        }));
    }
}
fn init(host: &StableHost) -> StableMod {
    let v = host.game_version();
    w(&format!("\n########## INIT game {}.{}.{} host_abi={} sdk_abi={} ##########", v.major, v.minor, v.patch, host.abi_level(), mod_api_stable::ABI_LEVEL));
    host.log(LogLevel::Info, "tfm2_ver_probe");
    install_all();
    let mut d = StableMod::new("tfm2_ver_probe");
    d.set_extension(Ext);
    d
}
declare_stable_mod!(init);
