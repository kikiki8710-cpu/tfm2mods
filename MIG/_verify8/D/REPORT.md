# 8차 배치 D 보고 — 축 `history` → 표 **전파** (G17 신설) · 게임 0.5.8

> 정본 스탬프: 작업 시작·`patch.json` 제출 직전 **두 번** `dossierfresh.py 8 D` 실행 → 둘 다 `FRESH`.

## §0 한 줄 결론

**이 축은 이미 닫혀 있었다.** `history[]` 188행 전량을 기계 4규칙 + 손검증으로 훑어
전파 실패는 **1건**(`specs[15] consts[1]`)뿐이다. 그래서 이 배치의 산출물은
「적발 목록」이 아니라 **재발을 막는 게이트(`G17`)와, 그 게이트의 오탐률 실측치**다.

| 항목 | 수 |
|---|---|
| `G17` 적발(엄격판) | **1건** |
| 손반증으로 걸러낸 오탐(느슨판 → 엄격판) | **9건** (오탐률 90%) |
| `patch.json` `errors[]` | **1건** (`보강` · `behavior_change=false`) |
| `patch.json` `ev_up[]` | 0건 |
| `brief_errors[]`(내 지시의 오류) | **4건** (판정 반전 1건 포함) |
| `G9` 처리 | **닫지 않았다 — 7차 제안을 반증하고 대안 3종을 실측 비교** (§5) |

## §1 무엇을 실제로 실행했는가 (명령줄 그대로)

```bash
cd /c/tfm2mods/MIG
python -X utf8 dossierfresh.py 8 D                      # 착수 시 · 제출 직전 → 둘 다 FRESH
python -X utf8 specgate.py                              # 전량 (G12=8 · G9=4)
python -X utf8 specgate.py --gate G9                    # (주의) 요약줄이 G12=0 으로 찍힌다(§5-4)
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/D/probe1.py    # history 취소선 전수 → 4개
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/D/probe2.py    # 표의 placeholder 문면 → 2개
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/D/probe3.py    # 정정 선언 90/188 · 오프셋 앵커 실태
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/D/probe4.py    # 「A 가 아니라 B」 쌍 → 3개
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/D/probe5.py    # 페이로드 토큰 전량미전파 → 9(전부 방법론어)
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/D/probe6.py    # 오프셋↔이름 결속 → 3 (전건 base 충돌 오탐)
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/D/probe7.py    # history 의 표 칸 지목 → 6
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/D/probe8.py    # ev N→M 선언 ↔ 표의 ev → 전건 일치
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/D/probe9.py    # 규칙 C 값앵커판 → 1
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/D/probe10.py   # 규칙 E(base 필수)·F → 둘 다 0
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/D/probe11.py   # G9: 안① Self 타입
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/D/probe12.py   # G9: 안①/안② 건전성 회귀
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/D/probe13.py   # G9: 안③ memn 버그
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/D/probe14.py   # G9: 안③ 적용 실건수 4→2
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/D/gate.py          # G17 엄격 → 1건
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/D/gate.py --loose  # G17 느슨 → 10건(오탐 9)
PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 8 --only D --dry   # 정정 1/1 성공
```
손검증(표를 직접 열어 대조)한 `history` 행 = 함수 20개 전역에 걸쳐 **약 25행**.
`_verify8/D/dump.py <i> [field] [정규식]` 로 표를 펴서 눈으로 대조했다.

## §2 G17 — 무엇을 신호로 썼고, 왜 그것인가

`history.now` 는 자유 서술이라 문장 단위 대조가 **불가능**하다(판정 어휘: `표기 불가`).
그래서 **문면 규약이 이미 만들어 둔 구조적 신호**만 골랐다. 규칙 4종 전부
「표에 **무엇이 있다**」를 조건으로 걸었다 — 「후보가 있는데 주장이 없다」 꼴의 **부재 판정**은
7차 G12 를 고치다 오탐을 16→22 로 늘린 바로 그 형태라 의도적으로 배제했다.

