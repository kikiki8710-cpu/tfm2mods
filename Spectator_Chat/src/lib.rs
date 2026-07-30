//! Spectator_Chat — 관전 가짜 관중 채팅 (트위치st), 커서(played_tick) 종속.
//!   + HP 추적 생존 감지 (부하 테스트 겸).

use mod_api::*;
use std::collections::HashMap;
use std::fs; use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};

#[path = "C:/tfm2mods/ui_kit/ui_kit.rs"]
mod ui_kit;
use ui_kit::*;

// 즉시보기용: game_view 타입(&Vec<Arc<GameFrameData>>)을 anchor 로 raw 포인터를 &T 로 캐스팅.
//   game_view 가 None 이어도 타입은 추론됨. scene+984(client.events) 를 리플레이와 동일 타입으로.
unsafe fn cast_as<T>(_anchor: &Option<&T>, ptr: usize) -> &'static T { &*(ptr as *const T) }

type HMODULE=isize; type DWORD=u32; type BOOL=i32;
#[link(name="kernel32")]
extern "system"{ fn GetModuleHandleExW(f:DWORD,m:*const u16,h:*mut HMODULE)->BOOL;
    fn GetModuleFileNameW(h:HMODULE,fname:*mut u16,n:DWORD)->DWORD; }
fn dll_dir()->Option<PathBuf>{unsafe{
    let a=dll_dir as *const() as usize; let mut h:HMODULE=0;
    if GetModuleHandleExW(0x4|0x2,a as *const u16,&mut h)==0||h==0{return None}
    let mut b=[0u16;4096]; let l=GetModuleFileNameW(h,b.as_mut_ptr(),b.len() as DWORD);
    if l==0{return None} PathBuf::from(String::from_utf16_lossy(&b[..l as usize])).parent().map(|p|p.to_path_buf())
}}

const MOD_ID: &str = "Spectator_Chat";

// ── 채팅 줄 슬롯 ──────────────────────────────────────────────────────────
// .ui 에 #line0..#line{LINE_SLOTS-1} 이 미리 선언돼 있고(전부 26px + spacing 2px),
// 실제로 몇 줄을 보여줄지는 창 높이에 따라 매 프레임 정한다(세로로 늘리면 더 많이 표시).
// 남는 슬롯은 visible=false 로 숨긴다(안 숨기면 창 밖으로 삐져나옴).
const LINE_SLOTS: usize = 20;
const LINE_H: f32 = 26.0;
const LINE_GAP: f32 = 2.0;
const CHAT_PAD_Y: f32 = 16.0;   // #chat_lines margin top(8) + bottom(8)

// 창 초기 크기 = 최소 크기 (.ui 의 #spectator_chat width/height 와 반드시 일치시킬 것)
const WIN_MIN_W: f32 = 414.0;
const WIN_MIN_H: f32 = 214.0;
// 화면(UI 가상 좌표계) 크기 — 창이 이 밖으로 못 나가게 클램프
const SCREEN_W: f32 = 1920.0;
const SCREEN_H: f32 = 1080.0;

/// 창 높이 h 에서 표시 가능한 줄 수. n줄 총높이 = n*LINE_H + (n-1)*LINE_GAP <= h - CHAT_PAD_Y
fn lines_for_height(h: f32) -> usize {
    if !h.is_finite() { return 1; }
    let avail = h - CHAT_PAD_Y + LINE_GAP;          // n*(LINE_H+LINE_GAP) <= avail
    let n = (avail / (LINE_H + LINE_GAP)).floor();
    if !n.is_finite() || n < 1.0 { return 1; }
    (n as usize).min(LINE_SLOTS)
}

/// 지금 보여줄 줄 수(창 높이에서 계산해 매 프레임 갱신). 초기값 = 최소 크기 기준.
static VIS_LINES: AtomicUsize = AtomicUsize::new(6);
fn vis_lines() -> usize { VIS_LINES.load(Ordering::Relaxed).clamp(1, LINE_SLOTS) }
const WINDOW_TICKS: i64 = 600;
const SCAN_PER_FRAME: usize = 3000;   // 청크 상한(주 제한은 1.5ms 시간예산). 가벼우면 이만큼, 무거우면 시간예산서 끊김
const SPREAD: i64 = 15;

static CHAT_VISIBLE: AtomicBool = AtomicBool::new(true);  // chat_btn 클릭 시 토글
static FH: AtomicUsize = AtomicUsize::new(usize::MAX);    // filter_handler 재등록 추적

// 채팅 패널 드래그 이동 = 엔진 내장 draggable_popup Runner가 처리(아래 post_update에서
// runner+0x1c0 충전). 구 Win32 방식(ui_kit::DraggableWindow)은 더 이상 사용 안 함.

// 진단 마커(sc_trace.txt). SC_TRACE=false 면 no-op(프로덕션). 디버그 시 true.
const SC_TRACE: bool = false;
fn mark(s: &str) {
    if !SC_TRACE { return; }
    use std::io::Write;
    let p = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\Spectator_Chat\sc_trace.txt";
    if let Ok(mut f) = std::fs::File::create(p) { let _ = write!(f, "{}", s); let _ = f.flush(); }
}

// ── 진단(2026-07-30 "응원만 계속" 증상 추적에 사용). 프로덕션 = false. ──
// mark()와 달리 append + 프레임 게이트 → 시계열이 남는다. 파일 = <mods\Spectator_Chat>\sc_diag.txt
// 켜면 매 30프레임 스캔상태 1줄 + REBUILD/DISPLAY/PANIC 이 기록된다(증상 재발 시 이것부터 켤 것).
const SC_DIAG: bool = false;
static DIAG_FRAME: AtomicUsize = AtomicUsize::new(0);
fn diag(s: &str) {
    if !SC_DIAG { return; }
    use std::io::Write;
    let Some(d) = dll_dir() else { return; };
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(d.join("sc_diag.txt")) {
        let _ = writeln!(f, "{}", s);
    }
}
/// 스캔 루프 등에서 패닉이 나면 그 프레임이 조용히 죽어 SCAN_DONE 이 영영 안 선다.
/// 기존 훅을 체이닝해서(다른 모드 훅 보존) 패닉 내용만 파일로 남긴다.
fn install_panic_hook() {
    static ONCE: AtomicBool = AtomicBool::new(false);
    if !SC_DIAG || ONCE.swap(true, Ordering::Relaxed) { return; }
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        diag(&format!("!! PANIC: {}", info));
        prev(info);
    }));
}

fn fill_lines(root: &mut Node, msgs: &[String]) {
    let cap = vis_lines();
    for i in 0..LINE_SLOTS {
        // 창 높이가 허용하는 만큼만 노출. 남는 슬롯은 숨긴다(창 밖 삐져나옴 방지).
        set_visible_by_id(root, &format!("line{i}"), i < cap);
        if i >= cap { continue; }
        let msg = msgs.get(i).map(|s| s.as_str()).unwrap_or("");
        let (nick_part, body) = match msg.find(": ") {
            Some(p) => (&msg[..p], &msg[p+2..]),
            None => ("", msg),
        };
        let nick_txt = if nick_part.is_empty() { String::new() } else { format!("{}: ", nick_part) };
        if let Some(n) = find_mut(root, &format!("line{i}_nick")) { label_set(n, &nick_txt); }
        if let Some(n) = find_mut(root, &format!("line{i}_text")) { label_set(n, body); }
    }
}

// ── 팝업 노드 레이아웃 조작 ─────────────────────────────────────────────────
// Node 레이아웃 = 4상태 블록(normal/hover/press/disabled). 읽기는 normal, 쓰기는 4블록 전부.
// 상수·헬퍼는 ui_kit 정본(NODE_LF_*, node_layout_read/node_layout_write_all)을 쓴다.

