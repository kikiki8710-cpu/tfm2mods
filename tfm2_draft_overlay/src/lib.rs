//! tfm2_draft_overlay — 밴픽(드래프트) 화면에 TFM2.gg 메타 분석을 표시하는 오버레이.
//! ───────────────────────────────────────────────────────────────────────────
//! ★완전 독립: 자체 "체이닝 로더 훅"으로 밴픽 템플릿을 잡아 팝업 조각을 루트에 가산주입.
//!   scrim/ai_adjust/item_tactics 등 같은 에셋게터(0x540ad0)를 후킹하는 모드들과 안전 공존
//!   (진입부 현재 12바이트를 트램펄린에 저장→앞에 끼움=체인. 늦게 설치해 다른 훅 뒤에 붙음. 멱등).
//!   → scrim 이 꺼져 있어도 이 모드 단독으로 동작.
//! 데이터 = TFM2.gg 가 write 한 overlay_data.txt(미리 포맷된 행) → 고정슬롯 row0..row19 에 fill_list.
//!   (분석/포맷은 TFM2.gg, 표시는 여기.) Phase 1: 텍스트 라인 브리지. Phase 2 에서 구조화 + 탭.
#![allow(dead_code)]
use mod_api::*;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;

#[path = r"C:\tfm2mods\ui_kit\ui_kit.rs"]
mod ui_kit;

const MAX_ROWS: usize = 100;
const NUM_TABS: usize = 5; // 메타통계/챔피언정보/메타해석/밴픽코치/모의밴픽
const BPP_H: f32 = 200.0;  // 모의밴픽 컨트롤 패널 높이(gen_popup.py BPP_H 와 일치) — content_h 반영용

// 티어 → 뱃지 배경색 (대시보드 styles.css 와 동일: OP=빨강, 그 외=파랑).
fn tier_color(t: &str) -> ui_kit::Rgba {
    if t == "OP" { ui_kit::Rgba::hex(0xef4b5fff) } else { ui_kit::Rgba::hex(0x5162bbff) }
}
// 승률 라벨: 대시보드가 붙인 접두 마커(\x02=초록/\x03=빨강)를 읽어 색 적용 후 마커 제거.
//   마커 없으면 중립색(흰색). 픽률·표본 등 승률 아닌 값엔 마커가 없어 자동으로 중립.
fn set_wr_label(n: &mut Node, text: &str) {
    let (color, body) = match text.as_bytes().first() {
        Some(&2) => (ui_kit::Rgba::hex(0x5ac77aff), &text[1..]), // \x02 ≥50% 초록
        Some(&3) => (ui_kit::Rgba::hex(0xef6b6bff), &text[1..]), // \x03 <50% 빨강
        Some(&4) => (ui_kit::Rgba::hex(0x6f7686ff), &text[1..]), // \x04 흐릿(사용됨/이월)
        _ => (ui_kit::Rgba::hex(0xe8e8e8ff), text),
    };
    ui_kit::label_set(n, body);
    ui_kit::label_set_color(n, color);
}

// ── 챔피언 이미지 (fog 모드 헬퍼 재사용) ──────────────────────────────────
// ImageRunner source 를 런타임에 `asset/base/aseprite_resources/champions/<id>` 로 세팅 →
//   엔진 aseprite 리졸버가 #sheet/#data 자동 해석해 챔프 전신 그림. 밴픽화면은 전 챔프 로드돼 있어 즉시 표시.
const IMG_PREFIX: &[u8] = b"asset/base/aseprite_resources/champions/";
const IMG_SUFFIX: &[u8] = b"#sheet";
static mut IMG_BUF: [u8; 160] = [0; 160]; // 전신(champ_img) 소스키 버퍼(단일)
// 행별 얼굴 소스키 버퍼: 여러 face ImageRunner가 동시에 서로 다른 챔프를 가리켜야 함
//   → 단일 버퍼 공유 시 엔진이 ptr에서 문자열을 읽을 때 전부 마지막 챔프로 덮여씀 → 행마다 독립 버퍼.
static mut FACE_BUF: [[u8; 96]; MAX_ROWS] = [[0u8; 96]; MAX_ROWS]; // 96=모드챔프 긴 에셋키 대비
const NGRID: usize = 120; // 챔프 그리드 셀 수(10×12). gen_popup.py GCOLS×GROWS 와 일치(가로 1.2배 → 10열).
const GCOLS: usize = 10;  // 그리드 열 수(동적 높이 계산용). gen_popup.py 와 일치.
const GC_H: f32 = 40.0;  // 셀 높이(px). gen_popup.py 와 일치.
static mut GRID_FACE_BUF: [[u8; 96]; NGRID] = [[0u8; 96]; NGRID]; // 그리드 셀별 얼굴 소스키 버퍼
// 자동생성 UV 테이블: (id, u0,v0,u1,v1, frame_w,frame_h) — gen_champ_uv.py.
include!(r"C:\tfm2mods\tfm2_draft_overlay\champ_uv.rs");
fn champ_uv(id: &str) -> Option<(f32, f32, f32, f32, f32, f32)> {
    CHAMP_UV.iter().find(|e| e.0 == id).map(|e| (e.1, e.2, e.3, e.4, e.5, e.6))
}
fn champ_face_uv(id: &str) -> Option<(f32, f32, f32, f32)> {
    CHAMP_FACE.iter().find(|e| e.0 == id).map(|e| (e.1, e.2, e.3, e.4))
}
// 게임 에셋 키(base=asset/base/..., 모드챔프=asset/<모드ns>/...). #sheet 접미는 여기서 안 붙음.
//   정적 표(base + gen 시점 모드) 우선 → 없으면 런타임 워크샵 스캔(RT_CHAMP): gen 이후 설치된 새 모드도 재빌드 없이 자동.
fn champ_key(id: &str) -> Option<String> {
    if let Some(e) = CHAMP_KEY.iter().find(|e| e.0 == id) { return Some(e.1.to_string()); }
    RT_CHAMP.lock().unwrap_or_else(|e| e.into_inner()).iter().find(|e| e.0 == id).map(|e| e.1.clone())
}

// ── 런타임 모드챔프 스캔(새 모드 자동 감지) ────────────────────────────────
//   워크샵 각 팩의 champion/*.data_champion sprite 키 + champion_view face 를 읽어 RT_CHAMP(id→(key,fx,fy)) 구성.
//   정적 표에 없는(gen 이후 설치된) 모드챔프 커버. #anim UV 는 이미 런타임이라 이 키/face 만 채우면 완결.
static RT_CHAMP: Mutex<Vec<(String, String, f32, f32)>> = Mutex::new(Vec::new());
static RT_SCAN_DONE: AtomicBool = AtomicBool::new(false);

fn json_str(txt: &str, key: &str) -> Option<String> {
    let pat = format!("\"{}\"", key);
    let i = txt.find(&pat)? + pat.len();
    let rest = &txt[i..];
    let rest = &rest[rest.find(':')? + 1..];
    let rest = &rest[rest.find('"')? + 1..];
    let e = rest.find('"')?;
    Some(rest[..e].to_string())
}
fn json_num(txt: &str, key: &str) -> Option<f32> {
    let pat = format!("\"{}\"", key);
    let i = txt.find(&pat)? + pat.len();
    let rest = txt[i..].trim_start_matches(|c: char| c == ':' || c.is_whitespace());
    let end = rest.find(|c: char| !(c.is_ascii_digit() || c == '-' || c == '.')).unwrap_or(rest.len());
    rest[..end].parse::<f32>().ok()
}
// champion_view 에서 id 의 face(x,y): "<id>" 뒤 첫 "face" 블록의 x/y (챔프마다 face 존재 가정, 없으면 (0,0)).
fn cv_face(cv: &str, id: &str) -> (f32, f32) {
    let find = || -> Option<(f32, f32)> {
        let idpat = format!("\"{}\"", id);
        let i = cv.find(&idpat)? + idpat.len();
        let f = cv[i..].find("\"face\"")? + i;
        let seg = &cv[f..(f + 96).min(cv.len())];
        Some((json_num(seg, "x").unwrap_or(0.0), json_num(seg, "y").unwrap_or(0.0)))
    };
    find().unwrap_or((0.0, 0.0))
}
fn workshop_dir() -> Option<String> {
    let mut buf = [0u16; 520];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n == 0 || n >= buf.len() { return None; }
    let exe = String::from_utf16_lossy(&buf[..n]);
    let m = exe.find("\\steamapps\\").or_else(|| exe.find("\\SteamApps\\"))?;
    Some(format!("{}\\steamapps\\workshop\\content\\3009300", &exe[..m]))
}
// 최초 밴픽 프레임에 1회 스캔. 실패해도(경로 없음 등) 정적 표로 폴백 → 안전.
fn ensure_rt_scan() {
    if RT_SCAN_DONE.swap(true, Ordering::Relaxed) { return; }
    let ws = match workshop_dir() { Some(w) => w, None => return };
    let packs = match std::fs::read_dir(&ws) { Ok(r) => r, Err(_) => return };
    let mut out: Vec<(String, String, f32, f32)> = Vec::new();
    for pack in packs.flatten() {
        let pdir = pack.path();
        let cv = std::fs::read_to_string(pdir.join("style").join("champion_view.champion_view")).unwrap_or_default();
        let entries = match std::fs::read_dir(pdir.join("champion")) { Ok(r) => r, Err(_) => continue };
        for e in entries.flatten() {
            let p = e.path();
            let fname = match p.file_name().and_then(|s| s.to_str()) { Some(s) => s.to_string(), None => continue };
            if !fname.ends_with(".data_champion") { continue; }
            let id = fname.trim_end_matches(".data_champion").to_string();
            let txt = std::fs::read_to_string(&p).unwrap_or_default();
            let key = match json_str(&txt, "sprite") {
                Some(k) if k.contains("/aseprite_resources/champions/") => k,
                _ => continue,
            };
            let (fx, fy) = cv_face(&cv, &id);
            out.push((id, key, fx, fy));
        }
    }
    if DBG {
        let sample: Vec<_> = out.iter().take(4).map(|e| (e.0.clone(), e.1.clone(), e.2, e.3)).collect();
        dbg_write(&format!("rt_scan: ws={} found={} champs, sample={:?}\n", ws, out.len(), sample));
    }
    *RT_CHAMP.lock().unwrap_or_else(|e| e.into_inner()) = out;
}

