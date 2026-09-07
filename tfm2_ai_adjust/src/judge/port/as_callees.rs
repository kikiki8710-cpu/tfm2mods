//! as_callees — 기저 스코어러(0xd57540) 콜리 **2단계 순수 포팅**(2026-09-06 19:20~).
//!   1단계(action_score.rs)는 콜리 값을 캡처로 받았다. 여기서 콜리를 하나씩 순수 재현해 캡처 훅을 "캡처+대조" 로 바꾸고,
//!   전부 DIFF=0 이 되면 base_score 를 live 로 올린다.
//!
//! ① `snapshot` = 0xc88300 "전투 컨텍스트 스냅샷"(sret 0x48B). 게임은 TLS 메모(키 seed/tick/sim.928, RefCell 플래그 +0)로 감싸지만
//!    값 자체는 순수(WorldOps vt+0xf8 가시성 · vt+0x150 로스터 · vt+0x28 틱 만 읽음). 디컴 dec/140c88300.c + fold 콜리
//!    0xe2fee0(적 챔프) / 0xd729b0(고정 6슬롯+미니언 체인) — 둘 다 `cmp acc,new; cmova ent; cmovae dist` = std `min_by_key`(동률 = 첫 원소).
//!    인자 p3 = [&seed, &tick, &sim.928, sim, X, lanes, self] (RE 기저스코어러 §3 L108).
//!    out: +0/+8 opt0 = 인지된 적 챔프 중 가장 가까운 (tag, 핸들) — 거리 제한 없음 · +0x10/+0x18 opt1 = 아군 고정6+미니언 중 150k 안 가장 가까운
//!         · +0x20/+0x28 opt2 = 적측 동일 · +0x30/+0x38 opt3 = 150k 안 인지된 적 챔프 중 가장 가까운 · +0x40 u8 적 챔프 마스크(150k ∧ 인지)
//!         · +0x41 u8 아군 챔프 수(≤100k, self 포함). tag=0 이면 핸들 워드는 게임도 쓰레기(미초기화 레지스터) → 대조 제외.
//!    인지(perceived) = vt_f8(my_side, h) || (rec = vt_150(h); rec != 0 && tick <= lanes[enemy*0x2e8+0x1e0+rec.9c0*8] + 0x78).
//!    바이트패치 사이트 0(aiport sites 실측) — 상수 그대로.
#![allow(dead_code)]
use crate::*;
use super::super::world::*;
use super::super::layout::*;

/// 150000² >> 8 (게임: `d2 >> 8 < 0x53d1ac1`)
const SNAP_STRUCT_D2_SHR8: u64 = 0x53d1ac1;
/// 150000² + 1 (적 챔프 마스크/opt3)
const SNAP_CHAMP_D2: u64 = 0x53d1ac101;
/// 100000² + 1 (아군 수)
const SNAP_ALLY_D2: u64 = 0x2540be401;

#[inline] fn sqd(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = if ax < bx { bx - ax } else { ax - bx }; let dy = if ay < by { by - ay } else { ay - by };
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}
#[inline] unsafe fn xy(e: usize) -> Option<(u64, u64)> { Some((rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?)) }

/// 0xc88300 인자 구조체(p3)에서 (sim, X, lanes, self) 를 꺼내 순수 재현.
pub unsafe fn snapshot_from_args(p3: usize) -> Option<[u64; 9]> {
    if !ptr_ok(p3) { return None; }
    let key = memo_key3(p3)?;
    if let Some(v) = SNAP_MEMO.with(|c| { let (k, v) = c.get(); if k == key { Some(v) } else { None } }) { return Some(v); }
    let sim = rd_u64(p3 + 0x18)? as usize; let x = rd_u64(p3 + 0x20)? as usize;
    let lanes = rd_u64(p3 + 0x28)? as usize; let me = rd_u64(p3 + 0x30)? as usize;
    if !ptr_ok(sim) || !ptr_ok(x) || !ptr_ok(lanes) || !ptr_ok(me) { return None; }
    let v = snapshot(sim, x, lanes, me)?;
    SNAP_MEMO.with(|c| c.set((key, v)));
    Some(v)
}
/// 게임 메모 키 = (*p[0], *p[1], *p[2]) 세 워드(seed, tick, sim/bb 키). 게임과 같은 호출 시점에 같은 키로 캐시해야 "메모 시점" 차이가 안 난다.
unsafe fn memo_key3(p: usize) -> Option<[u64; 3]> {
    let a = rd_u64(p)? as usize; let b = rd_u64(p + 8)? as usize; let c = rd_u64(p + 16)? as usize;
    if !ptr_ok(a) || !ptr_ok(b) || !ptr_ok(c) { return None; }
    Some([rd_u64(a)?, rd_u64(b)?, rd_u64(c)?])
}
thread_local! {
    static SNAP_MEMO: std::cell::Cell<([u64; 3], [u64; 9])> = const { std::cell::Cell::new(([0; 3], [0; 9])) };
    /// pct_c 메모: (키3, [(R.58 핸들, 값); 16]) — 게임은 hashbrown 이지만 에이전트 ≤10 이라 배열로 충분. 키가 바뀌면 통째로 비운다.
    static PCTC_MEMO: std::cell::RefCell<([u64; 3], Vec<(u64, i64)>)> = const { std::cell::RefCell::new(([0; 3], Vec::new())) };
}
pub fn memo_reset() { SNAP_MEMO.with(|c| c.set(([0; 3], [0; 9]))); PCTC_MEMO.with(|c| { let mut m = c.borrow_mut(); m.0 = [0; 3]; m.1.clear(); }); }

