//! config — 기능별 on/off + 세부 파라미터. 게임 exe 기준 동적 경로(설치위치 무관).
//! 파일: <게임>\mods\tfm2_view_plus\tfm2_view_plus.cfg
//! 없으면 생성, 키가 빠졌으면 정상값 보존하고 빠진 키만 기본값으로 채워 다시 씀.
//! (순수 std — mod_api 무관)
#![allow(dead_code)]

use std::sync::OnceLock;

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleFileNameW(h: usize, buf: *mut u16, n: u32) -> u32;
}

/// 통합 기능 스위치 + 세부 파라미터.
#[derive(Clone, Debug)]
pub struct Config {
    // ── 기능 마스터 스위치 ──
    pub finance: bool,
    pub coaching: bool,
    pub roster: bool,
    pub recruitment: bool,
    pub facility: bool,
    pub statistics: bool,
    pub banpick: bool,
    pub save_compat: bool,

    // ── finance 세부 ──
    pub finance_ratio: bool, // 항목 옆 비율 % 표시

    // ── banpick 세부 ──
    pub banpick_name_color: bool,   // 이름 버프/너프 색칠
    pub banpick_hover_radar: bool,  // 호버 시 스탯 레이더
    pub banpick_hover_splash: bool, // 호버 배경 스플래시 아트
    pub banpick_bottom_panel: bool, // 하단 패널 정리

    // ── statistics 세부 ──
    pub statistics_name_color: bool, // 이름 버프/너프 색칠

    // ── facility 세부 ──
    pub facility_default_qty: u32, // 공유 수량 팝업 기본값
}

impl Default for Config {
    fn default() -> Self {
        Config {
            finance: true,
            coaching: true,
            roster: true,
            recruitment: true,
            facility: true,
            statistics: true,
            banpick: true,
            save_compat: true,

            finance_ratio: true,

            banpick_name_color: true,
            banpick_hover_radar: true,
            banpick_hover_splash: true,
            banpick_bottom_panel: true,

            statistics_name_color: true,

            facility_default_qty: 1000,
        }
    }
}

static CONFIG: OnceLock<Config> = OnceLock::new();

/// init 시 1회 호출. 파일 로드/생성/보수 후 전역에 저장.
pub fn load() {
    let cfg = read_or_repair();
    let _ = CONFIG.set(cfg);
}

/// 매 프레임 조회. load 전이면 기본값.
pub fn get() -> &'static Config {
    CONFIG.get_or_init(Config::default)
}

fn mod_dir() -> Option<String> {
    let mut buf = [0u16; 512];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), 512) } as usize;
    if n == 0 || n >= 512 {
        return None;
    }
    let exe = String::from_utf16_lossy(&buf[..n]);
    let dir = std::path::Path::new(&exe).parent()?;
    Some(format!("{}\\mods\\tfm2_view_plus", dir.display()))
}

fn cfg_path() -> Option<String> {
    mod_dir().map(|d| format!("{d}\\tfm2_view_plus.cfg"))
}

/// key=value 파싱 (# 주석·빈 줄 무시). 없는 키는 기본값 유지.
fn read_or_repair() -> Config {
    let mut cfg = Config::default();
    let path = match cfg_path() {
        Some(p) => p,
        None => return cfg,
    };

    let mut present: std::collections::HashSet<String> = std::collections::HashSet::new();
    if let Ok(text) = std::fs::read_to_string(&path) {
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((k, v)) = line.split_once('=') else { continue };
            let k = k.trim();
            let v = v.trim();
            present.insert(k.to_string());
            apply(&mut cfg, k, v);
        }
    }

    // 파일이 없거나 키가 빠졌으면 (재)작성 — BOM 없는 UTF-8.
    let need_write = !std::path::Path::new(&path).exists() || missing_any(&present);
    if need_write {
        let _ = write_cfg(&path, &cfg);
    }
    cfg
}

