// ════════════════════════════════════════════════════════════════════════════════
// ★런타임 진단 프로브 (`probe` cfg 키, 기본 0=완전 OFF)
// ════════════════════════════════════════════════════════════════════════════════
// 정적 분석만으로는 못 뚫린 6건을 **한 경기 관찰로** 동시에 확정하기 위한 계측이다.
// 조사 결과는 `REPORT\tfm2_ai_adjust\RE\2026-08-04_*.md` 4건 참조.
//
// | # | 물음 | 방법 |
// |---|---|---|
// | 1 | 부쉬 왕복의 원인이 **시야 플립플롭(A)** 인가 **임계 데드밴드(B)** 인가 | 훅 3개 카운터 비율 |
// | 2 | `apply_input` 입구의 정체불명 게이트(`entity+0x2F0`)가 항상 통과하는가 | 패시브 읽기 분포 |
// | 3 | 갱 웨이브 인덱스의 방향(0 = 내 진영 끝인가) | `sim+0x21c0` 6칸 스냅샷 |
// | 4 | 맵 랜드마크 27좌표 중 어느 것이 포탑인가 | `sim+lane*0x20+0x180/0x190` 스냅샷 |
// | 5 | `outline_type`의 `Outline`/`Inline`이 死 값인가 | 값 히스토그램 |
// | 6 | 자리선택 밴드에 들어가는 "판단력"의 실제 값 범위 | 원시값 히스토그램 |
//
// ⛔⛔**설계 제약 — 핫패스에 파일 IO·락·할당 금지.**
//   2026-07-22에 계측 하나가 게임을 죽인 전례가 있다. 원인 = rayon 워커가 sim을 **병렬** 처리하는데
//   전환 감지가 단일 스레드를 가정 → 매 ms 수십 번 오인 → 동기 IO 폭주(4.7MB/판) → 게임 사망.
//   그때 남긴 규칙이 **"원자 카운터만 누적하고 post_update(단일 스레드)에서 주기 스냅샷 1줄"** 이고,
//   이 프로브는 그 규칙을 그대로 따른다. 훅 스텁은 `lock inc` 밖에 하지 않는다.
//
// [훅 4곳 — 전부 `.text` 전수 분기 스캔으로 "슬롯 내부 착지 0건" 확인]
//   H1 `0xca4ca1`(14B) hide: 적이 나를 **봄** 경로
//   H2 `0xca4d73`(15B) hide: **안 봄 → 부쉬로** 경로  (+ `out_line` 값 히스토)
//   H3 `0xca46ca`(14B) hide: 부쉬가 100,000보다 **멀다 → 제안 유지** 경로
//   H4 `0xd874cf`(12B) 자리선택 밴드: 판단력 원시값 `[rbp+0x290]`
//   ⛔`0xca46b7`(데드밴드 cmp 자체)은 **슬롯 내부로 분기가 들어와 제외**했다. H3(반대편 경로)로 대신 센다.
//
// [읽는 법]
//   `probe=1` → `probe.txt`에 60초마다 1줄. 경기 한 판이면 충분하다.
//   ⚠**측정이 끝나면 0으로 되돌릴 것**(배포 전 체크리스트 대상).
// ════════════════════════════════════════════════════════════════════════════════

// ════════════════════════════════════════════════════════════════════════════════
// ★2차 개정 (2026-08-04) — 1차 계측의 결함 2건을 고쳤다
//   ⛔결함1: `0xca46ca`와 `0xca4d73`을 같은 모집단으로 보고 비율을 냈다(215%가 나왔다).
//            실제로는 **hide가 2상태 기계**라 둘은 상호배타 경로다(RE 확정).
//            ⟹ Phase 0/1 각각의 말단을 **전수로** 세고, `hide` 총 진입(H6)을 분모로 둔다.
//   ⛔결함2: 웨이브 인덱스 베이스가 한 단계 깊었다. `sim`이 아니라 **`snap`(= movepri 6번째 인자의 역참조)**
//            이고 폭도 u32가 아니라 **u64**였다. `sim`은 `snap`의 0번 필드(world)일 뿐이다.
//   ✅포탑 좌표는 런타임 탐색이 **불필요해졌다** — 랜드마크 27좌표가 곧 라인 노드이고 정적으로 라벨링됐다.
// ════════════════════════════════════════════════════════════════════════════════
const PB_H1_RVA: usize = 0xca4ca1;   // Phase 1: 적이 나를 봄
const PB_H2_RVA: usize = 0xca4d73;   // Phase 1: 안 봄 → 부쉬로
const PB_H3_RVA: usize = 0xca46ca;   // Phase 0: 랜드마크가 멀다 (P분기)
const PB_H4_RVA: usize = 0xd874cf;   // 자리선택 밴드 진입
const PB_H5_RVA: usize = 0xca4825;   // Phase 0: 랜드마크가 멀다 (Q분기 — P의 짝, 분모 완성용)
const PB_H6_RVA: usize = 0xca43e6;   // ★hide 총 진입 (모든 비율의 공통 분모)
const PB_H7_RVA: usize = 0xca492e;   // ★Phase 0 → 1 래치 전이 (= 랜드마크 도착)
// ★[08-04 3차] AI 판단 스로틀 — `now < agent+0x21c8`면 판단을 통째로 건너뛰고 직전 결정을 유지한다.
//   주기 = gen_range([400+3r, 800+4r])/100 = 4~12틱, `r = 100 - clamp(p4[0x400]*p4[0x200]/1000, 0, 100)`.
//   ⟹ **능력치가 높을수록 자주 판단**한다. 기준이 sim tick이라 관전 배속과는 무관(RE 확정).
//   여기서 재는 것: ①게이트 진입 A ②실제 판단 B ⟹ **실행률 B/A** ③`r`의 두 입력값 분포
//     — 분포가 한 점에 몰리면 **게임 옵션**, 퍼지면 **선수 능력치**다(이게 판별의 핵심).
const PB_H8_RVA: usize = 0xd0cab0;   // 스로틀 게이트(판단 본체의 유일한 콜러)
const PB_H9_RVA: usize = 0xd0a060;   // 판단 본체
const PB_H8_LEN: usize = 12;
const PB_H9_LEN: usize = 12;
// ★[08-04] 버프 가치 평가기 `0xcc4740`의 **死항 판정 실측**.
//   이 함수가 읽는 StatDelta 필드 35개 중 19개가 "생산자가 항상 0으로 채운다"는 이유로 死 판정을 받았는데,
//   그 판정은 **생산자가 `cc94c0`인 경로 한정**이다(다른 생산자 경로는 미조사).
//   ⟹ 진입부에서 대표 8필드를 직접 읽어 **한 경기 동안 한 번이라도 비-0이 나오는지** 본다.
//   전부 0이면 死 판정이 실측으로 확정되고, 하나라도 켜지면 판정 범위가 좁혀진다.
const PB_HA_RVA: usize = 0xcc4740;
const PB_HA_LEN: usize = 12;
// ★[08-05] 층① 플랜 결정기의 **최상위 3-way 스위치** `G.vt[0x30]()` 반환값 실측.
//   `0xd48ec0`(handler.rs, plan 4·5·6의 생산자) 진입 직후:
//     `d48f45 mov rsi,[rax+0x30]` → `d48f4c call rsi` → **`d48f4e cmp rax,2`** → `d48f67 jne`
//   분기 구조(==2→plan6 / ==1→plan4·5 / 그 외→plan7)는 확정인데 **그 값의 게임 의미가 미상**이라
//   층①이 85%에 묶여 있다. 반환값 분포만 보면 정체가 좁혀진다
//   (2값뿐 = 불리언성 상태 / 3값 고정 = 열거형 / 넓게 퍼짐 = 카운트·핸들).
//   ⚠훅 지점은 `call` **다음** 명령이라 `rax`가 곧 반환값이다. 이 자리는 함수 진입부가 아니므로
//     rax를 파괴하는 12B 폼을 쓰면 안 된다 — 18B 확보해 **레지스터 무파괴 14B 폼**을 쓴다.
//   ⚠`cmp rax,2`가 세운 플래그를 `0xd48f67 jne`가 소비한다. 공용 스텁이 pushfq/popfq로 감싸고
//     훔친 원본(`cmp` 포함)을 work **뒤에** 재실행하므로 플래그는 정상이다.
//   ✅`[0xd48f4e, 0xd48f60)` 안으로 들어오는 분기 타깃 = **0개**(함수 전체 스캔으로 확인).
const PB_HB_RVA: usize = 0xd48f4e;
const PB_HB_LEN: usize = 18;
const PB_HB_ORIG: [u8; 18] = [0x48,0x83,0xf8,0x02,
                              0x48,0x89,0xbd,0xe0,0x05,0x00,0x00,
                              0x4c,0x89,0xbd,0x88,0x05,0x00,0x00];
