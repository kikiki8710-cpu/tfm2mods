//! tfm2_banpick_order 설정 — `mods\tfm2_banpick_order\tfm2_banpick_order.cfg`.
//! 없으면 기본값(예시 시퀀스 포함)으로 자동 생성. 값 수정 후 게임 재시작.
//!
//! 시퀀스 키:
//!   seq_<룰>          — 그 룰에서 팀당 밴 수가 "B1 토큰 개수"와 일치할 때 적용
//!   seq_<룰>_ban<n>   — 그 룰 + 팀당 밴 n 설정에서만 적용 (동일 슬롯이면 나중 줄 우선)
//! 토큰: B1=1팀(블루) 밴 / B2=2팀(레드) 밴 / P1=1팀 픽 / P2=2팀 픽 (대소문자 무관).
//!
//! 검증(어기면 그 시퀀스는 버리고 바닐라 순서 사용, debug=1 시 사유 로그):
//!   - B1 개수 == B2 개수 (이 값이 곧 적용될 팀당 밴 수)
//!   - P1 개수 == P2 개수 == 룰별 팀당 픽 수 (2v2=2, 3v3=3, 4v4=4, 5v5=5)
//!   - 마지막 토큰은 픽(P1/P2) — 픽 완료 마감(0x11d8ef0)이 시퀀스 끝에서 발화해야
//!     서버 종료판정(0xFF)과 어긋나지 않는다 (ANA discovered-banpick-ai.md §16).

use std::sync::OnceLock;

/// BanpickPhase (tfm2_lib_reference.md L378): Team1Pick=0 Team2Pick=1 Team1Ban=2 Team2Ban=3
pub const PH_P1: u8 = 0;
pub const PH_P2: u8 = 1;
pub const PH_B1: u8 = 2;
pub const PH_B2: u8 = 3;

pub const MAX_BAN: usize = 15; // 팀당 밴 수 상한(게임 설정 1~5, 자동값은 룰별 1~3)

pub struct Cfg {
    pub enabled: bool,
    pub debug: bool,
    /// ⚠실험: AI 조합/추천 함수의 인라인 phase 6사이트 패치(훅 H). 0.5.3 전용.
    /// 2026-07-30 최초 시도에서 사이트 3곳의 합류주소·arm 부작용 명령 추출이 틀려 크래시 →
    /// 재검증 전까지 **기본 0**. 켜면 코치 위임 시 AI가 커스텀 순서를 따르지만 미검증이다.
    pub ai_inline_phase: bool,
    /// UI 턴 하이라이트 계열 추가 패치(훅 I/J/L + hl_count).
    /// ★기본 ON(2026-07-30, §14.6 후속 인게임 확정): 이름은 "하이라이트"지만 이 세트의
    /// drain cur/next(J)·개수 루프가 **코치 위임 자동행동 펌프의 진행(드레인 state-4 타이머
    /// 전진)에 load-bearing**임이 실증됐다. 끄면 위임 턴이 total 불변으로 정체해 워치독에만
    /// 의존하게 된다(랜덤 정지 재발). drainA(K)는 이와 별개로 항상 상시 설치. ⛔끄지 말 것.
    pub ui_highlight: bool,
    /// ⚠진단 전용: 매 프레임 모든 `in_turn` 노드를 강제로 끈다. 흰 상자가 사라지면
    /// 모드의 노드 쓰기가 렌더까지 도달한다는 뜻(=대상 노드가 맞음), 그대로면 흰 상자는
    /// `in_turn` 이 아닌 다른 것이 그린다는 뜻. 어느 쪽이든 한 번에 갈린다.
    pub hl_force_off: bool,
    /// ⚠진단 전용(§14.5(6)): 칸 채움색 기록자 포착 — 픽슬롯 runner+0x13c 에 DR write BP.
    /// debug=1 과 함께 켜야 한다(VEH 가 diag::install 로 깔림). 켜는 동안 모드의
    /// set_fill 쓰기는 중단(게임 자신의 write 만 관찰).
    pub drbp: bool,
    /// 우리 밴픽 AI: 밴할 때 이미 확정된 픽을 함께 보고 점수 보정.
    pub ai_ban_context: bool,
    /// ① 상대 조합과의 시너지 가중치(상대가 원할 카드를 미리 자름).
    pub ai_w_syn: f32,
    /// ② 내 조합에 대한 위협 가중치(내 조합 카운터를 선제 차단).
    pub ai_w_cnt: f32,
    /// 보정 상한(네이티브 점수를 뒤엎지 않도록).
    pub ai_cap: f32,
    /// ★모든 경기에 커스텀 순서 적용.
    /// ON  = 내 밴픽 화면이 없는 **백그라운드 AI 경기**도 같은 순서로 진행한다.
    /// OFF = 밴픽 화면일 때만 커스텀, 백그라운드는 게임 기본 순서(v1.2.1 이하와 같은 동작).
    ///
    /// ⚠**기본값 = OFF**(2026-08-03 하향). 개발 환경에서는 이 값을 켜고 **328경기 무사 통과**를
    /// 실측했지만(커밋 6,555 전부 커스텀·거부 0·맴돎 0), **다른 사용자 환경에서 "일정 넘기기가
    /// 아예 시작되지 않는다"는 제보**가 있었고 그 원인이 아직 규명되지 않았다. 재현이 환경에
    /// 의존하므로(모드 조합·코어 수 등 미확정) 원인이 잡힐 때까지 기본값은 안전 쪽에 둔다.
    /// ⟹ 원인 규명 후 다시 ON 으로 올릴 것. 켤 사람은 cfg 에서 명시적으로 1 을 준다.
    /// ⚠이 값은 **진행 축 훅 전체**(phase·턴 오라클·커밋·AI 밴 파리티)에 한꺼번에 적용돼야
    /// 한다. 일부만 커스텀이면 서로 다른 순서를 기준으로 움직여 그 경기가 밴 단계를 못
    /// 빠져나오고 커밋만 반복한다(= 시즌 일정 정지, 2026-08-01 실사고).
    pub apply_all: bool,
    /// [rule 0..4][팀당 밴 수 0..=MAX_BAN] → 검증 통과한 phase 시퀀스.
    /// rule: 0=2v2 1=3v3 2=4v4 3=5v5 (MatchSetInfo+0xf9 값 그대로).
    pub seqs: Vec<Vec<Option<Box<[u8]>>>>,
    /// 파싱/검증 로그(로드 시 1회 기록용).
    pub load_log: Vec<String>,
}

