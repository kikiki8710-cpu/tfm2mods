const MASK_ALL: u8 = 0b11111;
const SERIES_GAMES: usize = 5;
fn compute(des: &Vec<u8>, style: u8, ban: Option<usize>, named_bits: u8) -> ([bool;5],[u16;32],[u16;32]) {
        let b = ban.unwrap_or(usize::MAX);
        let mut have = [0u16; 32];
        let mut need = [0u16; 32];
        let mut worst = [0u8; 5];
        // 시작 활성 집합 = 목록이 있는(로스터에 실재하는 지정이 있는) 포지션
        let mut active_bits: u8 = named_bits;
        let mut have_first = [0u16; 32];
        let mut need_first = [0u16; 32];
        let mut first = true;
        loop {
            let free = MASK_ALL & !active_bits;
            // 실효 마스크 = 규칙 ①②③ (mask_of 와 동일 식)
            let eff: Vec<u8> = des.iter().map(|&d| {
                let dd = d & active_bits;
                if dd != 0 { dd } else if free != 0 { free } else { MASK_ALL }
            }).collect();
            for s in 1..32usize {
                if s & active_bits as usize != s { have[s] = 0; need[s] = 0; continue; }
                let mut n = 0usize;
                let mut l = 0u8;
                for &m in &eff { if m as usize & s != 0 { n += 1; l |= m; } }
                let lcnt = (l & active_bits).count_ones() as usize; // 상대 픽·잠금은 활성 라인 안에서만 의미
                let opp = lcnt.min(5);
                let lock = (SERIES_GAMES - 1) * match style { 2 => 2 * opp, 1 => opp, _ => 0 };
                let nd = if b == usize::MAX { usize::MAX } else { s.count_ones() as usize + 2 * b + opp + lock };
                have[s] = n.min(u16::MAX as usize) as u16;
                need[s] = nd.min(u16::MAX as usize) as u16;
            }
            if first { have_first = have; need_first = need; first = false; }
            let mut fail: u8 = 0;
            for p in 0..5 {
                if active_bits & (1 << p) == 0 { continue; }
                let mut ws = (i64::MAX, 1usize << p);
                for s in 1..32usize {
                    if s & (1 << p) == 0 || s & active_bits as usize != s { continue; }
                    let slack = have[s] as i64 - need[s] as i64;
                    if slack < ws.0 { ws = (slack, s); }
                }
                worst[p] = ws.1 as u8;
                if ws.0 < 0 { fail |= 1 << p; }
            }
            if fail == 0 { break; }
            active_bits &= !fail;
        }
        let mut active = [false; 5];
        for p in 0..5 { active[p] = active_bits & (1 << p) != 0; }
        // 표시용 have/need: 꺼진 포지션은 첫 반복(원본 목록) 값, 살아남은 포지션은 최종 값
        for s in 1..32usize {
            if have[s] == 0 && need[s] == 0 { have[s] = have_first[s]; need[s] = need_first[s]; }
        }

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