/// 검사할 StatDelta 오프셋(전부 死 판정을 받은 것). 비트 순서 = 이 배열 순서.
const PB_SD_OFFS: [(u32, &str); 8] = [
    (0xa8, "방어구관통"), (0xb0, "마법저항관통"), (0xd0, "온힛%최대체력"), (0xc0, "행4·5행렬"),
    (0xa0, "아군밀집보너스"), (0xe0, "적체력비례"), (0xd8, "대상체력비례"), (0xf0, "스킬계수"),
];
const PB_H1_LEN: usize = 14;
const PB_H2_LEN: usize = 15;
const PB_H3_LEN: usize = 14;
const PB_H5_LEN: usize = 14;
const PB_H6_LEN: usize = 14;
const PB_H7_LEN: usize = 17;
const PB_H4_LEN: usize = 15;
/// 훅 설치 전 원본 바이트 대조용(하나라도 다르면 **아무것도 설치하지 않는다**)
const PB_H1_ORIG: [u8; 14] = [0x48,0x8b,0x85,0x98,0x05,0x00,0x00, 0x48,0x8b,0x88,0xa8,0x05,0x00,0x00];
const PB_H2_ORIG: [u8; 15] = [0x48,0x8b,0x85,0x50,0x05,0x00,0x00, 0x0f,0xb6,0x40,0x08, 0x88,0x44,0x24,0x20];
const PB_H3_ORIG: [u8; 14] = [0x48,0x8b,0x85,0x90,0x05,0x00,0x00, 0x48,0x8b,0x08, 0x48,0x8b,0x40,0x08];
// ⚠H4는 **15바이트**다. 12바이트로 끊으면 레지스터 무파괴 점프(14B)가 안 들어가고,
//   그 자리에서 `rax`가 살아 있어(`0xd874cb cmovb rax,r8` → `0xd874f2 lea eax,[rax+rax*4]`)
//   rax를 쓰는 12B 폼(`movabs rax; jmp rax`)은 **결과를 망가뜨린다.**
//   마지막 `sub rcx,rdx`까지 훔쳐 재실행하므로 뒤따르는 `cmovb rcx,r8`의 플래그도 정상이다.
const PB_H4_ORIG: [u8; 15] = [0xb9,0x32,0x00,0x00,0x00, 0x48,0x8b,0x95,0x90,0x02,0x00,0x00, 0x48,0x29,0xd1];
const PB_H5_ORIG: [u8; 14] = [0x48,0x8b,0x85,0x90,0x05,0x00,0x00, 0x48,0x8b,0x08, 0x48,0x8b,0x40,0x08];  // H3와 바이트 동일
const PB_H6_ORIG: [u8; 14] = [0x4c,0x89,0x8d,0x88,0x05,0x00,0x00, 0x4c,0x89,0x85,0x10,0x05,0x00,0x00];
const PB_H7_ORIG: [u8; 17] = [0x41,0xc6,0x04,0x24,0x01, 0x41,0x80,0x3c,0x24,0x00, 0x4c,0x8b,0xb5,0x48,0x05,0x00,0x00];
/// H8·H9는 함수 진입 프롤로그(8-push)가 정확히 12바이트다. 인자 레지스터는 그대로 살아 있다.
const PB_PROLOG8: [u8; 12] = [0x55, 0x41,0x57, 0x41,0x56, 0x41,0x55, 0x41,0x54, 0x56, 0x57, 0x53];
/// ⚠`0xcc4740`은 push 순서가 다르다(rbp가 뒤). 같은 8-push라도 **바이트열이 달라** 별도 상수가 필요하다.
const PB_PROLOG8B: [u8; 12] = [0x41,0x57, 0x41,0x56, 0x41,0x55, 0x41,0x54, 0x56, 0x57, 0x55, 0x53];

