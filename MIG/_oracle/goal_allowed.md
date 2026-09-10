# `goal_allowed` 전수 진리표 (오라클 실측)

> BigGoal 튜토리얼 게이트

> 출처: SDK rlib 직접 링크 실행(`_oracle/o_main.rs`,`o_plan.rs`) / SDK `sdk_058` / 게임 0.5.8

| 입력 | 0 None | 1 First | 2 TopSolo | 3 Bottom | 4 MidSolo | 5 MidBottom | 6 JungleOnly | 7 Line | 8 Total | 허용 tut |
|---|---|---|---|---|---|---|---|---|---|---|
| `goal=Line(Mid)` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `goal=Line(Bottom)` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `goal=Line(Top)` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `goal=Jungle(Rhino,0)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `goal=Jungle(Rhino,1)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `goal=Jungle(Mushroom,0)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `goal=Jungle(Mushroom,1)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `goal=Jungle(Stump,0)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `goal=Jungle(Stump,1)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `goal=Jungle(Bee,0)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `goal=Jungle(Bee,1)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `goal=Jungle(Morgard,0)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `goal=Jungle(Morgard,1)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `goal=Jungle(Serpen,0)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `goal=Jungle(Serpen,1)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `goal=Epic` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `goal=Serpen` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `goal=Nexus(0)` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `goal=Nexus(1)` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `goal=Battle(None)` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `goal=Battle(Some(0))` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `goal=Recall` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
