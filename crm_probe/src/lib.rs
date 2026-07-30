// crm_test — 새 공식 SDK 검증: 원본과 동일한 안전 API(data.db()->db.teams)로 팀/선수 읽기.
use mod_api::*;
use std::fs;
use std::io::Write as _;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

const MOD_ID: &str = "crm_probe";
static F: AtomicU64 = AtomicU64::new(0);
static DONE: AtomicU64 = AtomicU64::new(0);

fn log(s: &str) {
    let dir = Path::new("mods").join(MOD_ID);
    let _ = fs::create_dir_all(&dir);
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(dir.join("verify.txt")) {
        let _ = f.write_all(format!("{}\n", s).as_bytes());
        let _ = f.flush();
    }
}

struct T;
impl ModExtension for T {
    fn post_update(&self, scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        let Scene::InGame { data } = scene else { return; };
        let n = F.fetch_add(1, Ordering::Relaxed);
        if n < 180 { return; }
        if DONE.swap(1, Ordering::Relaxed) != 0 { return; }

        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let db = data.db();
            log(&format!("=== 새 SDK 검증 === player_team_id={} teams={} athletes={} match_replays={} matches={} leagues={}",
                data.player_team_id(), db.teams.len(), db.athletes.len(), db.match_replays.len(), db.matches.len(), db.leagues.len()));
            log("--- teams (최대 10) ---");
            for (id, team) in db.teams.iter().take(10) {
                log(&format!("  team[{}] name=\"{}\" manager=\"{}\" league={} fans={}",
                    id, team.name, team.manager_name, team.league_id, team.fan_count));
            }
            log("--- athletes (최대 8) ---");
            for (id, a) in db.athletes.iter().take(8) {
                log(&format!("  athlete[{}] name=\"{}\"", id, a.name));
            }
            if let Some(t) = data.player_team() {
                log(&format!("--- player_team: \"{}\" ---", t.name));
            }
            log("=== 검증 끝 (이름이 제대로 나오면 새 SDK로 해결!) ===");
        }));
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    std::panic::set_hook(Box::new(|info| {
        let dir = Path::new("mods").join(MOD_ID);
        let _ = fs::create_dir_all(&dir);
        let _ = fs::write(dir.join("panic.txt"), format!("PANIC: {}\n", info).as_bytes());
    }));
    log("INIT crm_test (new SDK verify)");
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(T);
    reg
}

declare_mod!(init);