// ── 누적 카운터 (훅 스텁이 `lock inc` 로만 건드린다) ──
static PB_SEEN:  AtomicU64 = AtomicU64::new(0);   // H1 Phase1: 적이 나를 봄
static PB_BUSH:  AtomicU64 = AtomicU64::new(0);   // H2 Phase1: 안 봄 → 부쉬로
static PB_FAR:   AtomicU64 = AtomicU64::new(0);   // H3 Phase0: 랜드마크 멂(P분기)
static PB_FARQ:  AtomicU64 = AtomicU64::new(0);   // H5 Phase0: 랜드마크 멂(Q분기)
static PB_ENTER: AtomicU64 = AtomicU64::new(0);   // H6 hide 총 진입 (공통 분모)
static PB_LATCH: AtomicU64 = AtomicU64::new(0);   // H7 Phase0→1 래치 전이
static PB_THINK_GATE: AtomicU64 = AtomicU64::new(0);   // H8 스로틀 게이트 진입
static PB_THINK_RUN:  AtomicU64 = AtomicU64::new(0);   // H9 실제 판단 실행
/// `r` 입력 2종의 분포(0..126 버킷, 127 = 범위 밖). 한 점에 몰리면 옵션, 퍼지면 선수 능력치.
static PB_S200: [AtomicU64; 128] = [const { AtomicU64::new(0) }; 128];
static PB_S400: [AtomicU64; 128] = [const { AtomicU64::new(0) }; 128];
/// ★[08-05] 위 히스토그램은 **127에서 클램프**한다. `p4[0x400]`은 1000으로 **추정**돼 있어
///   그 추정이 맞으면 표본이 전부 127칸(범위 밖)으로 몰려 **실제 값을 못 본다**.
///   ⟹ 원값을 그대로 남기고, OR·AND 누적으로 상수 여부까지 한 번에 판정한다:
///   경기 후 `OR == AND` 이면 **경기 내내 단일 상수**(= 게임 옵션·전역 설정),
///   다르면 **표본마다 다름**(= 선수 능력치). LAST는 그 상수/마지막 표본의 실제 값이다.
static PB_S200_LAST: AtomicU64 = AtomicU64::new(0);
static PB_S200_OR:   AtomicU64 = AtomicU64::new(0);
static PB_S200_AND:  AtomicU64 = AtomicU64::new(u64::MAX);
static PB_S400_LAST: AtomicU64 = AtomicU64::new(0);
static PB_S400_OR:   AtomicU64 = AtomicU64::new(0);
static PB_S400_AND:  AtomicU64 = AtomicU64::new(u64::MAX);
/// HB: `G.vt[0x30]()` 반환값 분포(0..6 버킷, 7 = 7 이상). 확정된 분기는 2 / 1 / 그 외 3-way.
static PB_VT30: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8];
static PB_VT30_TOT:  AtomicU64 = AtomicU64::new(0);
static PB_VT30_LAST: AtomicU64 = AtomicU64::new(0);
static PB_VT30_OR:   AtomicU64 = AtomicU64::new(0);
static PB_BUFF_CALLS: AtomicU64 = AtomicU64::new(0);   // HA 진입 수
/// 死 판정 필드가 **한 번이라도 비-0이면** 해당 비트가 켜진다(경기 내내 0이면 死 확정).
static PB_SD_MASK: AtomicU64 = AtomicU64::new(0);
static PB_OUTLINE: [AtomicU64; 4] = [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];
/// 판단력 원시값 0~100 히스토그램(범위 밖은 버린다)
static PB_JUDGE: [AtomicU64; 101] = [const { AtomicU64::new(0) }; 101];

// ── 패시브 관측 (mp_capture 경로에서 채운다 — 훅 추가 없음) ──
static PB_SNAP:     AtomicUsize = AtomicUsize::new(0);   // ★AI 판단 스냅샷 = 라인통제 인덱스의 진짜 베이스
static PB_MAP:      AtomicUsize = AtomicUsize::new(0);   // MapDef (랜드마크 좌표표)
static PB_SIM:      AtomicUsize = AtomicUsize::new(0);   // world (= snap의 0번 필드). 틱은 여기 +0xeb00
static PB_ENT:      AtomicUsize = AtomicUsize::new(0);   // 마지막으로 본 entity
static PB_E2F0_TOT: AtomicU64 = AtomicU64::new(0);       // entity+0x2F0 관측 횟수
static PB_E2F0_MIN: AtomicU64 = AtomicU64::new(0);       // 그중 == i64::MIN 이었던 횟수(게이트 통과 조건)
static PB_E2F0_LAST:AtomicU64 = AtomicU64::new(0);       // 마지막 값(참고)

static PB_INSTALLED: AtomicI64 = AtomicI64::new(-1);     // -1=미결정 / 0=미설치 / 1=설치됨
static PB_LAST_SNAP: AtomicU64 = AtomicU64::new(0);      // 마지막 스냅샷 시각(READY_TICKS)

#[inline] fn probe_on() -> bool { tune("probe", 0) != 0 }

/// mp_capture 진입에서 부르는 수집기. `judge_dump_capture`와 같은 포인터 경로를 타되
/// **읽기만** 하고 파일은 건드리지 않는다(핫패스 IO 금지 규칙).
unsafe fn probe_collect(_saved: usize, entry_rsp: usize) {
    let p5 = rd_u64(entry_rsp + 0x28).unwrap_or(0) as usize;
    let p6 = rd_u64(entry_rsp + 0x30).unwrap_or(0) as usize;
    if !ptr_ok(p5) || !ptr_ok(p6) { return; }
    // ★`snap`(= [p6])이 라인통제 인덱스의 베이스다. 1차 계측은 여기서 한 번 더 들어간
    //   `world`(= [snap])를 베이스로 써서 포인터 배열 위를 읽고 있었다.
    let snap = rd_u64(p6).unwrap_or(0) as usize;
    if !ptr_ok(snap) { return; }
    let sim = rd_u64(snap).unwrap_or(0) as usize;
    if !ptr_ok(sim) || !readable(sim + 0xec68, 8) { return; }
    PB_SNAP.store(snap, Ordering::Relaxed);
    // MapDef = [[p6+8] + 0x20] — 랜드마크 좌표표(+0x6BA0)를 읽기 위해
    let prov = rd_u64(p6 + 8).unwrap_or(0) as usize;
    if ptr_ok(prov) {
        let m = rd_u64(prov + 0x20).unwrap_or(0) as usize;
        if ptr_ok(m) { PB_MAP.store(m, Ordering::Relaxed); }
    }
    let handle = rd_u64(p5 + 0x938).unwrap_or(0);
    let ent = dd7_slot128(sim, handle);
    probe_observe(sim, ent);
}

/// sim/entity를 캐시하고 `+0x2F0` 분포만 센다.
unsafe fn probe_observe(sim: usize, ent: usize) {
    if !ptr_ok(sim) || !ptr_ok(ent) { return; }
    PB_SIM.store(sim, Ordering::Relaxed);
    PB_ENT.store(ent, Ordering::Relaxed);
    if let Some(v) = rd_u64(ent + 0x2f0) {
        PB_E2F0_TOT.fetch_add(1, Ordering::Relaxed);
        PB_E2F0_LAST.store(v, Ordering::Relaxed);
        if v == 0x8000_0000_0000_0000 { PB_E2F0_MIN.fetch_add(1, Ordering::Relaxed); }
    }
}

