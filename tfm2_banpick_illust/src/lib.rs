//! tfm2_banpick_illust — 밴픽 화면에서 픽이 확정된 슬롯 배경에 챔피언 스플래시 일러스트 표시.
//! ===========================================================================
//! 창작마당에서 내려간 daram2 "Pick Ban Plus"(banpick_view_plus)의 일러스트 기능만
//! 0.5.1 UI 에 맞춰 단독 모드로 재구현.
//! 하드코딩 RVA 는 0개지만, 이미지 소스 교체만은 **구조체 오프셋 raw 접근**을 쓴다
//! (SDK 다운캐스트가 이 게임에선 항상 실패 — 사유·오프셋 근거 = `raw.rs`).
//!
//! 동작:
//!   1) `.ui` 오버라이드가 blue/red pick_slot 의 상태 그룹 3개(`#wait`/`#in_turn`/`#done`)
//!      최상단에 `#bpi_illust`(image) + `#bpi_dim`(color) 을 심어둔다.
//!      최상단 선언 = 가장 뒤에 그려짐 = 배경.
//!   2) 이 dll 이 매 프레임 post_update 에서 슬롯 서브트리를 훑어 챔피언 스프라이트
//!      (`.../aseprite_resources/champions/<id>#sheet`)를 찾아 id 를 얻고,
//!      `#bpi_illust` 의 source 를 그 챔프의 일러스트 키로 설정 + visible.
//!
//! ※ 원본 모드는 노드를 안 건드리고 `post_render` 에서 `RenderCommand::Sprite` 를 직접
//!    push 하는 방식이었다(원본 dll 분석). 우리는 `.ui` 노드 + 오프셋 write 로 가는데,
//!    Sprite 에는 width/height 가 없어 크기 맞춤이 번거로운 반면 노드는 `.ui` 크기로
//!    자동 스케일되기 때문이다.
//!
//! 설정 = `mods\tfm2_banpick_illust\tfm2_banpick_illust.cfg` (없으면 자동 생성).
//! ===========================================================================

use mod_api::*;
use std::sync::atomic::{AtomicU32, Ordering};

mod config;
mod keys;
mod modchamps;
mod names_gen;
mod patchviz;
mod raw;
mod showcase;

pub(crate) const MOD_ID: &str = "tfm2_banpick_illust";

// build_inj.ps1 신원 검증용 — dll 안에 lib.rs 절대경로 문자열 필요(stale/타모드 차단).
// 패닉 사이트 경로가 최적화로 빠지는 케이스 대비, export 함수로 명시적으로 박는다
// (#[used] static은 링커가 문자열까지 못 지킴 — showcase §10 함정).
#[no_mangle]
pub extern "C" fn tfm2_banpick_illust_src_id() -> *const u8 {
    concat!(file!(), "\0").as_bytes().as_ptr()
}

/// 일러스트 에셋 키 접두사 = **이 모드 자신의 네임스페이스**.
///
/// ⚠ 처음엔 `asset/base/ui/banpick/illust/` 를 "생태계 표준 키" 로 보고 썼으나, 원본 dll
/// 분석 결과 그 키는 원본 어디에도 없었다. base 에 없는 키를 override 하는 건 무효일 수
/// 있어 자기 네임스페이스로 전환(모드 자체 에셋은 항상 로드 가능).
/// 타 아트 모드(Touhou 등)는 각 모드의 `asset/<mod_id>/BanPickIllust/<champ>` 를 스캔해
/// 쓴다(원본의 2_Mod 티어) — `keys.rs`.
const ILLUST_KEY: &str = "asset/tfm2_banpick_illust/illust/";

/// 우리가 .ui 로 심은 노드 id.
const N_ILLUST: &str = "bpi_illust";
const N_DIM: &str = "bpi_dim";

/// 픽 슬롯 컨테이너 (banpick/layout.ui `#blue_picks` / `#red_picks`).
const BLUE_PICKS: &str = "blue_picks";
const RED_PICKS: &str = "red_picks";

