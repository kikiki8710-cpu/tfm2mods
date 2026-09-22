use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;

// ★0.6.0 (2026-09-16): stable ABI 재작성. 클래식 원본 = src/lib_classic_058.rs.bak.
//   game_core::ItemBuildOverride → 로컬 enum(JSON 문자열) · ServerModContext → StableServerCtx
//   · team.champion_personal_tactics 직접 쓰기 → team_set_json("champion_personal_tactics.<champ>", [..]).
use mod_api_stable::{declare_stable_mod, LogLevel, StableHost, StableMod, StableServerCtx, StableServerExtension};

/// game_core::ItemBuildOverride 의 로컬 미러(serde 유닛 variant = 문자열). 순서·이름은 0.5.8 SDK 기준.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ItemBuildOverride { AD, Magic, AttackSpeed, MagicResistance, Hp, Auto }
impl ItemBuildOverride {
    fn name(self) -> &'static str {
        match self { Self::AD => "AD", Self::Magic => "Magic", Self::AttackSpeed => "AttackSpeed",
                     Self::MagicResistance => "MagicResistance", Self::Hp => "Hp", Self::Auto => "Auto" }
    }
    fn from_name(v: &str) -> Option<Self> { direction_from_str(v) }
}
/// ★0.6.0: `champion_personal_tactics` 값은 **4칸**(4번째 아이템 칸 정식 편입 — 실측 `["Auto","Auto","Auto","Auto"]`).
///   3칸으로 쓰면 서버 스키마가 거부한다(2026-09-16 실측 failed=84). 4번째는 "Auto"(게임 아이템 신경망 추천).
const SLOTS: usize = 4;
fn dirs_vec(d: &[ItemBuildOverride; 3]) -> Vec<&'static str> {
    let mut v: Vec<&'static str> = d.iter().map(|x| x.name()).collect();
    while v.len() < SLOTS { v.push(ItemBuildOverride::Auto.name()); }
    v
}
fn dirs_json(d: &[ItemBuildOverride; 3]) -> String {
    format!("[{}]", dirs_vec(d).iter().map(|n| format!("\"{}\"", n)).collect::<Vec<_>>().join(","))
}
use serde_json::Value;

const MOD_ID: &str = "tfm2_meta_item_delegate";
const DATA_FILE_NAME: &str = "core-item-builds.json";
const TSV_FILE_NAME: &str = "meta_item_builds.tsv";

static BUILD_CACHE: OnceLock<Mutex<BuildCache>> = OnceLock::new();
static LAST_APPLY: OnceLock<Mutex<Option<String>>> = OnceLock::new();
static LAST_ERROR: OnceLock<Mutex<Option<String>>> = OnceLock::new();

#[derive(Clone)]
struct MetaBuilds {
    source: PathBuf,
    modified: Option<SystemTime>,
    size: Option<u64>,
    generated_at: Option<String>,
    rows: HashMap<String, [ItemBuildOverride; 3]>,
}

#[derive(Default)]
struct BuildCache {
    source: Option<PathBuf>,
    modified: Option<SystemTime>,
    size: Option<u64>,
    builds: Option<MetaBuilds>,
}

// ── 서버 확장 ───────────────────────────────────────────────────────────────
//  v0.2.26 구조 재현(원본 DLL 리버스 기반, 2026-07-21):
//    - 클라이언트 UI 확장은 쓰지 않는다(구 "Meta Items" 버튼/현재세트 즉시적용은 제거됨).
//    - on_server_start        = 강제 1회 동기화
//    - after_management_tick  = 5초 스로틀 동기화
//    - before_management_tick = 미구현(원본 vtable slot1 도 빈 함수)
//  동기화 대상은 "플레이어가 조종하는 팀"뿐이다. AI 팀까지 건드리면 밸런스가 바뀐다.
struct MetaItemDelegateServer;

