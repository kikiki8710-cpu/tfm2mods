use mod_api::*;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "Match_Info_Exporter";

static LAST_POLL_MS: AtomicU64 = AtomicU64::new(0);
static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);
static SEEN_ENDED_MATCHES: Mutex<Vec<usize>> = Mutex::new(Vec::new());

type HMODULE = isize;
type DWORD   = u32;
type BOOL    = i32;
const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS:       DWORD = 0x0000_0004;
const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: DWORD = 0x0000_0002;

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleExW(dwFlags: DWORD, lpModuleName: *const u16, phModule: *mut HMODULE) -> BOOL;
    fn GetModuleFileNameW(hModule: HMODULE, lpFilename: *mut u16, nSize: DWORD) -> DWORD;
}

fn get_current_dll_path() -> Option<PathBuf> {
    unsafe {
        let addr = get_current_dll_path as *const () as usize;
        let mut hmod: HMODULE = 0;
        let ok = GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            addr as *const u16,
            &mut hmod as *mut HMODULE,
        );
        if ok == 0 || hmod == 0 { return None; }
        let mut buf: [u16; 4096] = [0; 4096];
        let len = GetModuleFileNameW(hmod, buf.as_mut_ptr(), buf.len() as DWORD);
        if len == 0 { return None; }
        Some(PathBuf::from(String::from_utf16_lossy(&buf[..len as usize])))
    }
}

fn get_output_file_path(file_name: &str) -> Option<PathBuf> {
    let dir = get_current_dll_path()?.parent()?.to_path_buf();
    Some(dir.join(file_name))
}

fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}

fn safe_write_file(file_name: &str, content: &str) {
    let Some(out_file) = get_output_file_path(file_name) else { return; };
    if let Some(parent) = out_file.parent() {
        let _ = fs::create_dir_all(Path::new(parent));
    }
    let _ = fs::write(out_file, content);
}

// ── 아이템 ID → 한국어 (티어 표기 없음) ─────────────────
fn item_name(id: usize) -> &'static str {
    match id {
        0  => "공격력1", 1  => "공격력2", 2  => "공격력3", 3  => "공격력4", 4  => "공격력5",
        5  => "공속1",   6  => "공속2",   7  => "공속3",   8  => "공속4",   9  => "공속5",
        10 => "방어력1", 11 => "방어력2", 12 => "방어력3", 13 => "방어력4", 14 => "방어력5",
        15 => "마저1",   16 => "마저2",   17 => "마저3",   18 => "마저4",   19 => "마저5",
        20 => "주문력1", 21 => "주문력2", 22 => "주문력3", 23 => "주문력4", 24 => "주문력5",
        25 => "체력1",   26 => "체력2",   27 => "체력3",   28 => "체력4",   29 => "체력5",
        _  => "알수없음",
    }
}

