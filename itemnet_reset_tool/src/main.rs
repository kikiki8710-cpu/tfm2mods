// TFM2 아이템 추천 신경망 "공장 초기화" 외부 툴
// -------------------------------------------------------------------------
// 유저가 세이브 파일(save_*.data)을 고르면, 그 안의 아이템 추천 신경망
// (LogisticSGDAgent, weight f32×16384)을 게임 공장 기본값으로 되돌린다.
// 신경망은 바닐라/모드 아이템이 SipHash 로 16384 해시버킷을 공유하므로
// "모드 점수만" 분리 초기화는 불가 → 신경망 전체를 공장값으로 복원한다.
//
// 세이브 포맷(실측):
//   [비압축 헤더] + [gzip 멤버 = 전체 Database 를 Spitz 직렬화]
//   헤더: "TFM2" + u8 ver(4) + u32 ?(0x2..) + u32 0x19f + u64 comp_len(@0x0d) + 날짜/팀명/모드목록/PNG로고
//   gzip 해제 스트림 안에서 item_network = [u64 len==16384][f32×16384][u64 fdim==1]
//   (base_network 등 다른 신경망과 구별: 이 시그니처는 세이브당 유일하게 매치됨 = 실측 검증)
// 편집: weight 65536 바이트만 공장값으로 교체 → re-gzip → 헤더의 comp_len 갱신 → 재조립.
#![windows_subsystem = "windows"]

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

// 게임 공장 기본망 파일을 exe 에 내장(포터블). 포맷 = [u64 16384][f32×16384][u32 fdim].
static FACTORY_FILE: &[u8] = include_bytes!("base_item_network.item_network");
const WCOUNT: usize = 16384;
const WBYTES: usize = WCOUNT * 4; // 65536

fn factory_weights() -> &'static [u8] {
    // 헤더 8바이트(u64 count) 스킵 → 다음 65536바이트가 weight
    &FACTORY_FILE[8..8 + WBYTES]
}

fn info(title: &str, msg: &str) {
    rfd::MessageDialog::new()
        .set_title(title)
        .set_description(msg)
        .set_level(rfd::MessageLevel::Info)
        .show();
}
fn err(msg: &str) {
    rfd::MessageDialog::new()
        .set_title("초기화 실패")
        .set_description(msg)
        .set_level(rfd::MessageLevel::Error)
        .show();
}

// gzip 멤버 시작 오프셋(1f 8b 08) 찾기
fn find_gzip(raw: &[u8]) -> Option<usize> {
    raw.windows(3).position(|w| w == [0x1f, 0x8b, 0x08])
}

fn decompress(comp: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut d = flate2::read::GzDecoder::new(comp);
    let mut out = Vec::new();
    d.read_to_end(&mut out)?;
    Ok(out)
}
fn compress(dec: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut e = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::new(6));
    e.write_all(dec)?;
    e.finish()
}

#[inline]
fn rd_u64(b: &[u8], off: usize) -> u64 {
    u64::from_le_bytes(b[off..off + 8].try_into().unwrap())
}

// 압축해제 스트림에서 item_network weight 블록의 "길이접두 시작 오프셋" 후보들을 찾는다.
// 시그니처: u64 len==16384 → 16384개 유한 f32(|w|<5) → u64 fdim==1.
fn locate_item_network(dec: &[u8]) -> Vec<usize> {
    let mut out = Vec::new();
    if dec.len() < 8 + WBYTES + 8 {
        return out;
    }
    let mut i = 0usize;
    let end = dec.len() - (8 + WBYTES + 8);
    while i <= end {
        if rd_u64(dec, i) == WCOUNT as u64 {
            let ws = i + 8;
            let mut ok = true;
            let mut maxabs = 0.0f32;
            let mut k = 0;
            while k < WCOUNT {
                let v = f32::from_le_bytes(dec[ws + k * 4..ws + k * 4 + 4].try_into().unwrap());
                if !v.is_finite() {
                    ok = false;
                    break;
                }
                let a = v.abs();
                if a > maxabs {
                    maxabs = a;
                }
                k += 1;
            }
            if ok && maxabs < 5.0 && rd_u64(dec, ws + WBYTES) == 1 {
                out.push(i);
            }
        }
        i += 1;
    }
    out
}

fn backup_path(src: &Path) -> PathBuf {
    let mut p = src.to_path_buf();
    let base = format!(
        "{}.itemnetbak",
        src.file_name().unwrap().to_string_lossy()
    );
    p.set_file_name(&base);
    let mut n = 1;
    while p.exists() {
        p.set_file_name(format!("{}.{}", base, n));
        n += 1;
    }
    p
}

fn default_save_dir() -> Option<PathBuf> {
    let appdata = std::env::var_os("APPDATA")?;
    let p = Path::new(&appdata)
        .join("TeamSamoyed")
        .join("TeamfightManager2")
        .join("data");
    if p.is_dir() {
        Some(p)
    } else {
        None
    }
}

// ghidra 확인(decode_single_save_payload @0x1f28210): 로더가 파일에서 읽는 헤더 필드는
// offset 0(매직)/4(버전 u8)/13(comp_len u64)/34+(가변스킵)뿐. offset 5-12(타임스탬프),
// offset 21(crc32)은 싱글세이브 로드 시 검증되지 않음 → comp_len(offset 13)만 갱신하면 됨.
const COMP_LEN_OFF: usize = 13;

