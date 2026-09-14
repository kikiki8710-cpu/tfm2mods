# -*- coding: utf-8 -*-
"""v22A_o1.rs 에 113 슬롯별 damage(perslot)·killmask 옵션을 넣는다."""
import io
p = r'C:\tfm2mods\MIG\_verify22\A\oracle\v22A_o1.rs'
s = io.open(p, encoding='utf-8').read()
old = """        if tatk >= 0 {
            for t in [cache.top_tower[1], cache.mid_tower[1], cache.bottom_tower[1], cache.top_tower2[1], cache.mid_tower2[1], cache.bottom_tower2[1]].iter().flatten() {
                // 타워 attack_effect.ty 를 TowerAttackEffect(damage=tatk, ratio=0) 로 교체(Arc 팻포인터 16B 덮어쓰기 · 옛 Arc 는 누수)
                let arc: Arc<dyn EffectType> = Arc::new(TowerAttackEffect::new(tatk as usize, 0));
                let raw: [usize; 2] = unsafe { std::mem::transmute(arc) };
                wr(ep(*t as *const Entity), 0x490, raw[0]); wr(ep(*t as *const Entity), 0x498, raw[1]);
            }
        }
"""
new = """        let perslot = arg(&a, 9, 0); let killmask = arg(&a, 10, 0);
        if tatk >= 0 {
            let slots = [cache.top_tower[1], cache.mid_tower[1], cache.bottom_tower[1], cache.top_tower2[1], cache.mid_tower2[1], cache.bottom_tower2[1]];
            let mut k = 0usize;
            for t in slots.iter().flatten() {
                // 타워 attack_effect.ty 를 TowerAttackEffect(damage=tatk(+슬롯별 가산), ratio=0) 로 교체(Arc 팻포인터 16B 덮어쓰기 · 옛 Arc 는 누수)
                let dmg = tatk as usize + if perslot == 1 { k * 1000 } else { 0 };
                let arc: Arc<dyn EffectType> = Arc::new(TowerAttackEffect::new(dmg, 0));
                let raw: [usize; 2] = unsafe { std::mem::transmute(arc) };
                wr(ep(*t as *const Entity), 0x490, raw[0]); wr(ep(*t as *const Entity), 0x498, raw[1]);
                if (killmask >> k) & 1 == 1 { wr(ep(*t as *const Entity), 0x4c0, -1i32); }
                k += 1;
            }
            let mut j = 0usize;
            for t in cache.twin_towers[1].iter() {
                let dmg = tatk as usize + if perslot == 1 { 10000 + j * 1000 } else { 0 };
                let arc: Arc<dyn EffectType> = Arc::new(TowerAttackEffect::new(dmg, 0));
                let raw: [usize; 2] = unsafe { std::mem::transmute(arc) };
                wr(ep(*t as *const Entity), 0x490, raw[0]); wr(ep(*t as *const Entity), 0x498, raw[1]);
                if (killmask >> (6 + j)) & 1 == 1 { wr(ep(*t as *const Entity), 0x4c0, -1i32); }
                j += 1;
            }
            println!("113\\tslots ids={:?}\\ttwin ids={:?}", slots.iter().map(|o| o.map(|e| e.id)).collect::<Vec<_>>(), cache.twin_towers[1].iter().map(|e| e.id).collect::<Vec<_>>());
        }
"""
assert old in s
s = s.replace(old, new)
io.open(p, 'w', encoding='utf-8').write(s)
print('patched')
