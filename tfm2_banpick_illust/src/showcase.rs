//! 쇼케이스 카드(밴 취소선/2분할 카드 + 픽 중앙 비행 카드) 일러스트 — 구 tfm2_banpick_showcase 통합(2026-07-25).
//! ===========================================================================
//! RE 근거·FFI 계약 정본 = `C:\tfm2mods\tfm2_banpick_showcase\FFI_CONTRACT.md` (0.5.2).
//! 구조:
//!   훅 A RVA_FX_SET(연출 상태 세팅) = 진영/모드/챔프 스태시 → 원본 (관찰만)
//!   훅 B RVA_CARD_DRAW(카드 드로우 헬퍼) = 가로형 커스텀 레이아웃 전체 대체 (아트/idle 폴백),
//!        미정(스태시 없음)·설치 실패 시 원본 트램폴린 폴백
//!   훅 C RVA_ILLUST_GET(밴픽 일러 에셋 조회) = 카드 크기 게이트 하 리다이렉트 (훅 B 실패 시 안전망)
//!   ※구 0.5.2 주소 표기(0x11e2370/0x11f9030/0xfdabe0)는 폐기 — 실제 값은 아래 const 참조.
//!   기하패치 12사이트 = 밴 분할 연출(스냅샷 타깃·컷·취소선)을 360x480→520x408 확대
//! 아트 해석 = crate::keys::illust_key 재사용(진영 폴백·타모드 아트·flip 판정 포함).
//! 안전수칙: detour 본문 catch_unwind, 포인터 range 가드, 게임이 free하는 String은
//!   게임 alloc(RVA_GAME_ALLOC) 사용, 훅·패치 1회 설치(재설치 금지), 사전검증 실패 시 전체 스킵.
//! ===========================================================================
#![allow(dead_code)]
use crate::{config, keys};
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU32, AtomicUsize, Ordering};

// ── 게임 함수 RVA (패치 시 재핀 대상 — MIGRATION §7.2-B) ────────────────────
// ★0.5.4 재핀(2026-08-05) — 방법 2종 교차검증:
//   ①CARD_DRAW 호출시퀀스 정렬(27/27 콜이 **동일 상대오프셋**)로 12함수 일괄 도출
//   ②전함수 구조지문(skel = imm/disp 정규화 명령열 해시) 완전일치 + **NEW 전역 유일**
//   ILLUST_GET 은 추가로 문자열 xref("asset/base/ui/banpick/illustrations") 로 독립 확인.
//   0.5.4 값 → 0.5.5 값을 각 줄 끝 주석에 남긴다(정정형). ★0.5.5 함수 16건 = version-migrator §7.5 확정 시드.
const RVA_FX_SET: usize = 0x1e598b0; // 0.5.6(구값 갱신) // 훅 A: 연출 상태 세팅 (진영 스태시) — 0.5.5 프롤로그 20B 완전일치  ⟵0.5.4 0x1d92980
const RVA_CARD_DRAW: usize = 0x1e6f350; // 0.5.6(구값 갱신) // 훅 B: 카드 드로우 헬퍼 — 0.5.5 프롤로그 일치 + ILLUST_GET 콜러 size2979 대응  ⟵0.5.4 0x1da8410
const RVA_ILLUST_GET: usize = 0x226f290; // 0.5.6(구값 갱신) // 훅 C: 밴픽 일러 에셋 조회 — 0.5.5 프롤로그 일치 + 콜러 3/3 프로필 완전대응  ⟵0.5.4 0x1ffd970
const RVA_SUBMIT: usize = 0x182430; // 0.5.6(구값 갱신) // (list, &cmd) 일반 제출 — §7.5 skel유일  ⟵0.5.4 0x181400
const RVA_SUBMIT_TEXT: usize = 0x1826b0; // 0.5.6(구값 갱신) // (list, &cmd) 텍스트 전용 — §7.5 skel유일  ⟵0.5.4 0x181680
const RVA_IMG_BUILD: usize = 0x183b50; // 0.5.6(구값 갱신) // (&cmd, key, len, x, y, layer, w, h, 0,0,0,0) — §7.5 skel유일  ⟵0.5.4 0x182b20
const RVA_IMG_UV: usize = 0x1839b0; // 0.5.6(구값 갱신) // (&out, &in, &uv) — §7.5 skel유일  ⟵0.5.4 0x182980
const RVA_IMG_FLAG: usize = 0x183e90; // 0.5.6(구값 갱신) // (&out, &in, 샘플링: 1=nearest) — §7.5 skel유일  ⟵0.5.4 0x182e30
const RVA_IMG_COLOR: usize = 0x1eeee0; // 0.5.6(구값 갱신) // (&out, &in, "color", 5, &rgba) — §7.5 콜슬롯8/19 정렬  ⟵0.5.4 0x1c2f8d0
const RVA_IMG_SHADER: usize = 0x185490; // 0.5.6(구값 갱신) // (&out, &in, shader_key, len) — §7.5 skel유일  ⟵0.5.4 0x184430
const RVA_TEXT_BUILD: usize = 0x183040; // 0.5.6(구값 갱신) // (...) 텍스트 cmd — §7.5 skel유일  ⟵0.5.4 0x182010
const RVA_NAME_GET: usize = 0x1e8d1e0; // 0.5.6(구값 갱신) // 챔프 표시명 String — ABI(sret GStr,ctx,key,len) 동일  ⟵0.5.4 0x1dd4240
// ★asset-get clone family(구조지문이 33개 함수와 동일 = skel 로 형제 구별 불가)는
//   0.5.4 에서 **콜슬롯 정렬**로 확정했다(투표 불요):
//     ASSET_GET  ILLUST_GET(0x1ffd970) 콜슬롯 1@+0x13 · SPRITE_CALC(0x1dd9170) 콜슬롯 5@+0x1d1
//                → 둘 다 0x143d50 = **0.5.3 과 동일 주소**(저번지 .text 구간 미이동)
//     ANIM_GET   CARD_DRAW 콜슬롯 14@+0x472 · SPRITE_CALC 콜슬롯 3@+0x74 → 0x74c010 (콜러 60→60)
const RVA_ASSET_GET: usize = 0x143de0; // 0.5.6(구값 갱신) // 키→텍스처 에셋 (obj,vtbl) — §7.5 ILLUST_GET 콜슬롯1 정렬  ⟵0.5.4 0x143d50
const RVA_ANIM_GET: usize = 0x74bf10; // 0.5.6(구값 갱신) // 키→애님 리소스 — §7.5 lea→call 1표 + CARD_DRAW 콜슬롯13  ⟵0.5.4 0x74c010
const RVA_SPRITE_CALC: usize = 0x1e92110; // 0.5.6(구값 갱신) // idle 시트키+UV+크기 계산기(무부작용) — §7.5 skel유일  ⟵0.5.4 0x1dd9170
// 0.5.3~: 범용 __rust_alloc 이 align별 심 + impl 로 분해 ⟹ impl 직접호출 **3인자**가 전 모드 정본.
//   0.5.4 실측 = 0x29bb920 은 `GetProcessHeap()` → `HeapAlloc(heap, flags, size)` tail-jmp 래퍼이고
//   0.5.3 0x28f7df0 과 **IAT disp 2개만 다른 바이트 동일 본문**(60B) + 콜러 33,708 = 신원 확정.
const RVA_GAME_ALLOC: usize = 0x2ab4010; // 0.5.6(구값 갱신) // (무시, flags, size) → ptr / GetProcessHeap→HeapAlloc tail-jmp 래퍼 — §7.5 skel/마스크 유일  ⟵0.5.4 0x29bb920
// ⛔RVA_GAME_FREE 폐지 — 0.5.3 에서 `__rust_dealloc` 범용 함수는 **인라인화로 소멸**했다
//   (HeapFree 참조 함수가 전부 Box/Vec drop 인라인이고 0.5.2 dealloc(0x25c4d90) 형태는 부재).
//   0.5.2 dealloc 본문 = `HeapFree(GetProcessHeap(), 0, align>16 ? ptr-8 : ptr)` 였고 우리는 항상
//   align=1 로 할당하므로 보정 분기를 타지 않는다 ⟹ **HeapFree 직접 호출이 비트동일한 해제**.


