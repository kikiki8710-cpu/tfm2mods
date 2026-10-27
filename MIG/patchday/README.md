# MIG\patchday — 패치날 런북 (기준 0.6.5 · 2026-10-28 업데이트 대비)

> **업데이트가 오면: 게임을 켜지 말고 → `run_patchday.py --skip addrbook`(①-a) → `SUMMARY.md` 를 읽고 → 아래 ③~⑦ 순서 (주소록 ①-b 는 그동안 따로 돌린다).**
> ⚠①-b 가 도는 동안 ③~⑤ 에서 targets·src·매니페스트를 고치면 실행기가 그것을 「비파괴 경고」(종료코드 1)로 잡는다 — 본인 수정이면 무시해도 된다(경고는 실행기가 쓴 게 아니라 "누가 바꿨나 보라"는 뜻).
> 실행기는 **비파괴**다(계산·보고만 한다). 매니페스트·모드 소스·targets·기준선을 바꾸는 일은 전부 ③~⑦ 에서 사람이 한다.
> 표준 절차 정본 = `MIG\README.md`(축 3개·run.py). 이 문서는 그 앞에 붙는 "패치 첫 1시간" 순서다.

## 구성
| 파일 | 하는 일 | 만든 곳 |
|---|---|---|
| `run_patchday.py` | ★패치날 실행기 — 0 detect … 10 summary (아래 표) | 과업 E |
| `addrbook.py` / `addrbook_0.6.5.json` | 전 주소록(매니페스트 + KB 수확) 구→신 일괄 대응 (`map` 하위명령 · `grade` = 정답지 채점) — 검증 기록 `rehearsal_addrbook.md` | 다른 과업 |
| `kb_harvest.py` / `kb_rows_0.6.5.json` | KB 문서(MEM·ANA·REPORT)의 버전 귀속 RVA 수확 → 주소록 `build --extra` 입력 (⑧-2) | 다른 과업 |
| `hotfix_exposure_0.6.5.json` | 핫픽스(버전 문자열 그대로·sha 만 다름) 때 낡은 주소로 패치하는 모드 노출 표 | 다른 과업 |
| `..\..\tfm2_detail_tactics\tools\track_update.py` + `tools\patchday\baseline_0.6.5.json` | 상세전술 100전술·218창 추적(이동만/기본값 변경/레지스터·스택 재배치/사본 분기/로직 변경/소실) — 채점 기록 `tools\patchday\rehearsal_0.6.5_to_0.6.4.md` | 다른 과업 |
| `crosscheck.py` | SUMMARY.md 수치 33항목을 각 도구 산출물에서 독립 재계산해 대조(인자 = 출력 폴더 · 0=일치) | 검증 C |
| 출력 = `MIG\_patchday\<라벨>\` (리허설 = `_rehearsal_<라벨>\`) | `SUMMARY.md` · `run.json` · `logs\NN_<단계>.log` · 단계별 산출물 | — |

(도구가 아직 없으면 그 단계만 「도구 없음」으로 표시되고 나머지는 돈다.)

---

## ⓪ 스팀 업데이트가 왔다 — **게임을 먼저 켜지 말 것**
- 이유: 모드 dll 의 `mod.mod_info` deps 는 `>=0.6.5, <0.6.6`(근거 `REPORT\_0.6.5_마이그.md` §3). **새 버전(0.6.6)** 이면 게임이
  모드를 자동 비활성해 위험이 작지만, **핫픽스**(base_version 문자열이 0.6.5 그대로, exe 만 바뀜)면 deps 를 통과해
  **0.6.5 주소로 바이트 패치·훅을 거는 모드가 그대로 로드된다** → 엉뚱한 명령 한복판 패치 = 크래시·세이브 오염 위험.
  어느 모드가 그런지는 `hotfix_exposure_0.6.5.json`(사람용 `.md` · SUMMARY 8절)이 말한다 — 2026-10-27 판 결론: 핫픽스면
  **comptest_unlock·elemental_serpen 은 끄고(필수)** banpick_view_plus 는 권장, 나머지는 안전하게 꺼지거나 기능만 죽음(그 파일 conclusion).
  상세전술(`tfm2_detail_tactics`)은 설치 시 exe 전체 sha256 을 `sites::EXE_SHA256` 와 대조해 다르면 **한 곳도 패치하지 않고 전 전술 「원본 유지」**(`src\micro.rs:218`) — 그래도 다른 모드 때문에 켜기 전에 노출 표를 볼 것.
- 게임 실행은 유저가 직접(CLAUDE.md §6 — computer-use 구동은 유저 명시 요청 때만). 지금 단계에선 **exe 만 있으면 된다.**
- ⚠메인 PC 의 LDPlayer 는 켜지 말 것(전역 규칙 — 서브 PC 카톡 세션이 끊김).

## ① 실행기 (권장: 두 번에 나눠서)
```
cd F:\claude\tfm2mods
python -X utf8 MIG\patchday\run_patchday.py --skip addrbook      # ①-a 빠른 판 — 백업·지문·재핀·상세전술까지(캐시 있을 때 5~10분, 신 exe 첫 회는 + fnindex·callgraph·aidiff 수 분)
python -X utf8 MIG\patchday\run_patchday.py --only addrbook      # ①-b 주소록(≈28분 · 메모리 약 9GB) — 앞 판 결과는 유지하고 SUMMARY 를 다시 만든다
```
- 한 번에 다 돌리려면 `python -X utf8 MIG\patchday\run_patchday.py` (2026-10-27 전 단계 리허설 실측 1940s ≈ 32분, 그중 addrbook 1673s).
- 오래 걸리는 단계라도 콘솔에 30초마다 `… <단계> 진행 중 (Ns 경과)` 가 찍힌다. 터미널을 닫지 말 것(백그라운드로 돌리면 끝 알림을 놓친다).
- 기본 = 설치본 exe(`mig_verify.GAME_EXE`) · 라벨 = `<게임폴더>\mod-sdk-stable\base_version.txt`(BOM 제거).
  **핫픽스면 라벨이 자동으로 `0.6.5_<sha앞8>`** (버전 문자열이 기준과 같거나, `F:\claude\tfm2\tfm2_<버전>\` 에 sha 가 다른 exe 가 이미 있을 때).
- 아직 업데이트가 아니면: `아직 업데이트 안 됨` · 종료코드 0 · 출력 폴더도 안 만든다(아무 때나 돌려 봐도 된다).
- 종료코드: **0** = 전 단계 정상 종료(검토 항목이 있어도 0 — 내용은 SUMMARY) · **1** = 단계 오류 또는 비파괴 경고 · **2** = 실행기 오류(인자·기준 exe 없음·라벨 불명).

| # | 단계 | 무엇을 | 산출물(출력 폴더 기준) | 예상 소요(근거) |
|---|---|---|---|---|
| 0 | detect | 신 exe sha256 vs 기준 `DAF040DB…` · 기준 exe sha 재확인 · 라벨 결정 | `logs\00_detect.log` | 1s |
| 1 | backup | `F:\claude\tfm2\tfm2_<라벨>\` 에 exe·bundle.game_data **파일 단위 복사**(`.part` → sha 대조 → 이름바꿈). 있으면 sha 비교 후 건너뜀, 다르면 **덮어쓰지 않고 오류**. 성공하면 이후 단계는 **백업본을 신 exe 로** 쓴다(스팀이 설치본을 또 바꿔도 안전) | `tfm2_<라벨>\` | ~1.2GB 복사 |
| 2 | fnindex | `fnindex.py` → `F:\claude\tfm2mods\_fnidx_<tok>.pkl`(tok = 라벨에서 점 뺀 것, 0.6.6→`066`). 있으면 **함수 경계 전수 + skel 표본 48 재계산**으로 그 exe 의 것인지 검증 후 재사용, 불일치면 기존 파일은 두고 출력 폴더에 `.rebuilt.pkl` | pkl | 41s(0.6.5 실측) |
| 3 | callgraph | `MIG\callgraph.py` → `_cg_<tok>.pkl` (있으면 callee 표본 40 재계산 검증) | pkl | 25s(0.6.5 실측) |
| 4 | aidiff | `MIG\aidiff.py 기준exe 신exe -o aidiff.md` — AI 판단함수 EDITED/SHIFTED/IDENTICAL | `aidiff.md` | 신 exe 첫 회 수 분(캐시 `MIG\_aidiff_<sha>_*.pkl`) |
| 5 | check | `MIG\mig_verify.py check <MODS 전부> --exe 신` (`tfm2_detail_tactics` 는 2026-10-27 MODS 편입 — 빠져 있으면 실행기가 명시 추가) | `logs\05_check.log` | 1s |
| 6 | repin | `MIG\repin.py plan` → `repin_map.json` → `MIG\sitealign.py` → `repin_map.aligned.json` → **함수시작 교차검증**(콜러 수·callee 사영·size 비 2/3 + 동형 클론) → `repin_fncheck.json` · 사람 판정 대기열 `repin_review.md`(★apply 대상 / 동형 클론 / (참고) 생성물 모드 3구역). ★apply/--write 는 부르지 않는다 | 5파일 | 합 117~296s(리허설 실측 · 동시 부하에 따라) |
| 7 | offsets | `offsets.py` 의 지문 함수를 import 해 **기준선 지문 ↔ 재핀 후보값의 신 exe 지문** 대조(아래 「왜 offsets.py check 를 안 부르나」) · 후보가 WEAK/CLONE 이면 행에 ⚠ · 기준선 키(구 RVA)를 매니페스트 **값으로 먼저** 잇는다(이름 중복 대비) | `offsets_shifts.json` | 1-8s |
| 8 | detail_tactics | `track_update.py track --base tools\patchday\baseline_0.6.5.json …` (0/1 정상 · 2 오류) | `detail_tactics\`(track.md·track.json·review_queue.json·draft_targets\) | 93~236s(리허설 실측 · 동시 부하에 따라) |
| 9 | addrbook | `addrbook.py map --book addrbook_0.6.5.json …` (0/1 정상 · 2 오류) | `addrmap\`(addrmap.json·addrmap.md·review.json) | **1616~1673s ≈ 27~28분 · 메모리 약 9GB**(2026-10-27 리허설 실측 2회 · 24,120행) |
| 10 | summary | 한국어 대시보드 + 기계용 `run.json` | `SUMMARY.md` | 즉시 |

- 각 단계는 실패해도 다음 단계로 간다(요약에 ★오류로 표시). 단계 하나만 다시: `--only detail_tactics` 처럼 —
  같은 sha 의 이전 `run.json` 이 있으면 **안 돌린 단계는 이전 결과를 유지**하고 SUMMARY 를 다시 만든다(`--only summary` = 요약만 재생성).
- `--skip a,b` / `--only a,b` 는 이름 또는 번호. detect 는 항상 돌고, summary 는 `--skip summary` 가 아니면 항상 돈다.
- 진행 표시: 하위 명령의 진행 줄(`...`·`[saved]`·`->`)은 바로 보이고, 30초 넘게 조용하면 `… 진행 중 (Ns 경과)`. 전문은 `logs\`.

### 비파괴 보장 (어떻게 지키나)
- 부르는 명령 = `fnindex`·`callgraph`(pkl) · `aidiff`(md + MIG 캐시 pkl) · `mig_verify check`(읽기 전용) · `repin plan`·`sitealign --write <출력 폴더>` · 오프셋 워커(출력 폴더) · track_update/addrbook(계약상 `--out` 만). **`repin apply`·`mig_verify rebase/gen`·`offsets.py snap/check`·`gen_sites`·`migrate_targets` 는 부르지 않는다.**
- 시작·끝에 `git -C F:\claude\tfm2mods status --porcelain` + 보호 파일(매니페스트·`MIG\offsets\*.json`·`repin_map_*.json`·`mig_verify.py`·`MIG\patchday\*.json`·상세전술/tactics_verify 의 src·targets·기준선·MODS 모드 src·mod.mod_info) 크기/시각 스냅샷.
  출력 폴더(`MIG\_patchday\`)·`*.pkl`·`__pycache__` 밖의 변화가 있으면 **SUMMARY 맨 위 경고 + 종료코드 1**. 같은 저장소에서 다른 세션이 일하고 있으면 그 변화도 잡힌다 — 경고는 "누가 바꿨나 확인하라"는 뜻.

### 왜 `offsets.py check` 를 직접 안 부르나 (코드 확인 2026-10-27)
- `cmd_snap`(L77-91)은 **기준선 `MIG\offsets\<MOD>.json` 을 덮어쓴다** → 실행기에선 절대 안 부른다(⑤-4 오프셋 작업 끝에 사람이 1회).
- `cmd_check`(L107-152)도 이동이 있으면 **git 추적 파일 `MIG\offsets\_shifts.json` 을 덮어쓴다**(L145-148).
- `cmd_check` 는 **매니페스트의 현재 value** 로 신 exe 를 지문한다(L118-128) — apply 전이면 그 value 는 구버전 RVA 라 **엉뚱한 함수**를 본다.
- ⟹ 실행기는 `offsets.fingerprint`·`shift_pairs` 를 import 해서 **재핀 후보값**(`new_aligned` > `new`, check PASS 는 그대로)으로 대조한다 = apply 전 "예보".
  정식 ④ `offsets.py check`/`sources` 는 apply 뒤 ⑤ 에서 표준대로 돈다.

## ② SUMMARY.md 읽는 순서
`MIG\_patchday\<라벨>\SUMMARY.md`
1. **맨 위 비파괴 경고**(있으면 먼저) → 0절 버전/sha(핫픽스 라벨인지) → 1절 단계 표(★오류·도구 없음 확인)
2. **8절 핫픽스 노출 표** → 게임을 켜도 되는지 판단
3. **6절 상세전술** — 판정별 전술 수 · 「기본값이 바뀐 전술」 · 「검토 필요 전술」 상위
4. 2절 AI 판단계층 EDITED(소스가 바뀐 함수 = 재현코드·judge 계층 영향) → 3절 STALE 모드별 수
5. 4절 재핀 — 「★repin apply 대상 사람 판정」(= 엔진 실패 + sitealign AGREE 아님 + 함수시작 WEAK, 생성물 모드 제외 · 전부 펼쳐 보임) · 「동형 클론 주의」 → 전 목록 `repin_review.md`.
   `tfm2_detail_tactics`·`tfm2_tactics_verify` 행은 **(참고)** 다 — sites.rs 생성물이라 repin 판정을 하지 않고 ③④ 도구가 따로 판정한다(2026-10-27 리허설: 사람 판정 132 중 116 이 이 두 모드, apply 대상은 16).
6. 5절 구조체 오프셋 예보(Δ 균일 이동 = 구조체가 커짐 · Δ 없이 「사라짐/생김」만 있으면 짝 없는 변화) → 7절 주소록(자동 반영 가능 수 · 종류×등급 표 · MOVED_BYTES_CHANGED 의 바뀐 방식)

## ③ 상세전술 (`tfm2_detail_tactics` — 전술 100 · 패치창 218)
1. 읽기: `MIG\_patchday\<라벨>\detail_tactics\track.md`(요약 · ★게임이 기본값을 바꾼 전술 · 검토 필요 전술 · 전술 전량 구→신 RVA) +
   `review_queue.json`(REVIEW 전술의 창별 근거·제안). 창 판정 어휘(track_update.py docstring 정본):
   | 판정 | 뜻 | 사람이 할 일 |
   |---|---|---|
   | `SAME` | 이동만(창 바이트·프레임·Src·Caller 동일) — 유일한 자동확정 | 없음 |
   | `REG_RENAME` | 꼴·즉치 같고 레지스터/스택 슬롯/변위/콜사이트만 다름 | 제안된 새 Src 확인(정렬 다수결 — 런타임 확인 권장) |
   | `VALUE_CHANGED` | ★게임이 기본값을 바꿈(즉치 구→신, tf 역산) — 절대 자동 아님 | 낮게/높게·min/max 재검토(기획 판단) |
   | `COPIES` | owner 안 같은 꼴 사본 수가 달라짐 | 사본 전부 패치할지 판정(0.6.5 tactics_verify 실례: 1→3) |
   | `LOGIC_CHANGED` | 명령 꼴·조건코드·비교 구조·분기 대상 변화 | ghidra-re 로 그 함수 재조사 |
   | `SITE_MISSING`·`OWNER_MISSING` | 창/함수 소실 | 그 전술 이번 버전 비활성 또는 재탐색 |
2. 사람 판정 → **draft_targets 승격**: 초안 = `detail_tactics\draft_targets\`(`knobs_<라벨>.json`·`subjects_<라벨>.json`·`extra_{lane,combat,ops,verified}_<라벨>.json`,
   targets 와 같은 스키마 · REVIEW 창/전술엔 `"review"` 키). 판정대로 값을 고치고 `"review"` 키를 지운 뒤 targets 로 복사(초안은 track_update 가 만들고 **tools\targets\ 는 절대 안 덮어쓴다**):
   ```
   robocopy F:\claude\tfm2mods\MIG\_patchday\<라벨>\detail_tactics\draft_targets F:\claude\tfm2mods\tfm2_detail_tactics\tools\targets *_<라벨>.json /SJ /SL
   ```
   (robocopy 종료코드 8 미만 = 성공.) ⚠targets 어디든 `"review"` 키가 남아 있으면 `gen_sites.py` 가 **아무것도 쓰지 않고 중단**한다(그게 안전장치다).
3. 생성 — `-I` 금지(스크립트 폴더 import 를 막아 gen_extras 를 못 찾는다 · gen_sites docstring):
   ```
   python -X utf8 F:\claude\tfm2mods\tfm2_detail_tactics\tools\gen_sites.py --exe F:\claude\tfm2\tfm2_<라벨>\TeamfightManager2.exe --ver <라벨>
   ```
   (knobs 의 `exe_sha256` 이 신 exe 와 다르면 중단 · 지점마다 asm·즉치=기본값 변환을 대조 · 생성물 `src\sites.rs` 는 손으로 고치지 않는다)
4. 빌드·유닛테스트(게임 폴더 복사 안 함):
   ```
   powershell -NoProfile -ExecutionPolicy Bypass -File F:\claude\tfm2mods\tfm2_detail_tactics\build_local.ps1 -RunTests
   ```
5. 인게임 검증(유저 요청 시) → 배포 = `/deploy` → `rel_commit.ps1 -ModId tfm2_detail_tactics -Summary "<변경점>"`.
6. 매니페스트(`MIG\manifest\tfm2_detail_tactics.json`, 322 엔트리 · `roles`·`contracts` 큐레이션): sites.rs 가 생성물이므로 **`repin apply` 대상이 아니다**
   (매니페스트 notes 의 정본 순서 = targets → `gen_sites.py` → 매니페스트 재생성). ⚠**전용 재생성기는 없다**(2026-10-27 `tools\` 확인).
   `mig_verify.py gen` 은 큐레이션 엔트리를 **같은 value 일 때만** 보존한다(`mig_verify.py` `cmd_gen` — 2026-10-27 기준 L209 `hexv in keep and curated`) → 값이 바뀐 창의 `roles`·`contracts` 가 사라진다.
   ⟹ 재생성 방식은 그 회차에 정한다(미검증 — 메인/상세전술 담당 판단). 최소한 재생성 뒤 `mig_verify.py check tfm2_detail_tactics --exe <신exe>` 전 PASS 와 `coverage tfm2_detail_tactics` 클린을 확인.

## ④ tactics_verify (`tfm2_tactics_verify` — sites.rs 생성물, repin apply 대상 아님)
```
cd F:\claude\tfm2mods\tfm2_tactics_verify
python -I tools\migrate_targets.py tools\targets\probe_targets_0.6.5.json F:\claude\tfm2\tfm2_0.6.5\TeamfightManager2.exe F:\claude\tfm2\tfm2_<라벨>\TeamfightManager2.exe <라벨> tools\targets\overrides_0.6.5_<라벨>.json --oldpkl F:\claude\tfm2mods\_fnidx_065.pkl --newpkl F:\claude\tfm2mods\_fnidx_<tok>.pkl --oldcg F:\claude\tfm2mods\_cg_065.pkl --newcg F:\claude\tfm2mods\_cg_<tok>.pkl
```
- 종료코드 1 = 사람 판정 필요(출력 「== 사람 판정 필요 ==」) → `overrides_0.6.5_<라벨>.json` 에 판정(`"0x…"` 옮김 · `["0x…","0x…"]` 사본 분기 · `null`+reason 비활성) → 재실행. overrides 파일이 없으면 빈 판정으로 시작한다.
  표본 = `overrides_0.6.4_0.6.5.json`(15 사람판정: 레지스터 개명 r13→rdi 등 · 사본 1→3 · 로직변경 비활성).
- 그다음 `python -I tools\gen_sites.py tools\targets\probe_targets_<라벨>.json F:\claude\tfm2\tfm2_<라벨>\TeamfightManager2.exe` → `build_local.ps1`.

## ⑤ 나머지 모드 — `MIG\README.md` 표준 절차 (축 3개)
대상 = SUMMARY 3·4절에 STALE 가 있는 모드(③④ 의 두 모드 제외). `<신exe>` = `F:\claude\tfm2\tfm2_<라벨>\TeamfightManager2.exe`. 아래 상대 경로 명령은 전부 `cd F:\claude\tfm2mods` 에서.
⚠`repin.py apply` 는 맵의 **`new` 만** 쓴다(`repin.py` `cmd_apply` L332·L363 — `new_aligned` 는 안 봄). SUMMARY 가 보여 준 「aligned」 값을 채택하려면 `_mkfinal<tok>.py` FIX 표에 그 값을 적어 `new` 로 올려야 한다.
1. **잔여 판정**: `repin_review.md` 의 「★repin apply 대상 — 사람 판정 필요」 + 「동형 클론 주의」 → 판정 에이전트(ghidra-re 등)에 나눠 맡기고 결과 원문을 즉시 `REPORT\<MOD>\RE\YYYY-MM-DD_<라벨>-재핀판정.md` 로(CLAUDE.md §11). 「(참고) 생성물 모드」 구역은 판정하지 않는다(③④).
   ★회차마다 수동이 정상인 4엔트리(0.6.4·0.6.5 연속 엔진 오답): comptest `INLINE@…:147`(server_pregate)·`:137`(daily_inc_gate) · serpen `LAUNCHER_RET_A/B` — **sitealign 정렬값이 정답이었다**.
2. 산출물 보관 + final 맵:
   ```
   copy MIG\_patchday\<라벨>\repin_map.json MIG\repin_map_<tok>.json
   copy MIG\_patchday\<라벨>\repin_map.aligned.json MIG\repin_map_<tok>.aligned.json
   copy MIG\_mkfinal065.py MIG\_mkfinal<tok>.py      # SRC/DST 경로와 FIX 표만 이번 판정으로 교체
   python -X utf8 MIG\_mkfinal<tok>.py                # -> MIG\repin_map_<tok>.final.json
   ```
3. apply (먼저 dry, 그다음 쓰기 · 대상 모드를 명시 — detail_tactics·tactics_verify 는 넣지 말 것):
   ```
   python -X utf8 MIG\repin.py apply <MOD ...> --map MIG\repin_map_<tok>.final.json
   python -X utf8 MIG\repin.py apply <MOD ...> --map MIG\repin_map_<tok>.final.json --write --ver <라벨>
   python -X utf8 MIG\mig_verify.py rebase <MOD> --exe <신exe>          # 모드마다
   python -X utf8 MIG\mig_verify.py check --exe <신exe>
   python -X utf8 MIG\mig_verify.py coverage                               # ★apply 뒤 필수(치환 누락은 여기서만 보인다)
   python -X utf8 MIG\mig_verify.py dups
   ```
   ⚠소스 치환은 `newline=''`/바이트로만(0.6.5 교훈: 텍스트 모드 재기록이 혼합 줄바꿈 파일을 통째로 CRLF 로 바꿨다).
4. 구조체 오프셋(축②) — 기준선 백업 → check → sources → 소스 수정 → (작업 끝난 뒤) snap:
   ```
   robocopy F:\claude\tfm2mods\MIG\offsets F:\claude\tfm2mods\MIG\_offsets_bak_065_<YYYYMMDD> *.json /SJ /SL
   python -X utf8 MIG\offsets.py check --exe <신exe> --pkl F:\claude\tfm2mods\_fnidx_<tok>.pkl
   python -X utf8 MIG\offsets.py sources
   python -X utf8 MIG\offsets.py snap --exe <신exe> --pkl F:\claude\tfm2mods\_fnidx_<tok>.pkl --ver <라벨>   # ★오프셋 작업 완료 후에만
   ```
   (robocopy 는 종료코드 8 미만이 성공. 실행기 7단계의 `offsets_shifts.json` 은 apply 전 예보 — 0.6.4→0.6.5 실측 Player +0x20 · LPH +0x30 같은 균일 Δ가 먼저 보인다.)
   ⚠`offsets.py check` 는 기준선 함수를 매니페스트 **이름**으로 잇는다(`offsets.py` L118·L121) → 이름이 같은 엔트리 2개(serpen `SPAWN_HOOKS` 0xb073c0·0xb080f0)는 한쪽 값으로 겹쳐 **같은 줄이 두 번** 나온다(0.6.5 `MIG\_offsets_check_065.log` L10-11). 그 모드는 실행기 7단계 예보(값으로 잇는다)와 대조해서 읽을 것(도구 결함 — 이 런북 범위 밖).
5. 환경(축③) + 단일 진입점:
   ```
   python -X utf8 MIG\bump_deps.py --to <라벨> --mods <대상 모드,쉼표>   # ⚠--mods 없이 금지(0.6.5: 제외·프로브 모드 125파일까지 건드림) · 핫픽스 라벨이면 불필요
   python -X utf8 MIG\env.py --ver <라벨> --exe <신exe>
   python -X utf8 MIG\run.py --exe <신exe> --pkl F:\claude\tfm2mods\_fnidx_<tok>.pkl --ver <라벨>
   ```
   ★`run.py` 0 일 때만 "마이그 완료"(⑤ 클래식 빌드스크립트 `sdk_058` 참조 8건은 0.6.2~0.6.5 내내 「해당 없음」 사람 판정이었다).
6. 빌드·배포 = `/build`·`/deploy` → deploy-verify → `rel_commit.ps1`(릴리스 = 커밋 한 세트).

## ⑥ 지식베이스 — 주소 구→신 일괄 갱신
- `MIG\_patchday\<라벨>\addrmap\addrmap.md`(구→신 표 + 등급) 를 **record-keeper 에이전트**에 넘겨 INDEX §2·메모리 RVA 표를 일괄 갱신(CLAUDE.md §9 — KB 파일 직접 편집 금지).
  등급 SAME/MOVED_BYTES_CHANGED 는 그대로 반영, FN_EDITED/AMBIGUOUS/MISSING 은 `review.json` 판정 뒤에.
- 2절 EDITED 함수 목록은 judge/재현 계층(`tfm2_ai_adjust` 등) 영향 확인용으로 함께 넘긴다.

## ⑦ 기록 (세션 종료 전 필수)
- 회차 문서 `F:\claude\tfm2\mods_report\_<라벨>_마이그.md`(틀 = `_0.6.5_마이그.md`: 0 패치 성격 · 1 대상 · 2 재핀 · 2a RVA 외 수정 · 2b 오프셋 · 3 빌드·배포 · 5 인게임 검증 · 6 교훈).
- 모드별 `REPORT\<MOD>\03_시행착오.md`(실패 포함) · 상세전술 `REPORT\tfm2_detail_tactics\` 검증표.
- `MEM\CURRENT.md`(버전) · `DONE.md`(판정 한 줄) = record-keeper 경유.
- 커밋은 메인이 명시 경로만(같은 저장소에 다른 세션 미커밋 변경이 있다).

## ⑧ 회차가 끝나면 — 다음 패치 대비
1. `run_patchday.py` 상단 `BASE_VER`·`BASE_SHA`·`BASE_SIZE` 를 이번 버전으로(백업 경로·pkl·cg·기준선 파일명은 `BASE_VER` 에서 파생).
2. 새 기준선(전부 **마이그·gen 이 끝난 뒤** 신 exe 로):
   ```
   python -X utf8 F:\claude\tfm2mods\tfm2_detail_tactics\tools\track_update.py snapshot --exe <신exe> --ver <라벨> --pkl F:\claude\tfm2mods\_fnidx_<tok>.pkl --cg F:\claude\tfm2mods\_cg_<tok>.pkl --out F:\claude\tfm2mods\tfm2_detail_tactics\tools\patchday\baseline_<라벨>.json
   python -X utf8 F:\claude\tfm2mods\MIG\patchday\kb_harvest.py --ver <라벨> --exe <신exe> --prev-exe F:\claude\tfm2\tfm2_0.6.5\TeamfightManager2.exe --since <신버전 설치 시각 YYYY-MM-DDTHH:MM> --out F:\claude\tfm2mods\MIG\patchday\kb_rows_<라벨>.json
   python -X utf8 F:\claude\tfm2mods\MIG\patchday\addrbook.py build --ver <라벨> --exe <신exe> --pkl F:\claude\tfm2mods\_fnidx_<tok>.pkl --cg F:\claude\tfm2mods\_cg_<tok>.pkl --extra F:\claude\tfm2mods\MIG\patchday\kb_rows_<라벨>.json --out F:\claude\tfm2mods\MIG\patchday\addrbook_<라벨>.json
   ```
   ★`--extra` 를 빼면 매니페스트 행(≈500)만 담긴 주소록이 된다(KB 행 2.4만이 빠짐) — kb_harvest 를 **먼저**, 그리고 KB 의 RVA 를 신버전으로 갱신(⑥)한 **뒤에** 돌릴 것.
   kb_harvest 는 문서에서 **그 버전 태그가 붙은 값**·설치(`--since` = CURRENT.md §1 의 설치 시각) 이후 문서만 귀속하므로(kb_harvest.py docstring L13-26), KB 가 아직 구버전 값뿐이면 수확이 비어 있다. 0.6.5 판 수확 명령·결과 = `rehearsal_addrbook.md`.
   + `hotfix_exposure_<라벨>.json` 재생성(전용 생성기 없음 — 0.6.5 판은 `MIG\_patchday\_tmp\hotfix_exposure\`의 `collect.py`·`guard_sim.py`·`pdata_shift.py` 등 + 사람 판정으로 만들었다. 같은 감사를 다시 맡길 것) + `MIG\offsets\` snap(⑤-4). (각 도구의 docstring 이 인자 정본)
3. 거꾸로 리허설로 실행기 확인(아래).

## 리허설 (지금 할 수 있는 검증)
```
python -X utf8 MIG\patchday\run_patchday.py --help
python -X utf8 MIG\patchday\run_patchday.py                    # 설치본 = 0.6.5 → "아직 업데이트 안 됨" · 0 · 출력 없음
python -X utf8 MIG\patchday\run_patchday.py --rehearse --new-exe F:\claude\tfm2\tfm2_0.6.4\TeamfightManager2.exe --label 0.6.4                    # 전 단계(≈32분)
python -X utf8 MIG\patchday\run_patchday.py --rehearse --new-exe F:\claude\tfm2\tfm2_0.6.4\TeamfightManager2.exe --label 0.6.4 --skip addrbook    # 빠른 판(≈5~10분)
```
2026-10-27 실측(0.6.5 → 0.6.4 거꾸로, 출력 `MIG\_patchday\_rehearsal_0.6.4\`) = 아래 「리허설 기록」.
SUMMARY 수치 독립 대조 = `python -X utf8 MIG\patchday\crosscheck.py [<출력 폴더>]`(각 도구 json·log 에서 다시 세어 33항목 대조 · 0=일치 · 원본 = `_tmp\verifyC\crosscheck.py` 사본).

## 리허설 기록 (2026-10-27 · 기준 0.6.5 → 0.6.4 거꾸로)
| 항목 | 결과 | 근거 |
|---|---|---|
| `--help` · 설치본 detect | 종료코드 0 / 「아직 업데이트 안 됨」 · 출력 폴더 안 생김 | 콘솔 |
| 2·3 캐시 | `_fnidx_064/065`·`_cg_064/065` 전부 **검증 통과 후 재사용**(경계 155,151/155,151 · skel 표본 48 · callee 표본 40) | `logs\02·03` |
| 4 aidiff | EDITED 34 · SHIFTED 208 · IDENTICAL 492 · 사라짐 13 · 새로 생김 7 (캐시로 1.5s) | `aidiff.md` |
| 5 check | 13모드 STALE·INVALID 566 (detail_tactics 321 · tactics_verify 108 · comptest 72 …) | `logs\05_check.log` |
| 6 repin | 대상 566 · 엔진 해결 512 · 실패 54 · 사람 판정 132 = **apply 대상 16**(comptest 10·serpen 4·ui_kit 2) + 생성물 모드 116(detail_tactics 96·tactics_verify 20 — 참고) · 동형 클론 12 · 합 117~296s | `repin_review.md` |
| ★정답지 역대응 ① | 0.6.5 사람 확정 `repin_map_065.final.json` 137건의 역: **후보(`new_aligned` > `new`) 128건 일치**(repin 값만 쓰면 120). 불일치 9 = 엔진 FAIL 7(후보 없음 → 대기열) + 오답 2 — ORACLE_RVA(`BYTES_UNIQUE` 32B, 콜러 15→1·사영 0.21·size 0.77 → **WEAK 로 잡힘**) · ui_kit RVA_DROP_PLAN(skel 같은 클론 2개 → **CLONE 으로 잡힘**) ⟹ **표시 없이 지나간 오답 0** | `_tmp\run_patchday\invcheck.py` |
| ★정답지 역대응 ② | tactics_verify `probe_targets_0.6.5.json` migrated_from 56쌍 중 맵에 있는 45: 정답 40(자동 35·검토표시 5) · 오답 5 = 전부 검토표시(FAIL_OWNER — 0.6.5 의 사본 1→3 분기 자리) ⟹ **무표시 오답 0** | `_tmp\run_patchday\invcheck_tv.py` |
| 7 offsets 예보 | serpen DMGB **Δ-0x20** · lane PASSIVE_PLAN **Δ-0x30 ×15** · comptest ARRIVE Δ-0x20 · CT_REGION_LO Δ-0xb = 0.6.5 정식 check(`MIG\_offsets_check_065.log`: +0x20 · +0x30 ×15 · +0x20 · +0xb)의 **정확한 역**. ORACLE 행은 후보 WEAK 표시(쓰레기 Δ-0xb8). serpen SPAWN_HOOKS 2개는 값으로 이어 각자 후보(0xc526a0·0xc53130)와 대조(수정 전엔 이름 겹침으로 한쪽만 두 번) | `offsets_shifts.json` |
| 8 detail_tactics | 전술 100 → **AUTO 47 · REVIEW 53** · 창 218 = SAME 97 · REG_RENAME 55 · LOGIC_CHANGED 62 · COPIES 1 · SITE_MISSING 1 · OWNER_MISSING 2 · 기본값 변경 0 · 93~236s · 종료코드 1(정상). (track_update D1~D5 수정 뒤 — 전 판 AUTO 40 은 수정 전 값) | `detail_tactics\track.md` |
| 8 기본값 변경 표(합성) | 0.6.5 사본에 즉치 3곳을 바꾼 합성 exe(`_tmp\synth\`)의 track.json 으로 6절 렌더: 「★게임이 기본값을 바꾼 전술 (2)」 = combat_retreat_hp 35→**40** ×2창 · gank_call_rng 350→**400** — value_changes 3건 전부 표에 나옴 | `_tmp\verifyC\render_synth.py` |
| 9 addrbook | 24,120행 → SAME 11,900 · MOVED_BYTES_CHANGED 3,722 · FN_EDITED 1,719 · AMBIGUOUS 4,477 · MISSING 2,302 = **자동 15,622(64.8%)** · 행 오류 0 · **1673s(전 단계 판)·1616s(`--only addrbook` 판, 앞 단계 결과 유지 확인) · 메모리 약 9GB** · 종료코드 1(정상) | `addrmap\addrmap.json` summary |
| ★전 단계 1회(검증 C) | `--rehearse … --label 0.6.4`(skip 없음) **종료코드 0 · 1940s** · 단계 종료코드 detect 0 · backup SKIP · fnindex 0(CACHED) · callgraph 0(CACHED) · aidiff 0 · check 1 · repin 0 · offsets 1 · detail_tactics 1 · addrbook 1 · summary 0 (전부 계약대로) · 비파괴 경고 0 · 실행 전후 `git status --porcelain` 동일 · 매니페스트·offsets·targets·src·기준선·pkl 111파일 sha256 동일. glue 수정 뒤 `--skip addrbook` 재실행 544s → `--only addrbook` 1618s(README ①-a→①-b 그대로) · 둘 다 종료코드 0 · 경고 0 · 같은 확인 통과 | `_tmp\verifyC\` |
| ★SUMMARY 수치 대조 | 33항목(aidiff EDITED · check 합 · repin 대상/해결/실패/apply·생성물 판정 수 · fncheck 3종 · offsets · 상세전술 AUTO/REVIEW·창 6종·worst 4종·기본값 · 주소록 행·등급 5종 · 핫픽스 표 28행)을 각 도구 산출물에서 독립 재계산 → **불일치 0** | `_tmp\verifyC\crosscheck.py` |
| 비파괴 | 리허설 6회 중 경고 0 = 4회. 경고 2회(각 1건)는 둘 다 **같은 준비 워크플로에서 동시에 돌던 다른 과업의 쓰기**를 잡은 것(외부 세션 아님 — 메인 확인 2026-10-27): ① `tfm2_detail_tactics\tools\patchday\baseline_0.6.5.json` 14:31:52 변경(그 실행은 detail_tactics 를 건너뜀 — 추적기 과업의 `track_update.py snapshot`) ② `MIG\patchday\hotfix_exposure_0.6.5.json` 14:35:15 생성(핫픽스 감사 과업 산출물). 같은 이유로 `MIG\mig_verify.py`(MODS 에 `tfm2_detail_tactics` 편입)·`MIG\manifest\tfm2_tactics_verify.json`(45→108, `.bak_20261027`) 변경도 주소록 과업의 것이다. 실행기 자신의 쓰기 = 출력 폴더·pkl 뿐 · `MIG\offsets\_shifts.json`·매니페스트 git diff 없음 | `run.json` nondestructive_warnings |
| 8절 핫픽스 노출 | `hotfix_exposure_0.6.5.json` 의 conclusion 7줄 + rows 28행 표로 렌더 확인(`--only summary`) | `SUMMARY.md` 8절 |
| 전체 소요 | 전 단계 1940s(addrbook 1673s) · `--skip addrbook` 544s · detail_tactics·addrbook 제외 117~186s (전부 캐시 있음 · 백업·fnindex·callgraph·aidiff 신규 계산 없음) — 실전은 + fnindex 41s + callgraph 25s + aidiff 첫 회 수 분 + 백업 복사 | `run.json` total_sec |
| glue 수정(검증 C, 2026-10-27) | ① 7단계 오프셋 워커가 기준선을 매니페스트 **이름**으로만 이어 serpen SPAWN_HOOKS 2개가 한 값으로 겹침 → 값 먼저·이름 다음 ② 4절 「사람 판정 필요」 132 중 116 이 생성물 모드라 SUMMARY 앞 60행에서 ui_kit 2건이 잘려 안 보임 → apply 대상/생성물 모드 분리(대기열 파일도 3구역) ③ 6절 검토 전술 근거가 「참고:」 줄을 보여 줌 → worst 판정의 근거 줄 선택 ④ 7절 주소록 요약이 JSON 400자 절단 → 자동/검토 수·종류×등급 표·바뀐 방식 상위 ⑤ 5절 Δ 없는 행에 사라짐/생김 표시 ⑥ 1절 상세전술·주소록 단계 메모에 집계 수 | `run_patchday.py` |

⚠거꾸로 리허설의 REVIEW 수는 실전 예측이 아니다(0.6.4 와 0.6.5 사이엔 소스 변경·전면 재링크가 있었다). 확인한 것은 **배선·비파괴·판정 정확도(무표시 오답 0)** 다.
