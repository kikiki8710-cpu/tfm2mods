// ===========================================================================
//  perf.rs — 훅별 비용 계측(모드가 게임에 주는 부담 측정)
// ===========================================================================
// 설계 원칙:
//  1. **프로브가 피측정물보다 크면 안 된다.** buy 디투어 조기탈출 경로는 메모리 읽기 2회(~100 cycle)
//     수준이라, 공유 AtomicU64 에 fetch_add 2회를 걸면 rayon 워커 다중 경합으로 캐시라인 핑퐁이
//     나서 측정값이 곧 부하가 된다 → **buy 계열만 thread_local 누적 + 주기 flush**.
//  2. 나머지(메인스레드 post_update 계열·저빈도 훅)는 호출빈도가 낮아 직접 원자연산으로 충분.
//     단 사이트끼리 **캐시라인 분리**(#[repr(align(64))]).
//  3. 시간축 = rdtsc(원시 사이클). ns 환산은 리포트 시점에 wall-clock 대비로 **런타임 캘리브레이션**
//     (고정 주파수 가정 금지 — 터보/절전으로 실주파수가 변함).
//  4. **프로브 자체 비용을 같이 측정**해서 리포트에 표기(빈 구간 계측 = PROBE_SELF).
//     각 사이트 수치에는 프로브 1회분이 포함돼 있으므로 해석 시 빼서 볼 수 있게.
//  ⚠ cap_launcher 는 91KB chkstk 프레임 위에서 도는 최소 디투어(락/할당/catch_unwind 금지)라
//     여기서도 **원자연산만** 쓰는 rec() 만 호출한다(rec_tl 금지 — TLS 지연초기화 경로 회피).

use std::cell::Cell;
use std::sync::atomic::{AtomicU64, Ordering};

/// 전역 게이트. false 면 모든 rec/rec_tl 이 빈 함수가 되어 호출부까지 DCE 된다.
/// ★프로덕션 OFF(2026-07-22 측정 완료). 계측이 필요하면 true 로만 바꾸면 전 사이트가 되살아난다
///   (계측 코드·사이트 배선은 의도적으로 남겨둠 — 다음 패치/최적화 때 재사용).
pub const PERF_ON: bool = false;
/// 리포트 주기(post_update 프레임 수). 60fps 기준 약 10초.
pub const REPORT_EVERY: u64 = 600;

// ── 사이트 ID (전역 원자 카운터) ────────────────────────────────────────────
pub const S_POST_TOTAL: usize = 0;
pub const S_POST_PT: usize = 1;
pub const S_POST_TACTICS: usize = 2;
pub const S_POST_COMPTEST: usize = 3;
pub const S_POST_HIDE_DD: usize = 4;
pub const S_POST_HIDE_CT: usize = 5;
pub const S_POST_SCENESIDE: usize = 6;
pub const S_POST_ROSTER: usize = 7;
pub const S_POST_UINJ: usize = 8;
pub const S_POST_SPACING: usize = 9;
pub const S_HOOK_RETRY: usize = 10;
pub const S_LAUNCHER: usize = 11;
pub const S_SEEDCTOR: usize = 12;
pub const S_SPAWN: usize = 13;
pub const S_FILLSLOTS: usize = 14;
pub const S_ITEMNET: usize = 15;
pub const S_PROBE_SELF: usize = 16;
pub const N_SITES: usize = 17;

pub const NAMES: [&str; N_SITES] = [
    "post_update(전체)",
    "  ├ PT스냅샷+override",
    "  ├ 개인전술 화면",
    "  ├ 조합테스트 화면",
    "  ├ 네이티브DD 숨김",
    "  ├ 조합테스트DD 숨김",
    "  ├ scene side 직독",
    "  ├ 로스터 폴링",
    "  ├ uinj::install(멱등)",
    "  └ 블루슬롯 간격강제",
    "훅 재시도(멱등,매프레임)",
    "cap_launcher(경기시작)",
    "cap_seed_ctor",
    "cap_spawn",
    "fill_slots(슬롯배열)",
    "itemnet forward",
    "※프로브 자체비용",
];

// ── thread-local 사이트 (sim 워커 핫패스 전용) ──────────────────────────────
pub const T_BUY_ALL: usize = 0;
pub const T_BUY_EARLY: usize = 1;
pub const N_TL: usize = 2;
pub const TL_NAMES: [&str; N_TL] = [
    "buy 디투어(전체, sim워커)",
    "  └ 그중 조기탈출(배경sim)",
];
/// thread_local 누적을 전역으로 넘기는 주기(호출 수).
const TL_FLUSH_EVERY: u64 = 4096;

