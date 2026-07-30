// TFM2 save probe with salvage fallback.
//
// [2026-07-14] TFM2.gg 대시보드 0.5.0_3 병합용 변종. 원본 = save_probe_047.rs
// (base_network.debug.txt export 포함) + `save_file`(basename) manifest 필드 추가
// (0.5.x refresh.ps1 이 manifest.tsv 의 save_file 로 세이브 일치를 검증하므로 필수).
// 빌드: rustc -O --edition 2021, SDK=C:\tfm2mods\sdk_050_hotfix2\mod-sdk,
//   toolchain=nightly-2026-05-24 (SDK rlib 에 맞춤; 모드빌드용 06-16 아님),
//   --extern game_core/engine_core/flate2 (SDK deps). 배포 =
//   workshop\...\3738242091\DashboardApp\resources\app\tfm2_meta_dashboard\tools\tfm2_save_probe.exe
//
// The dashboard's original probe calls game_core::Database::load(), which is
// all-or-nothing: a single later field whose serialization changed in a newer
// game build (e.g. the 0.4.14 hotfix `stadium_setting`) makes the whole load
// fail, so the dashboard gets nothing even though every match record decoded
// fine. This version first tries the normal full load; if that fails it
// decompresses the save itself and deserializes the Database field-by-field in
// declaration order, writing every debug file it can reach before the broken
// field. The primary champion-meta source (match_replays) plus teams/athletes/
// competitions all serialize before the break, so the dashboard keeps working.

use std::env;
use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process;

use engine_core::spitz::SpitzDatable;
use game_core::Database;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err("usage: tfm2_save_probe.exe <save-path-or-file-name> <output-dir>".into());
    }
    let requested_save = args[1].trim_matches('"').to_string();
    let out_dir = PathBuf::from(args[2].trim_matches('"'));
    fs::create_dir_all(&out_dir)?;

    // 1) Try the normal full load first (works for matching game builds).
    let load_name = save_name_for_database_load(&requested_save)?;
    match Database::load(&load_name) {
        Ok(database) => full_dump(&out_dir, &database, &requested_save, &load_name),
        Err(err) => {
            // 2) Salvage: decompress + field-by-field decode of what we can.
            let full_err = format!("{err:?}");
            eprintln!("full load failed, attempting salvage: {full_err}");
            salvage(&out_dir, &requested_save, &full_err)
        }
    }
}

// ---------------------------------------------------------------------------
// Full (normal) dump — identical output to the original probe.
// ---------------------------------------------------------------------------

