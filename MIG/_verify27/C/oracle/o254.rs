#![allow(unused, dead_code, non_snake_case)]
//! 27차 배치C 오라클 — #254 noncombat_steroid_window (hidden define, m10.ll:36654) — `#[link_name]` 직접 호출
//! 명세 logic 을 독립 재구현(모델)해 게임 함수 반환 {i8,i8} 와 바이트 단위로 대조한다(수법 ⓓ).
//! 사용: o254.exe <case>  (케이스당 프로세스 1개 — TLS 접점 0 이지만 규칙대로)
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

extern "Rust" {
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai10buff_value24noncombat_steroid_window"]
    fn nsw(player: *const PlayerState, data: *const OperationData, caster: *const Entity, bene: *const Entity)
        -> Option<game_ai::NoncombatSteroidWindow>;
}

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000; s.height = 960000; s.respawn_tick = 300; s.respawn_growth = 30; s.respawn_growth_term = 1800;
    s.respawn_growth_per_level = 180; s.respawn_max = 2400; s.visible_distance = 130000; s.tick_per_second = 60;
    s.champion_radius = 10000; s.nexus_heal = 10; s.nexus_heal_2v2 = 10; s.nexus_heal_3v3 = 10; s.nexus_heal_tick = 24;
    s.nexus_heal_decay = 100; s.kill_exp = 30; s.kill_exp_growth = 30; s.assist_exp_ratio = 40; s.kill_gold = 300;
    s.assist_gold = 100; s.start_gold = 500; s.gold_per_second = 7; s.return_tick = 120; s.epic_minion_buff_duration = 5400;
    s.epic_minion_buff_range = 140000; s.tower_reduce_tick_2v2 = 5400; s.tower_reduce_tick_3v3 = 7200; s.gold_ratio_2v2 = 200;
    s.gold_ratio_3v3 = 150; s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150; s.tower_attack_disable_tick = 9999999;
    s.tower_attack_disable_tick_2v2 = 10800; s.tower_attack_disable_tick_3v3 = 14400; s.well_damage = 600; s.well_damage_tick = 30;
    s.top_hp_regen_percent = 1; s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20; s.mid_exp_bonus = 20;
    s.bottom_gold_bonus = 20; s.support_gold_reduction = 15; s.support_exp_reduction = 30; s.stamina_zero_debuff_percent = 30;
    s
}
pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default(); st.judgement = 80; st.mental = 60;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new())); pid += 1;
    } }
    game.start_game(&mut rnd, ctx);
    game
}

unsafe fn p64(base: *mut u8, off: usize, v: i64) { std::ptr::write_volatile(base.add(off) as *mut i64, v); }
unsafe fn p32(base: *mut u8, off: usize, v: i32) { std::ptr::write_volatile(base.add(off) as *mut i32, v); }
unsafe fn set_xy(e: *mut Entity, x: u64, y: u64) { p64(e as *mut u8, 0x660, x as i64); p64(e as *mut u8, 0x668, y as i64); }

