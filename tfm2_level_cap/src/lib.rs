// tfm2_level_cap — 인게임 챔피언 최대 레벨 확장 (12 → MY_LEN+1)
// =====================================================================================
// 배경 (RE 2026-07-23 @0.5.2 buildid 24310934 / 재핀 2026-07-31 @0.5.3 buildid 24451609):
//   인게임 최대 레벨 = `need_exp.len() + 1`. exe에 하드코딩 상한 상수는 **없다**.
//   레벨업 함수 0.5.4 `0x14ec9e0`(0.5.3 `0x12c56d0` / 0.5.2 `0x22d3c60`) 내부:
//     0x14ece4c  mov r14,[r12+0x870]   ; level (u64, 초기 1)   [0.5.3: +0x880 / 0.5.2: rdi,[r13+0x880]]
//     0x14ece54  mov rdx,[rax+0xd10]   ; need_exp.len   ← ★여기를 후킹
//     0x14ece5b  cmp r14,rdx
//     0x14ece5e  ja  ...               ; level > len → 조용히 return (패닉 아님)
//     0x14ece68  cmp rcx,rdx / jae ... ; Rust bounds panic (위 ja 때문에 도달불가=가드)
//     0x14ece71  mov rdx,[rax+0xd08]   ; need_exp.ptr
//     0x14ece80  sub rcx,[rdx+r14*8-8] ; exp -= need_exp[level-1]
//   ★0.5.3 변화 = **GameSetting 베이스 레지스터 r14 → rax**(level 홀더도 r13 → r12).
//     ⟹ 레벨업 스텁은 rax를 파괴하면 안 된다(구 스텁은 rax를 스크래치로 썼음 = 그대로 쓰면 즉사).
//     UI 스텁은 0.5.2와 동일하게 rax 베이스라 무수정.
//   0.5.4 변화 = **레지스터 할당 무변화**(rax/r14/r12/rcx 그대로, 명령 바이트도 동일).
//     바뀐 것은 챔프 구조체 필드 오프셋뿐 = level +0x880→+0x870 / exp +0x870→+0x860 (스텁 무관).
//   ★0.5.5 변화(재핀 2026-08-12) = **레벨업 경로 GameSetting 베이스 rax → r14 회귀**(0.5.2와 동일 패턴).
//     레벨업 함수 0.5.5 `0x14d7e10` 내부 (0.5.4 `0x14ec9e0`):
//       0x14d8193  mov rdi,[r13+0x990]   ; level  (홀더 r12→r13, +0x870→+0x990, 레지스터 r14→rdi)
//       0x14d819a  mov rdx,[r14+0xd10]   ; need_exp.len  ← ★후킹 사이트 (베이스 rax→r14!)
//       0x14d81a1  cmp rdi,rdx / ja ...  ; 조용히 return
//       0x14d81b7  mov rcx,[r14+0xd08]   ; ptr (rdx→rcx로 변경)
//       0x14d81be  mov rax,[r13+0x980]   ; exp
//       0x14d81c5  sub rax,[rcx+rdi*8-8]
//     ⟹ 레벨업 스텁의 GameSetting 접근 베이스만 rax→r14로 인코딩 교체(스텁은 rdx·r11만 파괴 = 그대로 안전).
//     UI 경로는 0.5.5에서도 rax 베이스·rcx=index·바이트 완전 동일 = UI 스텁 무수정.
//   GameSetting: +0xd00=cap / +0xd08=ptr / +0xd10=len (0.5.2 = 0.5.3 = 0.5.4 = 0.5.5 **불변 실측**)
//
// ⛔ 하면 안 되는 것: `ja @0x22d3ff4`를 NOP으로 뭉개는 바이트패치. 가드가 사라지면 바로 뒤
//   `jae @0x22d4001` bounds panic이 실제 발화 → ud2 하드크래시. len만 늘리는 것도 동일(ptr이
//   원본이라 OOB read). ⟹ **ptr·len·cap을 한 번에 바꿔야만** 안전하다.
//
// ★★ 데이터(mod.override_info merge)와 이 DLL은 "둘 다" 필요하다 — 반드시 같은 값으로 유지할 것.
//   merge는 game_setting에 정상 적용된다(raw 보존 확장자 화이트리스트 포함, 배열은 replace).
//   실측(2026-07-23): UI가 읽는 인스턴스는 merge된 것(len=17 → 트램폴린 patched=0)이지만,
//   **len=11인 별도 인스턴스가 공존**한다(patched 카운터가 계속 증가 = 시뮬이 쓰는 쪽은 merge 미반영).
//   ⟹ merge만으로도, DLL만으로도 부족하다. (구 주석 "merge는 시뮬에 미반영" = 오진, 정정함)
//
//   ⚠⚠ 두 테이블 값이 어긋나면 **경험치 바가 폭주**한다(2026-07-23 실사고):
//     바 = `잔여exp / need_exp[level-1]` (계산 함수 0x803b30, 비율 저장 0x80b723, 노드
//     `champion_tooltip.exp.bar` width=Percent(ratio*100) @0x4f5ab7, 클리핑 없음).
//     분자(잔여 exp)는 시뮬 인스턴스 = 이 DLL의 NEED_EXP 스케일, 분모는 UI 인스턴스 = merge 데이터.
//     진단용으로 데이터만 `[10 × 17]`로 뒀더니 레벨1에서 149/10 = ratio 14.9 → width 1490%
//     ≈ 1907px로 화면 밖까지 뻗었다.
//   ⟹ **static NEED_EXP를 바꾸면 setting/*.game_setting 3벌도 같은 값으로 반드시 함께 갱신**
//     (BOM 없는 UTF-8 유지).
//   참고: cap(+0xd00)을 읽는 UI 코드는 없다(함수 0x803b30 전수 스캔 0건) ⟹ cap=0은 안전한 마커.
//
// 방식: 0x22d3fea(7B)를 `call rel32 + nop nop`으로 바꿔 트램폴린으로 보낸다.
//   트램폴린은 원본 로드를 수행한 뒤, len이 아직 우리 값이 아니면 GameSetting의
//   ptr/len/cap을 우리 정적 배열로 교체하고 rdx에 새 len을 실어 복귀한다.
//   ★cap=0으로 두는 것이 핵심 — Rust Vec::drop은 cap!=0일 때만 dealloc하므로,
//     우리 .rdata 배열이 게임 얼로케이터로 free되는 사고를 원천 차단한다(원본 88B는 leak, 무시 가능).
//   ★멱등 + 병렬 안전: 여러 sim 워커가 동시에 들어와도 같은 값을 쓰므로 무해(8B 정렬 write).
//
// ⚠ 시드 재시뮬레이션 게임이라 적용 시 sim 결과가 바닐라와 달라진다.
// ⚠ byte mismatch면 RVA stale(패치 옴) → 조용히 스킵 + 로그만. /migrate 후 재핀.
//
// 빌드: powershell -File C:\tfm2mods\build_inj.ps1 -Src C:\tfm2mods\tfm2_level_cap\src\lib.rs -ModId tfm2_level_cap
// =====================================================================================

