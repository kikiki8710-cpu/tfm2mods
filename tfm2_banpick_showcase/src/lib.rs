//! tfm2_banpick_showcase — 밴픽 쇼케이스 카드(밴 취소선 카드 / 픽 중앙 비행 카드) 일러팩 적용
//! ===========================================================================
//! 근거 RE = FFI_CONTRACT.md (0.5.2, 2026-07-25 ghidra-re 확정). 전 RVA 0.5.2 기준.
//! 구조:
//!   훅 A 0x11e2370(연출 상태 세팅) = 진영/모드/챔프 스태시 → 원본 (관찰만)
//!   훅 B 0x11f9030(카드 드로우 헬퍼) = 픽이면 가로형 커스텀 레이아웃 전체 대체,
//!        밴/아트부재/비활성이면 원본 트램폴린 폴백
//!   훅 C 0xfdabe0(밴픽 일러 에셋 조회) = 키를 아트팩(banpick_illust 자동등록 키)으로
//!        리다이렉트 — 밴 카드(2분할 포함 바닐라 지오메트리) 아트 교체용. 부재 시 원본.
//! 아트 = tfm2_banpick_illust 모드가 자동 등록하는
//!   asset/tfm2_banpick_illust/illust/{blue|red|red_noflip}/<champ>[-1]
//!   (그 모드가 비활성이면 키 부재 → 전부 원본 폴백 = 안전)
//! 안전수칙: detour 본문 catch_unwind 격리, 포인터 range 가드, 게임이 free하는
//!   String은 게임 alloc(0x8b7f80)으로 생성(교차 힙 원천 차단), 훅은 1회 설치(재설치 금지).
//! ===========================================================================
#![allow(dead_code, unused_imports, non_snake_case)]
use mod_api::*;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU32, AtomicUsize, Ordering};

const MOD_ID: &str = "tfm2_banpick_showcase";

// build_inj.ps1 신원 검증용 — dll 안에 소스 절대경로 문자열이 있어야 배포됨(stale/타모드 차단).
// 이 모드는 패닉 사이트가 최적화로 제거돼 경로가 안 박히는 케이스라 export 함수로 명시적으로 박는다.
// (#[used] static은 링커가 문자열까지 못 지킴 — export 심볼은 cdylib에서 확실히 유지.)
#[no_mangle]
pub extern "C" fn tfm2_banpick_showcase_src_id() -> *const u8 {
    concat!(file!(), "\0").as_bytes().as_ptr()
}

// ── 0.5.2 RVA (패치 시 migrate_rva.py 대상) ──────────────────────────────────
const RVA_FX_SET: usize = 0x11e2370; // 훅 A: 연출 상태 세팅 (진영 스태시)
const RVA_CARD_DRAW: usize = 0x11f9030; // 훅 B: 카드 드로우 헬퍼
const RVA_ILLUST_GET: usize = 0xfdabe0; // 훅 C: 밴픽 일러 에셋 조회
// draw cmd 체인 (FFI_CONTRACT.md §4~5)
const RVA_SUBMIT: usize = 0x248b1c0; // b1c0(list, &cmd) 일반 제출
const RVA_SUBMIT_TEXT: usize = 0x248b400; // b400(list, &cmd) 텍스트 전용
const RVA_IMG_BUILD: usize = 0x248c130; // c130(&cmd, key, len, x, y, layer, w, h, 0,0,0,0)
const RVA_IMG_UV: usize = 0x248c7c0; // c7c0(&out, &in, &uv)
const RVA_IMG_FLAG: usize = 0x248cd40; // cd40(&out, &in, flag)
const RVA_IMG_COLOR: usize = 0xff0c20; // ff0c20(&out, &in, "color", 5, &rgba)
const RVA_IMG_SHADER: usize = 0x248e850; // e850(&out, &in, shader_key, len)
const RVA_TEXT_BUILD: usize = 0x248c1e0; // c1e0(...) 텍스트 cmd
const RVA_NAME_GET: usize = 0x1217630; // 챔프 표시명 String
const RVA_ASSET_GET: usize = 0x99c860; // 키→텍스처 에셋 (obj,vtbl) 엔트리 주소
const RVA_ANIM_GET: usize = 0x5ab7d0; // 키→애님 리소스 (참조 반환)
const RVA_SPRITE_CALC: usize = 0x121aca0; // idle 시트키+UV+크기 계산기(무부작용)
const RVA_GAME_ALLOC: usize = 0x8b7f80; // (size, align) → ptr
const RVA_GAME_FREE: usize = 0x8b7f90; // (ptr, size, align)

