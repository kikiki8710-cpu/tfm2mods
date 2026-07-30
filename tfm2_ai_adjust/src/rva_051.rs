// rva_051.rs — 전 RVA/ORIG_LEN 상수 (게임 0.5.1 기준). ★패치 대응시 원칙적으로 이 파일만 수정 + MIGRATION §7.1 갱신.
// 들어있는 것: RVA_* 훅/함수/콜사이트 주소, ORIG_LEN_* 프롤로그 경계 길이. (일부 데이터 RVA는 각 재현파일에 잔류)
// 언제 손대나: 게임 패치로 exe 바뀌어 RVA 어긋날 때(version-migrator/migrate). image base=0x140000000, RVA=abs-base.

const RVA_RETREAT:  usize = 0x1e08cd0;   // ★0.5.1(was 0.5.0_3 0x1f37f70). retreat_engage. ghidra-re 확정(콜체인 retreat→0x1c63a50→fc59a0, disc==4게이트). 8push=12B, rip-rel無. install_replace_detour. 프레임 0xc8→0x308 재정렬(ABI/로직 동일→retreat_capture 무영향, ⚠스택로컬 재도출 task③)

const RVA_TG_CALL: usize = 0x1feca43;        // ⚠0.5.0 실패(stale 0.4.14값 유지). 0.5.0 아키텍처변경: threatgate가 gb 내부 임베드=독립콜사이트 없음. 0.5.0 exe 0x1feca43=0x02(E8아님)→install_threatgate_hook이 "not a CALL(E8)"서 bail=안전 inert(검증). 재RE 전 유지

const RVA_THREATGATE_FN: usize = 0x20a8680;  // ⚠0.5.0 실패(stale 0.4.14값 유지). gb 내부 mid-block 임베드=독립함수/콜사이트 없음(0.4.14 핫픽스서도 이미 inert). TG_CALL guard(E8+target)로 미설치=안전. 재RE 필요

// ★plan_lane_predicate(0x2080760) = my_lane_predicate로 순수재현 완료(2026-06-19, DIFF=0) → RVA_LANE_PRED/LanePred 제거(churn 소멸).
// facet#2 position: driver 내 FUN_141917430(generic_build, 이동좌표 최종화) 호출지점. 직후 rcx(=[RBP+0xd0])=최종 Move{tag,x,y}.
const RVA_F2_BUILD_CALL: usize = 0x22dd4fe;  // ⚠0.5.0_3값 유지=inert(스왑 금지). ★0.5.1 확정 콜사이트=0x1e27234(gb entry 0x1e1ebb0로 가는 GB 진입 유일 E8·콜러 FUN_141e25490, 2026-07-16 RE). ⚠실값 0x1e27234로 스왑하면 target-guard 통과→move-post 훅 설치=facet#2 재활성(미완·GB 메인빌드 전체재현 폐기 상태)이므로 재활성 준비 전엔 스왑 금지. facet#2 재활성 선결=이 값 스왑. [GB region-D 위성(REGIOND_HOOK/FUNNEL/203CB30/20C0690)도 INSTALL_DIAG_HOOKS false+MIG_GB_CHANGED true로 이중 inert] 상세=discovered-PROGRAM-STRUCTURE §3k

const RVA_GENERIC_BUILD: usize = 0x1e1ebb0;  // ★0.5.1(was 0.5.0_3 0x22db820). battle 최대함수 entry(24509B, F80320×7 호출지문, ghidra-re 확정). 8push+SUB0x5a8, 진입 MOV R14,RCX(out)→[RBP+0x500]. orig_len=12, rip-rel無. ⚠move-post 실설치는 F2_BUILD_CALL(0.5.1 미마이그) target-guard 경유=현재 fail-safe inert. body detour는 INSTALL_DIAG_HOOKS=false라 미설치

// ★facet#2 레인워크 waypoint 선택 = FUN_141dd7700. 프롤로그 12B(8 push). param_2=rdx,param_5/6/7=[entry+0x28/0x30/0x38].
// param_7=챔피언, param_6=지오메트리(*p6=L80,p6[1]=VOBJ,p6[2]=GEO), param_5=lane(side@+0x6a8,lane_kind@+0x738).
const RVA_FC59A0: usize = 0x1e2c980;  // ★0.5.1(was 0.5.0_3 0x1f553a0). recall_rng_score. ghidra-re 시맨틱 완전일치 확정(HP밴드+0x658/0x610·recall_point·A=+0x218·거리밴드·RNG Lemire·out SETGE[+4]/[+0]/[+8]). 8push=12B, rip-rel無, 프레임 0xe8→0xf8. ★install_replace_detour_rax 무조건설치. [pregate=0x1e2c320(0.5.1, ghidra-re 확정·순수재현 my_pregate·훅없음, fc59a0-0x660 인접)] ("438B 축소"=오정보, 실제 1750B)