use mod_api::*;
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "tfm2_level_cap";

// ── 설정 (tfm2_level_cap.cfg, DLL과 같은 폴더) ──
// 1번째 유효줄 = 최대 레벨 / 2번째 유효줄 = **12→13부터** 필요한 경험치를 ,로 나열
//   (`#`·`//`로 시작하는 줄과 빈 줄은 무시)
// ★레벨 12까지는 바닐라 값(VANILLA)을 그대로 쓴다. 설정에 적는 건 확장 구간뿐이다.
//   필요 개수 = 최대레벨 - 12. 더 많이 적으면 **끊고**, 모자라면 **마지막 값으로 채운다**.
// 파일이 없으면 기본값으로 새로 만든다.
const CFG_NAME: &str = "tfm2_level_cap.cfg";
const DEF_MAX_LEVEL: u64 = 18;
/// 바닐라 need_exp 11개 = 레벨 1→2 … 11→12. 이 구간은 설정으로 건드리지 않는다.
const VANILLA: [u64; 11] = [150, 250, 300, 450, 600, 900, 1200, 1500, 1800, 2100, 2400];
const VANILLA_MAX_LEVEL: u64 = VANILLA.len() as u64 + 1;   // = 12
/// 확장 구간 기본값(12→13 …). 바닐라 후반부가 +300 등차라 그 패턴을 이어간다.
const DEF_EXT: [u64; 6] = [2700, 3000, 3300, 3600, 3900, 4200];
const MAX_LEVEL_LIMIT: u64 = 500;   // sanity 상한 (스탯은 레벨당 선형 가산이라 과도하면 밸런스 붕괴)

/// 설정에서 만든 테이블. `Box::leak`으로 프로세스 수명 내내 고정한다 —
/// 트램폴린이 이 주소를 imm64로 박아넣으므로 절대 이동/해제되면 안 된다.
/// (해제되지 않는다는 점이 cap=0 정책과도 맞물린다: 게임이 이 배열을 free하지 못한다.)
static mut TABLE_PTR: u64 = 0;
static mut TABLE_LEN: u64 = 0;

