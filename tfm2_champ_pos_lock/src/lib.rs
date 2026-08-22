//! tfm2_champ_pos_lock — 특정 챔피언을 특정 포지션에서만 쓰게 제한.
//! ===========================================================================
//! 설정은 인게임 UI(환경설정 → 게임플레이 → '포지션 제한' 버튼)로 한다.
//!   UI 가 포지션별 허용 챔피언 화이트리스트를 champ_pos_lock_state.txt 에 쓰고,
//!   이 파일이 그걸 읽어 챔피언별 허용 포지션 5비트 마스크로 환산해 게이트가 소비.
//!   (포지션 화이트리스트가 비면 그 포지션은 전 챔피언 허용.)
//!
//! 축1 AI 픽 게이트(ModDraftScoreHook.score_pick): 매칭(홀 조건) 깨는 후보 Replace(-1e9).
//! 축2 AI 배정(hookA)·유저 픽 차단(hookC/D): hooks.rs.
//! ===========================================================================

use mod_api::*;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU64, Ordering};
use std::sync::OnceLock;

mod config;
mod hooks;
mod icon_data;
mod inject;

#[path = r"C:\tfm2mods\ui_kit\ui_kit.rs"]
mod ui_kit;

use config::{MASK_ALL, POS_NAMES};
use std::rc::Rc;

pub(crate) const MOD_ID: &str = "tfm2_champ_pos_lock";

// build_inj.ps1 신원 검증용 — dll 안에 lib.rs 절대경로 문자열 필요(stale/타모드 차단).
#[no_mangle]
pub extern "C" fn tfm2_champ_pos_lock_src_id() -> *const u8 {
    concat!(file!(), "\0").as_bytes().as_ptr()
}

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

// ── 챔피언 인덱스 → 이름/마스크 ─────────────────────────────────────────────
// ctx candidate 인덱스 = db.available_champions 순서(모드 챔프는 그 뒤) 전제.
// NAMES 는 1회 캡처(경기 중 불변), MASKS 는 상태(UI 편집) 버전이 바뀔 때 재계산.
pub(crate) static NAMES: OnceLock<Vec<String>> = OnceLock::new(); // 원본 표기(UI/덤프용)
/// 서버 Database 포인터(= GamePlayOption base, +0x0). InGame 프레임마다 갱신.
/// 룰 읽기: ban_count=*(p+0x720)u64, banpick_style=*(p+0x737)u8 (game-atlas 2026-08-20).
static DB_PTR: AtomicUsize = AtomicUsize::new(0);
static IDX_MASK: AtomicPtr<Vec<u8>> = AtomicPtr::new(core::ptr::null_mut());
static APPLIED_VER: AtomicU64 = AtomicU64::new(u64::MAX);

/// 인덱스별 마스크 스냅샷(락 없음 — 버전 변경 시 통째 교체·이전 것 leak=드묾).
pub fn masks() -> Option<&'static Vec<u8>> {
    let p = IDX_MASK.load(Ordering::Acquire);
    if p.is_null() {
        None
    } else {
        Some(unsafe { &*p })
    }
}
fn publish_masks(v: Vec<u8>) {
    let p = Box::into_raw(Box::new(v));
    IDX_MASK.store(p, Ordering::Release);
}
/// 사용 가능 챔피언 목록(NAMES) 노출 — UI 가 그리드를 채울 때 사용.
pub fn champ_names() -> Option<&'static Vec<String>> {
    NAMES.get()
}

// ── 설정 팝업 그리드: 한글 이름 가나다순 정렬 ─────────────────────────────
//   라벨은 i18n 태그(`#asset/...?description.{id}.name`)로 넘겨 게임이 해석하므로
//   모드는 한글 문자열을 모른다 ⟹ 정렬하려면 i18n 을 우리가 읽어야 한다.
//   게임 자산은 bundle.game_data(1.1GB, 포맷 미해독)라 **언팩/모드 i18n 파일들을 병합**한다.
//   실측(2026-08-22): base+mods+workshop 병합 시 94/94 커버(누락 0).
//   ★한글 음절은 유니코드가 곧 가나다순이라 단순 문자열 비교로 정렬된다.
static KR_MAP: OnceLock<std::collections::HashMap<String, String>> = OnceLock::new();
static SORTED_CHAMPS: OnceLock<Vec<String>> = OnceLock::new();

/// 게임 설치 폴더(exe 위치) — 경로 하드코딩 금지 규칙에 따라 동적 도출.
fn game_dir() -> Option<std::path::PathBuf> {
    let mut buf = [0u16; 512];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), 512) } as usize;
    if n == 0 || n >= 512 {
        return None;
    }
    let exe = String::from_utf16_lossy(&buf[..n]);
    Some(std::path::Path::new(&exe).parent()?.to_path_buf())
}

/// champion.i18n 텍스트에서 ko.description.<id>.name 을 뽑아 out 에 병합.
/// (외부 크레이트 없이 중괄호 깊이 추적으로 스캔 — 로케일 오인식 방지)
fn parse_ko_names(text: &str, out: &mut std::collections::HashMap<String, String>) {
    let Some(ko) = text.find("\"ko\"") else { return };
    let Some(drel) = text[ko..].find("\"description\"") else { return };
    let dpos = ko + drel;
    let Some(obr) = text[dpos..].find('{') else { return };
    let start = dpos + obr;
    let b = text.as_bytes();
    // description 오브젝트의 끝(균형 잡힌 '}') 찾기
    let (mut depth, mut in_str, mut esc, mut end) = (0usize, false, false, b.len());
    for i in start..b.len() {
        let c = b[i];
        if in_str {
            if esc {
                esc = false;
            } else if c == b'\\' {
                esc = true;
            } else if c == b'"' {
                in_str = false;
            }
            continue;
        }
        match c {
            b'"' => in_str = true,
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    end = i;
                    break;
                }
            }
            _ => {}
        }
    }
    // description 내부를 깊이 추적하며 (챔프id, name) 추출
    let seg = &text[start..end.min(text.len())];
    let sb = seg.as_bytes();
    let (mut d, mut i) = (0usize, 0usize);
    let mut cur_id: Option<String> = None;
    let mut last_key = String::new();
    while i < sb.len() {
        match sb[i] {
            b'"' => {
                // 문자열 읽기
                let mut j = i + 1;
                let mut val = String::new();
                let mut e = false;
                while j < sb.len() {
                    let c = sb[j];
                    if e {
                        val.push(c as char);
                        e = false;
                    } else if c == b'\\' {
                        e = true;
                    } else if c == b'"' {
                        break;
                    } else {
                        val.push(c as char);
                    }
                    j += 1;
                }
                let raw = &seg[i + 1..j.min(seg.len())];
                // 키인지 값인지: 다음 비공백이 ':' 이면 키
                let mut k = j + 1;
                while k < sb.len() && (sb[k] as char).is_whitespace() {
                    k += 1;
                }
                let is_key = k < sb.len() && sb[k] == b':';
                if is_key {
                    last_key = raw.to_string();
                } else if d == 2 && last_key == "name" {
                    if let Some(id) = cur_id.clone() {
                        out.insert(id.to_ascii_lowercase(), raw.to_string());
                    }
                }
                i = j + 1;
                continue;
            }
            b'{' => {
                d += 1;
                if d == 2 {
                    cur_id = Some(last_key.clone()); // 방금 본 키가 챔프 id
                }
            }
            b'}' => {
                if d == 2 {
                    cur_id = None;
                }
                d = d.saturating_sub(1);
            }
            _ => {}
        }
        i += 1;
    }
}

/// base/mods/workshop 의 champion.i18n 을 전부 병합해 id→한글이름 맵 생성.
fn load_kr_names() -> std::collections::HashMap<String, String> {
    let mut out = std::collections::HashMap::new();
    let Some(g) = game_dir() else { return out };
    let mut files: Vec<std::path::PathBuf> = Vec::new();
    files.push(g.join("bundle_unpacked_full").join("text").join("champion.i18n"));
    // mods/*/text|data/champion.i18n
    if let Ok(rd) = std::fs::read_dir(g.join("mods")) {
        for e in rd.flatten() {
            for sub in ["text", "data"] {
                files.push(e.path().join(sub).join("champion.i18n"));
            }
        }
    }
    // <steamapps>/workshop/content/3009300/*/text/champion.i18n
    //   g = ...\steamapps\common\Teamfight Manager2 → 2단계 위가 steamapps
    if let Some(steamapps) = g.parent().and_then(|p| p.parent()) {
        let ws = steamapps.join("workshop").join("content").join("3009300");
        if let Ok(rd) = std::fs::read_dir(&ws) {
            for e in rd.flatten() {
                files.push(e.path().join("text").join("champion.i18n"));
            }
        }
    }
    for f in files {
        if let Ok(t) = std::fs::read_to_string(&f) {
            parse_ko_names(&t, &mut out);
        }
    }
    out
}

/// 설정 팝업용: 한글 이름 가나다순으로 정렬된 챔피언 id 목록.
///   이름을 못 찾은 챔프는 뒤로 보내고 id 순으로(표시는 되되 순서만 뒤).
pub fn sorted_champs() -> Option<&'static Vec<String>> {
    if let Some(v) = SORTED_CHAMPS.get() {
        return Some(v);
    }
    let champs = NAMES.get()?;
    // ★1순위: 게임 밴픽 그리드 순회 순서(=게임이 쓰는 가나다순). 밴픽을 한 번 지나가면
    //   수집되고 파일로 캐시된다 ⟹ 패치로 챔프가 추가돼도 자동으로 따라감.
    let mut order: Vec<String> = hooks::grid_order();
    if order.len() < 30 {
        if let Some(dir) = mod_dir() {
            if let Ok(t) = std::fs::read_to_string(format!("{dir}\\champ_pos_lock_order.txt")) {
                let f: Vec<String> =
                    t.lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect();
                if f.len() > order.len() {
                    order = f;
                }
            }
        }
    }
    let rank = |id: &str| -> Option<usize> {
        order.iter().position(|n| n.eq_ignore_ascii_case(id))
    };
    // 2순위: i18n 한글 이름(파일 병합). 3순위: id.
    let kr = KR_MAP.get_or_init(load_kr_names);
    let mut v: Vec<String> = champs.clone();
    v.sort_by(|a, b| {
        match (rank(a), rank(b)) {
            (Some(x), Some(y)) => return x.cmp(&y),
            (Some(_), None) => return std::cmp::Ordering::Less,
            (None, Some(_)) => return std::cmp::Ordering::Greater,
            (None, None) => {}
        }
        let (ka, kb) = (kr.get(&a.to_ascii_lowercase()), kr.get(&b.to_ascii_lowercase()));
        match (ka, kb) {
            (Some(x), Some(y)) => x.cmp(y),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.cmp(b),
        }
    });
    if config::get().debug {
        let head: Vec<&str> = v.iter().take(8).map(|s| s.as_str()).collect();
        config::dlog(&format!(
            "sort_kr: names={} order={} kr_map={} head={:?}",
            v.len(),
            order.len(),
            kr.len(),
            head
        ));
    }
    let _ = SORTED_CHAMPS.set(v);
    SORTED_CHAMPS.get()
}

static CNT_VETO: AtomicU64 = AtomicU64::new(0);
static CNT_SEEN: AtomicU64 = AtomicU64::new(0);
static CNT_FAILOPEN: AtomicU64 = AtomicU64::new(0);
static FLUSH_TICK: AtomicU64 = AtomicU64::new(0);
/// score_pick raw-ctx 오프셋 진단 throttle(총 라인 상한).
static DBG_CTXN: AtomicU64 = AtomicU64::new(0);
/// veto 시점 ally vs 유저 씬 대조 로그 throttle.
static DBG_VETON: AtomicU64 = AtomicU64::new(0);
/// 유저 씬 활성 중 score_pick 상태 로그 throttle.
static DBG_LIVEN: AtomicU64 = AtomicU64::new(0);
/// veto 조기반환 단계 카운터(어디서 새는지 확정용).
static ST_EMPTY: AtomicU64 = AtomicU64::new(0);
static ST_FEAS: AtomicU64 = AtomicU64::new(0);
static ST_BROKEN: AtomicU64 = AtomicU64::new(0);
static DBG_FEASN: AtomicU64 = AtomicU64::new(0);
static ST_CONF: AtomicU64 = AtomicU64::new(0);
static DBG_CONFN: AtomicU64 = AtomicU64::new(0);
static DBG_MULTIN: AtomicU64 = AtomicU64::new(0);
static DBG_TIP: AtomicU64 = AtomicU64::new(0);
/// 지금 보이는 툴팁이 우리가 띄운 것인지(해제 시 게임 툴팁을 잘못 숨기지 않기 위해).
static TIP_OURS: AtomicBool = AtomicBool::new(false);

