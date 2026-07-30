//! coaching — 코치진 화면에 코치 능력치 10개 표시(순수 SDK).
//! .ui 오버라이드(staff/layout.ui)가 헤더 컬럼 csb_h0..9 (x=680, 92px×10, LeftToRight) 추가.
//! 행별 값 셀은 여기서 런타임 주입: 헤더 노드 #csb_hdr 를 통째 clone → 각 행에 넣고
//! 각 셀 #text 를 그 코치의 스탯값으로 채움(정렬은 clone 이 그대로 보존).
//! 행↔코치 바인딩 = 행의 #name 라벨 텍스트로 DB staff 이름 매칭.

use crate::config;
use crate::ui;
use mod_api::*;
use std::collections::HashMap;

// StaffStat 필드 순서 = 헤더 csb_h0..9 순서.
fn staff_stats(st: &Staff) -> [usize; 10] {
    let s = &st.stat;
    [
        s.banpick,
        s.strategy,
        s.negotiation,
        s.judge_ability,
        s.judge_potential,
        s.feedback,
        s.power_analysis,
        s.control_coaching,
        s.judgment_coaching,
        s.mental_coaching,
    ]
}

/// DB 전체 코치 이름→10스탯 맵. (코치진 화면 행 자체가 내 팀이라 팀필터 불요·이름매칭)
fn build_map(data: &ClientData) -> HashMap<String, [usize; 10]> {
    let mut m = HashMap::new();
    for sid in data.staff_ids() {
        if let Some(st) = data.staff(sid) {
            m.insert(st.name.to_string(), staff_stats(&st));
        }
    }
    m
}

/// 행에 값 셀 그룹(vp_csb_hdr) 보장 + 값/색 채움.
fn fill_row(row: &mut Node, tmpl: &Node, stats: &[usize; 10]) {
    if ui::find(row, "vp_csb_hdr").is_none() {
        let mut c = tmpl.clone();
        c.id = "vp_csb_hdr".to_string();
        row.child.push(c);
    }
    let Some(hdr) = ui::find_mut(row, "vp_csb_hdr") else { return };
    for i in 0..10 {
        let cell_id = format!("csb_h{i}");
        if let Some(cell) = ui::find_mut(hdr, &cell_id) {
            if let Some(lbl) = ui::find_mut(cell, "text") {
                let v = stats[i];
                ui::set_label(lbl, &v.to_string(), ui::stat_color(v));
            }
        }
    }
}

pub fn tick(scene: &mut Scene, ui_: &mut GameUI) {
    if !config::get().coaching {
        return;
    }
    // 코치진 화면 판정 + 헤더 템플릿 확보(우리 오버라이드가 넣은 csb_hdr).
    let Some(tmpl) = ui::find(&ui_.root, "csb_hdr").cloned() else {
        return;
    };
    // 데이터 맵(관리 씬에서만).
    let Scene::InGame { data } = scene else { return };
    let map = build_map(data);
    if map.is_empty() {
        return;
    }
    // 행 컨테이너 #contents 순회.
    let Some(contents) = ui::find_mut(&mut ui_.root, "contents") else {
        return;
    };
    for row in contents.child.iter_mut() {
        let Some(name) = ui::child_label_text(row, "name") else {
            continue;
        };
        let name = name.to_string();
        if let Some(stats) = map.get(&name) {
            fill_row(row, &tmpl, stats);
        }
    }
}
