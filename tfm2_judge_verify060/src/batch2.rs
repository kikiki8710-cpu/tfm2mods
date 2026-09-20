//! batch2 — 2단계 재현체 배치 2(2026-09-20 · tier 1 잎 · 0.6.0 디스어셈 `MIG\dis060.py` 로 ABI·오프셋 확정).
//!   #42/#43 with_runaway(dd87b0 · Single/BattlePlan 공용) · #39 is_wave_priority_start_line(f87ea0 · game_core::utils::distance 재현 포함)
//!   · #126 v55_banish_penalty(e83000 · player_by_champion_id d72be0 재현) · #63 should_keep_object_for_contested_wave_priority(f18bf0)
//!   · #23 is_object_being_taken_by_enemy(f88690 · #63 의 콜리이자 독립 sweep 대상).
//! 공용 헬퍼(utils::distance = 1660a00 정수 sqrt · 룩업표 256 + 뉴턴/이분) 는 이후 배치가 재사용한다.
#![allow(non_snake_case, dead_code)]
use crate::sweep060::{note, pop, r32, r64, r8, top, SW};
use std::arch::asm;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::Ordering;

#[inline(always)] fn adiff(a: u64, b: u64) -> u64 { if a >= b { a - b } else { b - a } }

// ═══════════════════════════════════════════════════════════════════════════
// game_core::utils::distance(x1,y1,x2,y2) -> u64 — 0.6.0 1660a00(386B) · 정수 sqrt(abs_diff² 합)
//   n < 1,000,001: 룩업표(rip+… @0x3b19b48 · u16[256]) 초기값 + 뉴턴 1~2회 · 이상: bsr 기반 이분 탐색
// ═══════════════════════════════════════════════════════════════════════════
static ISQRT_TBL: [u16; 256] = [16, 23, 28, 32, 36, 40, 43, 46, 48, 51, 54, 56, 58, 60, 62, 64, 66, 68, 70, 72, 74, 76, 77, 79, 80, 82, 84, 85, 87, 88, 90, 91, 92, 94, 95, 96, 98, 99, 100, 102, 103, 104, 105, 107, 108, 109, 110, 111, 112, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 144, 145, 146, 147, 148, 149, 150, 151, 151, 152, 153, 154, 155, 156, 156, 157, 158, 159, 160, 160, 161, 162, 163, 164, 164, 165, 166, 167, 168, 168, 169, 170, 171, 171, 172, 173, 174, 174, 175, 176, 176, 177, 178, 179, 179, 180, 181, 182, 182, 183, 184, 184, 185, 186, 186, 187, 188, 188, 189, 190, 190, 191, 192, 192, 193, 194, 194, 195, 196, 196, 197, 198, 198, 199, 200, 200, 201, 202, 202, 203, 204, 204, 205, 205, 206, 207, 207, 208, 208, 209, 210, 210, 211, 212, 212, 213, 213, 214, 215, 215, 216, 216, 217, 218, 218, 219, 219, 220, 220, 221, 222, 222, 223, 223, 224, 224, 225, 226, 226, 227, 227, 228, 228, 229, 230, 230, 231, 231, 232, 232, 233, 233, 234, 235, 235, 236, 236, 237, 237, 238, 238, 239, 239, 240, 240, 241, 242, 242, 243, 243, 244, 244, 245, 245, 246, 246, 247, 247, 248, 248, 249, 249, 250, 250, 251, 251, 252, 252, 253, 253, 254, 254, 255, 255, 256, 256];
pub fn my_distance(x1: u64, y1: u64, x2: u64, y2: u64) -> u64 {
    let dx = adiff(x1, x2); let dy = adiff(y1, y2);
    let n = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
    if n < 0xf4241 {
        if n == 0 { return 0; }
        let shift: u32 = if n >= 0x10000 { ((63 - n.leading_zeros()) - 14) & !1 } else { 0 };
        let t = n >> shift; if t >= 0x10000 { panic!("bounds"); }
        let r10 = (ISQRT_TBL[(t >> 8) as usize] as u64) << (shift / 2);
        let mut r8 = r10 + 1;
        let mut x = (n / r8 + r8) >> 1;
        if x > r10 { return r8; }
        loop { if x == 0 { panic!("div0"); } r8 = x; x = (n / r8 + r8) >> 1; if x >= r8 { return r8; } }
    } else {
        let bits = 64 - n.leading_zeros();                       // bsr+1
        let c = bits / 2 + (bits & 1);
        let (mut lo, mut hi, mut res) = (1u64 << (c - 1), 1u64 << c, 0u64);
        while lo <= hi {
            let mid = (lo + hi) >> 1; let sq = mid.wrapping_mul(mid);
            if sq > n { hi = mid.wrapping_sub(1); } else { res = mid; lo = mid + 1; }
        }
        res
    }
}
/// AbstractGame vt 슬롯 호출(Rust ABI 2-레지스터 반환 포함) — rcx 1인자 · (rax, rdx) 반환
#[inline(never)] unsafe fn call1_2reg(f: usize, a: usize) -> (u64, u64) {
    let (rax, rdx): (u64, u64);
    asm!("push rbp", "mov rbp, rsp", "and rsp, -16", "sub rsp, 0x20", "call {f}", "mov rsp, rbp", "pop rbp",
         f = in(reg) f, inout("rcx") a => _, out("rax") rax, out("rdx") rdx, clobber_abi("C"));
    (rax, rdx)
}
type Fn1 = unsafe extern "C" fn(usize) -> u64;
type Fn2 = unsafe extern "C" fn(usize, u64) -> u64;
type Fn3 = unsafe extern "C" fn(usize, i64, u64) -> u64;