/// raw ctx base+off 의 (ptr@off, len@off+8)를 u64 인덱스 배열로 읽어 이름 문자열로.
/// RE(2026-08-21): ctx+0x10=ally픽·+0x20=enemy픽 추정 — 실제 오프셋 확정용 진단.
fn dump_vec_names(base: usize, off: usize) -> String {
    unsafe {
        if !(0x10000..1usize << 48).contains(&base) {
            return "<badbase>".into();
        }
        let ptr = core::ptr::read((base + off) as *const usize);
        let len = core::ptr::read((base + off + 8) as *const usize);
        if len == 0 {
            return "-".into();
        }
        if !(0x10000..1usize << 48).contains(&ptr) || len > 16 {
            return format!("<p={ptr:#x} n={len}>");
        }
        let nm = |i: usize| NAMES.get().and_then(|v| v.get(i)).map(|s| s.as_str()).unwrap_or("?");
        let mut out = Vec::with_capacity(len);
        for k in 0..len {
            let v = core::ptr::read((ptr + k * 8) as *const u64) as usize;
            out.push(format!("{}#{v}", nm(v)));
        }
        out.join(",")
    }
}

/// ★권위 있는 아군 픽 = raw ctx+0x10(begin)/+0x18(len), u64 인덱스 배열.
/// SDK DraftScoreContext.ally_pick 은 +0x30(빈 오프셋)으로 매핑돼 항상 비어 있어 오염원이었음
/// (ctxdump 진단으로 SDK ab==raw@10==실제 아군픽 확정, 2026-08-21). 매치별로 정확·씬 불요.
fn raw_ally_idx(ctx: &DraftScoreContext) -> Vec<usize> {
    unsafe {
        let base = ctx as *const DraftScoreContext as usize;
        if !(0x10000..1usize << 48).contains(&base) {
            return Vec::new();
        }
        let ptr = core::ptr::read((base + 0x10) as *const usize);
        let len = core::ptr::read((base + 0x18) as *const usize);
        if len == 0 || len > 16 || !(0x10000..1usize << 48).contains(&ptr) {
            return Vec::new();
        }
        (0..len)
            .map(|k| core::ptr::read((ptr + k * 8) as *const u64) as usize)
            .collect()
    }
}

/// 남은 available 중 아군에 feasible 하게 추가되는 챔프가 하나라도 있나(=fail-open 판정).
/// ★raw ctx+0x00/+0x08 = available begin/count (SDK ctx.available_champions 는 필드 오매핑
///   위험이 확인된 소스라 사용 금지 — ally_pick 이 이미 그렇게 틀렸다, ctxdump 2026-08-21).
///   available 가 안 읽히면 **보수적으로 true**(=게이트 유지·veto 살림). 무제한 챔프 있으면 true.
fn pool_has_feasible_idx(
    ctx: &DraftScoreContext,
    ally_idx: &[usize],
    masks: &[u8],
    pinned: &[u8],
) -> bool {
    let avail: Vec<usize> = unsafe {
        let base = ctx as *const DraftScoreContext as usize;
        if !(0x10000..1usize << 48).contains(&base) {
            return true;
        }
        let ptr = core::ptr::read(base as *const usize);
        let len = core::ptr::read((base + 8) as *const usize);
        if len == 0 || len > 4096 || !(0x10000..1usize << 48).contains(&ptr) {
            return true; // 보수적: 못 읽으면 게이트 유지
        }
        (0..len)
            .map(|k| core::ptr::read((ptr + k * 8) as *const u64) as usize)
            .collect()
    };
    let mut v: Vec<u8> = Vec::with_capacity(pinned.len() + 1);
    for &c in &avail {
        if ally_idx.contains(&c) {
            continue;
        }
        let m = masks.get(c).copied().unwrap_or(MASK_ALL);
        if m == MASK_ALL {
            return true;
        }
        v.clear();
        v.extend_from_slice(pinned);
        v.push(m);
        if feasible(&mut v) {
            return true;
        }
    }
    false
}

// UI: '포지션 제한' 버튼 클릭 라우팅 + 팝업 열림 상태.
use std::sync::atomic::AtomicUsize;
static CLICK_LAST: AtomicUsize = AtomicUsize::new(usize::MAX);
static POPUP_OPEN: AtomicBool = AtomicBool::new(false);
static CNT_ROW_CLICK: AtomicU64 = AtomicU64::new(0);
/// 현재 선택된 포지션 탭: 0=탑 1=정글 2=미드 3=원딜 4=서폿 (전체 탭 없음 — 유저 지시)
static SEL_POS: AtomicUsize = AtomicUsize::new(0);
static ICON_SDK: AtomicU64 = AtomicU64::new(0);
static ICON_FB: AtomicU64 = AtomicU64::new(0);
const NCELLS: usize = 120;
const TAB_IDS: [&str; 5] = ["tab_top", "tab_jungle", "tab_mid", "tab_bottom", "tab_support"];
static GRID_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
static mut CELL_BUF: [[u8; 96]; NCELLS] = [[0u8; 96]; NCELLS];

// 챔피언 스프라이트 에셋 키 + UV 폴백 테이블(CHAMP_UV/CHAMP_KEY — icon_data 미커버분용).
include!(r"C:\tfm2mods\tfm2_champ_pos_lock\assets\champ_uv.rs");
fn champ_key(id: &str) -> Option<String> {
    CHAMP_KEY.iter().find(|e| e.0 == id).map(|e| e.1.to_string())
}
/// ImageRunner 커스텀 UV: +0xa4 flag=1, +0xa8..0xb4 = 4개 f32.
unsafe fn set_img_uv(dp: usize, a: f32, b: f32, c: f32, d: f32) {
    *((dp + 0xa4) as *mut u8) = 1;
    *((dp + 0xa8) as *mut f32) = a;
    *((dp + 0xac) as *mut f32) = b;
    *((dp + 0xb0) as *mut f32) = c;
    *((dp + 0xb4) as *mut f32) = d;
}
/// 노드 크기(복사본 6곳).
fn set_node_wh(node: &Node, w: f32, h: f32) {
    let na = node as *const Node as usize;
    for off in [0x74usize, 0xf4, 0x174, 0x1f4, 0x248, 0x258] {
        ui_kit::runner_wr_f32(na, off, w);
    }
    for off in [0x7cusize, 0xfc, 0x17c, 0x1fc, 0x24c, 0x25c] {
        ui_kit::runner_wr_f32(na, off, h);
    }
}

/// 셀 아이콘 = 챔피언 전신 스프라이트.
/// 챔피언 전신 스프라이트.
/// ①공식 함수 = Hook E 가 관리화면에서 캡처한 "로드된 Assets"로 set_champion_icon_center 호출
///   → raw-aseprite 모드챔프 포함 전부 게임이 직접 크롭/패킹 = 완전 동일. (관리 컨텍스트에서만
///   assets 캡처됨. 메인메뉴 등 미캡처 시 ②로.)
/// ②재현 = 라이브 bundle/팩에서 직접 계산(게임 캡처 UV와 픽셀 일치·box 100×94·scale 2).
///   단 raw-aseprite 10종은 런타임 패킹 재현 한계로 미세오차 가능(그래서 ①우선).
unsafe fn set_cell_icon(icon: &mut Node, k: usize, lower: &str) {
    if k >= NCELLS {
        return;
    }
    // ① 캡처된 로드-assets + 게임 세터(FUN_14250bc30) 직접 호출 = 게임정보 탭과 동일 동작.
    //   SDK 래퍼(set_champion_icon_center)는 Rust Assets 레이아웃 가정이 raw 게임 포인터와
    //   달라 no-op였음 → 게임 함수 자체를 그 ABI대로 호출(assets=rcx, node=rdx, id=r8/r9,
    //   box 100×94·scale2 = custom_champion_slot 규칙). raw-aseprite 10종 포함 전부 게임 그대로.
    let ga = hooks::game_assets();
    let setter = hooks::icon_setter_fn();
    if setter != 0 && (0x10000..1usize << 48).contains(&ga) {
        let node_ptr = icon as *mut Node as usize;
        let id_ptr = lower.as_ptr() as usize;
        let id_len = lower.len();
        let f: extern "C" fn(usize, usize, usize, usize, f32, f32, f32) =
            core::mem::transmute(setter);
        let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            f(ga, node_ptr, id_ptr, id_len, 100.0, 94.0, 2.0);
        }))
        .is_ok();
        if ok {
            if let Some(dp) = ui_kit::runner_base(icon, "ImageRunner") {
                if core::ptr::read_unaligned((dp + 0x10) as *const u64) > 0 {
                    ICON_SDK.fetch_add(1, Ordering::Relaxed);
                    return; // 게임이 소스+UV+노드 세팅 완료
                }
            }
        }
    }
    ICON_FB.fetch_add(1, Ordering::Relaxed);
    // ② 재현 폴백
    let Some(dp) = ui_kit::runner_base(icon, "ImageRunner") else {
        return;
    };
    if let Some((uv, key)) = icon_data::get(lower) {
        let full = format!("{key}#sheet");
        let kb = full.as_bytes();
        if kb.len() <= 96 {
            let buf = core::ptr::addr_of_mut!(CELL_BUF[k]) as *mut u8;
            core::ptr::copy_nonoverlapping(kb.as_ptr(), buf, kb.len());
            core::ptr::write_unaligned(dp as *mut u64, 0u64);
            core::ptr::write_unaligned((dp + 0x08) as *mut u64, buf as u64);
            core::ptr::write_unaligned((dp + 0x10) as *mut u64, kb.len() as u64);
            core::ptr::write_unaligned((dp + 0x18) as *mut i64, -1i64);
            set_img_uv(dp, uv.u0, uv.v0, uv.uw, uv.vh);
            // uv.w/uv.h = 게임 실측 노드(h≈94, w=2×fw). 셀을 게임과 동일한 152×171 로 만들었으니
            //   게임 값 그대로 사용 = 게임 밴픽/시작챔프 UI와 완전 동일한 표시.
            set_node_wh(icon, uv.w, uv.h);
            return;
        }
    }
    // 둘 다 실패 → 깨짐 대신 아이콘 숨김(이름은 표시).
    *((dp + 0xa4) as *mut u8) = 0;
    core::ptr::write_unaligned((dp + 0x10) as *mut u64, 0u64);
}

/// 노드 높이(복사본 6곳) — 스크롤 컨텐츠 높이 지정용.
fn set_node_h(node: &Node, h: f32) {
    let na = node as *const Node as usize;
    for off in [0x7cusize, 0xfc, 0x17c, 0x1fc, 0x24c, 0x25c] {
        ui_kit::runner_wr_f32(na, off, h);
    }
}

/// 게임 룰 설정 읽기 → (피어리스 스타일 0=없음/1=피어리스/2=하드, 밴카드 수).
/// GamePlayOption(=Database+0x0): ban_count=*(+0x720)u64, banpick_style=*(+0x737)u8
/// (game-atlas 2026-08-20, 0.5.0 채록). ⚠오프셋/enum값 0.5.6 미검증 → 범위 검증으로 가드,
/// 벗어나면 (0,0)=안전값. Database 포인터는 InGame 프레임마다 DB_PTR 에 캡처.
fn rule_info() -> (u8, usize) {
    let p = DB_PTR.load(Ordering::Relaxed);
    if !(0x10000..1usize << 48).contains(&p) {
        return (0, 0);
    }
    unsafe {
        let ban = core::ptr::read_unaligned((p + 0x720) as *const u64);
        // banpick_style = +0x737 (0.5.6 커밋-diff 확정: 룰 변경 후 경기 1판 진행해야 db에 커밋됨.
        //   그 뒤 0x737 이 0=클래식/1=피어리스/2=하드피어리스). 미커밋 시엔 0(클래식 기본).
        let style = core::ptr::read_unaligned((p + 0x737) as *const u8);
        if ban > 5 || style > 2 {
            return (0, 0); // 오프셋 미스/이상값 — 안전값
        }
        (style, ban as usize)
    }
}

