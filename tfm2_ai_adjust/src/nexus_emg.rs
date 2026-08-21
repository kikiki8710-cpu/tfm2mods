// nexus_emg.rs — "넥서스 비상 수비"를 **상황별 적극도**로 여는 노브 묶음. (2026-08-08 신설)
//
// ★게임의 원래 동작 (0.5.4 신규 술어 B `0xce3be0`)
//   "**쌍둥이 타워 2기가 모두 파괴됐고** + **넥서스가 실제로 맞는 중**" 이면 '비상'으로 본다.
//   비상이면 넥서스 수비 후보의 −9,999,999 하드리젝트가 면제되어 **맵 어디에 있든 수비하러 오고**,
//   경매 강제귀환(죽을 것 같으면 빠지는 동작)이 취소되어 도망가지 않는다.
//   즉 이게 "쌍둥이 포탑이 사라지면 적극적으로 넥서스를 지킨다"의 실체다.
//
// ★"쌍둥이 타워"인 것은 **확정**이다(추정 아님) — AI 월드뷰 빌더 `0x14e2c50` 이 `entity+0x128`
//   (TowerType, 8변형)로 점프테이블 분기하는데 **TwinA(3)·TwinB(4)만 이 Vec 에 push** 되고
//   레인 6종은 각자 고정 슬롯에 저장된다. 팀당 쌍둥이 = 2기.
//   ⟹ **레인 타워가 남아 있어도 쌍둥이만 다 부서지면 발동한다.**
//   근거 = REPORT\tfm2_ai_adjust\RE\2026-08-08_쌍둥이타워-Vec정체-확정.md
//
// ★이 파일이 여는 것 — 상황 8가지에 **각각 적극도**를 주고, 겹치면 **가장 높은 값**을 쓴다
//   ```
//   nxe_twin0      쌍둥이 0기 남음        (게임이 원래 발동하던 상황)
//   nxe_twin1      쌍둥이 1기 이하 남음
//   nxe_t2_1/2/3   2차 타워가 1/2/3개 이상 파괴됨
//   nxe_t1_1/2/3   1차 타워가 1/2/3개 이상 파괴됨
//   ```
//   각 값 = **적극도**. `0` = 그 상황은 비상 아님 / `1 이상` = 비상.
//   조건이 "이하 / 이상"이라 여러 개가 동시에 해당된다 — 예: 2차가 3개 부서지면 `t2_1`·`t2_2`·`t2_3`
//   이 전부 해당되고, 그중 **최댓값**을 쓴다(유저 지시 "적극도가 제일 높은 것 우선").
//
//   ⚠**지금 적극도는 "0인가 아닌가"까지만 반영된다.** 술어 B 자체가 참/거짓뿐이라 그 이상을 표현할
//   수 없다. 숫자 크기까지 살리려면 소비처(예: `0xe097b5` 의 거리 30,000 감산)를 함께 열어야 한다.
//   그때까지 1 과 100 은 동작이 같다. 다른 것은 **0 뿐**이다.
//
// ★개입 지점 (RE\2026-08-08_넥서스수비-술어B-단계노브-개입점확정.md)
//   ```
//   0xce3c18  48 83 BC 03 48 01 00 00 00   cmp qword [rbx+rax+0x148], 0   ; rbx=reg, rcx=side
//   0xce3c21  74 13                        je  0xce3c36                   ; 통과 → 조건② 검사로
//   0xce3c23  31 c0                        xor eax,eax                    ; 실패 → 0 반환
//   ```
//   창 11B 를 `E9 rel32` 로 우리 스텁에 보낸다. 스텁은 `reg`·`side` 를 넘겨 **Rust 콜백**이 판정하게 하고,
//   그 결과로 원본의 두 경로 중 하나로 되돌아간다(원본 흐름을 벗어나지 않는다).
//   ※조건②("넥서스가 실제로 맞는 중")는 **건드리지 않는다** — 그게 빠지면 아무 일 없을 때도 전원이
//     넥서스로 모여 라인이 빈다. 우리가 여는 것은 조건①(구조물 상태)뿐이다.
//
// ★안전 근거 (전부 RE 관측)
//   · 창 안으로 뛰어드는 분기 **0건**(.text 전역 rel8/rel32/jcc + 점프테이블 스캔).
//   · 복귀 지점 둘 다 플래그·RAX 를 소비하지 않는다 ⟹ 플래그 보존 불요.
//   · 사이트 시점 `rbx`=reg · `rcx`=side 생존.
//   · 1·2차 타워는 파괴되면 **슬롯이 null** 이 된다(수집 함수 `0x14e87d0` 이 슬롯마다 null 분기)
//     ⟹ HP 를 볼 필요 없이 `== 0` 하나로 판정.
//   · 이 술어는 **틱·엔티티 단위로 메모(캐시)** 되므로(`0xc797c0`) 콜백 호출 빈도가 낮다.
//
// ⚠부작용 (RE §5 — 비상을 자주 켜면 같이 따라오는 것)
//   · `0xe3f152` : 비상이면 어떤 액션을 −99,999 로 **억제**한다(방향은 같지만 강하다).
//   · `0xda365d` : 비상 플래그를 교전 판단 입력으로 저장한다(효과 방향 **미확정**).
//   둘 다 끄는 노브를 뒀다(`nxe_supp_off` · `nxe_battle_off`).

