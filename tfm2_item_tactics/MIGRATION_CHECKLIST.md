# tfm2_item_tactics — 버전 마이그레이션 체크리스트

게임 패치로 exe가 바뀌면 하드코딩된 **RVA·바이트패치 주소·프롤로그**가 어긋나 훅 미설치/크래시가 난다.
이 문서는 패치 시 **바꿔야 할 값 전부**를 한 곳에 정리한다. (현재 기준 = **0.4.14 + 핫픽스**, image base `0x140000000`, RVA = abs − base.)

> 실소스 = `C:\tfm2mods\tfm2_item_tactics\src\lib.rs` (+ `ui_inject.rs`)
> 빌드 = `powershell -File C:\tfm2mods\build_inj.ps1 -Src C:\tfm2mods\tfm2_item_tactics\src\lib.rs -ModId tfm2_item_tactics`
> 정본 RVA 표 = `C:\tfm2mods\MIGRATION.md §7` (여기 갱신도 병행 → version-migrator/record-keeper 경유)

---

## 0. 마이그레이션 절차 (CLAUDE.md §4)
1. 현재 게임 exe를 `_N\`로 백업 (migrate_rva.py의 OLD가 직전 백업).
2. `python C:\tfm2mods\migrate_rva.py` 실행 → capstone 마스크 시그 + string-xref로 함수 시작 RVA 자동 재탐색.
3. 아래 표의 값들을 새 RVA로 교체. **migrate_rva.py가 못 잡는 것(함수 내부 패치 주소·rbp 변위·프롤로그 변경)은 `ghidra-re` 서브에이전트로 재도출.**
4. 재빌드 → 인게임 검증(§4 체크).

---

## 1. ★ACTIVE 함수 RVA (필수 — 전부 재탐색)
migrate_rva.py가 함수 시작을 잡아준다. 프롤로그가 바뀌면 프롤로그 상수도 갱신.

| 상수 | 현재 RVA | 위치 | 용도 | 프롤로그/시그 |
|---|---|---|---|---|
| `FN_DD_SETOPT_RVA` | `0x218a5f0` | lib L30 | 네이티브 dropdown set-options (item3 칸) | — |
| `SETTER_NOP_RVA` | `0xf1a74b` | lib L803 | revert setter NOP (개인전술 되돌림 차단) | call `e8` (5B) |
| `C6_RVA_A` | `0xc76a89` | lib L1429 | c6 detour 팀A slot0 read (모드템 slot 주입) | `C6_PROLOGUE` |
| `C6_RVA_B` | `0xc76d81` | lib L1430 | c6 detour 팀B slot0 read | `C6_PROLOGUE` |
| `C6_PROLOGUE` | `[41 0f b6 44 24 f8 48 8d 0d]` | lib L1434 | movzx eax,[r12-8] + lea | — |
| `RVA_REALLOC` | `0x88c700` | lib L1448 | `__rust_realloc` (build Vec 3→4 확장) | — |
| `RVA_BUY_ITEM` | `0x2052ca0` | lib L2206 | buy_item replace-detour (4번째 실구매) | `BUY_PROLOGUE` |
| `BUY_PROLOGUE` | `[41 57 41 56 41 55 41 54 56 57 55 53]` | lib L2207 | push r15..rbx (12B, 다음=sub rsp 경계) | — |
| `ITEMNET_FORWARD_RVA` | `0x19f01a0` | lib L2253 | 아이템 신경망 forward (AUTO 4번째 채점) | `[55 41 57 41 56 41 55 41 54 56 57 53]` (L2342 검증) |
| `RVA_SLOT_HELPER` | `0xbbbd60` | lib L2639 | slot 경로 헬퍼 (경기중 4슬롯 아이콘) | `48 83` (sub) |

### ui_inject.rs (UI 주입 = 체이닝 로더 훅)
| 상수 | 현재 RVA | 위치 | 용도 |
|---|---|---|---|
| `LOADER_RVA` | `0x540ad0` | uinj L12 | 에셋 로더 훅 seam (조각 주입 진입) |
| `PARSER_RVA` | `0x220e100` | uinj L13 | .ui 파서 |
| `ALLOC_RVA` | `0x231fb70` | uinj L14 | 게임 alloc (자식노드 배열 확장) |

---

## 2. ★ACTIVE 바이트패치 주소 (필수 — ghidra-re로 재도출)
함수 시작이 아니라 **함수 내부 특정 명령**의 주소 → migrate_rva.py로 안 잡힘. 시그 바이트로 재탐색 필요.
각 패치는 **적용 전 시그 검증**을 하므로, 패치가 조용히 스킵되면 = 주소가 어긋난 것.

| 패치 | 주소(RVA) | 위치 | 시그(검증) | 동작 |
|---|---|---|---|---|
| owned cap | sig `0x1e32a7e` / imm `0x1e32a85` | lib L2569-70 | `48 83 b8 d0 03 00 00 03` (`cmp [rax+0x3d0],3`) | imm8 `03`→`04` (4번째 스탯 적용) |
| beam depth A | `0x19f14a5` | lib L2594 | `41 83 f8 02` (`cmp r8d,2`) | imm8 `02`→`03` (beam 4-item 계산) |
| beam depth B | `0x19f1a11` | lib L2594 | `41 83 f8 02` | imm8 `02`→`03` (백엣지, **둘 다 필수**) |
| slot bound 1 | `0x5e6f60` | lib L2644 | `49 83 fe 30` | imm8 `0x30`→`0x40` (slot 루프 상한, 창모드 blue) |
| slot bound 2 | `0x5e72c0` | lib L2645 | `49 83 ff 30` (`cmp r15`) | `0x30`→`0x40` |
| slot bound 3 | `0x5e7950` | lib L2646 | `49 83 fe 30` | `0x30`→`0x40` (전체화면) |
| slot bound 4 | `0x5e7cb0` | lib L2647 | `49 83 fe 30` | `0x30`→`0x40` (**4곳 다 해야 양팀·양화면**) |

### c6 build-base 스택 변위 (rbp 상대, RVA 아님)
| 상수 | 값 | 위치 | 주의 |
|---|---|---|---|
| `C6_BB_A` | `0x48c40` | lib L1431 | 팀A build base `[rbp+..]` 변위. c6 함수 스택 레이아웃 바뀌면 재도출(ghidra-re). |
| `C6_BB_B` | `0x48c28` | lib L1432 | 팀B build base 변위. |

---

## 3. 구조체 오프셋 (보통 안정 — SDK/AI 구조 변경 시만 재검)
패치가 **RVA만** 바꾸면 이 오프셋들은 대개 그대로. 단 SDK/ABI·AI 구조 개편(메이저 버전업) 시 재검 필요.

### athlete (= SimState+0x808 로스터 배열 원소, stride `ATH_STRIDE=0x758`)
| 오프셋 | 의미 |
|---|---|
| `+0x398` / `+0x3a0` | champion name String {ptr, len} |
| `+0x3d0` | owned 아이템 수 |
| `+0x408` / `+0x410` / `+0x418` | build Vec {cap, ptr, len} (항목=최종템 id u64) |
| `+0x6a8` | team (0/1) — 적팀 = 1-team |
| `+0x710` | 예산(gold) 상한 |
| `+0x738` | **포지션(0~4)** — 라인업 ctx 배치용 (팀별 고유) |

### SimState (run_tick_ext param_2)
| 오프셋 | 의미 |
|---|---|
| `+0x808` | 로스터 배열 base ptr (그 경기 10명) |
| `+0x810` | count |

### Database (신경망·모드템)
| 오프셋 | 의미 |
|---|---|
| anchor `+0x16698` | Database 시작 도출 기준 (champion_patch_statistics) — lib L2183 |
| `+0xda0` | 아이템 신경망 net (LogisticSGDAgent, self-check 16384/16384/1) — lib L2188 |
| `+0x15d78` | mod_items 통합 Vec (stride `0x1a8`) |
| `+0x16690` | 활성모드 서명 Vec |

### elem / Node / 기타
| 오프셋 | 의미 |
|---|---|
| elem `+0x180` / `+0x188` | price / tier |
| elem next_tier | 아이템 트리 판별 (오프셋 자동탐지 — lib `find` 계열) |
| `NT_SIZE = 0x90` | NodeTemplate 크기 (uinj L15) |
| Node `+0x08`/`+0x10` | id {ptr, len} (uinj) |
| Node `+0x48`/`+0x50`/`+0x58` | children {cap, ptr, len} (uinj) |
| `TEAM_ID_OFF = 0x3a8` | lib L1120 (c6 팀 판정 문맥) |

---

## 4. 비활성(DEAD) — 재활성 안 하면 무시
아래는 `*_ENABLED=false`로 꺼진 진단/폐기 훅. 마이그레이션 불필요(재활성 시에만 재탐색).
`INJECT_RVA_PASS1/2`(0xc76de5/0xc76aed, INJECT_DETOUR_ENABLED=false) · `WP_RVA_PASS1/2`(0xc76a96/0xc76d8e) · `PUSH_RVA`(0x1e4f310) · `BS_RVA`(0x19f0b90) · `RET_RVA`(0x1a33010) · `SIM_RVA`(0x204f810) · `VIEW_RVA`(0x1e84d5c) · `CAND_GATE_RVA`(0x1a35490, CAND_GATE_ON=false).

---

## 5. 검증 (마이그 후)
- 빌드 성공(exit 0) 후 인게임: **①개인전술서 모드템 지정칸 뜸 ②4번째 아이템 구매됨(적 포함) ③경기중 4번째 아이콘(양팀·창/전체화면) ④크래시 없음.**
- 각 훅/패치는 **프롤로그·시그 검증 내장** → 어긋나면 조용히 스킵(크래시 대신 기능 누락). 디버그로 확인하려면 `LOG_ENABLED=true`(lib L32) 후 `4items.txt`에서 "mismatch" 확인.
- ⚠ 프로덕션 배포 시 `LOG_ENABLED=false` 원복.

## 6. 파일별 변경 지점 요약
- **RVA 상수**: lib.rs L30·803·1429-48·2206-07·2253·2639, ui_inject.rs L12-14 → migrate_rva.py 후 교체.
- **바이트패치 주소**: lib.rs L2569-70·2594·2644-47 → ghidra-re 재도출.
- **rbp 변위**: lib.rs L1431-32 (C6_BB) → ghidra-re.
- **구조체 오프셋**: §3 — 메이저 버전업 아니면 대개 무변경.