// ═══════════════════════════════════════════════════════════════════════════
// #42/#43 with_runaway — 0.6.0 dd87b0(SinglePlanBattle/BattlePlan 공용) · ABI (rcx=&self, rdx=version(미사용), r8=&OperationData) -> al
//   self+0x60 main_goal(0=TryKill) · +0x1c8 start_tick · +0x208 main_objective 태그(u8 · None=니치) · +0x1f8 with_dive
// ═══════════════════════════════════════════════════════════════════════════
unsafe fn my_with_runaway(me: usize, data: usize) -> bool {
    if r64(me, 0x60) == 0 {
        let cache = r64(data, 0) as usize; let game = r64(cache, 0) as usize; let vt = r64(cache, 8) as usize;
        let tick: Fn1 = core::mem::transmute(r64(vt, 0x28) as usize);
        let elapsed = tick(game).saturating_sub(r64(me, 0x1c8));
        let tps = r64(r64(r64(data, 8) as usize, 8) as usize, 0x12f8);
        if elapsed <= tps { return false; }
    }
    let obj = r8(me, 0x208) as u32;
    if obj < 3 || obj == 4 { return false; }
    if obj == 5 { return r8(me, 0x1f8) & 1 == 0; }
    true
}
pub unsafe extern "C" fn w_with_runaway(a0: usize, a1: usize, a2: usize) -> bool {
    const I: usize = 5;
    SW[I].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe extern "C" fn(usize, usize, usize) -> u64 = core::mem::transmute(SW[I].orig.load(Ordering::Relaxed));
    let t = top();
    let g = f(a0, a1, a2) & 0xff != 0;
    if t {
        SW[I].cmp.fetch_add(1, Ordering::Relaxed);
        match catch_unwind(AssertUnwindSafe(|| my_with_runaway(a0, a2))) {
            Ok(m) => if m != g { note(I, format!("g={} m={} | self={:#x} goal={} start={} obj={} dive={}", g, m, a0, r64(a0, 0x60), r64(a0, 0x1c8), r8(a0, 0x208), r8(a0, 0x1f8))); },
            Err(_) => { SW[I].pan.fetch_add(1, Ordering::Relaxed); }
        }
    }
    pop(); g
}

