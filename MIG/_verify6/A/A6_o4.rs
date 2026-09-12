#![allow(unused, dead_code, non_snake_case)]
//! A6-O4 — `specs[2] AttackNexusPlan::sub_plan` 3분기 + sret 바이트(5차 A5_o5 와 같은 수법, 재확인)
//! 확장분:
//!   (N1) **version 0..=8 전값 스윕** — `open[0]`(version 이 무엇을 게이트하나)의 *이 함수 범위* 답을
//!        실행으로 못박는다(= 이 함수에선 어떤 version 에서도 결과가 같다).
//!   (N2) `_team_plan`·`_debug`·`rnd` 가 호출로 바뀌는지 **바이트 diff**(수법 ⓒ).
//!   (N3) `specs[4] open[0]` 잔여 물음 — `PlayerState::strategy(rnd, game)` 가 `rnd` 를 소비하는가.
//! argv: <case>  case = in_heal_low | in_heal_full | outside | no_twin
use game_core::*;
use rand::SeedableRng;
use std::mem::MaybeUninit;
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

use game_ai::plan_legacy::old::AttackNexusPlan;
use game_ai::plan_legacy::sub_plan::SubPlan;

fn dump_subplan(sp: &SubPlan) -> String {
    let p = sp as *const SubPlan as *const u8;
    unsafe {
        format!("tag={} b8={} b9={} b10={}",
                std::ptr::read_unaligned(p as *const i64), *p.add(8), *p.add(9), *p.add(10))
    }
}
fn bytes<T>(t: &T) -> Vec<u8> {
    unsafe { std::slice::from_raw_parts(t as *const T as *const u8, std::mem::size_of::<T>()).to_vec() }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let case: String = a.get(1).cloned().unwrap_or("outside".into());

    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off,
    };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let my_id = {
        let c = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        c.player_champion[0][0].unwrap().id
    };
    let f = map.fountains[0];
    {
        let e = game.world.entity.get_mut(my_id).unwrap();
        match case.as_str() {
            "in_heal_low" => { e.x = f.0 + 1000; e.y = f.1 + 1000; e.hp = 500; e.stat_cached.hp = 999; }
            "in_heal_full" => { e.x = f.0 + 1000; e.y = f.1 + 1000; e.hp = 999; e.stat_cached.hp = 999; }
            _ => { e.x = 500000; e.y = 500000; e.hp = 500; e.stat_cached.hp = 999; }
        }
    }
    if case == "no_twin" {
        let mut kill: Vec<usize> = Vec::new();
        {
            let c = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            for e in c.twin_towers[1].iter() { kill.push(e.id); }
        }
        for id in kill { game.world.remove_entity(id); }
    }

    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
    let champ: &Entity = cache.player_champion[0][0].unwrap();

    let ztp: MaybeUninit<game_ai::plan_legacy::team_plan::TeamPlan> = MaybeUninit::zeroed();
    let team_plan = unsafe { ztp.assume_init_ref() };

    println!("CASE\t{}\tchamp=({},{})\thp={}/{}\ttwin1_len={}\tsetting_ok={}",
             case, champ.x, champ.y, champ.hp, champ.stat_cached.hp, cache.twin_towers[1].len(), ok);

    // ── (N1)(N2) version 스윕 + &mut 인자 바이트 diff ──────────────────
    let mut first = String::new();
    let mut allsame = true;
    for ver in 0..=8usize {
        let plan = AttackNexusPlan::new(1, LineType::Mid);
        let mut rnd = rand::rngs::StdRng::seed_from_u64(3);
        let mut dbg: DebugFrameData = Default::default();
        let rb = bytes(&rnd);
        let db = bytes(&dbg);
        let sp = plan.sub_plan(ver, &mut rnd, player, &data, team_plan, &mut dbg);
        let ra = bytes(&rnd);
        let da = bytes(&dbg);
        let s = format!("{}|{:?}", dump_subplan(&sp), sp);
        if ver == 0 { first = s.clone(); } else if s != first { allsame = false; }
        if ver == 0 || ver == 8 {
            println!("SUBPLAN\tver={}\t{}\t{:?}\trnd_changed={}\tdebug_changed={}",
                     ver, dump_subplan(&sp), std::mem::discriminant(&sp), rb != ra, db != da);
            println!("SUBPLAN_DBG\tver={}\t{:?}", ver, sp);
        }
    }
    println!("SUBPLAN_VERSION\tv0..8_all_same={}\t{}", allsame,
             if allsame { "명세 sig.params[1](version 분기 없음) 유지" } else { "**명세 반증**" });

    // ── (N3) specs[4] open[0] 잔여 물음: strategy 가 rnd 를 소비하나 ────
    {
        let mut rnd = rand::rngs::StdRng::seed_from_u64(3);
        let rb = bytes(&rnd);
        let st = player.strategy(&mut rnd, &game as &dyn AbstractGame);
        let ra = bytes(&rnd);
        println!("STRATEGY\trnd_state_changed={}\tmorgard_defense={:?}\t(명세 specs[4] mem[21] Strategy+0xe)",
                 rb != ra, st.morgard_defense);
        let sb = &st as *const Strategy as usize;
        println!("STRATEGY\tmorgard_defense_offset=0x{:x}\t(명세 0xe)",
                 (&st.morgard_defense as *const _ as usize) - sb);
    }
}