fn apply(cfg: &mut Config, k: &str, v: &str) {
    let b = || matches!(v, "1" | "true" | "on" | "yes" | "TRUE" | "True");
    match k {
        "finance" => cfg.finance = b(),
        "coaching" => cfg.coaching = b(),
        "roster" => cfg.roster = b(),
        "recruitment" => cfg.recruitment = b(),
        "facility" => cfg.facility = b(),
        "statistics" => cfg.statistics = b(),
        "banpick" => cfg.banpick = b(),
        "save_compat" => cfg.save_compat = b(),
        "finance_ratio" => cfg.finance_ratio = b(),
        "banpick_name_color" => cfg.banpick_name_color = b(),
        "banpick_hover_radar" => cfg.banpick_hover_radar = b(),
        "banpick_hover_splash" => cfg.banpick_hover_splash = b(),
        "banpick_bottom_panel" => cfg.banpick_bottom_panel = b(),
        "statistics_name_color" => cfg.statistics_name_color = b(),
        "facility_default_qty" => {
            if let Ok(n) = v.parse::<u32>() {
                cfg.facility_default_qty = n;
            }
        }
        _ => {}
    }
}

const KEYS: &[&str] = &[
    "finance",
    "coaching",
    "roster",
    "recruitment",
    "facility",
    "statistics",
    "banpick",
    "save_compat",
    "finance_ratio",
    "banpick_name_color",
    "banpick_hover_radar",
    "banpick_hover_splash",
    "banpick_bottom_panel",
    "statistics_name_color",
    "facility_default_qty",
];

fn missing_any(present: &std::collections::HashSet<String>) -> bool {
    KEYS.iter().any(|k| !present.contains(*k))
}

fn onoff(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}

fn write_cfg(path: &str, c: &Config) -> std::io::Result<()> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let s = format!(
        "# tfm2_view_plus 설정 — 기능별 on/off (1=켬, 0=끔)\n\
         # 파일은 자동 생성/보수됩니다. 값 수정 후 게임 재시작.\n\
         \n\
         # ── 기능 마스터 스위치 ──\n\
         finance          = {finance}\n\
         coaching         = {coaching}\n\
         roster           = {roster}\n\
         recruitment      = {recruitment}\n\
         facility         = {facility}\n\
         statistics       = {statistics}\n\
         banpick          = {banpick}\n\
         save_compat      = {save_compat}\n\
         \n\
         # ── finance 세부 ──\n\
         finance_ratio    = {finance_ratio}   # 항목 옆 전체대비 비율 %\n\
         \n\
         # ── banpick 세부 ──\n\
         banpick_name_color   = {bnc}   # 이름 버프(초록)/너프(빨강) 색칠\n\
         banpick_hover_radar  = {bhr}   # 호버 시 Lv6.5 스탯 레이더\n\
         banpick_hover_splash = {bhs}   # 호버 배경 스플래시 아트\n\
         banpick_bottom_panel = {bbp}   # 하단 패널 정리\n\
         \n\
         # ── statistics 세부 ──\n\
         statistics_name_color = {snc}  # 이름 버프/너프 색칠\n\
         \n\
         # ── facility 세부 ──\n\
         facility_default_qty  = {fdq}  # 공유 수량 팝업 기본값\n",
        finance = onoff(c.finance),
        coaching = onoff(c.coaching),
        roster = onoff(c.roster),
        recruitment = onoff(c.recruitment),
        facility = onoff(c.facility),
        statistics = onoff(c.statistics),
        banpick = onoff(c.banpick),
        save_compat = onoff(c.save_compat),
        finance_ratio = onoff(c.finance_ratio),
        bnc = onoff(c.banpick_name_color),
        bhr = onoff(c.banpick_hover_radar),
        bhs = onoff(c.banpick_hover_splash),
        bbp = onoff(c.banpick_bottom_panel),
        snc = onoff(c.statistics_name_color),
        fdq = c.facility_default_qty,
    );
    // BOM 없는 UTF-8.
    std::fs::write(path, s.as_bytes())
}
