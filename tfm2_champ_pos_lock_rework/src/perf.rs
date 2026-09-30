//! perf — 구간별 소요시간 계측(cfg `perf_log=1` 일 때만 · 기본 OFF). ★09-28 렉 제보(밴픽·인게임 순간 멈춤) 실측용.
//! 구간마다 (최대 µs, 합 µs, 횟수)를 원자값으로 누적 → 600프레임마다 champ_pos_lock_perf.txt 에 한 줄.
//! post_update 한 프레임이 SPIKE_US 를 넘으면 그 프레임의 구간별 값을 즉시 한 줄(최대 200줄).
//! 워커 스레드(드래프트 훅·detour)에서도 호출되므로 락 없이 원자값만 쓴다(파일 쓰기는 메인 스레드 flush 에서만).
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::Instant;

pub const POST: usize = 0;
pub const ROSTER: usize = 1;
pub const SAVE: usize = 2;
pub const MASKS: usize = 3;
pub const OPTION: usize = 4;
pub const UI_BLOCK: usize = 5;
pub const I18N: usize = 6;
pub const SCORE: usize = 7;
pub const DECIDE: usize = 8;
pub const POS_MASK: usize = 9;
pub const AI_SWAP: usize = 10;
pub const HOOKS: usize = 11;
const N: usize = 12;
const NAMES: [&str; N] = ["post", "roster", "save", "masks", "option", "ui_block", "i18n", "score_pick", "decide", "pos_mask", "ai_swap", "hooks"];
const SPIKE_US: u64 = 4000;

static MAX: [AtomicU64; N] = [const { AtomicU64::new(0) }; N];
static SUM: [AtomicU64; N] = [const { AtomicU64::new(0) }; N];
static CNT: [AtomicU64; N] = [const { AtomicU64::new(0) }; N];
/// 이번 프레임 구간값(메인 스레드 구간만 의미 있음 — 스파이크 덤프용).
static FRAME_US: [AtomicU64; N] = [const { AtomicU64::new(0) }; N];
static SPIKES: AtomicUsize = AtomicUsize::new(0);
/// ★프레임 간격(직전 post_update 시작 → 이번 시작). 다른 모드·게임 본체가 멈춘 것까지 잡는 전역 지표.
static LAST_T: std::sync::Mutex<Option<Instant>> = std::sync::Mutex::new(None);
static GAP_MAX: AtomicU64 = AtomicU64::new(0);
static GAP_OVER50: AtomicU64 = AtomicU64::new(0);
const GAP_SPIKE_US: u64 = 100_000;
/// post_update 진입 시 호출. 간격이 GAP_SPIKE_US 이상이면 즉시 한 줄.
pub fn frame_gap(frame: u64, scene: &str) {
    if !on() { return; }
    let now = Instant::now();
    let prev = LAST_T.lock().unwrap_or_else(|e| e.into_inner()).replace(now);
    let Some(p) = prev else { return };
    let us = now.duration_since(p).as_micros() as u64;
    GAP_MAX.fetch_max(us, Ordering::Relaxed);
    if us >= 50_000 { GAP_OVER50.fetch_add(1, Ordering::Relaxed); }
    if us >= GAP_SPIKE_US && SPIKES.fetch_add(1, Ordering::Relaxed) < 400 { write(&format!("GAP f={frame} scene={scene} gap={}ms", us / 1000)); }
}

#[inline]
pub fn on() -> bool { crate::config::get().perf_log }

pub struct Sec { i: usize, t: Option<Instant> }
#[inline]
pub fn sec(i: usize) -> Sec { Sec { i, t: if on() { Some(Instant::now()) } else { None } } }
impl Drop for Sec {
    fn drop(&mut self) {
        let Some(t) = self.t else { return };
        let us = t.elapsed().as_micros() as u64;
        MAX[self.i].fetch_max(us, Ordering::Relaxed);
        SUM[self.i].fetch_add(us, Ordering::Relaxed);
        CNT[self.i].fetch_add(1, Ordering::Relaxed);
        FRAME_US[self.i].fetch_add(us, Ordering::Relaxed);
    }
}

fn write(line: &str) {
    let Some(d) = crate::mod_dir() else { return };
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(format!("{d}\\champ_pos_lock_perf.txt")) {
        let _ = writeln!(f, "[{}] {line}", crate::config::now_hms());
    }
}

/// 자유 형식 한 줄(메인 스레드에서만).
pub fn note(line: &str) { if on() { write(line); } }

/// 메인 스레드 post_update 끝에서 호출. frame_us = 이번 post_update 전체.
pub fn end_frame(frame: u64, scene: &str) {
    if !on() { return; }
    let post = FRAME_US[POST].load(Ordering::Relaxed);
    if post >= SPIKE_US && SPIKES.fetch_add(1, Ordering::Relaxed) < 200 {
        let parts: Vec<String> = (1..N).filter_map(|i| { let v = FRAME_US[i].load(Ordering::Relaxed); if v >= 100 { Some(format!("{}={}", NAMES[i], v)) } else { None } }).collect();
        write(&format!("SPIKE f={frame} scene={scene} post={post}us | {}", parts.join(" ")));
    }
    for a in FRAME_US.iter() { a.store(0, Ordering::Relaxed); }
    if frame % 600 == 0 {
        let parts: Vec<String> = (0..N).filter_map(|i| {
            let n = CNT[i].swap(0, Ordering::Relaxed);
            let s = SUM[i].swap(0, Ordering::Relaxed);
            let m = MAX[i].swap(0, Ordering::Relaxed);
            if n == 0 { None } else { Some(format!("{}: n={} avg={}us max={}us sum={}ms", NAMES[i], n, s / n, m, s / 1000)) }
        }).collect();
        let gm = GAP_MAX.swap(0, Ordering::Relaxed); let g50 = GAP_OVER50.swap(0, Ordering::Relaxed);
        write(&format!("f={frame} scene={scene} | gap_max={}ms gap>=50ms:{} | {}", gm / 1000, g50, parts.join(" | ")));
    }
}
