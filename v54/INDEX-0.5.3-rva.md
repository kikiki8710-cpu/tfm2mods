# INDEX §2 — 0.5.3 시절 원본 줄 (2026-08-07 아카이브)
#
# 0.5.4 재핀으로 `MEM\INDEX.md` 의 주소를 갱신하면서, 원본 줄을 여기에 통째로 보존한다.
# 대응표 정본 = `C:\tfm2mods\v54\map_053_054.tsv` (546쌍).

- (구 INDEX:290) ★itemnet beam **0x10591f0**·매처 0x2155a90·경기형성 0x182d3c0·후보수집 0xe4d570/폴백 0xe51a50·술어 0x1095650 (0.5.3, 07-31) | ⛔채점 0x1bc8cd0=인라인 소멸 | `MEM\tfm2-item-build-recommender.md` §2
- (구 INDEX:300) LineGankerPlan line+0x28·setup_limit+0x18·wait_limit+0x20·phase+0x29·Debug impl 0xd8e800 (0.5.3, 08-03) | 필드 오프셋(Debug 디컴) | `REPORT\tfm2_ai_adjust\RE\2026-08-03_Strategy-소비처-전수맵-라인개입분기-0.5.3.md`
- (구 INDEX:301) runner.rs 0xebb9f0 = 경기시작 Strategy 24B 복사(+0x1568/+0x1be8) (0.5.3, 08-03) | 경기시작 주입 사이트 후보 | `REPORT\tfm2_ai_adjust\RE\2026-08-03_Strategy-소비처-전수맵-라인개입분기-0.5.3.md`
- (구 INDEX:302) gk_* 14사이트 = gk_wait 0xe0237d/0xe02cba/0xd45967/0xd557ae/0xd55cda(각+4B짝)·gk_hp 0xe01e53·gk_margin 0xe01ef7/0xe01f9e/0xe020d4 (0.5.3, 08-03) | 갱셋업 레버·⬜인게임 미검증 | MIGRATION §7.3 §12.17 + `REPORT\tfm2_ai_adjust\RE\2026-08-03_갱셋업-타이밍상수-사이트표-0.5.3.md`
- (구 INDEX:303) team_plan **0xcf8b90**(wav!=0→+0x3f5=0xFF·LAB_cf9688) · 선택기 **0xe29980**(wav==0만) (0.5.3, 08-03) | ⛔~~마스터게이트~~=실측 반박·⬜write경로 미지 | MIGRATION §7.3 §12.19(2)(정정) + §12.18(1)
- (구 INDEX:311) ~~★SubPlan13 생성 0xc6e4aa·SubPlan15 0xcb2570~~ → **sub_plan 값이라 disc13/15와 무관**·plan13/15 생산자 0건 (0.5.3, 08-03 밤 정정) | ⛔사장 변이·재시도금지 | MIGRATION §7.3 §12.20(2)
- (구 INDEX:314) ec_* 5키 게이트 0xc6e451/0xcb2735(jne)·판정헬퍼 0xe2a540 (0.5.3, 08-03) | bld 스플릿+lane 전용·모이기/유연 死 | MIGRATION §7.3 §12.18(4)
- (구 INDEX:315) 전술별 카피상수: engage 0x46 bat≠0 **0xd5c1b0**/bat==0 **0xd5d58a** · epic bld 3-way **0xcff440** · def cmove **0xdf80fd** · gb reach **0xe08858**(fin==0) (0.5.3, 08-03) | 신규 개별 튜닝 레버 후보 | MIGRATION §7.3 §12.18(5)
- (구 INDEX:325) disc2·disc8 movepri JT = 0xc55a34 상수 SubPlan7 · disc4 생성 0xd8065a(전술 read 0) (0.5.3, 08-03) | d8_slot_thr 도달불가 확증 | MIGRATION §7.3 §12.18(4)
- (구 INDEX:337) recall.rs 0xcb1a80 우물좌표 즉치 32000/928000·jungle camp4/5 우회 latch=SubPlan +0x11(리셋 dfff1b) (0.5.3, 08-03) | 우물행·한바퀴 원인 | MIGRATION §7.3 §12.23(2)
- (구 INDEX:339) 핸들러 공통 헬퍼 0xc6efd0(grow)·0xc9c770(extend)·**0xc365a0**(최종 후처리) (0.5.3, 08-03) | 0xc365a0 ⬜미조사 | MIGRATION §7.3 §12.23(3)
- (구 INDEX:341) plan7 HP 게이트 imm 0xdffebf(21)·0xdfff03(41) / line imm ca2b76·ca2e0f·ca3766·c57ed3계열·c57dcf (0.5.3, 08-03) | ⚠d7_repl 대체 여부 확인 | MIGRATION §7.3 §12.23(5)
- (구 INDEX:352) ★AI 자체 갱 개시 = **0xd4c873**(mode==None일 때만·+0x420 gank_start_tick) (0.5.3, 08-03) | 갱 시작 게이트 | ↑같은 RE
- (구 INDEX:367) ★경매 오버라이드 = **pending_global_ult_target** unit_ai+0x4F8/+0x500/+0x508·기록 chat.rs 0xd562c8·전제 8개·커밋 99999+태그 0x12(Ult) (0.5.3, 08-05) | "플레이어 지시" 가설 기각 | ↑같은 RE(경매 3단)
- (구 INDEX:380) ★record 0x1093b50 2번째 인자=**&World**(0xEEC8)·MVP=팀내 rating argmax @0x19740f0·MatchResult(0xce0)=[TeamMatchInfo(0x670);2]·won=+0x668+team*0x670 (0.5.3, 07-31) | ⚠MatchResult엔 KDA·스코어 없음 | `MEM\tfm2-item-build-recommender.md` §3
- (구 INDEX:386) item_tactics mid-func 0.5.3 = owned_cap 0xf24a39(imm 0xf24a40)·gate3 0xd0c9be(jbe 0xd0c9c4)·retaddr 0x9a3287/0x9a7b03/0x1925f12 | 패턴 재탐색·오프셋 이식 아님 | MIGRATION §7.3 §11.2
- (구 INDEX:403) athlete 0.5.3 불변 확정: id **+0x810**·side +0x820·items +0x448/50/58·build +0x490/98/4a0·gold +0x888·pos +0x8b0·stride 0x8d0 | ctor 0xed32b0 3연속스토어·fix B 성립 | MIGRATION §7.3 §11.3
- (구 INDEX:404) Game +0x1dc0 provider ptr/+0x1dc8 vtable (0.5.3 유지)·vt슬롯+0x20=[rcx+0xeaf8] | launcher 0xeb9646·독립증명 | MIGRATION §7.3 §11.3
- (구 INDEX:407) serpen 0.5.3 훅 = SERPEN 0x1535810·MOBATICK 0xeeeac0·SPAWN 0xabdf60/0xabd340·RENDER_STEP 0x960df0·DMGA 0xfdbbb0·DMGB 0x12c3bb0·KEYRES 0x1b0aba0·ARG_STR 0x1228a90 | ★인게임 12/12 검증완 | MIGRATION §7.3 §13.1·§13.6
- (구 INDEX:409) serpen 0.5.3 런처 게이트 = LAUNCHER_RET_B 0x9a7b03(화면경기 1회) vs 배경리그 0x220acb/0x20dac9c/0x195c5be | 인게임 분류 적중 | MIGRATION §7.3 §13.6
- (구 INDEX:611) ★0x1bf3dd0(0.5.3 신설·콜러23) / ⛔0x1cd9380=0.5.3 소멸 | scene_step 단계enum 0밴1픽2완료0xff | MIGRATION §7.3 §14
- (구 INDEX:612) 0.5.3 phase 인라인 복제본 30개(0.5.2=11)·미보정 ~20 = 0x1c55300×7·0x188dd30×2·0x1890450×2·0x1bd3960×2 등 | 전수=_bo_sites_053.json | MIGRATION §7.3 §14.4
- (구 INDEX:615) ⛔칸채움색 아닌후보5(0.5.3): 0x1c252c0(호출0회)·0x1bf9560(순수렌더러)·0x1c1f270(라벨세터)·Runner+0x13c·scene+0x3d0(팀1ID고정) | 재시도금지 | MIGRATION §7.3 §14.5
- (구 INDEX:617) 0x20733a3(0.5.3) | Runner+0x13c 초기화 콜사이트 | MIGRATION §7.3 §14.5(3)-4
- (구 INDEX:620) ★밴픽 자동행동 펌프체인(0.5.3): 드레인 0x1c55300→scene_step 인라인 0x1c5a02a~(디스패치 0x1c5a0b2=drainA·합류 0x1c5a288)→트리거 0x1bf77d0(disc 0x93 큐잉)→서버 CP디스패처 0x17e0240→0x17e659f call 0x1827e00(AI턴 유일 콜사이트) | 코치위임 펌프체인 | MIGRATION §7.3 §14.6
- (구 INDEX:622) 0x182813f(0.5.3·서버 AI턴 0x1827e00 조기리턴: 0x1828309 cmp rdx,rbx 팀 불일치→[rsi]=-1→0x17e65a5 cmp -1) | 요청무산=응답없이 소비 | MIGRATION §7.3 §14.6 후속
- (구 INDEX:692) server_pregate 0x17ef5f6(04→ff)·inc_gate 0x17f239c(→ff)·dr_inline_b 0x18e3fd6 (0.5.3) | ★일일제한 실효 3점=진범 서버게이트 | MIGRATION §7.3 §15
- (구 INDEX:693) ~~dr_inline_a/c/d~~ a 0x18d9436·c 0x18f18c7(폐기)·d 0x1987a3d (0.5.3) | 게이트 아님(a/d표시·c시드) | MIGRATION §7.3 §15
- (구 INDEX:694) daily 레코드 count +0xdc10·rec_id +0xdc1c·outer_id +0xe434 / 서버 gate 0x17f239c(+0x1d0/+0x1dc) (0.5.3) | 일일횟수 필드·서버게이트 | MIGRATION §7.3 §15