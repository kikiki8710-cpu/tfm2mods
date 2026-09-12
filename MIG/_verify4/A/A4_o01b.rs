#![allow(unused, dead_code, non_snake_case)]
//! 4차 배치A 오라클 — /specs[1]/open[0](ev5) : `Effect::expected_damage_target` 의 반환 단위
//!
//! IR 독해(_gcbc/g06.ll:52355~52846, effect.rs:91) 결론:
//!   let (ad, ap) = if target.ty.is_structure() { ty.expected_damage_structure(..).unwrap_or_else(|| ty.expected_damage(..)) }
//!                  else { ty.expected_damage(ctx, caster) };            // vt+0x30 / vt+0x28
//!   let thr = ty.expected_target_hp_ratio(ctx, caster);                  // vt+0x38
//!   if thr != 0 { ad += target.stat_cached.hp * thr / 100 }              // Entity+0x628
//!   if ad|ap == 0 { return 0 }
//!   get_damage(caster,target,ad,self.attack_type,AD) + get_damage(caster,target,ap,..,AP)
//! get_damage(utils.rs:97, MIR 전문) 최종식 = (dmg*100/(100+def)).max(1)
//!   def = target.stat_cached.defence(+0x630) [AD] / .magic_resistance(+0x638) [AP]
//!         펜 있으면 def = def * 100.saturating_sub(pen) / 100
//! ⟹ 반환값은 HP 와 같은 축의 절대 피해량.
//!
//! 검증 = 방어력 D 를 스윕해 관측값이 `floor(ad*100/(100+D)).max(1) + floor(ap*100/(100+M)).max(1)` 과
//!        칸 단위로 일치하는지 (ad/ap 는 D=0,M=0 관측에서 역산)
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000; s.height = 960000;
    s.respawn_tick = 300; s.respawn_growth = 30; s.respawn_growth_term = 1800;
    s.respawn_growth_per_level = 180; s.respawn_max = 2400;
    s.visible_distance = 130000; s.tick_per_second = 60; s.champion_radius = 10000;
    s.nexus_heal = 10; s.nexus_heal_2v2 = 10; s.nexus_heal_3v3 = 10;
    s.nexus_heal_tick = 24; s.nexus_heal_decay = 100;
    s.kill_exp = 30; s.kill_exp_growth = 30; s.assist_exp_ratio = 40;
    s.kill_gold = 300; s.assist_gold = 100; s.start_gold = 500; s.gold_per_second = 7;
    s.return_tick = 120; s.epic_minion_buff_duration = 5400; s.epic_minion_buff_range = 140000;
    s.tower_reduce_tick_2v2 = 5400; s.tower_reduce_tick_3v3 = 7200;
    s.gold_ratio_2v2 = 200; s.gold_ratio_3v3 = 150; s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999; s.tower_attack_disable_tick_2v2 = 10800;
    s.tower_attack_disable_tick_3v3 = 14400;
    s.well_damage = 600; s.well_damage_tick = 30; s.top_hp_regen_percent = 1;
    s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20;
    s.mid_exp_bonus = 20; s.bottom_gold_bonus = 20;
    s.support_gold_reduction = 15; s.support_exp_reduction = 30;
    s.stamina_zero_debuff_percent = 30;
    s
}

pub fn setting_ok(s: &GameSetting) -> bool {
    let ok = s.width != 0 && s.height != 0 && s.tick_per_second != 0 && s.champion_radius != 0;
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}\tvisible={}",
             ok, s.width, s.height, s.tick_per_second, s.champion_radius, s.visible_distance);
    ok
}

/// ⛔ init_tower/init_nexus 를 부르지 않는다(타워 2배 함정)
pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize {
        for p in 0..5usize {
            let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
            let mut st: AthleteStat = Default::default();
            st.judgement = 80; st.mental = 60;
            game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
            pid += 1;
        }
    }
    game.start_game(&mut rnd, ctx);
    game
}

fn ids(g: &Game, team: usize) -> Vec<usize> {
    g.world.champion_ids.iter().cloned()
        .filter(|id| matches!(g.world.entity.get(*id).map(|e| e.team), Some(TeamType::Player(t)) if t == team))
        .collect()
}

