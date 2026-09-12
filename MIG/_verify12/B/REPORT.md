# 12차 배치 B 보고 — 담당 `05`~`09` (게임 0.5.8, 2026-09-11)

> 배치가 하네스 제약으로 파일을 못 써서 **메인이 반환 원문을 그대로 옮겨 적었다**(가공·요약 없음).

신선도: 착수 전·제출 직전 두 번 `python -X utf8 dossierfresh.py 12 B` → 둘 다 **FRESH**.

## 1. 정정 건수

| 분류 | 건수 | 어디 |
|---|---|---|
| **실오류** | **2** | `05 knobs[1].where`(귀속 오기) · `09 logic`(11차가 고친 4단을 옛 3단으로 되돌리는 주석이 같은 블록에 남아 있었다) |
| **보강** | **13** | `08 knobs[4]` value·where·effect 3 · `08 open[2]`·`open[3]` 2 · `09 one_line` 1 · `05 consts[13]~[19]` 7 |
| **행추가** | **1** | `06 knobs` 신설 — 시야 기억창 120틱 (`insert at=5` → v2 `new_knobs[0]`) |
| `behavior_change=true` | **4** | `08 knobs[4].value`·`.effect` · `08 open[2]` · `06 knobs` 신규행 — 전부 **같은 한 사실**(시야 기억창 120틱) |
| `found_by` | new 14 / reused 2 | |

**왜 그 4건이 `behavior_change=true` 인가.** `08 logic` 은 적 챔프 필터를 `.filter(|e| data.blackboard[1-team].is_recent_visible(game, player, e))` 라고만 적고 술어 내용을 「담당 범위 밖」으로 닫아 두었고, `knobs[4]` 는 `value: 0` 에 **「자리표시 — 임계값이 아니라 술어 자체다」**라고 못 박고 있었다. 이 명세만 보고 재구현하면 `is_recent_visible` 을 「지금 시야에 있음」으로 구현하게 되고 (c)(d) 종료 경로가 실제와 다르게 발화한다. 06 도 두 후보 집합이 전부 이 술어로 먼저 걸러지므로 같다.

## 2. ★`05` 의 `kind` 갈림 판정 — **`길이` 가 맞다. `산출값` 6행이 오분류다**

관측(`_verify12/B/obs05.py 5`):

```
c11 val=11 kind=길이   strong=(없음)         weak=DBGSTR@[28955,28957]
c12 val=13 kind=길이   strong=(없음)         weak=DBGSTR@[28959,28961]
c13 val=12 kind=길이   strong=(없음)         weak=DBGSTR@[28971,28973]
c14 val=8  kind=산출값 strong=SELECT@[29150] weak=DBGSTR@[28963,28965],GEP@[...]
c15 val=4  kind=산출값 strong=PHI@[29154]    weak=DBGSTR@[28967]
c16~c19    kind=산출값 strong=PHI@[29154]    weak=DBGSTR@[...]
```

IR 근거 — 9개 패턴 길이는 **전부 같은 성격**이고 **하나도 명령의 피연산자가 아니다**. `m13.ll:28954~28983` 이 (ptr,len) 쌍 전량이다:

```llvm
#dbg_value(ptr @anon…92, !35924, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !35928)
#dbg_value(i64 11,       !35924, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !35928)  ; "PassiveLine"
; 13 "PassiveJungle" / 8 "LineGank" / 4 "Epic"(!35946=contains) / 6 "Serpen"
; 12 "ActiveRecall" / 6 "Recall" / 6 "Battle" / 5 "Nexus"
```

`c14~c19` 에 붙은 `strong`(`SELECT@29150` = `%72 = select i1 %70, i8 8, i8 9` · `PHI@29154` = end_plan phi)은 **end_plan 코드값 4·5·6·8 과 숫자가 겹쳐서** 잡힌 것이지 패턴 길이의 소비 지점이 아니다. `c11~c13` 만 면한 이유도 **겹칠 상대가 없어서**일 뿐이다. ⟹ **`길이` 가 맞고 `산출값` 6행은 값 충돌 오분류**다.

⛔단 **`meaning` 으로는 못 고친다 — 도구 결함이다.** 낱말은 이미 「바이트 길이」라 `_word_kind` 는 9행 전부 `길이` 를 돌려준다. 지는 자리는 `mkspec3._kind` **규칙②**:

