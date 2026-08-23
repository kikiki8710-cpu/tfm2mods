//! tfm2_champion_exclude — 인게임 패치 신챔프 추가 대상에서 특정 챔피언 영구 제외 (게임 0.5.6)
//! ===========================================================================
//! 배경: 시즌 진행 중 패치 데이(ScheduleType Minor/Major/SeasonPatch)에 게임이
//!   "챔피언 registry(바닐라+모드챔프 전부) − available_champions" 를 후보로 뽑아
//!   셔플 후 앞 N개(N=db+0x708, champion_add 설정)를 available_champions 에 push
//!   = 신챔피언 출시. (RE 정본 = REPORT\tfm2_champion_exclude\RE\
//!   2026-08-19_신챔프추가-메커니즘-0.5.5.md)
//! 콜체인(0.5.6 재핀 2026-08-20): day-proceed 0x2109760 → 패치데이 0x2371820
//!   → 신챔프추가 0x2363ee0(콜사이트 0x236406e)
//!   → ★후보 Vec 생성 0x1894610(콜러 0x2363ee0 한 곳뿐 — 0.5.6 실측).
//!   (0.5.5 = 0x1e34a00 → 0x203acc0 → 0x202c440(@0x202c5f2) → 0x186e150)
//! 해법: 후보 Vec 생성 진입 트램폴린 detour — 원본 호출로 후보 Vec<String> 을 받은 뒤
//!   현재 세이브에 설정된 제외 챔피언 id 를 swap_remove 로 걸러낸다.
//!   후보에서 빠지면 셔플/선택/available push/액션 등록/팀 티어 반영/뉴스까지
//!   전부 자연히 배제 = "영원히 추가 안 됨". 이미 출시된 챔피언은 건드리지 않는다.
//!   '*' 한 줄이면 후보 전량 제거 = 신챔프 추가 전면 차단.
//!   빈 후보 = 바닐라 "전 챔피언 출시완료" 상태와 비트동일(알림 0건) — 정적 확증 =
//!   RE\2026-08-20_알림게이트-빈Vec-등가성.md.
//! ★v0.2.0 인게임 UI(2026-08-23): 환경설정 → 게임플레이 탭 맨 아래 '추가 챔피언 설정' 행
//!   (pos_lock_row 아래 — 타 모드 tfm2_champ_pos_lock 과 같은 주입 방식·같은 앵커) →
//!   클릭 시 팝업(틀 = pos_lock_popup 동일): **아직 추가 안 된 챔피언**(registry−available
//!   근사 = champ_uv 슈퍼셋∪mod_champions∪세이브설정∪seen 을 db.champion_info 로 검증) 그리드,
//!   클릭 토글 = 제외 선택, 확인 = 현재 세이브에 저장(패치데이 훅이 그대로 소비).
//! 적용 범위 주의: 이 모드는 "시즌 중 패치로 추가"만 막는다. 신규 게임 시작 시
//!   초기 풀 포함은 범위 외 — 바닐라 게임 생성 옵션(커스텀 챔피언)으로 제어 가능.
//! 안전:
//!   - detour 본문 = catch_unwind(AssertUnwindSafe) 격리(§3 — detour 패닉은 UB).
//!   - 프롤로그 17B 실측 검증 후에만 설치(불일치 = skip + 실측 바이트 로그).
//!     기존 외부 훅(48 B8 .. FF E0)이면 체인 훅. 로더 훅은 체인 설치(post_update 늦설치).
//!   - 제거된 String 힙 버퍼는 의도적으로 leak(FREE_REMOVED=false 기본).
//!   - 포인터는 범위체크 + VirtualQuery. exe base 는 GetModuleHandleW(null) 동적.
//! ★v0.4.0 세이브별 설정: 제외 목록은 **세이브 파일 안 mod save data**(공식 API,
//!   docs\mod-save-data.md — 네임스페이스 MOD_ID·키 "exclude"·키당 1MiB 한도)에 저장 ⟹
//!   세이브마다 독립·세이브 복사/백업에 동행. ★v0.4.2: cfg 파일 축 완전 제거(유저 지시) —
//!   세이브에 설정 없음 = 제외 없음(바닐라). 패치데이 detour 는 SDK 컨텍스트가 없어
//!   post_update(InGame)가 매 프레임 캐시한 SAVE_EXCL 을 읽는다(패치데이 = 항상 세이브
//!   로드 후 발생이라 캐시 신선). UI 확인 = PENDING_SAVE 이월 → 다음 프레임 기록.
//! 진단: mods\tfm2_champion_exclude\champion_exclude.txt + 후보 관측 캐시
//!   champion_exclude_seen.txt(패치데이 실후보 병합 — UI 목록 보강용).
//! ===========================================================================
#![allow(dead_code)]
use mod_api::*;
use std::collections::HashSet;
use std::io::Write as _;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

mod icon_data;
mod inject;

#[path = r"C:\tfm2mods\ui_kit\ui_kit.rs"]
mod ui_kit;

const MOD_ID: &str = "tfm2_champion_exclude";
const VERSION: &str = "0.5.0";
/// i18n 태그 접두(text/champion_exclude.i18n 을 mod.override_info merge 로
/// asset/base/text/ui 에 병합 — scrim ui.i18n 동형). 라벨 text = 이 태그면 게임이
/// 현재 언어(ko/en)로 자동 해석.
const I18N: &str = "#asset/base/text/ui?champ_excl.";

// build_inj.ps1 신원 검증용 — dll 안에 lib.rs 절대경로 문자열 필요.
#[no_mangle]
pub extern "C" fn tfm2_champion_exclude_src_id() -> *const u8 {
    concat!(file!(), "\0").as_bytes().as_ptr()
}

// ── 훅 사이트 (0.5.6 재핀 2026-08-20, 스켈레톤해시 UNIQUE — 0.5.5 = 0x186e150) ──
// 신챔프 추가 후보 Vec<String> 생성 (rcx=out, rdx=iter_ctx, ret rax=out)
const HOOK_RVA: usize = 0x1894610;
// 프롤로그: push rbp; push r15; push r14; push r12; push rsi; push rdi; push rbx;
//           sub rsp,0xA0  (10B + 7B = 17B, rip-rel 없음 — 0.5.6 실측 바이트 0.5.5와 완전 동일)
const HOOK_ORIG: [u8; 17] = [
    0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x54, 0x56, 0x57, 0x53,
    0x48, 0x81, 0xEC, 0xA0, 0x00, 0x00, 0x00,
];
const ORIG_LEN: usize = 17; // 12B jmp 훅 시 명령 경계
/// 게임 챔프아이콘 세터 FUN(assets,node,id_ptr,id_len,w,h,scale) — 0.5.6, pos_lock hooks.rs 동일값.
const RVA_ICON_SETTER: usize = 0x250bc30;
// 제거한 String 의 힙 버퍼를 HeapFree 할지 — 기본 false(의도적 leak, 상단 주석 참조)
const FREE_REMOVED: bool = false;
const MAX_CANDIDATES: usize = 4096;
const MAX_NAME_LEN: usize = 256;

// ── WinAPI ──
type BOOL = i32;
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleFileNameW(h: usize, buf: *mut u16, sz: u32) -> u32;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old: *mut u32) -> BOOL;
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, sz: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
    fn GetProcessHeap() -> usize;
    fn HeapFree(heap: usize, flags: u32, ptr: usize) -> BOOL;
}
#[repr(C)]
#[derive(Default)]
struct MemBasicInfo {
    base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32,
    region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32,
}

