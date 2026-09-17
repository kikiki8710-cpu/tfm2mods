//! patchviz — 버프/너프 가시성 (daram2 "Pick Ban Plus" 신판 기능의 0.5.2 마이그, v1.3.x)
//! ===========================================================================
//! 원본 RE 정본 = `ANA\daram2-viewplus-legacy.md §6b` (0.5.1-compat dll ghidra 확정).
//!
//! 0.5.1 원본 → 0.5.2 마이그에서 확정된 차이 (2026-07-26 인게임 진단 3회):
//!   - base(무패치 원본) 스탯 소스 = **`db.pre_patch_data` 의 가장 오래된 스냅샷**
//!     (`GamePatchState { game_setting, champion_info_sheet, available_champions }` —
//!     게임이 밸런스 패치마다 "패치 이전 시트"를 통째로 저장한다).
//!     ⚠`ChampionInfoSheet::default()` 는 플레이스홀더 값 = base 아님 (전원 버프 오판 사고).
//!     패치가 아직 없으면 base = 현재 시트 = diff 전부 0 (정상).
//!   - draw-list 의 Text 는 **해석된 표시명**("부두술사")·Sprite 커맨드는 0개(아이콘은
//!     다른 변형으로 그려짐) ⟹ 색칠 = 내장 표시명 테이블(names_gen.rs, 전 로케일) 매칭,
//!     레이더 아이콘 = **NinePatch**(borders 0 = 크기 지정 가능한 텍스처 쿼드) 로 드로우.
//!   - `Assets::get_i18n_text` 는 시도한 8가지 키 형식 전부 에코(미해석) — 사용 안 함.
//!   - 0.5.1 의 호버 셀 하이라이트(119×130 밝은 rect)는 0.5.2 에 없음 ⟹ 레이더 위치 =
//!     호버 챔프의 **이름 텍스트 옆 모달**(RoundingBox 배경 + 레이더 + 축 아이콘).
//!
//! 판정·기하 = daram2 원본 그대로: V=Lv1*2+성장*11, diff=(cur−base)/base, Σ±0.01,
//! 8각(외곽원 r60/스포크 r58/기준링 r33/다각형 v=clamp(diff/0.15,±1)·r=33+v*(21|27)/
//! 아이콘 r72), 색 = 버프 초록/너프 빨강/중립 파랑.

use crate::{config, modchamps, names_gen, raw};
use engine_core::render_state::RenderCommand;
use mod_api::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};

static RENDER_CALLS: AtomicU32 = AtomicU32::new(0);
static TICK_CALLS: AtomicU32 = AtomicU32::new(0);

/// 밴픽 화면 활성 여부 — post_update(tick)가 판정, post_render 가 게이트로 사용.
static BP_ACTIVE: AtomicBool = AtomicBool::new(false);

struct Viz {
    /// champ_id → (diff[8], 판정 −1/0/+1)
    diffs: HashMap<String, ([f32; 8], i8)>,
    /// 표시명(trim) → champ_id (names_gen 내장 테이블)
    disp2id: HashMap<&'static str, &'static str>,
    /// 모드 챔프 표시명 → id — 설치된 모드의 `text\*.i18n` 에서 수확(전 로케일).
    /// 종전엔 런타임 학습(learned)에만 의존해 호버 전엔 색칠이 절대 안 됐다.
    ext: HashMap<String, String>,
    /// 런타임 학습 표시명 → id (i18n 파일이 없는 모드 챔프 — 툴팁 안 텍스트에서 학습)
    learned: HashMap<String, String>,
    /// **호버 중**인 챔프 (champion_info_tooltip 이 보일 때만 Some — 레이더 게이트)
    hover: Option<String>,
    /// 호버 툴팁의 화면 rect (레이더 모달 앵커)
    tip_rect: Option<(f32, f32, f32, f32)>,
    /// 하단 champion_info 패널이 보여주는 챔프 (스탯 수치 색칠용 — 패널은 잔존형)
    hover_panel: Option<String>,
    /// 스탯 표시명 → diff 축 (names_gen 내장)
    statname2axis: HashMap<&'static str, usize>,
    /// diffs 재계산 키 (인게임 날짜 + 패치 스냅샷 수)
    built_key: String,
}

// 축별 아이콘 템플릿 — 패널이 그린 스탯 아이콘 NinePatch 통째 캐시.
// RenderCommand 는 Rc 를 품어 !Send ⟹ thread_local (렌더 스레드 전용 접근).
thread_local! {
    static ICON_TPL: std::cell::RefCell<Vec<Option<RenderCommand>>> =
        std::cell::RefCell::new(vec![None; 8]);
}

fn viz() -> &'static Mutex<Viz> {
    static V: OnceLock<Mutex<Viz>> = OnceLock::new();
    V.get_or_init(|| {
        Mutex::new(Viz {
            diffs: HashMap::new(),
            disp2id: names_gen::NAME2ID.iter().copied().collect(),
            ext: modchamps::names().clone(),
            learned: HashMap::new(),
            hover: None,
            tip_rect: None,
            hover_panel: None,
            statname2axis: names_gen::STATNAME2AXIS.iter().copied().collect(),
            built_key: String::new(),
        })
    })
}

// ── diff 빌더 ──────────────────────────────────────────────────────────────

/// V = Lv1*2 + 성장*11 (Lv1 + Lv12 합) — daram2 비교식.
fn lv_sum(l1: f64, g: f64) -> f64 {
    l1 * 2.0 + g * 11.0
}

