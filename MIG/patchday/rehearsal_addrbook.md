# addrbook 검증·리허설 기록 (검증 B · 2026-10-27 · 기준 0.6.5)

> 대상: `MIG\patchday\addrbook.py`(전 주소록 구→신 대응) · `MIG\patchday\kb_harvest.py`(KB 문서 RVA 수확) · `addrbook_0.6.5.json`.
> 판정 기준 = **자동 확정(SAME/MOVED_BYTES_CHANGED) 행의 오답 0**. 규칙을 풀어서 통과시키지 않았다(남는 단일 증거 행은 그대로 검토로 둔다).
> 전 산출물·로그 = `MIG\_patchday\_tmp\addrbook_v\` (아래 경로는 이 폴더 기준).

## 0. 결론

| 항목 | 결과 | 근거 |
|---|---|---|
| addrbook.py | 미완성(580줄에서 끊김) → **완성**(build/map/grade · 문법·실행 확인) | 이 문서 §5 |
| KB 표본 정확도 | 값: **85/90 정답**(오류 5 = 비RVA 1 · 창 끝 표기 2 · leaf 종류 오분류 2) → 결함 5종 전부 kb_harvest 수정·재수확. 이름: 오류 16/90 → 수정 뒤 같은 표본에서 4/27(표본 B) | §1 |
| 주소록 | **24,120행**(매니페스트 499 고유값 · KB 23,621 · 겹침 430) · 기각 2 | §2 |
| 자체시험 0.6.5→0.6.5 | **24,117 SAME / 3 AMBIGUOUS · 신≠구 0행** — 남은 3행은 증거 1갈래뿐인 .rdata(점프표 안쪽·16B 타입ID·leaf vtable). 규칙상 채택 안 함(후보는 전부 자기 자신) | §3 |
| ★전진 리허설 ① 0.6.4→0.6.5 (정답 = `repin_map_065.final.json` 137→고유 133) | **자동 101 · 자동 오답 0 · 자동 대응률 75.9%** · 검토 32 중 후보 정답 31 · 사람판정 16건 중 5건 자동 확정(전부 정답) | §4.1 |
| ★전진 리허설 ② tactics_verify (`probe_targets_0.6.5` 54 · `overrides_0.6.4_0.6.5` 15) | 자동 오답 **0 / 0** · 사본 1→3 분기(0xff95b2) = AMBIGUOUS(COPIES) · 로직변경 비활성 2건 = 자동 아님 | §4.2 |
| 역방향 0.6.5→0.6.4 (주소록 전량) | **실행 오류 0** · 24,120행 · 자동 64.8% · 정답 역대응(final 133 · tactics_verify 56) **자동 오답 0 / 0** · KB 전용 표본 10건 독립 재확인 10/10 | §4.3 |
| mig_verify | MODS 에 `tfm2_detail_tactics` 있음(L38) · `check --exe 0.6.5` **13모드 전부 PASS · STALE 0 (exit 0)** · `coverage` **13모드 클린 (exit 0)** | `check_065.log`·`cov_065.log` |
| tactics_verify 매니페스트 | `.bak_20261027` 대비: 엔트리 45→108(삭제 0) · 남은 45개의 바뀐 필드 = `name`·`locs` 뿐 · `curated` 0→0 · `ignore` 0→0 · 최상위 키·notes 동일 ⟹ **큐레이션 손상 없음**(원래 큐레이션 엔트리가 없었다) | §6 |

## 1. KB 행 품질 표본 (`kb_rows_0.6.5.json`)

### 1.1 방법
`kb_sample.py <seed> all|md` — 무작위 explicit 20 + inferred 10. 행마다 원문(sources 파일:줄)과 **0.6.5 exe 디스어셈블(그 주소 명령) · 0.6.4 exe 같은 주소** 를 나란히 출력해 "원문이 말하는 그 명령이 0.6.5 에 있는가 / 0.6.4 에선 쓰레기인가" 로 판정했다.
- 표본 A(seed 20261027 · 전체 풀) · 표본 B(seed 777 · md 출처만 — 사람이 쓴 문서라 버전 귀속 오류가 나올 곳) · 표본 C(seed 4242 · 1차 수정 뒤 새 표본). 출력 = `sample_all.txt`·`sample_md.txt`·`sample_post.txt`.

### 1.2 결과 (값 = 0.6.5 주소이고 kind/owner 까지 맞음)
| 표본 | explicit | inferred | 값 오류 | 이름 오류(엉뚱한 문장 조각·다른 대상) |
|---|---|---|---|---|
| A | 20/20 | 10/10 | 0 (0xd5e89d 는 함수 범위 끝 — note `fn_end` 로 표시돼 있어 정상) | 3 (`f6bd74` 「C042의4개60 창은…」 · `e55ddd` · `f08f09`) |
| B | 20/20 | 7/10 | **3**: `0x16ca0` = 스택 슬롯 오프셋(「슬롯 0x16ca0·0x10」 — RVA 아님) · `0xfb2561`·`0xe98903` = md 코드펜스 JSON `"end": "0x…"`(창 끝 표기) | 12 (「결과는」「이후」「ESI는」「대상이 있으면」 · `"end": "` 등) |
| C | 19/20 | 9/10 | **2**: `0x17217d0`·`0xfe4d30` = vtable 대상(원문 `"start"`·`methods.0x18`)인데 .pdata 틈 앞 leaf 의 MID 로 오분류 | 1 (`out576의0`) |
| 계 | 59/60 | 26/30 | **5/90 (94.4% 정답)** | 16/90 |