// ── 후킹 지점 (0.5.5, 재핀 2026-08-12 / 직전 0.5.4 재핀 2026-08-05) ──
// ★0.5.5 재핀도 아래 ①②를 그대로 돌려 **양 버전 각 1건**만 나왔다(0.5.4 재현 = 방법 검증).
//   GameSetting +0xd08(ptr)/+0xd10(len) = 0.5.5 코드에서 그대로 실측(len 비교→ptr 로드→[ptr+idx*8-8]).
//   +0xd00(cap)은 Vec(cap,ptr,len) 배치상 동반 불변 판단(0.5.2~0.5.4 실측 이력 + ptr/len 무이동).
// ★GameSetting은 시뮬마다 복제된다(로그 실측: patched 카운터가 계속 증가). 따라서 "한 번 고치고 끝"이
//   아니라, need_exp를 읽는 **각 경로에서 매번** 원본(len=11) 인스턴스를 잡아 교체해야 한다.
//   need_exp를 인덱싱하는 지점은 .text 전체에 2곳뿐이고(전수 스캔 확인), 둘 다 여기서 처리한다.
// ★재핀 방법(0.5.2에서 정답 1건만 뽑히는 것으로 방법 검증 후 0.5.3 적용):
//   ① `[reg+0xd10]` 로드 + 80B 창 안에 `[reg+0xd08]` 로드 + `sub r,[base+idx*8-8]` 조합
//      → 0.5.2 1건(=기지 정답 0x22d3fea) / 0.5.3 1건(0x12c5b44).
//   ② `cmp r,[reg+0xd10]` → 양 버전 .text 전체에 각 1건뿐(0x80ae73 / 0x95a359).
//   교차검증 = 둘 다 _MIGRATE_053.md의 컨테이너 후보 body 범위 내(0x12c56d0-0x12c5e69 / 0x952170-0x95b682).
// 0.5.4(2026-08-05 재핀): ~~0.5.3 0x12c5b44~~ → 0x14ece54(컨테이너 0x14ec9e0-0x14ed17a).
// 0.5.5(2026-08-12 재핀): ~~0.5.4 0x14ece54~~ → **0x14d819a**(컨테이너 0x14d7e10-0x14d855f, 함수내 +0x38a).
//   방법 ①을 0.5.4에 먼저 돌려 기지 정답 1건 재현 확인 후 0.5.5 적용 → 각 버전 유일 1건.
//   ★0.5.5 = 원본 바이트 변경: 48 8b 90(rax 베이스) → **49 8b 96(r14 베이스)** — mov rdx,[r14+0xd10].
//   가드검사(교체영역 site+1..+6 내부로 점프하는 분기) = 0건, 7B 치환 안전.
const RVA_LEN_LOAD: usize = 0x1090fba;   // 0.5.6(구0.5.5=0x14d819a). 컨테이너델타(레벨업 0x14d7e10→0x15037c0·BYTE=SAME·함수내 +0x38a)·orig 498b96100d0000 실측 일치(r14 베이스·0xd10 불변). // 레벨업 함수 0x14d7e10 내 (구 0.5.4 0x14ece54 / 0.5.3 0x12c5b44 / 0.5.2 0x22d3fea)
const ORIG_LEN_LOAD: [u8; 7] = [0x49, 0x8b, 0x96, 0x10, 0x0d, 0x00, 0x00]; // mov rdx,[r14+0xd10] (0.5.5: 베이스 rax→r14)

// UI 경험치 바 경로. 여기를 놓치면 레벨은 오르는데 경험치 막대가 깨진다(레벨 13+에서
//   원본 11칸 테이블을 보고 len 가드에 걸림). 뒤따르는 `mov rax,[rax+0xd08]`가 같은
//   인스턴스에서 ptr을 읽으므로, 앞선 이 지점에서 교체하면 인덱싱도 함께 따라온다.
// 0.5.3에서 **명령 바이트·주변 코드 모두 0.5.2와 동일**(rax=GameSetting, rcx=index 유지) = 스텁 무수정.
// 0.5.4(2026-08-05 재핀): ~~0.5.3 0x95a359~~ → 0xa99c29.
// 0.5.5(2026-08-12 재핀): ~~0.5.4 0xa99c29~~ → **0x95d8b9**(컨테이너 0x955680-0x95f05e, 함수내 +0x8239).
//   방법 ② 0.5.4 기지 정답 재현 후 적용 → 각 버전 유일 1건. 원본 7B·rax=GameSetting·rcx=index 전부
//   무변경, 뒤따르는 `mov rax,[rax+0xd08]` 까지 거리 0x912 도 0.5.4와 동일 = UI 스텁 무수정.
//   가드검사 0건, 7B 치환 안전.
const RVA_UI_CMP: usize = 0x9035e9;      // 0.5.6(구0.5.5=0x95d8b9). 컨테이너 0x955680→0xb43ed0(본문변경·BYTE=DIFF)이라 owner내 유일검색으로 확정·orig 483b88100d0000 실측 일치(rax=GameSetting·0xd10 불변). // UI 함수 0x955680 내 (구 0.5.4 0xa99c29 / 0.5.3 0x95a359 / 0.5.2 0x80ae73)
const ORIG_UI_CMP: [u8; 7] = [0x48, 0x3b, 0x88, 0x10, 0x0d, 0x00, 0x00];   // cmp rcx,[rax+0xd10]

