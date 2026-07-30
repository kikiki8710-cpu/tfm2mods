// rva_053.rs — 전 RVA/ORIG_LEN 상수 (게임 0.5.3 기준). ★패치 대응시 원칙적으로 이 파일만 수정 + MIGRATION §7.3 갱신.
// 들어있는 것: RVA_* 훅/함수/콜사이트 주소, ORIG_LEN_* 프롤로그 경계 길이. (일부 데이터 RVA는 각 재현파일에 잔류)
// 언제 손대나: 게임 패치로 exe 바뀌어 RVA 어긋날 때. image base=0x140000000, RVA=abs-base.
//
// ★★[0.5.3 마이그 상태 = 훅 전량 반영완 (2026-07-29)] ★★
//   0.5.2→0.5.3 = **전면 재컴파일**(.text 44.0→48.6MB +10.5%, 함수 120,995→132,960, AI가 libgame_ai 크레이트로 분리).
//   ⟹ 연속바이트 마스크시그 전멸·함수 크기 2~10% 증가·**함수내 오프셋 비보존**. "델타 더하기" 금지.
//   ✅검증 방식 = exe↔exe capstone 실측(도구 `C:\tfm2mods\verify_053.py`·`bytepatch*_053.py`·`desc_053.py`):
//      ① 프롤로그 선두 N바이트 완전일치 ② orig_len 명령경계 정확 일치 ③ rip-rel/상대분기 부재.
//   ✅반영완(선두 바이트 완전동일 실측): RETREAT / FC59A0 / CONDGATE / MOVEPRI / DISC18 / DISC19 / GENERIC_BUILD /
//      ITEMNET_SCORER / LOADER / PARSER / ALLOC(★형태변경) / C8C·DISC7 desc / TABLE_A / D19_TV7 / TEXT_END.
//   ⏸stale 유지 = **고의 보류(종전부터 inert)**: TG_CALL / THREATGATE_FN / F2_BUILD_CALL / GB_REGIOND_HOOK / GB_FUNNEL /
//      COMMIT_CALL / COMMIT_FN / ENGAGE_GATE / D19_SLOT2·STATIC·STATIC2. 전부 신원검증(target-guard·프롤로그·E8 체크)에
//      걸려 **미설치=inert**이므로 방치가 fail-safe.
//   ⛔미해결 byte-patch 4사이트(패치 skip=안전): an_cull_dist(disc18) / d19b_hp_low / d19b_near1 / d19b_near2.
//   상세 = MODS\MIGRATION.md §7.3 + C:\tfm2mods\_MIGRATE_053.md

const RVA_RETREAT:  usize = 0xe00350;   // ★0.5.3(was 0.5.2 0x1b94670). 실측: 선두 12B `55 41 57 41 56 41 55 41 54 56 57 53`(push8) **바이트 완전동일**·orig_len 12 경계정확·rip-rel/분기 無 ⟹ install_replace_detour 안전. // ★0.5.2(was 0.5.1 0x1e08cd0). retreat_engage. 콜체인 retreat→fc59a0, disc==4게이트. 프레임 0xc8→0x308 재정렬(ABI/로직 동일→retreat_capture 무영향)

const RVA_TG_CALL: usize = 0x1feca43;        // ⏸stale 유지(0.5.0 이후 계속 실패=inert). threatgate가 gb 내부 임베드=독립콜사이트 없음. install_threatgate_hook이 "not a CALL(E8)"서 bail=안전 inert. 재RE 전 유지

const RVA_THREATGATE_FN: usize = 0x20a8680;  // ⏸stale 유지=inert. gb 내부 mid-block 임베드=독립함수/콜사이트 없음. TG_CALL guard(E8+target)로 미설치=안전. 재RE 필요

// ★plan_lane_predicate = my_lane_predicate로 순수재현 완료(2026-06-19, DIFF=0) → RVA_LANE_PRED 제거(churn 소멸).
// facet#2 position: driver 내 generic_build(이동좌표 최종화) 호출지점. 직후 rcx=최종 Move{tag,x,y}.
const RVA_F2_BUILD_CALL: usize = 0x22dd4fe;  // ⏸stale 유지=inert(스왑 금지). 실값으로 스왑하면 target-guard 통과→move-post 훅 설치=facet#2 재활성(미완·GB 메인빌드 전체재현 폐기 상태). facet#2 재활성 선결=이 값 재핀. [GB region-D 위성도 INSTALL_DIAG_HOOKS false + MIG_GB_CHANGED true로 이중 inert]

