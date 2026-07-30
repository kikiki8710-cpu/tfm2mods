//! tfm2_viewresult_probe — "경기결과 즉시보기" 버튼 (A)페이싱/(B)저장로드 판별 + full-outcome 발산 체커
//! ===========================================================================
//! 목적(2축):
//!  [Part 1] 관전 중 "즉시보기" 누르면 그 경기 sim(MobaMode::tick)이 **폭주하며 sim_tick 끝까지 도는지**(=A 페이싱)
//!           vs **멈추고 결과가 뜨는지**(=B 저장 확정결과 로드)를, per-thread (qpc, sim_tick) 타임라인으로 판별.
//!  [Part 2] 같은 경기(=(seed,fp) 파티션)를 배경 worker sim vs 관전 sim이 돌릴 때 **팀 스코어(ed50/ed58) + 챔피언
//!           상태해시(HP/pos/name)**가 갈리는지(=관전≠확정 full-outcome 발산). 세르펜 킬로그만 보던 seedprobe 확장.
//!
//! 안전: MobaMode::tick(0x230c290) **하나만** entry-observe 후킹(원본 무변경 실행). 전 read SEH 보호, 본문 catch_unwind.
//!   ⚠파일IO는 detour 금지 → detour는 in-memory만 갱신, dump는 post_update(메인)에서. (seedprobe 검증 패턴 그대로)
//!   ⚠serpen 계열은 이 함수 충돌 → 반드시 off. ai_adjust는 이 함수 미후킹이라 공존 가능.
//! ===========================================================================
#![allow(unused_imports, unused_variables)]
use mod_api::*;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::collections::HashMap;

const MOD_ID: &str = "tfm2_viewresult_probe";

// ── MobaMode::tick (매 틱, rcx=World/provider) 0.5.2 (serpen/seedprobe 검증 상수) ──
const MOBATICK_RVA: usize = 0x230c290;
const MOBATICK_PROLOGUE: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
// provider(=World) 오프셋 (serpen 0.5.2 프로덕션 상수)
const SEED_OFF: usize = 0xeab8;        // 경기 seed(불변)
const SIM_TICK_OFF: usize = 0xeac0;    // sim tick(경과)
const KILLS_PTR_OFF: usize = 0xed20;   // serpen kills Vec<{team,tick}> ptr
const KILLS_LEN_OFF: usize = 0xed28;
const KILLS_BLUE_OFF: usize = 0xed50;  // 팀 스코어(blue kills)
const KILLS_RED_OFF: usize = 0xed58;   // 팀 스코어(red kills)
// 챔피언 슬롯맵 오프셋 (serpen 0.5.2)
const ENTITY_KIND_OFF: usize = 0x68;
const CHAMP_KIND: u64 = 0xd;
const W_CHAMP_DENSE: usize = 0x720;    // ptr / +8 len (stride 0x6a8)
const W_CHAMP_SLOTS: usize = 0x738;    // ptr / +8 len (stride 0x10)
const W_PLAYER_DENSE: usize = 0x840;   // ptr / +8 len (stride 0x8d0)
const P_TEAM: usize = 0x820;
const P_CHAMP_TAG: usize = 0x8b8;
const P_CHAMP_KEY: usize = 0x8c0;
const CHAMP_STRIDE: usize = 0x6a8;
const PLAYER_STRIDE: usize = 0x8d0;
// 챔피언 상태 오프셋 (serpen.rs 검증본)
const E_HP_CUR: usize = 0x658;
const E_HP_MAX: usize = 0x610;
const E_POS_X: usize = 0x648;
const E_POS_Y: usize = 0x650;
const E_NAME_PTR: usize = 0x250;
const E_NAME_LEN: usize = 0x258;

const KC_SLOTS: usize = 256;            // world 캐시 슬롯(동시 경기 多 → 넉넉히)
const SAMPLE_STRIDE: u64 = 32;          // ★이 배수의 **정확한** sim_tick에서만 샘플 → 두 스레드가 동일 tick 비교(정렬-robust, 위치 어긋남 아티팩트 제거)
const MAX_SAMPLES_PER_MATCH: usize = 1200;
const TIMELINE_CAP: usize = 8000;       // 타임라인 링버퍼 상한

// ══ [07-29] 위협/해저드 해시맵 해시코어 `0x21fdcd0` 진입 관측 ══
//   목적: 이 맵의 해시 시드(map+0x20=k0 / map+0x28=k1)가 **인스턴스·스레드마다 다른가**를 판정.
//   AI가 이 맵을 "전수 순회 → 조건 통과 첫 엔트리 채택"으로 소비(position_eval 0x22dd9a0 등)하므로,
//   시드가 다르면 **동일 키 집합이라도 순회 순서가 재배열**되어 통과 엔트리 ≥2일 때 판정이 갈린다.
//   ★판정: 전 슬롯 (k0,k1) 동일 = **고정 시드** ⟹ 비결정원 아님(순서 증폭기로 격하)
//          스레드/인스턴스마다 상이 = **비결정 확정** ⟹ 배경 sim vs 관전 sim 발산의 엔진측 근원
//   read-only(레지스터·메모리 읽기만), 원본 무변경 entry-observe.
const HAZ_RVA: usize = 0x21fdcd0;
const HAZ_PROLOGUE: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x38];
const HAZ_SLOTS: usize = 64;
#[allow(clippy::declare_interior_mutable_const)]
const HAZ_Z32: AtomicU32 = AtomicU32::new(0);
static HAZ_TID: [AtomicU32; HAZ_SLOTS] = [HAZ_Z32; HAZ_SLOTS];
static HAZ_K0: [AtomicU64; HAZ_SLOTS] = [KC_ZERO; HAZ_SLOTS];
static HAZ_K1: [AtomicU64; HAZ_SLOTS] = [KC_ZERO; HAZ_SLOTS];
static HAZ_CNT: [AtomicU64; HAZ_SLOTS] = [KC_ZERO; HAZ_SLOTS];
static HAZ_CALLS: AtomicU64 = AtomicU64::new(0);
static HAZ_INSTALLED: AtomicBool = AtomicBool::new(false);
static HAZ_STATE: AtomicU32 = AtomicU32::new(0);
// 같은 시드 연속 호출은 슬롯 스캔 없이 카운터만 증가(핫함수 부하 억제)
thread_local! { static HAZ_LAST: std::cell::Cell<(u64, u64, usize)> = const { std::cell::Cell::new((0, 0, usize::MAX)) }; }