// ★pre-gate(0x2080760) 상수 테이블(.rdata, RVA). p1(lane)<4만 사용. tableA[0..4]=[0,1,3,2](인덱스 변환), tableC/D=후보 비교좌표.
const RVA_TABLE_A: usize = 0x384ea20;  // ★0.5.1(was 0.5.0_3 0x38a75b0). r15 threshold 인덱스 변환표[0,1,3,2..]. ghidra-re HIGH 확정(pregate 0x1e2c320: &UNK_14384ea20 + lane*8 참조와 동형)

// ★0.5.0_2: tableC/D 정적표 폐기 → 목표좌표는 런타임 컨테이너 순회([p6+8]→[+0x20]holder→[+0x68]base/[+0x70]count, stride0x28, entry+0x20==p1). my_pregate 내 인라인.
// ── 영역 D 출력검증(gb_region_d): mid-func 캡처 detour 지점(genbuild_body_D.md "런타임 캡처 빌드") ──
const RVA_GB_REGIOND_HOOK: usize = 0x22dafea;  // ★07-10 정정(구 0x22daff8=명령중간절단+rel8 JBE2개 크래시): 안전슬롯 0x22dafea(INC RSI/MOV[R14-8],RSI/MOV RAX,[rbp+0x110]) 14B=정확히 0x22daff8 경계, 분기·rip-rel 전무. 복귀(passthrough)=0x22daff8. 결정게이트=직후 0x22daff8~0x22db0b0(CNT@rbp0x110·DA@0x100·DB@0x108·D2@0x170·L2@0x1d0). ⚠0.5.1=차단·갱신 보류(2026-07-16 RE): region-D 게이트=0.5.1 0x1e22306이나 클린14B슬롯 부재+RBP 전면재배치 재도출 필요+진단전용 이중게이트=억지 재핀 크래시 위험. 상세=discovered-PROGRAM-STRUCTURE §3k

const ORIG_LEN_GB_REGIOND: usize = 14;         // ★07-10 15→14: 0x22dafea+14=0x22daff8 정확경계(dafea3+dafed4+daff17). rip-rel/분기無(ghidra-re 확정).

const RVA_GB_FUNNEL: usize = 0x22dbc4e;           // ★0.5.0_3값 유지=inert(was 0x20e4a1a). region D 공통출구(result-copy→arena cleanup→에필로그)·gbskip jump 타겟. ★0.5.1 확정=0x1e22437(region-D 공통출구·arena rollback 2개·memcpy無=각 경로 out 직접기록, 2026-07-16 RE)·재활성 시 이 값. 상세=discovered-PROGRAM-STRUCTURE §3k

// facet#1 condgate(목표커밋 bool). 프롤로그 push3+sub0x40+mov rsi = 15B 클린. rcx=subplan_ctx(*=disc), r9=reg, 스택: rh_slot@entry+0x28, r11@+0x38, rsi@+0x40
const RVA_CONDGATE: usize = 0x1cbb8b0;   // ★0.5.1(was 0.5.0_3 0x19e40e0). version-migrator 확정 이동·본체동일(프레임 0x98→0x90). ⚠내부오프셋 재도출 필요(task③④). facet#1 목표커밋 bool

const RVA_MOVEPRI: usize = 0x1cbc220;   // ★0.5.1(was 0.5.0_3 0x19e4a50). version-migrator 확정 이동·본체동일. ⚠내부오프셋 시프트 재도출 필요(task④). condgate 인접

// ★cVar6==0 STAND vs roll 게이트: fa1ea0(액션큐)≠0xff면 STAND(8), ==0xff면 교전롤.
//   ⟹ my_fa1ea0(순수재현, 288/288 DIFF0 검증)로 완전대체 → 게임콜 RVA_FA1EA0 제거(churn 소멸, 2026-06-19).
// 광범위 이동/행동 커밋: driver가 매프레임 champion+0x590에 최종 Input을 쓰는 단일 지점(FUN_141a49fa0).
// 0x141d50341 LEA RCX,[champ+0x590]; 0x141d5034f LEA RDX,[RBP+0x200]=최종Input; 0x141d5035d CALL. dump=rdx(Input 0x90).
const RVA_COMMIT_CALL: usize = 0x1e3dfd2;     // 0.5.0(was 0.4.14 0x1c9bdca). handler.rs driver급 0x1e3d5a0이 commit_fn(0x2362f10) 3회호출 중 #1(exe검증 E8→0x2362f10; 나머지 0x1e3fca1/0x1e41839). champ Input오프셋 [champ+0x590]→[rbp+0x528] 이동가능성. target-guard 자체보호. ⚠어느 콜사이트가 정본인지 런타임 확인권장