// ── 진입부 검증/재배치 상수 (0.5.2 실측 — ghidra-re 2026-07-25 확정) ──────────
// 세 함수 모두 orig_len 구간에 rip-relative·분기타깃·chkstk 없음 → 재배치 안전.
// 0x11e2370 / 0x11f9030: push 8개 = 정확히 12B 경계.
const FX_SET_ORIG_LEN: usize = 12;
const FX_SET_PROLOGUE: &[u8] =
    &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
const CARD_DRAW_ORIG_LEN: usize = 12;
const CARD_DRAW_PROLOGUE: &[u8] =
    &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
// 0xfdabe0: push 4B + sub rsp,0x98 7B + lea rbp,[rsp+0x80] 8B = 19B(11B에서 걸려 lea까지).
const ILLUST_GET_ORIG_LEN: usize = 19;
const ILLUST_GET_PROLOGUE: &[u8] = &[
    0x55, 0x56, 0x57, 0x53, 0x48, 0x81, 0xEC, 0x98, 0x00, 0x00, 0x00, 0x48, 0x8D, 0xAC, 0x24,
    0x80, 0x00, 0x00, 0x00,
];

// ── 밴 분할 연출 기하 확대 패치 (360×480 → 520×408) ─────────────────────────
// RE 패치 테이블(2026-07-25, FFI_CONTRACT.md §기하패치) — 전부 0.5.2 RVA.
// 배타 상수는 값 교체, 공유 상수는 disp 재타깃(.rdata 패딩에 우리 float 슬롯).
// 취소선 기하 일관식: cut=60, dir={w, h−2·cut}, start={−w/2, h/2−cut},
//   normal=(h−2·cut, w)/‖·‖. (바닐라 360×480·cut70으로 식 검증됨)
const GEOM_W: f32 = 520.0;
const GEOM_H: f32 = 408.0;
const GEOM_CUT: f32 = 60.0;
// A. 배타 rdata 상수 (값 교체)
const RVA_C_CARD_RECT: usize = 0x3731380; // {-180,-240,360,480} 카드 로컬 rect(밴·픽 공용)
const RVA_C_SNAP_RECT: usize = 0x37313b0; // {0,0,360,480} 스냅샷 내부 rect(좌상단 원점)
const RVA_C_LINE_DIR: usize = 0x37313e0; // {360,340} 취소선 방향
const RVA_C_LINE_START: usize = 0x37313f0; // {-180,170} 취소선 시작
const RVA_C_LINE_ANCHOR: usize = 0x3731400; // {0,170} 앵커
const RVA_C_NORMAL: usize = 0x37313c0; // {0.6866,0.727} 분리 법선
// B. 코드 즉치 (스냅샷 렌더타깃 높이 480.0)
const RVA_I_SNAP_H: usize = 0x124e2ba; // mov dword [rsp+0x20], 0x43F00000
// C. 공유 상수 → disp 재타깃 (disp 위치, 현재 타깃 float 기대값)
const RVA_D_SNAP_W: usize = 0x124e2c2; // → 360.0 (스냅샷 폭, 광공유)
const RVA_D_CUT_LO: usize = 0x1201e19; // → -70.0 (1201d90 하단 컷)
const RVA_D_CUT_HI: usize = 0x1201e27; // → +70.0 (1201d90 상단 컷)
const RVA_D_ZIG_X1: usize = 0x124e8cf; // → -180.0 (지그재그 x, 외부 공유)
const RVA_D_ZIG_X2: usize = 0x124efa1; // → -180.0 (〃 두 번째 블록)
// 우리 float 슬롯 (.rdata 끝 패딩 — RE 확인 0x3FD2AD4~0x3FD3000, 사용 전 0 검증)
const RVA_SLOTS: usize = 0x3fd2b00; // [w=520, cut_lo=-60, cut_hi=60, zig_x=-260]
static GEOM_PATCHED: AtomicBool = AtomicBool::new(false);