// ══ [07-29] fight_check(`0x1dbad70`) 입력 관측 ══
//   후보 가설: 이 함수는 **TLS 메모 캐시**(키=엔티티 이름뿐, 결과는 좌표·HP 의존)를 쓴다.
//   thread-local이라 "그 스레드가 직전에 뭘 계산했나"가 hit/miss를 바꿔 배경 워커 vs 관전에서 결과가 갈릴 수 있다.
//   ★1단계(여기) = **입력**이 두 sim에서 언제 갈리는지 측정. 입력이 상태 발산보다 **먼저** 갈리면 이 함수는 하류(결과),
//     입력이 상태 발산 시점까지 **일치**하면 "같은 입력 → 다른 결과"(=캐시 오염) 가설이 살아남는다(2단계 리턴훅 필요).
//   인자 레이아웃 미확정이라 rcx/rdx/r8/r9 중 **챔피언 엔티티**(+0x68==0xd)인 것을 찾아 name+hp+pos를 값 기반 해싱.
const FC_RVA: usize = 0x1dbad70;
const FC_PROLOGUE: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
const FC_SEEDS: usize = 32;
const FC_BUCKETS: usize = 4096;   // ★8틱 × 4096 = 32,768틱 커버(구 512=4096틱은 경기 길이에 못 미쳐 tick 4096+가 마지막 버킷에 뭉쳐 무조건 불일치로 찍히는 계측버그였음)
static FC_SEED: [AtomicU64; FC_SEEDS] = [KC_ZERO; FC_SEEDS];
static FC_TID: [[AtomicU32; 2]; FC_SEEDS] = [const { [AtomicU32::new(0), AtomicU32::new(0)] }; FC_SEEDS];
static FC_H: [AtomicU64; FC_SEEDS * 2 * FC_BUCKETS] = [KC_ZERO; FC_SEEDS * 2 * FC_BUCKETS];
static FC_CALLS: AtomicU64 = AtomicU64::new(0);
static FC_INSTALLED: AtomicBool = AtomicBool::new(false);
static FC_STATE: AtomicU32 = AtomicU32::new(0);
// MobaMode::tick에서 본 World를 스레드별 보관 → fight_check(World 인자 없음)에서 seed/tick 취득
thread_local! { static CUR_WORLD: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

static MOBATICK_INSTALLED: AtomicBool = AtomicBool::new(false);
static MAIN_TID: AtomicU32 = AtomicU32::new(0);
static TICK_N: AtomicU64 = AtomicU64::new(0);
static LAST_DUMP_MS: AtomicU64 = AtomicU64::new(0);
static START_MS: AtomicU64 = AtomicU64::new(0);

// ── [Part 2] (seed,fp) 파티션 → **정확 sim_tick**별 첫 관측 (champ_hash, first_tid, kb, kr) ──
//   ★키=정확 sim_tick(32배수). 두 스레드가 동일 tick에서만 대조 → 동일 sim이면 위치·스코어 비트동일=발산0(정렬-robust).
type Sample = (u64 /*champ_hash*/, u32 /*first_tid*/, u64 /*kb*/, u64 /*kr*/);
static DIV: Mutex<Option<HashMap<(u64, u64), HashMap<u64, Sample>>>> = Mutex::new(None);
// seed → (first_tid, multi(2+스레드), score_div, hash_div, spec(메인포함), first_div_tick)
static DIVSEED: Mutex<Option<HashMap<u64, (u32, bool, bool, bool, bool, u64)>>> = Mutex::new(None);

// ── [Part 1] per-thread sim_tick 타임라인 (버킷 변화 시 1샘플) ──
// (qpc_ms, tid, is_main, seed, sim_tick, kb, kr)
#[derive(Clone, Copy)]
struct TL { ms: u64, tid: u32, main: u8, seed: u64, tick: u64, kb: u64, kr: u64 }
static TIMELINE: Mutex<Option<Vec<TL>>> = Mutex::new(None);

// fp 캐시
#[allow(clippy::declare_interior_mutable_const)]
const KC_ZERO: AtomicU64 = AtomicU64::new(0);
static FP_ADDR: [AtomicU64; KC_SLOTS] = [KC_ZERO; KC_SLOTS];
static FP_CHK: [AtomicU64; KC_SLOTS] = [KC_ZERO; KC_SLOTS];
static FP_VAL: [AtomicU64; KC_SLOTS] = [KC_ZERO; KC_SLOTS];
// 버킷 변화감지 캐시 (world slot → 마지막 처리 버킷)
static WB_ADDR: [AtomicU64; KC_SLOTS] = [KC_ZERO; KC_SLOTS];
static WB_BUCKET: [AtomicU64; KC_SLOTS] = [KC_ZERO; KC_SLOTS];

#[inline] fn splitmix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
#[inline] fn rotl(x: u64, r: u32) -> u64 { x.rotate_left(r) }
fn now_ms() -> u64 {
    unsafe {
        let mut c: i64 = 0; let mut f: i64 = 0;
        QueryPerformanceCounter(&mut c); QueryPerformanceFrequency(&mut f);
        if f == 0 { 0 } else { (c as i128 * 1000 / f as i128) as u64 }
    }
}

// ── 챔피언 구성 지문(fp) — 같은 seed의 다른 경기(Bo시리즈) 분리 (seedprobe world_fingerprint 이식) ──
unsafe fn world_fingerprint(w: usize) -> Option<u64> {
    let pp = safe_read_u64(w + W_PLAYER_DENSE)? as usize;
    let pn = safe_read_u64(w + W_PLAYER_DENSE + 8)? as usize;
    if pp < 0x10000 || pn == 0 || pn > 16 { return None; }
    let sp = safe_read_u64(w + W_CHAMP_SLOTS)? as usize;
    let sn = safe_read_u64(w + W_CHAMP_SLOTS + 8)? as usize;
    let cp = safe_read_u64(w + W_CHAMP_DENSE)? as usize;
    let cn = safe_read_u64(w + W_CHAMP_DENSE + 8)? as usize;
    if sp < 0x10000 || cp < 0x10000 || cn == 0 || cn > 4096 { return None; }
    let mut pairs: [u64; 16] = [0; 16];
    let mut np = 0usize;
    for i in 0..pn {
        let p = pp + i * PLAYER_STRIDE;
        let Some(team) = safe_read_u64(p + P_TEAM) else { continue };
        if team > 1 { continue; }
        if safe_read_u64(p + P_CHAMP_TAG).unwrap_or(0) == 0 { continue; }
        let Some(key) = safe_read_u64(p + P_CHAMP_KEY) else { continue };
        let idx = (key & 0xffff_ffff) as usize;
        if idx >= sn { continue; }
        let Some(dense) = safe_read_u64(sp + idx * 0x10 + 8) else { continue };
        if dense as usize >= cn { continue; }
        let ent = cp + dense as usize * CHAMP_STRIDE;
        if safe_read_i32(ent + ENTITY_KIND_OFF) != Some(CHAMP_KIND as i32) { continue; }
        let Some(nl) = safe_read_u64(ent + E_NAME_LEN) else { continue };
        let nl = (nl as usize).min(32);
        let Some(nptr) = safe_read_u64(ent + E_NAME_PTR) else { continue };
        if (nptr as usize) < 0x10000 || nl == 0 { continue; }
        let mut nb = [0u8; 32];
        if !safe_copy(nb.as_mut_ptr(), nptr as usize as *const u8, nl) { continue; }
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for &b in &nb[..nl] { h = (h ^ b as u64).wrapping_mul(0x1000_0000_01b3); }
        if np < 16 { pairs[np] = (team << 62) | (h >> 2); np += 1; }
    }
    if np < 4 { return None; }
    pairs[..np].sort_unstable();
    let mut h = 0x9e37_79b9_7f4a_7c15u64;
    for &v in &pairs[..np] { h = splitmix64(h ^ v); }
    Some(h | 1)
}
unsafe fn fp_for_world(w: usize) -> Option<u64> {
    let chk = safe_read_u64(w + W_CHAMP_DENSE).unwrap_or(0);
    if chk < 0x10000 { return None; }
    let slot = (w >> 4) & (KC_SLOTS - 1);
    if FP_ADDR[slot].load(Ordering::Relaxed) == w as u64 && FP_CHK[slot].load(Ordering::Relaxed) == chk {
        let v = FP_VAL[slot].load(Ordering::Relaxed);
        if v != 0 { return Some(v); }
    }
    let fp = world_fingerprint(w)?;
    FP_ADDR[slot].store(w as u64, Ordering::Relaxed);
    FP_CHK[slot].store(chk, Ordering::Relaxed);
    FP_VAL[slot].store(fp, Ordering::Relaxed);
    Some(fp)
}

// ── 챔피언 전체 상태 해시(순서무관): 발산 감지용. name_hash ^ hp_cur ^ pos_x ^ pos_y 를 splitmix 후 합산 ──
unsafe fn champ_state_hash(w: usize) -> Option<u64> {
    let cp = safe_read_u64(w + W_CHAMP_DENSE)? as usize;
    let cn = safe_read_u64(w + W_CHAMP_DENSE + 8)? as usize;
    if cp < 0x10000 || cn == 0 || cn > 4096 { return None; }
    let mut acc: u64 = 0;
    let mut cnt = 0u32;
    for i in 0..cn {
        let ent = cp + i * CHAMP_STRIDE;
        if safe_read_i32(ent + ENTITY_KIND_OFF) != Some(CHAMP_KIND as i32) { continue; }
        let hp = safe_read_u64(ent + E_HP_CUR).unwrap_or(0);
        let px = safe_read_u64(ent + E_POS_X).unwrap_or(0);
        let py = safe_read_u64(ent + E_POS_Y).unwrap_or(0);
        // 이름해시(정체성): 위치가 매틱 바뀌므로 정체성 고정
        let mut nh = 0xcbf2_9ce4_8422_2325u64;
        if let Some(nl) = safe_read_u64(ent + E_NAME_LEN) {
            let nl = (nl as usize).min(32);
            if let Some(nptr) = safe_read_u64(ent + E_NAME_PTR) {
                if (nptr as usize) >= 0x10000 && nl > 0 {
                    let mut nb = [0u8; 32];
                    if safe_copy(nb.as_mut_ptr(), nptr as usize as *const u8, nl) {
                        for &b in &nb[..nl] { nh = (nh ^ b as u64).wrapping_mul(0x1000_0000_01b3); }
                    }
                }
            }
        }
        let mix = splitmix64(nh ^ rotl(hp, 17) ^ rotl(px, 31) ^ rotl(py, 47));
        acc = acc.wrapping_add(mix);
        cnt += 1;
        if cnt > 64 { break; }
    }
    if cnt == 0 { return None; }
    Some(splitmix64(acc ^ (cnt as u64)) | 1)
}

// ── MobaMode::tick 관측: 버킷 변화 시 상태해시/스코어 대조 + 타임라인 샘플 ──
unsafe fn track(w: usize) {
    let seed = safe_read_u64(w + SEED_OFF).unwrap_or(0);
    if seed == 0 { return; }
    let sim_tick = safe_read_u64(w + SIM_TICK_OFF).unwrap_or(0);
    if sim_tick > (1u64 << 40) { return; }
    // ★정확 tick 배수에서만 샘플 (두 스레드가 동일 tick 비교 = 정렬-robust). 배수를 건너뛴 스레드는 그 tick 미비교(오탐 없음).
    if sim_tick % SAMPLE_STRIDE != 0 { return; }
    // 같은 world가 같은 정확 tick 중복 처리 방지(부하 억제)
    let slot = (w >> 4) & (KC_SLOTS - 1);
    if WB_ADDR[slot].load(Ordering::Relaxed) == w as u64 && WB_BUCKET[slot].load(Ordering::Relaxed) == sim_tick {
        return;
    }
    let kb = safe_read_u64(w + KILLS_BLUE_OFF).unwrap_or(u64::MAX);
    let kr = safe_read_u64(w + KILLS_RED_OFF).unwrap_or(u64::MAX);
    if kb > 1000 || kr > 1000 { return; } // 오독 방어
    let Some(fp) = fp_for_world(w) else { return };
    let Some(chash) = champ_state_hash(w) else { return };
    let cur_tid = GetCurrentThreadId();
    let is_main = (cur_tid == MAIN_TID.load(Ordering::Relaxed)) as u8;
    let ms = now_ms().saturating_sub(START_MS.load(Ordering::Relaxed));

    // [Part 2] 버킷별 첫 관측 대조 → 발산
    {
        let mut g = DIV.lock().unwrap_or_else(|e| e.into_inner());
        let map = g.get_or_insert_with(HashMap::new);
        let m = map.entry((seed, fp)).or_default();
        let mut score_div = false;
        let mut hash_div = false;
        if let Some(&(h0, t0, b0, r0)) = m.get(&sim_tick) {
            if t0 != cur_tid {   // ★다른 스레드가 **정확히 같은 sim_tick**에서 관측 → 동일 sim이면 비트동일이어야
                if h0 != chash { hash_div = true; }
                if b0 != kb || r0 != kr { score_div = true; }
            }
        } else if m.len() < MAX_SAMPLES_PER_MATCH {
            m.insert(sim_tick, (chash, cur_tid, kb, kr));
        }
        drop(g);
        let mut ds = DIVSEED.lock().unwrap_or_else(|e| e.into_inner());
        let e = ds.get_or_insert_with(HashMap::new).entry(seed).or_insert((cur_tid, false, false, false, false, 0));
        if e.0 != cur_tid { e.1 = true; }
        if score_div { if !e.2 { e.5 = sim_tick; } e.2 = true; }
        if hash_div { if !e.3 && !e.2 { e.5 = sim_tick; } e.3 = true; }
        if is_main == 1 { e.4 = true; }
    }
    // [Part 1] 타임라인 샘플
    {
        let mut tg = TIMELINE.lock().unwrap_or_else(|e| e.into_inner());
        let v = tg.get_or_insert_with(Vec::new);
        if v.len() >= TIMELINE_CAP { let drop_n = TIMELINE_CAP / 8; v.drain(0..drop_n); }
        v.push(TL { ms, tid: cur_tid, main: is_main, seed, tick: sim_tick, kb, kr });
    }
    WB_ADDR[slot].store(w as u64, Ordering::Relaxed);
    WB_BUCKET[slot].store(sim_tick, Ordering::Relaxed);
}

unsafe extern "C" fn cap_mobatick(saved: *mut u64, _rsp: usize) -> u64 {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved.is_null() { return; }
        TICK_N.fetch_add(1, Ordering::Relaxed);
        let w = unsafe { *saved.add(0) } as usize; // rcx = World/provider
        if w > 0x10000 && w < (1usize << 47) {
            CUR_WORLD.with(|c| c.set(w));   // ★fight_check 훅용 World 전달
            unsafe { track(w); }
        }
    }));
    0
}