// GameSetting 오프셋
const O_CAP: u32 = 0xd00;
const O_PTR: u32 = 0xd08;
const O_LEN: u32 = 0xd10;

/// cap은 항상 0으로 둔다. Rust `RawVec::drop`은 cap==0이면 dealloc하지 않으므로,
/// 우리 배열이 게임 얼로케이터로 free되는 사고를 원천 차단한다(원본 버퍼는 leak, 88B 수준).
/// UI가 cap을 읽는 코드는 없음이 확인됐다(계산 함수 0x803b30 전수 스캔 0건).
const CAP_ZERO: bool = true;

// ── 진단 카운터 (트램폴린이 직접 write) ──
static OBSERVED_LEN: AtomicU64 = AtomicU64::new(u64::MAX); // 게임이 실제로 들고 있던 need_exp.len
static OBSERVED_CAP: AtomicU64 = AtomicU64::new(u64::MAX); // 〃 cap (원본 cap 실측용)
static PATCH_COUNT: AtomicU64 = AtomicU64::new(0);         // 레벨업 경로 교체 횟수
static CALL_COUNT: AtomicU64 = AtomicU64::new(0);          // 레벨업 경로 진입 횟수
static UI_PATCH_COUNT: AtomicU64 = AtomicU64::new(0);      // UI 경로 교체 횟수
static UI_CALL_COUNT: AtomicU64 = AtomicU64::new(0);       // UI 경로 진입 횟수

type BOOL = i32; type DWORD = u32; type HMODULE = usize;
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleHandleExW(flags: u32, addr: *const u16, h: *mut HMODULE) -> BOOL;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, sz: DWORD) -> DWORD;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old: *mut u32) -> BOOL;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, sz: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
}
#[repr(C)] #[derive(Default)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32,
    region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32 }

#[inline] unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const RD: u32 = 0x02|0x04|0x20|0x40; const GUARD: u32 = 0x01|0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}

fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }
fn dir() -> Option<PathBuf> { unsafe {
    let mut h: HMODULE = 0;
    if GetModuleHandleExW(0x4|0x2, dir as *const () as *const u16, &mut h) == 0 || h == 0 { return None; }
    let mut b = [0u16; 4096];
    let n = GetModuleFileNameW(h, b.as_mut_ptr(), b.len() as DWORD);
    if n == 0 { return None; }
    let mut p = PathBuf::from(String::from_utf16_lossy(&b[..n as usize])); p.pop(); Some(p)
}}
fn log(s: &str) {
    if let Some(mut p) = dir() {
        p.push("tfm2_level_cap.txt");
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) {
            let _ = write!(f, "{}", s); let _ = f.flush();
        }
    }
}

/// 설정 파일을 읽어 need_exp 테이블을 만든다. 반환 = (테이블, 사람이 읽을 요약).
/// 파일이 없으면 기본값으로 생성한다. 형식 오류는 기본값으로 폴백하고 로그에 남긴다.
/// 확장 구간(ext)을 바닐라 뒤에 붙여 최종 테이블을 만든다.
/// max_level ≤ 12면 바닐라를 그만큼 잘라 쓰고 ext는 무시한다.
fn compose(max_level: u64, ext: &[u64]) -> Vec<u64> {
    let want_total = (max_level - 1) as usize;
    let take_vanilla = want_total.min(VANILLA.len());
    let mut t = VANILLA[..take_vanilla].to_vec();
    let want_ext = want_total - take_vanilla;
    if want_ext > 0 {
        let src: &[u64] = if ext.is_empty() { &DEF_EXT } else { ext };
        for i in 0..want_ext {
            // 모자라면 마지막 값으로 채운다
            t.push(src[i.min(src.len() - 1)]);
        }
    }
    t
}

