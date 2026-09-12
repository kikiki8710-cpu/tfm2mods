<!-- ★`mkdossier.py` 가 생성한다. 손으로 고치지 마라 — 다음 생성에 날아가고, 손으로 쓴 수치가 5차까지 반복된 지시 오류의 원인이었다. -->
# 7차 배치 A 도시에 — 담당 `00`~`04` (게임 0.5.8)

## §0 정본 스탬프 — **읽기 전에 스스로 확인하라**

| 정본 | sha256[:16] | mtime |
|---|---|---|
| `_spec/specs20_v3.json` | `dab6d7c803e1be63` | 2026-09-11 17:07:14 |
| `_spec/specs20.json` (손이 닿는 정본) | `6320fa42c1107160` | 2026-09-11 17:05:21 |

이 파일 생성 = `2026-09-11 17:58:27`

```bash
cd /c/tfm2mods/MIG && python -c "import hashlib,io;print(hashlib.sha256(io.open('_spec/specs20_v3.json','rb').read()).hexdigest()[:16])"
```
출력이 위 해시와 **다르면 이 도시에는 낡았다** — `python -X utf8 mkdossier.py 7` 를 다시 돌려라.

> ★**v3 를 직접 고치지 마라.** 정본은 `specs20.json`(v2)이고 v3 는 `mkspec3.py` 의 생성물이다.
> 네가 할 일은 파일 수정이 아니라 **`patch.json` 제출**이다(§5).

## §1 담당 함수와 이번 라운드의 표적

| # | 이름 | vis | mem | consts | knobs | open | ev≥4(미실행) |
|---|---|---|---|---|---|---|---|
| `00` | ult | pub | 29 | 10 | 9 | 0 | 9 |
| `01` | calculate_jungle_action_score | pub | 8 | 12 | 7 | 0 | 4 |
| `02` | sub_plan | pub | 21 | 6 | 5 | 1 | 2 |
| `03` | defensive_crisis | pub | 22 | 7 | 21 | 1 | 15 |
| `04` | handle_line_defense | pub | 27 | 5 | 7 | 1 | 2 |

★**`vis` 가 `pub` 이 아니면 오라클 직접 진입이 막힌다.** 5차에 `18`(`in:game_ai`)이
그걸 모르고 들어갔다가 47행을 그대로 남겼다 — `pub` 이 아니면 처음부터 **상위 `pub` 래퍼나**
**형제 복제본**을 노려라.

★**`ev≥4`** = IR 독해(4)·추론(5)뿐이라 **실행으로 확인되지 않은 행**이다. 오라클로 내려라.
⚠단 이 수는 **실행 대상 수가 아니다** — `knobs` 상당수가 인라인된 **다른 함수의 줄**을 가리켜
네 함수에 진입해도 안 닿는다(7차 배치A 적발). 숫자를 목표로 삼지 말고 **닿는 것부터** 내려라.
`ev` 는 손으로 매기는 값이 아니라 **근거 문면에서 파생**된다(`mkspec3.evtier`) — 
숫자를 고치려 하지 말고 **근거를 바꿔라**(예: 「오라클 실행 확인: …」을 `note` 에 쓰면 2로 내려간다).

## §2 읽어야 할 정본 — **요약본이 아니라 원문을 읽어라**

여기 요약은 없다. 아래 파일을 **직접 Read** 하라. 이 표는 「무엇이 어디 있고 지금 내용이 무엇인지」만 준다.

| 파일 | 줄 | sha256[:16] | 왜 |
|---|---|---|---|
| `MIG\METHOD_MAP.md` | 208 | `3920863caca9e0a3` | ★**「불가」를 쓰기 전에 §0 라우팅표를 보라.** 09-11 에 「원리적 불가」가 7건 뒤집혔고 원인은 매번 「가진 재료의 한계를 문제의 한계로 착각」이었다. |
| `MIG\SPEC_RUNBOOK.md` | 512 | `230d44256995d546` | ★라운드 절차. **§S5-b 게이트표 · §S5-c 「검사받지 않는 축」 · §S5-d 파이프라인 4계약**. |
| `MIG\SPEC_GUIDE.md` | 364 | `0b45fe38ad5ebf34` | 명세 필드의 뜻과 채우는 법. |
| `MIG\TOOLS.md` | 208 | `9edf59c5e846ae09` | 도구 인벤토리(자동 생성). **무엇이 있는가**만 센다 — 무엇을 집는가는 `METHOD_MAP §0`. |
| `MIG\_verify3\TEMPLATE.rs` | 226 | `c7f1a3efd83d7087` | ★오라클 작성 템플릿. **함정 ①~⑧ · 수법 ⓐ~ⓕ** — 오라클을 쓸 거면 먼저 읽어라. |

