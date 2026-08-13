// rva_055.rs — 전 RVA/ORIG_LEN 상수 (게임 0.5.5 기준). ★패치 대응시 원칙적으로 이 파일만 수정 + MIGRATION §7 갱신.
// 들어있는 것: RVA_* 훅/함수/콜사이트 주소, ORIG_LEN_* 프롤로그 경계 길이. image base=0x140000000, RVA=abs-base.
//
// ★★[0.5.5 마이그 상태 = 1단계 재핀완 (2026-08-12) — include! 전환·바이트패치·detour 반영은 2단계] ★★
//   0.5.4→0.5.5 = **온건 패치(AI 판단 측면). 대개편 아니다.**
//   함수 134,556→135,994(+1,438), .text +898KB. ★그러나 급증분은 **거의 전부 이미지/코덱 라이브러리**
//   (image-0.24.9 buffer +36·sample +30·bmp/pnm/webp, png-0.17.14 +15, jpeg-decoder +18, exr +6, std io/thread/sync).
//   ⟹ **AI 계열 함수는 사실상 불변**: path_finder 123→123·free_dist 83→83·path_field 49→49·minion_wave_risk 4→4 (전부 +0),
//      simulation.rs +6·game.rs -19(리팩터). **0.5.4 때의 경로/거리 하위시스템 신설 같은 신규 AI 서브시스템은 0.5.5에 없다.**
//   판단 트리 골격(Plan 16 arm / SubPlan 19 arm → 핸들러 소스 매핑)은 **1:1 그대로**다.
//
//   ⚠계약 변경(이 파일 밖 = detour.rs·tfm2_ai_adjust.rs 에서 2단계로 고칠 것):
//     · **Plan enum 번호가 +2 시프트(0.5.4 대비) = 0.5.3 번호로 복귀.** MOVEPRI 인덱스식이
//       0.5.4 idx=disc(바이어스 없음) → 0.5.5 idx=(disc>=2)?disc-2:4 (바이어스 부활, default 4).
//       ⟹ **plan_disc_053(raw)=raw+2 를 identity(raw+0)로 바꿔야** 한다(0.5.5 raw disc = 0.5.3 번호 = plan_name 기대값).
//       0.5.4 judge_dump 주석의 "Plan −2 시프트" 는 0.5.5에서 **되돌려졌다**. plan 을 해석해 쓰는 전 지점 재검토.
//     · **SubPlan 번호는 불변.** SUBPLAN 인덱스식 (disc>=2)?disc-2:N 의 바이어스 disc-2 **동일**,
//       default 만 7→8(disc<2 edge 라우팅만 이동, 실 subplan 번호 무영향).
//     · **구조체 오프셋 시프트**(§6 강한 사전값 정합): athlete **team +0x810→+0x930, role +0x8a0→+0x9c0**
//       (MOVEPRI 본문 실측 확정). ⏳판단력 +0x3f0 / 엔티티레코드 plan +0x5e8·sub_plan +0x708 / Order·SmallAction
//       stride 0xB8·tag +0xB1 = **2단계에서 게임함수 실측으로 확정**(§6 대역규칙만으론 순진 이동 금지 — task 경고).
//     · **MOVEPRI orig_len 12 → 14 필수.** 0.5.5 프롤로그에 push r15(41 57) 1개가 앞에 추가돼 명령경계가
//       [0,2,4,5,6,7,11,14,22] 로 밀렸다. 12는 경계가 아니라 sub rsp,0x50 한복판 → 그대로 쓰면 즉사
//       (0.5.3 때 13→12 사고와 같은 성격). install_replace_detour(RVA_MOVEPRI, 14, mp_capture) 로 고칠 것.
//
//   ✅검증 방식 = exe↔exe capstone 실측(도구 C:\tfm2mods\v54\ + repin55/jt55b 드라이버):
//      · 짝짓기 = 패닉 Location 소스앵커(버전무관) + 명령 골격 정렬(reloc.pair_fn) — 상수값 매칭 금지
//      · 훅 검증 = ①프롤로그 선두 N바이트 ②orig_len 명령경계 정확 ③트램폴린 구간 rip-rel/분기 부재
//
//   ⏸stale 유지 = **고의 보류(종전부터 inert, 0.5.4 주소 그대로 둠)**: TG_CALL / THREATGATE_FN / F2_BUILD_CALL /
//      GB_REGIOND_HOOK / GB_FUNNEL / COMMIT_CALL / COMMIT_FN / ENGAGE_GATE. 전부 신원검증(target-guard·E8·프롤로그)에
//      걸려 미설치=inert 이므로 방치가 fail-safe. 0.5.5 재핀 시도 안 함(rva_054 값 복사).

