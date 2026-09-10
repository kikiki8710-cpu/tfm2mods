# `plan_allowed` 전수 진리표 (오라클 실측)

> = goal_allowed(ctx, BigPlan::goal(plan))  (IR m13.ll:54066 확증)

> 출처: SDK rlib 직접 링크 실행(`_oracle/o_main.rs`,`o_plan.rs`) / SDK `sdk_058` / 게임 0.5.8

| 입력 | 0 None | 1 First | 2 TopSolo | 3 Bottom | 4 MidSolo | 5 MidBottom | 6 JungleOnly | 7 Line | 8 Total | 허용 tut |
|---|---|---|---|---|---|---|---|---|---|---|
| `plan=ForcePassive` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `plan=ActiveRecall` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `plan=EpicHuntAndPoke` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `plan=EpicHuntAndBattle` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `plan=SerpenHuntAndPoke` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `plan=SerpenHuntAndBattle` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `plan=AttackNexus(default)` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `plan=DefenseNexus(default)` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `plan=PassiveLine(Mid)` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `plan=SinglePlanLine(Mid)` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `plan=LineGanker(Mid)` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `plan=LineGankCover(Mid)` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `plan=PassiveLine(Bottom)` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `plan=SinglePlanLine(Bottom)` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `plan=LineGanker(Bottom)` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `plan=LineGankCover(Bottom)` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `plan=PassiveLine(Top)` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `plan=SinglePlanLine(Top)` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `plan=LineGanker(Top)` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `plan=LineGankCover(Top)` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `plan=PassiveJungle(Rhino,0)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `plan=PassiveJungle(Rhino,1)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `plan=PassiveJungle(Mushroom,0)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `plan=PassiveJungle(Mushroom,1)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `plan=PassiveJungle(Stump,0)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `plan=PassiveJungle(Stump,1)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `plan=PassiveJungle(Bee,0)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `plan=PassiveJungle(Bee,1)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `plan=PassiveJungle(Morgard,0)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `plan=PassiveJungle(Morgard,1)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `plan=PassiveJungle(Serpen,0)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `plan=PassiveJungle(Serpen,1)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
