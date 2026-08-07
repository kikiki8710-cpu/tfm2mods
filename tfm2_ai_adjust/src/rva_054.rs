// rva_054.rs — 전 RVA/ORIG_LEN 상수 (게임 0.5.4 기준). ★패치 대응시 원칙적으로 이 파일만 수정 + MIGRATION §7 갱신.
// 들어있는 것: RVA_* 훅/함수/콜사이트 주소, ORIG_LEN_* 프롤로그 경계 길이. image base=0x140000000, RVA=abs-base.
//
// ★★[0.5.4 마이그 상태 = 훅 전량 재핀완 (2026-08-05)] ★★
//   0.5.3→0.5.4 = **AI 대개편**. 함수 132,960→134,556(+1,596), .text +840KB.
//   실체는 판단 트리 변경이 아니라 **경로/거리 계산 하위 시스템 신설**(`free_dist.rs`·`path_field.rs`·
//   `minion_wave_risk.rs` 신규 + `path_finder.rs` 47→122함수). 판단 트리 골격(plan 16 arm / sub_plan 19 arm →
//   핸들러 소스 매핑)은 **1:1 그대로**다.
//
//   ⚠그러나 계약은 크게 깨졌다 — 아래는 이 파일 밖(구조체 오프셋·enum)에서 고쳐야 하는 것들:
//     · **Plan enum 번호가 −2 시프트**(Battle 9→7, DefenseNexus 17→15, AttackNexus 16→14, DeathMatchBattle 6→4)
//       SubPlan 번호는 **불변**. 디스패처 인덱스식에서 바이어스가 빠졌다(`disc>=2?disc-2:1` → `disc`).
//     · **Order/SmallAction stride 0xA8 → 0xB8**, 태그 오프셋 **+0xA1 → +0xB1**
//     · 엔티티 레코드 stride 0xa8 → 0xb8 / plan 필드 +0x598 → +0x5e8 / sub_plan +0x6b0 → +0x708
//     · team +0x820 → +0x810, role +0x8b0 → +0x8a0, 판단력 +0x3f8 → +0x3f0
//
//   ✅검증 방식 = exe↔exe capstone 실측(도구 `C:\tfm2mods\v54\`):
//      · 짝짓기 = 패닉 `Location{file,line,col}` 소스 앵커(컴파일러가 심어 버전 무관) + 명령 골격 정렬
//      · 훅 검증 = ①프롤로그 선두 N바이트 ②orig_len 명령경계 정확 ③트램폴린 구간 rip-rel/분기 부재 (`hooks.py`)
//      · 바이트패치 = 사이트 569개 전부 0.5.4 실바이트와 대조 통과 (`check054.py`)
//   ⚠**상수값으로 짝짓지 않았다.** 값만 맞춰 고르면 다른 자리를 잡고 "설정값이 죽었다"고 오판한다.
//
//   ⏸stale 유지 = **고의 보류(종전부터 inert)**: TG_CALL / THREATGATE_FN / F2_BUILD_CALL / GB_REGIOND_HOOK /
//      GB_FUNNEL / COMMIT_CALL / COMMIT_FN / ENGAGE_GATE. 전부 신원검증(target-guard·프롤로그·E8 체크)에
//      걸려 **미설치=inert**이므로 방치가 fail-safe.

const RVA_RETREAT:  usize = 0xe625e0;   // ★0.5.4(was 0.5.3 0xe00350). retreat_engage(`old\passive_jungle.rs`).
//   프롤로그 12B `55 41 57 41 56 41 55 41 54 56 57 53`(push8) 바이트 완전동일·orig_len 12 경계정확·rip-rel/분기 無.
//   13B째부터 `sub rsp,0x318`→`0x358`(프레임만 커짐, 트램폴린 12B 밖이라 무관).
//   ⚠이 함수 안의 `eng_role*` 4임계(100/70/50/30)는 살아있지만 **레지스터가 r12d→ecx** 로 바뀌어
//     imm 오프셋이 +2→+1 이다(detour.rs 해당 사이트는 재작성 반영완).

