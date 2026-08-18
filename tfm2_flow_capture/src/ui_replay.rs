//! ui_replay — 다시보기(리플레이) 편집 팝업 UI 주입 (flow_capture).
//!
//! 목적: 인게임 "다시보기"를 눌렀을 때 즉시 재생하지 않고, 편집 패널을 띄워 팀 전술(13노브×2팀)을
//!   수정한 뒤 "시작"으로 재생 → flow_capture 가 수정된 전술로 재시뮬 로그를 남긴다.
//!   전술 주입 엔진 = game+0xb248 [Strategy;2] overwrite(inject_override, 검증됨).
//!
//! 구조(2026-08-12 RE): 레이아웃 경로 `asset/base/ui/layout/pause`, 컨테이너 `replay_popup`.
//!   세트별 "다시보기" 버튼은 동적 복제(`pause_component/replay_match_slot`)라 정적 트리에 없다.
//!   ⟹ 패널은 replay_popup 에 고정 주입(1회), 세트 클릭 가로채기는 클릭 라우트/리플레이 핸들러 훅.
//!
//! 머신러리 = comptest_unlock/ui_inject.rs 재사용(로더 0x2e35d0 체인훅 + append_child + find_tmpl).
//! 노드 id 는 모드 접두 `fc_` (멱등·충돌 방지). 좌표는 replay_popup 로컬(팝업 좌상단 기준).
#![allow(dead_code)]
use crate::ui_kit;
use mod_api::{UIEvent, UIEventHandlerContext, UIOutEvent};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};

// ── 엔진 함수 (0.5.5 확정 — MIGRATION §7.5 §2 공통 3심볼 · comptest/draft_overlay 등 전 모드 동일) ──
const LOADER_RVA: usize = 0x2e42d0; // 레이아웃 로더(asset_mgr, path*, len) → root — 0.5.5(구0.5.4=0x2e35d0)
const PARSER_RVA: usize = 0x1a3e70; // .ui 조각 파서(out, text*, len) — 0.5.5(구0x1a3ce0)
const ALLOC_RVA: usize = 0x2a9bf30; // 게임 힙 alloc(무시, flags, size) → ptr — 0.5.5(구0x29bb920)
const NT_SIZE: usize = 0x90; // NodeTemplate 크기

type LoaderFn = extern "win64" fn(usize, *const u8, usize) -> usize;
type ParserFn = extern "win64" fn(*mut u8, *const u8, usize);
type AllocFn = extern "win64" fn(usize, usize, usize) -> usize;

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

// ── 전술 행 모델 (게임 네이티브 조합테스트 UI와 동일 라벨·순서) ──
//   각 행 = 게임 라벨 + 옵션 목록. 옵션 = (표시라벨, 바이트쓰기목록[(offset,value)]).
//   바이트 인코딩(데이터 역산 확정, REPORT 2026-08-12): 포지션 탑=0/정글=1/미드=2/바텀=3/서포터=4.
//     bld(b0): 0-4=스플릿@pos, 5=모이기(Gather), 6=유연(Flexible)
//     mor(b4)+mor_pos(b8): b4 0-4=1-3-1 pos1·b8=pos2 / b4=5=모두모이기 / b4=6=1-4·b8=pos
//     bat b12·twr b13·def b14·fin b15·wav b16 (2변형) / foc b17·jng b18·srp b19·srt b20·end b21 (3변형)
struct Opt {
    label: String,
    writes: Vec<(usize, u8)>,
}
struct Row {
    label: &'static str,
    opts: Vec<Opt>,
}
const NR: usize = 12; // 게임 전술 행 수(모르가드사용=mor+mor_pos 통합)

fn build_rows() -> Vec<Row> {
    let pos = ["탑", "정글", "미드", "바텀", "서포터"];
    let simple = |off: usize, labels: &[&str]| -> Row {
        // 첫 인자는 라벨이 아니라 아래 push에서 별도 지정 — 여기선 opts만.
        Row {
            label: "",
            opts: labels
                .iter()
                .enumerate()
                .map(|(i, l)| Opt { label: (*l).to_string(), writes: vec![(off, i as u8)] })
                .collect(),
        }
    };
    let named = |label: &'static str, mut r: Row| -> Row {
        r.label = label;
        r
    };
    let mut rows: Vec<Row> = Vec::new();
    // 1 foc @17
    rows.push(named("라인전 핵심 지역", simple(17, &["탑/미드 집중", "미드/바텀 집중", "구분 없이 전부"])));
    // 2 jng @18
    rows.push(named("라인전 정글 플레이", simple(18, &["성장/커버 위주", "라인 개입 위주", "카운터 정글 위주"])));
    // 3 srp @19
    rows.push(named("라인전 단계 세르펜 시도", simple(19, &["무조건 시도", "유연하게 판단", "되도록 포기"])));
    // 4 srt @20
    rows.push(named("세르펜 탑 라이너 합류", simple(20, &["반드시 합류", "유연하게 판단", "합류하지 않음"])));
    // 5 wav @16
    rows.push(named("미니언 웨이브 관리", simple(16, &["웨이브 우선", "합류 우선"])));
    // 6 bld @0 (게임 표시순: 스플릿×5, 유연하게판단(6), 최대한모이기(5))
    let mut bldo: Vec<Opt> = pos.iter().enumerate().map(|(i, p)| Opt { label: format!("스플릿 - {}", p), writes: vec![(0, i as u8)] }).collect();
    bldo.push(Opt { label: "유연하게 판단".into(), writes: vec![(0, 6)] });
    bldo.push(Opt { label: "최대한 모이기".into(), writes: vec![(0, 5)] });
    rows.push(Row { label: "오브젝트 빌드업", opts: bldo });
    // 7 bat @12
    rows.push(named("오브젝트 전투 전략", simple(12, &["거리 유지 견제", "빠른 이니시에이팅"])));
    // 8 fin @15
    rows.push(named("오브젝트 마무리", simple(15, &["처치 우선", "전투 우선"])));
    // 9 morgard = mor(b4)+mor_pos(b8) 통합
    let mut moro: Vec<Opt> = Vec::new();
    moro.push(Opt { label: "모두 모이기".into(), writes: vec![(4, 5), (8, 0)] });
    for (i, p) in pos.iter().enumerate() {
        moro.push(Opt { label: format!("1-4 스플릿 - {}", p), writes: vec![(4, 6), (8, i as u8)] });
    }
    for (i, p1) in pos.iter().enumerate() {
        for (j, p2) in pos.iter().enumerate() {
            if i != j {
                moro.push(Opt { label: format!("1-3-1 스플릿 - {}/{}", p1, p2), writes: vec![(4, i as u8), (8, j as u8)] });
            }
        }
    }
    rows.push(Row { label: "모르가드 버프 활용", opts: moro });
    // 10 twr @13
    rows.push(named("타워 압박", simple(13, &["다이브", "거리 유지 견제"])));
    // 11 def @14
    rows.push(named("수비 전술", simple(14, &["밀리는 라인 방어", "교전 유도"])));
    // 12 end @21
    rows.push(named("끝내기", simple(21, &["안정적", "유연하게", "공격적"])));
    rows
}
fn rows() -> &'static Vec<Row> {
    static R: std::sync::OnceLock<Vec<Row>> = std::sync::OnceLock::new();
    R.get_or_init(build_rows)
}
/// 24바이트에서 이 행의 현재 옵션 인덱스 해석(모든 writes 일치하는 opt). 없으면 0.
fn decode_row(r: &Row, b: &[u8; 24]) -> u32 {
    for (i, o) in r.opts.iter().enumerate() {
        if o.writes.iter().all(|&(off, v)| b.get(off).copied() == Some(v)) {
            return i as u32;
        }
    }
    0
}

