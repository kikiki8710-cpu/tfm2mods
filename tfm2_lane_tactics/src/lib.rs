//! tfm2_lane_tactics — 라인전 이후 포지션 전술(0.6.4 · stable 껍데기 + 네이티브 wrap detour).
//!
//! 게임 `LegacyPlanHandler::passive_plan`(라인전 이후 소극 플랜 = "지금 어느 라인에 설까")을 wrap 으로 감싸
//! **원본 실행 → 반환된 PassiveLine 의 line 을 포지션별 규칙으로 교체**한다. 규칙 = `lane_tactics.cfg`(모드 폴더).
//!   규칙(왼쪽부터 시도 · 첫 성립이 채택): `top|mid|bot` 고정 / `pushed[:임계]` 가장 밀린 라인 /
//!   `push[:임계]` 가장 밀고 있는 라인 / `ally:<pos>` 그 아군과 같은 라인(점유 → 그 아군 규칙 → 위치(기지 제외)) / `game` 게임 원래 선택 유지(여기서 중단).
//! 설계·RE 원문 = mods_report\tfm2_lane_tactics\ (RE\2026-09-30_G1~G4 · 0.6.1 기준, 0.6.2 재핀 G5, 0.6.3 재핀 RE\2026-10-14_G6, 0.6.4 재핀 RE\2026-10-14_G7).
//!
//! 안전: wrap 본문 = catch_unwind · 원본은 항상 먼저 실행하고 반환값을 그대로 돌려준다(모드 로직 실패 = 게임 원래 동작).
//!   RVA 는 게임 버전 0.6.4 에서만 설치(다른 버전이면 설치 안 함). 진입부 12B 가 기대값과 다르면(다른 모드 훅 포함) 설치 안 함.
use mod_api_stable::{declare_stable_mod, LogLevel, StableClient, StableExtension, StableHost, StableMod};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicU64, AtomicU8, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

const MOD_ID: &str = "tfm2_lane_tactics";

// ── 0.6.4 상수(근거 = mods_report\tfm2_lane_tactics\RE\2026-10-14_G7 · 0.6.3 = G6) ──
const GAME_VER: (u32, u32, u32) = (0, 6, 5);
/// passive_plan (0.6.1 0xd40310 → 0.6.2 0xd3a0f0 → 0.6.3 0xd5d860 → 0.6.4 0xd5c300(진입 32B 유일) · 소스 plan\handler\assign.rs)
const RVA_PASSIVE_PLAN: usize = 0xd5b2e0;
/// 진입부 12B(0.6.2 와 동일) = push rbp,r15,r14,r13,r12,rsi,rdi,rbx (RIP 상대 없음 · 명령 경계 12)
const PROLOG_PASSIVE_PLAN: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];

// ── 구조체 오프셋(0.6.4 · RE 2026-10-14_G7 · 0.6.3→0.6.4 = LPH 필드만 +0xa8 · 괄호 = 0.6.3 / 0.6.2) ──
const PLAN_TAG: usize = 0x0;          // BigPlan tag(qword · u32 비교 안전) · 3 = PassiveLine
const PL_WAIT: usize = 0x11e;         // 1 = sub_plan 즉시 LineWait(서포터 hold) (0x11f)
const PL_FLAGS: usize = 0x120;        // byte != 0 = 고정(웨이브 우선·SplitEpic 등) — 건드리지 않음 (0.6.2 = u32 bit0)
const PL_LINE: usize = 0x124;         // LineType u8 (Top0 Mid1 Bottom2) · ⚠0.6.1 은 +0x123(0.6.2 에선 플래그 바이트)
const RET_CODE: usize = 0x348;        // 발원 코드 u8 (BigPlan 0x348B 뒤) (0x370)
const LPH_PLAN_TAG: usize = 0x1670;   // 현재 커밋된 plan ★0.6.5 +0x30 (0.6.4 0x1640 · 0.6.3 0x1598 · 0.6.2 0xfc0)
const LPH_PLAN_LINE: usize = 0x1794;  // = +0x1670 + 0x124 (0.6.4 0x1764 · 0.6.3 0x16bc · 0.6.2 0x10e4)
const LPH_DEST_TAG: usize = 0x14e0;   // 목적지 hold Option(bit0) ★0.6.5 +0x30 (0.6.4 0x14b0 · 0.6.3 0x1408 · 0.6.2 0xe70) · ⚠0.6.3 은 hold 가 커밋을 막지 않음 → 진단 읽기만
const LPH_LATCH_LINE: usize = 0x2a6f; // v2_apply 래치 라인(0xff = None) ★0.6.5 +0x48 (0.6.4 0x2a27 · 0.6.3 0x297f · 0.6.2 0x26bf)
const PS_TEAM: usize = 0x820;         // ★0.6.5 Player +0x20 (0.6.4 0x800 · 0xa00)
const PS_POS: usize = 0x8b0;          // ★0.6.5 Player +0x20 (0.6.4 0x890 · 0xa90)
const BB_STRIDE: usize = 0xae0;       // ★0.6.5 imul 0xae0 19회 (0.6.4 0xac8 · 0.6.2 0x5e0 · 0.6.1 0x5c8)
const BB_CLAIM: usize = 0x808;        // ★0.6.5 +0x18 (0.6.4 0x7f0) · 라인 점유 tag u8(0=Line) · +1 = line · +pos*0x20 (0.6.2 +0x318 · 0.6.1 +0x300)
const CACHE_TICK: usize = 0x1b8;      // 게임 line_phase 가 읽는 tick 필드(0.6.3 · 0.6.2 는 vt+0x28 호출)
const CACHE_CHAMP: usize = 0x238;     // player_champion[team*0x28 + pos*8] (0x1e0)
const MINION_LIST: [usize; 3] = [0x10, 0x90, 0xd0]; // 라인별 미니언 리스트 + t*0x20 · len +0x18 (0.6.2 = 0x10+line*0x40)
const ENT_X: usize = 0x658;           // 엔티티 x (0x660)
const ENT_Y: usize = 0x660;           // 엔티티 y (0x668)

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleFileNameW(module: usize, buf: *mut u16, size: u32) -> u32;
    fn VirtualAlloc(addr: usize, size: usize, ty: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, prot: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(h: usize, addr: usize, size: usize) -> i32;
    fn GetCurrentProcess() -> usize;
}