// ── 챔피언 영문 → 한국어 ─────────────────────────────────
fn champion_name_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("acrobat",            "봉술사");
    m.insert("android",            "안드로이드");
    m.insert("archer",             "궁수");
    m.insert("bard",               "음유시인");
    m.insert("barrier_magician",   "결계사");
    m.insert("berserker",          "광전사");
    m.insert("bomber",             "폭탄병");
    m.insert("boomerang_hunter",   "부메랑 헌터");
    m.insert("cavalry_knight",     "기병");
    m.insert("chef",               "요리사");
    m.insert("circus_blade",       "곡예사");
    m.insert("clown",              "광대");
    m.insert("dancer",             "무희");
    m.insert("dark_mage",          "흑마술사");
    m.insert("demon",              "악마");
    m.insert("dokkaebi",           "도깨비");
    m.insert("druid",              "드루이드");
    m.insert("dual_blader",        "듀얼 블레이더");
    m.insert("enchanter",          "인챈터");
    m.insert("executioner",        "처형인");
    m.insert("exorcist",           "엑소시스트");
    m.insert("fighter",            "격투가");
    m.insert("gambler",            "도박사");
    m.insert("ghost",              "유령");
    m.insert("guardian_spirit",    "수호령");
    m.insert("gunner",             "총잡이");
    m.insert("hammerer",           "중보병");
    m.insert("hitman",             "히트맨");
    m.insert("hunter",             "사냥꾼");
    m.insert("ice_mage",           "얼음술사");
    m.insert("illusionist",        "환영술사");
    m.insert("inquisitor",         "이단심문관");
    m.insert("jiangshi",           "강시");
    m.insert("knight",             "기사");
    m.insert("lancer",             "창술사");
    m.insert("lightning_mage",     "번개술사");
    m.insert("magic_knight",       "마검사");
    m.insert("monk",               "몽크");
    m.insert("necromancer",        "네크로맨서");
    m.insert("ninja",              "닌자");
    m.insert("ogre",               "오우거");
    m.insert("plague_doctor",      "역병의사");
    m.insert("poison_dart_hunter", "독침술사");
    m.insert("pole_warrior",       "봉술사");
    m.insert("priest",             "성직자");
    m.insert("prisoner",           "죄수");
    m.insert("pyromancer",         "화염술사");
    m.insert("pythoness",          "무녀");
    m.insert("shadowmancer",       "그림자술사");
    m.insert("shield_bearer",      "방패병");
    m.insert("siege_breaker",      "공성병");
    m.insert("sniper",             "소총수");
    m.insert("soldier",            "중보병");
    m.insert("spirit_caller",      "정령사");
    m.insert("swordman",           "검사");
    m.insert("taoist",             "도사");
    m.insert("vampire",            "흡혈귀");
    m.insert("voodoo",             "부두술사");
    m.insert("voodoo_shaman",      "부두술사");
    m.insert("whip_master",        "채찍술사");
    m.insert("white_mage",         "백마법사");
    m.insert("wind_mage",          "바람술사");
    m
}

fn champ_kr(key: &str, map: &HashMap<&'static str, &'static str>) -> String {
    map.get(key).map(|s| s.to_string()).unwrap_or_else(|| key.to_string())
}

fn js(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"'  => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c    => out.push(c),
        }
    }
    out.push('"');
    out
}

fn pos_str(p: &Position) -> &'static str {
    match p {
        Position::Top     => "탑",
        Position::Jungle  => "정글",
        Position::Mid     => "미드",
        Position::Bottom  => "바텀",
        Position::Support => "서포터",
    }
}

fn parse_usize_field(s: &str, field: &str) -> usize {
    let key = format!("{}: ", field);
    let Some(start) = s.find(&key) else { return 0; };
    let rest = &s[start + key.len()..];
    let end = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
    rest[..end].parse().unwrap_or(0)
}

fn parse_bool_field(s: &str, field: &str) -> bool {
    let key = format!("{}: ", field);
    let Some(start) = s.find(&key) else { return false; };
    s[start + key.len()..].starts_with("true")
}

fn parse_usize_array(s: &str, field: &str) -> Vec<usize> {
    let key = format!("{}: [", field);
    let Some(start) = s.find(&key) else { return vec![]; };
    let rest = &s[start + key.len()..];
    let Some(end) = rest.find(']') else { return vec![]; };
    rest[..end].split(',').map(|v| v.trim().parse().unwrap_or(0)).collect()
}

fn parse_running_state(s: &str) -> Option<(usize, usize, usize)> {
    if !s.contains("End {") { return None; }
    Some((
        parse_usize_field(s, "winner"),
        parse_usize_field(s, "team1_score"),
        parse_usize_field(s, "team2_score"),
    ))
}

fn parse_normal_team(s: &str) -> Option<usize> {
    let start = s.find("Normal(")? + "Normal(".len();
    let rest = &s[start..];
    let end = rest.find(')')?;
    rest[..end].parse().ok()
}

fn parse_team_fan(team_s: &str) -> (String, String, usize) {
    let expectation = {
        let key = "fan_expectation: ";
        if let Some(pos) = team_s.find(key) {
            let rest = &team_s[pos + key.len()..];
            let end = rest.find(|c: char| c == ',' || c == '\n' || c == '}').unwrap_or(rest.len());
            rest[..end].trim().to_string()
        } else { String::new() }
    };
    let satisfaction = {
        let key = "fan_satisfaction: ";
        if let Some(pos) = team_s.find(key) {
            let rest = &team_s[pos + key.len()..];
            let end = rest.find(|c: char| c == ',' || c == '\n' || c == '}').unwrap_or(rest.len());
            rest[..end].trim().to_string()
        } else { String::new() }
    };
    let fan_count = parse_usize_field(team_s, "fan_count");
    (expectation, satisfaction, fan_count)
}