// ── 런타임 프레임 UV (fanim 없는 raw .aseprite 모드챔프용) ──────────────────
//   게임 Assets get 으로 <key>#anim 맵(hashbrown) 얻어 "idle"(없으면 첫 태그) frame0 rect 읽고,
//   전체 프레임 bbox=시트dims 로 정규화. ghidra-re 확정 경로. cfg 킬스위치 + IsBadReadPtr 가드 + 캐시.
// 0.5.3(2026-07-29 실측): ~~0.5.1 상대유도 0x40e250~~ = **0.5.2 시점에 이미 죽은 값**이었다
//   (0.5.2 exe 에서 0x40e250 은 함수 시작이 아니라 46KB 거대함수 0x4041a0 내부·콜러 0).
//   정답 도출 = 앵커맵 콜러-대응 양방향 투표: 0.5.2 애님게터 0x5ab7d0 → 0.5.3 0x888fd0
//   (정방향 43표/45·역방향 순도 100%(43/43)·콜러 77→73). 같은 절차로 알려진 정답 LOADER
//   0x5ac950→0x2e1550 이 193/194 로 재현돼 방법이 검증됨. banpick_illust RVA_ANIM_GET 과 동일 함수.
// 0.5.4(2026-08-05 재핀): ~~0.5.3 0x888fd0~~ → **0x74c010**. 근거 ①clone family(33원) 콜러수 지문
//   73→73 + 블록내 index 보존 ②'.../champions/priest#anim' 등 #anim 전체리터럴 lea→직후 call 2/2 수렴
//   (같은 절차가 0.5.3 에서 0x888fd0 을 재현 = 방법 검증됨).
// 0.5.5(2026-08-12 재핀): ~~0.5.4 0x74c010~~ → **0x844160**. 근거 ①'.../champions/priest#anim'
//   전체리터럴 lea→직후 call 1표 수렴(0.5.4 재현 OK) ②CARD_DRAW 콜슬롯13 정렬 이중확증.
// 0.5.7(2026-08-26): ~~0.5.6 0xbea4a0~~ -> **0x74bf10**. CARD_DRAW(0x1e6f350) 콜슬롯14 재확정(구·신 콜 27:27 완전대응).
const ANIM_GET_RVA: usize = 0x7507e0; // 0.5.6(구0.5.5=0x844160). CARD_DRAW 콜슬롯14로 확정(clone family라 xref/콜슬롯만 유효).// FUN(Assets, key_ptr, key_len) -> #anim map ptr
const RT_UV_ENABLED: bool = true;        // 크래시 시 false 로 끔
static ASSETS_PTR: AtomicUsize = AtomicUsize::new(0);
// id → Option<(u0,v0,너비uv,높이uv, 프레임픽셀너비, 프레임픽셀높이)>. 얼굴=정사각크롭 재계산, 전신=종횡비 세팅.
static RT_UV: Mutex<Vec<(String, Option<(f32, f32, f32, f32, f32, f32)>)>> = Mutex::new(Vec::new());

#[inline]
fn rok(p: usize, n: usize) -> bool {
    p >= 0x10000 && (p as u64) < 0x0000_8000_0000_0000 && unsafe { IsBadReadPtr(p, n) } == 0
}
#[inline]
unsafe fn ru64(p: usize) -> Option<u64> { if rok(p, 8) { Some(core::ptr::read_unaligned(p as *const u64)) } else { None } }
#[inline]
unsafe fn ru8(p: usize) -> Option<u8> { if rok(p, 1) { Some(core::ptr::read_unaligned(p as *const u8)) } else { None } }
#[inline]
unsafe fn rf(p: usize) -> Option<f32> { if rok(p, 4) { Some(core::ptr::read_unaligned(p as *const f32)) } else { None } }

unsafe fn rt_resolve_uv(assets: usize, key: &str) -> Option<(f32, f32, f32, f32, f32, f32)> {
    if assets < 0x10000 { return None; }
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 { return None; }
    let mut animkey = String::with_capacity(key.len() + 5);
    animkey.push_str(key);
    animkey.push_str("#anim");
    let get: extern "C" fn(usize, *const u8, usize) -> usize = core::mem::transmute(base + ANIM_GET_RVA);
    let map = get(assets, animkey.as_ptr(), animkey.len());
    if !rok(map, 0x20) { return None; }
    let ctrl = ru64(map)? as usize;         // map[0] = ctrl base
    let mask = ru64(map + 8)? as usize;     // map[1] = bucket_mask
    if ru64(map + 0x18).unwrap_or(0) == 0 { return None; } // 빈 맵
    if mask == 0 || mask > 0xffff || !rok(ctrl, mask + 1) { return None; }
    let mut idle: Option<(f32, f32, f32, f32)> = None;
    let mut first: Option<(f32, f32, f32, f32)> = None;
    let (mut tw, mut th) = (0.0f32, 0.0f32);
    for i in 0..=mask {
        if (ru8(ctrl + i)? & 0x80) != 0 { continue; }      // 빈 버킷
        let e = ctrl.wrapping_sub(i * 0x30);
        let klen = ru64(e.wrapping_sub(0x20)).unwrap_or(0) as usize;
        let kptr = ru64(e.wrapping_sub(0x28)).unwrap_or(0) as usize;
        let fptr = ru64(e.wrapping_sub(0x10)).unwrap_or(0) as usize;
        let flen = ru64(e.wrapping_sub(0x08)).unwrap_or(0) as usize;
        if fptr < 0x10000 || flen == 0 || flen > 4096 { continue; }
        for fi in 0..flen {
            let fp = fptr + fi * 0x14;
            let (x, y, w, h) = match (rf(fp), rf(fp + 4), rf(fp + 8), rf(fp + 0xc)) {
                (Some(a), Some(b), Some(c), Some(d)) => (a, b, c, d),
                _ => break,
            };
            if x + w > tw { tw = x + w; }
            if y + h > th { th = y + h; }
            if fi == 0 {
                let r = (x, y, w, h);
                if first.is_none() { first = Some(r); }
                if klen == 4 && rok(kptr, 4) {
                    if core::slice::from_raw_parts(kptr as *const u8, 4) == b"idle" { idle = Some(r); }
                }
            }
        }
    }
    let (x, y, w, h) = idle.or(first)?;
    if tw <= 0.0 || th <= 0.0 || w <= 0.0 || h <= 0.0 { return None; }
    Some((x / tw, y / th, w / tw, h / th, w, h))
}

// 캐시된 런타임 프레임 UV(챔프당 1회 resolve). 정적 표 없는 챔프(raw .aseprite)의 폴백.
fn rt_frame(id: &str) -> Option<(f32, f32, f32, f32, f32, f32)> {
    if !RT_UV_ENABLED { return None; }
    {
        let c = RT_UV.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(e) = c.iter().find(|e| e.0 == id) { return e.1; }
    }
    let assets = ASSETS_PTR.load(Ordering::Relaxed);
    let uv = champ_key(id).and_then(|key| unsafe { rt_resolve_uv(assets, &key) });
    if DBG {
        dbg_write(&format!("rt_frame: id={} assets={:#x} key={:?} -> uv={:?}\n",
            id, assets, champ_key(id), uv));
    }
    RT_UV.lock().unwrap_or_else(|e| e.into_inner()).push((id.to_string(), uv));
    uv
}
// raw .aseprite 챔프 face 오프셋(champion_view 기준). 정적 표 → 런타임 스캔 → (0,0).
fn champ_rt_face(id: &str) -> (f32, f32) {
    if let Some(e) = CHAMP_RT_FACE.iter().find(|e| e.0 == id) { return (e.1, e.2); }
    RT_CHAMP.lock().unwrap_or_else(|e| e.into_inner()).iter().find(|e| e.0 == id).map(|e| (e.2, e.3)).unwrap_or((0.0, 0.0))
}
// 얼굴 아이콘용: 런타임 프레임 rect + face 오프셋으로 17px 얼굴 크롭(base/Touhou 와 동일 레시피).
//   gen_champ_uv.py: u0px=x+fx/2+(w-17)/2, v0px=y+fy/2+(h-17)/2, 17px 정사각 → /시트dims.
fn rt_face_crop(id: &str) -> Option<(f32, f32, f32, f32)> {
    let (u0, v0, uw, vh, fw, fh) = rt_frame(id)?;
    if fw <= 0.0 || fh <= 0.0 { return None; }
    const CROP: f32 = 17.0;
    let (fx, fy) = champ_rt_face(id);
    let upp = uw / fw;   // uv per pixel x (= 1/시트너비)
    let vpp = vh / fh;   // uv per pixel y (= 1/시트높이)
    let ox = fx / 2.0 + if fw > CROP { (fw - CROP) * 0.5 } else { 0.0 };
    let oy = fy / 2.0 + if fh > CROP { (fh - CROP) * 0.5 } else { 0.0 };
    Some((u0 + ox * upp, v0 + oy * vpp, fw.min(CROP) * upp, fh.min(CROP) * vpp))
}
// 전신 이미지용: 프레임 전체 UV + 픽셀 종횡비. (u0,v0,너비uv,높이uv, fw, fh)
fn rt_body(id: &str) -> Option<(f32, f32, f32, f32, f32, f32)> { rt_frame(id) }
// ImageRunner 인스턴스 데이터 ptr.
unsafe fn img_runner_dp(n: &Node) -> usize {
    if !n.runner.type_name().contains("ImageRunner") { return 0; }
    let any: &dyn core::any::Any = n.runner.as_any();
    let parts: [usize; 2] = core::mem::transmute::<*const dyn core::any::Any, [usize; 2]>(any as *const dyn core::any::Any);
    let dp = parts[0];
    if dp < 0x10000 { 0 } else { dp }
}
// ImageRunner source = owning Rust String, 슬롯0 레이아웃(ghidra-re 확정): {cap@+0x00, ptr@+0x08, len@+0x10}.
//   소멸자/writer는 `cap != 0` 이면 __rust_dealloc(ptr, cap, 1) 로 free.
//   ★ptr을 우리 정적 버퍼로 바꾸므로 반드시 cap=0 (안 그러면 teardown 때 정적버퍼 free = 힙손상/AV 크래시).
//   렌더는 ptr(+0x08)·len(+0x10)만 읽어 표시 → cap=0 이어도 표시 정상. (원본 placeholder String은 free 없이 덮어써 소량 누수, 무해)
//   gate -1 = plain 전체 텍스처(#sheet). 크롭은 +0xa4=1 + +0xa8 UV로. (슬롯1~3(+0xd0/+0x1a0/+0x270)은 placeholder 유지 — 정상 free)
unsafe fn set_img_source(n: &mut Node, ptr: usize, len: usize, frame: i64) -> bool {
    let dp = img_runner_dp(n);
    if dp == 0 { return false; }
    core::ptr::write_unaligned((dp + 0x00) as *mut u64, 0u64);       // cap=0 → drop이 정적버퍼 free 안 함(크래시 방지)
    core::ptr::write_unaligned((dp + 0x08) as *mut u64, ptr as u64); // ptr → 정적 버퍼
    core::ptr::write_unaligned((dp + 0x10) as *mut u64, len as u64); // len (표시용)
    core::ptr::write_unaligned((dp + 0x18) as *mut i64, frame);      // frame gate
    true
}
// 노드 레이아웃 크기 세팅(width@+0x74, height@+0x7c, 복사본 6곳) — 프레임 종횡비 맞춤용.
unsafe fn set_node_size(node: &Node, w: f32, h: f32) {
    let na = node as *const Node as usize;
    for off in [0x74usize, 0xf4, 0x174, 0x1f4, 0x248, 0x258] { ui_kit::runner_wr_f32(na, off, w); }
    for off in [0x7cusize, 0xfc, 0x17c, 0x1fc, 0x24c, 0x25c] { ui_kit::runner_wr_f32(na, off, h); }
}
// 커스텀 UV 크롭 세팅: +0xa4(flag)=1, +0xa8..0xb4 = (u0,v0,u1,v1).
unsafe fn set_img_uv(n: &mut Node, u0: f32, v0: f32, u1: f32, v1: f32) {
    let dp = img_runner_dp(n);
    if dp == 0 { return; }
    *((dp + 0xa4) as *mut u8) = 1;
    *((dp + 0xa8) as *mut f32) = u0;
    *((dp + 0xac) as *mut f32) = v0;
    *((dp + 0xb0) as *mut f32) = u1;
    *((dp + 0xb4) as *mut f32) = v1;
}
static IMG_DUMP: AtomicBool = AtomicBool::new(false);
unsafe fn set_champ_image(node: &mut Node, champ_id: &str) {
    // 게임 에셋 키 = CHAMP_KEY[id] + "#sheet" (base=asset/base/…, 모드챔프=asset/<모드ns>/…).
    let key = match champ_key(champ_id) { Some(k) => k, None => return };
    let klen = key.len();
    let total = klen + IMG_SUFFIX.len();
    if total > 160 { return; }
    let buf = core::ptr::addr_of_mut!(IMG_BUF) as *mut u8;
    core::ptr::copy_nonoverlapping(key.as_ptr(), buf, klen);
    core::ptr::copy_nonoverlapping(IMG_SUFFIX.as_ptr(), buf.add(klen), IMG_SUFFIX.len());
    // gate -1(plain)+커스텀 UV 크롭. 정적 전신 UV → 없으면 런타임 #anim frame0 UV(raw .aseprite).
    let cuv = champ_uv(champ_id);
    // (u0,v0,너비uv,높이uv, Option<(fw,fh)픽셀종횡비>)
    let uv4: Option<(f32, f32, f32, f32, Option<(f32, f32)>)> = match cuv {
        Some((u0, v0, u1, v1, fw, fh)) => Some((u0, v0, u1 - u0, v1 - v0, Some((fw, fh)))),
        None => rt_body(champ_id).map(|(u0, v0, uw, vh, fw, fh)| (u0, v0, uw, vh, Some((fw, fh)))),
    };
    let ok = set_img_source(node, buf as usize, total, if uv4.is_some() { -1 } else { 0 });
    if let Some((u0, v0, uw, vh, aspect)) = uv4 {
        // UV 렉트 형식 = (u0, v0, 너비, 높이).
        set_img_uv(node, u0, v0, uw, vh);
        // ★프레임 픽셀 종횡비(fw:fh)를 박스에 맞춰 세팅(정적 UV만; 런타임은 기본 박스 유지).
        if let Some((fw, fh)) = aspect {
            if fw > 0.0 && fh > 0.0 {
                let (box_w, box_h) = (85.0_f32, 88.0_f32);
                let ar = fw / fh;
                let (mut w, mut h) = (box_h * ar, box_h); // 세로 맞춤
                if w > box_w { w = box_w; h = box_w / ar; } // 가로 초과 시 가로 맞춤
                set_node_size(node, w, h);
            }
        }
    } else {
        // UV 없음 → 스테일 커스텀 UV 끔.
        let dp = img_runner_dp(node);
        if dp != 0 { *((dp + 0xa4) as *mut u8) = 0; }
    }
    if DBG && !IMG_DUMP.swap(true, Ordering::Relaxed) {
        let tn = node.runner.type_name().to_string();
        let dp = img_runner_dp(node);
        let mut s = format!("champ_img: type={} dp={:#x} set_ok={}\n", tn, dp, ok);
        s.push_str(&format!("  key='{}' total_len={}\n", core::str::from_utf8_unchecked(core::slice::from_raw_parts(buf, total)), total));
        if dp > 0x10000 {
            for off in [0usize, 8, 0x10, 0x18, 0xa4, 0xa8] {
                let u = core::ptr::read_unaligned((dp + off) as *const u64);
                s.push_str(&format!("  dp+{:#x} = {:#x} ({})\n", off, u, u as i64));
            }
        }
        dbg_write(&s);
    }
}
// 행별 얼굴 아이콘: 전신 시트(champions/<id>#sheet) + 게임 정식 face 크롭 UV(CHAMP_FACE, gen_champ_uv.py).
//   레시피 = idle 프레임0에서 17px 정사각을 프레임중앙 + face.{x,y}/2 오프셋(머리로 당김). ghidra-re 확정.
// 반환 = 소스 세팅 성공(=CHAMP_KEY에 있는 챔프). 실패(테이블 없음)면 호출측이 얼굴 숨김.
unsafe fn set_face_image_buf(node: &mut Node, buf: *mut u8, champ_id: &str) -> bool {
    let key = match champ_key(champ_id) { Some(k) => k, None => return false };
    let klen = key.len();
    let total = klen + IMG_SUFFIX.len();
    if total > 96 { return false; }
    core::ptr::copy_nonoverlapping(key.as_ptr(), buf, klen);
    core::ptr::copy_nonoverlapping(IMG_SUFFIX.as_ptr(), buf.add(klen), IMG_SUFFIX.len());
    // 정적 얼굴 UV(base/Touhou) → 없으면 런타임 #anim frame0 정사각 크롭(raw .aseprite 모드챔프, 예: Leef).
    let face = champ_face_uv(champ_id).or_else(|| rt_face_crop(champ_id));
    // UV 있으면 -1(plain)+커스텀크롭, 없으면 0=게임 네이티브 프레임0.
    set_img_source(node, buf as usize, total, if face.is_some() { -1 } else { 0 });
    if let Some((u0, v0, uw, vh)) = face {
        set_img_uv(node, u0, v0, uw, vh);
    } else {
        // 커스텀 UV 비활성(이전 챔프의 스테일 크롭 제거) → 네이티브 프레임/전체.
        let dp = img_runner_dp(node);
        if dp != 0 { *((dp + 0xa4) as *mut u8) = 0; }
    }
    true
}
unsafe fn set_face_image(node: &mut Node, row_i: usize, champ_id: &str) -> bool {
    if row_i >= MAX_ROWS { return false; }
    set_face_image_buf(node, core::ptr::addr_of_mut!(FACE_BUF[row_i]) as *mut u8, champ_id)
}

