//! 기록 스레드만 파일과 GateStatus를 갱신한다. 예외 핸들러의 원자는 Relaxed 읽기만 한다.
use crate::{config::Config, patch::KnobStatus, platform, probe::{self, GateStatus}};
use std::fmt::Write as _;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering::Relaxed};
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub static FRAME: AtomicU64 = AtomicU64::new(0);
pub static MATCH_FRAMES: AtomicU64 = AtomicU64::new(0);
pub static SCENE: AtomicU64 = AtomicU64::new(u64::MAX);
static LOG_LOCK: Mutex<()> = Mutex::new(());

pub fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

pub fn event(dir: &Path, message: &str) {
    let _guard = LOG_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    // 임의 메시지의 개행도 이스케이프하여 한 사건을 한 줄에 보존한다.
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(dir.join("tactics_verify.log")) {
        let _ = writeln!(f, "{} {}", now_ms(), quote(message));
    }
}

fn quote(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""), '\\' => out.push_str("\\\\"),
            c if c < ' ' => { let _ = write!(out, "\\u{:04x}", c as u32); }
            c => out.push(c),
        }
    }
    out.push('"'); out
}

fn class_json(c: &platform::ClassStats) -> String {
    let cmp = c.cmp.iter().map(|v| v.load(Relaxed).to_string()).collect::<Vec<_>>().join(",");
    let hist = c.hist.iter().enumerate().filter_map(|(i, v)| {
        let count = v.load(Relaxed);
        (count != 0).then(|| format!("[{},{}]", i as i64 - 128, count))
    }).collect::<Vec<_>>().join(",");
    format!("{{\"hits\":{},\"cmp\":[{}],\"hist\":[{}]}}", c.hits.load(Relaxed), cmp, hist)
}

pub struct Session {
    pub dir: PathBuf,
    pub cfg: Config,
    pub exe_sha256: String,
    pub identity: bool,
    pub install_ms: u64,
    pub install_frame: u64,
    pub knobs: Vec<KnobStatus>,
    pub gates: Vec<GateStatus>,
}

impl Session {
    fn json(&self) -> String {
        let knobs = self.knobs.iter().map(|k| format!(
            "{{\"id\":{},\"value\":{},\"default\":{},\"result\":{}}}",
            quote(&k.id), k.value, k.default, quote(&k.result))).collect::<Vec<_>>().join(",");
        let gates = self.gates.iter().map(|g| {
            let stats = g.slot.and_then(|i| platform::SITE_GATES.get(i));
            let seen = stats.map_or(0, |s| s.seen.load(Relaxed));
            let samples = stats.map(|s| (0..seen.min(platform::SAMPLE_CAP as u64) as usize)
                .map(|i| format!("[{},{}]", s.sa[i].load(Relaxed), s.sb[i].load(Relaxed)))
                .collect::<Vec<_>>().join(",")).unwrap_or_default();
            let empty = "{\"hits\":0,\"cmp\":[0,0,0,0,0],\"hist\":[]}";
            format!("{{\"id\":{},\"rva\":{},\"install_result\":{},\"result\":{},\"armed\":{},\"patched\":{},\"b\":{},\"fg\":{},\"bg\":{},\"off_hits\":{},\"seen\":{},\"samples\":[{}]}}",
                quote(g.id), g.rva, quote(&g.install_result), quote(&g.result), g.armed, g.patched,
                g.b_value.map_or("null".into(), |v| v.to_string()),
                stats.map_or_else(|| empty.into(), |s| class_json(&s.cls[0])),
                stats.map_or_else(|| empty.into(), |s| class_json(&s.cls[1])),
                stats.map_or(0, |s| s.off_hits.load(Relaxed)), seen, samples)
        }).collect::<Vec<_>>().join(",");
        let scene = SCENE.load(Relaxed);
        format!("{{\"schema_version\":1,\"label\":{},\"exe_sha256\":{},\"exe_sha256_match\":{},\"install_unix_ms\":{},\"install_frame\":{},\"written_unix_ms\":{},\"frame\":{},\"scene\":{},\"match_frames\":{},\"sample_cap\":{},\"snapshot_consistency\":\"relaxed_nontransactional\",\"knobs\":[{}],\"gates\":[{}]}}\n",
            quote(&self.cfg.label), quote(&self.exe_sha256), self.identity, self.install_ms,
            self.install_frame, now_ms(), FRAME.load(Relaxed),
            if scene == u64::MAX { "null".into() } else { scene.to_string() },
            MATCH_FRAMES.load(Relaxed), platform::SAMPLE_CAP, knobs, gates)
    }

    fn retire(&mut self, all: bool) {
        for g in &mut self.gates {
            if !g.armed { continue; }
            let hits = g.slot.map_or(0, |i| {
                let s = &platform::SITE_GATES[i];
                s.cls[0].hits.load(Relaxed).saturating_add(s.cls[1].hits.load(Relaxed))
                    .saturating_add(s.off_hits.load(Relaxed))
            });
            if all || (self.cfg.max_hits != 0 && hits >= self.cfg.max_hits) {
                probe::disarm(g);
                if all && !g.armed { g.result = "종료: 무장 해제".into(); }
                event(&self.dir, &format!("gate {}: {}", g.id, g.result));
            }
        }
    }

    fn write(&self) -> std::io::Result<()> {
        atomic_write(&self.dir.join("tactics_verify_summary.json"), self.json().as_bytes())
    }
}

