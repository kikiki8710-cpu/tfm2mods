//! 이벤트 트레이서 (v0.7.0, 2026-10-21). 판단 함수의 정해진 명령에서 값 여러 개를 한 줄씩 기록해
//! "선수별로 어떤 조건이 성립/불성립했는지"를 시간순으로 따라간다(예: 0.6.5 시작 직후 우물 대기).
//!
//! 관측점·스텁은 site_trace 것을 그대로 쓴다(원명령 실행 — 게임 판단 불변). 이 모듈은
//!  - VEH 안: 이벤트 표(EVENTS)의 필드 식을 레지스터·메모리(veh_read)로 계산해 고정 크기 링에 넣는다.
//!    할당·락·포맷·게임 CALL 없음. 읽기 실패는 fail 비트로만 남긴다.
//!  - 작업자 스레드(site_trace worker, 20ms 주기): 링을 비워 events_<run>.jsonl(로그 폴더)에 쓴다.
//!    keep=change 이벤트는 (team,slot)별로 지정 필드가 바뀔 때 + 하트비트 프레임마다만 쓴다.
//! 이벤트 표는 tools/gen_events.py 가 tools/events/*.txt 에서 만든다.
use crate::events_generated::EVENTS;
use crate::platform;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

pub const MAXF: usize = 12;
const RING: usize = 1 << 16;
const STACK: usize = 8;

#[derive(Clone, Copy, Debug)]
pub enum Step {
    /// CONTEXT 정수 레지스터(Rax=0..R15=15)
    Reg(u8),
    Imm(u64),
    /// 맨 위 값을 주소로 읽어 바꾼다(1/2/4/8바이트)
    Load(u8),
    Scale(u64),
    Mask(u64),
    Add,
}
pub struct Field { pub name: &'static str, pub prog: &'static [Step] }
pub struct Event {
    pub rva: u32, pub name: &'static str,
    /// 비어 있으면 매번 기록. 아니면 이 필드들이 (team,slot)별로 바뀔 때만 + heartbeat 프레임마다
    pub change: &'static [&'static str], pub heartbeat: u64,
    pub fields: &'static [Field],
}

struct Slot { seq: AtomicU64, ev: AtomicU32, tid: AtomicU32, frame: AtomicU64, fail: AtomicU32, vals: [AtomicU64; MAXF] }
impl Slot {
    const fn new() -> Self {
        Self { seq: AtomicU64::new(0), ev: AtomicU32::new(0), tid: AtomicU32::new(0), frame: AtomicU64::new(0),
            fail: AtomicU32::new(0), vals: [const { AtomicU64::new(0) }; MAXF] }
    }
}
static RINGBUF: [Slot; RING] = [const { Slot::new() }; RING];
static HEAD: AtomicU64 = AtomicU64::new(0);
static TAIL: AtomicU64 = AtomicU64::new(0);
static DROPPED: AtomicU64 = AtomicU64::new(0);
static WRITTEN: AtomicU64 = AtomicU64::new(0);
static CAPTURED: AtomicU64 = AtomicU64::new(0);
/// 메인 스레드 post_update 가 올리는 프레임 번호(이벤트 시각 기준)
pub static FRAME: AtomicU64 = AtomicU64::new(0);

#[link(name = "kernel32")]
extern "system" { fn GetCurrentThreadId() -> u32; }

/// rva → 이벤트 번호(EVENTS 는 rva 오름차순)
pub fn event_index(rva: u32) -> Option<usize> { EVENTS.binary_search_by_key(&rva, |e| e.rva).ok() }

#[inline(always)]
unsafe fn ctx_reg(ctx: *const u8, reg: u8) -> u64 { std::ptr::read_unaligned(ctx.add(0x78 + 8 * reg as usize) as *const u64) }

/// 필드 식 계산. VEH 안에서 부른다(할당 없음). 실패 = None.
unsafe fn eval(ctx: *const u8, prog: &[Step]) -> Option<u64> {
    let mut st = [0u64; STACK];
    let mut sp = 0usize;
    for s in prog {
        match *s {
            Step::Reg(r) => { if r > 15 || sp >= STACK { return None; } st[sp] = ctx_reg(ctx, r); sp += 1; }
            Step::Imm(v) => { if sp >= STACK { return None; } st[sp] = v; sp += 1; }
            Step::Load(size) => { if sp == 0 { return None; } st[sp - 1] = platform::veh_read(st[sp - 1] as usize, size)?; }
            Step::Scale(k) => { if sp == 0 { return None; } st[sp - 1] = st[sp - 1].wrapping_mul(k); }
            Step::Mask(m) => { if sp == 0 { return None; } st[sp - 1] &= m; }
            Step::Add => { if sp < 2 { return None; } sp -= 1; st[sp - 1] = st[sp - 1].wrapping_add(st[sp]); }
        }
    }
    if sp == 1 { Some(st[0]) } else { None }
}

