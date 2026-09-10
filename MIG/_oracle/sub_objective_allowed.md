# `sub_objective_allowed` 전수 진리표 (오라클 실측)

> SubObjective 튜토리얼 게이트

> 출처: SDK rlib 직접 링크 실행(`_oracle/o_main.rs`,`o_plan.rs`) / SDK `sdk_058` / 게임 0.5.8

| 입력 | 0 None | 1 First | 2 TopSolo | 3 Bottom | 4 MidSolo | 5 MidBottom | 6 JungleOnly | 7 Line | 8 Total | 허용 tut |
|---|---|---|---|---|---|---|---|---|---|---|
| `so=LineBattle(Mid,dive=false)` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `so=LineBattle(Mid,dive=true)` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `so=LineBattle(Bottom,dive=false)` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `so=LineBattle(Bottom,dive=true)` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `so=LineBattle(Top,dive=false)` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `so=LineBattle(Top,dive=true)` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `so=JungleBattle(Rhino,0)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `so=JungleBattle(Rhino,1)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `so=JungleBattle(Mushroom,0)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `so=JungleBattle(Mushroom,1)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `so=JungleBattle(Stump,0)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `so=JungleBattle(Stump,1)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `so=JungleBattle(Bee,0)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `so=JungleBattle(Bee,1)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `so=JungleBattle(Morgard,0)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `so=JungleBattle(Morgard,1)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `so=JungleBattle(Serpen,0)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `so=JungleBattle(Serpen,1)` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