const RVA_COMMIT_FN: usize = 0x235ffa0;        // ★0.5.1(was 0.5.0_3 0x19e7d30). version-migrator RVA-only 본체동일(guard inert). commit_fn. ⚠콜사이트 COMMIT_CALL은 handler driver NO MATCH로 재핀보류(guard inert)

// 페이즈 게이트 threshold = objective*9 + min(B,100)*2 + BASE(=100). driver 0x141d4f8ca: ADD EAX,0x64 (83 C0 64).
// imm8(베이스)을 패치 = 교전 공격성 다이얼. 낮추면 threshold↓ → score>=threshold 자주 → active 전환 빨라짐.
const RVA_ENGAGE_GATE: usize = 0x1c9b33d;     // ⚠0.5.0 실패(stale 0.4.14값 유지). 0.5.0에 ADD EAX,0x64(83 c0 64) 소멸=교전+100보너스 로직 재구조화. 0.5.0 exe 0x1c9b33d=0f7f85(무관)→apply_engage_base가 83C0 sanity서 return=안전 inert(검증). 재RE 필요


// ════ disc18/19(진짜 넥서스 AttackNexus/DefenseNexus) 완전재현 Phase2-1: 캡처 훅 ════
//   핸들러(0x1c81980/0x1c83700) 프롤로그 replace-wrap: cap_fn(my_capture)가 game 원본(트램폴린)을 호출한 뒤
//   game이 채운 sret Vec<Command>를 덤프 → my 재현과 game==mine 대조 기반. cfg dcap=1일 때만 로깅(기본 관측만).
//   두 핸들러 유일 caller=disp3 단일콜사이트(§11.9.5)라 프롤로그 wrap=배타적. RNG-free라 재sim 무영향.
const RVA_DISC18_HANDLER: usize = 0x1c7ca20;   // ★0.5.1(was 0.5.0_3 0x1c81980). AttackNexus 핸들러(disp3 idx16). 프레임 0x5f8→0x608. ⚠내부오프셋 재도출(task③)

const RVA_DISC19_HANDLER: usize = 0x1e0ddb0;   // ★0.5.1(was 0.5.0_3 0x1c83700). DefenseNexus 핸들러(disp3 idx17). 프레임 0x638→0x648(+0x10). ⚠내부오프셋 재도출(task③). byte-patch 사이트=apply_disc19_imm 반영완


// ══ ★07-11 크래시대책②: itemnet 빌드 스코어러 NULL-모델 가드 (§12.23 튜토리얼/에이전트부재 AV 원천차단) ══
//   대상 FUN_141b78420(0.5.0_3): 시그 fn(model, ctx, build_ptr, build_len, flag)->f32. owner(팀AI블롭) NULL이면
//   최상위 콜러가 lea로 model=owner+0x1558≈0x1558 저주소를 3단계 관통시켜 0x141b784a1 `MOV RAX,[RBX+0x10]`서 결정적 AV.
//   가드 = rcx(model) 범위체크 [0x10000,2^48) — 순수 레지스터 검사(syscall/락/패닉 0)라 rayon 워커 포함 어느 스레드서도 안전(§6).
//   BAD면 push8 되감고 xmm0/xmm1=0(score 0.0=최저점) ret — 이 경로는 바닐라라면 크래시라 어떤 일관값도 안전.
//   ★설치 위치 = fn+12(push8 프롤로그 직후): scrim itemnet_addr_valid()가 첫 12B를 검증하므로 프롤로그는 불변 유지(모드 공존).
//   fn+12의 15B(sub rsp,0xd8 7B + lea rbp,[rsp+0x80] 8B, rip-rel無)를 byte-exact 검증 후 변위 — 불일치=미설치(패치판 대응 필요).
const RVA_ITEMNET_SCORER: usize = 0x1b78420;   // 0.5.0_3 (scrim ITEMNET_FORWARD_RVA와 동일 함수; scrim은 call만, 훅은 우리뿐=이중후킹 아님)

const RVA_C8C_DMG_SHEET: usize = 0x3830c58;   // ★0.5.1(was 0.5.0_3 0x380d138). 공격측 데미지시트 desc{vt=0x141c69300, 0x6a8}. ghidra-re 높음(combat dmg rdx-desc 클러스터 연속성 0x141c6~ce 유일 대역, 48↔58회). base쌍 게터 r9=probe_basedmg_r9(d19thr 게이트)

// disc7 위협 엔티티 e가 self(se)를 때리는 DPS(실데미지×1000/공속). my_c8c520 (b)루프(L9733~) 패턴 재사용.
const RVA_DISC7_DMG_SHEET: usize = 0x3846328;   // ★0.5.1(was 0.5.0_3 0x38503b0). "받는 데미지" 시트 desc{vt,0x6a8}. ghidra-re 확정(삼중확증: 총참조 43↔43·rdx 클러스터 disc핸들러 동행이동·0.5.1 disc19서 FUN_14230ee30(u+0x478,...,&UNK_143846328) 실사용)
