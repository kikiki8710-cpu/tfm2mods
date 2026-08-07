// nexus_emg.rs — "넥서스 비상" 발동 조건을 유저가 조절하게 여는 노브 묶음. (2026-08-08 신설)
//
// ★무엇을 여는가
//   0.5.4 가 새로 넣은 술어 B(`0xce3be0`)는 "**쌍둥이 타워 2기가 모두 파괴됐고** + **넥서스가 실제로
//   맞는 중**"이면 참이 된다. 참이 되면 넥서스 수비 후보의 −9,999,999 하드리젝트가 면제되어
//   **맵 어디에 있든 수비하러 온다**(+ 경매 강제귀환이 취소되어 도망가지 않는다).
//   즉 이게 "쌍둥이 포탑이 사라지면 적극적으로 넥서스를 지킨다"의 실체다.
//
// ★"쌍둥이 타워"인 것은 **확정**이다(추정 아님) — AI 월드뷰 빌더 `0x14e2c50` 이 `entity+0x128`
//   (TowerType, 8변형)로 점프테이블 분기하는데 **TwinA(3)·TwinB(4)만 이 Vec 에 push** 되고
//   레인 6종은 각자 고정 슬롯(`0x180`Top `0x190`Top2 `0x1a0`Mid `0x1b0`Mid2 `0x1c0`Bot `0x1d0`Bot2)
//   에 저장된다. 팀당 쌍둥이 = 2기(`init_twin_tower 0x13ae180` 이 등록 4회 = 2기×2팀).
//   ⟹ **레인 타워가 남아 있어도 쌍둥이만 다 부서지면 발동한다.**
//   근거 = REPORT\tfm2_ai_adjust\RE\2026-08-08_쌍둥이타워-Vec정체-확정.md
//
//   원본은 "쌍둥이 타워가 **하나도 안 남았을 때**"만 발동한다. 이 파일은 그 문턱을 두 축으로 연다:
//     ① `nxe_twin_max` — 쌍둥이 타워가 **N기 이하** 남았으면 발동 (0=원본 / 1=하나 남아도 / 2=둘 다 있어도)
//     ② `nxe_t2_lost`    — **어느 한 라인이라도 2차 타워가 파괴**됐으면 쌍둥이가 멀쩡해도 발동
//   두 축은 OR 이다. ②는 게임에 없던 조건이라 우리가 직접 판정한다.
//
// ★개입 지점 (RE\2026-08-08_넥서스수비-술어B-단계노브-개입점확정.md)
//   ```
//   0xce3c18  48 83 BC 03 48 01 00 00 00   cmp qword [rbx+rax+0x148], 0   ; rbx=reg, rax=side*0x20
//   0xce3c21  74 13                        je  0xce3c36                   ; 통과 → 조건② 검사로
//   0xce3c23  31 c0                        xor eax,eax                    ; 실패 → 0 반환
//   ```
//   · ①만 쓸 때 = **2바이트 imm 패치**(값 `00`→N, 분기 `74` je→`76` jbe). 회귀 위험 0.
//     ⚠반드시 세트다. 값만 바꾸면 "정확히 N개일 때만"이 되어 엉뚱하게 동작한다.
//   · ②도 쓸 때 = **11바이트 마이크로 디투어**(창 = cmp 9B + je 2B). 우리가 직접 판정해 분기시킨다.
//   두 경로는 **상호배타** — 같은 바이트를 다투면 나중 것이 이겨 조용히 무효가 된다(08-07 알리아스 사고).
//
// ★안전 근거 (전부 RE 관측)
//   · 창 안으로 뛰어드는 분기 **0건**(.text 전역 rel8/rel32/jcc + 점프테이블 스캔).
//   · 복귀 지점 둘 다 플래그·RAX 를 소비하지 않는다 ⟹ **플래그 보존 불요**.
//   · 사이트 시점 `rbx`=reg · `rcx`=side · `rax`=side*0x20 생존. `rax`·`r8~r11` 은 dead.
//   · 1·2차 타워는 파괴되면 **슬롯이 null** 이 된다(수집 함수 `0x14e87d0` 이 슬롯마다 null 분기).
//     ⟹ HP 를 볼 필요 없이 `== 0` 하나로 판정.
//
// ⚠부작용 (RE §5 — 술어 B 를 자주 참으로 만들면 같이 따라오는 것)
//   · `0xd8e0b2` : B 가 참이면 어떤 액션을 −99,999 로 **억제**한다(방향은 수비 적극화와 같지만 강하다).
//   · `0xda365d` : B 를 교전 판단 입력으로 저장한다(효과 방향 **미확정**).
//   둘 다 중화 노브를 뒀다(`nxe_supp_off` · `nxe_battle_off`). 체감이 이상하면 켜서 분리해 볼 것.

