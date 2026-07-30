// 예제(참고용, 컴파일 대상 아님): ui_kit 으로 커스텀 드롭다운 만들기.
// ===========================================================================
// [1] 짝이 되는 .ui — mods/<MOD>/ui/layout/<화면>.ui 의 원본에 아래 컨테이너 주입,
//     mod.override_info 로 그 레이아웃 override. (노드는 .ui 에 *미리* 선언해야 렌더됨)
//
//   #dd_root:empty {
//     x:1500 y:64 width:200 height:40;
//     #dd_header:color {                         // 클릭=펼침 토글
//       @"asset/base/style/main#tertiary_button";
//       width:200 height:40;
//       #dd_header_label:label {                 // 선택값 표시
//         @"asset/base/style/main#bold_label";
//         width:200 height:40 size:18 align_x:Center align_y:Center text:"1배속";
//       }
//     }
//     #dd_panel:empty {                          // 펼침 목록(visible 토글)
//       y:40 width:200 height:120; visible:false;
//       child_type: TopToBottom { spacing:2px }
//       #dd_item0:color_selectable { @"asset/base/style/main#strategy_option"; width:200 height:38 text:"1배속"; }
//       #dd_item1:color_selectable { @"asset/base/style/main#strategy_option"; width:200 height:38 text:"2배속"; }
//       #dd_item2:color_selectable { @"asset/base/style/main#strategy_option"; width:200 height:38 text:"3배속"; }
//     }
//   }
//
// [2] 코드 — ui_kit 의 Dropdown 타입으로 끝.
// ===========================================================================
use mod_api::*;
use std::sync::atomic::{AtomicBool, AtomicUsize};
#[path = "C:/tfm2mods/ui_kit/ui_kit.rs"]
mod ui_kit;
use ui_kit::*;

const ITEMS: [&str; 3] = ["1배속", "2배속", "3배속"];
const IDS: [&str; 3] = ["dd_item0", "dd_item1", "dd_item2"];
static OPEN: AtomicBool = AtomicBool::new(false);
static SEL: AtomicUsize = AtomicUsize::new(0);
static FH: AtomicUsize = AtomicUsize::new(usize::MAX);

static DD: Dropdown = Dropdown {
    header_id: "dd_header",
    header_label_id: "dd_header_label",
    panel_id: "dd_panel",
    item_ids: &IDS,
    items: &ITEMS,
    open: &OPEN,
    sel: &SEL,
};

struct M;
impl ModExtension for M {
    fn post_update(&self, scene: &mut Scene, ui: &mut GameUI, _a: &mut Assets, _d: f32) {
        let Scene::InGame { .. } = scene else { return; };

        DD.update(ui, &FH);                 // 펼침/선택/라벨 전부 처리
        let chosen = DD.selected();         // 선택 인덱스 (게임 로직에 사용)
        let _ = chosen;
    }
}
fn init(_c: &GameCtx) -> ModRegistration {
    let mut r = ModRegistration::new("dropdown_example");
    r.set_extension(M);
    r
}
declare_mod!(init);
