//! tfm2_champ_pos_lock 설정 (2본 분리).
//! ===========================================================================
//! ① 토글 = `tfm2_champ_pos_lock.cfg` (on/off·debug — 사람이 거의 안 건드림)
//! ② ★상태 = `champ_pos_lock_state.txt` (포지션별 허용 챔피언 화이트리스트)
//!    - 인게임 UI(환경설정 게임플레이 탭 → 포지션 제한)가 이 파일을 읽고 쓴다.
//!    - 형식: `<포지션> = <id>, <id>, …`  (포지션 = top/jungle/mid/bottom/support)
//!    - ★비어 있으면(줄 없음/우변 공백) 그 포지션은 **모든 챔피언 허용**.
//!    - 목록 = "현재 사용 가능한 챔피언"만(미출시 제외) — UI 가 available_champions 로 채운다.
//!
//! 챔피언이 포지션 P 에서 쓸 수 있는가 = allowed[P] 가 비었거나 그 안에 있으면 OK.
//! 이 판정을 챔피언별 5비트 마스크로 환산해 배정/픽 게이트가 소비한다(lib.rs·hooks.rs).
//! ===========================================================================

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{OnceLock, RwLock};

pub const MASK_ALL: u8 = 0b11111;
pub const POS_NAMES: [&str; 5] = ["top", "jungle", "mid", "bottom", "support"];
/// 한국어/약칭 → 포지션 인덱스
pub fn pos_index(tok: &str) -> Option<usize> {
    match tok.trim().to_ascii_lowercase().as_str() {
        "top" | "탑" | "0" => Some(0),
        "jungle" | "jg" | "정글" | "1" => Some(1),
        "mid" | "middle" | "미드" | "2" => Some(2),
        "bottom" | "bot" | "adc" | "원딜" | "바텀" | "3" => Some(3),
        "support" | "sup" | "서폿" | "서포터" | "4" => Some(4),
        _ => None,
    }
}

// ── 토글 (cfg) ──────────────────────────────────────────────────────────────
pub struct Cfg {
    pub enabled: bool,
    pub debug: bool,
    /// ★모든 매치(백그라운드 포함)의 완성된 5인 라인업을 champ_pos_lock_lineups.txt 에 기록.
    pub log_lineups: bool,
    /// 스왑 order 강제(0=끔 / 1=order[포지션]=픽인덱스 / 2=order[픽인덱스]=포지션). 방향 확정용.
    pub swap_force: u32,
    /// AI 픽 차단 (DraftScoreHook)
    pub ai_pick_gate: bool,
    /// AI 배정 마스크 강제 (hookA)
    pub ai_assign_mask: bool,
    /// 유저 픽 차단 (hookC — 피어리스처럼 회색+클릭불가)
    pub user_pick_block: bool,
    /// (예약·미구현) 최종 라인업 하드 강제
    pub enforce_lineup: bool,
    /// ★관찰 전용: score_pick 훅·로깅은 켜되 veto(Replace)는 안 함(항상 Pass). 대조실험용.
    pub ai_observe_only: bool,
    /// 옵션/팝업 노드 트리 덤프(디버그 — UI 주입점 파악용)
    pub dump_ui: bool,
    pub load_log: Vec<String>,
}

impl Cfg {
    fn empty() -> Self {
        Cfg {
            enabled: true,
            debug: false,
            log_lineups: false,
            swap_force: 1,
            ai_pick_gate: true,
            ai_assign_mask: true,
            user_pick_block: true,
            enforce_lineup: false,
            ai_observe_only: false,
            dump_ui: false,
            load_log: Vec::new(),
        }
    }
}

static CFG: OnceLock<Cfg> = OnceLock::new();
pub fn get() -> &'static Cfg {
    CFG.get_or_init(Cfg::empty)
}

// ── 상태: 포지션별 허용 챔피언 (state 파일 · UI 편집 대상) ───────────────────
#[derive(Default, Clone)]
pub struct PosState {
    /// allowed[pos] = 소문자 id 화이트리스트. **빈 Vec = 그 포지션 전 챔피언 허용.**
    pub allowed: [Vec<String>; 5],
}

impl PosState {
    /// 챔피언(소문자)의 허용 포지션 5비트 마스크.
    /// 이 포지션 화이트리스트 중 **지금 로스터에 실제로 있는** 챔프 수.
    ///   ⚠세이브/워크샵이 바뀌면 없어진 챔프 id 가 화이트리스트에 그대로 남는다
    ///     (state 파일은 세이브와 무관하게 유지된다). 그걸 세면 개수가 부풀어
    ///     "최소 선택 수 충족"으로 오판하고, 실제 풀은 말라 라인업이 깨진다.
    ///     (유저 제보 2026-08-23: 탑 9개만 켜져 있는데 "현재 선택 수 21")
    /// 이 포지션에 **명시로 지정된** 챔프 중 지금 로스터에 실재하는 수.
    /// ★`live_count` 와 구분할 것 — 저쪽은 "제한 판정용 풀"이라 미지정 챔프까지 더한다.
    ///   UI 의 "현재 선택 수" 는 사용자가 실제로 켠 개수여야 하므로 이 함수를 쓴다
    ///   (2026-09-02: 21개만 켰는데 94 로 보이던 오표시의 원인).
    pub fn named_count(&self, p: usize) -> usize {
        let g = ROSTER.read().unwrap_or_else(|e| e.into_inner());
        match g.as_ref() {
            Some(set) => self.allowed[p].iter().filter(|c| set.contains(*c)).count(),
            None => self.allowed[p].len(),
        }
    }

