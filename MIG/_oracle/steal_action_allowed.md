# `steal_action_allowed` 전수 진리표 (오라클 실측)

> None→true / Lurk(t),Commit(t)→steal_target_allowed(t)

> 출처: SDK rlib 직접 링크 실행(`_oracle/o_main.rs`,`o_plan.rs`) / SDK `sdk_058` / 게임 0.5.8

| 입력 | 0 None | 1 First | 2 TopSolo | 3 Bottom | 4 MidSolo | 5 MidBottom | 6 JungleOnly | 7 Line | 8 Total | 허용 tut |
|---|---|---|---|---|---|---|---|---|---|---|
| `act=None` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `act=Lurk(Epic)` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `act=Lurk(Serpen)` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `act=Commit(Epic)` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `act=Commit(Serpen)` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