/// 프로브 스텁 1개를 만든다. `work` = 카운터 조작 기계어(레지스터는 호출자가 이미 보존해 둠).
unsafe fn pb_build_stub(tag: usize, work: &[u8], orig: &[u8], ret_addr: usize) -> usize {
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = stub_reg(VirtualAlloc(0, 256, MEM_CR, RWX), 256, 0xF30 + tag);
    if stub == 0 { return 0; }
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x50, 0x51, 0x9c]);          // push rax; push rcx; pushfq
    s.extend_from_slice(work);
    s.extend_from_slice(&[0x9d, 0x59, 0x58]);          // popfq; pop rcx; pop rax
    s.extend_from_slice(orig);                         // 훔친 원본 재실행 (rbp/rsp 상대라 재배치 불필요)
    s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]);   // jmp qword [rip+0]
    s.extend_from_slice(&ret_addr.to_le_bytes());
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());
    stub
}

/// 단순 카운터 증가: `movabs rax, &cnt ; lock inc qword [rax]`
fn pb_work_inc(cnt: &AtomicU64) -> Vec<u8> {
    let mut w = vec![0x48, 0xb8];
    w.extend_from_slice(&(cnt as *const _ as usize).to_le_bytes());
    w.extend_from_slice(&[0xf0, 0x48, 0xff, 0x00]);    // lock inc qword [rax]
    w
}

/// 짧은 조건점프를 붙인다. **변위를 손으로 세지 않는다** — 뒤에 올 바이트열 길이에서 계산한다.
/// (이번 세션 교훈: 손으로 센 오프셋이 이 프로젝트에서 반복적으로 어긋났다.)
/// ⚠**128바이트를 넘으면 자동으로 rel32 폼으로 바꾼다.**
///   옛 코드는 `debug_assert!`만 두고 rel8을 강제했는데, **릴리스 빌드에선 assert가 꺼져** 있어
///   233바이트짜리 블록에서 변위가 음수로 접혀 **엉뚱한 곳으로 점프**했다(2026-08-04, 역어셈 검증에서 발견).
///   Jcc rel8 `0x7X` → rel32 는 `0x0F 0x(X+0x10)` 로 일대일 대응한다.
fn pb_jmp_over(w: &mut Vec<u8>, opcode: u8, skipped: &[u8]) {
    if skipped.len() <= 127 {
        w.push(opcode);
        w.push(skipped.len() as u8);
    } else {
        w.push(0x0f);
        w.push(opcode + 0x10);
        w.extend_from_slice(&(skipped.len() as u32).to_le_bytes());
    }
    w.extend_from_slice(skipped);
}

/// H2 전용: 부쉬 카운터 + `out_line`(= `[[rbp+0x550]+8]`) 히스토그램
fn pb_work_h2() -> Vec<u8> {
    // 범위(0..3) 안일 때만 세는 부분
    let mut tally: Vec<u8> = vec![0x48, 0xb8];                            // movabs rax, &PB_OUTLINE
    tally.extend_from_slice(&(&PB_OUTLINE as *const _ as usize).to_le_bytes());
    tally.extend_from_slice(&[0xf0, 0x48, 0xff, 0x04, 0xc8]);            // lock inc qword [rax+rcx*8]
    // 포인터가 살아 있을 때만 도는 부분
    let mut body: Vec<u8> = vec![0x0f, 0xb6, 0x48, 0x08];                // movzx ecx,byte [rax+8]
    body.extend_from_slice(&[0x48, 0x83, 0xf9, 0x03]);                   // cmp rcx,3
    pb_jmp_over(&mut body, 0x77, &tally);                                // ja → 범위 밖이면 버림

    let mut w = pb_work_inc(&PB_BUSH);
    w.extend_from_slice(&[0x48, 0x8b, 0x85, 0x50, 0x05, 0x00, 0x00]);   // mov rax,[rbp+0x550]
    w.extend_from_slice(&[0x48, 0x85, 0xc0]);                            // test rax,rax
    pb_jmp_over(&mut w, 0x74, &body);                                    // jz → null이면 통째로 건너뜀
    w
}

/// H8 전용: 게이트 카운터 + `r` 입력 2종(`[r9+0x200]`·`[r9+0x400]`)의 분포.
/// 함수 진입부라 4번째 인자 `r9`이 그대로 살아 있다. 클램프는 `cmova`(분기 없음).
fn pb_work_h8() -> Vec<u8> {
    let mut w = pb_work_inc(&PB_THINK_GATE);
    for (disp, hist) in [(0x200u32, &PB_S200), (0x400u32, &PB_S400)] {
        w.extend_from_slice(&[0x49, 0x8b, 0x81]);                     // mov rax,[r9+disp32]
        w.extend_from_slice(&disp.to_le_bytes());
        w.extend_from_slice(&[0xb9, 0x7f, 0x00, 0x00, 0x00]);         // mov ecx,127
        w.extend_from_slice(&[0x48, 0x83, 0xf8, 0x7f]);               // cmp rax,127
        w.extend_from_slice(&[0x48, 0x0f, 0x47, 0xc1]);               // cmova rax,rcx  (범위 밖 → 127칸)
        w.extend_from_slice(&[0x48, 0xb9]);                           // movabs rcx,&hist
        w.extend_from_slice(&(hist as *const _ as usize).to_le_bytes());
        w.extend_from_slice(&[0xf0, 0x48, 0xff, 0x04, 0xc1]);         // lock inc qword [rcx+rax*8]
    }
    // ★[08-05] 클램프 없는 원값 판독 — 위 히스토그램만으로는 127 초과 값을 구분할 수 없다.
    for (disp, last, orr, and) in [(0x200u32, &PB_S200_LAST, &PB_S200_OR, &PB_S200_AND),
                                   (0x400u32, &PB_S400_LAST, &PB_S400_OR, &PB_S400_AND)] {
        w.extend_from_slice(&[0x49, 0x8b, 0x81]);                     // mov rax,[r9+disp32]
        w.extend_from_slice(&disp.to_le_bytes());
        w.extend_from_slice(&[0x48, 0xb9]);                           // movabs rcx,&last
        w.extend_from_slice(&(last as *const _ as usize).to_le_bytes());
        w.extend_from_slice(&[0x48, 0x89, 0x01]);                     // mov [rcx],rax   (표본 1개면 충분)
        w.extend_from_slice(&[0x48, 0xb9]);                           // movabs rcx,&or
        w.extend_from_slice(&(orr as *const _ as usize).to_le_bytes());
        w.extend_from_slice(&[0xf0, 0x48, 0x09, 0x01]);               // lock or  [rcx],rax
        w.extend_from_slice(&[0x48, 0xb9]);                           // movabs rcx,&and
        w.extend_from_slice(&(and as *const _ as usize).to_le_bytes());
        w.extend_from_slice(&[0xf0, 0x48, 0x21, 0x01]);               // lock and [rcx],rax
    }
    w
}