| 규칙 | 신호 | 근거 | 적발 |
|---|---|---|---|
| **A** 취소선 | `history` 안의 `~~X~~` 가 표에 취소선 없이 살아 있다 | 취소선은 「이 값은 폐기됐다」는 **명시 선언**. `G8` 과 동일 판정이고 **원천만** `logic`→`history` 로 바꾼 짝 | 0 (원천 취소선 자체가 4개뿐) |
| **B** 등급 이동 | `ev N→M` + `consts[2]` 꼴 **칸 지목** ↔ 표의 `ev` 정수 | `ev` 는 표의 **정수 필드**라 완전 대조가 된다. 칸 지목이 없는 것은 대상이 산문에만 있어 후보로도 안 올린다 | 0 (지목 6건 전건 일치) |
| **C** 폐기 선언 | `노브가 아니다`/`소비처 0건` 류 + **폐기된 수치가 그 행의 `value` 칸에 그대로 있음** + 행에 폐기 흔적 없음 | 폐기 선언은 거의 항상 **특정 값**에 대한 것이다. 값 앵커가 없으면 일반 토큰 하나로 아무 행에나 붙는다(§3) | **1** |
| **D** 확정↔placeholder | 표 행이 `미상`/`불명`인데 **같은 (base, offset)** 을 `history` 가 확정 | `base` 를 빼면 `+0x10`·`+0xc0`·`+0x1e0` 이 서로 다른 구조체에 동시에 있어 **3/3 전건 오탐**(probe6 실측) | 0 |

시험했다가 **버린 신호** 2종(전부 0건이라 게이트에 안 넣었다 — 미래 라운드가 다시 파지 않도록 남긴다):
- **E** `Base+0xNN = name` 결속을 base 필수로 대조 → **0건**. 즉 오프셋↔이름은 전부 전파돼 있다.
- **F** 인용문을 `거짓/오답` 으로 선언한 것이 표에 잔존 → **0건**.

## §3 적발을 내가 직접 반증했다 (건수는 성능이 아니라 가설이다)

규칙 C 의 **초판**(값 앵커 없음)이 11건을 냈다. 표를 직접 열어 대조한 결과 **10건이 오탐**이었다.
`TRACE` 어휘를 넓히니 10건(오탐 9)이 됐고, **값 앵커를 넣으니 1건**이 됐다.

| # | 후보 | 반증 근거 | 판정 |
|---|---|---|---|
| 1 | `01 consts[11]`(+5 based 보너스) | history[3] 이 폐기한 것은 **티어 컷 노브**다. +5 보너스는 별개이고 오라클 20행 MATCH 로 살아 있다 | 오탐 |
| 2 | `01 mem[6]`(camp_type.__0) | 폐기의 **근거 쪽** 사실이지 폐기 대상이 아니다. 표기도 「팀 인덱스」로 정정 완료 | 오탐 |
| 3 | `03 knobs[6]`(die_tick 초→틱 60) | history[6] 이 폐기한 것은 「**슬롯별** 가중치 상수」. 이 60 은 fight_check.rs:1107 의 별개 하드 리터럴(tps 1/6/30/60/120 전부 60 고정 실측) | 오탐 |
| 4·5 | `10 mem[20]/[21]`(적 챔프 x/y) | history[3] 이 폐기한 것은 「**version 분기표**」. 좌표와 무관 | 오탐 |
| 6 | `15 consts[0]`(태그 0=TryKill) | 폐기 대상은 **두 번째 usize(60)**. 판별자 0 은 다른 값 | 오탐 |
| 7·8 | `15 mem[13]`(tag) · `15 mem[14]`(TryKill.0=target_id, +0x8) | 폐기 대상은 `TryKill.__1`(**+0x10**). 인접 필드를 잘못 집었다 | 오탐 |
| 9 | `15 mem[5]`(team_plan) | 행에 「피호출자는 이 필드를 **변경하지 않는다**」가 이미 있다. **게이트의 `TRACE` 어휘가 좁아서** 난 오탐 → 어휘 보강으로 해소 | 오탐(게이트 결함) |
| 10 | `18 knobs[1]`(세르펜 징벌 2중 AND) | history[2] 가 폐기한 것은 「version **분기표**」이고 그 분기는 `fight_model::resolve_fight` 안 = **다른 함수 담당**이라 이 노브가 서술을 바꿀 이유가 없다 | 오탐 |
| **11** | **`15 consts[1]`(value 60)** | **진짜다** — §4 | **실오류(보강)** |

## §4 유일한 적발 — `specs[15] consts[1]`

`history[1]` 은 `BattlePlanGoal::TryKill` 두 번째 usize 60 을
「**소비처가 존재하지 않는다 — 노브가 아니다**」로 닫았다(생성 14곳 전부 리터럴 · 읽는 코드는
`BattlePlanGoal::Debug::fmt`(m10.ll:61999) 하나뿐 = 로그 출력 전용).

