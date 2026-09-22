//! training_plan_stable — stable 서버 확장 `handle_command` 안에서 **내 팀 TeamTrainingPlan 의 선수별 훈련 챔피언 목록을 쓰고**
//! `ServerPacket::ResponseTrainingPlan` 을 클라에 유니캐스트한다(게임 `ClientPacket::UpdateTrainingPlan` 핸들러 재현).
//! 근거 RE: `mods_report/daram2_view_plus/RE/2026-09-18_0.6.0-훈련계획-TeamTrainingPlan-저장처-응답경로.md` (0.6.0 전용 RVA/오프셋).
//!   · Database(=ServerCtxV1.state)+0x16f08 = LinkedHashMap<team_id, TeamTrainingPlan(0x1f8)> · 노드 0x210 {+0 value, +0x1f8 next, +0x200 prev, +0x208 key} · 센티널 = *(map+0x30)
//!   · plan+0x198 = HashMap<u64 athlete_id, String "champ:ratio|…"(최대 4)> · +0x1ea delegated(코치 위임이면 AI 가 덮으므로 0 으로)
//!   · 게임 함수: clone 0x1d19d80 · drop_plan 0x3059c0 · drop_map 0x2f5410 · insert 0xb50720 · sanitize 0x227c570 · response 0x295d220 · send 0x220ce30 · drop_res 0x21ce320
//! 가드(team_sync 와 동일 3중 + 노드 key 자가검증): 하나라도 어긋나면 아무것도 안 한다. 게임 할당자 = 프로세스 힙 → 우리가 HeapAlloc 로 만든 String 을 게임이 free 해도 호환.
//! 사용: `#[path = r"C:\tfm2mods\ui_kit\training_plan_stable.rs"] mod training_plan;` → handle_command 안에서 `training_plan::apply(&ctx, team_id, &[(athlete_id, "a:25|b:25|c:25|d:25")…])`.
#![allow(dead_code)]
use mod_api_stable::StableServerCtx;

pub const GAME_VER: &str = "0.6.1";
const RVA_CLONE: usize = 0x1e69780;
const RVA_DROP_PLAN: usize = 0x306870;
const RVA_DROP_MAP: usize = 0x2f62c0;
const RVA_INSERT: usize = 0xced790;
const RVA_SANITIZE: usize = 0x25ca570;
const RVA_RESPONSE: usize = 0x228ac80;
const RVA_SEND: usize = 0x255a520;
const RVA_DROP_RES: usize = 0x251bd00;
const OFF_TRAINING_PLANS: usize = 0x16f08;
const OFF_MAP_SENTINEL: usize = 0x30;
const NODE_NEXT: usize = 0x1f8;
const NODE_PREV: usize = 0x200;
const NODE_KEY: usize = 0x208;
const PLAN_SIZE: usize = 0x1f8;
const OFF_CHAMP_MAP: usize = 0x198;
const OFF_DELEGATED: usize = 0x1ea;
const OFF_SERVER_IN_LOOP: usize = 0x6c0; // 0.6.1 재핀(0.6.0=0x9d8) — 서버 루프 콜사이트 lea rdx,[rbx+..]
const OFF_SENDER_IN_LOOP: usize = 0x290; // 0.6.1(0.6.0=0x3f0) lea r9
const OFF_ADDR_IN_LOOP: usize = 0x1c0; // 0.6.1(0.6.0=0x1a0) [rsp+0x28]
const OFF_STATE_IN_LOOP: usize = 0x60; // 0.6.1(0.6.0=0x50) mov r12,[rbx+..] 콜 직전
const PKT_SIZE: usize = 0x740;
const ADDR_SIZE: usize = 0x20;
const UNICAST: u64 = 0x57;
const TAG_RESPONSE_TRAINING_PLAN: u64 = 0x11;
const PUSH8: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
const PROL_DROP_PLAN: [u8; 8] = [0x55, 0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x48];
const PROL_RESPONSE: [u8; 8] = [0x55, 0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x38];
const PROL_INSERT: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x54, 0x56, 0x57, 0x53, 0x48, 0x83];
const PROL_DROP_MAP: [u8; 12] = [0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53];
const PROL_SEND: [u8; 12] = [0x55, 0x56, 0x57, 0x53, 0x48, 0x81, 0xec, 0x18, 0x0f, 0x00, 0x00, 0x48]; // 0.6.0 실측(09-19)

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn GetProcessHeap() -> usize;
    fn HeapAlloc(heap: usize, flags: u32, size: usize) -> usize;
    fn HeapFree(heap: usize, flags: u32, p: usize) -> i32;
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
#[repr(C, align(16))]
struct Plan([u8; PLAN_SIZE]);
/// hashbrown 빈 테이블 ctrl(16×EMPTY). bucket_mask 0 이라 게임이 free 하지 않는다.
#[repr(C, align(16))]
struct Ctrl([u8; 16]);
static EMPTY_CTRL: Ctrl = Ctrl([0xFF; 16]);

