//! lang — 화면 문구 언어(2026-11-11 · 공통 지시서 MIG/_patchday/0.7.2/i18n/BRIEF_I18N.md).
//! 게임 i18n 샘플 문구에 한글이 있으면 한국어(지금 문구 그대로 — 원래 리터럴을 그대로 돌려준다), 아니면 영어.
//! 판정 = custom_tier 방식. 게임 중 언어를 바꿀 수 있어 화면이 떠 있는 동안 주기적으로 다시 본다.
//! 전술 104개의 영어 이름·설명 = `lang_en.rs`(생성물 · 원천 tools/i18n/tactics_en.json · tools/gen_lang_en.py).
use crate::tactics;
use mod_api_stable::StableClient;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

#[path = "lang_en.rs"]
mod en;

static KO: AtomicBool = AtomicBool::new(true);
const SAMPLE: &str = "#asset/base/text/ui?main.champion_info.reset_tier";

pub fn ko() -> bool { KO.load(Relaxed) }

/// 게임 언어를 다시 본다. 바뀌었으면 true(화면 문구를 다시 써야 한다). 빈 값이면 한국어(현행 유지).
pub fn refresh(ctx: &StableClient<'_>) -> bool {
    let s = ctx.i18n(SAMPLE).unwrap_or_default();
    let k = s.is_empty() || s.chars().any(|c| ('\u{ac00}'..='\u{d7a3}').contains(&c));
    KO.swap(k, Relaxed) != k
}

/// 화면 고정 문구. 한국어면 받은 리터럴 그대로, 아니면 영어(표에 없으면 리터럴 — 테스트가 막는다).
pub fn ui(ko: &'static str) -> &'static str {
    if self::ko() { return ko; }
    en_ui(ko).unwrap_or(ko)
}

fn en_ui(ko: &str) -> Option<&'static str> {
    Some(match ko {
        "상세전술" => "Detail Tactics",
        "종류별 탭에서 전술을 고르고 풀다운으로 설정합니다. 확인하면 내 팀에 적용합니다. 게임을 저장(저장하기·자동 저장)해야 다음 실행에도 남습니다." =>
            "Pick a tab, then set each tactic from its dropdown. OK applies it to your team. Save the game (manual or autosave) to keep it after restarting.",
        "전술 / 적용 상태" => "Tactic / Status",
        "전부 기본으로" => "Reset All",
        "단계별 숫자는 모드 폴더의 detail_tactics_values.cfg에서 바꾸면 자동 반영됩니다." =>
            "Low/High values can be edited in detail_tactics_values.cfg in the mod folder (applied automatically).",
        "취소" => "Cancel",
        "확인" => "OK",
        "서로 다름 · 다시 선택" => "Mixed - pick again",
        "기본 · 켬" => "Default - On",
        "끔" => "Off",
        "기본" => "Default",
        "낮게" => "Low",
        "높게" => "High",
        "내 팀 적용 준비됨" => "Ready for your team",
        "내 팀 선수 / 세이브 설정을 확인 중입니다. 확인이 끝나면 자동 적용합니다." =>
            "Checking your players and saved settings. Applies automatically when done.",
        " · 이번 버전 미지원" => " - Not supported in this version",
        " · 설치 실패(원본 유지)" => " - Install failed (game default)",
        " · 준비 중" => " - Preparing",
        "원본 유지" => "Game default",
        // 탭(presentation::CATEGORIES 이름 — 데이터는 한국어 그대로 두고 화면에서만 바꾼다)
        "전투·생존" => "Combat & Survival",
        "라인·이동" => "Lane & Movement",
        "정글" => "Jungle",
        "갱·로밍" => "Gank & Roam",
        "푸시·타워" => "Push & Tower",
        "기지 방어" => "Base Defense",
        "오브젝트" => "Objectives",
        "위치·기술" => "Position & Skill",
        // 포지션(tactics::POS_NAMES · 게임 영어 position.*)
        "탑" => "Top",
        "미드" => "Mid",
        "원딜" => "Bottom",
        "서폿" => "Support",
        _ => return None,
    })
}

/// 탭 안내 줄(형식 문구라 따로).
pub fn category_info(name: &'static str, n: usize) -> String {
    if ko() { format!("{} · {}개 전술  |  적용되지 않는 포지션은 빈칸으로 표시합니다.", name, n) }
    else { format!("{} - {} tactics  |  Blank cells = the tactic does not apply to that position.", ui(name), n) }
}

pub fn tactic_name(t: usize) -> &'static str {
    let d = &tactics::TACTICS[t];
    if ko() { d.name } else { en::name(d.id).unwrap_or(d.id) }
}

pub fn tactic_desc(t: usize) -> &'static str {
    let d = &tactics::TACTICS[t];
    if ko() { d.desc } else { en::desc(d.id).unwrap_or("") }
}

#[cfg(test)]
pub(crate) fn set_ko_for_test(k: bool) { KO.store(k, Relaxed); }

#[cfg(test)]
mod tests {
    use super::*;
    fn hangul(s: &str) -> bool { s.chars().any(|c| ('\u{ac00}'..='\u{d7a3}').contains(&c) || ('\u{3131}'..='\u{318e}').contains(&c)) }
    #[test] fn every_tactic_and_ui_string_has_english_without_hangul() {
        for (t, d) in tactics::TACTICS.iter().enumerate() {
            let (n, ds) = (en::name(d.id), en::desc(d.id));
            assert!(n.is_some() && ds.is_some(), "영어 없음: {} ({t})", d.id);
            assert!(!hangul(n.unwrap()) && !hangul(ds.unwrap()) && n.unwrap().is_ascii() && ds.unwrap().is_ascii(), "{}", d.id);
        }
        for c in crate::presentation::CATEGORIES { assert!(en_ui(c.name).is_some(), "탭 {}", c.name); }
        for p in tactics::POS_NAMES { assert!(en_ui(p).is_some(), "포지션 {p}"); }
    }
    #[test] fn korean_mode_returns_original_literals() {
        set_ko_for_test(true);
        for (t, d) in tactics::TACTICS.iter().enumerate() {
            assert!(std::ptr::eq(tactic_name(t), d.name) && std::ptr::eq(tactic_desc(t), d.desc));
        }
        assert_eq!(ui("확인"), "확인");
        assert_eq!(category_info("정글", 4), "정글 · 4개 전술  |  적용되지 않는 포지션은 빈칸으로 표시합니다.");
    }
}
