//! sweep060 — 2단계(game==mine): 게임 원본 함수를 wrap 으로 감싸 **원본 실행 → 내 재현체(명세에서 직접 쓴 Rust) 실행 → 반환 비교**.
//!
//! 기구(0.5.8 `hookw.rs`/`sweep20.rs` 이식 · 클래식 SDK rlib 링크사본은 0.6.0 에 없으므로 재현체는 이 파일 안의 `my_*`):
//!   wrap 설치 = 트램폴린([원본 프롤로그 n바이트] + `jmp [rip+0]` + `dq fn+n`) + 진입부 `movabs rax,wrap; jmp rax`(12B)+NOP.
//!   wrap 은 원본과 같은 레지스터 상태로 진입하므로(꼬리 대체) `extern "C"` 로 같은 시그니처를 받아 트램폴린을 부른다.
//!   ★orig 는 진입부 패치 **전에** 게시(배경 sim 이 rayon 워커에서 같은 함수를 부르는 경합 — 0.5.8 hookw ①).
//!   ★재진입 깊이 추적(DEPTH · 0.5.8 top/pop) — 배치 1 은 순수 함수뿐이라 깊이 <4 까지 비교(상태 변이 함수는 depth==0 만).
//!   ★비교는 top-level 호출마다: g(원본 반환) != m(재현 반환) 이면 DIFF++ 하고 앞 32건은 인자와 함께 note.
//! 켜는 법(기본 OFF): `<게임>\mods\tfm2_judge_verify060\sweep060_on.txt` 한 줄에 `all` / `name:<부분>` / `idx,…`.
//!   sweep 이 걸린 함수엔 1단계 프로브를 걸지 않는다(같은 진입부 — 공존 불가).
//! ABI 는 0.6.0 exe 디스어셈(`MIG\dis060.py`)에서 직접 읽었다(인자 승격이 0.5.8 IR 시그니처와 다른 함수 있음 — 각 wrap 주석).
//! ⚠ vtable 슬롯 호출(AbstractGame +0x28 tick · +0xf8 is_visible · +0x150 entity_player)은 game_core 상태 접근자라 재현 범위 밖 = 그대로 호출(0.5.8 rlib 사본도 동일).
#![allow(non_snake_case, dead_code)]
use crate::{w, FlushInstructionCache, GetCurrentProcess, VirtualAlloc, VirtualProtect, BASE};
use std::cell::Cell;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;

