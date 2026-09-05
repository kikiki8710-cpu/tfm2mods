//! judge — AI 판단 계층(`game-ai`) **버전별 재구현**. 착수서 = REPORT\tfm2_ai_adjust\04_판단재구현_착수서.md · 설계 = 05_judge_계층_설계.md.
//!
//! 방침(유저, 2026-09-06): 구 재현 코드를 마이그(재핀)하지 않는다. **매 버전 디컴 소스를 따와 새로 포팅**하고,
//! 버전마다 반복되는 기계적 일(RVA 재탐색·프롤로그 안전성·상수 파일·스켈레톤)은 `MIG\aiport.py` 가 한다.
//!
//! 구성(파일 1개 = 관심사 1개):
//!   gen_fns.rs   ★자동생성(aiport gen) — 함수별 RVA/크기/옮길 프롤로그 바이트. 손으로 고치지 않는다.
//!   layout.rs    구조체 오프셋·vtable 슬롯(버전 태그 붙은 상수). 포팅 코드에 매직 넘버를 두지 않기 위한 유일한 자리.
//!   world.rs     게임 상태 읽기(메모리 읽기만 — 게임 함수 호출 0). 핸들→엔티티 등 vtable 슬롯의 **순수 재현**.
//!   hook.rs      바이트 검증 wrap 설치기(프롤로그가 gen_fns 와 완전 일치할 때만 패치 = 스테일 RVA 방어).
//!   port\*.rs    포팅 본체. 함수 1개 = 파일 1개, 스켈레톤은 `aiport skeleton <name>` 이 디컴 C 를 동봉해 만든다.
//!
//! 훅 3종류:
//!   judge_hook!      스칼라 반환 함수(스코어러): mine(i64) vs 게임 rax.
//!   judge_hook_out!  out-writer(Plan 핸들러, out 0x30B): mine = 쓰기집합(MpOut) vs 게임이 실제로 쓴 바이트. 게임 호출 **뒤**에 mine 계산
//!                    (콜리 캡처를 쓰기 위해). live 면 쓰기집합을 out 에 직접 쓰고 원본 skip.
//!   judge_capture!   콜리 캡처(검증 전용): 게임 콜리를 그대로 실행하고 결과만 thread-local 에 남긴다 → 아직 안 포팅한 콜리(예: RNG 소비
//!                    ability_pick 0xe7a8c0)를 부모 포팅이 검증 단계에서 "게임 결과" 로 대신 쓴다. ⚠live 는 그 콜리를 포팅한 뒤에만.
//!
//! 동작 모드(cfg 키 — 파서 미지키는 TUNE_TABLE 로 들어오므로 배선 불요):
//!   judge_verify (기본 **0** — opt-in) : 훅 설치. 게임 원본을 실행하고 mine 과 **대조만** 한다 → 행동 무변경.
//!   judge_live   (기본 0) : mine 이 Some 이면 게임 원본을 건너뛰고 mine 을 반환/기록(대체). None 이면 원본.
//!   ⚠ 대체(live)는 RNG-free 함수에만 안전하다. 난수를 소비하는 함수는 이중 소비 desync(방법론 메모리 07-23 교훈).
//!
//! 산출(전부 직접 write = LOG_ON 무관):
//!   judge_install.txt   설치 결과 / judge_status.txt   누적 카운터(n/ok/diff/na/live/entered) / judge_<fn>.txt   표본·DIFF 상세(≤240줄)
//!
//! 규약(CLAUDE.md §3): 포팅 코드는 게임 헬퍼를 FFI 로 호출하지 않는다. 읽기는 전부 `rd_*`(VEH 경유). wrap 본문은 catch_unwind.
//! ★트램폴린 복귀는 `jmp [rip+0]`(hook.rs) — `movabs rax` 는 옮긴 프롤로그가 rax 를 쓰는 함수에서 값을 파괴한다(2026-09-06 실사고).
#![allow(dead_code)]
use crate::*;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

pub mod gen_fns;
pub mod layout;
pub mod world;
pub mod hook;
pub mod laycheck;
pub mod port {
    pub mod steal_score;
    pub mod hunt_battle;
    pub mod epic_hunt_battle;
    pub mod serpen_hunt_battle;
    pub mod passive_line_callees;
    pub mod passive_line;
}
use gen_fns::*;

/// 8인자 공통 뷰. SubPlan 스코어러: p1=상태 · p5=선수 sim · p6=&Holder · p7=SmallAction(→i64).
/// Plan 핸들러(디스패처 `0xcaf9f0` 가 `add rdx,8` 후 call): p1=out(MovePriority 0x30) · p2=Plan payload · p3/p4(rng) · p5=선수 sim · p6=&Holder · p7=타이머 등.
#[derive(Clone, Copy)]
pub struct ScorerArgs { pub p1: usize, pub p2: usize, pub p3: usize, pub p4: usize, pub p5: usize, pub p6: usize, pub p7: usize, pub p8: usize }
pub type Args8 = ScorerArgs;