fn ratio(base: f64, cur: f64) -> f32 {
    if base > 0.0 {
        ((cur - base) / base) as f32
    } else {
        0.0
    }
}

/// 챔프 한 마리의 비교용 스탯 스냅샷.
///
/// ★왜 `&dyn ChampionInfo` 를 직접 비교하지 않고 값으로 뽑는가(v1.3.2):
///   base 소스가 3갈래(패치 스냅샷 시트 / 스냅샷의 `mod_champions` / 모드 배포 파일)라
///   빌림 수명이 제각각이고, 파일 base 는 애초에 `dyn ChampionInfo` 가 아니다.
#[derive(Clone, Copy, Debug, Default)]
struct Snap {
    /// Lv1 값 — 0공격 1주문력 2체력 3방어 4마저 5이속
    lv1: [f64; 6],
    /// 레벨당 성장치 (같은 축 순서)
    grow: [f64; 6],
    /// 평타 duration (공속 프록시)
    dur: f64,
    rng1: f64,
    rngg: f64,
    /// 사거리 축 비교 가능 여부 — 파일 base 에는 대응값이 없어 false.
    has_rng: bool,
}

fn snap_of(c: &dyn ChampionInfo) -> Snap {
    let (s, g) = (c.stat(), c.growth());
    let a = c.attack();
    let (mut rng1, mut rngg, mut has_rng) = (0.0, 0.0, false);
    if let Some(e) = a.effect() {
        rng1 = e.range as f64;
        rngg = e.growth_range as f64;
        has_rng = true;
    }
    Snap {
        lv1: [
            s.attack as f64,
            s.magic_power as f64,
            s.hp as f64,
            s.defence as f64,
            s.magic_resistance as f64,
            s.move_speed as f64,
        ],
        grow: [
            g.attack as f64,
            g.magic_power as f64,
            g.hp as f64,
            g.defence as f64,
            g.magic_resistance as f64,
            g.move_speed as f64,
        ],
        dur: a.duration() as f64,
        rng1,
        rngg,
        has_rng,
    }
}

/// 챔프 하나의 diff. b = 무패치 원본, c = 현재 병합값.
/// 반환 = (축별 종합 diff[8], 축별 [Lv1, 성장, Lv12] 개별 증감률).
/// 축 = 0공격 1주문력 2체력 3방어 4마저 5이속 6공속 7사거리
fn diff_full(b: &Snap, c: &Snap) -> ([f32; 8], [[f32; 3]; 8]) {
    let mut comb = [0f32; 8];
    let mut per = [[0f32; 3]; 8];
    for i in 0..6 {
        let (bl, bgr, cl, cgr) = (b.lv1[i], b.grow[i], c.lv1[i], c.grow[i]);
        comb[i] = ratio(lv_sum(bl, bgr), lv_sum(cl, cgr));
        per[i] = [
            ratio(bl, cl),
            ratio(bgr, cgr),
            ratio(bl + bgr * 11.0, cl + cgr * 11.0),
        ];
    }
    if b.dur > 0.0 && c.dur > 0.0 {
        let v = (b.dur / c.dur - 1.0) as f32;
        comb[6] = v;
        per[6] = [v, v, v];
    }
    if b.has_rng && c.has_rng {
        comb[7] = ratio(b.rng1 * 2.0 + b.rngg * 11.0, c.rng1 * 2.0 + c.rngg * 11.0);
        per[7] = [
            ratio(b.rng1, c.rng1),
            ratio(b.rngg, c.rngg),
            ratio(b.rng1 + b.rngg * 11.0, c.rng1 + c.rngg * 11.0),
        ];
    }
    (comb, per)
}

fn flag_of(d: &[f32; 8]) -> i8 {
    let sum: f32 = d.iter().sum();
    if sum > 0.01 {
        1
    } else if sum < -0.01 {
        -1
    } else {
        0
    }
}

/// 버전 문자열 → 숫자열 정렬 키 ("1.0.10" > "1.0.9" 정상 비교).
fn ver_key(s: &str) -> (Vec<u64>, String) {
    let nums: Vec<u64> = s
        .split(|c: char| !c.is_ascii_digit())
        .filter(|p| !p.is_empty())
        .filter_map(|p| p.parse().ok())
        .collect();
    (nums, s.to_string())
}

