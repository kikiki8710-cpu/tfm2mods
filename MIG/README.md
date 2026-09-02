# MIG — 매니페스트 구동 마이그레이션 (0.5.7 재설계, 2026-08-28)

> **마이그 = "매니페스트의 각 엔트리를 새 버전에서 찾는 것."**
> 소스 스캔 휴리스틱·기억·이력 문서에서 즉석 재구성하던 구 방식은 폐기
> (왜 = `RETRO-0.5.7.md` — 0.5.6→0.5.7 에서 stale 사고 10건 전부 사후 발견).

## 구성
```
MIG\
  README.md            이 문서 (절차)
  RETRO-0.5.7.md       재설계 배경 = 0.5.7 마이그 실패 회고
  ★run.py              ★단일 진입점 — 전 축을 순서대로 돌리고 종료코드로 막는다
  mig_verify.py        축① RVA: gen / check / coverage / dups / rebase  (+ mask_code 정본)
  ★offsets.py          축② 구조체 오프셋: snap / check / sources
  ★env.py              축③ 환경: mod_info·deps / 버전게이트 / 빌드SDK / apply누락 / stale dll
  ★bump_deps.py        deps 대역 일괄(.bak + 재파싱 검증) — 구 bump_deps_058.ps1 은 폐기
  ★repin.py            도구: plan / apply / rdata / resolve — STALE 엔트리 **자동 재핀 엔진**(0.5.8 신설)
  ★callgraph.py        도구: exe 전 함수의 call/jmp 대상 인덱스(repin 의 MULTI/NONE 판별 재료)
  manifest\<MOD>.json  ★모드별 버전 민감 지점 전수 목록 (기계 정본)
```
### 재핀 엔진 (repin.py, 2026-09-02 신설 — 0.5.8 에서 1,454 STALE 중 1,136 자동 해결)
```
python fnindex.py <신exe> _fnidx_<신버전>.pkl              # 함수 지문(skel/head/mnem)
python MIG\callgraph.py <exe> <fnidx.pkl> _cg_<버전>.pkl   # 콜그래프(구·신 각 1회, ~4분)
python MIG\repin.py plan  [MOD...] --old --new --oldpkl --newpkl --oldcg --newcg --out map.json
python MIG\repin.py apply [MOD...] --map map.json [--write --ver <신버전>]
python MIG\repin.py rdata --addrs 0x... --old ... --new ...  # .rdata 포인터 표(vtable) 전용
python MIG\repin.py resolve --addrs 0x... --old ... --new ... # 개별 구 RVA 직접 재핀(잔여 처리)
```
판별 단계(강한 것부터): ①채록 바이트 유일검색(부족하면 구 exe 에서 12→64B 연장)
②skeleton 지문 ③**콜그래프 사영**(UNIQUE 매칭 3.7만건을 기준맵으로 caller/callee 집합 대조)
④owner 함수 안에서 사이트 로컬 바이트 재탐색. ⑤ 그래도 남으면 **지역창**(주소순 이웃 중
매핑된 앞/뒤 함수 사이)으로 후보 축소 → 그래도 동형 클론이면 ghidra-re.
⚠`apply` 는 **주석·문자열을 길이보존 마스킹**(`mig_verify.mask_code`)한 위치에서만 치환한다 —
`strip_code`(길이 줄어듦) 오프셋으로 치환하면 소스가 깨진다(2026-09-02 실사고, 아래 절).
매니페스트 엔트리 = 이름 · 값 · 버전 · 종류 · 소스 위치 · **현행 exe 채록 바이트(12B)** · 재탐색 방법.
`offsets` = 구조체 오프셋 축(exe 대조 불가 — RE 로 검증), `notes` = 모드별 함정, `build` = 빌드 명령.

## ★★마이그는 축이 3개다 — 단일 진입점 `run.py` (2026-09-02 신설)
```
python MIG\run.py --exe <신exe> --pkl <신fnidx.pkl> --sdk sdk_<신> --ver <신>
```
| 축 | 도구 | 무엇을 보는가 | 이걸 안 봐서 생긴 일 |
|---|---|---|---|
| ① RVA | `mig_verify check/coverage/dups` | 주소가 옮겨졌는가 | — |
| ② **구조체 오프셋** | **`offsets.py check/sources`** | 필드가 밀렸는가 | ★**0.5.8 크래시의 진범** |
| ③ **환경** | **`env.py`** | deps 대역·버전게이트 상수·빌드 SDK 경로·apply 누락·stale dll | 0.5.8 에서 5건 전부 여기 |

