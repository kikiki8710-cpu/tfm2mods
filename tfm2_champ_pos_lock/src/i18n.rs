//! i18n — 사용자 표시 문자열의 언어 분리 (2026-08-23)
//!
//! ## 저장소는 하나, 소비 경로는 둘
//! 정본 = `text/poslock.i18n` (JSON: `lang > 그룹 > 키`). 이 한 파일을 두 경로가 쓴다.
//!
//! 1. **`.ui` 정적 라벨** — 게임이 직접 해석한다.
//!    `mod.override_info` 의 merge 엔트리로 게임의 `asset/base/text/ui` 에 우리 파일을 **재귀 병합**하고,
//!    `.ui` 에서 `#asset/base/text/ui?pos_lock.<키>` 로 참조한다. 언어 전환은 게임이 알아서 한다.
//!    (`.i18n` 은 merge 가능 확장자 화이트리스트에 있다 — [[tfm2-asset-override-merge]])
//! 2. **런타임 조합 문자열** — 여기 `tr()` 로 우리가 조회한다.
//!    ⚠**게임의 `#asset` 참조는 조합 문자열 중간(인라인)에서 안 풀린다**(tfm2_elemental_serpen 2026-07-25 실측).
//!      `"현재 밴카드 수: 3"` 처럼 숫자를 끼워 만드는 문자열은 우리가 직접 만들어야 한다.
//!
//! ## 언어
//! `<게임 설치 폴더>\config\game\base.json` 의 `lang` 을 따른다(게임 UI 언어와 동일).
//! ⚠게임과 언어를 맞춰야 한다 — 어긋나면 폰트 불일치로 글자가 깨진다(영어 폰트 + 한글 = □).
//! 없는 키는 `en` 으로 폴백하고, 그래도 없으면 키 자체를 노출한다(누락이 즉시 눈에 띄도록).
//!
//! ## 파일이 없을 때
//! dll 에 `include_str!` 로 사본을 박아 두고, 모드 폴더에 파일이 없으면 그걸 쓴다 + 파일도 만들어 준다.
//! ⚠단 **`.ui` 라벨 경로는 게임 시작 시점에 파일이 있어야** 해석된다(런타임 생성은 다음 실행부터 유효).

use std::collections::HashMap;
use std::sync::Mutex;

/// 배포 폴더에 파일이 없을 때 쓰는 사본 + 자동 생성 원본.
const EMBEDDED: &str = include_str!("../assets/text/poslock.i18n");
/// 모드 폴더 기준 상대 경로. `mod.override_info` 의 remapping 과 반드시 같은 이름이어야 한다.
pub const REL_PATH: &str = "text\\poslock.i18n";
/// 우리 키가 사는 그룹(게임 `base/text/ui` 에 병합되므로 다른 모드와 안 겹치게 네임스페이스).
const GROUP: &str = "pos_lock";

static TABLE: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);
/// 마지막으로 테이블을 만든 언어. 게임 언어가 바뀌면 다시 만든다.
static LOADED_LANG: Mutex<Option<String>> = Mutex::new(None);
/// base.json 재확인 주기(프레임). 매 프레임 파일을 읽을 이유는 없다.
static TICK: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

// ── 미니 JSON 파서 (tfm2_scrim → tfm2_elemental_serpen 계보 이식) ──────────
enum J {
    Str(String),
    Obj(Vec<(String, J)>),
    Other,
}
impl J {
    fn get(&self, key: &str) -> Option<&J> {
        match self {
            J::Obj(o) => o.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }
}
struct P<'a> {
    b: &'a [u8],
    i: usize,
}
impl<'a> P<'a> {
    fn new(s: &'a str) -> Self {
        P { b: s.as_bytes(), i: 0 }
    }
    fn ws(&mut self) {
        while self.i < self.b.len() {
            match self.b[self.i] {
                b' ' | b'\t' | b'\r' | b'\n' | b',' => self.i += 1,
                _ => break,
            }
        }
    }
    fn value(&mut self) -> Option<J> {
        self.ws();
        match *self.b.get(self.i)? {
            b'{' => self.object(),
            b'[' => {
                // 이 파일 형식엔 배열이 없다. 나와도 안전하게 건너뛴다.
                self.i += 1;
                let mut depth = 1;
                while self.i < self.b.len() && depth > 0 {
                    match self.b[self.i] {
                        b'[' => depth += 1,
                        b']' => depth -= 1,
                        b'"' => {
                            self.string()?;
                            continue;
                        }
                        _ => {}
                    }
                    self.i += 1;
                }
                Some(J::Other)
            }
            b'"' => self.string().map(J::Str),
            _ => {
                while self.i < self.b.len() {
                    match self.b[self.i] {
                        b',' | b'}' | b']' => break,
                        _ => self.i += 1,
                    }
                }
                Some(J::Other)
            }
        }
    }
    fn string(&mut self) -> Option<String> {
        if *self.b.get(self.i)? != b'"' {
            return None;
        }
        self.i += 1;
        let mut out: Vec<u8> = Vec::new();
        while self.i < self.b.len() {
            let c = self.b[self.i];
            self.i += 1;
            match c {
                b'"' => return Some(String::from_utf8_lossy(&out).into_owned()),
                b'\\' => {
                    let e = *self.b.get(self.i)?;
                    self.i += 1;
                    match e {
                        b'n' => out.push(b'\n'),
                        b't' => out.push(b'\t'),
                        b'r' => out.push(b'\r'),
                        b'u' => {
                            if self.i + 4 <= self.b.len() {
                                if let Some(cp) = std::str::from_utf8(&self.b[self.i..self.i + 4])
                                    .ok()
                                    .and_then(|h| u32::from_str_radix(h, 16).ok())
                                    .and_then(char::from_u32)
                                {
                                    let mut buf = [0u8; 4];
                                    out.extend_from_slice(cp.encode_utf8(&mut buf).as_bytes());
                                }
                                self.i += 4;
                            }
                        }
                        other => out.push(other),
                    }
                }
                _ => out.push(c),
            }
        }
        None
    }
    fn object(&mut self) -> Option<J> {
        self.i += 1;
        let mut pairs = Vec::new();
        loop {
            self.ws();
            match *self.b.get(self.i)? {
                b'}' => {
                    self.i += 1;
                    break;
                }
                _ => {}
            }
            let k = self.string()?;
            self.ws();
            if *self.b.get(self.i)? != b':' {
                return None;
            }
            self.i += 1;
            let v = self.value()?;
            pairs.push((k, v));
        }
        Some(J::Obj(pairs))
    }
}