const SYNC_THROTTLE_MS: u64 = 5_000;
static TICK_LOGGED: AtomicBool = AtomicBool::new(false);
static NOTEAM_LOGGED: AtomicBool = AtomicBool::new(false);
static LAST_SYNC_MS: AtomicU64 = AtomicU64::new(0);

impl StableServerExtension for MetaItemDelegateServer {
    fn on_server_start(&self, ctx: &mut StableServerCtx<'_>) {
        let _ = catch_unwind(AssertUnwindSafe(|| sync_meta_items(ctx, "server_start", true)));
    }

    fn after_management_tick(&self, ctx: &mut StableServerCtx<'_>) {
        let _ = catch_unwind(AssertUnwindSafe(|| sync_meta_items(ctx, "after_tick", false)));
    }
}

// 플레이어가 조종하는 팀 찾기.
//  ⚠ `active_multiplayer_player_team_ids` 만 쓰면 **싱글플레이에서 항상 빈 집합**이다
//    (2026-07-21 실측 로그: teams=120 인데 active_mp={} joined_mp={} mp_players=None → no player team found).
//    이름 그대로 멀티 전용 필드다. `multiplayer_joined_team_ids` / `multiplayer_players` 도 마찬가지.
//  ⚠ 다음도 전부 **존재하지 않는다**(컴파일로 확인, 다시 시도하지 말 것):
//    `Database::player_team_id` (메서드·필드), `Database::player_team_ids`,
//    `Team::is_player_team` (메서드·필드 — game_core 의 is_player_team 은 NegotiationContext 필드라 무관).
//  → 정본은 상시 필드 `ServerState::players : HashMap<PlayerId, PlayerData>` 이고 `PlayerData::team_id` 가 답이다.
//    클라측 `ClientDatabase::id`(=클라의 team_id)의 서버쪽 원천이 바로 이것.
fn find_player_teams(ctx: &mut StableServerCtx<'_>) -> Vec<usize> {
    // stable: `player_team_id(player_id)`. 싱글은 로컬 서버의 플레이어 1명 — id 는 호스트 내부값이라
    //   0..PLAYER_ID_SCAN 을 훑어 Some 인 것을 모은다(⬜0.6.0 인게임: 어느 id 가 맞는지 diag 로그로 확정).
    const PLAYER_ID_SCAN: usize = 32;
    let mut ids: Vec<usize> = (0..PLAYER_ID_SCAN).filter_map(|pid| ctx.player_team_id(pid)).collect();
    ids.sort_unstable();
    ids.dedup();
    ids
}

static DIAG_LOGGED: AtomicBool = AtomicBool::new(false);

fn log_team_diagnostics(ctx: &mut StableServerCtx<'_>, source: &str) {
    if DIAG_LOGGED.swap(true, Ordering::Relaxed) {
        return;
    }
    let hits: Vec<(usize, usize)> = (0..32usize).filter_map(|pid| ctx.player_team_id(pid).map(|t| (pid, t))).collect();
    let n_teams = ctx.record_ids(mod_api_stable::RecordKindV1::Team).len();
    log_line(&format!("diag[{source}]: teams={n_teams} player_team_id hits(pid,team)={hits:?}"));
    if let Some((_, t)) = hits.first() {
        let cur = ctx.team_get_json(*t, "champion_personal_tactics").unwrap_or_default();
        log_line(&format!("diag[{source}]: team {t} name={:?} champion_personal_tactics={}", ctx.team_get_string(*t, "name"), &cur[..cur.len().min(400)]));
    }
}