/// out-writer 의 쓰기집합. 게임도 경로별로 out 의 일부만 쓰므로(잔재는 그대로 커밋) 검증은 **쓴 바이트만** 대조한다.
#[derive(Clone, Copy, Default)]
pub struct MpWrite { pub off: u8, pub len: u8, pub val: u64 }
#[derive(Clone, Copy, Default)]
pub struct MpOut { pub n: u8, pub w: [MpWrite; 6] }
impl MpOut {
    pub fn push(&mut self, off: usize, len: usize, val: u64) { if (self.n as usize) < self.w.len() { self.w[self.n as usize] = MpWrite { off: off as u8, len: len as u8, val }; self.n += 1; } }
    pub fn code(&mut self, c: u64) { self.push(0, 8, c); }
    pub fn get_code(&self) -> Option<u64> { self.w[..self.n as usize].iter().find(|w| w.off == 0 && w.len == 8).map(|w| w.val) }
}
unsafe fn read_n(addr: usize, len: u8) -> Option<u64> {
    match len { 1 => Some(rd_u8(addr) as u64), 2 => Some((rd_u8(addr) as u64) | ((rd_u8(addr + 1) as u64) << 8)), 4 => Some(rd_u32(addr) as u64), 8 => rd_u64(addr), _ => None }
}
/// 검증: 쓰기집합의 각 항목을 게임이 쓴 out 과 대조. Some(true)=전부 일치 / Some(false)=불일치 / None=읽기 실패.
unsafe fn compare_out(out: usize, mine: &MpOut) -> Option<bool> {
    for w in &mine.w[..mine.n as usize] { if read_n(out + w.off as usize, w.len)? != w.val { return Some(false); } }
    Some(true)
}
/// live: 쓰기집합을 out 에 적용. 첫 write 는 wr_u64 프로브(쓰기 가능 확인).
unsafe fn apply_out(out: usize, mine: &MpOut) -> bool {
    if !ptr_ok(out) || !writable(out, 0x30) { return false; }
    for w in &mine.w[..mine.n as usize] {
        let a = out + w.off as usize;
        match w.len { 1 => core::ptr::write_unaligned(a as *mut u8, w.val as u8), 2 => core::ptr::write_unaligned(a as *mut u16, w.val as u16),
                      4 => core::ptr::write_unaligned(a as *mut u32, w.val as u32), _ => core::ptr::write_unaligned(a as *mut u64, w.val) }
    }
    true
}
/// 두 쓰기집합이 같은가(같은 포팅이 같은 순서로 push 하므로 순서 비교로 충분).
pub fn same_out(a: &MpOut, b: &MpOut) -> bool { a.n == b.n && a.w[..a.n as usize].iter().zip(b.w[..b.n as usize].iter()).all(|(x, y)| x.off == y.off && x.len == y.len && x.val == y.val) }
unsafe fn hex30(out: usize) -> String { (0..0x30).map(|i| format!("{:02x}", rd_u8(out + i))).collect::<Vec<_>>().join("") }
fn fmt_writes(m: &MpOut) -> String { m.w[..m.n as usize].iter().map(|w| format!("+{:#x}/{}={:#x}", w.off, w.len, w.val)).collect::<Vec<_>>().join(" ") }

/// 함수별 검증 카운터(전부 원자 — 디투어 문맥에서 lock/alloc 없이 갱신).
pub struct Stat {
    pub n: AtomicU64, pub ok: AtomicU64, pub diff: AtomicU64, pub na: AtomicU64, pub live: AtomicU64,
    pub logged: AtomicU64,          // 파일에 쓴 줄 수(상한)
    pub entered: AtomicU64,         // wrap 진입 수(게임 호출 전 증가 — n 과 달리 크래시 직전 발화도 센다)
    pub vt_rva: AtomicUsize,        // 진단: 첫 호출에서 읽은 WorldOps vt+VT_WORLD_ENTITY 타깃 RVA(순수 read)
    /// 분기 태그별 OK/DIFF 카운터 — 포팅이 `tr(3, tag<<56)` 로 "내가 탄 분기" 를 표시하면(tag 1..7) 그 분기의 일치율을 따로 센다.
    /// (DIFF 만 보면 "게임이 이 분기를 아예 안 타는지 / 데이터 한 건만 다른지" 를 못 가른다.) 태그별 OK 표본도 ≤4줄 남긴다.
    pub tag_ok: [AtomicU64; 8], pub tag_diff: [AtomicU64; 8], pub tag_logged: [AtomicU64; 8],
    /// live 에서 "노브 적용 출력 ≠ 게임 동치 출력" 이었던 횟수 = 노브가 실제로 판단을 바꾼 횟수(knob effect).
    pub knob_eff: AtomicU64,
}
impl Stat {
    pub const fn new() -> Self {
        Stat { n: AtomicU64::new(0), ok: AtomicU64::new(0), diff: AtomicU64::new(0), na: AtomicU64::new(0), live: AtomicU64::new(0),
               logged: AtomicU64::new(0), entered: AtomicU64::new(0), vt_rva: AtomicUsize::new(0),
               tag_ok: [const { AtomicU64::new(0) }; 8], tag_diff: [const { AtomicU64::new(0) }; 8], tag_logged: [const { AtomicU64::new(0) }; 8], knob_eff: AtomicU64::new(0) }
    }
}
/// 분기 태그 규약: `tr(3, (tag as u64) << 56 | 하위값)` — 상위 바이트가 태그(1..7), 하위는 자유(기존 c15 등과 충돌 없음).
#[inline] pub fn tag_of(t3: u64) -> usize { ((t3 >> 56) & 7) as usize }

