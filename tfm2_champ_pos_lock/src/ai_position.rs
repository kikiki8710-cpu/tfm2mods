//! ai_position — ★0.6.4(ABI 10) AI 팀 포지션 배정 = stable `StableDraftHook::decide_positions`.
//! 구 RVA 2축(`legacy_assign` hookA 마스크 detour · `ai_swap` compute_rule_swap_order detour)을 **완전 대체**(유저 결정 2026-10-14).
//!   ⟹ 패치마다 재핀할 RVA 5 → 3(auto_swap·pick_click·swap_confirm 만 남음 — 내 팀 스왑·UI 클릭 소비는 여전히 stable 미노출).
//! 계약(SDK traits.rs): 픽이 끝난 뒤 AI 팀마다 1회(`ctx.phase()==Position`) · 반환 = `slot_lanes()` 슬롯 순으로 `ally_picks()` 인덱스 ·
//!   완전한 순열이 아니면 무시 · `None` = 게임 배정 유지.
//! ⚠ctx 에 **게임이 원래 정한 배치가 없다** → 구 ai_swap 처럼 "최소 수정"을 못 한다. 그래서
//!   ① 규칙(포지션 목록)에 맞게 앉는 인원 최대 ② 그 라인 선수(`ally_athletes()[slot]`)의 해당 챔피언 숙련도 합 최대 ③ 픽 순서 유지
//!   순으로 고른다. 숙련도 = record API(`champion_proficiency`)를 클라 post_update 에서 조금씩 캐시(draft hook 스레드엔 record API 없음).
//! 무개입 조건: 설정 꺼짐(`enabled`·`swap_force=0`) · 제한 목록 없음 · 이 팀 픽 전부 무제한 · 형태 이상.
//! 안전: 순수 계산(raw 메모리 접근 없음) · 호출부(draft.rs)에서 catch_unwind · 캐시 락은 짧게.
use crate::config::{self, MASK_ALL};
use mod_api_stable::{DraftPhaseV1, RecordKindV1, StableClient, StableDraftContext};
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

pub static CNT_CALL: AtomicU64 = AtomicU64::new(0);
pub static CNT_SET: AtomicU64 = AtomicU64::new(0);
pub static CNT_NOREST: AtomicU64 = AtomicU64::new(0);
pub static CNT_SKIP: AtomicU64 = AtomicU64::new(0);
pub static CNT_PROF_MISS: AtomicU64 = AtomicU64::new(0);
static LAST_LOG: Mutex<Option<String>> = Mutex::new(None);

// ── 숙련도 캐시: athlete id → [(챔피언 키 소문자, 값)] ──
static PROF: Mutex<Option<HashMap<usize, Vec<(String, i64)>>>> = Mutex::new(None);
/// decide 가 캐시에 없어서 못 쓴 선수(다음 틱에 우선 채움).
static WANT: Mutex<BTreeSet<usize>> = Mutex::new(BTreeSet::new());
static QUEUE: Mutex<VecDeque<usize>> = Mutex::new(VecDeque::new());
static REFILL_AT: AtomicU64 = AtomicU64::new(0);
static SAMPLE_LOGGED: AtomicU64 = AtomicU64::new(0);
const FETCH_PER_TICK: usize = 8;
const REFILL_FRAMES: u64 = 36_000; // 전수 재채움 주기(≈10분 @60fps) — 숙련도는 경기·훈련으로 천천히 변한다