fn mod_dir() -> Option<String> {
    let mut buf = [0u16; 520];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n == 0 || n >= buf.len() { return None; }
    let exe = String::from_utf16_lossy(&buf[..n]);
    exe.rfind(|c| c == '\\' || c == '/').map(|i| format!(r"{}\mods\{}", &exe[..i], MOD_ID))
}
fn w(s: &str) {
    if let Some(d) = mod_dir() {
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(format!(r"{}\lane_tactics.log", d)) { let _ = writeln!(f, "{}", s); }
    }
}

// ═══════════════════════════ 설정 ═══════════════════════════
#[derive(Clone, Copy, Debug)]
enum Rule { Line(u8), Pushed(i64), Push(i64), Ally(u8), Game }

struct Cfg {
    enabled: bool,
    rules: [Vec<Rule>; 5],
    codes: [bool; 64],
    teams: [bool; 2],
    clear_latch: bool,
    clear_hold: bool,
    trace: bool,
}

const POS_NAMES: [&str; 5] = ["top", "jungle", "mid", "bot", "sup"];
const DEFAULT_CFG: &str = "# tfm2_lane_tactics 설정 (UTF-8 · BOM 없이 저장 · 수정 후 게임 재시작)
# 라인전이 끝난 뒤(첫 에픽 스폰 30초 전부터) AI 가 '평소에 설 라인'을 포지션별로 정한다.
# 왼쪽부터 시도해서 처음 성립하는 규칙을 쓴다.
#   top / mid / bot      : 그 라인 고정
#   pushed[:임계]         : 우리가 가장 밀리고 있는 라인 (임계 이하일 때만 · 기본 -2000 · 범위 -10000~10000)
#   push[:임계]           : 우리가 가장 밀고 있는 라인 (임계 이상일 때만 · 기본 -10000 = 항상)
#   ally:<top|jungle|mid|bot|sup> : 그 아군의 라인 (라인 플랜이면 그 라인 → 귀환·교전 중이면 그 아군 규칙이 가리키는 라인)
#                                   그 아군이 죽었으면 나와 가장 가까운 살아 있는 아군의 라인
#   game                 : 게임 원래 선택을 그대로 둔다 (여기서 중단)
# 비워 두면(또는 game) 그 포지션은 손대지 않는다. 정글은 라인 플랜이 아니라 영향 없음.
enabled=1
top=game
jungle=game
mid=pushed:-2000, bot
bot=mid
sup=ally:bot, game

# 게임이 이 발원 코드로 라인을 골랐을 때만 덮어쓴다(오브젝트·웨이브 우선·스플릿 등 목적 있는 이동은 건드리지 않음).
# 18/19 = 리드 폴백(평소 라인) · 30/31 = 서포터 재선택 · 29 = 오브젝트 쪽 앰비언트 라인(0.6.3 에선 안 나옴) · 6 = 에고 웨이브 · 13 = 커버
override_codes=18,19,29,30,31
# 적용 진영: both / 0 / 1   (진영 = 경기 내 블루0·레드1, 백그라운드 리그 경기 포함)
teams=both
# 게임이 기억해 둔 라인 래치를 지워 되돌림을 막는다(1 권장)
clear_latch=1
# (0.6.2 전용 · 0.6.3 은 게임이 hold 중에도 라인 변경을 반영하므로 무시됨)
clear_hold=0
# 1 = 처음 200건의 교체 내역을 로그에 남김(진단용)
trace=0
";

