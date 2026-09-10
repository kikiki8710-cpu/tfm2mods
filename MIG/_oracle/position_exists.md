# `position_exists` 전수 진리표 (오라클 실측)

> 포지션이 그 튜토리얼에 존재하는가

> 출처: SDK rlib 직접 링크 실행(`_oracle/o_main.rs`,`o_plan.rs`) / SDK `sdk_058` / 게임 0.5.8

| 입력 | 0 None | 1 First | 2 TopSolo | 3 Bottom | 4 MidSolo | 5 MidBottom | 6 JungleOnly | 7 Line | 8 Total | 허용 tut |
|---|---|---|---|---|---|---|---|---|---|---|
| `pos=Top` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `pos=Jungle` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `pos=Mid` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `pos=Bottom` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `pos=Support` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