/// 창이 화면 밖으로 나가지 못하게 x/y 를 매 프레임 보정.
/// 엔진 on_mouse_move(0.5.0_3=0x100a8c0, 구0.5.0_2=0x15e6190) 는 x/y 를 clamp(0,1820)/clamp(0,1030) 으로만 막는다(창 크기 무시) →
/// 오른쪽·아래 경계는 우리가 창 실제 크기로 다시 클램프해야 한다.
/// (리사이즈로 left/top 을 과하게 끌면 x/y 가 음수로도 나가므로 하한 0 도 함께 건다.)
fn clamp_popup_into_screen(n: &mut Node) {
    let (w, h) = (node_layout_read(n, NODE_LF_W), node_layout_read(n, NODE_LF_H));
    let (x, y) = (node_layout_read(n, NODE_LF_X), node_layout_read(n, NODE_LF_Y));
    if !(w.is_finite() && h.is_finite() && x.is_finite() && y.is_finite()) { return; }
    if w <= 0.0 || h <= 0.0 { return; }
    let nx = x.clamp(0.0, (SCREEN_W - w).max(0.0));
    let ny = y.clamp(0.0, (SCREEN_H - h).max(0.0));
    if nx != x { node_layout_write_all(n, NODE_LF_X, nx); }
    if ny != y { node_layout_write_all(n, NODE_LF_Y, ny); }
}

struct Indexed { tick: i64, text: String }
static INDEX: Mutex<Vec<Indexed>> = Mutex::new(Vec::new());
static ANCHORS: Mutex<Vec<(usize, i64)>> = Mutex::new(Vec::new());
static PENDING: Mutex<Vec<(usize, String, String, usize)>> = Mutex::new(Vec::new());
static CHAMP2NAME: Mutex<Option<HashMap<String,String>>> = Mutex::new(None);
// 생존 감지(시간기반): 위험진입 (frame, id) 와 죽음 (frame, id) 모았다가 스캔후 매칭
static DANGER: Mutex<Vec<(usize, i64)>> = Mutex::new(Vec::new());
static DEATHS: Mutex<Vec<(usize, i64)>> = Mutex::new(Vec::new());
// entity_id -> 선수명, champion키 -> entity_id
static ID2NAME: Mutex<Option<HashMap<i64,String>>> = Mutex::new(None);
static ID2TEAM: Mutex<Option<HashMap<i64,i64>>> = Mutex::new(None);   // entity_id -> team(0/1)
static CHAMP2ID: Mutex<Option<HashMap<String,i64>>> = Mutex::new(None);
// HP 추적 상태: id -> (max_hp, last_hp, in_danger)
static HPSTATE: Mutex<Option<HashMap<i64,(i64,i64,bool)>>> = Mutex::new(None);
// 스코어 추이: (tick, blue, red) — 잡담 단계 판단용
static SCORELINE: Mutex<Vec<(i64,i64,i64)>> = Mutex::new(Vec::new());
// 새 이벤트 수집 버퍼 (스캔 중 모았다가 스캔 완료 후 처리)
static CCEV: Mutex<Vec<(usize, i64, u8)>> = Mutex::new(Vec::new());      // (frame_idx, entity_id, cc종류 0=스턴 1=에어본 2=속박)
static ULTEV: Mutex<Vec<(usize, i64)>> = Mutex::new(Vec::new());        // (frame_idx, caster_id)
static TOWERLINE: Mutex<Vec<(i64, i64, i64)>> = Mutex::new(Vec::new()); // (tick, tower_blue, tower_red)
static STATLINE: Mutex<Vec<(i64, i64, i64, i64, i64)>> = Mutex::new(Vec::new()); // (tick, gold_b, gold_r, deal_b, deal_r) 누적 스냅샷
// 로딩 중 응원 채팅 버퍼 (실시간 한 줄씩 추가)
static CHEER: Mutex<Vec<String>> = Mutex::new(Vec::new());

static SCAN_TOTAL: AtomicUsize = AtomicUsize::new(0);
static SCAN_POS: AtomicUsize = AtomicUsize::new(0);
static SCAN_DONE: AtomicBool = AtomicBool::new(false);
static CHEER_MERGED: AtomicBool = AtomicBool::new(false);  // 로딩응원→인덱스 통합 1회 처리
// 증분 스캔용: 마지막으로 채팅 인덱스를 재생성한 시점의 SCAN_POS (usize::MAX = 아직 없음).
static REBUILD_AT: AtomicUsize = AtomicUsize::new(usize::MAX);
// 스코어 중복 방지값. 구코드는 스캔 청크마다 초기화돼 청크 경계에서 같은 스코어가 또 나왔다.
static SCORE_LAST: Mutex<(i64,i64)> = Mutex::new((-1,-1));
// 채팅 문구 생성은 전부 후처리(rebuild)에서 한다 → 스캔 단계는 "언제 무슨 일이 있었나"만 모은다.
// (스캔 중에 index 로 바로 문구를 밀어넣으면 증분 재생성 때 중복이 생긴다.)
static SCORELINE_SCORED: Mutex<Vec<(i64,i64,i64)>> = Mutex::new(Vec::new());  // (tick, 블루, 레드) — 득점 순간만
static SERPENLINE: Mutex<Vec<(i64,i64)>> = Mutex::new(Vec::new());            // (tick, 획득팀)
static DISP_PHASE: AtomicUsize = AtomicUsize::new(0);
static RNG: AtomicUsize = AtomicUsize::new(0x12345);

fn rng() -> usize {
    let mut x = RNG.load(Ordering::Relaxed);
    x ^= x << 13; x ^= x >> 17; x ^= x << 5;
    RNG.store(x, Ordering::Relaxed); x
}

// ───────── 외부 문구 파일 시스템 ─────────
// chat_lines.txt (dll 옆)에서 섹션별 문구 로드. 없으면 내장 기본값.
static PHRASES: Mutex<Option<HashMap<String, Vec<String>>>> = Mutex::new(None);
// 섹션별 최근 뽑은 문구 인덱스들 (연속 중복 방지, 최근 2개)
static LAST_PICK: Mutex<Option<HashMap<String, Vec<usize>>>> = Mutex::new(None);

fn load_phrases() {
    let mut text = String::new();
    if let Some(d) = dll_dir() {
        if let Ok(s) = fs::read_to_string(d.join("chat_lines.txt")) { text = s; }
    }
    if text.trim().is_empty() { text = default_phrases(); }   // 폴백
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut cur = String::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') { continue; }
        if t.starts_with('[') && t.ends_with(']') {
            cur = t[1..t.len()-1].trim().to_string();
            map.entry(cur.clone()).or_default();
        } else if !cur.is_empty() {
            map.entry(cur.clone()).or_default().push(t.to_string());
        }
    }
    *PHRASES.lock().unwrap_or_else(|e| e.into_inner()) = Some(map);
}

// 섹션에서 랜덤 문구 뽑기 (없으면 빈 문자열)
fn line_of(section: &str) -> String {
    let g = PHRASES.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(m) = g.as_ref() {
        if let Some(v) = m.get(section) {
            let n = v.len();
            if n == 0 { return String::new(); }
            if n == 1 { return v[0].clone(); }
            // 최근 뽑은 인덱스들을 피해서 뽑기 (연속 중복 방지)
            let mut lp = LAST_PICK.lock().unwrap_or_else(|e| e.into_inner());
            let map = lp.get_or_insert_with(HashMap::new);
            let recent = map.entry(section.to_string()).or_insert_with(Vec::new);
            // 피할 개수: 문구가 적으면 줄임 (최대 2개, 단 n-1 이하)
            let avoid = recent.len().min(2).min(n - 1);
            let mut idx = rng() % n;
            // 최근 avoid개에 들면 다시 뽑기 (최대 8회 시도)
            let mut tries = 0;
            while tries < 8 && recent.iter().rev().take(avoid).any(|&r| r == idx) {
                idx = rng() % n;
                tries += 1;
            }
            recent.push(idx);
            if recent.len() > 2 { recent.remove(0); }  // 최근 2개만 유지
            return v[idx].clone();
        }
    }
    String::new()
}

