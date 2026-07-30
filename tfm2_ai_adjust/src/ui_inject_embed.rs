//! ui_inject_embed — tfm2_ai_adjust 내장 UI 가산주입 프레임워크 (scrim 의 흡수판과 동일 설계).
//! 게임 에셋게터 트램폴린 훅 → main/strategy 템플릿 로드 시 mods/*/ui_inject.txt 조각을 가산주입.
//! scrim 도 동일 프레임워크를 내장 → 둘 중 "먼저 설치한 쪽"이 모든 모드의 매니페스트를 처리(다른 쪽은
//! prologue mismatch 로 안전 bail). 즉 scrim/ai_adjust 어느 하나만 켜도 UI 주입 동작 = 자기완결.
//! ★0.5.2 RVA: LOADER 0x5ac950 / PARSER 0x24b5a00 / ALLOC 0x25c4d30 (2026-07-22 version-migrator, 아래 상수 주석 참조).
//! ~~0.4.14 핫픽스 RVA: LOADER 0x540ad0 / PARSER 0x220e100 / ALLOC 0x231fb70~~ (0.5.x 내내 stale=미동작이었음).
#![allow(dead_code)]
use mod_api::{Node, GameUI};
use std::fs;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};

// ★★[0.5.2 마이그 완료 = 3종 전부 갱신 (version-migrator 2026-07-22)] ★★
//   ⚠구값(0x540ad0/0x220e100/0x231fb70)은 **0.4.14 기준 stale**이었음 → 0.5.x 내내 prologue mismatch로 bail =
//   ai_adjust 자체 UI 주입은 사실상 미동작(scrim이 대신 처리)이었다. 이번에 0.5.2 실주소로 복구.
//   `.ui` asset-get은 **모노모픽 copy 분화** 함정이 있어 스켈레톤 매칭 불가(0.5.2 동일해시 후보 162개) →
//   **대상 경로 문자열 xref → call 타깃** 방식으로 확정(정본 방법, MIGRATION §7.1 경고 참조).
//   실측(2026-07-22): 0.5.2에서 `"asset/base/ui/layout/main"`(len 25) lea@0x6d4fea → **call 0x5ac950**,
//     `".../strategy"`(len 29) lea@0xcdee2b·0xce3a19 → **call 0x5ac950** ⇒ 두 경로가 **동일 copy로 병합**(0.5.1은
//     main=0x40f3d0 / strategy=0xeb17d0 별개였음). 이 모듈은 단일 LOADER 훅에서 경로로 분기하므로 병합이 오히려 단순화.
//   교차확증: tfm2_item_tactics 0.5.2 마이그(타 세션)가 player_info/wide/strategy/training 4경로에 대해 동일하게 0x5ac950 도출.
//   PARSER/ALLOC = item_tactics exe2exe UNIQUE 확정값 원용 + 본 세션 재검증(0.5.1↔0.5.2 프롤로그 20B 완전동일·둘 다 .pdata 함수시작).
//   ⚠scrim/item_tactics와 **같은 0x5ac950을 후킹**하게 됨 → 먼저 설치한 쪽이 처리, 나중 쪽은 prologue mismatch로 bail(설계된 협조 동작).
const LOADER_RVA: usize = 0x2e1550;   // ★0.5.3(was 0.5.2 0x5ac950). **문자열 xref 확정**(2026-07-29, `loader_053.py`):
//   `"asset/base/ui/layout/{main,strategy,training}"` 리터럴을 lea 하는 사이트들의 직후 call 타겟 집계 = **0x2e1550 ×31 만장일치**.
//   ⚠방법 검증 = 동일 절차를 0.5.2 에 적용하면 알려진 정답 `0x5ac950`이 ×28 로 재생산된다.
//   ⛔**폐기값 `0x91ab0`**(_MIGRATE_053.md 표의 "확정" 등급 + 본 세션 1차 채택): 근거였던 "선두 12B push8 동일"은
//     0.5.3 `.text` 에 **66,635회** 등장 = 변별력 0이었다. 문자열 xref 집계에서 **0표**. install() 의 12B 프롤로그 검증도
//     같은 이유로 오후킹을 못 거른다(clone family 형제 혼동) ⟹ **UI 로더류는 프롤로그가 아니라 문자열 xref 로 잡을 것.**
//     같은 오답이 _MIGRATE_053.md 를 통해 다른 모드(banpick_illust RVA_ASSET_GET/RVA_ANIM_GET, comptest_unlock·serpen 의 LOADER)에도
//     퍼져 있으니 그쪽도 재검증 필요.
const PARSER_RVA: usize = 0x1a6530;   // ★0.5.3(was 0.5.2 0x24b5a00). 실측 동형 확인: push8 → sub rsp(0x178→**0x208**, 프레임만 확대) → lea rbp,[rsp+0x80] → `mov qword[rbp+X],-2` → **mov rsi,rcx / mov [..],rdx / mov [..],r8** 순서까지 동일 ⟹ 3인자 시그(out=rcx, text=rdx, len=r8) 불변.
// ★★[0.5.3] ALLOC 형태 변경 — 0.5.2 의 범용 `__rust_alloc(size, align)`(align 분기 있음)이 **사라졌다**(0.5.3 전역에 `cmp rdx,0x11` 소형함수 0건).
//   대신 **align 별 전용 심**이 생겼다: 예) `0xbb2bd0` = align8 전용 1인자 심(`mov r8,rcx; xor edx,edx; call 0x28f7df0`).
//   ⚠그 심은 할당 실패 시 `handle_alloc_error` → **`ud2`(abort)** 로 간다. 우리 코드는 `if new_ptr == 0 { return true; }` 로
//     **null 을 정상 처리**하므로, 심을 쓰면 그 null 체크가 무의미해지고 OOM 시 게임이 죽는다.
//   ⟹ 심이 아니라 **impl 을 직접 호출**한다(실패 시 null 반환 = 0.5.2 래퍼 경로와 동일 거동).
//   0.5.2: __rust_alloc(rcx=size, rdx=align) → align<=0x10이면 `xor edx,edx; mov r8,rcx; jmp impl` = impl(rcx무시, edx=flags, r8=size).
//   0.5.3: 래퍼 없이 **impl 을 직접 call**(전역 32,890 사이트). impl = 0.5.2 0x25d9640 과 **명령별 완전 동일**(rip 변위만 차이):
//          `mov rsi,r8; mov edi,edx; call [rip](=heap 획득); test rax; mov rcx,rax; mov edx,edi; mov r8,rsi; jmp [rip](=HeapAlloc)`.
//   ⟹ 우리는 align=8 고정이라 0.5.2에서도 항상 위 경로였다 ⟹ **impl(0, 0, size) 직접 호출이 완전 등가**.
//   그래서 상수는 impl 을 가리키고 타입도 3인자로 바꾼다(2인자로 두면 r8=size가 안 실려 쓰레기 크기로 할당).
const ALLOC_RVA: usize  = 0x28f7df0;  // ★0.5.3 = alloc **impl**(was 0.5.2 0x25c4d30 = 래퍼). 지문 `4c 89 c6 89 d7 ff 15 .. 48 85 c0 74 .. 48 89 c1 89 fa 49 89 f0` 로 0.5.3 전역 **유일** 매칭.
//   (교차확증: 같은 클러스터의 realloc 은 0.5.2 0x25c4dd0 ↔ 0.5.3 0x28e3b10 이 크기 174B·명령열 완전 동일로 확정됨.)
const TARGET: &[u8]  = b"asset/base/ui/layout/main";
const TARGET2: &[u8] = b"asset/base/ui/layout/strategy";   // 전술화면(#row0~#row4)
const NT_SIZE: usize = 0x90;
const BG_BLOCK: &str = "top";