- 이름 외 관찰: JSON 출처 행 이름은 대부분 키 경로(`direct_edges.source`·`rva`)다 — 틀린 건 아니지만 역할명은 아니다(표본 90 중 64). 사람이 붙인 이름을 보려면 `src_types` 에 md 가 있는 행만 쓴다.

### 1.3 kb_harvest.py 수정 (결함 → 규칙 · KB 원문은 안 건드림)
| # | 결함 | 수정 | 재수확 영향 |
|---|---|---|---|
| 1 | 스택 슬롯 오프셋을 RVA 로 수확 | `OFFSET_WORD` 에 `슬롯·slot·disp·변위` | 아래 2와 합쳐 −65행 |
| 2 | md 코드펜스 JSON `"end": "0x…"`(창/범위 끝) 수확 — JSON 파일은 `JSON_END_KEYS` 로 이미 빼던 것 | `MD_JSON_END` 를 `RANGE_BEFORE` 와 같이 적용 | (64행 = 전부 inferred MID) |
| 3 | .pdata 틈의 leaf 가 여럿일 때 뒤 leaf 를 앞 leaf 의 MID 로 분류 · CC 패딩 안 주소를 명령 경계로 인정 | `Exe.classify` nopdata 갈래: 종결(ret/jmp) 뒤 int3(CC) 패딩을 건너뛴 첫 바이트 = 새 leaf | 263행 정정(MID→FN_START 213 · owner 정정 50) · 패딩 안 24건 기각(`observed_end` 같은 범위 끝) |
| 4 | 「29166B」(바이트 수) 를 맨 hex 0x29166b 로 수확 | 맨 hex 가 `\d+[bB]` 면 제외 | −10행 |
| 5 | `rva32_file_offsets`(**파일 오프셋**) 를 RVA 로 수확 — 자체시험에서 G 증거만 남은 .rdata 행을 추적하다 발견 | `JSON_SKIP_KEYS` 에 `file_?off·raw_?off·fileoff` | −10행 · 기각 −26 |
| 6 | 이름: 조사로 끝나는 짧은 문장 머리·다음 문장 조각 | `WEAK_NAME`(조사 끝 짧은 조각 → 버림) · 오른쪽 조각은 표 칸이거나 24자 이하 비문장일 때만 · 못 찾으면 `§<제목>` · md JSON 키(`"anchor": "`) → 키 이름 | 표본 B 이름 오류 12 → 4 (같은 행 재판정) |
- 재수확: 행 24,162 → **24,053** · 기각 269 → 235 · 매니페스트 겹침 430 그대로(정상 행 손실 없음 — 줄어든 109행은 전부 inferred/비매니페스트, 1~5 결함 유형). 실행 16~18s. 원본 = `kb_rows_0.6.5.orig.json` · `kb_harvest.py.orig`.
- ⚠수정 후 새 표본(C)은 3번 수정 전이었다. 3·4·5 뒤의 독립 새 표본은 미실시(같은 행 재판정만).