// ═══════════════════════════════════════════════════════════════════════════
// #39 is_wave_priority_start_line — 0.6.0 f87ea0 · ABI (rcx=&PlayerState, rdx=&OperationData, r8=line u8) -> al
//   line_minions = cache+{0x10,0x50,0x90}[line] + team*32 {ptr +0, len +0x18} · nexus cache+0x170+team*8 · first_tower cache+0x180+line*32+team*8
// ═══════════════════════════════════════════════════════════════════════════
unsafe fn my_is_wave_priority_start_line(player: usize, data: usize, line: u8) -> bool {
    let cache = r64(data, 0) as usize; let team = r64(player, 0xa00); let enemy = 1u64.wrapping_sub(team);
    let base = match line { 0 => 0x10usize, 1 => 0x50, _ => 0x90 };
    if enemy >= 2 { panic!("bounds"); }
    let lst = cache + base + enemy as usize * 32;
    if r64(lst, 0x18) < 9 { return false; }
    if team > 1 { panic!("bounds"); }
    let nexus = r64(cache, 0x170 + team as usize * 8) as usize; if nexus == 0 { return false; }
    let (nx, ny) = (r64(nexus, 0x660), r64(nexus, 0x668));
    let tower = r64(cache, 0x180 + (line as usize) * 32 + team as usize * 8) as usize;
    let (tx, ty) = if tower != 0 { (r64(tower, 0x660), r64(tower, 0x668)) } else {
        let (a, b): (u64, u64) = match line { 1 => (0x59d80, 0x90880), 2 => (0xa7f80, 0xdea80), _ => (0xbb80, 0x42680) };
        if team == 0 { (a, b) } else { (b, a) }
    };
    let td = my_distance(tx, ty, nx, ny);
    let (ptr, len) = (r64(lst, 0) as usize, r64(lst, 0x18) as usize);
    for k in 0..len {
        let m = r64(ptr, k * 8) as usize;
        if my_distance(r64(m, 0x660), r64(m, 0x668), nx, ny) < td { return true; }
    }
    false
}
pub unsafe extern "C" fn w_is_wave_priority_start_line(a0: usize, a1: usize, a2: u8) -> bool {
    const I: usize = 6;
    SW[I].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe extern "C" fn(usize, usize, u8) -> u64 = core::mem::transmute(SW[I].orig.load(Ordering::Relaxed));
    let t = top();
    let g = f(a0, a1, a2) & 0xff != 0;
    if t {
        SW[I].cmp.fetch_add(1, Ordering::Relaxed);
        match catch_unwind(AssertUnwindSafe(|| my_is_wave_priority_start_line(a0, a1, a2))) {
            Ok(m) => if m != g { note(I, format!("g={} m={} | team={} line={} player={:#x} data={:#x}", g, m, r64(a0, 0xa00), a2, a0, a1)); },
            Err(_) => { SW[I].pan.fetch_add(1, Ordering::Relaxed); }
        }
    }
    pop(); g
}

// ═══════════════════════════════════════════════════════════════════════════
// AbstractGameWithCache::player_by_champion_id(cache, id) -> Option<&PlayerState> — 0.6.0 d72be0
//   player_champion[team][slot](cache+0x1e0+team*40+slot*8).id == id → cache+0x230+team*40+slot*8 의 포인터
// ═══════════════════════════════════════════════════════════════════════════
pub unsafe fn my_player_by_champion_id(cache: usize, id: u64) -> usize {
    for team in 0..2usize { for slot in 0..5usize {
        let c = r64(cache, 0x1e0 + team * 40 + slot * 8) as usize;
        if c != 0 && r64(c, 0x5c0) == id { return r64(cache, 0x230 + team * 40 + slot * 8) as usize; }
    } }
    0
}

