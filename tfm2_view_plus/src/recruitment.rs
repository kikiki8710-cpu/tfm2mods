//! recruitment — 영입 스카우팅 리스트: 영입 정보 ↔ 선수 능력치 12개 전환(순수 SDK).
//! .ui 오버라이드(scout.ui): 능력치 헤더 sab_hdr(여러 탭에 존재) + ⇄ 토글 recr_ability_btn.
//! base 행슬롯(scout_component/*_slot.ui) 정보컬럼 = name/age/position/team/contract/squad_status/
//!   potential/salary/transfer_fee/league/recommendation/report_date/delete.
//! 능력치 모드 ON: 정보컬럼(team/contract/squad_status/salary/transfer_fee/league/report_date) 숨김 +
//!   sab_hdr(헤더) 표시 + 각 행에 sab_hdr clone 주입. 유지: name/age/position/potential/recommendation/delete.
//! 행↔선수 = 행 #name 텍스트로 DB athlete 이름 매칭(영입은 타팀 포함이라 팀필터 없음).
//! ⚠ 다중 리스트/2슬롯/탭게이팅은 인게임 튜닝 필요(첫 구현).

use crate::config;
use crate::ui;
use mod_api::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

static ABILITY_MODE: AtomicBool = AtomicBool::new(false);

// 능력치 모드에서 숨길 정보 컬럼(유지: name/age/position/potential/recommendation/delete).
const HIDE_COLS: [&str; 7] = [
    "team",
    "contract",
    "squad_status",
    "salary",
    "transfer_fee",
    "league",
    "report_date",
];

fn athlete_row(a: &Athlete) -> [usize; 12] {
    let s = &a.stat;
    [
        s.last_hit,
        s.skill_avoid,
        s.skill_hit,
        s.control_speed,
        s.positioning,
        s.judgement,
        s.mental,
        s.concentration,
        s.order,
        s.roaming,
        s.aggressive,
        s.ego,
    ]
}

fn build_map(data: &ClientData) -> HashMap<String, [usize; 12]> {
    let mut m = HashMap::new();
    for aid in data.athlete_ids() {
        if let Some(a) = data.athlete(aid) {
            m.insert(a.name.to_string(), athlete_row(&a));
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

/// contents 컨테이너의 각 행 처리.
fn process_row(row: &mut Node, tmpl: &Node, map: &HashMap<String, [usize; 12]>, ability: bool) {
    let name = match ui::find_text(row, "name") {
        Some(n) => n.to_string(),
        None => return,
    };
    let Some(stats) = map.get(&name) else { return };
    for c in HIDE_COLS {
        ui::set_visible_all(row, c, !ability);
    }
    if ability {
        fill_row(row, tmpl, stats);
    } else {
        ui::set_visible_all(row, "vp_sab", false);
    }
}

pub fn tick(scene: &mut Scene, ui_: &mut GameUI) {
    if !config::get().recruitment {
        return;
    }
    // 영입 화면 판정(오버라이드가 넣은 recr_ability_btn 존재).
    if ui::find(&ui_.root, "recr_ability_btn").is_none() {
        return;
    }
    // 토글 버튼 노출(⚠ 원본은 지원 탭만 — 탭게이팅은 인게임 튜닝) + 클릭 반전.
    ui::set_visible_all(&mut ui_.root, "recr_ability_btn", true);
    if ui::clicked("recr_ability_btn") {
        ABILITY_MODE.fetch_xor(true, Ordering::Relaxed);
    }
    let ability = ABILITY_MODE.load(Ordering::Relaxed);

    // 헤더 sab_hdr 표시 전환(여러 개일 수 있음).
    ui::set_visible_all(&mut ui_.root, "sab_hdr", ability);

    let map = if ability {
        let Scene::InGame { data } = scene else { return };
        build_map(data)
    } else {
        HashMap::new()
    };
    let tmpl = ui::find(&ui_.root, "sab_hdr").cloned();
    let Some(tmpl) = tmpl else { return };

    // 모든 contents 컨테이너의 직속 자식을 행으로 처리(이름 매칭된 행만 변경).
    ui::for_each_mut(&mut ui_.root, &mut |node| {
        if node.id == "contents" {
            for row in node.child.iter_mut() {
                process_row(row, &tmpl, &map, ability);
            }
        }
    });
}
