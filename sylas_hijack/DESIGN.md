# Sylas Hijack — 엔진 네이티브 궁 강탈 (설계 정본)

목표: 사일러스(데이터 챔프 `sylas`)의 궁극기 R이 **적 챔피언의 실제 엔진 궁극기를 그대로 시전**하게 만든다.
방식: 네이티브 트램폴린 디투어 DLL(이 프로젝트의 plan_reimpl/scrim과 동일 계열). SDK 챔피언 시스템에 의존하지 않음.

## 아키텍처
- `sylas`(데이터 챔피언) = 비주얼(도트)·스탯·Q(skill)·E(skill2)·기본 R(ult, 강탈 트리거용 사슬 연출). 이미 배포됨.
- `sylas_hijack`(이 모드, 네이티브 DLL) = 강탈 로직. `declare_mod!`로 로드만 되고, 실제 작업은 init/스폰스레드에서 **트램폴린 디투어 설치**(scrim 패턴).
- 빌드 툴체인 = **nightly-2026-05-24** (현행 0.4.14 SDK, `sdk_0414_new/mod-sdk`). 빌드 = `build_mod.bat`(SDK) 또는 rustc 직접.

## RE 확정 사실 (ghidra-re 2026-06-29, 메모리 tfm2-champion-system.md 참조)
엔티티(stride 0x6a8)는 `entity+0x6a0` 단일참조 → **ACTIONSET**(통합 액션객체) 경로로 모든 액션을 얻음.
AI input 평가 시: `pcVar7 = (*vtable[+0x128])(self_data, *(u64*)(caster_entity+0x6a0))` → ACTIONSET.

### ACTIONSET 레이아웃 (stride 0x6a8, entity-like)
| 오프셋 | 내용 |
|---|---|
| `+0x5b0` | 보유 액션 개수 (ult 활성 = >2, skill2 활성 = >4) |
| `+0x4b0` | attack/skill action 블록 |
| **`+0x4e8`** | **ult action 블록** (개수≤2면 더미 `&DAT_143672110`) |
| `+0x520` | skill2 action 블록 |
| `+0x2b0/+0x2b8` | effect-list (Vec\<CastedEffect\> stride 0x28, disc 4/5/6/7) — ⚠ ACTIONSET 공용 |
| `+0x420` | range term, `+0x458` range base, `+0x668` range growth |
| `+0x648/+0x650` | action 좌표, `+0x3e8` stat, `+0x68` casting state, `+0x6a8` team |

action sub-block stride 0x38: `[+0]`data_ptr / `[+8]`effect-builder vtable(slot+0x90 빌드) / `[+0x10][+0x18]`range coeffs / `[+0x28]`cooltime / `[+0x30](int)`casting_type(-1None/0Targeting/1Position/2Direction).

### ★후킹 지점 확정 (ghidra-re 2026-06-29 후속, 현행 0.4.14 핫픽스 exe 직접 정합)
**후킹 = ult dispatch `0x18da700`** (abs 0x1418da700). ult **전용**(skill/skill2와 분리됨).
- ABI = **sret** (RCX=출력버퍼, 반환 RAX=RCX). ⟹ plan_reimpl `install_replace_detour`(sret rax=rcx)와 **동일 형태 → 그 메커니즘 재사용**.
- **param_4 = R9 = caster 엔티티** (`mov rdx,[r9+0x6a0]` @0x18da731). champion_name @ caster+0x250/+0x398.
- **param_7 = [RSP+0x120] = target 엔티티** (궁 대상, stride 0x6a8 전 필드 접근: 좌표 +0x648/+0x650, hp +0x658, +0x6a0=ACTIONSET ref).
- 프롤로그 19B(rip-rel 없음, 5B jmp 안전): `41 57 41 56 41 55 41 54 56 57 55 53 48 81 EC A8 00 00 00` (8×push + sub rsp,0xa8).
- 본문서 getter(`[[param_5]+0x128]`, param_5=[RSP+0x110] self_data)로 ACTIONSET(R15) 얻고 R15+0x4e8(ult블록)/+0x5b0(개수>2)/+0x3e8 사용.
- 다른 dispatch RVA: skill=0x18d9e80, skill2=0x18da2d0. cooltime gate(3스킬 **공유**, ult분리 불가)=0x1b8cd40(target=R8, `cmp byte[r8+0x6a0],1`).
- 마스크시그(다음 패치 재탐색): cooltime gate `4883ec2831c04180b8a006000001`, dispatch core `498b91a0060000 ff90280100004885c0`.

