//! ui_inject — tfm2_4items 체이닝 로더 훅 UI 수정.
//!   ① 체이닝 install: 진입부 현재 12바이트를 저장하고 앞에 끼움(prepend) → 다른 모드(ai_adjust/scrim)의
//!      단일소유 훅 "뒤에" 체인. post_update에서 늦게 설치(그 모드들 init 후)해서 순서 보장.
//!      ai_adjust off여도 원본프롤로그 저장→단독 작동. 재init 멱등(INSTALLED).
//!   ② player_info/wide_player_info 템플릿 로드 시 → 내 수정본(.ui, 압축 4슬롯)으로 root 자식 교체.
//!      (override와 달리 로더 훅이라 다른 override 위에 얹힘·체이닝. 단 "수정"은 모드간 합성 안 되나
//!       player_info는 아무도 안 건드려 안전.)
//!   RVA(0.4.14 핫픽스): LOADER 0x540ad0 / PARSER 0x220e100 / ALLOC 0x231fb70.
#![allow(dead_code)]
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

// scrim/draft_overlay와 동일 엔진 함수. (구 0.5.0_3: LOADER 0x51cd40/PARSER 0x2499f30/ALLOC 0x25ab3d0/DEALLOC 0x25ab430)
// ★★0.5.2 재확인 결과(2026-07-22, string-xref 정적 재도출 — 0.5.1 정답 재현으로 방법론 검증됨):
//   asset-get 모노모픽 copy 분화(0.5.1=copy#1 0x40f3d0 main계열 / copy#2 0xeb17d0 strategy·training·밴픽)가
//   **0.5.2에선 우리 4개 대상 경로 전부 같은 copy 0x5ac950 으로 수렴**했다.
//   근거: player_info lea@0x50a671→call 0x5ac950 / wide lea@0x50a6b5→call 0x5ac950 /
//         strategy lea@0xcdee2b·0xce3a19→call 0x5ac950 / training lea 다수→전부 call 0x5ac950.
//   (0.5.1에서 같은 스크립트가 0x40f3d0·0xeb17d0을 정확히 재현 = 방법 신뢰.)
//   ⇒ STRAT_LOADER = LOADER 동일주소 → install()에서 세컨드 훅은 **스킵**(같은 함수 이중 체인 방지).
// ★★0.5.3 재핀(2026-07-29) — LOADER는 **문자열-xref 정본 경로**로 직접 도출(자동매칭 표의 0x91ab0은 오답:
//   같은 모노모픽 clone family의 형제였다). 도구를 0.5.2에 먼저 돌려 문서화된 정답(0x5ac950)을 재현해 검증한 뒤 적용.
//   0.5.3 실측: player_info lea@0xa93f3a / wide lea@0xa93f7e / strategy lea@0x200fb1b·0x201446f /
//               training lea 12곳 — **전부 call 0x2e1550 으로 수렴** ⟹ STRAT_LOADER = LOADER 유지(세컨드 훅 스킵).
const LOADER_RVA: usize = 0x2ea930; // 0.5.6(구0.5.5=0x2e42d0). 문자열-xref layout/main×17 수렴(clone family라 xref만 유효). // 0.5.5(구0.5.4=0x2e35d0). 문자열-xref 정본(layout/main×17·banpick 계열) 재현 OK, clone family라 xref만 유효. // 0.5.4(구0.5.3=0x2e1550). 문자열-xref 정본 경로 재도출: player_info 1 / wide 1 / strategy 2 / training 12 = 전부 이 함수로 수렴(0.5.3과 동수) + 본문 100% 동일. // 0.5.3(구0.5.2=0x5ac950). 진입 24B **바이트 완전동일**(push8+sub 0x98) → 12B 체이닝 install 무수정 동작.
const STRAT_LOADER_RVA: usize = 0x2ea930; // 0.5.6: LOADER와 동일 copy(구0.5.5=0x2e42d0).
const PARSER_RVA: usize = 0x1ab310; // 0.5.6(구0.5.5=0x1a3e70, skel UNIQUE·BYTE=SAME). NT_SIZE 0x90 무변경. // 0.5.5(구0.5.4=0x1a3ce0, skel 유일). NT_SIZE 0x90 무변경. // 0.5.4(구0.5.3=0x1a6530). 본문 100% 동일·크기 0x890·콜러 5개 동수·mod.rs panic-loc 3개 동일 ⇒ NT_SIZE 0x90 무변경. // 0.5.3(구0.5.2=0x24b5a00). 3인자(out, ptr, len) 계약·`:`/`{`/`}` 파싱·에러 시 out[2]=-1·노드 stride 0x90 전부 동일 확인 ⟹ NT_SIZE 무변경.
// ⚠0.5.3에서 2인자 `__rust_alloc(size, align)` 심은 **소멸**(전 호출부 인라인) ⟹ 내부 힙 헬퍼를 직접 호출한다.
//   ~~후보 0xbb2bd0(align8 고정 심·OOM 시 abort)~~ → **0x28f7df0 채택**(0.5.2 0x25d9640과 명령 동일·OOM 시 0 반환 보존
//   ·병행 ai_adjust 세션과 동일 값 = 모드 간 일원화). 계약은 위 `AllocFn` 주석 참조.
const ALLOC_RVA: usize  = 0x2ab4010; // 0.5.6(구0.5.5=0x2a9bf30, skel/마스크 UNIQUE·BYTE=SAME). // 0.5.5(구0.5.4=0x29bb920, skel/마스크 유일). // 0.5.4(구0.5.3=0x28f7df0). 본문 100% 동일(IAT rip-rel만 이동)·콜러 32890→33708. // 0.5.3 힙 alloc 헬퍼(구0.5.2=0x25d9640에 대응. 구 __rust_alloc 심 0x25c4d30은 0.5.3에 없음).
const DEALLOC_RVA: usize = 0x1000; // 0.5.3(구0.5.2=0x25c4d90). __rust_dealloc(ptr,size,align) 형태 유일. 현재 미사용.
const NT_SIZE: usize = 0x90;