## §3 담당 함수 명세 **전문** (무손실 — 잘린 칸이 없다)

> 표의 어떤 칸도 `...` 로 줄이지 않았다. 길어 보이는 게 정상이고, **줄인 자리가 곧 지시 오류가 난 자리**였다.

★**함수마다 파일이 따로 있다. 한 번에 하나씩 Read 하라.**
(7차 첫 시도에서 도시에를 통째로 읽으려던 배치 둘이 600초 무진전으로 죽었다. 내용은 그대로고 파일만 나눴다.)

| # | 이름 | 파일 |
|---|---|---|
| `00` | ult | `MIG\_verify7\A\spec_00_ult.md` |
| `01` | calculate_jungle_action_score | `MIG\_verify7\A\spec_01_calculate_jungle_action_score.md` |
| `02` | sub_plan | `MIG\_verify7\A\spec_02_sub_plan.md` |
| `03` | defensive_crisis | `MIG\_verify7\A\spec_03_defensive_crisis.md` |
| `04` | handle_line_defense | `MIG\_verify7\A\spec_04_handle_line_defense.md` |

## §4 게이트 미해소 — **이 배치 몫만 추렸다**

★**이번 라운드의 주 표적이다.** 6차 통제실험 결론 = 「오류는 **검사받지 않는 축**에 고인다」 — 
`consts.src_line` 은 5라운드 동안 아무도 안 봤고 G12 를 붙이자 **즉시 56건**이 나왔다.
그 오류들은 **1차부터 그대로 있었다.** 아래는 아직 안 닫힌 것들이다.

**이 배치 몫 = 15건** (게이트 머리줄의 `G12=47`·`G13=8` 은 **20함수 전역 합계**다 — 혼동 말 것)

⚠★**게이트의 「실제 후보」를 그대로 믿지 마라.** 7차 배치A 가 G12 8건을 전수 검증했더니 **전부 오탐**이었다 — `srclinecheck.py` 가 ①`switch` case 라벨 줄에 `!dbg` 가 없어 못 보고 ②`phi` 상수를 `line 0` 으로 버리고 ③리터럴 정규식이 `%2`·`%16` 같은 SSA 레지스터와 gep 오프셋까지 잡는다. **지적된 줄의 IR 원문을 직접 열어 확인**하고, 명세가 옳으면 `kind: "오탐"` 으로 보고하라(고치는 게 아니라 게이트를 고쳐야 한다).

```

**[G12 src_line 대조] 47건**
  [00] ult                                             consts[3] src_line=293 인데 IR 사슬에 그 줄이 없다
  실제 후보 = [310, 336, 338, 340]
  [00] ult                                             consts[4] src_line=293 인데 IR 사슬에 그 줄이 없다
  실제 후보 = [281, 328]
  [01] calculate_jungle_action_score                   consts[3] src_line=528 인데 IR 사슬에 그 줄이 없다
  실제 후보 = [522, 523, 530, 542]
  [01] calculate_jungle_action_score                   consts[5] src_line=530 인데 IR 사슬에 그 줄이 없다
  실제 후보 = [542]
  [01] calculate_jungle_action_score                   consts[8] src_line=536 인데 IR 사슬에 그 줄이 없다
  실제 후보 = [522]
  [01] calculate_jungle_action_score                   consts[10] src_line=536 인데 IR 사슬에 그 줄이 없다
  실제 후보 = [542, 548]
  [02] sub_plan                                        consts[1] src_line=42 인데 IR 사슬에 그 줄이 없다
  실제 후보 = [36, 37]
  [02] sub_plan                                        consts[4] src_line=50 인데 IR 사슬에 그 줄이 없다
  실제 후보 = [36, 37]

**[G13 knobs.where 대조] 8건**
  [00] ult                                             knobs[0] 인용한 명령이 그 줄(±2)에 없다
  m04.ll:44033  인용=icmp ugt i64 %30, 4
  [00] ult                                             knobs[1] 인용한 명령이 그 줄(±2)에 없다
  m04.ll:44283  인용=llvm.usub.sat.i64(%205, 150000)
  [03] defensive_crisis                                knobs[1] 인용한 명령이 그 줄(±2)에 없다
  m10.ll:34144  인용=icmp ugt %85, %17
  [03] defensive_crisis                                knobs[3] 인용한 명령이 그 줄(±2)에 없다
  m10.ll:34081  인용=icmp ugt %76, 2
  [03] defensive_crisis                                knobs[4] 인용한 명령이 그 줄(±2)에 없다
  m10.ll:34086  인용=icmp ugt %76, 4
  [04] handle_line_defense                             knobs[5] 인용한 명령이 그 줄(±2)에 없다
  m04.ll:60446  인용=icmp slt i64 %22, -3000
  [04] handle_line_defense                             knobs[6] 인용한 명령이 그 줄(±2)에 없다
  m04.ll:60451  인용=icmp slt i32 %26, -2
```

