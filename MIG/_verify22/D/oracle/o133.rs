#![allow(unused, dead_code, non_snake_case)]
//! 22차 배치D 오라클 — #133 buff_value_v54 (hidden define, m10.ll:30000) — `#[link_name]` 직접 호출
//! near_enemies 는 비움(앵커 None → epic_incoming 경로) · blackboard 기본값(small_actions None → 기동 항 0) · crisis 는 2B 버퍼
//! 사용: o133.exe <case>  (케이스당 프로세스 1개)
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

extern "Rust" {
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai10buff_value14buff_value_v54"]
    fn bv(buff: *const u8, recv: *const Entity, data: *const OperationData, player: *const PlayerState, param: *const u8,
          crisis: *const u8, incoming: i64, epic_incoming: i64, on_attack_damage: i64, recv_hp_value: i64) -> i64;
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

#[repr(C, align(8))]
struct Buf<const N: usize>([u8; N]);
fn w32(b: &mut [u8], off: usize, v: i32) { b[off..off + 4].copy_from_slice(&v.to_le_bytes()); }
fn w64(b: &mut [u8], off: usize, v: i64) { b[off..off + 8].copy_from_slice(&v.to_le_bytes()); }
unsafe fn p64(base: *mut u8, off: usize, v: i64) { std::ptr::write_unaligned(base.add(off) as *mut i64, v); }

#[derive(Default, Clone, Copy)]
struct B { attack_mult: i32, attack: i32, skill_cd: i32, ult_cd: i32, vamp: i32, hp_regen: i32, defence: i32, damaged_reduce: i64,
           hp: i32, undying: bool, cc_immune: bool, toughness: i64, time_tick: i64, def_pen: i64, heal_reduce: i64, damaged_amplify: i64,
           magic_power_mult: i32, hp_mult: i32, damage_reflect: i64 }
fn buf_of(b: &B) -> Box<Buf<288>> {
    let mut o = Box::new(Buf([0u8; 288]));
    if b.time_tick > 0 { w32(&mut o.0, 0x48, 1); w64(&mut o.0, 0x50, b.time_tick); }
    w32(&mut o.0, 0x5c, b.attack_mult); w32(&mut o.0, 0x58, b.attack); w32(&mut o.0, 0x90, b.skill_cd); w32(&mut o.0, 0xfc, b.ult_cd);
    w32(&mut o.0, 0x80, b.vamp); w32(&mut o.0, 0x74, b.hp_regen); w32(&mut o.0, 0x68, b.defence); w64(&mut o.0, 0xe8, b.damaged_reduce);
    w32(&mut o.0, 0x70, b.hp); o.0[0x118] = b.undying as u8; o.0[0xf8] = b.cc_immune as u8; w64(&mut o.0, 0xb8, b.toughness);
    w64(&mut o.0, 0xa8, b.def_pen); w64(&mut o.0, 0xc0, b.heal_reduce); w64(&mut o.0, 0xa0, b.damaged_amplify);
    w32(&mut o.0, 0x64, b.magic_power_mult); w32(&mut o.0, 0x84, b.hp_mult); w64(&mut o.0, 0x98, b.damage_reflect);
    o
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
    let game = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);

    let mut b = B::default();
    let (mut incoming, mut epic, mut oad, mut hv) = (0i64, 0i64, 0i64, 100i64);
    let mut crisis: Option<(bool, bool)> = None;   // (die_imminent, cc_threat)
    let (atk, mp, maxhp, def, mr, hp) = (100i64, 100i64, 1000i64, 50i64, 30i64, 600i64);
    let (ca, cs, cs2, cu) = (100i64, 50i64, 50i64, 200i64);     // recv 캐시 → a_ps 100 · s_ps 100 · u_ps 200
    let (e_def, e_mr, e_heal) = (100i64, 40i64, 10i64);          // 적 5명 스탯·힐 캐시
    match case {
        0 => {}
        1 => { b.attack_mult = 50; epic = 1 }            // delta 50 → offense 300 → 30
        2 => { b.attack_mult = 50 }                      // 앵커 없음·epic 0 → 0
        3 => { b.skill_cd = 100 }                        // cd_gain 300*100/200=150 → 6*100*150/600 = 150
        4 => { b.ult_cd = 100 }                          // 200*100/200=100 → 100
        5 => { b.vamp = 50 }                             // heal 300, realized min(400,300) → 50
        6 => { b.hp_regen = 10 }                         // 60 → 10
        7 => { incoming = 200; b.defence = 100 }         // d_def 100 → 100*100/250=40 → 6
        8 => { incoming = 200; b.damaged_reduce = 50 }   // 100 → 16
        9 => { incoming = 200; b.hp = 500 }              // min(200,500)=200 → 33
        10 => { crisis = Some((true, true)); b.undying = true; b.cc_immune = true; b.toughness = 100; hv = 20 } // 20+20+10=50
        11 => { crisis = Some((false, true)); b.cc_immune = true; b.toughness = 100 }   // 33 + 50 = 83
        12 => { crisis = Some((true, false)); b.undying = true; b.cc_immune = true; b.toughness = 100 } // 100
        13 => { b.time_tick = 120; b.attack_mult = 50; epic = 1 }   // window 2 → 100 → 10
        14 => { b.attack = 50; epic = 1 }                // 100*50/100=50 → 300 → 30
        15 => { oad = 10; epic = 1 }                     // hits = max(360/max(cooltime,1),1)
        16 => { b.def_pen = 50; epic = 1 }               // def_after 50 → 100*(100-50)/150=33 → 198 → 19
        17 => { b.heal_reduce = 50; epic = 1 }           // e_heal 50 → 50*50/100*6=150 → 15
        18 => { b.damaged_amplify = 100; epic = 1 }      // 400*3*6=7200 → 720 → 160
        19 => { incoming = 200; b.vamp = 50 }            // realized min(600,300)=300 → 50
        20 => { b.attack_mult = 50; epic = 1; b.skill_cd = 100 }   // 30+150=180 → 160
        21 => { b.magic_power_mult = 50; epic = 1 }      // (100+200)*50/100=150 → 900 → 90
        22 => { incoming = 200; b.hp_mult = 10; b.damage_reflect = 20 }   // d_hp=100 → 100 ; reflect 40 → 140 → 23
        23 => { crisis = Some((true, true)); b.undying = true; hv = -30 }  // score -30 → smax 0 → 0
        24 => { b.skill_cd = 100; hv = 1000 }            // 1500 → 160 캡
        _ => {}
    }
    let player: &PlayerState = game.get_player_by_position(0, Position::Mid).expect("player");
    let recv: &Entity = cache.player_champion[0][2].expect("champ");
    let rp = cache.player_by_champion_id(recv.id).expect("rp");
    unsafe {
        let ep = recv as *const Entity as *mut u8;
        p64(ep, 0x618, atk); p64(ep, 0x620, mp); p64(ep, 0x628, maxhp); p64(ep, 0x630, def); p64(ep, 0x638, mr); p64(ep, 0x670, hp);
        let cp = (&cache as *const AbstractGameWithCache as *mut u8).add(0x280 + (rp.info.team * 5 + rp.info.position as usize) * 800);
        for i in 0..5 { p64(cp, 400 + i * 8, ca); p64(cp, 440 + i * 8, cs); p64(cp, 480 + i * 8, cs2); p64(cp, 520 + i * 8, cu); }
        for p in 0..5 { if let Some(e) = cache.player_champion[1][p] {
            let q = e as *const Entity as *mut u8; p64(q, 0x630, e_def); p64(q, 0x638, e_mr);
            let ap = cache.player_by_champion_id(e.id).expect("ep");
            let cq = (&cache as *const AbstractGameWithCache as *mut u8).add(0x280 + (ap.info.team * 5 + ap.info.position as usize) * 800);
            for i in 0..5 { p64(cq, 560 + i * 8, e_heal); p64(cq, 600 + i * 8, 0); }
        } }
    }
    let bufb = buf_of(&b);
    let param = Box::new(Buf([0u8; 5384]));   // near_enemies len 0 → 앵커 없음
    let cr: [u8; 2] = crisis.map(|(d, c)| [d as u8, c as u8]).unwrap_or([0, 0]);
    let crp: *const u8 = if crisis.is_some() { cr.as_ptr() } else { std::ptr::null() };

    // ── 모델(명세 logic §1~§9, 앵커 None·기동 None 가정) ──
    let tps = 60i64;
    let window = if b.time_tick > 0 { (b.time_tick / tps).max(1).min(6) } else { 6 };
    let (a_ps, s_ps, u_ps) = (ca * 5 / 5, (cs * 5 + cs2 * 5) / 5, cu * 5 / 5);
    let e_n = 5i64;
    let mut delta = 0i64;
    if b.attack_mult > 0 { delta += a_ps * b.attack_mult as i64 / 100 }
    if b.attack > 0 { delta += a_ps * b.attack as i64 / atk.max(1) }
    if b.magic_power_mult > 0 { delta += (s_ps + u_ps) * b.magic_power_mult as i64 / 100 }
    if e_n > 0 && b.def_pen != 0 { let da = (100 - b.def_pen).max(0) * e_def / 100; delta += a_ps * (e_def - da) / (da.max(-99) + 100) }
    let mut offense = delta * window;
    let hits = (window * tps / (recv.attack_cooltime() as i64).max(1)).max(1);
    if oad > 0 { offense += hits * oad }
    if b.heal_reduce != 0 { let eh = 5 * ((e_heal * 5 + 0) / 5); offense += (eh * b.heal_reduce / 100) * window }
    if b.damaged_amplify != 0 {
        let mut na = 0i64; for p in 0..5 { if let Some(e) = cache.player_champion[0][p] {
            let dx = (e.x as i64 - recv.x as i64).abs(); let dy = (e.y as i64 - recv.y as i64).abs();
            if dx * dx + dy * dy < 14400000001 { na += 1 } } }
        offense += ((a_ps + s_ps + u_ps) * b.damaged_amplify / 100) * na.min(3) * window }
    let mut score = 0i64;
    if offense > 0 && epic > 0 { score = offense * hv / maxhp.max(1) }
    if b.skill_cd > 0 || b.ult_cd > 0 {
        let mut cov = 0i64; for p in 0..5 { if let Some(e) = cache.player_champion[0][p] {
            let dx = (e.x as i64 - recv.x as i64).abs(); let dy = (e.y as i64 - recv.y as i64).abs();
            if dx * dx + dy * dy < 3600000001 { cov += 1 } } }
        if cov > 1 {
            let mut g = if b.skill_cd > 0 { (s_ps + u_ps) * b.skill_cd as i64 / (b.skill_cd as i64 + 100) } else { 0 };
            if b.ult_cd > 0 { g += u_ps * b.ult_cd as i64 / (b.ult_cd as i64 + 100) }
            score += window * hv * g / hp.max(1) } }
    let mut heal = 0i64;
    if b.vamp > 0 { heal = (a_ps * b.vamp as i64 / 100) * window }
    heal += if b.hp_regen > 0 { window * b.hp_regen as i64 } else { 0 };
    let realized = ((maxhp - hp).max(0) + incoming).min(heal);
    if realized > 0 { score += realized * hv / hp.max(1) }
    if incoming > 0 {
        let mut mit = 0i64;
        let dd = b.defence as i64 + def * 0 / 100; if dd > 0 { mit = dd * (incoming / 2) / (def + 100 + dd).max(1) }
        if b.damaged_reduce != 0 { mit += b.damaged_reduce * incoming / 100 }
        if b.damage_reflect != 0 { mit += b.damage_reflect * incoming / 100 }
        let dh = b.hp as i64 + maxhp * b.hp_mult as i64 / 100; if dh > 0 { mit += incoming.min(dh) }
        if mit > 0 { score += mit * hv / hp.max(1) } }
    if let Some((d, c)) = crisis {
        if b.undying { score += if d { hv } else { 0 } }
        if b.cc_immune { if c { score += if d { hv } else { hv / 3 } } }
        if b.toughness != 0 { if c { score += b.toughness * hv / 200 } } }
    let exp = score.clamp(0, 160);

    let got = unsafe { bv(bufb.0.as_ptr(), recv as *const Entity, &data as *const OperationData, player as *const PlayerState,
                          param.0.as_ptr(), crp, incoming, epic, oad, hv) };
    println!("case={}\tgot={}\tmodel={}\t{}\t(cooltime={} window={})", case, got, exp, if got == exp { "MATCH" } else { "DIFF" },
             recv.attack_cooltime(), window);
}