// ★[07-29] fight_check 입력 관측: 인자 중 챔피언 엔티티를 찾아 name+hp+pos를 (seed,tick버킷)별 누적 해싱.
unsafe extern "C" fn cap_fc(saved: *mut u64, _rsp: usize) -> u64 {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved.is_null() { return; }
        FC_CALLS.fetch_add(1, Ordering::Relaxed);
        let w = CUR_WORLD.with(|c| c.get());
        if w == 0 { return; }
        let Some(seed) = (unsafe { safe_read_u64(w + SEED_OFF) }) else { return };
        if seed == 0 { return; }
        let Some(tick) = (unsafe { safe_read_u64(w + SIM_TICK_OFF) }) else { return };
        if tick > (1u64 << 40) { return; }
        // 인자 4개 중 챔피언 엔티티(kind==0xd) 탐색 → 값 기반 입력 해시
        let mut ih: u64 = 0;
        let mut found = false;
        for a in 0..4usize {
            let p = unsafe { *saved.add(match a { 0 => 0, 1 => 1, 2 => 2, _ => 3 }) } as usize;
            if p < 0x10000 || p >= (1usize << 47) { continue; }
            if unsafe { safe_read_i32(p + ENTITY_KIND_OFF) } != Some(CHAMP_KIND as i32) { continue; }
            let hp = unsafe { safe_read_u64(p + E_HP_CUR) }.unwrap_or(0);
            let px = unsafe { safe_read_u64(p + E_POS_X) }.unwrap_or(0);
            let py = unsafe { safe_read_u64(p + E_POS_Y) }.unwrap_or(0);
            let mut nh = 0xcbf2_9ce4_8422_2325u64;
            if let (Some(nl), Some(np)) = (unsafe { safe_read_u64(p + E_NAME_LEN) }, unsafe { safe_read_u64(p + E_NAME_PTR) }) {
                let nl = (nl as usize).min(32);
                if (np as usize) >= 0x10000 && nl > 0 {
                    let mut nb = [0u8; 32];
                    if unsafe { safe_copy(nb.as_mut_ptr(), np as usize as *const u8, nl) } {
                        for &b in &nb[..nl] { nh = (nh ^ b as u64).wrapping_mul(0x1000_0000_01b3); }
                    }
                }
            }
            ih ^= splitmix64(nh ^ hp.rotate_left(13) ^ px.rotate_left(29) ^ py.rotate_left(43) ^ ((a as u64) << 60));
            found = true;
        }
        if !found { return; }
        // seed 슬롯 확보(선형 8칸)
        let start = ((seed >> 3) as usize) & (FC_SEEDS - 1);
        let mut slot = usize::MAX;
        for i in 0..8 {
            let s = (start + i) & (FC_SEEDS - 1);
            let cur = FC_SEED[s].load(Ordering::Relaxed);
            if cur == seed { slot = s; break; }
            if cur == 0 && FC_SEED[s].compare_exchange(0, seed, Ordering::Relaxed, Ordering::Relaxed).is_ok() { slot = s; break; }
        }
        if slot == usize::MAX { return; }
        let tid = unsafe { GetCurrentThreadId() };
        let mut ts = usize::MAX;
        for t in 0..2usize {
            let cur = FC_TID[slot][t].load(Ordering::Relaxed);
            if cur == tid { ts = t; break; }
            if cur == 0 && FC_TID[slot][t].compare_exchange(0, tid, Ordering::Relaxed, Ordering::Relaxed).is_ok() { ts = t; break; }
        }
        if ts == usize::MAX { return; }
        let b = ((tick >> 3) as usize).min(FC_BUCKETS - 1);
        FC_H[(slot * 2 + ts) * FC_BUCKETS + b].fetch_add(splitmix64(ih ^ tick.rotate_left(7) ^ 1), Ordering::Relaxed);
    }));
    0
}