// ── 편집 상태 ──
// EDIT_VAL[team][row] = 현재 선택 옵션 인덱스. 팀 0=블루, 1=레드.
static EDIT_VAL: [[AtomicU32; NR]; 2] = [
    [const { AtomicU32::new(0) }; NR],
    [const { AtomicU32::new(0) }; NR],
];
static PANEL_ON: AtomicBool = AtomicBool::new(false); // 편집창 표시 여부
// 드롭다운(풀다운) 상태: DD_ON이면 (DD_T,DD_K) 셀의 옵션 목록 표시.
static DD_ON: AtomicBool = AtomicBool::new(false);
static DD_T: AtomicUsize = AtomicUsize::new(0);
static DD_K: AtomicUsize = AtomicUsize::new(0);
static ARMED_SEED: AtomicU64 = AtomicU64::new(0); // 편집 대상 리플레이 시드(가로챌 때 채움)
static FH: AtomicUsize = AtomicUsize::new(usize::MAX); // 패널 버튼 클릭 라우트 재등록 추적
// Phase 1c: 클릭 가로채기 배선 전 단계 — 다시보기 버튼의 정확한 런타임 클릭 경로를 알아내기 위해
//   모든 클릭 경로를 비소비(차단 안 함)로 로깅한다. 경로 확보 후 그 suffix 만 정밀 차단으로 전환.
const PANEL_TEST_ALWAYS: bool = true; // ★렌더 검증용 임시 — 리플레이 팝업서 패널 항상 표시
static CLICK_LOG: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
static FH_LOG: AtomicUsize = AtomicUsize::new(usize::MAX); // 클릭 로거 재등록 추적

// ── 클릭 가로채기(v1a) ──
// 다시보기 버튼 경로 = "pause_ui.replay_popup.contents.<행ID>.view" (실측 2026-08-12).
//   첫 클릭 = 소비(차단) + 편집창 표시. "시작" 후 그 행을 PASS_ROW 로 무장 → 두 번째 클릭은 통과(네이티브 재생).
const VIEW_PREFIX: &str = "pause_ui.replay_popup.contents.";
const VIEW_SUFFIX: &str = ".view";
static PENDING_ROW: std::sync::Mutex<String> = std::sync::Mutex::new(String::new()); // 가로챈 행 경로
static PASS_ROW: std::sync::Mutex<String> = std::sync::Mutex::new(String::new()); // 통과 허용(1회) 행 경로
static NEW_INTERCEPT: AtomicBool = AtomicBool::new(false); // 새 가로채기 발생 → pump서 행 덤프
static ROW_DUMP: std::sync::Mutex<String> = std::sync::Mutex::new(String::new()); // 클릭 행 서브트리(진단)
static POPUP_DUMPED: AtomicBool = AtomicBool::new(false); // 리플레이 팝업 런타임 덤프 1회
static PUMP_CTR: AtomicUsize = AtomicUsize::new(0); // pump 프레임 카운터(세트목록 갱신 스로틀)
static STATUS_MSG: std::sync::Mutex<String> = std::sync::Mutex::new(String::new()); // 패널 제목/상태 문구
static FH_ICEPT: AtomicUsize = AtomicUsize::new(usize::MAX);
pub static ICEPT_REG: AtomicUsize = AtomicUsize::new(0); // install_interceptor insert 횟수
pub static ICEPT_MATCH: AtomicUsize = AtomicUsize::new(0); // 필터가 view 경로 매칭한 횟수
pub static ICEPT_LOGGED: AtomicUsize = AtomicUsize::new(0); // 로거가 본 클릭 총수

/// 다시보기 버튼 클릭 감지(비소비) — 클릭 시각을 crate::DABO_CLICK_MS 에 기록.
///   Step1: 생성자 호출자 RVA 캡처를 이 클릭과 상관(3초 창)시켜 리플레이 런처 특정.
fn install_interceptor(ui: &mut mod_api::GameUI) {
    let cur = ui.filter_handler.len();
    let prev = FH_ICEPT.load(Ordering::Relaxed);
    if prev == usize::MAX || cur < prev {
        let filter: Rc<dyn Fn(&UIEvent) -> bool> = Rc::new(move |e: &UIEvent| {
            if let UIEvent::Click { path, .. } = e {
                if path.starts_with(VIEW_PREFIX) && path.ends_with(VIEW_SUFFIX) {
                    ICEPT_MATCH.fetch_add(1, Ordering::Relaxed);
                    let ms = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_millis() as u64)
                        .unwrap_or(0);
                    crate::DABO_CLICK_MS.store(ms, Ordering::Relaxed);
                }
            }
            false // 비소비(네이티브 재생 진행 — Step1은 캡처만)
        });
        let handler: Rc<dyn Fn(&mut UIEventHandlerContext<(), UIOutEvent>)> = Rc::new(|_ctx| {});
        ui.filter_handler.insert(0, (filter, handler));
        ICEPT_REG.fetch_add(1, Ordering::Relaxed);
    }
    FH_ICEPT.store(ui.filter_handler.len(), Ordering::Relaxed);
}

/// 런타임 Node 서브트리 덤프(id·종류·라벨텍스트) — 클릭 행에서 팀명/세트/시드 단서 찾기용.
fn dump_runtime(n: &mod_api::Node, depth: usize, out: &mut String) {
    if depth > 8 || out.len() > 20000 {
        return;
    }
    for _ in 0..depth {
        out.push_str("  ");
    }
    let lbl = ui_kit::label_get(n).unwrap_or_default();
    out.push_str(&format!("{} [{}] '{}'\n", n.id, ui_kit::kind(n), lbl));
    for c in n.child.iter() {
        dump_runtime(c, depth + 1, out);
    }
}

/// 클릭 경로 로거 등록(비소비 = 게임 기본동작 차단 안 함). 진단 전용.
fn install_click_logger(ui: &mut mod_api::GameUI) {
    let cur = ui.filter_handler.len();
    let prev = FH_LOG.load(Ordering::Relaxed);
    if prev == usize::MAX || cur < prev {
        let filter: Rc<dyn Fn(&UIEvent) -> bool> = Rc::new(move |e: &UIEvent| {
            if let UIEvent::Click { path, .. } = e {
                ICEPT_LOGGED.fetch_add(1, Ordering::Relaxed);
                if let Ok(mut g) = CLICK_LOG.lock() {
                    if g.len() < 200 && !g.iter().any(|p| p == path) {
                        g.push(path.clone());
                    }
                }
            }
            false // 비소비
        });
        let handler: Rc<dyn Fn(&mut UIEventHandlerContext<(), UIOutEvent>)> = Rc::new(|_ctx| {});
        // 맨 앞에 삽입 — 게임 핸들러보다 먼저 경로를 관찰(소비는 안 하므로 게임 동작 유지).
        ui.filter_handler.insert(0, (filter, handler));
    }
    FH_LOG.store(ui.filter_handler.len(), Ordering::Relaxed);
}

