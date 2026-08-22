//! icon_data.rs — 챔피언 아이콘 UV/크기: 공식 SDK `game_view::set_champion_icon`(0.5.6
//! ABI level 4)의 계산을 그대로 재현하되, 입력을 **라이브 bundle.game_data 에서 직접** 읽는다.
//! ⟹ 어느 유저 환경이든(신규 챔프·아트 업데이트 포함) 게임과 동일한 아이콘.
//!
//! 왜 API 직접 호출이 아닌가: set_champion_icon 은 넘긴 assets 에서
//! `champions/<id>#anim`(RectAnimationSet)을 조회하는데, 설정 화면 assets 엔 그 리소스가
//! 없고(밴픽/매치 씬 전용 로드) 모드가 강제 로드할 방법도 없다(Assets::load = private,
//! 엔진 내부 LoadingCommand 채널). = RE\2026-08-20 셋 참조.
//!
//! ★공식(네이티브 슬롯 캡처 8종과 대조 검증 — 7/8 정확일치, 1건은 스냅샷 낡음으로 설명):
//!   입력: idle frame0 rect(fx,fy,fw,fh) = fanim JSON `anims.idle.frames[0].data` /
//!         시트 크기(W,H) = #sheet PNG 헤더 / face 오프셋 = ChampionImageViewSetting(현재 빈 값=0)
//!   box 100×94, scale 2.0, y_offset −24 (게임 슬롯 빌더가 쓰는 값):
//!   u0px = fx + max(0,fw−50)/2          uw = min(fw,50)   node_w = fw>50 ? 100 : 2·fw
//!   v0px = fy − 12 + max(0,fh−47)/2     vh = min(fh,47)   node_h = fh>47 ? 94  : 2·fh
//!   uv = (u0px/W, v0px/H, uw/W, vh/H)
//!
//! 번들 포맷(ANA tfm2-ui-dsl-reference §1): [u32 count] 뒤 레코드
//!   [u32 extlen][ext][u32 pathlen][path][u32 bodylen][body] 반복. 헤더만 읽고 body 는 seek.

use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

pub const KEY_PREFIX: &str = "asset/base/aseprite_resources/champions/";

#[derive(Clone, Copy)]
pub struct IconUv {
    pub u0: f32,
    pub v0: f32,
    pub uw: f32,
    pub vh: f32,
    pub w: f32,
    pub h: f32,
}

// id -> (uv, source_key). source_key = "asset/<ns>/aseprite_resources/champions/<id>"(#sheet 없이).
static MAP: Mutex<Option<HashMap<String, (IconUv, String)>>> = Mutex::new(None);
pub static READY: AtomicBool = AtomicBool::new(false);
static STARTED: AtomicBool = AtomicBool::new(false);

pub fn get(id: &str) -> Option<(IconUv, String)> {
    MAP.lock()
        .unwrap_or_else(|e| e.into_inner())
        .as_ref()?
        .get(id)
        .cloned()
}

/// 백그라운드 1회 로드(파일 IO 만 — 게임 상태 무접근). post_update 첫 프레임에 호출.
pub fn start_load() {
    if STARTED.swap(true, Ordering::Relaxed) {
        return;
    }
    std::thread::spawn(|| {
        let r = std::panic::catch_unwind(build);
        let map = match r {
            Ok(Some(m)) => m,
            _ => HashMap::new(),
        };
        crate::config::dlog(&format!("icon_data: {}챔프 UV 계산 완료", map.len()));
        *MAP.lock().unwrap_or_else(|e| e.into_inner()) = Some(map);
        READY.store(true, Ordering::SeqCst);
    });
}

const BASE_KEY: &str = "asset/base/aseprite_resources/champions/";