#[repr(align(64))] // 사이트별 캐시라인 분리 — 인접 사이트 간 false sharing 방지
pub struct Slot {
    pub calls: AtomicU64,
    pub cycles: AtomicU64,
    // ★min: rdtsc는 **CPU 시간이 아니라 경과 시간**이라, 구간 도중 스레드가 선점되면 off-CPU 시간이
    //   통째로 들어간다(배경 sim이 전 코어를 물면 메인스레드에서 상시 발생). 최솟값 = 선점이 없었던
    //   프레임의 값 ⇒ **평균과 최소의 격차 = 선점 노이즈**, 최소 ≈ 실제 작업 비용.
    pub min: AtomicU64,
}
impl Slot {
    const fn new() -> Self { Slot { calls: AtomicU64::new(0), cycles: AtomicU64::new(0), min: AtomicU64::new(u64::MAX) } }
}

static SLOTS: [Slot; N_SITES] = [const { Slot::new() }; N_SITES];
static TL_SLOTS: [Slot; N_TL] = [const { Slot::new() }; N_TL];

// 캘리브레이션 기준점(최초 rec 시각). tsc↔wall 대응으로 실효 주파수 산출.
static CAL_TSC0: AtomicU64 = AtomicU64::new(0);
static CAL_MS0: AtomicU64 = AtomicU64::new(0);

#[inline(always)]
pub fn tsc() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe { core::arch::x86_64::_rdtsc() }
    #[cfg(not(target_arch = "x86_64"))]
    0
}

/// 전역 사이트 기록. start = 구간 진입 시 tsc().
#[inline(always)]
pub fn rec(site: usize, start: u64) {
    if !PERF_ON { return; }
    let d = tsc().wrapping_sub(start);
    if let Some(s) = SLOTS.get(site) {
        s.calls.fetch_add(1, Ordering::Relaxed);
        s.cycles.fetch_add(d, Ordering::Relaxed);
        s.min.fetch_min(d, Ordering::Relaxed);
    }
}

struct Tl {
    calls: [Cell<u64>; N_TL],
    cycles: [Cell<u64>; N_TL],
    n: Cell<u64>,
}
thread_local! {
    // const 초기화 = 지연초기화 플래그 검사 없음(TLS 접근 최소 비용). Drop 없음 → TLS dtor 미등록.
    static TL: Tl = const {
        Tl { calls: [const { Cell::new(0) }; N_TL], cycles: [const { Cell::new(0) }; N_TL], n: Cell::new(0) }
    };
}

/// 워커 핫패스 기록(thread_local 누적 → TL_FLUSH_EVERY 마다 전역 반영).
#[inline(always)]
pub fn rec_tl(site: usize, start: u64) {
    if !PERF_ON { return; }
    let d = tsc().wrapping_sub(start);
    let _ = TL.try_with(|t| {
        if site < N_TL {
            t.calls[site].set(t.calls[site].get() + 1);
            t.cycles[site].set(t.cycles[site].get() + d);
            TL_SLOTS[site].min.fetch_min(d, Ordering::Relaxed); // min 만 즉시 반영(경합 미미)
        }
        let n = t.n.get() + 1;
        t.n.set(n);
        if n >= TL_FLUSH_EVERY {
            t.n.set(0);
            for i in 0..N_TL {
                let c = t.calls[i].replace(0);
                let cy = t.cycles[i].replace(0);
                if c != 0 {
                    TL_SLOTS[i].calls.fetch_add(c, Ordering::Relaxed);
                    TL_SLOTS[i].cycles.fetch_add(cy, Ordering::Relaxed);
                }
            }
        }
    });
}

/// 프로브 자체 비용 샘플(빈 구간 계측) — 리포트에서 사이트당 1회분으로 표기.
#[inline(always)]
pub fn sample_self() {
    if !PERF_ON { return; }
    let t = tsc();
    rec(S_PROBE_SELF, t);
}

fn calib(now_ms: u64) -> (u64, u64) {
    let t = tsc();
    let t0 = CAL_TSC0.load(Ordering::Relaxed);
    if t0 == 0 {
        CAL_TSC0.store(t, Ordering::Relaxed);
        CAL_MS0.store(now_ms, Ordering::Relaxed);
        return (0, 0);
    }
    (t.wrapping_sub(t0), now_ms.saturating_sub(CAL_MS0.load(Ordering::Relaxed)))
}

