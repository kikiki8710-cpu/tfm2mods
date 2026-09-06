//! action_score — 기저 스코어러 `0xd57540`(action_score.rs:617~932, 1,988명령) **1단계 포팅**(2026-09-06 17:30).
//!   정본 = REPORT RE\2026-09-06_action_score-기저스코어러-0xd57540-구조RE-순수재현용-0.5.8.md (§2 의사코드 순서 그대로).
//!   호출 = 각 SubPlan 스코어러가 tail-call. 반환 i64. 검증 = `judge_scorer_cmp!`(원본 먼저 → 재현 → i64 대조).
//! 1단계 = 미해독·메모 콜리는 **캡처**로 값을 받는다(검증 전용): 스냅샷 0xc88300(out 0x48B) · 틱 캐시 비트 0xc87850 · 퍼센타일 C 0xc87fe0 ·
//!   목적지 0xe23170(out) · position_eval 0xd84db0(out) · fight_check 0xeb82d0 · 최대사거리 0xe0e890 · 전투점수 0xd5bbf0(cat6~9).
//!   순수 포팅: 위협합 0xd83230 · 사거리식 0xe27e50/0x1285320/0xd47620 · sim표 0xd31bb0 · 수적우세 카운트 · dyn Champion 게터 디코드.
//! 계약(인자): p1 mode(정수) · p2 불투명(전투점수 전달) · p3 sim(+0x930 side·+0x9c0 role·+0x928 키·+0x510/0x518 dyn Champion) · p4 Holder ·
//!   p5 bb(팀 블랙보드) · p6 SmallAction(+0xb1 tag) · p7 부가 ctx. JT1(태그→cat·payload) / JT2(cat→분기).
use crate::*;
use super::super::world::*;
use super::super::layout::*;
use super::super::{ScorerArgs, tr, live_imm8, live_imm32, live_imm64, cap_est_dmg, cap_dn_cache, cap_util_c87fe0, cap_combat_score, cap_as_c88300_out, cap_as_e23170, cap_as_d84db0, cap_as_eb82d0, cap_as_e0e890, cap_as_d83230};
use super::passive_jungle::estimate_damage;
use super::dn_reach::eff_e8;

const NEG: i64 = -99999;
const NEG2: i64 = -9_999_999;