// 타깃 경로 + 내 수정본 .ui 텍스트(컴파일타임 임베드).
const PATH_PI: &[u8]   = b"asset/base/ui/layout/ingame_component/player_info";
const PATH_WIDE: &[u8] = b"asset/base/ui/layout/ingame_component/wide_player_info";
const PATH_STRAT: &[u8] = b"asset/base/ui/layout/strategy";
const PATH_TRAIN: &[u8] = b"asset/base/ui/layout/training"; // ★조합테스트(공식 신기능) 개인전술 탭
const UI_PI: &str   = include_str!("../ui/layout/ingame_component/player_info.ui");
const UI_WIDE: &str = include_str!("../ui/layout/ingame_component/wide_player_info.ui");

// 개인전술 모드소유 아이템 드롭다운. 베이스 item0/1/2=315/565/815(간격250)·헤더 300/550/800.
// ★2026-07-08 리워크: slot1~3(모드템 선택)을 위해 item0m/1m/2m 를 네이티브 item0/1/2 위(315/565/815)에 오버레이.
//   네이티브는 클릭이 모델(personal_tactics 바닐라)에만 커밋돼 모드템(idx7+) 불가(ghidra-re 확증) → 모드소유
//   드롭다운(item3처럼 +0x1788 직접 커밋)으로 대체. append=나중child=상단렌더+상단히트 → 클릭 캡처.
// item3 x=1035(간격220, 오른쪽 여백 축소)·col_item4 x=1020(헤더-드롭다운 −15). ai_adjust AI버튼은 x1300(aiadj_p*.ui).
fn mod_dd(id: &str, x: u32) -> String {
    format!("{}:dropdown {{\n@\"asset/base/style/main#dropdown\";\nx: {}px;\nwidth: 220px;\nheight: 40px;\ny: 5px;\nmax_items_height: 280;\ntext: {{ size: 15; }}\nitem_text: {{ size: 15; }}\ntext_layout: {{ x: 10px; y: 6px; width: 100%; height: 28px; }}\nitem_layout: {{ height: 36px; width: 216px; x: 10px; }}\nitem_text_layout: {{ y: 4px; width: 216px; height: 28px; }}\n}}", id, x)
}
// ★조합테스트 개인전술 = item_tactics 단독 관리(07-21 유저확정).
//   네이티브 item0/1/2는 모드템을 담을 수 없다(클릭이 게임 모델에만 커밋 — ghidra-re 확정).
//   → 4칸 전부를 모드소유 드롭다운으로 대체하고 네이티브는 lib.rs 에서 visible=false 로 숨긴다.
//   ⟹ 좌표는 이 선언값이 최종 = 런타임 좌표강제 불필요(히트박스 깨짐·떨림 위험 원천 제거).
//   ⚠ append_child 조각은 id 앞에 '#'을 붙이면 파서 실패(mod_dd/COL4와 동일 규칙).
fn train_dd_frag(id: &str, x: u32, w: u32, fs: u32) -> String {
    // 텍스트 영역 = 폭 − 26(우측 화살표 아이콘 자리). 바닐라 옵션 "선수에게 맡김"(6자+공백)이
    //   개행되지 않는 것이 기준 — 폰트 12에서 약 78px 이므로 87px 확보면 한 줄에 들어간다.
    //   (모드템 이름은 길어서 어차피 잘리거나 줄바꿈됨 — 유저 확정으로 그대로 둔다.)
    let tw = w.saturating_sub(26);
    format!("{}:dropdown {{ @\"asset/base/style/main#dropdown\"; x: {}px; y: 10px; width: {}px; height: 36px; text: {{ size: {}; }} item_text: {{ size: {}; }} text_layout: {{ x: 8px; y: 5px; width: {}px; height: 26px; }} item_layout: {{ x: 2px; y: 0px; width: {}px; height: 36px; }} item_text_layout: {{ x: 8px; y: 5px; width: {}px; height: 26px; }} max_items_height: 252; }}",
        id, x, w, fs, fs, tw, w.saturating_sub(4), tw)
}
// 슬롯 id(모드 접두 — 네이티브 #item3:image 10개와 충돌 회피). 인덱스 = SEL 슬롯 0~3.
pub const CT_DD_IDS: [&str; 4] = ["it4_s0", "it4_s1", "it4_s2", "it4_slot3"];
// ★4칸 간격 수정(2026-07-22 유저 제보 "살짝 겹쳐져있어"): 구 [142,257,372,487] 폭113 = **간격 2px**이라
//   스타일 rounding(6)과 테두리 때문에 이웃 박스가 붙어 겹쳐 보였다. 폭을 3px씩 깎아 간격 6px 확보.
//   전체 footprint(142~600)는 구값과 동일 = 오른쪽 여백 변화 없음.
//   근거(0.5.1 번들 training.ui 실측): 행 폭 608.5 / 네이티브 item0·1·2 = x146·296·446, 폭 140(끝 586).
//   ⚠폭 하한: text_layout = 폭−26 이고 바닐라 옵션 "선수에게 맡김"이 폰트12에서 ≈78px ⟹ 폭 ≥104 필요.
//     110 → 텍스트칸 84px = 한 줄 유지(구 87에서 3px 감소, 개행 없음).
const CT_X4: [u32; 4] = [142, 258, 374, 490]; // 4칸 압축(폭 110·간격 6px, 142~600 구간)
const CT_X3: [u32; 3] = [146, 296, 446];      // 3칸 = 바닐라 좌표(폭 140)
const TRAIN_ROWS: [(&str, &str); 10] = [
    ("blue0", "asset/base/ui/icons/top"),    ("blue1", "asset/base/ui/icons/jungle"),
    ("blue2", "asset/base/ui/icons/mid"),    ("blue3", "asset/base/ui/icons/bottom"),
    ("blue4", "asset/base/ui/icons/support"),
    ("red0",  "asset/base/ui/icons/top"),    ("red1",  "asset/base/ui/icons/jungle"),
    ("red2",  "asset/base/ui/icons/mid"),    ("red3",  "asset/base/ui/icons/bottom"),
    ("red4",  "asset/base/ui/icons/support"),
];
// 10행(블루5+레드5)에 모드소유 드롭다운 append. 3칸/4칸 모두 우리가 관리(네이티브는 lib.rs에서 숨김).
// ⛔이 행에 replace_children 금지 — 타 모드(comptest_unlock 등)가 append한 노드를 날린다(멀티모드 공존 §3).
//   (아이콘 컬럼은 구 행-재현 방식의 잔재로 현재 미사용.)
unsafe fn inject_training(r: usize) -> bool {
    if r <= 0x10000 { return false; }
    if find_tmpl(r, b"it4_s0", 0) != 0 { TR_IDEM.fetch_add(1, Ordering::Relaxed); return true; } // 멱등
    let mode4 = MODE4.load(Ordering::Relaxed);
    // 4칸=압축(폭113·폰트12) / 3칸=바닐라 동일(폭140·폰트13)
    let (n_slots, w, fs) = if mode4 { (4usize, 110u32, 12u32) } else { (3usize, 140u32, 13u32) };
    let mut n = 0;
    for (rid, _icon) in TRAIN_ROWS.iter() {
        if find_tmpl(r, rid.as_bytes(), 0) == 0 { continue; }
        TR_ROWS.fetch_add(1, Ordering::Relaxed);
        for si in 0..n_slots {
            let x = if mode4 { CT_X4[si] } else { CT_X3[si] };
            if append_child(r, rid.as_bytes(), &train_dd_frag(CT_DD_IDS[si], x, w, fs)) { n += 1; }
        }
    }
    TR_REPL.fetch_add(n as usize, Ordering::Relaxed);
    logln(&format!("injected training: {}칸 × 행 = {} (mode4={})", n_slots, n, mode4));
    n > 0
}
const COL4: &str = "col_item4:label {\n@\"asset/base/style/main#label\";\nx: 1050px;\nwidth: 250px;\nheight: 30px;\nalign_x: Center;\nalign_y: Center;\nsize: 16;\ntext: \"4번째 아이템\";\n}";

