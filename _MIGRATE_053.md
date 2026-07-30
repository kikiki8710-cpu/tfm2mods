# 0.5.3 마이그레이션 지시서 — 모드별 세션 인계용

> 이 파일 하나만 보고 각 모드 세션이 작업할 수 있게 쓴 것. 생성 = `rva_catalog.py` → `fnindex.py` → `match2_053.py` → `match3_053.py` (전부 `C:\tfm2mods\`)

## 0. 버전

| | 0.5.2 (OLD = 모드 소스 베이스) | 0.5.3 (NEW) |
|---|---|---|
| buildid | 24310934 | **24451609** |
| exe | 69,209,088B | **74,970,624B** |
| sha256[:16] | 40b55c1b819dff50 | **6afff2cdb6bfa98e** |
| exe 백업 | `tfm2_0.5.2\` | `tfm2_0.5.3\` (+bundle, bundle_unpacked 1.1GB) |
| Ghidra MCP | `ghidra` (8080) | `ghidra_beta` (8081) |

## 1. 전 모드 공통 (반드시)

- **SDK = `C:\tfm2mods\sdk_053\mod-sdk`** (base_version 0.5.3). `build_inj.ps1` L29 `$SDK` 전환 필요.
- **toolchain 무변경** = `nightly-2026-05-24` (rustc 1.98.0-nightly 23a3312d9) — 재설치 불필요.
- ★**게임 rlib 236개 전원 내용 DIFF ⟹ RVA 0 모드까지 전 모드 재빌드 필수.** 재빌드만 하면 되는 모드 = `community_reaction_mod` · `Spectator_Chat` · `tfm2_meta_item_delegate` · save_probe · daram2 뷰플러스 9종.
- ⚠**빌드 플래그는 rustc 명령줄에 직접**: `-C opt-level=1 -C overflow-checks=off` (opt-level 2/3 = 재현 디투어 프레임 팽창 → STATUS_STACK_OVERFLOW).
- ★**신설 `libgame_ai` 크레이트** — 0.5.3에서 AI가 `game_core`에서 분리됐다(game_core rlib 407MB→333MB). AI 계열 함수는 위치뿐 아니라 코드가 바뀌었다고 보고 접근할 것.
- **대응 제외**: `tfm2_fog_damage_fix`(게임측에서 수정 — 마지막에 인게임 확인만) · `tfm2_transfer_tweak`(불필요 판정, 유저 지시 2026-07-29).

## 1b. ⛔ 이 표의 알려진 오답 — 반드시 먼저 읽을 것 (2026-07-29 추가)

- **`LOADER_RVA`(UI asset-get) = `0x91ab0` 은 오답이다.** 표에서 "확정" 등급이지만 근거인 "선두 12B push8 완전동일"은
  0.5.3 `.text` 에 **66,635회** 등장 = 변별력 0이었다(clone family 형제 혼동).
  **정답 = `0x2e1550`** — 문자열 xref 로 확정: `"asset/base/ui/layout/{main,strategy,training}"` 리터럴을 lea 하는 사이트의
  직후 call 타겟 집계가 **0x2e1550 ×31 만장일치**(0x91ab0 은 0표). 같은 절차를 0.5.2 에 돌리면 알려진 정답 `0x5ac950`이 ×28 로
  재생산되므로 방법 자체가 검증됐다. 재현 = `python C:	fm2mods\loader_053.py`.
  ⟹ 표에서 `0x91ab0` 으로 적힌 **전부**(ai_adjust `LOADER_RVA` / banpick_illust `RVA_ASSET_GET`·`RVA_ANIM_GET` /
  comptest_unlock `LOADER_RVA` / elemental_serpen `UILOADER_RVA` / item_tactics `LOADER_RVA`)를 재검증할 것.
  ⚠`ui_inject` 의 12B 프롤로그 검증은 이 오답을 **못 거른다**(어차피 push8이라 통과) → 조용히 엉뚱한 함수를 후킹한다.

- **교훈(일반화)**: 0.5.3 은 재컴파일로 프롤로그가 흔해졌다. **프롤로그 일치는 신원 근거가 아니다.**
  문자열 xref / 고유 imm(movabs) / 호출관계처럼 **변별력이 실제로 있는 지문**으로 확정할 것.

- **`ALLOC_RVA`**: 0.5.2 범용 `__rust_alloc(size, align)` 은 사라졌고 **align 별 전용 심**(예 align8 = `0xbb2bd0`)과
  **impl `0x28f7df0`** 로 갈라졌다. 심은 실패 시 `ud2`(abort), impl 은 null 반환 —
  null 을 정상 처리하는 코드라면 **impl 3인자 직접호출**(`f(0, 0, size)`)이 맞다.
  ✅**2026-07-29 확정·일원화**: 전 모드 정본 = **`0x28f7df0` + 3인자 `(rcx=무시, rdx=flags(0), r8=size)->rax`·실패 시 0 반환**
  (ai_adjust 도출 → serpen 독립 재도출 2:1 → **item_tactics도 교체·재빌드·배포 완료**). ⛔심 `0xbb2bd0`은 **채택 안 된 대안**. = MIGRATION §7.3 §11.4·§12.6

- ★★**clone family 식별의 결정적 지문 = 콜러 수**(2026-07-29 실사고에서 도출): 진입부 **24B가 완전 동일**해 바이트로는 형제를 못 가른다.
  실측 = 0.5.2 `0x5ac950`(**507**)/`0x99c860`(**67**)/`0x5ab7d0`(**77**) ↔ 0.5.3 정답 `0x2e1550`(**511**·규모 일치) vs 오답 `0x91ab0`(**2**·완전 불일치).
  ⟹ 후보가 clone family로 의심되면 **콜러 수 스펙트럼 대조 + 문자열-xref**로 확정할 것. = MIGRATION §7.3 §11.6

## 1c. ✅ 완료 모드 — 확정값 (2026-07-30, 이 3종 세션)

**공통 도구(재사용)**: `dov_053.py`(family 지문) · `dov_053b.py`(앵커맵 콜러-대응 **양방향 투표**) · `dov_053c.py`(역방향 순도)
· `mig3_053.py`(함수시작 일괄) · `illust_053.py/b`(프롤로그·.rdata·mid-func 필드) · `ct_053b/c/d.py`(byte-patch 재핀).
★**방법 검증** = 알려진 정답 LOADER 0x5ac950→0x2e1550 이 정방향 193/194·역방향 순도 98% 로 재현됨.

### ★asset-get clone family 최종 확정 (3모드 공통)
| 용도 | 0.5.2 | 0.5.3 | 근거 |
|---|---|---|---|
| layout 로더(LOADER/UILOADER) | `0x5ac950` | **`0x2e1550`** | 문자열-xref 만장일치 + 콜러 507→511 |
| 텍스처 게터(ASSET_GET) | `0x99c860` | **`0x143d50`** | 양방향 투표·역방향 순도 **100%(38/38)**·콜러 67→65 |
| 애님 게터(ANIM_GET) | `0x5ab7d0` | **`0x888fd0`** | 양방향 투표·역방향 순도 **100%(43/43)**·콜러 77→73 |
⟹ **`0x91ab0` 오답 파급 정리 완료**(전 모드에서 제거). ⚠진입 24B 가 449개 함수와 동일 = 바이트 구별 불가.

### alloc / dealloc (전 모드 정본)
- alloc = **`0x28f7df0`** · 3인자 `(rcx 무시, rdx=flags, r8=size)->rax`. 실측 = `GetProcessHeap()`→`HeapAlloc` tail-jmp 래퍼이고
  **exe 전체에서 HeapAlloc 을 참조하는 함수가 이것 하나뿐** = 신원 확정.
- dealloc = ⛔**0.5.3 에서 범용 함수가 인라인화로 소멸**(0.5.2 `0x25c4d90` 형태 부재). ⟹ 그 본문과 동일한
  **`HeapFree(GetProcessHeap(), 0, ptr)` 직접 호출**로 대체(align=1 할당이라 `ptr-8` 보정 분기 해당 없음).

### tfm2_draft_overlay ✅ 배포완 (dll 684,032B)
- ★**구 값 `LOADER 0x40f3d0` / `ANIM_GET 0x40e250` 는 0.5.2 시점에 이미 죽어 있었다** — 0.5.2 exe 에서 함수 시작이
  아니라 46KB 거대함수 `0x4041a0` 내부이고 콜러 0. ⟹ 0.5.2 마이그 때 이 파일만 갱신 누락됐던 것
  (= CURRENT.md "밴픽 asset-get copy 재확인 잔여" 의 실체). PARSER 도 0.5.1 값 `0x24b4590` 그대로였다.
- LOADER `0x2e1550` / ANIM_GET `0x888fd0` / PARSER `0x1a6530` / ALLOC `0x28f7df0`(3인자)
- ⛔**BANPICK_LOADER(copy #2) 폐지 = 0**. 0.5.2·0.5.3 모두 밴픽 레이아웃 문자열이 copy #1 로 ×19 수렴 = 분화 없음.
  ★같은 주소를 install_one 으로 두 번 걸면 **자기 체인 무한재귀**라 0 검사 가드를 넣었다(제거 금지).

### tfm2_banpick_illust ✅ 배포완 (dll 2,914,816B · v1.3.3)
- 함수시작 15종 전부 확정(역순도 대부분 100%). ⚠**표의 "확정" 2건이 오답이었다**:
  `RVA_IMG_COLOR` 표 `0x1875b0` → 실측 **`0x23b8150`** / `RVA_TEXT_BUILD` 표 `0x1165380` → 실측 **`0x186600`**.
  표가 "미해결"로 뒀던 5건(SUBMIT `0x1859f0`·SUBMIT_TEXT `0x185c70`·IMG_BUILD `0x187110`·IMG_SHADER `0x188a20`·NAME_GET `0x1c19520`)은 전부 해결.
- ★**프롤로그 변경 1건**: `ILLUST_GET` push4→push6 ⟹ **ORIG_LEN 19→13**, 배열 교체(안 고치면 명령 절단 = 즉사).
- geom `.rdata` 상수 6종 = **블록 통째 델타 `-0x4898b0`**(6/6 일관 + ZIG rip 타겟까지 교차검증).
- geom mid-func 6종 = **명령 시작이 아니라 필드(imm4/disp4) 위치** — 컨테이너 안에서 "같은 니모닉 + 같은 타겟 float"로 재산출.
- `RVA_SLOTS` = `0x3f11000`(0.5.3 .rdata 최장 0런 0x3f10294~0x3f27950 내부).

### tfm2_comptest_unlock ✅ 배포완 (dll 194,048B) — byte-patch **12/14**
- ⚠**orig 바이트도 바뀐다**(점프 거리·레지스터 할당 변경) ⟹ orig/fixed 를 실측으로 재생성해야 한다.
  예: `allow_dup_players` 거리 76→47 · `collect_err_gate` 6a→50 · `roster_count_gate` jmp rel32 재계산(0x14b→0x15f).
- ★핵심 `server_roster_min` = **`0x1830d2e`** — 문서화된 10B 시그는 0건(r15→rsi, dil→bl 변경).
  컨테이너 안 `lea r?,[r?+r?]` 직후 `cmp rdx,rax; jb` 가 유일. fixed 도 `mov rax,rsi; nop`(`48 89 f0 90`)로 재작성.
- ★btn5v5 3종은 **레지스터가 바뀌었다**(min_a r12→rbx `0x1987e64` / warn rbx→rdi `0x1987a7d`).
  ⛔imm 을 정규화한 자동매칭은 `cmp r12,0x30` 을 오답으로 집었다 — **imm 0xa 고정이 필수**.
- ⬜**미해결 2건 = `daily_remaining`(leaf·본문 시그 0건 = 함수 재작성) · `btn5v5_roster_min_b`** → `rva: 0` 으로 두고 스킵.
- ⬜훅 미해결 = `DISP_RVA`·`LOADING_RVA`·`EF1EA0_RVA` → **0 으로 두고 미설치**.
  ★이유: 0.5.3 에선 12B push8 프롤로그 검증이 **오답을 못 거른다**(66,635곳 통과) ⟹ 확정 못 한 주소는 반드시 0.

## 2. 이번 패치의 성격 — 읽고 시작할 것

- ⚠ **연속바이트 마스크시그(`migrate_rva.py`)는 이번에 전멸했다.** `.text` 44.0→48.6MB(+10.5%), 함수 120,995→132,960개(+11,965). 핵심 훅 6종을 160B 마스크시그로 찾으면 **전부 NONE**이 나온다. 그래서 `.pdata` 함수경계 + 명령 스켈레톤 해시 + 니모닉 코사인 + 국소 앵커 투표로 매칭했다.
- **0.5.3 함수는 0.5.2 대비 대체로 2~10% 크다**(재컴파일로 코드 자체가 변함). ⟹ **함수내 오프셋이 보존되지 않는다.**
- 신뢰도 등급:
  - **확정** = 명령 구조 일치 + 유일. 상수만 교체하면 된다. ⚠★**단 clone family(모노모픽 제네릭 형제) 함수에선 "확정" 등급도 신뢰 불가** — 실사고: `LOADER_RVA`(0.5.2 `0x5ac950`)를 "확정 `0x91ab0`"으로 매핑했으나 **오답**이었고 정답은 **`0x2e1550`**(문자열-xref로 도출). **서로 다른 0.5.2 함수가 같은 0.5.3 주소로 매핑되면 그 자체가 clone family 충돌 신호**(`0x91ab0`엔 3건이 몰림) ⟹ **문자열-xref 재검증 필수**. (2026-07-29 item_tactics 세션, MIGRATION §7.3 §11.6)
  - **유력** = 니모닉 코사인 최상위 + 2순위와 갭 + 크기비 정상. 대개 맞지만 **훅 설치 전 프롤로그(12B push8 등)·orig_len 경계·rip-rel 유무를 반드시 실측**할 것.
  - **추정** = 쌍둥이(제네릭 모노모픽) 함수를 주소 순서로 짝지은 것. Ghidra 확인 권장.
  - **미해결** = ghidra-re 필요. 억지로 넣지 말 것 — 신원검증 실패 시 미설치=inert가 안전하다.
- ⚠ **mid-func 사이트**(byte-patch imm·콜사이트)는 컨테이너 함수가 확정돼도 **오프셋을 그대로 옮기면 안 된다**(위 크기 변화). 컨테이너 안에서 원래 명령 패턴으로 재탐색해야 한다. `ai_adjust`의 byte-patch 62사이트가 전부 여기 해당.
- ⚠ 표의 `(inline)` 행은 소스 본문에서 긁어온 리터럴이라 **RVA가 아닌 상수(마스크·크기값)가 섞여 있다.** 실제 RVA인지 소스에서 확인하고 쓸 것. 상수 선언(`const RVA_*`)과 `patch_site` 행이 진짜 대상이다.

## 3. 모드별 표

## tfm2_ai_adjust

함수시작(훅 대상) **11/17 해결** · mid-func 사이트 156 · .text밖 22


### 함수 시작 RVA — ★주 대상(상수 선언)


| 상수 | 0.5.2 | → 0.5.3 | 신뢰도 | 근거 | 위치 |
|---|---|---|---|---|---|
| `RVA_RETREAT` | `0x1b94670` | `0xe00350` | 유력 | cos 0.9994/2nd 0.9971; 크기1.102 | src/rva_052.rs:15 |
| `RVA_GENERIC_BUILD` | `0x22b2280` | `0xe06c10` | 유력 | cos 0.9995/2nd 0.9969; 크기1.023 | src/rva_052.rs:25 |
| `RVA_FC59A0` | `0x1bdb3e0` | `0xe168d0` | 유력 | cos 0.9981/2nd 0.9946; 크기0.948 | src/rva_052.rs:29 |
| `RVA_CONDGATE` | `0x21338d0` | `0xc550b0` | 유력 | cos 0.9984/2nd 0.9804; 앵커22; 크기1.083 | src/rva_052.rs:43 |
| `RVA_MOVEPRI` | `0x2134240` | `0xc559e0` | 유력 | cos 0.998/2nd 0.9924; 앵커21; 크기1.015 | src/rva_052.rs:45 |
| `RVA_DISC18_HANDLER` | `0x2376320` | `0xd94d00` | 유력 | cos 0.9968/2nd 0.9962; 크기0.81 | src/rva_052.rs:64 |
| `RVA_DISC19_HANDLER` | `0x2380820` | `0xdece30` | 유력 | cos 0.9977/2nd 0.9972; 앵커16; 크기0.948 | src/rva_052.rs:66 |
| `RVA_ITEMNET_SCORER` | `0x1b9cce0` | `0x10587e0` | 유력 | cos 0.9994/2nd 0.9812; 앵커50; 크기1.027 | src/rva_052.rs:76 |
| `LOADER_RVA` | `0x5ac950` | ~~`0x91ab0` (확정)~~ → **`0x2e1550`** | ✅**정정·실측 확정**(0.5.3, 07-29 item_tactics 세션) | 구값 `0x91ab0`=**clone family 오답**. 정답=문자열-xref 16곳 수렴·진입 24B 동일 = MIGRATION §7.3 §11.6 | src/ui_inject_embed.rs:24 |
| `PARSER_RVA` | `0x24b5a00` | `0x1a6530` | 유력 | cos 0.9968/2nd 0.9958; 앵커13; 크기0.798 | src/ui_inject_embed.rs:25 |
| `ALLOC_RVA` | `0x25c4d30` | `후보: 0x2c26e30, 0x2b2f140, 0x28bc510` | 미해결 | cos 0.9661/2nd 0.9547; 크기0.634 | src/ui_inject_embed.rs:26 |

### 함수 시작 RVA — 참고(소스 본문 리터럴)
> ⚠ 소스에서 긁어온 값이라 RVA가 아닌 상수가 섞여 있다. 쓰기 전에 소스에서 용도를 확인할 것.

| 상수 | 0.5.2 | → 0.5.3 | 신뢰도 | 근거 | 위치 |
|---|---|---|---|---|---|
| `(inline)` | `0x9a1230` | `후보: 0x16270, 0x16c10, 0x290d0` | 미해결 | 형제 1 vs 후보 6 개수 불일치 → 미해결 | src/disc19_repro.rs:795 |
| `(inline)` | `0x1eacc00` | `후보: 0x11c7e90, 0x11ca1f0, 0x11d1e30` | 미해결 | cos 1.0/2nd 1.0; 앵커17; 크기0.913; 형제 1 vs 후보 5 개수 불일치 → 미해결 | src/disc19_repro.rs:1409 |
| `(inline)` | `0x23bd370` | `후보: 0xe2c710, 0xe2ec30` | 미해결 | 형제 1 vs 후보 2 개수 불일치 → 미해결 | src/disc19_repro.rs:1520 |
| `(inline)` | `0x1f23680` | `후보: 0x1756f0, 0x183ce0, 0x183ec0` | 미해결 | 형제 1 vs 후보 6 개수 불일치 → 미해결 | src/disc19_repro.rs:1546 |
| `(inline)` | `0x20958d0` | `후보: 0x2904520, 0xb63b0, 0x73b720` | 미해결 | cos 0.9984/2nd 0.9969; 크기0.826; 형제 7 vs 후보 5 개수 불일치 → 미해결 | src/disc19_repro.rs:1566 |
| `(inline)` | `0x2000000` | `0x29bbbf0` | 유력 | cos 1.0/2nd 0.9891; 크기1.038 | src/mem_safety.rs:130 |

### mid-func 사이트 (컨테이너 기준 재도출 필요)

| 상수 | 0.5.2 | 컨테이너 0.5.2 → 0.5.3 | 함수내 오프셋 | 컨테이너 신뢰도 |
|---|---|---|---|---|
| `(inline)` | `0x20def90` | `0x20def40` → `—` | +80 | 미해결 |
| `(inline)` | `0x1b934a4` | `0x1b92e40` → `0xdec6b0` | +1636 | 유력 |
| `(inline)` | `0x1b934b0` | `0x1b92e40` → `0xdec6b0` | +1648 | 유력 |
| `(inline)` | `0x1b934ec` | `0x1b92e40` → `0xdec6b0` | +1708 | 유력 |
| `(inline)` | `0x1b9351c` | `0x1b92e40` → `0xdec6b0` | +1756 | 유력 |
| `(inline)` | `0x1b9302c` | `0x1b92e40` → `0xdec6b0` | +492 | 유력 |
| `(inline)` | `0x1b93152` | `0x1b92e40` → `0xdec6b0` | +786 | 유력 |
| `(inline)` | `0x1b933d8` | `0x1b92e40` → `0xdec6b0` | +1432 | 유력 |
| `(inline)` | `0x1bdac25` | `0x1bdaaa0` → `0xdf9320` | +389 | 유력 |
| `(inline)` | `0x1bdac95` | `0x1bdaaa0` → `0xdf9320` | +501 | 유력 |
| `(inline)` | `0x2376e86` | `0x2376320` → `0xd94d00` | +2918 | 유력 |
| `(inline)` | `0x23777fe` | `0x2376320` → `0xd94d00` | +5342 | 유력 |
| `(inline)` | `0x237780a` | `0x2376320` → `0xd94d00` | +5354 | 유력 |
| `(inline)` | `0x2126ae3` | `0x2126610` → `—` | +1235 | 미해결 |
| `(inline)` | `0x22b2555` | `0x22b2280` → `0xe06c10` | +725 | 유력 |
| `(inline)` | `0x22b2ca5` | `0x22b2280` → `0xe06c10` | +2597 | 유력 |
| `(inline)` | `0x22b2bb1` | `0x22b2280` → `0xe06c10` | +2353 | 유력 |
| `(inline)` | `0x22b58ad` | `0x22b2280` → `0xe06c10` | +13869 | 유력 |
| `(inline)` | `0x2398342` | `0x2398240` → `—` | +258 | 미해결 |
| `(inline)` | `0x2398ef3` | `0x2398240` → `—` | +3251 | 미해결 |
| `(inline)` | `0x2398f3c` | `0x2398240` → `—` | +3324 | 미해결 |
| `(inline)` | `0x23ad9d7` | `0x23ad980` → `0xcdd010` | +87 | 확정 |
| `(inline)` | `0x23ba8f3` | `0x23ba8d0` → `—` | +35 | 미해결 |
| `(inline)` | `0x22b43ae` | `0x22b2280` → `0xe06c10` | +8494 | 유력 |
| `(inline)` | `0x22e3cdf` | `0x22dd9a0` → `0xcc9d70` | +25407 | 유력 |
| `(inline)` | `0x22e3cf0` | `0x22dd9a0` → `0xcc9d70` | +25424 | 유력 |
| `(inline)` | `0x22e3cf6` | `0x22dd9a0` → `0xcc9d70` | +25430 | 유력 |
| `(inline)` | `0x22e3d00` | `0x22dd9a0` → `0xcc9d70` | +25440 | 유력 |
| `(inline)` | `0x22e3d06` | `0x22dd9a0` → `0xcc9d70` | +25446 | 유력 |
| `(inline)` | `0x22e3d10` | `0x22dd9a0` → `0xcc9d70` | +25456 | 유력 |
| `(inline)` | `0x22e3d16` | `0x22dd9a0` → `0xcc9d70` | +25462 | 유력 |
| `(inline)` | `0x22e3d2b` | `0x22dd9a0` → `0xcc9d70` | +25483 | 유력 |
| `(inline)` | `0x22e3d2f` | `0x22dd9a0` → `0xcc9d70` | +25487 | 유력 |
| `(inline)` | `0x22e3d33` | `0x22dd9a0` → `0xcc9d70` | +25491 | 유력 |
| `(inline)` | `0x22edb5f` | `0x22e6460` → `—` | +30463 | 미해결 |
| `(inline)` | `0x22edb65` | `0x22e6460` → `—` | +30469 | 미해결 |
| `(inline)` | `0x22edb6b` | `0x22e6460` → `—` | +30475 | 미해결 |
| `(inline)` | `0x22edb71` | `0x22e6460` → `—` | +30481 | 미해결 |
| `(inline)` | `0x22edb7b` | `0x22e6460` → `—` | +30491 | 미해결 |
| `(inline)` | `0x22effff` | `0x22efed0` → `0xcd4b40` | +303 | 확정 |
| `(inline)` | `0x22f0005` | `0x22efed0` → `0xcd4b40` | +309 | 확정 |
| `(inline)` | `0x22f000b` | `0x22efed0` → `0xcd4b40` | +315 | 확정 |
| `(inline)` | `0x22f0011` | `0x22efed0` → `0xcd4b40` | +321 | 확정 |
| `(inline)` | `0x22f0017` | `0x22efed0` → `0xcd4b40` | +327 | 확정 |
| `(inline)` | `0x22f001d` | `0x22efed0` → `0xcd4b40` | +333 | 확정 |
| `(inline)` | `0x22f0023` | `0x22efed0` → `0xcd4b40` | +339 | 확정 |
| `(inline)` | `0x23a0c21` | `0x23a04d0` → `0xc7f640` | +1873 | 유력 |
| `(inline)` | `0x23a0c27` | `0x23a04d0` → `0xc7f640` | +1879 | 유력 |
| `(inline)` | `0x23a0c2d` | `0x23a04d0` → `0xc7f640` | +1885 | 유력 |
| `(inline)` | `0x23a0c33` | `0x23a04d0` → `0xc7f640` | +1891 | 유력 |
| `(inline)` | `0x23a0c39` | `0x23a04d0` → `0xc7f640` | +1897 | 유력 |
| `(inline)` | `0x23a0c41` | `0x23a04d0` → `0xc7f640` | +1905 | 유력 |
| `(inline)` | `0x23a0c47` | `0x23a04d0` → `0xc7f640` | +1911 | 유력 |
| `SIMUNCHUNK_RVA` | `0x19b40c3` | `0x19b3e70` → `0x25b12e0` | +595 | 유력 |
| `(inline)` | `0x2380e16` | `0x2380820` → `0xdece30` | +1526 | 유력 |
| `(inline)` | `0x2380e22` | `0x2380820` → `0xdece30` | +1538 | 유력 |
| `(inline)` | `0x2380e2e` | `0x2380820` → `0xdece30` | +1550 | 유력 |
| `(inline)` | `0x2380e3c` | `0x2380820` → `0xdece30` | +1564 | 유력 |
| `(inline)` | `0x2380e1c` | `0x2380820` → `0xdece30` | +1532 | 유력 |
| `(inline)` | `0x2380e28` | `0x2380820` → `0xdece30` | +1544 | 유력 |

(외 96건 — 전체는 `_rva_final_053.json`)

## tfm2_banpick_illust

함수시작(훅 대상) **10/15 해결** · mid-func 사이트 6 · .text밖 9


### 함수 시작 RVA — ★주 대상(상수 선언)


| 상수 | 0.5.2 | → 0.5.3 | 신뢰도 | 근거 | 위치 |
|---|---|---|---|---|---|
| `RVA_FX_SET` | `0x11e2370` | `0x1bd8e50` | 유력 | cos 0.9943/2nd 0.9842; 앵커45; 크기1.128 | src/showcase.rs:19 |
| `RVA_CARD_DRAW` | `0x11f9030` | `0x1bee8e0` | 확정 |  | src/showcase.rs:20 |
| `RVA_ILLUST_GET` | `0xfdabe0` | `0x1e91400` | 유력 | cos 0.9973/2nd 0; 앵커4; 크기1.013 | src/showcase.rs:21 |
| `RVA_SUBMIT` | `0x248b1c0` | `후보: 0x1859f0, 0x185f40, 0x1140660` | 미해결 | cos 0.9839/2nd 0.9828; 앵커53; 크기1.178; 형제 1 vs 후보 2 개수 불일치 → 미해결 | src/showcase.rs:22 |
| `RVA_SUBMIT_TEXT` | `0x248b400` | `후보: 0x1859f0, 0x185f40, 0x185c70` | 미해결 | cos 0.9949/2nd 0.994; 앵커53; 크기0.981; 형제 1 vs 후보 3 개수 불일치 → 미해결 | src/showcase.rs:23 |
| `RVA_IMG_BUILD` | `0x248c130` | `후보: 0x2a2c9b0, 0x2a685f0, 0x2d85250` | 미해결 | cos 0.9779/2nd 0.9654; 크기0.84 | src/showcase.rs:24 |
| `RVA_IMG_UV` | `0x248c7c0` | `0x186f70` | 확정 |  | src/showcase.rs:25 |
| `RVA_IMG_FLAG` | `0x248cd40` | `0x187420` | 확정 |  | src/showcase.rs:26 |
| `RVA_IMG_COLOR` | `0xff0c20` | `0x1875b0` | 유력 | cos 0.9963/2nd 0.9928; 크기1.079 | src/showcase.rs:27 |
| `RVA_IMG_SHADER` | `0x248e850` | `후보: 0xeabd50, 0xeae7d0, 0xeb32d0` | 미해결 | cos 0.9927/2nd 0.9929; 크기1.2; 형제 1 vs 후보 5 개수 불일치 → 미해결 | src/showcase.rs:28 |
| `RVA_TEXT_BUILD` | `0x248c1e0` | `0x1165380` | 확정 |  | src/showcase.rs:29 |
| `RVA_NAME_GET` | `0x1217630` | `후보: 0x2bc46a0, 0x12550f0, 0x1f2e390` | 미해결 | cos 0.9953/2nd 0.9951; 크기1.151; 형제 1 vs 후보 5 개수 불일치 → 미해결 | src/showcase.rs:30 |
| `RVA_ASSET_GET` | `0x99c860` | ~~`0x91ab0` (확정)~~ → **오답 의심·미해결 취급** | ⚠**신뢰 불가**(0.5.3, 07-29) | 서로 다른 0.5.2 함수 3종이 전부 `0x91ab0`으로 매핑됨 = **clone family 충돌**. 그중 `LOADER_RVA`는 오답임이 실증됨(§11.6) ⟹ **문자열-xref 재검증 필수** | src/showcase.rs:31 |
| `RVA_ANIM_GET` | `0x5ab7d0` | ~~`0x91ab0` (확정)~~ → **오답 의심·미해결 취급** | ⚠**신뢰 불가**(0.5.3, 07-29) | 위와 동일(clone family 충돌) | src/showcase.rs:32 |
| `RVA_SPRITE_CALC` | `0x121aca0` | `0x1c1e4e0` | 유력 | cos 0.9938/2nd 0.9879; 앵커39; 크기1.028 | src/showcase.rs:33 |

### mid-func 사이트 (컨테이너 기준 재도출 필요)

| 상수 | 0.5.2 | 컨테이너 0.5.2 → 0.5.3 | 함수내 오프셋 | 컨테이너 신뢰도 |
|---|---|---|---|---|
| `RVA_I_SNAP_H` | `0x124e2ba` | `0x124db10` → `0x1c52950` | +1962 | 유력 |
| `RVA_D_SNAP_W` | `0x124e2c2` | `0x124db10` → `0x1c52950` | +1970 | 유력 |
| `RVA_D_CUT_LO` | `0x1201e19` | `0x1201d90` → `0x1bf89a0` | +137 | 유력 |
| `RVA_D_CUT_HI` | `0x1201e27` | `0x1201d90` → `0x1bf89a0` | +151 | 유력 |
| `RVA_D_ZIG_X1` | `0x124e8cf` | `0x124db10` → `0x1c52950` | +3519 | 유력 |
| `RVA_D_ZIG_X2` | `0x124efa1` | `0x124db10` → `0x1c52950` | +5265 | 유력 |

## tfm2_banpick_order

> ✅★**마이그 완료(2026-07-30) — 아래 표는 전부 capstone+pefile 디스크 exe 실측으로 확정·정정됨**(구 "유력/추정/미해결" 등급 폐기). **정본 = `MODS\MIGRATION.md §7.3 §14`** · 배포 = v1.1.0·dll 2,538,496B / ⬜인게임 미검증.
> ★**구조 변화**: 훅 A `0x1cd9380`(phase getter) = **0.5.3에서 함수째 소멸·인라인화** → 신설 leaf **A′ `0x1bf3dd0` `scene_step`**(단계 enum 0밴/1픽/2완료/0xff)으로 대체 + **훅 G 신설**(AI턴 `0x1827e00` 인라인 phase 38B 패치). phase 인라인 복제본 11→30개, phase_from 직접 콜러 26→3.

### 함수 시작 RVA — ★주 대상(상수 선언) · **전부 실측 확정(07-30)**


| 상수 | 0.5.2 | → 0.5.3 | 신뢰도 | 근거 | 위치 |
|---|---|---|---|---|---|
| `RVA_PANIC_HOOK` | `0x25d4764` | `0x28f2f34` | ✅확정(실측) | 프롤로그·크기 일치 | src/diag.rs:636 |
| `RVA_APPLIER` | `0x11e2140` | `0x1bd8c20` | ✅확정(실측) | 크기 547 동일·disp 히스토그램 완전 동일·프롤로그 12B 동일 | src/hooks.rs:25 |
| `RVA_PHASE_SCENE`(★신설 A′) | — (0.5.2 A `0x1cd9380` 소멸) | **`0x1bf3dd0`** | ✅확정(실측) | 씬 오프셋 지문(`0x148/0x160/0x178/0x190`·`0xce`·`0x3c0`)·콜러 23 | src/hooks.rs |
| `RVA_PHASE_FROM`(B) | `0x1d04120` | **`0x167c0e0`** | ✅확정(실측) | 진입 시그 `4d 01 c0 0f b6 c2 48 8d 15` exe 유일 1히트 | src/hooks.rs |
| `RVA_APP_PICK_T1` | `0x11ce240` | `0x1bc47f0` | ✅확정(실측) | ~~추정(순서)~~ → 씬 `+0x168/0x170/0x178` 지문 | src/hooks.rs:275 |
| `RVA_APP_PICK_T2` | `0x11ce400` | `0x1bc4980` | ✅확정(실측) | ~~추정(순서)~~ → `+0x180/0x188/0x190` | src/hooks.rs:276 |
| `RVA_APP_BAN_T1` | `0x120c020` | `0x1c028d0` | ✅확정(실측) | ~~추정(순서)~~ → `+0x138/0x140/0x148` | src/hooks.rs:277 |
| `RVA_APP_BAN_T2` | `0x120c1d0` | `0x1c02a50` | ✅확정(실측) | ~~추정(순서)~~ → `+0x150/0x158/0x160` | src/hooks.rs:278 |
| `RVA_TRANSITION` | `0x11d8ef0` | `0x1bcf010` | ✅확정(실측) | 프롤로그 동일·크기비 1.107 | src/hooks.rs:279 |
| `RVA_BANNER` | `0x11df9f0` | `0x1bd63a0` | ✅확정(실측) | `+0x43e`/`+0x380` 지문. ⚠**프롤로그 변경**(`56 57 53 48 83 ec 30`) — 호출만 해서 무관 | src/hooks.rs:357 |
| `RVA_LINEUP` | `0x11cedb0` | `0x1bc52b0` | ✅확정(실측) | 프롤로그 12B 동일 | src/hooks.rs:384 |
| `RVA_COMMIT` | `0x1d075d0` | `0x167fdd0` | ✅확정(실측) | disp 히스토그램 완전 동일 | src/hooks.rs:402 |
| `RVA_TURN_ORACLE`(D′) | `0x1d07cf0` | **`0x1680500`** | ✅확정(실측) | 진입 13B 완전 동일·콜러 5↔5 대응 | src/hooks.rs |
| 픽테이블(.rdata) | `0x38397a8` | **`0x3277c70`** | ✅확정(실측) | 28B 내용 동일·하위 오프셋 `+0/+4/+0xa/+0x12` 동일 | src/hooks.rs |

### mid-func 사이트 — **전부 컨테이너 내 패턴 재탐색으로 실측 확정(07-30·오프셋 이식 아님)**

| 상수 | 0.5.2 | 0.5.3 (실측) | 컨테이너 0.5.2 → 0.5.3 | 비고 |
|---|---|---|---|---|
| `PANIC_SITES` 6종 | `0x11da680` 등 | ⬜**미재핀 → 소스에서 0으로 비움** | `0x11d8ef0` → `0x1bcf010` | 라벨 전용·기능 무영향 |
| `RVA_AI_SITE1` | `0x1c04389` | **`0x10a04e2`** | `0x1c041c0` → **`0x10a0320`** | SIG1 바이트 동일·exe 유일. al 미러 `[rbp+0x6f]` 불변 |
| `RVA_AI_JOIN1` | `0x1c04475` | **`0x10a05f0`** | 〃 | |
| `RVA_AI_SITE2` | `0x1c07938` | **`0x10a3cf8`** | `0x1c07880` → **`0x10a3c40`** | ~~미해결~~ → SIG2 유일 |
| `RVA_AI_JOIN2` | `0x1c07a09` | **`0x10a3dc9`** | 〃 | |
| ★`RVA_G_AI_TURN_SITE`(신설) | — (0.5.2엔 훅 A가 담당) | **`0x1828213`** | 서버 AI턴 `0xebe530` → **`0x1827e00`** | 인라인 phase **38B 패치**·창 231B. 스택사본 total `[rbp+0x5eb0]`·rule `[rbp+0x5d61]`·ban `[rbp+0x5d58]` |
| ★`RVA_G_AI_TURN_JOIN`(신설) | — | **`0x18282fa`** | 〃 | 합류 `mov [rbp+0x5ebf],al` |
| `RVA_SFX_SITE` | `0x1251303` | **`0x1c56245`** | `0x1250370` → `0x1c55300` | 창 79B 동일. ⚠**씬 스택슬롯 `[rbp+0x12b0]`→`[rbp+0x12d0]`** |
| `RVA_SFX_END` | `0x1251352` | **`0x1c56294`** | 〃 | |
| sfx 문자열 ban / pick | `0x373d596` / `0x373d5b2` | **`0x32adfb6` / `0x32adfd2`** | — | 문자열 검색 |

## tfm2_comptest_unlock

함수시작(훅 대상) **8/14 해결** · mid-func 사이트 29 · .text밖 4


### 함수 시작 RVA — ★주 대상(상수 선언)


| 상수 | 0.5.2 | → 0.5.3 | 신뢰도 | 근거 | 위치 |
|---|---|---|---|---|---|
| `DISP_RVA` | `0xd3f780` | `후보: 0x2c9fec0, 0x2ca2880, 0x2ca72e0` | 미해결 | cos 0.9768/2nd 0.9768; 크기0.609; 형제 1 vs 후보 5 개수 불일치 → 미해결 | src/tfm2_comptest_unlock.rs:292 |
| `CT_REGION_LO` | `0xe7ccd0` | `0x17e0240` | 유력 | cos 0.9998/2nd 0.9777; 앵커46; 크기1.096 | src/tfm2_comptest_unlock.rs:364 |
| `RUN_RVA` | `0xd0a440` | `0x18f1180` | 유력 | cos 0.9968/2nd 0.9947; 앵커44; 크기1.151 | src/tfm2_comptest_unlock.rs:526 |
| `LOADING_RVA` | `0xd186f0` | `후보: 0x80e9d0, 0x2249030, 0x22199b0` | 미해결 | cos 0.9913/2nd 0.9912; 크기0.694; 형제 1 vs 후보 5 개수 불일치 → 미해결 | src/tfm2_comptest_unlock.rs:604 |
| `FN_DD_SETOPT_RVA` | `0x242f250` | ~~미해결~~ → **`0x1bfc80`** | ✅**실측 확정**(0.5.3, 07-29 item_tactics 세션) | 직접 콜러 103개=구 exe와 동수 + 오프셋 지문 불변(+0x1788/+0x1528·30·38/+0x1570·78/원소 0xf8/입력 stride 0x28). ⚠**프롤로그 `55 56 57 48 83 ec 70`→`55 41 57 41 56 56 57 53 48 81 ec 88`(7B→12B) 교체 필요**. 드롭다운 `runner+0x1150`/`+0x1154`는 **불변** | src/tfm2_comptest_unlock.rs:830 |
| `ITEMCONV_RVA` | `0xed8770` | `0x18429d0` | 확정 |  | src/tfm2_comptest_unlock.rs:970 |
| `COLLECT_RVA` | `0xd0bd80` | `후보: 0xcde820, 0x28f4e20, 0xb4bde0` | 미해결 | cos 0.9967/2nd 0.9946; 크기0.865 | src/tfm2_comptest_unlock.rs:1042 |
| `EF1EA0_RVA` | `0xe58c30` | `후보: 0x1927fa0, 0x2847b30, 0x1befb10` | 미해결 | cos 0.9776/2nd 0.9775; 크기0.867; 형제 1 vs 후보 5 개수 불일치 → 미해결 | src/tfm2_comptest_unlock.rs:1125 |
| `ATH_GET_SC_RVA` | `0xe3b200` | `0x1794280` | 확정 |  | src/tfm2_comptest_unlock.rs:1133 |
| `ORACLE_RVA` | `0x1d94720` | `0xeb6590` | 유력 | cos 0.9974/2nd 0.9856; 앵커10; 크기1.155 | src/tfm2_comptest_unlock.rs:1192 |
| `SLOT_RVA` | `0xd1acf0` | `0x1904640` | 확정 |  | src/tfm2_comptest_unlock.rs:1383 |
| `LOADER_RVA` | `0x5ac950` | ~~`0x91ab0` (확정)~~ → **`0x2e1550`** | ✅**정정·실측 확정**(0.5.3, 07-29 item_tactics 세션) | 구값 `0x91ab0`=**clone family 오답**. 정답=문자열-xref 16곳 수렴 = MIGRATION §7.3 §11.6 | src/ui_inject.rs:32 |
| `PARSER_RVA` | `0x24b5a00` | `0x1a6530` | ✅**실측 확정**(07-29) | 3인자 계약·노드 stride 0x90 유지 | src/ui_inject.rs:33 |
| `ALLOC_RVA` | `0x25c4d30` | ~~`0xbb2bd0` 병존~~ → **`0x28f7df0`**(3인자) | ✅**일원화 확정**(07-29) | 전 모드 정본 = impl 직접호출 `(rcx=무시, rdx=flags 0, r8=size)->rax`·**실패 시 0 반환**. ⛔심 `0xbb2bd0`은 OOM 시 abort = 미채택 = §11.4·§12.6 | src/ui_inject.rs:34 |

### mid-func 사이트 (컨테이너 기준 재도출 필요)

| 상수 | 0.5.2 | 컨테이너 0.5.2 → 0.5.3 | 함수내 오프셋 | 컨테이너 신뢰도 |
|---|---|---|---|---|
| `no_stamina_cost` | `0xe93b2d` | `0xe7ccd0` → `0x17e0240` | +93789 | 유력 |
| `daily_inc_gate` | `0xe8cb20` | `0xe7ccd0` → `0x17e0240` | +65104 | 유력 |
| `server_dedup_real` | `0xec7758` | `0xec71b0` → `—` | +1448 | 미해결 |
| `allow_dup_players` | `0xd00ee5` | `0xd00a80` → `—` | +1125 | 미해결 |
| `server_dedup` | `0xe8b5fa` | `0xe7ccd0` → `0x17e0240` | +59690 | 유력 |
| `btn5v5_roster_min_a` | `0xd967cf` | `0xd95450` → `0x19866f0` | +4991 | 확정 |
| `btn5v5_roster_min_b` | `0xcf7b68` | `0xcf7970` → `—` | +504 | 미해결 |
| `btn5v5_warn_text` | `0xd9662c` | `0xd95450` → `0x19866f0` | +4572 | 확정 |
| `server_roster_min` | `0xec768e` | `0xec71b0` → `—` | +1246 | 미해결 |
| `roster_count_gate` | `0xd0a74c` | `0xd0a440` → `0x18f1180` | +780 | 유력 |
| `collected_gate` | `0xd0a740` | `0xd0a440` → `0x18f1180` | +768 | 유력 |
| `collect_err_gate` | `0xd0a728` | `0xd0a440` → `0x18f1180` | +744 | 유력 |
| `run_push_gate` | `0xd0adf1` | `0xd0a440` → `0x18f1180` | +2481 | 유력 |
| `INSERT_RVA` | `0xcabac0` | `0xcabab0` → `—` | +16 | 미해결 |
| `CT_CLIENT_LO` | `0xcf0000` | `0xcee980` → `0x18cf8a0` | +5760 | 유력 |
| `CT_CLIENT_HI` | `0xda0000` | `0xd9ffc0` → `0x1992270` | +64 | 유력 |
| `ATH_ID_HI` | `0x100000` | `0xfe830` → `0x3f4760` | +6096 | 확정 |
| `(inline)` | `0xd00ed0` | `0xd00a80` → `—` | +1104 | 미해결 |
| `ENQ_RVA` | `0xcb9c80` | `0xcb9aa0` → `0x1b8a180` | +480 | 유력 |
| `SRV_RVA` | `0x13d4af0` | `0x13d44e0` → `0x240b2e0` | +1552 | 확정 |
| `DEDUP_INS_RVA` | `0xca75f0` | `0xc9cad0` → `0x1874b90` | +43808 | 확정 |
| `SPAWN_CP_RVA` | `0x13c71b0` | `0x13c6a90` → `0x23fd0f0` | +1824 | 유력 |
| `PUSH_RVA` | `0x101cc08` | `0x1015670` → `—` | +30104 | 미해결 |
| `(inline)` | `0xf794c0` | `0xf79470` → `—` | +80 | 미해결 |
| `(inline)` | `0x20566c0` | `0x20566a0` → `0x15530d0` | +32 | 유력 |
| `ATH_GET_RVA` | `0x402840` | `0x4025c0` → `0xb89b20` | +640 | 유력 |
| `ATH_GET_JE_TARGET_RVA` | `0x4028fb` | `0x4025c0` → `0xb89b20` | +827 | 유력 |
| `CT_ARM_LO` | `0x13e1c00` | `0x13e0b60` → `0x2417ea0` | +4256 | 유력 |
| `CT_ARM_HI` | `0x13ea200` | `0x13e74d0` → `0x241e920` | +11568 | 확정 |

## tfm2_draft_overlay

함수시작(훅 대상) **0/0 해결** · mid-func 사이트 5 · .text밖 0


### mid-func 사이트 (컨테이너 기준 재도출 필요)

| 상수 | 0.5.2 | 컨테이너 0.5.2 → 0.5.3 | 함수내 오프셋 | 컨테이너 신뢰도 |
|---|---|---|---|---|
| `ANIM_GET_RVA` | `0x40e250` | `0x4041a0` → `—` | +41136 | 미해결 |
| `LOADER_RVA` | `0x40f3d0` | `0x4041a0` → `—` | +45616 | 미해결 |
| `BANPICK_LOADER_RVA` | `0xeb17d0` | `0xeb1780` → `—` | +80 | 미해결 |
| `PARSER_RVA` | `0x24b4590` | `0x24b4470` → `0x1a4f00` | +288 | 확정 |
| `ALLOC_RVA` | `0x25c5a40` | `0x25c5a00` → `—` | +64 | 미해결 |

## tfm2_elemental_serpen

함수시작(훅 대상) **9/14 해결** · mid-func 사이트 3 · .text밖 0


### 함수 시작 RVA — ★주 대상(상수 선언)


| 상수 | 0.5.2 | → 0.5.3 | 신뢰도 | 근거 | 위치 |
|---|---|---|---|---|---|
| `SERPEN_RVA` | `0x21f8ca0` | `0x1535810` | 확정 |  | src/lib.rs:34 |
| `MOBATICK_RVA` | `0x230c290` | `후보: 0xeeeac0, 0x2328370, 0x1831020` | 미해결 | cos 0.9901/2nd 0.9802; 앵커38; 크기0.875 | src/lib.rs:350 |
| `SPAWN_HOOKS` | `0x539f40` | `0xabd340` | 추정(순서) | cos 0.995/2nd 0.995; 앵커58; 크기1.08; 형제 2개 ↔ 후보 2개 순서대응 | src/lib.rs:405 |
| `SPAWN_HOOKS` | `0x53aae0` | `0xabdf60` | 추정(순서) | cos 0.9951/2nd 0.9951; 앵커58; 크기1.08; 형제 2개 ↔ 후보 2개 순서대응 | src/lib.rs:405 |
| `LAUNCHER_RVA` | `0x1d96870` | ~~미해결~~ → **`0xeb8810`** | ✅**실측 확정**(0.5.3, 07-29 — **item_tactics `CL_LAUNCHER_RVA`와 동일 함수**) | 콜러 9/9 대응·프롤로그 17B(프레임 0x165c8→**0x25108**) = MIGRATION §7.3 §11.1·§11.6 | src/lib.rs:414 |
| `UILOADER_RVA` | `0x5ac950` | ~~`0x91ab0` (확정)~~ → **`0x2e1550`** | ✅**정정·실측 확정**(0.5.3, 07-29) | 구값=**clone family 오답**. 정답=문자열-xref 16곳 수렴 = §11.6 | src/lib.rs:513 |
| `UIPARSER_RVA` | `0x24b5a00` | `0x1a6530` | ✅**실측 확정**(07-29) | 3인자 계약·노드 stride 0x90 유지 | src/lib.rs:514 |
| `UIALLOC_RVA` | `0x25c4d30` | ~~`0xbb2bd0`~~ → **`0x28f7df0`**(3인자) | ✅**일원화 확정**(07-29) | impl 직접호출 `(rcx=무시, rdx=0, r8=size)->rax`·실패 시 0 반환. ⛔심 `0xbb2bd0`=미채택(OOM abort) = §11.4·§12.6·§13.4 | src/lib.rs:515 |
| `RENDER_STEP_RVA` | `0x811500` | `0x960df0` | 유력 | cos 0.9986/2nd 0.9728; 앵커56; 크기1.13 | src/lib.rs:717 |
| `RUNNER_CTOR_RVA` | `0x1d981e0` | `후보: 0x2413d10, 0x18f6c30, 0x1925ab0` | 미해결 | cos 0.9921/2nd 0.9906; 크기1.136; 형제 1 vs 후보 3 개수 불일치 → 미해결 | src/lib.rs:744 |
| `DMGA_RVA` | `0x22164a0` | `0xfdbbb0` | 확정 |  | src/lib.rs:1707 |
| `DMGB_RVA` | `0x22d2b20` | `0x12c3bb0` | 유력 | cos 0.9997/2nd 0.9953; 앵커13; 크기1.063 | src/lib.rs:1710 |
| `KEYRES_RVA` | `0xc2f990` | `0x1b0aba0` | 확정 |  | src/lib.rs:1902 |
| `ARG_STR_RVA` | `0xfef190` | `후보: 0x1e7610, 0x1228a90, 0x1a2ed40` | 미해결 | cos 0.9959/2nd 0.9959; 크기1.0; 형제 1 vs 후보 4 개수 불일치 → 미해결 | src/lib.rs:2427 |

### mid-func 사이트 (컨테이너 기준 재도출 필요)

| 상수 | 0.5.2 | 컨테이너 0.5.2 → 0.5.3 | 함수내 오프셋 | 컨테이너 신뢰도 |
|---|---|---|---|---|
| `LAUNCHER_RET_A` | `0x759c36` | `0x74d510` → `0x997740` | ~~+50982 이식~~ → ✅**`0x9a3287`**(패턴 재탐색) | 실측 확정(07-29) |
| `LAUNCHER_RET_B` | `0x75e5cf` | `0x74d510` → `0x997740` | ~~+69823 이식~~ → ✅**`0x9a7b03`**(패턴 재탐색) | 실측 확정(07-29) |
| `LAUNCHER_RET_C` | `0x1555215` | `0x1554930` → `0x229a410` | ~~+2277 이식~~ → ✅**`0x229ad94`**(패턴 재탐색) | 실측 확정(07-29) |

⚠**위 3건은 오프셋 이식이 아니라 컨테이너 내 명령 패턴 재탐색 결과**(§2 "함수내 오프셋 비보존" 규칙). item_tactics 세션 실측 = MIGRATION §7.3 §11.2·§11.6.

## tfm2_item_tactics

~~함수시작(훅 대상) **6/12 해결**~~ → ✅**마이그 완료·빌드·배포완(2026-07-29, dll 504,832B·mod_info v2.5.0) / ⬜인게임 미검증**. 아래 표는 **실측 확정값으로 정정 완료**(자동매칭 결과 아님). 경위·근거 전문 = **`MODS\MIGRATION.md §7.3 §11`**(정본). 재핀 도구 = `C:\tfm2mods\_it_scan.py`(capstone+pefile, 재사용).


### 함수 시작 RVA — ★주 대상(상수 선언) — **전부 실측 확정(0.5.3, 07-29)**


| 상수 | 0.5.2 | → 0.5.3 | 신뢰도 | 근거 | 위치 |
|---|---|---|---|---|---|
| `FN_DD_SETOPT_RVA` | `0x242f250` | ~~미해결~~ → **`0x1bfc80`** | ✅**실측 확정**(0.5.3, 07-29) | 직접 콜러 103개=구 exe와 동수 + 오프셋 지문 불변(+0x1788/+0x1528·30·38/+0x1570·78/원소 0xf8/입력 stride 0x28). ⚠**프롤로그 7B→12B 교체 필요** | src/lib.rs:32 |
| `RVA_REALLOC` | `0x25c4dd0` | `0x28e3b10` | ✅**실측 확정** | 진입 112B 마스크시그 유일 1히트 + 본문 동형 | src/lib.rs:1772 |
| `CL_LAUNCHER_RVA` | `0x1d96870` | ~~미해결~~ → **`0xeb8810`** | ✅**실측 확정**(0.5.3, 07-29) | 콜러 9/9 대응·렌더 씬빌더 2회 호출·seedctor 콜. 프롤로그 17B `55 41 57 41 56 41 55 41 54 56 57 53 b8 08 51 02 00`(프레임 0x165c8→**0x25108**). ★serpen `LAUNCHER_RVA`도 **동일 함수** | src/lib.rs:1813 |
| `SEEDCTOR_RVA` | `0x22c1da0` | `0x12b9ab0` | ✅**실측 확정** | 프롤로그 12B 동일(push8)·프레임 0x11b58→0x11b98·launcher 내부 콜 라인대응 | src/lib.rs:1928 |
| `SPAWN_RVA` | `0x1d9e0e0` | ~~미해결~~ → **`0xebfe50`**(~`0xec0302`) | ✅**실측 확정**(07-29) — 단 **게이트 OFF 유지** | 본문 명령 1:1 대응·콜러 컨테이너 `0x1d94640`→**`0xeb6480`**(+0x91 콜 @`0xeb6511`)·직접 콜러 15곳. ⚠**프롤로그 7push+chkstk → 8push(12B)+`sub rsp,0xf8`(chkstk 없음) ⟹ ORIG_LEN 15→12·`install_detour_r11` 불요** / ⚠**인자계약 변경 r8=&desc → r8/r9=desc 2워드 쌍**(빌더가 전역 함수포인터 `0x144531340` 간접호출)·rcx=Game/rdx=athlete 유지 = §11.1a | src/lib.rs:1976 |
| `RVA_BUY_ITEM` | `0x211e070` | `0xd0c680` | ✅**실측 확정** | 진입 24B 완전동일·exe **유일 1히트**·본체 동형·인자계약 유지(r8=athlete·[rsp_entry+0x30]=Game·Game+0x30=catalog)·**orig_len=19 유지** | src/lib.rs:2658 |
| `ITEMNET_FORWARD_RVA` | `0x1b9cce0` | `0x10587e0` | ✅**실측 확정** | 진입 24B 동일 + 피처명 5종 일치(self_item/champ_pos_build/lane_counter/synergy/global_counter) + net 레이아웃 불변(+0x8/+0x10=16384/+0x18=1) | src/lib.rs:2706 |
| `RVA_SLOT_HELPER` | `0xc5cd80` | ⛔**0.5.3에 존재하지 않음**(UI 메가함수 `0xa5c1e0`에 **완전 인라인**) | ⛔**포팅 불가 확정** | "blue_pla"/"red_play" movabs 0건·콜사이트 0건. ⟹ 경기중 4번째 슬롯 **아이콘 표시 기능 봉인**(`DIAG_SLOT_UI_OFF=true`) = MIGRATION §7.3 §11.5. **재조사 금지** | src/lib.rs:3975 |
| `LOADER_RVA` | `0x5ac950` | ~~`0x91ab0` (확정)~~ → **`0x2e1550`** | ✅**실측 확정·구값은 오답**(0.5.3, 07-29) | ★자동매칭 `0x91ab0`은 **clone family 형제 혼동 오답**. 정답은 **문자열-xref**로 도출(player_info/wide/strategy/training **16곳 전부 이 함수로 수렴**)·진입 24B 완전동일 | src/ui_inject.rs:20 |
| `PARSER_RVA` | `0x24b5a00` | `0x1a6530` | ✅**실측 확정** | 3인자 계약·노드 stride 0x90 유지(NT_SIZE 무변경) | src/ui_inject.rs:22 |
| `ALLOC_RVA` | `0x25c4d30` | ~~`0xbb2bd0`~~ → **`0x28f7df0`**(3인자·교체·재빌드·배포완) | ✅**일원화 확정**(07-29) | 2인자 `__rust_alloc` 심 소멸 → **impl 직접호출** `(rcx=무시, rdx=flags 0, r8=size)->rax`·**실패 시 0 반환**(0.5.2 `0x25d9640`과 명령 단위 동일). ⛔구안 심 `0xbb2bd0`=OOM 시 abort로 null 체크가 死코드 = 미채택 = §11.4 | src/ui_inject.rs:23 |
| `DEALLOC_RVA` | `0x25c4d90` | ~~미해결~~ → **`0x1000`** | ✅**실측 확정(모드 미사용)** | `__rust_dealloc(ptr,size,align)` 형태 유일 | src/ui_inject.rs:24 |

★**mid-func 사이트도 전부 재핀 완료**(컨테이너 내 패턴 재탐색·오프셋 이식 아님) — launcher retaddr A `0x759c36`→**`0x9a3287`** / B `0x75e5cf`→**`0x9a7b03`** / 조합테스트 `0xd40a63`→**`0x1925f12`**(컨테이너 `0xd405c0`→`0x1925ab0`) · `patch_owned_cap` `0x2341440`→**`0xf24a39`**(imm `0xf24a40`·시그 `48 83 be 58 04 00 00 03`=R15→**RSI 회귀**·신 exe 유일 1건) · `patch_gate3` `0x211e428`→**`0xd0c9be`**(jbe `0xd0c9c4`·시그 `48 83 7c 24 40 02 76`=스필 rsp+0x78→**rsp+0x40**·유일 1건·resolver 컨테이너 `0x211e150`→**`0xd0c770`**) · `SLOT_BOUNDS` 4곳 `0xa63166`/`0xa638df`/`0xa64486`/`0xa64c16`(전부 `48 83 fb 30`·r14/r15→**rbx 통일**) = ⛔**적용 금지**(위 SLOT_HELPER 봉인). 상세 = MIGRATION §7.3 §11.2·§11.5.

★**구조체 변화(타 모드 파급)**: provider(RNG) 구조체 **`≥0xb278` 구간이 전부 `+0x40` 시프트**(★**seed 저장 `0xeab8`→`0xeaf8`** · 0xb278→0xb2b8 · 0xce98→0xced8 · 0xeac0→0xeb00 · 0xeae8→0xeb28) / `0xb274` 이하 불변. ★**athlete 레이아웃 0.5.3 전면 불변(검증완)** = champ String `+0x418`/`+0x420`/`+0x428` · items `+0x448`/`+0x450`/`+0x458` · build `+0x490`/`+0x498`/`+0x4a0` · **id `+0x810`** · team(side) `+0x820` · gold `+0x888` · position(dword) `+0x8b0` · 스택사본 `0x8b8` · **로스터 stride `0x8d0`**(근거: ctor `0x22cb050`→**`0xed32b0`** 3연속 스토어 관용구 동일 + 로스터 순회 `0x1740380` + VIEW `0xee9070`) ⟹ **fix B(athlete+0x810 ∈ MY_ATHLETES) 0.5.3 성립**. ★**Game `+0x1dc0`(provider ptr)/`+0x1dc8`(vtable) 유지**(launcher `0xeb9646`·vtable 슬롯 `+0x20`이 `mov rax,[rcx+0xeaf8]`=새 seed 오프셋과 정합) · `+0x1dd0`/`+0x1dd8`/`+0x2060`도 유지. 전역 코드젠: **alloc/dealloc 직접 call → 간접 썽크** ⟹ alloc 앵커 시그 전멸. buy 호출 = direct → **vtable(+0x78) 썽크 `0xd22340`** 경유(진입부 훅이라 무영향).

### mid-func 사이트 (컨테이너 기준 재도출 필요)

| 상수 | 0.5.2 | 컨테이너 0.5.2 → 0.5.3 | 함수내 오프셋 | 컨테이너 신뢰도 |
|---|---|---|---|---|
| `SETTER_NOP_RVA` | `0xda42ee` | `0xda3fa0` → `0x19987d0` | +846 | 유력 |
| `(inline)` | `0xd40a63` | `0xd405c0` → `—` | +1187 | 미해결 |
| `(inline)` | `0x759c36` | `0x74d510` → `0x997740` | +50982 | 확정 |
| `(inline)` | `0x75e5cf` | `0x74d510` → `0x997740` | +69823 | 확정 |
| `SIM_RVA` | `0x223d1b0` | `0x223d030` → `—` | +384 | 미해결 |
| `VIEW_RVA` | `0x20ae1ac` | `0x20adf20` → `—` | +652 | 미해결 |
| `(inline)` | `0x722ca0` | `0x721b10` → `0xb03ee0` | +4496 | 확정 |
| `(inline)` | `0x740000` | `0x73e170` → `0x983040` | +7824 | 유력 |
| `(inline)` | `0x2060280` | `0x2060200` → `—` | +128 | 미해결 |
| `(inline)` | `0x2341440` | `0x233e9d0` → `0xf21fe0` | +10864 | 유력 |
| `(inline)` | `0x2341447` | `0x233e9d0` → `0xf21fe0` | +10871 | 유력 |
| `(inline)` | `0x211e428` | `0x211e150` → `—` | +728 | 미해결 |
| `(inline)` | `0x211e42e` | `0x211e150` → `—` | +734 | 미해결 |
| `CAND_GATE_RVA` | `0x1a3b280` | `0x1a3b0d0` → `0x112b000` | +432 | 확정 |
| `(inline)` | `0x4e46c0` | `0x4e07f0` → `0xa5c1e0` | +16080 | 확정 |
| `(inline)` | `0x4e4a30` | `0x4e07f0` → `0xa5c1e0` | +16960 | 확정 |
| `(inline)` | `0x4e5110` | `0x4e07f0` → `0xa5c1e0` | +18720 | 확정 |
| `(inline)` | `0x4e5480` | `0x4e07f0` → `0xa5c1e0` | +19600 | 확정 |

## tfm2_level_cap

함수시작(훅 대상) **0/0 해결** · mid-func 사이트 2 · .text밖 0 — ✅**2/2 실측 확정·마이그·배포완(2026-07-31, v2.1.0)** = 정본 **`MODS\MIGRATION.md §7.3 §16`**


### mid-func 사이트 (~~컨테이너 기준 재도출 필요~~ → ✅**실측 완료**)

| 상수 | 0.5.2 | 컨테이너 0.5.2 → 0.5.3 | ~~함수내 오프셋~~ → **0.5.3 실측 RVA** | 컨테이너 신뢰도 |
|---|---|---|---|---|
| `RVA_LEN_LOAD` | `0x22d3fea` | `0x22d3c60` → `0x12c56d0` | ~~+906~~ → **`0x12c5b44`**(실측 오프셋 +1140) | ~~유력~~ → **실측 확정** |
| `RVA_UI_CMP` | `0x80ae73` | `0x803b30` → `0x952170` | ~~+29507~~ → **`0x95a359`**(실측 오프셋 +33257) | ~~유력~~ → **실측 확정** |

> ⚠**이 표의 "함수내 오프셋" 컬럼은 오답이었다**(+906/+29507 ← 실측 +1140/+33257) — **컨테이너 판정만 맞았음**. 다른 모드에서도 이 컬럼을 그대로 더하지 말 것(0.5.3 = 함수 크기 2~10% 증가로 오프셋 비보존). ⚠★**레지스터 할당도 바뀐다**: 레벨업 사이트 GameSetting 베이스 **`r14`→`rax`**(스텁이 rax를 스크래치로 쓰면 크래시) = §7.3 §16.3.
