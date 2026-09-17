//! export — 서버측 요약 텍스트 조립 → `mods\community_reaction_mod\latest_match.js` (원작 `export_match_reactions` 의 stable 판).
//! 원작은 클라 `ClientDatabase`(match_replays/matches/league_competitions/…) 를 직접 읽었지만 stable 클라 API 는 Team/Athlete/Staff 만 노출
//! → 서버 `handle_command("crm_export")` 에서 `record_get_json` 으로 전부 읽는다. 로컬 서버 = 같은 프로세스라 하이라이트 static 도 그대로 본다.
//! 출력 형식은 원작·EXPORT_FORMAT.md 와 동일(갤러리 파서 호환). 차이:
//!   · 챔피언 통계: `champion_patch_statistics` 미노출 → 진행 중(finalized=false) 대회의 statistics.champion_detail(픽/승) + 리플레이 밴 집계로 대체
//!   · 형식 라벨: GamePlayOption(league_rule 등) 미노출 → "설정 미확인 [raw league_type=…/ty=…]"
//!   · 상대 전적: 클라 전용 API(`head_to_head`) → 클라가 payload 로 전달(opp/w/l)
//! 캐시: Match 는 종료(End) 레코드 영구·미종료는 감시 집합만 재독, MatchReplay 는 영구, 대회는 finalized 만 영구.
use mod_api_stable::{RecordKindV1, StableServerCtx};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;

use crate::hl;
use crate::names::*;

// ───────── JSON 헬퍼 ─────────
fn ji(v: &Value, k: &str) -> i64 { v.get(k).and_then(|x| x.as_i64().or_else(|| x.as_f64().map(|f| f as i64))).unwrap_or(0) }
fn jf(v: &Value, k: &str) -> f64 { v.get(k).and_then(|x| x.as_f64()).unwrap_or(0.0) }
fn js(v: &Value, k: &str) -> String { v.get(k).map(|x| match x { Value::String(s) => s.clone(), other => other.to_string() }).unwrap_or_default() }
fn jb(v: &Value, k: &str) -> bool { v.get(k).and_then(|x| x.as_bool()).unwrap_or(false) }
fn jids(v: &Value, k: &str) -> Vec<usize> { v.get(k).and_then(|x| x.as_array()).map(|a| a.iter().filter_map(|x| x.as_u64().map(|n| n as usize)).collect()).unwrap_or_default() }
fn jstrs(v: &Value, k: &str) -> Vec<String> { v.get(k).and_then(|x| x.as_array()).map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect()).unwrap_or_default() }
fn jarr_i(v: &Value, k: &str) -> Vec<i64> { v.get(k).and_then(|x| x.as_array()).map(|a| a.iter().filter_map(|x| x.as_i64()).collect()).unwrap_or_default() }
/// 날짜 값 → "YYYY-MM-DD HH:MM:SS"(문자열 "2026-02-04T09:00:00" 의 T 만 공백) / 객체 {year,month,day,…} 도 처리
fn date_str(v: &Value) -> String {
    match v {
        Value::String(s) => s.replace('T', " "),
        Value::Object(o) => {
            let g = |k: &str| o.get(k).and_then(|x| x.as_i64()).unwrap_or(0);
            if o.contains_key("year") { format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", g("year"), g("month"), g("day"), g("hour"), g("minute"), g("second")) } else { "알 수 없음".into() }
        }
        Value::Null => "알 수 없음".into(),
        other => other.to_string(),
    }
}
fn day_of(s: &str) -> &str { if s.len() >= 10 { &s[..10] } else { s } }
/// enum 값 → (variant, 숫자들): "Wait" → ("Wait",[]) / {"Normal":5} → ("Normal",[5]) / {"CompetitionRank":[3,1]} → ("CompetitionRank",[3,1])
fn enum_of(v: &Value) -> (String, Vec<i64>) {
    match v {
        Value::String(s) => (s.clone(), vec![]),
        Value::Object(o) => {
            if let Some((k, inner)) = o.iter().next() {
                let nums = match inner {
                    Value::Number(n) => n.as_i64().into_iter().collect(),
                    Value::Array(a) => a.iter().filter_map(|x| x.as_i64()).collect(),
                    Value::Object(io) => io.values().filter_map(|x| x.as_i64()).collect(),
                    _ => vec![],
                };
                (k.clone(), nums)
            } else { ("?".into(), vec![]) }
        }
        _ => ("?".into(), vec![]),
    }
}
fn find_key<'a>(v: &'a Value, key: &str) -> Option<&'a Value> {
    match v {
        Value::Object(o) => { if let Some(x) = o.get(key) { return Some(x); } o.values().find_map(|x| find_key(x, key)) }
        Value::Array(a) => a.iter().find_map(|x| find_key(x, key)),
        _ => None,
    }
}

// ───────── 캐시 ─────────
#[derive(Clone)]
struct MatchRec { id: usize, kind: RecordKindV1, need_win: i64, team1: Value, team2: Value, state: Value, replays: Vec<usize>, is_practice: bool, date: String, ended: bool }
#[derive(Clone)]
struct RepAth { athlete_id: usize, position: String, champion: String, items: Vec<Value>, statistics: Value }
#[derive(Clone)]
struct Rep { id: usize, blue_team_id: usize, red_team_id: usize, blue_ban: Vec<String>, red_ban: Vec<String>, blue_team: Vec<RepAth>, red_team: Vec<RepAth>, blue_win: bool, version: String, blue_perf: Value, red_perf: Value }
#[derive(Clone)]
struct CompRec { id: usize, is_tournament: bool, finalized: bool, matches: Vec<usize>, playoffs: Vec<usize>, promotion: Vec<usize>, promotion_finalized: bool, group_matches: Vec<usize>, tournament_matches: Vec<usize>, standings: Value, groups: Vec<Value>, champ_stats: HashMap<String, (i64, i64)>, format_raw: String }

static MATCHES: Mutex<Option<HashMap<usize, MatchRec>>> = Mutex::new(None);
static REPLAYS: Mutex<Option<HashMap<usize, Rep>>> = Mutex::new(None);
static COMPS_FINAL: Mutex<Option<HashMap<(bool, usize), CompRec>>> = Mutex::new(None);
static LAST_SEQ: AtomicU64 = AtomicU64::new(0);
static DIRTY: AtomicBool = AtomicBool::new(true);
static EXPORT_N: AtomicU64 = AtomicU64::new(0);