/// 스냅샷 본체. 반환 워드 = out 0x48B 를 9워드로 본 것(w[8] = mask | cnt<<8, 상위는 0).
pub unsafe fn snapshot(sim: usize, x: usize, lanes: usize, me: usize) -> Option<[u64; 9]> {
    let w = World { x, data: rd_u64(x)? as usize, vt: rd_u64(x + 8)? as usize };
    if !ptr_ok(w.data) || !ptr_ok(w.vt) { return None; }
    let my = rd_u64(sim + P5_SIDE)?; if my > 1 { return None; }
    let en = 1 - my;
    let tick = rd_u64(w.data + W_TICK)?;
    let (mx, my_y) = xy(me)?;
    let d2 = |e: usize| -> Option<u64> { let (ex, ey) = xy(e)?; Some(sqd(ex, ey, mx, my_y)) };
    // 인지: 지금 보이거나, 로스터 레코드의 마지막 목격 틱 + 0x78 이 현재 틱 이상
    let perceived = |e: usize| -> Option<bool> {
        let h = rd_u64(e + ENT_HANDLE)?;
        if w.visible(my, h)? { return Some(true); }
        let rec = w.roster_rec(h)?; if rec == 0 { return Some(false); }
        let idx = rd_u32(rec + REC_ROLE) as usize;
        let seen = rd_u64(lanes + (en as usize) * LANE_STRIDE + LANE_ROSTER + idx * 8)?;
        Some(tick <= seen.wrapping_add(LANE_ROSTER_MARGIN))
    };
    // opt0: 인지된 적 챔프 중 가장 가까운(거리 제한 없음, 동률 = 로스터 순 첫 원소)
    let mut best0: Option<(u64, usize)> = None;
    for i in 0..5usize {
        let e = rd_u64(x + X_ROSTER + (en as usize) * ROSTER_SIDE_STRIDE + i * 8)? as usize; if e == 0 { continue; }
        if !perceived(e)? { continue; }
        let d = d2(e)?;
        if best0.map_or(true, |b| d < b.0) { best0 = Some((d, e)); }
    }
    // opt1/opt2: side 별 고정 6슬롯 + 미니언(0x1820ee0 순서) 중 150k 안 가장 가까운
    let nearest_struct = |side: u64| -> Option<Option<usize>> {
        let s = side as usize; let mut best: Option<(u64, usize)> = None;
        let mut consider = |e: usize, best: &mut Option<(u64, usize)>| -> Option<()> {
            let d = d2(e)?;
            if d >> 8 < SNAP_STRUCT_D2_SHR8 && best.map_or(true, |b| d < b.0) { *best = Some((d, e)); }
            Some(())
        };
        for off in X_FIXED6 { let e = rd_u64(x + off + s * 8)? as usize; if e != 0 { consider(e, &mut best)?; } }
        let ptr = rd_u64(x + X_MINION_PTR + s * 0x20)? as usize; let len = rd_u64(x + X_MINION_LEN + s * 0x20)?;
        if len != 0 && !ptr_ok(ptr) { return None; }
        for i in 0..len.min(512) as usize { let e = rd_u64(ptr + i * 8)? as usize; if e == 0 { return None; } consider(e, &mut best)?; }
        Some(best.map(|b| b.1))
    };
    let s1 = nearest_struct(my)?; let s2 = nearest_struct(en)?;
    // opt3 + mask: 150k 안 ∧ 인지된 적 챔프
    let mut mask = 0u8; let mut best3: Option<(u64, usize)> = None;
    for i in 0..5usize {
        let e = rd_u64(x + X_ROSTER + (en as usize) * ROSTER_SIDE_STRIDE + i * 8)? as usize; if e == 0 { continue; }
        let d = d2(e)?; if d >= SNAP_CHAMP_D2 { continue; }
        if !perceived(e)? { continue; }
        mask |= 1 << i;
        if best3.map_or(true, |b| d < b.0) { best3 = Some((d, e)); }
    }
    // cnt: 100k 안 아군 챔프 수(self 포함)
    let mut cnt = 0u8;
    for j in 0..5usize {
        let e = rd_u64(x + X_ROSTER + (my as usize) * ROSTER_SIDE_STRIDE + j * 8)? as usize; if e == 0 { continue; }
        if d2(e)? < SNAP_ALLY_D2 { cnt = cnt.wrapping_add(1); }
    }
    let h = |o: Option<usize>| -> Option<(u64, u64)> { match o { Some(e) => Some((1, rd_u64(e + ENT_HANDLE)?)), None => Some((0, 0)) } };
    let (t0, h0) = h(best0.map(|b| b.1))?; let (t1, h1) = h(s1)?; let (t2, h2) = h(s2)?; let (t3, h3) = h(best3.map(|b| b.1))?;
    Some([t0, h0, t1, h1, t2, h2, t3, h3, mask as u64 | ((cnt as u64) << 8)])
}

/// 대조: tag 워드는 항상, 핸들 워드는 tag==1 일 때만, w[8] 은 하위 16비트만.
pub fn snapshot_eq(game: &[u64; 9], mine: &[u64; 9]) -> bool {
    for k in 0..4 {
        if (game[2 * k] & 0xff) != (mine[2 * k] & 0xff) { return false; }
        if (game[2 * k] & 0xff) == 1 && game[2 * k + 1] != mine[2 * k + 1] { return false; }
    }
    (game[8] & 0xffff) == (mine[8] & 0xffff)
}

// ─────────────────────────────────────────────────────────────────────────────────────────────
// ② `pct_c` = 0xc87fe0 "백분위 C" 의 값 함수 0xd390a0(bb, &R). 0xc87fe0 은 hashbrown 스레드로컬 메모(키 seed/tick/bb.970 이 바뀌면 표 초기화,
//    엔트리 키 = R+0x58 에이전트 핸들) — 값은 순수. 인자 p2 = [&seed, &tick, &bb.970, R, bb].
//    bb: +0x978 자기 팀 · +0x9d0 자기 값 · +0x9d8/+0x9e0/+0x9e8 자기 계수(base, ‰, ‱) · +0x14b8/+0x14d0 아군 Record Vec · +0x14d8/+0x14f0 적 Record Vec(stride 0xd8)
//    R(Record): +0x60 팀 · +0xb8 값 · +0xc0/+0xc8/+0xd0 계수 · +0xa8/+0xb0 가중치(값 백분위·점수 백분위)
//    raw[k] = 값(자기 → 아군 순 → 적 순, 총 ≤10) · 팀합 = raw 를 "팀 == R.60" / "≠" 둘로 나눈 합 · score[k] = base + 팀합*‰/1000 + 팀합*‱/10000(자기 팀합)
//    n>1: v1 = pct(raw vs R.b8) · v2 = pct(score vs R_score) · pct = ((eq/2)+lt)*100/(n−1) → clamp[0,100] → (<51 ? +50 : ×2) ; n≤1: 100/100
//    반환 = (v2*R.b0 + v1*R.a8)/100 (부호 절사 나눗셈). 바이트패치 사이트 0.
const BB_TEAM: usize = 0x978; const BB_SELF_VAL: usize = 0x9d0; const BB_SELF_C0: usize = 0x9d8; const BB_SELF_C1: usize = 0x9e0; const BB_SELF_C2: usize = 0x9e8;
const BB_RECA_PTR: usize = 0x14b8; const BB_RECA_LEN: usize = 0x14d0;
const R_TEAM: usize = 0x60; const R_VAL: usize = 0xb8; const R_C0: usize = 0xc0; const R_C1: usize = 0xc8; const R_C2: usize = 0xd0; const R_WA: usize = 0xa8; const R_WB: usize = 0xb0;

pub unsafe fn pct_c_from_args(p2: usize) -> Option<u64> {
    if !ptr_ok(p2) { return None; }
    let key = memo_key3(p2)?;
    let r = rd_u64(p2 + 0x18)? as usize; let bb = rd_u64(p2 + 0x20)? as usize;
    if !ptr_ok(r) || !ptr_ok(bb) { return None; }
    let h = rd_u64(r + AS_REC_AGENT_H)?;
    let hit = PCTC_MEMO.with(|c| { let mut m = c.borrow_mut(); if m.0 != key { m.0 = key; m.1.clear(); } m.1.iter().find(|e| e.0 == h).map(|e| e.1) });
    if let Some(v) = hit { return Some(v as u64); }
    let v = pct_c(bb, r)?;
    PCTC_MEMO.with(|c| { let mut m = c.borrow_mut(); if m.1.len() < 64 { m.1.push((h, v)); } });
    Some(v as u64)
}