// ── 상태(주입 머신러리) ──
static BASE: AtomicUsize = AtomicUsize::new(0);
static TRAMP: AtomicUsize = AtomicUsize::new(0);
static INSTALLED: AtomicBool = AtomicBool::new(false);
pub static INSTALL_OK: AtomicUsize = AtomicUsize::new(0);
pub static LOADER_CALLS: AtomicUsize = AtomicUsize::new(0);
pub static PAUSE_HITS: AtomicUsize = AtomicUsize::new(0);
pub static INJECTED: AtomicUsize = AtomicUsize::new(0);
static ASSETS: AtomicUsize = AtomicUsize::new(0);
static LAST_ROOT: AtomicUsize = AtomicUsize::new(0);
static SEEN_PATHS: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
static TREE_DUMP: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
static TREE_DONE: AtomicBool = AtomicBool::new(false);

fn diag_dir() -> Option<std::path::PathBuf> {
    crate::mod_dir()
}

/// 템플릿 트리에서 id 로 노드 찾기
unsafe fn find_tmpl(node: usize, target: &[u8], depth: usize) -> usize {
    if node <= 0x10000 || depth > 12 {
        return 0;
    }
    let idptr = *((node + 0x08) as *const usize);
    let idlen = *((node + 0x10) as *const usize);
    if idlen == target.len() && idptr > 0x10000 {
        if core::slice::from_raw_parts(idptr as *const u8, idlen) == target {
            return node;
        }
    }
    let cptr = *((node + 0x50) as *const usize);
    let clen = *((node + 0x58) as *const usize);
    if cptr > 0x10000 && clen < 1000 {
        for i in 0..clen {
            let f = find_tmpl(cptr + i * NT_SIZE, target, depth + 1);
            if f != 0 {
                return f;
            }
        }
    }
    0
}

/// 진단: 트리의 노드 id 를 들여쓰기와 함께 문자열로 수집(깊이 제한).
unsafe fn dump_tree(node: usize, depth: usize, out: &mut String) {
    if node <= 0x10000 || depth > 8 || out.len() > 60000 {
        return;
    }
    let idptr = *((node + 0x08) as *const usize);
    let idlen = *((node + 0x10) as *const usize);
    let mut label = String::from("?");
    if idptr > 0x10000 && idlen > 0 && idlen < 128 {
        if let Ok(s) = core::str::from_utf8(core::slice::from_raw_parts(idptr as *const u8, idlen)) {
            label = s.to_string();
        }
    }
    let cptr = *((node + 0x50) as *const usize);
    let clen = *((node + 0x58) as *const usize);
    for _ in 0..depth {
        out.push_str("  ");
    }
    out.push_str(&format!("{} (children={})\n", label, clen));
    if cptr > 0x10000 && clen < 1000 {
        for i in 0..clen {
            dump_tree(cptr + i * NT_SIZE, depth + 1, out);
        }
    }
}

/// container_id 컨테이너 끝에 노드 1개 append (comptest 와 동일 계약).
unsafe fn append_child(root: usize, container_id: &[u8], text: &str) -> bool {
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 {
        return false;
    }
    let target = find_tmpl(root, container_id, 0);
    if target == 0 {
        return false;
    }
    let parser: ParserFn = core::mem::transmute(base + PARSER_RVA);
    let mut out = [0u8; 0x400];
    parser(out.as_mut_ptr(), text.as_ptr(), text.len());
    let my = out.as_ptr().add(0x10) as usize;
    if *(my as *const usize) == usize::MAX {
        return false;
    }
    let ptr = *((target + 0x50) as *const usize);
    let len = *((target + 0x58) as *const usize);
    if len > 2000 {
        return false;
    }
    let galloc: AllocFn = core::mem::transmute(base + ALLOC_RVA);
    let new_n = len + 1;
    let new_ptr = galloc(0, 0, new_n * NT_SIZE);
    if new_ptr == 0 {
        return false;
    }
    if ptr != 0 && len != 0 {
        core::ptr::copy_nonoverlapping(ptr as *const u8, new_ptr as *mut u8, len * NT_SIZE);
    }
    core::ptr::copy_nonoverlapping(my as *const u8, (new_ptr + len * NT_SIZE) as *mut u8, NT_SIZE);
    *((target + 0x50) as *mut usize) = new_ptr;
    *((target + 0x48) as *mut usize) = new_n;
    *((target + 0x58) as *mut usize) = new_n;
    true
}

// ── 패널 조각 빌드 ──
// replay_popup 로컬 좌표. 두 열(블루 좌 / 레드 우), 각 12행 = [게임라벨][◀][값][▶].
const PANEL_X: u32 = 18;
const PANEL_Y: u32 = 90;
const PANEL_W: u32 = 912;
const PANEL_H: u32 = 524;
const ROW_Y0: u32 = 96; // 상태줄(y66) 아래로 행 시작
const ROW_H: u32 = 29;
const COL_X: [u32; 2] = [16, 462];

fn opt_row(t: usize, k: usize) -> String {
    let cx = COL_X[t];
    let y = ROW_Y0 + (k as u32) * ROW_H;
    let label = rows()[k].label;
    // 게임 조합테스트 전술 행 그대로: 라벨 | ◀(아이콘) | [드롭다운박스 좌측텍스트 … ▼] | ▶(아이콘).
    let dx = cx + 150; // ◀ 아이콘 버튼
    let bx = cx + 180; // 드롭다운 박스
    let ix = bx + 218; // ▶ 아이콘 버튼
    format!(
        "#fc_{t}_{k}_lbl:label {{ @\"asset/base/style/main#bold_label\"; x: {lx}px; y: {y}px; \
width: 146px; height: 26px; align_y: Center; text: \"{name}\"; size: 13; color: #c2c6ceff; fit_width: true; }} \
#fc_{t}_{k}_dec:color_icon_button {{ @\"asset/base/style/main#tertiary_button\"; x: {dx}px; y: {y}px; \
width: 26px; height: 26px; icon: {{ source: \"asset/base/ui/icons/left\"; rect: {{ x: 10; y: 9; w: 6; h: 8; }} }} }} \
#fc_{t}_{k}_box:color {{ x: {bx}px; y: {y}px; width: 214px; height: 26px; \
color: #4a4c56ff; stroke: 1; back_color: #1a1c26ff; rounding: Uniform {{ rounding: 4; }} \
#fc_{t}_{k}_val:color_selectable {{ @\"asset/base/style/main#strategy_option\"; x: 0px; y: 0px; width: 214px; height: 26px; \
rounding: Uniform {{ rounding: 4; }} label: {{ size: 1; }} selected_label: {{ size: 1; }} text: \"\"; \
#fc_{t}_{k}_txt:label {{ @\"asset/base/style/main#label\"; ignore_event: true; x: 10px; y: 0px; width: 176px; height: 26px; \
align_x: Left; align_y: Center; text: \"\"; size: 12; }} \
#fc_{t}_{k}_chev:image {{ ignore_event: true; x: 192px; y: 9px; width: 12px; height: 8px; \
source: \"asset/base/ui/icons/dropdown\"; color: #9aa0aaff; }} }} }} \
#fc_{t}_{k}_inc:color_icon_button {{ @\"asset/base/style/main#tertiary_button\"; x: {ix}px; y: {y}px; \
width: 26px; height: 26px; icon: {{ source: \"asset/base/ui/icons/right\"; rect: {{ x: 10; y: 9; w: 6; h: 8; }} }} }} ",
        t = t, k = k, y = y, name = label, lx = cx, dx = dx, bx = bx, ix = ix,
    )
}