pub struct Sw {
    pub name: &'static str,
    pub rva: usize,
    pub prolog: &'static [u8],
    pub wrap: fn() -> usize,
    pub orig: AtomicUsize,
    pub calls: AtomicU64,
    pub cmp: AtomicU64,
    pub diff: AtomicU64,
    pub pan: AtomicU64,
    pub installed: AtomicUsize, // 0 미설치 · 1 설치 · 2 프롤로그 불일치 · 3 실패
}
macro_rules! sw {
    ($name:expr, $rva:expr, $prolog:expr, $wrap:expr) => {
        Sw { name: $name, rva: $rva, prolog: $prolog, wrap: || $wrap as *const () as usize, orig: AtomicUsize::new(0), calls: AtomicU64::new(0), cmp: AtomicU64::new(0), diff: AtomicU64::new(0), pan: AtomicU64::new(0), installed: AtomicUsize::new(0) }
    };
}
/// 배치 1(tier 1 잎 · 동치 A) — 0.6.0 RVA · 프롤로그 = probe_tbl 채록 바이트(설치 시 완전 일치 검사).
pub static SW: [Sw; K] = [
    sw!("line_recall_pressure_penalty", 0xe73260, &[0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x20, 0x80, 0xb9, 0x88, 0x04, 0x00, 0x00, 0x00], w_line_recall),
    sw!("target_bush_v41", 0xfb9b70, &[0x48, 0x83, 0xec, 0x58, 0x89, 0xc8, 0x48, 0x8b, 0x8a, 0x00, 0x0a, 0x00, 0x00], w_target_bush_v41),
    sw!("AgentVerHamster::count_nearby_enemies", 0xf3d140, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], w_count_nearby),
    sw!("v23_healthy_allies_near_point", 0xf882e0, &[0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x20, 0x48, 0x8b, 0x89, 0x00, 0x0a, 0x00, 0x00], w_healthy_allies),
    sw!("v23_should_break_objective_hunt_anchor", 0xf89c30, &[0x41, 0x57, 0x41, 0x56, 0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x30, 0x48, 0x89, 0xcf], w_should_break_anchor),
    // ── 배치 2(09-20 · batch2.rs) ──
    sw!("with_runaway(Single/BattlePlan)", 0xdd87b0, &[0x56, 0x57, 0x48, 0x83, 0xec, 0x28, 0x48, 0x89, 0xce, 0x48, 0x83, 0x79, 0x60, 0x00], crate::batch2::w_with_runaway),
    sw!("is_wave_priority_start_line", 0xf87ea0, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53, 0x48, 0x83, 0xec, 0x20], crate::batch2::w_is_wave_priority_start_line),
    sw!("v55_banish_penalty", 0xe83000, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], crate::batch2::w_v55_banish_penalty),
    sw!("is_object_being_taken_by_enemy", 0xf88690, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], crate::batch2::w_is_object_being_taken),
    sw!("should_keep_object_for_contested_wave_priority", 0xf18bf0, &[0x56, 0x57, 0x48, 0x83, 0xec, 0x28, 0x4c, 0x89, 0xc6, 0x48, 0x89, 0xd7], crate::batch2::w_should_keep_object),
    // ── 배치 3(09-20 · batch3.rs) ──
    sw!("champion_hp_value_uncached", 0xde5780, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], crate::batch3::w_champion_hp_value),
    // ── 배치 4(09-20 · batch4.rs) ──
    sw!("SmallActionRecall::is_end", 0xfdb0e0, &[0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x20, 0x48, 0x89, 0xce, 0x49, 0x8b, 0x89, 0x00, 0x0a, 0x00, 0x00], crate::batch4::w_is_end),
    sw!("LegacyPlanHandler::take_misunderstood_received_chat", 0xd59280, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], crate::batch4::w_take_misunderstood),
];
pub const K: usize = 13;
const NOTE_MAX: usize = 32;
static NOTES: Mutex<Vec<String>> = Mutex::new(Vec::new());
thread_local! { static DEPTH: Cell<u32> = const { Cell::new(0) }; }

/// 비교 허용 깊이. 배치 1 은 전부 순수 함수(상태 변이 0)라 중첩 호출도 비교한다(#4 원본 → #3 wrap 재진입 같은 경우 · 안 그러면 #3 은 top-level 표본이 0).
///   상태 변이 함수를 얹을 때는 그 wrap 에서 `depth==0` 조건을 따로 걸 것(0.5.8 top/pop 규율).
const CMP_MAX_DEPTH: u32 = 4;
#[inline(always)] pub(crate) fn top() -> bool { DEPTH.with(|d| { let v = d.get(); d.set(v + 1); v < CMP_MAX_DEPTH }) }
#[inline(always)] pub(crate) fn pop() { DEPTH.with(|d| d.set(d.get().saturating_sub(1))); }
pub(crate) fn note(i: usize, s: String) {
    let n = SW[i].diff.fetch_add(1, Ordering::Relaxed);
    if n < NOTE_MAX as u64 { let mut g = NOTES.lock().unwrap_or_else(|e| e.into_inner()); g.push(format!("[diff] #{} {} — {}", i, SW[i].name, s)); }
}
#[inline(always)] pub(crate) unsafe fn r64(p: usize, off: usize) -> u64 { core::ptr::read_unaligned((p + off) as *const u64) }
#[inline(always)] pub(crate) unsafe fn r32(p: usize, off: usize) -> u32 { core::ptr::read_unaligned((p + off) as *const u32) }
#[inline(always)] pub(crate) unsafe fn r8(p: usize, off: usize) -> u8 { *((p + off) as *const u8) }
#[inline(always)] fn adiff(a: u64, b: u64) -> u64 { if a >= b { a - b } else { b - a } }
/// game_core::utils::distance_sq(x1,y1,x2,y2) 인라인형 = abs_diff² 합(wrapping)
#[inline(always)] fn dist_sq(x1: u64, y1: u64, x2: u64, y2: u64) -> u64 { let dx = adiff(x1, x2); let dy = adiff(y1, y2); dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) }