type LoaderFn = extern "win64" fn(usize, *const u8, usize) -> usize;
type ParserFn = extern "win64" fn(*mut u8, *const u8, usize);
// ★0.5.3: 2인자 `__rust_alloc(size, align)` 심이 소멸(전 호출부 인라인)해서 **내부 힙 헬퍼를 직접** 부른다.
//   계약 = (rcx=무시, rdx=flags(0), r8=size) -> rax=ptr / 실패 시 **0 반환**(abort 아님 = 아래 null 체크가 살아있다).
//   구 exe 대응 함수(0.5.2 0x25d9640)와 명령 단위로 동일하고, 0.5.2 __rust_alloc의 align≤0x10 경로가
//   바로 이 헬퍼로 tail-jmp 했으므로 결과 블록도 동일(→ __rust_dealloc(align 8)·게임 파서 free 경로와 정합).
//   ⚠align 인자를 받지 않는다 — align>0x10이 필요해지면 이 경로를 쓰면 안 된다(현 용도는 align 8).
type AllocFn  = extern "win64" fn(usize, usize, usize) -> usize;

static BASE: AtomicUsize = AtomicUsize::new(0);
static TRAMP: AtomicUsize = AtomicUsize::new(0);
static LAST_TRAIN: AtomicUsize = AtomicUsize::new(0); // 조합테스트 재주입 가드
// ★진단(07-21): 후킹한 두 copy가 실제로 어떤 asset 경로를 보는지 실측 → training.ui가 제3의
//   모노모픽 copy를 타는지 확정. PATH_PROBE=false 로 끄면 오버헤드 0.
const PATH_PROBE: bool = false; // 프로덕션 OFF(경로 문자열 수집=매 로드 alloc+lock)
pub static LOADER_CALLS: AtomicUsize = AtomicUsize::new(0);
pub static TRAIN_SEEN: AtomicUsize = AtomicUsize::new(0);
pub static SEEN_PATHS: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
// ★단계별 실패지점 계측
pub static TR_BRANCH: AtomicUsize = AtomicUsize::new(0);   // loader_body training 분기 진입
pub static TR_MODE4: AtomicUsize = AtomicUsize::new(0);    // 그때 mode4=1
pub static TR_SKIP_SAME: AtomicUsize = AtomicUsize::new(0);// r == LAST_TRAIN 으로 skip
pub static TR_IDEM: AtomicUsize = AtomicUsize::new(0);     // item3 이미 존재(멱등 skip)
pub static TR_ROWS: AtomicUsize = AtomicUsize::new(0);     // find_tmpl 로 찾은 행 수
pub static TR_REPL: AtomicUsize = AtomicUsize::new(0);     // replace_children 성공 수
static TRAMP2: AtomicUsize = AtomicUsize::new(0); // 0xeb17d0(strategy copy) 세컨드 훅 트램폴린
static INSTALLED: AtomicBool = AtomicBool::new(false);
// ★4-아이템 모드 게이트: false면 UI 수정 안 함(3-모드=바닐라). lib.rs가 cfg로 세팅.
pub static MODE4: AtomicBool = AtomicBool::new(true);
// ★경기중 player_info 슬롯 UI 교체 게이트: 4번째 구매 연결 후 켬(미연결시 빈 슬롯 방지). 기본 off.
pub static IN_MATCH_UI: AtomicBool = AtomicBool::new(true); // ★ON: #slot3 노드 주입(오버레이 fill 대상). bound패치는 계속 off라 OOB 크래시 없음.
// ★전술화면 드롭다운 오버레이(item0m/1m/2m) 주입 게이트 = mode 3·4 공통(3칸 지정용). item3(4번째)만 MODE4 추가.
pub static STRAT_INJECT: AtomicBool = AtomicBool::new(true);
// 교체 멱등(같은 템플릿 ptr 재교체 방지, reload=새 ptr시 재교체).
static LAST_PI: AtomicUsize = AtomicUsize::new(0);
static LAST_WIDE: AtomicUsize = AtomicUsize::new(0);
static LAST_STRAT: AtomicUsize = AtomicUsize::new(0);