fn parse_rule(tok: &str) -> Option<Rule> {
    let t = tok.trim().to_ascii_lowercase();
    if t.is_empty() { return None; }
    let (head, arg) = match t.split_once(':') { Some((a, b)) => (a.trim().to_string(), Some(b.trim().to_string())), None => (t.clone(), None) };
    let pos_of = |s: &str| -> Option<u8> { match s { "top" => Some(0), "jungle" | "jg" | "jug" => Some(1), "mid" => Some(2), "bot" | "bottom" | "adc" => Some(3), "sup" | "support" => Some(4), _ => None } };
    match head.as_str() {
        "top" => Some(Rule::Line(0)),
        "mid" => Some(Rule::Line(1)),
        "bot" | "bottom" => Some(Rule::Line(2)),
        "game" => Some(Rule::Game),
        "pushed" => Some(Rule::Pushed(arg.and_then(|a| a.parse().ok()).unwrap_or(-2000))),
        "push" => Some(Rule::Push(arg.and_then(|a| a.parse().ok()).unwrap_or(-10000))),
        "ally" => arg.as_deref().and_then(pos_of).map(Rule::Ally),
        _ => None,
    }
}

fn load_cfg() -> Cfg {
    let mut c = Cfg { enabled: true, rules: Default::default(), codes: [false; 64], teams: [true; 2], clear_latch: true, clear_hold: true, trace: false };
    let path = mod_dir().map(|d| format!(r"{}\lane_tactics.cfg", d));
    let txt = match path.as_deref().map(std::fs::read_to_string) {
        Some(Ok(t)) => t,
        _ => { if let Some(p) = &path { let _ = std::fs::write(p, DEFAULT_CFG); w("[cfg] lane_tactics.cfg 없음 → 기본값 파일 생성"); } DEFAULT_CFG.to_string() }
    };
    let txt = txt.trim_start_matches('\u{feff}');
    for raw in txt.lines() {
        let line = raw.split('#').next().unwrap_or("").trim();
        let Some((k, v)) = line.split_once('=') else { continue };
        let (k, v) = (k.trim().to_ascii_lowercase(), v.trim());
        let flag = || v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("on");
        match k.as_str() {
            "enabled" => c.enabled = flag(),
            "clear_latch" => c.clear_latch = flag(),
            "clear_hold" => c.clear_hold = flag(),
            "trace" => c.trace = flag(),
            "teams" => c.teams = match v { "0" => [true, false], "1" => [false, true], _ => [true, true] },
            "override_codes" => { c.codes = [false; 64]; for t in v.split(',') { if let Ok(n) = t.trim().parse::<usize>() { if n < 64 { c.codes[n] = true; } } } }
            _ => {
                if let Some(p) = POS_NAMES.iter().position(|n| *n == k) {
                    let mut rs = vec![];
                    for t in v.split(',') { match parse_rule(t) { Some(r) => rs.push(r), None => if !t.trim().is_empty() { w(&format!("[cfg] ⚠ 알 수 없는 규칙 '{}' ({}) — 무시", t.trim(), k)); } } }
                    c.rules[p] = rs;
                }
            }
        }
    }
    c
}
static CFG: OnceLock<Cfg> = OnceLock::new();
fn cfg() -> &'static Cfg { CFG.get_or_init(load_cfg) }

// ═══════════════════════════ 메모리 접근 ═══════════════════════════
#[inline(always)] fn ok(p: usize) -> bool { p >= 0x10000 && p < (1usize << 47) }
#[inline(always)] unsafe fn r64(p: usize, off: usize) -> u64 { core::ptr::read_unaligned((p + off) as *const u64) }
#[inline(always)] unsafe fn r32(p: usize, off: usize) -> u32 { core::ptr::read_unaligned((p + off) as *const u32) }
#[inline(always)] unsafe fn r8(p: usize, off: usize) -> u8 { *((p + off) as *const u8) }
#[inline(always)] unsafe fn w8(p: usize, off: usize, v: u8) { *((p + off) as *mut u8) = v; }

/// 현재 tick — 0.6.3 게임 line_phase 와 같은 필드를 읽는다(0.6.2 의 vt+0x28 getter 호출 제거 · 게임함수 호출 없음)
unsafe fn tick(cache: usize) -> Option<u64> { Some(r64(cache, CACHE_TICK)) }

