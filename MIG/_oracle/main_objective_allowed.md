# `main_objective_allowed` 전수 진리표 (오라클 실측)

> MainObjective 튜토리얼 게이트

> 출처: SDK rlib 직접 링크 실행(`_oracle/o_main.rs`,`o_plan.rs`) / SDK `sdk_058` / 게임 0.5.8

| 입력 | 0 None | 1 First | 2 TopSolo | 3 Bottom | 4 MidSolo | 5 MidBottom | 6 JungleOnly | 7 Line | 8 Total | 허용 tut |
|---|---|---|---|---|---|---|---|---|---|---|
| `mo=Morgard(ph=Setup,wb=false)` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `mo=Morgard(ph=Setup,wb=true)` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `mo=Morgard(ph=Assemble,wb=false)` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `mo=Morgard(ph=Assemble,wb=true)` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `mo=Morgard(ph=Hunt,wb=false)` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `mo=Morgard(ph=Hunt,wb=true)` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `mo=Morgard(ph=None,wb=false)` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `mo=Morgard(ph=None,wb=true)` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `mo=Serpen(ph=Setup,wb=false)` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `mo=Serpen(ph=Setup,wb=true)` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `mo=Serpen(ph=Assemble,wb=false)` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `mo=Serpen(ph=Assemble,wb=true)` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `mo=Serpen(ph=Hunt,wb=false)` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `mo=Serpen(ph=Hunt,wb=true)` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `mo=Serpen(ph=None,wb=false)` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `mo=Serpen(ph=None,wb=true)` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `mo=Defense` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `mo=Repair` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `mo=DefenseLine(Mid)` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `mo=DefenseLine(Bottom)` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `mo=DefenseLine(Top)` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `mo=Nexus(Mid)` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `mo=Nexus(Bottom)` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `mo=Nexus(Top)` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `mo=PressEpic(Mid)` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `mo=PressEpic(Bottom)` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `mo=PressEpic(Top)` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `mo=SplitEpic(Mid)` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `mo=SplitEpic(Bottom)` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `mo=SplitEpic(Top)` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `mo=Gank(Mid)` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `mo=Gank(Bottom)` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `mo=Gank(Top)` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `mo=Dive(Mid)` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `mo=Dive(Bottom)` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `mo=Dive(Top)` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `mo=PressTower(Mid)` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `mo=PressTower(Bottom)` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `mo=PressTower(Top)` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `mo=ComebackPick(Mid,ready=false)` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `mo=ComebackPick(Mid,ready=true)` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `mo=ComebackPick(Bottom,ready=false)` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `mo=ComebackPick(Bottom,ready=true)` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `mo=ComebackPick(Top,ready=false)` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `mo=ComebackPick(Top,ready=true)` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