// ── AbstractGame dyn 슬롯(0.6.0 · f3d140/f895f0 실측) ──
type FnIsVisible = unsafe extern "C" fn(usize, i64, u64) -> u64;   // vt+0xf8 (game, team, entity_id) -> bool(al)
type FnEntityPlayer = unsafe extern "C" fn(usize, u64) -> usize;    // vt+0x150 (game, entity_id) -> Option<&PlayerState>
type FnTick = unsafe extern "C" fn(usize) -> u64;                   // vt+0x28 (game) -> tick
/// Blackboard::is_recent_visible(bb, game(cache+0/+8), my_team, enemy entity) 인라인형(blackboard.rs:348~350)
///   = game.is_visible(my_team, e.id) || (game.entity_player(e.id) 가 Some(p) 이고 bb.last_visible[p.position]+120 >= game.tick())
unsafe fn is_recent_visible(bb: usize, cache: usize, my_team: i64, ent: usize) -> bool {
    let game = r64(cache, 0) as usize; let vt = r64(cache, 8) as usize;
    let eid = r64(ent, 0x5c0);
    let f_vis: FnIsVisible = core::mem::transmute(r64(vt, 0xf8) as usize);
    if f_vis(game, my_team, eid) & 0xff != 0 { return true; }
    let f_ep: FnEntityPlayer = core::mem::transmute(r64(vt, 0x150) as usize);
    let p = f_ep(game, eid);
    if p == 0 { return false; }
    let pos = r32(p, 0xa90) as usize;
    let last = r64(bb, 0x3e8 + pos * 8).wrapping_add(0x78);
    let f_tick: FnTick = core::mem::transmute(r64(vt, 0x28) as usize);
    last >= f_tick(game)
}
/// v23_healthy(e, min_hp_ratio) = e.hp*100 / e.stat_cached.hp >= min (unsigned · max==0 → div0 패닉)
#[inline(always)] unsafe fn healthy(e: usize, min: u64) -> bool {
    let max = r64(e, 0x628); if max == 0 { panic!("div0"); }
    r64(e, 0x670).wrapping_mul(100) / max >= min
}

// ═══════════════════════════════════════════════════════════════════════════
// #120 line_recall_pressure_penalty — 0.6.0 e73260 · ABI (rcx=&Entity, rdx=punish_damage i64, r8=hp_value i64) -> rax i64
// ═══════════════════════════════════════════════════════════════════════════
unsafe fn my_line_recall(champ: usize, punish: i64, hp_value: i64) -> i64 {
    if r8(champ, 0x488) != 0 { return 0; }                                  // L239 undying
    let max_raw = r64(champ, 0x628); let max_hp = if max_raw < 1 { 1 } else { max_raw } as i64; // L242 umax(.,1)
    let hp = r64(champ, 0x670) as i64;
    let after = hp.wrapping_sub(punish); let after = if after > 0 { after } else { 0 };   // L243 smax(.,0)
    let pct = after.wrapping_mul(100) / max_hp;                             // L244
    let mut penalty: i64 = 0;
    if pct < 45 {                                                           // L246
        penalty = (45 - pct).wrapping_mul(hp_value) / 60;                   // L247 (signed /60)
        if pct < 25 { penalty = penalty.wrapping_add((hp_value as u64 / 5) as i64); } // L249 (unsigned /5)
    }
    if punish.wrapping_mul(100) / max_hp > 11 {                             // L252
        let hp1 = if (hp as u64) < 1 { 1 } else { hp };                     // max(hp,1) (unsigned)
        let q = hp_value.wrapping_mul(punish) / hp1;                        // L253 분모 = 현재 hp
        penalty = penalty.wrapping_add(q / 2);
    }
    penalty
}
unsafe extern "C" fn w_line_recall(a0: usize, a1: i64, a2: i64) -> i64 {
    const I: usize = 0;
    SW[I].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe extern "C" fn(usize, i64, i64) -> i64 = core::mem::transmute(SW[I].orig.load(Ordering::Relaxed));
    let t = top();
    let g = f(a0, a1, a2);
    if t {
        SW[I].cmp.fetch_add(1, Ordering::Relaxed);
        match catch_unwind(AssertUnwindSafe(|| my_line_recall(a0, a1, a2))) {
            Ok(m) => if m != g { note(I, format!("g={} m={} | champ={:#x} undying={} max={} hp={} punish={} hp_value={}", g, m, a0, r8(a0, 0x488), r64(a0, 0x628), r64(a0, 0x670), a1, a2)); },
            Err(_) => { SW[I].pan.fetch_add(1, Ordering::Relaxed); }
        }
    }
    pop(); g
}