/// map_regions::is_near_line(ctx, x, y, line) 재현(0.6.0 judge_verify060 helpers.rs · 0.6.1 G1 §4 와 동일 규칙 · 게임 헬퍼 호출 안 함)
/// strict = 기지 모서리(모든 라인에 속한다고 보는 구역)를 제외 — 아군 위치로 라인을 추정할 때 기지에 있으면 Top 으로 오판하던 v0.1.0 버그 방지
unsafe fn in_lane_region(ctx: usize, x: u64, y: u64, line: u8, strict: bool) -> bool {
    let s = r64(ctx, 8) as usize; if !ok(s) { return false; }
    let (width, height) = (r64(s, 0x12b8), r64(s, 0x12c0));
    let umax = |a: u64, b: u64| if a > b { a } else { b };
    let umin = |a: u64, b: u64| if a < b { a } else { b };
    let c = height.wrapping_sub(y);
    if umax(c, x) < 0x2ee01 { return !strict; }
    if umin(c, x) >= width.wrapping_sub(0x2ee00) { return !strict; }
    if line == 1 { return x.abs_diff(c) < 0xfa00; }
    let w2 = width.wrapping_sub(0xabe00) >> 1;
    let out_x = x < w2 || x > w2.wrapping_add(0xabe00);
    let inner = !out_x && { let h2 = height.wrapping_sub(0xabe00) >> 1; !(y < h2 || y > h2.wrapping_add(0xabe00)) };
    if line == 0 {
        if !inner { if c < x || c < 0x2ee01 { return false; } return c.wrapping_sub(x) >= 0xfa00; }
        let d = c.abs_diff(x);
        if c < 0x2ee01 || c < x || d > 0x176ff { return false; }
        return c.wrapping_sub(x) >= 0xfa00;
    }
    if !inner { if x < 0x2ee01 || x < c { return false; } return x.wrapping_sub(c) >= 0xfa00; }
    let d = c.abs_diff(x);
    if c > x || x < 0x2ee01 || d > 0x176ff { return false; }
    x.wrapping_sub(c) >= 0xfa00
}

/// 우리 팀 기준 라인 푸시 정도(from_mid · 양수 = 우리가 밀고 있음).
/// ⚠게임 식은 "적 미니언만 있는" 라인에서 방향이 뒤집힌다(raw = (−10000−e)/2) → 대칭식 (e−10000)/2 = −raw−10000 으로 보정.
unsafe fn from_mid(cache: usize, bbs: usize, team: usize, line: usize) -> i64 {
    let ms = bbs + team * BB_STRIDE + line * 0x28;
    let raw = r64(ms, 0x10) as i64;
    let list = |t: usize| cache + MINION_LIST[line] + t * 0x20;
    let (my, en) = (r64(list(team), 0x18), r64(list(1 - team), 0x18));
    if my == 0 && en > 0 { raw.wrapping_neg().wrapping_sub(10000) } else { raw }
}

/// 규칙 평가. Some(line) = 교체 · None = 게임 선택 유지.
unsafe fn eval(rules: &[Rule], pos: usize, team: usize, cache: usize, ctx: usize, bbs: usize) -> Option<u8> { eval_in(rules, pos, team, cache, ctx, bbs, true) }

/// count = 진단 카운트 기록 여부(아군 의도 추정용 내부 평가는 false · Ally 규칙은 재귀하지 않음)
unsafe fn eval_in(rules: &[Rule], pos: usize, team: usize, cache: usize, ctx: usize, bbs: usize, count: bool) -> Option<u8> {
    for (i, r) in rules.iter().enumerate() {
        let i = i.min(6);
        let hit = |l: u8| { if count { C_MATCH[fg()][pos * 8 + i].fetch_add(1, Ordering::Relaxed); } Some(l) };
        match *r {
            Rule::Game => { if count { C_MATCH[fg()][pos * 8 + i].fetch_add(1, Ordering::Relaxed); } return None; }
            Rule::Line(l) => return hit(l),
            Rule::Pushed(th) => {
                let (mut best, mut bl) = (i64::MAX, 0u8);
                for l in 0..3 { let v = from_mid(cache, bbs, team, l); if v < best { best = v; bl = l as u8; } }
                if best <= th { return hit(bl); }
            }
            Rule::Push(th) => {
                let (mut best, mut bl) = (i64::MIN, 0u8);
                for l in 0..3 { let v = from_mid(cache, bbs, team, l); if v > best { best = v; bl = l as u8; } }
                if best >= th { return hit(bl); }
            }
            Rule::Ally(p) => {
                if !count { continue; }                                        // 아군 의도 추정 중에는 Ally 재귀 금지
                let p = p as usize; if p == pos || p > 4 { continue; }
                let champ = |q: usize| r64(cache, CACHE_CHAMP + team * 0x28 + q * 8) as usize;
                if ok(champ(p)) {
                    if let Some(l) = ally_line(p, team, cache, ctx, bbs) { return hit(l); }
                } else {
                    // 그 아군이 죽었으면 → 나와 가장 가까운 살아 있는 아군의 라인(라인을 알 수 있는 아군 중)
                    let me = champ(pos);
                    if ok(me) {
                        let (mx, my) = (r64(me, ENT_X), r64(me, ENT_Y));
                        let mut cand: Vec<(u64, usize)> = (0..5).filter(|&q| q != pos && q != p && ok(champ(q)))
                            .map(|q| { let e = champ(q); let (dx, dy) = (r64(e, ENT_X).abs_diff(mx), r64(e, ENT_Y).abs_diff(my)); (dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)), q) }).collect();
                        cand.sort();
                        for (_, q) in cand { if let Some(l) = ally_line(q, team, cache, ctx, bbs) { C_ALLY[fg()][4].fetch_add(1, Ordering::Relaxed); return hit(l); } }
                    }
                }
                C_ALLY[fg()][3].fetch_add(1, Ordering::Relaxed);
            }
        }
    }
    if count { C_MATCH[fg()][pos * 8 + 7].fetch_add(1, Ordering::Relaxed); }     // 규칙 전부 불성립
    None
}

