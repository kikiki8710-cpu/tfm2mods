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

mod crashlog;   // ★VEH 크래시 로거(2026-08-27)
mod config;
mod hooks;
mod i18n;
mod icon_data;
mod inject;

#[path = r"C:\tfm2mods\ui_kit\ui_kit.rs"]
mod ui_kit;

use config::MASK_ALL;
use engine_core::ui::length::Length;
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
// NAMES 는 **로스터가 바뀌면 재게시**(세이브 전환·워크샵 토글), MASKS 는 상태 버전이 바뀔 때 재계산.
/// ★사용 가능 챔피언 목록. ~~`OnceLock`(프로세스당 1회 캡처)~~ → **로스터가 바뀌면 재게시**.
///   ⚠1회 캡처였을 때: ①워크샵 챔프를 켜도 재시작 전엔 목록에 안 나오고 ②다른 세이브를 불러와도
///     **이전 세이브의 챔프가 남아 있었다**(2026-08-23 유저 제보).
///   해제는 하지 않는다(leak) — 다른 스레드가 이전 슬라이스를 들고 있을 수 있고, 로스터 변경은
///   세이브 전환·워크샵 토글 때만 일어나 횟수가 극히 적다. `masks()` 와 같은 방식.
static NAMES_PTR: AtomicPtr<Vec<String>> = AtomicPtr::new(std::ptr::null_mut());
/// 마지막으로 게시한 로스터의 서명(내용 해시 + 길이). 같은 길이·다른 구성도 잡는다.
static ROSTER_SIG: AtomicU64 = AtomicU64::new(0);
/// ★재캡처 요청 플래그. **매 프레임 해시를 돌리지 않기 위한 것** — 로스터가 바뀔 수 있는
///   순간에만 켜고, 다음 InGame 프레임이 소비한다.
///   켜는 곳 = ①설정 모달 열기(유저가 목록을 보는 시점) ②세이브 밖으로 나감(세이브 전환)
///   ③최초 1회(초기 캡처). 클릭 콜백엔 `db` 가 없어서 플래그로 이월한다.
static ROSTER_DIRTY: AtomicBool = AtomicBool::new(true);

pub(crate) fn names() -> Option<&'static Vec<String>> {
    let p = NAMES_PTR.load(Ordering::Acquire);
    if p.is_null() {
        None
    } else {
        Some(unsafe { &*p })
    }
}
fn publish_names(v: Vec<String>) {
    NAMES_PTR.store(Box::into_raw(Box::new(v)), Ordering::Release);
}
/// 로스터 서명 — **길이만으로는 부족**하다(같은 수의 다른 챔프 구성이 가능).
fn roster_sig(ids: &[String]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for s in ids {
        for b in s.as_bytes() {
            h ^= *b as u64;
            h = h.wrapping_mul(0x100_0000_01b3);
        }
        h ^= 0xff;
        h = h.wrapping_mul(0x100_0000_01b3);
    }
    h ^ ((ids.len() as u64) << 32)
}
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
    names()
}

// ── 설정 팝업 그리드: 한글 이름 가나다순 정렬 ─────────────────────────────
//   라벨은 i18n 태그(`#asset/...?description.{id}.name`)로 넘겨 게임이 해석하므로
//   모드는 한글 문자열을 모른다 ⟹ 정렬하려면 i18n 을 우리가 읽어야 한다.
//   게임 자산은 bundle.game_data(1.1GB, 포맷 미해독)라 **언팩/모드 i18n 파일들을 병합**한다.
//   실측(2026-08-22): base+mods+workshop 병합 시 94/94 커버(누락 0).
//   ★한글 음절은 유니코드가 곧 가나다순이라 단순 문자열 비교로 정렬된다.
/// 언어별 id→표시명 맵(지연 캐시). 키 = `base.json` 의 lang 값 그대로("ko"/"en"/"ja"/…).
static LANG_MAPS: std::sync::Mutex<
    Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
> = std::sync::Mutex::new(None);
/// ★정렬 결과 캐시 + 그걸 만든 시그니처(언어 + 로스터 크기).
///   ⚠구현은 `OnceLock` 이었다 — **세션 중 언어가 바뀌면 낡은 정렬이 고정**된다
///     (게임정보 탭은 즉시 따라가는데 우리만 옛 언어 순서로 남음).
static SORTED_CHAMPS: std::sync::Mutex<Option<(String, u64, Vec<String>)>> =
    std::sync::Mutex::new(None);

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
fn parse_lang_names(
    text: &str,
    lang: &str,
    out: &mut std::collections::HashMap<String, String>,
) {
    let Some(ko) = text.find(&format!("\"{lang}\"")) else { return };
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
fn load_names(lang: &str) -> std::collections::HashMap<String, String> {
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
            parse_lang_names(&t, lang, &mut out);
        }
    }
    out
}

/// ★현재 게임 언어 = `<게임 설치 폴더>\config\gamease.json` 의 `lang`(RE 확정 · 유일 소스).
///   게임의 i18n 해석기가 매 해석마다 이 값으로 언어 블록을 고르므로 **정의상 표시 언어와 동일**하다.
///   Steam/Windows 언어가 아니다. 파싱은 `i18n` 모듈이 이미 하고 있으니 그대로 쓴다.
fn cur_lang() -> String {
    i18n::current_lang()
}

/// 언어별 id→표시명 맵(최초 1회 로드 후 캐시).
fn lang_map(lang: &str) -> std::collections::HashMap<String, String> {
    {
        let g = LANG_MAPS.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(m) = g.as_ref().and_then(|m| m.get(lang)) {
            return m.clone();
        }
    }
    let built = load_names(lang);
    let mut g = LANG_MAPS.lock().unwrap_or_else(|e| e.into_inner());
    g.get_or_insert_with(Default::default).insert(lang.to_string(), built.clone());
    built
}

/// ★게임과 **같은 정렬 키**. 표시명이 없으면 게임과 동일하게 태그 문자열 자체가 키가 된다
///   (`description.<id>.name` — 라틴 소문자라 한글 이름들보다 앞에 온다. 그 위치까지 게임과 같아야
///    순서가 비트동일하다).
fn sort_key(id: &str, map: &std::collections::HashMap<String, String>) -> String {
    let key = id.to_ascii_lowercase();
    map.get(&key)
        .cloned()
        .unwrap_or_else(|| format!("description.{key}.name"))
        .to_lowercase()
}

/// 설정 팝업용: **현재 게임 언어의 표시명** 순으로 정렬된 챔피언 id 목록.
///   이름을 못 찾은 챔프는 뒤로 보내고 id 순으로(표시는 되되 순서만 뒤).
pub fn sorted_champs() -> Option<Vec<String>> {
    let champs = names()?;
    let lang = cur_lang();
    // 캐시 히트 = 같은 언어 + 같은 로스터(서명). ⚠길이만 보면 세이브 전환 시 같은 수의 다른
    //   구성을 못 잡는다.
    let sig = ROSTER_SIG.load(Ordering::Relaxed);
    {
        let g = SORTED_CHAMPS.lock().unwrap_or_else(|e| e.into_inner());
        if let Some((l, n, v)) = g.as_ref() {
            if l == &lang && *n == sig {
                return Some(v.clone());
            }
        }
    }
    // ★게임(게임정보 탭)의 정렬을 그대로 재현한다(0.5.6 RE 확정):
    //   키 = 현재 lang 의 i18n 표시명 → `to_lowercase()` → **(키, id) 튜플의 순수 바이트 비교**.
    //   로케일 콜레이션이 없으므로 한글은 코드포인트 순 = 가나다순이 된다.
    //   ~~구: 밴픽 그리드 순회 순서(hooks::grid_order) 를 1순위로 썼다~~ → **폐기**:
    //     그건 게임이 *그때의 언어로* 만든 순서라 언어를 바꾸면 낡고, 밴픽을 한 번 지나가야만
    //     수집된다. 표시명으로 직접 정렬하면 두 문제가 같이 사라진다.
    let map = lang_map(&lang);
    let mut v: Vec<String> = champs.clone();
    v.sort_by(|a, b| sort_key(a, &map).cmp(&sort_key(b, &map)).then_with(|| a.cmp(b)));
    if config::get().debug {
        let head: Vec<&str> = v.iter().take(8).map(|s| s.as_str()).collect();
        config::dlog(&format!(
            "sort: lang={lang} names={} map={} head={head:?}",
            v.len(),
            map.len()
        ));
        // ★워크샵/모드 추가 챔프 진단(2026-08-27): 목록에 있는데 안 보인다는 제보 대응.
        //   id 는 로스터에 있어도 i18n 표시명이 없으면 **영문 id 로 뜨고 정렬 위치도 예상 밖**이라
        //   유저 눈엔 "리스트가 갱신 안 된 것"처럼 보인다 ⟹ id·표시명·인덱스를 같이 남긴다.
        {
            let base: std::collections::HashSet<&str> = map.keys().map(|s| s.as_str()).collect();
            let extra: Vec<String> = v
                .iter()
                .enumerate()
                .filter(|(_, id)| !base.contains(id.to_ascii_lowercase().as_str()))
                .map(|(i, id)| format!("{i}:{id}=\"{}\"", disp_name(&id.to_ascii_lowercase())))
                .collect();
            config::dlog(&format!(
                "roster: 총 {}종 | i18n 표시명 없는 챔프 {}개 {:?}",
                v.len(),
                extra.len(),
                extra
            ));
        }
    }
    *SORTED_CHAMPS.lock().unwrap_or_else(|e| e.into_inner()) = Some((lang, sig, v.clone()));
    Some(v)
}