const RVA_TG_CALL: usize = 0x1feca43;        // ⏸stale 유지(0.5.0 이후 계속 실패=inert). threatgate가 gb 내부 임베드=독립콜사이트 없음. install_threatgate_hook이 "not a CALL(E8)"서 bail=안전 inert. 재RE 전 유지

const RVA_THREATGATE_FN: usize = 0x20a8680;  // ⏸stale 유지=inert. gb 내부 mid-block 임베드=독립함수/콜사이트 없음. TG_CALL guard(E8+target)로 미설치=안전. 재RE 필요

// ★plan_lane_predicate = my_lane_predicate로 순수재현 완료(2026-06-19, DIFF=0) → RVA_LANE_PRED 제거(churn 소멸).
// facet#2 position: driver 내 generic_build(이동좌표 최종화) 호출지점. 직후 rcx=최종 Move{tag,x,y}.
const RVA_F2_BUILD_CALL: usize = 0x22dd4fe;  // ⏸stale 유지=inert(스왑 금지). 실값으로 스왑하면 target-guard 통과→move-post 훅 설치=facet#2 재활성(미완·GB 메인빌드 전체재현 폐기 상태).

const RVA_GENERIC_BUILD: usize = 0xdca240;  // ★0.5.4(was 0.5.3 0xe06c10). `old\battle.rs` 최대함수 entry.
//   프롤로그 12B push8 완전동일·orig_len 12 경계정확·rip-rel無. 함수크기 22,764 → **29,389B(+6,625)**.
//   ⚠골격 일치 67% = 본문이 실제로 많이 바뀌었다(fight_model 호출이 1종×2회 → 4종×2회). body detour는
//     INSTALL_DIAG_HOOKS=false라 미설치·move-post는 F2_BUILD_CALL target-guard 경유 inert 이므로 당장 영향 없음.

// ★facet#2 레인워크 waypoint 선택. 프롤로그 12B(8 push).
const RVA_FC59A0: usize = 0xe79c00;  // ★0.5.4(was 0.5.3 0xe168d0). recall_rng_score.
//   크기 1459→1459 **동일**, 프롤로그 20B **바이트 완전동일**, 골격 100%. install_replace_detour_rax 무조건설치 경로라 최우선 검증 대상이었다.

// ★pre-gate 상수 테이블(.rdata, RVA). p1(lane)<4만 사용. tableA[0..4]=[0,1,3,2](인덱스 변환).
const RVA_TABLE_A: usize = 0x327aa60;  // ★0.5.4(was 0.5.3 0x31c0168). .rdata 값지문 매칭: 선두 32B(=[0,1,3,2] u64 4엔트리)
//   0.5.3 3건 ↔ 0.5.4 3건(0x327aa60/0x328d0e8/0x32cdaf8) → 주소순 대응 #1. ⚠3건 모두 앞 4엔트리가 같고
//   소스는 p1<4만 읽으므로 어느 것이든 기능 등가(0.5.3에서도 같은 상황이었다).

// ── 영역 D 출력검증(gb_region_d): mid-func 캡처 detour 지점 ── ⏸SKIP 유지
const RVA_GB_REGIOND_HOOK: usize = 0x22dafea;  // ⏸SKIP 유지(=차단). 값 무효. MIG_GB_CHANGED=true로 SKIP.
//   ⚠0.5.3에서 재핀했던 0xe09f3e/0xe08654 도 **0.5.4에선 무효**(generic_build 가 0xdca240 으로 이동+본문 대폭 변경).
//   재활성하려면 0xdca240 안에서 영역 D 진입·funnel 을 처음부터 다시 찾아야 한다.

const ORIG_LEN_GB_REGIOND: usize = 14;         // (REGIOND_HOOK이 SKIP이라 미사용)

const RVA_GB_FUNNEL: usize = 0x22dbc4e;           // ⏸SKIP 유지=inert. 값 무효(0.5.3 재핀값 0xe085fc 도 0.5.4에선 무효).

