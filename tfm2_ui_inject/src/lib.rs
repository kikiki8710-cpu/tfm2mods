//! tfm2_ui_inject — 범용 UI 가산-주입 프레임워크 (네이티브 후킹).
//! ───────────────────────────────────────────────────────────────────────────
//! 게임의 에셋 게터 FUN_140587640(asset_mgr, path_ptr, path_len) → NodeTemplate ptr 를
//! 트램펄린 detour 로 가로채, 지정 경로(예 main) 로드 시 내 .ui 조각의 자식들을 그 템플릿
//! child Vec(@0x48) 에 안전한 Rust 로 push(clone). 게임이 인스턴스화 → override 없이 가산.
//!
//! Phase 1 (이 빌드): 훅 설치 + detour 가 ①발화 ②path 읽기 ③트램펄린 원본호출
//!   ④내 조각이 game asset_mgr 에 로드돼 있나(FUN_140587640 으로 조회) 만 로그. append 는 Phase 2.
#![allow(dead_code, unused_imports)]
use mod_api::*;
use std::fs;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const LOG: &str = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_ui_inject\inject_log.txt";
// ★0.4.13-hotfix(06-17) 마이그레이션: 구RVA→신RVA. 구exe 바디 마스크시그(rip-rel 와일드)+문자열xref 로 확정.
// 핫픽스4(06-18 21:18, exe 65,778,176): asset게터 0x61d4a0→0x7cc820(크게 이동, string-xref 17→0x7cc820, 프롤로그 8-PUSH 바이트동일), 파서/alloc/dealloc 이동.
const LOADER_RVA: usize = 0x540ad0;  // 에셋 게터 — "asset/base/ui/layout/main" 17개 LEA→call 전부 이걸 호출(hotfix3=0x61d4a0)
const PARSER_RVA: usize = 0x220e100; // (hotfix3=0x21eb2b0) FUN(out, text_ptr, text_len) — .ui 텍스트→NodeTemplate
const ALLOC_RVA: usize = 0x231fb70;  // (hotfix3=0x22faeb0) alloc(size, align) -> ptr
const DEALLOC_RVA: usize = 0x231fbd0;// (hotfix3=0x22faf10) dealloc(ptr, size, align) — 현재 미사용(leak설계)
type AllocFn = extern "win64" fn(usize, usize) -> usize;
type DeallocFn = extern "win64" fn(usize, usize, usize);
const TARGET: &[u8] = b"asset/base/ui/layout/main";
const MODS_DIR: &str = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods";
static BASE: AtomicUsize = AtomicUsize::new(0);
static MAIN_TMPL: AtomicUsize = AtomicUsize::new(0);
static LAST_INJECTED: AtomicUsize = AtomicUsize::new(0); // 마지막으로 성공 주입한 템플릿 ptr (reload 감지용)
static INJECT_ATTEMPTS: AtomicUsize = AtomicUsize::new(0); // 현재 템플릿 주입 재시도 횟수(캡=무한IO방지)
// 진단: 훅 발화/경로매칭 카운터
static DETOUR_HITS: AtomicUsize = AtomicUsize::new(0);
static LEN25_HITS: AtomicUsize = AtomicUsize::new(0);
static MAIN_HITS: AtomicUsize = AtomicUsize::new(0);
static DIAG_FRAME: AtomicUsize = AtomicUsize::new(0);
static MODAL_IDS: Mutex<Vec<String>> = Mutex::new(Vec::new()); // modal 플래그 조각의 루트 id (visible 감시→배경차단)
const BG_BLOCK: &str = "top"; // 모달 열렸을 때 입력/호버 차단할 배경 컨테이너 (사이드바+콘텐츠)
type ParserFn = extern "win64" fn(*mut u8, *const u8, usize);

// 라이브 Node 트리에서 id 로 찾기 (모달 가시성/배경 차단용; mod_api Node = 라이브 호환).
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
// 조각 .ui 텍스트의 루트 노드 id (첫 토큰, ':' 앞).
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

