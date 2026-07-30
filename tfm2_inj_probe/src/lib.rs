//! tfm2_inj_probe v6 — 템플릿 가산-주입 검증 (가장 우아한 방식의 핵심).
//!  on_init / pre_update 에서:
//!   Q1 main NodeTemplate 를 get_mut 으로 잡을 수 있나 (어느 키?)
//!   Q2 내 모드 .ui 조각이 get::<NodeTemplate> 로 로드되나 (비-override 에셋 auto-load?)
//!   Q3 main 템플릿에 노드 append → 라이브 GameUI.root 에 반영되나 (언제?)
#![allow(dead_code, unused_imports, unused_variables)]
use mod_api::*;
use std::fs;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const LOG: &str = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_inj_probe\probe_log.txt";
fn append(s: &str) {
    use std::io::Write;
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(LOG) { let _ = f.write_all(s.as_bytes()); }
}
fn nfind<'a>(n: &'a Node, id: &str) -> Option<&'a Node> {
    if n.id == id { return Some(n); }
    for c in n.child.iter() { if let Some(f) = nfind(c, id) { return Some(f); } }
    None
}

const MAIN_KEYS: [&str; 6] = [
    "asset/base/ui/layout/main", "base/ui/layout/main", "ui/layout/main", "main",
    "asset/tfm2_item_editor/ui/layout/main", "asset/Scrim_Probe/ui/layout/main",
];
const FRAG_KEYS: [&str; 3] = [
    "asset/tfm2_inj_probe/ui/layout/inject", "asset/tfm2_inj_probe/ui/inject", "tfm2_inj_probe/ui/layout/inject",
];

// 어떤 main 키가 get_mut 으로 잡히면 그 템플릿 첫 자식 clone→id 변경→append. 반영여부는 post_update 가 검사.
fn try_append(assets: &mut Assets, marker: &str, phase: &str) {
    for key in MAIN_KEYS {
        if let Some(mt) = assets.get_mut::<NodeTemplate>(key) {
            if let Some(c0) = mt.child.get(0).cloned() {
                let mut c = c0;
                c.id = marker.into();
                mt.child.push(c);
                append(&format!("  [{}] APPEND ok via '{}' (child now {})\n", phase, key, mt.child.len()));
                return;
            } else {
                append(&format!("  [{}] '{}' Some but no child\n", phase, key));
            }
        }
    }
    append(&format!("  [{}] APPEND failed (no main template via get_mut)\n", phase));
}
fn log_reach(assets: &Assets, phase: &str) {
    for key in MAIN_KEYS {
        let r = assets.get::<NodeTemplate>(key).map(|t| (t.id.clone(), t.name.clone(), t.child.len()));
        append(&format!("  [{}] main get '{}': {:?}\n", phase, key, r));
    }
    for key in FRAG_KEYS {
        let r = assets.get::<NodeTemplate>(key).map(|t| (t.name.clone(), t.child.len()));
        append(&format!("  [{}] frag get '{}': {:?}\n", phase, key, r));
    }
}

static FRAME: AtomicUsize = AtomicUsize::new(0);
static PRE_DONE: AtomicBool = AtomicBool::new(false);

struct Probe;
impl ModExtension for Probe {
    fn on_init(&self, _s: &mut Scene, _u: &mut GameUI, assets: &mut Assets) {
        let _ = fs::write(LOG, "=== tfm2_inj_probe v6 (template inject) ===\n[ON_INIT]\n");
        log_reach(assets, "on_init");
        try_append(assets, "ZZ_oninit", "on_init");
    }
    fn pre_update(&self, _s: &mut Scene, _u: &mut GameUI, assets: &mut Assets, _dt: f32) {
        if PRE_DONE.swap(true, Ordering::Relaxed) { return; }
        append("[PRE_UPDATE first]\n");
        log_reach(assets, "pre");
        try_append(assets, "ZZ_pre", "pre");
    }
    fn post_update(&self, _s: &mut Scene, ui: &mut GameUI, _a: &mut Assets, _dt: f32) {
        let f = FRAME.fetch_add(1, Ordering::Relaxed);
        if f % 120 == 0 && f <= 1200 {
            let oi = nfind(&ui.root, "ZZ_oninit").is_some();
            let pr = nfind(&ui.root, "ZZ_pre").is_some();
            append(&format!("  post f{}: ZZ_oninit_live={} ZZ_pre_live={} (root_total_kids={})\n", f, oi, pr, ui.root.child.len()));
        }
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new("tfm2_inj_probe");
    reg.set_extension(Probe);
    reg
}
declare_mod!(init);