## 2. 주소록 `addrbook_0.6.5.json`
```
python -X utf8 MIG\patchday\addrbook.py build --ver 0.6.5 --exe F:\claude\tfm2\tfm2_0.6.5\TeamfightManager2.exe --pkl F:\claude\tfm2mods\_fnidx_065.pkl --cg F:\claude\tfm2mods\_cg_065.pkl --extra MIG\patchday\kb_rows_0.6.5.json --out MIG\patchday\addrbook_0.6.5.json
```
- 15s · **24,120행** · 기각 2(`0xe562fa` 패딩 · `0x5019800` .rdata 원시 끝 = 섹션 밖) · 매니페스트 12B 불일치 0.
- 출처: KB 만 23,621 · 매니페스트 만 69 · 둘 다 430 (매니페스트 입력 566 = 고유값 499 · 모드 10개: detail_tactics 321 · tactics_verify 108 · comptest 72 · view_plus 28 · serpen 18 · ui_kit 11 · pos_lock 4 · level_cap 2 · lane 1 · champion_exclude 1).
- kind: MID 18,512 · FN_START 4,399(.pdata 없는 leaf 465 포함) · FN_END 220(.pdata 끝 상수) · RDATA 988 · DATA 1 · 명령 중간 지점(즉치·변위 바이트) 28 · 반환주소(RETURN_SITE) 66 · 「함수 시작이자 앞 함수 끝」 308.
- 신뢰 등급(`conf`): manifest 499 · KB explicit 7,619 · KB inferred 16,002.

## 3. 자체시험 0.6.5 → 0.6.5 (`self_full\`)
- **24,117 SAME · 3 AMBIGUOUS · 행 오류 0 · 신≠구 0행** · 455s (Location 색인·xref 색인 포함).
- 남은 3행(전부 .rdata · 후보 = 자기 자신 · 증거 1갈래):
  - `0x3b4655c` 상대 점프표 안쪽(G 12B 약) · `0x3bf23a0` 16B 타입ID 해시(G 20B — 코드 참조 0) · `0x3d54ba0` leaf/클론 함수만 든 vtable(Y 앵커 1갈래).
  - 「G(바이트 유일)만으로 채택 금지」(README 0.6.0) 규칙 그대로. 풀지 않았다.
- 이 시험으로 고친 엔진 결함(처음 400행 스모크: SAME 321 → 400): 
  ① owner 함수 증거 부족(작은·클론 함수 · caller 0) → **P**(전단사 caller 의 같은 서수 call) · **V**(vtable 슬롯 사영) · **Z**(전단사 앵커 블록 통째 이동) 추가
  ② repin `match_fn` 이 CG 판별 실패 시 skel 후보 목록을 버림 → 후보 집합 재구성(K)
  ③ leaf(.pdata 없음) 증거 → 앵커 블록 · 호출자 2곳 이상 사영 채택
  ④ .rdata: 정확 참조 우선 X · **Y**(4KB 안 코드 참조 앵커 + 내용 동일) · **SEC**(섹션 경계) · vtable 재투표(채택 대응 사영) — 1차 전량 SAME 23,954 → 24,112 → 24,117.

## 4. 리허설

### 4.1 전진 ① 0.6.4 → 0.6.5 · 정답 = `MIG\repin_map_065.final.json`
```
build --ver 0.6.4 --exe <0.6.4> --pkl _fnidx_064.pkl --cg _cg_064.pkl --from-repin-map MIG\repin_map_065.final.json --side old --out book_064_from_final.json   # 133행(137 중 구값 중복 4)
map --book book_064_from_final.json --old-exe <0.6.4> --new-exe <0.6.5> --new-ver 0.6.5 … --out fwd_final      # 36~42s
grade --addrmap fwd_final\addrmap.json --truth MIG\repin_map_065.final.json --out fwd_final\grade.json         # exit 0
```
| | 행 | 비고 |
|---|---|---|
| SAME | 89 | |
| MOVED_BYTES_CHANGED | 12 | change: next:reloc 5 · insn 4 · next:insn 4 · bytes 2 · next:imm 1 |
| FN_EDITED | 15 | 후보 15/15 정답 (owner 소스 변경 = 사람 확인) |
| AMBIGUOUS | 16 | 후보 16/16 정답 |
| MISSING | 1 | `A15E20_RVA`(EH 펑클릿 · owner 0x79a550 대응 실패 — 0.6.5 회차도 수동) |
- **자동 101 · 자동 오답 0 · 자동 대응률 75.9% · 후보 정답 132/133(99.2%).**
- 사람판정 16건(`manual`): 자동 확정 5(RPLY_RVA · RPLY_RESUME_RVA · SLOT_RVA · SERPEN_RVA · ui_kit RVA_UPDATE — 전부 정답) · 검토 11(후보 10 정답 · A15E20 없음). CT_REGION_HI(.pdata 끝 상수) = 「함수 시작 대응 실패 → 앞 함수 끝 해석」 후보 0x28cd18e(정답) + 검토 · ui_kit RVA_DROP(leaf) = 호출자 사영 후보 정답 + 검토 · RUNNER_VT_RVA = 포인터 표 후보 정답 + 검토 · serpen LAUNCHER_RET_A/B = 콜 서수 C 로 후보 정답(owner 미채택이라 검토).
- 비교: 기존 엔진(repin plan + sitealign `new_aligned` 우선)은 같은 137건에서 후보 오답 0 · 실패 7 이었지만 **자동 확정 표시가 없어** 사람이 전부(특히 DISAGREE·FAIL·16 수동) 봐야 했다. addrbook 은 101건을 근거 2갈래로 확정하고 나머지만 남긴다.

