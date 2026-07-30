//! TFM2_Meta_Dashboard 의 인게임 부분 — 아이템 상하위 관계(아이템 트리)를 세이브에 기록한다.
//!
//! 왜 여기에 있나:
//!   원래 이 기능은 `tfm2_meta_item_delegate` 안에 있었다. 하지만 delegate 는 "챔피언 개인전술을
//!   메타 통계로 자동 지정"하는 **게임플레이 개입** 모드다. 대시보드만 쓰고 싶은 사람은 그 자동
//!   지정을 원하지 않을 수 있는데, 트리가 delegate 안에 있으면 둘을 떼어낼 수 없었다.
//!   → 트리 덤프를 **대시보드 모드 폴더 자체의 dll** 로 옮겼다. 대시보드는 원래 dll 이 없는
//!     껍데기 모드였으므로, 모드 개수를 늘리지 않고 "대시보드를 켜면 트리도 따라오는" 구조가 된다.
//!
//! 무엇을 하나:
//!   아이템의 상하위 관계(next_tier/previous_tier)는 **게임 런타임에만 존재**한다. 세이브의
//!   리플레이에는 "그 시점에 들고 있던 아이템"만 남아, 대시보드는 재료템과 완성템을 구분할 수
//!   없었다(모드 아이템은 item_setting/i18n 어디에도 tier 가 없어 더 심하다).
//!   그래서 `ItemSetting::get_item_list()` 로 전 아이템을 훑어 `Database::mod_save_data` 의
//!   모드 전용 네임스페이스에 JSON 으로 저장한다. 게임이 세이브를 쓸 때 함께 저장되고,
//!   save_probe → build_meta_data.py 가 읽어 간다.
//!
//! 안전성:
//!   - 게임 상태를 **읽기만** 한다. 쓰는 곳은 모드 전용 저장공간(mod_save_data)뿐이라
//!     시뮬레이션·밸런스에 영향이 없다.
//!   - 메모리 스캔·오프셋 하드코딩을 쓰지 않는다(전부 공개 SDK API) → 게임 패치에 영향받지 않음.
//!   - 서버 확장 훅은 catch_unwind 로 감싼다(패닉이 게임 콜스택으로 새면 UB).
//!
//! 비용:
//!   내용이 그대로면 쓰지 않는다(FNV-1a64 해시 비교). 매 틱 도는 훅이라 변경 시에만 write.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::SystemTime;

use game_core::Database;
use mod_api::*;
use serde_json::Value;

// ★대시보드 모드 폴더의 dll 이므로 MOD_ID = 대시보드 mod_id (폴더명과 일치해야 로더가 인식).
const MOD_ID: &str = "TFM2_Meta_Dashboard";

// mod_save_data 는 모드별 네임스페이스라 MOD_ID 가 곧 네임스페이스가 된다(= TFM2_Meta_Dashboard).
//   빌더(build_meta_data.py load_item_tree)는 네임스페이스를 **순회**하며 item_tree.json 을 찾으므로
//   구 delegate 네임스페이스로 저장된 기존 세이브도 그대로 읽힌다(빌더 수정 불필요).
const TREE_KEY: &str = "item_tree";
const TREE_HASH_KEY: &str = "item_tree_hash";
const TREE_FORMAT: u32 = 1;

// 매 틱 도는 훅이라 스로틀을 둔다(해시 비교 전에 JSON 을 만드는 비용도 아깝다).
const SCAN_THROTTLE_MS: u64 = 5_000;
static LAST_SCAN_MS: AtomicU64 = AtomicU64::new(0);
static TICK_LOGGED: AtomicBool = AtomicBool::new(false);

struct ItemTreeProbeServer;

impl ModServerExtension for ItemTreeProbeServer {
    fn on_server_start(&self, ctx: &mut ServerModContext<'_>) {
        let _ = catch_unwind(AssertUnwindSafe(|| scan(ctx, "server_start", true)));
    }

    fn after_management_tick(&self, ctx: &mut ServerModContext<'_>) {
        let _ = catch_unwind(AssertUnwindSafe(|| scan(ctx, "after_tick", false)));
    }
}

fn now_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn scan(ctx: &mut ServerModContext<'_>, source: &str, force: bool) {
    if !TICK_LOGGED.swap(true, Ordering::Relaxed) {
        log_line(&format!("server: management tick active source={source}"));
    }
    if !force {
        let now = now_unix_ms();
        if now.saturating_sub(LAST_SCAN_MS.load(Ordering::Relaxed)) < SCAN_THROTTLE_MS {
            return;
        }
        LAST_SCAN_MS.store(now, Ordering::Relaxed);
    } else {
        LAST_SCAN_MS.store(now_unix_ms(), Ordering::Relaxed);
    }
    dump_item_tree(ctx.database);
}

fn dump_item_tree(db: &mut Database) {
    let items = db.item_setting.get_item_list();
    if items.is_empty() {
        return;
    }

    // key -> { tier, price, icon, category, next[], prev[] }
    let mut map = serde_json::Map::new();
    for item in items.iter() {
        let mut row = serde_json::Map::new();
        row.insert("tier".into(), Value::from(item.tier() as u64));
        row.insert("price".into(), Value::from(item.price() as u64));
        row.insert("icon".into(), Value::from(item.icon().clone()));
        row.insert("category".into(), Value::from(format!("{:?}", item.category())));
        row.insert(
            "next".into(),
            Value::Array(item.next_tier().iter().map(|k| Value::from(k.clone())).collect()),
        );
        row.insert(
            "prev".into(),
            Value::Array(item.previous_tier().iter().map(|k| Value::from(k.clone())).collect()),
        );
        map.insert(item.key().clone(), Value::Object(row));
    }

    let count = map.len() as u64;
    let mut root = serde_json::Map::new();
    root.insert("format".into(), Value::from(TREE_FORMAT));
    root.insert("count".into(), Value::from(count));
    root.insert("items".into(), Value::Object(map));
    let Ok(text) = serde_json::to_string(&Value::Object(root)) else {
        return;
    };

    // 내용 무변경이면 write 생략(세이브 dirty 방지)
    let hash = fnv1a64(text.as_bytes()).to_string();
    if db.mod_save_data.get_string(MOD_ID, TREE_HASH_KEY).as_deref() == Some(hash.as_str()) {
        return;
    }
    db.mod_save_data.set_string(MOD_ID, TREE_KEY, &text);
    db.mod_save_data.set_string(MOD_ID, TREE_HASH_KEY, &hash);
    log_line(&format!(
        "tree: wrote item_tree items={} bytes={} hash={hash}",
        count,
        text.len()
    ));
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

// ── 로그 ────────────────────────────────────────────────────────────────────
//  게임 exe 기준으로 경로를 도출한다(설치 경로 하드코딩 금지).
fn game_root() -> Option<PathBuf> {
    std::env::current_exe().ok()?.parent().map(|p| p.to_path_buf())
}

fn mod_dir() -> Option<PathBuf> {
    Some(game_root()?.join("mods").join(MOD_ID))
}

fn log_line(message: impl AsRef<str>) {
    let message = message.as_ref();
    let Some(dir) = mod_dir() else { return };
    let _ = fs::create_dir_all(&dir);
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join("debug.log"))
    {
        let _ = writeln!(file, "{message}");
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    log_line("mod: init dashboard item tree probe");

    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_server_extension(ItemTreeProbeServer);
    reg
}

declare_mod!(init);
