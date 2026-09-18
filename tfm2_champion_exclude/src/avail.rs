//! avail — 현재 세이브의 `available_champions`(출시된 챔피언 id) 읽기. stable API 에 열거 슬롯이 없어(0.6.0 실측:
//! DataVtableV1 엔 champion_count/champion_name_at = registry 전체뿐) `ClientDatabase` raw 읽기로 보충한다.
//! 포인터 = `cdb::client_db(ctx)`(자가검증 포함). 오프셋 = 0.6.0 RE(REPORT\tfm2_champion_exclude\RE\2026-09-17_*.md).
//! 검증: len ≤ registry, 원소 전부 registry id → 하나라도 어긋나면 None(레이아웃 stale = 폴백 경로).
//! `OFF_AVAIL_CAP == 0` 이면 미확정 상태 — 진단 스캔으로 후보 오프셋을 로그에 남긴다(1회). 검증 실패 시에도 스캔 1회.
use crate::{cdb, hook};
use mod_api_stable::StableClient;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};

/// ClientDatabase 기준 available_champions Vec<String> 의 cap 오프셋(ptr=+8, len=+0x10). 0 = 미확정.
/// ★0.6.0 = 0xe740 (ptr 0xe748 / len 0xe750) — ghidra-re 2026-09-17: champion_info_ui FUN_1423f5680·챔피언 탭 FUN_1423843e0 교차(0.5.8 tcx 0xdcc0 대응).
pub const OFF_AVAIL_CAP: usize = 0xe740;
static SCAN_DONE: AtomicBool = AtomicBool::new(false);
static FAIL_LOGGED: AtomicBool = AtomicBool::new(false);

pub fn read(ctx: &StableClient<'_>, registry: &HashSet<String>) -> Option<HashSet<String>> {
    let db = cdb::client_db(ctx)?;
    if OFF_AVAIL_CAP == 0 {
        if !SCAN_DONE.swap(true, Ordering::Relaxed) { diag_scan(db, registry); }
        return None;
    }
    let v = unsafe { hook::read_string_vec(db + OFF_AVAIL_CAP, 1024) };
    match v {
        Some(v) if !v.is_empty() && v.len() <= registry.len() && v.iter().all(|s| registry.contains(&s.to_ascii_lowercase())) => Some(v.into_iter().map(|s| s.to_ascii_lowercase()).collect()),
        other => {
            if !FAIL_LOGGED.swap(true, Ordering::Relaxed) {
                crate::log(&format!("available_champions raw read FAILED @db+0x{:x}: {:?} registry={} (layout stale?) - fallback to seen cache", OFF_AVAIL_CAP, other.as_ref().map(|v| v.iter().take(5).cloned().collect::<Vec<_>>()), registry.len()));
                if !SCAN_DONE.swap(true, Ordering::Relaxed) { diag_scan(db, registry); }
            }
            None
        }
    }
}

/// 진단: ClientDatabase 앞 0x20000 을 8B 보폭으로 훑어 "원소 전부 registry id 인 Vec<String>" 후보를 로그(오프셋·len·앞 3개).
fn diag_scan(db: usize, registry: &HashSet<String>) {
    let mut hits: Vec<String> = Vec::new();
    let mut off = 0usize;
    while off < 0x20000 && hits.len() < 12 {
        let addr = db + off;
        if unsafe { !cdb::readable(addr, 0x18) } { off += 0x1000; continue; }
        let cap = unsafe { cdb::rd_u64(addr) }.unwrap_or(0) as usize;
        let len = unsafe { cdb::rd_u64(addr + 0x10) }.unwrap_or(0) as usize;
        if len >= 3 && len <= cap && cap <= 4096 {
            if let Some(v) = unsafe { hook::read_string_vec(addr, 1024) } {
                if v.len() == len && v.iter().all(|s| registry.contains(&s.to_ascii_lowercase())) {
                    hits.push(format!("+0x{:x}(len={} cap={} [{}])", off, len, cap, v.iter().take(3).cloned().collect::<Vec<_>>().join(",")));
                }
            }
        }
        off += 8;
    }
    crate::log(&format!("avail diag scan: registry={} hits={:?}", registry.len(), hits));
}