#[inline] fn sqd(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = if ax < bx { bx - ax } else { ax - bx }; let dy = if ay < by { by - ay } else { ay - by };
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}
#[inline] unsafe fn xy(e: usize) -> Option<(u64, u64)> { Some((rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?)) }
#[inline] unsafe fn d2(a: usize, b: usize) -> Option<u64> { let (ax, ay) = xy(a)?; let (bx, by) = xy(b)?; Some(sqd(ax, ay, bx, by)) }
/// rng(e) = e.0x470==0 ? e.0x680 : (e.0x470+100)*e.0x680/100
#[inline] unsafe fn rng(e: usize) -> Option<u64> {
    let p = rd_i32(e + ENT_F470)? as i64; let r = rd_u64(e + ENT_F680)?;
    Some(if p == 0 { r } else { ((p + 100) as u64).wrapping_mul(r) / 100 })
}
/// 사거리식(0xe27e50 계열): slot.base + (slot.flag==0 ? rng(e) : 0) + e.0x438 + (lv−1)*slot.perlv + rng(other) + vt_e8(slot, e, other)
unsafe fn reach_val(slot: usize, e: usize, other: usize) -> Option<u64> {
    let base = rd_u64(slot + 0x10)?; let perlv = rd_u64(slot + 0x18)?;
    let self_r = if rd_i32(slot + 0x30)? == 0 { rng(e)? } else { 0 };
    let bonus = eff_e8(rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize, e, other, 0)?;
    Some(base.wrapping_add(self_r).wrapping_add(rd_u64(e + ENT_F438)?).wrapping_add(rd_u64(e + ENT_LEVEL)?.wrapping_sub(1).wrapping_mul(perlv)).wrapping_add(rng(other)?).wrapping_add(bonus))
}
/// 0x1285320/0xd47620: dist²(e, other) <= reach(e→other)²
unsafe fn in_reach(slot: usize, e: usize, other: usize) -> Option<bool> { let r = reach_val(slot, e, other)?; Some(d2(e, other)? <= r.wrapping_mul(r)) }
/// 0xd83230: Record 위협 리스트(+0x18 ptr/+0x30 len, stride 0x18 {src, delay, amount}) 를 delay<=thr 로 거르고 src 별 max 를 합산
pub unsafe fn threat_sum(rec: usize, thr: u64) -> Option<i64> {
    let n = rd_u64(rec + AS_REC_THR_LEN)?; if n == 0 { return Some(0); }
    let p = rd_u64(rec + AS_REC_THR_PTR)? as usize; if !ptr_ok(p) { return None; }
    let mut srcs: [(u64, i64); 64] = [(0, 0); 64]; let mut m = 0usize;
    for i in 0..n.min(256) as usize {
        let e = p + i * 0x18; if rd_u64(e + 8)? > thr { continue; }
        let (src, amt) = (rd_u64(e)?, rd_i64(e + 0x10)?);
        if let Some(k) = srcs[..m].iter().position(|s| s.0 == src) { if amt > srcs[k].1 { srcs[k].1 = amt; } }
        else if m < 64 { srcs[m] = (src, amt); m += 1; }
    }
    Some(srcs[..m].iter().map(|s| s.1).fold(0i64, |a, b| a.wrapping_add(b)))
}
/// 0xd31bb0: X 로스터(side 별 5슬롯) 에서 핸들 일치 슬롯 i → X+0x230+side*0x28+i*8 (sim 포인터)
pub unsafe fn sim_of_handle(x: usize, h: u64) -> Option<usize> {
    for side in 0..2usize { for i in 0..5usize {
        let e = rd_u64(x + X_ROSTER + side * 0x28 + i * 8)? as usize;
        if e != 0 && rd_u64(e + ENT_HANDLE)? == h { return Some(rd_u64(x + X_SIM_TABLE + side * 0x28 + i * 8)? as usize); } } }
    Some(0)
}
/// dyn Champion 게터 디코드: vt+0x20 → `8b 81 disp32 c3`(mov eax,[rcx+d]) 로 u32 kind ; vt+0x28 → `48 8d 41 d8` / `48 8d 81 d32` / `48 89 c8` 로 &Vec<i32>
pub unsafe fn champ_kind(data: usize, vt: usize) -> Option<u32> {
    let f = rd_u64(vt + 0x20)? as usize; if !ptr_ok(f) { return None; }
    let b0 = rd_u8(f); let b1 = rd_u8(f + 1);
    if b0 == 0x8b && b1 == 0x81 { let d = rd_i32(f + 2)? as isize; return Some(rd_u32((data as isize + d) as usize)); }
    if b0 == 0x8b && b1 == 0x41 { let d = rd_u8(f + 2) as i8 as isize; return Some(rd_u32((data as isize + d) as usize)); }
    None
}
pub unsafe fn champ_tags(data: usize, vt: usize) -> Option<(usize, u64)> {
    let f = rd_u64(vt + 0x28)? as usize; if !ptr_ok(f) { return None; }
    let (b0, b1, b2) = (rd_u8(f), rd_u8(f + 1), rd_u8(f + 2));
    let base = if b0 == 0x48 && b1 == 0x8d && b2 == 0x41 { (data as isize + rd_u8(f + 3) as i8 as isize) as usize }
               else if b0 == 0x48 && b1 == 0x8d && b2 == 0x81 { (data as isize + rd_i32(f + 3)? as isize) as usize }
               else if b0 == 0x48 && b1 == 0x89 && b2 == 0xc8 { data } else { return None };
    Some((rd_u64(base + 8)? as usize, rd_u64(base + 0x10)?))
}
/// dyn Champion vt+0x30 (out, obj) 복사 구현에서 원본 블록 오프셋을 디코드: `0f 10 81 d32` / `0f 10 41 d8` (movups xmm0,[rcx+d]) / `f3 0f 6f 81 d32` (movdqu). 반환 = [obj+d+0x10]
pub unsafe fn champ_vt30_w2(obj: usize, vt: usize) -> Option<u64> {
    let f = rd_u64(vt + 0x30)? as usize; if !ptr_ok(f) { return None; }
    static DBG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    if DBG.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 6 {
        let bytes = (0..32).map(|k| format!("{:02x}", rd_u8(f + k))).collect::<Vec<_>>().join(" ");
        let line = format!("vt30 vt={:#x} f={:#x} [{}]
", vt.wrapping_sub(crate::exe_base()), f.wrapping_sub(crate::exe_base()), bytes);
        if let Some(pp) = super::super::pth("judge_as_dbg.txt") { let _ = std::fs::OpenOptions::new().create(true).append(true).open(pp).and_then(|mut fh| { use std::io::Write; fh.write_all(line.as_bytes()) }); }
    }
    for k in 0..40usize {
        let (b0, b1, b2) = (rd_u8(f + k), rd_u8(f + k + 1), rd_u8(f + k + 2));
        if b0 == 0x0f && b1 == 0x10 && (b2 == 0x81 || b2 == 0x82) { return rd_u64((obj as isize + rd_i32(f + k + 3)? as isize) as usize + 0x10); }   // movups xmm0,[rcx/rdx+d32]
        if b0 == 0x0f && b1 == 0x10 && (b2 == 0x41 || b2 == 0x42) { return rd_u64((obj as isize + rd_u8(f + k + 3) as i8 as isize) as usize + 0x10); }
        if b0 == 0xf3 && b1 == 0x0f && b2 == 0x6f { let m = rd_u8(f + k + 3); if m == 0x81 { return rd_u64((obj as isize + rd_i32(f + k + 4)? as isize) as usize + 0x10); } if m == 0x41 { return rd_u64((obj as isize + rd_u8(f + k + 4) as i8 as isize) as usize + 0x10); } }
        if b0 == 0xc3 { break; }
    }
    None
}
pub unsafe fn tags_has(ptr: usize, len: u64, v: i32) -> Option<bool> {
    if len == 0 { return Some(false); } if !ptr_ok(ptr) { return None; }
    for i in 0..len.min(64) as usize { if rd_i32(ptr + i * 4)? == v { return Some(true); } } Some(false)
}
#[inline] fn mult_of(diff: i64, t: &[i64; 5]) -> i64 { if diff > 1 { t[4] } else if diff < -1 { t[0] } else { t[(diff + 2) as usize] } }
/// 게임의 부호 나눗셈(매직 0xa3d70a3d70a3d70b + sar s): s=6 /100 · 7 /200 · 9 /800 — 사이트 패치로 s 가 바뀌면 그대로 따라간다.
#[inline] fn div_a3d7(x: i64, s: u32) -> i64 {
    let m: i64 = 0xa3d70a3d70a3d70bu64 as i64;
    let hi = ((x as i128 * m as i128) >> 64) as i64;
    let q = hi.wrapping_add(x) >> s;
    q.wrapping_add((q >> 63) & 1)
}
/// `sar s`(+보정) 부호 나눗셈: x / 2^s (0 방향 절사)
#[inline] fn div_sar(x: i64, s: u32) -> i64 { let bias = if x < 0 { (1i64 << s) - 1 } else { 0 }; (x.wrapping_add(bias)) >> s }