// ── 모델(명세 logic 독립 재구현) ─────────────────────────────────────────
fn engage(e: &Entity) -> u64 {
    e.attack_effect.as_ref().map(|ef| ef.range(e)).unwrap_or(0) + e.radius() as u64 + (e.stat_cached.move_speed as u64) * 120
}
fn model(player: &PlayerState, data: &OperationData, cache: &AbstractGameWithCache, bb: &[Blackboard; 2],
         game: &dyn AbstractGame, caster: &Entity, bene: &Entity) -> Option<(bool, bool)> {
    let team = player.info.team;
    let enemy = 1 - team;
    let be = engage(bene); let ce = engage(caster);
    let enemy_near = cache.iter_champions(enemy).any(|c| {
        let rb = be + c.radius() as u64; let rc = ce + c.radius() as u64;
        (c.distance_sq(bene) <= rb * rb || c.distance_sq(caster) <= rc * rc)
            && bb[enemy].is_recent_visible(game, player, c)
    });
    if enemy_near { return None; }
    let hits = |tid: usize| -> bool {
        cache.iter_champions(team).enumerate().any(|(pi, ally)| {
            (ally.id == bene.id || ally.distance_sq(bene) <= 3600000000)
                && match bb[team].small_actions[pi] {
                    Some(SmallAction::Attack { target_id }) | Some(SmallAction::Skill { target_id })
                    | Some(SmallAction::Skill2 { target_id }) | Some(SmallAction::Ult { target_id })
                    | Some(SmallAction::Trace { target_id }) | Some(SmallAction::Around { target_id }) => target_id == tid,
                    _ => false,
                }
        })
    };
    let atk = bene.attack_effect.as_ref()?;
    let move_allow = (bene.stat_cached.move_speed as u64) * 30;
    let in_window = |t: &Entity| -> bool {
        if !hits(t.id) { return false; }
        let range = atk.range(bene) + atk.range_adjust(bene, t) + bene.radius() as u64 + t.radius() as u64 + move_allow;
        bene.distance_sq(t) <= range * range
    };
    if cache.jungles.iter().any(|e| e.can_target() && in_window(e)) { return Some((true, false)); }
    if cache.iter_towers_without_nexus(enemy).any(|t| t.can_target() && in_window(t)) { return Some((false, false)); }
    if let Some(n) = cache.nexus[enemy] { if n.can_target() && in_window(n) { return Some((false, false)); } }
    if cache.iter_minions(enemy).any(|m| m.can_target() && in_window(m)) { return Some((false, true)); }
    None
}
fn window_range(bene: &Entity, t: &Entity) -> u64 {
    let atk = bene.attack_effect.as_ref().unwrap();
    atk.range(bene) + atk.range_adjust(bene, t) + bene.radius() as u64 + t.radius() as u64 + (bene.stat_cached.move_speed as u64) * 30
}