type LoaderFn = extern "win64" fn(usize, *const u8, usize) -> usize;
type ParserFn = extern "win64" fn(*mut u8, *const u8, usize);
type AllocFn  = extern "win64" fn(usize, u32, usize) -> usize;   // ★0.5.3: alloc impl 직접호출 (rcx=미사용, edx=flags, r8=size). 구 래퍼 시그 fn(size, align)에서 변경 — ALLOC_RVA 주석 참조.

static BASE: AtomicUsize = AtomicUsize::new(0);
static MAIN_TMPL: AtomicUsize = AtomicUsize::new(0);
static LAST_INJECTED: AtomicUsize = AtomicUsize::new(0);
static INJECT_ATTEMPTS: AtomicUsize = AtomicUsize::new(0);
static STRAT_TMPL: AtomicUsize = AtomicUsize::new(0);
static LAST_INJ_STRAT: AtomicUsize = AtomicUsize::new(0);
static INJ_ATT_STRAT: AtomicUsize = AtomicUsize::new(0);
static TRAMP: AtomicUsize = AtomicUsize::new(0);
static INSTALLED: AtomicBool = AtomicBool::new(false);
static MAIN_TID: AtomicU32 = AtomicU32::new(0);
static INJECTING: AtomicBool = AtomicBool::new(false);
static MODAL_IDS: Mutex<Vec<String>> = Mutex::new(Vec::new());

