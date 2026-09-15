const MASK_ALL: u8 = 0b11111;
fn max_match(masks: &[u8]) -> usize {
    fn rec(ms: &[u8], used: u8) -> usize {
        let Some((&m, rest)) = ms.split_first() else {
            return 0;
        };
        let mut best = rec(rest, used); // 이 챔프를 배정 안 함
        for p in 0..5u8 {
            let b = 1u8 << p;
            if m & b != 0 && used & b == 0 {
                let v = 1 + rec(rest, used | b);
                if v > best {
                    best = v;
                }
            }
        }
        best
    }
    rec(masks, 0)
}

fn helps(pinned: &[u8], cand: u8) -> bool {
    let base = max_match(pinned);
    let mut v = pinned.to_vec();
    v.push(cand);
    max_match(&v) > base
}

fn cover_sets(masks: &[u8]) -> u32 {
    let mut reach: u32 = 1; // bit0 = 공집합
    for &m in masks {
        let mut nxt = reach;
        for s in 0..32usize {
            if reach & (1 << s) == 0 {
                continue;
            }
            for p in 0..5usize {
                if m & (1 << p) != 0 && s & (1 << p) == 0 {
                    nxt |= 1 << (s | (1 << p));
                }
            }
        }
        reach = nxt;
    }
    reach
}

fn pool_cover_sets(pool: &[u8]) -> u32 {
    let mut by = [0usize; 32];
    for &m in pool {
        by[(m & MASK_ALL) as usize] += 1;
    }
    let mut avail = [0usize; 32];
    for t in 1..32usize {
        avail[t] = (1..32usize).filter(|m| m & t != 0).map(|m| by[m]).sum();
    }
    let mut ok: u32 = 1; // 공집합은 항상 가능
    for s in 1..32usize {
        let mut good = true;
        let mut t = s; // s 의 모든 비공집합 부분집합 순회
        while t > 0 {
            if avail[t] < t.count_ones() as usize {
                good = false;
                break;
            }
            t = (t - 1) & s;
        }
        if good {
            ok |= 1 << s;
        }
    }
    ok
}

fn achievable(pinned: &[u8], pool: &[u8], slots: usize) -> usize {
    let pin = cover_sets(pinned);
    let pl = pool_cover_sets(pool);
    let mut best = 0usize;
    for a in 0..32usize {
        if pin & (1 << a) == 0 {
            continue;
        }
        let rest = MASK_ALL as usize & !a;
        let mut b = rest;
        loop {
            if pl & (1 << b) != 0 && b.count_ones() as usize <= slots {
                let v = a.count_ones() as usize + b.count_ones() as usize;
                if v > best {
                    best = v;
                }
            }
            if b == 0 {
                break;
            }
            b = (b - 1) & rest;
        }
    }
    best
}

fn free_left(pinned: &[u8], pool: &[u8], team: usize) -> usize {
    let slots = team.saturating_sub(pinned.len());
    let target = achievable(pinned, pool, slots);
    let budget = team.saturating_sub(target); // 총 자유 슬롯
    let used = pinned.len().saturating_sub(max_match(pinned)); // 이미 쓴 자유 슬롯
    budget.saturating_sub(used)
}

fn feasible(masks: &mut Vec<u8>) -> bool {
    if masks.len() > 5 {
        return false;
    }
    masks.sort_by_key(|m| m.count_ones());
    fn rec(ms: &[u8], used: u8) -> bool {
        let Some((&m, rest)) = ms.split_first() else {
            return true;
        };
        for p in 0..5u8 {
            let b = 1u8 << p;
            if m & b != 0 && used & b == 0 && rec(rest, used | b) {
                return true;
            }
        }
        false
    }
    rec(masks, 0)
}

// 간단 LCG (파이썬과 동일 시퀀스)
fn lcg(s: &mut u64) -> u64 { *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); *s >> 33 }
fn main() {
    let mut s: u64 = 12345;
    for _ in 0..4000 {
        let np = (lcg(&mut s) % 6) as usize; let nq = (lcg(&mut s) % 12) as usize; let team = 2 + (lcg(&mut s) % 4) as usize;
        let pinned: Vec<u8> = (0..np).map(|_| (lcg(&mut s) % 32) as u8).collect();
        let pool: Vec<u8> = (0..nq).map(|_| (lcg(&mut s) % 32) as u8).collect();
        let cand = (lcg(&mut s) % 32) as u8;
        let mut f = pinned.clone();
        println!("{} {} {} {} {} {}", max_match(&pinned), helps(&pinned, cand) as u8, cover_sets(&pinned), pool_cover_sets(&pool), free_left(&pinned, &pool, team), feasible(&mut f) as u8);
    }
}
