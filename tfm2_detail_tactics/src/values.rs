//! values — 단계 값 설정 파일 `detail_tactics_values.cfg`(모드 폴더 · 전 세이브 공통).
//! 유저 지시(2026-10-22): "전투중 후퇴체력이 35였던거를 낮게하면 20, 높게하면 50 … 설정파일에 15, 60 이런식으로 바꾸면 설정값이 바뀌는 느낌".
//! 형식 = `[전술 id]` 아래 `낮게 = 값` / `높게 = 값`. `기본` 줄은 원본 표시용(읽지 않음). # 이후는 주석.
//! 파일이 없으면 기본 내용으로 만든다. 고친 파일은 수정 시각이 바뀌면 다시 읽는다(재시작 불필요).
//! 범위 밖·숫자 아닌 값 = 그 단계만 원본값으로(= 그 단계를 골라도 바뀌지 않음) + 경고 로그. 나머지는 정상 적용.
//! ⚠ BOM 없는 UTF-8 로 쓴다(CLAUDE.md §2 함정 ②). 읽을 때는 BOM 이 있어도 벗겨 낸다.
use crate::tactics::{self, Kind, Scope, LV_HIGH, LV_LOW, TACTICS};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;

pub const FILE_NAME: &str = "detail_tactics_values.cfg";

/// 파일이 없을 때 만드는 내용(정의 기본값).
pub fn default_text() -> String {
    let mut s = String::new();
    s.push_str("# 상세전술 단계 값 — 전술마다 '낮게'/'높게'를 골랐을 때 실제로 쓰는 값.\n");
    s.push_str("# 숫자만 고치면 된다(게임 실행 중에 고쳐도 몇 초 안에 다시 읽음). '기본' 줄은 원본값 표시용이라 고쳐도 무시한다.\n");
    s.push_str("# 범위 밖 값은 그 단계만 원본값으로 돌아간다(로그 detail_tactics.log 에 경고). 켬·끔 전술은 값이 없다.\n");
    s.push_str("# 파일을 지우면 다음 실행 때 기본 내용으로 다시 만든다.\n\n");
    for d in TACTICS {
        s.push_str(&format!("[{}]\n", d.id));
        s.push_str(&format!("이름 = {}\n", d.name));
        s.push_str(&format!("설명 = {}\n", d.desc));
        s.push_str(&format!("적용 = {}\n", if d.scope == Scope::Team { "팀 전체" } else { "선수(포지션별)" }));
        match d.kind {
            Kind::Toggle => s.push_str("기본 = 켬      # 켬·끔 전술 — 모달에서 '끔'을 고르면 꺼진다\n"),
            Kind::Value => {
                s.push_str(&format!("기본 = {}      # 원본(표시용 — 고쳐도 무시)\n", d.default));
                s.push_str(&format!("낮게 = {}\n", d.low));
                s.push_str(&format!("높게 = {}\n", d.high));
                s.push_str(&format!("# 허용 범위 {} ~ {}\n", d.min, d.max));
            }
        }
        s.push('\n');
    }
    s
}

/// 읽은 결과: 적용한 값 수, 경고 목록.
pub struct Parsed { pub applied: usize, pub warnings: Vec<String> }

/// 설정 본문을 읽어 단계 값을 바꾼다(먼저 전부 정의 기본값으로 되돌린 뒤).
pub fn apply_text(txt: &str) -> Parsed {
    tactics::reset_values();
    let txt = txt.trim_start_matches('\u{feff}');
    let mut cur: Option<usize> = None;
    let mut out = Parsed { applied: 0, warnings: Vec::new() };
    for (no, raw) in txt.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() { continue; }
        if line.starts_with('[') && line.ends_with(']') {
            let id = line[1..line.len() - 1].trim();
            cur = tactics::index_of(id);
            if cur.is_none() { out.warnings.push(format!("{}행: 모르는 전술 [{}] — 건너뜀", no + 1, id)); }
            continue;
        }
        let Some(t) = cur else { continue };
        let Some((k, v)) = line.split_once('=') else {
            out.warnings.push(format!("{}행: '키 = 값' 꼴이 아님 — 건너뜀", no + 1));
            continue;
        };
        let lv = match k.trim() {
            "낮게" | "low" => LV_LOW,
            "높게" | "high" => LV_HIGH,
            _ => continue, // 이름·설명·적용·기본 = 표시용
        };
        let d = &TACTICS[t];
        if d.kind == Kind::Toggle { continue; }
        match v.trim().parse::<i64>() {
            Ok(n) if tactics::set_value(t, lv, n) => out.applied += 1,
            Ok(n) => {
                tactics::set_value(t, lv, d.default);
                out.warnings.push(format!("{}행: [{}] {} = {} 는 허용 범위 {}~{} 밖 — 이 단계는 원본값 {}", no + 1, d.id, k.trim(), n, d.min, d.max, d.default));
            }
            Err(_) => {
                tactics::set_value(t, lv, d.default);
                out.warnings.push(format!("{}행: [{}] {} 값 '{}' 가 숫자가 아님 — 이 단계는 원본값 {}", no + 1, d.id, k.trim(), v.trim(), d.default));
            }
        }
    }
    out
}