// ★[07-29] 해저드 해시맵 해시코어 진입 관측: rdx=맵 포인터 → (k0,k1) 시드 수집.
//   saved 레이아웃(스텁 push 순서 rcx가 마지막) = [0]=rcx [1]=rdx [2]=r8 [3]=r9 …
unsafe extern "C" fn cap_haz(saved: *mut u64, _rsp: usize) -> u64 {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved.is_null() { return; }
        HAZ_CALLS.fetch_add(1, Ordering::Relaxed);
        let map = unsafe { *saved.add(1) } as usize;   // rdx = map
        if map < 0x10000 || map >= (1usize << 47) { return; }
        let (Some(k0), Some(k1)) = (unsafe { safe_read_u64(map + 0x20) }, unsafe { safe_read_u64(map + 0x28) }) else { return };
        // fast path: 직전과 같은 시드면 스캔 생략
        if let Some(i) = HAZ_LAST.with(|c| { let (a, b, i) = c.get(); if a == k0 && b == k1 && i != usize::MAX { Some(i) } else { None } }) {
            HAZ_CNT[i].fetch_add(1, Ordering::Relaxed);
            return;
        }
        let tid = unsafe { GetCurrentThreadId() };
        for i in 0..HAZ_SLOTS {
            let t = HAZ_TID[i].load(Ordering::Relaxed);
            if t == tid && HAZ_K0[i].load(Ordering::Relaxed) == k0 && HAZ_K1[i].load(Ordering::Relaxed) == k1 {
                HAZ_CNT[i].fetch_add(1, Ordering::Relaxed);
                HAZ_LAST.with(|c| c.set((k0, k1, i)));
                return;
            }
            if t == 0 && HAZ_TID[i].compare_exchange(0, tid, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
                HAZ_K0[i].store(k0, Ordering::Relaxed);
                HAZ_K1[i].store(k1, Ordering::Relaxed);
                HAZ_CNT[i].store(1, Ordering::Relaxed);
                HAZ_LAST.with(|c| c.set((k0, k1, i)));
                return;
            }
        }
    }));
    0
}

