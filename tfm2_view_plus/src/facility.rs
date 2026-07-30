//! facility — 상품 생산시설: 전체 추가 생산 / 전체 신규 생산(순수 SDK, game_core 타입드).
//! .ui 오버라이드가 버튼(restock_all_btn / produce_new_btn) + 수량팝업(restock_panel) 베이크.
//! 재현(daram2 규명): 클릭 → data.db_mut().teams.get_mut(player_team_id) →
//!   전체 추가생산 = 각 MerchandiseProduct.stock += 수량 (daram2 +0x10=stock 일치, 게임이 비용처리).
//! 수량 = config.facility_default_qty (원본은 공유 팝업; v1은 config 수량 — 팝업 연동은 후속).
//! ⚠ 신규생산(produce_new)= 신규 MerchandiseProduct 생성은 15필드 구성 필요 → 후속(TODO).
//! ⚠ stock 증가가 정상비용을 유발하는지는 인게임 검증 필요(daram2도 차감코드 없이 게임 파이프라인 의존).

use crate::config;
use crate::ui;
use mod_api::*;

fn restock_all(data: &ClientData, qty: usize) {
    let tid = data.player_team_id();
    let mut db = data.db_mut();
    if let Some(team) = db.teams.get_mut(&tid) {
        for p in team.merchandise_products.iter_mut() {
            p.stock = p.stock.saturating_add(qty);
        }
    }
}

pub fn tick(scene: &mut Scene, ui_: &mut GameUI) {
    if !config::get().facility {
        return;
    }
    // 시설(생산) 화면 판정 — 오버라이드가 넣은 버튼 존재.
    if ui::find(&ui_.root, "restock_all_btn").is_none() {
        return;
    }
    let restock = ui::clicked("restock_all_btn");
    let produce_new = ui::clicked("produce_new_btn");
    if !restock && !produce_new {
        return;
    }
    let qty = config::get().facility_default_qty as usize;
    let Scene::InGame { data } = scene else { return };
    if restock {
        restock_all(data, qty);
    }
    if produce_new {
        // TODO: 아직 없는 상품종류(팀 선수별 등) 신규 MerchandiseProduct 생성.
        //   product_type/athlete_id/sell_price/stock 등 15필드 구성 필요 — 인게임 확인 후.
    }
}