// ── cfg (배포 기본: debug=0) ────────────────────────────────────────────────
static CFG_ENABLED: AtomicBool = AtomicBool::new(true);
static CFG_RED_FLIP: AtomicBool = AtomicBool::new(true); // 1=red, 0=red_noflip
// 기본 0 = 원본 구도(<champ>.png)만. 모드챔프 아트를 챔프당 1장만 추가해도 되게(유저 요청 07-25).
// 1이면 -1 확대구도 우선. 어느 쪽이든 없으면 반대 변형으로 자동 폴백.
static CFG_ZOOM: AtomicBool = AtomicBool::new(false);
static CFG_PICK_LAYOUT: AtomicBool = AtomicBool::new(true); // 픽 가로형 커스텀 레이아웃
static CFG_BAN_LAYOUT: AtomicBool = AtomicBool::new(true); // 밴도 가로형 전체표시 레이아웃
static CFG_BAN_ART: AtomicBool = AtomicBool::new(true); // 밴 바닐라 레이아웃일 때 아트 리다이렉트(훅 C)
static CFG_DEBUG: AtomicBool = AtomicBool::new(false);

// ── 상태 ──
static BASE: AtomicUsize = AtomicUsize::new(0);
static INSTALLED: AtomicBool = AtomicBool::new(false);
static TRAMP_FX_SET: AtomicUsize = AtomicUsize::new(0);
static TRAMP_CARD: AtomicUsize = AtomicUsize::new(0);
static TRAMP_ILLUST: AtomicUsize = AtomicUsize::new(0);
/// 마지막 셀렉트 확정의 모드: 0=밴 1=픽blue 2=픽red, 0xff=미정 (훅 A가 기록)
static LAST_MODE: AtomicU8 = AtomicU8::new(0xff);
/// 마지막 셀렉트 확정 진영: 1=blue 0=red (밴 포함 — 훅 A의 팀id 비교)
static LAST_IS_BLUE: AtomicU8 = AtomicU8::new(1);
static DBG_SEQ: AtomicU32 = AtomicU32::new(0);

// ── WinAPI ──
const MEM_CR: u32 = 0x1000 | 0x2000;
const RWX: u32 = 0x40;
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(n: *const u16) -> usize;
    fn GetModuleFileNameW(h: usize, buf: *mut u16, n: u32) -> u32;
    fn VirtualAlloc(addr: usize, size: usize, ty: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, prot: u32, old: *mut u32) -> u32;
    fn FlushInstructionCache(h: isize, addr: usize, size: usize) -> u32;
    fn GetCurrentProcess() -> isize;
}

// ── 로그 (게임 exe 기준 동적 경로) ──
fn mod_dir() -> Option<String> {
    let mut buf = [0u16; 520];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), 520) } as usize;
    if n == 0 || n >= 520 {
        return None;
    }
    let exe = String::from_utf16_lossy(&buf[..n]);
    let dir = std::path::Path::new(&exe).parent()?;
    Some(format!("{}\\mods\\{}", dir.display(), MOD_ID))
}
fn dlog(msg: &str) {
    if !CFG_DEBUG.load(Ordering::Relaxed) {
        return;
    }
    if let Some(d) = mod_dir() {
        use std::io::Write;
        let _ = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("{d}\\showcase_log.txt"))
            .and_then(|mut f| writeln!(f, "[{}] {}", DBG_SEQ.fetch_add(1, Ordering::Relaxed), msg));
    }
}

fn load_cfg() {
    let Some(d) = mod_dir() else { return };
    let Ok(txt) = std::fs::read_to_string(format!("{d}\\{MOD_ID}.cfg")) else {
        return;
    };
    for line in txt.lines() {
        let line = line.trim();
        let Some((k, v)) = line.split_once('=') else { continue };
        let on = v.trim() == "1";
        match k.trim() {
            "enabled" => CFG_ENABLED.store(on, Ordering::Relaxed),
            "red_flip" => CFG_RED_FLIP.store(on, Ordering::Relaxed),
            "zoom" => CFG_ZOOM.store(on, Ordering::Relaxed),
            "pick_layout" => CFG_PICK_LAYOUT.store(on, Ordering::Relaxed),
            "ban_layout" => CFG_BAN_LAYOUT.store(on, Ordering::Relaxed),
            "ban_art" => CFG_BAN_ART.store(on, Ordering::Relaxed),
            "debug" => CFG_DEBUG.store(on, Ordering::Relaxed),
            _ => {}
        }
    }
}