// ═══════════════════════════════════════════════════════════════════════════
// #58 LineGankerPlan::target_bush_v41 — 0.6.0 fb9b70 · ABI (cl=self.line u8, rdx=&PlayerState, r8=&OperationData) -> rax usize
//   (0.5.8 IR 는 team/position 스칼라 승격이었으나 0.6.0 exe 는 player 포인터를 받아 +0xa00/+0xa90 을 직접 읽는다)
// ═══════════════════════════════════════════════════════════════════════════
unsafe fn my_target_bush_v41(line: u8, player: usize, data: usize) -> usize {
    let team = r64(player, 0xa00) as usize;
    let cache = r64(data, 0) as usize;
    let (tbl, lead): ([usize; 7], u64) = match line {
        0 => {                                                              // Top
            if team > 1 { panic!("bounds"); }
            (if team == 0 { [16, 6, 3, 3, 3, 2, 2] } else { [2, 3, 6, 6, 6, 16, 16] }, r64(cache, 0x23a0 + team * 8))
        }
        1 => {                                                              // Mid
            if team > 1 { panic!("bounds"); }
            let pos = r32(player, 0xa90) as usize;
            let champ = r64(cache, 0x1e0 + team * 40 + pos * 8) as usize;
            if champ == 0 { panic!("unwrap None"); }
            let ctx = r64(data, 8) as usize; let setting = r64(ctx, 8) as usize;
            let height = r64(setting, 0x12c0);
            let top_side = height.wrapping_sub(r64(champ, 0x668)) < r64(champ, 0x660) as u64;   // is_top_side: (height-y) < x
            let s = top_side as usize;
            let a = s * 3 + 11; let d = s * 4 + 17; let e = s * 5 + 4;
            (if team == 0 { [d, a, a, a, a, e, e] } else { [e, a, a, a, a, d, d] }, r64(cache, 0x23b0 + team * 8))
        }
        _ => {                                                              // Bottom
            if team > 1 { panic!("bounds"); }
            (if team == 0 { [21, 20, 15, 15, 15, 9, 7] } else { [9, 15, 20, 20, 20, 21, 23] }, r64(cache, 0x23c0 + team * 8))
        }
    };
    let idx = if lead < 6 { lead as usize } else { 6 };
    tbl[idx]
}
unsafe extern "C" fn w_target_bush_v41(a0: u8, a1: usize, a2: usize) -> usize {
    const I: usize = 1;
    SW[I].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe extern "C" fn(u8, usize, usize) -> usize = core::mem::transmute(SW[I].orig.load(Ordering::Relaxed));
    let t = top();
    let g = f(a0, a1, a2);
    if t {
        SW[I].cmp.fetch_add(1, Ordering::Relaxed);
        match catch_unwind(AssertUnwindSafe(|| my_target_bush_v41(a0, a1, a2))) {
            Ok(m) => if m != g { note(I, format!("g={} m={} | line={} team={} pos={} player={:#x} data={:#x}", g, m, a0, r64(a1, 0xa00), r32(a1, 0xa90), a1, a2)); },
            Err(_) => { SW[I].pan.fetch_add(1, Ordering::Relaxed); }
        }
    }
    pop(); g
}

// ═══════════════════════════════════════════════════════════════════════════
// #229 AgentVerHamster::count_nearby_enemies — 0.6.0 f3d140 · ABI (rcx=champ.x, rdx=champ.y, r8=player.team i64, r9=&AbstractGameWithCache, [rsp+0x28]=&[Blackboard;2]) -> ax u16 비트마스크
//   (0.5.8 IR 는 %2=&PlayerState 였으나 0.6.0 exe 는 team 스칼라로 승격 · 5번째 인자 = blackboard 배열 기저)
// ═══════════════════════════════════════════════════════════════════════════
unsafe fn my_count_nearby(x: u64, y: u64, team: i64, cache: usize, bb: usize) -> u16 {
    let enemy = 1i64.wrapping_sub(team) as u64; if enemy > 1 { panic!("bounds"); }
    let pc = cache + 0x1e0 + enemy as usize * 40;
    let bbe = bb + enemy as usize * 0x5c8;
    let mut mask: u16 = 0;
    for k in 0..5 {
        let c = r64(pc, k * 8) as usize; if c == 0 { continue; }
        if !is_recent_visible(bbe, cache, team, c) { continue; }
        if dist_sq(r64(c, 0x660), r64(c, 0x668), x, y) < 40000000001 { mask |= 1 << k; }   // < 200000²+1
    }
    mask
}
unsafe extern "C" fn w_count_nearby(a0: u64, a1: u64, a2: i64, a3: usize, a4: usize) -> u16 {
    const I: usize = 2;
    SW[I].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe extern "C" fn(u64, u64, i64, usize, usize) -> u64 = core::mem::transmute(SW[I].orig.load(Ordering::Relaxed));
    let t = top();
    let g = (f(a0, a1, a2, a3, a4) & 0xffff) as u16;
    if t {
        SW[I].cmp.fetch_add(1, Ordering::Relaxed);
        match catch_unwind(AssertUnwindSafe(|| my_count_nearby(a0, a1, a2, a3, a4))) {
            Ok(m) => if m != g { note(I, format!("g={:#x} m={:#x} | x={} y={} team={} cache={:#x} bb={:#x}", g, m, a0, a1, a2, a3, a4)); },
            Err(_) => { SW[I].pan.fetch_add(1, Ordering::Relaxed); }
        }
    }
    pop(); g
}