### ⚠ +0x6a0 = 태그형 작은 구조 (raw 포인터 아님)
cooltime gate가 `cmp byte[r8+0x6a0],1` → +0x6a0은 {tag,...,ptr} 형태(Option/enum). getter(vtable+0x128, 다형성)가 실제 ACTIONSET 포인터 추출. **모드는 +0x6a0 내부해석 불요** — caster/target raw 엔티티 포인터를 직접 다루고, getter 반환 ACTIONSET을 스왑/편집하면 됨. identity vs 인덱싱 최종판정은 계측에서(getter 반환 RAX vs caster+0x6a0 비교).

### effect 발화 = 캐스팅 state-machine VM(0x1e41800류, 매틱 ACTIONSET effect-list 순회 apply)

## ★계측 결과 (2026-06-29, 인게임 실측) — 엔티티 레이아웃 확정
- **champion_name (char*) @ entity+0x250** (0.4.14; String은 +0x248. 구 메모 +0x250 맞음. r9가 챔프일 땐 +0x390에도 String 보였으나 = 다른 뷰).
- **엔티티 액션 포인터(8B each)**: attack/skill 블록 ptr @ **+0x4b0**, **ult 블록 ptr @ +0x4e8**, skill2 블록 ptr @ **+0x520**, 액션개수 @ **+0x5b0**. (count=1 미니언은 셋 다 동일 ptr; count=3 챔프는 셋 다 다름). +0x4f0=vtable/code ptr.
- ⟹ **강탈 = target_entity[+0x4e8] (ult 액션 ptr)를 sylas_entity[+0x4e8]에 복사** + count(+0x5b0)≥3 보장. 8바이트 1개 복사로 ult만 교체(Q/E 유지). effect-list(+0x2b0)는 액션 ptr가 가리키는 객체 내부라 자동 동반(공용 아님 — 액션 ptr 단위).
- ⚠ **후킹 함수 abstract_input::ult(0x18da700)는 부적합**: AI 후보입력 생성에 빈번 호출돼 caster(r9)/target(p7) 역할 불안정(대부분 미니언 대상). r9는 종종 작은 디스크립터. ⟹ **per-tick 엔티티 스캔으로 전환**(매 틱 엔티티 순회→name=="sylas" 찾기→타깃 챔프 ult-ptr 복사).
- 잔여 RE: ①엔티티 열거 루트(전체 엔티티 배열/Vec 위치 — RE 3라운드 미수렴) ②per-tick 훅=**run_tick_ext @ 0x1e2f2a0**(12B push 프롤로그 `55 41 57 41 56 41 55 41 54 56 57 53` + mov eax,0x2898 + chkstk, rip-rel 없음, 12B 트램폴린 안전) ③team/alive 오프셋(미상).

## ★전략 전환(2026-06-29): 레지스트리 기반 강탈 (풀 열거 RE 우회)
엔티티 풀 열거 RE가 3라운드 미수렴 → **이미 작동하는 dispatch 훅(0x18da700)이 넘겨주는 살아있는 엔티티 포인터로 챔피언 레지스트리 구축**.
- dispatch 훅 매 콜: param_7(p7, 클린 엔티티 base)가 챔프(name char*@+0x250 읽힘 + count@+0x5b0 in 3..8)면 레지스트리 upsert: `name → {ptr, x@+0x648, y@+0x650, ultptr@+0x4e8}`.
- 사일러스 강탈: 레지스트리에 "sylas" 존재 + 다른 챔프 존재 시 → **가장 가까운(x/y 거리) 다른 챔프의 ultptr을 sylas[+0x4e8]에 복사**(team 불필요). ARM 플래그로 쓰기 게이트(기본 OFF=탐지전용).
- ⚠ r9는 +0x148 시프트된 뷰(name@+0x390)라 오프셋 혼선 → **수집은 p7만**(클린 base). 사일러스도 적에게 타겟될 때 p7로 들어옴.
- ⚠ +0x4e8(액션객체 ptr)은 per-entity/타입별 → 소스 사망 시 dangling 위험. v1=상시복사 데모, v2=트리거/복원 정교화.
- 1단계 검증=탐지전용(ARM=false): 사일러스가 p7로 닿는지 + 최근접 챔프 계산 확인(쓰기 없음=안전).

