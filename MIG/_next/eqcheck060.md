# eqcheck060 — 「동치 192」 신뢰도 종합(0.5.8→0.6.0 · 2026-09-17 09:5x)

질문: 「변경 없음(동치)으로 해둔 192 는 어느 정도 믿을 만한가?」 → 두 검사로 답했다.
① `dispcheck060.py`: 「오프셋만」 판정의 변위 짝이 **같은 필드의 이동**인지 §A 표로 역참조 · ② `calleeprop060.py`: 본체 동치라도 **콜리가 변경**이면 동작은 다르다 — 그 목록.

## 1. 등급표(최종)

| 등급 | 수 | 조건 | 뜻 |
|---|---|---|---|
| **A** 본체 동치 + 콜리 전부 동치 + 변위 전부 설명 | **119** | dispcheck 미설명 0 · calleeprop 「콜리 전부 동치」 | 명세 logic 그대로 · 오프셋/태그/슬롯 갱신만 하면 game==mine 기대 |
| **B** 본체 동치 · 콜리 변경 있음 | **55** | 콜리 = estimate_damage_to 41 · check_kill_die_tick 7 · calculate_action_score 6 · evaluate_action 3 · item_v26 2 · 기타 | 명세 logic 그대로 · **런타임 값은 콜리를 v3 로 갈아야** 같음(`specs20_v060.json` `v060.callee_changed`) |
| **C** 본체 동치 · 콜리 미대조/짝 불일치 | **18** | 콜리 = BigPlan 디스패처 · TLS LocalKey 래퍼 · PathFinder get_input 계열 ICF 짝 불일치 · d52030/d60920 | 콜리를 한 번 더 봐야 함(대부분 vtable/ICF 짝 문제 · 본체 영향 낮음) |
| 합계 | 192 | | |

변위 검사: 「오프셋만」 147 함수 · 변위 짝 658 → **규칙 설명 658 / 미설명 0**. 의심 11 사이트 디컴 결과 「다른 필드」 **0** — 7 은 §A 에 없던 규칙(TeamPlan 선두 +0x20 · EffectType vt 0x30→0x38 · ChampionCache +0x30 · TLS · next_respawn_tick · chats Vec), 4 는 **정렬 도구의 짝 오배열**(Entity 슬롯 switch 블록이 SmallActionPlay 태그 재번호로 재배열 · 접근 집합 동일). 원문 = REPORT RE `2026-09-17_0.6.0_dispcheck060_…_원문.md`.

## 2. 출처별 신뢰(정정판)

| 출처 | 수 | 근거 | 신뢰 |
|---|---|---|---|
| mig060_same 완전 동일/오프셋만/TLS/enum 재번호 | 131 | 명령열 100% 정렬 + **변위 짝 658 전부 §A 역참조 통과** | ★★★(변위 구멍 닫힘) |
| r19/r20/r21 디컴 대조 동치 | 61 | 에이전트 줄 단위 대조 · 후속 웨이브 정정 6 중 동치→변경 뒤집힘 0 | ★★☆ |

남는 한계 = **정적**이라는 것 하나. 최종 심판(runtime DIFF=0)은 0.6.0 에서 아직 0건 — 단 오늘 stable 껍데기 detour 가 되는 걸 봤으니(03 §56) sweep 이식이 다음.

## 3. 반영 때 규칙
- A 119: `spec_patch_060.md` §A 표만 적용(변위·태그·vt 슬롯) · logic 원문 유지.
- B 55: 위 + `callee_changed` 의 콜리를 v3 재현으로 교체(estimate_damage_to 는 신챔프 override 3 · ckdt 는 edb240 v3 타임라인).
- C 18: 콜리 짝을 `mig060_same_eq46.md`/`mig060_same.md` 「불일치」 행에서 확인 후 A/B 로 편입.

## 4. 산출물
`MIG\dispcheck060.py` → `_next\dispcheck060.{md,json}` · `MIG\calleeprop060.py` → `_next\calleeprop060.md` + `specs20_v060.json` `callee_changed` · `_next\mig060_same_eq46.{json,md}`(r19/r20/r21 동치 46 의 콜리 등급) · §A 추가 규칙 = `spec_patch_060.md` §A 「dispcheck 추가」 행.