/// ★라이브 즉치 읽기 — 이 모드의 바이트패치 노브가 덮어쓴 exe 바이트를 그대로 읽는다(layout::SITE_*). 읽기 실패·base 미확정이면 원본값.
/// 왜: 포팅의 검증 대상은 "게임 원본" 이 아니라 "바이트패치까지 적용된 실행 이미지" 다. recently_seen 창 0x78 을 정적으로 박았다가
///     cfg `vw_check=90` 패치(0x1323a5b) 와 어긋나 5판(2%·22만 건)을 태웠다(2026-09-06). 라이브 승격 시엔 해당 노브를 포팅 인자로 옮긴다.
#[inline] pub unsafe fn live_imm8(rva_imm: usize, orig: u8) -> u8 { let b = crate::exe_base(); if b == 0 { orig } else { crate::rd_u8(b + rva_imm) } }
#[inline] pub unsafe fn live_imm16(rva_imm: usize, orig: u16) -> u16 { let b = crate::exe_base(); if b == 0 { orig } else { (crate::rd_u8(b + rva_imm) as u16) | ((crate::rd_u8(b + rva_imm + 1) as u16) << 8) } }

#[inline] pub fn live() -> bool { tune("judge_live", 0) != 0 }
/// ★함수별 live 게이트: 마스터 `judge_live=1` **그리고** `judge_live_<fn>` = 1(live: 원본 건너뛰고 mine) / 2(shadow: 원본도 돌려 대조 기록하되 **mine 을 반환**).
/// shadow 는 RNG-free 함수에만(원본 실행이 부수효과 없을 때). 첫 승격은 shadow 로 시작해 DIFF 계측을 유지한다.
#[inline] pub fn live_mode(key: &str) -> i64 { if tune("judge_live", 0) == 0 || !laycheck::ok_for_live() { 0 } else { tune(key, 0) } }   // layout FAIL 이면 live 전부 차단

/// 포팅 내부 추적값(DIFF 원인 분리용). 포팅이 `tr(i, v)` 로 채우고 record 가 DIFF/NA 줄에 같이 찍는다. thread-local·고정배열(alloc 없음).
thread_local! { static TRACE: std::cell::Cell<[u64; 12]> = const { std::cell::Cell::new([0; 12]) }; }
#[inline] pub fn tr(i: usize, v: u64) { if i < 12 { TRACE.with(|c| { let mut a = c.get(); a[i] = v; c.set(a); }); } }
#[inline] pub fn tr_get(i: usize) -> u64 { if i < 12 { TRACE.with(|c| c.get()[i]) } else { 0 } }
pub fn tr_reset() { TRACE.with(|c| c.set([0; 12])); }
pub fn tr_fmt() -> String { TRACE.with(|c| c.get().iter().enumerate().filter(|(_, v)| **v != 0).map(|(i, v)| format!("t{}={:#x}", i, v)).collect::<Vec<_>>().join(" ")) }
#[inline] fn status_due(n: u64) -> bool { n == 1 || n == 16 || n == 64 || n == 256 || n % 500 == 0 }

/// 직접 append(로그 인프라·LOG_ON 과 무관). 디투어 문맥에서 호출되므로 호출 빈도는 record() 가 제한한다.
pub fn append_direct(name: &str, s: &str) {
    if let Some(p) = pth(name) {
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = std::io::Write::write_all(&mut f, s.as_bytes()); }
    }
}

type F12 = unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;

/// 스칼라 반환 훅. ⚠인자 12개 과선언 = Win64 에서 항상 안전, 과소선언은 AV(08-05 실사고).
macro_rules! judge_hook {
    ($m:ident, $spec:expr, $mine:path, $live_mine:path, $lkey:expr) => {
        pub mod $m {
            use std::sync::atomic::{AtomicUsize, Ordering};
            pub static ORIG: AtomicUsize = AtomicUsize::new(0);
            pub static ST: super::Stat = super::Stat::new();
            pub unsafe extern "C" fn wrap(p1: usize, p2: usize, p3: usize, p4: usize, p5: usize, p6: usize, p7: usize, p8: usize,
                                          p9: usize, p10: usize, p11: usize, p12: usize) -> usize {
                let orig = ORIG.load(Ordering::Relaxed);
                if orig == 0 { return 0; }
                let f: super::F12 = core::mem::transmute(orig);
                let a = super::ScorerArgs { p1, p2, p3, p4, p5, p6, p7, p8 };
                let en = ST.entered.fetch_add(1, Ordering::Relaxed) + 1;
                if en <= 3 { super::append_direct(&format!("judge_{}.txt", $spec.name), &format!("[{} ENTER #{}] p1={:#x} p5={:#x} p6={:#x} p7={:#x}\n", $spec.name, en, p1, p5, p6, p7)); }
                super::tr_reset();
                let mine: Option<i64> = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $mine(&a))).unwrap_or(None);
                let mode = super::live_mode($lkey);
                // live 출력 = 노브 적용판($live_mine). $mine 은 게임 동치판(검증 지표 유지). 둘이 다르면 knob_eff.
                let live_out = |mine: Option<i64>| -> Option<i64> {
                    let lv: Option<i64> = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $live_mine(&a))).unwrap_or(None);
                    if lv.is_some() && lv != mine { ST.knob_eff.fetch_add(1, Ordering::Relaxed); }
                    lv
                };
                if mode == 1 {
                    if let Some(v) = live_out(mine) { ST.live.fetch_add(1, Ordering::Relaxed); return v as usize; }
                }
                let game = f(p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12);
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| super::record(&$spec, &ST, game as i64, mine, &a)));
                if mode == 2 {
                    if let Some(v) = live_out(mine) { ST.live.fetch_add(1, Ordering::Relaxed); return v as usize; }   // shadow: 대조는 기록하고 행동은 mine
                }
                game
            }
        }
    };
}