/// 진단: cat2 gain 의 estimate_damage 가 게임 캡처(judge_cap_est=1)와 다를 때 내부값 덤프(≤12줄)
unsafe fn est_dbg(slot: usize, att: usize, target: usize, mine: i64, game: u64) {
    static N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    if N.fetch_add(1, std::sync::atomic::Ordering::Relaxed) >= 12 { return; }
    let b = crate::exe_base();
    let (data, vt) = (rd_u64(slot).unwrap_or(0) as usize, rd_u64(slot + 8).unwrap_or(0) as usize);
    let me_ = super::dyn_eff::arc_payload(data, vt).unwrap_or(0);
    let w = |a: usize, n: usize| -> String { (0..n).map(|k| format!("{:#x}", rd_u64(a + k * 8).unwrap_or(0))).collect::<Vec<_>>().join(" ") };
    let children = { let n = rd_u64(me_ + 0x28).unwrap_or(0).min(8); let arr = rd_u64(me_ + 0x20).unwrap_or(0) as usize;
        (0..n as usize).map(|i| { let cv = rd_u64(arr + i * 0x18 + 8).unwrap_or(0) as usize; format!("[{}: vt={:#x} i28={:#x} i38={:#x} i30={:#x}]", i, cv.wrapping_sub(b), super::dyn_eff::impl_rva(cv, 0x28).unwrap_or(0), super::dyn_eff::impl_rva(cv, 0x38).unwrap_or(0), super::dyn_eff::impl_rva(cv, 0x30).unwrap_or(0)) }).collect::<Vec<_>>().join(" ") };
    let line = format!("mine={} game={} vt_rva={:#x} i28={:#x} i38={:#x} i30={:#x} atk={} | payload {} | children {} | att.stats {} | att.buffs+a8 {} | tgt kind={} maxhp={} hp={} defp={} defm={}
",
        mine, game, vt.wrapping_sub(b), super::dyn_eff::impl_rva(vt, 0x28).unwrap_or(0), super::dyn_eff::impl_rva(vt, 0x38).unwrap_or(0), super::dyn_eff::impl_rva(vt, 0x30).unwrap_or(0), rd_u32(slot + 0x2c),
        w(me_, 6), children, w(att + ENT_STATS, 8), w(att + ENT_BUFF_BLOCK + 0xa8, 10), rd_u32(target + ENT_KIND), rd_u64(target + ENT_MAXHP).unwrap_or(0), rd_u64(target + ENT_HP).unwrap_or(0), rd_u64(target + ENT_DEF_P).unwrap_or(0), rd_u64(target + ENT_DEF_M).unwrap_or(0));
    if let Some(pp) = super::super::pth("judge_as_est.txt") { let _ = std::fs::OpenOptions::new().create(true).append(true).open(pp).and_then(|mut f| { use std::io::Write; f.write_all(line.as_bytes()) }); }
}
struct Snap { e: Option<usize>, s: Option<usize>, e2: Option<usize>, near: Option<usize>, mask: u8, count: u8 }