fn fan_expect_kr(s: &str) -> &'static str {
    match s {
        "VeryHigh" => "매우 높음", "High" => "높음", "Upper" => "보통 이상",
        "Normal"   => "보통",      "Lower" => "보통 이하", "Low" => "낮음",
        "VeryLow"  => "매우 낮음", _ => "알수없음",
    }
}

fn fan_satisf_kr(s: &str) -> &'static str {
    match s {
        "VerySatisfied" => "매우 만족", "Satisfied" => "만족", "Normal" => "보통",
        "Dissatisfied"  => "불만족",    "VeryDissatisfied" => "매우 불만족", _ => "알수없음",
    }
}

fn calc_tier(win_rate: f64, appear_rate: f64) -> &'static str {
    if win_rate >= 0.65 || (win_rate >= 0.55 && appear_rate >= 0.55) { "S" }
    else if win_rate >= 0.52 { "A" }
    else if win_rate >= 0.46 { "B" }
    else if win_rate >= 0.38 { "C" }
    else { "D" }
}

fn build_champion_stats_json(db: &ClientDatabase, name_map: &HashMap<&'static str, &'static str>) -> (String, usize, String) {
    let patch_key = db.champion_patch_statistics
        .keys().max_by(|a, b| a.cmp(b)).cloned().unwrap_or_default();
    let mut total_match = 0usize;
    let mut entries: Vec<String> = Vec::new();

    if let Some(patch_data) = db.champion_patch_statistics.get(&patch_key) {
        total_match = patch_data.total_match as usize;
        let mut list: Vec<(String, usize, usize, usize, f64, f64)> = Vec::new();
        for (key, stat) in patch_data.data.iter() {
            let name = champ_kr(key, name_map);
            let ban  = stat.bans as usize;
            let mut pick = 0usize; let mut win = 0usize;
            for (_pos, cs) in stat.by_position.iter() {
                pick += cs.matches as usize; win += cs.wins as usize;
            }
            let wr = if pick > 0 { win as f64 / pick as f64 } else { 0.0 };
            let ar = if total_match > 0 { (pick + ban) as f64 / (total_match * 2) as f64 } else { 0.0 };
            list.push((name, ban, pick, win, wr, ar));
        }
        list.sort_by(|a, b| b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal));
        for (name, ban, pick, win, wr, ar) in &list {
            entries.push(format!(
                "{{\"챔피언\":{},\"티어\":{},\"밴\":{},\"픽\":{},\"승\":{},\"승률\":{:.1},\"등장률\":{:.1}}}",
                js(name), js(calc_tier(*wr, *ar)), ban, pick, win,
                (wr * 1000.0).round() / 10.0, (ar * 1000.0).round() / 10.0,
            ));
        }
    }
    (patch_key, total_match, entries.join(","))
}