// 0.4.14 핫픽스 RVA (scrim/item_tactics 와 동일값 — 패치 시 함께 마이그).
// 0.5.3(2026-07-29 실측): ~~0.5.1 0x40f3d0~~ = **0.5.2 시점에 이미 죽은 값**(함수 시작 아님·콜러 0)
//   ⟹ 0.5.2 에서 이 훅은 애초에 안 걸려 있었다(= "밴픽 asset-get copy 재확인" 잔여의 실체).
//   정답 = 문자열-xref 확정: 'asset/base/ui/layout/{main,strategy,training,banpick}' lea 직후 call 이
//   0x2e1550 로 만장일치(main×17·banpick×19). 0.5.2 에 같은 절차 → 알려진 정답 0x5ac950 재현.
// 0.5.4(2026-08-05 재핀): ~~0.5.3 0x2e1550~~ → **0x2e35d0**. 근거 ①'asset/base/ui/layout/{main,banpick,
//   strategy,training}' lea→직후 call 이 17/19/2/12 로 0.5.3 과 **완전히 같은 표수**로 수렴
//   (특히 주입 대상 'asset/base/ui/layout/banpick/layout' 단일 리터럴 → 1/1) ②콜러수 511→511.
// 0.5.5(2026-08-12): ~~0.5.4 0x2e35d0~~ → **0x2e42d0**. 'asset/base/ui/layout/{main,banpick}' lea→직후
//   call 이 17/20 표로 수렴(0.5.4 재현 OK). clone family라 문자열-xref 만이 유효(프롤로그·바이트 불가).
// 0.5.6(2026-08-20): ~~0.5.5 0x2e42d0~~ → **0x2e6f60**. layout/main lea→call 17/17 수렴(string-xref).
// 0.5.7(2026-08-26): ~~0.5.6 0x2e6f60~~ -> **0x2ea930**. string-xref layout/main 17/17(0.5.6과 동일 카운트).
const LOADER_RVA: usize = 0x2ea830; // 에셋 게터 FUN(am, path, len) -> NodeTemplate*
// ~~★밴픽 화면 전용 asset-get copy #2 (0.5.1 모노모픽 분화)~~
// ⛔0.5.3(2026-07-29): **copy #2 는 없다 — 0 = 미사용**. 실측으로 밴픽 레이아웃 문자열
//   "asset/base/ui/layout/banpick..." 의 lea 직후 call 이 copy #1 과 **같은 0x2e1550 으로 ×19 수렴**
//   (0.5.2 에서도 0x5ac950 ×19 로 동일 구조였다 = 0.5.1 의 분화는 이미 되돌려진 상태).
// ★★같은 주소를 install_one 으로 두 번 걸면 **자기 자신을 체인**해 detour→tramp→detour 무한재귀
//   (= L642 주석의 먹통 사고와 동일 기전) ⟹ copy #2 설치는 영구 제거. 값 0 은 "미사용" 표식.
const BANPICK_LOADER_RVA: usize = 0;
// 0.5.3(2026-07-29 실측): ~~0.5.1 0x24b4590~~ → **0x1a6530**. 구값은 0.5.2 시점에 이미 stale
//   (0.5.2 정답은 0x24b5a00 이었고 이 파일만 갱신 누락). 3인자 계약·노드 stride 0x90 유지.
//   = comptest_unlock / ai_adjust 세션 실측 확정값과 동일 함수.
// 0.5.4(2026-08-05): ~~0.5.3 0x1a6530~~ → **0x1a3ce0**. 마스크 전체본문(2192B, 고정 1789B) 유일매치
//   + 콜러수 5→5. 본문이 바이트동일 ⟹ 3인자 계약·노드 stride 0x90 그대로.
// 0.5.5(2026-08-12): ~~0.5.4 0x1a3ce0~~ → **0x1a3e70**. skel 완전일치·전역 유일(2192B) + 프롤로그 동일.
// 0.5.6(2026-08-20): ~~0.5.5 0x1a3e70~~ → **0x19ab40**. skel UNIQUE·BYTE=SAME·2192B·프롤로그 동일.
// 0.5.7(2026-08-26): ~~0.5.6 0x19ab40~~ -> **0x1ab310**. skel UNIQUE·BYTE=SAME·2192B·프롤로그 20B 동일.
const PARSER_RVA: usize = 0x1ab140; // .ui 텍스트 → NodeTemplate
// 0.5.3(2026-07-29): 범용 __rust_alloc(size, align) 이 **align별 심 + impl 로 분해**됐다.
//   전 모드 정본 = impl 직접호출 0x28f7df0 · **3인자** (rcx=무시, rdx=flags(0), r8=size) -> rax,
//   실패 시 0 반환. ⛔align8 심(0xbb2bd0)은 OOM 시 abort 라 미채택. ⚠2인자로 두면 랜덤 크래시.
// 0.5.4(2026-08-05): ~~0.5.3 0x28f7df0~~ → **0x29bb920**. 마스크 전체본문(60B) 유일매치 + 콜러수
//   32890→33708(전역 alloc impl). 본문 바이트동일 ⟹ 3인자 계약(rcx무시/rdx=flags/r8=size) 유지.
// 0.5.5(2026-08-12): ~~0.5.4 0x29bb920~~ → **0x2a9bf30**. skel/마스크 전체본문(60B) 유일 + 프롤로그 동일.
// 0.5.6(2026-08-20): ~~0.5.5 0x2a9bf30~~ → **0x2ab1670**. skel/마스크 UNIQUE·BYTE=SAME·60B·프롤로그 동일.
// 0.5.7(2026-08-26): ~~0.5.6 0x2ab1670~~ -> **0x2ab4010**. skel UNIQUE·BYTE=SAME·60B.
const ALLOC_RVA: usize = 0x2b1b410;  // 게임 alloc impl(_, flags, size) -> ptr
const NT_SIZE: usize = 0x90;
// 밴픽 화면 팝업 조각(컴파일타임 임베드). 접미사 매칭으로 banpick_view_plus 리매핑도 대응.
const POPUP_UI: &str = include_str!("../ui_inject/draft_popup.ui");
const SUFFIX: &[u8] = b"banpick/layout";

type LoaderFn = extern "win64" fn(usize, *const u8, usize) -> usize;
type ParserFn = extern "win64" fn(*mut u8, *const u8, usize);
type AllocFn = extern "win64" fn(usize, usize, usize) -> usize; // 0.5.3: (무시, flags, size) -> ptr

