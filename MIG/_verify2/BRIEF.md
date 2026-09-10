# 20함수 2차 재검증 — 공용 브리핑 (2026-09-11 / 게임 0.5.8)

## 0. 이건 재작성이 아니라 **반증**이다

기존 명세(`_spec\specs20.json`)를 **깨보는 것**이 임무다. "맞다"를 확인하러 가지 말고
**틀린 곳을 찾으러** 가라. 오류 0건이면 그렇게 보고하면 된다(그것도 결과다).

1차 반증검증(오늘 오전)에서 **정정 13건**이 나왔고 그중 2건은 **방법론 문서 자체의 오류**였다.
그 13건은 **이미 본 표에 반영 완료**다(`_spec\apply13.py` · `apply2.py`, `stalecheck.py` STALE 0).
**같은 것을 또 찾지 마라.** 1차 결과 전문 = `_verify\REPORT_{A,B,C,D}.md` — **착수 전에 자기 배치 것을 읽어라.**

## 1. ★1차에 없었던 새 재료 (여기서 새 결과가 나온다)

### ① SDK 실행 오라클 — **이제 거의 모든 pub 함수에 쓸 수 있다**
1차 때 배치 A·B·C 는 이걸 **못 썼다.** 배치 C 는 `&OperationData`·`&PlayerState` 를
「인자 구성 불가」로 기록하고 넘어갔는데, **배치 D 가 실행으로 반증했다.**

```
Game::new(seed, bool, &GameSetting, &MapSetting, &MapDef)                      pub
game.add_player(GamePlayer::new(id,"p",team,pos,AthleteStat::default(),
        "swordman", Arc::new(SwordmanChampionInfo::default()), vec![]))        pub
game.start_game(&mut StdRng, &ctx)                                             pub
AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx)                   pub
OperationData::new(&cache, &ctx, &[Blackboard::default(); 2])                  pub
game.get_player_by_position(team,pos) -> Option<&PlayerState>                  pub  (start_game 후 10/10 Some)
cache.player_champion[t][p] -> Option<&Entity>                                 pub
TeamPlan::default() / DebugFrameData::default()                                pub
```
작동 예제 = `_verify\D_oracle1..4.rs`. **이걸 복사해서 시작해라.** 레시피 = `IR_TOOLKIT.md §7`.
⚠**`GameSetting::default()` 는 `tick_per_second = 0` 이다** — `udiv by tps` 가 있는 함수는
**반드시 60 으로 세팅**하고 돌려라(1차의 미해결 숙제).
⚠한계: `pub(crate)` 모듈(`fight_check`/`position_eval`/`path_finder`/`score_parameter`/
`buff_value`/`small_action`)은 여전히 막힌다. 챔피언 실전 데이터 로딩은 미탐색.

### ② `_tcx\mirdump_game_core.txt` (403MB · 17,575 바디)
1차엔 **배치 A 만** 갖고 있었다. `xinl=1` 인 작은 game_core 헬퍼 술어 본문이 전부 여기 있다
(`is_*`, `Entity::radius`, `is_visible_from`, `Position::as_index`, `remain_epic_time` …).
추출기 = `_verify\A_mirget.py`. **"술어 본문을 못 본다"고 쓰기 전에 여기부터 쳐라.**

### ③ 패닉 Location **칼럼** — `_verify\B_plocfull.py`
`panicloc.py` 는 칼럼을 버린다. 보정: `bounds_check` 칼럼 = **인덱싱식 시작**,
`.unwrap()` 칼럼 = **메서드 이름 시작**. ⚠한계(실측): 패닉 가능 연산이 없는 줄엔 상수 자체가 없다.

### ④ 줄 길이 산술 — `IR_TOOLKIT.md §8`
`rmeta_srcmap` 줄 길이(**값 − 1** = LF 포함) + `_tcx` `sp`(줄:칸) + tcx 필드명 + IR 분기 유무.
게임 소스는 **2-스페이스 들여쓰기**. ⚠`A || B` 순서는 길이 불변이라 **원리적으로 못 가른다**.

## 2. 함정 (1차에서 실제로 밟은 것만)

1. ★**IR 의 `< N` 상수를 소스 임계로 읽지 마라.** `x==0||x==1` → `icmp ult x,2` 는 표준 접힘.
   **호출 술어의 tcx `sig` 를 먼저 봐라** — 인자를 받는 술어면 그 상수는 십중팔구 접힘이다.
   (1차에서 "정글 캠프 티어 컷" 노브가 이렇게 **유령으로 만들어졌다.**)