static CNT_VETO: AtomicU64 = AtomicU64::new(0);
static CNT_SEEN: AtomicU64 = AtomicU64::new(0);
static CNT_FAILOPEN: AtomicU64 = AtomicU64::new(0);
static FLUSH_TICK: AtomicU64 = AtomicU64::new(0);
/// score_pick raw-ctx 오프셋 진단 throttle(총 라인 상한).
static DBG_CTXN: AtomicU64 = AtomicU64::new(0);
/// veto 시점 ally vs 유저 씬 대조 로그 throttle.
static DBG_VETON: AtomicU64 = AtomicU64::new(0);
/// veto 조기반환 단계 카운터(어디서 새는지 확정용).
static ST_EMPTY: AtomicU64 = AtomicU64::new(0);
static ST_FEAS: AtomicU64 = AtomicU64::new(0);
static ST_BROKEN: AtomicU64 = AtomicU64::new(0);
static DBG_FEASN: AtomicU64 = AtomicU64::new(0);
static ST_CONF: AtomicU64 = AtomicU64::new(0);
static DBG_CONFN: AtomicU64 = AtomicU64::new(0);
static DBG_MULTIN: AtomicU64 = AtomicU64::new(0);
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
        let nm = |i: usize| names().and_then(|v| v.get(i)).map(|s| s.as_str()).unwrap_or("?");
        let mut out = Vec::with_capacity(len);
        for k in 0..len {
            let v = core::ptr::read((ptr + k * 8) as *const u64) as usize;
            out.push(format!("{}#{v}", nm(v)));
        }
        out.join(",")
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

/// 현재 밴픽 룰(스타일, 실효 밴카드 수). `None` = 아직 Database 를 못 읽음(메인메뉴 등).
fn rule_info() -> (u8, Option<usize>) {
    config::cur_rule()
}