/// `{"<champ>":{"floor":..,"value":N},...}` 또는 `{"<champ>":N}` → [(키 소문자, N)].
fn parse_prof(j: &str) -> Vec<(String, i64)> {
    let b = j.as_bytes();
    let mut out = Vec::new();
    let (mut i, mut depth) = (0usize, 0i32);
    let mut key: Option<String> = None;
    while i < b.len() {
        match b[i] {
            b'"' => {
                let s = i + 1; let mut e = s;
                while e < b.len() && b[e] != b'"' { if b[e] == b'\\' { e += 1; } e += 1; }
                if depth == 1 { key = Some(j[s..e.min(b.len())].to_ascii_lowercase()); }
                i = e + 1; continue;
            }
            b'{' => {
                depth += 1;
                if depth == 2 {
                    if let Some(k) = key.take() {
                        let end = j[i..].find('}').map(|x| i + x).unwrap_or(b.len());
                        if let Some(p) = j[i..end].find("\"value\"") {
                            let rest = j[i + p + 7..end].trim_start().trim_start_matches(':').trim_start();
                            let num: String = rest.chars().take_while(|c| c.is_ascii_digit() || *c == '-').collect();
                            if let Ok(v) = num.parse::<i64>() { out.push((k, v)); }
                        }
                    }
                }
            }
            b'}' => depth -= 1,
            b':' if depth == 1 => {
                let rest = j[i + 1..].trim_start();
                if rest.starts_with(|c: char| c.is_ascii_digit() || c == '-') {
                    let num: String = rest.chars().take_while(|c| c.is_ascii_digit() || *c == '-').collect();
                    if let (Some(k), Ok(v)) = (key.take(), num.parse::<i64>()) { out.push((k, v)); }
                }
            }
            _ => {}
        }
        i += 1;
    }
    out
}

/// 클라 post_update(InGame)에서 매 프레임 호출 — 틱당 최대 FETCH_PER_TICK 명 숙련도 채움.
pub fn cache_tick(ctx: &StableClient<'_>, frame: u64) {
    let cfg = config::get();
    if !cfg.enabled || cfg.swap_force == 0 || !config::any_restricted() { return; }
    {
        let mut q = QUEUE.lock().unwrap_or_else(|e| e.into_inner());
        if q.is_empty() && frame >= REFILL_AT.load(Ordering::Relaxed) {
            q.extend(ctx.record_ids(RecordKindV1::Athlete));
            REFILL_AT.store(frame + REFILL_FRAMES, Ordering::Relaxed);
        }
    }
    let mut ids: Vec<usize> = { let mut w = WANT.lock().unwrap_or_else(|e| e.into_inner()); let v: Vec<usize> = w.iter().take(FETCH_PER_TICK).copied().collect(); for x in &v { w.remove(x); } v };
    if ids.len() < FETCH_PER_TICK {
        let mut q = QUEUE.lock().unwrap_or_else(|e| e.into_inner());
        while ids.len() < FETCH_PER_TICK { match q.pop_front() { Some(a) => ids.push(a), None => break } }
    }
    if ids.is_empty() { return; }
    let mut got: Vec<(usize, Vec<(String, i64)>)> = Vec::with_capacity(ids.len());
    for a in ids {
        let Some(j) = ctx.record_get_json(RecordKindV1::Athlete, a, "champion_proficiency") else { continue };
        let v = parse_prof(&j);
        if SAMPLE_LOGGED.fetch_add(1, Ordering::Relaxed) == 0 { config::dlog(&format!("ai_position 숙련도 샘플 athlete={} n={} {:?}", a, v.len(), &v[..v.len().min(4)])); }
        got.push((a, v));
    }
    let mut g = PROF.lock().unwrap_or_else(|e| e.into_inner());
    let m = g.get_or_insert_with(HashMap::new);
    for (a, v) in got { m.insert(a, v); }
}

fn fits(m: u8, lane: u32) -> bool { m == MASK_ALL || (lane < 5 && m & (1 << lane) != 0) }

