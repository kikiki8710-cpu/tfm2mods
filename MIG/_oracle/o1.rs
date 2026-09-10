#![allow(unused, dead_code)]
use game_core::*;
fn main(){
    let pool = bumpalo::Bump::new();
    let setting: GameSetting = Default::default();
    let mw: MacroWeights = Default::default();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off,
    };
    println!("OK ctx built, is_line_phase? tutorial={:?}", ctx.tutorial);
}
