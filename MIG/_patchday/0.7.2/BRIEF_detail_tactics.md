# 상세전술 0.7.2 서브세션 지시서 (메인 세션 작성 · 2026-11-11)

> 유저 지시(11-11): 「상세 전술모드 하나만 따로 서브세션 만들어서 거기서 마이그 진행, 기존 전술중에 로직 변경된거 확인해서 추가 수정 및 삭제 등 고려하고, ai판단함수 전수조사 해」
> 메인 세션은 나머지 모드(재핀 6 · ui_kit 소비 3 · transfer_fix · ui_kit)를 맡는다. 언어는 전부 한국어.

## 이미 된 것 (다시 하지 말 것)
- 0.7.2 설치(정식 버전업, base_version `0.7.2`) · exe 94,603,776B · sha `4B324F3127953A67EE1972F0D8CEB0673BB4FACCAAB530F0538DA64DD70319FB`
- exe 백업 `F:\claude\tfm2\tfm2_0.7.2\` · `run_patchday.py --skip addrbook` 완료(1114s · 오류 0 · 비파괴 경고 0)
  → 출력 `F:\claude\tfm2mods\MIG\_patchday\0.7.2\` (`SUMMARY.md` · `aidiff.md` · `detail_tactics\track.md` · `review_queue.json` · `draft_targets\`)
  → pkl `F:\claude\tfm2mods\_fnidx_072.pkl` · `_cg_072.pkl`
- 상세전술 추적: 전술 100 → AUTO 18 · REVIEW 82 (LOGIC_CHANGED 39 · REG_RENAME 27 · SITE_MISSING 15 · OWNER_MISSING 1) · 창 208
  - ★게임 기본값 변경 1 = #27 `base_defense_min_enemies` 6 → 2 (즉치 5→1)
- aidiff: AI 판단함수 EDITED 157 · SHIFTED 393 · IDENTICAL 222 · 새로 생김 159 · 사라짐 10

## 할 일
1. **상세전술 마이그** — 런북 `F:\claude\tfm2mods\MIG\patchday\README.md` ③ 그대로:
   track.md·review_queue 판정 → draft_targets 승격 → `gen_sites.py --ver 0.7.2` → `verify_default_equiv.py`(+`--mutate`)
   → callerframe/callercase 스캔 → `build_local.ps1 -RunTests` → 매니페스트 재생성
   + `mig_verify.py check tfm2_detail_tactics --exe <신exe>` 전 PASS · `coverage` 클린.
   0.7.1 회차 절차·함정 = `REPORT\tfm2_detail_tactics\23_0.7.1_마이그.md` · `03_시행착오.md`.
2. **로직이 바뀐 기존 전술 재검토** — LOGIC_CHANGED·SITE_MISSING·OWNER_MISSING 전술마다 0.7.2 에서 그 판단이 어떻게 바뀌었는지
   RE(ghidra-re 등)로 확인하고, 전술별로 「그대로 이식 / 의미 수정(이름·범위·기본값) / 새 지점으로 재설계 / 삭제·비활성」 중 하나를 근거와 함께 정한다.
   #27 기본값 6→2 는 노브 범위·설명도 재검토. 0.7.2 의 새 판단 로직으로 **새 전술 후보**가 생겼으면 목록으로 제안.
   - ⚠전술 **삭제·의미 변경·신규 추가는 유저에게 표로 제안하고 승인받은 뒤** 적용. 단순 이식(REG_RENAME 등)은 바로 진행.
3. **AI 판단함수 전수조사** — 0.7.1 방식(`REPORT\_0.7.1_AI변경점_종합.md` · 작업 폴더 `MIG\_patchday\0.7.1\aiwide\` 의
   `BRIEF_AI.md`·`BRIEF_U.md`·`tools\`)을 따라 0.7.1→0.7.2 EDITED 157 · 신규 159 · 사라짐 10 을 영역별로 조사.
   작업 폴더 `MIG\_patchday\0.7.2\aiwide\` · 결과 `REPORT\_0.7.2_AI변경점_종합.md` + 원문 `REPORT\_0.7.2_RE\`. 2번 판정과 근거를 공유.

## 경계 (메인 세션과 충돌 방지)
- `F:\claude\tfm2mods` 는 메인과 **같은 체크아웃**이다. 상세전술 세션이 쓰는 곳 = `tfm2_detail_tactics\` · `MIG\manifest\tfm2_detail_tactics.json`
  · `MIG\_patchday\0.7.2\detail_tactics\` · `MIG\_patchday\0.7.2\aiwide\`.
  **`MIG\mig_verify.py` · `MIG\offsets\` · `MIG\repin_map_072*` · `bump_deps.py` · 다른 모드 소스/매니페스트는 건드리지 않는다**(메인 몫 — 필요하면 메인에 메시지).
- 커밋은 명시 경로만(`git add -A` 금지 · 다른 세션 미커밋 변경 있음). `tfm2_tactics_verify` 는 보류 상태라 손대지 않는다.
- REPORT(`mods_report\`)는 자기 워크트리 안에 쓰고 자기 브랜치에 커밋(0.7.1 때 `claude/detailed-tactics-migration-591e25` 처럼 나중에 병합).
  MEM·ANA 반영은 record-keeper 경유, CURRENT.md 는 상세전술 줄만.
- RE 결과는 받는 즉시 `REPORT\tfm2_detail_tactics\RE\2026-11-11_0.7.2-*.md`(지시 → 요약 → 원문).
- 게임 실행·인게임 검증은 유저가 요청할 때만. 게임 폴더 dll 배포는 게임이 꺼져 있을 때만(`/deploy`).
  서브 PC(1호기) 설치는 메인이 일괄로 하니 하지 않는다. 릴리스 zip 은 인게임 PASS 뒤 `rel_commit.ps1 -ModId tfm2_detail_tactics -Summary …`.

## 끝
gen_sites 생성 + 등가검사 0 + 유닛테스트 통과 + `mig_verify check` 상세전술 전 PASS + 전술별 처리표(유저 승인 반영)
+ `_0.7.2_AI변경점_종합.md` + REPORT(`24_0.7.2_마이그.md` · 03 · 검증표) 갱신·커밋. 인게임 검증 대기면 그 상태를 명시해 보고.
