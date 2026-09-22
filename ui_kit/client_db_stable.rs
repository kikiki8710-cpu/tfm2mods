//! client_db_stable — stable 클라 ctx(`StableClient`) → 엔진 `ClientDatabase` 포인터 + 재생 커서/이벤트 길이 (0.6.0 전용 RE 값).
//! 근거: `mods_report/Spectator_Chat/RE/2026-09-17_0.6.0-stable클라ctx→ClientDatabase-경로.md`
//!   db = [[state+0x68]+0x38] + 0x18  (state = ClientCtxV1+0x30 · state+0x68 = &Scene(tag 6 = InGame) · payload+0x30 = Rc<RefCell<ClientDatabase>> · RcBox value +0x18)
//!   자가검증: `[db+0xf178] == ctx.player_team_id()` 불일치면 None(레이아웃 stale).
//! 0.6.0 오프셋: scene 태그 db+0x1360(u64, InGame=0xb) · 라이브 played db+0x15c8 · 라이브 events cap/ptr/len db+0x16a0/0x16a8/0x16b0
//!   · 리플레이 game_view Option db+0x948(None ⇔ u64::MAX) · 리플레이 played db+0xba8 · 리플레이 events db+0xc80/0xc88/0xc90.
//! 사용: `#[path = r"C:\tfm2mods\ui_kit\client_db_stable.rs"] mod cdb;` → `cdb::client_db(ctx)` / `cdb::play_cursor(ctx)`.
#![allow(dead_code)]
use mod_api_stable::StableClient;

pub const GAME_VER: &str = "0.6.1"; // 0.6.1: 0.6.0 오프셋 전부 불변 확인(참조 함수 0x291b10→0x2929c0 disp 7/7 동일 · 0xf178 11/11 · 2026-09-22)
pub const OFF_SCENE_TAG: usize = 0x1360;
pub const SCENE_TAG_INGAME: u64 = 0xb;
pub const OFF_LIVE_PLAYED: usize = 0x15c8;
pub const OFF_LIVE_EV_CAP: usize = 0x16a0;
pub const OFF_LIVE_EV_PTR: usize = 0x16a8;
pub const OFF_LIVE_EV_LEN: usize = 0x16b0;
pub const OFF_GAME_VIEW: usize = 0x948;
pub const OFF_REPLAY_PLAYED: usize = 0xba8;
pub const OFF_REPLAY_EV_CAP: usize = 0xc80;
pub const OFF_REPLAY_EV_PTR: usize = 0xc88;
pub const OFF_REPLAY_EV_LEN: usize = 0xc90;
pub const OFF_PLAYER_TEAM: usize = 0xf178;
const OFF_CTX_STATE: usize = 0x30;

#[link(name = "kernel32")]
extern "system" { fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize; }
#[repr(C)] #[derive(Default)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32, region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32 }
pub unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || addr >= (1usize << 48) || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const RD: u32 = 0x02 | 0x04 | 0x20 | 0x40; const GUARD: u32 = 0x01 | 0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr.wrapping_add(len) <= mbi.base.wrapping_add(mbi.region_size)
}
pub unsafe fn rd_u64(addr: usize) -> Option<u64> { if readable(addr, 8) { Some(core::ptr::read_unaligned(addr as *const u64)) } else { None } }
pub unsafe fn rd_u32(addr: usize) -> Option<u32> { if readable(addr, 4) { Some(core::ptr::read_unaligned(addr as *const u32)) } else { None } }

fn raw_ctx(ctx: &StableClient<'_>) -> usize { unsafe { core::ptr::read(ctx as *const StableClient<'_> as *const usize) } }

/// ClientDatabase 시작 주소. 씬이 InGame(앱 씬)이 아니거나 RefCell 이 borrow_mut 중이거나 자가검증 실패면 None.
pub fn client_db(ctx: &StableClient<'_>) -> Option<usize> {
    unsafe {
        let raw = raw_ctx(ctx);
        if !readable(raw, 0x38) { return None; }
        let state = rd_u64(raw + OFF_CTX_STATE)? as usize;
        let scene = rd_u64(state + 0x68)? as usize;
        if rd_u32(scene)? != 6 { return None; }
        let q = rd_u64(scene + 0x38)? as usize;
        let borrow = rd_u64(q + 0x10)? as i64;
        if borrow < 0 { return None; }
        let db = q + 0x18;
        if !readable(db + OFF_PLAYER_TEAM, 8) { return None; }
        if let Some(t) = ctx.player_team_id() { if rd_u64(db + OFF_PLAYER_TEAM)? as usize != t { return None; } }
        Some(db)
    }
}

pub struct PlayCursor { pub replay: bool, pub played: u64, pub ev_len: u64, pub ev_ptr: u64, pub scene_tag: u64 }
/// 재생 커서(리플레이 game_view 우선, 없으면 라이브 InGame 씬). 불변식(ptr 유효·len ≤ 1e7·played ≤ len+600) 위반 시 None.
pub fn play_cursor(ctx: &StableClient<'_>) -> Option<PlayCursor> {
    unsafe {
        let db = client_db(ctx)?;
        let tag = rd_u64(db + OFF_SCENE_TAG)?;
        let gv = rd_u64(db + OFF_GAME_VIEW)?;
        let (replay, played, len, ptr) = if gv != u64::MAX {
            (true, rd_u64(db + OFF_REPLAY_PLAYED)?, rd_u64(db + OFF_REPLAY_EV_LEN)?, rd_u64(db + OFF_REPLAY_EV_PTR)?)
        } else if tag == SCENE_TAG_INGAME {
            (false, rd_u64(db + OFF_LIVE_PLAYED)?, rd_u64(db + OFF_LIVE_EV_LEN)?, rd_u64(db + OFF_LIVE_EV_PTR)?)
        } else { return None };
        if ptr < 0x10000 || ptr >= (1u64 << 48) || len > 10_000_000 || played > len + 600 { return None; }
        Some(PlayCursor { replay, played, ev_len: len, ev_ptr: ptr, scene_tag: tag })
    }
}