impl Cfg {
    fn empty() -> Self {
        Cfg {
            enabled: true,
            debug: false,
            ai_inline_phase: false,
            ui_highlight: true, // ★코치 위임 펌프에 load-bearing(§14.6) — 기본 ON
            hl_force_off: false,
            drbp: false,
            ai_ban_context: true,
            ai_w_syn: 1.0,
            ai_w_cnt: 1.0,
            ai_cap: 1.5,
            apply_all: false,
            seqs: vec![(0..=MAX_BAN).map(|_| None).collect(); 4],
            load_log: Vec::new(),
        }
    }

    #[inline]
    pub fn seq_for(&self, rule: u8, ban: u64) -> Option<&[u8]> {
        let r = rule as usize;
        let b = ban as usize;
        if r >= 4 || b > MAX_BAN {
            return None;
        }
        self.seqs[r][b].as_deref()
    }
}

static CFG: OnceLock<Cfg> = OnceLock::new();

pub fn get() -> &'static Cfg {
    CFG.get_or_init(Cfg::empty)
}

const TEMPLATE: &str = "# tfm2_banpick_order — 밴픽 순서 커스텀. 수정 후 게임 재시작.

enabled = 1
debug   = 0

# ── 밴픽 순서 ─────────────────────────────────────────────
# 토큰: B1/B2 = 1팀(블루)/2팀(레드) 밴, P1/P2 = 픽
# 키:   seq_<룰>_ban<n>  (n은 표기용, 실제 적용은 B1 개수 기준)
# 규칙: B1개수 == B2개수 == 게임의 팀당 밴 수
#       P1개수 == P2개수 == 팀당 픽 수 (2v2=2 … 5v5=5)
#       마지막 토큰은 픽. 어기면 무시되고 게임 기본 순서로 진행.

# 5v5 · 팀당 밴5: 밴3씩 → 픽6 → 밴2씩 → 픽4
seq_5v5_ban5 = B1 B2 B1 B2 B1 B2 P1 P2 P2 P1 P1 P2 B2 B1 B2 B1 P2 P1 P1 P2

# 5v5 · 팀당 밴4: 밴2씩 → 픽6 → 밴2씩 → 픽4
seq_5v5_ban4 = B1 B2 B1 B2 P1 P2 P2 P1 P1 P2 B2 B1 B2 B1 P2 P1 P1 P2

# 5v5 · 팀당 밴3: 밴2씩 → 픽6 → 밴1씩 → 픽4
#seq_5v5_ban3 = B1 B2 B1 B2 P1 P2 P2 P1 P1 P2 B2 B1 P2 P1 P1 P2

# ── 적용 범위 ─────────────────────────────────────────────
# 0 = 내 밴픽 화면에서만 커스텀, 나머지는 게임 기본 순서 (기본)
# 1 = 내가 보지 않는 AI끼리의 경기도 같은 순서로 밴픽
#
# 1로 켜면 리그 전체가 같은 규칙으로 돌지만, 일부 환경에서 일정 진행이
# 시작되지 않는다는 제보가 있어 원인 확인 전까지 기본값을 0으로 둡니다.
# 켜신 뒤 일정이 안 넘어가면 다시 0으로 되돌려 주세요.
apply_all = 0

# ── 밴픽 AI 보정 ──────────────────────────────────────────
# 게임 기본 밴 AI는 이미 확정된 픽을 보지 않습니다.
# 켜면 게임 점수에 보정치를 더합니다(덮어쓰지 않음).
# 다른 밴픽 AI 모드를 쓰려면 0으로 끄세요(순서 기능은 그대로).
ai_ban_context = 1