#[inline]
fn ptr_sane(addr: usize) -> bool {
    addr >= 0x10000 && addr < (1usize << 48)
}

#[inline]
unsafe fn readable(addr: usize, len: usize) -> bool {
    if !ptr_sane(addr) || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000;
    const RD: u32 = 0x02 | 0x04 | 0x20 | 0x40;
    const GUARD: u32 = 0x01 | 0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr.wrapping_add(len) <= mbi.base + mbi.region_size
}

// ── 로그 (게임 exe 기준 동적 경로 — 설치위치 하드코딩 금지) ──
fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}
fn ts() -> String {
    // KST(UTC+9) HH:MM:SS.mmm
    let ms = now_ms();
    let s = ms / 1000 + 9 * 3600;
    format!("{:02}:{:02}:{:02}.{:03}", (s / 3600) % 24, (s / 60) % 60, s % 60, ms % 1000)
}
pub(crate) fn mod_dir() -> Option<PathBuf> {
    let mut buf = [0u16; 1024];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), 1024) } as usize;
    if n == 0 || n >= 1024 { return None; }
    let exe = String::from_utf16_lossy(&buf[..n]);
    let mut p = PathBuf::from(exe);
    p.pop();
    p.push("mods");
    p.push(MOD_ID);
    Some(p)
}
pub(crate) fn log(msg: &str) {
    if let Some(d) = mod_dir() {
        let _ = std::fs::create_dir_all(&d);
        let p = d.join("champion_exclude.txt");
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&p) {
            let _ = writeln!(f, "[{}] {}", ts(), msg);
        }
    }
}

// ── 제외 목록 파서 (v0.4.2: cfg 파일 축 완전 제거·유저 지시 — 설정 = 세이브 단일) ──
/// 제외 목록 텍스트 파서(세이브 값): '#' 주석·'*'=전면차단·소문자 정규화.
fn parse_exclude_text(text: &str) -> (Vec<String>, bool) {
    let mut list = Vec::new();
    let mut block_all = false;
    let text = text.trim_start_matches('\u{feff}'); // BOM 허용
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() { continue; }
        if line == "*" { block_all = true; continue; }
        list.push(line.to_ascii_lowercase());
    }
    (list, block_all)
}

// ── 세이브별 설정 (v0.4.0 도입·v0.4.2 단일화 — 공식 mod save data, docs\mod-save-data.md) ──
// 세이브 안 네임스페이스 MOD_ID·키 "exclude"(텍스트 포맷·키당 1MiB 한도)에 저장 ⟹
// 설정이 세이브 파일에 따라다닌다(세이브 식별 불요). 세이브에 설정 없음 = 제외 없음(바닐라).
// (v0.4.2: cfg 파일 폴백 완전 제거 — 유저 지시 "cfg 안 쓰니까 빼줘".)
const SAVE_KEY: &str = "exclude";
const SAVE_NS_VERSION: usize = 1;
/// 현재 로드된 세이브에서 읽은 제외 목록(None = 세이브에 설정 없음 또는 세이브 밖 화면).
/// post_update(InGame)가 매 프레임 갱신 — 패치데이 detour 는 SDK 컨텍스트가 없어 이 캐시를 읽는다.
static SAVE_EXCL: Mutex<Option<(Vec<String>, bool)>> = Mutex::new(None);
/// UI 확인 → 세이브 기록 대기 본문. 다음 InGame 프레임의 post_update 가 소비
/// (클릭 콜백엔 ClientData 접근이 없어서 프레임으로 이월).
static PENDING_SAVE: Mutex<Option<String>> = Mutex::new(None);

/// 유효 제외 목록 = 현재 세이브의 설정(없으면 빈 목록 = 바닐라 동작). 3번째 = 출처 라벨.
fn effective_exclusion() -> (Vec<String>, bool, &'static str) {
    if let Some((l, s)) = SAVE_EXCL.lock().unwrap_or_else(|e| e.into_inner()).clone() {
        return (l, s, "세이브");
    }
    (Vec::new(), false, "설정없음")
}

// ── detour (패치데이 후보 필터) ──
// 원본 계약(0.5.5 RE): rcx=out(*mut Vec<String>), rdx=iter_ctx, 반환 rax=out.
// Vec{cap@0, ptr@8, len@0x10} / 요소 String{cap@0, ptr@8, len@0x10} stride 0x18.
type HookFn = extern "C" fn(usize, usize, usize, usize) -> usize;
static TRAMPOLINE: AtomicUsize = AtomicUsize::new(0);
static FIRE_COUNT: AtomicUsize = AtomicUsize::new(0);

extern "C" fn detour_candidates(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    let tramp = TRAMPOLINE.load(Ordering::Acquire);
    if tramp == 0 {
        // 트램폴린 미확보 시 detour 를 설치하지 않으므로 도달 불가(방어적 반환).
        return rcx;
    }
    let orig: HookFn = unsafe { core::mem::transmute(tramp) };
    let ret = orig(rcx, rdx, r8, r9);
    // 필터는 전부 격리 — 무슨 일이 있어도 게임 콜스택으로 패닉 전파 금지
    let _ = catch_unwind(AssertUnwindSafe(|| filter_candidates(ret)));
    ret
}

fn filter_candidates(out: usize) {
    let n = FIRE_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    if !ptr_sane(out) || unsafe { !readable(out, 0x18) } {
        log(&format!("fire#{}: out 포인터 이상(0x{:x}) — 필터 skip", n, out));
        return;
    }
    let vec_ptr = unsafe { *((out + 8) as *const usize) };
    let mut len = unsafe { *((out + 0x10) as *const usize) };
    if len == 0 {
        log(&format!("fire#{}: 후보 0개(추가 설정 off 또는 전원 출시) — 조치 없음", n));
        return;
    }
    if len > MAX_CANDIDATES || !ptr_sane(vec_ptr) || unsafe { !readable(vec_ptr, len * 0x18) } {
        log(&format!("fire#{}: 후보 Vec 이상(ptr=0x{:x} len={}) — 필터 skip(오독 방지)", n, vec_ptr, len));
        return;
    }

    let read_name = |idx: usize| -> Option<String> {
        let elem = vec_ptr + idx * 0x18;
        let sptr = unsafe { *((elem + 8) as *const usize) };
        let slen = unsafe { *((elem + 0x10) as *const usize) };
        if slen == 0 || slen > MAX_NAME_LEN || !ptr_sane(sptr) || unsafe { !readable(sptr, slen) } {
            return None;
        }
        let bytes = unsafe { core::slice::from_raw_parts(sptr as *const u8, slen) };
        std::str::from_utf8(bytes).ok().map(|s| s.to_string())
    };

    // 후보 전체 덤프(모드챔프 포함 여부 확증용 진단 — 패치데이당 1~2회라 저비용)
    let mut names: Vec<String> = Vec::with_capacity(len);
    for i in 0..len {
        names.push(read_name(i).unwrap_or_else(|| "<판독불가>".into()));
    }
    let (exclude, block_all, src) = effective_exclusion();
    log(&format!("fire#{}: 후보 {}개 = [{}] / 제외목록 {}개(출처={}){}",
        n, len, names.join(", "), exclude.len(), src, if block_all { " + 전면차단(*)" } else { "" }));
    save_seen(&names); // 실후보 관측 캐시(UI 목록 보강)

    if exclude.is_empty() && !block_all {
        return;
    }

    let mut removed: Vec<String> = Vec::new();
    let mut i = 0usize;
    while i < len {
        let name = read_name(i);
        let hit = match (&name, block_all) {
            (_, true) => true,
            (Some(nm), false) => exclude.iter().any(|e| e == &nm.to_ascii_lowercase()),
            (None, false) => false, // 판독불가 요소는 건드리지 않음
        };
        if hit {
            unsafe {
                let elem = vec_ptr + i * 0x18;
                if FREE_REMOVED {
                    let scap = *(elem as *const usize);
                    let sptr = *((elem + 8) as *const usize);
                    if scap > 0 && ptr_sane(sptr) {
                        HeapFree(GetProcessHeap(), 0, sptr);
                    }
                }
                // swap_remove: 마지막 요소(0x18B)를 이 자리에 복사 후 len−1
                let last = vec_ptr + (len - 1) * 0x18;
                if last != elem {
                    core::ptr::copy_nonoverlapping(last as *const u8, elem as *mut u8, 0x18);
                }
                len -= 1;
                *((out + 0x10) as *mut usize) = len;
            }
            removed.push(name.unwrap_or_else(|| "<판독불가>".into()));
            // i 는 그대로(방금 당겨온 요소 재검사)
        } else {
            i += 1;
        }
    }
    if removed.is_empty() {
        log(&format!("fire#{}: 매치 없음 — 후보 {}개 유지", n, len));
    } else {
        log(&format!("fire#{}: {}개 제거 = [{}] → 후보 {}개 남음",
            n, removed.len(), removed.join(", "), len));
    }
}

