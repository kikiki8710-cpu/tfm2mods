//! tfm2_banpick_order — 밴픽 진행 순서(밴/픽 턴 시퀀스)를 config로 재정의.
//! ===========================================================================
//! RE 근거 = ANA\discovered-banpick-ai.md §16 (0.5.2 buildid 24310934).
//! 게임의 턴 순서는 "시퀀스 데이터"가 아니라 순수 함수 2개가 매 판정마다 계산:
//!   0x1cd9380 current_banpick_phase(&MatchSetInfo) -> BanpickPhase u8
//!   0x1d04120 phase_from(total, rule, ban_count) -> BanpickPhase u8 (룰별 점프테이블)
//! 두 함수를 전체 대체(완전 재구현 원칙 — 트램폴린/게임 헬퍼 FFI 호출 없음)해서
//! (rule, ban_count)에 맞는 커스텀 시퀀스가 있으면 그걸, 없으면 바닐라 로직
//! (밴 전부 선행 T1부터 교대 → .rdata 픽테이블)을 그대로 재현해 반환한다.
//!
//! 시퀀스 검증: B1/B2 개수 == 게임의 팀당 밴 수, P1/P2 개수 == 룰별 팀당 픽 수
//! (2v2=2 … 5v5=5)와 정확히 일치할 때만 적용 — 총길이가 게임 각처의
//! 2*ban+2*picks 재계산과 자동 일치하므로 종료판정(0xFF)도 어긋나지 않는다.
//!
//! 클라 라이브 적용기 0x11e2140은 밴/픽 분류를 인라인 보유(턴 함수 미호출) —
//! 밴픽 인터리브 시 오분류하므로 hooks::applier 쪽 shim으로 보정한다.
//!
//! 설정 = mods\tfm2_banpick_order\tfm2_banpick_order.cfg (없으면 자동 생성).
//! ===========================================================================

use mod_api::*;

mod config;
mod diag;
mod draft_ai;
mod hooks;

pub(crate) const MOD_ID: &str = "tfm2_banpick_order";

// build_inj.ps1 신원 검증용 — dll 안에 lib.rs 절대경로 문자열 필요(stale/타모드 차단).
#[no_mangle]
pub extern "C" fn tfm2_banpick_order_src_id() -> *const u8 {
    concat!(file!(), "\0").as_bytes().as_ptr()
}

// ── 게임 exe 기준 모드 폴더 (설치위치 무관 — 경로 하드코딩 금지 규칙) ──
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleFileNameW(h: usize, buf: *mut u16, n: u32) -> u32;
    fn GetModuleHandleW(n: *const u16) -> usize;
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

// ── 밴픽 화면 감지 ─────────────────────────────────────────────────────────
/// 밴픽 컨테이너 노드(레이아웃 banpick/layout.ui). 하나라도 있으면 밴픽 화면.
const BP_MARKERS: [&str; 3] = ["blue_picks", "red_picks", "banpick"];

fn node_exists(n: &Node, id: &str) -> bool {
    if n.id == id {
        return true;
    }
    n.child.iter().any(|c| node_exists(c, id))
}

fn is_banpick_screen(root: &Node) -> bool {
    BP_MARKERS.iter().any(|m| node_exists(root, m))
}

/// runner 의 데이터 포인터(= trait object 의 data ptr). item_tactics 검증 패턴.
unsafe fn runner_base(n: &Node) -> usize {
    let any: &dyn std::any::Any = n.runner.as_any();
    let parts: [usize; 2] =
        std::mem::transmute::<*const dyn std::any::Any, [usize; 2]>(any as *const dyn std::any::Any);
    parts[0]
}

/// 진단: 슬롯 노드의 runner 타입과 앞부분 바이트를 남긴다.
/// 칸 채움색은 자식이 아니라 **슬롯 자신(color_icon_button)의 btn.back_color** 이므로
/// (레이아웃 red_pick_slot.ui 실측) 그 필드를 찾기 위한 것.
fn dump_runner(n: &Node) -> String {
    let mut s = format!("[{}] type={} ", n.id, n.runner.type_name());
    unsafe {
        let b = runner_base(n);
        s.push_str(&format!("base={:#x}", b));
        if b > 0x10000 {
            for o in (0..0x140).step_by(8) {
                s.push_str(&format!(" +{o:#x}={:#x}", std::ptr::read_unaligned((b + o) as *const u64)));
            }
        }
    }
    s
}