# ai_w_syn: 상대 조합과 시너지 좋은 챔프를 미리 밴
# ai_w_cnt: 내 조합을 잘 때리는 챔프를 선제 밴
# ai_cap  : 보정 상한 (가중치 0 = 해당 기준 끔)
ai_w_syn = 1.0
ai_w_cnt = 1.0
ai_cap   = 1.5
";

fn parse_tokens(v: &str) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    for t in v.split_whitespace() {
        let ph = match t.to_ascii_uppercase().as_str() {
            "B1" => PH_B1,
            "B2" => PH_B2,
            "P1" => PH_P1,
            "P2" => PH_P2,
            other => return Err(format!("알 수 없는 토큰 '{other}'")),
        };
        out.push(ph);
    }
    if out.is_empty() {
        return Err("빈 시퀀스".into());
    }
    Ok(out)
}

/// 검증 → 통과 시 (rule, 팀당 밴 수) 슬롯 반환.
fn validate(rule: usize, seq: &[u8]) -> Result<usize, String> {
    let b1 = seq.iter().filter(|&&p| p == PH_B1).count();
    let b2 = seq.iter().filter(|&&p| p == PH_B2).count();
    let p1 = seq.iter().filter(|&&p| p == PH_P1).count();
    let p2 = seq.iter().filter(|&&p| p == PH_P2).count();
    let ppt = rule + 2; // 룰별 팀당 픽 수
    if b1 != b2 {
        return Err(format!("B1({b1}) != B2({b2})"));
    }
    if b1 > MAX_BAN {
        return Err(format!("팀당 밴 {b1} > 상한 {MAX_BAN}"));
    }
    if p1 != ppt || p2 != ppt {
        return Err(format!("P1({p1})/P2({p2}) != 팀당 픽 수 {ppt}"));
    }
    match seq.last() {
        Some(&PH_P1) | Some(&PH_P2) => {}
        _ => return Err("마지막 토큰이 픽(P1/P2)이 아님".into()),
    }
    Ok(b1)
}

pub fn load() {
    let mut cfg = Cfg::empty();
    let Some(dir) = crate::mod_dir() else {
        let _ = CFG.set(cfg);
        return;
    };
    let path = format!("{dir}\\tfm2_banpick_order.cfg");
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(_) => {
            // BOM 없는 UTF-8 로 생성 (게임이 읽는 파일 아님 — 모드 전용이지만 규칙 준수)
            let _ = std::fs::create_dir_all(&dir);
            let _ = std::fs::write(&path, TEMPLATE.as_bytes());
            TEMPLATE.to_string()
        }
    };

    for raw in text.lines() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let Some((k, v)) = line.split_once('=') else { continue };
        let (k, v) = (k.trim(), v.trim());
        match k {
            "enabled" => cfg.enabled = v != "0",
            "debug" => cfg.debug = v != "0",
            "ai_inline_phase" => cfg.ai_inline_phase = v != "0",
            "ui_highlight" => cfg.ui_highlight = v != "0",
            "hl_force_off" => cfg.hl_force_off = v != "0",
            "drbp" => cfg.drbp = v != "0",
            "ai_ban_context" => cfg.ai_ban_context = v != "0",
            "apply_all" => cfg.apply_all = v != "0",
            "ai_w_syn" => cfg.ai_w_syn = v.parse().unwrap_or(1.0),
            "ai_w_cnt" => cfg.ai_w_cnt = v.parse().unwrap_or(1.0),
            "ai_cap" => cfg.ai_cap = v.parse().unwrap_or(1.5),
            _ if k.starts_with("seq_") => {
                // "seq_5v5" / "seq_5v5_ban3" 등 — "seq_" 뒤 3글자만 룰 토큰(접미사 무시).
                let rest = &k[4..];
                let rule = match &rest[..3.min(rest.len())] {
                    "2v2" => 0usize,
                    "3v3" => 1,
                    "4v4" => 2,
                    "5v5" => 3,
                    _ => {
                        cfg.load_log.push(format!("{k}: 룰 표기 인식 실패 — 무시"));
                        continue;
                    }
                };
                match parse_tokens(v).and_then(|seq| {
                    let ban = validate(rule, &seq)?;
                    Ok((ban, seq))
                }) {
                    Ok((ban, seq)) => {
                        cfg.load_log.push(format!(
                            "{k}: OK — rule={rule} 팀당밴={ban} 길이={}",
                            seq.len()
                        ));
                        cfg.seqs[rule][ban] = Some(seq.into_boxed_slice());
                    }
                    Err(e) => cfg.load_log.push(format!("{k}: 검증 실패({e}) — 바닐라 사용")),
                }
            }
            _ => {}
        }
    }
    let _ = CFG.set(cfg);
}

/// debug=1 일 때만 로그 append (메인 스레드 전용 — detour 안에서 호출 금지).
pub fn dlog(msg: &str) {
    if !get().debug {
        return;
    }
    if let Some(d) = crate::mod_dir() {
        use std::io::Write;
        let _ = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("{d}\\order_log.txt"))
            .and_then(|mut f| writeln!(f, "{msg}"));
    }
}