fn install_hook() {
    if MOBATICK_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    let ok = unsafe { install_stub_generic(MOBATICK_RVA, 12, cap_mobatick as usize, &MOBATICK_PROLOGUE) };
    INSTALL_STATE.store(if ok { 1 } else { 2 }, Ordering::Relaxed);
    if !HAZ_INSTALLED.swap(true, Ordering::Relaxed) {
        let ok2 = unsafe { install_stub_generic(HAZ_RVA, 12, cap_haz as usize, &HAZ_PROLOGUE) };
        HAZ_STATE.store(if ok2 { 1 } else { 2 }, Ordering::Relaxed);
    }
    if !FC_INSTALLED.swap(true, Ordering::Relaxed) {
        let ok3 = unsafe { install_stub_generic(FC_RVA, 12, cap_fc as usize, &FC_PROLOGUE) };
        FC_STATE.store(if ok3 { 1 } else { 2 }, Ordering::Relaxed);
    }
}
static INSTALL_STATE: AtomicU32 = AtomicU32::new(0);

fn dump() {
    let now = now_ms();
    if now.saturating_sub(LAST_DUMP_MS.load(Ordering::Relaxed)) < 1000 { return; }
    LAST_DUMP_MS.store(now, Ordering::Relaxed);
    let install = match INSTALL_STATE.load(Ordering::Relaxed) { 1 => "OK", 2 => "프롤로그 mismatch(미설치)", _ => "미시도" };

    // ── [Part 2] 발산 요약 ──
    let ds = DIVSEED.lock().unwrap_or_else(|e| e.into_inner());
    let (total, multi, spec, score_hit, hash_hit, samples) = if let Some(m) = ds.as_ref() {
        let total = m.len();
        let multi = m.values().filter(|(_, mu, ..)| *mu).count();
        let spec = m.values().filter(|(_, mu, _, _, sm, _)| *mu && *sm).count();
        let score_hit = m.values().filter(|(_, mu, sd, ..)| *mu && *sd).count();
        let hash_hit = m.values().filter(|(_, mu, _, hd, _, _)| *mu && *hd).count();
        let mut s: Vec<(u64, u64)> = m.iter()
            .filter(|(_, (_, mu, sd, hd, _, _))| *mu && (*sd || *hd))
            .map(|(k, v)| (*k, v.5)).take(50).collect();
        s.sort_unstable();
        (total, multi, spec, score_hit, hash_hit, s)
    } else { (0, 0, 0, 0, 0, Vec::new()) };
    let srate = if multi > 0 { score_hit as f64 * 100.0 / multi as f64 } else { 0.0 };
    let hrate = if multi > 0 { hash_hit as f64 * 100.0 / multi as f64 } else { 0.0 };
    let mut out = format!(
        "# tfm2_viewresult_probe v2(정확tick 대조) — MobaMode::tick {:#x} 후킹 = {}  (read-only, 게임 무변경)\n\
         # ★모드조합 확인: serpen 반드시 off. ai_adjust ON/OFF 나눠 측정 권장.\n\
         # MAIN_TID={}  틱관측={}  경과={}ms\n\n\
         ## [Part 2] full-outcome 발산 (같은 (seed,fp)를 2+스레드가 sim = 배경 vs 관전)\n\
         # 관측 경기(distinct seed) = {}\n\
         # 2+스레드 sim(=비교가능) = {}  (그중 관전[메인tid] 포함 = {})\n\
         # ★스코어(ed50/ed58) 발산 = {}  → 발산율 {:.1}%\n\
         # ★챔프상태해시(HP/pos/name) 발산 = {}  → 발산율 {:.1}%\n\n\
         [발산 경기 seed / 첫발산 sim_tick (최대 50)]\n",
        MOBATICK_RVA, install, MAIN_TID.load(Ordering::Relaxed), TICK_N.load(Ordering::Relaxed),
        now.saturating_sub(START_MS.load(Ordering::Relaxed)),
        total, multi, spec, score_hit, srate, hash_hit, hrate);
    for (s, t) in &samples { out.push_str(&format!("  {:#018x}  @tick {}\n", s, t)); }
    drop(ds);
    write_log("viewresult_probe.txt", &out);

    // ── [Part 1] per-thread sim_tick 타임라인 (즉시보기 눌렀을 때 폭주=A / 멈춤=B 판별) ──
    let tg = TIMELINE.lock().unwrap_or_else(|e| e.into_inner());
    let mut tl = String::from(
        "# tfm2_viewresult_probe — sim_tick 타임라인 (32tick마다 1샘플)\n\
         # 열: 경과ms | tid | M=관전(메인) | seed(하위) | sim_tick | 스코어(blue:red)\n\
         # ★즉시보기 누른 시각(ms) 알려주면, 그 경기(seed) 행에서 sim_tick이\n\
         #   급가속→끝점 도달=(A)페이싱 / 정지=(B)저장로드 를 판별합니다.\n\n");
    if let Some(v) = tg.as_ref() {
        // 최근 3000샘플만
        let start = v.len().saturating_sub(3000);
        for s in &v[start..] {
            tl.push_str(&format!("{:>8} | {:>5} | {} | {:#08x} | {:>7} | {}:{}\n",
                s.ms, s.tid, if s.main == 1 { "M" } else { " " },
                (s.seed & 0xffff_ffff), s.tick,
                if s.kb == u64::MAX { -1i64 } else { s.kb as i64 },
                if s.kr == u64::MAX { -1i64 } else { s.kr as i64 }));
        }
    }
    drop(tg);
    write_log("viewresult_timeline.txt", &tl);

    // ── [07-29] 해저드 해시맵 시드 판정 ──
    let hinstall = match HAZ_STATE.load(Ordering::Relaxed) { 1 => "OK", 2 => "프롤로그 mismatch(미설치)", _ => "미시도" };
    let mut rows: Vec<(u32, u64, u64, u64)> = Vec::new();
    for i in 0..HAZ_SLOTS {
        let t = HAZ_TID[i].load(Ordering::Relaxed);
        if t == 0 { continue; }
        let c = HAZ_CNT[i].load(Ordering::Relaxed);
        if c == 0 { continue; }
        rows.push((t, HAZ_K0[i].load(Ordering::Relaxed), HAZ_K1[i].load(Ordering::Relaxed), c));
    }
    rows.sort_unstable_by_key(|r| (r.0, r.1, r.2));
    // 서로 다른 (k0,k1) 종류 수 / 스레드 종류 수
    let mut seeds: Vec<(u64, u64)> = rows.iter().map(|r| (r.1, r.2)).collect();
    seeds.sort_unstable(); seeds.dedup();
    let mut tids: Vec<u32> = rows.iter().map(|r| r.0).collect();
    tids.sort_unstable(); tids.dedup();
    // ★판정 정정: "시드 종류 수"가 아니라 **같은 시드가 여러 스레드에 나타나는가**가 핵심.
    //   스레드별 OS 난수라면 64비트 두 워드가 스레드 간에 겹칠 확률은 사실상 0 ⟹ 공유되면 **결정적 파생**.
    let mut shared = 0usize;
    for sd in &seeds {
        let mut ts: Vec<u32> = rows.iter().filter(|r| r.1 == sd.0 && r.2 == sd.1).map(|r| r.0).collect();
        ts.sort_unstable(); ts.dedup();
        if ts.len() >= 2 { shared += 1; }
    }
    let verdict = if seeds.is_empty() {
        "(관측 없음 — 경기 관전 중에만 발화합니다)".to_string()
    } else if shared > 0 {
        format!("★★**결정적 파생 시드** — {}종 중 {}종이 2개 이상 스레드에서 동일하게 관측됨.\n\
                 스레드별 난수라면 128비트가 겹칠 확률≈0 ⟹ 시드는 데이터에서 유도되는 값이고,\n\
                 같은 논리적 맵이면 어느 스레드서든 순회 순서가 같다 ⟹ **이 해시맵은 비결정원이 아님(배제)**.", seeds.len(), shared)
    } else {
        format!("★★시드 {}종이 **스레드 간 전혀 공유되지 않음** ⟹ 스레드/인스턴스별 난수 시드 = 순회 순서 비결정 확정.\n\
                 배경 sim vs 관전 sim 발산의 엔진측 근원 유력.", seeds.len())
    };
    let mut hz = format!(
        "# 해저드/위협 해시맵 해시코어 {:#x} 진입 관측 = {}   (read-only)\n\
         # 목적: 해시 시드(map+0x20=k0 / map+0x28=k1)가 인스턴스·스레드마다 다른가?\n\
         #   AI가 이 맵을 '전수 순회 → 조건통과 첫 엔트리 채택'으로 소비(position_eval 0x22dd9a0 등)하므로,\n\
         #   시드가 다르면 같은 키집합이라도 순회 순서가 재배열 → 통과 엔트리 2개 이상일 때 판정이 갈림.\n\n\
         # 총 호출 = {}   서로 다른 (k0,k1) = {}   관측 스레드 = {}\n\n\
         {}\n\n[슬롯: tid | k0 | k1 | 호출수]\n",
        HAZ_RVA, hinstall, HAZ_CALLS.load(Ordering::Relaxed), seeds.len(), tids.len(), verdict);
    for (t, k0, k1, c) in rows.iter().take(48) {
        hz.push_str(&format!("  {:>6} | {:#018x} | {:#018x} | {}\n", t, k0, k1, c));
    }
    write_log("hazmap_seed.txt", &hz);

    // ── [07-29] fight_check 입력 발산 시점 ──
    let finstall = match FC_STATE.load(Ordering::Relaxed) { 1 => "OK", 2 => "프롤로그 mismatch(미설치)", _ => "미시도" };
    let mut fc = format!(
        "# fight_check {:#x} 입력 관측 = {}   총 호출 = {}\n\
         # 두 sim(배경/관전)이 같은 경기를 돌 때, **이 함수에 들어가는 입력**(챔프 name+hp+pos)이 언제 갈리는지.\n\
         # ★해석: 입력 최초 불일치가 상태 발산보다 **빠르면** fight_check는 하류(결과) → 다른 원인.\n\
         #        입력이 상태 발산 시점까지 **일치**하면 '같은 입력 → 다른 결과'(TLS 캐시 오염) 가설 유지 → 리턴훅 2단계 필요.\n\n",
        FC_RVA, finstall, FC_CALLS.load(Ordering::Relaxed));
    let mut any_fc = false;
    for s in 0..FC_SEEDS {
        let seed = FC_SEED[s].load(Ordering::Relaxed);
        if seed == 0 || FC_TID[s][1].load(Ordering::Relaxed) == 0 { continue; }
        let h = |ts: usize, b: usize| FC_H[(s * 2 + ts) * FC_BUCKETS + b].load(Ordering::Relaxed);
        let first = (0..FC_BUCKETS).find(|&b| { let (a, c) = (h(0, b), h(1, b)); a != 0 && c != 0 && a != c });
        any_fc = true;
        fc.push_str(&format!("== seed {:#018x}  tid {}/{}  → 입력 최초 불일치 = {}\n", seed,
            FC_TID[s][0].load(Ordering::Relaxed), FC_TID[s][1].load(Ordering::Relaxed),
            match first { Some(b) => format!("tick {}~{}", b << 3, ((b + 1) << 3) - 1), None => "없음(입력 전 구간 일치)".into() }));
    }
    if !any_fc { fc.push_str("(2-스레드 관측 경기 없음)\n"); }
    write_log("fightcheck_input.txt", &fc);
}

