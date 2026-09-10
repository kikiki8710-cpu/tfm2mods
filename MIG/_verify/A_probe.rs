#![allow(unused, dead_code)]
// 배치 A 반증용 오라클 — 담당 5함수의 "구성 가능한 하위 주장"만 실행으로 대조
use game_core::*;

fn main() {
    let setting: GameSetting = Default::default();
    let map = MapDef::moba(&setting);

    // --- 02 sub_plan: MapDef.fountains / nexus_pos 실값 ---
    for t in 0usize..2 {
        let f = map.fountain(t);
        println!("FOUNTAIN\t{}\t{}\t{}\t{}\t{}", t, f.0, f.1, f.2, f.3);
        let n = map.nexus_pos(t);
        println!("NEXUS\t{}\t{}\t{}", t, n.0, n.1);
    }

    // --- 04 handle_line_defense: LineType::get_start_position ---
    for (nm, l) in [("Top", LineType::Top), ("Mid", LineType::Mid), ("Bottom", LineType::Bottom)] {
        for t in 0usize..2 {
            let p = l.get_start_position(&setting, t);
            println!("START\t{}\t{}\t{}\t{}", nm, t, p.0, p.1);
        }
    }

    // --- 04: morgard_exists 의 튜토리얼 게이트 = TutorialType::spawn_epic ---
    for (nm, tt) in [
        ("None", TutorialType::None), ("First", TutorialType::First),
        ("TopSolo", TutorialType::TopSolo), ("Bottom", TutorialType::Bottom),
        ("MidSolo", TutorialType::MidSolo), ("MidBottom", TutorialType::MidBottom),
        ("JungleOnly", TutorialType::JungleOnly), ("Line", TutorialType::Line),
        ("Total", TutorialType::Total),
    ] {
        println!("SPAWN_EPIC\t{}\t{}\t{}", nm, tt as u8, tt.spawn_epic());
    }

    // --- 03 defensive_crisis: judge_accuracy 는 필드가 아니라 함수다 ---
    let st: AthleteStat = Default::default();
    let ap = AthleteParameter::new(&st);
    println!("JUDGE_ACC_DEFAULT\t{}\t{}", ap.judge_accuracy(), ap.judgement_base());
    println!("SIZE\tAthleteParameter\t{}", std::mem::size_of::<AthleteParameter>());
    println!("SIZE\tPlayerState\t{}", std::mem::size_of::<PlayerState>());

    // --- 01: EntityType 술어 3종의 인자 유무를 실행으로 확인 ---
    let nx = EntityType::Nexus;
    println!("PRED\tNexus.is_nexus\t{}", nx.is_nexus());
    println!("PRED\tNexus.is_tower\t{}", nx.is_tower());
    println!("PRED\tNexus.is_any_type_minion\t{}", nx.is_any_type_minion());
    println!("PRED\tNexus.is_any_jungle\t{}", nx.is_any_jungle());
    println!("PRED\tNexus.is_jungle(0)\t{}", nx.is_jungle(0));
    println!("PRED\tNexus.is_minion(Top)\t{}", nx.is_minion(LineType::Top));

    // --- 각종 크기·태그 ---
    println!("SIZE\tLineType\t{}", std::mem::size_of::<LineType>());
    println!("TAG\tLineType::Top\t{}", LineType::Top as u8);
    println!("TAG\tLineType::Mid\t{}", LineType::Mid as u8);
    println!("TAG\tLineType::Bottom\t{}", LineType::Bottom as u8);
    println!("DEFAULT\tLineType\t{:?}", <LineType as Default>::default());
    println!("SIZE\tMapDef\t{}", std::mem::size_of::<MapDef>());
    println!("SIZE\tGameSetting\t{}", std::mem::size_of::<GameSetting>());
    println!("SETTING\ttick_per_second\t{}", setting.tick_per_second);
}
