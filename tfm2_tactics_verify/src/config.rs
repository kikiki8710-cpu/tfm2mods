//! tactics_verify.cfg — 잘못된 설정은 설치를 거절한다(조용히 무시하지 않는다).
use std::collections::BTreeSet;

#[derive(Clone, Debug)]
pub struct Config {
    pub armed: bool,
    pub label: String,
    /// 첫 post_update 뒤 이 프레임 수가 지나면 현재 화면에서 한 번 설치한다.
    pub install_delay_frames: u64,
    /// 사이트별 (관전+배경) 호출이 이 값을 넘으면 그 게이트를 무장 해제한다. 0 = 무제한.
    pub max_hits: u64,
    pub summary_interval_ms: u64,
    /// 게이트 id 접두 목록. 비어 있으면 전부.
    pub gates: Vec<String>,
    /// 문턱 바꾸기(논리값). 같은 id 두 번은 거절.
    pub sets: Vec<(String, i64)>,
}

impl Default for Config {
    fn default() -> Self {
        Self { armed: false, label: "base".into(), install_delay_frames: 60, max_hits: 50_000_000,
            summary_interval_ms: 2000, gates: Vec::new(), sets: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defaults_and_multiple_sets() {
        assert!(!parse("").unwrap().armed);
        let cfg = parse("armed=true\nlabel=trial_A # 주석\nset=a=55\nset=b=-1\ngates=hp, count").unwrap();
        assert!(cfg.armed); assert_eq!(cfg.label, "trial_A");
        assert_eq!(cfg.sets, vec![("a".into(), 55), ("b".into(), -1)]);
        assert_eq!(cfg.gates, vec!["hp", "count"]);
    }
    #[test]
    fn invalid_configs_are_rejected() {
        for cfg in ["armed=1", "armed=false\narmed=true", "set=a=1\nset=a=2",
            "unknown=1", "summary_interval_ms=0", "install_delay_frames=0", "label=../bad", "set=a=bad"] {
            assert!(parse(cfg).is_err(), "{cfg}");
        }
    }
}

pub fn parse(text: &str) -> Result<Config, String> {
    let mut out = Config::default();
    let mut seen = BTreeSet::new();
    let mut set_ids = BTreeSet::new();
    for (line, raw) in text.lines().enumerate() {
        let value = raw.split('#').next().unwrap_or("").trim().trim_start_matches('\u{feff}');
        if value.is_empty() { continue; }
        let (key, value) = value.split_once('=').ok_or_else(|| format!("{}행: key=value 필요", line + 1))?;
        let key = key.trim(); let value = value.trim();
        if key != "set" && !seen.insert(key.to_string()) { return Err(format!("중복 설정: {key}")); }
        fn number(s: &str, lo: u64, hi: u64) -> Result<u64, String> {
            let n = s.parse::<u64>().map_err(|_| format!("정수 설정 오류: {s}"))?;
            if !(lo..=hi).contains(&n) { return Err(format!("범위 {lo}..{hi} 밖: {n}")); }
            Ok(n)
        }
        match key {
            "armed" => out.armed = match value { "true" => true, "false" => false, _ => return Err("armed 는 true/false".into()) },
            "label" => {
                if value.is_empty() || !value.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
                    return Err("label 은 영숫자/_/- 만".into());
                }
                out.label = value.to_string();
            }
            "install_delay_frames" => out.install_delay_frames = number(value, 1, 100_000)?,
            "max_hits" => out.max_hits = number(value, 0, u64::MAX / 2)?,
            "summary_interval_ms" => out.summary_interval_ms = number(value, 250, 60_000)?,
            "gates" => out.gates = value.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect(),
            "set" => {
                let (id, v) = value.split_once('=').ok_or("set 은 set=<id>=<값>")?;
                let id = id.trim();
                let v = v.trim().parse::<i64>().map_err(|_| format!("set {id}: 정수 필요"))?;
                if !set_ids.insert(id.to_string()) { return Err(format!("set 중복: {id}")); }
                out.sets.push((id.to_string(), v));
            }
            _ => return Err(format!("알 수 없는 설정: {key}")),
        }
    }
    Ok(out)
}