#[inline] fn score_of(sum: i64, c0: i64, c1: i64, c2: i64) -> i64 {
    sum.wrapping_mul(c2).wrapping_div(10000).wrapping_add(c1.wrapping_mul(sum).wrapping_div(1000)).wrapping_add(c0)
}
#[inline] fn pct_of(eq: u64, lt: u64, n1: u64) -> i64 {
    let p = ((eq >> 1).wrapping_add(lt)).wrapping_mul(100);
    let p = if (p | n1) >> 32 == 0 { ((p as u32) / (n1 as u32)) as i64 } else { (p as i64).wrapping_div(n1 as i64) };
    let c = p.max(0).min(100);
    if p < 0x33 { c + 0x32 } else { c * 2 }
}
pub unsafe fn pct_c(bb: usize, r: usize) -> Option<i64> {
    let rt = rd_u64(r + R_TEAM)?;
    let mut raw = [0i64; 10]; let mut team_is_rt = [false; 10]; let mut n = 0usize;
    raw[0] = rd_i64(bb + BB_SELF_VAL)?; team_is_rt[0] = rd_u64(bb + BB_TEAM)? == rt; n = 1;
    let mut push = |ptr: usize, len: u64, n: &mut usize| -> Option<()> {
        if len == 0 { return Some(()); } if !ptr_ok(ptr) { return None; }
        for i in 0..len as usize { if *n >= 10 { break; } let rec = ptr + i * AS_REC_STRIDE; raw[*n] = rd_i64(rec + R_VAL)?; team_is_rt[*n] = rd_u64(rec + R_TEAM)? == rt; *n += 1; }
        Some(())
    };
    let (pa, la) = (rd_u64(bb + BB_RECA_PTR)? as usize, rd_u64(bb + BB_RECA_LEN)?);
    let (pb, lb) = (rd_u64(bb + AS_BB_RECB_PTR)? as usize, rd_u64(bb + AS_BB_RECB_LEN)?);
    push(pa, la, &mut n)?; push(pb, lb, &mut n)?;
    let mut sum_rt = 0i64; let mut sum_ot = 0i64;
    for k in 0..n { if team_is_rt[k] { sum_rt = sum_rt.wrapping_add(raw[k]); } else { sum_ot = sum_ot.wrapping_add(raw[k]); } }
    let sum_for = |is_rt: bool| if is_rt { sum_rt } else { sum_ot };
    let mut score = [0i64; 10];
    score[0] = score_of(sum_for(team_is_rt[0]), rd_i64(bb + BB_SELF_C0)?, rd_i64(bb + BB_SELF_C1)?, rd_i64(bb + BB_SELF_C2)?);
    let mut k = 1usize;
    for i in 0..la.min(9) as usize { if k >= n { break; } let rec = pa + i * AS_REC_STRIDE; score[k] = score_of(sum_for(team_is_rt[k]), rd_i64(rec + R_C0)?, rd_i64(rec + R_C1)?, rd_i64(rec + R_C2)?); k += 1; }
    for j in 0..lb as usize { if k >= n { break; } let rec = pb + j * AS_REC_STRIDE; score[k] = score_of(sum_for(team_is_rt[k]), rd_i64(rec + R_C0)?, rd_i64(rec + R_C1)?, rd_i64(rec + R_C2)?); k += 1; }
    let (v1, v2) = if n > 1 {
        let rv = rd_i64(r + R_VAL)?; let rs = score_of(sum_rt, rd_i64(r + R_C0)?, rd_i64(r + R_C1)?, rd_i64(r + R_C2)?);
        let (mut eq1, mut lt1, mut eq2, mut lt2) = (0u64, 0u64, 0u64, 0u64);
        for k in 0..n { if raw[k] == rv { eq1 += 1; } if raw[k] < rv { lt1 += 1; } if score[k] == rs { eq2 += 1; } if score[k] < rs { lt2 += 1; } }
        (pct_of(eq1, lt1, (n - 1) as u64), pct_of(eq2, lt2, (n - 1) as u64))
    } else { (100, 100) };
    Some(v2.wrapping_mul(rd_i64(r + R_WB)?).wrapping_add(v1.wrapping_mul(rd_i64(r + R_WA)?)).wrapping_div(100))
}

// ─────────────────────────────────────────────────────────────────────────────────────────────
// ③ `max_reach` = 0xe0e890(e, o): 4 스킬 슬롯(0x490 / 0x4c8 / 0x500[lv≥3] / 0x538[lv≥5]; ZERO desc 0x33e21a0 = id −1) 중 "o 를 겨냥 가능한" 슬롯의
//    사거리 최대값(부호없는 max). 슬롯 사용 가능 = slot.30(id) != −1 && kind_pred(slot.28, e, o).
//    reach(slot) = e.438 + slot.10 + (lv−1)*slot.18 + rng(e) + rng(o) + vt_e8(slot, e, o)  — reach_val 과 달리 rng(e) 를 항상 더한다.
//    kind_pred = 0x12a0180: e.6b9==1 && e.6a0==0 이어야 하고, 능력 종류(0~13) JT 로 판정(디스어셈 jt_12a0180.txt, 2026-09-06 19:32):
//      T = (e.0, e.8)·(o.0, o.8) 태그/값 — same_team = tag 같음 && (tag≠0 || 값 같음) / other_team = tag 다름 || (tag==0 && 값 다름)
//      (게이트: o.6b9==1 && o.6a0==0 — 대상 기준) 0 same_team · 1 same_team && o.kind==13 · 2 1+버프{0,1,2,6,8,9} · 3 1 && o≠e · 4 o==e · 5 other_team · 6 tag 같음&&tag==0&&값 다름&&o.kind∉{2,3}
//      7 other_team && o.kind==13 · 8 7+버프 · 9 (tag 같음&&tag==0&&값 다름&&o.kind==13) && o.688[e.8]!=0 (e.8<2, 아니면 panic) · 10 항상 · 11 o≠e && o.kind∉{2,3} · 12 o.kind==13 · 13 없음
//    바이트패치 사이트 0.
const ENT_FLAG_6B9: usize = 0x6b9; const ENT_STATE_6A0: usize = 0x6a0; const ENT_KIND: usize = 0x68;
const ENT_BUFF_PTR: usize = 0x2c8; const ENT_BUFF_LEN: usize = 0x2d0; const ENT_688: usize = 0x688;
const SLOT0: usize = 0x490; const SLOT1: usize = 0x4c8; const SLOT2: usize = 0x500; const SLOT3: usize = 0x538;
const BUFF_KIND_MASK: u32 = 0x347;

