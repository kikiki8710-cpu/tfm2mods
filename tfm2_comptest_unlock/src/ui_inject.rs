//! ui_inject — comp_test(조합 테스트) 아이템칸에 "모드 소유" 드롭다운 오버레이 주입.
//!
//! 왜 오버레이인가:
//!   네이티브 #item0/1/2 드롭다운은 클릭이 카테고리 0~6 으로만 커밋된다(모드템 id>=30 표현 불가,
//!   ghidra-re 확증 + item_tactics 개인전술에서 동일 결론). 그래서 네이티브 위에 모드 소유
//!   드롭다운을 append 로 덮어 클릭을 가로채고, 선택값은 모드 자체 저장소에 보관한다.
//!   (append = 나중 child = 상단 렌더 + 상단 히트)
//!
//! 주입 대상: training.ui  #comp_test_popup → #builds → #blue0..4 / #red0..4
//!   각 행의 네이티브 item0/1/2 는 x=146/296/446 (width 140) → 같은 자리에 ct_i{0,1,2}_{행} 을 덮는다.
//!
//! ⚠ item_tactics 의 4칸 모드(MODE4)가 켜지면 같은 training 행을 replace_children 으로 통째 재구성한다.
//!   그 경우 이 오버레이가 지워지므로 순서/체인 조율이 필요(현재 4items.cfg = slots 3 이라 비활성).
//! ⚠ 로더 훅은 체인 방식(진입부 현재 12B 를 트램폴린에 담아 보존) — 다른 모드 훅 위에 얹힌다.
#![allow(dead_code)]
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

// 엔진 함수 (**0.5.2**) — item_tactics/ai_adjust/draft_overlay 공용 확정값
//
// ★★0.5.2 최대 소득 = **잠복 버그 동시 수정**. 0.5.1에서 asset-get `.ui` 로더는 바이트동일 copy로 분화돼
//   있었는데, 이 파일의 종전 주석("0x40f3d0 = ...training 계열 / 0xeb17d0 = strategy 전용이라 제거")은
//   **사실과 반대**였다. 0.5.1 exe string-xref 실측:
//     "asset/base/ui/layout/training" → **0xeb17d0 ×12**  (0x40f3d0 = 0건)
//     "asset/base/ui/layout/main"     → 0x40f3d0 ×17
//     "asset/base/ui/layout/strategy" → 0xeb17d0 ×2
//   ⇒ training은 **0xeb17d0을 탄다**. "strategy 전용이라 불필요"라며 그 훅을 제거한 시점부터 0.5.1에서
//   **comp_test 드롭다운 오버레이 주입이 죽어 있었다**(07-21 인게임 검증은 양쪽 다 훅하던 때의 결과).
//
// ★0.5.2에선 그 copy 분화가 **단일 0x5ac950으로 수렴**(main 17 / strategy 2 / training 12 전부 같은 타깃).
//   ⇒ LOADER_RVA를 0x5ac950으로 두면 마이그와 위 회귀 수정이 동시에 된다. **세컨드 훅 추가 금지**
//   (같은 주소에 두 번 걸면 자기체인 = 본문 2회 실행). 타 세션 3건(ai_adjust·item_tactics·serpen)과 교차확증.
// ★0.5.3(2026-07-29~30): 0.5.2 0x5ac950 → **0x2e1550**. 문자열-xref 로 확정
//   ('asset/base/ui/layout/{main,strategy,training,banpick}' lea 직후 call 이 만장일치로 수렴).
//   ⛔자동매칭 표의 "확정 0x91ab0" 은 **오답**(clone family 형제 혼동) — 0.5.3 은 진입 24B 가 449개
//   함수와 동일해 바이트로 형제를 못 가른다. 결정적 지문 = 콜러 수(0.5.2 507 ↔ 0.5.3 511 vs 오답 2).
const LOADER_RVA: usize = 0x2e35d0;        // ★0.5.4(구0.5.3=0x2e1550) 문자열-xref 31표 만장일치·콜사이트 511=511
const PARSER_RVA: usize = 0x1a3ce0;        // ★0.5.4(구0.5.3=0x1a6530) 내부시그 59/59표·크기 2192B 동일·콜사이트 5=5 // 0.5.3(0.5.2=0x24b5a00) 실측 확정·3인자 계약·노드 stride 0x90 유지
// 0.5.3: __rust_alloc 이 align별 심 + impl 로 분해 ⟹ impl 직접호출 **3인자**가 전 모드 정본.
//   (rcx 무시, rdx=flags(0), r8=size) -> rax, 실패 시 0. ⛔align8 심은 OOM 시 abort 라 미채택.
const ALLOC_RVA: usize = 0x29bb920;        // ★0.5.4(구0.5.3=0x28f7df0) 내부시그 14/14표·60B 동일·GetProcessHeap IAT 확인 // 0.5.3(0.5.2=0x25c4d30) HeapAlloc 래퍼 = exe 내 유일
const NT_SIZE: usize = 0x90;               // NodeTemplate 크기 — 0.5.2 struct 오프셋 불변(CASE-불변)이라 유지