## 강탈 구현 방안 (택1, 계측 후 확정)
1. **ult 블록 복사(선호)**: 타깃 ACTIONSET `+0x4e8` ult 블록(0x38B)을 사일러스 ACTIONSET `+0x4e8`에 복사 + `+0x5b0`≥3 보장. Q/E는 사일러스 유지, R만 적 궁.
   - ⚠ effect-list(+0x2b0)가 ACTIONSET 공용 → ult effect가 effect-list로 인덱싱되면 블록 복사만으론 부족할 수 있음(계측으로 확인).
2. **+0x6a0 통째 스왑**: 사일러스 entity+0x6a0 = 타깃 값. 전 액션 교체(과함). 폴백용.

## 미해결 (계측/RE로 확정)
- `+0x6a0`이 직접 포인터(A)인지 인덱스(B)인지 → 런타임 1회 확인(caster+0x6a0 값이 ACTIONSET 주소와 일치하면 포인터).
- 후킹 함수의 caster/target 파라미터 위치 + 현행 exe RVA → ghidra-re 후속.
- effect-list(+0x2b0)와 ult 블록(+0x4e8) 결합(ult만 복사 시 effect 동반 여부).

## 작업 순서 (안전 우선: 읽기 → 검증 → 쓰기)
1. **[진행] 후킹지점 시그니처·현행RVA 핀포인트** (ghidra-re 배경)
2. **읽기전용 계측 DLL**: 사일러스 entity 탐지(champion_name +0x250 == "sylas") → +0x6a0 → ACTIONSET 레이아웃 덤프(로그). 쓰기 없음=안전. → +0x6a0 포인터/인덱스, ult블록, effect-list 결합 확인.
3. **강탈 쓰기**: 사일러스 R 시전 감지 → 타깃 ult 블록 복사 → 타이머/복원. catch_unwind + safe_write(VEH) + champion_name=="sylas" 게이트.
4. **인게임 검증 루프**: 사일러스로 적 궁(예: 광역 궁) 강탈 → 실제 시전 확인.

## 안전수칙 (tfm2-mod-safety)
- detour 본문 = `panic::catch_unwind(AssertUnwindSafe)`. 락 = poison-safe.
- raw 포인터 read/write = scrim의 SEH safe_copy/safe_read_* 재사용(직접 *mut 금지).
- 같은 게임함수 두 모드 후킹 금지(plan_reimpl/scrim과 후킹지점 충돌 점검).
- 쓰기 로직은 cfg 플래그 기본 OFF로 격리, 계측부터.

## 재사용 자산
- SEH safe_copy/safe_read_u64/safe_read_bytes/write_log/트램폴린 install = `C:\tfm2mods\tfm2_scrim\src\lib.rs` (L427~, SEH L240~425).
- 마이그레이터 = `C:\tfm2mods\migrate_rva.py`.

---

# W38 — 방법2 재설계: 네이티브 apply 그대로 실행 (0.5.2, 2026-07-26)

유저 확정 방향: "하드코딩된 바닐라 궁을 데이터챔프(사일러스)가 그대로 실행". 방법1(replica)은
소환·변신·네이티브 비주얼을 데이터챔프 JSON 어휘(44종)로 표현 불가 = 구조적 천장.
방법2는 원본 네이티브 코드를 실행하므로 천장 없음 — 남은 건 **판별**과 **일반화**뿐.

## 회고: 왜 W36 성공·W37 실패였나
- **W36(실행층 돌파)**: datachamp apply → gambler 네이티브 apply를 살아있는 인자로 명시 CALL.
  사일러스 문맥에서 **64회 실행·크래시 0**(ret 일관 non-zero = 유효 트리 통과, 최대 AV 리스크 통과).
  ⟹ "봉쇄"라던 네이티브 apply 재사용이 **실행층에서는 작동**함이 실증됨.
- **W37(판별 실패=핵심 벽)**: "사일러스 궁 descriptor(entity+0x588) == datachamp apply의 rcx"로 판별 →
  **단 한 번도 일치 안 함(리다이렉트 0회)**. 원인 = datachamp apply가 실행하는 descriptor(rcx)는
  effect-tree/state에서 오는 **실행 descriptor**이지, casting-block(+0x588)의 궁 descriptor가 아님.
  ⟹ 판별 기준 자체가 틀렸다. 실행층은 멀쩡, **판별만 재설계하면 됨**.