fn now_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn sync_meta_items(ctx: &mut StableServerCtx<'_>, source: &str, force: bool) {
    if !TICK_LOGGED.swap(true, Ordering::Relaxed) {
        log_line(&format!("server: management tick active source={source}"));
    }
    // 스로틀: 매 틱 도는 훅이라 강제 호출이 아니면 5초에 한 번만 일한다.
    let now = now_unix_ms();
    if !force {
        let last = LAST_SYNC_MS.load(Ordering::Relaxed);
        if now.saturating_sub(last) < SYNC_THROTTLE_MS {
            return;
        }
    }
    LAST_SYNC_MS.store(now, Ordering::Relaxed);

    // ※아이템 트리 덤프는 별도 모드 `tfm2_item_tree_probe` 로 분리했다(2026-07-23).
    //   대시보드만 쓰고 이 모드의 개인전술 자동지정은 원하지 않는 사용자를 위해서다.

    log_team_diagnostics(ctx, source);

    let player_teams = find_player_teams(ctx);
    if player_teams.is_empty() {
        // 한 번만 찍는다(5초마다 도는 훅이라 매번 찍으면 로그가 폭발한다).
        // 단 한 번이라도 팀을 찾은 뒤 다시 못 찾게 되면 재차 찍히도록 아래에서 플래그를 되돌린다.
        if !NOTEAM_LOGGED.swap(true, Ordering::Relaxed) {
            log_line(&format!("server: no player team found source={source}"));
        }
        return;
    }
    if NOTEAM_LOGGED.swap(false, Ordering::Relaxed) {
        log_line(&format!(
            "server: player team recovered source={source}, teams={player_teams:?}"
        ));
    }

    let Some(builds) = load_builds() else {
        return;
    };

    for team_id in player_teams {
        // 현재 맵(JSON 객체 champ -> [3 문자열]) 읽기 — 값이 같으면 건드리지 않는다(세이브 dirty 방지).
        let cur_json = ctx.team_get_json(team_id, "champion_personal_tactics").unwrap_or_else(|| "{}".into());
        let mut cur: Value = serde_json::from_str(&cur_json).unwrap_or(Value::Object(Default::default()));
        if !cur.is_object() { cur = Value::Object(Default::default()); }
        let before_len = cur.as_object().map(|o| o.len()).unwrap_or(0);
        let mut changed = 0usize;
        let mut failed = 0usize;
        // ★0.6.0 실측(2026-09-16): `champion_personal_tactics.<champ>` 경로로 **새 키**를 넣는 쓰기는
        //   호스트가 거부한다(failed=84 · 기존 키 10개만 통과). ⟹ 맵 전체를 병합해 한 번에 쓴다.
        for (champion, directions) in builds.rows.iter() {
            let want: Vec<Value> = dirs_vec(directions).into_iter().map(|n| Value::String(n.to_string())).collect();
            let same = cur.get(champion).and_then(|v| v.as_array()).map(|a| *a == want).unwrap_or(false);
            if same { continue; }
            if let Some(o) = cur.as_object_mut() { o.insert(champion.clone(), Value::Array(want)); }
            changed += 1;
        }
        if changed > 0 {
            let body = cur.to_string();
            if !ctx.team_set_json(team_id, "champion_personal_tactics", &body) {
                failed = changed; changed = 0;
                log_error_once(format!("apply: team_set_json(맵 전체) 거부 team={team_id} bytes={}", body.len()));
            }
        }
        if changed > 0 || failed > 0 {
            let after = ctx.team_get_json(team_id, "champion_personal_tactics").unwrap_or_default();
            let after_len = serde_json::from_str::<Value>(&after).ok().and_then(|v| v.as_object().map(|o| o.len())).unwrap_or(0);
            log_line(&format!("apply: target team name={:?}", ctx.team_get_string(team_id, "name")));
            log_line(&format!(
                "apply: team {team_id} personal item defaults changed={changed} failed={failed}, map_size={before_len}->{after_len}"
            ));
            log_line(&format!(
                "server: auto synced meta item defaults source={source}, team={team_id}, changed={changed}"
            ));
        }
    }
}