★**`run.py` 가 종료코드 0 을 줄 때만 "마이그 완료"라고 말할 수 있다.** 그 다음이 인게임 검증이다.
⚠**0.5.8 교훈**: RVA 1,454건을 전부 재핀하고 `check` 전건 PASS·`coverage` 클린을 받고도 게임이 크래시했다.
원인은 **구조체가 0x10 커져 그 뒤 필드가 전부 밀린 것**. ⑦에 "offsets 는 별도 확인"이라고 **글로는**
적혀 있었지만 기계 검사가 없어 지켜지지 않았다 — **글로 적힌 절차는 지켜지지 않는다.**

### ② 구조체 오프셋 축이 어떻게 동작하나
매니페스트는 이미 "우리가 의존하는 게임 함수" 주소를 들고 있다. 그 함수가 **어떤 필드 오프셋을 쓰는지**
히스토그램 지문을 떠 두고, 다음 버전에서 재핀한 뒤 다시 떠서 diff 한다.
```
python MIG\offsets.py snap  --exe <현행exe> --pkl <현행pkl>   # ★마이그 "완료 후" 채록(다음 회차 기준선)
python MIG\offsets.py check --exe <신exe>  --pkl <신pkl>      # 재핀 후 대조 -> 이동한 오프셋 전수
python MIG\offsets.py sources                                # 그 값을 하드코딩한 소스 위치 지목
```
⚠**snap 타이밍**: 오프셋 작업이 **끝난 뒤에** 떠야 한다. 작업 전에 뜨면 기준선이 새 버전으로 덮여
이동이 영영 안 보인다. (0.5.8 실측 = ORACLE `+0x110/0x130/0x158/0x198/0x2a8/0x310` → 전부 +0x10,
sylas `+0x1e0→0x1f0` 5개 함수, serpen MOBATICK Δ+0x38 등)

## 패치가 오면 (표준 절차)

```
① 새 exe 확보 (백업: C:\Users\jungs\Desktop\claude\tfm2\tfm2_<신버전>\)
② python MIG\mig_verify.py check --exe <새exe>
     → STALE = 이동한 지점 전수 목록. 이것이 이번 마이그의 작업 목록이다.
     ⚠~~PASS 여도 안심 금물 아님 — PASS = "바이트 그대로" = 진짜로 안 움직였다는 뜻(강한 증거).~~
       → ★정정(2026-08-29·버전무관): **PASS = 바이트 그대로일 뿐, "구조가 여전히 맞는가"는 못 본다** = 아래 절.
③ 모드별로 STALE 엔트리 재탐색:
     1순위 = 매니페스트의 bytes 12B 를 새 exe 에서 find_unique (단일 매치면 끝)
     2순위 = _mig 엔진 match_fn(함수시작) / match_mid(컨테이너 승계)
     3순위 = ghidra-re (본문 변경 = 로직이 바뀐 것)
     ⚠컨테이너 승계 실패 ≠ 재핀 불가 — sig 단일 매치로 잡히는 경우 있음(banpick_order HL_COUNT).
④ 소스 수정 + 매니페스트 value 동시 갱신 (둘이 어긋나면 coverage 가 잡음)
⑤ python MIG\mig_verify.py rebase <MOD> --exe <새exe>   ← 바이트 재채록
⑥ 완료 판정 3종 세트 (전부 통과해야 "마이그 완료"):
     check 전 PASS  +  coverage 클린  +  dups 로 연동 그룹 동시 갱신 확인
⑦ offsets(구조체 축)는 exe 대조 불가 — 각 항목의 verify 방법(RE/런타임 스캔)으로 별도 확인
⑧ SDK 교체(sdk_<신버전>) 후 전 모드 재빌드 — ★RVA 0 모드도 재빌드 필수(rlib DIFF)
⑨ ★python MIG\offsets.py check → sources  = 구조체 오프셋 이동 전수 + 고칠 소스 위치
⑩ ★python MIG\bump_deps.py --to <신>     = mod_info base 대역(안 하면 전 모드 자동 비활성)
⑪ ★python MIG\env.py                       = 버전게이트 상수·빌드 SDK 경로·apply 누락·stale dll
⑫ ★python MIG\run.py 가 종료코드 0 → 그때만 "마이그 완료"
⑬ 오프셋 작업이 끝났으면 python MIG\offsets.py snap  (다음 회차 기준선 채록)
⑭ 인게임 검증 → REPORT 검증표 갱신 → rel_commit
```

## ★★check PASS ≠ 기능 정상 (2026-08-29 · 버전무관 · 실사고 sylas)
`mig_verify` 가 검사하는 명제 = **"그 주소의 12B 가 채록 때와 같은가"**.
정작 필요한 명제 = **"그 주소가 여전히 올바른 구조인가"**. 둘은 다르다.
- 실사고: `check sylas` **PASS 26 / STALE 0** 인데 `EFF_VT_BASE` 가 가리키는 곳은 0.5.7에서
  **effect vtable 표가 아니었고**, 모드의 AI 마스킹은 0.5.7 내내 **무동작**이었다.
  주소가 여전히 "유효한 바이트"를 담고 있어 **바이트 대조를 그대로 통과**한 경우.