static BASE: AtomicUsize = AtomicUsize::new(0);
static TRAMP: AtomicUsize = AtomicUsize::new(0);
static TRAMP2: AtomicUsize = AtomicUsize::new(0); // 0xeb17d0(밴픽 copy #2) 세컨드 훅 트램폴린
static INSTALLED: AtomicBool = AtomicBool::new(false); // 최초 설치 완료 표시(로그 1회용)
static LAST_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
// 탭/역할 상태 + 파싱된 섹션 캐시 + 렌더 변경감지.
static SEL_TAB: AtomicUsize = AtomicUsize::new(0);
static SEL_ROLE: AtomicUsize = AtomicUsize::new(0); // 0=전체 1=탑 2=정글 3=미드 4=바텀
static SEL_SCOPE: AtomicUsize = AtomicUsize::new(0); // 데이터 범위 0=전체(overall) 1=대회(tournament) 2=솔랭(solo)
const SCOPE_IDS: [&str; 3] = ["overall", "tournament", "solo"];
static SEL_DSLOT: AtomicUsize = AtomicUsize::new(0); // 밴픽코치 활성 슬롯(0=상대밴 1=내밴 2=상대픽 3=내픽) — 그리드 클릭이 여기로 토글
const DSLOT_KEYS: [&str; 4] = ["enemyBans", "myBans", "enemyPicks", "myPicks"];
static SEL_CHAMPSUB: AtomicUsize = AtomicUsize::new(0); // 챔피언정보 서브탭 0=통계 1=기본정보 2=상대·빌드 3=패치
static LAST_CHAMP_ID: Mutex<String> = Mutex::new(String::new()); // 마지막 선택 챔프 id(상태 유지용)
static ROW_CHAMP: Mutex<Vec<String>> = Mutex::new(Vec::new()); // 현재 표시된 각 행의 챔프 id("" = 비-챔프 행)
static GRID_CHAMP: Mutex<Vec<String>> = Mutex::new(Vec::new()); // 밴픽코치 그리드 셀별 챔프 id
static CLICK_LAST: AtomicUsize = AtomicUsize::new(usize::MAX);
// 행 표시 데이터에서 클릭대상 챔프 id 를 숨겨싣는 구분자(US, 0x1f). "표시텍스트\x1f챔프id".
const CHAMP_SEP: char = '\u{1f}';
static LAST_RENDER: AtomicU64 = AtomicU64::new(u64::MAX);
static LAST_NAV: AtomicU64 = AtomicU64::new(u64::MAX); // 탭/서브탭/역할/챔프 조합(변하면 스크롤 리셋)
static SECTIONS: Mutex<Vec<(String, Vec<String>)>> = Mutex::new(Vec::new());
static IMG_HAS_CHAMP: AtomicBool = AtomicBool::new(false); // 챔피언정보 탭에 표시할 챔프 있음(스크롤 표시 판정용)
static SHOW_OVERLAY: AtomicBool = AtomicBool::new(true); // 팝업 표시 여부(X버튼=off, 좌하단 토글=flip)

// 컨트롤 채널(게임 → TFM2.gg): 클릭 의도를 overlay_control.txt 로 write → 대시보드가 감지·반영·재export.
//   ★_seq nonce: main.cjs 가 lastControlContent 로 중복내용 무시 → 같은 명령(예 draft_toggle=X 두 번)이
//   dedup 되어 씹히는 것 방지. 대시보드는 _seq= 를 파싱만 하고 핸들러 없어 무시.
static CTRL_SEQ: AtomicUsize = AtomicUsize::new(1);
fn write_control(s: &str) {
    let mut buf = [0u16; 520];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n == 0 || n >= buf.len() { return; }
    let exe = String::from_utf16_lossy(&buf[..n]);
    if let Some(i) = exe.rfind(|c| c == '\\' || c == '/') {
        let seq = CTRL_SEQ.fetch_add(1, Ordering::Relaxed);
        let p = format!(r"{}\mods\tfm2_draft_overlay\overlay_control.txt", &exe[..i]);
        let _ = std::fs::write(&p, format!("{}\n_seq={}\n", s, seq));
    }
}
// 전체 선택상태(역할 + 마지막 챔프 id)를 한 번에 write → 대시보드 3분 갱신 후에도 상태 유지.
fn write_control_state() {
    let role = SEL_ROLE.load(Ordering::Relaxed);
    let scope = SCOPE_IDS[SEL_SCOPE.load(Ordering::Relaxed).min(2)];
    let champ = LAST_CHAMP_ID.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let mut s = format!("role={}\nscope={}", role, scope);
    if !champ.is_empty() {
        s.push_str(&format!("\nchamp={}", champ));
    }
    write_control(&s);
}

// overlay_data.txt 파싱: "#TAB <이름>" 마커로 섹션 분할, 그 아래 줄들이 그 탭의 표시행.
fn parse_sections(txt: &str) -> Vec<(String, Vec<String>)> {
    let mut out: Vec<(String, Vec<String>)> = Vec::new();
    for line in txt.lines() {
        if let Some(name) = line.strip_prefix("#TAB ") {
            out.push((name.trim().to_string(), Vec::new()));
        } else if let Some(last) = out.last_mut() {
            last.1.push(line.to_string()); // 빈 줄도 유지 → 단락 간격(spaceOut)
        }
    }
    out
}

#[link(name = "kernel32")]
extern "system" {
    fn VirtualAlloc(addr: usize, size: usize, typ: u32, protect: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, new: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(proc: usize, addr: usize, size: usize) -> i32;
    fn GetCurrentProcess() -> usize;
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleFileNameW(module: usize, buf: *mut u16, size: u32) -> u32;
    fn IsBadReadPtr(p: usize, n: usize) -> i32;
}
const MEM_CR: u32 = 0x1000 | 0x2000;
const RWX: u32 = 0x40;

// 진단 로그(배포시 false). 밴픽 팝업 주입/파싱 결과를 파일로 기록.
const DBG: bool = false;
static DUMP_DONE: AtomicBool = AtomicBool::new(false);
static DUMP_FRAME: AtomicUsize = AtomicUsize::new(0);
// 팝업 트리 구조 덤프(id/러너종류/visible/자식수) — 스크롤 진단용.
// ★히트테스트 진단(읽기 전용, write 없음): build_hit_tester 의 게이트 값들을 덤프.
//   게이트 = visible(node+0x260)==1 && node+0x261==0 && 러너 block_event() && rect(w,h)!=0.
//   차단막이 후보 등록에 실패하는 원인을 정상 동작하는 row0 와 비교해 판별한다.
//   Node 레이아웃(ghidra 확정): rect x,y,w,h f32 @+0x240 / visible u8 @+0x260 / unknown u8 @+0x261.
fn dbg_hit_fields(root: &Node, id: &str, out: &mut String) {
    match ui_kit::find(root, id) {
        Some(n) => {
            let na = n as *const Node as usize;
            unsafe {
                let rd = |off: usize| -> f32 { core::ptr::read_unaligned((na + off) as *const f32) };
                let rb = |off: usize| -> u8 { core::ptr::read_unaligned((na + off) as *const u8) };
                out.push_str(&format!(
                    "  {:<14} rect=(x{:.0} y{:.0} w{:.0} h{:.0})  visible@260={}  unk@261={}\n",
                    id, rd(0x240), rd(0x244), rd(0x248), rd(0x24c), rb(0x260), rb(0x261)
                ));
            }
        }
        None => out.push_str(&format!("  {:<14} NOT FOUND\n", id)),
    }
}

// 팝업이 떠 있는 동안 게임 밴픽 호버 툴팁을 강제로 숨긴다.
//   ★호버는 클릭과 다른 경로(MatchUIRunner::update)로 처리되고 occlusion(가림) 판정이 없어,
//     클릭 차단막으로는 못 막는다(ghidra 확정 0x247ae70/MatchUIRunner::update). 게임이 매 프레임
//     tooltip.visible = hover_bool 로 덮으므로, post_update(=게임 update 이후)에 false 로 다시 덮으면
//     그 프레임엔 항상 안 보인다. write 는 Node.visible(=false) 뿐 → 인접필드 위험 0.
//   부수효과: build_hit_tester 첫 게이트가 visible!=1 이면 서브트리 통째 제외 → 툴팁이 우리 클릭을
//     뺏던 문제(레이어 재배치)도 동시 해소.
//   ⚠ 이 id 들은 pick_slot 마다 중복 존재 → find(첫 매치) 로는 부족. 서브트리 전체를 walk 해 전부 끈다.
//     (우리 조각엔 이 id 가 없으니 자기 오염 없음.)
const HOVER_TOOLTIP_IDS: &[&str] = &[
    "player_tooltip",
    "champion_position_tooltip",
    "champion_tooltip",
    "fearless_tooltip",
];
fn hide_hover_tooltips(n: &mut Node) {
    if HOVER_TOOLTIP_IDS.iter().any(|t| n.id == *t) {
        if n.visible { n.visible = false; }
    }
    for c in n.child.iter_mut() { hide_hover_tooltips(c); }
}

// ★우리 조각(draft_root)을 부모 children Vec 의 맨 끝으로 이동(매 프레임 post_update).
//   근거(ghidra 확정): 히트/렌더는 children 을 DFS 순회하고 승자는 '가장 나중' 노드.
//     게임은 매 프레임 move_child_after(순서 memmove)로 자기 노드를 뒤로 보내 히트 우선순위를 뺏는다.
//     ⟹ 우리도 매 프레임 진짜 최후미로 가면 DFS 최후 승자는 항상 우리.
//   "런타임 트리 push 금지" 규칙은 *새 노드 추가*(길이 증가) 얘기 — **순서 재배열(길이 불변)은
//     게임 자신이 매 프레임 하는 정상 동작이라 반영된다.** remove+push = memmove, 재할당 없음, Node
//     통째 이동(DraggablePopup 상태 보존). 이미 최후미면 no-op.
//   반환: (found, moved) — 진단용.
fn raise_to_top(parent: &mut Node) -> (bool, bool) {
    if let Some(pos) = parent.child.iter().position(|n| n.id == "draft_root") {
        if pos + 1 != parent.child.len() {
            let node = parent.child.remove(pos);
            parent.child.push(node);
            return (true, true);
        }
        return (true, false);
    }
    // 자식에 없으면 하위로 재귀(draft_root 가 u.root 바로 밑이 아닐 수 있음).
    for c in parent.child.iter_mut() {
        let r = raise_to_top(c);
        if r.0 { return r; }
    }
    (false, false)
}

fn dump_tree(n: &Node, depth: usize, out: &mut String) {
    out.push_str(&format!("{}{} <{}> vis={} nch={}\n", "  ".repeat(depth), n.id, ui_kit::kind(n), n.visible, n.child.len()));
    if depth < 7 {
        for c in n.child.iter() { dump_tree(c, depth + 1, out); }
    }
}
fn dbg_write(s: &str) {
    if !DBG { return; }
    let mut buf = [0u16; 520];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n == 0 || n >= buf.len() { return; }
    let exe = String::from_utf16_lossy(&buf[..n]);
    if let Some(i) = exe.rfind(|c| c == '\\' || c == '/') {
        let p = format!(r"{}\mods\tfm2_draft_overlay\overlay_debug.txt", &exe[..i]);
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&p) {
            let _ = writeln!(f, "{}", s);
        }
    }
}