### §4-b 아직 **게이트가 없는 축** — 여기를 의심하라

| 축 | 전체 건수 | 상태 |
|---|---|---|
| `mem.dir`(읽기/쓰기 방향) | 451 | ⚠**무검사** — IR 의 load/store 와 대조된 적이 없다 |
| `consts.kind`(상수 종류) | 186 | ⚠**무검사** |
| `sig.params.role`(인자 역할) | 124 | ⚠**무검사** |

⚠`consts.kind` 는 **v2 에 없는 파생 필드**다(`mkspec3.py` 가 `meaning` 의 낱말로 정한다) — 
`errors[]` 로 직접 못 쓰니 **`meaning` 쪽 낱말**을 고쳐야 한다(7차 배치A 가 3건 거부당했다).

⟹ 담당 함수에서 **표본을 떠서 직접 대조하라.** 틀린 게 나오면 그건 「그 축 전체가 무검사였다」는 뜻이고,
보고에 **검사기를 어떻게 만들면 되는지**까지 적어라(그게 다음 라운드의 게이트가 된다).

★**불일치 0 도 결과다.** 7차 배치A 가 `mem.dir` 89행을 대조해 **불일치 0** 을 냈다 — 
「계측기를 붙이면 오류가 나온다」가 법칙이 아니라는 반대 사례이고, 그 축은 닫아도 된다는 뜻이다.

## §5 보고 형식 — **`patch.json` 하나. 산문 보고서는 부수적이다**

★5차에 ev 상향 **492행 중 317행이 유실**됐다. 원인은 보고를 **집계표로 받은 것**이다.
기계가 적용할 수 있는 형식으로만 받는다 — 사람이 옮겨 적는 경계를 없앤다.

★**이 스키마는 `applypatch.py` 의 실제 계약이다.** 다른 최상위 키를 쓰면 도구가 **적용을 거부**한다
(7차 배치A 적발 — 이 자리에 없는 계약 `entries` 가 예시로 실려 있었고, 그대로 냈으면 **0건 적용**이었다).

```json
{
  "round": 7, "batch": "A",
  "errors": [
    {"path": "/specs[0]/consts[3]/src_line",
     "kind": "실오류",
     "old": 293, "new": 310,
     "evidence": "m04.ll:44120 `store i64 2, !dbg !56400` (!56400 = abstract_input.rs:310).
                   293 은 함수 머리줄이다",
     "behavior_change": false,
     "found_by": "new"}
  ],
  "ev_up": [
    {"path": "/specs[0]/mem[0]", "from": 4, "to": 3,
     "evidence": "tcx 정본 대조: offset_of!(PlayerState, info.team)==0x930",
     "found_by": "new"}
  ],
  "brief_errors": ["이 도시에에서 발견한 내 지시의 오류 — 문장으로"]
}
```

