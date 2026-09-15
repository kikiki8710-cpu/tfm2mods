#![allow(unused, dead_code, non_snake_case)]
//! 26차 배치D · 207 resolve_fight_uncached 오라클.
//!  진입 = `resolve_fight`(define hidden · link_name) → resolve_fight_full(arrivals=&[], baseline=0) → 캐시(TLS ResolveFightCache)
//!  미스 → resolve_fight_uncached. ★allies.len()>8 || enemies.len()>8 이면 캐시를 우회해 uncached 직행(m10.ll:39977~39986).
//!  대조 = 명세 logic 을 독립 재구현(pub 헬퍼 fight_dps/self_sustain_in_window/available_cc_in_window/expected_dps/error_ratio_noise 호출)
//!  ↔ 실행 sret 64B 의 live 바이트(0x0/0x8/0x10/0x18/0x20/0x30/0x38/0x39). 케이스당 프로세스 1개(argv seed=…).
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify26/D/oracle/o207.rs
use game_core::*;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;
use game_ai::plan_legacy::old::FightPrediction;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

extern "Rust" {
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model13resolve_fight"]
    fn resolve_fight(version: usize, data: &OperationData, champ: &Entity, allies: &[&Entity], enemies: &[&Entity],
                     dir: i8, tower: Option<&Entity>, judge: usize) -> FightPrediction;
}

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }

fn mkeff(damage: usize, range: u64) -> Effect {
    let ae = AttackEffect { ty: AttackEffectType::EnemyTarget, damage, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(ae), range, growth_range: 0, start_timing: 10,
        target: CastingTarget::Enemy, attack_type: AttackType::BaseAttack, casting: CastingType::Targeting }
}

struct Lcg(u64);
impl Lcg { fn next(&mut self) -> u64 { self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); self.0 >> 33 }
           fn r(&mut self, n: u64) -> u64 { self.next() % n } }

struct Args { m: HashMap<String, String> }
impl Args { fn i(&self, k: &str, d: i64) -> i64 { self.m.get(k).and_then(|v| v.parse::<i64>().ok()).unwrap_or(d) } }

const INF: i64 = 2305843009213693951;

fn dist2(a: &Entity, e: &Entity) -> u64 {
    let dx = if a.x > e.x { a.x - e.x } else { e.x - a.x };
    let dy = if a.y > e.y { a.y - e.y } else { e.y - a.y };
    dx * dx + dy * dy
}