/// 팝업 그리드를 현재 탭·상태에 맞게 채운다(변경 시에만).
fn fill_grid(root: &mut Node) {
    let pos = SEL_POS.load(Ordering::Relaxed);
    let ver = config::state_version();
    let Some(champs) = sorted_champs() else { return };
    let ready = icon_data::READY.load(Ordering::Relaxed) as u64;
    let (style, ban_count) = rule_info();
    let rule_sig = ((style as u64) << 40) ^ ((ban_count as u64) << 32);
    let sig = (pos as u64) << 48 ^ ver ^ ((champs.len() as u64) << 32) ^ (ready << 47) ^ rule_sig;
    if GRID_SIG.swap(sig, Ordering::Relaxed) == sig {
        return;
    }
    let Some(pop) = ui_kit::find_mut(root, "pos_lock_popup") else {
        return;
    };
    // 탭 하이라이트
    for (i, t) in TAB_IDS.iter().enumerate() {
        ui_kit::toggle_set_by_id(pop, t, i == pos);
    }
    // 요약 라벨 + 카운트(밴카드 / 최소 / 현재 선택)
    let cnt = config::pos_count(pos);
    let base_need = config::min_required(style, ban_count); // 이 포지션 자체 필요
    // 겹침: 공유풀(컴포넌트 합집합) ≥ 픽수요×포지션수 + 밴 이어야 함.
    let (comp, union_size) = config::overlap_component(pos);
    let comp_size = comp.len().max(1);
    let ban_part = ban_count * 2;
    let pick_part = base_need.saturating_sub(ban_part); // SERIES 부분(하드10/피어5/클2)
    let union_need = pick_part * comp_size + ban_part; // 공유풀이 넘어야 할 값
    if let Some(n) = ui_kit::find_mut(pop, "summary") {
        ui_kit::label_set(n, &format!("{} 포지션", POS_NAMES_KR[pos]));
    }
    if let Some(n) = ui_kit::find_mut(pop, "rule_label") {
        let name = match style {
            2 => "하드 피어리스",
            1 => "피어리스",
            _ => "없음 (클래식)",
        };
        ui_kit::label_set(n, &format!("현재 밴픽 룰: {name}"));
    }
    if let Some(n) = ui_kit::find_mut(pop, "ban_label") {
        ui_kit::label_set(n, &format!("현재 밴카드 수: {ban_count}"));
    }
    if let Some(n) = ui_kit::find_mut(pop, "min_label") {
        let s = if comp_size > 1 {
            format!(
                "최소 선택 수: {base_need}  ·  공유풀({comp_size}포지션) {union_size}/{union_need}"
            )
        } else {
            format!("최소 선택 수: {base_need}")
        };
        ui_kit::label_set(n, &s);
    }
    if let Some(n) = ui_kit::find_mut(pop, "count_label") {
        let s = if cnt == 0 {
            "현재 선택 수: 0 (모든 챔피언 허용)".to_string()
        } else {
            format!("현재 선택 수: {cnt}")
        };
        ui_kit::label_set(n, &s);
    }
    if let Some(n) = ui_kit::find_mut(pop, "warning_min") {
        // cnt==0 = 전체 허용(제한 없음) → 경고 없음. 아니면 두 제약 체크:
        //  ①이 포지션 자체 ≥ base_need  ②겹친 컴포넌트 공유풀 ≥ union_need.
        let s = if cnt == 0 {
            String::new()
        } else if cnt < base_need {
            format!(
                "⚠ 이 포지션 최소 {base_need}개 필요 — {}개 더 선택하세요",
                base_need - cnt
            )
        } else if comp_size > 1 && union_size < union_need {
            format!(
                "⚠ 겹친 {comp_size}개 포지션이 챔프풀 공유 — 합쳐서 최소 {union_need}개 필요 (현재 {union_size})"
            )
        } else {
            String::new()
        };
        ui_kit::label_set(n, &s);
    }
    // 그리드 셀
    let Some(contents) = ui_kit::find_mut(pop, "contents") else {
        return;
    };
    let dbg = config::get().debug;
    let mut sample = String::new();
    ICON_SDK.store(0, Ordering::Relaxed);
    ICON_FB.store(0, Ordering::Relaxed);
    for (k, cell) in contents.child.iter_mut().enumerate() {
        if let Some(champ) = champs.get(k) {
            cell.visible = true;
            let lower = champ.to_ascii_lowercase();
            let listed = config::is_listed(pos, &lower);
            for c in cell.child.iter_mut() {
                match c.id.as_str() {
                    "name" => {
                        // i18n 태그로 세팅 → 게임이 로케일 문자열로 자동 해석(전 챔프·라이브).
                        ui_kit::label_set(
                            c,
                            &format!("#asset/base/text/champion?description.{lower}.name"),
                        );
                    }
                    "sel" => c.visible = listed,
                    "icon" => unsafe {
                        set_cell_icon(c, k, &lower);
                        let _ = (dbg, &mut sample);
                    },
                    _ => {}
                }
            }
        } else {
            cell.visible = false;
        }
    }
    if dbg {
        config::dlog(&format!(
            "icons: game_assets={:#x} sdk={} fallback={}",
            hooks::game_assets(),
            ICON_SDK.load(Ordering::Relaxed),
            ICON_FB.load(Ordering::Relaxed)
        ));
    }
    // 스크롤: 컨텐츠 높이를 보이는 셀 수로 지정(게임과 동일 셀 152×171, 세로간격 15, 7열).
    //   (contents 폭 1154 / (152+15) → 정확히 7열: 7·152+6·15=1154.)
    let n = champs.len().min(NCELLS);
    let rows = n.div_ceil(7);
    let h = (rows as f32) * (171.0 + 15.0) + 16.0;
    set_node_h(contents, h);
    if dbg {
        config::dlog(&format!("grid fill: h={h} icons={sample}"));
    }
}

pub(crate) const POS_NAMES_KR: [&str; 5] = ["탑", "정글", "미드", "원딜", "서폿"];

fn mask_str(m: u8) -> String {
    (0..5)
        .filter(|p| m & (1 << p) != 0)
        .map(|p| POS_NAMES[p])
        .collect::<Vec<_>>()
        .join(",")
}

/// 제한 챔프 마스크들에 서로 다른 포지션을 하나씩 줄 수 있는가 (5×5 이하 백트래킹).
/// ★최대 이분매칭 크기(마스크 ≤6, 포지션 5) — "이미 깨진" 상태에서도 판정 가능.
///   feasible(=완전매칭)은 한 번 깨지면 전 후보가 false 라 fail-open 으로 무력화됐다
///   (실측: 미드 2명 픽 후 block=0). 최대매칭은 그 뒤에도 "빈 라인을 채우는 픽"을 구분한다.
/// 챔프 id → 한글 이름(i18n 병합 맵). 없으면 id 그대로.
pub(crate) fn kr_name(id: &str) -> String {
    let kr = KR_MAP.get_or_init(load_kr_names);
    kr.get(&id.to_ascii_lowercase())
        .cloned()
        .unwrap_or_else(|| id.to_string())
}

/// 각 챔프에게 서로 다른 포지션을 하나씩 배정(완전 매칭). 불가하면 None.
///   백트래킹 — 최대 5개라 비용 무시 가능.
pub(crate) fn assign_positions(masks: &[u8]) -> Option<Vec<usize>> {
    fn go(masks: &[u8], i: usize, used: u8, out: &mut Vec<usize>) -> bool {
        if i == masks.len() {
            return true;
        }
        for p in 0..5 {
            let bit = 1u8 << p;
            if masks[i] & bit != 0 && used & bit == 0 {
                out.push(p);
                if go(masks, i + 1, used | bit, out) {
                    return true;
                }
                out.pop();
            }
        }
        false
    }
    let mut out = Vec::with_capacity(masks.len());
    if go(masks, 0, 0, &mut out) { Some(out) } else { None }
}

/// ★스왑 order 최적해. `order[포지션] = 픽 인덱스`.
///   5! = 120 순열 전수 탐색으로 다음 우선순위로 고른다:
///     ①우리 설정에 맞게 앉은 인원 수(최대) — 완전 배정이 불가능해도 **최대한 맞춘다**
///     ②게임이 이미 계산해 둔 order 와의 일치 수(최대) — 선택지가 여러 개면
///       **게임의 원래 우선순위(스왑 점수)를 따른다**
///   반환 = (order, 맞게 앉은 인원 수)
/// 씬의 팀 픽 → 우리 마스크 배열.
/// ★확정 버튼이 눌린 순간 **상대 팀 order** 를 우리 배정으로 맞춘다(훅 SC 에서 호출).
///   완전 배정이 안 되면 최대한 맞추고, 선택지가 여러 개면 게임의 원래 order 와 최대한 일치시킨다.
pub(crate) fn apply_opponent_swap() {
    if config::get().swap_force == 0 {
        return;
    }
    let (scene, _) = hooks::scene_cap();
    if scene < 0x10000 {
        return;
    }
    // 내 팀의 반대쪽.
    let t2 = MY_IS_T2.load(Ordering::Relaxed);
    let (off, vo) = if t2 {
        (0x198usize, hooks::O_PICK1)
    } else {
        (0x1b0usize, hooks::O_PICK2)
    };
    let Some((cur, want, n)) = swap_plan(scene, off, vo) else {
        return;
    };
    if cur == want {
        return;
    }
    let ptr = unsafe { hooks::ru64(scene + off + 8) } as usize;
    if ptr < 0x10000 {
        return;
    }
    for (i, &w) in want.iter().enumerate() {
        let a = ptr + i * 8;
        if hooks::ptr_ok(a) {
            unsafe { core::ptr::write(a as *mut u64, w) };
        }
    }
    config::llog(&format!("swapset(상대): +{off:x} {cur:?} -> {want:?} (맞춘수={n}/5)"));
}

pub(crate) fn swap_masks(scene: usize, pick_off: usize) -> Option<Vec<u8>> {
    let picks = unsafe { hooks::read_scene_vec(scene, pick_off) }?;
    if picks.len() != 5 {
        return None;
    }
    Some(picks.iter().map(|n| config::mask_of(n)).collect())
}

/// (현재 order, 최적 order, 최적에서 맞게 앉는 인원 수). 재료가 부족하면 None.
pub(crate) fn swap_plan(scene: usize, ord_off: usize, pick_off: usize) -> Option<(Vec<u64>, Vec<u64>, usize)> {
    let ptr = unsafe { hooks::ru64(scene + ord_off + 8) } as usize;
    let len = unsafe { hooks::ru64(scene + ord_off + 0x10) };
    if len != 5 || ptr < 0x10000 {
        return None;
    }
    let cur: Vec<u64> = (0..5).map(|i| unsafe { hooks::ru64(ptr + i * 8) }).collect();
    if cur.iter().any(|&v| v >= 5) {
        return None; // 순열이 아님(과도기/해제된 메모리)
    }
    let masks = swap_masks(scene, pick_off)?;
    let (want, n) = best_order(&masks, &cur);
    Some((cur, want, n))
}

pub(crate) fn best_order(masks: &[u8], cur: &[u64]) -> (Vec<u64>, usize) {
    fn fits(m: u8, p: usize) -> bool {
        m == MASK_ALL || m & (1 << p) != 0
    }
    let n = masks.len().min(5);
    let mut idx: Vec<usize> = (0..n).collect();
    let mut best: (usize, usize, Vec<u64>) = (0, 0, (0..n as u64).collect());
    // 순열 생성(Heap's algorithm)
    fn perm(k: usize, idx: &mut Vec<usize>, masks: &[u8], cur: &[u64], best: &mut (usize, usize, Vec<u64>)) {
        if k == 1 {
            let n = idx.len();
            let matched = (0..n).filter(|&p| fits(masks[idx[p]], p)).count();
            let agree = (0..n).filter(|&p| cur.get(p) == Some(&(idx[p] as u64))).count();
            if (matched, agree) > (best.0, best.1) {
                *best = (matched, agree, idx.iter().map(|&x| x as u64).collect());
            }
            return;
        }
        for i in 0..k {
            perm(k - 1, idx, masks, cur, best);
            if k % 2 == 0 {
                idx.swap(i, k - 1);
            } else {
                idx.swap(0, k - 1);
            }
        }
    }
    if n > 0 {
        perm(n, &mut idx, masks, cur, &mut best);
    }
    (best.2, best.0)
}

/// 지금 order 로 몇 명이 우리 설정에 맞게 앉았는지.
pub(crate) fn order_matched(masks: &[u8], ord: &[u64]) -> usize {
    (0..masks.len().min(5))
        .filter(|&p| {
            let m = masks[ord[p] as usize];
            m == MASK_ALL || m & (1 << p) != 0
        })
        .count()
}

