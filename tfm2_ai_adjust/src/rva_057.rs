// rva_057.rs — 전 RVA/ORIG_LEN 상수 (게임 0.5.7 기준). ★패치 대응시 원칙적으로 이 파일만 수정 + MIGRATION §7 갱신.
//    (구판 = rva_056.rs 보존)
//
// ★★[0.5.7 재핀 (2026-08-26)] ★★  판정 도구 = C:	fm2mods\_mig057.py + _t057i.py(콜러그래프) + _t057j.py(LCS)
//   · 0.5.6→0.5.7 = 전면 재링크(.text는 −4KB로 거의 무변화이나 RVA 제자리 725 / 이동 37,001).
//   · ★**활성 훅 10 중 5종이 본문 변경**(RVA-only 아님) — RETREAT(size 11030→10995) · CONDGATE(513→522) ·
//     SUBPLAN(1310→1278) · DISC18(5891→5934) · DISC19(11221→11292). 패치노트 "일부 챔피언 스킬 효과가
//     선수 AI 스킬 사용 판단에 미반영되던 문제 수정"(흡혈귀·성직자·얼음술사)의 실체로 **추정**.
//   · 나머지 5(GENERIC_BUILD·FC59A0·MOVEPRI·AUCTION·ITEMNET)는 RVA-only 이동(BYTE=SAME).
//   · 재핀 근거: RETREAT=콜러그래프 슬롯정렬(호스트 0xe23f00→0xdb2b90) · CONDGATE=LCS 콜슬롯 정렬
//     (호스트 0xdf4ce0→0xe723c0의 2사이트 모두 동일 결과 + MOVEPRI와의 간격 0x940 보존) ·
//     DISC18/19=마스크시그와 콜러그래프 2방법 일치 · SUBPLAN=HEAD_UNIQUE.
//   · ★CONDGATE 실변경(디스어셈 1:1 대조) = 레지스터 재할당(rsi↔rdi) + 인자 로드 프롤로그 호이스팅
//     + disc 분기 1곳에 `add rcx,8`와 스택 인자 1개 추가. **구조체 오프셋·JT 골격은 불변** = 국소 변경.
//     신 프롤로그 = 7B(3push+sub) + `mov rsi,[rsp+0x98]`(8B) = **정확히 15B 경계** ⟹ orig_len 15 유효.
//
//   ⬜**미재핀(이번 회차 범위 밖)**: 바이트패치 908엔트리(orig_table.rs) · JT 베이스(Plan/SubPlan) ·
//      class_micro 18사이트 · nx_imm · bt_vision · 재현 코드(완전재구현) 정합성 검토.
//      ⟹ 이 파일만 교체한 빌드는 **훅 10만 신주소**이고 바이트패치·재현 축은 0.5.6 기준이다.
//
// rva_056.rs — 전 RVA/ORIG_LEN 상수 (게임 0.5.6 기준). ★패치 대응시 원칙적으로 이 파일만 수정 + MIGRATION §7 갱신.
// 들어있는 것: RVA_* 훅/함수/콜사이트 주소. image base=0x140000000, RVA=abs-base. (구판 = rva_055.rs 보존)
//
// ★★[0.5.6 재핀 완료 (2026-08-20) — version-migrator 2단계] ★★
//   0.5.5→0.5.6 = **순수 재링크 핫픽스(ai_adjust 관점)**. 판정 도구 = C:\tfm2mods\_mig056.py(스켈레톤해시)
//   + mig056_ai*.py(오너-델타 전수 재핀·바이트검증).
//   · 활성 훅 10 전건 UNIQUE·마스킹 바이트 완전동일(BYTE=SAME)·size 동일 ⟹ RVA-only 이동. 저대역 델타 +0x9dd0.
//   · 프롤로그 전건 동일 ⟹ **orig_len 전부 0.5.5값 유지**(MOVEPRI 14 포함 — 변경 없음).
//   · Plan/SubPlan 계약 불변: MOVEPRI idx=(disc>=2)?disc-2:4 · SUBPLAN idx=(disc>=2)?disc-2:8 바이트동일,
//     JT arm 오프셋 16/16·19/19 일치, 핸들러 콜 타깃 15+22 전건 BYTE=SAME. **Plan enum 번호 이동 없음.**
//   · 구조체 오프셋 시프트 없음(§7.6 저대역 전면 불변) ⟹ 0.5.5 2단계 같은 오프셋 작업 불요.
//   · 바이트패치 사이트 호스트 80종 중 본문변경 1건(0xdeaf10→0xdf4ce0, ratio 0.9998 — shr 상수 2명령 국소,
//     tm_cancel 사이트 자체는 보존 @0xdf8a6a) 외 전건 BYTE=SAME.
//   JT 베이스(참고): Plan 0.5.5 0x336637C → 0.5.6 **0x338128c** / SubPlan 0x3366E5C → **0x3381d6c**.
//
//   ⏸stale 유지 = **고의 보류(종전부터 inert, 0.5.4 주소 그대로)**: TG_CALL / THREATGATE_FN / F2_BUILD_CALL /
//      GB_REGIOND_HOOK / GB_FUNNEL / COMMIT_CALL / COMMIT_FN / ENGAGE_GATE. 전부 신원검증(target-guard·E8·프롤로그)에
//      걸려 미설치=inert 이므로 방치가 fail-safe. 0.5.6 재핀 시도 안 함(rva_055 값 그대로).