// ═══════════════════════════════════════════════════════════════════════════
// #24 v23_healthy_allies_near_point — 0.6.0 f882e0 · ABI (rcx=&PlayerState, rdx=&OperationData, r8=x, r9=y, [rsp+0x28]=range, [rsp+0x30]=min_hp_ratio) -> rax usize
// ═══════════════════════════════════════════════════════════════════════════
unsafe fn my_healthy_allies(player: usize, data: usize, x: u64, y: u64, range: u64, min_hp: u64) -> usize {
    let team = r64(player, 0xa00) as usize; if team > 1 { panic!("bounds"); }
    let cache = r64(data, 0) as usize; let pc = cache + 0x1e0 + team * 40;
    let rs = range.wrapping_mul(range);
    let mut n = 0usize;
    for k in 0..5 {
        let c = r64(pc, k * 8) as usize; if c == 0 { continue; }
        if !healthy(c, min_hp) { continue; }
        if dist_sq(r64(c, 0x660), r64(c, 0x668), x, y) <= rs { n += 1; }
    }
    n
}
/// #29 v23_recent_visible_enemies_near_point — 0.6.0 f895f0 · 같은 ABI · 적팀(1-team) · healthy → 거리 → is_recent_visible(bb[enemy], my_team) 단락
unsafe fn my_recent_visible_enemies(player: usize, data: usize, x: u64, y: u64, range: u64, min_hp: u64) -> usize {
    let team = r64(player, 0xa00); let enemy = 1u64.wrapping_sub(team); if enemy > 1 { panic!("bounds"); }
    let cache = r64(data, 0) as usize; let pc = cache + 0x1e0 + enemy as usize * 40;
    let bbe = r64(data, 0x10) as usize + enemy as usize * 0x5c8;
    let rs = range.wrapping_mul(range);
    let mut n = 0usize;
    for k in 0..5 {
        let c = r64(pc, k * 8) as usize; if c == 0 { continue; }
        if !healthy(c, min_hp) { continue; }
        if dist_sq(r64(c, 0x660), r64(c, 0x668), x, y) > rs { continue; }
        if is_recent_visible(bbe, cache, team as i64, c) { n += 1; }
    }
    n
}
unsafe extern "C" fn w_healthy_allies(a0: usize, a1: usize, a2: u64, a3: u64, a4: u64, a5: u64) -> usize {
    const I: usize = 3;
    SW[I].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe extern "C" fn(usize, usize, u64, u64, u64, u64) -> usize = core::mem::transmute(SW[I].orig.load(Ordering::Relaxed));
    let t = top();
    let g = f(a0, a1, a2, a3, a4, a5);
    if t {
        SW[I].cmp.fetch_add(1, Ordering::Relaxed);
        // 헬퍼 자가검증(is_near_line · 실좌표 (x,y) · 3 라인)
        let _ = catch_unwind(AssertUnwindSafe(|| crate::helpers::selftest_is_near_line(r64(a1, 8) as usize, a2, a3)));
        match catch_unwind(AssertUnwindSafe(|| my_healthy_allies(a0, a1, a2, a3, a4, a5))) {
            Ok(m) => if m != g { note(I, format!("g={} m={} | team={} x={} y={} range={} min_hp={} player={:#x} data={:#x}", g, m, r64(a0, 0xa00), a2, a3, a4, a5, a0, a1)); },
            Err(_) => { SW[I].pan.fetch_add(1, Ordering::Relaxed); }
        }
    }
    pop(); g
}