fn run_case(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext,
            label: &str, mkeff: &dyn Fn() -> Option<Effect>,
            atk: usize, mp: usize, hp: usize, def: usize, mr: usize, structure: bool) {
    let eff = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| mkeff())) {
        Ok(Some(e)) => e, _ => { println!("o01\t{}\tNO_EFFECT", label); return } };
    let mut g = mkgame(setting, ms, map, ctx);
    let a = ids(&g, 0); let b = ids(&g, 1);
    if a.is_empty() || b.is_empty() { println!("o01\tNO_CHAMP"); return }
    let (cid, tid) = (a[0], b[0]);
    // caster 스탯 주입
    if let Some(e) = g.world.entity.get_mut(cid) {
        e.stat_cached.attack = atk; e.stat_cached.magic_power = mp;
        e.stat_cached.hp = 3000; e.level = 5;
    }
    // target 스탯 주입
    if let Some(e) = g.world.entity.get_mut(tid) {
        e.stat_cached.hp = hp; e.stat_cached.defence = def; e.stat_cached.magic_resistance = mr;
    }
    let towerid = g.world.tower_ids.iter().cloned().next();
    let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, ctx);
    let caster = match g.world.entity.get(cid) { Some(e) => e, None => return };
    let target = if structure {
        match towerid.and_then(|t| g.world.entity.get(t)) { Some(e) => e, None => { println!("o01\tNO_TOWER"); return } }
    } else {
        match g.world.entity.get(tid) { Some(e) => e, None => return }
    };
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
        eff.expected_damage_target(ctx, caster as &dyn AbstractEntity, target)));
    println!("o01\t{}\tatk={}\tmp={}\thp={}\tdef={}\tmr={}\tstruct={}\tty={:?}\tret={:?}",
             label, atk, mp, target.stat_cached.hp, target.stat_cached.defence,
             target.stat_cached.magic_resistance, structure, target.ty.is_structure(), r);
}

fn main() {
    std::panic::set_hook(Box::new(|_| {}));
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
    {   // 무결성 지표
        let g = mkgame(&setting, &ms, &map, &ctx);
        let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx);
        let mut top = 0usize;
        for t in 0..2usize { for p in 0..5usize {
            if let Some(e) = cache.player_champion[t][p] {
                if game_core::is_top_side(&ctx, e.x, e.y) { top += 1; } } } }
        println!("towers\t{}\ttwin0={}\ttwin1={}", g.world.tower_ids.len(),
                 cache.twin_towers[0].len(), cache.twin_towers[1].len());
        println!("is_top_side_true\t{}/10\t(height=0 이면 10/10 로 붕괴한다)", top);
        println!("expect\ttowers=16 twin=2/2 is_top_side_true=8/10");
    }

    println!("\n#o01  Effect::expected_damage_target — 단위 확인");
    let sk = || SwordmanChampionInfo::default().skill().effect();
    let at = || SwordmanChampionInfo::default().attack().effect();
    let ex = || ExecutionerChampionInfo::default().skill().effect();
    let pm = || PyromancerChampionInfo::default().skill().effect();
    // (A) 방어력 스윕 — armor 공식 확인
    for d in [0usize, 25, 50, 100, 200, 400, 900] {
        run_case(&setting,&ms,&map,&ctx,"Swordman.skill",&sk, 1000, 1000, 2000, d, 0, false);
    }
    // (B) 마저 스윕
    for m in [0usize, 100, 300] {
        run_case(&setting,&ms,&map,&ctx,"Pyromancer.skill",&pm, 1000, 1000, 2000, 0, m, false);
    }
    // (C) 최대HP 스윕 — expected_target_hp_ratio 성분 확인
    for h in [1000usize, 2000, 4000, 8000] {
        run_case(&setting,&ms,&map,&ctx,"Executioner.skill",&ex, 1000, 1000, h, 0, 0, false);
        run_case(&setting,&ms,&map,&ctx,"Swordman.attack",&at, 1000, 1000, h, 0, 0, false);
    }
    // (D) 구조물(타워) 대상 — is_structure 게이트 / expected_damage_structure
    for d in [0usize, 100] {
        run_case(&setting,&ms,&map,&ctx,"Swordman.skill/STRUCT",&sk, 1000, 1000, 2000, d, 0, true);
        run_case(&setting,&ms,&map,&ctx,"Executioner.skill/STRUCT",&ex, 1000, 1000, 2000, d, 0, true);
    }
    // (E) 캐스터 공격력 스윕 — 선형성
    for a in [0usize, 500, 1000, 2000] {
        run_case(&setting,&ms,&map,&ctx,"Swordman.skill",&sk, a, 0, 2000, 0, 0, false);
    }
}
