#![allow(unused, dead_code, non_snake_case)]
//! 15차 배치 B 오라클 — #26 v23_objective_setup_pressure_line · #27 nexus_under_direct_attack ·
//! #29 v23_recent_visible_enemies_near_point (셋 다 pub). TEMPLATE.rs(real_setting/mkgame) 기반.
//! TLS 메모 함수 없음(세 함수의 호출 그래프에 thread_local 없음 — is_recent_visible/champions/minions 는 순수 읽기).
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;
use std::mem::MaybeUninit;

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000; s.height = 960000; s.respawn_tick = 300; s.respawn_growth = 30;
    s.respawn_growth_term = 1800; s.respawn_growth_per_level = 180; s.respawn_max = 2400;
    s.visible_distance = 130000; s.tick_per_second = 60; s.champion_radius = 10000;
    s.nexus_heal = 10; s.nexus_heal_2v2 = 10; s.nexus_heal_3v3 = 10; s.nexus_heal_tick = 24;
    s.nexus_heal_decay = 100; s.kill_exp = 30; s.kill_exp_growth = 30; s.assist_exp_ratio = 40;
    s.kill_gold = 300; s.assist_gold = 100; s.start_gold = 500; s.gold_per_second = 7;
    s.return_tick = 120; s.epic_minion_buff_duration = 5400; s.epic_minion_buff_range = 140000;
    s.tower_reduce_tick_2v2 = 5400; s.tower_reduce_tick_3v3 = 7200; s.gold_ratio_2v2 = 200;
    s.gold_ratio_3v3 = 150; s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999; s.tower_attack_disable_tick_2v2 = 10800;
    s.tower_attack_disable_tick_3v3 = 14400; s.well_damage = 600; s.well_damage_tick = 30;
    s.top_hp_regen_percent = 1; s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20;
    s.mid_exp_bonus = 20; s.bottom_gold_bonus = 20; s.support_gold_reduction = 15;
    s.support_exp_reduction = 30; s.stamina_zero_debuff_percent = 30;
    s
}
pub fn setting_ok(s: &GameSetting) -> bool {
    let ok = s.width != 0 && s.height != 0 && s.tick_per_second != 0 && s.champion_radius != 0;
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}\tvisible={}",
             ok, s.width, s.height, s.tick_per_second, s.champion_radius, s.visible_distance);
    ok
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

/// Entity 바이트 복제본(절대 drop 하지 않는다 — Arc/Vec 이중해제 방지)
fn clone_entity(e: &Entity) -> *mut Entity {
    unsafe {
        let b: Box<MaybeUninit<Entity>> = Box::new(MaybeUninit::uninit());
        let p = Box::leak(b);
        std::ptr::copy_nonoverlapping(e as *const Entity as *const u8, p.as_mut_ptr() as *mut u8, 1728);
        p.as_mut_ptr()
    }
}
unsafe fn w64(e: *mut Entity, off: usize, v: u64) { std::ptr::write_unaligned(((e as usize) + off) as *mut u64, v); }
unsafe fn r64(e: *mut Entity, off: usize) -> u64 { std::ptr::read_unaligned(((e as usize) + off) as *const u64) }
fn st(e: *mut Entity) -> &'static Entity { unsafe { &*e } }

