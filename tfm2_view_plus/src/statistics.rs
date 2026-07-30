//! statistics — 챔피언 통계표 티어 직접 부여 + 이름 색칠(순수 SDK, game_core 타입드).
//! 티어 저장 = Team.champion_tiers: HashMap<String, ChampionTier{S,A,B,C,D,NoTier}> (게임 세이브 자동저장,
//!   게임이 champion-info/banpick 에서 같은 필드 읽어 자동 전파 — UI relabel 불요).
//! 상호작용: 챔프 행의 champion_name(오버라이드가 button 으로 만듦) 클릭 → 티어 사이클(S→A→B→C→D→무→S).
//! 표시: champion_name 텍스트에 [S] 프리픽스 + 티어 색.
//! ⚠ 챔프 식별키 = 행 node.id 가 챔피언 내부 id 라고 가정 — 인게임서 실제 행 id/맵 키와 대조 필요(1줄 수정).
//! ⚠ 이름 버프/너프(svp_nc_btn 토글)는 승률필드 기반(daram2) — 소스 확정 후. 현재 토글만 배선.

use crate::config;
use crate::ui;
use game_core::ChampionTier;
use mod_api::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

static NAME_COLOR: AtomicBool = AtomicBool::new(false);

const LETTERS: [&str; 6] = ["S", "A", "B", "C", "D", ""]; // idx5 = 무티어

fn tier_idx(t: &ChampionTier) -> usize {
    match t {
        ChampionTier::S => 0,
        ChampionTier::A => 1,
        ChampionTier::B => 2,
        ChampionTier::C => 3,
        ChampionTier::D => 4,
        ChampionTier::NoTier => 5,
    }
}
fn tier_from_idx(i: usize) -> ChampionTier {
    match i % 6 {
        0 => ChampionTier::S,
        1 => ChampionTier::A,
        2 => ChampionTier::B,
        3 => ChampionTier::C,
        4 => ChampionTier::D,
        _ => ChampionTier::NoTier,
    }
}
fn tier_color(i: usize) -> (f32, f32, f32) {
    match i {
        0 => (0.95, 0.45, 0.55), // S 핑크레드
        1 => (0.95, 0.62, 0.38), // A 주황
        2 => (0.92, 0.85, 0.45), // B 노랑
        3 => (0.55, 0.80, 0.55), // C 초록
        4 => (0.55, 0.70, 0.90), // D 파랑
        _ => (0.80, 0.82, 0.86), // 무 회색
    }
}

/// "[X] Name" → "Name" (재기입 시 중복 프리픽스 방지).
fn strip_tag(s: &str) -> String {
    if let Some(rest) = s.strip_prefix('[') {
        if let Some(pos) = rest.find("] ") {
            return rest[pos + 2..].to_string();
        }
    }
    s.to_string()
}

fn snapshot(data: &ClientData, tid: usize) -> HashMap<String, usize> {
    data.db()
        .teams
        .get(&tid)
        .map(|t| {
            t.champion_tiers
                .iter()
                .map(|(k, v)| (k.clone(), tier_idx(v)))
                .collect()
        })
        .unwrap_or_default()
}

fn cycle_tier(data: &ClientData, tid: usize, champ: &str) {
    let mut db = data.db_mut();
    if let Some(team) = db.teams.get_mut(&tid) {
        let cur = team.champion_tiers.get(champ).map(tier_idx).unwrap_or(5);
        let next = (cur + 1) % 6;
        team.champion_tiers
            .insert(champ.to_string(), tier_from_idx(next));
    }
}

/// 챔프 행 id 수집(contents 하위 + champion_name 보유).
fn champion_rows(root: &Node) -> Vec<String> {
    let mut ids = Vec::new();
    ui::for_each(root, &mut |n| {
        if n.id == "contents" {
            for r in n.child.iter() {
                if !r.id.is_empty() && ui::find(r, "champion_name").is_some() {
                    ids.push(r.id.clone());
                }
            }
        }
    });
    ids
}

pub fn tick(scene: &mut Scene, ui_: &mut GameUI) {
    if !config::get().statistics {
        return;
    }
    if ui::find(&ui_.root, "svp_nc_btn").is_none() {
        return;
    }
    if ui::clicked("svp_nc_btn") {
        NAME_COLOR.fetch_xor(true, Ordering::Relaxed);
    }

    let Scene::InGame { data } = scene else { return };
    let tid = data.player_team_id();
    let rows = champion_rows(&ui_.root);

    // champion_name 클릭 → 해당 챔프 티어 사이클.
    for path in ui::click_paths() {
        if ui::last_segment(&path) == "champion_name" {
            if let Some(champ) = rows.iter().find(|id| ui::path_has_segment(&path, id)) {
                cycle_tier(data, tid, champ);
            }
        }
    }

    // 티어 스냅샷 → 각 챔프 행 이름에 [X] 프리픽스 + 색.
    let snap = snapshot(data, tid);
    ui::for_each_mut(&mut ui_.root, &mut |n| {
        if n.id != "contents" {
            return;
        }
        for r in n.child.iter_mut() {
            if r.id.is_empty() {
                continue;
            }
            let idx = match snap.get(&r.id) {
                Some(i) => *i,
                None => 5,
            };
            let Some(cn) = ui::find_mut(r, "champion_name") else {
                continue;
            };
            let Some(txt) = ui::find_mut(cn, "text") else {
                continue;
            };
            let base = strip_tag(ui::label_text(txt).unwrap_or(""));
            if idx >= 5 {
                ui::set_label_text(txt, &base);
            } else {
                ui::set_label(txt, &format!("[{}] {}", LETTERS[idx], base), tier_color(idx));
            }
        }
    });
}