// 술어 B 조건① 사이트 (0.5.4)
const NXE_RVA: usize = 0xce3c18;
/// 창 11B = `cmp qword [rbx+rax+0x148], 0` (9B) + `je +0x13` (2B).
const NXE_WIN: [u8; 11] = [0x48, 0x83, 0xBC, 0x03, 0x48, 0x01, 0x00, 0x00, 0x00, 0x74, 0x13];
const NXE_IMM_OFF: usize = 8;    // 창 안에서 imm8(=0) 위치  → 0xce3c20
const NXE_JCC_OFF: usize = 9;    // 창 안에서 je(0x74) 위치   → 0xce3c21
const NXE_FAIL_RVA: usize = 0xce3c23;   // 조건① 실패 → 0 반환
const NXE_PASS_RVA: usize = 0xce3c36;   // 조건① 통과 → 조건② 검사로

// 디투어가 매번 읽는 값(콜백 없이 스텁이 직접 메모리를 읽는다 — 호출이 없어 비용 거의 0).
static NXE_MAX: AtomicI64 = AtomicI64::new(0);      // 남은 쌍둥이 타워 허용 개수(팀당 2기)
static NXE_T2: AtomicI64 = AtomicI64::new(0);       // 0=off, 1=어느 라인이든 2차 파괴 시 발동
static NXE_DETOUR_ON: AtomicBool = AtomicBool::new(false);
static NXE_SIG: AtomicU64 = AtomicU64::new(u64::MAX);

/// 넥서스 비상 조건 노브 적용. apply 체인에서 부른다.
pub(crate) unsafe fn apply_nxe() {
    let smax = tune("nxe_twin_max", -1);   // 쌍둥이 N기 이하 남았으면 발동(-1=원본=0기 / 0~2)
    let t2 = tune("nxe_t2_lost", -1);        // 1 = 어느 라인이든 2차 타워 파괴 시 발동(-1/0=원본)
    let supp = tune("nxe_supp_off", -1);     // 1 = 비상일 때 다른 액션을 −99,999 로 죽이는 조항 해제
    let batt = tune("nxe_battle_off", -1);   // 1 = 교전 판단이 이 플래그를 무시

    let mut sig = 0u64;
    for v in [smax, t2, supp, batt] { sig = sig.wrapping_mul(0x100000001b3) ^ (v as u64); }
    if sig == NXE_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }

    // 값은 디투어가 매 판정마다 읽으므로, 설치돼 있으면 여기 갱신만으로 즉시 반영된다.
    NXE_MAX.store(if smax < 0 { 0 } else { smax.min(127) }, Ordering::Relaxed);
    NXE_T2.store(if t2 > 0 { 1 } else { 0 }, Ordering::Relaxed);

    let want_detour = t2 > 0;                       // ②를 쓰면 디투어가 필요하다
    let mut rep = String::from("=== 넥서스 비상 발동 조건 ===\n\
        # 원본 = \"쌍둥이 타워가 하나도 안 남았을 때\"만 발동.\n\
        # nxe_twin_max = 쌍둥이 N기 이하 남았으면 발동 (0=원본 / 1=하나 남아도 / 2=둘 다 있어도)\n\
        # nxe_t2_lost    = 1이면 어느 라인이든 2차 타워가 깨졌을 때도 발동(구조물 상태와 무관)\n");

    let addr = base + NXE_RVA;
    let installed = NXE_DETOUR_ON.load(Ordering::Relaxed);

    if want_detour && !installed {
        match install_nxe_detour(base) {
            Ok(stub) => {
                NXE_DETOUR_ON.store(true, Ordering::Relaxed);
                rep.push_str(&format!("경로: 마이크로 디투어(2차 타워 조건 포함)  스텁={:#x}\n", stub));
            }
            Err(e) => rep.push_str(&format!("★디투어 설치 실패: {} — 2차 타워 조건은 적용되지 않는다\n", e)),
        }
    }

    if NXE_DETOUR_ON.load(Ordering::Relaxed) {
        // 디투어가 그 자리를 가졌으면 imm 패치는 **절대** 하지 않는다(E9 를 덮으면 엉뚱한 주소로 점프).
        rep.push_str(&format!("경로: 마이크로 디투어  ·  쌍둥이 {}기 이하 / 2차타워 조건 {}\n",
            NXE_MAX.load(Ordering::Relaxed),
            if NXE_T2.load(Ordering::Relaxed) != 0 { "켬" } else { "끔" }));
    } else if smax >= 0 {
        // ①만 쓰는 경로 — 2바이트 imm 패치. **값 먼저, 분기 나중**(중간 상태가 안전한 쪽).
        let n = smax.min(127) as u8;
        let ok_v = patch_raw_bytes(addr + NXE_IMM_OFF, &[n]);
        let ok_j = patch_raw_bytes(addr + NXE_JCC_OFF, &[0x76]);   // je → jbe (부호없는 <=)
        rep.push_str(&format!("경로: 2바이트 패치  ·  쌍둥이 {}기 이하  (값 {} / 분기 {})\n",
            n, if ok_v { "OK" } else { "실패" }, if ok_j { "OK" } else { "실패" }));
    } else {
        rep.push_str("경로: 무개입(원본 그대로 — 쌍둥이가 하나도 안 남아야 발동)\n");
    }

    // ── 부작용 중화(각 1사이트, 순수 imm) ──
    let mut ok = 0u32; let mut tot = 0u32;
    if supp > 0 {
        tot += 1;
        // `test eax,0x100` → `test eax,0` : 비상일 때 다른 액션을 −99,999 로 죽이던 조항이 사라진다.
        ok += patch_imm_bytes(base + 0xd8e0b2, &[0xa9], 1, 4, 0) as u32;
    }
    if batt > 0 {
        tot += 1;
        // `and eax,1` → `and eax,0` : 교전 판단이 이 플래그를 항상 0 으로 본다.
        ok += patch_imm_bytes(base + 0xda3660, &[0x83, 0xe0], 2, 1, 0) as u32;
    }
    if tot > 0 {
        rep.push_str(&format!("부작용 중화: {}/{} (억제조항 해제={} · 교전판단 무시={})\n",
            ok, tot, supp > 0, batt > 0));
    }

    NXE_SIG.store(sig, Ordering::Relaxed);
    if let Some(p) = pth("nxe.txt") { let _ = fs::write(p, rep); }
}