// ── 디버그 로깅 ──
pub static DBG: AtomicBool = AtomicBool::new(false);
pub static LOG_DIR: Mutex<String> = Mutex::new(String::new());
fn logln(s: &str) {
    if !DBG.load(Ordering::Relaxed) { return; }
    let d = LOG_DIR.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if d.is_empty() { return; }
    use std::io::Write;
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(format!("{}\\uinj_log.txt", d)) {
        let _ = f.write_all(s.as_bytes()); let _ = f.write_all(b"\n");
    }
}

type BOOL = i32;
#[link(name = "kernel32")]
extern "system" {
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old_protect: *mut u32) -> BOOL;
    fn FlushInstructionCache(proc: usize, addr: usize, size: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
    fn GetCurrentThreadId() -> u32;
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleFileNameW(module: usize, buf: *mut u16, size: u32) -> u32;
}
const MEM_COMMIT_RESERVE: u32 = 0x1000 | 0x2000;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;

fn nfind<'a>(n: &'a Node, id: &str) -> Option<&'a Node> {
    if n.id == id { return Some(n); }
    for c in n.child.iter() { if let Some(f) = nfind(c, id) { return Some(f); } }
    None
}
fn nfind_mut<'a>(n: &'a mut Node, id: &str) -> Option<&'a mut Node> {
    if n.id == id { return Some(n); }
    for c in n.child.iter_mut() { if let Some(f) = nfind_mut(c, id) { return Some(f); } }
    None
}
fn root_id_of(text: &[u8]) -> Option<String> {
    let s = std::str::from_utf8(text).ok()?;
    for line in s.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') || t.starts_with("//") { continue; }
        let id: String = t.chars().take_while(|&c| c != ':' && c != ' ' && c != '\t' && c != '{').collect();
        if !id.is_empty() { return Some(id); }
    }
    None
}
fn game_mods_dir() -> String {
    let mut buf = [0u16; 520];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n > 0 && n < buf.len() {
        let exe = String::from_utf16_lossy(&buf[..n]);
        if let Some(i) = exe.rfind(|c| c == '\\' || c == '/') { return format!(r"{}\mods", &exe[..i]); }
    }
    String::new()
}
fn module_loaded(dll_name: &str) -> bool {
    let w: Vec<u16> = dll_name.encode_utf16().chain(core::iter::once(0)).collect();
    unsafe { GetModuleHandleW(w.as_ptr()) != 0 }
}

extern "win64" fn detour(am: usize, path: *const u8, len: usize) -> usize {
    let tramp_addr = TRAMP.load(Ordering::Relaxed);
    if tramp_addr == 0 { return 0; }
    let tramp: LoaderFn = unsafe { core::mem::transmute(tramp_addr) };
    let r = tramp(am, path, len);
    if !path.is_null() && r > 0x10000 && len < 200 {
        let s = unsafe { core::slice::from_raw_parts(path, len) };
        let mt = MAIN_TID.load(Ordering::Relaxed);
        let on_main = mt == 0 || unsafe { GetCurrentThreadId() } == mt;
        if len == TARGET.len() && s == TARGET {
            MAIN_TMPL.store(r, Ordering::Relaxed);
            if on_main && r != LAST_INJECTED.load(Ordering::Relaxed) {
                let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { do_inject(r) })).unwrap_or(false);
                if ok { LAST_INJECTED.store(r, Ordering::Relaxed); INJECT_ATTEMPTS.store(0, Ordering::Relaxed); }
            }
        } else if len == TARGET2.len() && s == TARGET2 {
            STRAT_TMPL.store(r, Ordering::Relaxed);
            if on_main && r != LAST_INJ_STRAT.load(Ordering::Relaxed) {
                let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { do_inject(r) })).unwrap_or(false);
                if ok { LAST_INJ_STRAT.store(r, Ordering::Relaxed); INJ_ATT_STRAT.store(0, Ordering::Relaxed); }
            }
        }
    }
    r
}