const PATH_TRAIN: &[u8] = b"asset/base/ui/layout/training";

// 행 id (blue0..4 = 블루 pos0~4, red0..4 = 레드 pos0~4) — 결과창 슬롯 순서와 동일 체계
pub const ROWS: [&str; 10] = ["blue0", "blue1", "blue2", "blue3", "blue4",
                              "red0", "red1", "red2", "red3", "red4"];
// 오버레이 좌표 = **템플릿 생성 시점에 한 번만** 결정. 런타임 write 금지.
//   ⛔런타임에 레이아웃 블록(+0x84 계열)만 덮어쓰면 screen_x(+0x240)가 안 따라와 시각/히트 박스가
//     어긋나 **클릭이 관통**하고, 매 프레임 덮어쓰면 게임과 싸워 **떨림**이 생긴다(2026-07-21 실측).
//   레이아웃: 4칸 모드(item_tactics) = x146/258/370 · w104 (it4_slot3=482) / 바닐라 3칸 = x146/296/446 · w140.
//   선택 = comptest_items.cfg 의 `layout = 3|4` (기본 4). 변경 시 게임 재시작 필요(4items.cfg 와 동일 제약).
pub static LAYOUT4: AtomicBool = AtomicBool::new(true);
const ITEM_X4: [u32; 3] = [146, 258, 370];
const ITEM_X3: [u32; 3] = [146, 296, 446];

fn item_x(j: usize) -> u32 { if LAYOUT4.load(Ordering::Relaxed) { ITEM_X4[j] } else { ITEM_X3[j] } }
fn item_w() -> u32 { if LAYOUT4.load(Ordering::Relaxed) { 104 } else { 140 } }

/// 모드 소유 드롭다운 노드 텍스트. id 는 반드시 모드 접두(ct_) — 멱등 마커 오판·타모드 충돌 방지.
// ⚠ append_child 용 조각은 id 앞에 `#` 을 붙이지 않는다(붙이면 파서 실패 → 주입 0).
//   `#id:type` 형태는 replace_children(행 자식 전체 재작성)에서만 쓰는 문법. (item_tactics mod_dd/COL4 대조)
fn ct_dd(id: &str, x: u32) -> String {
    let w = item_w();
    format!("{}:dropdown {{ @\"asset/base/style/main#dropdown\"; x: {}px; y: 10px; width: {}px; \
height: 36px; text: {{ size: 12; }} item_text: {{ size: 12; }} \
text_layout: {{ x: 6px; y: 5px; width: {}px; height: 26px; }} \
item_layout: {{ x: 2px; y: 0px; width: {}px; height: 36px; }} \
item_text_layout: {{ x: 6px; y: 5px; width: {}px; height: 26px; }} max_items_height: 252; }}",
        id, x, w, w - 32, w - 4, w - 32)
}

/// 슬롯(0~9) + 칸(0~2) → 드롭다운 노드 id
pub fn dd_id(slot: usize, item: usize) -> String { format!("ct_i{}_{}", item, ROWS[slot]) }