/// 진단: 슬롯 하나의 **전체 하위 트리**(깊이 4)를 id+visible 로 펼친다.
/// 흰 상자가 `in_turn` 이 아님이 실증돼(강제 OFF 실험) 진짜 대상을 찾기 위한 것.
fn find_node_ref<'a>(n: &'a Node, id: &str) -> Option<&'a Node> {
    if n.id == id {
        return Some(n);
    }
    n.child.iter().find_map(|c| find_node_ref(c, id))
}

fn dump_deep(n: &Node, depth: usize, out: &mut String) {
    if depth > 4 || out.len() > 3000 {
        return;
    }
    out.push_str(&format!(
        "{}{}{} ",
        ".".repeat(depth),
        if n.id.is_empty() { "_" } else { n.id.as_str() },
        if n.visible { "+" } else { "-" }
    ));
    for c in n.child.iter() {
        dump_deep(c, depth + 1, out);
    }
}

/// 진단: 이름이 `want` 인 노드를 전부 찾아 (자식 id·visible 포함) 문자열로 덤프.
/// 흰칸(하이라이트)이 어느 노드/자식으로 표현되는지 특정하기 위한 것. debug=1 전용.
fn dump_group(n: &Node, want: &str, out: &mut Vec<String>) {
    if n.id == want {
        let mut kids = Vec::new();
        for c in n.child.iter() {
            let mut g = Vec::new();
            for cc in c.child.iter() {
                g.push(format!("{}{}", cc.id, if cc.visible { "+" } else { "-" }));
            }
            kids.push(format!(
                "{}{}{}",
                c.id,
                if c.visible { "+" } else { "-" },
                if g.is_empty() { String::new() } else { format!("({})", g.join(",")) }
            ));
        }
        out.push(format!("[{} v={} : {}]", n.id, n.visible, kids.join(" ")));
    }
    for c in n.child.iter() {
        dump_group(c, want, out);
    }
}

/// 진단: ui.root 트리의 노드 id를 depth 2까지 수집(밴픽 감지 실패 원인 추적용).
/// 0.5.3에서 IN_BANPICK이 한 번도 안 켜지는 현상 때문에 추가 — debug=1 에서만 호출.
fn dump_node_ids(n: &Node, depth: usize, out: &mut Vec<String>) {
    if out.len() >= 60 {
        return;
    }
    if !n.id.is_empty() {
        out.push(format!("{}{}", "  ".repeat(depth), n.id));
    }
    if depth < 2 {
        for c in n.child.iter() {
            dump_node_ids(c, depth + 1, out);
        }
    }
}

/// 트리 전체에서 마커 후보(밴픽/픽/밴 류 이름)를 찾아 경로 없이 나열.
fn scan_bp_like(n: &Node, out: &mut Vec<String>) {
    if out.len() >= 40 {
        return;
    }
    let id = n.id.as_str();
    if id.contains("pick") || id.contains("ban") || id.contains("match") {
        out.push(id.to_string());
    }
    for c in n.child.iter() {
        scan_bp_like(c, out);
    }
}

// ── 흰칸(in_turn) 직접 제어 ────────────────────────────────────────────────
// 하단 슬롯 렌더러는 씬을 안 읽는 순수 렌더러라(hooks::ui_turn_state 주석 참조) 게임의
// 흰칸 계산은 "밴이 항상 먼저 다 찬다"는 바닐라 전제를 벗어나지 못한다. 인터리브에서는
// 픽 차례에도 "다음 빈 밴 슬롯"이 계속 켜져 보인다. 순서를 아는 쪽은 모드이므로
// **모드가 in_turn 노드를 직접 켜고 끈다** — 노드 구조(실측):
//   bottom.<side>_side.bans.ban_slot_N.in_turn / <side>_picks.pick_slot_N.in_turn
// phase 규약: 0=T1픽 1=T2픽 2=T1밴 3=T2밴 0xff=종료. T1=blue, T2=red.
fn find_child<'a>(n: &'a mut Node, id: &str) -> Option<&'a mut Node> {
    n.child.iter_mut().find(|c| c.id == id)
}

fn find_path<'a>(root: &'a mut Node, path: &[&str]) -> Option<&'a mut Node> {
    fn rec<'b>(n: &'b mut Node, id: &str) -> Option<&'b mut Node> {
        if n.id == id {
            return Some(n);
        }
        for c in n.child.iter_mut() {
            if let Some(f) = rec(c, id) {
                return Some(f);
            }
        }
        None
    }
    let mut cur = rec(root, path[0])?;
    for seg in &path[1..] {
        cur = find_child(cur, seg)?;
    }
    Some(cur)
}