/// out-writer 훅(Plan 핸들러). $pre = 게임 호출 직전 훅(콜리 캡처 리셋 등). live 는 $live_ok 가 true 일 때만 허용.
macro_rules! judge_hook_out {
    ($m:ident, $spec:expr, $mine:path, $live_mine:path, $pre:path, $live_ok:expr, $lkey:expr) => {
        pub mod $m {
            use std::sync::atomic::{AtomicUsize, Ordering};
            pub static ORIG: AtomicUsize = AtomicUsize::new(0);
            pub static ST: super::Stat = super::Stat::new();
            pub unsafe extern "C" fn wrap(p1: usize, p2: usize, p3: usize, p4: usize, p5: usize, p6: usize, p7: usize, p8: usize,
                                          p9: usize, p10: usize, p11: usize, p12: usize) -> usize {
                let orig = ORIG.load(Ordering::Relaxed);
                if orig == 0 { return 0; }
                let f: super::F12 = core::mem::transmute(orig);
                let a = super::ScorerArgs { p1, p2, p3, p4, p5, p6, p7, p8 };
                let en = ST.entered.fetch_add(1, Ordering::Relaxed) + 1;
                if en <= 3 { super::append_direct(&format!("judge_{}.txt", $spec.name), &format!("[{} ENTER #{}] out={:#x} p2={:#x} p5={:#x} p6={:#x} p7={:#x}\n", $spec.name, en, p1, p2, p5, p6, p7)); }
                $pre();
                super::tr_reset();
                let mode = if $live_ok { super::live_mode($lkey) } else { 0 };
                let live_out = |mine: Option<super::MpOut>| -> Option<super::MpOut> {
                    let lv: Option<super::MpOut> = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $live_mine(&a))).unwrap_or(None);
                    if let (Some(l), Some(m)) = (lv.as_ref(), mine.as_ref()) { if !super::same_out(l, m) { ST.knob_eff.fetch_add(1, Ordering::Relaxed); } }
                    lv
                };
                if mode == 1 {
                    let mine: Option<super::MpOut> = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $mine(&a))).unwrap_or(None);
                    if let Some(m) = live_out(mine) { if super::apply_out(p1, &m) { ST.live.fetch_add(1, Ordering::Relaxed); return p1; } }
                }
                let game = f(p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12);
                let mine: Option<super::MpOut> = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $mine(&a))).unwrap_or(None);
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| super::record_out(&$spec, &ST, p1, mine, &a)));
                if mode == 2 {
                    if let Some(m) = live_out(mine) { if super::apply_out(p1, &m) { ST.live.fetch_add(1, Ordering::Relaxed); } }   // shadow: 게임 out 위에 노브 적용 쓰기집합 덮어쓰기
                }
                game
            }
        }
    };
}

/// 콜리 캡처(검증 전용). 게임 콜리를 그대로 돌리고 `[p1]`,`[p1+8]`,`[p1+0x10]` 를 thread-local 에 남긴다.
macro_rules! judge_capture {
    ($m:ident, $spec:expr) => {
        pub mod $m {
            use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
            use std::cell::Cell;
            pub static ORIG: AtomicUsize = AtomicUsize::new(0);
            pub static ST: super::Stat = super::Stat::new();
            static SEQ: AtomicU64 = AtomicU64::new(0);
            thread_local! { static LAST: Cell<(u64, u64, u64, u64)> = const { Cell::new((0, 0, 0, 0)) }; }   // (seq, r0, r1, r2)
            pub fn reset() { LAST.with(|c| c.set((0, 0, 0, 0))); }
            /// 리셋 이후 이 스레드에서 캡처된 결과. None = 콜리가 안 불렸다(부모 경로 불일치 → NA).
            pub fn take() -> Option<(u64, u64, u64)> { LAST.with(|c| { let v = c.get(); if v.0 == 0 { None } else { Some((v.1, v.2, v.3)) } }) }
            pub unsafe extern "C" fn wrap(p1: usize, p2: usize, p3: usize, p4: usize, p5: usize, p6: usize, p7: usize, p8: usize,
                                          p9: usize, p10: usize, p11: usize, p12: usize) -> usize {
                let orig = ORIG.load(Ordering::Relaxed);
                if orig == 0 { return 0; }
                let f: super::F12 = core::mem::transmute(orig);
                ST.entered.fetch_add(1, Ordering::Relaxed);
                let r = f(p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12);
                let seq = SEQ.fetch_add(1, Ordering::Relaxed) + 1;
                let (r0, r1, r2) = (crate::rd_u64(p1).unwrap_or(u64::MAX), crate::rd_u64(p1 + 8).unwrap_or(0), crate::rd_u64(p1 + 0x10).unwrap_or(0));
                LAST.with(|c| c.set((seq, r0, r1, r2)));
                ST.n.fetch_add(1, Ordering::Relaxed);
                r
            }
        }
    };
}