fn parse_match(id: usize, kind: RecordKindV1, v: &Value, practice_ids: &HashSet<usize>) -> MatchRec {
    let state = v.get("running_state").cloned().unwrap_or(Value::Null);
    let (sk, _) = enum_of(&state);
    MatchRec {
        id, kind, need_win: ji(v, "need_win").max(1),
        team1: v.get("team1").cloned().unwrap_or(Value::Null), team2: v.get("team2").cloned().unwrap_or(Value::Null),
        ended: sk == "End", state,
        replays: jids(v, "replays"),
        is_practice: jb(v, "is_practice") || jb(v, "is_room_practice") || practice_ids.contains(&id) || kind == RecordKindV1::MatchPractice,
        date: date_str(v.get("date").unwrap_or(&Value::Null)),
    }
}
fn parse_rep(v: &Value) -> Rep {
    let ath = |k: &str| -> Vec<RepAth> {
        v.get(k).and_then(|x| x.as_array()).map(|a| a.iter().map(|p| RepAth {
            athlete_id: ji(p, "athlete_id") as usize,
            position: enum_of(p.get("position").unwrap_or(&Value::Null)).0,
            champion: js(p, "champion"),
            items: p.get("items").and_then(|x| x.as_array()).cloned().unwrap_or_default(),
            statistics: p.get("statistics").cloned().unwrap_or(Value::Null),
        }).collect()).unwrap_or_default()
    };
    Rep {
        id: ji(v, "id") as usize, blue_team_id: ji(v, "blue_team_id") as usize, red_team_id: ji(v, "red_team_id") as usize,
        blue_ban: jstrs(v, "blue_ban"), red_ban: jstrs(v, "red_ban"), blue_team: ath("blue_team"), red_team: ath("red_team"),
        blue_win: jb(v, "blue_team_win"), version: js(v, "version"),
        blue_perf: v.get("blue_performance").cloned().unwrap_or(Value::Null), red_perf: v.get("red_performance").cloned().unwrap_or(Value::Null),
    }
}
fn parse_comp(id: usize, is_tournament: bool, v: &Value) -> CompRec {
    let mut champ_stats: HashMap<String, (i64, i64)> = HashMap::new();
    if let Some(stats) = v.get("statistics").and_then(|x| x.as_object()) {
        for (_aid, a) in stats {
            if let Some(cd) = a.get("champion_detail").and_then(|x| x.as_object()) {
                for (champ, d) in cd { let e = champ_stats.entry(champ.clone()).or_insert((0, 0)); e.0 += ji(d, "matches"); e.1 += ji(d, "wins"); }
            }
        }
    }
    CompRec {
        id, is_tournament, finalized: jb(v, "finalized"),
        matches: jids(v, "matches"), playoffs: jids(v, "playoffs"), promotion: jids(v, "promotion_series"), promotion_finalized: jb(v, "promotion_series_finalized"),
        group_matches: jids(v, "group_matches"), tournament_matches: jids(v, "tournament_matches"),
        standings: v.get("standings").cloned().unwrap_or(Value::Null),
        groups: v.get("groups").and_then(|x| x.as_array()).cloned().unwrap_or_default(),
        champ_stats,
        format_raw: if is_tournament { format!("ty={}", js(v, "ty")) } else { format!("league_type={}", js(v, "league_type")) },
    }
}

fn get_rep(ctx: &StableServerCtx<'_>, id: usize) -> Option<Rep> {
    { let g = REPLAYS.lock().unwrap_or_else(|e| e.into_inner()); if let Some(m) = g.as_ref() { if let Some(r) = m.get(&id) { return Some(r.clone()); } } }
    let j = ctx.record_get_json(RecordKindV1::MatchReplay, id, "")?;
    let v: Value = serde_json::from_str(&j).ok()?;
    let r = parse_rep(&v);
    let mut g = REPLAYS.lock().unwrap_or_else(|e| e.into_inner());
    g.get_or_insert_with(HashMap::new).insert(id, r.clone());
    Some(r)
}

/// 대회 전부 로드(finalized 는 캐시). 반환 = (bool is_tournament, id) → CompRec
fn load_comps(ctx: &StableServerCtx<'_>) -> Vec<CompRec> {
    let mut out = Vec::new();
    let mut cache = COMPS_FINAL.lock().unwrap_or_else(|e| e.into_inner());
    let cache = cache.get_or_insert_with(HashMap::new);
    for (kind, is_t) in [(RecordKindV1::LeagueCompetition, false), (RecordKindV1::TournamentCompetition, true)] {
        for id in ctx.record_ids(kind) {
            if let Some(c) = cache.get(&(is_t, id)) { out.push(c.clone()); continue; }
            let Some(j) = ctx.record_get_json(kind, id, "") else { continue };
            let Ok(v) = serde_json::from_str::<Value>(&j) else { continue };
            let c = parse_comp(id, is_t, &v);
            if c.finalized { cache.insert((is_t, id), c.clone()); }
            out.push(c);
        }
    }
    out
}

/// Match 캐시 갱신. dirty(첫 실행·MatchFinished/SeasonRollover 이벤트) 면 미종료 전부 재독, 아니면 감시 집합만.
fn refresh_matches(ctx: &StableServerCtx<'_>, our_team: usize, watch: &HashSet<usize>) -> HashMap<usize, MatchRec> {
    // 관리 이벤트로 dirty 판정
    let seq0 = LAST_SEQ.load(Ordering::Relaxed);
    let evs = ctx.management_events_after(seq0);
    let mut max_seq = seq0;
    for e in &evs { if e.seq > max_seq { max_seq = e.seq; } if e.kind.map(|k| k as u32 != 1).unwrap_or(true) { DIRTY.store(true, Ordering::Relaxed); } }
    LAST_SEQ.store(max_seq, Ordering::Relaxed);
    let dirty = DIRTY.swap(false, Ordering::Relaxed);

    let practice_ids: HashSet<usize> = ctx.record_ids(RecordKindV1::MatchPractice).into_iter().collect();
    let mut ids: Vec<(usize, RecordKindV1)> = Vec::new();
    let mut seen: HashSet<usize> = HashSet::new();
    for kind in [RecordKindV1::Match, RecordKindV1::MatchNormal, RecordKindV1::MatchPractice] {
        for id in ctx.record_ids(kind) { if seen.insert(id) { ids.push((id, kind)); } }
    }
    let mut g = MATCHES.lock().unwrap_or_else(|e| e.into_inner());
    let cache = g.get_or_insert_with(HashMap::new);
    let mut reads = 0usize;
    for (id, kind) in ids {
        let need = match cache.get(&id) {
            None => true,
            Some(m) if m.ended => false,
            Some(m) => dirty || watch.contains(&id) || slot_team(&m.team1) == Some(our_team) || slot_team(&m.team2) == Some(our_team),
        };
        if !need { continue; }
        let kind = cache.get(&id).map(|m| m.kind).unwrap_or(kind);
        let mut j = ctx.record_get_json(kind, id, "");
        let mut k2 = kind;
        if j.is_none() { for alt in [RecordKindV1::Match, RecordKindV1::MatchNormal, RecordKindV1::MatchPractice] { if alt == kind { continue; } j = ctx.record_get_json(alt, id, ""); if j.is_some() { k2 = alt; break; } } }
        let Some(j) = j else { continue };
        let Ok(v) = serde_json::from_str::<Value>(&j) else { continue };
        reads += 1;
        cache.insert(id, parse_match(id, k2, &v, &practice_ids));
    }
    if reads > 0 { crate::log(&format!("[export] matches read={} cached={} dirty={} watch={}", reads, cache.len(), dirty, watch.len())); }
    cache.clone()
}
fn slot_team(v: &Value) -> Option<usize> { let (k, n) = enum_of(v); if k == "Normal" { n.first().map(|x| *x as usize) } else { None } }