// ── 게임 함수 타입 ──────────────────────────────────────────────────────────
type FxSetFn = unsafe extern "win64" fn(usize, usize, usize, u64, usize, usize, u8);
type CardDrawFn = unsafe extern "win64" fn(
    usize,             // ctx (asset store)
    usize,             // draw-list
    *const u8,         // champ ptr
    usize,             // champ len
    *const [f32; 4],   // 카드 rect (로컬)
    *const [f32; 4],   // 틴트 RGBA
    u8,                // greyscale
    f32,               // t
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
type AllocFn = unsafe extern "win64" fn(usize, usize) -> usize;
type FreeFn = unsafe extern "win64" fn(usize, usize, usize);
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
/// 게임 Rust String {cap, ptr, len} (0x1217630 out / fdabe0 out 키)
#[repr(C)]
struct GStr {
    cap: usize,
    ptr: usize,
    len: usize,
}
/// 0xfdabe0 out 구조체 (0x28B): 키 String + cover UV
#[repr(C)]
struct IllustOut {
    cap: usize, // -1(usize::MAX) = 없음 센티널
    ptr: usize,
    len: usize,
    u0: f32,
    v0: f32,
    uw: f32,
    vh: f32,
}
/// 0x121aca0 out 구조체 (0x30B): 시트 키 String + UV + 그릴 크기
#[repr(C)]
struct SpriteOut {
    cap: usize, // -1 = 실패 센티널(타 필드 미기록)
    ptr: usize,
    len: usize,
    u0: f32,
    v0: f32,
    uw: f32,
    vh: f32,
    w: f32, // 그릴 w = min(tw, scale·fw)
    h: f32,
}

#[inline]
fn ptr_ok(p: usize) -> bool {
    (0x10000..1usize << 48).contains(&p)
}

// ── 아트 키 조립 + 에셋 조회 ────────────────────────────────────────────────
/// side: true=blue. 반환 = "asset/tfm2_banpick_illust/illust/<side>/<champ>[-1]"
fn art_key(champ: &[u8], is_blue: bool, zoom: bool) -> Vec<u8> {
    let side: &[u8] = if is_blue {
        b"blue"
    } else if CFG_RED_FLIP.load(Ordering::Relaxed) {
        b"red"
    } else {
        b"red_noflip"
    };
    let mut k = Vec::with_capacity(64 + champ.len());
    k.extend_from_slice(b"asset/tfm2_banpick_illust/illust/");
    k.extend_from_slice(side);
    k.push(b'/');
    k.extend_from_slice(champ);
    if zoom {
        k.extend_from_slice(b"-1");
    }
    k
}

/// 선호 변형(cfg zoom) 우선 조회, 없으면 반대 변형 폴백 — 챔프당 1장만 있어도 동작.
unsafe fn art_lookup(store: usize, champ: &[u8], is_blue: bool) -> Option<(Vec<u8>, f32, f32)> {
    let pref = CFG_ZOOM.load(Ordering::Relaxed);
    for zoom in [pref, !pref] {
        let key = art_key(champ, is_blue, zoom);
        if let Some((_, _, w, h)) = tex_lookup(store, &key) {
            return Some((key, w, h));
        }
    }
    None
}

/// 키→텍스처 (obj, vtbl, w, h). 부재/비텍스처 = None.
unsafe fn tex_lookup(store: usize, key: &[u8]) -> Option<(usize, usize, f32, f32)> {
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
    Some((obj, vtbl, w, h))
}

/// cover-크롭 UV: 대상 (tw,th)에 텍스처 (w,h)를 꽉 채우고 긴 축 중앙 크롭 (fdabe0 동식)
fn cover_uv(w: f32, h: f32, tw: f32, th: f32) -> [f32; 4] {
    let tex_a = w / h;
    let tgt_a = tw / th;
    if tex_a > tgt_a {
        // 텍스처가 더 넓다 → 가로 크롭
        let uw = tgt_a / tex_a;
        [(1.0 - uw) * 0.5, 0.0, uw, 1.0]
    } else {
        let vh = tex_a / tgt_a;
        [0.0, (1.0 - vh) * 0.5, 1.0, vh]
    }
}

// ── idle 스프라이트 폴백 (아트 없는 챔프 — 레드팀 좌우반전) ─────────────────
/// "idle" 태그의 대표 프레임(duration>0 첫 프레임, 없으면 마지막) (fw, fh) 반환.
/// anim obj = hashbrown SwissTable: ctrl ptr+mask, 엔트리 stride 0x30 역방향.
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
            continue; // empty/deleted
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
        // Frame 0x14B {x,y,w,h,duration} — duration>0 첫 프레임, 없으면 마지막
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

/// idle 스프라이트를 일러 영역 중앙에 드로우. flip=레드팀 좌우반전(+0xad).
/// 실패해도 무해(플레이트-온리 = 바닐라 폴백 실패 시와 동일) — 성공 여부 반환.
unsafe fn draw_idle_sprite(
    store: usize,
    list: usize,
    champ: &[u8],
    illust: &[f32; 4],
    greyscale: u8,
    t: f32,
    flip: bool,
) -> bool {
    // anim 리소스 → idle 프레임 크기
    let mut anim_key = Vec::with_capacity(64 + champ.len());
    anim_key.extend_from_slice(b"asset/base/aseprite_resources/champions/");
    anim_key.extend_from_slice(champ);
    anim_key.extend_from_slice(b"#anim");
    let anim_get: AnimGetFn = core::mem::transmute(gfn(RVA_ANIM_GET));
    let anim = anim_get(store, anim_key.as_ptr(), anim_key.len());
    let Some((fw, fh)) = idle_frame_size(anim) else {
        return false;
    };
    // 스케일 (바닐라 식: iw·0.82/fw 등, clamp[2.0, 6.4])
    let (tw, th) = (illust[2] * 0.82, illust[3] * 0.82);
    let mut scale = (tw / fw).min(th / fh);
    if !scale.is_finite() {
        scale = tw / fw;
    }
    scale = scale.clamp(2.0, 6.4);
    // 시트키+UV+크기 계산 (0x121aca0 — 무부작용 순수 계산기)
    let calc: SpriteCalcFn = core::mem::transmute(gfn(RVA_SPRITE_CALC));
    let mut out = SpriteOut {
        cap: usize::MAX, ptr: 0, len: 0, u0: 0.0, v0: 0.0, uw: 0.0, vh: 0.0, w: 0.0, h: 0.0,
    };
    calc(&mut out, store, champ.as_ptr(), champ.len(), tw, th, scale);
    if out.cap == usize::MAX || !ptr_ok(out.ptr) || out.len == 0 || out.w <= 0.0 || out.h <= 0.0 {
        return false;
    }
    // 이미지 cmd 체인 — flip_x=1이면 화면 커버가 [x−w, x]라 x에 "오른쪽 끝"을 넣는다
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
    // 시트 키 String 해제 (aca0가 caller 소유로 반환 — 게임 free 사용)
    let gf: FreeFn = core::mem::transmute(gfn(RVA_GAME_FREE));
    gf(out.ptr, out.cap, 1);
    true
}

// ── 훅 A: 0x11e2370 — 진영/모드 스태시 후 원본 ─────────────────────────────
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
            let mode = if is_pick == 0 {
                0u8
            } else if is_blue {
                1
            } else {
                2
            };
            LAST_MODE.store(mode, Ordering::Relaxed);
            dlog(&format!("fx_set mode={mode} is_blue={is_blue} champ_len={champ_len}"));
        }
    }));
    let orig = TRAMP_FX_SET.load(Ordering::Relaxed);
    if orig != 0 {
        let f: FxSetFn = core::mem::transmute(orig);
        f(self_, ui_ctx, app_ctx, team_id, champ_ptr, champ_len, is_pick);
    }
}