/// 쇼케이스 이름 텍스트 z층 — patchviz 이름 색칠 제외 기준(카드 이름은 기본색 유지).
pub(crate) const Z_NAME_TEXT: u32 = 0x4be;

// ── 진입부 검증/재배치 상수 (★0.5.4 전건 재실측 2026-08-05) — 전부 rip-rel·chkstk 없음 ─
const FX_SET_ORIG_LEN: usize = 12;
const FX_SET_PROLOGUE: &[u8] =
    &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
const CARD_DRAW_ORIG_LEN: usize = 12;
const CARD_DRAW_PROLOGUE: &[u8] =
    &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
// ★0.5.3 변경(2026-07-29 실측): 프롤로그가 push4 → **push6** 로 바뀌며 명령경계가 이동했었다
//   (0.5.2 = 19 → 0.5.3 = 13). ⚠구 19 를 그대로 두면 명령 한복판을 절단해 **즉사**.
// ★0.5.4(2026-08-05 재실측): 훅 3종 모두 진입 20B·명령경계 **완전 동일** ⟹ 12/12/13 유지.
// ★0.5.5(2026-08-12 재실측): 0.5.5 시드 주소(FX 0x1951330·CARD 0x1966dd0·ILLUST 0x1edf1f0)에서도
//   진입 20B·명령경계·rip-rel 없음 **완전 동일** ⟹ 12/12/13 유지.
//   0.5.5 ILLUST_GET 진입 = 55 41 56 56 57 53 48 81 EC 80 00 00 00 (경계 1,3,4,5,6,13,21,26).
const ILLUST_GET_ORIG_LEN: usize = 13;
const ILLUST_GET_PROLOGUE: &[u8] = &[
    0x55, 0x41, 0x56, 0x56, 0x57, 0x53, 0x48, 0x81, 0xEC, 0x80, 0x00, 0x00, 0x00,
];

// ── 밴 분할 연출 기하 확대 패치 (360×480 → 520×408) — FFI_CONTRACT §기하패치 ──
const GEOM_W: f32 = 520.0;
const GEOM_H: f32 = 408.0;
const GEOM_CUT: f32 = 60.0;
// ★0.5.5 재핀(2026-08-12): ⚠0.5.4 와 달리 **블록 통째 이동이 아니다**. 델타 불균일
//   (CARD +0xa62e0 / SNAP +0xa62f0 / LINE_DIR·START +0xa6300) = **블록 내부 배치가 벌어졌다**
//   (0.5.4 SNAP=CARD+0x30 → 0.5.5 SNAP=CARD+0x40). 도출 = 0.5.4 원본 float 유일검색(4/6) +
//   **블록 내부 순서 재구성**(0.5.5 배치: CARD+0/SNAP+0x40/NORMAL+0x60/LINE_DIR+0x80/
//   LINE_START+0x90/LINE_ANCHOR+0xa0/ZIG+0xb0)으로 중복값 2건(NORMAL·LINE_ANCHOR)까지 유일화 + 값 실측.
//   교차검증 = ZIG 명령 rip 타겟 0x338b510 → 0x3431810 = **+0xa6300** = LINE_DIR/START 블록 델타와 일치.
const RVA_C_CARD_RECT: usize = 0x348a740; // 0.5.6(구값 갱신) // {-180,-240,360,480} 카드 로컬 rect(밴·픽 공용)  ⟵0.5.4 0x338b480
const RVA_C_SNAP_RECT: usize = 0x348a770; // 0.5.6(구값 갱신) // {0,0,360,480} 스냅샷 내부 rect(좌상단 원점)  ⟵0.5.4 0x338b4b0
const RVA_C_LINE_DIR: usize = 0x348a7b0; // 0.5.6(구값 갱신) // {360,340} 취소선 방향  ⟵0.5.4 0x338b4e0
const RVA_C_LINE_START: usize = 0x348a7c0; // 0.5.6(구값 갱신) // {-180,170} 취소선 시작  ⟵0.5.4 0x338b4f0
const RVA_C_LINE_ANCHOR: usize = 0x348a7d0; // 0.5.6(구값 갱신) // {0,170} 앵커(블록 내부 LINE_START<x<ZIG 유일)  ⟵0.5.4 0x338b500
const RVA_C_NORMAL: usize = 0x348a790; // 0.5.6(구값 갱신) // {0.6866,0.727} 분리 법선(블록 내부 SNAP<x<LINE_DIR 유일)  ⟵0.5.4 0x338b4c0
// ★mid-func = 명령 시작이 아니라 **필드(imm4/disp4) 위치**다. 오프셋 이식 금지 —
//   컨테이너(A 0x1e16c90→**0x19e5f20**, 8149B / B 0x1db2370→**0x1970f40**, 1062B) 안에서
//   "같은 니모닉 + 같은 타겟 float" 명령을 찾아 필드 위치를 재산출.
//   ★0.5.5(2026-08-12) 는 6건 전부 **컨테이너 내 상대오프셋 Δrel=0**(+0x87c/+0x892/+0x83/+0x91/+0x16de/+0x1dbc).
//   컨테이너 매핑 근거: 0x19e5f20 = ILLUST_GET 콜러 프로필 3/3 완전대응(size 8149·480imm 보유),
//                       0x1970f40 = A 의 콜리(size 1062·A 가 유일 콜러·±70 movss 보유).
const RVA_I_SNAP_H: usize = 0x1ed5830; // 0.5.6(구값 갱신) // mov dword [rsp+0x20], 480.0 의 imm4 (스냅샷 타깃 높이)  ⟵0.5.4 0x1e17510
const RVA_D_SNAP_W: usize = 0x1ed5846; // 0.5.6(구값 갱신) // disp4 → 360.0 (스냅샷 폭, 광공유)  ⟵0.5.4 0x1e17526
const RVA_D_CUT_LO: usize = 0x1e794b8; // 0.5.6(구값 갱신) // disp4 → -70.0 (컨테이너 0x1970f40 하단 컷)  ⟵0.5.4 0x1db23f8
const RVA_D_CUT_HI: usize = 0x1e794c6; // 0.5.6(구값 갱신) // disp4 → +70.0 (〃 상단 컷)  ⟵0.5.4 0x1db2406
// ZIG 2건은 컨테이너 안에 -180.0 참조가 정확히 2곳(0.5.2 와 동수)이고 **둘 다 slots+12 로 리타겟**
//   하므로 X1/X2 배정 순서가 결과에 영향을 주지 않는다(주소 순으로 배정).
const RVA_D_ZIG_X1: usize = 0x1ed6692; // 0.5.6(구값 갱신) // disp4 → -180.0 (지그재그 x)  ⟵0.5.4 0x1e18372
const RVA_D_ZIG_X2: usize = 0x1ed6d70; // 0.5.6(구값 갱신) // disp4 → -180.0 (〃 두 번째 블록)  ⟵0.5.4 0x1e18a50
// 0.5.5 .rdata 최장 0런 = **0x40c41c4..0x40db530(95,084B)** 내부의 4K 정렬 지점(±64B 전부 0 실측).
//   precheck 가 "슬롯 4개 전부 0.0" 을 요구하므로 오염 시 자동 SKIP(안전).
const RVA_SLOTS: usize = 0x40e6000; // 0.5.6(구값 갱신) // .rdata 패딩 슬롯 [w, cut_lo, cut_hi, zig_x]  ⟵0.5.4 0x3fe2000
static GEOM_PATCHED: AtomicBool = AtomicBool::new(false);