/// 창 전수 대조 후 raw 바이트 쓰기. `patch_imm_bytes` 는 prefix+imm 규약이라 여기선 못 쓴다
/// (우리는 **분기 opcode** 도 바꾸기 때문). 대신 창 11B 를 통째로 확인해 안전을 보장한다.
unsafe fn patch_raw_bytes(addr: usize, bytes: &[u8]) -> bool {
    if !readable(addr, bytes.len()) { return false; }
    let mut old: u32 = 0;
    if VirtualProtect(addr, bytes.len(), 0x40, &mut old) == 0 { return false; }
    core::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, bytes.len());
    VirtualProtect(addr, bytes.len(), old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, bytes.len());
    true
}

/// 조건①+② 를 우리가 직접 판정하는 스텁을 깔고, 사이트를 그리로 보낸다.
///
/// 스텁은 **함수 호출을 하지 않는다** — `NXE_MAX`/`NXE_T2` 를 메모리에서 직접 읽고 비교만 한다.
/// 그래서 shadow space·정렬·xmm 보존이 전부 불필요하고, 비용이 원본 `cmp` 몇 개 수준이다.
unsafe fn install_nxe_detour(base: usize) -> Result<usize, &'static str> {
    let addr = base + NXE_RVA;
    if !readable(addr, NXE_WIN.len()) { return Err("사이트 읽기 불가"); }
    // ★원본 전수 대조 — 한 바이트라도 다르면 설치하지 않는다(게임 패치 방어).
    //   ⚠단 imm8 자리는 건너뛴다. ①만 쓰던 상태에서 ②를 켜면 그 자리에 이미 N 이 들어가 있다.
    for (i, &b) in NXE_WIN.iter().enumerate() {
        if i == NXE_IMM_OFF { continue; }
        // 분기 자리는 je(원본) 또는 jbe(①패치 뒤) 둘 다 허용.
        if i == NXE_JCC_OFF {
            let cur = rd_u8(addr + i);
            if cur != 0x74 && cur != 0x76 { return Err("분기 opcode 불일치"); }
            continue;
        }
        if rd_u8(addr + i) != b { return Err("명령 골격 불일치 — 게임이 바뀌었다"); }
    }

    let mut s: Vec<u8> = Vec::with_capacity(128);
    // ★분기 rel8 은 **자리를 기록해 두고 나중에 채운다.** 손으로 오프셋을 세면 반드시 틀린다
    //   (한 바이트만 어긋나도 명령 중간으로 뛰어 즉사한다).
    let mut fix_take: Vec<usize> = Vec::new();   // `.take` 로 가는 jcc 의 rel8 바이트 위치
    let mut fix_skip: Vec<usize> = Vec::new();   // `.skip` 으로 가는 jcc

    // 게임 레지스터 보존: 우리가 건드리는 것은 rdx·r8 뿐(rax 는 사이트 직후 재정의되는 dead 값이지만
    // 굳이 아끼지 않고 그대로 둔다). 플래그는 복귀 지점이 소비하지 않으므로 보존 불요.
    s.push(0x52);                                              // push rdx
    s.extend_from_slice(&[0x41, 0x50]);                        // push r8
    // ── 조건① : 살아있는 쌍둥이 타워 수 <= NXE_MAX ──
    s.extend_from_slice(&[0x48, 0x8b, 0x94, 0x03]);            // mov rdx,[rbx+rax+0x148]
    s.extend_from_slice(&0x148u32.to_le_bytes());
    s.extend_from_slice(&[0x49, 0xb8]);                        // movabs r8, &NXE_MAX
    s.extend_from_slice(&(core::ptr::addr_of!(NXE_MAX) as usize).to_le_bytes());
    s.extend_from_slice(&[0x49, 0x3b, 0x10]);                  // cmp rdx,[r8]
    s.extend_from_slice(&[0x76, 0x00]); fix_take.push(s.len() - 1);   // jbe .take
    // ── 조건② : 어느 한 라인이라도 2차 타워가 파괴(슬롯 null) ──
    s.extend_from_slice(&[0x49, 0xb8]);                        // movabs r8, &NXE_T2
    s.extend_from_slice(&(core::ptr::addr_of!(NXE_T2) as usize).to_le_bytes());
    s.extend_from_slice(&[0x49, 0x83, 0x38, 0x00]);            // cmp qword [r8],0
    s.extend_from_slice(&[0x74, 0x00]); fix_skip.push(s.len() - 1);   // je .skip  (조건② 꺼짐)
    for disp in [0x190u32, 0x1b0, 0x1d0] {                     // lane0/1/2 의 2차 타워
        s.extend_from_slice(&[0x48, 0x83, 0xbc, 0xcb]);        // cmp qword [rbx+rcx*8+disp],0
        s.extend_from_slice(&disp.to_le_bytes());
        s.push(0x00);                                          //   imm8 = 0
        s.extend_from_slice(&[0x74, 0x00]); fix_take.push(s.len() - 1);   // je .take (파괴됨)
    }
    // .skip : 원본의 "조건① 실패" 경로로
    let skip_off = s.len();
    s.extend_from_slice(&[0x41, 0x58]);                        // pop r8
    s.push(0x5a);                                              // pop rdx
    s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]); // jmp [rip+0]
    s.extend_from_slice(&(base + NXE_FAIL_RVA).to_le_bytes());
    // .take : 원본의 "조건① 통과" 경로로
    let take_off = s.len();
    s.extend_from_slice(&[0x41, 0x58]);                        // pop r8
    s.push(0x5a);                                              // pop rdx
    s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]); // jmp [rip+0]
    s.extend_from_slice(&(base + NXE_PASS_RVA).to_le_bytes());

    // ── rel8 채우기 (rel8 기준점 = 그 바이트 **다음** 주소) ──
    for (list, target) in [(&fix_take, take_off), (&fix_skip, skip_off)] {
        for &pos in list.iter() {
            let d = target as i64 - (pos as i64 + 1);
            if !(0..=127).contains(&d) { return Err("분기 rel8 범위 초과"); }
            s[pos] = d as u8;
        }
    }

    let stub = micro_alloc(addr, s.len());
    if stub == 0 { return Err("스텁 할당 실패(±2GB 내 여유 없음)"); }
    let rel = stub as i64 - (addr as i64 + 5);
    if !(-0x7fff_0000..=0x7fff_0000).contains(&rel) { return Err("스텁이 rel32 범위 밖"); }
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());

    // ★앞 5바이트만 쓴다 — 뒤 6B 는 E9 가 자리잡는 순간 도달 불가가 되므로 덮을 필요가 없고,
    //   두 번 쓰면 그 사이에 낀 스레드가 반쯤 덮인 명령을 실행할 수 있다(class_micro 와 같은 규칙).
    let mut e9 = [0x90u8; 5];
    e9[0] = 0xe9;
    e9[1..5].copy_from_slice(&(rel as i32).to_le_bytes());
    let mut old: u32 = 0;
    if VirtualProtect(addr, NXE_WIN.len(), 0x40, &mut old) == 0 { return Err("VirtualProtect 실패"); }
    core::ptr::copy_nonoverlapping(e9.as_ptr(), addr as *mut u8, 5);
    VirtualProtect(addr, NXE_WIN.len(), old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, NXE_WIN.len());
    Ok(stub)
}