pub(crate) fn max_match(masks: &[u8]) -> usize {
    fn rec(ms: &[u8], used: u8) -> usize {
        let Some((&m, rest)) = ms.split_first() else {
            return 0;
        };
        let mut best = rec(rest, used); // 이 챔프를 배정 안 함
        for p in 0..5u8 {
            let b = 1u8 << p;
            if m & b != 0 && used & b == 0 {
                let v = 1 + rec(rest, used | b);
                if v > best {
                    best = v;
                }
            }
        }
        best
    }
    rec(masks, 0)
}

/// cand 를 추가하면 커버되는 포지션 수가 늘어나는가(= 새 라인을 채우는가).
///   늘지 않으면 "이미 있는 라인과 중복" → 차단 대상.
pub(crate) fn helps(pinned: &[u8], cand: u8) -> bool {
    let base = max_match(pinned);
    let mut v = pinned.to_vec();
    v.push(cand);
    max_match(&v) > base
}

pub(crate) fn feasible(masks: &mut Vec<u8>) -> bool {
    if masks.len() > 5 {
        return false;
    }
    masks.sort_by_key(|m| m.count_ones());
    fn rec(ms: &[u8], used: u8) -> bool {
        let Some((&m, rest)) = ms.split_first() else {
            return true;
        };
        for p in 0..5u8 {
            let b = 1u8 << p;
            if m & b != 0 && used & b == 0 && rec(rest, used | b) {
                return true;
            }
        }
        false
    }
    rec(masks, 0)
}

fn taken(c: usize, ctx: &DraftScoreContext) -> bool {
    ctx.ally_ban.contains(&c)
        || ctx.enemy_ban.contains(&c)
        || ctx.ally_pick.contains(&c)
        || ctx.enemy_pick.contains(&c)
}

/// ★fail-open: 남은 풀에 "픽해도 매칭 유지되는" 후보가 하나도 없으면 게이트 해제.
fn pool_has_feasible(ctx: &DraftScoreContext, masks: &[u8], pinned: &[u8]) -> bool {
    use std::cell::Cell;
    thread_local! {
        static CACHE: Cell<(u64, bool)> = const { Cell::new((0, false)) };
    }
    let mut key: u64 = 0xcbf29ce484222325;
    let mut mix = |v: u64| {
        key ^= v;
        key = key.wrapping_mul(0x100000001b3);
    };
    for &i in ctx.ally_pick {
        mix(i as u64 + 1);
    }
    mix(0x5eed ^ (ctx.ally_ban.len() as u64) << 8 ^ (ctx.enemy_ban.len() as u64));
    mix(ctx.available_champions.len() as u64 | 1 << 32);
    if let Some(hit) = CACHE.with(|c| {
        let (k, v) = c.get();
        (k == key && k != 0).then_some(v)
    }) {
        return hit;
    }
    let mut found = false;
    let mut v: Vec<u8> = Vec::with_capacity(pinned.len() + 1);
    for &c in ctx.available_champions {
        if taken(c, ctx) {
            continue;
        }
        let m = masks.get(c).copied().unwrap_or(MASK_ALL);
        v.clear();
        v.extend_from_slice(pinned);
        v.push(m);
        if feasible(&mut v) {
            found = true;
            break;
        }
    }
    CACHE.with(|c| c.set((key, found)));
    found
}

// ── AI 픽 게이트 (공식 확장점 — detour 불요) ───────────────────────────────
#[derive(Debug)]
struct PosLockDraftAi;

impl ModDraftScoreHook for PosLockDraftAi {
    fn id(&self) -> &str {
        "tfm2_champ_pos_lock.pick_gate"
    }

    fn score_pick(
        &self,
        ctx: &DraftScoreContext,
        candidate: usize,
        _base_score: f32,
    ) -> DraftScoreDecision {
        let cfg = config::get();
        if !cfg.enabled || !cfg.ai_pick_gate || !config::any_restricted() {
            return DraftScoreDecision::Pass;
        }
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let masks = masks()?;
            let cand = masks.get(candidate).copied().unwrap_or(MASK_ALL);
            CNT_SEEN.fetch_add(1, Ordering::Relaxed);
            if cand == MASK_ALL {
                return None;
            }
            let nm = |i: usize| {
                NAMES.get().and_then(|v| v.get(i)).map(|s| s.as_str()).unwrap_or("?")
            };
            // ★진단(2026-08-21): SDK 필드 매핑 vs raw ctx 오프셋 대조 — 어느 게 진짜 아군 픽인지
            //   씬 실픽과 비교해 확정. 총 상한 200줄. (동작 변경 없음 = 로깅만.)
            // ★진단(2026-08-22 재설계 1단계): 픽 페이즈(@30 또는 @40 비어있지 않음)에서만
            //   ctx 4슬라이스를 덤프 — "@30/@40 = 아군/적 픽" 가설 확정용. (기존 덤프는 전부
            //   밴 페이즈에서만 찍혀 @30/@40이 늘 비어 보였음. 픽 턴에선 @10/@20=밴(RE 7차).)
            if false {
                let base = ctx as *const DraftScoreContext as usize;
                let l30 = unsafe { core::ptr::read((base + 0x38) as *const usize) };
                let l40 = unsafe { core::ptr::read((base + 0x48) as *const usize) };
                let _ = l40;
                if l30 >= 1 && l30 <= 5 {
                    let n = DBG_CTXN.fetch_add(1, Ordering::Relaxed);
                    if n < 150 {
                        let scn = hooks::scene_pick_names()
                            .map(|(a, b)| format!("T1=[{}] T2=[{}]", a.join(","), b.join(",")))
                            .unwrap_or_else(|| "none".into());
                        config::dlog(&format!(
                            "pickctx: cand='{}' @10=[{}] @20=[{}] @30=[{}] @40=[{}] | scn {scn}",
                            nm(candidate),
                            dump_vec_names(base, 0x10),
                            dump_vec_names(base, 0x20),
                            dump_vec_names(base, 0x30),
                            dump_vec_names(base, 0x40),
                        ));
                    }
                }
            }
            // ★진단(2026-08-22): mod_api DraftScoreContext 는 Rust struct(slice 필드)라 raw offset
            //   무의미. 필드 직접 접근. available 이 차있으면 ctx 정상 = ally_pick 만 빈 것 확정.
            let ally_idx: Vec<usize> = ctx.ally_pick.iter().copied().collect();
            if cfg.debug {
                let n = DBG_CTXN.fetch_add(1, Ordering::Relaxed);
                if n < 40 {
                    config::dlog(&format!(
                        "ctxfld: ally_pick(n={})=[{}] avail_n={} cand={}",
                        ctx.ally_pick.len(),
                        ctx.ally_pick.iter().map(|&i| nm(i)).collect::<Vec<_>>().join(","),
                        ctx.available_champions.len(),
                        nm(candidate),
                    ));
                }
            }
            let pinned: Vec<u8> = ally_idx
                .iter()
                .map(|&i| masks.get(i).copied().unwrap_or(MASK_ALL))
                .filter(|&m| m != MASK_ALL)
                .collect();
            // ★진단: @30 이 2개 이상 누적되는지(누적 안 되면 "아군 픽 리스트" 가설 기각).
            if cfg.debug && ally_idx.len() >= 2 {
                let n = DBG_MULTIN.fetch_add(1, Ordering::Relaxed);
                if n < 30 {
                    let an: Vec<&str> = ally_idx.iter().map(|&i| nm(i)).collect();
                    config::dlog(&format!(
                        "multi#{n}: cand={} ally(n={})=[{}]",
                        nm(candidate),
                        ally_idx.len(),
                        an.join(",")
                    ));
                }
            }
            if pinned.is_empty() {
                ST_EMPTY.fetch_add(1, Ordering::Relaxed);
                return None; // 제한 픽 없음 → 어떤 후보도 충돌 불가
            }
            let mut pins = pinned.clone();
            pins.push(cand);
            if feasible(&mut pins) {
                ST_FEAS.fetch_add(1, Ordering::Relaxed);
                // ★진단: "충돌 0건"은 불가능 → 마스크 비트 실측(처음 30건).
                if cfg.debug {
                    let n = DBG_FEASN.fetch_add(1, Ordering::Relaxed);
                    if n < 30 {
                        let pb: Vec<String> =
                            pinned.iter().map(|m| format!("{m:05b}")).collect();
                        let an: Vec<&str> = ally_idx.iter().map(|&i| nm(i)).collect();
                        config::dlog(&format!(
                            "feas#{n}: cand={}(m={:05b}) ally=[{}] pinned=[{}]",
                            nm(candidate),
                            cand,
                            an.join(","),
                            pb.join(",")
                        ));
                    }
                }
                return None; // 이 후보는 문제없음
            }
            // ★진단: 충돌 낙하 지점 — 여기 도달 수가 0이면 위 gate 가 전부 삼키는 것(모순 규명).
            ST_CONF.fetch_add(1, Ordering::Relaxed);
            if cfg.debug {
                let n = DBG_CONFN.fetch_add(1, Ordering::Relaxed);
                if n < 20 {
                    let an: Vec<&str> = ally_idx.iter().map(|&i| nm(i)).collect();
                    config::dlog(&format!(
                        "conf#{n}: cand={}(m={:05b}) ally=[{}]",
                        nm(candidate),
                        cand,
                        an.join(",")
                    ));
                }
            }
            // ⚠pinned 자체가 이미 불가(이미 깨진 상태)면 fail-open 판정이 무의미하게 게이트를
            //   풀어버린다 → 개입 포기(Pass).
            if !feasible(&mut pinned.clone()) {
                ST_BROKEN.fetch_add(1, Ordering::Relaxed);
                return None;
            }
            // 이 후보는 아군 픽과 포지션 충돌. 남은 풀에 feasible 대체가 있으면 veto,
            // 하나도 없으면(=제한이 라인업을 불가능하게) fail-open(제한 자동 해제).
            if !pool_has_feasible_idx(ctx, &ally_idx, masks, &pinned) {
                CNT_FAILOPEN.fetch_add(1, Ordering::Relaxed);
                return None;
            }
            if cfg.debug {
                let n = DBG_VETON.fetch_add(1, Ordering::Relaxed);
                if n < 200 {
                    let ally: Vec<&str> = ally_idx.iter().map(|&i| nm(i)).collect();
                    config::dlog(&format!(
                        "veto#{n}: cand={} allypick=[{}]",
                        nm(candidate),
                        ally.join(",")
                    ));
                }
            }
            Some(())
        }));
        match r {
            Ok(Some(())) => {
                CNT_VETO.fetch_add(1, Ordering::Relaxed);
                if cfg.debug {
                    let name = NAMES
                        .get()
                        .and_then(|v| v.get(candidate))
                        .map(|s| s.as_str())
                        .unwrap_or("?");
                    config::dlog(&format!("pick_veto: {name} (idx={candidate})"));
                }
                // ★관찰 전용 모드: 판정·로깅은 다 하되 실제 개입(Replace)은 안 함(바닐라 관찰).
                if cfg.ai_observe_only {
                    DraftScoreDecision::Pass
                } else {
                    DraftScoreDecision::Replace(-1.0e9)
                }
            }
            _ => DraftScoreDecision::Pass,
        }
    }
}

// ── 마스크 재계산 (NAMES + 현재 상태) ───────────────────────────────────────
/// scene_step 발화 스탬프 직전값(밴픽 활성 프레임 감지 — scene_step 은 매 프레임 호출).
static LAST_SCENE_STAMP: AtomicU64 = AtomicU64::new(u64::MAX);

/// 회색(선택 불가) 셀의 툴팁 문구. 게임의 fearless_tooltip 노드에 그대로 주입된다
///   (호버 판정·표시·배치는 게임이 담당 — RE 2026-08-22).
pub(crate) fn block_msg(name: &str) -> String {
    match hooks::block_reason(name) {
        Some(p) if !p.is_empty() => format!("해당 포지션은 더이상 선택할 수 없습니다: {p}"),
        _ => "해당 포지션은 더이상 선택할 수 없습니다".to_string(),
    }
}