/// 패치데이에 관측한 실후보를 seen 파일에 병합(소문자·정렬·멱등).
fn save_seen(names: &[String]) {
    let Some(d) = mod_dir() else { return };
    let p = d.join("champion_exclude_seen.txt");
    let mut set = load_seen();
    let before = set.len();
    for n in names {
        if n != "<판독불가>" {
            set.insert(n.to_ascii_lowercase());
        }
    }
    if set.len() == before {
        return;
    }
    let mut v: Vec<&String> = set.iter().collect();
    v.sort();
    let body = v.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n");
    let _ = std::fs::write(&p, format!("# 패치데이에 관측된 추가 후보(자동 기록 — 편집 불요)\n{body}\n"));
}
fn load_seen() -> HashSet<String> {
    let mut set = HashSet::new();
    if let Some(d) = mod_dir() {
        if let Ok(t) = std::fs::read_to_string(d.join("champion_exclude_seen.txt")) {
            for line in t.lines() {
                let line = line.split('#').next().unwrap_or("").trim();
                if !line.is_empty() {
                    set.insert(line.to_ascii_lowercase());
                }
            }
        }
    }
    set
}

// ── 트램폴린 설치 (패치데이 훅) ──
unsafe fn install_hook() -> Result<String, String> {
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { return Err("GetModuleHandleW(null)=0".into()); }
    MOD_BASE.store(base, Ordering::Relaxed);
    let addr = base + HOOK_RVA;
    if !readable(addr, ORIG_LEN) {
        return Err(format!("훅 지점 unreadable @abs=0x{:x} (base=0x{:x} rva=0x{:x})", addr, base, HOOK_RVA));
    }
    let mut cur = [0u8; ORIG_LEN];
    core::ptr::copy_nonoverlapping(addr as *const u8, cur.as_mut_ptr(), ORIG_LEN);

    const MEM_COMMIT_RESERVE: u32 = 0x1000 | 0x2000;
    const PAGE_EXECUTE_READWRITE: u32 = 0x40;
    let tramp = VirtualAlloc(0, 0x1000, MEM_COMMIT_RESERVE, PAGE_EXECUTE_READWRITE);
    if tramp == 0 { return Err("VirtualAlloc 트램폴린 실패".into()); }

    let mode: &str;
    let mut tlen = 0usize;
    if cur == HOOK_ORIG {
        core::ptr::copy_nonoverlapping(cur.as_ptr(), tramp as *mut u8, ORIG_LEN);
        tlen = ORIG_LEN;
        let back = addr + ORIG_LEN;
        let jmp: [u8; 12] = jmp_abs(back);
        core::ptr::copy_nonoverlapping(jmp.as_ptr(), (tramp + tlen) as *mut u8, 12);
        tlen += 12;
        mode = "정상 프롤로그 → 트램폴린";
    } else if cur[0] == 0x48 && cur[1] == 0xB8 && cur[10] == 0xFF && cur[11] == 0xE0 {
        core::ptr::copy_nonoverlapping(cur.as_ptr(), tramp as *mut u8, 12);
        tlen = 12;
        let ext = usize::from_le_bytes(cur[2..10].try_into().unwrap());
        mode = "기존 외부 훅 감지 → 체인 훅";
        log(&format!("체인: 외부 훅 타깃=0x{:x}", ext));
    } else {
        return Err(format!(
            "프롤로그 불일치 → 설치 SKIP @abs=0x{:x} 실측={:02x?} 기대={:02x?} (패치버전 확인 필요)",
            addr, cur, HOOK_ORIG
        ));
    }
    let _ = tlen;
    TRAMPOLINE.store(tramp, Ordering::Release);

    let mut patch = [0x90u8; ORIG_LEN];
    patch[..12].copy_from_slice(&jmp_abs(detour_candidates as usize));
    let mut old: u32 = 0;
    if VirtualProtect(addr, ORIG_LEN, PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        return Err("VirtualProtect 실패".into());
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), addr as *mut u8, ORIG_LEN);
    let mut old2: u32 = 0;
    VirtualProtect(addr, ORIG_LEN, old, &mut old2);
    FlushInstructionCache(GetCurrentProcess(), addr, ORIG_LEN);

    let mut landed = [0u8; ORIG_LEN];
    core::ptr::copy_nonoverlapping(addr as *const u8, landed.as_mut_ptr(), ORIG_LEN);
    if landed == patch {
        Ok(format!("설치+VERIFIED @abs=0x{:x} (rva=0x{:x}) 모드={} tramp=0x{:x}", addr, HOOK_RVA, mode, tramp))
    } else {
        Err(format!("write 미반영 @abs=0x{:x} landed={:02x?}", addr, landed))
    }
}

#[inline]
fn jmp_abs(target: usize) -> [u8; 12] {
    let mut b = [0u8; 12];
    b[0] = 0x48; b[1] = 0xB8; // movabs rax, imm64
    b[2..10].copy_from_slice(&target.to_le_bytes());
    b[10] = 0xFF; b[11] = 0xE0; // jmp rax
    b
}

// ═══════════════════════════ 인게임 설정 UI (v0.2.0) ═══════════════════════════
// 참조구현 = tfm2_champ_pos_lock(행 주입·팝업·아이콘·클릭 라우팅 전부 동형).

static MOD_BASE: AtomicUsize = AtomicUsize::new(0);
/// 로더 detour 가 넘겨주는 현 씬 Assets(관리 씬 = 챔프 로드된 AssetServer).
static GAME_ASSETS: AtomicUsize = AtomicUsize::new(0);
pub(crate) fn note_assets(am: usize) {
    if (0x10000..1usize << 48).contains(&am) {
        GAME_ASSETS.store(am, Ordering::Relaxed);
    }
}

