# 6차 반증검증 — 배치 D (`specs[15]`~`specs[19]`) 보고서

게임 0.5.8 / SDK `sdk_058` / toolchain `nightly-2026-05-24` / 2026-09-11
작업 폴더 = `C:\tfm2mods\MIG\_verify6\D\`
프로브 = `D6_o1.rs`(offset 일괄) `D6_o2.rs`(18 pub 헬퍼 진리표) `D6_o3.rs`·`D6_o4.rs`·`D6_o5.rs`(18 도달 추적)
`D6_o6.rs`(타워 이터레이터·update version) `D6_o7.rs`(engage_requires_dive·single_tower_dive_is_viable)
출력 = `o1.txt` `o2.txt` `o3.txt` `o4_h0.txt` `o5.txt` `o6.txt` `o7.txt`
보조 스크립트 = `count.py`(ev 재집계) `ev4list.py`(표적 열거) `view.py`(명세 열람)
게이트 출력 = `gate_specgate.txt` `gate_auditrounds.txt` `gate_audit4.txt` `gate_prose.txt`
기계 판독 패치 = **`patch.json`** (정정 6 · ev 상향 38)

---

## ① 결론 한 줄

**실오류 1건(그중 `reused` 1) · 판정반전 0 · 새 발견 13 · ev 상향 38행** (+ 5차 유실분 복구 2건·보강 3건).
`offset_of!` 일괄 대조는 **107/107 MISMATCH 0**, 오라클 진리표는 **560칸 전부 명세와 일치**,
`ev<=3` 표본 재확인에서 **뒤집힘 0건**. ⟹ **같은 조건에서 명세는 흔들리지 않았다.**
유일한 실오류도 「새 사실」이 아니라 **2차에 이미 확정된 정정이 표 한 칸에만 반영되지 않은 것**이다.

> ★이번 라운드 최대 소득은 오류가 아니라 **표적 `18` 의 미탐색 범위를 절반 이하로 좁힌 것**이다(§5·아래 N4~N6).

---

## ② `patch.json` 요약

### 정정 6건 (`applypatch.py 6 --only D --dry` → **정정 6/6 · ev상향 38/38 · 적용 실패 0**)

| # | 경로 | kind | 요지 | `found_by` |
|---|---|---|---|---|
| E1 | `/specs[18]/knobs[1]` | **실오류** | `is_object_being_taken_by_enemy(…, **true**)` → `WavePriorityObject::Serpen(=1)`. 2차에 확정된 정정이 `logic`·`closed[1]`·`history[3]` 에만 반영되고 **이 행에는 `true` 가 남아 있었다** | `reused` |
| E2 | `/specs[16]/knobs[6]` | 보강(**5차 유실 복구**) | HARD_CC 제외가 「Taunt·Animation 2개」로 적혀 있으나 실제 **5개**(3 BlockAttack·4 BlockSkill·5 BlockMoveSkill·7 Taunt·10 Animation) | `reused` |
| E3 | `/specs[17]/mem[5]` | 보강(**5차 유실 복구**) | `vtable+0x28 = tick` 의 런타임 확증을 `note` 에 기록 + **`chk` 에는 반영할 채널이 없다**는 사실 명시 | `reused` |
| E4 | `/specs[19]/mem[6]` | 보강 | `vtable+0x40 = get_game_mode` 런타임 확증(`as_moba()` 포인터로 `MobaMode+0x240` 를 써서 `remain_epic_time` 이 추종) | `reused` |
| E5 | `/specs[18]/notes[0]` | 보강 | **반증 실패 = 유지**. 덤으로 `range(i64 2,0)` 의 **출처 확정** — 호출부 `objective_handlers.rs:1119` 가 `version > 1` 일 때만 부른다 | `reused` |
| E6 | `/specs[15]/open[0]` | 보강 | version 축 잔여 2함수를 실측했으나 **산출이 상수 ⟹ 판별력 부재**. 「무영향」이 아니라 **범위를 좁혀** 남긴다 | `inherited` |

**`behavior_change` = 0건.** 여섯 건 모두 표기·근거·범위의 문제이고 재구현 산출을 바꾸지 않는다.

### ev 상향 38행 (내 대역 `ev≥4` **81행 → 43행**, 37.5% → **19.9%**)

| 대상 | 행수 | 근거 |
|---|---|---|
| `mem` → **ev3** (상한) | **25** | ⓐ `offset_of!` 일괄 107/107 MISMATCH 0 (`o1.txt`) + `tcxdict` 오프셋 조회 |
| `consts` → **ev2** | **8** | 실제 열거형 값의 태그 바이트/워드 직독 (`o1.txt`) |
| `knobs` → **ev2** | **5** | pub 헬퍼 진리표 (`o2.txt` 540행 · `o7.txt` ERD 14행) |

함수별: `15` 9행 · `16` 5행 · `18` 22행 · `19` 2행 · `17` 0행(이미 ev≥3 로 1행만 남아 있었다).

**미상향(범위 명시)**
- `15/mem[16],[17]`(반환 `Option<SinglePlanBattle>` 니치) — `Option` 판별자 자체를 안 쟀다.
- `15/consts[2]`(`1 - team` 의 1) · `15/consts[4]`(−1 두 종류) · `18/consts[0],[1]`(`repair_need` 반환 1/2) — `v3_epicops_repair_need` 가 `in:game_ai` 라 직접 못 부른다.
- `15/knobs[0],[1],[3]~[8]` · `16/knobs[5],[7],[8]` · `17/knobs[4]` · `19/knobs[6]` — **이 함수 밖**의 임계이거나(다른 함수 담당) 판별력 있는 세계가 필요하다.
- `18/knobs[6]~[14],[16],[19]~[26]` — `v3_epicops_repair_need`·`is_object_being_taken_by_enemy`(둘 다 `in:game_ai`)와 세르펜 세계가 필요하다. `v3_serpen_contest_clear_win` 은 **60/60 false**(판별력 0, 5차 80/80 과 동일).

---

## ③ `ev<=3` 표본 재확인 — 뒤집힘 **0건**

| 재확인 항목 | 방법 | 결과 |
|---|---|---|
| `mem` 오프셋 43행(이미 ev3 이던 것 포함) | ⓐ `offset_of!` + 구조체 크기 20건 | **107/107 OK, MISMATCH 0** (`o1.txt TOTAL`) |
| 구조체 크기 20건 (`Entity` 1728 · `TeamPlan` 1064 · `SinglePlanBattle` 144 · `DeathMatchBattle` 384 · `LegacyPlanHandler` 6168 · `AbstractGameWithCache` 8840 · `MapDef` 28112 · `EntityType` 480 …) | `size_of` | 20/20 일치 |
| `16/mem[10]` 주석의 크기 교차검증(「Epic/Serpen info 472B ↔ EntityType 480B」) | tcx + `size_of` | **Epic=Serpen=472, EntityType=480** ✓ |
| `15/logic:249` **타워 이터레이터 순서** | `iter_towers_without_nexus`(pub) ↔ 명세 배열을 **포인터 동일성**으로 대조 | **팀당 8/8 · 양팀 16/16 일치**, 순서 `[top, mid, bottom, top2, mid2, bottom2] + twin_towers`, `nexus` 미포함 (`o6.txt`) |
| `18/history[15]` `v3_epic_formation_role` 80칸 진리표 | 450칸 재실행 | **전건 일치**(Gather→전원 Mid/false · Split14{P}→P만 그 라인/true · Split131→p1 Top/true·p2 Bottom/true) |
| `18/history[0]` `MorgardUseStrategy` 니치 태그 | 실값 워드 직독 | **5 / 6+position / position1+position2** 그대로 (`o2.txt MUTAG`) |
| `18/history[0]` 튜토리얼별 후보 라인표 | 9종 전수 | **전건 일치**(JungleOnly·First/Bottom+Split14 에서 `None`) |
| `17` 43행 `mem` 중 pub 필드 25행 | `offset_of!` | 25/25 OK — **private 18행은 ⓐ 로 접근 불가**(§⑤-4) |
| `18/consts[2]` `MainObjective::Repair = 7`, `mem[13]` `Serpen{Setup,true} = 1/1/1` | 실값 바이트 직독 | ✓ |
| `15/consts[0]` `TryKill = 0`, `17/consts[1]` `Chat::Battle = 3` | 실값 태그 직독 | ✓ |
| `callees_unmatched` 에 판정 술어 혼입 | 15~19 육안 | **0건**(3·4·5차와 동일) |

---

## ④ 게이트 실측

```
$ cd /c/tfm2mods/MIG && PYTHONIOENCODING=utf-8 python -X utf8 specgate.py
   G1 자기모순=0  G10 class 오분류=0  G11 ev 지시 미반영=2  G2 호출부 전수=0  G3 형제 함수=0
   G4 술어 시그니처=0  G5 sig 정본 대조=0  G6 logic 미반영=0  G7 과열림=0  G8 표에 옛 값=0  G9 callees 오염=4