fn load_builds() -> Option<MetaBuilds> {
    let Some(path) = find_data_file() else {
        log_error_once("data: core-item-builds.json not found".to_string());
        return None;
    };

    let metadata = fs::metadata(&path).ok();
    let modified = metadata.as_ref().and_then(|meta| meta.modified().ok());
    let size = metadata.as_ref().map(|meta| meta.len());
    let cache = BUILD_CACHE.get_or_init(|| Mutex::new(BuildCache::default()));
    if let Ok(mut guard) = cache.lock() {
        if guard.source.as_ref() == Some(&path)
            && guard.modified == modified
            && guard.size == size
        {
            return guard.builds.clone();
        }

        match fs::read_to_string(&path)
            .map_err(|err| err.to_string())
            .and_then(|text| parse_builds(&text, path.clone(), modified, size))
        {
            Ok(builds) => {
                log_line(format!(
                    "data: loaded {} rows from {} bytes={}",
                    builds.rows.len(),
                    builds.source.display(),
                    builds.size.unwrap_or(0),
                ));
                guard.source = Some(path);
                guard.modified = modified;
                guard.size = size;
                guard.builds = Some(builds.clone());
                clear_last_error();
                Some(builds)
            }
            Err(err) => {
                log_error_once(format!("data: parse failed {}: {err}", path.display()));
                guard.source = Some(path);
                guard.modified = modified;
                guard.size = size;
                guard.builds = None;
                None
            }
        }
    } else {
        None
    }
}

fn parse_builds(
    text: &str,
    source: PathBuf,
    modified: Option<SystemTime>,
    size: Option<u64>,
) -> Result<MetaBuilds, String> {
    if source.file_name().and_then(|name| name.to_str()) == Some(TSV_FILE_NAME) {
        return parse_tsv_builds(text, source, modified, size);
    }

    let root: Value = serde_json::from_str(text).map_err(|err| err.to_string())?;
    let generated_at = root
        .get("generatedAt")
        .and_then(Value::as_str)
        .map(str::to_string);
    let latest_patch = root.get("latestPatch").and_then(Value::as_str);
    let min_games = root
        .get("rules")
        .and_then(|rules| rules.get("recommendedMinGames"))
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let builds = root
        .get("builds")
        .and_then(Value::as_object)
        .ok_or_else(|| "missing builds object".to_string())?;

    let mut rows = HashMap::new();

    for scope in ["tournament", "solo"] {
        if let Some(patch) = latest_patch
            .and_then(|patch| builds.get(scope).and_then(|scope_value| scope_value.get(patch)))
        {
            collect_patch_rows(patch, &mut rows, min_games);
        }
    }

    for scope in ["tournament", "solo"] {
        if let Some(patch) = builds.get(scope).and_then(|scope_value| scope_value.get("all")) {
            collect_patch_rows(patch, &mut rows, min_games);
        }
    }

    for scope in ["tournament", "solo"] {
        if let Some(scope_object) = builds.get(scope).and_then(Value::as_object) {
            for (patch_key, patch) in scope_object {
                if Some(patch_key.as_str()) == latest_patch || patch_key == "all" {
                    continue;
                }
                collect_patch_rows(patch, &mut rows, min_games);
            }
        }
    }

    if rows.is_empty() {
        return Err("no usable meta item data found".to_string());
    }

    Ok(MetaBuilds {
        source,
        modified,
        size,
        generated_at,
        rows,
    })
}

fn parse_tsv_builds(
    text: &str,
    source: PathBuf,
    modified: Option<SystemTime>,
    size: Option<u64>,
) -> Result<MetaBuilds, String> {
    let mut rows = HashMap::new();
    for (line_number, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let columns: Vec<&str> = line.split('\t').collect();
        if columns.len() < 2 {
            return Err(format!("line {} has no item directions", line_number + 1));
        }

        let champion = columns[0].trim();
        if champion.is_empty() {
            return Err(format!("line {} has an empty champion id", line_number + 1));
        }

        let mut directions = [ItemBuildOverride::Auto; 3];
        for index in 0..3 {
            if let Some(value) = columns.get(index + 1).map(|value| value.trim()) {
                if value.is_empty() {
                    continue;
                }
                directions[index] = direction_from_token(value)
                    .ok_or_else(|| format!("line {} has unsupported direction {value}", line_number + 1))?;
            }
        }
        rows.insert(champion.to_string(), directions);
    }

    if rows.is_empty() {
        return Err("no usable TSV item data found".to_string());
    }

    Ok(MetaBuilds {
        source,
        modified,
        size,
        generated_at: None,
        rows,
    })
}