    pub fn live_count(&self, p: usize) -> usize {
        let g = ROSTER.read().unwrap_or_else(|e| e.into_inner());
        let named = match g.as_ref() {
            Some(set) => self.allowed[p].iter().filter(|c| set.contains(*c)).count(),
            None => self.allowed[p].len(), // 로스터 미캡처 → 보수적으로 전부 인정
        };
        // 명시 지정이 0 = 그 포지션엔 제한을 안 건 것 ⟹ 기존대로 0(=비활성).
        if named == 0 {
            return 0;
        }
        // ★★미지정 챔프도 이 포지션에서 쓸 수 있다(mask_of 가 MASK_ALL 로 정규화) ⟹ 풀에 포함.
        //   (2026-08-27) 이걸 빼면 "밴카드 5장+하드피어리스 → 포지션당 최소 20개" 같은 조건에서
        //   실제로는 쓸 챔프가 충분한데도 `pos_active=false` 가 되어 **제한이 통째로 꺼진다**
        //   (유저 제보: 밴카드 5장으로 바꾸니 제한이 안 걸림 — 지정 14 < 최소 20 이었다).
        // ⚠guard 를 여기서 재사용한다 — `ROSTER.read()` 를 중첩으로 잡으면
        //   같은 스레드 재귀 read 로 데드락 가능(std RwLock).
        let unassigned = match g.as_ref() {
            Some(set) => set
                .iter()
                .filter(|id| !(0..5).any(|q| self.allowed[q].iter().any(|x| x == *id)))
                .count(),
            None => 0,
        };
        named + unassigned
    }
    /// 이 포지션의 제한이 **실제로 적용되나**.
    ///   ★빈 화이트리스트 = 전체 허용. 그리고 **최소 선택 수 미달도 전체 허용으로 취급**한다
    ///     (유저 지시 2026-08-23). 최소 미달이면 밴/픽 몇 장에 풀이 말라 라인업이 성립 못 하고,
    ///     그때마다 fail-open 으로 널뛰느니 **처음부터 제한을 안 건 것**으로 보는 게 예측 가능하다.
    pub fn pos_active(&self, p: usize) -> bool {
        // ★★[2026-09-16 정정] ~~`live_count(p) >= min_required(style, ban)`~~ → **부분집합별 정확식**(`safety()`).
        //   구 공식은 포지션별 머릿수만 봐서 ①피어리스가 1 부족(이번 세트 상대 픽 누락) ②복수 라인 지정 시
        //   챔프를 여러 포지션에 중복 계산해 크게 과소(2라인·12/라인·피어리스를 "적용 가능"으로 통과) —
        //   적대 상대 시뮬 150행 실측(REPORT RE 09-16). 정확식은 슬랙≥0 이면 강제 위반 0.
        if self.named_count(p) == 0 {
            return false;
        }
        self.safety_cached().active[p]
    }
    /// ★★[2026-09-04 유저 지시 — 규칙 반전] **지정한 포지션에만 갈 수 있다.**
    ///   ~~구 규칙: 비활성 포지션의 비트를 항상 켰다~~ ⟹ 탑에만 21종을 지정하면 그 21종이
    ///   `MASK_ALL`(어디든 가능)이 되어, "탑 제한"이 사실상 **"탑은 이 21종만"** 만 뜻하고
    ///   **"이 21종은 탑만"** 은 뜻하지 않았다. 유저 의도는 후자를 **포함**한다:
    ///   "선택한 탑 제한 챔피언은 탑만 갈 수 있어야 해. 다른 포지션은 못 가고.
    ///    다른 포지션에도 갈 수 있게 하려면 그 포지션에서도 똑같이 골라야지."
    ///
    ///   새 규칙:
    ///     ① 어느 활성 포지션에든 지정됨 → **지정된 포지션 비트만** (다른 데는 못 감)
    ///     ② 어디에도 지정 안 됨      → **비활성(목록 없는) 포지션만** (목록 있는 자리엔 못 감)
    ///     ③ ②인데 전 포지션이 활성   → `MASK_ALL` fail-open
    ///        (그 챔프를 아예 못 뽑게 만들면 드래프트가 성립하지 않는다. 2026-08-27 사고와 같은 이유:
    ///         `helps(pinned, 0)` 이 항상 false 라 전부 회색 처리된다.)
    pub fn mask_of(&self, lower: &str) -> u8 {
        let mut designated = 0u8;
        let mut free = 0u8;
        for p in 0..5 {
            if !self.pos_active(p) {
                free |= 1 << p; // 목록이 없거나 최소 미달 = 제한 없음
            } else if self.allowed[p].iter().any(|x| x == lower) {
                designated |= 1 << p;
            }
        }
        if designated != 0 {
            return designated; // ①지정된 자리에만
        }
        if free != 0 {
            return free; // ②목록 있는 자리엔 못 감
        }
        MASK_ALL // ③전 포지션 활성인데 어디에도 없음 → fail-open
    }
    /// 제한이 하나라도 걸렸나(전부 빈/최소미달이면 모드 무효과).
    pub fn any_restricted(&self) -> bool {
        (0..5).any(|p| self.pos_active(p))
    }
}