fn build() -> Option<HashMap<String, (IconUv, String)>> {
    let dir = crate::mod_dir()?; // <game>\mods\tfm2_champ_pos_lock
    let game = std::path::Path::new(&dir).parent()?.parent()?.to_path_buf();
    let mut f = std::fs::File::open(game.join("bundle.game_data")).ok()?;
    let flen = f.metadata().ok()?.len();

    let mut anims: HashMap<String, (u64, u32)> = HashMap::new();
    let mut pngs: HashMap<String, u64> = HashMap::new();
    let mut base_cv: Option<(u64, u32)> = None; // 베이스 champion_view body (off,len)
    let mut b4 = [0u8; 4];
    let mut pos: u64 = 4;
    while pos + 12 < flen {
        f.seek(SeekFrom::Start(pos)).ok()?;
        f.read_exact(&mut b4).ok()?;
        let extlen = u32::from_le_bytes(b4) as usize;
        if extlen == 0 || extlen > 64 {
            break;
        }
        let mut ext = [0u8; 64];
        f.read_exact(&mut ext[..extlen]).ok()?;
        f.read_exact(&mut b4).ok()?;
        let pathlen = u32::from_le_bytes(b4) as usize;
        if pathlen == 0 || pathlen > 512 {
            break;
        }
        let mut pathb = vec![0u8; pathlen];
        f.read_exact(&mut pathb).ok()?;
        f.read_exact(&mut b4).ok()?;
        let bodylen = u32::from_le_bytes(b4);
        let body_off = pos + 4 + extlen as u64 + 4 + pathlen as u64 + 4;
        if let Ok(path) = core::str::from_utf8(&pathb) {
            if let Some(rest) = path.strip_prefix(KEY_PREFIX) {
                if &ext[..extlen] == b"fanim" {
                    if let Some(id) = rest.strip_suffix("#anim") {
                        anims.insert(id.to_string(), (body_off, bodylen));
                    }
                } else if &ext[..extlen] == b"png" {
                    if let Some(id) = rest.strip_suffix("#sheet") {
                        pngs.insert(id.to_string(), body_off);
                    }
                }
            } else if &ext[..extlen] == b"champion_view" && path == "asset/base/style/champion_view" {
                base_cv = Some((body_off, bodylen));
            }
        }
        pos = body_off + bodylen as u64;
    }

    // center_map: id→center.y (베이스 champion_view). 워크샵 팩 것은 scan_workshop 에서 병합.
    let mut center_map: HashMap<String, f32> = HashMap::new();
    if let Some((cvoff, cvlen)) = base_cv {
        if cvlen > 0 && cvlen < 4_000_000 && f.seek(SeekFrom::Start(cvoff)).is_ok() {
            let mut cvb = vec![0u8; cvlen as usize];
            if f.read_exact(&mut cvb).is_ok() {
                if let Ok(cvt) = core::str::from_utf8(&cvb) {
                    parse_centers(cvt, &mut center_map);
                }
            }
        }
    }

    let mut out = HashMap::new();
    for (id, (aoff, alen)) in anims {
        let Some(&poff) = pngs.get(&id) else { continue };
        // PNG IHDR: body+16 에 BE u32 width/height
        if f.seek(SeekFrom::Start(poff + 16)).is_err() {
            continue;
        }
        let mut wh = [0u8; 8];
        if f.read_exact(&mut wh).is_err() {
            continue;
        }
        let tw = u32::from_be_bytes(wh[..4].try_into().unwrap()) as f32;
        let th = u32::from_be_bytes(wh[4..].try_into().unwrap()) as f32;
        if alen == 0 || alen > 1_000_000 {
            continue;
        }
        if f.seek(SeekFrom::Start(aoff)).is_err() {
            continue;
        }
        let mut body = vec![0u8; alen as usize];
        if f.read_exact(&mut body).is_err() {
            continue;
        }
        let cy = center_map.get(&id).copied().unwrap_or(0.0);
        if let Some(uv) = compute(&body, tw, th, cy) {
            let key = format!("{BASE_KEY}{id}");
            out.insert(id, (uv, key));
        }
    }
    // 워크샵 익스포트/raw 팩(#anim.fanim + #sheet.png 루즈파일)도 스캔 — 익스포트한 모드챔프 커버.
    // raw .aseprite 만 있는 팩은 런타임 재익스포트라 정적으로 좌표 불가 → 미커버(호출측이 아이콘 숨김).
    if let Some(ws) = workshop_dir(&game) {
        scan_workshop(&ws, &mut out, &mut center_map);
    }
    Some(out)
}