// ───────────────────────── asm 스텁 트램폴린 (entry-observe, serpen 검증본) ─────────────────────────
unsafe fn install_stub_generic(rva: usize, orig_len: usize, cap_fn: usize, prologue: &[u8]) -> bool {
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { return false; }
    let fn_addr = base + rva;
    for i in 0..prologue.len() { if *((fn_addr + i) as *const u8) != prologue[i] { return false; } }
    let stub = VirtualAlloc(0, 256, 0x3000, 0x40);
    if stub == 0 { return false; }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x41, 0x54, 0x56, 0x57, 0x53, 0x41, 0x53, 0x41, 0x52, 0x41, 0x51, 0x41, 0x50, 0x52, 0x51]);
    s.extend_from_slice(&[0x48, 0x89, 0xe1]);       // mov rcx, rsp
    s.extend_from_slice(&[0x48, 0x89, 0xe3]);       // mov rbx, rsp
    s.extend_from_slice(&[0x48, 0x83, 0xe4, 0xf0]); // and rsp, -16
    s.extend_from_slice(&[0x48, 0x83, 0xec, 0x20]); // sub rsp, 0x20
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff, 0xd0]);             // call rax
    s.extend_from_slice(&[0x48, 0x89, 0xdc]);       // mov rsp, rbx
    s.extend_from_slice(&[0x59, 0x5a, 0x41, 0x58, 0x41, 0x59, 0x41, 0x5a, 0x41, 0x5b, 0x5b, 0x5f, 0x5e, 0x41, 0x5c]);
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;
    let mut old = 0u32;
    if VirtualProtect(fn_addr, orig_len, 0x40, &mut old) == 0 { return false; }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    true
}