// ── 상태 ──
static BASE: AtomicUsize = AtomicUsize::new(0);
static INSTALLED: AtomicBool = AtomicBool::new(false);
static TRAMP_FX_SET: AtomicUsize = AtomicUsize::new(0);
static TRAMP_CARD: AtomicUsize = AtomicUsize::new(0);
static TRAMP_ILLUST: AtomicUsize = AtomicUsize::new(0);
/// 마지막 셀렉트 확정: 0=밴 1=픽blue 2=픽red, 0xff=미정 (훅 A가 기록)
static LAST_MODE: AtomicU8 = AtomicU8::new(0xff);
/// 마지막 셀렉트 확정 진영: 1=blue 0=red (밴 포함)
static LAST_IS_BLUE: AtomicU8 = AtomicU8::new(1);
static DBG_SEQ: AtomicU32 = AtomicU32::new(0);

const MEM_CR: u32 = 0x1000 | 0x2000;
const RWX: u32 = 0x40;
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(n: *const u16) -> usize;
    fn VirtualAlloc(addr: usize, size: usize, ty: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, prot: u32, old: *mut u32) -> u32;
    fn FlushInstructionCache(h: isize, addr: usize, size: usize) -> u32;
    fn GetCurrentProcess() -> isize;
    fn GetProcessHeap() -> usize;
    fn HeapFree(heap: usize, flags: u32, mem: usize) -> i32;
}

fn slog(msg: &str) {
    if !config::get().debug {
        return;
    }
    if let Some(d) = crate::mod_dir() {
        use std::io::Write;
        let _ = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("{d}\\showcase_log.txt"))
            .and_then(|mut f| writeln!(f, "[{}] {}", DBG_SEQ.fetch_add(1, Ordering::Relaxed), msg));
    }
}

// ── 게임 함수 타입 ──────────────────────────────────────────────────────────
type FxSetFn = unsafe extern "win64" fn(usize, usize, usize, u64, usize, usize, u8);
type CardDrawFn = unsafe extern "win64" fn(
    usize, usize, *const u8, usize, *const [f32; 4], *const [f32; 4], u8, f32,
);
type IllustGetFn = unsafe extern "win64" fn(*mut IllustOut, usize, *const u8, usize, f32, f32);
type Submit2Fn = unsafe extern "win64" fn(usize, *const Cmd);
type ImgBuildFn = unsafe extern "win64" fn(
    *mut Cmd, *const u8, usize, f32, f32, u32, f32, f32, f32, f32, f32, f32,
);
type ImgUvFn = unsafe extern "win64" fn(*mut Cmd, *const Cmd, *const [f32; 4]);
type ImgFlagFn = unsafe extern "win64" fn(*mut Cmd, *const Cmd, u8);
type ImgColorFn = unsafe extern "win64" fn(*mut Cmd, *const Cmd, *const u8, usize, *const [f32; 4]);
type ImgShaderFn = unsafe extern "win64" fn(*mut Cmd, *const Cmd, *const u8, usize);
type TextBuildFn = unsafe extern "win64" fn(
    *mut Cmd, *const u8, usize, *const u8, usize, *const [f32; 4], *const [f32; 4], u32, f32, u8,
    u8, *const Outline, f32,
);
type NameGetFn = unsafe extern "win64" fn(*mut GStr, usize, *const u8, usize);
type AssetGetFn = unsafe extern "win64" fn(usize, *const u8, usize) -> usize;
type AnimGetFn = unsafe extern "win64" fn(usize, *const u8, usize) -> usize;
type SpriteCalcFn = unsafe extern "win64" fn(*mut SpriteOut, usize, *const u8, usize, f32, f32, f32);
type AllocFn = unsafe extern "win64" fn(usize, usize, usize) -> usize; // 0.5.3: (무시, flags, size)

