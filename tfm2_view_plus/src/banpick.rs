//! banpick — 밴픽 뷰 플러스(순수 SDK). 레이아웃 대개편은 .ui 오버라이드로 완성(dll 코드 불요).
//! dll 폴리시 = 매프레임 update 훅에서 노드 source/color/좌표 조작(daram2 공통, 렌더훅 없음).
//!
//! ✅ 구현(확정): 화면감지 + 설정 토글 상태(5개) + 하단 패널 표시 토글.
//! ⬜ 런타임 검증 필요(정적분석서도 미확정 — daram2 dll 분석 결론):
//!   - 호버 배경 스플래시: 호버 감지 + bp_hover_bg 노드 image source 를 호버 챔프 illust 경로로 스왑
//!     (엔진이 경로로 PNG 로드) + illust 488MB 에셋 복사 필요.
//!   - 스킨/아트 순환(bp_skin_cycle/bp_illust_cycle): source 스왑 + 챔프별 영속(우리는 .cfg/별도파일).
//!     스킨의 인-매치 모델 반영 여부 = 미확증.
//!   - 스탯 레이더(호버 Lv6.5 8각): 프리미티브 노드 주입. Lv6.5 산출식·8각 지오메트리 미추출.
//!   - 이름 버프/너프 색: #name:label color 설정. 판정 소스 필드(champion_patch_statistics/
//!     pre_patch_data/head_to_head 중) 미확정.
//! 스탯 read 경로(확보): ClientData::db().champion_info(name) -> Arc<dyn ChampionInfo>.

use crate::config;
use crate::ui;
use mod_api::*;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

// 설정 토글 런타임 상태(config 초기값 → 토글 클릭으로 반전). 0=미초기화 표식.
static INIT: AtomicBool = AtomicBool::new(false);
static SHOW_PANEL: AtomicBool = AtomicBool::new(true);
static ILLUST_ALT: AtomicBool = AtomicBool::new(false);
static NAME_COLOR: AtomicBool = AtomicBool::new(true);
static RED_NOFLIP: AtomicBool = AtomicBool::new(false);
static HOVER_BG: AtomicBool = AtomicBool::new(true);

fn init_from_config() {
    if INIT.swap(true, Ordering::Relaxed) {
        return;
    }
    let c = config::get();
    SHOW_PANEL.store(c.banpick_bottom_panel, Ordering::Relaxed);
    NAME_COLOR.store(c.banpick_name_color, Ordering::Relaxed);
    HOVER_BG.store(c.banpick_hover_splash, Ordering::Relaxed);
}

pub fn tick(scene: &mut Scene, ui_: &mut GameUI) {
    if !config::get().banpick {
        return;
    }
    // 밴픽 화면 판정(오버라이드가 넣은 호버배경 노드).
    if ui::find(&ui_.root, "bp_hover_bg").is_none() {
        return;
    }
    init_from_config();

    // 설정 토글 클릭 → 상태 반전.
    if ui::clicked("bp_panel_toggle") {
        SHOW_PANEL.fetch_xor(true, Ordering::Relaxed);
    }
    if ui::clicked("bp_illust_toggle") {
        ILLUST_ALT.fetch_xor(true, Ordering::Relaxed);
    }
    if ui::clicked("bp_namecolor_toggle") {
        NAME_COLOR.fetch_xor(true, Ordering::Relaxed);
    }
    if ui::clicked("bp_redflip_toggle") {
        RED_NOFLIP.fetch_xor(true, Ordering::Relaxed);
    }
    if ui::clicked("bp_hoverbg_toggle") {
        HOVER_BG.fetch_xor(true, Ordering::Relaxed);
    }

    // ✅ 하단 스탯 패널 표시/숨김.
    let show_panel = SHOW_PANEL.load(Ordering::Relaxed);
    for id in ["bp_panel_bg", "bp_panel_topline", "bp_spacer"] {
        ui::set_visible_all(&mut ui_.root, id, show_panel);
    }

    // ⬜ 스킨/아트 순환·호버 스플래시·레이더·이름 버프너프색 = 런타임 검증 후 구현.
    //   호버배경 예시(구현 시): bp_hover_bg 노드의 image source 를 호버 챔프 illust 경로로 설정.
    //   스탯 예시: if let Scene::InGame{data}=scene { data.db().champion_info(name) ... }
    let _ = scene;
}
