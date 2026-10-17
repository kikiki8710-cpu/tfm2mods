//! 잘못된 설정은 설치를 거절한다. 원본 판단을 바꾸는 설정은 없다.
use std::collections::BTreeSet;

#[derive(Clone, Debug)]
pub struct Config {
    pub armed: bool,
    pub probes: BTreeSet<usize>,
    pub sample_stride: u64,
    pub max_samples_per_probe: u64,
    pub queue_capacity: usize,
    pub install_delay_frames: u64,
    pub allow_foreign_chain: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { armed: false, probes: BTreeSet::new(), sample_stride: 64,
            max_samples_per_probe: 1000, queue_capacity: 256,
            install_delay_frames: 120, allow_foreign_chain: false }
    }
}

pub fn parse(text: &str) -> Result<Config, String> {
    let mut out = Config::default();
    let mut seen = BTreeSet::new();
    for (line, raw) in text.lines().enumerate() {
        let value = raw.split('#').next().unwrap_or("").trim();
        if value.is_empty() { continue; }
        let (key, value) = value.split_once('=').ok_or_else(|| format!("{}행: key=value 필요", line + 1))?;
        let key = key.trim(); let value = value.trim();
        if !seen.insert(key.to_string()) { return Err(format!("중복 설정: {key}")); }
        fn boolean(s: &str) -> Result<bool, String> {
            match s { "true" => Ok(true), "false" => Ok(false), _ => Err("bool은 true/false만 허용".into()) }
        }
        fn number(s: &str, lo: u64, hi: u64) -> Result<u64, String> {
            let n = s.parse::<u64>().map_err(|_| "정수 설정 오류".to_string())?;
            if !(lo..=hi).contains(&n) { return Err(format!("범위 {lo}..{hi} 밖")); }
            Ok(n)
        }
        match key {
            "armed" => out.armed = boolean(value)?,
            "allow_foreign_chain" => out.allow_foreign_chain = boolean(value)?,
            "sample_stride" => out.sample_stride = number(value, 1, 1_000_000)?,
            "max_samples_per_probe" => out.max_samples_per_probe = number(value, 1, 1_000_000)?,
            "queue_capacity" => out.queue_capacity = number(value, 8, 4096)? as usize,
            "install_delay_frames" => out.install_delay_frames = number(value, 60, 3600)?,
            "probes" => {
                for item in value.split(',').map(str::trim).filter(|s| !s.is_empty()) {
                    let rva = usize::from_str_radix(item.trim_start_matches("0x"), 16)
                        .map_err(|_| format!("잘못된 RVA: {item}"))?;
                    if !out.probes.insert(rva) { return Err(format!("중복 probe: {item}")); }
                }
            }
            _ => return Err(format!("알 수 없는 설정: {key}")),
        }
    }
    if out.armed && out.probes.is_empty() { return Err("armed=true에는 probes가 필요".into()); }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn malformed_configuration_never_arms() {
        for x in ["armed=typo", "armed=true", "armed=true\nprobes=XYZ", "sample_stride=0",
                  "armed=false\narmed=true", "queue_capacity=99999", "unknown=1"] {
            assert!(parse(x).is_err(), "{x}");
        }
        assert!(!parse("").unwrap().armed);
        let c = parse("armed=true\nprobes=dd87a0,e52ee0\nsample_stride=1").unwrap();
        assert!(c.armed && c.probes.len() == 2 && c.sample_stride == 1);
    }
}
