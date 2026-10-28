import json
J={
"banpick_view_plus":{
"0x16cca0":["0x159060","HIGH · RVA_ASSET_GET · 콜슬롯 계보투표 47:0(29클론 중 유일) · ILLUST_GET(ratio1.0) 콜슬롯1 이 0x159060 호출 · 콜러 60→63 · TypeId 전부 재생성(비교불가) · 본문 Δ+0x338 은 자기 store 내부(map 0x30→0x368)로 소스 무관"],
"0x1bd180":["0x1c8030","HIGH · RVA_SUBMIT · BYTES_UNIQUE · ratio1.000 · CARD_DRAW 콜슬롯 7곳 일치 · 계보 87:0"],
"0x1bd400":["0x1c82b0","HIGH · RVA_SUBMIT_TEXT · BYTES_UNIQUE · ratio1.000 · CARD_DRAW 콜슬롯27 · 계보 5:0"],
"0x1bde50":["0x1c8cc0","HIGH · RVA_TEXT_BUILD · ★엔진 0x13194e0(FN_WEAK, size4860·콜러1) 오답 · CARD_DRAW 콜슬롯26(+0xb1f) · 콜러 6=6·계보 6:0 · 0.6.0 TEXT_BUILD 0x1b5240 과 skel 완전일치(1297B, 0.6.1 축소 이전 형태로 회귀) · 13인자 콜사이트 셋업 동일(13번째만 조건부)"],
"0x1be430":["0x1c9630","HIGH · RVA_IMG_UV · BYTES_UNIQUE · ratio1.000 · 계보 69:0 · CARD_DRAW 콜슬롯 2곳"],
"0x1be5d0":["0x1c97d0","HIGH · RVA_IMG_BUILD · 계보 71:0 · CARD_DRAW 콜슬롯 2곳 · 프롤로그 push5→push6(호출만, 훅 아님) · 12인자+arg13(byte) ABI·출력 cmd 필드(0x00~0xc9) 동일"],
"0x1be8e0":["0x1c9af0","HIGH · RVA_IMG_FLAG · BYTES_UNIQUE · ratio1.000 · CARD_DRAW 콜슬롯 2곳"],
"0x1bff40":["0x1cb0f0","HIGH · RVA_IMG_SHADER · 계보 26:0 · CARD_DRAW 콜슬롯 2곳 · 본문 5148→4853(ratio0.49)이나 콜 42개 순서·출력 store 오프셋 집합 동일"],
"0x2245a0":["0x2ce4e00","HIGH · RVA_IMG_COLOR · 0.7.0 에 skel 클론 1개만 남음(0.6.5 의 2245a0/2024f60 두 계보 통합) · ratio1.000 · CARD_DRAW 콜슬롯 2곳 · 계보(2024f60) 28:0 · 콜러 10"],
"0x20f5f0":["0x261f50","HIGH · RVA_ANIM_GET · 콜슬롯 계보투표 54:0(29클론 중 유일) · CARD_DRAW 콜슬롯15(+0x5b1) · 콜러 65→67 · Δ+0x338 은 store 내부(소스 무관)"],
"0x22b56b0":["0x2ef8990","HIGH · RVA_FX_SET(훅A) · BYTES_UNIQUE · ratio0.96(차이=self 오프셋만) · 진입 12B 동일 · ★self+blue_team 0x3f8→0x410"],
"0x22cb030":["0x2f0e320","HIGH · RVA_CARD_DRAW(훅B) · 엔진 FAIL→확정 · ILLUST_GET 신 콜러 3개 중 2997B(구 2979B) · 27/27 콜슬롯이 같은 함수·같은 상대오프셋(+0xb0d 이후만 +0x12) · 유일 콜러=컨테이너A 0x2fcf6f0 · 진입 12B 동일"],
"0x22d5228":["0x2f18ad8","HIGH · RVA_D_CUT_LO · 컨테이너B 0x22d51a0→0x2f18a50(1062=1062B, ratio1.000, 유일콜러 A) · +0x88 불변 · disp→-70.0 확인"],
"0x22d5236":["0x2f18ae6","HIGH · RVA_D_CUT_HI · 컨테이너B +0x96 불변 · disp→+70.0 확인"],
"0x22f9770":["0x2f66e70","HIGH · RVA_NAME_GET · BYTES_UNIQUE · ratio1.000 · CARD_DRAW 콜슬롯1"],
"0x22feb50":["0x2f6c180","HIGH · RVA_SPRITE_CALC · BYTES_UNIQUE · ratio1.000 · CARD_DRAW 콜슬롯17"],
"0x235ee00":["0x2fcff5a","HIGH · RVA_I_SNAP_H · 엔진 FAIL·aligned 값 채택 · 컨테이너A 0x235e580→0x2fcf6f0(8149→8717, ILLUST_GET 콜러) · 'mov dword [rsp+0x20],480.0' imm4 · 480/360/-180 참조 12곳 1:1 · +0x880→+0x86a"],
"0x235ee16":["0x2fcff70","HIGH · RVA_D_SNAP_W · aligned 값 · movss xmm3,[rip] disp→360.0 · I_SNAP_H+0x16 관계 유지"],
"0x235fc62":["0x2fd1002","HIGH · RVA_D_ZIG_X1 · aligned 값 · -180.0(블록+0x80) 참조 2곳 중 첫째 · +0x16e2→+0x1912"],
"0x2360340":["0x2fd16e0","HIGH · RVA_D_ZIG_X2 · aligned 값 · 둘째 -180.0 참조 · +0x1dc0→+0x1ff0"],
"0x220dda0":["0x23cc2c0","HIGH · RVA_ILLUST_GET(훅C) · BYTES_UNIQUE · ratio1.000 · 진입 13B 동일 · CARD_DRAW 콜슬롯5"],
"0x2f2dc50":["0x356f190","HIGH · RVA_GAME_ALLOC · FN_UNIQUE · ratio1.000(60B) · 계보 15378:9"],
"0x3c8a990":["0x420ea60","HIGH · RVA_C_CARD_RECT · BYTES_UNIQUE · 값 (-180,-240,360,480) · 컨테이너A 블록 참조 상대분포 동일"],
"0x3c8a9b0":["0x420ea80","HIGH · RVA_C_SNAP_RECT · 블록+0x20 불변 · 값 (0,0,360,480)"],
"0x3c8a9c0":["0x420ea90","HIGH · RVA_C_NORMAL · 블록+0x30 불변 · 값 (0.6866,0.7270)"],
"0x3c8a9e0":["0x420eab0","HIGH · RVA_C_LINE_DIR · 블록+0x50 · 값 (360,340) · mulps 참조 3곳"],
"0x3c8a9f0":["0x420eac0","HIGH · RVA_C_LINE_START · 블록+0x60 · 값 (-180,170)"],
"0x3c8aa00":["0x420ead0","HIGH · RVA_C_LINE_ANCHOR · 블록+0x70 · 값 (0,170)"]
},
"ui_kit":{
"0x1e79d0":["0x216d50","HIGH · DD_SETOPT_RVA · BYTES_UNIQUE(32B) · 24B 프롤로그 동일 · ratio0.917(차이=runner 오프셋만) · 옵션 stride 0xf8·문자열 +0xe0 불변 · ★DD_SELECTED_OFF 0x1790→0x1810"],
"0x2f60f0":["0x351e10","HIGH · RVA_DROP_MAP · 계보 67:0(형제 0x194920→0x1ac100 212표로 분리) · ratio1.000 · 신 DROP_PLAN 0x364070 꼬리 jmp 0x351e10 · PROL 12B 동일"],
"0x3066f0":["0x364070","HIGH · RVA_DROP_PLAN · BYTES_UNIQUE · skel 유일 · ratio1.000 · 계보 15:0 · PROL 8B 동일"],
"0xa30c90":["0xa315c0","HIGH · RVA_INSERT · 계보 5:1(0xa30f60 1표) · ★SANITIZE(신 0x28c5a30) 콜슬롯6 = 0xa315c0 · 클론 0xa30f60(0x1d9e1f0 계보 8표)과 정규화 명령열 완전동일(기능 동등) · PROL 12B 동일"],
"0x29305b0":["0x28f2de0","HIGH · RVA_CLONE · 계보 9:0(형제 0x215ec60→0x2e62540 10표로 분리) · ratio1.000(2755B) · PUSH8 동일"],
"0x2859590":["0x27a29e0","HIGH · RVA_DROP(=training_plan RVA_DROP_RES) · 엔진 FAIL(pdata 없는 leaf)→바이트패턴 검색 2곳 중 대핸들러 0x284eed0 안 E8/E9 258회(구 246) · BUILD→SEND→DROP 시퀀스에서 SEND 직후 호출 확인 · ⚠니치 sub 0x56→0x59 · 첫 바이트 0x48(가드③ 통과)"],
"0x29c8000":["0x2802060","HIGH · RVA_SEND · BYTES_UNIQUE · 계보 31:0(형제 0x29c8170→0x28021d0 16표) · ratio1.000 · 대핸들러 내 258회(구 246) · PROL_SEND 12B 동일"],
"0x28f7c20":["0x28aac40","HIGH · RVA_BUILD · skel 유일·콜러 2=2(대핸들러+232B 헬퍼) · 대핸들러 유일 BUILD 콜 · 해시 인라인화로 464→448B·ratio0.64지만 5인자 ABI·프레임 동일 · 태그 0x10 기록 헬퍼(0x2803080) 불변"],
"0x2910e20":["0x28c5a30","HIGH · RVA_SANITIZE · BYTES_UNIQUE · ratio0.991(TLS 오프셋만) · 유일 콜러=신 대핸들러 · UpdateTrainingPlan 블록에서 직후 db+0x17938 조회"],
"0x2360f30":["0x2fd22e0","HIGH · RVA_UPDATE · 엔진 FAIL→확정 · head 일치 3개 중 유일 대형(140682B)·콜러0·vtable 절대포인터 1곳(구 1) · 니모닉 코사인 0.99995(2위 0.9972) · 콜리 148→158 · 씬 phase/T1/T2/sent 사용 패턴 1:1(+0x18) · PROL 12B 동일"],
"0x1db7810":["0x2803cb0","HIGH · RVA_RESPONSE · BYTES_UNIQUE · 형제 0x2803df0 은 tag 0x30 기록(구 형제 0x2e) · 0x2803cb0 은 tag 0x11 기록(TAG_RESPONSE_TRAINING_PLAN) · ratio1.000 · PROL 8B 동일"]
}}
json.dump(J,open(r'F:\claude\tfm2mods\MIG\_patchday\0.7.0\judge_viewplus_uikit.json','w',encoding='utf-8'),ensure_ascii=False,indent=1)
print(len(J['banpick_view_plus']),len(J['ui_kit']))