// ── 체이닝 로더 훅 ──────────────────────────────────────────────────────────
extern "win64" fn detour(am: usize, path: *const u8, len: usize) -> usize {
    let ta = TRAMP.load(Ordering::Relaxed);
    if ta == 0 { return 0; }
    let tramp: LoaderFn = unsafe { core::mem::transmute(ta) };
    let r = tramp(am, path, len); // 체인(또는 원본) 호출 → 템플릿 ptr
    if !path.is_null() && r > 0x10000 && len >= SUFFIX.len() && len < 200 {
        let s = unsafe { core::slice::from_raw_parts(path, len) };
        if s.ends_with(SUFFIX) {
            // 패닉이 게임 콜스택(extern win64)으로 unwind = UB 차단. 멱등이라 중복 안전.
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { append_root(r) }));
        }
    }
    r
}

// copy #2(0xeb17d0, 밴픽 전용) 훅 — 본문은 detour 와 동일, 트램폴린만 TRAMP2.
extern "win64" fn detour2(am: usize, path: *const u8, len: usize) -> usize {
    let ta = TRAMP2.load(Ordering::Relaxed);
    if ta == 0 { return 0; }
    let tramp: LoaderFn = unsafe { core::mem::transmute(ta) };
    let r = tramp(am, path, len); // 체인(item_tactics 스텁 경유 가능) → 원본
    if !path.is_null() && r > 0x10000 && len >= SUFFIX.len() && len < 200 {
        let s = unsafe { core::slice::from_raw_parts(path, len) };
        if s.ends_with(SUFFIX) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { append_root(r) }));
        }
    }
    r
}

// 밴픽 루트 템플릿 r 의 child 배열 끝에 내 팝업 조각 append (멱등: 이미 있으면 skip).
unsafe fn append_root(r: usize) -> bool {
    if r <= 0x10000 { return false; }
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 { return false; }
    if find_tmpl(r, b"draft_overlay", 0) != 0 { return true; } // 이미 주입됨
    let parser: ParserFn = core::mem::transmute(base + PARSER_RVA);
    let mut out = [0u8; 0x400];
    parser(out.as_mut_ptr(), POPUP_UI.as_ptr(), POPUP_UI.len());
    let my = out.as_ptr().add(0x10) as usize; // 내 root NodeTemplate
    if *(my as *const usize) == usize::MAX {
        dbg_write("draft_popup.ui PARSE ERROR — .ui 문법 확인 필요");
        return false;
    }
    let ptr = *((r + 0x50) as *const usize);
    let len = *((r + 0x58) as *const usize);
    if len > 2000 { return false; }
    let galloc: AllocFn = core::mem::transmute(base + ALLOC_RVA);
    let new_n = len + 1;
    let np = galloc(0, 0, new_n * NT_SIZE); // 0.5.3 impl 계약: rcx 무시·rdx=flags·r8=size
    if np == 0 { return false; }
    if ptr != 0 && len != 0 { core::ptr::copy_nonoverlapping(ptr as *const u8, np as *mut u8, len * NT_SIZE); }
    core::ptr::copy_nonoverlapping(my as *const u8, (np + len * NT_SIZE) as *mut u8, NT_SIZE);
    // ptr→cap→len 순(중간 read 안전). 옛 배열 leak=무해(startup 경합 UAF 회피).
    *((r + 0x50) as *mut usize) = np;
    *((r + 0x48) as *mut usize) = new_n;
    *((r + 0x58) as *mut usize) = new_n;
    dbg_write("draft_popup.ui 주입 OK (탭 팝업)");
    true
}

unsafe fn find_tmpl(node: usize, target: &[u8], depth: usize) -> usize {
    if node <= 0x10000 || depth > 12 { return 0; }
    let idptr = *((node + 0x08) as *const usize);
    let idlen = *((node + 0x10) as *const usize);
    if idlen == target.len() && idptr > 0x10000
        && core::slice::from_raw_parts(idptr as *const u8, idlen) == target { return node; }
    let cptr = *((node + 0x50) as *const usize);
    let clen = *((node + 0x58) as *const usize);
    if cptr > 0x10000 && clen < 1000 {
        for i in 0..clen {
            let f = find_tmpl(cptr + i * NT_SIZE, target, depth + 1);
            if f != 0 { return f; }
        }
    }
    0
}

// 체이닝 install: 진입부 현재 12B(원본 프롤로그 or 다른모드 jmp패치) 저장 → 앞에 끼움. 멱등.
// 두 asset-get copy 를 모두 후킹(0.5.1 모노모픽 분화 대응).
//   copy #1 = LOADER_RVA  : main/player_info/wide/banpick/strategy/training 전부 (0.5.3~0.5.4 단일 copy)
//   copy #2 = BANPICK_LOADER_RVA = 0 (폐지 — 0.5.3부터 없음, 0.5.4 재확인: banpick 문자열 19표 전부 copy#1)
// ★★1회만 설치(INSTALLED 게이트) — 절대 매프레임 재설치로 바꾸지 말 것.
//   [2026-07-18 먹통 사고] 재체인을 매프레임으로 돌렸다가 게임 hang 제보. 원인 = 상호 체인 사이클:
//     ①draft 설치(entry=draft_stub) → ②item_tactics 가 1회 설치하며 draft_stub 12B 를 자기 tramp 에 담음
//     (tramp_item = jmp draft_detour, entry=item_stub) → ③draft 가 "내 스텁 아님" 판정하고 재설치하며
//     item_stub 12B 를 자기 tramp 에 담음(tramp_draft = jmp item_detour)
//     ⇒ draft_detour→tramp_draft→item_detour→tramp_item→draft_detour… **무한 재귀 = 원본 미도달 = 먹통**.
//   상대 트램폴린은 그쪽 static 이라 런타임 사이클 검사가 불가능하다. 따라서 "덮어써도 재설치 안 함"이
//   유일한 안전 선택(고아가 되면 오버레이만 안 뜸 = 먹통보다 훨씬 나음). 순서 보장은 늦은 설치로만 한다.
//   ※CLAUDE.md §3 의 "매프레임 재검증" 권고는 상대가 1회 설치일 때만 성립 — 여기선 적용 금지.
unsafe fn install() -> bool {
    if INSTALLED.swap(true, Ordering::Relaxed) { return true; }
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { INSTALLED.store(false, Ordering::Relaxed); return false; }
    BASE.store(base, Ordering::Relaxed);
    let a = install_one(base + LOADER_RVA, &TRAMP, detour as usize);
    // 0.5.3: copy #2 소멸(밴픽도 copy #1 이 처리) ⟹ 설치 스킵. BANPICK_LOADER_RVA==0 이면 영구 미설치.
    //   ★같은 주소를 두 번 걸면 자기 체인 무한재귀가 되므로 0 검사는 **절대 제거 금지**.
    let b = if BANPICK_LOADER_RVA != 0 && BANPICK_LOADER_RVA != LOADER_RVA {
        install_one(base + BANPICK_LOADER_RVA, &TRAMP2, detour2 as usize)
    } else { let _ = (&TRAMP2, detour2 as usize); false };
    if !a && !b { INSTALLED.store(false, Ordering::Relaxed); return false; } // 둘 다 실패 시 재시도 허용
    dbg_write(&format!("loader hooks installed: copy1={} copy2={}", a, b));
    true
}

// 진입부 12B 를 내 detour 로 교체하고, 원래 12B 는 트램폴린 앞에 보존 → 체인 유지.
//   ⚠체인 시 진입부는 원본 프롤로그가 아니라 상대 모드 스텁(48 b8 <tgt> ff e0)일 수 있다.
//   그 12B 를 그대로 트램폴린에 담으므로 실행 시 상대 detour 로 점프 = 두 모드 순차 발화.
//   (movabs+jmp rax 가 정확히 12B → 뒤의 fn+0xc 점프에는 도달하지 않는다.)
unsafe fn install_one(fn_addr: usize, tramp: &AtomicUsize, dfn: usize) -> bool {
    if fn_addr <= 0x10000 { return false; }
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    // 이미 내 훅이 앞단이면 no-op(정상 경로).
    if cur[0] == 0x48 && cur[1] == 0xb8 {
        let tgt = usize::from_le_bytes(cur[2..10].try_into().unwrap());
        if tgt == dfn { return true; }
    }
    let stub = VirtualAlloc(0, 64, MEM_CR, RWX);
    if stub == 0 { return false; }
    // tramp = [현재 진입부 12B(원본 프롤로그 또는 상대 스텁)] + [movabs rax, fn+0xc; jmp rax]
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&cur);
    s.extend_from_slice(&[0x48, 0xb8]);
    s.extend_from_slice(&(fn_addr + 0xc).to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    tramp.store(stub, Ordering::Relaxed);
    // patch entry = movabs rax, dfn; jmp rax
    let mut patch = [0u8; 12];
    patch[0] = 0x48; patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&dfn.to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, RWX, &mut old) == 0 { return false; }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    true
}

// ── 데이터 채움 ─────────────────────────────────────────────────────────────
fn data_path() -> String {
    let mut buf = [0u16; 520];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n > 0 && n < buf.len() {
        let exe = String::from_utf16_lossy(&buf[..n]);
        if let Some(i) = exe.rfind(|c| c == '\\' || c == '/') {
            return format!(r"{}\mods\tfm2_draft_overlay\overlay_data.txt", &exe[..i]);
        }
    }
    r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_draft_overlay\overlay_data.txt".to_string()
}