// ── 훅 C: 0xfdabe0 — 일러 에셋 리다이렉트 (밴 경로 + 바닐라 레이아웃 공용) ──
unsafe extern "win64" fn hook_illust_get(
    out: *mut IllustOut,
    store: usize,
    champ_ptr: *const u8,
    champ_len: usize,
    iw: f32,
    ih: f32,
) {
    let redirected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if !CFG_ENABLED.load(Ordering::Relaxed) || !CFG_BAN_ART.load(Ordering::Relaxed) {
            return false;
        }
        // ★카드 경로만 리다이렉트 (타깃 크기 게이트 — iw/ih는 rect에서 파생:
        //   바닐라 360x480 → 304x356 / 기하 패치 520x408 → 464x284).
        //   fdabe0 콜러는 3곳 — 카드(0x11f9030) 외에 픽 슬롯 위젯(0x1220a70)·미니 아이콘도
        //   지나가는데, 거기까지 바꾸면 banpick_illust의 슬롯 일러와 이중 표시 + 사이드
        //   스태시 타이밍 불일치. 카드만 걸러서 안전하게.
        let is_card = ((iw - 304.0).abs() < 0.5 && (ih - 356.0).abs() < 0.5)
            || ((iw - (GEOM_W - 56.0)).abs() < 0.5 && (ih - (GEOM_H - 124.0)).abs() < 0.5);
        if !is_card {
            return false;
        }
        if out.is_null() || !ptr_ok(champ_ptr as usize) || champ_len == 0 || champ_len > 64 {
            return false;
        }
        let champ = core::slice::from_raw_parts(champ_ptr, champ_len);
        let Some((key, w, h)) = art_lookup(store, champ, LAST_IS_BLUE.load(Ordering::Relaxed) == 1)
        else {
            return false;
        };
        // 키 String은 게임이 0x8b7f90(ptr,cap,1)로 free → 게임 alloc으로 생성(교차 힙 차단)
        let ga: AllocFn = core::mem::transmute(gfn(RVA_GAME_ALLOC));
        let p = ga(key.len(), 1);
        if p == 0 {
            return false;
        }
        core::ptr::copy_nonoverlapping(key.as_ptr(), p as *mut u8, key.len());
        let uv = cover_uv(w, h, iw.max(1.0), ih.max(1.0));
        (*out).cap = key.len();
        (*out).ptr = p;
        (*out).len = key.len();
        (*out).u0 = uv[0];
        (*out).v0 = uv[1];
        (*out).uw = uv[2];
        (*out).vh = uv[3];
        dlog(&format!("illust_get redirect ok ({}x{})", w, h));
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
        (*out).cap = usize::MAX; // 안전 폴백: "없음" 센티널
    }
}