const RVA_GENERIC_BUILD: usize = 0xe06c10;  // ★0.5.3(was 0.5.2 0x22b2280). 실측: 선두 12B push8 **바이트 완전동일**·orig_len 12 경계정확·rip-rel無. 함수크기 22,764B(0.5.2 대비 -7%). body detour는 INSTALL_DIAG_HOOKS=false라 미설치·move-post는 F2_BUILD_CALL target-guard 경유 inert. ⚠내부 gb_* byte-patch는 아래 재핀분만 라이브. // ★0.5.2(was 0.5.1 0x1e1ebb0). battle 최대함수 entry. 진입 MOV R14,RCX(out)

// ★facet#2 레인워크 waypoint 선택. 프롤로그 12B(8 push). param_2=rdx,param_5/6/7=[entry+0x28/0x30/0x38].
// param_7=챔피언, param_6=지오메트리(*p6=L80,p6[1]=VOBJ,p6[2]=GEO), param_5=lane(side@+0x6a8,lane_kind@+0x738).
const RVA_FC59A0: usize = 0xe168d0;  // ★0.5.3(was 0.5.2 0x1bdb3e0). 실측: 선두 12B push8 **바이트 완전동일**·orig_len 12 경계정확·rip-rel無. ★install_replace_detour_rax 무조건설치 경로라 최우선 검증 대상이었음. [pregate = ⬜0.5.3 미재핀(0.5.2 0x1bdae60) — 순수재현 my_pregate라 **훅 없음 = 무영향**, 상수로도 안 쓰임] // ★0.5.2(was 0.5.1 0x1e2c980). recall_rng_score. HP밴드+0x658/0x610·recall_point·A=+0x218·거리밴드·RNG Lemire·out SETGE[+4]/[+0]/[+8]

// ★pre-gate 상수 테이블(.rdata, RVA). p1(lane)<4만 사용. tableA[0..4]=[0,1,3,2](인덱스 변환).
const RVA_TABLE_A: usize = 0x31c0168;  // ★0.5.3(was 0.5.2 0x3828818). .rdata 값지문 매칭: 선두 32B(=[0,1,3,2] 4엔트리 u64) OLD 3건 == NEW 3건 → 주소순 순서대응 #1. ⚠3건 모두 **앞 4엔트리 값이 동일**하고 소스는 p1<4만 읽으므로 셋 중 어느 것이든 기능 등가(오답 리스크 실질 0). 48B 이상으로는 NEW 매칭 0 = 뒤쪽 엔트리는 변경됨. // ★0.5.2(was 0.5.1 0x384ea20). r15 threshold 인덱스 변환표

// ── 영역 D 출력검증(gb_region_d): mid-func 캡처 detour 지점 ──
const RVA_GB_REGIOND_HOOK: usize = 0x22dafea;  // ⏸stale 유지=차단(0.5.1부터 보류 지속). 클린14B슬롯 부재 + RBP 전면재배치 재도출 필요 + 진단전용 이중게이트 ⟹ 억지 재핀은 크래시 위험. MIG_GB_CHANGED=true로 SKIP.

const ORIG_LEN_GB_REGIOND: usize = 14;         // (REGIOND_HOOK이 SKIP이라 미사용)

const RVA_GB_FUNNEL: usize = 0x22dbc4e;           // ⏸stale 유지=inert. region D 공통출구·gbskip jump 타겟. 재활성 시 재핀 필요.

// facet#1 condgate(목표커밋 bool). 프롤로그 push3+sub0x40+mov = 15B 클린. rcx=subplan_ctx(*=disc), r9=reg
const RVA_CONDGATE: usize = 0xc550b0;   // ★0.5.3(was 0.5.2 0x21338d0). 실측: 선두 15B `56 57 53 48 83 ec 40 4c 8b 9c 24 90 00 00 00` **바이트 완전동일**·orig_len 15 경계정확·rip-rel無 ⟹ 최고신뢰. // ★0.5.2(was 0.5.1 0x1cbb8b0). facet#1 목표커밋 bool