fn load_config() -> (Vec<u64>, String) {
    let default = || compose(DEF_MAX_LEVEL, &DEF_EXT);
    let path = match dir() { Some(mut p) => { p.push(CFG_NAME); p }, None => {
        return (default(), "cfg 경로 확인 실패 → 기본값".into()); } };

    if !path.exists() {
        let body = format!("\
# {} 설정\n\
# 1줄: 최대 레벨 (2 ~ {})\n\
# 2줄: {}→{}부터 레벨업에 필요한 경험치를 ,로 구분해 나열\n\
#      (레벨 {}까지는 바닐라 값을 그대로 씁니다. 필요 개수 = 최대레벨 - {})\n\
#      더 많이 적으면 끊고, 모자라면 마지막 값으로 나머지를 채웁니다.\n\
{}\n{}\n",
            MOD_ID, MAX_LEVEL_LIMIT,
            VANILLA_MAX_LEVEL, VANILLA_MAX_LEVEL + 1, VANILLA_MAX_LEVEL, VANILLA_MAX_LEVEL,
            DEF_MAX_LEVEL,
            DEF_EXT.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","));
        let _ = fs::write(&path, body);
        return (default(), format!("{} 없음 → 기본값으로 생성", CFG_NAME));
    }

    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => return (default(), format!("cfg 읽기 실패({}) → 기본값", e)),
    };
    // BOM 제거 + 주석/빈 줄 걸러내기
    let lines: Vec<&str> = text.trim_start_matches('\u{feff}').lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#') && !l.starts_with("//"))
        .collect();
    if lines.is_empty() { return (default(), "cfg 내용 없음 → 기본값".into()); }

    let max_level: u64 = match lines[0].parse() {
        Ok(v) if (2..=MAX_LEVEL_LIMIT).contains(&v) => v,
        Ok(v) => return (default(),
                         format!("최대 레벨 {}이 허용 범위(2~{}) 밖 → 기본값", v, MAX_LEVEL_LIMIT)),
        Err(_) => return (default(), format!("최대 레벨 파싱 실패('{}') → 기본값", lines[0])),
    };
    // 설정에 적는 건 확장 구간뿐 = (12→13) … ((max-1)→max)
    let want_ext = (max_level.saturating_sub(VANILLA_MAX_LEVEL)) as usize;

    let raw: Vec<u64> = if lines.len() >= 2 {
        lines[1].split(',').map(|s| s.trim()).filter(|s| !s.is_empty())
                .filter_map(|s| s.parse::<u64>().ok()).collect()
    } else { Vec::new() };

    if want_ext == 0 {
        return (compose(max_level, &[]),
                format!("최대 레벨 {} (바닐라 상한 {} 이하 → 확장 구간 없음, 경험치 줄 무시)",
                        max_level, VANILLA_MAX_LEVEL));
    }
    if raw.is_empty() {
        return (compose(max_level, &DEF_EXT),
                format!("최대 레벨 {} / 경험치 줄 없음 → 확장 구간 기본값 사용", max_level));
    }

    let given = raw.len();
    let note = if given > want_ext {
        format!("최대 레벨 {} / 확장 경험치 {}개 중 {}개 사용(초과분 {}개 무시)",
                max_level, given, want_ext, given - want_ext)
    } else if given < want_ext {
        format!("최대 레벨 {} / 확장 경험치 {}개 → 마지막 값 {}로 {}개 채움",
                max_level, given, raw[given - 1], want_ext - given)
    } else {
        format!("최대 레벨 {} / 확장 경험치 {}개 (정확히 일치)", max_level, given)
    };
    (compose(max_level, &raw), note)   // compose가 끊기/채우기를 함께 처리
}

/// call rel32(±2GB) 사거리 안에 RWX 스텁을 잡는다. VirtualAlloc(0,..)은 임의 주소라
/// 2GB를 넘을 수 있으므로, 대상 주소 주변을 64KB 단위로 왕복 스캔한다.
unsafe fn alloc_near(target: usize) -> usize {
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let base = target & !0xFFFF;
    for step in 1..0x7F00usize {              // 최대 ~2GB
        for dir in [1i64, -1] {
            let addr = (base as i64 + dir * (step as i64) * 0x10000) as usize;
            if addr < 0x10000 { continue; }
            let p = VirtualAlloc(addr, 256, MEM_CR, RWX);
            if p != 0 {
                let d = (p as i64) - (target as i64);
                if d > i32::MIN as i64 && d < i32::MAX as i64 { return p; }
                // 사거리 밖이면 버리고 계속 (해제 생략 — 256B, 무시 가능)
            }
        }
    }
    0
}

/// `je near`(0F 84 rel32) 자리를 잡아두고, 나중에 목적지를 채운다.
/// ★rel8(`74 xx`)을 쓰면 교체 블록이 127B를 넘는 순간 조용히 어긋난다 — near 고정.
fn je_near_placeholder(s: &mut Vec<u8>) -> usize {
    let at = s.len();
    s.extend_from_slice(&[0x0f, 0x84, 0, 0, 0, 0]);
    at
}
fn patch_je_near(s: &mut Vec<u8>, at: usize) {
    let rel = (s.len() - (at + 6)) as u32;
    s[at + 2..at + 6].copy_from_slice(&rel.to_le_bytes());
}