judge_hook!(steal_hook, crate::judge::gen_fns::STEAL_SCORE, crate::judge::port::steal_score::steal_score, crate::judge::port::steal_score::steal_score, "judge_live_steal_score");
judge_capture!(cap_ability_pick, crate::judge::gen_fns::ABILITY_PICK);
judge_hook_out!(epic_hb_hook, crate::judge::gen_fns::EPIC_HUNT_BATTLE, crate::judge::port::epic_hunt_battle::epic_hunt_battle, crate::judge::port::epic_hunt_battle::epic_hunt_battle, crate::judge::cap_ability_pick::reset, false, "judge_live_epic_hunt_battle");
judge_hook_out!(passive_line_hook, crate::judge::gen_fns::PASSIVE_LINE, crate::judge::port::passive_line::passive_line, crate::judge::port::passive_line::passive_line_live, crate::judge::cap_recent_seen::reset, true, "judge_live_passive_line");

/// recently_seen(0x1323a00) 캡처(검증 전용) — 게임 콜리를 그대로 돌리고, **같은 순간**에 내 재현(lane_pred)을 같은 인자로 계산해
/// (게임 반환, 내 반환, last_seen, tick) 을 thread-local 링(8)에 남긴다. 부모(passive_line) 포팅이 적별로 꺼내 대조한다.
/// 용도 = "코드는 같은데 결과가 다르다" 를 ①그 순간에도 다르다(로직 오독) / ②그 순간엔 같고 나중(부모 원본 실행 후) 읽으면 다르다(부수효과) 로 가른다.
pub mod cap_recent_seen {
    use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
    use std::cell::Cell;
    pub static ORIG: AtomicUsize = AtomicUsize::new(0);
    pub static ST: super::Stat = super::Stat::new();
    static LOGGED: AtomicU64 = AtomicU64::new(0);
    static OKLOG: AtomicU64 = AtomicU64::new(0);
    #[derive(Clone, Copy)] pub struct Cap { pub ent: usize, pub game: u8, pub mine: u8, pub last: u64, pub tick: u64 }
    const Z: Cap = Cap { ent: 0, game: 0, mine: 0, last: 0, tick: 0 };
    thread_local! { static RING: Cell<([Cap; 8], usize)> = const { Cell::new(([Z; 8], 0)) }; }
    pub fn reset() { RING.with(|c| { let mut v = c.get(); v.1 = 0; c.set(v); }); }
    /// 리셋 이후 이 스레드에서 ent 로 불린 마지막 캡처.
    pub fn find(ent: usize) -> Option<Cap> { RING.with(|c| { let (a, n) = c.get(); a[..n.min(8)].iter().rev().find(|e| e.ent == ent).copied() }) }
    pub unsafe extern "C" fn wrap(p1: usize, p2: usize, p3: usize, p4: usize, p5: usize, p6: usize, p7: usize, p8: usize,
                                  p9: usize, p10: usize, p11: usize, p12: usize) -> usize {
        let orig = ORIG.load(Ordering::Relaxed);
        if orig == 0 { return 0; }
        let f: super::F12 = core::mem::transmute(orig);
        ST.entered.fetch_add(1, Ordering::Relaxed);
        let r = f(p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12);
        let game = (r & 0xff) as u8;
        let res: Option<(u8, u64)> = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| crate::judge::port::passive_line_callees::lane_pred(p1, p2, p3, p4, p5, super::live_imm8(super::layout::SITE_VW_CHECK_IMM, 0x78) as u64))).unwrap_or(None);
        let (mine, last) = res.unwrap_or((0xff, 0));
        let tick = crate::rd_u64(p2 + super::layout::W_TICK).unwrap_or(0);
        RING.with(|c| { let (mut a, n) = c.get(); a[n % 8] = Cap { ent: p5, game, mine, last, tick }; c.set((a, n + 1)); });
        ST.n.fetch_add(1, Ordering::Relaxed);
        let win = super::live_imm8(super::layout::SITE_VW_CHECK_IMM, 0x78) as u64;
        let margin = (last.wrapping_add(win) as i64).wrapping_sub(tick as i64);
        let agree = mine != 0xff && (game != 0) == (mine != 0);
        if mine == 0xff { ST.na.fetch_add(1, Ordering::Relaxed); }
        else if agree { ST.ok.fetch_add(1, Ordering::Relaxed); }
        else { ST.diff.fetch_add(1, Ordering::Relaxed); }
        // 경계 표본: 기록 경로(mine==2)에서 여유 30~70 인 OK 도 남긴다(게임 임계 실측용) + DIFF 는 ≤60줄
        let want = if mine == 0xff { false } else if !agree { LOGGED.fetch_add(1, Ordering::Relaxed) < 60 } else { mine == 2 && (30..=70).contains(&margin) && OKLOG.fetch_add(1, Ordering::Relaxed) < 12 };
        if want {
            {
                let h = crate::rd_u64(p5 + super::layout::ENT_HANDLE).unwrap_or(0);
                let side = crate::rd_u64(p4 + super::layout::P5_SIDE).unwrap_or(99);
                let w = super::world::World { x: 0, data: p2, vt: p3 };
                let vis = w.visible(side, h); let rec = w.roster_rec(h).unwrap_or(usize::MAX);
                let idx = if rec != 0 && rec != usize::MAX { crate::rd_u32(rec + super::layout::REC_ROLE) } else { 0xffff };
                // 런타임 실체 덤프: vt RVA·슬롯 타깃 RVA(+첫 8B — 런타임 패치 탐지)·tick 이웃·적팀 last 표·레코드 표 헤더·rec 필드
                let base = crate::exe_base();
                let rva = |a: usize| if base != 0 && a > base && a - base < 0x8000000 { a - base } else { 0 };
                let tgt = |slot: usize| crate::rd_u64(p3 + slot).unwrap_or(0) as usize;
                let (t28, tf8, t150) = (tgt(0x28), tgt(0xf8), tgt(0x150));
                let b8 = |a: usize| crate::rd_u64(a).unwrap_or(0);
                let tbl: Vec<String> = (0..5usize).map(|i| format!("{}", crate::rd_u64(p1 + 0x1e0 + i * 8).unwrap_or(u64::MAX) as i64)).collect();
                let wn: Vec<String> = (0..5usize).map(|i| format!("{:#x}", crate::rd_u64(p2 + 0xec90 + i * 8).unwrap_or(0))).collect();
                let rf = |off: usize| if rec != 0 && rec != usize::MAX { crate::rd_u64(rec + off).unwrap_or(0) } else { 0 };
                // ★진단 전용 FFI 프로브(cfg judge_ffi_probe=1 일 때만): 게임의 순수 리더 3종을 같은 인자로 직접 불러 내 재현과 나란히 찍는다.
                //   호출 대상은 RVA 가 정적 규명값과 정확히 일치할 때만(0x1851b50 = 스캔 / 0x1847490 = visible / 0x1851f10 = tick). 전부 leaf·부수효과 없음.
                //   CLAUDE.md §3 의 "재현에 FFI 금지" 는 포팅 본체 규칙 — 이건 검증 로그 한 줄이며 기본 OFF.
                let mut probe = String::new();
                if crate::tune("judge_ffi_probe", 0) != 0 && rva(t150) == 0x1851b50 && rva(tf8) == 0x1847490 && rva(t28) == 0x1851f10 && side < 2 {
                    type F3 = unsafe extern "C" fn(usize, usize, usize) -> usize;
                    let f150: F3 = core::mem::transmute(t150); let ff8: F3 = core::mem::transmute(tf8); let f28: F3 = core::mem::transmute(t28);
                    let grec = f150(p2, h as usize, 0); let gvis = ff8(p2, side as usize, h as usize) & 0xff; let gtick = f28(p2, 0, 0);
                    let gidx = if grec != 0 { crate::rd_u32(grec + 0x9c0) } else { 0xffff };
                    let glast = if grec != 0 && gidx < 8 { crate::rd_u64(p1 + 0x1e0 + gidx as usize * 8).unwrap_or(u64::MAX) } else { u64::MAX };
                    probe = format!(" | PROBE game: rec={:#x} idx={} last={} vis={} tick={}", grec, gidx, glast as i64, gvis, gtick);
                }
                super::append_direct("judge_recently_seen.txt", &format!(
                    "[recently_seen {}] game={} mine={} | team={:#x} data={:#x} vt={:#x}(rva {:#x}) rec_self={:#x} ent={:#x} | h={:#x} side={} vis={:?} rec={:#x} idx={} last={} tick={} margin={} | slots +28={:#x}[{:016x}] +f8={:#x}[{:016x}] +150={:#x}[{:016x}] entry[{:016x}] | w+ec90..={} | team.last[0..5]={} | rectbl base={:#x} cnt={} | rec.9c0={} .9c8={} .9d0={} .930={}{}\n",
                    if agree { "OK" } else { "DIFF" }, game, mine, p1, p2, p3, rva(p3), p4, p5, h, side, vis, rec, idx, last, tick, margin,
                    rva(t28), b8(t28), rva(tf8), b8(tf8), rva(t150), b8(t150), b8(base + 0x1323a00),
                    wn.join(","), tbl.join(","),
                    crate::rd_u64(p2 + 0x858).unwrap_or(0), crate::rd_u64(p2 + 0x860).unwrap_or(0),
                    rf(0x9c0) & 0xffff_ffff, rf(0x9c8), rf(0x9d0), rf(0x930), probe));
            }
        }
        r
    }
}
judge_hook_out!(serpen_hb_hook, crate::judge::gen_fns::SERPEN_HUNT_BATTLE, crate::judge::port::serpen_hunt_battle::serpen_hunt_battle, crate::judge::port::serpen_hunt_battle::serpen_hunt_battle, crate::judge::cap_ability_pick::reset, false, "judge_live_serpen_hunt_battle");