/// 게임 UI 언어 = `<게임 설치 폴더>\config\game\base.json` 의 `lang`.
fn game_lang() -> Option<String> {
    let d = crate::mod_dir()?; // <게임>\mods\<MOD_ID>
    let p = std::path::Path::new(&d)
        .parent()?
        .parent()?
        .join("config")
        .join("game")
        .join("base.json");
    let txt = std::fs::read_to_string(p).ok()?;
    P::new(&txt)
        .value()?
        .get("lang")
        .and_then(|v| match v {
            J::Str(s) => Some(s.to_lowercase()),
            _ => None,
        })
}

fn merge_lang(map: &mut HashMap<String, String>, root: &J, lang: &str) {
    let Some(J::Obj(groups)) = root.get(lang) else { return };
    for (g, gobj) in groups {
        if let J::Obj(keys) = gobj {
            for (k, v) in keys {
                if let J::Str(s) = v {
                    map.insert(format!("{g}.{k}"), s.clone());
                }
            }
        }
    }
}

/// ★게임 언어가 바뀌었으면 테이블을 다시 만든다.
///   ⚠**언어는 게임 실행 중에 바뀔 수 있다.** `.ui` 라벨은 게임이 매번 다시 해석해 바로 따라가지만,
///     우리 테이블은 시작 시 1회 로드라 그대로 남는다 ⟹ 화면 절반만 번역되고, 영어 폰트에 한글이
///     들어가 **글자가 깨진다**(2026-08-23 실측: 왼쪽 .ui 는 영어인데 오른쪽 조합 문자열만 □□□).
///   비용은 수백 바이트 JSON 1회 읽기라, 주기적으로 확인하는 편이 안전하다.
pub fn poll_lang() {
    let n = TICK.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if n % 300 != 0 {
        return; // ~5초에 한 번만 파일 확인
    }
    poll_now();
}

/// 주기를 기다리지 않고 **지금** 확인한다(팝업을 여는 순간처럼 결과가 바로 보여야 할 때).
pub fn poll_now() {
    let cur = game_lang().unwrap_or_else(|| "en".to_string());
    let same = LOADED_LANG
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .as_deref()
        == Some(cur.as_str());
    if !same {
        load();
    }
}

/// 로드(재로드 포함). 파일이 없으면 내장 사본을 쓰고 파일도 만들어 준다(`.ui` 경로는 다음 실행부터 유효).
pub fn load() {
    let path = crate::mod_dir().map(|d| format!("{d}\\{REL_PATH}"));
    let txt = match path.as_ref().and_then(|p| std::fs::read_to_string(p).ok()) {
        Some(t) => t,
        None => {
            if let Some(p) = path.as_ref() {
                if let Some(parent) = std::path::Path::new(p).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                // ⚠BOM 없는 UTF-8 — 게임이 읽는 에셋이다.
                let _ = std::fs::write(p, EMBEDDED.as_bytes());
            }
            EMBEDDED.to_string()
        }
    };
    let lang = game_lang().unwrap_or_else(|| "en".to_string());
    let mut map = HashMap::new();
    if let Some(root) = P::new(&txt).value() {
        merge_lang(&mut map, &root, "en"); // en 베이스(폴백)
        if lang != "en" {
            merge_lang(&mut map, &root, &lang); // 선택 언어로 덮어쓰기
        }
    }
    remember_lang(&lang);
    *TABLE.lock().unwrap_or_else(|e| e.into_inner()) = Some(map);
}

/// 위 `load()` 안에서 확정한 언어를 기록(중복 로드 방지). `load()` 말미에서 호출된다.
fn remember_lang(lang: &str) {
    *LOADED_LANG.lock().unwrap_or_else(|e| e.into_inner()) = Some(lang.to_string());
}

/// 키 조회. 미존재 시 키 자체를 반환한다(누락이 화면에서 바로 보이도록).
pub fn tr(key: &str) -> String {
    let full = format!("{GROUP}.{key}");
    let g = TABLE.lock().unwrap_or_else(|e| e.into_inner());
    match g.as_ref().and_then(|m| m.get(&full)) {
        Some(v) => v.clone(),
        None => full,
    }
}

/// 키 조회 + `{이름}` 치환.
pub fn trf(key: &str, args: &[(&str, &str)]) -> String {
    let mut s = tr(key);
    for (k, v) in args {
        s = s.replace(&format!("{{{k}}}"), v);
    }
    s
}

/// 포지션 표시명(탭 라벨과 같은 키를 쓴다 — 한 곳만 고치면 둘 다 바뀐다).
pub fn pos_name(p: usize) -> String {
    tr(["tab_top", "tab_jungle", "tab_mid", "tab_bottom", "tab_support"][p.min(4)])
}