// ── 게임 exe 기준 모드 폴더 (설치위치 무관 — 경로 하드코딩 금지 규칙) ──
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleFileNameW(h: usize, buf: *mut u16, n: u32) -> u32;
}

pub(crate) fn mod_dir() -> Option<String> {
    let mut buf = [0u16; 512];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), 512) } as usize;
    if n == 0 || n >= 512 {
        return None;
    }
    let exe = String::from_utf16_lossy(&buf[..n]);
    let dir = std::path::Path::new(&exe).parent()?;
    Some(format!("{}\\mods\\{}", dir.display(), MOD_ID))
}

// ── 노드 탐색 헬퍼 ────────────────────────────────────────────────────────
fn child<'a>(n: &'a Node, id: &str) -> Option<&'a Node> {
    n.child.iter().find(|c| c.id == id)
}

fn child_mut<'a>(n: &'a mut Node, id: &str) -> Option<&'a mut Node> {
    n.child.iter_mut().find(|c| c.id == id)
}

pub(crate) fn find<'a>(n: &'a Node, id: &str) -> Option<&'a Node> {
    if n.id == id {
        return Some(n);
    }
    n.child.iter().find_map(|c| find(c, id))
}

fn find_mut<'a>(n: &'a mut Node, id: &str) -> Option<&'a mut Node> {
    if n.id == id {
        return Some(n);
    }
    n.child.iter_mut().find_map(|c| find_mut(c, id))
}

/// ⚠ SDK 다운캐스트(`downcast_ref::<ImageRunner>()`)는 이 게임에서 **항상 실패**한다
/// (인게임 실측 130/130 실패). 이유·대안 = `raw.rs` 헤더 주석. 이미지 접근은 전부 raw 경유.
fn image_source(n: &Node) -> Option<String> {
    raw::get_source(n)
}

/// 챔피언 스프라이트 에셋의 경로 접두 — 인게임 실측(`illust_debug.txt`)으로 확정:
/// `asset/base/aseprite_resources/champions/patchouli#sheet`
const CHAMP_ASSET_DIR: &str = "aseprite_resources/champions/";

/// 상대 픽이 아직 공개되지 않은 구간(내 로스터 순서 확정 전)에 게임이 넣는 플레이스홀더.
/// 이때도 원본 모드는 전용 일러스트(`question_blue`/`question_red`)를 깔았다.
const QUESTION_ICON: &str = "ui/icons/question";
/// 위 플레이스홀더를 나타내는 내부 표식(진영에 따라 실제 아트명으로 번역된다).
pub(crate) const QUESTION_ID: &str = "question";

// ── 픽 완료 슬롯 하단 정보 행 좌표 (레드 기준, 블루는 좌우 대칭) ───────────
// ⚠ `.ui` 오버라이드 생성기(scratchpad\layout_row.py)와 **같은 값을 유지**할 것.
//   게임이 매 프레임 좌표를 되돌리므로 `.ui` 만으론 안 먹고 여기서 재기입한다.
const SLOT_W: f32 = 300.0;
// 요소별 y (숙련도 초상화만 확정값, 나머지는 미세조정 대상)
const PORT_Y: f32 = 140.0; // 숙련도 챔프 목록 (확정)
const NAME_Y: f32 = 145.0; // 선수 이름
const POS_Y: f32 = 117.0; // 이름 윗줄 = 포지션 아이콘
const BADGE_Y: f32 = 148.0; // 보너스 배지
const TITLE_Y: f32 = 121.0; // 배지 윗줄 = "숙련도 보너스" 글자

const NAME_W: f32 = 96.0; // 바닐라 폭
const NAME_H: f32 = 28.0;
const POS_W: f32 = 22.0;
const BADGE_W: f32 = 30.0;
/// 숙련도 챔프 '목록' 컨테이너 폭 — 아이콘 1개가 아니라 여러 개가 들어가는 행.
const PORT_W: f32 = 124.0;
const TITLE_W: f32 = 110.0;
const TITLE_H: f32 = 18.0;