### 4.2 전진 ② tactics_verify 정답 (상세전술 기본 전술의 전신 지점)
```
build --ver 0.6.4 … --from-pairs tfm2_tactics_verify\tools\targets\probe_targets_0.6.5.json overrides_0.6.4_0.6.5.json --side old --out book_064_tv.json   # 56행
map … --out fwd_tv ; grade --truth probe_targets_0.6.5.json ; grade --truth overrides_0.6.4_0.6.5.json
```
| 정답지 | n | 자동 정답 | 자동 오답 | 검토(후보 정답) | 비고 |
|---|---|---|---|---|---|
| probe_targets migrated_from | 54 | 25 | **0** | 29 (28) | 자동 대응률 46.3% — FN_EDITED 18(0.6.5 에서 AI 함수 소스 변경) |
| overrides(사람 판정 15) | 15 | 2 | **0** | 11 (10) | 정답없음(로직변경 비활성) 2 = 자동 아님(AMBIGUOUS 1 · MISSING 1) |
- ★사본 1→3 분기 `0xff95b2`(`cmp r14,0x1e` → 신 `cmp r15,0x1e` 3곳): 처음엔 단일 확정(MOVED_BYTES_CHANGED) = **자동 오답 1** → 「구 owner 에서 유일한 즉치 보존·레지스터 지움 서명이 신 owner 에 여럿」 사본 검사 추가 → COPIES 충돌 = AMBIGUOUS. 바이트(A)·긴 n-gram 은 이 경우를 못 본다(레지스터가 바뀌고 주변이 재구성됨).
- 레지스터 개명(r13→rdi·r12→r13·r14→r15)은 change=`reg` 로 표시된다(예 `0xfc57bc` MOVED_BYTES_CHANGED `reg,next:stack`).

### 4.3 역방향 0.6.5 → 0.6.4 (주소록 전량 · `rev_064\`)
```
map --book MIG\patchday\addrbook_0.6.5.json --old-exe <0.6.5> --new-exe <0.6.4> --new-ver 0.6.4-rev --old-pkl _fnidx_065.pkl --new-pkl _fnidx_064.pkl --old-cg _cg_065.pkl --new-cg _cg_064.pkl --out rev_064
```
- **실행 오류 0** · 24,120행 · 550~960s(동시 부하에 따라) · SAME 11,900 · MOVED_BYTES_CHANGED 3,722 · FN_EDITED 1,719 · AMBIGUOUS 4,477 · MISSING 2,302 · 자동 64.8%.
- change(MOVED_BYTES_CHANGED · `next:` = 지점 다음 명령): reloc 1,053(+next 1,114) · stack 638(+505) · **imm 422(+28) = 기본값이 다른 지점 후보** · disp 213(+207) · reg 131(+205) · insn 6(+44) · 데이터 content 90.
- 매니페스트 499: SAME 287 · MOVED 46 · FN_EDITED 86 · AMBIGUOUS 63 · MISSING 17. 상세전술 321: SAME 167 · MOVED 32 · FN_EDITED 63 · AMBIGUOUS 47 · MISSING 12.
- 정답 역대응(0.6.5 값 → 0.6.4 정답): final 133 = 자동 정답 102 · **자동 오답 0** · 검토 31(후보 정답 29) / tactics_verify 56(사본 3곳 각각) = 자동 26 · **자동 오답 0** · 검토 29(26) · 주소록 밖 1.
- ⚠거꾸로 방향이라 실전 예측이 아니다(0.6.4↔0.6.5 사이 소스 변경·전면 재링크). 확인한 것은 배선·속도·**무표시 오답 0**.
- KB 전용 자동 행 표본 10건 독립 재확인(`rev_indep.py 1027` → `rev_indep_final.txt` · MID 6 · FN_START 3 · RDATA 1): 10건 모두 구·신 명령열/내용이 같은 지점(디스어셈 대조). `repin.py resolve` 와 일치 8 · 불일치 1(`0xd6149b`: repin = `0xd624bb`「동일오프셋 가정」→ 0.6.4 에서 명령 한복판 `add al,0; add bh,bh` · addrbook = `0xd6243d` 같은 `cmp dword [rbp+…],2; jb` 꼴 + 직전 `call 0xee2120↔0xf3d1d0` 서수 일치 — addrbook 이 맞음) · repin 실패 1(`0x3bf79b0` "params" 문자열 — 코드 참조 5함수가 0.6.4 `0x3baa18d` "params#asset…" 를 집음 · 앞 6B 동일 = 문자열 병합).

