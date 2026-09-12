# -*- coding: utf-8 -*-
import io
p = r'C:\tfm2mods\MIG\_verify4\B\B4_o09m.rs'
s = io.open(p, encoding='utf-8').read()
mark = u'    println!("DONE\\tsetting_ok={}", ok);'
k = s.index(mark)
extra = u'''    // (8) buff x champion_action : \ud45c \uc120\ud0dd\uc774 `or(ca, buff!=0)` \uc778\uac00
    if let Some(&(px, py)) = probe_pts.first() {
        for buff in [0usize, 600usize] {
            game.mode.epic_minion_buff_time = [buff, buff];
            for (hp, mx) in [(400usize, 1000usize), (300usize, 1000usize)] {
                for ca in [false, true] {
                    let mut ce = base.clone();
                    ce.x = px; ce.y = py; ce.hp = hp; ce.stat_cached.hp = mx;
                    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
                    cache.player_champion[0][0] = Some(&ce);
                    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
                    let data = OperationData::new(&cache, &ctx, &bb);
                    let raw = game_ai::enemy_minion_line_action_damage_at(0, &data, &ce, px, py, tps * 2, ca, false);
                    let d = game_ai::enemy_minion_line_action_danger_damage_at(0, &data, &ce, px, py, tps * 2, ca, false);
                    let hp_pct = hp * 100 / mx;
                    let dmg_pct = raw * 100 / std::cmp::max(hp, 1);
                    let dg = raw != 0 && (raw >= hp || dmg_pct > 49 || (hp_pct < 66 && dmg_pct > 29)
                             || (hp_pct < 41 && dmg_pct > 17) || (hp_pct < 26 && dmg_pct > 9));
                    let cr = raw != 0 && (raw >= hp || (hp_pct < 26 && dmg_pct > 34) || (hp_pct < 16 && dmg_pct > 19));
                    let strict = ca || buff != 0;
                    let pred = if strict { if dg { raw } else { 0 } } else { if cr { raw } else { 0 } };
                    println!("OR\\tbuff={}\\tca={}\\thp={}/{}\\traw={}\\tdmg_pct={}\\tdanger={}\\tpred={}\\tmatch={}",
                             buff, ca, hp, mx, raw, dmg_pct, d, pred, d == pred);
                }
            }
        }
        game.mode.epic_minion_buff_time = [0, 0];
    }

    // (9) 09 \ubcf8\uc9c4 \uac8c\uc774\ud2b8 \ubc1c\ud654 : champ hp \ub97c \ub0ae\ucd94\uba74 dmg!=0 -> \uc9c1\ud589 false
    {
        let enemy_base = {
            let c2 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            c2.player_champion[1][0].unwrap().clone()
        };
        if let Some(&(px, py)) = probe_pts.first() {
            for (hp, mx) in [(1_000_000usize, 1_000_000usize), (400usize, 1000usize), (100usize, 1000usize)] {
                let mut ce = base.clone();
                ce.hp = hp; ce.stat_cached.hp = mx;
                // \uc790\uae30 \uc704\uce58\ub3c4 \uc801 \uc606\uc5d0 \ub454\ub2e4(front \uc804\uac1c \uc720\ub3c4)
                ce.x = px; ce.y = py;
                let mut en = enemy_base.clone(); en.x = px; en.y = py;
                let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
                cache.player_champion[0][0] = Some(&ce);
                let bb: [Blackboard; 2] = [Default::default(), Default::default()];
                let data = OperationData::new(&cache, &ctx, &bb);
                let ps = game.get_player_by_position(0, Position::Top).unwrap();
                let dmg = game_ai::enemy_minion_line_action_danger_damage_at(0, &data, &ce, px, py, tps * 2, true, false);
                let r = game_ai::check_favorable_engage_formation(0, ps, &data, &en, 200000);
                println!("GATE\\thp={}/{}\\tgate_dmg={}\\tformation={}", hp, mx, dmg, r);
            }
        }
    }
'''
s = s[:k] + extra + s[k:]
io.open(p, 'w', encoding='utf-8', newline='\n').write(s)
print('patched', len(s))