```python
strong = dict((k,v) for k,v in obs.items() if k not in KC.WEAK)   # DBGSTR·CALLARG·GEP 제거
if not strong: return word or "미상"                               # ①
if word and any(c in strong for c in KC.SUPPORT.get(word, ())):   # ② ← strong 을 본다
    return word
for c,k in KIND_PRI:
    if c in strong: return k                                      # ③
```

`SUPPORT["길이"] = ("DBGSTR","CALLARG","CMP_EQ")` 인데 앞 둘이 `WEAK` 라 이미 빠져 있다. ⟹ **`kindchk` 가 명시한 「`DBGSTR` 은 구제에 쓰고 기각엔 안 쓴다」가 코드에서 무효**다. 고침 = 규칙②의 `strong` → `obs` **한 낱말**. 전수 영향(`kindfix.py`): `총 consts 193행 · 분류가 바뀌는 행 6` — 전부 05 c14~c19, 부수 피해 0. (`길이` 는 지지가 전부 WEAK 라 여전히 기각엔 못 쓰이고 `CMP_ORD` 관측 시 규칙③이 `임계` 로 되돌리므로, 도시에가 경고한 「배열 길이 6행」은 그대로 보호된다.)

## 3. ★11차가 고친 자리 재검 — **세 자리 모두에서 잔여 오류가 나왔다**

### ⓐ `09` 4단 — 표는 맞다. 그런데 같은 블록 끝에 옛 3단 주석이 살아 있었다
독립 재측정(`rmeta_srcmap game_ai fight_check.rs 1305 1350`, 본문 들여쓰기 2) 잔차 전부 ±0:

| 줄 | bytes | 본문 | 후보 | 잔차 |
|---|---|---|---|---|
| 1316 | 24 | 23 | `2 + if rear_allies >= 1 {`(21) | 0 |
| 1321 | 46 | 45 | `2 + if flank_allies >= 1 && front_allies >= 1 {`(43) | 0 |
| 1326 | 25 | 24 | `2 + if flank_allies >= 2 {`(22) | 0 |
| 1331 | 25 | 24 | `2 + if front_allies >= 2 {`(22) | 0 |
| 1318/23/28 · 1319/24/29 · 1347 · 1348 | 17·4·8·2 | 16·3·7·1 | `return true;` · `}` · `false` · `}` | 0 |

한 줄짜리 3단(`… || flank>=2 {` = 64~66자)은 L1321 이 45자라 **물리적으로 불가**. 카운터 귀속도 증가 지점 `!dbg` 로 독립 확정: `%85`=front(1258/1289/1307) · `%84`=flank(1300/1304) · `%83`=rear(1298).

**남은 오류** = `logic` 맨 끝 주석 `// 1321행의 || 는 IR 에서 비단축 or 로 접혀 두 항의 소스상 순서는 결과에 영향 없음.` — 12줄 위의 「★소스는 `||` 한 줄이 아니라 독립한 if 둘」과 정면 모순. IR 의 `%99 = or i1 %98, %97`(m15.ll:35602)는 1321 안의 두 항이 아니라 **1321(`%97 = select i1 %95, i1 %96, i1 false`)과 1326(`%98 = icmp sgt i32 %84, 1`)을 합친 것**이고, `%98` 은 호이스트로 `!dbg` 가 지워졌으며 합쳐진 `or` 가 `!46393`(line 1321)을 물었다. (`!46388`=1316 · `!46393`=1321 · `!46394`=1331 · `!46402`=1340 · `!46403`=1348)

### ⓑ `05 knobs[1]` — 내용은 맞다. **귀속이 틀렸다**
지시는 「전제(들여쓰기 4)가 실측(6)과 달라 틀렸다」고 했는데 정본엔 **이미 「4 가 아니라 6」**이 있다(그게 11차의 결과). 독립 재측정도 전부 ±0: `L125=112B(2+109) L130=6B(4+1) L131=59B(6+52) L138=82B(8+73) L139=43B(8+34) L140=89B(10+78) L141/142/143=10/8/6B L156=55B(4+50)`.

실제 남은 오류는 **귀속**: 그 문단 머리가 `★정정(10차 배치B, 판정반전)` 인데 내용은 **10차를 뒤집은 11차 판정**이다. 같은 명세 `closed[0]` 은 11차로 올바로 적고 있어 **한 명세 안에서 두 칸이 어긋나 있었다**(G20 은 명세 *사이*만 보므로 이런 명세 *안* 모순은 어떤 게이트도 안 본다). 덧붙여 `!dbg` 사슬(`!36000 = line 138, scope !35913` · `!36002 = line 139`)은 두 독해를 **가르지 못한다**는 것도 확인했다 — 판정은 줄 길이 산술이 하고 있고, 그 산술이 지금 맞다.