- ⚠**엔트리 `ver` 가 낡았는데 PASS면 그 자체가 경고 신호다.** "라벨만 낡았나 보다"로 읽지 말 것 —
  라벨을 고칠 게 아니라 **"이 검증이 무엇을 안 보고 있는가"** 를 물어야 한다.

**대응 3종 (완료 판정 3종 세트와 별도로 수행)**
1. ★**구조 불변식으로 검사한다.** RVA 하나가 아니라 **표 전체의 성질**을 본다.
   예(sylas): stride `0x120` 으로 57개를 훑을 때 **`+0x108` 이 57/57 전부 동일**
   (`0xfbf0a0` = `b8 80 bb 00 00 c3` = `mov eax,0xbb80; ret`) ⟹ base·stride 를 이걸로 확정.
2. ★**모드 시작 시 자기점검** — 매 실행 **첫 줄에 PASS/FAIL** 을 찍고, FAIL 이면 재핀 방법·정본 경로까지
   로그에 남긴다(참조 구현 = sylas v129b).
3. ★**soft-fail 경로에 눈에 띄는 신호를 붙인다.** 안전한 폴백 자체는 옳지만(잘못된 함수 포인터를 심는 것보다 낫다),
   **폴백이 조용하면 기능이 죽은 채로 몇 버전을 간다.**
- ⚠**시그니처가 약한 엔트리**(채록 12B 가 대부분 `00` 등)는 `note` 에 그 사실을 명시하고,
  진짜 검증은 위 자기점검에 맡긴다(예: sylas `EFF_VT_BASE` 0.5.7).
- 근거·전문 = `REPORT\sylas\03_시행착오.md` "마이그 검증이 PASS인데 기능은 죽어 있었다" 절.

## 개발 규칙 (마이그 아닐 때)
- 소스에 **RVA 대역 상수를 새로 넣으면** `gen` 재실행(기존 큐레이션은 보존됨).
  안 하면 다음 `coverage` 가 미등록으로 잡아준다 — 그게 이 시스템의 안전망이다.
- 구조체 오프셋을 새로 확정하면 해당 모드 매니페스트의 `offsets` 에 수동 추가.
- 새 모드 = `mig_verify.py` 의 `MODS` 목록에 추가 후 `gen`.
  ★설치 여부 기준이 아니다 — **공유모듈(ui_kit)·mod_info 유실 모드도 이 목록에 있어야 한다**(0.5.7 사고 1·9).

## 사각지대 7종 — 이 시스템이 어떻게 막나
| # | 사각지대 (0.5.7 실사고) | 방어 |
|---|---|---|
| 1 | 함수시작 아닌 중간 사이트 | check(바이트 대조는 종류 무관) |
| 2 | .rdata 주소 (RUNNER_VT) | check + sect 필드(.text 아님이 명시됨) |
| 3 | 코드 인라인 imm (byte-patch 대상) | check |
| 4 | 여러 줄 배열 (CPROD_CALLSITES) | 추출기가 const 8줄 창으로 배열 원소 귀속 |
| 5 | 공유모듈 (ui_kit) | MODS 목록에 명시 포함 |
| 6 | 콜사이트 대응 오류 | method 필드에 판별자 기록(재발 방지) |
| 7 | 인라인 리터럴 (rva == 0x…) | 추출기가 const 여부 무관 전수 추출 + coverage |
| + | 로컬 복사본 (0x1788×3모드) | **dups** — 같은 값 보유 모드 그룹 보고 |
| + | 침묵 사망 (sig fail-safe) | check 는 로그·증상과 무관하게 exe 만 본다 |
| 9 | ★**구조만 바뀐 주소**(sylas EFF_VT_BASE 0.5.7 — 바이트 유효·표 아님 ⟹ check PASS인데 기능 사망) | check로는 **못 막는다** ⟹ **구조 불변식 검사 + 모드 자기점검**(위 절) |
| 8 | 무효 구값 엔트리 (HR_AE_FN 오핀 08-28 — 구값이 함수시작 아닌데 바이트 이주가 유일 매치 통과) | **재핀 전 구값 유효성 선검증**(함수시작=프롤로그/vtable=.rdata)·무효=ghidra-re 재규명·`*_FN`은 재핀 후 함수시작 검증 |

