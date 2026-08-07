# INDEX.md 반영 패치 (내용 기반) — 2026-08-07

## A. 치환 29건 — OLD 줄을 NEW 줄로 교체
### A1 (원 290행)
OLD: - ★itemnet beam **0x10591f0**·매처 0x2155a90·경기형성 0x182d3c0·후보수집 0xe4d570/폴백 0xe51a50·술어 0x1095650 (0.5.3, 07-31) | ⛔채점 0x1bc8cd0=인라인 소멸 | `MEM\tfm2-item-build-recommender.md` §2
NEW: - ★itemnet beam **0x145b090**·매처 0x1e76c50·경기형성 0x2123590·후보수집 0xedf7f0/폴백 0xee3cd0·술어 0x1496cb0 (0.5.4 재핀 2026-08-07 / 구값 07-31) | ⛔채점 0x1bc8cd0=인라인 소멸 | `MEM\tfm2-item-build-recommender.md` §2

### A2 (원 300행)
OLD: - LineGankerPlan line+0x28·setup_limit+0x18·wait_limit+0x20·phase+0x29·Debug impl 0xd8e800 (0.5.3, 08-03) | 필드 오프셋(Debug 디컴) | `REPORT\tfm2_ai_adjust\RE\2026-08-03_Strategy-소비처-전수맵-라인개입분기-0.5.3.md`
NEW: - LineGankerPlan line+0x28·setup_limit+0x18·wait_limit+0x20·phase+0x29·Debug impl 0xe0fc80 (0.5.4 재핀 2026-08-07 / 구값 08-03) | 필드 오프셋(Debug 디컴) | `REPORT\tfm2_ai_adjust\RE\2026-08-03_Strategy-소비처-전수맵-라인개입분기-0.5.3.md`

### A3 (원 301행)
OLD: - runner.rs 0xebb9f0 = 경기시작 Strategy 24B 복사(+0x1568/+0x1be8) (0.5.3, 08-03) | 경기시작 주입 사이트 후보 | `REPORT\tfm2_ai_adjust\RE\2026-08-03_Strategy-소비처-전수맵-라인개입분기-0.5.3.md`
NEW: - runner.rs 0x13b85b0 = 경기시작 Strategy 24B 복사(+0x1568/+0x1be8) (0.5.4 재핀 2026-08-07 / 구값 08-03) | 경기시작 주입 사이트 후보 | `REPORT\tfm2_ai_adjust\RE\2026-08-03_Strategy-소비처-전수맵-라인개입분기-0.5.3.md`

### A4 (원 302행)
OLD: - gk_* 14사이트 = gk_wait 0xe0237d/0xe02cba/0xd45967/0xd557ae/0xd55cda(각+4B짝)·gk_hp 0xe01e53·gk_margin 0xe01ef7/0xe01f9e/0xe020d4 (0.5.3, 08-03) | 갱셋업 레버·⬜인게임 미검증 | MIGRATION §7.3 §12.17 + `REPORT\tfm2_ai_adjust\RE\2026-08-03_갱셋업-타이밍상수-사이트표-0.5.3.md`
NEW: - gk_* 14사이트 = gk_wait 0xe642a5/0xe64d88/0xe8e89e/0xea1a10/0xea20d5(각+4B짝)·gk_hp 0xe63d92·gk_margin 0xe63e26/0xe63ecc/0xe63ff5 (0.5.4 재핀 2026-08-07 / 구값 08-03) | 갱셋업 레버·⬜인게임 미검증 | MIGRATION §7.3 §12.17 + `REPORT\tfm2_ai_adjust\RE\2026-08-03_갱셋업-타이밍상수-사이트표-0.5.3.md`

### A5 (원 303행)
OLD: - team_plan **0xcf8b90**(wav!=0→+0x3f5=0xFF·LAB_cf9688) · 선택기 **0xe29980**(wav==0만) (0.5.3, 08-03) | ⛔~~마스터게이트~~=실측 반박·⬜write경로 미지 | MIGRATION §7.3 §12.19(2)(정정) + §12.18(1)
NEW: - team_plan **0xd4aae0**(wav!=0→+0x3f5=0xFF·LAB_cf9688) · 선택기 **0xdbf440**(wav==0만) (0.5.4 재핀 2026-08-07 / 구값 08-03) | ⛔~~마스터게이트~~=실측 반박·⬜write경로 미지 | MIGRATION §7.3 §12.19(2)(정정) + §12.18(1)

### A6 (원 311행)
OLD: - ~~★SubPlan13 생성 0xc6e4aa·SubPlan15 0xcb2570~~ → **sub_plan 값이라 disc13/15와 무관**·plan13/15 생산자 0건 (0.5.3, 08-03 밤 정정) | ⛔사장 변이·재시도금지 | MIGRATION §7.3 §12.20(2)
NEW: - ~~★SubPlan13 생성 0xd8beba·SubPlan15 0xe20150~~ → **sub_plan 값이라 disc13/15와 무관**·plan13/15 생산자 0건 (0.5.3, 08-03 밤 정정) | ⛔사장 변이·재시도금지 | MIGRATION §7.3 §12.20(2)

### A7 (원 314행)
OLD: - ec_* 5키 게이트 0xc6e451/0xcb2735(jne)·판정헬퍼 0xe2a540 (0.5.3, 08-03) | bld 스플릿+lane 전용·모이기/유연 死 | MIGRATION §7.3 §12.18(4)
NEW: - ec_* 5키 게이트 0xd8be61/0xe20315(jne)·판정헬퍼 0xdc0000 (0.5.4 재핀 2026-08-07 / 구값 08-03) | bld 스플릿+lane 전용·모이기/유연 死 | MIGRATION §7.3 §12.18(4)

### A8 (원 315행)
OLD: - 전술별 카피상수: engage 0x46 bat≠0 **0xd5c1b0**/bat==0 **0xd5d58a** · epic bld 3-way **0xcff440** · def cmove **0xdf80fd** · gb reach **0xe08858**(fin==0) (0.5.3, 08-03) | 신규 개별 튜닝 레버 후보 | MIGRATION §7.3 §12.18(5)
NEW: - 전술별 카피상수: engage 0x46 bat≠0 **0xea60ca**/bat==0 **0xea6146** · epic bld 3-way **0xd52fd5** · def cmove **0xce115d** · gb reach **0xdcd2d7**(fin==0) (0.5.4 재핀 2026-08-07 / 구값 08-03) | 신규 개별 튜닝 레버 후보 | MIGRATION §7.3 §12.18(5)

### A9 (원 325행)
OLD: - disc2·disc8 movepri JT = 0xc55a34 상수 SubPlan7 · disc4 생성 0xd8065a(전술 read 0) (0.5.3, 08-03) | d8_slot_thr 도달불가 확증 | MIGRATION §7.3 §12.18(4)
NEW: - disc2·disc8 movepri JT = 0xe145f7 상수 SubPlan7 · disc4 생성 0xcd57aa(전술 read 0) (0.5.4 재핀 2026-08-07 / 구값 08-03) | d8_slot_thr 도달불가 확증 | MIGRATION §7.3 §12.18(4)

