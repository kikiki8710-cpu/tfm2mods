const MASK_ALL: u8 = 0b11111;
const SERIES_GAMES: usize = 5;
fn compute(des: &Vec<u8>, style: u8, ban: Option<usize>, named_in: u8) -> ([bool;5],[u16;32],[u16;32]) {
        let b = ban.unwrap_or(usize::MAX);
        let named_bits: u8 = named_in;
        // 활성 집합 A 에서의 (have, need) 표 — S ⊄ A 는 0.
        let table = |a: u8, have: &mut [u16; 32], need: &mut [u16; 32]| {
            let free = MASK_ALL & !a;
            let eff: Vec<u8> = des.iter().map(|&d| { let dd = d & a; if dd != 0 { dd } else if free != 0 { free } else { MASK_ALL } }).collect();
            for s in 1..32usize {
                if s & a as usize != s { have[s] = 0; need[s] = 0; continue; }
                let mut n = 0usize; let mut l = 0u8;
                for &m in &eff { if m as usize & s != 0 { n += 1; l |= m; } }
                let opp = ((l & a).count_ones() as usize).min(5);
                let lock = (SERIES_GAMES - 1) * match style { 2 => 2 * opp, 1 => opp, _ => 0 };
                let nd = if b == usize::MAX { usize::MAX } else { s.count_ones() as usize + 2 * b + opp + lock };
                have[s] = n.min(u16::MAX as usize) as u16;
                need[s] = nd.min(u16::MAX as usize) as u16;
            }
        };
        let feasible = |a: u8, have: &[u16; 32], need: &[u16; 32]| -> (bool, i64) {
            let mut mn = i64::MAX;
            for s in 1..32usize {
                if s & a as usize != s { continue; }
                let sl = have[s] as i64 - need[s] as i64;
                if sl < mn { mn = sl; }
            }
            (mn >= 0 || a == 0, mn)
        };
        // ★[2026-09-16 4차] 순서 의존 제거: A ⊆ named 32가지를 전부 검사해 **살아남는 포지션 수 최대**(동률이면 최소 슬랙 최대)를 택한다.
        //   (한 포지션을 끄면 그 지정이 사라진 챔프의 마스크가 바뀌어 다른 묶음의 필요치가 달라지므로 순차 제거는 결과가 순서에 좌우된다.)
        // 동률 처리: ①살아남는 포지션 수 ②그 포지션에만 지정된(전용) 챔프 수 합(유저 의도 보존) ③최소 슬랙.
        let excl: [u32; 5] = core::array::from_fn(|p| des.iter().filter(|&&d| d == 1 << p).count() as u32);
        let mut best: Option<(u8, (u32, u32, i64))> = None;
        let mut th = [0u16; 32]; let mut tn = [0u16; 32];
        for a in 0..32u8 {
            if a & !named_bits != 0 { continue; }
            table(a, &mut th, &mut tn);
            let (ok, mn) = feasible(a, &th, &tn);
            if !ok { continue; }
            let ex: u32 = (0..5).filter(|&p| a & (1 << p) != 0).map(|p| excl[p]).sum();
            let key = (a.count_ones(), ex, mn);
            if best.map_or(true, |(_, k)| key > k) { best = Some((a, key)); }
        }
        let a = best.map(|x| x.0).unwrap_or(0);
        let mut have = [0u16; 32]; let mut need = [0u16; 32]; let mut worst = [0u8; 5];
        table(a, &mut have, &mut need);
        for p in 0..5 {
            if a & (1 << p) != 0 {
                let mut ws = (i64::MAX, 1usize << p);
                for s in 1..32usize {
                    if s & (1 << p) == 0 || s & a as usize != s { continue; }
                    let sl = have[s] as i64 - need[s] as i64;
                    if sl < ws.0 { ws = (sl, s); }
                }
                worst[p] = ws.1 as u8;
            } else if named_bits & (1 << p) != 0 {
                // 꺼진 포지션: A∪{p} 에서 p 를 포함하는 가장 나쁜 S (왜 못 켜는지 표시용)
                let ap = a | (1 << p);
                table(ap, &mut th, &mut tn);
                let mut ws = (i64::MAX, 1usize << p);
                for s in 1..32usize {
                    if s & (1 << p) == 0 || s & ap as usize != s { continue; }
                    let sl = th[s] as i64 - tn[s] as i64;
                    if sl < ws.0 { ws = (sl, s); }
                }
                worst[p] = ws.1 as u8;
                for s in 1..32usize { if s & (1 << p) != 0 && s & ap as usize == s { have[s] = th[s]; need[s] = tn[s]; } }
            }
        }
        let mut active = [false; 5];
        for p in 0..5 { active[p] = a & (1 << p) != 0; }

        (active, have, need)
}
fn lcg(s: &mut u64) -> u64 { *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); *s >> 33 }
fn main() {
    let mut s: u64 = 777;
    for _ in 0..3000 {
        let n = 5 + (lcg(&mut s) % 40) as usize; let style = (lcg(&mut s) % 3) as u8; let b = (lcg(&mut s) % 6) as usize;
        let des: Vec<u8> = (0..n).map(|_| (lcg(&mut s) % 32) as u8).collect();
        let mut named = 0u8; for &d in &des { named |= d; }
        let (a, h, nd) = compute(&des, style, Some(b), named);
        let act: String = a.iter().map(|&x| if x {'1'} else {'0'}).collect();
        let hs: Vec<String> = h.iter().map(|x| x.to_string()).collect();
        let ns: Vec<String> = nd.iter().map(|x| x.to_string()).collect();
        println!("{} {} {}", act, hs.join(","), ns.join(","));
    }
}