/// 셀(t,k)의 드롭다운 조각 — 게임처럼 단일 열 스크롤 목록(:scroll_view).
fn dd_fragment(t: usize, k: usize) -> String {
    let opts = &rows()[k].opts;
    let n = opts.len() as u32;
    let iw = 226u32; // 항목 폭(단일 열)
    let ih = 24u32; // 항목 높이
    let maxh = 240u32; // 목록 최대 높이(초과 시 스크롤)
    let sh = (n * ih).min(maxh); // scroll_view 높이
    let bx = COL_X[t] + 180; // 드롭다운 박스 좌측 정렬
    let by = ROW_Y0 + (k as u32) * ROW_H;
    let ch = sh + 2; // 컨테이너(테두리 포함)
    let cw = iw + 2;
    let dd_y = if by + 27 + ch <= PANEL_H { by + 27 } else { by.saturating_sub(ch) };
    let mut s = format!(
        "#fc_dd_{t}_{k}:color {{ visible: false; x: {x}px; y: {y}px; width: {cw}px; height: {ch}px; \
color: #4a4c56ff; stroke: 1; back_color: #0b0d13ff; rounding: Uniform {{ rounding: 4; }} \
#fc_ddsv_{t}_{k}:scroll_view {{ x: 1px; y: 1px; width: {sw}px; height: {sh}px; speed: 100; bar_width: 4; \
bar: {{ source: \"asset/base/sprite/white\"; color: #37d5b3ff; hover: {{ color: #ecfbf8ff; }} }} \
back: {{ source: \"asset/base/sprite/white\"; color: #4a4c56ff; }} \
#fc_ddc_{t}_{k}:empty {{ width: {iw}px; height: {conh}px; child_type: TopToBottom {{ spacing: 0px; }} ",
        t = t, k = k, x = bx, y = dd_y, cw = cw, ch = ch, sw = iw, sh = sh, iw = iw, conh = n * ih
    );
    for (i, o) in opts.iter().enumerate() {
        s.push_str(&format!(
            "#fc_ddi_{t}_{k}_{i}:color_selectable {{ @\"asset/base/style/main#strategy_option\"; \
width: {iw}px; height: {ih}px; align_x: Left; label: {{ size: 12; }} selected_label: {{ size: 12; }} text: \"{lbl}\"; }} ",
            t = t, k = k, i = i, iw = iw, ih = ih, lbl = o.label
        ));
    }
    s.push_str("} } } ");
    s
}

fn panel_fragment() -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "fc_panel:color {{ visible: false; x: {x}px; y: {y}px; width: {w}px; height: {h}px; \
color: #4a4c56ff; stroke: 1; back_color: #12141bf2; rounding: Uniform {{ rounding: 10; }} \
#fc_title:label {{ @\"asset/base/style/main#bold_label\"; x: 16px; y: 8px; width: 300px; height: 28px; \
align_x: Left; align_y: Center; text: \"다시보기 편집\"; size: 19; }} \
#fc_setprev:color_icon_button {{ @\"asset/base/style/main#tertiary_button\"; x: 322px; y: 6px; \
width: 30px; height: 30px; icon: {{ source: \"asset/base/ui/icons/left\"; rect: {{ x: 12; y: 11; w: 6; h: 8; }} }} }} \
#fc_setlabel:label {{ @\"asset/base/style/main#bold_label\"; x: 356px; y: 8px; width: 512px; height: 28px; \
align_x: Center; align_y: Center; text: \"세트 선택\"; size: 15; }} \
#fc_setnext:color_icon_button {{ @\"asset/base/style/main#tertiary_button\"; x: 872px; y: 6px; \
width: 30px; height: 30px; icon: {{ source: \"asset/base/ui/icons/right\"; rect: {{ x: 12; y: 11; w: 6; h: 8; }} }} }} \
#fc_status:label {{ @\"asset/base/style/main#label\"; x: 16px; y: 42px; width: 886px; height: 22px; \
align_x: Center; align_y: Center; text: \"\"; size: 13; color: #fba37cff; }} \
#fc_hb:label {{ @\"asset/base/style/main#bold_label\"; x: 16px; y: 68px; width: 440px; height: 26px; \
align_x: Center; align_y: Center; text: \"블루\"; size: 16; color: #37d5b3ff; }} \
#fc_hr:label {{ @\"asset/base/style/main#bold_label\"; x: 462px; y: 68px; width: 440px; height: 26px; \
align_x: Center; align_y: Center; text: \"레드\"; size: 16; color: #eb3d4dff; }} ",
        x = PANEL_X, y = PANEL_Y, w = PANEL_W, h = PANEL_H
    ));
    for t in 0..2 {
        for k in 0..NR {
            s.push_str(&opt_row(t, k));
        }
    }
    let by = ROW_Y0 + (NR as u32) * ROW_H + 10; // 버튼 줄
    s.push_str(&format!(
        "#fc_start:color_selectable {{ @\"asset/base/style/main#strategy_option\"; x: 316px; y: {by}px; \
width: 264px; height: 34px; label: {{ size: 16; }} selected_label: {{ size: 16; }} text: \"이 전술로 시작\"; }} \
#fc_cancel:color_selectable {{ @\"asset/base/style/main#strategy_option\"; x: 596px; y: {by}px; \
width: 130px; height: 34px; label: {{ size: 16; }} selected_label: {{ size: 16; }} text: \"취소\"; }} ",
        by = by
    ));
    // 드롭다운(맨 뒤 = 최상단 z) — 각 셀별 옵션 목록.
    for t in 0..2 {
        for k in 0..NR {
            s.push_str(&dd_fragment(t, k));
        }
    }
    s.push_str("}"); // fc_panel 닫기
    s
}

/// 밝은 조각 파싱 실패 시 대비 — replay_popup 에 패널 append(멱등).
unsafe fn inject_panel(r: usize) -> bool {
    if r <= 0x10000 {
        return false;
    }
    if find_tmpl(r, b"fc_panel", 0) != 0 {
        return true; // 멱등
    }
    let frag = panel_fragment();
    let n = if append_child(r, b"replay_popup", &frag) { 1 } else { 0 };
    INJECTED.store(n, Ordering::Relaxed);
    n > 0
}