static BASE: AtomicUsize = AtomicUsize::new(0);
static TRAMP: AtomicUsize = AtomicUsize::new(0);
static INSTALLED: AtomicBool = AtomicBool::new(false);
static LAST_TRAIN: AtomicUsize = AtomicUsize::new(0);
pub static INJECTED: AtomicUsize = AtomicUsize::new(0);   // 주입 성공 노드 수(진단)
// ── 진단 계측: 어디서 막혔는지 단계별로 ──
pub static INSTALL_OK: AtomicUsize = AtomicUsize::new(0);   // 0.5.2=로더 단일수렴이라 0=미설치 / 1=설치(~~구 3=copy 둘 다~~)
pub static LOADER_CALLS: AtomicUsize = AtomicUsize::new(0); // 로더 훅 발화 총 횟수
pub static TRAIN_SEEN: AtomicUsize = AtomicUsize::new(0);   // 경로에 "training" 포함
pub static PATH_HIT: AtomicUsize = AtomicUsize::new(0);     // PATH_TRAIN 정확 일치
pub static ROWS_FOUND: AtomicUsize = AtomicUsize::new(0);   // find_tmpl 로 찾은 행 수
pub static SEEN_PATHS: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

type LoaderFn = extern "win64" fn(usize, *const u8, usize) -> usize;
type ParserFn = extern "win64" fn(*mut u8, *const u8, usize);
type AllocFn = extern "win64" fn(usize, usize, usize) -> usize; // 0.5.3: (무시, flags, size)

#[link(name = "kernel32")]
extern "system" {
    fn VirtualAlloc(a: usize, s: usize, t: u32, p: u32) -> usize;
    fn VirtualProtect(a: usize, s: usize, p: u32, o: *mut u32) -> i32;
    fn FlushInstructionCache(h: usize, a: usize, s: usize) -> i32;
    fn GetCurrentProcess() -> usize;
    fn GetModuleHandleW(m: *const u16) -> usize;
}
const MEM_CR: u32 = 0x1000 | 0x2000;
const RWX: u32 = 0x40;

/// 템플릿 트리에서 id 로 노드 찾기
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
            let f = find_tmpl(cptr + i * NT_SIZE, target, depth + 1);
            if f != 0 { return f; }
        }
    }
    0
}

/// container_id 컨테이너 끝에 노드 1개 append (자식 배열 재할당 — 옛 배열 leak 은 무해)
unsafe fn append_child(root: usize, container_id: &[u8], text: &str) -> bool {
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 { return false; }
    let target = find_tmpl(root, container_id, 0);
    if target == 0 { return false; }
    let parser: ParserFn = core::mem::transmute(base + PARSER_RVA);
    let mut out = [0u8; 0x400];
    parser(out.as_mut_ptr(), text.as_ptr(), text.len());
    let my = out.as_ptr().add(0x10) as usize;
    if *(my as *const usize) == usize::MAX { return false; }
    let ptr = *((target + 0x50) as *const usize);
    let len = *((target + 0x58) as *const usize);
    if len > 2000 { return false; }
    let galloc: AllocFn = core::mem::transmute(base + ALLOC_RVA);
    let new_n = len + 1;
    let new_ptr = galloc(0, 0, new_n * NT_SIZE); // 0.5.3 impl 계약: rcx 무시·rdx=flags·r8=size
    if new_ptr == 0 { return false; }
    if ptr != 0 && len != 0 { core::ptr::copy_nonoverlapping(ptr as *const u8, new_ptr as *mut u8, len * NT_SIZE); }
    core::ptr::copy_nonoverlapping(my as *const u8, (new_ptr + len * NT_SIZE) as *mut u8, NT_SIZE);
    *((target + 0x50) as *mut usize) = new_ptr;
    *((target + 0x48) as *mut usize) = new_n;
    *((target + 0x58) as *mut usize) = new_n;
    true
}

/// ★2026-07-22 유저 확정: 조합테스트 개인전술 아이템 드롭다운 = **tfm2_item_tactics 단독 소유**.
///   두 모드가 같은 행(#blue0..4/#red0..4)에 각각 append 해서 **4px 어긋난 이중 박스**로 겹쳐 보이는
///   문제(유저 제보 "4개일때 살짝 겹쳐져있어")를 해소. 런타임 실측으로 확인된 중복:
///     네이티브 item0/1/2 = x146/296/446(w140, item_tactics가 매프레임 hide)
///     item_tactics       = it4_s0~it4_slot3  x142/258/374/490(w110)  ← 4칸·모드템 = 상위호환
///     이 모드            = ct_i0~2_<행>      x146/258/370            ← 3칸, 중복이라 끔
///   ⚠ 이 상수를 false 로 두면 아래 선택 폴링(dd_selected→ITEM_OVERRIDE)도 노드 부재로 자연히
///     조기탈출하므로, 이 모드의 comp_test **아이템 지정 기능 전체가 잠긴다**(선수 중복 해제 등
///     다른 기능은 무관하게 그대로 동작). 되살리려면 이 상수만 true — 나머지 코드는 전부 보존.
const ITEM_DD_ENABLED: bool = false;