// 파일 없을 때 폴백(이자 chat_lines.txt 기본 템플릿)
fn default_phrases() -> String {
r#"# Spectator_Chat 문구 파일. # 는 주석. [섹션] 아래 한 줄에 하나씩.
# 치환자: {k}킬러 {d}죽은선수 {p}선수명 {b}블루점수 {r}레드점수 {n}관여인원 {t}팀(블루/레드)
[kill]
{k} 굿킬
{d} 잘렸다
{k} 따냄
ㅋㅋ {d} 사망
나이스 갱
{k} +1
깔끔한 킬
{d} 짐 쌌네
[kill_solo]
ㅋㅋㅋㅋ {d} 솔킬당함
{d} 뭐함ㅋㅋ
{k} 1대1 압살
{d} 라인에서 녹음ㅋㅋ
{k} 손 좋네
{d} 이걸 짤리네ㄷㄷ
ㄷㄷ {k} 솔로킬
{d} 또 죽네
[kill_multi]
{k} 미친 캐치ㄷㄷ
{d} 물려서 끔살
{n}명이 달려드네ㅋㅋㅋ
한타 터졌다
{d} 포커싱 광속삭제
이걸 다 붙네ㅋㅋ
{k} 막타 강탈
집중공격 ㄷㄷ
[surv]
{p} 살았다ㄷㄷ
{p} 도망 지렸다
{p} 한 칸 남았는데 생존ㄷㄷ
{p} 왜 안죽음ㅋㅋ
{p} 기적의 생존
{p} 피 1 남기고 빠짐
[serpen]
{t} 세르펜 꿀꺽
세르펜 가져간다
{t} 세르펜 스틸ㄷㄷ
세르펜 먹고 스노우볼
{t} 오브젝트 챙김
세르펜 났다
[score]
{b}:{r} 박빙
치열하네
스코어 따라붙음
{b}:{r} ㄷㄷ
엎치락뒤치락
스코어 벌어진다
이거 터졌네
{b}:{r} 차이 크다
[idle_early]
오 시작이네
긴장된다
누가 이길까
초반 중요하지
오늘 누가 잘함?
라인전 보자
ㄷㄷ 기대된다
어디가 셀까
잘 좀 해봐라
집중집중
[idle_mid_close]
치열하네
해볼만한데
오 비등비등
아직 모름
둘 다 잘함
팽팽하다
[idle_mid_lead]
분위기 탔네
이거 벌어지나
한쪽이 잡았다
흐름 좋다
우세 굳히나
오 차이난다
[idle_late_close]
끝까지 모름ㄷㄷ
심장 떨린다
이거 진짜 박빙
한 끗 차이
누가 이겨도 인정
미쳤다 이거
[idle_late_lead]
거의 굳었나
역전 가능?
아직 희망 있나
이거 넘어가나
마무리각
따라잡을수 있을까
[idle_late_crush]
이거 GG각
클린업이네
사실상 끝
압도적이다
경기 터졌다
수고하셨습니다ㅋㅋ
[idle_player]
{p} 잘하네
{p} 기대된다
{p} 폼 좋아보임
{p} 집중하자
{p} 살아있네
[cheer]
블루팀 화이팅~
레드팀 가자!
오늘 경기 기대된다
꿀잼각ㅋㅋ
드디어 시작이네
가즈아~~
오늘 누가 이기냐
명경기 가자
두근두근
화이팅!
재밌겠다
다들 잘하자
[cheer_player]
{p} 화이팅
{p} 캐리 가자
{p} 믿는다
{p} 오늘 폼 보여줘
{p} 가즈아
하나 둘 셋 {p} 화이팅!
{p} 슈퍼플레이 가자
"#.to_string()
}

const NICK_A: &[&str] = &[
    "롤","새벽","야식","퇴근","본방","익명","침착","광동","한타","정글차이",
    "페이커","망겜","갱맘","다이브","노데스","캐리","트수","백수","직장인","학생",
    "모쏠","치킨","라면","콜라","닥터","골드","다이아","챌린저","브론즈","운지",
    "고독한","지나가던","화난","신난","졸린","배고픈","현질","무과금","랜덤","익명의",
];
const NICK_B: &[&str] = &[
    "장인","충","러","갓생","빠","안티","관전러","골수팬","분석가","평론가",
    "코치","해설","빌런","고수","뉴비","린저씨","아재","큰손","워리어","마스터",
    "헌터","폐인","거북이","워치맨","구독자","스트리머","택배","감자","고양이","너구리",
    "12년차","3년차","복귀","휴면","주작러","팩트","오타쿠","프로","아마추어","구경꾼",
];
fn nick() -> String {
    let a = NICK_A[rng() % NICK_A.len()];
    let b = NICK_B[rng() % NICK_B.len()];
    format!("{}{}", a, b)
}

fn field_i64(s:&str,key:&str)->Option<i64>{
    let p=s.find(key)?+key.len(); let r=&s[p..];
    let r=r.trim_start_matches(|c:char|c==' '||c==':');
    let e=r.find(|c:char|!(c.is_ascii_digit()||c=='-')).unwrap_or(r.len());
    r[..e].parse().ok()
}
fn field_quoted(s:&str,key:&str)->Option<String>{
    let p=s.find(key)?+key.len(); let r=&s[p..];
    let q1=r.find('"')?+1; let q2=r[q1..].find('"')?+q1; Some(r[q1..q2].to_string())
}
fn count_assist(s:&str)->usize{
    if let Some(p)=s.find("assist:"){ let r=&s[p..]; let e=r.find(']').unwrap_or(r.len());
        return r[..e].matches('"').count()/2; } 0
}

// 8% 확률로 뜬금없는 random 채팅 (모든 상황 공통). 해당되면 Some.
fn maybe_random()->Option<String>{
    if rng()%100 < 8 {
        let l=line_of("random");
        if !l.is_empty(){ return Some(format!("{}: {}", nick(), l)); }
    }
    None
}
fn sub(s:&str, k:&str,d:&str,p:&str,b:i64,r:i64,n:usize)->String{
    // {w}=앞선 팀, {l}=밀리는 팀 (점수 비교). 동점이면 둘다 "" (문구가 어색하면 안 쓰면 됨)
    let (win, lose) = if b > r { ("블루","레드") } else if r > b { ("레드","블루") } else { ("","") };
    s.replace("{k}",k).replace("{d}",d).replace("{p}",p)
     .replace("{b}",&b.to_string()).replace("{r}",&r.to_string())
     .replace("{w}",win).replace("{l}",lose)
     .replace("{n}",&n.to_string())
}
fn msg_kill(killer:&str, killed:&str, an:usize)->String{
    if let Some(x)=maybe_random(){return x;}
    let sec = if an==0 {"kill_solo"} else if an>=3 {"kill_multi"} else {"kill"};
    let mut l = line_of(sec);
    if l.is_empty() { l = line_of("kill"); }
    if l.is_empty() { l = format!("{} 처치", killer); }
    format!("{}: {}", nick(), sub(&l, killer, killed, "", 0, 0, an+1))
}
fn msg_serpen(team:i64)->String{
    if let Some(x)=maybe_random(){return x;}
    let t=if team==0 {"블루"} else {"레드"};
    let mut l=line_of("serpen"); if l.is_empty(){ l="세르펜 획득".into(); }
    format!("{}: {}", nick(), l.replace("{t}",t))
}
fn team_str(team:i64)->&'static str{ if team==0 {"블루"} else {"레드"} }
// 타워 파괴 (team = 파괴한 쪽)
fn msg_tower(team:i64)->String{
    if let Some(x)=maybe_random(){return x;}
    let mut l=line_of("tower"); if l.is_empty(){ l="{t} 타워 파괴".into(); }
    format!("{}: {}", nick(), l.replace("{t}",team_str(team)))
}
// N인 CC. hit_team=true → CC 건 쪽 시점(긍정), false → 당한 쪽 시점(부정).
fn msg_cc(n:usize, cc:&str, taken:bool, who:&str)->String{
    if let Some(x)=maybe_random(){return x;}
    let sec = if taken {"cc_taken"} else {"cc_hit"};
    let mut l=line_of(sec); if l.is_empty(){ l="{n}인 {cc}ㄷㄷ".into(); }
    let l=l.replace("{n}",&n.to_string()).replace("{cc}",cc).replace("{p}",who);
    format!("{}: {}", nick(), l)
}
// 궁 사용 (who = 시전 선수명)
fn msg_ult(who:&str)->String{
    if let Some(x)=maybe_random(){return x;}
    let mut l=line_of("ult"); if l.is_empty(){ l="{p} 궁".into(); }
    format!("{}: {}", nick(), l.replace("{p}",who))
}
// 골드/딜 우위 (team = 앞선 쪽, deal=true 면 딜량)
fn msg_lead(team:i64, deal:bool)->String{
    if let Some(x)=maybe_random(){return x;}
    let sec = if deal {"lead_deal"} else {"lead_gold"};
    let mut l=line_of(sec); if l.is_empty(){ l="{t} 우세".into(); }
    format!("{}: {}", nick(), l.replace("{t}",team_str(team)))
}
fn msg_score(b:i64,r:i64)->String{
    if let Some(x)=maybe_random(){return x;}
    let mut l=line_of("score"); if l.is_empty(){ l=format!("{}:{}",b,r); }
    format!("{}: {}", nick(), sub(&l,"","","",b,r,0))
}
fn ambient_section(b:i64,r:i64)->&'static str{
    let sum=b+r; let diff=(b-r).abs();
    if sum<=3 { "idle_early" }
    else if sum<=8 { if diff<=1 {"idle_mid_close"} else {"idle_mid_lead"} }
    else { if diff<=1 {"idle_late_close"} else if diff<=4 {"idle_late_lead"} else {"idle_late_crush"} }
}
fn msg_ambient(names:&[String], b:i64, r:i64)->String{
    if let Some(x)=maybe_random(){return x;}
    // 25% 선수 언급(idle_player 섹션)
    if !names.is_empty() && rng()%4==0 {
        let who=&names[rng()%names.len()];
        let l=line_of("idle_player");
        if !l.is_empty() { return format!("{}: {}", nick(), sub(&l,"","",who,b,r,0)); }
    }
    let mut l=line_of(ambient_section(b,r));
    if l.is_empty(){ l=line_of("idle_early"); }
    if l.is_empty(){ l="음...".into(); }
    // ambient 섹션도 {p} 쓰면 랜덤 선수명으로 치환 (idle_mid_close 등)
    let who = if names.is_empty() { String::new() } else { names[rng()%names.len()].clone() };
    format!("{}: {}", nick(), sub(&l,"","",&who,b,r,0))
}
fn msg_survive(who:&str)->String{
    if let Some(x)=maybe_random(){return x;}
    let mut l=line_of("surv"); if l.is_empty(){ l="{p} 생존".into(); }
    format!("{}: {}", nick(), l.replace("{p}",who))
}
fn msg_cheer(names:&[String])->String{
    if !names.is_empty() && rng()%10<4 {
        let who=&names[rng()%names.len()];
        let l=line_of("cheer_player");
        if !l.is_empty() { return format!("{}: {}", nick(), l.replace("{p}",who)); }
    }
    let mut l=line_of("cheer"); if l.is_empty(){ l="화이팅!".into(); }
    format!("{}: {}", nick(), l)
}