pub unsafe fn install() -> Result<(), &'static str> {
    if INSTALLED.swap(true, Ordering::Relaxed) { return Err("already"); }
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { return Err("module base 0"); }
    BASE.store(base, Ordering::Relaxed);
    let fn_addr = base + LOADER_RVA;
    let expect: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
    for i in 0..12 { if *((fn_addr + i) as *const u8) != expect[i] {
        logln(&format!("install bail: prologue @+{} = {:#x}(want {:#x}) — 이미 다른 모드(scrim등)가 후킹함, 그쪽이 주입", i, *((fn_addr + i) as *const u8), expect[i]));
        INSTALLED.store(false, Ordering::Relaxed); return Err("prologue mismatch (already hooked?)");
    } }
    // ★크래시 시 module=unknown 역해석용 등록(detour.rs 스텁 인벤토리). 여긴 별도 모듈이라 crate:: 경로로 부른다.
    let stub = crate::stub_reg(VirtualAlloc(0, 64, MEM_COMMIT_RESERVE, PAGE_EXECUTE_READWRITE), 64, LOADER_RVA);
    if stub == 0 { return Err("VirtualAlloc"); }
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&expect);
    s.extend_from_slice(&[0x48, 0xb8]);
    s.extend_from_slice(&(fn_addr + 0xc).to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    TRAMP.store(stub, Ordering::Relaxed);
    let d = detour as usize;
    let mut patch = [0u8; 12];
    patch[0] = 0x48; patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&d.to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, PAGE_EXECUTE_READWRITE, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    logln(&format!("install OK: hooked asset-getter fn={:#x}", fn_addr));
    Ok(())
}