pub unsafe fn base_score(a: &ScorerArgs) -> Option<i64> {
    let (mode, p2, sim, hld, bb, act) = (a.p1 as i64, a.p2, a.p3, a.p4, a.p5, a.p6);
    if !ptr_ok(sim) || !ptr_ok(hld) || !ptr_ok(bb) || !ptr_ok(act) { return None; }
    let side = rd_u64(sim + P5_SIDE)?; if side > 1 { return None; }                      // 게임: panic 617:15
    let role = rd_u32(sim + P5_ROLE);
    let h = Holder::new(hld)?; let w = h.world()?; let g = h.g()?;
    let me = w.roster(side, role)?; if me == 0 { return None; }                         // 게임: panic 617:93
    let tag = rd_u8(act + SA_TAG); let idx = if tag >= 3 { (tag - 3) as usize } else { 7 };
    if idx > 16 { return None; }                                                       // JT OOB
    // JT1: idx → (cat, payload off)
    let (cat, poff): (u8, usize) = match idx { 0 | 1 | 5 => (0, 0), 2 | 3 | 10 => (2, 8), 4 => (3, 0x10), 6 => (1, 8), 7 => (3, 0x30), 8 => (3, 8), 9 => (3, 0x18),
                                              11 => (4, 0x60), 12 => (6, 8), 13 => (7, 8), 14 => (8, 8), 15 => (9, 8), _ => (10, 0) };
    tr(0, 0x100 | tag as u64 | (cat as u64) << 8 | (mode.clamp(0, 15) as u64) << 12 | side << 16 | (role as u64) << 20);
    if cat > 9 { return Some(0); }
    // 스냅샷(0xc88300 out 0x48B, 캡처) · 틱 캐시 비트(0xc87850, 캡처)
    let sn = cap_as_c88300_out::last()?;
    let ent_of = |tg: u64, hh: u64| -> Option<usize> { if tg != 0 { w.entity(hh).map(|e| e.0) } else { None } };
    let snap = Snap { e: ent_of(sn.w[0], sn.w[1]), s: ent_of(sn.w[2], sn.w[3]), e2: ent_of(sn.w[4], sn.w[5]), near: ent_of(sn.w[6], sn.w[7]), mask: (sn.w[8] & 0xff) as u8, count: ((sn.w[8] >> 8) & 0xff) as u8 };
    let hp = rd_u64(me + ENT_HP)?; let maxhp = rd_u64(me + ENT_MAXHP)?;
    let cc = rd_u8(me + ENT_IMMOBILE);
    let bits = || -> Option<u64> { cap_dn_cache::take() };
    let r13: bool = if cc != 0 { true } else if mode < 2 { false } else {
        let b = bits()?;
        if b & 0x100 != 0 { true } else if hp.wrapping_mul(100) <= maxhp.max(1).wrapping_mul(35) { false } else { b & 0x10001 != 0 } };
    tr(1, 0x100 | (snap.e.is_some() as u64) | (snap.s.is_some() as u64) << 1 | (snap.e2.is_some() as u64) << 2 | (snap.near.is_some() as u64) << 3 | (r13 as u64) << 4 | (cc as u64) << 5 | (snap.mask as u64) << 8 | (snap.count as u64) << 16);
    let tick = rd_u64(w.data + W_TICK)?; let cfg = rd_u64(g.0 + G_CFG)? as usize; if !ptr_ok(cfg) { return None; }
    let tick_cap = rd_u64(cfg + CFG_TICK_CAP)?;
    let c_val = || -> Option<i64> { cap_util_c87fe0::last().map(|v| v as i64) };
    let rec_self = bb + AS_BB_REC;
    let hp_i = hp as i64;
    let thr_a = live_imm32(SITE_AS_THR_A, 9999) as u64; let thr_b = live_imm32(SITE_AS_THR_B, 9999) as u64; let thr_c = live_imm32(SITE_AS_THR_C, 9999) as u64;
    let m0 = [live_imm32(SITE_AS_M0_LO, 40) as i64, AS_MULT0[1], AS_MULT0[2], AS_MULT0[3], live_imm32(SITE_AS_M0_HI, 300) as i64];
    let m4 = [live_imm32(SITE_AS_M4_LO, 30) as i64, AS_MULT4[1], AS_MULT4[2], AS_MULT4[3], live_imm32(SITE_AS_M4_HI, 200) as i64];
    let shr4 = live_imm8(SITE_AS_SHR4, 2) as u32; let shr800 = live_imm8(SITE_AS_SHR800, 9) as u32; let minus2 = live_imm8(SITE_AS_MINUS2, 254) as i8 as i64;
    let near_d2 = live_imm64(SITE_AS_NEAR_D2, 100_000u64.pow(2) + 1); let al_d2 = live_imm64(SITE_AS_AL_D2, 100_000u64.pow(2) + 1);
    let bonus_v = live_imm32(SITE_AS_BONUS, 10) as i64; let g950 = live_imm32(SITE_AS_950, 950) as u64;
    let margin_a = live_imm32(SITE_AS_MARGIN_A, 30000) as u64; let margin_b = live_imm32(SITE_AS_MARGIN_B, 30000) as u64;
    // ★상한은 `cmp rax,A ; mov r,B ; cmovb r,rax` 두 조각 = (raw < A ? raw : B). 사이트 패치로 A≠B 가 될 수 있어 그대로 재현.
    let cap2 = |raw: i64, a: i64, b: i64| -> i64 { if raw < a { raw } else { b } };
    let (pc2a, pc2b) = (live_imm8(SITE_AS_PCT_C2A, 100) as i64, live_imm32(SITE_AS_PCT_C2B, 100) as i64);
    let shr200 = live_imm8(SITE_AS_IDX7, 7) as u32;
    let (pxa, pxb, pxc, pxd) = (live_imm8(SITE_AS_PCT_XA, 100) as i64, live_imm32(SITE_AS_PCT_XB, 100) as i64, live_imm8(SITE_AS_PCT_XC, 100) as i64, live_imm32(SITE_AS_PCT_XD, 100) as i64);
    match cat {
        1 | 3 | 5 => Some(0),
        0 => {
            if cc != 0 { tr(2, 0x100); return Some(NEG); }
            if mode > 1 {
                let b = bits()?;
                if b & 0x100 != 0 { tr(2, 0x101); return Some(NEG); }
                if hp.wrapping_mul(100) / maxhp.max(1) >= 36 && b & 0x10001 != 0 { tr(2, 0x102); return Some(NEG); }
            }
            let dest = cap_as_e23170::last()?;
            let (mut risk1, mut risk2) = (rd_i64(bb + AS_BB_998)?, rd_i64(bb + AS_BB_9B0)?);
            if dest.w[0] != 0 {
                let pe = cap_as_d84db0::last()?;
                if (pe.w[6] >> 8) & 0xff != 2 {
                    let hp1 = hp_i.max(1);
                    risk1 = (risk1 - (pe.w[0] as i64).max(0).wrapping_mul(hp1) / 100).max(0);
                    risk2 = (risk2 - (pe.w[1] as i64).max(0).wrapping_mul(hp1) / 100).max(0);
                }
            }
            let dsum = threat_sum(rec_self, thr_b)?;
            let vis_to_enemy = w.visible(1 - side, rd_u64(me + ENT_HANDLE)?)?;
            let in_e2 = match snap.e2 { Some(e2) => { if rd_i32(e2 + ENT_SLOT0_FLAG)? == -1 { false } else { in_reach(e2 + ENT_SLOT0, e2, me)? } } None => false };   // 슬롯0 없음(-1) = 사거리 없음(18:20 실측: NA 1.3% 였음)
            if !vis_to_enemy && !in_e2 { risk2 = 0; }
            let c = c_val()?;
            let bx = g.home_box(side)?; let (mx, my) = xy(me)?;
            if bx.contains(mx, my) { tr(2, 0x103); return Some(NEG); }
            let mut rbp = NEG;
            if rd_i64(bb + AS_BB_988)? < hp_i {
                if hp == 0 { return None; }
                let t1 = risk1.wrapping_mul(c) / hp_i;
                let t2 = c.wrapping_mul(risk2.wrapping_add(dsum)) / hp_i;
                let diff = (snap.mask.count_ones() as i64) - snap.count as i64;
                let mult = mult_of(diff, &m0);
                rbp = div_sar(t1, shr4) + div_a3d7(mult.wrapping_mul(t2), shr800) + minus2;
                tr(3, (risk1.clamp(-0x7fff, 0x7fff) as u64 & 0xffff) | (risk2.clamp(-0x7fff, 0x7fff) as u64 & 0xffff) << 16 | (dsum.clamp(-0x7fff, 0x7fff) as u64 & 0xffff) << 32 | (c.clamp(0, 0xffff) as u64) << 48);
            }
            // tail: cat0 +10 보너스
            let (cd, cv) = (rd_u64(sim + P5_CHAMP_DATA)? as usize, rd_u64(sim + P5_CHAMP_VT)? as usize);
            if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
            let align = rd_u64(cv + 0x10)?; let obj = cd.wrapping_add(((align.wrapping_sub(1)) & !0xf) as usize).wrapping_add(0x10);
            let kind_o = champ_kind(obj, cv); let tags_o = champ_tags(obj, cv);
            if kind_o.is_none() || tags_o.is_none() {
                static DBG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
                if DBG.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 8 {
                    let f20 = rd_u64(cv + 0x20).unwrap_or(0) as usize; let f28 = rd_u64(cv + 0x28).unwrap_or(0) as usize; let b = crate::exe_base();
                    let bytes = |f: usize| (0..12).map(|k| format!("{:02x}", rd_u8(f + k))).collect::<Vec<_>>().join(" ");
                    let line = format!("vt={:#x} f20={:#x} [{}] f28={:#x} [{}]
", cv.wrapping_sub(b), f20.wrapping_sub(b), bytes(f20), f28.wrapping_sub(b), bytes(f28));
                    if let Some(pp) = super::super::pth("judge_as_dbg.txt") { let _ = std::fs::OpenOptions::new().create(true).append(true).open(pp).and_then(|mut f| { use std::io::Write; f.write_all(line.as_bytes()) }); }
                }
            }
            let kind = kind_o?; let (tp, tl) = tags_o?;
            tr(4, 0x100 | kind as u64 | tl.min(0xff) << 8);
            if tags_has(tp, tl, 8)? { tr(2, 0x104); return Some(rbp); }
            let gate = match kind {
                2 => true,
                3 => { if tags_has(tp, tl, 2)? || tags_has(tp, tl, 3)? { true } else {
                    // vt30(out,obj): obj 안 0x48B 블록 복사 → out+0x10 ≥ 950 이면 보너스 없이 반환. 구현 기계어에서 첫 `movups xmm0,[rcx+disp]` 의 disp 를 디코드해 [obj+disp+0x10] 을 읽는다.
                    let v10 = champ_vt30_w2(obj, cv)?; tr(6, 0x100 | v10.min(0xffff)); if v10 >= g950 { tr(2, 0x107); return Some(rbp); } true } }
                _ => false,
            };
            if !gate { tr(2, 0x105); return Some(rbp); }
            let mut bonus = 0;
            { let s2c = if rd_u64(me + ENT_LEVEL)? >= 3 { rd_i32(me + ENT_SLOT2 + 0x30).unwrap_or(-9) } else { -1 };
              let nd = match snap.near { Some(nr) => d2(me, nr).unwrap_or(u64::MAX), None => u64::MAX };
              tr(5, 0x100 | (rd_i32(me + ENT_SLOT1_FLAG).unwrap_or(-9) != -1) as u64 | ((rd_u32(me + ENT_KIND) == 13) as u64) << 1 | (rd_i64(me + ENT_B8).unwrap_or(-1).clamp(-1, 0xff) as u64 & 0xff) << 8 | (rd_i64(me + ENT_C0).unwrap_or(-1).clamp(-1, 0xff) as u64 & 0xff) << 16 | ((s2c != -1) as u64) << 2 | (snap.near.is_some() as u64) << 3 | ((nd <= 100_000u64.pow(2)) as u64) << 4); }
            { let nd = match snap.near { Some(nr) => d2(me, nr).unwrap_or(u64::MAX), None => u64::MAX };
              let ed = match snap.e { Some(e) => d2(me, e).unwrap_or(u64::MAX), None => u64::MAX };
              tr(6, 0x100 | (super::obj_helpers::isqrt(nd).min(0xfffff)) << 8 | (super::obj_helpers::isqrt(ed).min(0xfffff)) << 28 | ((rd_i64(me + ENT_B8).unwrap_or(-2) as u64) & 0xff) << 48 | ((rd_i64(me + ENT_C0).unwrap_or(-2) as u64) & 0xff) << 56); }
            if rd_i32(me + ENT_SLOT1_FLAG)? != -1 && rd_u32(me + ENT_KIND) == 13 && rd_i64(me + ENT_B8)? >= 31 {
                let s2 = if rd_u64(me + ENT_LEVEL)? >= 3 { me + ENT_SLOT2 } else { crate::exe_base() + AS_ZERO_DESC };
                if rd_i32(s2 + 0x30)? != -1 {
                    if let Some(nr) = snap.near { if rd_i64(me + ENT_C0)? >= 31 && d2(me, nr)? < near_d2 { bonus = bonus_v; } }
                }
            }
            tr(2, 0x106 | (bonus as u64) << 8);
            Some(rbp + bonus)
        }
        2 => {
            let th = rd_u64(act + SA_PAYLOAD8)?;
            let t = match w.entity(th) { Some(e) => e.0, None => { tr(2, 0x200); return Some(0) } };     // 死: -99999 → 꼬리 재resolve 실패 → 0
            let c = c_val()?; if hp == 0 { return None; }
            let cost = rd_i64(bb + AS_BB_998)?.wrapping_mul(c) / hp_i;
            let mut gain: i64 = 0;
            if let Some(s) = snap.s {
                let same = rd_u64(t)? == rd_u64(me)? && rd_u64(t + 8)? == rd_u64(me + 8)?;
                if !same && rd_i32(s + ENT_SLOT0_FLAG)? != -1 {
                    if in_reach(s + ENT_SLOT0, s, t)? { if let Some(e) = snap.e { if tick < tick_cap {
                        let dmg = estimate_damage(s + ENT_SLOT0, s, e, false)? as i64; let ehp = rd_u64(e + ENT_HP)? as i64; if ehp == 0 { return None; }
                        let cap = cap_est_dmg::find(s + ENT_SLOT0, e);
                        if let Some(gd) = cap { if gd as i64 != dmg { est_dbg(s + ENT_SLOT0, s, e, dmg, gd); } }
                        tr(6, 0x100 | (dmg.clamp(0, 0xffff) as u64) << 16 | (ehp.clamp(0, 0xffff) as u64) << 32 | (cap.is_some() as u64) << 8 | (cap.map(|v| v as i64 == dmg).unwrap_or(false) as u64) << 9 | (rd_u64(e + ENT_MAXHP).unwrap_or(0).min(0xffff)) << 48);
                        tr(7, 0x100 | (pc2a as u64) << 8 | (pc2b as u64) << 16 | (in_reach(s + ENT_SLOT0, s, t).unwrap_or(false) as u64) | (rd_u32(s + ENT_KIND) as u64 & 0xff) << 32 | (rd_u32(e + ENT_KIND) as u64 & 0xff) << 40);
                        gain = cap2(dmg.wrapping_mul(100) / ehp, pc2a, pc2b); } } }
                }
            }
            let goal = if idx == 2 { div_a3d7(c.wrapping_mul(rd_i64(act + SA_GOAL20)?), shr200) } else if idx == 10 { rd_i64(act + SA_GOAL20)? } else { 0 };
            tr(3, (gain as u64 & 0xffff) | (cost.clamp(-0x7fff, 0x7fff) as u64 & 0xffff) << 16 | (goal.clamp(-0x7fff, 0x7fff) as u64 & 0xffff) << 32 | (c.clamp(0, 0xffff) as u64) << 48);
            if w.entity(th).is_none() { return Some(0); }
            tr(2, 0x201); Some(gain - cost + goal)
        }
        4 => {
            let th = rd_u64(act + SA_PAYLOAD60)?;
            let t = match w.entity(th) { Some(e) => e.0, None => { tr(2, 0x400); return Some(0) } };
            let flag = tag == 14 && rd_u8(act + SA_FLAG94) != 0;
            let same = rd_u64(t)? == rd_u64(me)? && rd_u64(t + 8)? == rd_u64(me + 8)?;
            let (mtag, _) = w.mode()?;
            if mode > 1 && rd_u32(t + ENT_KIND) == 13 && !same && mtag != 2 {
                if rd_u8(bb + AS_BB_1500) != 0 { tr(2, 0x401); return Some(NEG2); }
                if !(r13 || flag) {
                    let reach = if rd_i32(me + ENT_SLOT0_FLAG)? != -1 { reach_val(me + ENT_SLOT0, me, t)? } else { 0 };
                    let d = super::obj_helpers::isqrt(d2(me, t)?);
                    if d > reach {
                        // 인지된 적 챔프(150k·보임‖120틱) 목록이 비어 있지 않으면 fight_check(캡처) ≤ ticks → -9999999
                        let lanes = rd_u64(hld + HOLDER_LANES)? as usize; let other = 1 - side; let mut n = 0;
                        for i in 0..5 { let e = rd_u64(w.x + X_ROSTER + other as usize * 0x28 + i * 8)? as usize; if e == 0 { continue; }
                            if d2(me, e)? > 150_000u64.pow(2) { continue; }
                            let hh = rd_u64(e + ENT_HANDLE)?;
                            let seen = w.visible(side, hh)? || { let rec = w.roster_rec(hh)?; rec != 0 && rd_u64(lanes + other as usize * LANE_STRIDE + LANE_ROSTER + rd_u32(rec + REC_ROLE) as usize * 8)?.wrapping_add(120) >= tick };
                            if seen { n += 1; } }
                        if n > 0 {
                            let ticks = (d - reach) / rd_u64(me + ENT_SPEED)?.max(1);
                            let r = cap_as_eb82d0::last()? as i64;
                            tr(5, 0x100 | n as u64 | (ticks.min(0xffff)) << 8 | (r.clamp(-0x7fff, 0x7fff) as u64 & 0xffff) << 32);
                            if r <= ticks as i64 { tr(2, 0x402); return Some(NEG2); }
                        }
                    }
                }
            }
            let dest = cap_as_e23170::last()?;
            let (mut aa, mut bbv) = (rd_i64(bb + AS_BB_998)?, if flag { 0 } else { rd_i64(bb + AS_BB_9B0)? / 3 });
            if dest.w[0] != 0 {
                let pe = cap_as_d84db0::last()?;
                if (pe.w[6] >> 8) & 0xff != 2 {
                    let hp1 = hp_i.max(1);
                    aa = (pe.w[0] as i64).max(0).wrapping_mul(hp1) / 100;
                    bbv = if flag { 0 } else { ((pe.w[1] as i64).max(0).wrapping_mul(hp1) / 100) / 3 };
                }
            }
            let dsum = threat_sum(rec_self, thr_a)?; let c = c_val()?;
            { let cap = cap_as_d83230::find(rec_self, hld).map(|v| v as i64); tr(7, 0x100 | (dsum.clamp(-0x7fff, 0x7fff) as u64 & 0xffff) << 16 | (cap.unwrap_or(-1).clamp(-0x7fff, 0x7fff) as u64 & 0xffff) << 32 | (cap.is_some() as u64) << 8 | ((cap == Some(dsum)) as u64) << 9);
              tr(6, (aa.clamp(-0x7fff, 0x7fff) as u64 & 0xffff) | (bbv.clamp(-0x7fff, 0x7fff) as u64 & 0xffff) << 16 | (c.clamp(0, 0xffff) as u64) << 32 | (dest.w[0].min(0xf)) << 48 | ((cap_as_d84db0::last().map(|p| (p.w[6] >> 8) & 0xff).unwrap_or(0xff)) & 0xff) << 52); }
            let (cost1, cost2) = if r13 { (0, 0) } else { if hp == 0 { return None; } (aa.wrapping_mul(c) / hp_i, dsum.wrapping_add(bbv).wrapping_mul(c) / hp_i) };
            // 교환비 X
            let mut xv: i64 = 0;
            tr(11, 0x100);
            if let Some(s) = snap.s { if w.entity(th).is_some() && rd_i32(s + ENT_SLOT0_FLAG)? != -1 {
                let r = match reach_val(s + ENT_SLOT0, s, t) { Some(v) => v, None => { tr(11, 0x901); return None } }.saturating_sub(margin_a);
                if d2(t, s)? <= r.wrapping_mul(r) { if let Some(e) = snap.e { if tick < tick_cap {
                    let dmg = estimate_damage(s + ENT_SLOT0, s, e, false)? as i64; let ehp = rd_u64(e + ENT_HP)? as i64; if ehp == 0 { return None; }
                    xv = cap2(dmg.wrapping_mul(100) / ehp, pxc, pxd); } } }   // 0xd59509/0d 사이트(S→E 가산)
            } }
            if let Some(e2) = snap.e2 { if !flag && w.entity(th).is_some() && rd_i32(e2 + ENT_SLOT0_FLAG)? != -1 {
                let r = match reach_val(e2 + ENT_SLOT0, e2, t) { Some(v) => v, None => { tr(11, 0x902); return None } }.saturating_sub(margin_b);
                if d2(t, e2)? <= r.wrapping_mul(r) && tick < tick_cap {
                    let dmg = estimate_damage(e2 + ENT_SLOT0, e2, me, false)? as i64; if hp == 0 { return None; }
                    xv -= cap2(dmg.wrapping_mul(100) / hp_i, pxa, pxb); }   // 0xd594da/de 사이트(E2→self 감산)
            } }
            // record 가치 V
            let mut v: i64 = 0;
            let nrec = rd_u64(bb + AS_BB_RECB_LEN)?; let precs = rd_u64(bb + AS_BB_RECB_PTR)? as usize;
            let mut found = 0usize;
            let (mut m58, mut m130) = (0u64, 0u64);
            if nrec != 0 { if !ptr_ok(precs) { return None; }
                let th_t = rd_u64(t + ENT_HANDLE)?;
                for i in 0..nrec.min(64) as usize { let r = precs + i * AS_REC_STRIDE;
                    if rd_u64(r + AS_REC_AGENT_H)? == th_t { m58 += 1; if found == 0 { found = r; } }   // ★+0x58 (t11 실측: +0x130 은 오독)
                    if rd_u64(r + AS_REC_ENT_H)? == th_t { m130 += 1; } } }
            tr(11, 0x100 | m58 | m130 << 4 | nrec.min(0xff) << 8);
            if found != 0 {
                if hp == 0 { return None; }
                let ts = threat_sum(found, thr_c)?; let r80 = rd_i64(found + AS_REC_80)?;
                let capv = cap_as_d83230::find(found, hld).map(|x| x as i64);
                tr(9, 0x100 | (ts.clamp(-0x7fff, 0x7fff) as u64 & 0xffff) << 16 | (r80.clamp(-0x7fff, 0x7fff) as u64 & 0xffff) << 32 | (capv.unwrap_or(-1).clamp(-0x7fff, 0x7fff) as u64 & 0xffff) << 48 | (capv.is_some() as u64) << 8);
                v = ts.wrapping_add(r80).wrapping_mul(c) / hp_i;
                let ag = sim_of_handle(w.x, rd_u64(found + AS_REC_AGENT_H)?)?; if ag == 0 { return None; }
                let lanes = rd_u64(hld + HOLDER_LANES)? as usize; if !ptr_ok(lanes) { return None; }
                let rolebyte = rd_u8(lanes + (1 - side) as usize * LANE_STRIDE + rd_u32(ag + P5_ROLE) as usize * 32 + LANE_ROLE_BYTE);
                tr(11, 0x200 | m58 | m130 << 4 | nrec.min(0xff) << 8);
                let reach2 = match cap_as_e0e890::find(me, t) { Some(v) => v, None => { tr(11, 0x300); return None } };
                tr(11, 0x400 | reach2.min(0xffff) << 16);
                if rolebyte == 6 && rd_u64(t + ENT_SPEED)?.wrapping_add(100) > rd_u64(me + ENT_SPEED)? && d2(me, t)? > reach2.wrapping_mul(reach2) { tr(2, 0x403); return Some(NEG); }
            }
            // 수적 우세
            let lanes = rd_u64(hld + HOLDER_LANES)? as usize; if !ptr_ok(lanes) { return None; }
            let other = 1 - side; let (mut en, mut al) = (0i64, 0i64);
            for i in 0..5 { let e = rd_u64(w.x + X_ROSTER + other as usize * 0x28 + i * 8)? as usize; if e == 0 { continue; }
                if d2(me, e)? > 150_000u64.pow(2) { continue; }
                let hh = rd_u64(e + ENT_HANDLE)?;
                let seen = w.visible(side, hh)? || { let rec = w.roster_rec(hh)?; rec != 0 && rd_u64(lanes + other as usize * LANE_STRIDE + LANE_ROSTER + rd_u32(rec + REC_ROLE) as usize * 8)?.wrapping_add(120) >= tick };
                if seen { en += 1; } }
            for i in 0..5 { let e = rd_u64(w.x + X_ROSTER + side as usize * 0x28 + i * 8)? as usize; if e == 0 { continue; }
                if d2(me, e)? < al_d2 { al += 1; } }
            let mult = mult_of(en - al, &m4);
            let rbp = mult.wrapping_mul(v) / 100 + xv - (cost1 + cost2);
            tr(3, (v.clamp(-0x7fff, 0x7fff) as u64 & 0xffff) | (xv.clamp(-0x7fff, 0x7fff) as u64 & 0xffff) << 16 | ((cost1 + cost2).clamp(-0x7fff, 0x7fff) as u64 & 0xffff) << 32 | (en as u64) << 48 | (al as u64) << 52 | (found != 0) as u64 >> 0 << 56);
            if w.entity(th).is_none() { return Some(0); }
            tr(2, 0x404); Some(rbp)
        }
        6..=9 => {
            let th = rd_u64(act + SA_PAYLOAD8)?;
            if w.entity(th).is_none() { tr(2, 0x600); return Some(if cat == 9 { NEG } else { 0 }); }
            let rbp = cap_combat_score::find4(a.p1, p2, sim, hld)? as i64;
            if cat != 9 && w.entity(th).is_none() { return Some(0); }
            tr(2, 0x601); Some(rbp)
        }
        _ => None,
    }
}