/// 살아 있는 아군 q 의 라인: ① 라인 플랜 점유 → ② (귀환·교전 등) 그 아군 규칙이 가리키는 라인 → ③ 현재 위치(기지 모서리 제외)
unsafe fn ally_line(q: usize, team: usize, cache: usize, ctx: usize, bbs: usize) -> Option<u8> {
    let bb = bbs + team * BB_STRIDE;
    if r8(bb, BB_CLAIM + q * 0x20) == 0 {
        let l = r8(bb, BB_CLAIM + 1 + q * 0x20); if l <= 2 { C_ALLY[fg()][0].fetch_add(1, Ordering::Relaxed); return Some(l); }
    }
    if let Some(l) = eval_in(&cfg().rules[q], q, team, cache, ctx, bbs, false) { C_ALLY[fg()][1].fetch_add(1, Ordering::Relaxed); return Some(l); }
    let e = r64(cache, CACHE_CHAMP + team * 0x28 + q * 8) as usize;
    if ok(e) {
        let (x, y) = (r64(e, ENT_X), r64(e, ENT_Y));
        for l in 0..3u8 { if in_lane_region(ctx, x, y, l, true) { C_ALLY[fg()][2].fetch_add(1, Ordering::Relaxed); return Some(l); } }
    }
    None
}

// ═══════════════════════════ 진단 카운터 ═══════════════════════════
#[link(name = "kernel32")]
extern "system" { fn GetCurrentThreadId() -> u32; }
/// 관전 경기 = 메인 스레드(post_update 가 도는 스레드)에서 sim 된다는 기존 실측(0.5.x · tfm2-background-sim-parallel)에 기댄 구분 — 0 관전 · 1 배경(추정 분류)
static MAIN_TID: AtomicU64 = AtomicU64::new(0);
#[inline(always)] fn fg() -> usize { if unsafe { GetCurrentThreadId() } as u64 == MAIN_TID.load(Ordering::Relaxed) { 0 } else { 1 } }

static C_CALLS: [AtomicU64; 2] = [const { AtomicU64::new(0) }; 2];
static C_ELIG: [AtomicU64; 2] = [const { AtomicU64::new(0) }; 2];
static C_OVR: [AtomicU64; 2] = [const { AtomicU64::new(0) }; 2];
static C_SAME: [AtomicU64; 2] = [const { AtomicU64::new(0) }; 2];
static C_KEEP: [AtomicU64; 2] = [const { AtomicU64::new(0) }; 2];
static C_HOLD: [AtomicU64; 2] = [const { AtomicU64::new(0) }; 2];
static C_PAN: AtomicU64 = AtomicU64::new(0);
static C_COMMIT_OK: [AtomicU64; 2] = [const { AtomicU64::new(0) }; 2];
static C_COMMIT_BAD: [AtomicU64; 2] = [const { AtomicU64::new(0) }; 2];
static C_POS_OVR: [[AtomicU64; 5]; 2] = [const { [const { AtomicU64::new(0) }; 5] }; 2];
/// (진영,포지션)별 직전 목표 라인(0xff 없음) — 커밋 반영 확인용(백그라운드 경기와 섞이므로 근사치)
static LAST_WANT: [AtomicU8; 10] = [const { AtomicU8::new(0xff) }; 10];
/// ally 규칙 해석 경로 [점유 · 아군 규칙 · 위치 · 불성립 · (사망→)가까운 아군]
static C_ALLY: [[AtomicU64; 5]; 2] = [const { [const { AtomicU64::new(0) }; 5] }; 2];
/// 규칙 채택 분포 [pos][규칙 idx 0..6 · 7 = 전부 불성립]
static C_MATCH: [[AtomicU64; 40]; 2] = [const { [const { AtomicU64::new(0) }; 40] }; 2];
/// 라인전 이후 반환 분포 [pos][code 0..63][최종 line 0..2 · 3 = PassiveLine 아님]
static C_RET: [[AtomicU64; 5 * 64 * 4]; 2] = [const { [const { AtomicU64::new(0) }; 5 * 64 * 4] }; 2];
static TRACE: Mutex<Vec<String>> = Mutex::new(Vec::new());
static TRACE_N: AtomicU64 = AtomicU64::new(0);