### ⓒ 시야 기억창 120틱 — **`06` 에는 실렸고 `08` 에는 안 실렸다**
`grep -c 120` → `spec_06`=1(11차가 `closed[3]` 에 기록) · **`spec_08`=0**. 그리고 08 쪽은 더 나쁘다 — `closed[2]`(「`is_recent_visible` 의 판정 조건, 몇 틱인가 미독해」)가 **닫은 근거 `3차 배치B: 오라클 13/13, 임계 정확히 900`** 으로 닫혀 있는데, **900 은 L194 의 15초 리스폰 임계**라 이 물음과 무관하다. `closed[].why` 는 어떤 게이트도 안 보는 칸이고, 여기선 stale 이 아니라 **애초에 어긋난 근거**였다.

실체(`_gcbc/g07.ll:157005~157047` 원문):

```llvm
%6  = gep %3, 2352   ; judger.info.team  (PlayerState+0x930)
%8  = gep %4, 1472   ; target.id         (Entity+0x5c0)
%10 = gep %2, 248    ; vtable +0xf8  is_visible_cell(team,id) → true 면 즉시 true
%14 = gep %2, 336    ; vtable +0x150 get_entity_by_id(id)     → null 이면 false
%19 = gep %16, 2496  ; 그 적의 info.position (PlayerState+0x9c0)
%22 = gep %0, 480    ; Blackboard + 0x1e0  (i64 [5])
%25 = add i64 %24, 120
%26 = gep %2, 40     ; vtable +0x28 tick()
%29 = icmp uge i64 %25, %28       ; last_seen + 120 >= game.tick()
```

⟹ 「최근 목격」 = **마지막 목격 이후 120틱 이내**(60tps 기준 2초). **08 이 왜 못 찾았나**: `_gaibc` 만 뒤졌다. `METHOD_MAP §1-①`(「`_gaibc` 에 `declare` 만 있으면 `_gcbc`/`_gvbc` 에서 `define` 을 찾아라」)이 그대로 적용되는 자리였다 — 1~6차에 17건이 「본문 없음」으로 포기된 바로 그 실패 형태다. `08 open[3]`(blackboard 인덱스 의미론)도 같은 `define` 이 답이라 함께 닫았다.

## 4. ★도구 결함 · 내 지시 오류

**① `mkspec3._kind` 규칙②가 `strong` 을 본다** → WEAK 지지 kind(`길이`)는 원리적으로 구제 불가. 1낱말 수정, 전수 193행 중 6행만 변화(`kindfix.py`).

**② `_rank_callees` 의 `_anchor` 가 길이접두 없는 부분문자열 매칭** → 거짓 `ev3`. docstring 은 「`<길이><이름>` 이라 부분문자열로 대조된다」고 적었지만 코드는 `s in sym` 이다. 그래서 `AbstractGame` 이 `21AbstractGameWithCache` 안에 걸린다. 내 담당 실례 = **`08 callees[11] game_core::AbstractGame::is_end` 가 `ev3`(IR 호출 심볼 일치)** 인데 08 의 IR 범위(m10.ll 7655~7899)에 그 호출은 **없다**. 실제 걸린 심볼은 m10.ll:7779 의 min_by_key 어댑터 `…21AbstractGameWithCache14iter_champions0E…19EpicHuntAndPokePlan6is_end0E…` = **자기 함수 이름**이다. 전수 = 56행 중 3행(`00 c20`·`08 c11`·`16 c5` — `5range` 가 `12range_adjust` 에 걸린 것 포함, `anchorchk.py`).

**③ `<impl …>` 경로가 `::` 로 쪼개져 실제 직접 호출까지 앵커를 잃는다.** `['…','<impl game_ai','plan_legacy','team_plan','TeamPlan>','v24_…']` 의 `<`/`>` 조각은 어떤 망글링 심볼과도 절대 안 맞는다 → `08 callees[21] v24_objective_setup_should_release_to_passive` 는 m10.ll:7709 에서 **직접 호출되는데도** `ev4 ⚠미확정`. ②+③ 동시 수정 시(`anchorfix.py`): **`총 262행 · 현행 ev3 56행 → 수정판 62행 (신규앵커 +9 / 거짓앵커 -3)`**. 내 몫 = 신규 3(`06 c7` `06 c8` `08 c21`) · 거짓 1(`08 c11`).

