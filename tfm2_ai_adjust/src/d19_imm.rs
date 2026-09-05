// d19_imm.rs — disc19(DefenseNexus) 판단상수 severity 바이트패치(노브 계층). ★2026-09-06 disc19_repro.rs 에서 분리.
//   disc19_repro.rs(3,945줄 · my_disc19/18·my_disc13/15/16/17·dcmp 검증하네스)는 0.4.13~0.5.3 구조를 깐 낡은 재현물이라
//   제거했다(착수서 §5). 그중 **실제 게임에 반영되는 것은 이 함수 하나**(d19i_enable=1 → imm8 패치 10사이트)였다.
//   제거 전 RE 사실 = REPORT\tfm2_ai_adjust\RE\2026-09-06_낡은재현모듈3종_제거전_사실추출.md, 원본 = MODS\_archive\ai_adjust_legacy_repro_20260906\.
// 들어있는 것: apply_disc19_imm (로드시점 1회 호출 — tfm2_ai_adjust.rs 훅 설치 블록 끝).
// 의존: D19IMM_DONE·D19_RETREAT_HP(tfm2_ai_adjust.rs static), tune(), patch_imm_bytes(), exe_base(), pth().
// 언제 손대나: 패치로 defense_nexus severity 래더 사이트가 이동했을 때(MIG 매니페스트 tfm2_ai_adjust.json 의 d19 항목과 동기).