// ───────────────────────── SEH 안전 r/w (serpen 검증본) ─────────────────────────
static mut SEH: [u64; 8] = [0u64; 8];
static SEH_INSTALLED: AtomicBool = AtomicBool::new(false);
static SEH_BUSY: AtomicBool = AtomicBool::new(false);
#[repr(C)]
struct ExceptionRecord { code: u32, flags: u32, rec: usize, addr: usize, nparams: u32, _p: u32, params: [usize; 15] }
#[repr(C)]
struct ExceptionPointers { rec: *mut ExceptionRecord, ctx: *mut core::ffi::c_void }
type VehHandler = extern "system" fn(*mut ExceptionPointers) -> i32;
extern "system" fn seh_veh(p: *mut ExceptionPointers) -> i32 {
    unsafe {
        if p.is_null() { return 0; }
        let rec = (*p).rec;
        if rec.is_null() || (*rec).code != 0xC0000005 { return 0; }
        let g = core::ptr::addr_of!(SEH) as *const u64;
        if *g.add(0) == 0 { return 0; }
        if *g.add(1) != GetCurrentThreadId() as u64 { return 0; }
        let ctx = (*p).ctx as usize;
        if ctx == 0 { return 0; }
        let rip = *((ctx + 0xF8) as *const u64);
        if rip < *g.add(5) || rip >= *g.add(6) { return 0; }
        *((ctx + 0xF8) as *mut u64) = *g.add(2);
        *((ctx + 0x98) as *mut u64) = *g.add(3);
        *((ctx + 0xA0) as *mut u64) = *g.add(4);
        let gm = core::ptr::addr_of_mut!(SEH) as *mut u64;
        *gm.add(7) += 1;
        -1
    }
}
fn seh_install() { if SEH_INSTALLED.swap(true, Ordering::Relaxed) { return; } unsafe { AddVectoredExceptionHandler(1, seh_veh); } }
#[inline(never)]
unsafe fn safe_copy(dst: *mut u8, src: *const u8, len: usize) -> bool {
    if !SEH_INSTALLED.load(Ordering::Relaxed) { return false; }
    while SEH_BUSY.swap(true, Ordering::Acquire) { core::hint::spin_loop(); }
    let g = core::ptr::addr_of_mut!(SEH) as *mut u64;
    *g.add(1) = GetCurrentThreadId() as u64;
    let ok: u64;
    core::arch::asm!(
        "lea rax, [rip + 200f]", "mov [{g} + 40], rax",
        "lea rax, [rip + 201f]", "mov [{g} + 48], rax",
        "lea rax, [rip + 202f]", "mov [{g} + 16], rax",
        "mov [{g} + 24], rsp", "mov [{g} + 32], rbp",
        "mov qword ptr [{g} + 0], 1",
        "cld",
        "200:", "rep movsb",
        "201:", "mov {ok}, 1", "jmp 203f",
        "202:", "mov {ok}, 0",
        "203:", "mov qword ptr [{g} + 0], 0",
        g = in(reg) g, ok = out(reg) ok,
        inout("rcx") len => _, inout("rdi") dst => _, inout("rsi") src => _, out("rax") _,
    );
    SEH_BUSY.store(false, Ordering::Release);
    ok != 0
}
unsafe fn safe_read_u64(addr: usize) -> Option<u64> {
    let mut b = [0u8; 8];
    if safe_copy(b.as_mut_ptr(), addr as *const u8, 8) { Some(u64::from_le_bytes(b)) } else { None }
}
unsafe fn safe_read_i32(addr: usize) -> Option<i32> {
    let mut b = [0u8; 4];
    if safe_copy(b.as_mut_ptr(), addr as *const u8, 4) { Some(i32::from_le_bytes(b)) } else { None }
}