struct Watch { path: PathBuf, mtime: Option<SystemTime>, checked_frame: u64 }
static WATCH: Mutex<Option<Watch>> = Mutex::new(None);

/// 시작 시 1회 — 파일이 없으면 만들고 읽는다. 반환 = 로그 줄.
pub fn init(mod_dir: &Path) -> Vec<String> {
    let path = mod_dir.join(FILE_NAME);
    let mut log = Vec::new();
    if !path.exists() {
        match std::fs::write(&path, default_text().as_bytes()) {
            Ok(()) => log.push(format!("설정 파일 새로 만듦: {}", path.display())),
            Err(e) => log.push(format!("설정 파일 만들기 실패({e}) — 정의 기본값 사용")),
        }
    }
    *WATCH.lock().unwrap_or_else(|e| e.into_inner()) = Some(Watch { path, mtime: None, checked_frame: 0 });
    log.extend(reload_if_changed(0, true));
    log
}

/// 매 프레임 불러도 된다 — 120프레임마다 수정 시각만 보고, 바뀌었으면 다시 읽는다.
pub fn reload_if_changed(frame: u64, force: bool) -> Vec<String> {
    let mut g = WATCH.lock().unwrap_or_else(|e| e.into_inner());
    let Some(w) = g.as_mut() else { return Vec::new() };
    if !force && frame.saturating_sub(w.checked_frame) < 120 { return Vec::new(); }
    w.checked_frame = frame;
    let mtime = std::fs::metadata(&w.path).and_then(|m| m.modified()).ok();
    if !force && mtime == w.mtime { return Vec::new(); }
    w.mtime = mtime;
    let mut log = Vec::new();
    match std::fs::read(&w.path) {
        Ok(bytes) => {
            let txt = String::from_utf8_lossy(&bytes);
            let p = apply_text(&txt);
            log.push(format!("설정 파일 읽음: 값 {}개 적용 · 경고 {}건", p.applied, p.warnings.len()));
            log.extend(p.warnings);
        }
        Err(e) => {
            tactics::reset_values();
            log.push(format!("설정 파일 읽기 실패({e}) — 정의 기본값 사용"));
        }
    }
    log
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactics::{index_of, value_of};
    #[test]
    fn default_text_round_trips() {
        if let Some(dir)=std::env::var_os("DT_EXPORT_DIR") {
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(std::path::Path::new(&dir).join(FILE_NAME),default_text()).unwrap();
        }
        let p = apply_text(&default_text());
        assert!(p.warnings.is_empty(), "{:?}", p.warnings);
        for (t, d) in TACTICS.iter().enumerate() {
            if d.kind == Kind::Value {
                assert_eq!(value_of(t, LV_LOW), d.low, "{}", d.id);
                assert_eq!(value_of(t, LV_HIGH), d.high, "{}", d.id);
            }
        }
    }
    #[test]
    fn edits_and_bad_values() {
        let t = index_of("combat_retreat_hp").unwrap();
        let p = apply_text("\u{feff}[combat_retreat_hp]\n낮게 = 15 # 주석\n높게 = 999\n[nope]\n낮게 = 3\n");
        assert_eq!(value_of(t, LV_LOW), 15);
        assert_eq!(value_of(t, LV_HIGH), TACTICS[t].default); // 범위 밖 → 원본값
        assert_eq!(p.applied, 1);
        assert_eq!(p.warnings.len(), 2);
        apply_text("");
        assert_eq!(value_of(t, LV_LOW), TACTICS[t].low); // 다시 읽으면 정의 기본값부터
    }
}