fn interp(frame_idx: usize, anchors: &[(usize,i64)]) -> i64 {
    if anchors.is_empty() { return -1; }
    match anchors.binary_search_by_key(&frame_idx, |&(f,_)| f) {
        Ok(i) => anchors[i].1,
        Err(i) => {
            if i==0 { anchors[0].1 }
            else if i>=anchors.len() { anchors[anchors.len()-1].1 }
            else { let (f0,t0)=anchors[i-1]; let (f1,t1)=anchors[i];
                if f1==f0 {t0} else {t0+(t1-t0)*(frame_idx as i64-f0 as i64)/(f1 as i64-f0 as i64)} }
        }
    }
}

struct P;
impl ModExtension for P {
    fn post_update(&self, scene:&mut Scene, ui:&mut GameUI, _a:&mut Assets, _dt:f32){

        let Scene::InGame{ data } = scene else { return; };
        mark("A_enter");

        // chat_btn 클릭 토글 + spectator_chat 패널 표시 반영 + 드래그 이동.
        let mut f = Frame::new();
        mark("B_frame_pre");
        f.toggle_flag("chat_btn", &CHAT_VISIBLE);
        f.apply(ui, &FH);
        mark("B_frame_post");
        set_visible_by_id(&mut ui.root, "spectator_chat", CHAT_VISIBLE.load(Ordering::Relaxed));
        mark("C_visible");
        // 드래그 이동 = 엔진 내장 draggable_popup Runner가 처리(.ui에서 spectator_chat:draggable_popup).
        // 기존 Win32 폴링 드래그(DRAG.update)는 비활성화 — 같은 노드 x/y를 두 곳이 쓰면 충돌.
        mark("D_drag_done");

        // === draggable_popup 수치필드 런타임 충전 (ctor가 전부 0으로 둠 → 우리가 채움) ===
        // .ui 속성 파서(0.5.0_3=0xf1e760, 구0.5.0_2=0x15e3670)는 ignore_event/z 만 읽는다 → 아래 4개는 .ui 선언 불가, 런타임 write가 유일.
        // 게임 밴픽 코치 채팅창(chat_popup)도 동일 방식: downcast 후 movups 로 (40,300,250,12) 한 방에 씀.
        //
        // on_mouse_down(0.5.0_3=0x100a3e0, 구0.5.0_2=0x15e5ec0) 판정 (lx,ly = 커서 - 창 좌상단, H = resize_handle):
        //   H <= 0.0                        → 리사이즈 완전 OFF, 드래그만
        //   4변 H px 밴드 안                → 리사이즈 시작 (리사이즈가 드래그보다 우선)
        //   그 외 & 0<=ly<header_height     → 드래그 시작
        // on_mouse_move(0.5.0_3=0x100a8c0, 구0.5.0_2=0x15e6190)가 노드 width/height/x/y 를 직접 갱신하므로 모드는 크기를 만지지 않는다.
        // ⚠ +0x1a8~+0x1bc(resize_start_*), +0x1d0~+0x1d7(ignore_event/dirty/dragging/resize_*/anchored)는
        //    엔진 상태 → 절대 write 금지. 우리는 +0x1c0..+0x1cf 4개만 매 프레임 보정(상태와 독립, 안전).
        if let Some(n) = find(&ui.root, "spectator_chat") {
            if let Some(b) = runner_base(n, "DraggablePopup") {
                // 드래그 밴드 = 창 전체(가장자리 밴드 제외) → 어디를 잡아도 이동.
                // ⚠창 높이 상수(구 214.0)를 쓰면 리사이즈로 키운 뒤 아래쪽이 드래그 불가 → 충분히 큰 값 고정.
                //   (ly는 창 안에서만 오므로 상한만 크면 됨. 가장자리는 리사이즈가 먼저 먹는다.)
                runner_wr_f32(b, 0x1c0, 4000.0);      // header_height
                runner_wr_f32(b, 0x1c4, WIN_MIN_W);   // min_w = 초기 크기 (더 못 줄임)
                runner_wr_f32(b, 0x1c8, WIN_MIN_H);   // min_h = 초기 크기
                runner_wr_f32(b, 0x1cc, 8.0);         // resize_handle: 4변/4코너 8px 밴드 → 리사이즈 ON
            }
        }
        // 창 크기 → 표시 줄 수 갱신 + 화면 밖으로 못 나가게 x/y 보정.
        // (엔진은 크기 상한 1920x1080, 위치 하한 0 만 걸고 오른쪽·아래 경계는 안 막는다.)
        if let Some(n) = find_mut(&mut ui.root, "spectator_chat") {
            let h = node_layout_read(n, NODE_LF_H);
            VIS_LINES.store(lines_for_height(h), Ordering::Relaxed);
            clamp_popup_into_screen(n);
        }
        if find(&ui.root, "game_time").is_none() {
            SCAN_DONE.store(false, Ordering::Relaxed);
            SCAN_POS.store(0, Ordering::Relaxed);
            SCAN_TOTAL.store(0, Ordering::Relaxed);
            INDEX.lock().unwrap_or_else(|e| e.into_inner()).clear();
            ANCHORS.lock().unwrap_or_else(|e| e.into_inner()).clear();
            PENDING.lock().unwrap_or_else(|e| e.into_inner()).clear();
            DANGER.lock().unwrap_or_else(|e| e.into_inner()).clear();
            DEATHS.lock().unwrap_or_else(|e| e.into_inner()).clear();
            SCORELINE.lock().unwrap_or_else(|e| e.into_inner()).clear();
            CHEER.lock().unwrap_or_else(|e| e.into_inner()).clear();
            CHEER_MERGED.store(false,Ordering::Relaxed);
            *ID2NAME.lock().unwrap_or_else(|e| e.into_inner())=None;
            *CHAMP2ID.lock().unwrap_or_else(|e| e.into_inner())=None;
            *HPSTATE.lock().unwrap_or_else(|e| e.into_inner())=None;
            CCEV.lock().unwrap_or_else(|e| e.into_inner()).clear();
            ULTEV.lock().unwrap_or_else(|e| e.into_inner()).clear();
            TOWERLINE.lock().unwrap_or_else(|e| e.into_inner()).clear();
            STATLINE.lock().unwrap_or_else(|e| e.into_inner()).clear();
            SCORELINE_SCORED.lock().unwrap_or_else(|e| e.into_inner()).clear();
            SERPENLINE.lock().unwrap_or_else(|e| e.into_inner()).clear();
            REBUILD_AT.store(usize::MAX, Ordering::Relaxed);
            *SCORE_LAST.lock().unwrap_or_else(|e| e.into_inner()) = (-1,-1);
            *ID2TEAM.lock().unwrap_or_else(|e| e.into_inner())=None;
            *CHAMP2NAME.lock().unwrap_or_else(|e| e.into_inner()) = None;
            *PHRASES.lock().unwrap_or_else(|e| e.into_inner()) = None;
            return;
        }
        mark("D2_gametime_ok");
        // 문구가 아직 로드 안 됐으면 즉시 로드 (로딩 응원이 폴백("화이팅!")만 나오는 것 방지)
        if PHRASES.lock().unwrap_or_else(|e| e.into_inner()).is_none() { load_phrases(); }
        mark("D3_phrases");

        let db = data.db();
        mark("D4_db");

        // ★ 프레임 소스 결정: 리플레이(game_view Some) vs 즉시보기(None → db.scene+984)
        //   둘다 &Vec<Arc<GameFrameData>> 동일 타입. 즉시보기는 game_view 타입 anchor 로 transmute.
        let gv_anchor: Option<&_> = db.game_view.as_ref().map(|gv| &gv.client.events);
        mark("D5a_anchor");
        let (played, frames): (i64, &Vec<_>) = if let Some(gv) = db.game_view.as_ref() {
            // 리플레이: 기존 경로
            mark("D5b_gv_some");
            let pt = gv.client.view.played_tick as i64;
            mark("D5b2_played");
            (pt, &gv.client.events)
        } else {
            mark("D5c_gv_none");
            // ★0.5.0 즉시보기(라이브 경기) 절대 오프셋 (SDK PDB CodeView TPI 정적 재도출 2026-07-08):
            //   played_tick = db+5512 (재생위치, 매프레임 증가), client.events Vec = db+5728 (cap@+0/ptr@+8/len@+16).
            //   산식: ClientDatabase.scene(+0x1328) + InGame payload(+8) + X
            //     X = GameView.played_tick(+0x258)  → 0x1588 = 5512   (GameClient.view 는 GameClient+0x0)
            //     X = GameClient.events(+0x330)     → 0x1660 = 5728
            //   ⚠ClientScene 은 niche enum(InGame tag=9, u32) — payload 는 tag+8 부터.
            //   구 0.4.14: scene(0xF90)+8+0x2F8=4752 / +8+0x3D0=4968 (같은 산식으로 재현 확인)
            //   ※events 5728 은 community_reaction_mod 0.5.0 인게임 검증으로 런타임 실증됨.
            //   ★0.5.0_3: ClientDatabase 가 scene 앞에 필드 +0x10 추가 → scene 0x1328→0x1338, db-절대 오프셋 전부 +0x10.
            //     (scene 내부 상대 0x258/0x330 은 불변. 런타임 스캔 실증: scene_tag@0x1338, events@5744 cap=32768 len=30785, played@5528.)
            const LIVE_PLAYED_OFF: usize = 5528;            // 0.5.0_3(구5512). scene(db+4920=0x1338)+8+0x258
            const LIVE_EVENTS_OFF: usize = 5744;            // 0.5.0_3(구5728). scene(db+4920=0x1338)+8+0x330
            unsafe {
                let base = (&*db as *const _) as usize;
                let played = *((base + LIVE_PLAYED_OFF) as *const u64) as i64;
                let ce = base + LIVE_EVENTS_OFF;
                let rp = *((ce + 8) as *const usize);
                let rl = *((ce + 16) as *const usize);
                mark(&format!("D5c_live played={} rl={}", played, rl));
                if rp > 0x10000 && rp < (1usize<<48) && rl >= 1 && rl <= 10_000_000 {
                    let frames = cast_as(&gv_anchor, ce);
                    (played, frames)
                } else {
                    if DIAG_FRAME.load(Ordering::Relaxed) % 60 == 0 {
                        diag(&format!("LIVE-REJECT played={} rp={:#x} rl={}", played, rp, rl));
                    }
                    DIAG_FRAME.fetch_add(1, Ordering::Relaxed);
                    return; // frames 아직 준비 안됨
                }
            }
        };

        mark(&format!("F_frames played={} total={}", played, frames.len()));
        if true {
            let total = frames.len();
            mark(&format!("G_scan_check total={}", total));
            let prev_total = SCAN_TOTAL.swap(total, Ordering::Relaxed);
            // ★2026-07-30 수정: 구코드는 stable(= total 이 두 프레임 연속 동일)일 때만 스캔을 시작했다.
            //   라이브는 백그라운드 sim 이 frames 를 계속 append 하므로 sim 이 완주할 때까지
            //   스캔이 시작조차 못 하고, 그동안 "로딩 중" 분기의 응원 문구만 계속 나온다
            //   (= 재생이 sim 을 따라잡아 화면이 '대기중'인 구간 내내 응원만. 2026-07-30 실측 확인).
            //   frames 는 append-only(이미 스캔한 앞부분의 인덱스·내용 불변)이므로
            //   total 이 늘어도 리셋하지 말고 그대로 이어서 스캔하면 된다.
            //   total 이 줄어든 경우만(새 경기/리플레이 전환으로 Vec 이 갈렸을 때) 전체 리셋.
            let stable = total > 100;
            let restart = total < prev_total;
            // ★진단: 30프레임(≈0.5초)마다 스캔 상태 1줄. 응원만 나오는 원인이
            //   stable 미충족(total 요동)인지 / 스캔 진행중(pos<total)인지 / 다른 데서 죽는지 판별.
            {
                let fno = DIAG_FRAME.fetch_add(1, Ordering::Relaxed);
                if fno % 30 == 0 {
                    diag(&format!("f={} src={} played={} total={} prev={} stable={} done={} pos={} idx={}",
                        fno,
                        if gv_anchor.is_some() { "replay" } else { "live" },
                        played, total, prev_total, stable as u8,
                        SCAN_DONE.load(Ordering::Relaxed) as u8,
                        SCAN_POS.load(Ordering::Relaxed),
                        INDEX.lock().unwrap_or_else(|e| e.into_inner()).len()));
                }
            }
            if restart {
                SCAN_POS.store(0, Ordering::Relaxed);
                SCAN_DONE.store(false, Ordering::Relaxed);
                ANCHORS.lock().unwrap_or_else(|e| e.into_inner()).clear();
                PENDING.lock().unwrap_or_else(|e| e.into_inner()).clear();
            DANGER.lock().unwrap_or_else(|e| e.into_inner()).clear();
            DEATHS.lock().unwrap_or_else(|e| e.into_inner()).clear();
            SCORELINE.lock().unwrap_or_else(|e| e.into_inner()).clear();
            CHEER.lock().unwrap_or_else(|e| e.into_inner()).clear();
            CHEER_MERGED.store(false,Ordering::Relaxed);
            *ID2NAME.lock().unwrap_or_else(|e| e.into_inner())=None;
            *CHAMP2ID.lock().unwrap_or_else(|e| e.into_inner())=None;
            *HPSTATE.lock().unwrap_or_else(|e| e.into_inner())=None;
            CCEV.lock().unwrap_or_else(|e| e.into_inner()).clear();
            ULTEV.lock().unwrap_or_else(|e| e.into_inner()).clear();
            TOWERLINE.lock().unwrap_or_else(|e| e.into_inner()).clear();
            STATLINE.lock().unwrap_or_else(|e| e.into_inner()).clear();
            SCORELINE_SCORED.lock().unwrap_or_else(|e| e.into_inner()).clear();
            SERPENLINE.lock().unwrap_or_else(|e| e.into_inner()).clear();
            REBUILD_AT.store(usize::MAX, Ordering::Relaxed);
            *SCORE_LAST.lock().unwrap_or_else(|e| e.into_inner()) = (-1,-1);
            *ID2TEAM.lock().unwrap_or_else(|e| e.into_inner())=None;
                INDEX.lock().unwrap_or_else(|e| e.into_inner()).clear();
                *CHAMP2NAME.lock().unwrap_or_else(|e| e.into_inner()) = None;
            }

            if stable && SCAN_POS.load(Ordering::Relaxed) < total {
                let mut pos = SCAN_POS.load(Ordering::Relaxed);
                let end = (pos + SCAN_PER_FRAME).min(total);
                let mut anchors = ANCHORS.lock().unwrap_or_else(|e| e.into_inner());
                let mut pending = PENDING.lock().unwrap_or_else(|e| e.into_inner());
                let mut index = INDEX.lock().unwrap_or_else(|e| e.into_inner());
                let mut danger = DANGER.lock().unwrap_or_else(|e| e.into_inner());
                let mut deaths = DEATHS.lock().unwrap_or_else(|e| e.into_inner());
                let mut ccev = CCEV.lock().unwrap_or_else(|e| e.into_inner());
                let mut ultev = ULTEV.lock().unwrap_or_else(|e| e.into_inner());
                let mut score_guard = SCORE_LAST.lock().unwrap_or_else(|e| e.into_inner());
                let score_last = &mut *score_guard;
                let mut hpguard = HPSTATE.lock().unwrap_or_else(|e| e.into_inner());
                let hp = hpguard.get_or_insert_with(HashMap::new);
                let mut idguard = ID2NAME.lock().unwrap_or_else(|e| e.into_inner());
                let id2name = idguard.get_or_insert_with(HashMap::new);
                let mut itguard = ID2TEAM.lock().unwrap_or_else(|e| e.into_inner());
                let id2team = itguard.get_or_insert_with(HashMap::new);
                let mut cidguard = CHAMP2ID.lock().unwrap_or_else(|e| e.into_inner());
                let champ2id = cidguard.get_or_insert_with(HashMap::new);
                // ★시간예산: 매 프레임 스캔에 이만큼만 쓰고 끊음(스터터 방지). 누적상태는 static에 보존돼 다음 프레임 SCAN_POS서 그대로 재개.
                //   ⚠아직 채팅 인덱스가 없는 동안(=응원만 나오는 상태)은 예산을 키운다.
                //     라이브 sim 은 게임프레임당 ~100 프레임을 생산하는데 1.5ms 로는 ~109 프레임밖에
                //     못 훑어 스캔이 sim 을 겨우 턱걸이로 따라잡는다(2026-07-30 실측). 따라잡은 뒤엔 1.5ms 로 복귀.
                let budget_us = if SCAN_DONE.load(Ordering::Relaxed) { 1500 } else { 4000 };
                let t0 = std::time::Instant::now();
                while pos < end {
                    if pos % 16 == 0 && t0.elapsed().as_micros() > budget_us { break; }
                    for ev in frames[pos].events.iter() {
                        let s = format!("{:?}", ev);
                        let b = s.as_bytes();
                        if s.starts_with("EntityEvent") {
                            if let Some(id)=field_i64(&s,"id:") {
                                if s.contains("EntityInfo") {
                                    if let Some(mx)=field_i64(&s,"max_hp:") {
                                        let e=hp.entry(id).or_insert((mx,mx,false)); e.0=mx;
                                    }
                                } else if s.contains("EntityHp") {
                                    if let Some(h)=field_i64(&s,"hp:") {
                                        if !id2name.contains_key(&id) { continue; }   // 선수만
                                        let e=hp.entry(id).or_insert((2000,h,false));
                                        let (mx,_last,in_danger)=*e;
                                        let low_line=mx/5;   // 20%
                                        if h>0 && h<=low_line {
                                            if !in_danger {   // 위험 첫 진입만 기록
                                                danger.push((pos, id));
                                                e.2=true;
                                            }
                                        } else if h > low_line {
                                            e.2=false;   // 위험선 위로 올라오면 해제(다음 진입 가능)
                                        }
                                        e.1=h;
                                    }
                                } else if s.contains("ty: Die") {
                                    // ★죽음: 선수(id2name 등록된 id)만. EntityHp/danger 와 동일 id 체계 → 생존매칭 정확.
                                    if id2name.contains_key(&id) { deaths.push((pos, id)); }
                                } else if s.contains("Stun(") {
                                    if id2name.contains_key(&id) { ccev.push((pos, id, 0)); }   // 선수만
                                } else if s.contains("Airborne(") {
                                    if id2name.contains_key(&id) { ccev.push((pos, id, 1)); }
                                } else if s.contains("Bind(") {
                                    if id2name.contains_key(&id) { ccev.push((pos, id, 2)); }
                                } else if s.contains("action: \"ult\"") {
                                    if id2name.contains_key(&id) { ultev.push((pos, id)); }   // 선수 궁만
                                }
                            }
                            continue;
                        }
                        // EntitySpawnData: 재spawn 마다 새 entity_id 로 선수 매핑 갱신 (궁/CC 의 id 해석에 필수)
                        if s.starts_with("EntitySpawn") {
                            if let Some(id)=field_i64(&s,"id:") {
                                if let Some(pn)=field_quoted(&s,"player_name:") {
                                    if !pn.is_empty() {   // 챔피언만 (미니언/몹은 빈 문자열)
                                        id2name.insert(id, pn.clone());
                                        // team: Player(0) / Player(1) 형태에서 숫자 추출
                                        if let Some(tp)=s.find("team:") {
                                            let r=&s[tp..];
                                            if let Some(tm)=field_i64(r,"Player(").or_else(||field_i64(r,"team:")) {
                                                id2team.insert(id, tm);
                                            }
                                        }
                                    }
                                }
                            }
                            continue;
                        }
                        if b.first()==Some(&b'E') { continue; }
                        if s.starts_with("PlayerStatistics") {
                            // 팀별 gold/deal 누적 (team 0/1 의 한 선수 스냅샷씩 옴). tick 은 없어서 frame 기준.
                            if let Some(team)=field_i64(&s,"team:") {
                                let g=field_i64(&s,"gold:").unwrap_or(0);
                                let d=field_i64(&s,"deal:").unwrap_or(0);
                                let t=interp(pos,&anchors);
                                let mut sl=STATLINE.lock().unwrap_or_else(|e| e.into_inner());
                                // 같은 tick 묶음에 누적 (마지막 항목이 같은 t 면 합산, 아니면 새 항목)
                                if let Some(last)=sl.last_mut() {
                                    if last.0==t {
                                        if team==0 { last.1+=g; last.3+=d; } else { last.2+=g; last.4+=d; }
                                    } else {
                                        let (mut gb,mut gr,mut db,mut dr)=(0,0,0,0);
                                        if team==0 {gb=g;db=d;} else {gr=g;dr=d;}
                                        sl.push((t,gb,gr,db,dr));
                                    }
                                } else {
                                    let (mut gb,mut gr,mut db,mut dr)=(0,0,0,0);
                                    if team==0 {gb=g;db=d;} else {gr=g;dr=d;}
                                    sl.push((t,gb,gr,db,dr));
                                }
                            }
                            continue;
                        }
                        if b.first()==Some(&b'P') { continue; }
                        if s.starts_with("Score") {
                            if let Some(t)=field_i64(&s,"tick:"){
                                anchors.push((pos,t));
                                if let Some(p)=s.find("scores:"){ let r=&s[p..];
                                    if let Some(lb)=r.find('['){ let inner=&r[lb+1..];
                                        let n:Vec<i64>=inner.split(|c|c==','||c==']').filter_map(|x|x.trim().parse().ok()).take(2).collect();
                                        if n.len()==2 && (n[0],n[1])!=*score_last {
                                            SCORELINE_SCORED.lock().unwrap_or_else(|e| e.into_inner()).push((t,n[0],n[1]));
                                            *score_last=(n[0],n[1]);
                                        }
                                        if n.len()==2 { SCORELINE.lock().unwrap_or_else(|e| e.into_inner()).push((t,n[0],n[1])); }
                                        }}
                                // 타워 점수 추적
                                if let Some(p)=s.find("tower_score:"){ let r=&s[p..];
                                    if let Some(lb)=r.find('['){ let inner=&r[lb+1..];
                                        let tw:Vec<i64>=inner.split(|c|c==','||c==']').filter_map(|x|x.trim().parse().ok()).take(2).collect();
                                        if tw.len()==2 { TOWERLINE.lock().unwrap_or_else(|e| e.into_inner()).push((t,tw[0],tw[1])); }
                                    }}
                            }
                        } else if s.starts_with("SerpenKill") {
                            if let Some(t)=field_i64(&s,"tick:"){
                                anchors.push((pos,t));
                                let team=field_i64(&s,"team:").unwrap_or(0);
                                SERPENLINE.lock().unwrap_or_else(|e| e.into_inner()).push((t,team));
                            }
                        } else if s.starts_with("KillEvent") {
                            let kr=field_quoted(&s,"killer:").unwrap_or_else(||"?".into());
                            let kd=field_quoted(&s,"killed:").unwrap_or_else(||"?".into());
                            // (죽음 수집은 EntityEvent::Die 로 이동 — id 체계 일치. 여기선 킬채팅만)
                            pending.push((pos, kr, kd, count_assist(&s)));
                        } else if s.contains("GamePlayerState") {
                            if let (Some(ch),Some(nm))=(field_quoted(&s,"champion:"),field_quoted(&s,"name:")){
                                if let Some(p)=s.find(", id:").or_else(||s.find("{ id:")) {
                                    let r=&s[p..];
                                    if let Some(eid)=field_i64(r,"id:") {
                                        id2name.insert(eid,nm.clone());
                                        champ2id.insert(ch.clone(), eid);
                                        if let Some(tm)=field_i64(&s,"team:") { id2team.insert(eid, tm); }
                                    }
                                }
                                CHAMP2NAME.lock().unwrap_or_else(|e| e.into_inner()).get_or_insert_with(HashMap::new).entry(ch).or_insert(nm);
                            }
                        }
                    }
                    pos += 1;
                }
                SCAN_POS.store(pos, Ordering::Relaxed);
                mark(&format!("I_scan_loopdone pos={}", pos));

                // ★재생성 게이트(2026-07-30). 구코드는 "경기 전체를 완주 스캔한 뒤 1회" 전제였다.
                //   라이브는 sim 이 프레임을 계속 append 하므로 그 전제가 성립하지 않아
                //   채팅이 영영 안 만들어졌다(→ 응원 문구만). 이제 두 조건 중 하나면 만든다:
                //     ① 스캔이 현재 total 을 따라잡음  ② 스캔한 지점이 재생 커서보다 앞섬(표시에 충분)
                //   여러 번 돌아도 안전하도록, 이미 지나간 구간(tick<=played)은 보존하고 미래만 갈아끼운다.
                const REBUILD_STEP: usize = 3000;   // 이만큼 더 스캔되면 미래 구간 갱신
                let scanned_tick = interp(pos, &anchors);
                let ready = pos >= total || scanned_tick > played + WINDOW_TICKS;
                let rb_at = REBUILD_AT.load(Ordering::Relaxed);
                if ready && (rb_at == usize::MAX || pos >= rb_at.saturating_add(REBUILD_STEP)) {
                    REBUILD_AT.store(pos, Ordering::Relaxed);
                    let mut nidx: Vec<Indexed> = Vec::new();
                    let map = CHAMP2NAME.lock().unwrap_or_else(|e| e.into_inner());
                    let to_name=|k:&str|->String{ map.as_ref().and_then(|m|m.get(k)).cloned().unwrap_or_else(||k.to_string()) };
                    // 스코어 변동 / 세르펜 (스캔 단계에서 모아둔 원시 기록)
                    for &(t,b_,r_) in SCORELINE_SCORED.lock().unwrap_or_else(|e| e.into_inner()).iter() {
                        let cnt=2+rng()%3;  // 2~4개
                        for j in 0..cnt { nidx.push(Indexed{tick:t+(j as i64)*SPREAD,text:msg_score(b_,r_)}); }
                    }
                    for &(t,team) in SERPENLINE.lock().unwrap_or_else(|e| e.into_inner()).iter() {
                        let cnt=2+rng()%3;
                        for j in 0..cnt { nidx.push(Indexed{tick:t+(j as i64)*SPREAD,text:msg_serpen(team)}); }
                    }
                    // 킬: pending 은 drain 하지 않는다(재생성 때 다시 필요).
                    for (fidx,kr,kd,an) in pending.iter().map(|x|(x.0,x.1.clone(),x.2.clone(),x.3)){
                        let base=interp(fidx,&anchors);
                        let (kn,dn)=(to_name(&kr),to_name(&kd));
                        let cnt=2+rng()%3;
                        for j in 0..cnt { nidx.push(Indexed{tick:base+(j as i64)*SPREAD,text:msg_kill(&kn,&dn,an)}); }
                    }
                    // 생존 매칭: 위험진입 후 SURV_WINDOW 프레임 내 같은 id 죽음 없으면 생존.
                    const SURV_WINDOW: usize = 600;   // 10초
                    const SURV_GAP: usize = 600;      // 같은 선수 생존 메시지 최소 간격(중복 방지)
                    let mut last_surv: HashMap<i64,usize> = HashMap::new();
                    for (df, did) in danger.iter() {
                        // 이 위험진입 후 SURV_WINDOW 내에 같은 id 죽음이 있나?
                        let died = deaths.iter().any(|(kf,kid)| *kid==*did && *kf>=*df && *kf<=*df+SURV_WINDOW);
                        if died { continue; }
                        // 중복 방지: 같은 선수 최근 생존과 너무 가까우면 skip
                        if let Some(&lf)=last_surv.get(did) { if df.saturating_sub(lf)<SURV_GAP { continue; } }
                        last_surv.insert(*did, *df);
                        // 생존 시점 = 위험진입 + WINDOW (버텨낸 시점)
                        let sf = df + SURV_WINDOW;
                        if let Some(nm)=idguard.as_ref().and_then(|m|m.get(did)).cloned() {
                            let base=interp(sf,&anchors);
                            let cnt=2+rng()%3;
                            for j in 0..cnt { nidx.push(Indexed{tick:base+(j as i64)*SPREAD,text:msg_survive(&nm)}); }
                        }
                    }
                    // ── 새 이벤트 처리 ──
                    // borrow 충돌 방지: 매핑을 스냅샷으로 복사해서 사용 (선수 10명 수준이라 가벼움).
                    let name_map: HashMap<i64,String> = idguard.as_ref().cloned().unwrap_or_default();
                    let team_map: HashMap<i64,i64> = itguard.as_ref().cloned().unwrap_or_default();
                    let to_name2=|id:&i64|->Option<String>{ name_map.get(id).cloned() };
                    let id_team=|id:&i64|->Option<i64>{ team_map.get(id).copied() };

                    // N인 CC: 같은 frame 근처(±30프레임=0.5초)에 같은 당한 팀 선수 2명 이상 CC.
                    {
                        let mut cc=ccev.clone();
                        cc.sort_by_key(|x|x.0);
                        const CC_WIN: usize = 30;        // 0.5초
                        let cc_name=|k:u8|->&'static str{ match k {0=>"스턴",1=>"에어본",_=>"속박"} };
                        let mut used=vec![false;cc.len()];
                        for i in 0..cc.len() {
                            if used[i] { continue; }
                            let (f0,_,k0)=cc[i];
                            let team0 = id_team(&cc[i].1);
                            let mut ids=vec![cc[i].1]; used[i]=true;
                            for j in (i+1)..cc.len() {
                                if used[j] { continue; }
                                if cc[j].0 > f0+CC_WIN { break; }
                                if id_team(&cc[j].1)==team0 && !ids.contains(&cc[j].1) {
                                    ids.push(cc[j].1); used[j]=true;
                                }
                            }
                            let n=ids.len();
                            if n>=2 {
                                let base=interp(f0,&anchors);
                                let cc_kind=cc_name(k0);
                                let cnt=2+rng()%2;
                                for jj in 0..cnt {
                                    let taken = jj%2==0;
                                    let who = to_name2(&ids[rng()%ids.len()]).unwrap_or_default();
                                    nidx.push(Indexed{tick:base+(jj as i64)*SPREAD, text:msg_cc(n,cc_kind,taken,&who)});
                                }
                            }
                        }
                    }

                    // 궁 사용: 너무 잦지 않게 일부만(40%), 선수명 표기.
                    for (f,cid) in ultev.iter() {
                        if rng()%100 >= 40 { continue; }
                        if let Some(nm)=to_name2(cid) {
                            let base=interp(*f,&anchors);
                            nidx.push(Indexed{tick:base, text:msg_ult(&nm)});
                        }
                    }

                    // 타워 파괴: tower_score 가 늘어난 순간. team = 늘어난 쪽.
                    {
                        let tl=TOWERLINE.lock().unwrap_or_else(|e| e.into_inner());
                        let (mut pb,mut pr)=(0i64,0i64); let mut first=true;
                        for &(t,tb,tr) in tl.iter() {
                            if first { pb=tb; pr=tr; first=false; continue; }
                            if tb>pb { let cnt=1+rng()%2; for j in 0..cnt { nidx.push(Indexed{tick:t+(j as i64)*SPREAD,text:msg_tower(0)}); } }
                            if tr>pr { let cnt=1+rng()%2; for j in 0..cnt { nidx.push(Indexed{tick:t+(j as i64)*SPREAD,text:msg_tower(1)}); } }
                            pb=tb; pr=tr;
                        }
                    }

                    // 골드/딜 우위: 격차가 크게 벌어지는 순간 가끔. 쿨다운으로 도배 방지.
                    {
                        let sl=STATLINE.lock().unwrap_or_else(|e| e.into_inner());
                        const GOLD_GAP:i64=3000;
                        const DEAL_GAP:i64=5000;
                        const LEAD_COOLDOWN:i64=1800;  // 30초
                        let mut last_gold_t=-100000i64; let mut last_deal_t=-100000i64;
                        for &(t,gb,gr,db,dr) in sl.iter() {
                            let gd=gb-gr;
                            if gd.abs()>=GOLD_GAP && t-last_gold_t>=LEAD_COOLDOWN {
                                let cnt=1+rng()%2;
                                for j in 0..cnt { nidx.push(Indexed{tick:t+(j as i64)*SPREAD,text:msg_lead(if gd>0{0}else{1},false)}); }
                                last_gold_t=t;
                            }
                            let dd=db-dr;
                            if dd.abs()>=DEAL_GAP && t-last_deal_t>=LEAD_COOLDOWN {
                                let cnt=1+rng()%2;
                                for j in 0..cnt { nidx.push(Indexed{tick:t+(j as i64)*SPREAD,text:msg_lead(if dd>0{0}else{1},true)}); }
                                last_deal_t=t;
                            }
                        }
                    }

                    nidx.sort_by_key(|m| m.tick);
                    // 잡담 (스코어 단계별). STEP 8초로 빈도 완화.
                    const AMBIENT_GAP:i64=300; const AMBIENT_STEP:i64=480;
                    let names:Vec<String>=map.as_ref().map(|m|m.values().cloned().collect()).unwrap_or_default();
                    let scoreline = SCORELINE.lock().unwrap_or_else(|e| e.into_inner());
                    // tick 이전의 마지막 스코어 (b,r) 조회
                    let score_at = |tk:i64| -> (i64,i64) {
                        let mut br=(0i64,0i64);
                        for &(st,sb,sr) in scoreline.iter() { if st<=tk { br=(sb,sr); } else { break; } }
                        br
                    };
                    let mut amb:Vec<Indexed>=Vec::new();
                    if !nidx.is_empty(){
                        for w in 0..nidx.len().saturating_sub(1){
                            let (a,bb)=(nidx[w].tick,nidx[w+1].tick);
                            if bb-a>=AMBIENT_GAP{ let mut tk=a+AMBIENT_STEP;
                                while tk<bb-SPREAD{ let (sb,sr)=score_at(tk);
                                    amb.push(Indexed{tick:tk,text:msg_ambient(&names,sb,sr)}); tk+=AMBIENT_STEP; } }
                        }
                        // 첫 이벤트 전(경기 시작 직후) 구간은 응원 채팅으로 채움
                        let first=nidx[0].tick; let mut tk=first-AMBIENT_STEP;
                        while tk>30{ amb.push(Indexed{tick:tk,text:msg_cheer(&names)}); tk-=AMBIENT_STEP; }
                    }
                    drop(scoreline);
                    nidx.extend(amb);
                    nidx.sort_by_key(|m| m.tick);
                    let mut prev=i64::MIN;
                    for m in nidx.iter_mut(){ if m.tick<prev+SPREAD{m.tick=prev+SPREAD;} prev=m.tick; }
                    // ★확정 구간 보존 + 미래 구간 교체.
                    //   재생성은 rng 로 문구를 새로 뽑으므로, 이미 화면에 나갔을 수 있는
                    //   tick <= played 구간을 건드리면 표시 중인 채팅이 바뀐다 → 그 구간은 기존 것을 유지.
                    let keep = played;
                    let before = index.len();
                    index.retain(|m| m.tick <= keep);
                    index.extend(nidx.into_iter().filter(|m| m.tick > keep));
                    index.sort_by_key(|m| m.tick);
                    diag(&format!("REBUILD pos={} total={} scanned_tick={} played={} idx={}->{} anchors={} deaths={} cc={} ult={} kills={}",
                        pos, total, scanned_tick, played, before, index.len(),
                        anchors.len(), deaths.len(), ccev.len(), ultev.len(), pending.len()));
                    SCAN_DONE.store(true, Ordering::Relaxed);
                }
            }
        }
        drop(db);

        let phase = DISP_PHASE.fetch_add(1, Ordering::Relaxed);
        if phase % 6 == 0 {
            if SCAN_DONE.load(Ordering::Relaxed) {
                let mut index = INDEX.lock().unwrap_or_else(|e| e.into_inner());
                // 전환 1회: 로딩 때 쌓인 응원을, 현재 played_tick 직전 구간에 심어 연속성 확보
                if !CHEER_MERGED.swap(true, Ordering::Relaxed) {
                    let cheer = CHEER.lock().unwrap_or_else(|e| e.into_inner());
                    let n = cheer.len();
                    for (i, txt) in cheer.iter().enumerate() {
                        // played 직전 (n-i) 칸 앞에 배치
                        let tk = played - ((n - i) as i64) * SPREAD - 1;
                        index.push(Indexed{ tick: tk, text: txt.clone() });
                    }
                    index.sort_by_key(|m| m.tick);
                }
                let lo = played - WINDOW_TICKS;
                let cap = vis_lines();
                let mut buf:Vec<String>=Vec::with_capacity(cap);
                for m in index.iter(){
                    if m.tick>played {break;}
                    if m.tick>=lo { buf.push(m.text.clone()); if buf.len()>cap {buf.remove(0);} }
                }
                if DIAG_FRAME.load(Ordering::Relaxed) % 120 < 6 {
                    diag(&format!("DISPLAY played={} lo={} idx={} buf={}", played, lo, index.len(), buf.len()));
                }
                mark("J_fill_ingame_pre");
                fill_lines(&mut ui.root, &buf);
                mark("K_fill_ingame_done");
            } else {
                // 로딩 중: 응원 채팅만 (분석% 표시 없음). 빈도 절반(12프레임).
                let mut cheer=CHEER.lock().unwrap_or_else(|e| e.into_inner());
                if phase % 12 == 0 {
                    let names:Vec<String>=CHAMP2NAME.lock().unwrap_or_else(|e| e.into_inner()).as_ref()
                        .map(|m|m.values().cloned().collect()).unwrap_or_default();
                    cheer.push(msg_cheer(&names));
                    while cheer.len()>vis_lines() { cheer.remove(0); }
                }
                let buf:Vec<String>=cheer.clone();
                mark("J_fill_loading_pre");
                fill_lines(&mut ui.root, &buf);
                mark("K_fill_loading_done");
            }
        }
    }
}

fn init(_ctx:&GameCtx)->ModRegistration{
    install_panic_hook();
    diag("=== Spectator_Chat init (diag build) ===");
    let mut reg=ModRegistration::new(MOD_ID); reg.set_extension(P); reg
}
declare_mod!(init);