### A10 (원 337행)
OLD: - recall.rs 0xcb1a80 우물좌표 즉치 32000/928000·jungle camp4/5 우회 latch=SubPlan +0x11(리셋 dfff1b) (0.5.3, 08-03) | 우물행·한바퀴 원인 | MIGRATION §7.3 §12.23(2)
NEW: - recall.rs 0xdabbc0 우물좌표 즉치 32000/928000·jungle camp4/5 우회 latch=SubPlan +0x11(리셋 dfff1b) (0.5.4 재핀 2026-08-07 / 구값 08-03) | 우물행·한바퀴 원인 | MIGRATION §7.3 §12.23(2)

### A11 (원 339행)
OLD: - 핸들러 공통 헬퍼 0xc6efd0(grow)·0xc9c770(extend)·**0xc365a0**(최종 후처리) (0.5.3, 08-03) | 0xc365a0 ⬜미조사 | MIGRATION §7.3 §12.23(3)
NEW: - 핸들러 공통 헬퍼 0xca4f20(grow)·0xd739e0(extend)·**0xc7fec0**(최종 후처리) (0.5.4 재핀 2026-08-07 / 구값 08-03) | 0xc7fec0 ⬜미조사 | MIGRATION §7.3 §12.23(3)

### A12 (원 341행)
OLD: - plan7 HP 게이트 imm 0xdffebf(21)·0xdfff03(41) / line imm ca2b76·ca2e0f·ca3766·c57ed3계열·c57dcf (0.5.3, 08-03) | ⚠d7_repl 대체 여부 확인 | MIGRATION §7.3 §12.23(5)
NEW: - plan7 HP 게이트 imm 0xe621d7(21)·0xe621f4(41) / line imm ca2b76·ca2e0f·ca3766·c57ed3계열·c57dcf (0.5.4 재핀 2026-08-07 / 구값 08-03) | ⚠d7_repl 대체 여부 확인 | MIGRATION §7.3 §12.23(5)

### A13 (원 352행)
OLD: - ★AI 자체 갱 개시 = **0xd4c873**(mode==None일 때만·+0x420 gank_start_tick) (0.5.3, 08-03) | 갱 시작 게이트 | ↑같은 RE
NEW: - ★AI 자체 갱 개시 = **0xe96bbc**(mode==None일 때만·+0x420 gank_start_tick) (0.5.4 재핀 2026-08-07 / 구값 08-03) | 갱 시작 게이트 | ↑같은 RE

### A14 (원 367행)
OLD: - ★경매 오버라이드 = **pending_global_ult_target** unit_ai+0x4F8/+0x500/+0x508·기록 chat.rs 0xd562c8·전제 8개·커밋 99999+태그 0x12(Ult) (0.5.3, 08-05) | "플레이어 지시" 가설 기각 | ↑같은 RE(경매 3단)
NEW: - ★경매 오버라이드 = **pending_global_ult_target** unit_ai+0x4F8/+0x500/+0x508·기록 chat.rs 0xea2996·전제 8개·커밋 99999+태그 0x12(Ult) (0.5.4 재핀 2026-08-07 / 구값 08-05) | "플레이어 지시" 가설 기각 | ↑같은 RE(경매 3단)

### A15 (원 380행)
OLD: - ★record 0x1093b50 2번째 인자=**&World**(0xEEC8)·MVP=팀내 rating argmax @0x19740f0·MatchResult(0xce0)=[TeamMatchInfo(0x670);2]·won=+0x668+team*0x670 (0.5.3, 07-31) | ⚠MatchResult엔 KDA·스코어 없음 | `MEM\tfm2-item-build-recommender.md` §3
NEW: - ★record 0x1493eb0 2번째 인자=**&World**(0xEEC8)·MVP=팀내 rating argmax @0x23b4730·MatchResult(0xce0)=[TeamMatchInfo(0x670);2]·won=+0x668+team*0x670 (0.5.4 재핀 2026-08-07 / 구값 07-31) | ⚠MatchResult엔 KDA·스코어 없음 | `MEM\tfm2-item-build-recommender.md` §3

### A16 (원 386행)
OLD: - item_tactics mid-func 0.5.3 = owned_cap 0xf24a39(imm 0xf24a40)·gate3 0xd0c9be(jbe 0xd0c9c4)·retaddr 0x9a3287/0x9a7b03/0x1925f12 | 패턴 재탐색·오프셋 이식 아님 | MIGRATION §7.3 §11.2
NEW: - item_tactics mid-func 0.5.3 = owned_cap 0x1420b29(imm 0x1420b30)·gate3 0xe76b1e(jbe 0xe76b24)·retaddr 0x9a3287/0x9a7b03/0x235c382 | 패턴 재탐색·오프셋 이식 아님 | MIGRATION §7.3 §11.2

### A17 (원 403행)
OLD: - athlete 0.5.3 불변 확정: id **+0x810**·side +0x820·items +0x448/50/58·build +0x490/98/4a0·gold +0x888·pos +0x8b0·stride 0x8d0 | ctor 0xed32b0 3연속스토어·fix B 성립 | MIGRATION §7.3 §11.3
NEW: - athlete 0.5.3 불변 확정: id **+0x810**·side +0x820·items +0x448/50/58·build +0x490/98/4a0·gold +0x888·pos +0x8b0·stride 0x8d0 | ctor 0x13cf550 3연속스토어·fix B 성립 | MIGRATION §7.3 §11.3

### A18 (원 404행)
OLD: - Game +0x1dc0 provider ptr/+0x1dc8 vtable (0.5.3 유지)·vt슬롯+0x20=[rcx+0xeaf8] | launcher 0xeb9646·독립증명 | MIGRATION §7.3 §11.3
NEW: - Game +0x1dc0 provider ptr/+0x1dc8 vtable (0.5.3 유지)·vt슬롯+0x20=[rcx+0xeaf8] | launcher 0x13b6206·독립증명 | MIGRATION §7.3 §11.3

### A19 (원 407행)
OLD: - serpen 0.5.3 훅 = SERPEN 0x1535810·MOBATICK 0xeeeac0·SPAWN 0xabdf60/0xabd340·RENDER_STEP 0x960df0·DMGA 0xfdbbb0·DMGB 0x12c3bb0·KEYRES 0x1b0aba0·ARG_STR 0x1228a90 | ★인게임 12/12 검증완 | MIGRATION §7.3 §13.1·§13.6
NEW: - serpen 0.5.3 훅 = SERPEN 0x1328950·MOBATICK 0x13ee0a0·SPAWN 0xabdf60/0xabd340·RENDER_STEP 0x960df0·DMGA 0x10670a0·DMGB 0x14eaef0·KEYRES 0x218be90·ARG_STR 0x16a31e0 | ★인게임 12/12 검증완 | MIGRATION §7.3 §13.1·§13.6