**④ `applypatch.EVMARK` 의 tier3 라벨이 `tcx 정본 대조` 하나뿐이다.** `ev_up to:3` 을 쓰면 근거가 DWARF 든 줄길이든 **거짓 문면이 정본에 박힌다**(`evtier` tier3 키워드는 `tcx/MIR/tcxdict/정본/DWARF/srcmap/줄 길이/dienum/distruct/divtable` 로 훨씬 넓다). 그래서 `05 consts[13]~[19]` 7행의 ev4→ev3 은 `ev_up` 이 아니라 **`meaning` 에 DWARF 근거를 직접 적는 `errors[]`** 로 냈다. 고침 = `EVMARK[3]` 를 `evidence` 에서 재료 이름을 뽑아 쓰도록.

**⑤ 지시(도시에)의 오류 4건** — `brief_errors[]` 에 그대로 실었다:
1. §4-b 가 「`kind` 는 `meaning` 낱말을 고쳐라」고만 안내하는데 `05 c14~c19` 는 **낱말이 이미 맞다**. 낱말로 못 고치는 자리가 있다는 것을 말하지 않는다.
2. §5 는 「`old` 는 **정확히 일치**」라 하는데 `applypatch.apply_error` 는 문자열 필드에서 `old in cur` → `cur.replace(old,new)` **부분문자열 치환**이다(정확 일치는 정수·불리언뿐). 정확 일치로 믿으면 `logic` 같은 긴 필드를 통째로 복사해 내게 된다.
3. `05 knobs[1]` 의 상태를 **한 판 옛 것**으로 기술했다(§3-ⓑ).
4. §1 표의 `ev≥4`(05=13)에 **`callees`(tcx 자동생성)**가 섞여 있다 — `errors[]`·`ev_up[]` 어느 쪽으로도 손이 안 닿아 목표가 부풀려진다.

**⑥ 도시에가 물은 `08` 의 잘리던 `ev4 callees`에 대한 답**
- `is_end`(후보 106) → **거짓 앵커**(위 ②). 실제 호출 0.
- `tick`/`is_visible_cell`/`get_entity_by_id`/`get_game_mode` → **「이 방식(IR 호출 심볼)으로는 원리적으로 확정 불가」**. 넷 다 **간접 vtable 호출**(`%40 = call { i64, ptr } %39(...)`)이라 심볼 자체가 없다. 확정 가능한 앵커는 **vtable gep 오프셋**이고 전부 실측: `%38 = gep %37, 64`(0x40 get_game_mode, 7717) · `%50 = gep %47, 256`(0x100 is_visible_cell, 7738) · `%101 = gep %37, 496`(0x1f0 get_entity_by_id, 7853) · `%114 = gep %37, 40`(0x28 tick, 7879). ⟹ 맞는 행은 **트레이트 선언**(`game_core::AbstractGame::*`)이고 `Game`/`SingleLaneGame` 두 impl 은 **런타임 분기**라 하나로 못 고른다. **게이트 제안**: 간접 호출은 vtable gep 오프셋으로 앵커하고 트레이트 decl 을 `ev3`, impl 행은 `⚠런타임 분기(단일 확정 불가)` 로 — 지금처럼 「미확정(재료 부재)」과 뭉치면 안 된다.

## 5. ★불일치 0 도 결과다 — 전수 대조 수치

게이트 미해소는 이 배치 몫 **0건**이었고(`specgate.py` 전량 = G1~G20 중 내 5함수 적발 **0**), 아래는 **게이트가 안 보는 칸을 손으로 전수 대조**한 결과다.

| 축 | 대조 행 수 | 방법 | 불일치 |
|---|---|---|---|
| `mem` 전수 (IR→명세) | **108행** (05:30·06:13·07:29·08:22·09:14) | `irwalk.py <i> mem` — IR 범위의 모든 `load`/`store` 를 그 포인터를 만든 `getelementptr` 오프셋과 짝지어 표와 대조 | **0** (※ 1건은 대상 아님) |
| `consts` 리터럴 전수 (05) | 리터럴 소비 지점 **35줄 전량** | `irwalk.py 5 lit` | **미등재 0** |
| `consts.src_line` (09) | **9행** | `!DILocation` 개별 조회 | **0** |
| `logic` ↔ IR 판정 사슬 (09) | 최종 3블록 + 카운터 증가 6지점 | m15.ll:35560~35640 + `!46433/44/53/54/55/56` | **0**(주석 1건만 stale) |
| `logic` ↔ IR (07 L36) | OR/AND 재결합 5식 | m02.ll:49060~49105 | **0** |
| 줄길이 산술 재현 (05·09) | 05 8줄 · 09 12줄 | `rmeta_srcmap` | **전부 ±0** |
| `callees` 앵커 (전 20명세) | **262행** | `anchorchk.py`·`anchorfix.py` | 거짓 3 · 손실 9 |
| `consts.kind` (전 20명세) | **193행** | `kindfix.py` | 오분류 6 (전부 05) |
| `one_line`·`layer` | 10칸 | 본문 대조 | `09 one_line` 1 보강 · 나머지 9칸 이상 없음 |
| `closed[].why` 재검 | **37건** (05:7·06:7·07:9·08:10·09:4) | 본문 대조 | **1건 어긋남**(`08 closed[2]`) |
| `callers`·`siblings` | 05:2/41 · 06:2/41 · 09:8/0 등 | 표본 대조 | **0** |