// 세이브 raw 바이트를 받아 item_network 를 공장값으로 초기화한 새 raw 바이트를 만든다.
// 반환: (새 파일 바이트, 이전 weights[0..4], 공장 weights[0..4], 이미_공장값이었나)
fn reset_bytes(raw: &[u8]) -> Result<(Vec<u8>, [f32; 4], [f32; 4], bool), String> {
    if FACTORY_FILE.len() < 8 + WBYTES || rd_u64(FACTORY_FILE, 0) != WCOUNT as u64 {
        return Err("내장 공장망 파일이 손상되었습니다(빌드 오류).".into());
    }
    if raw.len() < 21 || &raw[0..4] != b"TFM2" {
        return Err("TFM2 세이브 파일이 아닙니다(매직 불일치).".into());
    }
    let gz = find_gzip(raw).ok_or("gzip 데이터 블록을 찾지 못했습니다(포맷 변경?).")?;
    let header = &raw[..gz];
    let comp = &raw[gz..];
    let orig_comp_len = comp.len() as u64;

    let dec = decompress(comp).map_err(|e| format!("gzip 해제 실패: {e}"))?;
    let hits = locate_item_network(&dec);
    if hits.is_empty() {
        return Err("세이브에서 아이템 신경망을 찾지 못했습니다.\n(게임 버전이 바뀌어 신경망 크기가 달라졌을 수 있습니다.)".into());
    }
    if hits.len() > 1 {
        return Err(format!(
            "아이템 신경망 후보가 {}개 발견되어 안전을 위해 중단합니다.\n(예상치 못한 포맷)",
            hits.len()
        ));
    }
    let h = hits[0];
    let ws = h + 8;

    let sample = |buf: &[u8], off: usize| -> [f32; 4] {
        let mut a = [0f32; 4];
        for k in 0..4 {
            a[k] = f32::from_le_bytes(buf[off + k * 4..off + k * 4 + 4].try_into().unwrap());
        }
        a
    };
    let cur = sample(&dec, ws);
    let already = &dec[ws..ws + WBYTES] == factory_weights();

    let mut dec2 = dec.clone();
    dec2[ws..ws + WBYTES].copy_from_slice(factory_weights());
    let newcomp = compress(&dec2).map_err(|e| format!("재압축 실패: {e}"))?;

    let mut out = Vec::with_capacity(header.len() + newcomp.len());
    out.extend_from_slice(header);
    out.extend_from_slice(&newcomp);

    // comp_len(u64) 갱신 — 우선 offset 13 직접, 값이 안 맞으면 앞 64B 탐색 폴백.
    let new_len = (newcomp.len() as u64).to_le_bytes();
    if header.len() >= COMP_LEN_OFF + 8 && rd_u64(&out, COMP_LEN_OFF) == orig_comp_len {
        out[COMP_LEN_OFF..COMP_LEN_OFF + 8].copy_from_slice(&new_len);
    } else {
        let want = orig_comp_len.to_le_bytes();
        let scan = header.len().min(64);
        let mut patched = false;
        let mut p = 0;
        while p + 8 <= scan {
            if out[p..p + 8] == want {
                out[p..p + 8].copy_from_slice(&new_len);
                patched = true;
                break;
            }
            p += 1;
        }
        if !patched {
            return Err("헤더의 압축길이(comp_len) 필드를 찾지 못했습니다(포맷 변경?).".into());
        }
    }

    let factory_sample = sample(factory_weights(), 0);
    Ok((out, cur, factory_sample, already))
}

fn run() -> Result<String, String> {
    // 1) 세이브 선택
    let mut dlg = rfd::FileDialog::new()
        .set_title("초기화할 세이브 파일을 선택하세요")
        .add_filter("TFM2 세이브 (*.data)", &["data"])
        .add_filter("모든 파일", &["*"]);
    if let Some(d) = default_save_dir() {
        dlg = dlg.set_directory(d);
    }
    let path = match dlg.pick_file() {
        Some(p) => p,
        None => return Ok("취소되었습니다.".into()),
    };

    let raw = std::fs::read(&path).map_err(|e| format!("파일 읽기 실패: {e}"))?;
    let (out, cur, factory_sample, already) = reset_bytes(&raw)?;

    // 백업 후 원본에 덮어쓰기
    let bak = backup_path(&path);
    std::fs::write(&bak, &raw).map_err(|e| format!("백업 생성 실패: {e}"))?;
    std::fs::write(&path, &out).map_err(|e| format!("저장 실패: {e}\n(백업: {})", bak.display()))?;

    Ok(format!(
        "✅ 아이템 신경망을 공장 기본값으로 초기화했습니다.\n\n\
         파일: {}\n\
         {}\
         이전 weights[0..4]: {:?}\n\
         공장 weights[0..4]: {:?}\n\n\
         원본 백업: {}\n\n\
         이제 게임에서 이 세이브를 불러오면 학습된 모드아이템 편향이 사라집니다.\n\
         (바닐라 개인학습도 함께 공장값으로 돌아가며, 이후 경기하며 다시 학습됩니다.)",
        path.display(),
        if already { "※ 이미 공장값 상태였지만 다시 기록했습니다.\n" } else { "" },
        cur,
        factory_sample,
        bak.display()
    ))
}

fn main() {
    // headless 테스트/CLI 모드: --test <src> <dst> (다이얼로그 없이 dst 로 출력만)
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 4 && args[1] == "--test" {
        match std::fs::read(&args[2]).map_err(|e| e.to_string()).and_then(|raw| reset_bytes(&raw)) {
            Ok((out, cur, fac, already)) => {
                let _ = std::fs::write(&args[3], &out);
                let _ = std::fs::write(
                    format!("{}.log", args[3]),
                    format!("OK cur={:?} factory={:?} already={} out_bytes={}\n", cur, fac, already, out.len()),
                );
            }
            Err(e) => {
                let _ = std::fs::write(format!("{}.log", args[3]), format!("ERR {e}\n"));
            }
        }
        return;
    }
    match run() {
        Ok(msg) => info("아이템 신경망 초기화", &msg),
        Err(e) => err(&e),
    }
}