/// 등록된 훅 전부(status 덤프용). 훅을 늘리면 여기와 install() 에 한 줄씩.
pub fn stats() -> Vec<(&'static str, &'static Stat)> {
    vec![(STEAL_SCORE.name, &steal_hook::ST), (ABILITY_PICK.name, &cap_ability_pick::ST), (RECENTLY_SEEN.name, &cap_recent_seen::ST),
         (EPIC_HUNT_BATTLE.name, &epic_hb_hook::ST), (SERPEN_HUNT_BATTLE.name, &serpen_hb_hook::ST), (PASSIVE_LINE.name, &passive_line_hook::ST)]
}

fn sample_line(st: &Stat, n: u64, verdict: &str) -> bool {
    let want = n <= 16 || verdict == "DIFF" || n % 2000 == 0;
    if want && st.logged.load(Ordering::Relaxed) < 240 { st.logged.fetch_add(1, Ordering::Relaxed); true } else { false }
}

pub unsafe fn record(spec: &FnSpec, st: &Stat, game: i64, mine: Option<i64>, a: &ScorerArgs) {
    let n = st.n.fetch_add(1, Ordering::Relaxed) + 1;
    let verdict = match mine {
        None => { st.na.fetch_add(1, Ordering::Relaxed); "NA" }
        Some(m) if m == game => { st.ok.fetch_add(1, Ordering::Relaxed); "OK" }
        Some(_) => { st.diff.fetch_add(1, Ordering::Relaxed); "DIFF" }
    };
    if n == 1 {
        if let Some(w) = world::World::from_holder(a.p6) { st.vt_rva.store(w.slot_target_rva(layout::VT_WORLD_ENTITY), Ordering::Relaxed); laycheck::run_once(&w); }
    }
    if sample_line(st, n, verdict) {
        let tag = if ptr_ok(a.p7) { rd_u8(a.p7 + layout::SA_TAG) } else { 0xff };
        let phase = if ptr_ok(a.p1) { rd_u8(a.p1 + layout::STEAL_PHASE) } else { 0xff };
        append_direct(&format!("judge_{}.txt", spec.name),
            &format!("[{} #{}] {} game={} mine={:?} | p1={:#x} phase={} p5={:#x} p6={:#x} p7={:#x} tag={} vt_rva={:#x} | {}\n",
                spec.name, n, verdict, game, mine, a.p1, phase, a.p5, a.p6, a.p7, tag, st.vt_rva.load(Ordering::Relaxed), tr_fmt()));
    }
    if status_due(n) { write_status(); }
}