// 술어 B 조건① 사이트 (0.5.4)
const NXE_RVA: usize = 0xddbad8;   // ★0.5.6(was 0.5.5 0xddbad8 — 호스트 0xdd1cd0→0xddbaa0 UNIQUE·BYTE=SAME·delta+0x9dd0·win 11B 일치 재확증). 구 0.5.5(~~0xce3c18~~) 정렬 이력 유지
/// 창 11B = `cmp qword [rbx+rax+0x148], 0` (9B) + `je +0x13` (2B).
const NXE_WIN: [u8; 11] = [0x48, 0x83, 0xBC, 0x03, 0x48, 0x01, 0x00, 0x00, 0x00, 0x74, 0x13];
const NXE_FAIL_RVA: usize = 0xddbae3;   // 비상 아님 → 0 반환 (★0.5.5 ~~0xce3c23~~ = NXE_RVA+11 fallthru)
const NXE_PASS_RVA: usize = 0xddbaf6;   // 비상 → 조건②(넥서스가 맞는 중) 검사로 (★0.5.5 ~~0xce3c36~~ = je타깃 mov eax,1)

// 월드뷰 구조체 오프셋 (RE 2026-08-08 확정)
// ★0.5.5 확인: region 저대역 불변. 0x148은 NXE 함수 cmp 사이트서 직접 정렬확인(불변). O_T1/O_T2(0x180~0x1d0)는
//   같은 region struct 인접 저대역(≤0x258, §6 World 저대역 불변)이라 불변 확정(0x148 불변이 struct 미시프트 방증).
const O_TWIN_LEN: usize = 0x148;                  // + side*0x20 — 살아있는 쌍둥이 타워 수 (0.5.5 불변)
const O_T1: [usize; 3] = [0x180, 0x1a0, 0x1c0];   // 1차 Top/Mid/Bottom (+ side*8) (0.5.5 불변)
const O_T2: [usize; 3] = [0x190, 0x1b0, 0x1d0];   // 2차 Top/Mid/Bottom (+ side*8) (0.5.5 불변)

/// 상황별 적극도. 순서 = `NXE_KEYS` 와 1:1.
static NXE_LV: [AtomicI64; 8] = [const { AtomicI64::new(0) }; 8];
const NXE_KEYS: [&str; 8] = [
    "nxe_twin0", "nxe_twin1",
    "nxe_t2_1", "nxe_t2_2", "nxe_t2_3",
    "nxe_t1_1", "nxe_t1_2", "nxe_t1_3",
];
static NXE_DETOUR_ON: AtomicBool = AtomicBool::new(false);
static NXE_SIG: AtomicU64 = AtomicU64::new(u64::MAX);
// 진단
static NXE_HITS: AtomicU64 = AtomicU64::new(0);   // 판정 횟수
static NXE_EMG: AtomicU64 = AtomicU64::new(0);    // 그중 비상으로 본 횟수
static NXE_BY: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8];   // 어느 상황이 채택됐나

