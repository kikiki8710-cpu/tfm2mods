# `fallback_line` 전수 진리표 (오라클 실측)

> line_exists면 그대로, 아니면 대체 라인

> 출처: SDK rlib 직접 링크 실행(`_oracle/o_main.rs`,`o_plan.rs`) / SDK `sdk_058` / 게임 0.5.8

| 입력 | 0 None | 1 First | 2 TopSolo | 3 Bottom | 4 MidSolo | 5 MidBottom | 6 JungleOnly | 7 Line | 8 Total | 허용 tut |
|---|---|---|---|---|---|---|---|---|---|---|
| `line=Mid` | Mid | Bottom | Top | Bottom | Mid | Mid | Bottom | Mid | Mid |  |
| `line=Bottom` | Bottom | Bottom | Top | Bottom | Mid | Bottom | Bottom | Bottom | Bottom |  |
| `line=Top` | Top | Bottom | Top | Bottom | Mid | Bottom | Bottom | Top | Top |  |