// ── ★실효 밴픽 룰 캐시 (최소 선택 수 계산용) ────────────────────────────────
//   min_required 는 룰(클래식/피어리스/하드) + 실효 밴카드 수에 달려 있는데, 마스크를 만드는
//   시점엔 그 값을 인자로 받을 수 없다 ⟹ 밴픽 씬/설정 팝업에서 관측한 값을 여기 캐시한다.
//   값이 바뀌면 STATE_VER 를 올려 마스크 재계산(recompute_masks_if_needed)을 트리거한다.
static RULE_STYLE: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
/// 실효 밴카드 수 **+1**. `0` = 미관측(설정이 "자동"이고 밴픽 씬을 아직 못 봄).
///   ⚠+1 인코딩인 이유: **밴카드 0장도 유효한 설정**이라 0 을 "모름"으로 쓸 수 없다.
static RULE_BAN1: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn set_rule_inner(style: u8, enc: usize, persist: bool) {
    // ★한 번 확정된 값을 "모름"으로 되돌리지 않는다.
    //   설정 팝업은 db+0x720(원본 설정값)을 읽는데 "자동"이면 0 이라 None 이 온다 ⟹ 가드가 없으면
    //   팝업을 여는 것만으로 관측/복원된 실효값이 지워진다.
    if enc == 0 && RULE_BAN1.load(Ordering::Relaxed) != 0 {
        return;
    }
    let a = RULE_STYLE.swap(style, Ordering::Relaxed);
    let b = RULE_BAN1.swap(enc, Ordering::Relaxed);
    let _ = persist;
    if a != style || b != enc {
        STATE_VER.fetch_add(1, Ordering::Relaxed);
        if enc != 0 {
            // ★밴카드 자동 공식(SDK) 결과를 남긴다 — 5V5 는 로스터 40종 미만 2장 / 40 이상 3장.
            slog(&format!(
                "룰 확정: 밴카드 {}장 / 스타일 {} (0=클래식 1=피어리스 2=하드) → 포지션당 최소 {}개",
                enc - 1,
                style,
                min_required(style, enc - 1)
            ));
        }
        // ~~구: state 파일에 ban_eff 영속~~ → **2026-08-23 폐기**.
        //   SDK `ban_count_or_default` 로 관리화면 진입 즉시 계산되므로 저장할 이유가 없고,
        //   파일은 세이브 공용이라 다른 세이브의 값이 새어 들어오는 오염원이 된다.
    }
}
/// 룰 캐시 갱신. `ban = None` = 아직 실효 밴카드 수를 모른다(설정이 "자동", 경기 관측 전).
pub fn set_rule(style: u8, ban: Option<usize>) {
    set_rule_inner(style, ban.map(|b| b + 1).unwrap_or(0), true);
}
pub fn cur_rule() -> (u8, Option<usize>) {
    let e = RULE_BAN1.load(Ordering::Relaxed);
    (RULE_STYLE.load(Ordering::Relaxed), (e != 0).then(|| e - 1))
}
/// 지금 룰에서 포지션 하나가 갖춰야 할 최소 화이트리스트 크기.
///   ★미관측이면 `usize::MAX` — 어떤 화이트리스트도 못 미치므로 **전 포지션 제한 없음**이 된다.
pub fn cur_min_required() -> usize {
    match cur_rule() {
        (s, Some(b)) => min_required(s, b),
        _ => usize::MAX,
    }
}
/// 제한이 실제로 걸린 포지션 비트마스크(UI 표시·진단용).
pub fn active_pos_mask() -> u8 {
    with_state(|s| {
        let mut m = 0u8;
        for p in 0..5 {
            if s.pos_active(p) {
                m |= 1 << p;
            }
        }
        m
    })
}

/// 지금 게임에 실제로 존재하는 챔프 id(소문자) 집합. `None` = 아직 캡처 전.
static ROSTER: RwLock<Option<std::collections::HashSet<String>>> = RwLock::new(None);

/// 로스터 캡처(챔프 목록 확정 시 1회). 마스크·개수 판정이 이 집합으로 걸러진다.
pub fn set_roster(ids: &[String]) {
    let set: std::collections::HashSet<String> =
        ids.iter().map(|s| s.to_ascii_lowercase()).collect();
    ROSTER_VER.fetch_add(1, Ordering::Relaxed);
    *ROSTER.write().unwrap_or_else(|e| e.into_inner()) = Some(set);
    STATE_VER.fetch_add(1, Ordering::Relaxed); // 마스크 재계산 트리거
}

static STATE: RwLock<Option<PosState>> = RwLock::new(None);
static STATE_VER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub fn state_version() -> u64 {
    STATE_VER.load(Ordering::Relaxed)
}
/// 읽기 스냅샷(락 최소화 — 호출 측이 복제해 씀).
pub fn with_state<R>(f: impl FnOnce(&PosState) -> R) -> R {
    let g = STATE.read().unwrap_or_else(|e| e.into_inner());
    let empty = PosState::default();
    f(g.as_ref().unwrap_or(&empty))
}
pub fn any_restricted() -> bool {
    with_state(|s| s.any_restricted())
}
pub fn mask_of(lower: &str) -> u8 {
    with_state(|s| s.mask_of(lower))
}