/// comp_test 10행 × 3칸에 모드 소유 드롭다운 오버레이 append. 이미 있으면 skip(멱등).
unsafe fn inject_comptest(r: usize) -> bool {
    if !ITEM_DD_ENABLED { return true; } // true=완료 취급(로더가 매 로드마다 재시도하지 않도록)
    if r <= 0x10000 { return false; }
    if find_tmpl(r, b"ct_i0_blue0", 0) != 0 { return true; }   // 멱등 마커(모드 접두 id)
    let mut n = 0;
    for (si, rid) in ROWS.iter().enumerate() {
        if find_tmpl(r, rid.as_bytes(), 0) == 0 { continue; }
        ROWS_FOUND.fetch_add(1, Ordering::Relaxed);
        for j in 0..3 {
            let txt = ct_dd(&dd_id(si, j), item_x(j));
            if append_child(r, rid.as_bytes(), &txt) { n += 1; }
        }
    }
    INJECTED.store(n, Ordering::Relaxed);
    n > 0
}

fn loader_body(path: *const u8, len: usize, r: usize) {
    if path.is_null() || r <= 0x10000 || len >= 200 { return; }
    let s = unsafe { core::slice::from_raw_parts(path, len) };
    LOADER_CALLS.fetch_add(1, Ordering::Relaxed);
    if s.windows(8).any(|w| w == b"training") {
        TRAIN_SEEN.fetch_add(1, Ordering::Relaxed);
        if let Ok(txt) = core::str::from_utf8(s) {
            let mut g = SEEN_PATHS.lock().unwrap_or_else(|e| e.into_inner());
            if g.len() < 40 && !g.iter().any(|p| p == txt) { g.push(txt.to_string()); }
        }
    }
    if s == PATH_TRAIN { PATH_HIT.fetch_add(1, Ordering::Relaxed); }
    if s == PATH_TRAIN && r != LAST_TRAIN.load(Ordering::Relaxed) {
        let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { inject_comptest(r) }))
            .unwrap_or(false);
        if ok { LAST_TRAIN.store(r, Ordering::Relaxed); }
    }
}

extern "win64" fn detour(am: usize, path: *const u8, len: usize) -> usize {
    let t = TRAMP.load(Ordering::Relaxed);
    if t == 0 { return 0; }
    let r = unsafe { core::mem::transmute::<usize, LoaderFn>(t)(am, path, len) };
    loader_body(path, len, r);
    r
}
/// 체인 설치: 진입부 현재 12B(원본 프롤로그 or 다른 모드의 jmp)를 트램폴린에 담아 보존.
unsafe fn install_one(base: usize, rva: usize, tramp_slot: &AtomicUsize, detour_addr: usize) -> bool {
    let fn_addr = base + rva;
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    if cur[0] == 0x48 && cur[1] == 0xb8 {
        let tgt = usize::from_le_bytes(cur[2..10].try_into().unwrap());
        if tgt == detour_addr { return true; }   // 이미 내 훅
    }
    let stub = VirtualAlloc(0, 64, MEM_CR, RWX);
    if stub == 0 { return false; }
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&cur);
    s.extend_from_slice(&[0x48, 0xb8]);
    s.extend_from_slice(&(fn_addr + 0xc).to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    tramp_slot.store(stub, Ordering::Relaxed);
    let mut patch = [0u8; 12];
    patch[0] = 0x48; patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&detour_addr.to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, RWX, &mut old) == 0 { return false; }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    true
}

pub unsafe fn install() -> bool {
    if INSTALLED.swap(true, Ordering::Relaxed) { return true; }
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { INSTALLED.store(false, Ordering::Relaxed); return false; }
    BASE.store(base, Ordering::Relaxed);
    let a = install_one(base, LOADER_RVA, &TRAMP, detour as usize);
    INSTALL_OK.store(a as usize, Ordering::Relaxed);
    if !a { INSTALLED.store(false, Ordering::Relaxed); return false; }
    true
}