/// 레드 기준 x — (이름, 포지션, 숙련도 초상화, 보너스 배지, 보너스 글자)
const X_NAME: f32 = 10.0;
const X_POS: f32 = 10.0;
const X_BADGE: f32 = 109.0;
const X_TITLE: f32 = 90.0;
const X_PORT: f32 = 166.0; // 166+124=290 = 슬롯(300) 안

/// image source 문자열에서 챔피언 id 추출.
///
/// ⚠ 반드시 챔피언 스프라이트 경로인 것만 받는다. 예전 구현은 "경로 마지막 세그먼트"를
/// 무조건 챔프 id 로 봤는데, 픽 직후 이 노드가 아직 플레이스홀더
/// `asset/base/ui/icons/question` 이라서 `question` 을 챔프로 오인했다
/// (illust 폴더에 question.png 가 있어 조용히 물음표 그림이 잡혔다).
pub(crate) fn champ_id_from_source(src: &str) -> Option<String> {
    let path = src.split('#').next().unwrap_or("");
    // 미공개 픽 = 챔피언 경로가 아니라 물음표 아이콘이 들어온다.
    if path.ends_with(QUESTION_ICON) {
        return Some(QUESTION_ID.to_string());
    }
    let rest = path.split(CHAMP_ASSET_DIR).nth(1)?;
    let id = rest.split('/').next().unwrap_or("").trim();
    if id.is_empty() || id.contains(' ') {
        return None;
    }
    Some(id.to_string())
}

/// 상태 그룹 하나가 "확정한 챔피언"을 들고 있으면 그 id.
///
/// ★판별 범위 = **그 그룹의 `champion > icon` 한 곳뿐**. 슬롯 서브트리 전체를 훑던
/// 이전 판(2026-07-18)은 그 선수의 **모스트 챔피언 목록 아이콘을 픽으로 오인**해서,
/// 아직 픽 안 한 슬롯에도 일러스트를 깔아 모스트 챔프 사진을 덮어버렸다.
/// 그룹 단위로 보면 자기 그룹이 챔프 초상화를 들고 있을 때만 일러스트가 뜨므로,
/// 모스트 챔프를 보여주는 그룹은 자연히 건드리지 않는다.
fn group_champ(grp: &Node) -> Option<String> {
    let icon = child(grp, "champion").and_then(|c| child(c, "icon"))?;
    champ_id_from_source(&image_source(icon)?)
}

// ── 슬롯 적용 ─────────────────────────────────────────────────────────────

/// 픽 슬롯(`pick_slot_0` ..)을 찾아 적용.
fn walk_apply(n: &mut Node, side: &str, cfg: &config::Cfg, dbg: &mut Vec<String>, idx: &mut usize) {
    if n.id.starts_with("pick_slot") {
        if cfg.test_mode {
            test_write(n, *idx);
        } else {
            apply_slot(n, side, cfg, dbg);
        }
        *idx += 1;
        return;
    }
    for c in n.child.iter_mut() {
        walk_apply(c, side, cfg, dbg, idx);
    }
}

/// 슬롯 하나 → 상태 그룹(wait/in_turn/done)마다 **각자** 판별해서 반영.
///
/// 일러스트 노드는 세 그룹 모두에 심어져 있지만, 실제로 켜지는 건 자기 그룹이
/// 챔프 초상화를 들고 있는 그룹뿐이다(= 사실상 픽이 확정된 슬롯).
fn apply_slot(slot: &mut Node, side: &str, cfg: &config::Cfg, dbg: &mut Vec<String>) {
    let slot_id = slot.id.clone();
    apply_states(slot, side, cfg, dbg, &slot_id);
}

