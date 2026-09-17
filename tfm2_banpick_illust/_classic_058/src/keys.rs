//! 에셋 키 해석 — (챔피언 id, 진영, 확대여부) → 실제로 존재하는 일러스트 에셋 키.
//!
//! ★아트팩 구조 (2026-07-19 유저가 넣은 실팩 기준, 원본 dll 문자열 `blue`/`red`/`red_noflip` 과 일치)
//! ```
//! illust/blue/<champ>.png        블루 진영용
//! illust/red/<champ>.png         레드 진영용(좌우 뒤집힌 구도로 그려져 있음)
//! illust/red_noflip/<champ>.png  레드 진영용(뒤집지 않은 구도)
//! illust/<side>/<champ>-1.png    같은 그림의 **확대 구도** 버전
//! ```
//! 세 폴더는 서로 다른 그림이다(실측: md5 전부 상이). 즉 좌우반전은 아트에 이미
//! 반영돼 있으므로 **엔진 `flip_x` 는 쓰지 않는다** — 켜면 이중 반전된다.
//! `-1` 은 해상도만 2/3(1140x696 vs 1710x1044)이고 화면비는 동일 = 저해상도 사본이
//! 아니라 피사체를 크게 잡은 별도 구도.
//!
//! ★타 모드 아트(원본의 `2_Mod` 티어) = `asset/<mod_id>/BanPickIllust/<champ>` (진영 구분 없음).
//! 우리 팩에 없는 모드 챔피언(예: Touhou)이 여기서 잡힌다.
//!
//! ※0.5.2 공식 규격 일러스트 팩(`mods\<팩>\banpick_illustrations\<champ>.png`)을 아트
//!   공급원으로 함께 읽는 구현을 넣었다가 **유저 판단으로 되돌렸다**(2026-07-22).
//!   조사 결론 자체는 유효하므로 재착수 시 참고 = `MEM\tfm2-banpick-illust-mod.md`.

use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;

/// 진영 폴더명.
pub const BLUE: &str = "blue";
pub const RED: &str = "red";
pub const RED_NOFLIP: &str = "red_noflip";

/// 상대키(예: `blue/knight-1`, 모드아트는 `@/koishi`) → 완성된 에셋 키.
fn table() -> &'static HashMap<String, &'static str> {
    static T: OnceLock<HashMap<String, &'static str>> = OnceLock::new();
    T.get_or_init(build)
}

fn leak(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

/// `<dir>` 의 `*.png` 를 `key_of(stem)` → `val_of(stem)` 로 등록.
fn scan_png<K, V>(dir: &Path, map: &mut HashMap<String, &'static str>, key_of: K, val_of: V)
where
    K: Fn(&str) -> String,
    V: Fn(&str) -> String,
{
    let rd = match std::fs::read_dir(dir) {
        Ok(r) => r,
        Err(_) => return,
    };
    for e in rd.flatten() {
        let p = e.path();
        if p.extension().and_then(|s| s.to_str()) != Some("png") {
            continue;
        }
        if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
            if !stem.is_empty() {
                map.entry(key_of(stem)).or_insert_with(|| leak(val_of(stem)));
            }
        }
    }
}

/// mod.mod_info 에서 mod_id 추출(JSON 파서 없이 최소 스캔).
fn read_mod_id(dir: &Path) -> Option<String> {
    let s = std::fs::read_to_string(dir.join("mod.mod_info")).ok()?;
    let i = s.find("\"mod_id\"")?;
    let rest = &s[i + 8..];
    let rest = &rest[rest.find(':')? + 1..];
    let rest = &rest[rest.find('"')? + 1..];
    let id = rest[..rest.find('"')?].trim().to_string();
    if id.is_empty() {
        None
    } else {
        Some(id)
    }
}

/// `<root>/*/BanPickIllust/` 를 훑어 타 모드 아트 등록(2_Mod 티어).
fn scan_mod_art(root: &Path, map: &mut HashMap<String, &'static str>) {
    let rd = match std::fs::read_dir(root) {
        Ok(r) => r,
        Err(_) => return,
    };
    for e in rd.flatten() {
        let d = e.path();
        let art = d.join("BanPickIllust");
        if !art.is_dir() {
            continue;
        }
        if let Some(mod_id) = read_mod_id(&d) {
            scan_png(
                &art,
                map,
                |stem| format!("@/{stem}"),
                move |stem| format!("asset/{mod_id}/BanPickIllust/{stem}"),
            );
        }
    }
}

fn build() -> HashMap<String, &'static str> {
    let mut map: HashMap<String, &'static str> = HashMap::new();
    let dir = match crate::mod_dir() {
        Some(d) => d,
        None => return map,
    };
    let me = Path::new(&dir);

    // 우리 팩 — 진영 폴더별.
    for side in [BLUE, RED, RED_NOFLIP] {
        scan_png(
            &me.join("illust").join(side),
            &mut map,
            |stem| format!("{side}/{stem}"),
            move |stem| format!("{}{side}/{stem}", crate::ILLUST_KEY),
        );
    }
    // 구버전 평면 배치(illust/<champ>.png) 호환 — 진영 폴더가 없을 때만 의미 있음.
    scan_png(
        &me.join("illust"),
        &mut map,
        |stem| format!("flat/{stem}"),
        |stem| format!("{}{stem}", crate::ILLUST_KEY),
    );

    // 타 모드 아트.
    if let Some(g) = me.parent().and_then(|p| p.parent()) {
        scan_mod_art(&g.join("mods"), &mut map);
        if let Some(steamapps) = g.parent().and_then(|p| p.parent()) {
            scan_mod_art(
                &steamapps.join("workshop").join("content").join("3009300"),
                &mut map,
            );
        }
    }
    map
}

