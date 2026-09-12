#![allow(unused, dead_code, non_snake_case)]
//! B5_o2 — 09 보강: ① rear 의 base-거리 타이브레이크(1296/1298/1300) 경계 격리
//!                  ② 독립 재구현 ↔ 실함수 무작위 대조
//!                  ③ 그 재구현으로 **상수별 민감도**(노브 실효성) 정량화
//!                     ⟹ 1286 의 `9`(sin² 임계)가 실효 노브인지 판정
use game_core::*;
use rand::SeedableRng;
use rand::Rng;
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
    s.return_tick = 120;
    s.epic_minion_buff_duration = 5400; s.epic_minion_buff_range = 140000;
    s.tower_reduce_tick_2v2 = 5400; s.tower_reduce_tick_3v3 = 7200;
    s.gold_ratio_2v2 = 200; s.gold_ratio_3v3 = 150;
    s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999;
    s.tower_attack_disable_tick_2v2 = 10800; s.tower_attack_disable_tick_3v3 = 14400;
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
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default();
        st.judgement = 80; st.mental = 60;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, ctx);
    game
}

fn dsq(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = ax.abs_diff(bx); let dy = ay.abs_diff(by);
    dx*dx + dy*dy
}

/// 명세 그대로의 독립 재구현. 미니언 게이트는 제외(프로브 세계엔 미니언이 0마리라 항상 0).
/// K = (N_front, N_rear_num, N_rear_den, N_flank_num, N_flank_den, hp_pct, slack, r_l, r_r)
#[derive(Clone, Copy)]
struct Knobs { nfront: i128, nrear: i128, nflank: i128, den: i128, hp: usize, slack: u64, rl: u128, rr: u128 }
impl Default for Knobs {
    fn default() -> Self { Knobs { nfront: 4, nrear: 9, nflank: 9, den: 100, hp: 40, slack: 100000, rl: 5, rr: 6 } }
}
/// 분기 카운터도 같이 돌려준다: [front, rear, flank_from_rear, flank, else1307]
fn mine(k: Knobs, champ: (u64,u64), allies: &[(u64,u64,usize,usize)], ex: u64, ey: u64,
        bx: u64, by: u64, er: u64, cnt: &mut [u64;5]) -> bool {
    let rdx = bx as i128 - ex as i128;
    let rdy = by as i128 - ey as i128;
    let rl = rdx*rdx + rdy*rdy;
    if rl < 1 { return true; }
    let (mut front, mut flank, mut rear) = (0i64, 0i64, 0i64);
    let e2b = dsq(ex, ey, bx, by);
    let maxd = er + k.slack;
    for &(ax, ay, hp, maxhp) in allies {
        if hp*100/maxhp < k.hp { continue; }
        if dsq(ax, ay, ex, ey) > maxd*maxd { continue; }
        let adx = ex as i128 - ax as i128;
        let ady = ey as i128 - ay as i128;
        let al = adx*adx + ady*ady;
        if al < 1 { front += 1; cnt[0]+=1; continue; }
        let dot = adx*rdx + ady*rdy;
        let cross = adx*rdy - ady*rdx;
        let lp = al * rl;
        let is_front = dot > 0 && dot*dot*k.nfront > lp;
        let is_rear  = dot < 0 && dot*dot*k.den > lp*k.nrear;
        let is_flank = cross*cross*k.den > lp*k.nflank;
        if is_front { front += 1; cnt[0]+=1 }
        else if is_rear {
            if dsq(ax, ay, bx, by) < e2b { rear += 1; cnt[1]+=1 } else { flank += 1; cnt[2]+=1 }
        }
        else if is_flank { flank += 1; cnt[3]+=1 }
        else { front += 1; cnt[4]+=1 }
    }
    if rear > 0 { return true }
    if flank > 1 || (flank > 0 && front > 0) { return true }
    if front > 1 {
        let c2b = dsq(champ.0, champ.1, bx, by) as u128;
        return c2b * k.rl <= (e2b as u128) * k.rr;
    }
    false
}