fn full_dump(
    out_dir: &Path,
    database: &Database,
    requested_save: &str,
    load_name: &str,
) -> Result<(), Box<dyn Error>> {
    let teams = write_debug(out_dir, "teams.debug.txt", &database.teams)?;
    let athletes = write_debug(out_dir, "athletes.debug.txt", &database.athletes)?;
    // matches: DataTable<MatchInfo> — has is_practice + replays(=match_replays keys).
    // Used by the dashboard to exclude practice/scrim games from stats.
    write_debug(out_dir, "matches.debug.txt", &database.matches)?;
    // base_network: 밴픽 AI(FactoredBanpickAgent) 학습 가중치/시너지·카운터 테이블.
    write_debug(out_dir, "base_network.debug.txt", &database.base_network)?;
    write_debug(
        out_dir,
        "champion_patch_statistics.debug.txt",
        &database.champion_patch_statistics,
    )?;
    let solo = write_debug(out_dir, "solo_rank_matches.debug.txt", &database.solo_rank_matches)?;
    let replays = write_debug(out_dir, "match_replays.debug.txt", &database.match_replays)?;
    let leaguec = write_debug(out_dir, "league_competitions.debug.txt", &database.league_competitions)?;
    let tournc = write_debug(
        out_dir,
        "tournament_competitions.debug.txt",
        &database.tournament_competitions,
    )?;
    let years = write_debug(out_dir, "year_schedules.debug.txt", &database.year_schedules)?;
    let stats = write_debug(out_dir, "match_stats.debug.txt", &database.match_stats)?;
    write_debug(out_dir, "champion_info_sheet.debug.txt", &database.champion_info_sheet)?;
    write_debug(out_dir, "pre_patch_data.debug.txt", &database.pre_patch_data)?;
    write_debug(
        out_dir,
        "champion_action_patch_state.debug.txt",
        &database.champion_action_patch_state,
    )?;

    // ★모드 전용 세이브 공간(ModSaveData). 게임이 공식 제공하는 네임스페이스 키-값 저장소로,
    //   모드가 "게임 실행 중에만 알 수 있는 정보"(예: 아이템 상하위 트리 next_tier)를 여기에 남긴다.
    //   Debug 덤프 + 대시보드가 바로 읽을 수 있는 평문(JSON 문자열 그대로)도 함께 내보낸다.
    write_debug(out_dir, "mod_save_data.debug.txt", &database.mod_save_data)?;
    dump_mod_save_namespaces(out_dir, &database.mod_save_data)?;

    let rows = [
        ("reason", "save_probe".to_string()),
        ("save", requested_save.to_string()),
        // save_file: basename the 0.5.x refresh.ps1 validates against the selected save.
        ("save_file", load_name.to_string()),
        ("load_name", load_name.to_string()),
        ("decode", "ok".to_string()),
        ("teams", count_entries(&teams).to_string()),
        ("athletes", count_entries(&athletes).to_string()),
        (
            "champion_patch_statistics",
            database.champion_patch_statistics.len().to_string(),
        ),
        ("solo_rank_matches", count_entries(&solo).to_string()),
        ("match_replays", count_entries(&replays).to_string()),
        ("league_competitions", count_entries(&leaguec).to_string()),
        ("tournament_competitions", count_entries(&tournc).to_string()),
        ("year_schedules", count_entries(&years).to_string()),
        ("match_stats", count_entries(&stats).to_string()),
    ];
    write_manifest(&out_dir.join("manifest.tsv"), &rows)?;
    write_manifest(&out_dir.join("save_probe_manifest.tsv"), &rows[1..])?;
    println!(
        "save_probe ok: save={} teams={} athletes={} match_replays={}",
        requested_save, rows[5].1, rows[6].1, rows[9].1
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Salvage path
// ---------------------------------------------------------------------------

/// Deserialize a single Database field. The closure ties the inferred type `T`
/// to the matching Database field so the field type never has to be named
/// (the field types live in a private module).
fn de<T: SpitzDatable>(_p: impl Fn(&Database) -> &T, b: &[u8], pos: &mut usize) -> Result<T, String> {
    T::deserialize(b, pos)
}

fn salvage(out_dir: &Path, requested_save: &str, full_err: &str) -> Result<(), Box<dyn Error>> {
    let raw = decompress_save(requested_save)?;
    let b = &raw[..];
    // Database serialization = u64 version, then fields in declaration order.
    let mut pos: usize = 8;

    let mut counts: Vec<(&str, String)> = Vec::new();
    let mut reached_break: Option<String> = None;

    // Helper that deserializes a needed field, writes its debug file, records
    // the entry count, and reports failure (stopping the salvage) on error.
    macro_rules! grab {
        ($field:ident, $file:expr, $key:expr) => {{
            match de(|d| &d.$field, b, &mut pos) {
                Ok(v) => match write_debug(out_dir, $file, &v) {
                    Ok(text) => {
                        counts.push(($key, count_entries(&text).to_string()));
                        true
                    }
                    Err(e) => {
                        reached_break = Some(format!("write {}: {}", stringify!($field), e));
                        false
                    }
                },
                Err(e) => {
                    reached_break = Some(format!("{}: {}", stringify!($field), e));
                    false
                }
            }
        }};
    }
    // Helper that just advances `pos` past a field we don't export.
    macro_rules! skip {
        ($field:ident) => {{
            match de(|d| &d.$field, b, &mut pos) {
                Ok(_) => true,
                Err(e) => {
                    reached_break = Some(format!("{}: {}", stringify!($field), e));
                    false
                }
            }
        }};
    }

    // Fields in serialization order up to the point everything still decodes.
    let _ = (|| -> bool {
        skip!(time)
            && skip!(leagues)
            && skip!(tournaments)
            && grab!(teams, "teams.debug.txt", "teams")
            && skip!(training_plans)
            && skip!(team_research_datas)
            && skip!(networks)
            && skip!(team_policies)
            && skip!(knowledge_bases)
            && skip!(base_network)
            && skip!(item_network)
            && grab!(athletes, "athletes.debug.txt", "athletes")
            && skip!(athlete_policies)
            && skip!(staffs)
            && grab!(matches, "matches.debug.txt", "matches")
            && grab!(match_replays, "match_replays.debug.txt", "match_replays")
            && grab!(league_competitions, "league_competitions.debug.txt", "league_competitions")
            && grab!(tournament_competitions, "tournament_competitions.debug.txt", "tournament_competitions")
            && grab!(year_schedules, "year_schedules.debug.txt", "year_schedules")
    })();

    // Fields the dashboard reads that live *after* the hotfix-changed field and
    // cannot be recovered with this game_core build. Write empty placeholders so
    // the builder's parsers see present-but-empty rather than erroring on a
    // missing file.
    for (file, empty) in [
        ("champion_patch_statistics.debug.txt", "{}\n"),
        ("solo_rank_matches.debug.txt", "DataTable {\n    last_id: 0,\n    data: {},\n}\n"),
        ("match_stats.debug.txt", "DataTable {\n    last_id: 0,\n    data: {},\n}\n"),
        ("champion_info_sheet.debug.txt", "{}\n"),
        ("pre_patch_data.debug.txt", "{}\n"),
        ("champion_action_patch_state.debug.txt", "{}\n"),
    ] {
        fs::write(out_dir.join(file), empty)?;
    }

    let get = |k: &str| counts.iter().find(|(c, _)| *c == k).map(|(_, v)| v.clone()).unwrap_or_else(|| "0".into());
    let save_file = Path::new(requested_save)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| requested_save.to_string());
    let mut rows: Vec<(&str, String)> = vec![
        ("reason", "save_probe".to_string()),
        ("save", requested_save.to_string()),
        ("save_file", save_file),
        ("decode", "partial".to_string()),
        ("salvage", "1".to_string()),
        ("full_load_error", full_err.replace(['\r', '\n', '\t'], " ")),
    ];
    if let Some(b) = &reached_break {
        rows.push(("salvage_break", b.replace(['\r', '\n', '\t'], " ")));
    }
    rows.push(("teams", get("teams")));
    rows.push(("athletes", get("athletes")));
    rows.push(("match_replays", get("match_replays")));
    rows.push(("league_competitions", get("league_competitions")));
    rows.push(("tournament_competitions", get("tournament_competitions")));
    rows.push(("year_schedules", get("year_schedules")));
    write_manifest(&out_dir.join("manifest.tsv"), &rows)?;
    write_manifest(&out_dir.join("save_probe_manifest.tsv"), &rows[1..])?;

    println!(
        "save_probe SALVAGE: save={} teams={} athletes={} match_replays={} (match_stats/champion stats unavailable on this game build)",
        requested_save, get("teams"), get("athletes"), get("match_replays")
    );
    // Exit 0: the dashboard treats a non-zero exit as "no data".
    Ok(())
}

/// Read a TFM2 `.data` save, locate its embedded gzip member (the header is
/// uncompressed), and inflate the Database stream.
fn decompress_save(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let data = fs::read(path)?;
    let gz = data
        .windows(3)
        .position(|w| w == [0x1f, 0x8b, 0x08])
        .ok_or("no gzip member found in save file")?;
    let mut dec = flate2::read::GzDecoder::new(&data[gz..]);
    let mut raw = Vec::new();
    dec.read_to_end(&mut raw)?;
    Ok(raw)
}

// ---------------------------------------------------------------------------
// Shared helpers (from the original probe)
// ---------------------------------------------------------------------------

fn save_name_for_database_load(save: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new(save);
    if path.is_absolute() {
        return path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .ok_or_else(|| format!("save path has no file name: {save}").into());
    }
    Ok(save.to_string())
}

fn write_debug<T: Debug>(out_dir: &Path, file_name: &str, value: &T) -> Result<String, Box<dyn Error>> {
    let text = format!("{value:#?}\n");
    fs::write(out_dir.join(file_name), text.as_bytes())?;
    Ok(text)
}

// mod_save_data 를 대시보드가 읽기 쉬운 형태로 풀어쓴다.
//   구조: mod_save/<namespace>/<key>.txt  (값이 JSON 이면 그대로 .json 확장자)
//   Debug 출력만으로는 긴 JSON 문자열이 이스케이프돼 파싱이 번거로워 평문으로도 뽑는다.
fn dump_mod_save_namespaces(out_dir: &Path, data: &game_core::ModSaveData) -> Result<(), Box<dyn Error>> {
    let ids = data.namespace_ids();
    if ids.is_empty() {
        return Ok(());
    }
    let root = out_dir.join("mod_save");
    fs::create_dir_all(&root)?;
    let mut index = String::from("namespace	key	bytes
");
    for ns in ids {
        let dir = root.join(sanitize_path_component(&ns));
        fs::create_dir_all(&dir)?;
        for key in data.keys(&ns) {
            let Some(value) = data.get_string(&ns, &key) else {
                continue;
            };
            let trimmed = value.trim_start();
            let ext = if trimmed.starts_with('{') || trimmed.starts_with('[') { "json" } else { "txt" };
            let name = format!("{}.{}", sanitize_path_component(&key), ext);
            fs::write(dir.join(&name), value.as_bytes())?;
            index.push_str(&format!("{}	{}	{}
", ns, key, value.len()));
        }
    }
    fs::write(root.join("index.tsv"), index.as_bytes())?;
    Ok(())
}

// 파일명에 쓸 수 없는 문자 제거(네임스페이스/키는 모드가 정하므로 방어)
fn sanitize_path_component(raw: &str) -> String {
    let cleaned: String = raw
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.') { c } else { '_' })
        .collect();
    if cleaned.is_empty() { "_".to_string() } else { cleaned }
}

fn write_manifest(path: &Path, rows: &[(&str, String)]) -> Result<(), Box<dyn Error>> {
    let mut text = String::from("field\tvalue\n");
    for (field, value) in rows {
        text.push_str(field);
        text.push('\t');
        text.push_str(&value.replace(['\r', '\n', '\t'], " "));
        text.push('\n');
    }
    fs::write(path, text)?;
    Ok(())
}

fn count_entries(text: &str) -> usize {
    let Some(data_start) = text.find("data: {") else {
        return 0;
    };
    let body = &text[(data_start + "data: ".len())..];
    count_top_level_map_entries(body)
}

fn count_top_level_map_entries(text: &str) -> usize {
    let mut depth = 0usize;
    let mut count = 0usize;
    let mut in_string = false;
    let mut escape = false;
    let mut started = false;
    for ch in text.chars() {
        if in_string {
            if escape {
                escape = false;
            } else if ch == '\\' {
                escape = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' => in_string = true,
            '{' | '[' | '(' => {
                depth += 1;
                started = true;
            }
            '}' | ']' | ')' => {
                if depth == 0 {
                    return count;
                }
                depth -= 1;
                if started && depth == 0 {
                    return count;
                }
            }
            ':' if depth == 1 => count += 1,
            _ => {}
        }
    }
    count
}