### A20 (원 409행)
OLD: - serpen 0.5.3 런처 게이트 = LAUNCHER_RET_B 0x9a7b03(화면경기 1회) vs 배경리그 0x220acb/0x20dac9c/0x195c5be | 인게임 분류 적중 | MIGRATION §7.3 §13.6
NEW: - serpen 0.5.3 런처 게이트 = LAUNCHER_RET_B 0x9a7b03(화면경기 1회) vs 배경리그 0x220acb/0x19b9a4c/0x239f242 | 인게임 분류 적중 | MIGRATION §7.3 §13.6

### A21 (원 611행)
OLD: - ★0x1bf3dd0(0.5.3 신설·콜러23) / ⛔0x1cd9380=0.5.3 소멸 | scene_step 단계enum 0밴1픽2완료0xff | MIGRATION §7.3 §14
NEW: - ★0x1dad900(0.5.3 신설·콜러23) / ⛔0x1cd9380=0.5.3 소멸 | scene_step 단계enum 0밴1픽2완료0xff | MIGRATION §7.3 §14

### A22 (원 612행)
OLD: - 0.5.3 phase 인라인 복제본 30개(0.5.2=11)·미보정 ~20 = 0x1c55300×7·0x188dd30×2·0x1890450×2·0x1bd3960×2 등 | 전수=_bo_sites_053.json | MIGRATION §7.3 §14.4
NEW: - 0.5.3 phase 인라인 복제본 30개(0.5.2=11)·미보정 ~20 = 0x1e19640×7·0x215e050×2·0x2160680×2·0x1d8d250×2 등 | 전수=_bo_sites_053.json | MIGRATION §7.3 §14.4

### A23 (원 615행)
OLD: - ⛔칸채움색 아닌후보5(0.5.3): 0x1c252c0(호출0회)·0x1bf9560(순수렌더러)·0x1c1f270(라벨세터)·Runner+0x13c·scene+0x3d0(팀1ID고정) | 재시도금지 | MIGRATION §7.3 §14.5
NEW: - ⛔칸채움색 아닌후보5(0.5.3): 0x1ddff30(호출0회)·0x1db2f30(순수렌더러)·0x1dd9f00(라벨세터)·Runner+0x13c·scene+0x3d0(팀1ID고정) | 재시도금지 | MIGRATION §7.3 §14.5

### A24 (원 617행)
OLD: - 0x20733a3(0.5.3) | Runner+0x13c 초기화 콜사이트 | MIGRATION §7.3 §14.5(3)-4
NEW: - 0x20b20e3(0.5.3) | Runner+0x13c 초기화 콜사이트 | MIGRATION §7.3 §14.5(3)-4

### A25 (원 620행)
OLD: - ★밴픽 자동행동 펌프체인(0.5.3): 드레인 0x1c55300→scene_step 인라인 0x1c5a02a~(디스패치 0x1c5a0b2=drainA·합류 0x1c5a288)→트리거 0x1bf77d0(disc 0x93 큐잉)→서버 CP디스패처 0x17e0240→0x17e659f call 0x1827e00(AI턴 유일 콜사이트) | 코치위임 펌프체인 | MIGRATION §7.3 §14.6
NEW: - ★밴픽 자동행동 펌프체인(0.5.3): 드레인 0x1e19640→scene_step 인라인 0x1e1e32a~(디스패치 0x1e1e3b2=drainA·합류 0x1e1e588)→트리거 0x1db11a0(disc 0x93 큐잉)→서버 CP디스패처 0x20d5bf0→0x20dc175 call 0x211dd40(AI턴 유일 콜사이트) | 코치위임 펌프체인 | MIGRATION §7.3 §14.6

### A26 (원 622행)
OLD: - 0x182813f(0.5.3·서버 AI턴 0x1827e00 조기리턴: 0x1828309 cmp rdx,rbx 팀 불일치→[rsi]=-1→0x17e65a5 cmp -1) | 요청무산=응답없이 소비 | MIGRATION §7.3 §14.6 후속
NEW: - 0x211e07f(0.5.3·서버 AI턴 0x211dd40 조기리턴: 0x211e245 cmp rdx,rbx 팀 불일치→[rsi]=-1→0x20dc17b cmp -1) | 요청무산=응답없이 소비 | MIGRATION §7.3 §14.6 후속

### A27 (원 692행)
OLD: - server_pregate 0x17ef5f6(04→ff)·inc_gate 0x17f239c(→ff)·dr_inline_b 0x18e3fd6 (0.5.3) | ★일일제한 실효 3점=진범 서버게이트 | MIGRATION §7.3 §15
NEW: - server_pregate 0x20e5471(04→ff)·inc_gate 0x20e8246(→ff)·dr_inline_b 0x2310c86 (0.5.3) | ★일일제한 실효 3점=진범 서버게이트 | MIGRATION §7.3 §15

### A28 (원 693행)
OLD: - ~~dr_inline_a/c/d~~ a 0x18d9436·c 0x18f18c7(폐기)·d 0x1987a3d (0.5.3) | 게이트 아님(a/d표시·c시드) | MIGRATION §7.3 §15
NEW: - ~~dr_inline_a/c/d~~ a 0x2306164·c 0x18f18c7(폐기)·d 0x23ce6bc (0.5.3) | 게이트 아님(a/d표시·c시드) | MIGRATION §7.3 §15

### A29 (원 694행)
OLD: - daily 레코드 count +0xdc10·rec_id +0xdc1c·outer_id +0xe434 / 서버 gate 0x17f239c(+0x1d0/+0x1dc) (0.5.3) | 일일횟수 필드·서버게이트 | MIGRATION §7.3 §15
NEW: - daily 레코드 count +0xdc10·rec_id +0xdc1c·outer_id +0xe434 / 서버 gate 0x20e8246(+0x1d0/+0x1dc) (0.5.3) | 일일횟수 필드·서버게이트 | MIGRATION §7.3 §15

