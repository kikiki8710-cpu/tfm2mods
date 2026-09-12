<!-- ★이 파일은 `mkbrief.py` 가 생성한다. 손으로 고치지 마라 — 다음 생성에 날아가고, 손으로 쓴 수치가 5차까지 반복된 브리핑 오류의 원인이었다. -->
# 7차 반증검증 — **생성된 사실 절** (게임 0.5.8)

> **정본 스탬프** — `specs20_v3.json` sha256[:16] = `9dc10ee1f43de452` · mtime `2026-09-11 16:53:10` · 이 파일 생성 `2026-09-11 16:56:43`
> ★**읽기 전에 스스로 확인하라.** 아래 명령의 출력이 위 해시와 다르면 **이 표는 낡았다** — `python -X utf8 mkbrief.py <N>` 을 다시 돌려라.
> ```bash
> cd /c/tfm2mods/MIG && python -c "import hashlib,io;print(hashlib.sha256(io.open('_spec/specs20_v3.json','rb').read()).hexdigest()[:16])"
> ```
> 수치는 전부 `_spec/specs20_v3.json` 에서 **센 값**이고, 심볼 경로는 `_tcx` 에 **실재 확인**을 통과한 것만 적혀 있다. 브리핑의 판단·지침 절은 이 파일 **뒤에** 붙는다.

## ① 집계 스코프 — **이 정의로 센 값이다**

이 문서의 「행」 = **`mem` + `consts` + `knobs`** 세 배열의 원소만이다. `sig.params` · `callees` · `open`/`notes` 는 **포함하지 않는다.**
⚠5차에 이 정의를 안 적어서 한 배치가 다른 분모(296행)로 세고 「브리핑이 틀렸다」고 보고했다. 네 분모가 다르면 **그건 불일치가 아니라 스코프 차이다** — 먼저 이 정의로 다시 세라.

| 배치 | 함수 | ev1 | ev2 실행 | ev3 tcx | ev4 IR | ev5 추론 | 계 | **ev≥4** |
|---|---|---|---|---|---|---|---|---|
| **A** | 00~04 | 0 | 60 | 104 | 32 | 0 | 196 | **16.3%** |
| **B** | 05~09 | 0 | 49 | 116 | 27 | 0 | 192 | **14.1%** |
| **C** | 10~14 | 0 | 115 | 112 | 27 | 0 | 254 | **10.6%** |
| **D** | 15~19 | 0 | 48 | 125 | 43 | 0 | 216 | **19.9%** |
| 계 | 00~19 | 0 | 272 | 457 | 129 | 0 | 858 | **15.0%** |

## ② 표적 — `ev≥4`(미실행) 가 많은 함수

`sig.tcx` 가 `pub` 인지 함께 적었다(5차 배치D 요청). **`pub` 가 아니면 오라클 진입 자체가 막힐 수 있다** — 5차에 `18` 이 47행을 그대로 남겼다.

- **배치 A**: `03 defensive_crisis` 15행(pub) · `00 ult` 9행(pub) · `01 calculate_jungle_action_sc` 4행(pub) · `02 sub_plan` 2행(pub) · `04 handle_line_defense` 2행(pub)
- **배치 B**: `05 v50_fold_dive_episode` 7행(in:game_ai) · `07 sub_plan` 6행(pub) · `06 v2_response_retreat_stance` 5행(in:game_ai) · `09 check_favorable_engage_for` 5행(pub) · `08 is_end` 4행(pub)
- **배치 C**: `12 handle_chat` 13행(pub) · `11 v3_fall_back_to_passive` 11행(pub) · `13 target_bush_v30` 2행(in:game_ai::plan_legacy::old::line_gank::cover) · `14 update` 1행(pub)
- **배치 D**: `18 v3_epicops_buff_window` 25행(in:game_ai) · `15 single_try_engage` 12행(in:game_ai) · `16 max_range_nearly_can_use` 4행(pub) · `17 new` 1행(pub) · `19 best_jungle_goal` 1행(pub)

## ③ `open` 8건 · `notes` 8건

`open` = 아직 답이 없는 것. `notes` = **확정된 사실 서술이라 물음이 아니다** — 파지 말고, 틀렸다고 보면 **반증**하라.