const RVA_RETREAT:  usize = 0xd2f180;   // ★0.5.6(was 0.5.5 0xce0f70). retreat_engage. UNIQUE·size 11030 동일·BYTE=SAME·프롤로그 20B 동일. orig_len 12 유지.

const RVA_TG_CALL: usize = 0x1feca43;        // ⏸stale 유지(0.5.4 주소, inert). install guard "not a CALL(E8)" bail=안전.

const RVA_THREATGATE_FN: usize = 0x20a8680;  // ⏸stale 유지=inert(0.5.4 주소). TG_CALL guard로 미설치=안전.

// facet#2 position: driver 내 generic_build(이동좌표 최종화) 호출지점.
const RVA_F2_BUILD_CALL: usize = 0x1a1ef3e;  // ⏸stale 유지=inert(0.5.4 주소, 스왑 금지). target-guard로 미설치.

const RVA_GENERIC_BUILD: usize = 0xceb5f0;  // ★0.5.6(was 0.5.5 0xcc1030). UNIQUE·size 27883 동일·BYTE=SAME·프롤로그 동일. orig_len 12 유지.

// ★facet#2 레인워크 waypoint 선택. install_replace_detour_rax 무조건설치 경로.
const RVA_FC59A0: usize = 0xd40f10;  // ★0.5.6(was 0.5.5 0xcf7b80). recall_rng_score. UNIQUE·size 1459 동일·BYTE=SAME·프롤로그 20B 동일. orig_len 12.

// ★pre-gate 상수 테이블(.rdata). p1(lane)<4만 사용. tableA[0..4]=[0,1,3,2](인덱스 변환).
const RVA_TABLE_A: usize = 0x33e1808;  // ★0.5.6(was 0.5.5 0x33550f8). .rdata 값지문 [0,1,3,2] u64 — 0.5.5와 동일하게 2건(0x3370008/0x33eedf0)·주소순 #1 대응. ← 0.5.7 재핀 — lea참조 유일 + 48B 내용매칭 두 방법 일치 (0.5.6=0x3370008)

// ── 영역 D 출력검증(gb_region_d): mid-func 캡처 detour 지점 ── ⏸SKIP 유지
const RVA_GB_REGIOND_HOOK: usize = 0x22dafea;  // ⏸SKIP 유지=차단(0.5.4 주소, MIG_GB_CHANGED=true).

const ORIG_LEN_GB_REGIOND: usize = 14;         // (REGIOND_HOOK이 SKIP이라 미사용)

const RVA_GB_FUNNEL: usize = 0x22dbc4e;           // ⏸SKIP 유지=inert(0.5.4 주소, 무효).

// facet#1 condgate(목표커밋 bool). rcx=subplan_ctx(*=disc), r9=reg
const RVA_CONDGATE: usize = 0xcaf0d0;   // ★0.5.6(was 0.5.5 0xe193b0). UNIQUE·size 513 동일·BYTE=SAME·프롤로그 20B 동일. orig_len 15 유지(경계 [0,1,2,3,7,15..] 동일).

const RVA_MOVEPRI: usize = 0xcaf9f0;   // ★0.5.6(was 0.5.5 0xe19cf0). **Plan 디스패처**(SubPlan 아님).
//   UNIQUE·size 1062 동일·BYTE=SAME. **orig_len 14 유지**(0.5.5와 프롤로그 동일: 41 57 41 56... 경계 [0,2,4,5,6,7,11,14,22]).
//   인덱스식 idx=(disc>=2)?disc-2:4 **바이트동일 유지** ⟹ plan_disc identity 유지(0.5.5 그대로).
//   JT 베이스 0.5.5 0x336637C → 0.5.6 **0x338128c**(16엔트리·arm 오프셋 16/16 일치).

// ★cVar6==0 STAND vs roll 게이트 → my_fa1ea0 순수재현으로 완전대체(RVA_FA1EA0 제거).
const RVA_COMMIT_CALL: usize = 0x2125602;     // ⏸stale 유지=inert(0.5.4 주소). target-guard 자체보호.