/// 슬롯 하나의 표시 상태를 정한다 — **`in_turn` 만 건드린다**.
/// ★게임 자신의 프레임에서 채록한 관용구(2026-07-30 로그 실측):
///   활성 픽 슬롯 = `wait+ in_turn+ done-` · 활성 밴 슬롯 = `icon+ ban_icon+ in_turn+`
/// 즉 대기 오버레이(`wait`)는 **켜진 채로 두고** in_turn 만 덧켜는 게 게임 규약이다.
/// (v2에서 wait 를 껐다가 흰 상자 자체가 사라졌음 — 재발 방지용 주석)
/// 슬롯 표시 = 3상태. 사진 실측(2026-07-30, 유저 제공):
///   현재 차례  → `in_turn`(+자식 `turn_outline`) = **팀색 채움**
///   다음 차례  → `wait` = **흰색 채움**
///   그 외      → 둘 다 꺼짐
#[derive(Clone, Copy, PartialEq)]
enum SlotMark {
    None,
    Current,
    Next,
}

/// 칸 채움색(2026-07-30 rdiff 실측 확정) = `ColorIconButtonRunner` **+0x208/+0x20c/+0x210
/// 의 RGB f32 3워드**가 라이브 렌더 소스 — 게임은 턴 전환 순간에만 쓰고 draw 가 매 프레임
/// 읽는다 ⟹ post_update 에서 매 프레임 덮어쓰면 모드가 이긴다(in_turn visible 과 동일 성립).
/// ⚠구 +0x13c(u32)는 오답: 생성 시 외부 memcpy 1회 초기화뿐(게임 콜사이트 rva 0x20733a3),
///   밴픽 중 불변(fill_obs 0건)·렌더 미사용 — 재시도 금지.
/// 팔레트(rdiff 실측 + champion_slot.ui L95/L124 짝):
///   기본 #1d1f2c(.ui back_color 일치) · 다음 차례 #4a4c56(실측) ·
///   현재 차례 = 팀색: red #b02e3a(실측) / blue #263cbf(자산 짝 — 인게임 검증 대상).
const FILL_RGB_OFF: usize = 0x208;
const FILL_DEFAULT: u32 = 0x1d1f2c;
const FILL_NEXT: u32 = 0x4a4c56;
const FILL_BLUE: u32 = 0x263cbf;
const FILL_RED: u32 = 0xb02e3a;

fn set_fill_rgb(slot: &Node, rgb: u32) {
    // drbp 프로브 중엔 게임 자신의 write 만 관찰해야 하므로 우리 쓰기를 중단
    // (자기 write 가 BP 를 계속 때리고, 게임 값을 덮어 관찰을 오염시킨다).
    if config::get().drbp {
        return;
    }
    unsafe {
        let b = runner_base(slot);
        if b > 0x10000 && slot.runner.type_name().contains("ColorIconButtonRunner") {
            for (i, ch) in [(rgb >> 16) & 0xff, (rgb >> 8) & 0xff, rgb & 0xff]
                .iter()
                .enumerate()
            {
                // n/255.0 은 게임 기록값과 비트 동일(29/255 = 0x3de8e8e9 실측 일치).
                let v = *ch as f32 / 255.0;
                let p = (b + FILL_RGB_OFF + i * 4) as *mut f32;
                if std::ptr::read_unaligned(p).to_bits() != v.to_bits() {
                    std::ptr::write_unaligned(p, v);
                }
            }
        }
    }
}

fn set_slot_mark(slot: &mut Node, mark: SlotMark, team_fill: u32) {
    let cur = mark == SlotMark::Current;
    for c in slot.child.iter_mut() {
        match c.id.as_str() {
            "in_turn" => {
                if c.visible != cur {
                    c.visible = cur;
                }
                // ★큰 카드의 테두리는 `in_turn` 의 자식이라 부모만 꺼도 남는다(실측).
                for g in c.child.iter_mut() {
                    if g.id == "turn_outline" && g.visible != cur {
                        g.visible = cur;
                    }
                }
            }
            // ⛔`wait` 는 **빈 카드의 내용 컨테이너**(bar·position·name 이 그 안에 있다).
            // 끄면 선수 카드가 통째로 사라진다(2026-07-30 실사고) ⟹ 절대 건드리지 않는다.
            // "다음 차례 흰색"은 wait 가 아니라 슬롯 버튼 배경색이 담당한다.
            _ => {}
            _ => {}
        }
    }
    // 칸 채움: 현재 차례 = 팀색 · 다음 차례 = 밝은 회색 · 그 외 = 기본(게임 관용구 그대로).
    let rgb = match mark {
        SlotMark::Current => team_fill,
        SlotMark::Next => FILL_NEXT,
        SlotMark::None => FILL_DEFAULT,
    };
    set_fill_rgb(slot, rgb);
}

