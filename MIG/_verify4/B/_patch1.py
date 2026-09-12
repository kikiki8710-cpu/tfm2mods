# -*- coding: utf-8 -*-
import io
p = r'C:\tfm2mods\MIG\_verify4\B\B4_o09m.rs'
s = io.open(p, encoding='utf-8').read()
mark = u'    // \u2500\u2500 \u2460 damage_at'
k = s.index(mark)
head = s[:k]
tail = u'''    // \u2500\u2500 \uace0HP \ub300\uc5ed \uc81c\uc5b4\uad70 (\ubc18\ud658 \uc0c1\ud55c\uc5d0 \uc548 \ubb3c\ub9ac\uac8c) \u2500\u2500
    let base = {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        cache.player_champion[0][0].unwrap().clone()
    };
    println!("BASE\\thp={}\\tmax={}\\tx={}\\ty={}", base.hp, base.stat_cached.hp, base.x, base.y);

    let probe_pts: Vec<(u64, u64)> = {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let mut v = Vec::new();
        for m in cache.iter_minions(1) { v.push((m.x, m.y)); if v.len() >= 4 { break; } }
        v
    };
    println!("PROBE_PTS\\t{:?}", probe_pts);
    let tps = setting.tick_per_second;
    const BIG: usize = 1_000_000;

    // (1) damage_at / danger / wave_risk
    for (i, &(px, py)) in probe_pts.iter().enumerate() {
        let mut ce = base.clone();
        ce.hp = BIG; ce.stat_cached.hp = BIG; ce.x = px; ce.y = py;
        let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        cache.player_champion[0][0] = Some(&ce);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let d_raw = game_ai::enemy_minion_line_action_damage_at(0, &data, &ce, px, py, tps * 2, true, false);
        let d_dng = game_ai::enemy_minion_line_action_danger_damage_at(0, &data, &ce, px, py, tps * 2, true, false);
        let d_wav = game_ai::enemy_minion_wave_risk_damage_at(0, &data, &ce, px, py, tps * 2);
        println!("PT{}\\t({},{})\\tdamage_at={}\\tdanger={}\\twave_risk={}", i, px, py, d_raw, d_dng, d_wav);
    }

    // (2) epic_minion_buff_time != 0 -> damage_at delegates to wave_risk ?
    if let Some(&(px, py)) = probe_pts.first() {
        for buff in [0usize, 1usize, 600usize] {
            game.mode.epic_minion_buff_time = [buff, buff];
            let mut ce = base.clone();
            ce.hp = BIG; ce.stat_cached.hp = BIG; ce.x = px; ce.y = py;
            let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            cache.player_champion[0][0] = Some(&ce);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let d_raw = game_ai::enemy_minion_line_action_damage_at(0, &data, &ce, px, py, tps * 2, true, false);
            let d_wav = game_ai::enemy_minion_wave_risk_damage_at(0, &data, &ce, px, py, tps * 2);
            let t_t = game_ai::enemy_minion_line_action_danger_damage_at(0, &data, &ce, px, py, tps * 2, true, false);
            let t_f = game_ai::enemy_minion_line_action_danger_damage_at(0, &data, &ce, px, py, tps * 2, false, false);
            println!("BUFF\\t{}\\tdamage_at={}\\twave_risk={}\\tequal={}\\tdangerT={}\\tdangerF={}",
                     buff, d_raw, d_wav, d_raw == d_wav, t_t, t_f);
        }
        game.mode.epic_minion_buff_time = [0, 0];
    }

    // (3) window_tick clamp
    if let Some(&(px, py)) = probe_pts.first() {
        let mut ce = base.clone();
        ce.hp = BIG; ce.stat_cached.hp = BIG; ce.x = px; ce.y = py;
        let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        cache.player_champion[0][0] = Some(&ce);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        for w in [0usize, 1, 15, 29, 30, 31, 45, 59, 60, 61, 120, 600] {
            let d = game_ai::enemy_minion_line_action_damage_at(0, &data, &ce, px, py, w, true, false);
            println!("WIN\\tw={}\\tdamage_at={}", w, d);
        }
    }

    // (4) champion_action / predict_retarget 4 combos
    if let Some(&(px, py)) = probe_pts.first() {
        let mut ce = base.clone();
        ce.hp = BIG; ce.stat_cached.hp = BIG; ce.x = px; ce.y = py;
        let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        cache.player_champion[0][0] = Some(&ce);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        for ca in [false, true] { for pr in [false, true] {
            let d = game_ai::enemy_minion_line_action_damage_at(0, &data, &ce, px, py, tps * 2, ca, pr);
            println!("FLAG\\tca={}\\tpr={}\\tdamage_at={}", ca, pr, d);
        }}
    }

    // (5) return cap = damage.min(max(hp*2,1))
    if let Some(&(px, py)) = probe_pts.first() {
        let raw = {
            let mut ce = base.clone();
            ce.hp = BIG; ce.stat_cached.hp = BIG; ce.x = px; ce.y = py;
            let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            cache.player_champion[0][0] = Some(&ce);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            game_ai::enemy_minion_line_action_damage_at(0, &data, &ce, px, py, tps * 2, true, false)
        };
        println!("RAW\\t{}", raw);
        for hp in [0usize, 1, 2, 3, 10, 50, 100, raw / 2, raw, raw * 2] {
            let mut ce = base.clone();
            ce.stat_cached.hp = BIG; ce.x = px; ce.y = py; ce.hp = hp;
            let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            cache.player_champion[0][0] = Some(&ce);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let d = game_ai::enemy_minion_line_action_damage_at(0, &data, &ce, px, py, tps * 2, true, false);
            let cap = std::cmp::max(hp.saturating_mul(2), 1);
            println!("CAP\\thp={}\\tcap={}\\tdamage_at={}\\tpred={}\\tmatch={}",
                     hp, cap, d, std::cmp::min(raw, cap), d == std::cmp::min(raw, cap));
        }
    }

    // (6) danger/critical threshold tables
    if let Some(&(px, py)) = probe_pts.first() {
        let cases: [(usize, usize); 9] = [(BIG, BIG), (1000, 1000), (400, 1000), (300, 1000), (200, 1000),
                                          (100, 1000), (60, 1000), (30, 1000), (10, 1000)];
        for (hp, mx) in cases {
            let mut ce = base.clone();
            ce.x = px; ce.y = py; ce.hp = hp; ce.stat_cached.hp = mx;
            let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            cache.player_champion[0][0] = Some(&ce);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let raw = game_ai::enemy_minion_line_action_damage_at(0, &data, &ce, px, py, tps * 2, true, false);
            let d_t = game_ai::enemy_minion_line_action_danger_damage_at(0, &data, &ce, px, py, tps * 2, true, false);
            let d_f = game_ai::enemy_minion_line_action_danger_damage_at(0, &data, &ce, px, py, tps * 2, false, false);
            let hp_pct = hp * 100 / std::cmp::max(mx, 1);
            let dmg_pct = raw * 100 / std::cmp::max(hp, 1);
            let dg = raw != 0 && (raw >= hp || dmg_pct > 49 || (hp_pct < 66 && dmg_pct > 29)
                     || (hp_pct < 41 && dmg_pct > 17) || (hp_pct < 26 && dmg_pct > 9));
            let cr = raw != 0 && (raw >= hp || (hp_pct < 26 && dmg_pct > 34) || (hp_pct < 16 && dmg_pct > 19));
            let pdg = if dg { raw } else { 0 };
            let pcr = if cr { raw } else { 0 };
            println!("TBL\\thp={}/{}\\thp_pct={}\\traw={}\\tdmg_pct={}\\tdangerT={}\\tpredDG={}\\tdangerF={}\\tpredCR={}\\tokT={}\\tokF={}",
                     hp, mx, hp_pct, raw, dmg_pct, d_t, pdg, d_f, pcr, d_t == pdg, d_f == pcr);
        }
    }

    // (7) 09 body with enemy placed on a minion line
    {
        let mut ce = base.clone();
        ce.hp = BIG; ce.stat_cached.hp = BIG;
        let enemy_base = {
            let c2 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            c2.player_champion[1][0].unwrap().clone()
        };
        for &(px, py) in probe_pts.iter() {
            let mut en = enemy_base.clone(); en.x = px; en.y = py;
            let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            cache.player_champion[0][0] = Some(&ce);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let ps = game.get_player_by_position(0, Position::Top).unwrap();
            let dmg = game_ai::enemy_minion_line_action_danger_damage_at(0, &data, &ce, px, py, tps * 2, true, false);
            let r = game_ai::check_favorable_engage_formation(0, ps, &data, &en, 200000);
            println!("FORM\\tenemy=({},{})\\tgate_dmg={}\\tresult={}", px, py, dmg, r);
        }
    }
    println!("DONE\\tsetting_ok={}", ok);
}
'''
io.open(p, 'w', encoding='utf-8', newline='\n').write(head + tail)
print('written', len(head + tail))