그 결론이 **세 형제 칸 중 둘에만** 실렸다:

| 칸 | 현재 문면 | 전파 |
|---|---|---|
| `mem[15]` | 「소비처 0건 — 노브가 아니다(history 참조)」 | O |
| `knobs[4]` | 「산술 소비처 0건이라 사실상 노브가 아니다(history 참조)」 | O |
| **`consts[1]`** | 「…도 전부 60 — **공통 파라미터**」 | **X 누락 + 오도** |

「공통 파라미터」는 **튜닝 가능한 값**으로 읽힌다. 7차가 `mem[15]`·`knobs[4]` 를 같은 사유로
실오류 처리했으므로 **같은 사건의 세 번째 형제**다. `errors[0]` 으로 냈다.

- `kind` = **보강**. 값(60)·줄번호(253)는 맞고 틀린 것은 **문면**이다.
- `behavior_change` = **false**. 이 명세대로 재구현해 goal+0x10 에 60 을 넣으면 게임과 같다.
  (「노브 목록에 올린다」는 오도이지 `game==mine` 을 깨는 오류가 아니다.)
- 주의 — **파생 필드**: `consts.kind` 는 `mkspec3.py` 가 `meaning` 의 낱말에서 정하므로
  이 정정으로 `임계` 분류가 바뀔 수 있다. `errors[]` 로 직접 못 쓰는 필드라 **메인이 반영 후 확인**할 것.
  (사실 소비처 0건인 값에 `임계`는 부적절하다 — 바뀐다면 개선이다.)

## §5 `G9` — 닫지 못했다. **7차 제안을 반증했다** (판정 반전 1건)

`G9 callees 오염=4`(`07 target_bush` / `13`·`14` `line·nearest_enemy·position·team` / `15 chats`).
**먼저 4건 전부를 손검증했다 — 전건 오탐이다**(필드 접근이 맞다):
`target_bush` = `EpicHuntAndBattlePlan` 필드 · `nearest_enemy` = `Minion`/`Tower` 필드
(04 h[5] 가 「`Minion::nearest_enemy`(Entity +0x88/+0x90)」로 이미 확정) ·
`position`/`team` = `PlayerState.info` 필드 · `chats` = `SinglePlanBattle+0x68 Vec<Chat>` 필드.

### 5-1 7차 배치B 제안은 채택하면 안 된다 (실측)
제안 = 「동명 함수가 **다른 Self 타입**이면 후보에서 빼라」.
회귀 시험(= G9 가 원래 잡으라고 만들어진 **4차 D-E2 의 Entity 4메서드**를 지우지 않는가):

| 후보 | 안①(다른 Self) | 안②(mem base) | 안③(memn 버그수정) |
|---|---|---|---|
| `attack_effect` | 유지 O | **제외 X** | 제외 X |
| `skill_effect` | 유지 O | **제외 X** | 제외 X |
| `skill2_cooldown` | **제외 X** | 유지 O | 제외 X |
| `ult_cooldown` | **제외 X** | 유지 O | 제외 X |

⟹ **세 안 모두 회귀를 깬다.** 안①은 필드 소유자(`Champion`/`Bear`)와 함수 Self(`Entity`)가
달라서 진짜 메서드를 지우고, 안②·안③은 명세가 그 오프셋을 `mem` 에 적어 뒀다는 이유로 지운다.
**판정 반전 1건**: 도시에가 「실제로 `07 target_bush` 는 …」이라며 검증된 해법처럼 제시했으나
그 규칙은 G9 의 본래 목적을 무력화한다.

### 5-2 그래도 남는 개선 — `gate9` 의 `memn` 수확에 **실제 버그**가 있다
현행(`specgate.py` gate9)은 `for part in (x.get("name") or u"").split("."): memn.add(part)` 다.
`mem[16].name = "ty.Tower.nearest_enemy 태그(Option<(usize,usize)>, range 0..2)"` 처럼
**주석 꼬리**가 붙으면 마지막 조각이 `nearest_enemy 태그(...)` 가 되어 `nearest_enemy` 가 안 들어간다.
⟹ 명세가 자기 표에 「이건 필드다」를 써 뒀는데도 손확인으로 올라온다.
식별자 토큰화(`re.findall(r"[A-Za-z_]\w*", name)`)로 고치면 실측 **G9 4건 → 2건**(`13 line`·`15 chats`).