fn build_standings_json(db: &ClientDatabase, player_team_id: usize) -> String {
    let mut best_comp_id: Option<usize> = None;
    let mut best_max_match_id = 0usize;
    for cid in db.league_competition_ids() {
        if let Some(comp) = db.league_competition(cid) {
            let s = format!("{:?}", *comp);
            let needle = format!("{}: LeagueStanding", player_team_id);
            if !s.contains(&needle) { continue; }
            let max_id = comp.matches.iter().cloned().max().unwrap_or(0);
            if max_id > best_max_match_id {
                best_max_match_id = max_id;
                best_comp_id = Some(cid);
            }
        }
    }
    let Some(cid) = best_comp_id else { return "[]".to_string(); };
    let Some(comp) = db.league_competition(cid) else { return "[]".to_string(); };
    let s = format!("{:?}", *comp);
    let Some(st_pos) = s.find("standings: {") else { return "[]".to_string(); };
    let after_st = &s[st_pos + "standings: {".len()..];
    let mut depth = 1usize; let mut st_end = 0usize;
    for (i, c) in after_st.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => { depth -= 1; if depth == 0 { st_end = i; break; } }
            _ => {}
        }
    }
    let standings_str = &after_st[..st_end];
    let mut rows: Vec<(usize, usize, usize, usize, usize)> = Vec::new();
    let mut scan = standings_str;
    while let Some(colon) = scan.find(": LeagueStanding {") {
        let before = &scan[..colon];
        let tid_str = before.split_whitespace().last().unwrap_or("0");
        let tid: usize = tid_str.trim_matches(',').parse().unwrap_or(0);
        let block = &scan[colon + ": LeagueStanding {".len()..];
        rows.push((tid,
            parse_usize_field(block, "win"), parse_usize_field(block, "lose"),
            parse_usize_field(block, "set_win"), parse_usize_field(block, "set_lose"),
        ));
        scan = &scan[colon + 1..];
    }
    rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| (b.3 as i64 - b.4 as i64).cmp(&(a.3 as i64 - a.4 as i64))));
    let mut entries: Vec<String> = Vec::new();
    for (rank, (tid, win, lose, set_win, set_lose)) in rows.iter().enumerate() {
        let team_name = db.teams.get(tid).map(|t| t.name.clone()).unwrap_or_else(|| format!("Team{}", tid));
        entries.push(format!(
            "{{\"순위\":{},\"팀ID\":{},\"팀\":{},\"승\":{},\"패\":{},\"세트승\":{},\"세트패\":{}}}",
            rank + 1, tid, js(&team_name), win, lose, set_win, set_lose
        ));
    }
    format!("[{}]", entries.join(","))
}

fn build_team_info_json(db: &ClientDatabase, team_id: usize) -> String {
    let Some(team) = db.teams.get(&team_id) else { return format!("{{\"팀ID\":{}}}", team_id); };
    let team_s = format!("{:?}", *team);
    let (expect, satisf, fan_count) = parse_team_fan(&team_s);
    format!(
        "{{\"팀명\":{},\"감독\":{},\"팬기대치\":{},\"팬만족도\":{},\"팬수\":{}}}",
        js(&team.name), js(&team.manager_name),
        js(fan_expect_kr(&expect)), js(fan_satisf_kr(&satisf)), fan_count,
    )
}

fn is_match_ended(match_info: &MatchInfo) -> bool { !match_info.replays.is_empty() }

fn get_match_kind(db: &ClientDatabase, match_id: usize, player_team_id: usize, is_practice: bool) -> (&'static str, String) {
    if is_practice { return ("연습경기", String::new()); }
    for cid in db.league_competition_ids() {
        if let Some(comp) = db.league_competition(cid) {
            let s = format!("{:?}", *comp);
            if !s.contains(&format!("{}: LeagueStanding", player_team_id)) { continue; }
            if comp.playoffs.contains(&match_id) { return ("리그 플레이오프", String::new()); }
            if comp.matches.contains(&match_id)  { return ("정규시즌", String::new()); }
        }
    }
    // 어디에도 없으면 국제전 - 토너먼트 이름 탐색
    let mut tournament_name = String::new();
    for tid in db.tournament_ids() {
        if let Some(t) = db.tournament(tid) {
            for &cid in &t.competitions {
                if let Some(comp) = db.league_competition(cid) {
                    let s = format!("{:?}", *comp);
                    if s.contains(&format!("{}: LeagueStanding", player_team_id)) {
                        tournament_name = match t.name.as_str() {
                            "Challengers E" => "챌린저스 이스턴".to_string(),
                            "Challengers W" => "챌린저스 웨스턴".to_string(),
                            "Masters"       => "마스터스".to_string(),
                            "Champions"     => "챔피언스".to_string(),
                            _               => t.name.clone(),
                        };
                        break;
                    }
                }
            }
        }
        if !tournament_name.is_empty() { break; }
    }
    ("국제전", tournament_name)
}