pub static DBG: AtomicBool = AtomicBool::new(false);
static LOGP: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
pub fn set_log(p: String) { *LOGP.lock().unwrap_or_else(|e| e.into_inner()) = p; }
fn logln(s: &str) {
    if !DBG.load(Ordering::Relaxed) { return; }
    let p = LOGP.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if p.is_empty() { return; }
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&p) {
        let _ = writeln!(f, "{}", s);
    }
}

type BOOL = i32;
#[link(name = "kernel32")]
extern "system" {
    fn VirtualAlloc(addr: usize, size: usize, typ: u32, protect: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, new: u32, old: *mut u32) -> BOOL;
    fn FlushInstructionCache(proc: usize, addr: usize, size: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
    fn GetModuleHandleW(name: *const u16) -> usize;
}
const MEM_CR: u32 = 0x1000 | 0x2000;
const RWX: u32 = 0x40;

// 공통 로더 detour 본문 — asset-get의 두 모노모픽 copy(0x40f3d0=main계열, 0xeb17d0=strategy계열)가 공유.
//   r = 조회 결과(UI 템플릿 ptr). fan-in 100+이므로 len 선체크로 경량 유지.
fn loader_body(path: *const u8, len: usize, r: usize) {
    if path.is_null() || r <= 0x10000 || len >= 200 { return; }
    let s = unsafe { core::slice::from_raw_parts(path, len) };
    if PATH_PROBE {
        LOADER_CALLS.fetch_add(1, Ordering::Relaxed);
        if s.windows(8).any(|w| w == b"training") { TRAIN_SEEN.fetch_add(1, Ordering::Relaxed); }
        if let Ok(txt) = core::str::from_utf8(s) {
            let mut g = SEEN_PATHS.lock().unwrap_or_else(|e| e.into_inner());
            if g.len() < 120 && !g.iter().any(|p| p == txt) { g.push(txt.to_string()); }
        }
    }
    let mode4 = MODE4.load(Ordering::Relaxed);
    // 경기중 player_info(4슬롯 교체)=MODE4 전용. IN_MATCH_UI 게이트(4번째 구매 연결 후 켬 — 미연결시 빈칸 방지).
    //   mode=3=바닐라 3슬롯 유지(교체 안 함). 전술화면 오버레이(STRAT)는 mode 3·4 공통.
    if mode4 && IN_MATCH_UI.load(Ordering::Relaxed) && s == PATH_PI && r != LAST_PI.load(Ordering::Relaxed) {
        let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { replace_children(r, UI_PI) })).unwrap_or(false);
        if ok { LAST_PI.store(r, Ordering::Relaxed); logln(&format!("replaced player_info r={:#x}", r)); }
    } else if mode4 && IN_MATCH_UI.load(Ordering::Relaxed) && s == PATH_WIDE && r != LAST_WIDE.load(Ordering::Relaxed) {
        let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { replace_children(r, UI_WIDE) })).unwrap_or(false);
        if ok { LAST_WIDE.store(r, Ordering::Relaxed); logln(&format!("replaced wide r={:#x}", r)); }
    } else if STRAT_INJECT.load(Ordering::Relaxed) && s == PATH_STRAT && r != LAST_STRAT.load(Ordering::Relaxed) {
        let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { inject_strategy(r) })).unwrap_or(false);
        if ok { LAST_STRAT.store(r, Ordering::Relaxed); logln(&format!("injected strategy r={:#x}", r)); }
    } else if s == PATH_TRAIN {
        TR_BRANCH.fetch_add(1, Ordering::Relaxed);
        if mode4 { TR_MODE4.fetch_add(1, Ordering::Relaxed); }
        if r == LAST_TRAIN.load(Ordering::Relaxed) { TR_SKIP_SAME.fetch_add(1, Ordering::Relaxed); }
        // ★07-21: 조합테스트 개인전술은 3칸/4칸 모두 우리가 관리 → mode4 게이트 제거.
        //   (⚠게이트를 남기면 3칸에서 네이티브만 숨겨지고 대체 드롭다운이 안 붙어 "칸이 전부 사라짐".)
        if r != LAST_TRAIN.load(Ordering::Relaxed) {
        let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { inject_training(r) })).unwrap_or(false);
        if ok { LAST_TRAIN.store(r, Ordering::Relaxed); logln(&format!("injected training r={:#x}", r)); }
        }
    }
}
// copy #1: 0x40f3d0 (main/player_info/wide/title 등)
extern "win64" fn detour(am: usize, path: *const u8, len: usize) -> usize {
    let t = TRAMP.load(Ordering::Relaxed);
    if t == 0 { return 0; }
    let r = unsafe { core::mem::transmute::<usize, LoaderFn>(t)(am, path, len) };
    loader_body(path, len, r);
    r
}
// copy #2: 0xeb17d0 (strategy 화면 전용 copy — 0.5.1 모노모픽 분화, ghidra-re 확정)
extern "win64" fn detour2(am: usize, path: *const u8, len: usize) -> usize {
    let t = TRAMP2.load(Ordering::Relaxed);
    if t == 0 { return 0; }
    let r = unsafe { core::mem::transmute::<usize, LoaderFn>(t)(am, path, len) };
    loader_body(path, len, r);
    r
}