unsafe fn post(sret: usize, lph: usize, player: usize, data: usize) {
    let c = cfg();
    if !c.enabled || !ok(sret) || !ok(lph) || !ok(player) || !ok(data) { return; }
    let team = r64(player, PS_TEAM) as usize; if team > 1 || !c.teams[team] { return; }
    let pos = r32(player, PS_POS) as usize; if pos > 4 { return; }
    let (cache, ctx, bbs) = (r64(data, 0) as usize, r64(data, 8) as usize, r64(data, 0x10) as usize);
    if !ok(cache) || !ok(ctx) || !ok(bbs) { return; }
    let tut = r8(ctx, 0x38); if !(tut == 0 || tut == 7 || tut == 8) { return; }  // 3라인 맵만
    let setting = r64(ctx, 8) as usize; if !ok(setting) { return; }
    let tps = r64(setting, 0x12f8); let first = r64(setting, 0x8a8);
    let Some(now) = tick(cache) else { return };
    if now < first.saturating_sub(tps.wrapping_mul(30)) { return; }          // 라인전 중
    let code = (r8(sret, RET_CODE) as usize).min(63);
    let is_pl = r32(sret, PLAN_TAG) == 3;
    let decided = decide(c, sret, lph, cache, ctx, bbs, team, pos, code, is_pl, now);
    let fin = if is_pl { (r8(sret, PL_LINE) as usize).min(2) } else { 3 };
    let _ = decided;
    C_RET[fg()][(pos * 64 + code) * 4 + fin].fetch_add(1, Ordering::Relaxed);
}

#[allow(clippy::too_many_arguments)]
unsafe fn decide(c: &Cfg, sret: usize, lph: usize, cache: usize, ctx: usize, bbs: usize, team: usize, pos: usize, code: usize, is_pl: bool, now: u64) {
    if !is_pl || !c.codes[code] || c.rules[pos].is_empty() { return; }      // PassiveLine · 교체 대상 코드만
    if r8(sret, PL_FLAGS) != 0 { return; }
    C_ELIG[fg()].fetch_add(1, Ordering::Relaxed);

    // 직전 교체가 실제로 커밋됐는지(래치/hold 되돌림 감시)
    let slot = team * 5 + pos;
    let prev = LAST_WANT[slot].load(Ordering::Relaxed);
    if prev != 0xff && r32(lph, LPH_PLAN_TAG) == 3 {
        if r8(lph, LPH_PLAN_LINE) == prev { C_COMMIT_OK[fg()].fetch_add(1, Ordering::Relaxed); } else { C_COMMIT_BAD[fg()].fetch_add(1, Ordering::Relaxed); }
    }

    let cur = r8(sret, PL_LINE);
    let Some(want) = eval(&c.rules[pos], pos, team, cache, ctx, bbs) else {
        C_KEEP[fg()].fetch_add(1, Ordering::Relaxed); LAST_WANT[slot].store(0xff, Ordering::Relaxed); return;
    };
    LAST_WANT[slot].store(want, Ordering::Relaxed);
    if c.clear_latch { w8(lph, LPH_LATCH_LINE, 0xff); }
    if want == cur { C_SAME[fg()].fetch_add(1, Ordering::Relaxed); return; }
    w8(sret, PL_LINE, want);
    w8(sret, PL_WAIT, 0);
    C_OVR[fg()].fetch_add(1, Ordering::Relaxed); C_POS_OVR[fg()][pos].fetch_add(1, Ordering::Relaxed);
    // 0.6.3: 목적지 hold 는 커밋을 막지 않는다(RE G6 §4) → 쓰기 없이 "hold 중이던 교체" 진단 카운트만(clear_hold 는 무시)
    if r32(lph, LPH_PLAN_TAG) == 3 && r8(lph, LPH_PLAN_LINE) != want && r8(lph, LPH_DEST_TAG) & 1 != 0 {
        C_HOLD[fg()].fetch_add(1, Ordering::Relaxed);
    }
    if c.trace && TRACE_N.fetch_add(1, Ordering::Relaxed) < 200 {
        if let Ok(mut g) = TRACE.try_lock() {
            let fm: Vec<i64> = (0..3).map(|l| from_mid(cache, bbs, team, l)).collect();
            g.push(format!("  [ovr] tick={} team={} pos={} code={} {}→{} from_mid(T/M/B)={:?}", now, team, POS_NAMES[pos], code, cur, want, fm));
        }
    }
}

