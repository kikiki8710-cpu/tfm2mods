//! save — 전술 단계 선택을 세이브마다 저장(stable mod save data). 선례 = tfm2_champ_pos_lock `save_tick`.
//! · 쓰기: 모달 '확인' → `request(본문)` → 다음 프레임부터 `save_can_write` 될 때 기록(안 되면 계속 재시도).
//! · 읽기: 세이브에 들어온 뒤 1회. 로드 직후엔 None 이 나올 수 있어 240프레임까지 기다린 뒤에야 "설정 없음"(전부 기본)으로 본다.
//! · 세이브 밖으로 나가면(scene ≠ InGame) 선택을 비운다 — 다른 세이브로 새지 않게.
//! 본문 형식 = `tactics::state_text()`(줄마다 "<전술 id> <칸 6개>").
use crate::tactics;
use mod_api_stable::StableClient;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed};
use std::sync::Mutex;

const SAVE_KEY: &str = "detail_tactics";
const SAVE_NS_VERSION: usize = 1;
const SAVE_MISS_GRACE: usize = 240;

static LOADED: AtomicBool = AtomicBool::new(false);
static MISS: AtomicUsize = AtomicUsize::new(0);
static PENDING: Mutex<Option<String>> = Mutex::new(None);

pub fn loaded() -> bool { LOADED.load(Relaxed) }

/// 모달 '확인' — 단계는 이미 tactics 에 반영된 상태에서 부른다.
pub fn request(body: String) { *PENDING.lock().unwrap_or_else(|e| e.into_inner()) = Some(body); }

/// 세이브 안(InGame)에서 매 프레임.
pub fn tick(ctx: &mut StableClient<'_>) {
    let pending = PENDING.lock().unwrap_or_else(|e| e.into_inner()).take();
    // QA still reads this save and verifies the roster; discard the taken write atomically.
    #[cfg(feature="replay_ab")]
    let pending = if crate::replay_ab::active() { None } else { pending };
    if let Some(body) = pending {
        if ctx.save_can_write() {
            ctx.save_set_version(SAVE_NS_VERSION);
            let ok = ctx.save_set_string(SAVE_KEY, &body);
            crate::log(&format!("세이브 기록 {}: {}B", if ok { "OK" } else { "거부" }, body.len()));
            if ok { LOADED.store(true, Relaxed); }
            else { *PENDING.lock().unwrap_or_else(|e| e.into_inner()) = Some(body); }
        } else {
            *PENDING.lock().unwrap_or_else(|e| e.into_inner()) = Some(body); // 다음 프레임 재시도
        }
    }
    if LOADED.load(Relaxed) { return; }
    match ctx.save_get_string(SAVE_KEY) {
        Some(txt) => {
            let n = tactics::apply_state_text(&txt);
            LOADED.store(true, Relaxed);
            let miss = MISS.swap(0, Relaxed);
            crate::log(&format!("세이브 설정 로드: 전술 {n}개 (대기 {miss}프레임)"));
        }
        None => {
            let n = MISS.fetch_add(1, Relaxed) + 1;
            if n >= SAVE_MISS_GRACE {
                tactics::clear_levels();
                LOADED.store(true, Relaxed);
                crate::log(&format!("이 세이브엔 상세전술 설정 없음({n}프레임 확인) → 전부 기본(원본)"));
            }
        }
    }
}

/// 세이브 밖으로 나감.
pub fn leave() {
    MISS.store(0, Relaxed);
    *PENDING.lock().unwrap_or_else(|e| e.into_inner()) = None;
    if LOADED.swap(false, Relaxed) {
        tactics::set_armed(false);
        tactics::clear_levels();
        tactics::set_my_athletes(&[]);
        crate::log("세이브 밖으로 나감 → 선택·내 팀 비움");
    }
}