static CLICK_LAST: AtomicUsize = AtomicUsize::new(usize::MAX);
static POPUP_OPEN: AtomicBool = AtomicBool::new(false);
/// 팝업 열림 직후 세이브 설정 → 선택 상태 로드 요청.
static LOAD_SEL_REQ: AtomicBool = AtomicBool::new(false);
static GRID_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
/// 선택 상태 버전(토글마다 ++) — 그리드 시그니처 재료.
static SEL_VER: AtomicU64 = AtomicU64::new(0);
const NCELLS: usize = 120;
static mut CELL_BUF: [[u8; 96]; NCELLS] = [[0u8; 96]; NCELLS];
static ICON_SDK: AtomicU64 = AtomicU64::new(0);
static ICON_FB: AtomicU64 = AtomicU64::new(0);

/// 미출시 후보 목록(소문자 id, 정렬) + 재계산 시그니처.
static CAND: Mutex<Vec<String>> = Mutex::new(Vec::new());
static CAND_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
static CAND_FORCE: AtomicBool = AtomicBool::new(false);
/// 제외 선택 상태(소문자 id).
static SEL: Mutex<Option<HashSet<String>>> = Mutex::new(None);
/// 로드한 설정에 '*' 가 있었는지(저장 시 유지 판단).
static HAD_STAR: AtomicBool = AtomicBool::new(false);

// ── 편의 필터(v0.3.0 — pos_lock 동형): 클래스 드롭다운 + 이름 검색 ──
/// 클래스 라벨(드롭다운 옵션 순서 = CLASS_SEL 인덱스, 0=전체).
/// ⚠드롭다운 옵션은 런타임 ABI 주입이라 i18n 태그 해석이 보장되지 않음 → 한/영 병기 리터럴.
const CLASS_LABELS: [&str; 6] = [
    "전체/All", "전사/Melee", "원거리/Range", "마법사/Magic", "보조/Util", "암살자/Assassin",
];
static CLASS_SEL: AtomicUsize = AtomicUsize::new(0);
static SEARCH_TXT: Mutex<String> = Mutex::new(String::new());
static SEARCH_CLEAR: AtomicBool = AtomicBool::new(false);
static DD_INIT: AtomicBool = AtomicBool::new(false);
static CLASS_DD: ui_kit::NativeDropdown = ui_kit::NativeDropdown::new("cx_class_filter");
/// 필터 적용 후 실제 그리드에 보이는 목록 — 셀 인덱스 ↔ 챔피언 대응의 단일 출처
/// (그리드와 클릭 라우트가 같은 목록을 봐야 엉뚱한 챔프가 토글되지 않는다 — pos_lock 교훈).
static VISIBLE: Mutex<Vec<String>> = Mutex::new(Vec::new());
/// 후보 id → 클래스 인덱스(0=전사 … 4=암살자). recompute_candidates 가 채움.
static CAT_MAP: Mutex<Option<std::collections::HashMap<String, u8>>> = Mutex::new(None);

// ── 한글 이름(검색·정렬용) — pos_lock 동형: champion.i18n 파싱(base/mods/workshop 병합) ──
static KR_MAP: std::sync::OnceLock<std::collections::HashMap<String, String>> =
    std::sync::OnceLock::new();