struct Ext;
impl ModExtension for Ext {
    fn post_update(&self, _s: &mut Scene, u: &mut GameUI, a: &mut Assets, _dt: f32) {
        // ★콜백 본문 panic 이 게임 콜스택으로 unwind = UB/하드크래시 → catch_unwind 로 격리.
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // 늦은 체이닝 설치(다른 모드 init 후 → 훅 뒤에 붙음). 멱등이라 매프레임 no-op.
        unsafe { install(); }
        // 팝업이 트리에 없으면(=밴픽 화면 아님) 아무것도 안 함.
        if ui_kit::find(&u.root, "draft_overlay").is_none() { return; }
        // 런타임 #anim UV 리졸버용 Assets 포인터 갱신(밴픽 화면에서만·이 프레임 유효).
        ASSETS_PTR.store(a as *mut Assets as usize, Ordering::Relaxed);
        // 워크샵 모드챔프 1회 스캔(새 모드 자동 감지 → champ_key/face 동적). 최초 밴픽 프레임에만 실제 스캔.
        ensure_rt_scan();

        // 팝업 표시/숨김(X버튼·토글) + 드래그 밴드(헤더 44px) 충전. 매 프레임.
        {
            let show = SHOW_OVERLAY.load(Ordering::Relaxed);
            if let Some(ov) = ui_kit::find_mut(&mut u.root, "draft_overlay") {
                if ov.visible != show { ov.visible = show; }
                if show {
                    // draggable_popup 드래그 밴드 = 헤더 높이 → 상단 잡고 드래그(빌더가 0으로 둠 → 매프레임 set).
                    if let Some(b) = ui_kit::runner_base(ov, "DraggablePopup") {
                        ui_kit::runner_wr_f32(b, 0x1c0, 44.0);
                    }
                }
            }
            // 팝업이 보이는 동안: 게임 밴픽 호버 툴팁(선수 능력치 등)을 강제 숨김 → 팝업 위 호버 시
            //   툴팁이 안 뜨고, 툴팁이 우리 클릭을 뺏던 문제도 해소. (숨김이면 안 건드림 = 밴픽 정상.)
            if show {
                hide_hover_tooltips(&mut u.root);
                // ★근본 해결: 우리 조각을 매 프레임 부모 children 맨 끝으로 → 히트/렌더 DFS 최후 승자 유지.
                //   (게임이 호버 후 자기 노드를 뒤로 재배치해 클릭/드래그를 뺏던 것을 무력화.)
                raise_to_top(&mut u.root);
            }
        }

        // ★진단: 팝업 표시 후 ~30프레임(레이아웃 안정) 뒤 트리 + scroll_view 러너 높이 필드 1회 덤프.
        if DBG && !DUMP_DONE.load(Ordering::Relaxed) {
            if DUMP_FRAME.fetch_add(1, Ordering::Relaxed) == 30 {
                DUMP_DONE.store(true, Ordering::Relaxed);
                if let Some(ov) = ui_kit::find(&u.root, "draft_overlay") {
                    let mut s = String::from("\n=== HIT-TEST 게이트 진단 (차단막 vs 정상 노드) ===\n");
                    for id in ["draft_overlay", "ov_blocker", "ov_bg", "body", "list", "row0", "gcell0", "tab0"] {
                        dbg_hit_fields(&u.root, id, &mut s);
                    }
                    s.push_str("=== draft_overlay TREE ===\n");
                    dump_tree(ov, 0, &mut s);
                    // scroll 러너: content/viewport 높이(626/1580 근처) 찾기 — 넓게 스캔.
                    if let Some(sc) = ui_kit::find(&u.root, "scroll") {
                        if let Some(base) = ui_kit::runner_base(sc, "ScrollView") {
                            s.push_str(&format!("scroll base={:#x}: 626/1580 근처 값:\n", base));
                            let mut off = 0;
                            while off < 0x400 {
                                if let Some(f) = ui_kit::runner_rd_f32(base, off) {
                                    if f.is_finite() && ((f - 626.0).abs() < 3.0 || (f - 1580.0).abs() < 3.0 || (f - 604.0).abs() < 3.0) {
                                        s.push_str(&format!("  +{:#x} = {:.1} ★\n", off, f));
                                    }
                                }
                                off += 4;
                            }
                        }
                    }
                    // list 노드 레이아웃 높이(1057)/폭(604) 필드 찾기 — 동적 높이 세팅용.
                    if let Some(li) = ui_kit::find(&u.root, "list") {
                        s.push_str(&format!("list runner_type = {}\n", ui_kit::runner_type_name(li)));
                        // (a) 러너 인스턴스 메모리 스캔
                        if let Some(base) = ui_kit::runner_base(li, "Runner") {
                            s.push_str(&format!("list runner base={:#x}: 1057/604 근처:\n", base));
                            let mut off = 0;
                            while off < 0x300 {
                                if let Some(f) = ui_kit::runner_rd_f32(base, off) {
                                    if f.is_finite() && ((f - 1057.0).abs() < 3.0 || (f - 604.0).abs() < 3.0) {
                                        s.push_str(&format!("  runner+{:#x} = {:.1} ★\n", off, f));
                                    }
                                }
                                off += 4;
                            }
                        }
                        // (b) mod_api Node 구조체 주소 자체 스캔(라이브 게임노드면 레이아웃 h/w 있음)
                        let node_addr = li as *const Node as usize;
                        s.push_str(&format!("list node addr={:#x}: 1057/604 근처:\n", node_addr));
                        let mut off = 0;
                        while off < 0x300 {
                            if let Some(f) = ui_kit::runner_rd_f32(node_addr, off) {
                                if f.is_finite() && ((f - 1057.0).abs() < 3.0 || (f - 604.0).abs() < 3.0) {
                                    s.push_str(&format!("  node+{:#x} = {:.1} ★\n", off, f));
                                }
                            }
                            off += 4;
                        }
                    }
                    dbg_write(&s);
                }
            }
        }

        // 1) 파일 변경 시 재파싱 → SECTIONS 캐시.
        //    ★변경감지 = mtime(±len). 길이만 보면 "값만 바뀌고 바이트길이 동일"한 재export(새로고침 등)를 놓침.
        let sig = std::fs::metadata(data_path()).map(|m| {
            let t = m.modified().ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_nanos() as u64).unwrap_or(0);
            t ^ m.len().rotate_left(17)
        }).unwrap_or(0);
        if LAST_SIG.swap(sig, Ordering::Relaxed) != sig {
            let txt = std::fs::read_to_string(data_path()).unwrap_or_default();
            *SECTIONS.lock().unwrap_or_else(|e| e.into_inner()) = parse_sections(&txt);
        }

        // 2) 클릭 라우트: 탭(3) + 역할필터(5) + 행(30). 멀티모드 안전 재등록.
        //    역할/행 클릭은 write_control 로 TFM2.gg 에 의도 전달(대시보드가 재계산·재export).
        let mut routes: Vec<(String, ui_kit::ClickFn)> = Vec::new();
        for i in 0..NUM_TABS {
            let ii = i;
            routes.push(ui_kit::route(&format!("tab{}", i), Rc::new(move || { SEL_TAB.store(ii, Ordering::Relaxed); })));
        }
        // 새로고침 버튼 → 대시보드 세이브 재읽기·갱신(일회성 refresh=1, 지속상태 미저장).
        routes.push(ui_kit::route("refresh_btn", Rc::new(|| { write_control("refresh=1"); })));
        // 창 닫기(X) = 숨김 / 좌하단 토글 = 표시 flip.
        routes.push(ui_kit::route("close_btn", Rc::new(|| { SHOW_OVERLAY.store(false, Ordering::Relaxed); })));
        routes.push(ui_kit::route("draft_toggle", Rc::new(|| {
            let v = SHOW_OVERLAY.load(Ordering::Relaxed);
            SHOW_OVERLAY.store(!v, Ordering::Relaxed);
        })));
        // 챔피언정보 서브탭 (통계/기본정보/상대·빌드/패치) — 게임 로컬 전환.
        for i in 0..4 {
            let ii = i;
            routes.push(ui_kit::route(&format!("csub{}", i), Rc::new(move || { SEL_CHAMPSUB.store(ii, Ordering::Relaxed); })));
        }
        for i in 0..6 {
            let ii = i;
            routes.push(ui_kit::route(&format!("role{}", i), Rc::new(move || {
                SEL_ROLE.store(ii, Ordering::Relaxed);
                write_control_state();
            })));
        }
        // 데이터 범위(전체/대회/솔랭) — 대시보드 state.scope 로 전달.
        for i in 0..3 {
            let ii = i;
            routes.push(ui_kit::route(&format!("scope{}", i), Rc::new(move || {
                SEL_SCOPE.store(ii, Ordering::Relaxed);
                write_control_state();
            })));
        }
        // 밴픽코치 슬롯(상대밴/내밴/상대픽/내픽) — 활성 슬롯 선택. 그리드 챔프 클릭이 이 슬롯으로 토글.
        for i in 0..4 {
            let ii = i;
            routes.push(ui_kit::route(&format!("dslot{}", i), Rc::new(move || {
                SEL_DSLOT.store(ii, Ordering::Relaxed);
                write_control(&format!("draft_slot={}", DSLOT_KEYS[ii]));
            })));
        }
        // 챔프 그리드 셀(72) 클릭 = 활성 슬롯에 추가/제거(토글).
        for k in 0..NGRID {
            let kk = k;
            routes.push(ui_kit::route(&format!("gcell{}", k), Rc::new(move || {
                let id = {
                    let g = GRID_CHAMP.lock().unwrap_or_else(|e| e.into_inner());
                    g.get(kk).cloned().unwrap_or_default()
                };
                if !id.is_empty() {
                    // 탭4(모의밴픽) 그리드클릭 = 현재 스텝에 배치(bp_assign) / 탭3(밴픽코치) = 슬롯 토글.
                    if SEL_TAB.load(Ordering::Relaxed) == 4 { write_control(&format!("bp_assign={}", id)); }
                    else { write_control(&format!("draft_toggle={}", id)); }
                }
            })));
        }
        // 모의밴픽(탭4) 컨트롤 패널 버튼 → bp 명령(대시보드가 상태변경·재export).
        for i in 0..5 {
            let ii = i;
            routes.push(ui_kit::route(&format!("bpset{}", i), Rc::new(move || { write_control(&format!("bp_setidx={}", ii)); })));
        }
        const BP_RULES: [&str; 3] = ["classic", "fearless", "hardFearless"];
        for i in 0..3 {
            let ii = i;
            routes.push(ui_kit::route(&format!("bprule{}", i), Rc::new(move || { write_control(&format!("bp_rule={}", BP_RULES[ii])); })));
        }
        for (i, n) in [(0usize, 2u8), (1usize, 3u8)] {
            routes.push(ui_kit::route(&format!("bpban{}", i), Rc::new(move || { write_control(&format!("bp_bans={}", n)); })));
        }
        routes.push(ui_kit::route("bpside", Rc::new(|| { write_control("bp_flip=1"); })));
        routes.push(ui_kit::route("bppatch", Rc::new(|| { write_control("bp_patch=1"); })));
        for k in ["stat", "meta", "game", "solo", "syn", "ctr"] {
            routes.push(ui_kit::route(&format!("bpwdec_{}", k), Rc::new(move || { write_control(&format!("bp_w={}:dec", k)); })));
            routes.push(ui_kit::route(&format!("bpwinc_{}", k), Rc::new(move || { write_control(&format!("bp_w={}:inc", k)); })));
        }
        for i in 0..MAX_ROWS {
            let ii = i;
            routes.push(ui_kit::route(&format!("row{}", i), Rc::new(move || {
                // ★행 클릭 = 그 행의 챔프 id 로 챔프 선택(탭 무관). 챔프 id 없는 행(헤더/텍스트)은 무시.
                let id = {
                    let rc = ROW_CHAMP.lock().unwrap_or_else(|e| e.into_inner());
                    rc.get(ii).cloned().unwrap_or_default()
                };
                if id.is_empty() { return; }
                // 모의밴픽(탭4) 등 raw 컨트롤 토큰(key=value: bp_set_next=1/bp_w=stat 등) → 그대로 전달.
                //   (챔프 id·SLOT:·CLEAR 엔 '=' 없음 → 탭3 무영향.)
                if id.contains('=') { write_control(&id); return; }
                // 밴픽코치 전체 초기화 행.
                if id == "CLEAR" { write_control("draft_clear=1"); return; }
                // 밴픽코치 슬롯 행(SLOT:<key>) 클릭 = 활성 슬롯 선택. 그 외 = 챔프정보로 이동.
                if let Some(key) = id.strip_prefix("SLOT:") {
                    if let Some(idx) = DSLOT_KEYS.iter().position(|d| *d == key) {
                        SEL_DSLOT.store(idx, Ordering::Relaxed);
                        write_control(&format!("draft_slot={}", key));
                    }
                    return;
                }
                // 모의밴픽(탭4) 챔프 행(추천) 클릭 = 현재 스텝에 배치.
                if SEL_TAB.load(Ordering::Relaxed) == 4 { write_control(&format!("bp_assign={}", id)); return; }
                *LAST_CHAMP_ID.lock().unwrap_or_else(|e| e.into_inner()) = id;
                SEL_TAB.store(1, Ordering::Relaxed); // 챔피언정보 탭으로 이동
                write_control_state();
            })));
        }
        ui_kit::ensure_clicks(u, &CLICK_LAST, routes);