const LOG_ENABLED: bool = false; // ★ 배포=false: inject_log.txt 출력 끔. 디버그시 true.
fn logln(s: &str) {
    if !LOG_ENABLED { return; }
    use std::io::Write;
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(LOG) { let _ = f.write_all(s.as_bytes()); let _ = f.write_all(b"\n"); }
}

// ───────────── WinAPI ─────────────
type BOOL = i32;
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> isize;
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old_protect: *mut u32) -> BOOL;
    fn FlushInstructionCache(proc: usize, addr: usize, size: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
}
const MEM_COMMIT_RESERVE: u32 = 0x1000 | 0x2000;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;

// 트램펄린(원본 호출용) 포인터. detour 가 이걸 호출.
static TRAMP: AtomicUsize = AtomicUsize::new(0);
static INSTALLED: AtomicBool = AtomicBool::new(false);
static LOGGED: AtomicBool = AtomicBool::new(false);

type LoaderFn = extern "win64" fn(usize, *const u8, usize) -> usize;

// detour: 원본 시그니처와 동일. 원본 entry 가 여기로 jmp(args 레지스터 그대로).
extern "win64" fn detour(am: usize, path: *const u8, len: usize) -> usize {
    let tramp_addr = TRAMP.load(Ordering::Relaxed);
    if tramp_addr == 0 { return 0; }
    let tramp: LoaderFn = unsafe { core::mem::transmute(tramp_addr) };
    let r = tramp(am, path, len); // 원본 호출 → 템플릿 ptr

    DETOUR_HITS.fetch_add(1, Ordering::Relaxed);
    // 타깃(main) 로드 감지
    if !path.is_null() && len == TARGET.len() {
        LEN25_HITS.fetch_add(1, Ordering::Relaxed);
        let s = unsafe { core::slice::from_raw_parts(path, len) };
        if s == TARGET && r > 0x10000 {
            MAIN_HITS.fetch_add(1, Ordering::Relaxed);
            MAIN_TMPL.store(r, Ordering::Relaxed); // 매 로드마다 갱신 (reload→새 ptr 감지)
            // ★ 첫화면 표시: 템플릿을 게임이 인스턴스화하기 "전"(여기, getter 반환 직전)에 주입 →
            //   게임이 주입된 템플릿을 그려서 첫 진입부터 버튼 보임. (post_update 주입은 인스턴스화 후라 첫화면 누락.)
            //   catch_unwind: 우리 코드 패닉이 게임 콜스택(extern win64)으로 unwind=UB 차단. 멱등이라 중복X.
            if r != LAST_INJECTED.load(Ordering::Relaxed) {
                let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { do_inject(r) })).unwrap_or(false);
                if ok { LAST_INJECTED.store(r, Ordering::Relaxed); INJECT_ATTEMPTS.store(0, Ordering::Relaxed); }
            }
            if !LOGGED.swap(true, Ordering::Relaxed) {
                logln(&format!("MAIN loaded: template={:#x} (detour 주입)", r));
            }
        }
    }
    r
}

unsafe fn install() -> Result<(), &'static str> {
    if INSTALLED.swap(true, Ordering::Relaxed) { return Err("already"); }
    let base = GetModuleHandleW(core::ptr::null()) as usize;
    if base == 0 { return Err("module base 0"); }
    BASE.store(base, Ordering::Relaxed);
    let fn_addr = base + LOADER_RVA;
    // 프롤로그 검증: 8 PUSH = 12B
    let expect: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
    for i in 0..12 { if *((fn_addr + i) as *const u8) != expect[i] { logln(&format!("prologue mismatch @+{}: {:#x}", i, *((fn_addr + i) as *const u8))); return Err("prologue mismatch"); } }

    // 트램펄린 stub (RWX): 원본 12B + jmp (fn_addr+0xc)
    let stub = VirtualAlloc(0, 64, MEM_COMMIT_RESERVE, PAGE_EXECUTE_READWRITE);
    if stub == 0 { return Err("VirtualAlloc"); }
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&expect);                       // 훔친 8 PUSH
    s.extend_from_slice(&[0x48, 0xb8]);                 // mov rax,
    s.extend_from_slice(&(fn_addr + 0xc).to_le_bytes()); //   fn+0xc
    s.extend_from_slice(&[0xff, 0xe0]);                 // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    TRAMP.store(stub, Ordering::Relaxed);

    // entry 12B → mov rax, detour ; jmp rax
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
    logln(&format!("hook installed: fn={:#x} stub={:#x} detour={:#x}", fn_addr, stub, d));
    Ok(())
}