const RVA_RETREAT:  usize = 0xce0f70;   // ★0.5.5(was 0.5.4 0xe625e0). retreat_engage(old\passive_jungle.rs).
//   프롤로그 선두 12B 55 41 57 41 56 41 55 41 54 56 57 53(push8) 바이트 완전동일·orig_len 12 경계정확·rip-rel/분기 無.
//   13B째부터 sub rsp,0x358→0x368(프레임만 커짐, 트램폴린 12B 밖이라 무관). 크기 10626→11030, 골격 91%.
//   ⚠eng_role 4임계(100/70/50/30) 레지스터가 0.5.4에서 또 바뀌었을 수 있음 → detour.rs 사이트 2단계 재확인.

const RVA_TG_CALL: usize = 0x1feca43;        // ⏸stale 유지(0.5.4 주소, inert). threatgate가 gb 내부 임베드. install_threatgate_hook이 "not a CALL(E8)"서 bail=안전 inert.

const RVA_THREATGATE_FN: usize = 0x20a8680;  // ⏸stale 유지=inert(0.5.4 주소). gb 내부 mid-block 임베드. TG_CALL guard(E8+target)로 미설치=안전.

// facet#2 position: driver 내 generic_build(이동좌표 최종화) 호출지점.
const RVA_F2_BUILD_CALL: usize = 0x22dd4fe;  // ⏸stale 유지=inert(0.5.4 주소, 스왑 금지). target-guard로 미설치. facet#2 재현 폐기 상태.

const RVA_GENERIC_BUILD: usize = 0xcc1030;  // ★0.5.5(was 0.5.4 0xdca240). old\battle.rs 최대함수 entry.
//   프롤로그 선두 12B push8 완전동일·orig_len 12 경계정확·rip-rel無. 프레임 sub rsp 0x668→0x688. 크기 29,389→27,883, 골격 87%.
//   body detour 는 INSTALL_DIAG_HOOKS=false 라 미설치·move-post 는 F2_BUILD_CALL target-guard 경유 inert 이라 당장 영향 없음.

// ★facet#2 레인워크 waypoint 선택. install_replace_detour_rax 무조건설치 경로.
const RVA_FC59A0: usize = 0xcf7b80;  // ★0.5.5(was 0.5.4 0xe79c00). recall_rng_score.
//   크기 1459→1459 **동일**, 프롤로그 20B **바이트 완전동일**, 골격 100%. orig_len 12(push8=12B 경계) 정확·rip-rel無.

// ★pre-gate 상수 테이블(.rdata). p1(lane)<4만 사용. tableA[0..4]=[0,1,3,2](인덱스 변환).
const RVA_TABLE_A: usize = 0x33550f8;  // ★0.5.5(was 0.5.4 0x327aa60). .rdata 값지문 매칭: 선두 32B(=[0,1,3,2] u64 4엔트리)
//   0.5.4 3건(0x327aa60/0x328d0e8/0x32cdaf8) → 0.5.5 **2건(0x33550f8/0x3404ce0)** → 주소순 대응 #1.
//   ⚠앞 4엔트리 동일하고 소스는 p1<4만 읽으므로 어느 것이든 기능 등가(0.5.4에서도 같은 상황).