fn build_match_json(db: &ClientDatabase, match_id: usize, name_map: &HashMap<&'static str, &'static str>) -> Option<String> {
    let match_info = db.normal_match(match_id)?;
    let state_str = format!("{:?}", match_info.running_state);
    let (winner_id, team1_score, team2_score) = parse_running_state(&state_str)?;
    let team1_id = parse_normal_team(&format!("{:?}", match_info.team1))?;
    let team2_id = parse_normal_team(&format!("{:?}", match_info.team2))?;
    let winner_name = db.teams.get(&winner_id).map(|t| t.name.clone()).unwrap_or_else(|| format!("Team{}", winner_id));
    let team1_info = build_team_info_json(db, team1_id);
    let team2_info = build_team_info_json(db, team2_id);

    let player_team_id = db.player_team_id();
    let is_practice = false; // normal_match이므로
    let (match_kind, tournament_name) = get_match_kind(db, match_id, player_team_id, is_practice);
    let (h2h_my_wins, h2h_opp_wins) = if team1_id == player_team_id {
        db.head_to_head.get(&team2_id).copied().unwrap_or((0, 0))
    } else if team2_id == player_team_id {
        let (w, l) = db.head_to_head.get(&team1_id).copied().unwrap_or((0, 0));
        (l, w)
    } else {
        (0, 0)
    };
    let h2h_json = format!("{{\"팀1승\":{},\"팀2승\":{}}}", h2h_my_wins, h2h_opp_wins);

    let mut sets: Vec<String> = Vec::new();
    for &replay_id in &match_info.replays {
        if let Some(replay) = db.match_replays.get(&replay_id) {
            let rs = format!("{:?}", replay);
            let bp_start = rs.find("blue_performance:").unwrap_or(0);
            let rp_start = rs.find("red_performance:").unwrap_or(0);
            let bp = if bp_start > 0 { let e = rs[bp_start..].find('}').map(|e| bp_start+e+1).unwrap_or(rs.len()); &rs[bp_start..e] } else { "" };
            let rp = if rp_start > 0 { let e = rs[rp_start..].find('}').map(|e| rp_start+e+1).unwrap_or(rs.len()); &rs[rp_start..e] } else { "" };

            let bka = parse_usize_array(bp, "kills");
            let bda = parse_usize_array(bp, "deaths");
            let bdl = parse_usize_array(bp, "deal");
            let rka = parse_usize_array(rp, "kills");
            let rda = parse_usize_array(rp, "deaths");
            let rdl = parse_usize_array(rp, "deal");

            // 게임시간 (game_tick → 분:초, 1틱 ≈ 1/30초)
            let total_secs = replay.game_tick / 30;
            let game_time = format!("{}분{}초", total_secs / 60, total_secs % 60);

            let blue_win = replay.blue_team_id == winner_id;
            let blue_team_name = db.teams.get(&replay.blue_team_id).map(|t| t.name.clone()).unwrap_or_default();
            let red_team_name  = db.teams.get(&replay.red_team_id).map(|t| t.name.clone()).unwrap_or_default();
            let blue_bans: Vec<String> = replay.blue_ban.iter().map(|k| js(&champ_kr(k, name_map))).collect();
            let red_bans: Vec<String>  = replay.red_ban.iter().map(|k| js(&champ_kr(k, name_map))).collect();

            let mut blue_players: Vec<String> = Vec::new();
            for (i, a) in replay.blue_team.iter().enumerate() {
                let name   = db.athletes.get(&a.athlete_id).map(|ath| ath.name.clone()).unwrap_or_else(|| format!("P{}", a.athlete_id));
                let stat_s = format!("{:?}", a.statistics);
                let kills  = if i < bka.len() { bka[i] } else { parse_usize_field(&stat_s, "kills") };
                let deaths = if i < bda.len() { bda[i] } else { parse_usize_field(&stat_s, "deaths") };
                let deal   = if i < bdl.len() { bdl[i] } else { parse_usize_field(&stat_s, "dealing") };
                let ath_s  = db.athletes.get(&a.athlete_id).map(|ath| format!("{:?}", *ath)).unwrap_or_default();
                let items_json: Vec<String> = a.items.iter().map(|&id| js(item_name(id as usize))).collect();
                // 시즌 전적
                let s_kills   = parse_usize_field(&ath_s, "kills");
                let s_deaths  = parse_usize_field(&ath_s, "deaths");
                let s_assists = parse_usize_field(&ath_s, "assists");
                let s_matches = parse_usize_field(&ath_s, "matches");
                let s_wins    = parse_usize_field(&ath_s, "wins");
                let s_mvp     = parse_usize_field(&ath_s, "mvp");
                blue_players.push(format!(
                    "{{\"이름\":{},\"포지션\":{},\"챔피언\":{},\"킬\":{},\"데스\":{},\"어시\":{},\"딜량\":{},\"받은피해\":{},\"힐량\":{},\"골드\":{},\"아이템\":[{}],\"팬수\":{},\"팬기대치\":{},\"팬만족도\":{},\"시즌킬\":{},\"시즌데스\":{},\"시즌어시\":{},\"시즌경기수\":{},\"시즌승수\":{},\"시즌MVP\":{}}}",
                    js(&name), js(pos_str(&a.position)), js(&champ_kr(&a.champion, name_map)),
                    kills, deaths, parse_usize_field(&stat_s, "assists"),
                    deal, parse_usize_field(&stat_s, "tanking"), parse_usize_field(&stat_s, "healing"),
                    parse_usize_field(&stat_s, "gold"), items_json.join(","),
                    parse_usize_field(&ath_s, "fan_count"), parse_usize_field(&ath_s, "fan_expectation"), parse_usize_field(&ath_s, "fan_satisfaction"),
                    s_kills, s_deaths, s_assists, s_matches, s_wins, s_mvp,
                ));
            }

            let mut red_players: Vec<String> = Vec::new();
            for (i, a) in replay.red_team.iter().enumerate() {
                let name   = db.athletes.get(&a.athlete_id).map(|ath| ath.name.clone()).unwrap_or_else(|| format!("P{}", a.athlete_id));
                let stat_s = format!("{:?}", a.statistics);
                let kills  = if i < rka.len() { rka[i] } else { parse_usize_field(&stat_s, "kills") };
                let deaths = if i < rda.len() { rda[i] } else { parse_usize_field(&stat_s, "deaths") };
                let deal   = if i < rdl.len() { rdl[i] } else { parse_usize_field(&stat_s, "dealing") };
                let ath_s  = db.athletes.get(&a.athlete_id).map(|ath| format!("{:?}", *ath)).unwrap_or_default();
                let items_json: Vec<String> = a.items.iter().map(|&id| js(item_name(id as usize))).collect();
                let s_kills   = parse_usize_field(&ath_s, "kills");
                let s_deaths  = parse_usize_field(&ath_s, "deaths");
                let s_assists = parse_usize_field(&ath_s, "assists");
                let s_matches = parse_usize_field(&ath_s, "matches");
                let s_wins    = parse_usize_field(&ath_s, "wins");
                let s_mvp     = parse_usize_field(&ath_s, "mvp");
                red_players.push(format!(
                    "{{\"이름\":{},\"포지션\":{},\"챔피언\":{},\"킬\":{},\"데스\":{},\"어시\":{},\"딜량\":{},\"받은피해\":{},\"힐량\":{},\"골드\":{},\"아이템\":[{}],\"팬수\":{},\"팬기대치\":{},\"팬만족도\":{},\"시즌킬\":{},\"시즌데스\":{},\"시즌어시\":{},\"시즌경기수\":{},\"시즌승수\":{},\"시즌MVP\":{}}}",
                    js(&name), js(pos_str(&a.position)), js(&champ_kr(&a.champion, name_map)),
                    kills, deaths, parse_usize_field(&stat_s, "assists"),
                    deal, parse_usize_field(&stat_s, "tanking"), parse_usize_field(&stat_s, "healing"),
                    parse_usize_field(&stat_s, "gold"), items_json.join(","),
                    parse_usize_field(&ath_s, "fan_count"), parse_usize_field(&ath_s, "fan_expectation"), parse_usize_field(&ath_s, "fan_satisfaction"),
                    s_kills, s_deaths, s_assists, s_matches, s_wins, s_mvp,
                ));
            }

            sets.push(format!(
                "{{\"리플레이ID\":{},\"블루팀\":{},\"레드팀\":{},\"블루승\":{},\"게임시간\":{},\
                  \"블루밴\":[{}],\"레드밴\":[{}],\
                  \"블루총골드\":{},\"레드총골드\":{},\
                  \"블루총킬\":{},\"레드총킬\":{},\
                  \"블루모르가드\":{},\"레드모르가드\":{},\
                  \"블루세르펜\":{},\"레드세르펜\":{},\
                  \"모르가드선취(블루)\":{},\"세르펜선취(블루)\":{},\
                  \"블루선수\":[{}],\"레드선수\":[{}]}}",
                replay_id,
                js(&blue_team_name), js(&red_team_name), blue_win, js(&game_time),
                blue_bans.join(","), red_bans.join(","),
                parse_usize_field(bp, "total_gold"), parse_usize_field(rp, "total_gold"),
                parse_usize_field(bp, "total_kills"), parse_usize_field(rp, "total_kills"),
                parse_usize_field(bp, "epic_secured"), parse_usize_field(rp, "epic_secured"),
                parse_usize_field(bp, "serpen_secured"), parse_usize_field(rp, "serpen_secured"),
                parse_bool_field(bp, "first_epic"), parse_bool_field(bp, "first_serpen"),
                blue_players.join(","), red_players.join(","),
            ));
        }
    }

    let tournament_part = if !tournament_name.is_empty() {
        format!(",\"토너먼트\":{}", js(&tournament_name))
    } else { String::new() };

    Some(format!(
        "{{\"경기종류\":{},\"경기ID\":{},\"날짜\":{},\"팀1\":{},\"팀2\":{},\"승자\":{},\"팀1점수\":{},\"팀2점수\":{},\"상대전적\":{},\"세트\":[{}]{}}}",
        js(match_kind), match_id, js(&format!("{}", match_info.date)),
        team1_info, team2_info, js(&winner_name), team1_score, team2_score,
        h2h_json, sets.join(","), tournament_part,
    ))
}