// ── 훅 B: 0x11f9030 — 픽 카드 가로형 커스텀 레이아웃 ────────────────────────
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
        if !CFG_ENABLED.load(Ordering::Relaxed) {
            return false;
        }
        // mode: 1/2=픽(pick_layout 게이트), 0=밴(ban_layout 게이트 — 취소선/2분할은
        // 호출자가 카드 위에 그리므로 커스텀 카드에도 그대로 얹힘. greyscale은 우리가
        // 일러 cmd에 직접 래핑). 0xff(훅A 미설치/스태시 없음)=바닐라 폴백.
        let mode = LAST_MODE.load(Ordering::Relaxed);
        let allowed = match mode {
            1 | 2 => CFG_PICK_LAYOUT.load(Ordering::Relaxed),
            0 => CFG_BAN_LAYOUT.load(Ordering::Relaxed),
            _ => false,
        };
        if !allowed {
            return false;
        }
        if !ptr_ok(store) || !ptr_ok(list) || !ptr_ok(champ_ptr as usize) || champ_len == 0
            || champ_len > 64 || !ptr_ok(rect as usize) || !ptr_ok(tint as usize)
        {
            return false;
        }
        let champ = core::slice::from_raw_parts(champ_ptr, champ_len);
        let is_blue = if mode == 0 {
            LAST_IS_BLUE.load(Ordering::Relaxed) == 1
        } else {
            mode == 1
        };
        // 아트 없어도 폴백을 우리 카드로 직접 그린다(idle 스프라이트 + 레드팀 좌우반전).
        let art = art_lookup(store, champ, is_blue);
        // 카드 폭: 픽=520(넓은 가로형). 밴은 기하 패치 성공 시 520(분할 스냅샷도 520x408로
        // 확대됨), 실패 시 360 — 바닐라 360x480 스냅샷에 맞춰 잘림 방지.
        let card_w = if mode != 0 || GEOM_PATCHED.load(Ordering::Relaxed) { GEOM_W } else { 360.0 };
        // ★배치는 호출자가 넘긴 rect의 "중심" 기준 — 콜사이트마다 좌표계가 다르다:
        //   일반 밴/픽 = {-180,-240,360,480}(중심 0,0) / 분할 스냅샷 = 렌더타깃 내부
        //   좌표(좌상단 원점). rect 무시하고 (0,0) 중심에 그리면 스냅샷에서 카드가
        //   사분면 밖으로 나가 "우하단 조각만 좌상단에 보임"(07-25 실증 버그).
        let r = &*rect;
        let (cx, cy) = (r[0] + r[2] * 0.5, r[1] + r[3] * 0.5);
        let art_ref = art.as_ref().map(|(k, w, h)| (k.as_slice(), *w, *h));
        draw_card(store, list, champ, art_ref, &*tint, greyscale, t, card_w, cx, cy, is_blue);
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