fn collect_patch_rows(
    patch: &Value,
    rows: &mut HashMap<String, [ItemBuildOverride; 3]>,
    min_games: u64,
) {
    let Some(champions) = patch.as_object() else {
        return;
    };

    for (champion, by_position) in champions {
        if rows.contains_key(champion) {
            continue;
        }
        if let Some(directions) = best_champion_directions(by_position, min_games) {
            rows.insert(champion.clone(), directions);
        }
    }
}

fn best_champion_directions(value: &Value, min_games: u64) -> Option<[ItemBuildOverride; 3]> {
    const POSITION_ORDER: [&str; 6] = ["all", "top", "jungle", "mid", "bot", "support"];

    for position in POSITION_ORDER {
        if let Some(directions) = value
            .get(position)
            .and_then(|position| best_position_directions(position, min_games))
        {
            return Some(directions);
        }
    }

    value.as_object().and_then(|positions| {
        positions
            .values()
            .find_map(|position| best_position_directions(position, min_games))
    })
}

fn best_position_directions(value: &Value, min_games: u64) -> Option<[ItemBuildOverride; 3]> {
    value
        .get("core3")
        .and_then(|core| best_entry_directions(core, min_games))
        .or_else(|| {
            value
                .get("core2")
                .and_then(|core| best_entry_directions(core, min_games))
        })
}

fn best_entry_directions(value: &Value, min_games: u64) -> Option<[ItemBuildOverride; 3]> {
    let entries = value.as_array()?;
    entries
        .iter()
        .find_map(|entry| {
            if entry_games(entry) >= min_games {
                entry_directions(entry)
            } else {
                None
            }
        })
        .or_else(|| entries.iter().find_map(entry_directions))
}

fn entry_games(value: &Value) -> u64 {
    value.get("games").and_then(Value::as_u64).unwrap_or(0)
}

fn entry_directions(value: &Value) -> Option<[ItemBuildOverride; 3]> {
    value
        .get("directions")
        .and_then(parse_direction_array)
        .or_else(|| value.get("itemIds").and_then(parse_item_id_array))
}

fn parse_direction_array(value: &Value) -> Option<[ItemBuildOverride; 3]> {
    let values = value.as_array()?;
    let mut directions = [ItemBuildOverride::Auto; 3];
    for (index, raw) in values.iter().take(3).enumerate() {
        let direction = raw.as_str().and_then(direction_from_str)?;
        directions[index] = direction;
    }
    Some(directions)
}

fn parse_item_id_array(value: &Value) -> Option<[ItemBuildOverride; 3]> {
    let values = value.as_array()?;
    let mut directions = [ItemBuildOverride::Auto; 3];
    for (index, raw) in values.iter().take(3).enumerate() {
        let item_id = raw.as_u64()?;
        directions[index] = direction_from_item_id(item_id)?;
    }
    Some(directions)
}

fn direction_from_token(value: &str) -> Option<ItemBuildOverride> {
    direction_from_str(value).or_else(|| {
        value
            .parse::<u64>()
            .ok()
            .and_then(direction_from_item_id)
    })
}

fn direction_from_item_id(value: u64) -> Option<ItemBuildOverride> {
    match value {
        0..=4 => Some(ItemBuildOverride::AD),
        5..=9 => Some(ItemBuildOverride::AttackSpeed),
        // Teamfight Manager 2 0.4.11 has no separate armor item override.
        10..=14 => Some(ItemBuildOverride::Hp),
        15..=19 => Some(ItemBuildOverride::MagicResistance),
        20..=24 => Some(ItemBuildOverride::Magic),
        25..=29 => Some(ItemBuildOverride::Hp),
        _ => None,
    }
}