/// HB 전용: `G.vt[0x30]()` 반환값(= 진입 시 `rax`) 분포 + 원값.
/// ⚠**work의 첫 명령이 rax를 건드리기 전에** 값을 rcx로 옮겨야 한다
///   (공용 스텁이 rax를 push해 두긴 하지만 그건 복원용이고, work 안에서는 즉시 덮인다).
fn pb_work_hb() -> Vec<u8> {
    let mut w: Vec<u8> = vec![0x48, 0x89, 0xc1];                      // mov rcx,rax  ← 반환값 확보
    for (cnt, op) in [(&PB_VT30_TOT, 0xffu8), (&PB_VT30_LAST, 0x89), (&PB_VT30_OR, 0x09)] {
        w.extend_from_slice(&[0x48, 0xb8]);                           // movabs rax,&cnt
        w.extend_from_slice(&(cnt as *const _ as usize).to_le_bytes());
        match op {
            0xff => w.extend_from_slice(&[0xf0, 0x48, 0xff, 0x00]),   // lock inc qword [rax]
            0x89 => w.extend_from_slice(&[0x48, 0x89, 0x08]),         // mov  [rax],rcx
            _    => w.extend_from_slice(&[0xf0, 0x48, 0x09, 0x08]),   // lock or [rax],rcx
        }
    }
    // 0..7 클램프 후 히스토그램 (분기 없이 cmova)
    w.extend_from_slice(&[0xb8, 0x07, 0x00, 0x00, 0x00]);             // mov eax,7
    w.extend_from_slice(&[0x48, 0x83, 0xf9, 0x07]);                   // cmp rcx,7
    w.extend_from_slice(&[0x48, 0x0f, 0x47, 0xc8]);                   // cmova rcx,rax  (7 초과 → 7칸)
    w.extend_from_slice(&[0x48, 0xb8]);                               // movabs rax,&PB_VT30
    w.extend_from_slice(&(&PB_VT30 as *const _ as usize).to_le_bytes());
    w.extend_from_slice(&[0xf0, 0x48, 0xff, 0x04, 0xc8]);             // lock inc qword [rax+rcx*8]
    w
}

/// HA 전용: 진입 시 `rcx` = StatDelta 포인터. 死 판정 필드가 비-0이면 마스크 비트를 켠다.
/// 분기 없이 `cmp`+`cmovne`로 비트를 만들어 `lock or` 한 번에 합친다(변위 계산 자체를 없앴다).
fn pb_work_ha() -> Vec<u8> {
    let mut w = pb_work_inc(&PB_BUFF_CALLS);
    // ⚠공용 스텁은 rax·rcx·플래그만 보존한다. 여기서 쓰는 rdx·r10·r11은 **함수 인자이거나 호출자 소유**라
    //   내가 직접 저장·복원해야 한다(rdx는 이 함수의 2번째 인자다 — 날리면 게임이 깨진다).
    w.extend_from_slice(&[0x52, 0x41, 0x52, 0x41, 0x53]);            // push rdx; push r10; push r11
    w.extend_from_slice(&[0x48, 0x85, 0xc9]);                        // test rcx,rcx
    let mut scan: Vec<u8> = Vec::new();
    scan.extend_from_slice(&[0x4d, 0x31, 0xdb]);                     // xor r11,r11   (누적 마스크)
    for (i, (off, _)) in PB_SD_OFFS.iter().enumerate() {
        scan.extend_from_slice(&[0x48, 0x8b, 0x81]);                 // mov rax,[rcx+off32]
        scan.extend_from_slice(&off.to_le_bytes());
        scan.extend_from_slice(&[0x48, 0x31, 0xd2]);                 // xor rdx,rdx
        scan.extend_from_slice(&[0x49, 0xc7, 0xc2]);                 // mov r10, 1<<i
        scan.extend_from_slice(&(1u32 << i).to_le_bytes());
        scan.extend_from_slice(&[0x48, 0x85, 0xc0]);                 // test rax,rax
        scan.extend_from_slice(&[0x49, 0x0f, 0x45, 0xd2]);           // cmovne rdx,r10
        scan.extend_from_slice(&[0x49, 0x09, 0xd3]);                 // or r11,rdx
    }
    scan.extend_from_slice(&[0x48, 0xb8]);                           // movabs rax,&PB_SD_MASK
    scan.extend_from_slice(&(&PB_SD_MASK as *const _ as usize).to_le_bytes());
    scan.extend_from_slice(&[0xf0, 0x4c, 0x09, 0x18]);               // lock or [rax],r11
    pb_jmp_over(&mut w, 0x74, &scan);                                // jz → 포인터 null이면 통째로 건너뜀
    w.extend_from_slice(&[0x41, 0x5b, 0x41, 0x5a, 0x5a]);            // pop r11; pop r10; pop rdx  (건너뛴 경로도 여기로 합류)
    w
}

/// H4 전용: 판단력 원시값 `[rbp+0x290]` 히스토그램(0..100만)
fn pb_work_h4() -> Vec<u8> {
    let mut tally: Vec<u8> = vec![0x48, 0xb9];                           // movabs rcx, &PB_JUDGE
    tally.extend_from_slice(&(&PB_JUDGE as *const _ as usize).to_le_bytes());
    tally.extend_from_slice(&[0xf0, 0x48, 0xff, 0x04, 0xc1]);            // lock inc qword [rcx+rax*8]

    let mut w: Vec<u8> = Vec::new();
    w.extend_from_slice(&[0x48, 0x8b, 0x85, 0x90, 0x02, 0x00, 0x00]);   // mov rax,[rbp+0x290]
    w.extend_from_slice(&[0x48, 0x83, 0xf8, 0x64]);                      // cmp rax,100
    pb_jmp_over(&mut w, 0x77, &tally);                                   // ja → 0..100 밖이면 버림(음수도 unsigned로 걸러짐)
    w
}