#[inline]
fn c01(v: f32) -> f32 {
    v.clamp(0.0, 1.0)
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

/// 카드 커스텀 드로우(밴·픽 공용) — 가로형 일러 영역(아트 원비율 표시), 로컬좌표.
/// 변환(비행/흔들림)·취소선·2분할 스냅샷은 호출자 몫. greyscale!=0이면 일러에 셰이더 래핑.
/// art=None이면 idle 스프라이트 폴백(레드팀 좌우반전).
unsafe fn draw_card(
    store: usize,
    list: usize,
    champ: &[u8],
    art: Option<(&[u8], f32, f32)>,
    tint: &[f32; 4],
    greyscale: u8,
    t: f32,
    card_w: f32,
    cx: f32,
    cy: f32,
    is_blue: bool,
) {
    let t = c01(t);
    // ── 레이아웃: 가로형 카드 (유저 요청 07-25 — "세로형 말고 가로로") ──
    //   일러 = (card_w-56) x (비율 유지) 크롭 없음, 아래 네임플레이트.
    //   (cx,cy) = 호출자 rect 중심(콜사이트별 좌표계 대응 — 호출부 주석 참조).
    //   단 밴은 2분할 스냅샷(360x480) 한계로 card_w=360 (호출부 참조).
    let iw = card_w - 56.0;
    // 폴백(스프라이트)일 땐 아트팩 표준 비율(1710:1044)로 영역 고정 — 카드 크기 일관 유지
    let ih = match art {
        Some((_, tw, th)) => (iw * th / tw).clamp(120.0, 480.0),
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
    if let Some((key, tex_w, tex_h)) = art {
        let build: ImgBuildFn = core::mem::transmute(gfn(RVA_IMG_BUILD));
        let set_uv: ImgUvFn = core::mem::transmute(gfn(RVA_IMG_UV));
        let set_flag: ImgFlagFn = core::mem::transmute(gfn(RVA_IMG_FLAG));
        let set_color: ImgColorFn = core::mem::transmute(gfn(RVA_IMG_COLOR));
        let submit: Submit2Fn = core::mem::transmute(gfn(RVA_SUBMIT));
        let mut a = Cmd::zero();
        let mut b = Cmd::zero();
        build(
            &mut a, key.as_ptr(), key.len(), illust[0], illust[1], 0x4bc, illust[2], illust[3],
            0.0, 0.0, 0.0, 0.0,
        );
        let uv = cover_uv(tex_w, tex_h, illust[2], illust[3]);
        set_uv(&mut b, &a, &uv);
        set_flag(&mut a, &b, 0);
        let fade = [1.0f32, 1.0, 1.0, t];
        set_color(&mut b, &a, b"color".as_ptr(), 5, &fade);
        if greyscale != 0 {
            // 밴 회색화 — 바닐라 계약(일러 cmd만 셰이더 래핑, 장식/이름은 비대상)
            let wrap: ImgShaderFn = core::mem::transmute(gfn(RVA_IMG_SHADER));
            let shader = b"asset/base/shader/greyscale";
            wrap(&mut a, &b, shader.as_ptr(), shader.len());
            submit(list, &a);
        } else {
            submit(list, &b);
        }
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
            tb(
                &mut c, name.ptr as *const u8, name.len, font.as_ptr(), font.len(), &rgba, &plate,
                0x4be, 30.0, 1, 1, &outline, 4.0,
            );
            submit_t(list, &c);
        }
        if name.cap != 0 && name.cap != usize::MAX && ptr_ok(name.ptr) {
            let gf: FreeFn = core::mem::transmute(gfn(RVA_GAME_FREE));
            gf(name.ptr, name.cap, 1);
        }
    }
}

// ── 설치 (대체형: 진입 12B 패치 + 원본 트램폴린. 체인 없음 — 전용 훅 3지점) ──
/// 반환 = 원본 트램폴린 주소. 진입부가 이미 외부 훅이면 실패(대체형은 체인 불가).
unsafe fn install_replace(
    rva: usize,
    orig_len: usize,
    prologue: &[u8],
    repl: usize,
    tail_r11: bool,
) -> Result<usize, &'static str> {
    if orig_len < 12 || prologue.is_empty() {
        return Err("RE 상수 미기입");
    }
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 {
        return Err("base 0");
    }
    let fn_addr = base + rva;
    // 외부 훅 감지 → 대체형은 체인 불가, 설치 포기(고아 방지)
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
    if tail_r11 {
        s.extend_from_slice(&[0x49, 0xbb]); // movabs r11, ret
        s.extend_from_slice(&ret_addr.to_le_bytes());
        s.extend_from_slice(&[0x41, 0xff, 0xe3]); // jmp r11
    } else {
        s.extend_from_slice(&[0x48, 0xb8]); // movabs rax, ret
        s.extend_from_slice(&ret_addr.to_le_bytes());
        s.extend_from_slice(&[0xff, 0xe0]); // jmp rax
    }
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    // entry patch = movabs rax, repl; jmp rax + NOP 패딩
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

// ── 기하 확대 패치 적용 ─────────────────────────────────────────────────────
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
/// disp 사이트 검증: 현재 disp가 가리키는 f32 == expect 인지
unsafe fn disp_target_is(disp_loc: usize, expect: f32) -> bool {
    let disp = core::ptr::read_unaligned(disp_loc as *const i32) as isize;
    let tgt = (disp_loc + 4).wrapping_add_signed(disp);
    ptr_ok(tgt) && (core::ptr::read_unaligned(tgt as *const f32) - expect).abs() < 0.001
}
/// disp 재타깃: disp_loc의 disp32를 new_target 기준으로 재계산해 기록
unsafe fn retarget_disp(disp_loc: usize, new_target: usize) -> bool {
    let disp = new_target as isize - (disp_loc + 4) as isize;
    if disp > i32::MAX as isize || disp < i32::MIN as isize {
        return false;
    }
    wr_bytes(disp_loc, &(disp as i32).to_le_bytes())
}

/// 밴 분할 연출 기하를 520×408로 확대. 전 사이트 사전 검증 → 전부 통과 시에만 적용.
/// 하나라도 불일치(패치로 어긋남)면 전체 스킵 → 밴 카드 360 유지(안전).
unsafe fn apply_geom_patches(base: usize) {
    let cut = GEOM_CUT;
    let (w, h) = (GEOM_W, GEOM_H);
    let d = h - 2.0 * cut; // 288
    let nlen = (d * d + w * w).sqrt();
    let (nx, ny) = (d / nlen, w / nlen);
    // ── 사전 검증 (바닐라 값 확인) ──
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
        f32s_at(base + RVA_SLOTS, 4).iter().all(|v| *v == 0.0), // 패딩 슬롯 비어있나
    ];
    if let Some(i) = checks.iter().position(|c| !c) {
        dlog(&format!("geom patch SKIP: precheck #{i} mismatch"));
        return;
    }
    // ── 적용 ──
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
        dlog("geom patches applied (520x408)");
    } else {
        dlog("geom patch PARTIAL FAIL — 일부만 적용됐을 수 있음");
    }
}