// ───────────────────────── 파일 ─────────────────────────
fn dll_path() -> Option<PathBuf> {
    unsafe {
        let addr = dll_path as *const () as usize;
        let mut h: isize = 0;
        if GetModuleHandleExW(0x4 | 0x2, addr as *const u16, &mut h) == 0 || h == 0 { return None; }
        let mut buf = [0u16; 4096];
        let n = GetModuleFileNameW(h, buf.as_mut_ptr(), buf.len() as u32);
        if n == 0 { return None; }
        Some(PathBuf::from(String::from_utf16_lossy(&buf[..n as usize])))
    }
}
fn mod_dir() -> Option<PathBuf> { dll_path()?.parent().map(|p| p.to_path_buf()) }
fn write_log(name: &str, content: &str) {
    if let Some(p) = mod_dir().map(|d| d.join(name)) { let _ = fs::write(p, content); }
}

// ───────────────────────── WinAPI ─────────────────────────
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleHandleExW(f: u32, name: *const u16, h: *mut isize) -> i32;
    fn GetModuleFileNameW(h: isize, buf: *mut u16, n: u32) -> u32;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old_protect: *mut u32) -> i32;
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, size: usize) -> i32;
    fn GetCurrentProcess() -> usize;
    fn GetCurrentThreadId() -> u32;
    fn AddVectoredExceptionHandler(first: u32, handler: VehHandler) -> usize;
    fn QueryPerformanceCounter(c: *mut i64) -> i32;
    fn QueryPerformanceFrequency(f: *mut i64) -> i32;
}

// ───────────────────────── 로더 ABI ─────────────────────────
static SETUP_DONE: AtomicBool = AtomicBool::new(false);
fn setup() {
    if SETUP_DONE.swap(true, Ordering::Relaxed) { return; }
    MAIN_TID.store(unsafe { GetCurrentThreadId() }, Ordering::Relaxed);
    START_MS.store(now_ms(), Ordering::Relaxed);
    seh_install();
    install_hook();
}
struct Ext;
impl ModExtension for Ext {
    fn on_init(&self, _s: &mut Scene, _u: &mut GameUI, _a: &mut Assets) { setup(); }
    fn post_update(&self, _s: &mut Scene, _u: &mut GameUI, _a: &mut Assets, _dt: f32) {
        setup();
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(dump));
    }
}
struct SExt;
impl ModServerExtension for SExt {
    fn on_server_start(&self, _c: &mut ServerModContext) { setup(); }
}
fn init(_ctx: &GameCtx) -> ModRegistration {
    setup();
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(Ext);
    reg.set_server_extension(SExt);
    reg
}
declare_mod!(init);