```
- **G11=2 는 내 대역이 아니다** — `[03] defensive_crisis`(3행) · `[04] handle_line_defense`(2행), 전부 **배치 A**. 내 5개 함수는 0건. (브리핑 생성 시점 실측도 2였다 = 신규 발생 아님.)
- **G9=4 는 전부 `손확인`** 등급(필드 `chats` 와 메서드 `chats()` 공존 등). 내 대역의 `[15] chats` 1건도 실제 `TeamPlan::chats` 필드 접근이라 정상.
- ⚠**G8=0 인데 실오류 E1 을 못 잡았다** — §⑤-5 참조.

```
$ PYTHONIOENCODING=utf-8 python -X utf8 auditrounds.py
커버리지 결손 11 · 유실 0 · STALE 0 · ev되돌아감 0
```
**불변 게이트 = 유실 0 · STALE 0 · ev되돌아감 0 — 유지.** 「커버리지 결손 11」은 1~5차 배치들이 `patch.json` 을 안 냈다는 과거 사실이고 이번 라운드가 만든 것이 아니다(6차는 A·B·C·D 전부 제출됨 = 「반영 대기」로 잡힌다).

```
$ PYTHONIOENCODING=utf-8 python -X utf8 _spec/audit4.py
미반영 0 · STALE 0 · ev불일치 0   → 전건 반영됨

$ PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 6 --only D --dry
[D] 정정 6/6 · ev상향 38/38      ★적용 실패 0
발견 경위: inherited=1 · reused=5

$ PYTHONIOENCODING=utf-8 python -X utf8 tcxaudit.py --prose              # 기준선
총 687건  오귀속=0  밀림=0  부분일치=1  확인불가=18  OK=668
$ PYTHONIOENCODING=utf-8 python -X utf8 tcxaudit.py --prose _verify6/D/D6_REPORT.md
총 688건  오귀속=0  밀림=0  부분일치=1  확인불가=18  OK=669
⟹ 델타 = 스캔 +1행 · **오귀속 +0 · 밀림 +0 · 부분일치 +0**
  (부분일치 1 은 기준선에도 있는 `specs20.json` 쪽 팻포인터 오탐 — 5차 §6-b 에서 이미 도구 결함으로 보고했다)
```
이 보고서의 오프셋은 **한 행에 하나씩** 적었다(4차 D-E5 교훈).

---

## ⑤ 브리핑·`BRIEF_FACTS` 의 오류

1. ★**`BRIEF_FACTS.md §1` 집계표의 배치 D 행이 낡았다.**
   브리핑 = `ev2 0 · ev3 22 · ev4 194 · ev≥4 89.8%` / 같은 스코프(`mem`+`consts`+`knobs`) 실측 = **`ev2 35 · ev3 100 · ev4 81 · ev≥4 37.5%`**.
   원인 = 파일 생성 `15:49:57` < `_spec/specs20_v3.json` 재생성 `15:53:26`(그 사이 `15:51:52` 에 `specs20.bak.evup.json` = 5차 ev 상향 복원본 생성).
   배치 A 행은 정확, B(`ev2 7→6, ev3 19→20`)·C(`ev2 5→4, ev3 10→11`)는 1행씩 어긋난다. 총계도 `15/59/782` → 실측 `48/139/669`.
   재계산 스크립트 = `_verify6/D/count.py`(스코프 정의는 브리핑 §① 그대로).

2. ★**그래서 §2 의 내 표적 수치가 대부분 틀렸다.**
   `17 new` **47행 → 실측 1행** · `16` 43 → **9** · `15` 31 → **21** · `19` 26 → **3**. 맞은 것은 `18` 47행뿐.
   ⟹ 「`17` 이 47행으로 두 번째 표적」이라는 지시대로 갔으면 **이미 끝난 함수에 라운드를 썼을 것**이다.

3. ★**생성물끼리 모순한다.** `BRIEF.md §2` 산문은 「`ev>=4` 가 아직 **669행**」이라 적었고 이것이 현재 JSON 과 맞는다(`ev4=669`). 반대로 **생성물**인 `BRIEF_FACTS.md` 가 **782** 로 낡았다.
   ⟹ 「사실은 생성하고 판단만 손으로 쓴다」는 원칙은 **생성 시각 ≠ 정본 갱신 시각**이면 역전된다.
   처방 = `mkbrief.py` 가 `specs20_v3.json` 의 **mtime + 해시**를 브리핑 머리에 박고, 배치가 착수 시 `--check` 로 대조하게 할 것(지금 `--check` 는 총계만 찍고 파일이 낡았는지는 말하지 않는다 — 실제로 `mkbrief.py 6 --check` 는 `행 858 · open 11 · notes 8` 로 **통과했다**).

4. **`TEMPLATE.rs` ⓐ(`offset_of!` 일괄 대조)에 적용 범위가 빠져 있다 — private 필드에는 못 쓴다**(rustc `E0616`).
   실측: `game_ai::plan_legacy::old::DeathMatchBattle` 36필드 중 **18개**(`region` `well_runaway` `death_focus` `seal_basis` `hold_scene_basis` `repo_scene_basis` `flee_dir` `far_noout_since` `flee_die` `lean_last_tick` `scene_change_tick` `ep_follow_until` `idle_prev_pos` `last_unseal_tick` `dodge_commit` `scene_last_from` `main_objective` `lean_last_sign`)와 `TeamPlan::v3_press_chat_line` 이 막힌다.
   ⓐ 의 설명이 `game_core` 기준(`Entity` 41필드 전부 pub)이라 그대로 `game_ai` 에 적용하면 **컴파일이 통째로 죽는다**(내 첫 빌드가 정확히 그랬다).
   **우회 = ⓑ(`derive(Debug)`) 또는 `tcx adt` 오프셋 + raw 기록/판독.** 이 문장을 ⓐ 항목에 넣어 주기 바란다.

5. **`G8`(표에 옛 값)이 E1 을 못 잡는다.** E1 은 `logic` 에 `~~/*p5=*/true~~ 는 오독` 이라고 **취소선으로** 정정이 적혀 있고 `knobs[1]` 에는 그 `true` 가 **맨몸으로** 남아 있는 형태다. G8 이 「취소선 안의 값이 표에 살아 있나」를 본다면 이 패턴을 잡을 수 있다(현재는 못 잡는다). **4차 D-E3(`ty.Tower.0`)와 정확히 같은 모양**이 세 라운드째 반복된다.

6. **`chk` 는 `tcxaudit` 파생이라 런타임 해소를 반영할 채널이 없다.** 5차 배치D 가 제안한 `chk` 갱신 2건(`/specs[17]/mem[5]`·`/specs[19]/mem[6]` = 확인불가→OK)이 **2라운드 연속 미반영**인 구조적 원인이다. `patch.json` 에 `chk_override` 를 두거나 `tcxaudit` 에 수동 확정 목록이 필요하다. 이번엔 `note` 에 적어 우회했다(E3·E4).

7. **5차 배치D 의 산문 전용 항목 3건이 유실됐다.** 5차 `patch.json` 의 `errors` 가 **빈 배열**이었고(=`ev_up` 128행만 냈다), 실오류 3건은 사람이 `patch5.py` 로 옮겨 살아남았지만 **보강 1건 + `chk` 갱신 2건은 사라졌다**. ⟹ 계약이 「`ev_up` 만 구조화」로 읽히면 안 된다. **보강·`chk` 도 구조화 대상**이다.

---

## ⑥ ★`found_by` 집계 — 이번 라운드의 결론

`patch.json` 의 `errors` 기준(`applypatch` 가 세는 값):

| `found_by` | 건수 | 내역 |
|---|---|---|
| `reused` | **5** | E1(교차필드 모순 읽기) · E2·E3(5차 산문 유실분 재제출) · E4(포인터 주입) · E5(IR 인자 전수 grep) |
| `inherited` | **1** | E6 — ⓒ(구조체 통째 바이트 diff)로 「version 무영향」과 「판별력 부재」를 갈랐다 |
| `new` | **0** | — |

**실오류만 따지면 1건이고, 그것이 `reused` 다.** 다만 그 1건의 성격이 결론을 가른다:

> **E1 은 「명세가 틀렸다」가 아니라 「2차에 맞게 고친 것이 표 한 칸에 안 옮겨졌다」이다.**
> E2·E3 도 마찬가지로 **5차가 이미 옳게 찾아 놓고 반영 채널이 없어 사라진 것**이다.

⟹ **유저 질문(「조건이 완전히 같을 때도 오류가 생기는가」)에 대한 내 대역의 답:**

- **새로운 사실 오류는 0건이었다.** 오프셋 107/107, 진리표 560칸, `ev<=3` 표본 재확인 뒤집힘 0 — **같은 조건에서 명세 자체는 흔들리지 않는다.** 이 점에서 메인 세션의 주장은 **이 배치 범위에서 지지된다.**
- **그러나 발견된 3건 전부가 「전파 실패」다.** 정정이 `logic` 에만 · 보강이 산문에만 · `chk` 가 파생이라 못 씀.
  ⟹ **라운드를 더 도는 것이 답이 아니라는 결론은 같지만, 이유가 다르다.** 명세가 불안정해서가 아니라 **정정의 전파 경로에 구멍이 있어서** 라운드마다 같은 종류의 「오류」가 계속 나온다.
  실제로 `~~취소선~~ → 표 미반영` 은 **4차(D-E3) · 5차(`flee_die` usize::MAX) · 6차(E1)** 세 라운드 연속 같은 패턴이다.
  **처방은 새 라운드가 아니라 G8 의 감도 개선 + `patch.json` 의 대상 확장(보강·`chk`)이다.**
- `inherited` 1건은 예측대로 **계측기 확산**이었고 명세 문제가 아니었다. ⓒ 가 없었으면 나는 5차와 똑같이 「도달 불가」로 닫았을 것이다.

---

## ⑦ 새 발견 13건

| # | 내용 | 근거 | `found_by` |
|---|---|---|---|
| N1 | ★**`18` 호출부 가드 완전 해독** — `objective_handlers.rs:1116~1128`: ①`game.as_moba()` Some ②`remain_epic_time(team) > tps*10` ③**`version > 1` 이면 `v3_epicops_buff_window`, `version <= 1` 이면 `handle_press_epic`**(이쪽만 `my_alive >= enemy_alive` 가 추가로 걸린다) ④`remain <= tps*10` 이고 `version>1` 이면 `self.v3_press_chat_line = None` 만 하고 끝 | `_gaibc/m09.ll` 14780~14834 + `dloc.py` | reused |
| N2 | ★그래서 `18` 의 `sig.params[2]` `usize(range 2..)` 의 **출처가 확정**됐다 — 함수의 가정이 아니라 **호출부 게이트 `version > 1` 의 그림자**다 | 같음 | reused |
| N3 | ★**`handle_none_or_gank_objective` 는 도달한다.** `TeamPlan::update_objective`(pub)에 `objective = Repair/Defense` 를 주면 `+0x210 mf_obj_clear` 와 `+0x41f objective→255` 가 바뀌고, `None`/`Gank` 를 주면 그 핸들러로 갈린다 ⟹ 5차의 「도달 불가」는 **디스패처가 아니라 그 안쪽**의 문제였다 | `o4_h0.txt` 48행 | inherited(ⓒ) |
| N4 | ★**막는 값의 정체 = `MobaMode+0x240 + 8*team`(=`remain_epic_time`)이 tick 0 에서 `0`** 이다. `mb.remain_epic_time(t)` 가 그 워드를 그대로 돌려준다(0 → raw 로 100000 을 써 넣으면 100000). 다만 이것만 풀어도 아직 안 닿는다 — 남은 분기는 `m09.ll:14206(%1867)` ~ `14225(%1873)` 구간 | `o5.txt GUARD/RAW/FORCED` | inherited(ⓒ)+reused |
| N5 | ★**5차가 세 필드만 봐서 놓칠 뻔한 마커** — `18` 의 압박 경로는 발화 여부와 무관하게 `v3_press_chat_line`(+0x41e)을 **항상** 갱신하고, 호출부 1128행도 그 자리에 `None` 을 쓴다. 5차는 `objective`/`chats`/`eo_serpen_punish_issues` 만 읽었다 ⟹ **통째 바이트 diff 가 아니면 이 축이 안 보인다** | `o3.txt`/`o4_h0.txt` | inherited(ⓒ) |
| N6 | `v3_epic_group_line`(pub) **튜토리얼 9종 × morgard_use 5종 × 팀 2 전수** — `{None,Line,Total}→Mid` / `TopSolo→Top` / `MidSolo·MidBottom→Mid` / `{First,Bottom}→Bottom`(단 `Split14` 는 **None**) / `JungleOnly→전부 None`. 「`group_line==None → false`」 경로가 **두 가지 방식으로** 성립함을 실행 확인 | `o2.txt GL` | reused |
| N7 | `v3_epic_formation_role`(pub) 450칸 — `history[15]` 표와 **전건 일치**. 부수: **발표자는 슬롯 0 이 아닐 수 있다** — `Split14(Top)`·`Split131(Top,*)` 에서는 슬롯 0 이 분열 담당이라 **슬롯 1(Jungle)** 이 발표자가 된다 | `o2.txt GL/ANN` | reused |
| N8 | `MorgardUseStrategy` 니치 태그를 **실값으로 직독** — `Gather` low32=5(high32 는 미초기화 쓰레기) / `Split14` low32=6·high32=Position / `Split131` low32=position1·high32=position2 | `o2.txt MUTAG` | reused |
| N9 | `iter_towers_without_nexus`(pub) **순서 포인터 동일성 16/16** — `[top, mid, bottom, top2, mid2, bottom2] + twin_towers[team]`, `nexus` 미포함, 팀당 8개. `15/logic:249` 의 순서 주장이 실행으로 확정 | `o6.txt ITER` | reused |
| N10 | `game_ai::engage_requires_dive`(pub) 진리표 14표적 — **적 타워 3/3 true**, 적·아군 챔피언 10/10 false, **적 넥서스 false**(타워가 아니다) | `o7.txt ERD` | reused |
| N11 | ⚠**프로브 함정** — `single_tower_dive_is_viable` 에 **타워/넥서스를 표적으로 주면 `fight_check.rs:979` 에서 `Option::unwrap` 패닉**한다. 챔피언 표적만 안전 | `o7.txt`(1차 실행 크래시) | reused |
| N12 | ⚠**ⓐ 의 한계** — `offset_of!` 는 **private 필드에 막힌다**(E0616). `DeathMatchBattle` 18필드 · `TeamPlan::v3_press_chat_line`. `game_core` 는 거의 전부 pub 이라 안 보이던 한계다 | 빌드 로그 | inherited(ⓐ) |
| N13 | `EntityType` **480B** = 태그 8B + `Epic`/`Serpen` info **472B** — `16/mem[10]` 주석의 크기 교차검증이 실측으로 성립 | `o1.txt`/`o6.txt` + tcx | reused |

---

## ⑧ 표적 `18` — 남은 미탐색을 어디까지 좁혔나

5차 판정: 「**이 세계 구성으로는 도달 불가** · 미탐색 = 에픽/세르펜 스폰 + 부상 상태 + `group_line` 이 성립하는 세계」

6차 판정(**범위 한정**):

- ✅ **`group_line` 은 세계 조건이 아니다** — 기본 세계에서 이미 `Some(Mid)` 가 나온다(`o2.txt`). 5차가 넣은 이 항목은 **불필요했다.**
- ✅ **부상 상태도 조건이 아니다** — 부상 주입은 `v3_epicops_repair_need` 용이고, 압박채팅 경로는 부상과 무관하다. (덧: `SwordmanChampionInfo::default()` 는 **`stat_cached.hp = 0`** 이라 `hp*100/max` 형 판정에 판별력이 없다 — TEMPLATE 함정 ④의 새 사례.)
- ✅ **디스패처까지는 도달한다**(N3).
- ❌ **진짜 벽은 하나** — `MobaMode+0x240+8*team`(에픽 잔여시간)이 `0` 이고, 그것을 raw 로 풀어도 `m09.ll:14206~14225` 사이의 분기가 아직 남는다(N4).
- ⟹ **남은 미탐색 = 「에픽이 실제로 스폰 대기 중인 세계」 하나**로 좁혀졌다. 도달 방법 후보는 ①`run_tick` 을 돌려 에픽 리스폰 타이머를 정상 초기화(4차 배치B 의 미니언 600틱 수법과 동형) ②`m09.ll:14206(%1867)` 의 switch 입력(팀 전략 태그로 보인다)을 `Game::set_strategy`(pub)로 지정.
- ⛔**「불가」가 아니다.** 위 두 방법은 이번 라운드에서 **시도하지 않았다**(범위 명시).

---

## ⑨ 재현 절차

```bash
cd /c/tfm2mods/MIG
sh _verify3/build.sh _verify6/D/D6_o1.rs && "$TEMP/tfm2_spanprobe/D6_o1.exe"        # 오프셋 107/107
sh _verify3/build.sh _verify6/D/D6_o2.rs && "$TEMP/tfm2_spanprobe/D6_o2.exe"        # 18 pub 헬퍼 진리표
sh _verify3/build.sh _verify6/D/D6_o5.rs && "$TEMP/tfm2_spanprobe/D6_o5.exe" 100000 # 18 가드 계측
sh _verify3/build.sh _verify6/D/D6_o6.rs && "$TEMP/tfm2_spanprobe/D6_o6.exe"        # 타워 순서·update version
sh _verify3/build.sh _verify6/D/D6_o7.rs && "$TEMP/tfm2_spanprobe/D6_o7.exe"        # ERD·STDV
PYTHONIOENCODING=utf-8 python -X utf8 _verify6/D/count.py                            # ev 재집계(브리핑 대조)
PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 6 --only D --dry                 # 적용 실패 0 확인
```