// facet#1 condgate(목표커밋 bool). 프롤로그 push3+sub0x40+mov = 15B 클린. rcx=subplan_ctx(*=disc), r9=reg
const RVA_CONDGATE: usize = 0xe13ca0;   // ★0.5.4(was 0.5.3 0xc550b0).
//   프롤로그 20B `56 57 53 48 83 ec 40 4c 8b 9c 24 90 00 00 00 4c 8b 94 24 80` **바이트 완전동일**·
//   orig_len 15 경계정확(경계 [0,1,2,3,7,15,23…])·rip-rel無 ⟹ 최고신뢰. 크기 510→497, 골격 97%.

const RVA_MOVEPRI: usize = 0xe145b0;   // ★0.5.4(was 0.5.3 0xc559e0). **Plan 디스패처**(SubPlan 아님 — 아래 주의).
//   프롤로그 20B `41 56 56 57 53 48 83 ec 48 48 89 ce …` **바이트 완전동일**, 크기 969→969, 골격 99%.
//   orig_len **12** 유지(경계 [0,2,3,4,5,9,12,20] — 0.5.3과 동일 배열).
//   ★★**인덱스식이 바뀌었다**: 0.5.3 `idx = disc>=2 ? disc-2 : 1` → 0.5.4 **`idx = disc`**(바이어스 제거).
//     ⟹ **Plan 번호가 전부 −2 시프트**. mp_capture 가 disc 값을 해석해 쓰고 있다면 그쪽도 같이 고쳐야 한다.
//     16 arm 의 이름 배열(ForcePassive…DefenseNexus)은 순서·내용 완전 동일 = 변형이 사라진 게 아니라 번호만 밀렸다.
//   JT 베이스 0.5.3 0x31A7494 → 0.5.4 **0x3286AC4**(16엔트리).

// ★cVar6==0 STAND vs roll 게이트 → my_fa1ea0(순수재현, 288/288 DIFF0)로 완전대체 → RVA_FA1EA0 제거(2026-06-19).
const RVA_COMMIT_CALL: usize = 0x1e3dfd2;     // ⏸stale 유지=inert. target-guard 자체보호.

const RVA_COMMIT_FN: usize = 0x235ffa0;        // ⏸stale 유지=inert. 재핀 불가(함수가 너무 작아 후보 0). target-guard로 inert(안전).

// 페이즈 게이트 threshold = objective*9 + min(B,100)*2 + BASE(=100). imm8(베이스) 패치 = 교전 공격성 다이얼.
const RVA_ENGAGE_GATE: usize = 0x1c9b33d;     // ⏸stale 유지=inert(0.5.0부터). apply_engage_base가 83C0 sanity서 return=안전 inert. 재RE 필요


// ════ disc18/19(진짜 넥서스 AttackNexus/DefenseNexus) 완전재현 Phase2-1: 캡처 훅 ════
const RVA_DISC18_HANDLER: usize = 0xda1850;   // ★0.5.4(was 0.5.3 0xd94d00). `sub_plan\attack_nexus.rs`.
//   프롤로그 12B push8 완전동일·orig_len 12 경계정확·rip-rel無. 크기 5,923→5,891, 골격 99%.
//   ★HARNESS_ON 하에 install_wrap **무조건 설치** 경로 = 구값 방치 시 오후킹이라 최우선 반영 대상이었다.

// ★SubPlan 디스패처(= disc18/19 핸들러를 호출하는 그 함수).
//   ⚠**2계층 주의**: `RVA_MOVEPRI`는 **Plan** 디스패처이고 이건 **SubPlan** 디스패처다. 번호공간이 다르다.
//     그리고 0.5.4에서 **Plan 번호만 −2 밀렸고 SubPlan 번호는 그대로**다 — 둘을 같이 밀면 안 된다.
const RVA_SUBPLAN_DISPATCH: usize = 0xe52990;   // ★0.5.4(was 0.5.3 0xd98740).
//   프롤로그 24B `55 41 57 41 56 41 55 41 54 56 57 53 48 81 ec 88 00 00 00 48 8d ac 24 80` **바이트 완전동일**.
//   크기 1278→1310. 인덱스식 `idx=(disc>=2)?disc-2:7` **불변**, JT 19엔트리 **불변**, arm→핸들러 소스 매핑 **1:1 동일**.
//   JT 베이스 0.5.3 0x31BA310 → 0.5.4 **0x3288D1C**.