#[inline] unsafe fn rng_e(e: usize) -> Option<u64> {
    let p = rd_i32(e + ENT_F470)? as i64; let r = rd_u64(e + ENT_F680)?;
    Some(if p == 0 { r } else { ((p + 100) as u64).wrapping_mul(r) / 100 })
}
unsafe fn has_buff_in_set(o: usize) -> Option<bool> {
    let n = rd_u64(o + ENT_BUFF_LEN)?; if n == 0 { return Some(false); }
    let p = rd_u64(o + ENT_BUFF_PTR)? as usize; if !ptr_ok(p) { return None; }
    for i in 0..n.min(CAP_ITER) as usize { let k = rd_u32(p + i * 0x28); if k <= 9 && (BUFF_KIND_MASK >> k) & 1 == 1 { return Some(true); } }
    Some(false)
}
pub unsafe fn kind_pred(kind: u32, e: usize, o: usize) -> Option<bool> {
    // ★r8 = o(대상): 대상이 겨냥 가능 상태(+0x6b9==1 && +0x6a0==0)여야 한다 — e 가 아니다(첫 리플레이 1.4% DIFF 의 원인, 2026-09-06 19:40)
    if rd_u8(o + ENT_FLAG_6B9) != 1 || rd_u64(o + ENT_STATE_6A0)? != 0 { return Some(false); }
    let (te, ve) = (rd_u64(e)?, rd_u64(e + 8)?); let (to, vo) = (rd_u64(o)?, rd_u64(o + 8)?);
    let okind = rd_u32(o + ENT_KIND);
    let same_tag = te == to;
    let same_team = same_tag && (te != 0 || ve == vo);
    let other_team = !same_tag || (te == 0 && ve != vo);
    let self_eq = rd_u64(e + ENT_HANDLE)? == rd_u64(o + ENT_HANDLE)?;
    let not_23 = (okind & !1) != 2;
    Some(match kind {
        0 => same_team,
        1 => same_team && okind == 13,
        2 => same_team && okind == 13 && has_buff_in_set(o)?,
        3 => same_team && okind == 13 && !self_eq,
        4 => self_eq,
        5 => other_team,
        6 => same_tag && te == 0 && ve != vo && not_23,
        7 => other_team && okind == 13,
        8 => other_team && okind == 13 && has_buff_in_set(o)?,
        9 => { if !(same_tag && te == 0 && ve != vo && okind == 13) { false } else { if ve >= 2 { return None; } rd_u64(o + ENT_688 + (ve as usize) * 8)? != 0 } }
        10 => true,
        11 => !self_eq && not_23,
        12 => okind == 13,
        13 => false,
        _ => return None,
    })
}
unsafe fn slot_reach(slot: usize, e: usize, o: usize, lv: u64) -> Option<u64> {
    let base = rd_u64(slot + 0x10)?; let perlv = rd_u64(slot + 0x18)?;
    let bonus = super::dn_reach::eff_e8(rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize, e, o, 0)?;
    Some(rd_u64(e + ENT_F438)?.wrapping_add(base).wrapping_add(lv.wrapping_sub(1).wrapping_mul(perlv)).wrapping_add(rng_e(e)?).wrapping_add(rng_e(o)?).wrapping_add(bonus))
}
unsafe fn slot_usable(slot: usize, e: usize, o: usize) -> Option<bool> {
    if rd_i32(slot + 0x30)? == -1 { return Some(false); }
    kind_pred(rd_u32(slot + 0x28), e, o)
}
pub unsafe fn max_reach(e: usize, o: usize) -> Option<u64> {
    if !ptr_ok(e) || !ptr_ok(o) { return None; }
    let lv = rd_u64(e + ENT_LEVEL)?;
    let mut r = if slot_usable(e + SLOT0, e, o)? { slot_reach(e + SLOT0, e, o, lv)? } else { 0 };
    if slot_usable(e + SLOT1, e, o)? { r = r.max(slot_reach(e + SLOT1, e, o, lv)?); }
    if lv >= 3 && slot_usable(e + SLOT2, e, o)? { r = r.max(slot_reach(e + SLOT2, e, o, lv)?); }
    if lv >= 5 && slot_usable(e + SLOT3, e, o)? { r = r.max(slot_reach(e + SLOT3, e, o, lv)?); }
    Some(r)
}

// ─────────────────────────────────────────────────────────────────────────────────────────────
// 진단: capture_ring_cmp 의 DIFF/NA 줄에 붙는 콜리별 내부 상태 문자열
pub unsafe fn cmp_diag8(name: &str, p1: usize, p2: usize, p3: usize, p4: usize, p5: usize, p6: usize, p7: usize, p8: usize) -> String {
    if name == "as_132b310" { return super::position_eval::threat_diag(p1, p3, p4); }
    if name == "combat_score" { return super::combat_score::diag(p1, p3, p4); }
    cmp_diag8_x(name, p1, p2, p3, p4, p5, p6, p7, p8)
}
#[allow(clippy::too_many_arguments)]
pub unsafe fn cmp_diag8_x(name: &str, p1: usize, p2: usize, p3: usize, p4: usize, p5: usize, p6: usize, p7: usize, _p8: usize) -> String {
    if name == "as_eb82d0" { return super::fight_check::diag(p3, p4, p5, p6, p7); }
    cmp_diag(name, p1, p2, p3, p4)
}
pub unsafe fn cmp_diag(name: &str, p1: usize, p2: usize, _p3: usize, _p4: usize) -> String {
    match name {
        "as_d84db0" => super::position_eval::diag(p2, _p3, _p4),
        "as_e0e890" => {
            let (e, o) = (p1, p2);
            if !ptr_ok(e) || !ptr_ok(o) { return "bad ptr".into(); }
            let lv = rd_u64(e + ENT_LEVEL).unwrap_or(0);
            let mut out = format!("lv={} e.tag={:#x}/{:#x} o.tag={:#x}/{:#x} o.kind={} o.6b9={} o.6a0={:#x} e.6b9={} e.6a0={:#x}",
                lv, rd_u64(e).unwrap_or(0), rd_u64(e + 8).unwrap_or(0), rd_u64(o).unwrap_or(0), rd_u64(o + 8).unwrap_or(0), rd_u32(o + ENT_KIND),
                rd_u8(o + ENT_FLAG_6B9), rd_u64(o + ENT_STATE_6A0).unwrap_or(0), rd_u8(e + ENT_FLAG_6B9), rd_u64(e + ENT_STATE_6A0).unwrap_or(0));
            for (i, off) in [SLOT0, SLOT1, SLOT2, SLOT3].iter().enumerate() {
                if (i == 2 && lv < 3) || (i == 3 && lv < 5) { out += &format!(" s{}=zero", i); continue; }
                let sl = e + off; let id = rd_i32(sl + 0x30).unwrap_or(-9); let kind = rd_u32(sl + 0x28);
                let pred = kind_pred(kind, e, o); let reach = slot_reach(sl, e, o, lv);
                let vt = rd_u64(sl + 8).unwrap_or(0); let i_e8 = super::dyn_eff::impl_rva(vt as usize, 0xe8).unwrap_or(0);
                out += &format!(" s{}=[id={} k={} pred={:?} reach={:?} e8={:#x}]", i, id, kind, pred, reach, i_e8);
            }
            let n = rd_u64(e + ENT_EFFS_LEN).unwrap_or(0).min(CAP_ITER) as usize; let p = rd_u64(e + ENT_EFFS_PTR).unwrap_or(0) as usize;
            if n > 0 && ptr_ok(p) { out += " effs48=["; for i in 0..n { let v = rd_u64(p + i * 16 + 8).unwrap_or(0) as usize; out += &format!("{:#x} ", super::dyn_eff::impl_rva(v, 0x48).unwrap_or(0)); } out += "]"; }
            out
        }
        "as_e23170" => {
            let sa = p2; if !ptr_ok(sa) { return "bad sa".into(); }
            let tag = rd_u8(sa + SA_TAG); let c = if tag > 2 { tag - 3 } else { 7 };
            format!("tag={} c={} sa+8={:#x} sa+0x10={:#x} sa+0x18={:#x}", tag, c, rd_u64(sa + 8).unwrap_or(0), rd_u64(sa + 0x10).unwrap_or(0), rd_u64(sa + 0x18).unwrap_or(0))
        }
        _ => String::new(),
    }
}