/// 로더 본문 — 진단 + 패널 주입.
fn loader_body(path: *const u8, len: usize, r: usize) {
    if path.is_null() || r <= 0x10000 || len >= 200 {
        return;
    }
    let s = unsafe { core::slice::from_raw_parts(path, len) };
    LOADER_CALLS.fetch_add(1, Ordering::Relaxed);
    if let Ok(txt) = core::str::from_utf8(s) {
        if let Ok(mut g) = SEEN_PATHS.lock() {
            if g.len() < 120 && !g.iter().any(|p| p == txt) {
                g.push(txt.to_string());
            }
        }
    }
    // 정확히 pause 레이아웃일 때만 주입(pause_component/* 서브템플릿엔 replay_popup 컨테이너 없음).
    let is_pause = s == b"asset/base/ui/layout/pause";
    if !is_pause {
        return;
    }
    PAUSE_HITS.fetch_add(1, Ordering::Relaxed);
    if r == LAST_ROOT.load(Ordering::Relaxed) {
        return;
    }
    if !TREE_DONE.load(Ordering::Relaxed) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            let mut dump = String::new();
            let path_str = core::str::from_utf8(s).unwrap_or("?");
            dump.push_str(&format!("path={} root={:#x}\n", path_str, r));
            dump_tree(r, 0, &mut dump);
            if let Ok(mut g) = TREE_DUMP.lock() {
                *g = dump;
            }
        }));
        TREE_DONE.store(true, Ordering::Relaxed);
    }
    let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { inject_panel(r) }))
        .unwrap_or(false);
    if ok {
        LAST_ROOT.store(r, Ordering::Relaxed);
    }
}

extern "win64" fn detour(am: usize, path: *const u8, len: usize) -> usize {
    if am > 0x10000 {
        ASSETS.store(am, Ordering::Relaxed);
    }
    let t = TRAMP.load(Ordering::Relaxed);
    if t == 0 {
        return 0;
    }
    let r = unsafe { core::mem::transmute::<usize, LoaderFn>(t)(am, path, len) };
    loader_body(path, len, r);
    r
}

/// 체인 설치: 진입부 현재 12B 를 트램폴린에 보존.
unsafe fn install_one(base: usize, rva: usize, tramp_slot: &AtomicUsize, detour_addr: usize) -> bool {
    let fn_addr = base + rva;
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    if cur[0] == 0x48 && cur[1] == 0xb8 {
        let tgt = usize::from_le_bytes(cur[2..10].try_into().unwrap());
        if tgt == detour_addr {
            return true;
        }
    }
    let stub = VirtualAlloc(0, 64, MEM_CR, RWX);
    if stub == 0 {
        return false;
    }
    let mut sbuf: Vec<u8> = Vec::new();
    sbuf.extend_from_slice(&cur);
    sbuf.extend_from_slice(&[0x48, 0xb8]);
    sbuf.extend_from_slice(&(fn_addr + 0xc).to_le_bytes());
    sbuf.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(sbuf.as_ptr(), stub as *mut u8, sbuf.len());
    tramp_slot.store(stub, Ordering::Relaxed);
    let mut patch = [0u8; 12];
    patch[0] = 0x48;
    patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&detour_addr.to_le_bytes());
    patch[10] = 0xff;
    patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, RWX, &mut old) == 0 {
        return false;
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    true
}

pub unsafe fn install() -> bool {
    if INSTALLED.swap(true, Ordering::Relaxed) {
        return true;
    }
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 {
        INSTALLED.store(false, Ordering::Relaxed);
        return false;
    }
    BASE.store(base, Ordering::Relaxed);
    let a = install_one(base, LOADER_RVA, &TRAMP, detour as usize);
    INSTALL_OK.store(a as usize, Ordering::Relaxed);
    if !a {
        INSTALLED.store(false, Ordering::Relaxed);
        return false;
    }
    true
}

// ── SDK 전술 파싱 + 시드별 저장(현재값 로드용) ──
// RE 2026-08-12(데이터 역산): Debug 문자열 → 13노브 인덱스. 인코딩 정본 = REPORT RE.
struct RInfo {
    blue: [u8; 24], // 블루팀 Strategy 24B(SDK 파싱)
    red: [u8; 24],  // 레드팀 Strategy 24B
    blue_team: String,
    red_team: String,
    set_idx: u32,   // 0-based 세트 순번(SDK mi.replays 순서)
    match_id: u64,  // 소속 경기 id(같은 대진 다수 구분용)
    blue_win: bool, // 블루팀 승리
}
static REPLAY_MAP: std::sync::Mutex<Option<std::collections::HashMap<u64, RInfo>>> =
    std::sync::Mutex::new(None);
static LOADED_SEED: AtomicU64 = AtomicU64::new(u64::MAX); // 현재 EDIT_VAL에 로드된 시드
// 현재 팝업이 보여주는 세트 목록 = (표시라벨, 시드). pump가 팝업 행에서 매프레임 갱신.
static CUR_SETS: std::sync::Mutex<Vec<(String, u64)>> = std::sync::Mutex::new(Vec::new());
static SEL_SET: AtomicUsize = AtomicUsize::new(0); // 선택된 세트 인덱스

/// UI 색/폰트 마크업(<...>) 제거 → 깨끗한 팀명.
fn strip_markup(s: &str) -> String {
    let mut out = String::new();
    let mut depth = 0i32;
    for c in s.chars() {
        match c {
            '<' => depth += 1,
            '>' => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            _ if depth == 0 => out.push(c),
            _ => {}
        }
    }
    out.trim().to_string()
}
/// (블루팀, 레드팀, 세트번호1based) → 시드. set_idx + 팀사이드로 매칭.
fn find_seed(blue: &str, red: &str, set_num: u32) -> Option<u64> {
    let g = REPLAY_MAP.lock().ok()?;
    let m = g.as_ref()?;
    // 1순위: 팀사이드 + set_idx 일치
    for (&seed, ri) in m.iter() {
        if ri.blue_team == blue && ri.red_team == red && ri.set_idx + 1 == set_num {
            return Some(seed);
        }
    }
    // 2순위: 팀사이드만 일치(set_idx 불명확 대비)
    for (&seed, ri) in m.iter() {
        if ri.blue_team == blue && ri.red_team == red {
            return Some(seed);
        }
    }
    None
}