fn apply_states(n: &mut Node, side: &str, cfg: &config::Cfg, dbg: &mut Vec<String>, slot: &str) {
    if child(n, N_ILLUST).is_some() {
        let champ = group_champ(n);
        // 미공개 픽은 진영별 전용 아트명으로 번역 (blue/question_blue, red/question_red).
        let art = champ.as_deref().map(|c| {
            if c == QUESTION_ID {
                let team = if side == keys::BLUE { "blue" } else { "red" };
                format!("{QUESTION_ID}_{team}")
            } else {
                c.to_string()
            }
        });
        let found = art
            .as_deref()
            .and_then(|c| keys::illust_key(c, side, cfg.zoom));
        let key = found.map(|(k, _)| k);
        let flip = found.map(|(_, f)| f).unwrap_or(false);
        if cfg.debug && champ.is_some() {
            dbg.push(format!(
                "{slot}.{} champ={champ:?} art={art:?} key={key:?} flip={flip}",
                n.id
            ));
        }
        apply_one(n, key, flip, cfg);
        // 픽 전(wait/in_turn)에도 같은 배치를 쓴다 — 그 그룹엔 배지·보너스글자가 없어
        // 해당 노드는 자동으로 건너뛴다.
        if cfg.move_info_row && matches!(n.id.as_str(), "done" | "wait" | "in_turn") {
            force_info_row(n, side, n.id == "done");
        }
        return;
    }
    for c in n.child.iter_mut() {
        apply_states(c, side, cfg, dbg, slot);
    }
}

/// 픽 연출(선택 순간 떠서 픽창으로 날아가는 이미지) 노드를 찾기 위한 진단.
///
/// 그 연출 노드는 `banpick/layout.ui` 에 선언이 없다(=런타임 생성)라서 정적으로는
/// 못 찾는다. 그래서 **픽 슬롯과 챔피언 목록 바깥**에 나타나는 챔피언 스프라이트를
/// 경로째 기록한다. 연출이 뜬 프레임이 잡히면 그 경로가 곧 후보 노드다.
fn scan_stray_champ(n: &Node, path: &str, out: &mut Vec<String>) {
    if out.len() > 200 {
        return;
    }
    if n.visible {
        if let Some(s) = image_source(n) {
            if let Some(id) = champ_id_from_source(&s) {
                out.push(format!("{path}/{} = {id}", n.id));
            }
        }
    }
    for c in n.child.iter() {
        // 챔프 아이콘이 원래 잔뜩 있는 곳은 전부 제외 — 그래야 연출 노드가 드러난다.
        // (1차 관측에서 통계팝업/코치대화가 40개 상한을 다 먹어 연출까지 못 갔다.)
        let noisy = matches!(
            c.id.as_str(),
            BLUE_PICKS
                | RED_PICKS
                | "champions"
                | "stats_popup"
                | "swap"
                | "bottom"
                | "blue_pool"
                | "red_pool"
                | "champion_info"
                | "champion_info_tooltip"
                | "pick_slot_tooltip_layer"
        ) || c.id.starts_with("coach_dialogue");
        if noisy {
            continue;
        }
        let p = format!("{path}/{}", n.id);
        scan_stray_champ(c, &p, out);
    }
}

/// 쓰기 경로 시험 — 챔프 감지와 무관하게 강제로 source 를 넣고 켠다.
///
/// 세 가지를 한 번에 가른다:
///   · 일러스트 키 슬롯도, base 아이콘 슬롯도 보임 → 쓰기 OK + 에셋 등록 OK (읽기만 고치면 끝)
///   · base 아이콘만 보임                        → 쓰기 OK, 일러스트 에셋 등록/키가 문제
///   · 둘 다 안 보임                             → style.normal.source 쓰기가 엔진에 안 먹음
fn test_write(done: &mut Node, idx: usize) {
    let src: &str = if idx < 3 {
        "asset/tfm2_banpick_illust/illust/knight"
    } else {
        "asset/base/ui/icons/top"
    };
    if let Some(n) = child_mut(done, N_ILLUST) {
        raw::set_source(n, src);
        n.visible = true;
    }
    done.visible = true;
}