fn direction_from_str(value: &str) -> Option<ItemBuildOverride> {
    match value {
        "AD" => Some(ItemBuildOverride::AD),
        "Magic" => Some(ItemBuildOverride::Magic),
        "AttackSpeed" => Some(ItemBuildOverride::AttackSpeed),
        "MagicResistance" => Some(ItemBuildOverride::MagicResistance),
        "Hp" => Some(ItemBuildOverride::Hp),
        "Auto" => Some(ItemBuildOverride::Auto),
        // Teamfight Manager 2 exposes no separate armor/defense item override in 0.4.11.
        "Defense" => Some(ItemBuildOverride::Hp),
        _ => None,
    }
}

fn find_data_file() -> Option<PathBuf> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mod_dir = cwd.join("mods").join(MOD_ID);
    let mut candidates = vec![
        mod_dir.join(DATA_FILE_NAME),
        cwd.join("TFM2.gg")
            .join("resources")
            .join("app")
            .join("tfm2_meta_dashboard")
            .join("data")
            .join(DATA_FILE_NAME),
        cwd.join("resources")
            .join("app")
            .join("tfm2_meta_dashboard")
            .join("data")
            .join(DATA_FILE_NAME),
        cwd.join("tfm2_meta_dashboard")
            .join("data")
            .join(DATA_FILE_NAME),
        mod_dir.join("data").join(DATA_FILE_NAME),
    ];

    if let Some(dll_dir) = loaded_module_dir() {
        candidates.push(dll_dir.join(DATA_FILE_NAME));
        candidates.push(dll_dir.join("data").join(DATA_FILE_NAME));
    }

    candidates.push(mod_dir.join("data").join(TSV_FILE_NAME));
    if let Some(dll_dir) = loaded_module_dir() {
        candidates.push(dll_dir.join("data").join(TSV_FILE_NAME));
    }

    candidates.into_iter().find(|path| path.is_file())
}

#[cfg(windows)]
fn loaded_module_dir() -> Option<PathBuf> {
    use std::ffi::{c_void, OsString};
    use std::os::windows::ffi::OsStringExt;
    use std::ptr::null_mut;

    const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 0x00000002;
    const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 0x00000004;

    #[link(name = "kernel32")]
    extern "system" {
        fn GetModuleHandleExW(
            dw_flags: u32,
            lp_module_name: *const u16,
            ph_module: *mut *mut c_void,
        ) -> i32;
        fn GetModuleFileNameW(h_module: *mut c_void, lp_filename: *mut u16, n_size: u32) -> u32;
    }

    let mut module = null_mut();
    let address = loaded_module_dir as *const () as *const u16;
    let ok = unsafe {
        GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            address,
            &mut module,
        )
    };
    if ok == 0 || module.is_null() {
        return None;
    }

    let mut buffer = vec![0u16; 32768];
    let len = unsafe { GetModuleFileNameW(module, buffer.as_mut_ptr(), buffer.len() as u32) };
    if len == 0 {
        return None;
    }
    buffer.truncate(len as usize);
    PathBuf::from(OsString::from_wide(&buffer))
        .parent()
        .map(Path::to_path_buf)
}

#[cfg(not(windows))]
fn loaded_module_dir() -> Option<PathBuf> {
    None
}

fn mod_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("mods")
        .join(MOD_ID)
}

fn log_path() -> PathBuf {
    mod_dir().join("debug.log")
}

fn log_line(message: impl AsRef<str>) {
    let message = message.as_ref();
    let path = log_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(file, "{message}");
    }
}

fn log_error_once(message: String) {
    let state = LAST_ERROR.get_or_init(|| Mutex::new(None));
    if let Ok(mut guard) = state.lock() {
        if guard.as_ref() == Some(&message) {
            return;
        }
        *guard = Some(message.clone());
    }
    log_line(message);
}