// ═══════════════════════════════════════════════════════════════════════════
// #126 v55_banish_penalty — 0.6.0 e83000 · ABI (rcx=version, rdx=&Effect{arc_ptr,vt}, r8=&OperationData, r9=&PlayerState, [rsp+0x28]=champ, [rsp+0x30]=t, [rsp+0x38]=hp_value) -> rax i64
//   EffectType vt +0xe0 has_banish_deep(&self)->bool · +0xa8 effect_cc_time(&self)->Option<usize>(al 태그 · rdx 값 · 0.5.8 +0xc0/+0x88 에서 +0x20 이동)
//   Arc<dyn EffectType> 데이터 = arc_ptr + ((vt.align(+0x10)-1) & !15) + 16
// ═══════════════════════════════════════════════════════════════════════════
unsafe fn my_v55_banish_penalty(effect: usize, data: usize, player: usize, champ: usize, t: usize, hp_value: i64) -> i64 {
    let vt = r64(effect, 8) as usize;
    let obj = (r64(effect, 0) as usize).wrapping_add(((r64(vt, 0x10) as usize).wrapping_sub(1)) & !15).wrapping_add(16);
    let has: Fn1 = core::mem::transmute(r64(vt, 0xe0) as usize);
    if has(obj) & 0xff == 0 { return 0; }
    let (tag, cc) = call1_2reg(r64(vt, 0xa8) as usize, obj);
    if tag & 1 == 0 { return 0; }
    let tps = r64(r64(r64(data, 8) as usize, 8) as usize, 0x12f8); if tps == 0 { panic!("div0"); }
    let mut banish_sec = cc / tps; if banish_sec < 1 { banish_sec = 1; }
    let cache = r64(data, 0) as usize;
    let tp = my_player_by_champion_id(cache, r64(t, 0x5c0)); if tp == 0 { return 0; }
    let team = r64(player, 0xa00); if team > 1 { panic!("bounds"); }
    let ti = r32(tp, 0xa90) as usize;
    let pc = cache + 0x1e0 + team as usize * 40;
    let champ_id = r64(champ, 0x5c0); let (tx, ty) = (r64(t, 0x660), r64(t, 0x668));
    let mut ally_dps: u64 = 0;
    for slot in 0..5usize {
        let a = r64(pc, slot * 8) as usize; if a == 0 { continue; }
        let aid = r64(a, 0x5c0); if aid == champ_id { continue; }
        let dx = adiff(r64(a, 0x660), tx); let dy = adiff(r64(a, 0x668), ty);
        if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) > 0x53d1ac100 { continue; }
        let ap = my_player_by_champion_id(cache, aid); if ap == 0 { continue; }
        let at = r64(ap, 0xa00); if at >= 2 { panic!("bounds"); }
        let cc_ = cache + 0x280 + at as usize * 0x1090 + r32(ap, 0xa90) as usize * 0x350;
        ally_dps = ally_dps.wrapping_add(r64(cc_, 0x190 + ti * 8)).wrapping_add(r64(cc_, 0x1b8 + ti * 8)).wrapping_add(r64(cc_, 0x1e0 + ti * 8)).wrapping_add(r64(cc_, 0x208 + ti * 8));
    }
    let mut lost = ally_dps.wrapping_mul(banish_sec); let max = r64(t, 0x628); if max < lost { lost = max; }
    let v = (lost as i64).wrapping_mul(hp_value);
    let hp = r64(t, 0x670) as i64; let hp1 = if hp >= 2 { hp } else { 1 };
    let q = v / hp1; let q2 = q / 2;
    if q2 < 80 { q2 } else { 80 }
}
pub unsafe extern "C" fn w_v55_banish_penalty(a0: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize, a6: i64) -> i64 {
    const I: usize = 7;
    SW[I].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, i64) -> i64 = core::mem::transmute(SW[I].orig.load(Ordering::Relaxed));
    let t = top();
    let g = f(a0, a1, a2, a3, a4, a5, a6);
    if t {
        SW[I].cmp.fetch_add(1, Ordering::Relaxed);
        match catch_unwind(AssertUnwindSafe(|| my_v55_banish_penalty(a1, a2, a3, a4, a5, a6))) {
            Ok(m) => if m != g { note(I, format!("g={} m={} | ver={} effect={:#x} team={} champ={:#x} t={:#x} t.hp={}/{} hp_value={}", g, m, a0, a1, r64(a3, 0xa00), a4, a5, r64(a5, 0x670), r64(a5, 0x628), a6)); },
            Err(_) => { SW[I].pan.fetch_add(1, Ordering::Relaxed); }
        }
    }
    pop(); g
}

