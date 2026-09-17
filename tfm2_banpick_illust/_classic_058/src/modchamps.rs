//! 모드 챔피언 지원 — 표시명 수확 + 무패치 원본 스탯(파일 base).
//! ===========================================================================
//! 배경(2026-07-29, v1.3.2): 모드 챔피언은 patchviz 의 두 경로 모두에서 빠져 있었다.
//!   ① **표시명 매핑 부재** — `names_gen.rs` 는 바닐라 번들 i18n 에서 생성한 718항목뿐이라
//!      모드 챔프 이름은 draw-list Text 와 매칭되지 않는다(= 이름 색칠 영구 미발화).
//!      종전엔 "툴팁 박스 안 미지 텍스트" 런타임 학습에만 의존 → 호버 전엔 절대 안 되고
//!      게임 재시작마다 소실.
//!   ② **base(무패치 원본) 부재** — base 는 `db.pre_patch_data` 의 最古 스냅샷인데,
//!      모드 챔프는 그 스냅샷에 없을 수 있다(모드 설치 이전 시점 스냅샷이면 당연히 부재).
//!      부재 시 diff 전부 0 ⟹ 레이더가 항상 중립 파랑 + 이름 색칠 없음(유저 제보 증상).
//!
//! 해법 = **모드가 배포하는 원본 파일을 직접 읽는다**(게임 데이터에 의존하지 않는 소스):
//!   - `<모드>\text\*.i18n`         → `<locale>.description.<id>.name` = 표시명 (전 로케일)
//!   - `<모드>\champion\*.data_champion` → `id/stat/growth/attack.duration` = **무패치 원본 스탯**
//!     (모드가 배포한 정의값 그 자체 = 게임 밸런스 패치가 적용되기 전 값)
//!
//! 스캔 범위 = keys.rs 와 동일(게임 `mods\` + 워크샵 `content\3009300\`), 프로세스 1회.
//!
//! ⚠ 사거리 축은 파일 base 에서 비교하지 않는다 — 현재값은 SDK `attack().effect().range`
//!   인데 파일의 `attack.range` 와 단위/의미 일치를 확증하지 못했다(잘못 매칭하면 전 축이
//!   엉뚱한 값으로 표시됨). 공속(duration)·6종 스탯만 비교한다.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

// ── 미니 JSON 파서 (외부 crate 없이 — SDK deps 링크 추가 회피) ──────────────

#[derive(Debug, Clone)]
pub enum J {
    Null,
    B(bool),
    N(f64),
    S(String),
    A(Vec<J>),
    /// 순서 보존 오브젝트.
    O(Vec<(String, J)>),
}

impl J {
    pub fn get(&self, k: &str) -> Option<&J> {
        match self {
            J::O(v) => v.iter().find(|(kk, _)| kk == k).map(|(_, vv)| vv),
            _ => None,
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            J::N(n) => Some(*n),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            J::S(s) => Some(s.as_str()),
            _ => None,
        }
    }
    pub fn obj(&self) -> Option<&Vec<(String, J)>> {
        match self {
            J::O(v) => Some(v),
            _ => None,
        }
    }
}