/// UI/파일에서 새 상태 게시(버전 증가 → 마스크 재계산 트리거).
pub fn set_state(st: PosState) {
    *STATE.write().unwrap_or_else(|e| e.into_inner()) = Some(st);
    STATE_VER.fetch_add(1, Ordering::Relaxed);
}

fn mutate(f: impl FnOnce(&mut PosState)) {
    let mut g = STATE.write().unwrap_or_else(|e| e.into_inner());
    let st = g.get_or_insert_with(PosState::default);
    f(st);
    drop(g);
    STATE_VER.fetch_add(1, Ordering::Relaxed);
}
/// pos(0..4) 화이트리스트에서 챔피언(소문자) 켜고 끄기.
pub fn toggle(pos: usize, lower: &str) {
    if pos >= 5 {
        return;
    }
    mutate(|st| {
        if let Some(i) = st.allowed[pos].iter().position(|x| x == lower) {
            st.allowed[pos].remove(i);
        } else {
            st.allowed[pos].push(lower.to_string());
        }
    });
}
pub fn clear_pos(pos: usize) {
    if pos >= 5 {
        return;
    }
    mutate(|st| st.allowed[pos].clear());
}
pub fn set_pos(pos: usize, list: Vec<String>) {
    if pos >= 5 {
        return;
    }
    mutate(|st| st.allowed[pos] = list);
}
pub fn is_listed(pos: usize, lower: &str) -> bool {
    if pos >= 5 {
        return false;
    }
    with_state(|st| st.allowed[pos].iter().any(|x| x == lower))
}
pub fn pos_count(pos: usize) -> usize {
    if pos >= 5 {
        return 0;
    }
    // ★현재 로스터에 실제로 있는 것만 센다(없어진 챔프 id 는 UI 에서도 안 보이므로 세면 혼란).
    with_state(|st| st.named_count(pos))   // ★UI 표시용 = 실제로 켠 개수(풀 크기는 pos_pool)
}
/// 제한 판정에 실제로 쓰이는 **풀 크기**(명시 지정 + 미지정 챔프).
/// `pos_active` 가 최소 선택 수와 비교하는 값이 이것이다 — 화면에도 같이 보여 줘야
/// "21개만 켰는데 왜 최소 20을 넘지?" 가 설명된다.
pub fn pos_pool(pos: usize) -> usize {
    if pos >= 5 {
        return 0;
    }
    with_state(|st| st.live_count(pos))
}
/// ★이 포지션의 제한이 **실제로 적용되나** — 게이트와 화면이 같은 식을 쓰게 하는 단일 창구.
///   ⚠화면 경고를 `pos_count`(지정 수)로 판정하지 말 것: 게이트는 `pos_pool`(지정+미지정)로
///     판정하므로, 지정 수만 보면 **"제한 없음"이라 안내해 놓고 실제로는 제한이 걸리는**
///     구간(`pos_count < 최소 <= pos_pool`)이 생긴다(2026-09-04 발견 · 교훈 #4).
pub fn pos_active_of(pos: usize) -> bool {
    if pos >= 5 {
        return false;
    }
    with_state(|st| st.pos_active(pos))
}
/// 화이트리스트에 남아 있지만 지금 로스터엔 없는 챔프 수(pos 기준) — 안내 문구용.
pub fn pos_stale(pos: usize) -> usize {
    if pos >= 5 {
        return 0;
    }
    with_state(|st| st.allowed[pos].len().saturating_sub(st.named_count(pos)))
}
/// pos 와 챔프를 (직·간접) 공유하는 제한 포지션들의 컴포넌트(pos 포함) + 그 합집합 크기.
/// ★겹침은 필요수를 "1씩 늘리는" 게 아니다 — 겹친 포지션들이 한 챔프풀을 나눠 쓰므로,
///   그 **합집합**이 "픽수요×포지션수 + 밴"을 넘기만 하면 충분(유저 지적 2026-08-20:
///   1000개를 전포지션에 다 넣으면 공유풀 1000 ≥ 필요라 문제없어야 함). 컴포넌트 BFS + 합집합.
pub fn overlap_component(pos: usize) -> (Vec<usize>, usize) {
    if pos >= 5 {
        return (vec![], 0);
    }
    with_state(|st| {
        let restricted: Vec<usize> = (0..5).filter(|&p| !st.allowed[p].is_empty()).collect();
        if !restricted.contains(&pos) {
            return (vec![], 0);
        }
        let shares = |a: usize, b: usize| {
            st.allowed[a]
                .iter()
                .any(|c| st.allowed[b].iter().any(|x| x == c))
        };
        let mut comp = vec![pos];
        let mut i = 0;
        while i < comp.len() {
            let cur = comp[i];
            for &p in &restricted {
                if !comp.contains(&p) && shares(cur, p) {
                    comp.push(p);
                }
            }
            i += 1;
        }
        let mut set = std::collections::HashSet::new();
        for &p in &comp {
            for c in &st.allowed[p] {
                set.insert(c.as_str());
            }
        }
        (comp, set.len())
    })
}

// ── 피어리스 최소수 검증 (유저 규칙, 2026-08-20) ─────────────────────────────
/// 시리즈 최장 길이 가정(Bo5). 실제 시리즈 길이는 대회/라운드마다 다르나 설정 화면에서
/// 알기 어려워 최악(Bo5)으로 고정 — 유저도 "bo5면"으로 기준을 잡음.
pub const SERIES_GAMES: usize = 5;