/// 게임 힙 해제. 0.5.3 에서 `__rust_dealloc` 이 인라인 소멸했으므로 그 본문과 동일한 일을 직접 한다:
/// `HeapFree(GetProcessHeap(), 0, ptr)` — align=1 할당분이라 0.5.2 의 `ptr-8` 보정 분기는 해당 없음.
#[inline]
unsafe fn game_free(ptr: usize) {
    if ptr > 0x10000 {
        let h = GetProcessHeap();
        if h != 0 {
            HeapFree(h, 0, ptr);
        }
    }
}
type TexDimFn = unsafe extern "win64" fn(usize) -> f32;

#[inline]
fn gfn(rva: usize) -> usize {
    BASE.load(Ordering::Relaxed) + rva
}

/// draw cmd 버퍼 0xd0B — memset(0) 후 필요 필드만 (FFI_CONTRACT §3)
#[repr(C, align(16))]
#[derive(Clone, Copy)]
struct Cmd {
    b: [u8; 0xd0],
}
impl Cmd {
    fn zero() -> Self {
        Cmd { b: [0u8; 0xd0] }
    }
    unsafe fn wr<T: Copy>(&mut self, off: usize, v: T) {
        core::ptr::write_unaligned(self.b.as_mut_ptr().add(off) as *mut T, v);
    }
}
#[repr(C)]
struct Outline {
    a: u64,
    b: u32,
    c: f32,
}
#[repr(C)]
struct GStr {
    cap: usize,
    ptr: usize,
    len: usize,
}
/// RVA_ILLUST_GET out (0x28B): 키 String + cover UV. cap=-1 = 없음 센티널.
#[repr(C)]
struct IllustOut {
    cap: usize,
    ptr: usize,
    len: usize,
    u0: f32,
    v0: f32,
    uw: f32,
    vh: f32,
}
/// RVA_SPRITE_CALC out (0x30B): 시트 키 String + UV + 그릴 크기. cap=-1 = 실패.
#[repr(C)]
struct SpriteOut {
    cap: usize,
    ptr: usize,
    len: usize,
    u0: f32,
    v0: f32,
    uw: f32,
    vh: f32,
    w: f32,
    h: f32,
}

#[inline]
fn ptr_ok(p: usize) -> bool {
    (0x10000..1usize << 48).contains(&p)
}
#[inline]
fn c01(v: f32) -> f32 {
    v.clamp(0.0, 1.0)
}

// ── 아트 해석 (keys.rs 재사용) + 에셋 조회 ──────────────────────────────────
/// 키→텍스처 (w, h). 부재/비텍스처 = None.
unsafe fn tex_dims(store: usize, key: &[u8]) -> Option<(f32, f32)> {
    let get: AssetGetFn = core::mem::transmute(gfn(RVA_ASSET_GET));
    let entry = get(store, key.as_ptr(), key.len());
    if !ptr_ok(entry) {
        return None;
    }
    let obj = *(entry as *const usize);
    let vtbl = *((entry + 8) as *const usize);
    if !ptr_ok(obj) || !ptr_ok(vtbl) {
        return None;
    }
    let wf: TexDimFn = core::mem::transmute(*((vtbl + 0x28) as *const usize));
    let hf: TexDimFn = core::mem::transmute(*((vtbl + 0x30) as *const usize));
    let (w, h) = (wf(obj), hf(obj));
    if !(w.is_finite() && h.is_finite() && w > 1.0 && h > 1.0) {
        return None;
    }
    Some((w, h))
}

/// keys::illust_key 로 아트 해석(진영 폴백·타모드 아트·flip 포함) 후 텍스처 크기 확인.
/// 반환 = (에셋 키, flip, tex_w, tex_h).
unsafe fn resolve_art(store: usize, champ: &str, is_blue: bool) -> Option<(&'static str, bool, f32, f32)> {
    let cfg = config::get();
    let side = if is_blue {
        keys::BLUE
    } else if cfg.red_flip {
        keys::RED
    } else {
        keys::RED_NOFLIP
    };
    // 선호 zoom 없으면 반대 변형까지 시도 — 챔프당 1장만 있어도 동작.
    let (key, flip) = keys::illust_key(champ, side, cfg.zoom)
        .or_else(|| keys::illust_key(champ, side, !cfg.zoom))?;
    let (w, h) = tex_dims(store, key.as_bytes())?;
    Some((key, flip, w, h))
}

/// cover-크롭 UV: 대상 (tw,th)에 텍스처 (w,h)를 꽉 채우고 긴 축 중앙 크롭
fn cover_uv(w: f32, h: f32, tw: f32, th: f32) -> [f32; 4] {
    let tex_a = w / h;
    let tgt_a = tw / th;
    if tex_a > tgt_a {
        let uw = tgt_a / tex_a;
        [(1.0 - uw) * 0.5, 0.0, uw, 1.0]
    } else {
        let vh = tex_a / tgt_a;
        [0.0, (1.0 - vh) * 0.5, 1.0, vh]
    }
}

// ── idle 스프라이트 폴백 (아트 없는 챔프 — 레드팀 좌우반전) ─────────────────
/// "idle" 태그 대표 프레임(duration>0 첫, 없으면 마지막)의 (fw, fh).
/// anim obj = hashbrown SwissTable: ctrl/mask, 엔트리 stride 0x30 역방향.
unsafe fn idle_frame_size(anim: usize) -> Option<(f32, f32)> {
    if !ptr_ok(anim) || *((anim + 0x18) as *const u64) == 0 {
        return None;
    }
    let ctrl = *(anim as *const usize);
    let mask = *((anim + 0x08) as *const usize);
    if !ptr_ok(ctrl) || mask > 0x1000 {
        return None;
    }
    for i in 0..=mask {
        if *((ctrl + i) as *const u8) & 0x80 != 0 {
            continue;
        }
        let e = ctrl - (i + 1) * 0x30;
        let kptr = *((e + 0x08) as *const usize);
        let klen = *((e + 0x10) as *const usize);
        if klen != 4 || !ptr_ok(kptr) || *(kptr as *const u32) != 0x656c6469 {
            continue; // != "idle"
        }
        let fptr = *((e + 0x20) as *const usize);
        let flen = *((e + 0x28) as *const usize);
        if !ptr_ok(fptr) || flen == 0 || flen > 4096 {
            return None;
        }
        let mut pick = fptr + (flen - 1) * 0x14;
        for f in 0..flen {
            let fr = fptr + f * 0x14;
            if *((fr + 0x10) as *const f32) > 0.0 {
                pick = fr;
                break;
            }
        }
        let (fw, fh) = (*((pick + 0x08) as *const f32), *((pick + 0x0c) as *const f32));
        if fw > 0.0 && fh > 0.0 {
            return Some((fw, fh));
        }
        return None;
    }
    None
}