fn set_slot(slot: &mut Node, active: bool) {
    if let Some(t) = slot.child.iter_mut().find(|c| c.id == "in_turn") {
        if t.visible != active {
            t.visible = active;
        }
        // ★`in_turn` 은 카드 오른쪽의 얇은 띠일 뿐이고, **큰 카드의 흰 테두리는 그 자식
        // `turn_outline`** 이다(2026-07-30 깊이덤프 실측). 부모를 꺼도 이 자식이 켜진 채
        // 남아 흰 상자가 유지되므로 **반드시 같이** 제어해야 한다.
        for c in t.child.iter_mut() {
            if c.id == "turn_outline" && c.visible != active {
                c.visible = active;
            }
        }
    }
}

/// 그룹 하위에서 채워진 슬롯 수(픽=done, 밴=icon)를 센다 — 팀↔좌우 매핑 판정용.
fn filled_count(group: &Node, marker: &str) -> u64 {
    group
        .child
        .iter()
        .filter(|s| {
            s.child
                .iter()
                .any(|c| c.id == marker && c.visible)
        })
        .count() as u64
}

/// 그룹의 슬롯들에 흰칸을 적용. want=None 이면 전부 끔. team_fill = 그룹의 팀색.
fn apply_group(group: &mut Node, prefix: &str, want: Option<u64>, want_next: Option<u64>, team_fill: u32) {
    for slot in group.child.iter_mut() {
        let idx = slot
            .id
            .strip_prefix(prefix)
            .and_then(|t| t.parse::<u64>().ok());
        let mark = if matches!((idx, want), (Some(i), Some(w)) if i == w) {
            SlotMark::Current
        } else if matches!((idx, want_next), (Some(i), Some(w)) if i == w) {
            SlotMark::Next
        } else {
            SlotMark::None
        };
        set_slot_mark(slot, mark, team_fill);
    }
}

