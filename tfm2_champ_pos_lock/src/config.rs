//! tfm2_champ_pos_lock 설정 — `mods\tfm2_champ_pos_lock\tfm2_champ_pos_lock.cfg`.
//! 없으면 기본값(주석 포함 예시)으로 자동 생성. 값 수정 후 게임 재시작.
//!
//! 핵심 키:
//!   lock=<챔피언id>:<포지션[,포지션...]>
//!     포지션 = top/jungle/mid/bottom/support (한국어 탑/정글/미드/원딜|바텀/서폿|서포터,
//!     숫자 0~4 도 허용). 여러 줄 가능. 같은 챔피언이 여러 줄이면 마지막 줄 우선.
//!   챔피언 id 목록은 debug=1 로 게임 한 번 띄우면 mods 폴더의
//!   champ_pos_lock_champions.txt 에 떨어진다.

use std::sync::OnceLock;

/// 포지션 비트 (bit0=top bit1=jungle bit2=mid bit3=bottom bit4=support)
pub const MASK_ALL: u8 = 0b11111;

pub struct Cfg {
    pub enabled: bool,
    pub debug: bool,
    /// AI 픽 차단 (DraftScoreHook — 배정 불가능해지는 챔프를 AI가 안 집게)
    pub ai_pick_gate: bool,
    /// AI 배정 마스크 강제 (hookA — eligible-positions 비트마스크 산출기 detour)
    pub ai_assign_mask: bool,
    /// (예약·미구현) 최종 라인업 하드 강제 — apply_lineup Rec 교정 후보 확보됨
    /// (RE\2026-08-19_서버권위라인업-개입점.md). sim의 pos 소스 런타임 확인 후 구현.
    pub enforce_lineup: bool,
    /// 소문자 챔피언 id → 허용 포지션 마스크
    pub locks: Vec<(String, u8)>,
    pub load_log: Vec<String>,
}

impl Cfg {
    fn empty() -> Self {
        Cfg {
            enabled: true,
            debug: false,
            ai_pick_gate: true,
            ai_assign_mask: true,
            enforce_lineup: true,
            locks: Vec::new(),
            load_log: Vec::new(),
        }
    }

    pub fn mask_of(&self, lower_name: &str) -> Option<u8> {
        self.locks
            .iter()
            .rev() // 마지막 줄 우선
            .find(|(n, _)| n == lower_name)
            .map(|(_, m)| *m)
    }
}

static CFG: OnceLock<Cfg> = OnceLock::new();

pub fn get() -> &'static Cfg {
    CFG.get_or_init(|| Cfg::empty())
}

fn parse_pos(tok: &str) -> Option<u8> {
    let t = tok.trim().to_ascii_lowercase();
    let bit = match t.as_str() {
        "top" | "탑" | "0" => 0,
        "jungle" | "jg" | "정글" | "1" => 1,
        "mid" | "middle" | "미드" | "2" => 2,
        "bottom" | "bot" | "adc" | "원딜" | "바텀" | "3" => 3,
        "support" | "sup" | "서폿" | "서포터" | "4" => 4,
        _ => return None,
    };
    Some(1u8 << bit)
}

const DEFAULT_CFG: &str = "\
# tfm2_champ_pos_lock — 특정 챔피언을 특정 포지션에서만 쓰게 제한\r\n\
# 수정 후 게임 재시작. 포지션: top/jungle/mid/bottom/support (탑/정글/미드/원딜/서폿, 0~4 도 됨)\r\n\
enabled=1\r\n\
debug=0\r\n\
# AI 가 배정 불가능한 챔프를 픽하지 않게 차단 (권장 1)\r\n\
ai_pick_gate=1\r\n\
# AI 의 챔피언-포지션 배정을 허용 포지션으로 제한 (권장 1)\r\n\
ai_assign_mask=1\r\n\
# (예약 - 아직 미구현) 최종 라인업 하드 강제. 현재 버전은 AI 픽/배정까지 제한하고,\r\n\
# 유저 본인이 스왑 화면에서 수동으로 어기는 것은 막지 않는다.\r\n\
enforce_lineup=1\r\n\
# 예시 (챔피언 id 는 debug=1 로 생성되는 champ_pos_lock_champions.txt 참고):\r\n\
# lock=alice:top\r\n\
# lock=bright:mid,support\r\n\
";

pub fn load() {
    let mut c = Cfg::empty();
    let path = crate::mod_dir().map(|d| format!("{d}\\{}.cfg", crate::MOD_ID));
    if let Some(p) = &path {
        match std::fs::read_to_string(p) {
            Ok(text) => {
                for raw in text.lines() {
                    let line = raw.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }
                    let Some((k, v)) = line.split_once('=') else {
                        continue;
                    };
                    let (k, v) = (k.trim().to_ascii_lowercase(), v.trim());
                    let on = |s: &str| s == "1" || s.eq_ignore_ascii_case("true");
                    match k.as_str() {
                        "enabled" => c.enabled = on(v),
                        "debug" => c.debug = on(v),
                        "ai_pick_gate" => c.ai_pick_gate = on(v),
                        "ai_assign_mask" => c.ai_assign_mask = on(v),
                        "enforce_lineup" => c.enforce_lineup = on(v),
                        "lock" => {
                            let Some((name, poss)) = v.split_once(':') else {
                                c.load_log.push(format!("lock 파싱 실패(콜론 없음): {v}"));
                                continue;
                            };
                            let mut mask = 0u8;
                            let mut bad = false;
                            for tok in poss.split(',') {
                                match parse_pos(tok) {
                                    Some(b) => mask |= b,
                                    None => {
                                        bad = true;
                                        c.load_log.push(format!("모르는 포지션 '{tok}': {v}"));
                                    }
                                }
                            }
                            if !bad && mask != 0 {
                                c.locks
                                    .push((name.trim().to_ascii_lowercase(), mask & MASK_ALL));
                            }
                        }
                        _ => {}
                    }
                }
                c.load_log.push(format!("locks={}", c.locks.len()));
            }
            Err(_) => {
                // 최초 실행 — 기본 cfg 생성
                let _ = std::fs::write(p, DEFAULT_CFG);
                c.load_log.push("cfg 없음 → 기본 생성".into());
            }
        }
    }
    let _ = CFG.set(c);
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