/// 팝업 그리드를 현재 탭·상태에 맞게 채운다(변경 시에만).
fn fill_grid(root: &mut Node) {
    let pos = SEL_POS.load(Ordering::Relaxed);
    let ver = config::state_version();
    let Some(all) = sorted_champs() else { return };
    // ── 편의 필터(제한 설정과 무관): 클래스 + 이름 검색 ──────────────────────
    //   ⚠필터가 걸리면 셀 인덱스 ↔ 챔피언 대응이 sorted_champs 와 달라진다.
    //     그리드와 **클릭 라우트가 같은 목록**(VISIBLE)을 봐야 엉뚱한 챔프가 토글되지 않는다.
    let class_sel = CLASS_SEL.load(Ordering::Relaxed);
    let search = SEARCH_TXT
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .clone();
    let champs: Vec<String> = all
        .iter()
        .filter(|c| {
            let lower = c.to_ascii_lowercase();
            if class_sel > 0 {
                let want = (class_sel - 1) as u8;
                let g = CHAMP_CAT.lock().unwrap_or_else(|e| e.into_inner());
                match g.iter().find(|(k, _)| *k == lower).map(|(_, v)| *v) {
                    Some(c) if c == want => {}
                    _ => return false,
                }
            }
            if !search.is_empty() {
                // ★현재 언어 표시명 + id 로 매칭한다(영어 로케일 유저가 영문 이름으로 찾을 수 있게).
                let name = disp_name(&lower).to_lowercase();
                if !name.contains(&search) && !lower.contains(&search) {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();
    *VISIBLE.lock().unwrap_or_else(|e| e.into_inner()) = champs.clone();
    let ready = icon_data::READY.load(Ordering::Relaxed) as u64;
    {
        let (st, bn) = rule_info();
        config::set_rule(st, bn);
    }
    // ★라벨은 **캐시된 실효 룰**을 쓴다 — rule_info() 는 밴카드가 "자동"이면 매번 None 이라
    //   관측/복원으로 이미 확정된 값을 화면에서 못 보여준다.
    let (style, ban_opt) = config::cur_rule();
    let ban_count = ban_opt.unwrap_or(0);
    let rule_sig = ((style as u64) << 40) ^ ((ban_count as u64) << 32);
    let filter_sig = {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        (class_sel, &search).hash(&mut h);
        h.finish()
    };
    let sig = (pos as u64) << 48
        ^ ver
        ^ ((champs.len() as u64) << 32)
        ^ (ready << 47)
        ^ rule_sig
        ^ filter_sig;
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
        ui_kit::label_set(n, &i18n::trf("summary_fmt", &[("pos", &i18n::pos_name(pos))]));
    }
    if let Some(n) = ui_kit::find_mut(pop, "rule_label") {
        let name = i18n::tr(match style {
            2 => "rule_fearless_hard",
            1 => "rule_fearless",
            _ => "rule_classic",
        });
        ui_kit::label_set(n, &i18n::trf("rule_label", &[("name", &name)]));
    }
    if let Some(n) = ui_kit::find_mut(pop, "ban_label") {
        // ★밴카드 설정이 "자동"이면 원본값이 0 이라 실효값을 알 수 없다(밴픽 씬을 봐야 확정).
        //   0 을 그대로 "0장"으로 보여주면 최소 선택 수도 함께 틀리게 읽힌다(유저 제보 2026-08-23).
        let s = if ban_opt.is_none() {
            i18n::tr("ban_reading")
        } else {
            i18n::trf("ban_label", &[("n", &ban_count.to_string())])
        };
        ui_kit::label_set(n, &s);
    }
    if let Some(n) = ui_kit::find_mut(pop, "min_label") {
        let s = if ban_opt.is_none() {
            i18n::tr("min_unknown")
        } else if comp_size > 1 {
            i18n::trf(
                "min_shared",
                &[
                    ("need", &base_need.to_string()),
                    ("count", &comp_size.to_string()),
                    ("have", &union_size.to_string()),
                    ("want", &union_need.to_string()),
                ],
            )
        } else {
            i18n::trf("min_label", &[("need", &base_need.to_string())])
        };
        ui_kit::label_set(n, &s);
    }
    if let Some(n) = ui_kit::find_mut(pop, "count_label") {
        // ★[2026-09-02] `cnt` 는 이제 **실제로 켠 개수**(config::pos_count = named_count).
        //   제한 판정은 `pos_pool`(명시 + 미지정)로 하므로 둘이 다르면 풀도 같이 보여 준다 —
        //   안 그러면 "21개만 켰는데 최소 20을 어떻게 넘지?" 가 화면에서 설명되지 않는다.
        let pool = config::pos_pool(pos);
        let s = if cnt == 0 {
            i18n::tr("count_zero")
        } else if pool > cnt {
            i18n::trf(
                "count_label_pool",
                &[("n", &cnt.to_string()), ("pool", &pool.to_string())],
            )
        } else {
            i18n::trf("count_label", &[("n", &cnt.to_string())])
        };
        ui_kit::label_set(n, &s);
    }
    if let Some(n) = ui_kit::find_mut(pop, "warning_min") {
        // cnt==0 = 전체 허용(제한 없음) → 경고 없음. 아니면 두 제약 체크:
        //  ①이 포지션 자체 ≥ base_need  ②겹친 컴포넌트 공유풀 ≥ union_need.
        let s = if ban_opt.is_none() {
            // ★밴카드가 "자동"이면 최소 선택 수를 계산할 수 없다 ⟹ 제한을 아예 걸지 않는다.
            //   (유저 지시 2026-08-23: 그 상태를 화면에 명시하고 해결 방법까지 적을 것.)
            i18n::tr("warn_ban_unknown")
        } else if cnt == 0 {
            String::new()
        } else if cnt < base_need {
            // ★최소 미달 = **제한 없음 취급**(유저 지시 2026-08-23). 경고가 아니라 현재 상태 안내다.
            let stale = config::pos_stale(pos);
            let tail = if stale > 0 {
                i18n::trf("warn_min_tail_stale", &[("n", &stale.to_string())])
            } else {
                String::new()
            };
            i18n::trf(
                "warn_min",
                &[
                    ("need", &base_need.to_string()),
                    ("more", &(base_need - cnt).to_string()),
                    ("tail", &tail),
                ],
            )
        } else if comp_size > 1 && union_size < union_need {
            i18n::trf(
                "warn_shared",
                &[
                    ("count", &comp_size.to_string()),
                    ("want", &union_need.to_string()),
                    ("have", &union_size.to_string()),
                ],
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

/// ⚠**로그·진단 전용**(로그는 한국어 고정). 유저에게 보이는 문자열은 `i18n::pos_name()` 을 쓸 것.
pub(crate) const POS_NAMES_KR: [&str; 5] = ["탑", "정글", "미드", "원딜", "서폿"];

/// 제한 챔프 마스크들에 서로 다른 포지션을 하나씩 줄 수 있는가 (5×5 이하 백트래킹).
/// ★최대 이분매칭 크기(마스크 ≤6, 포지션 5) — "이미 깨진" 상태에서도 판정 가능.
///   feasible(=완전매칭)은 한 번 깨지면 전 후보가 false 라 fail-open 으로 무력화됐다
///   (실측: 미드 2명 픽 후 block=0). 최대매칭은 그 뒤에도 "빈 라인을 채우는 픽"을 구분한다.
/// 챔프 id → **한글** 이름. ⚠**로그·진단 전용**(로그는 한국어 고정).
///   유저에게 보이는 이름은 `disp_name()`(현재 게임 언어)을 쓸 것.
pub(crate) fn kr_name(id: &str) -> String {
    lang_map("ko")
        .get(&id.to_ascii_lowercase())
        .cloned()
        .unwrap_or_else(|| id.to_string())
}

/// 챔프 id → **현재 게임 언어** 표시명. 없으면 id 그대로(검색 매칭용이라 태그 폴백은 쓰지 않는다).
pub(crate) fn disp_name(id: &str) -> String {
    lang_map(&cur_lang())
        .get(&id.to_ascii_lowercase())
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
/// ~~확정 시 상대 팀 order 를 우리 배정으로 맞춘다~~ → ❌**클라 경로로는 불가(2026-08-22 RE 확정)**.
///   `FUN_141d7bc10`(확정 핸들러) 디컴 결과, 보내는 order 는 **한 팀 것뿐**이다:
///     `lVar8 = ctrl+0x1b0; if (*(p1+0xb8) == *(p1+0xc0)) lVar8 = ctrl+0x198;`
///     → 그 하나만 memcpy 해서 `ClientPacket::SwapDone{match_id, order, flag}` 로 보낸다.
///   즉 **상대 팀 배정은 클라가 보내지 않는다** = 서버(worker.rs `compute_rule_swap_order`)가 정한다.
///   ⟹ 여기서 상대 벡터를 써봐야 패킷에 안 실리고, 화면만 잠깐 어긋나 오해를 부른다.
///   적용 범위: **클라 스왑 확정 경로로는** 불가. 개입하려면 worker.rs 계층을 잡아야 한다.
pub(crate) fn apply_opponent_swap() {
    static ONCE: AtomicBool = AtomicBool::new(false);
    if !ONCE.swap(true, Ordering::Relaxed) {
        config::llog("swapset(상대): 생략 — 확정 패킷은 내 팀 order 만 보낸다(RE 2026-08-22)");
    }
    if true {
        return;
    }
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

// -- ★자유 슬롯(정배치 불가 자리) 회계 -- 2026-08-23 ------------------------------
//   어떤 포지션의 풀이 말라(밴/픽으로 전소) 아무 챔프도 못 앉히게 되면 그 자리는 "아무나" 앉는다.
//   ★그 자유 자리를 **몇 번째 픽으로 채울지는 순서와 무관**하다 -- 스왑 배정은 5픽이 끝난 뒤
//     순열로 정해지기 때문(유저 지적 2026-08-23).
//   구 구현은 `helps()` 한 조건뿐이라 자유 픽이 **마지막 픽에서만** 통과했다
//   (앞 픽에서는 "새 라인을 채우는" 챔프만 합법 -> 중복 라인 차단).
//   -> 예산제로 교체: 자유 슬롯 = 팀픽수 - 달성가능최대치. 예산이 남아 있으면 전면 허용,
//     한 장 쓰면(= 이미 찬 라인을 또 집으면) 다시 `helps()` 로 조인다.
//   ⚠모든 포지션이 건강하면 달성가능최대치 = 팀픽수 -> 예산 0 -> 구 동작과 완전히 동일하다.

/// masks 의 부분집합을 서로 다른 포지션에 배정해 **정확히 S 를 덮을 수 있는** S 들의 비트셋.
fn cover_sets(masks: &[u8]) -> u32 {
    let mut reach: u32 = 1; // bit0 = 공집합
    for &m in masks {
        let mut nxt = reach;
        for s in 0..32usize {
            if reach & (1 << s) == 0 {
                continue;
            }
            for p in 0..5usize {
                if m & (1 << p) != 0 && s & (1 << p) == 0 {
                    nxt |= 1 << (s | (1 << p));
                }
            }
        }
        reach = nxt;
    }
    reach
}

/// 남은 풀이 **서로 다른 챔프로 S 전부를 덮을 수 있는** S 들의 비트셋(Hall 조건).
fn pool_cover_sets(pool: &[u8]) -> u32 {
    let mut by = [0usize; 32];
    for &m in pool {
        by[(m & MASK_ALL) as usize] += 1;
    }
    let mut avail = [0usize; 32];
    for t in 1..32usize {
        avail[t] = (1..32usize).filter(|m| m & t != 0).map(|m| by[m]).sum();
    }
    let mut ok: u32 = 1; // 공집합은 항상 가능
    for s in 1..32usize {
        let mut good = true;
        let mut t = s; // s 의 모든 비공집합 부분집합 순회
        while t > 0 {
            if avail[t] < t.count_ones() as usize {
                good = false;
                break;
            }
            t = (t - 1) & s;
        }
        if good {
            ok |= 1 << s;
        }
    }
    ok
}

/// 이 팀이 최종적으로 설정대로 앉힐 수 있는 **최대 인원**(0..=team).
///   pinned = 이미 픽한 챔프 마스크 **전부**(무제한 MASK_ALL 도 포함), pool = 아직 고를 수 있는 마스크.
pub(crate) fn achievable(pinned: &[u8], pool: &[u8], slots: usize) -> usize {
    let pin = cover_sets(pinned);
    let pl = pool_cover_sets(pool);
    let mut best = 0usize;
    for a in 0..32usize {
        if pin & (1 << a) == 0 {
            continue;
        }
        let rest = MASK_ALL as usize & !a;
        let mut b = rest;
        loop {
            if pl & (1 << b) != 0 && b.count_ones() as usize <= slots {
                let v = a.count_ones() as usize + b.count_ones() as usize;
                if v > best {
                    best = v;
                }
            }
            if b == 0 {
                break;
            }
            b = (b - 1) & rest;
        }
    }
    best
}

/// 아직 **안 쓴** 자유 슬롯 수. >0 이면 이번 픽은 무엇을 골라도 최종 정배치 인원이 줄지 않는다.
pub(crate) fn free_left(pinned: &[u8], pool: &[u8], team: usize) -> usize {
    let slots = team.saturating_sub(pinned.len());
    let target = achievable(pinned, pool, slots);
    let budget = team.saturating_sub(target); // 총 자유 슬롯
    let used = pinned.len().saturating_sub(max_match(pinned)); // 이미 쓴 자유 슬롯
    budget.saturating_sub(used)
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
                names().and_then(|v| v.get(i)).map(|s| s.as_str()).unwrap_or("?")
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
                    let name = names()
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
/// ★[2026-09-03 진단] swapdiag 중복 억제용 시그니처.
static SWAPDIAG_SIG: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// 한 판에서 내 팀 order 를 다시 쓸 수 있는 최대 횟수(게임과 무한 줄다리기 방지).
const SWAP_APPLY_MAX: usize = 240;
/// 유저가 직접 두 칸을 맞바꿨는가(= 자동 교정 중지). 위임하면 해제.
static USER_SWAPPED: AtomicBool = AtomicBool::new(false);
/// 챔피언 id(소문자) → 클래스 인덱스(0=전사 1=원거리 2=마법사 3=전투보조 4=암살자).
///   ★라이브 DB 조회라 **패치·워크샵으로 챔프가 추가돼도 그대로 따라간다.**
///   챔프 수가 바뀌면 재캡처한다(세션 중 다른 세이브를 열어 구성이 달라지는 경우).
pub(crate) static CHAMP_CAT: std::sync::Mutex<Vec<(String, u8)>> =
    std::sync::Mutex::new(Vec::new());
/// CHAMP_CAT 를 캡처했을 때의 챔프 수(0=미캡처).
/// 설정 팝업 클래스 필터(0=전체, 1..=5 → 클래스 0..=4). 드롭다운 선택 인덱스와 같다.
static CLASS_SEL: AtomicUsize = AtomicUsize::new(0);
/// 설정 팝업 이름 검색어(소문자).
static SEARCH_TXT: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
/// 현재 그리드에 보이는 챔프 목록(셀 인덱스와 1:1) — 필터가 걸리면 sorted_champs 와 달라진다.
static VISIBLE: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
// ── ★세이브별 설정 (2026-08-23 유저 지시) ──────────────────────────────────
//   정본 = 그 세이브 안의 mod save data. 파일(state.txt)은 새 세이브의 씨앗.
//   ⚠엔진 쓰기는 **큐잉**이라 기록 직후 `mod_save_get_string` 이 옛값을 줄 수 있다
//     (tfm2_champion_exclude v0.4.0 실측). ⟹ 기록 본문을 로컬에 선반영한다.
//   ⚠설정은 **게임이 세이브를 저장할 때** 함께 디스크로 간다(mod save data 의 성질).
const SAVE_KEY: &str = "positions";
const SAVE_NS_VERSION: usize = 1;
/// 이 세이브의 설정을 이미 로드했나. 세이브 밖(메인메뉴) 프레임에서 false 로 리셋 → 세이브 전환 대응.
static SAVE_LOADED: AtomicBool = AtomicBool::new(false);
/// 게이트 중단 사유 코드(바뀔 때만 로그) + 스탬프 정지 카운터.
static EXIT_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
static STAMP_STALL: AtomicUsize = AtomicUsize::new(0);

/// ★상시 진단 — 게이트가 판정까지 못 가고 어디서 빠져나갔나. 사유가 바뀔 때만 한 줄.
fn exit_note(code: u64, detail: &str) {
    if EXIT_SIG.swap(code, Ordering::Relaxed) == code {
        return;
    }
    config::slog(&format!("게이트 중단[{code}]: {detail}"));
}

/// 마지막으로 남긴 게이트 상태 서명(내용이 바뀔 때만 로그).
static GATE_SIG: AtomicU64 = AtomicU64::new(u64::MAX);

/// ★상시 진단 — 제한이 실제로 걸리고 있는지 한 줄. debug 플래그와 무관(내용 변화 시에만 기록).
///   "제한이 안 먹힌다" 류 제보를 로그만으로 판정하기 위한 것(2026-08-23).
fn gate_note(picks: usize, free: usize, blocked: usize, feasible: bool) {
    let amask = config::active_pos_mask();
    let sig = (picks as u64)
        | (free.min(255) as u64) << 8
        | (blocked.min(65535) as u64) << 16
        | (feasible as u64) << 32
        | (amask as u64) << 40;
    if GATE_SIG.swap(sig, Ordering::Relaxed) == sig {
        return;
    }
    let need = config::cur_min_required();
    config::slog(&format!(
        "게이트: 내픽 {picks} / 자유슬롯 {free} / 차단 {blocked}개 / 후보있음={feasible} / 제한활성={amask:05b} / 최소={} / 포지션별{:?}",
        if need == usize::MAX { "미정".to_string() } else { need.to_string() },
        (0..5).map(config::pos_count).collect::<Vec<_>>()
    ));
}

/// mod save 를 **연속으로** 못 읽은 프레임 수. 이만큼 지나야 "정말 설정이 없다"로 단정한다.
///   ★★세이브 로드 직후 몇 프레임은 `mod_save_get_string` 이 아직 None 을 준다
///     (tfm2_champion_exclude 03_시행착오: 쓰기는 큐잉·읽기는 지연 — "None 이어도 캐시를 지우지 말 것").
///     로스터는 프로세스당 1회만 캡처되므로 **두 번째 세이브 로드부터는 첫 프레임에 바로** 판정에
///     들어간다 ⟹ 유예 없이는 매번 "설정 없음"으로 오판해 기존 설정을 덮어썼다(2026-08-23 실사고).
static SAVE_MISS: AtomicUsize = AtomicUsize::new(0);
const SAVE_MISS_GRACE: usize = 240; // ~4초(60fps)
/// UI 확인이 이월한 기록 대기 본문(다음 InGame 프레임의 post_update 가 소비).
static PENDING_SAVE: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

/// 밴픽 씬에서 관측한 **실효** 밴카드 수(설정이 "기본"일 때 이걸 쓴다). 0=미관측.
static BAN_CNT_EFF: AtomicUsize = AtomicUsize::new(0);
/// 위 값을 밴픽 씬에서 **실제로 관측했나**. ⚠0장도 유효한 설정이라 값만으론 구분 못 한다.
static BAN_CNT_SEEN: AtomicBool = AtomicBool::new(false);
/// 검색 지우기 버튼이 눌렸음(다음 프레임에 text_edit 비움).
static SEARCH_CLEAR: AtomicBool = AtomicBool::new(false);
/// 드롭다운 옵션 주입 1회 완료.
/// ★드롭다운 옵션을 **어떤 언어로** 주입했는지. `None` = 아직 주입 전.
///   ⚠구현은 `AtomicBool`(1회 주입)이었다 — 네이티브 드롭다운 옵션은 ABI 로 한 번 밀어 넣으면
///     그걸로 끝이라, **언어가 바뀌어도 옛 언어 문자열이 그대로 남는다**. 비-한국어 폰트엔 한글
///     글리프가 없어 □ 로 깨진다(2026-08-23 실측: 그리드·우측 패널은 영어인데 드롭다운만 □□).
///   ⟹ 주입 언어를 기억해 두고 달라지면 다시 주입한다.
static DD_LANG: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
/// 클래스 필터 드롭다운(네이티브).
static CLASS_DD: ui_kit::NativeDropdown = ui_kit::NativeDropdown::new("class_filter");
/// 클래스 라벨(드롭다운 옵션 순서 = CLASS_SEL 인덱스).
/// 클래스 필터 드롭다운 라벨 키(표시명은 i18n — `text/poslock.i18n`).
const CLASS_KEYS: [&str; 6] = [
    "class_all",
    "class_melee",
    "class_range",
    "class_magician",
    "class_util",
    "class_assassin",
];
fn class_labels() -> Vec<String> {
    CLASS_KEYS.iter().map(|k| i18n::tr(k)).collect()
}

/// ★유저(관리 중인) 팀 id — `db.player_team_id()` 로 관리화면에서 캡처. u64::MAX=미확보.
pub(crate) static PLAYER_TEAM: AtomicU64 = AtomicU64::new(u64::MAX);
static PID_NONZERO: AtomicBool = AtomicBool::new(false);
/// 확보된 내 팀 id(없으면 None).
pub(crate) fn player_team() -> Option<u64> {
    match PLAYER_TEAM.load(Ordering::Relaxed) {
        u64::MAX => None,
        v => Some(v),
    }
}
/// post_update 프레임 카운터(주기 작업·클릭 디바운스용).
static FRAME: AtomicU64 = AtomicU64::new(0);
/// 코치 위임 클릭 디바운스(같은 클릭이 여러 번 들어오는 것 방지).
static COACH_CLICK_AT: AtomicU64 = AtomicU64::new(0);
/// 관찰 전용 클릭 필터 재등록 추적(코치 위임 버튼).
static OBS_LAST: AtomicUsize = AtomicUsize::new(usize::MAX);
/// 직전 프레임에 본 내 팀 order(전위 감지용).
static LAST_MY_ORDER: std::sync::Mutex<Option<Vec<u64>>> = std::sync::Mutex::new(None);
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
/// 전 매치 스왑order 사후 스캔(진단) — 주기 카운터·중복억제 서명.
static ORDSCAN_TICK: AtomicU64 = AtomicU64::new(0);
static ORDSCAN_SIG: AtomicU64 = AtomicU64::new(0);
/// ★확정 버튼(ColorIconButtonRunner) 원래 색 백업 — [normal, hover, active] × [icon,sub,text,btn].
///   러너의 `disabled` 불리언은 private 라 못 켠다. 대신 **스타일 4상태 중
///   normal/hover/active 의 색을 게임 자체 `disabled` 색으로 덮어써서** 비활성처럼 보이게 한다.
///   (Style<P>{normal,hover,active,disabled} · P{icon,sub,text,btn} 전부 pub —
///    SDK rlib 에 컴파일타임 대조로 확정, 오프셋 하드코딩 없음 = 패치 내성 있음.)
static CONFIRM_SAVED: std::sync::Mutex<Option<[BtnColors; 3]>> = std::sync::Mutex::new(None);
type BtnColors = [common::color::Color; 4];

fn cib_get(p: &game_view::ColorIconButtonRunnerProperty) -> BtnColors {
    [p.icon.color, p.sub.color, p.text.color, p.btn.color]
}
fn cib_set(p: &mut game_view::ColorIconButtonRunnerProperty, c: BtnColors) {
    p.icon.color = c[0];
    p.sub.color = c[1];
    p.text.color = c[2];
    p.btn.color = c[3];
}
/// 스타일 asset 에 disabled 가 정의돼 있지 않아 normal 과 같으면(=시각차 0)
/// 직접 어둡게 만든 값으로 폴백한다.
fn dim(c: common::color::Color) -> common::color::Color {
    common::color::Color { r: c.r * 0.4, g: c.g * 0.4, b: c.b * 0.4, a: c.a }
}
// ── 커서 위치 (Win32) — 엔진 UIEvent 에 마우스이동/호버 이벤트가 없어서 직접 읽는다.
//    (UIEvent 변종 실측: Click/RightClick/CheckboxSelect/TreeViewSelect/TreeViewRightClick/
//     TreeViewMove/TextEditComplete/Remove/Custom/Changed — 호버 없음.)
#[repr(C)]
#[derive(Clone, Copy)]
struct POINT {
    x: i32,
    y: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct RECTW {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}
#[link(name = "user32")]
extern "system" {
    fn GetCursorPos(p: *mut POINT) -> i32;
    fn ScreenToClient(h: usize, p: *mut POINT) -> i32;
    fn GetForegroundWindow() -> usize;
    fn GetClientRect(h: usize, r: *mut RECTW) -> i32;
}
/// 커서를 UI 좌표계(= `GameUI.rect`, 실측 1920x1080)로 환산.
///   ⚠`GameUI.scale`(실측 3)로 나누면 안 된다 — scale 은 픽셀아트 배율이지
///     클라이언트 픽셀↔UI 유닛 비율이 아니다(2026-08-22 tipdbg 로 확인).
///   실제 비율 = 클라이언트 크기(GetClientRect) 대 UI rect.
fn cursor_ui(uiw: f32, uih: f32) -> Option<(f32, f32)> {
    unsafe {
        let h = GetForegroundWindow();
        if h == 0 {
            return None;
        }
        let mut p = POINT { x: 0, y: 0 };
        if GetCursorPos(&mut p) == 0 || ScreenToClient(h, &mut p) == 0 {
            return None;
        }
        let mut r = RECTW::default();
        if GetClientRect(h, &mut r) == 0 {
            return None;
        }
        let (cw, ch) = ((r.right - r.left) as f32, (r.bottom - r.top) as f32);
        if cw < 1.0 || ch < 1.0 || uiw < 1.0 || uih < 1.0 {
            return None;
        }
        Some((p.x as f32 * uiw / cw, p.y as f32 * uih / ch))
    }
}
/// 확정 버튼 툴팁 문구(비활성 사유). `hint: String` = 러너 pub 필드.
const CONFIRM_HINT: &str = "스왑이 올바르지 않습니다.";
/// 원래 hint 백업(대개 빈 문자열).
static CONFIRM_HINT0: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
/// gray=true → 비활성 색 / false → 백업 복구. 러너 타입이 다르면 false.
fn confirm_gray(n: &mut Node, gray: bool) -> bool {
    let Some(b) = ui_kit::runner_base(n, "ColorIconButtonRunner") else {
        return false;
    };
    unsafe {
        let r = &mut *(b as *mut game_view::ColorIconButtonRunner);
        // 툴팁: 비활성 사유를 hint 에 넣는다(원래 값은 1회 백업).
        {
            let mut h0 = CONFIRM_HINT0.lock().unwrap_or_else(|e| e.into_inner());
            if h0.is_none() {
                config::llog(&format!("confirm hint 원본=\"{}\"", r.hint));
                *h0 = Some(r.hint.clone());
            }
            let want: &str = if gray {
                CONFIRM_HINT
            } else {
                h0.as_deref().unwrap_or("")
            };
            if r.hint != want {
                r.hint = want.to_string();
            }
        }
        let mut g = CONFIRM_SAVED.lock().unwrap_or_else(|e| e.into_inner());
        if g.is_none() {
            *g = Some([
                cib_get(&r.style.normal),
                cib_get(&r.style.hover),
                cib_get(&r.style.active),
            ]);
        }
        let saved = g.unwrap();
        if gray {
            let mut d = cib_get(&r.style.disabled);
            // disabled 가 normal 과 동일 = 스타일에 비활성 룩이 없다 → 직접 어둡게.
            if d == saved[0] {
                d = [dim(d[0]), dim(d[1]), dim(d[2]), dim(d[3])];
            }
            cib_set(&mut r.style.normal, d);
            cib_set(&mut r.style.hover, d);
            cib_set(&mut r.style.active, d);
        } else {
            cib_set(&mut r.style.normal, saved[0]);
            cib_set(&mut r.style.hover, saved[1]);
            cib_set(&mut r.style.active, saved[2]);
        }
    }
    true
}

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
/// 직전 프레임의 드래프트 진행 수(줄면 새 세트 = 캐시 리셋 신호).
static LAST_DRAFT_TOTAL: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);
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
/// ★★픽 차례인 진영 — **픽 슬롯의 `in_turn` 만** 보고, **정확히 한쪽만** 켜져 있을 때만 확신한다.
///   ⚠구현 `side_in_turn`(폐기)은 `Blue픽 → Blue밴 → Red픽 → Red밴` 순으로 훑었다.
///     밴 페이즈가 끝난 뒤에도 **Blue 밴 슬롯의 `in_turn` 이 남아 있으면 항상 Blue** 를 돌려줘,
///     Red 차례를 전부 "남의 차례"로 만들었다 ⟹ 회색화가 드래프트 내내 통째로 죽었다.
///     실측 2026-08-23: `in_turn blue=false red=true` 인데 `side_in_turn=Blue`.
///   ⟹ 픽 게이트는 밴 슬롯을 보면 안 된다(밴 차례는 호출 전에 이미 걸러진다).
///   모호(양쪽 다 켜짐/양쪽 다 꺼짐)하면 `None` — 호출측이 fail-open 한다.
fn pick_side_in_turn(root: &Node) -> Option<Side> {
    let b = side_root(root, Side::Blue, true)
        .map(any_in_turn_visible)
        .unwrap_or(false);
    let r = side_root(root, Side::Red, true)
        .map(any_in_turn_visible)
        .unwrap_or(false);
    match (b, r) {
        (true, false) => Some(Side::Blue),
        (false, true) => Some(Side::Red),
        _ => None,
    }
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
    // ★상시 진단: 여기서 빠져나가면 제한이 아예 안 걸린다. 어느 조건 때문인지 남긴다.
    if !cfg.enabled || !cfg.user_pick_block || !config::any_restricted() {
        {
            let sig = 0xE000
                | (cfg.enabled as u64)
                | (cfg.user_pick_block as u64) << 1
                | (config::any_restricted() as u64) << 2
                | (config::active_pos_mask() as u64) << 8;
            if GATE_SIG.swap(sig, Ordering::Relaxed) != sig {
                let need = config::cur_min_required();
                config::slog(&format!(
                    "게이트 OFF: enabled={} user_pick_block={} 제한있음={} / 제한활성={:05b} / 최소={} / 포지션별{:?}",
                    cfg.enabled,
                    cfg.user_pick_block,
                    config::any_restricted(),
                    config::active_pos_mask(),
                    if need == usize::MAX { "미정".to_string() } else { need.to_string() },
                    (0..5).map(config::pos_count).collect::<Vec<_>>()
                ));
            }
        }
        clear();
        return;
    }
    let Some(names) = names() else {
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
    if scene < 0x10000 {
        exit_note(1, "밴픽 씬 미포착(scene_cap=0) — scene_step 훅 미설치/무효화 의심");
        return;
    }
    if stamp == last {
        // 정상(연출·전환 프레임). 다만 **오래 멈춰 있으면** 훅이 안 도는 것이므로 한 번 남긴다.
        let n = STAMP_STALL.fetch_add(1, Ordering::Relaxed) + 1;
        if n == 600 {
            config::slog("게이트: 씬 스탬프 600프레임 정지 — scene_step 훅 미발화 의심");
        }
        return;
    }
    STAMP_STALL.store(0, Ordering::Relaxed);
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
        exit_note(2, "씬 4벡터(픽/밴) 읽기 실패");
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
        exit_note(3, &format!("씬 값 이상 fmt={fmt} ban_cnt={ban_cnt}"));
        return; // 씬 형태 이상 → 일시적이므로 직전 목록 유지
    }
    // ★밴픽 씬의 ban_count(+0x3c0) = 이 경기에 **실제 적용된** 밴카드 수.
    //   평소엔 SDK 로 계산한 값과 같아야 한다(같지 않으면 연습실 등 특수 룰이거나 우리 계산이 틀린 것)
    //   ⟹ 실경기 값을 우선하고, 어긋나면 로그를 남겨 공식을 검증한다.
    //   ⚠씬 형태 검증(위 return)을 통과한 뒤에 기록해야 쓰레기값을 안 남긴다.
    BAN_CNT_EFF.store(ban_cnt, Ordering::Relaxed);
    BAN_CNT_SEEN.store(true, Ordering::Relaxed);
    {
        let (st, calc) = config::cur_rule();
        if calc != Some(ban_cnt) {
            if ban_cnt <= 10 {
                config::set_rule(st, Some(ban_cnt));
            }
            config::dlog(&format!(
                "밴카드: SDK계산={calc:?} vs 실경기={ban_cnt} → 실경기 채택 (스타일={st})"
            ));
        }
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
        exit_note(6, "밴 차례(포지션 무관)");
        clear(); // 밴 차례 = 포지션 무관, 게다가 회색이면 밴 자체를 못 하게 된다
        return;
    }
    // ── 내 팀 판정 ─────────────────────────────────────────────────────────
    //   ★★2026-08-22 정정(유저 보고: Bo3 2세트부터 상대 차례에 내 차례처럼 회색화):
    //     **세트마다 진영(그리고 T1/T2)이 바뀐다.** UI 로 한 번 배워서 캐시하면 다음 세트에 틀린다.
    //     게다가 "위임 버튼이 보이면 내 차례" 가정도 틀렸다 — 위임 버튼은 **상대 차례에도 보인다**.
    //   ⟹ ①내 팀 = 씬에서 매번 읽는다: `*(*(scene+0x388)+0xe3b8)` vs T1(+0x3d0)/T2(+0x3d8).
    //      ②진영↔픽벡터 매핑(SIDE_MAP)은 확정 픽 수 대조로만 배우고, **드래프트가 바뀌면 리셋**.
    //      ③내 차례가 아니면 회색화하지 않는다.
    {
        // 드래프트 교체 감지: 진행 수가 줄면 새 세트 → UI 유도 캐시 리셋.
        let last_total = LAST_DRAFT_TOTAL.swap(total as i64, Ordering::Relaxed);
        if (total as i64) < last_total {
            SIDE_MAP.store(0, Ordering::Relaxed);
            MY_SIDE.store(-1, Ordering::Relaxed);
            config::llog("draft: 새 세트 감지 → 진영 캐시 리셋");
        }
    }
    //   진영 → 씬 픽벡터 매핑: 양쪽 확정 픽 수가 다를 때 대조해서 확정(그 뒤 캐시).
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
    let side_map = SIDE_MAP.load(Ordering::Relaxed);
    // ★내 팀 판정 = **SDK `db.player_team_id()`**(관리화면에서 캡처, 세이브 내내 불변) vs team1(+0x3d0).
    //   ⚠구버전들이 쓰던 `*(database+0xe3b8)` 은 실측에서 team1 과 같은 값을 돌려줘 어긋났다(폐기).
    //   미확보 시에만 구 폴백.
    let my_is_t2 = match player_team() {
        Some(pid) => pid != sel_team,
        None => sel_team == t2_team,
    };
    MY_IS_T2.store(my_is_t2, Ordering::Relaxed);
    // 내 진영(로그·차례 판정용) — SIDE_MAP 이 확정된 뒤에만 알 수 있다.
    let my_side: i32 = match side_map {
        1 => {
            if my_is_t2 {
                1
            } else {
                0
            }
        } // Blue=picks_a(T1)
        2 => {
            if my_is_t2 {
                0
            } else {
                1
            }
        } // Blue=picks_e(T2)
        _ => -1,
    };
    MY_SIDE.store(my_side, Ordering::Relaxed);
    // ③내 차례가 아니면 회색화하지 않는다(상대 차례에 회색이 뜨던 버그).
    if my_side >= 0 {
        // ★모호하면(None) 억제하지 않는다 = fail-open. 확신이 있을 때만 회색화를 건너뛴다.
        if let Some(sd) = pick_side_in_turn(root) {
            let turn_side = if sd == Side::Blue { 0 } else { 1 };
            if turn_side != my_side {
                // ★판정 재료 전부: side_map 이 뒤집혔는지 / side_in_turn 이 한쪽만 주는지 구분용.
                let bt = side_root(root, Side::Blue, true)
                    .map(any_in_turn_visible)
                    .unwrap_or(false);
                let rt = side_root(root, Side::Red, true)
                    .map(any_in_turn_visible)
                    .unwrap_or(false);
                exit_note(
                    4,
                    &format!(
                        "내 차례 아님(turn={turn_side} my={my_side}) | pid={:?} team1={sel_team} team2={t2_team} is_t2={my_is_t2} side_map={side_map} | UI확정 blue={} red={} / 씬픽 t1={} t2={} | in_turn blue={bt} red={rt}",
                        player_team(),
                        side_done_count(root, Side::Blue),
                        side_done_count(root, Side::Red),
                        picks_a.len(),
                        picks_e.len()
                    ),
                );
                // 확신(한쪽만 in_turn)이 있을 때만 억제한다. 모호하면 위 `if let` 이 아예 안 걸려
                // 그대로 진행 = fail-open(제한이 죽는 것보다 상대 차례 회색이 낫다).
                clear();
                return;
            }
        }
    }
    let my_picks_raw: &Vec<String> = if my_is_t2 { &picks_e } else { &picks_a };
    // ★빈 문자열은 "선택 대기" 자리표시자일 수 있으므로 실제 픽에서 제외.
    let my_picks: Vec<&String> = my_picks_raw.iter().filter(|s| !s.is_empty()).collect();
    // ★★2026-08-22 정정: 종료 판정을 `total >= base + picks_n` 로 하면
    //   **마지막 픽(20/20)에서 제한이 통째로 풀렸다**(유저 보고). 상대 픽까지 합산한 total 은
    //   자리표시자·집계 시점 차이로 한 스텝 일찍 상한에 닿는다.
    //   → **내 팀 픽이 다 찼는가**로 판정한다(팀당 픽 수 = picks_n/2). 내 픽이 남아 있으면 항상 활성.
    if my_picks.len() >= picks_n / 2 {
        exit_note(5, &format!("내 픽 완료 {}/{}", my_picks.len(), picks_n / 2));
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
    // ★자유 슬롯 예산(2026-08-23 유저 지적): 포지션 풀이 말라 정배치가 불가능해진 자리는
    //   아무나 앉혀도 되고, **그 자리를 몇 번째 픽으로 채울지는 순서와 무관**하다.
    //   구 동작은 `helps()` 만 봐서 자유 픽을 마지막 픽에서만 허용했다 → 예산이 남아 있으면 전면 허용.
    //   ⚠회계는 무제한(MASK_ALL) 픽까지 포함한 "내 픽 전부"로 한다(그 챔프도 슬롯 하나를 쓴다).
    {
        let pinned_all: Vec<u8> = my_picks.iter().map(|p| config::mask_of(p)).collect();
        let pool: Vec<u8> = names
            .iter()
            .enumerate()
            .filter(|(_, n)| !taken.contains(n.to_ascii_lowercase().as_str()))
            .map(|(i, _)| masks.get(i).copied().unwrap_or(MASK_ALL))
            .collect();
        let free = free_left(&pinned_all, &pool, picks_n / 2);
        if free > 0 {
            gate_note(my_picks.len(), free, 0, true);
            clear();
            return;
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
            // ★유저에게 보이는 툴팁 사유(차단된 챔프가 갈 수 있는 포지션들) → i18n.
            let label = (0..5)
                .filter(|p| m & (1 << p) != 0)
                .map(i18n::pos_name)
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
    gate_note(my_picks.len(), 0, published.len(), any_feasible);
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
    let Some(names) = names() else { return };
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
            // ★게임 언어 변경 추종(~5초 주기 확인). 안 하면 .ui 만 번역되고 조합 문자열은 옛 언어로 남아
            //   영어 폰트 + 한글 = 글자 깨짐이 된다(2026-08-23 실측).
            i18n::poll_lang();
            // ★밴픽 씬이 죽었으면 캡처 포인터를 무효화 — 워커 스레드의 stale read 차단.
            hooks::scene_gc();
            // 챔피언 목록 1회 캡처(관리화면 프레임에서도 Scene::InGame 매치).
            if let Scene::InGame { data } = scene {
                let db = data.db();
                // ★★밴픽 룰 = **게임 자신의 결정 함수**로 확정(2026-08-23 RE, SDK 공개 API).
                //   `GamePlayOption::ban_count_or_default(rule, 사용가능챔프수)` 가 밴카드 "자동"까지
                //   풀어 준다. 자동 공식(GameRule::ban_count_for_available_champions):
                //     2V2=1 / 3V3=2 / 4V4=2 고정, **5V5 = 챔프 40마리 미만 2, 40 이상 3**
                //     (임계값 = `GameRule::THREE_BAN_AVAILABLE_CHAMPION_COUNT` = 40).
                //   ⟹ raw 오프셋(db+0x720/+0x737)도, 밴픽 씬 관측도 더 이상 필요 없다.
                {
                    let o = &db.game_play_option;
                    let ban = o.ban_count_or_default(
                        o.match_game_rule_for_current_mode(),
                        db.available_champions.len(),
                    );
                    let style = match o.banpick_style {
                        game_core::BanpickStyle::Classic => 0u8,
                        game_core::BanpickStyle::Fearless => 1,
                        game_core::BanpickStyle::FearlessHard => 2,
                    };
                    if ban <= 10 {
                        config::set_rule(style, Some(ban));
                    }
                }
                // ★★내 팀 id = **SDK 공개 API `db.player_team_id()`** (2026-08-23).
                //   관리화면도 `Scene::InGame` 이라 이어하기 직후 바로 잡힌다. 세이브 내내 불변.
                //   ⚠tfm2_item_tactics 실측 교훈 이식: 조합테스트 등 팀 개념 없는 컨텍스트에서
                //     **0 을 반환**한다 → 비0 을 한 번이라도 봤으면 0 으로 후퇴시키지 않는다.
                {
                    let pid = db.player_team_id() as u64;
                    if pid < 10000 {
                        if pid != 0 {
                            if PLAYER_TEAM.swap(pid, Ordering::Relaxed) != pid {
                                PID_NONZERO.store(true, Ordering::Relaxed);
                                config::llog(&format!("myteam: player_team_id={pid} (SDK)"));
                            }
                        } else if !PID_NONZERO.load(Ordering::Relaxed)
                            && PLAYER_TEAM.swap(0, Ordering::Relaxed) != 0
                        {
                            config::llog("myteam: player_team_id=0 (SDK · 잠정)");
                        }
                    }
                }
                // ★챔피언 클래스(전사/원거리/마법사/전투보조/암살자) 1회 캡처 — 설정 팝업 필터용.
                //   기본 챔프 = `db.champion_info(id).category()`, 워크샵 챔프 = 시트의 `mod_champions[].category`.
                //   (게임 정보 탭이 워크샵 챔프까지 필터하는 이유가 이것 — 파일 파싱·UI 학습 불요.)
                // ★로스터 갱신 — 요청이 있을 때만 확인한다(매 프레임 해시 금지).
                //   확인 자체는 싸지만 필요 없는 일이고, 실제로 바뀌는 순간은 정해져 있다.
                if ROSTER_DIRTY.swap(false, Ordering::Relaxed) || names().is_none() {
                    let ids: Vec<String> = db.available_champions.clone();
                    let sig = roster_sig(&ids);
                    // 서명이 같으면 게시하지 않는다(불필요한 마스크 재계산·정렬 무효화 방지).
                    if !ids.is_empty() && ROSTER_SIG.swap(sig, Ordering::Relaxed) != sig {
                        // 클래스(전사/원거리/마법사/전투보조/암살자) — 설정 팝업 필터용.
                        //   기본 챔프 = `db.champion_info(id).category()`,
                        //   워크샵 챔프 = 시트의 `mod_champions[].category`.
                        let cat_idx = |c: &ChampionCategory| -> u8 {
                            match c {
                                ChampionCategory::Melee => 0,
                                ChampionCategory::Range => 1,
                                ChampionCategory::Magician => 2,
                                ChampionCategory::Util => 3,
                                ChampionCategory::Assassin => 4,
                            }
                        };
                        let mut m: Vec<(String, u8)> = Vec::new();
                        for id in ids.iter() {
                            if let Some(c) = db.champion_info(id) {
                                m.push((id.to_ascii_lowercase(), cat_idx(&c.category())));
                            }
                        }
                        for e in db.champion_info_sheet.mod_champions.iter() {
                            m.push((e.id.to_ascii_lowercase(), cat_idx(&e.category)));
                        }
                        *CHAMP_CAT.lock().unwrap_or_else(|e| e.into_inner()) = m;
                        if cfg.debug {
                            let mut s = String::from("# 현재 사용 가능한 챔피언 id 목록\n");
                            for id in &ids {
                                s.push_str(id);
                                s.push('\n');
                            }
                            if let Some(d) = mod_dir() {
                                let _ =
                                    std::fs::write(format!("{d}\\champ_pos_lock_champions.txt"), s);
                            }
                        }
                        config::slog(&format!("로스터 갱신: 챔프 {}종", ids.len()));
                        // 화이트리스트의 "지금 없는 챔프" 를 개수에서 제외 + 마스크 재계산 트리거.
                        config::set_roster(&ids);
                        publish_names(ids);
                    }
                }
            }
            // ── ★세이브별 설정 (2026-08-23) ──
            if let Scene::InGame { data } = scene {
                // ①UI 확인이 이월한 기록 대기분을 이 세이브에 기록.
                let pending = PENDING_SAVE.lock().unwrap_or_else(|e| e.into_inner()).take();
                if let Some(body) = pending {
                    if data.can_write_mod_save() {
                        data.mod_save_set_version(MOD_ID, SAVE_NS_VERSION);
                        let ok = data.mod_save_set_string(MOD_ID, SAVE_KEY, &body);
                        config::slog(&format!(
                            "세이브 기록 {}: {}B",
                            if ok { "OK" } else { "거부(FALSE)" },
                            body.len()
                        ));
                        // ★기록 본문을 즉시 반영 — 엔진 쓰기는 큐잉이라 곧바로 다시 읽으면 옛값이 온다.
                        if ok {
                            config::apply_state_text(&body);
                            SAVE_LOADED.store(true, Ordering::Relaxed);
                        }
                    } else {
                        config::slog("세이브 기록 불가: can_write_mod_save=false — 이번 저장은 반영 안 됨");
                    }
                }
                // ②이 세이브의 설정을 최초 1회 로드. 없으면 파일(씨앗)에서 마이그레이션.
                //   ⚠로스터 캡처 전에 하면 전부 "유령"으로 걸러져 빈 설정이 심어진다 → 반드시 대기.
                if !SAVE_LOADED.load(Ordering::Relaxed) && config::roster_ready() {
                    match data.mod_save_get_string(MOD_ID, SAVE_KEY) {
                        Some(txt) => {
                            config::apply_state_text(&txt);
                            SAVE_LOADED.store(true, Ordering::Relaxed);
                            let miss = SAVE_MISS.swap(0, Ordering::Relaxed);
                            config::slog(&format!(
                                "세이브 설정 로드: {}B / 포지션별 {:?} (대기 {miss}프레임)",
                                txt.len(),
                                (0..5).map(config::pos_count).collect::<Vec<_>>()
                            ));
                        }
                        None => {
                            // ★유예: 아직 못 읽었을 뿐일 수 있다. 연속 미스가 임계를 넘어야 단정한다.
                            //   ⚠여기서 early-return 하면 안 된다 — 이 블록은 catch_unwind 클로저
                            //     안이라 post_update 의 나머지(마스크 재계산·훅 설치)까지 건너뛴다.
                            let n = SAVE_MISS.fetch_add(1, Ordering::Relaxed) + 1;
                            if n >= SAVE_MISS_GRACE {
                                // ★설정 저장소 = 이 세이브 하나. 없으면 **제한 없음**(바닐라)이다.
                                //   ⚠아무것도 기록하지 않는다 — 읽기가 늦었을 뿐인 경우 기존 설정을
                                //     파괴한다(2026-08-23 실사고: 빈 본문으로 덮어써서 설정 유실).
                                config::apply_state_text("");
                                SAVE_LOADED.store(true, Ordering::Relaxed);
                                config::slog(&format!(
                                    "이 세이브엔 설정 없음({n}프레임 확인) → 제한 없음 / 로스터 {}종",
                                    names().map(|v| v.len()).unwrap_or(0)
                                ));
                            }
                        }
                    }
                }
            } else {
                // 세이브 밖(메인메뉴/새 게임) → 다음 세이브 진입 때 다시 로드하도록 리셋.
                //   ★설정도 함께 비운다 — 안 비우면 로드가 끝나기 전 프레임에 직전 세이브 설정이 산다.
                SAVE_MISS.store(0, Ordering::Relaxed);
                if SAVE_LOADED.swap(false, Ordering::Relaxed) {
                    config::clear_state();
                    // ★★세이브 종속 전역을 **전부** 리셋한다(2026-08-23 실사고).
                    //   `PLAYER_TEAM` 은 "비0 을 한 번 봤으면 0 으로 후퇴 금지" 가드가 걸려 있다
                    //   (조합테스트가 0 을 돌려주는 것 대응). 그런데 프로세스 전역이라
                    //   **세이브를 갈아타도 이전 세이브의 팀 id 가 살아남아** 새 세이브에서 내 팀을
                    //   오판했다 → 픽 차례를 전부 "남의 차례"로 보고 회색화가 통째로 안 걸렸다.
                    //   `SIDE_MAP`(진영↔픽벡터 대응)도 같은 이유로 리셋.
                    ROSTER_DIRTY.store(true, Ordering::Relaxed); // 세이브가 바뀌면 로스터도 바뀐다
                    PLAYER_TEAM.store(u64::MAX, Ordering::Relaxed);
                    PID_NONZERO.store(false, Ordering::Relaxed);
                    SIDE_MAP.store(0, Ordering::Relaxed);
                    config::slog("세이브 밖으로 나감 → 설정·내팀·진영맵 비움");
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
            // ── 편의 필터 위젯 배선(클래스 드롭다운 + 이름 검색) ────────────────
            if open && present {
                // ①옵션 주입 1회 — ⚠ABI 호출이라 **팝업이 실제로 열린 뒤에만** 부른다.
                //   ⚠라벨은 i18n 조회라 &str 슬라이스를 그 자리에서 만들어 넘긴다.
                let lang = i18n::current_lang();
                let need = DD_LANG.lock().unwrap_or_else(|e| e.into_inner()).as_deref() != Some(lang.as_str());
                if need {
                    let labels = class_labels();
                    let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
                    // 재주입 시 현재 선택을 유지한다(언어만 바뀌었을 뿐 필터는 그대로여야 한다).
                    let sel = CLASS_SEL.load(Ordering::Relaxed) as u64;
                    if CLASS_DD.set_options(&ui.root, &label_refs, sel) {
                        *DD_LANG.lock().unwrap_or_else(|e| e.into_inner()) = Some(lang);
                        config::llog("filter: 클래스 드롭다운 옵션 주입 OK");
                    }
                }
                // ②선택 인덱스 폴링(게임이 클릭 시 runner+0x1788 에 기록).
                if let Some(sel) = CLASS_DD.selected(&ui.root) {
                    let sel = sel.min(CLASS_KEYS.len() - 1);
                    if CLASS_SEL.swap(sel, Ordering::Relaxed) != sel {
                        GRID_SIG.store(u64::MAX, Ordering::Relaxed);
                    }
                }
                // ③검색어(비우기 버튼이 눌렸으면 먼저 지운다).
                if SEARCH_CLEAR.swap(false, Ordering::Relaxed) {
                    if let Some(n) = ui_kit::find_mut(&mut ui.root, "champ_search") {
                        ui_kit::textedit_set(n, "");
                    }
                }
                let cur_txt = ui_kit::find(&ui.root, "champ_search")
                    .and_then(ui_kit::textedit_get)
                    .unwrap_or_default()
                    .trim()
                    .to_ascii_lowercase();
                {
                    let mut g = SEARCH_TXT.lock().unwrap_or_else(|e| e.into_inner());
                    if *g != cur_txt {
                        *g = cur_txt;
                        GRID_SIG.store(u64::MAX, Ordering::Relaxed);
                    }
                }
            }
            if open && present {
                fill_grid(&mut ui.root);
                // ④결과 수 라벨(필터가 걸렸을 때만).
                let shown = VISIBLE.lock().unwrap_or_else(|e| e.into_inner()).len();
                let total = sorted_champs().map(|v| v.len()).unwrap_or(0);
                if let Some(n) = ui_kit::find_mut(&mut ui.root, "filter_count") {
                    let s = if shown == total {
                        String::new()
                    } else {
                        format!("{shown} / {total}")
                    };
                    ui_kit::label_set(n, &s);
                }
            }

            // 클릭 라우팅
            let mut routes: Vec<(String, ui_kit::ClickFn)> = Vec::with_capacity(NCELLS + 16);
            routes.push(ui_kit::route(
                "pos_lock_configure",
                Rc::new(|| {
                    CNT_ROW_CLICK.fetch_add(1, Ordering::Relaxed);
                    POPUP_OPEN.store(true, Ordering::Relaxed);
                    GRID_SIG.store(u64::MAX, Ordering::Relaxed); // 열 때 강제 재채움
                    i18n::poll_now(); // 언어가 바뀌었으면 여는 순간 반영(주기 대기 없이)
                    ROSTER_DIRTY.store(true, Ordering::Relaxed); // 목록을 보는 시점 = 재캡처 시점
                    config::dlog("포지션 제한 버튼 클릭됨");
                }),
            ));
            let close: ui_kit::ClickFn = Rc::new(|| POPUP_OPEN.store(false, Ordering::Relaxed));
            routes.push(ui_kit::route("pos_lock_popup.close", close.clone()));
            routes.push(ui_kit::route("pos_lock_popup.cancel", close));
            routes.push(ui_kit::route(
                "pos_lock_popup.ok",
                Rc::new(|| {
                    // ★정본 = 이 세이브. 유효 챔프만 담는다(유령 id 가 세이브로 넘어가지 않게).
                    let body = config::state_text(true);
                    *PENDING_SAVE.lock().unwrap_or_else(|e| e.into_inner()) = Some(body.clone());
                    // ⚠씨앗 파일(state.txt)은 **덮어쓰지 않는다**. 덮어쓰면 지금 로스터에 없는
                    //   챔프(다른 워크샵 구성의 선택)가 파일에서까지 지워진다 — 실제로 한 번 날렸다
                    //   (2026-08-23: 서폿 22개 → 7개). 파일은 읽기 전용 씨앗으로 둔다.
                    POPUP_OPEN.store(false, Ordering::Relaxed);
                    config::slog(&format!("확인: {}B 기록 대기 (이 세이브)", body.len()));
                }),
            ));
            for (i, t) in TAB_IDS.iter().enumerate() {
                routes.push(ui_kit::route(
                    t,
                    Rc::new(move || SEL_POS.store(i, Ordering::Relaxed)),
                ));
            }
            routes.push(ui_kit::route(
                "search_clear",
                Rc::new(|| SEARCH_CLEAR.store(true, Ordering::Relaxed)),
            ));
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
                        // ★필터된 목록을 본다(그리드와 같은 출처) — 안 그러면 필터 중 엉뚱한 챔프가 토글된다.
                        let v = VISIBLE.lock().unwrap_or_else(|e| e.into_inner());
                        if let Some(c) = v.get(k) {
                            config::toggle(SEL_POS.load(Ordering::Relaxed), &c.to_ascii_lowercase());
                        }
                    }),
                ));
            }
            ui_kit::ensure_clicks(ui, &CLICK_LAST, routes);

            // ★코치 위임 = **버튼 클릭**(유저 확정 2026-08-22). 노드 가시성 변화로 유추하지 말고
            //   클릭 자체를 관찰한다. ⚠소비하지 않는 관찰 전용 필터 — 소비하면 위임이 안 걸린다.
            //   위임을 누르면 "유저가 손댔다" 상태를 풀어 자동 교정을 재개한다.
            ui_kit::ensure_clicks_observe(
                ui,
                &OBS_LAST,
                vec![ui_kit::route(
                    "coach",
                    Rc::new(|| {
                        // 같은 클릭이 여러 프레임에 걸쳐 들어오는 실측(08-22: 11연발) → 디바운스.
                        let now = FRAME.load(Ordering::Relaxed);
                        let last = COACH_CLICK_AT.swap(now, Ordering::Relaxed);
                        USER_SWAPPED.store(false, Ordering::Relaxed);
                        SWAP_ARMED.store(true, Ordering::Relaxed);
                        SWAP_APPLIED.store(0, Ordering::Relaxed);
                        *LAST_MY_ORDER.lock().unwrap_or_else(|e| e.into_inner()) = None;
                        if now.saturating_sub(last) > 30 {
                            config::llog("swaparm: 코치 위임 버튼 클릭 → 내 팀 자동 배정 재개");
                        }
                    }),
                )],
            );

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
                        USER_SWAPPED.store(false, Ordering::Relaxed);
                        *LAST_MY_ORDER.lock().unwrap_or_else(|e| e.into_inner()) = None;
                    }
                    // 위임 감지 — 이제 "재무장" 용도. 위임하면 유저 수동 스왑 존중 상태를 푼다.
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
                            USER_SWAPPED.store(false, Ordering::Relaxed);
                            config::llog("swaparm: 코치 위임 감지 → 자동 교정 재개");
                        }
                    }
                    // ── 내 팀 order 자동 교정 (2026-08-22 재설계) ─────────────────
                    //   ⚠구버전은 "코치 위임 감지" 때만 6회 썼다. 실측 로그(08-22)에
                    //     `swaparm:` 이 **한 번도 안 찍혔다** = 감지 실패 → 위임해도 배정 안 됨.
                    //   ⟹ 감지에 기대지 않고 **스왑 화면이 열려 있는 동안 상시 교정**한다.
                    //     단 유저가 직접 두 칸을 맞바꾼 경우(= 직전 order 대비 **전위 1회**)는
                    //     존중해서 그 판 동안 자동 교정을 멈춘다(위임하면 다시 재개).
                    // ★[2026-09-03 진단] 자동 교정이 왜 안 도는지 갈래를 찍는다(값 변화 시에만).
                    {
                        let sf = config::get().swap_force;
                        let ap = SWAP_APPLIED.load(Ordering::Relaxed);
                        let t2d = MY_IS_T2.load(Ordering::Relaxed);
                        let offd = if t2d { 0x1b0usize } else { 0x198 };
                        let vod = if t2d { hooks::O_PICK2 } else { hooks::O_PICK1 };
                        let planned = swap_plan(scene, offd, vod);
                        let masks_len = unsafe { hooks::read_scene_vec(scene, vod) }
                            .map(|v| v.len() as i64)
                            .unwrap_or(-1);
                        let line = match &planned {
                            Some((cur, want, n)) => format!(
                                "swapdiag: sf={sf} picks={picks_done} applied={ap} usr={} t2={t2d}                                  picklen={masks_len} cur={cur:?} want={want:?} best_n={n} eq={}",
                                USER_SWAPPED.load(Ordering::Relaxed),
                                cur == want
                            ),
                            None => format!(
                                "swapdiag: sf={sf} picks={picks_done} applied={ap} usr={} t2={t2d}                                  picklen={masks_len} plan=None(재료부족 → 교정·차단 모두 스킵)",
                                USER_SWAPPED.load(Ordering::Relaxed)
                            ),
                        };
                        use std::hash::{Hash, Hasher};
                        let mut h = std::collections::hash_map::DefaultHasher::new();
                        line.hash(&mut h);
                        let sg = h.finish();
                        if SWAPDIAG_SIG.swap(sg, Ordering::Relaxed) != sg {
                            config::llog(&line);
                        }
                    }
                    if config::get().swap_force != 0
                        && picks_done >= 10
                        && SWAP_APPLIED.load(Ordering::Relaxed) < SWAP_APPLY_MAX
                    {
                        let t2 = MY_IS_T2.load(Ordering::Relaxed);
                        let off = if t2 { 0x1b0usize } else { 0x198 };
                        let vo = if t2 { hooks::O_PICK2 } else { hooks::O_PICK1 };
                        if let Some((cur, want, n)) = swap_plan(scene, off, vo) {
                            let mut last = LAST_MY_ORDER.lock().unwrap_or_else(|e| e.into_inner());
                            if let Some(prev) = last.as_ref() {
                                if prev.len() == cur.len() && *prev != cur {
                                    let d: Vec<usize> =
                                        (0..cur.len()).filter(|&i| prev[i] != cur[i]).collect();
                                    let transposed = d.len() == 2
                                        && prev[d[0]] == cur[d[1]]
                                        && prev[d[1]] == cur[d[0]];
                                    if transposed && !USER_SWAPPED.swap(true, Ordering::Relaxed) {
                                        config::llog(&format!(
                                            "swapuser: 유저 수동 스왑 감지({prev:?} -> {cur:?}) → 자동 교정 중지"
                                        ));
                                    }
                                }
                            }
                            if !USER_SWAPPED.load(Ordering::Relaxed) && cur != want {
                                let (_, ptr, _) = rd(off);
                                for (i, &w) in want.iter().enumerate() {
                                    let a = ptr as usize + i * 8;
                                    if hooks::ptr_ok(a) {
                                        unsafe { core::ptr::write(a as *mut u64, w) };
                                    }
                                }
                                let c = SWAP_APPLIED.fetch_add(1, Ordering::Relaxed) + 1;
                                if c <= 3 || c % 60 == 0 {
                                    config::llog(&format!(
                                        "swapset(내팀): +{off:x} {cur:?} -> {want:?} (맞춘수={n}/5, {c}회)"
                                    ));
                                }
                                *last = Some(want);
                            } else {
                                *last = Some(cur);
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
            //   ⚠러너가 `ColorIconButtonRunner`(라벨/사각형 아님)라 ui_kit 의
            //     label_set_color/rect_set_back_all 은 **안 먹는다**(2026-08-22 실측 label=false rect=false).
            //   ⟹ SDK 타입을 직접 써서 스타일 색을 바꾼다(오프셋 하드코딩 없음).
            {
                let bad = SWAP_BAD.load(Ordering::Relaxed);
                let prev = CONFIRM_HIDDEN.swap(bad, Ordering::Relaxed);
                let mut btn_rect: Option<(f32, f32, f32, f32)> = None;
                if bad || prev {
                    if let Some(btn) = ui_kit::find_mut(&mut ui.root, "swap")
                        .and_then(|n| ui_kit::find_mut(n, "bottom"))
                        .and_then(|n| ui_kit::find_mut(n, "confirm"))
                    {
                        // ★엔진 레벨 비활성 — `Node.disabled` 는 **pub 필드**였다(SDK 프로브 2026-08-22).
                        //   이걸 켜면 호버 커서(손가락)·호버 효과·클릭이 게임 자체 규칙으로 죽는다.
                        //   러너의 private `disabled` 를 찾을 필요가 없었다(privdump 두 버튼 동일 = 헛다리).
                        btn.disabled = bad;
                        let ok = confirm_gray(btn, bad);
                        btn_rect = Some((btn.rect.x, btn.rect.y, btn.rect.w, btn.rect.h));
                        if bad != prev {
                            config::llog(&format!(
                                "swapgate: 확정버튼 {} (style={ok})",
                                if bad { "비활성 표시" } else { "복구" }
                            ));
                        }
                    }
                }
                // ★비활성 사유 툴팁 — 게임 레이아웃(banpick/layout.ui)의
                //   `#coach_dialogue_button_tooltip`(색박스 + #text 라벨, ignore_event) 을 재사용한다.
                //   ⚠`Node.disabled=true` 를 켜면 게임의 호버 처리가 죽어서 러너 `hint` 는 안 뜬다
                //     (게임의 disabled-hint 툴팁 = `update_disabled_hint_tooltip` 은 옵션 UI 전용).
                //   ⟹ 커서 위치를 Win32 로 직접 읽어 버튼 rect 안이면 우리가 띄운다.
                {
                    let (uiw, uih) = (ui.rect.w, ui.rect.h);
                    let hover = match (bad, btn_rect, cursor_ui(uiw, uih)) {
                        (true, Some((x, y, w, h)), Some((cx, cy))) => {
                            cx >= x && cx <= x + w && cy >= y && cy <= y + h
                        }
                        _ => false,
                    };
                    let prev_tip = TIP_OURS.swap(hover, Ordering::Relaxed);
                    if hover || prev_tip {
                        if let Some(tip) = ui_kit::find_mut(&mut ui.root, "pos_lock_swaptip") {
                            tip.visible = hover;
                            if hover {
                                // ★위치는 `Node.rect` 가 아니라 **layout** 으로 준다.
                                //   rect 는 매 프레임 레이아웃이 덮어쓴다(밴픽 셀 툴팁에서 실측한 교훈).
                                if let Some((x, y, w, _h)) = btn_rect {
                                    let (tw, th) = (240.0f32, 34.0f32);
                                    for l in [
                                        &mut tip.layout.normal,
                                        &mut tip.layout.hover,
                                        &mut tip.layout.active,
                                        &mut tip.layout.disabled,
                                    ] {
                                        l.width = Length::Pixel(tw);
                                        l.height = Length::Pixel(th);
                                        l.x = Length::Pixel(x + (w - tw) * 0.5);
                                        l.y = Length::Pixel(y - th - 8.0);
                                        l.anchor_x = 0.0;
                                        l.anchor_y = 0.0;
                                        l.pivot_x = 0.0;
                                        l.pivot_y = 0.0;
                                    }
                                }
                            }
                            if hover != prev_tip {
                                config::llog(&format!(
                                    "swaptip: {} btn={:?} tip.rect=({},{},{},{})",
                                    if hover { "표시" } else { "숨김" },
                                    btn_rect,
                                    tip.rect.x,
                                    tip.rect.y,
                                    tip.rect.w,
                                    tip.rect.h
                                ));
                            }
                        } else if hover != prev_tip {
                            config::llog("swaptip: pos_lock_swaptip 노드 없음(주입 실패?)");
                        }
                    }
                }
            }

            // ★A(백그라운드 스왑) 진단 — 드래프트가 **끝난 뒤에도** 스냅샷의
            //   vecC(+0x90)/vecD(+0xa8)가 identity 인지 주기적으로 확인한다.
            //   커밋 시점 관측(2026-08-22)은 6매치 전부 identity 였는데, 유저 경기의
            //   진짜 order 는 비-identity 가 나온다 ⟹ 둘 중 하나: ①아직 안 채워짐
            //   ②그 Vec 은 스왑 order 가 아님. 사후 스캔이 이걸 가른다.
            //   함께 결정레코드 tag 분포도 남긴다(스왑 결정이 같은 큐로 오는지).
            if cfg.log_lineups {
                let t = ORDSCAN_TICK.fetch_add(1, Ordering::Relaxed);
                if t % 600 == 599 {
                    let lines = unsafe { hooks::scan_orders() };
                    if !lines.is_empty() {
                        use std::hash::{Hash, Hasher};
                        let mut h = std::collections::hash_map::DefaultHasher::new();
                        lines.hash(&mut h);
                        let sig = h.finish();
                        if ORDSCAN_SIG.swap(sig, Ordering::Relaxed) != sig {
                            let tags: Vec<String> = hooks::CP_TAGS
                                .iter()
                                .enumerate()
                                .filter(|(_, c)| c.load(Ordering::Relaxed) != 0)
                                .map(|(i, c)| format!("{i}:{}", c.load(Ordering::Relaxed)))
                                .collect();
                            let so: Vec<String> = hooks::SO_SKIP
                                .iter()
                                .enumerate()
                                .map(|(i, c)| format!("{i}:{}", c.load(Ordering::Relaxed)))
                                .collect();
                            config::llog(&format!(
                                "ordscan({}건) tag={} | tag7={} 교체={} 스킵[{}]",
                                lines.len(),
                                tags.join(" "),
                                hooks::SO_SEEN.load(Ordering::Relaxed),
                                hooks::CNT_SWAPORDER.load(Ordering::Relaxed),
                                so.join(" ")
                            ));
                            for l in lines.iter().take(24) {
                                config::llog(&format!("  {l}"));
                            }
                        }
                    }
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
                    let names_len = names().map(|n| n.len()).unwrap_or(0);
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
    crashlog::install();   // ★가장 먼저 — 이후 어떤 초기화가 죽어도 RIP 가 남는다
    config::load();
    crashlog::report();
    i18n::load();
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(PosLockExt);
    reg.add_draft_score_hook(PosLockDraftAi);
    reg
}

declare_mod!(init);