fn game_dir() -> Option<PathBuf> {
    let mut buf = [0u16; 512];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), 512) } as usize;
    if n == 0 || n >= 512 {
        return None;
    }
    let exe = String::from_utf16_lossy(&buf[..n]);
    Some(std::path::Path::new(&exe).parent()?.to_path_buf())
}
/// champion.i18n 텍스트에서 <lang>.description.<id>.name 을 뽑아 out 에 병합.
fn parse_lang_names(text: &str, lang: &str, out: &mut std::collections::HashMap<String, String>) {
    let anchor = format!("\"{lang}\"");
    let Some(ko) = text.find(&anchor) else { return };
    let Some(drel) = text[ko..].find("\"description\"") else { return };
    let dpos = ko + drel;
    let Some(obr) = text[dpos..].find('{') else { return };
    let start = dpos + obr;
    let b = text.as_bytes();
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
    let seg = &text[start..end.min(text.len())];
    let sb = seg.as_bytes();
    let (mut d, mut i) = (0usize, 0usize);
    let mut cur_id: Option<String> = None;
    let mut last_key = String::new();
    while i < sb.len() {
        match sb[i] {
            b'"' => {
                let mut j = i + 1;
                let mut e = false;
                while j < sb.len() {
                    let c = sb[j];
                    if e {
                        e = false;
                    } else if c == b'\\' {
                        e = true;
                    } else if c == b'"' {
                        break;
                    }
                    j += 1;
                }
                let raw = &seg[i + 1..j.min(seg.len())];
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
                    cur_id = Some(last_key.clone());
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
fn load_kr_names() -> std::collections::HashMap<String, String> {
    load_names("ko")
}
fn load_names(lang: &str) -> std::collections::HashMap<String, String> {
    let mut out = std::collections::HashMap::new();
    let Some(g) = game_dir() else { return out };
    let mut files: Vec<PathBuf> = Vec::new();
    files.push(g.join("bundle_unpacked_full").join("text").join("champion.i18n"));
    if let Ok(rd) = std::fs::read_dir(g.join("mods")) {
        for e in rd.flatten() {
            for sub in ["text", "data"] {
                files.push(e.path().join(sub).join("champion.i18n"));
            }
        }
    }
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
/// 챔프 id → 한글 이름(없으면 id 그대로).
fn kr_name(id: &str) -> String {
    KR_MAP
        .get_or_init(load_kr_names)
        .get(&id.to_ascii_lowercase())
        .cloned()
        .unwrap_or_else(|| id.to_string())
}
/// 챔프 id → 영문 이름(검색 매칭용 — 영어 로케일 유저 대비). 없으면 빈 문자열.
static EN_MAP: std::sync::OnceLock<std::collections::HashMap<String, String>> =
    std::sync::OnceLock::new();
fn en_name(id: &str) -> String {
    EN_MAP
        .get_or_init(|| load_names("en"))
        .get(&id.to_ascii_lowercase())
        .cloned()
        .unwrap_or_default()
}

// 챔피언 스프라이트 에셋 키 테이블(champ_uv.rs — pos_lock 08-20 생성본 사본, 97 id).
include!(r"C:\tfm2mods\tfm2_champion_exclude\assets\champ_uv.rs");

/// ImageRunner 커스텀 UV: +0xa4 flag=1, +0xa8..0xb4 = 4개 f32.
unsafe fn set_img_uv(dp: usize, a: f32, b: f32, c: f32, d: f32) {
    *((dp + 0xa4) as *mut u8) = 1;
    *((dp + 0xa8) as *mut f32) = a;
    *((dp + 0xac) as *mut f32) = b;
    *((dp + 0xb0) as *mut f32) = c;
    *((dp + 0xb4) as *mut f32) = d;
}
fn set_node_wh(node: &Node, w: f32, h: f32) {
    let na = node as *const Node as usize;
    for off in [0x74usize, 0xf4, 0x174, 0x1f4, 0x248, 0x258] {
        ui_kit::runner_wr_f32(na, off, w);
    }
    for off in [0x7cusize, 0xfc, 0x17c, 0x1fc, 0x24c, 0x25c] {
        ui_kit::runner_wr_f32(na, off, h);
    }
}
fn set_node_h(node: &Node, h: f32) {
    let na = node as *const Node as usize;
    for off in [0x7cusize, 0xfc, 0x17c, 0x1fc, 0x24c, 0x25c] {
        ui_kit::runner_wr_f32(na, off, h);
    }
}

/// 셀 아이콘 = 챔피언 전신 스프라이트 (pos_lock set_cell_icon 동형).
/// ①캡처된 로드-assets + 게임 아이콘 세터 직접 호출 ②bundle UV 재현 폴백(icon_data).
unsafe fn set_cell_icon(icon: &mut Node, k: usize, lower: &str) {
    if k >= NCELLS {
        return;
    }
    let ga = GAME_ASSETS.load(Ordering::Relaxed);
    let base = MOD_BASE.load(Ordering::Relaxed);
    if base != 0 && (0x10000..1usize << 48).contains(&ga) {
        let node_ptr = icon as *mut Node as usize;
        let id_ptr = lower.as_ptr() as usize;
        let id_len = lower.len();
        let f: extern "C" fn(usize, usize, usize, usize, f32, f32, f32) =
            core::mem::transmute(base + RVA_ICON_SETTER);
        let ok = catch_unwind(AssertUnwindSafe(|| {
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
            set_node_wh(icon, uv.w, uv.h);
            return;
        }
    }
    // 둘 다 실패 → 깨짐 대신 아이콘 숨김(이름은 표시).
    *((dp + 0xa4) as *mut u8) = 0;
    core::ptr::write_unaligned((dp + 0x10) as *mut u64, 0u64);
}

/// 미출시 후보 재계산: 슈퍼셋(champ_uv ∪ mod_champions ∪ 세이브설정 ∪ seen)을
/// "registry 등재(champion_info Some 또는 mod_champions) && available 아님" 으로 검증.
/// = 패치데이 후보 빌더(registry − available)의 UI 근사.
/// (db 타입명을 시그니처에 박지 않으려고 필요한 조각만 받는다 — SDK 타입명 비의존.)
fn recompute_candidates(
    avail_ids: &[String],
    mod_ids_raw: &[String],
    in_registry: impl Fn(&str) -> bool,
    cat_of: impl Fn(&str) -> Option<u8>,
) {
    let avail_n = avail_ids.len();
    if avail_n == 0 {
        return; // 목록 미로드 프레임 — 전원 미출시로 오판 방지
    }
    let modn = mod_ids_raw.len();
    let sig = ((avail_n as u64) << 32) ^ ((modn as u64) << 8) ^ 1;
    if CAND_SIG.load(Ordering::Relaxed) == sig && !CAND_FORCE.swap(false, Ordering::Relaxed) {
        return;
    }
    let avail: HashSet<String> = avail_ids.iter().map(|s| s.to_ascii_lowercase()).collect();
    let mod_ids: HashSet<String> = mod_ids_raw.iter().map(|s| s.to_ascii_lowercase()).collect();
    let mut superset: HashSet<String> = CHAMP_KEY.iter().map(|e| e.0.to_string()).collect();
    superset.extend(mod_ids.iter().cloned());
    superset.extend(load_seen());
    let (excl_list, _, _) = effective_exclusion(); // 세이브 설정에만 있는 id 도 목록에 노출
    superset.extend(excl_list);
    let mut cand: Vec<String> = superset
        .into_iter()
        .filter(|id| !avail.contains(id) && (mod_ids.contains(id) || in_registry(id)))
        .collect();
    // 한글 이름 가나다순(이름 없는 챔프는 id 순으로 뒤) — pos_lock sorted_champs 동형.
    cand.sort_by(|a, b| kr_name(a).cmp(&kr_name(b)).then_with(|| a.cmp(b)));
    // 클래스 맵(드롭다운 필터용).
    let mut cats = std::collections::HashMap::new();
    for id in &cand {
        if let Some(c) = cat_of(id) {
            cats.insert(id.clone(), c);
        }
    }
    *CAT_MAP.lock().unwrap_or_else(|e| e.into_inner()) = Some(cats);
    let n = cand.len();
    *CAND.lock().unwrap_or_else(|e| e.into_inner()) = cand;
    if CAND_SIG.swap(sig, Ordering::Relaxed) != sig {
        log(&format!("후보 재계산: 미출시 {n}개 (출시 {avail_n} · 모드챔프 {modn})"));
    }
    GRID_SIG.store(u64::MAX, Ordering::Relaxed);
}

/// 현재 세이브 설정 → 선택 상태 로드(팝업 열릴 때).
fn load_selection() {
    let (list, star, src) = effective_exclusion();
    log(&format!("선택 로드: {}개 (출처={}{})", list.len(), src, if star { "·*" } else { "" }));
    HAD_STAR.store(star, Ordering::Relaxed);
    let cand = CAND.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let sel: HashSet<String> = if star {
        cand.iter().cloned().collect()
    } else {
        let cs: HashSet<&String> = cand.iter().collect();
        list.into_iter().filter(|e| cs.contains(e)).collect()
    };
    *SEL.lock().unwrap_or_else(|e| e.into_inner()) = Some(sel);
    SEL_VER.fetch_add(1, Ordering::Relaxed);
}

/// 선택 상태 → **현재 세이브의 mod save data 에 저장**(확인 버튼, v0.4.0).
/// - 후보가 아닌 기존 항목(이미 출시됐거나 수동 기입)은 그대로 보존.
/// - '*' 는 "원래 있었고 여전히 전부 선택"일 때만 유지, 아니면 명시 목록으로 전환.
/// - 클릭 콜백엔 ClientData 가 없어 본문을 PENDING_SAVE 로 이월 → post_update 가 기록.
///   (v0.4.2: 저장처 = 세이브 단일 — cfg 파일 없음.)
fn save_selection() {
    let cand = CAND.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let sel = match SEL.lock().unwrap_or_else(|e| e.into_inner()).clone() {
        Some(s) => s,
        None => return,
    };
    let (prev, _star, _src) = effective_exclusion();
    let cand_set: HashSet<&String> = cand.iter().collect();
    let mut foreign: Vec<String> = prev.into_iter().filter(|e| !cand_set.contains(e)).collect();
    foreign.sort();
    foreign.dedup();
    let all_selected = !cand.is_empty() && sel.len() == cand.len();
    let keep_star = HAD_STAR.load(Ordering::Relaxed) && all_selected;
    let mut out = String::new();
    if keep_star {
        out.push_str("*\n");
    } else {
        for c in &cand {
            if sel.contains(c) {
                out.push_str(c);
                out.push('\n');
            }
        }
    }
    for f in &foreign {
        out.push_str(f);
        out.push('\n');
    }
    *PENDING_SAVE.lock().unwrap_or_else(|e| e.into_inner()) = Some(out);
    log(&format!(
        "저장 요청: 제외 {}개{}{} → 이 세이브(mod save data) 기록 대기",
        sel.len(),
        if keep_star { " ('*' 유지)" } else { "" },
        if foreign.is_empty() { String::new() } else { format!(" + 보존 {}개", foreign.len()) }
    ));
}

/// 팝업 .ui 런타임 생성(틀 = pos_lock_popup 동형 — 탭/검색 없이 그리드+우측 패널).
pub(crate) fn build_popup_ui() -> String {
    let mut s = String::with_capacity(96 * 1024);
    s.push_str(
        r##"champ_excl_popup:color {
  width: 1763px;
  height: 1003px;
  anchor_x: 0.5;
  pivot_x: 0.5;
  anchor_y: 0.5;
  pivot_y: 0.5;
  visible: false;
  color: #161721ff;
  rounding: Uniform { rounding: 12; }

  #header:label {
    @"asset/base/style/main#bold_label";
    x: 32px; y: 24px; width: 900px; height: 42px; size: 24; align_y: Center;
    text: "#asset/base/text/ui?champ_excl.title";
  }

  #close:button {
    width: 16px; height: 16px; anchor_x: 1; pivot_x: 1; x: -32px; y: 32px;
    source: "asset/base/ui/icons/cross"; color: #c2c6ceff;
    hover: { color: #e8e8e8ff; } active: { color: #e8e8e8ff; }
  }

  #cx_filter_bar:empty {
    x: 32px; y: 81px; width: 700px; height: 40px;

    #cx_class_filter:dropdown {
      @"asset/base/style/main#dropdown";
      width: 150px; height: 40px;
      max_items_height: 280;
      text: { size: 17; }
      item_text: { size: 17; }
      text_layout: { x: 14px; y: 6px; width: 100%; height: 28px; }
      item_layout: { height: 40px; width: 145.5px; x: 14px; }
      item_text_layout: { y: 6px; width: 145.5px; height: 28px; }
    }

    #cx_champ_search:text_edit {
      @"asset/base/style/main#text_edit";
      x: 158px; width: 200px; height: 40px;
      size: 16; align_y: Center;
      padding: { left: 44px; top: 5px; right: 15px; bottom: 5px; }
      placeholder: "챔피언 검색 / Search...";
      max_length: 40;

      #icon:image {
        ignore_event: true;
        x: -30px; y: 5px; width: 20px; height: 20px;
        source: "asset/base/ui/banpick/fi-rr-search";
        color: #858d9dff;
      }
    }

    #cx_search_clear:color_icon_button {
      @"asset/base/style/main#tertiary_button";
      x: 366px; width: 40px; height: 40px;
      icon: { source: "asset/base/ui/icons/cross"; rect: { x: 12; y: 12; w: 16; h: 16; } }
    }

    #cx_filter_count:label {
      @"asset/base/style/main#label";
      x: 420px; width: 240px; height: 40px; size: 15; align_y: Center;
      color: #858d9dff;
    }
  }

  #left:color {
    x: 32px; y: 136px; width: 1207px; height: 787px; color: #1d1f2cff;
    rounding: Uniform { rounding: 8; }

    #scroll:scroll_view {
      x: 16px; y: 16px; width: 1175px; height: 755px; speed: 100; bar_width: 4;
      bar_padding: { top: 8px; bottom: 8px; }
      bar: { source: "asset/base/sprite/white"; color: #37d5b3ff; hover: { color: #ecfbf8ff; } }
      back: { source: "asset/base/sprite/white"; color: #4a4c56ff; }

      #contents:empty {
        x: 8px; y: 8px; width: 1154px; height: 1900px;
        child_type: Table { spacing_x: 15px; spacing_y: 15px; }
"##,
    );
    for k in 0..NCELLS {
        s.push_str(&format!(
            r##"        #cx_cell{k}:color_icon_button {{ @"asset/base/style/main#tertiary_button"; width: 152px; height: 171px; visible: false;
          #icon:image {{ anchor_x: 0.5; pivot_x: 0.5; pivot_y: 1; x: 0px; y: 122px; width: 84px; height: 84px; ignore_event: true; }}
          #name:label {{ @"asset/base/style/main#label"; x: 2px; y: 132px; width: 148px; height: 22px; size: 13; align_x: Center; align_y: Center; ignore_event: true; }}
          #sel:color {{ visible: false; x: 0px; y: 0px; width: 152px; height: 171px; back_color: #00000000; color: #ff5c5cff; stroke: 3; rounding: Uniform {{ rounding: 8; }} ignore_event: true; }}
        }}
"##
        ));
    }
    s.push_str(
        r##"      }
    }
  }

  #right:color {
    x: 1254px; y: 136px; width: 477px; height: 787px; color: #1d1f2cff;
    rounding: Uniform { rounding: 8; }

    #summary:label { @"asset/base/style/main#bold_label"; x: 24px; y: 24px; width: 429px; height: 28px; size: 18; align_y: Center; text: "#asset/base/text/ui?champ_excl.summary"; }
    #hint:label { @"asset/base/style/main#label"; x: 24px; y: 68px; width: 429px; height: 130px; size: 16; line_height: 26; align_y: Center; text: "#asset/base/text/ui?champ_excl.hint"; }
    #cnt_total_k:label { @"asset/base/style/main#label"; x: 24px; y: 218px; width: 290px; height: 24px; size: 16; align_y: Center; text: "#asset/base/text/ui?champ_excl.lbl_total"; }
    #cnt_total_v:label { @"asset/base/style/main#label"; x: 320px; y: 218px; width: 133px; height: 24px; size: 16; align_y: Center; }
    #cnt_sel_k:label { @"asset/base/style/main#bold_label"; x: 24px; y: 246px; width: 290px; height: 26px; size: 16; align_y: Center; text: "#asset/base/text/ui?champ_excl.lbl_sel"; }
    #cnt_sel_v:label { @"asset/base/style/main#bold_label"; x: 320px; y: 246px; width: 133px; height: 26px; size: 16; align_y: Center; }
    #cx_src:label { @"asset/base/style/main#label"; x: 24px; y: 276px; width: 429px; height: 44px; size: 14; line_height: 20; color: #858d9dff; align_y: Center; }
    #note_all:label { @"asset/base/style/main#label"; x: 24px; y: 324px; width: 429px; height: 100px; size: 15; line_height: 24; color: #ffb84aff; align_y: Center; }

    #cx_none:color_icon_button { @"asset/base/style/main#tertiary_button"; x: 24px; y: 470px; width: 429px; height: 40px; text: { text: "#asset/base/text/ui?champ_excl.btn_none"; font: "asset/base/font/set/bold"; size: 17; align_x: Center; align_y: Center; } }
    #cx_all:color_icon_button { @"asset/base/style/main#tertiary_button"; x: 24px; y: 522px; width: 429px; height: 40px; text: { text: "#asset/base/text/ui?champ_excl.btn_all"; font: "asset/base/font/set/bold"; size: 17; align_x: Center; align_y: Center; } }
  }

  #cancel:color_icon_button { @"asset/base/style/main#tertiary_button"; x: 654px; y: 943px; width: 220px; height: 40px; text: { text: "#asset/base/text/ui?champ_excl.cancel"; font: "asset/base/font/set/bold"; size: 18; align_x: Center; align_y: Center; } }
  #ok:color_icon_button { @"asset/base/style/main#tertiary_button"; x: 890px; y: 943px; width: 220px; height: 40px; text: { text: "#asset/base/text/ui?champ_excl.ok"; font: "asset/base/font/set/bold"; size: 18; align_x: Center; align_y: Center; } }
}
"##,
    );
    s
}

