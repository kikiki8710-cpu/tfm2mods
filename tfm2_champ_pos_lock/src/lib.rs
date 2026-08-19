//! tfm2_champ_pos_lock — 특정 챔피언을 특정 포지션에서만 쓰게 제한.
//! ===========================================================================
//! 축 1 (이 파일, SDK 정공법): ModDraftScoreHook.score_pick 으로 AI 픽 게이트.
//!   제한 챔프끼리 허용 포지션 매칭(홀 조건)이 깨지는 픽 후보를 Replace(-1e9)로
//!   차단 → 상대 AI·코치 위임·백그라운드 리그 전부 recommend_pick 경유라 적용됨.
//!   (RE 근거: ANA\discovered-banpick-ai.md §9 — DraftScoreHook 는 픽 점수 후처리,
//!    적용 후 내림차순 정렬 → top-K 풀이라 -1e9 는 풀에 못 든다.)
//! 축 2 (hooks.rs): 최종 라인업(참가자레코드 pos)을 허용 포지션으로 강제 교정 —
//!   유저 수동 픽/스왑까지 커버. RE(0.5.5) 확정 지점에 detour.
//!
//! 설정 = mods\tfm2_champ_pos_lock\tfm2_champ_pos_lock.cfg (없으면 자동 생성).
//! 챔피언 id 목록 = debug=1 후 mods 폴더 champ_pos_lock_champions.txt.
//! ===========================================================================

use mod_api::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

mod config;
mod hooks;

use config::MASK_ALL;

pub(crate) const MOD_ID: &str = "tfm2_champ_pos_lock";

// build_inj.ps1 신원 검증용 — dll 안에 lib.rs 절대경로 문자열 필요(stale/타모드 차단).
#[no_mangle]
pub extern "C" fn tfm2_champ_pos_lock_src_id() -> *const u8 {
    concat!(file!(), "\0").as_bytes().as_ptr()
}

// ── 게임 exe 기준 모드 폴더 (설치위치 무관 — 경로 하드코딩 금지 규칙) ──
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleFileNameW(h: usize, buf: *mut u16, n: u32) -> u32;
}

pub(crate) fn mod_dir() -> Option<String> {
    let mut buf = [0u16; 512];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), 512) } as usize;
    if n == 0 || n >= 512 {
        return None;
    }
    let exe = String::from_utf16_lossy(&buf[..n]);
    let dir = std::path::Path::new(&exe).parent()?;
    Some(format!("{}\\mods\\{}", dir.display(), MOD_ID))
}

// ── 챔피언 인덱스 → 이름/허용마스크 (post_update 1회 캡처) ──────────────────
// ctx candidate 인덱스 = db.available_champions 순서(모드 챔프는 그 뒤) 전제 —
// tfm2_banpick_order draft_ai 와 동일 전제(디버그로 검증된 바 있음). 목록 밖
// 인덱스(모드 챔프 등)는 unrestricted(MASK_ALL) 취급.
static NAMES: OnceLock<Vec<String>> = OnceLock::new(); // 원본 표기(덤프용)
static MASKS: OnceLock<Vec<u8>> = OnceLock::new(); // 인덱스별 허용 포지션 마스크

static CNT_VETO: AtomicU64 = AtomicU64::new(0);
static CNT_SEEN: AtomicU64 = AtomicU64::new(0);
static FLUSH_TICK: AtomicU64 = AtomicU64::new(0);

const POS_NAMES: [&str; 5] = ["top", "jungle", "mid", "bottom", "support"];

fn mask_str(m: u8) -> String {
    (0..5)
        .filter(|p| m & (1 << p) != 0)
        .map(|p| POS_NAMES[p])
        .collect::<Vec<_>>()
        .join(",")
}

