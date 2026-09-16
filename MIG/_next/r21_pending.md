# r21 잔여 — 미독 구역(r22 후보) · 게임 필요 보류(2026-09-17 · 0.6.0)

## A. 정적으로 더 팔 수 있는 미독 구역(r22 후보) — 주소는 0.6.0 RVA

| # | 함수 | 구역 | 왜 미독 | 출처 |
|---|---|---|---|---|
| 1 | LegacyPlanHandler::update `d3d210` | 강제 플랜 배정 `d42145..d43c65` | 17.5KB 신규 오케스트레이션 중 일부만 독해 | w4 |
| 2 | 〃 | 강제 LineGanker `d43c70..d44e64` | 〃 | w4 |
| 3 | 〃 | 서포터 사전플랜 · v3 정글 귀환 꼬리 · 로밍 트리거 | 〃 | w4 |
| 4 | 〃 래퍼 `d5a940` | 호출자 래퍼 본체 | 골격만 | w4 |
| 5 | TeamPlan::handle_none_or_gank_objective `f024d0` | `f02ef6..f04f9f` | V6 commit 뒤 구간 | w7 |
| 6 | passive_plan 후처리 `fb8da0` | ambient(코드 29) 본체 | 호출부만 | w6 |
| 7 | cc2 플래너 `d56ac0` | 후반(코드 32/33/34 뒤) | 전반만 | w6 |
| 8 | handle_chat `d5d700` | Battle/Dive/Help 채팅 아웃라인 5.2KB | 미착수 | w7 |
| 9 | make_gank_battle `fa9d60` | 본체(site 인자 1~7 · escape_possible) | w11 에서 닫는 중 | w8/w11 |
| 10 | ObjContest 생성처 `dcbe00`/`fd7f70` | Serpen/EpicHuntAndPoke sub_plan 본체 | w9 에서 닫는 중 | r20/w9 |
| 11 | phase 핸들러 `efeeb0`/`f00450`/`f00e80`/`f01380`/`f082d0` | 12 핸들러 중 5 | 표만 | w7 |
| 12 | `ef7680` | TeamPlan 씬 보조 | 미착수 | w7 |
| 13 | 격자 경로탐색 `ee0fe0`/`ee2310`/`ee0dd0` | Hide stealth 이동·카정 move | 호출 계약만 | w1/w8 |
| 14 | best_jungle_goal `fafe50` | ABI 변경 본체 | 골격만 | w3/w6/w11 |
| 15 | position_eval_at_uncached `ff62e0` | 종반 신규 항(중심점 이격) 상수 | 부분 | r20 C |

## B. 게임 실행이 필요해 보류(정적으로 확정 불가)

| # | 항목 | 왜 필요 | 확인 방법(게임 가능 시) |
|---|---|---|---|
| 1 | ★`version` 값(AgentVerHamster+0x3608 · 2 인지 3 인지) | 초기화 상수가 정적 경로에서 안 잡힘(세이브/설정 유래 가능) | 훅 1회로 +0x3608 읽기 · 또는 `version>=3` 분기 하나에 카운터 |
| 2 | check_kill_die_tick(eda920) 마지막 인자 `&Option<(x,y,t)>` 가 Some 인 콜러 | 정적 콜러 6곳 모두 None 리터럴 · 나머지 간접 | 인자 로그 훅 |
| 3 | Hide region id 2·7 의미(부시 vs 카정 셀) | 맵 region 테이블이 데이터 파일 | region 좌표 덤프 |
| 4 | sweep(실측 game==mine 비트동일) 전 함수 | 재명세는 정적 · 최종 심판은 런타임 | 기존 sweep 하네스 0.6.0 재빌드 후 |
| 5 | ObjContest 씬 실제 발생 빈도(v2 에서도 on 인지) | +0xcc7 세팅은 정적 확정이나 씬 진입 조건은 런타임 값 의존 | TeamPlan +0x3c8/+0x3e8 로그 |