/// 날짜/패치 수가 바뀌었으면 diff 맵 재계산.
fn build_diffs(data: &ClientData, v: &mut Viz, debug: bool) {
    let db = data.db();
    let key: String = format!(
        "{}|{}",
        format!("{:?}", db.time).chars().take(10).collect::<String>(),
        db.pre_patch_data.len()
    );
    if v.built_key == key && !v.diffs.is_empty() {
        return;
    }
    // 가장 오래된 "패치 이전" 스냅샷 = 무패치 원본. 없으면 현재 시트(=diff 0).
    let mut vers: Vec<&String> = db.pre_patch_data.keys().collect();
    vers.sort_by_key(|s| ver_key(s));
    let mut base_sheet: ChampionInfoSheet = match vers.first() {
        Some(k0) => db.pre_patch_data[k0.as_str()].champion_info_sheet.clone(),
        None => db.champion_info_sheet.clone(),
    };

    // 대상 = available_champions ∪ 현재 시트의 모드 챔프.
    // ★모드 챔프는 available_champions 에 안 잡히는 경우가 있어(시트 밖 컨테이너) 합집합.
    let mut ids: Vec<String> = db.available_champions.clone();
    for e in &db.champion_info_sheet.mod_champions {
        if !ids.iter().any(|x| x == &e.id) {
            ids.push(e.id.clone());
        }
    }

    // 모드 챔프의 base: 정렬된 **모든** 패치 스냅샷을 순서대로 훑어 처음 나오는 값.
    // (모드를 나중에 깔았으면 最古 스냅샷엔 없고 그 다음 스냅샷부터 있다.)
    let base_mod = |id: &str| -> Option<Snap> {
        for k in &vers {
            let s = &db.pre_patch_data[k.as_str()].champion_info_sheet;
            if let Some(e) = s.mod_champions.iter().find(|e| e.id == id) {
                return Some(snap_of(e));
            }
        }
        None
    };

    let mut out: HashMap<String, ([f32; 8], i8)> = HashMap::new();
    let mut srcs: Vec<(String, &'static str)> = Vec::new(); // 진단: id → base 출처
    for name in &ids {
        // 현재값 — champion_info() 가 못 잡는 모드 챔프는 시트의 mod_champions 에서.
        let cur = match db.champion_info(name) {
            Some(c) => snap_of(&*c),
            None => match db
                .champion_info_sheet
                .mod_champions
                .iter()
                .find(|e| &e.id == name)
            {
                Some(e) => snap_of(e),
                None => continue,
            },
        };
        // base — ①패치 스냅샷 시트(바닐라) ②스냅샷의 모드챔프 ③모드가 배포한 원본 파일.
        let (base, src) = if let Some(b) = base_sheet.get_champion_mut(name) {
            (Some(snap_of(&*b)), "sheet")
        } else if let Some(b) = base_mod(name) {
            (Some(b), "snap_mod")
        } else if let Some(fb) = modchamps::file_bases().get(name) {
            (
                Some(Snap {
                    lv1: fb.lv1,
                    grow: fb.grow,
                    dur: fb.dur,
                    ..Default::default()
                }),
                "file",
            )
        } else {
            (None, "none")
        };
        let d = match &base {
            Some(b) => diff_full(b, &cur).0,
            None => [0f32; 8],
        };
        out.insert(name.clone(), (d, flag_of(&d)));
        srcs.push((name.clone(), src));
    }
    if debug {
        let mut body = String::from("# champ: [atk ap hp def mr spd as rng] sum flag base출처\n");
        body.push_str(&format!(
            "# pre_patch versions(정렬) = {vers:?} / 채택 base = {:?}\n",
            vers.first()
        ));
        body.push_str(&format!(
            "# available={} 현재시트 mod_champions={} 대상합계={} / {}\n",
            db.available_champions.len(),
            db.champion_info_sheet.mod_champions.len(),
            ids.len(),
            modchamps::summary()
        ));
        for k in &vers {
            body.push_str(&format!(
                "#   snapshot {k}: mod_champions={}\n",
                db.pre_patch_data[k.as_str()].champion_info_sheet.mod_champions.len()
            ));
        }
        let smap: HashMap<&str, &str> =
            srcs.iter().map(|(a, b)| (a.as_str(), *b)).collect();
        let mut rows: Vec<(&String, &([f32; 8], i8))> = out.iter().collect();
        rows.sort_by(|a, b| a.0.cmp(b.0));
        for (n, (d, f)) in &rows {
            let sum: f32 = d.iter().sum();
            body.push_str(&format!(
                "{n}: {d:?} sum={sum:.4} flag={f} base={}\n",
                smap.get(n.as_str()).copied().unwrap_or("?")
            ));
        }
        config::write_diag("patchviz_diffs.txt", &body);
    }
    v.diffs = out;
    v.built_key = key;
}

// ── 호버 판정 (champion_info 패널 라벨의 i18n 키 파싱) ────────────────────

/// `#asset/base/text/champion?description.<id>.name` / `...?name.<id>` → id
fn i18n_champ_id(t: &str) -> Option<String> {
    if !t.starts_with('#') {
        return None;
    }
    let q = t.split('?').nth(1)?;
    let parts: Vec<&str> = q.split('.').collect();
    match parts.as_slice() {
        ["description", id, "name"] => Some((*id).to_string()),
        ["name", id] => Some((*id).to_string()),
        _ => None,
    }
}

fn subtree_champ_id(n: &Node) -> Option<String> {
    if let Some(t) = raw::label_text(n) {
        if let Some(id) = i18n_champ_id(t.trim()) {
            return Some(id);
        }
    }
    n.child.iter().find_map(subtree_champ_id)
}

/// 하단 잔존 패널(champion_info)의 현재 챔프 — 스탯 수치 색칠용.
fn hover_champ_panel(root: &Node) -> Option<String> {
    let panel = crate::find(root, "champion_info")?;
    if !panel.visible {
        return None;
    }
    subtree_champ_id(panel)
}

// (호버 툴팁은 UI 트리에서 안 보인다(0.5.2 실측 — 호버 중에도 노드 invisible)
//  ⟹ daram2 원본과 동일하게 **draw-list 에서 툴팁 박스를 직접 탐지**한다. render_impl 참조.)

// (하단 스탯 패널 수치 색칠 기능은 유저 판단으로 제거 — 2026-07-26 v1.3.0 개발 중.
//  값별 색칠까지 시도했으나 체감 품질 미달. 재도입 시 diff_full 의 per-value 결과 사용.)

// ── 진입 (post_update 에서 호출) ──────────────────────────────────────────

pub fn tick(scene: &mut Scene, ui: &mut GameUI, _assets: &Assets) {
    let cfg = config::get();
    if !cfg.enabled || !(cfg.name_color || cfg.radar) {
        BP_ACTIVE.store(false, Ordering::Relaxed);
        return;
    }
    let bp = crate::find(&ui.root, "blue_picks").is_some();
    BP_ACTIVE.store(bp, Ordering::Relaxed);
    if !bp {
        return;
    }
    let mut v = viz().lock().unwrap_or_else(|e| e.into_inner());
    // 챔프 id = 하단 패널의 i18n 키 (모드 챔프 포함). 호버 여부 게이트는
    // render 쪽 draw-list 툴팁 박스 탐지가 담당.
    v.hover_panel = hover_champ_panel(&ui.root);
    v.hover = v.hover_panel.clone();
    v.tip_rect = None;
    if let Scene::InGame { data } = scene {
        build_diffs(data, &mut v, cfg.debug);
    }

    if cfg.debug {
        let t = TICK_CALLS.fetch_add(1, Ordering::Relaxed);
        if t % 120 == 0 {
            let (mut nb, mut nn) = (0, 0);
            for (_, f) in v.diffs.values() {
                if *f > 0 {
                    nb += 1;
                } else if *f < 0 {
                    nn += 1;
                }
            }
            config::write_diag(
                "patchviz_debug.txt",
                &format!(
                    "# patchviz 진단 (tick={t}, render={r})\n\
                     bp_active=true diffs={dl}개 (버프 {nb} / 너프 {nn}) built_key={bk:?}\n\
                     disp2id={nd}개(내장) ext={ne}개(모드i18n) learned={nl}개\n\
                     hover(레이더)={hv:?} hover_panel={hp:?} tip_rect={tr:?}\n\
                     render 상세 = patchviz_render.txt / diff 상세 = patchviz_diffs.txt\n",
                    r = RENDER_CALLS.load(Ordering::Relaxed),
                    dl = v.diffs.len(),
                    nd = v.disp2id.len(),
                    ne = v.ext.len(),
                    nl = v.learned.len(),
                    bk = v.built_key,
                    hv = v.hover,
                    hp = v.hover_panel,
                    tr = v.tip_rect,
                ),
            );
        }
    }
}

// ── post_render — 이름 색칠 + 레이더 모달 ─────────────────────────────────

/// daram2 색상 (RE §6b 확정값)
const BUFF_RGB: (f32, f32, f32) = (0.30, 0.82, 0.40); // ≈#4DD166
const NERF_RGB: (f32, f32, f32) = (0.93, 0.36, 0.34); // ≈#ED5C57

/// 포지션 표시명 전 로케일 (ui.i18n `position` 블록, 0.5.x 79종) — 툴팁 안에
/// 챔프 이름과 같이 뜨는 라벨이라, 모드챔프 이름 학습에서 제외하지 않으면
/// "미드"/"서포터" 같은 라벨이 챔프로 오학습돼 화면 전체가 색칠된다(07-27 제보).
static POS_LABELS: &[&str] = &[
    "Alt", "Base", "Bot", "Bottom", "Destek", "Dolna aleja", "Dżungla", "Dự Bị", "Ersatzsp.",
    "Górna aleja", "Hỗ Trợ", "Jungla", "Jungle", "Kākoʻo", "Lalo", "Luna", "Mea Pāʻani Pani",
    "Medio", "Meio", "Mid", "Orman", "Orta", "Rempl.", "Reservas", "Reserve", "Rezerwa",
    "Riserva", "Selva", "Soporte", "Sub", "Suplente", "Suporte", "Support", "Top", "Topo",
    "Ululāʻau", "Waena", "Wsparcie", "Yedek", "Üst", "Đi Rừng", "Đường Dưới", "Đường Giữa",
    "Đường Trên", "Środkowa aleja", "Запас", "Керри", "Лес", "Мид", "Поддержка", "Топ",
    "ซัพพอร์ต", "ป่า", "ผู้เล่นตัวสำรอง", "เลนกลาง", "เลนบน", "เลนล่าง", "サブ選手", "サポート",
    "ジャングル", "トップ", "ボトム", "ミッド", "上单", "上路", "下路", "中单", "中路", "打野",
    "替补选手", "替補選手", "輔助", "辅助", "미드", "바텀", "서포터", "정글", "탑", "후보 선수",
];

/// 레이더 아이콘 시트 — 정규화 UV (sprite_sheet json 원본값). 축 순서 = 스탯 패널 순:
/// 공격, 주문력, 공속, 체력, 방어, 마저, 사거리, 이속
const AXIS_DIFF: [usize; 8] = [0, 1, 6, 2, 3, 4, 7, 5];
const AXIS_UVX: [f32; 8] = [0.0, 0.125, 0.75, 0.375, 0.25, 0.875, 0.5, 0.625];
const ICON_SHEET: &str = "asset/base/ui/banpick/champion_stat_icon#sheet";
const UV_W: f32 = 0.1125;
const UV_H: f32 = 0.9;

pub fn render(state: &mut RenderState) {
    let n = RENDER_CALLS.fetch_add(1, Ordering::Relaxed);
    let cfg = config::get();
    let dbg_on = cfg.debug && n % 120 == 0;
    let mut dbg = String::new();
    let reason = render_impl(state, &cfg, if dbg_on { Some(&mut dbg) } else { None });
    if dbg_on {
        config::write_diag(
            "patchviz_render.txt",
            &format!("# patchviz render #{n} exit={reason}\n{dbg}"),
        );
    }
}

fn render_impl(
    state: &mut RenderState,
    cfg: &config::Cfg,
    mut dbg: Option<&mut String>,
) -> &'static str {
    use std::fmt::Write as _;
    macro_rules! dlog {
        ($($a:tt)*) => { if let Some(d) = dbg.as_deref_mut() { let _ = writeln!(d, $($a)*); } };
    }
    if !BP_ACTIVE.load(Ordering::Relaxed) {
        return "bp_inactive";
    }
    let mut v = viz().lock().unwrap_or_else(|e| e.into_inner());
    if v.diffs.is_empty() {
        return "no_diffs";
    }

    // 호버 챔프의 표시명 집합 (툴팁 박스 판별용 — 내장 테이블 + 학습분)
    let hover_names: Vec<String> = match &v.hover {
        Some(hv) => names_gen::NAME2ID
            .iter()
            .filter(|(_, id)| *id == hv.as_str())
            .map(|(n, _)| (*n).to_string())
            .chain(
                v.ext
                    .iter()
                    .chain(v.learned.iter())
                    .filter(|(_, id)| *id == hv.as_str())
                    .map(|(n, _)| n.clone()),
            )
            .collect(),
        None => Vec::new(),
    };

    // ① 이름 색칠(내장 표시명 테이블 + 원시 i18n 키 폴백) + 호버 이름 텍스트 앵커.
    let mut best_key: Option<String> = None;
    let mut best_hits = 0usize;
    let mut fallback_key: Option<String> = None;
    let mut fallback_texts = 0usize;
    let mut anchor: Option<(f32, f32)> = None; // 호버 챔프 이름 텍스트 중심 (백업 앵커)
    // 호버 툴팁 박스 후보 (daram2 방식 draw-list 탐지):
    // .ui 지문 = 테두리 #4a4c56 + **배경 #161721** + 크기 범위. 최종 선별은 루프 뒤에서
    // "박스 안에 호버 챔프 이름 텍스트가 있는가"로 확정.
    let mut box_cands: Vec<(f32, f32, f32, f32)> = Vec::new();
    let mut hover_txts: Vec<(f32, f32)> = Vec::new(); // 호버 챔프 이름 텍스트 중심들
    let mut stat_labels: Vec<(usize, f32, f32)> = Vec::new(); // (축, 라벨 좌x, 중심y)
    let mut box_dump: Vec<String> = Vec::new(); // debug 전수 인벤토리
    let mut cells: Vec<(f32, f32, f32, f32)> = Vec::new(); // 그리드 셀 (z=100, 132×130)
    // 미지의 텍스트(모드 챔프 이름 후보) — 툴팁 박스 안에 있으면 학습.
    let mut unknown_texts: Vec<(String, f32, f32)> = Vec::new();
    for (k, cmds) in state.commands.iter_mut() {
        let mut texts = 0usize;
        let mut hits = 0usize;
        for c in cmds.iter_mut() {
            if let RenderCommand::Text { text, color, rect, z, .. } = c {
                texts += 1;
                let t = text.trim();
                let id_owned;
                let id: Option<&str> = match v.disp2id.get(t) {
                    Some(i) => Some(i),
                    // 모드 챔프 = 설치 모드의 i18n 에서 수확한 표시명 → 런타임 학습 순.
                    None => match v.ext.get(t).or_else(|| v.learned.get(t)) {
                        Some(i) => Some(i.as_str()),
                        None => match i18n_champ_id(t) {
                            Some(i) => {
                                id_owned = i;
                                Some(id_owned.as_str())
                            }
                            None => None,
                        },
                    },
                };
                if let Some(id) = id {
                    // diffs 미보유 챔프(base 데이터드리븐 4종 등)도 best_key 판정엔 포함.
                    hits += 1;
                    if let Some((_, f)) = v.diffs.get(id) {
                        // 쇼케이스 카드(중앙 대형 카드)의 이름 텍스트는 색칠 제외 — 기본색 유지.
                        if cfg.name_color && *f != 0 && *z != crate::showcase::Z_NAME_TEXT as i32 {
                            if *f > 0 {
                                color.r = BUFF_RGB.0;
                                color.g = BUFF_RGB.1;
                                color.b = BUFF_RGB.2;
                            } else {
                                color.r = NERF_RGB.0;
                                color.g = NERF_RGB.1;
                                color.b = NERF_RGB.2;
                            }
                        }
                    }
                    if hover_names.iter().any(|n| n == t) {
                        let c0 = (rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
                        if anchor.is_none() {
                            anchor = Some(c0);
                        }
                        if hover_txts.len() < 16 {
                            hover_txts.push(c0);
                        }
                    }
                } else if let Some(ax) = v.statname2axis.get(t) {
                    // 스탯 이름 라벨 ("공격력" 등) — 아이콘 짝짓기 기준점.
                    if stat_labels.len() < 24 {
                        stat_labels.push((*ax, rect.x, rect.y + rect.h * 0.5));
                    }
                } else {
                    let n_chars = t.chars().count();
                    if (2..=30).contains(&n_chars)
                        && !t.starts_with('#')
                        && t.parse::<f64>().is_err()
                        && !POS_LABELS.contains(&t)
                        && unknown_texts.len() < 64
                    {
                        unknown_texts.push((
                            t.to_string(),
                            rect.x + rect.w * 0.5,
                            rect.y + rect.h * 0.5,
                        ));
                    }
                }
            } else if let RenderCommand::RoundingBox {
                rect,
                color,
                background,
                z: bz,
                ..
            } = c
            {
                // 전수 인벤토리 (debug — 툴팁 실물 지문 확정용)
                if dbg.is_some() && box_dump.len() < 150 {
                    let bg = background
                        .as_ref()
                        .map(|(a0, b)| format!("({a0:.2},{:.3},{:.3},{:.3},{:.2})", b.r, b.g, b.b, b.a))
                        .unwrap_or_else(|| "None".into());
                    box_dump.push(format!(
                        "RB z={bz} rect=({:.0},{:.0},{:.0},{:.0}) col=({:.3},{:.3},{:.3},{:.2}) bg={bg}",
                        rect.x, rect.y, rect.w, rect.h, color.r, color.g, color.b, color.a
                    ));
                }
                // 툴팁 지문: 테두리 #4a4c56 + 배경 #161721 + 크기 범위.
                let bg_ok = background
                    .as_ref()
                    .map(|(_, b)| {
                        (b.r - 0.0863).abs() < 0.03
                            && (b.g - 0.0902).abs() < 0.03
                            && (b.b - 0.1294).abs() < 0.03
                    })
                    .unwrap_or(false);
                // 0.5.2 실측(patchviz_boxes.txt, 07-26) 확정 지문:
                //   호버 툴팁 = **z=101**(유일한 오버레이 층) + 이 색 + 130×72급
                //   그리드 셀 = z=100 + 같은 색 + 132×130 (이름판 자식 130×38 보유)
                let col_ok = (color.r - 0.290).abs() < 0.04
                    && (color.g - 0.298).abs() < 0.04
                    && (color.b - 0.337).abs() < 0.04;
                if bg_ok && col_ok {
                    if *bz == 101
                        && rect.w >= 80.0
                        && rect.w <= 300.0
                        && rect.h >= 25.0
                        && rect.h <= 150.0
                        && box_cands.len() < 16
                    {
                        box_cands.push((rect.x, rect.y, rect.w, rect.h));
                    } else if *bz == 100
                        && (rect.w - 132.0).abs() < 6.0
                        && (rect.h - 130.0).abs() < 6.0
                        && cells.len() < 64
                    {
                        cells.push((rect.x, rect.y, rect.w, rect.h));
                    }
                }
            }
        }
        dlog!(
            "pass {k:?}: cmds={} texts={texts} name_hits={hits} map_size={:?}",
            cmds.len(),
            state.map_size.get(k)
        );
        if hits > best_hits {
            best_hits = hits;
            best_key = Some(k.clone());
        }
        if texts > fallback_texts {
            fallback_texts = texts;
            fallback_key = Some(k.clone());
        }
    }
    if best_key.is_none() {
        best_key = fallback_key;
    }
    // 후보 확정: ①호버 챔프 이름 텍스트를 품은 박스(=툴팁 확정) ②미지 텍스트를 품은
    // 박스(모드 챔프) ③마지막 후보(가장 늦게 그려진 오버레이) 순.
    let contains = |b: &(f32, f32, f32, f32), px: f32, py: f32| {
        px >= b.0 && px <= b.0 + b.2 && py >= b.1 && py <= b.1 + b.3
    };
    if !box_dump.is_empty() {
        config::append_diag(
            "patchviz_boxes.txt",
            &format!(
                "\n== render #{} hover={:?} ==\n{}\n",
                RENDER_CALLS.load(Ordering::Relaxed),
                v.hover,
                box_dump.join("\n")
            ),
        );
    }
    // z=101 오버레이 = 툴팁 (실측상 호버 중 정확히 1개).
    let tooltip_box = box_cands.last().copied();
    dlog!(
        "best_key={best_key:?} best_hits={best_hits} hover={:?} box={tooltip_box:?}(cands={}) hover_txts={} anchor={anchor:?} learned={} unknown={}",
        v.hover,
        box_cands.len(),
        hover_txts.len(),
        v.learned.len(),
        unknown_texts.len()
    );

    // ② 호버 레이더 모달 — **draw-list 에 호버 툴팁 박스가 그려진 프레임에만** (daram2 동일).
    if !cfg.radar {
        return "radar_off";
    }
    let Some((bx0, by0, bw0, bh0)) = tooltip_box else {
        return "no_tooltip_box";
    };
    let Some(hv) = v.hover.clone() else {
        return "no_hover";
    };
    // 모드 챔프 표시명 학습: 툴팁 박스 안의 미지의 텍스트 = 호버 챔프의 이름.
    for (t, tx0, ty0) in unknown_texts {
        if tx0 >= bx0 && tx0 <= bx0 + bw0 && ty0 >= by0 && ty0 <= by0 + bh0 {
            v.learned.entry(t).or_insert_with(|| hv.clone());
        }
    }
    // diffs 미보유 = base 데이터드리븐 챔프 4종(alchemist/crossbowman/nightmare/sand_mage,
    // 시트 밖 별도 컨테이너·pre_patch 스냅샷에도 부재 = 게임 데이터상 "변화 기록 없음")
    // → 중립 레이더 폴백. 종전엔 hover_diff_miss로 레이더 자체가 안 떴음(07-27 제보).
    let (d, flag) = v.diffs.get(&hv).copied().unwrap_or(([0f32; 8], 0));
    let Some(key) = best_key else {
        return "no_pass";
    };
    let (sw, sh) = state.map_size.get(&key).copied().unwrap_or((1920.0, 1080.0));
    let Some(cmds) = state.commands.get_mut(&key) else {
        return "pass_gone";
    };

    // 타입명 미노출(Color/Rect/Border) 회피 — 기존 커맨드에서 clone 템플릿 확보.
    let mut tcol = None; // Color 템플릿 (Text 에서)
    // 패널이 그린 스탯 아이콘 NinePatch 들 (중심좌표 + 통째 클론 — 라벨과 짝지어 축별 캐시)
    let mut stat_icons: Vec<(f32, f32, RenderCommand)> = Vec::new();
    let mut tbox = None; // RoundingBox 템플릿 (모달 배경용)
    let mut maxz = 0i32;
    for c in cmds.iter() {
        match c {
            RenderCommand::Text { color, z, .. } => {
                if tcol.is_none() {
                    tcol = Some(color.clone());
                }
                if *z > maxz {
                    maxz = *z;
                }
            }
            RenderCommand::NinePatch { texture, x, y, w, h, z, .. } => {
                if texture.contains("champion_stat_icon") && stat_icons.len() < 24 {
                    stat_icons.push((*x + *w * 0.5, *y + *h * 0.5, c.clone()));
                }
                if *z > maxz {
                    maxz = *z;
                }
            }
            RenderCommand::RoundingBox { z, .. } => {
                if tbox.is_none() {
                    tbox = Some(c.clone());
                }
                if *z > maxz {
                    maxz = *z;
                }
            }
            _ => {}
        }
    }
    // 축별 아이콘 템플릿 채우기: 스탯 이름 라벨의 왼쪽 60px 내 가장 가까운 아이콘.
    ICON_TPL.with(|tl| {
        let mut tpl = tl.borrow_mut();
        if tpl.iter().all(|t| t.is_some()) || stat_icons.is_empty() {
            return;
        }
        for (ax, lx, ly) in &stat_labels {
            if tpl[*ax].is_some() {
                continue;
            }
            let mut best: Option<(f32, usize)> = None;
            for (i, (ix, iy, _)) in stat_icons.iter().enumerate() {
                let dx = lx - ix;
                let dy = (ly - iy).abs();
                if dx > 0.0 && dx < 60.0 && dy < 16.0 {
                    let d = dx + dy;
                    if best.map(|(bd, _)| d < bd).unwrap_or(true) {
                        best = Some((d, i));
                    }
                }
            }
            if let Some((_, i)) = best {
                tpl[*ax] = Some(stat_icons[i].2.clone());
            }
        }
    });

    let Some(tc) = tcol else {
        return "no_text_template";
    };
    let mk = |r: f32, g: f32, b: f32, a: f32| {
        let mut c = tc.clone();
        c.r = r;
        c.g = g;
        c.b = b;
        c.a = a;
        c
    };

    let s = if cfg.radar_scale > 0.1 { cfg.radar_scale } else { 1.0 };
    let half = 96.0 * s; // 모달 반너비 (아이콘 r72 + 아이콘 반크기 + 여백)
    let _ = anchor;
    let inside = |b: &(f32, f32, f32, f32), px: f32, py: f32| {
        px >= b.0 && px <= b.0 + b.2 && py >= b.1 && py <= b.1 + b.3
    };
    // 위치: cfg 강제 > **호버 챔프의 그리드 셀(실측 z=100 132×130) 옆** (셀 안 가리게
    //       좌/우 자동) > 툴팁 박스 옆 폴백(모드 챔프 첫 호버 등).
    let cell = cells
        .iter()
        .find(|b| hover_txts.iter().any(|(x, y)| inside(b, *x, *y)))
        .copied();
    let (mut cx, mut cy) = match cell {
        Some((cxr, cyr, cw, ch)) => {
            let right = cxr + cw + 12.0 + half;
            if right + half <= sw - 6.0 {
                (right, cyr + ch * 0.5)
            } else {
                (cxr - 12.0 - half, cyr + ch * 0.5)
            }
        }
        None => {
            let right = bx0 + bw0 + 24.0 + half;
            if right + half <= sw - 6.0 {
                (right, by0 + bh0 * 0.5)
            } else {
                (bx0 - 24.0 - half, by0 + bh0 * 0.5)
            }
        }
    };
    if cfg.radar_x >= 0.0 {
        cx = cfg.radar_x;
    }
    if cfg.radar_y >= 0.0 {
        cy = cfg.radar_y;
    }
    cx = cx.clamp(half + 4.0, sw - half - 4.0);
    cy = cy.clamp(half + 4.0, sh - half - 4.0);
    let ang = |i: usize| -std::f32::consts::FRAC_PI_2 + (i as f32) * std::f32::consts::FRAC_PI_4;
    let z0 = maxz + 1;

    // 모달 배경 (RoundingBox 템플릿 재활용 — 게임 스타일의 라운딩 유지)
    if let Some(tb) = tbox {
        let mut bx = tb.clone();
        if let RenderCommand::RoundingBox { rect, z, color, .. } = &mut bx {
            rect.x = cx - half;
            rect.y = cy - half;
            rect.w = half * 2.0;
            rect.h = half * 2.0;
            *z = z0;
            // 게임 툴팁 배경(#161721)과 겹쳐 안 보인다는 피드백 → 살짝 밝은 인디고로 구분.
            let c = mk(0.125, 0.135, 0.235, 0.94);
            color.r = c.r;
            color.g = c.g;
            color.b = c.b;
            color.a = c.a;
            cmds.push(bx);
        }
    }
    // 외곽 원 (36세그, r60)
    let rc = 60.0 * s;
    for i in 0..36 {
        let a0 = (i as f32) * std::f32::consts::TAU / 36.0;
        let a1 = ((i + 1) as f32) * std::f32::consts::TAU / 36.0;
        cmds.push(RenderCommand::DrawLine {
            from_x: cx + a0.cos() * rc,
            from_y: cy + a0.sin() * rc,
            to_x: cx + a1.cos() * rc,
            to_y: cy + a1.sin() * rc,
            from_color: mk(0.55, 0.60, 0.72, 0.55),
            to_color: mk(0.55, 0.60, 0.72, 0.55),
            width: 1.2,
            z: z0 + 1,
        });
    }
    // 스포크 8 (r58)
    for i in 0..8 {
        let a = ang(i);
        cmds.push(RenderCommand::DrawLine {
            from_x: cx,
            from_y: cy,
            to_x: cx + a.cos() * 58.0 * s,
            to_y: cy + a.sin() * 58.0 * s,
            from_color: mk(0.85, 0.88, 0.95, 0.16),
            to_color: mk(0.85, 0.88, 0.95, 0.16),
            width: 1.0,
            z: z0 + 1,
        });
    }
    // 기준 8각 링 (r33 = 변화없음 기준선)
    let rb = 33.0 * s;
    for i in 0..8 {
        let (a0, a1) = (ang(i), ang(i + 1));
        cmds.push(RenderCommand::DrawLine {
            from_x: cx + a0.cos() * rb,
            from_y: cy + a0.sin() * rb,
            to_x: cx + a1.cos() * rb,
            to_y: cy + a1.sin() * rb,
            from_color: mk(0.95, 0.96, 0.98, 0.95),
            to_color: mk(0.95, 0.96, 0.98, 0.95),
            width: 1.3,
            z: z0 + 2,
        });
    }
    // 스탯 다각형
    let (fr, fg, fb, fa, lr, lg, lb) = match flag {
        1 => (0.34, 0.86, 0.47, 0.28, 0.34, 0.86, 0.47),
        -1 => (0.97, 0.40, 0.42, 0.28, 0.97, 0.40, 0.42),
        _ => (0.45, 0.72, 0.98, 0.16, 0.45, 0.72, 0.98),
    };
    let mut pts: Vec<(f32, f32)> = Vec::with_capacity(8);
    for i in 0..8 {
        let dv = (d[AXIS_DIFF[i]] / 0.15).clamp(-1.0, 1.0);
        let r = (33.0 + dv * if dv < 0.0 { 21.0 } else { 27.0 }) * s;
        let a = ang(i);
        pts.push((cx + a.cos() * r, cy + a.sin() * r));
    }
    cmds.push(RenderCommand::FilledPath {
        points: pts.clone(),
        z: z0 + 3,
        color: mk(fr, fg, fb, fa),
        gradient: None,
    });
    for i in 0..8 {
        let (x0, y0) = pts[i];
        let (x1, y1) = pts[(i + 1) % 8];
        cmds.push(RenderCommand::DrawLine {
            from_x: x0,
            from_y: y0,
            to_x: x1,
            to_y: y1,
            from_color: mk(lr, lg, lb, 1.0),
            to_color: mk(lr, lg, lb, 1.0),
            width: 2.4,
            z: z0 + 4,
        });
    }
    // 스탯 아이콘 8 (r72) — 게임이 실제로 champion_stat_icon 을 그린 NinePatch 를
    // 통째로 복제하고 **텍스처 rect 의 x 슬롯만 바꾼다** (키 형식·rect 단위·크기 전부
    // 게임 것 그대로 = 좌표계 추측 배제). 템플릿이 없으면(패널 미표시 프레임) 스킵.
    // 축별 아이콘 = 패널이 그린 그 스탯의 아이콘 커맨드를 통째 복제 (rect 계산 없음).
    let (drawn_icons, tpl_cnt) = ICON_TPL.with(|tl| {
        let tpl = tl.borrow();
        let mut n = 0usize;
        for i in 0..8 {
            let Some(t) = &tpl[AXIS_DIFF[i]] else {
                continue;
            };
            let a = ang(i);
            let mut np = t.clone();
            if let RenderCommand::NinePatch { x, y, w, h, z, .. } = &mut np {
                let (iw, ih) = (*w, *h);
                *x = cx + a.cos() * 72.0 * s - iw * 0.5;
                *y = cy + a.sin() * 72.0 * s - ih * 0.5;
                *z = z0 + 5;
                cmds.push(np);
                n += 1;
            }
        }
        (n, tpl.iter().filter(|t| t.is_some()).count())
    });
    dlog!(
        "icons drawn={drawn_icons} tpl={tpl_cnt}개 labels={} icons_seen={}",
        stat_labels.len(),
        stat_icons.len()
    );
    dlog!("radar drawn @({cx},{cy}) s={s} z0={z0}");
    "drawn"
}