/// ⬜칸 채움색 기록자 포착(MIGRATION §7.3 §14.5(6)) — debug=1 && drbp=1 전용.
/// 정적 후보 5종·phase 훅 6종이 전부 오답이라(§14.5(3)(4)) 런타임 DR write BP 로
/// "누가 픽슬롯 runner+0x13c 를 쓰는가"를 직접 잡는다.
///   ① DR0..3 = blue/red pick_slot_0·1 의 runner+0x208(채움 R float, 메인 스레드에 arm)
///   ② 러너 블록(≤0x400B) 프레임 diff — 턴 전환과 함께 변하는 필드 탐색
///      (라운드 2 성과: +0x208/20c/210 = 채움 RGB f32 확정. +0x13c 는 오답 — 생성 1회뿐)
///   ③ 120프레임마다 히트 rip 통계 → order_log
fn drbp_probe(root: &Node, in_bp: bool) {
    use std::sync::atomic::{AtomicU32, Ordering};
    if !in_bp {
        unsafe { diag::arm_watch(&[]) };
        return;
    }
    let mut addrs: Vec<usize> = Vec::new();
    let mut obs: Vec<(String, usize)> = Vec::new();
    for (grp, tag) in [("blue_picks", "b"), ("red_picks", "r")] {
        if let Some(g) = find_node_ref(root, grp) {
            for s in g.child.iter() {
                let Some(ix) = s.id.strip_prefix("pick_slot_") else { continue };
                unsafe {
                    let b = runner_base(s);
                    if b > 0x10000 && s.runner.type_name().contains("ColorIconButtonRunner") {
                        let a = b + FILL_RGB_OFF;
                        if addrs.len() < 4 && (ix == "0" || ix == "1") {
                            addrs.push(a);
                        }
                        obs.push((format!("{tag}{ix}"), a));
                    }
                }
            }
        }
    }
    unsafe { diag::arm_watch(&addrs) };
    // ② 러너 블록 스냅샷 diff(라운드 2, 07-30) — 라운드 1 실측: 게임은 밴픽 내내
    //   +0x13c 를 한 번도 안 씀(fill_obs 0건) = 그 필드는 렌더 소스가 아니다.
    //   ⟹ 턴 전환과 "함께" 변하는 러너 필드를 전 픽슬롯 × ≤0x400B 프레임 diff 로 찾는다.
    //   같은 (슬롯,오프셋)은 4회까지만 로그(애니메이션 카운터류 노이즈 억제).
    {
        use std::collections::HashMap;
        use std::sync::{Mutex, OnceLock};
        const SNAP: usize = 0x400;
        struct Snap {
            addr: usize,
            cap: usize,
            buf: Vec<u8>,
        }
        static SNAPS: OnceLock<Mutex<(HashMap<String, Snap>, HashMap<(String, usize), u32>)>> =
            OnceLock::new();
        let mx = SNAPS.get_or_init(|| Mutex::new((HashMap::new(), HashMap::new())));
        let mut g = mx.lock().unwrap_or_else(|e| e.into_inner());
        let (snaps, counts) = &mut *g;
        for (tag, fill_addr) in obs.iter() {
            let base = fill_addr - FILL_RGB_OFF; // runner_base 복원
            let cap = unsafe { diag::region_cap(base, SNAP) };
            if cap < 0x20 {
                continue;
            }
            let mut cur = vec![0u8; cap];
            unsafe {
                std::ptr::copy_nonoverlapping(base as *const u8, cur.as_mut_ptr(), cap)
            };
            match snaps.get_mut(tag.as_str()) {
                Some(s) if s.addr == base && s.cap == cap => {
                    let n = cap / 4;
                    for k in 0..n {
                        let o = k * 4;
                        let ov = u32::from_le_bytes(s.buf[o..o + 4].try_into().unwrap());
                        let nv = u32::from_le_bytes(cur[o..o + 4].try_into().unwrap());
                        if ov != nv {
                            let c = counts.entry((tag.clone(), o)).or_insert(0);
                            *c += 1;
                            if *c <= 4 {
                                config::dlog(&format!(
                                    "rdiff {tag} +{o:#x}: {ov:#010x} -> {nv:#010x} n={c}"
                                ));
                            }
                        }
                    }
                    s.buf = cur;
                }
                _ => {
                    config::dlog(&format!("rdiff {tag}: snap start base={base:#x} cap={cap:#x}"));
                    snaps.insert(
                        tag.clone(),
                        Snap { addr: base, cap, buf: cur },
                    );
                }
            }
        }
        // 억제된 노이즈 오프셋 요약(어떤 걸 숨겼는지 알아야 오판 안 함)
        static NS: AtomicU32 = AtomicU32::new(0);
        if NS.fetch_add(1, Ordering::Relaxed) % 300 == 299 {
            let mut noisy: Vec<String> = counts
                .iter()
                .filter(|(_, &c)| c > 4)
                .map(|((t, o), c)| format!("{t}+{o:#x}x{c}"))
                .collect();
            noisy.sort();
            if !noisy.is_empty() {
                config::dlog(&format!("rdiff noisy: {}", noisy.join(" ")));
            }
        }
    }
    // ③ 주기 통계
    static N: AtomicU32 = AtomicU32::new(0);
    if N.fetch_add(1, Ordering::Relaxed) % 120 == 0 {
        let base = unsafe { GetModuleHandleW(core::ptr::null()) } as usize;
        let st = diag::drbp_stats();
        let s: Vec<String> = st
            .iter()
            .map(|&(r, c)| {
                let rv = r.wrapping_sub(base);
                if base != 0 && rv < 0x4400000 {
                    format!("g{rv:#x}x{c}")
                } else {
                    format!("{r:#x}x{c}")
                }
            })
            .collect();
        config::dlog(&format!(
            "drbp armed={:x?} hits=[{}]",
            addrs,
            s.join(" ")
        ));
    }
}