/// idle 스프라이트를 일러 영역 중앙에 드로우. flip = 좌우반전(+0xad, x=오른쪽 끝).
unsafe fn draw_idle_sprite(
    store: usize,
    list: usize,
    champ: &[u8],
    illust: &[f32; 4],
    greyscale: u8,
    t: f32,
    flip: bool,
) -> bool {
    let mut anim_key = Vec::with_capacity(64 + champ.len());
    anim_key.extend_from_slice(b"asset/base/aseprite_resources/champions/");
    anim_key.extend_from_slice(champ);
    anim_key.extend_from_slice(b"#anim");
    let anim_get: AnimGetFn = core::mem::transmute(gfn(RVA_ANIM_GET));
    let anim = anim_get(store, anim_key.as_ptr(), anim_key.len());
    let Some((fw, fh)) = idle_frame_size(anim) else {
        return false;
    };
    let (tw, th) = (illust[2] * 0.82, illust[3] * 0.82);
    let mut scale = (tw / fw).min(th / fh);
    if !scale.is_finite() {
        scale = tw / fw;
    }
    scale = scale.clamp(2.0, 6.4);
    let calc: SpriteCalcFn = core::mem::transmute(gfn(RVA_SPRITE_CALC));
    let mut out = SpriteOut {
        cap: usize::MAX, ptr: 0, len: 0, u0: 0.0, v0: 0.0, uw: 0.0, vh: 0.0, w: 0.0, h: 0.0,
    };
    calc(&mut out, store, champ.as_ptr(), champ.len(), tw, th, scale);
    if out.cap == usize::MAX || !ptr_ok(out.ptr) || out.len == 0 || out.w <= 0.0 || out.h <= 0.0 {
        return false;
    }
    let cx0 = illust[0] + (illust[2] - out.w) * 0.5;
    let cy0 = illust[1] + (illust[3] - out.h) * 0.5;
    let x = if flip { cx0 + out.w } else { cx0 };
    {
        let build: ImgBuildFn = core::mem::transmute(gfn(RVA_IMG_BUILD));
        let set_uv: ImgUvFn = core::mem::transmute(gfn(RVA_IMG_UV));
        let set_flag: ImgFlagFn = core::mem::transmute(gfn(RVA_IMG_FLAG));
        let set_color: ImgColorFn = core::mem::transmute(gfn(RVA_IMG_COLOR));
        let submit: Submit2Fn = core::mem::transmute(gfn(RVA_SUBMIT));
        let sheet_key = core::slice::from_raw_parts(out.ptr as *const u8, out.len);
        let mut a = Cmd::zero();
        let mut b = Cmd::zero();
        build(&mut a, sheet_key.as_ptr(), sheet_key.len(), x, cy0, 0x4bc, out.w, out.h,
            0.0, 0.0, 0.0, 0.0);
        let uv = [out.u0, out.v0, out.uw, out.vh];
        set_uv(&mut b, &a, &uv);
        set_flag(&mut a, &b, 1); // 1 = nearest 샘플링(픽셀아트)
        let fade = [1.0f32, 1.0, 1.0, c01(t)];
        set_color(&mut b, &a, b"color".as_ptr(), 5, &fade);
        let fin: &mut Cmd = if greyscale != 0 {
            let wrap: ImgShaderFn = core::mem::transmute(gfn(RVA_IMG_SHADER));
            let shader = b"asset/base/shader/greyscale";
            wrap(&mut a, &b, shader.as_ptr(), shader.len());
            &mut a
        } else {
            &mut b
        };
        if flip {
            fin.b[0xad] = 1; // flip_x — c7c0/cd40/ff0c20/e850 전부 보존 확인됨
        }
        submit(list, fin);
    }
    game_free(out.ptr); // 0.5.3: 게임 dealloc 소멸 → HeapFree 직접
    true
}

// ── 훅 A: RVA_FX_SET — 진영/모드 스태시 후 원본 ───────────────────────────
unsafe extern "win64" fn hook_fx_set(
    self_: usize,
    ui_ctx: usize,
    app_ctx: usize,
    team_id: u64,
    champ_ptr: usize,
    champ_len: usize,
    is_pick: u8,
) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if ptr_ok(self_) {
            let blue_id = *((self_ + 0x3d0) as *const u64);
            let is_blue = blue_id == team_id;
            LAST_IS_BLUE.store(is_blue as u8, Ordering::Relaxed);
            let mode = if is_pick == 0 { 0u8 } else if is_blue { 1 } else { 2 };
            LAST_MODE.store(mode, Ordering::Relaxed);
            slog(&format!("fx_set mode={mode} is_blue={is_blue}"));
        }
    }));
    let orig = TRAMP_FX_SET.load(Ordering::Relaxed);
    if orig != 0 {
        let f: FxSetFn = core::mem::transmute(orig);
        f(self_, ui_ctx, app_ctx, team_id, champ_ptr, champ_len, is_pick);
    }
}