// ── ② 마진(강도) 사이트 ──────────────────────────────────────────────────
// `0xe097b5` = `test eax,0x100` + `je e097f5` (7B). **여기를 잡아야 술어 B 경로만** 조절된다.
// ⚠바로 아래 감산 블록(`0xe097bc`)에는 **진입 경로가 5개**나 합류한다(HP>45% 경로 포함) —
//   거기에 손대면 비상과 무관한 **정상 플레이 전반**이 바뀐다. 그래서 한 단계 위를 잡는다.
const NXM_RVA: usize = 0xda65f5;   // ★0.5.6(was 0.5.5 0xda65f5 — 호스트 0xd9b960→0xda5730 UNIQUE·BYTE=SAME·win 7B 일치 재확증). 구 0.5.5(~~0xe097b5~~) 정렬 이력 유지
const NXM_WIN: [u8; 7] = [0xa9, 0x00, 0x01, 0x00, 0x00, 0x74, 0x39];   // test eax,0x100 ; je +0x39
const NXM_PLAIN_RVA: usize = 0xda6635;   // 비상 아님 → 평범 경로 (★0.5.5 ~~0xe097f5~~ = je타깃 mov r13,[rbp+0x988])
const NXM_AFTER_RVA: usize = 0xda661b;   // 감산 블록 다음 (★0.5.5 ~~0xe097db~~ = cmp[rbp+0x840],r13)
// ★0.5.5 확인: NXM 함수(free_dist e088f0→d9b960 r=1.00) 프레임 슬롯 전부 불변(정렬서 [rbp+0x8d0/0x8c0/0x880] 동일).
const NXM_BASE_MARGIN_SLOT: usize = 0x880;   // [rbp+0x880] = dist(나, 최근접 적) (0.5.5 불변)
const O_REG_SLOT: i32 = 0x8d0;   // [rbp+0x8d0] = reg   (프롤로그에서 1회만 기록) (0.5.5 불변)
const O_SIDE_SLOT: i32 = 0x8c0;  // [rbp+0x8c0] = side  (〃) (0.5.5 불변)
/// 게임 원본 마진(월드 유닛). 맵 폭 ≈960,000 기준 3.1%.
const NXE_BASE_MARGIN: u64 = 30_000;
static NXM_ON: AtomicBool = AtomicBool::new(false);
static NXE_MHITS: AtomicU64 = AtomicU64::new(0);    // 마진 콜백 발화
static NXE_LAST_M: AtomicI64 = AtomicI64::new(-1);  // 마지막으로 공급한 마진

/// 지금 상황의 **적극도**를 구한다. 0 = 비상 아님. 두 콜백이 공유한다.
/// ★전부 fault-safe read — 못 읽으면 **보수적 기본값**(쌍둥이 2기·타워 생존)이라 함부로 발동하지 않는다.
unsafe fn nxe_level(reg: usize, side: u64, count: bool) -> i64 {
    if !ptr_ok(reg) || side > 1 { return 0; }
    let s = side as usize;
    let twin = rd_u64(reg + O_TWIN_LEN + s * 0x20).unwrap_or(2) as i64;
    let mut c1 = 0i64;
    let mut c2 = 0i64;
    for k in 0..3usize {
        if rd_u64(reg + O_T1[k] + s * 8).unwrap_or(1) == 0 { c1 += 1; }
        if rd_u64(reg + O_T2[k] + s * 8).unwrap_or(1) == 0 { c2 += 1; }
    }
    // ★해당되는 상황들 중 **가장 높은 적극도**를 쓴다(유저 지시).
    //   조건이 "이하 / 이상"이라 여러 개가 동시에 해당된다.
    let cond = [
        twin <= 0, twin <= 1,
        c2 >= 1, c2 >= 2, c2 >= 3,
        c1 >= 1, c1 >= 2, c1 >= 3,
    ];
    let mut best = 0i64;
    let mut who = usize::MAX;
    for i in 0..8usize {
        if !cond[i] { continue; }
        let l = NXE_LV[i].load(Ordering::Relaxed);
        if l > best { best = l; who = i; }
    }
    if count && best > 0 {
        NXE_EMG.fetch_add(1, Ordering::Relaxed);
        if who < 8 { NXE_BY[who].fetch_add(1, Ordering::Relaxed); }
    }
    best
}