/// 레벨업 경로 트램폴린. **진입 시 r14 = GameSetting**
///   (0.5.5 재핀 2026-08-12: ~~0.5.3·0.5.4 rax~~ → r14 회귀. 0.5.2도 r14였다).
/// ★판정 기준 = **ptr이 이미 내 테이블인가**(len 비교가 아니라). len으로 판정하면 merge된
///   인스턴스(len은 같지만 ptr은 게임 것)를 그냥 지나쳐, 시뮬과 UI가 서로 다른 테이블을
///   보게 된다 — 2026-07-23 경험치 바 폭주 사고의 구조적 원인.
/// ★r14는 GameSetting 베이스라 **파괴 금지**(rax·rcx는 사이트 이후 게임이 다시 로드하지만
///   ja 분기 경로에서의 라이브니스가 불확실하니 건드리지 않는다). 스크래치는
///   rdx(원본이 덮어쓰는 목적 레지스터)와 r11(push/pop 보존)뿐이다.
///   rdx는 마지막에 항상 우리 len으로 설정한다.
/// flags는 직후 원본 `cmp rdi,rdx`(0.5.5 — 구 `cmp r14,rdx`)가 새로 세팅하므로 무관.
unsafe fn build_stub(stub: usize) {
    let arr = TABLE_PTR;
    let len = TABLE_LEN;
    let obs = (&OBSERVED_LEN as *const AtomicU64) as u64;
    let ocap = (&OBSERVED_CAP as *const AtomicU64) as u64;
    let cnt = (&PATCH_COUNT as *const AtomicU64) as u64;
    let cal = (&CALL_COUNT as *const AtomicU64) as u64;

    // 0.5.5: GameSetting 베이스 = r14 (REX.B) — mod=10 rm=110(r14)
    //   구 0.5.3/0.5.4(rax 베이스)는 48 8b 90 / 48 89 90 / 48 c7 80 이었다.
    const LD_R14: [u8; 3] = [0x49, 0x8b, 0x96];   // mov rdx,[r14+disp32]
    const ST_R14: [u8; 3] = [0x49, 0x89, 0x96];   // mov [r14+disp32],rdx
    const MI_R14: [u8; 3] = [0x49, 0xc7, 0x86];   // mov qword [r14+disp32],imm32

    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x41, 0x53]);                             // push r11
    // CALL_COUNT += 1
    s.extend_from_slice(&[0x49, 0xbb]); s.extend_from_slice(&cal.to_le_bytes()); // mov r11, &CALL_COUNT
    s.extend_from_slice(&[0x49, 0xff, 0x03]);                       // inc qword [r11]
    // 이미 내 테이블을 가리키는가?
    s.extend_from_slice(&LD_R14); s.extend_from_slice(&O_PTR.to_le_bytes());     // mov rdx,[r14+0xd08]
    s.extend_from_slice(&[0x49, 0xbb]); s.extend_from_slice(&arr.to_le_bytes());         // mov r11, TABLE
    s.extend_from_slice(&[0x4c, 0x39, 0xda]);                       // cmp rdx, r11
    let je_at = je_near_placeholder(&mut s);                        // je skip
    // 진단: 교체 직전의 원본 len / cap 기록
    s.extend_from_slice(&LD_R14); s.extend_from_slice(&O_LEN.to_le_bytes());     // mov rdx,[r14+0xd10]
    s.extend_from_slice(&[0x49, 0xbb]); s.extend_from_slice(&obs.to_le_bytes());         // mov r11, &OBSERVED_LEN
    s.extend_from_slice(&[0x49, 0x89, 0x13]);                       // mov [r11], rdx
    s.extend_from_slice(&LD_R14); s.extend_from_slice(&O_CAP.to_le_bytes());     // mov rdx,[r14+0xd00]
    s.extend_from_slice(&[0x49, 0xbb]); s.extend_from_slice(&ocap.to_le_bytes());        // mov r11, &OBSERVED_CAP
    s.extend_from_slice(&[0x49, 0x89, 0x13]);                       // mov [r11], rdx
    // ptr / len / cap 교체
    s.extend_from_slice(&[0x48, 0xba]); s.extend_from_slice(&arr.to_le_bytes());         // mov rdx, TABLE
    s.extend_from_slice(&ST_R14); s.extend_from_slice(&O_PTR.to_le_bytes());     // mov [r14+0xd08], rdx
    s.extend_from_slice(&MI_R14); s.extend_from_slice(&O_LEN.to_le_bytes());
    s.extend_from_slice(&(len as u32).to_le_bytes());               // mov qword [r14+0xd10], len
    if CAP_ZERO {
        s.extend_from_slice(&MI_R14); s.extend_from_slice(&O_CAP.to_le_bytes());
        s.extend_from_slice(&0u32.to_le_bytes());                   // mov qword [r14+0xd00], 0
    }
    s.extend_from_slice(&[0x49, 0xbb]); s.extend_from_slice(&cnt.to_le_bytes());         // mov r11, &PATCH_COUNT
    s.extend_from_slice(&[0x49, 0xff, 0x03]);                       // inc qword [r11]
    // skip:
    patch_je_near(&mut s, je_at);
    s.extend_from_slice(&[0x41, 0x5b]);                             // pop r11
    s.extend_from_slice(&[0x48, 0xc7, 0xc2]); s.extend_from_slice(&(len as u32).to_le_bytes()); // mov rdx, len
    s.push(0xc3);                                                   // ret

    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());
}

