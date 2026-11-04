# 0.7.0 → 0.7.1 재핀 판정 공통 브리프 (2026-11-04)

너는 TFM2(게임 exe, PE32+ x64, 심볼 없음, image base 0x140000000, RVA = abs − base) 패치 마이그의 **판정 담당**이다.
맡은 모드의 매니페스트 엔트리 **전부**에 대해 0.7.1 RVA 를 확정하고, RVA 외에 고쳐야 할 소스 상수를 찾아낸다.
모든 출력은 한국어. 코드·경로·식별자는 원문.

## 하지 말 것
- 모드 소스(.rs)·매니페스트(`MIG\manifest\`)·`MIG\offsets\`·`mig_verify.py` 를 고치지 말 것. 이 작업은 **판정만** 한다. 적용은 메인이 한다.
- 게임을 실행하지 말 것. `git commit`·`git stash` 하지 말 것.
- 역슬래시가 든 텍스트를 Bash heredoc 으로 쓰지 말 것(경로가 제어문자로 깨진다). 문서·스크립트는 Write 도구로 쓴다.
- 큰 출력은 작업 폴더에만. C 드라이브에 만들지 말 것.
- `tfm2_detail_tactics` 는 범위 밖이다(별도 세션). 건드리지 말 것.

## 재료
| 무엇 | 경로 |
|---|---|
| 구 exe (0.7.0, sha D678E012…) | `F:\claude\tfm2\tfm2_0.7.0\TeamfightManager2.exe` |
| 신 exe (0.7.1, sha BD011C42…) | `F:\claude\tfm2\tfm2_0.7.1\TeamfightManager2.exe` |
| 함수 지문 | `F:\claude\tfm2mods\_fnidx_070.pkl` · `_fnidx_071.pkl` |
| 콜그래프 | `F:\claude\tfm2mods\_cg_070.pkl` · `_cg_071.pkl` |
| 패치날 요약 | `F:\claude\tfm2mods\MIG\_patchday\0.7.1\SUMMARY.md` |
| 재핀 후보 | `F:\claude\tfm2mods\MIG\_patchday\0.7.1\repin_map.aligned.json` (`new` = 엔진값, `new_aligned` = 정렬값, `align` = 근거) |
| 함수시작 교차검증 | `...\0.7.1\repin_fncheck.json` |
| 사람 판정 대기열 | `...\0.7.1\repin_review.md` |
| 오프셋 예보 (0.7.0 기준, 재채록본) | `...\0.7.1\offsets_shifts.json` · SUMMARY 5절 |
| AI 함수 대조 | `...\0.7.1\aidiff.md` |
| 매니페스트 | `F:\claude\tfm2mods\MIG\manifest\<MOD>.json` |
| 지난 회차 판정 원문 | `F:\claude\tfm2\mods_report\<MOD>\RE\2026-10-28_0.7.0-재핀판정*.md` |
| 지난 회차 RVA 외 수정 목록 | `F:\claude\tfm2\mods_report\_0.7.0_마이그.md` §2·§2a·§2b·§6 |
| 지난 회차 판정 도구 | `F:\claude\tfm2mods\MIG\_patchday\0.7.0\work_*\` (`h.py` 콜슬롯 계보 투표, `adiff.py` 정규화 명령 diff 등) — 복사해서 0.7.0/0.7.1 로 경로만 바꿔 재사용 |
| 방법 라우팅 | `F:\claude\tfm2mods\MIG\METHOD_MAP.md` §0 · `MIG\README.md` 「0.6.0 추가」 절 |

Ghidra 프로젝트에는 0.6.5 이후 exe 가 들어 있지 않다. 기본 도구는 capstone 으로 exe 를 직접 읽는 파이썬이다.
`F:\claude\tfm2mods\` 의 `fnindex.py` 와 지난 회차 `h.py` 를 먼저 볼 것.

## 이번 패치에서 이미 아는 것
- 함수 165,552 → 166,780. 코드 전체가 재배치됐다. 함수 순서도 크게 바뀌었다(예: 0x1bdbdb0 → 0x140dxxx 대).
- stable SDK ABI 10 불변. `.ui`·`.style` 552개 바이트 동일.
- 패치 내용(번들 텍스트 diff):
  - 퇴마사 스킬 설명에 정화 대상 문구 추가
  - 급료 예산 미리 잡는 기간 3년 → 2년 (`budget_note`)
  - **연습경기 자동 영입 동의 팝업 신규** (`practice_recruitment`: 선발 인원이 부족하면 자동 영입 후 연습경기 진행을 묻는다)
- 팝업이 새로 생겼으니 **서버↔클라이언트 메시지 enum 태그·핸들러 표·씬 구조체가 또 밀렸을 수 있다**(0.7.0 에서 메시지 태그 전체 +3 이었다).

## 판정 규칙 (전부 실사고에서 나온 것)
1. **엔진값은 후보다.** `BYTES_UNIQUE` 도 오답이 있었다. 정렬값도 한 명령 밀린 적이 있다(0.7.0 comptest DRIVE). AGREE·STRONG 도 표본이 아니라 전 엔트리를 근거와 함께 확인한다.
2. **함수 시작** = 콜러 수 · callee 사영 · size 비 3종 중 2종 일치. 동형 클론은 콜슬롯 계보 투표로(구 콜러의 신 짝을 찾아, 같은 콜 순서 자리의 신 타깃을 센다). 클론 skel 일치만으로 채택하지 말 것.
3. **함수 중간 사이트** = owner 명령열 정렬로 대응시키고, 콜사이트면 call 타깃이 독립 재핀된 피호출 함수와 일치하는지, 바이트패치면 orig 바이트가 소스 기대값과 같은지 본다.
4. **RET(리턴주소) 엔트리** = 피호출 함수의 콜사이트 전수 대응.
5. **vtable 슬롯 함수** = 표를 먼저 재핀하고 슬롯에서 읽는다. .rdata 엔트리는 그 표·상수를 참조하는 코드 쪽에서 역으로 확정한다.
6. **PASS·바이트 일치 ≠ 구조 일치.** 함수가 바뀌었으면 소스가 기대하는 인자·필드·호출 규약이 그대로인지 본다.
7. **소스의 orig/SIG/프롤로그 상수**: 훅 진입부·패치 지점의 새 바이트를 소스 기대값과 대조해 달라진 것을 전부 보고한다(파일:줄).
8. **오프셋 예보의 Δ ≠ 소스 수정.** 함수별로 「소스가 그 필드를 쓰나」를 가린다. 반대로 예보에 안 잡히는 변경(imul 즉값, 구조체 stride, `const` 오프셋, 인라인 `+0x..`)은 소스의 오프셋 상수를 전수로 훑어서 신 exe 의 대응 명령으로 직접 확인한다. `offsets.py sources` 는 `const` 를 못 본다.
9. 신뢰도는 HIGH / MED / LOW 로 쓰고 근거를 붙인다. 근거 없는 값은 쓰지 말 것. 못 찾았으면 `null` 과 이유를 쓴다. 모르는 것을 아는 척하지 말 것.
10. 판정에는 적용 범위를 쓴다(「X 불가」가 아니라 「A 방식으로 X 불가」).

## 산출물 3가지
1. **판정 JSON** `F:\claude\tfm2mods\MIG\_patchday\0.7.1\judge_<갈래>.json`
   형식(지난 회차 `MIG\_patchday\0.7.0\judge_*.json` 과 동일):
   `{"<MOD>": {"<구 RVA hex>": ["<신 RVA hex 또는 null>", "HIGH · 근거 한 줄"], ...}}`
   키는 `repin_map.aligned.json` 의 해당 모드 키 **전부**(엔진값 채택분 포함). 빠진 키가 없게 스크립트로 검사할 것.
2. **RE 원문** `F:\claude\tfm2\.claude\worktrees\0-7-1-migration-a61329\mods_report\<MOD>\RE\2026-11-04_0.7.1-재핀판정<-주제>.md`
   (⚠ `F:\claude\tfm2\mods_report\` 가 아니라 이 워크트리 경로에 쓴다.) Write 도구로 쓴다.
   첫 줄 `지시:` → 다음 줄 `요약:`(3~5줄) → 원문 전체. 형식은 지난 회차 파일을 볼 것.
3. **최종 반환 메시지** (메인에게): 아래만.
   - 판정 분포(HIGH/MED/LOW, 엔진값 그대로 / 엔진 오답 / FAIL→확정)
   - 엔진 오답 표(이름·엔진값·정답·왜 위험한가)
   - **RVA 외 소스 수정 필요 목록**: `파일:줄` · 상수명 · 구값 → 신값 · 근거. 이게 가장 중요하다.
   - orig/SIG/프롤로그 변화
   - 남은 불확실·런타임 확인이 필요한 것
   - 쓴 파일 경로

작업 폴더 = `F:\claude\tfm2mods\MIG\_patchday\0.7.1\work_<갈래>\`.