/// 스텁①이 부르는 **비상 여부** 콜백. 반환 1 = 비상(원본의 "조건① 통과" 경로로).
///
/// ★게임의 원래 판정(쌍둥이 0기)도 `nxe_twin0` 으로 표현된다 — 기본 100 이라 **끄지 않는 한 원본과 같다**.
unsafe extern "C" fn nxe_judge(reg: usize, side: u64) -> u64 {
    NXE_HITS.fetch_add(1, Ordering::Relaxed);
    // ★패닉이 게임 콜스택으로 새면 UB(§3). 어떤 실패든 "비상 아님"으로 폴백한다.
    let v = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| nxe_level(reg, side, true)))
        .unwrap_or(0);
    if v > 0 { 1 } else { 0 }
}

/// 스텁②가 부르는 **마진** 콜백. 반환 = 전환 판정에서 뺄 거리(월드 유닛).
///
/// ★★부호에 주의 — 이 값은 "수비하러 오는 거리"가 아니라 **전환 히스테리시스 마진**이다.
///   `dist(지킬곳, 넥서스) <= dist(나, 넥서스) − M` 형태로 쓰이므로 **M 이 클수록 전환이 어렵다**
///   = 덜 온다. 그래서 적극도와 **반비례**로 매핑한다(적극도 100 = 원본 30,000).
///     100 → 30,000(원본) · 200 → 15,000 · 300 → 10,000 · 아주 크게 → 0(마진 없음 = 최대 적극)
///     50  → 60,000(원본보다 소극적)
unsafe extern "C" fn nxe_margin(reg: usize, side: u64) -> u64 {
    NXE_MHITS.fetch_add(1, Ordering::Relaxed);
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // count=false — 여기서 또 세면 비상 횟수가 두 배로 잡힌다(같은 판정을 두 지점이 본다).
        let lv = nxe_level(reg, side, false);
        if lv <= 0 { return NXE_BASE_MARGIN; }        // 비상 아님 → 원본 그대로
        let m = NXE_BASE_MARGIN as i64 * 100 / lv.clamp(1, 3_000_000);
        NXE_LAST_M.store(m, Ordering::Relaxed);
        m.clamp(0, 0x7fff_ffff) as u64
    })).unwrap_or(NXE_BASE_MARGIN)
}

