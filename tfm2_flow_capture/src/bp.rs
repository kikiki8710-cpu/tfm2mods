//! bp — 밴픽 실시간 상태 → banpick_live.json/.js (feedback_tools\banpick.html 이 읽는다).
//! 클래식 = scene_step detour 로 BanpickScene raw Vec 을 읽었다. stable = 밴픽 화면 UI 노드 상태를 읽는다
//!   (champ_pos_lock ui_block 과 같은 경로: `main.champions.contents.<id>` 카드의 `.blue`/`.red`/`.ban`/`.fearless_icon` 가시성).
//! 진영별 밴 = 하단 `main.bottom.<side>_side.bans.<slot>.icon` 상태 JSON 에서 챔프 id 를 찾는다(못 찾으면 `bans_unknown` 에 남김).
//! 픽 순서 = 카드에 `.blue/.red` 가 켜진 **관측 순서**(15프레임 해상도).
use mod_api_stable::StableClient;
use std::sync::Mutex;

const CARDS: &str = "main.champions.contents";

#[derive(Default)]
struct St {
    active: bool,
    picks: [Vec<String>; 2],
}
static ST: Mutex<Option<St>> = Mutex::new(None);

fn vis(ctx: &StableClient<'_>, p: &str) -> bool {
    ctx.ui_visible(p) == Some(true)
}
fn arr(v: &[String]) -> String {
    format!("[{}]", v.iter().map(|s| format!("\"{}\"", s.replace(['"', '\\'], ""))).collect::<Vec<_>>().join(","))
}
fn out(body: &str) {
    crate::write_file("banpick_live.json", body);
    crate::write_file("banpick_live.js", &format!("window.__BP_LIVE={};", body));
}
fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}

/// 하단 밴 슬롯의 챔프 id: 아이콘 노드 상태 JSON 안에 카드 id(챔프 id)가 들어 있으면 그것(가장 긴 일치).
fn side_bans(ctx: &StableClient<'_>, side: &str, ids: &[String]) -> Vec<String> {
    let root = format!("main.bottom.{}_side.bans", side);
    let mut v = Vec::new();
    for c in ctx.ui_child_names(&root) {
        let js = ctx.ui_state_json(&format!("{}.{}.icon", root, c)).unwrap_or_default().to_ascii_lowercase();
        if js.is_empty() {
            continue;
        }
        if let Some(id) = ids.iter().filter(|id| js.contains(&format!("/{}", id)) || js.contains(&format!("\"{}\"", id))).max_by_key(|id| id.len()) {
            if !v.contains(id) {
                v.push(id.clone());
            }
        }
    }
    v
}

pub fn tick(ctx: &mut StableClient<'_>) {
    let mut g = ST.lock().unwrap_or_else(|e| e.into_inner());
    let st = g.get_or_insert_with(St::default);
    let my_tid = ctx.player_team_id().map(|t| t as i64).unwrap_or(-1);
    let on = ctx.ui_exists(CARDS) && vis(ctx, "main.champions_bg");
    if !on {
        if st.active {
            st.active = false;
            st.picks = [Vec::new(), Vec::new()];
            out(&format!("{{\"active\":false,\"ts\":{},\"my_tid\":{}}}", now_ms(), my_tid));
        }
        return;
    }
    st.active = true;
    let ids: Vec<String> = ctx.ui_child_names(CARDS).into_iter().map(|s| s.to_ascii_lowercase()).collect();
    let mut bans_all = Vec::new();
    let mut locked = Vec::new();
    let mut now_picks: [Vec<String>; 2] = [Vec::new(), Vec::new()];
    for id in &ids {
        let cp = format!("{}.{}", CARDS, id);
        if vis(ctx, &format!("{cp}.ban")) { bans_all.push(id.clone()); }
        if vis(ctx, &format!("{cp}.fearless_icon")) { locked.push(id.clone()); }
        if vis(ctx, &format!("{cp}.blue")) { now_picks[0].push(id.clone()); }
        if vis(ctx, &format!("{cp}.red")) { now_picks[1].push(id.clone()); }
    }
    // 관측 순서 유지(새로 켜진 것만 뒤에 붙이고, 꺼진 것은 뺀다)
    for s in 0..2 {
        st.picks[s].retain(|x| now_picks[s].contains(x));
        for x in &now_picks[s] {
            if !st.picks[s].contains(x) { st.picks[s].push(x.clone()); }
        }
    }
    let b1 = side_bans(ctx, "blue", &ids);
    let b2 = side_bans(ctx, "red", &ids);
    let unknown: Vec<String> = bans_all.iter().filter(|x| !b1.contains(x) && !b2.contains(x)).cloned().collect();
    let ppt = ctx.ui_child_names("main.blue_picks").iter().filter(|c| c.starts_with("pick_slot_")).count();
    // 내 진영: 하단 팀명(시드 접미 "#2" 허용)과 내 팀 이름 비교(champ_pos_lock 09-17 과 같은 규칙)
    let my_side = match ctx.player_team_id().and_then(|t| ctx.team_name(t)) {
        Some(name) => {
            let eq = |ui: &str| { let ui = ui.trim(); ui == name || ui.strip_prefix(name.as_str()).map(|r| r.trim_start().starts_with('#') || r.trim().is_empty()).unwrap_or(false) };
            if eq(&ctx.ui_text("main.bottom.blue_side.name").unwrap_or_default()) { 1 } else if eq(&ctx.ui_text("main.bottom.red_side.name").unwrap_or_default()) { 2 } else { 0 }
        }
        None => 0,
    };
    let body = format!(
        "{{\"active\":true,\"ts\":{},\"rule\":{},\"picks_per_team\":{},\"my_tid\":{},\"blue_tid\":-1,\"my_side\":{},\"team1\":{{\"bans\":{},\"picks\":{}}},\"team2\":{{\"bans\":{},\"picks\":{}}},\"bans_unknown\":{},\"locked\":{}}}",
        now_ms(), if ppt >= 2 { ppt as i64 - 2 } else { -1 }, ppt, my_tid, my_side, arr(&b1), arr(&st.picks[0]), arr(&b2), arr(&st.picks[1]), arr(&unknown), arr(&locked)
    );
    out(&body);
}