/// 매 프레임: 커스텀 순서 기준으로 흰칸을 정정한다.
///
/// ★팀↔좌우 매핑은 고정하지 않는다. phase 의 팀비트(T1/T2)는 **경기별 side 규약**이라
/// blue=T1 이 항상 성립하지 않는다(§11). 그래서 화면에 이미 채워진 슬롯 수를 세서
/// (blue 채운수, red 채운수) 를 (t1, t2) 와 대조해 어느 쪽이 T1 인지 판정한다.
/// 양쪽 수가 같아 구분이 안 되는 순간엔 직전 판정을 유지한다.
fn apply_turn_highlight(root: &mut Node) {
    use std::sync::atomic::{AtomicU8, Ordering};
    /// 0=미정, 1=blue가 T1, 2=red가 T1
    static SIDE: AtomicU8 = AtomicU8::new(0);

    // ⚠진단: 전부 끄기 모드 — 흰 상자가 사라지는지로 "노드 쓰기가 렌더에 닿는가"를 판정.
    if config::get().hl_force_off {
        for path in [
            &["bottom", "blue_side", "bans"][..],
            &["bottom", "red_side", "bans"][..],
            &["blue_picks"][..],
            &["red_picks"][..],
        ] {
            if let Some(g) = find_path(root, path) {
                for slot in g.child.iter_mut() {
                    set_slot(slot, false);
                }
            }
        }
        return;
    }
    let Some((phase, t1b, t2b, t1p, t2p, next_phase)) = hooks::ui_turn_state() else {
        return;
    };
    // 1) 좌우 매핑 판정 (픽 수 → 안 되면 밴 수)
    let bp = find_path(root, &["blue_picks"]).map(|g| filled_count(g, "done"));
    let rp = find_path(root, &["red_picks"]).map(|g| filled_count(g, "done"));
    if let (Some(b), Some(r)) = (bp, rp) {
        if b != r {
            SIDE.store(if b == t1p { 1 } else { 2 }, Ordering::Relaxed);
        }
    }
    if SIDE.load(Ordering::Relaxed) == 0 {
        let bb = find_path(root, &["bottom", "blue_side", "bans"]).map(|g| filled_count(g, "icon"));
        let rb = find_path(root, &["bottom", "red_side", "bans"]).map(|g| filled_count(g, "icon"));
        if let (Some(b), Some(r)) = (bb, rb) {
            if b != r {
                SIDE.store(if b == t1b { 1 } else { 2 }, Ordering::Relaxed);
            }
        }
    }
    let blue_is_t1 = SIDE.load(Ordering::Relaxed) != 2; // 미정이면 blue=T1 가정

    // 2) phase 규약: 0=T1픽 1=T2픽 2=T1밴 3=T2밴 0xff=종료
    let (pick_t1, pick_t2) = if blue_is_t1 { (0u8, 1u8) } else { (1u8, 0u8) };
    let (ban_t1, ban_t2) = if blue_is_t1 { (2u8, 3u8) } else { (3u8, 2u8) };
    let (blue_pick_n, red_pick_n) = if blue_is_t1 { (t1p, t2p) } else { (t2p, t1p) };
    let (blue_ban_n, red_ban_n) = if blue_is_t1 { (t1b, t2b) } else { (t2b, t1b) };

    let groups: [(&[&str], &str, u8, u64); 4] = [
        (&["bottom", "blue_side", "bans"], "ban_slot_", ban_t1, blue_ban_n),
        (&["bottom", "red_side", "bans"], "ban_slot_", ban_t2, red_ban_n),
        (&["blue_picks"], "pick_slot_", pick_t1, blue_pick_n),
        (&["red_picks"], "pick_slot_", pick_t2, red_pick_n),
    ];
    // ★훅 O 용 슬롯 표: (슬롯 노드 포인터, 현재 차례인가). 게임의 색 적용기가 이 표를 보고
    //   param_7(색 세트 선택)을 정하게 된다 — 색값·노드 구조를 추측할 필요가 없다.
    let mut tbl: Vec<(usize, bool)> = Vec::with_capacity(hooks::SLOT_TBL_N);
    for (path, prefix, want_phase, next_idx) in groups {
        if let Some(g) = find_path(root, path) {
            let is_cur = phase == want_phase;
            for slot in g.child.iter() {
                let idx = slot
                    .id
                    .strip_prefix(prefix)
                    .and_then(|t| t.parse::<u64>().ok());
                let cur = is_cur && idx == Some(next_idx);
                tbl.push((slot as *const Node as usize, cur));
            }
        }
    }
    hooks::set_slot_table(&tbl);

    for (path, prefix, want_phase, next_idx) in groups {
        if let Some(g) = find_path(root, path) {
            let is_cur = phase == want_phase;
            let want = if is_cur { Some(next_idx) } else { None };
            // "다음 차례" 밝은 회색: **같은 팀이 연속으로 행동할 때만**(유저 선택, 07-30).
            // 커스텀 순서에선 상대 칸에 다음 표시가 뜨는 게 어색하다는 피드백 —
            // 팀비트(phase&1)가 현재와 같을 때만 next_phase 그룹에 표시한다.
            let same_team_next = next_phase != 0xff && (next_phase & 1) == (phase & 1);
            let want_next = if same_team_next && next_phase == want_phase {
                Some(if is_cur { next_idx + 1 } else { next_idx })
            } else {
                None
            };
            // 채움 팀색 = 화면 좌우(그룹) 소속 — 게임도 빨간 슬롯엔 빨강을 칠한다(실측).
            let team_fill = if path.iter().any(|s| s.contains("blue")) {
                FILL_BLUE
            } else {
                FILL_RED
            };
            apply_group(g, prefix, want, want_next, team_fill);
        }
    }
    if config::get().debug {
        static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        if N.fetch_add(1, Ordering::Relaxed) % 60 == 0 {
            // 적용 "직후" 상태를 다시 읽어 남긴다 — 우리 쓰기가 실제로 트리에 반영되는지,
            // 그리고 다음 프레임의 `node:` 줄(=게임이 다시 계산한 값)과 비교하기 위함.
            let mut after = String::new();
            for (path, prefix) in [
                (&["bottom", "blue_side", "bans"][..], "ban_slot_"),
                (&["bottom", "red_side", "bans"][..], "ban_slot_"),
                (&["blue_picks"][..], "pick_slot_"),
                (&["red_picks"][..], "pick_slot_"),
            ] {
                let on = find_path(root, path).map(|g| {
                    g.child
                        .iter()
                        .filter(|s| {
                            s.child.iter().any(|c| c.id == "in_turn" && c.visible)
                        })
                        .filter_map(|s| s.id.strip_prefix(prefix).map(|t| t.to_string()))
                        .collect::<Vec<_>>()
                        .join(",")
                });
                after.push_str(&format!("{:?} ", on));
            }
            config::dlog(&format!(
                "hl: phase={phase:#x} blue_is_t1={blue_is_t1} bans={t1b}/{t2b} picks={t1p}/{t2p}                  filled(bp,rp)={bp:?},{rp:?} after=[{after}]"
            ));
        }
    }
}

