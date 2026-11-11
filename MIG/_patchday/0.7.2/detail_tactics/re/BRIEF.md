# 0.7.2 상세전술 창 판정 — 공통 지침 (2026-11-11)

## 상황
- TFM2 가 0.7.1 → **0.7.2** 로 올라갔다(정식 버전업 · 전면 재링크 · 함수 166,780 → 167,313 · AI 판단계층 aidiff **EDITED 157** · SHIFTED 393 · 사라짐 10 · 새로 생김 159 — 0.7.1 회차(EDITED 98)보다 경기 AI 변경이 크다).
- 구 exe = `F:\claude\tfm2\tfm2_0.7.1\TeamfightManager2.exe` · 신 exe = `F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe` (image base 0x140000000, RVA = abs − base).
- 모드 `tfm2_detail_tactics`(상세전술)는 게임 AI 판단함수 안의 **비교 상수 창**(예: `cmp rax, 0x23` = HP 35%)을 런타임에 내 팀 선수에 한해 바꾼다. 창마다 ①위치(RVA) ②창 바이트 ③값 필드 ④주체 `Src`(창 시점에 판단 대상 **Player 포인터**를 어디서 읽는지) ⑤`team_chk`(팀 번호를 어디서 읽는지·없을 수도) ⑥`dynamic_hex`(값 비교를 대신하는 손으로 쓴 조각) ⑦`tf`(값 변환 — `Compute` 면 Rust 콜백)가 있다.
- 자동 추적기 `track_update.py` 가 208창을 판정했다(전술 100 = AUTO 18 · REVIEW 82 = LOGIC_CHANGED 39 · REG_RENAME 27 · SITE_MISSING 15 · OWNER_MISSING 1). 결과 = `F:\claude\tfm2mods\MIG\_patchday\0.7.2\detail_tactics\track.md`·`track.json`·`review_queue.json`.
- ★게임이 기본값을 바꾼 전술 1 = #27 `base_defense_min_enemies` 6 → 2(즉치 5→1 · 창 `base_defense_min_enemies_6@0xf3b00c`). 그 전술이 든 묶음은 노브 범위·설명도 재검토해 `proposal` 에 쓴다.
- 추적기는 **보수적으로** 표시한다(owner 소스가 어딘가 바뀌었고 창 주변이 달라지면 LOGIC_CHANGED). 동시에 0.7.0 회차에서는 **그럴듯한 제안이 오답인 경우가 많았다**(`F:\claude\tfm2\.claude\worktrees\salary-transfer-fee-analysis-1bf168\mods_report\tfm2_detail_tactics\21_0.7.0_대응조사.md` §4 — 착수 전 필독). 추적기 결론을 그대로 믿지도, 반대로 가정하지도 말고 **독립적으로** 판정한다.