// ═══════════════════════════ wrap detour ═══════════════════════════
static BASE: AtomicUsize = AtomicUsize::new(0);
static ORIG: AtomicUsize = AtomicUsize::new(0);
static INSTALLED: AtomicUsize = AtomicUsize::new(0); // 0 미시도 · 1 설치 · 2 프롤로그 불일치 · 3 실패 · 4 버전 불일치
static DONE: AtomicUsize = AtomicUsize::new(0);
type FnPassivePlan = unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize) -> usize;

/// passive_plan(sret, &LPH, version, rnd, player, data, debug) — 원본 실행 후 사후 교체
unsafe extern "C" fn w_passive_plan(sret: usize, lph: usize, ver: usize, rnd: usize, player: usize, data: usize, dbg: usize) -> usize {
    let f: FnPassivePlan = core::mem::transmute(ORIG.load(Ordering::Acquire));
    let r = f(sret, lph, ver, rnd, player, data, dbg);
    C_CALLS[fg()].fetch_add(1, Ordering::Relaxed);
    if catch_unwind(AssertUnwindSafe(|| post(sret, lph, player, data))).is_err() { C_PAN.fetch_add(1, Ordering::Relaxed); }
    r
}

/// 트램폴린([원본 12B] + jmp [rip+0] → fn+12) 게시 후 진입부를 `movabs rax,wrap; jmp rax` 로 덮는다. 실패 시 .text 무손상.
unsafe fn install_wrap(fn_addr: usize, prolog: &[u8], wrap: usize) -> usize {
    let n = prolog.len();
    if *(fn_addr as *const u8) == 0x48 && *((fn_addr + 1) as *const u8) == 0xb8 { return 2; } // 이미 다른 훅(체인 미지원 → 설치 안 함)
    for k in 0..n { if *((fn_addr + k) as *const u8) != prolog[k] { return 2; } }
    let stub = VirtualAlloc(0, 64, 0x1000 | 0x2000, 0x40); if stub == 0 { return 3; }
    let mut t: Vec<u8> = Vec::with_capacity(n + 14);
    t.extend_from_slice(prolog);
    t.extend_from_slice(&[0xff, 0x25, 0, 0, 0, 0]); t.extend_from_slice(&(fn_addr + n).to_le_bytes());
    core::ptr::copy_nonoverlapping(t.as_ptr(), stub as *mut u8, t.len());
    FlushInstructionCache(GetCurrentProcess(), stub, t.len());
    ORIG.store(stub, Ordering::SeqCst);                                                          // ★패치 전에 게시(배경 sim 경합)
    let mut patch = vec![0x90u8; n];
    patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&wrap.to_le_bytes()); patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, n, 0x40, &mut old) == 0 { return 3; }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, n);
    VirtualProtect(fn_addr, n, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, n);
    1
}

fn install_all(ver: (u32, u32, u32)) {
    if DONE.swap(1, Ordering::SeqCst) != 0 { return; }
    let _ = catch_unwind(|| unsafe {
        let c = cfg();
        if !c.enabled { w("[install] enabled=0 → 설치 안 함"); return; }
        if ver != GAME_VER { INSTALLED.store(4, Ordering::Relaxed); w(&format!("[install] 게임 {:?} ≠ 대상 {:?} → 설치 안 함(RVA 재핀 필요)", ver, GAME_VER)); return; }
        let base = GetModuleHandleW(core::ptr::null()); BASE.store(base, Ordering::Relaxed);
        let r = install_wrap(base + RVA_PASSIVE_PLAN, &PROLOG_PASSIVE_PLAN, w_passive_plan as *const () as usize);
        INSTALLED.store(r, Ordering::Relaxed);
        w(&format!("[install] passive_plan {:#x} → {}", RVA_PASSIVE_PLAN, match r { 1 => "설치", 2 => "✗ 진입부 불일치(다른 모드 훅 또는 버전) — 설치 안 함", _ => "✗ 실패" }));
        let rs: Vec<String> = (0..5).map(|p| format!("{}={:?}", POS_NAMES[p], c.rules[p])).collect();
        let codes: Vec<usize> = (0..64).filter(|&i| c.codes[i]).collect();
        w(&format!("[cfg] {} · codes={:?} teams={:?} clear_latch={} clear_hold={} trace={}", rs.join(" "), codes, c.teams, c.clear_latch, c.clear_hold, c.trace));
    });
}