fn pos_idx(w: &str) -> u32 {
    match w {
        "Top" => 0,
        "Jungle" => 1,
        "Mid" => 2,
        "Bottom" => 3,
        "Support" => 4,
        _ => 0,
    }
}
/// "key: Word" 에서 Word(영숫자/_) 반환.
fn enum_after<'a>(s: &'a str, key: &str) -> &'a str {
    let pat = format!("{}:", key);
    if let Some(i) = s.find(&pat) {
        let rest = s[i + pat.len()..].trim_start();
        let end = rest
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .unwrap_or(rest.len());
        &rest[..end]
    } else {
        ""
    }
}
/// from_key 이후 첫 pos_key 위치의 포지션 인덱스(0-4).
fn pos_after(s: &str, from_key: &str, pos_key: &str) -> u32 {
    if let Some(i) = s.find(&format!("{}:", from_key)) {
        let sub = &s[i..];
        let pk = format!("{}:", pos_key);
        if let Some(j) = sub.find(&pk) {
            let rest = sub[j + pk.len()..].trim_start();
            let end = rest
                .find(|c: char| !c.is_ascii_alphanumeric())
                .unwrap_or(rest.len());
            return pos_idx(&rest[..end]);
        }
    }
    0
}
/// SDK strategy Debug → Strategy 24바이트(검증된 인코딩).
fn parse_to_bytes(s: &str) -> [u8; 24] {
    let mut b = [0u8; 24];
    b[0] = match enum_after(s, "object_buildup") {
        "Gather" => 5,
        "Flexible" => 6,
        "Split" => pos_after(s, "object_buildup", "position") as u8,
        _ => 5,
    };
    match enum_after(s, "morgard_use") {
        "Gather" => {
            b[4] = 5;
            b[8] = 0;
        }
        "Split14" => {
            b[4] = 6;
            b[8] = pos_after(s, "morgard_use", "position") as u8;
        }
        "Split131" => {
            b[4] = pos_after(s, "morgard_use", "position1") as u8;
            b[8] = pos_after(s, "morgard_use", "position2") as u8;
        }
        _ => {
            b[4] = 5;
            b[8] = 0;
        }
    }
    b[12] = if enum_after(s, "object_battle") == "Initiating" { 1 } else { 0 };
    b[13] = if enum_after(s, "tower_press") == "Poking" { 1 } else { 0 };
    b[14] = if enum_after(s, "morgard_defense") == "Battle" { 1 } else { 0 };
    b[15] = if enum_after(s, "object_finish") == "BattlePriority" { 1 } else { 0 };
    b[16] = if enum_after(s, "minion_wave") == "JoinPriority" { 1 } else { 0 };
    b[17] = match enum_after(s, "focused") {
        "Bottom" => 1,
        "All" => 2,
        _ => 0,
    };
    b[18] = match enum_after(s, "early_jungle") {
        "Ganking" => 1,
        "CounterJungle" => 2,
        _ => 0,
    };
    b[19] = match enum_after(s, "early_serpen") {
        "Flexible" => 1,
        "Giveup" => 2,
        _ => 0,
    };
    b[20] = match enum_after(s, "early_serpen_top") {
        "Flexible" => 1,
        "Giveup" => 2,
        _ => 0,
    };
    b[21] = match enum_after(s, "game_finish") {
        "Flexible" => 1,
        "Aggressive" => 2,
        _ => 0,
    };
    b
}

/// dump_replays 시작 시 호출 — 맵 초기화.
pub fn clear_replays() {
    if let Ok(mut g) = REPLAY_MAP.lock() {
        *g = Some(std::collections::HashMap::new());
    }
}
/// 리플레이 1건 등록(시드 → 파싱된 전술 + 팀명 + 세트순번 + 경기id + 승패).
pub fn ingest(
    seed: u64,
    blue_team: &str,
    red_team: &str,
    bstrat: &str,
    rstrat: &str,
    set_idx: u32,
    match_id: u64,
    blue_win: bool,
) {
    if seed == 0 {
        return;
    }
    if let Ok(mut g) = REPLAY_MAP.lock() {
        let m = g.get_or_insert_with(std::collections::HashMap::new);
        m.insert(
            seed,
            RInfo {
                blue: parse_to_bytes(bstrat),
                red: parse_to_bytes(rstrat),
                blue_team: blue_team.to_string(),
                red_team: red_team.to_string(),
                set_idx,
                match_id,
                blue_win,
            },
        );
    }
}
/// 시드의 실제 전술을 EDIT_VAL 에 로드(+ARMED_SEED·제목). 성공 시 true.
pub fn load_seed(seed: u64) -> bool {
    let (blue, red, bt, rt) = {
        let g = match REPLAY_MAP.lock() {
            Ok(g) => g,
            Err(_) => return false,
        };
        let Some(m) = g.as_ref() else { return false };
        let Some(ri) = m.get(&seed) else { return false };
        (ri.blue, ri.red, ri.blue_team.clone(), ri.red_team.clone())
    };
    let rs = rows();
    for k in 0..NR {
        EDIT_VAL[0][k].store(decode_row(&rs[k], &blue), Ordering::Relaxed);
        EDIT_VAL[1][k].store(decode_row(&rs[k], &red), Ordering::Relaxed);
    }
    ARMED_SEED.store(seed, Ordering::Relaxed);
    LOADED_SEED.store(seed, Ordering::Relaxed);
    let _ = (bt, rt); // 세트 라벨(fc_setlabel)이 팀명을 표시하므로 제목은 기본 유지.
    if let Ok(mut m) = STATUS_MSG.lock() {
        *m = String::new();
    }
    true
}
/// 맵의 아무 시드나 하나(데모/테스트용) — 없으면 0.
pub fn any_seed() -> u64 {
    if let Ok(g) = REPLAY_MAP.lock() {
        if let Some(m) = g.as_ref() {
            if let Some((&s, _)) = m.iter().next() {
                return s;
            }
        }
    }
    0
}

/// 팝업 행에서 세트 목록(라벨+시드) 구성 → CUR_SETS. 시리즈 지문으로 정확한 경기 특정.
fn build_cur_sets(ui: &mut mod_api::GameUI) {
    // 1) 팝업 행 읽기 → 지문 (set_idx, blue, red, blue_win, 표시라벨).
    let mut pop: Vec<(u32, String, String, bool, String)> = Vec::new();
    if let Some(popn) = ui_kit::find_mut(&mut ui.root, "replay_popup") {
        if let Some(contents) = ui_kit::find_mut(popn, "contents") {
            for row in contents.child.iter_mut() {
                if row.id.parse::<u64>().is_err() {
                    continue; // info/header 제외, 숫자 id 행만
                }
                let setlbl = ui_kit::find_mut(row, "match").and_then(|n| ui_kit::label_get(n)).unwrap_or_default();
                let blue = ui_kit::find_mut(row, "blue_name").and_then(|n| ui_kit::label_get(n)).map(|s| strip_markup(&s)).unwrap_or_default();
                let red = ui_kit::find_mut(row, "red_name").and_then(|n| ui_kit::label_get(n)).map(|s| strip_markup(&s)).unwrap_or_default();
                let bres = ui_kit::find_mut(row, "blue_result").and_then(|n| ui_kit::label_get(n)).unwrap_or_default();
                let blue_win = bres.contains("win");
                let num = setlbl.rsplit(|c: char| !c.is_ascii_digit()).find(|t| !t.is_empty()).and_then(|t| t.parse::<u32>().ok()).unwrap_or(1);
                let label = format!("{} · {} vs {}", setlbl, blue, red);
                pop.push((num.saturating_sub(1), blue, red, blue_win, label));
            }
        }
    }
    if pop.is_empty() {
        if let Ok(mut g) = CUR_SETS.lock() {
            g.clear();
        }
        return;
    }
    // 2) REPLAY_MAP에서 지문 전체와 맞는 match_id 찾기(각 세트 팀사이드+승패+순번 교집합).
    let mut out: Vec<(String, u64)> = Vec::new();
    if let Ok(g) = REPLAY_MAP.lock() {
        if let Some(m) = g.as_ref() {
            let mut cand: Option<std::collections::HashSet<u64>> = None;
            for (pi, pb, pr, pw, _) in &pop {
                let mut s = std::collections::HashSet::new();
                for ri in m.values() {
                    if ri.set_idx == *pi && ri.blue_win == *pw && ri.blue_team == *pb && ri.red_team == *pr {
                        s.insert(ri.match_id);
                    }
                }
                cand = Some(match cand.take() {
                    None => s,
                    Some(c) => c.intersection(&s).copied().collect(),
                });
            }
            let mid = cand.and_then(|c| c.into_iter().next());
            for (pi, pb, pr, pw, label) in &pop {
                let seed = mid.and_then(|mid| {
                    m.iter().find(|(_, ri)| {
                        ri.match_id == mid && ri.set_idx == *pi && ri.blue_win == *pw && ri.blue_team == *pb && ri.red_team == *pr
                    }).map(|(&s, _)| s)
                });
                match seed {
                    Some(s) => out.push((label.clone(), s)),
                    None => out.push((format!("{} (미발견)", label), 0)),
                }
            }
        }
    }
    if let Ok(mut g) = CUR_SETS.lock() {
        *g = out;
    }
}

