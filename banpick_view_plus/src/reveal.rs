//! reveal — ★09-28 유저 요청(Flover): 스왑 중엔 상대 챔피언을 "?"로 가리고, **스왑이 끝나면 결과를 보여준 뒤** 전술창.
//! 게임은 양쪽 확정 즉시 전술창으로 넘어가므로(밴픽 화면에 머물 틈이 없다) ⟹ 전술창 위에 "상대 스왑 결과" 패널을
//! 3초 띄웠다 지운다(유저 선택 09-28: 게임 흐름을 건드리지 않는 방식).
//! 데이터 = 스왑 단계 동안 씬 raw `draft_scene::read().lineup(side)`(포지션 p → 챔피언 id · 상대 AI 스왑까지 반영, 09-18 RE)를
//!   매 틱 갱신해 두었다가, 스왑 화면이 닫히는 순간의 값을 쓴다.
//! 부모 경로 = 전술창 런타임 루트를 모른다(2026-09-28 기준 미확정) → 후보를 차례로 시도하고 성공 경로를 로그(`bp_reveal.txt`).
use mod_api_stable::StableClient;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

const NODE: &str = "bp_reveal";
const SHOW_FRAMES: u64 = 180; // 60fps 기준 3초
const WAIT_FRAMES: u64 = 600; // 전술창 부모를 못 찾으면 10초 뒤 포기
/// 전술창 부모 후보(앞선 것 우선). 성공 경로를 bp_reveal.txt 에 남겨 다음 판에 정본으로 고정한다.
const PARENTS: &[&str] = &["main", "strategy", "root", "body", "ui", "pre_game", "lineup", "strategy_ui"];
const PROBE_TAIL: &str = "contents.strategy.sub4.matchup";

/// 스왑 중 최신 상대 라인업(포지션 순 champ id).
static LAST: Mutex<Option<Vec<Option<String>>>> = Mutex::new(None);
/// 표시 대기/진행: (라인업, 무장 프레임, 표시 시작 프레임(0 = 미표시), 부모 경로)
static ARMED: Mutex<Option<(Vec<Option<String>>, u64, u64, String)>> = Mutex::new(None);
static LOGGED: AtomicU64 = AtomicU64::new(0);

fn ko() -> bool {
    static K: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *K.get_or_init(|| {
        let Some(d) = crate::mod_dir() else { return true };
        let p = std::path::Path::new(&d).parent().and_then(|m| m.parent()).map(|g| g.join("config").join("game").join("base.json"));
        p.and_then(|p| std::fs::read_to_string(p).ok()).map(|s| !s.contains("\"lang\": \"en\"") && !s.contains("\"lang\":\"en\"")).unwrap_or(true)
    })
}
fn log(s: &str) {
    if !crate::DBG || LOGGED.fetch_add(1, Ordering::Relaxed) > 60 { return; } // 진단(bp_reveal.txt)은 DBG 일 때만
    if let Some(d) = crate::mod_dir() { use std::io::Write; if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(format!("{}\\bp_reveal.txt", d)) { let _ = writeln!(f, "{}", s); } }
}

/// 스왑 화면이 보이는 동안 호출 — 숨기는 쪽(상대) 라인업을 갱신.
pub fn track(lineup: Vec<Option<String>>) {
    if lineup.iter().any(|c| c.is_some()) { *LAST.lock().unwrap_or_else(|e| e.into_inner()) = Some(lineup); }
}
/// 스왑 화면이 닫히는 순간 호출 — 마지막 라인업으로 표시 예약.
pub fn arm(f: u64) {
    let Some(l) = LAST.lock().unwrap_or_else(|e| e.into_inner()).take() else { return };
    log(&format!("arm f={} lineup={:?}", f, l));
    *ARMED.lock().unwrap_or_else(|e| e.into_inner()) = Some((l, f, 0, String::new()));
}

