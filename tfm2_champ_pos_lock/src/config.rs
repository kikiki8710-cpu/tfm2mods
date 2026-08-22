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
            log_lineups: true,
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
    pub fn mask_of(&self, lower: &str) -> u8 {
        let mut m = 0u8;
        for p in 0..5 {
            if self.allowed[p].is_empty() || self.allowed[p].iter().any(|x| x == lower) {
                m |= 1 << p;
            }
        }
        m
    }
    /// 제한이 하나라도 걸렸나(모든 포지션 빈 화이트리스트면 모드 무효과).
    pub fn any_restricted(&self) -> bool {
        self.allowed.iter().any(|v| !v.is_empty())
    }
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
    with_state(|st| st.allowed[pos].len())
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

/// 화이트리스트한 포지션이 가져야 할 최소 챔피언 수.
/// - 클래식(0): 단판이라도 내 픽1 + 상대 픽1(같은 판 배타) + 양팀 밴(ban×2) 고려 → 2 + ban×2
///   (유저 규칙 2026-08-20: "클래식이어도 밴카드·상대 선택 생각하면 최소개수 필요")
/// - 피어리스(1): 내 팀이 시리즈 내내 안 겹치게 → Bo5 = 5, + 밴카드 빠지는 몫(밴 양팀 = ban×2)
/// - 하드피어리스(2): ★양팀이 서로도 못 겹침 → Bo5 = 10(=5×2), + 밴카드 ban×2
///   (유저: "하드피어리스 bo5면 10개, 밴카드 5장이면 +10, 3장이면 +6")
pub fn min_required(style: u8, ban_count: usize) -> usize {
    match style {
        2 => SERIES_GAMES * 2 + ban_count * 2, // 하드피어리스
        1 => SERIES_GAMES + ban_count * 2,     // 피어리스
        _ => 2 + ban_count * 2,                // 클래식
    }
}
/// 포지션별 검증: (pos, 현재수, 최소수) — 부족한 포지션만. 빈 화이트리스트(=전체허용)는 제외.
pub fn shortfalls(st: &PosState, style: u8, ban_count: usize) -> Vec<(usize, usize, usize)> {
    let need = min_required(style, ban_count);
    (0..5)
        .filter(|&p| !st.allowed[p].is_empty() && st.allowed[p].len() < need)
        .map(|p| (p, st.allowed[p].len(), need))
        .collect()
}

// ── 파일 IO ─────────────────────────────────────────────────────────────────
fn state_path() -> Option<String> {
    crate::mod_dir().map(|d| format!("{d}\\champ_pos_lock_state.txt"))
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

    // 상태
    let mut st = PosState::default();
    if let Some(p) = state_path() {
        match std::fs::read_to_string(&p) {
            Ok(t) => {
                for raw in t.lines() {
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
            }
            Err(_) => {
                let _ = std::fs::write(&p, DEFAULT_STATE);
            }
        }
    }
    set_state(st);
}

/// 현재 상태를 state 파일에 저장(인게임 UI 확인 버튼이 호출).
pub fn save_state_to_file() {
    let Some(p) = state_path() else { return };
    let mut s = String::from(
        "# tfm2_champ_pos_lock — 포지션별 허용 챔피언 (인게임 UI 가 관리)\r\n\
         # 빈 줄(또는 줄 없음) = 그 포지션은 모든 챔피언 허용\r\n",
    );
    with_state(|st| {
        for p in 0..5 {
            s.push_str(&format!("{} = {}\r\n", POS_NAMES[p], st.allowed[p].join(", ")));
        }
    });
    let _ = std::fs::write(&p, s);
}

static DUMP_ONCE: AtomicBool = AtomicBool::new(false);
pub fn dump_reset() {
    DUMP_ONCE.store(false, Ordering::Relaxed);
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
# tfm2_champ_pos_lock — 챔피언 포지션 제한 (토글만; 실제 제한 목록은 인게임 UI)\r\n\
# 인게임: 환경설정 → 게임플레이 → 맨 아래 '포지션 제한' 버튼\r\n\
enabled=1\r\n\
debug=0\r\n\
# AI 픽/배정 제한\r\n\
ai_pick_gate=1\r\n\
ai_assign_mask=1\r\n\
# 내 밴픽 화면에서 매칭 깨는 챔피언을 피어리스처럼 회색+선택불가 (고를 게 0 이면 자동 해제)\r\n\
user_pick_block=1\r\n\
# UI 주입점 파악용 노드 트리 덤프(개발용 — 인게임 UI 완성 후 0)\r\n\
dump_ui=1\r\n\
";

const DEFAULT_STATE: &str = "\
# tfm2_champ_pos_lock — 포지션별 허용 챔피언 목록\r\n\
# 형식: <포지션> = <챔피언id>, <챔피언id>, ...\r\n\
# 포지션 = top / jungle / mid / bottom / support (탑/정글/미드/원딜/서폿)\r\n\
# ★비어 있으면 그 포지션은 '모든 챔피언' 허용.\r\n\
# 챔피언 id 는 debug=1 로 게임을 한 번 켜면 champ_pos_lock_champions.txt 에 생성됩니다.\r\n\
# (곧 인게임 UI: 환경설정 -> 게임플레이 -> 포지션 제한 버튼으로 이 파일을 편집합니다.)\r\n\
top =\r\n\
jungle =\r\n\
mid =\r\n\
bottom =\r\n\
support =\r\n\
";