/// 상태 그룹(wait/in_turn/done) 하나에 일러스트를 반영.
/// 하단 정보 행을 매 프레임 제자리에 고정한다.
///
/// 게임이 `name`/`proficiency_badge`/`proficiency_top` 의 좌표를 런타임에 다시 쓰기
/// 때문에 `.ui` 값만으론 화면에 반영되지 않는다(자식 노드만 옮겨져 보이는 증상).
fn force_info_row(done: &mut Node, side: &str, _is_done: bool) {
    let is_red = side != keys::BLUE;
    // 레드 기준 좌표를 블루에선 좌우 대칭으로 뒤집는다.
    let x_of = |x: f32, w: f32| if is_red { x } else { SLOT_W - x - w };

    if let Some(n) = child_mut(done, "name") {
        raw::force_layout(n, x_of(X_NAME, NAME_W), NAME_Y, Some((NAME_W, NAME_H)));
    }
    if let Some(n) = child_mut(done, "position") {
        raw::force_layout(n, x_of(X_POS, POS_W), POS_Y, None);
    }
    if let Some(n) = child_mut(done, "proficiency_top") {
        // ⚠ 폭(124)은 건드리지 않는다 — 아이콘 여러 개가 들어가는 행이라 줄이면
        //   오른쪽 아이콘이 잘리고, 키우면 슬롯 밖으로 넘친다(2026-07-20 실측).
        raw::force_layout(n, x_of(X_PORT, PORT_W), PORT_Y, None);
    }
    if let Some(n) = child_mut(done, "proficiency_badge") {
        raw::force_layout(n, x_of(X_BADGE, BADGE_W), BADGE_Y, None);
    }
    if let Some(n) = child_mut(done, "proficiency_badge_title") {
        raw::force_layout(n, x_of(X_TITLE, TITLE_W), TITLE_Y, Some((TITLE_W, TITLE_H)));
    }
}

fn apply_one(grp: &mut Node, key: Option<&'static str>, flip: bool, cfg: &config::Cfg) {
    match key {
        Some(key) => {
            if let Some(n) = child_mut(grp, N_ILLUST) {
                // 이미 같은 키면 다시 쓰지 않는다(매 프레임 write 회피).
                let same = raw::get_source(n).map(|s| s == key).unwrap_or(false);
                if !same {
                    raw::set_source(n, key);
                }
                // 반대 진영 아트를 끌어온 경우 구도를 맞춘다.
                raw::set_flip_x(n, flip);
                n.visible = true;
            }
            set_vis(grp, N_DIM, cfg.dim);
            // 일러스트가 슬롯을 채우므로 바닐라 소형 초상화/구분선은 중복 → 숨김(옵션).
            if cfg.hide_portrait {
                set_vis(grp, "champion", false);
                set_vis(grp, "center", false);
            }
        }
        None => {
            // 픽 전이거나 id 추출 실패 → 바닐라 그대로.
            set_vis(grp, N_ILLUST, false);
            set_vis(grp, N_DIM, false);
            if cfg.hide_portrait {
                set_vis(grp, "champion", true);
                set_vis(grp, "center", true);
            }
        }
    }
}

fn set_vis(parent: &mut Node, id: &str, v: bool) {
    if let Some(n) = child_mut(parent, id) {
        n.visible = v;
    }
}

// ── 진단 덤프 (cfg debug=1 일 때만) ───────────────────────────────────────
// 목적: ①dll 이 붙었는가 ②밴픽 UI 가 ui.root 에서 보이는가
//       ③.ui 오버라이드가 먹어 #bpi_illust 가 존재하는가 ④아이콘 source 실값.
fn dump_ids(n: &Node, depth: usize, max: usize, out: &mut String) {
    if depth > max {
        return;
    }
    let rt = n.runner.type_name();
    out.push_str(&"  ".repeat(depth));
    out.push_str(&format!("{} :{}", if n.id.is_empty() { "(무명)" } else { &n.id }, rt));
    if !n.visible {
        out.push_str(" [hidden]");
    }
    if let Some(s) = image_source(n) {
        if !s.is_empty() {
            out.push_str(&format!("  source={s:?}"));
        }
    }
    out.push('\n');
    for c in n.child.iter() {
        dump_ids(c, depth + 1, max, out);
    }
}