/// 명세 logic 의 독립 재구현. 반환 = (focus, soaker, net, line, trace)
fn reimpl(version: usize, ctx: &GameContext, game: &dyn AbstractGame, champ: &Entity, allies: &[&Entity], enemies: &[&Entity],
          dir: i8, tower: Option<&Entity>, judge: usize, arrivals: &[i64], baseline: i64) -> (Option<usize>, Option<usize>, i64, u8, String) {
    let mut tr = String::new();
    if allies.is_empty() || enemies.is_empty() { return (None, None, 0, 3, "early".into()); }
    let mut set_h: u64 = 0;
    for a in allies.iter().take(5) { set_h ^= (a.id as u64).wrapping_mul(0x9E3779B97F4A7C15); }
    for e in enemies.iter().take(5) { set_h ^= (e.id as u64).wrapping_mul(0x517CC1B727220A95); }
    let seed = if version > 1 { game.seed() ^ set_h.rotate_left(17) ^ (champ.id as u64) } else {
        let tps = ctx.setting.tick_per_second as u64;
        let bucket = (game.tick() as u64) / std::cmp::max(tps * 2, 1);
        (champ.id as u64) ^ (bucket << 40) };
    let mut jrng = NoiseRng::new(seed);
    let mut misjudge = |v: i64| -> i64 { if judge > 999 { v } else { v * (game_ai::error_ratio_noise(&mut jrng, judge) as i64) / 100 } };
    let tps = ctx.setting.tick_per_second as i64;
    let horizon = tps * 6;
    let mut our_hp = [0i64; 5]; let mut our_dps = [0i64; 5]; let mut our_alive = [false; 5]; let mut our_n = 0usize; let mut our_cc = 0i64;
    let mut our_arrive = [0i64; 5];
    for (ai, a) in allies.iter().copied().take(5).enumerate() {
        our_arrive[ai] = arrivals.get(ai).copied().unwrap_or(0);
        let tgt = enemies.iter().copied().min_by_key(|e| dist2(a, e));
        let dps = tgt.map(|t| game_ai::fight_dps(version, ctx, a, t) as i64).unwrap_or(0);
        let ehp = a.hp as i64 + game_ai::self_sustain_in_window(version, ctx, a, horizon as usize) as i64;
        our_cc += game_ai::available_cc_in_window(version, a, horizon as usize) as i64;
        our_hp[our_n] = misjudge(ehp * 1000); our_dps[our_n] = misjudge(dps); our_alive[our_n] = true; our_n += 1;
    }
    let mut their_hp = [0i64; 5]; let mut their_dps = [0i64; 5]; let mut their_alive = [false; 5]; let mut their_n = 0usize;
    let mut their_id = [0usize; 5]; let mut their_cc = 0i64;
    for e in enemies.iter().copied().take(5) {
        let nearest_a = allies.iter().copied().min_by_key(|a| dist2(a, e)).unwrap_or(champ);
        let ehp = e.hp as i64 + game_ai::self_sustain_in_window(version, ctx, e, horizon as usize) as i64;
        their_cc += game_ai::available_cc_in_window(version, e, horizon as usize) as i64;
        their_hp[their_n] = misjudge(ehp * 1000);
        their_dps[their_n] = misjudge(game_ai::fight_dps(version, ctx, e, nearest_a) as i64);
        their_alive[their_n] = true; their_id[their_n] = e.id; their_n += 1;
    }
    tr += &format!("our_hp={:?} our_dps={:?} their_hp={:?} their_dps={:?} our_cc={} their_cc={} | ", &our_hp[..our_n], &our_dps[..our_n], &their_hp[..their_n], &their_dps[..their_n], our_cc, their_cc);
    if horizon != 0 {
        let h = horizon;
        let their_eff = std::cmp::max(h - std::cmp::min(h / 2, our_cc), 1);
        let our_eff = std::cmp::max(h - std::cmp::min(h / 2, their_cc), 1);
        for j in 0..their_n { their_dps[j] = their_dps[j] * their_eff / h; }
        for i in 0..our_n { our_dps[i] = our_dps[i] * our_eff / h; }
    }
    let tower_dps: i64 = tower.and_then(|t| (0..our_n).max_by_key(|&i| allies[i].stat_cached.hp).map(|i| game_ai::expected_dps(ctx, t, allies[i]) as i64)).unwrap_or(0);
    let mut soaker: Option<usize> = (0..our_n).filter(|&i| our_alive[i]).max_by_key(|&i| allies[i].stat_cached.hp);
    let soaker_id: Option<usize> = soaker.map(|i| allies[i].id);
    tr += &format!("tower_dps={} soaker0={:?} | ", tower_dps, soaker);
    let mut t: i64 = 0; let mut our_dead: i64 = 0; let mut their_dead: i64 = 0; let mut first_focus: Option<usize> = None;
    let mut iters = 0;
    loop {
        iters += 1; if iters > 10000 { tr += "LOOPGUARD "; break; }
        let arrived = |i: usize| our_arrive[i] <= t;
        let our_total: i64 = (0..our_n).map(|i| if our_alive[i] && arrived(i) { our_dps[i] } else { 0 }).sum();
        let their_total: i64 = (0..their_n).filter(|&j| their_alive[j]).map(|j| their_dps[j]).sum();
        let te = match (0..their_n).filter(|&j| their_alive[j]).min_by_key(|&j| their_hp[j]) { None => break, Some(x) => x };
        let ta = match (0..our_n).filter(|&i| our_alive[i] && arrived(i)).min_by_key(|&i| our_hp[i]) {
            None => {
                let next = (0..our_n).filter(|&i| our_alive[i] && our_arrive[i] > t).map(|i| our_arrive[i]).min();
                match next { Some(n) if n < horizon => { t = n; continue; } _ => break }
            }
            Some(x) => x,
        };
        if t >= horizon || (our_total == 0 && their_total == 0) { break; }
        if first_focus.is_none() { first_focus = Some(their_id[te]); }
        let soak = soaker.filter(|&s| our_alive[s] && arrived(s));
        let ta_incoming = their_total + if soak == Some(ta) { tower_dps } else { 0 };
        let t_te = if our_total > 0 { (their_hp[te] + our_total - 1) / our_total } else { INF };
        let t_ta = if ta_incoming > 0 { (our_hp[ta] + ta_incoming - 1) / ta_incoming } else { INF };
        let t_soak = match soak { Some(s) if s != ta && tower_dps > 0 => (our_hp[s] + tower_dps - 1) / tower_dps, _ => INF };
        let dt = std::cmp::max(1, std::cmp::min(horizon - t, std::cmp::min(t_soak, std::cmp::min(t_ta, t_te))));
        their_hp[te] -= dt * our_total;
        our_hp[ta] -= dt * ta_incoming;
        if let Some(s) = soak { if s != ta { our_hp[s] -= dt * tower_dps; } }
        t += dt;
        tr += &format!("[t={} te={} ta={} soak={:?} dt={} tot=({},{})]", t, te, ta, soak, dt, our_total, their_total);
        if their_hp[te] < 1 { their_alive[te] = false; their_dead += their_dps[te]; }
        if our_hp[ta] < 1 {
            our_alive[ta] = false; our_dead += our_dps[ta];
            if soaker == Some(ta) { soaker = (0..our_n).filter(|&i| our_alive[i]).max_by_key(|&i| allies[i].stat_cached.hp); }
        }
        if let Some(s) = soak { if s != ta && our_hp[s] < 1 {
            our_alive[s] = false; our_dead += our_dps[s];
            if soaker == Some(s) { soaker = (0..our_n).filter(|&i| our_alive[i]).max_by_key(|&i| allies[i].stat_cached.hp); }
        } }
    }
    let net = their_dead - (our_dead + baseline);
    let our_unit = if our_n > 0 { (0..our_n).map(|i| our_dps[i]).sum::<i64>() / (our_n as i64) } else { 0 };
    let line: u8 = match dir {
        1 => if net < -our_unit { 2 } else if net > -1 { 0 } else { 3 },
        -1 => if net > our_unit { 0 } else if net < 1 { 2 } else { 3 },
        _ => if net > our_unit { 0 } else if net < -our_unit { 2 } else { 3 },
    };
    tr += &format!(" net={} our_unit={} dead=({},{})", net, our_unit, our_dead, their_dead);
    (first_focus, soaker_id, net, line, tr)
}