// ═══════════════════════════════════════════════════════════════════════════
// #136 v23_should_break_objective_hunt_anchor — 0.6.0 f89c30 · ABI (rcx=&PlayerState, rdx=&OperationData, r8=&Entity champ, r9=&Entity objective, [rsp+0x28]=camp_x, [rsp+0x30]=camp_y) -> al bool
//   내부 호출 f882e0/f895f0 도 재현체(my_*)로 대체(완전 재구현) — 원본은 트램폴린 경유로 f882e0 wrap 에 재진입하지만 DEPTH>0 이라 비교 없음.
// ═══════════════════════════════════════════════════════════════════════════
unsafe fn my_should_break_anchor(player: usize, data: usize, champ: usize, obj: usize, cx: u64, cy: u64) -> bool {
    let max = r64(obj, 0x628); if max == 0 { panic!("div0"); }
    if r64(obj, 0x670).wrapping_mul(100) / max < 21 { return false; }               // L60 오브젝트 HP<21% → 앵커 유지
    let (px, py) = (r64(champ, 0x660), r64(champ, 0x668));
    let pa = my_healthy_allies(player, data, px, py, 180000, 40);                   // L64
    let pe = my_recent_visible_enemies(player, data, px, py, 160000, 40);           // L65
    if pe >= 2 && (pe as u32) > (pa as u32) { return true; }                        // L66
    let a = my_healthy_allies(player, data, cx, cy, 180000, 40);                    // L70 v23_visible_objective_overload
    let e = my_recent_visible_enemies(player, data, cx, cy, 180000, 40);
    e >= 3 && e >= a + 2
}
unsafe extern "C" fn w_should_break_anchor(a0: usize, a1: usize, a2: usize, a3: usize, a4: u64, a5: u64) -> bool {
    const I: usize = 4;
    SW[I].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe extern "C" fn(usize, usize, usize, usize, u64, u64) -> u64 = core::mem::transmute(SW[I].orig.load(Ordering::Relaxed));
    let t = top();
    let g = f(a0, a1, a2, a3, a4, a5) & 0xff != 0;
    if t {
        SW[I].cmp.fetch_add(1, Ordering::Relaxed);
        match catch_unwind(AssertUnwindSafe(|| my_should_break_anchor(a0, a1, a2, a3, a4, a5))) {
            Ok(m) => if m != g { note(I, format!("g={} m={} | team={} champ=({},{}) obj hp={}/{} camp=({},{})", g, m, r64(a0, 0xa00), r64(a2, 0x660), r64(a2, 0x668), r64(a3, 0x670), r64(a3, 0x628), a4, a5)); },
            Err(_) => { SW[I].pan.fetch_add(1, Ordering::Relaxed); }
        }
    }
    pop(); g
}