/// 유저 픽 차단 목록 계산 → hooks::BLOCKLIST 게시. 밴픽 활성 프레임에만 갱신.
/// 유저 현재 픽(씬 O_PICK1)의 제한 마스크를 pin → 각 후보를 pin+후보로 feasible 체크,
/// 매칭 깨는(=남은 포지션 못 채우는) 챔프를 차단. fail-open: 픽 가능한 게 0이면 차단 안 함.
/// ⚠씬 오프셋(O_PICK1 등)은 0.5.5 채록·0.5.6 미검증 — read_scene_vec 이 형태 이상 시 None 반환 →
///   그 경우 안전하게 차단 해제(오프셋 어긋나면 기능 무동작, 크래시 아님).
/// 현재 밴픽 차례의 종류. **게임 UI 자체의 `in_turn` 표시**로 판정하므로
///   밴픽 순서를 바꾸는 다른 모드가 있어도 그대로 맞는다(순서를 가정하지 않는다).
///   트리(0.5.6 실측):
///     main/blue_picks|red_picks/pick_slot_N/in_turn      → 그 팀의 **픽** 차례
///     main/bottom/blue_side|red_side/bans/ban_slot_N/in_turn → 그 팀의 **밴** 차례
static TURN_SIG: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static BLOCK_SIG: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static MASKSTAT_SIG: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);
static SWAPVEC_SIG: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static SWAP_STAMP: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// 드래프트당 스왑 order 를 쓴 횟수(양 팀 합산). 게임이 뒤늦게 기본값을 덮어쓸 수 있어 소수 회 허용.
static SWAP_APPLIED: AtomicUsize = AtomicUsize::new(0);
const SWAP_APPLY_MAX: usize = 6;
static SWAP_PANEL: AtomicBool = AtomicBool::new(false);
static SWAP_WAIT: AtomicBool = AtomicBool::new(false);
static SWAP_COACH: AtomicBool = AtomicBool::new(false);
static MY_IS_T2: AtomicBool = AtomicBool::new(false);
/// 스왑 배정이 우리 설정을 위반 중인가(확정 버튼 게이트용).
static SWAP_BAD: AtomicBool = AtomicBool::new(false);
/// 우리가 확정 버튼을 숨긴 상태인가(우리가 숨긴 것만 되돌린다).
static CONFIRM_HIDDEN: AtomicBool = AtomicBool::new(false);
/// 코치 위임으로 무장됐는가(무장된 동안만 내 팀 order 를 쓴다).
static SWAP_ARMED: AtomicBool = AtomicBool::new(false);
static CONFIRM_PROBED: AtomicBool = AtomicBool::new(false);
static BTNPROBE_A: AtomicBool = AtomicBool::new(false);
static BTNPROBE_B: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum TurnKind {
    Ban,
    Pick,
    Unknown,
}

fn any_in_turn_visible(n: &Node) -> bool {
    if n.id == "in_turn" && n.visible {
        return true;
    }
    n.child.iter().any(any_in_turn_visible)
}

/// 밴픽 화면의 좌/우 진영.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Side {
    Blue,
    Red,
}

/// ★내 팀 = **"위임" 버튼이 보이는(=내가 지금 행동할 수 있는) 순간에 `in_turn` 이 켜진 쪽**.
///   씬의 팀 id 오프셋(+0x3d0)은 "내 팀"이 아니라 1팀 id 였다(실측 2026-08-22:
///   상대 픽을 내 픽으로 계산해 엉뚱한 포지션을 막았다). 그래서 UI 에서 직접 유도한다.
///   -1=미확정 / 0=Blue / 1=Red.
static MY_SIDE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1);
/// 진영 → 씬 픽 벡터 매핑. 0=미확정 / 1=Blue가 picks_a / 2=Blue가 picks_e.
static SIDE_MAP: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn side_root<'a>(root: &'a Node, side: Side, picks: bool) -> Option<&'a Node> {
    let id = match (side, picks) {
        (Side::Blue, true) => "blue_picks",
        (Side::Red, true) => "red_picks",
        (Side::Blue, false) => "blue_side",
        (Side::Red, false) => "red_side",
    };
    ui_kit::find(root, id)
}

/// 지금 `in_turn` 이 켜진 진영(픽 슬롯이든 밴 슬롯이든).
fn side_in_turn(root: &Node) -> Option<Side> {
    for side in [Side::Blue, Side::Red] {
        if side_root(root, side, true)
            .map(any_in_turn_visible)
            .unwrap_or(false)
        {
            return Some(side);
        }
        if side_root(root, side, false)
            .and_then(|n| ui_kit::find(n, "bans"))
            .map(any_in_turn_visible)
            .unwrap_or(false)
        {
            return Some(side);
        }
    }
    None
}

/// 그 진영의 확정된 픽 수 = `done` 이 보이는 pick_slot 개수.
fn side_done_count(root: &Node, side: Side) -> usize {
    let Some(n) = side_root(root, side, true) else {
        return 0;
    };
    n.child
        .iter()
        .filter(|slot| {
            slot.child
                .iter()
                .any(|c| c.id == "done" && c.visible)
        })
        .count()
}

pub(crate) fn detect_turn(root: &Node) -> TurnKind {
    let pick = ["blue_picks", "red_picks"]
        .iter()
        .any(|k| ui_kit::find(root, k).map(any_in_turn_visible).unwrap_or(false));
    if pick {
        return TurnKind::Pick;
    }
    let ban = ["blue_side", "red_side"].iter().any(|k| {
        ui_kit::find(root, k)
            .and_then(|n| ui_kit::find(n, "bans"))
            .map(any_in_turn_visible)
            .unwrap_or(false)
    });
    if ban {
        return TurnKind::Ban;
    }
    TurnKind::Unknown
}

fn recompute_blocklist(root: &Node) {
    // ★해제(clear)와 유지(hold)를 구분한다 —
    //   **확정적 조건**(모드 off / 제한 없음 / 밴 페이즈 / 드래프트 종료)에서만 회색을 푼다.
    //   **일시적 조건**(연출·전환 중이라 밴픽 씬을 못 읽는 프레임)에서는 직전 목록을 그대로 둔다.
    //   연출 중에 씬 스텝이 멈춰 stamp 가 안 오르는데, 예전엔 이때도 clear 해서
    //   **회색이 잠깐 풀렸다**(유저 보고 2026-08-22). 어차피 그 순간엔 못 고르지만 헷갈린다.
    let clear = || {
        *hooks::BLOCKLIST.lock().unwrap_or_else(|e| e.into_inner()) = None;
        *hooks::BLOCK_REASON.lock().unwrap_or_else(|e| e.into_inner()) = None;
    };
    let cfg = config::get();
    if !cfg.enabled || !cfg.user_pick_block || !config::any_restricted() {
        clear();
        return;
    }
    let Some(names) = NAMES.get() else {
        clear();
        return;
    };
    let Some(masks) = masks() else {
        clear();
        return;
    };
    // 진짜 클라 밴픽 씬(scene_step rcx 캡처) 읽기. scene_step 은 매 프레임 호출 → stamp 진행.
    let (scene, stamp) = hooks::scene_cap();
    let last = LAST_SCENE_STAMP.swap(stamp, Ordering::Relaxed);
    // 밴픽 렌더가 이번 프레임에 없으면(스탬프 불변) = 연출·전환 중 → **직전 목록 유지**.
    if scene < 0x10000 || stamp == last {
        return;
    }
    // O_PICK1=T1PICK(내 팀)·O_PICK2=T2PICK(상대)·O_BAN1/2=T1/T2 밴. 원소 = 소문자 챔프 id.
    let (picks_a, picks_e, bans_a, bans_e) = unsafe {
        (
            hooks::read_scene_vec(scene, hooks::O_PICK1),
            hooks::read_scene_vec(scene, hooks::O_PICK2),
            hooks::read_scene_vec(scene, hooks::O_BAN1),
            hooks::read_scene_vec(scene, hooks::O_BAN2),
        )
    };
    let (Some(picks_a), Some(picks_e), Some(bans_a), Some(bans_e)) =
        (picks_a, picks_e, bans_a, bans_e)
    else {
        return; // 씬 형태 이상(과도기) → 일시적이므로 직전 목록 유지
    };
    // taken = 4벡터(픽·밴) 이름 합집합.
    let mut taken: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for v in [&picks_a, &picks_e, &bans_a, &bans_e] {
        for s in v {
            taken.insert(s.as_str());
        }
    }
    // ★★2026-08-22 정정: "픽 수가 적은 쪽"(next-picker) 추정은 스네이크 픽 순서에서 틀리고,
    //   드래프트가 끝난 뒤에도 계산돼 pinned=5 → 전 후보 불가 → anyfeasible=false → fail-open
    //   으로 **아무것도 차단하지 않는** 상태였다(실측 ui_block=0).
    //   씬 = 밴픽 뷰 컨트롤러이므로(축E 확정: 4벡터 +0x138/0x150/0x168/0x180 = 우리 O_* 와 일치)
    //   **내 팀 ID(+0x3d0)·팀당 밴 수(+0x3c0)·포맷(+0xce)** 을 직접 읽어 정확히 판정한다.
    let (ban_cnt, fmt, sel_team, t2_team) = unsafe {
        (
            hooks::ru64(scene + hooks::O_BANCNT) as usize,
            hooks::ru8(scene + hooks::O_RULE) as usize,
            hooks::ru64(scene + hooks::O_SELTEAM),
            hooks::ru64(scene + hooks::O_T2TEAM),
        )
    };
    if fmt >= 4 || ban_cnt > 10 {
        return; // 씬 형태 이상 → 일시적이므로 직전 목록 유지
    }
    let total = picks_a.len() + picks_e.len() + bans_a.len() + bans_e.len();
    let base = ban_cnt * 2;
    let picks_n = 4 + fmt * 2; // fmt0=4, 1=6, 2=8, 3=10
    // ★★2026-08-22 정정: 단계 판정을 `total < 밴수*2`(= 밴이 전부 먼저)로 했더니,
    //   밴픽 **순서를 바꾸는 모드**(tfm2_banpick_order 등)가 끼면 픽 차례를 밴으로 오판해
    //   차단이 통째로 안 걸렸다(유저 보고: 9/20 픽인데 회색 없음).
    //   ⟹ 순서를 가정하지 말고 **게임 UI 의 `in_turn` 표시**로 현재 차례를 직접 읽는다.
    //   읽히지 않을 때만(Unknown) 옛 공식으로 폴백.
    let tk = detect_turn(root);
    // ★커밋 훅(CP)도 같은 판정을 써야 한다 — 커밋 쪽이 옛 공식이면 2차 밴 페이즈의 밴을
    //   픽으로 착각해 교체해 버린다(유저 보고 2026-08-22: 소총수 밴 → 엉뚱한 챔프 밴).
    hooks::set_ui_turn(match tk {
        TurnKind::Ban => 1,
        TurnKind::Pick => 2,
        TurnKind::Unknown => 0,
    });
    let is_ban = match tk {
        TurnKind::Ban => true,
        TurnKind::Pick => false,
        TurnKind::Unknown => total < base,
    };
    // 차례가 바뀔 때만 한 줄 남긴다(라인업 로그 파일). 판정이 맞는지 사후 확인용.
    {
        let k = detect_turn(root);
        let sig = ((total as u64) << 4) | (k as u64) | 0x1000_0000;
        if TURN_SIG.swap(sig, Ordering::Relaxed) != sig {
            config::llog(&format!(
                "turn: step={} kind={:?} is_ban={is_ban} (bans={}/{} picks={}/{})",
                total + 1,
                k,
                bans_a.len() + bans_e.len(),
                base,
                picks_a.len() + picks_e.len(),
                picks_n
            ));
        }
    }
    if is_ban {
        clear(); // 밴 차례 = 포지션 무관, 게다가 회색이면 밴 자체를 못 하게 된다
        return;
    }
    // ── 내 팀 판정 ──────────────────────────────────────────────────────────
    //   ①"위임" 버튼이 보이는 순간(=내 차례) `in_turn` 이 켜진 진영 = 내 진영. 한 번 배우면 유지.
    if ui_kit::find(root, "delegate_btn").map(|n| n.visible).unwrap_or(false) {
        if let Some(sd) = side_in_turn(root) {
            MY_SIDE.store(if sd == Side::Blue { 0 } else { 1 }, Ordering::Relaxed);
        }
    }
    //   ②진영 → 씬 픽벡터 매핑: 양쪽 확정 픽 수가 다를 때 대조해서 확정(그 뒤 캐시).
    {
        let (b, r) = (side_done_count(root, Side::Blue), side_done_count(root, Side::Red));
        if b != r {
            if b == picks_a.len() && r == picks_e.len() {
                SIDE_MAP.store(1, Ordering::Relaxed);
            } else if b == picks_e.len() && r == picks_a.len() {
                SIDE_MAP.store(2, Ordering::Relaxed);
            }
        }
    }
    let my_side = MY_SIDE.load(Ordering::Relaxed);
    // (스왑 검증에서 재사용 — 내 팀 order 벡터를 고르는 데 필요)
    let side_map = SIDE_MAP.load(Ordering::Relaxed);
    let my_is_t2 = if my_side >= 0 && side_map != 0 {
        // Blue=picks_a(1) 이면 Red 가 T2; Blue=picks_e(2) 면 Blue 가 T2.
        if side_map == 1 { my_side == 1 } else { my_side == 0 }
    } else {
        sel_team == t2_team // 폴백(구 방식)
    };
    MY_IS_T2.store(my_is_t2, Ordering::Relaxed);
    let my_picks_raw: &Vec<String> = if my_is_t2 { &picks_e } else { &picks_a };
    // ★빈 문자열은 "선택 대기" 자리표시자일 수 있으므로 실제 픽에서 제외.
    let my_picks: Vec<&String> = my_picks_raw.iter().filter(|s| !s.is_empty()).collect();
    // ★★2026-08-22 정정: 종료 판정을 `total >= base + picks_n` 로 하면
    //   **마지막 픽(20/20)에서 제한이 통째로 풀렸다**(유저 보고). 상대 픽까지 합산한 total 은
    //   자리표시자·집계 시점 차이로 한 스텝 일찍 상한에 닿는다.
    //   → **내 팀 픽이 다 찼는가**로 판정한다(팀당 픽 수 = picks_n/2). 내 픽이 남아 있으면 항상 활성.
    if my_picks.len() >= picks_n / 2 {
        clear(); // 내 픽 완료 → 더 고를 게 없으니 차단 불필요
        return;
    }
    let mut pinned: Vec<u8> = Vec::with_capacity(5);
    for p in &my_picks {
        let m = config::mask_of(p);
        if m != MASK_ALL {
            pinned.push(m);
        }
    }
    // 각 후보: 유저가 지금 고르면 남은 포지션까지 매칭 유지되나?
    let mut block: std::collections::HashSet<String> = std::collections::HashSet::new();
    // 차단 사유(그 챔프가 갈 수 있는 포지션들 = 이미 다 찬 포지션) 라벨.
    let mut reason: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut any_feasible = false;
    for (i, n) in names.iter().enumerate() {
        let lower = n.to_ascii_lowercase();
        if taken.contains(lower.as_str()) {
            continue;
        }
        let m = masks.get(i).copied().unwrap_or(MASK_ALL);
        if m == MASK_ALL {
            any_feasible = true; // 무제한 챔프는 언제나 픽 가능
            continue;
        }
        if helps(&pinned, m) {
            any_feasible = true;
        } else {
            let label = (0..5)
                .filter(|p| m & (1 << p) != 0)
                .map(|p| POS_NAMES_KR[p])
                .collect::<Vec<_>>()
                .join("/");
            reason.insert(lower.clone(), label);
            block.insert(lower);
        }
    }
    let published = if any_feasible {
        block
    } else {
        std::collections::HashSet::new() // fail-open
    };
    // 씬이 바뀐 프레임에만 로그(스팸 방지) — 실제 픽/밴을 이름으로 찍어 검증(T1=내팀 확인용).
    if cfg.debug && stamp != last {
        config::dlog(&format!(
            "userblock: my={} pick_t1=[{}] pick_t2=[{}] ban={}/{} total={total} pinned={} block={} anyfeasible={}",
            if my_is_t2 { "T2" } else { "T1" },
            picks_a.join(","),
            picks_e.join(","),
            bans_a.len(),
            bans_e.len(),
            pinned.len(),
            published.len(),
            any_feasible
        ));
    }
    let pub_reason = if published.is_empty() {
        std::collections::HashMap::new()
    } else {
        reason
    };
    // ── 진단(항상): 내 픽이 어느 포지션을 차지했고 그 결과 무엇이 몇 개 막혔는지 ──
    //   내용이 바뀔 때만 한 줄. "드루이드 골랐는데 탑이 막혔다" 같은 보고를 바로 검증한다.
    {
        let mask_label = |m: u8| -> String {
            if m == MASK_ALL {
                return "무제한".into();
            }
            (0..5)
                .filter(|p| m & (1 << p) != 0)
                .map(|p| POS_NAMES_KR[p])
                .collect::<Vec<_>>()
                .join("/")
        };
        let mine: Vec<String> = my_picks
            .iter()
            .map(|n| format!("{}({})", kr_name(n), mask_label(config::mask_of(n))))
            .collect();
        let mut by: std::collections::BTreeMap<String, usize> = Default::default();
        for r in pub_reason.values() {
            *by.entry(r.clone()).or_insert(0) += 1;
        }
        let by_s = by
            .iter()
            .map(|(k, v)| format!("{k}:{v}"))
            .collect::<Vec<_>>()
            .join(" ");
        let sig = {
            use std::hash::{Hash, Hasher};
            let mut h = std::collections::hash_map::DefaultHasher::new();
            mine.hash(&mut h);
            by_s.hash(&mut h);
            published.len().hash(&mut h);
            h.finish()
        };
        if BLOCK_SIG.swap(sig, Ordering::Relaxed) != sig {
            config::llog(&format!(
                "block: 내픽=[{}] 차단={}개 [{}] | my_side={} side_map={} is_t2={}",
                mine.join(" "),
                published.len(),
                by_s,
                match my_side { 0 => "Blue", 1 => "Red", _ => "?" },
                side_map,
                my_is_t2
            ));
        }
    }
    *hooks::BLOCK_REASON.lock().unwrap_or_else(|e| e.into_inner()) = Some(pub_reason);
    *hooks::BLOCKLIST.lock().unwrap_or_else(|e| e.into_inner()) = Some(published);
}