// ── 진입 ──────────────────────────────────────────────────────────────────
struct BanpickOrderExt;

impl ModExtension for BanpickOrderExt {
    fn post_update(&self, scene: &mut Scene, ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // 챔프 능력치 테이블 1회 캡처(경기 중 불변) — 우리 밴픽 AI 판단 재료.
            // ChampionInfoSheet의 순서가 ctx candidate 인덱스와 같다는 전제(debug로 검증).
            if config::get().ai_ban_context {
                if let Scene::InGame { data } = scene {
                    let db = data.db();
                    draft_ai::capture_table(|t| {
                        // 챔프 목록 = available_champions(id 문자열) ∪ 모드 챔프.
                        // ★순서가 곧 ctx candidate 인덱스라는 전제 — debug 로그로 검증한다.
                        let mut ids: Vec<String> = db.available_champions.clone();
                        for e in &db.champion_info_sheet.mod_champions {
                            if !ids.iter().any(|x| x == &e.id) {
                                ids.push(e.id.clone());
                            }
                        }
                        for id in &ids {
                            let a = match db.champion_info(id) {
                                Some(c) => {
                                    let s = c.stat();
                                    draft_ai::Attr {
                                        ad: s.attack as f32,
                                        ap: s.magic_power as f32,
                                        hp: s.hp as f32,
                                        def: s.defence as f32,
                                        mr: s.magic_resistance as f32,
                                        range: 0.0,
                                    }
                                }
                                None => match db
                                    .champion_info_sheet
                                    .mod_champions
                                    .iter()
                                    .find(|e| &e.id == id)
                                {
                                    Some(e) => {
                                        let s = e.stat();
                                        draft_ai::Attr {
                                            ad: s.attack as f32,
                                            ap: s.magic_power as f32,
                                            hp: s.hp as f32,
                                            def: s.defence as f32,
                                            mr: s.magic_resistance as f32,
                                            range: 0.0,
                                        }
                                    }
                                    None => continue,
                                },
                            };
                            t.push((id.clone(), a));
                        }
                    });
                }
            }
            // 진단(crash_log.txt: panic 위치·상태 덤프)은 debug=1 일 때만 설치.
            if config::get().debug {
                let base = unsafe { GetModuleHandleW(core::ptr::null()) };
                if base != 0 {
                    if let Some(dir) = mod_dir() {
                        diag::install(base, &dir);
                    }
                }
            }
            if !config::get().enabled {
                hooks::set_in_banpick(false);
                diag::CTX_IN_BANPICK.store(0, std::sync::atomic::Ordering::Relaxed);
                return;
            }
            // ★컨텍스트 게이트: 밴픽 화면일 때만 커스텀 순서 적용(경기 진행/시뮬 중엔
            //   원본 폴백 — 로스터 오염 크래시 방지, hooks.rs IN_BANPICK 주석 참조).
            let in_bp = is_banpick_screen(&ui.root);
            // ⚠진단(0.5.3): IN_BANPICK이 안 켜지는 원인 추적 — 상태가 바뀌거나 주기적으로
            //   ui.root 노드 id를 덤프한다. 정상화되면 이 블록은 제거할 것.
            if config::get().debug {
                use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
                static LAST: AtomicBool = AtomicBool::new(false);
                static N: AtomicU64 = AtomicU64::new(0);
                let f = N.fetch_add(1, Ordering::Relaxed);
                if LAST.swap(in_bp, Ordering::Relaxed) != in_bp || (in_bp && f % 60 == 0) {
                    let mut ids = Vec::new();
                    dump_node_ids(&ui.root, 0, &mut ids);
                    let mut like = Vec::new();
                    scan_bp_like(&ui.root, &mut like);
                    // ★러너 덤프: 칸 채움색(btn.back_color) 필드 위치를 찾기 위해
                    //   현재 차례 슬롯과 아닌 슬롯의 runner 바이트를 비교한다.
                    for (grp, sl) in [("red_picks", "pick_slot_3"), ("red_picks", "pick_slot_4")] {
                        if let Some(g) = find_node_ref(&ui.root, grp) {
                            if let Some(s) = g.child.iter().find(|c| c.id == sl) {
                                config::dlog(&format!("runner {grp}/{sl}: {}", dump_runner(s)));
                            }
                        }
                    }
                    // ★깊이 덤프: 흰 상자의 진짜 노드를 찾기 위해 슬롯 하나를 통째로 펼친다.
                    for (grp, sl) in [
                        ("blue_picks", "pick_slot_3"),
                        ("red_picks", "pick_slot_3"),
                        ("bans", "ban_slot_3"),
                    ] {
                        let mut deep = String::new();
                        if let Some(g) = find_node_ref(&ui.root, grp) {
                            if let Some(s) = g.child.iter().find(|c| c.id == sl) {
                                dump_deep(s, 0, &mut deep);
                            }
                        }
                        if !deep.is_empty() {
                            config::dlog(&format!("deep {grp}/{sl}: {deep}"));
                        }
                    }
                    let mut bans = Vec::new();
                    dump_group(&ui.root, "bans", &mut bans);
                    let mut picks = Vec::new();
                    dump_group(&ui.root, "blue_picks", &mut picks);
                    dump_group(&ui.root, "red_picks", &mut picks);
                    config::dlog(&format!("node: bans {}", bans.join(" ")));
                    config::dlog(&format!("node: picks {}", picks.join(" ")));
                    config::dlog(&format!(
                        "ui: in_bp={in_bp} root_id='{}' children={} depth2=[{}] bp_like=[{}]",
                        ui.root.id,
                        ui.root.child.len(),
                        ids.join(" | "),
                        like.join(",")
                    ));
                }
            }
            hooks::set_in_banpick(in_bp);
            diag::CTX_IN_BANPICK.store(in_bp as u32, std::sync::atomic::Ordering::Relaxed);
            hooks::tick();
            // ⬜칸 채움색 기록자 포착(§14.5(6)) — debug=1(VEH 설치) && drbp=1 전용.
            if config::get().debug && config::get().drbp {
                drbp_probe(&ui.root, in_bp);
            }
            // ★흰칸 정정은 훅 설치·상태 갱신 뒤에(같은 프레임의 최종 상태를 쓴다).
            if in_bp {
                apply_turn_highlight(&mut ui.root);
            }
        }));
    }
}

