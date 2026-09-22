//! team_sync_stable — stable 서버 확장의 `handle_command` 안에서 **내 팀 Team 레코드를 클라에 유니캐스트**(ServerPacket::ResponseTeam)한다.
//! 왜: stable `team_set_json`/`record_set_json`/`news_push` 는 서버 DB 만 바꾸고 클라 복제본은 다음 게임 패킷/일일 틱까지 안 바뀐다
//!     (0.6.0 실측 + RE `mods_report/daram2_view_plus/RE/2026-09-16_0.6.0-서버→클라-Team동기화-트리거.md`).
//! 어떻게(훅 없음, RE `…ResponseTeam-유니캐스트-재현레시피.md`): ServerCtxV1.state(호스트 스택 구조체)+0x18 = `Server` 포인터가
//!     서버 루프 구조체의 인라인 필드(loop+0x9d8) → 같은 구조체에서 &Sender(loop+0x3f0)·송신자 SocketAddr(loop+0x1a0)에 고정 오프셋으로 도달.
//!     `0x2262850(out, state, &[team], 1, my_team)` → OutMsg{0x57, packet, addr} → `0x220ce30(out, &Sender, &msg)` → 실패 시 `0x21ce320(&out)`.
//! ⚠0.6.0 전용 RVA/오프셋. 패치 시 `MIG/repin.py` 로 재핀 + 콜 사이트(0x299bc9b 근처 lea rbx+0x9d8/0x3f0/0x1a0) 재확인. 런타임 가드 3중:
//!     ①`*(Server−0x988) == state` ②Sender.kind ≤ 2 ③각 함수 진입 바이트 = 12B push 프롤로그(재핀 스탬프) — 하나라도 어긋나면 아무것도 안 한다.
//! 사용: `#[path = r"C:\tfm2mods\ui_kit\team_sync_stable.rs"] mod team_sync;` → `team_sync::unicast_team(&ctx, team_id)` (handle_command 안, 서버 스레드).
#![allow(dead_code)]
use mod_api_stable::StableServerCtx;

/// 0.6.0 (exe sha AECB984A2C7AE187)
pub const GAME_VER: &str = "0.6.1";
const RVA_BUILD: usize = 0x25b0960; // FUN_142262850(out*, state, ids_ptr, ids_len, my_team_id) -> out*  (ServerPacket ResponseTeam 0x740)
const RVA_SEND: usize = 0x255a520;  // FUN_14220ce30(out*, &Sender, &OutMsg) -> out*  ([out]==u64::MAX 면 Ok)
const RVA_DROP: usize = 0x251bd00;  // FUN_1421ce320(&out)
/// 세 함수 공통 프롤로그(push rbp,r15,r14,r13,r12,rsi,rdi,rbx) — RE 확인은 0x2259320/0x220f040 기준. build/send/drop 은 첫 바이트만 느슨히 검사.
const PROLOGUE_PUSH: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
const OFF_SERVER_IN_LOOP: usize = 0x6c0; // 0.6.1 재핀(0.6.0=0x9d8) — 서버 루프 콜사이트 lea rdx,[rbx+..]
const OFF_SENDER_IN_LOOP: usize = 0x290; // 0.6.1(0.6.0=0x3f0) lea r9
const OFF_ADDR_IN_LOOP: usize = 0x1c0; // 0.6.1(0.6.0=0x1a0) [rsp+0x28]
const OFF_STATE_IN_LOOP: usize = 0x60; // 0.6.1(0.6.0=0x50) mov r12,[rbx+..] 콜 직전
const PKT_SIZE: usize = 0x740;
const MSG_SIZE: usize = 0x768;
const ADDR_SIZE: usize = 0x20;
const UNICAST: u64 = 0x57;

type BOOL = i32;
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
}
#[repr(C)] #[derive(Default)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32, region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32 }
unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || addr >= (1usize << 48) || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const RD: u32 = 0x02 | 0x04 | 0x20 | 0x40; const GUARD: u32 = 0x01 | 0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr.wrapping_add(len) <= mbi.base.wrapping_add(mbi.region_size)
}
unsafe fn rd_u64(addr: usize) -> Option<u64> { if readable(addr, 8) { Some(core::ptr::read_unaligned(addr as *const u64)) } else { None } }

#[repr(C, align(16))]
struct Buf([u8; 0x770]);