2. ★**internal fastcc 함수의 IR 인자 목록 ≠ 소스 인자 목록**(ArgumentPromotion).
   판별 = tcx `sig` + DWARF `!DILocalVariable(name:…, arg:N)`.
   ➕부수 소득: promotion 이 일어났다는 것 자체가 **"그 인자에서 그 필드만 읽는다"의 증명**이다.
3. **니치 밀림**(메모리 태그 ≠ 논리 인덱스). ⚠단 **「선언 순서 ≠ 태그」는 실증 사례 0건**이다
   — 1차에 그런 함정이 있다고 브리핑했는데 **거짓이었다**(LineType/ObjectPhase 둘 다 선언순서=태그).
   `tcxdict --enum` 의 `idx` 와 variant `def_span` 을 믿어라.
4. **`or`/`and` 피연산자 순서로 소스 순서를 추정하지 마라** — rank 정렬로 설명되며 정보량 0.
   단 **`select i1 A, i1 B, i1 false/true` 로 남았으면 단축평가 보존 = 순서 확정**.
5. **인라인 체인의 `DISubprogram(name:)` 을 봐라** — `first()` vs `get(0)` 은 파일:줄로 못 가른다.
6. **쌍둥이 파일**: `old\epic\hunt_and_battle.rs` 와 `old\serpen\hunt_and_battle.rs` 는 줄번호까지 같다.
7. **형제 함수를 보라.** `update` 만 읽으면 `sub_plan` 이 쓰는 **다른** 선택기를 놓친다
   (1차 배치 C 가 `target_bush_v41` 을 그렇게 발견했다). `grep "^define.*<PlanName>"` 한 번은 쳐라.

## 3. ★판정 어휘 — 범위를 반드시 써라

**"X 불가능"이라고 쓰지 마라. "A 방식으로는 X 불가능, 미탐색 = B" 라고 써라.**
- **표기 불가** = 동작은 확정, 소스 표기만 외연 동일해 못 가름 (예: 같은 길이의 두 표기)
- **재료 부재** = 어디를 뒤졌는지 **범위를 열거**해야 쓸 수 있다
- **미탐색** = 불가능이 아니라 **아직 안 읽은 것**. 대부분 여기다

1차에서 **「원리적 불가」 7건이 뒤집혔고 원인은 매번 같았다: 가진 재료의 한계를 문제의 한계로 착각.**

## 4. ★★결과 형식 — 산문 말고 **패치 목록**으로 내라

1차의 최대 실패는 **정정이 `resolved[]` 로그에만 들어가고 본 표(`reads`/`writes`/`signature`/
`constants`/`knobs`/`unknown`)는 옛 값 그대로였던 것**이다. 표를 기계로 소비하는 쪽은 계속 옛 값을 읽었고,
`tcxaudit` 은 base+offset 만 보므로 이 오염을 **못 잡았다**.

⟹ 정정을 찾으면 **정확한 JSON 경로**로 내라:
```
/specs[17]/writes[35]/note   구: "Option<Option<MainObjective>> 3B …"
                             신: "Option<MainObjective>(3B) 단일 Option …"
                             근거: tcx 정본 + m05.ll:17800
```
경로가 없으면 반영이 안 된다. **`~~구~~ → 신` 표기**로 옛 값을 남긴다(§7).

## 5. 산출물 규칙

- 작업 파일은 **자기 배치 전용 디렉터리에만** 쓴다: `_verify2\<배치문자>\`
  (⚠1차에서 배치들이 같은 폴더를 써서 충돌 위험이 있었다. `_verify\` 와 `_spec\` 은 **읽기만**.)
- ⛔**`_spec\specs20.json` 을 직접 고치지 마라** — 패치 목록만 내면 메인이 반영한다.
- ⛔`distruct.json` · `dienum.json` 은 **절대 덮어쓰지 마라.**
- 보고서 `.md` 작성이 하네스에 막히면(1차에 A·B·D 가 막혔다) **본문을 응답에 통째로** 실어라.

## 6. 착수 순서

1. `METHOD_MAP.md §0` 라우팅표 → 무엇을 집을지 정한다
2. `_verify\REPORT_<자기배치>.md` 로 1차 결과를 읽는다(중복 방지)
3. `_spec\specs20.json` 의 자기 5개 + `shared` 를 읽는다 (**이미 정정 반영됨**)
4. 새 재료(§1) 4종을 **실제로 대 본다**. 안 통하면 "왜 안 통하는지"를 범위와 함께 적는다
5. 자기 5개의 `still_unknown` / `unknown` 을 **하나씩 닫으려 시도**한다