// ── ★★[2026-09-16] 최소 선택 수 = 홀(Hall) 정리 기반 부분집합별 정확식 ─────────────────────────────
//   라인 집합 S 에 대해 N(S) = S 어느 라인에든 갈 수 있는 챔프(미지정 = 전 라인), L(S) = N(S) 챔프들이 갈 수 있는 라인 집합.
//   적대적 최악에서 N(S) 에서 빠질 수 있는 수:
//     내 픽 |S| + 양팀 밴 2b + 이번 세트 상대 픽 min(5,|L|)(내 후보를 다른 라인용으로 집어감)
//     + 이전 세트 잠금 (SERIES-1) × { 클래식 0 / 피어리스 min(5,|L|) / 하드 2·min(5,|L|) }
//   안전 ⇔ ∀S: |N(S)| ≥ need(S).  단일 라인·S={p} 이면 클래식 2b+2 · 피어리스 2b+6 · 하드 2b+10 (구 공식은 피어리스 2b+5).
//   포지션 p 의 게이트 = p 를 포함하는 모든 S 의 슬랙 ≥ 0 (가장 빡빡한 S 를 UI 에 보여 준다).
//   근거·실측 = REPORT\tfm2_champ_pos_lock\RE\2026-09-16_최소선택수-공식검증.md
#[derive(Clone, Copy)]
pub struct Safety {
    pub key: u64,
    pub active: [bool; 5],
    /// 부분집합(비트마스크 1..31)별 (|N(S)|, need(S))
    pub have: [u16; 32],
    pub need: [u16; 32],
    /// p 를 포함하는 S 중 슬랙 최소인 S (표시용)
    pub worst: [u8; 5],
}
static SAFETY: RwLock<Option<Safety>> = RwLock::new(None);

impl PosState {
    /// 정확식 계산(캐시 없이). ★[2026-09-16 2차] **고정점 반복**: 포지션 p 를 끄면 규칙①에 따라 `{p,q}` 지정 챔프가
    ///   `{q}` 전용으로 줄어 다른 포지션의 여유가 깎이므로, 끈 뒤의 **실효 마스크**로 다시 계산해 더 꺼질 게 없을 때까지
    ///   반복한다(단조 감소·≤5회). 미지정 챔프 = 비활성 포지션(없으면 전 라인).
    pub fn compute_safety(&self, style: u8, ban: Option<usize>) -> Safety {
        let g = ROSTER.read().unwrap_or_else(|e| e.into_inner());
        let roster: Vec<&str> = match g.as_ref() {
            Some(set) => set.iter().map(|s| s.as_str()).collect(),
            None => {
                let mut v: Vec<&str> = Vec::new();
                for p in 0..5 { for c in &self.allowed[p] { if !v.contains(&c.as_str()) { v.push(c.as_str()); } } }
                v
            }
        };
        // 챔프별 지정 라인 비트(원본 목록 기준)
        let des: Vec<u8> = roster
            .iter()
            .map(|c| { let mut m = 0u8; for p in 0..5 { if self.allowed[p].iter().any(|x| x == c) { m |= 1 << p; } } m })
            .collect();
        let b = ban.unwrap_or(usize::MAX);
        let named_bits: u8 = (0..5)
            .filter(|&p| self.allowed[p].iter().any(|c| roster.contains(&c.as_str()) || g.is_none()))
            .fold(0u8, |m, p| m | (1 << p));
        // 활성 집합 A 에서의 (have, need) 표 — S ⊄ A 는 0.
        let table = |a: u8, have: &mut [u16; 32], need: &mut [u16; 32]| {
            let free = MASK_ALL & !a;
            let eff: Vec<u8> = des.iter().map(|&d| { let dd = d & a; if dd != 0 { dd } else if free != 0 { free } else { MASK_ALL } }).collect();
            for s in 1..32usize {
                if s & a as usize != s { have[s] = 0; need[s] = 0; continue; }
                let mut n = 0usize; let mut l = 0u8;
                for &m in &eff { if m as usize & s != 0 { n += 1; l |= m; } }
                let opp = ((l & a).count_ones() as usize).min(5);
                let lock = (SERIES_GAMES - 1) * match style { 2 => 2 * opp, 1 => opp, _ => 0 };
                let nd = if b == usize::MAX { usize::MAX } else { s.count_ones() as usize + 2 * b + opp + lock };
                have[s] = n.min(u16::MAX as usize) as u16;
                need[s] = nd.min(u16::MAX as usize) as u16;
            }
        };
        let feasible = |a: u8, have: &[u16; 32], need: &[u16; 32]| -> (bool, i64) {
            let mut mn = i64::MAX;
            for s in 1..32usize {
                if s & a as usize != s { continue; }
                let sl = have[s] as i64 - need[s] as i64;
                if sl < mn { mn = sl; }
            }
            (mn >= 0 || a == 0, mn)
        };
        // ★[2026-09-16 4차] 순서 의존 제거: A ⊆ named 32가지를 전부 검사해 **살아남는 포지션 수 최대**(동률이면 최소 슬랙 최대)를 택한다.
        //   (한 포지션을 끄면 그 지정이 사라진 챔프의 마스크가 바뀌어 다른 묶음의 필요치가 달라지므로 순차 제거는 결과가 순서에 좌우된다.)
        // 동률 처리: ①살아남는 포지션 수 ②그 포지션에만 지정된(전용) 챔프 수 합(유저 의도 보존) ③최소 슬랙.
        let excl: [u32; 5] = core::array::from_fn(|p| des.iter().filter(|&&d| d == 1 << p).count() as u32);
        let mut best: Option<(u8, (u32, u32, i64))> = None;
        let mut th = [0u16; 32]; let mut tn = [0u16; 32];
        for a in 0..32u8 {
            if a & !named_bits != 0 { continue; }
            table(a, &mut th, &mut tn);
            let (ok, mn) = feasible(a, &th, &tn);
            if !ok { continue; }
            let ex: u32 = (0..5).filter(|&p| a & (1 << p) != 0).map(|p| excl[p]).sum();
            let key = (a.count_ones(), ex, mn);
            if best.map_or(true, |(_, k)| key > k) { best = Some((a, key)); }
        }
        let a = best.map(|x| x.0).unwrap_or(0);
        let mut have = [0u16; 32]; let mut need = [0u16; 32]; let mut worst = [0u8; 5];
        table(a, &mut have, &mut need);
        for p in 0..5 {
            if a & (1 << p) != 0 {
                let mut ws = (i64::MAX, 1usize << p);
                for s in 1..32usize {
                    if s & (1 << p) == 0 || s & a as usize != s { continue; }
                    let sl = have[s] as i64 - need[s] as i64;
                    if sl < ws.0 { ws = (sl, s); }
                }
                worst[p] = ws.1 as u8;
            } else if named_bits & (1 << p) != 0 {
                // 꺼진 포지션: A∪{p} 에서 p 를 포함하는 가장 나쁜 S (왜 못 켜는지 표시용)
                let ap = a | (1 << p);
                table(ap, &mut th, &mut tn);
                let mut ws = (i64::MAX, 1usize << p);
                for s in 1..32usize {
                    if s & (1 << p) == 0 || s & ap as usize != s { continue; }
                    let sl = th[s] as i64 - tn[s] as i64;
                    if sl < ws.0 { ws = (sl, s); }
                }
                worst[p] = ws.1 as u8;
                for s in 1..32usize { if s & (1 << p) != 0 && s & ap as usize == s { have[s] = th[s]; need[s] = tn[s]; } }
            }
        }
        let mut active = [false; 5];
        for p in 0..5 { active[p] = a & (1 << p) != 0; }
        Safety { key: 0, active, have, need, worst }
    }
}

