//! roster — 선수단(squad) 화면 우측 컬럼: 경기 스탯 ↔ 선수 능력치 12개 전환(순수 SDK).
//! .ui 오버라이드(squad/layout.ui): 헤더에 능력치 블록 sab_hdr(sab_h0..11, 처음 visible:false) 추가,
//!   하단 ⇄ 토글 버튼 squad_ability_btn.
//! base 행 슬롯(squad/athlete.ui) 경기스탯 셀 = game/kill/death/assist/level/rating.
//! 능력치 모드 ON: 헤더/행의 경기스탯 셀 숨김 + sab_hdr(헤더) 표시 + 각 행에 sab_hdr clone 주입해 값 채움.
//! 행↔선수 바인딩 = 행 #name 텍스트로 DB athlete 이름 매칭(팀 필터).

use crate::config;
use crate::ui;
use mod_api::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

static ABILITY_MODE: AtomicBool = AtomicBool::new(false);

const MATCH_CELLS: [&str; 6] = ["game", "kill", "death", "assist", "level", "rating"];

// sab_h0..11 헤더 순서(⚠ AthleteStat 필드 선언순서와 다름 — 라벨 의미로 매핑).
fn athlete_row(a: &Athlete) -> [usize; 12] {
    let s = &a.stat;
    [
        s.last_hit,      // sab_h0
        s.skill_avoid,   // sab_h1
        s.skill_hit,     // sab_h2
        s.control_speed, // sab_h3
        s.positioning,   // sab_h4
        s.judgement,     // sab_h5
        s.mental,        // sab_h6
        s.concentration, // sab_h7
        s.order,         // sab_h8
        s.roaming,       // sab_h9
        s.aggressive,    // sab_h10
        s.ego,           // sab_h11
    ]
}

fn build_map(data: &ClientData) -> HashMap<String, [usize; 12]> {
    let team = data.player_team_id();
    let mut m = HashMap::new();
    for aid in data.athlete_ids() {
        if let Some(a) = data.athlete(aid) {
            if a.contract.team_id() == Some(team) {
                m.insert(a.name.to_string(), athlete_row(&a));
            }
        }
    }
    m
}

fn fill_row(row: &mut Node, tmpl: &Node, stats: &[usize; 12]) {
    if ui::find(row, "vp_sab").is_none() {
        let mut c = tmpl.clone();
        c.id = "vp_sab".to_string();
        row.child.push(c);
    }
    if let Some(blk) = ui::find_mut(row, "vp_sab") {
        blk.visible = true;
        for i in 0..12 {
            let cid = format!("sab_h{i}");
            if let Some(cell) = ui::find_mut(blk, &cid) {
                if let Some(lbl) = ui::find_mut(cell, "text") {
                    let v = stats[i];
                    ui::set_label(lbl, &v.to_string(), ui::stat_color(v));
                }
            }
        }
    }
}

pub fn tick(scene: &mut Scene, ui_: &mut GameUI) {
    if !config::get().roster {
        return;
    }
    // 선수단 화면 판정(우리 오버라이드가 넣은 sab_hdr 존재).
    if ui::find(&ui_.root, "sab_hdr").is_none() {
        return;
    }
    // 토글 클릭 → 모드 반전.
    if ui::clicked("squad_ability_btn") {
        ABILITY_MODE.fetch_xor(true, Ordering::Relaxed);
    }
    let ability = ABILITY_MODE.load(Ordering::Relaxed);

    // 헤더 경기스탯 컬럼 ↔ sab_hdr 표시 전환.
    {
        let root = &mut ui_.root;
        for c in MATCH_CELLS {
            ui::set_visible(root, c, !ability);
        }
        ui::set_visible(root, "sab_hdr", ability);
    }

    // 능력치 모드 아닐 땐 주입 블록만 숨기고 경기스탯 복원.
    let tmpl = ui::find(&ui_.root, "sab_hdr").cloned();
    let map = if ability {
        let Scene::InGame { data } = scene else { return };
        build_map(data)
    } else {
        HashMap::new()
    };

    let Some(contents) = ui::find_mut(&mut ui_.root, "contents") else {
        return;
    };
    for row in contents.child.iter_mut() {
        // 경기스탯 셀 표시/숨김.
        for c in MATCH_CELLS {
            ui::set_visible(row, c, !ability);
        }
        if ability {
            if let (Some(tmpl), Some(name)) = (tmpl.as_ref(), ui::find_text(row, "name")) {
                let name = name.to_string();
                if let Some(stats) = map.get(&name) {
                    fill_row(row, tmpl, stats);
                    continue;
                }
            }
        }
        // 비능력치 모드: 주입 블록 숨김.
        ui::set_visible(row, "vp_sab", false);
    }
}