// ── 훅 C: RVA_ILLUST_GET — 일러 에셋 리다이렉트 (훅 B 실패 시 안전망) ───────
unsafe extern "win64" fn hook_illust_get(
    out: *mut IllustOut,
    store: usize,
    champ_ptr: *const u8,
    champ_len: usize,
    iw: f32,
    ih: f32,
) {
    let redirected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if !config::get().showcase {
            return false;
        }
        // ★카드 경로만 (iw/ih = rect 파생: 바닐라 304x356 / 기하패치 464x284).
        //   슬롯 위젯·미니 아이콘 콜러는 제외 — 슬롯 일러(bpi_illust)와
        //   이중 표시 + 사이드 스태시 타이밍 불일치 방지.
        let is_card = ((iw - 304.0).abs() < 0.5 && (ih - 356.0).abs() < 0.5)
            || ((iw - (GEOM_W - 56.0)).abs() < 0.5 && (ih - (GEOM_H - 124.0)).abs() < 0.5);
        if !is_card {
            return false;
        }
        if out.is_null() || !ptr_ok(champ_ptr as usize) || champ_len == 0 || champ_len > 64 {
            return false;
        }
        let champ = core::slice::from_raw_parts(champ_ptr, champ_len);
        let Ok(champ_str) = core::str::from_utf8(champ) else {
            return false;
        };
        let is_blue = LAST_IS_BLUE.load(Ordering::Relaxed) == 1;
        let Some((key, flip, w, h)) = resolve_art(store, champ_str, is_blue) else {
            return false;
        };
        if flip {
            return false; // 바닐라 경로엔 flip 필드가 없음 → 반전 필요한 아트는 원본(idle)로
        }
        // 키 String은 게임이 자기 힙으로 free → 게임 alloc으로 생성(교차 힙 차단)
        let kb = key.as_bytes();
        let ga: AllocFn = core::mem::transmute(gfn(RVA_GAME_ALLOC));
        let p = ga(0, 0, kb.len()); // 0.5.3 impl 계약: rcx 무시·rdx=flags·r8=size
        if p == 0 {
            return false;
        }
        core::ptr::copy_nonoverlapping(kb.as_ptr(), p as *mut u8, kb.len());
        let uv = cover_uv(w, h, iw.max(1.0), ih.max(1.0));
        (*out).cap = kb.len();
        (*out).ptr = p;
        (*out).len = kb.len();
        (*out).u0 = uv[0];
        (*out).v0 = uv[1];
        (*out).uw = uv[2];
        (*out).vh = uv[3];
        true
    }))
    .unwrap_or(false);
    if redirected {
        return;
    }
    let orig = TRAMP_ILLUST.load(Ordering::Relaxed);
    if orig != 0 {
        let f: IllustGetFn = core::mem::transmute(orig);
        f(out, store, champ_ptr, champ_len, iw, ih);
    } else if !out.is_null() {
        (*out).cap = usize::MAX;
    }
}

// ── 훅 B: RVA_CARD_DRAW — 카드 가로형 커스텀 레이아웃 (밴·픽 공용) ─────────
unsafe extern "win64" fn hook_card_draw(
    store: usize,
    list: usize,
    champ_ptr: *const u8,
    champ_len: usize,
    rect: *const [f32; 4],
    tint: *const [f32; 4],
    greyscale: u8,
    t: f32,
) {
    let drawn = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if !config::get().showcase {
            return false;
        }
        // mode: 0=밴 / 1·2=픽 / 0xff=미정(훅A 미설치·스태시 없음 → 바닐라 폴백)
        let mode = LAST_MODE.load(Ordering::Relaxed);
        if mode > 2 {
            return false;
        }
        if !ptr_ok(store) || !ptr_ok(list) || !ptr_ok(champ_ptr as usize) || champ_len == 0
            || champ_len > 64 || !ptr_ok(rect as usize) || !ptr_ok(tint as usize)
        {
            return false;
        }
        let champ = core::slice::from_raw_parts(champ_ptr, champ_len);
        let Ok(champ_str) = core::str::from_utf8(champ) else {
            return false;
        };
        let is_blue = if mode == 0 { LAST_IS_BLUE.load(Ordering::Relaxed) == 1 } else { mode == 1 };
        // 아트 없어도 idle 폴백을 우리 카드로 직접 그림(레드팀 좌우반전).
        let art = resolve_art(store, champ_str, is_blue);
        // 카드 폭: 기하 패치 성공 시 밴·픽 공통 520, 실패 시 밴만 360(분할 스냅샷 한계).
        let card_w = if mode != 0 || GEOM_PATCHED.load(Ordering::Relaxed) { GEOM_W } else { 360.0 };
        // ★배치 = 호출자 rect의 "중심" 기준 — 콜사이트마다 좌표계가 다름:
        //   일반 밴/픽 = 중심 (0,0) / 분할 스냅샷 = 렌더타깃 내부 좌표(좌상단 원점).
        //   (0,0) 고정 시 스냅샷에서 "우하단 조각만 좌상단에 보임" — 07-25 실증 버그.
        let r = &*rect;
        let (cx, cy) = (r[0] + r[2] * 0.5, r[1] + r[3] * 0.5);
        draw_card(store, list, champ, art, &*tint, greyscale, t, card_w, cx, cy, is_blue);
        true
    }))
    .unwrap_or(false);
    if drawn {
        return;
    }
    let orig = TRAMP_CARD.load(Ordering::Relaxed);
    if orig != 0 {
        let f: CardDrawFn = core::mem::transmute(orig);
        f(store, list, champ_ptr, champ_len, rect, tint, greyscale, t);
    }
}

/// 라운드사각 cmd 구성+제출 (FFI_CONTRACT §3)
unsafe fn submit_round_rect(
    list: usize,
    rect: [f32; 4],
    radius: f32,
    layer: u64,
    stroke_w: f32,
    stroke: [f32; 4],
    fill: [f32; 4],
) {
    let mut c = Cmd::zero();
    c.wr(0x00, 0x800000000000000Au64);
    c.wr(0x0C, radius);
    c.wr(0x1C, 1u32);
    c.wr(0x20, stroke_w);
    c.wr(0x24, stroke[0]);
    c.wr(0x28, stroke[1]);
    c.wr(0x2C, stroke[2]);
    c.wr(0x30, stroke[3]);
    c.wr(0x58, rect[0]);
    c.wr(0x5C, rect[1]);
    c.wr(0x60, rect[2]);
    c.wr(0x64, rect[3]);
    c.wr(0x68, layer);
    c.wr(0x70, fill[0]);
    c.wr(0x74, fill[1]);
    c.wr(0x78, fill[2]);
    c.wr(0x7C, fill[3]);
    let submit: Submit2Fn = core::mem::transmute(gfn(RVA_SUBMIT));
    submit(list, &c);
}