/// 넥서스 비상 조건 노브 적용. apply 체인에서 부른다.
pub(crate) unsafe fn apply_nxe() {
    // ★`nxe_twin0` 기본 1 = **게임 원본 동작**(쌍둥이 0기면 비상). 나머지는 기본 0 = 추가 발동 없음.
    //   ⟹ cfg 를 안 건드리면 원본과 똑같이 동작한다.
    let mut lv = [0i64; 8];
    for i in 0..8usize {
        let d = if i == 0 { 100 } else { 0 };   // ★적극도 = 퍼센트. 100 = 게임 원본
        let v = tune(NXE_KEYS[i], d);
        lv[i] = if v < 0 { d } else { v };
    }
    let supp = tune("nxe_supp_off", -1);     // 1 = 비상일 때 다른 액션을 −99,999 로 죽이는 조항 해제
    let batt = tune("nxe_battle_off", -1);   // 1 = 교전 판단이 이 플래그를 무시

    let mut sig = 0u64;
    for v in lv.iter().chain([supp, batt].iter()) { sig = sig.wrapping_mul(0x100000001b3) ^ (*v as u64); }
    if sig == NXE_SIG.load(Ordering::Relaxed) { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }

    // 값은 콜백이 매 판정마다 읽는다 ⟹ 설치 후엔 여기 갱신만으로 즉시 반영(재설치 불요).
    for i in 0..8usize { NXE_LV[i].store(lv[i], Ordering::Relaxed); }

    // 원본과 다른 설정이 하나라도 있을 때만 디투어를 건다(기본 상태에선 게임을 아예 안 건드린다).
    let need = lv[0] != 100 || lv[1..].iter().any(|&x| x > 0);
    let mut rep = String::from(
        "=== 넥서스 비상 수비 — 상황별 적극도 ===\n\
         # 게임 원본 = \"쌍둥이 타워가 하나도 안 남았고 + 넥서스가 실제로 맞는 중\" 이면 비상.\n\
         # 비상이면 멀리 있어도 수비 후보가 살아남고(맵 어디서든 달려옴), 강제귀환이 취소된다.\n\
         # 아래 상황 중 해당되는 것들의 **가장 높은 적극도**를 쓴다. 0 = 그 상황은 비상 아님.\n\
         # ⚠지금은 적극도의 '크기'가 아니라 '0인가 아닌가'만 반영된다(술어가 참/거짓뿐이라).\n");
    for i in 0..8usize {
        rep.push_str(&format!("  {:<12} = {}\n", NXE_KEYS[i], lv[i]));
    }

    if need && !NXE_DETOUR_ON.load(Ordering::Relaxed) {
        match install_nxe_detour(base) {
            Ok(stub) => {
                NXE_DETOUR_ON.store(true, Ordering::Relaxed);
                rep.push_str(&format!("\n설치① 비상 판정   : OK  스텁={:#x}\n", stub));
            }
            Err(e) => rep.push_str(&format!("\n★설치① 실패: {} — 게임 원본 동작 그대로다\n", e)),
        }
        // ②는 ①이 실패해도 따로 시도한다(서로 독립된 자리다).
        match install_nxm_detour(base) {
            Ok(stub) => {
                NXM_ON.store(true, Ordering::Relaxed);
                rep.push_str(&format!("설치② 적극도 강도 : OK  스텁={:#x}\n", stub));
            }
            Err(e) => rep.push_str(&format!("★설치② 실패: {} — 강도는 원본(30,000) 고정\n", e)),
        }
    } else if !need {
        rep.push_str("\n설치: 안 함(설정이 전부 원본과 같음 — 게임을 건드리지 않는다)\n");
    } else {
        rep.push_str("\n설치: 이미 됨(값만 갱신 — 재시작 불요)\n");
    }

    // ── 부작용 중화(각 1사이트, 순수 imm) ──
    let mut ok = 0u32; let mut tot = 0u32;
    if supp > 0 {
        tot += 1;
        // `test eax,0x100` → `test eax,0` : 비상일 때 다른 액션을 −99,999 로 죽이던 조항이 사라진다.
        ok += patch_imm_bytes(base + 0xe3f152, &[0xa9], 1, 4, 0) as u32;   // ★0.5.6(was 0.5.4 0xd8e0b2 — ⚠0.5.5 회차 미재핀으로 stale였음. 054호스트 0xd8db90→056 0xe3ec30 정렬 r=0.743 + a9 00 01 00 00 0f 85 7B 일치 + 055 0xe35372 교차확증)
    }
    if batt > 0 {
        tot += 1;
        // `and eax,1` → `and eax,0` : 교전 판단이 이 플래그를 항상 0 으로 본다.
        ok += patch_imm_bytes(base + 0xd37a4b, &[0x83, 0xe0], 2, 1, 0) as u32;   // ★0.5.6(was 0.5.4 0xda3660 — ⚠0.5.5 회차 미재핀 stale였음. bt_vis 동일 호스트 0xda3570→055 0xd2db90→056 0xd37960 정렬·83 e0 01 3B 일치·뒤 명령 disp 0x3d8→0x3e8 이동은 별개 명령이라 무관)
    }
    if tot > 0 {
        rep.push_str(&format!("부작용 중화: {}/{} (억제조항 해제={} · 교전판단 무시={})\n",
            ok, tot, supp > 0, batt > 0));
    }

    NXE_SIG.store(sig, Ordering::Relaxed);
    if let Some(p) = pth("nxe.txt") { let _ = fs::write(p, rep); }
}

/// 런타임 확인용 요약(`class_verify=1` 일 때 class_verify.txt 에 덧붙는다).
pub(crate) fn nxe_summary() -> String {
    if !NXE_DETOUR_ON.load(Ordering::Relaxed) { return String::new(); }
    let hits = NXE_HITS.load(Ordering::Relaxed);
    let emg = NXE_EMG.load(Ordering::Relaxed);
    let mut s = format!("\n[넥서스 비상 수비] 판정={} 비상={} ({}%)\n",
        hits, emg, if hits > 0 { emg * 100 / hits } else { 0 });
    for i in 0..8usize {
        let n = NXE_BY[i].load(Ordering::Relaxed);
        if n > 0 { s.push_str(&format!("  {:<12} 로 발동 {}회\n", NXE_KEYS[i], n)); }
    }
    s.push_str("  ※ '로 발동' = 그 상황의 적극도가 가장 높아 채택된 횟수.\n");
    if NXM_ON.load(Ordering::Relaxed) {
        let m = NXE_LAST_M.load(Ordering::Relaxed);
        s.push_str(&format!("  강도 콜백 발화={} · 마지막 마진={} (원본 {})\n",
            NXE_MHITS.load(Ordering::Relaxed),
            if m < 0 { "—".to_string() } else { m.to_string() }, NXE_BASE_MARGIN));
        s.push_str("  ※ 마진이 작을수록 수비 전환이 쉬워진다(= 더 적극적). 적극도와 반비례.\n");
    }
    s
}

