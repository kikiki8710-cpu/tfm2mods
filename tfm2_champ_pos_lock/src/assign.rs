//! assign — 포지션 배정 순수 로직(클래식 lib.rs 에서 그대로 추출, SDK 무관).
//! 마스크 = 챔피언별 5비트(bit p = 포지션 p 허용). `MASK_ALL` = 무제한.
#![allow(dead_code)]
use crate::config::MASK_ALL;

/// masks[i] 를 서로 다른 포지션에 배정하는 해 하나(포지션 인덱스 벡터). 없으면 None.
pub fn assign_positions(masks: &[u8]) -> Option<Vec<usize>> {
    fn go(masks: &[u8], i: usize, used: u8, out: &mut Vec<usize>) -> bool {
        if i == masks.len() { return true; }
        for p in 0..5 {
            let bit = 1u8 << p;
            if masks[i] & bit != 0 && used & bit == 0 {
                out.push(p);
                if go(masks, i + 1, used | bit, out) { return true; }
                out.pop();
            }
        }
        false
    }
    let mut out = Vec::with_capacity(masks.len());
    if go(masks, 0, 0, &mut out) { Some(out) } else { None }
}

/// 스왑 order 최적해. `order[포지션] = 픽 인덱스`. 5! 전수: ①설정에 맞게 앉은 인원 최대 ②게임 order 와 일치 최대.
pub fn best_order(masks: &[u8], cur: &[u64]) -> (Vec<u64>, usize) {
    fn fits(m: u8, p: usize) -> bool { m == MASK_ALL || m & (1 << p) != 0 }
    let n = masks.len().min(5);
    let mut idx: Vec<usize> = (0..n).collect();
    let mut best: (usize, usize, Vec<u64>) = (0, 0, (0..n as u64).collect());
    fn perm(k: usize, idx: &mut Vec<usize>, masks: &[u8], cur: &[u64], best: &mut (usize, usize, Vec<u64>)) {
        if k == 1 {
            let n = idx.len();
            let matched = (0..n).filter(|&p| fits(masks[idx[p]], p)).count();
            let agree = (0..n).filter(|&p| cur.get(p) == Some(&(idx[p] as u64))).count();
            if (matched, agree) > (best.0, best.1) { *best = (matched, agree, idx.iter().map(|&x| x as u64).collect()); }
            return;
        }
        for i in 0..k {
            perm(k - 1, idx, masks, cur, best);
            if k % 2 == 0 { idx.swap(i, k - 1); } else { idx.swap(0, k - 1); }
        }
    }
    if n > 0 { perm(n, &mut idx, masks, cur, &mut best); }
    (best.2, best.0)
}

/// 지금 order 로 몇 명이 설정에 맞게 앉았는지.
pub fn order_matched(masks: &[u8], ord: &[u64]) -> usize {
    (0..masks.len().min(5)).filter(|&p| { let m = masks[ord[p] as usize]; m == MASK_ALL || m & (1 << p) != 0 }).count()
}

/// masks 중 서로 다른 포지션에 앉힐 수 있는 최대 인원.
pub fn max_match(masks: &[u8]) -> usize {
    fn rec(ms: &[u8], used: u8) -> usize {
        let Some((&m, rest)) = ms.split_first() else { return 0 };
        let mut best = rec(rest, used);
        for p in 0..5u8 {
            let b = 1u8 << p;
            if m & b != 0 && used & b == 0 { let v = 1 + rec(rest, used | b); if v > best { best = v; } }
        }
        best
    }
    rec(masks, 0)
}

/// cand 를 추가하면 커버되는 포지션 수가 늘어나는가(= 새 라인을 채우는가).
pub fn helps(pinned: &[u8], cand: u8) -> bool {
    let base = max_match(pinned);
    let mut v = pinned.to_vec();
    v.push(cand);
    max_match(&v) > base
}

/// masks 의 부분집합으로 정확히 S 를 덮을 수 있는 S 들의 비트셋(bit0 = 공집합).
fn cover_sets(masks: &[u8]) -> u32 {
    let mut reach: u32 = 1;
    for &m in masks {
        let mut nxt = reach;
        for s in 0..32usize {
            if reach & (1 << s) == 0 { continue; }
            for p in 0..5usize { if m & (1 << p) != 0 && s & (1 << p) == 0 { nxt |= 1 << (s | (1 << p)); } }
        }
        reach = nxt;
    }
    reach
}