※ **05 에서 표에 없는 접근 1건**을 찾았으나 `mem` 의 대상이 아니다 — `BigPlan::get_name` 이 sret 로 돌려준 `String`(24B 스택 alloca `%4`)의 `+0x8`(ptr)·`+0x10`(len)을 **8회** 읽는다(m13.ll:29002/29004 … 29142/29143, 술어 9개 중 8개마다 한 쌍). 게임 구조체 필드가 아니라 로컬 임시값이라 행을 추가하지 않고 **사실 서술**로 남긴다.

### 판정 어휘
- **`표기 불가`** — `05 consts[14]~[19]` 의 `kind`. 동작(패턴 바이트 길이)은 확정됐는데 **현행 `mkspec3._kind` 로는 `길이` 를 표기할 방법이 없다**(낱말은 이미 맞고 규칙②가 WEAK 지지를 못 본다). `patch.json` 으로 못 내고 §2 의 1낱말 도구 수정이 필요하다.
- **`미탐색`** — `06 L22`(「`fight_model::` 접두 없음, L22=53자 ±0」). L22=54B(53자)는 실측했으나 **이 라운드에서 시도한 방식 = 줄 길이 산술 단독**으로는 들여쓰기와 인자 형태(`c` vs `*c`)가 동시에 미지수라 후보가 갈린다. 같은 클로저 L21/L23/L25~L27 을 묶어 **연립으로 푸는 방식은 미탐색**이다.
- **`사실 서술`** — 위 ※. (`notes[]` 는 `errors[]` 로 행을 못 넣으므로 산문으로 보고 — 메인이 넣어 달라.)
- `재료 부재` — 이번 라운드에 새로 닫은 것 없음.

## 6. 실행한 것 (명령줄 그대로)

```bash
cd /c/tfm2mods/MIG
python -X utf8 dossierfresh.py 12 B                 # 착수 전·제출 직전 = 둘 다 FRESH
python -X utf8 specgate.py                          # 전량 — 내 5함수 적발 0
python -X utf8 rmeta_srcmap.py game_ai dive_episode.rs 124 172
python -X utf8 rmeta_srcmap.py game_ai fight_check.rs 1305 1350
python -X utf8 rmeta_srcmap.py game_ai engage.rs 12 40
python -X utf8 _verify12/B/obs05.py 5               # consts kind 관측(신설)
python -X utf8 _verify12/B/irwalk.py {5,6,7,8,9} {mem,call,lit}  # IR→명세 전수 워크(신설)
python -X utf8 _verify12/B/anchorchk.py             # callees 거짓 ev3(신설)
python -X utf8 _verify12/B/anchorfix.py             # 앵커 수정판 영향(신설)
python -X utf8 _verify12/B/kindfix.py               # kind 구제 수정 영향(신설)
python -X utf8 _verify12/B/trial.py                 # 제출 전 G13/G15/G19 사전 반응(신설)
PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 12 --only B --dry   # 16/16
```

IR 원문은 `sed -n '<a>,<b>p' /c/tfm2mods/_gaibc/m{02,10,13,15}.ll` 와 `/c/tfm2mods/_gcbc/g07.ll` 직접 열람. `trial.py` 는 제출 예정 변경(08 knobs[4] 값 교체·06 knobs 행추가)을 v3 사본에 얹어 **G13/G15/G19 가 새로 발화하지 않음**을 미리 확인한 것이다(현행과 동일 — `보류` 1건씩 그대로).

⚠ 적용 후 **`applypatch.py 12 --restamp`** 필요(06 knobs 삽입으로 뒤 인덱스가 밀린다 — dry-run 이 경고로 안내).