impl PosState {
    /// 상태 버전·룰·로스터 서명으로 캐시된 정확식 결과. ⚠`&self` 로 받는다 — `with_state` 안(STATE read 보유)에서
    ///   호출되므로 여기서 다시 `with_state` 를 잡으면 같은 스레드 재귀 read = 데드락 위험(config.rs 상단 주석).
    pub fn safety_cached(&self) -> Safety {
        let (style, ban) = cur_rule();
        let key = STATE_VER.load(Ordering::Relaxed)
            ^ ((style as u64) << 40)
            ^ ((ban.map(|b| b as u64 + 1).unwrap_or(0)) << 44)
            ^ (ROSTER_VER.load(Ordering::Relaxed) << 48);
        if let Some(s) = SAFETY.read().unwrap_or_else(|e| e.into_inner()).as_ref() {
            if s.key == key { return *s; }
        }
        let mut s = self.compute_safety(style, ban);
        s.key = key;
        *SAFETY.write().unwrap_or_else(|e| e.into_inner()) = Some(s);
        s
    }
}
pub fn safety() -> Safety {
    with_state(|st| st.safety_cached())
}
/// 로스터 게시 횟수(안전식 캐시 키).
pub static ROSTER_VER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// UI 표시용: 포지션 p 의 (판정 풀 |N({p})|, 필요 need({p}), 가장 빡빡한 S 비트, 그 S 의 (have, need)).
pub fn pos_safety(p: usize) -> (usize, usize, u8, usize, usize) {
    let s = safety();
    let one = 1usize << p;
    let w = s.worst[p] as usize;
    (s.have[one] as usize, s.need[one] as usize, s.worst[p], s.have[w] as usize, s.need[w] as usize)
}

/// 화이트리스트한 포지션이 가져야 할 최소 챔피언 수.
/// - 클래식(0): 단판이라도 내 픽1 + 상대 픽1(같은 판 배타) + 양팀 밴(ban×2) 고려 → 2 + ban×2
///   (유저 규칙 2026-08-20: "클래식이어도 밴카드·상대 선택 생각하면 최소개수 필요")
/// - 피어리스(1): 내 팀이 시리즈 내내 안 겹치게 → Bo5 = 5, + 밴카드 빠지는 몫(밴 양팀 = ban×2)
/// - 하드피어리스(2): ★양팀이 서로도 못 겹침 → Bo5 = 10(=5×2), + 밴카드 ban×2
///   (유저: "하드피어리스 bo5면 10개, 밴카드 5장이면 +10, 3장이면 +6")
/// ⚠[2026-09-16] 이 값은 **단일 라인·포지션 하나** 기준 참고치(로그·설명용)다. 실제 게이트는 `safety()`(부분집합 정확식).
///   피어리스 = ~~5+2b~~ → **6+2b**(이번 세트 상대 픽 1 누락 정정).
pub fn min_required(style: u8, ban_count: usize) -> usize {
    match style {
        2 => 2 + (SERIES_GAMES - 1) * 2 + ban_count * 2, // 하드: 내1+상대1+잠금 4세트×2 = 10+2b
        1 => 2 + (SERIES_GAMES - 1) + ban_count * 2,     // 피어리스: 내1+상대1+잠금 4 = 6+2b
        _ => 2 + ban_count * 2,                          // 클래식: 내1+상대1 = 2+2b
    }
}