const RVA_MOVEPRI: usize = 0xc559e0;   // ★0.5.3(was 0.5.2 0x2134240). ★★**orig_len 13 → 12 필수**(tfm2_ai_adjust.rs install_replace_detour 호출부도 같이 수정) — 0.5.3 프롤로그가 push6→**push4**로 줄어 명령경계가 0,2,3,4,5,9,**12**,20이 됐다. 13이면 `mov rax,[rsp+0xb8]`(8B) 한복판을 잘라 트램폴린이 깨진 명령을 실행 = 즉사.
//   복사 12B = `41 56 56 57 53 48 83 ec 48 48 89 ce`(push r14/rsi/rdi/rbx; sub rsp,0x48; mov rsi,rcx) → 복귀 fn+12=0xc559ec. rip-rel/상대분기 無(함수내 유일 rip-rel LEA는 +0x44 = 복사범위 밖).
//   ghidra-re 확정 근거 3중: ①디컴 시맨틱 동일(*param_2 서브플랜 id → -2 → cmovnc 1클램프 → 16엔트리 점프테이블)
//     ②콜러 1:1(0.5.2 FUN_1420d6e50이 MP×3+CG×2 ↔ 0.5.3 FUN_140d48ec0이 0xc559e0×3 + 확정 CONDGATE 0xc550b0×2, 인터리브 순서까지 동일)
//     ③**push 2개 감소가 스택오프셋 0x10 감소와 정확히 상쇄** → entry_rsp 기준 인자 위치 +0x28/+0x30/+0x38/+0x40/+0x50 **완전 불변** ⟹ mp_capture 무수정.
//   ⚠단 case-body 내부 재현은 레지스터 배정이 바뀜(r15→r11, r14→rdi, 인덱스 rdi→rbx) — 로컬 오프셋 그대로 믿지 말 것.
//   ⚠CONDGATE와의 간격은 0x930(0.5.2는 0x970) = 간격 비보존(참고용).

// ★cVar6==0 STAND vs roll 게이트 → my_fa1ea0(순수재현, 288/288 DIFF0)로 완전대체 → RVA_FA1EA0 제거(2026-06-19).
const RVA_COMMIT_CALL: usize = 0x1e3dfd2;     // ⏸stale 유지=inert. target-guard 자체보호.

const RVA_COMMIT_FN: usize = 0x235ffa0;        // ⏸stale 유지=inert. 함수가 너무 작아(15 instr) 스켈레톤/멀티셋 매칭 후보 0 = 재핀 불가. target-guard로 inert(안전).

// 페이즈 게이트 threshold = objective*9 + min(B,100)*2 + BASE(=100). imm8(베이스) 패치 = 교전 공격성 다이얼.
const RVA_ENGAGE_GATE: usize = 0x1c9b33d;     // ⏸stale 유지=inert(0.5.0부터). ADD EAX,0x64(83 c0 64) 소멸=교전+100보너스 로직 재구조화. apply_engage_base가 83C0 sanity서 return=안전 inert. 재RE 필요


// ════ disc18/19(진짜 넥서스 AttackNexus/DefenseNexus) 완전재현 Phase2-1: 캡처 훅 ════
//   핸들러 프롤로그 replace-wrap: cap_fn(my_capture)가 game 원본(트램폴린)을 호출한 뒤
//   game이 채운 sret Vec<Command>를 덤프 → my 재현과 game==mine 대조 기반. RNG-free라 재sim 무영향.
const RVA_DISC18_HANDLER: usize = 0xd94d00;   // ★0.5.3(was 0.5.2 0x2376320). 실측: 선두 12B push8 **바이트 완전동일**·orig_len 12 경계정확·rip-rel無. 함수크기 5,923B. ★HARNESS_ON 하에 install_wrap **무조건 설치** 경로 = 구값 방치 시 오후킹이라 최우선 반영 대상. // ★0.5.2(was 0.5.1 0x1c7ca20). AttackNexus 핸들러(disp3 idx16)

// ★[07-31 신설] SubPlan 디스패처(= disc18/19 핸들러를 호출하는 그 함수). ghidra-re 확정(RE\2026-07-31_disc18-19-발화조건).
//   프롤로그 push8 + `sub rsp,0x88` ⟹ 명령경계 [0,1,3,5,7,9,10,11,12,19] = **orig_len 12 경계정확**.
//   구조: `r13=[rdx]`(=SubPlan) → `idx=(r13>=2)?r13-2:7` → JT `0x1431ba310` → idx16→disc18 / idx17→disc19.
//   호출자는 exe 전역 **단 1곳**(`0xd5fc32`) ⟹ comp_test 든 실경기든 챔피언 AI 를 돌리면 반드시 이 함수를 탄다.
//   ⚠**2계층 주의**: `RVA_MOVEPRI`(0xc559e0)는 **Plan** 디스패처(0~17)이고 이건 **SubPlan** 디스패처다. 번호공간이 다르다.
//     Plan16(진입결정 old\attack_nexus.rs) → SubPlan18(실행 sub_plan\attack_nexus.rs) / Plan17 → SubPlan19.
const RVA_SUBPLAN_DISPATCH: usize = 0xd98740;

