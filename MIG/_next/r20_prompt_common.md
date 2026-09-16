# r20 공통 지시 (변경 99 중 티어 2/3 · 0.5.8→0.6.0 디컴 대조)

Ghidra MCP `ghidra`(8080) = 0.6.0 exe · `ghidra_beta`(8081) = 0.5.8 exe · image base 0x140000000(RVA = abs − base). **게임 실행 금지.** 브리핑 파일(반드시 먼저 읽을 것)에 짝마다 exe 정규화 정렬의 「잔여」 차이(정렬로 못 닫은 구조/분기/즉치/변위)와 0.5.8 명세 요지가 있다.

## 이미 확정된 0.6.0 사실(재조사 불요 · 판정에 그대로 적용)
- `SmallActionPlay` 17→16(`AroundHide` idx 3 제거). 태그 바이트 `+0xb1` = 3+idx(AroundPosition 만 0/1/2 · 구멍 10→9). 0.6.0 태그: RunAway 3 · Recall 4 · Around 5 · AroundRegion 6 · AroundRunAway 7 · Positioning 8 · AroundPositionBush 10 · AroundBush 11 · LaneMinionPosition 12 · Trace 13 · Attack 14 · Skill 15 · Skill2 16 · Ult 17 · Stop 18. switch 기본 idx `mov r,7→6` · 점프테이블 17→16.
- `BattleSubPlanGoal` 8→5: Protect·Assassin·AssassinReady 제거 → Trace 0 · Kiting 1 · KitingBack 2 · RunAway 3 · End 4 (0.5.8: Trace 0 Protect 1 Kiting 2 KitingBack 3 RunAway 4 Assassin 5 AssassinReady 6 End 7).
- `SubPlan` 18: idx 17 AttackNexusSubPlan → ObjContest{slot @+8 (0 Serpen/1 Epic), poke @+0x10} · 나머지 동일 · 태그 0x8000000000000000+idx · 니치 dataful 4(Jungle). `BigPlan` 16 변형 순서 불변 · LegacyPlanHandler 니치 기본 idx 4→7.
- Effect 트레이트 vtable: 신규 슬롯 4(0x30 damage_vs_target · 0x40 · 0x48 · 0x70) ⟹ 구 0x38~0x50 → +0x18 · 구 ≥0x58 → +0x20(0x60→0x80 · 0x68→0x88 · 0x80→0xa0 · 0x88→0xa8 · 0xa0→0xc0 · 0xe8→0x108 · 0xf0→0x110).
- 구조체: PlayerState +0xd0(0x928/0x930/0x9c0 → 0x9f8/0xa00/0xa90) · Blackboard stride 0x2e8→0x5c8 · 팀블록 0x1e0→0x3e8 · LegacyPlanHandler v3_armed 0x1808→0x24b5 · plan 태그 0x5e8→0xf58 · PassiveLine.line 0x706→0x107b · PassiveJungle.team 0x638→0xfc0 · .jungle 0x650→0xfe3 · LineGanker.line 0x618→0x100b · TeamPlan 신규 +0xcc7(ObjContest 게이트) · +0xcd5(phase) · +0x3e3/+0x3e4(Serpen flag/모드) · +0x403/+0x404(Epic).
- `estimate_damage_to` 0x12857f0→0x1643790(산식 동일 · 신챔프 override 3) · 챔피언 특성 `ChampTrait{AggressivePlay,DefensivePlay,FightJoiner,LaneIntervention}` · agent_ctx `+0x49c/+0x49d/+0x49e` = 특성 플래그(추정) · `0xd72e70` = 다이브/킬 창 평가(신규).
- Ghidra 디컴이 점프테이블 오염·memcpy noreturn 오인으로 자주 실패한다 → capstone 선형 디스어셈 + 점프테이블 직접 덤프 + 명령 다중집합 diff 가 실전 수단(스크립트 예 = `C:\tfm2mods\MIG\scr060\`).

## 요청(함수마다)
① 두 버전 디컴/디스어셈 → 분기·상수·읽는 필드·콜리를 단계별로 대조.
② **판정** = 「동치(재번호·오프셋·vt 슬롯·인라인 경계·컴파일러 재인코딩만)」 / 「로직 변경 — 무엇이(새 조건·계수·필드·분기·콜리)」 · 확정/추정 구분. 로직 변경이면 **0.5.8 명세의 어느 줄이 어떻게 바뀌는지**(재현 시 반영할 등가 Rust 한 줄) 까지.
③ 변경이면 0.6.0 디컴 변경 구간 20~40줄 인용 · 동치면 근거 3~5줄.
④ 새 enum/구조체/vt 사실이 나오면 별도 절로.
반환 = 결론 표(함수 · 판정 · 근거 · 갱신할 상수/오프셋/콜리 RVA) + 함수별 상세. 시간이 모자라면 작은 것부터 하고 못 한 것을 명시.