pub fn load() {
    // 토글
    let mut c = Cfg::empty();
    if let Some(d) = crate::mod_dir() {
        let p = format!("{d}\\{}.cfg", crate::MOD_ID);
        match std::fs::read_to_string(&p) {
            Ok(t) => {
                for raw in t.lines() {
                    let line = raw.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }
                    let Some((k, v)) = line.split_once('=') else { continue };
                    let on = |s: &str| s.trim() == "1" || s.trim().eq_ignore_ascii_case("true");
                    match k.trim().to_ascii_lowercase().as_str() {
                        "enabled" => c.enabled = on(v),
                        "debug" => c.debug = on(v),
                        "log_lineups" => c.log_lineups = on(v),
                        "swap_force" => c.swap_force = v.trim().parse().unwrap_or(0),
                        "ai_pick_gate" => c.ai_pick_gate = on(v),
                        "ai_observe_only" => c.ai_observe_only = on(v),
                        "ai_assign_mask" => c.ai_assign_mask = on(v),
                        "user_pick_block" => c.user_pick_block = on(v),
                        "enforce_lineup" => c.enforce_lineup = on(v),
                        "dump_ui" => c.dump_ui = on(v),
                        _ => {}
                    }
                }
            }
            Err(_) => {
                let _ = std::fs::write(&p, DEFAULT_CFG);
                c.load_log.push("cfg 없음 → 기본 생성".into());
            }
        }
    }
    let _ = CFG.set(c);

    // ★상태(포지션 화이트리스트) = **세이브 전용**(2026-08-23 유저 지시).
    //   ~~구: champ_pos_lock_state.txt 를 읽어 초기값으로 삼음~~ → **폐기**.
    //   파일은 세이브와 무관하게 오래 살아서 ①다른 세이브 설정이 새 세이브로 새고
    //   ②없어진 챔프 id 가 개수를 부풀리고 ③"파일 vs 세이브" 어느 쪽이 정본인지 모호했다.
    //   ⟹ 저장소는 세이브 안 mod save data 하나. 세이브에 설정 없음 = 제한 없음(바닐라).
    //   (같은 결론에 먼저 도달한 선례 = tfm2_champion_exclude v0.4.2)
    set_state(PosState::default());
}

// ── ★세이브별 설정 (2026-08-23 유저 지시) ──────────────────────────────────
//   설정의 정본 = **그 세이브 안의 mod save data**(공식 API). 파일(state.txt)은
//   "새 세이브의 초기값" 씨앗 역할만 한다: 세이브 영역이 비어 있으면 파일을 읽어
//   **지금 로스터에 실제로 있는 챔프만 남겨** 그 세이브에 심는다.
//   ⟹ 세이브마다 독립. 세이브 복사/백업에 설정이 동행. 유령 챔프가 넘어가지 않는다.

/// 현재 상태를 세이브 본문 텍스트로 직렬화. `live_only=true` 면 지금 로스터에 있는 것만.
pub fn state_text(live_only: bool) -> String {
    with_state(|st| state_text_of(st, live_only))
}