/// 1회 설치 (재설치 금지 — 07-18 상호 체인 사이클 사고 교훈)
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
    match install_replace(RVA_FX_SET, FX_SET_ORIG_LEN, FX_SET_PROLOGUE, hook_fx_set as usize, true)
    {
        Ok(tr) => {
            TRAMP_FX_SET.store(tr, Ordering::Relaxed);
            dlog("hook A (fx_set) installed");
        }
        Err(e) => dlog(&format!("hook A FAIL: {e}")),
    }
    match install_replace(
        RVA_CARD_DRAW, CARD_DRAW_ORIG_LEN, CARD_DRAW_PROLOGUE, hook_card_draw as usize, true,
    ) {
        Ok(tr) => {
            TRAMP_CARD.store(tr, Ordering::Relaxed);
            dlog("hook B (card_draw) installed");
        }
        Err(e) => dlog(&format!("hook B FAIL: {e}")),
    }
    match install_replace(
        RVA_ILLUST_GET, ILLUST_GET_ORIG_LEN, ILLUST_GET_PROLOGUE, hook_illust_get as usize, true,
    ) {
        Ok(tr) => {
            TRAMP_ILLUST.store(tr, Ordering::Relaxed);
            dlog("hook C (illust_get) installed");
        }
        Err(e) => dlog(&format!("hook C FAIL: {e}")),
    }
    // 밴 카드 확대(분할 연출 기하 포함) — 커스텀 레이아웃 켜져 있을 때만
    if CFG_ENABLED.load(Ordering::Relaxed) && CFG_BAN_LAYOUT.load(Ordering::Relaxed) {
        apply_geom_patches(base);
    }
}

// ── SDK 라이프사이클 ──
struct ShowcaseExt;
impl ModExtension for ShowcaseExt {
    fn post_update(&self, _s: &mut Scene, _u: &mut GameUI, _a: &mut Assets, _dt: f32) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            install(); // 늦은 1회 설치(멱등 게이트)
        }));
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    load_cfg();
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(ShowcaseExt);
    reg
}
declare_mod!(init);