// ★[07-15] 로드시점 1회 호출(install_wrap과 동일 타이밍 = sim 실행 전 = .text 패치 안전).
//   게임플레이중(백그라운드 sim이 disc19 동시실행) 패치는 쓰기 AV폴트(exe+0x1c83e6a) → post_update 호출 금지.
// ✅★★[0.5.2 재핀 완료 = 아래 10사이트 전부 0.5.2 주소 (ghidra-re 2026-07-23, 3자 exe 바이트대조 확정)] ★★
//   ~~구 15사이트(0.5.1 주소·applied=0/15 무개입)~~ → **10사이트로 확정**. 사이트가 5개 줄어든 건 미탐색이 아니라
//   **0.5.2에서 게임이 phase 게이트를 통째로 제거**했기 때문 ⟹ 10/10 = 그 버전 기준 **전량 반영**(부분반영 아님).
//   ⚠0.5.2 구조변경 3건:
//     ① **컨테이너 = disc19 본체 `0x2380820` 단일**(구 주석의 "두 함수로 분리·0x22f8a90"은 **오답**.
//        0x22f8a90은 자체 severity 사본을 가진 **다른 핸들러**라 거기 패치하면 완전 오패치. tr49=0x22f8d6e 등 4개 리드 폐기).
//        지문 = severity 인라인 사본이 exe에 6개 있는데 **ally 0x32 ×2 + rhB 0x2e 트레일러를 가진 건 0x2380e16 하나뿐**.
//     ② **레지스터 변경**: hp 비교 R15(`49 83 ff`) → **RSI(`48 83 fe`)** — hp66/hp41/hp26/rhB 4곳 prefix 교체함.
//     ③ **phase 게이트 4개 → 0개 (전부 삭제)**: pt(ph>=30, 구 0x1e0e2d7) · pa#1/2/3(ph>=39, 구 0x1e0e498/532/5c2) + 모드가
//        안 건드리던 4번째(구 0x1e0e1ea)까지 소멸. 0.5.1↔0.5.2 명령 시퀀스 대조로 "제거" 확증(추정 아님).
//        ⟹ ★**cfg `d19_phase_threat`·`d19_phase_ally` = ⛔DEAD(영구 무효)** — 설정편집기 표기 필요.
//        ⟹ ★부수효과: 0.5.2 disc19는 **매치 phase와 무관하게 threat/ally 판정을 상시 수행**한다.
//     ④ **rhA(hp>45 #1, 구 0x1e0e4b4)도 소멸** → `d19_retreat_hp`는 **rhB 한 곳(`rh+1` 인코딩)** 으로 완결.
//   ⛔재조사 금지: `0x2380c28`/`0x2380c40`의 `cmp ?,0x1d`는 phase가 아니라 **맵 그리드 타일 clamp**(0.5.1에도 동일 위치 존재).
//     여기에 pt를 배선하면 오패치. 0.5.2 disc19 전 구간(0x2380820~0x2382cb0)에 `cmp ?,0x2d`·`cmp ?,0x26`은 **0개**.
unsafe fn apply_disc19_imm() {
    if D19IMM_DONE.swap(true, Ordering::Relaxed) { return; }   // 1회만
    let enable = tune("d19i_enable", 0) != 0;
    // 위협비율표(threat를 curHP%로 본 tr 문턱): 이 값보다 크면 후퇴검토. HP경계별로 낮아짐.
    let sr0 = tune("d19_sev_ratio_0", 49);   // HP무관 기본 문턱
    let sr1 = tune("d19_sev_ratio_1", 29);   // HP<hp1
    let sr2 = tune("d19_sev_ratio_2", 17);   // HP<hp2
    let sr3 = tune("d19_sev_ratio_3",  9);   // HP<hp3
    let sh1 = tune("d19_sev_hp_1", 66);      // 1단계 HP% 경계
    let sh2 = tune("d19_sev_hp_2", 41);      // 2단계
    let sh3 = tune("d19_sev_hp_3", 26);      // 3단계
    let ah  = tune("d19_ally_hp", 50);       // ally넥서스 위기 HP%(이 위면 지원 안 감)
    let rh  = D19_RETREAT_HP.load(Ordering::Relaxed);    // ★[수정 07-16] 명시 파서arm(:1761)이 채우는 static 직독. 구 tune("d19_retreat_hp")는 키충돌로 항상 45 고정=배선버그였음
    let pt  = tune("d19_phase_threat", 30);  // ⛔DEAD(0.5.2: 대응 게이트 삭제) — 로그 표시용으로만 읽음
    let pa  = tune("d19_phase_ally", 39);    // ⛔DEAD(0.5.2: 대응 게이트 3곳 전부 삭제) — 로그 표시용
    let base = exe_base();
    if base == 0 { D19IMM_DONE.store(false, Ordering::Relaxed); return; }   // base 미준비=재시도 허용
    let b1 = |v: i64| (v.max(0).min(0x7f)) as u64;   // imm8 sign-safe clamp
    // enable=cfg(경계변환 적용) / disable=게임 원본 imm 복원
    let (p_sr0,p_sr1,p_sr2,p_sr3,p_sh1,p_sh2,p_sh3,p_ah,p_rhb) = if enable {
        (b1(sr0), b1(sr1), b1(sr2), b1(sr3+1), b1(sh1-1), b1(sh2-1), b1(sh3-1), b1(ah), b1(rh))
    } else {
        (0x31, 0x1d, 0x11, 0x0a, 0x41, 0x28, 0x19, 0x32, 0x2d)   // ★0.5.4: retreat 원본 0x2e→**0x2d**
    };
    let _ = (pt, pa);   // ⛔0.5.2 대응 게이트 소멸 = 패치 사이트 없음(로그 표시용으로만 유지)
    let mut ok = 0u32;
    // ★[0.5.7 재핀 2026-09-01] disc19 severity 래더 = defense_nexus(FUN_140eae620) 내부 **0xeaf239~0xeaf30e**(sp19 P2 RE + capstone 확정).
    //   ~~0.5.6 0xd432xx (stale·0.5.7 guard-dead=no-op)~~ → 0.5.7 재핀. 값·극성 sp19 P2와 비트일치.
    //   prefix 2종: **Q/넥서스2 = 48 83 f8(rax)** / **HP%/retreat = 48 83 fb(rbx)** (0.5.6 hp/retreat RDI 48 83 ff → 0.5.7 RBX 48 83 fb).
    //   ⚠넥서스2(ally) 문턱 = 32/64bit div **2사이트 중복**(0xeaf2d3 JA=64bit + 0xeaf2f8 JBE=32bit) — 둘 다 패치해야 일관.
    //   전 사이트 width=1·imm_off=3. sr3 +1 인코딩(9→0x0a) 유지, hp경계 V−1(66→0x41) 유지, retreat orig 0x2d(JBE, HP%<=45 후퇴) — 0.5.7은 +1/−1 조정 없음.
    // 위협비율표 (RAX=tr, 48 83 f8)
    ok += patch_imm_bytes(base + 0xe93517, &[0x48,0x83,0xf8], 3, 1, p_sr0) as u32;   // tr>49  orig 0x31 (JA)   // ←s6 d432eb/dacc9b
    ok += patch_imm_bytes(base + 0xe93523, &[0x48,0x83,0xf8], 3, 1, p_sr1) as u32;   // tr>29  orig 0x1d   // ←s6 d432f7
    ok += patch_imm_bytes(base + 0xe9352f, &[0x48,0x83,0xf8], 3, 1, p_sr2) as u32;   // tr>17  orig 0x11   // ←s6 d43303
    ok += patch_imm_bytes(base + 0xe9353b, &[0x48,0x83,0xf8], 3, 1, p_sr3) as u32;   // tr<=9  orig 0x0a (JAE, +1 인코딩)   // ←s6 d4330f
    // HP단계 경계 (★RBX=hp_pct, 48 83 fb ← 0.5.6 RDI 48 83 ff) — V−1 인코딩
    ok += patch_imm_bytes(base + 0xe9351d, &[0x48,0x83,0xfb], 3, 1, p_sh1) as u32;   // hp>65(<66)  orig 0x41   // ←s6 d432f1
    ok += patch_imm_bytes(base + 0xe93529, &[0x48,0x83,0xfb], 3, 1, p_sh2) as u32;   // hp>40  orig 0x28   // ←s6 d432fd
    ok += patch_imm_bytes(base + 0xe93535, &[0x48,0x83,0xfb], 3, 1, p_sh3) as u32;   // hp>25  orig 0x19   // ←s6 d43309
    // ally넥서스 위기 HP% (RAX 48 83 f8) — 32/64bit div 2사이트 중복(둘 다 동일값)
    ok += patch_imm_bytes(base + 0xe9358f, &[0x48,0x83,0xf8], 3, 1, p_ah) as u32;    // ally>50 #1(64bit div, JA) orig 0x32   // ←s6 d43362
    ok += patch_imm_bytes(base + 0xe935bb, &[0x48,0x83,0xf8], 3, 1, p_ah) as u32;    // ally>50 #2(32bit div, JBE) orig 0x32   // ←s6 d4338e
    // retreat_hp (★RBX 48 83 fb) — orig 0x2d(JBE, HP%<=45 후퇴)
    ok += patch_imm_bytes(base + 0xe935d1, &[0x48,0x83,0xfb], 3, 1, p_rhb) as u32;   // retreat  orig 0x2d   // ←s6 d433a4/dacd54
    //   ⟹ **기대값·기록값이 −1**(0x2e→0x2d). 값만 옮기면 임계가 1 어긋난다.
    // ⛔phase 진입 게이트 4곳(pt·pa#1/2/3) = 0.5.2에서 게임이 전부 삭제 → 패치 사이트 없음(상단 주석 ③).
    // ★LOG_ON 무관 직접 write(설치확증 — itemnet_guard와 동일). write_named은 LOG_ON 게이트라 미확인됐었음.
    if let Some(p) = pth("d19_imm.txt") {
        let _ = fs::write(p, format!("d19i_enable={} applied={}/10 sev=[{} {} {} {}] hp=[{} {} {}] ally={} retreat={} phase=[{} {}]=DEAD @base{:#x}\n",
            enable, ok, sr0, sr1, sr2, sr3, sh1, sh2, sh3, ah, rh, pt, pa, base));
    }
}