/// 임의의 PosState 를 세이브 본문으로 직렬화.
///   ⚠**세이브 이관에는 반드시 파일에서 읽은 PosState 를 넘길 것.** 메모리의 현재 STATE 를 쓰면
///     직전 세이브의 설정이 새 세이브로 그대로 복사된다(2026-08-23 실사고 — 유저가 파일을
///     지웠는데도 새 세이브에 옛 설정이 들어갔다. `state_text()` 가 STATE 를 읽는데 로그만
///     "파일에서 이관"이라고 찍고 있었다).
pub fn state_text_of(st: &PosState, live_only: bool) -> String {
    let mut s = String::from(
        "# tfm2_champ_pos_lock — 포지션별 허용 챔피언 (이 세이브 전용)
",
    );
    let g = ROSTER.read().unwrap_or_else(|e| e.into_inner());
    for p in 0..5 {
        let list: Vec<&str> = st.allowed[p]
            .iter()
            .filter(|c| !live_only || g.as_ref().map(|set| set.contains(*c)).unwrap_or(true))
            .map(|c| c.as_str())
            .collect();
        s.push_str(&format!("{} = {}
", POS_NAMES[p], list.join(", ")));
    }
    s
}

/// 텍스트 → PosState (반영하지 않고 파싱만).
pub fn parse_state_text(text: &str) -> PosState {
    let mut st = PosState::default();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((k, v)) = line.split_once('=') else { continue };
        if let Some(pi) = pos_index(k) {
            st.allowed[pi] = v
                .split(',')
                .map(|s| s.trim().to_ascii_lowercase())
                .filter(|s| !s.is_empty())
                .collect();
        }
    }
    st
}


/// 세이브 밖(메인메뉴/새 게임)으로 나갈 때 설정을 비운다 — 다음 세이브로 새는 것 차단.
pub fn clear_state() {
    let empty = PosState::default();
    let changed = with_state(|st| st.allowed.iter().any(|v| !v.is_empty()));
    if changed {
        set_state(empty);
    }
}

/// 세이브 본문 텍스트 → 상태 반영(마스크 재계산 트리거 포함).
pub fn apply_state_text(text: &str) {
    set_state(parse_state_text(text));
}

/// 로스터가 캡처됐나(세이브 마이그레이션은 이게 참일 때만 — 안 그러면 전부 유령 취급된다).
pub fn roster_ready() -> bool {
    ROSTER
        .read()
        .unwrap_or_else(|e| e.into_inner())
        .as_ref()
        .map(|s| !s.is_empty())
        .unwrap_or(false)
}


/// 라인업 진단 전용 로그(debug 와 무관, log_lineups 노브로 제어).
static LINEUP_FILE_FRESH: AtomicBool = AtomicBool::new(false);
pub fn llog(msg: &str) {
    if !get().log_lineups {
        return;
    }
    if let Some(d) = crate::mod_dir() {
        use std::io::Write;
        // 세션 시작 시 1회 비우기 — 지난 판 기록과 섞이면 판독이 어렵다.
        if !LINEUP_FILE_FRESH.swap(true, Ordering::Relaxed) {
            let _ = std::fs::write(
                format!("{d}\\champ_pos_lock_lineups.txt"),
                "",
            );
        }
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{d}\\champ_pos_lock_lineups.txt"))
        {
            let _ = writeln!(f, "{msg}");
        }
    }
}

/// ★설정 저장/로드 이벤트 로그 — **debug 와 무관하게 항상 남긴다**.
///   세이브당 몇 줄뿐이라 스팸이 안 되고, 유저가 debug=1 로 안 바꿔도 "설정이 어디에 저장됐나"를
///   확인할 수 있어야 한다(2026-08-23: debug=0 이라 세이브 배선을 아예 검증 못 한 사고).
///   세션 시작 시 1회 비운다.
#[repr(C)]
struct WinSysTime {
    year: u16,
    month: u16,
    dow: u16,
    day: u16,
    hour: u16,
    min: u16,
    sec: u16,
    ms: u16,
}
extern "system" {
    fn GetLocalTime(p: *mut WinSysTime);
}
/// 로컬 시각 "HH:MM:SS" — 로그를 세이브 파일 mtime 과 대조하려면 벽시계가 필요하다.
fn now_hms() -> String {
    let mut s = WinSysTime { year: 0, month: 0, dow: 0, day: 0, hour: 0, min: 0, sec: 0, ms: 0 };
    unsafe { GetLocalTime(&mut s) };
    format!("{:02}:{:02}:{:02}", s.hour, s.min, s.sec)
}

pub fn slog(msg: &str) {
    let Some(d) = crate::mod_dir() else { return };
    let msg = &format!("[{}] {msg}", now_hms());
    use std::io::Write;
    let path = format!("{d}\\champ_pos_lock_save.txt");
    if !SAVE_FILE_FRESH.swap(true, Ordering::Relaxed) {
        let _ = std::fs::write(&path, "# 포지션 제한 — 설정 저장/로드 이벤트
");
    }
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
        let _ = writeln!(f, "{msg}");
    }
}
static SAVE_FILE_FRESH: AtomicBool = AtomicBool::new(false);

pub fn dlog(msg: &str) {
    if !get().debug {
        return;
    }
    if let Some(d) = crate::mod_dir() {
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{d}\\champ_pos_lock_log.txt"))
        {
            let _ = writeln!(f, "{msg}");
        }
    }
}

const DEFAULT_CFG: &str = "\
    # tfm2_champ_pos_lock - Champion Position Lock (toggles only; the actual per-position\r\n\
    #   champion lists are edited in-game).\r\n\
    # In game: Settings -> Gameplay -> 'Position Lock' button at the bottom.\r\n\
    enabled=1\r\n\
    # Write a diagnostic log (champ_pos_lock_log.txt). Off unless you are debugging.\r\n\
    debug=0\r\n\
    # Restrict AI teams too (their picks and their position assignment).\r\n\
    ai_pick_gate=1\r\n\
    ai_assign_mask=1\r\n\
    # In your own draft, grey out champions that would break the position assignment\r\n\
    #   (like fearless does). Automatically lifted when nothing legal is left.\r\n\
    user_pick_block=1\r\n\
    # Dump the UI node tree to find injection points. Development only.\r\n\
    dump_ui=0\r\n\
    # Log the final line-up of every match to champ_pos_lock_lineups.txt.\r\n\
    #   For bug reports; keep it off normally.\r\n\
    log_lineups=0\r\n\
    # Rewrite the position assignment (swap order) so every champion ends up in a\r\n\
    #   position you allowed. This is what fixes the OPPONENT and every AI team's\r\n\
    #   line-up - turning it off leaves their assignment untouched (and stops the\r\n\
    #   'assignment' lines in the line-up log). 1 = on, 0 = off.\r\n\
    swap_force=1\r\n\
";