### 5-3 권고 (`MIG` 루트는 고치지 않았다 — 제안만)
1. `gate9` 의 `memn` 버그는 고쳐라(순수 버그 수정, 추론 아님). **단** 그것만으로 회귀가 보장되지는 않으니
   `G9 강함`(enum 관통) 분기는 `memn` 면제를 유지할 것.
2. **`G9 손확인`을 `specgate` 요약표의 오류 건수에 세지 마라.** 게이트 주석 자신이
   「기계로 못 가르는 것. 목록만 낸다」고 적어 놓았는데 요약표가 **오류 4건**으로 집계해
   3라운드째 「미해소」로 보이게 하고 있다. 별도 섹션(`[정보] 손확인 N개`)으로 분리하는 것이 맞다.
3. `G9` 를 진짜로 닫으려면 필요한 재료는 **수신자의 타입**이다 — `logic` 의 `a.b.c` 에서 `b` 의 타입을
   tcx 로 해석해야 하고, 그건 `logic` 산문 파싱이라 **미탐색**(현재 재료로 「원리적 불가」가 아니라
   「안 해봤다」다. 범위 = `distruct` 필드 타입 사슬을 따라가는 타입 추론기를 새로 짜는 일).

### 5-4 부수 발견 — `specgate.py --gate` 의 요약줄이 거짓말을 한다
`--gate G9` 로 돌리면 **선택하지 않은 게이트가 전부 0 으로 찍힌다**
(요약줄이 `GATES` 전체를 순회하며 `CNT` 를 읽는데 안 돌린 게이트의 `CNT` 가 0이라서).
실측: `--gate G9` → `G12 src_line 대조=0`, 전량 실행 → `G12 src_line 대조=8`.
다음 배치가 이 줄을 「현황」으로 읽으면 **「G12 는 해소됐다」로 오독**한다.

## §6 판정 어휘 (§5-b 규칙)

| 항목 | 판정 | 범위 |
|---|---|---|
| `history.now` 를 문장 단위로 표와 대조하는 것 | **표기 불가** | 자유 서술이라 대조 대상 형식이 없다. 그래서 A~D 의 **구조적 신호**로 우회했다 |
| 규칙 E(오프셋↔이름 결속)·F(거짓선언 인용문 잔존) | **재료 부재 아님 — 실행 결과 0건** | 재료(`base`+`offset`, 인용부호 문면)는 있었고 **실제로 전파가 다 돼 있었다** |
| `G9` 손확인 4건의 기계 분리 | **미탐색** | 수신자 타입 추론기를 안 짜 봤다. 「원리적 불가」가 아니다 |
| `12 h[9] 명세정정`(chat_allowed 태그 50 → :131 병합) | **사실 서술 / 범위 밖** | 그 표는 spec 12 의 `mem`/`consts`/`knobs` 가 아니라 `_shared`·`resolved` 에 있다. 이 축(표 전파)의 대상이 아니므로 `errors[]` 로 내지 않았다 |
| `15 h[9]` 의 신규 상수 `15000`(rs:925)·`max(1)`(rs:930) | **보류** | `single_tower_dive_is_viable` 은 **피호출자**라 그 리터럴이 spec 15 `consts` 에 들어가야 하는지가 `SPEC_GUIDE §3` 로 확정되지 않았다. 판정 없이 남긴다 |

## §7 다음 라운드에 남기는 것

- **`G17` 승격**: `_verify8/D/gate.py` → `check_spec(sp)` 진입점 구비(`G13`/`whereline` 과 같은 모양).
  `specgate.py` 에 `import gate as G17` 로 붙이면 된다. 이 배치는 `MIG` 루트를 고치지 않았다.
- **`G17` 의 가치는 적발 수가 아니라 회귀 방어다.** 지금 1건이지만, 다음 라운드가 `history` 에
  새 결론을 넣고 표를 안 고치면 **그 자리에서** 걸린다. 특히 **형제 칸 누락**
  (`mem`/`knobs` 는 고치고 `consts` 를 빼먹는 것)이 이번 유일 적발의 형태였다.
- `--loose` 는 **후보 제시형**이다. 오탐률 **90%(9/10)** 를 실측해 게이트 실행 시 배너로 찍게 해 뒀다.
  건수를 성능으로 읽지 마라.