/// VEH 진입점(site_trace::on_breakpoint 가 원명령 실행 전에 부른다). 링 한 칸을 채운다.
pub(crate) unsafe fn capture(ctx: *const u8, ev_idx: usize) {
    let Some(ev) = EVENTS.get(ev_idx) else { return; };
    CAPTURED.fetch_add(1, Ordering::Relaxed);
    let idx = HEAD.fetch_add(1, Ordering::AcqRel);
    let slot = &RINGBUF[(idx as usize) & (RING - 1)];
    // seqlock: 0 = 쓰는 중, idx+1 = 완료
    slot.seq.store(0, Ordering::Release);
    let mut fail = 0u32;
    for (k, f) in ev.fields.iter().enumerate().take(MAXF) {
        match eval(ctx, f.prog) {
            Some(v) => slot.vals[k].store(v, Ordering::Relaxed),
            None => { fail |= 1 << k; slot.vals[k].store(0, Ordering::Relaxed); }
        }
    }
    slot.ev.store(ev_idx as u32, Ordering::Relaxed);
    slot.tid.store(GetCurrentThreadId(), Ordering::Relaxed);
    slot.frame.store(FRAME.load(Ordering::Relaxed), Ordering::Relaxed);
    slot.fail.store(fail, Ordering::Relaxed);
    slot.seq.store(idx + 1, Ordering::Release);
}

/// keep=change 거르기 상태: (이벤트, team, slot) → (마지막 기록 키값들, 마지막 기록 프레임)
pub(crate) struct Drain { last: HashMap<(usize, u64, u64), (Vec<u64>, u64)> }
impl Drain { pub fn new() -> Self { Self { last: HashMap::new() } } }

fn field_pos(ev: &Event, name: &str) -> Option<usize> { ev.fields.iter().position(|f| f.name == name) }

/// 링을 비워 파일에 쓴다(작업자 스레드). 아직 쓰는 중인 칸을 만나면 다음 주기로 미룬다.
pub(crate) fn drain(dir: &Path, run_id: &str, d: &mut Drain) -> std::io::Result<()> {
    let head = HEAD.load(Ordering::Acquire);
    let mut tail = TAIL.load(Ordering::Relaxed);
    if head == tail { return Ok(()); }
    if head - tail > RING as u64 {
        DROPPED.fetch_add(head - tail - RING as u64, Ordering::Relaxed);
        tail = head - RING as u64;
    }
    let mut out = String::with_capacity(((head - tail) as usize).min(RING) * 160);
    while tail < head {
        let slot = &RINGBUF[(tail as usize) & (RING - 1)];
        let s1 = slot.seq.load(Ordering::Acquire);
        if s1 != tail + 1 {
            if s1 == 0 || s1 < tail + 1 { break; } // 쓰는 중 → 다음 주기
            DROPPED.fetch_add(1, Ordering::Relaxed); tail += 1; continue; // 덮어써짐
        }
        let ev_idx = slot.ev.load(Ordering::Relaxed) as usize;
        let tid = slot.tid.load(Ordering::Relaxed);
        let frame = slot.frame.load(Ordering::Relaxed);
        let fail = slot.fail.load(Ordering::Relaxed);
        let mut vals = [0u64; MAXF];
        for (k, v) in vals.iter_mut().enumerate() { *v = slot.vals[k].load(Ordering::Relaxed); }
        if slot.seq.load(Ordering::Acquire) != s1 { DROPPED.fetch_add(1, Ordering::Relaxed); tail += 1; continue; }
        tail += 1;
        let Some(ev) = EVENTS.get(ev_idx) else { continue; };
        let nf = ev.fields.len().min(MAXF);
        if !ev.change.is_empty() {
            let team = field_pos(ev, "team").map(|p| vals[p]).unwrap_or(u64::MAX);
            let slotv = field_pos(ev, "slot").map(|p| vals[p]).unwrap_or(u64::MAX);
            let mut key: Vec<u64> = ev.change.iter().map(|n| match *n {
                // full = hp >= maxhp (우물 대기 조건 b)
                "full" => match (field_pos(ev, "hp"), field_pos(ev, "maxhp")) { (Some(a), Some(b)) => (vals[a] >= vals[b]) as u64, _ => 2 },
                n => field_pos(ev, n).map(|p| vals[p]).unwrap_or(u64::MAX),
            }).collect();
            key.push(fail as u64);
            let ent = d.last.entry((ev_idx, team, slotv)).or_insert((Vec::new(), 0));
            let due = ev.heartbeat > 0 && frame >= ent.1 + ev.heartbeat;
            if ent.0 == key && !due { continue; }
            *ent = (key, frame);
        }
        out.push_str(&format!("{{\"ev\":\"{}\",\"f\":{},\"tid\":{}", ev.name, frame, tid));
        for k in 0..nf {
            if fail & (1 << k) != 0 { out.push_str(&format!(",\"{}\":null", ev.fields[k].name)); }
            else { out.push_str(&format!(",\"{}\":{}", ev.fields[k].name, vals[k])); }
        }
        out.push_str("}\n");
        WRITTEN.fetch_add(1, Ordering::Relaxed);
    }
    TAIL.store(tail, Ordering::Release);
    if out.is_empty() { return Ok(()); }
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(dir.join(format!("events_{run_id}.jsonl")))?;
    f.write_all(out.as_bytes())
}