/// 리포트 생성. frames = post_update 호출 수(전체 사이트의 프레임 기준).
pub fn report(now_ms: u64, frames: u64) -> String {
    let (dtsc, dms) = calib(now_ms);
    // 실효 주파수: 캘리브레이션 구간이 너무 짧으면 환산 생략(사이클만 표기).
    let cyc_per_ms = if dms >= 1000 { dtsc / dms.max(1) } else { 0 };
    let to_ns = |cycles: u64| -> Option<u64> {
        if cyc_per_ms == 0 { None } else { Some(cycles.saturating_mul(1_000_000) / cyc_per_ms.max(1)) }
    };
    let mut s = String::new();
    s.push_str(&format!(
        "[perf] 경과 {:.1}s / post_update {} 프레임 / 실효 TSC {}\n",
        dms as f64 / 1000.0, frames,
        if cyc_per_ms == 0 { "측정중(≥1s 필요)".to_string() }
        else { format!("{:.2} GHz", cyc_per_ms as f64 / 1_000_000.0) }
    ));
    s.push_str("  ⚠ 각 수치에는 프로브 1회분(맨 아래)이 포함됨. 사이클→ns 는 wall-clock 대비 실측 환산.\n\n");
    s.push_str("  ★최소 = 선점(preempt) 없이 끝난 프레임의 값 ≈ 실제 작업 비용. 평균≫최소면 그 차이는\n     구간 도중 스레드가 밀려난 시간이지 모드가 쓴 CPU가 아니다.\n\n");
    s.push_str(&format!("{:<28} {:>12} {:>12} {:>12} {:>12} {:>10}\n", "사이트", "호출", "총 ms", "평균 ns", "★최소 ns", "wall 점유"));
    s.push_str(&"─".repeat(92));
    s.push('\n');

    let mut row = |name: &str, calls: u64, cycles: u64, mn: u64, out: &mut String| {
        let tot_ns = to_ns(cycles);
        let avg_ns = if calls == 0 { None } else { to_ns(cycles / calls) };
        let occupancy = match (tot_ns, dms) {
            (Some(t), d) if d > 0 => format!("{:.3}%", (t as f64 / 1e6) / d as f64 * 100.0),
            _ => "-".to_string(),
        };
        let min_ns = if mn == u64::MAX { None } else { to_ns(mn) };
        out.push_str(&format!(
            "{:<28} {:>12} {:>12} {:>12} {:>12} {:>10}\n",
            name,
            calls,
            tot_ns.map(|n| format!("{:.2}", n as f64 / 1e6)).unwrap_or_else(|| "-".into()),
            avg_ns.map(|n| n.to_string()).unwrap_or_else(|| format!("{}cyc", if calls == 0 { 0 } else { cycles / calls })),
            min_ns.map(|n| n.to_string()).unwrap_or_else(|| if mn == u64::MAX { "-".into() } else { format!("{}cyc", mn) }),
            occupancy
        ));
    };

    // 메인스레드 계열
    for i in 0..N_SITES {
        if i == S_PROBE_SELF { continue; }
        let c = SLOTS[i].calls.load(Ordering::Relaxed);
        let cy = SLOTS[i].cycles.load(Ordering::Relaxed);
        if c == 0 { continue; }
        row(NAMES[i], c, cy, SLOTS[i].min.load(Ordering::Relaxed), &mut s);
    }
    // sim 워커 계열(thread_local 합산 — 미flush 잔량은 최대 4096콜/스레드 누락 가능)
    s.push('\n');
    s.push_str("── sim 워커(rayon, 여러 스레드 합산) ──\n");
    for i in 0..N_TL {
        let c = TL_SLOTS[i].calls.load(Ordering::Relaxed);
        let cy = TL_SLOTS[i].cycles.load(Ordering::Relaxed);
        if c == 0 { continue; }
        row(TL_NAMES[i], c, cy, TL_SLOTS[i].min.load(Ordering::Relaxed), &mut s);
    }
    s.push_str("  ※ wall 점유가 100%를 넘을 수 있음 = 여러 워커 스레드 시간의 합(정상).\n");
    s.push_str("  ※ 스레드별 미flush 잔량(최대 4096콜)은 누락 — 총량이 클수록 오차 무시 가능.\n");

    // 프로브 자체 비용
    let pc = SLOTS[S_PROBE_SELF].calls.load(Ordering::Relaxed);
    let pcy = SLOTS[S_PROBE_SELF].cycles.load(Ordering::Relaxed);
    s.push('\n');
    if pc > 0 {
        let avg = pcy / pc;
        s.push_str(&format!(
            "{:<28} {:>12} {:>12} {:>12}\n", NAMES[S_PROBE_SELF], pc, "-",
            to_ns(avg).map(|n| n.to_string()).unwrap_or_else(|| format!("{}cyc", avg))
        ));
        s.push_str("  → 위 모든 사이트의 '평균'에서 이 값을 빼면 순수 작업 비용에 가깝다.\n");
    }
    s
}