// ─────────────────────────────────────────────────────────────────────────────────────────────
// ④ `dest` = 0xe23170(small_action.rs): SmallAction → 목표 좌표 Option<(x,y)>(sret 0x18B: +0 tag, +8 x, +0x10 y).
//    self = X 로스터[side][role] (X+0x1e0+side*0x28+role*8; 0 이면 tag 0). 태그(+0xb1) c = tag>2 ? tag−3 : 7 →
//    좌표 원천: c∈{0,6,7,8} sa+8/+0x10 · c=1 sa+0x50/+0x58 · c∈{2,3,10} sa+0x10/+0x18 · c=9 sa+0x18/+0x20 · c=16 self 위치 · c∈{4,5} tag 0
//    · c=11 (0xcaff00) · c∈{12..15} 스킬 타깃(dyn vt+0x68/+0x60 술어 + vt+0x1f0) → 아직 미포팅(NA).
//    그 다음 `move_step`(0x129d800) 로 self 위치에서 (tx,ty) 쪽으로 tps×speed 만큼 격자 유도 이동한 점이 결과.
//    정본 = REPORT RE\2026-09-06_small_action-목적지투영-0x129d800-…-0.5.8.md (ghidra-re, 부호·표·즉치 전량).
const DIR8: [(i32, i32); 8] = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];   // DAT_1433f7530 실덤프
const CELL: u64 = 32000; const HALF: u64 = 16000; const MAXC: u64 = 959_999;
const CFG_XMAX: usize = 0x12b8; const CFG_YMAX: usize = 0x12c0; const CFG_TPS: usize = 0x12f8;
const CTX2_CFG: usize = 0x8; const CTX2_PT: usize = 0x18; const CTX2_WMAP: usize = 0x20;
const PT_DATA: usize = 0x20; const PT_BLEN: usize = 0x28; const PT_NLEN: usize = 0x30;
const WMAP_GRID: usize = 0x78; const WMAP_ROW: usize = 0xf0;
const ENT_SPEED_640: usize = 0x640;
/// DAT_143436870 (u16×256) = ceil(sqrt((i+1)*256)) = isqrt(i*256+255)+1 — RE 실덤프 전량 일치
static SQRT_TAB: [u16; 256] = { let mut t = [0u16; 256]; let mut i = 0; while i < 256 { let v = (i as u64) * 256 + 255; let mut r = 0u64; while (r + 1) * (r + 1) <= v { r += 1; } t[i] = (r + 1) as u16; i += 1; } t };
/// 게임 인라인 isqrt(표 + Newton) — 0x129d800 은 임계 없이 이 경로만 쓴다
pub fn isqrt_fast(n: u64) -> u64 {
    if n == 0 { return 0; }
    let b = 63 - n.leading_zeros();
    let sh = if n < 0x10000 { 0 } else { b.wrapping_sub(14) & !1 };
    let m = n >> sh;
    let x0 = (SQRT_TAB[((m >> 8) & 0xff) as usize] as u64) << (sh >> 1);
    let mut r = x0 + 1;
    let mut yv = (n / r + r) >> 1;
    if yv <= x0 { loop { r = yv; if r == 0 { return 0; } yv = (n / r + r) >> 1; if yv >= r { break; } } }
    r
}
#[inline] unsafe fn wcell(wmap: usize, j: i64, i: i64) -> Option<u64> { rd_u64(wmap + WMAP_GRID + (j as usize) * WMAP_ROW + (i as usize) * 8) }
/// 0x18096a0: 목표점 보정(셀값 1 = 밀어내기 / ≠0 차단 → 최근접 통행셀) + signed 클램프
pub unsafe fn adjust_target(wmap: usize, cfg: usize, tx0: i64, ty0: i64) -> Option<(i64, i64)> {
    let (mut tx, mut ty) = (tx0, ty0);
    let cx = tx0 / 32000; let cy = ty0 / 32000;
    if (cx as u64) < 30 && (cy as u64) < 30 && wcell(wmap, cy, cx)? == 1 {
        let left = cx * 32000; let top = cy * 32000;
        let (mut dx, mut dy) = (0i64, 0i64);
        let mut has_ur = false;
        if cy >= 1 && wcell(wmap, cy - 1, cx)? == 0 { dy = -((top - 50 - ty).abs()); has_ur = true; }
        if cx <= 28 && wcell(wmap, cy, cx + 1)? == 0 {
            let rdx = (left + 32050 - tx).abs();
            if !has_ur { dx = rdx; dy = 0; has_ur = true; }
            else if dy.abs() <= rdx { dx = 0; } else { dy = 0; dx = rdx; }
        } else { dx = 0; }
        let mut has = has_ur;
        if cy <= 28 && wcell(wmap, cy + 1, cx)? == 0 {
            let ddy = (top + 32050 - ty).abs();
            if !has_ur { dx = 0; dy = ddy; } else if dy.abs() + dx > ddy { dx = 0; dy = ddy; }
            has = true;
        }
        if cx >= 1 && wcell(wmap, cy, cx - 1)? == 0 {
            let ldx = -((left - 50 - tx).abs());
            if !has { dx = ldx; dy = 0; } else if dy.abs() + dx > -ldx { dx = ldx; dy = 0; }
            has = true;
        }
        if has { tx += dx; ty += dy; }
    }
    let xmax = rd_i64(cfg + CFG_XMAX)?; let ymax = rd_i64(cfg + CFG_YMAX)?;
    if tx <= 0 { tx = 0; } if xmax < tx { tx = xmax; }
    ty = if ty > 0 { ty } else { 0 }; if ymax < ty { ty = ymax; }
    let need_search = (tx | ty) < 0 || (tx as u64) > MAXC || (ty as u64) > MAXC || wcell(wmap, ((ty as u64) / 32000) as i64, ((tx as u64) / 32000) as i64)? != 0;
    if need_search {
        let mut found = false; let (mut bx, mut by, mut bd) = (0i64, 0i64, 0i64);
        for j in 0..30i64 {
            let py = ty.clamp(j * 32000, j * 32000 + 31999);
            for i in 0..30i64 {
                if wcell(wmap, j, i)? != 0 { continue; }
                let pxv = tx.clamp(i * 32000, i * 32000 + 31999);
                let d = (pxv - tx) * (pxv - tx) + (py - ty) * (py - ty);
                if !found || d < bd { bx = pxv; by = py; bd = d; found = true; }
            }
        }
        if found {
            tx = bx; if tx <= 0 { tx = 0; } if xmax < tx { tx = xmax; }
            ty = if by > 0 { by } else { 0 }; if ymax < ty { ty = ymax; }
        }
    }
    Some((tx, ty))
}
fn partial(x: u64, y: u64, dx: i64, dy: i64, budget: u64, dist: u64, xmax: i64, ymax: i64) -> (u64, u64) {
    let sx = dx.wrapping_mul(budget as i64).wrapping_div(dist as i64);
    let sy = dy.wrapping_mul(budget as i64).wrapping_div(dist as i64);
    let mut rx = (x as i64).wrapping_add(sx); let mut ry = (y as i64).wrapping_add(sy);
    if rx <= 0 { rx = 0; } if xmax < rx { rx = xmax; }
    if ry <= 0 { ry = 0; } if ymax < ry { ry = ymax; }
    (rx as u64, ry as u64)
}
/// 0x129d800: 격자 유도 이동 스텝. 반환 = 결과 (x,y).
pub unsafe fn move_step(cfg: usize, pt: usize, wmap: usize, mut x: u64, mut y: u64, mut budget: u64, mut tx: u64, mut ty: u64) -> Option<(u64, u64)> {
    let data = rd_u64(pt + PT_DATA)? as usize; let blen = rd_u64(pt + PT_BLEN)?; let nlen = rd_u64(pt + PT_NLEN)?;
    let xmax = rd_i64(cfg + CFG_XMAX)?; let ymax = rd_i64(cfg + CFG_YMAX)?;
    let mut guard = 0u32;
    loop {
        guard += 1; if guard > 4096 { return None; }
        let cx = x / CELL; let cy = y / CELL;
        let (ntx, nty) = adjust_target(wmap, cfg, tx as i64, ty as i64)?; tx = ntx as u64; ty = nty as u64;
        if ty > MAXC || x > MAXC || y > MAXC || tx > MAXC { break; }
        let tcx = tx / CELL; let tcy = ty / CELL;
        let idx = cx * 27000 + cy * 900 + tcx * 30 + tcy;
        if idx >= nlen { break; }
        let bi = idx >> 1; if bi >= blen { break; }
        if !ptr_ok(data) { return None; }
        let dir = (rd_u8(data + bi as usize) >> (((tcy as u32) << 2) & 4)) & 0xf;
        if dir > 7 { break; }
        let ncx = (DIR8[dir as usize].0 as u32).wrapping_add(cx as u32); if ncx > 29 { break; }
        let ncy = (DIR8[dir as usize].1 as u32).wrapping_add(cy as u32); if ncy >= 30 { break; }
        if cx == ncx as u64 && cy == ncy as u64 { break; }
        let nx = ncx as u64 * CELL + HALF; let ny = ncy as u64 * CELL + HALF;
        let dx = nx.wrapping_sub(x) as i64; let dy = ny.wrapping_sub(y) as i64;
        let d2 = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) as u64;
        let dist = if d2 == 0 { 0 } else { isqrt_fast(d2) };
        if dist > budget { return Some(partial(x, y, dx, dy, budget, dist, xmax, ymax)); }
        if nx == tx && ny == ty { return Some((tx, ty)); }
        budget -= dist; x = nx; y = ny;
    }
    let dx = tx.wrapping_sub(x) as i64; let dy = ty.wrapping_sub(y) as i64;
    let d2 = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) as u64;
    if d2 == 0 { return Some((tx, ty)); }
    let dist = isqrt_fast(d2);
    if dist <= budget { return Some((tx, ty)); }
    Some(partial(x, y, dx, dy, budget, dist, xmax, ymax))
}
/// 0xe23170 재현: (p2 = SmallAction, p4 = sim, p5 = &(X, ctx2)) → [tag, x, y]
pub unsafe fn dest_from_args(p2: usize, p4: usize, p5: usize) -> Option<[u64; 3]> {
    if !ptr_ok(p2) || !ptr_ok(p4) || !ptr_ok(p5) { return None; }
    let side = rd_u64(p4 + P5_SIDE)?; if side > 1 { return None; }
    let role = rd_u32(p4 + P5_ROLE) as usize;
    let x = rd_u64(p5)? as usize; if !ptr_ok(x) { return None; }
    let me = rd_u64(x + X_ROSTER + (side as usize) * ROSTER_SIDE_STRIDE + role * 8)? as usize;
    if me == 0 { return Some([0, 0, 0]); }
    let tag = rd_u8(p2 + SA_TAG); let c = if tag > 2 { tag - 3 } else { 7 };
    let (tx, ty) = match c {
        0 | 6 | 7 | 8 => (rd_u64(p2 + 8)?, rd_u64(p2 + 0x10)?),
        1 => (rd_u64(p2 + 0x50)?, rd_u64(p2 + 0x58)?),
        2 | 3 | 10 => (rd_u64(p2 + 0x10)?, rd_u64(p2 + 0x18)?),
        9 => (rd_u64(p2 + 0x18)?, rd_u64(p2 + 0x20)?),
        16 => (rd_u64(me + ENT_X)?, rd_u64(me + ENT_Y)?),
        4 | 5 => return Some([0, 0, 0]),
        11 => match trace_point(p2, me, p5)? { Some((x, y)) => (x, y), None => (rd_u64(p2 + 0x68)?, rd_u64(p2 + 0x70)?) },
        12 | 13 | 14 | 15 => {
            // 스킬 슬롯 타깃: 슬롯 id != −1 && (vt68(payload) || vt60(payload)) && vt1f0(sa+8) → 대상 위치 그대로(투영 없음)
            let lv = rd_u64(me + ENT_LEVEL)?;
            let slot = match c { 12 => Some(me + SLOT0), 13 => Some(me + SLOT1), 14 => if lv > 2 { Some(me + SLOT2) } else { None }, _ => if lv > 4 { Some(me + SLOT3) } else { None } };
            let slot = match slot { Some(s) => s, None => return Some([0, 0, 0]) };
            if rd_i32(slot + 0x30)? == -1 { return Some([0, 0, 0]); }
            let (d, v) = (rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize);
            let ok = eff_bool(d, v, 0x68, 0)? || eff_bool(d, v, 0x60, 0)?;
            if !ok { return Some([0, 0, 0]); }
            let w = World { x, data: rd_u64(x)? as usize, vt: rd_u64(x + 8)? as usize }; if !ptr_ok(w.data) { return None; }
            return match w.entity(rd_u64(p2 + 8)?) { Some(e) => Some([1, rd_u64(e.0 + ENT_X)?, rd_u64(e.0 + ENT_Y)?]), None => Some([0, 0, 0]) };
        }
        _ => return None,
    };
    let ctx2 = rd_u64(p5 + 8)? as usize; if !ptr_ok(ctx2) { return None; }
    let cfg = rd_u64(ctx2 + CTX2_CFG)? as usize; let pt = rd_u64(ctx2 + CTX2_PT)? as usize; let wmap = rd_u64(ctx2 + CTX2_WMAP)? as usize;
    if !ptr_ok(cfg) || !ptr_ok(pt) || !ptr_ok(wmap) { return None; }
    let budget = rd_i64(cfg + CFG_TPS)?.wrapping_mul(rd_i64(me + ENT_SPEED_640)?) as u64;
    let (rx, ry) = move_step(cfg, pt, wmap, rd_u64(me + ENT_X)?, rd_u64(me + ENT_Y)?, budget, tx, ty)?;
    Some([1, rx, ry])
}
pub fn dest_eq(game: &[u64; 9], mine: &[u64; 9]) -> bool {
    if (game[0] & 0xff) != (mine[0] & 0xff) { return false; }
    if (game[0] & 0xff) == 1 { game[1] == mine[1] && game[2] == mine[2] } else { true }
}