/// `probe` 토글 처리. 켜면 훅 4개 설치, 끄면 원본 복원.
unsafe fn apply_probe() {
    let want = if probe_on() { 1i64 } else { 0i64 };
    if PB_INSTALLED.load(Ordering::Relaxed) == want { return; }
    let base = exe_base();
    if base == 0 || READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return; }
    let sites: [(usize, usize, &[u8]); 11] = [
        (PB_H1_RVA, PB_H1_LEN, &PB_H1_ORIG),
        (PB_H2_RVA, PB_H2_LEN, &PB_H2_ORIG),
        (PB_H3_RVA, PB_H3_LEN, &PB_H3_ORIG),
        (PB_H4_RVA, PB_H4_LEN, &PB_H4_ORIG),
        (PB_H5_RVA, PB_H5_LEN, &PB_H5_ORIG),
        (PB_H6_RVA, PB_H6_LEN, &PB_H6_ORIG),
        (PB_H7_RVA, PB_H7_LEN, &PB_H7_ORIG),
        (PB_H8_RVA, PB_H8_LEN, &PB_PROLOG8),
        (PB_H9_RVA, PB_H9_LEN, &PB_PROLOG8),
        (PB_HA_RVA, PB_HA_LEN, &PB_PROLOG8B),
        (PB_HB_RVA, PB_HB_LEN, &PB_HB_ORIG),
    ];

    if want == 0 {
        if PB_INSTALLED.load(Ordering::Relaxed) == 1 {
            for (rva, _, orig) in sites.iter() { fs2_write(base + rva, orig); }
        }
        PB_INSTALLED.store(0, Ordering::Relaxed);
        if let Some(p) = pth("probe.txt") { let _ = fs::write(p, "probe=0 (계측 꺼짐 · 원본 복원)\n"); }
        return;
    }

    // ── 설치 전 4곳 원본 바이트 전수 대조. 하나라도 다르면 **아무것도 쓰지 않는다.** ──
    for (i, (rva, _, orig)) in sites.iter().enumerate() {
        if !fs2_bytes_eq(base + rva, orig) {
            PB_INSTALLED.store(0, Ordering::Relaxed);
            if let Some(p) = pth("probe.txt") {
                let _ = fs::write(p, format!(
                    "SKIP: H{} ({:#x}) 원본 바이트 불일치 — 아무것도 쓰지 않았다.\n\
                     (게임 패치로 주소가 옮겨졌거나 다른 모드가 먼저 건드린 상태)\n", i + 1, rva));
            }
            return;
        }
    }

    let works: [Vec<u8>; 11] = [
        pb_work_inc(&PB_SEEN), pb_work_h2(), pb_work_inc(&PB_FAR), pb_work_h4(),
        pb_work_inc(&PB_FARQ), pb_work_inc(&PB_ENTER), pb_work_inc(&PB_LATCH),
        pb_work_h8(), pb_work_inc(&PB_THINK_RUN), pb_work_ha(), pb_work_hb(),
    ];
    let mut ok = 0usize;
    for (i, (rva, len, orig)) in sites.iter().enumerate() {
        let addr = base + rva;
        let stub = pb_build_stub(i, &works[i], orig, addr + len);
        if stub == 0 { continue; }
        // ★패치 폼은 **슬롯 길이에 따라** 고른다. 옛 코드는 14B 폼을 무조건 써서
        //   12B 슬롯(H8·H9)에서 `patch[6..14]`가 범위를 벗어나 **패닉**했다(2026-08-04 크래시).
        let mut patch = vec![0x90u8; *len];
        if *len >= 14 {
            // 레지스터 무파괴: jmp qword [rip+0] + .quad stub
            patch[0] = 0xff; patch[1] = 0x25;
            patch[2..6].copy_from_slice(&0u32.to_le_bytes());
            patch[6..14].copy_from_slice(&stub.to_le_bytes());
        } else if *len >= 12 {
            // 12B: movabs rax,stub ; jmp rax — rax를 파괴하지만 **함수 진입부에선 rax가 dead**라 안전
            //   (win64 인자는 rcx/rdx/r8/r9이고 rax로 넘어오는 것은 없다.)
            patch[0] = 0x48; patch[1] = 0xb8;
            patch[2..10].copy_from_slice(&stub.to_le_bytes());
            patch[10] = 0xff; patch[11] = 0xe0;
        } else {
            continue;   // 12B 미만은 어떤 절대점프도 안 들어간다
        }
        if fs2_write(addr, &patch) { ok += 1; }
    }
    PB_INSTALLED.store(if ok == 11 { 1 } else { 0 }, Ordering::Relaxed);
    if let Some(p) = pth("probe.txt") {
        let _ = fs::write(p, format!("probe=1 설치 {}/11 @base{:#x} — 경기를 한 판 돌린 뒤 이 파일을 확인하세요.\n", ok, base));
    }
}