fn main() {
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
    let tick_arg: usize = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(0);
    if tick_arg > 0 { game.set_tick(tick_arg); }
    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    println!("towers\t{}\ttwin0={}\ttwin1={}", game.world.tower_ids.len(), cache.twin_towers[0].len(), cache.twin_towers[1].len());
    println!("tick\t{}", game.tick());

    // offset_of! 교차검증(pub 필드)
    println!("offset_of\tEntity.x={:#x}\ty={:#x}\thp={:#x}\tid={:#x}\tty={:#x}\tstat_cached={:#x}\tteam={:#x}",
        std::mem::offset_of!(Entity, x), std::mem::offset_of!(Entity, y), std::mem::offset_of!(Entity, hp),
        std::mem::offset_of!(Entity, id), std::mem::offset_of!(Entity, ty), std::mem::offset_of!(Entity, stat_cached), std::mem::offset_of!(Entity, team));
    println!("offset_of\tCache.nexus={:#x}\tplayer_champion={:#x}\ttop_lead={:#x}\tmid_lead={:#x}\tbottom_lead={:#x}\ttop_minions={:#x}\tmid_minions={:#x}\tbottom_minions={:#x}",
        std::mem::offset_of!(AbstractGameWithCache, nexus), std::mem::offset_of!(AbstractGameWithCache, player_champion),
        std::mem::offset_of!(AbstractGameWithCache, top_lead), std::mem::offset_of!(AbstractGameWithCache, mid_lead), std::mem::offset_of!(AbstractGameWithCache, bottom_lead),
        std::mem::offset_of!(AbstractGameWithCache, top_minions), std::mem::offset_of!(AbstractGameWithCache, mid_minions), std::mem::offset_of!(AbstractGameWithCache, bottom_minions));
    println!("offset_of\tBlackboard.last_visible={:#x}\ttop_ms={:#x}\tmid_ms={:#x}\tbottom_ms={:#x}\tBMP.from_mid={:#x}\tminion_count={:#x}\tPlayerState.info={:#x}\tOperationData.blackboard={:#x}\tGameSetting.tps={:#x}",
        std::mem::offset_of!(Blackboard, last_visible), std::mem::offset_of!(Blackboard, top_minion_state), std::mem::offset_of!(Blackboard, mid_minion_state), std::mem::offset_of!(Blackboard, bottom_minion_state),
        std::mem::offset_of!(BrainMinionParameter, from_mid), std::mem::offset_of!(BrainMinionParameter, minion_count), std::mem::offset_of!(PlayerState, info),
        std::mem::offset_of!(OperationData, blackboard), std::mem::offset_of!(GameSetting, tick_per_second));

    let player: &PlayerState = game.get_player_by_position(0, Position::Top).expect("player");
    let team = 0usize; let enemy_ix = 1usize;

    // ── 적 챔프 5명을 복제본으로 교체(좌표·hp 자유 조작) ─────────────────────
    let en: Vec<*mut Entity> = (0..5).map(|p| clone_entity(cache.player_champion[enemy_ix][p].unwrap())).collect();
    let en_ids: Vec<usize> = en.iter().map(|&e| st(e).id).collect();
    let en_pos: Vec<usize> = (0..5).map(|p| {
        let ps = cache.player_state[enemy_ix][p].unwrap();
        unsafe { std::ptr::read_unaligned(((ps as *const PlayerState as usize) + 0x9c0) as *const u32) as usize }
    }).collect();
    println!("enemy_ids\t{:?}\tenemy_pos_tags\t{:?}", en_ids, en_pos);
    let tick = game.tick();
    // 직접 가시 여부(월드 fog) 기록
    for p in 0..5 { println!("is_visible(team0, enemy{})\t{}", p, game.is_visible(0, en_ids[p])); }

    // ═══════ #29 v23_recent_visible_enemies_near_point ═══════
    {
        let (x0, y0) = (500000u64, 500000u64);
        let dist = [0u64, 100000, 200000, 200001, 400000];
        let hp_pct = [100usize, 50, 49, 100, 100];
        for p in 0..5 {
            let e = en[p];
            unsafe {
                w64(e, 0x660, x0 + dist[p]); w64(e, 0x668, y0);
                let maxhp = 1000u64; // default 챔프 stat_cached.hp==1 이라 50% 가 0 이 된다(입력 판별력) → 1000 으로 고정
                w64(e, 0x628, maxhp); w64(e, 0x670, maxhp * hp_pct[p] as u64 / 100);
                println!("enemy{}	maxhp={}	hp={}	ratio={}", p, maxhp, r64(e, 0x670), r64(e, 0x670) * 100 / maxhp);
            }
        }
        for p in 0..5 { cache.player_champion[enemy_ix][p] = Some(st(en[p])); }
        // 최근가시: blackboard[enemy_ix].last_visible[pos] = tick
        for p in 0..5 { bb[enemy_ix].last_visible[en_pos[p]] = tick; }
        let data = OperationData::new(&cache, &ctx, &bb);
        for (range, minhp, expect) in [(200000u64, 0usize, 3usize), (200000, 50, 2), (200000, 51, 1), (199999, 0, 2), (200001, 0, 4), (0, 0, 1), (400000, 0, 5), (400000, 100, 3)] {
            let got = game_ai::plan_legacy::team_plan::v23_recent_visible_enemies_near_point(player, &data, x0, y0, range, minhp);
            println!("o29\trange={}\tminhp={}\tgot={}\texpect={}\t{}", range, minhp, got, expect, if got == expect { "MATCH" } else { "MISMATCH" });
        }
        drop(data);
        // 최근가시 창: last_visible = tick-120 → 아직 가시 / tick-121 → 비가시 (tick=0 이면 usize 언더플로라 tick 을 올린다)
        // (set_tick 이 없으면 이 절은 건너뛴다)
        // 비가시(last_visible 아주 과거) — 직접 가시(is_visible)가 false 인 슬롯만 빠진다
        for p in 0..5 { bb[enemy_ix].last_visible[en_pos[p]] = 0; }
        let data = OperationData::new(&cache, &ctx, &bb);
        let got = game_ai::plan_legacy::team_plan::v23_recent_visible_enemies_near_point(player, &data, x0, y0, 400000, 0);
        println!("o29_novis\tlast_visible=0,tick={}\tgot={}\t(120틱 창 안이면 5, 창 밖+fog 면 is_visible 참인 수)", tick, got);
        drop(data);
        if tick >= 121 {
            // 창 경계: last_visible + 120 >= tick — tick-120 → 가시 / tick-121 → 비가시 (fog 라 is_visible 전부 false)
            for (name, lv, expect) in [("tick-120", tick - 120, 5usize), ("tick-121", tick - 121, 0), ("tick-119", tick - 119, 5)] {
                for p in 0..5 { bb[enemy_ix].last_visible[en_pos[p]] = lv; }
                let data = OperationData::new(&cache, &ctx, &bb);
                let got = game_ai::plan_legacy::team_plan::v23_recent_visible_enemies_near_point(player, &data, x0, y0, 400000, 0);
                println!("o29_win\tlast_visible={}\ttick={}\tgot={}\texpect={}\t{}", name, tick, got, expect, if got == expect { "MATCH" } else { "MISMATCH" });
                drop(data);
            }
            for p in 0..5 { bb[enemy_ix].last_visible[en_pos[p]] = tick; }
        }
    }

    // ═══════ #26 v23_objective_setup_pressure_line ═══════
    {
        use LineType::*;
        let set = |cache: &mut AbstractGameWithCache, bb: &mut [Blackboard; 2], lead: [usize; 3], fm: [i64; 3], mc: [i32; 3]| {
            cache.top_lead[team] = lead[0]; cache.mid_lead[team] = lead[1]; cache.bottom_lead[team] = lead[2];
            bb[team].top_minion_state.from_mid = fm[0]; bb[team].mid_minion_state.from_mid = fm[1]; bb[team].bottom_minion_state.from_mid = fm[2];
            bb[team].top_minion_state.minion_count = mc[0]; bb[team].mid_minion_state.minion_count = mc[1]; bb[team].bottom_minion_state.minion_count = mc[2];
        };
        let cases: Vec<(&str, [usize;3], [i64;3], [i32;3], Vec<LineType>, Option<LineType>)> = vec![
            ("all0 tie→first", [0,0,0], [0,0,0], [0,0,0], vec![Top,Mid,Bottom], Some(Top)),
            ("tie order MBT→Mid", [0,0,0], [0,0,0], [0,0,0], vec![Mid,Bottom,Top], Some(Mid)),
            ("top excl(3,2000,2) mid100 bot50→Bottom", [3,0,0], [2000,100,50], [2,0,0], vec![Top,Mid,Bottom], Some(Bottom)),
            ("top lead2(not excl) score24000 vs 30000→Top", [2,0,0], [2000,30000,30000], [2,0,0], vec![Top,Mid,Bottom], Some(Top)),
            ("top fm1999(not excl)→Top", [3,0,0], [1999,40000,40000], [2,0,0], vec![Top,Mid,Bottom], Some(Top)),
            ("top mc1(not excl)→Top", [3,0,0], [2000,40000,40000], [1,0,0], vec![Top,Mid,Bottom], Some(Top)),
            ("all excl→None", [3,3,3], [2000,2000,2000], [2,2,2], vec![Top,Mid,Bottom], None),
            ("neg mc: mid -5 → -5000 min→Mid", [0,0,0], [0,0,0], [0,-5,0], vec![Top,Mid,Bottom], Some(Mid)),
            ("lead weight: top lead1 fm0 vs mid fm9999→Mid", [1,0,0], [0,9999,20000], [0,0,0], vec![Top,Mid,Bottom], Some(Mid)),
            ("lead weight: top lead1 fm0 vs mid fm10000→Top(tie first)", [1,0,0], [0,10000,20000], [0,0,0], vec![Top,Mid,Bottom], Some(Top)),
            ("mc weight: top mc1 fm0 vs mid fm999→Mid", [0,0,0], [0,999,20000], [1,0,0], vec![Top,Mid,Bottom], Some(Mid)),
            ("only Bottom", [0,0,0], [0,0,0], [0,0,0], vec![Bottom], Some(Bottom)),
            ("empty→None", [0,0,0], [0,0,0], [0,0,0], vec![], None),
            ("dup lines [Mid,Mid]→Mid", [0,0,0], [0,0,0], [0,0,0], vec![Mid,Mid], Some(Mid)),
        ];
        for (name, lead, fm, mc, lines, expect) in cases {
            set(&mut cache, &mut bb, lead, fm, mc);
            let data = OperationData::new(&cache, &ctx, &bb);
            let got = game_ai::plan_legacy::team_plan::v23_objective_setup_pressure_line(player, &data, &lines);
            let g = got.map(|l| l as u8); let e = expect.map(|l| l as u8);
            println!("o26\t{}\tgot={:?}\texpect={:?}\t{}", name, g, e, if g == e { "MATCH" } else { "MISMATCH" });
        }
        set(&mut cache, &mut bb, [0,0,0], [0,0,0], [0,0,0]);
    }

    // ═══════ #27 nexus_under_direct_attack ═══════
    {
        let nexus: &Entity = cache.nexus[team].expect("nexus[0]");
        let (nx, ny, nid) = (nexus.x, nexus.y, nexus.id);
        println!("nexus0\tid={}\tx={}\ty={}", nid, nx, ny);
        // 적 챔프 전부 멀리(가시 여부 무관하게 거리로 탈락)
        for p in 0..5 { unsafe { w64(en[p], 0x660, nx + 500000); w64(en[p], 0x668, ny); } }
        for p in 0..5 { bb[enemy_ix].last_visible[en_pos[p]] = tick; }
        let run = |cache: &AbstractGameWithCache, bb: &[Blackboard; 2], name: &str, expect: bool| {
            let data = OperationData::new(cache, &ctx, bb);
            let got = game_ai::plan_legacy::old::nexus_under_direct_attack(player, &data);
            println!("o27\t{}\tgot={}\texpect={}\t{}", name, got, expect, if got == expect { "MATCH" } else { "MISMATCH" });
        };
        run(&cache, &bb, "all far", false);
        unsafe { w64(en[0], 0x660, nx + 240000); }
        run(&cache, &bb, "enemy0 d=240000 recent", true);
        unsafe { w64(en[0], 0x660, nx + 240001); }
        run(&cache, &bb, "enemy0 d=240001 recent", false);
        unsafe { w64(en[0], 0x660, nx + 240000); }
        bb[enemy_ix].last_visible[en_pos[0]] = 0;
        let vis0 = game.is_visible(0, en_ids[0]);
        run(&cache, &bb, &format!("enemy0 d=240000 last_visible=0 (is_visible={} tick={})", vis0, tick), vis0 || tick <= 120);
        unsafe { w64(en[0], 0x660, nx + 500000); }
        bb[enemy_ix].last_visible[en_pos[0]] = tick;
        // 미니언 분기: 복제본을 Minion 으로 위장(0x68 태그=1, 0x88 Some=1, 0x90 = nexus.id)
        let mp = clone_entity(cache.player_champion[enemy_ix][1].unwrap());
        unsafe { w64(mp, 0x68, 1); w64(mp, 0x88, 1); w64(mp, 0x90, nid as u64); }
        let m: &'static Entity = st(mp);
        let n_before = cache.top_minions[enemy_ix].len();
        cache.top_minions[enemy_ix].push(m);
        run(&cache, &bb, &format!("minion nearest_enemy=Some(nexus.id) (top_minions len {}→{})", n_before, n_before + 1), true);
        unsafe { w64(mp, 0x90, nid as u64 + 1); }
        run(&cache, &bb, "minion nearest_enemy=Some(other)", false);
        unsafe { w64(mp, 0x88, 0); w64(mp, 0x90, nid as u64); }
        run(&cache, &bb, "minion nearest_enemy=None", false);
        unsafe { w64(mp, 0x88, 1); w64(mp, 0x68, 2); }
        run(&cache, &bb, "ty=Tower(2) nearest=Some(nexus.id)", false);
        unsafe { w64(mp, 0x68, 1); }
        cache.top_minions[enemy_ix].pop();
        cache.bottom_minions[enemy_ix].push(m);
        run(&cache, &bb, "same minion in bottom_minions", true);
        cache.bottom_minions[enemy_ix].pop();
        // 아군(team0) 미니언 목록에 넣으면 무시돼야 한다(enemy_ix 만 본다)
        cache.top_minions[team].push(m);
        run(&cache, &bb, "minion in OWN team list → ignored", false);
        cache.top_minions[team].pop();
        // 챔프 우선: 챔프 조건 참이면 미니언 검사 생략 — 결과는 어차피 true 라 외연 동일(표기 불가), 순서만 IR
    }
    println!("expect\ttowers=16 twin=2/2");
}
