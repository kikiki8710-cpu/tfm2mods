#![allow(unused, dead_code, non_snake_case)]
//! 17차 배치C 오라클 2 — `MapDef::camp_pos(map, JungleType, bool)` 의 bool 인자 의미(#50 open[3] · #53 open[6] · #54 open[6]).
//! camp_pos 는 TLS 메모(CAMP_POS_MEMO)를 쓰지만 키가 (JungleType,bool) 이라 한 프로세스에서 전 조합을 재도 된다(같은 맵).
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify17/C/oracle/v17C_o2.rs
use game_core::*;
#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;
fn main() {
    let setting = real_setting();
    setting_ok(&setting);
    let map = MapDef::moba(&setting);
    for j in [JungleType::Rhino, JungleType::Mushroom, JungleType::Stump, JungleType::Bee, JungleType::Morgard, JungleType::Serpen] {
        let a = MapDef::camp_pos(&map, j, true);
        let b = MapDef::camp_pos(&map, j, false);
        println!("camp_pos\t{:?}\tside_true(team0)={:?}\tside_false(team1)={:?}\tmirror(w-x,h-y)={:?}\tsame={}",
                 j, a, b, (setting.width - a.0, setting.height - a.1), a == b);
    }
}