fn get(k: &str) -> Option<&'static str> {
    table().get(k).copied()
}

/// 챔피언 id + 진영 폴더 + 확대여부 → **실존하는** 에셋 키. 없으면 None(=바닐라 유지).
///
/// 폴백 순서: `<side>/<champ>-1`(확대 요청 시) → `<side>/<champ>` → 평면 배치 →
/// 타 모드 아트. 없는 키를 넣어 빈 그림이 뜨는 걸 막으려고 전부 실파일 존재 확인 기반.
/// 아트의 구도 방향. `red/` 만 좌우반전된 구도이고 나머지는 전부 원본 방향이다.
/// 타 모드 아트(`@`)와 평면배치(`flat`)는 진영 구분이 없어 원본 방향으로 본다.
fn mirrored(folder: &str) -> bool {
    folder == RED
}

/// 해당 진영에 쓸 아트를 찾는다. 반환 = `(에셋 키, 좌우반전 필요 여부)`.
///
/// ★한쪽 진영 아트만 있는 경우 반대쪽 것을 끌어와 **엔진 flip 으로 맞춰준다**.
/// 예) `blue/` 에만 있는 그림을 레드 슬롯(`red` = 반전 구도)에 쓸 때 flip_x=true.
/// 타 모드 아트는 진영 구분이 없으므로 레드에선 항상 반전된다.
/// 반대로 이미 반전된 `red/` 그림을 블루에 쓸 땐 되뒤집어 원래 방향으로 돌린다.
pub fn illust_key(champ_id: &str, side: &str, zoom: bool) -> Option<(&'static str, bool)> {
    if champ_id.is_empty() || champ_id.len() > 64 {
        return None;
    }
    // 요청 진영 → 폴더 후보(우선순위). 같은 구도를 먼저, 없으면 반전해서라도.
    let folders: [&str; 3] = match side {
        RED => [RED, BLUE, RED_NOFLIP],
        RED_NOFLIP => [RED_NOFLIP, BLUE, RED],
        _ => [BLUE, RED_NOFLIP, RED],
    };
    let want_mirror = mirrored(side);

    let try_get = |folder: &str| -> Option<(&'static str, bool)> {
        let flip = mirrored(folder) != want_mirror;
        if zoom {
            if let Some(k) = get(&format!("{folder}/{champ_id}-1")) {
                return Some((k, flip));
            }
        }
        get(&format!("{folder}/{champ_id}")).map(|k| (k, flip))
    };

    for f in folders {
        if let Some(r) = try_get(f) {
            return Some(r);
        }
    }
    // 진영 폴더가 없는 배치 — 평면 배치와 타 모드 아트는 원본 방향으로 간주.
    if zoom {
        if let Some(k) = get(&format!("flat/{champ_id}-1")) {
            return Some((k, want_mirror));
        }
    }
    get(&format!("flat/{champ_id}"))
        .or_else(|| get(&format!("@/{champ_id}")))
        .map(|k| (k, want_mirror))
}

/// 진단용 요약.
pub fn summary() -> String {
    let t = table();
    let mut n = HashMap::<&str, usize>::new();
    for k in t.keys() {
        let pre = k.split('/').next().unwrap_or("?");
        *n.entry(match pre {
            "blue" => "blue",
            "red" => "red",
            "red_noflip" => "red_noflip",
            "@" => "타모드아트",
            _ => "평면",
        })
        .or_insert(0) += 1;
    }
    let mut parts: Vec<String> = n.iter().map(|(k, v)| format!("{k}={v}")).collect();
    parts.sort();
    format!("에셋 등록 {}건 ({})", t.len(), parts.join(" "))
}
