#![allow(unused, dead_code, non_snake_case)]
//! 4차 배치A 오라클 — /specs[1]/open[0](ev5) : `Effect::expected_damage_target` 의 반환 단위
//!
//! ★핵심 우회: 실전 ChampionInfo 의 `::default()` 는 **액션 파라미터까지 0** 이라
//!   effect 로는 ad|ap 가 0 이 되어 조기반환 0 만 나온다(A4_o01b.tsv 26/26 = 0).
//!   `AttackEffect`(72B) 는 **전 필드 pub** 이므로 손으로 만들어 꽂으면 된다.
//!
//! IR 독해 예측식 (검증 대상):
//!   AttackEffect::expected_damage  (_gcbc/g13.ll:144578)
//!     ad0 = damage + attack_ratio*caster_stat.attack/100 + hp_ratio*caster_stat.hp/100 ; ap0 = 0
//!   AttackEffect::expected_target_hp_ratio = self.target_hp_ratio (self+0x28)
//!   Effect::expected_damage_target (_gcbc/g06.ll:52355, effect.rs:91)
//!     ad = ad0 + (thr!=0 ? target.stat_cached.hp*thr/100 : 0)
//!     if ad|ap == 0 { return 0 }
//!     get_damage(caster,target,ad,attack_type,AD) + get_damage(caster,target,ap,attack_type,AP)
//!   get_damage (utils.rs:97, MIR 전문) = (dmg*100/(100+def)).max(1)
//!     def = target.stat_cached.defence [AD] / .magic_resistance [AP]   (펜/증폭 버프는 default 0)
//!   ⟹ pred = max(ad*100/(100+def),1) + max(0*100/(100+mr),1) = ad*100/(100+def) + 1
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

fn mkeff(damage: usize, attack_ratio: usize, hp_ratio: usize, target_hp_ratio: usize,
         atk_ty: AttackType) -> Effect {
    let ae = AttackEffect {
        ty: AttackEffectType::EnemyTarget,
        damage, attack_ratio, hp_ratio, target_hp_ratio,
        cc_damage: 0, cc_damage_attack_ratio: 0, shared: false,
    };
    Effect {
        ty: Arc::new(ae),
        range: 100000, growth_range: 0, start_timing: 0,
        target: CastingTarget::Enemy, attack_type: atk_ty, casting: CastingType::Targeting,
    }
}

struct Case { dmg: usize, ar: usize, hr: usize, thr: usize,
              catk: usize, chp: usize, thp: usize, def: usize, mr: usize, st: bool }

fn run(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext, c: &Case,
       bad: &mut usize, tot: &mut usize) {
    let eff = mkeff(c.dmg, c.ar, c.hr, c.thr, AttackType::Skill);
    let mut g = mkgame(setting, ms, map, ctx);
    let a = ids(&g, 0); let b = ids(&g, 1);
    let (cid, tid) = (a[0], b[0]);
    if let Some(e) = g.world.entity.get_mut(cid) {
        e.stat_cached.attack = c.catk; e.stat_cached.hp = c.chp; e.level = 5;
    }
    if let Some(e) = g.world.entity.get_mut(tid) {
        e.stat_cached.hp = c.thp; e.stat_cached.defence = c.def; e.stat_cached.magic_resistance = c.mr;
    }
    let towerid = g.world.tower_ids.iter().cloned().next();
    let caster = g.world.entity.get(cid).unwrap();
    let target = if c.st { g.world.entity.get(towerid.unwrap()).unwrap() }
                 else   { g.world.entity.get(tid).unwrap() };
    // 예측
    let ad0 = c.dmg + c.ar * caster.stat_cached.attack / 100 + c.hr * caster.stat_cached.hp / 100;
    let thr = c.thr;
    let ad = if thr != 0 { ad0 + target.stat_cached.hp * thr / 100 } else { ad0 };
    let pred: usize = if (ad | 0) == 0 { 0 } else {
        std::cmp::max(ad * 100 / (100 + target.stat_cached.defence), 1)
        + std::cmp::max(0 * 100 / (100 + target.stat_cached.magic_resistance), 1)
    };
    let got = eff.expected_damage_target(ctx, caster as &dyn AbstractEntity, target);
    *tot += 1;
    let ok = got == pred;
    if !ok { *bad += 1; }
    println!("o01c\t{}\tdmg={}\tar={}\thr={}\tthr={}\tcatk={}\tchp={}\tthp={}\tdef={}\tmr={}\tstruct={}({})\tpred={}\tgot={}",
             if ok {"MATCH"} else {"MISMATCH"}, c.dmg, c.ar, c.hr, c.thr,
             caster.stat_cached.attack, caster.stat_cached.hp,
             target.stat_cached.hp, target.stat_cached.defence, target.stat_cached.magic_resistance,
             c.st, target.ty.is_structure(), pred, got);
}