const RVA_DISC19_HANDLER: usize = 0xdac090;   // ★0.5.4(was 0.5.3 0xdece30). `sub_plan\defense_nexus.rs`.
//   프롤로그 12B push8 완전동일·orig_len 12 경계정확. 크기 8,872→11,221(+2,349), 골격 80%.
//   **교차확증 = DISC7 desc 를 참조하는 `lea r9,[rip]` 가 0.5.3과 똑같이 이 함수 내부 유일 사이트로 재발견**
//   (0.5.3 @0xdeeebe → 0x31bcef8  ↔  0.5.4 @0xdae76a → 0x327fba0).


// ══ itemnet 빌드 스코어러 NULL-모델 가드 (튜토리얼/에이전트부재 AV 원천차단) ══
const RVA_ITEMNET_SCORER: usize = 0x145a680;   // ★0.5.4(was 0.5.3 0x10587e0).
//   프롤로그 20B **바이트 완전동일**, 크기 1,609→1,609, 골격 100% ⟹ 사실상 무변경 함수.
//   ★설치 위치 = fn+12(push8 프롤로그 직후)의 15B 검증 후 변위 — 그 15B도 동일.

const RVA_C8C_DMG_SHEET: usize = 0x3288e48;   // ★0.5.4(was 0.5.3 0x31be1a8). 공격측 데미지시트 desc.
//   재핀 경로 = 0.5.3과 동일: `old\passive_jungle.rs`(053 0xdff660 → 054 0xe619c0) 안의 `lea r9,[rip]` **2사이트가 둘 다 같은 값**.
//   sanity 통과: {size=**0x6a8**, align=**8**, drop=0x140e5cbd0} + **vt+0x30 = 0xc7ead0**(0.5.3은 0xc51bc0).
//   ⚠이 값이 stale이면 임의 바이트를 vtable로 삼아 호출 → AV. **0.5.2 disc14 크래시 2·3차의 진범이 이 상수 방치**였다.
//     probe_basedmg_r9 의 화이트리스트(OK_DESC)도 반드시 같이 갱신할 것(→ tfm2_ai_adjust.rs 반영완).

// disc7 위협 엔티티 e가 self(se)를 때리는 DPS(실데미지×1000/공속).
const RVA_DISC7_DMG_SHEET: usize = 0x327fba0;   // ★0.5.4(was 0.5.3 0x31bcef8). "받는 데미지" 시트 desc.
//   재핀 경로 = **0.5.3과 완전히 동일한 xref 경로 재현**: disc19 핸들러 내부의 `lea r9,[rip]` 가 함수 내 유일 사이트.
//   sanity 통과: {size=0x6a8, align=8, drop=0x140d9b590, vt+0x30=0xc7ead0}.

// ★[0.5.4 프로브] 경매(전술 입찰) 진입 — `TeamPlan.version` 이 **3번째 인자(r8)** 로 들어온다.
//   version 은 0.5.4 신규 필드이고 `>=2` 게이트가 exe 전역 8곳(경매 강제귀환·점수식 넥서스 게이트 등)을
//   여닫는데 **정적으로는 값을 못 밝혔다**(팩토리 e8c020 이 정적 호출 0건).
//   선두 12B = disc18(da1850)과 바이트 완전 동일한 push8, orig_len 12 경계정확, rip-rel/분기 無.
//   ⚠passthrough 프로브 전용 — 원본을 항상 호출한다(기능 무영향).
const RVA_AUCTION: usize = 0xeacf10;