fn diag(root: &Node) {
    let mut out = String::new();
    out.push_str("# tfm2_banpick_illust 진단\n");
    out.push_str(&format!("ui.root.id = {:?}, 최상위 자식 {}개\n", root.id, root.child.len()));
    out.push_str("\n## ui.root 최상위 자식 id\n");
    for c in root.child.iter() {
        out.push_str(&format!("  - {} :{}\n", c.id, c.runner.type_name()));
    }

    for id in [BLUE_PICKS, RED_PICKS, "match_ui", "banpick", "bpi_illust"] {
        let hit = find(root, id).is_some();
        out.push_str(&format!("\n{id} 발견 = {hit}"));
    }
    out.push('\n');

    if let Some(bp) = find(root, BLUE_PICKS) {
        out.push_str("\n## blue_picks 서브트리 (전개)\n");
        dump_ids(bp, 0, 8, &mut out);
    } else {
        out.push_str("\n## ui.root 트리 (depth<=4)\n");
        dump_ids(root, 0, 4, &mut out);
    }
    config::write_diag("ui_tree.txt", &out);
}

// ── 진입 ──────────────────────────────────────────────────────────────────
struct BanpickIllustExt;

impl ModExtension for BanpickIllustExt {
    fn post_update(&self, scene: &mut Scene, ui: &mut GameUI, assets: &mut Assets, _dt: f32) {
        // 확장 패닉이 게임 콜스택으로 unwind 되지 않도록 방어.
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let cfg = config::get();
            if !cfg.enabled {
                return;
            }
            // 쇼케이스 카드 훅 — 늦은 1회 설치(멱등 게이트, showcase.rs)
            showcase::tick();
            // 버프/너프 가시성 — diff 갱신 + 표시명 해석 + 호버 판정 (patchviz.rs)
            patchviz::tick(scene, ui, assets);
            // 밴픽 화면이 아니면 컨테이너가 없어 즉시 반환.
            let mut dbg: Vec<String> = Vec::new();
            let mut idx = 0usize;
            let red_dir = if cfg.red_flip { keys::RED } else { keys::RED_NOFLIP };
            for (id, side) in [(BLUE_PICKS, keys::BLUE), (RED_PICKS, red_dir)] {
                if let Some(c) = find_mut(&mut ui.root, id) {
                    walk_apply(c, side, &cfg, &mut dbg, &mut idx);
                }
            }
            if cfg.debug && !dbg.is_empty() {
                dbg.insert(0, keys::summary());
                config::debug_dump(&dbg);
            }
            // 픽 연출 노드 탐색 — 슬롯/목록 밖 챔프 스프라이트가 보이면 기록.
            if cfg.debug {
                let mut stray = Vec::new();
                scan_stray_champ(&ui.root, "", &mut stray);
                if !stray.is_empty() {
                    config::stray_dump(&stray);
                }
            }
            // 진단: 약 2초마다 현재 UI 트리를 덮어쓴다(마지막으로 본 화면이 남음).
            if cfg.debug {
                let f = FRAME.fetch_add(1, Ordering::Relaxed);
                if f % 120 == 0 {
                    diag(&ui.root);
                }
            }
        }));
    }

    fn post_render(
        &self,
        _scene: &Scene,
        _ui: &GameUI,
        _assets: &Assets,
        state: &mut RenderState,
    ) {
        // 이름 버프/너프 색칠 + 호버 8각 레이더 (patchviz.rs).
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            patchviz::render(state);
        }));
    }
}

static FRAME: AtomicU32 = AtomicU32::new(0);

fn init(_ctx: &GameCtx) -> ModRegistration {
    config::load();
    // dll 로드 확인 스탬프 — 진단용이므로 debug=1 일 때만 남긴다
    // (릴리스본이 매 실행 파일을 쓰지 않도록).
    if config::get().debug {
        config::write_diag("load_stamp.txt", "tfm2_banpick_illust init() 호출됨\n");
    }
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(BanpickIllustExt);
    reg
}

declare_mod!(init);