fn main() {
    let setting = real_setting();
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}\tvisible={}",
             setting.width != 0 && setting.height != 0 && setting.tick_per_second != 0 && setting.champion_radius != 0,
             setting.width, setting.height, setting.tick_per_second, setting.champion_radius, setting.visible_distance);
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
    {
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

    let mut bad = 0usize; let mut tot = 0usize;
    println!("\n#o01c  AttackEffect 수동 구성 — 예측식 대조");
    // (A) 방어력 스윕
    for d in [0usize, 25, 50, 100, 200, 400, 900, 1900] {
        run(&setting,&ms,&map,&ctx,&Case{dmg:500,ar:0,hr:0,thr:0,catk:0,chp:0,thp:2000,def:d,mr:0,st:false}, &mut bad,&mut tot);
    }
    // (B) attack_ratio x caster.attack
    for (ar, catk) in [(100usize,300usize),(50,1000),(200,777),(0,1000)] {
        run(&setting,&ms,&map,&ctx,&Case{dmg:0,ar,hr:0,thr:0,catk,chp:0,thp:2000,def:0,mr:0,st:false}, &mut bad,&mut tot);
    }
    // (C) hp_ratio x caster.hp
    for (hr, chp) in [(10usize,3000usize),(3,1234),(0,3000)] {
        run(&setting,&ms,&map,&ctx,&Case{dmg:0,ar:0,hr,thr:0,catk:0,chp,thp:2000,def:0,mr:0,st:false}, &mut bad,&mut tot);
    }
    // (D) target_hp_ratio x target.max_hp  (thr 성분)
    for (thr, thp) in [(10usize,2000usize),(7,3333),(25,1000),(0,2000)] {
        run(&setting,&ms,&map,&ctx,&Case{dmg:100,ar:0,hr:0,thr,catk:0,chp:0,thp,def:0,mr:0,st:false}, &mut bad,&mut tot);
    }
    // (E) 마저 (ap=0 이므로 항상 +1 이어야 한다)
    for m in [0usize, 100, 500] {
        run(&setting,&ms,&map,&ctx,&Case{dmg:500,ar:0,hr:0,thr:0,catk:0,chp:0,thp:2000,def:0,mr:m,st:false}, &mut bad,&mut tot);
    }
    // (F) ad|ap == 0 조기반환
    for _ in 0..1 {
        run(&setting,&ms,&map,&ctx,&Case{dmg:0,ar:0,hr:0,thr:0,catk:1000,chp:3000,thp:2000,def:0,mr:0,st:false}, &mut bad,&mut tot);
    }
    // (G) 구조물(타워) 대상 — is_structure 게이트 + expected_damage_structure 폴백 여부
    for d in [0usize, 100] {
        run(&setting,&ms,&map,&ctx,&Case{dmg:500,ar:0,hr:0,thr:0,catk:0,chp:0,thp:2000,def:d,mr:0,st:true}, &mut bad,&mut tot);
    }
    // (H) thr 이 구조물에도 붙는가
    run(&setting,&ms,&map,&ctx,&Case{dmg:100,ar:0,hr:0,thr:50,catk:0,chp:0,thp:2000,def:0,mr:0,st:true}, &mut bad,&mut tot);
    println!("\no01c_summary\ttotal={}\tmismatch={}", tot, bad);
}