fn panel_source(ko: bool) -> String {
    let title = if ko { "상대 스왑 결과" } else { "Opponent swap result" };
    let mut rows = String::new();
    for i in 0..5 {
        rows.push_str(&format!(
            "#r{i}:empty {{ x: 60px; y: {y}px; width: 480px; height: 64px; \
               #pos:label {{ @\"asset/base/style/main#bold_label\"; z: 2002; x: 0px; y: 0px; width: 110px; height: 64px; size: 22; color: #9a9db0ff; align_y: Center; }} \
               #icon:image {{ z: 2002; x: 120px; y: 0px; width: 64px; height: 64px; }} \
               #name:label {{ @\"asset/base/style/main#bold_label\"; z: 2002; x: 204px; y: 0px; width: 276px; height: 64px; size: 22; align_y: Center; }} }} ",
            i = i, y = 84 + i * 72));
    }
    // ⚠z 는 노드마다(상속 안 됨 — 바닐라 .ui 도 부모·자식에 각각 지정). 09-28 실측: 자식 z 미지정 → 전술 패널(z 101/202) 아래 깔림 ⟹ 전부 2000대.
    format!("{NODE}:color {{ anchor_x: 0.5; pivot_x: 0.5; anchor_y: 0.5; pivot_y: 0.5; width: 600px; height: 470px; z: 2000; \
             color: #161721ff; rounding: Uniform {{ rounding: 14; }} ignore_event: true; \
             #title:label {{ @\"asset/base/style/main#bold_label\"; z: 2002; x: 0px; y: 18px; width: 600px; height: 48px; size: 28; color: #ff6b6bff; align_x: Center; align_y: Center; text: \"{title}\"; }} \
             {rows} }}")
}

/// 매 틱(3프레임) — 밴픽 화면 판정(DISC)보다 **먼저** 불러야 한다(전술창에선 밴픽 화면이 이미 사라짐).
pub fn tick(ctx: &mut StableClient<'_>, f: u64) {
    let mut g = ARMED.lock().unwrap_or_else(|e| e.into_inner());
    let Some((lineup, armed_at, shown_at, parent)) = g.as_mut() else { return };
    if *shown_at == 0 {
        if f.saturating_sub(*armed_at) > WAIT_FRAMES {
            let probe: Vec<String> = PARENTS.iter().map(|p| format!("{}={}/{}", p, ctx.ui_exists(p), ctx.ui_exists(&format!("{}.{}", p, PROBE_TAIL)))).collect();
            log(&format!("give up f={} (전술창 부모 못 찾음) probe={:?} roots={:?}", f, probe, ctx.ui_child_names("")));
            *g = None; return;
        }
        // 전술창이 뜬 것 = 매치업 노드가 보임. 그 루트(= 후보)에 스폰.
        let Some(p) = PARENTS.iter().find(|p| ctx.ui_visible(&format!("{}.{}", p, PROBE_TAIL)) == Some(true)) else { return };
        let path = format!("{}.{}", p, NODE);
        if ctx.ui_exists(&path) { ctx.ui_remove_node(&path); }
        let ok = ctx.ui_spawn_source(p, &panel_source(ko()));
        log(&format!("spawn f={} parent={} ok={} rect={:?}", f, p, ok, ctx.ui_node_rect(&path)));
        if !ok { *g = None; return; }
        let names_ko = ["탑", "정글", "미드", "원딜", "서폿"]; let names_en = ["Top", "Jungle", "Mid", "Bottom", "Support"];
        for i in 0..5 {
            let row = format!("{}.r{}", path, i);
            let _ = ctx.ui_set_text(&format!("{}.pos", row), if ko() { names_ko[i] } else { names_en[i] });
            match lineup.get(i).cloned().flatten() {
                Some(id) => {
                    let _ = ctx.ui_set_champion_icon(&format!("{}.icon", row), &id, 64.0, 64.0, 2.0);
                    let _ = ctx.ui_set_text(&format!("{}.name", row), &format!("#asset/base/text/champion?description.{}.name", id));
                }
                None => { let _ = ctx.ui_set_text(&format!("{}.name", row), "?"); }
            }
        }
        *shown_at = f; *parent = p.to_string();
    } else if f.saturating_sub(*shown_at) > SHOW_FRAMES {
        let path = format!("{}.{}", parent, NODE);
        if ctx.ui_exists(&path) { ctx.ui_remove_node(&path); }
        log(&format!("hide f={}", f));
        *g = None;
    }
}