// ═══════════════════════════════════════════════════════════════════════════
// #23 is_object_being_taken_by_enemy — 0.6.0 f88690 · ABI (rcx=&PlayerState, rdx=&OperationData, r8=&GoalData, r9=&TeamPlan, [rsp+0x28]=target u8(0 Morgard/1 Serpen)) -> al
//   ctx+0x38 tutorial(u8) · Serpen 존재 = tut≤8 ∧ bit(0x1a1) · Morgard 존재 = !(1≤tut≤6)
//   ★3번째 인자(r8)=GoalData: +0x88(Morgard)/+0xc0(Serpen) damaged_since · 4번째(r9)=TeamPlan: +0xa0/+0xa8 last_seen_tick (f18bf0 이 r8=goal_data, r9=self 로 부른다)
//   vt +0x40(game)→ (rax 오류태그, rdx &Objectives) · Objectives+0x1a0/0x1a8(Morgard) +0x1d0/0x1d8(Serpen) = 핸들 Option · +0x1f0(game, handle)→ Option<&Entity> · +0xf8 is_visible(game, team, id) · +0x28 tick
//   return last_seen+tps ≥ tick ∧ e.hp < e.max_hp ∧ damaged_since ≤ tps*20
// ═══════════════════════════════════════════════════════════════════════════
pub unsafe fn my_is_object_being_taken(player: usize, data: usize, goal: usize, tp: usize, target: u8) -> bool {
    let ctx = r64(data, 8) as usize; let tps = r64(r64(ctx, 8) as usize, 0x12f8); let tut = r8(ctx, 0x38) as u32;
    let (last, since, hoff) = if target != 0 {
        if !(tut <= 8 && (0x1a1u32 >> tut) & 1 == 1) { return false; }
        (r64(tp, 0xa8).wrapping_add(tps), r64(goal, 0xc0), 0x1d0usize)   // ★last_seen 은 4번째 인자(r9=TeamPlan)+0xa8 · since 는 3번째(r8=GoalData)+0xc0 — 첫 배포는 뒤바꿔 DIFF 126(진단으로 적발)
    } else {
        if tut.wrapping_sub(1) < 6 { return false; }
        (r64(tp, 0xa0).wrapping_add(tps), r64(goal, 0x88), 0x1a0usize)
    };
    let cache = r64(data, 0) as usize; let game = r64(cache, 0) as usize; let vt = r64(cache, 8) as usize;
    let tick: Fn1 = core::mem::transmute(r64(vt, 0x28) as usize); let now = tick(game);
    // vt+0x40 은 (rax=Err 태그, rdx=&Objectives) 2-레지스터 반환 — rax≠0 이면 unwrap 패닉 · 이후 [rdx+0x1a0/0x1d0] 은 그 구조체의 필드
    let (err, root) = call1_2reg(r64(vt, 0x40) as usize, game); if err != 0 { panic!("unwrap None"); }
    let root = root as usize;
    if r64(root, hoff + 8) == 0 { return false; }
    let h = r64(r64(root, hoff) as usize, 0);
    let ent: Fn2 = core::mem::transmute(r64(vt, 0x1f0) as usize); let e = ent(game, h) as usize; if e == 0 { return false; }
    let vis: Fn3 = core::mem::transmute(r64(vt, 0xf8) as usize);
    if vis(game, r64(player, 0xa00) as i64, r64(e, 0x5c0)) & 0xff == 0 { return false; }
    if last < now { return false; }
    if r64(e, 0x670) >= r64(e, 0x628) { return false; }
    since <= tps.wrapping_mul(20)
}
/// #8 갈림 진단 — 각 게이트 값을 그대로 찍는다(재현체 자체는 건드리지 않음)
unsafe fn dbg_obj(player: usize, data: usize, goal: usize, tp: usize, target: u8) -> String {
    let ctx = r64(data, 8) as usize; let tps = r64(r64(ctx, 8) as usize, 0x12f8);
    let (last, since, hoff) = if target != 0 { (r64(tp, 0xa8), r64(goal, 0xc0), 0x1d0usize) } else { (r64(tp, 0xa0), r64(goal, 0x88), 0x1a0usize) };
    let cache = r64(data, 0) as usize; let game = r64(cache, 0) as usize; let vt = r64(cache, 8) as usize;
    let tick: Fn1 = core::mem::transmute(r64(vt, 0x28) as usize); let now = tick(game);
    let (err, root) = call1_2reg(r64(vt, 0x40) as usize, game); let root = root as usize;
    let some = if err == 0 && root != 0 { r64(root, hoff + 8) } else { u64::MAX };
    let hptr = if some != 0 && some != u64::MAX { r64(root, hoff) as usize } else { 0 };
    let h = if hptr != 0 { r64(hptr, 0) } else { 0 };
    let ent: Fn2 = core::mem::transmute(r64(vt, 0x1f0) as usize); let e = if some != 0 && some != u64::MAX { ent(game, h) as usize } else { 0 };
    let (hp, max, vis) = if e != 0 { let v: Fn3 = core::mem::transmute(r64(vt, 0xf8) as usize); (r64(e, 0x670), r64(e, 0x628), v(game, r64(player, 0xa00) as i64, r64(e, 0x5c0)) & 0xff) } else { (0, 0, 9) };
    format!("tps={} now={} last={} since={} err={} root={:#x} some={:#x} hptr={:#x} h={:#x} e={:#x} hp={}/{} vis={}", tps, now, last, since, err, root, some, hptr, h, e, hp, max, vis)
}
pub unsafe extern "C" fn w_is_object_being_taken(a0: usize, a1: usize, a2: usize, a3: usize, a4: u8) -> bool {
    const I: usize = 8;
    SW[I].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe extern "C" fn(usize, usize, usize, usize, u8) -> u64 = core::mem::transmute(SW[I].orig.load(Ordering::Relaxed));
    let t = top();
    let g = f(a0, a1, a2, a3, a4) & 0xff != 0;
    if t {
        SW[I].cmp.fetch_add(1, Ordering::Relaxed);
        match catch_unwind(AssertUnwindSafe(|| my_is_object_being_taken(a0, a1, a2, a3, a4))) {
            Ok(m) => if m != g { note(I, format!("g={} m={} | team={} target={} goal={:#x} tp={:#x} tut={} | {}", g, m, r64(a0, 0xa00), a4, a2, a3, r8(r64(a1, 8) as usize, 0x38), dbg_obj(a0, a1, a2, a3, a4))); },
            Err(_) => { SW[I].pan.fetch_add(1, Ordering::Relaxed); }
        }
    }
    pop(); g
}