/// ★set_champion_icon 공식 계산식(RE 확정): idle frame0 rect(x,y,w,h) + 시트(tw,th) → uv/크기.
/// box 100×94, scale 2.0, face=0(ChampionImageViewSetting 빈 값).
/// `cy` = champion_view.champion_view entries[id].center.y (텍스처 픽셀 raw 가산, 미등재=0 폴백).
/// ⚠구버전은 -12 하드코딩이었으나 실제로는 챔프별 center.y였음(53챔프만 -12, 나머지·미등재는 상이)
/// → 아래 잘림/위치 어긋남 원인. RE\2026-08-20_set_entity_icon-크롭식-centerY.md.
fn uv_from_rect(x: f32, y: f32, w: f32, h: f32, tw: f32, th: f32, cy: f32) -> Option<IconUv> {
    if !(1.0..=65536.0).contains(&tw) || !(1.0..=65536.0).contains(&th) || w <= 0.0 || h <= 0.0 {
        return None;
    }
    let (u0px, uw, nw) = if w > 50.0 {
        (x + 0.5 * (w - 50.0), 50.0, 100.0)
    } else {
        (x, w, 2.0 * w)
    };
    let (v0px, vh, nh) = if h > 47.0 {
        (y + cy + 0.5 * (h - 47.0), 47.0, 94.0)
    } else {
        (y + cy, h, 2.0 * h)
    };
    Some(IconUv {
        u0: u0px / tw,
        v0: v0px / th,
        uw: uw / tw,
        vh: vh / th,
        w: nw,
        h: nh,
    })
}

/// #sheet.png 크기(tw,th) + #anim.fanim(JSON RectAnimationSet) → IconUv. cy=center.y.
fn compute(fanim: &[u8], tw: f32, th: f32, cy: f32) -> Option<IconUv> {
    let txt = core::str::from_utf8(fanim).ok()?;
    let (fx, fy, fw, fh) = idle_frame0(txt)?;
    uv_from_rect(fx, fy, fw, fh, tw, th, cy)
}