## ★★apply 오프셋 사고 (2026-09-02 · 버전무관 · 실사고)
0.5.8 1차 `apply` 가 **12개 모드 소스를 통째로 훼손**했다. 원인은 한 줄:
`mig_verify.strip_code` 는 주석을 **지워서 길이가 줄어드는데**, 그 오프셋으로 원본을 치환했다.
주석이 하나라도 있는 파일은 그 뒤 모든 위치가 밀려, 상수 대신 **주석 한복판이 덮였다**
(`// 위 ja 때문에 도달불가=가드` → `// 위 ja 때문0x1835b4a`).
- 방어 = **`mask_code`(길이보존 마스킹)** — mig_verify 정본, repin 이 재수출. `extract`/`coverage` 도 같은 규칙.
- 부수효과(의도한 것): **문자열 리터럴도 마스킹**한다 ⟹ 진단 로그·설명 문자열에 적힌
  과거 버전 RVA 를 재핀값으로 덮어쓰지 않는다(1차에는 그것도 덮어써 이력 주석이 오염됐다).
- ⚠**마스커의 함정**: 문자 리터럴 `'"'` 를 처리 못하면 스캐너가 문자열 모드에 빠져 **이후 전 라인이
  마스킹**되고, 치환이 조용히 **누락**된다(serpen 5건). 훼손이 아니라 **누락**이라 diff 로는 안 보이고
  `coverage` 만 잡아낸다 ⟹ **apply 뒤 coverage 는 선택이 아니라 필수 단계다.**
- 교훈: 소스 자동치환은 **"덮어쓰기"가 아니라 "마스킹된 좌표계에서의 치환"** 이다.
  치환 좌표를 만드는 함수와 치환 대상 문자열은 **길이가 같아야 한다**.

## 현행 상태 (2026-09-02 · 게임 0.5.8) — ⛔**마이그 미완**
★`run.py` 종료코드 **1**. 축① RVA 는 클린이지만 **축② 구조체 오프셋 · 축③ 환경이 열려 있다.**
- ⛔**인게임 크래시 발생**(0xc0000005 읽기·`faultAddr=0x5df3` = 널 컨테이너 순회). 원인 = **축②**.
  콜체인 = comptest ORACLE → serpen MOBATICK → sylas ETICK → `0x132f630` 에서 폭발.
  이 함수들의 0.5.7 대응본과 오프셋을 diff 하니 **필드가 +0x10 밀려 있었다**.
- 현재 조치 = `tfm2_ai_adjust`·`sylas`·`tfm2_elemental_serpen`·`tfm2_comptest_unlock` 를
  **deps 대역으로 비활성**(구 대역 `>=0.5.7, <0.5.8`). 오프셋 수정 후 되살릴 것.
- 남은 작업 = `offsets.py sources` 가 찍어 준 **38곳**(sylas `0x1e0→0x1f0` 등) + 아래 잔여 재핀 137건.
- ✅**0.5.8 축① 정합 (check PASS + coverage 클린)**: item_tactics·champ_pos_lock·comptest·banpick_order·
  banpick_illust·serpen·draft_overlay·level_cap·champion_exclude·bancard_keep·sylas·ui_kit
  + SDK 전용 5종(mod_order·html_overlay·Spectator_Chat·community_reaction_mod·meta_item_delegate)
- **부분 정합(재핀 실패 잔여)** — 매니페스트에 `"unresolved": "0.5.8"` 낙인이 찍혀 있다(grep 대상):
  | 모드 | 잔여 | 성격 |
  |---|---|---|
  | `tfm2_ai_adjust` | 125 | 대부분 detour.rs 인라인 사이트. 0.5.7 때도 PENDING 이었다 |
  | `tfm2_banpick_order` | 7 | AI6 6사이트 + `RVA_PHASE_SCALAR` — 동형 클론 2,849후보에 막힘 |
  | `tfm2_comptest_unlock` | 4 | `SRV_RVA`·`CT_ARM_LO/HI`·`CT_REGION_HI` |
  | `tfm2_stat_exp` | 1 | `RUN_TICK_RVA`(보류 모드) |
- ⚠★**`rebase` 는 재핀 실패분까지 check PASS 로 둔갑시킨다**(그 주소의 현재 바이트를 그냥 다시 채록하므로).
  그래서 실패분에 `unresolved` 낙인을 남긴다 — **check 결과만 보고 "다 됐다"고 읽지 말 것.**
- ~~0.5.7 정합 목록(2026-08-28)~~ → 위로 대체. 0.5.7 경위는 REPORT\_공통 마이그 문서 참조.

## 구버전 자료
- 구 MIGRATION.md(636KB, §7 세션 이력 누적) → `_archive\MIGRATION-이력-2026-08-28.md`
- 구 mig 스크립트 35종 → `_archive\mig_scripts\` (현행 엔진 `_mig057*.py`·`migrate_rva.py` 만 잔류)