/// 팝업 그리드/라벨 갱신(변경 시에만).
fn fill_grid(root: &mut Node) {
    let cand = CAND.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let selected: HashSet<String> = SEL
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .clone()
        .unwrap_or_default();
    // ── 편의 필터(클래스 + 이름 검색) — 그리드와 클릭 라우트가 같은 VISIBLE 을 본다 ──
    let class_sel = CLASS_SEL.load(Ordering::Relaxed);
    let search = SEARCH_TXT.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let visible: Vec<String> = cand
        .iter()
        .filter(|c| {
            if class_sel > 0 {
                let want = (class_sel - 1) as u8;
                let g = CAT_MAP.lock().unwrap_or_else(|e| e.into_inner());
                match g.as_ref().and_then(|m| m.get(*c)).copied() {
                    Some(v) if v == want => {}
                    _ => return false,
                }
            }
            if !search.is_empty() {
                let kr = kr_name(c).to_ascii_lowercase();
                let en = en_name(c).to_ascii_lowercase();
                if !kr.contains(&search) && !en.contains(&search) && !c.contains(&search) {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();
    *VISIBLE.lock().unwrap_or_else(|e| e.into_inner()) = visible.clone();
    let ready = icon_data::READY.load(Ordering::Relaxed) as u64;
    let filter_sig = {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        (class_sel, &search).hash(&mut h);
        h.finish()
    };
    let from_save = SAVE_EXCL.lock().unwrap_or_else(|e| e.into_inner()).is_some() as u64;
    let sig = SEL_VER.load(Ordering::Relaxed)
        ^ ((cand.len() as u64) << 32)
        ^ ((visible.len() as u64) << 16)
        ^ (ready << 47)
        ^ (from_save << 46)
        ^ filter_sig
        ^ CAND_SIG.load(Ordering::Relaxed).rotate_left(17);
    if GRID_SIG.swap(sig, Ordering::Relaxed) == sig {
        return;
    }
    let Some(pop) = ui_kit::find_mut(root, "champ_excl_popup") else {
        return;
    };
    // 숫자 라벨은 값만(키 라벨은 .ui 의 i18n 태그가 담당 — 게임 언어 자동 해석)
    if let Some(n) = ui_kit::find_mut(pop, "cnt_total_v") {
        ui_kit::label_set(n, &cand.len().to_string());
    }
    if let Some(n) = ui_kit::find_mut(pop, "cnt_sel_v") {
        ui_kit::label_set(n, &selected.len().to_string());
    }
    if let Some(n) = ui_kit::find_mut(pop, "cx_src") {
        let from_save = SAVE_EXCL.lock().unwrap_or_else(|e| e.into_inner()).is_some();
        ui_kit::label_set(n, &format!("{I18N}{}", if from_save { "src_save" } else { "src_none" }));
    }
    if let Some(n) = ui_kit::find_mut(pop, "note_all") {
        let s = if cand.is_empty() {
            format!("{I18N}note_none")
        } else if selected.len() == cand.len() {
            format!("{I18N}note_all")
        } else {
            String::new()
        };
        ui_kit::label_set(n, &s);
    }
    if let Some(n) = ui_kit::find_mut(pop, "cx_filter_count") {
        let s = if visible.len() == cand.len() {
            String::new()
        } else {
            format!("{} / {}", visible.len(), cand.len())
        };
        ui_kit::label_set(n, &s);
    }
    let Some(contents) = ui_kit::find_mut(pop, "contents") else {
        return;
    };
    ICON_SDK.store(0, Ordering::Relaxed);
    ICON_FB.store(0, Ordering::Relaxed);
    for (k, cell) in contents.child.iter_mut().enumerate() {
        if let Some(champ) = visible.get(k) {
            cell.visible = true;
            let lower = champ.clone(); // 이미 소문자
            let listed = selected.contains(&lower);
            for c in cell.child.iter_mut() {
                match c.id.as_str() {
                    "name" => {
                        // i18n 태그 → 게임이 로케일 표시명으로 자동 해석(모드챔프 포함).
                        ui_kit::label_set(
                            c,
                            &format!("#asset/base/text/champion?description.{lower}.name"),
                        );
                    }
                    "sel" => c.visible = listed,
                    "icon" => unsafe {
                        set_cell_icon(c, k, &lower);
                    },
                    _ => {}
                }
            }
        } else {
            cell.visible = false;
        }
    }
    // 스크롤 컨텐츠 높이 = 행 수 기준(셀 152×171·간격 15·7열 — pos_lock 동일 공식).
    let n = visible.len().min(NCELLS);
    let rows = n.div_ceil(7);
    let h = (rows as f32) * (171.0 + 15.0) + 16.0;
    set_node_h(contents, h);
}

fn toggle_sel(id: &str) {
    let mut g = SEL.lock().unwrap_or_else(|e| e.into_inner());
    let set = g.get_or_insert_with(HashSet::new);
    if !set.remove(id) {
        set.insert(id.to_string());
    }
    SEL_VER.fetch_add(1, Ordering::Relaxed);
}

// ── 진입 ──────────────────────────────────────────────────────────────────
struct ChampExclExt;

impl ModExtension for ChampExclExt {
    fn post_update(&self, scene: &mut Scene, ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        let _ = catch_unwind(AssertUnwindSafe(|| {
            // 미출시 후보 갱신(관리화면 = Scene::InGame).
            if let Scene::InGame { data } = scene {
                // ── 세이브별 설정 (v0.4.0) ──
                // ①UI 확인이 이월한 기록 대기분을 이 세이브의 mod save data 에 기록.
                let pending = PENDING_SAVE.lock().unwrap_or_else(|e| e.into_inner()).take();
                if let Some(body) = pending {
                    if data.can_write_mod_save() {
                        data.mod_save_set_version(MOD_ID, SAVE_NS_VERSION);
                        let ok = data.mod_save_set_string(MOD_ID, SAVE_KEY, &body);
                        log(&format!(
                            "세이브 기록 {}: {}B (mod_save_set_string)",
                            if ok { "OK" } else { "거부(FALSE)" },
                            body.len()
                        ));
                        if ok {
                            // ★즉시 캐시 반영 — 엔진 쓰기는 "큐잉"이라 mod_save_get 이 다음
                            //   데이터 동기까지 옛값(None/구값)을 줄 수 있다(v0.4.0 실측:
                            //   기록 OK 15초 뒤 재열람이 cfg기본값으로 로드 = 방금 저장을
                            //   되돌릴 수 있는 함정). 로컬 캐시를 기록 본문으로 선반영한다.
                            *SAVE_EXCL.lock().unwrap_or_else(|e| e.into_inner()) =
                                Some(parse_exclude_text(&body));
                        }
                    } else {
                        log("세이브 기록 불가: can_write_mod_save=false (멀티 비호스트?) — 이번 선택은 저장 안 됨");
                    }
                }
                // ②세이브의 현행 설정을 캐시(패치데이 detour 가 이걸 읽음 — SDK 컨텍스트 없음).
                //   ★None 이어도 기존 캐시를 지우지 않는다 — 큐잉 지연 동안 로컬 선반영값 유지.
                //   (세이브 전환은 메인메뉴 경유 = 비-InGame 프레임의 else 절이 클리어해 오염 없음.)
                if let Some(t) = data.mod_save_get_string(MOD_ID, SAVE_KEY) {
                    *SAVE_EXCL.lock().unwrap_or_else(|e| e.into_inner()) =
                        Some(parse_exclude_text(&t));
                }
                let db = data.db();
                let avail: Vec<String> = db.available_champions.clone();
                let mod_ids: Vec<String> = db
                    .champion_info_sheet
                    .mod_champions
                    .iter()
                    .map(|e| e.id.clone())
                    .collect();
                let cat_idx = |c: &ChampionCategory| -> u8 {
                    match c {
                        ChampionCategory::Melee => 0,
                        ChampionCategory::Range => 1,
                        ChampionCategory::Magician => 2,
                        ChampionCategory::Util => 3,
                        ChampionCategory::Assassin => 4,
                    }
                };
                let mod_cat: std::collections::HashMap<String, u8> = db
                    .champion_info_sheet
                    .mod_champions
                    .iter()
                    .map(|e| (e.id.to_ascii_lowercase(), cat_idx(&e.category)))
                    .collect();
                recompute_candidates(
                    &avail,
                    &mod_ids,
                    |id| db.champion_info(id).is_some(),
                    |id| {
                        mod_cat
                            .get(id)
                            .copied()
                            .or_else(|| db.champion_info(id).map(|c| cat_idx(&c.category())))
                    },
                );
            } else {
                // 세이브 밖 화면(메인메뉴 등): 이전 세이브 설정이 다른 세이브에 적용되지 않게 캐시 해제.
                *SAVE_EXCL.lock().unwrap_or_else(|e| e.into_inner()) = None;
            }
            // UI 주입(로더 체인 훅 — 늦설치·매 프레임 재시도 가드).
            inject::install();
            // 게임플레이 탭이 보일 때만 행 표시(같은 탭 banpick_style 의 visible 을 따라감).
            let gp_vis = ui_kit::find(&ui.root, "banpick_style")
                .map(|n| n.visible)
                .unwrap_or(false);
            if let Some(row) = ui_kit::find_mut(&mut ui.root, "champ_excl_row") {
                row.visible = gp_vis;
            }
            // 팝업 표시/숨김.
            let open = POPUP_OPEN.load(Ordering::Relaxed);
            let present = ui_kit::find_mut(&mut ui.root, "champ_excl_popup").is_some();
            if present {
                // 아이콘 UV 백그라운드 로드 = 환경설정 노드가 트리에 있을 때만 1회 착수
                //   (시작 로딩 중 bundle IO 경합 = 검은 화면, pos_lock 08-20 실사고).
                icon_data::start_load();
            }
            if !present {
                POPUP_OPEN.store(false, Ordering::Relaxed);
            } else if let Some(pop) = ui_kit::find_mut(&mut ui.root, "champ_excl_popup") {
                pop.visible = open;
            }
            // ── 편의 필터 위젯 배선(클래스 드롭다운 + 이름 검색 — pos_lock 동형) ──
            if open && present {
                // ①옵션 주입 1회 — ⚠ABI 호출이라 **팝업이 실제로 열린 뒤에만** 부른다.
                if !DD_INIT.load(Ordering::Relaxed)
                    && CLASS_DD.set_options(&ui.root, &CLASS_LABELS, 0)
                {
                    DD_INIT.store(true, Ordering::Relaxed);
                    log("filter: 클래스 드롭다운 옵션 주입 OK");
                }
                // ②선택 인덱스 폴링(게임이 클릭 시 runner 에 기록).
                if let Some(sel) = CLASS_DD.selected(&ui.root) {
                    let sel = sel.min(CLASS_LABELS.len() - 1);
                    if CLASS_SEL.swap(sel, Ordering::Relaxed) != sel {
                        GRID_SIG.store(u64::MAX, Ordering::Relaxed);
                    }
                }
                // ③검색어(비우기 버튼이 눌렸으면 먼저 지운다).
                if SEARCH_CLEAR.swap(false, Ordering::Relaxed) {
                    if let Some(n) = ui_kit::find_mut(&mut ui.root, "cx_champ_search") {
                        ui_kit::textedit_set(n, "");
                    }
                }
                let cur_txt = ui_kit::find(&ui.root, "cx_champ_search")
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
                if LOAD_SEL_REQ.swap(false, Ordering::Relaxed) {
                    load_selection();
                }
                fill_grid(&mut ui.root);
            }

            // 클릭 라우팅(id 는 전부 모드 접두 cx_/champ_excl_ — 타 모드와 충돌 없음).
            let mut routes: Vec<(String, ui_kit::ClickFn)> = Vec::with_capacity(NCELLS + 8);
            routes.push(ui_kit::route(
                "champ_excl_configure",
                Rc::new(|| {
                    POPUP_OPEN.store(true, Ordering::Relaxed);
                    CAND_FORCE.store(true, Ordering::Relaxed);
                    LOAD_SEL_REQ.store(true, Ordering::Relaxed);
                    GRID_SIG.store(u64::MAX, Ordering::Relaxed);
                    log("추가 챔피언 설정 버튼 클릭");
                }),
            ));
            let close: ui_kit::ClickFn = Rc::new(|| POPUP_OPEN.store(false, Ordering::Relaxed));
            routes.push(ui_kit::route("champ_excl_popup.close", close.clone()));
            routes.push(ui_kit::route("champ_excl_popup.cancel", close));
            routes.push(ui_kit::route(
                "champ_excl_popup.ok",
                Rc::new(|| {
                    save_selection();
                    POPUP_OPEN.store(false, Ordering::Relaxed);
                }),
            ));
            routes.push(ui_kit::route(
                "cx_all",
                Rc::new(|| {
                    let cand = CAND.lock().unwrap_or_else(|e| e.into_inner()).clone();
                    *SEL.lock().unwrap_or_else(|e| e.into_inner()) =
                        Some(cand.into_iter().collect());
                    SEL_VER.fetch_add(1, Ordering::Relaxed);
                }),
            ));
            routes.push(ui_kit::route(
                "cx_search_clear",
                Rc::new(|| SEARCH_CLEAR.store(true, Ordering::Relaxed)),
            ));
            routes.push(ui_kit::route(
                "cx_none",
                Rc::new(|| {
                    *SEL.lock().unwrap_or_else(|e| e.into_inner()) = Some(HashSet::new());
                    SEL_VER.fetch_add(1, Ordering::Relaxed);
                }),
            ));
            for k in 0..NCELLS {
                routes.push(ui_kit::route(
                    &format!("cx_cell{k}"),
                    Rc::new(move || {
                        // ★필터된 목록(VISIBLE)을 본다 — 그리드와 같은 출처가 아니면
                        //   필터 중 엉뚱한 챔프가 토글된다(pos_lock 교훈).
                        let id = VISIBLE
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .get(k)
                            .cloned();
                        if let Some(id) = id {
                            toggle_sel(&id);
                        }
                    }),
                ));
            }
            ui_kit::ensure_clicks(ui, &CLICK_LAST, routes);
        }));
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    // src=file!() — build_inj.ps1 신원 검증(dll 내 소스 절대경로 문자열) 요구
    log(&format!("mod init v{VERSION} (src={}) — 설정 = 세이브별(mod save data) 단일", file!()));
    let r = catch_unwind(AssertUnwindSafe(|| unsafe { install_hook() }));
    match r {
        Ok(Ok(m)) => log(&format!("HOOK OK: {}", m)),
        Ok(Err(e)) => log(&format!("HOOK FAIL: {}", e)),
        Err(_) => log("HOOK FAIL: panic in install_hook"),
    }
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(ChampExclExt);
    reg
}
declare_mod!(init);