// ── raw .aseprite 파서 (게임 로더/패킹 규칙 재현 — RE\2026-08-20_aseprite-로더-패킹.md) ──
fn ru16(b: &[u8], o: usize) -> Option<u16> {
    b.get(o..o + 2).map(|s| u16::from_le_bytes([s[0], s[1]]))
}
fn ri16(b: &[u8], o: usize) -> Option<i16> {
    b.get(o..o + 2).map(|s| i16::from_le_bytes([s[0], s[1]]))
}
fn ru32(b: &[u8], o: usize) -> Option<u32> {
    b.get(o..o + 4).map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

/// raw .aseprite → idle frame0 슬롯 rect(x,y,w,h) + 아틀라스(texW,texH). 게임 규칙:
/// 슬롯 = 프레임 visible cel 사각형 union(캔버스 클램프) / 가로 배치 x+=w+1(4096 줄바꿈) / texH=행최대h+1.
fn aseprite_idle(d: &[u8]) -> Option<(f32, f32, f32, f32, f32, f32)> {
    if ru16(d, 4)? != 0xA5E0 {
        return None;
    }
    let cw = ru16(d, 8)? as i32;
    let ch = ru16(d, 10)? as i32;
    let frames = ru16(d, 6)? as usize;
    if cw <= 0 || ch <= 0 || frames == 0 || frames > 4096 {
        return None;
    }
    let mut layers_vis: Vec<bool> = Vec::new();
    // 프레임별 cel: (layer, vis, 이미지 rect(x,y,cw,ch) 또는 None+링크 src)
    type Cel = (usize, bool, Option<(i32, i32, i32, i32)>, Option<usize>);
    let mut fcels: Vec<Vec<Cel>> = Vec::with_capacity(frames);
    let mut idle_from: Option<usize> = None;
    let mut pos = 128usize;
    for fi in 0..frames {
        let fbytes = ru32(d, pos)? as usize;
        let fstart = pos;
        let old_n = ru16(d, pos + 6)? as usize;
        let new_n = ru32(d, pos + 12)? as usize;
        let nchunks = if new_n != 0 { new_n } else { old_n };
        let mut cp = pos + 16;
        let mut cels: Vec<Cel> = Vec::new();
        for _ in 0..nchunks {
            let csz = ru32(d, cp)? as usize;
            if csz < 6 || cp + csz > d.len() {
                return None;
            }
            let ctype = ru16(d, cp + 4)?;
            let cd = &d[cp + 6..cp + csz];
            match ctype {
                0x2004 => {
                    if fi == 0 {
                        let flags = ru16(cd, 0)?;
                        layers_vis.push(flags & 1 != 0);
                    }
                }
                0x2005 => {
                    let layer = ru16(cd, 0)? as usize;
                    let x = ri16(cd, 2)? as i32;
                    let y = ri16(cd, 4)? as i32;
                    let celtype = ru16(cd, 7)?;
                    let vis = layers_vis.get(layer).copied().unwrap_or(true);
                    match celtype {
                        0 | 2 => {
                            let w = ru16(cd, 16)? as i32;
                            let h = ru16(cd, 18)? as i32;
                            cels.push((layer, vis, Some((x, y, w, h)), None));
                        }
                        1 => {
                            let src = ru16(cd, 16)? as usize;
                            cels.push((layer, vis, None, Some(src)));
                        }
                        _ => {}
                    }
                }
                0x2018 => {
                    let ntags = ru16(cd, 0)? as usize;
                    let mut tp = 10usize;
                    for _ in 0..ntags {
                        let from = ru16(cd, tp)? as usize;
                        let namelen = ru16(cd, tp + 17)? as usize;
                        if cd.get(tp + 19..tp + 19 + namelen) == Some(b"idle") {
                            idle_from = Some(from);
                        }
                        tp += 19 + namelen;
                    }
                }
                _ => {}
            }
            cp += csz;
        }
        fcels.push(cels);
        pos = fstart + fbytes;
    }
    // 프레임별 슬롯 크기 = visible cel union(캔버스 클램프) bbox = 트림.
    //   ★게임 네이티브 패커도 트림한다(베이스 챔프 #sheet가 전부 트림된 상태로 게임 캡처 UV와
    //   픽셀 일치 = crop 50×47·scale2). 작은 프레임의 음수 v0 는 게임도 동일(GPU clamp) = 정상.
    let mut slots: Vec<(i32, i32)> = Vec::with_capacity(frames);
    for cels in &fcels {
        let (mut x0, mut y0, mut x1, mut y1) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
        for &(layer, vis, img, link) in cels {
            if !vis {
                continue;
            }
            let rect = img.or_else(|| {
                link.and_then(|src| {
                    fcels
                        .get(src)
                        .and_then(|sc| sc.iter().find(|c| c.0 == layer && c.2.is_some()))
                        .and_then(|c| c.2)
                })
            });
            if let Some((x, y, w, h)) = rect {
                let (ax, ay, bx, by) = (x.max(0), y.max(0), (x + w).min(cw), (y + h).min(ch));
                if bx > ax && by > ay {
                    x0 = x0.min(ax);
                    y0 = y0.min(ay);
                    x1 = x1.max(bx);
                    y1 = y1.max(by);
                }
            }
        }
        if x1 > x0 && y1 > y0 {
            slots.push((x1 - x0, y1 - y0));
        } else {
            slots.push((0, 0));
        }
    }
    // 아틀라스 패킹
    let (mut cur_x, mut cur_y, mut row_h, mut wrapped) = (0i32, 0i32, 0i32, false);
    let mut placed: Vec<(i32, i32, i32, i32)> = Vec::with_capacity(frames);
    for &(sw, sh) in &slots {
        if cur_x + sw >= 4096 {
            cur_x = 0;
            cur_y += row_h;
            row_h = 0;
            wrapped = true;
        }
        placed.push((cur_x, cur_y, sw, sh));
        cur_x += sw + 1;
        row_h = row_h.max(sh);
    }
    let tex_w = if wrapped { 4096 } else { cur_x };
    let tex_h = cur_y + row_h + 1;
    let from = idle_from?;
    let &(px, py, pw, ph) = placed.get(from)?;
    if pw <= 0 || ph <= 0 || tex_w <= 0 {
        return None;
    }
    Some((
        px as f32, py as f32, pw as f32, ph as f32, tex_w as f32, tex_h as f32,
    ))
}

fn png_wh(path: &std::path::Path) -> Option<(f32, f32)> {
    let mut f = std::fs::File::open(path).ok()?;
    f.seek(SeekFrom::Start(16)).ok()?;
    let mut wh = [0u8; 8];
    f.read_exact(&mut wh).ok()?;
    Some((
        u32::from_be_bytes(wh[..4].try_into().unwrap()) as f32,
        u32::from_be_bytes(wh[4..].try_into().unwrap()) as f32,
    ))
}

fn workshop_dir(game: &std::path::Path) -> Option<std::path::PathBuf> {
    // <...>\steamapps\common\Teamfight Manager2 → <...>\steamapps\workshop\content\3009300
    let steamapps = game.parent()?.parent()?; // common → steamapps
    let ws = steamapps.join("workshop").join("content").join("3009300");
    ws.is_dir().then_some(ws)
}

/// 팩 mod.mod_info 의 top-level(최소 들여쓰기) mod_id → 에셋 네임스페이스.
fn pack_ns(pack: &std::path::Path) -> Option<String> {
    let txt = std::fs::read_to_string(pack.join("mod.mod_info")).ok()?;
    let mut best: Option<(usize, String)> = None;
    for line in txt.lines() {
        if line.contains("\"mod_id\"") {
            let indent = line.len() - line.trim_start().len();
            if let Some(v) = line.split(':').nth(1).and_then(|s| s.split('"').nth(1)) {
                if best.as_ref().map_or(true, |(bi, _)| indent < *bi) {
                    best = Some((indent, v.to_string()));
                }
            }
        }
    }
    best.map(|(_, v)| v)
}

/// champion_view.champion_view(JSON) → id→center.y 맵. entries 객체를 중괄호 매칭으로 순회.
/// (값에 문자열 중괄호 없음 — 전부 숫자라 안전.) 이미 있는 키는 덮지 않음(base 우선).
fn parse_centers(txt: &str, map: &mut HashMap<String, f32>) {
    let b = txt.as_bytes();
    let Some(e) = txt.find("\"entries\"") else {
        return;
    };
    let mut i = e + 9;
    while i < b.len() && b[i] != b'{' {
        i += 1;
    }
    i += 1; // entries 객체 내부
    loop {
        while i < b.len() {
            let c = b[i];
            if c == b' ' || c == b'\n' || c == b'\r' || c == b'\t' || c == b',' {
                i += 1;
            } else {
                break;
            }
        }
        if i >= b.len() || b[i] != b'"' {
            break; // '}' 또는 끝
        }
        i += 1;
        let ks = i;
        while i < b.len() && b[i] != b'"' {
            i += 1;
        }
        if i >= b.len() {
            break;
        }
        let key = &txt[ks..i];
        i += 1;
        while i < b.len() && b[i] != b'{' {
            i += 1;
        }
        let vs = i;
        let mut depth = 0i32;
        while i < b.len() {
            match b[i] {
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        i += 1;
                        break;
                    }
                }
                _ => {}
            }
            i += 1;
        }
        let val = &txt[vs..i.min(txt.len())];
        if let Some(c) = val.find("\"center\"") {
            if let Some(cy) = num_after(val, "y", c, 60) {
                map.entry(key.to_string()).or_insert(cy);
            }
        }
    }
}