fn recompute_masks_if_needed() {
    let Some(names) = NAMES.get() else { return };
    let ver = config::state_version();
    if APPLIED_VER.load(Ordering::Relaxed) == ver {
        return;
    }
    let v: Vec<u8> = names
        .iter()
        .map(|n| config::mask_of(&n.to_ascii_lowercase()))
        .collect();
    publish_masks(v);
    APPLIED_VER.store(ver, Ordering::Relaxed);
    if config::get().debug {
        config::dlog(&format!("masks 재계산 (ver={ver})"));
    }
}

// ── 디버그: 노드 트리 덤프 (UI 주입점 파악용) ───────────────────────────────
fn node_has_id(n: &Node, sub: &str) -> bool {
    if n.id.contains(sub) {
        return true;
    }
    n.child.iter().any(|c| node_has_id(c, sub))
}
fn dump_tree(n: &Node, depth: usize, out: &mut String) {
    if depth > 14 {
        return;
    }
    out.push_str(&"  ".repeat(depth));
    out.push_str(&format!("{}  (child={}, vis={})\n", n.id, n.child.len(), n.visible));
    for c in &n.child {
        dump_tree(c, depth + 1, out);
    }
}
static DUMP_FRAME: AtomicU64 = AtomicU64::new(0);
static DUMPED_OPTION: AtomicBool = AtomicBool::new(false);
static DUMPED_SWAP: AtomicBool = AtomicBool::new(false);
static DUMPED_POPUP: AtomicBool = AtomicBool::new(false);

fn maybe_dump_ui(root: &Node) {
    let f = DUMP_FRAME.fetch_add(1, Ordering::Relaxed);
    // 매 ~1초, 화면에 떠 있는 트리 전체를 통째로 덮어쓴다(무조건 — 어떤 root 를 받는지부터 확인).
    if f % 60 == 0 {
        let mut s = format!(
            "frame={f}  root.id='{}'  children={}\n",
            root.id,
            root.child.len()
        );
        s.push_str("[top-level child ids] ");
        for c in &root.child {
            s.push_str(&c.id);
            s.push(' ');
        }
        s.push_str("\n\n");
        dump_tree(root, 0, &mut s);
        if let Some(d) = mod_dir() {
            let _ = std::fs::write(format!("{d}\\ui_tree_live.txt"), &s);
        }
    }
    // ★스왑 단계 트리 스냅샷(배치 데이터 위치 파악용) — swap 관련 노드가 뜨면 1회.
    let sw = node_has_id(root, "swap_phase")
        || node_has_id(root, "swap_slot")
        || node_has_id(root, "swap_waiting");
    if sw && !DUMPED_SWAP.swap(true, Ordering::Relaxed) {
        let mut s = String::from("=== swap phase ===");
        dump_tree(root, 0, &mut s);
        if let Some(d) = mod_dir() {
            let _ = std::fs::write(format!("{d}\\ui_tree_swap.txt"), s);
        }
    }
    if !sw {
        DUMPED_SWAP.store(false, Ordering::Relaxed);
    }
    // 옵션/팝업이 트리에 나타나면 그 순간 스냅샷을 따로 남긴다(id 다양성 대비 넓게 매칭).
    let opt = node_has_id(root, "option") || node_has_id(root, "gameplay");
    if opt && !DUMPED_OPTION.swap(true, Ordering::Relaxed) {
        let mut s = String::from("=== option/settings ===\n");
        dump_tree(root, 0, &mut s);
        if let Some(d) = mod_dir() {
            let _ = std::fs::write(format!("{d}\\ui_tree_option.txt"), s);
        }
    }
    if !opt {
        DUMPED_OPTION.store(false, Ordering::Relaxed);
    }
    let pop = node_has_id(root, "custom_champion");
    if pop && !DUMPED_POPUP.swap(true, Ordering::Relaxed) {
        let mut s = String::from("=== custom_champion_popup ===\n");
        dump_tree(root, 0, &mut s);
        if let Some(d) = mod_dir() {
            let _ = std::fs::write(format!("{d}\\ui_tree_popup.txt"), s);
        }
    }
    if !pop {
        DUMPED_POPUP.store(false, Ordering::Relaxed);
    }
}

// ── 진입 ──────────────────────────────────────────────────────────────────
struct PosLockExt;

