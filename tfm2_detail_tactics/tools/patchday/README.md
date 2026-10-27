# 상세전술 패치데이 추적 (tools/patchday)

게임이 업데이트되면 상세전술 **전술 100개 · 패치창 218개**(+ Src::Caller 반환 콜사이트 66 · 창 안 분기 대상)가
새 exe 에서 어디로 갔고 **어떻게 바뀌었는지**를 `tools/track_update.py` 가 판정한다.
이 폴더의 `baseline_<버전>.json` 이 기준선(현행 버전에서 `snapshot` 으로 만든 것)이다.

- 현재 기준선: `baseline_0.6.5.json` (exe sha256 `DAF040DB…11D30B5`, 2026-10-27 생성)
- 도구 전체 설명·판정 어휘 = `tools/track_update.py` 첫 docstring

## 패치 당일 순서

0. **준비** — 새 exe 를 백업하고(`F:\claude\tfm2\tfm2_<신>\TeamfightManager2.exe`) 지문 캐시를 만든다.
   ```
   python -X utf8 F:\claude\tfm2mods\fnindex.py <신exe> F:\claude\tfm2mods\_fnidx_<신>.pkl
   python -X utf8 F:\claude\tfm2mods\MIG\callgraph.py <신exe> F:\claude\tfm2mods\_fnidx_<신>.pkl F:\claude\tfm2mods\_cg_<신>.pkl
   ```
   (패닉 Location census 는 track 이 처음 돌 때 `MIG\_aidiff_<sha16>_8fabeb_v6.pkl` 로 자동 생성 — 수 분. aidiff 의 표준 캐시 위치다.)

1. **track** (게임 실행 불필요 · 게임 폴더에 쓰지 않음 · `tools/targets/` 를 덮어쓰지 않음)
   ```
   python -X utf8 F:\claude\tfm2mods\tfm2_detail_tactics\tools\track_update.py track ^
     --base F:\claude\tfm2mods\tfm2_detail_tactics\tools\patchday\baseline_0.6.5.json ^
     --old-exe F:\claude\tfm2\tfm2_0.6.5\TeamfightManager2.exe --new-exe <신exe> --new-ver <신> ^
     --old-pkl F:\claude\tfm2mods\_fnidx_065.pkl --new-pkl F:\claude\tfm2mods\_fnidx_<신>.pkl ^
     --old-cg F:\claude\tfm2mods\_cg_065.pkl --new-cg F:\claude\tfm2mods\_cg_<신>.pkl ^
     --out F:\claude\tfm2mods\MIG\_patchday\<신>\track
   ```
   종료코드 `0` = 전 전술 자동확정 · `1` = 검토 필요 항목 있음(**정상**) · `2` = 실행 오류(입력 불일치 등 — 성공으로 보지 말 것).
   2~3분 걸린다(repin 바이트 유일검색 교차 확인 포함).

2. **`track.md` 확인** — 맨 위 요약 표(판정별 전술 수·창 수) → 「게임이 기본값을 바꾼 전술」 → 「검토 필요 전술」(사유·제안) → 전술 100개 전량 표.

3. **`review_queue.json` 사람 판정** — REVIEW 전술마다 창별 사유와 제안(`proposal`: 새 rva·win·src·team_chk)이 있다.
   | 판정 | 사람이 볼 것 |
   |---|---|
   | `VALUE_CHANGED` | 게임 기본값 구→신. **기획 재검토**(낮게/높게·min/max). 기본32 전술은 `src/tactics.rs` 의 손으로 쓴 default 도 고친다. 절대 자동확정 안 됨 |
   | `REG_RENAME` | 레지스터·스택 슬롯·프레임·필드 변위·콜사이트가 바뀜. 제안 Src 는 정렬 다수결이라 **디스어셈/런타임으로 확인**. `dynamic_hex 재작성 필요` 사유가 있으면 extras 의 손 조각을 다시 짠다 |
   | `COPIES` | 같은 꼴 창의 사본 수가 바뀜 → 사본마다 지점을 추가/삭제할지 판단(0.6.5 의 1→3 사례) |
   | `LOGIC_CHANGED` | 소비 조건코드·비교 구조·분기 대상 변화, 또는 owner 소스 변경(aidiff EDITED)+창 주변 변화 → 그 전술 의미가 그대로인지 RE |
   | `SITE_MISSING` / `OWNER_MISSING` | 창·Caller 콜사이트·함수를 못 찾음 → ghidra-re |
   | `BLOCK:` 사유(SAME 인데 보류) | owner 대응 Location≠repin, Location 약한 근거, repin 바이트 유일검색 충돌 |

