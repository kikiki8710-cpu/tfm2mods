# r21 잔여 — 미독 구역(r22 후보) · 게임 필요 보류(2026-09-17 · 0.6.0 · w11 까지 반영)

## A. 정적으로 더 팔 수 있는 미독 구역(r22 후보) — 주소는 0.6.0 RVA

| # | 함수 | 구역 | 왜 미독 | 출처 |
|---|---|---|---|---|
| 1 | LegacyPlanHandler::update `d3d210` | 강제 플랜 배정 `d42145..d43c65` | 17.5KB 신규 오케스트레이션 중 일부만 독해 | w4 |
| 2 | 〃 | 강제 LineGanker `d43c70..d44e64` | 〃 | w4 |
| 3 | 〃 | 서포터 사전플랜 · v3 정글 귀환 꼬리 · 로밍 트리거 | 〃 | w4 |
| 4 | 〃 래퍼 `d5a940` | 호출자 래퍼 본체 | 골격만 | w4 |
| 5 | TeamPlan::handle_none_or_gank_objective `f024d0` | `f02ef6..f04f9f` | V6 commit 뒤 구간 | w7 |
| 6 | passive_plan 후처리 `fb8da0` | ambient(코드 29) 본체 | 호출부만 | w6 |
| 7 | cc2 플래너 `d56ac0` | 후반(코드 32/33/34 뒤) | 전반만 · **cc2 는 v≥99 에서만 1 = 사실상 사장(w9)** → 우선순위 최하 | w6/w9 |
| 8 | handle_chat `d5d700` | Battle/Dive/Help 채팅 아웃라인 5.2KB | 미착수 | w7 |
| 9 | ~~make_gank_battle `fa9d60`~~ | ✅ w11 에서 닫힘(소변경) | | w11 |
| 10 | ~~ObjContest 생성처 `dcbe00`/`fd7f70`~~ | ✅ w9 에서 닫힘 | | w9 |
| 11 | phase 핸들러 `efeeb0`/`f00450`/`f00e80`/`f01380`/`f082d0` | 12 핸들러 중 5 | 표만 | w7 |
| 12 | `ef7680` contest_member_tick | allow 플래그 X(rbp+0x228) 정체 | 마스크 관리 본체는 w9 에서 독해 | w9 |
| 13 | 격자 경로탐색 `ee0fe0`/`ee2310`/`ee0dd0` | Hide stealth 이동·카정 move | 호출 계약만 | w1/w8 |
| 14 | best_jungle_goal `fafe50` v3 블록 B | `faf2a0`(faf2a0~fafb13) · `dbd370` · `dbe860` · game vt+0x108 구조체 정체 · fb0a3f 이후 꼬리 | 역정글 후보 세부 | w11 |
| 15 | position_eval_at_uncached `ff62e0` | 종반 신규 항(중심점 이격) 상수 | 부분 | r20 C |
| ~~16~~ | ✅ r22 a 에서 닫힘(TurnBack veto · e74a50 = nexus_final_stand 메모 · pos_ring LPH+0x458) · 잔여 = can_* 세부(IR) · +0x46c 추정 | | | r22 a |
| ~~17~~ | ✅ r22 c 에서 닫힘(d75840 은 v3 dead · d750f0/d75780 신규 · +0x24d6 세팅처 LPH::update) | | | r22 c |
| ~~18~~ | ✅ r22 b 에서 닫힘(hunt_call_pending 소비 · V6 sync-IN/OUT 0x158B · sanitize f105d0) · 잔여 = bb+0x78 리셋 시점·17d40b0 순서(런타임) | | | r22 b |
| 19 | TeamPlan::update `f18f90` 감사 프롤로그 | `f20f60`/`f21490` 카운터(+0xcc8~+0xccf) 가 결정 경로에서 읽히는지 | 「영향 없음」 은 추정 | w11 |
| 20 | `100bb90` expected_die_tick_at | cache+0xf0 리스트 · e14420 3능력 arrival 산식 cc 항 | 구조만 | w9 |

## B. 게임 실행이 필요해 보류(정적으로 확정 불가) — ★09-17 09:0x `tfm2_ver_probe` 실측으로 1·2·5 해소(`RE/2026-09-17_0.6.0_version값_실측_tfm2_ver_probe_원문.md`)

| # | 항목 | 왜 필요 | 확인 방법(게임 가능 시) |
|---|---|---|---|
| ~~1~~ | ✅ **version = 3**(3.7M+29.7M 히트 전부) ⟹ v2 분기 사장 | 초기화 상수가 정적 경로에서 안 잡힘(세이브/설정 유래 가능). **0.5.8 은 update_on_dead 콜러가 상수 0x5e(94) 를 넘겼다(w11)** — 0.6.0 도 상수 경로가 있는지 재확인 겸 | 훅 1회로 +0x3608 읽기 · 또는 `version>=3` 분기 하나에 카운터 |
| ~~2~~ | ✅ 29.7M 호출 전부 None · 콜러 43 사이트 확정 ⟹ Some 경로 사장(배경 sim 기준 · 실경기 미확인) | 정적 콜러 모두 None 리터럴 · 나머지 간접 | 인자 로그 훅 |
| 3 | Hide region id 2·7 의미(부시 vs 카정 셀) | 맵 region 테이블이 데이터 파일 | region 좌표 덤프 |
| 4 | sweep(실측 game==mine 비트동일) 전 함수 | 재명세는 정적 · 최종 심판은 런타임 | 기존 sweep 하네스 0.6.0 재빌드 후(동치 192 부터) |
| ~~5~~ | ✅ 실발생: serpen 213k 틱(phase 2/3) · morgard 191k 틱 · +0xcc7=1 상시 | +0xcc7==(v≥2) 는 정적 확정이나 씬 진입 조건은 런타임 값 의존 | TeamPlan +0x3c8/+0x3e8 로그 |
| 6 | TeamPlan v6 감사 카운터 소비 여부(A-19) · BattlePlan +0x40 gank_open_snap 소비처 | 정적 xref 로 소비처 미발견 | 쓰기 감시 훅 |