// ───────── 팀/선수 정보(한 export 내 캐시) ─────────
#[derive(Clone, Default)]
struct TeamInfo { name: String, manager: String, expectation: String, satisfaction: String, fan_count: i64, league_id: Option<usize> }
#[derive(Clone, Default)]
struct AthInfo { name: String, fan_sat: i64, fan_exp: i64, fan_count: i64, weekly_salary: f64 }
struct Db<'a> { ctx: &'a StableServerCtx<'a>, teams: std::cell::RefCell<HashMap<usize, TeamInfo>>, aths: std::cell::RefCell<HashMap<usize, AthInfo>>, leagues: std::cell::RefCell<Option<HashMap<usize, (String, i64, Vec<usize>)>>>, tours: std::cell::RefCell<Option<Vec<(usize, String, Vec<usize>)>>> }
impl<'a> Db<'a> {
    fn team(&self, id: usize) -> TeamInfo {
        if let Some(t) = self.teams.borrow().get(&id) { return t.clone(); }
        let c = self.ctx;
        let t = TeamInfo {
            name: c.team_get_string(id, "name").unwrap_or_else(|| format!("팀{}", id)),
            manager: c.team_get_string(id, "manager_name").unwrap_or_default(),
            expectation: c.team_get_json(id, "fan_expectation").unwrap_or_default(),
            satisfaction: c.team_get_json(id, "fan_satisfaction").unwrap_or_default(),
            fan_count: c.team_get_i64(id, "fan_count").unwrap_or(0),
            league_id: c.team_get_i64(id, "league_id").map(|x| x as usize),
        };
        self.teams.borrow_mut().insert(id, t.clone());
        t
    }
    fn team_name(&self, id: usize) -> String { self.team(id).name }
    fn ath(&self, id: usize) -> Option<AthInfo> {
        if let Some(a) = self.aths.borrow().get(&id) { return Some(a.clone()); }
        let c = self.ctx;
        let name = c.athlete_get_string(id, "name")?;
        let mut a = AthInfo { name, ..Default::default() };
        if let Some(m) = c.athlete_get_json(id, "management").and_then(|j| serde_json::from_str::<Value>(&j).ok()) {
            a.fan_sat = ji(&m, "fan_satisfaction"); a.fan_exp = ji(&m, "fan_expectation"); a.fan_count = ji(&m, "fan_count");
        }
        if let Some(cv) = c.athlete_get_json(id, "contract").and_then(|j| serde_json::from_str::<Value>(&j).ok()) {
            a.weekly_salary = find_key(&cv, "weekly_salary").and_then(|x| x.as_f64()).unwrap_or(0.0);
        }
        self.aths.borrow_mut().insert(id, a.clone());
        Some(a)
    }
    /// league id → (name, division, competitions)
    fn leagues(&self) -> HashMap<usize, (String, i64, Vec<usize>)> {
        if let Some(l) = self.leagues.borrow().as_ref() { return l.clone(); }
        let mut m = HashMap::new();
        for id in self.ctx.record_ids(RecordKindV1::League) {
            if let Some(v) = self.ctx.record_get_json(RecordKindV1::League, id, "").and_then(|j| serde_json::from_str::<Value>(&j).ok()) {
                m.insert(id, (js(&v, "name"), ji(&v, "division"), jids(&v, "competitions")));
            }
        }
        *self.leagues.borrow_mut() = Some(m.clone());
        m
    }
    fn tournaments(&self) -> Vec<(usize, String, Vec<usize>)> {
        if let Some(l) = self.tours.borrow().as_ref() { return l.clone(); }
        let mut out = Vec::new();
        for id in self.ctx.record_ids(RecordKindV1::Tournament) {
            if let Some(v) = self.ctx.record_get_json(RecordKindV1::Tournament, id, "").and_then(|j| serde_json::from_str::<Value>(&j).ok()) {
                out.push((id, js(&v, "name"), jids(&v, "competitions")));
            }
        }
        *self.tours.borrow_mut() = Some(out.clone());
        out
    }
}

fn salary_fmt(weekly: f64) -> String {
    let annual = (weekly * 52.0) as u64;
    if annual >= 100_000_000 { format!("{:.2}억원", annual as f64 / 100_000_000.0) } else { format!("{}만원", annual / 10_000) }
}
fn items_str(items: &[Value]) -> String {
    items.iter().map(|x| match x { Value::Number(n) => item_kr(n.as_i64().unwrap_or(0)), Value::String(s) => s.clone(), o => o.to_string() }).collect::<Vec<_>>().join(", ")
}