// Windows std::fs::rename은 대상 파일이 존재하면 실패할 수 있다. 삭제 간격 없이 교체한다.
#[link(name = "kernel32")]
extern "system" { fn MoveFileExW(from: *const u16, to: *const u16, flags: u32) -> i32; }
fn atomic_write(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    let tmp = path.with_extension("json.tmp");
    {
        let mut f = std::fs::File::create(&tmp)?;
        f.write_all(bytes)?; f.sync_all()?;
    }
    let from = tmp.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>();
    let to = path.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>();
    if unsafe { MoveFileExW(from.as_ptr(), to.as_ptr(), 1 | 8) } == 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}

pub struct Worker { stop: Arc<AtomicBool>, handle: Option<JoinHandle<()>> }
impl Drop for Worker {
    fn drop(&mut self) {
        self.stop.store(true, Relaxed);
        if let Some(h) = self.handle.take() { h.thread().unpark(); let _ = h.join(); }
    }
}

pub fn start(session: Session) -> Result<Worker, String> {
    for k in &session.knobs { event(&session.dir, &format!("knob {}: {}", k.id, k.result)); }
    for g in &session.gates { event(&session.dir, &format!("gate {}: {}", g.id, g.result)); }
    let stop = Arc::new(AtomicBool::new(false));
    let flag = stop.clone();
    let dir = session.dir.clone();
    // spawn 실패 시 설치된 게이트를 잃지 않도록 Session을 공유 보관한다.
    let pending = Arc::new(Mutex::new(session));
    let thread_pending = pending.clone();
    let handle = std::thread::Builder::new().name("tactics-summary".into()).spawn(move || {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let interval = Duration::from_millis(thread_pending.lock().unwrap_or_else(|e| e.into_inner()).cfg.summary_interval_ms);
            let mut last_error = None;
            loop {
                let stopping = flag.load(Relaxed);
                let mut session = thread_pending.lock().unwrap_or_else(|e| e.into_inner());
                session.retire(stopping);
                match session.write() {
                    Ok(()) => { last_error = None; }
                    Err(e) => {
                        let error = e.to_string();
                        if last_error.as_ref() != Some(&error) { event(&session.dir, &format!("summary IO: {error}")); }
                        last_error = Some(error);
                    }
                }
                drop(session);
                if stopping { break; }
                std::thread::park_timeout(interval);
            }
        }));
        if result.is_err() {
            event(&dir, "요약 스레드 panic: 기록 중단, 무장 해제 시도; 게시된 슬롯/스텁 보존");
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut s = thread_pending.lock().unwrap_or_else(|e| e.into_inner());
                s.retire(true); let _ = s.write();
            }));
        }
    }).map_err(|e| {
        let mut s = pending.lock().unwrap_or_else(|e| e.into_inner());
        s.retire(true); let _ = s.write();
        format!("요약 스레드 생성 실패: {e}")
    })?;
    Ok(Worker { stop, handle: Some(handle) })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn escape_and_atomic_replace_existing_file() {
        assert_eq!(quote("a\n\"\\"), "\"a\\u000a\\\"\\\\\"");
        let dir = std::env::temp_dir().join(format!("tactics-summary-{}-{}", std::process::id(), now_ms()));
        std::fs::create_dir_all(&dir).unwrap(); let file = dir.join("summary.json");
        atomic_write(&file, b"{\"n\":1}").unwrap();
        atomic_write(&file, b"{\"n\":2}").unwrap();
        assert_eq!(std::fs::read(&file).unwrap(), b"{\"n\":2}");
        assert!(!file.with_extension("json.tmp").exists());
        std::fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn rejected_gate_stays_in_summary_without_fake_samples() {
        let s = Session { dir: PathBuf::new(), cfg: Config::default(), exe_sha256: "test".into(), identity: false,
            install_ms: 123, install_frame: 60, knobs: vec![],
            gates: vec![GateStatus { id: "rejected", rva: 123, slot: None, entry: 0, first: 0,
                armed: false, patched: false, b_value: None, install_result: "거절".into(), result: "거절".into() }] };
        let json = s.json();
        assert!(json.contains("\"b\":null"));
        assert!(json.contains("\"samples\":[]"));
        assert!(json.contains("\"exe_sha256_match\":false"));
        assert!(json.contains("\"install_frame\":60"));
    }
    #[test]
    fn sparse_histogram_and_comparison_order() {
        use std::sync::atomic::AtomicU64;
        let c = platform::ClassStats { hits: AtomicU64::new(7),
            cmp: std::array::from_fn(|i| AtomicU64::new(i as u64)),
            hist: std::array::from_fn(|_| AtomicU64::new(0)) };
        c.hist[0].store(1, Relaxed); c.hist[128].store(4, Relaxed); c.hist[256].store(2, Relaxed);
        assert_eq!(class_json(&c), "{\"hits\":7,\"cmp\":[0,1,2,3,4],\"hist\":[[-128,1],[0,4],[128,2]]}");
    }
    #[test]
    fn worker_shutdown_flushes_and_joins_without_installing_gates() {
        let dir = std::env::temp_dir().join(format!("tactics-worker-{}-{}", std::process::id(), now_ms()));
        std::fs::create_dir_all(&dir).unwrap();
        let s = Session { dir: dir.clone(), cfg: Config::default(), exe_sha256: "fixture".into(),
            identity: false, install_ms: 1, install_frame: 60, knobs: vec![], gates: vec![] };
        let worker = start(s).unwrap();
        drop(worker);
        let text = std::fs::read_to_string(dir.join("tactics_verify_summary.json")).unwrap();
        assert!(text.contains("\"gates\":[]"));
        assert!(!dir.join("tactics_verify_summary.json.tmp").exists());
        std::fs::remove_dir_all(dir).unwrap();
    }
}