static FRAME: AtomicUsize = AtomicUsize::new(0);
static PARSED: AtomicBool = AtomicBool::new(false);

const NT_SIZE: usize = 0x90; // NodeTemplate 크기

// 템플릿 트리에서 id 로 노드 찾기. id: ptr@+0x08, len@+0x10. child: ptr@+0x50, count@+0x58, elem 0x90.
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

// 모든 모드 폴더의 ui_inject.txt 매니페스트 수집.
// 각 줄: "<ui상대경로> <타깃id> <위치> [modal]" (# 주석).
//   위치 = 숫자 | after:<id> | before:<id> | end(기본). 4번째 토큰 "modal" = 이 조각의 루트가
//   보일 때 배경(top) 입력차단(호버누수 방지) 대상.
// 모드 dll 이 실제 프로세스에 로드됐는지(=진짜 활성). enabled_mods(파일)는 부정확해서 못 씀.
fn module_loaded(dll_name: &str) -> bool {
    let w: Vec<u16> = dll_name.encode_utf16().chain(core::iter::once(0)).collect();
    unsafe { GetModuleHandleW(w.as_ptr()) != 0 }
}
fn collect_fragments() -> Vec<(String, String, String, bool)> {
    let mut out = Vec::new();
    let rd = match fs::read_dir(MODS_DIR) { Ok(r) => r, Err(_) => return out };
    for ent in rd.flatten() {
        let dir = ent.path();
        let txt = match fs::read_to_string(dir.join("ui_inject.txt")) { Ok(t) => t, Err(_) => continue };
        // ★ 그 모드의 dll 이 로드돼 있을 때만 주입 (안 로드면 inert 버튼 방지)
        let mod_id = dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        if !module_loaded(&format!("{}.dll", mod_id)) {
            logln(&format!("  skip '{}' — dll 미로드(비활성)", mod_id));
            continue;
        }
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

// 컨테이너 child 배열에서 id 로 자식 인덱스 찾기 (위치 앵커용).
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

// 모든 모드의 조각을 타깃 컨테이너에 주입. detour/post_update 에서 템플릿당 1회(LAST_INJECTED 가드).
unsafe fn do_inject(r: usize) -> bool { // true=전부 완료(성공) / false=일부 타깃 미준비(재시도 요)
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 || r <= 0x10000 { return false; }
    MODAL_IDS.lock().unwrap_or_else(|e| e.into_inner()).clear(); // 재주입마다 재수집 (poison-safe)
    let frags = collect_fragments();
    let mut all_ok = true;
    for (ui_path, target_id, pos, is_modal) in &frags {
        if !inject_one(r, base, ui_path, target_id, pos, *is_modal) { all_ok = false; } // 멱등: 이미주입된건 스킵
    }
    if all_ok { logln(&format!("--- inject 완료: {} 조각, template {:#x} ---", frags.len(), r)); }
    all_ok
}

// 조각 1개: .ui 파싱 → 타깃(id; "root"=루트) 컨테이너 찾기 → 위치(pos) 계산 → insert.
// 반환: true = 완료(주입됨/이미있음/영구실패) — 재시도 불필요 / false = 타깃 컨테이너 아직 없음 — 다음 프레임 재시도.
unsafe fn inject_one(r: usize, base: usize, ui_path: &str, target_id: &str, pos: &str, is_modal: bool) -> bool {
    let fname = ui_path.rsplit(['\\', '/']).next().unwrap_or(ui_path).to_string();
    let text = match fs::read(ui_path) { Ok(t) => t, Err(e) => { logln(&format!("  '{}' read err: {}", fname, e)); return true; } };
    if text.is_empty() { return true; }
    // ★ 멱등: 조각 루트가 이미 템플릿에 있으면 스킵 (중복주입 방지 → 재시도 안전).
    if let Some(rid) = root_id_of(&text) { if find_tmpl(r, rid.as_bytes(), 0) != 0 { return true; } }
    let parser: ParserFn = core::mem::transmute(base + PARSER_RVA);
    let mut out = [0u8; 0x400];
    parser(out.as_mut_ptr(), text.as_ptr(), text.len());
    let f = out.as_ptr().add(0x10);
    if *(f as *const usize) == usize::MAX { logln(&format!("  '{}' parse ERROR (.ui 문법/루트# 확인)", fname)); return true; }

    // modal 플래그: 이 조각 루트 id 를 가시성 감시 목록에 등록 (visible 일때 배경 차단)
    if is_modal {
        if let Some(rid) = root_id_of(&text) {
            let mut ids = MODAL_IDS.lock().unwrap();
            if !ids.contains(&rid) { logln(&format!("  modal watch: '{}'", rid)); ids.push(rid); }
        }
    }

    let target = if target_id == "root" { r } else {
        let t = find_tmpl(r, target_id.as_bytes(), 0);
        if t != 0 { t } else { return false; } // ★ 타깃 컨테이너 아직 없음 = 다음프레임 재시도 (root 폴백 안 함=엉뚱위치 방지)
    };
    let cap_p = (target + 0x48) as *mut usize;
    let ptr_p = (target + 0x50) as *mut usize;
    let len_p = (target + 0x58) as *mut usize;
    let old_ptr = *ptr_p; let old_cap = *cap_p; let len = *len_p;
    if len > 2000 { logln("  len 비정상, 중단"); return true; }
    // 위치 계산: 숫자 | after:<id> | before:<id> | end
    let idx = if let Some(a) = pos.strip_prefix("after:") {
        child_index(target, a.as_bytes()).map(|i| i + 1).unwrap_or(len)
    } else if let Some(b) = pos.strip_prefix("before:") {
        child_index(target, b.as_bytes()).unwrap_or(len)
    } else if pos == "end" {
        len
    } else {
        pos.parse::<usize>().unwrap_or(len)
    }.min(len);

    let galloc: AllocFn = core::mem::transmute(base + ALLOC_RVA);
    let new_n = len + 1;
    let new_ptr = galloc(new_n * NT_SIZE, 8);
    if new_ptr == 0 { logln("  game alloc 실패"); return true; }
    // [0..idx) 기존 + [idx]=조각 + [idx..len) 기존
    if old_ptr != 0 && idx != 0 {
        core::ptr::copy_nonoverlapping(old_ptr as *const u8, new_ptr as *mut u8, idx * NT_SIZE);
    }
    core::ptr::copy_nonoverlapping(f, (new_ptr + idx * NT_SIZE) as *mut u8, NT_SIZE);
    if old_ptr != 0 && idx < len {
        core::ptr::copy_nonoverlapping((old_ptr + idx * NT_SIZE) as *const u8, (new_ptr + (idx + 1) * NT_SIZE) as *mut u8, (len - idx) * NT_SIZE);
    }
    // ptr 먼저 → (cap) → len 마지막 순서로 갱신: 중간에 게임이 읽어도 (ptr=new,len=old)=정상범위라 안전.
    *ptr_p = new_ptr; *cap_p = new_n; *len_p = new_n;
    // ★ 옛 배열은 free 안 함(누수 허용). startup 경합에서 게임이 옛 배열을 아직 참조 중일 때
    //   free 하면 use-after-free 크래시(타이틀화면 간헐 꺼짐 원인). 누수는 작고(템플릿 child 몇 개)
    //   main 로드도 드물어 무시 가능. old_ptr/old_cap 은 안전상 의도적 미사용.
    let _ = (old_ptr, old_cap);
    logln(&format!("  OK '{}' → '{}' idx{} (child {}→{})", fname, target_id, idx, len, new_n));
    true
}

struct Ext;
impl ModExtension for Ext {
    fn on_init(&self, _s: &mut Scene, _u: &mut GameUI, _a: &mut Assets) {
        // 백업 설치 (init 에서 이미 됐으면 guard 로 no-op). 로그는 덮지 않음.
        if let Err(e) = unsafe { install() } { if e != "already" { logln(&format!("install @on_init: {}", e)); } }
    }
    fn post_update(&self, _s: &mut Scene, u: &mut GameUI, _a: &mut Assets, _dt: f32) {
        let _ = FRAME.fetch_add(1, Ordering::Relaxed);
        // 진단: 300프레임마다 훅 발화/매칭 상태 기록 (왜 주입 안 되는지 파악)
        let df = DIAG_FRAME.fetch_add(1, Ordering::Relaxed);
        if df % 300 == 100 {
            logln(&format!("diag f={} detour_hits={} len25={} main_hits={} MAIN_TMPL={:#x} LAST_INJ={:#x}",
                df, DETOUR_HITS.load(Ordering::Relaxed), LEN25_HITS.load(Ordering::Relaxed),
                MAIN_HITS.load(Ordering::Relaxed), MAIN_TMPL.load(Ordering::Relaxed), LAST_INJECTED.load(Ordering::Relaxed)));
        }
        // 견고화: 현재 main 템플릿이 아직 주입 안 된(=새로 로드/reload된) 것이면 주입.
        // ★ 첫 로드부터 확실히: 타깃 컨테이너 준비될 때까지 매프레임 재시도(멱등이라 중복 없음).
        //   성공해야 LAST_INJECTED 세팅 → 첫 시도 실패(타깃 미준비)해도 재진입 없이 다음 프레임에 잡힘.
        //   캡 120: 타깃이 끝내 없어도 무한 read_dir IO 안 돌게 포기.
        let r = MAIN_TMPL.load(Ordering::Relaxed);
        if r > 0x10000 && r != LAST_INJECTED.load(Ordering::Relaxed) {
            let n = INJECT_ATTEMPTS.fetch_add(1, Ordering::Relaxed);
            if unsafe { do_inject(r) } || n > 120 {
                LAST_INJECTED.store(r, Ordering::Relaxed);
                INJECT_ATTEMPTS.store(0, Ordering::Relaxed);
            }
        }
        // 모달 가시성 → 배경(top) 입력/호버 차단 (호버누수 방지). modal 플래그 조각의 루트가 하나라도
        //   보이면 top.disabled=true, 다 숨으면 false. (모든 소비자 모달이 공짜로 적용)
        let ids = MODAL_IDS.lock().unwrap().clone();
        if !ids.is_empty() {
            let any = ids.iter().any(|id| nfind(&u.root, id).map(|n| n.visible).unwrap_or(false));
            if let Some(top) = nfind_mut(&mut u.root, BG_BLOCK) { top.disabled = any; }
        }
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    // ★ 훅을 가능한 한 일찍(등록 시점) 설치 — main 템플릿이 on_init 보다 먼저 로드돼도 잡도록.
    if LOG_ENABLED { let _ = fs::write(LOG, "=== tfm2_ui_inject (install @init) ===\n"); }
    match unsafe { install() } { Ok(()) => logln("install ok @init"), Err(e) => logln(&format!("install @init: {}", e)) }
    let mut reg = ModRegistration::new("tfm2_ui_inject");
    reg.set_extension(Ext);
    reg
}
declare_mod!(init);