/// ② 마진 사이트 설치 — 술어 B 가 참일 때 감산할 값을 **적극도에 맞춰** 공급한다.
///
/// 원본은 `test eax,0x100` / `je 평범` 뒤에 `sub r13,30000` `cmovb` `mov rcx,[rbp+0x880]`
/// `sub rcx,30000` `cmovb` 를 실행한다. 우리는 그 블록을 **M 으로 재현**하고 그 다음(`0xe097db`)으로 간다.
unsafe fn install_nxm_detour(base: usize) -> Result<usize, &'static str> {
    let addr = base + NXM_RVA;
    if !readable(addr, NXM_WIN.len()) { return Err("마진 사이트 읽기 불가"); }
    for (i, &b) in NXM_WIN.iter().enumerate() {
        if rd_u8(addr + i) != b { return Err("마진 사이트 골격 불일치 — 게임이 바뀌었다"); }
    }

    let cb = nxe_margin as *const () as usize;
    let mut s: Vec<u8> = Vec::with_capacity(224);
    let mut fix_body: Vec<usize> = Vec::new();

    // ★비상이 아니면 아무것도 저장하지 않고 곧장 평범 경로로 — 가장 흔한 경로를 최단으로.
    s.extend_from_slice(&[0xa9, 0x00, 0x01, 0x00, 0x00]);          // test eax, 0x100
    s.extend_from_slice(&[0x75, 0x00]); fix_body.push(s.len() - 1); // jnz .body
    s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]);     // jmp [rip+0] → 평범
    s.extend_from_slice(&(base + NXM_PLAIN_RVA).to_le_bytes());
    let body_off = s.len();

    // ── .body : 마진 M 을 구해 원본 감산 블록을 재현 ──
    // 스택(높은→낮은): [M 8B] [rax rcx rdx r8 r9 r10 r11 rbp] [xmm0~5]
    s.extend_from_slice(&[0x48, 0x8d, 0x64, 0x24, 0xf8]);          // lea rsp,[rsp-8]  (M 슬롯)
    for b in [0x50u8, 0x51, 0x52, 0x41, 0x50, 0x41, 0x51, 0x41, 0x52, 0x41, 0x53, 0x55] { s.push(b); }
    s.extend_from_slice(&[0x48, 0x8d, 0xa4, 0x24, 0xa0, 0xff, 0xff, 0xff]);   // lea rsp,[rsp-0x60]
    for k in 0..6u8 { s.extend_from_slice(&[0x0f, 0x11, 0x44 | ((k & 7) << 3), 0x24, k * 16]); }
    // ★rbp 는 아직 게임 프레임 — 앵커를 세우기 **전에** reg·side 를 읽는다.
    s.extend_from_slice(&[0x48, 0x8b, 0x8d]); s.extend_from_slice(&O_REG_SLOT.to_le_bytes());   // mov rcx,[rbp+0x8d0]
    s.extend_from_slice(&[0x48, 0x8b, 0x95]); s.extend_from_slice(&O_SIDE_SLOT.to_le_bytes());  // mov rdx,[rbp+0x8c0]
    s.extend_from_slice(&[0x48, 0x89, 0xe5]);                      // mov rbp, rsp   (앵커)
    s.extend_from_slice(&[0x48, 0x8d, 0x64, 0x24, 0xd0]);          // lea rsp,[rsp-0x30]
    s.extend_from_slice(&[0x48, 0x83, 0xe4, 0xf0]);                // and rsp,-16
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&cb.to_le_bytes());
    s.extend_from_slice(&[0xff, 0xd0]);                            // call rax  → rax = M
    s.extend_from_slice(&[0x48, 0x89, 0x85]); s.extend_from_slice(&0xa0u32.to_le_bytes()); // mov [rbp+0xa0],rax
    s.extend_from_slice(&[0x48, 0x89, 0xec]);                      // mov rsp, rbp
    for k in 0..6u8 { s.extend_from_slice(&[0x0f, 0x10, 0x44 | ((k & 7) << 3), 0x24, k * 16]); }
    s.extend_from_slice(&[0x48, 0x8d, 0x64, 0x24, 0x60]);          // lea rsp,[rsp+0x60]
    for b in [0x5du8, 0x41, 0x5b, 0x41, 0x5a, 0x41, 0x59, 0x41, 0x58, 0x5a, 0x59, 0x58] { s.push(b); }
    // 여기서 rsp = M 슬롯. 원본 블록을 M 으로 재현한다(순서·의미 동일).
    s.extend_from_slice(&[0x31, 0xc0]);                            // xor eax,eax      (포화용 0)
    s.extend_from_slice(&[0x4c, 0x2b, 0x2c, 0x24]);                // sub r13,[rsp]
    s.extend_from_slice(&[0x4c, 0x0f, 0x42, 0xe8]);                // cmovb r13,rax
    s.extend_from_slice(&[0x48, 0x8b, 0x8d]);
    s.extend_from_slice(&(NXM_BASE_MARGIN_SLOT as u32).to_le_bytes());   // mov rcx,[rbp+0x880]
    s.extend_from_slice(&[0x48, 0x2b, 0x0c, 0x24]);                // sub rcx,[rsp]
    s.extend_from_slice(&[0x48, 0x0f, 0x42, 0xc8]);                // cmovb rcx,rax
    s.extend_from_slice(&[0x48, 0x8d, 0x64, 0x24, 0x08]);          // lea rsp,[rsp+8]  (플래그 무영향)
    s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]);    // jmp [rip+0] → 감산 블록 다음
    s.extend_from_slice(&(base + NXM_AFTER_RVA).to_le_bytes());
    for &pos in fix_body.iter() {
        let d = body_off as i64 - (pos as i64 + 1);
        if !(0..=127).contains(&d) { return Err("마진 스텁 분기 rel8 범위 초과"); }
        s[pos] = d as u8;
    }

    let stub = micro_alloc(addr, s.len());
    if stub == 0 { return Err("마진 스텁 할당 실패"); }
    let rel = stub as i64 - (addr as i64 + 5);
    if !(-0x7fff_0000..=0x7fff_0000).contains(&rel) { return Err("마진 스텁이 rel32 범위 밖"); }
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());

    let mut e9 = [0x90u8; 5];
    e9[0] = 0xe9;
    e9[1..5].copy_from_slice(&(rel as i32).to_le_bytes());
    let mut old: u32 = 0;
    if VirtualProtect(addr, NXM_WIN.len(), 0x40, &mut old) == 0 { return Err("VirtualProtect 실패"); }
    core::ptr::copy_nonoverlapping(e9.as_ptr(), addr as *mut u8, 5);
    VirtualProtect(addr, NXM_WIN.len(), old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, NXM_WIN.len());
    Ok(stub)
}