        // 3) 렌더(파일/탭/역할/서브탭 변경 시만). 첫 진입(row0 빈칸)이면 강제 1회.
        let sel = SEL_TAB.load(Ordering::Relaxed).min(NUM_TABS - 1);
        let role_sel = SEL_ROLE.load(Ordering::Relaxed);
        let csub_sel = SEL_CHAMPSUB.load(Ordering::Relaxed);
        let scope_sel = SEL_SCOPE.load(Ordering::Relaxed);
        let dslot_sel = SEL_DSLOT.load(Ordering::Relaxed);

        // ★챔프 이미지 스크롤 연동(매 프레임, 렌더 게이트 밖 — 스크롤은 render 키를 안 바꿔서 여기서 처리).
        //   루트 절대노드라 뷰포트 클리핑 불가 → 스크롤 내리면 콘텐츠와 함께 사라지게(최상단 비율≈0에서만 표시).
        {
            let at_top = ui_kit::find(&u.root, "scroll")
                .and_then(ui_kit::scroll_get).unwrap_or(0.0) <= 0.01;
            let show = sel == 1 && IMG_HAS_CHAMP.load(Ordering::Relaxed) && at_top;
            if let Some(img) = ui_kit::find_mut(&mut u.root, "champ_img") {
                if img.visible != show { img.visible = show; }
            }
        }

        let key = sig.wrapping_shl(12) ^ ((sel as u64) << 8) ^ ((role_sel as u64) << 4) ^ (csub_sel as u64) ^ ((scope_sel as u64) << 20) ^ ((dslot_sel as u64) << 24);
        let row0_empty = ui_kit::find(&u.root, "row0")
            .and_then(ui_kit::label_get).map(|s| s.is_empty()).unwrap_or(true);
        if !row0_empty && LAST_RENDER.swap(key, Ordering::Relaxed) == key { return; }
        LAST_RENDER.store(key, Ordering::Relaxed);

        // 새 뷰(탭/서브탭/역할/챔프) 진입 시 스크롤 최상단으로 → 새 내용 처음부터 + 챔프 이미지 at_top 판정 복구.
        //   (sig-only 변경=대시보드 자동갱신 시엔 nav 동일 → 스크롤 유지)
        {
            let champ_hash = {
                let c = LAST_CHAMP_ID.lock().unwrap_or_else(|e| e.into_inner());
                c.bytes().fold(0xcbf29ce484222325u64, |h, b| (h ^ b as u64).wrapping_mul(0x100000001b3))
            };
            let nav = ((sel as u64) << 56) ^ ((role_sel as u64) << 48) ^ ((csub_sel as u64) << 40) ^ ((dslot_sel as u64) << 32) ^ champ_hash;
            if LAST_NAV.swap(nav, Ordering::Relaxed) != nav {
                ui_kit::scroll_set_by_id(&mut u.root, "scroll", 0.0);
            }
        }

        // 바 가시성: 메타=rolebar / 챔프=csub / 해석=둘다 숨김. + 탭/역할/서브탭 라벨 하이라이트.
        let teal = ui_kit::Rgba::hex(0x37d5b3ff);
        let dim = ui_kit::Rgba::hex(0xa3a9b6ff);
        if let Some(rb) = ui_kit::find_mut(&mut u.root, "rolebar") { rb.visible = sel == 0; }
        if let Some(cb) = ui_kit::find_mut(&mut u.root, "csub") { cb.visible = sel == 1; }
        if let Some(db) = ui_kit::find_mut(&mut u.root, "dslot") { db.visible = sel == 3; }
        if let Some(bp) = ui_kit::find_mut(&mut u.root, "bppanel") { bp.visible = sel == 4; }
        for i in 0..NUM_TABS {
            if let Some(l) = ui_kit::find_mut(&mut u.root, &format!("tab{}l", i)) {
                ui_kit::label_set_color(l, if i == sel { teal } else { dim });
            }
        }
        for i in 0..6 {
            if let Some(l) = ui_kit::find_mut(&mut u.root, &format!("role{}l", i)) {
                ui_kit::label_set_color(l, if i == role_sel { teal } else { dim });
            }
        }
        for i in 0..4 {
            if let Some(l) = ui_kit::find_mut(&mut u.root, &format!("csub{}l", i)) {
                ui_kit::label_set_color(l, if i == csub_sel { teal } else { dim });
            }
        }
        for i in 0..3 {
            if let Some(l) = ui_kit::find_mut(&mut u.root, &format!("scope{}l", i)) {
                ui_kit::label_set_color(l, if i == scope_sel { teal } else { dim });
            }
        }
        for i in 0..4 {
            if let Some(l) = ui_kit::find_mut(&mut u.root, &format!("dslot{}l", i)) {
                ui_kit::label_set_color(l, if i == dslot_sel { teal } else { dim });
            }
        }