/// draft hook(서버 워커)에서 호출. `Some(order)` = order[slot] = ally_picks 인덱스.
pub fn decide(ctx: &StableDraftContext<'_>) -> Option<Vec<usize>> {
    CNT_CALL.fetch_add(1, Ordering::Relaxed);
    let cfg = config::get();
    if !cfg.enabled || cfg.swap_force == 0 || !config::any_restricted() { return None; }
    if ctx.phase() != Some(DraftPhaseV1::Position) { CNT_SKIP.fetch_add(1, Ordering::Relaxed); return None; }
    let lanes: Vec<u32> = ctx.slot_lanes().iter().map(|l| l.code()).collect();
    let picks = ctx.ally_picks();
    let n = lanes.len();
    if n == 0 || n > 5 || picks.len() != n { CNT_SKIP.fetch_add(1, Ordering::Relaxed); return None; }
    let names: Vec<String> = picks.iter().map(|&id| ctx.champion_name(id).unwrap_or("").to_ascii_lowercase()).collect();
    if names.iter().any(|s| s.is_empty()) { CNT_SKIP.fetch_add(1, Ordering::Relaxed); return None; }
    let masks: Vec<u8> = names.iter().map(|s| crate::mask_of(s)).collect();
    if masks.iter().all(|&m| m == MASK_ALL) { CNT_NOREST.fetch_add(1, Ordering::Relaxed); return None; }
    // 숙련도 p[pick][slot]
    let athletes = ctx.ally_athletes();
    let mut prof = vec![vec![0i64; n]; n];
    let mut known = 0usize;
    {
        let g = PROF.lock().unwrap_or_else(|e| e.into_inner());
        let mut miss: Vec<usize> = Vec::new();
        for s in 0..n {
            let Some(&a) = athletes.get(s) else { continue };
            match g.as_ref().and_then(|m| m.get(&a)) {
                Some(list) => { known += 1; for (i, nm) in names.iter().enumerate() { prof[i][s] = list.iter().find(|(k, _)| k == nm).map(|x| x.1).unwrap_or(0); } }
                None => miss.push(a),
            }
        }
        drop(g);
        if !miss.is_empty() { CNT_PROF_MISS.fetch_add(1, Ordering::Relaxed); let mut w = WANT.lock().unwrap_or_else(|e| e.into_inner()); for a in miss { if w.len() < 256 { w.insert(a); } } }
    }
    // n! 전수(≤120): (규칙 맞춤 수, 숙련도 합, 픽 순서 일치 수) 최대
    let mut idx: Vec<usize> = (0..n).collect();
    let mut best: ((usize, i64, usize), Vec<usize>) = ((0, i64::MIN, 0), idx.clone());
    fn perm(k: usize, idx: &mut Vec<usize>, eval: &dyn Fn(&[usize]) -> (usize, i64, usize), best: &mut ((usize, i64, usize), Vec<usize>)) {
        if k <= 1 { let sc = eval(idx); if sc > best.0 { *best = (sc, idx.clone()); } return; }
        for i in 0..k { perm(k - 1, idx, eval, best); if k % 2 == 0 { idx.swap(i, k - 1); } else { idx.swap(0, k - 1); } }
    }
    let eval = |o: &[usize]| -> (usize, i64, usize) {
        let matched = (0..n).filter(|&s| fits(masks[o[s]], lanes[s])).count();
        let ps: i64 = (0..n).map(|s| prof[o[s]][s]).sum();
        let agree = (0..n).filter(|&s| o[s] == s).count();
        (matched, ps, agree)
    };
    perm(n, &mut idx, &eval, &mut best);
    let ((matched, ps, _), order) = best;
    CNT_SET.fetch_add(1, Ordering::Relaxed);
    if cfg.debug || cfg.log_lineups {
        let lane_nm = |l: u32| config::POS_NAMES.get(l as usize).copied().unwrap_or("?");
        let seat: Vec<String> = (0..n).map(|s| format!("{}={}", lane_nm(lanes[s]), names[order[s]])).collect();
        if let Ok(mut g) = LAST_LOG.try_lock() { *g = Some(format!("aipos: team={:?} {} | 맞춤 {}/{} 숙련합 {} (숙련 캐시 {}/{}){}", ctx.ally_team(), seat.join(" "), matched, n, ps, known, n, if cfg.ai_observe_only { " [관찰만]" } else { "" })); }
    }
    if cfg.ai_observe_only { return None; }
    Some(order)
}

/// 메인 스레드 폴링: 마지막 결정 로그.
pub fn drain_log() -> Option<String> { LAST_LOG.try_lock().ok().and_then(|mut g| g.take()) }
pub fn counters() -> String {
    format!("aipos call={} set={} norest={} skip={} prof_miss={}", CNT_CALL.load(Ordering::Relaxed), CNT_SET.load(Ordering::Relaxed), CNT_NOREST.load(Ordering::Relaxed), CNT_SKIP.load(Ordering::Relaxed), CNT_PROF_MISS.load(Ordering::Relaxed))
}