/// 카드 커스텀 드로우 — 가로형 일러 영역, 로컬좌표. 변환·취소선·2분할은 호출자 몫.
/// art = (키, flip, tex_w, tex_h) / None이면 idle 스프라이트 폴백.
unsafe fn draw_card(
    store: usize,
    list: usize,
    champ: &[u8],
    art: Option<(&'static str, bool, f32, f32)>,
    tint: &[f32; 4],
    greyscale: u8,
    t: f32,
    card_w: f32,
    cx: f32,
    cy: f32,
    is_blue: bool,
) {
    let t = c01(t);
    let iw = card_w - 56.0;
    // 폴백(스프라이트)일 땐 아트팩 표준 비율(1710:1044)로 영역 고정 — 카드 크기 일관
    let ih = match art {
        Some((_, _, tw, th)) => (iw * th / tw).clamp(120.0, 480.0),
        None => iw * (1044.0 / 1710.0),
    };
    let card_h = 28.0 + ih + 12.0 + 56.0 + 28.0;
    let x0 = cx - card_w * 0.5;
    let y0 = cy - card_h * 0.5;
    let card = [x0, y0, card_w, card_h];
    let illust = [x0 + 28.0, y0 + 28.0, iw, ih];
    let plate = [x0 + 28.0, y0 + card_h - 84.0, card_w - 56.0, 56.0];
    let (pr, pg, pb, pa) = (tint[0], tint[1], tint[2], tint[3]);

    // 장식 (원본 값 계승 — FFI_CONTRACT §3 표)
    submit_round_rect(
        list,
        [card[0] - 10.0, card[1] - 10.0, card[2] + 20.0, card[3] + 20.0],
        22.0,
        0x4b8,
        8.0,
        [pr, pg, pb, c01(0.05 * t) * pa],
        [pr, pg, pb, c01(0.18 * t) * pa],
    );
    submit_round_rect(
        list,
        card,
        18.0,
        0x4ba,
        3.0,
        [0.0627, 0.0706, 0.1020, (0.98 * t).min(1.0)],
        [pr, pg, pb, (0.95 * t).min(1.0) * pa],
    );
    submit_round_rect(
        list,
        illust,
        14.0,
        0x4bb,
        1.0,
        [0.0275, 0.0314, 0.0431, (0.92 * t).min(1.0)],
        [0.2902, 0.2980, 0.3373, (0.55 * t).min(1.0)],
    );

    // ── 일러: 아트팩 이미지 or idle 스프라이트 폴백 ──
    if let Some((key, flip, tex_w, tex_h)) = art {
        let build: ImgBuildFn = core::mem::transmute(gfn(RVA_IMG_BUILD));
        let set_uv: ImgUvFn = core::mem::transmute(gfn(RVA_IMG_UV));
        let set_flag: ImgFlagFn = core::mem::transmute(gfn(RVA_IMG_FLAG));
        let set_color: ImgColorFn = core::mem::transmute(gfn(RVA_IMG_COLOR));
        let submit: Submit2Fn = core::mem::transmute(gfn(RVA_SUBMIT));
        let kb = key.as_bytes();
        let mut a = Cmd::zero();
        let mut b = Cmd::zero();
        // flip_x=1이면 화면 커버 [x−w, x] → x에 오른쪽 끝 전달
        let ix = if flip { illust[0] + illust[2] } else { illust[0] };
        build(&mut a, kb.as_ptr(), kb.len(), ix, illust[1], 0x4bc, illust[2], illust[3],
            0.0, 0.0, 0.0, 0.0);
        let uv = cover_uv(tex_w, tex_h, illust[2], illust[3]);
        set_uv(&mut b, &a, &uv);
        set_flag(&mut a, &b, 0);
        let fade = [1.0f32, 1.0, 1.0, t];
        set_color(&mut b, &a, b"color".as_ptr(), 5, &fade);
        let fin: &mut Cmd = if greyscale != 0 {
            let wrap: ImgShaderFn = core::mem::transmute(gfn(RVA_IMG_SHADER));
            let shader = b"asset/base/shader/greyscale";
            wrap(&mut a, &b, shader.as_ptr(), shader.len());
            &mut a
        } else {
            &mut b
        };
        if flip {
            fin.b[0xad] = 1;
        }
        submit(list, fin);
    } else {
        // 아트 없음 → idle 스프라이트(레드팀 좌우반전). 실패 시 플레이트-온리(무해).
        let _ = draw_idle_sprite(store, list, champ, &illust, greyscale, t, !is_blue);
    }

    // ── 네임플레이트 + 이름 텍스트 ──
    submit_round_rect(
        list,
        plate,
        12.0,
        0x4bd,
        2.0,
        [0.0588, 0.0627, 0.0863, (0.96 * t).min(1.0)],
        [pr, pg, pb, (0.72 * t).min(1.0) * pa],
    );
    {
        let name_get: NameGetFn = core::mem::transmute(gfn(RVA_NAME_GET));
        let mut name = GStr { cap: 0, ptr: 1, len: 0 };
        name_get(&mut name, store, champ.as_ptr(), champ.len());
        if ptr_ok(name.ptr) && name.len > 0 && name.len < 256 {
            let tb: TextBuildFn = core::mem::transmute(gfn(RVA_TEXT_BUILD));
            let submit_t: Submit2Fn = core::mem::transmute(gfn(RVA_SUBMIT_TEXT));
            let mut c = Cmd::zero();
            let rgba = [0.9098f32, 0.9098, 0.9098, t];
            let outline = Outline { a: 0, b: 0, c: t * 0.8667 };
            let font = b"asset/base/font/set/bold";
            tb(&mut c, name.ptr as *const u8, name.len, font.as_ptr(), font.len(), &rgba, &plate,
                Z_NAME_TEXT, 30.0, 1, 1, &outline, 4.0);
            submit_t(list, &c);
        }
        if name.cap != 0 && name.cap != usize::MAX && ptr_ok(name.ptr) {
            game_free(name.ptr); // 0.5.3: 게임 dealloc 소멸 → HeapFree 직접
        }
    }
}

// ── 기하 확대 패치 ──────────────────────────────────────────────────────────
unsafe fn wr_bytes(addr: usize, bytes: &[u8]) -> bool {
    let mut old: u32 = 0;
    if VirtualProtect(addr, bytes.len(), RWX, &mut old) == 0 {
        return false;
    }
    core::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, bytes.len());
    VirtualProtect(addr, bytes.len(), old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, bytes.len());
    true
}
unsafe fn f32s_at(addr: usize, n: usize) -> Vec<f32> {
    (0..n).map(|i| core::ptr::read_unaligned((addr + i * 4) as *const f32)).collect()
}
fn f32s_eq(a: &[f32], b: &[f32]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (x - y).abs() < 0.001)
}
fn le(vals: &[f32]) -> Vec<u8> {
    vals.iter().flat_map(|v| v.to_le_bytes()).collect()
}
unsafe fn disp_target_is(disp_loc: usize, expect: f32) -> bool {
    let disp = core::ptr::read_unaligned(disp_loc as *const i32) as isize;
    let tgt = (disp_loc + 4).wrapping_add_signed(disp);
    ptr_ok(tgt) && (core::ptr::read_unaligned(tgt as *const f32) - expect).abs() < 0.001
}
unsafe fn retarget_disp(disp_loc: usize, new_target: usize) -> bool {
    let disp = new_target as isize - (disp_loc + 4) as isize;
    if disp > i32::MAX as isize || disp < i32::MIN as isize {
        return false;
    }
    wr_bytes(disp_loc, &(disp as i32).to_le_bytes())
}

