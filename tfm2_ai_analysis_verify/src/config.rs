//! 잘못된 설정은 설치를 거절한다. 원본 판단을 바꾸는 설정은 없다.
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
pub struct Config {
    pub armed: bool,
    pub probes: BTreeSet<usize>,
    pub sample_stride: u64,
    pub probe_strides: BTreeMap<usize, u64>,
    pub max_samples_per_probe: u64,
    pub queue_capacity: usize,
    pub install_delay_frames: u64,
    pub allow_foreign_chain: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { armed: false, probes: BTreeSet::new(), sample_stride: 4096, probe_strides: BTreeMap::new(),
            max_samples_per_probe: 5000, queue_capacity: 4096,
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
            "probe_strides" => {
                // 이 옵션의 내부 공백/빈 항목/중복 정규화 RVA를 숨기지 않는다.
                if value.is_empty() || value.chars().any(char::is_whitespace) {
                    return Err("probe_strides는 공백 없는 RVA:stride 목록 필요".into());
                }
                for item in value.split(',') {
                    let (address, stride) = item.split_once(':')
                        .ok_or("probe_strides 항목은 RVA:stride 필요")?;
                    let hex = address.strip_prefix("0x").unwrap_or(address);
                    if hex.is_empty() || !hex.bytes().all(|x| x.is_ascii_hexdigit()) {
                        return Err(format!("probe_strides 잘못된 RVA: {address}"));
                    }
                    let rva = usize::from_str_radix(hex, 16)
                        .map_err(|_| "probe_strides RVA 범위 오류")?;
                    if stride.is_empty() || !stride.bytes().all(|x| x.is_ascii_digit()) {
                        return Err("probe_strides stride는 부호 없는 십진정수 필요".into());
                    }
                    let stride = number(stride, 1, 1_000_000)?;
                    if out.probe_strides.insert(rva, stride).is_some() {
                        return Err(format!("중복 probe_strides RVA: 0x{rva:x}"));
                    }
                }
            }
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
    for rva in out.probe_strides.keys() {
        if !out.probes.contains(rva) { return Err(format!("선택되지 않은 probe_strides RVA: 0x{rva:x}")); }
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
    #[test] fn probe_stride_default_and_explicit_map_order() {
        let d=parse("").unwrap();assert!(d.probe_strides.is_empty());assert_eq!(d.sample_stride,4096);
        let c=parse("probe_strides=dc0bb0:1,0xF27690:17\nprobes=dc0bb0,f27690\nsample_stride=4096").unwrap();
        assert_eq!(c.probe_strides[&0xdc0bb0],1);assert_eq!(c.probe_strides[&0xf27690],17);
        assert_eq!(c.sample_stride,4096);
    }
    #[test] fn probe_stride_duplicate_rva_aliases_are_errors() {
        assert!(parse("probes=dc0bb0\nprobe_strides=dc0bb0:1\nprobe_strides=dc0bb0:2").is_err());
        for value in ["dc0bb0:1,dc0bb0:2","dc0bb0:1,0x00DC0BB0:2"] {
            assert!(parse(&format!("probes=dc0bb0\nprobe_strides={value}")).is_err());
        }
    }
    #[test] fn probe_stride_unselected_rva_is_error_even_disarmed() {
        assert!(parse("armed=false\nprobes=dc0bb0\nprobe_strides=f27690:1").is_err());
    }
    #[test] fn probe_stride_zero_limit_and_bad_decimal_are_errors() {
        for value in ["0","1000001","18446744073709551616","+1","-1","1:2"] {
            assert!(parse(&format!("probes=dc0bb0\nprobe_strides=dc0bb0:{value}")).is_err());
        }
        assert_eq!(parse("probes=dc0bb0\nprobe_strides=dc0bb0:1000000").unwrap().probe_strides[&0xdc0bb0],1000000);
    }
    #[test] fn probe_stride_whitespace_hex_and_empty_items_are_errors() {
        for value in ["dc0 bb0:1","dc0bb0 :1","dc0bb0: 1","dc0bb0:1, f27690:1",":1","0x:1","0Xdc0bb0:1","0x0xdc0bb0:1","dc0bb0:","dc0bb0:1,",",dc0bb0:1",""] {
            assert!(parse(&format!("probes=dc0bb0,f27690\nprobe_strides={value}")).is_err(),"{value}");
        }
        assert!(parse("probes=dc0bb0\nprobe_strides=ffffffffffffffffffffffff:1").is_err());
    }

}