/// 제한 챔프 마스크들에 서로 다른 포지션을 하나씩 줄 수 있는가 (5×5 이하 백트래킹).
/// 무제한 챔프는 전포지션 가능이라 매칭 성립에 영향 없음(홀 조건 축소).
fn feasible(masks: &mut Vec<u8>) -> bool {
    if masks.len() > 5 {
        return false;
    }
    masks.sort_by_key(|m| m.count_ones()); // 선택지 좁은 것부터
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

// ── AI 픽 게이트 (공식 확장점 — detour 불요) ───────────────────────────────
#[derive(Debug)]
struct PosLockDraftAi;

impl ModDraftScoreHook for PosLockDraftAi {
    fn id(&self) -> &str {
        "tfm2_champ_pos_lock.pick_gate"
    }

    fn score_pick(
        &self,
        ctx: &DraftScoreContext,
        candidate: usize,
        _base_score: f32,
    ) -> DraftScoreDecision {
        let cfg = config::get();
        if !cfg.enabled || !cfg.ai_pick_gate || cfg.locks.is_empty() {
            return DraftScoreDecision::Pass;
        }
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let masks = MASKS.get()?;
            let cand = masks.get(candidate).copied().unwrap_or(MASK_ALL);
            CNT_SEEN.fetch_add(1, Ordering::Relaxed);
            // 후보가 무제한이면 어떤 픽 상태에서도 매칭을 깨지 않는다(전포지션 후보).
            if cand == MASK_ALL {
                return None;
            }
            let mut pins: Vec<u8> = Vec::with_capacity(6);
            pins.push(cand);
            for &i in ctx.ally_pick {
                let m = masks.get(i).copied().unwrap_or(MASK_ALL);
                if m != MASK_ALL {
                    pins.push(m);
                }
            }
            if feasible(&mut pins) {
                None
            } else {
                Some(())
            }
        }));
        match r {
            Ok(Some(())) => {
                CNT_VETO.fetch_add(1, Ordering::Relaxed);
                if cfg.debug {
                    let name = NAMES
                        .get()
                        .and_then(|v| v.get(candidate))
                        .map(|s| s.as_str())
                        .unwrap_or("?");
                    config::dlog(&format!(
                        "pick_veto: {name} (idx={candidate}) — 허용 포지션 매칭 불가 (ally_pick={:?})",
                        ctx.ally_pick
                    ));
                }
                DraftScoreDecision::Replace(-1.0e9)
            }
            _ => DraftScoreDecision::Pass,
        }
    }
}

// ── 진입 ──────────────────────────────────────────────────────────────────
struct PosLockExt;

impl ModExtension for PosLockExt {
    fn post_update(&self, scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let cfg = config::get();
            if !cfg.enabled {
                return;
            }
            // 챔피언 인덱스 테이블 1회 캡처 (관리화면 프레임에서도 Scene::InGame 매치).
            if MASKS.get().is_none() {
                if let Scene::InGame { data } = scene {
                    let db = data.db();
                    if !db.available_champions.is_empty() {
                        let ids: Vec<String> = db.available_champions.clone();
                        let masks: Vec<u8> = ids
                            .iter()
                            .map(|n| cfg.mask_of(&n.to_ascii_lowercase()).unwrap_or(MASK_ALL))
                            .collect();
                        // cfg 에 있는데 챔프 목록에 없는 lock 경고 + 챔프 목록 덤프
                        if cfg.debug {
                            let lower: Vec<String> =
                                ids.iter().map(|s| s.to_ascii_lowercase()).collect();
                            for (n, m) in &cfg.locks {
                                if !lower.iter().any(|x| x == n) {
                                    config::dlog(&format!(
                                        "경고: lock 챔피언 '{n}'({}) 이 available_champions 에 없음",
                                        mask_str(*m)
                                    ));
                                }
                            }
                            let mut s = String::from(
                                "# 챔피언 id 목록 (lock=<id>:<pos> 의 id 로 사용)\n",
                            );
                            for (i, id) in ids.iter().enumerate() {
                                let m = masks[i];
                                if m != MASK_ALL {
                                    s.push_str(&format!("{id}  [lock: {}]\n", mask_str(m)));
                                } else {
                                    s.push_str(&format!("{id}\n"));
                                }
                            }
                            if let Some(d) = mod_dir() {
                                let _ = std::fs::write(
                                    format!("{d}\\champ_pos_lock_champions.txt"),
                                    s,
                                );
                            }
                            for l in &cfg.load_log {
                                config::dlog(&format!("cfg: {l}"));
                            }
                            config::dlog(&format!("캡처: 챔프 {}개", ids.len()));
                        }
                        let _ = NAMES.set(ids);
                        let _ = MASKS.set(masks);
                    }
                }
            }
            // 훅 설치 — MASKS 캡처 후 1회 (late install, §3)
            if MASKS.get().is_some() {
                hooks::install_once();
            }
            // 디버그 카운터 주기 flush (~10초)
            if cfg.debug {
                let t = FLUSH_TICK.fetch_add(1, Ordering::Relaxed);
                if t % 600 == 599 {
                    config::dlog(&format!(
                        "counters: seen={} veto={} mask_call={} mask_adj={} hookA={}",
                        CNT_SEEN.load(Ordering::Relaxed),
                        CNT_VETO.load(Ordering::Relaxed),
                        hooks::CNT_MASK_CALL.load(Ordering::Relaxed),
                        hooks::CNT_MASK_ADJ.load(Ordering::Relaxed),
                        hooks::INSTALL_STATE.load(Ordering::Relaxed)
                    ));
                }
            }
        }));
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    config::load();
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(PosLockExt);
    reg.add_draft_score_hook(PosLockDraftAi);
    reg
}

declare_mod!(init);