## 이미 있는 것 (먼저 읽기 — 같은 일을 두 번 하지 말 것)
경로 앞부분 `WT` = `F:\claude\tfm2\.claude\worktrees\salary-transfer-fee-analysis-1bf168`
- **0.7.1 회차 창 판정(직전 판)** `WT\mods_report\tfm2_detail_tactics\RE\2026-11-04_0.7.1-창판정_<네 묶음>_*.md` + 기계판 `F:\claude\tfm2mods\MIG\_patchday\0.7.1\detail_tactics\re\<네 묶음>.json` — 각 창의 0.7.1 의미·Src 근거·콜백 입력이 여기 있다. 0.7.2 판정의 출발점.
- **0.7.1 회차 요약** `WT\mods_report\tfm2_detail_tactics\23_0.7.1_마이그.md`(§1 사양 변화 · §2 콜백 입력 수정표) · 함정 `03_시행착오.md`.
- **0.7.1 AI 변경 종합** `WT\mods_report\_0.7.1_AI변경점_종합.md`(0.7.1 시점 이름표·구조) + 원문 `WT\mods_report\_0.7.1_RE\`.
- 전술 설명 `WT\mods_report\tfm2_detail_tactics\00_기획_전술목록.md`·`19_실측12종_정식추가.md`·`22_0.7.0_승격구현.md`(§9 범위 확장 = Caller 사례 구성 근거).
- 같은 시각에 다른 세션 에이전트들이 0.7.1→0.7.2 AI 판단함수 전수 조사를 한다(`F:\claude\tfm2mods\MIG\_patchday\0.7.2\aiwide\`). 아직 결과가 없을 수 있다 — 기다리지 말고 네 창 기준으로 직접 본다.

## 확인 안 된 것 (0.7.2 · ★G1 만 먼저 확인해서 G1.md 맨 위에 적는다 · 다른 묶음은 결론을 가정하지 말고 Src 판정에 필요한 범위만 본다)
- 캐시 조회 헬퍼 바이트(캐시 `+0x238` 챔프 · `+0x288` Player · 엔티티 `+0x5b8`)가 구·신 같은가.
- Player `+0x810`(선수) · `+0x820`(팀) · `+0x8b0`(포지션)이 그대로인가 — 0.7.1 회차는 Athlete Debug 표 31필드 구·신 대조로 확인했다(0.7.1 위치 `0x1c361f0` 부근 · 도구 `F:\claude\tfm2mods\MIG\_patchday\0.7.2\aiwide\tools\dbgfield.py`·`s13.py` 참고). 모드 사용처 = `F:\claude\tfm2mods\tfm2_detail_tactics\src\micro.rs` L64-116.

## 도구 (읽기 전용 · 파일을 안 쓴다)
```
cd F:\claude\tfm2mods\MIG\_patchday\0.7.2\detail_tactics
python -X utf8 sbs.py <tactic_id | 창 이름 일부 | 구RVA> [-n 40]
```
- 창마다 ①추적기 판정·사유·제안 ②0.7.1 targets 원래 의미(asm·operand_meaning·branch taken/not_taken) ③구 owner 창 ±n 명령(`LOC 파일:행:열` = 패닉 Location, ▶=창) ④신 후보 ±n 명령. (한 번에 10초 남짓)
- `re\tools\` (old|new 인자 = 0.7.1|0.7.2):
  - `flow.py <old|new> <func_start> <func_end> <win_rva>...` : 진입 인자 라벨이 창 시점에 어디 있는지 전 경로 추적(Src 확정의 기본 도구 · chkstk 프롤로그 처리됨). `..\widen\fl.py <ret_rva>...` = 콜사이트 시점 flow.
  - `d.py`·`dz.py <old|new> START END` 디스어셈 · `dz.py <old|new> func RVA` 함수 전체 · `fcmp.py`·`fdiff.py`·`ndiff.py` 구/신 함수 비교 · `callers.py <old|new> <rva>...` E8 호출처 · `xref.py` 분기 포함 참조 · `dispscan.py <old|new> DISP` 변위 검색 · `hpscan.py` HP 패턴 검색 · `wincheck.py` 창 바이트 검사.
  - `..\widen\pro.py <rva>` 프롤로그·rbp 덮어쓰기 · `..\widen\slotw.py <fn>:<disp>` 슬롯 쓰기/읽기 수.
- 직접 스크립트: `import sys; sys.path.insert(0, r'F:\claude\tfm2mods\MIG'); import aidiff as A` → `A.Img(path).code(rva, n)` · `.owner(rva)` · `A.find_locations(img)`. ⚠`python -I` 금지(`-X utf8` 사용). ⚠역슬래시 든 텍스트를 Bash heredoc 으로 파일에 쓰지 말 것(Write 도구를 쓴다).
- AI 함수별 소스 변경(사라진/생긴 패닉 행·상수 변화) = `F:\claude\tfm2mods\MIG\_patchday\0.7.2\aidiff.md`(크다 — grep 으로 해당 함수만).
- 0.7.1 targets 원본 = `F:\claude\tfm2mods\tfm2_detail_tactics\tools\targets\*_0.7.1.json` · 추적기 초안 = `F:\claude\tfm2mods\MIG\_patchday\0.7.2\detail_tactics\draft_targets\*_0.7.2.json`.
- 콜백 입력표 = `F:\claude\tfm2mods\MIG\_patchday\0.7.2\detail_tactics\calc_table.md`(현행 0.7.1 sites.rs 기준 · 창×콜백 110 중 입력 있는 53). 콜백 소스 = `F:\claude\tfm2mods\tfm2_detail_tactics\src\extra_*.rs`·`verified_candidates.rs`.
- 임시 파일 = `C:\Users\jungs\AppData\Local\Temp\claude\F--claude-tfm2--claude-worktrees-salary-transfer-fee-analysis-1bf168\c5a49446-ad0c-452b-95b1-138bd849eaca\scratchpad\<네 묶음>\` 에만.
- Ghidra 에는 0.7.x 가 없다. capstone 도구로 충분하다.

## Src 표기 (micro.rs)
레지스터 번호 0=rax 1=rcx 2=rdx 3=rbx 4=rsp 5=rbp 6=rsi 7=rdi 8~15=r8~r15.
`reg:r`=레지스터 값 · `stack:d`=[rsp+d] · `mem:r:d`=[reg+d] · `stackmem:a:b`=[[rsp+a]+b] · `mem2`=[[reg+a]+b] · `{"kind":"caller","return_off":off,"cases":[{"ret_rva","src"}]}`=[rsp+off] 의 반환주소(콜사이트 RVA)로 경우를 골라 그 Src · `CachePlayer`/`CacheSlot` = 캐시에서 Player 를 찾는 형식. 모두 창 시점 rsp 기준이다(프레임 크기가 바뀌면 스택 변위가 바뀐다).

## 할 일 — 배정된 전술의 **모든 창**에 대해
1. **위치**: 신 후보가 맞는가(같은 비교·같은 의미의 값). 없거나 틀리면 신 owner 안(또는 다른 함수)에서 진짜 자리를 찾는다. 함수 분할·인라인·아웃라인·사본 복제 여부도 본다. SITE_MISSING·OWNER_MISSING 은 특히 — 같은 판단이 다른 함수로 옮겨갔는지 콜그래프·Location 행으로 끝까지 찾는다.
2. **의미**: 비교 대상 값·소비 분기의 taken/not_taken 이 0.7.1 기록과 같은가. 창 주변 소스가 실제로 바뀌었다면 **무엇이** 바뀌었는지(조건 추가/삭제·비교 대상 교체·이동·삭제).
3. **주체 Src·team_chk**: 신 코드에서 같은 대상을 가리키는지. ★**호출자 프레임을 읽는 Src**(`stackmem`·`caller` 하위·rbp 없는 owner 의 `mem:rbp`)는 추적기가 AUTO 로 통과시키는 사각지대다 — 호출자 슬롯이 그대로인지 직접 본다(0.7.0 회귀 3건 원인). Caller 는 ret_rva 전부·return_off(=프롤로그 push 수×8 + sub) 전부를 신 값으로 다시 계산하고, 호출처 수(콜사이트 전수)가 달라졌으면 새 호출처를 보고한다.
4. **dynamic_hex**: 창 명령의 레지스터·형태·변위가 바뀌었으면 다시 쓴다(다른 레지스터 오염 없는지 포함).
5. ★**calc 콜백 입력**(`calc_table.md`): 그 전술의 `Compute` 콜백이 읽는 레지스터·프레임 슬롯이 신 창 시점에도 같은 값인가. **창이 SAME 이어도 반드시 본다**(콜백 입력은 targets·추적기 어디에도 없다 · 0.7.1 회차에 11개를 고쳤다 = `23_0.7.1_마이그.md` §2). 틀리면 새 입력을 제시한다. `+0x12f8` 등 구조체 변위도 신 코드에서 같은 필드인지 확인한다.
6. **AUTO 전술**(추적기 전 창 SAME)도 배정돼 있으면 3·5항만 확인한다(창 재조사 불요). 문제없으면 grade=AUTO.
7. ★**사양 변화 평가**(유저가 궁금해하는 것): 그 전술이 게임 안에서 하는 일이 0.7.1 → 0.7.2 에서 달라졌는가. 창 안뿐 아니라 그 판단 함수의 **창 밖 로직**이 바뀌어 체감 효과가 달라지는지도 본다(aidiff 의 EDITED 행·상수 변화). 행번호만 밀린 것은 변화가 아니다.
   - `spec` = `NONE`(변화 없음) / `MINOR`(주변 변화, 효과 거의 같음) / `MAJOR`(전술의 효과·범위가 실제로 달라짐 — 전술 설명 수정이나 유저 판단 필요) / `DEAD`(더 쓸 수 없음 — 판단 삭제 등).
   - `spec_note` = 게임을 아는 사람에게 쉬운 말 한두 문장. 추정이면 「추정」이라고 쓴다.
8. ★**처리 제안**(이번 회차 신설 · 유저 지시 「로직 변경된 거 확인해서 추가 수정 및 삭제 등 고려」): 전술마다 하나 —
   - `PORT` 그대로 이식(의미·이름·범위 그대로) · `REDEFINE` 의미 수정(이름·설명·범위·기본값을 바꿔야 함 — 무엇을 어떻게) · `REDESIGN` 새 지점으로 재설계(같은 의도를 0.7.2 의 다른 판단에서 구현 — 후보 지점 제시) · `DISABLE` 삭제·비활성(왜 더 의미가 없는지).
   - PORT 가 아니면 `user_text` 에 유저에게 보여 줄 한 문장(쉬운 말 · 바꾸는 이유와 바뀐 뒤 효과).
9. **새 경로·새 전술 후보**: 0.7.2 가 같은 판단을 새 자리에서 부르면(새 호출처·새 사본) `new_paths` 에 적는다. 0.7.2 에서 **새로 생긴 판단 상수**(새 조건·새 문턱)가 네 함수들 안에 있고 전술로 노출할 가치가 있으면 `new_candidates` 에 적는다(위치·비교·의미·기본값 · 값을 넣을지는 메인이 유저 승인 뒤 정한다).

창 판정 어휘(하나만):
- `EQUIV` — 같은 의미 · 추적기 제안(rva/src/team_chk/dyn) 그대로 써도 됨
- `EQUIV_FIX` — 같은 의미 · 단 rva/src/team_chk/dynamic/win/calc 중 고칠 것이 있음(정확한 신 값 제시)
- `CHANGED` — 창은 있으나 게임 로직이 바뀌어 전술 의미가 달라짐(무엇이 어떻게 · 대응 제안: 재정의/유지/비활성)
- `MOVED` — 로직이 다른 곳으로 옮겨감(신 위치·형태 제시 — 가능하면 fix 에 신 창 전 필드)
- `GONE` — 해당 로직이 0.7.2 에서 사라짐(근거 제시)
- `UNSURE` — 정적으로 확정 불가(무엇이 막는지 · 런타임으로 무엇을 보면 되는지)

## 규칙
- **읽기 전용**: 게임 실행 금지 · targets/src/sites.rs/매니페스트/기준선 수정 금지. 쓰는 파일은 아래 결과 파일 2개 + 네 스크래치 폴더뿐.
- 모든 주장에 근거(구/신 RVA + 명령 인용, 또는 LOC 행). 근거 없는 것은 「추정」이라고 쓴다.
- 판정에는 적용 범위를 쓴다("이 창은 …라서 CHANGED" — "불가능" 같은 단정 금지).
- 지점 교차: 창이 들어갈 바이트 길이(≥5 · E9 rel32)·분기 유입이 안전한지(창 안으로 점프 유입 없음, call/ret/RIP 상대 명령을 창이 품지 않음)는 `gen_sites.py` 가 검증하지만, 후보를 고를 때 미리 본다. 창을 다시 잡을 땐 0.7.1 창과 같은 규칙으로 잡는다.

## 결과 파일 (둘 다 Write 도구로 · BOM 없는 UTF-8)
**`F:\claude\tfm2mods\MIG\_patchday\0.7.2\detail_tactics\re\<묶음>.md`** — 맨 위 전술별 요약표(전술·창 수·창 판정 목록·등급·spec·처리 제안), 그 아래 창별 상세(근거 명령 인용), 끝에 새 전술 후보.

**`...\re\<묶음>.json`** — 메인이 승격 스크립트에 그대로 쓰는 기계용 판정:
```json
{"tactics": {"<tactic_id>": {
  "grade": "AUTO|A|B|C|D",
  "spec": "NONE|MINOR|MAJOR|DEAD",
  "spec_note": "쉬운 말",
  "proposal": {"action": "PORT|REDEFINE|REDESIGN|DISABLE", "why": "근거 요지", "user_text": "PORT 가 아니면 유저용 한 문장"},
  "windows": {"<track.json 창 이름>": {
    "verdict": "EQUIV|EQUIV_FIX|CHANGED|MOVED|GONE|UNSURE",
    "fix": {}
  }},
  "calc_fix": [{"fn": "gank_deadline", "file": "extra_lane.rs", "old": "Mem(5,0xb8)", "new": "Mem(5,0xc8)", "why": "근거"}],
  "new_paths": ["ret 0x...: 설명"],
  "new_candidates": [{"where": "0x...", "asm": "cmp ..., N", "meaning": "쉬운 말", "default": "N 의 뜻", "why": "노출 가치"}]
}}}
```
- 등급: `AUTO` 추적기 자동확정 그대로 / `A` 추적기 제안 그대로 승격 / `B` 값 고친 뒤 승격 / `C` 재설계 필요 / `D` 이번 버전 비활성 권고.
- `fix` 는 **추적기 제안과 달라지는 필드만** 쓴다(EQUIV 면 빈 객체). 쓸 수 있는 키 = 초안 스키마 그대로: 노브 창 `rva`·`asm`·`imm_at`·`src`·`team_chk`·`patch_start`·`prefix_hex` · extra 창 `rva`·`win_hex`·`first_len`·`first_asm`·`src`·`team_chk`·`dynamic_hex`·`function`·`tf`. 값 표기도 초안과 같게(rva 는 `"0x..."` 문자열, hex 는 공백 구분, src 는 위 표기 문자열 또는 caller 객체). SITE_MISSING 창처럼 추적기 제안이 비어 있으면 승격에 필요한 필드를 **전부** 쓴다.
- 창 이름은 `track.json` 의 `windows[].name`(구 RVA 가 든 이름)을 그대로 쓴다. 배정 전술의 창이 하나도 빠지면 안 된다(AUTO 전술은 창 판정 생략 가능).
- JSON 은 쓴 뒤 `python -X utf8 -c "import json;json.load(open(r'<경로>',encoding='utf-8'))"` 로 파싱을 확인한다.

## 최종 응답
**요약표 + 눈에 띄는 발견 3~5줄 + PORT 가 아닌 처리 제안 목록 + 새 전술 후보**만(상세는 파일에). 한국어.