/// post_update(단일 스레드)에서 주기적으로 부르는 스냅샷. **여기서만 파일을 쓴다.**
unsafe fn probe_snapshot() {
    if PB_INSTALLED.load(Ordering::Relaxed) != 1 { return; }
    let now = READY_TICKS.load(Ordering::Relaxed);
    let last = PB_LAST_SNAP.load(Ordering::Relaxed);
    if now < last + 3600 { return; }        // 약 60초마다 1줄 (60fps 기준)
    PB_LAST_SNAP.store(now, Ordering::Relaxed);

    let seen  = PB_SEEN.load(Ordering::Relaxed);
    let bush  = PB_BUSH.load(Ordering::Relaxed);
    let far   = PB_FAR.load(Ordering::Relaxed) + PB_FARQ.load(Ordering::Relaxed);
    let enter = PB_ENTER.load(Ordering::Relaxed);
    let latch = PB_LATCH.load(Ordering::Relaxed);
    let ph0 = far + latch;          // Phase 0 진입 = 랜드마크 멂(P+Q) + 래치 전이
    let mut s = String::new();
    s.push_str(&format!("\n════ t{} ════\n", now));

    // ── ①왕복: Phase 0(랜드마크 우회) vs Phase 1(부쉬) 체류 ──
    s.push_str("[1] 매복 왕복 — 랜드마크 우회 vs 부쉬\n");
    if enter == 0 {
        s.push_str("  아직 hide(매복) 판단이 한 번도 안 떴습니다. 정글 전술을 '라인 개입'으로 두고 경기를 돌리세요.\n");
    } else {
        let ph1 = seen + bush;
        s.push_str(&format!("  hide 총 진입           {}\n", enter));
        s.push_str(&format!("  Phase 0 (랜드마크로)   {}  ({:.1}%)   ← 부쉬가 아닌 곳으로 걸어가는 중\n",
            ph0, 100.0 * ph0 as f64 / enter as f64));
        s.push_str(&format!("  Phase 1 (부쉬)         {}  ({:.1}%)\n",
            ph1, 100.0 * ph1 as f64 / enter as f64));
        s.push_str(&format!("  └ 그중 적이 나를 봄    {}  ({:.1}%)\n",
            seen, 100.0 * seen as f64 / ph1.max(1) as f64));
        s.push_str(&format!("  ★래치 전이(랜드마크 도착) {}\n", latch));
        if ph0 > 0 {
            s.push_str(&format!("  ★래치율 = 전이/Phase0 = {:.2}%  (1회 도착까지 평균 {:.0}프레임)\n",
                100.0 * latch as f64 / ph0 as f64,
                if latch > 0 { ph0 as f64 / latch as f64 } else { 0.0 }));
        }
        s.push_str("  판독: 래치 전이가 **잦으면** = 플랜이 자주 새로 만들어져 phase가 계속 0으로 리셋되는 것\n\
                    \x20      (이 경우 게이트만 올려선 부족하고 생성자 쪽을 손대야 함).\n\
                    \x20      래치 전이가 **드물면** = 랜드마크가 그냥 멀어서 오래 걷는 것\n\
                    \x20      (이 경우 게이트를 올려 Phase 0을 없애면 바로 해결됨).\n");
    }

    // ── ⑦AI 판단 스로틀 (4~12틱 주기) ──
    let (tg, tr) = (PB_THINK_GATE.load(Ordering::Relaxed), PB_THINK_RUN.load(Ordering::Relaxed));
    s.push_str("[7] AI 판단 주기\n");
    if tg == 0 { s.push_str("  관측 0건\n"); }
    else {
        s.push_str(&format!("  판단 요청 {} / 실제 판단 {}  → 실행률 {:.1}% (평균 {:.1}틱마다 1회)\n",
            tg, tr, 100.0 * tr as f64 / tg as f64,
            if tr > 0 { tg as f64 / tr as f64 } else { 0.0 }));
        s.push_str("  판독: 평균 주기가 4~12틱 사이면 게임 설계대로입니다.\n");
        // r 입력 2종의 분포 — 한 점에 몰리면 게임 옵션, 퍼지면 선수 능력치
        for (nm, hist) in [("[+0x200]", &PB_S200), ("[+0x400]", &PB_S400)] {
            let mut tot = 0u64; let mut distinct = 0usize;
            let (mut lo, mut hi) = (127usize, 0usize); let mut top = (0usize, 0u64);
            for (v, a) in hist.iter().enumerate() {
                let c = a.load(Ordering::Relaxed);
                if c > 0 {
                    tot += c; distinct += 1;
                    if v < lo { lo = v; } if v > hi { hi = v; }
                    if c > top.1 { top = (v, c); }
                }
            }
            if tot == 0 { s.push_str(&format!("  {} 관측 0건\n", nm)); continue; }
            let share = 100.0 * top.1 as f64 / tot as f64;
            s.push_str(&format!("  {} 서로 다른 값 {}종, 범위 {}~{}{}, 최빈값 {}({:.1}%)\n",
                nm, distinct, lo, hi, if hi >= 127 { "(127=범위밖)" } else { "" }, top.0, share));
        }
        s.push_str("  ★판독: 두 값이 **한 종류로 몰리면 게임 옵션**(= 옵션이 AI 판단 빈도를 바꾼다),\n\
                    \x20      **여러 값으로 퍼지면 선수 능력치**(= 잘하는 선수가 더 자주 판단한다).\n");
        // ★[08-05] 위 히스토그램은 127에서 클램프하므로 큰 값을 구분 못 한다. 원값을 따로 낸다.
        s.push_str("  ── 클램프 없는 원값 ──\n");
        for (nm, last, orr, and) in [("[+0x200]", &PB_S200_LAST, &PB_S200_OR, &PB_S200_AND),
                                     ("[+0x400]", &PB_S400_LAST, &PB_S400_OR, &PB_S400_AND)] {
            let (l, o, a) = (last.load(Ordering::Relaxed), orr.load(Ordering::Relaxed), and.load(Ordering::Relaxed));
            if o == 0 && a == u64::MAX { s.push_str(&format!("  {} 관측 0건\n", nm)); continue; }
            s.push_str(&format!("  {} 마지막값 {} ({:#x}) · OR {:#x} · AND {:#x} → {}\n",
                nm, l, l, o, a,
                if o == a { "**경기 내내 단일 상수**(= 게임 옵션·전역 설정)" }
                else { "표본마다 다름(= 선수 능력치)" }));
        }
        s.push_str("  ★`p4[0x400]`은 그동안 **1000으로 추정**만 하고 있었습니다 — 위 '마지막값'이 그 실측치입니다.\n");
    }

    // ── ⑨층① 플랜 결정기의 최상위 3-way 스위치 ──
    let vt = PB_VT30_TOT.load(Ordering::Relaxed);
    s.push_str("[9] 플랜 결정기 최상위 스위치 (G.vt[0x30] 반환값)\n");
    if vt == 0 { s.push_str("  관측 0건 (0xd48ec0이 한 번도 안 돌았습니다 — plan 4·5·6이 안 나온 경기)\n"); }
    else {
        s.push_str(&format!("  호출 {}회, 마지막값 {} · OR {:#x}\n",
            vt, PB_VT30_LAST.load(Ordering::Relaxed), PB_VT30_OR.load(Ordering::Relaxed)));
        let mut distinct = 0usize;
        for (v, a) in PB_VT30.iter().enumerate() {
            let c = a.load(Ordering::Relaxed);
            if c == 0 { continue; }
            distinct += 1;
            let plan = match v { 2 => "→ plan 6", 1 => "→ plan 4·5", 7 => "(7 이상)", _ => "→ plan 7" };
            s.push_str(&format!("     값 {} : {:>8}회 ({:.1}%)  {}\n",
                v, c, 100.0 * c as f64 / vt as f64, plan));
        }
        s.push_str(&format!("  서로 다른 값 {}종.\n", distinct));
        s.push_str("  ★판독: **2종뿐이면 불리언성 상태**(예: 교전중/아님) · **3종 고정이면 열거형**(경기 페이즈 등)\n\
                    \x20      · **넓게 퍼지면 카운트나 핸들**이고 `==1`/`==2` 비교는 특수값 판정입니다.\n\
                    \x20      값 7 이상이 잡히면 히스토그램 상한을 늘려 다시 재야 합니다.\n");
    }

    // ── ⑧버프 가치 평가기의 死항 실측 ──
    let bc = PB_BUFF_CALLS.load(Ordering::Relaxed);
    s.push_str("[8] 버프 가치 평가 — 死 판정 필드 실측\n");
    if bc == 0 { s.push_str("  관측 0건 (버프/힐 스킬이 한 번도 평가되지 않았습니다)\n"); }
    else {
        let m = PB_SD_MASK.load(Ordering::Relaxed);
        s.push_str(&format!("  평가 {}회, 비-0이 한 번이라도 나온 필드:\n", bc));
        let mut any = false;
        for (i, (off, nm)) in PB_SD_OFFS.iter().enumerate() {
            let live = m & (1u64 << i) != 0;
            if live { any = true; }
            s.push_str(&format!("     +{:#05x} {:<16} {}\n", off, nm, if live { "★살아있음" } else { "0 (死)" }));
        }
        s.push_str(if any {
            "  ★판독: 살아있는 필드가 있습니다 ⟹ 死 판정은 **특정 생산 경로 한정**이었습니다.\n"
        } else {
            "  ★판독: 전부 0 ⟹ 死 판정이 실측으로 확정됩니다(이 필드들을 튜닝해도 효과가 없습니다).\n"
        });
    }

    // ── ⑤outline_type 死 판정 ──
    let ol: Vec<u64> = PB_OUTLINE.iter().map(|a| a.load(Ordering::Relaxed)).collect();
    s.push_str(&format!("[5] out_line 값 분포: 0={} 1={} 2={} 3={}  (1 말고 다른 값이 0이면 死 값 확정)\n",
        ol[0], ol[1], ol[2], ol[3]));

    // ── ⑥판단력 원시값 ──
    let mut jt = 0u64; let (mut jmin, mut jmax) = (255usize, 0usize); let mut jsum = 0u64;
    for (v, a) in PB_JUDGE.iter().enumerate() {
        let c = a.load(Ordering::Relaxed);
        if c > 0 { jt += c; jsum += c * v as u64; if v < jmin { jmin = v; } if v > jmax { jmax = v; } }
    }
    if jt == 0 { s.push_str("[6] 자리선택 밴드 관측 0건\n"); }
    else {
        s.push_str(&format!("[6] 자리선택에 들어간 '판단력' 원시값: 관측 {}건, 범위 {}~{}, 평균 {}\n",
            jt, jmin, jmax, jsum / jt));
        s.push_str("     판독: 범위가 로스터의 판단력 스탯과 겹치면 이 값이 판단력이 맞습니다.\n");
    }

    // ── ②apply_input 입구 게이트 ──
    let (t2, m2) = (PB_E2F0_TOT.load(Ordering::Relaxed), PB_E2F0_MIN.load(Ordering::Relaxed));
    if t2 == 0 { s.push_str("[2] entity+0x2F0 관측 0건\n"); }
    else {
        s.push_str(&format!("[2] apply_input 입구 게이트: 관측 {} / i64::MIN {} ({:.1}%) · 마지막값 {:#x}\n",
            t2, m2, 100.0 * m2 as f64 / t2 as f64, PB_E2F0_LAST.load(Ordering::Relaxed)));
        s.push_str("     판독: i64::MIN 비율이 0%에 가까우면 이 게이트는 사실상 '항상 통과'가 아니라 실제로 거르고 있다는 뜻.\n");
    }

    // ── ③라인 통제 인덱스 (베이스 = snap, u64) ──
    //    ⚠snap은 AI 판단 스택 프레임 위의 임시 객체다. 여기(post_update)서 읽는 값은
    //      "마지막 판단 시점의 잔상"이라 0~6을 벗어나면 그 프레임이 이미 사라진 것이다.
    let snap = PB_SNAP.load(Ordering::Relaxed);
    if !ptr_ok(snap) { s.push_str("[3] snap 미확보 — 경기 중에 다시 확인하세요.\n"); }
    else {
        s.push_str("[3] 라인 통제 인덱스 (0=내 진영 끝 … 6=상대 진영 끝) — lane 0=탑 1=미드 2=봇\n");
        let mut sane = true;
        for lane in 0..3usize {
            let a = rd_u64(snap + 0x21c0 + lane * 0x10).unwrap_or(u64::MAX);
            let b = rd_u64(snap + 0x21c0 + lane * 0x10 + 8).unwrap_or(u64::MAX);
            if a > 6 || b > 6 { sane = false; }
            s.push_str(&format!("     lane{} : side0={} side1={}\n", lane, a, b));
        }
        if !sane {
            s.push_str("     ⚠값이 0~6을 벗어났습니다 = 이 시점엔 스냅샷 프레임이 이미 사라진 것입니다\n\
                        \x20     (구조는 확정됐으니 값이 필요하면 판단 훅 안에서 스냅해야 합니다).\n");
        }
        // 27 랜드마크 통제 점수(i32) — 인덱스의 원재료. +면 side0 우세
        let mut lm = String::new();
        for i in 0..27usize {
            let v = if readable(snap + 0x2218 + i * 4, 4) {
                std::ptr::read_unaligned((snap + 0x2218 + i * 4) as *const i32)
            } else { 0 };
            lm.push_str(&format!("{}{}", if i % 9 == 0 { "\n     " } else { " " }, v));
        }
        s.push_str(&format!("     27 랜드마크 통제점수(+면 side0 우세, 임계 ±3):{}\n", lm));
    }

    // ── ④랜드마크 좌표 (라인 노드 = 포탑 위치) ──
    //    ✅정적으로 라벨링 완료 — 런타임에서는 표가 실제로 그 값인지만 대조한다.
    let map = PB_MAP.load(Ordering::Relaxed);
    if ptr_ok(map) {
        const LINE_NODES: [(usize, &str); 6] = [
            (5, "탑 L0.n1(내측)"), (22, "탑 L0.n2(외측)"),
            (19, "미드 L1.n1"),    (4,  "미드 L1.n2"),
            (9, "봇 L2.n1"),       (8,  "봇 L2.n2"),
        ];
        s.push_str("[4] 랜드마크 좌표 대조 (side0 기준 라인 노드 = 포탑 위치)\n");
        for (i, name) in LINE_NODES.iter() {
            let x = rd_u64(map + 0x6ba0 + i * 0x10).unwrap_or(0);
            let y = rd_u64(map + 0x6ba0 + i * 0x10 + 8).unwrap_or(0);
            s.push_str(&format!("     lm{:<2} {:<16} = ({}, {})\n", i, name, x, y));
        }
    }
    if let Some(p) = pth("probe.txt") {
        use std::io::Write as _;
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) {
            let _ = f.write_all(s.as_bytes());
        }
    }
}

#[inline] unsafe fn rd_u32_opt(a: usize) -> Option<u32> {
    if readable(a, 4) { Some(std::ptr::read_unaligned(a as *const u32)) } else { None }
}