pub unsafe fn record_out(spec: &FnSpec, st: &Stat, out: usize, mine: Option<MpOut>, a: &ScorerArgs) {
    let n = st.n.fetch_add(1, Ordering::Relaxed) + 1;
    if n == 1 { if let Some(w) = world::World::from_holder(a.p6) { laycheck::run_once(&w); } }
    let verdict = match mine {
        None => { st.na.fetch_add(1, Ordering::Relaxed); "NA" }
        Some(ref m) => match compare_out(out, m) {
            Some(true) => { st.ok.fetch_add(1, Ordering::Relaxed); "OK" }
            Some(false) => { st.diff.fetch_add(1, Ordering::Relaxed); "DIFF" }
            None => { st.na.fetch_add(1, Ordering::Relaxed); "NA(read)" }
        },
    };
    // 분기 태그 집계(t3 상위 바이트) + 태그별 OK 표본 ≤4줄(DIFF 표본과 나란히 비교하려고)
    let tag = tag_of(tr_get(3));
    let mut tag_sample = false;
    if tag != 0 {
        match verdict { "OK" => { st.tag_ok[tag].fetch_add(1, Ordering::Relaxed); } "DIFF" => { st.tag_diff[tag].fetch_add(1, Ordering::Relaxed); } _ => {} }
        if verdict == "OK" && st.tag_logged[tag].fetch_add(1, Ordering::Relaxed) < 4 { tag_sample = true; }
    }
    if tag_sample || sample_line(st, n, verdict) {
        let gcode = rd_u64(out).unwrap_or(u64::MAX);
        append_direct(&format!("judge_{}.txt", spec.name),
            &format!("[{} #{}] {} game_code={} mine=[{}] | out={} | p2={:#x} p5={:#x} p6={:#x} p7={:#x} | {}\n",
                spec.name, n, verdict, gcode, mine.as_ref().map(fmt_writes).unwrap_or_else(|| "None".into()), hex30(out), a.p2, a.p5, a.p6, a.p7, tr_fmt()));
    }
    if status_due(n) { write_status(); }
}