fn clear_last_error() {
    if let Some(state) = LAST_ERROR.get() {
        if let Ok(mut guard) = state.lock() {
            *guard = None;
        }
    }
}

fn remember_apply_signature(signature: String) -> bool {
    let state = LAST_APPLY.get_or_init(|| Mutex::new(None));
    if let Ok(mut guard) = state.lock() {
        if guard.as_ref() == Some(&signature) {
            return false;
        }
        *guard = Some(signature);
        true
    } else {
        true
    }
}

fn init(host: &StableHost) -> StableMod {
    let v = host.game_version();
    log_line(format!("mod: init 0.3.0 server item delegate (stable, game {}.{}.{} host_abi={})", v.major, v.minor, v.patch, host.abi_level()));
    host.log(LogLevel::Info, "tfm2_meta_item_delegate (stable 0.6.0)");
    let mut decl = StableMod::new(MOD_ID);
    decl.set_server_extension(MetaItemDelegateServer);
    decl
}

declare_stable_mod!(init);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_supported_direction_names() {
        assert!(matches!(direction_from_str("AD"), Some(ItemBuildOverride::AD)));
        assert!(matches!(
            direction_from_str("AttackSpeed"),
            Some(ItemBuildOverride::AttackSpeed)
        ));
        assert!(matches!(
            direction_from_str("MagicResistance"),
            Some(ItemBuildOverride::MagicResistance)
        ));
        assert!(matches!(direction_from_str("Defense"), Some(ItemBuildOverride::Hp)));
    }

    #[test]
    fn parses_latest_patch_all_position_core3() {
        let text = r#"{
            "generatedAt": "test",
            "latestPatch": "2026.0.0",
            "builds": {
                "tournament": {
                    "2026.0.0": {
                        "hunter": {
                            "all": {
                                "core3": [
                                    { "directions": ["AttackSpeed", "AD", "Defense"] }
                                ]
                            }
                        }
                    }
                }
            }
        }"#;

        let builds = parse_builds(text, PathBuf::from("core-item-builds.json"), None, None).unwrap();
        let directions = builds.rows.get("hunter").unwrap();
        assert!(matches!(directions[0], ItemBuildOverride::AttackSpeed));
        assert!(matches!(directions[1], ItemBuildOverride::AD));
        assert!(matches!(directions[2], ItemBuildOverride::Hp));
    }

    #[test]
    fn uses_item_ids_when_directions_are_missing() {
        let text = r#"{
            "latestPatch": "2026.0.0",
            "rules": { "recommendedMinGames": 5 },
            "builds": {
                "tournament": {
                    "2026.0.0": {
                        "fighter": {
                            "all": {
                                "core3": [
                                    { "itemIds": [4, 14, 24], "games": 6 }
                                ]
                            }
                        }
                    }
                }
            }
        }"#;

        let builds = parse_builds(text, PathBuf::from("core-item-builds.json"), None, None).unwrap();
        let directions = builds.rows.get("fighter").unwrap();
        assert!(matches!(directions[0], ItemBuildOverride::AD));
        assert!(matches!(directions[1], ItemBuildOverride::Hp));
        assert!(matches!(directions[2], ItemBuildOverride::Magic));
    }

    #[test]
    fn parses_tsv_fallback_rows() {
        let text = "pyromancer\tMagic\tMagic\tMagic\nfighter\t4\t14\t14\n";
        let builds = parse_builds(text, PathBuf::from(TSV_FILE_NAME), None, None).unwrap();

        let pyromancer = builds.rows.get("pyromancer").unwrap();
        assert!(matches!(pyromancer[0], ItemBuildOverride::Magic));

        let fighter = builds.rows.get("fighter").unwrap();
        assert!(matches!(fighter[0], ItemBuildOverride::AD));
        assert!(matches!(fighter[1], ItemBuildOverride::Hp));
        assert!(matches!(fighter[2], ItemBuildOverride::Hp));
    }
}