// ═══════════════════════════════════════════════════════════════════════════
// 설치
// ═══════════════════════════════════════════════════════════════════════════
/// 성공 1 · 프롤로그 불일치 2 · 실패 3. 실패 시 .text 무손상.
pub unsafe fn install_wrap(i: usize, stub: usize) -> usize {
    let s = &SW[i]; let base = BASE.load(Ordering::Relaxed); let fn_addr = base + s.rva; let n = s.prolog.len();
    if n < 12 || n > 40 || stub == 0 { return 3; }
    if *(fn_addr as *const u8) == 0x48 && *((fn_addr + 1) as *const u8) == 0xb8 { return 2; }   // 이미 훅됨(체인 미지원)
    for k in 0..n { if *((fn_addr + k) as *const u8) != s.prolog[k] { return 2; } }
    let mut t: Vec<u8> = Vec::with_capacity(n + 14);
    t.extend_from_slice(s.prolog);
    t.extend_from_slice(&[0xff, 0x25, 0, 0, 0, 0]); t.extend_from_slice(&(fn_addr + n).to_le_bytes());   // jmp [rip+0]; dq fn+n
    core::ptr::copy_nonoverlapping(t.as_ptr(), stub as *mut u8, t.len());
    FlushInstructionCache(GetCurrentProcess(), stub, t.len());
    s.orig.store(stub, Ordering::SeqCst);                                                                 // ★패치 전에 게시
    let wrap = (s.wrap)();
    let mut patch = vec![0x90u8; n];
    patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&wrap.to_le_bytes()); patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, n, 0x40, &mut old) == 0 { s.orig.store(0, Ordering::SeqCst); return 3; }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, n);
    VirtualProtect(fn_addr, n, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, n);
    1
}
pub fn selected(dir: &str) -> Vec<usize> {
    let Ok(txt) = std::fs::read_to_string(format!(r"{}\sweep060_on.txt", dir)) else { return vec![] };
    let mut out = vec![];
    for line in txt.lines() {
        let line = line.trim(); if line.is_empty() || line.starts_with('#') { continue; }
        if line == "all" { return (0..K).collect(); }
        for tok in line.split(',') {
            let tok = tok.trim();
            if let Some(s) = tok.strip_prefix("name:") { out.extend((0..K).filter(|&i| SW[i].name.contains(s))); }
            else if let Ok(v) = tok.parse::<usize>() { if v < K { out.push(v); } }
        }
    }
    out.sort(); out.dedup(); out
}
/// 설치 후 sweep 이 걸린 RVA 집합(프로브가 건너뛸 대상)
pub unsafe fn install_all(dir: &str) -> Vec<usize> {
    let sel = selected(dir);
    if sel.is_empty() { w("[sweep] sweep060_on.txt 없음/빈 파일 → sweep 0 (기본 OFF)"); return vec![]; }
    let block = VirtualAlloc(0, sel.len() * 64, 0x1000 | 0x2000, 0x40);
    if block == 0 { w("[sweep] VirtualAlloc 실패"); return vec![]; }
    let mut rvas = vec![];
    for (k, &i) in sel.iter().enumerate() {
        let r = install_wrap(i, block + k * 64);
        SW[i].installed.store(r, Ordering::Relaxed);
        match r { 1 => { rvas.push(SW[i].rva); w(&format!("[sweep] ✓ #{} {:x} {} (wrap {:#x} orig {:#x})", i, SW[i].rva, SW[i].name, (SW[i].wrap)(), SW[i].orig.load(Ordering::Relaxed))); }
                  2 => w(&format!("[sweep] ✗ 프롤로그 불일치/선점 #{} {:x} {}", i, SW[i].rva, SW[i].name)),
                  _ => w(&format!("[sweep] ✗ 실패 #{} {:x} {}", i, SW[i].rva, SW[i].name)) }
    }
    w(&format!("[sweep] 선택 {} · 설치 {}", sel.len(), rvas.len()));
    rvas
}
static LAST: Mutex<Vec<(u64, u64)>> = Mutex::new(Vec::new());
static HLAST: Mutex<(u64, u64)> = Mutex::new((0, 0));
/// 5초마다: 변화 있는 항목 + 누적 note 를 내보낸다.
pub fn snapshot(f: u64) -> Option<String> {
    let mut g = LAST.lock().unwrap_or_else(|e| e.into_inner());
    if g.len() != K { *g = vec![(0, 0); K]; }
    let mut lines = vec![];
    for i in 0..K {
        if SW[i].installed.load(Ordering::Relaxed) != 1 { continue; }
        let (c, d) = (SW[i].calls.load(Ordering::Relaxed), SW[i].diff.load(Ordering::Relaxed));
        if (c, d) == g[i] { continue; }
        lines.push(format!("  [sweep #{}] {} calls={} cmp={} DIFF={} pan={}", i, SW[i].name, c, SW[i].cmp.load(Ordering::Relaxed), d, SW[i].pan.load(Ordering::Relaxed)));
        g[i] = (c, d);
    }
    let mut notes: Vec<String> = { let mut n = NOTES.lock().unwrap_or_else(|e| e.into_inner()); std::mem::take(&mut *n) };
    { let (hc, hd) = (crate::helpers::H_CALLS.load(Ordering::Relaxed), crate::helpers::H_DIFF.load(Ordering::Relaxed));
      if hc > 0 && (hc, hd) != *HLAST.lock().unwrap_or_else(|e| e.into_inner()) { lines.push(format!("  [helper] is_near_line selftest calls={} DIFF={}", hc, hd)); *HLAST.lock().unwrap_or_else(|e| e.into_inner()) = (hc, hd); }
      let mut hn = crate::helpers::H_NOTE.lock().unwrap_or_else(|e| e.into_inner()); notes.extend(std::mem::take(&mut *hn)); }
    if lines.is_empty() && notes.is_empty() { return None; }
    Some(format!("[f{} sweep]\n{}\n{}", f, lines.join("\n"), notes.join("\n")))
}