// ═══════════════════════════════════════════════════════════════════════════
// #63 should_keep_object_for_contested_wave_priority — 0.6.0 f18bf0 · ABI (rcx=&TeamPlan, rdx=&PlayerState, r8=&OperationData, r9=&GoalData, [rsp+0x28]=target u8) -> al
// ═══════════════════════════════════════════════════════════════════════════
unsafe fn my_should_keep_object(tp: usize, player: usize, data: usize, goal: usize, target: u8) -> bool {
    let tut = r8(r64(data, 8) as usize, 0x38) as u32;
    if target != 0 { if !(tut <= 8 && (0x1a1u32 >> tut) & 1 == 1) { return false; } }
    else if tut.wrapping_sub(1) < 6 { return false; }
    if !my_is_object_being_taken(player, data, goal, tp, target) { return false; }
    let team = r64(player, 0xa00); if team > 1 { panic!("bounds"); }
    let cache = r64(data, 0) as usize;
    let count = |t: u64| -> u64 {
        let pc = cache + 0x1e0 + t as usize * 40; let mut n = 0u64;
        for k in 0..5usize {
            let c = r64(pc, k * 8) as usize; if c == 0 { continue; }
            let max = r64(c, 0x628); if max == 0 { panic!("div0"); }
            if r64(c, 0x670).wrapping_mul(100) / max >= 40 { n += 1; }
        }
        n
    };
    count(team) + 1 >= count(1 - team)
}
pub unsafe extern "C" fn w_should_keep_object(a0: usize, a1: usize, a2: usize, a3: usize, a4: u8) -> bool {
    const I: usize = 9;
    SW[I].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe extern "C" fn(usize, usize, usize, usize, u8) -> u64 = core::mem::transmute(SW[I].orig.load(Ordering::Relaxed));
    let t = top();
    let g = f(a0, a1, a2, a3, a4) & 0xff != 0;
    if t {
        SW[I].cmp.fetch_add(1, Ordering::Relaxed);
        match catch_unwind(AssertUnwindSafe(|| my_should_keep_object(a0, a1, a2, a3, a4))) {
            Ok(m) => if m != g { note(I, format!("g={} m={} | team={} target={} tp={:#x} goal={:#x}", g, m, r64(a1, 0xa00), a4, a0, a3)); },
            Err(_) => { SW[I].pan.fetch_add(1, Ordering::Relaxed); }
        }
    }
    pop(); g
}