| 키 | 뜻 |
|---|---|
| `errors[]` | 값·문면 정정. `kind` = `실오류` / `오탐`(게이트가 틀렸다) / `보강` |
| `errors[].evidence` | ★**IR 줄 원문을 인용하라.** 「확인했다」는 근거가 아니다 |
| `errors[].behavior_change` | 이 정정이 **재현 동작을 바꾸는가**(노브 방향 뒤집힘 등) |
| `errors[].found_by` | `new`(이번에 처음) / `reused`(전 라운드 기법 재사용) — **집계에 쓴다** |
| `ev_up[]` | 증거등급 **이동**. `from`/`to` 는 숫자지만 **`ev` 필드를 직접 쓰는 게 아니라** 근거 갱신 요청이다 |
| `brief_errors[]` | ★**내 지시(이 도시에)의 오류.** 매 라운드 나왔다 — 비워 두지 마라 |

제출 전 **반드시** 사전 검증하라(적용 없이 성공/실패만 본다):
```bash
cd /c/tfm2mods/MIG && PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 7 --only A --dry
```

경로 문법(`applypatch.py` 가 파싱한다):

```
/specs[i]/<필드>                     예: /specs[3]/one_line
/specs[i]/<배열>[n]/<키>             예: /specs[3]/consts[2]/src_line
/specs[i]/<바깥>/<필드>              예: /specs[3]/sig/vis
/specs[i]/<바깥>/<배열>[n]/<키>      예: /specs[3]/sig/params[1]/role
```

규칙:
- `old` 는 **현재 값과 정확히 일치**해야 한다(불일치 = 그 항목만 거부 · 나머지는 적용).
  ⟹ 네가 본 값이 정본과 다르면 **네 도시에가 낡은 것**이다. §0 해시부터 다시 확인하라.
- 새 항목 추가는 `"op": "append"` + `"path": "/specs[i]/notes"` + `"new": {...}`.
- ★**`ev` 를 `errors[]` 로 직접 쓰지 마라** — `ev` 는 근거 문면에서 파생되는 값이다(`mkspec3.evtier`). 등급을 옮기려면 **`ev_up[]`** 을 쓰고 `evidence` 에 새 근거를 대라.
  (`mem` 의 `ev` 는 근거가 무엇이든 **상한 3**이다. 오프셋 주장의 최강 근거가 tcx 이기 때문.)
- ⚠**파생 필드는 `errors[]` 로 못 쓴다.** `consts.kind` 는 v2 에 없고 `mkspec3.py` 가 `meaning` 의 낱말에서 정한다 — 분류를 고치려면 **`meaning` 쪽 낱말**을 고쳐라(7차 배치A 적발).
- ⛔`_spec/specs20*.json` 을 **직접 쓰지 마라**(읽기 전용). 반영은 메인이 `applypatch.py` 로 한다.

생성 보조: `python -X utf8 mkpatch.py --help` (참조구현 — 손으로 JSON 을 쓰다 오타를 내지 마라)

### §5-b 산문 보고서에 **반드시** 적을 것

1. ★**판정 어휘를 골라 써라** — 「불가」 한 단어로 뭉치면 다음 라운드가 시도조차 안 한다:
   - `미탐색` = 아직 안 해봤다
   - `재료 부재` = 해봤는데 재료가 없다 → **어떤 재료를 어떻게 시도했는지 범위를 열거**하라
   - `표기 불가` = **동작은 확정됐는데** 명세 칸에 담을 형식이 없다
   - `사실 서술` = 물음이 아니다 → `notes[]` 로 보내라(`open` 에 두지 마라)
2. **무엇을 실제로 실행했는지** — 명령줄 그대로. 「확인했다」만 쓰면 재현이 안 된다.
3. **내 지시(이 도시에)의 오류**를 찾으면 그것부터 보고하라. 6차까지 매 라운드 나왔다.
4. **판정 반전도 오류로 센다**(유저 확정). 「전에 A 라 했는데 B 였다」면 그건 오류 1건이다.

## §6 출력 디렉터리 — **_verify7/A 만 써라**

```
MIG/_verify7/A/patch.json      ← 기계 적용분(필수)
MIG/_verify7/A/REPORT.md       ← 산문 보고서
MIG/_verify7/A/oracle/         ← 오라클 .rs · 출력 로그
```

⚠**다른 배치 폴더에 쓰지 마라.** 네 배치가 동시에 돌아 경합한다.
⚠`_spec/` 아래 어떤 파일도 쓰지 마라(읽기 전용).
⚠`distruct.json`·`dienum.json` 은 **절대 수정 금지**.