struct ExporterExtension;

impl ModExtension for ExporterExtension {
    fn post_update(&self, scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        let Scene::InGame { data } = scene else { return; };
        let t = now_ms();
        let last = LAST_POLL_MS.load(Ordering::Relaxed);
        if t.saturating_sub(last) < 5000 { return; }
        LAST_POLL_MS.store(t, Ordering::Relaxed);

        let db = data.db();
        let player_team_id = db.player_team_id();
        let name_map = champion_name_map();
        let mut seen = SEEN_ENDED_MATCHES.lock().unwrap();

        if !IS_INITIALIZED.load(Ordering::Relaxed) {
            for (match_type, match_info) in db.matches.iter() {
                let match_id = match match_type { MatchType::Normal { match_id } => *match_id, _ => continue };
                if !db.match_has_player_team(match_info) { continue; }
                if is_match_ended(match_info) && !seen.contains(&match_id) { seen.push(match_id); }
            }
            IS_INITIALIZED.store(true, Ordering::Relaxed);
            return;
        }

        let mut newly_ended: Vec<usize> = Vec::new();
        for (match_type, match_info) in db.matches.iter() {
            let match_id = match match_type { MatchType::Normal { match_id } => *match_id, _ => continue };
            if !db.match_has_player_team(match_info) { continue; }
            if is_match_ended(match_info) && !seen.contains(&match_id) {
                newly_ended.push(match_id);
                seen.push(match_id);
            }
        }
        if newly_ended.is_empty() { return; }

        let latest_id = newly_ended.iter().max_by(|a, b| {
            let da = db.normal_match(**a).map(|m| m.date).unwrap_or_default();
            let db_d = db.normal_match(**b).map(|m| m.date).unwrap_or_default();
            da.cmp(&db_d)
        }).copied();

        let Some(mid) = latest_id else { return; };
        let Some(match_json) = build_match_json(&db, mid, &name_map) else { return; };
        let (patch_key, total_match, champ_stats_json) = build_champion_stats_json(&db, &name_map);

        // 경기종류 확인 - 정규시즌/플레이오프만 순위표 출력
        let (match_kind, _) = get_match_kind(&db, mid, player_team_id, false);
        let standings_json = if match_kind == "정규시즌" || match_kind == "리그 플레이오프" {
            build_standings_json(&db, player_team_id)
        } else {
            "[]".to_string()
        };

        let json = format!(
            "{{\"패치버전\":{},\"게임날짜\":{},\"총경기수\":{},\"순위표\":{},\"챔피언통계\":[{}],\"경기\":{}}}",
            js(&patch_key), js(&format!("{}", db.time)),
            total_match, standings_json, champ_stats_json, match_json,
        );
        safe_write_file("latest_match.json", &json);
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(ExporterExtension);
    reg
}

declare_mod!(init);