fn raw_ctx(ctx: &StableServerCtx<'_>) -> usize { unsafe { core::ptr::read(ctx as *const StableServerCtx<'_> as *const usize) } }

/// 하위 개수 확인용 — plan+0x198 맵의 items.
pub fn champ_map_items(plan: usize) -> Option<u64> { unsafe { rd_u64(plan + OFF_CHAMP_MAP + 0x18) } }

/// 내 팀 `team_id` 의 훈련 계획에서 선수별 훈련 챔피언 문자열을 `targets` 로 **통째로** 교체(목록에 없는 선수는 항목 삭제 = 팀 기본 규칙)하고 클라에 유니캐스트.
/// `targets` 원소 = (athlete_id, "champ:ratio|champ:ratio|…"). 성공 Ok(설명) / 실패 Err(이유 · 부작용 없음(교체 전 실패) 또는 명시).
/// plan+0x198 의 HashMap<u64, String> 항목 전부 읽기(hashbrown ctrl 바이트 순회 · 버킷 = ctrl − (i+1)·32).
unsafe fn read_map_entries(map: usize) -> Vec<(u64, String)> {
    let mut out = Vec::new();
    let (Some(ctrl), Some(mask)) = (rd_u64(map), rd_u64(map + 8)) else { return out };
    let (ctrl, mask) = (ctrl as usize, mask as usize);
    if mask == 0 || mask > 4096 || !readable(ctrl, mask + 1) { return out; }
    for i in 0..=mask {
        let c = *((ctrl + i) as *const u8);
        if c & 0x80 != 0 { continue; }
        let b = ctrl.wrapping_sub((i + 1) * 32);
        if !readable(b, 32) { continue; }
        let key = core::ptr::read_unaligned(b as *const u64);
        let ptr = core::ptr::read_unaligned((b + 0x10) as *const u64) as usize;
        let len = core::ptr::read_unaligned((b + 0x18) as *const u64) as usize;
        if len > 512 || (len > 0 && !readable(ptr, len)) { continue; }
        let s = if len == 0 { String::new() } else { String::from_utf8_lossy(core::slice::from_raw_parts(ptr as *const u8, len)).into_owned() };
        out.push((key, s));
    }
    out
}