fn main() {
    if std::env::args().count() > 99 { let _ = game_ai::champion_hp_value as *const (); }
    let case: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let setting = real_setting();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms, map: &map,
        champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    if case >= 20 {
        // 정글 캠프·미니언은 start_game 직후엔 없다 — 틱을 돌려 스폰시킨다(함정 ⑧).
        let mut rnd2 = rand::rngs::StdRng::seed_from_u64(11);
        let mut n = 0usize;
        loop {
            game.run_tick(&ctx, &mut rnd2, &mut None);
            n += 1;
            let c = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let nj = c.jungles.len(); let nm = c.iter_minions(1).count();
            if (nj > 0 && nm > 0) || n >= 4000 { println!("ran_ticks\t{}\tjungles={}\tminions1={}", n, nj, nm); break; }
        }
    }
    let tick = 1000usize + game.tick();
    game.set_tick(tick);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];

    let player: &PlayerState = game.get_player_by_position(0, Position::Mid).expect("player");
    let bene: &Entity = cache.player_champion[0][2].expect("bene");
    let mut caster: &Entity = bene;
    let ally0: &Entity = cache.player_champion[0][0].expect("ally0");
    let enemy: &Entity = cache.player_champion[1][2].expect("enemy");
    let epos = game.get_player_by_position(1, Position::Mid).expect("ep").info.position as usize;
    let tower: &Entity = cache.iter_towers_without_nexus(1).filter(|t| t.can_target()).next().expect("tower");
    let nexus: &Entity = cache.nexus[1].expect("nexus");
    let jungle: Option<&Entity> = cache.jungles.iter().copied().filter(|e| e.can_target()).next();
    println!("setup\ttick={}\tjungles={}\ttowers1={}\tminions1={}\tbene.id={}\ttower.id={}\tnexus.id={}\tbene_atk={}\tms={}",
             game.tick(), cache.jungles.len(), cache.iter_towers_without_nexus(1).count(), cache.iter_minions(1).count(),
             bene.id, tower.id, nexus.id, bene.attack_effect.is_some(), bene.stat_cached.move_speed);
    let bp = bene as *const Entity as *mut Entity;
    let ep = enemy as *const Entity as *mut Entity;
    let a0 = ally0 as *const Entity as *mut Entity;
    unsafe {
        let r_t = window_range(bene, tower);
        match case {
            0 => {}
            1 => { set_xy(bp, tower.x + r_t / 2, tower.y); bb[0].small_actions[2] = Some(SmallAction::Attack { target_id: tower.id }); }
            2 => { set_xy(bp, tower.x + r_t / 2, tower.y); bb[0].small_actions[2] = Some(SmallAction::Attack { target_id: tower.id });
                   set_xy(ep, tower.x + r_t / 2 + 5000, tower.y); bb[1].last_visible[epos] = tick - 10; }
            3 => { set_xy(bp, tower.x + r_t / 2, tower.y); bb[0].small_actions[2] = Some(SmallAction::Attack { target_id: tower.id });
                   set_xy(ep, tower.x + r_t / 2 + 5000, tower.y); bb[1].last_visible[epos] = 0; }
            4 => { if let Some(j) = jungle { let r = window_range(bene, j); set_xy(bp, j.x + r / 2, j.y);
                   bb[0].small_actions[2] = Some(SmallAction::Attack { target_id: j.id }); } }
            5 => { set_xy(bp, tower.x + r_t / 2, tower.y); bb[0].small_actions[0] = Some(SmallAction::Attack { target_id: tower.id }); }
            6 => { set_xy(bp, tower.x + r_t / 2, tower.y); bb[0].small_actions[0] = Some(SmallAction::Attack { target_id: tower.id });
                   set_xy(a0, tower.x + r_t / 2 + 50000, tower.y); }
            7 => { set_xy(bp, tower.x + r_t / 2, tower.y); bb[0].small_actions[2] = Some(SmallAction::Around { target_id: tower.id }); }
            8 => { set_xy(bp, tower.x + r_t / 2, tower.y); bb[0].small_actions[2] = Some(SmallAction::RunAway); }
            9 => { set_xy(bp, tower.x + r_t / 2, tower.y); bb[0].small_actions[2] = Some(SmallAction::Positioning { x: tower.x, y: tower.y }); }
            10 => { set_xy(bp, tower.x + r_t + 1, tower.y); bb[0].small_actions[2] = Some(SmallAction::Attack { target_id: tower.id }); }
            11 => { set_xy(bp, tower.x + r_t, tower.y); bb[0].small_actions[2] = Some(SmallAction::Attack { target_id: tower.id }); }
            12 => { set_xy(bp, tower.x + r_t / 2, tower.y); bb[0].small_actions[2] = Some(SmallAction::Attack { target_id: tower.id });
                    caster = ally0; set_xy(ep, tower.x + r_t / 2 + 5000, tower.y); bb[1].last_visible[epos] = tick - 10; }
            13 => { set_xy(bp, tower.x + r_t / 2, tower.y); bb[0].small_actions[2] = Some(SmallAction::Attack { target_id: tower.id });
                    caster = ally0; set_xy(ep, ally0.x + 5000, ally0.y); bb[1].last_visible[epos] = tick - 10; }
            14 => { let r = window_range(bene, nexus); set_xy(bp, nexus.x + r / 2, nexus.y);
                    bb[0].small_actions[2] = Some(SmallAction::Attack { target_id: nexus.id }); }
            15 => { set_xy(bp, tower.x + r_t / 2, tower.y); bb[0].small_actions[2] = Some(SmallAction::Attack { target_id: tower.id });
                    p32(bp as *mut u8, 0x4c0, -1); }
            16 => { set_xy(bp, tower.x + r_t / 2, tower.y); bb[0].small_actions[0] = Some(SmallAction::Attack { target_id: tower.id });
                    set_xy(a0, tower.x + r_t / 2 + 60001, tower.y); }
            17 => { set_xy(bp, tower.x + r_t / 2, tower.y); bb[0].small_actions[0] = Some(SmallAction::Attack { target_id: tower.id });
                    set_xy(a0, tower.x + r_t / 2 + 60000, tower.y); }
            18 => { set_xy(bp, tower.x + r_t / 2, tower.y); bb[0].small_actions[2] = Some(SmallAction::Attack { target_id: tower.id });
                    set_xy(ep, tower.x + r_t / 2 + 5000, tower.y); bb[1].last_visible[epos] = tick - 120; }
            19 => { set_xy(bp, tower.x + r_t / 2, tower.y); bb[0].small_actions[2] = Some(SmallAction::Attack { target_id: tower.id });
                    set_xy(ep, tower.x + r_t / 2 + 5000, tower.y); bb[1].last_visible[epos] = tick - 121; }
            20 => { if let Some(j) = jungle { let r = window_range(bene, j); set_xy(bp, j.x + r / 2, j.y);
                    bb[0].small_actions[2] = Some(SmallAction::Attack { target_id: j.id }); println!("jungle\tid={}\tcan_target={}", j.id, j.can_target()); } }
            21 => { let np = nexus as *const Entity as *mut u8; println!("nexus_can_target_before\t{}", nexus.can_target());
                    std::ptr::write_volatile(np.add(0x6b9), 1u8); p64(np, 0x6a0, 0);
                    let r = window_range(bene, nexus); set_xy(bp, nexus.x + r / 2, nexus.y);
                    bb[0].small_actions[2] = Some(SmallAction::Attack { target_id: nexus.id }); }
            22 => { if let Some(m) = cache.iter_minions(1).filter(|m| m.can_target()).next() { let r = window_range(bene, m); set_xy(bp, m.x + r / 2, m.y);
                    bb[0].small_actions[2] = Some(SmallAction::Attack { target_id: m.id }); println!("minion\tid={}", m.id); } }
            23 => { if let (Some(j), Some(m)) = (jungle, cache.iter_minions(1).filter(|m| m.can_target()).next()) {
                    // 우선순위: 정글이 창이면 미니언도 창이어도 {1,0} — 미니언을 정글 옆으로 옮겨 둘 다 in_window 로 만든다
                    let r = window_range(bene, j); set_xy(bp, j.x + r / 2, j.y); set_xy(m as *const Entity as *mut Entity, j.x, j.y + 1000);
                    bb[0].small_actions[2] = Some(SmallAction::Attack { target_id: j.id }); bb[0].small_actions[0] = Some(SmallAction::Attack { target_id: m.id });
                    set_xy(a0, j.x + 30000, j.y); } }
            24 | 25 => {   // engage 경계: rb = engage(bene)+c.radius() · dist == rb → 적 근접(None) / rb+1 → Some — ×120 판별(ms=1 이라 1단위 감도)
                    set_xy(bp, tower.x + r_t / 2, tower.y); bb[0].small_actions[2] = Some(SmallAction::Attack { target_id: tower.id });
                    let rb = engage(bene) + enemy.radius() as u64 + if case == 25 { 1 } else { 0 };
                    set_xy(ep, tower.x + r_t / 2 + rb, tower.y); bb[1].last_visible[epos] = tick - 10; println!("rb	{}", rb); }
            _ => {}
        }
    }
    let data = OperationData::new(&cache, &ctx, &bb);
    let exp = model(player, &data, &cache, &bb, &game as &dyn AbstractGame, caster, bene);
    let got = unsafe { nsw(player as *const PlayerState, &data as *const OperationData, caster as *const Entity, bene as *const Entity) };
    let gb: [u8; 2] = unsafe { std::ptr::read(&got as *const _ as *const [u8; 2]) };
    let gm = match got { None => None, Some(w) => Some((w.fights_back, w.minion_wave)) };
    println!("case={}\tgot_bytes=[{},{}]\tgot={:?}\tmodel={:?}\t{}", case, gb[0], gb[1], gm, exp,
             if gm == exp { "MATCH" } else { "DIFF" });
}