const RVA_COMMIT_FN: usize = 0x2d4500;        // ⏸stale 유지=inert(0.5.4 주소). target-guard로 inert(안전).

// 페이즈 게이트 threshold = objective*9 + min(B,100)*2 + BASE(=100). imm8(베이스) 패치 = 교전 공격성 다이얼.
const RVA_ENGAGE_GATE: usize = 0x1c9b33d;     // ⏸stale 유지=inert(0.5.4 주소). apply_engage_base가 83C0 sanity서 return=안전 inert. 재RE 필요

// ════ disc18/19(진짜 넥서스 AttackNexus/DefenseNexus) 완전재현 Phase2-1: 캡처 훅 ════
const RVA_DISC18_HANDLER: usize = 0xe81680;   // ★0.5.6(was 0.5.5 0xd28da0). UNIQUE·size 5891 동일·BYTE=SAME·프롤로그 동일. orig_len 12.

// ★SubPlan 디스패처(= disc18/19 핸들러를 직접 call 하는 그 함수).
//   ⚠2계층 주의: RVA_MOVEPRI 는 Plan 디스패처, 이건 SubPlan 디스패처. 번호공간이 다르다.
const RVA_SUBPLAN_DISPATCH: usize = 0xe35bd0;   // ★0.5.6(was 0.5.5 0xe4b460).
//   UNIQUE·size 1310 동일·BYTE=SAME·프롤로그 동일. orig_len 12.
//   인덱스식 idx=(disc>=2)?disc-2:8 바이트동일(바이어스·default 불변 = SubPlan 번호 불변).
//   19 arm call 순서 disc18(call@+0x10e)·disc19(call@+0x32c) 위치까지 1:1 보존. JT 베이스 0.5.5 0x3366E5C → 0.5.6 **0x3381d6c**.

const RVA_DISC19_HANDLER: usize = 0xe928f0;   // ★0.5.6(was 0.5.5 0xd38910). UNIQUE·size 11221 동일·BYTE=SAME·프롤로그 동일. orig_len 12.
//   교차확증 = 함수 내 유일 lea r9,[rip](DISC7 desc)가 동일 위치서 재발견(055 @0xd3906a→0x3358fc8 ↔ 056 @0xd44dba→0x3372198).

// ══ itemnet 빌드 스코어러 NULL-모델 가드 (튜토리얼/에이전트부재 AV 원천차단) ══
const RVA_ITEMNET_SCORER: usize = 0x11e1b10;   // ★0.5.6(was 0.5.5 0x12624f0 — ⚠고대역 대이동 −0x30e710·저대역 델타 가정 금지 사례).
//   UNIQUE·size 1609 동일·BYTE=SAME·프롤로그 20B 동일. 설치 위치 = fn+12(push8 프롤로그 직후).

const RVA_C8C_DMG_SHEET: usize = 0x337f778;   // ★0.5.6(was 0.5.5 0x3354c40). 공격측 데미지시트 desc. ← 0.5.7 재핀 — owner 2함수 명령정렬, 5사이트 만장일치 (0.5.6=0x336dcd0)
//   재핀 경로 = 0.5.5와 동일: passive_jungle 데미지시트 함수(055 0xce0280 → 056 0xcea050, BYTE=SAME) 안의 lea r9,[rip] 2사이트(@0xcea3a9·@0xceaa9f) 둘 다 동일값.
//   ⚠stale이면 임의 바이트를 vtable로 삼아 호출 → AV. probe_basedmg_r9 화이트리스트(OK_DESC_056)와 동기 갱신 완료(2026-08-20).

// disc7 위협 엔티티 e가 self(se)를 때리는 DPS(실데미지×1000/공속).
const RVA_DISC7_DMG_SHEET: usize = 0x3384c30;   // ★0.5.6(was 0.5.5 0x3358fc8). "받는 데미지" 시트 desc. ← 0.5.7 재핀 — lea참조 62건 중 49표 (0.5.6=0x3372198)
//   재핀 경로 = disc19 핸들러(0xd426e0) 내부의 lea r9,[rip] 함수 내 유일 사이트(@0xd44dba).

// ★[0.5.4 도입] 경매(전술 입찰) 진입 — passthrough 프로브. push8 프롤로그.
const RVA_AUCTION: usize = 0xe65b10;  // ★0.5.6(was 0.5.5 0xe04800). UNIQUE·size 19492 동일·BYTE=SAME.
//   선두 15B 동일(pro20 차이 = __chkstk call rel32뿐 = 트램폴린 12B 밖). orig_len 12 유지. ⚠passthrough 프로브 전용.