fn main() {
    let av: Vec<String> = std::env::args().collect();
    if av.len() > 99 { let _ = game_ai::check_kill_die_tick as *const (); }
    let mut m = HashMap::new();
    for s in av.iter().skip(1) { if let Some((k, v)) = s.split_once('=') { m.insert(k.to_string(), v.to_string()); } }
    let a = Args { m };
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    game.set_tick(a.i("tick", 3000) as usize);
    let version = a.i("version", 2) as usize;
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let seed = a.i("seed", 1) as u64;
    let mut g = Lcg(seed.wrapping_mul(0x9E3779B97F4A7C15) ^ 0xabcdef);
    // 세계 조립: 챔프 10명 위치·hp·max_hp·공격 이펙트 무작위
    let cx = 480000u64; let cy = 480000u64;
    for t in 0..2usize { for p in 0..5usize {
        let e = cache.player_champion[t][p].unwrap(); let eb = ep(e);
        let spread = a.i("spread", 60000) as u64;
        wr(eb, 0x660, cx + g.r(spread)); wr(eb, 0x668, cy + g.r(spread) + (t as u64) * 30000);
        let mhp = 500 + g.r(2000) as usize; let hp = 1 + g.r(mhp as u64) as usize;
        wr(eb, 0x628, mhp); wr(eb, 0x670, hp);
        let dmg = if a.i("nodmg", 0) == 1 { 0 } else { 10 + g.r(200) as usize };
        unsafe { std::ptr::write(&mut (*(eb as *mut Entity)).attack_effect, Some(mkeff(dmg, 100000 + g.r(200000)))); }
    } }
    let na = a.i("na", -1); let ne = a.i("ne", -1);
    let na = if na < 0 { 1 + g.r(5) as usize } else { na as usize };
    let ne = if ne < 0 { 1 + g.r(5) as usize } else { ne as usize };
    let me = a.i("me", 0) as usize;
    let champ = cache.player_champion[0][me].unwrap();
    let mut allies: Vec<&Entity> = Vec::new();
    for i in 0..na { allies.push(cache.player_champion[0][i % 5].unwrap()); }
    let mut enemies: Vec<&Entity> = Vec::new();
    for i in 0..ne { enemies.push(cache.player_champion[1][i % 5].unwrap()); }
    let dir: i8 = a.i("dir", (g.r(3) as i64) - 1) as i8;
    let judge = a.i("judge", if g.r(2) == 0 { 1000 } else { (g.r(1000)) as i64 }) as usize;
    let tower: Option<&Entity> = if a.i("tower", g.r(2) as i64) == 1 { cache.top_tower[1] } else { None };
    if let Some(t) = tower { let tb = ep(t); let tdmg = a.i("tdmg", 50 + g.r(300) as i64) as usize;
        unsafe { std::ptr::write(&mut (*(tb as *mut Entity)).attack_effect, Some(mkeff(tdmg, 400000))); } }
    println!("case\tseed={}\tna={}\tne={}\tdir={}\tjudge={}\ttower={}\tversion={}\tgseed={}", seed, na, ne, dir, judge, tower.is_some(), version, game.seed());
    for (k, e) in allies.iter().enumerate().take(5) { println!("ally{}\tid={}\tpos=({},{})\thp={}/{}\tdps_vs_e0={}", k, e.id, e.x, e.y, e.hp, e.stat_cached.hp, if enemies.is_empty() { 0 } else { game_ai::fight_dps(version, &ctx, e, enemies[0]) }); }
    for (k, e) in enemies.iter().enumerate().take(5) { println!("enemy{}\tid={}\tpos=({},{})\thp={}/{}", k, e.id, e.x, e.y, e.hp, e.stat_cached.hp); }
    if let Some(t) = tower { println!("tower\tid={}\tpos=({},{})\texp_dps_vs_a0={}", t.id, t.x, t.y, game_ai::expected_dps(&ctx, t, allies[0])); }
    // 예측(재구현) 먼저? — 헬퍼 TLS 메모는 같은 프로세스에서 같은 값이므로 순서 무관. 오라클을 먼저 부른다(첫 값 = 게임 순서).
    let mut buf = std::mem::MaybeUninit::<FightPrediction>::uninit();
    unsafe { std::ptr::write_bytes(buf.as_mut_ptr() as *mut u8, 0xAA, 64); }
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        let v = resolve_fight(version, &data, champ, &allies, &enemies, dir, tower, judge);
        std::ptr::write(buf.as_mut_ptr(), v);
    }));
    if let Err(e) = res {
        let msg = e.downcast_ref::<&str>().map(|s| s.to_string()).or_else(|| e.downcast_ref::<String>().cloned()).unwrap_or("?".into());
        println!("RESULT\tPANIC\t{}", msg.replace('\n', " ")); return;
    }
    let b = buf.as_ptr() as *const u8;
    let raw: [u8; 64] = unsafe { std::ptr::read(b as *const [u8; 64]) };
    let g_focus_tag: i64 = rd(b, 0); let g_focus: usize = rd(b, 8); let g_soak_tag: i64 = rd(b, 0x10); let g_soak: usize = rd(b, 0x18);
    let g_resc_tag: i64 = rd(b, 0x20); let g_net: i64 = rd(b, 0x30); let g_line: u8 = rd(b, 0x38); let g_line2: u8 = rd(b, 0x39);
    println!("game\tfocus={}:{}\tsoaker={}:{}\trescue_tag={}\tnet={}\tline={}\tline_abs={}\traw={}", g_focus_tag, g_focus, g_soak_tag, g_soak, g_resc_tag, g_net, g_line, g_line2,
             raw.iter().map(|x| format!("{:02x}", x)).collect::<Vec<_>>().join(""));
    let (m_focus, m_soak, m_net, m_line, tr) = reimpl(version, &ctx, &game as &dyn AbstractGame, champ, &allies, &enemies, dir, tower, judge, &[], 0);
    println!("mine\tfocus={:?}\tsoaker={:?}\tnet={}\tline={}\ttrace={}", m_focus, m_soak, m_net, m_line, tr);
    let ok_focus = match m_focus { None => g_focus_tag == 0, Some(v) => g_focus_tag == 1 && g_focus == v };
    let ok_soak = match m_soak { None => g_soak_tag == 0, Some(v) => g_soak_tag == 1 && g_soak == v };
    let all = ok_focus && ok_soak && g_resc_tag == 0 && g_net == m_net && g_line == m_line && g_line2 == m_line;
    println!("RESULT\t{}\tfocus={}\tsoak={}\tnet={}\tline={}", if all { "MATCH" } else { "DIFF" }, ok_focus, ok_soak, g_net == m_net, g_line == m_line && g_line2 == m_line);
}