## 재설계 ①: 판별 = descriptor 일치 → **caster 식별**
game-atlas/ghidra-re 단서: apply 인자에서 caster를 신뢰성 있게 뽑을 수 있다.
- datachamp apply 5인자: rcx=descriptor / **rdx=caster_handle** / r8=sim_state / r9=target_ctx / [rsp+0x28]=casting_ctx.
- **rdx(caster handle) → sim_state[+0x140] resolve → entity → name(@+0x250, 0.5.2 검증됨) == "sylas"** 로 판별.
- 즉 "이 apply 호출을 부른 시전자가 사일러스인가"로 게이트. descriptor가 뭐든 무관.
- ⚠ 궁/평타/스킬 구분 필요(궁만 리다이렉트): casting_type(Targeting0/Position1/Direction2/None3) 또는
  슬롯 판별 = ghidra-re 재핀 항목5. (사일러스 데이터챔프 궁의 casting_type을 프로브로 실측해 상수화)

## 재설계 ②: 강탈 대상 = 하드코딩(gambler) → **살아있는 것 빌리기(일반화)**
W36은 gambler apply를 하드코딩 CALL + gambler descriptor를 static POD로 위조 → gambler 한정.
임의 챔프 일반화의 부담(챔프별 descriptor·apply RVA 60종 디컴프)을 피하는 설계:
- 경기에 **실제로 존재하는 바닐라 챔프가 궁을 시전할 때** 그 apply 호출을 훅해서
  **(descriptor_ptr, apply_fn, casting_type)를 caster별로 캡처 → 레지스트리**. (W31 데이터챔프 강탈의 바닐라판)
- 사일러스 궁 apply 진입 감지 → 레지스트리에서 타깃 바닐라 챔프의 **(그 챔프 자신의 descriptor, 그 apply_fn)** 로 CALL 리다이렉트.
- ★핵심 안전점: 타깃 apply는 **그 챔프 자신의 descriptor**로 호출 → 형식 일치 보장(W36 gambler POD 위조 불필요, 힙 오버리드 AV 회피).
- cfg `target=<champ>` 로 대상 지정. 미지정 시 "경기 내 임의/최근접 바닐라 챔프".
- 한계: 타깃 바닐라 챔프가 **경기에 실제 존재해야** 그 궁을 빌릴 수 있음(replica처럼 "경기 밖 정의 훔치기"는
  네이티브는 descriptor 소스가 없어 불가 — 이건 방법1 replica의 몫). 방법2 = "경기 내 바닐라 궁 실시간 복제".

## ⛔ 절대 금지 (문서 확정)
- **rcx-swap 금지**: datachamp apply는 rcx descriptor를 0x170 전제로 `[rcx+0x148]` 읽음 → 작은 descriptor 넣으면
  힙 오버리드 = AV. 게다가 실행 함수는 vtable이 결정하므로 rcx만 바꿔선 datachamp apply가 그대로 실행됨.
  ⟹ **명시 CALL(대상 apply_fn을 직접 호출)만 유효**. (champion-system.md :393)
- casting VM(0.4.14 0x1e41800류) 후킹 금지 = 즉시 크래시.
- 바닐라 궁 직접 강탈 4주입층(W28) = 아키텍처 봉쇄, 재시도 금지 (이번 건 그게 아니라 apply 실행 리다이렉트).

## 프로브 v2 계측 목표 (게임 실행 시)
1. datachamp apply 진입점(ghidra-re 재핀 or 런타임 캡처)이 사일러스 궁을 실행하는가 — caster=="sylas" 뽑히는가.
2. 사일러스 데이터챔프 궁의 casting_type/슬롯 시그니처 실측(궁만 리다이렉트하려면).
3. 경기 바닐라 챔프 궁 시전 시 apply 호출을 caster별로 잡아 (descriptor, apply_fn) 캡처 — 레지스트리 성립 확인.
쓰기(리다이렉트) 없이 계측만 = 안전. 판별 시그니처 확정 후 리다이렉트 배선.

## 0.5.2 재핀 필요 RVA (ghidra-re 진행중, 확정 시 채움)
- datachamp descriptor vtable (0.4.14 0x35914f8) → slot26(+0xd0) datachamp apply (0.4.14 0x1c90c20) = TBD
- gambler descriptor vtable (0.4.14 0x35a7980) → slot26 apply (0.4.14 0x1d10190) = TBD
- caster resolve 경로 sim_state[+0x140] 0.5.2 유효성 = TBD
- 못 찾으면 런타임 캡처: 프로브 v1(probe052)이 master.vtable → descriptor vtable → slot26을 인게임에 얻는 경로.