unsafe fn find_tmpl(node: usize, target: &[u8], depth: usize) -> usize {
    if node <= 0x10000 || depth > 12 { return 0; }
    let idptr = *((node + 0x08) as *const usize);
    let idlen = *((node + 0x10) as *const usize);
    if idlen == target.len() && idptr > 0x10000 {
        if core::slice::from_raw_parts(idptr as *const u8, idlen) == target { return node; }
    }
    let cptr = *((node + 0x50) as *const usize);
    let clen = *((node + 0x58) as *const usize);
    if cptr > 0x10000 && clen < 1000 {
        for i in 0..clen {
            let found = find_tmpl(cptr + i * NT_SIZE, target, depth + 1);
            if found != 0 { return found; }
        }
    }
    0
}
unsafe fn child_index(container: usize, id: &[u8]) -> Option<usize> {
    let cptr = *((container + 0x50) as *const usize);
    let clen = *((container + 0x58) as *const usize);
    if cptr <= 0x10000 || clen > 2000 { return None; }
    for i in 0..clen {
        let c = cptr + i * NT_SIZE;
        let idptr = *((c + 0x08) as *const usize);
        let idlen = *((c + 0x10) as *const usize);
        if idlen == id.len() && idptr > 0x10000 && core::slice::from_raw_parts(idptr as *const u8, idlen) == id {
            return Some(i);
        }
    }
    None
}
// mods/*/ui_inject.txt 스캔: "<ui상대경로> <타깃id> <위치> [modal]".
fn collect_fragments() -> Vec<(String, String, String, bool)> {
    let mut out = Vec::new();
    let md = game_mods_dir();
    if md.is_empty() { return out; }
    let rd = match fs::read_dir(&md) { Ok(r) => r, Err(_) => return out };
    for ent in rd.flatten() {
        let dir = ent.path();
        let txt = match fs::read_to_string(dir.join("ui_inject.txt")) { Ok(t) => t, Err(_) => continue };
        let mod_id = dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        if mod_id != "tfm2_ai_adjust" && !module_loaded(&format!("{}.dll", mod_id)) { continue; }
        for line in txt.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            let p: Vec<&str> = line.split_whitespace().collect();
            if p.len() >= 2 {
                let ui = dir.join(p[0]).to_string_lossy().into_owned();
                let is_modal = p.iter().skip(2).any(|t| *t == "modal");
                let pos = match p.get(2) { Some(&t) if t != "modal" => t.to_string(), _ => "end".to_string() };
                out.push((ui, p[1].to_string(), pos, is_modal));
            }
        }
    }
    out
}
unsafe fn do_inject(r: usize) -> bool {
    if INJECTING.swap(true, Ordering::Acquire) { return false; }
    struct Guard; impl Drop for Guard { fn drop(&mut self) { INJECTING.store(false, Ordering::Release); } }
    let _g = Guard;
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 || r <= 0x10000 { return false; }
    MODAL_IDS.lock().unwrap_or_else(|e| e.into_inner()).clear();
    let frags = collect_fragments();
    let mut all_ok = true;
    for (ui, target, pos, modal) in &frags {
        if !inject_one(r, base, ui, target, pos, *modal) { all_ok = false; }
    }
    all_ok
}
unsafe fn inject_one(r: usize, base: usize, ui_path: &str, target_id: &str, pos: &str, is_modal: bool) -> bool {
    let text = match fs::read(ui_path) { Ok(t) => t, Err(_) => return true };
    if text.is_empty() { return true; }
    if let Some(rid) = root_id_of(&text) { if find_tmpl(r, rid.as_bytes(), 0) != 0 { return true; } }
    let parser: ParserFn = core::mem::transmute(base + PARSER_RVA);
    let mut out = [0u8; 0x400];
    parser(out.as_mut_ptr(), text.as_ptr(), text.len());
    let f = out.as_ptr().add(0x10);
    if *(f as *const usize) == usize::MAX { logln(&format!("parse ERR '{}'", target_id)); return true; }
    if is_modal {
        if let Some(rid) = root_id_of(&text) {
            let mut ids = MODAL_IDS.lock().unwrap_or_else(|e| e.into_inner());
            if !ids.contains(&rid) { ids.push(rid); }
        }
    }
    let target = if target_id == "root" { r } else {
        let t = find_tmpl(r, target_id.as_bytes(), 0);
        if t != 0 { t } else { return false; }
    };
    let cap_p = (target + 0x48) as *mut usize;
    let ptr_p = (target + 0x50) as *mut usize;
    let len_p = (target + 0x58) as *mut usize;
    let old_ptr = *ptr_p; let len = *len_p;
    if len > 2000 { return true; }
    let idx = if let Some(a) = pos.strip_prefix("after:") {
        child_index(target, a.as_bytes()).map(|i| i + 1).unwrap_or(len)
    } else if let Some(b) = pos.strip_prefix("before:") {
        child_index(target, b.as_bytes()).unwrap_or(len)
    } else if pos == "end" { len } else { pos.parse::<usize>().unwrap_or(len) }.min(len);
    let galloc: AllocFn = core::mem::transmute(base + ALLOC_RVA);
    let new_n = len + 1;
    let new_ptr = galloc(0, 0, new_n * NT_SIZE);   // ★0.5.3 impl 규약: (미사용, flags=0, size). 구 래퍼 규약 (size, align=8)과 등가.
    if new_ptr == 0 { return true; }
    if old_ptr != 0 && idx != 0 {
        core::ptr::copy_nonoverlapping(old_ptr as *const u8, new_ptr as *mut u8, idx * NT_SIZE);
    }
    core::ptr::copy_nonoverlapping(f, (new_ptr + idx * NT_SIZE) as *mut u8, NT_SIZE);
    if old_ptr != 0 && idx < len {
        core::ptr::copy_nonoverlapping((old_ptr + idx * NT_SIZE) as *const u8, (new_ptr + (idx + 1) * NT_SIZE) as *mut u8, (len - idx) * NT_SIZE);
    }
    *ptr_p = new_ptr; *cap_p = new_n; *len_p = new_n;
    logln(&format!("OK inject '{}' idx{} (child {}→{})", target_id, idx, len, new_n));
    true
}
// post_update: main/strategy 재시도 주입 + 모달 배경차단. (scrim 이 이미 후킹했으면 이쪽 MAIN/STRAT_TMPL=0 이라 무동작.)
pub fn tick(u: &mut GameUI) {
    if MAIN_TID.load(Ordering::Relaxed) == 0 { MAIN_TID.store(unsafe { GetCurrentThreadId() }, Ordering::Relaxed); }
    let r = MAIN_TMPL.load(Ordering::Relaxed);
    if r > 0x10000 && r != LAST_INJECTED.load(Ordering::Relaxed) {
        let n = INJECT_ATTEMPTS.fetch_add(1, Ordering::Relaxed);
        if unsafe { do_inject(r) } || n > 120 { LAST_INJECTED.store(r, Ordering::Relaxed); INJECT_ATTEMPTS.store(0, Ordering::Relaxed); }
    }
    let rs = STRAT_TMPL.load(Ordering::Relaxed);
    if rs > 0x10000 && rs != LAST_INJ_STRAT.load(Ordering::Relaxed) {
        let n = INJ_ATT_STRAT.fetch_add(1, Ordering::Relaxed);
        if unsafe { do_inject(rs) } || n > 120 { LAST_INJ_STRAT.store(rs, Ordering::Relaxed); INJ_ATT_STRAT.store(0, Ordering::Relaxed); }
    }
    let ids = MODAL_IDS.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if !ids.is_empty() {
        let any = ids.iter().any(|id| nfind(&u.root, id).map(|n| n.visible).unwrap_or(false));
        if let Some(top) = nfind_mut(&mut u.root, BG_BLOCK) { top.disabled = any; }
    }
}