// 로드된 템플릿 r(루트)의 자식 배열을 내 .ui 파싱본의 자식으로 교체.
unsafe fn replace_children(r: usize, text: &str) -> bool {
    if r <= 0x10000 { return false; }
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 { return false; }
    let parser: ParserFn = core::mem::transmute(base + PARSER_RVA);
    let mut out = [0u8; 0x400];
    parser(out.as_mut_ptr(), text.as_ptr(), text.len());
    let my = out.as_ptr().add(0x10) as usize; // 내 root NodeTemplate
    if *(my as *const usize) == usize::MAX { logln("parse ERR"); return false; }
    // 내 root의 자식 {cap@+0x48, ptr@+0x50, len@+0x58} → 힙(파서 할당). r의 자식으로 복사.
    let mcap = *((my + 0x48) as *const usize);
    let mptr = *((my + 0x50) as *const usize);
    let mlen = *((my + 0x58) as *const usize);
    if mptr <= 0x10000 || mlen == 0 || mlen > 2000 { logln("bad my child"); return false; }
    // r 자식 교체(옛 배열 leak=무해). ptr→cap→len 순(중간read 안전).
    *((r + 0x50) as *mut usize) = mptr;
    *((r + 0x48) as *mut usize) = mcap;
    *((r + 0x58) as *mut usize) = mlen;
    true
}

