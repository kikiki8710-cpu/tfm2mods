#![allow(unused, dead_code, non_snake_case)]
//! A5-O5 — specs[2] `AttackNexusPlan::sub_plan` 3분기 + sret 페이로드 바이트 · specs[4] 튜토리얼 게이트.
//! argv: <case>   case = in_heal_low | in_heal_full | outside | no_twin
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
                std::ptr::read_unaligned(p as *const i64),
                *p.add(8), *p.add(9), *p.add(10))
    }
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

    // ── specs[4] 튜토리얼 게이트: TutorialType::spawn_epic() 전 9값 ──
    let tts = [TutorialType::None, TutorialType::First, TutorialType::TopSolo, TutorialType::Bottom,
               TutorialType::MidSolo, TutorialType::MidBottom, TutorialType::JungleOnly,
               TutorialType::Line, TutorialType::Total];
    let mut s = String::new();
    for (i, t) in tts.iter().enumerate() {
        let tag = unsafe { *( t as *const TutorialType as *const u8) };
        s += &format!("{}:{}={} ", i, tag, t.spawn_epic());
    }
    println!("SPAWN_EPIC\t{}\t(명세: tutorial-1 <u 6 이면 false ⟹ 0/7/8 만 true)", s);
    println!("MORGARD_EXISTS\t{}\t(ctx.tutorial=None)", game_ai::plan_legacy::rule_scope::morgard_exists(&ctx));

    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let my_id = {
        let c = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        c.player_champion[0][0].unwrap().id
    };
    let f = map.fountains[0];
    println!("FOUNTAIN0\t{:?}", f);

    // 위치·HP 조작
    {
        let e = game.world.entity.get_mut(my_id).unwrap();
        match case.as_str() {
            "in_heal_low" => { e.x = f.0 + 1000; e.y = f.1 + 1000; e.hp = 500; e.stat_cached.hp = 999; }
            "in_heal_full" => { e.x = f.0 + 1000; e.y = f.1 + 1000; e.hp = 999; e.stat_cached.hp = 999; }
            _ => { e.x = 500000; e.y = 500000; e.hp = 500; e.stat_cached.hp = 999; }
        }
    }
    // no_twin: 적팀(1) 트윈타워 엔티티를 월드에서 제거
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
    let mut dbg: DebugFrameData = Default::default();

    let plan = AttackNexusPlan::new(1, LineType::Mid);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(3);
    let sp = plan.sub_plan(1, &mut rnd, player, &data, team_plan, &mut dbg);

    println!("CASE\t{}\tchamp=({},{})\thp={}/{}\ttwin1_len={}",
             case, champ.x, champ.y, champ.hp, champ.stat_cached.hp, cache.twin_towers[1].len());
    println!("SUBPLAN\t{}\t{:?}", dump_subplan(&sp), std::mem::discriminant(&sp));
    println!("SUBPLAN_DBG\t{:?}", sp);
}