## 5. addrbook.py 완성 내용 (이번 과업에서 쓴 부분)
- 이어 쓴 곳: `sitemap` 나머지(U·B·C·O·사본 검사·명령 중간 지점 검사) · xref/포인터표/호출자 투표 · `change_of`(reloc/reg/imm/stack/disp/insn/encoding · 데이터는 content) · `map_row`(FN_START/FN_END/leaf/MID/RDATA/DATA) · `build`(매니페스트 + `--extra` + `--from-repin-map` + `--from-pairs`) · `map`(addrmap.json · addrmap.md · review.json · 종료코드 0/1/2) · `grade`(repin 맵 / probe_targets / overrides / {구:신} 자동 판별).
- CLI 는 docstring 계약 그대로 + `build --from-pairs`(리허설용) · `map --limit`(디버그) · `grade --out` 추가. run_patchday.py 9단계가 부르는 `map --book … --old-exe … --out <out>\addrmap` 와 맞는다(요약 파서는 `rows[].grade`·`summary` 를 읽는다).
- 성능: 전역 대응맵 (skel,size) 색인으로 init 단축 · 거대 함수 정렬 창 900명령 양자화 · 블록 검사 캐시. 24k 행 = 7.5~16분(Location·xref 색인 포함 · 동시 부하에 따라 · 캐시 = `MIG\_patchday\_tmp\_cache\`). run_patchday 9단계에 시간 제한은 없다(30초마다 진행 표시).

## 6. tactics_verify 매니페스트 diff (`MIG\manifest\tfm2_tactics_verify.json` vs `.bak_20261027`)
- 최상위 키(mod·game_ver·build·notes·offsets·entries) 값 동일 · 엔트리 45 → 108(삭제 0 · 추가 63) · 남은 45개에서 바뀐 필드 = `name` 45 · `locs` 45 뿐(`method`·`kind`·`ver`·`bytes` 동일) · `curated` 0 → 0 · `ignore` 0 → 0.
- 이름 변화는 개선: 옛 gen 이 `0xd6f935` 등을 근처 상수명 `BASELINE_EXE_SHA256` 로 잘못 붙였던 것이 `INLINE@src/sites.rs:16` 으로(v0.3.0 sites.rs 줄 이동 반영).

## 7. 남은 한계 (미검증 포함)
- 자체시험 3행 · 전진 검토 32행은 **규칙상** 사람 판정 몫이다(후보는 대부분 정답).
- KB 이름은 여전히 거칠다(JSON 행 = 키 경로). 표본 3회는 수정 전 데이터 기준이며, 수정 3~5 뒤 새 무작위 표본은 미실시.
- 0x 없는 맨 hex 수용 행(1,500여) 전수 검토 안 함(kb_harvest `--no-bare` 로 뺄 수 있음).
- addrmap.md 는 등급별 상위 400행(SAME 200행)만 표로 싣는다 — 전량은 addrmap.json.
- 패치날 순서(README ⑧ 보충): **새 버전 기준선 = `kb_harvest.py --ver <신> …` 먼저 → `addrbook.py build … --extra kb_rows_<신>.json`**. README ⑧-2 의 build 줄에는 `--extra` 가 빠져 있다(README 는 이 과업 수정 범위 밖 — 메인이 반영).
