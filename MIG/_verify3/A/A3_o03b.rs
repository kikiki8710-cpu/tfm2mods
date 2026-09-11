#![allow(unused, dead_code, non_snake_case)]
// 3차 배치A 오라클 2 — 03 defensive_crisis 의 cc_threat 를 실제로 true 로 만들고
//   슬롯 게이트(쿨 <= tps / level>2 / level>4)를 진리표로 확인한다.
//   ⚠BRIEF §3① : start_game 뒤 init_tower/init_nexus 재호출 금지(타워 2배 오염).
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext,
          enemy_champ: u8) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = if t == 1 {
            match enemy_champ {
                1 => Arc::new(ExecutionerChampionInfo::default()),
                2 => Arc::new(WerewolfChampionInfo::default()),   // CC = skill2 만
                3 => Arc::new(WindMageChampionInfo::default()),    // CC = ult 만
                _ => Arc::new(SwordmanChampionInfo::default()),
            }
        } else { Arc::new(SwordmanChampionInfo::default()) };
        let mut st: AthleteStat = Default::default();
        st.judgement = 80; st.mental = 60;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, ctx);
    game
}

fn main() {
    let pool = bumpalo::Bump::new();
    let mut setting: GameSetting = Default::default();
    setting.tick_per_second = 60;
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
    println!("case	champ	lvl	sk_cd	sk2_cd	ult_cd	has_sk	has_sk2	has_ult	near	die_imminent	cc_threat");
    // enemy_champ: 0 Swordman / 1 Executioner(skill cc=Some) / 2 Lancer(skill cc=Some) / 3 Knight(skill cc=Some(0))
    for ec in [0u8, 1, 2, 3] {
      for &(lvl, sk, sk2, ult) in [(1usize, 0usize, 0usize, 0usize),
                                   (2, 0, 0, 0), (3, 0, 0, 0), (4, 0, 0, 0), (5, 0, 0, 0),
                                   (5, 61, 61, 61), (5, 60, 60, 60), (5, 61, 60, 60),
                                   (5, 60, 61, 61), (5, 61, 61, 60), (5, 61, 60, 61)].iter() {
        let mut g = mkgame(&setting, &ms, &map, &ctx, ec);
        // 아군(팀0) Top 챔피언 = target, 적팀 챔피언들을 그 옆에 붙인다
        let my: Vec<usize> = g.world.champion_ids.iter().cloned()
            .filter(|id| matches!(g.world.entity.get(*id).map(|e| e.team), Some(TeamType::Player(0)))).collect();
        let en: Vec<usize> = g.world.champion_ids.iter().cloned()
            .filter(|id| matches!(g.world.entity.get(*id).map(|e| e.team), Some(TeamType::Player(1)))).collect();
        let tid = my[0];
        let (tx, ty) = { let e = g.world.entity.get(tid).unwrap(); (e.x, e.y) };
        let mut has = (false, false, false);
        for id in en.iter() {
            if let Some(e) = g.world.entity.get_mut(*id) {
                e.x = tx + 1000; e.y = ty;
                e.level = lvl;
                if let EntityType::Champion(c) = &mut e.ty {
                    c.skill_cooldown = sk; c.skill2_cooldown = sk2; c.ult_cooldown = ult;
                }
                has = (e.skill_effect.is_some(), e.skill2_effect.is_some(), e.ult_effect.is_some());
            }
        }
        let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
        for b in bb.iter_mut() {
            b.last_visible = [usize::MAX; 5]; b.last_reveal_tick = [usize::MAX; 5];
            b.last_reveal_hidden_span = [0; 5];
        }
        let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx);
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut dbg: DebugFrameData = Default::default();
        let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
        let ps = g.get_player_by_position(0, Position::Top).unwrap();
        let tgt = cache.player_champion[0][0].unwrap();
        let r = game_ai::defensive_crisis(3, &mut rnd, ps, &data, tgt, &mut dbg);
        println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            "case", ec, lvl, sk, sk2, ult, has.0, has.1, has.2, en.len(), r.die_imminent, r.cc_threat);
      }
    }
}