/// 스킬 dyn Effect 의 bool 슬롯(+0x60 / +0x68) — Arc 페이로드 정렬(dn_reach::eff_payload 와 동일) 후 구현체 RVA 로 판정. 미재현 → unseen(slot) + None.
/// 단순 필드 게터 impl 을 기계어로 디코드해 값만 읽는다. dyn 구현체 수십 종이 전부 이 형태라
/// (실측 2026-09-07 01:20: sp.vt+0x80 상위 9종이 모두 `mov rax,[rcx+K]; ret`), RVA 표를 늘리는 대신 패턴으로 처리한다.
///   지원: `48 8b 81 d32` / `48 8b 41 d8`(mov rax,[rcx+d]) · `8b 81 d32` / `8b 41 d8`(mov eax) ·
///        `0f b6 81 d32` / `0f b6 41 d8`(movzx eax, byte) · `48 8b 01`(mov rax,[rcx]) · `31 c0`/`33 c0`(xor eax,eax → 0) · `b8 imm32`(mov eax,imm)
pub unsafe fn decode_getter(f: usize, obj: usize) -> Option<u64> {
    if !ptr_ok(f) { return None; }
    let b0 = rd_u8(f); let b1 = rd_u8(f + 1); let b2 = rd_u8(f + 2);
    // ★패턴 길이만큼 뒤에 ret(0xc3) 가 있어야 "그 게터가 전부"다. 안 그러면 더 긴 함수의 앞부분을
    //   잘라 읽는 것이라 값이 조용히 틀린다(2026-09-07: vt+0xa8 impl 0x1716a40 이 이 경로로 0 을 냈다).
    let ret_at = |n: usize| rd_u8(f + n) == 0xc3;
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x81 && ret_at(7) { return rd_u64((obj as isize + rd_i32(f + 3)? as isize) as usize); }
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x41 && ret_at(4) { return rd_u64((obj as isize + rd_u8(f + 3) as i8 as isize) as usize); }
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x01 && ret_at(3) { return rd_u64(obj); }
    if b0 == 0xb0 && ret_at(2) { return Some(b1 as u64); }                          // mov al, imm8; ret
    if b0 == 0x8b && b1 == 0x81 && ret_at(6) { return Some(rd_u32((obj as isize + rd_i32(f + 2)? as isize) as usize) as u64); }
    if b0 == 0x8b && b1 == 0x41 && ret_at(3) { return Some(rd_u32((obj as isize + rd_u8(f + 2) as i8 as isize) as usize) as u64); }
    if b0 == 0x0f && b1 == 0xb6 && b2 == 0x81 && ret_at(7) { return Some(rd_u8((obj as isize + rd_i32(f + 3)? as isize) as usize) as u64); }
    if b0 == 0x0f && b1 == 0xb6 && b2 == 0x41 && ret_at(4) { return Some(rd_u8((obj as isize + rd_u8(f + 3) as i8 as isize) as usize) as u64); }
    if (b0 == 0x31 || b0 == 0x33) && b1 == 0xc0 && ret_at(2) { return Some(0); }
    if b0 == 0xb8 && ret_at(5) { return Some(rd_u32(f + 1) as u64); }
    // cmp qword [rcx+d32], 0 ; setne/sete al ; ret   (0x146b0d0 = ability_pick vt+0x50, RE 2026-09-07)
    //   `.pdata` 미등재 leaf 라 Ghidra 가 함수로 잡지도 못한다 — 개별 RVA 등재 대신 패턴으로 흡수한다.
    if b0 == 0x48 && b1 == 0x83 && b2 == 0xb9 && rd_u8(f + 7) == 0x00
        && rd_u8(f + 8) == 0x0f && (rd_u8(f + 9) == 0x95 || rd_u8(f + 9) == 0x94)
        && rd_u8(f + 10) == 0xc0 && ret_at(11) {
        let v = rd_u64((obj as isize + rd_i32(f + 3)? as isize) as usize)?;
        return Some(((v != 0) == (rd_u8(f + 9) == 0x95)) as u64);
    }
    if b0 == 0x48 && b1 == 0x83 && b2 == 0x79 && rd_u8(f + 4) == 0x00
        && rd_u8(f + 5) == 0x0f && (rd_u8(f + 6) == 0x95 || rd_u8(f + 6) == 0x94)
        && rd_u8(f + 7) == 0xc0 && ret_at(8) {
        let v = rd_u64((obj as isize + rd_u8(f + 3) as i8 as isize) as usize)?;
        return Some(((v != 0) == (rd_u8(f + 6) == 0x95)) as u64);
    }
    None
}
pub unsafe fn eff_bool(data: usize, vt: usize, slot: usize, depth: u32) -> Option<bool> {
    if depth > 40 { super::dyn_eff::unseen(0x4000 | slot as u32, depth as usize); return None; }
    if !ptr_ok(vt) {
        super::dyn_eff::unseen(0x2000 | slot as u32, vt & 0xffff_ffff);
        super::dyn_eff::unseen(0x2100 | slot as u32, vt >> 32);
        return None;
    }
    let r = match super::dyn_eff::impl_rva(vt, slot) {
        Some(r) => r,
        None => { super::dyn_eff::unseen(0x3000 | slot as u32, 0); return None; }
    };
    let align = rd_u64(vt + 0x10)?; let p = data.wrapping_add(((align.wrapping_sub(1)) & !0xfu64) as usize).wrapping_add(0x10);
    let any_child = |s: usize| -> Option<bool> {
        let n = rd_u64(p + 0x10)?; if n == 0 { return Some(false); }
        let arr = rd_u64(p + 8)? as usize; if !ptr_ok(arr) { return None; }
        for i in 0..n.min(CAP_ITER) as usize { let (cd, cv) = (rd_u64(arr + i * 16)? as usize, rd_u64(arr + i * 16 + 8)? as usize); if eff_bool(cd, cv, s, depth + 1)? { return Some(true); } }
        Some(false)
    };
    match (slot, r) {
        (_, EFF_E8_ZERO) => Some(false),
        (_, EFF_TRUE) => Some(true),
        (0x60, EFF60_PAIR_NONZERO) => Some(rd_u64(p + 0x18)? != 0 && rd_u64(p + 0x10)? != 0),
        (0x60, EFF60_ANY_CHILD) => any_child(0x60),
        (0x68, EFF68_NONZERO18) => Some(rd_u64(p + 0x18)? != 0),
        (0x68, EFF68_ANY_CHILD) => any_child(0x68),
        (0x68, 0x13bede0) => { let n = rd_u64(p + 0x58)?; if n == 0 { return Some(false); } let arr = rd_u64(p + 0x50)? as usize; if !ptr_ok(arr) { return None; }
            for i in 0..n.min(CAP_ITER) as usize { let (cd, cv) = (rd_u64(arr + i * 0x18)? as usize, rd_u64(arr + i * 0x18 + 8)? as usize); if eff_bool(cd, cv, 0x68, depth + 1)? { return Some(true); } } Some(false) }
        (0x68, 0x122e650) => {
            // any(list1 @ p+0x50, len p+0x58, stride 0x18) || any(list2 @ p+0x68, len p+0x70, stride 0x10)
            let n1 = rd_u64(p + 0x58)?;
            if n1 != 0 { let a1 = rd_u64(p + 0x50)? as usize; if !ptr_ok(a1) { return None; }
                for i in 0..n1.min(CAP_ITER) as usize { let e = a1 + i * 0x18;
                    if eff_bool(rd_u64(e)? as usize, rd_u64(e + 8)? as usize, 0x68, depth + 1)? { return Some(true); } } }
            let n2 = rd_u64(p + 0x70)?;
            if n2 != 0 { let a2 = rd_u64(p + 0x68)? as usize; if !ptr_ok(a2) { return None; }
                for i in 0..n2.min(CAP_ITER) as usize { let e = a2 + i * 0x10;
                    if eff_bool(rd_u64(e)? as usize, rd_u64(e + 8)? as usize, 0x68, depth + 1)? { return Some(true); } } }
            Some(false)
        }
        _ => {
            // ★위임형 일반 디코더: `mov rax,[rcx+A]; mov rdx,[rcx+B]; …; jmp qword ptr [rdx+SLOT]`
            //   = 자식 fat-ptr (p+A, p+B) 로 같은 판정을 그대로 넘긴다(0x1606650 · 0x164ec10 등).
            if let Some((da, db, sl)) = delegate_pair(rd_u64(vt + slot)? as usize) {
                let (cd, cv) = (rd_u64(p + da)? as usize, rd_u64(p + db)? as usize);
                if ptr_ok(cd) && ptr_ok(cv) { return eff_bool(cd, cv, sl, depth + 1); }
            }
            super::dyn_eff::unseen(slot as u32, r); None
        }
    }
}
/// 위임형 impl 판별 → (자식 data 오프셋, 자식 vt 오프셋, 넘길 슬롯)
pub unsafe fn delegate_pair(f: usize) -> Option<(usize, usize, usize)> {
    if !ptr_ok(f) { return None; }
    let (a, mut i) = if rd_u8(f) == 0x48 && rd_u8(f + 1) == 0x8b && rd_u8(f + 2) == 0x01 { (0usize, 3usize) }
        else if rd_u8(f) == 0x48 && rd_u8(f + 1) == 0x8b && rd_u8(f + 2) == 0x41 { (rd_u8(f + 3) as usize, 4) }
        else if rd_u8(f) == 0x48 && rd_u8(f + 1) == 0x8b && rd_u8(f + 2) == 0x81 { (rd_i32(f + 3)? as usize, 7) }
        else { return None };
    let b = if rd_u8(f + i) == 0x48 && rd_u8(f + i + 1) == 0x8b && rd_u8(f + i + 2) == 0x11 { i += 3; 0usize }
        else if rd_u8(f + i) == 0x48 && rd_u8(f + i + 1) == 0x8b && rd_u8(f + i + 2) == 0x51 { let v = rd_u8(f + i + 3) as usize; i += 4; v }
        else if rd_u8(f + i) == 0x48 && rd_u8(f + i + 1) == 0x8b && rd_u8(f + i + 2) == 0x91 { let v = rd_i32(f + i + 3)? as usize; i += 7; v }
        else { return None };
    for k in 0..0x18usize {
        let a2 = f + i + k;
        if rd_u8(a2) == 0xff && rd_u8(a2 + 1) == 0x62 { return Some((a, b, rd_u8(a2 + 2) as usize)); }
        if rd_u8(a2) == 0xff && rd_u8(a2 + 1) == 0xa2 { return Some((a, b, rd_i32(a2 + 2)? as usize)); }
    }
    None
}
/// 0xcaff00 (small_action/trace.rs): 추적 대상의 접근점. 반환 Some(None) = tag 0(대상 없음/슬롯0 없음).
///   대상 = vt1f0(sa+0x60) · reach = sa.0(u32)==1 ? sa+8 : min(슬롯0 [필수], 슬롯1·2 [sa+0x91==0 && 술어]) 의 사거리(rng(self)+rng(tgt)+0x438+base+(lv−1)perlv+e8)
///   self.0==0 && 대상이 self 측에 비가시 → 대상 위치 그대로. 아니면 wmap 2번째 표(+0x1c98) 셀 0 이면 "대상에서 self 쪽으로 max(0, reach−sa.78) 물린 점" 을 adjust_target, 셀≠0 이면 대상 위치.
unsafe fn trace_point(sa: usize, me: usize, p5: usize) -> Option<Option<(u64, u64)>> {
    let x = rd_u64(p5)? as usize; let w = World { x, data: rd_u64(x)? as usize, vt: rd_u64(x + 8)? as usize };
    if !ptr_ok(w.data) { return None; }
    let tgt = match w.entity(rd_u64(sa + 0x60)?) { Some(e) => e.0, None => return Some(None) };
    let lv = rd_u64(me + ENT_LEVEL)?;
    let reach = if rd_u32(sa) == 1 { rd_u64(sa + 8)? } else {
        if rd_i32(me + SLOT0 + 0x30)? == -1 { return Some(None); }
        let base16 = rng_e(me)?.wrapping_add(rng_e(tgt)?).wrapping_add(rd_u64(me + ENT_F438)?);
        let sl = |slot: usize| -> Option<u64> {
            let bonus = super::dn_reach::eff_e8(rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize, me, tgt, 0)?;
            Some(bonus.wrapping_add(rd_u64(slot + 0x18)?.wrapping_mul(lv.wrapping_sub(1))).wrapping_add(rd_u64(slot + 0x10)?).wrapping_add(base16))
        };
        let mut r = sl(me + SLOT0)?;
        if rd_u8(sa + 0x91) == 0 {
            if rd_i32(me + SLOT1 + 0x30)? != -1 && kind_pred(rd_u32(me + SLOT1 + 0x28), me, tgt)? { r = r.min(sl(me + SLOT1)?); }
            if lv > 2 && rd_i32(me + SLOT2 + 0x30)? != -1 && kind_pred(rd_u32(me + SLOT2 + 0x28), me, tgt)? { r = r.min(sl(me + SLOT2)?); }
        }
        r
    };
    let (tx, ty) = (rd_u64(tgt + ENT_X)?, rd_u64(tgt + ENT_Y)?);
    if rd_u8(me) == 0 {
        let side = rd_u64(me + 8)?; if side > 1 { return None; }
        if rd_u64(tgt + ENT_VIS_BASE + (side as usize) * ENT_VIS_STRIDE)? != 0 { return Some(Some((tx, ty))); }
    }
    let ctx2 = rd_u64(p5 + 8)? as usize; if !ptr_ok(ctx2) { return None; }
    let wmap = rd_u64(ctx2 + CTX2_WMAP)? as usize; let cfg = rd_u64(ctx2 + CTX2_CFG)? as usize;
    if !ptr_ok(wmap) || !ptr_ok(cfg) { return None; }
    let cx = (tx / 32000).min(29) as usize; let cy = (ty / 32000).min(29) as usize;
    if rd_u64(wmap + WMAP_GRID2 + cy * WMAP_ROW + cx * 8)? != 0 { return Some(Some((tx, ty))); }
    let dx = (rd_u64(me + ENT_X)? as i64).wrapping_sub(tx as i64); let dy = (rd_u64(me + ENT_Y)? as i64).wrapping_sub(ty as i64);
    let d2 = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
    if d2 < 0 { return None; }   // 게임: isqrt 음수 → panic
    let mut d = isqrt_fast(d2 as u64); if d == 0 { d = 1; }
    let back_lim = rd_u64(sa + 0x78)?; let back = if reach >= back_lim { reach - back_lim } else { 0 } as i64;
    let px = dx.wrapping_mul(back).wrapping_div(d as i64).wrapping_add(tx as i64);
    let py = back.wrapping_mul(dy).wrapping_div(d as i64).wrapping_add(ty as i64);
    let (ax, ay) = adjust_target(wmap, cfg, px, py)?;
    Some(Some((ax as u64, ay as u64)))
}