struct P<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> P<'a> {
    fn ws(&mut self) {
        while self.i < self.b.len() {
            match self.b[self.i] {
                b' ' | b'\t' | b'\r' | b'\n' => self.i += 1,
                // 일부 모드 파일에 // 주석이 섞여 있어도 죽지 않게.
                b'/' if self.i + 1 < self.b.len() && self.b[self.i + 1] == b'/' => {
                    while self.i < self.b.len() && self.b[self.i] != b'\n' {
                        self.i += 1;
                    }
                }
                _ => break,
            }
        }
    }
    fn val(&mut self, depth: u32) -> Option<J> {
        if depth > 64 {
            return None;
        }
        self.ws();
        let c = *self.b.get(self.i)?;
        match c {
            b'{' => {
                self.i += 1;
                let mut out = Vec::new();
                loop {
                    self.ws();
                    match self.b.get(self.i)? {
                        b'}' => {
                            self.i += 1;
                            return Some(J::O(out));
                        }
                        b',' => {
                            self.i += 1;
                        }
                        b'"' => {
                            let k = self.string()?;
                            self.ws();
                            if self.b.get(self.i)? != &b':' {
                                return None;
                            }
                            self.i += 1;
                            let v = self.val(depth + 1)?;
                            out.push((k, v));
                        }
                        _ => return None,
                    }
                }
            }
            b'[' => {
                self.i += 1;
                let mut out = Vec::new();
                loop {
                    self.ws();
                    match self.b.get(self.i)? {
                        b']' => {
                            self.i += 1;
                            return Some(J::A(out));
                        }
                        b',' => {
                            self.i += 1;
                        }
                        _ => out.push(self.val(depth + 1)?),
                    }
                }
            }
            b'"' => Some(J::S(self.string()?)),
            b't' => {
                self.lit("true")?;
                Some(J::B(true))
            }
            b'f' => {
                self.lit("false")?;
                Some(J::B(false))
            }
            b'n' => {
                self.lit("null")?;
                Some(J::Null)
            }
            _ => {
                let s = self.i;
                while self.i < self.b.len() {
                    match self.b[self.i] {
                        b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E' => self.i += 1,
                        _ => break,
                    }
                }
                if s == self.i {
                    return None;
                }
                std::str::from_utf8(&self.b[s..self.i])
                    .ok()?
                    .parse::<f64>()
                    .ok()
                    .map(J::N)
            }
        }
    }
    fn lit(&mut self, s: &str) -> Option<()> {
        if self.b.len() >= self.i + s.len() && &self.b[self.i..self.i + s.len()] == s.as_bytes() {
            self.i += s.len();
            Some(())
        } else {
            None
        }
    }
    fn string(&mut self) -> Option<String> {
        if self.b.get(self.i)? != &b'"' {
            return None;
        }
        self.i += 1;
        let mut out = String::new();
        loop {
            let c = *self.b.get(self.i)?;
            self.i += 1;
            match c {
                b'"' => return Some(out),
                b'\\' => {
                    let e = *self.b.get(self.i)?;
                    self.i += 1;
                    match e {
                        b'n' => out.push('\n'),
                        b't' => out.push('\t'),
                        b'r' => out.push('\r'),
                        b'b' | b'f' => {}
                        b'u' => {
                            let h = self.b.get(self.i..self.i + 4)?;
                            self.i += 4;
                            let n = u32::from_str_radix(std::str::from_utf8(h).ok()?, 16).ok()?;
                            out.push(char::from_u32(n).unwrap_or('?'));
                        }
                        o => out.push(o as char),
                    }
                }
                _ => {
                    // UTF-8 바이트를 그대로 모아 뒤에서 한 번에 해석.
                    let s = self.i - 1;
                    let mut e = self.i;
                    while e < self.b.len() && self.b[e] != b'"' && self.b[e] != b'\\' {
                        e += 1;
                    }
                    out.push_str(&String::from_utf8_lossy(&self.b[s..e]));
                    self.i = e;
                }
            }
        }
    }
}

pub fn parse(s: &[u8]) -> Option<J> {
    // BOM 관용.
    let s = if s.starts_with(&[0xEF, 0xBB, 0xBF]) { &s[3..] } else { s };
    P { b: s, i: 0 }.val(0)
}

// ── 모드 폴더 스캔 ────────────────────────────────────────────────────────

/// 모드가 설치될 수 있는 루트들(게임 `mods\`, 워크샵 `content\3009300\`).
/// 경로 하드코딩 금지 규칙에 따라 전부 이 모드 폴더 기준으로 도출한다(keys.rs 와 동일).
fn mod_roots() -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Some(dir) = crate::mod_dir() else {
        return out;
    };
    let me = Path::new(&dir); // ...\<게임>\mods\tfm2_banpick_illust
    if let Some(game) = me.parent().and_then(|p| p.parent()) {
        out.push(game.join("mods"));
        if let Some(steamapps) = game.parent().and_then(|p| p.parent()) {
            out.push(steamapps.join("workshop").join("content").join("3009300"));
        }
    }
    out
}

fn mod_dirs() -> Vec<PathBuf> {
    let mut out = Vec::new();
    for r in mod_roots() {
        if let Ok(rd) = std::fs::read_dir(&r) {
            for e in rd.flatten() {
                let p = e.path();
                if p.is_dir() {
                    out.push(p);
                }
            }
        }
    }
    out
}

fn files_in(dir: &Path, ext: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(rd) = std::fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.extension().and_then(|s| s.to_str()) == Some(ext) {
                out.push(p);
            }
        }
    }
    out
}

// ── ① 표시명 수확 (text\*.i18n 의 description.<id>.name) ────────────────────