fn json_str(txt: &str, key: &str) -> Option<String> {
    let pat = format!("\"{key}\"");
    let i = txt.find(&pat)? + pat.len();
    let rest = &txt[i..];
    let rest = &rest[rest.find(':')? + 1..];
    let rest = &rest[rest.find('"')? + 1..];
    let e = rest.find('"')?;
    Some(rest[..e].to_string())
}

/// 워크샵 = data_champion(id↔sprite 매핑) 주도로 스캔. 챔피언 id 와 스프라이트 파일명이
/// 다를 수 있어(예: meiling→hong) 반드시 data_champion 의 "sprite" 키를 따른다.
/// 소스 키 = data_champion 의 sprite 값 그대로. 스프라이트 파일 = 그 키의 basename.
fn scan_workshop(
    ws: &std::path::Path,
    out: &mut HashMap<String, (IconUv, String)>,
    center_map: &mut HashMap<String, f32>,
) {
    let Ok(packs) = std::fs::read_dir(ws) else { return };
    // 1) 전 팩 스프라이트 파일 인덱스(basename → 경로)
    let mut fanim: HashMap<String, std::path::PathBuf> = HashMap::new();
    let mut png: HashMap<String, std::path::PathBuf> = HashMap::new();
    let mut aseprite: HashMap<String, std::path::PathBuf> = HashMap::new();
    // 2) 전 팩 data_champion: (champ_id, sprite_key)
    let mut dcs: Vec<(String, String)> = Vec::new();
    for pack in packs.flatten() {
        let pack_dir = pack.path();
        // 팩 champion_view(있으면) → center_map 병합(base 미등재 챔프 채움)
        let cvp = pack_dir.join("style").join("champion_view.champion_view");
        if cvp.is_file() {
            if let Ok(cvt) = std::fs::read_to_string(&cvp) {
                parse_centers(&cvt, center_map);
            }
        }
        if let Ok(files) = std::fs::read_dir(pack_dir.join("aseprite_resources").join("champions")) {
            for e in files.flatten() {
                let p = e.path();
                let Some(n) = p.file_name().and_then(|s| s.to_str()) else {
                    continue;
                };
                if let Some(b) = n.strip_suffix("#anim.fanim") {
                    fanim.insert(b.to_string(), p.clone());
                } else if let Some(b) = n.strip_suffix("#sheet.png") {
                    png.insert(b.to_string(), p.clone());
                } else if let Some(b) = n.strip_suffix(".aseprite") {
                    aseprite.insert(b.to_string(), p.clone());
                }
            }
        }
        if let Ok(dcf) = std::fs::read_dir(pack_dir.join("champion")) {
            for e in dcf.flatten() {
                let p = e.path();
                if p.extension().and_then(|s| s.to_str()) != Some("data_champion") {
                    continue;
                }
                let Ok(txt) = std::fs::read_to_string(&p) else {
                    continue;
                };
                if let (Some(id), Some(sprite)) = (json_str(&txt, "id"), json_str(&txt, "sprite")) {
                    dcs.push((id, sprite));
                }
            }
        }
    }
    // 3) 조인: 각 챔피언 id → sprite basename 파일에서 UV 계산
    for (id, sprite_key) in dcs {
        if out.contains_key(&id) {
            continue;
        }
        let cy = center_map.get(&id).copied().unwrap_or(0.0);
        let base = sprite_key.rsplit('/').next().unwrap_or(&sprite_key);
        let uv = if let (Some(fa), Some(pg)) = (fanim.get(base), png.get(base)) {
            // 익스포트 형: #anim.fanim + #sheet.png
            match (std::fs::read(fa), png_wh(pg)) {
                (Ok(body), Some((tw, th))) => compute(&body, tw, th, cy),
                _ => None,
            }
        } else if let Some(ase) = aseprite.get(base) {
            // raw .aseprite: 게임 로더 규칙 재현
            std::fs::read(ase).ok().and_then(|b| {
                aseprite_idle(&b)
                    .and_then(|(x, y, w, h, tw, th)| uv_from_rect(x, y, w, h, tw, th, cy))
            })
        } else {
            None
        };
        if let Some(uv) = uv {
            out.insert(id, (uv, sprite_key));
        }
    }
}

fn num_after(txt: &str, key: &str, from: usize, window: usize) -> Option<f32> {
    let end = txt.len().min(from + window);
    let seg = &txt[from..end];
    let pat = format!("\"{key}\"");
    let i = seg.find(&pat)? + pat.len();
    let rest = seg[i..].trim_start_matches(|c: char| c == ':' || c.is_whitespace());
    let stop = rest
        .find(|c: char| !(c.is_ascii_digit() || c == '-' || c == '.'))
        .unwrap_or(rest.len());
    rest[..stop].parse().ok()
}

/// fanim JSON 에서 idle(없으면 첫 anims) frame0 의 data{x,y,w,h}.
fn idle_frame0(txt: &str) -> Option<(f32, f32, f32, f32)> {
    let start = txt.find("\"idle\"").or_else(|| txt.find("\"frames\""))?;
    let d = txt[start..].find("\"data\"")? + start;
    Some((
        num_after(txt, "x", d, 240)?,
        num_after(txt, "y", d, 240)?,
        num_after(txt, "w", d, 240)?,
        num_after(txt, "h", d, 240)?,
    ))
}
