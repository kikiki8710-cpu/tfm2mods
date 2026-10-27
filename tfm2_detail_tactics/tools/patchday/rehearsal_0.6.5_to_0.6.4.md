# 리허설 채점 — 0.6.5 → 0.6.4 역방향 (검증 A, 2026-10-27)

`tools/track_update.py` 를 반증하려는 입장에서 돌린 결과다. 0.6.4→0.6.5 는 전면 재링크였으므로, 거꾸로(구=0.6.5, 신=0.6.4) 돌리면
사람이 확정한 정답지로 채점할 수 있다.

- 구 exe = `F:\claude\tfm2\tfm2_0.6.5\TeamfightManager2.exe` (sha256 `DAF040DB…`) · 신 exe = `F:\claude\tfm2\tfm2_0.6.4\TeamfightManager2.exe` (`A1D05045…`)
- 기준선 = `patchday/baseline_0.6.5.json` (재생성 안 함 — snapshot 형식 불변)
- 최종 판정에 쓴 추적기 = 이 문서와 같은 날 수정한 `track_update.py` (sha256 `265EF72D…`, 2263줄). 수정 내용은 §4
- 산출물 = `F:\claude\tfm2mods\MIG\_patchday\_tmp\verifyA\` (`rehearse_final\` · `selftest_final\` · `synth2\` · 채점 스크립트)

## 0. 결론

| 항목 | 결과 | 판정 |
|---|---|---|
| 1. 생성기 불변 + review 가드 | `gen_sites --ver 0.6.5` 재실행 뒤 `src/` · `gen_sites_0.6.5.log.txt` git diff **0**. `_tmp` 사본 4곳(extra_ops 지점·subjects 지점·knobs 전술·extra_verified `review: null`)에 키를 넣으면 종료코드 1, `src` 쓰기 0, 로그 생성 0 | PASS |
| 2. 자체시험 0.6.5→0.6.5 | 전술 AUTO **100** · 창 SAME **218** · 초안 6파일 차이 **0** (수정 전후 모두) | PASS |
| 3(a). 자동확정 오답 | 자동확정 창 **97개 중 오답 0**. 정답지로 채점한 93개 모두 맞음. 정답지 없는 4개는 (e)에서 따로 확인했고 모두 맞음 | PASS |
| 3(b). 레지스터 개명 | 정답지의 개명 지점(r13↔rdi · r12↔r13 · r14↔r15)은 모두 REVIEW로 잡혔고 레지스터 대응도 맞음. CSV가 `reg`/`disp`로 기록한 16개 창도 모두 REVIEW이고 명령문이 16/16 일치. Src 슬롯 제안 1건이 틀렸던 것을 고침(§4 D3) | PASS (수정 후) |
| 3(c). 사본 1→3 | 상세전술에는 해당 전술이 없음. 하네스로 시험함. 정방향은 `COPIES 1→3`으로 사본 3곳을 정확히 나열하고, 역방향은 3개 모두 REVIEW + `COPIES 3→1`. 수정 전에는 REVIEW이긴 했지만 COPIES 사유가 없었음 | PASS (수정 후) |
| 3(d). 후반 푸시 문턱 130 | 상세전술 `extra_c022` `push_start_base`가 바로 그 지점이다(0.6.5 `0xfb37c7` → 0.6.4 `0xf24fab`). **LOGIC_CHANGED**로 잡히고 사유는 소비 명령 `cmp [rsp+0x88],eax` ↔ `sub ecx,eax`(감산항 삭제). 하네스 정방향: `0xf24fab` LOGIC_CHANGED, `0xf24fcc` SITE_MISSING | PASS |
| 3(e). SAME 표본 15 독립 확인 | 창 명령 15/15 같음 · owner가 fnmap과 15/15 일치 · CSV 11/11 일치. `repin resolve`는 13/15 일치하고, 나머지 2건은 repin의 「동일오프셋 가정」이 틀렸음을 디스어셈으로 확인 | PASS |
| 3(f). VALUE_CHANGED 누수 | 코드 리뷰상 즉치가 다른데 SAME이 되는 경로는 없음. 합성 exe 경계 사례 7건이 모두 REVIEW. 그중 **2건은 수정 전 코드였다면 SAME/AUTO로 빠져나갔을 결함**(§4 D1·D2) | PASS (수정 후) |

**자동확정 오답 = 0** (수정 전과 후 모두). 자동확정률은 창 기준 97/218(44.5%), 전술 기준 47/100이다. 수정 뒤에도 자동확정 수는 변하지 않았다.

## 1. 리허설 판정 분포 (최종)

`track` 종료코드 1(검토 항목 있음 = 정상) · 실행 약 56초.

| 판정 | 창 수 | 자동확정 |
|---|---|---|
| SAME | 97 | 97 |
| REG_RENAME | 55 (수정 전 57) | 0 |
| LOGIC_CHANGED | 62 (수정 전 60) | 0 |
| COPIES | 1 | 0 |
| SITE_MISSING | 1 | 0 |
| OWNER_MISSING | 2 | 0 |
| 전술 | AUTO 47 · REVIEW 53 | |

수정 전후로 판정이 바뀐 창은 3개다.
- `extra_c023@0xfb3dac`, `extra_c028@0xdd26ab`: REG_RENAME → LOGIC_CHANGED. 창 밖 소비 Jcc의 대상이 정렬되지 않았다는 새 검사가 걸렸다.
- `lane_retreat_final_50@0xf0a51e`: Src 제안이 `mem:rbp:0x118` → `mem:rbp:0x110`으로 바뀌었다(§2 b).

## 2. 채점 상세

### 정답지
| 출처 | 성격 | 창 수 |
|---|---|---|
| knobs `key`(0.6.4) | 사람 확정(tactics_verify 이전) | 53 (기본32 전부) |
| `overrides_0.6.4_0.6.5.json` | 사람 판정 15지점 | 3 (그중 상세전술 해당분) |
| `probe_targets_0.6.5.json` `migrated_from` / `key`(key_source = sites_064_065.csv) | 사람 확정 / 반자동 | 기본32와 겹치는 것 제외 16 |
| `sites_064_065.csv` (`tfm2_ai_analysis_verify/migration_065`, site_migrate.py) | **자동**(다른 파이프라인: aidiff 교차교정 + 지점 대응) | 124 |
| 없음 | — | 25 |

정답지끼리 충돌은 0건이다(knobs·overrides·probe·CSV가 서로 일치).
⚠ CSV와 `fnmap_global_065.json`은 지금 `F:\claude\tfm2\.claude\worktrees\ingame-verification-mode-dd9a33\mods_report\tfm2_ai_analysis_verify\migration_065\`에만 있다. 메인 체크아웃에는 없다.

### (a) 자동확정 오답
- AUTO 97 = 정답지 일치 93 + 정답지 없음 4(`extra_c031@0xea66ca`·`@0xea6722`, `extra_c050@0xe54293`·`@0xe5429b` → (e)에서 확인, 모두 맞음). **오답 0.**
- AUTO 97개 owner는 모두 fnmap_global_065(atlas_migrate 산출)와 일치한다.
- CSV가 있는 185개 창 전부에서, 추적기가 고른 0.6.4 값 명령의 문자열이 CSV `insn_old`와 정확히 같다(185/185).
- REVIEW 121개 창: 제안 RVA가 맞음 99 · 정답지 없음 21 · 제안 없음 1.
  - 제안 없는 1건은 `extra_c035@0xe92ac1` SITE_MISSING이다. 0.6.4에서는 `add rax,rax`와 `cmp rax,[r10+0x668]` 사이에 `mov r10,[rbp+0x58]`이 끼어 있어 2명령 창을 그대로 놓을 수 없다.
  - 수정 후 SITE_MISSING 사유에 값 조각 후보 5곳을 나열하며, 정답 `0xebb1ff`가 그 안에 있다.

### (b) 레지스터 개명
| 지점(0.6.5→0.6.4) | 정답(정방향 기록) | 추적기 | 창 레지스터 제안 |
|---|---|---|---|
| `lane_retreat_final_50@0xf0a51e` → `0xfc57bc` | r13→rdi (`cmp r13,0x32`→`cmp rdi,0x32`) | REG_RENAME | rdi→r13 ✔ (+ `setge r15b`→`setge bl`) |
| `lane_retreat_final_50@0xf9603d` → `0xf07f93` | r12→r13 | LOGIC_CHANGED(owner EDITED + 주변 변화) | r13→r12 ✔ |
| (하네스) `0xe035d3` → `0xff952e` | r14→r15 | LOGIC_CHANGED | r15→r14 ✔ |
| (하네스) `0xe0361c` → `0xff95b2` | r14→r15 | LOGIC_CHANGED | r15→r14 ✔ |

- CSV `reg` 15개와 `disp` 1개(`extra_c020`·`c028`·`c032`·`c052`·`c053`·`n012` 등)는 모두 REVIEW이고, 신 명령문이 CSV와 16/16 일치한다. AUTO로 샌 것은 0이다.
- 하네스 기준으로 창 첫 명령의 레지스터 개명은 정방향 7·역방향 8개이고, AUTO로 샌 것은 0이다.
- **Src 슬롯 제안(주체 위치)**
  - `0xf0a51e`의 src `mem:rbp:0x1a0`은 진입 인자 r9를 저장하는 슬롯이다(0.6.5 `0xf08f68 mov [rbp+0x1a0], r9`).
  - 수정 전 제안 `mem:rbp:0x118`은 **틀렸다**. 0.6.4 `[rbp+0x118]`은 `rdx`(`[r9+0x890]`에서 읽은 dword)를 저장하는 슬롯으로, `cmp dword [rbp+0x118],1`로 쓰인다.
  - 0.6.4에서 r9가 저장되는 곳은 `0xfc448a mov [rbp+0x110], r9`다. 수정 후 제안은 `mem:rbp:0x110`(3표)이다.
  - `0xf9603d`의 제안 src 0x270→0x260(10표), team_chk 0x2a0→0x298(24표)은 사용 패턴(`imul …,0xae0/0xac8`, `cmp …,1`/`0`)이 짝을 이룬다. 수동 대조로는 맞지만 **런타임 미검증**이다.

### (c) 오브젝트 사냥 후퇴 30% 사본 1→3
상세전술 창 218개에 `objective_hunt_*` 지점은 없다(`tools/targets`·`src`에서 `e035cb|e03612|e0361c|ff95b2` grep 0건). 그래서 tactics_verify 정답 지점 119곳을 임시 창으로 만들어
`Tracker.window`에 넣는 하네스(`_tmp\verifyA\harness_tv.py`)로 양방향 시험을 했다.

| 방향 | 결과 |
|---|---|
| 정방향 0.6.4→0.6.5 | `0xff95b2` → `0xe0361c`, LOGIC_CHANGED. 사유에 **`COPIES: 값 조각 사본 1→3개 (… 위치 ['0xe035cb','0xe03612','0xe0361c'])`** — 정답 3곳과 정확히 같다. 수정 전에는 COPIES 사유가 없었다(창 키 `cmp;jb`와 `cmp;jae`가 서로 달라서) |
| 역방향 0.6.5→0.6.4 | `0xe0361c` → `0xff95b2` ✔. `0xe035cb`·`0xe03612` → `0xff9386`(`cmp rcx,2`)은 ✘이지만 둘 다 REVIEW + `COPIES 3→1`이고, `참고: … 구 조각과 같은 명령이 신 owner ['0xff95b2'] 에 있음` + VALUE_CHANGED `[정렬 의심]` 표시가 붙는다 |
| 상세전술 안 유사 사례 | `extra_c035@0xe96af2` COPIES(같은 꼴 사본 3→2) |

하네스 전체 결과(119지점): 정방향 OK/AUTO 90 · OK/REVIEW 24 · 비활성 정답(null) 2개 모두 비-SAME · 사본 분기 1 · 오답 제안(REVIEW) 2. 역방향 OK/AUTO 90 · OK/REVIEW 25 · 오답 제안(REVIEW) 4. **AUTO 오답 0.**
오답 제안은 모두 `objective_hunt_danger_mult_3`/`exit_hp_30`(owner `0xff9370`↔`0xe03370`대)에 있다. 같은 `lea r?,[rax+rax*2]`가 3개 있고 컴파일러가 비교를 복제한 함수다.

### (d) 후반 푸시 문턱 130
- 상세전술 `extra_c022` `push_start_base`(「푸시 개시 기본 점수」) 창 `0xfb37c7 add eax, 0x82`가 0.6.4 `0xf24fab add ecx, 0x82`로 대응한다. 이 0.6.4 지점은 tactics_verify `late_push_threshold_130`의 지점이다.
- 판정은 **LOGIC_CHANGED**이고 사유는 두 가지다: 소비 명령 변화(`cmp dword ptr [rsp+0x88], eax` ↔ `sub ecx, eax`), 그리고 ±8 비교 구조 변화.
- 정답지(overrides `0xf24fab: null` — 0.6.5 로직변경 「감산항 삭제」)와 같은 판단이다.
- 하네스 정방향: `0xf24fab` → `0xfb37c7` LOGIC_CHANGED, `0xf24fcc`(cmp r10d,ecx) → SITE_MISSING.

### (e) SAME 표본 15건 독립 재확인 (`_tmp\verifyA\e_check.py`)
쓴 방법은 다섯 가지다.
1. `MIG\repin.py resolve`를 별도 프로세스로 실행.
2. 자체 capstone 정규화로 창과 ±16 명령 대조.
3. fnmap_global_065로 owner 대조.
4. CSV 대조.
5. Src 성분: 레지스터의 마지막 정의 명령 비교, Caller는 신 ret 앞 call이 신 owner를 부르는지와 콜사이트 ±12 확인.

| 표본 | 0.6.5 → 0.6.4 | 창 | 주변 ±16 | repin | 비고 |
|---|---|---|---|---|---|
| c031@0xea66ca · @0xea6722 | → 0xed693a · 0xed6992 | 같음 | 32/32 | 일치 | 정답지 없음. rdi 정의 `mov rdi,[rsp+0xd0]` 같음 |
| c050@0xe54293 · @0xe5429b | → 0xdd7033 · 0xdd703b | 같음 | 32/32 | 일치 | 정답지 없음 |
| c010@0xe4d559 | → 0xe37c68 | 같음 | 27/32 | **0xe37bf9** | repin = 「동일오프셋 가정」. 0xe37bf9는 `lea rax,[rcx+rax*8]`(다른 명령). 추적기·CSV가 맞음 |
| c051@0xfd109b | → 0xf8bdb9 | 같음 | 26/32 | **0xf8bdcb** | repin = 「동일오프셋 가정」. 0xf8bdcb는 `imul r9,r9`. 추적기·CSV가 맞음 |
| c027@0xecad63 · c045@0x100c69f · n010@0xe78267 | → 0xfb94b1 · 0xee75bf · 0xdff6f7 | 같음 | 22·17·32/32 | 일치 | repin MID 계열 |
| n004@0xd3dae1 · d001@0x100a769 · c030@0xf16ee9 · n001@0xf3223f | → 0xd3d1c1 · 0xee57b9 · 0xfcca19 · 0xe4b43f | 같음 | 32/32 | 일치 | Caller src: 신 ret 앞 call → 신 owner ✔, 콜사이트 ±12 24/24 |
| c058@0xfe3446 | → 0xf98d96 | 같음 | 31/32 | 일치 | cache_slot src. rsi·r12의 정의가 `[r15+0x820]`→`[r15+0x800]`, `+0x8b0`→`+0x890`로 **필드 오프셋만 다름**(Player 구조체 0x20 이동 — `0xf08f09`/`0xfc42fd`에서도 같은 이동). 레지스터는 같으므로 주체는 같음 |
| c040@0xf85263 | → 0xe9bf33 | 같음 | 32/32 | 일치 | |

15/15 맞음. 함수 전체가 정규화 기준으로 같은 표본은 0이다(재링크 코드젠 차이).

### (f) VALUE_CHANGED 자동확정 금지 — 코드 리뷰와 합성 시험
코드 리뷰(`Tracker.window` · `_finish`)
- 창 안 모든 명령(조각 밖 보조 명령 포함)을 쌍으로 비교한다. 꼴 다름 → LOGIC, 즉치 다름 → VALUE_CHANGED, 레지스터/메모리 기준 → REG, 스택·필드 변위 → REG, 바이트만 다름 → ENC(REG)다. 건너뛰는 것은 rel 분기 명령의 대상 변위뿐이고, 그 대상은 별도 「창 안 분기 대상」 검사로 본다.
- Map 토글(Jcc 첫 바이트)은 `values` 루프에서 따로 비교한다. Tf::Compute 창은 역산은 못 하지만, 즉치 차이는 위 쌍 비교로 VALUE_CHANGED가 된다.
- `_finish`: `code = PRIORITY 첫 flag`이고, VALUE_CHANGED는 PRIORITY에 있으므로 `auto`가 될 수 없다. 전술 AUTO는 소속 창이 전부 auto일 때만이다.

합성 exe 2(`_tmp\verifyA\synth2\`, 0.6.5 사본에 길이 불변 바이트 패치 7건)

| # | 변이 | 수정 후 | 수정 전 코드(새 검사 사유 제거로 재현) |
|---|---|---|---|
| M1 | `line_chase_target_guard_hp_65` 창 밖 소비 `ja` 대상 +2 | LOGIC_CHANGED / REVIEW | **SAME / AUTO** ← 결함 |
| M2 | `combat_retreat_hp_35` owner 먼 곳에 값 조각 `cmp rax,0x23` 사본 추가 | COPIES / REVIEW | **SAME / AUTO** ← 결함 |
| M3 | `minion_damage_ratio_49` 조각 밖 보조 즉치 `mov eax,0→1` | VALUE_CHANGED | VALUE_CHANGED |
| M4 | `bot_duo_partner_required` 토글 `ja`→`jmp`(off 바이트) | LOGIC + VALUE 기본값 1→0 | 같음 |
| M5 | `extra_c004` Tf::Compute 창 `cmp rax,0x23→0x24` | COPIES(5→1)+VALUE | 같음 |
| M6 | `jg_gank_hp_start_50` `cmp qword [rbp+0x138],0x32→0x33` | VALUE_CHANGED 50→51 | 같음 |
| M7 | `extra_c002` `lea ecx,[r10+rdx+0x32]→+0x3c` | REG_RENAME(「구조체 필드 변위」) | 같음 |

변이 대상 밖에서 REVIEW가 된 창은 7개다. 같은 전술의 다른 사본(c004 5→4 COPIES)과, M3 변이가 ±24 문맥에 들어간 같은 owner 창 3개(호출자 상속 rbp 근거 부족)다. 모두 보수적 방향이다.

## 3. 오답·미확인 목록

- **자동확정 오답: 없음.**
- REVIEW 제안이 틀린 것(사람이 고칠 대상이고 자동확정 아님):
  - 리허설: `extra_c035@0xe92ac1`(제안 없음, 후보 5곳 안에 정답).
  - 하네스: 역방향 `0xe03492`·`0xe035cb`·`0xe03608`·`0xe03612`, 정방향 `0xff951f`·`0xff955e` — 전부 objective_hunt 함수.
- 정답지 없는 REVIEW 21창은 제안 RVA가 **미검증**이다: c007 ×2(OWNER_MISSING — fnmap에도 대응이 없다. 0.6.5 신규 함수로 **추정**, 미검증), c013 ×5, c020 ×2, c022, c028, c033, c035@0xe96af2, c038, c039, c041, c042 ×2, c045 ×2, n005.

## 4. 이번 검증에서 고친 결함 (`track_update.py`만 수정)

| # | 결함 | 영향 | 수정 |
|---|---|---|---|
| D1 | 사본 수를 **창 전체 키**로만 셌다. 컴파일러가 비교를 복제하면서 뒤 분기 꼴을 바꾸면(`cmp;jb` 1 → `cmp;jae`×2+`cmp;jb`×1) 놓친다 | 나머지 창이 그대로면 **SAME/AUTO로 사본 2곳 누락**(M2로 재현) | 즉치를 담은 값 조각(창 첫 `frag_n` 명령)의 사본 수도 비교한다. 즉치 없는 조각(`add rax,rax` 등)은 흔해서 제외 |
| D2 | 창 **밖** 소비 Jcc의 분기 대상을 검사하지 않았다(꼴만 비교) | 같은 비교가 다른 블록으로 가도 **SAME/AUTO**(M1로 재현) | 소비 kind=`flags`이고 Jcc가 창 밖이면 `map_target` 대응을 검사한다. 불일치나 정렬 불가면 LOGIC_CHANGED |
| D3 | 스택 슬롯 대응이 꼴 정렬 표 1표로도 RENAMED 제안이 되었다. 인자 저장 슬롯이 다른 레지스터 저장 슬롯과 짝지어졌다 | `0xf0a51e` src 제안 오류(§2 b) | ① 진입 인자 저장(spill) 근거: 구 슬롯이 인자 X를 한 번만 저장하고 신 owner에 X를 저장하는 슬롯이 하나뿐이면 +3표. 다른 슬롯일 때만 적용해서 동일 판정을 새로 만들지 않는다 ② 구 쪽이 인자 X 저장인데 신 짝이 다른 레지스터를 저장하면 그 표를 버린다 ③ 다른 슬롯 1표뿐이면 UNKNOWN(「약한 근거」) |
| D4 | 정렬이 다른 사본을 짚으면 「게임 기본값 30 → 2」처럼 허위 기본값 변경이 보고된다 | 기획 오판 위험 | 정렬 위치의 값 조각이 구와 다르고 구 조각이 신 owner 다른 곳에 있으면 `참고:` 사유를 붙이고, 값 변화에 `suspect`(md 표에 ⚠)를 붙인다 |
| D5 | SITE_MISSING에 단서가 없었다 | 사람 작업량 | 값 조각과 같은 꼴·즉치 위치 후보를 나열한다(`detail.fragment_candidates`) |

판정 규칙은 느슨해지지 않았다. 모든 수정은 flag나 UNKNOWN을 더하는 방향이고, SAME 조건은 그대로다. 자체시험은 수정 후에도 100 AUTO / 218 SAME / 초안 차이 0이다.

## 5. 남은 한계

- owner **밖으로** 인라인된 사본은 여전히 세지 않는다. 값 조각 사본도 owner 안만 센다.
- Src 제안(RENAMED)은 정렬 다수결과 인자 저장 근거뿐이고 제어흐름 분석이 아니다. 프레임이 바뀐 함수의 슬롯 제안은 디스어셈이나 런타임 `game==mine`으로 확인해야 한다(이번에도 `0xf9603d`는 수동 대조만 함).
- `lea` 변위에 담긴 상수(M7)는 VALUE_CHANGED가 아니라 「구조체 필드 변위」(REG_RENAME)로 보고된다. 자동확정은 되지 않지만 이름표가 부정확하다.
- 창 한가운데 명령이 끼면(`extra_c035@0xe92ac1`) SITE_MISSING이 되고 자동 제안이 없다. 후보 나열만 한다.
- D2 검사는 소비 Jcc 대상이 정렬되지 않으면 LOGIC_CHANGED를 낸다. 이번 리허설에서는 원래 REVIEW이던 2창에만 걸렸지만, 큰 재배치가 있는 패치에서는 검토 항목이 늘 수 있다.
- tactics_verify 정답지로 대조한 것은 69창(knobs 53 + probe 16, 그중 probe `key` 10은 CSV 유래 반자동)뿐이다. 124창은 자동 파이프라인 CSV로만 대조했다(독립 파이프라인이 일치했다는 것은 방증이지 증명이 아니다). 25창은 정답지가 없다.
- 이 리허설은 **역방향 전면 재링크** 한 사례다. 내일 패치는 성격이 다를 수 있다(핫픽스면 SAME 비중이 훨씬 클 것으로 **추정**).
- `patchday/README.md`의 「검증 기록」 표는 이 수정을 반영하지 않았다(과업상 `track_update.py` 외 수정 금지 — 메인이 갱신할 것).

## 6. 재현

```
python -X utf8 F:\claude\tfm2mods\tfm2_detail_tactics\tools\track_update.py track --base F:\claude\tfm2mods\tfm2_detail_tactics\tools\patchday\baseline_0.6.5.json ^
  --old-exe F:\claude\tfm2\tfm2_0.6.5\TeamfightManager2.exe --new-exe F:\claude\tfm2\tfm2_0.6.4\TeamfightManager2.exe --new-ver 0.6.4 ^
  --old-pkl F:\claude\tfm2mods\_fnidx_065.pkl --new-pkl F:\claude\tfm2mods\_fnidx_064.pkl --old-cg F:\claude\tfm2mods\_cg_065.pkl --new-cg F:\claude\tfm2mods\_cg_064.pkl ^
  --out F:\claude\tfm2mods\MIG\_patchday\_tmp\verifyA\rehearse_final
python -X utf8 F:\claude\tfm2mods\MIG\_patchday\_tmp\verifyA\grade.py F:\claude\tfm2mods\MIG\_patchday\_tmp\verifyA\rehearse_final      (a) 채점
python -X utf8 F:\claude\tfm2mods\MIG\_patchday\_tmp\verifyA\e_check.py F:\claude\tfm2mods\MIG\_patchday\_tmp\verifyA\rehearse_final   (e) 표본
python -X utf8 F:\claude\tfm2mods\MIG\_patchday\_tmp\verifyA\harness_tv.py fwd|rev                                                       (b)(c)(d) 하네스
python -X utf8 F:\claude\tfm2mods\MIG\_patchday\_tmp\verifyA\synth2\make_synth2.py ; ...\synth2\run_synth2.py                              (f) 합성
python -X utf8 F:\claude\tfm2mods\MIG\_patchday\_tmp\verifyA\guard_test.py                                                                (1) review 가드
```