/// 내 팀 `team_id` 훈련 계획의 선수별 훈련 챔피언 문자열을 갱신하고 클라에 유니캐스트.
/// `set` = (athlete_id, "champ:ratio|…") 덮어쓰기 · `clear` = 항목 삭제(팀 기본 규칙) · 그 외 선수 항목은 유지.
pub fn apply(ctx: &StableServerCtx<'_>, team_id: usize, set: &[(u64, String)], clear: &[u64]) -> Result<String, String> {
    unsafe {
        let raw = raw_ctx(ctx);
        if !readable(raw, 0x18) { return Err("ctx raw 읽기 불가".into()); }
        let host_state = rd_u64(raw + 0x10).ok_or("ctx.state 읽기 불가")? as usize;
        if !readable(host_state, 0x28) { return Err("host_state 읽기 불가".into()); }
        let db = rd_u64(host_state + 0x10).ok_or("state 읽기 불가")? as usize;
        let server = rd_u64(host_state + 0x18).ok_or("server 읽기 불가")? as usize;
        if db == 0 || server < OFF_SERVER_IN_LOOP { return Err(format!("state/server 이상 db=0x{:x} server=0x{:x}", db, server)); }
        let lp = server - OFF_SERVER_IN_LOOP;
        match rd_u64(lp + OFF_STATE_IN_LOOP) { Some(v) if v as usize == db => {}, v => return Err(format!("가드① 실패: loop+state_off={:?}", v)) }
        let sender = lp + OFF_SENDER_IN_LOOP;
        match rd_u64(sender) { Some(k) if k <= 2 => {}, k => return Err(format!("가드② 실패: sender.kind={:?}", k)) }
        if !readable(lp + OFF_ADDR_IN_LOOP, ADDR_SIZE) { return Err("송신자 addr 읽기 불가".into()); }
        let base = GetModuleHandleW(core::ptr::null());
        if base == 0 { return Err("module base 0".into()); }
        // 가드 ③: 진입 바이트 스탬프
        let chk = |name: &str, rva: usize, want: &[u8]| -> Result<usize, String> {
            let f = base + rva;
            if !readable(f, 12) { return Err(format!("{} 코드 읽기 불가", name)); }
            let b = core::slice::from_raw_parts(f as *const u8, want.len());
            if b != want { return Err(format!("가드③ 실패: {} 진입 {:02x?} ≠ 기대 (RVA stale?)", name, b)); }
            Ok(f)
        };
        let f_clone = chk("clone", RVA_CLONE, &PUSH8)?;
        let f_drop_plan = chk("drop_plan", RVA_DROP_PLAN, &PROL_DROP_PLAN)?;
        let f_drop_map = chk("drop_map", RVA_DROP_MAP, &PROL_DROP_MAP)?;
        let f_insert = chk("insert", RVA_INSERT, &PROL_INSERT)?;
        let f_sanitize = chk("sanitize", RVA_SANITIZE, &PUSH8)?;
        let f_response = chk("response", RVA_RESPONSE, &PROL_RESPONSE)?;
        let f_send = chk("send", RVA_SEND, &PROL_SEND)?;
        let f_drop_res = base + RVA_DROP_RES;
        if !readable(f_drop_res, 12) || !matches!(*(f_drop_res as *const u8), 0x55 | 0x41 | 0x53 | 0x56 | 0x57 | 0x48 | 0x40) { return Err("가드③ 실패: drop_res".into()); }
        // ── 노드 탐색(센티널 → prev 순회)
        let map = db + OFF_TRAINING_PLANS;
        let sent = rd_u64(map + OFF_MAP_SENTINEL).ok_or("센티널 읽기 불가")? as usize;
        if !readable(sent, 0x210) { return Err("센티널 노드 읽기 불가".into()); }
        let mut n = rd_u64(sent + NODE_PREV).ok_or("센티널 prev 읽기 불가")? as usize;
        let mut plan: usize = 0;
        let mut hops = 0;
        while n != sent && hops < 256 {
            if !readable(n, 0x210) { return Err(format!("노드 0x{:x} 읽기 불가", n)); }
            if rd_u64(n + NODE_KEY) == Some(team_id as u64) { plan = n; break; }
            n = rd_u64(n + NODE_PREV).ok_or("노드 prev 읽기 불가")? as usize;
            hops += 1;
        }
        if plan == 0 { return Err(format!("team {} 의 훈련 계획 노드 없음(순회 {})", team_id, hops)); }
        let items_before = rd_u64(plan + OFF_CHAMP_MAP + 0x18).unwrap_or(u64::MAX);
        let delegated_before = *((plan + OFF_DELEGATED) as *const u8);
        // 기존 항목 + 덮어쓰기 병합(clear 는 제외)
        let mut targets: Vec<(u64, String)> = read_map_entries(plan + OFF_CHAMP_MAP).into_iter().filter(|(k, _)| !clear.contains(k) && !set.iter().any(|(a, _)| a == k)).collect();
        let kept = targets.len();
        for (a, v) in set { if !clear.contains(a) { targets.push((*a, v.clone())); } }

        type CloneFn = unsafe extern "C" fn(*mut u8, usize) -> *mut u8;
        type DropFn = unsafe extern "C" fn(*mut u8);
        type InsertFn = unsafe extern "C" fn(*mut u64, *mut u8, u64, *const u64) -> *mut u64;
        type SanitizeFn = unsafe extern "C" fn(usize, *mut u8);
        type ResponseFn = unsafe extern "C" fn(*mut u8, *const u8) -> *mut u8;
        type SendFn = unsafe extern "C" fn(*mut u8, usize, *const u8) -> *mut u8;
        let clone: CloneFn = core::mem::transmute(f_clone);
        let drop_plan: DropFn = core::mem::transmute(f_drop_plan);
        let drop_map: DropFn = core::mem::transmute(f_drop_map);
        let insert: InsertFn = core::mem::transmute(f_insert);
        let sanitize: SanitizeFn = core::mem::transmute(f_sanitize);
        let response: ResponseFn = core::mem::transmute(f_response);
        let send: SendFn = core::mem::transmute(f_send);
        let drop_res: DropFn = core::mem::transmute(f_drop_res);

        // ── tmp = clone(plan); 챔피언 맵 비우고 재구성
        let mut tmp = Plan([0u8; PLAN_SIZE]);
        clone(tmp.0.as_mut_ptr(), plan);
        let tmap = tmp.0.as_mut_ptr().add(OFF_CHAMP_MAP);
        drop_map(tmap);
        // 빈 hashbrown 테이블 템플릿(ctrl → 정적 EMPTY, mask/growth/items 0, k0/k1 = 기존 값 유지)
        core::ptr::write_unaligned(tmap as *mut u64, EMPTY_CTRL.0.as_ptr() as u64);
        core::ptr::write_unaligned(tmap.add(8) as *mut u64, 0);
        core::ptr::write_unaligned(tmap.add(0x10) as *mut u64, 0);
        core::ptr::write_unaligned(tmap.add(0x18) as *mut u64, 0);
        let heap = GetProcessHeap();
        let mut inserted = 0usize;
        for (aid, s) in &targets {
            if s.is_empty() { continue; }
            let p = HeapAlloc(heap, 0, s.len().max(1));
            if p == 0 { return Err("HeapAlloc 실패".into()); }
            core::ptr::copy_nonoverlapping(s.as_ptr(), p as *mut u8, s.len());
            let gs: [u64; 3] = [s.len() as u64, p as u64, s.len() as u64]; // 게임 String {cap, ptr, len}
            let mut old: [u64; 3] = [0; 3];
            insert(old.as_mut_ptr(), tmap, *aid, gs.as_ptr());
            if old[0] != 0 && old[0] < (1u64 << 63) { HeapFree(heap, 0, old[1] as usize); } // Some(old String) — None 은 니치(cap 최상위 비트)
            inserted += 1;
        }
        sanitize(db, tmp.0.as_mut_ptr());
        *tmp.0.as_mut_ptr().add(OFF_DELEGATED) = 0;
        let items_tmp = core::ptr::read_unaligned(tmap.add(0x18) as *const u64);
        // ── 교체(원본 drop 후 memcpy · tmp 는 move)
        drop_plan(plan as *mut u8);
        core::ptr::copy_nonoverlapping(tmp.0.as_ptr(), plan as *mut u8, PLAN_SIZE);
        // ── 응답 유니캐스트
        let mut resp = Plan([0u8; PLAN_SIZE]);
        clone(resp.0.as_mut_ptr(), plan);
        let mut pkt = Buf([0u8; 0x770]);
        response(pkt.0.as_mut_ptr(), resp.0.as_ptr()); // resp move
        let tag = core::ptr::read_unaligned(pkt.0.as_ptr() as *const u64);
        if tag != TAG_RESPONSE_TRAINING_PLAN { return Err(format!("서버 DB 는 교체됨(items {}→{}) 그러나 응답 tag 0x{:x} 예상 밖 — 클라 반영은 다음 진행 시", items_before, items_tmp, tag)); }
        let mut msg = Buf([0u8; 0x770]);
        core::ptr::write_unaligned(msg.0.as_mut_ptr() as *mut u64, UNICAST);
        core::ptr::copy_nonoverlapping(pkt.0.as_ptr(), msg.0.as_mut_ptr().add(8), PKT_SIZE);
        core::ptr::copy_nonoverlapping((lp + OFF_ADDR_IN_LOOP) as *const u8, msg.0.as_mut_ptr().add(8 + PKT_SIZE), ADDR_SIZE);
        let mut res = Buf([0u8; 0x770]);
        send(res.0.as_mut_ptr(), sender, msg.0.as_ptr());
        let r = core::ptr::read_unaligned(res.0.as_ptr() as *const u64);
        if r != u64::MAX { drop_res(res.0.as_mut_ptr()); return Err(format!("서버 DB 는 교체됨(items {}→{}) 그러나 send 실패(res 0x{:x})", items_before, items_tmp, r)); }
        Ok(format!("team {} 훈련 챔피언 {}명 기록(유지 {} · 삭제 {} · items {}→{} · delegated {}→0 · 유니캐스트 OK)", team_id, inserted, kept, clear.len(), items_before, items_tmp, delegated_before))
    }
}