// 템플릿 트리서 id로 컨테이너 찾기.
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
// container_id 컨테이너 끝에 조각(fragment) 노드 append.
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
    let new_ptr = galloc(0, 0, new_n * NT_SIZE); // 0.5.3: (무시, flags=0, size) — align 8 블록
    if new_ptr == 0 { return false; }
    if ptr != 0 && len != 0 { core::ptr::copy_nonoverlapping(ptr as *const u8, new_ptr as *mut u8, len * NT_SIZE); }
    core::ptr::copy_nonoverlapping(my as *const u8, (new_ptr + len * NT_SIZE) as *mut u8, NT_SIZE);
    *((target + 0x50) as *mut usize) = new_ptr;
    *((target + 0x48) as *mut usize) = new_n;
    *((target + 0x58) as *mut usize) = new_n;
    true
}
// strategy: 개인전술 헤더+각 행에 4번째 컬럼 append. 이미 item3 있으면 skip(멱등).
unsafe fn inject_strategy(r: usize) -> bool {
    if r <= 0x10000 { return false; }
    let mode4 = MODE4.load(Ordering::Relaxed);
    // 멱등 마커(마지막 append 노드): mode4=item3, mode3=item2m.
    let marker: &[u8] = if mode4 { b"item3" } else { b"item2m" };
    if find_tmpl(r, marker, 0) != 0 { return true; }
    if mode4 { let _ = append_child(r, b"personal_header", COL4); } // 4번째 컬럼 헤더=mode4만
    // 모드소유 드롭다운: item0m/1m/2m(mode 3·4 공통, 네이티브 위 오버레이) + item3(4번째=mode4만). 순서=마지막이 상단.
    let mut dds = vec![mod_dd("item0m", 315), mod_dd("item1m", 565), mod_dd("item2m", 815)];
    if mode4 { dds.push(mod_dd("item3", 1065)); }
    let mut any = false;
    for i in 0..5u32 {
        let id = format!("row{}", i);
        for dd in &dds {
            if append_child(r, id.as_bytes(), dd.as_str()) { any = true; }
        }
    }
    any
}