// ── 영역 D 출력검증(gb_region_d): mid-func 캡처 detour 지점 ── ⏸SKIP 유지
const RVA_GB_REGIOND_HOOK: usize = 0x22dafea;  // ⏸SKIP 유지=차단(0.5.4 주소, MIG_GB_CHANGED=true). generic_build 이동+본문변경으로 무효. 재활성=0xcc1030 안 재추출 필요.

const ORIG_LEN_GB_REGIOND: usize = 14;         // (REGIOND_HOOK이 SKIP이라 미사용)

const RVA_GB_FUNNEL: usize = 0x22dbc4e;           // ⏸SKIP 유지=inert(0.5.4 주소, 무효).

// facet#1 condgate(목표커밋 bool). rcx=subplan_ctx(*=disc), r9=reg
const RVA_CONDGATE: usize = 0xe193b0;   // ★0.5.5(was 0.5.4 0xe13ca0).
//   프롤로그 20B 56 57 53 48 83 ec 40 4c 8b 9c 24 90 00 00 00 4c 8b 94 24 80 **바이트 완전동일**·
//   orig_len 15 경계정확(경계 [0,1,2,3,7,15,23,26,30])·rip-rel無 ⟹ 최고신뢰. 크기 497→513, 골격 99%.

const RVA_MOVEPRI: usize = 0xe19cf0;   // ★0.5.5(was 0.5.4 0xe145b0). **Plan 디스패처**(SubPlan 아님).
//   ★★프롤로그가 바뀌었다: 0.5.4 41 56 …(push r14…) → 0.5.5 41 57 41 56 …(push r15 추가) + sub rsp 0x48→0x50.
//     ⟹ **orig_len 12 → 14 필수**(경계 [0,2,4,5,6,7,11,14,22]. 12는 sub rsp 한복판 = 즉사). 14B 안 rip-rel/분기 無.
//   ★★인덱스식이 또 바뀌었다: 0.5.4 idx=disc → 0.5.5 **idx=(disc>=2)?disc-2:4**(바이어스 부활).
//     ⟹ **Plan 번호가 +2 시프트(0.5.4 대비) = 0.5.3 번호로 복귀.** plan_disc_053(=raw+2)를 identity 로 되돌릴 것.
//   JT 16엔트리 arm 레이아웃 유지(첫명령 대조 15/16 동일: 13 완전동일 + [9][14]는 team 0x810→0x930 시프트뿐,
//     arm4 만 본문 변경). 크기 969→1062, 골격 93%. JT 베이스 0.5.4 0x3286AC4 → 0.5.5 **0x336637C**(16엔트리).

// ★cVar6==0 STAND vs roll 게이트 → my_fa1ea0 순수재현으로 완전대체(RVA_FA1EA0 제거).
const RVA_COMMIT_CALL: usize = 0x1e3dfd2;     // ⏸stale 유지=inert(0.5.4 주소). target-guard 자체보호.

const RVA_COMMIT_FN: usize = 0x235ffa0;        // ⏸stale 유지=inert(0.5.4 주소). target-guard로 inert(안전).

// 페이즈 게이트 threshold = objective*9 + min(B,100)*2 + BASE(=100). imm8(베이스) 패치 = 교전 공격성 다이얼.
const RVA_ENGAGE_GATE: usize = 0x1c9b33d;     // ⏸stale 유지=inert(0.5.4 주소). apply_engage_base가 83C0 sanity서 return=안전 inert. 재RE 필요


// ════ disc18/19(진짜 넥서스 AttackNexus/DefenseNexus) 완전재현 Phase2-1: 캡처 훅 ════
const RVA_DISC18_HANDLER: usize = 0xd28da0;   // ★0.5.5(was 0.5.4 0xda1850). sub_plan\attack_nexus.rs.
//   프롤로그 12B push8 완전동일·orig_len 12 경계정확·rip-rel無. 크기 5,891→5,891 **동일**, 골격 100%.
//   ★HARNESS_ON 하에 install_wrap 무조건 설치 경로 = 최우선 반영 대상.