/// 남은 풀이 서로 다른 챔프로 S 전부를 덮을 수 있는 S 들의 비트셋(Hall 조건).
fn pool_cover_sets(pool: &[u8]) -> u32 {
    let mut by = [0usize; 32];
    for &m in pool { by[(m & MASK_ALL) as usize] += 1; }
    let mut avail = [0usize; 32];
    for t in 1..32usize { avail[t] = (1..32usize).filter(|m| m & t != 0).map(|m| by[m]).sum(); }
    let mut ok: u32 = 1;
    for s in 1..32usize {
        let mut good = true;
        let mut t = s;
        while t > 0 { if avail[t] < t.count_ones() as usize { good = false; break; } t = (t - 1) & s; }
        if good { ok |= 1 << s; }
    }
    ok
}

/// 이 팀이 최종적으로 설정대로 앉힐 수 있는 최대 인원. pinned = 이미 픽한 마스크 전부, pool = 아직 고를 수 있는 마스크.
pub fn achievable(pinned: &[u8], pool: &[u8], slots: usize) -> usize {
    let pin = cover_sets(pinned);
    let pl = pool_cover_sets(pool);
    let mut best = 0usize;
    for a in 0..32usize {
        if pin & (1 << a) == 0 { continue; }
        let rest = MASK_ALL as usize & !a;
        let mut b = rest;
        loop {
            if pl & (1 << b) != 0 && b.count_ones() as usize <= slots {
                let v = a.count_ones() as usize + b.count_ones() as usize;
                if v > best { best = v; }
            }
            if b == 0 { break; }
            b = (b - 1) & rest;
        }
    }
    best
}

/// 아직 안 쓴 자유 슬롯 수. >0 이면 이번 픽은 무엇을 골라도 최종 정배치 인원이 줄지 않는다.
pub fn free_left(pinned: &[u8], pool: &[u8], team: usize) -> usize {
    let slots = team.saturating_sub(pinned.len());
    let target = achievable(pinned, pool, slots);
    let budget = team.saturating_sub(target);
    let used = pinned.len().saturating_sub(max_match(pinned));
    budget.saturating_sub(used)
}

/// masks 전부를 서로 다른 포지션에 앉힐 수 있는가(정렬 부작용 있음).
pub fn feasible(masks: &mut Vec<u8>) -> bool {
    if masks.len() > 5 { return false; }
    masks.sort_by_key(|m| m.count_ones());
    fn rec(ms: &[u8], used: u8) -> bool {
        let Some((&m, rest)) = ms.split_first() else { return true };
        for p in 0..5u8 { let b = 1u8 << p; if m & b != 0 && used & b == 0 && rec(rest, used | b) { return true; } }
        false
    }
    rec(masks, 0)
}

/// 유저/AI 픽 공통 판정: 내 팀이 이미 고른 마스크(pinned_all, 무제한 포함)·남은 풀(pool)·팀 인원(team)에서
/// 후보 cand(마스크)를 고르는 것이 허용되는가. 반환 (허용, 사유코드): 0=허용 1=중복라인(차단) / 자유슬롯 있으면 허용.
pub fn pick_allowed(pinned_all: &[u8], pool: &[u8], team: usize, cand: u8) -> (bool, u8) {
    if cand == MASK_ALL { return (true, 0); }
    if free_left(pinned_all, pool, team) > 0 { return (true, 0); }
    let pinned: Vec<u8> = pinned_all.iter().copied().filter(|&m| m != MASK_ALL).collect();
    if helps(&pinned, cand) { (true, 0) } else { (false, 1) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basics() {
        assert!(feasible(&mut vec![0b00001, 0b00010, 0b11111]));
        assert!(!feasible(&mut vec![0b00001, 0b00001]));
        assert_eq!(max_match(&[0b00001, 0b00001, 0b00010]), 2);
        assert!(helps(&[0b00001], 0b00010));
        assert!(!helps(&[0b00001], 0b00001));
        let (ord, n) = best_order(&[0b00010, 0b00001, 0b11111, 0b11111, 0b11111], &[0, 1, 2, 3, 4]);
        assert_eq!(n, 5); assert_eq!(ord[0], 1); assert_eq!(ord[1], 0);
        // 자유 슬롯: 서폿 풀이 없으면 서폿 자리는 아무나
        let pool = vec![0b00001, 0b00010, 0b00100, 0b01000];
        assert_eq!(free_left(&[], &pool, 5), 1);
        assert!(pick_allowed(&[0b00001], &pool, 5, 0b00001).0);
    }
}