/// 창 전수 대조 후 `E9` 로 우리 스텁에 보낸다.
unsafe fn install_nxe_detour(base: usize) -> Result<usize, &'static str> {
    let addr = base + NXE_RVA;
    if !readable(addr, NXE_WIN.len()) { return Err("사이트 읽기 불가"); }
    // ★원본 전수 대조 — 한 바이트라도 다르면 설치하지 않는다(게임 패치 방어).
    for (i, &b) in NXE_WIN.iter().enumerate() {
        if rd_u8(addr + i) != b { return Err("명령 골격 불일치 — 게임이 바뀌었다"); }
    }

    let cb = nxe_judge as *const () as usize;
    let mut s: Vec<u8> = Vec::with_capacity(224);
    let mut fix_take: Vec<usize> = Vec::new();

    // ── 게임 레지스터 보존 (class_micro 스텁과 같은 배치) ──
    // 스택(높은→낮은): [판정결과 8B] [rax rcx rdx r8 r9 r10 r11 rbp] [xmm0~5]
    // 플래그는 복귀 지점이 소비하지 않으므로 보존하지 않는다.
    s.extend_from_slice(&[0x48, 0x8d, 0x64, 0x24, 0xf8]);          // lea rsp,[rsp-8]  (결과 슬롯)
    for b in [0x50u8, 0x51, 0x52, 0x41, 0x50, 0x41, 0x51, 0x41, 0x52, 0x41, 0x53, 0x55] { s.push(b); }
    s.extend_from_slice(&[0x48, 0x8d, 0xa4, 0x24, 0xa0, 0xff, 0xff, 0xff]);   // lea rsp,[rsp-0x60]
    for k in 0..6u8 { s.extend_from_slice(&[0x0f, 0x11, 0x44 | ((k & 7) << 3), 0x24, k * 16]); }
    // ★인자: arg1=reg(rbx), arg2=side(rcx). **rdx 를 먼저** 채운다 — rcx 를 먼저 덮으면 side 가 사라진다.
    s.extend_from_slice(&[0x48, 0x89, 0xca]);                      // mov rdx, rcx   (side)
    s.extend_from_slice(&[0x48, 0x89, 0xd9]);                      // mov rcx, rbx   (reg)
    s.extend_from_slice(&[0x48, 0x89, 0xe5]);                      // mov rbp, rsp   (앵커)
    s.extend_from_slice(&[0x48, 0x8d, 0x64, 0x24, 0xd0]);          // lea rsp,[rsp-0x30]
    s.extend_from_slice(&[0x48, 0x83, 0xe4, 0xf0]);                // and rsp,-16
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&cb.to_le_bytes());   // movabs rax, cb
    s.extend_from_slice(&[0xff, 0xd0]);                            // call rax
    s.extend_from_slice(&[0x48, 0x89, 0x85]); s.extend_from_slice(&0xa0u32.to_le_bytes()); // mov [rbp+0xa0],rax
    s.extend_from_slice(&[0x48, 0x89, 0xec]);                      // mov rsp, rbp
    for k in 0..6u8 { s.extend_from_slice(&[0x0f, 0x10, 0x44 | ((k & 7) << 3), 0x24, k * 16]); }
    s.extend_from_slice(&[0x48, 0x8d, 0x64, 0x24, 0x60]);          // lea rsp,[rsp+0x60]
    for b in [0x5du8, 0x41, 0x5b, 0x41, 0x5a, 0x41, 0x59, 0x41, 0x58, 0x5a, 0x59, 0x58] { s.push(b); }
    // 여기서 rsp = 결과 슬롯.
    s.extend_from_slice(&[0x48, 0x83, 0x3c, 0x24, 0x00]);          // cmp qword [rsp], 0
    s.extend_from_slice(&[0x48, 0x8d, 0x64, 0x24, 0x08]);          // lea rsp,[rsp+8]  (LEA=플래그 무영향)
    s.extend_from_slice(&[0x75, 0x00]); fix_take.push(s.len() - 1); // jne .take
    // .skip : 원본의 "비상 아님" 경로
    s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]);
    s.extend_from_slice(&(base + NXE_FAIL_RVA).to_le_bytes());
    let take_off = s.len();
    // .take : 원본의 "비상" 경로(조건② 검사로)
    s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]);
    s.extend_from_slice(&(base + NXE_PASS_RVA).to_le_bytes());
    // ★분기 rel8 은 자리를 기록해 두고 채운다 — 손으로 세면 한 바이트만 어긋나도 명령 중간으로 뛴다.
    for &pos in fix_take.iter() {
        let d = take_off as i64 - (pos as i64 + 1);
        if !(0..=127).contains(&d) { return Err("분기 rel8 범위 초과"); }
        s[pos] = d as u8;
    }

    let stub = micro_alloc(addr, s.len());
    if stub == 0 { return Err("스텁 할당 실패(±2GB 내 여유 없음)"); }
    let rel = stub as i64 - (addr as i64 + 5);
    if !(-0x7fff_0000..=0x7fff_0000).contains(&rel) { return Err("스텁이 rel32 범위 밖"); }
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());

    // ★앞 5바이트만 쓴다 — 뒤 6B 는 E9 가 자리잡는 순간 도달 불가가 되고,
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