impl ModExtension for PosLockExt {
    fn post_update(&self, scene: &mut Scene, ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let cfg = config::get();
            if !cfg.enabled {
                return;
            }
            // 챔피언 목록 1회 캡처(관리화면 프레임에서도 Scene::InGame 매치).
            if let Scene::InGame { data } = scene {
                let db = data.db();
                // Database 포인터 캡처(룰 카운트 읽기용) — 매 프레임 갱신. Ref<ClientDatabase> deref.
                DB_PTR.store(&*db as *const _ as usize, Ordering::Relaxed);
                if NAMES.get().is_none() {
                    if !db.available_champions.is_empty() {
                        let ids: Vec<String> = db.available_champions.clone();
                        if cfg.debug {
                            let mut s = String::from("# 현재 사용 가능한 챔피언 id 목록\n");
                            for id in &ids {
                                s.push_str(id);
                                s.push('\n');
                            }
                            if let Some(d) = mod_dir() {
                                let _ = std::fs::write(
                                    format!("{d}\\champ_pos_lock_champions.txt"),
                                    s,
                                );
                            }
                            config::dlog(&format!("캡처: 챔프 {}개", ids.len()));
                        }
                        let _ = NAMES.set(ids);
                    }
                }
            }
            // 상태(UI 편집)가 바뀌었으면 마스크 재계산.
            recompute_masks_if_needed();

            // Hook E: 로드된 Assets 캡처 — 아이콘 표시용이라 lock 유무 무관·enabled면 설치.
            hooks::install_once_e();
            // 훅 설치 — 마스크 준비 후 1회 (late install, §3)
            if masks().is_some() {
                hooks::install_once();
                hooks::install_once_c();
            }
            // Hook D'(scene_step 씬 캡처) — 외부훅(banpick_order) 대기 재시도라 매 프레임 호출.
            hooks::install_once_dp();
            // ★유저 UI: 회색화(0x2553a16) + 클릭차단(0x25508de) — RE 2026-08-22.
            //   기존 hookC(0xb31840)는 진입 1회 빌더 전용이라 무효였음(188회 정지 확인).
            if masks().is_some() {
                hooks::install_once_uiblock();
            }
            // Hook P(픽 디스패처 차단) — ⛔폐기(드롭이 라인업 desync → sim 크래시). 03 참조.
            // Hook COMMIT(서버 커밋 강제거부) — 매치별 독립·전 매치·반환0=안전거부.
            //   각 매치 rmi에서 그 매치 픽만 읽어 판정 → 남의 매치 오염 없음(hookP 문제 해결).
            hooks::install_once_commit();
            // ★VEH fault-safe 읽기 설치(recommend_filter 전) — 워커스레드 raw read AV 방지.
            hooks::seh_install();
            // ★★★Hook CP(커밋 producer 0x1f16ea0 확정픽 교체) — 2026-08-22 최종 정답.
            //   RE 2건이 독립적으로 지목한 유일 안전 지점. 결정레코드(0xd18)+0x18 String 을
            //   합법 대체로 바꾸면 커밋 인자와 사후 브로드캐스트가 같은 이름 → desync 없음.
            //   커밋 1430회/세션 = 결정당 1회(lookahead 아님). REJECT 안 하므로 hang 없음.
            //   ⛔AM(f848f0 감점) 제거: 1300만회 감점에도 라인업 불변 = lookahead 내부 확정.
            if masks().is_some() {
                // DQ: 디스패처 출력 [0]을 첫 합법 차순위와 스왑(품질 보존, 1순위).
                hooks::install_once_cb();
                hooks::install_once_dq();
                // CP: 커밋 producer 에서 확정 String 교체(모든 경로 안전망).
                hooks::install_once_cprod();
            hooks::install_once_sc(); // 스왑 확정 버튼(클릭 게이트 + 상대 팀 배정)
            }
            // ★Hook R(recommend available 필터) — AI 결정단계 하드블록. score_pick 훅이 유저
            //   라이브/코치 매치엔 미발화하므로, recommend 진입서 available 후보를 직접 필터.
            // hookR(밴 페이즈 recommend 0x2148ca0) — 픽 차단엔 불요 + 설치 경합/검은화면 위험이라 비활성.
            // hooks::install_once_recommend();
            // hooks::install_once_recommend_wbc(); — 제거(2026-08-22): veto 는 SDK score_pick
            //   디스패치로 발화하므로 recommend 계열 훅은 전부 불필요. suspend 기반 설치가
            //   시작 검은화면(간헐)의 원인이라 원천 제거.
            // hooks::install_once_finalize(); — 보류(재설계: score_pick 축 교정이 1순위, 불필요 변수 제거)
            // ★전 매치 최종 라인업 드레인 — veto 가 실제로 중복을 막는지 결과로 검증.
            {
                let fin: Vec<hooks::FinalLineup> = {
                    let mut g = hooks::FINAL_RING.lock().unwrap_or_else(|e| e.into_inner());
                    std::mem::take(&mut *g)
                };
                if config::get().debug {
                    for f in fin {
                        config::dlog(&format!(
                            "lineup{}: [{}]",
                            if f.dup { "★DUP" } else { "-OK" },
                            f.picks.join(",")
                        ));
                    }
                }
            }
            // 확정 관측 링 드레인.
            {
                let fz: Vec<hooks::FzEntry> = {
                    let mut g = hooks::FZ_RING.lock().unwrap_or_else(|e| e.into_inner());
                    std::mem::take(&mut *g)
                };
                if config::get().debug {
                    for e in fz {
                        let scn = hooks::scene_pick_names()
                            .map(|(a, b)| format!("T1=[{}] T2=[{}]", a.join(","), b.join(",")))
                            .unwrap_or_else(|| "none".into());
                        let args: Vec<String> =
                            e.args.iter().map(|v| format!("{v:#x}")).collect();
                        config::dlog(&format!(
                            "fz: name={:?} args=[{}] | scn {scn}",
                            e.name,
                            args.join(" ")
                        ));
                    }
                }
            }
            // 디스패처 관측 링 드레인(메인스레드 = 포맷/로그 안전).
            {
                let entries: Vec<hooks::DpEntry> = {
                    let mut g = hooks::DP_RING.lock().unwrap_or_else(|e| e.into_inner());
                    std::mem::take(&mut *g)
                };
                if config::get().debug {
                    for e in entries {
                        let scn = hooks::scene_pick_names()
                            .map(|(a, b)| format!("T1=[{}] T2=[{}]", a.join(","), b.join(",")))
                            .unwrap_or_else(|| "none".into());
                        let raw: Vec<String> =
                            e.raw.iter().map(|v| format!("{v:#x}")).collect();
                        config::dlog(&format!(
                            "disp: live={} cands=[{}] raw=[{}] | scn {scn}",
                            e.live,
                            e.cands.join(","),
                            raw.join(" ")
                        ));
                    }
                }
            }
            hooks::update_banpick_active(); // 라이브 밴픽 UI 활성 판정(recommend 필터 게이트)
            hooks::note_my_teams(); // 유저 매치 팀 id 학습(백그라운드 매치 배제용)
            // 유저 픽 차단 목록 갱신(밴픽 활성 프레임에만 — 내부 스탬프 가드).
            recompute_blocklist(&ui.root);

            // UI 주입점 덤프(개발용)
            if cfg.dump_ui {
                maybe_dump_ui(&ui.root);
            }

            // ── 인게임 UI: 환경설정 게임플레이 탭 '포지션 제한' 행 ──
            inject::install();
            // 게임플레이 탭이 보일 때만 내 행 표시(같은 탭의 banpick_style 행 visible 을 따라감).
            let gp_vis = ui_kit::find(&ui.root, "banpick_style")
                .map(|n| n.visible)
                .unwrap_or(false);
            if let Some(row) = ui_kit::find_mut(&mut ui.root, "pos_lock_row") {
                row.visible = gp_vis;
            }
            // 팝업 표시/숨김 + 그리드 채우기
            let open = POPUP_OPEN.load(Ordering::Relaxed);
            let present = ui_kit::find_mut(&mut ui.root, "pos_lock_popup").is_some();
            if present {
                // ★아이콘 UV 백그라운드 로드 = 환경설정 화면이 실제로 열렸을 때만 착수(1회).
                //   시작 로딩(검은 화면) 중엔 이 노드가 트리에 없어 build()가 안 돌아 IO 경합 없음.
                //   (시작 시 무조건 로드하면 게임 startup 에셋 로딩과 bundle.game_data IO 경합 →
                //    검은 화면. 2026-08-20 실사고, 03_시행착오.)
                icon_data::start_load();
            }
            if !present {
                POPUP_OPEN.store(false, Ordering::Relaxed);
            } else if let Some(pop) = ui_kit::find_mut(&mut ui.root, "pos_lock_popup") {
                pop.visible = open;
            }
            if open && present {
                fill_grid(&mut ui.root);
            }

            // 클릭 라우팅
            let mut routes: Vec<(String, ui_kit::ClickFn)> = Vec::with_capacity(NCELLS + 16);
            routes.push(ui_kit::route(
                "pos_lock_configure",
                Rc::new(|| {
                    CNT_ROW_CLICK.fetch_add(1, Ordering::Relaxed);
                    POPUP_OPEN.store(true, Ordering::Relaxed);
                    GRID_SIG.store(u64::MAX, Ordering::Relaxed); // 열 때 강제 재채움
                    config::dlog("포지션 제한 버튼 클릭됨");
                }),
            ));
            let close: ui_kit::ClickFn = Rc::new(|| POPUP_OPEN.store(false, Ordering::Relaxed));
            routes.push(ui_kit::route("pos_lock_popup.close", close.clone()));
            routes.push(ui_kit::route("pos_lock_popup.cancel", close));
            routes.push(ui_kit::route(
                "pos_lock_popup.ok",
                Rc::new(|| {
                    config::save_state_to_file();
                    POPUP_OPEN.store(false, Ordering::Relaxed);
                    config::dlog("포지션 제한 저장");
                }),
            ));
            for (i, t) in TAB_IDS.iter().enumerate() {
                routes.push(ui_kit::route(
                    t,
                    Rc::new(move || SEL_POS.store(i, Ordering::Relaxed)),
                ));
            }
            routes.push(ui_kit::route(
                "clear_pos",
                Rc::new(|| config::clear_pos(SEL_POS.load(Ordering::Relaxed))),
            ));
            routes.push(ui_kit::route(
                "select_all_pos",
                Rc::new(|| {
                    if let Some(champs) = champ_names() {
                        let all: Vec<String> =
                            champs.iter().map(|c| c.to_ascii_lowercase()).collect();
                        config::set_pos(SEL_POS.load(Ordering::Relaxed), all);
                    }
                }),
            ));
            for k in 0..NCELLS {
                routes.push(ui_kit::route(
                    &format!("cell{k}"),
                    Rc::new(move || {
                        if let Some(champs) = sorted_champs() {
                            if let Some(c) = champs.get(k) {
                                config::toggle(
                                    SEL_POS.load(Ordering::Relaxed),
                                    &c.to_ascii_lowercase(),
                                );
                            }
                        }
                    }),
                ));
            }
            ui_kit::ensure_clicks(ui, &CLICK_LAST, routes);

            // ★스왑 order 벡터 탐침 — RE 2026-08-22: 스왑 확정 핸들러(0x1d7bc10)가
            //   `controller+0x198`(한 팀) / `+0x1b0`(다른 팀) 의 Vec 을 그대로 복사해
            //   ClientPacket::SwapDone.order 로 보낸다. Vec = {cap@0, ptr@8, len@0x10}, 원소 8B.
            //   그 컨트롤러가 우리가 캡처 중인 밴픽 씬과 같은 객체인지 런타임으로 확인한다.
            {
                let (scene, sstamp) = hooks::scene_cap();
                // ★★씬 생존 게이트 (2026-08-22 크래시 원인).
                //   밴픽 씬이 해제된 뒤에도 stale 포인터로 계속 읽고 **썼다**.
                //   해제된 메모리에서 len 이 우연히 5로 읽혀 가드를 통과 → 힙 오염 → 지연 크래시.
                //   scene_step 은 매 프레임 stamp 를 올리므로, **이번 프레임에 stamp 가 올라간
                //   경우에만** 씬이 살아있다고 본다.
                let live = {
                    let last = SWAP_STAMP.swap(sstamp, Ordering::Relaxed);
                    sstamp != last && sstamp != 0
                };
                if scene >= 0x10000 && live {
                    let rd = |o: usize| -> (u64, u64, u64) {
                        unsafe {
                            (
                                hooks::ru64(scene + o),
                                hooks::ru64(scene + o + 8),
                                hooks::ru64(scene + o + 0x10),
                            )
                        }
                    };
                    let dump = |o: usize| -> String {
                        let (cap, ptr, len) = rd(o);
                        if len == 0 || len > 16 || ptr < 0x10000 {
                            return format!("+{o:x}(cap={cap} ptr={ptr:#x} len={len})");
                        }
                        let v: Vec<u64> = (0..len as usize)
                            .map(|i| unsafe { hooks::ru64(ptr as usize + i * 8) })
                            .collect();
                        if v.iter().any(|&x| x >= 5) {
                            return format!("+{o:x}(len={len} INVALID)");
                        }
                        format!(
                            "+{o:x}(len={len} [{}])",
                            v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")
                        )
                    };
                    // ★강제: 각 팀의 order 를 우리 매칭(assign_positions)대로 덮어쓴다.
                    //   방향(슬롯→픽 / 픽→슬롯)이 미확정이라 cfg swap_force 로 전환해 실측한다.
                    //   +0x198 = T1(picks_a) / +0x1b0 = T2(picks_e) 로 가정(로그로 검증).
                    // ── 스왑 규칙 (유저 확정 2026-08-22) ─────────────────────────
                    //   ①"스왑을 수석 코치에게 위임" → 내 팀을 포지션별로 배정
                    //   ②확정 버튼 → 상대 팀도 포지션별로 배정 (훅 0x1d7bc10 에서 수행)
                    //   ③완전 배정이 불가능하면 **최대한 맞춘다**, 선택지가 여러 개면
                    //     게임이 이미 계산한 order 와 최대한 일치시켜 원래 우선순위를 따른다
                    //   그 외에는 order 를 건드리지 않는다(유저 수동 스왑 존중).
                    let picks_done = unsafe {
                        hooks::read_scene_vec(scene, hooks::O_PICK1)
                            .map(|v| v.len())
                            .unwrap_or(0)
                            + hooks::read_scene_vec(scene, hooks::O_PICK2)
                                .map(|v| v.len())
                                .unwrap_or(0)
                    };
                    if picks_done < 10 {
                        SWAP_APPLIED.store(0, Ordering::Relaxed);
                        SWAP_ARMED.store(false, Ordering::Relaxed);
                    }
                    // 위임 감지: swap_waiting 이 뜨거나 coach 버튼이 사라지는 순간.
                    {
                        let swap_node = ui_kit::find(&ui.root, "swap");
                        let waiting = swap_node
                            .and_then(|n| ui_kit::find(n, "swap_waiting"))
                            .map(|n| n.visible)
                            .unwrap_or(false);
                        let coach_btn = swap_node
                            .and_then(|n| ui_kit::find(n, "coach"))
                            .map(|n| n.visible)
                            .unwrap_or(false);
                        let panel = swap_node.map(|n| n.visible).unwrap_or(false);
                        let prev_wait = SWAP_WAIT.swap(waiting, Ordering::Relaxed);
                        let prev_coach = SWAP_COACH.swap(coach_btn, Ordering::Relaxed);
                        SWAP_PANEL.store(panel, Ordering::Relaxed);
                        if (waiting && !prev_wait) || (panel && prev_coach && !coach_btn) {
                            SWAP_ARMED.store(true, Ordering::Relaxed);
                            SWAP_APPLIED.store(0, Ordering::Relaxed);
                            config::llog("swaparm: 코치 위임 감지 → 내 팀 포지션 배정");
                        }
                    }
                    // 위임되었을 때만 내 팀 order 를 쓴다(게임이 되돌릴 수 있어 소수 회 재시도).
                    if config::get().swap_force != 0
                        && SWAP_ARMED.load(Ordering::Relaxed)
                        && picks_done >= 10
                        && SWAP_APPLIED.load(Ordering::Relaxed) < SWAP_APPLY_MAX
                    {
                        let t2 = MY_IS_T2.load(Ordering::Relaxed);
                        let off = if t2 { 0x1b0usize } else { 0x198 };
                        let vo = if t2 { hooks::O_PICK2 } else { hooks::O_PICK1 };
                        if let Some((cur, want, _)) = swap_plan(scene, off, vo) {
                            if cur != want {
                                let (_, ptr, _) = rd(off);
                                for (i, &w) in want.iter().enumerate() {
                                    let a = ptr as usize + i * 8;
                                    if hooks::ptr_ok(a) {
                                        unsafe { core::ptr::write(a as *mut u64, w) };
                                    }
                                }
                                SWAP_APPLIED.fetch_add(1, Ordering::Relaxed);
                                config::llog(&format!(
                                    "swapset(내팀): +{off:x} {cur:?} -> {want:?} (n={})",
                                    SWAP_APPLIED.load(Ordering::Relaxed)
                                ));
                            }
                        }
                    }
                    // ── 확정 버튼 게이트 ──────────────────────────────────────
                    //   "지금 배정보다 더 잘 맞출 수 있는가"로 판단한다.
                    //   완전 배정이 불가능한 경우엔 **최대한 맞춘 상태면 통과**시킨다(유저 지시).
                    {
                        let t2 = MY_IS_T2.load(Ordering::Relaxed);
                        let off = if t2 { 0x1b0usize } else { 0x198 };
                        let vo = if t2 { hooks::O_PICK2 } else { hooks::O_PICK1 };
                        let bad = match swap_plan(scene, off, vo) {
                            Some((cur, _want, best_n)) => {
                                let masks = swap_masks(scene, vo).unwrap_or_default();
                                !masks.is_empty() && order_matched(&masks, &cur) < best_n
                            }
                            None => false, // 재료 부족 → 막지 않는다(fail-open)
                        };
                        SWAP_BAD.store(bad, Ordering::Relaxed);
                        hooks::set_swap_block(bad);
                    }
                    let line = format!("swapvec: {} {}", dump(0x198), dump(0x1b0));
                    let sig = {
                        use std::hash::{Hash, Hasher};
                        let mut h = std::collections::hash_map::DefaultHasher::new();
                        line.hash(&mut h);
                        h.finish()
                    };
                    if SWAPVEC_SIG.swap(sig, Ordering::Relaxed) != sig {
                        config::llog(&line);
                    }
                }
            }
            // ★스왑 확정 버튼 비활성 표시 — 숨기지 않고 **회색 처리**한다(유저 지시 2026-08-22).
            //   클릭 차단은 훅 SC(0x1d7bc10)가 담당하고, 여기서는 보이기만 죽인다.
            //   게임 기본 스타일의 "비활성" 룩(회색 글자 + 어두운 배경)을 흉내낸다.
            {
                let bad = SWAP_BAD.load(Ordering::Relaxed);
                let prev = CONFIRM_HIDDEN.swap(bad, Ordering::Relaxed);
                if bad || prev {
                    if let Some(btn) = ui_kit::find_mut(&mut ui.root, "swap")
                        .and_then(|n| ui_kit::find_mut(n, "bottom"))
                        .and_then(|n| ui_kit::find_mut(n, "confirm"))
                    {
                        let (fg, bg) = if bad {
                            (ui_kit::Rgba::new(0.45, 0.47, 0.50, 1.0), ui_kit::Rgba::new(0.10, 0.12, 0.13, 1.0))
                        } else {
                            (ui_kit::Rgba::new(1.0, 1.0, 1.0, 1.0), ui_kit::Rgba::new(0.12, 0.45, 0.33, 1.0))
                        };
                        // 진단: 이 노드가 어떤 러너인지 1회 기록(색 세터가 안 먹는 원인 파악).
                        if !CONFIRM_PROBED.swap(true, Ordering::Relaxed) {
                            config::llog(&format!(
                                "swapprobe: confirm kind={} runner={} children={}",
                                ui_kit::kind(btn),
                                ui_kit::runner_type_name(btn),
                                btn.child.len()
                            ));
                        }
                        let a = ui_kit::label_set_color(btn, fg);
                        let b = ui_kit::rect_set_back_all(btn, bg);
                        let _ = ui_kit::text_set_deep(btn, "");
                        if bad != prev {
                            config::llog(&format!(
                                "swapgate: 확정버튼 {} (label={a} rect={b})",
                                if bad { "비활성 표시" } else { "복구" }
                            ));
                        }
                    }
                }
            }
            // ★버튼 러너 바이트 덤프 — 비활성 플래그 오프셋을 찾기 위한 대조군 수집.
            //   대조: (A) 스왑 확정 `confirm`(활성) vs (B) 환경설정 `current_database_edit/change`(비활성).
            //   둘 다 .ui 상 `color_icon_button` + `@main#primary_button` 계열이라 레이아웃이 같다.
            {
                let dump_runner = |n: &Node, tag: &str, done: &AtomicBool| {
                    if done.load(Ordering::Relaxed) {
                        return;
                    }
                    let ty = ui_kit::runner_type_name(n);
                    let Some(b) = ui_kit::runner_base(n, &ty) else {
                        config::llog(&format!("btnprobe {tag}: runner_base 실패 ty={ty}"));
                        done.store(true, Ordering::Relaxed);
                        return;
                    };
                    let mut hex = String::new();
                    for o in 0..0xa0usize {
                        match ui_kit::runner_rd_u8(b, o) {
                            Some(v) => hex.push_str(&format!("{v:02x}")),
                            None => hex.push_str("??"),
                        }
                        if o % 16 == 15 {
                            hex.push(' ');
                        }
                    }
                    config::llog(&format!("btnprobe {tag}: ty={ty} base=0x{b:x}
  {hex}"));
                    done.store(true, Ordering::Relaxed);
                };
                if let Some(n) = ui_kit::find(&ui.root, "swap")
                    .and_then(|n| ui_kit::find(n, "bottom"))
                    .and_then(|n| ui_kit::find(n, "confirm"))
                {
                    if n.visible {
                        dump_runner(n, "확정(활성)", &BTNPROBE_A);
                    }
                }
                if let Some(n) = ui_kit::find(&ui.root, "current_database_edit")
                    .and_then(|n| ui_kit::find(n, "change"))
                {
                    dump_runner(n, "변경하기(비활성)", &BTNPROBE_B);
                }
            }
            // ★hookA(포지션 적합도 마스크) 상태를 항상 로그에 — 스왑 배정이 우리 마스크를
            //   보고 있는지 판정하는 재료. 값이 크게 변할 때만 한 줄.
            {
                let fire = hooks::CNT_MASK_FIRE.load(Ordering::Relaxed);
                let call = hooks::CNT_MASK_CALL.load(Ordering::Relaxed);
                let adj = hooks::CNT_MASK_ADJ.load(Ordering::Relaxed);
                let bucket = fire / 2000;
                if MASKSTAT_SIG.swap(bucket, Ordering::Relaxed) != bucket {
                    config::llog(&format!(
                        "maskstat: install={} fire={fire} call={call} adj={adj}",
                        hooks::INSTALL_STATE.load(Ordering::Relaxed)
                    ));
                }
            }
            // 디버그 카운터 주기 flush
            if cfg.debug {
                let t = FLUSH_TICK.fetch_add(1, Ordering::Relaxed);
                if t % 600 == 599 {
                    let rdx: Vec<u64> = hooks::DBG_RDX
                        .iter()
                        .map(|s| s.load(Ordering::Relaxed))
                        .filter(|&v| v != u64::MAX)
                        .collect();
                    let names_len = NAMES.get().map(|n| n.len()).unwrap_or(0);
                    config::dlog(&format!(
                        "counters: GY(cell={} paint={}) CK={} CB(seen={} cut={}) DQ(fix={}) CP(seen={} swap={}) | am_hist={:?} am_pen={} seen={} veto={} failopen={} st(e/f/b/c)={}/{}/{}/{} mask_fire={} mask_call={} mask_adj={} A={} C={} D={} E={} CM={} RC={} rc_seen={} rw_seen={} rw_live={} dp_seen={} dp_live={} rc_filt={} rc_inj={} ag0={} agp={} min_stk={} cm_seen={} cm_rej={} cm_redir={} ui_q={} ui_block={} rdx={:?} model_cnt={} max_rdx={} names={}",
                        hooks::CNT_GY_CELL.load(Ordering::Relaxed),
                        hooks::CNT_GY_PAINT.load(Ordering::Relaxed),
                        hooks::CNT_CK_BLOCK.load(Ordering::Relaxed),
                        hooks::CNT_CB_SEEN.load(Ordering::Relaxed),
                        hooks::CNT_CB_CUT.load(Ordering::Relaxed),
                        hooks::CNT_DQ_FIX.load(Ordering::Relaxed),
                        hooks::CNT_CP_SEEN.load(Ordering::Relaxed),
                        hooks::CNT_CP_SWAP.load(Ordering::Relaxed),
                        hooks::AM_COUNT_HIST
                            .iter()
                            .map(|a| a.load(Ordering::Relaxed))
                            .collect::<Vec<_>>(),
                        hooks::CNT_ARGMAX_PEN.load(Ordering::Relaxed),
                        CNT_SEEN.load(Ordering::Relaxed),
                        CNT_VETO.load(Ordering::Relaxed),
                        CNT_FAILOPEN.load(Ordering::Relaxed),
                        ST_EMPTY.load(Ordering::Relaxed),
                        ST_FEAS.load(Ordering::Relaxed),
                        ST_BROKEN.load(Ordering::Relaxed),
                        ST_CONF.load(Ordering::Relaxed),
                        hooks::CNT_MASK_FIRE.load(Ordering::Relaxed),
                        hooks::CNT_MASK_CALL.load(Ordering::Relaxed),
                        hooks::CNT_MASK_ADJ.load(Ordering::Relaxed),
                        hooks::INSTALL_STATE.load(Ordering::Relaxed),
                        hooks::INSTALL_STATE_C.load(Ordering::Relaxed),
                        hooks::INSTALL_STATE_D.load(Ordering::Relaxed),
                        hooks::INSTALL_STATE_E.load(Ordering::Relaxed),
                        hooks::INSTALL_STATE_CM.load(Ordering::Relaxed),
                        hooks::INSTALL_STATE_RC.load(Ordering::Relaxed),
                        hooks::CNT_RC_SEEN.load(Ordering::Relaxed),
                        hooks::CNT_RW_SEEN.load(Ordering::Relaxed),
                        hooks::CNT_RW_LIVE.load(Ordering::Relaxed),
                        hooks::CNT_DP_SEEN.load(Ordering::Relaxed),
                        hooks::CNT_DP_LIVE.load(Ordering::Relaxed),
                        hooks::CNT_RC_FILT.load(Ordering::Relaxed),
                        hooks::CNT_RC_INJ.load(Ordering::Relaxed),
                        hooks::CNT_AG_LEN0.load(Ordering::Relaxed),
                        hooks::CNT_AG_LENP.load(Ordering::Relaxed),
                        hooks::MIN_STK.load(Ordering::Relaxed),
                        hooks::CNT_CM_SEEN.load(Ordering::Relaxed),
                        hooks::CNT_CM_BLOCK.load(Ordering::Relaxed),
                        hooks::CNT_CM_REDIR.load(Ordering::Relaxed),
                        hooks::CNT_UI_QUERY.load(Ordering::Relaxed),
                        hooks::CNT_UI_BLOCK.load(Ordering::Relaxed),
                        rdx,
                        hooks::MASK_MODEL_CNT.load(Ordering::Relaxed),
                        hooks::MASK_MAX_RDX.load(Ordering::Relaxed),
                        names_len,
                    ));
                }
            }
        }));
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    config::load();
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(PosLockExt);
    reg.add_draft_score_hook(PosLockDraftAi);
    reg
}

declare_mod!(init);