| # | 분류 | ev | 물음 |
|---|---|---|---|
| 02 | 미탐색 | 4 | `version`(p2) 이 무엇을 게이트하는 값인지 — 이 함수 본문에서 단 한 번도 참조되지 않아 여기서는 확정 불가. AI 버전 분기가 없다는 사실만 확실하다 |
| 03 | 미탐색 | 3 | fnparts 기준 DWARF 서브프로그램 23개 중 별도 define 은 3개뿐 — 나머지 20개(iter_champions, distance_sq, abs_diff, is_some, is_som |
| 04 | 미탐색 | 4 | 이 함수 자체는 구조체 필드에 **아무것도 쓰지 않는다**(store 대상은 alloca %7/%10 뿐) — 그래서 writes 가 빈 배열이다. 다만 `rnd`(&mut StdRng)는 read |
| 05 | 미탐색 | 4 | RawVec::grow_one 내부(재할당 정책)는 안 봄 |
| 07 | 표기 불가 | 4 | L36 세 OR 항의 소스상 원래 순서 — LLVM 이 %94|%85, %96&%83 으로 재결합해 한 블록에 몰아넣었고 !dbg 가 전부 line 36 이라 소스 순서를 줄번호로 복원할 수 없다. |
| 09 | 미탐색 | 3 | src_line 은 DWARF !DILocation 에서 복원한 값이고 game-ai\src\fight_check.rs 원본 `.rs` 파일 자체는 이 환경에 없다. ⚠단 이 항목은 **미탐색**이 |
| 15 | 미탐색 | 4 | version(p2) 이 이 함수 안에서 분기를 만드는지 — 본문에는 version 비교가 하나도 없다. 전달만 하므로 하위 함수(new/new_dive/update/single_tower_dive |
| 18 | 미탐색 | 4 | version(2번 인자)은 v3_serpen_contest_clear_win 에만 전달된다. IR range(i64 2,0) 이 '2 이상'을 뜻하는 것 외에 이 함수 안에서의 버전 분기는 없다. |

<details><summary>`notes` 8건 (파지 말 것)</summary>

- `02` has_enemy_twin_tower 의 dbg 표현이 이상하다: `#dbg_value(i1 %60, !62934, DIExpression(DW_OP_not, DW_OP_not, ...))` 로 NOT 이 두 번 걸려 있어 문자대로면
- `04` `_version`(p1)·`_debug`(p6)는 본문에서 전혀 쓰이지 않는다(p6 는 readnone). 버전 게이트가 상수접힘으로 사라진 게 아니라 애초에 참조가 없다.
- `09` 1280행 is_front 의 리터럴 4 와 1209행의 ×2 는 각각 shl 로 접혀 그 자리엔 리터럴이 없다. constants 에 적은 4/2 는 본문 다른 위치(루프 상한 4, team bounds-check 2)의 같은 리터
- `10` phase 리터럴 3 이 본문에 없다 — 768(=3<<8) 로 상수접힘. constants 에는 실제 본문 값 768 만 올렸다.
- `10` 적 챔피언 슬롯 5칸은 IR 에서 완전 언롤돼 루프 변수가 없다. 5 라는 상수는 배열 길이라 constants 에 넣지 않았다(SPEC_GUIDE §3 표 규칙).
- `13` Top 의 L151(2차타워)과 L156(적 미인지)이 3/6 으로 완전히 같고, Bottom 의 L181/L186 도 15/20 으로 같다. 두 분기를 따로 쓴 이유는 **중복 서술이 아니라 중첩 구조** 때문이다(6차 배치C, 줄
- `13` 함수 이름의 `v30` 은 **AI 내부 버전 번호 30** 을 가리키는 도입 시점 표식이고, 런타임 버전 분기와는 무관하다. 근거 ①`_tcx/game_ai.json` 의 아이템 중 `vNN_` 접두가 v2·v3·v15~v17·v2
- `14` target_bush_v30 은 별도 define 이 없다(update 안에 전량 인라인). fnparts target_bush_v30 이 내놓는 m10.ll 11483~11731 은 LineGankCoverPlan 쪽 동명 함수라 
</details>

## ④ 게이트 — 숫자를 외우지 말고 **직접 재라**

```bash
cd /c/tfm2mods/MIG
PYTHONIOENCODING=utf-8 python -X utf8 specgate.py        # G1~G8·G10·G11 = 0
PYTHONIOENCODING=utf-8 python -X utf8 _spec/audit4.py    # 과거 라운드 회귀
PYTHONIOENCODING=utf-8 python -X utf8 tcxaudit.py --prose <내보고서.md>
```
**불변 게이트는 총 건수가 아니라 `오귀속 0 · 밀림 0` 이다.** 총 건수는 명세가 자라면 변한다.

생성 시점 실측(참고용 — 네가 다시 재라):
```
G1 자기모순=0  G10 class 오분류=0  G11 ev 지시 미반영=0  G12 src_line 대조=47  G13 knobs.where 대조=8  G2 호출부 전수=0  G3 형제 함수=0  G4 술어 시그니처=0  G5 sig 정본 대조=0  G6 logic 미반영=0  G7 과열림=0  G8 표에 옛 값=0  G9 callees 오염=4
```