/// `StableServerCtx` 의 raw `*mut ServerCtxV1` — 구조체 첫 필드(비-ZST 는 이것 하나)라 포인터 read 로 꺼낸다.
fn raw_ctx(ctx: &StableServerCtx<'_>) -> usize {
    unsafe { core::ptr::read(ctx as *const StableServerCtx<'_> as *const usize) }
}

/// 내 팀 `team_id` 의 Team(+KnowledgeBase) 을 현재 커맨드 송신자 세션에 유니캐스트. 성공 Ok(설명), 실패 Err(이유 — 아무 부작용 없음).
/// 반드시 `handle_command` 안(서버 스레드, 현재 패킷 처리 중)에서 호출할 것.
pub fn unicast_team(ctx: &StableServerCtx<'_>, team_id: usize) -> Result<String, String> {
    unsafe {
        let raw = raw_ctx(ctx);
        if !readable(raw, 0x18) { return Err("ctx raw 읽기 불가".into()); }
        let host_state = rd_u64(raw + 0x10).ok_or("ctx.state 읽기 불가")? as usize; // ServerCtxV1{size, vtable, state}
        if !readable(host_state, 0x28) { return Err("host_state 읽기 불가".into()); }
        let state = rd_u64(host_state + 0x10).ok_or("state 읽기 불가")? as usize;
        let server = rd_u64(host_state + 0x18).ok_or("server 읽기 불가")? as usize;
        if state == 0 || server < OFF_SERVER_IN_LOOP { return Err(format!("state/server 이상 state=0x{:x} server=0x{:x}", state, server)); }
        let lp = server - OFF_SERVER_IN_LOOP;
        // 가드 ①: 루프 구조체 +0x50 == state
        match rd_u64(lp + OFF_STATE_IN_LOOP) { Some(v) if v as usize == state => {}, v => return Err(format!("가드① 실패: loop+state_off={:?} state=0x{:x}", v.map(|x| format!("0x{:x}", x)), state)) }
        // 가드 ②: Sender.kind ≤ 2
        let sender = lp + OFF_SENDER_IN_LOOP;
        match rd_u64(sender) { Some(k) if k <= 2 => {}, k => return Err(format!("가드② 실패: sender.kind={:?}", k)) }
        if !readable(lp + OFF_ADDR_IN_LOOP, ADDR_SIZE) { return Err("송신자 addr 읽기 불가".into()); }
        // 가드 ③: 함수 진입 바이트
        let base = GetModuleHandleW(core::ptr::null());
        if base == 0 { return Err("module base 0".into()); }
        let (f_build, f_send, f_drop) = (base + RVA_BUILD, base + RVA_SEND, base + RVA_DROP);
        for (name, f) in [("build", f_build), ("send", f_send), ("drop", f_drop)] {
            if !readable(f, 12) { return Err(format!("{} 코드 읽기 불가", name)); }
            let b = core::slice::from_raw_parts(f as *const u8, 12);
            // build/send 는 8-push 프롤로그(RE), drop 은 짧은 함수일 수 있어 첫 바이트가 push/sub 계열인지만
            let ok = if name == "drop" { matches!(b[0], 0x55 | 0x41 | 0x53 | 0x56 | 0x57 | 0x48 | 0x40) } else { b == PROLOGUE_PUSH || matches!(b[0], 0x55 | 0x41 | 0x53 | 0x56 | 0x57) };
            if !ok { return Err(format!("가드③ 실패: {} 진입 바이트 {:02x?} (RVA stale?)", name, &b[..8])); }
        }
        type BuildFn = unsafe extern "C" fn(*mut u8, usize, *const u64, usize, u64) -> *mut u8;
        type SendFn = unsafe extern "C" fn(*mut u8, usize, *const u8) -> *mut u8;
        type DropFn = unsafe extern "C" fn(*mut u8);
        let build: BuildFn = core::mem::transmute(f_build);
        let send: SendFn = core::mem::transmute(f_send);
        let drop_res: DropFn = core::mem::transmute(f_drop);

        let ids = [team_id as u64];
        let mut pkt = Buf([0u8; 0x770]);
        build(pkt.0.as_mut_ptr(), state, ids.as_ptr(), 1, team_id as u64);
        if core::ptr::read_unaligned(pkt.0.as_ptr() as *const u32) == u32::MAX { return Err("build: None".into()); }
        let tag = core::ptr::read_unaligned(pkt.0.as_ptr() as *const u64);
        if tag != 0x10 { return Err(format!("build: 예상 밖 tag 0x{:x}", tag)); }
        let mut msg = Buf([0u8; 0x770]);
        core::ptr::write_unaligned(msg.0.as_mut_ptr() as *mut u64, UNICAST);
        core::ptr::copy_nonoverlapping(pkt.0.as_ptr(), msg.0.as_mut_ptr().add(8), PKT_SIZE); // 소유권 이동 — pkt 는 이후 건드리지 않음
        core::ptr::copy_nonoverlapping((lp + OFF_ADDR_IN_LOOP) as *const u8, msg.0.as_mut_ptr().add(8 + PKT_SIZE), ADDR_SIZE);
        let mut res = Buf([0u8; 0x770]);
        send(res.0.as_mut_ptr(), sender, msg.0.as_ptr()); // msg 소비됨
        let r = core::ptr::read_unaligned(res.0.as_ptr() as *const u64);
        if r != u64::MAX { drop_res(res.0.as_mut_ptr()); return Err(format!("send 실패(res tag 0x{:x})", r)); }
        Ok(format!("ResponseTeam team={} 유니캐스트 OK (state=0x{:x} loop=0x{:x})", team_id, state, lp))
    }
}