const RVA_DISC19_HANDLER: usize = 0xdece30;   // ★0.5.3(was 0.5.2 0x2380820). 실측: 선두 12B push8 **바이트 완전동일**·orig_len 12 경계정확. 함수범위 0xdece30~0xdef0d8. **교차확증 = DISC7 desc를 참조하는 `lea r9`가 0.5.2와 똑같이 이 함수 내부 유일 사이트로 재발견**(0.5.2 @0x2382a95 ↔ 0.5.3 @0xdeeebe). // ★0.5.2(was 0.5.1 0x1e0ddb0). DefenseNexus 핸들러(disp3 idx17)


// ══ itemnet 빌드 스코어러 NULL-모델 가드 (튜토리얼/에이전트부재 AV 원천차단) ══
//   시그 fn(model, ctx, build_ptr, build_len, flag)->f32. owner(팀AI블롭) NULL이면 model=owner+0x1558≈저주소가
//   3단계 관통해 `MOV RAX,[RBX+0x10]`서 결정적 AV. 가드 = rcx(model) 범위체크+readable — 순수검사라 rayon 워커서도 안전.
//   ★설치 위치 = fn+12(push8 프롤로그 직후). fn+12의 15B(sub rsp,0xd8 7B + lea rbp,[rsp+0x80] 8B, rip-rel無)를 byte-exact 검증 후 변위.
const RVA_ITEMNET_SCORER: usize = 0x10587e0;   // ★0.5.3(was 0.5.2 0x1b9cce0). 실측 2중 확인: ①프롤로그 12B push8 완전동일 ②**fn+12의 15B = `48 81 ec d8 00 00 00 48 8d ac 24 80 00 00 00` 0.5.2와 바이트 완전동일**(=가드 신원검증 통과 보장, 15B가 명령경계 정확). ⚠fn+0x81은 `48 8b 43 10`→`48 89 45 10`으로 변경(0.5.2 재핀 때 쓴 **변별 필터**였을 뿐 가드 검증 대상 아님 — 설치에 무영향).

const RVA_C8C_DMG_SHEET: usize = 0x31be1a8;   // ★0.5.3(was 0.5.2 0x381e1e0). 재핀 경로 = 0.5.2 원참조함수 0x1b93830(니모닉 cos 0.9972 최상위)→0.5.3 **0xdff660**, 그 안의 `lea r9,[rip]` 2사이트가 **둘 다 0x31be1a8**(0.5.2도 같은 함수에서 C8C가 정확히 2사이트 = 구조 일치).
//   sanity 통과: desc = {drop=0x140dfbc60, size=**0x6a8**, align=**8**} + **vt+0x30 = 0xc51bc0**(= 우리가 실제 shadow-call 하는 base데미지쌍 게터).
//   ※0.5.3에서도 `call [reg+0x28]`에 쓰이는 desc 전부가 vt+0x30 == 0xc51bc0 **동일** ⟹ CGU 클론 간 동작·위험 등가(0.5.2 결론이 그대로 성립).
//   ⚠이 값이 stale이면 임의 바이트를 vtable로 삼아 호출 → AV. **0.5.2 disc14 크래시 2·3차의 진범이 바로 이 상수 방치**였다. probe_basedmg_r9의 화이트리스트(OK_DESC)도 반드시 같이 갱신할 것.

// disc7 위협 엔티티 e가 self(se)를 때리는 DPS(실데미지×1000/공속).
const RVA_DISC7_DMG_SHEET: usize = 0x31bcef8;   // ★0.5.3(was 0.5.2 0x38d1918). 재핀 경로 = **0.5.2와 완전히 동일한 xref 경로 재현**: disc19 핸들러 내부의 `lea r9,[rip]`가 함수 내 유일 사이트이고 그 타겟이 답(0.5.2 disc19 @0x2382a95 → 0x38d1918 ↔ 0.5.3 disc19 @0xdeeebe → **0x31bcef8**). sanity 통과: {drop=0x140de52e0, size=0x6a8, align=8, vt+0x30=0xc51bc0}.