/// 상태 요약(스냅샷 줄에 붙인다)
pub fn stats_json() -> String {
    format!("{{\"captured\":{},\"written\":{},\"dropped\":{},\"pending\":{}}}",
        CAPTURED.load(Ordering::Relaxed), WRITTEN.load(Ordering::Relaxed), DROPPED.load(Ordering::Relaxed),
        HEAD.load(Ordering::Relaxed).saturating_sub(TAIL.load(Ordering::Relaxed)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_programs_on_fake_context() {
        // CONTEXT: rbp(5) = 실제 버퍼 주소. [rbp+0x10] = 0x1234, [[rbp+0x18]+8] = 77, 인덱스 식 [[rbp+0x20]+[rbp+0x28]*8]
        let mut mem = vec![0u64; 16];
        let base = mem.as_ptr() as u64;
        mem[2] = 0x1234;                 // [rbp+0x10]
        mem[3] = base + 0x30;            // [rbp+0x18] → &mem[6]
        mem[7] = 77;                     // [&mem[6]+8]
        mem[4] = base + 0x40;            // [rbp+0x20] → 배열 &mem[8]
        mem[5] = 2;                      // [rbp+0x28] = 인덱스 2
        mem[10] = 0xabcd;                // 배열[2]
        let mut ctx = vec![0u8; 0x200];
        ctx[0x78 + 8 * 5..0x78 + 8 * 6].copy_from_slice(&base.to_le_bytes());
        ctx[0x78..0x80].copy_from_slice(&0x1ffu64.to_le_bytes()); // rax
        platform::init_safe_reader().unwrap();
        unsafe {
            let p = ctx.as_ptr();
            assert_eq!(eval(p, &[Step::Reg(5), Step::Imm(0x10), Step::Add, Step::Load(8)]), Some(0x1234));
            assert_eq!(eval(p, &[Step::Reg(5), Step::Imm(0x18), Step::Add, Step::Load(8), Step::Imm(8), Step::Add, Step::Load(8)]), Some(77));
            assert_eq!(eval(p, &[Step::Reg(5), Step::Imm(0x20), Step::Add, Step::Load(8),
                Step::Reg(5), Step::Imm(0x28), Step::Add, Step::Load(8), Step::Scale(8), Step::Add, Step::Load(8)]), Some(0xabcd));
            assert_eq!(eval(p, &[Step::Reg(0), Step::Mask(0xff)]), Some(0xff));
            // null 역참조 → None(크래시 없음)
            assert_eq!(eval(p, &[Step::Imm(0), Step::Load(8)]), None);
            assert_eq!(eval(p, &[Step::Add]), None);
        }
    }

    #[test]
    fn generated_events_sorted_and_bounded() {
        assert!(EVENTS.windows(2).all(|w| w[0].rva < w[1].rva));
        for e in EVENTS.iter() {
            assert!(e.fields.len() <= MAXF);
            for c in e.change { assert!(*c == "full" || e.fields.iter().any(|f| f.name == *c), "{} {}", e.name, c); }
        }
    }
}