// 체이닝 install: 진입부 12바이트 저장 → tramp=[saved][jmp fn+0xc] → patch entry = jmp detour.
// 한 copy(rva)에 체이닝 detour 설치 + 트램폴린을 tramp_slot에 저장.
unsafe fn install_one(base: usize, rva: usize, tramp_slot: &AtomicUsize, detour_addr: usize) -> bool {
    let fn_addr = base + rva;
    // 이미 내 훅이면(재설치) skip — 진입부가 우리 detour로 가는 movabs(0x48 0xb8 + detour주소)면.
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    if cur[0] == 0x48 && cur[1] == 0xb8 {
        let tgt = usize::from_le_bytes(cur[2..10].try_into().unwrap());
        if tgt == detour_addr { logln(&format!("already my hook fn={:#x}", fn_addr)); return true; }
    }
    let stub = VirtualAlloc(0, 64, MEM_CR, RWX);
    if stub == 0 { return false; }
    // tramp = [현재 진입부 12바이트(원본프롤로그 or 다른모드 jmp)] + [movabs rax, fn+0xc; jmp rax]
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&cur);
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&(fn_addr + 0xc).to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    tramp_slot.store(stub, Ordering::Relaxed);
    // patch entry = movabs rax, detour; jmp rax
    let mut patch = [0u8; 12];
    patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&detour_addr.to_le_bytes()); patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, RWX, &mut old) == 0 { return false; }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    logln(&format!("chained install OK fn={:#x} tramp={:#x}", fn_addr, stub));
    true
}

pub unsafe fn install() -> bool {
    if INSTALLED.swap(true, Ordering::Relaxed) { return true; }
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { INSTALLED.store(false, Ordering::Relaxed); return false; }
    BASE.store(base, Ordering::Relaxed);
    // 0.5.1: copy#1(0x40f3d0)=main/player_info/wide, copy#2(0xeb17d0)=strategy 전용(모노모픽 분화).
    // 0.5.2: 대상 4경로가 전부 같은 copy(0x5ac950)로 수렴 → 세컨드 훅 스킵(동일 함수 이중 체인=본문 2회 실행/자기체인 위험).
    let a = install_one(base, LOADER_RVA, &TRAMP, detour as usize);
    let b = if STRAT_LOADER_RVA != LOADER_RVA {
        install_one(base, STRAT_LOADER_RVA, &TRAMP2, detour2 as usize)
    } else { logln("STRAT_LOADER == LOADER (0.5.2 copy 병합) → 세컨드 훅 스킵"); false };
    if !a && !b { INSTALLED.store(false, Ordering::Relaxed); return false; }
    a || b
}