// ── 우리만의 밴픽 AI: 밴할 때 이미 확정된 픽을 함께 본다 ─────────────────────
// 게임 네이티브 밴 AI는 픽을 전혀 안 본다(RE 확정) → 공식 확장점(ModDraftScoreHook)에
// 우리 보정을 얹는다. 픽 정보는 ctx가 안 주므로 draft_ai의 스냅샷(phase 훅이 갱신)을 쓴다.
#[derive(Debug)]
struct OurDraftAi;

impl ModDraftScoreHook for OurDraftAi {
    fn id(&self) -> &str {
        "tfm2_banpick_order.ban_context"
    }

    fn score_ban(
        &self,
        ctx: &DraftScoreContext,
        candidate: usize,
        _base_score: f32,
    ) -> DraftScoreDecision {
        let cfg = config::get();
        if !cfg.enabled || !cfg.ai_ban_context {
            return DraftScoreDecision::Pass;
        }
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            draft_ai::compute_ban_bias(
                candidate,
                ctx.ally_ban.len(),
                ctx.enemy_ban.len(),
                cfg.ai_w_syn,
                cfg.ai_w_cnt,
                cfg.ai_cap,
            )
        }));
        match r {
            Ok(Some((bias, name, syn, cnt))) if bias.abs() > 0.0001 => {
                draft_ai::CNT_BAN_ADJ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if cfg.debug {
                    config::dlog(&format!(
                        "ban_adj {name}: bias={bias:+.3} (syn={syn:.2} cnt={cnt:.2})"
                    ));
                }
                DraftScoreDecision::Add(bias)
            }
            _ => {
                draft_ai::CNT_BAN_PASS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                DraftScoreDecision::Pass
            }
        }
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    config::load();
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(BanpickOrderExt);
    reg.add_draft_score_hook(OurDraftAi);
    reg
}

declare_mod!(init);