pub fn write_status() {
    let lay = match laycheck::STATE.load(Ordering::Relaxed) { 0 => "미실행", 1 => "PASS", 2 => "FAIL(live 차단)", _ => "실행불가" };
    let mut s = format!("=== judge 계층 검증 누적 (게임 {}) judge_verify={} judge_live={} layout={} ===\n", GAME_VER, tune("judge_verify", 0), tune("judge_live", 0), lay);
    for (name, st) in stats() {
        s.push_str(&format!("{:<20} entered={} n={} ok={} diff={} na={} live={} knob_eff={} | vt_rva={:#x}\n", name,
            st.entered.load(Ordering::Relaxed), st.n.load(Ordering::Relaxed), st.ok.load(Ordering::Relaxed), st.diff.load(Ordering::Relaxed),
            st.na.load(Ordering::Relaxed), st.live.load(Ordering::Relaxed), st.knob_eff.load(Ordering::Relaxed), st.vt_rva.load(Ordering::Relaxed)));
        let tags: Vec<String> = (1..8).filter(|&t| st.tag_ok[t].load(Ordering::Relaxed) + st.tag_diff[t].load(Ordering::Relaxed) > 0)
            .map(|t| format!("tag{}: ok={} diff={}", t, st.tag_ok[t].load(Ordering::Relaxed), st.tag_diff[t].load(Ordering::Relaxed))).collect();
        if !tags.is_empty() { s.push_str(&format!("{:<20}   분기별 | {}\n", "", tags.join(" | "))); }
    }
    s.push_str("판정: diff=0 && na=0 이면 그 함수 DIFF=0(이번 판 표본 한정). na>0 = 가드 경로/콜리 캡처 없음 → judge_<fn>.txt 의 NA 줄 확인. ability_pick 은 캡처 전용(n=호출 수).\n");
    if let Some(p) = pth("judge_status.txt") { let _ = fs::write(p, s); }
}

static INSTALLED: AtomicBool = AtomicBool::new(false);

unsafe fn install_one(log: &mut String, spec: &FnSpec, orig_slot: &AtomicUsize, cap: usize, kind: &str) {
    match hook::install_wrap_bytes(spec.rva, spec.prolog, cap) {
        Ok(orig) => { orig_slot.store(orig, Ordering::Relaxed);
                      log.push_str(&format!("[judge] {} {} OK @rva {:#x} (orig_len={} sym={})\n", spec.name, kind, spec.rva, spec.prolog.len(), spec.sym)); }
        Err(e) => log.push_str(&format!("[judge] {} {} 실패: {} @rva {:#x}\n", spec.name, kind, e, spec.rva)),
    }
}

/// 로드 시점 1회(훅 설치 블록 끝, cfg 로드 후). 실패해도 게임 무영향(미설치 = 원본).
pub unsafe fn install() {
    if INSTALLED.swap(true, Ordering::Relaxed) { return; }
    let verify = tune("judge_verify", 0) != 0;
    // 판단 파일은 프로세스마다 새로(누적되면 지난 판 DIFF 가 섞여 오독 — 03:05 실사고)
    for s in ALL { if let Some(p) = pth(&format!("judge_{}.txt", s.name)) { let _ = fs::remove_file(p); } }
    if let Some(p) = pth("judge_layout.txt") { let _ = fs::remove_file(p); }
    let mut log = format!("judge 계층: 게임 {} · 등록 {}함수 · judge_verify={} judge_live={}\n", GAME_VER, ALL.len(), verify as u8, tune("judge_live", 0));
    if verify {
        install_one(&mut log, &STEAL_SCORE, &steal_hook::ORIG, steal_hook::wrap as *const () as usize, "wrap");
        // ★캡처를 핸들러보다 먼저 설치 — 핸들러 wrap 이 부르는 게임 원본이 콜리로 진입할 때 캡처가 살아 있어야 한다.
        install_one(&mut log, &ABILITY_PICK, &cap_ability_pick::ORIG, cap_ability_pick::wrap as *const () as usize, "capture");
        install_one(&mut log, &RECENTLY_SEEN, &cap_recent_seen::ORIG, cap_recent_seen::wrap as *const () as usize, "capture");
        install_one(&mut log, &EPIC_HUNT_BATTLE, &epic_hb_hook::ORIG, epic_hb_hook::wrap as *const () as usize, "wrap-out");
        install_one(&mut log, &SERPEN_HUNT_BATTLE, &serpen_hb_hook::ORIG, serpen_hb_hook::wrap as *const () as usize, "wrap-out");
        install_one(&mut log, &PASSIVE_LINE, &passive_line_hook::ORIG, passive_line_hook::wrap as *const () as usize, "wrap-out");
    } else {
        log.push_str("[judge] judge_verify=0 → 훅 미설치(원본)\n");
    }
    append_log(&log);
    if let Some(p) = pth("judge_install.txt") { let _ = fs::write(p, &log); }
    write_status();
}