static FRAME: AtomicU64 = AtomicU64::new(0);
static LAST_SNAP: Mutex<[u64; 4]> = Mutex::new([0; 4]);
fn snapshot() {
    let (calls, elig, ovr) = (C_CALLS[0].load(Ordering::Relaxed) + C_CALLS[1].load(Ordering::Relaxed), C_ELIG[0].load(Ordering::Relaxed) + C_ELIG[1].load(Ordering::Relaxed), C_OVR[0].load(Ordering::Relaxed) + C_OVR[1].load(Ordering::Relaxed));
    let mut g = LAST_SNAP.lock().unwrap_or_else(|e| e.into_inner());
    if g[0] == calls && g[1] == elig && g[2] == ovr { return; }
    *g = [calls, elig, ovr, 0];
    const LN: [&str; 4] = ["T", "M", "B", "비PL"];
    for k in 0..2 {
        let tag = if k == 0 { "관전" } else { "배경" };
        let c = C_CALLS[k].load(Ordering::Relaxed); if c == 0 { continue; }
        let per: Vec<String> = (0..5).map(|p| format!("{}:{}", POS_NAMES[p], C_POS_OVR[k][p].load(Ordering::Relaxed))).collect();
        w(&format!("[stat {}] calls={} eligible={} override={} same={} keep(game)={} hold중교체={} commit ok/bad={}/{} panic(전체)={} | {}", tag,
            c, C_ELIG[k].load(Ordering::Relaxed), C_OVR[k].load(Ordering::Relaxed), C_SAME[k].load(Ordering::Relaxed), C_KEEP[k].load(Ordering::Relaxed), C_HOLD[k].load(Ordering::Relaxed),
            C_COMMIT_OK[k].load(Ordering::Relaxed), C_COMMIT_BAD[k].load(Ordering::Relaxed), C_PAN.load(Ordering::Relaxed), per.join(" ")));
        let a = &C_ALLY[k];
        w(&format!("  [ally {}] 점유={} 아군규칙={} 위치={} 사망→가까운아군={} 불성립={}", tag, a[0].load(Ordering::Relaxed), a[1].load(Ordering::Relaxed), a[2].load(Ordering::Relaxed), a[4].load(Ordering::Relaxed), a[3].load(Ordering::Relaxed)));
        for p in 0..5 {
            let m: Vec<String> = (0..8).filter_map(|i| { let v = C_MATCH[k][p * 8 + i].load(Ordering::Relaxed); if v == 0 { None } else { Some(format!("{}:{}", if i == 7 { "불성립".to_string() } else { format!("r{}", i) }, v)) } }).collect();
            let mut r: Vec<String> = vec![];
            for code in 0..64 { let v: Vec<u64> = (0..4).map(|l| C_RET[k][(p * 64 + code) * 4 + l].load(Ordering::Relaxed)).collect();
                if v.iter().sum::<u64>() == 0 { continue; }
                let parts: Vec<String> = (0..4).filter(|&l| v[l] > 0).map(|l| format!("{}{}", LN[l], v[l])).collect();
                r.push(format!("c{}[{}]", code, parts.join(" "))); }
            if !m.is_empty() || !r.is_empty() { w(&format!("  [dist {} {}] 규칙 {} | 코드→최종라인 {}", tag, POS_NAMES[p], m.join(" "), r.join(" "))); }
        }
    }
    let t: Vec<String> = { let mut v = TRACE.lock().unwrap_or_else(|e| e.into_inner()); std::mem::take(&mut *v) };
    for s in t { w(&s); }
}

static VER: Mutex<(u32, u32, u32)> = Mutex::new((0, 0, 0));
struct Ext;
impl StableExtension for Ext {
    fn post_update(&self, _ctx: &mut StableClient<'_>, _dt: u64) {
        let _ = catch_unwind(AssertUnwindSafe(|| {
            MAIN_TID.store(unsafe { GetCurrentThreadId() } as u64, Ordering::Relaxed);
            install_all(*VER.lock().unwrap_or_else(|e| e.into_inner()));
            let f = FRAME.fetch_add(1, Ordering::Relaxed);
            if f % 600 == 0 && INSTALLED.load(Ordering::Relaxed) == 1 { snapshot(); }
        }));
    }
}
fn init(host: &StableHost) -> StableMod {
    let v = host.game_version();
    let ver = (v.major as u32, v.minor as u32, v.patch as u32);
    *VER.lock().unwrap_or_else(|e| e.into_inner()) = ver;
    w(&format!("\n########## INIT {} v{} · game {}.{}.{} ##########", MOD_ID, env!("CARGO_PKG_VERSION"), v.major, v.minor, v.patch));
    host.log(LogLevel::Info, MOD_ID);
    install_all(ver);
    let mut d = StableMod::new(MOD_ID);
    d.set_extension(Ext);
    d
}
declare_stable_mod!(init);