// ★SubPlan 디스패처(= disc18/19 핸들러를 직접 call 하는 그 함수). ⚠소스앵커 없어 disc18/19 call-site 역추적으로 확정.
//   ⚠2계층 주의: RVA_MOVEPRI 는 Plan 디스패처, 이건 SubPlan 디스패처. 번호공간이 다르다.
const RVA_SUBPLAN_DISPATCH: usize = 0xe4b460;   // ★0.5.5(was 0.5.4 0xe52990).
//   프롤로그 12B push8(55 41 57 41 56 41 55 41 54 56 57 53) 완전동일·orig_len 12 경계정확. 크기 1310→1310 **동일**.
//   인덱스식 idx=(disc>=2)?disc-2:8 (default 0.5.4 7→8, **바이어스 disc-2 불변 = SubPlan 번호 불변**).
//   19 arm call 순서 disc18(call@+0x10e)·disc19(call@+0x32c) 위치까지 1:1 보존. JT 베이스 0.5.4 0x3288D1C → 0.5.5 **0x3366E5C**.

const RVA_DISC19_HANDLER: usize = 0xd38910;   // ★0.5.5(was 0.5.4 0xdac090). sub_plan\defense_nexus.rs.
//   프롤로그 12B push8 완전동일·orig_len 12 경계정확. 크기 11,221→11,221 **동일**, 골격 100%.
//   교차확증 = 함수 내 유일 lea r9,[rip](DISC7 desc)가 0.5.4처럼 재발견(054 @0xdadfa2→0x327fba0 ↔ 055 @0xd3906a→0x3358fc8).


// ══ itemnet 빌드 스코어러 NULL-모델 가드 (튜토리얼/에이전트부재 AV 원천차단) ══
const RVA_ITEMNET_SCORER: usize = 0x12624f0;   // ★0.5.5(was 0.5.4 0x145a680).
//   프롤로그 20B **바이트 완전동일**, 크기 1,609→1,609 **동일**, 골격 100% ⟹ 사실상 무변경 함수.
//   설치 위치 = fn+12(push8 프롤로그 직후)의 15B 검증 후 변위.

const RVA_C8C_DMG_SHEET: usize = 0x3354c40;   // ★0.5.5(was 0.5.4 0x3288e48). 공격측 데미지시트 desc.
//   재핀 경로 = 0.5.4와 동일: passive_jungle 데미지시트 함수(054 0xe619c0 → 055 0xce0280, 골격0.88) 안의 lea r9,[rip] 2사이트 둘 다 동일값.
//   ⚠이 값이 stale이면 임의 바이트를 vtable로 삼아 호출 → AV. probe_basedmg_r9 화이트리스트(OK_DESC)도 2단계에서 같이 갱신.
//   ⏳sanity(size 0x6a8 / vt+0x30)는 2단계에서 실측 재확인(0.5.4 vt+0x30=0xc7ead0 → 0.5.5 이동).

// disc7 위협 엔티티 e가 self(se)를 때리는 DPS(실데미지×1000/공속).
const RVA_DISC7_DMG_SHEET: usize = 0x3358fc8;   // ★0.5.5(was 0.5.4 0x327fba0). "받는 데미지" 시트 desc.
//   재핀 경로 = disc19 핸들러(0xd38910) 내부의 lea r9,[rip] 함수 내 유일 사이트(@0xd3906a).
//   ⏳sanity(size 0x6a8 / vt+0x30)는 2단계 실측 재확인.

// ★[0.5.4 도입] 경매(전술 입찰) 진입 — passthrough 프로브. push8 프롤로그.
const RVA_AUCTION: usize = 0xe04800;  // ★0.5.5(was 0.5.4 0xeacf10). handler\auction.rs.
//   선두 12B push8 완전동일·orig_len 12 경계정확·rip-rel無(13B째부터 __chkstk imm 0x2e38→0x2e48 차이=트램폴린 밖).
//   크기 19,332→19,492, 골격 99%. ⚠passthrough 프로브 전용(원본 항상 호출, 기능 무영향).