4. **초안 승격** — `track` 출력 `draft_targets/` 의 `knobs_<신>.json · subjects_<신>.json · extra_*_<신>.json`(기존과 같은 스키마)을
   사람이 고친 뒤 `tools/targets/` 로 옮긴다. REVIEW 항목엔 `"review"` 키가 붙어 있다 — **판정이 끝난 것만 키를 지운다.**

5. **생성** — `python -X utf8 tools/gen_sites.py --exe <신exe> --ver <신>`
   targets 어디든 `"review"` 키가 남아 있으면 아무것도 쓰지 않고 중단한다(검토 안 끝난 초안이 빌드로 새는 것 방지).
   `--ver` 는 knobs/subjects 와 `extra_*_<신>.json`(gen_extras) 모두에 쓰인다.

6. **빌드·검증** — `/build` → 인게임/리플레이 검증 → 매니페스트(`MIG\manifest\tfm2_detail_tactics.json`) 재생성 → `/deploy`.
   그다음 새 버전 기준선을 만든다:
   `python -X utf8 tools/track_update.py snapshot --exe <신exe> --ver <신> --pkl <신fnidx> --cg <신cg> --out tools/patchday/baseline_<신>.json`

## 무엇을 판정하나 (요지)

- owner 대응 = **패닉 Location 집합**(aidiff.pair, 모듈별 행 시프트) 1순위. AI 계층(census) 밖 함수는 같은 모듈 Location 을 `lea [rip+]` 로
  참조하는 신 함수 중 집합이 같은 유일 후보(LOC_SEARCH). `repin.build_match`(fnidx skel·콜그래프)로 교차 확인해 일치 여부를 남긴다.
  모듈 이름만 바뀐 경우(`MODULE_RENAMED`)와 aidiff 단일 시프트 한계(`EDITED(시프트 갈림)`)를 따로 표시한다.
- owner 안 위치 = 명령열 정렬(레지스터 지운 키 LCS → 꼴 키로 틈 메움; 6천 명령 넘는 함수는 창 주변 구간만). 실패하면 같은 꼴 후보 0/1/여러 개로 가른다.
- SAME = 창 바이트(rel 변위 제외) 동일 + owner 프레임 동일 + Src/team_chk 성분(레지스터·슬롯·Caller 체인)이 근거 명령으로 동일 + Caller 콜사이트 동일 + 창 안 분기 대상 대응 + 사본 수 동일.
- Src 성분은 **창 사이트 rsp 기준**(micro.rs `Frame::reg(4)`)이므로 `[rsp+d]` 를 프레임 체인(owner → Caller → 중첩 Caller)의
  지역 슬롯 / push 저장 레지스터 / 반환주소 / 진입 인자 영역으로 풀어 단별로 사상한다. `return_off` 는 프롤로그 frame(push×8+sub)과 일치함을 기준선에서 확인(0.6.5: 불일치 0).

## 한계 (사람 확인 필요)

- owner **밖으로** 인라인된 사본은 세지 않는다(COPIES 는 owner 안 사본 수 비교).
- Src 제안(`RENAMED`)은 정렬된 명령쌍 다수결 — 제어흐름 분석이 아니다. 반드시 디스어셈 또는 런타임(`game==mine`)으로 확인.
- `reference_windows`(참고창)·`notes`·`func_rva_note`·knobs 의 `key`(tactics_verify 0.6.4 키) 안 RVA 는 추적하지 않는다.
- `Tf::Compute` 창은 기본값을 역산하지 못한다(즉치 구→신만 보고).

## 검증 기록 (2026-10-27)

| 시험 | 결과 |
|---|---|
| 자체시험 0.6.5→0.6.5 (`MIG\_patchday\_tmp\selftest_tracker\`) | 100 전술 AUTO · 218 창 SAME · RVA 전부 동일 · draft_targets 6파일이 `tools/targets/*_0.6.5.json` 과 파싱 후 완전 동일(키 순서 포함) |
| 리허설 0.6.5→0.6.4 (`MIG\_patchday\_tmp\rehearse_tracker_064\`, 전면 재링크 역방향) | 오류 없이 종료(코드 1). 기본32 의 53 창 신 RVA 가 knobs `key`(=0.6.4 정답) 와 53/53 일치. 정밀 채점은 미실시(다음 단계) |
| 합성 exe(0.6.5 사본에 즉치 2·조건코드 1·레지스터 1 바이트 패치, `MIG\_patchday\_tmp\synth\`) | 의도한 4 전술만 REVIEW(VALUE_CHANGED 35→40·350→400, LOGIC_CHANGED ja→jae, REG_RENAME rax→rcx), 나머지 96 AUTO. review 키 제거 후 gen_sites 생성 성공 |
| gen_sites review 가드 | review 키 있는 초안 버전에서 중단, 없는 버전은 통과 |