/// UI 경험치 바 경로용 트램폴린. 진입 시 rax = GameSetting, rcx = 인덱스(=level).
/// ★원본 `cmp rcx,[rax+0xd10]`이 세우는 flags를 바로 뒤 `ja`가 쓰므로, 교체로 오염된 flags를
///   **마지막에 원본 cmp를 다시 수행**해 정확히 복원한다(pop·mov는 flags 불변).
/// 판정 기준은 레벨업 스텁과 동일하게 ptr — 이래야 UI와 시뮬이 항상 같은 테이블을 본다.
unsafe fn build_stub_ui(stub: usize) {
    let arr = TABLE_PTR;
    let len = TABLE_LEN;
    let cnt = (&UI_PATCH_COUNT as *const AtomicU64) as u64;
    let cal = (&UI_CALL_COUNT as *const AtomicU64) as u64;

    let mut s: Vec<u8> = Vec::new();
    s.push(0x52);                                                   // push rdx
    // UI_CALL_COUNT += 1
    s.extend_from_slice(&[0x48, 0xba]); s.extend_from_slice(&cal.to_le_bytes()); // mov rdx, &UI_CALL_COUNT
    s.extend_from_slice(&[0x48, 0xff, 0x02]);                       // inc qword [rdx]
    // 이미 내 테이블인가?
    s.extend_from_slice(&[0x41, 0x53]);                             // push r11
    s.extend_from_slice(&[0x48, 0x8b, 0x90]); s.extend_from_slice(&O_PTR.to_le_bytes()); // mov rdx,[rax+0xd08]
    s.extend_from_slice(&[0x49, 0xbb]); s.extend_from_slice(&arr.to_le_bytes());         // mov r11, TABLE
    s.extend_from_slice(&[0x4c, 0x39, 0xda]);                       // cmp rdx, r11
    s.extend_from_slice(&[0x41, 0x5b]);                             // pop r11
    let je_at = je_near_placeholder(&mut s);                        // je skip
    s.extend_from_slice(&[0x48, 0xba]); s.extend_from_slice(&arr.to_le_bytes());         // mov rdx, TABLE
    s.extend_from_slice(&[0x48, 0x89, 0x90]); s.extend_from_slice(&O_PTR.to_le_bytes()); // mov [rax+0xd08], rdx
    s.extend_from_slice(&[0x48, 0xc7, 0x80]); s.extend_from_slice(&O_LEN.to_le_bytes());
    s.extend_from_slice(&(len as u32).to_le_bytes());               // mov qword [rax+0xd10], len
    if CAP_ZERO {
        s.extend_from_slice(&[0x48, 0xc7, 0x80]); s.extend_from_slice(&O_CAP.to_le_bytes());
        s.extend_from_slice(&0u32.to_le_bytes());                   // mov qword [rax+0xd00], 0
    }
    s.extend_from_slice(&[0x48, 0xba]); s.extend_from_slice(&cnt.to_le_bytes()); // mov rdx, &UI_PATCH_COUNT
    s.extend_from_slice(&[0x48, 0xff, 0x02]);                       // inc qword [rdx]
    // skip:
    patch_je_near(&mut s, je_at);
    s.push(0x5a);                                                   // pop rdx
    s.extend_from_slice(&ORIG_UI_CMP);                              // cmp rcx,[rax+0xd10]  ← flags 복원
    s.push(0xc3);                                                   // ret

    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());
}