/// 표시명(trim) → champ id. 서로 다른 id 가 같은 표시명을 쓰면 **제외**(오색칠 방지,
/// names_gen 생성 정책과 동일).
pub fn names() -> &'static HashMap<String, String> {
    static N: OnceLock<HashMap<String, String>> = OnceLock::new();
    N.get_or_init(|| {
        let mut map: HashMap<String, String> = HashMap::new();
        let mut bad: Vec<String> = Vec::new();
        let known = file_bases();
        for d in mod_dirs() {
            for f in files_in(&d.join("text"), "i18n") {
                // ⚠ `description.<id>.name` 구조는 아이템 i18n 도 동일하다 ⟹ 챔프 파일이
                //   아니면 **data_champion 으로 실존이 확인된 id 만** 받는다(아이템 이름이
                //   챔프 표시명으로 등록되는 오염 방지).
                let champ_file = f.file_name().and_then(|s| s.to_str()) == Some("champion.i18n");
                let Ok(bytes) = std::fs::read(&f) else { continue };
                let Some(j) = parse(&bytes) else { continue };
                // { "<locale>": { "description": { "<id>": { "name": "..." } } } }
                let Some(locales) = j.obj() else { continue };
                for (_loc, lv) in locales {
                    let Some(desc) = lv.get("description").and_then(|d| d.obj()) else {
                        continue;
                    };
                    for (id, entry) in desc {
                        if !champ_file && !known.contains_key(id) {
                            continue;
                        }
                        let Some(nm) = entry.get("name").and_then(|n| n.as_str()) else {
                            continue;
                        };
                        let nm = nm.trim();
                        if nm.is_empty() || nm.chars().count() > 40 {
                            continue;
                        }
                        match map.get(nm) {
                            Some(prev) if prev != id => bad.push(nm.to_string()),
                            _ => {
                                map.insert(nm.to_string(), id.clone());
                            }
                        }
                    }
                }
            }
        }
        for b in bad {
            map.remove(&b);
        }
        map
    })
}

// ── ② 무패치 원본 스탯 (champion\*.data_champion) ───────────────────────────

/// 파일에서 읽은 원본 스탯. 축 순서 = patchviz 와 동일한 6종
/// (공격/주문력/체력/방어/마저/이속) + 공격 duration(공속 프록시).
#[derive(Clone, Copy, Debug, Default)]
pub struct FileBase {
    pub lv1: [f64; 6],
    pub grow: [f64; 6],
    pub dur: f64,
}

const STAT_KEYS: [&str; 6] = [
    "attack",
    "magic_power",
    "hp",
    "defence",
    "magic_resistance",
    "move_speed",
];

fn read_stat(j: Option<&J>) -> [f64; 6] {
    let mut out = [0f64; 6];
    if let Some(o) = j {
        for (i, k) in STAT_KEYS.iter().enumerate() {
            out[i] = o.get(k).and_then(|v| v.as_f64()).unwrap_or(0.0);
        }
    }
    out
}

/// champ id → 모드가 배포한 원본 스탯.
pub fn file_bases() -> &'static HashMap<String, FileBase> {
    static B: OnceLock<HashMap<String, FileBase>> = OnceLock::new();
    B.get_or_init(|| {
        let mut map: HashMap<String, FileBase> = HashMap::new();
        for d in mod_dirs() {
            for f in files_in(&d.join("champion"), "data_champion") {
                let Ok(bytes) = std::fs::read(&f) else { continue };
                let Some(j) = parse(&bytes) else { continue };
                // id 는 파일 안 값 우선, 없으면 파일명(stem).
                let id = j
                    .get("id")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .or_else(|| f.file_stem().and_then(|s| s.to_str()).map(|s| s.to_string()));
                let Some(id) = id else { continue };
                map.insert(
                    id,
                    FileBase {
                        lv1: read_stat(j.get("stat")),
                        grow: read_stat(j.get("growth")),
                        dur: j
                            .get("attack")
                            .and_then(|a| a.get("duration"))
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0),
                    },
                );
            }
        }
        map
    })
}

/// 진단 요약 한 줄.
pub fn summary() -> String {
    format!(
        "모드챔프 파일: 표시명 {}개 / 원본스탯 {}개 (스캔 루트 {}개)",
        names().len(),
        file_bases().len(),
        mod_roots().len()
    )
}
