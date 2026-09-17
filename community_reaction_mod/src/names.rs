//! names — 한국어 표기 변환(원작 표 그대로) + 클라 i18n 으로 채우는 챔피언 표시명 맵.
//! 챔피언: 클라가 `champion_names()`×`i18n("#asset/base/text/champion?description.<id>.name")` 로 `CHAMP_KR` 를 채운다(같은 프로세스라
//!   서버 export 도 이 맵을 본다). 비어 있으면 원작 하드코딩 표 → 그래도 없으면 id 그대로.
use std::collections::HashMap;
use std::sync::Mutex;

pub static CHAMP_KR: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

pub fn champ_kr(id: &str) -> String {
    if let Some(m) = CHAMP_KR.lock().unwrap_or_else(|e| e.into_inner()).as_ref() {
        if let Some(n) = m.get(id) { if !n.is_empty() { return n.clone(); } }
    }
    champ_kr_table(id).to_string()
}

pub fn champ_kr_table(id: &str) -> &str {
    match id {
        "fighter" => "격투가", "knight" => "기사", "swordman" => "검사", "archer" => "궁수", "soldier" => "소총수",
        "priest" => "성직자", "pythoness" => "무녀", "monk" => "몽크", "pyromancer" => "화염술사", "ice_mage" => "얼음술사",
        "ninja" => "닌자", "magic_knight" => "마검사", "berserker" => "광전사", "executioner" => "처형인", "lancer" => "창술사",
        "ogre" => "오우거", "dual_blader" => "듀얼블레이더", "cavalry_knight" => "기병", "gunner" => "총잡이", "pole_warrior" => "봉술사",
        "jiangshi" => "강시", "gambler" => "도박사", "hammerer" => "중보병", "demon" => "악마", "vampire" => "흡혈귀",
        "spirit_caller" => "정령사", "boomerang_hunter" => "부메랑헌터", "inquisitor" => "이단심문관", "shield_bearer" => "방패병",
        "whip_master" => "채찍술사", "werewolf" => "늑대인간", "dokkaebi" => "도깨비", "necromancer" => "네크로맨서", "bard" => "음유시인",
        "barrier_magician" => "결계술사", "chef" => "요리사", "clown" => "광대", "dancer" => "무희", "dark_mage" => "흑마술사",
        "exorcist" => "엑소시스트", "ghost" => "유령", "illusionist" => "환영술사", "lightning_mage" => "번개술사", "plague_doctor" => "역병의사",
        "poison_dart_hunter" => "독침술사", "shadowmancer" => "그림자술사", "taoist" => "도사", "siege_breaker" => "공성병", "android" => "안드로이드",
        "druid" => "드루이드", "prisoner" => "죄수", "bomber" => "폭탄병", "voodoo_shaman" => "부두술사", "white_mage" => "백마술사",
        "wind_mage" => "바람술사", "enchanter" => "인챈터", "hitman" => "히트맨", "guardian_spirit" => "수호령", "hunter" => "사냥꾼",
        "circus_blade" => "곡예사",
        _ => id,
    }
}

pub fn league_kr(name: &str) -> &str {
    match name {
        "TACK" => "한국 1부 (TACK)", "TACK2" => "한국 2부 (TACK2)", "TACJ" => "일본 1부 (TACJ)", "TACJ2" => "일본 2부 (TACJ2)",
        "TACC" => "중국 1부 (TACC)", "TACC2" => "중국 2부 (TACC2)", "TACE" => "유럽 1부 (TACE)", "TACE2" => "유럽 2부 (TACE2)",
        "TACA" => "북미 1부 (TACA)", "TACA2" => "북미 2부 (TACA2)", "TACS" => "남미 1부 (TACS)", "TACS2" => "남미 2부 (TACS2)",
        _ => name,
    }
}

pub fn tournament_kr(name: &str) -> String {
    match name {
        "Challengers E" => "챌린저스 이스턴".to_string(), "Challengers W" => "챌린저스 웨스턴".to_string(),
        "Masters" => "마스터스".to_string(), "Champions" => "챔피언스".to_string(),
        other => other.to_string(),
    }
}

/// Team.fan_expectation (0.6.0 enum: Bottom/Lower/Mid/Upper/Top — 원작 Low/High 판정 유지 + Top/Bottom 추가)
pub fn expectation_kr(exp: &str) -> &'static str {
    if exp.contains("Top") { "매우 높음" }
    else if exp.contains("Bottom") { "매우 낮음" }
    else if exp.contains("Lower") || exp.contains("Low") { "낮음" }
    else if exp.contains("Higher") || exp.contains("High") || exp.contains("Upper") { "높음" }
    else { "보통" }
}

/// Team.fan_satisfaction (0.6.0 enum: VeryDissatisfied/Dissatisfied/Normal/Satisfied/VerySatisfied)
pub fn satisfaction_kr(sat: &str) -> &'static str {
    if sat.contains("VerySatisfied") { "매우 만족" }
    else if sat.contains("VeryUnsatisfied") || sat.contains("VeryDissatisfied") { "매우 불만족" }
    else if sat.contains("Unsatisfied") || sat.contains("Dissatisfied") { "불만족" }
    else if sat.contains("Satisfied") { "만족" }
    else { "보통" }
}

pub fn num_rating_kr(val: i64) -> &'static str {
    match val { i64::MIN..=20 => "매우 낮음(불만)", 21..=40 => "낮음(불만족)", 41..=60 => "보통", 61..=80 => "높음(만족)", _ => "매우 높음(만족)" }
}

/// 아이템 id(0~29) → "스탯[T{n}]" (원작). 문자열 키면 그대로.
pub fn item_kr(id: i64) -> String {
    let category_idx = id / 5;
    let tier_num = (id % 5) + 1;
    let stat_name = match category_idx { 0 => "공격력", 1 => "공속", 2 => "방어력", 3 => "마저", 4 => "주문력", 5 => "체력", _ => "기타" };
    format!("{}[T{}]", stat_name, tier_num)
}

pub fn position_index(pos: &str) -> usize {
    match pos { "Top" => 0, "Jungle" => 1, "Mid" => 2, "Bottom" => 3, "Support" => 4, _ => 0 }
}