// ───────── 세트 요약(원작 build_single_match_summary) ─────────
fn set_summary(db: &Db<'_>, r: &Rep, is_main: bool, set_num: usize, attention: usize) -> String {
    let mut t = String::new();
    let bt = db.team(r.blue_team_id);
    let rt = db.team(r.red_team_id);
    let (bn, rn) = (bt.name.clone(), rt.name.clone());
    t.push_str("==================================================\n");
    if is_main { t.push_str(&format!("메인 매치 [{}세트] (우리 팀 경기 분석) [주목도: {}/20]\n", set_num, attention)); }
    else { t.push_str(&format!("동일 라운드 다른 경기 [{}세트] 분석 [주목도: {}/20]\n", set_num, attention)); }
    t.push_str("==================================================\n");
    t.push_str(&format!("대진: {} vs {}\n\n", bn, rn));
    for (label, ti) in [("블루", &bt), ("레드", &rt)] {
        t.push_str(&format!("[{} 팀: {} 정보]\n", label, ti.name));
        t.push_str(&format!(" - 감독: {}\n", ti.manager));
        t.push_str(&format!(" - 팬 기대치: {}\n", expectation_kr(&ti.expectation)));
        t.push_str(&format!(" - 팬 만족도: {}\n", satisfaction_kr(&ti.satisfaction)));
        t.push_str(&format!(" - 팬 규모: {}명\n\n", ti.fan_count));
    }
    for (name, roster) in [(&bn, &r.blue_team), (&rn, &r.red_team)] {
        t.push_str(&format!("* [{}] 선수단 프로필\n", name));
        for p in roster {
            let Some(a) = db.ath(p.athlete_id) else { continue };
            t.push_str(&format!(" - {} (ID: {}) | 팬 만족도: {}, 기대치: {}, 연봉: {} | 팬 수: {}명\n",
                a.name, p.athlete_id, num_rating_kr(a.fan_sat), num_rating_kr(a.fan_exp), salary_fmt(a.weekly_salary), a.fan_count));
        }
        t.push_str("\n");
    }
    let winner = if r.blue_win { &bn } else { &rn };
    t.push_str("--- 세부 성적 및 밴픽 ---\n");
    t.push_str(&format!("승자(Winner): {}\n", winner));
    let bb: Vec<String> = r.blue_ban.iter().map(|b| champ_kr(b)).collect();
    let rb: Vec<String> = r.red_ban.iter().map(|b| champ_kr(b)).collect();
    t.push_str(&format!("블루 밴카드: [{}]\n", bb.join(", ")));
    t.push_str(&format!("레드 밴카드: [{}]\n\n", rb.join(", ")));
    let (bg, rg) = (ji(&r.blue_perf, "total_gold"), ji(&r.red_perf, "total_gold"));
    let leader = if bg >= rg { &bn } else { &rn };
    t.push_str("--- 팀 종합 지표 및 오브젝트 ---\n");
    t.push_str(&format!("골드 현황: 블루 {}G vs 레드 {}G ({} 골드 리드: {}G 차이)\n", bg, rg, leader, (bg - rg).abs()));
    for (label, pf) in [("블루", &r.blue_perf), ("레드", &r.red_perf)] {
        t.push_str(&format!("{}팀 오브젝트: 모르가드 {}회 ({}), 세르펜 {}회 ({})\n", label, ji(pf, "epic_secured"), if jb(pf, "first_epic") { "선취" } else { "-" }, ji(pf, "serpen_secured"), if jb(pf, "first_serpen") { "선취" } else { "-" }));
    }
    t.push_str("\n");
    let mut best = -999.0f32;
    let (mut pog_champ, mut pog_name, mut pog_kda, mut pog_deal) = (String::from("Unknown"), String::from("Unknown"), String::new(), 0i64);
    for (name, roster, pf) in [(&bn, &r.blue_team, &r.blue_perf), (&rn, &r.red_team, &r.red_perf)] {
        t.push_str(&format!("<{}> 선수별 인게임 데이터\n", name));
        let (pk, pd, pdl) = (jarr_i(pf, "kills"), jarr_i(pf, "deaths"), jarr_i(pf, "deal"));
        for p in roster {
            let idx = position_index(&p.position);
            let st = &p.statistics;
            let kills = pk.get(idx).copied().unwrap_or_else(|| ji(st, "kills"));
            let deaths = pd.get(idx).copied().unwrap_or_else(|| ji(st, "deaths"));
            let assists = ji(st, "assists");
            let deal = pdl.get(idx).copied().unwrap_or_else(|| ji(st, "dealing"));
            let (gold, tanking, healing) = (ji(st, "gold"), ji(st, "tanking"), ji(st, "healing"));
            let kor = champ_kr(&p.champion);
            let pname = db.ath(p.athlete_id).map(|a| a.name).unwrap_or_else(|| "Unknown".into());
            let df = if deaths == 0 { 0.7 } else { deaths as f32 };
            let score = (kills as f32 + assists as f32) / df;
            if score > best { best = score; pog_champ = kor.clone(); pog_name = pname.clone(); pog_kda = format!("{}/{}/{}", kills, deaths, assists); pog_deal = deal; }
            t.push_str(&format!(" - [{}] {} ({}) | K/D/A: {}/{}/{} | 골드: {}G | 딜량: {} | 받은피해: {} | 힐량: {} | 장착 템: [{}]\n",
                p.position, pname, kor, kills, deaths, assists, gold, deal, tanking, healing, items_str(&p.items)));
        }
        t.push_str("\n");
    }
    t.push_str("--- 오늘의 세트 POG (MVP) ---\n");
    t.push_str(&format!("선정 선수: {} ({}) | 세트 K/D/A: {} (평점: {:.2}) | 딜량: {}\n\n", pog_name, pog_champ, pog_kda, best, pog_deal));
    t
}