/// 세트 선택 이동(+무장 해제·재로드 허용).
fn set_nav(d: i32) {
    let n = CUR_SETS.lock().map(|s| s.len()).unwrap_or(0);
    if n == 0 {
        return;
    }
    let cur = SEL_SET.load(Ordering::Relaxed) as i32;
    let nv = (cur + d).clamp(0, n as i32 - 1);
    SEL_SET.store(nv as usize, Ordering::Relaxed);
    DD_ON.store(false, Ordering::Relaxed);
    // 세트 바꾸면 무장 해제 + 재로드 허용.
    crate::OVERRIDE_ON.store(false, Ordering::Relaxed);
    crate::OVERRIDE_FROM_PANEL.store(false, Ordering::Relaxed);
    LOADED_SEED.store(u64::MAX, Ordering::Relaxed);
    if let Ok(mut m) = STATUS_MSG.lock() {
        *m = String::new();
    }
}

// ── 편집 로직 ──
fn adj(t: usize, k: usize, d: i32) {
    if t >= 2 || k >= NR {
        return;
    }
    DD_ON.store(false, Ordering::Relaxed); // 화살표 조작 시 드롭다운 닫기
    let n = rows()[k].opts.len() as i32;
    if n == 0 {
        return;
    }
    let cur = EDIT_VAL[t][k].load(Ordering::Relaxed) as i32;
    let nv = (cur + d).clamp(0, n - 1);
    EDIT_VAL[t][k].store(nv as u32, Ordering::Relaxed);
}

/// 편집 상태(행별 옵션)를 24B×2 로 패킹해 crate 의 OVERRIDE_* 에 반영.
fn apply_override(seed: u64) {
    let rs = rows();
    for t in 0..2 {
        let mut bytes = [0u8; 24];
        for k in 0..NR {
            let idx = EDIT_VAL[t][k].load(Ordering::Relaxed) as usize;
            if let Some(o) = rs[k].opts.get(idx) {
                for &(off, v) in o.writes.iter() {
                    if off < 24 {
                        bytes[off] = v;
                    }
                }
            }
        }
        for i in 0..3 {
            let word = u64::from_le_bytes(bytes[i * 8..i * 8 + 8].try_into().unwrap());
            crate::OVERRIDE_STRAT[t * 3 + i].store(word, Ordering::Relaxed);
        }
    }
    crate::OVERRIDE_SEED.store(seed, Ordering::Relaxed);
    crate::OVERRIDE_ON.store(seed != 0, Ordering::Relaxed);
}

fn on_start() {
    // Path A: 로드된 세트(ARMED_SEED)에 편집 전술을 무장 → 그 다시보기 재생 시 game+0xb248 덮어씀.
    let seed = ARMED_SEED.load(Ordering::Relaxed);
    if seed == 0 {
        if let Ok(mut m) = STATUS_MSG.lock() {
            *m = "세트를 선택하세요 (◀▶)".to_string();
        }
        return;
    }
    apply_override(seed);
    crate::OVERRIDE_FROM_PANEL.store(true, Ordering::Relaxed); // 파일기반 read_override 덮어쓰기 방지
    if let Ok(mut m) = STATUS_MSG.lock() {
        *m = "전술 변경완료. 다시보기를 누르면 해당 전술로 재생".to_string();
    }
    crate::log(&format!("replay-edit armed: seed={} override ON", seed));
}

