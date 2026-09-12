#![allow(unused, dead_code, non_snake_case)]
//! 7차 배치 C 오라클 ② — `chat_allowed` 의 **라인코드 게이트**(rule_scope.rs:113 `push_line_code_allowed`)
//! 를 2차원으로 뜬다. o7c §D 가 specs[12] knobs[7] 의 「code>=3 → 무조건 허용」과 어긋나
//! (TopSolo 에서 허용 코드가 {2,5,6,7}) 실제 규칙을 재구성하려는 것이다.
//!
//! 방법: `Chat::PlayCall(u8 @+0x1, u8 @+0x2, usize @+0x8)` 를 24B 날바이트로 조립해
//! (code=b1) × (b2) × tutorial 9종을 전수 평가한다.
//! 튜토리얼별로 **존재하는 라인**이 다르므로(Top/Mid/Bottom), 허용 패턴이
//! 「이 코드가 어느 라인을 가리키나」를 역으로 드러낸다.
use game_core::*;

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000; s.height = 960000; s.tick_per_second = 60; s.champion_radius = 10000;
    s.visible_distance = 130000; s.respawn_tick = 300; s.well_damage = 600;
    s
}

const TUTS: [TutorialType; 9] = [
    TutorialType::None, TutorialType::First, TutorialType::TopSolo, TutorialType::Bottom,
    TutorialType::MidSolo, TutorialType::MidBottom, TutorialType::JungleOnly,
    TutorialType::Line, TutorialType::Total];

unsafe fn mkchat(tag: u8, b1: u8, b2: u8) -> Chat {
    let mut raw = [0u8; 24];
    raw[0] = tag; raw[1] = b1; raw[2] = b2;
    std::mem::transmute::<[u8; 24], Chat>(raw)
}

fn main() {
    let setting = real_setting();
    println!("setting_ok\t{}\theight={}\ttps={}", setting.height != 0 && setting.tick_per_second != 0,
             setting.height, setting.tick_per_second);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let mkctx = |t: TutorialType| GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false, tutorial: t, trace_level: TraceLevel::Off,
    };

    for tag in [47u8, 48, 49] {
        println!("\n===== Chat tag {} — 행 b1(=+0x1, 0..15) / 열 b2(=+0x2, 0..7) =====", tag);
        for (ti, t) in TUTS.iter().enumerate() {
            let c = mkctx(*t);
            println!("-- tutorial {} {:?}", ti, t);
            for b1 in 0u8..16 {
                let mut row = String::new();
                for b2 in 0u8..8 {
                    let ch = unsafe { mkchat(tag, b1, b2) };
                    row.push(if game_ai::plan_legacy::rule_scope::chat_allowed(&c, &ch) { 'T' } else { '.' });
                }
                println!("   b1={:2}  {}", b1, row);
            }
        }
    }

    // 라인 존재 ↔ 코드 허용의 대응을 한눈에
    println!("\n===== 요약: tag47 에서 (b1,b2=0) 이 허용되는 tutorial 집합 =====");
    for b1 in 0u8..16 {
        let mut s = vec![];
        for (ti, t) in TUTS.iter().enumerate() {
            let c = mkctx(*t);
            let ch = unsafe { mkchat(47, b1, 0) };
            if game_ai::plan_legacy::rule_scope::chat_allowed(&c, &ch) { s.push(ti) }
        }
        println!("b1={:2}\tallow_tut={:?}", b1, s);
    }
    println!("\n참고: position_exists 집합 = Top{{0,2,7,8}} Jungle{{0,6,8}} Mid{{0,4,5,7,8}} Bottom/Support{{0,1,3,5,7,8}}");
    println!("DONE");
}