fn escape_json(s: &str) -> String { s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n").replace('\r', "") }

struct ChampTier { id: String, bans: i64, matches: i64, wins: i64, win_rate: f32, presence: f32, tier: &'static str }
fn weighted_quantile(sorted: &[ChampTier], q: f32, total: f32) -> f32 {
    if sorted.is_empty() { return 0.5; }
    let target = q * total; let mut cum = 0.0;
    for c in sorted { cum += c.presence; if cum >= target { return c.win_rate; } }
    sorted.last().map(|c| c.win_rate).unwrap_or(0.5)
}

pub struct Req { pub our_team: usize, pub today: String, pub opp: Option<(usize, usize, usize)> }

/// export 본체. Ok((opponent id, 요약 메모)) / Err(사유). 파일은 `<mod_dir>\latest_match.js`.
pub fn run(ctx: &StableServerCtx<'_>, req: &Req) -> Result<(Option<usize>, String), String> {
    let n = EXPORT_N.fetch_add(1, Ordering::Relaxed);
    hl::flush_idle();
    let db = Db { ctx, teams: Default::default(), aths: Default::default(), leagues: Default::default(), tours: Default::default() };
    let our = req.our_team;
    let comps = load_comps(ctx);
    // 감시 집합: 진행 중 대회의 플옵/승강/녹아웃/조별 매치 (슬롯·상태가 바뀌는 곳)
    let mut watch: HashSet<usize> = HashSet::new();
    for c in comps.iter().filter(|c| !c.finalized) { for l in [&c.playoffs, &c.promotion, &c.tournament_matches, &c.group_matches] { watch.extend(l.iter().copied()); } }
    let matches = refresh_matches(ctx, our, &watch);

    // 우리 팀 최신 리플레이: 우리 팀이 슬롯에 있는 매치의 리플레이들 중 max id (팀 대조 재확인)
    let mut cand: Vec<usize> = Vec::new();
    for m in matches.values() { if slot_team(&m.team1) == Some(our) || slot_team(&m.team2) == Some(our) { cand.extend(m.replays.iter().copied()); } }
    cand.sort_unstable(); cand.dedup();
    let mut latest: Option<Rep> = None;
    for rid in cand.iter().rev() {
        if let Some(r) = get_rep(ctx, *rid) { if r.blue_team_id == our || r.red_team_id == our { latest = Some(r); break; } }
    }
    let Some(latest) = latest else { return Err(format!("우리 팀 리플레이 없음 (matches={} cand={})", matches.len(), cand.len())); };
    let opp_id = if latest.blue_team_id == our { latest.red_team_id } else { latest.blue_team_id };
    let our_team = db.team(our);
    let leagues = db.leagues();
    let league_name = our_team.league_id.and_then(|l| leagues.get(&l)).map(|l| league_kr(&l.0).to_string()).unwrap_or_else(|| "알 수 없는 리그".into());

    // 우리 매치 = 최신 리플레이를 품은 매치 / 폴백 = Running 정규 매치(우리 팀 슬롯)
    let mut our_match: Option<MatchRec> = matches.values().find(|m| m.replays.contains(&latest.id)).cloned();
    if our_match.is_none() {
        our_match = matches.values().find(|m| !m.is_practice && enum_of(&m.state).0 == "Running" && (slot_team(&m.team1) == Some(our) || slot_team(&m.team2) == Some(our))).cloned();
    }
    let need_win = our_match.as_ref().map(|m| m.need_win).unwrap_or(2);
    let match_date = our_match.as_ref().map(|m| m.date.clone()).unwrap_or_else(|| "알 수 없음".into());
    let our_match_id = our_match.as_ref().map(|m| m.id);
    let our_is_practice = our_match.as_ref().map(|m| m.is_practice).unwrap_or(false);

    // 시리즈: 매치의 replays(있으면) / 없으면 원작 규칙(id ±5 & 같은 팀 쌍)
    let mut our_series: Vec<Rep> = Vec::new();
    if let Some(m) = &our_match { for rid in &m.replays { if let Some(r) = get_rep(ctx, *rid) { our_series.push(r); } } }
    if our_series.is_empty() {
        for rid in cand.iter() {
            if (*rid as i64 - latest.id as i64).abs() > 5 { continue; }
            if let Some(r) = get_rep(ctx, *rid) {
                let same = (r.blue_team_id == latest.blue_team_id && r.red_team_id == latest.red_team_id) || (r.blue_team_id == latest.red_team_id && r.red_team_id == latest.blue_team_id);
                if same { our_series.push(r); }
            }
        }
    }
    our_series.sort_by_key(|r| r.id);
    our_series.dedup_by_key(|r| r.id);

    // 주목도: 같은 날 같은 리그 다른 매치와 팬 합 비교
    let fans_of = |r: &Rep| -> i64 {
        let mut f = db.team(r.blue_team_id).fan_count + db.team(r.red_team_id).fan_count;
        for p in r.blue_team.iter().chain(r.red_team.iter()) { if let Some(a) = db.ath(p.athlete_id) { f += a.fan_count; } }
        f
    };
    let our_fans = fans_of(&latest);
    let (mut our_attention, mut other_attention) = (20usize, 0usize);
    let mut other_series: Vec<Rep> = Vec::new();
    if let Some(m) = &our_match {
        let our_day = day_of(&m.date).to_string();
        let mut others: Vec<&MatchRec> = matches.values().filter(|o| o.id != m.id && day_of(&o.date) == our_day && !o.is_practice).collect();
        others.sort_by_key(|o| o.id);
        let other = others.into_iter().find(|o| {
            [slot_team(&o.team1), slot_team(&o.team2)].iter().flatten().any(|tid| db.team(*tid).league_id == our_team.league_id && our_team.league_id.is_some())
        });
        if let Some(o) = other {
            for rid in &o.replays { if let Some(r) = get_rep(ctx, *rid) { other_series.push(r); } }
            other_series.sort_by_key(|r| r.id);
            if let Some(ol) = other_series.last() {
                let grand = our_fans + fans_of(ol);
                if grand > 0 { our_attention = (((our_fans as f32 / grand as f32) * 20.0).ceil() as usize).clamp(1, 20); other_attention = 20 - our_attention; }
            }
        }
    }

    let mut text = String::new();
    // ── 챔피언 통계 및 티어 (진행 중 대회 statistics + 리플레이 밴) ──
    text.push_str("==================================================\n챔피언 통계 및 티어\n==================================================\n");
    {
        let live: Vec<&CompRec> = comps.iter().filter(|c| !c.finalized && !c.champ_stats.is_empty()).collect();
        let mut agg: HashMap<String, (i64, i64, i64)> = HashMap::new(); // (matches, wins, bans)
        let mut sets = 0usize;
        let mut mids: Vec<usize> = Vec::new();
        for c in &live {
            for (k, (m, w)) in &c.champ_stats { let e = agg.entry(k.clone()).or_insert((0, 0, 0)); e.0 += m; e.1 += w; }
            for l in [&c.matches, &c.playoffs, &c.promotion, &c.group_matches, &c.tournament_matches] { mids.extend(l.iter().copied()); }
        }
        mids.sort_unstable(); mids.dedup();
        for mid in mids {
            let Some(m) = matches.get(&mid) else { continue };
            for rid in &m.replays {
                let Some(r) = get_rep(ctx, *rid) else { continue };
                sets += 1;
                for b in r.blue_ban.iter().chain(r.red_ban.iter()) { agg.entry(b.clone()).or_insert((0, 0, 0)).2 += 1; }
            }
        }
        if agg.is_empty() {
            text.push_str("패치 통계 데이터를 발견하지 못했습니다.\n");
        } else {
            let n_ch = agg.len();
            let presence_threshold = 0.40 * (30.0 / n_ch.max(1) as f32);
            let max_s = 5 + n_ch.saturating_sub(30) / 15;
            let total_f = sets.max(1) as f32;
            let mut list: Vec<ChampTier> = agg.into_iter().map(|(id, (m, w, b))| ChampTier { id, bans: b, matches: m, wins: w, win_rate: if m > 0 { w as f32 / m as f32 } else { 0.0 }, presence: (m + b) as f32 / total_f, tier: "" }).collect();
            let mut s_cand: Vec<ChampTier> = Vec::new(); let mut others: Vec<ChampTier> = Vec::new();
            for c in list.drain(..) { if c.win_rate > 0.55 && c.presence >= presence_threshold { s_cand.push(c); } else { others.push(c); } }
            s_cand.sort_by(|a, b| b.win_rate.partial_cmp(&a.win_rate).unwrap_or(std::cmp::Ordering::Equal));
            let mut s_tier: Vec<ChampTier> = Vec::new();
            for c in s_cand { if s_tier.len() < max_s { s_tier.push(c); } else { others.push(c); } }
            for c in s_tier.iter_mut() { c.tier = "S"; }
            let mut non_s = others;
            non_s.sort_by(|a, b| a.win_rate.partial_cmp(&b.win_rate).unwrap_or(std::cmp::Ordering::Equal));
            let tw: f32 = non_s.iter().map(|c| c.presence).sum();
            let (q20, q45, q70, q75, q80, q85) = (weighted_quantile(&non_s, 0.20, tw), weighted_quantile(&non_s, 0.45, tw), weighted_quantile(&non_s, 0.70, tw), weighted_quantile(&non_s, 0.75, tw), weighted_quantile(&non_s, 0.80, tw), weighted_quantile(&non_s, 0.85, tw));
            let ab = q75.clamp(q70.max(q80 - 0.03), q85.max(q70.max(q80 - 0.03)));
            for c in non_s.iter_mut() { c.tier = if c.win_rate < q20 { "D" } else if c.win_rate < q45 { "C" } else if c.win_rate < ab { "B" } else { "A" }; }
            let mut all = s_tier; all.extend(non_s);
            let tv = |t: &str| match t { "S" => 0, "A" => 1, "B" => 2, "C" => 3, _ => 4 };
            all.sort_by(|a, b| tv(a.tier).cmp(&tv(b.tier)));
            text.push_str(&format!("적용 패치 버전: {}\n", if latest.version.is_empty() { "?".to_string() } else { latest.version.clone() }));
            text.push_str(&format!("총 매치 표본 수: {}경기\n\n", sets));
            for c in all {
                text.push_str(&format!(" - {}: [{}]티어 | 밴 {}회 | 픽 {}회 | 승리 {}회 (승률: {:.1}%, 등장률: {:.1}%)\n", champ_kr(&c.id), c.tier, c.bans, c.matches, c.wins, c.win_rate * 100.0, c.presence * 100.0));
            }
        }
    }
    text.push_str("\n");
    text.push_str(&format!("리그 명칭: {}\n", league_name));
    text.push_str(&format!("시즌 패치버전: {}\n", latest.version));
    text.push_str(&format!("경기 일정: {}\n\n", match_date));

    text.push_str("--- 역대 상대 전적 (Rivalry) ---\n");
    match req.opp { Some((o, w, l)) if o == opp_id => text.push_str(&format!("vs {} 상대 전적: {}승 {}패\n\n", db.team_name(opp_id), w, l)), _ => text.push_str("상대 팀과의 공식 경기 기록이 아직 없습니다.\n\n") }

    // ── 원본 순위 섹션 ──
    let league_comps: Vec<&CompRec> = comps.iter().filter(|c| !c.is_tournament).collect();
    let mut our_comp: Option<&CompRec> = our_match_id.and_then(|mid| league_comps.iter().copied().find(|c| c.matches.contains(&mid) || c.playoffs.contains(&mid) || c.promotion.contains(&mid)));
    if our_comp.is_none() { our_comp = league_comps.iter().copied().filter(|c| c.standings.get(our.to_string()).is_some()).max_by_key(|c| c.id); }
    let standing_rows = |st: &Value| -> Vec<(usize, i64, i64, i64, i64)> {
        st.as_object().map(|o| o.iter().filter_map(|(k, s)| Some((k.parse::<usize>().ok()?, ji(s, "win"), ji(s, "lose"), ji(s, "set_win") - ji(s, "set_lose"), ji(s, "kill")))).collect()).unwrap_or_default()
    };
    text.push_str("--- 현재 리그 순위 (Standings) ---\n");
    if let Some(c) = our_comp {
        let mut stage = String::from("국제전 및 기타 토너먼트");
        if let Some(mid) = our_match_id {
            if c.matches.contains(&mid) { stage = "정규 리그".into(); }
            else if let Some(pos) = c.playoffs.iter().position(|&x| x == mid) {
                stage = match pos { 0 => "플레이오프 1경기", 1 => "플레이오프 2경기", 2 | 3 => "플레이오프 패자전", 4 => "플레이오프 2라운드 승자조", 5 => "플레이오프 2라운드 패자조", 6 => "결승 진출전", 7 => "결승전", _ => "플레이오프" }.into();
            } else if c.promotion.contains(&mid) { stage = "승강전".into(); }
        }
        text.push_str(&format!("대회 형태: {}\n", stage));
        let mut rows = standing_rows(&c.standings);
        if let Some(r) = rows.iter().find(|r| r.0 == our) {
            let total_teams = rows.len();
            let total_matches = if total_teams > 0 { (total_teams - 1) * 2 } else { 0 };
            text.push_str(&format!("이번 리그 현재 경기 수: {}/{} 매치\n\n", r.1 + r.2, total_matches));
        }
        rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.3.cmp(&a.3)).then_with(|| b.4.cmp(&a.4)));
        for (rank, (tid, w, l, sd, _)) in rows.iter().enumerate() {
            let sds = if *sd > 0 { format!("+{}", sd) } else { sd.to_string() };
            text.push_str(&format!(" - {}위: {} ({}승 {}패 | 세트득실: {})\n", rank + 1, db.team_name(*tid), w, l, sds));
        }
    } else { text.push_str("현재 리그의 순위표 데이터를 찾을 수 없습니다.\n"); }
    text.push_str("\n");

    // ── 경기 구분 상세 ──
    {
        let push_rows = |text: &mut String, rows: &mut Vec<(usize, i64, i64, i64, i64)>| {
            rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.3.cmp(&a.3)).then_with(|| b.4.cmp(&a.4)));
            for (rank, (tid, w, l, sd, _)) in rows.iter().enumerate() {
                let star = if *tid == our { " ◀ 우리 팀" } else { "" };
                let sds = if *sd > 0 { format!("+{}", sd) } else { sd.to_string() };
                text.push_str(&format!(" - {}위: {} ({}승 {}패 | 세트득실: {}){}\n", rank + 1, db.team_name(*tid), w, l, sds, star));
            }
        };
        fn playoff_round_label(pos: usize, total: usize) -> String {
            if total == 8 { match pos { 0 | 1 => "플레이오프 승자조 1라운드", 2 | 3 => "플레이오프 패자조 1라운드", 4 => "플레이오프 승자조 결승", 5 => "플레이오프 패자조 2라운드", 6 => "플레이오프 패자조 결승 (결승 진출전)", _ => "결승전" }.to_string() }
            else if total > 0 && pos + 1 == total { "결승전".to_string() }
            else { format!("플레이오프 {}번째 경기 (총 {}경기)", pos + 1, total.max(pos + 1)) }
        }
        fn promotion_round_label(pos: usize, total: usize) -> String {
            if total <= 1 { "단판 승강전".to_string() } else if pos + 1 == total { format!("승강전 최종 경기 (총 {}경기 중 마지막)", total) } else { format!("승강전 {}번째 경기 (총 {}경기)", pos + 1, total.max(pos + 1)) }
        }
        let end_field = |mid: usize, key: &str| -> Option<usize> {
            let m = matches.get(&mid)?;
            let (k, _) = enum_of(&m.state);
            if k != "End" { return None; }
            m.state.get("End").and_then(|e| e.get(key)).and_then(|x| x.as_i64()).map(|x| x as usize)
        };
        let slot_str = |slot: &Value, list: &Vec<usize>| -> String {
            let local = |mid: usize| list.iter().position(|&x| x == mid).map(|k| format!("M{}", k + 1)).unwrap_or_else(|| format!("경기#{}", mid));
            let (k, nums) = enum_of(slot);
            match k.as_str() {
                "Normal" => nums.first().map(|t| db.team_name(*t as usize)).unwrap_or_else(|| slot.to_string()),
                "WinnerOf" => { let mid = nums.first().copied().unwrap_or(-1).max(0) as usize; let who = end_field(mid, "winner").map(|t| db.team_name(t)).unwrap_or_else(|| "미정".into()); format!("{} 승자({})", local(mid), who) }
                "LoserOf" => { let mid = nums.first().copied().unwrap_or(-1).max(0) as usize; let who = end_field(mid, "loser").map(|t| db.team_name(t)).unwrap_or_else(|| "미정".into()); format!("{} 패자({})", local(mid), who) }
                "LeagueRank" => nums.get(1).map(|r| format!("리그 {}위 시드", r)).unwrap_or_else(|| format!("리그시드{:?}", nums)),
                "CompetitionRank" => match (nums.first().copied(), nums.get(1).copied()) {
                    (Some(cid), Some(r)) if r >= 1 => {
                        let mut nm = String::new();
                        if let Some(c2) = league_comps.iter().find(|c| c.id as i64 == cid) {
                            let mut rows = standing_rows(&c2.standings);
                            rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.3.cmp(&a.3)).then_with(|| b.4.cmp(&a.4)));
                            if let Some(row) = rows.get((r - 1) as usize) { nm = db.team_name(row.0); }
                        }
                        if nm.is_empty() { format!("정규 {}위 시드", r) } else { format!("정규 {}위 시드({})", r, nm) }
                    }
                    _ => format!("대회시드{:?}", nums),
                },
                "GroupRank" => format!("조시드{:?}", nums),
                _ => slot.to_string(),
            }
        };
        let push_bracket = |text: &mut String, list: &Vec<usize>, fmt_label: &str, title: &str| {
            if list.is_empty() { return; }
            let (mut has_loser, mut has_winner, mut any_ended) = (false, false, false);
            for mid in list { if let Some(m) = matches.get(mid) { let k1 = enum_of(&m.team1).0; let k2 = enum_of(&m.team2).0; if k1 == "LoserOf" || k2 == "LoserOf" { has_loser = true; } if k1 == "WinnerOf" || k2 == "WinnerOf" { has_winner = true; } if m.ended { any_ended = true; } } }
            let shape = if has_loser { "더블 엘리미네이션형(패자조 간선 존재)" } else if has_winner { "녹아웃형(싱글 엘리미네이션 계열)" } else if any_ended { "전 경기 확정(간선 정보 소실 — 형식은 설정값 참조)" } else { "고정 대진형(조별/스위스 라운드)" };
            text.push_str(&format!("\n{} 형식: {} | 구조: {}\n", title, fmt_label, shape));
            for (i, mid) in list.iter().enumerate() {
                let Some(m) = matches.get(mid) else { text.push_str(&format!("[M{}] (매치 {} 정보 없음)\n", i + 1, mid)); continue };
                let bo = m.need_win.max(1) * 2 - 1;
                let (sk, _) = enum_of(&m.state);
                let state_s = if sk == "End" {
                    let e = m.state.get("End").cloned().unwrap_or(Value::Null);
                    format!("종료 {}:{} → 승자 {}", ji(&e, "team1_score"), ji(&e, "team2_score"), db.team_name(ji(&e, "winner").max(0) as usize))
                } else if sk == "Running" { "진행중".into() } else { "예정".into() };
                text.push_str(&format!("[M{}] {} | BO{} | {} vs {} | {}\n", i + 1, day_of(&m.date), bo, slot_str(&m.team1, list), slot_str(&m.team2, list), state_s));
            }
        };
        let format_label = |c: &CompRec| -> String {
            if c.is_tournament { format!("국제대회 규칙=설정 미확인 / 그룹 형식=설정 미확인 [raw {}]", c.format_raw) } else { format!("리그 플레이오프 규칙=설정 미확인 [raw {}]", c.format_raw) }
        };

        text.push_str("--- 경기 구분 상세 (추가 정보) ---\n");
        let mut done = false;
        if our_is_practice { text.push_str("경기 구분: 연습 경기 (스크림/비공식전 — 리그 순위와 무관)\n"); done = true; }
        if !done { if let Some(mid) = our_match_id {
            for c in league_comps.iter() {
                let in_regular = c.matches.contains(&mid);
                let po = c.playoffs.iter().position(|&x| x == mid);
                let ps = c.promotion.iter().position(|&x| x == mid);
                if !in_regular && po.is_none() && ps.is_none() { continue; }
                if in_regular {
                    text.push_str(&format!("경기 구분: 자국 리그 정규 시즌 — {}\n", league_name));
                } else if let Some(pp) = ps {
                    text.push_str(&format!("경기 구분: 승강전 — {}\n", promotion_round_label(pp, c.promotion.len())));
                    let league_of = |tid: usize| -> Option<(usize, String, i64)> { let lid = db.team(tid).league_id?; let l = leagues.get(&lid)?; Some((lid, league_kr(&l.0).to_string(), l.1)) };
                    match (league_of(our), league_of(opp_id)) {
                        (Some((ol, oln, odv)), Some((pl, pln, pdv))) => {
                            text.push_str(&format!("승강전 대진: 우리 팀 {} (division {}) vs 상대 팀 {} (division {})\n", oln, odv, pln, pdv));
                            text.push_str(if ol != pl { "→ 서로 다른 리그 소속 간 맞대결 = 상위 리그 잔류권과 승격권이 걸린 경기\n" } else { "→ 같은 리그 소속 간 승강전 경기\n" });
                        }
                        _ => text.push_str(&format!("승강전 대진: {} (상대 팀 소속 리그 확인 불가)\n", league_name)),
                    }
                    text.push_str(&format!("승강 결과 확정 여부: {}\n", if c.promotion_finalized { "확정됨" } else { "미확정 (진행중)" }));
                } else {
                    text.push_str(&format!("경기 구분: 리그 플레이오프 — {} / {}\n", league_name, playoff_round_label(po.unwrap_or(0), c.playoffs.len())));
                }
                push_bracket(&mut text, &c.playoffs, &format_label(c), "[플레이오프 대진표]");
                push_bracket(&mut text, &c.promotion, &format_label(c), "[승강전 대진표]");
                done = true;
                break;
            }
            if !done {
                let tours = db.tournaments();
                for tc in comps.iter().filter(|c| c.is_tournament) {
                    let in_group = tc.group_matches.contains(&mid);
                    let ko = tc.tournament_matches.iter().position(|&x| x == mid);
                    if !in_group && ko.is_none() { continue; }
                    let tname = tours.iter().find(|t| t.2.contains(&tc.id)).map(|t| tournament_kr(&t.1)).unwrap_or_else(|| "국제전".into());
                    let filled: Vec<usize> = tc.groups.iter().enumerate().filter(|(_, g)| g.as_object().map(|o| !o.is_empty()).unwrap_or(false)).map(|(i, _)| i).collect();
                    let is_swiss = filled.len() == 1 && tc.groups.get(filled[0]).and_then(|g| g.as_object()).map_or(false, |g| g.len() >= 8);
                    let stage = if in_group {
                        if is_swiss { "스위스 스테이지".to_string() }
                        else { match tc.groups.iter().position(|g| g.get(our.to_string()).is_some()) { Some(gi) => format!("조별 리그 ({}조)", gi + 1), None => "그룹 스테이지".into() } }
                    } else {
                        let pos = ko.unwrap_or(0); let total = tc.tournament_matches.len();
                        if total == 8 { playoff_round_label(pos, 8) } else if total > 0 && pos + 1 == total { "결승전".into() } else { format!("녹아웃 스테이지 {}번째 경기 (총 {}경기)", pos + 1, total.max(pos + 1)) }
                    };
                    text.push_str(&format!("경기 구분: 국제전 [{}] — {}\n", tname, stage));
                    if is_swiss {
                        if let Some(g) = tc.groups.get(filled[0]) {
                            text.push_str(&format!("\n[{} 스위스 스테이지 현황 (3승 진출 / 3패 탈락)]\n", tname));
                            let mut rows = standing_rows(g);
                            rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.2.cmp(&b.2)).then_with(|| b.3.cmp(&a.3)));
                            let mut cur: Option<(i64, i64)> = None;
                            for (tid, w, l, sd, _) in rows {
                                if cur != Some((w, l)) { cur = Some((w, l)); let tag = if w >= 3 { " — 녹아웃 진출" } else if l >= 3 { " — 탈락" } else { "" }; text.push_str(&format!("[{}-{}]{}\n", w, l, tag)); }
                                let star = if tid == our { " ◀ 우리 팀" } else { "" };
                                let sds = if sd > 0 { format!("+{}", sd) } else { sd.to_string() };
                                text.push_str(&format!(" - {} (세트득실 {}){}\n", db.team_name(tid), sds, star));
                            }
                        }
                        push_bracket(&mut text, &tc.group_matches, &format_label(tc), "[스위스 라운드 매치]");
                    } else {
                        for (gi, g) in tc.groups.iter().enumerate() {
                            if g.as_object().map(|o| o.is_empty()).unwrap_or(true) { continue; }
                            text.push_str(&format!("\n[{} {}조 순위]\n", tname, gi + 1));
                            let mut rows = standing_rows(g);
                            push_rows(&mut text, &mut rows);
                        }
                    }
                    push_bracket(&mut text, &tc.tournament_matches, &format_label(tc), "[플레이오프 대진표]");
                    done = true;
                    break;
                }
            }
        } }
        if !done { text.push_str("경기 구분: 판별 불가(진행 정보 부족)\n"); }
        text.push_str("\n");
    }

    // ── 경기 하이라이트 ──
    {
        let blocks = hl::blocks();
        let series_ids: Vec<u64> = our_series.iter().map(|r| r.id as u64).collect();
        let matched: Vec<&hl::HlBlock> = blocks.iter().filter(|b| {
            let (_, bm, br, _) = b.origin;
            if bm != u64::MAX { our_match_id.map(|m| m as u64) == Some(bm) }
            else if br != u64::MAX { series_ids.contains(&br) }
            else { our_series.iter().any(|r| ji(&r.blue_perf, "total_kills") == b.score.0 && ji(&r.red_perf, "total_kills") == b.score.1) }
        }).collect();
        if !matched.is_empty() {
            text.push_str("--- 경기 하이라이트 (직접 관전한 세트 프레임 분석) ---\n");
            text.push_str("연속킬 / 다인 궁극기 / 포탑 데스 / 골드 대역전 등 명장면 목록입니다.\n");
            let mut used: Vec<usize> = Vec::new();
            for (i, b) in matched.iter().enumerate() {
                let (_, _, br, bs) = b.origin;
                let mut set_idx: Option<usize> = if bs != u64::MAX && (bs as usize) < our_series.len() { Some(bs as usize) } else { None };
                if set_idx.is_none() && br != u64::MAX { set_idx = our_series.iter().position(|r| r.id as u64 == br); }
                if set_idx.is_none() { set_idx = our_series.iter().enumerate().position(|(si, r)| !used.contains(&si) && ji(&r.blue_perf, "total_kills") == b.score.0 && ji(&r.red_perf, "total_kills") == b.score.1); }
                let label = match set_idx { Some(si) => { used.push(si); format!("[{}세트 하이라이트 (킬스코어 {}:{})]", si + 1, b.score.0, b.score.1) } None => format!("[관전 세트 #{} (킬스코어 {}:{})]", i + 1, b.score.0, b.score.1) };
                text.push_str(&format!("{}\n", label));
                // 자리표시자 해석: {p:pid} → 선수명(리플레이 로스터: 팀0=블루 로스터에서 같은 챔피언), {c:champ} → 한글명
                let rep = set_idx.and_then(|si| our_series.get(si));
                for (tick, line) in &b.lines {
                    let mut s = line.clone();
                    for (pid, (champ, team, _)) in &b.players {
                        let ph = format!("{{p:{}}}", pid);
                        if !s.contains(&ph) { continue; }
                        let mut nm = champ_kr(champ);
                        if let Some(r) = rep {
                            let roster = if *team == 0 { &r.blue_team } else { &r.red_team };
                            if let Some(p) = roster.iter().find(|p| p.champion == *champ) { if let Some(a) = db.ath(p.athlete_id) { nm = a.name; } }
                        }
                        s = s.replace(&ph, &nm);
                    }
                    while let Some(st) = s.find("{c:") { if let Some(en) = s[st..].find('}') { let key = s[st + 3..st + en].to_string(); s.replace_range(st..st + en + 1, &champ_kr(&key)); } else { break; } }
                    text.push_str(&format!(" - [{}] {}\n", hl::fmt_tick(*tick), s));
                }
            }
            text.push_str("\n");
        }
    }

    for (i, r) in our_series.iter().enumerate() { text.push_str(&set_summary(&db, r, true, i + 1, our_attention)); }
    for (i, r) in other_series.iter().enumerate() { text.push_str(&set_summary(&db, r, false, i + 1, other_attention)); }

    // 시리즈 종료 = 팀 id 기준 승수(EXPORT_FORMAT §5 함정)
    let team_a = latest.blue_team_id;
    let (mut wa, mut wb) = (0i64, 0i64);
    for r in &our_series { let w = if r.blue_win { r.blue_team_id } else { r.red_team_id }; if w == team_a { wa += 1; } else { wb += 1; } }
    let finished = wa.max(wb) >= need_win;
    let js_out = format!(r#"var latestMatchData = {{"summary": "{}", "series_id": {}, "is_series_finished": {}, "date": "{}"}};"#, escape_json(&text), latest.id, finished, match_date);
    let dir = crate::mod_dir().ok_or("mod_dir 없음")?;
    let path = format!(r"{}\latest_match.js", dir);
    std::fs::write(&path, js_out.as_bytes()).map_err(|e| format!("write 실패 {}: {}", path, e))?;
    Ok((Some(opp_id), format!("#{} replay={} match={:?} sets={} other={} finished={} text={}B today={}", n, latest.id, our_match_id, our_series.len(), other_series.len(), finished, text.len(), req.today)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn json_helpers() {
        assert_eq!(enum_of(&json!("Wait")), ("Wait".to_string(), vec![]));
        assert_eq!(enum_of(&json!({"Normal": 5})), ("Normal".to_string(), vec![5]));
        assert_eq!(enum_of(&json!({"CompetitionRank": [3, 1]})), ("CompetitionRank".to_string(), vec![3, 1]));
        let end = json!({"End": {"winner": 4, "loser": 9, "team1_score": 3, "team2_score": 1}});
        assert_eq!(enum_of(&end).0, "End");
        assert_eq!(ji(end.get("End").unwrap(), "winner"), 4);
        assert_eq!(date_str(&json!("2026-02-04T09:00:00")), "2026-02-04 09:00:00");
        assert_eq!(day_of("2026-02-04 09:00:00"), "2026-02-04");
        let c = json!({"InContract": {"team_id": 1, "weekly_salary": 1234.5}});
        assert_eq!(find_key(&c, "weekly_salary").and_then(|x| x.as_f64()), Some(1234.5));
        assert_eq!(salary_fmt(1234.5), "6만원");
        assert_eq!(salary_fmt(3_000_000.0), "1.56억원");
        assert_eq!(slot_team(&json!({"Normal": 12})), Some(12));
        assert_eq!(slot_team(&json!({"WinnerOf": 12})), None);
        let m = parse_match(9, RecordKindV1::Match, &json!({"need_win": 3, "team1": {"Normal": 1}, "team2": {"WinnerOf": 8}, "running_state": "Running", "replays": [10, 11], "is_practice": false, "date": "2026-04-16T16:30:00"}), &HashSet::new());
        assert_eq!((m.need_win, m.ended, m.replays.clone(), m.day()), (3, false, vec![10, 11], "2026-04-16".to_string()));
        let r = parse_rep(&json!({"id": 10, "blue_team_id": 1, "red_team_id": 2, "blue_ban": ["ninja"], "red_ban": [], "blue_team_win": true, "version": "2026.1.0",
            "blue_team": [{"athlete_id": 3, "position": "Mid", "champion": "ninja", "items": [0, 7], "statistics": {"assists": 2, "gold": 100}}], "red_team": [],
            "blue_performance": {"kills": [1,2,3,4,5], "total_gold": 100}, "red_performance": {}}));
        assert_eq!(r.blue_team[0].position, "Mid");
        assert_eq!(items_str(&r.blue_team[0].items), "공격력[T1], 공속[T3]");
    }
    impl MatchRec { fn day(&self) -> String { day_of(&self.date).to_string() } }
}