        // 선택 탭의 제목 + 행들을 소유값으로 뽑고 락 해제(트리 수술 중 락 미보유).
        let (title, mut lines): (String, Vec<String>) = {
            let secs = SECTIONS.lock().unwrap_or_else(|e| e.into_inner());
            if secs.is_empty() {
                ("데이터 대기 중… (TFM2.gg 를 켜세요)".to_string(), Vec::new())
            } else {
                let idx = sel.min(secs.len() - 1);
                (format!("{}  ·  TFM2.gg 연동됨", secs[idx].0), secs[idx].1.clone())
            }
        };
        // 챔피언정보 탭: "#SUB <이름>" 마커로 서브탭 분할 → 헤더(첫 #SUB 앞) + 선택 서브탭 행.
        if sel == 1 {
            let mut champ_id = String::new();
            let mut header: Vec<String> = Vec::new();
            let mut subs: Vec<Vec<String>> = Vec::new();
            for ln in &lines {
                if let Some(id) = ln.strip_prefix("#IMG ") {
                    champ_id = id.trim().to_string();
                } else if ln.starts_with("#SUB ") {
                    subs.push(Vec::new());
                } else if subs.is_empty() {
                    header.push(ln.clone());
                } else if let Some(last) = subs.last_mut() {
                    last.push(ln.clone());
                }
            }
            // 챔프 전신 이미지: source 세팅 + 존재 플래그(표시 여부는 매프레임 스크롤 판정이 결정).
            if let Some(img) = ui_kit::find_mut(&mut u.root, "champ_img") {
                if champ_id.is_empty() {
                    IMG_HAS_CHAMP.store(false, Ordering::Relaxed);
                } else {
                    unsafe { set_champ_image(img, &champ_id); }
                    IMG_HAS_CHAMP.store(true, Ordering::Relaxed);
                }
            }
            let cs = csub_sel.min(subs.len().saturating_sub(1));
            let mut out = header;
            if let Some(rows) = subs.get(cs) { out.extend(rows.iter().cloned()); }
            lines = out;
        } else {
            IMG_HAS_CHAMP.store(false, Ordering::Relaxed); // 챔프탭 아님
        }
        // 모의밴픽(탭4): "#BPSTATE k=v …" 라인 파싱 → 컨트롤 패널 버튼 하이라이트/값 갱신 후 라인 제거.
        if sel == 4 {
            if let Some(pos) = lines.iter().position(|l| l.starts_with("#BPSTATE ")) {
                let st = lines.remove(pos);
                let body = &st["#BPSTATE ".len()..];
                let getv = |key: &str| -> Option<String> {
                    body.split_whitespace().find_map(|tok| {
                        tok.split_once('=').and_then(|(k, v)| if k == key { Some(v.to_string()) } else { None })
                    })
                };
                let seti: usize = getv("set").and_then(|v| v.parse().ok()).unwrap_or(0);
                let rule = getv("rule").unwrap_or_else(|| "classic".into());
                let ruleidx = ["classic", "fearless", "hardFearless"].iter().position(|r| *r == rule).unwrap_or(0);
                let bans: u8 = getv("bans").and_then(|v| v.parse().ok()).unwrap_or(2);
                for i in 0..5 {
                    if let Some(l) = ui_kit::find_mut(&mut u.root, &format!("bpset{}l", i)) {
                        ui_kit::label_set_color(l, if i == seti { teal } else { dim });
                    }
                }
                for i in 0..3 {
                    if let Some(l) = ui_kit::find_mut(&mut u.root, &format!("bprule{}l", i)) {
                        ui_kit::label_set_color(l, if i == ruleidx { teal } else { dim });
                    }
                }
                for (i, n) in [(0usize, 2u8), (1usize, 3u8)] {
                    if let Some(l) = ui_kit::find_mut(&mut u.root, &format!("bpban{}l", i)) {
                        ui_kit::label_set_color(l, if n == bans { teal } else { dim });
                    }
                }
                if let Some(l) = ui_kit::find_mut(&mut u.root, "bpsidel") {
                    ui_kit::label_set(l, if getv("side").as_deref() == Some("red") { "레드" } else { "블루" });
                }
                if let Some(l) = ui_kit::find_mut(&mut u.root, "bppatchl") {
                    ui_kit::label_set(l, if getv("patch").as_deref() == Some("1") { "패치보정 ON" } else { "패치보정 OFF" });
                }
                for k in ["stat", "meta", "game", "solo", "syn", "ctr"] {
                    if let Some(l) = ui_kit::find_mut(&mut u.root, &format!("bpw_{}val", k)) {
                        ui_kit::label_set(l, &getv(k).unwrap_or_else(|| "0".into()));
                    }
                }
            }
        }
        // 밴픽코치: "#GRID" 마커로 recs(위)/그리드 엔트리(아래) 분리 → recs만 스크롤행, 엔트리로 챔프 그리드 채움.
        let grid_on = sel == 3 || sel == 4; // 밴픽코치·모의밴픽 = 챔프 그리드 표시
        let mut grid_h = 0.0f32; // 동적 그리드 높이(사용 행수만큼) — content_h·#pgrid 높이에 반영.
        if grid_on {
            let entries: Vec<String> = match lines.iter().position(|l| l == "#GRID") {
                Some(p) => lines.split_off(p).into_iter().skip(1).collect(), // #GRID 이후 = 엔트리, lines엔 recs만 남음
                None => Vec::new(),
            };
            // 실제 챔프 수 → 사용 행수 → 그리드 높이(빈 줄 축소·챔프 많으면 확장). 상단 여백 2 + 하단 4.
            let used = entries.len().min(NGRID);
            let rows_used = (used + GCOLS - 1) / GCOLS;
            grid_h = (rows_used as f32) * GC_H + 6.0;
            let mut gchamps = vec![String::new(); NGRID];
            if let Some(pg) = ui_kit::find_mut(&mut u.root, "pgrid") {
                for k in 0..NGRID {
                    let (id, state): (String, u8) = match entries.get(k) {
                        Some(e) => {
                            let (i, s) = e.split_once('\t').unwrap_or((e.as_str(), "0"));
                            (i.to_string(), s.trim().parse::<u8>().unwrap_or(0))
                        }
                        None => (String::new(), 0),
                    };
                    gchamps[k] = id.clone();
                    if let Some(cell) = ui_kit::find_mut(pg, &format!("gcell{}", k)) {
                        cell.visible = !id.is_empty();
                        cell.disabled = id.is_empty();
                        if let Some(bg) = ui_kit::find_mut(cell, &format!("gbg{}", k)) {
                            let c = match state { 1 => 0x37d5b3aa, 2 => 0x00000088, _ => 0x00000000 }; // 1=선택 청록, 2=이월 어둡게
                            ui_kit::rect_set_color(bg, ui_kit::Rgba::hex(c));
                        }
                        if let Some(img) = ui_kit::find_mut(cell, &format!("gimg{}", k)) {
                            let ok = !id.is_empty() && unsafe { set_face_image_buf(img, core::ptr::addr_of_mut!(GRID_FACE_BUF[k]) as *mut u8, &id) };
                            img.visible = ok;
                        }
                    }
                }
            }
            *GRID_CHAMP.lock().unwrap_or_else(|e| e.into_inner()) = gchamps;
        }
        if let Some(pg) = ui_kit::find(&u.root, "pgrid") {
            // 가시성 + 동적 높이(사용 행수만큼) → TopToBottom 부모가 아래 행들을 위로 당김.
            let na = pg as *const Node as usize;
            for off in [0x7c_usize, 0xfc, 0x17c, 0x1fc, 0x24c, 0x25c] { ui_kit::runner_wr_f32(na, off, grid_h); }
        }
        if let Some(pg) = ui_kit::find_mut(&mut u.root, "pgrid") { pg.visible = grid_on; }
        // ★동적 list 높이: 현재 탭 내용 행수에 맞춰 list 노드 레이아웃 높이 세팅 →
        //   탭별 스크롤 길이(내용 적으면 스크롤 없음, 여백 제거). 노드 주소 내 height 필드 6곳 실측.
        {
            let visible_rows = lines.len().min(MAX_ROWS);
            let mut content_h = 34.0 + (visible_rows as f32) * 31.0;
            if grid_on { content_h += grid_h; } // 그리드(#pgrid) 실제 높이 포함(동적)
            if sel == 4 { content_h += BPP_H; } // 모의밴픽 컨트롤 패널 높이 포함
            if let Some(li) = ui_kit::find(&u.root, "list") {
                let na = li as *const Node as usize;
                for off in [0x7c_usize, 0xfc, 0x17c, 0x1fc, 0x24c, 0x25c] {
                    ui_kit::runner_wr_f32(na, off, content_h);
                }
            }
            // 자체 스크롤바(네이티브 스크롤바는 z 무시라 콘텐츠에 덮임 — 규명 확정). content_h·스크롤비율로 thumb 갱신.
            const VP: f32 = 626.0; // 뷰포트(스크롤존) 높이
            let ratio = ui_kit::find(&u.root, "scroll").and_then(ui_kit::scroll_get).unwrap_or(0.0).clamp(0.0, 1.0);
            let need = content_h > VP + 1.0;
            if let Some(tr) = ui_kit::find_mut(&mut u.root, "sbar_track") {
                if tr.visible != need { tr.visible = need; }
            }
            if need {
                let thumb_h = (VP * VP / content_h).max(24.0).min(VP);
                let thumb_y = ratio * (VP - thumb_h);
                if let Some(th) = ui_kit::find_mut(&mut u.root, "sbar_thumb") {
                    ui_kit::set_layout_size(th, 0.0, thumb_h); // width 유지, height만
                    ui_kit::set_layout_xy(th, 0.0, thumb_y);   // 트랙 자식 → 상대 y
                }
            }
        }
        // 컬럼 헤더: 메타통계(0)·메타해석(2)에서 표시. 해석은 픽률→픽밴 라벨 + 표본 컬럼.
        if let Some(ch) = ui_kit::find_mut(&mut u.root, "colhead") {
            ch.visible = (sel == 0 || sel == 2) && !lines.is_empty();
        }
        if let Some(n) = ui_kit::find_mut(&mut u.root, "ch_pr") { ui_kit::label_set(n, if sel == 2 { "픽밴" } else { "픽률" }); }
        if let Some(n) = ui_kit::find_mut(&mut u.root, "ch_p5") { ui_kit::label_set(n, if sel == 2 { "표본" } else { "" }); }
        let mut row_champs: Vec<String> = vec![String::new(); MAX_ROWS];
        if let Some(list) = ui_kit::find_mut(&mut u.root, "list") {
            if let Some(st) = ui_kit::find_mut(list, "status") { ui_kit::label_set(st, &title); }
            // 각 행: "표시텍스트\x1f챔프id". 표시텍스트는 탭(\t)구분 컬럼 [c0=순위·이름, tier=뱃지, sc, wr, pr].
            //   챔프id 있으면 클릭가능(disabled=false), 없으면(헤더/텍스트) disabled=true → 호버/클릭 차단.
            for i in 0..MAX_ROWS {
                if let Some(row) = ui_kit::find_mut(list, &format!("row{}", i)) {
                    // 모의밴픽(탭4)은 상단 컨트롤 패널(#bppanel) 뒤 그리드가 row4 뒤 고정 →
                    //   콘텐츠(보드/추천/분석)를 row5+ 로 밀고 row0-4 는 숨겨(TopToBottom collapse) 패널→그리드→보드 순서 유지.
                    let li = if sel == 4 { if i < 5 { usize::MAX } else { i - 5 } } else { i };
                    if li == usize::MAX || li >= lines.len() {
                        row.visible = false;
                        row.disabled = true;
                        continue;
                    }
                    row.visible = true;
                    let (disp, cid) = match lines[li].split_once(CHAMP_SEP) {
                        Some((d, c)) => (d, c),
                        None => (lines[li].as_str(), ""),
                    };
                    row_champs[i] = cid.to_string();
                    row.disabled = cid.is_empty();
                    let parts: Vec<&str> = disp.split('\t').collect();
                    let is_slot = cid.starts_with("SLOT:");           // 밴픽코치 슬롯 선택 행
                    let is_action = is_slot || cid == "CLEAR" || cid.contains('=');  // 액션 행(슬롯/초기화/모의밴픽 컨트롤) = 얼굴 없음
                    let has_champ = !cid.is_empty() && !is_action;    // 진짜 챔프 행(얼굴 표시)
                    // 얼굴: 진짜 챔프 행에만(메타·해석 챔프행 + 챔프탭 관계행). 슬롯행/헤더는 X.
                    if let Some(f) = ui_kit::find_mut(row, &format!("r{}face", i)) {
                        let ok = has_champ && unsafe { set_face_image(f, i, cid) };
                        f.visible = ok;
                    }
                    // c0(번호/불릿/라벨) · mname(이름) 배치.
                    let (c0_text, name_text): (&str, &str) = if sel == 1 || sel == 3 || sel == 4 {
                        if has_champ {
                            ("", parts.first().copied().unwrap_or("").trim_start()) // 관계행: 얼굴+이름(mname)
                        } else {
                            (parts.first().copied().unwrap_or(""), "") // 라벨/헤더/슬롯행: c0
                        }
                    } else {
                        (parts.first().copied().unwrap_or(""), parts.get(6).copied().unwrap_or("")) // 메타/해석: c0=번호, 이름=mname
                    };
                    if let Some(n) = ui_kit::find_mut(row, &format!("r{}c0", i)) {
                        ui_kit::label_set(n, c0_text);
                        // 슬롯행 = 활성이면 청록, 아니면 어둡게. 일반행 = 기본색.
                        let c = if is_slot {
                            let active = cid.strip_prefix("SLOT:").and_then(|k| DSLOT_KEYS.iter().position(|d| *d == k)) == Some(dslot_sel);
                            if active { teal } else { dim }
                        } else { ui_kit::Rgba::hex(0xe8e8e8ff) };
                        ui_kit::label_set_color(n, c);
                    }
                    if let Some(n) = ui_kit::find_mut(row, &format!("r{}mname", i)) { set_wr_label(n, name_text); } // \x04 흐릿 지원
                    if sel == 1 || sel == 3 || sel == 4 {
                        // 챔피언정보(1)·밴픽코치(3)·모의밴픽(4): 넓은 val 필드(우측)에 값, 컬럼(뱃지/점수/승률/픽률) 비움.
                        if let Some(bg) = ui_kit::find_mut(row, &format!("r{}bg", i)) {
                            ui_kit::rect_set_color(bg, ui_kit::Rgba::hex(0x00000000));
                            if let Some(tt) = ui_kit::find_mut(bg, &format!("r{}tt", i)) { ui_kit::label_set(tt, ""); }
                        }
                        if let Some(n) = ui_kit::find_mut(row, &format!("r{}sc", i)) { ui_kit::label_set(n, ""); }
                        if let Some(n) = ui_kit::find_mut(row, &format!("r{}wr", i)) { ui_kit::label_set(n, ""); }
                        if let Some(n) = ui_kit::find_mut(row, &format!("r{}pr", i)) { ui_kit::label_set(n, ""); }
                        if let Some(n) = ui_kit::find_mut(row, &format!("r{}p5", i)) { ui_kit::label_set(n, ""); }
                        if let Some(n) = ui_kit::find_mut(row, &format!("r{}val", i)) { set_wr_label(n, parts.get(4).copied().unwrap_or("")); }
                    } else {
                        // 메타/해석 탭: 컬럼(티어뱃지/점수/승률/픽률), 넓은 val 비움.
                        let tier = parts.get(1).copied().unwrap_or("");
                        if let Some(bg) = ui_kit::find_mut(row, &format!("r{}bg", i)) {
                            let c = if tier.is_empty() { ui_kit::Rgba::hex(0x00000000) } else { tier_color(tier) };
                            ui_kit::rect_set_color(bg, c);
                            if let Some(tt) = ui_kit::find_mut(bg, &format!("r{}tt", i)) { ui_kit::label_set(tt, tier); }
                        }
                        if let Some(n) = ui_kit::find_mut(row, &format!("r{}sc", i)) { ui_kit::label_set(n, parts.get(2).copied().unwrap_or("")); }
                        if let Some(n) = ui_kit::find_mut(row, &format!("r{}wr", i)) { set_wr_label(n, parts.get(3).copied().unwrap_or("")); }
                        if let Some(n) = ui_kit::find_mut(row, &format!("r{}pr", i)) { ui_kit::label_set(n, parts.get(4).copied().unwrap_or("")); }
                        if let Some(n) = ui_kit::find_mut(row, &format!("r{}p5", i)) { ui_kit::label_set(n, parts.get(5).copied().unwrap_or("")); }
                        if let Some(n) = ui_kit::find_mut(row, &format!("r{}val", i)) { ui_kit::label_set(n, ""); }
                    }
                }
            }
        }
        *ROW_CHAMP.lock().unwrap_or_else(|e| e.into_inner()) = row_champs;

        // ★탭/바 라벨 색 재확정(행 렌더 이후). 행 렌더가 승률색(초록/빨강)을 쓰는데,
        //   드물게 탭 라벨이 그 색으로 물드는 현상 보고 → 렌더 마지막에 선택탭=teal / 나머지=dim 로 되돌린다.
        for i in 0..NUM_TABS {
            if let Some(l) = ui_kit::find_mut(&mut u.root, &format!("tab{}l", i)) {
                ui_kit::label_set_color(l, if i == sel { teal } else { dim });
            }
        }
        for i in 0..6 {
            if let Some(l) = ui_kit::find_mut(&mut u.root, &format!("role{}l", i)) {
                ui_kit::label_set_color(l, if i == role_sel { teal } else { dim });
            }
        }
        for i in 0..4 {
            if let Some(l) = ui_kit::find_mut(&mut u.root, &format!("csub{}l", i)) {
                ui_kit::label_set_color(l, if i == csub_sel { teal } else { dim });
            }
        }
        for i in 0..3 {
            if let Some(l) = ui_kit::find_mut(&mut u.root, &format!("scope{}l", i)) {
                ui_kit::label_set_color(l, if i == scope_sel { teal } else { dim });
            }
        }
        for i in 0..4 {
            if let Some(l) = ui_kit::find_mut(&mut u.root, &format!("dslot{}l", i)) {
                ui_kit::label_set_color(l, if i == dslot_sel { teal } else { dim });
            }
        }
        })); // catch_unwind
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new("tfm2_draft_overlay");
    reg.set_extension(Ext);
    reg
}
declare_mod!(init);