/// 밴 분할 연출 기하를 520×408로 확대. 12사이트 전부 사전검증 통과 시에만 일괄 적용,
/// 하나라도 불일치(패치로 어긋남)면 전체 스킵 → 밴 카드 360 폴백(안전).
unsafe fn apply_geom_patches(base: usize) {
    let cut = GEOM_CUT;
    let (w, h) = (GEOM_W, GEOM_H);
    let d = h - 2.0 * cut;
    let nlen = (d * d + w * w).sqrt();
    let (nx, ny) = (d / nlen, w / nlen);
    let checks = [
        f32s_eq(&f32s_at(base + RVA_C_CARD_RECT, 4), &[-180.0, -240.0, 360.0, 480.0]),
        f32s_eq(&f32s_at(base + RVA_C_SNAP_RECT, 4), &[0.0, 0.0, 360.0, 480.0]),
        f32s_eq(&f32s_at(base + RVA_C_LINE_DIR, 2), &[360.0, 340.0]),
        f32s_eq(&f32s_at(base + RVA_C_LINE_START, 2), &[-180.0, 170.0]),
        f32s_eq(&f32s_at(base + RVA_C_LINE_ANCHOR, 2), &[0.0, 170.0]),
        f32s_eq(&f32s_at(base + RVA_C_NORMAL, 2), &[0.6866, 0.727]),
        core::ptr::read_unaligned((base + RVA_I_SNAP_H) as *const u32) == 0x43F00000,
        disp_target_is(base + RVA_D_SNAP_W, 360.0),
        disp_target_is(base + RVA_D_CUT_LO, -70.0),
        disp_target_is(base + RVA_D_CUT_HI, 70.0),
        disp_target_is(base + RVA_D_ZIG_X1, -180.0),
        disp_target_is(base + RVA_D_ZIG_X2, -180.0),
        f32s_at(base + RVA_SLOTS, 4).iter().all(|v| *v == 0.0),
    ];
    if let Some(i) = checks.iter().position(|c| !c) {
        slog(&format!("geom patch SKIP: precheck #{i} mismatch"));
        return;
    }
    let slots = base + RVA_SLOTS;
    let ok = wr_bytes(slots, &le(&[w, -cut, cut, -w * 0.5]))
        && wr_bytes(base + RVA_C_CARD_RECT, &le(&[-w * 0.5, -h * 0.5, w, h]))
        && wr_bytes(base + RVA_C_SNAP_RECT, &le(&[0.0, 0.0, w, h]))
        && wr_bytes(base + RVA_C_LINE_DIR, &le(&[w, d]))
        && wr_bytes(base + RVA_C_LINE_START, &le(&[-w * 0.5, h * 0.5 - cut]))
        && wr_bytes(base + RVA_C_LINE_ANCHOR, &le(&[0.0, h * 0.5 - cut]))
        && wr_bytes(base + RVA_C_NORMAL, &le(&[nx, ny]))
        && wr_bytes(base + RVA_I_SNAP_H, &h.to_le_bytes())
        && retarget_disp(base + RVA_D_SNAP_W, slots)
        && retarget_disp(base + RVA_D_CUT_LO, slots + 4)
        && retarget_disp(base + RVA_D_CUT_HI, slots + 8)
        && retarget_disp(base + RVA_D_ZIG_X1, slots + 12)
        && retarget_disp(base + RVA_D_ZIG_X2, slots + 12);
    if ok {
        GEOM_PATCHED.store(true, Ordering::Relaxed);
        slog("geom patches applied (520x408)");
    } else {
        slog("geom patch PARTIAL FAIL");
    }
}

// ── 설치 (대체형: 진입 패치 + 원본 트램폴린. 체인 없음 — 전용 훅 3지점) ──────
unsafe fn install_replace(
    rva: usize,
    orig_len: usize,
    prologue: &[u8],
    repl: usize,
) -> Result<usize, &'static str> {
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 {
        return Err("base 0");
    }
    let fn_addr = base + rva;
    if *(fn_addr as *const u8) == 0x48 && *((fn_addr + 1) as *const u8) == 0xb8 {
        return Err("foreign hook");
    }
    for (i, b) in prologue.iter().enumerate() {
        if *((fn_addr + i) as *const u8) != *b {
            return Err("prologue mismatch");
        }
    }
    let stub = VirtualAlloc(0, 128, MEM_CR, RWX);
    if stub == 0 {
        return Err("VirtualAlloc");
    }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x49, 0xbb]); // movabs r11, ret (r11 = scratch, 진입부 미사용 확인)
    s.extend_from_slice(&ret_addr.to_le_bytes());
    s.extend_from_slice(&[0x41, 0xff, 0xe3]); // jmp r11
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0] = 0x48;
    patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&repl.to_le_bytes());
    patch[10] = 0xff;
    patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 {
        return Err("VirtualProtect");
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(stub)
}

/// 매 프레임 post_update에서 호출 — 1회 설치(재설치 금지, 07-18 상호 체인 사이클 교훈).
pub fn tick() {
    if !config::get().showcase {
        return;
    }
    unsafe { install() };
}

unsafe fn install() {
    if INSTALLED.swap(true, Ordering::Relaxed) {
        return;
    }
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 {
        INSTALLED.store(false, Ordering::Relaxed);
        return;
    }
    BASE.store(base, Ordering::Relaxed);
    match install_replace(RVA_FX_SET, FX_SET_ORIG_LEN, FX_SET_PROLOGUE, hook_fx_set as usize) {
        Ok(tr) => TRAMP_FX_SET.store(tr, Ordering::Relaxed),
        Err(e) => slog(&format!("hook A FAIL: {e}")),
    }
    match install_replace(RVA_CARD_DRAW, CARD_DRAW_ORIG_LEN, CARD_DRAW_PROLOGUE, hook_card_draw as usize)
    {
        Ok(tr) => TRAMP_CARD.store(tr, Ordering::Relaxed),
        Err(e) => slog(&format!("hook B FAIL: {e}")),
    }
    match install_replace(RVA_ILLUST_GET, ILLUST_GET_ORIG_LEN, ILLUST_GET_PROLOGUE, hook_illust_get as usize)
    {
        Ok(tr) => TRAMP_ILLUST.store(tr, Ordering::Relaxed),
        Err(e) => slog(&format!("hook C FAIL: {e}")),
    }
    apply_geom_patches(base);
    slog("showcase hooks installed");
}