/// 매프레임 pump — 클릭 라우트 등록 + 패널 표시/값 반영.
pub fn pump(ui: &mut mod_api::GameUI) {
    // 패널이 런타임 트리에 있으면(=다시보기 화면) 클릭 라우트 등록 + 반영.
    if ui_kit::find_mut(&mut ui.root, "fc_panel").is_none() {
        return;
    }
    // 리플레이 팝업이 떠 있으면 패널 표시.
    if PANEL_TEST_ALWAYS {
        PANEL_ON.store(true, Ordering::Relaxed);
    }
    // Step1: 다시보기 클릭 시각 기록(비소비) — 생성자 호출자 캡처 상관용.
    install_interceptor(ui);
    // ★진단(1회): 리플레이 팝업 런타임 서브트리 덤프 — 팀명/세트/시드 단서 위치 파악용.
    if !POPUP_DUMPED.swap(true, Ordering::Relaxed) {
        if let Some(pop) = ui_kit::find_mut(&mut ui.root, "replay_popup") {
            let mut d = String::new();
            dump_runtime(&*pop, 0, &mut d);
            if let Ok(mut g) = ROW_DUMP.lock() {
                *g = d;
            }
        }
    }
    // ★팝업 세트 목록 갱신(팝업 행에서 SET N + 팀명 읽어 시드 매핑). 30프레임마다(맵 스캔 부하).
    let ctr = PUMP_CTR.fetch_add(1, Ordering::Relaxed);
    let need = CUR_SETS.lock().map(|s| s.is_empty()).unwrap_or(true);
    if ctr % 30 == 0 || need {
        build_cur_sets(ui);
    }
    let nsets = CUR_SETS.lock().map(|s| s.len()).unwrap_or(0);
    if nsets > 0 {
        let cur = SEL_SET.load(Ordering::Relaxed);
        if cur >= nsets {
            SEL_SET.store(nsets - 1, Ordering::Relaxed);
        }
    }
    // 선택 세트 → 자동 로드(무장 전에만).
    let sel_seed = CUR_SETS
        .lock()
        .ok()
        .and_then(|s| s.get(SEL_SET.load(Ordering::Relaxed)).map(|e| e.1))
        .unwrap_or(0);
    if sel_seed != 0
        && sel_seed != LOADED_SEED.load(Ordering::Relaxed)
        && !crate::OVERRIDE_FROM_PANEL.load(Ordering::Relaxed)
    {
        load_seed(sel_seed);
    }
    // 클릭 라우트: 세트◀▶ + 행◀▶ ×24 + 시작 + 취소.
    let mut routes: Vec<(String, ui_kit::ClickFn)> = Vec::with_capacity(2 * NR * 2 + 4);
    routes.push(ui_kit::route("fc_setprev", Rc::new(|| set_nav(-1))));
    routes.push(ui_kit::route("fc_setnext", Rc::new(|| set_nav(1))));
    for t in 0..2 {
        for k in 0..NR {
            let (tt, kk) = (t, k);
            routes.push(ui_kit::route(&format!("fc_{}_{}_dec", t, k), Rc::new(move || adj(tt, kk, -1))));
            routes.push(ui_kit::route(&format!("fc_{}_{}_inc", t, k), Rc::new(move || adj(tt, kk, 1))));
            // 값 박스 + ▼ 클릭 = 드롭다운 토글.
            let toggle = Rc::new(move || {
                if DD_ON.load(Ordering::Relaxed) && DD_T.load(Ordering::Relaxed) == tt && DD_K.load(Ordering::Relaxed) == kk {
                    DD_ON.store(false, Ordering::Relaxed);
                } else {
                    DD_T.store(tt, Ordering::Relaxed);
                    DD_K.store(kk, Ordering::Relaxed);
                    DD_ON.store(true, Ordering::Relaxed);
                }
            });
            routes.push(ui_kit::route(&format!("fc_{}_{}_val", t, k), toggle));
        }
    }
    // 모든 드롭다운 항목 클릭 라우트를 항상 등록(숨은 항목은 렌더 안 돼 클릭 불가 → 안전).
    //   ⚠동적으로 열린 셀만 등록하면 ensure_clicks가 재등록 안 해 항목 클릭이 죽는다(실측).
    for t in 0..2 {
        for k in 0..NR {
            let nopt = rows()[k].opts.len();
            for i in 0..nopt {
                let (tt, kk, ii) = (t, k, i);
                routes.push(ui_kit::route(&format!("fc_ddi_{}_{}_{}", t, k, i), Rc::new(move || {
                    EDIT_VAL[tt][kk].store(ii as u32, Ordering::Relaxed);
                    DD_ON.store(false, Ordering::Relaxed);
                })));
            }
        }
    }
    routes.push(ui_kit::route("fc_start", Rc::new(on_start)));
    routes.push(ui_kit::route("fc_cancel", Rc::new(|| {
        // 무장 해제 + 재로드 허용.
        crate::OVERRIDE_ON.store(false, Ordering::Relaxed);
        crate::OVERRIDE_FROM_PANEL.store(false, Ordering::Relaxed);
        LOADED_SEED.store(u64::MAX, Ordering::Relaxed);
        if let Ok(mut m) = STATUS_MSG.lock() {
            *m = String::new();
        }
    })));
    ui_kit::ensure_clicks_front(ui, &FH, routes);
    // 세트 라벨 반영.
    {
        let lbl = CUR_SETS
            .lock()
            .ok()
            .and_then(|s| s.get(SEL_SET.load(Ordering::Relaxed)).map(|e| e.0.clone()))
            .unwrap_or_else(|| "세트정보 읽는중".to_string());
        if let Some(n) = ui_kit::find_mut(&mut ui.root, "fc_setlabel") {
            if ui_kit::label_get(n).as_deref() != Some(lbl.as_str()) {
                ui_kit::label_set(n, &lbl);
            }
        }
    }

    // 표시 반영.
    let on = PANEL_ON.load(Ordering::Relaxed);
    ui_kit::set_visible_by_id(&mut ui.root, "fc_panel", on);
    if !on {
        return;
    }
    // 상태 문구 반영(블루/레드 아래 별도 1줄). 제목(fc_title)은 조각에 고정.
    {
        let msg = STATUS_MSG.lock().map(|g| g.clone()).unwrap_or_default();
        if let Some(n) = ui_kit::find_mut(&mut ui.root, "fc_status") {
            if ui_kit::label_get(n).as_deref() != Some(msg.as_str()) {
                ui_kit::label_set(n, &msg);
            }
        }
    }
    // 값 박스 텍스트 반영(color_selectable = toggle_text_set). 매프레임(팝업 재열림 시 stale 방지).
    let rs = rows();
    for t in 0..2 {
        for k in 0..NR {
            let vi = EDIT_VAL[t][k].load(Ordering::Relaxed) as usize;
            let txt = rs[k].opts.get(vi).map(|o| o.label.as_str()).unwrap_or("?");
            let id = format!("fc_{}_{}_txt", t, k);
            if let Some(n) = ui_kit::find_mut(&mut ui.root, &id) {
                if ui_kit::label_get(n).as_deref() != Some(txt) {
                    ui_kit::label_set(n, txt);
                }
            }
        }
    }
    // 드롭다운 표시 반영(열린 셀만 보임).
    let (don, dt, dk) = (DD_ON.load(Ordering::Relaxed), DD_T.load(Ordering::Relaxed), DD_K.load(Ordering::Relaxed));
    for t in 0..2 {
        for k in 0..NR {
            let vis = don && t == dt && k == dk;
            ui_kit::set_visible_by_id(&mut ui.root, &format!("fc_dd_{}_{}", t, k), vis);
        }
    }
}

/// 진단 파일 flush — post_update 에서 저빈도 호출.
pub fn write_diag() {
    let Some(mut dir) = diag_dir() else { return };
    let paths = SEEN_PATHS.lock().map(|g| g.join("\n")).unwrap_or_default();
    dir.push("ui_paths.txt");
    let _ = std::fs::write(
        &dir,
        {
            let watched = crate::LAST_WATCHED_SEED.load(Ordering::Relaxed);
            let map_size = REPLAY_MAP
                .lock()
                .ok()
                .and_then(|g| g.as_ref().map(|m| m.len()))
                .unwrap_or(0);
            let in_map = REPLAY_MAP
                .lock()
                .ok()
                .and_then(|g| g.as_ref().map(|m| m.contains_key(&watched)))
                .unwrap_or(false);
            format!(
                "install_ok={} pause_hits={} injected={} panel_on={}\n\
last_watched={} map_size={} watched_in_map={} loaded_seed={} armed_seed={} from_panel={}\n\n{}\n",
                INSTALL_OK.load(Ordering::Relaxed),
                PAUSE_HITS.load(Ordering::Relaxed),
                INJECTED.load(Ordering::Relaxed),
                PANEL_ON.load(Ordering::Relaxed),
                watched,
                map_size,
                in_map,
                LOADED_SEED.load(Ordering::Relaxed),
                ARMED_SEED.load(Ordering::Relaxed),
                crate::OVERRIDE_FROM_PANEL.load(Ordering::Relaxed),
                paths
            )
        },
    );
    dir.pop();
    if let Ok(g) = TREE_DUMP.lock() {
        if !g.is_empty() {
            dir.push("ui_tree.txt");
            let _ = std::fs::write(&dir, g.as_str());
            dir.pop();
        }
    }
    // 클릭 경로 로그(다시보기 버튼 suffix 확보용).
    if let Ok(g) = CLICK_LOG.lock() {
        if !g.is_empty() {
            dir.push("click_paths.txt");
            let _ = std::fs::write(&dir, g.join("\n"));
            dir.pop();
        }
    }
    // 클릭된 행 서브트리 덤프(팀명/세트/시드 단서 — 시드 매핑 설계용).
    if let Ok(g) = ROW_DUMP.lock() {
        if !g.is_empty() {
            dir.push("row_dump.txt");
            let _ = std::fs::write(&dir, g.as_str());
            dir.pop();
        }
    }
}