fn main() {
    let setting = real_setting();
    println!("setting_ok\t{}", setting.width!=0 && setting.height!=0 && setting.tick_per_second!=0 && setting.champion_radius!=0);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let game = mkgame(&setting, &ms, &map, &ctx);
    let f = map.fountains[1];
    let bx = (f.0+f.2)/2; let by = (f.1+f.3)/2;
    let c0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let proto: Entity = c0.player_champion[0][0].unwrap().clone();
    let penemy: Entity = c0.player_champion[1][0].unwrap().clone();
    let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();

    // 호출 헬퍼
    let call = |champ: (u64,u64), allies: &[(u64,u64,usize,usize)], ex: u64, ey: u64, er: u64| -> bool {
        let mut ents: Vec<Entity> = Vec::new();
        let mut c = proto.clone(); c.id = 9000; c.x = champ.0; c.y = champ.1; c.hp = 1000; c.stat_cached.hp = 1000;
        ents.push(c);
        for (i, a) in allies.iter().enumerate() {
            let mut e = proto.clone(); e.id = 9100+i; e.x = a.0; e.y = a.1; e.hp = a.2; e.stat_cached.hp = a.3;
            ents.push(e);
        }
        let mut t = penemy.clone(); t.id = 9500; t.x = ex; t.y = ey; t.hp = 1000; t.stat_cached.hp = 1000;
        let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        for tt in 0..2 { for p in 0..5 { cache.player_champion[tt][p] = None; } }
        cache.player_champion[0][0] = Some(&ents[0]);
        for i in 0..allies.len().min(4) { cache.player_champion[0][1+i] = Some(&ents[1+i]); }
        let bb: [Blackboard;2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        game_ai::check_favorable_engage_formation(3, player, &data, &t, er)
    };

    // ── ① rear 타이브레이크 경계 (E 를 base 근처로 끌어와야 도달한다)
    // E=(900000,32000), base=(926000,32000), enemy_to_base = 26000
    println!("#R\tally_x\tto_base\te2b\tgame\ttheory\tverdict");
    let ex = 900000u64; let ey = 32000u64;
    let far_champ = (bx, by + 900000);            // ratio 게이트 false (front 경로 무력화)
    let mut bad = 0;
    for ax in [913000u64, 925999, 926000, 951999, 952000, 952001, 955000] {
        let to_base = (bx as i64 - ax as i64).abs();
        let th = (to_base as u64) < 26000;        // rear(true) / flank(false, 단독이면 false)
        let got = call(far_champ, &[(ax, ey, 100, 100)], ex, ey, 200000);
        let v = if got == th { "MATCH" } else { bad += 1; "MISMATCH" };
        println!("R\t{}\t{}\t26000\t{}\t{}\t{}", ax, to_base, got, th, v);
    }
    println!("R_SUMMARY\tmismatch={}", bad);

    // ── ② 독립 재구현 ↔ 실함수 무작위 대조
    let mut rng = rand::rngs::StdRng::seed_from_u64(20260911);
    let mut n = 0u32; let mut mm = 0u32;
    let mut cnt = [0u64;5];
    let mut tcount = 0u32;
    for _ in 0..3000 {
        let ex = rng.gen_range(0u64..=960000);
        let ey = rng.gen_range(0u64..=960000);
        let cx = rng.gen_range(0u64..=960000);
        let cy = rng.gen_range(0u64..=960000);
        let na = rng.gen_range(0usize..=4);
        let er = [50000u64, 100000, 200000, 400000][rng.gen_range(0usize..4)];
        let mut allies: Vec<(u64,u64,usize,usize)> = Vec::new();
        for _ in 0..na {
            // 적 주변 ±400000 (경계 근처를 자주 치도록)
            let ax = (ex as i64 + rng.gen_range(-400000i64..=400000)).clamp(0, 960000) as u64;
            let ay = (ey as i64 + rng.gen_range(-400000i64..=400000)).clamp(0, 960000) as u64;
            let hp = rng.gen_range(1usize..=100);
            allies.push((ax, ay, hp, 100));
        }
        let got = call((cx,cy), &allies, ex, ey, er);
        let th = mine(Knobs::default(), (cx,cy), &allies, ex, ey, bx, by, er, &mut cnt);
        if got { tcount += 1; }
        if got != th { mm += 1;
            if mm <= 5 { println!("DIFF\tE=({},{}) champ=({},{}) er={} allies={:?} game={} mine={}", ex,ey,cx,cy,er,allies,got,th); } }
        n += 1;
    }
    println!("XCHK\tn={}\tmismatch={}\ttrue={}\tbranch[front={} rear={} flank_from_rear={} flank={} else1307={}]",
        n, mm, tcount, cnt[0], cnt[1], cnt[2], cnt[3], cnt[4]);

    // ── ③ 상수별 민감도 (검증된 재구현 기준). 같은 3000 시나리오 고정 재사용.
    let mut scen: Vec<((u64,u64), Vec<(u64,u64,usize,usize)>, u64, u64, u64)> = Vec::new();
    let mut rng2 = rand::rngs::StdRng::seed_from_u64(20260911);
    for _ in 0..3000 {
        let ex = rng2.gen_range(0u64..=960000); let ey = rng2.gen_range(0u64..=960000);
        let cx = rng2.gen_range(0u64..=960000); let cy = rng2.gen_range(0u64..=960000);
        let na = rng2.gen_range(0usize..=4);
        let er = [50000u64,100000,200000,400000][rng2.gen_range(0usize..4)];
        let mut allies = Vec::new();
        for _ in 0..na {
            let ax = (ex as i64 + rng2.gen_range(-400000i64..=400000)).clamp(0,960000) as u64;
            let ay = (ey as i64 + rng2.gen_range(-400000i64..=400000)).clamp(0,960000) as u64;
            allies.push((ax, ay, rng2.gen_range(1usize..=100), 100usize));
        }
        scen.push(((cx,cy), allies, ex, ey, er));
    }
    let base_k = Knobs::default();
    let mut zz = [0u64;5];
    let base_true: u32 = scen.iter().map(|s| mine(base_k,s.0,&s.1,s.2,s.3,bx,by,s.4,&mut zz) as u32).sum();
    println!("#SENS\tknob\tvalue\ttrue_count\tdelta_vs_base({})\telse1307", base_true);
    for v in [1i128,2,3,4,5,8,16] {
        let mut k = base_k; k.nfront = v; let mut c=[0u64;5];
        let t: u32 = scen.iter().map(|s| mine(k,s.0,&s.1,s.2,s.3,bx,by,s.4,&mut c) as u32).sum();
        println!("SENS\tnfront(1280)\t{}\t{}\t{:+}\t{}", v, t, t as i64 - base_true as i64, c[4]);
    }
    for v in [0i128,1,5,9,25,50,91,100] {
        let mut k = base_k; k.nrear = v; let mut c=[0u64;5];
        let t: u32 = scen.iter().map(|s| mine(k,s.0,&s.1,s.2,s.3,bx,by,s.4,&mut c) as u32).sum();
        println!("SENS\tnrear(1283)\t{}\t{}\t{:+}\t{}", v, t, t as i64 - base_true as i64, c[4]);
    }
    for v in [0i128,1,5,9,25,50,74,75,80,91,99,100] {
        let mut k = base_k; k.nflank = v; let mut c=[0u64;5];
        let t: u32 = scen.iter().map(|s| mine(k,s.0,&s.1,s.2,s.3,bx,by,s.4,&mut c) as u32).sum();
        println!("SENS\tnflank(1286)\t{}\t{}\t{:+}\t{}", v, t, t as i64 - base_true as i64, c[4]);
    }
    for v in [0usize,20,39,40,41,60,80,101] {
        let mut k = base_k; k.hp = v; let mut c=[0u64;5];
        let t: u32 = scen.iter().map(|s| mine(k,s.0,&s.1,s.2,s.3,bx,by,s.4,&mut c) as u32).sum();
        println!("SENS\thp(1240)\t{}\t{}\t{:+}\t{}", v, t, t as i64 - base_true as i64, c[4]);
    }
    for v in [0u64,50000,100000,200000,400000] {
        let mut k = base_k; k.slack = v; let mut c=[0u64;5];
        let t: u32 = scen.iter().map(|s| mine(k,s.0,&s.1,s.2,s.3,bx,by,s.4,&mut c) as u32).sum();
        println!("SENS\tslack(1246)\t{}\t{}\t{:+}\t{}", v, t, t as i64 - base_true as i64, c[4]);
    }
    for v in [1u128,3,6,12,100] {
        let mut k = base_k; k.rr = v; let mut c=[0u64;5];
        let t: u32 = scen.iter().map(|s| mine(k,s.0,&s.1,s.2,s.3,bx,by,s.4,&mut c) as u32).sum();
        println!("SENS\tratio_rhs(1340)\t{}\t{}\t{:+}\t{}", v, t, t as i64 - base_true as i64, c[4]);
    }
}