## B. 아카이브 이동 97건 — INDEX 에서 제거하고 `ANA\_archive\INDEX-0.5.2-rva.md` 에 추가
MOVE: - 모드관리 렌더 FUN_14080df80(0x80df80) / collect FUN_140590d80(0x590d80·라이브 0x678210) (0.5.2) | 모드행 per-row 인스턴스화·mod_id ASCI… | tfm2-mod-order-mod §RE정본
MOVE: - 모드정렬 삽입 FUN_1406419b0/live0x86d250·대형 FUN_14084f240/live0x44e6e0·cmp FUN_140535010/live0x48c670 (0.5.2) | mod_id memcmp 콜사이트정렬… | tfm2-mod-order-mod §RE정본
MOVE: - 모드레지스트리 HashMap SceneState+0x1ba00(Ghidra)/+0x1c8c0(live)·버킷stride0xf0·ModEntry name+0x18/author+0x30 (0.5.2) | enabled=mods.json enabled_mod… | tfm2-mod-order-mod §RE정본
MOVE: - EntityInfoEventData ser 0x2324c60(테이블 0x1438cc570·+0x58 ult_cooltime·+0xb4 can_use_ult)·nearest 0x2324c10·궁diff 0x2324bc0 (0.5.2) | 관전스냅샷 남은쿨·max없음 | tfm2-entity-cooldown-ult §3
MOVE: - champion_info_ui 0x36eac54(cooltime.text 0x37379ba/icon 0x373882e)·object_info 토글 0xcb3190/탭 0xc2dc20·클라엔티티표 0xd43760 stride0xe8 (0.5.2) | 궁쿨 UI 노출·바인딩 | tfm2-entity-cooldown-ult §4
MOVE: - 착탄 함수 0x21ff390(A@0x22022ca/B@0x22022f4) (0.5.2, 구 0.5.1=윗줄) | 0.5.2 신주소·B는 sete dil로 인코딩 변경 | MIGRATION §7.2
MOVE: - tfm2_fog_damage_fix 5곳 (0.5.2) @0x22022ca/@0x22022f4(sete→mov1)+@0x201c274/@0x2019aa4/@0x2005085(jne→NOP6) | ✅배포완·⬜인게임검증·완화 재시도금지 | MIGRATION §7.2 + tfm2-vision-fog-in-ai
MOVE: - ★레벨업 0x22d3c60 / 상한게이트 ja@0x22d3ff4(⛔NOP금지) / panic가드 jae@0x22d4001 / UI expbar 0x80b6eb / ★levelcap훅 0x22d3fea·0x80ae73 (0.5.2) | 레벨상한=len+1·S1불가·런타임강제 | discovered-PROGRAM-STRUCTURE §13.2-levelcap + MIGRATION §7.2-A12
MOVE: - 데미지 코어 3형제 0x1dbb2a0/0x1db9f50/0x1db47a0 (0.5.1) → 0.5.2 0x201c1b0(native ult/스킬 `attack.rs:141`)/0x20199e0(data 평타 `:459`)/0x2004fc0 | 바이트 쌍둥이=panic-Location으로만 구분 | discovered-PROGRAM-STRUCTURE §15.2 + MIGRATION §7.2
MOVE: - 0.5.2 신설 AI 교전 타겟 필터 0x2367c20(시야게이트 @0x2367c3f)·콜러 0x20e4600 engage.rs@0x20e894d (0.5.2) | 데미지 무관·fog모드 의도적 미패치 | tfm2-0.5.2-migration + DONE.md
MOVE: - RangePeriodProjectile per-period 루프 프리헤더/게이트 0x142200478·JT 0x1422004a6(jmp r9)·필터 FUN_142218d80 (0.5.2) | casting None/Position=매period… | tfm2-teemo-standalone-mod §1
MOVE: - SwitchByBuff.apply 0x2020d60·RangePeriodProjectile.apply 0x1b3cee0(*param_7==2 필수)·Delayed.apply 0x1e27720 (0.5.2) | 조건분기 spawn 컨텍스트 4-arg 드롭… | tfm2-teemo-standalone-mod §2·§3
MOVE: - 런처 0x1d96870 계약·콜사이트9·World ctor 0x22c1da0·r9d→World+0xeae9 (0.5.2) | dl=[Db+0x738]·세트인덱스 부재 확정 | discovered-PROGRAM-STRUCTURE §2d
MOVE: - 런처 FUN_141d96870 r8=seed→[World+0xeab8]salt=buy is_live판별자·구성site FUN_140d405c0(seed[param_3+0x258]·로스터[OBJ+0x9410]→arg5 0x1548)·변형 FUN_14151dc60/0x151e3f4 (0.5.2)·★런타임 실측 07-28: launch 110건 중 중복seed 1개=내경기(배경 idx103→관전 idx104 동일 seed·2세션 재현)·⛔스택인자 blob athlete_id 스캔=오탐(v3 07-28·0x15d8 고정) | 경기identity앵커·seed식별확증·blob스캔오탐 | tfm2-item-slot-count 진범절 + discovered-PROGRAM-STRUCTURE §2d
MOVE: - ~~현재경기 단일슬롯 `[mgmt+0x1cd88]`·`*(+8)`=0x268레코드(team +0x228/+0x240·seed +0x258·rule +0x260)=프리리드 가능~~ → ⛔**런타임 반증(0.5.2·07-28 tfm2_match_seed_probe v2)**: record ptr=null(밴픽·관전)·홀더4종 쓰레기·mgmt 0..0x20000 스캔 seed부재 | 0x268프리리드=런타임반증·재시도금지 | tfm2-item-slot-count 진범절 + discovered-PROGRAM-STRUCTURE §2d③
MOVE: - mgmt tick 훅 FUN_1404025c0=RVA 0x4025c0(rcx=mgmt·12B 8-push 프롤로그)·라이브체인 …←FUN_14046d920←FUN_1404025c0←FUN_1405643a1·seed소스 FUN_140d405c0[p3+0x258]←memcpy FUN_14074d510 (0.5.2) | 앵커함수 생존·양씬 발화 실측 | tfm2-item-slot-count 진범절 + reimpl 07-28
MOVE: - 런처 retaddr 런타임 히스토그램: 0x1306ed0 x53(solo)·0xd64af1 x52(배경리그sim)·0x1d9ef07 x4(래퍼)·0x75e5cf x1(관전=화면경기) (0.5.2·07-28 실측) | static xref "5개뿐" 반증·구채록유효 | tfm2-item-slot-count 진범절 + reimpl 07-28
MOVE: - seed 재사용 경로 0x1306ed0(FUN_1412fb150 콜러7곳)·화면 0x759c31/0x75e5ca (0.5.2) | Bo세트 seed 공유 후보 3경로 | discovered-PROGRAM-STRUCTURE §2d③
MOVE: - 0x1dac1f0 · 0x2388fd0 · 테이블 0x38d72b4(8엔트리)/0x38d72d4(41) (0.5.2) | MP 실행기(idx=tag-2, tag0/1/9→7)… | MIGRATION §7.2-A3 A
MOVE: - 0x1dc88b0 / 0x1dca940 / 0x1dcb750 (0.5.2) | vt+0x30 3-inst leaf… | MIGRATION §7.2-A9 §1
MOVE: - 0x1421266bf · 0x1421269b7 / 0x1423a5a60 (0.5.2) | f22e80 mode 비교는 `!=2` 2곳뿐… | MIGRATION §7.2-A9 §3
MOVE: - 0x141b78380·0x141b18f30 / 0x141b190a0 (0.5.2) | dd7 engage RNG=로컬스택·전역0… | MIGRATION §7.2-A9 §3
MOVE: - 0x1dc8b80 (0.5.2) | dd7700 `s20`=vt+0x28=`i64[sim… | MIGRATION §7.2-A9 §4
MOVE: - 0x2391ed0 (0.5.2) | disc10 원본·`dd7_slot128`… | MIGRATION §7.2-A7 §2·§5
MOVE: - serpen.rs:431 / 0x23b5fe5 (0.5.2) | `gchild+0xeaf0`이 정답… | MIGRATION §7.2-A9 §5
MOVE: - 0x238f289 · 0x238f215/f237/f3b3/f586/f5a4/fa71 (0.5.2) | picker 호출=헤드 무조건1회… | ANA\disc12-epiccheck-tail-spec.md §D·§F
MOVE: - 0x238f81b · 0x238f971 · 0x238f9cd/f9d8 (0.5.2) | disc12 가시성 side 고정 2사이트… | ANA\disc12-epiccheck-tail-spec.md §G
MOVE: - 0x24eaf10 · 0x24eb9e0/0x24ec170/0x24ec920/0x24ed010/0x24eaad0 · 0x36e7480/0x36e74c0 (0.5.2) | ChaCha refill 디스패처·백엔드4+1… | ANA\disc12-epiccheck-tail-spec.md §J-1
MOVE: - 0x38acf40 · 0x38acf18(=0x38d6c70) · 0x38a60b3/0x38a60b5 (0.5.2) | engage_gate REQUIRED[9]… | ANA\disc12-epiccheck-tail-spec.md §L-2
MOVE: - 0x38a3350 · 0x38b25d0/0x38b2700/0x38b2830/0x38b2960 · ⬜0x2c77e18 (0.5.2) | picker 오브젝티브 vtable 5종 / 위임형… | ANA\disc12-epiccheck-tail-spec.md §K
MOVE: - 0x2b9af53 (0.5.2) | memcmp(picker 이름비교 — len 선비교… | ANA\disc12-epiccheck-tail-spec.md §K-3
MOVE: - 0x1daf6d0 movepri소비처 / 0x2101740 넥서스판정(serpen_poke.rs 내부·독립함수아님) (0.5.2, 07-29) | 완전결정적(해시·FP·RandomState… | ANA\ai_adjust-rng-desync-전수조사.md §8.2
MOVE: - ⬜0x1dbad70 fight_check TLS 메모캐시(TLS+0x1e8 RefCell<HashMap<String,[Res;3]>>·해시 0x7bf840) (0.5.2, 07-29) | ⬜순서민감 후보(키=이름뿐) | ANA\ai_adjust-rng-desync-전수조사.md §8.3
MOVE: - disc13 0x238fdd0(JT[11]) · disc15 0x2390160(JT[13]) (0.5.2) | ⛔죽은 틀=편입 금지(실익0)·인자/write-set… | MIGRATION §7.2-A10 + 100퍼-잔여-트래커 #0e
MOVE: - 0x238fe7f~fe86 (0.5.2) | disc13 이중 역참조([rdx+0x1a0]→[ra… | MIGRATION §7.2-A10 §2
MOVE: - ~~0x21a45dc~0x21a4b54 / 0x21ee085~ / 0x231cd04 (0.5.1)~~ | ⛔stale·0.5.2 재핀=위 A11 §3줄 | MIGRATION §7.2-A11 §3
MOVE: - serpen 리플레이 콜사이트 0x1555210(핸들러 0x1554930)·RET_C 0x1555215 (0.5.2) | 리플레이 LIVE_SEED 게이트(v0.4.1) | ANA discovered-serpen §21b
MOVE: - 0xf192c0 | UI러너 레지스트리 등록 54종(0.5.2) | tfm2-banpick-showcase
MOVE: - 0x124db10 | 밴픽 쇼케이스 렌더 본체(0.5.2) | tfm2-banpick-showcase
MOVE: - 0x11e2140 / 0x11e2370 | 셀렉트확정→연출상태 세팅(0.5.2) | tfm2-banpick-showcase
MOVE: - 0x11f9030 | 쇼케이스 카드 헬퍼·계약 규명완(0.5.2) | tfm2-banpick-showcase §5
MOVE: - 0xfdabe0 | 밴픽 일러 에셋 조회·훅 seam(0.5.2) | tfm2-banpick-showcase §5a
MOVE: - 0x99c860 | 키→텍스처 trait 객체 조회(0.5.2) | tfm2-banpick-showcase §5a
MOVE: - 0x121aca0 | ~~캐시 렌더타깃~~→순수 계산기 정정(0.5.2) | tfm2-banpick-showcase §11
MOVE: - 0x5ab7d0 | 키→애님 리소스 조회(0.5.2) | tfm2-banpick-showcase §11
MOVE: - tag4 +0xad/+0xae(0x9f3090) | 스프라이트 flip_x/y 반전(0.5.2) | tfm2-banpick-showcase §11
MOVE: - 0x1217630 | 텍스트테이블 챔프 표시명 획득(0.5.2) | tfm2-banpick-showcase §5
MOVE: - 0x248b690 / 0x248b730 | 렌더타깃 스냅샷 열기·명명등록(0.5.2) | tfm2-banpick-showcase §9
MOVE: - 0x1201d90 | 밴 분할카드 독립 드로우 ×2(0.5.2) | tfm2-banpick-showcase §9
MOVE: - 0x3731380 / 0x37313b0 | 카드rect 중심 vs 좌상단 원점(0.5.2) | tfm2-banpick-showcase §8
MOVE: - 0x11dd200(vt 0x144101080) | 밴픽 로스터재구성·phase공유(0.5.2) | ANA discovered-banpick-ai §17i-0
MOVE: - 0x11e3000 | 밴픽 진행 상태머신 al=0xff/2(0.5.2) | ANA discovered-banpick-ai §17i-0
MOVE: - 0x11fdb00(vt 0x144101da0) | 밴픽 완료판정 셀렉터(0.5.2) | ANA discovered-banpick-ai §17i-0
MOVE: - 0xebe530 / 0xefef00 / 0xefff70 | 서버 AI턴·추천 ban/pick(0.5.2) | ANA discovered-banpick-ai §16
MOVE: - 0x1d075d0 / 0x1d07cf0 | 밴픽 턴팀 헬퍼(0.5.2) | ANA discovered-banpick-ai §16
MOVE: - 0x11df9f0 | 밴→픽 배너/전이 세팅 유일점(0.5.2) | ANA discovered-banpick-ai §17
MOVE: - 0x1c041c0/0x1c07880/0x1c051a0/0x1c07ec0 | 밴픽 스코어러4·phase인라인복제(0.5.2) | ANA discovered-banpick-ai §17
MOVE: - phase JT 0x14370f740(4-arm 픽팀0 0xd4fff3/픽팀1 0xd51933/밴팀0 0xd51737/밴팀1 0xd51817/종료 0xd51c35·팀parity만·total재계산無append)·⚠직전 0x143710740=+0x1000 stale정정 / phase내부식 total=Σ4벡터len+ban×2(+0xf0)+rule·pick_index=total−ban×2·pick_table@0x38397ba (0.5.2) | 크래시 진범=0x11cedb0 unwrap(None) | ANA discovered-banpick-ai §17i-0·§17i
MOVE: - 배정 다운스트림(순서무관=해결책A근거): 로스터빌드 콜러 0x1413f9170·팀라인업 0x140ebe530·라인업조립 0x1a26690(index위치순회·pick_index無)·챔프→로스터 0x20189d0(이름memcmp)+memcpy 0x142063640·set builder 0x142009740 (0.5.2) | 확정경계(종료arm 0xd51c35 직후~빌드 前)… | ANA discovered-banpick-ai §17i-0
MOVE: - 픽테이블 .rdata 0x38397a8(0.5.2) → **0x3277c70**(0.5.3·28B 동일) | 밴픽 픽순서 테이블 | MIGRATION §7.3 §14.1
MOVE: - 이적협상 0x1cdd4c0(선수응답 메인)·0x1d15e90(이적의향)·0x1d17e60(성향0~1)·0x1d1cb00(즉시수락)·0x1d11990(재정신뢰) (0.5.2) | ★수락/거절 판정식 정본·결정론 | discovered-PROGRAM-STRUCTURE §9a-1
MOVE: - 평가체인 0x1d1e4e0→0x1d1b270(ratio 저장)→0x1d1f720(bool 게이트) · 팀연봉합계 0x1cd7110/개수 0x1cd7550 · AI오퍼발주 0x1cf17b0 · 나이테이블 0x383a900 (0.5.2) | 협상 보조함수·문턱테이블 | discovered-PROGRAM-STRUCTURE §9a-1
MOVE: - 이적의향 성향점수 0x1d17e60 입력 loyalty lookup 0x1d18180/0x1d1a290(game+0x16950/0x16970·Athlete+0x3a8/0x3b0) (0.5.2) | 성향=loyalty관계·팀전력 아님 | discovered-PROGRAM-STRUCTURE §9a-1 이적의향입력
MOVE: - 재정스냅샷 0x234dd60 · 연봉지출합산 0x1a07520 · 팀재정평가 0x1b262c0 · 경쟁오퍼 0x23d98a0 (0.5.2) | 협상 축B/C·분모=현재연봉 | discovered-PROGRAM-STRUCTURE §9a-1
MOVE: - 셀러 이적응답: 수집 0x1ce82c0→판정 0x1e2c420 · 의향 0x1d15410/0x1d15c70 · 지불능력 0x1d11dc0 (0.5.2) | 셀러 스테이지·i64 인코딩 | discovered-PROGRAM-STRUCTURE §9a-1 축D
MOVE: - 이적료가치 0x1d18330(base 0x1d1e950×잔여계수) · 스태프몸값 0x105b150 · 이력표시 0xfddec0 · 문자열화 0x1a71620 (0.5.2) | 가치·스태프몸값·salary표시 | discovered-PROGRAM-STRUCTURE §9a-1
MOVE: - 연말 은퇴/FA/전환 0x1a17690 · 오케스트레이터 0x19f0e40(콜러 0x1409320) · 스태프 계약반영·FA방출 0x1a3c8b0 · 선수측 0x1a43180(추정) (0.5.2) | 스태프 라이프사이클 정본 | discovered-PROGRAM-STRUCTURE §9a-2
MOVE: - create_staffs 0x19df930(초기풀 전용·콜러 0x1cd4560) · add_staff 0x1a70e10(DB에디터 0x13ca5e0뿐) · add_athlete 0x19f8470 · 풀보충 0x1a26e50(선수만) · rand 0x1b19c30/0x1b19f00 · gen_bool 0x1394a90 (0.5.2) | 스폰=선수전환뿐·정기보충無 | discovered-PROGRAM-STRUCTURE §9a-2
MOVE: - Staff 테이블 game+0x16c20(head 0x16c50·id 0x16c60·next +0x1b0) · retirement +0x10/+0x18 · +0x38 팀id(-1=FA) · 계약종료=(+0xa4)>>13 · 계약변경맵 game+0x17290 (0.5.2) | Staff 테이블·은퇴 오프셋 | discovered-PROGRAM-STRUCTURE §9a-2
MOVE: - 선수연봉 산정 본체 0x1d19650(param3능력·나이커브16/21/24/26/32/36·pot^1.25/1.35) · 종합가치 0x1d1e950(능력q 0x1d17870) · AthleteStat 이름표 0x2c602a0 · 뉴스사유매핑 0xf9359a · Team예산산정 0x1cd4c9c (0.5.2) | 연봉base·가치커브·8스탯 | discovered-PROGRAM-STRUCTURE §9a-1
MOVE: - RejectReason deser 0x1b5cdd0(11v·이름표 0x381bb10) · 오퍼트레이스 0x1f4cfa0(게이트 0x406f730·env TFM2_OFFER_TRACE) (0.5.2) | 거절사유 enum·트레이스 로거 | discovered-PROGRAM-STRUCTURE §9a-1
MOVE: - 선수아레나 헤드 game+0x16bc0·next Athlete+0x788(스태프 game+0x16c50/+0x1b0) · 로스터=team_id+0x568 파생쿼리(status+0x10!=2) · 오케 14140ac00→140e643d0 · apply후보4개배제⚠단일진입점미확정 · AI훅 1415a8630(Team+0x72c=AI)·142351a40 (0.5.2) | 트레이드 스코핑·로스터=team_id수술·AI훅… | discovered-PROGRAM-STRUCTURE §9b
MOVE: - transfer_tweak 사이트: rdata 0x3835560 · disp 5곳 0x1d1626b/0x1d162ab/0x1d162db/0x1d162e9/0x1d16340 · detour 0x1d15e90 · 콜사이트 0x1cdf57e/0x1d1257b/0x235f0a7 (0.5.2) | 이적문턱 모드·패치시 재핀 | tfm2-transfer-negotiation §모드
MOVE: - sylas_hijack seam 0x54a800(챔프등록)·0x621a40(clone디스패처)·0x53e886/0x53e652(master0x748복사)·0x63de80·0x23bf20 (0.5.2) | 궁강탈 런타임BP seam(정적마이그死) | tfm2-champion-system §사일러스
MOVE: - sylas 방법2 base effect-action(前"datachamp" 07-29정정) descriptor vtable 0x3876088(쌍둥이 0x3891368·size 0x1a8)·apply slot+0xd0 0x1fe5cc0(=base지 datachamp 아님)·inner exec 0x2004d90·JT 0x389fd84·clone_box 0x1f7a700·drop 0x1f4d670·caster=[sim_state+0x1b8]·type-info 0x142c76278 (0.5.2) | 궁강탈 방법2 base effect seam | tfm2-champion-system §사일러스
MOVE: - sylas datachamp 실경로(base와 분리·07-29): data_driven.rs setup 0x1f5cc90·등록table 0x144174658·native effect ~~apply~~내부 0x1b785d0(=데미지1인스턴스 내부·최상위apply아님 07-29·vtable 0x38beee0)·base setup priest 0x2036410·분기=master0x748유무 (0.5.2) | 방법2 base/datachamp 분리 | tfm2-champion-system §사일러스
MOVE: - sylas 방법2 발화 dispatch FUN_1423a76e0(0x23a76e0)·균일발화·발화=[vt+0xd0](this,arg2,caster+0x5a8)·프롤로그8push+sub0x188·유사0x2118d05/0x23a774e (0.5.2·⬜인게임미검증) | 방법2 궁 발화 dispatch seam | tfm2-champion-system §사일러스
MOVE: - sylas 방법2 슬롯 궁 apply set 0x20b0460=lightning_mage·0x1e60ce0=fighter·0x21dcb20=exorcist·0x1e5e1b0=executioner·0x1fe5cc0=base공용·수렴 executor 0x2004d90 (0.5.2) | 슬롯궁=대상 base챔프 궁 apply | tfm2-champion-system §사일러스
MOVE: - sylas 방법2 option B(슬롯복사)무효=cast-gate 재조립·dispatch FUN_1423a76e0 kind0xd JT@0x1438d7574→핸들러0x1423b5b72(sub-kind재dispatch)·+0x308=grab대상handle·option C(active주입 entity+0x290)채택 (0.5.2·07-29) | 방법2 option판정·cast-gate seam | tfm2-champion-system §사일러스
MOVE: - 0x21a0c10 / ModChampionEntry Debug 0x142021cf8 (0.5.2) | ModItemEntry Debug impl=+0x19… | tfm2-active-items-detection
MOVE: - 0x1408f0870 / 0x96eb00 (0.5.2) | +0x190 독립확증: inactive 엔트리만 도는… | tfm2-active-items-detection
MOVE: - 0x21990e0(구 0x1d90190) / 0x9a6380(구 0x983310) / 0x9a5f30·0x8f05f0(구 0x982ec0) / 0x98ffd0 / 0x217f250 / 0xbcdb0(구 0xb99b0) (0.5.2) | override_info 키조회(⚠"병합코어" 아님)… | tfm2-active-items-detection + tfm2-0.5.2-migration
MOVE: - 0x9b8da0 / 0x9bac00 / 0x2493010 / 0x24974e0 / 0x98da60 (0.5.2) | merge/override 적용 코어·확장자표 | tfm2-asset-override-merge
MOVE: - 0x9b90f0 / 0x9c2210 / 0x24956f0 (0.5.2) | override분기 target검사無·alias신규 | tfm2-asset-override-merge §3
MOVE: - comptest_unlock 0.5.2 바이트패치 14종 | stamina 0xe93b2d·daily 0x1f14… | MIGRATION §7.2
MOVE: - comptest_unlock 0.5.2 훅 16종 | DISP 0xd3f780·RUN 0xd0a440… | MIGRATION §7.2
MOVE: - comptest_unlock 死상수(재핀 불요·inert) | ATH_GET 0x402840(=ATH_GET_SC와… | tfm2-0.5.2-migration
MOVE: - crm ClientDatabase witness 재핀 A 0x75027f(fn 0x74d510)/B 0x79deb7(fn 0x79c330)/C 0x754628 | 오프셋 불변 근거함수(0.5.2)… | MIGRATION §7.2 crm절
MOVE: - 0xc2f990 (0.5.2) | ★★스프라이트 교체 정답 seam=에셋키 리졸버… | ANA discovered-serpen §18.3 + MIGRATION §7.2
MOVE: - 0x13c4e90→0xc33950(0.5.2) / 0xeb0880 / 0xeb0420 (⬜0.5.2 미확정=구값 inert) | 엔티티 드로우 빌더(ex=ent+0x164… | ANA discovered-serpen §18.2
MOVE: - 0x803b30 (드라이버 0x803db0·~~소비루프 0x811500~~→0x811500=1스텝적용·루프=0x14079c330 §2e⑦·jt 0x2c61428) | ★렌더 킬이벤트 적용 tag5/6 (0.5.2) | discovered-PROGRAM-STRUCTURE §2e
MOVE: - 재생 소비/게이트 루프 0x14079c330(게이트 0x1407a0260~480)·1스텝적용 0x811500·프리페치 0xc50840·catchup flag db+0x1768/target db+0x1770·세터 0xd22ce0·결과빌더 0x1407f1b40·컨트롤러 0x506a60 (0.5.2) | 재생루프 정밀·811500 정정 | discovered-PROGRAM-STRUCTURE §2e⑦
MOVE: - UI 노드 option_buttons.view_result(blob 0x142c332c4/0x142c57d08)·매치뷰=재생UI·형제 speed_buttons/pause/exit (0.5.2) | ★「즉시보기」버튼=순수페이싱·⬜클릭1비트 | discovered-PROGRAM-STRUCTURE §2e⑦
MOVE: - 0x4e07f0 (사이트 0x4f0324) | 뷰미러 +0x91e8←Game+0x278 (0.5.2) | discovered-PROGRAM-STRUCTURE §2e
MOVE: - 0x2311dc8 (구 0.5.1 0x2202ac8) | sim 세르펜카운터 inc 0.5.2 재핀 | discovered-PROGRAM-STRUCTURE §2e
MOVE: - 0x231fc30 (래퍼 0x231ef20·core 0x2344e80, 구 0.5.1 0x2210900) | World 딥클론 0.5.2 재핀(재시드 0건) | discovered-PROGRAM-STRUCTURE §2f
MOVE: - 0x872950(★0.5.2 소비루프=0x14079c330 §2e⑦) / 0xd711a0 / 0xd7cef0 / 0x55c8e0 / 0x55ba40 / 0x4b0e70 · 테이블@0x14409df18 | ★재생 tick 아키텍처(0.5.1·0.5.2정정) | ANA discovered-serpen §16.1

## C. 표시만 19건 — 줄 끝에 다음을 덧붙임(주소는 건드리지 말 것)
  덧붙일 문구: `  ⚠0.5.3 주소 — 0.5.4 대응표 = `C:	fm2mods54\map_053_054.tsv``
TAG: - beam W=3 imm32 0x215606a·depth imm8 0x1059a28/0x1059f7d·decay 0.99999 .rdata 0x31e2c50/40·lr 0x317e2f0 (0.5.3, 07-31) | decay만 값패치 안전·N30/C6 | `MEM\tfm2-item-build-recommender.md` §1a·§2
TAG: - sv_pa_* 소극4임계 0xcd4cd7/dd/e3/e9 (0.5.3, 08-03) | sev[C] branchA·✅적용검증완 33/33 | `REPORT\tfm2_ai_adjust\02_구현정보.md` §7b
TAG: - **+0x3f6 = 오브젝티브 ID**(0xFF=없음)·write는 0xcef570~0xcf837b ~40사이트 / +0x3f5는 0xcf8b90 단독 (0.5.3, 08-03) | wav 반박의 기계적 이유 | MIGRATION §7.3 §12.20(4)
TAG: - ★**unit_ai Debug 0xd23cf0·필드명표 `.rdata 0x31B1C30`(70엔트리)** → 필드명 직접 추출(추정 X) (0.5.3, 08-03) | ★오프셋 막히면 Debug부터 | ↑같은 RE
TAG: - 채팅 프로토콜 **46종 이름 전수**(Debug 0xd66d00·JT 0x31b6a20)·수신 디스패처 JT 0x31b6640(index=tag−1) (0.5.3, 08-03) | 팀 커뮤니케이션 전량 | ↑같은 RE
TAG: - 경매 死코드 ≈2,440B = 0xd62163~0xd621bd + 0xd62673~0xd62aae + 0xd62d28~0xd62d4d + 커밋 0xd62ab3 (0.5.3, 08-05) | ⚠원본 기준(0xd61e54 NOP시 부활)… | ↑같은 RE(경매 3단)
TAG: - action_score 골격 = 2단 JT(태그 0x31aa7d0[16] → cat 0x31aa814[10])·반환 rax=rbx+r12(rbx=cat0 전용 +10·타 cat은 0xc7d4ca xor)·수적우위 배율 **cat0 40/75/100/200/300 · cat4 30/60/80/150/200** (0.5.3, 08-04) | 死후보 2건 검증 후 기각 | ↑같은 RE(action_score 후반부) + `…\RE\2026-08-04_action_score-c7b730-전량해독-2단디스패치-死3건-0.5.3.md`
TAG: - ★기법 Rust `#[derive(Debug)]` impl 디컴 = 구조체명·필드명·오프셋 일괄 확보(.rdata 필드명 blob→lea 함수) — **필드명 슬라이스 역매핑 실증 = unit_ai(Debug 0xd23cf0·이름표 0x31B1C30)** (버전무관, 07-31·08-03 실증) | 오프셋 막히면 Debug부터 | `MEM\tfm2-item-build-recommender.md` §3d + 08-03 judge_noise RE
TAG: - 노드 <player_info|wide_data.player_info>.<lane>.{blue,red}_player.slotN.bg.icon·레인표 0x318b000·빌더 0xa8ed10·탐색 0x19f170·"bg.icon" 0x318afe0 (0.5.3) | ★레인당 1개(5+5) | MIGRATION §7.3 §11.5c
TAG: - ★툴팁 show 0x1ab52f0(item_tooltip.rs·인자11 win64/void: rcx=0x960df0 arg5·rdx=arg6·r8=vt상수 0x318b4c0·r9=노드·rsp+0x20 item·+0x30 f32 xy·+0x50 clamp) (0.5.3) | ✅통짜호출 검증완·인자출처 필수 | MIGRATION §7.3 §11.5e
TAG: - 라벨 i18n 해석 0x1d28d0·말줄임 0x1d2500·스탯줄 빌더 0x1a3f900(i32)/0x1a3f5e0(i64)·아이콘 세팅 0x1a43720·클램프 0x3189d90 (0.5.3) | 매프레임 재계산·실패시 원문 | MIGRATION §7.3 §11.6
TAG: - SPAWN 0x1d9e0e0→**0xebfe50**(콜러 0xeb6480 @0xeb6511) (0.5.3) | ⚠ORIG_LEN 15→12·게이트OFF | MIGRATION §7.3 §11.1a
TAG: - serpen 0.5.3 LAUNCHER 0xeb8810(item_tactics와 동일)·LAUNCHER_RET_C 0x1555215→0x229ad94 | 실측 확정 | MIGRATION §7.3 §11.6 + §13.1
TAG: - level_cap RVA_LEN_LOAD 0x22d3fea→**0x12c5b44** · RVA_UI_CMP 0x80ae73→**0x95a359** (0.5.3, 07-31) | ★실측확정·컨테이너 0x12c56d0/0x952170 | MIGRATION §7.3 §16
TAG: - ★tfm2_banpick_order **0.5.3 훅8**(위 줄=0.5.2 이력) A′ 0x1bf3dd0 scene_step/B 0x167c0e0/C 0x1bd8c20/D′ 0x1680500/E 0x1bc52b0/F 0x167fdd0/AI패치 0x10a04e2·0x10a3cf8/**G 0x1828213**(AI턴 인라인) | ~~v1.2.0~~→**v1.2.1** 릴리스·검증완 | MIGRATION §7.3 §14 + tfm2-banpick-order-mod §12·§13·§14
TAG: - ★0x1bce8e0(원시 phase 0..3/0xff·콜러 0x2262ca0@0x2262f25) ↔ 0x1bf3dd0(단계enum) | 0.5.3 phase leaf 2종 분리 | MIGRATION §7.3 §14.5
TAG: - ⛔무효 phase훅6(0.5.3): I 0x193b434/J 0x1c6605d·0x1c66374/K 0x1c5a0b2·0x1c5a5b9·0x1c5a9b1/M 0x1c5aa99/N 0x1bce8e0/O 0x1c252c0 ⚠단 K 0x1c5a0b2(drainA)=펌프 게이트라 상시설치(§14.6) | 하이라이트 무효·재시도금지 | MIGRATION §7.3 §14.5
TAG: - 흰칸개수 0x1c55300 루프(0x1c5aa95→0x1c5ab31→[rbp+0x520]) / UI빌더 0x1bd94f0=phase_from call(훅B커버) / pick_slot 노드 wait=카드내용·in_turn>turn_outline | 밴픽 슬롯 하이라이트 구조 | MIGRATION §7.3 §14.5
TAG: - ★트리거 0x1bf77d0 dedup 3층(0.5.3): L0 scene+0xe0(-1=무효)/+0xe8/+0x120/+0x128/+0x130/+0xf8·0x100 · L1 지문String +0x288/0x290/0x298(발사시 갱신·format! 0x1400339e0·비교 0x1bf7cee) · L2 +0x200/0x208 stride 0x740 · 프롤로그 ORIG_LEN=15 | 위임 랜덤정지=L1 교착·워치독 | MIGRATION §7.3 §14.6 후속