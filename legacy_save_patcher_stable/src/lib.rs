//! legacy_save_patcher — stable ABI 포팅본 (2026-07-30).
//!
//! 원본: C:\Users\dev\Desktop\claude\tfm2\tfm2-mods-main\legacy_save_patcher (daram2, 클래식 SDK).
//! 이 모드는 게임 API를 전혀 쓰지 않는다(세이브 파일·mods.json·mod_info 파일 IO뿐) —
//! 유일한 변경점은 엔트리(declare_mod → declare_stable_mod)이며 run() 이하 로직은 원본과 바이트 단위 동일.
//! stable ABI라 게임이 업데이트돼도 재빌드 없이 계속 로드된다(abi_level 1 <= 게임 레벨인 한).

use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use mod_api_stable::{declare_stable_mod, LogLevel, StableHost, StableMod};

const MOD_ID: &str = "legacy_save_patcher";

const FAMILY: &[&str] = &[
    "banpick_view_plus",
    "custom_tier_assignment",
    "statistics_view_plus",
    "roster_view_plus",
    "coaching_staff_view_plus",
    "recruitment_view_plus",
    "facility_view_plus",
    "finance_view_plus",
    "training_view_plus",
];

fn game_dir() -> Option<PathBuf> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    extern "system" {
        fn GetModuleFileNameW(module: *mut core::ffi::c_void, filename: *mut u16, size: u32) -> u32;
    }
    let mut buf = [0u16; 1024];
    let len = unsafe { GetModuleFileNameW(core::ptr::null_mut(), buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if len == 0 || len >= buf.len() {
        return None;
    }
    PathBuf::from(OsString::from_wide(&buf[..len])).parent().map(|p| p.to_path_buf())
}

fn data_dir() -> PathBuf {
    if let Some(g) = game_dir() {
        let d = g.join("ModData");
        if std::fs::create_dir_all(&d).is_ok() {
            return d;
        }
    }
    std::env::temp_dir()
}

fn saves_dir() -> Option<PathBuf> {
    let appdata = std::env::var_os("APPDATA")?;
    Some(PathBuf::from(appdata).join("TeamSamoyed").join("TeamfightManager2").join("data"))
}

fn mods_json_path() -> Option<PathBuf> {
    Some(game_dir()?.join("config").join("game").join("mods.json"))
}

fn read_enabled_mods() -> Vec<String> {
    let Some(p) = mods_json_path() else { return Vec::new() };
    let Ok(t) = std::fs::read_to_string(&p) else { return Vec::new() };
    let Some(k) = t.find("\"enabled_mods\"") else { return Vec::new() };
    let rest = &t[k..];
    let Some(lb) = rest.find('[') else { return Vec::new() };
    let Some(rb) = rest[lb..].find(']') else { return Vec::new() };
    rest[lb + 1..lb + rb]
        .split(',')
        .map(|s| s.trim().trim_matches('"').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn read_mod_version(id: &str) -> String {
    let fallback = || "0.4.13".to_string();
    let Some(g) = game_dir() else { return fallback() };
    let p = g.join("mods").join(id).join("mod.mod_info");
    let Ok(t) = std::fs::read_to_string(&p) else { return fallback() };
    let Some(k) = t.find("\"version\"") else { return fallback() };
    let rest = &t[k + "\"version\"".len()..];
    let Some(c) = rest.find(':') else { return fallback() };
    let after = &rest[c + 1..];
    let Some(q1) = after.find('"') else { return fallback() };
    let after2 = &after[q1 + 1..];
    let Some(q2) = after2.find('"') else { return fallback() };
    let v = after2[..q2].trim().to_string();
    if v.is_empty() { fallback() } else { v }
}

fn gunzip(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut d = GzDecoder::new(data);
    let mut out = Vec::new();
    d.read_to_end(&mut out)?;
    Ok(out)
}

fn gzip(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut e = GzEncoder::new(Vec::new(), Compression::fast());
    e.write_all(data)?;
    e.finish()
}

fn rd_u64(d: &[u8], p: usize) -> Option<u64> {
    d.get(p..p + 8).map(|s| u64::from_le_bytes(s.try_into().unwrap()))
}

fn gzoffset(save: &[u8]) -> Option<usize> {
    let gzsize = rd_u64(save, 0x0d)? as usize;
    if gzsize > save.len() {
        return None;
    }
    let off = save.len() - gzsize;
    if save.get(off..off + 3)? == [0x1f, 0x8b, 0x08] {
        Some(off)
    } else {
        None
    }
}

fn parse_list(d: &[u8], start: usize, n: usize) -> Option<usize> {
    let mut p = start;
    for _ in 0..n {
        if rd_u64(d, p)? != 0 {
            return None;
        }
        let il = rd_u64(d, p + 8)? as usize;
        if il == 0 || il > 64 || p + 16 + il > d.len() {
            return None;
        }
        if !d[p + 16..p + 16 + il].iter().all(|b| b.is_ascii_graphic()) {
            return None;
        }
        let vp = p + 16 + il;
        let vl = rd_u64(d, vp)? as usize;
        if vl == 0 || vl > 32 || vp + 8 + vl > d.len() {
            return None;
        }
        if !d[vp + 8..vp + 8 + vl].iter().all(|b| b.is_ascii_graphic()) {
            return None;
        }
        p = vp + 8 + vl;
    }
    Some(p)
}

fn find_count_pos(d: &[u8]) -> Option<usize> {
    let mut anchor: Vec<u8> = vec![0u8; 8];
    anchor.extend_from_slice(&(4u64).to_le_bytes());
    anchor.extend_from_slice(b"base");
    let mut base_leads = Vec::new();
    let mut i = 0;
    while i + anchor.len() <= d.len() {
        if &d[i..i + anchor.len()] == anchor.as_slice() {
            base_leads.push(i);
        }
        i += 1;
    }
    for &base_lead in &base_leads {
        let lo = base_lead.saturating_sub(2048);
        for cpos in (lo..=base_lead.saturating_sub(8)).rev() {
            let n = match rd_u64(d, cpos) {
                Some(v) => v as usize,
                None => continue,
            };
            if n == 0 || n > 256 {
                continue;
            }
            if let Some(end) = parse_list(d, cpos + 8, n) {
                if cpos + 8 <= base_lead && base_lead < end {
                    return Some(cpos);
                }
            }
        }
    }
    None
}

fn list_ids(d: &[u8], cpos: usize) -> Vec<String> {
    let n = rd_u64(d, cpos).unwrap_or(0) as usize;
    let mut p = cpos + 8;
    let mut out = Vec::new();
    for _ in 0..n {
        let il = match rd_u64(d, p + 8) {
            Some(v) => v as usize,
            None => break,
        };
        if p + 16 + il > d.len() {
            break;
        }
        let id = String::from_utf8_lossy(&d[p + 16..p + 16 + il]).to_string();
        let vp = p + 16 + il;
        let vl = match rd_u64(d, vp) {
            Some(v) => v as usize,
            None => break,
        };
        out.push(id);
        p = vp + 8 + vl;
    }
    out
}

fn build_entry(id: &str, ver: &str) -> Vec<u8> {
    let mut e = Vec::new();
    e.extend_from_slice(&0u64.to_le_bytes());
    e.extend_from_slice(&(id.len() as u64).to_le_bytes());
    e.extend_from_slice(id.as_bytes());
    e.extend_from_slice(&(ver.len() as u64).to_le_bytes());
    e.extend_from_slice(ver.as_bytes());
    e
}

fn marker_path() -> PathBuf {
    data_dir().join("legacy_save_patcher.marker")
}

fn load_marker() -> std::collections::HashMap<String, String> {
    let mut m = std::collections::HashMap::new();
    if let Ok(t) = std::fs::read_to_string(marker_path()) {
        for line in t.lines() {
            if let Some((name, sig)) = line.rsplit_once('\t') {
                m.insert(name.to_string(), sig.to_string());
            }
        }
    }
    m
}

fn parse_marker_val(v: &str) -> (&str, Vec<&str>) {
    match v.split_once('|') {
        Some((sig, ids)) if !ids.is_empty() => (sig, ids.split(',').collect()),
        Some((sig, _)) => (sig, Vec::new()),
        None => (v, Vec::new()),
    }
}

fn save_marker(m: &std::collections::HashMap<String, String>) {
    let mut entries: Vec<(&String, &String)> = m.iter().collect();
    entries.sort();
    let mut out = String::from("# legacy_save_patcher: 처리한 세이브 (이름\\tsize:mtime|보유모드id목록). 지우면 다시 검사함.\n");
    for (name, sig) in entries {
        out.push_str(name);
        out.push('\t');
        out.push_str(sig);
        out.push('\n');
    }
    if std::fs::read_to_string(marker_path()).ok().as_deref() == Some(out.as_str()) {
        return;
    }
    let _ = std::fs::write(marker_path(), out);
}

fn file_sig(meta: &std::fs::Metadata) -> String {
    let mtime = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{}:{}", meta.len(), mtime)
}

fn cleanup_legacy_log() {
    let _ = std::fs::remove_file(data_dir().join("legacy_save_patcher.log"));
}

fn process_save(path: &std::path::Path, targets: &[(String, String)]) -> std::io::Result<Vec<String>> {
    let save = std::fs::read(path)?;
    let off = gzoffset(&save).ok_or_else(|| err("gzip offset/magic 불일치"))?;
    let main = gunzip(&save[off..])?;
    let cpos = find_count_pos(&main).ok_or_else(|| err("내부 모드목록을 찾지 못함"))?;
    let existing = list_ids(&main, cpos);

    let present_family: Vec<String> =
        existing.iter().filter(|e| FAMILY.contains(&e.as_str())).cloned().collect();

    let missing: Vec<&(String, String)> = targets.iter().filter(|(id, _)| !existing.iter().any(|e| e == id)).collect();
    if missing.is_empty() {
        return Ok(present_family);
    }

    let mut blob = Vec::new();
    for (id, ver) in &missing {
        blob.extend_from_slice(&build_entry(id, ver));
    }
    let ins = cpos + 8;
    let mut new_main = Vec::with_capacity(main.len() + blob.len());
    new_main.extend_from_slice(&main[..ins]);
    new_main.extend_from_slice(&blob);
    new_main.extend_from_slice(&main[ins..]);
    let n = rd_u64(&main, cpos).unwrap_or(0);
    new_main[cpos..cpos + 8].copy_from_slice(&(n + missing.len() as u64).to_le_bytes());

    let mut gz = gzip(&new_main)?;
    if gz.len() > 9 {
        gz[8] = 0x04;
        gz[9] = 0xff;
    }
    let mut out = Vec::with_capacity(off + gz.len());
    out.extend_from_slice(&save[..off]);
    out.extend_from_slice(&gz);
    out[0x0d..0x15].copy_from_slice(&(gz.len() as u64).to_le_bytes());

    backup(path)?;
    let tmp = path.with_extension("data.sctmp");
    let write_res = (|| -> std::io::Result<()> {
        use std::io::Write;
        let mut f = std::fs::File::create(&tmp)?;
        f.write_all(&out)?;
        f.sync_all()?;
        Ok(())
    })();
    if let Err(e) = write_res {
        let _ = std::fs::remove_file(&tmp);
        return Err(e);
    }
    if let Err(e) = std::fs::rename(&tmp, path) {
        let _ = std::fs::remove_file(&tmp);
        return Err(e);
    }

    let mut present = present_family;
    present.extend(missing.iter().map(|(id, _)| id.clone()));
    Ok(present)
}

const BACKUP_KEEP: usize = 3;

fn backup(path: &std::path::Path) -> std::io::Result<()> {
    let dir = path.parent().ok_or_else(|| err("save 경로에 부모 디렉터리 없음"))?;
    let bk = dir.join("_backups");
    std::fs::create_dir_all(&bk)?;
    let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("save");
    let prefix = format!("{name}.scbak_");
    let mut mine: Vec<(u64, PathBuf)> = std::fs::read_dir(&bk)
        .map(|rd| {
            rd.flatten()
                .filter_map(|e| {
                    let fname = e.file_name().to_string_lossy().to_string();
                    let idx: u64 = fname.strip_prefix(&prefix)?.parse().ok()?;
                    Some((idx, e.path()))
                })
                .collect()
        })
        .unwrap_or_default();
    let next = mine.iter().map(|(i, _)| *i).max().map(|m| m + 1).unwrap_or(0);
    let new_path = bk.join(format!("{prefix}{next}"));
    std::fs::copy(path, &new_path)?;
    mine.push((next, new_path));
    mine.sort_by(|a, b| b.0.cmp(&a.0));
    for (_, p) in mine.into_iter().skip(BACKUP_KEEP) {
        let _ = std::fs::remove_file(p);
    }
    Ok(())
}

fn err(msg: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, msg)
}

fn run() {
    cleanup_legacy_log();
    let enabled = read_enabled_mods();
    let targets: Vec<(String, String)> = FAMILY
        .iter()
        .filter(|f| enabled.iter().any(|e| e == *f))
        .map(|f| (f.to_string(), read_mod_version(f)))
        .collect();
    if targets.is_empty() {
        return;
    }
    let Some(dir) = saves_dir() else {
        return;
    };
    let Ok(rd) = std::fs::read_dir(&dir) else {
        return;
    };

    let ids: Vec<&str> = targets.iter().map(|(id, _)| id.as_str()).collect();

    let mut marker = load_marker();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    for ent in rd.flatten() {
        let path = ent.path();
        if path.extension().and_then(|e| e.to_str()) != Some("data") {
            continue;
        }
        let Ok(meta) = ent.metadata() else { continue };
        if !meta.is_file() {
            continue;
        }
        let name = ent.file_name().to_string_lossy().to_string();
        seen.insert(name.clone());
        let sig = file_sig(&meta);
        if let Some(v) = marker.get(&name) {
            let (msig, present) = parse_marker_val(v);
            if msig == sig && ids.iter().all(|t| present.contains(t)) {
                continue;
            }
        }

        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| process_save(&path, &targets)));
        let present = match res {
            Ok(Ok(present)) => present,
            Ok(Err(_)) | Err(_) => continue,
        };
        if let Ok(m2) = std::fs::metadata(&path) {
            let mut ps = present;
            ps.sort();
            ps.dedup();
            marker.insert(name, format!("{}|{}", file_sig(&m2), ps.join(",")));
        }
    }

    marker.retain(|k, _| seen.contains(k));
    save_marker(&marker);
}

fn init(host: &StableHost) -> StableMod {
    let v = host.game_version();
    host.log(
        LogLevel::Info,
        &format!("legacy_save_patcher (stable abi) loaded, game {}.{}.{}", v.major, v.minor, v.patch),
    );
    std::thread::spawn(|| {
        let _ = std::panic::catch_unwind(run);
    });
    StableMod::new(MOD_ID)
}

declare_stable_mod!(init);