/// 7바이트 사이트를 `call rel32 + nop nop`으로 치환한다. builder가 스텁 본문을 조립한다.
unsafe fn install_site(rva: usize, orig: &[u8; 7], builder: unsafe fn(usize)) -> Result<String, String> {
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { return Err("module 0".into()); }
    let site = base.wrapping_add(rva);
    if !readable(site, 7) { return Err(format!("site unreadable @abs=0x{:x}", site)); }

    let mut cur = [0u8; 7];
    core::ptr::copy_nonoverlapping(site as *const u8, cur.as_mut_ptr(), 7);
    if cur[0] == 0xe8 { return Ok(format!("already hooked @abs=0x{:x}", site)); }  // 멱등
    if cur != *orig {
        return Err(format!("byte mismatch @abs=0x{:x} cur={:02x?} want={:02x?} (RVA stale?)",
                           site, cur, orig));
    }

    let stub = alloc_near(site);
    if stub == 0 { return Err("alloc_near 실패 (call rel32 사거리 내 여유 없음)".into()); }
    builder(stub);

    let rel = (stub as i64) - (site as i64 + 5);
    if rel < i32::MIN as i64 || rel > i32::MAX as i64 { return Err("rel32 범위 초과".into()); }
    let mut patch = [0x90u8; 7];
    patch[0] = 0xe8;
    patch[1..5].copy_from_slice(&(rel as i32).to_le_bytes());       // call rel32 + nop nop

    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(site, 7, RWX, &mut old) == 0 { return Err("VirtualProtect".into()); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), site as *mut u8, 7);
    VirtualProtect(site, 7, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), site, 7);

    let mut back = [0u8; 7];
    core::ptr::copy_nonoverlapping(site as *const u8, back.as_mut_ptr(), 7);
    if back != patch { return Err(format!("write 미반영 landed={:02x?}", back)); }
    Ok(format!("hooked @abs=0x{:x} stub=0x{:x} rel={}", site, stub, rel))
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    let (table, note) = load_config();
    let max_level = table.len() as u64 + 1;
    // ★Box::leak — 트램폴린이 이 주소를 imm64로 박으므로 절대 이동/해제되면 안 된다.
    let leaked: &'static [u64] = Box::leak(table.into_boxed_slice());
    unsafe {
        TABLE_PTR = leaked.as_ptr() as u64;
        TABLE_LEN = leaked.len() as u64;
    }

    log(&format!("\n[{}ms] === {} INIT (0.5.5) ===\n", now_ms(), MOD_ID));
    log(&format!("[cfg] {}\n", note));
    log(&format!("[cfg] 최대 레벨 {} / need_exp({}개, 앞 {}개는 바닐라) = {:?}\n",
                 max_level, leaked.len(), VANILLA.len().min(leaked.len()), leaked));

    unsafe {
        // 레벨업 경로와 UI 경험치 바 경로를 독립 설치 — 한쪽이 실패해도 다른 쪽은 살린다.
        match install_site(RVA_LEN_LOAD, &ORIG_LEN_LOAD, build_stub) {
            Ok(st) => log(&format!("[hook:levelup] {}\n", st)),
            Err(e) => log(&format!("[hook:levelup] 실패: {}\n", e)),
        }
        match install_site(RVA_UI_CMP, &ORIG_UI_CMP, build_stub_ui) {
            Ok(st) => log(&format!("[hook:expbar] {}\n", st)),
            Err(e) => log(&format!("[hook:expbar] 실패: {}\n", e)),
        }
    }
    // 진단 스레드. orig(len/cap) = 교체 직전 게임이 들고 있던 값.
    //   expbar patched가 0이 아니면 UI도 우리 테이블로 강제됐다는 뜻(= 분모·분자 스케일 일치).
    std::thread::spawn(|| {
        let mut last = (u64::MAX, u64::MAX);
        loop {
            std::thread::sleep(std::time::Duration::from_secs(5));
            let obs = OBSERVED_LEN.load(Ordering::Relaxed);
            let calls = CALL_COUNT.load(Ordering::Relaxed);
            let ui_calls = UI_CALL_COUNT.load(Ordering::Relaxed);
            if obs == u64::MAX && calls == 0 && ui_calls == 0 { continue; } // 아직 미진입
            let cur = (calls, ui_calls);
            if cur == last { continue; }
            last = cur;
            let ocap = OBSERVED_CAP.load(Ordering::Relaxed);
            log(&format!("[{}ms] orig(len={} cap={}) levelup(calls={} patched={}) expbar(calls={} patched={})\n",
                         now_ms(), obs, ocap as i64, calls, PATCH_COUNT.load(Ordering::Relaxed),
                         ui_calls, UI_PATCH_COUNT.load(Ordering::Relaxed)));
        }
    });
    ModRegistration::new(MOD_ID)
}
declare_mod!(init);
