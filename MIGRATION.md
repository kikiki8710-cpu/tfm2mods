# TFM2 모드 버전 마이그레이션 가이드 (패치/핫픽스 대응)

> 게임이 업데이트(버전업/핫픽스)되면 exe가 바뀌어 **하드코딩 RVA(훅/transmute 주소)가 어긋남** → 모드가 안 뜨거나 크래시.
> 이 문서를 **위에서 아래로 쭉 따라가면** 복구 끝. 도구 = `C:\tfm2mods\migrate_rva.py`.
> (최초 작성: 0.4.13 → 0.4.13-hotfix 2026-06-17 복구 경험 기준. §7에 그때 신RVA표.)

---

## 0. 언제 / 증상
- **언제:** `TeamfightManager2.exe` 크기/수정일이 바뀜 (Steam 자동 업데이트, 핫픽스 포함).
  - 확인: `ls -la "<게임>/TeamfightManager2.exe"` — 크기 다르면 패치됨. (0.4.13=65,740,800 / hotfix(06-17)=65,783,296)
- **증상:**
  - ui_inject 사이드바 버튼(스크림/아이템편집) 안 뜸 → `mods/tfm2_ui_inject/inject_log.txt`에 `prologue mismatch @+0: 0xXX` + `detour_hits=0`.
  - 스크림 모달 드롭다운/itemnet 동작 시 크래시 or 무동작.
  - item_editor 전투 스탯 주입 안 됨 (stathook 로그에 `prologue mismatch`).

## 1. 무엇이 깨지나 / 무엇이 안전한가
- **깨짐 = 하드코딩 RVA** (`const ..._RVA: usize = 0x...; base + RVA` 로 훅/호출). 함수가 이동하면 그 주소가 다른 코드라 프롤로그 검증 실패→훅 미설치(안전 가드), 검증 없으면 크래시.
- **대개 안전(무변경):**
  - **동적 메모리 스캔** (item_editor의 price/stat/아이템배열 탐지 — db영역 시그니처 스캔이라 자가적응).
  - **구조체 오프셋** (entity/item element 등 — 핫픽스는 보통 레이아웃 유지. ⚠ 단 큰 버전업은 오프셋도 바뀔 수 있으니 의심되면 진단 덤프 확인).
  - **UI 조각 `.ui` + 매니페스트** (데이터라 RVA 무관. dll 재빌드 불필요).
  - **SDK rlib / 빌드 toolchain** (base_version 그대로면 재빌드만으로 OK).

## 2. 준비물
- **구 exe 백업** (시그니처 추출용). 위치: `C:\Users\dev\Desktop\claude\tfm2_0.4.13\TeamfightManager2.exe` (구버전마다 폴더 보관).
  - ⚠ 패치 직후, **새 exe로 덮이기 전에 구 exe를 백업**해두면 다음 마이그가 쉬움. (이미 백업폴더 있으면 OK)
- capstone (설치됨, `python -c "import capstone"` 확인. 5.0.7).
- 게임 **종료** (dll 잠김 방지).

## 3. 절차 — RVA 재탐색 (migrate_rva.py)
원리: 구 함수 바디를 디스어셈 → **rip-relative·상대 call/jmp 바이트는 와일드카드**, 나머지 고정 = 마스크 시그니처. 핫픽스는 바디 거의 동일(주소만 이동)이라 새 exe `.text`에서 유일 매치 = 새 RVA.

1. `C:\tfm2mods\migrate_rva.py` 열어 **OLD/NEW 경로** + **TARGETS**(구RVA, 이름) 확인. (현재 8개 등록됨)
2. 실행: `python C:\tfm2mods\migrate_rva.py`
3. 출력 해석:
   - `... -> [0xNEW] OK` = 유일 매치 = 그게 신 RVA.
   - `... -> [...] MULTI` = 다중 매치(제네릭 모노모픽 함수, 예: 에셋게터). → **string-xref로 확정** (아래).
   - `... -> [] NONE` = 못 찾음(함수 삭제/대개편). → 큰 버전업이면 수동 Ghidra 필요.
4. **다중매치 확정 (string-xref):** migrate_rva.py 의 `find_callee_of_string(nd,nib,ns, b"<경로문자열>")` — 그 문자열을 LEA(rip)로 싣고 직후 call 하는 함수 = 정답. (에셋게터는 `b"asset/base/ui/layout/main"`.) 출력의 호출 카운트 최다 = 그것.
5. 각 모드 소스의 `const ..._RVA` 를 신RVA로 교체 (§4 위치표).
6. 빌드·배포 (§4) → 검증 (§5).

## 4. 모드별 체크리스트

### A. tfm2_ui_inject  (프레임워크 — scrim/item_editor 버튼이 여기 의존)
- **상수:** `src/lib.rs` L16-19
  - `LOADER_RVA` = 에셋게터 (타입드 get, 트램폴린 훅 대상). **다중매치** → string-xref `"asset/base/ui/layout/main"`.
  - `PARSER_RVA` = `.ui` 텍스트 파서 (out,txt,len). 유일매치.
  - `ALLOC_RVA` = alloc(size,align). 유일매치.
  - `DEALLOC_RVA` = dealloc(ptr,sz,align). 현재 미사용(leak설계)이라 안 맞아도 무방하나 갱신 권장. (구 0x8943f0=jmp썽크 → real은 migrate_rva가 0x2300710 으로 등록)
- **프롤로그 검증값:** install()이 `55 41 57 41 56 41 55 41 54 56 57 53`(8 PUSH) 기대 — 신 게터도 같은 프롤로그라야 정상(다르면 RVA 틀림).
- **빌드:** `& C:\tfm2mods\build_inj.ps1 -Src C:\tfm2mods\tfm2_ui_inject\src\lib.rs -ModId tfm2_ui_inject`

### B. tfm2_item_editor
- **상수 = 전투스탯 주입 트램폴린 3개** (`src/lib.rs`, 전부 마스크 유일매치):
  - `STAT_FN_RVA`(L77) = FUN_141b860d0 최종계산. **★stat_detour 가 inject_effects→recompute_sums→orig** 하는 메인 훅. 프롤로그 `41 57 41 56 41 55 41 54 56 57 55 53`.
  - `PER_ITEM_RVA` = FUN_141b85c50 키매칭합산. 같은 프롤로그.
  - `SUM_RVA` = FUN_141b86380 단순합산. 프롤로그 `56 48 83 ec 70 66 44 0f 7f 64 24 60`(push rsi+sub+movdqa).
- **recompute_sums 오프셋은 RVA 아님(마이그 무관, 단 구조변경엔 민감):** entry+0x58→entity+0x3b0, +0x60→+0x3b8, +0x68→+0x3c0, +0x70→+0x3c8, +0x78→+0x3d0. **핫픽스는 오프셋 유지라 OK. 대형 버전업(엔티티/이펙트 레이아웃 변경)때만 [[tfm2-combat-stat-pipeline]] 재확인 필요.**
- **나머지 무변경:** price/tier/stat 오프셋·아이템배열은 **동적 스캔**이라 자가적응. 진단 `item_editor_probe.txt`로 확인(바닐라 30 + 모드 N 정상이면 OK).
- **빌드:** `& C:\tfm2mods\build_inj.ps1 -Src C:\tfm2mods\tfm2_item_editor\src\lib.rs -ModId tfm2_item_editor` (~475KB, <1MB라 가드 통과).

### C. tfm2_scrim
- **상수:** `src/lib.rs`
  - L59 `FN_DD_SETOPT_RVA` = 네이티브 DropdownRunner 옵션set. (구 0x21184e0은 프롤로그가 `72 18..`=jb로 시작하는 특이함수지만 마스크매치 됨.)
  - L2227 `ITEMNET_FORWARD_RVA` = 아이템 자동빌드 신경망 forward. 8-push 프롤로그. **itemnet_addr_valid()가 프롤로그 검증** → 틀리면 드롭다운 일부 스킵(크래시 방지).
- **빌드 ⚠ scrim은 ~3.5MB라 build_inj.ps1 의 1MB 가드에 막힘** → rustc 직접 실행 후 수동복사:
  ```powershell
  $SDK="C:\tfm2mods\mod_sdk\0.4.13\mod-sdk"; $DEPS="$SDK\deps"; $NAT="$SDK\native"
  $MODAPI=(gci "$DEPS\libmod_api-*.rlib")[0].FullName; $EUI=(gci "$DEPS\libengine_ui-*.rlib")[0].FullName
  $out="$SDK\lib.dll"; $t0=Get-Date   # ⚠ $env:RUSTFLAGS는 cmd/c 자식에 전달 안 됨 → 플래그는 아래 rustc 명령줄에 직접(-C opt-level=1 필수: 없으면 opt-level=0 디버그빌드=∼5배 느림 / opt2·3은 재현 디투어 프레임 팽창→STATUS_STACK_OVERFLOW 크래시라 ~~opt-level=3~~→opt1 확정, 2026-07-18)
  cmd /c "rustup run nightly-2026-06-16 rustc --crate-type cdylib --edition 2021 -C opt-level=1 -C overflow-checks=off -L dependency=$DEPS -L native=$NAT --extern mod_api=$MODAPI --extern engine_ui=$EUI C:\tfm2mods\tfm2_scrim\src\lib.rs -o $out 2> $env:TEMP\e.txt"
  if((gc $env:TEMP\e.txt|sls 'error\[|error:').Count){gc $env:TEMP\e.txt|sls 'error'|select -First 20}
  elseif((Test-Path $out)-and((gi $out).LastWriteTime -ge $t0)){Copy-Item $out "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_scrim\tfm2_scrim.dll" -Force; "OK scrim"}
  ```
  (⚠ Remove-Item이 샌드박스에 막히면 쓰지 말고 위처럼 mtime≥t0으로 신선도 확인)
- **scrim UI 조각:** RVA 무관(데이터). 단 `gen_scrim_fragments.py`는 main.ui 루트블록 **7개 전부** 추출해야 함(누락시 전술/드롭다운 무반응) — §6 참고.

### D. 기타 모드 (프로브/실험)
- `mods/` 의 다른 dll들(packet_interceptor, plan_reimpl, Spectator_Chat 등)도 자체 RVA 쓰면 stale. **켜서 쓰는 것만** migrate_rva.py TARGETS에 추가해 마이그. 안 쓰면 무시(대개 프롤로그 가드로 미설치=안전).
- 참고 RVA: packet hook(0xee4b80류), banpick(unmoved 경향) — INDEX §2.

## 5. 검증
- **ui_inject:** `mods/tfm2_ui_inject/inject_log.txt` → `hook installed: fn=0x..` + `MAIN loaded` 있고 `prologue mismatch` 없음. 인게임 사이드바에 버튼 뜸.
- **scrim:** 버튼 클릭→모달, 선수/챔프/전술/아이템 드롭다운 다 뜸. `scrim_item_filter.txt`(아이템드롭다운 열면 생성).
- **item_editor:** `item_editor_probe.txt` 레지스트리 정상 개수. 편집창 열려 아이템 목록 + 가격/스탯 표시. (전투주입은 stathook 로그 `INJECT ..` 라인.)

## 6. 함정 / 증상별 진단 (이미 고친 것 = 재발 안 하지만 원리 참고)
- **에셋게터 다중매치:** 0x230 간격 18개 모노모픽 블록(타입별 get). 바디 동일 → 마스크론 구분 불가. **string-xref 필수**(§3.4). 위치(블록내 index)도 보존됨(보조 확증).
- **scrim override→inject 조각 누락:** dll이 노드를 `set_visible_by_id`/find로 **찾기만**(생성X) → 누락 블록은 클릭해도 무반응. **dll의 `"scrim_*"` by_id 참조 전수 grep → 조각 커버리지 확인**. scrim은 루트블록 7개(category/modal/strat_modal/items_modal/dd_p/dd_s/dd_c) 전부 추출해야. (미커버가 원본 main.ui에 0개면 죽은참조=무시: nat_dd/probe_ai/ddp_up/dn/xbtn.)
- **mod_id 의존성 함정:** `mod.mod_info`에 `"dependencies":[{"mod_id":"base"}]` + 최상위 `"mod_id":"riot_items_tfm2"` 처럼 mod_id 2개 → 첫 것(base)만 잡으면 모드 스킵. **extract_mod_id는 dependencies 블록 잘라낸 뒤 최상위 mod_id** 추출(item_editor 적용됨). 워크샵 i18n은 `steamapps/workshop/content/3009300/<번호>/text/item.i18n`.
- **비아이템 오탐(선수/더미):** 아이템 배열 스캔이 athlete Vec(Faker 등)·더미(spectator_chat_probe price=99999)를 긁음 → **tier∈0..4 + price∈[1,2M] 검증(is_item_elem) + 센티넬가격(≥50000) 제외**.
- **enabled_mods 부정확:** mods.json `enabled_mods`는 비활성 모드도 남음 + 코드모드는 presence 로드. ui_inject는 `GetModuleHandle("<mod>.dll")` 로 실제 로드 확인. 활성 아이템 판정은 활성모드 i18n 키 유무로.
- **MPRICE_OFF 탐지실패:** 사다리패턴(500/500/800/1400/2000) 의존이라 riot처럼 비사다리 가격이면 0 반환 → price_off_for가 **VPRICE_OFF(0x180)로 폴백**(mod도 price는 +0x180 동일).
- **filter_handler 공존:** 여러 모드가 클릭핸들러 push → `is_empty()` 판단 금지. **len 추적**(줄면 재등록)이 표준.

## 7. 현재 RVA 표

> ★★★**현행(2026-08-05) = 0.5.4 = 파일 맨 끝 `§7.4`가 정본**(exe 75,936,256B·sha256[:16] `78105410D74836F2`·`.text` +1.51% 온건 패치). **§7.3(0.5.3) 이하는 전부 이력** — ~~단 ⚠`tfm2_ai_adjust`만 0.5.4 대응 제외(유저 지시)라 그 모드의 현행 정본은 계속 §7.3 §12.x~~ → ★**정정(2026-08-06): ai_adjust도 0.5.4 전환 완료(RVA 정본 = `src\rva_054.rs`) ⟹ 예외 없이 §7.4가 현행**. 아래 문단들은 0.5.2/0.5.3 시점 서술 = 이력.

> **★★~~현행 최신~~(이력) = ~~0.5.0_3 핫픽스 (buildid 24125999, exe 69,047,296B)~~ → ~~0.5.1 정식 (buildid 24215274, exe 69,233,664B) — 정정 2026-07-18, 현행 RVA 정본=§7.1~~ → **0.5.2 (buildid 24310934, exe 69,209,088B) — 정정 2026-07-22, 현행 RVA 정본=§7.2(⏳진행중)·직전 베이스=§7.1(0.5.1)**(+버전 사실=`MEM\CURRENT.md`).** ~~모드 소스 상수는 0.5.0_3 신값 반영 완료 → §7.0-3 핫픽스 델타표가 현행 정본~~ → §7.0-3은 직전 베이스(0.5.0_3) 이력표(0.5.1 델타의 "구" 컬럼). 아래 §7.0-2 0.5.0_2 표는 그 직전 베이스(0.5.0_2→0.5.0_3 델타의 "구" 컬럼). 그 아래 §7.0 0.5.0·0.4.14/hotfix 표는 이력 보존용 — STALE.
> **상세 마이그 정본 = `MEM\tfm2-0.5.0-migration.md`** (RVA-only 이동표 + NOMATCH 재RE 목록 + move_guard 패치사이트 + SDK/toolchain).
> **⏳0.5.1 마이그레이션 진행중** = 아래 §7.1 (tfm2_ai_adjust 우선). ★정정(2026-07-15): ~~소스 상수는 아직 0.5.0_3 값·§7.0-3이 소스 정본~~ → **tfm2_ai_adjust·tfm2_item_tactics 소스 상수는 0.5.1 값 반영 완료(§7.1)·컴파일 exit0**. ✅tfm2_ai_adjust=배포·인게임검증완(07-15, dll 4,202,496B, d19_imm applied=15/15·무크래시·itemnet차단0)=DONE / ~~⬜item_tactics·item_editor·scrim 인게임 검증 잔여~~ → item_tactics=07-18 검증완(DONE)·**item_editor·scrim 잔여=폐기(유저 지시 2026-07-22: 마이그 대상 8종 한정·제외 — §7.2·MOD_REGISTRY 참조), §7.1=이력**.

### 7.2 0.5.2 마이그레이션 표 (⏳진행중 2026-07-22 — 대상 8종 한정(유저 지시)·ai_adjust부터)

> **0.5.1 정식 → 0.5.2 (buildid 24215274→24310934, exe 69,233,664→69,209,088B, −24,576B, 2026-07-22).** ~~성격=⬜미판정~~ → **성격 = 버전업급(전역 델타 없는 함수 재정렬) + 대부분 로직 불변 + struct 오프셋 불변** (version-migrator exe↔exe 확정 2026-07-22·아래 각 모드 절에서 반복 확증). **SDK**=GitHub 릴리스 `0.5.2.zip`(416,477,858B)→`C:\tfm2mods\sdk_052\` **다운로드·전개 완료**(build_inj.ps1 `$SDK` 전환완). Ghidra 2인스턴스 가동중(8080=`ghidra`, 8081=`ghidra_beta`).
> **마이그 대상 = 8종 한정(유저 지시 2026-07-22)**: ①`tfm2_ai_adjust`(⏳이번 세션) ②`tfm2_item_tactics` ③`community_reaction_mod` ④`tfm2_banpick_illust`(순수 SDK=재빌드만) ⑤`tfm2_draft_overlay` ⑥`tfm2_elemental_serpen`(✅마이그·빌드·배포완) ⑦`tfm2_fog_damage_fix`(✅완) ⑧`Spectator_Chat`(raw 오프셋 재도출). ~~item_editor·scrim~~=**마이그 제외·0.5.1 동결**(0.5.1 잔여도 폐기). 진행 현황·세션 간 공유=`MEM\tfm2-0.5.2-migration.md`.
> ⚠**§7.1 상단 "asset-get copy 43분화" 경고는 0.5.2에서도 재확인 필수**(UI 주입 모드 = 대상 화면 copy 재확인).

**RVA 표: 모드별로 채우는 중(형식=상수/함수 | 구 0.5.1 | 신 0.5.2 | 판정).**

> ★**패치 성격 판정(version-migrator exe↔exe, 2026-07-22, item_tactics 대상군 기준)**: **버전업급 = 전역 델타 없는 함수 재정렬 + 대부분 로직 불변**.
> item_tactics 대상 21건 중 **UNIQUE(exe2exe 스켈레톤 md5 완전일치=로직·mem disp·imm 전부 불변) 17건 / NO MATCH(로직변경) 4건**(SPAWN·SIM(off)·CAND_GATE(off)·owned_cap 컨테이너).
> 델타는 함수마다 제각각(−0x124db0 ~ +0x3f84c0)이라 **전역 델타 스왑 금지**. ★**struct 오프셋은 불변**(0.5.1=0.5.2): owned_cap `+0x458`·build len `+0x4a0`(buy 프롤로그 바이트 동일)·gate3 `[rsp+0x78]` 전부 그대로 ⇒ CASE-불변 유지, 재현부 내부오프셋 재도출 불요.
> ★**SDK는 ABI 무변경이 아님**: sdk_052 rlib md5 대조 = `mod_api`·`engine`·`engine_asset`·`engine_core`·`engine_ui` **전부 변경**(engine_network만 동일) ⇒ **RVA 0인 순수 SDK 모드도 sdk_052 재빌드 필수**. toolchain은 **nightly-2026-05-24 무변경**(sdk_052 `toolchain_version.txt`=rustc 1.98.0-nightly 23a3312d9 2026-05-23 실측).
> ~~⚠`C:\tfm2mods\build_inj.ps1` L26 `$SDK`는 아직 `sdk_051` — 0.5.2 빌드 전 전환 필요~~ → **전환 완료(2026-07-22, fog 세션)**: `$SDK` = `C:\tfm2mods\sdk_052\mod-sdk`. 전 모드 공용이므로 **이후 build_inj.ps1 빌드는 전부 sdk_052 링크** — 아직 재빌드 안 한 모드는 재빌드 필요.

**★asset-get `.ui` 로더 copy 분화 — 0.5.2 재확인 결과(2026-07-22, string-xref 정적 재도출):**
0.5.1의 copy#1 `0x40f3d0`(main/player_info/wide) / copy#2 `0xeb17d0`(strategy·training·밴픽) 분화가 **0.5.2에선 대상 4경로 전부 단일 copy `0x5ac950`으로 수렴**.
근거(NEW): `player_info` lea@0x50a671→call **0x5ac950** / `wide_player_info` lea@0x50a6b5→call **0x5ac950** / `strategy` lea@0xcdee2b·0xce3a19→call **0x5ac950** / `training` lea 다수→전부 **0x5ac950**. 같은 스크립트가 OLD에서 0x40f3d0·0xeb17d0을 정확히 재현 = 방법 검증됨.
⇒ **UI 주입 모드(item_tactics·draft_overlay·item_editor 등)는 세컨드 훅을 같은 주소에 이중 설치하지 말 것**(자기체인/본문 2회 실행). item_tactics는 `STRAT_LOADER_RVA == LOADER_RVA`면 세컨드 훅 스킵하도록 수정. ⬜**밴픽 화면 copy는 draft_overlay 마이그 시 별도 재확인**(이번 조사 범위=item_tactics 4경로).

**tfm2_item_tactics (0.5.1 → 0.5.2, 2026-07-22 version-migrator exe↔exe 확정·빌드 exit0):**
| 상수/사이트 | 구 0.5.1 | 신 0.5.2 | 판정 |
|---|---|---|---|
| `FN_DD_SETOPT_RVA` | 0x2450f40 | **0x242f250** | UNIQUE·프롤로그 16B 동일 |
| `RVA_REALLOC` | 0x25c5ae0 | **0x25c4dd0** | UNIQUE·프롤로그 동일 |
| `CL_LAUNCHER_RVA` | 0x20588a0 | **0x1d96870** | NO MATCH→**니모닉 0.9860 + 콜사이트 9/9 bijection + 내부 seedctor 2콜 동형**으로 확정. 로직동일, chkstk 프레임만 0x16628→**0x165c8**(프롤로그 상수 갱신) |
| launcher 렌더 retaddr A | 0x72f507 | **0x759c36** | 컨테이너 0x722ca0→0x74d510(니모닉 **0.9928**) 내 동형 콜 |
| launcher 렌더 retaddr B | 0x733e9f | **0x75e5cf** | 〃 |
| launcher comptest retaddr | 0xc884fa | **0xd40a63** | ⬜**잠정**: 컨테이너 0xc831b0→0xd405c0(9/9 bijection 잔여 1쌍·단일콜러 0x75fe90→0x78a5c0 동형·델타 +0x2a730≈렌더빌더 +0x2a870). 컨테이너 크기 0x5b8f→0xce1 축소=리팩터 → ghidra-re 확인 권장 |
| `SEEDCTOR_RVA` | 0x21d03e0 | **0x22c1da0** | UNIQUE·프롤로그 17B 동일(frame 0x11b58 불변) |
| `SPAWN_RVA` | 0x2060280 | **0x1d9e0e0** | ★**NO MATCH=로직변경**(0x714→0x51f, **push 8→7**=`41 55` 소멸). 콜러 0x20565e0→0x1d94640(니모닉 **1.0000**)의 동일 오프셋 +0x8c 콜 타깃으로 재핀. 프롤로그 15B(7push+`mov eax,0x4d20`)·ORIG_LEN 12→**15**·rax 보존 tail 필요(`install_detour_r11` 신설). ⬜**`SPAWN_INJECT_ENABLED=false`로 게이트오프**(인자계약 미검증) — buy 경로 단독으로 도달 8/8(07-19) |
| `RVA_BUY_ITEM` | 0x1f01090 | **0x211e070** | UNIQUE·프롤로그 24B 완전동일(본체 무변경) |
| `ITEMNET_FORWARD_RVA` | 0x1bc82e0 | **0x1b9cce0** | UNIQUE·프롤로그 동일 |
| `RVA_SLOT_HELPER` | 0xd81b30 | **0xc5cd80** | UNIQUE·선두 24B 동일("blue_pla" movabs 포함) |
| `SLOT_BOUNDS` ×4 | 0x4b4d40 / 0x4b50b0 / 0x4b5790 / 0x4b5b00 | **0x4e46c0 / 0x4e4a30 / 0x4e5110 / 0x4e5480** | 컨테이너 UI 메가함수 0x4b0e70→0x4e07f0(UNIQUE, +0x2f980) 동일 오프셋·4곳 전부 신주소 바이트 일치(BYTE-OK) |
| owned_cap sig / imm | 0x2238410 / +7 | **0x2341440 / 0x2341447** | 컨테이너 리팩터(0x2234430→0x233e9d0, 0x4664→0x30d7)로 **레지스터 RSI→R15**: 시그 `48 83 be…`→**`49 83 bf 58 04 00 00 03`**. disp 0x458·imm 3 불변, 신 exe 전체 유일 |
| gate3 sig / jbe | 0x1f01448 / 0x1f0144e | **0x211e428 / 0x211e42e** | 컨테이너 0x1f01170→0x211e150(UNIQUE, +0x21cfe0) 동일 오프셋 +0x2d8·7B 시그 바이트동일 |
| `LOADER_RVA`(uinj) | 0x40f3d0 | **0x5ac950** | string-xref 확정(위 copy 절)·프롤로그 24B 동일 |
| `STRAT_LOADER_RVA`(uinj) | 0xeb17d0 | **0x5ac950**(=LOADER와 병합) | 세컨드 훅 스킵 가드 추가 |
| `PARSER_RVA`(uinj) | 0x24b4590 | **0x24b5a00** | UNIQUE(+0x1470) |
| `ALLOC_RVA` / `DEALLOC_RVA`(uinj) | 0x25c5a40 / 0x25c5aa0 | **0x25c4d30 / 0x25c4d90** | UNIQUE(−0xd10) |
| (OFF) `SIM_RVA` 0x223d1b0 · `CAND_GATE_RVA` 0x1a3b280 | — | **미마이그(STALE 마킹)** | exe2exe NO MATCH·게이트 false라 무영향 |
| (OFF) `SETTER_NOP_RVA` 0xda42ee · beam A/B 0x19f14a5/0x19f1a11 | — | **미마이그(STALE 마킹)** | 게이트 false·beam은 0.5.1부터 이미 시그 불일치(fail-safe 스킵) |

**tfm2_ai_adjust (0.5.1 → 0.5.2, 2026-07-22 version-migrator exe↔exe · ⏳부분완료·빌드 미실행):**

> ★**item_tactics(위)와 성격 판정이 갈리는 이유**: item_tactics 대상은 작은 함수가 많아 스켈레톤 md5(L1) UNIQUE 17/21이었으나, **ai_adjust 대상은 대형 AI 함수(1300~4800 instr)라 L1 대부분 실패** → **니모닉 멀티셋 코사인(L4) 강건매칭**으로 복원. 동일 패치를 봐도 대상 함수 크기에 따라 체감 난이도가 다르다(도구 선택 기준).
> **★0.5.2 신규 함정 = disc19 구조변경**: ~~①severity 판정 블록이 두 함수로 분리(아웃라이닝) — tr49/tr29/hp66/hp41은 신규 함수 0x22f8a90~~ → ⛔**오판정, 정정(2026-07-22 ghidra-re, §7.2-A)**: 아웃라이닝 **아님**. severity 블록이 0.5.2 exe에 **6곳 인라인 복제**(0x22e3cdf/0x22edb5f/0x22effff/0x22f8d6e/**0x2380e16**(=disc19)/0x23a0c21)돼 있고 **0x22f8a90은 disc19가 아니라 남의 핸들러** — 배선했으면 전면 오패치. 변별점 ①tr9 imm이 0x22f8a90 사본은 **9**, disc19 사본은 **0xa** ②disc19 사본만 뒤에 ally 0x32 ×2 + rhB 0x2e가 따르고 ally#2→rhB 간격 0xd가 0.5.1과 일치. 10사이트 전부 disc19 본체(0x2380820) 내부. ②**레지스터 재할당** hp 비교 R15(`49 83 ff`)→**RSI**(`48 83 fe`) ⇒ `patch_imm_bytes`의 **prefix 배열도 함께** 고쳐야 함(주소만 바꾸면 조용히 skip).
> ⚠**판단 상수(imm) 자체는 전부 불변**(tr 0x31/0x1d/0x11/0xa · hp 0x41/0x28/0x19 · ally 0x32 · reach 0x490404400 · lane_margin 0x78 · vis 600) ⇒ **밸런스 수치 변경 아님 = 순수 코드 재배치**. 재현식 상수 재도출 불요.

| 상수 | 0.5.1 | 0.5.2 | 판정 · 근거 |
|---|---|---|---|
| RVA_CONDGATE | 0x1cbb8b0 | **0x21338d0** | ✅**최고신뢰**·반영완. L1-UNIQUE(스켈레톤 md5 완전동일=명령/오프셋/imm 불변)·프롤로그 20B 동일·orig_len 15 경계OK·rip-rel無 |
| RVA_MOVEPRI | 0x1cbc220 | **0x2134240** | ✅확정·반영완. cos 0.9995(2nd 0.9879)·프롤로그 20B 동일·orig_len 13 경계OK. **교차확증=CONDGATE와 상대간격 0x970 완전보존** |
| RVA_DISC19_HANDLER | 0x1e0ddb0 | **0x2380820** | ✅확정·반영완. cos 0.9999(2nd 0.9970)·프롤로그 12B(push8) 동일. **교차확증=severity 사이트 5개가 이 함수 범위서 동일 시그 재발견**. 프레임 0x648→0x638 |
| RVA_RETREAT | 0x1e08cd0 | **0x1b94670** | ✅확정·반영완. cos 0.9999(2nd 0.9893)·프롤로그 20B 완전동일(SUB 0x308까지)·orig_len 12 경계OK |
| RVA_FC59A0 | 0x1e2c980 | **0x1bdb3e0** | ✅확정(중상)·반영완. cos 0.9995(2nd 0.9940)·프롤로그 12B 동일(프레임 0xf8→0xe8은 13B째=트램폴린 무관)·인접 pregate L1-UNIQUE 순서보존 |
| RVA_PREGATE(주석) | 0x1e2c320 | **0x1bdae60** | ✅**최고신뢰**·반영완. L1-UNIQUE=로직 100% 불변(순수재현 my_pregate 유효성 보장) |
| RVA_TABLE_A | 0x384ea20 | **0x3828818** | ✅확정·반영완. 참조함수(pregate) L1-UNIQUE → rip-rel 순서대응 UNANIMOUS + **값 sanity 통과**(앞4=[0,1,3,2] 구값 일치) |
| D19_TV7_RVA | 0x38b7d50 | **0x3863a28** | ✅확정·반영완. 참조 마스크시그 UNANIMOUS(2/2) + **값 16B 완전동일**(u32==7 desc 헤더) |
| SIMUNCHUNK_RVA | 0x19adc93 | **0x19b40c3** | ✅확정·반영완. 컨테이너 L3-UNIQUE + 사이트 12B 바이트 완전동일. 원본바이트(74 a0) 재검증 후 패치=fail-safe |
| vis_window 사이트 | 0x1caedd3 | **0x2126ae3** | ✅반영완(중상). 컨테이너 한정 유일 + 사이트 12B 바이트 완전동일. prefix 3B 검증으로 어긋나면 skip |
| TEXT_END_RVA | 0x2c0ed7f | **0x2c087ff** | ✅확정·반영완. PE .text vsz_end=0x2c08800 실측 |
| **uinj LOADER** | (0.4.14 stale 0x540ad0) | **0x5ac950** | ✅확정·반영완. **문자열 xref 확정**: main(len25) lea@0x6d4fea·strategy(len29) lea@0xcdee2b 둘 다 `call 0x5ac950` = **두 경로 동일 copy 병합**(0.5.1은 0x40f3d0/0xeb17d0 별개). item_tactics 세션과 교차일치. ⚠구값은 0.4.14 기준이라 0.5.x 내내 미동작이었음(이번에 복구) |
| **uinj PARSER** | (stale 0x220e100) | **0x24b5a00** | ✅반영완. item_tactics exe2exe UNIQUE + 본 세션 재검증(0.5.1 0x24b4590↔0.5.2 프롤로그 20B 동일·둘 다 .pdata 함수시작) |
| **uinj ALLOC** | (stale 0x231fb70) | **0x25c4d30** | ✅반영완. 〃(0.5.1 0x25c5a40, −0xd10) |
| RVA_DISC18_HANDLER | 0x1c7ca20 | **0x2376320** | ~~⏸보류(갭 0.0010=변별 불충분)~~ → ✅**확정·반영완**(2026-07-22 ghidra-re, §7.2-A §3). 결정적 근거=`cmp ?,0x5f5e0`(an_cull_dist)가 **exe 전역 단 2곳**이고 그 둘이 각각 이 함수 내부(+0xb66)·disc19 내부 = 0.5.1과 동일 구도. 프롤로그 push8 12B·`SUB 0x5f8`·rip-rel無 → orig_len 12 안전. ★**HARNESS_ON 아래 INSTALL_DIAG_HOOKS 게이트 없이 install_wrap이 무조건 설치되는 경로(tfm2_ai_adjust.rs:6755)** — 구값 방치 시 0.5.2의 그 주소가 우연히 push8이면 **엉뚱한 함수 오후킹** 위험이라 반영이 특히 중요 |
| RVA_GENERIC_BUILD | 0x1e1ebb0 | **0x22b2280** | ~~⏸보류(교차확증 없음)~~ → ✅**확정·반영완**(2026-07-22 ghidra-re, §7.2-A §2). 프롤로그 push8 12B·rip-rel無·`SUB 0x558` → orig_len 12 안전. 내부 상수 3종이 0.5.1과 거의 동일 오프셋. INSTALL_DIAG_HOOKS=false라 미설치(정확도 목적 반영) |
| RVA_ITEMNET_SCORER | 0x1b78420(0.5.0_3) / 0x1bc82e0(0.5.1) | **0x1b9cce0** | ~~⏸보류(fn+12 15B 검증 실패=가드 미설치)~~ → ✅**확정·반영완**(2026-07-22, 0.5.2). 재핀법=install_itemnet_guard 신원검증 **27B 시그**(push8 12B + fn+12 `48 81 ec d8 00 00 00`/`48 8d ac 24 80 00 00 00`) 스캔 216후보 → **변별자 `fn+0x81 == 48 8b 43 10`**(07-11 AV 명령 `mov rax,[rbx+0x10]`)로 **버전당 정확히 1건**. 0.5.0_3 자기일치로 방법 검증·본문 96.4% 바이트 일치(잔차=call rel32). 시그 매칭 산물이라 프롤로그·fn+12 바이트 일치 확정·rip-rel無 ⟹ **설치 성공 보장(런타임 재확인 불요)**, ⬜인게임 미검증 |
| RVA_COMMIT_FN | 0x235ffa0 | ⏸**보류** | 15 instr로 너무 작아 매칭 후보 0 = 재핀 불가. target-guard로 inert |
| RVA_C8C_DMG_SHEET | 0x3830c58 | ⏸**보류** | desc{vt,0x6a8,8,ptr} 구조 스캔 **OLD 11개→NEW 9개 개수 변동**=순서대응 불가·참조 투표 SPLIT |
| RVA_DISC7_DMG_SHEET | 0x3846328 | **0x38d1918** | ~~⏸보류(강후보 0x381e1e0)~~ → ✅**확정·반영완**(2026-07-22 ghidra-re, §7.2-A §5). ⚠자동매칭 강후보 **0x381e1e0은 오답**. 0.5.1 확정 때와 **동일 xref 경로 재현**: disc19 `0x2382a82 lea rcx,[r10+0x478]` … `0x2382a95 lea r9,[rip]→0x38d1918` … call. desc 9개 중 disc19가 참조하는 유일 desc. disc19_repro=dcap 게이트 dev코드라 프로덕션 무영향(정확도 목적) |
| D19_SLOT2_EMPTY / STATIC | 0x3846d50 | **0x38d1af0** | ~~⏸보류(값 변별 불가)~~ → ✅**확정·반영완**(2026-07-22 ghidra-re, §7.2-A §5). 두 상수는 0.5.1서 통합된 단일 empty-descriptor라 **같은 값**. `[r15+0x5b0]<3/<5` fallback 양갈래 + `[+0x30]==-1` 가드 실측 |
| D19_STATIC2_TEMPLATE | 0x38d17b8 | ⏸**보류(유지)** | ghidra-re도 **미확정**(2차 emitter 재식별 실패) → 0.5.1값 0x38d17b8 그대로. ⚠확정 이웃 0x38d1918/0x38d1af0과 같은 0x38d1 대역이지만 **근접 추정 금지**(소스 주석에도 명시) |

**byte-patch 사이트 = 3그룹 전량 보류 (★부분 반영 금지 결정):**
| 그룹 | 사이트 | 자동 재핀 | 결정 |
|---|---|---|---|
| disc19 severity (`apply_disc19_imm`) | 15 | 9 | ⏸**전량 0.5.1 유지** — 부분 적용 시 임계 일부만 사용자값=**판단 일관성 파괴**. prefix 불일치로 `applied=0/15`=무개입(안전) |
| oi_* 넥서스 (`apply_objective_imm`) | 13 | 3 | ⏸전량 유지 = `applied=0/13` 무개입 |
| gb_* 로밍/운영 (`apply_gb_imm`) | 12 | 2 | ⏸전량 유지 = `applied=0/12` 무개입 |

> 참고 — **0.5.2 자동확정 사이트**: ⛔~~d19i tr49=0x22f8d6e tr29=0x22f8d7a hp66=0x22f8d74 hp41=0x22f8d80 (이상 신규함수 0x22f8a90)~~ → **전부 오답·사용 금지**(0x22f8a90=남의 핸들러). **정본 = §7.2-A §1 표**(tr49=0x2380e16 hp66=0x2380e1c tr29=0x2380e22 hp41=0x2380e28 …). / tr17=0x2380e2e tr9=0x2380e3c hp26=0x2380e36 ally2=0x2380ec0 rhB=0x2380ecd (이상 disc19 본체) / **미해결 6**: ally1·rhA·ph30·ph39a/b/c. oi: nexhp1=0x1b934a4 lanemgn=0x1bdac95 pred=0x1bdac25(컨테이너 강후보 dn=0x1b92e40·lane=0x1bdaaa0). gb: reach_cap1=0x23ad9d7 reach_cap2=0x23ba8f3.

> **소스 반영**: `rva_051.rs` → **`rva_052.rs`**(복사 후 갱신, 구파일 이력 보존·참조 없음) + `tfm2_ai_adjust.rs` L25 include 갱신. 상수별 0.5.2 태그 주석. 보류분은 **"고의 보류 + inert 안전근거"** 를 코드 주석에 명시(다음 세션 오해 방지). ~~⚠빌드 미실행~~ → **1차 빌드·배포완**($SDK sdk_052 전환 완료).
> **2차 회차(2026-07-22, ghidra-re 확정 4종 반영)**: 반영처 = `src\rva_052.rs`(DISC18/GENERIC_BUILD/DISC7_DMG_SHEET) + `src\tfm2_ai_adjust.rs` **L2557·L2563**(D19_SLOT2_EMPTY/D19_STATIC_TEMPLATE). 각 줄에 정정형 주석(~~구값~~→신값·근거·안전성) 기입. **재빌드 exit 0 · dll 3,461,632B · md5[:8] `BB1A8CCF` · mtime 07-22 13:39**(1차본 `6C23F33A` 대체). 배포 후 **4/4 링크 바이트 검증 PASS** = 신 주소(0x2376320·0x22b2280·0x38d1918·0x38d1af0) 전부 존재 + 구 주소(0x1c7ca20·0x1e1ebb0·0x3846328·0x3846d50) 전부 부재. ⬜**인게임 미검증**(0.5.2 dll로 게임 미실행 — 훅 실설치·프롤로그 정합 확인 필요).

**tfm2_fog_damage_fix (0.5.1 → 0.5.2, 2026-07-22 — ✅순수 RVA 이동 마이그·빌드/배포완·⬜인게임 검증):**

> 성격 = **5사이트 전부 로직 무변경**(순수 주소 이동). 시야배열 레이아웃 `*(target+0x38+side*0x18)`·게이트 함수내 오프셋(+0xc4/+0xc4/+0xc5)·`jne rel32`(+0x1df/+0x1df/+0x19f) 전부 0.5.1=0.5.2 보존 ⇒ 패치 방식 자체는 그대로.
> ★**코어 native↔data 게이트는 바이트 쌍둥이라 시그 대조로 구분 불가** → **panic-Location(소스경로:행번호) 추출로 판정**(도구 `scratchpad\locs.py` — 다음 마이그의 쌍둥이 함수 판별에도 재사용 권장). 0.5.1 나열순 대비 **앞 두 개가 서로 스왑**됨(패치 내용 동일해 결과는 무관·라벨만 주의).

| 사이트 | 구 0.5.1 | 신 0.5.2 | 판정·근거 |
|---|---|---|---|
| impact_vision_gate_A (착탄 태그6) | 0x1b7e3a7 | **0x22022ca** | 바이트 동일(sete r14b→`41 B6 01 90`) |
| impact_vision_gate_B (착탄 태그7) | 0x1b7e3d1 | **0x22022f4** | ★레지스터만 변경 `sete r8b`(41 0F 94 C0)→**`sete dil`**(40 0F 94 C7), 로직 동일 ⇒ fixed도 `41 B0 01 90`→**`40 B7 01 90`**(mov dil,1+nop) |
| dmgcore_native_gate | 0x1dbb364 | **0x201c274** | 함수 0x1dbb2a0→**0x201c1b0**(`effect\type\attack.rs:141`), jne 6B→NOP×6 |
| dmgcore_data_gate | 0x1dba014 | **0x2019aa4** | 함수 0x1db9f50→**0x20199e0**(`effect\type\attack.rs:459`) |
| dmgcore_v3_gate | 0x1db4865 | **0x2005085** | 함수 0x1db47a0→**0x2004fc0** |
| (참조) 착탄 함수 | 0x1b7b770 (0x4b04) | **0x21ff390** (0x5312) | `simulation\projectile.rs` |

- 검증: 0.5.2 exe 대상 **orig 바이트 5/5 PASS**(`scratchpad\verify052.py`). 빌드 exit0·dll **135,168B**(md5 3B5D2FE39F0A7C25727E3C98F96FBB21)·배포완·deploy-verify FAIL 0건. `mod.mod_info` 0.3.0→**0.3.1**·description "(0.5.1)"→"(0.5.2)"·last_updated 2026-07-22·BOM 없음(첫 3B `7b 0a 20`).
- ★**0.5.2는 이 게이트들을 고치지 않았다** — 5개 전부 로직 그대로 생존 ⇒ "시야 밖 대상 데미지 미적용(석궁병 ult 등)"은 0.5.2에서도 **공식 미수정**, 모드 계속 유효·필요.
- ★**0.5.2 신설 4번째 시야 게이트 = 의도적 미패치**: 신규 함수 **0x2367c20**(0x19d, 0.5.1 대응 없음) 내 게이트 **0x2367c3f**(`cmp qword[rdx+rax*8+0x38],0 ; jne +0x155`)는 **AI 교전 타겟 후보 필터**(데미지 파이프라인 아님) — 콜체인 0x211b520→0x20d6e50→0x20e4600(`plan_legacy\handler\engage.rs`)@0x20e894d. 무력화하면 안 보이는 적까지 교전대상 삼는 전지적 AI가 됨 = 모드 목적 밖 ⇒ **패치하지 않음(소스 주석 명시·"6번째 게이트 발견?" 재조사 금지)**.
- ⬜**잔여 = 인게임 검증**(게임 1회 기동 후 `mods\tfm2_fog_damage_fix\tfm2_fog_damage_fix.txt` 에 `0.5.2 buildid 24310934` 헤더 + 5건 `patched+VERIFIED`) — **이것만 잔여**.
- ~~⬜릴리스 zip 패키징 시 로그 txt 제외~~ → ✅**릴리스 zip 생성완(2026-07-22, 0.5.2)**: `C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\release\0.5.2\tfm2_fog_damage_fix.zip` **69,293B·엔트리 2**(루트 `tfm2_fog_damage_fix\` 한 겹: `mod.mod_info` 574B + dll 135,168B). dll MD5 zip=배포본 `3B5D2FE3…FBB21`·mod_info MD5 `54CF9634…CB072` 일치, zip 내 mod_info 첫3B `7b 0a 20`(BOM無)·UTF-8 파싱 OK(version 0.3.1). **개인·런타임 파일 0건** — 런타임 로그 `tfm2_fog_damage_fix.txt`(11,426B) 정상 제외·`.bak`/`save_`/`dev` 0건. 기존 0.5.2 zip 없어 신규 생성(0.5.1본은 `release\0.5.1\`에 69,288B·엔트리2 동일 구조).
- 재사용 도구(scratchpad): `fog_sig.py`~`fog_sig5.py`(exe↔exe 시그 창 확대 재핀·마스크 정규식 게이트 전수 스캔·PE RVA↔파일오프셋) · `verify052.py`(패치 orig 바이트 대조) · `tfm_scan.py`(dis/xref/pdata-owner) · `locs.py`(함수별 panic-Location).

**tfm2_elemental_serpen (0.5.1 → 0.5.2, 2026-07-22 version-migrator exe↔exe — ~~✅마이그·빌드 exit0·배포완·⬜인게임 검증~~ → ✅✅**인게임 검증완(유저 "잘나온다")·릴리스 배포완 = DONE**, 07-22):**

> 대상 25종 중 **확정 21 / 보류 4**(보류분 전부 cfg 프로브 OFF = inert). 성격 판정은 타 세션과 동일(버전업급 재정렬·델타 제각각 −0x791540 ~ +0x49f3f0).
> ★★**KEYRES = 0.5.2에서 프롤로그 바이트가 바뀐 유일한 훅**: `sub rsp,0x70`/`lea rbp,[rsp+0x70]` → **0x60**. **RVA만 갈고 `KEYRES_PROLOGUE`를 안 고치면 프롤로그 검증 실패 → 훅 조용히 미설치 → 스프라이트 교체 전멸**(디스패처 폴백은 07-19에 코드째 삭제됨). 변경 성격은 패닉/포맷 인자셋업 4명령어 삭제뿐(L2=0.9884)이라 **ABI·시맨틱 불변**, 바닐라 베이스키 43자·3회 등장도 불변 ⇒ 제자리 치환(≤43자) 제약 그대로 유효.
> ★**콜사이트 retaddr은 컨테이너-델타도 콜-서수도 오답**: LAUNCHER_RET_A 컨테이너(0x722ca0→0x74d510)가 84명령어 축소돼 단순델타=0x759d72·콜서수=0x759fc1·**정답=0x759c36**. 정답 도출=**컨테이너 명령어열 difflib 정렬** + 정렬된 call의 타깃이 독립 투표 함수RVA와 자기일치하는지 교차검증(도구 `scratchpad\mig4.py`). **item_tactics 세션이 독립 산출한 값과 3개 전부 일치**(0x1d96870/0x759c36/0x75e5cf).
> ★**스켈레톤 NO-MATCH여도 콜그래프 앵커링이면 확정 가능**: KEYRES/LAUNCHER/RUNNER_CTOR은 L1~L3 전부 NO-MATCH·마스크시그 0건이었으나 구 exe 콜사이트를 전수 수집→콜러를 NEW로 매핑→정렬하니 **7/7·7/7·3/3 만장일치**. 본문이 바뀌어도 호출관계는 남는다.
> ★**struct 오프셋 불변 재확증**: .text **실제 mem-operand disp 센서스**로 provider 계열 11개(0xeab8·0xeac0·0xecc0/c8/d0/d8·0xed18/20/28/50/58) 사용횟수 **11/11 정확 동일**. 엔티티 +0x68(kind)/+0x5a8/+0x658도 SERPEN 본문 참조 15개 완전일치. **스탯블록 물리배치** 근거함수 0x1f097b0→**0x220b470**의 결정 4명령어(`or r9b,[rsi-0x21]`/`movq xmm12,[rsi-0x1d]`/`add r8d,[rsi-0x15]`/`movdqu [rsi-0x11]`) 바이트동일 ⇒ `stat_off()`·`TMPL_STAT_OFF` 유효. ⚠**단 ClientDatabase 계열(0x1338/0x1340/0x1598/0x1630/0x1678/0x1680/0x2970/0x1dc0)은 이 방법으로 판정 불가**(값이 흔해 무관 구조체에 묻힘) → crm·Spectator_Chat 세션 결과와 교차확인 필요.

| 상수 | 구 0.5.1 | 신 0.5.2 | 판정·근거 |
|---|---|---|---|
| `SERPEN_RVA` (kind6 per-tick) | 0x1f8d0c0 | **0x21f8ca0** | L1-UNIQUE(크기 0xf17 동일·mem disp/imm 전부 불변). ★**kind6 확증**: 함수내 +0x73에 `cmp dword[rax+0x68],6`(구/신 동일 위치)·엔티티오프셋 참조 15개 일치 ⇒ 과거 kind5 Epic(0x1c70e90) 오답 재발 없음 |
| `MOBATICK_RVA` (장로 처형) | 0x21fcf90 | **0x230c290** | 마스크시그 UNIQUE + 본문 L1=0.976/L2=0.987(경미 리인라인)·프롤로그 12B 동일·크기 0xbe49→0xbe79 |
| `LAUNCHER_RVA` | 0x20588a0 | **0x1d96870** | 콜그래프 앵커 **7/7 만장일치**(전 콜사이트 EQ 정렬)·프롤로그 12B 동일. item_tactics 세션과 교차일치 |
| `LAUNCHER_RET_A` (화면경기 A) | 0x72f507 | **0x759c36** | 컨테이너 0x722ca0→0x74d510 명령어 정렬(EQ)·정렬된 call 타깃이 LAUNCHER_RVA와 자기일치. **단순델타/콜서수는 오답** |
| `LAUNCHER_RET_B` (화면경기 B) | 0x733e9f | **0x75e5cf** | 〃 |
| `SPAWN_HOOKS[0]` | 0x50edd0 | **0x53aae0** | L1-UNIQUE·프롤로그 동일 |
| `SPAWN_HOOKS[1]` | 0x50e230 | **0x539f40** | L1-UNIQUE·프롤로그 동일 |
| `RUNNER_CTOR_RVA` | 0x205a2f0 | **0x1d981e0** | 콜그래프 앵커 **3/3 만장일치**(EQ 정렬)·프롤로그 12B 동일 |
| `RENDER_STEP_RVA` | 0x872950 | **0x811500** | L1-UNIQUE·프롤로그 동일 |
| `DISP_RVA` (Skia flush) | 0x9f40a0 | **0x9f3090** | L1-UNIQUE(−0x1010)·프롤로그 동일. 현재 `producer_seam=1`이라 미사용 경로 |
| `BUILD_RVA` | 0x414800 | **0x5b1d80** | L1-UNIQUE·프롤로그 동일 |
| `PUSH_RVA` (effect push) | 0x1f15940 | **0x2217600** | L1-UNIQUE(+0x301cc0)·프롤로그 동일 |
| `BUFFAPPLY_RVA` | 0x21df4f0 | **0x1daa7b0** | L1-UNIQUE·프롤로그 동일 |
| `DMGA_RVA` | 0x1f147e0 | **0x22164a0** | L2/L3 UNIQUE·크기 0x32b 동일·L1=0.9945(imm 극소변경)·프롤로그 동일. 델타 +0x301cc0 = PUSH와 동일 |
| `DMGB_RVA` | 0x21e2400 | **0x22d2b20** | L1-UNIQUE·프롤로그 동일 |
| `ENTBUILD_RVA` | 0x13c4e90 | **0xc33950** | 마스크시그 UNIQUE + L2=0.9892·프롤로그 동일 |
| ★`KEYRES_RVA` (에셋키 리졸버) | 0x13c0e90 | **0xc2f990** | 콜그래프 **7/7 만장일치**. ★**`KEYRES_PROLOGUE`도 반드시 동시 갱신** → `55 56 57 48 83 EC 60 48 8D 6C 24 60`(구=…EC **70**…24 **70**) |
| `UILOADER_RVA` | 0x40f3d0 | **0x5ac950** | string-xref 확정(`…ui/layout/ingame` 14회·`…/main` 17회). **item_tactics·ai_adjust 세션과 3중 교차확증**. serpen은 로더 훅 1개뿐 = 이중설치 이슈 없음 |
| `UIPARSER_RVA` | 0x24b4590 | **0x24b5a00** | L1-UNIQUE(+0x1470) |
| `UIALLOC_RVA` | 0x25c5a40 | **0x25c4d30** | L1-UNIQUE(−0xd10) |
| `ARG_STR_RVA` (툴팁 i18n) | 0xb4fda0 | **0xfef190** | L1-UNIQUE·프롤로그 동일 |
| ⬜`ANIM_LOOKUP_RVA` | 0xeb0880 | **보류(구값 유지)** | 제네릭 게터 모노모픽 copy — 0.5.2서 바이트동일 후보 **26개**(stride 0x230) 정적 변별 불가. cfg `anim_probe=0`·07-19에 판정 끝난 1회용 프로브 = inert |
| ⬜`SHEET_LOOKUP_RVA` | 0xeb0420 | **보류(구값 유지)** | 〃 |
| ⬜`RENDER_RVA` | 0x1136600 | **보류(구값 유지)** | ★**0.5.1에서도 이미 죽어 있던 상수**: 실제 바이트가 `ff90 4883c428 5b5d c3`=함수 꼬리, 선언 프롤로그(8push)와 불일치 → 훅 설치된 적 없음(`render_probe=0`이라 무증상) |
| ⬜`SHEET_RVA` | 0x51bbc0 | **보류(구값 유지)** | 〃 실제 바이트 `0f8589.. 488b96..`=함수 시작 아님. 0.5.2 후보도 완전동일 사본 2개(0x820b70/0xc1f1f0)로 변별 불가 |

- 검증: 신 RVA 21종 **프롤로그 12B 전수 확인**(OLD==NEW·모드 선언상수 일치·orig_len 12 명령어 경계 OK·rip-rel 없음). KEYRES만 OLD≠NEW라 선언상수 동시 갱신.
- 빌드: `build_inj.ps1`(sdk_052) **exit 0**·dll **425,984B** sha256[:16] `935F87E17E6A812E`·자동 배포완. 진단/디버그 플래그는 **건드리지 않음**(현행 cfg: `attr_system=1 spawn_hook=1 tooltip=1 tip_panel=1 execute=1 producer_seam=1` / `anim_probe=0 build_probe=0 probe_log=0 render_probe=0 buffapply_probe=0 push_probe=0 entity_lookup=0` — 소스 기본값 `CFG_PUSH_PROBE=true`는 cfg가 0으로 덮음).
- ~~⬜**잔여 = 인게임 검증(유저 몫)**: ①속성별 세르펜 색/스프라이트 교체 ②처치 시 팀버프 ③장로 처형 ④툴팁+버프 패널~~ → ✅**인게임 검증완(유저 "잘나온다", 07-22)** = 위 4축 포괄 확인(세부 항목별 분리 확인은 미수행) ⟹ **DONE 승격·재마이그 금지**.
- **릴리스 배포(07-22)**: zip = `C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\release\0.5.2\tfm2_elemental_serpen.zip` **1,150,441B·엔트리 25**(dll + mod_info + serpen_probe.cfg + config 8 + s 14). 개인/런타임 파일 제외(0.5.1 zip에 섞여 있던 `fanim_editor….lnk` 개인 바로가기 제거·`serpen_probe.txt` 제외). **deploy-verify 6/6 PASS**(zip↔라이브 25엔트리 바이트 해시 diff 0). `mod.mod_info` = version 0.1.0→**0.2.0** / dependency `base >=0.5.0`→**`>=0.5.2, <0.6.0`**(★0.5.2 RVA dll을 0.5.1에서 로드하면 크래시 ⇒ 로더 가드) / last_updated 2026-07-22 / stale "Stage1 프로브" 서술 제거 / BOM無(7b).
- ★**전 모드 공통 규칙(deploy-verify 발견)**: 직전 07-19 serpen 빌드도 **Length가 똑같이 425,984B**인데 SHA256은 달랐음 ⇒ **stale dll 판정에 Length+LastWriteTime만으론 부족 — SHA256 대조 필수**.
- ⬜**DONE 이후에도 남은 것(마이그 잔여 아님)**: ①미확정 4종(위 표 ⬜행) 그대로 유지 ②**죽어 있던 상수 2건(RENDER/SHEET) 소스 정리** ③**ClientDatabase 계열 오프셋(0x1338/0x1340/0x1598/0x1630/0x1678/0x1680/0x2970/~~0x1dc0~~) 미확정 → crm·Spectator_Chat 세션 결과와 교차확인**. ~~0x1dc0~~ → ✅**Game+0x1dc0(provider) = 0.5.2 런타임 검증완(07-24 (seed,fp) 파티션 인게임 검증 — +0xeab8 seed 자기검증 통과·세트마다 재확정, 폴백 +0x1660 코드 유지; ANA discovered-PROGRAM-STRUCTURE §2d⑥ + reimpl-tracker 07-24)**. ⚠07-24 검증 빌드 dll=437,760B(위 425,984B 표기는 07-22 마이그 시점 값 — (seed,fp) 파티션 수정 반영본이 현행).
- 재사용 도구(scratchpad): `mig_serpen.py`(스켈레톤 L1 일괄) · `mig2.py`(L1/L2/L3 3단 퍼지) · `mig_xref.py`(string-xref + 마스크시그 폴백) · `mig3.py`(후보쌍 difflib 유사도) · **`mig4.py`(명령어 정렬 기반 콜사이트/콜타깃 재매핑 = retaddr류 정본)** · `mig6.py`(.text 실 disp 센서스 = struct 시프트 탐지) · `mig7.py`(kind 게이트 확증).

**tfm2_comptest_unlock (0.5.1 → 0.5.2, 2026-07-22 마이그 → ✅2026-07-23 인게임 검증완·최초 릴리스 = DONE):**

> ✅**인게임 검증완(2026-07-23, buildid 24310934, 유저 "잘된다")**: 로그 실측 `[patch] 13/13 patched+VERIFIED`·훅 전량 설치 성공. `[dedup_ins]`·`[spawn_cp]`·`[probe]` 3건만 prologue mismatch = **死상수 판정이 런타임으로 실증됨**(종전 정적 판정과 일치).

> ⚠**대상 8종 한정에서 제외돼 있던 모드이나 유저 명시 요청으로 추가 진행**(2026-07-22).
> 성격 = 버전업급 재정렬(타 세션 판정과 일치)이나 **comptest 대상 함수는 로직 거의 불변**: 컨테이너 6개 중 **4개 L1-UNIQUE** ⇒ **struct 오프셋 불변(CASE-불변) 재확증**·`NT_SIZE 0x90` 유지.
> ★**인코딩 변경 1건 = `server_dedup_real`**: orig `0f 85 **d4** 00 00 00`→`0f 85 **cd** 00 00 00`(jne rel32 변위) ⇒ **주소만 갈면 byte mismatch로 조용히 skip = 선수중복 기능 사망**. 시맨틱 동일 실측(call SwissSet insert 0x1177a10 직후 `test al,al; jne`). fog 착탄B와 동일 계열 함정 = **"로직 동일 ≠ 인코딩 동일"**.
> ⚠**금지 사이트 0x1683e43(라인업 10슬롯 상한)이 btn5v5 빌더A와 같은 컨테이너** — 바이트패턴 검색 방식이면 오매칭 위험 실재. 이번엔 **명령어 인덱스 정렬 방식**이라 구조적으로 회피(패턴검색 금지).

| 바이트패치 사이트 | 구 0.5.1 | 신 0.5.2 | 판정 |
|---|---|---|---|
| no_stamina_cost | 0xf3411d | **0xe93b2d** | orig MATCH·HIGH |
| daily_remaining | 0x1c0b480 | **0x1f14090** | orig MATCH·HIGH |
| daily_inc_gate | 0xf2d110 | **0xe8cb20** | orig MATCH·HIGH |
| ★server_dedup_real | 0xf67b91 | **0xec7758** | orig MATCH·HIGH. ★**orig 바이트 변경**(rel32 d4→cd) — 시그도 함께 갱신 필수 |
| allow_dup_players | 0x1615495 | **0xd00ee5** | orig MATCH·HIGH |
| server_dedup (no-op) | 0xf2bbea | **0xe8b5fa** | orig MATCH·HIGH (07-20 오진 판명분·no-op 유지) |
| btn5v5_roster_min_a | 0x167fecf | **0xd967cf** | orig MATCH·HIGH (⚠금지사이트 0x1683e43과 동일 컨테이너) |
| btn5v5_roster_min_b | 0x160c238 | **0xcf7b68** | orig MATCH·HIGH |
| btn5v5_warn_text | 0x167fd2c | **0xd9662c** | orig MATCH·HIGH |
| roster_count_gate | 0x161edbc | **0xd0a74c** | orig MATCH·HIGH |
| collected_gate | 0x161edb0 | **0xd0a740** | orig MATCH·HIGH |
| collect_err_gate | 0x161ed98 | **0xd0a728** | orig MATCH·HIGH |
| run_push_gate | 0x161f461 | **0xd0adf1** | orig MATCH·HIGH |
| ★**server_roster_min**(14번째·07-23 신규) | (0.5.1 `0xf67ace`) | **0xec768e** | orig `4b 8d 04 3f`(lea rax,[r15+r15]=필요치 2×N) → fixed `4c 89 f8 90`(mov rax,r15 ; nop = 1×N ⇒ 5v5=5·lane=1). **인게임 검증완**·HIGH |

> ★★**"선수단 수가 부족합니다" 진범 = 서버측 로스터 인원 게이트(2026-07-23 ghidra-re, 신뢰도 HIGH)** — btn5v5_* 3건이 전부 patched+VERIFIED인데도 메시지가 계속 뜨던 증상의 규명 결과.
> - **기존 `btn5v5_roster_min_a/b`·`btn5v5_warn_text` 3건은 클라 패널 버튼/툴팁 전용**이었고 정상 동작 중이었다. 실제 거부는 **서버(game_core)**가 한다. ⛔"클라 게이트 완화로 해결(07-20)"류 서술은 이 항목으로 정정.
> - 텍스트키 = `training.comp_test.not_enough_roster`(문자열 @0x1437042c8·len 0x38). ★**LEA xref가 아니라 오프셋 테이블 `0x14370ebb0`의 index 2로 참조돼 문자열 xref에 안 잡힌다** — 07-20 조사 누락의 근본 원인. 테이블 정본(=DISP_RVA 0xd3f780): `0=not_enough_lane_roster 1=no_attempts 2=not_enough_roster 3=duplicate_players 4=champion_required`. 디스패처 호출점은 exe 전체 **단 1곳 `0x74d8bc`**(`r8d`=게임코어 거부코드). 모드 `disp_detour`는 idx 3(dup)만 억제해서 **idx 2가 통과**하고 있었다.
> - 게이트 본체 = 함수 `0xec71b0..0xec786f` = **`server_dedup_real`(0xec7758)과 같은 등록루프 함수**. 거부코드 반환 레지스터 `dil`(0xff=OK / 4=champion / 3=dup / **2=roster부족**). `0xec7641~` 팀 가용선수 카운트 루프(레지스트리 순회·`[a+0x568]==내 팀id` 필터·`[a+0x520]==-1` skip)→rdx / `0xec768e lea rax,[r15+r15]`=필요치 2×팀당인원(5v5=10·lane=2) / `0xec7692 mov dil,2`·`0xec7695 cmp rdx,rax`·`0xec7698 jb →거부`.
> - ★★**핵심 판정: 이 게이트는 선택 배열을 전혀 보지 않고 레지스트리만 순회한다** ⟹ 세는 대상은 **(a) 로스터 보유(가용) 선수 수**이지 distinct 수도 선택 수도 아님 ⟹ **중복 선택으로 통과 가능 = 임계만 낮추면 해결**(접근법 변경 불요). 중복 검사는 완전 별개 지점 `0xec7758`.
> - ⚠**0.5.2 신규 아님**: 0.5.1에 `0xf67ace 48 8d 04 36 lea rax,[rsi+rsi]`로 동일 존재(레지스터만 rsi→r15). **0.5.1에서 안 걸렸던 이유 = 당시 세이브의 팀 가용인원이 10 이상이었기 때문** ⟹ **마이그 실수가 아니라 세이브 인원 조건 문제. 재조사 금지.**
> - 패치 근거: `rax`는 직후 call에서 즉시 덮어써지는 **dead 값**이라 부작용 없음. 완전해제(`31 c0 90 90`=필요치 0)도 가능하나 상위 제출검증(`0xe81787`)에 의존하게 되므로 **보수적으로 1×N 선택**. **다음 버전 마이그 시그 = `4b 8d 04 3f 40 b7 02 48 39 c2`(10B) = .text 전역 UNIQUE**. ⚠금지 사이트(라인업 10슬롯 루프 상한) 오염 위험 없음(별도 함수·imm 0xa가 아닌 `r15*2` 계산값·이 함수 내 `cmp r64,0xa`는 0건).
> - 부수 확정: **`collect`(0xd0bd80) 반환값 = 슬롯 인덱스가 아니라 athlete_id**. 로직 = `id = roster[min(dropdown_sel, roster_len-1)]`·athlete_id 필드 = **athlete+0x6a8**. 로그의 `0x0 0x1 0x2 0x3 0x4 0x4 0x4 0x4 0x4 0x4`는 **roster_len==5에서의 클램프 signature**(슬롯인덱스 오해 금지).
> - ⬜미확정(추정 라벨): 카운트 루프의 `[a+0x0]`/`[a+0x10]`/`[a+0x520]` 필드 의미(은퇴·직군·계약 등). **카운트 로직 자체는 확정.**

| 훅 상수 (16종·전부 PROL-OK) | 신 0.5.2 |
|---|---|
| DISP / RUN / LOADING | 0xd3f780 / 0xd0a440 / 0xd186f0 |
| DD_SETOPT / ITEMCONV / COLLECT | 0x242f250 / 0xed8770 / 0xd0bd80 |
| EF1EA0 / ORACLE / SLOT | 0xe58c30 / 0x1d94720 / 0xd1acf0 |
| RUST_ALLOC / RUST_DEALLOC | 0x8b7f80 / 0x8b7f90 |
| FORGE_CALLERS | 0xd00ed0 |
| CT_REGION (범위) / CT_CLIENT (범위) | 0xe7ccd0..0xea2345 / 0xcf0000..0xda0000 |
| ★ATH_GET_SC | **0xe3b200** — 스켈레톤 NO-MATCH를 **콜사이트 199↔199 일치**로 확정 |
| uinj LOADER / PARSER / ALLOC | **0x5ac950** / 0x24b5a00 / 0x25c4d30 (PARSER·ALLOC은 ai_adjust 세션과 **독립 재도출 일치**) |

**★발견 사실 3건 — 죽은 상수/오선택 실증(다른 모드에도 적용):**
1. ★**`ui_inject.rs` LOADER_RVA가 0.5.1에서 오선택 = 잠복 회귀**. 0.5.1 string-xref 실측 `training`→**0xeb17d0 ×12**(0x40f3d0은 **0건**), `main`→0x40f3d0 ×17. 소스 주석("0x40f3d0=training 계열")은 **사실과 반대** ⇒ 그 훅 제거 시점부터 **comp_test 아이템칸 드롭다운 주입이 0.5.1에서 죽어 있었음**(07-21 인게임 검증은 양쪽 다 훅하던 때 결과). 0.5.2는 단일 0x5ac950 수렴이라 **마이그+회귀수정 동시 완료**. 교훈 = **"오래 안 건드린 uinj 상수는 stale 의심"**.
2. **`ATH_GET_RVA 0x402840`은 0.4.x 잔재** — 선언 PROLOGUE 17B가 0.5.1 `0xeaad40`(=ATH_GET_SC_RVA)와 일치 ⇒ **두 상수가 같은 함수**인데 SC만 마이그돼 옴. HYBRID=폐기 프로토타입이라 **의도적 미부활**.
3. **`DEDUP_INS_RVA`·`SPAWN_CP_RVA`도 죽은 상수였음(종전 미표기)** — 0.5.1에서 이미 함수 중간 주소 → `install_hook_n` Err ⇒ **두 진단 카운터는 0.5.1 내내 0**(로그 오독 금지).
→ 일반 교훈: **마이그 착수 시 "구 RVA의 프롤로그가 모드 선언 상수와 실제로 맞는지" 먼저 확인**(serpen 세션 교훈과 동일 계열, 이번 3건 추가 실증).

**보류 10종 = 전부 死상수·0.5.2에서도 프롤로그 불일치 실측확인 = inert·오후킹 위험 없음**: INSERT 0xcabac0 · ENQ 0xcb9c80 · DEDUP_INS 0xca75f0 · SPAWN_CP 0x13c71b0 · SRV 0x13d4af0 · PUSH 0x101cc08 · ATH_GET 0x402840(+je 0x4028fb) · CT_ARM_LO/HI. 모든 install 경로가 프롤로그 검증(fail-safe), 유일한 무검증 경로 `install_push_probe`는 이미 호출 비활성.

- **빌드/배포**: `build_inj.ps1` **exit 0**·dll **254,464B** md5 **8AF194AC**(07-22 14:25:17)·sdk_052·nightly-2026-05-24·`-C opt-level=1`. `mod.mod_info` **0.2.0/2026-07-22**·description 갱신(바이트패치 2곳→13곳)·**BOM無(0x7b)**·dependencies `>=0.5.0,<0.6.0` **무변경**(0.5.2 포함) — mod_info는 스크립트가 안 옮겨 **수동 배포**. **배포 dll 바이트 RVA 스캔 = 신주소 27/27 존재·구주소 0/27 잔존**(미검출 3건 규명: CT_REGION은 컴파일러가 `(rva−LO)<span` 변환·RUST_ALLOC/DEALLOC은 `COSMETIC_ON=false` DCE). ⚠**`build_inj.ps1`은 구 dll을 백업 없이 덮어씀**(`.bak` 없음 — 타 모드 관례와 다른 지점). stale 로그는 `tfm2_comptest_unlock.txt.bak_pre052_20260722`로 회전.
- ⬜**인게임 검증법**: 게임 1회 기동 후 `<게임설치>\mods\tfm2_comptest_unlock\tfm2_comptest_unlock.txt`에 `[patch]` **13줄 전부 성공**(실패면 `byte mismatch`) + `[hook]/[run]/[item]/[collect]/[oracle]/[ef1ea0]/[slot]` 성공. **`[dedup_ins]`·`[spawn_cp]`·`[probe]`는 실패가 정상**(죽은 상수). 기능 = ①훈련탭 5v5 버튼 로스터 5명서 활성 ②같은 선수 10명 실행 ③**아이템칸 모드템 드롭다운 표시(★이번 회귀 수정분)** ④일일횟수·스태미나 무소모.
- ✅**배포 전 디버그 OFF (07-23·종전 ⬜잔여 해소)**: ★**`log()`에 게이트가 아예 없던 것을 신설** — `const LOG_ENABLED: bool = false` + `log()` 첫 줄 early-return(종전엔 무조건 append라 로그가 **633KB**까지 자랐음). **지원 시 이 상수 하나만 true로 재빌드하면 전량 복구**. `SIM_PROBE_ON: true→false`(oracle·ef1ea0·slot 진단 훅 3개 미설치) — load-bearing 아님 점검완(GAME_CTX/ORACLE_*/SELECTED_*는 프로브 경로 안에서만 읽히고 유일 소비처 A2 스탯주입은 `A2_WRITE_ON=false`). ★**부수이득: 모드의 유일한 게임함수 shadow-CALL(`shadow_ath_get`) 경로가 도달불가**가 됨(oracle_detour 안에서만 호출) = **AV 위험 제거**. 실증 = clean dll에서 `tfm2_comptest_unlock.txt`·`oracle 0x`·`shadow MISS` 문자열 **0건**(검증본은 각 1건), 기능 문자열 `athlete_pool.txt`·`comptest_items.cfg` 및 패치 RVA는 **유지**.
- ✅**최초 릴리스 (07-23 — 이 모드는 0.4.13~0.5.1 어느 릴리스에도 없었음)**: **`<게임설치>\mods\release\0.5.2\tfm2_comptest_unlock.zip`** = **98,046B·3엔트리**(`tfm2_comptest_unlock\` 루트 한 겹) = dll(199,680B) + `mod.mod_info` + `comptest_items.cfg`. 배포 dll **199,680B md5[:8] `9E4680D2`**(로그 DCE로 254,464B→199,680B). 직전 검증본(로그ON) `C73E0DA3`=`.dll.bak_20260723_verified_logon`, 그 전 `8AF194AC`=`.dll.bak_20260723_pre_rostermin`. `mod.mod_info` **0.3.0/2026-07-23**·BOM無(0x7b)·dependencies `>=0.5.0,<0.6.0` 무변경. ★**개인데이터 제외**: `athlete_pool.txt`(유저 로스터 athlete_id 20개)·633KB 로그 백업·`*.bak` 전부 미포함(zip 내 `dev`/`C:\Users`/`Steam`/`steamapps`/`save_2026` **0건**·`GetModuleFileNameW` 존재=경로 하드코딩 없음). 릴리스용 `comptest_items.cfg`는 **유저 실사용본을 건드리지 않고 스테이징에서 템플릿화**(`layout=4→3` 바닐라 기본·샘플 지정 `0 = 106,95,81` 주석처리·"[moditem] 로그 참조"→"LOG_ENABLED=true로 재빌드" 안내 정정).
- ⬜**ghidra-re 인계(선택·전부 현재 inert)**: ①DEDUP_INS/INSERT 재핀(SwissTable insert 모노모픽 다수=정적 변별 곤란) ②SPAWN_CP 재핀 ③CT_ARM_LO/HI 재확정(되면 HYBRID 부활 판단 가능) ④PUSH 콜사이트 재핀(재활성 전 필수·blind INT3 write라 위험).

**community_reaction_mod (crm) (0.5.1 → 0.5.2, 2026-07-22 — ✅마이그·빌드·배포완·⬜인게임 검증):**

> 성격 = **소스 무수정 + sdk_052 재빌드만**. crm은 **하드코딩 RVA 0개**, ClientDatabase raw 오프셋만 사용 ⇒ 아래 오프셋 불변 확정으로 마이그 종료.

| raw 오프셋 | 0.5.1 | 0.5.2 | 판정 |
|---|---|---|---|
| scene tag (u32, InGame==9) | db+0x1338 | **db+0x1338** | ✅불변(HIGH) |
| MatchType (u64, Normal==1) | db+0x1818 | **db+0x1818** | ✅불변(HIGH) |
| match id | db+0x1820 | **db+0x1820** | ✅불변(HIGH) |
| match_info.id(교차검증) | db+0x17F8 | **db+0x17F8** | ✅불변(HIGH) |
| client.events Vec (cap@+0x1670=5744 · ptr@+0x1678 · len@+0x1680) | db+0x1670 | **db+0x1670** | ✅불변(HIGH) |
| (부수) Spectator_Chat `LIVE_EVENTS_OFF=5744` · `LIVE_PLAYED_OFF=5528`(db+0x1598) | 동일 | **동일** | ✅불변(같은 witness) |

- 소스 참조 = `C:\tfm2mods\community_reaction_mod\src\lib.rs` L34, L401-406.
- ★★**검증 방법(재조사 방지)**: mem-operand disp **센서스 방식 금지** — ClientDatabase 계열은 값이 흔해 무관 구조체 사용분에 묻혀 ±7~86% 요동 = **판정 불가**(serpen 세션 결론과 동일). **정답 = 마스크 시그(modrm reg·jne rel32만 와일드카드)로 전역 유일성 스캔 → 소비 함수를 독립 재핀 → 창 바이트 diff 0 확인.**
- 근거 함수(witness) 0.5.1→0.5.2 재핀 대응표:

| witness | 0.5.1 site(owner) | 0.5.2 site(owner) | 읽는 오프셋 |
|---|---|---|---|
| A | 0x725a7f (fn 0x722ca0) | **0x75027f** (fn **0x74d510**) | +0x1338, +0x1902, +0x1818, +0x1820 |
| B | 0x773787 (fn 0x771c00) | **0x79deb7** (fn **0x79c330**) | +0x1598, +0x1768, +0x1770, +0x1828, +0x1670/+0x1678/+0x1680 |
| C | 0x729e28 (fn 0x722ca0) | **0x754628** (fn 0x74d510) | +0x17F8, +0x1808, +0x1810, +0x1816, +0x1818, +0x18E8 |

- 0.5.1 1hit / 0.5.2 1hit **전부 전역 유일**. 재핀 창 diff **0바이트**(disp·jne rel32까지 동일; B를 0xC0B로 넓혀도 diff는 call rel32 2B뿐=콜리 재배치).
- **교차확증**: 오너 매핑 `0x722ca0 → 0x74d510`이 serpen 세션 LAUNCHER 컨테이너 도출값과 **일치**.
- **negative check**: +0x10 시프트 후보(0x1348 / 0x1828-mt / 0x1830 / 0x1808 / 0x1680) 스캔 = 0hit 또는 동수. `cmp dword[r+0x1338],9` idiom 개수 0.5.1=**6** / 0.5.2=**6** 일치 ⇒ 0.5.0_3식 +0x10 시프트 재발 없음.
- **빌드/배포**: `build_inj.ps1` **exit 0** · dll **614,912B** md5 **747C7CF8145B2A98409A65E5239AF76C**(07-22 18:50:29). 배포 2곳 = 로컬 `<게임설치>\mods\community_reaction_mod\`(build_inj 자동 — ⚠**이 폴더엔 `mod.mod_info`가 없어 로더 미인식 = 단순 미러**) + ★**authoritative 워크샵 `steamapps\workshop\content\3009300\3738958482\`**(수동 복사). 구본 md5 070D2258 / 1,035,264B = `.dll.bak_20260722_pre052` 백업.
- **deploy-verify FAIL 0**: md5 3자 일치·mtime 정합 / mod_info BOM無(0x7b)·한글 무손상·파싱OK / 경로 하드코딩 0 / `LOG_ENABLED=false`(로그 문자열이 DCE로 dll에 부재함을 실증).
- ★★**dll 크기 급감 1,035,264 → 614,912B(−40.6%)는 기능 누락이 아니라 `opt-level` 0→1 전환 효과**(구본=07-15 = RUSTFLAGS 미전달 시대). 근거 = 소스 mtime(07-15 18:57)이 구본 빌드시각과 같아 **소스 무변경 실증** + 타 모드 4건이 동일 계단(serpen −41.0%·item_tactics −48.2%·ai_adjust 1분새 −18.7%). ⇒ **07-18 이전 빌드본과 크기 비교로 stale/기능누락을 판정하지 말 것**(전 모드 공통 규칙).
- ⬜**인게임 검증법**: 관전 하이라이트 스캔 + 시리즈 종료 반응 자동생성 동작 확인.
- ⚠**워크샵 폴더 = Steam 구독 콘텐츠**라 재검증 시 게시버전으로 롤백 가능 ⇒ 영구화는 작성자 수동 워크샵 게시 필요(기존 기록 유지).
- ⚠**워크샵 mod_info 메타 stale**: `version 1.1.3`·`last_updated 2026-06-25`·`dependencies base >=0.1.0`(serpen처럼 0.5.2로 조이지 **않음** — 워크샵 구독자 호환 고려한 **의도적 미변경**). 갱신 시 BOM 금지.
- ⚠**별건 TODO**: `gpo_debug.txt` 덤프(`lib.rs:101-109`)가 `LOG_ENABLED` 게이트 **바깥**이고 `GPO_DUMPED` 1회성 가드만 있어 **세션당 1회 무조건 write**. 기존부터 그러함(이번 회귀 아님)·기능 무영향.
- ⚠**워크샵 폴더 누적 잔재**(다음 업로드 전 정리 사안·동작 무영향): `community_reaction_mod.pdb` **97MB**·백업 dll 5개·`.dll.exp`/`.dll.lib`·구 `TFA2_gallery.html`(07-06본, 로컬 07-09본과 상이).

**Spectator_Chat (0.5.1 → 0.5.2, 2026-07-22 — ✅마이그·빌드·배포·릴리스 zip 완·⬜인게임 검증):**

> 성격 = **소스 무수정 + sdk_052 재빌드만**. 하드코딩 RVA **0개**(`Spectator_Chat\src\lib.rs` grep — RVA 언급은 전부 주석). 실사용 = 라이브경기 raw 오프셋 2개뿐.

| 상수 | 소스 | 0.5.1 | 0.5.2 | 판정 |
|---|---|---|---|---|
| `LIVE_PLAYED_OFF` | lib.rs:584 | 5528 | **5528** | ✅불변(TPI 정적 재도출) |
| `LIVE_EVENTS_OFF` | lib.rs:585 | 5744 | **5744** | ✅불변(TPI 정적 재도출) |

- ★★**검증 = PDB TPI 정적 재도출**(crm의 마스크시그 바이트-diff와 **독립적인 두 번째 증거**): sdk_052로 `-C debuginfo=2` **별도 빌드** → `sc_dbg.pdb` → `C:\tfm2mods\tools\pdb\tpi_dump.py`.
  - `ClientDatabase.scene` **+0x1338**(4920) · ClientDatabase size **0xe448**
  - `GameView.played_tick` **+0x258**(600) · GameView size **0x330** (`remain_time` +0x250 · `scores` +0x230)
  - `GameClient.events` **+0x330**(816) · GameClient size **0x428**
  - 산식 검산: `4920+8+600 = 5528` ✓ / `4920+8+816 = 5744` ✓ ⇒ **0.5.0_3 = 0.5.1 = 0.5.2 동일**(0.5.0_2→0.5.0_3의 +0x10 시프트 같은 변화 없음).
- ★**절차 함정**: `build_inj.ps1`은 **pdb를 만들지 않음**(L44에 debuginfo 없음) ⇒ TPI 검증하려면 **`-C debuginfo=2` 별도 빌드를 따로 돌려야 함**. 이 수단이 raw 오프셋 마이그 검증 **1순위**임은 0.5.2에서 재실증됨.
- ★**UI Runner 오프셋 0.5.2 불변**(같은 TPI 덤프·**재조사 금지**): `DraggablePopupRunner` size **0x1d8** — `header_height` **+0x1c0**·`min_w` **+0x1c4**·`min_h` **+0x1c8**·`resize_handle` **+0x1cc**(매 프레임 4 write 대상) / 부수 z +0x190·cursor_x +0x198·drag_offset_x +0x1a0·resize_start_* +0x1a8~0x1bc·ignore_event +0x1d0·dragging +0x1d2·anchored +0x1d7. `LabelRunner` size **0x1f0** — `text` **+0x160(=352)** ⇒ `ui_kit.rs off::LABEL_TEXT=352` 유효. **0.4.14↔0.5.0↔0.5.1↔0.5.2 전부 무변경.**
- ★**base `ingame.ui` 재머지 = 이번엔 불필요**: 0.5.2 번들 전량 재추출(`C:\tfm2mods\tools\bundle\extract_bundle_ui.py` → `bundle_ui_052\`, .ui 286개/.style 2개) 결과 base `ui/layout/ingame.ui` = 0.5.1↔0.5.2 **완전 동일**(3073줄/70,503B·diff 0줄) ⇒ override 재머지 불요·신규 UI 소멸 위험 없음. ⚠**0.5.2 한정 판정** — "패치마다 재추출·diff" 원칙 자체는 유효.
- **빌드/배포**: `build_inj.ps1 -Src ...\Spectator_Chat\src\lib.rs -ModId Spectator_Chat` **exit 0** · dll **328,704B** sha256[:16] **cfc50bbc769038a2** · PE TimeDateStamp **0x6a609a27 = 07-22 19:23:35**(mtime과 초단위 일치 ⇒ stale 원리적 배제) · `<게임설치>\mods\Spectator_Chat\Spectator_Chat.dll`.
- ★**613,376 → 328,704B(−46.4%)는 기능 누락 아님 = opt-level 0→1 계단**(crm −40.6%·serpen −41.0%·item_tactics −48.2%). 직접 증거 = `sc_trace.txt` 문자열이 신 dll에서 **DCE 소거**(`SC_TRACE=false` 상수전파 실증)·export 2종(`tfm2_mod_api_version`/`tfm2_mod_entry`) 구본 동일.
- **deploy-verify 5/5 PASS**: stale 아님(TimeDateStamp) / `mod.mod_info`·`mod.override_info` **BOM無(0x7b)**·JSON OK·한글 무손상·`dependencies base >=0.4.12` 0.5.2 통과(수정 불요) / override `ui\layout\ingame.ui` **77,479B** 배포본↔소스 정본 `ui_layout_ingame_FIXED.ui` sha256 일치 / `SC_TRACE=false`(lib.rs:70).
- **릴리스 zip**: `<게임설치>\mods\release\0.5.2\Spectator_Chat.zip` **161,239B / 6엔트리**(`Spectator_Chat\` 폴더 한 겹) = dll + mod.mod_info + mod.override_info + chat_lines.txt + chat_lines_프롬프트가이드.txt + ui\layout\ingame.ui. mod_info **무변경**(version bump 없음 = 0.5.1 때와 동일 방침). ⚠★**`chat_lines.txt`는 load-bearing**(lib.rs:160 런타임 로드) ⇒ "런타임 txt"라고 제외하면 사고. 제외 = SC_LOADED.txt·io_test.txt·len_off_detect.txt·chat_lines_생성기.html.
- ⬜**인게임 검증법**: 리플레이 채팅 표시 / 즉시보기(라이브 = 5528·5744 실증축) / 패널 드래그·리사이즈 / 데스매치·연장전 HUD 정상.
- ⬜**잔여**: ①소스 폴더 stale 산출물 `C:\tfm2mods\Spectator_Chat\lib.dll`(583,680B·06-30) 삭제 권장(cwd 오복사 씨앗) ②`lib.rs` L74 게임 설치경로 하드코딩(SC_TRACE 게이트 뒤=프로덕션 무영향, CLAUDE.md §2-③ 위반) → SC_TRACE 재활성 시 동적 도출 교체.


### 7.1 0.5.1 마이그레이션 표 (이력 — 0.5.2 도착 07-22로 종결·0.5.2 델타의 "구" 컬럼 — ✅tfm2_ai_adjust 배포·인게임검증완(07-15, d19_imm 15/15·무크래시)·~~⬜item_editor/scrim 잔여~~→폐기(07-22))

> **버전업급 (0.5.0_3 → 0.5.1, buildid 24125999→24215274, exe 69,047,296→69,233,664B, +186,368B).** 핫픽스 아님 — **전역 델타 없이 함수 재정렬** + struct/스택프레임 오프셋 광역 시프트로 mask-sig·스켈레톤해시 대부분 NO MATCH → **강건매칭(니모닉 멀티셋 0.97~0.98)** 으로 함수시작 복원(version-migrator 확정 2026-07-15, exe↔exe). **SDK/toolchain 무변경**(sdk_051 rlib 4종 해시=0.5.0hf2 동일=ABI 무변경, toolchain nightly-2026-05-24).
> **★정정(2026-07-15, ghidra-re 0.5.1 확정): ~~반쯤 마이그 상태 빌드/배포 금지~~ → 함수시작 RVA 전량 소스 반영 완료·rustc 컴파일 성공(exit0, dll 4,202,496B).** ✅**배포·인게임 검증완(07-15 18:18~18:20 유저 실행, DONE)**: d19_imm.txt applied=15/15(disc19 severity byte-patch 15사이트 0.5.1 주소 적용성공)·KEEP훅 install 무크래시 완주·crash_log 신규스택0·itemnet_guard 차단0. ~~⏳배포 블록=게임 실행중~~ 해소. **★재조사 방지 핵심=관문판정 CASE-불변**: 엔티티/SimState struct 오프셋 전부 0.5.0_3=0.5.1 **동일**(type+0x68·dtype+0x4a4·champ+0x5a8·pos+0x648/0x650·hp+0x658/0x610·speed+0x628·SimState +0x818/0x820/0x218/0x400/0x100 등)·vtable 슬롯 불변(vt+0x28 base게터·vt+0x30/0x38 combat게터·vt+0xd8/0x1a0/0x1b8). ⇒ 재현부는 capture(shim ABI-args+entry_rsp 스택args)+절대 struct오프셋으로 계산=원본 프레임 스택로컬 **미참조** → **함수 프레임 성장(disc19 0x638→0x648 +0x10 등)은 재현에 무영향=내부오프셋 재도출 불요, 주소스왑만으로 재현 유효.** 상세 진행상태=`MEM\CURRENT.md 0.5.1 마이그 기준`.

> ★★**[모든 `.ui` 주입 모드 필독] 0.5.1 asset-get "모노모픽 copy 분화"** (2026-07-18 ghidra-re 확정): `.ui` 에셋 게터 `get(asset_manager, path, len)`가 **바이트동일 copy 43개로 분화**(0x230 간격 연속블록 0x40e020~0x410550·0x9c2ff0·0xa2b480~0xa2bb10·0xeb0420~0xeb2950·0x248c020/0x248c250)했고 **어느 화면이 어느 copy를 타는지 화면마다 다르다**. **copy#1 0x40f3d0**=main/player_info/wide/title / **copy#2 0xeb17d0**=**밴픽 + strategy**. ⇒ **RVA 상수만 스왑하면 detour가 조용히 미발화(무증상 실패)**. **T1 잔여(item_editor·scrim) 포함 모든 UI 주입 모드는 "내 대상 화면이 부르는 copy"를 반드시 확인**(대상 경로 문자열 xref → `call` 타깃). 근거: 밴픽 = 문자열 `"asset/base/ui/layout/banpick/layout"`(RVA **0x374cb6d**·len 0x23·exe 유일) → 사이트 0x1411b0174 `lea rdx,[rip+0x259c9f2]; mov r8d,0x23; call 0x140eb17d0` (밴픽 빌더 **0x11b0120**, 그 안에 호출 1회). 두 copy 선두 64B 바이트동일(차이=rip-rel disp32)·프롤로그 push8 → 12B 트램폴린 안전. 사례 = 아래 draft_overlay 소절 + §7.1 C(item_tactics strategy).

**tfm2_ai_adjust 함수시작 (0.5.0_3 → 0.5.1, 2026-07-15 확정):**
| 상수/함수 | 0.5.0_3 | 0.5.1 | 판정 |
|---|---|---|---|
| DISC19_HANDLER | 0x1c83700 | **0x1e0ddb0** | 이동(니모닉 98%·프레임 0x638→0x648 +0x10)·소스반영완·프롤로그 push8 12B 불변 (~~내부오프셋 재도출 필요~~ → **불요: CASE-불변, 주소스왑만으로 유효**, 2026-07-15) |
| DISC18_HANDLER | 0x1c81980 | **0x1c7ca20** | 이동(프레임 0x5f8→0x608)·소스반영완·프롤로그 push8 12B 불변 (~~내부 재도출 필요~~ → **불요: CASE-불변**, 2026-07-15) |
| RVA_COMBAT_FN | 0x22e85a0 | **0x1f19c00** | RVA-only 본체동일(detour off=상수만)·소스반영완 |
| RVA_COMMIT_FN | 0x19e7d30 | **0x235ffa0** | RVA-only 본체동일(guard inert)·소스반영완 |
| RVA_CONDGATE | 0x19e40e0 | **0x1cbb8b0** | 이동 본체동일·소스반영완·orig_len 15B(3번째명령 MOV RSI→R11 바이트변경이나 relocate 무영향) (~~내부 재도출 필요~~ → **불요: CASE-불변**, 2026-07-15) |
| RVA_MOVEPRI | 0x19e4a50 | **0x1cbc220** | 이동 본체동일·소스반영완·orig_len 13B 불변 (~~내부오프셋 시프트~~ → **재도출 불요: CASE-불변**, 2026-07-15) |
| RVA_TTD | 0x20a5030 | **0x21eb300** | 이동 본체동일(detour off=상수만) |
| RVA_RETREAT | 0x1f37f70 | **0x1e08cd0** | ✅확정(ghidra-re 0.5.1, 콜체인검증)·소스반영완 (~~중신뢰~~ 확정, 2026-07-15) |
| RVA_FC59A0 | 0x1f553a0 | **0x1e2c980** | ✅확정(시맨틱 완전일치)·소스반영완 (~~로직변경 축소 516→438B~~ → 오정보, 실제 1750B·시맨틱 동일, 2026-07-15) |
| RVA_GENERIC_BUILD | 0x22db820 | **0x1e1ebb0** | ✅확정(F80320×7 최대함수 entry)·소스반영완 (~~잠정 0x2388d00 대폭변경~~ → 정정 0x1e1ebb0, 2026-07-15) |
| RVA_F80320 | 0x2375b90 | **0x2389950** | ✅확정(GB 콜리)·소스반영완 (~~잠정 로직변경 1064→1257B~~ → 오정보, 2026-07-15) |
| RVA_PREGATE | 0x1f54d10 | **0x1e2c320** | ✅확정(순수재현)·소스반영완 (~~잠정 0x18fb7a0 "1257B확장"~~ → 오정보, 정정 0x1e2c320, 2026-07-15) |
| RVA_GB_EPILOGUE | 0x22dbd22 | **0x1e2248f** | ✅확정·소스반영완 (ORIG_LEN 15→14·out-local rbp+0x2b8→0x500, 2026-07-15) |

**disc19 severity byte-patch 사이트 (0.5.1, ghidra 0.5.1 디스어셈 확정 — `apply_disc19_imm`에 반영 완료):** 함수 0x1e0ddb0. threat CALL **0x141e0e311→0x14237e6c0**. 레지스터 마이그: hp_pct RSI→**R15**(`49 83 ff`), phase R10→**R9**(`49 83 f9`), RAX/RCX 불변. 전부 cmp reg,imm8·패치=CMP+3.
| 그룹 | reg(프리픽스) | 사이트(abs, 0.5.1) |
|---|---|---|
| 위협비율 tr>49/29/17/9 | RAX(`48 83 f8`) | 0x1e0e503 / 0x1e0e50f / 0x1e0e51b / 0x1e0e529 |
| HP경계 hp<66/41/26 | R15(`49 83 ff`) | 0x1e0e509 / 0x1e0e515 / 0x1e0e523 |
| ally(0x32) | RAX(`48 83 f8`) | 0x1e0e589 / 0x1e0e5d5 |
| retreat hp>45 / hp>=46 | R15(`49 83 ff`) | 0x1e0e4b4 / 0x1e0e5e2 |
| phase>=30 | RCX(`48 83 f9`) | 0x1e0e2d7 |
| phase>=39 | R9(`49 83 f9`) | 0x1e0e498 / 0x1e0e532 / 0x1e0e5c2 |

**oi_* 넥서스 오브젝트 판단 byte-patch 11사이트 (0.5.1, 2026-07-16 — `apply_objective_imm`[detour.rs]의 `patch_imm_bytes` 사이트에 반영 완료):** ★disc19 severity(위 15사이트 0x1e0e5xx)와 **별개 함수**=넥서스 진입/수비 결정(구 0.5.0_x FUN_142101a80 계열). **시그·imm_len 불변·RVA만 교체**(버전업급=함수 재배치·사이트 상대위치 보존). 11/11 확정·an_count_gate(0x231cd04)=**시맨틱 규명(07-16·구 "적 넥서스 진격 인원" 라벨 오류→정정)**: [rbx+0x5b0]=아군 진격인원 아님=**엔티티 reach 선형계수 겸 능력-def 선택 count**(rbx=구조물 엔티티 자신 type0xd)·≥5→empowered def(struct+0x520)·미만→기본더미·disc18 핸들러 0x1c7ccea(cmp5/+0x520)·0x1c7ccc8(인히비터 cmp3/+0x4e8)와 동일 "0x5b0 임계→능력프로파일 선택" 메커니즘 공유. **★효과방향 정정=임계↓→넥서스 더 잘 지켜짐(진격 어려워짐)**(구 "↓=소수도 진격" 반대). 클러스터: dn_count/nexus_hp/hp_crit/hp_low/near_dist=단일함수 0x21a45dc~0x21a4b54(수비결정)·lane_margin/pred_dist=0x21ee085~0x21ee0f5.
| 의미 | 0.5.0_2 | 0.5.1 | imm | 상태 |
|---|---|---|---|---|
| dn_count_gate | 0x2102135 | **0x21a4aa5** | 0x26 | ✅확정 |
| dn_nexus_hp #1 | 0x2102125 | **0x21a4a95** | 0x32 | ✅확정 |
| dn_nexus_hp #2 | 0x2102167 | **0x21a4ad7** | 0x32 | ✅확정 |
| dn_hp_crit #1 | 0x210214b | **0x21a4abb** | 0x15 | ✅확정 |
| dn_hp_crit #2 | 0x21021e4 | **0x21a4b54** | 0x15 | ✅확정 |
| dn_hp_low | 0x21021b6 | **0x21a4b26** | 0x1f | ✅확정 |
| dn_lane_margin | 0x2115c72 | **0x21ee0f5** | 0x78 | ✅확정 |
| dn_near_dist #1 | 0x2101c6c | **0x21a45dc** | movabs 0x35a4e9001 | ✅확정 |
| dn_near_dist #2 | 0x2101cb9 | **0x21a4629** | movabs 0x35a4e9001 | ✅확정 |
| dn_pred_dist | 0x2115c05 | **0x21ee085** | movabs 0xd693a4001 | ✅확정 |
| an_count_gate | 0x232351e | **0x231cd04** | 5 | ✅규명(07-16)=[rbx+0x5b0] 엔티티 reach계수 겸 능력-def선택 count(진격인원 아님·임계↓→넥서스 더 잘지켜짐) |

**데이터 RVA (재현 leaf용, 0.5.1):**
| 상수 | 0.5.0_3 | 0.5.1 | 상태 |
|---|---|---|---|
| RVA_TABLE_A | 0x38a75b0 | **0x384ea20** | ✅확정(HIGH)·소스반영완 |
| D19_SLOT2_EMPTY | 0x385e5e0 | **0x3846d50** | ✅확정(HIGH)·소스반영완 |
| D19_STATIC_TEMPLATE | 0x380d3f0 | ~~NEEDS-DEEPER~~ **0x3846d50** | ✅확정(dcap하네스, 2026-07-15)·★SLOT2_EMPTY와 동일주소 통합·소스반영완 |
| D19_STATIC2 | 0x38aecc0 | ~~NEEDS-DEEPER~~ **0x38d17b8** | ✅확정(별도desc·FUN_14238b290 LEA@0x14238b738+F80320 이중확증, 2026-07-15)·소스반영완 |
| D19_TV7 | 0x38796f8 | ~~NEEDS-DEEPER~~ **0x38b7d50** | ✅확정(LEA@0x142281e09/eba 간격0xb4, 2026-07-15)·소스반영완 |
| RVA_C8C_DMG_SHEET | 0x380d138 | ~~NEEDS-DEEPER~~ **0x3830c58** | ✅확정(desc{vt=0x141c69300,0x6a8}, 2026-07-15)·소스반영완 |
| RVA_DISC7_DMG_SHEET | 0x38503b0 | ~~NEEDS-DEEPER~~ **0x3846328** | ✅확정(삼중확증·disc19 FUN_14230ee30 실사용, 2026-07-15)·소스반영완 |

> ✅**정정(2026-07-15, 0.5.1): dcap-leaf ~~NEEDS-DEEPER 5개~~ → 7/7 확정 반영**(dcap 검증하네스로 재도출). 위 5개 RVA 확정=소스반영완. 참고: 공용 r9 base쌍 desc 0x38ae878→**0x38d12d8**(소스 미사용). ~~dcap 켜기 전 재도출 필수~~ 해소 — dcap=1 disc19cmp 인게임 비트동일 대조는 대기중(cfg dcap/d19abil/d19abil2/d19gate1=1 설정됨, 유저 실행).

**GB 서브시스템 = 프로덕션 안전-inert (마이그 불요):** INSTALL_DIAG_HOOKS=false(F80320/GENERIC_BUILD body/GB_203/690/EPILOGUE 미설치)·MIG_GB_CHANGED=true(region D skip)·fail-safe 가드훅(move_post/commit/threatgate=target-guard로 stale이어도 inert 무크래시). RVA_F2_BUILD_CALL/GB_REGIOND/FUNNEL/203CB30/20C0690=rva_051.rs에 **0.5.0_3값 유지=inert**(스왑 안 함).

**★GB 메인빌드 전체재현 = 폐기·재시도금지 → 로밍/운영=byte-patch(apply_gb_imm 예정) (2026-07-16 대규모 RE, 0.5.1):** generic_build 본체=**0x1e1ebb0~0x1e24b6d**(24509B·RET 0x1e224a9·에필로그 0x1e22496·out@rbp+0x500). 메인빌드(영역 A/B/C) 전체재현 **3중 차단으로 폐기**(구조변경·stale스택 rbp+0x108/+0x88 미초기화·region-D 부활훅 차단). **훅 재핀 0.5.1 확정값(rva_051.rs는 inert 유지=스왑 금지)**: RVA_F2_BUILD_CALL=**0x1e27234**(GB 진입 유일 E8·콜러 FUN_141e25490·move-post facet#2 재활성 선결=이 값으로 스왑)·RVA_GB_FUNNEL=**0x1e22437**(region-D 공통출구·gbskip 타겟)·RVA_GB_REGIOND_HOOK=**차단**(게이트 0x1e22306이나 억지 재핀=크래시 위험·갱신 보류). callee RVA표·struct 오프셋 이동·byte-patch 후보상수 전량 = **정본 `ANA\discovered-PROGRAM-STRUCTURE.md §3k`** + DONE.md.

**⏳0.5.1 미완 잔여**: ~~①함수시작 RVA 일괄반영 ②disc19 내부오프셋 재도출 ④로직변경 5함수 ghidra-re ⑤재빌드~~ → **①②④⑤ 완료(2026-07-15): 함수시작 RVA 전량 소스반영·5함수 ghidra-re 확정·컴파일 exit0.** ~~남은것: ⏳배포·인게임 A/B 검증 · ⬜dcap-leaf 5개(NEEDS-DEEPER) 재도출~~ → **배포·인게임 검증완(DONE) + dcap-leaf 7/7 확정(위 데이터RVA표)**. 남은것: **⏳dcap=1 인게임 disc19cmp 비트동일 대조**(cfg dcap/d19abil/d19abil2/d19gate1=1, 유저 실행 대기·프로덕션 무관). → 진행상태 정본=`MEM\CURRENT.md`.

**★dcap=1 검증하네스 ~~복원~~ → 은퇴 (설계결정, 2026-07-16, 0.5.1):** ⚠**은퇴**: disc19 출력축(abil emit tag 0xf/0x10/0x11)은 **게임원본 사용**(우리 재현·대체 안 함=order-call-cosmetic 동류)·판단축(severity 임계값)만 byte-patch(apply_disc19_imm 15/15)로 조작=프로덕션 → **dcap=1 full-output abil bit-exact 재현/검증은 설계상 불필요**. dcap=1 그라인드 ~89% count parity(under=0)까지 갔으나 100% 미달·구조규명 완료(지름길 없음 확정)=재조사 금지. ★스킬-desc vtable 슬롯(pred 0x78/aim 0xc8/range 0x90/child 0x48·50·58)=0.5.1 **불변**(ghidra 실측·Command struct +8과 무관·별도 클래스)=이 방향 재조사 금지. "미등재=8730"=허수지표(하드코딩 arm 미포함 카운트일 뿐·decode_bool_leaf_d가 실제 처리). 아래 A~D의 확보 RE(base-dmg 게터 6종·descvt 23·compFlag 0x2090ec0→0x236b6b0·Command stride 0x4f8→0x500·decode_descvt/decode_bool_leaf_d·leaf 데이터RVA 7종)=**유지**(재사용 가능). abil 재현부(disc19_repro.rs descvt/게터/range arm)=dcap-gated 미발화 dev코드로 잔존(프로덕션 무영향). 이하 과거 진행기록(참고): dcap=1 disc19cmp 하네스를 0.5.1로 추가 복원. **크래시0 완주·disc19cmp 생성**·abil 측정축 정화(559 오판 제거). 남은 실차이=**+127 over-emit**(+71 descvt 진행중·+56 threat deferred).

*A. Command 구조체 0.5.1 +8 성장(disc18/19 신규):* stride **0x4f8→0x500**·verb 앞 +0x4a0에 8B(destY) 삽입 → verb/aux/tag/trailer 전부 +8·tag **+0x4f1→+0x4f9**. 앞필드(issuer+0/target+8/flag+0x10) 불변. shape-A(어빌 tag 0xf/0x10/0x11)=앞필드가 데이터·shape-B(이동/후퇴 tag 2/3/5)=앞필드는 공유컨텍스트 ptr·실좌표는 뒤 +0x490(issuer)/+0x498(destX)/+0x4a0(destY). 소스: D19_STRIDE 0x500·d19_cmd_rd shape-aware(abil=target+flag만).

*B. dcap=1 크래시 2모드 근본수정:* ① probe_basedmg_r9 garbage gptr로 game CALL→AV = **code_ptr_ok(.text+VirtualQuery) 게이트**. ② d19_basedmg 하드코딩 리터럴 exe+0x38503b0(옛 DISC7 sheet)→**RVA_DISC7_DMG_SHEET**(=0x3846328). ③ compFlag shadow Fn2090 **0x2090ec0→0x236b6b0**(D19_G1CF_SHADOW 기본 OFF=순수).

*C. shadow-call base-dmg 게터 6종 RVA 0.5.1 재배치(프롤로그 바이트동일=로직불변, d19_basedmg_dispatch match 키):*
| 게터 | 0.5.0_3(옛캡처) | 0.5.1 |
|---|---|---|
| champ16Bwalker | 0x19ebd90 | **0x1f23a60**(+secondary 0x1d204c0) |
| minion24B | 0x1e67de0 | **0x1a5ee60** |
| dual | 0x1e67090 | **0x1d1f630** |
| terminal | 0x2273350 | **0x1dce1d0** |
| stackTerm | 0x1e114e0 | **0x1d328e0** |
| condPicker | 0x1e65740 | **0x23a4d90** |
| name-dict | 0x1fc0d70 | **0x1f090a0**(순수=마이그 불요) |

*D. dcap=1 잔여(진행중/deferred, DONE 아님):*
- ⏳ **descvt 디스패치 테이블 ~27 RVA**(`disc19_repro.rs:1185~1372`·descvt_78/c8/90 pred/aim/range 게터)=0.5.1 재배치 stale → **abil over-emit +71 원인**. ghidra 재핀 진행중(task⑯). 캡처값 예: descvt_c8→0x1f23680.
- ⬜ **HOME 게이트 abil over-emit +56** = d19_threat 기본0(threat 모델 shadow-only deferred)=0.5.0_3에도 동일 baseline → **0.5.1 회귀 아님**.
- 나머지 shadow-call RVA(threat 0x20a3fd0·usable 0x1fce700/0x1fbe950·VisionRoll 0x237d910)=기본 OFF·소스에 stale 마커. 배포 dll 4,019,200B.

**★소스 파일 구조(2026-07-15 대규모 리팩터, 의미 무변경):** `tfm2_ai_adjust.rs` 단일 16,484줄 → **메인 6,574줄 + `include!` 6파일**. ★**패치 시 RVA 단일 수정점 = `src\rva_051.rs`**(71줄=전 RVA_*/ORIG_LEN_* 상수·기존 "소스 곳곳"→이 파일 하나). 나머지 include: mem_safety.rs(270·VEH/safe_rd)·detour.rs(1,215·install_*/hook_return/apply_*)·disc19_repro.rs(3,500·my_disc19/18·dcmp하네스)·serpen.rs(1,008)·gb_kit.rs(110)·기존 genbuild_repro.rs(768)·cand_filter_repro.rs(145). 죽은코드 -3,985줄 제거(16,484→12,499)·잔여 dead_code 경고 22건=disc4 클러스터 등 의도적 보존(⚠disc4=07-12 배포·라이브검증대기 pending→소스 마커). fresh compile exit0(dll 4,017,152B, 구 4,202,496B→.preclean_051bak).

**tfm2_item_tactics 0.5.1 마이그 (0.5.0_3 → 0.5.1, ✅✅ ~~순수 RVA 마이그·⬜인게임 검증 전=DONE 미승격~~ → 정정 2026-07-15: 인게임 완전 검증완[개인전술 지정→경기중 모드템 주입 실작동]=DONE 승격. ★순수 RVA를 넘어 여러 서브시스템 재작업 필요[아래 ui_inject·strategy 세컨드훅·catalog vtable slot]. ~~⚠단 AUTO4 자동추천 itemnet 크래시만 미해결=OFF 회피[별도 잔여, 아래 E]~~ → **✅AUTO4도 복구완·인게임 검증완[net 시그강화, AUTO4 ON, 2026-07-15, 아래 E]=완전 DONE**):** SDK rlib 4종 해시=0.5.0hf2 동일=ABI 무변경·toolchain nightly-2026-05-24 무변경·**구조체 오프셋 전부 불변**(athlete +0x420/+0x450/+0x458/+0x498 등). 함수시작=ghidra-re 재-ID(HIGH)·LIVE 훅=patch-migrator mask-sig 재도출.

*LIVE 훅 8종 (mask-sig):*
| 상수 | 0.5.0_3 | 0.5.1 |
|---|---|---|
| DD_SETOPT | 0x2416070 | **0x2450f40** |
| REALLOC | 0x25ab470 | **0x25c5ae0** |
| ITEMNET_FORWARD | 0x1b78420 | **0x1bc82e0** |
| owned_cap sig(imm=sig+7) | 0x20eb870 | **0x2238410** |
| SLOT_HELPER | 0xb8d100 | **0xd81b30** |
| SLOT_BOUNDS ×4 | 0x4186d0/0x418a40/0x419120/0x419490 | **0x4b4d40/0x4b50b0/0x4b5790/0x4b5b00** |
| CL_LIVE_SPAWN ×2 | 0x473040/0x4724a0 | **0x50edd0/0x50e230** |

*함수 대개편 재-ID (ghidra-re, 전부 HIGH):*
| 함수/seam | 0.5.0_3 | 0.5.1 | 비고 |
|---|---|---|---|
| BUY_ITEM(유일 라이브 주입점) | 0x1fb8b10 | **0x1f01090** | 프롤로그 8push/sub0x38→5push/sub0x50(신 첫12B `41 57 41 56 56 57 53 48 83 EC 50 48`)·build/이름비교가 서브함수 0x1f00920로 분리(mask NONE)·인자계약 불변(r8=athlete·p6=Game@rsp_entry+0x30·Game+0x30=catalog)·**트램폴린 재배치 12B→19B**(신 clean경계 11B<12B→다음경계 +mov rax,[rsp+0xa8]=19B) |
| buy 드라이버 | FUN_1420e76e0 | **FUN_142234430** | 후계 |
| buy resolver | 0x1fb8c40 | **0x1f01170** | |
| gate3 seam(owned>2) | 0x1fb8cdd | **0x1f01448** | jbe 0x1fb8ce6(sig+9)→**0x1f0144e(sig+6)**·시퀀스 `mov rsi,[rsp+0x40];jbe`(10B)→`cmp qword[rsp+0x78],2;jbe`(7B, owned_count=[rsp+0x78] spill) |
| RUNNER_CTOR | 0x19c9470 | **0x205a2f0** | 프롤로그 6push→8push(신 첫13B `55 41 57 41 56 41 55 41 54 56 57 53 B8`·완전17B=+mov eax,0x16618)·**재배치 13B→17B**·콜사이트 6:6 1:1+UI에셋 문자열(tutorial_morgad×5+prologue_first×1) 완전일치·near-twin 0x20588a4(vtable전용) 배제 |
| SPEC_RET_RVAS ×3(관전 화이트리스트 retaddr) | 0x6d3a9f/0x6d4490/0x72f2a6 | **0x719fef/0x71a9e0/0x775456** | #1/#2=FUN_140713900 tutorial_morgad쌍·#3=prologue_first singleton |

*B. ui_inject.rs UI 프레임워크 RVA (누락됐다가 추가 수행 2026-07-15, 高신뢰·델타 +0x1a670 교차검증) — `C:\tfm2mods\tfm2_item_tactics\src\ui_inject.rs`:*
| 상수 | 0.5.0_3 | 0.5.1 |
|---|---|---|
| LOADER(제네릭 asset-get) | 0x51cd40 | **0x40f3d0** |
| PARSER | 0x2499f30 | **0x24b4590** |
| ALLOC | 0x25ab3d0 | **0x25c5a40** |
| DEALLOC | 0x25ab430 | **0x25c5aa0** |
> ⚠LOADER 0x40f3d0=scrim·draft_overlay 공유 함수(그쪽도 0.5.1 마이그 시 0x40f3d0로 갱신 대상).

*C. ★★strategy.ui 세컨드 훅 (0.5.1 핵심 회귀·해결완):* "LOADER"는 실은 제네릭 asset-get `get(registry,path,len)`. **0.5.1 재컴파일에서 바이트동일 모노모픽 copy 여러개로 분화** → main/player_info/title은 copy **0x40f3d0**, **strategy 화면만 별도 copy 0xeb17d0** 호출(strategy 빌더 0xd64770가 호출·strategy 문자열 xref 유일). 모드가 0x40f3d0만 훅해서 개인전술 드롭다운(item0m/1m/2m/item3) 삽입 실패 → **해결: ui_inject.rs에 STRAT_LOADER_RVA=0xeb17d0 세컨드 훅**(detour2+TRAMP2·install_one 리팩터·loader_body 공통화). 0x40f3d0/0xeb17d0=물리적 별개주소=이중훅 규칙 위반 아님. ⚠fan-in 100+라 detour 경량 필수. 인게임 드롭다운 복구 확인. **부수: override_info={} 비활성**(strategy remapping asset/base→asset/tfm2_item_tactics가 detour path 매칭 방해·detour mod_dd가 max_items_height 포함하므로 override 불필요). strategy.ui 자체=0.5.1 불변(89344B 동일).

*D. ★★catalog element vtable slot +8 이동 (모드템 카탈로그 스캔·ghidra-re 확증):* 0.5.1에서 catalog element vtable 앞에 predicate slot 삽입 → 이후 전 슬롯 +8. **이름 getter +0x50→+0x58**·**next_tier/레시피 getter +0x68→+0x70**. string_obj({chars@+8,len@+0x10})·element{edata@0,vtable@8,stride16}·collection{data@+8,len@+0x10}·catalog=Game+0x30 = **전부 불변**. 헬퍼 0x1f00920·BUY_ITEM 0x1f01090. 소스 4곳 수정(evt+0x50→+0x58 ×3·evt+0x68→+0x70 ×1·readable 범위). 인게임 주입 실작동 확인.

*E. ✅AUTO4 itemnet 크래시 = ~~미해결·OFF 유지·DONE 아님~~ → **복구완·인게임 검증완(net 0xd30 시그강화, AUTO4 ON, 0.5.1, 2026-07-15)**:* itemnet_forward(0.5.1=0x1bc82e0) 직접호출이 함수내부 +0x44a에서 AV(minidump 2건 faultAddr=exe+0x1bc872a 확정). **★진범 정정: forward 함수는 0.5.0_3↔0.5.1 바이트동일 불변**(itemnet_forward·인자계약·net레이아웃·ctx[11×u64]·build 전량 무변경, ghidra-re 확인) — 크래시 원인은 **모드가 net을 db+0xd30 lookalike에서 읽던 것**. 그 lookalike는 헤더 시그(16384/16384/1)만 맞고 +0x8 가중치포인터가 dangling(net 미초기화 순간)→forward가 죽은 포인터 deref하다 AV. (게임 실net=GameData+0x1558 양버전 동일. 단 모드 db=cps−0x16698 base가 GameData와 미묘히 안 맞아 db+0x1558 실패, db+0xd30이 실측 유효 net.) **수정(lib.rs)**: ①net 후보 오프셋 `[0xd30,0xda0]`→`[0x1558,0xd30,0xda0]` ②sig_ok에 **가중치포인터 readable 검증** 추가(`w=rd_u64(a+0x8); w>=0x10000 && readable(w,16384*4)`)→dangling lookalike 탈락, 가중치 살아있는 net만 통과 ③AUTO4_FORWARD_SCORE=true 재활성. ★진짜 수정효과=시그강화(오프셋보다). dangling 순간엔 net=0→AUTO4 스킵(무크래시)·유효할때 작동. **인게임 검증완**(`item_net=db+0xd30 ★유효 fwd_valid=true`·itemnet 크래시 dump 재발 0·주입 정상 write=6)·프로덕션 배포(dll 842,752B, LOG OFF).

*F. mask-sig 오판 정정 (교훈):* SLOT_BOUNDS(0x4b4d40 외 3)·DD_SETOPT(0x2450f40)·SLOT_HELPER(0xd81b30) = 처음 "오식별 의심"했으나 **ghidra-re 확증결과 mask-sig 픽 전부 정확**(오식별 아님). 초기 UI 깨짐은 이들 성급히 OFF했다가 재활성해 해결. **교훈: mask-sig 검증은 의미검증 병행하되 성급한 OFF 금지.**

**비-T1 3모드 0.5.1 배포 완료 (2026-07-15, ⬜인게임 검증 전=DONE 미승격):** tfm2_mod_scroll_fix(dll 206,848B·RVA0 순수SDK·패치무관)·Spectator_Chat(613,376B)·community_reaction_mod(1,035,264B·워크샵 3738958482 authoritative). 전부 **RVA 재도출 불요** — ★ClientDatabase 6 raw 오프셋(scene db+0x1338·played 5528/0x1598·events 5744/0x1670·MatchType db+0x1818·match_id db+0x1820·match_info.id db+0x17F8) = **0.5.1에서 0.5.0_3와 완전 동일(델타0·ghidra-re 정적검증=명령어·displacement 바이트 동일·근거함수 구 0x6df54a→신 0x725a7f·구 0x72d5e5→신 0x773795·구 0x6e390e→신 0x729e52·0.5.0_3때의 +0x10 삽입 같은 변화 없음)**. base ingame.ui도 0.5.0==0.5.1 diff=0(Spectator override 재머지 불필요). 소스 무변경·sdk_051 리빌드만·릴리스 zip 3종=`mods\release\0.5.1\`. 상세=`MEM\tfm2-spectator-chat-mod.md`·`MEM\DONE.md`.

**tfm2_draft_overlay 0.5.1 마이그·배포 (~~2026-07-15 ⬜인게임 검증 전~~ → ✅**2026-07-18 유저 인게임 검증완("잘된다")=DONE 승격**·非T1):** version-migrator 확정 RVA4 (lib.rs, OLD 0.5.0_3 → NEW 0.5.1) — ⚠**RVA4만으론 밴픽 오버레이 미발화**였고 아래 BANPICK_LOADER 행 추가로 해결:
| 상수 | 0.5.0_3 | 0.5.1 | 근거 |
|---|---|---|---|
| LOADER (L359) | 0x51cd40 | **0x40f3d0** | string-xref `"asset/base/ui/layout/main"` count17·**item_tactics LOADER와 동일 공유함수** |
| PARSER (L360) | 0x2499f30 | **0x24b4590** | UNIQUE migrate·item_tactics ui_inject 확정값과 일치 |
| ALLOC (L361) | 0x25ab3d0 | **0x25c5a40** | UNIQUE migrate·item_tactics 일치 |
| ANIM_GET (L142) | 0x51bbc0 | **0x40e250** | LOADER−0x1180 상대유도·MULTI family 후보有·검산통과·RT 킬스위치+IsBadReadPtr 가드有 → **런타임확인 권장** |
| ★**BANPICK_LOADER** (신규, 07-18) | (없음·0.5.0_3은 단일 copy) | **0xeb17d0** | **밴픽 화면이 타는 asset-get copy#2**(=item_tactics STRAT_LOADER와 동일 함수). 근거=문자열 0x374cb6d → 사이트 0x1411b0174 → `call 0x140eb17d0`(빌더 0x11b0120). **이 훅 없으면 오버레이 전혀 안 뜸** |
> ★**세컨드 훅 (0.5.1 핵심 회귀·해결완 2026-07-18)**: `BANPICK_LOADER_RVA=0xeb17d0` 훅 추가(TRAMP2+detour2, 본문=detour와 동일)·`install_one()` 일반화·~~**INSTALLED 1회 게이트 제거 → post_update마다 진입부 검사 후 내 스텁 아니면 재설치**(상대 모드가 덮어써도 다음 프레임 복구)~~ → ★**정정(2026-07-18, 0.5.1): 매프레임 재체인 폐기·INSTALLED 1회 설치 게이트 복원** — item_tactics(1회 설치)와 재설치 정책이 엇갈려 **상호 체인 사이클(draft→item→draft 무한재귀) = 게임 먹통(hang)**. draft 먼저 로드되는 환경에서만 발생(개발환경 미재현). 수정본 dll 684,544B sha256[:16]=**4db13f529eeac02a**(결함본 4d0b30602c697ed2)·zip 144.6KB 교체완(결함본=`.zip.bak_hang_20260718`)·⬜제보자 재검증 대기. 메커니즘·교훈 전문=`ANA\tfm2-draft-overlay-mod.md` 매프레임 재체인 사고 절, 수칙=`MEM\tfm2-mod-safety.md` §3. ⚠**체인 후킹 필수**: item_tactics가 같은 0xeb17d0을 STRAT_LOADER로 후킹 중 → 진입부 12B(원본 프롤로그 또는 상대 스텁 `48 b8 <tgt> ff e0`)를 트램폴린 앞에 보존해야 두 모드 detour가 순차 발화(덮어쓰면 한쪽 고아). 배포 dll **684,544B**(구 829,440B, install 구조 변경 인라이닝 차이)·상수 6종 바이너리 검증 통과·구 0x51cd40 0회·릴리스 zip `<게임>\mods\release\0.5.1\tfm2_draft_overlay.zip`(148,066B).
> ★z/히트테스트/호버 8 RVA(build_hit_tester 0x247ae70 등)는 lib.rs에 **훅/CALL 하드코딩 없음**(0x247ae70만 L476 주석에 정적언급) → **마이그 불요**. Node 레이아웃 오프셋도 item_tactics 불변확정 재사용. 빌드=DBG=false(L449) 확인·build_inj.ps1 exit0·dll **829,440B** 배포(게임 `mods\tfm2_draft_overlay\`)·ghidra-re 불요. 대시보드 파이프라인(save_probe sdk_051 재빌드·full-load검증완·apply 배포완)은 `MEM\tfm2gg-dashboard-save-probe.md`. 스텁=`MEM\tfm2-draft-overlay-mod.md`.

**tfm2_comptest_unlock 0.5.1 마이그·배포·인게임 검증 완료 (0.5.0_3 → 0.5.1, 2026-07-16, ~~⬜인게임 검증 전=DONE 미승격~~ → ✅인게임 검증완(07-16)=DONE 승격·非T1):** patch-migrator + 수동 안전화 2건. 빌드 exit0·dll **205,824B**(2026-07-16) 배포·mod.mod_info BOM 없음. 소스=`src\tfm2_comptest_unlock.rs` 10곳 RVA 상수 + init() 안전화 2건(아래). 델타 전량 orig/프롤로그 NEW exe 검증 MATCH.
| 상수 | 0.5.0_3 | 0.5.1 | 비고 |
|---|---|---|---|
| daily_remaining | 0x1a73f30 | **0x1c0b480** | MATCH |
| ★**server_dedup_real** | — (신규) | **0xf67b91** | ★**중복선수 실현 지점**(2026-07-20 신설, 0.5.1 인게임 검증완·로그 `patched+VERIFIED`). 서버 등록루프 `0xf675f0` 내 SwissSet insert(`0xf67b89`) 직후 `test al,al; jne 0xf67c6b`(=`mov dil,3` 중복거부 return). `orig [0f 85 d4 00 00 00]` → `fixed [66 0f 1f 44 00 00]`(6B nop). 패치 시 fall-through `0xf67b97`→레지스트리 find→유효성 4조건→루프 계속→`0xf67c84 mov dil,0xff`(run 진입). 소스 배열상 `allow_dup_players` **바로 앞** 배치 |
| allow_dup_players | 0xf5f2a5 | **0x1615495** | MATCH·**0.5.1 orig `75 76` 2026-07-20 실측 재확인**(→`90 90`) |
| DISP(dup경고억제) | 0xc57100 | **0xc82370** | MATCH |
| FORGE_CALLERS[0] | 0xf5f290 | **0x1615480** | MATCH |
| RUN 컨테이너 | 0xf687e0 | **0x161eab0** | mask-sig NONE→**니모닉 멀티셋 강건매칭 jaccard=1.0** 확정·클라영역 델타 +0x6b62d0 정합 |
| roster_count_gate | 0xf68aec | **0x161edbc** | RUN 내부 게이트 |
| collected_gate | 0xf68ae0 | **0x161edb0** | RUN 내부 게이트 |
| collect_err_gate | 0xf68ac8 | **0x161ed98** | RUN 내부 게이트 |
| run_push_gate | 0xf69191 | **0x161f461** | RUN 내부 게이트 |
| LOADING(진단) | 0xf76b00 | **0x162cf10** | MATCH |

> ★**0.5.1 안전화 2건(init() 소스 직접수정, 2026-07-16)**: ① **install_push_probe 비활성** — PUSH_RVA 0x101cc08=0.5.0_2 STALE인데 이 함수는 프롤로그 검증 없이 INT3 blind write=명령 중간 파괴 위험 → `let _ =`로 무력화+로그(재활성 조건=ghidra-re 재핀 후). ② **install_collect_hook 비활성** — 선수중복 sim 미러 프로토타입=~~서버 athlete_id HashMap 하드리밋(DONE 기존판정)으로~~ 기능 폐기(폐기 자체는 유효 — ⚠2026-07-20 정정: 하드리밋은 오진이었고 실제 해결은 `server_dedup_real` 서버 NOP이라 미러/치환 경로는 여전히 불요) + COLLECT_RVA 0x101d970=0.5.0_2 STALE(push8 12B 공용 프롤로그라 오설치 여지).
> ★★**중복선수 = 해결·인게임 검증완 (0.5.1, 2026-07-20)**: 위 표 신규 행 `server_dedup_real 0xf67b91` + 기존 `allow_dup_players 0x1615495` 2패치로 **같은 선수 10명 comp_test 실행 성공**(유저 확인: 경기 실행·결과창·다시보기 전부 정상). 부작용=스태미나 −5×10=−50(0 클램프)·크래시 無. ⛔**아래 §7.1 구 서술 중 "중복=서버 athlete_id HashMap 하드리밋·불가·재시도금지"는 전부 오진 무효** — 당시 NOP한 `server_dedup 0xf2bbea`는 등록 dedup이 아니라 **로스터 join 브로드캐스트 dedup**(lobby+0x600)이라 수신측 리스트가 깨져 크래시한 것을 "중복=크래시"로 잘못 일반화했고 **진짜 등록 dedup은 한 번도 건드린 적 없었음**. 크래시 근거 `0x140402840`도 함수가 아니라 `0x402730`(Rc/Arc 생성)의 EH cleanup funclet 내부·명령어 경계도 아님. ⟹ 구 `server_dedup`(rva `0xf2bbea`·`orig==fixed=[75 10]` no-op)는 **오진 주소·패치 무효**로 소스에만 잔존, **마이그 대상 아님(재핀 불요)**. 해결 후 `collect` 치환·이름/스탯 주입 전부 불요(`SUBST_ON=false`·주입 OFF — ⚠주입을 켜면 sim 입력 변경 **시작 시점이 원경기(캐시 채운 뒤)와 리플레이(캐시 기보유→첫 틱부터)에서 달라** 같은 시드라도 궤적이 갈림 = 다시보기 결과 불일치 원인, 2026-07-20 유저 관측). ⬜잔여=팀 내 raw 스탯 동일 시 init 정규화 스킵(전원 100/100) 실발생 여부 미검증. 정본=`MEM\tfm2-scrim-comptest-port.md` + `ANA\comptest-match-engine-deepdive.md` §9 배너 + DONE.md.
> **STALE 유지(fail-safe, 프롤로그/orig 검증으로 미설치·무해)**: 서버영역 6종(no_stamina 0x13ebfb2·daily_inc_gate·~~server_dedup~~=오진 주소·마이그 불요(위 07-20 주)·SRV 0x13d4af0·ENQ·INSERT)=0.5.0_3 때부터 mask-sig NONE→스킵 설계 유지(참고: 이번 migrate서 SRV=0x15d3010·ENQ=0xce0b80 UNIQUE 매치 나왔으나 무영향 진단이라 미반영). COLLECT 0x101d970/PUSH 0x101cc08/ATH_GET 0x402840=**0.5.0_2 값(0.5.0_3 마이그 때 누락 발견)**→이번에 비활성/기존 비활성이라 재핀 불요(기능 폐기). DEDUP_INS 0xca75f0/SPAWN_CP 0x13c71b0(진단)=미마이그·install_hook_n 프롤로그 게이트로 fail-safe.
> ✅ **인게임 검증완(2026-07-16, 유저 실행+INIT 로그 `mods\tfm2_comptest_unlock\tfm2_comptest_unlock.txt` 실측)**: 클라 패치 6종 전부 patched+VERIFIED(위 표 6게이트)·훅 3종 설치성공(DISP 0xc82370·RUN 0x161eab0·LOADING 0x162cf10)·서버영역 3패치(no_stamina 0x13ebfb2·daily_inc_gate 0x13e4d07·server_dedup 0x13e3773)=byte mismatch→fail-safe 스킵 정상(설계대로)·비활성 push probe/collect=비활성 로그 확인(blind write 없음)·유니크 라인업 comp_test 2회=RUN hit+LOADING ★ACCEPT(서버 수락·sim 완주 정상). 유저 확증: 체력 깎임=서버 게이트 미해제(예상)·중복인원 배치 후 시작=버튼 비활성 후 무반응(~~기존 서버 봉쇄~~ → **원인=서버 등록 dedup 미패치·2026-07-20 `server_dedup_real`로 해결**). ★**기능 범위 확정**: 0.5.1 실효=클라측만(중복 배치 허용·경고 억제·인원부족 게이트 해제·일일횟수 표시 5 고정·유니크 실행 정상)—~~중복인원 **실행**=서버 athlete_id HashMap 하드리밋으로 불가~~ → **오진 무효·해결됨(0.5.1·2026-07-20, `server_dedup_real 0xf67b91`)**. ⏳이연=서버 게이트 3종 재핀(체력 무소모·일일 실해제)=보류·유저 결정 대기(원하면 ghidra-re 별건)·인원부족 실행 여부=미확인.
> ★**임의 실존선수(distinct) 주입 = 조건부 가능·소속(팀소유) 검증 관문 없음 (2026-07-16 ghidra-re 규명·구현은 ⬜미착수)**: COLLECT/ATH_GET **0.5.1 재핀 완료**(구 STALE 대체)=**COLLECT_RVA 0x16203f0**(구 0.5.0_2 0x101d970·마스크시그 유일매치+RUN 0x161eab0 콜러 교차확인·신뢰도 高)·**ATH_GET_RVA 0xeaad40**(구 0.402840·신뢰도 高). 서버 comp_test 핸들러(0xf1d2c0, 153KB)의 수집 id 소비 3경로(등록루프 0xf329f5·stamina 0xf340c2·일일게이트 0xf2d0e8) 전부 **team_id/소속검증 부재** → athlete_id가 전역 레지스트리 **game_ctx+0x16b90**(athlete-by-id HashMap·exe 285곳)에 존재만 하면 타팀/임의 실존선수도 서버 수용. **접근법=서버 NOP 불요·클라 COLLECT 출력(id배열) 트램폴린 후킹→distinct 실존 id 10개 치환**(클라 distinct게이트 FUN_141615030=distinct 자동통과). ⚠~~**서버 하드리밋(DONE 봉쇄)=중복/위조 id에만 적용·distinct 무관.**~~ → **무효(0.5.1·2026-07-20): 하드리밋 자체가 오진·중복도 수용됨**(위 ★★ 주). 이 distinct 주입 설계도 그에 따라 **불요**(모드 `SUBST_ON=false`). 미규명=athlete team/club id 필드 오프셋·핸들러 197콜 전수(조기 소속거부 잔여가능성 낮음·배제 100% 아님). 구조사실 정본=`ANA\discovered-PROGRAM-STRUCTURE.md §14`.


#### 7.1-Z 0.5.1 마이그 기준 요약 (MEM\CURRENT.md에서 이관, 2026-07-23 — 원문 그대로)

**0.5.1 마이그 기준**(이력 — 0.5.2 도착 07-22로 종결·잔여 item_editor·scrim=폐기) — 순수 RVA 마이그(SDK/toolchain 무변경·성격=버전업급)
> 완료 항목 상세(델타표·함수시작/데이터 RVA·사이트표)=`MODS\MIGRATION.md §7.1`. 완료 판정=`MEM\DONE.md`. (아래=현행 유효 상태+미완만·과거 완료 서술은 §7.1로 이관)
- **SDK/toolchain**: `0.5.1.zip`(GitHub 릴리스)→`C:\tfm2mods\sdk_051\mod-sdk`·rlib 4종 해시=0.5.0hf2 동일=**ABI 무변경**(빌드=링크경로만)·toolchain nightly-2026-05-24.
- **migrate_rva.py OLD/NEW**: OLD=0.5.0_3(`tfm2_0.5.0_3\...exe`·buildid 24125999) / NEW=0.5.1(`tfm2_0.5.1\TeamfightManager2.exe`=Steam 설치본·buildid 24215274)·함수시작 RVA=version-migrator 강건매칭(니모닉 0.97~0.98·exe↔exe).
- **★관문 CASE-불변(재조사 방지 핵심)**: 엔티티/SimState struct 오프셋·vtable 슬롯 = 0.5.0_3=0.5.1 **동일** → 재현부는 절대 struct오프셋 계산=원본 프레임 미참조 → **함수 프레임 성장은 재현 무영향=주소스왑만으로 유효**. 성격=버전업급(함수 재정렬+프레임 광역시프트).
- **완료(✅/DONE·재검증 금지)**: item_tactics(인게임 완전검증·AUTO4 itemnet net 시그강화 해결) · ai_adjust(disc19 severity byte-patch 15/15·인게임완·소스 리팩터 단일 수정점 `src\rva_051.rs`) · 비-T1 3모드 scroll_fix/Spectator_Chat/crm(ClientDatabase 6 raw 오프셋 델타0·인게임완) · comptest_unlock(클라측 인게임완) · draft_overlay(~~배포완~~ → **인게임 검증완 07-18**: BANPICK_LOADER 0xeb17d0 세컨드훅=asset-get copy 분화 대응. ⚠단 **타 모드 공존 환경 먹통**(매프레임 재체인↔item_tactics 상호 사이클) → 1회설치 게이트 복원 수정본 배포완·⬜제보자 재검증 대기) · 대시보드 save_probe(full-load 검증완). 상세=MIGRATION §7.1 + DONE.md.
- ✅**(07-18) TFM2.gg 대시보드 파이프라인 0.5.1 대응 완료**: save_probe sdk_051 재빌드(exit0)+**실 0.5.1 세이브 full-load 검증완**(teams=120/athletes=1130/replays=4)=**세이브 포맷 0.5.0hf2=0.5.1 무변경 실증**(빌더/프론트 무변경 유효)·apply.ps1 워크샵(3738242091) 배포완(BOM fail0)·릴리스 **덮어쓰기 패치** zip `mods\release\0.5.1\TFM2_Meta_Dashboard_0.5.1_patch.zip` 2,355,953B/17엔트리(워크샵 루트 기준 경로·README_PATCH.txt 동봉·경로검증 16/16·신규0·**개인데이터 전량 제외** 922MB→2.25MB, `dev`/`save_2026` 0건) → tfm2gg-dashboard-save-probe
- ★**0.5.1 UI 주입 공통 함정(07-18)**: `.ui` asset-get이 바이트동일 copy 43개로 분화 → 화면마다 타는 copy가 다름(밴픽/strategy=**0xeb17d0**, main계열=0x40f3d0). **RVA 상수만 스왑하면 detour 조용히 미발화** → item_editor·scrim 마이그 시 대상 화면 copy 확인 필수. 상세=MIGRATION §7.1 상단 경고.
- **⬜미완/이연**: ①~~T1 프로덕션 item_editor·scrim 마이그 잔여~~ → **폐기(유저 지시 2026-07-22: 마이그 대상 8종 한정·item_editor·scrim 제외)** ②ai_adjust disc19 출력축(abil) full-output 재현=**은퇴 확정(재조사 금지)**—출력축=게임원본·판단축 byte-patch만·disc19_repro.rs=dcap-gated dev코드 잔존(프로덕션 무영향)·스킬-desc vtable 슬롯 0.5.1 불변 ③GB 서브시스템=안전-inert(INSTALL_DIAG_HOOKS=false·region D skip·fail-safe 가드)·재활성 전 재RE.


### 7.0-3 0.5.0_3 핫픽스 RVA 델타표 (★현행 소스 정본, 구 0.5.0_2 → 신 0.5.0_3)

> **핫픽스 (0.5.0_2 → 0.5.0_3, buildid 24109342→24125999, exe 69,060,608→69,047,296B, −13,312B).** **순수 RVA 재링크**(로직/구조 변경 0건). exe↔exe 마스크시그 ~30함수 UNIQUE+PROL-OK, **non-rip-rel displacement 포함 시그가 다 매칭 = 구조체 오프셋 전부 불변 확정**. **SDK 정본 = `sdk_050_hotfix2`**(game_core/game_view/mod_api rlib 실제 교체·SHA256 상이, engine_ui/native 동일) → **RVA 무영향 모드(mod_scroll_fix)도 재빌드 필수**. toolchain 불변(nightly-2026-05-24). 상세 = `MEM\tfm2-0.5.0-migration.md §16`.

**tfm2_ai_adjust** (구 0.5.0_2 → 신 0.5.0_3):
| 상수 | 구 0.5.0_2 | 신 0.5.0_3 |
|---|---|---|
| RVA_RETREAT | 0x2241710 | **0x1f37f70** |
| RVA_GENERIC_BUILD | 0x22d6120 | **0x22db820** |
| RVA_FC59A0 | 0x2265290 | **0x1f553a0** |
| RVA_PREGATE | 0x2264c00 | **0x1f54d10** |
| RVA_F80320 | 0x23737f0 | **0x2375b90** |
| RVA_CONDGATE | 0x22e22c0 | **0x19e40e0** |
| RVA_MOVEPRI | 0x22e2c20 | **0x19e4a50** |
| RVA_COMMIT_FN | 0x23660f0 | **0x19e7d30** |
| RVA_COMBAT_FN | 0x22e45f0 | **0x22e85a0** |
| RVA_TTD | 0x21a63c0 | **0x20a5030** |

**tfm2_item_editor**:
| 상수 | 구 0.5.0_2 | 신 0.5.0_3 |
|---|---|---|
| STAT_FN | 0x1d04700 | **0x1fc12b0** |
| PER_ITEM | 0x1d03db0 | **0x1fc0960** |
| SUM | 0x1d049b0 | **0x1fc1560** |

**tfm2_scrim**:
| 상수 | 구 0.5.0_2 | 신 0.5.0_3 |
|---|---|---|
| DD_SETOPT | 0x2418cf0 | **0x2416070** |
| ITEMNET_FWD | 0x1b73e50 | **0x1b78420** |
| PARSER | 0x24960d0 | **0x2499f30** |
| ALLOC | 0x25a7b80 | **0x25ab3d0** |
| DEALLOC | 0x25a7be0 | **0x25ab430** |
| LOADER | 0x4d8fb0 | **0x51cd40** |

**tfm2_draft_overlay**:
| 상수 | 구 0.5.0_2 | 신 0.5.0_3 |
|---|---|---|
| LOADER | 0x4d8fb0 | **0x51cd40** |
| PARSER | 0x24960d0 | **0x2499f30** |
| ALLOC | 0x25a7b80 | **0x25ab3d0** |
| ANIM_GET | 0x4d7e30 | **0x51bbc0** (MULTI, LOADER−0x1180 상대유도, 런타임확인 권장) |

**tfm2_item_tactics** (src/lib.rs + ui_inject.rs):
| 상수 | 구 0.5.0_2 | 신 0.5.0_3 |
|---|---|---|
| DD_SETOPT | 0x2418cf0 | **0x2416070** |
| BUY_ITEM | 0x1cfc100 | **0x1fb8b10** |
| REALLOC | 0x25a7c20 | **0x25ab470** |
| SLOT_HELPER | 0xdbf6e0 | **0xb8d100** |
| C6_SEAM | 0x1437ccb | **0x14aa3db** (resume 0x14aa3ea) |
| owned_cap (사이트/imm) | 0x21eaaa0 / 0x21eaaa7 | **0x20eb870 / 0x20eb877** |
| gate3 (CMP/JBE) | 0x1cfc2cd / 0x1cfc2d6 | **0x1fb8cdd / 0x1fb8ce6** |
| VIEW_RVA (AUTO4 로스터) | 0x22360cc (MULTI) | **0x20ae1ac** (CONFIRMED UNIQUE·`mov rax,[rcx+0x840];imul rcx,r9,0x8d0` 전체1회) |
| SLOT_BOUNDS 4곳 (`30`→`40` @+3) | 0x54b760/bad0/c1b0/c520 (오식별·폐기) | **0x4186d0 / 0x418a40 / 0x419120 / 0x419490** (재탐색 완료, UI메가함수 0x414800..0x42b4c5) |
| PARSER/ALLOC/DEALLOC/LOADER (ui_inject.rs) | (scrim 구값) | scrim과 동일 신값(0x2499f30/0x25ab3d0/0x25ab430/0x51cd40) |

> **★item_tactics 0.5.0_3 팀게이트 근본재설계(2026-07-09, dll 722,944B, ⬜인게임검증)**: 0.5.0_2 SIDE_VOTE(전역 다수결) 폐기·버그4종 수정 → 경기별 결정적 `player_side_for_match(athlete)`(로스터스캔). VIEW_RVA/SLOT_BOUNDS 위 표에서 해소(구 MULTI/NONE 확정). athlete CONFIRMED: champ+0x420/0x428·목표빌드Vec+0x450·owned_count+0x458·build ptr+0x498/len+0x4a0/cap+0x490·gold+0x888·side+0x820. 상세=`MEM\tfm2-item-slot-count.md`(0.5.0_3 팀게이트 재설계).

**tfm2_comptest_unlock** (클라측 패치):
| 상수 | 구 0.5.0_2 | 신 0.5.0_3 |
|---|---|---|
| RUN_handler | 0x101c030 | **0xf687e0** |
| LOADING | 0x102a2c0 | **0xf76b00** |
| daily_remaining | 0x1a6e0b0 | **0x1a73f30** |
| FORGE_CALLER | 0x1012b10 | **0xf5f290** |
| DISP | 0xcd1e60 | **0xc57100** |
| allow_dup_players | — | **0xf5f2a5** |
| roster_count_gate | — | **0xf68aec** |
| collected_gate | — | **0xf68ae0** |
| collect_err_gate | — | **0xf68ac8** |
| run_push_gate | — | **0xf69191** |
| CT_CLIENT_LO / HI | 0x1010000 / 0x1030000 | **0xf50000 / 0xf80000** |

**tfm2_mod_scroll_fix**: 하드코딩 RVA 0개 = RVA 마이그 불요. **단 SDK rlib 교체로 재빌드는 필요**(sdk_050_hotfix2).

**Spectator_Chat**: 컴파일 RVA 없음(순수 raw 오프셋). 주석 3개만 0.5.0_3 값 갱신 — ui_parser **0xf1e760** / on_mouse_down **0x100a3e0** / on_mouse_move **0x100a8c0**. DraggablePopup 오프셋 불변·로직 무수정, SDK 재빌드. ⚠**정정(2026-07-09): 엔진 ClientDatabase struct가 scene 앞 +0x10 필드추가로 이동** → `lib.rs:582-583` LIVE_PLAYED_OFF 5512→**5528**, LIVE_EVENTS_OFF 5728→**5744**(라이브 경기 채팅 미표시로 발각·스캔 실증·재빌드 613,376B). 상세=`MEM\tfm2-0.5.0-migration.md §16.7`.

**community_reaction_mod**: 순수 SDK, 하드코딩 RVA 0개 = RVA 마이그 불요. SDK 재빌드만(0.4.14 이후 첫 0.5.0계열 빌드). ⚠**정정(2026-07-09): 엔진 ClientDatabase +0x10 이동** → `lib.rs:34,401-406` LIVE_EVENTS_OFF **5744**·scene **0x1338**·mt_tag **0x1818**·mid **0x1820**·mid2 **0x17F8**(Spectator_Chat과 동일 오프셋 공유·재빌드 1,035,264B·워크샵 3738958482 dll 교체). §16.7.

> ✅ **빌드·배포 완료(2026-07-09, sdk_050_hotfix2 재빌드)**: mod_scroll_fix **206,848B** · community_reaction_mod **1,035,264B** · Spectator_Chat **613,376B** · item_tactics **718,336B**. ⬜인게임 검증 미실시. 나머지 RVA 모드(ai_adjust/item_editor/scrim/comptest_unlock)는 상수·표 확정, 이번 세션 빌드·배포는 위 4모드만.
> ✅ **draft_overlay 0.5.0_3 재빌드·배포 완료(2026-07-14, sdk_050_hotfix2)**: 소스 lib.rs는 이미 0.5.0_3 마이그 완료(상수 4개 위 표=ANIM_GET 0x51bbc0/LOADER 0x51cd40/PARSER 0x2499f30/ALLOC 0x25ab3d0, 구0.5.0_2값 주석 이력). 문제=배포 dll이 0.5.0_2 빌드(7/8, 768000B, stale)라 라이브 0.5.0_3서 훅 미개입 → `DBG=true→false`(L448) 후 재빌드·배포 **dll 742,912B(Jul14 11:50, verified)**, stale overlay_debug.txt(2.8MB) 제거. version-migrator 독립검증=0.5.0_2→0.5.0_3 순수 핫픽스(RVA-only, 구조체 오프셋 불변, UI 오프셋 crash위험 0). ⬜인게임 밴픽 팝업 표시 검증 대기.

> ⬜ **ghidra-re 후속 큐(0.5.0_3 Ghidra 업로드 후, capstone NONE/MULTI)**:
> - comptest 서버영역 0x13xxxxx: no_stamina(0x13ebfb2 근처)·daily_inc_gate·server_dedup·SRV_handler/CT_REGION(0x13d4af0)·ENQ·INSERT(MULTI) — mask-sig NONE, fail-safe 스킵(서버측 스태미나/일일한도/선수중복 게이트 재핀 전 부분미작동).
> - ✅**item_tactics VIEW_RVA/SLOT_BOUNDS 해소(2026-07-09)**: VIEW_RVA=**0x20ae1ac**(UNIQUE 확정)·SLOT_BOUNDS 4곳=**0x4186d0/0x418a40/0x419120/0x419490**(구 0x54b760/bad0/c1b0/c520=오식별 폐기)·SLOT_HELPER 0xb8d100. §7.0-3 표 반영·소스 반영 완료. SETTER_NOP(비활성)만 잔여.
> - ai F2_BUILD_CALL(0x22dd4fe)/COMMIT_CALL(0x1e3dfd2) — OLD서도 콜사이트 부재=원래 inert stale(핫픽스 무관).

### 7.0-2 0.5.0_2 핫픽스 RVA 델타표 (직전 베이스, 구 0.5.0 → 신 0.5.0_2)

> **핫픽스 (0.5.0 → 0.5.0_2, buildid 24102827→24109342, exe 69,048,320→69,060,608B, +12,288B).** **순수 RVA 재링크**(로직/구조 변경 0건, 함수 120,633→120,698). 델타는 구간별 비균일(재컴파일 재정렬). **toolchain/SDK 무변**(nightly-2026-05-24, sdk_050, mod_info dep `>=0.5.0,<0.6.0`).
> - **★배포 상태(2026-07-08)**: 5모드 중 **4개 재빌드·배포**(게임 종료상태서 성공) — item_editor(481,792B)·scrim(3,864,576B)·draft_overlay(768,000B)·item_tactics(726,016B). **mod_scroll_fix=RVA 0개(순수 SDK)라 미변경.**
> - **★ITEMNET_FORWARD stale 교정(재조사 방지)**: item_tactics AUTO4 forward 상수 0x19f01a0은 **0.5.0부터 이미 stale**였음(push8 프롤로그 아님 → 가드 `itemnet_addr_valid()`가 fail-safe로 AUTO4 채점 무발화 중). 이번에 scrim 검증본과 동일 함수 **0x1b73e50**으로 동기화. ⬜인게임 AUTO4 정상화 확인 필요(크래시 무위험).

**item_editor** (델타 −0x14fa60, dll 481,792B):
| 상수 | 구 0.5.0 | 신 0.5.0_2 |
|---|---|---|
| STAT_FN (L77) | 0x1e54160 | **0x1d04700** |
| PER_ITEM (L913) | 0x1e53810 | **0x1d03db0** |
| SUM (L956) | 0x1e54410 | **0x1d049b0** |

**scrim** (src/lib.rs, dll 3,864,576B):
| 상수 | 구 0.5.0 | 신 0.5.0_2 |
|---|---|---|
| DD_SETOPT (L59) | 0x24167b0 | **0x2418cf0** |
| ITEMNET_FWD (L2583) | 0x1b14f00 | **0x1b73e50** |
| PARSER (L4986) | 0x2493b90 | **0x24960d0** |
| ALLOC (L4987) | 0x25a5620 | **0x25a7b80** |
| DEALLOC (L4989) | 0x25a5680 | **0x25a7be0** |
| LOADER | 0x4d8fb0 | 0x4d8fb0 (불변) |

**draft_overlay** (src/lib.rs, dll 768,000B):
| 상수 | 구 0.5.0 | 신 0.5.0_2 |
|---|---|---|
| PARSER (L359) | 0x2493b90 | **0x24960d0** |
| ALLOC (L360) | 0x25a5620 | **0x25a7b80** |
| LOADER (L358) | 0x4d8fb0 | 0x4d8fb0 (불변) |
| ANIM_GET (L141) | 0x4d7e30 | 0x4d7e30 (불변) |

**item_tactics** (src/lib.rs + ui_inject.rs, dll 726,016B):
| 상수 | 구 0.5.0 | 신 0.5.0_2 |
|---|---|---|
| DD_SETOPT | 0x24167b0 | **0x2418cf0** |
| RVA_BUY_ITEM | 0x1e4bb60 | **0x1cfc100** |
| RVA_REALLOC | 0x25a56c0 | **0x25a7c20** |
| RVA_SLOT_HELPER | 0xdc2390 | **0xdbf6e0** |
| C6_RVA (주입 seam) | 0x143593b | **0x1437ccb** |
| C6_RESUME | 0x143594a | **0x1437cda** |
| owned_cap (사이트/imm8) | 0x2271db0 / 0x2271db7 | **0x21eaaa0 / 0x21eaaa7** |
| gate3 (CMP/JBE) | 0x1e4bd2d / 0x1e4bd36 | **0x1cfc2cd / 0x1cfc2d6** |
| SETTER_NOP | 0xf2eb89 | **0xf2a899** |
| PARSER/ALLOC/DEALLOC | (scrim 구값) | scrim과 동일 신값(0x24960d0/0x25a7b80/0x25a7be0) |
| LOADER | 0x4d8fb0 | 0x4d8fb0 (불변) |
| SLOT_BOUNDS 4곳 | 0x54b760/bad0/c1b0/c520 | 전부 불변 |

> ⬜ **비활성 진단훅(이번 무영향, 재활성시만 재RE 필요)**: item_tactics PUSH/BS/SIM/CAND_GATE/VIEW/RET/beam/INJECT/WP 등 시그미스매치(전부 게이트 false).

### 7.0 0.5.0 RVA 표 (★현행, 구 0.4.14h → 신 0.5.0)

> **버전업 (0.4.14→0.5.0, buildid 24102827).** 대부분 RVA-only 이동(스켈레톤해시 UNIQUE 매칭·재링크만) + AI 디스패처는 구조 리팩터로 NOMATCH(재RE 필요).
> - **SDK = `sdk_050`** · **toolchain = nightly-2026-05-24** · **`mod.mod_info` dep 범위 = `>=0.5.0,<0.6.0`**. (구 0.4.14 SDK·dep로 빌드하면 로드 거부/desync.) build_inj.ps1 **L10 SDK경로 sdk_0414_new→sdk_050 교체**.
> - "구" 컬럼 = 각 모드 소스에 들어있던 0.4.14h 상수값. "신 0.5.0" = 스켈레톤해시 UNIQUE 매칭 신 RVA.
> - **★배포 상태(2026-07-08, 승격)**: **5종 전부 상수반영+빌드+배포 완료** — item_editor(484,352B) · mod_scroll_fix(206,848B, RVA 0개 순수 SDK) · scrim(3,867,136B) · draft_overlay(768,000B) · **item_tactics(663,040B, 코어 ENABLED·빌드 exit0)**(⚠인게임 검증 대기 — 게임 실행중, 메이저업이라 구조체 오프셋 잔여리스크). **item_tactics = 코어 3기능 ENABLED 배포**(구 "미배포"→승격): 4번째 실구매+경기중 slot3 UI+#item3 드롭다운+**AUTO4 자동선택**(2026-07-08 재활성화 완료). ⛔여전히 게이트오프(후속 RE): C6_ENABLED(personal_tactics 주입, 심층 RE 진행중)·SETTER_NOP_ENABLED(delegate revert-NOP, B 종속) → §7.0 item_tactics 표·NOMATCH 참조. ai_adjust·move_guard·대시보드=별도 세션. 정본 = `MEM\tfm2-0.5.0-migration.md §5`.

**item_editor** (전투스탯 주입 트램폴린 3개, 전부 UNIQUE):
| 상수 | 구 0.4.14h | 신 0.5.0 | 매칭 |
|---|---|---|---|
| STAT_FN (FUN 최종계산) | 0x1e412e0 | **0x1e54160** | UNIQUE |
| PER_ITEM (키매칭합산) | 0x1e40990 | **0x1e53810** | UNIQUE |
| SUM (단순합산) | 0x1e41590 | **0x1e54410** | UNIQUE |

**scrim / item_tactics** (드롭다운·itemnet forward):
| 상수 | 구 0.4.14h | 신 0.5.0 | 매칭 |
|---|---|---|---|
| DD_SETOPT (DropdownRunner 옵션set) | 0x218a5f0 | **0x24167b0** | UNIQUE |
| ITEMNET_FWD (아이템 자동빌드 신경망 forward) | 0x19f01a0 | **0x1b14f00** | UNIQUE |
| ui_kit DD_SETOPT (공유 모듈) | 0x21184e0 | **0x1fa5e30** | UNIQUE |

**scrim / draft_overlay** (UI 로더/파서/얼록):
| 상수 | 구 0.4.14h | 신 0.5.0 | 매칭 |
|---|---|---|---|
| PARSER (`.ui` 텍스트 파서) | 0x220e100 | **0x2493b90** | UNIQUE |
| ALLOC (alloc) | 0x231fb70 | **0x25a5620** | UNIQUE |
| DEALLOC (dealloc) | 0x231fbd0 | **0x25a5680** | UNIQUE |
| LOADER (에셋게터, 체이닝 로더훅) | 0x540ad0 | **0x4d8fb0** | MULTI→위치확정 |
| ANIM_GET (draft_overlay 에셋게터) | 0x53f950 | **0x4d7e30** | MULTI→위치확정 |

> ⚠ LOADER/ANIM_GET = asset-getter 제네릭 가족(해시 MULTI). **위치+상대거리 보존으로 확정**(ANIM 0x4d7e30 +0x1180 = LOADER 0x4d8fb0). 배포 전 런타임 프롤로그 확인 권장(**미수행**).

**ai_adjust primitive** (보존 primitive — 재작성 대상 아님, 재링크만):
| 상수 | 구 0.4.14h | 신 0.5.0 | 매칭 |
|---|---|---|---|
| FCD980 | 0x18b1550 | **0x1a86f40** | UNIQUE |
| FCDAF0 | 0x18b16c0 | **0x1a870b0** | UNIQUE |
| CHACHA (PRNG 코어) | 0x2245cf0 | **0x24cb7e0** | UNIQUE |
| COMBAT_FN (전투물리) | 0x1fdb5b0 | **0x1aab0d0** | UNIQUE |
| A1DA50 | 0x1a24720 | **0x2201030** | UNIQUE |

**item_tactics** (push/후보게이트/슬롯헬퍼):
| 상수 | 구 0.4.14h | 신 0.5.0 | 매칭 |
|---|---|---|---|
| PUSH | 0x1e4f310 | **0x1e60b70** | UNIQUE |
| CAND_GATE (후보 필터 게이트) | 0x1a35490 | **0x1b654c0** | UNIQUE |
| SLOT_HELPER (slot3 경로) | 0xbbbd60 | **0xdc2390** | UNIQUE |
| REALLOC (__rust_realloc 실fn, build Vec 확장) | 0x231fc10 | **0x25a56c0** | UNIQUE |
| BUY_ITEM | 0x2052ca0 | **0x1e4bb60** | ✅재RE확정 (프롤로그 12B replace-detour) |
| 구매 resolver | 0x2052dd0 | **0x1e4bc90** | ✅재RE확정 (gate3=0x1e4bd36 JBE→JMP) |
| run_tick_ext | 0x1e2f2a0 | **0x226dc20** | ✅재RE확정 (owned_cap 패치사이트 0x2271db0, imm8 @0x2271db7) |

> REALLOC: 구 0x88c700은 thunk, 실fn = 0x231fc10→0x25a56c0.
> **★item_tactics 0.5.0 코어 완료·배포(2026-07-08 승격)**(ghidra_beta=0.5.0): BUY_ITEM/resolver/run_tick_ext/gate3/owned_cap 전부 신RVA 확보 → **코어 3기능 ENABLED 구현·빌드(dll 663,040B, exit0)·배포**. ⚠**구조체 오프셋 비균일 이동(0.4.14→0.5.0)**: owned +0x3d0→**+0x458**, build Vec cap+0x408→**+0x490**·ptr+0x410→**+0x498**·len+0x418→**+0x4a0**, 슬롯배열 +0x3c8→**+0x450**, 골드 +0x710→**+0x888**, champ name ptr/len **+0x420/+0x428**·readable바운드 **+0x4a8**. **경기중 slot3 UI 패치사이트(신)**: SLOT_BOUNDS 4곳 **0x54b763/0x54bad3/0x54c1b3/0x54c523** `30`→`40` + 헬퍼 0xdc2390. **#item3 드롭다운**: ui_inject(LOADER 0x4d8fb0/PARSER 0x2493b90/ALLOC 0x25a5620/DEALLOC 0x25a5680) + FN_DD_SETOPT 0x24167b0 + 선택폴링 +0x1788(불변). 드롭다운 오프셋 +0x1788/+0x1528/+0x1570/+0x1150/+0x1154 전부 불변. stale 0.4.x 주소=enabled 경로에 0(grep 검증). 상세표·바이트패턴 = `MEM\tfm2-item-slot-count.md`(§0.5.0). **✅AUTO4_FORWARD_SCORE=true 재활성화 완료(2026-07-08, dll 690,688B)**: 신경망 자동 4번째 view 룩업 함수시작 **0x22360c0**·detour mid-func **0x22360cc**(VIEW_PROLOGUE 14B `48 8B 81 40 08 00 00 49 69 C9 D0 08 00 00`), 로스터 ptr +0x808→**+0x840**·count +0x810→**+0x848**·stride 0x758→**0x8d0**, 원소 champ +0x398→**+0x420**·team +0x6a8→**+0x820**·pos +0x738→**+0x8b0**(비균일), view=base−0x840, net=Database+0xda0(불변)·Database=cps−0x16698. **✅C6_ENABLED=true 복구(구현·빌드·배포 dll 699,392B, ⬜인게임 검증 대기, 2026-07-08)**: ★2차 오판정정 — tactics-loop가 candidate-build(0x10620e0)서 **독립함수 FUN_1414357e0(RVA 0x14357e0)로 추출**(macro_op 함수분리)이지 소멸 아님. **새 주입 seam(단일)=RVA 0x143593b**(slot0 카테고리 read `movzx ecx,[rbp-8]`, 0.4.14 이중seam 0xc76a89/0xc76d81 통합 analog, 양팀·3슬롯 단일경유, resume=0x143594a). personal_tactics=Team+0x348 SwissTable(ctrl+0x348/mask+0x350/len+0x360/hash+0x368, 버킷 stride0x20, 챔프키[bkt-0x18/-0x10]·카테고리 3B[bkt-8/-7/-6]), 카테고리→아이템 JT 0x374c610/62c/648 cat1→4·2→24·3→9·4→14·5→19·6→29=VANILLA_FINAL·cat0 zero-skip. 트램폴린: rax=elem LIVE→movzx로 dead된 rcx 사용(`48 b9 <stub> ff e1`)+push/pop rax로 elem 보존+register-free jmp qword[rip+0]. SETTER_NOP_ENABLED=false 유지(RVA만 0xf1a74b→**0xf2eb89**, revert콜 0xf2eb89 vs 0xf2edd8 런타임 미확정·#item3=모드주입이라 revert무관·delegate 카테고리는 C6로 커버). ⚠인게임 검증 필요(C6_FIRED·cat0 4아이템 반영·병렬sim 오염). 상세=`MEM\tfm2-item-slot-count.md`(§0.5.0 C6 seam).

**move_guard 빌더** (이동가드):
| 상수 | 구 0.4.14h | 신 0.5.0 | 매칭 |
|---|---|---|---|
| MOVE_GUARD_BUILDER | 0x1b94c50 | **0x1f6f610** | UNIQUE |

**move_guard 0.5.0 패치사이트(신)** — ⚠⚠ **정정(2026-07-08, 런타임 프로브 실측)**: 구 "small_action.rs 단일 apply `FUN_14231e3a0` 통합, 패치 0x231e3f0·0x231e4a5 2사이트"는 **오조사·폐기**(정적 함수-접기 오인). 0.5.0도 0.4.14와 **동일 per-effect 파일**(주소만 이동). 패닉헬퍼=unwrap-None **FUN_142bee240(0x2bee240)**. 유효 패치 **3사이트**:
| 사이트(파일:줄) | apply RVA | 패닉JZ RVA | 구 바이트 | 스킵목적지 | 신 바이트 |
|---|---|---|---|---|---|
| rush_move_to_back.rs:29 | 0x1e7d320 | 0x1e7d3c6 | `0F84A9030000` | 0x1e7d761 | `0F8495030000` |
| moveback.rs:20 | 0x1f69f70 | 0x1f69fc6 | `7427`(short je) | 0x1f69fe3 | `741B` |
| move_to.rs:83 | 0x1f9dea0 | 0x1f9df1c | `0F84F7010000` | 0x1f9dec0 | `0F849EFFFFFF` |

미적용(0.5.0): move_to.rs 24/33/29=단일 case 통합(부재)·rush_time.rs=소스문자열 부재·airborne.rs(FUN_1421a1630)=이미 clean skip 가드(패닉=bounds-check뿐)→패치불요. ✅런타임 검증(2026-07-08 멈춤 해소). 상세=`MEM\tfm2-0.5.0-migration.md §3`.

**★NOMATCH (로직 변경 = 재RE 필요, → ghidra-re 위임):**
- **ai_adjust plan_v2 디스패처 전부** → 0.5.0에서 `macro_op`/`team_op` 구조로 재작성됨. 보존 primitive(위 표)는 재링크로 살지만 디스패처 층은 전면 재RE. (정본 앵커 = `MEM\tfm2-0.5.0-migration.md` §6 1차RE / `MEM\tfm2-0.5.1-migration.md` 베타 디컴.)
- **item_tactics — ✅완전 마이그(C6 포함), SETTER_NOP만 런타임대기**(2026-07-08, 구 "AUTO4까지 복구·C6만 후속"→C6 복구 승격; C6=구현·배포됨 ⬜인게임 검증 대기):
  - ✅**코어 3기능 ENABLED·빌드·배포**: ①4번째 실구매(BUY_ITEM 0x2052ca0→**0x1e4bb60** + __rust_realloc **0x25a56c0** + owned_cap imm8 **0x2271db7** `03`→`04` + gate3 0x2052e76→**0x1e4bd36** JBE `76`→JMP `EB`) ②경기중 slot3 UI(SLOT_BOUNDS 4곳 **0x54b763/0x54bad3/0x54c1b3/0x54c523** `30`→`40` + 헬퍼 0xdc2390) ③#item3 드롭다운(ui_inject + FN_DD_SETOPT 0x24167b0 + 선택폴링 +0x1788). run_tick_ext 0x1e2f2a0→**0x226dc20**. 상세=위 표 + `MEM\tfm2-item-slot-count.md`.
  - ✅**AUTO4_FORWARD_SCORE=true 재활성화 완료(dll 690,688B)**: 신경망 자동 4번째 view 룩업 함수시작 **0x22360c0**·detour mid-func **0x22360cc**(VIEW_PROLOGUE 14B `48 8B 81 40 08 00 00 49 69 C9 D0 08 00 00`=mov rax,[rcx+0x840]+imul rcx,r9,0x8d0), 로스터 배열ptr +0x808→**+0x840**·count +0x810→**+0x848**·stride 0x758→**0x8d0**, 원소 champ +0x398→**+0x420**(name/id 0x420/0x428)·team +0x6a8→**+0x820**·pos +0x738→**+0x8b0**(⚠비균일). view=base−0x840. **불변**: net=Database+0xda0, Database 베이스=cps−0x16698(disp 0x16698/0x16690/0x15d78 잔존 확인).
  - ✅**C6_ENABLED=true 복구(구현·빌드·배포 dll 699,392B, ⬜인게임 검증 대기)**: ★2차 오판정정 — tactics-loop가 candidate-build(0x10620e0)서 **독립함수 FUN_1414357e0(RVA 0x14357e0)로 추출**(macro_op 함수분리, 소멸 아님). **새 주입 seam(단일)=RVA 0x143593b**(slot0 카테고리 read `movzx ecx,[rbp-8]`, 0.4.14 이중seam 0xc76a89/0xc76d81 통합 analog, 양팀·3슬롯 단일경유, resume=0x143594a). personal_tactics=Team+0x348 SwissTable(ctrl+0x348/mask+0x350/len+0x360/hash+0x368·버킷 stride0x20·챔프키[bkt-0x18/-0x10]·카테고리 3B[bkt-8/-7/-6]), JT 0x374c610/62c/648 cat1→4·2→24·3→9·4→14·5→19·6→29=VANILLA_FINAL·cat0 zero-skip. seam 진입 rax=elem LIVE→movabs rax금지→**movzx로 dead된 rcx 사용(`48 b9 <stub> ff e1`)+push/pop rax로 elem 보존+register-free jmp qword[rip+0]**, tail=movzx ecx,[rbp-8]+movabs rdx,0x14374c610+movsxd rcx,[rdx+rcx*4]+jmp 0x143594a, C6_ORIG_LEN=12. **SETTER_NOP_ENABLED=false 유지**(RVA만 0xf1a74b→**0xf2eb89**, revert콜 0xf2eb89 vs 0xf2edd8 런타임 미확정·#item3=모드주입이라 revert무관·delegate 카테고리는 C6로 커버). **배제**: FUN_141e54680=B 아님(엔티티 거리로직). ⚠인게임 검증 필요(C6_FIRED·cat0 4아이템 반영·병렬sim 오염 가능성).
  - ✅**구조체 오프셋 변경 확정**(구 "의심"→확정): owned +0x3d0→**+0x458**(+0x88), build Vec +0x408/0x410/0x418→**+0x490/0x498/0x4a0**, 슬롯배열 +0x3c8→**+0x450**, 골드 +0x710→**+0x888**(+0x178, 비균일), champ name ptr/len **+0x420/+0x428**·readable바운드 **+0x4a8**. macro_op 리팩터 정황 실증됨.
- **draft_overlay ANIM_GET** — ✅**해소됨 0x4d7e30**(위 scrim/draft_overlay 표, 위치+상대거리 확정; string-xref 불요, 런타임 프롤로그 확인 미수행).

---

> **(이력) ★0.4.14 갱신완료 (2026-06-24, exe 65,923,584B, buildid 23869708, 버전 "0.4.14") — STALE, 현행=0.5.0(§7.0)**
> - **toolchain = nightly-2026-05-24** (0.4.13의 06-16서 변경! mod_api ABI 변경) + **SDK = `mod_sdk\0.4.14\mod-sdk`**. build_inj.ps1 갱신완료.
> - **UI RVA(0.4.14)**: ui_inject LOADER **0x6e46d0**/PARSER **0x2215960**/ALLOC **0x2327370**/DEALLOC **0x23273d0** · item_editor STAT_FN **0x1da7980**/PER_ITEM **0x1da7030**/SUM **0x1da7c30** · scrim DD_SETOPT **0x213fbd0**/ITEMNET_FWD **0x19f82d0**.
> - **plan_reimpl RVA + AI 변경진단 정본 → `MEM\tfm2-0.4.14-migration.md`** (condgate/movepri/recall 무변경, retreat/generic_build/f80320 로직변경).
> - migrate_rva.py/migrate_plan.py/anchor_plan.py/check_prologue.py OLD=`tfm2_0.4.13_5`/NEW=게임폴더로 갱신됨. **exe 백업: 0.4.13_5=`tfm2_0.4.13_5\`, 0.4.14=`tfm2_0.4.14\`. 다음 패치=현 게임exe를 `tfm2_0.4.15\`로 백업 후 migrate.**
> - 4모드 빌드·배포 완료: ui_inject 246784 / item_editor 483328 / plan_reimpl 948224 / scrim 3525632 B.

**(구) "hotfix4"(0.4.13_5, exe 65,778,176B) 표 — STALE, 0.4.14값은 위/메모리 참조** (모드 소스에 들어가 있던 값 = migrate_rva.py 의 직전 TARGETS).
**exe 백업:** hotfix2=`tfm2_0.4.13_3\`, hotfix3(06-18)=`tfm2_0.4.13_4\`, **hotfix4(06-18 21:18)=`tfm2_0.4.13_5\`** (이전: 0.4.13=`_(없음)`, hotfix06-17=`_2\`). **패치 받으면 먼저 현재 게임exe를 `tfm2_0.4.13_6\` 로 백업 후 migrate.**
| 모드 | 상수 (파일:줄) | hotfix2 | hotfix3 | **hotfix4(현재)** | 확정법 |
|---|---|---|---|---|---|
| ui_inject | LOADER_RVA (lib.rs:18) | 0x61d490 | 0x61d4a0 | **0x7cc820** | string-xref 17→ (프롤로그 8-PUSH 동일) |
| ui_inject | PARSER_RVA (lib.rs:19) | 0x2206fb0 | 0x21eb2b0 | **0x21f9700** | 마스크 유일 |
| ui_inject | ALLOC_RVA (lib.rs:20) | 0x2316bd0 | 0x22faeb0 | **0x2309320** | 마스크 유일 |
| ui_inject | DEALLOC_RVA (lib.rs:21) | 0x2316c30 | 0x22faf10 | **0x2309380** | 마스크 유일(미사용) |
| item_editor | STAT_FN_RVA (lib.rs:77) | 0x1ce05b0 | 0x1deea60 | **0x1cfebc0** | 마스크 유일 |
| item_editor | PER_ITEM_RVA (lib.rs:913) | 0x1ce0130 | 0x1dee5e0 | **0x1cfe740** | 마스크 유일 |
| item_editor | SUM_RVA (lib.rs:956, FUN_141b86380) | 0x1ce0860 | 0x1deed10 | **0x1cfee70** | 마스크 유일 |
| scrim | FN_DD_SETOPT_RVA (lib.rs:59) | 0x212e990 | 0x2112800 | **0x21216b0** | 마스크 유일 |
| scrim | ITEMNET_FORWARD_RVA (lib.rs:2227) | 0x19fdb30 | 0x19f1cd0 | **0x19c8af0** | 마스크 유일 |

toolchain: nightly-2026-06-16. 메모리 상세: `MEM\tfm2-hotfix-migration-tooling.md`. **hotfix4 마이그 = 9개 전부 깔끔 (8개 마스크 유일 + asset게터 string-xref 0x7cc820, 프롤로그 8-PUSH 바이트동일 확인). asset게터는 hotfix3→4 에서 0x61d4a0→0x7cc820 크게 이동(블록 자체가 .text 후방으로 점프).**

---

## §7.2-A · 0.5.2 ai_adjust 잔여 RVA — ghidra-re 확정분 (2026-07-22, ghidra-re 세션)

> ⚠**환경 제약**: Ghidra 8080/8081 **두 인스턴스 모두 0.5.2**(Ghidra 상에서는 신구 대조 불가 — 0.5.1 대조가 필요하면 **0.5.1 exe를 Ghidra에 별도 로드**할 것).
> ~~0.5.1 exe 백업도 디스크에 없음~~ → **오류 정정(2026-07-22)**: 백업은 **있다**. ★**역대 exe 백업 위치 = `C:\Users\dev\Desktop\claude\tfm2\tfm2_<버전>\TeamfightManager2.exe`** (0.4.10~0.5.2 **17개 보관중**, 0.5.2도 `tfm2_0.5.2\`에 존재). 0.5.1 = `tfm2_0.5.1\TeamfightManager2.exe` (69,233,664B, 2026-07-15 16:16 실측). version-migrator는 실제로 이 파일을 OLD로 써서 exe↔exe 매칭함. ghidra-re가 `ANA\` 상위인 `Desktop\claude\tfm2\`를 안 본 탓. ⇒ **"대조 수단 전무"로 오판해 헛수고하지 말 것.**
> → 이번 ghidra-re 판정은 **exe 내재적 근거**(시맨틱·상대오프셋·상수 유일성·xref 경로)만으로 수행됨. capstone 정적 스캔 사용.

### ★최우선 정정: "disc19 severity 아웃라이닝 분리"는 **오판정이었음**
0.5.2 exe에는 severity 블록(`cmp rax,0x31`→`cmp hp,0x41`→…)이 **6군데** 인라인 복제돼 있다
(0x22e3cdf / 0x22edb5f / 0x22effff / 0x22f8d6e / **0x2380e16** / 0x23a0c21).
자동매칭이 "분리된 신규 함수"로 지목한 **0x22f8a90은 disc19가 아니라 남의 핸들러**(자기 사본 보유, hp레지스터=R15, tr9 imm=**9**).
disc19 사본만 tr9 imm=**0xa**이고 **ally 0x32 ×2 + rhB 0x2e가 뒤따르는** 유일 사본 → **DISC19_HANDLER=0x2380820 재확인(확정)**.
⇒ `apply_disc19_imm`을 0x22f8a90 쪽으로 배선했다면 **완전한 오패치**가 될 뻔했음.

### 1) disc19 severity 15사이트 → **10 확정 / 5 소멸**
컨테이너 = DISC19_HANDLER **0x2380820**. hp_pct 레지스터 **R15 → RSI**(`49 83 ff` → `48 83 fe`), tr(RAX)·ally(RAX) 불변.
시맨틱 확증: 0x2380dab `mov rsi,rax`(=imul rdi,0x64 → div → HP%), 0x2380e79 `imul rax,[rax+0x658],0x64`+`div rcx`→`cmp rax,0x32`(ally#1) = 0.5.1 서술과 일치.

| 사이트 | 0.5.1 | **0.5.2** | prefix | imm@ | 비고 |
|---|---|---|---|---|---|
| tr49 | 0x1e0e503 | **0x2380e16** | `48 83 f8` | +3 | 불변 |
| hp66 | 0x1e0e509 | **0x2380e1c** | `48 83 fe` | +3 | ★R15→RSI |
| tr29 | 0x1e0e50f | **0x2380e22** | `48 83 f8` | +3 | 불변 |
| hp41 | 0x1e0e515 | **0x2380e28** | `48 83 fe` | +3 | ★R15→RSI |
| tr17 | 0x1e0e51b | **0x2380e2e** | `48 83 f8` | +3 | 불변 |
| hp26 | 0x1e0e523 | **0x2380e36** | `48 83 fe` | +3 | ★R15→RSI |
| tr9  | 0x1e0e529 | **0x2380e3c** | `48 83 f8` | +3 | 불변 |
| ally#1 | 0x1e0e589 | **0x2380e92** | `48 83 f8` | +3 | 불변(64bit div) |
| ally#2 | 0x1e0e5d5 | **0x2380ec0** | `48 83 f8` | +3 | 불변 |
| rhB | 0x1e0e5e2 | **0x2380ecd** | `48 83 fe` | +3 | ★R15→RSI |
클러스터 내부 간격(6,6,6,6,8,6)·ally#2→rhB 간격 0xd가 0.5.1과 **완전 보존** = 교차확증.

⛔**소멸 5사이트**: rhA(`0x2d`)·pa#1/#2/#3(`0x26`) — **exe 0x2380820~+0x4000 전 구간에 imm 0x2d·0x26 cmp가 단 하나도 없음**(모든 레지스터/메모리 인코딩 확인).
0.5.1 대비 tr9→ally#1 간격 0x60→0x56, ally#1→ally#2 0x4c→0x2e로 **줄어든 폭이 제거된 cmp+jcc 크기와 정합** ⇒ 0.5.2에서 **phase 게이트(≥39)와 retreat_hp#1 비교가 실제로 제거/재구조화**된 로직 변경. pt(0x1d) 후보는 0x2380c28/0x2380c40(RCX→**RDX**)이나 2개 중 어느 쪽인지 미확정.
⇒ **판정: d19i 그룹은 계속 전량 보류**(부분 반영 금지 규칙). 상단 10건은 소멸 5건 처리 방침이 정해진 뒤 일괄 반영할 것.

### 2) GENERIC_BUILD = **0x22b2280 (확정)**
프롤로그 push8(12B, rip-rel無) + `SUB RSP,0x558`(0.5.1 0x5a8) + `lea rbp,[rsp+0x80]` → **orig_len=12 경계OK·트램폴린 안전**.
확증 = 내부 상수 3종이 0.5.1과 **거의 동일 오프셋**에 존재: close_radius imm +0x2da(0.5.1 +0x2db), line_range, join_dist.

| gb 사이트 | 0.5.1 | **0.5.2** | prefix | imm@ | w |
|---|---|---|---|---|---|
| close_radius | 0x1e1ee86 | **0x22b2555** | `48 c7 44 24 40` (불변) | +5 | 4 |
| line_range | 0x1e1f5f1 | **0x22b2ca5** | ★`48 c7 85 **b0** 01 00 00` (disp 0x180→**0x1b0**) | +7 | 4 |
| join_dist | 0x1e1f4c7 | **0x22b2baf** | ★`01 c1 41 b8` (was `49 01 c9 b9`; ECX→R8D) | +4 | 4 |
| reach_margin | 0x1e21248 | **0x22b43ae** | `41 b8` (불변, GB 내 유일) | +2 | 4 |
| reach_cap#1 | 0x1e30c57 | 0x23ad9d7(자동매칭) | `48 b8` | +2 | 8 |
| reach_cap#2 | 0x1e39183 | 0x23ba8f3(자동매칭) | `49 ba` | +2 | 8 |

⛔**미확정 6**: scout_radius#1/#2(0.5.1 컨텍스트 `48 c7 45 10 …49 b9` / `4c 8b 75 70 49 b9` **소멸**, 일반 `49 b9`+imm64는 16곳), op_phase(`48 83 bb b8 00 00 00 1f` **0 hit** = disp/reg 변경), join_phase ×2(GB 내부에 imm 0xc cmp가 **전무**), push_hp.
⇒ **gb 그룹도 전량 보류 유지.** 단 **GENERIC_BUILD 함수주소 자체는 확정**(F2_BUILD_CALL/body detour 재활성 시 사용 가능).

### 3) DISC18_HANDLER = **0x2376320 (확정)** — 종전 "갭 부족 보류" 해소
결정적 근거: `an_cull_dist`(cmp ?,0x5f5e0)가 **exe 전체에 단 2곳**뿐이며 그 둘이 각각 **disc18 후보 내부(0x2376e86, +0xb66)**·**disc19 내부(0x2381df5)**.
0.5.1도 동일 구도(disc18 +0xbd9 만 패치) → 후보 확정.
프롤로그 push8(12B, rip-rel無) + `SUB RSP,0x5f8` → **orig_len=12 경계OK·install_wrap 안전**.
- `oi_an_cull_dist`: 0.5.1 0x1c7d5f9 → **0x2376e86**, prefix ★`49 81 fa`(was `49 81 f8`; **R8→R10**), imm@+3, w4.
- `oi_an_finish_hp`: 0.5.1 0x1c7df47 → 후보 **0x23777fe(+0x14de)** 또는 **0x237780a(+0x14ea)**, prefix `48 83 f8`(불변), imm@+3, w1. ⚠**2곳 존재(64bit/32bit div 쌍 추정) = 어느 쪽이 정본인지 미확정**.

### 4) oi_* 13사이트 → 3 확정 / 10 미확정
- `dn_pred_dist`: 0.5.1 0x21ee085 → **0x1bdac25** — imm64 sq(240000) 패턴이 **exe 전역 유일** = 확정. prefix `48 b8` 불변.
- `dn_lane_margin`: 0.5.1 0x21ee0f5 → **0x1bdac95** — pred_dist와의 **간격 0x70이 0.5.1과 완전 일치** = 확정. prefix `49 83 c6` 불변.
- dn_* 컨테이너 ≈ **0x1b92e40**(near_dist 후보 0x1b9302c/0x1b93152, nexus_hp 후보 0x1b934a4/0x1b934b0).
- ⛔`dn_count_gate`(`48 83 7d b8 26`)·`dn_hp_crit`(`48 83 7d 08 15`)·`dn_hp_low`(`48 83 7d 08 1f`) = **exe 전역 0 hit** → rbp 상대 인코딩 전면 변경, 개별 RE 필요. near_dist 쌍 간격도 0x4d→0x126로 불일치.
⇒ **oi 그룹 전량 보류 유지.**

### 5) 데이터 심볼
- ★`DISC7_DMG_SHEET`: 0.5.1 0x3846328 → **0x38d1918 (확정)**. 근거 = 0.5.1을 확정했던 **동일 xref 경로 재현**: disc19 0x2382a82 `lea rcx,[r10+0x478]` … 0x2382a95 `lea r9,[rip]→0x38d1918` … `call 0x2004f10` (0.5.1 `FUN_14230ee30(u+0x478,…,&UNK_143846328)`와 인자 형태 동일). 9개 desc{vt,0x6a8,8,ptr} 중 **disc19가 참조하는 유일 desc**.
- ★`D19_SLOT2_EMPTY` / `D19_STATIC_TEMPLATE`(0.5.1 통합 0x3846d50) → **0x38d1af0 (확정, 높음)**. 근거 = disc19 0x2380abd~0x2380afb에서 `[r15+0x5b0]<3`·`<5` fallback 두 갈래가 모두 이 객체를 `lea`하고 곧바로 `mov eax,[rsi+0x30]` 가드 → `cmp eax,-1`; 실제 0x38d1af0+0x30 = `ff ff ff ff`. 반대편 갈래는 `[r15+0x4e8]`/`+0x520` = 소스 재현식과 **오프셋 전부 bit-동일**. disc18(0x23765d2 등) 포함 10 refs = 0.5.1의 "단일 empty-descriptor 통합"과 정합.
- ⛔`D19_STATIC2_TEMPLATE`(0.5.1 0x38d17b8): 2차 emitter 재식별 실패 → **미확정**.
- ⛔`C8C_DMG_SHEET`(0.5.1 0x3830c58): desc 총 9개(OLD 11) 중 **DISC7(0x38d1918) 외 8개를 변별할 내재적 근거 없음**(vt 함수 동일성 대조에 0.5.1 exe 필요) → **미확정**. d19thr 게이트 기본 OFF라 실사용 없음.

### 미확정분 안전성
위 미확정/보류 상수는 전부 ①`patch_imm_bytes` prefix 검증 실패 → 조용히 skip, 또는 ②`install_wrap`/target-guard 신원검증 실패 → 미설치 = **inert(무크래시)**. 즉 현 상태 유지가 fail-safe.

---

## §7.2-A2 · sim vtable 6사본 / lane_gate / fc59a0 — 신구 exe 대조 확정 (2026-07-22, ghidra-re 2차)

> ★**본 세션은 0.5.1·0.5.0_3·0.4.13_5 exe를 capstone/pefile로 직접 정적 파싱해 신구 대조**로 판정. §7.2-A의 "exe 내재적 근거만" 제약은 이 절에는 해당 없음.

### ★★A. sim vt0x30 "6사본" = Rust trait-object vtable — **정체 규명 + 0.5.2 확정**

**정체(오해 정정)**: `disc4_vt30_kind()`가 비교하는 6개 값은 "모노모픽 함수 6사본"이 아니라
**Rust trait-object vtable 6개**다(.rdata). 레이아웃 = `+0x00 drop_in_place / +0x08 size / +0x10 align(8) / +0x18~ 메서드`.
**3 concrete type × 2 trait = 6 vtable**. 슬롯 `+0x30`이 kind getter이고 그 본체는 3개 버전 모두 **완전히 동일**:
```
lea rdx, [rcx + 0xeaf0]    ; ← 소스 주석 `let comp = gchild + 0xeaf0` 와 정확히 일치
mov eax, <0|1|2>           ; ← 이게 곧 kind (k0는 xor eax,eax)
ret
```
⇒ **kind 매핑은 추측이 아니라 exe 안에 상수로 박혀 있다. 뒤바뀔 여지 0.**

**★★소스 상수는 0.5.1이 아니라 `0.5.0_3` 값이었다** — 즉 **0.5.1 마이그 때 이미 누락**돼
0.5.1 시절부터 movepri 대체가 죽어 있었다(0.5.2에서 처음 죽은 게 아님).
17개 exe 전수 스캔에서 6개 상수가 전부 유효한 버전 = **0.5.0_3 단 하나**.

**전수 스캔 결과(.rdata 전역, kind별 정확히 2사본 — 누락·초과 없음)**:

| kind | 0.5.0_3 (=현 소스, stale) | 0.5.1 (미반영이었음) | **0.5.2 (신규 확정)** | 타입 size |
|---|---|---|---|---|
| **0** ★**= 실경기(2중 실측 07-22)** | 0x37d9ee0 / 0x386b080 | 0x38942f8 / **0x38a66d8** | **0x383cd68 / 0x38c5d78** | 0xed90→**0xee88** |
| **1** ~~(stage1=튜토리얼/축소)~~ ⚠라벨 반증 | 0x37da190 / 0x386ae10 | 0x3894610 / 0x38a6400 | **0x383d080 / 0x38c5aa0** | 0xeb08 |
| **2** ~~(stage2=정규+백그라운드 풀매치)~~ ⚠**라벨 반증 = 비전투** | 0x37da400 / 0x386aba0 | 0x38948e8 / 0x38a6128 | **0x383d358 / 0x38c57c8** | 0xeb08 |

### ★★A2. [2026-07-22 추가확정] **실경기 sim = kind0** ⟹ `kind==2` 게이트는 조건 자체가 오류
- **2중 독립 실측**: ①0.5.1 comp_test oracle 캡처 `engine_vt = 0x38a66d8` = **kind0**, 타입 size **0xee88**(kind1/2는 0xeb08이라 배타적으로 kind0 확정) ②0.5.2 buildid 24310934 movepri `gvt` 런타임 실측 = **0x38c5d78** = kind0 (런타임 바이트 도출과 위 상수표가 양쪽 일치, 진단로그 `probe=Some(0) kind=0`).
- ⟹ 위 표의 stage 라벨("stage1=튜토리얼", "stage2=정규 풀매치", `ANA\tfm2-0.5.0-migration.md §11.10.1-B` 출처)은 **정적 추정이었고 반증됨**(원문도 "정확한 모드 대응=런타임 확인 잔여"라 자인). Family B를 "백그라운드/별개 sim"이라 본 것도 반증 = **Family B가 live 실경기 엔진**.
- **반대 방향 정적 증거(정합)**: 전투코어 P1(0x2234620)에서 `engine_vt+0x30 == 2`면 **전투해결 스킵(=비전투)** — "kind2=풀매치"와 모순, "실경기=kind0"과 정합.
- ⚠**kind0의 의미 자체는 미규명**, **튜토리얼/데모의 kind는 실측 0건**(kind0 허용 전환의 유일한 안전 미지수 — 인게임 컨텍스트별 kind 히스토그램으로 확인 가능, 신규 RE 불요).
- ★**판정 정정**: "movepri 대체 = 0.5.1부터 사망"의 원인은 ~~상수 stale 단독~~ → **상수 stale + 게이트 조건 오류(`kind==2`)의 2중**. 상수를 0.5.2로 갱신해도 **`kind==2`는 실경기서 절대 참이 안 되는 사실상 false 상수**라 여전히 전량 skip(0.5.2 실측 11만 회 100% skip, 게이트 도입 07-11 이후 발화·DIFF=0 기록 0건).
- ★**전환 시 안전조건(필수)**: kind0 허용으로 바꾸면 "판별 실패(미상)→폴백 0" 때문에 **"미해석인데 대체 ON"으로 위험이 반전**된다 ⟹ 게이트는 반드시 "**런타임 도출로 명시적으로 kind0이 확인된 경우만 허용**(도출 실패=불허)". 현 소스에 런타임 도출(`vt30_kind_cached`, Option 반환)이 이미 있으므로 **`matches!(vt30_kind_cached(gvt), Some(0))`** 형태가 정답.
- ⬜**ghidra-re 필요 3건**: ①팩토리 `FUN_1419c7a30` param_2 결정 콜사이트 9곳 0.5.2 재핀(튜토리얼 kind 정적 루트) ②Family A(serde 템플릿)→`clone_box 0x2239f29`→Family B(live) 경로의 stage 보존 여부 ③전투코어 P1 `vt+0x30==2` 분기와 §11.10.1-B 모순 해소.
- 구조 정본 = `ANA\discovered-PROGRAM-STRUCTURE.md §15.6/§15.7`.

신뢰도 = **확정**. 근거 4중 교차확증:
1. `+0x30` 대상 함수가 `mov eax,<kind>`로 kind를 **문자 그대로 반환** (3버전 모두 바이트 동일).
2. `.rdata` **전역** 스캔 결과 각 kind당 정확히 2사본 = 6개, 3버전 모두 동일 개수.
3. 두 family가 연속 배치되며 **family A는 오름차순 k0,k1,k2 / family B는 내림차순 k2,k1,k0** — 3버전 모두 동일 토폴로지.
4. `lea rdx,[rcx+0xeaf0]`가 소스의 `comp = gchild + 0xeaf0`와 일치 → gvt가 이 vtable이 맞음을 독립 확증.

> ⚠**다른 모드 파급 함정**: 이 6상수는 **버전마다 전부 바뀐다**(패치마다 100% stale). 게다가 stale 시
> `_ => 0` 폴백이라 **조용히 kind0으로 오판**하고 게이트가 전량 skip → 기능이 죽어도 크래시가 안 나
> 로그를 안 보면 모른다. 실제로 **0.5.1·0.5.2 두 버전 연속으로 발각 없이 사망**해 있었다
> (0.5.2 인게임 실측: `[mp STAGE-GATE] stage!=2 skip 누적=15,209,000` = 1,520만 회 전량 skip).
> ⇒ 같은 vtable 상수를 쓰는 코드는 **매 마이그 필수 점검 대상**. 재발 방지책 = 상수 비교 대신
> **런타임에 `*(gvt+0x30)` 함수의 `mov eax,imm` 바이트를 읽어 kind를 뽑는 self-migrating 판별**로 바꾸면 영구 해결.

### B. lane_gate — **미확정(재핀 불가, 신규 RE 필요)**
`LANE_GATE_RVA=0x20d9bf9` / ORIG `0f 86 41 ff ff ff`(JBE 0x20d9b40).
17개 exe 전수 대조 → 이 바이트가 맞는 버전은 **0.4.13_5** 단 하나. 즉 **0.4.14 이후 계속 stale**(0.5.2에서 처음 깨진 게 아님).
0.5.2 실측 `cur=[89,85,08,02,00,00]` = `mov [rbp+0x208], eax` (무관 코드).
**재핀 실패 사유** = 원본 루프의 변별 idiom이 **0.5.0_3부터 전부 소멸**:
`mov eax,0x7e64; bt rax,r12`(비트마스크 게이트) / `sub r12,2; cmovb; cmp r12,0x11` / `cmp rdi,5` 루프.
특히 `mov eax,imm32; bt rax,r` 전수 스캔에서 **다른 마스크(0x4011·0x3d·0x1c·0x1a·0x6f·0xf·0x30·0x3e60·0x80000)는 0.4.13_5→0.5.2 1:1 생존하는데 0x7e64만 사라짐**
⇒ 인코딩 변화가 아니라 **해당 라인후보 루프 자체가 재작성됨**. 시그니처 마이그로는 불가, 신규 RE 필요.
🟢**실사용 영향 없음**: `lane_gate` cfg 기본 0(원본) + prefix 검증 실패 시 조용히 skip = inert.

### C. fc59a0(recall) — **RVA 0x1bdb3e0 = ✅정타 확정**, 훅 미발화 아님
- **①오매칭 여부 = 정타.** 0.5.1 `0x1e2c980`(size 0x6d7) ↔ 0.5.2 `0x1bdb3e0`(size 0x603).
  크기가 0xd4 줄어 한때 "로직 변경?"으로 보였으나 실제는 **블록 아웃라이닝**:
  0.5.2에서 사라진 `mul`/`neg`/`bsr`/`not` 및 상수 `0x3e8`·`0x6400000000`·`0xcccccccccccccccd`가
  **신규 아웃라인 함수 `0x23a4870`(size 0xcc)**에 그대로 있고, fc59a0 본체가 이를 `call`한다.
  본체+아웃라인 합산 시 니모닉 멀티셋이 0.5.1과 거의 완전 일치(잔차 = push5/pop5/call1/ret1/ud2 1 = 아웃라이닝 오버헤드 정확히 그만큼).
  시맨틱 마커도 1:1 보존: `+0x658`×2 · `+0x610`×2 · `+0x218`×1 · `setge`×1 · 간접호출 `call [r+0x1a0]`×1.
  프롤로그 push8 = 정확히 **12바이트**(rbp,r15,r14,r13,r12,rsi,rdi,rbx = 1+2+2+2+2+1+1+1) → `orig_len=12` 경계 안전(신구 동일).
  ⇒ score 계산식은 **상수 수준까지 불변** → `my_fc59a0_full` 재현 로직 그대로 유효.
- **②raw=0 = 정상.** `tfm2_ai_adjust.rs:4402` 가 `if RECALLCAP.load() { FC59_RAW.fetch_add(1) }` — **카운터가 recallcap 게이트 안쪽**.
  `recallcap=0`이면 훅이 정상 진입해도 raw는 영원히 0. **훅 미설치/미도달 증거가 아니다.**
  훅 발화를 실측하려면 `recallcap=1`로 켜고 재측정할 것.

## §7.2-A3 · ★movepri 대체 재개통 + **AV 크래시 메커니즘 완전 규명** + disc별 write-set 감사 (2026-07-22, 0.5.2 buildid 24310934)

> 이 절이 movepri 대체 안전성의 **정본**. disc 추가 배선 전 반드시 여기부터 읽을 것.

### A. 크래시 메커니즘 = "out 부분 write → 스택 잔재가 영속 객체에 커밋 → 무경계 점프테이블 OOB 쓰기" (**덤프 실측 확증**)
**증상**: mp_repl ON 직후 AV(07-22 18:59). WER `0xc0000005`, Fault offset RVA **`0x238e5f2`**, faulting module = 게임 exe 자신. 덤프 `TeamfightManager2.exe.54644.dmp` 파싱으로 확증.

**연쇄(확증)**:
1. movepri의 **out 버퍼 = 콜러 스택 슬롯** `[rbp+0x360]`, **크기 0x30, 제로화 안 됨**. 콜러 = `FUN_1420d6e50` @`0x20d7846`.
2. 반환 후 **병합기 `FUN_141daf160`(RVA `0x1daf160`)** 이 out **0x30을 통째로** 영속 **MovePriority 객체(`agent+0x6b0`)** 에 복사(tag별 fast-path 7개를 제외한 전 경로).
3. ⟹ **모드가 out의 일부 필드만 쓰면 나머지는 직전 호출의 스택 잔재가 그대로 영속 객체에 커밋된다.**
4. 실행기 **`FUN_141dac1f0`(`0x1dac1f0`)**: `idx = tag>=2 ? tag-2 : 7` ⟹ **tag 0·1·9가 전부 idx7** → `0x1dad95a` → `call FUN_142388fd0`(`0x2388fd0`).
5. `FUN_142388fd0`: ① `qword[MP+0x10]` → 테이블 `0x38d72b4`(**유효 8엔트리**) ② `byte[MP+0x2c]` → 테이블 `0x38d72d4`(**유효 41엔트리**). **둘 다 상한 체크 없음** ⇒ 41 초과 값이면 인접 테이블 침범 → **명령 중간에 착지** → `add [rax],al` 로 읽기전용 `.text`에 **쓰기** → AV(`ExceptionInformation[0]=1`=write).

⚠**오독 방지 2건**:
- **폴트 주소 `0x238e5f2`는 명령 경계가 아니다**(`0x238e5ef`부터 재디코딩해야 맞음). `FUN_14238e570`은 그 바이트를 소유한 **무관한 함수 = 범인 아님**.
- 크래시 구조체 정체 = **subplan이 아니라 MovePriority 객체**(1차 조사의 "subplan+0x2c" 서술은 2차에서 정정됨).

**게임 원본이 안전한 이유** = 원본도 out의 일부만 쓰지만, **해당 variant가 실제로 소비하는 필드는 전부** 쓰기 때문.
⟹ ★★**안전 기준 = "재현이 원본의 out write-set을 정확히 일치시키는가" 하나뿐. 컨텍스트/kind는 무관.**

**07-11 itemnet 크래시와 다른 신규 경로(확증)**: 07-11 = `FUN_141b78420` fn+0x81 `MOV RAX,[RBX+0x10]` **읽기** AV / 이번 = 점프테이블 오착지 **쓰기** AV.

### B. 0.5.2 movepri 디스패처 실측
- **`RVA_MOVEPRI = 0x2134240` 자체가 디스패처**(별도 함수 아님). 점프테이블 **`0x38ae274`**, `idx = disc>=2 ? disc-2 : 1`, **유효 16엔트리(disc 2~17)·상한 체크 없음** ⟹ **disc≥18이면 movepri도 같은 OOB 크래시 클래스**.
- 에필로그 `0x21345b0`. 호출규약 **rcx=out(sret) / rdx=subplan** (모드의 `p1=[saved+0x28]`=out, `p2=[saved+0x20]`=subplan 매핑 정확). **out 크기 = 0x30**.
- disc→핸들러(0.5.2): 2·8=**0x2134298 공유**(인라인) / 3 `0x21344ee` / 4 `0x2134446` / 5 `0x213448b` / 6 `0x2134393` / 7 `0x213450f` / 9 `0x21344ce` / 10 `0x2134578` / 11 `0x21343fa` / 12 `0x2134551`→헬퍼 **`0x238f130`** / 13 `0x213436e` / 14 `0x21343d0`→헬퍼 **`0x2118ef0`** / 15 `0x213452f` / 16 **인라인 `0x21342a4`** / 17 `0x213446b`→헬퍼 **`0x1b92e40`**.

### C. disc별 write-set 감사 (ghidra-re 2회차, 실 disasm 대조)
| disc | 판정 | 근거 |
|---|---|---|
| **2·8** | ✅**안전(확증)** | 원본 = `mov qword[rsi],7` + `jmp 에필로그` **2명령**·분기/call/RNG 0 · **3버전(0.5.0_3/0.5.1/0.5.2) 바이트 완전동일** / 모드 `wr_u64(p1,7)` = **비트동일** |
| **16** | ✅**안전(확증)** | 인라인 `0x21342a4`. code7·0x12 = out+0만 / code2 = +8=0·+9=byte[subplan+0x10]·+0xa=2·+0=2. 모드 완전일치. 0.5.0_3 블록과 명령단위 동일(차이=vt슬롯 0x138→0x1a0인데 모드는 `dd7_slot128` 순수재현이라 무영향) |
| **17** | ✅**안전(확증)** | `0x1b92e40`, 원본도 `mov [rdi],rcx` **단일 write**(값 7 또는 0x13)·payload write 0건. 모드 out+0만 = 일치. 임계상수 3버전 동일 |
| **14** | ⛔**위험 — write-set 불일치** ~~1차 크래시 최유력 원인(확증)~~ → **정정(07-22, §7.2-A5)**: 1차의 조건은 "**화이트리스트 없이 전 disc를 켠 것**"이고, 2·3차의 진범은 **미마이그 C8C 상수를 shadow-call this로 넘긴 것**(write-set 무관). ★**런타임 실측 확보**: 관측 code **7·0x11은 원본도 out+0만** 씀=명세·모드 구현 일치, ⬜미관측 code 0x14/2/0xf/0x10은 재캡처 필요. 재편입 조건 = **§7.2-A5 §6** | 원본은 code별로 **+8/+9/+0xa/+0x10/+0x11**을 쓴다(code 0x14: +8=0·+0x10=1·+0x11=byte[payload+0x18] / code 0xf·0x10: +8=0 / code 2: +8=0·+9=al·+0xa=2). 모드는 **out+0만** 기록(aux를 로컬 scratch에 쓰고 버림). ★소스 주석 "code-only…aux 미터치(게임도 동일)" = **오기이며 0.5.0_3에서도 이미 틀렸음** |
| **12** | ⛔**위험(확증)** | out write-set은 일치하나 **payload write 전량 누락**: `payload+0x1a`(bool)·`payload+0x10`(len)·`movups payload+0..0xf`={cap,ptr} = **Vec{cap,ptr,len}** + 기존 Vec `__rust_dealloc`. ⟹ 다음 틱 FLEE 판정이 **stale 힙 포인터**로 진행(해제된 엔티티 순회 위험) |
| 10·13 | ⬜**미감사** | disc13은 emit `{7,0x11,2}` vs 재현 주석 `{7,0xb,0xd}` **불일치 의심** |

### D. 0.5.2 로직 변경 신규 발견 (확증)
- **disc12·disc14의 `param3(tick/level) < 0x21 → code 3` 분기가 0.5.2에서 삭제**됨(0.5.0_3·0.5.1엔 존재). ⟹ 0.5.2 출력집합 disc12=`{0x14,0xc,7,0xd,0xe,2}` / disc14=`{0x14,0xf,7,0x10,0x11,2}` — **양쪽 다 code 3 없음**. 모드는 여전히 code3 출력 = **초과**.
- **disc5·disc6 로직 변경**: `param_3(r8) >= 0xb` 게이트가 **양쪽 모두 제거**됨. ⟹ 모드 `mp_write_disc5`/`mp_write_disc6`(`tfm2_ai_adjust.rs` L5671·L5686)는 **0.5.2 기준 이미 틀림**(r8 항 잔존). 현재 `MP_D56_REPL=0`이라 inert지만 **켜기 전 반드시 r8 항 제거**.
- 그 외 임계상수(`0x33`·`0x1eb` 마스크·`0x9502f9001`·`0x53d1ac101`·`0x20c49ba5e353f7d` 등) 불변. **vtable 슬롯만 일괄 +0x68**(0x138→0x1a0, 0x150→0x1b8, 0.5.1부터).
- ⟹ ★**disc12·disc14의 "400/400 검증·DIFF=0"(0.5.0_3 시절) 기록은 0.5.2에서 무효**(code3 경로 한정). **disc16·17은 로직·상수 불변이라 여전히 유효.**

### E. 소스 조치 + 배포 (완료)
- **`MP_SAFE_DISC` 화이트리스트 신설**(`tfm2_ai_adjust.rs` L5946 부근): `const MP_SAFE_DISC: [u64;4] = [2,8,16,17];` + `disc_rd.filter(...)`로 비화이트리스트 disc를 `None` 처리 = 대체 블록 전체 skip = **원본 passthrough(비트동일)**. 주석에 위 메커니즘·판정근거·"**disc 추가 시 반드시 0.5.2 원본 disasm 대조 후에만**" 규칙 명시.
- **stage 게이트 폐기·교체**: `mp_stage2_ok`의 `disc4_vt30_kind(gvt)==2` → **`vt30_kind_cached(gvt).is_some()`**(sim 신원검증만, 도출 실패=불허 fail-safe). 폐기 근거 = §12.23.2 원문 "스테이지 게이트는 이 크래시엔 무효" + 실경기/튜토리얼 모두 kind0 + family A/B 병렬 동시실행.
- **cfg**: `mp_repl=1`(재개통) / `d4_repl` 1→**0**(disc4 격리: 07-11 크래시 유발원이고 07-12 3-diff 수정의 **라이브 재검증 부재**).
- 배포 dll ~~3,463,680B md5[:8]=FE3FE270~~ → **3,463,680B md5[:8]=`AADE99B6`**(주석 정정 반영본·동작 동일). **인게임 무크래시 완주 확인**(19:45~19:56 — crash_log·panic_log 미생성, APPCRASH 이벤트 0건, 새 덤프 0건, 유저 "잘 돌아간다").
- ~~⬜미확증: 대체 실제 발화 미확인(log=0)~~ → ✅**발화 확증(log=1 재측정, 07-22, 0.5.2 buildid 24310934)**: `mpcmp.txt` 40,374B·누적 **`[mp REPL #417000]`** = **대체 417,000회 발화**. disc별(500회/줄) **disc17=399줄≈199,500 / disc8=238줄≈119,000 / disc16=198줄≈99,000**(합=누적과 정합). **disc2=0건은 정상**(원래 거의 발화 안 하는 disc — 버그 아님). 관측 code: disc8=7, disc16=2, disc17=0x19. **`[mp GATE] sim 신원검증 실패`=0건** ⟹ 새 게이트 `vt30_kind_cached(gvt).is_some()`가 정상 sim 전량 통과·미상 vtable 조우 0 = **게이트 축 전환 실전 유효 실증**. 크래시 0. ⟹ **07-11 stage 게이트 도입 이래 처음으로 movepri 재현 대체가 실제 게임에 개입한 것이 확증**. 증거=배포폴더 `mpcmp_발화확증_0722.txt`(40,374B).
- **운영**: 검증 후 cfg 임시 `log=1` **제거 완료** — 백업 `tfm2_ai_adjust.cfg.bak_pre_replverify`와 **diff 0**(완전 원복)·BOM無(첫 3B `23 20 70`)·325줄·한글 무결. **소스 주석 3건 정정 완료**(disc14 "aux 미터치(게임도 동일)"=오기 취소선+사유+재활성조건 / disc12 "검증 DIFF=0"에 0.5.2 무효 마킹 / `mp_write_disc5/6`에 0.5.2 r8 게이트 삭제 경고).

### F. ~~⛔별건(중요·미해결) — itemnet 가드가 0.5.2에서 미설치~~ → ✅**해결(2026-07-22): RVA 재핀 완료**
**정정 결론(0.5.2)**: `RVA_ITEMNET_SCORER` = **`0x1b9cce0`**(3버전: 0.5.0_3 `0x1b78420` / 0.5.1 `0x1bc82e0` / 0.5.2 `0x1b9cce0`). 정체=itemnet 팩터드 선형모델 점수함수(모델 ptr `{+8 weights, +0x10 len, +0x18 flag}`), 07-11 §12.23 AV 지점=`fn+0x81 MOV RAX,[RBX+0x10]`.
- ★**재핀 방법(자동매칭 실패 상수의 일반 기법)**: ①모드가 이미 가진 신원검증 **27바이트 시그**로 exe 전역 스캔(프롤로그 push8 12B `55 41 57 41 56 41 55 41 54 56 57 53` + fn+12의 15B `48 81 ec d8 00 00 00` / `48 8d ac 24 80 00 00 00`) → 후보 **216건**(대형함수 공통 프롤로그라 이것만으론 변별 불가) ②**변별자 = 알려진 폴트 오프셋의 명령 `fn+0x81 == 48 8b 43 10`** → **각 버전 정확히 1건**만 생존(0.5.0_3은 기대값과 자기일치=방법 검증) ③확증=0.5.0_3 본문(len 0x61e)과 **96.4% 바이트 일치**(0.5.1 95.8%·잔차=call rel32 변위). ⟹ **니모닉 해시/코사인 없이 유일 변별 가능**.
- **설치 성공 보장**: 시그 매칭 산물이므로 프롤로그·fn+12 15B 바이트 일치 확정·해당 구간 rip-rel 없음 ⟹ 런타임 재확인 불요. 링크 바이트 검증 PASS(신 `0x1b9cce0` 존재 / 구 `0x1b78420` 부재). 배포 dll **3,463,680B md5[:8] `D66F787C`**(직전 `AADE99B6`). ⬜**인게임 미검증**(이 dll로 미실행).
- **역사적 사실 보존**: 0.5.1·0.5.2 구간에 가드가 실제로 미설치였고, 그래서 그 기간의 `차단 누적 = 0`은 **방어 성공의 증거가 아니었다**(그 기간 해석에 여전히 유효).
- ★**차단 카운터 의미 회복**: 이제 `itemnet_guard.txt`의 `차단 누적`이 처음으로 유효 지표(>0 = 실제 AV 차단). movepri 대체가 417,000회 발화 중이므로 **1차 관측 지표로 삼을 것**.

## §7.2-A4 · disc14 재편입 시도 = **2차 크래시로 실패** + 롤백가설 2건 오판 정정 (2026-07-22, 0.5.2 buildid 24310934)

> ⚠★**본 절의 §3 "유력 원인" 추정은 전부 오판으로 확정됨 — 진범 확정 = §7.2-A5**(2026-07-22 후속). §0~§2·§6은 유효. §5 재편입 조건은 A5 기준으로 축소됨.

### 0. ★진단 지식(재사용 최상) — `faultAddr=0xffffffffffffffff`(-1)는 "주소 -1 읽음"이 아니라 **non-canonical 표식**
Windows는 **non-canonical 주소 접근의 #GP를 `STATUS_ACCESS_VIOLATION (0, 0xFFFFFFFFFFFFFFFF)`로 변환**해 보고한다. ⟹ 모드 `crash_log`에서 -1을 보면 **언더플로/NULL을 먼저 의심하지 말고 "레지스터에 쓰레기값이 들어가 그걸 주소로 썼다"**를 먼저 의심할 것. (이번 세션에 이 오해로 엉뚱한 롤백 대상을 골랐다.)

### 1. 크래시 실측(덤프 파싱·확증)
- 폴트 = `FUN_141c8f770`(RVA **0x1c8f770**) 내부 **`0x1c8f785`의 `call qword ptr [r9+0x30]`**. R9 = `exe+0x3830c58` = **Rust 타입명 문자열 블롭 중간**("…ltAction") → `[R9+0x30]`=`0x6e6f69746341746c` = non-canonical 타겟 → #GP.
- ★**exe 전역에 `0x143830c58` 절대포인터 0건·rip-rel 0건 + 문자열 리터럴 시작도 아님** ⟹ 어떤 코드경로도 이 값을 산출할 수 없다 = **순수 쓰레기값**(잘못된 분기로 읽힌 정상값이 아님).
- 정상값 = `exe+0x38c5d78`(sim vt0x30 kind0 vtable·drop@+0/size 0xee88/slot 0x30=`0x1dc88b0`) — 바깥 프레임 스택 슬롯엔 실제로 남아 있었다.
- R9 역추적: `FUN_141acad70`(RVA **0x1acad70**)이 `Box<[(data,typeinfo,extra)]>`(ptr@+0x20·len@+0x28·stride 0x18)를 순회하며 `[typeinfo+0x28]` 호출. **param_4는 루프 밖 RSI 재로드 ⟹ 루프 중 오염 불가 = 진입 시점에 이미 쓰레기**. 배열 원소는 정상.
- 성격 = `0x1acad70`→`0x1c8f770` = **아이템/이펙트 스탯 모디파이어 합산기**("base + pct×stat/100") ⟹ **disc14 자체가 아니라 하류 소비 지점**.
- 미해결: `FUN_141acad70` 리턴주소 슬롯의 `0x000002460038001e`(어느 모듈에도 불속) — 런타임 트램폴린 or 스택 손상, **부분덤프라 확정 불가**.

### 2. ★내 롤백 가설 2건 = **오판으로 확정**(정정)
- ~~⑤ payload 무효화 write(`+0x1a=0`,`+0x10=0`)가 크래시 원인으로 유력~~ → **무죄·원본과 정확히 일치**. 원본 `0x2119464` 재확인: 조건(`len==0` @0x2119385 / 순회소진 @0x21193b8)·대상(`mov rax,[rbp+0x48]`=cmd)·**순서(+0x1a byte 먼저 → +0x10 qword 나중)**·그 뒤 추가정리 없이 **폴스루**(→0x2119474 카운팅블록) = 모드 `serpen.rs` L989-990과 전부 일치. 레이아웃 `{cap@0,ptr@8,len@0x10}`·cap/ptr 미터치도 확증.
- **언더플로 가설 기각 3점**: ①확인된 소비자 `FUN_141acad70`은 진입 즉시 `TEST R12,R12; JZ`로 **len==0 가드** 보유 ②**원본 스스로 len=0을 만들고 폴스루** = 게임이 정상 도달하는 상태 ③언더플로 산술은 힙(`0x246…`) 주소만 낳지 `.rdata 0x143830c58`을 만들 수 없다.
- ~~⑥ out 0x30 제로화가 원인~~ → **직접 원인 아님**(0-write로 문자열 포인터 생성 불가). 병합기 `FUN_141daf160` default arm=6qword(0x30) 복사, 특수 arm=`+8/+0x10/+0x11/+0x12(4B)/+0x14(4B)/+0x16(2B)` 선택복사 — 커밋 범위는 0x30 이내.
- ⟹ ★**"⑤·⑥만 되돌리면 안전"은 근거 없음**(그대로 재시도했으면 같은 크래시 재발). **추정으로 롤백 대상을 정한 것 자체가 오류.**

### 3. ~~유력 원인·미검증 잔여~~ → **전건 오판으로 확정 (정정 2026-07-22, 진범=§7.2-A5)**
- ~~**①(재현 결과를 실제 out=p1에 직접 커밋)이 성격상 가장 유력**~~ → ⛔**오판·무죄 확정**. 3차 크래시(23:16)는 disc14가 **화이트리스트 밖=대체를 전혀 하지 않은 상태**에서 mpcap 캡처만으로 재현 함수가 실행됐는데도 **2차와 완전히 동일한 크래시**가 났다 ⟹ 원인은 "대체(커밋)"가 아니라 **재현 함수 실행 자체**.
- ~~★**모드 helper의 내부 필드 오프셋이 0.5.2 미검증**(유력 후보)~~ → ⛔**오판·실측 기각**. 3버전 vtable 슬롯 함수 바이트 대조(0.5.0_3 kind0 `0x386b080` ↔ 0.5.2 kind0 `0x38c5d78`) 결과 **함수 본체·필드 오프셋 전부 3버전 불변**: vt0x20 seed=`gc+0xeab8` / vt0x28 tick=`gc+0xeac0` / vt0x30 kind=`gc+0xeaf0` / vt0x50→**0xb8** anchor=`gc+0xeb08`(vt0x58/0x68/0x78도 →0xc0/0xd0/0xe0). ⟹ **슬롯 번호만 0.5.1부터 +0x68 이동, 오프셋 상수 4종은 전부 유효**.
- 모드 shadow-call 게이트 헬퍼(dll fn `0x616c0`·화이트리스트 테이블 `MOD+0x32c5f0`)가 콜체인에 있고 **arg4를 로컬 스택슬롯 `[rsp+0x60]`에서 로드** — 초기화 누락 시 정확히 이번 폴트를 재생산. 규명/제거 대상.
- ⚠**모드 ⑤ 코드에 원본에 없는 `vlen <= 16` 가드** 존재 → `vlen>16`이면 원본은 순회해 code7을 낼 수 있는데 모드는 무효화로 감 = **비트동일 위반**(크래시와 별개 결함·수정 필요).

### 4. 조치·현재 상태
- **disc14를 `MP_SAFE_DISC`에서 재제외 → 현행 `[2, 8, 16, 17]`**. 배포 dll **3,464,704B md5[:8]=`8C339418`**·cfg `mp_repl=1` 복구(BOM無·325줄·한글 무결) = 앞서 **417,000회 발화·무크래시** 검증분 조합.
- 로그 보존: 배포폴더 `crash_log_disc14_2차.txt` / 덤프 `%LOCALAPPDATA%\CrashDumps\TeamfightManager2.exe.77968.dmp`.
- ⚠**크래시 빌드 dll 미보존**(19분 뒤 재빌드본으로 교체 — 덤프 모듈 TimeDateStamp `0x6a60adf5` vs 디스크 `0x6a60b296`) ⟹ MOD 프레임 매핑이 근사에 머물렀다. ★**규칙화: 다음 시도 전 크래시 빌드 dll을 `_crash\`에 사본 보관**.

### ~~5. ★disc14 재편입 조건(다음 세션용 — 이 순서 그대로)~~ → ✅**완료 (2026-07-23, 0.5.2)**
> ✅**조건 5단계 전부 충족 + disc14 `MP_SAFE_DISC` 편입 성공**(~~재현 400/400 DIFF=0~~=~~07-23 후반 무효·⬜재검증 대기~~ → ✅**재검증 통과**(0.5.2, 07-23, dll `854B23F3`, passthrough+캡처 400/400 DIFF 0, §7.2-A8), 인게임 무크래시·대체 437회 발화). 진범은 §7.2-A5(미마이그 C8C 상수)였고 C8C RVA도 `0x381e1e0`으로 확정됨. **정정 정본 = `§7.2-A7`**(§7.2-A6 §2도 참조). 아래 5개 항목은 **이력으로만** 보존 — 재수행 대상 아님.
1. **①을 그대로 두고 재편입 금지.** 먼저 ①을 되돌린 상태(out은 원본이 쓰고 모드는 shadow 비교만)에서 **code별(0x14/2/7/0xf/0x10/0x11) out `+0..+0x2f` 0x30 전체 game↔mine 바이트 대조** 통과. 대조 지점 = 병합기 `FUN_141daf160` 진입 직전(param_2).
2. `vlen<=16` 가드 제거(원본에 없음).
3. 모드 helper 내부 필드 오프셋(`0xeb08`/`0xeaf0` 등)의 **0.5.2 유효성 검증** 선행.
4. shadow-call 게이트 헬퍼가 disc14 경로에서 무엇을 호출하는지 규명 → 완전재구현 원칙대로 제거하거나 cfg OFF.
5. **한 항목씩** 재편입(①만 → 검증 → ③ → ④ …). 5개 동시 적용은 원인 분리를 불가능하게 만든다(이번 실패의 직접 원인).

### 6. 교훈(운영 규칙)
- **명세에 없는 "권장/개선"을 재현에 얹지 말 것.** 목표는 원본 write-set 일치이고, 원본이 잔재를 남기면 잔재를 남기는 것이 맞다(⑤⑥은 결과적으로 무죄였지만, 검증 없이 얹은 판단 자체가 오류).
- **"과거 검증된 재현"을 새 버전에서 켤 때는 write-set뿐 아니라 그 재현이 딛고 선 오프셋 체계 전체가 미검증임을 전제할 것.**

## §7.2-A5 · ★disc14 크래시 **진범 확정**(확증) = 미마이그 RVA 상수를 shadow-call의 this로 넘김 (2026-07-22, 0.5.2 buildid 24310934)

### 1. 진범 (크래시 3건 전부 설명)
**`my_c8c520`(`tfm2_ai_adjust\src\serpen.rs` L742)이 `RVA_C8C_DMG_SHEET`를 vt+0x28 다형 shadow-call의 this(r9)로 넘긴 것**:
```rust
probe_basedmg_r9(st, plan, exe, exe + RVA_C8C_DMG_SHEET)
```
- `RVA_C8C_DMG_SHEET`는 **0.5.2 미마이그(보류)** 상태의 0.5.1 값 **`0x3830c58`**. 0.5.2에서 그 주소는 **Rust 타입명 문자열 블롭**("…ltAction")이다.
- ⟹ 문자열을 vtable로 삼아 `call [r9+0x30]` → non-canonical → **#GP** → crash_log `faultAddr=0xffffffffffffffff`, RIP=`exe+0x1c8f785`.
- ★**덤프의 `R9 = exe+0x3830c58`이 정확히 이 모드 상수였다.** §7.2-A4 §1이 "exe 전역 참조 0건 = 순수 쓰레기값"으로 판정한 것은 **맞았으나 함의를 놓쳤다** — exe에 참조가 없는 게 당연했다(게임 코드가 아니라 **모드가 하드코딩한 상수**이므로).
- ⚠**`rva_052.rs`의 C8C 주석 "d19thr 게이트 기본 OFF라 미사용"은 오기** — 이 사이트는 **disc14 재현 경로(`my_c8c520`)에서 게이트 없이** 호출된다. **보류 방치가 곧 AV였다.**

### 2. 크래시 3건 인과 (기존 판정 정정)
| 회차 | 시각 | 원인 |
|---|---|---|
| 1차 | 18:59 | **화이트리스트 도입 전** 빌드(18:49·게이트만 열림) ⇒ **전 disc 대체** → disc12/14 write-set 불일치 → MovePriority 오염 → 점프테이블 OOB(`0x238e5f2`) |
| 2차 | 21:05 | disc14 편입 → **C8C shadow-call**(`0x1c8f785`) |
| 3차 | 23:16 | **대체 안 함(화이트리스트 밖) + mpcap 캡처만**으로도 재현 함수 실행 → **동일 C8C shadow-call** |
- ★**3차가 결정적 증거**: 대체를 하지 않았는데 2차와 완전히 동일한 크래시 ⟹ **①(out 직접 커밋)은 무죄**, 원인은 **재현 함수 실행 자체**.
- 1차 원인 정정: ~~"disc14 write-set 불일치가 1차 크래시 최유력"~~ → **화이트리스트 없이 전 disc를 켠 것**이 1차의 조건. 화이트리스트 도입(19:44) 이후 동일 크래시 **재현 0건**.

### 3. helper 오프셋 = 전부 유효(A4 §3 "유력 후보" 실측 기각)
| 항목 | 슬롯(050→052) | 함수 본체 | 판정 |
|---|---|---|---|
| vt0x20 seed | 0x20→0x20 | `48 8b 81 b8 ea 00 00 c3` = `gc+0xeab8` | 동일·모드 OK |
| vt0x28 tick | 0x28→0x28 | `48 8b 81 c0 ea 00 00 c3` = `gc+0xeac0` | 동일·모드 OK |
| vt0x30 kind | 0x30→0x30 | `48 8d 91 f0 ea 00 00` = `gc+0xeaf0` | 동일·모드 OK |
| vt0x50 anchor | 0x50→**0xb8** | `48 8d 81 08 eb 00 00 c3` = `gc+0xeb08` | 동일·모드 OK |
| vt0x58/0x68/0x78 | →0xc0/0xd0/0xe0 | 첫 12B 동일 | 동일 |
⟹ **슬롯 번호만 +0x68 이동(0.5.1부터), 함수 본체·필드 오프셋은 3버전 불변. 모드 상수 4종 전부 유효.**

### 4. disc14 원본 write-set **런타임 실측**(정적 명세 교차검증)
3차 크래시 직전까지 캡처 성공 → 배포폴더 `mpws.txt`·`mpout.txt`(23:16):
```
[disc=14] write-set=0b00000001 (오프셋: +0x0)
[disc=14 code=17] +8=0x0 +0x10=0x8 +0x18=0x0 +0x20=0x0 +0x28=0x78b30c400 | b+0x12=0 b+0x21=0
[disc=14 code=7]  (동일)
```
- 관측 code = **7·0x11(17)** 2종, 둘 다 원본이 **out+0만** 씀 ⟹ **Ghidra 정적 명세와 런타임 실측 일치**(명세도 code 7/0x11을 "+0만"으로 서술).
- ⟹ 종전 모드 구현(`wr_u64(p1, code)` = out+0만)은 **code 7/0x11에 한해 정확했다**.
- ⬜**미관측 = code 0x14·2·0xf·0x10**(aux 기록 명세분) — 크래시 조기종료로 표본 부족, **재캡처 필요**.

### 5. 조치·현재 상태
- **`C8C_SHEET_MIGRATED: bool = false`** 상수로 해당 shadow-call **봉인**(`serpen.rs`). 컴파일러 DCE로 **dll에서 `0x3830c58` 링크 완전 소멸**(바이트 검증 확인). 영향: `b0=b1=0` → 그 항목 dmg=0 = **재현 정확도 저하**(크래시는 원천 차단). **해제 조건 = C8C RVA 0.5.2 확정**.
- 배포 dll **3,464,704B md5[:8]=`29599554`** · `MP_SAFE_DISC=[2,8,16,17]`(disc14 제외 유지) · cfg `mp_repl=1`·`mpcap=0`·log 없음(BOM無·325줄).
- ✅**인게임 무크래시 확인**(23:19 배포 후 실행: crash_log 미생성·APPCRASH 0건·새 덤프 0건, 유저 "잘되는 거 같다") ⟹ **진범 판정의 최종 확인**.
- 로그 보존: `crash_log_disc14_2차.txt`·`crash_log_disc14_3차.txt`·덤프 `TeamfightManager2.exe.77968.dmp`.

### 6. ★잔여
- ~~⬜`RVA_C8C_DMG_SHEET` 0.5.2 확정 필요·유력 후보 `0x38832a8`~~ → ✅**확정 = `0x381e1e0`**(2026-07-23, 인게임 검증완). **`0x38832a8`은 기각**(그건 `serpen_dmg_core` 인자인 **cand 쪽** vtable, `probe_basedmg_r9`가 쓰는 **st 쪽**이 정답 — 같은 함수 내 두 `lea r9` 혼동). 상세=**§7.2-A6**.
- ⬜★**리터럴 하드코딩 상수가 마이그 체계 밖에 존재**: `0x35e4d00`(ATK_VT)·`0x3599b30`(ability_table) — `rva_052.rs`에 없어 마이그 대상에서 **누락**됐고 shadow-call/vt_call에 쓰인다(`tfm2_ai_adjust.rs:1463·5260·5482`). **같은 부류의 잠재 크래시원 ⇒ 전수 점검 필요.**
- ~~⬜disc14 재캡처 → 대조 통과 시 대체 편입 검토~~ → ✅**편입 성공(2026-07-23)**: ~~code 재현 400/400 DIFF=0~~(→~~07-23 후반 공유 콜리 3종 수정으로 무효·⬜재검증 대기~~ → ✅**재검증 통과**(0.5.2, 07-23, dll `854B23F3`, passthrough+캡처 400/400 DIFF 0, §7.2-A8). 편입 유지=활성) + write-set 실측 정합 + 인게임 무크래시(대체 437회 발화). `MP_SAFE_DISC=[2,8,14,16,17]`. 상세=**§7.2-A6**.

### 7. ★교훈(운영 규칙화)
- ★**"보류/미마이그 RVA"가 shadow-call·vt_call의 인자로 쓰이면 그 자체가 크래시원이다.** 보류는 "미사용이라 안전"이 아니다 — **사용처를 실제로 grep해 확인**해야 하고, 확인 전에는 그 상수를 쓰는 경로를 게이트로 봉인할 것.
- ★**크래시 원인을 추정으로 지목하지 말 것.** 오늘 disc14 건에서만 롤백 가설 3건(⑤·⑥·①)과 오프셋 가설 1건이 **전부 오판**이었고, 덤프의 `R9`를 모드 상수와 대조한 뒤에야 진범이 나왔다. ⟹ **크래시 로그의 레지스터 값을 모드 하드코딩 상수 목록과 대조하는 것을 1순위 절차로.**

---

## §7.2-A6 · ★C8C 시트 RVA **확정+인게임 검증완** + **disc14 `MP_SAFE_DISC` 편입 성공** + disc별 재현 정확도 실측 (2026-07-23, 0.5.2 buildid 24310934)

> ⚠ **STALE** — 이 절의 §3(disc 정확도 판정)·§2의 "현행" 값·§5의 dll 해시는 **§7.2-A7이 대체**(오진 정정 07-23). 경위 이력으로만 읽을 것.

### 1. `RVA_C8C_DMG_SHEET` = **`0x381e1e0` 확정 (0.5.2)** — 인게임 검증완
- 3버전 값: **0.5.0_3 `0x380d138` / 0.5.1 `0x38d12d8` / 0.5.2 `0x381e1e0`**.
  ⚠**소스에 있던 0.5.1 값 `0x3830c58`은 원래부터 오답**이었다(같은 CGU 중복본이라 증상만 없었음) — §7.2-A5의 크래시는 "0.5.1 값 미마이그"가 아니라 **애초에 틀린 값의 미마이그**였던 셈.
- ★**재핀법(방법론 검증됨)**: c8c520 원본 함수를 **상수집합 지문**으로 특정 — imm64 `0x53d1ac100` ∧ `0x13880` ∧ `0x1c2` ∧ `0x27100` ∧ `0xfa00` + `call [reg+0x28]` 2회 ⇒ **각 버전 유일 1개**. 그 함수의 **2번째 `call [+0x28]` 직전 `lea r9`** 타깃이 답. **0.5.0_3에 적용해 알려진 정답을 재생산**함으로써 방법 자체를 검증했다. 0.5.2 원본 함수 = `FUN_141bd73a0`(RVA `0x1bd73a0`).
- ⛔**1차 후보 `0x38832a8` 기각**: 그건 `serpen_dmg_core`의 인자인 **cand 쪽 vtable**이고, `probe_basedmg_r9`가 쓰는 것은 **st 쪽**(=`0x381e1e0`). **같은 함수 안의 두 `lea r9`를 혼동**한 것 ⇒ 교훈: rip-rel lea 후보는 **어느 콜의 어느 인자인지**까지 확인할 것.
- ⚠종전 기록의 "자동매칭 `0x381e1e0`=오답"(INDEX §2) 도 **정정** — 자동매칭이 맞았다.
- ✅**인게임 검증**: 봉인 해제(`C8C_SHEET_MIGRATED=true`) 후 캡처 완주 · **크래시 0**(§7.2-A5 3차에서 죽던 그 shadow-call 지점 통과).
- 참고: `{ptr,0x6a8,8}` desc 후보 9개 중 **7개가 메서드 슬롯 포인터까지 동일**(CGU 중복본). 모드가 호출하는 slot `0x30`(`0x141bebd80`)이 전부 같으므로 **오동작 위험은 낮다**(잘못 골라도 조용히 같은 동작).

### 2. ★disc14 = `MP_SAFE_DISC` 편입 성공 (**당시** = `[2, 8, 14, 16, 17]`)
근거 전부 실측:
- ~~**code 재현 400/400 OK · DIFF 0**(mpcmp, C8C 복구 상태).~~ → ~~★정정(0.5.2, 2026-07-23 후반): 이 400/400은 disc12 편입작업(공유 콜리 3종 수정) 前 코드 기준이라 현행 무효 · ⬜재검증 대기.~~ → ✅✅**재검증 통과 = 정당 복원**(0.5.2, 07-23, dll `854B23F3`, `d14_repl=0` passthrough+캡처 **subplan14 OK 400 / DIFF 0**, 크래시0). 콜리 3종 수정(`serpen_engage_gate` tick삭제+UNIT `ctx`→`mapobj` / `serpen_rng_pick` 가용게이트+상한완화 / `serpen_reposition_fight` tick삭제)이 **전부 옳았음이 실측 확인**. **정본 = §7.2-A8** + `ANA\disc12-epiccheck-tail-spec.md` §N-2.
- **원본 write-set 실측이 정적 명세와 정합**(mpws: `[disc=14] 0b1(+0)` / `0b11(+0,+8)` = 명세의 code `0x11`→+0만 / code `0x10`→+0,+8).
- **편입 후 인게임 무크래시 완주** + disc14 대체 **437회 발화**(전체 REPL 최다).
- ★**code 분포 관측**: `0x10`(16) 212 / `0x11`(17) 190 / `7` 27 / `0xf`(15) 4 / `0x14`(20) 3 ⟹ 명세 6종 중 **5종 관측**. 특히 `+0/+8/+0x10/+0x11` 4필드를 쓰는 **가장 복잡한 code `0x14`가 3회 발화하고도 무크래시**. ⬜미관측 = **code 2**뿐.
- **적용 수정 5건**: ①emit이 결과를 버리던 `scratch` → **p1 직결**(+out `0x30` 제로화) ②`sf` `0x3ea`/`0x3eb` → **`0x3f6`/`0x3f7`** ③**code3 분기 삭제**(0.5.2서 사라진 분기) ④다이브추적 **`level>0x2d` 게이트 삭제** ⑤**캐시 무효화 write**(`payload+0x1a=0`·`+0x10=0`, cap/ptr 불변).
- ⚠**out 제로화는 비트동일이 아니다**: 원본은 미기록 필드에 **스택 잔재**를 남긴다. 잔재 재현은 직전 호출 의존이라 **원리적으로 불가**하고, 0은 결정적이며 무경계 LUT(`+0x2c`/`+0x10`)에서 유효 인덱스라 **안전 우위**로 채택. 단 병합기 특수 arm(code `0x14`)이 `+0x12`/`+0x16`을 복사하므로 **그 두 필드는 원본과 갈릴 수 있음 ⇒ ⬜재확인 대상**.

### 3. ★다른 disc 재현 정확도 실측 (신규 사실 — 중요)
같은 캡처에서 disc별 code 일치율:

| disc | OK | DIFF | 판정 |
|---|---|---|---|
| **14** | 400 | 0 | ~~✅100%~~ → ~~⬜현행 무효·재검증 대기~~ → ✅**재검증 통과**(0.5.2, 07-23, dll `854B23F3`, passthrough+캡처 400/400 DIFF 0, §7.2-A8) |
| 7 | 400 | 0 | ✅100%(`D7_REPL=0`이라 미대체) |
| 0 | 364 | 36 | 91% |
| 9 | 2210 | 790 | 74% |
| **1** | 4 | 396 | ⛔**1% = 사실상 전멸** |
| **10** | 0 | 281(+pending 119) | ⛔**0%** |
| **11** | 0 | 3000 | ⛔**0% 완전 붕괴** |
| 12 | 0 | 0 | pending 400(my=-99·미실행) |

⟹ **disc1·10·11 재현이 0.5.2에서 깨져 있다**(전부 화이트리스트 밖이라 **현재 게임 영향 0**). ⛔**절대 켜지 말 것** — 재활성 전 재RE 필수. disc12는 pending(재현이 `-99` 반환) 원인 미규명.

### 4. ⛔운영 사고 — **모드 폴더 안의 dll 사본을 로더가 로드한다** (재발방지 규칙)
- 크래시 빌드 보존용으로 `mods\tfm2_ai_adjust\_crash\tfm2_ai_adjust_crash3.dll`을 만들었더니 **로더가 재귀 스캔해 그 사본을 로드**했고, 그 dll이 `_crash\`를 자기 모드 디렉토리로 인식해 **로그·cfg를 거기에 따로 생성**했다.
- 결과: 이후 여러 판이 **구 dll + 캡처 꺼진 별도 cfg**로 실행 → 상위 폴더 로그가 멈춰 "모드 미로드"로 오진, 무크래시 판정도 구 dll 기준이라 무효(≈45분 허비).
- ★**진단 지표**: `itemnet_guard.txt`는 **LOG_ON 무관·매 post_update 무조건 갱신**(소스 주석 명시) ⇒ **모드 생존 판정의 단일 지표**. 그게 멈추면 모드가 안 도는 것.
- ⟹ ★**규칙**: 백업/사본은 **모드 폴더 밖**(`C:\tfm2mods\<mod>\_crash_<날짜>\`)에. **배포 폴더엔 `.dll` 1개만** — `Get-ChildItem -Recurse -Filter *.dll` **개수 = 1** 확인을 배포 검증 항목에 포함.

### 5. 현재 상태 / 잔여
- **(당시)** 배포 dll **3,464,704B md5[:8]=`52BCE779`** · 모드폴더 dll **1개** 확인.
- cfg: `mp_repl=1`(주석도 disc 2/8/14/16/17 동기화)·`mpcap=0`·log 없음·`d4_repl=0`·BOM無 325줄.
- 증거 보존: 배포폴더 `mpcmp_disc14편입검증.txt`(703,784B)·`mpout_disc14편입검증.txt`·`mpws_disc14편입검증.txt` / 사고 당시 로그 일체 = `C:\tfm2mods\tfm2_ai_adjust\_crash_20260722\from_moddir\`.
- ⬜잔여: ①disc14 **code 2 미관측** + out 제로화의 `+0x12`/`+0x16` 비트동일 미확인 ②**disc12 재현에 code3 분기 잔존**(`serpen.rs:873` `if tick < 0x21 { wr_u64(out,3) }`) = disc14와 같은 결함·화이트리스트 밖이라 현재 무해 ③~~`0x35e4d00`(ATK_VT)·`0x3599b30`(ability_table) 재핀~~ → ✅**해소(0.5.2, 2026-07-23): 둘 다 `0x381e1e0`으로 통합·상수 폐기 = §7.2-A11 E** ④~~리터럴 하드코딩 상수 전수 점검(`rva_052.rs` 밖)~~ → ✅**해소(0.5.2, 2026-07-23): 전수 감사완·라이브 위험 0 = §7.2-A11 F** ⑤disc1·10·11 재현 붕괴 원인 규명.

---

## §7.2-A7 · ★disc 전수 규명·수정·**편입 8종** + 앞 회차 오진 4건 정정 (2026-07-23, 0.5.2 buildid 24310934) — **본 절 = 이 건의 정본**

> ⛔**§7.2-A6 §3의 "disc1 1% / disc9 74% / disc10 0% / disc11 0% ⇒ 재현 붕괴·절대 켜지 말 것"은 오진으로 확정**. 원인은 disc10·11이 **계측 오배선**, disc1·9는 **예상과 다른 원인**(아래 §2). 이 절이 그 자리를 대체한다.

### 1. 편입 완료 — ~~`MP_SAFE_DISC = [2, 8, 9, 10, 11, 14, 16, 17]` (8종)~~ → ★**현행 = 12종 `[0,1,2,3,8,9,10,11,12,14,16,17]`**(disc0/1/3 편입=아래 §7 / disc12 편입=§7.2-A10 §6, 0.5.2·2026-07-23) — **이 값이 MP_SAFE_DISC 정본**, 아래는 8종 편입 시점 기록
- 배포 dll **3,463,680B md5[:8]=`498BEB1E`** · cfg `mp_repl=1`·`poke_repl=1`·`mpcap=0`·log 없음·BOM無 325줄 · 모드폴더 dll **1개**.
- ✅**인게임 검증완**: **크래시 0**(`crash_log`·`panic_log` 미생성) · 대체 **총 5,289회 발화** — disc9 3461 / disc14 641 / disc17 527 / disc8 294 / disc16 264 / disc10 86 / disc11 16 / **disc2 0건(원래 희소 = 정상)**.
- 증거 보존: 배포폴더 `mpcmp_disc9_10_11편입검증.txt` 외 3종 · 직전 회차 `*_disc14편입.txt`.

### 2. disc별 진단 — 초기 판정 정정표

| disc | ~~초기 판정(A6 §3)~~ | 실제 원인(확증) | 조치 | 결과 |
|---|---|---|---|---|
| **11** | ~~0% 완전 붕괴~~ | ★**애초에 정상이었음**. `out+0`이 상수 `0xb`인데 하네스(`detour.rs` `rd_i64(op)`)가 그걸 판단값으로 읽어 `my`(char)와 비교 = **범주 오류**. 오프셋 8종 유효·로직 차집합 0 | **없음(코드 무수정)** | pokecmp 오답 **0** |
| **9** | ~~74%~~ | ①**오프셋 3종 stale**: gateflag/clane/lane = `sub+0x88/0x8c/0x8d` → **`0xBE`/`0xC4`/`0xC5`** ②p3 게이트(`p3>0x31`, `p3<0xb`)가 **0.5.2서 삭제**됨 ③code 지표 오배선(`active` vs `*(subplan+8)`) | 3종 수정 | mpcmp **3000/3000** · pokecmp `[★DIFF@+0x29]` **2814→0** |
| **10** | ~~0%~~ | ★**위험 shadow-call**: `vt_call1(vt,0x138,obj)` = 슬롯 stale(→**`0x1a0`**) + **원본은 2인자 리졸버인데 rdx 미전달** ⟹ C8C와 동일 클래스 잠복 AV(지금까진 `!ptr_ok`에 걸려 조용) | `dd7_slot128` **순수재현**으로 교체 | pending **191→0** · pokecmp OK |
| **1** | ~~1% = 전멸·오프셋 이동 추정~~ | 오프셋(`p2+0x110`/`0x113`)은 **3버전 불변**. 진짜 원인 = **0.5.2서 삭제된 p3 게이트 2개**(`dd_early_p3_thr` 조기분기 · `dd_cover_p3_thr` 커버)를 모드가 계속 걸어 **code 4·6이 전멸** | 게이트 2개 삭제 + `0x112` 비교 `!=0`→**`==1`** + sf `0x3ea`/`0x3eb`→**`0x3f6`/`0x3f7`**(6곳) + vt `0x150`→**`0x1b8`** + STAGE6 선택자 `p4`→**`f`** | mpcmp **400/400**(my=4 225·my=6 52 복구) |

### 3. ★공통 패턴(0.5.2 마이그 일반 규칙 — 다음 패치에도 우선 의심)
- **0.5.2가 `p3`/`level`/`tick` 류 "사전 게이트"를 대거 삭제**했는데 모드가 계속 걸고 있었다 — **disc1·9·12·14 전부 동일 증상**. ⟹ 재현 정확도가 떨어지면 **오프셋 이동보다 게이트 삭제를 먼저 의심**.
- 일부 구조체 오프셋 이동(disc9 3종).
- ★**vtable 슬롯이 0.5.1부터 일괄 `+0x68`**: `0x138→0x1a0` · `0x150→0x1b8` · `0x50→0xb8`.

### 4. ★계측 주의 (정본화 — 재조사 방지)
- ⛔**disc10·disc11의 mpcmp OK/DIFF 수치는 무의미**(`out+0`이 `0xb` 고정 상수라 판단값이 아님). 이 둘의 **정본 지표는 `pokecmp` 바이트 대조**.
- 하네스를 고치려면 `detour.rs`에서 disc10/11만 **`rd_i64(op+8)` 비교**로 바꿀 것 — ⬜**미적용**.

### 5. 위험 제거 2건 (C8C에 이은 3·4번째 stale shadow-call)
- disc10 `vt_call1(vt,0x138,obj)` → **순수재현 교체**(위 표).
- `dd7700` `rd_u64(vtab+0x150)` → **`0x1b8`**. ⚠0.5.2의 `vt+0x150`은 **다른 유효 함수 포인터**라 `ptr_ok`를 통과해 **그대로 호출된다**. disc0/1/3은 화이트리스트 밖이어도 **재현 자체는 매 판단 실행**되므로 위험이 **상시 노출**돼 있었다.
- ⟹ [[tfm2-mod-safety]] §9-B 규칙 강화: "보류/미마이그 RVA가 shadow-call 인자면 그 자체가 크래시원" + **"vtable 슬롯 시프트도 동일 부류"**.

### 6. ⬜disc12 사양 확보 (구현 미착수 — 다음 세션)
- 원본 `0x238f130`. ★**레인 슬롯 산출 규명**: `lane = if u8[gchild+0xeae9]!=0 { read24(sim+0x4a8) } else { read24(gchild+0xb248 + side*0x18) }` — `vt+0xc0`(`0x1dc87d0`, 바이트 게터) · `vt+0xe0`(`0x231eed0`, 24B sret) **둘 다 순수재현 가능**.
- ⛔**모드가 RNG picker의 `tag`를 레인에 재사용한 것은 확정 오독**(tag는 넥서스존 게이트에서 **불리언으로만** 소비).
- ~~테일 [A]~[G] 전 조건표 확보(넥서스존 / sf / 홈박스 / threat / 다이브추적 / 5슬롯 role7 카운트 / engage).~~ → ★**정정(0.5.2 buildid 24310934, 2026-07-23)**: 이 줄은 **라벨 7개만 남고 본문이 소실**된 상태였다. **테일 조건표 전문(의사코드 전체 + 매직넘버표 + 종단별 write-set표 + 인자/심볼맵 + picker draw 규약 + 미전개 목록) = 정본 `ANA\disc12-epiccheck-tail-spec.md`**(근거 = `0x14238f130` 실 disasm). ⛔**다음에 또 요약만 남기지 말 것** — 재도출 비용이 크다.
- ★**07-23 신규 확정(정본 = 위 ANA 파일)**: ①**RNG picker `0x2135350` 호출 지점 = 테일이 아니라 헤드 `@14238f289`, 무조건 1회**(함수 전체 호출 1곳) — `st.flagA!=0 || st.flagB!=0` 조기분기는 **picker 이전**이라 **draw 0** ⟹ ★**RNG-free 안전 슬라이스 = 이 조기분기뿐**(write-set 확정: code 0x14 / 0x0c). 부분 편입 시 **이 분기만 대체 + 나머지 `-99` passthrough**가 유일하게 안전한 1차 착지점. ②종단별 write-set 6종 확정 — **code 7·0xe는 `out+8..` 잔재 보존이 비트동일 조건**(0x14만 `+8`을 qword 클리어). ③재구축 경로 출구 3개(`fb88`/`fb9a`/`fba8`) **전부 code 7·예외 출구 없음** ⟹ 힙 재현 생략 가능, **단 `st.track=1`·`st.len` 갱신은 다음 tick을 바꾸므로 재현 필수**.
- ⚠**완전대체 = 다중 세션 프로젝트**(다음 세션 과제 아님). ~~게이팅 = **대형 미전개 함수 4종** `0x1bd73a0`(다이브 후보 빌더)·`0x2117ae0`(engage 게이트)·`0x23b6800`(레인푸시 viability)·**picker 후보수 `n` 산출 경로**(문자열 memcmp + vtable 슬롯 4종 = C8C형 shadow-call 위험 구간).~~ → ★**정정(07-23, 0.5.2)**: **게이팅 4종 중 2종 해소** — `0x2117ae0`=**전개 완료**(§L) / **picker 후보수 경로**=전개 완료 + **vt 슬롯 4개 전부 leaf라 C8C형 shadow-call 위험 소멸**(§K). **남은 미전개 RE = 2종**(`0x1bd73a0` 다이브 후보 빌더 / `0x23b6800` 로직 델타 — RNG-free·시그니처만 확정). ⟹ 현행 잔여 = **작업 13건 + ⬜미검증 2 + 미전개 RE 2종**. picker가 헤드에서 무조건 호출되므로 대체하려면 **draw 수를 정확히 재현**해야 함(=후보수 `n` 정확 산출).
- ★**disc14와 동형**(5슬롯 열거 · role 테이블 `*(plan+0x20)+0x38b8` · last-seen · `role==7`) ⟹ **disc14 구현 재사용 가능**. 단 **차이 2건**: ①가시성 호출 side 인자가 **두 루프 모두 `side` 고정**(last-seen 테이블만 루프 `sd`) ②최종 비교가 **self** hp% `<=0x14 && allyN<enemyN`(모드는 target hp `<0x15` = **오모델**). → **좌표 확정(07-23, 0.5.2)**: ①`@14238f81b`(루프1)·`@14238f971`(루프2) **둘 다 `MOV RDX,[RBP+0x70]`=side**, lastSeen 베이스만 루프별(`@f760` `1-side`·`@f8ab` `side`) ②`@14238f9cd CMP qword[RBP+8],0x14 ; JA skip` → `@14238f9d8 CMP R14,R15 ; JC → code 7`.
- payload: 무효화(`mem[0x1a]=0`, `mem[0x10]=0`, cap/ptr 불변) / 재구축(`__rust_dealloc`+Vec 교체, **항상 code 7 수렴**).
- ★★**RNG picker `0x2135350`은 조건부로 draw를 소비**(후보 있으면 거부샘플링 루프, 없으면 draw 0) ⟹ 대체 시 **원본 호출 또는 draw 수 동일 재현 필수**(desync 위험).
- ~~⚠**추정(미실증)**: `reposition_fight 0x23b6800` · `engage_gate 0x2117ae0`의 RNG-free 여부 — **켜기 전 확인 권고**.~~ → ★**정정·확정(0.5.2 buildid 24310934, 2026-07-23, 실 disasm 기계적 전수검증)**: **두 함수 모두 RNG-free 확정**. 근거 = ChaCha sigma 테이블 `0x1436e7480`/`0x1436e74c0` 경유 강제(.text 인라인 0) → refill `0x24eaf10`+백엔드 4종 진입점 고정 → `.pdata` 120,995 함수경계 + capstone 전량 디코드 콜그래프 → **RNG 역도달 854함수** vs 전방폐포(reposition 71 / engage 74) **교집합 ∅**, 간접호출 구멍은 `.rdata` vtable 패밀리 6종(`0x14383cd68`·`0x14383d080`·`0x14383d358`·`0x1438c57c8`·`0x1438c5aa0`·`0x1438c5d78`) 슬롯 타겟 14개 전수검사로 메움(전원 non-tainted). ★★**단 "RNG-free ≠ 투입 안전"** — **호출자는 draw를 소비**(`0x142118ef0`/`0x14238f130`→`0x2135350`→`0x24eaf10`, `0x14233e9d0`→직접; 조상 7개 전부 tainted) ⟹ **반환값 비트동일이어야만 안전**(다르면 콜러 분기 변경 → draw 수·순서 desync). `0x1bd73a0`은 **여전히 미검증**. 전문 = `ANA\disc12-epiccheck-tail-spec.md §J`.
- ★**07-23 추가: 콜리 2종 0.5.2 델타 + 모드 테일 직접대조 완료** — **picker `0x2135350`**(§K): vt 슬롯 4개 **전부 leaf ⟹ shadow-call 불필요**(C8C형 위험 해소)·draw 수는 후보수 `n`만이 결정·⛔결함 2건(**(c) `vt[0x50]` 가용게이트 누락**(실패 시 `break`)·**임의 상한가드 4개**) / **engage_gate `0x2117ae0`**(§L): 전체 분기 트리 G0~G10+⑤ 확정·⛔결함 5건(**tick 사전게이트 삭제**·**UNIT 출처 `ctx`→`mapobj`**·② 슬라이스 base `geom` 직접·REQUIRED 꼬리·스캐너 위치) / **테일 대조**(§M): 일치 = [A] 넥서스존 전부 + [B] 오더게이트 + **write-set 6종 전부**(07-22 안전기준 통과), ⛔**불일치 7건**(lane 산출 tag 재사용·code3 잔존·[C]홈코너 없음·[D]threat 없음·[E]다이브 없음·[F] HP 소스 target→self·`serpen_role7_count`가 role7 필터 **전무**). ⟹ **편입 작업목록 = 13건 + 미검증 2 + 미전개 RE 2종**(`0x1bd73a0`·`0x23b6800` 로직델타) = 정본 `ANA\disc12-epiccheck-tail-spec.md §J~§N`.
- 현재 arm이 `12 => return -99` 하드코딩이라 재현 미실행(주석이 가리키는 `SERPEN_VERIFY` 검증 브랜치는 **소스에 존재하지 않음**).

### 7. ~~⬜disc0/1/3 emit 명세 확보 (구현 미착수 — 실익 최대)~~ → ✅**편입·배포완 (정정 2026-07-23, 0.5.2)** · ⬜인게임 미검증
> ★**정정 1 — "emit 미구현" 판정은 오진이었다.** emit 함수 `my_dd7700_full`이 **이미 완비**돼 있었고(종단 5종 `T_G1`/`T_G2_6`/`T_G2_4`/`T_COVER`/`T_MAIN2`), 배선 arm도 완비, cfg도 `mp_repl=1`·`dd7_repl=1`. **유일한 차단 = `MP_SAFE_DISC` 화이트리스트에 0/1/3이 없어 filter가 떨군 것**(arm 전체가 死코드). 아래 §4의 "return 20+곳 종단 분류 = 시그니처 변경 필요"는 **불필요했음**.
> **적용 변경**(`C:\tfm2mods\tfm2_ai_adjust\src\tfm2_ai_adjust.rs`): ①`my_dd7700_full` 조기분기 게이트 `dd_early_p3_thr < p3` 삭제 + `rd_u8(p2+0x112) != 0` → `== 1` ②COVER 게이트 `dd_cover_p3_thr < p3` 삭제(무조건 블록화) ③`MP_SAFE_DISC: [u64;8] = [2,8,9,10,11,14,16,17]` → **`[u64;11] = [0,1,2,3,8,9,10,11,14,16,17]`**(→ **현행 = `[u64;12] = [0,1,2,3,8,9,10,11,12,14,16,17]`**, disc12 편입완·07-23·§7.2-A10 §6) ④명세 주석 정정.
> **빌드·배포 실측**: 컴파일 exit=0(신규 에러 0)·rustc 직접(사이즈가드 초과)·toolchain nightly-2026-05-24·sdk_052·`-C opt-level=1 -C overflow-checks=off` / dll **3,463,168B md5[:8] `2453713B`**(직전 3,463,680B `498BEB1E`)·신원검증 True·게임 미실행 상태서 배포완 / 롤백 백업 = `<게임>\mods\tfm2_ai_adjust\tfm2_ai_adjust.dll.bak_pre_disc013`.
> **write-set 대조(07-22 안전기준 통과)**: T_G1 `+0`만 / T_G2_6·T_G2_4 `+0,+8`(byte) / T_COVER `+0,+8=2` / T_MAIN2 `+0,+8,+9,+0xa` / SF경로·engage code6·7 = `None` passthrough(바닐라 비트동일) / `+0x0b..+0x2f` 미터치.
> ⬜**미검증(사실 승격 금지)**: 인게임 검증 전혀 안 됨. 이번 변경은 AI 판단을 실제로 바꾸므로 크래시뿐 아니라 **AI 행동 변화**도 관찰 필요. **첫 지표 = `mpws.txt`의 `[disc=0]/[disc=1]/[disc=3]` write-set이 `0b11`(+0,+8) 이하** — bit2(+0x10) 이상이 켜지면 명세 불완전 ⟹ 즉시 화이트리스트에서 제외. 보조 = `mpcmp.txt` disc0/1/3 OK/DIFF·크래시 로그.
> ⬜**미채택 확장(별건 후보)**: 모드는 `plan == 8`이면 무조건 passthrough(`my_dd7700_full` MAIN 진입부)라 **`plan==8 && sf!=f`**(원본은 MAIN BODY) 케이스를 놓침. passthrough=바닐라 비트동일이라 **안전**하고 커버리지만 손해. 게이트를 `plan==8 && u8[p7+0x3f7]==f`로 바꾸면 넓어지나 blast radius 억제 위해 보류.

> ★**정정 2 — `bl`(`byte[out+0xa]`) "4갈래" 명세는 오기** (ghidra-re 0.5.2 `0x1b91e40` 전수 disasm, 07-23):
> - ~~"bl ∈ {0,1,2}: 후보리스트 빔→2 / 루프히트→(i64[LR+0x18]<0?2:0) / sf경로 조건충족→1 / fallback→(i64[LR+0x10]<0x7d1) as u8"~~ → 뒤 2갈래는 T_MAIN2가 아니라 **SF 경로(`0x1b9219e~`) 전용**.
> - 진입 게이트 `0x1b9216e` = `plan(u8[p7+0x3f6]) == 8` **정확히 8**(`&0xfe` 아님 — 그 마스크는 COVER 게이트 `0x1b91ec9`에만) **AND** `sf(u8[p7+0x3f7]) == f`(`0x1b92191`) ⟹ **MAIN BODY 진입 = `plan != 8 || sf != f`**(plan==9도 MAIN). 두 경로는 **상호배타**.
> - ⟹ **MAIN BODY의 `bl`은 정확히 2갈래**: HIT(`0x1b92509`)→`(i64[LR+0x18]<0)?2:0`(`0x1b92583`/`0x1b9258b`) / 빈리스트·루프미히트→`2`(`0x1b92546` **단일 사이트**, `0x1b923f6`·`0x1b92437`이 동일 타겟). **현행 모드 재현이 정답 = 수정 불요.**
> - 원본에 `bl` **디폴트 대입 없음**(4사이트 배타 대입). 모드의 `local_58=2` 초기화는 MAIN 한정 무해하나 **"원본도 디폴트 2"는 사실 아님**.
> - **교차확인 일치(재조사 금지)**: 근접임계 `0x1b92482 SHR 8`+`0x1b92486 CMP 0x53d1ac0;JA` ≡ 모드 `<0x53d1ac1` / lane→오프셋 `f==0?0:f==1?0x28:0x50`(`0x1b9253a`) / `thr = nav+(1-team)*0x2e8+0x1e0+rlane*8` / 통과조건 `thr+0x78 >= vt[+0x28](ctx)` ≡ 모드 `s20<=thr+lane_margin`(리터럴 `0x78`=`dd_lane_margin` 기본값) / 팀 인덱스 비대칭(후보·thr=상대팀 `1-team`, LR=자기팀 `team`).
> - ⬜**미규명(추정 — 사실 승격 금지)**: `vt[+0xd0]`(팀 시야/인지 판정)·`vt[+0x128]`(id→엔티티)·`vt[+0x28]`(사거리류 스칼라)의 **의미**. `nav=param_6[2]`·`0x648/0x650=x/y`·`+0x5a8=엔티티id`·`0x1c98 그리드`는 기존문서 기반 **추정**.

> ★**편입 실익(실제)**: cfg의 `dd_frontier_mult=350`·`dd_lane_margin=600`·`dd_cover_count=0`·`dd_ratio_thr=31`(⚠**유저 튜닝값** — 아래 정정 참조) **4종이 전부 COVER 블록 안**이라 이제야 실제 AI 판단에 반영됨. `dd_near_dist=110250000`은 COVER가 아니라 `my_dd7700_code` STAGE4(engage) 소관 ⟹ engage가 code 2를 낼 때만 반영 = **부분 적용**.

> ★**정정 4 (0.5.3, 2026-08-03) — `dd_ratio_thr` 게임 원본값 = `31`이 아니라 **51**(`0x33`·사이트 `d80861`)**: 위 목록의 "31"은 **유저 튜닝값을 원본 상수로 오기**한 것(본문 §8 아래 표의 `(51)` 표기가 맞다). **의미도 확정 = "COVER 종단의 자기 체력% 임계"**이며, 구 설명 "라인 비율"은 **부정확**. 근거 = `0xd803f0` 전수 디스어셈 = **§7.3 §12.21(2)** + `REPORT\tfm2_ai_adjust\RE\2026-08-03_passive_line-조건트리-dd키매핑-0.5.3.md`.

(이하 = 편입 전 확보한 명세 원문 — 정정 2 적용해 읽을 것)
소스 `tfm2_ai_adjust.rs`의 `my_dd7700_code` 바로 위에 **전체 명세를 주석으로 기록**해 둠.
- 종단별 write-set: `T_G1`/`T_G7_7`/`T_G8_7`(code7)=**`+0`만** / `T_G2_6`·`T_G2_4`·`T_G7_6`=`+0`,`byte[+8]=f` / ★**`T_COVER`(code 4|7)=`+0`,`byte[+8]=2`** / `T_MAIN2`(code2)=`+0`,`byte[+8]=(i32[p5+0x8b0]==1)`,`byte[+9]=f`,`byte[+0xa]=bl`.
- ⛔★**함정: "code 7 = aux 없음"으로 단순화하면 `T_COVER`에서 갈린다** ⟹ code만으론 부족, 종단 정보 필요. ~~현 함수는 code만 반환 = 시그니처 변경 필요, return 20+곳 분류~~ → **불필요(정정 07-23)**: 종단 정보를 담은 `my_dd7700_full`이 이미 별도로 존재했음.
- ★`+8`은 **반드시 byte 스토어**(qword로 쓰면 `+9..+0xf` 잔재를 0으로 덮어 원본과 갈림. 실측 `code=7 +8=0x8`이 **잔재 보존 증거**).
- `+0x0b..+0x2f`는 **손대지 않는다**(원본도 잔재 커밋 · 병합기가 code 2/4/6/7에서 0x30 통복사). ⬜대안(0-fill)은 크래시 안전하나 비트동일 깨짐 — 소비자가 그 JT를 읽는지 **미규명**.
- `lane == u8[p2+0x116] == f` **항등 증명됨**. write-set은 **disc0/1/3 완전 동일**(dd7700이 `*p2`를 안 읽음).
- ★★**편입 실익이 이 회차 최대**: cfg에 `dd_frontier_mult=350`(기본30) · `dd_lane_margin=600`(120) · `dd_cover_count=0`(2) · `dd_ratio_thr=31`(51) · `dd_near_dist=110250000`(87891648) — ~~유저 튜닝 5종이 들어가 있는데 emit이 없어 전혀 반영되지 않는 상태~~ → **정정(07-23)**: 차단 원인은 emit 부재가 아니라 **화이트리스트 누락**이었고, 반영되는 건 **4종(COVER 블록 안)** + `dd_near_dist` **부분 적용**(engage code 2일 때만). 편입으로 **즉시 AI 판단이 바뀐다**(= 지금까지 편입한 disc들과 성격이 다름. 그것들은 레버가 전부 기본값이라 원본 비트동일 유지).

### 8. ⚠死레버 ~~3종+1~~ → **2종 확정 제거완 + 2종 잔존** (설정편집기 정리 대상)
0.5.2에서 **대응 게이트가 삭제돼 설정해도 무효**: `dd_early_p3_thr`(cfg=45) · `dd_cover_p3_thr`(cfg=4) · `poke_phase_gate`(cfg=49) · `poke_active_min`(cfg=11). 넷 다 **마침 기본값과 같은 값**이라 실질 손실은 없으나 **"안 먹는 슬라이더"**로 남는다.
- ✅**정정 3 (07-23)**: `dd_early_p3_thr`·`dd_cover_p3_thr` = 0.5.2 원본에 대응 게이트 없음 ⟹ **확정 死레버**(설정편집기 제거 대상). `my_dd7700_code`엔 07-23에 이미 반영돼 있었으나 **`my_dd7700_full`은 stale이었고 이번에 동반 수정**. ⚠특히 **`dd_cover_p3_thr`가 full의 COVER 블록 전체를 차단**하고 있었음(= disc0/1/3 편입 실익을 죽이던 두 번째 요인).
- ⬜잔존 = `poke_phase_gate`·`poke_active_min`(설정편집기 제거 미착수).

---

## §7.2-A8 · ★disc14 인게임 **재검증 통과 = 400/400 DIFF 0** + reposition_fight `0x23b6800` 델타 확정 + `d14_repl` 토글 신설 (2026-07-23, 0.5.2 buildid 24310934, dll `854B23F3`) — **본 절 = 이 건의 정본**

> ★§7.2-A6/A7이 disc12 편입작업(공유 콜리 3종 수정)으로 **무효화**했던 disc14 "400/400 DIFF=0"을 **정당하게 복원**한다. A6 §2·§3표·A7 §6의 "⬜재검증 대기"는 본 절로 해소.

### 1. 재검증 실측 (근거)
- **방법**: cfg `d14_repl=0` 토글로 **disc14만 passthrough+캡처** 전환(대체 중엔 리턴훅 미실행이라 game↔mine 비교 불가 — 이 우회가 필수) + dll `854B23F3` · `log=1` · `mpcap=1`.
- **결과**: `mpcmp.txt` **subplan=14 = OK 400 / DIFF 0 / PEND 0** · 크래시 0.
- ⟹ 07-23 콜리 3종 수정(`serpen_engage_gate` tick 사전게이트 삭제 + UNIT 출처 `ctx`→`mapobj` / `serpen_rng_pick` 가용게이트 신설 + 임의 상한 완화 / `serpen_reposition_fight` tick 사전게이트 삭제)이 **전부 옳았음이 실측 확인**됨.

### 2. ★reposition_fight `0x23b6800` 델타 확정 (관문 #1 해소)
- **오류 1건 = tick 사전게이트 삭제**(0.5.2 원본 tick 미참조) + **나머지 전항목 0.5.2 유효 확정**.
- **앵커 tag 하드코딩 `4`→`p4` 라이브 버그 동반 수정**(disc14 p4=5 경로에서 발화).
- ⟹ **5번째 연속 "사전게이트 삭제"** 패턴: disc1 p3 · disc12/14 code3 · engage_gate tick · reposition_fight tick. (0.5.2 마이그 일반 규칙 = §7.2-A7 §3.)

### 3. `d14_repl` cfg 토글 신설
- 기본 `1`=대체 / `0`=passthrough+캡처(재검증용). **死레버 아님 = 검증 도구.** ★`ec_gate_tick`은 reposition_fight·engage_gate **양쪽**에서 삭제 확인.

### 4. ⚠신규 미해결 2건 (별건 — 사실 승격 금지)
- **subplan7(disc7/Recall) DIFF 27건**(`my=8→game=7`) — ⚠**이번 수정과 무관**(disc7 콜리 미사용·`d7_repl` 기본0 passthrough). 1차 경기 0 / 2차 27 = 경기별 표본차로 드러난 disc7 재현 기존 미세오차. 별건 조사 대상.
- **subplan0(disc0) tail 오판 재현확인**: `my=7 path=0→game=2` 19건(2차) / 1차 25건(`my∈{6,7}→2`) = 동일 패턴 재현. STAGE6가 게임의 2(전진)를 7/6으로 과잉판정. 원인 미규명·추적 가능.

### 5. disc0/1/3 인게임 1차 확인 (편입 안전 확증)
- write-set `[disc=0]=0b11`(명세 부합·크래시0) + disc0 381/400(2차)·375/400(1차) + disc3 대체 12026회 발화. ⟹ **편입 자체 안전(write-set 합격)**, 재현 정확도 잔여 = 위 §4 tail 오판.

### 6. 死레버 현황 (갱신)
- 확정 死레버 = `dd_early_p3_thr`·`dd_cover_p3_thr`·`ec_gate_tick`(3종, 설정편집기 제거 대상) + `poke_phase_gate`·`poke_active_min`(기존 잔존).

---

## §7.2-A9 · ★★disc0/1/3(dd7700) 재현 **400/400 DIFF 0 달성** — STAGE6 ref-path 전면 교정 + `f22e80` 마이그 누락 복구 + **mode≠2 경로 신설** + 프론티어 폴스루 정정 (2026-07-23, 0.5.2 buildid 24310934, dll `CE992D10`) — **본 절 = 이 건의 정본**

> ★§7.2-A7 §7(disc0/1/3 편입·⬜인게임 미검증)·§7.2-A8 §4(subplan0 tail 오판 19~25건 / subplan7 DIFF 27건)·§5(disc0 381/400)를 **전부 해소**한다. dd7700 재현의 현행 정본 = 본 절.
> ⛔**요약 압축 금지 대상**(오늘 조건표 본문 소실 전례 있음). 수정 범위 = `C:\tfm2mods\tfm2_ai_adjust\src\` 전부.

### 0. 최종 실측 (근거)
- `mpcmp.txt`: **subplan 0 = OK 400 / DIFF 0 / PEND 0** · subplan 7 = **400 / 0 / 0** · subplan 14 = **400 / 0 / 0** · 크래시 0 · `dd0diff.txt` **0줄**.
- ★**유저 튜닝이 켜진 실사용 상태에서 400/400**: `dd_frontier_mult=350` · `dd_lane_margin=600` · `dd_cover_count=0` · `dd_ratio_thr=31` · `dd_near_dist=110250000`.
- 경위: **375/400**(시작) → 라우팅 교정 → `f22e80` 4건 → `f22e80` mode≠2 신설 → 프론티어 폴스루 → **400/400**.

### 1. [A] dd7700 STAGE6 ref-path **전면 교정** (`my_dd7700_code`) — 0.5.0_3 잔재로 base·오프셋이 통째로 stale
- **base**: ~~`sim + 0x860`(옛 "vt+0x168 모델")~~ → **`sim + 0xeaf0`** (= `vt->0x30(sim)`의 **RDX 반환**).
- **f==0(route_8679)**: ~~`+0x1c0`(게이트)/`+0x1b8`(ptr)~~ → **`+0x1d8` / `+0x1d0`**.
- **f==2**: ~~`+0x190`/`+0x188`~~ → **`+0x1a8` / `+0x1a0`** (= serpen W큐와 동일 = **교차확증**).
- ★**이중 역참조 복원**: 원본 `@141b92bc2→bc9` = `refv = u64[ u64[comp+0x1d0] ]`. 모드는 **1단만** 해서 **포인터를 핸들로 오인**하고 있었음.
- ★**kind 게이트 신설**: `vt->0x30`의 **RAX = GameMode enum**(0=Moba / 1=SingleLane / 2=DeathMatch). `kind!=0`이면 객체 크기가 `0xeb08`이라 `+0x1a8`/`+0x1d8` 읽기가 **OOB** ⟹ 원본이 `flag!=0 → LAB_141b92bf5`로 건너뛰는 이유. 판별은 기존 **`disc4_vt30_kind`(순수 리드) 재사용 — shadow-call 없음**.
- ★`vt->0x30` 본체 = **3-instruction leaf**: `lea rdx,[rcx+0xeaf0]` / `xor eax,eax`(또는 `mov eax,1` / `mov eax,2`) / `ret`. 타겟 3종 = **`0x141dc88b0`(kind0) / `0x141dca940`(kind1) / `0x141dcb750`(kind2)**. vtable 6개 = **3 구체타입**(`"Game"`(Moba) · `"SingleLaneGame"` · `"DeathMatchGame"`, 소스 `game-core\src\simulation\expected_game.rs`).
- **교차확증**: `vt0x30(gchild) RDX = gchild+0xeaf0`은 `serpen.rs` L877이 이미 순수재현 중이었고, **dd7700의 `sim`과 serpen의 `gchild`는 동일 객체**(둘 다 `geom[0][0]`).

### 2. [B] `my_f22e80_count` — 0.4.x→0.5.0 마이그 **통째 누락** 복구 (원본 `0x2126610`)
★**이 함수만 마이그레이션에서 빠져 있었음**(같은 값이 모드 다른 곳엔 이미 반영돼 있어 **한 함수 안에서 side가 두 값으로 갈리던** 상태).
1. **side**: ~~`p5+0x6a8`(0.4.x)~~ → **`p5+0x820`** (원본 `@142126846`).
2. **geo stride**: ~~`other*0x228`~~ → **`other*0x2e8`**(`@142126873`) / **lane idx**: ~~`h+0x738`~~ → **`h+0x8b0`**(`@142126ac4`).
3. ★**RNG draw 순서**: 원본은 `vt[0xd0]`(=`dd7_slot48_h`) 필터를 **draw 이전**에 통과시킴(`@142126985`). 모드는 draw를 먼저 해서 **한 건이라도 필터에 걸리면 이후 모든 roll이 밀림 = RNG 스트림 오염**. 추가로 `h==0`이면 원본은 `vt[0xd0]` 재호출(항상 false) ⟹ **무조건 reject**인데 모드는 accept로 뒤집고 있었음. accept-test의 **중복 `slot48` 항 제거**.
4. **윈도우 t 식 2단계 복원**: 원본 `t0=(a400*a218)/1000` → **`t=(t0 * u64[p5+0x3f8])/1000`**(`@14212667c..69c`), `clamp(100)`은 **마지막에만**. ⚠**0.5.2 신설이 아니라 0.5.0_3에도 있던 장기 누락**(당시 `0x141fd27e8`).
- 반환 = `Vec<*Building>`의 **len**(`out+0x18`) = `count_survivors`. margin `0x249f0`=150000은 **dd7700 콜사이트 리터럴**.

### 3. [C] ★★`my_f22e80_count` **mode != 2 경로 신설** — 본 회차 최대 발견
- ★**실경기 GameMode = 0(Moba)** 을 런타임 실측 확정(`dd0diff`의 `kind=0`). 그런데 기존 재현은 **mode==2(DeathMatch) 경로만** 구현 ⟹ **실전에서 한 번도 맞은 적이 없었음**.
- 원본의 mode 비교는 함수 전체에서 **`!= 2` 한 종류뿐**(`@1421266bf` · `@1421269b7`) ⟹ **mode 0·1은 완전 동일 경로**, 2만 별도. **2갈래 분기로 충분**.
- ★**전역 RNG(p4) draw = 0회**(로컬 시드 RNG만 사용) — mode2와 근본적으로 다름. ⟹ ★**RNG축 후속(07-28, tfm2_viewresult_probe v2)**: 이 「native dd7700=전역 0 draw」가 desync 진범 규명의 핵심 — 모드 `my_dd7700_rng_final`이 구 0.5.0/0.5.1 CAND_FILTER(`FUN_141fecbe0`) per-cand 전역draw 모델을 유지해 engage-code2서 전역 RNG **1~5 over-draw**→배경≠관전 desync였고, `tfm2_ai_adjust.rs:3719` `Some(true)`→`Some(false)`(engage-code2 consumes_rng 게이트)로 rng_final/writeback skip=전역 불변=native 일치 수정(**score 0/8**·⬜위치잔차50%). engage RNG소비자 f22e80=`FUN_142126610`은 kind!=2면 로컬 스택RNG `0x141b78380`/`0x141b18f30`, kind==2만 전역 `0x141b190a0`. 정본 = `MEM\DONE.md` 07-28줄 + `ANA\ai_adjust-rng-desync-전수조사.md` W1. ★본 §7.2-A9의 400/400은 **code축**이라 이 RNG축을 못 잡았음(교훈: code축≠RNG축≠위치축).
- **조기 빈-Vec 게이트**(`@1421266c9`~`@1421267af`): `kind!=2 && u8[p5+0x414]==1`이면 `FUN_1423a5a60` 술어 **2값**(★**AL·DL 2바이트 반환** — Ghidra 디컴 오독 주의, 호출부 `@142126785 OR DL,AL`로 확증) 중 **하나라도 참이면 빈 Vec 반환(COUNT=0)**.
  - 술어 내부: `den=max(10L,1)` · `q=T/den` · `rem=T%den` · `seed = A ^ (B<<4) ^ (q<<40) ^ 0x1A75E` (A=`sim+0xeab8`, B=`p5+0x818`) · `LocalRng::seed_from_u64(seed)` → `r1,r2 = gen_range(3L..=6L)` · `d3,d4 = gen_range(0..=9999)` · `S = min((T>=210L ? T-210L : 0)*6000/max(810L,1), 6750)` · `a=100-min(p5+0x208,100)` · `b=100-min(p5+0x210,100)` · `c=1000-p5+0x3f0`(각 **saturating**) · `P0=a²·S/10000` · `P1`(3단 곱/나눗셈) · `ret0=(d3<P0)&&(rem<min(r1+(S·L)/6000, 7L))` · `ret1=(d4<P1)&&(rem<r2)`. **비교는 전부 u32**.
- **본체 루프**(`@1421268e9`~`@142126e30`): 적팀 5슬롯 순회 · `vt[0xd0]` 필터(**draw 前**) 후 **목격 신선도로 2분기**:
  - **FRESH**(`age <= 3L`): 마지막 목격 좌표(`p7+0x218+i*0x10` / `+0x220`)로 `d=isqrt_dist`, **accept ⟺ `age*sp >= sat_sub(d,k)`**.
  - **STALE**: `r = age*K/max(L,1) + 40000` · `r>300000`이면 **skip** · `seed = ((T/max(6L,1))<<40) ^ (i<<8) ^ (A ^ u64[p5+0x818])` · 로컬RNG **3 draw**(`x1,x2 = gen_range(-1000..=1000)` · `rr = gen_range(0..=r)`) → 단위벡터화(`h=isqrt(x1²+x2²)`, 0이면 1) → **추정 위치** `px=max(lx+(x1*rr)/h,0)` · `py=max(ly+(rr*x2)/h,0)` → **accept ⟺ `d <= sp*3L + k + r`**.
  - 상수: `K = (x²·0x2d99999a4718 >> 43) + 3000` where `x = 100-min(i64[p5+0x218],100)` / `L = u64[ u64[ u64[p6+8]+8 ] + 0x12f8 ]`(**3단 역참조**).
  - **의미** = "안 보이는 적이 어디쯤 있을지 **원판 안에서 무작위 추정**".
- **구현**: 모드에 이미 있던 `LocalRng::seed_from_u64`(PCG-XSH-RR + ChaCha12) 재사용. RE가 준 시드 확장식 `(st>>45)^(st>>27)`과 모드의 `((st>>18)^st)>>27`이 **수학적으로 동일**함을 확인. 시그니처에 **`p6`·`kind` 인자 추가**.

### 4. [D] ★프론티어 게이트 — `return` → **폴스루** (잔여 6/400의 진범)
- 원본 `@1b91f7c`의 `jae 0x1b9216e`에서 ★**`0x1b9216e`는 MAIN BODY 진입점이지 반환 지점이 아님**. 함께 세팅되는 `r12b=2`도 **반환코드가 아니라 MAIN에서 소비되는 변수**(`@1b92175`).
- ⟹ **프론티어 bail = "커버 블록만 포기하고 MAIN BODY로 폴스루"**, 최종 code는 MAIN이 결정. (= 2026-07-19 0.5.1 교훈 "중간게이트 출력 상수단정 금지"의 **0.5.2 주소 수준 재확증** — [[tfm2-full-repro-methodology]] ★LESSON 07-19.)
- **커버 블록의 모든 실패 경로가 MAIN으로 폴스루**함: count 미달 `@1b92153` · `lane<3` `@1b91f8e` 동일.
- ★**게이트 자체는 전부 정확했음(재조사 금지)**: 발화조건 `u8[vobj+0x38]`+`bt 0x1a1`={0,5,7,8} / base `v1=u64[vobj+8]` **1단 역참조**(serpen·f22e80과 달리 **2단 아님**) / `30*l15`(원본은 `shl 5 - 2x` 강도축소, 리터럴 30 부재) / `prog=sat_sub(u19,30*l15)`(`cmovae`) / `prog<=s20`(`cmp rax,r8; jae`) / `s20 = vt+0x28 = i64[sim+0xeac0]`(타겟 `0x1dc8b80`, **exe 전역 유일 패턴**).
- ★`my_dd7700_full`은 **처음부터 불리언 플래그로 올바르게** 구현돼 있었음 — **`code`만 틀렸음**.

### 5. [E] 부수 수정
- `my_dd7700_rng_final`의 **`dd_cover_p3_thr` 게이트 혼자 잔존** → 제거(code·full은 같은 날 삭제했으나 여기만 누락). 원본 프롤로그에 p3를 4와 비교하는 코드 없음. 방치 시 `p3<=4`에서 커버 fire 예측 스킵 → **RNG-sync 오예측**. ⬜인게임 미검증.
- ★**회귀 롤백**: `serpen.rs:431` — 같은 날 `gchild+0xeaf0` → `geom`으로 바꾼 것이 **회귀**였음. Ghidra 디컴이 `param_2[0x34]/[0x35]`로 표시했으나 **실제 asm은 `@1423b5fe5 CALL [RBP+0x30]` 직후의 RDX**(=`gchild+0xeaf0`). **디컴파일러가 콜 클로버를 놓친 케이스** ⟹ **원래가 맞았음**, 복원 완료.
- ★**라이브 stale vt 슬롯 4곳**(`disc19_repro.rs`): `vt_call2(rvt,0x138,…)` → **`dd7_slot128`** / `vt_call2(rvt,0x150,…)`×3 → **`geom_resolve150`** (0.5.2에서 구 슬롯 ≥0x50이 **+0x68 시프트**: `0x138→0x1a0` · `0x150→0x1b8`). 구 번호로 부르면 엉뚱한 함수(`0x2306870` 등)가 불려 **값 오염**(읽기전용이라 크래시는 없음) ⟹ ★**disc7 DIFF 27건의 진범**, 수정 후 **disc7 400/400**.
- `tfm2_ai_adjust.rs` **5406·5629** `vt_call1(vt,0x168,obj)` = 0.5.2에서 `0x1d0`으로 시프트, 현 슬롯 타겟은 **쓰기 있는 대형 함수**(`mov [rdx+0x660]` + 0x6a8B memcpy)인데 rdx 미전달 ⟹ **부활 금지 경고 주석** 추가(현재 死코드라 무해).

### 6. ★검증 방법론 (재사용 규칙)
- ★**튜닝이 켜진 상태에서 game↔mine mpcmp를 재면 DIFF는 당연**하다(재현이 튜닝을 반영하므로). **순수 재현 정확도는 cfg를 게임 기본값으로 되돌리고 측정**해야 한다. 본 회차에서 **29건 → 6건**이 이 방법으로 분리됨(**23건이 튜닝 효과**).
- 튜닝 백업 = `tfm2_ai_adjust.cfg.bak_tuning_유저값`. 측정 후 **복구 완료**.
- ⚠**`DD7_TERM`은 STAGE6 도달 시에만 갱신**되므로 `path=3`(커버 단계 이탈) 로그의 `TERM` 값은 **직전 호출의 잔재** — 해석 시 무시할 것.
- **계측 인프라**: `DD7_PATH`(경로) + `DD7_TERM`(STAGE6 종단 40~56) + `DD7_DBG[0..10]`(ivar2/plan/bl/route/t86dd/t872d/count/near/n/efield/kind) → `dd0diff.txt`. `my_dd7700_code`는 **캡처 비교 전용**이라 계측 추가가 게임 동작에 무영향.
- ★`my_movepriority`는 **디스패처 진입 디투어에서 원본 실행 전에** 계산된다 ⟹ `RngSim::new(p4)`가 읽는 건 **진입 시점 state**. 소스에 있던 "리턴훅이라 원리적으로 어긋난다"는 주석은 **오진이었고 정정됨**(disc10/11 부류 아님).

### 7. ⬜미해결 (사실 승격 금지)
- **disc12 2건**: ①캡처 경로에서 `my_serpen_battle`이 **진입 가드로 733/1350 튕김**(`serpendiag [0]=733`) ⟹ `pokecmp`의 disc12 `[OK]`는 **`my=-99` 아티팩트로 무의미** ②재현이 **`0xe`를 한 번도 안 냄**(`serpendiag [11]=0`)인데 게임은 냄(`sgate` `my=12 game_q0=14`). engage 도달 1221회 전부 `0xc` ⟹ `serpen_engage_gate`가 항상 true이거나 거리 비교 이상 의심. **단 위 `serpen.rs:431` 회귀 롤백 이후 미측정**.
- **`0x1bd73a0`(다이브 후보 빌더) 전수 규명 완료**했으나 **[E] 재구축 경로는 여전히 미구현·passthrough**. 이유 = `st.len`만 뽑는 **축약 불가**(게이트가 양팀 데미지 매트릭스·스킬 가용·타워 데미지에 전부 의존). **순수재현 자체는 가능**(RNG 0·부작용 0)하고 알고리즘·상수·챔프별 쿨다운 테이블은 확보됨.
- ~~`disc19_repro.rs:3645`(disc13) **단일 역참조 = 이중이어야 할 가능성 HIGH·미확정**~~ → ✅**버그 CONFIRMED(07-23·§7.2-A10)**: 원본 `@14238fe7f-fe86`이 `mov rax,[rdx+0x1a0]`→`mov rdx,[rax]` **이중**. disc13/15가 **죽은 틀**이라 실害 0이므로 미수정(되살릴 때만 필수 수정).

### 8. 死레버 현황 (갱신)
- 확정 ~~3종~~ **5종** = `dd_early_p3_thr` · `dd_cover_p3_thr` · `ec_gate_tick`(reposition_fight·engage_gate **양쪽**에서 삭제 확인) · `poke_phase_gate` · `poke_active_min`. ~~(설정편집기 UI 제거만 잔여.)~~ → ✅**설정편집기 정리 완료(07-23·§7.2-A10)** + `d13_engage_hp_pct`/`d15_engage_hp_pct`도 ⛔DEAD(죽은 틀).

---

## §7.2-A10 · ⛔disc13/15 편입 **실익 0 판정·중단(재시도 금지)** + 라이브 결함 2건(1수정·1잔여) + 설정편집기 死레버 정리 (2026-07-23, 0.5.2 buildid 24310934, dll `32317D90`) — **본 절 = 이 건의 정본**

### 1. ⛔결론 = 편입 안 함 (재시도 금지)
- disc13(**EpicHunt**)·disc15(**SerpenCheck**) 둘 다 **죽은 틀 확정 = 영구 미발화**. 근거(기존 자산 07-11 §11.10-B) = 런타임 0발화 실측 + **정적 생성부재** 이중 증거(개인 subplan 교체 사이트 **69곳 전수** + 팀플랜 오더코드→disc 로우어링 매핑에 13/15 **없음**. disc13=Hunt용 오더코드 부재[Epic 분기는 `1→disc14`/`2→disc12`뿐], disc15=Serpen 스위치에 15 산출 케이스 없음).
- ⚠**disc3 반례는 적용 불가**: disc3도 "죽은 disc" 판정이었으나 **JT idx1이 `disc<2` 폴백 겸용**이라 핸들러가 실제로 살아 있었던 것. **disc13(idx11)/disc15(idx13)에는 그런 폴백 공유가 없음** ⟹ 편입해도 **한 줄도 실행되지 않음**, 리스크(§7.2-A3 movepri AV 클래스)만 부담.
- ⬜단 이 판정의 근거는 **0.5.0_3(07-11) 기준**이고 0.5.1/0.5.2 생성 사이트 재확인은 안 됨(부활 가능성 배제 불가하나 뒷받침 기록도 없음).

### 2. 0.5.2 RE 산출물 (편입은 안 하지만 확보된 사실 = 보존·재도출 금지)
- **디스패처 `0x2134240`** · **JT `0x38ae274`**(base-relative i32) · 좌석 `idx = (disc>=2) ? disc-2 : 1`(`@0x142134273 sub r11,2 / cmovae`).
- **disc13 핸들러 = `0x238fdd0`**(JT[11]) · **disc15 = `0x2390160`**(JT[13]). 0.5.0_3(`0x22d6d30`/`0x235d230`) 대비 **델타 불일치 = 전면 재정렬 재확인**.
- 교차확증: idx10=`0x238f130`(=disc12 헬퍼) ✓ / idx0·6=`mov [rsi],7`(disc2/8) ✓ / idx4=`0xa` writer(disc6) ✓ ⟹ **0.5.2 renumber 없음**.
- **인자(양쪽 동일)**: `rcx=out(sret)` · `rdx=subp+8` · `r8=p3` · `r9=rng` · `[rsp+0xe0]=p5=sim` · `[rsp+0xe8]=p6=geom` · `[rsp+0xf0]=p7=ctx/tp` · `[rsp+0xf8]=p8` **미사용**. ★`p3`는 picker의 param_2로 넘어가나 **picker 본문이 rdx를 한 번도 읽지 않음** ⟹ **p3 완전 미사용**(모드가 `_param3`으로 버리는 것이 정답).
- ★**write-set 전수**(두 핸들러 동일 형태 — 안전기준은 "0x30 전량"이 아니라 이 3집합과 **정확히 일치**):
  - code **7** = `+0x00`(8B)만, `+0x08~+0x2F` **전부 미터치**
  - code **0xb** = `+0x00`(8B)=0xb · `+0x08`(8B)=`*(mem+8)` · `+0x10`(**2B**)=1 · `+0x12`(1B)=0
  - code **0xd**(disc13)·**0x10**(disc15) = `+0x00`(8B)=code · `+0x08`(**1B**)=0
  ⟹ **원본도 잔재를 그대로 커밋한다**. 재현 함수(`my_disc13`/`my_disc15`) write-set = **원본과 비트동일** ✓
- **삭제된 사전 게이트 없음**(전체 disasm에 `cmp`+`jb/jbe` 조기 return 0건·panic 4종만) = 0.5.2 "게이트 대거 삭제" 패턴의 예외.
- **분기 트리**: `selfe=vt[0x1a0](gchild,*(sim+0x818))` → `pct=cur*100/maxhp` → `(rax,comp)=vt[0x30](gchild)`(rax!=0→panic·comp=`gchild+0xeaf0`) → `tgt = (*(comp+0x1a8)!=0) ? vt[0x1b8](gchild, **(comp+0x1a0)) : 0`(disc15는 `0x1d8`/`0x1d0`) → side/박스 판정 → **picker 무조건 호출** → `tgt 풀피`면 3게이트(inside&&부상→7 / pct<0x33→7 / picker.tag!=0→7) → tail `v = *(*(geom1+8)+0x12f8) + *(ctx+0x98)`(disc15 `+0xd0`), `v >u *(ctx+0xa0)`(disc15 `+0xd8`) && `*(mem)!=0` → 0xb, else 0xd(disc15 0x10).
- ⛔**되살릴 때만 필수 수정 2건**: ①`disc19_repro.rs:3645` 단일 역참조 → **이중**(원본 `@14238fe7f-fe86`, disc15는 이미 이중=맞음) ②`tfm2_ai_adjust.rs` disc13 arm 배선 결함 — `my_movepriority(13,…,0,0,0,0)`으로 **r8/r9/p7p 전부 0 전달**(ctx=0이라 tail 조건 무의미) + `if code == 2` 분기인데 **code 2는 나올 수 없음**({7,0xb,0xd}) ⟹ 0xb/0xd에서 aux 미기록 = **잔재 커밋 = §7.2-A3형 AV**. **`MP_SAFE_DISC`에 13/15 추가 금지.** 주석 출력계약 `{7,0x11,2}`는 0.4.x 잔재.
- ★**라벨 정정**: disc13 실명 = **EpicHunt**. 소스 주석 "AttackNexus"는 **0.4.14 유산 오라벨**이고 진짜 AttackNexus는 **disc18**. 정본 = `ANA\discovered-PROGRAM-STRUCTURE.md §3f-E1`(내장 Debug name-getter `FUN_141f3399c` 바이트 디코드·HIGH).

### 3. ★★라이브 결함 — 수정 완료 1건
- **`detour.rs` `apply_numbers_sp`의 stale 오프셋**(~L1361): ~~`r14+0x6a8`(side)·`r14+0x6a0`(self handle)~~ = **0.4.x 잔재** → **`+0x820`·`+0x818`**.
- ⚠**라이브였다**: cfg에 `tower_threat=200`·`numbers_threat=49`·`numbers_threat_move=2`가 켜져 실행 중이었고, side가 쓰레기·selfe가 잘못된 핸들로 해석돼 **numbers 후퇴 판정이 전 disc 공통 경로에서 오작동**하고 있었음.
- 모드 전역은 이미 `p5+0x820`/`p5+0x818`(dd7700·serpen·f22e80)인데 **이 함수만** 구 오프셋 = `my_f22e80_count`(§7.2-A9 §2)와 **동일한 "한 함수만 마이그 누락" 패턴** ⟹ ★규칙: 오프셋 정정 시 **동일 의미 read를 exe 전역 grep**할 것. ⬜인게임 미검증.

### 4. ~~⬜★라이브 결함 — 미해결 (잔여)~~ → ✅**해소 (2026-07-23, 0.5.2, dll `F0BD6F6C`) = 정본 §7.2-A13**
> ⚠아래 서술은 **발견 시점 기록(이력)**. 수정 내용·안전성 논거·⬜잔여는 **§7.2-A13**에만.
- **disc14의 RNG 홀**(`serpen.rs` ~L1074, `my_defense_nexus_050` 내부): `serpen_rng_pick(0, sim, …, false)` = **`rng=0`·`live=false`라 picker draw를 재현하지 않음**.
- 0.5.2 원본은 이 지점에서 picker(`0x2135350`)를 **무조건 호출**하고 후보 n>0이면 전역 RNG를 **실제 소비** ⟹ **disc14 대체 시 그 draw가 통째 누락 → RNG 스트림이 게임보다 뒤처짐 = desync**.
- ⚠**disc14는 이미 `MP_SAFE_DISC` 편입·라이브**이고 §7.2-A8에서 400/400을 받았으나 **RNG 축은 그 검증이 잡지 못한다**(code만 비교).
- **수정 방향**: `my_defense_nexus_050` 시그니처에 `rng` 추가 + **대체 경로에서만 `live=true`**(검증 경로에서 true면 이중 소비) — disc12 `my_serpen_battle`의 live 배선 패턴 준용. ~~**이번엔 경고 주석만·미수정**(배선 변경이 새 desync를 부를 수 있어 인게임 검증 동반 필요).~~ → ✅**이 방향 그대로 구현·배포완(07-23) = §7.2-A13**.
- ⬜동일 의혹: picker `n` 산출의 draw 수 완전 일치는 07-23 disc12 감사분에 의존 = 재검증 안 됨. **(이 항목은 여전히 ⬜ — A13 수정은 "draw를 하느냐"만 고쳤고 "draw 수"는 별개.)**

### 5. 설정편집기 정리 (`C:\tfm2mods\ai_adjust_editor\src\main.rs` · 재빌드 **8,378,368B**)
> ⚠**정정(2026-07-23 19:44)**: 본 절의 재빌드는 **빌드만 되고 게임 폴더에 배포되지 않은 채였음**(배포본은 07-16 16:56 · 8,376,320B 그대로). **07-23 19:44 배포(8,378,880B)로 해소** = §7.2-A11 §8. ⟹ ★**규칙: 편집기 수정 완료 기준 = 빌드 성공이 아니라 `mods\tfm2_ai_adjust\설정편집기.exe` 배포·타임스탬프 확인까지**(dll stale 함정과 동일 부류).
- 死레버 5종을 **폐기/死레버 섹션 이동 + 설명 `⛔DEAD`**: `dd_early_p3_thr`·`dd_cover_p3_thr`(lane) / `poke_phase_gate`·`poke_active_min`(견제) / `ec_gate_tick`(모르가드). 사유(0.5.2 원본에 대응 게이트 없음=값 무반영)를 설명문에 명시.
- **키 대체**: `ec_tgt_hp_low` → **`ec_self_hp_low`**(구 키 ⛔DEAD 표기·신 키 설명 신설). 사유 = 기존 재현이 **타겟** 체력을 봤으나 원본은 **자기** 체력 기준(0.5.2 disasm 확정).
- **`d13_engage_hp_pct` → ⛔DEAD**(disc13=죽은 틀).
- **✅LIVE(07-23 부활) 표기**: `dd_frontier_mult`·`dd_cover_count`·`dd_ratio_thr`·`dd_lane_margin`(이전엔 COVER 블록이 통째 차단돼 값이 안 먹었음) / `dd_near_dist` = **△부분반영**(engage 경로 한정). lane/battle 탭 note 갱신.

### 6. 기록 정정 (prior-work 지적)
- `MEM\DONE.md`·`MEM\CURRENT.md`·`MEM\tfm2-0.5.2-migration.md`·본 파일의 `MP_SAFE_DISC` ~~11종·"disc12 미편입"~~ → **12종 `[0,1,2,3,8,9,10,11,12,14,16,17]`**(disc12 편입완·`d12_repl` 격리 플래그 보유·소스 실측) = 제자리 정정완.

---

## §7.2-A11 · ★★byte-patch 노브 **40/41이 죽어 있었음**(라이브 결함) → disc19 imm **10사이트**·objective imm **12사이트** 재핀 + `O_ATHLETE_ID=0x810` 확정 + 잠복 지뢰 2건 `0x381e1e0` 통합 (2026-07-23, 0.5.2 buildid 24310934, dll `FB52A3AE`) — **본 절 = 이 건의 정본**

### 0. 배포 실측 (근거)
- dll **3,472,384B · md5[:8] `FB52A3AE`**(직전 3,473,920B `32317D90`) · 배포 **19:39** · 모드폴더 dll **1개** 확인 · 롤백 백업 `tfm2_ai_adjust.dll.bak_pre_immrepin`. ~~⬜**인게임 미검증**~~ → ✅**적용 검증완(0.5.2, 07-23 22:23 유저 인게임, 후속 dll `08CC1D7F` 런): `d19_imm applied=10/10`(구 0/15에서 첫 실반영·유저 retreat=51 반영·phase DEAD 표기 정상)·`obj_imm 12/12`(구 0/13)·크래시 0·신 base `0x7ff7f8160000`**.

### 1. ★★라이브 결함 = byte-patch 노브 **40 MISMATCH / 1 OK**
- 0.5.2 exe 실바이트 vs 소스 41사이트 **전수 대조**(PE 섹션 RVA→파일오프셋 매핑, 확정 함수 프롤로그 7개로 매핑 검증) ⟹ 40곳 stale.
- 런타임 로그 확증: `d19_imm.txt applied=0/15` · `obj_imm.txt applied=0/13` · `gb_imm.txt applied=0/12` · `vis_imm.txt applied=1/1`.
- ⚠**라이브였다**: 유저 cfg가 `d19i_enable=1`·`oi_enable=1`로 켜져 있었으므로 설정한 후퇴 임계·넥서스 방어/공격 튜닝이 **전부 게임 원본값으로 돌고 있었음**. 크래시는 없음(prefix 검증 실패 → skip = 무개입).
- ★**재사용 감사 기법(규칙화)**: byte-patch 사이트는 exe 실바이트로 **prefix + 원본 imm 지문**을 대조하면 Ghidra 없이 5분에 stale 여부를 **전수 판정** 가능. 패치 후에도 같은 스크립트로 재검증(본 건도 이 방식으로 에이전트 결과를 독립 확인).

### 2. ✅disc19 severity ~~15사이트~~ → **10사이트 재핀 확정** (`disc19_repro.rs` `apply_disc19_imm`)
- 컨테이너 = **disc19 본체 `0x2380820`**, severity 인라인 블록 시작 **`0x2380e16`**(단일).
- 확정 10 (전부 width=1·imm_off=3, 원본 imm 지문 실바이트 재확인):
  `sr0 tr>49=0x2380e16(48 83 f8, 0x31)` / `sh1 hp<66=0x2380e1c(**48 83 fe**, 0x41)` / `sr1 tr>29=0x2380e22(48 83 f8, 0x1d)` / `sh2 hp<41=0x2380e28(**48 83 fe**, 0x28)` / `sr2 tr>17=0x2380e2e(48 83 f8, 0x11)` / `sh3 hp<26=0x2380e36(**48 83 fe**, 0x19)` / `sr3 tr>9=0x2380e3c(48 83 f8, 0x0a)` / `ah#1 ally>50=0x2380e92(48 83 f8, 0x32, 64bit div)` / `ah#2 ally>50=0x2380ec0(48 83 f8, 0x32, 32bit div)` / `rhB hp>=46=0x2380ecd(**48 83 fe**, 0x2e)`
- ★**레지스터 변경**: hp 비교가 0.5.1 R15(`49 83 ff`) → 0.5.2 **RSI(`48 83 fe`)**.
- ⛔**나머지 5사이트는 미탐색이 아니라 게임이 삭제**(0.5.1↔0.5.2 명령 시퀀스 대조로 확증): phase 게이트 4곳(`pt ph>=30` 구 `0x1e0e2d7` / `pa#1·2·3 ph>=39` 구 `0x1e0e498`·`0x1e0e532`·`0x1e0e5c2`, + 모드가 안 건드리던 4번째 구 `0x1e0e1ea`) + **rhA**(hp>45 #1, 구 `0x1e0e4b4`) ⟹ **10/10 = 그 버전 기준 전량 반영**(부분반영 아님).
- ⟹ ★cfg **`d19_phase_threat`·`d19_phase_ally` = ⛔DEAD 신규 확정**(설정편집기 死레버 목록 추가 대상 = 아래 §7).
- ⟹ ★**동작 변화(체감 가능)**: 0.5.2 disc19는 **매치 phase와 무관하게 threat/ally 판정을 상시 수행**한다(0.5.1엔 phase≥30·≥39 게이트가 있었음).
- ⛔**오답 폐기(재조사 금지)**: 구 리드 "severity가 두 함수로 분리·`0x22f8a90`(tr49=`0x22f8d6e` 등 4개)" = **오답**. `0x22f8a90`은 자체 severity 사본을 가진 **다른 핸들러**(hp 레지스터 R15·tr9 imm=`0x09`·트레일러가 ally 아님) — 거기 패치했으면 완전 오패치.
- ⛔**재조사 금지**: `0x2380c28`/`0x2380c40`의 `cmp ?,0x1d`는 phase가 아니라 **맵 그리드 타일 clamp**(0.5.1 `0x1e0e1ca`/`0x1e0e1e2`에도 동일 존재). pt 후보로 쓰면 오패치. 0.5.2 disc19 전 구간(`0x2380820`~`0x2382cb0`)에 `cmp ?,0x2d`·`cmp ?,0x26`은 **0개**.
- **식별 지문(보존)**: severity 인라인 사본이 exe에 6개(`0x22e3cdf` `0x22edb5f` `0x22effff` `0x22f8d6e` `0x2380e16` `0x23a0c21`)인데 **ally `0x32` ×2 + rhB `0x2e` 트레일러를 가진 건 `0x2380e16` 하나뿐**(tr9=`0x0a` 단독은 지문 불충분).

### 3. ✅objective imm ~~13사이트~~ → **12사이트 재핀 확정** (`detour.rs` `apply_objective_imm`)
- 컨테이너 3종: **dn-A `0x1b92e40`**(`0x1b92e40`~`0x1b93569`) / **dn-B `0x1bdaaa0`**(`0x1b934bc`·`0x1b934d6`에서 호출) / **an `0x2376320`**(`0x2376320`~`0x2377af1`, 0.5.1 disc18 `0x1c7ca20` 후계).
- 확정 12(실바이트 재확인 완료): `dn_nexus_hp#1=0x1b934a4(48 83 f8, 0x32, 64bit div)` / `dn_nexus_hp#2=0x1b934b0(동, 32bit div)` / `dn_hp_low=0x1b934ec(48 83 7d d8, 0x1f)` / `dn_hp_crit=0x1b9351c(48 83 7d d8, 0x15)` / `dn_near_dist#1=0x1b9302c(48 b8, 0x35a4e9001)` / `#2=0x1b93152(동)` / **`#3=0x1b933d8(49 ba, 0x35a4e9000 = +1 없음)`** / `dn_pred_dist=0x1bdac25(48 b8, 0xd693a4001, exe 전역 유일)` / `dn_lane_margin=0x1bdac95(49 83 c6, 0x78)` / `an_cull_dist=0x2376e86(**49 81 fa** = cmp r10, 0.5.1은 r8, imm32 0x5f5e0)` / `an_finish_hp#1=0x23777fe(48 83 f8, 0x38, 64bit div)` / `#2=0x237780a(동, 32bit div)`
- **신규 발견 3사이트** = `dn_near_dist#3` · `dn_nexus_hp#2` · `an_finish_hp#2`. ★**#2 계열은 64bit/32bit div 두 경로의 동일 임계라 둘 다 패치해야 한다** — 0.5.1 배선이 한쪽만 패치했던 것은 **잠재 결함**이었음.
- ★**인코딩 함정**: `dn_near_dist#3`만 `d²`(+1 없음)라 기존 `sq()`(d²+1) 헬퍼를 쓰면 임계가 1 어긋남 ⟹ **`sq0()` 신설**.
- ⛔**0.5.2 삭제 2**: `dn_count_gate`(해당 위치가 `cmp rdx,[rbp-0x50]` **레지스터 비교로 대체 = 상수 소멸**) · `dn_hp_crit #2`(두 사이트가 1곳으로 병합).
- ⛔★**`an_count_gate`는 오식별 = 영구 폐기(재핀 금지)**: 구 사이트 `cmp qword[rbx+0x5b0],5`는 튜닝 레버가 아니라 **컴파일러 배열 bounds-check 관용구**였다. 0.5.2에 동일 패턴 **37곳**, 예외없이 `cmp [X+0x5b0],N` → `lea 정적더미` → `cmovae 실원소` → `cmp [reg+0x30],-1` 형태이고 imm=3/5가 **항상 짝**으로 등장(0.5.1에서 "짝구조 일치"를 강후보 근거로 삼았던 게 바로 이 지문). **N을 바꾸면 없는 원소를 실포인터로 읽어 OOB → 크래시/미정의.** an 컨테이너(`0x2376320`) 안엔 이 패턴이 아예 없음. ⟹ 구 §7.1/config-editor의 "an_count_gate = 엔티티 reach 선형계수" 시맨틱 규명(07-16)도 **무효**.
- **컨테이너 확증 근거(보존)**: dn-A는 `cmp rax,0x32` + `cmp qword[rbp±],0x1f` + `cmp qword[rbp±],0x15`를 **동시 만족하는 exe 유일 함수** / dn-B는 `0xd693a4001`이 **exe 전역 1건** / an은 컬링 `0x5f5e0` 2곳 중 `cmp rax,0x38`을 함께 가진 유일 함수.
- **dn-A 프레임**: `[rbp-0x50]`=넥서스 maxHP(`[obj+0x610]`) · `[rbp-0x48]`=curHP(`[obj+0x658]`) · `[rbp-0x28]`=HP%(`cur*100/max`).
- ⟹ ★cfg **`oi_dn_count_gate`·`oi_an_count_gate` = ⛔DEAD 신규 확정**.

### 4. ✅`O_ATHLETE_ID` = **`0x810`** 확정 (라이브 결함 수정 — `tfm2_ai_adjust.rs:503` `0x698`→`0x810`)
- 근거 = struct B **생성자 `FUN_1422cb050`(RVA `0x22cb050`)** 의 필드 3연속 스토어(실바이트 확인):
  `0x22cb52d: 48 89 be 10 08 00 00`(`mov [rsi+0x810],rdi` = id) / `0x22cb534: 48 c7 86 18 08 00 00 00 00 00 00`(`mov qword[rsi+0x818],0` = self handle) / `0x22cb53f: 48 89 86 20 08 00 00`(`mov [rsi+0x820],rax` = team).
- 0.4.13_5 동일 생성자 `FUN_1418b1c40`의 `0x698/0x6a0/0x6a8`과 **1:1 동형** ⟹ 의미 승계. 이 3연속 패턴은 각 버전에서 **정확히 1건**(0.5.0_3 `0x2079480` · 0.5.1 `0x21d9810` · 0.5.2 `0x22cb050`) = 다중매치 없음.
- 팽창 정체 = `[+0x180]` 인라인 블록 `0x210→0x298`(+0x88) + 후속 +0xF0 = **+0x178**(`0x698+0x178=0x810`, `0x6a0`/`0x6a8`→`0x818`/`0x820`과 동일 델타).
- ⟹ **team_gate(선수별 판단 오버라이드 우리팀 게이트) 오작동 = 소스 수정 완료**(⬜인게임 미검증). 07-17 이래의 "⛔STALE·수정 미착수" 상태 **해소**.

### 5. ✅잠복 지뢰 2건(`0x35e4d00` ATK_VT / `0x3599b30` ability_table) 해소 — **둘 다 `0x381e1e0`으로 통합**
- **ATK_VT = `0x381e1e0`**(= `RVA_C8C_DMG_SHEET`와 동일 값) · 확신도 **HIGH**·실바이트 확인: 0.4.13_5 `FUN_14206e530`의 0.5.2 대응 = **`FUN_141b93830`(`0x1b93830`, len `0xd44`)**(imm 지문 + 엔티티 memdisp 카운트 전량 일치 + athlete `0x6a0/0x6a8`→`0x818/0x820` 이동 + 간접호출 12개 순서 동일, vt 슬롯만 `0x128→0x1a0`·`0x40→0x90` 이동). 사이트 `0x1b93bb6`·`0x1b94354` = `4c 8d 0d …`(lea r9 → RVA `0x381e1e0`) 직후 `41 ff 52 28`(`call [r10+0x28]`).
- **`ability_table`은 별개 테이블이 아님**(확신도 MED) ⟹ **상수 폐기·통합**: `0x3599b30`은 **0.4.13_4에만** 존재했고 그 버전에서 플랜-AI 3함수(`0x1e03900`·`0x1e124f0`·`0x1e190b0`)가 전부 그 하나를 공유 = ATK_VT와 같은 논리 desc의 **CGU 클론**. 0.5.2에서 `call [+0x28]`에 쓰이는 desc는 5개(`0x381e1e0`·`0x38832a8`·`0x38a22b0`·`0x38c61b0`·`0x38d1918`)뿐인데 **slot0(drop) 외 size `0x6a8`·align 8·메서드 7슬롯이 전부 바이트 동일**(모드가 쓰는 slot `0x30` = `0x141bebd80` 포함) ⟹ 어느 클론이든 동작·AV 위험 동등.
- ⛔과거 오답 `0x38832a8` = 무관 함수 `FUN_142031110`(len `0x4e`) 전용 클론.
- ★**단 실害는 원래 0이었음**: 유일 소비처 `disc4_subplan_r13b`·`disc4_ttd_acc`가 **호출자 없는 死코드**(disc4는 `my_disc4_050`으로 라우팅) + desc 화이트리스트 가드 이중 차단.
- ⛔**死코드를 되살릴 때 필수 수정 3건**(소스 주석에 기록완): ①능력 게터2 슬롯 `vth+0x40` → **`vth+0x90`** ②호출규약 `rcx=sret, rdx=buf, r8=sim, r9=target, [rsp+0x20]=desc`(**5인자·desc가 스택**) ③반환 검사 위치 `cmp dword[sret+0x48],-1` / `cmp dword[sret+0x80],0`(현 코드는 `out[0]`/`out+0x40`). + 기존 기록된 `disc4_ttd_acc` vt 슬롯 `0x168`→`0x1d0`.

### 6. ✅기타 감사 (라이브 위험 없음 확인)
- `rva_052.rs` 밖 하드코딩 리터럴 나머지(disc19 shadow-call 5종 `0x20a3fd0`·`0x1fce700`·`0x1fbe950`·`0x237d910`·`0x236b6b0` / genbuild 0.4.13_5 값 5종)는 전부 **cfg 기본 OFF + `code_ptr_ok` 가드** 또는 `MIG_GB_CHANGED=true`로 훅 미설치 ⟹ **잠복만·라이브 위험 0**(= §7.2-A6 §5 잔여④ 해소).
- ~~`apply_gb_imm` 12사이트는 여전히 **0.5.1 주소·`applied=0/12`**이나 cfg **`gb_enable=false`** 라 무해 ⟹ ⬜**잔여로 남김**~~ → ✅**해소(0.5.2, 07-23): 10사이트 재핀 = §7.2-A14 §4, 인게임 `gb_imm 10/10`(07-23 22:23)**.
- mpcmp 계측 오배선(disc10/11이 `out+0`=상수 `0xb`를 판단값으로 읽음)은 **수치 표시만 무의미**하고 disc10/11 실검증은 별도 full-output 바이트대조 경로로 수행됨 ⟹ 긴급도 낮음·다음 캡처 때 동반 수정 권장.

### 7. ⬜미해결·후속 (사실 승격 금지)
- ⬜★**disc 번호 정본 재확인 필요**: 0.5.2에 서브플랜 디스패처가 최소 2개(`0x2134240` JT `0x38ae274` / `0x1dabcc0` JT `0x3842688`)이고 **같은 idx에 다른 타깃**을 준다(전자 idx15→`0x1b92e40`, 후자 idx15→`0x2376320`·idx16→`0x2380820`). 기존 기록의 "disc18=JT[16]"은 JT[16] 이후가 **인접 테이블 쓰레기값**이라 성립하지 않을 수 있음. ★단 위 §2·§3의 배선은 disc 번호가 아니라 **imm 지문 기반 함수 동정**이라 이 불확실성의 영향을 받지 않음.
- ~~⬜disc14 RNG 홀(§7.2-A10 §4) = 여전히 미수정.~~ → ✅**해소(07-23, dll `F0BD6F6C`) = §7.2-A13** / ~~⬜인게임 미검증~~ → ✅라이브 생존 확인(07-23 22:23)·⬜desync 축 누적 관찰 잔여(A13 §4).
- ~~⬜인게임 미검증: 본 건 전체~~ → ✅**본 건(imm 재핀) 적용 검증완(07-23 22:23, §0)** / ⬜유지 = disc12 편입분·`apply_numbers_sp` stale 수정분(발화·동작 지표는 log=1 필요·이번 런으론 판정 불가).
- ~~⬜문서 동기(다른 작업으로 남김) 2건~~ → ✅**둘 다 완료 (2026-07-23 19:44, 0.5.2)** = 아래 §8.

### 8. ✅문서 동기 + 설정편집기 반영 (완료, 2026-07-23 19:44)
- **설정편집기** `C:\tfm2mods\ai_adjust_editor\src\main.rs`: disc19 탭에 **`§⛔ 死레버 (0.5.2에서 무효 — 값 바꿔도 무반영)` 섹션 신설** + `oi_dn_count_gate`·`oi_an_count_gate` 이동(작동 목록에서 제거)·설명 ⛔DEAD 교체(`oi_dn_count_gate`=상수 소멸[`cmp rdx,[rbp-0x50]`] / `oi_an_count_gate`=**애초에 오식별**·bounds-check 37곳·**바꾸면 OOB 크래시·재핀 금지** + 구 07-16 "강화방어 프로파일 임계" 설명 **폐기 명시**). 탭 note = "0.5.2 재핀 완료(07-23) — 그 전까진 0.5.1 주소라 oi_*·d19_* 전부 무반영(applied=0/13·0/15) → 현재 oi **12/12**·d19 **10/10**" + "⚠phase 게이트 삭제 ⟹ 시간대 무관 상시 판정". 빌드 exit 0 → **`ai_adjust_editor.exe` 8,378,880B(19:44:38)** → 게임 `mods\tfm2_ai_adjust\설정편집기.exe` **배포 완료**.
- ⚠★**부수 발견 = 배포 누락 사고**: 배포 직전 게임 폴더에 있던 편집기는 **8,376,320B·2026-07-16 16:56** ⟹ **§7.2-A10 §5의 "07-23 死레버 5종 정리·재빌드 8,378,368B"가 빌드만 되고 배포되지 않은 채였음**(이번 배포에 함께 실려 해소). ⟹ ★**규칙: 편집기 수정은 "빌드 성공"이 아니라 `설정편집기.exe` 배포·타임스탬프 확인까지가 완료**(dll stale 함정과 동일 부류).
- ⚠**`d19_phase_threat`·`d19_phase_ally`는 편집기에도 cfg에도 원래 없는 키**(소스 `tune()` 기본값만 존재) ⟹ **편집기 표기 대상 아님·유저 영향 없음**. 다음에 "표기 누락"으로 오인하지 말 것. ⟹ 신규 ⛔DEAD 4종 중 **편집기 반영 대상은 `oi_*_count_gate` 2종뿐**.
- **`MODS\tfm2_ai_adjust\SUBPLAN_동작_전수조사.md` 동기 완료**(CLAUDE.md §3): 플랜18 절 = `oi_an_count_gate` 취소선+⛔죽음(쉬운말 사유)·설정값 줄 2개로 축소·"재핀 전까지 무반영 + `oi_an_finish_hp` 두 갈래 중 한쪽만 고치고 있었음" 인용블록 / 플랜19 절 = 단계값(30/39)·`oi_dn_count_gate` 취소선+⛔죽음 + "2026-07-23(0.5.2) 중요 변경 3가지" 인용블록(①그동안 무반영 ②단계 조건 삭제→초반부터 상시 작동 ③`oi_dn_nexus_hp` 두 갈래 동시 패치·`oi_dn_near_dist` 3번째 사이트).

---

## §7.2-A12 · ★인게임 챔피언 **레벨 상한 메커니즘 규명** + 레벨업 함수 0.5.2 재핀 + 참가자 레코드 오프셋 마이그 (2026-07-23, 0.5.2 buildid 24310934) — **RE 정본 = `ANA\discovered-PROGRAM-STRUCTURE.md §13.2-levelcap`**(여기는 RVA 표만)

### 1. 결론 (모드 seam 판정)
- **인게임 최대 레벨 = `need_exp.len() + 1`. 하드코딩 상수 상한 없음**(확신도 A). 현행 = **레벨 12**(need_exp 11엔트리). 상한 게이트 = 순수 len 경계 `cmp rdi,rdx`/`ja` → **조용한 return**(패닉 아님), 루프 없음(호출당 최대 1레벨).
- ~~★**권장 seam(S1) = `need_exp` 배열 확장(데이터)**(`mod.override_info` merge로 3벌 game_setting 교체)~~ → ⛔**정정(0.5.2, 07-23 런타임 실측): S1 단독 불가** — **GameSetting은 시뮬마다 복제**되어 `len=11` 인스턴스가 계속 새로 생기고, 테이블 2벌 공존 시 **경험치 바 폭주**(§6-D). ★**정답 = S2(런타임 ptr 강제) = `tfm2_level_cap` v2.0.0, ✅인게임 검증완** — 상세 = **§6**. (override_info 처리 정본 = `MEM\tfm2-asset-override-merge.md`.)
- ⛔**S4 = `ja @0x22d3ff4` NOP 바이트패치 = 재시도 금지**(가드 제거 시 바로 뒤 `jae @0x22d4001` bounds panic 발화 → `ud2` 하드크래시). ⛔S5 = len만 리다이렉트 = ptr 그대로라 OOB read.
- S2(런타임 Vec 교체, 얼로케이터 `0x1408b7f80` 필수)·S3(함수 완전 detour, 본문 ~0x620B) = 중위험/고비용.

### 2. 0.5.2 RVA (레벨업 함수 = **`0x22d3c60`**, 구 0.4.14 `FUN_141a2a330`=`0x1a2a330`)
| 항목 | RVA(0.5.2) |
|---|---|
| level 로드 `mov rdi,[r13+0x880]` / need_exp.len `mov rdx,[r14+0xd10]` / `cmp rdi,rdx` | 0x22d3fe3 / 0x22d3fea / 0x22d3ff1 |
| ★상한 게이트 `ja`(조용한 return) `0F 87 62 02 00 00` | **0x22d3ff4** |
| bounds panic `jae` `0F 83 9A 03 00 00`(도달불가 가드) | 0x22d4001 |
| need_exp.ptr 로드 / `exp -= need[lv−1]` | 0x22d4007 / 0x22d4015 |
| entity 획득 `call [rax+0x1a8]`(구 0.4.14 vt+0x130) / 레벨 기록 `mov qword[rax+0x5b0],rdi` | 0x22d412e / 0x22d413d |
| 조용한 return 에필로그 | 0x22d425c |
| exp배율 오프셋 테이블 `[0x13a0,0x13a0,0x13a0,0x13a8]`(idx=mode−2) | 0x38cff50 |
| 직접 콜러 17곳 | 0x2312919, 0x2312a0b, 0x2312d92, 0x2313916, 0x2313b07, 0x2314d75, 0x2314f0b, 0x2315025, 0x232b7f6, 0x232cbbc, 0x232cd46, 0x232ce68, 0x232d8c5, 0x2336fa3, 0x2338d80, 0x2338fa0, 0x2339101 |
| UI 경험치 바 need_exp 소비처(동일 len 가드 `cmp rcx,[rax+0xd10]`@0x80ae73 / `ja`@0x80ae7a) | 0x80b6eb |

- ★**재핀 기법(툴화 권장)**: 마스크 시그니처는 **0.5.0에서 이미 깨져 실패** ⟹ **Rust panic `Location` 앵커**로 확정. 0.4.14 `game-core\src\simulation.rs` **651:40/663:42** → 0.5.2 **851:40/863:42**(라인 +200·컬럼 동일). 스크립트 `loc052.py`.

### 3. 구조체 오프셋 마이그 (0.4.14 → 0.5.2) — 기존 기록 정정분
- **참가자 레코드 stride ~~0x758~~ → `0x8d0`**, 배열 = **GameData `+0x840`(ptr) / `+0x848`(len)**(구 `+0x808`) [stride=A · 베이스 객체 동일성=B]
- exp ~~0x6f8~~→**0x870** / level ~~0x708~~→**0x880** / growth EntityStat ~~0x6b0..0x6f8~~→**0x828..0x870** / 이벤트 플래그 ~~0x740~~→**0x8b8** / 이벤트 id ~~0x748~~→**0x8c0** / 부가필드 ~~0x560~~→**0x5f0**
- **GameSetting 무변경**: need_exp `+0xd00`(cap)/`+0xd08`(ptr)/`+0xd10`(len), `+0x8a8`·`+0x12f8`·`+0x13a0`/`+0x13a8`·`+0x14c8` 동일. **entity 무변경**: `+0x5b0`(level)·`+0x5b8..0x5f0`(base 스탯 4×xmm)·`+0x5f8`·`+0x658`(curHP).
- ⚠**정정**: `entity+0x5b0` = ~~i32~~ → **8바이트(qword)**(기록도 `mov qword`, 소비처도 `mov r64` 후 `cmp r64,3/5`). ⚠**정정**: `tfm2_comptest_unlock` 소스 주석의 "레벨 +0x5e0" = **오기**, 정본 `+0x5b0`(소스 주석 미수정 = 잔여).

### 4. 안전성 (상한 상승 시)
- need_exp 소비처는 **.text 전체에 위 2곳뿐**이고 둘 다 len 가드 보유 ⟹ UI도 데이터 구동. **레벨 값을 배열 인덱스로 쓰는 지점 0건**, 레벨 상수 비교는 스킬 해금 `>=` 게이트뿐(`cmp r64,3`/`cmp r64,5` — 0x1b89310·0x1b893d2·0x1b8e580·0x20f32a7·0x2101a3b 등) ⟹ **OOB로 터질 지점 없음**(스탯은 growth 가산식 = 테이블 인덱싱 아님 → 값 폭주만).
- GameSetting `+0xd08`/`+0xd10` **쓰기** 2사이트 = `0x1c27272`(+0x1c27280) / `0x21bbf2e`(+0x21bbf3c) — Deserialize/Clone **추정(확신도 B)**.

### 5. ⬜미확인 (사실 승격 금지)
- `[rdi+0x140]`(레코드 획득)·`[rdi+0x1a8]`(엔티티 획득) vtable 슬롯의 구체 타깃 / ~~GameSetting Clone 전파 여부~~ → **해소 = 시뮬마다 복제됨**(§6-C).

### 6. ✅**구현 정본 — `tfm2_level_cap` v2.0.0**(2026-07-23, 0.5.2 buildid 24310934, **인게임 검증완**)
- **결과**: 인게임 레벨 **12 → 설정값(기본 18)**. 검증 = **레벨 17 도달 스크린샷 + 경험치 바 정상 복구**(⟹ DONE 승격). 릴리스 `<게임설치>\mods\release\0.5.2\tfm2_level_cap.zip` **95,275B**(zip 루트 `tfm2_level_cap\` 한 겹·README/로그 제외). 소스 `C:\tfm2mods\tfm2_level_cap\`(lib.rs+README.txt+cfg). 구성 = `mod.mod_info` / `mod.override_info`(**`{}` 빈 값**) / `tfm2_level_cap.cfg` / `tfm2_level_cap.dll`. **티어 = 비-T1**(패치 마이그 대상 아님 — 단 아래 2 RVA는 패치마다 재핀 필요).

**A. 트램폴린 2사이트** — 7바이트 사이트를 `call rel32 + nop nop`으로 치환(mid-function). 진입 시 GameSetting의 need_exp Vec을 모드 테이블로 강제 교체.

| 사이트 | RVA(0.5.2) | 원본 7B | GameSetting 레지스터 |
|---|---|---|---|
| 레벨업 | **0x22d3fea** | `49 8b 96 10 0d 00 00`(mov rdx,[r14+0xd10]) | **r14** |
| UI 경험치 바 | **0x80ae73** | `48 3b 88 10 0d 00 00`(cmp rcx,[rax+0xd10]) | **rax** |

- 교체 내용 = `+0xd08`=모드 테이블 ptr / `+0xd10`=len / ★**`+0xd00`(cap)=0**.
- ★**cap=0이 핵심 안전장치**: Rust `RawVec::drop`은 cap==0이면 dealloc하지 않으므로 **모드 배열이 게임 얼로케이터로 free되는 사고를 원천 차단**(원본 버퍼는 leak = 88B 수준). UI가 cap을 읽는 코드는 **0건**(계산함수 `0x803b30` 전수 스캔) ⟹ 부작용 없음.
- ★**판정 기준 = ptr 비교(`[+0xd08] == 내 테이블?`), ⛔len 비교 금지** — len으로 판정하면 merge로 len만 같아진 인스턴스를 지나쳐 **시뮬/UI가 서로 다른 테이블**을 보게 된다(=D의 사고 구조).
- **기법 3가지**: ①UI 스텁은 원본 `cmp`의 flags를 바로 뒤 `ja`가 쓰므로 **스텁 말미에 원본 cmp를 재실행해 flags 복원**(pop·mov는 flags 불변) ②`je`는 **near(`0F 84 rel32`)** — rel8이면 교체 블록이 127B 넘을 때 조용히 깨짐 ③`call rel32` 사거리(±2GB) 확보 = **대상 주소 주변 64KB 단위 왕복 스캔 VirtualAlloc**(`alloc_near`). 테이블은 cfg에서 읽어 **`Box::leak`으로 고정**(트램폴린이 주소를 imm64로 박음 ⟹ 이동/해제 금지).

**C. ★GameSetting은 시뮬마다 복제된다**(신규 구조 사실, 확신 A — 로그 실측): `patched` 카운터가 계속 증가 ⟹ **"한 번 고치고 끝"이 아님**, need_exp를 읽는 경로마다 매번 잡아야 한다. 관측 인스턴스 2종 = `orig(len=11 cap=11)`(원본에서 새로 생성) / `orig(len=17 cap=17)`(**모드가 고친 인스턴스의 `Vec::clone`** — clone은 len에 맞춰 재할당하므로 cap==len·내용은 모드 값). 둘 다 ptr이 달라 훅이 재차 통일시킨다 ⟹ **에셋(merge) 단계 1회 수정으로 전 경로를 덮을 수 없음**.

**D. ⚠실사고 정본 — "테이블 2벌 공존 시 경험치 바 폭주"**: 바 = `잔여exp / need_exp[level−1]`, 계산 `0x803b30`·비율 저장 `0x80b723`·노드 `champion_tooltip.exp.bar` width = **Percent(ratio×100)** @`0x4f5ab7`·**클리핑 없음** ⟹ ratio>1이면 화면 밖까지 뻗는다. 사고 = 진단용 merge 데이터만 `[10×17]`인데 DLL은 `[150…4200]` 주입 ⟹ 분모 10 / 분자 150 스케일 → 레벨1에서 149/10 = **ratio 14.9 → width 1490% ≈ 1907px** = "레벨 1부터 화면 벗어남"의 정체. 항구 대책 = **테이블을 cfg 한 곳에만 두고 ptr 기준 전 경로 강제**(merge 데이터 제거·`mod.override_info={}`). ⚠**오진 정정**: 세션 중반의 "merge는 시뮬에 미반영"은 **틀림** — merge는 정상 적용되며(UI 인스턴스가 merge본을 봄) len=11 인스턴스가 공존할 뿐.

**F. 사용법·운용**: cfg 1줄 = 최대 레벨(2~500) / 2줄 = **레벨 12→13부터**의 필요 경험치 CSV(레벨 12까지는 코드 내 `VANILLA` 11값 고정). 필요 개수 = 최대레벨−12(초과분 절삭·부족하면 **마지막 값으로 채움**), cfg 없으면 기본 생성·파싱 실패 시 기본값 폴백+로그. 진단 로그 = `mods\tfm2_level_cap\tfm2_level_cap.txt`(`[cfg]` 해석 결과·`[hook:levelup]`/`[hook:expbar]`·5초 주기 `orig(len/cap)`·calls·patched). ★**함정**: 로컬 모드는 폴더에 넣는 것만으로 활성화되지 않는다(게임 내 모드 메뉴에서 ON 필요) + **`mod.mod_info`의 version을 올리면 활성 목록에서 빠질 수 있다**(1.0.0→2.0.0에서 실제 발생 = 세 번의 "안 됨"의 진짜 원인). 활성 확인 = log.log의 `Save mod session differs from active mods:` 줄 `active=[...]`.

**G. ⬜미확인(사실 승격 금지)**: len=11 인스턴스의 생성 지점(백그라운드 sim 경로 추정) / `asset/base/setting/single_lane/game_setting`·`death_match/game_setting` **문자열이 exe에 없음** ⟹ 해당 override 항목은 no-op이었을 가능성(확신 B) / 레벨업 루프 재진입 `0x22d424c` 미후킹(같은 인스턴스라 무해).

---

## §7.2-A13 · ✅**disc14 RNG 홀 = 수정 완료**(§7.2-A10 §4 잔여 해소) — `my_defense_nexus_050` rng/live 배선 (2026-07-23, 0.5.2 buildid 24310934, dll `F0BD6F6C`) — **본 절 = 이 건의 정본**

> ⚠절 번호: 원래 "§7.2-A12"로 예정했으나 **A12는 레벨 상한 건이 이미 점유** ⟹ 본 건 = **A13**.

### 0. 배포 실측 (근거)
- dll **3,472,384B · md5[:8] `F0BD6F6C`**(직전 `FB52A3AE`) · 배포 **2026-07-23 20:13:02** · 모드폴더 dll **1개** 확인 · 롤백 백업 `tfm2_ai_adjust.dll.bak_pre_d14rng`.
- 빌드 = rustc 직접(`-C opt-level=1 -C overflow-checks=off`, nightly-2026-05-24, `sdk_052`) **exit 0**. ~~⬜**인게임 미검증**~~ → ✅**라이브 생존 검증완(0.5.2, 07-23 22:23 유저 인게임, 후속 dll `08CC1D7F` 런: crash_log·panic_log 미생성·itemnet_guard 22:25 갱신) / ⬜단 disc14 RNG desync 축은 "적용 확인"으로 못 잡음 = 플레이 누적 관찰 잔여(§4)**.

### 1. 결함 (발견 = §7.2-A10 §4)
- `serpen.rs` `my_defense_nexus_050`(disc14 재현) 내 picker 호출이 **`serpen_rng_pick(0, sim, plan, false)`** = `rng=0`·`live=false` ⟹ draw 미재현.
- 0.5.2 원본은 그 지점에서 picker `0x2135350`을 **무조건 호출**하고 후보 `n>0`이면 Lemire 루프로 전역 RNG를 **실제 소비** ⟹ disc14 대체 시 그 draw가 통째 누락 = **RNG 스트림이 게임보다 뒤처짐 = desync**.
- ⚠disc14는 이미 `MP_SAFE_DISC` 편입·라이브였고 **code 대조 400/400**(§7.2-A8)을 받았으나 **RNG 축은 그 검증이 못 잡는다**(code만 비교).

### 2. 수정 내용 (disc12 `my_serpen_battle` live 배선 패턴 준용)
1. **시그니처 확장**: `my_defense_nexus_050(out, cmd, level, sim, geom, tp, sf)` → **`(out, cmd, level, rng, sim, geom, tp, sf, live)`**.
2. **picker 사이트**(`serpen.rs`, 구 RNG 홀 주석 자리): `serpen_rng_pick(0, …, false)` → **`serpen_rng_pick(rng, …, live)`**. ★호출 **위치는 원본대로 "무조건 호출" 지점 그대로 유지**(뒤의 "T 풀피" 게이트보다 앞) — 옮기면 draw 발생 조건이 게임과 갈린다.
3. **대체 경로 = `live=true` 유일 지점**(`tfm2_ai_adjust.rs` mp_repl disc14 arm): 그 arm이 이미 잡아두고 안 쓰던 **`let _r9 = rd_u64(saved + 0x10)`**(= P4 = RNG state)를 **`r9`로 승격**해 전달.
4. **검증(리턴훅) 경로 = `live=false` 고정**(`my_movepriority` disc14 arm): rng(`r9`)은 넘기되 live 게이트가 소비를 막는다. ⚠**여기서 true면 게임이 이미 소비한 draw를 한 번 더 소비 = 이중 소비 desync.**

### 3. ★이중 소비 안전성 근거 (배선 전에 확인 — 본 수정의 핵심 논거)
- `my_defense_nexus_050` 전 구간에서 **`-99` passthrough는 진입부 `ptr_ok` 가드 5곳뿐이고 전부 draw 지점보다 앞**. draw 이후 구간엔 `-99` **0건**(실측 grep) ⟹ "우리가 draw → `-99` 반환 → 게임이 재실행하며 또 draw"가 **구조적으로 불가**.
- ★대조: disc12 `my_serpen_battle`은 target-null 경로가 draw보다 **뒤**에 있어 그 경로에 **별도 draw를 삽입**해야 했으나, disc14는 위치 이동 없이 **인자만 교체**하면 됐다.
- ⟹ ★**일반화 규칙: live(RNG 소비) 배선 전에는 반드시 "draw 이후에 passthrough 반환 경로가 있는가"를 먼저 확인**할 것. 있으면 그 경로에 draw 보정 삽입, 없으면 인자 교체만으로 충분.

### 4. ⬜미해결 (사실 승격 금지)
- ~~⬜**인게임 검증**: 첫 지표 = 무크래시~~ → ✅무크래시 확인(07-23 22:23, §0) / ⬜유지 = disc14 대체 발화 카운트(직전 회차 **437회**급)·`pokerng.txt`/`mpcmp.txt` 대조는 **log=1 켜야 관측 가능** + **RNG desync 축 = 플레이 누적 관찰 필요**(적용 확인으론 못 잡음).
- ⬜picker `n` 산출의 **draw 수** 완전 일치는 07-23 disc12 감사분에 의존 = **재검증 안 됨**(본 수정은 "draw를 하느냐"만 해결).
- ⬜`MEM\DONE.md` 미등재(파일 포화 15,358B/15,360B) ⟹ **`/dream` 롤오버 후 등재 대상** = 잔여트래커 #0c.

## §7.2-A14 · ★★subplan별 레버 **전수 감사** + 라이브 결함 2건(`pos_enter_p56`·stat seed stale) 수정 + **`gb_*` 부활(10사이트 재핀·`gb_join_phase` 死)** + **신규 공유 위협 severity 레버 `sv_*` 신설(4사본 29사이트)** (2026-07-23, 0.5.2 buildid 24310934, dll `08CC1D7F`) — **본 절 = 이 건의 정본**

### 0. 배포 실측 (근거)
- dll **3,475,968B · md5[:8] `08CC1D7F`**(직전 3,472,384B `F0BD6F6C`) · 배포 21:31 · 모드폴더 dll 1개 · 롤백백업 `*.bak_pre_levers`.
- 설정편집기 **8,382,464B** 21:33 배포(severity 신규 탭 "[공통] 위협 민감도(severity)" + `gb_join_phase` ⛔DEAD 반영).
- ✅**byte-patch 전수 바이트 검증 = 62/62 OK**(소스 전 사이트 vs 0.5.2 exe 실바이트: disc19 10 + objective 12 + vis 1 + gb 10 + **sv 신규 29**). ~~⬜**인게임 미검증(전체)** = §7~~ → ✅**적용 검증완(0.5.2, 07-23 22:23~22:25 유저 인게임): d19 10/10·oi 12/12·`gb_imm 10/10`(gb_enable=false·join_ph DEAD 표기 정상)·`sev_imm 29/29`(sv_enable=**false**=원본값 복원 write 경로 = 29사이트 prefix 일치·.text write 성공 = **사이트 무결성 라이브 실증**)·vis 1/1·크래시 0 / ⬜잔여 2건 = §7**.

### 1. ★라이브 결함 2건 발견·수정 (감사 1부)
1. **`pos_enter_p56` stale**(`tfm2_ai_adjust.rs` 구 L558-559): 포지션/클래스/선수별 오버라이드 컨텍스트의 **단일 진입점**이 `p5+0x6a8/0x6a0`(0.4.x 잔재) → **`+0x820/+0x818`** 수정. ★**"한 함수만 마이그 누락" 4번째 사례**(apply_numbers_sp·my_f22e80_count·O_ATHLETE_ID에 이어). 영향: ①dd7700(disc0/1/3)·engage judge의 `[pos]`/클래스/선수별 오버라이드가 쓰레기 핸들 → champ 해석 실패 → **전역값으로 조용히 폴백**(조용한 미적용) ②`pos_record()` 불발 → `POS_MAP` 미충전 → **recall(`rc_*`) 포지션 조회 연쇄 무력화**. 전역 레버는 tune 폴백 덕에 정상이었음. 구조체 동일성 근거 = 같은 p5의 engage 콜리 `my_e9a30_count`가 이미 `+0x888`(+0x178 델타) 사용.
2. **스탯 노이즈 seed stale**(구 L5617): `stat_influence=25` 라이브 상태에서 판단력 노이즈의 챔피언별 seed가 `p5+0x6a0` 쓰레기 → **`0x818`** 수정(챔피언별 탈상관 복구).
- ★**일반화 규칙 재확인: 오프셋 정정 시 동일 의미 read를 소스 전역 grep**(§7.2-A10 §3의 규칙이 또 적중 — `0x6a0|0x6a8` grep으로 두 건 발견).

### 2. skip_untuned 그룹 배선 버그 2건 수정
- `skip_untuned=1` 활성 상태에서 그룹 목록(`tfm2_ai_adjust.rs` `mp_misc_t`)이 낡음: **`ec_tgt_hp_low`(死)→`ec_self_hp_low` 교체 + `disc16_home_hp` 누락 보충**. 방치 시 "신 키만 튜닝하면 MP_REPL이 통째 꺼져 값이 무시"되는 잠복버그.
- `config/default.txt`에 `ec_self_hp_low=20` baseline 추가 + 배포 cfg 구 키 주석화·신 키 이행(21≡20, 행동 불변). 둘 다 BOM 없음 확인.
- ★**규칙: 키 개명·신설 시 3점 세트 동기 필수 = ①skip_untuned 그룹 ②default.txt baseline ③편집기**(이번에 2점이 빠져 있었음).

### 3. 레버 전수 감사 결과표 (감사 방법 = tune() 키 152개 소비지점 추적 + 게이트 실측)
- ✅**라이브 확인**: disc0/1/3 `dd_*` 12종+`aggr_lane` / disc12·14·16·17 `ec_*`·`ep_*`·`disc16/17_*`(★**`ec_count_radius`도 실소비 확인** — `tfm2-ai-adjust-2-redesign` 메모리의 "개입 불가"는 **byte-patch 방식(_2 모드) 한정**, 본 모드 재현 경로에선 유효 레버) / recall `rc_*` 26종 / engage `eng_role*`·`t_engage` / byte-patch 33종(imm 재핀분).
- ⏸**휴면(게이트 OFF)**: disc4 `d4_*` 7종(**`d4_repl=0` 격리** — 문서가 "기본 작동"으로 잘못 기재돼 있어 정정) / disc7(가) `d7_*` 3종(`d7_repl=0`, 의도된 기본).
- ⛔**死 확인**: `d8_slot_thr`(도달 불가) / `poke_phase_gate`·`poke_active_min`(소스에서 `_` 처리 확인) / `dd_early/cover_p3_thr`(read 사이트 0).
- **문서 정정 3곳**(`SUBPLAN_동작_전수조사.md`, 동기완): 플랜4 헤더 "기본 작동"→"⏸d4_repl=0 격리 중" / 플랜9 phase 게이트 삭제 반영 / 플랜12 "~~반영 안 됨~~→✅편입완(다이버 재구축 갈래만 게임 위임)".

### 4. ✅`gb_*` 레버 부활 — GenericBuild 12→**10사이트 재핀** (`detour.rs` `apply_gb_imm`)
- **컨테이너 4종(pdata 검증)**: 본체 **`0x22b2280`**(0.5.1 `0x1e1ebb0`↔) / 거점헬퍼 **`0x2398240`** / reach 공유헬퍼 **`0x23ad980`** / reach#2 **`0x23ba8d0`**.
- **확정 10사이트**: `gb_close_radius=0x22b2555`(`48 c7 44 24 40`, 0x249f0) / `gb_line_range=0x22b2ca5`(`48 c7 85 b0 01 00 00` — disp 0x180→**0x1b0**, 0x3d090) / `gb_join_dist=0x22b2bb1`(**`41 b8`**=mov r8d — 인코딩 변경, 구 `49 01 c9 b9`, off 4→2, 0xd693a401) / `gb_push_hp=0x22b58ad`(`48 83 f8`, 0x1e) / `gb_op_phase=0x2398342`(**`49 83 be` b8 00 00 00** — rbx→r14, 0x1f) / `gb_scout#1=0x2398ef3`(`48 c7 45 18 00..` — disp 0x10→**0x18**, 0x35a4e9001) / `gb_scout#2=0x2398f3c`(**`4c 8b ad 80 00 00 00` `49 b9`** — off 9, 동) / `gb_reach_cap#1=0x23ad9d7`(`48 b8`, 0x490404400) / `#2=0x23ba8f3`(`49 ba`, +1) / `gb_reach_margin=0x22b43ae`(`41 b8`, 0x61a8).
- ⛔**`gb_join_phase` 2사이트(구 0x1e1f4ea/0x1e1fa74) = 0.5.2 삭제 = 死레버**: 본체 전 영역 `cmp r/m,0xc` 전 인코딩 스캔 0건 — **합류는 이제 phase 무관·거리만으로** 갈림. ⟹ ★**0.5.2 "phase 게이트 광범위 삭제" 경향의 3번째 확인**(disc19 4곳·disc1 p3에 이어).
- ⚠`gb_scout(0x35a4e9001)`은 objective `dn_near_dist`와 **같은 값·다른 함수** — 혼동 금지(pdata 소속 재확인됨).
- 로그 `gb_imm.txt` 포맷 `applied=N/10` 갱신. 편집기·`SUBPLAN_동작_전수조사.md` 부록 B 동기완.

### 5. ★★신규 레버 신설 — 공유 위협 severity 사다리 `sv_*` (`detour.rs` `apply_sev_imm` 신설, 4사본 29사이트)
ghidra-re가 disc19 사다리와 동형인 **인라인 사본 5곳의 소속을 전부 규명**(0.5.2):
- **[A] `0x22dd9a0` = 위협 평가 정본 본체** — TLS per-tick 메모이즈 래퍼 **`0x22dd690`**(정본 API) 경유로 **전 핸들러 ~60콜사이트가 공유**. 사다리 7사이트(`0x22e3cdf`~`0x22e3d16`) + **할인 레버 3**('사소' 위협 = `min(cap, threat>>shift)` 축소: shift@`0x22e3d2b`(`48 c1 f8`, orig 2)·cap cmp@`0x22e3d2f`·cap mov@`0x22e3d33`(`bb`, imm_off 1, w4, orig 0x12)). **최상급 레버**.
- **[B] `0x22e6460`** = 드라이버B(JT2 계열=넥서스 공방 포함) 디스패치 직전 공통 위협 컨텍스트 빌더 — 축약 사다리 5사이트(`0x22edb5f`~`0x22edb7b`, hp는 r8=`49 83 f8`).
- **[C] `0x22efed0`** = 위협 유의성 필터 leaf(disc0/1/3·disc4 경로) — 사다리 7(`0x22effff`~`0x22f0023`). branch A(소극 경로 별도 4임계 @`0x22f0067~79`) = ~~⬜미배선 후보~~ → ✅**0.5.3서 배선완(2026-08-03, `sv_pa_*` = §7.3 §12.15)**.
- **[E] `0x23a04d0`** = 공유 후보-스코어링 평가자(엔진 `0x239a4e0`의 하위, 얇은 래퍼 15종 경유로 JT2 다수에서 호출) — 사다리 7(`0x23a0c21`~`0x23a0c47`, **tr3만 jb 인코딩=+1**).
- **[D] `0x22f8a90`** = disc5/6 후퇴판정 leaf(JT3 s3 `0x2295760`·s4 `0x236cb90`·GenericBuild 콜러) — ⬜**미배선**(라이브 발화 미확정·트레일러 매핑 신뢰도 중). 전표는 에이전트 결과에 보존.
- **배선**: 신규 cfg 키 10종 = `sv_enable`(0=원본) / `sv_tr0~3`(49/29/17/9 — ↓=겁쟁이·↑=대담) / `sv_hp1~3`(65/40/25) / `sv_discount_shift`(2)·`sv_discount_cap`(18). ★**4사본 29사이트 일괄 동일값 패치**(사본별 개별화 금지 = 판단 일관성 — disc19 사다리만 기존 `d19_sev_*`로 별도). 로그 = `sev_imm.txt` `applied=N/29`. `SEVIMM_SIG` static + `apply_sev_imm()` 호출 배선(apply_gb_imm 옆). `default.txt` baseline 10키 추가. 편집기 신규 탭 "[공통] 위협 민감도(severity)".
- ★**의미(디컴 확정)**: `tr = threat*100/hp_cur` — 사다리는 "이 위협이 유의미한가" 필터. 통과(사소)시 [A]에선 1/4 할인+상한 18, [B]에선 누산 스킵, [C]에선 0 반환, [E]에선 스코어 페널티 무시.

### 6. ★부가 신규 발견 (구조 사실 — discovered-* 누적은 game-atlas 후속)
- **think 래퍼 `0x211b520` → 드라이버A `0x20d6e50`**(JT3 `0x2133ab0`×3 + JT1 movepri `0x2134240`×3) → **드라이버B `0x20ec050`**(전처리 `0x22e6460` 1회 → JT2 `0x1dabcc0`). **JT2·JT3 = 이번 신규 발견 facet 디스패처**.
- 좌석표 3벌 전문 확보(에이전트 결과 보존): JT1(s10=disc12 `0x238f130`·s11=disc13·s13=disc15·s15=`0x1b92e40`) / JT2(s15=`0x2376320`(an)·s16=`0x2380820`(disc19)) / JT3(s1 `0x1b8d710`=disc0/1/3·s2 `0x1b88220`=disc4 등).
- ⚠**disc 번호 +1 어긋남 미스터리 구체화**: JT1 앵커는 `idx=disc-2` 정합인데 **JT2 앵커(an=s15·disc19=s16)는 `idx=disc-3`으로 계통적 +1** — 기존 disc18/19 명명의 enum 기준 재확인 필요(런타임 확인 권장). ⟹ ⬜미해결.
- 위협량 계산 형제 함수 = `0x22ef780`·`0x22ef2f0`. engage_gate `0x2117ae0`의 0.5.2 콜러 = JT1 s12 `0x2118ef0`·s10 disc12.

### 7. ⬜잔여 (사실 승격 금지)
- ~~⬜**인게임 검증(전체)**~~ → ✅**통과(0.5.2, 07-23 22:23~22:25)**: sev 29/29·gb 10/10·d19 10/10·oi 12/12·vis 1/1·크래시 0. ⚠예측 정정: enable=false에서도 **원본값 복원 write 경로가 applied로 집계**됨(0/29 아닌 29/29가 정상) / ⬜유지 2건 = ①단시간 런이라 풀매치 완주 미단정·심층지표(disc14 발화 카운트 등)는 log=1 필요 ②disc14 RNG desync 축 = 누적 관찰(A13 §4).
- ⬜sv 미배선 후보 2건: [D] disc5/6 leaf(발화 확인 선행) · ~~[C] branch A 4임계~~ → ✅**0.5.3서 배선완(2026-08-03, `sv_pa_*` 4키·apply_sev_imm 29→33 = §7.3 §12.15)**.
- ⬜disc 번호 정본 재확인(JT2 +1 미스터리, §6) / ⬜disc4 재검증→`d4_repl=1` 복귀 검토(레버 7종 부활 효과) / ~~⬜개별 120틱 단기 시야창 8곳(미착수 아이디어)~~ → ✅**0.5.3서 배선완(2026-08-03, `vw_*` 25사이트 = §7.3 §12.16 — ★구 "8곳" 카운트는 sev[B] 스킬타이머 게이트 8곳 오인 가능성 = 정정)**.
- ⬜`MEM\DONE.md` 포화로 본 절 판정 미등재 ⟹ **롤오버 후 등재 대상** = 잔여트래커 #0c.

## §7.2-B · `tfm2_banpick_illust` v1.2.0 쇼케이스 모듈(구 `tfm2_banpick_showcase`) — RVA 등재 (2026-07-25, 0.5.2 buildid 24310934, ✅인게임 검증완→통합판 ⬜재검증 대기)

> ~~**비-T1 신규 모드(MOD_REGISTRY T3)**~~ → ★**07-25 `tfm2_banpick_illust` v1.2.0에 통합·별도 모드 철수(유저 지시) = 본 절은 banpick_illust(T1) 소속 ⇒ 패치 시 자동 마이그 대상.** RVA 상수부 = `C:\tfm2mods\tfm2_banpick_illust\src\showcase.rs`. — 밴픽 쇼케이스 연출 전체 대체(가로형 520 커스텀 카드 + banpick_illust 아트팩 일러, 아트 해석=keys::illust_key 재사용). 최종 dll ~~179,712B~~ → ~~182,272B~~ → **244,224B(illust v1.2.0 통합 dll, 07-25 배포·⬜인게임 재검증=유저 확인 대기)**. **아래 전부 0.5.2 기준 = 패치 시 마이그 대상.** 상세 시그니처·cmd 필드맵·기하 패치 테이블 전문 = **`C:\tfm2mods\tfm2_banpick_showcase\FFI_CONTRACT.md`(정본 — 소스 폴더는 이 계약 보존용 유지)** / 모드 정본 = `MEM\tfm2-banpick-showcase.md` §7~10(RE) + `MEM\tfm2-banpick-illust-mod.md` §쇼케이스 통합.

- **훅 3**(대체형 detour, 진입 12B/19B 패치+원본 트램폴린, 외부훅 감지 시 설치 포기): `0x11e2370`(진영·모드 스태시) / `0x11f9030`(카드 드로우 전체 대체 — 콜사이트 3곳 `0x124e390`·`0x124e4d1`·`0x124f3bf`) / `0xfdabe0`(일러 에셋 조회 리다이렉트 — 크기 게이트 제외 참조사이트 `0x1220a70`·`0x124f45a`).
- **FFI 함수 14종**(07-25 idle 폴백분 +2): `0x248b1c0` / `0x248b400` / `0x248c130` / `0x248c7c0` / `0x248cd40` / `0xff0c20` / `0x248e850` / `0x248c1e0` / `0x1217630` / `0x99c860` / `0x8b7f80` / `0x8b7f90` / `0x5ab7d0`(키→애님 리소스) / `0x121aca0`(폴백 시트키·UV·크기 순수 계산기).
- **기하 패치 12사이트**(밴 분할 360×480→520×408): A=배타 rdata 6(`0x3731380`·`0x37313b0`·`0x37313c0`·`0x37313e0`·`0x37313f0`·`0x3731400`) + B=코드 즉치 `0x124e2ba` + C=공유상수 disp 재타깃 4(.rdata 패딩 `0x3fd2b00` 슬롯) — 사이트 좌표·바이트 = FFI_CONTRACT.md §기하패치. **12/12 사전검증 통과 시에만 적용**(실패=밴 360 폴백). ⚠`0x1436e8e98`(0.82)=0x11f9030 폴백 일러 스케일과 공유 = **패치 금지**.

## §7.2-C · `tfm2_banpick_order` — 밴픽 진행 순서 cfg 재정의 훅 RVA 등재 (~~2026-07-26 v1.0.0·훅3·⬜인게임 미검증~~ → ~~07-29 훅5·인터리브 검증완~~ → **2026-07-29 v1.0.0 릴리스: 훅 7종·인터리브+팀순서+배너+밴픽AI 검증완**, 0.5.2 buildid 24310934)

> **비-T1 신규 모드(MOD_REGISTRY T3, transfer_tweak과 동급)** — 마이그 8종 대상 아님, 단 **하드코딩 RVA ~~3종~~ → ~~5종~~ → 7종(훅 6 + 바이트패치 2사이트) = 패치 시 전부 재핀 필요(본 절 = 재핀 대상 정본)**. 밴/픽 턴 시퀀스를 cfg로 재정의 — 밴↔픽 인터리브 자유 배치 **+ 팀 순서 지정**. 소스 = `C:\tfm2mods\tfm2_banpick_order\src\{lib.rs, config.rs, hooks.rs, draft_ai.rs}` · dll ~~162,816B~~ → ~~186,880B~~ → **2,671,104B**(v1.0.0 = `ChampionInfoSheet` 정적링크) 배포완(게임 mods\tfm2_banpick_order\ + mod.mod_info BOM無). ⚠★**빌드 = `build_inj.ps1` 사이즈가드(1.3MB) 초과 ⟹ rustc 직접 + 수동 신원검증 + `Copy-Item`**(banpick_illust·ai_adjust와 동일 예외·`-C opt-level=1 -C overflow-checks=off` 필수). RE 정본 = `ANA\discovered-banpick-ai.md` §16 + **§17i·§17j(최종)** / 모드 정본 = `MEM\tfm2-banpick-order-mod.md` **§11**.
> ★**검증(07-29 인게임·유저 확인 "잘된다")**: 인터리브 + 팀 순서 지정 + 경기 진행 전부 정상 · 카운터 `applier=20` **`lineup_skip=0`** · PANIC 0건. (중간 이력: `seq_5v5_ban5 = B1 B2 B1 B2 B1 B2 P1 P2 P2 P1 P1 P2 B2 B1 B2 B1 P2 P1 P1 P2` → `applier=20 forced_pick=6 lineup_skip=1` · 권위 `AUTH total=20 vec=05.05.05.05`.)
> ★**릴리스** = `<게임설치>\mods\release\0.5.2\tfm2_banpick_order.zip` **843,291B**(루트 `tfm2_banpick_order\` 한 겹·4파일: dll·mod.mod_info(v1.0.0)·cfg(debug=0)·README.md) · deploy-verify **7항목 PASS**(수정 2건 = mod_info description `\t` 이스케이프 깨짐→슬래시 표기 / 템플릿 키명 `_ban3`인데 B1 5개→**`_ban5`**) · 조사용 **int3(0xCC) 게임코드 패치 제거**·진단 전량 debug 게이트화.

- **훅 지점 (0.5.2 RVA — 아래 전부 패치 시 재핀 대상)**:

| RVA (0.5.2) | 대상 | 방식 | 프롤로그 (재핀 시그·기본 12B) |
|---|---|---|---|
| **A** `0x1cd9380` | current_banpick_phase(&MatchSetInfo)→u8 | **전체 대체**(커스텀 seq 순수함수 — 바닐라 로직+픽테이블 자체 보유, 트램폴린 없음) · ~~MY_MSI 매치 한정 게이트~~ → ⚠**게이트 제거(v1.0.0)** = 전 경기 적용(**포인터 동일성 게이트 금지** — AI턴 레코드 스택 clone에 오판→phase만 바닐라 폴백→커밋과 어긋나 **내 픽↔상대 픽 꼬임** 실사고) | `48 8B 51 58 48 03 51 40 48 03 51 70` |
| **B** `0x1d04120` | phase_from(total,rule,ban)→u8 | **전체 대체**(진입 7B째 **rip-rel lea** 점프테이블 = 트램폴린 불가 → 전체 대체 전용) · 게이트 없음(위와 동일) | `4D 01 C0 0F B6 C2 48 8D 15 83 6D B3` |
| **C** `0x11e2140` | 클라 셀렉트 확정 적용기 | **트램폴린 detour + shim** — ban_count(`+0x3c0`)·t2밴len(`+0x160`) **일시 조작으로 밴/픽 type 유도** → 원본 호출 → **Drop 가드 원복**. **완료 유발 마지막 픽은 orig 위임**(원본이 `@0x11e2328` `jmp 0x11d8ef0` tail-jmp transition — 직접 `call` 금지, §17i-0 tail-call 규약) | `41 57 41 56 41 55 41 54 56 57 55 53` |
| **D′** `0x1d07cf0` (v1.0.0에서 D를 대체) | 턴 오라클 `fn turn(RMI* rcx) -> (rax: 0/1, rdx: acting team_id)` = "지금 누구 차례냐" | ★**전체 대체**(2워드 반환 ABI라 **raw 스텁**으로 처리) ⟹ **팀 지배**(F와 조합해 팀 순서 지정 실현). ~~구 D = 진단용 13B 특수 트램폴린(진입 `jz`를 절대점프로 재배치)~~ = 이력 | `48 8B 41 10 48 85 C0 0F 84 <rel32>` (13B, 전체대체라 트램폴린 불요) |
| **E** `0x11cedb0` (신규 07-29·★크래시 회피 핵심) | `banpick_scene__apply_lineup`(서버 tag28 최종 라인업 적용) | **트램폴린 detour** — 라인업 이름이 대상 팀 픽벡터(**T1 ptr `+0x170`/len `+0x178`, T2 ptr `+0x188`/len `+0x190`**)에 **없으면 원본 스킵** → `match_ui.rs:4181` **unwrap(None) 패닉 회피**(레인 표시만 스테일, 진행·전환은 별개 이벤트라 정상) | `55 41 57 41 56 41 55 41 54 56 57 53` |
| **F** `0x1d075d0` (신규 v1.0.0) | 권위 커밋기 `bool banpick_commit(RMI* rcx, u64 acting_team rdx, String* r8)`(콜러 `0xd5f380`@`0xd60d18`) | **트램폴린 detour** — **ban_count 유도로 타입**, **side 일시조정으로 밴 팀**, 픽은 **k 역산** ⟹ 팀 순서 지정의 핵심. 추가로 **매 액션 후 레코드 4벡터(챔프 이름) 스냅샷**(모드 밴픽 AI 입력, `src\draft_ai.rs`) | 재핀 시 진입 12B 실측 대조 필수 |
| **AI 파리티 바이트패치 2사이트** (신규 v1.0.0) | 밴 AI 인라인 phase 복제 지점 | `0x1c04389` **35B**(합류 `0x1c04475` `cmp cl,2`) · `0x1c07938` **40B**(합류 `0x1c07a09` `cmp al,2`) → `movabs rax,<모드 ai_ban_phase>; call rax`. ⚠**인자 = 전체 진행 수가 아니라 밴 개수**(bans_done, rule, ban) | 패치 길이·합류점·orig 바이트 **실측 대조 필수**(로직 동일≠인코딩 동일) |

> ⚠**패치 대응 주의**: ①**7종(훅 6 + 바이트패치 2사이트) 전부 재핀 + 프롤로그 재검증 필수**(로직 동일≠인코딩 동일 — §7.2 규칙) ②~~D는 13B 특수 패치~~ → **D′는 전체 대체**(2워드 반환 raw 스텁)라 트램폴린 불요, 단 **반환 ABI(rax/rdx) 유지 확인** ③**panic 사이트 `0x11cf5de`(match_ui.rs:4181:69)와 `C:\tfm2mods\panicmap\panic_sites_0.5.2.csv`는 버전 종속 = 패치마다 재생성**(생성 스크립트 = `C:\tfm2mods\panicmap\`, .pdata 기반·Ghidra 불요). 크래시 재진단 표준절차(panic 훅 `0x25d4764`) = `MEM\tfm2-crash-diagnosis-panic-hook.md`.

- **cfg 계약**: 게임 `mods\tfm2_banpick_order\tfm2_banpick_order.cfg` — 토큰 `B1/B2/P1/P2`, 키 `seq_<룰>[_ban<n>]`(적용 밴 수 = 항상 B1 토큰 개수). 검증 = B1==B2 개수 · P1==P2==rule+2(룰별 팀당 픽 수) · **마지막 토큰=픽 필수**(픽 완료 마감 `0x11d8ef0`이 시퀀스 끝에서 발화해야 서버 0xFF 판정과 일치 — discovered-banpick-ai §16). 검증 실패 = 바닐라 폴백. **핵심 훅(A/B/C) 중 1개라도 설치 실패 = 커스텀 전체 비활성**(`CUSTOM_ACTIVE` 게이트 — A/B 훅은 바닐라 재현이라 켜져 있어도 무해).
- **알려진 한계(수용·재조사 불요)**: ①콜러 사전 SFX 선택(밴/픽 효과음) 불일치 가능 = 연출 한정 ②AI 스코어링 인라인 파리티 4사이트(`0x1c041c0` 등) 미보정 — 인터리브 시 AI deny/그리디 가치판단 왜곡 가능(시뮬 크래시 없음·품질 문제만) ③~~UI 턴 표시 클러스터(`0x2360eb0`/`0x2362500`) 미후킹~~ → **정정(07-26)**: 실제 하이라이트 = `0x11e2980`(match_ui)이 phase_from `0x1d04120` 단일 의존 = **훅 B로 이미 반영**(그 둘은 AI 전용) ④순서 변경 시 그 이전 설정의 다시보기와 비호환(시드 재시뮬) ⑤~~★팀 순서 임의 지정(레드n픽/블루n픽) = 구조상 불가~~ → ★**철회·가능으로 정정(v1.0.0, 07-29)**: 훅 **F(ban_count 유도) + D′(턴 오라클 전체대체)** 조합으로 실현·인게임 검증완, 팀 판정은 각 경기 자체 side 규약(**T1 = team[side^1]**) — §17i-2 ⑥훅 E 발동 시 해당 라인업 레인별 표시가 스테일(경기 진행엔 무영향, v1.0.0 실측은 `lineup_skip=0`) ⑦연출(카드 `0x11e2370`)은 **스킵** 유지(유저 선호·복원법 §17i-0 연출/배너 시그), 단 **단계 배너는 모드가 FSM 재발동**(`scene+0x380=0`(u64) + `+0x43e=1`, arm0가 그림 · 타이밍 = 카드연출 종료 `+0x348==-1` 후 · **경계 도달 즉시 `+0x43e=1`로 턴 잠금**해야 게임이 먼저 커밋 안 함 · 밴이 이번 커밋으로 다 차면 원본 배너와 중복이라 스킵 · 소프트락 타임아웃 600프레임) ⑧**자체 밴픽 AI**(cfg `ai_ban_context`/`ai_w_syn`/`ai_w_cnt`/`ai_cap`) = 공식 확장점 `ModDraftScoreHook` + 훅 F 스냅샷(네이티브 밴 AI는 픽 미참조 — §17j) · ⚠타 밴픽 AI 모드(`draft_winrate_penalty`·`tfm2_ai_banpick_probe`)와 **동시 사용 시 보정 중첩**.

---

## §7.3 · ★★게임 **0.5.3** 마이그레이션 — 패치 성격 · SDK · **전 모드 RVA 일괄 재탐색** (2026-07-29, 0.5.3 buildid **24451609**) — ⚠**RVA 표 본체 = `C:\tfm2mods\_MIGRATE_053.md`**(모드별 세션 인계 정본), 본 절은 경위·성격·규칙만

> ★**§8 사실1개=1파일**: 개별 RVA·모드별 표는 여기 복제하지 말 것. 데이터 = `_rva_final_053.json` · 카탈로그 = `_rva_catalog.md/json` · 지시서 = `_MIGRATE_053.md`. 현행 버전 사실 = `MEM\CURRENT.md`.

### 0. 버전 / 백업
| | 0.5.2 (OLD = 모드 소스 베이스) | 0.5.3 (NEW) |
|---|---|---|
| buildid | 24310934 | **24451609** |
| exe | 69,209,088B | **74,970,624B** |
| sha256[:16] | 40b55c1b819dff50 | **6afff2cdb6bfa98e** |
| 백업 | `…\tfm2\tfm2_0.5.2\` | `…\tfm2\tfm2_0.5.3\`(exe+bundle.game_data+TFM2ModUploader.exe+**bundle_unpacked\ 1.1GB 전량**+_manifest.json) |
| Ghidra MCP | `ghidra`(8080) | `ghidra_beta`(8081) |

⚠`tfm2_0.5.3_b\`(2026-06-27) = **구 베타 브랜치 백업**으로 정식 0.5.3과 **무관** — 혼동 금지.

### 1. ★패치 성격 = **전면 재컴파일**(0.5.1→0.5.2보다 큼)
- `.text` **44.0MB → 48.6MB (+4.84MB, +10.5%)** · 함수 수 **120,995 → 132,960 (+11,965)** (양쪽 `.pdata` RUNTIME_FUNCTION 실측).
- ⛔**`migrate_rva.py` 연속바이트 마스크시그 = 전멸**. 핵심 훅 6종(RETREAT/CONDGATE/FC59A0/MOVEPRI/GENERIC_BUILD/PREGATE)을 160B 마스크시그로 재탐색 → **전부 NONE**. 시그를 0x20까지 줄여야 겨우 다중매치. 프롤로그 12B(push8 `554157415641554154565753`)는 NEW `.text`에 **66,635회** 등장 = 신원값으로 사용 불가.
- **0.5.3 함수는 0.5.2 대비 대체로 크기 2~10% 증가**(코드 자체가 바뀜) ⟹ ★**함수내 오프셋이 보존되지 않는다.** mid-func 사이트(byte-patch imm·콜사이트)에 델타를 그대로 더하면 **안 됨** — 컨테이너 함수 안에서 원래 명령 패턴으로 재탐색해야 한다.
- **전역 단일 델타 없음**(예: CONDGATE = **−0x14de820**, 21MB 앞으로 이동). 링커가 **블록 단위로 재배치**했고 **국소 인접성만 보존**(CONDGATE↔MOVEPRI 쌍 간격 유지가 실증).

### 2. SDK / 빌드
- 클래식 SDK 0.5.3 = **`C:\tfm2mods\sdk_053\mod-sdk`**(GitHub 릴리스 `0.5.3.zip` 537,840,130B 추출·`base_version.txt`=0.5.3). ~~`build_inj.ps1` L29 `$SDK` 전환 필요~~ → ✅**전환 완료(2026-07-29)**: `build_inj.ps1` L31 `$SDK = "C:\tfm2mods\sdk_053\mod-sdk"`.
- **toolchain 무변경** = `nightly-2026-05-24`(rustc 1.98.0-nightly 23a3312d9) — 재설치 불필요.
- ★**게임 rlib 236개 전원 내용 DIFF** ⟹ **RVA 0 모드 포함 전 모드 재빌드 필수**(0.5.2 때와 동일 규칙).
- ★**`libgame_ai` 크레이트 신설**(0.5.3에만 존재). `game_core` 407MB→**333MB**, `game_view` 346MB→**315MB** = **AI가 game_core에서 분리**됨 ⟹ AI 계열 함수는 위치뿐 아니라 **코드가 바뀐 것**으로 보고 접근할 것.
- `mod_api` rmeta에 심볼 **`ModAiSmallActionExt`** 신규 등장(~~0.5.2 "API 표면 변화 없음"~~ → 0.5.3은 **표면 증가**).
- ⚠빌드 플래그는 rustc 명령줄에 직접 `-C opt-level=1 -C overflow-checks=off`(불변 규칙).
- ★★**0.5.3부터 링커 = `rust-lld` 필수**(MSVC 2019 link.exe는 LNK1107로 죽음) — 전문 = **§9**.

### 3. ★방식 전환 + 도구 4종(다음 패치에도 재사용)
- 유저 지시(07-29): ~~모드별 세션이 각자 주소찾기~~ → **전 모드 RVA를 한 번에 조사해 리스트화 → 각 세션은 그 표를 보고 마이그**.
- 파이프라인(전부 `C:\tfm2mods\`): `rva_catalog.py`(전 모드 소스에서 RVA 자동 수집 — const/patch_site/array/inline 4계층) → `fnindex.py`(`.pdata` 함수경계 + 명령 스켈레톤 md5 + 니모닉 빈도 인덱스, exe당 ~40초) → `match2_053.py`(스켈레톤 완전일치 → head 일치 → 니모닉 코사인 + 국소 앵커 투표) → `match3_053.py`(쌍둥이 함수 순서대응 + 지시서 생성). 부가 = `…\tfm2\bundle_extract_all.py`(bundle 전체 언팩기).
- **산출물**: ★지시서 **`_MIGRATE_053.md`** / 데이터 `_rva_final_053.json` / 카탈로그 `_rva_catalog.md`·`.json`.
- **결과**: 카탈로그 **432주소(고유 344)** → **함수시작 47/70 해결**(확정 20 · 유력 28 · 추정 6 · 미해결 28 — 모드중복 포함 집계) / ⬜**mid-func 231건 미해결**(§1 크기증가로 오프셋 이전 불가 — 컨테이너 내 명령패턴 재탐색 도구가 **별도로 필요**) / `.text` 밖 40건 = 데이터 테이블이거나 RVA 아닌 상수.
- **모드별 함수시작 해결**: ai_adjust 11/17 · banpick_illust 10/15 · elemental_serpen 9/14 · comptest_unlock 8/14 · banpick_order 10/10 · item_tactics 6/12 · **draft_overlay·level_cap = 보유 RVA가 전부 mid-func라 함수시작 0건**.

### 4. 확인된 핵심 AI 훅 이동 (0.5.2 → 0.5.3) — ⚠신뢰도 등급 준수
| 심볼 | 0.5.2 | 0.5.3 | 근거 | 등급 |
|---|---|---|---|---|
| CONDGATE | `0x21338d0` | **`0xc550b0`** | 앵커 22 + **Ghidra 디컴 구조 완전동일 교차검증** | ★**확정**(유일) |
| MOVEPRI | `0x2134240` | `0xc559e0` | 앵커 21 · CONDGATE와 간격 `0x930` 유지 | 유력 |
| RETREAT | `0x1b94670` | `0xe00350` | 니모닉 cos 0.9994 | 유력 |
| FC59A0(RECALL) | `0x1bdb3e0` | `0xe168d0` | cos 0.9981 | 유력 |
| GENERIC_BUILD | `0x22b2280` | `0xe06c10` | cos 0.9995 · 크기비 1.02 | 유력 |
| DISC18_HANDLER | `0x2376320` | `0xd94d00` | 매칭 | 유력 |
| DISC19_HANDLER | `0x2380820` | `0xdece30` | 매칭 | 유력 |
| ITEMNET_SCORER | `0x1b9cce0` | `0x10587e0` | 매칭 | 유력 |

⛔**CONDGATE 외에는 "유력"이지 확정이 아니다 — 사실 승격 금지.** 훅 설치 전 **프롤로그·orig_len 경계·rip-rel 실측 필수**(§7.2 규칙 "로직 동일 ≠ 인코딩 동일"). 신원검증 실패 시 **미설치=inert가 안전**.

### 5. 대응 제외 2종 (유저 지시 2026-07-29)
- `tfm2_fog_damage_fix` = **게임측에서 수정됐다고 하여 0.5.3 대응 제외** — 전 모드 마이그 완료 후 **인게임 확인만**.
- `tfm2_transfer_tweak` = **불필요 판정으로 제외**(이적 협상 문턱 완화 편의 모드·비-T1·인게임 검증도 안 된 상태). 카탈로그·매칭 대상에서 제외됨.

### 6. ★도구 설계 교훈 (다음 패치에 그대로 적용)
- ⛔**앵커(국소 인접) 지지에 가산점을 주면 안 된다** — cos **0.9995 정답**이 cos 0.9969 오답에 밀리는 사고가 실제로 발생(GENERIC_BUILD). 해법 = **유사도 우선 + 앵커/크기비는 동점 타이브레이커로 강등**.
- ⛔**앵커쌍(스켈레톤 완전일치)의 크기비 분포로 "기대 크기비"를 구하려는 시도 = 순환 논리라 무효**(정의상 100% 일치 표본).

### 7. stable mod API (별건·포팅 보류)
- 공식 저장소가 2026-07-23 커밋으로 **클래식 경로 sunset** 방침 — 게임 동봉 `<게임설치>\mod-sdk-stable\`(abi_level=1). ⛔**클래식 `mod-api` = deprecated, 게임 0.5까지만 지원(0.6부터 SDK 미배포)**. **유저 결정(07-29) = stable 포팅 보류, 0.5.3 마이그에만 집중.** 전문 = `MEM\tfm2-stable-mod-api.md`.

### 8. ⬜잔여 (사실 승격 금지)
- ⬜**mid-func 231건** 재탐색 도구 미작성 = 최대 잔여.
- ⬜**함수시작 미해결 28건** = ghidra-re 필요.
- ~~⬜모드별 실제 마이그·재빌드·배포 **전량 미착수**~~ → **정정(2026-07-29)**: **RVA 0 모드 16종 = 재빌드·배포 완료 + 릴리스 zip 5종 완료**(§10). ⬜**남은 것 = 하드코딩 RVA 보유 T1 모드**(ai_adjust·item_tactics·banpick_illust·draft_overlay·elemental_serpen 등) **마이그 미착수**(RVA 재핀 필요·`_MIGRATE_053.md` 표 기준) + 대시보드 `save_probe` 재빌드 미착수.
- ⬜**0.5.3 인게임 검증 = 여전히 0건**(재빌드 16종도 배포만 확인·실행 검증 미실시).

### 9. ★★0.5.3 SDK = **MSVC link.exe 링크 불가 → `rust-lld` 전환**(2026-07-29 확정·재시도 금지)
- 증상: MSVC 2019 `link.exe`(**14.29.30133**)로 링크 시 **`LNK1107: 파일이 잘못되었거나 손상되었습니다. 0x55E40에서 읽을 수 없습니다`** — `0x55E40`(=350,272)은 **`libmod_api-36f682f2648263b7.rlib`의 파일 길이 그 자체** = 링커가 **파일 끝 너머를 읽는다**. 동반 `LNK2019` 미해결 심볼 = `Rc<dyn Fn(&engine_core::ui::UIEvent)->bool>::drop_slow`(game_view 인스턴스).
- ★**rlib은 정상임을 실증**(파일 손상 재조사 금지): ar 구조 `end_ptr == len` 일치 · 1차/2차 심볼테이블의 멤버 오프셋(8440/244008/244240) 전부 파일 내 유효 · GitHub 릴리스 zip ↔ 추출본 **329/329 엔트리 크기 일치**.
- 발생 조건 = **미해결 심볼이 생겨 링커가 아카이브를 재스캔할 때만**. 심볼 0인 SDK template은 MSVC로도 링크 성공(rc=0) ⟹ "SDK 자체는 링크된다"는 관찰에 속지 말 것.
- ⛔무효 시도(재시도 금지): `-Z share-generics=off` · `-C opt-level=0` · `-C codegen-units=1` 전부 무효.
- ✅**해결 = 툴체인 동봉 `rust-lld`**: rustc 명령줄에 **`-C linker-flavor=lld-link -C linker=rust-lld`** 추가. 동일 소스가 그대로 링크됨(0.5.3 재빌드 **16모드 전원 성공**). VS2022는 이 머신에 미설치(대안 미검증).
- 반영처 = `C:\tfm2mods\build_inj.ps1`(L46~54 주석 포함) + 신설 `C:\tfm2mods\build_full.ps1`.

### 10. RVA 0 모드 **16종 재빌드·배포 + 릴리스 zip 5종**(2026-07-29) — ⬜인게임 검증 전량 미실시
- **전부 하드코딩 RVA 0 = 소스 무수정·순수 SDK 재빌드**. 게임 `mods\<id>\<id>.dll` 배포 존재·크기·시각 전수 확인.
- 목록(바이트): `tfm2_mod_order` 203,776 · `tfm2_mod_scroll_fix` 150,016 · `community_reaction_mod` 619,008 · `Spectator_Chat` 325,632 · `tfm2_meta_item_delegate` 254,464 · `tfm2_meta_champion_tiers` 229,888 · `tfm2_ai_banpick_probe` 251,904 · `coaching_staff_view_plus` 283,136 · `custom_tier_assignment` 2,707,968 · `facility_view_plus` 287,744 · `finance_view_plus` 147,456 · `legacy_save_patcher` 375,296 · `recruitment_view_plus` 332,288 · `roster_view_plus` 433,152 · `statistics_view_plus` 2,736,640 · `training_view_plus` 2,632,704.
- 소스 = daram2 뷰플러스 9종 `…\tfm2\tfm2-mods-main\<mod>\src\lib.rs` / gg native 3종 `C:\tfm2mods\TFM2.gg-upstream\native\<mod>\src\lib.rs`. `banpick_view_plus` = 종전 유저 지시대로 **SKIP**.
- ★신설 **`C:\tfm2mods\build_full.ps1`** = 게임 crate 전량(`mod_api`/`common`/`engine_core`/`engine_ui`/`engine_asset`/`game_core`/`game_view` + `serde_json`/`flate2`)을 `--extern`으로 주입. daram2 뷰플러스 계열은 `extern crate common/game_core`를 직접 써서 `-L dependency`만으론 **E0463**. 사이즈가드 = `-MaxSize` 파라미터(기본 5MB)라 2~3MB 모드(custom_tier/statistics/training) 겸용. 안전장치(exit code·stale mtime·소스경로 신원검증·게임 락)는 `build_inj.ps1`과 동일.
  - ⚠함정① deps에 `serde_json`/`flate2`가 **2벌씩** 있어 **게임이 실제 링크하는 해시로 고정** 필요 = `libserde_json-9ce4f0220edb6ae7.rlib` / `libflate2-76adaee9a71bfe42.rlib`(rustc 링커 인자 로그로 식별).
  - ⚠함정② PowerShell 배열 리터럴에서 `"a=" + (f x), "b=" + (f y)`는 `+`가 **배열 결합으로 파싱**돼 전 항목이 문자열 1개로 뭉친다(→ `--extern` 접두가 첫 항목에만 붙음). **각 항목을 괄호로 묶을 것.**
- ⚠★**신원검증 가드 오탐 2종 = 재조사 금지**: `facility_view_plus`·`tfm2_meta_champion_tiers`는 dll에 panic location 절대경로가 안 박혀(`lib.rs` 상대경로 + 자기 mod_id만 존재, `C:\Users\dev` 없음) `build_*.ps1`의 소스 절대경로 신원검증에 걸린다. **타 모드 문자열 부재 확인 후 수동 `Copy-Item` 배포**가 정답. 다음 패치에도 동일 현상 예상.
- **릴리스 zip 5종** = `<게임설치>\mods\release\0.5.3\` — `daram2_viewplus.zip` 9,468,526B · `community_reaction_mod.zip` 321,253B · `Spectator_Chat.zip` 160,468B · `tfm2_mod_order.zip` 103,459B · `팀파매gg모드3종.zip` 632,192B.
  - 방식 = **0.5.2 zip을 스테이징에 풀어 dll만 신규 산출물로 교체**(자산 누락 방지). 엔트리 수 0.5.2와 전수 일치(47/3/6/3/17) · `mod_info` BOM 불량 0(첫바이트 `7B`) · 한글 엔트리명 정상.
  - daram2 9종 `mod.mod_info`: version `0.5.2`→**`0.5.3`**, deps base `>=0.5.2,<0.6.0`→**`>=0.5.3,<0.6.0`**, `last_updated` 2026-07-29(**author=daram2 보존**). gg 세트 zip 루트 폴더 `팀파매gg_0.5.2`→**`팀파매gg_0.5.3`** 리네임.
  - ⚠★**신규 함정(zip 한글 깨짐)**: `[System.IO.Compression.ZipFile]::CreateFromDirectory(...)`에 **`entryNameEncoding` 인자를 명시하면 UTF-8 플래그(general purpose bit 11)를 세우지 않아 한글 파일명이 깨진다**(`README_설치안내.txt` → `README_?ㅼ튂?덈궡.txt`) ⟹ **인코딩 인자를 생략한 4-인자 오버로드**를 쓸 것. (추출 `ExtractToDirectory`는 UTF8 지정 무해.)
  - ⚠★**개인정보 유출 발견·수정**: gg 세트의 `tfm2_ai_banpick_probe\ai_champion_policy.tsv` · `tfm2_meta_champion_tiers\champion_tier_policy.tsv` 4행 주석에 유저 로컬 세이브 절대경로(`C:\Users\dev\AppData\Roaming\TeamSamoyed\TeamfightManager2\data\save_*.data`)가 박혀 있었음 → **0.5.3 zip에서 마스킹**(스테이징 사본만 수정·라이브/원본 미변경). ⬜**0.5.2 및 그 이전 릴리스 zip에는 그대로 잔존 = 후속 정리 필요**.
- ⚠**로드경로 주의(유지 기록)**: `community_reaction_mod`·`tfm2_meta_item_delegate`는 실 로드처가 **워크샵 content 폴더**라 게임 `mods\` 배포만으로는 적용되지 않는다.
- ⚠**환경 이슈(재발 시 1순위 의심)**: 유저 **안티바이러스가 빌드/압축 산출물과 PowerShell 자식 프로세스를 삭제**해 명령이 **출력 0줄 + exit 1**로 조용히 죽는 증상 반복(백신 종료로 해소). 별개로 **Claude Code 샌드박스는 게임 폴더(Program Files) 대상 `Remove-Item`/`File::Delete`를 차단** ⟹ zip은 로컬에 만들고 **`Copy-Item -Force`로 덮어쓰는** 우회 필요.

### 11. ★`tfm2_item_tactics` 0.5.3 마이그 **완료·배포**(2026-07-29, 0.5.3 buildid 24451609, dll ~~504,832B~~ → ~~535,040B~~ → ~~542,208B~~ → **535,040B**(★07-30 배포본 실측 = 아이콘+툴팁 최종·mtime 07-30 04:04. "542,208B"는 **오기/미배포본이므로 신뢰 금지**), mod_info **v2.5.0**) — **본 절 = 이 건의 정본** / ~~⬜인게임 미검증~~ → ✅**경기중 4번째 슬롯 아이콘(§11.5a) + 툴팁(§11.5e) 둘 다 인게임 검증완(2026-07-30, 유저 "잘된다"·"다 잘뜬다")** — ★**툴팁은 이름·티어·가격·스탯·효과설명까지 전부 정상**(게임 show 함수 통짜 호출로 전환) · ⬜나머지 기능(구매·is_live 등) 인게임 미검증

> T1 모드 중 0.5.3 RVA 재핀을 **실측 근거로 끝낸 첫 모드**. RVA는 전부 실측 확정(등급 "유력" 아님) — `_MIGRATE_053.md` item_tactics 표도 이 값으로 정정 완료.

#### 11.1 함수시작 RVA (0.5.2 → 0.5.3) — 전부 실측 확정
| 상수 | 0.5.2 | **0.5.3** | 확정 근거 |
|---|---|---|---|
| `RVA_BUY_ITEM` | `0x211e070` | **`0xd0c680`** | 진입 24B 바이트 완전동일·exe 전체 **유일 1히트**, 본체 명령 대 명령 동형, 인자계약 유지(r8=athlete·`[rsp_entry+0x30]`=Game·Game+0x30=catalog), **orig_len=19 유지** |
| `ITEMNET_FORWARD_RVA` | `0x1b9cce0` | **`0x10587e0`** | 진입 24B 동일 + 피처명 문자열 5종 일치(self_item/champ_pos_build/lane_counter/synergy/global_counter) + net 레이아웃 불변(+0x8 가중치ptr·+0x10=16384·+0x18=1) |
| `CL_LAUNCHER_RVA` | `0x1d96870` | **`0xeb8810`** | 콜러 **9곳 = 구 exe와 9/9 대응**·렌더 씬빌더가 2회 호출·seedctor를 rdx=저장된 r8(seed)로 호출. 프롤로그 17B `55 41 57 41 56 41 55 41 54 56 57 53 b8 08 51 02 00`(chkstk 프레임 `0x165c8`→**`0x25108`** = 91KB→148KB) |
| `SEEDCTOR_RVA` | `0x22c1da0` | **`0x12b9ab0`** | 프롤로그 12B 동일(push8)·프레임 `0x11b58`→`0x11b98`·launcher 내부 콜 라인대응 |
| `LOADER_RVA`/`STRAT_LOADER_RVA` | `0x5ac950` | **`0x2e1550`** | ★**문자열-xref 정본 경로**로 도출(player_info/wide/strategy/training **16곳 전부 이 함수로 수렴**)·진입 24B 바이트 완전동일 |
| `PARSER_RVA` | `0x24b5a00` | **`0x1a6530`** | 3인자 계약·노드 stride `0x90` 유지(NT_SIZE 무변경) |
| `ALLOC_RVA` | `0x25c4d30` | ~~`0xbb2bd0`~~ → **`0x28f7df0`**(3인자) | shim 소멸 → **실할당자 직접 호출**로 일원화(07-29 확정) — 아래 11.4 |
| `DEALLOC_RVA` | `0x25c4d90` | **`0x1000`** | `__rust_dealloc(ptr,size,align)` 형태 유일. 모드 **미사용** |
| `RVA_REALLOC` | `0x25c4dd0` | **`0x28e3b10`** | 진입 112B 마스크시그 유일 1히트 + 본문 동형 |
| `FN_DD_SETOPT_RVA` | `0x242f250` | **`0x1bfc80`** | 직접 콜러 **103개 = 구 exe와 동수** + 오프셋 지문 전부 불변(+0x1788 selected · +0x1528/0x1530/0x1538 옵션Vec · +0x1570/0x1578 콜백 · 원소 `0xf8` · 입력 stride `0x28`). ⚠**프롤로그 변경** `55 56 57 48 83 ec 70` → `55 41 57 41 56 56 57 53 48 81 ec 88`(가드 **7B→12B로 교체**) |
| `SPAWN_RVA` | `0x1d9e0e0` | **`0xebfe50`**(~`0xec0302`) | 본문 명령 1:1 대응 · 콜러 컨테이너 `0x1d94640`→**`0xeb6480`**(+0x91 콜 @`0xeb6511`) · 직접 콜러 15곳. ⚠**게이트는 OFF 유지** — 아래 11.1a |

#### 11.1a `SPAWN_RVA` 재핀 성공 — **단 게이트는 OFF 유지**(`SPAWN_INJECT_ENABLED=false`)
- `0x1d9e0e0` → **`0xebfe50`**(~`0xec0302`) · 콜러 컨테이너 `0x1d94640`→**`0xeb6480`**(+0x91 콜 @`0xeb6511`) · 본문 명령 1:1 대응 · 직접 콜러 15곳. **소스 상수 갱신·재빌드 완료.**
- ⚠**OFF 유지 사유 2건**(켜기 전 반드시 처리):
  - ① **프롤로그 변경**: `7push + mov eax + chkstk` → **`8push`(12B) + `sub rsp,0xf8`**(chkstk **없음**) ⟹ **`ORIG_LEN` 15 → 12**, 0.5.2에 신설했던 **`install_detour_r11` 불요**(rax 보존 tail이 필요했던 이유가 사라짐).
  - ② ★**인자계약 변경**: `r8 = &descriptor` → **`r8`/`r9` = descriptor 2워드 쌍**(콜러가 빌더를 **전역 함수포인터 `0x144531340` 간접호출**로 변경). `rcx=Game`·`rdx=athlete` 스택사본은 **유지**.
- (종전 봉인 사유인 "SelectLineup 페이즈라 내 팀 side 판정 불가"는 그대로 유효 — 위 2건은 **RVA·계약 쪽 추가 선결조건**.)

#### 11.2 mid-func 사이트 — 전부 **컨테이너 내 패턴 재탐색**으로 도출(오프셋 이식 아님, §1 규칙 준수)
- launcher retaddr(내 경기 판정): 렌더 A `0x759c36`→**`0x9a3287`** / 렌더 B `0x75e5cf`→**`0x9a7b03`** / 조합테스트 `0xd40a63`→**`0x1925f12`**(컨테이너 `0xd405c0`→`0x1925ab0`).
- `patch_owned_cap`: `0x2341440`(imm+7) → **`0xf24a39`**(imm **`0xf24a40`**). 시그 `49 83 bf …` → **`48 83 be 58 04 00 00 03`**(R15→**RSI 회귀**). `cmp qword[reg+0x458],3`은 신 exe `.text` **전체 유일 1건**.
- `patch_gate3`: `0x211e428`(jbe+6) → **`0xd0c9be`**(jbe **`0xd0c9c4`**). 시그 `48 83 7c 24 78 02 76` → **`48 83 7c 24 40 02 76`**(스필 `rsp+0x78`→**`rsp+0x40`**). 신 exe **유일 1건**. resolver 컨테이너 `0x211e150`→**`0xd0c770`**.

#### 11.3 ★구조체 오프셋 변화 (다른 모드에도 파급 — 재조사 방지)
- ★**provider(RNG) 구조체가 `+0x40` 시프트**: 오프셋 **`≥0xb278` 구간이 전부 +0x40**(0xb278→0xb2b8 · 0xce98→0xced8 · ★**seed 저장 `0xeab8`→`0xeaf8`** · 0xeac0→0xeb00 · 0xeae8→0xeb28). **`0xb274` 이하는 불변.** item_tactics는 `O_PROVIDER_SEED` 상수로 단일화해 반영. ⟹ **provider+0xeab8을 쓰는 타 모드(ai_adjust A8_CACHE seed 키 등)는 0.5.3에서 반드시 `0xeaf8`로 갱신할 것.**
- ★**athlete 구조체 레이아웃 = 0.5.3 전면 불변 확인(검증완)**: champ String `+0x418`/`+0x420`/`+0x428` · items Vec `+0x448`/`+0x450`(슬롯ptr)/`+0x458`(owned) · build Vec `+0x490`/`+0x498`(ptr)/`+0x4a0`(len) · **id `+0x810`** · team(side) `+0x820` · 골드 `+0x888` · position(dword) `+0x8b0` · 스택사본 크기 `0x8b8` · **로스터 stride `0x8d0` 유지**.
- ★~~⬜`+0x810`(athlete_id) 검증 진행 중·미확정~~ → ✅**`O_ATHLETE_ID = +0x810` 0.5.3 유지 확정(검증완, 07-29)**: **athlete ctor `0x22cb050`→`0xed32b0` 재핀**, 3연속 스토어 관용구가 **명령 단위로 동일**(`mov [rsi+0x810],reg` / `+0x818,0` / `+0x820,rax`, reg←rdx=arg2). 교차검증 = 로스터 순회 `0x1740380`(`add rbx,0x8d0` → `mov r12,[rbx+0x810]`) + VIEW 시그 `0xee9070`(`[rcx+0x840]` 배열·`[rcx+0x848]` count·`imul rcx,r9,0x8d0`). ⟹ **fix B(`is_my_athlete` = `athlete+0x810` ∈ `MY_ATHLETES`)가 0.5.3에서도 그대로 성립 = 팀 스코프 판정 안전**(0.5.2 "관전≠확정" fix 유지).
- ★**Game `+0x1dc0`(provider data ptr)/`+0x1dc8`(vtable) = 0.5.3 유지 확인**: launcher `0xeb9646`(`mov [rsi+0x1dc0],rax; mov [rsi+0x1dc8],rax`) · spawn 본문 `[rcx+0x1dc0]`/`[rcx+0x1dc8]`→`call [r15+0x160]`. ★**vtable 슬롯 `+0x20`이 `mov rax,[rcx+0xeaf8]`** = 위 새 seed 오프셋과 정합 ⟹ **`+0x1dc0`이 provider라는 독립 증명**. Game `+0x1dd0`/`+0x1dd8`·`+0x2060`도 유지.
- 0.5.3 전역 코드젠 변화: **alloc/dealloc 직접 call → 간접 썽크 호출** ⟹ "alloc 직접 call"을 앵커로 쓰던 시그는 **전멸**.
- buy 호출 경로: direct call → **vtable(+0x78) 썽크(`0xd22340`) 경유**로 변경(단 **함수 진입부 훅**이라 전 호출 포착됨 = 모드 영향 없음).

#### 11.4 `ALLOC_RVA` — ~~충돌 병존(미조정)~~ → ✅**해소·일원화 완료 = `0x28f7df0` + 3인자**(2026-07-29 확정)
- 사실: 0.5.3엔 2인자 `__rust_alloc(size,align)` **shim이 소멸**(전 호출부 인라인).
- ✅**정본(3세션 합의, item_tactics도 이 값으로 교체·재빌드·배포 완료)** = **`0x28f7df0`**(0.5.2 `0x25d9640`과 **명령 단위로 동일**·GetProcessHeap→HeapAlloc). 계약 = **`(rcx=무시, rdx=flags(0), r8=size) -> rax`**, ★**실패 시 0 반환**. 0.5.2 `__rust_alloc`의 **align≤0x10 경로가 바로 이 헬퍼로 tail-jmp** 했으므로 결과 블록이 동일 = `__rust_dealloc`(align 8)·게임 파서 free 경로와 **정합**.
- ⛔~~item_tactics 채택안 `0xbb2bd0`(align 8 고정 심·rcx=size)~~ = **채택 안 된 대안**: 동작 자체는 정상이나 ⚠**OOM 시 0이 아니라 abort**라 **호출부의 null 체크가 死코드**가 되고 align≠8은 조용히 틀림 ⟹ **쓰지 말 것**.
- 검증: 배포 dll에 `0x28f7df0` **3회 존재** / `0xbb2bd0` **0회**. 상세 = §12.3(ai_adjust 최초 도출) · §13.4(serpen 독립 재도출 2:1).

#### 11.5 ~~★기능 1건 비활성 — 경기중 4번째 아이템 슬롯 아이콘 표시 = 0.5.3 포팅 불가 → 봉인·재조사 금지~~ → ✅★**정정·해결(2026-07-30, 인게임 검증완)** — 봉인 판정은 **게임 코드 바이트패치(루프 상한 확장) 방식에 한정**이고, **뷰모델 직독 + 노드 조작 방식은 게임 코드 무패치로 성립**

> ⚠★**적용 범위 주의(다음 세션 필독)**: 아래 ①②(구 봉인 근거)는 사실로 여전히 유효하나 **"게임 코드를 고쳐 4칸째를 그리게 한다"는 방식에만** 해당한다. **"봉인됨"만 보고 재시도 자체를 포기하지 말 것.** 스위치도 별개다 — `DIAG_SLOT_UI_OFF = true`는 **구 바이트패치 방식의 스위치라 계속 true 유지**, 신 방식 스위치는 **`SLOT3_ICON_ENABLED = true`**(문제 시 false로 즉시 원복).

- ① (바이트패치 한정) `RVA_SLOT_HELPER`(0.5.2 `0xc5cd80`)가 0.5.3에선 **UI 메가함수 `0xa5c1e0` 안으로 완전 인라인**(신 exe에 `"blue_pla"`/`"red_play"` movabs **0건**·콜사이트 **0건**). 인라인 블록 **4곳(각 75B)** 이 (ptr,len) 3쌍을 `rbp+0x10d20`/`+0x10d30`/`+0x10d40`에 직접 스토어.
- ② (바이트패치 한정) **루프 상한만 늘리는 것도 불가**: 4번째 엔트리 자리 `rbp+0x10d50`/`+0x10d58`이 0.5.3에선 **다른 지역변수로 이미 사용 중**(각 **40회·27회** 참조 실측) ⟹ 상한 `0x30`→`0x40`으로 바꾸면 그 지역변수를 문자열 (ptr,len)으로 읽어 **확정 크래시**. 프레임 여유도 없음(rbp 상한 `+0x10f88`·상단은 xmm 스필).
- `SLOT_BOUNDS` 4곳의 0.5.3 주소는 **재핀해 소스에 남겨둠**(재조사 방지): `0xa63166`/`0xa638df`/`0xa64486`/`0xa64c16`, 전부 `48 83 fb 30`(레지스터 r14/r15 → **rbx 통일**). ⛔**단 적용 금지**(위 ② 사유).

##### 11.5a ✅**확정 구현 = 뷰모델 직독 + 노드 조작(0.5.3 인게임 검증완 2026-07-30, dll 535,040B)** — 이 소절이 정본
- **원리**: 게임이 slot0~2를 그릴 때 읽는 데이터를 **모드도 그대로 읽어** slot3 노드에 세팅한다(게임 코드 무패치).
```
GameView (= App + 0x4a50, 프로세스 수명 내내 불변)
  → player_view HashMap (hashbrown RawTable: ctrl +0x1d0 / mask +0x1d8 / items +0x1e8, 엔트리 stride 0x260)
      키 = (team u64 @+0x00: 0=blue/1=red, position u32 @+0x08: 0 top/1 jungle/2 mid/3 bottom/4 support)
      값 PlayerViewInfo: items Vec<u64> = {cap +0x50, ptr +0x58, len +0x60}   ← 원소 = item_list 인덱스
  → item_list (GameView +0xa8 cap(-1=None) / +0xb0 ptr / +0xb8 len), 원소 16B = (data, vtable)
  → vtable +0x60 = icon() -> &String    ← 게임 set_item_icon(0x97b540)과 동일 경로
  → 노드 <player_info|wide_data.player_info>.<lane>.{blue_player|red_player}.slot3.bg.icon 에 세팅
```
- **GameView 캡처** = `game.rs update` **RVA `0x960df0`** 진입 detour(`rcx` = &mut GameView · **ORIG_LEN 12** · 프롤로그 `55 41 57 41 56 41 55 41 54 56 57 53`). **읽기전용·1회 저장이면 충분**(값 불변). `mod_api`는 GameView를 노출하지 않는다(SDK 경로 없음) ⟹ **detour 캡처가 정답**.
- **해시 계산 불요 = 버킷 선형 스캔**: ctrl 바이트 최상위비트 0 = FULL, 엔트리는 ctrl 기준 **역방향** `ctrl - (i+1)*0x260`.
- **아이콘 세팅** = ImageRunner 4상태(stride 208: normal/hover/active/disabled)의 `source`(+0)=고정 시트 / `rect_tag`(+0x18)=Some(태그). **빈칸 = `Node.visible`(+0x260)=0만**(이미지 필드는 건드리지 않음 = 게임과 동일 동작).
- 노드 탐색 `0x19f170(node, segs, n)` = `'.'` split 계층 재귀 · `"bg.icon"` 리터럴 `0x318afe0`.
- **뷰상태 정체** = `game_view::view::game::GameView`(`game-view\src\view\game.rs`) · UI 메가함수 `0xa5c1e0` = `game-view\src\ui\ingame_ui.rs`.
- ★**해시맵 키가 athlete_id가 아니라 (team, position)** ⟹ UI 측에서 `athlete+0x810` 조인 **불요**.

##### 11.5b ★★**items[3] 존재 = 확정(핵심 발견 — "게임이 4번째를 안 준다"는 오해 폐기)**
- 게임 슬롯 루프의 `cmp rbx,0x30`은 **아이템 개수 제한이 아니라, 하드코딩된 노드명 3개("slotN") 배열의 바이트 크기**다. 실제 아이템 순회는 **`i < items.len()` 길이가드**(`0xa6339f`).
- 뷰 체인 전 구간에 take(3)/min(3) 없음(`GameView::apply_frame` `0x952170` = **capless collect**) ⟹ **4번째 아이템 데이터는 원래부터 뷰모델에 존재**했고 게임이 읽지 않았을 뿐. 0.4.x 시절 "owned=4면 아이콘 배열도 4개 → slot3 자동충전" 실증과도 정합.

##### 11.5c ★**노드 구조 정정 — 구 기록이 틀렸다(`blue_player`는 1개가 아니다)**
- `blue_player`/`red_player`는 **레인당 1개씩(5+5)**, 레이아웃 변형까지 합쳐 **최대 20개**:
  `<ingame>/player_info/<lane>/{blue_player|red_player}/slot<N>/bg/icon` (일반) · `<ingame>/wide_data/player_info/<lane>/{…}` (와이드).
- 레인 상수 테이블 = `[(u32 position, &str name); 5]` = top/jungle/mid/bottom/support(문자열 RVA `0x318b000`~) · UI 빌더 `0xa8ed10`이 레인명으로 **5개 인스턴스 생성**.
- ⚠★**이것이 "엉뚱한 아이템이 뜬다"의 진짜 원인**이었다 — 구현이 `find_node(root,"blue_player")`로 **첫 매치 1개만** 처리해 한 레인 노드에 다른 선수 값을 쓰고 있었다.

##### 11.5d ⛔**폐기된 접근 2종(재시도 금지)**
1. **게임 코드 수술**(프레임 확장 `mov eax,0x11008`→`0x11048` + 배열 base 이전 + 인라인 블록 트램폴린 + 호출자 스택참조 **68곳** 보정) = **84/84 사이트 시그 검증 통과에도 경기 진입 프리즈**. ⟹ **주소가 다 맞아도 전제(프레임 확장 부수효과·배열 통째 전달 코드 유무)가 틀리면 실패** = 134KB 렌더 메가함수 수술은 **정적 분석만으로 안전 보장 불가**.
2. **챔프 이름 캐시**(buy 훅에서 champ→icon 캐시 후 UI에서 조회) = **구조상 오염 불가피**. 내 선수는 배경 pre-sim과 화면 경기에 **동시 존재**(athlete+0x810 조인이 양쪽 유효)라, 배경에서 4개 완성한 값이 화면(3개 보유) 선수에게 샌다. `is_live` 게이트·"앞 3칸 다 차야" 조건·(챔프,slot0~2태그) 튜플 키 **전부 유저 케이스(4개 전부 지정 = 결정론적 빌드)에서 무력**.

##### 11.5e ~~⛔**툴팁 = 게임 원천 미지원(별건·⬜미해결)** → 모드 자체 렌더(C안)만이 경로~~ → ✅★**완전 해결·인게임 검증완(2026-07-30, 0.5.3, 유저 "다 잘뜬다") = 게임 `#item_tooltip` 노드 + ★게임 툴팁 `show` 함수 통짜 호출**(자체 렌더·라벨 직접 write 둘 다 불필요) — 이 소절이 정본
- **유효한 사실(변함없음)**: 게임 툴팁 코드는 `"<side>_player.item0/1/2"` **3경로만 하드코딩**·상한 3 ⟹ **`#slot3`을 절대 방문하지 않는다**(emit도 메가함수 프레임 로컬 `[rbp+0x10e90]`·UI ctx 레지스터에 강결합 = 외부 shadow-call 불가 = **B안 비권장 유지**).
- ⛔**정정**: 그러나 ~~"그러므로 모드가 직접 그리는 C안만이 경로"~~ 는 **오판정 = 폐기**. **제3의 길 = 게임 툴팁 UI 노드 재사용**이 성립하고 인게임 검증됐다 — 게임 툴팁 노드가 bundle `base\ui\layout\ingame.ui` **L2611에 이미 존재**(평소 `visible:false`)하므로 **모드가 내용만 채워 켜면 모양이 게임과 100% 동일**하다. ★**교훈(버전무관)**: **게임이 로직에서 안 잡아줘도 UI 자산(노드)은 재사용 가능** — `.ui` 레이아웃 실물을 먼저 뒤지면 자체 렌더를 피할 수 있다.
- **노드 구조**(`ingame.ui` L2611 실측):
```
#item_tooltip : color   (274x250, rounding 12, color #4a4c56ff, visible:false, ignore_event:true)
  ├ #bg : color         (272x248, #161721ff)
  └ #data : empty       (264x237)
      ├ #slot : color > #icon : image   ← 아이템 아이콘
      ├ #name : label   (bold, size 18, x45)
      ├ #tier : label   (size 16, x145, align Right, #666666ff)
      ├ #gold_icon : image + #price : label (size 16, #fde99fff)
      ├ #bar : color    (264x1, y46)
      └ #desc : label   (264x185, size 16, line_height 20, anchor_y:1)
```
  ⚠`champion_tooltip.ui`는 별도 파일(z:230)이나 **아이템 툴팁은 `ingame.ui` 내장**.
- **구현 계약**:
  - **호버 감지 = `Node.focus`(+0x262) ∈ {1,2}** — ★게임 hit-test가 **모드 주입 노드(#slot3)에도 세팅해준다**(런타임 실증) ⟹ 마우스 좌표 자체 판정 불요.
  - ★★**최종 확정 = 게임 툴팁 `show` 함수 통짜 호출**(라벨 직접 write 방식은 **폐기** = §11.5g③). **RVA `0x1ab52f0`** = `game-view\src\ui\item_tooltip.rs`의 `show` · **인자 11개 `extern "win64"` · 반환 void**:

| 자리 | 인자 | 값의 출처(★실측 = 이게 틀리면 즉사) |
|---|---|---|
| `rcx` | p1 = 애셋/설정 레지스트리 | `game.rs update` **`0x960df0`의 arg5** |
| `rdx` | p2 = 텍스트 계측 ctx | 같은 함수의 **arg6** |
| `r8` | p3 = 계측 ctx vtable | **상수 = base + `0x318b4c0`** |
| `r9` | node | `#item_tooltip` 노드(모드가 `find_node(&ui.root,"item_tooltip")`로 획득) |
| `[rsp+0x20]` | item_data | ★**빌림만 — drop 금지**(`item_list` 원본 그대로 = 안전) |
| `[rsp+0x28]` | item_vtable | 동상 |
| `[rsp+0x30..0x48]` | f32 `x`,`y`,`pivot_x`(0.0),`pivot_y`(0.0) | ※f32도 **스택 슬롯**에 배치 |
| `[rsp+0x50]` | `*const [f32;4]` clamp_rect | `[0,0,1920,1080]` (게임 상수 = `0x3189d90`) |

  - **인자 캡처** = 기존 `0x960df0` 진입 detour 그대로. 스텁 레이아웃(`push r12,rsi,rdi,rbx,r11,r10,r9,r8,rdx,rcx`)에서 **r9 = saved+3 / 진입 rsp = saved+10** ⟹ **p1 = `[sp+0x28]` · p2 = `[sp+0x30]`**.
  - ★**노드 탐색 루트 = 그 함수의 arg4(r9)** (단 모드는 자기가 찾은 노드를 직접 넘기므로 루트는 실사용 안 함).
  - ★**이 함수가 이름·티어·가격·스탯 24종·효과설명·i18n 해석·폭/높이 계산·자식 rect 조정·화면 클램프까지 전부** 처리한다 ⟹ **모드(워크샵) 아이템도 자동으로 정확**(번들 파일엔 없지만 게임 레지스트리엔 있으므로).
  - **앵커(게임 규칙 그대로)**: **blue = (`slot.rect.x + slot.rect.w − tip.authored_w`, `slot.rect.y + slot.rect.h + 12`)** / **red = (`slot.rect.x`, `slot.rect.y − tip.authored_h − 12`)**. authored w/h = **`tip+0x74` / `tip+0x7c`**.
  - ⚠★**호출 전제(위반 시 게임 내부 unwrap 패닉 → abort)**: `#item_tooltip` 하위 **8개 노드 전부 존재** 필수 = `bg`, `data`, `data.slot.icon`, `data.name`, `data.tier`, `data.price`, `data.bar`, `data.desc`. **모드는 호출 전 전수 `find_node` 검증**한다.
  - ★**아이템 vtable 슬롯맵(확정·구 기록 정정)**: **`+0x50` bool**(`self+0x190 != 0`) / **`+0x58` key(&String)** / **`+0x60` icon(&String)** / **`+0x68` price(u64 값)** / **`+0x70` tier(u64, 0-base)** / **`+0x78` sret 스탯 구조체(out,self)** / **`+0x90` sret String 효과설명(out,self,registry)** / **`+0x18` = clone_box**. ⛔**이름 게터는 vtable 슬롯이 아예 없다** — 게임은 `key`로 i18n 키를 조립한다. (구 기록 `+0x50`=name·`+0x68`=has_recipe·`+0x18`=content 객체는 **전부 오류 = 정정됨**.)
  - **위치·크기** = 게임 show가 계산·클램프까지 하므로 **모드는 앵커 x/y만 준다**. ⛔~~"툴팁 노드 rect 직접 세팅으로 이동 가능(게임이 매 프레임 재계산하지 않음)"~~ = **오류 = 폐기**(§11.5g③②: authored 4블록 + 자식 크기까지 매 프레임 재계산되므로 rect만 쓰면 다음 프레임 원복).
  - ★**소유권 규칙(핵심 — 없으면 게임 툴팁이 깨진다)**: 게임도 slot0~2 호버에 **같은 노드**를 쓴다 ⟹ ①게임이 이미 띄운 프레임(`tip.visible==true`이고 우리 소유 아님)엔 **양보·무개입** ②우리가 띄운 것만 `TIP_OWNED` 플래그로 추적해 호버 종료 시 **우리 것만** 내린다.
- ~~⬜기능 제한: `#name`·`#icon`만 채움·`#tier`/`#price`/`#desc` 미구현~~ → ✅**해소(07-30) = show 함수가 전 칸을 채운다**(이름·티어·가격·스탯·효과설명 전부 정상 = 유저 "다 잘뜬다").
- ⬜**미확정으로 남긴 것(사실 승격 금지)**: ① **p2(텍스트 계측 ctx)의 프레임 간 수명·스레드 안전성** — 런타임 확인 필요(현재는 **매 프레임 갱신**으로 회피 중) ② 엔진 레이아웃 패스가 매 프레임 rect를 authored로부터 재계산하는지(정황은 그렇다).
- ★**0.5.2 대조** = 게임 툴팁 코드는 0.5.2도 `item0/1/2` 3경로만 순회 ⟹ **0.5.2에서도 4번째 칸 툴팁은 안 떴다**(아이콘만 떴음) ⟹ 이번 구현은 **마이그 복구가 아니라 신규 기능**(~~⬜유저 체감 대조 필요~~ 해소).
- 스위치 = **`TOOLTIP_ENABLED`**(false로 즉시 원복 / 아이콘은 `SLOT3_ICON_ENABLED`와 **별개**).

##### 11.5g ⛔★**툴팁 실패 3종 = 전부 "추정을 사실로 쓴" 결과(재시도 금지) + ★프로세스 교훈(이번 세션 최대 손실)**
1. ⛔**vtable 슬롯 오해로 크래시**: `+0x50`을 name(&String)으로 알고 역참조 → **실제는 bool**(`self+0x190!=0`). **이름 게터는 슬롯이 없다**(key로 i18n 키 조립). 동반 정정 = `+0x68`은 has_recipe가 아니라 **price(u64 값)** · `+0x70` = **tier(u64, 0-base)** · `+0x18` = **clone_box**(content 객체 아님). 확정 슬롯맵 = §11.5e.
2. ⛔**show 호출인데 인자가 한 칸씩 밀려 즉사**: p1←arg4(r9)·p2←arg5·root←arg7로 넣었다 ⟹ **UI 루트 Node 포인터를 레지스트리로 넘겨** 게임이 그것으로 hashbrown 조회(`[p1+0xf0]` ctrl 역참조) → 크래시. 정답 = **p1=arg5 / p2=arg6 / root=arg4**. ★**인자 개수·순서·타입 자체는 처음부터 맞았고 "출처"만 틀렸다** = 가장 잡기 어려운 실패 모드.
3. ⛔**라벨 직접 write 방식 = 빈 툴팁 + 크기·위치 잔류 ⟹ 폐기**(번들 파일 파싱 방식 원천 폐기 — 모드 아이템은 애초에 못 잡음):
   - ① 모드가 읽은 i18n 경로 `<게임>\bundle_unpacked\base\text\item.i18n`이 **게임 폴더에 존재하지 않는다** — 게임엔 **`bundle_unpacked_full\`** 만 있고, `bundle_unpacked\`는 `Desktop\claude\tfm2\tfm2_0.5.3\` **백업 전용**이다 ⟹ i18n 조회 전건 실패 → 빈 문자열이 라벨을 지웠다. ★**버전무관 함정: 게임 폴더 번들 경로는 `bundle_unpacked_full`**.
   - ② 게임은 rect(`+0x240`)만이 아니라 **authored 레이아웃 4블록**(base `0x70`/`0xf0`/`0x170`/`0x1f0`, stride `0x80`: `+0x04 w`/`+0x0c h`/`+0x14 x`/`+0x1c y`/`+0x38,0x3c pivot`, 각 앞 u32 tag=1)과 **자식(`bg`/`data`/`bar`/라벨 4종) 크기까지 전부** 재계산한다 ⟹ **rect만 쓰면 다음 프레임 원복** = "이전 아이템 크기·위치 그대로" 증상.
- ★★**프로세스 교훈(반드시 남길 것)**: 툴팁에서 **3연속 실패**했고 원인은 전부 **"RE 보고서를 부분만 받아 적고 나머지를 추정으로 메운 것"**이다(아이콘 작업은 조사를 끝까지 받고 구현해 **한 번에 성공**). ⟹ **규칙: 게임 함수 직접 호출·구조체 오프셋 사용 시 "인자 출처(어느 콜사이트의 몇 번째 인자인지)"까지 실측 확인 전에는 코딩하지 않는다.** 슬롯/오프셋이 하나라도 미확정이면 **그 항목은 구현에서 제외**하거나 확정 후 착수.

##### 11.5f ★**구매 순서 = 섞이는 것이 정상 동작(설계 확인)**
- ~~"4번째 아이템을 먼저 산다"는 유저 관측~~ = **아이콘 오표시 아티팩트**(유저가 슬롯 UI로 판단)였고 **실제 구매 순서 문제가 아니다 — 구매 로직은 정상**.
- 설계상 **내 팀은 지정 4개가 경기 시작부터 `build[]`에 들어가 게임이 골드 되는 대로 산다 ⟹ 순서가 섞이는 것이 정상**(적팀은 신경망이라 순차 구매).

#### 11.6 다른 모드에 바로 쓸 수 있는 부수 확정
- ★★**LabelRunner(size `0x1f0`) 내부 확정 + 구 기록 정정**(0.5.3 실측, 07-30): 상태블록 4개(stride `0x58` — 내부 `+0x00 String font`/`+0x18 lh_mode`/`+0x1c line_height`/`+0x48 size`) → **`+0x160` String text = {cap@`0x160`, ptr@`0x168`, len@`0x170`}** / **`+0x178` args Vec** / **`+0x190` Option\<String\>**. ⟹ ★**메모리 `[[tfm2-native-dropdown]]`의 "LabelRunner text = len@352, ptr@360, cap@368"은 오류 = 정정**(정답 = **cap@352 · ptr@360 · len@368**). **게임 String 공통 배치 = `{cap@0, ptr@8, len@0x10}`**. (⚠단 ui_kit `wr_string`/`rd_string`은 Rust `String` 타입 통째 대입이라 **필드 순서와 무관하게 안전** = 코드 수정 불요.)
- ★★**라벨 텍스트에 캐시·dirty 플래그가 없다(확정)**: 표시 문자열은 **매 프레임 재계산**(i18n 해석 `0x1d28d0` → 말줄임 `0x1d2500`). `#`로 시작하면 `'?'` 앞을 애셋 경로로 로드해 뒤를 키로 조회하고 **args Vec 원소로 `{...}` 치환**, **조회 실패 시 원문 그대로 표시**. ⟹ **String만 바꾸면 다음 프레임에 반영된다** = 구 "라벨이 안 바뀌는 건 캐시 때문" 가설 **폐기**.
- `#tier` 라벨은 게임이 **text를 안 쓰고 args Vec에 `tier+1`을 push만** 한다(**clear 없음** = 매 프레임 호출 시 누적). 모드가 직접 채울 땐 `tier_header.<N>`(플레이스홀더 없는 완성문)을 **text**에 넣는 게 안전.
- **Node 오프셋**: `+0x230` runner data / `+0x238` runner vtable / `+0x240..0x24f` rect(x,y,w,h) / `+0x260` visible / `+0x262` focus / **authored 레이아웃 4블록 = `0x70`/`0xf0`/`0x170`/`0x1f0`**(stride `0x80`·§11.5g③②).
- **Runner vtable** `+0x48` = as_any / `+0x50` = as_any_mut → 반환 `&dyn Any` vtable `+0x18` = type_id. **TypeId**: ImageRunner `(0x7aeec23a0875a029, 0xf4de862992c22f62)` / LabelRunner `(0xe79dbe134c100aa5, 0x7c3ff064287074de)` ⟹ **러너 종류 런타임 판별 가능**(문자열 비교 불요).
- **툴팁 크기 공식(게임 show 내부)**: `maxW = clamp(clampRect.w − 16, 274, 920)` · `W = min(maxW, max(274, max(descW+20, nameW+16+45+8+T+10)))` · `wrapW = max(W−10, 1)` · `H = descH + 95`. `#desc` = `stats.join(", ")` + `"\n"` + 효과설명(`vt+0x90`).
- **스탯 줄 빌더**: `0x1a3f900`(i32, >0만) / `0x1a3f5e0`(i64, ≠0만) — 시그 `(Vec<String>* out, p1, locale_ptr, locale_len, key_ptr, key_len, value)`. 내부에서 `format!("#asset/base/text/item?spec.{key}")` 조회 후 `{Value}` 치환.
- ⚠**스탯 오프셋 표는 `vt+0x78`이 반환하는 구조체 기준**이며 **`item_data + 상수`가 아니다** ⟹ 모드가 가정했던 `item_data+0x60 = attack`은 **근거 없음 = 폐기**.
- 관련 RVA(0.5.3): 툴팁 show **`0x1ab52f0`** / 계측 ctx vtable **`0x318b4c0`** / 클램프 상수 `0x3189d90` / 라벨 i18n 해석 `0x1d28d0` / 말줄임 `0x1d2500` / 아이콘 세팅 `0x1a43720` / 설정맵 get `0x143bf0` / 애셋 로드 `0x1f0110` / UI 메가함수 `0xa5c1e0` / `game.rs update` `0x960df0`.
- 유저 스크린샷의 아이콘 문자(⚔️💚⏳)는 **게임에 서식 로직이 없음이 확정** ⟹ **워크샵 모드 아이템 자체 i18n 문자열**로 판단(강한 추정 — 100% 확증 아님).
- `tfm2_comptest_unlock`도 쓰는 **`FN_DD_SETOPT_RVA` = `0x1bfc80`**(⚠프롤로그 7B→12B 교체 필요). 드롭다운 `runner+0x1150`(present)/`+0x1154`(f32 px) = **0.5.3 불변**.
- `tfm2_elemental_serpen`의 `LAUNCHER_RVA` = item_tactics와 **동일 함수** ⟹ **`0xeb8810`** · `LAUNCHER_RET_C`(0.5.2 `0x1555215`) → **`0x229ad94`**(컨테이너 `0x1554930`→`0x229a410`).
- ★★**`_MIGRATE_053.md` 자동매칭 표의 오답 1건 확정**: `LOADER_RVA`류(0.5.2 `0x5ac950`)를 **`0x91ab0`으로 매핑한 것은 오답**(모노모픽 clone family 형제와 혼동). 정답 = **`0x2e1550`**. ⟹ ★**표의 "확정" 등급도 clone family 함수에선 신뢰 불가 = 문자열-xref 재검증 필수**(§3 등급 정의의 예외로 기억할 것). ⚠같은 사유로 `RVA_ASSET_GET`(0x99c860)·`RVA_ANIM_GET`(0x5ab7d0)도 `0x91ab0` 매핑 = ~~오답 의심·미해결 취급~~ → ✅**오답 확정(0.5.3, 07-29 serpen 세션 §13.3)** — asset-get은 0.5.3에서 **30-copy 모노모픽 군집**이라 통계매칭이 군집 대표를 오집하는 것이 기전(0.5.2의 서로 다른 3함수가 전부 `0x91ab0`으로 매칭된 것이 증거). ai_adjust·comptest_unlock·banpick_illust의 `0x91ab0` 값도 **전부 재검증 대상**.
  - ✅~~⚠충돌(미조정) — ai_adjust `0x91ab0` ↔ item_tactics `0x2e1550`, 어느 쪽도 확정 아님~~ → **판정 확정(2026-07-29): `0x2e1550`이 정답 · `0x91ab0`은 오답.** ★**결정적 근거 = 콜러 수 대조(실측)**: 0.5.2 정답 `0x5ac950` **507개** ↔ 0.5.3 `0x2e1550` **511개**(규모 일치) ↔ 0.5.3 `0x91ab0` **2개**(규모 완전 불일치 = clone family 중 거의 안 쓰이는 형제). 보조 = 문자열-xref 정본 경로(player_info/wide_player_info/strategy/training **16곳**)가 **전부 `0x2e1550`으로 수렴**, `0x91ab0`으로 가는 경로 **0건**(도구는 0.5.2에 먼저 돌려 정답 `0x5ac950` 재현으로 검증). 독립 확증 = §13.3(serpen 세션 — 콜러 사상 193/194 일치) · §12.6 해소.
  - ★★**방법론 교훈(다음 패치에 그대로)**: **clone family 식별의 결정적 지문 = 콜러 수**. 진입부 **24B가 완전 동일**해 바이트로는 형제를 구별할 수 없고, push8 12B 프롤로그는 `.text`에 66,635회 등장(§1) = 변별력 0. **0.5.2 clone 3형제 콜러수 = `0x5ac950`(507) / `0x99c860`(67) / `0x5ab7d0`(77)** — 이 스펙트럼을 신 exe 후보들과 대조하면 형제가 갈린다.
  - ⚠★**파급(⬜미조치 액션)**: **`tfm2_ai_adjust`는 `0x91ab0`(콜러 2개짜리 무관 형제)을 후킹하도록 배포된 상태** ⟹ **크래시가 아니라 UI 주입이 조용히 미발화**한다. **`LOADER_RVA`를 `0x2e1550`으로 교체 후 재빌드 필요**(§12.6·잔여트래커 등재). `0x91ab0`을 받아쓴 comptest_unlock·banpick_illust(`RVA_ASSET_GET`/`RVA_ANIM_GET`)도 동일 재검증 대상.

#### 11.7 방법론 (재사용 자산)
- 이번 재핀은 **Ghidra가 아니라 capstone+pefile 직접 바이트스캔** = **`C:\tfm2mods\_it_scan.py`**(재사용 가능: PE 섹션 / `.pdata` 함수경계 / 패턴스캔 / rip-rel 역참조 / call·jmp 전수열거 **numpy 벡터화**).
- ★**도구 신뢰 확보 절차** = **0.5.2에 먼저 돌려 이미 문서화된 정답을 재현**시켜 도구를 검증한 뒤 0.5.3에 적용 — 다음 패치에도 이 순서를 따를 것.

#### 11.8 배포 상태 / ⬜인게임 검증 시 최우선 확인
- 빌드·배포 완료: `tfm2_item_tactics.dll` **504,832B** · `mod.mod_info` **v2.5.0**(747B · `last_updated` 2026-07-29 · **BOM 없는 UTF-8 확인**). SDK = `sdk_053` · toolchain `nightly-2026-05-24` · `-C opt-level=1 -C overflow-checks=off` · 링커 `rust-lld`.
- ⚠**SPAWN 상수 갱신·`ALLOC_RVA` 교체 후 재빌드해도 dll 크기 동일(504,832B)** = 이상 아님 — **게이트 OFF 상수는 DCE로 제거**되고 나머지는 **상수 교체**뿐이기 때문(§11.1a·§11.4). ⟹ "크기 안 바뀜 = stale dll"로 오판하지 말 것(신원 판정은 mtime+해시로).
- ✅**최종 배포 스팟체크(07-29)**: 배포 dll 바이트 스캔 = `0x2e1550` **2회** / `0x28f7df0` **3회** / 오답 `0x91ab0` **0회** / 구 0.5.2 값 **전부 0회**.
- ★★**교훈(버전무관·재발방지)**: **`build_inj.ps1`은 dll만 복사한다** — `mod.mod_info`·`.ui`·`.cfg`를 바꿨으면 **수동 배포 필요**. 이번에 배포 폴더 mod_info가 구버전으로 남아 **deploy-verify가 FAIL로 잡아냈고 수동 교체**로 해소(v2.5.0·747B·BOM無·소스와 해시 일치).
- ~~⬜**인게임 미검증**(유저 확인 대기)~~ → ✅**부분 검증완(2026-07-30)**: **경기중 4번째 슬롯 아이콘 표시**(유저 "잘된다") + ★**툴팁 완전 동작 검증완**(유저 "다 잘뜬다" — 이름·티어·가격·스탯·효과설명 전부, §11.5e) · 배포 dll ~~535,040B~~ → ~~542,208B~~ → **535,040B**(★07-30 04:04 배포본 **실측** = 최종. "542,208B"는 오기/미배포본) · 스위치 `SLOT3_ICON_ENABLED`(아이콘)/`TOOLTIP_ENABLED`(툴팁) **각각 false로 즉시 원복 가능** / `DIAG_SLOT_UI_OFF`는 **true 유지**(구 바이트패치 경로 봉인용 = **아이콘·툴팁 기능과 무관**, 소스 주석에도 명시 — 혼동 주의) / ~~⚠진단 `BUILD_EXT_DIAG`·`BUY_ORDER_DIAG` 아직 true~~ → ✅**정리완(둘 다 `false`)**. ~~⬜툴팁 `#tier`/`#price`/`#desc` 미구현~~ → ✅**해소(show 함수가 전 칸 처리, §11.5e)** / ⬜나머지(구매·`is_live`·AUTO4) 인게임 미검증 / ⬜**릴리스 zip = 이번 dll(535,040B)로 재생성 필요**(직전 zip은 툴팁 이전 버전).
- ⬜★**최우선 확인 항목 = buy 훅 `is_live` 게이트의 provider 인자**: 이 게이트는 `Game+0x1dc0`이 아니라 **`r9`(arg4)=provider**를 쓴다(`lib.rs` `*saved.add(3)`). **buy 본문이 arg4를 즉시 덮어써 정적 검증이 불가**하며 0.5.2 때도 **인게임 seed 매칭으로만** 확정됐다. ⚠**0.5.3에서 어긋나면 크래시가 아니라 "관전 경기 미인식"으로 조용히** 나타난다 ⟹ **`is_live` 히트 카운터로 판별**할 것.

---

### 12. ★`tfm2_ai_adjust` 0.5.3 마이그 **완료·빌드·배포·✅인게임 검증완**(2026-07-29 → ★**최종 2026-07-30**, 0.5.3 buildid 24451609, dll ~~3,354,112B(07-29 23:34:16)~~ → ~~3,334,144B · 01:32:10(UI 주입 OFF 반영분)~~ → ★**최종 3,338,240B · 2026-07-30 10:50:12**(vt슬롯 `0x1c8`·world `+0x40`·disc7 재가동/경로계측 반영분), rustc 직접빌드=사이즈가드 초과 모드) — **본 절 = 이 건의 정본** / ★**byte-patch 62/62 전량 라이브(§12.5)** · **훅 7종 설치 실증(§12.8 스텁 인벤토리)** · **⏸UI 주입 중단(§12.9)** / ⬜경기 구동(재현 detour 발화) 미확인

> 검증 방식 = **exe↔exe capstone 실측**(Ghidra 아님). RVA 단일수정점 = ~~`src\rva_052.rs`~~ → **신규 `src\rva_053.rs`**(`tfm2_ai_adjust.rs:25` `include!` 전환 완료, 구 파일은 이력 보존·참조 없음).
> 신규 도구 5종(전부 `C:\tfm2mods\`, 다음 패치 재사용): `verify_053.py`(프롤로그·orig_len 경계·rip-rel 실측) / `bytepatch_053.py`(`patch_imm_bytes` 사이트 전수 파싱 → 컨테이너 내 "prefix+원본imm" 시그 재탐색) / `bytepatch2_053.py`(레지스터·스택오프셋 마스크) / `bytepatch3_053.py`(전역 순서대응) / `desc_053.py`(.rdata desc 재핀) + `repin_053.py`·`alloc_053.py`·`scan_053.py`·`midfunc_053.py`.

#### 12.1 함수시작 RVA (0.5.2 → 0.5.3) — 전부 선두 바이트 완전동일 실측(=§7.3 §4의 "유력" 등급 → **실측 확정으로 승격**)
| 상수 | 0.5.2 | **0.5.3** | 실측 근거 |
|---|---|---|---|
| `RVA_RETREAT` | `0x1b94670` | **`0xe00350`** | 선두 12B push8 동일·orig_len 12 경계정확·rip-rel 無 |
| `RVA_FC59A0`(RECALL) | `0x1bdb3e0` | **`0xe168d0`** | 동상. `install_replace_detour_rax` 무조건설치 경로라 최우선 검증 대상이었음 |
| `RVA_CONDGATE` | `0x21338d0` | **`0xc550b0`** | 선두 **15B** 동일·orig_len 15 경계정확 |
| `RVA_MOVEPRI` | `0x2134240` | **`0xc559e0`** | ★**orig_len 13→12 필수** — 12.2 참조 |
| `RVA_DISC18_HANDLER` | `0x2376320` | **`0xd94d00`** | push8 동일·5,923B. `HARNESS_ON` 하 `install_wrap` 무조건 설치 |
| `RVA_DISC19_HANDLER` | `0x2380820` | **`0xdece30`** | push8 동일. 교차확증 = DISC7 desc `lea r9`가 함수 내 유일 사이트로 재발견(0.5.2 @`0x2382a95` ↔ 0.5.3 @`0xdeeebe`) |
| `RVA_GENERIC_BUILD` | `0x22b2280` | **`0xe06c10`** | push8 동일·22,764B(−7%) |
| `RVA_ITEMNET_SCORER` | `0x1b9cce0` | **`0x10587e0`** | ①프롤로그 12B 동일 ②**fn+12의 15B도 바이트 동일 = 가드 신원검증 통과 보장** |
| `LOADER_RVA`(ui_inject_embed) | `0x5ac950` | ~~`0x91ab0`~~ → **`0x2e1550`**(정정 07-29·확정) | ~~선두 12B push8 동일~~=**변별력 0의 오답**. 정답 근거 = 문자열-xref 만장일치 — 12.6 |
| `PARSER_RVA` | `0x24b5a00` | **`0x1a6530`** | 프레임만 `0x178`→`0x208` 확대·3인자 시그(rcx=out/rdx=text/r8=len) 불변 |
| `ALLOC_RVA` | `0x25c4d30` | **`0x28f7df0`** | ★★**시그니처 변경** — 12.3 |

데이터(.rdata): `RVA_C8C_DMG_SHEET` `0x381e1e0`→**`0x31be1a8`** · `RVA_DISC7_DMG_SHEET` `0x38d1918`→**`0x31bcef8`** · `RVA_TABLE_A` `0x3828818`→**`0x31c0168`** · `D19_TV7_RVA` `0x3863a28`→**`0x32105a8`**.
`TEXT_END_RVA`(`mem_safety.rs:310`) `0x2c087ff`→**`0x30a61ff`**(PE `.text` va `0x1000`·vsz `0x30a5200` → vsz_end `0x30a6200`). ⚠구값 방치 시 `.text` 후반 게터가 전부 "범위밖"으로 차단돼 **재현이 조용히 기본값으로 퇴화**.

#### 12.2 ★★`RVA_MOVEPRI` = **orig_len 13 → 12 필수**(안 고치면 즉사)
- 0.5.3에서 프롤로그가 **push6 → push4로 축소** ⟹ 명령경계가 `0,2,3,4,5,9,`**`12`**`,20`. 13으로 두면 `mov rax,[rsp+0xb8]`(8B) **한복판을 잘라** 트램폴린이 깨진 명령을 실행 = **확정 크래시**. `tfm2_ai_adjust.rs`의 `install_replace_detour` 호출부도 동반 수정 완료.
- ★ghidra-re **3중 확증**: ①디컴 시맨틱 동일(16엔트리 점프테이블) ②콜러 1:1(0.5.2 `FUN_1420d6e50` ↔ 0.5.3 `FUN_140d48ec0`가 MP×3+CG×2 **인터리브까지 동일**) ③**push 2개 감소가 스택오프셋 0x10 감소와 상쇄**돼 **entry_rsp 기준 인자 오프셋 `+0x28/+0x30/+0x38/+0x40/+0x50`이 완전 불변** ⟹ `mp_capture` **무수정**.
- ⚠case-body 내부는 레지스터 배정 변경(r15→r11, r14→rdi, 인덱스 rdi→rbx) — mid-func 사이트 재핀 시 마스크 필요.

#### 12.3 ★★`ALLOC_RVA` = 범용 래퍼 분해 → **impl 직접 호출 + 3인자 시그**(07-29 실측 정정 반영)
- ~~"0.5.3에서 `__rust_alloc` 래퍼가 **소멸**"~~ → **정정(07-29)**: 정확히는 **0.5.2의 범용 `__rust_alloc(size, align)`(align 분기 포함)이 사라지고 → `align`별 전용 심 + impl로 갈라졌다**(exe 전역에 `cmp rdx,0x11` 소형함수 0건인 것은 "범용 래퍼 부재"의 지문). 실제 호출은 **impl 직접 call 32,890 사이트**. impl `0x28f7df0`은 0.5.2 `0x25d9640`과 **명령별 완전 동일**(GetProcessHeap→HeapAlloc), 지문 `4c 89 c6 89 d7 ff 15 .. 48 85 c0 74 .. 48 89 c1 89 fa 49 89 f0`로 0.5.3 전역 **유일** 매칭.
- ⟹ `AllocFn`을 ~~`fn(usize,usize)->usize`~~ → **`extern "win64" fn(usize,u32,usize)->usize`**(rcx=미사용, edx=flags, r8=size)로 바꾸고 **`galloc(0,0,size)`** 호출(`ui_inject_embed.rs:32·41·276`).
- 교차확증: `realloc` `0x25c4dd0`→**`0x28e3b10`**(크기 174B·명령열 완전 동일 — item_tactics §11.1과 일치).
- ★**`ui_inject`를 공유하는 다른 T1 모드(item_tactics·comptest_unlock·elemental_serpen)에도 그대로 해당 = 그쪽 세션에 전파 필요.**
- ✅**~~충돌~~ 해소·일원화 확정(07-29 실측)**: item_tactics의 `0xbb2bd0`은 **align8 전용 1인자 심**이고 본문이 `mov r8,rcx; xor edx,edx; call 0x28f7df0` = **내부적으로 이 impl을 호출**한다 ⟹ **할당 경로는 동일·충돌 아님**. 차이는 **실패 처리뿐**: 심 = `handle_alloc_error` → **`ud2`(abort)** / impl = **null 반환**. ⟹ ★**호출부가 null을 정상 처리하면(ai_adjust `if new_ptr == 0 { return true; }`) impl 3인자 직접호출 `f(0,0,size)`가 정본**이다. 심을 쓰면 그 null 체크가 무의미해지고 **OOM 시 게임이 죽는다**. ⟹ ✅**item_tactics도 `0x28f7df0`+3인자로 교체·재빌드·배포 완료**(배포 dll `0x28f7df0` 3회 / `0xbb2bd0` 0회 — §11.4·§11.8).

#### 12.4 ★`probe_basedmg_r9` desc 화이트리스트 = **`OK_DESC_052` → `OK_DESC_053 = [0x31be1a8, 0x31bcef8]`**(`tfm2_ai_adjust.rs:1621`)
- **이걸 안 옮기면 전 호출 차단(dmg=0 퇴화), 구값을 통과시키면 AV** — ★**0.5.2 disc14 크래시 2·3차의 진범이 바로 이 상수 방치**였다(§7.2-A5와 동일 메커니즘).
- 두 desc 모두 sanity 실측 통과 `{size=0x6a8, align=8, vt+0x30=0xc51bc0}`. 0.5.3에서도 **`call [reg+0x28]` 대상 desc 전부가 vt+0x30 동일 = CGU 클론 간 등가**(0.5.2 결론 그대로 성립).

#### 12.5 byte-patch 사이트 ~~62 중 58 재핀 완료~~ → ✅**62/62 전량 재핀·라이브 확정**(정정 2026-07-30, 0.5.3) — 오프셋 이전 금지의 실례
- `detour.rs` 52사이트 중 **48 반영** + `disc19_repro.rs` 10사이트 중 **10 반영**.
- **sev 클러스터 4벌 전부 확정**: #1 `0xcd103f` / #2 **`0xd1af8f`**(컨테이너 `0x22e6460`→`0xd159f0`) / #3 `0xcd4c6f` / #4 `0xc82224`.
- ★**오프셋 비보존 실례**: sev#2의 `tr17`이 OLD **+28** → NEW **+24**(rel32 `ja`가 rel8로 축소) ⟹ §1 "컨테이너 안에서 재탐색" 규칙이 왜 강제인지의 실증.
- ★**교훈①(자동 문맥점수 매칭은 쌍둥이 사이트를 뒤집는다)**: disc19 ally **#1/#2가 실제로 역전 판정**됐고 실측으로 정정 — `div rcx`(64bit) 직후 = **#1 `0xded4d3`** / `div ecx`(32bit) 직후 = **#2 `0xded4df`**. ⟹ **#1/#2 쌍 구조 사이트는 반드시 앞 명령(`div rcx` vs `div ecx`)을 눈으로 확인**할 것. 나머지 `OK?` 7건도 전부 앞 문맥 실측으로 확정.
- ★★**교훈③(인게임 실증·2026-07-29) — \"주소만 고치고 prefix를 안 고쳐 조용히 죽는다\"**: 1차 인게임 로그가 `obj_imm` **8/12**·`gb_imm` **5/10**으로 나왔다. **주소는 옳게 재핀했는데 `patch_imm_bytes`의 prefix 배열을 0.5.2 그대로 둬서** 5개 사이트가 prefix 불일치로 skip된 것(**크래시 없음 = fail-safe 정상 동작**). 정정 내역 = `0xdecd48`·`0xdecd78`(dn_hp_low/hp_crit) 스택변위 `[rbp-0x28]`→**`[rbp-0x30]`** ⟹ prefix `48 83 7d d8`→**`48 83 7d d0`** / `0xdf9513`(dn_lane_margin) **`add r14`→`add r13`** ⟹ `49 83 c6`→**`49 83 c5`** / `0xe07610`(gb 라인range²) rbp 변위 `0x1b0`→**`0x270`** ⟹ `48 c7 85 b0 01 00 00`→**`48 c7 85 70 02 00 00`** / `0xe075c9`(gb 합류max거리²) **`41 b8`(mov r8d)→`b8`(mov eax)** 로 인코딩 **1B 축소** ⟹ ★**사이트 주소 자체가 +1 이동(`0xe075ca`)**·prefix `[0xb8]`·off **2→1**. ⟹ ★**규칙: 마스크 스캔(레지스터·변위 와일드카드)으로 주소를 찾았으면 prefix 배열도 반드시 실측값으로 같이 갱신**할 것 — 안 그러면 **크래시 없이 노브만 죽어 \"적용된 줄 알고\" 넘어간다**.
  - ✅**재발 방지 도구 신설 = `C:\tfm2mods\verify_sites_053.py`**: 소스의 `patch_imm_bytes` 사이트가 0.5.3 exe에서 **prefix 검증을 통과하는지 정적 확인**하고 `apply_*` 함수별 **예상 applied를 집계**한다. ★**인게임 로그 카운트와 정확히 일치함이 실증됨**(8/12·5/10 → 정정 후 **11/12·7/10**) ⟹ **다음 패치부터는 인게임 전에 이걸 먼저 돌릴 것.**
  - 부수 정정: `vis_imm` 로그 문구에 구주소 `@0x2126ae3(0.5.2)`가 하드코딩돼 있던 것 → **`@0xc43083`(0.5.3)** 로 수정(**패치 자체는 정상**, 로그 표기만 오해 소지였음).
- ★**교훈②(전역 순서대응은 위험)**: `vis_window`를 순서대응으로 `0x2558d08`에 잡았으나 실측 문맥이 전혀 달랐다(OLD `add rsi,0x258; cmp rsi,rax; setae al` ↔ 후보 `add rsi,0x258; mov rcx,rsi; 에필로그`). 원 12B 문맥시그 `48 81 c6 58 02 00 00 48 39 c6 0f 93`로 재탐색해 **`0xc43083`**(OLD 1건 == NEW 1건 유일)로 정정. ⟹ **문맥 시그 우선, 순서대응은 최후수단.** (다행히 구주소 `0x2126ae3`은 0.5.3에서 prefix 불일치라 방치해도 안전했음.)
- ~~⛔**미해결 4사이트 = 값 0.5.2 유지**(prefix 불일치로 skip = fail-safe, 노브만 죽음)~~ → ✅✅**전건 해결·정정(2026-07-30, 0.5.3)** — **byte-patch 62/62 전량 라이브**(인게임 실측 `obj_imm` **14/14** · `gb_imm` **9/9** · `sev_imm` **29/29** · `d19_imm` **10/10** · `vis_imm` **1/1** = **FAIL 0**, `verify_sites_053.py` 정적 예측과 인게임 로그 재차 일치):
  - ✅**`an_cull_dist` = 1사이트 → 3사이트로 분열**. 0.5.3 = **`0xd95603` / `0xd95693` / `0xd95717`**(전부 컨테이너 **`0xd94d00`**=disc18 후계 내부) · prefix **`49 81 f8`**(`cmp r8`; 0.5.2는 `49 81 fa`=`cmp r10`) · off **3** / width **4**.
    - **구조 변경**: 0.5.2는 헬퍼가 만든 **Vec 1개를 1회 스캔** → 0.5.3은 `[rbp+0x198]+idx*32`의 **(ptr,len) 3쌍을 인라인 체이닝**하는 3연속 루프. 셋 다 같은 본문(`cmp byte[r12],0` → `call 0xfcb660`)으로 합류 = **논리적으로 같은 한 스캔** ⟹ **3곳 전부 패치해야 0.5.2와 커버리지 동일**.
    - ★★**비교 극성 반전**: 0.5.2 `cmp r10,0x5f5e0; jbe(수락)` ⟺ 0.5.3 `cmp r8,0x5f5e1; jae(스킵)` ⟹ **임계에 +1 필요**(`u32c(cd).saturating_add(1)`). **상수 자체는 안 바뀌었다.**
    - ⚠**폐기(재조사 금지)**: 한때 후보였던 `0x12c4ce2`(`cmp rdi,0x5f5e0`·컨테이너 `0x12c4b70`)는 **무관한 별개 코드** — disc18/19 어디서도 호출되지 않는다.
  - ✅**컨테이너 0.5.2 `0x2398240`(거점헬퍼 op·scout) → 0.5.3 `0xcc3960`(size 3474) 확정**. 3중 교차 근거 = ①**콜러집합 동형**(0.5.2 `{0x2295760, 0x22b2280=generic_build, 0x236cb90}` ↔ 0.5.3 `{0xc9ce50, 0xd73d90, 0xe06c10=gb확정}`) ②**콜리 크기 지문 일치**(351B·547B 쌍) ③**5-way 완전 언롤 체인이 단일 루프로 롤업**(`0xcc3c98`, `inc r12; cmp r12,5`) — ★이것이 **스켈레톤·니모닉 코사인 매칭이 전부 실패한 원인**(구조가 바뀌면 형상 지문은 무력 ⟹ **콜러/콜리 관계 지문으로 전환**할 것).
    - **운영진입 phase 게이트** = **`0xcc3bbd`** · prefix **`48 83 b9 b8 00 00 00`**(`[rcx+0xb8]`; 0.5.2는 `49 83 be`=`[r14+0xb8]`) · off **7** / w **1** · 원본 imm `0x1f`.
    - **거점반경² = 2사이트 → 1사이트로 병합** = **`0xcc40e6`** · prefix **`48 b8`**(`movabs rax`; 0.5.2는 `49 b9`=`movabs r9`) · off **2** / w **8** · 원본 imm **`0x35a4e9000`**. 0.5.2는 같은 루프 임계값을 **프리헤더+latch 2곳에 호이스트**했으나 0.5.3은 **루프 본문 1곳뿐**.
    - ★★**극성 반전**: `jae(스킵)`·임계 `d²+1` → `ja(스킵)`·임계 `d²` ⟹ 인코딩을 **`sq1`→`sqd`** 로 바꿔야 한다(`e_sr2`). **sq1을 쓰면 반경이 1 어긋난다.**
    - ⚠**같은 함수의 `movabs r13, 0x53d1ac101`(=150000²+1, `0xcc4399`)은 다른 반경**이고 원래도 미패치 — **값만 보고 잡지 말 것**.
  - ★**일반화 교훈(다음 패치)**: 0.5.3 재컴파일은 **사이트 개수 자체를 바꾼다**(1→3 분열 / 2→1 병합) ⟹ "0.5.2와 같은 개수"를 전제로 한 순서대응·1:1 이식은 무효. 그리고 **imm이 같아도 비교 극성(jbe수락↔jae/ja스킵)이 뒤집히면 임계 ±1이 필요** — 값만 옮기면 **조용히 1 어긋난 채 동작**한다.
- ⏸`SIMUNCHUNK_RVA` `0x19b40c3` = 0.5.2값 유지(`detour.rs:880`): 12B 시그가 0.5.3 전역 0건(rayon 브리지 코드 변경). **원본바이트 재검증 후 패치라 ABORT = fail-safe**(1매치/job 분할 노브만 죽음).

#### 12.6 ⚠★~~미조정 충돌 2건~~ → ✅**2건 전부 해소·판정 확정**(2026-07-29, serpen·item_tactics 세션 독립 확증) — ★**서로 다른 독립 3방법(콜러 수 대조 / 경로 문자열 LEA→직후 call / 문자열-xref 수렴)이 모두 `0x2e1550`으로 수렴**
- ~~★**`LOADER_RVA`의 0.5.3 값이 모드마다 다르다**: ai_adjust `0x91ab0` ↔ item_tactics `0x2e1550` — 어느 쪽도 "확정"으로 쓰지 말 것~~ → ✅**해소·정정(0.5.3, 07-29)**: **`0x2e1550`이 정답, ai_adjust의 `0x91ab0`은 오답 확정**. 근거 3계통 = ①§13.3(경로 문자열 LEA→직후 call 도출법을 0.5.2에 돌려 `0x5ac950` 재현으로 방법 검증 + 콜러 사상 193/194 독립 일치 / `0x91ab0`은 copy 1개짜리 무관 함수) ②★**콜러 수 대조**(§11.6): 0.5.2 `0x5ac950` **507** ↔ 0.5.3 `0x2e1550` **511**(규모 일치) ↔ `0x91ab0` **2**(불일치) ③문자열-xref 16곳이 전부 `0x2e1550` 수렴·`0x91ab0` 0건.
  - ④**ai_adjust 세션 독립 4번째 확증(07-29·신규 도구 `C:\tfm2mods\loader_053.py`·재현 가능)**: `"asset/base/ui/layout/{main,strategy,training}"`를 `lea` 하는 `.text` 사이트 전수 → **직후 `call` 타겟 집계 = `0x2e1550` ×31 만장일치**(`0x91ab0` **0표**). ★**동일 절차를 0.5.2에 돌리면 알려진 정답 `0x5ac950`이 ×28로 재생산 = 방법 자체가 검증됨.**
  - **오답 원인** = ai_adjust가 채택 근거로 삼은 "선두 12B push8 완전동일"이 0.5.3 `.text`에 **66,635회** 등장 = **변별력 0**. ⚠★**파생 위험: `ui_inject`의 12B 프롤로그 검증은 이 오답을 못 거른다**(어차피 push8이라 통과) ⟹ **조용히 엉뚱한 함수를 후킹**. 프롤로그 검증을 신원 보증으로 오해 금지.
  - ★★**일반화 교훈(0.5.3 전반·다음 패치도)**: **프롤로그 일치는 신원 근거가 아니다**(전면 재컴파일로 흔해짐) ⟹ **문자열 xref · 고유 imm(movabs) · 호출관계처럼 변별력이 실증된 지문으로만 확정**할 것.
  - ~~⬜미조치 액션(다음 ai_adjust 세션 1순위)~~ → ✅**완료(07-29 ai_adjust 세션)**: 소스 교체 → 재빌드 → **배포 완료(23:12·3,354,112B)**, 배포본에 오답 `0x91ab0` **부재** 실측 확인(12.7). 남은 것은 **⬜인게임 검증뿐**(게임 완전 재시작 필요).
  - ★**공용 지시서 경고 신설 = `C:\tfm2mods\_MIGRATE_053.md` §1b "이 표의 알려진 오답"**: 표에서 `0x91ab0`으로 적힌 **전부가 재검증 대상** — ai_adjust `LOADER` / banpick_illust `RVA_ASSET_GET`·`RVA_ANIM_GET` / comptest_unlock `LOADER` / elemental_serpen `UILOADER_RVA` / item_tactics `LOADER`.
- ~~`ALLOC_RVA` 병존~~ → ✅**해소·일원화 완료**: 정본 = **`0x28f7df0` + 3인자 `(rcx=무시, rdx=flags(0), r8=size)->rax`·실패 시 0 반환**(12.3 도출 → §13.4 serpen 독립 재도출 2:1 → **item_tactics도 이 값으로 교체·재빌드·배포 완료**, §11.4). ⛔item_tactics 구안 `0xbb2bd0`(align 8 고정 심)은 **채택 안 된 대안**(OOM 시 abort라 null 체크가 死코드·align≠8 위험). ★**실측 확증(07-29 ai_adjust 세션)**: `0xbb2bd0` 본문이 `mov r8,rcx; xor edx,edx; call 0x28f7df0` = **심이 결국 이 impl을 호출** ⟹ 애초에 **경로 충돌이 아니었고 차이는 실패 처리뿐**(심=`handle_alloc_error`→**`ud2` abort** / impl=**null 반환**).

#### 12.7 상태 / ⬜잔여
- ✅**빌드·배포 완료**: ~~`tfm2_ai_adjust.dll` **3,354,112B** — 최종 배포본 = **2026-07-29 23:34:16(prefix 정정분)**~~ → ~~정정(2026-07-30): 최종 배포본 = 3,334,144B · 01:32:10(UI 주입 OFF 반영분)~~ → ★★**재정정(2026-07-30 밤 실측): 최종 배포본 = `tfm2_ai_adjust.dll` 3,338,240B · 2026-07-30 10:50:12**(vt슬롯 `0x1c8`·world `+0x40`·disc7 재가동/계측 반영분. 동봉 `설정편집기.exe` **8,382,976B · 11:08:12**, cfg **360줄**·`d7_repl=1`). 이전 **3,358,208B(00:37, 스텁 인벤토리 진단 인프라)** / **3,354,112B(00:00, byte-patch 62/62)** / 07-29 23:34:16·23:12:01은 전부 **상위 버전으로 대체됨**(stale 판단 금지) · **rustc 직접 빌드**(1MB 사이즈가드 초과 모드 — `-C opt-level=1 -C overflow-checks=off` + 링커 `rust-lld` 필수).
  - ✅**크래시 = 00:37 이후 0건**(여러 판 구동) · `crash_log.txt` mtime이 **00:35:13에서 정지**.
  - ✅**배포본 바이트 검증 5/5 통과**: `0x2e1550`(`50 15 2e 00`) 존재 / 오답 `0x91ab0`(`b0 1a 09 00`) **부재** / MOVEPRI `0xc559e0` 존재 / ALLOC impl `0x28f7df0` 존재 / desc C8C `0x31be1a8` 존재. `mod.mod_info` 첫 바이트 `0x7b`(BOM 없음).
- ✅**인게임 검증완 = 크래시 0**(2026-07-29 **23:44:52~23:47:10** 관측·게임 재시작 후 모드 활성화·로그 전량 갱신 확인 = **타임스탬프로 0.5.2 잔재와 구분됨**).
  - **적용 카운트**: `obj_imm` **11/12** · `gb_imm` **7/10** · `sev_imm` **29/29** · `d19_imm` **10/10** · `vis_imm` **1/1**(`@0xc43083` 0.5.3) · `sim_unchunk` **ABORT(의도된 fail-safe)** · `itemnet_guard` **설치됨·차단 0**.
  - **크래시 0 근거**: `_crash` 폴더 **비어 있음** · `crash_log.txt` mtime이 **23:00에서 안 움직임**(= 23:12·23:34 빌드로는 크래시 0). ★**23:00자 크래시 2건은 그 이전 22:56 빌드 것 = 0.5.3 마이그 결함이 아니다**(재조사 금지).
  - ⚠**확인 범위 = 로딩·byte-patch 적용 단계까지**. 재현 detour(retreat/fc59a0/condgate/movepri/disc18/disc19)의 **실제 발화는 미확인**.
- ✅~~⬜**경기 구동 검증(잔여)**~~ → ★**부분 해소(2026-07-30, 0.5.3)**: **disc7(플랜7=귀환) 재현이 실경기에서 발화·대조 완료 = 400건 중 396건 DIFF=0**(잔여 4건 = scan2 원본 위임 `path10` ↔ `PEND` 정확 일치)·**크래시 0**·**`d7_repl` 0→1 재가동(릴리스 zip에도 포함)**. ★★**되살아난 경위 = disc7 로직 무수정**, 같은 날의 **vt슬롯 `0x1b8`→`0x1c8`** + **world `+0x40`** 수정만으로 정상화(그 전엔 DIFF **7,365건 전부 `my=7 game=8`**) ⟹ **그 두 마이그 결함이 disc7 재현까지 오염시키고 있었다는 실증**. 오차 원인은 **scan2 경로 단독**(100% path7·zonebox/hp% 0건, 3런 교차 = scan2 발생런 100/400 ↔ 미발생런 0/400·재현 로직은 동일 = 차이는 경기 상황뿐) ⟹ **채택 해법 = scan2만 원본 위임**(`d7_mark(10,-99)` → `tfm2_ai_adjust.rs` **L6661**의 `code != -99` 가드가 대체 스킵·**손실 0**: scan2 블록엔 `tune()` 노브 0개, disc7 노브 3종은 path5/path8 소관·ΣDPS 계산도 함께 스킵). 계측 = `disc19_repro.rs` `d7_mark()`+`d7_diag.txt`(LOG_ON 무관 write·경로 태그 1~10). ⚠**flush 주기 교훈(버전무관)**: `tot % 500`이면 총 400호출에서 **데이터가 잘려 path4/7이 0으로 보인다** → `% 25`. ⬜**scan2 정밀 RE**(ΣDPS 합산 범위 또는 임계 `th`=**`w8+0x12f8`**·샘플 `hp=705 sumdps=13998 th=60 → ttd=50`) = 저우선. 상세 = `ANA\reimpl-tracker.md` 2026-07-30 disc7 항.
- ⬜**나머지 detour 경기 구동 검증(잔여)**: `mpcmp.txt`·`repl_status.txt`로 **movepri 대체 발화** 확인. ★**MOVEPRI 우선 관찰** — case-body 내부 레지스터 배정이 바뀐 것이 확인된 상태(r15→r11 · r14→rdi · 인덱스 rdi→rbx, 12.2).
  - ✅~~⚠**재시도 시 주의(07-30 추가)**: 진단 ON 상태에서 크래시 관측 이력(07-30 00:03 · `exe+0x12b95fb`)~~ → ★**해소(07-30 오전)**: 그 크래시의 **진범 = vt 슬롯 오후킹(`vt+0x1b8`→`+0x1c8`)** 으로 규명·수정 완료 ⟹ **`mpcap=1` 진단 캡처 복구**(실제로 disc7 400건 대조에 사용). 전문 = **§12.11(1)·(3)**. (원 문구) 현재 cfg는 **`log=0`·`mpcap=0`(프로덕션) 복귀** 상태이고 백업 **`tfm2_ai_adjust.cfg.bak_pre_053verify`** 보존.
  - ⚠**기존 로그 파일(2026-07-29 20:30자)은 0.5.2 실행 잔재**다(`sim_unchunk`가 `bytes=74a0 APPLIED`로 찍혀 있는데 그 시그는 0.5.3에 없음) ⟹ **0.5.3 검증 근거로 쓰지 말 것.**
- ✅~~⬜`disc19_repro.rs`의 **게임 vtable 슬롯 RVA 테이블**(match 아암 수십 개, 예 `0x9a1230`·`0x1eacc00`·`0x23bd370`·`0x1f23680`·`0x20958d0`) = **0.5.3 미재핀**. 미등재 시 fallback으로 떨어져 **크래시는 없고 재현 정확도만 저하**.~~ → ★**해소·재핀 완료(2026-07-30~31, 0.5.3)** = `disc19_repro.rs` **47종** + `serpen.rs` `c8c_cast_get` **9종** 재핀·폐기 4종. ★**이 표의 베이스는 0.5.2가 아니라 버전 혼재**(0.5.1 34 / 0.5.0_3 14 / 0.5.0_2 1 / 미상 3) = **0.5.1·0.5.2 두 번의 마이그에서 통째로 누락**돼 있었다. 전문 = **§12.12**.
  - ✅~~⛔**신규 잔여 = shadow-CALL 대상 게임함수 4종은 지문 매칭으로 재핀 불가 = ghidra-re 위임**~~ → **해소(2026-07-31, ghidra-re가 4종 전부 확정 = `0xcd3f00`·`0x16b70f0`·`0xc8e4e0`·`0xcc9960`, §12.12(3))**. ⚠VisionRoll 인자 **3→4개** 반영 필수.
- ✅~~⬜**provider/World `+0xeab8` 오프셋 점검 미실시**~~ → ★**해소(2026-07-30, 0.5.3): 소스 전량 치환 완료 = 49곳**(매핑 8쌍 `0xeab8→0xeaf8`·`0xeac0→0xeb00`·`0xeae9→0xeb29`·`0xeaf0→0xeb30`·`0xeb08→0xeb48`·`0xec98→0xecd8`·`0xecc8→0xed08`·`0xecd8→0xed18`). **재확인 실측(07-30 밤)** = 코드부 구 오프셋 잔존 **0곳** / 신 오프셋 코드부 **42곳**(`tfm2_ai_adjust.rs` 35 + `disc19_repro.rs` 7) / 두 exe disp 분포 대조 `0xeab8` 6→0 & `0xeaf8` 5 · `0xeae9` 2→0 & `0xeb29` 2 · `0xeaf0` 15→`0xeb30` 15 · `0xeac0` 43→`0xeb00` 47 ⟹ **A8_CACHE seed 키 무효화 우려도 해소**. 전문 = **§12.11(2)**.
- 유지=**inert**(재핀 불요 — 종전부터 신원검증에 걸려 미설치): `TG_CALL`·`THREATGATE_FN`·`F2_BUILD_CALL`·`GB_REGIOND_HOOK`·`GB_FUNNEL`·`COMMIT_CALL`·`COMMIT_FN`·`ENGAGE_GATE`·`D19_SLOT2_EMPTY`·`D19_STATIC_TEMPLATE`(`0x38d1af0`)·`D19_STATIC2_TEMPLATE`(`0x38d17b8`). **D19 3종은 empty-descriptor(전 0)라 `.rdata` 값지문 변별 불가** — `rd_u64`(VEH 가드) 읽기 전용이라 크래시 없음, 재현 정확도만 저하. `apply_lane_gate`·`type3_ablate`·`call_ablate`도 원본바이트 검증형이라 stale이어도 **ABORT=안전**.
  - ★**후속(07-29 serpen 세션 독립 실측)**: provider의 `+0x40` 시프트는 **실재 확정**(seed `0xeab8`→**`0xeaf8`**·SIM_TICK `0xeac0`→`0xeb00`·World→MobaMode `0xeaf0`→`0xeb30` 등 `0xea00~0xf000` 대역 전건) ⟹ ai_adjust의 `0xeab8` 7사이트는 **반드시 `0xeaf8`로 갱신**(A8_CACHE 교차오염 수정이 조용히 무효). 단 **삽입 지점은 `0xe000~0xea00` 사이**라 그보다 아래(엔티티 `0x40~0x400`·World 슬롯맵 `0x400~0x1000`·db `0x1000~0x2000`)는 **불변 = 같이 밀면 안 됨**(§13.2).

#### 12.8 ★신규 진단 인프라 = **스텁 인벤토리**(2026-07-30 신설 · **전 모드 이식 가치 있음**)
- **해결한 공백**: 우리 트램폴린은 `VirtualAlloc(RWX)`로 잡히므로 **어느 모듈에도 속하지 않는다** ⟹ 거기서 죽으면 WER은 `Faulting module name: unknown` + 절대주소만 남기고, `crash_log.txt`도 `exe+`/`MOD+` 매칭에 실패해 **단서가 0**이 된다(07-30 실측: 크래시 3건 중 **2건이 이 상태**).
- **구현**(`MODS\tfm2_ai_adjust\src\detour.rs`): 고정배열 **`STUB_TBL[24]`** + **`STUB_N`** 원자카운터 + **`stub_reg(addr,size,tag)` / `stub_lookup(addr)` / `stub_dump()`**. **`VirtualAlloc` 실행가능 할당 12지점 전수 등록**(훅 6종 + `ret_thunk`·`shim_both`·`shim_rdx`·`call_stub`·`alloc_near` + `uinj`). **tag = 타깃 RVA**.
  - 크래시 문맥에서 도는 `stub_lookup`은 **고정배열 순회만**(alloc/lock/`format!` 없음 = CLAUDE.md §3 메모리안전 수칙 준수).
  - `crash_log.txt`가 RIP를 자동으로 **`= STUB(tag=0x…)+off`** 로 역해석한다.
  - `hooks.txt` = **`LOG_ON` 무관 직접 write** ⟹ **진단 로그를 꺼둔 프로덕션에서도 훅 설치 현황 확인 가능**.
- ★★**이 인프라로 얻은 실증(가장 중요) = 0.5.3 마이그한 훅 7종이 전부 실제로 설치됨**(`hooks.txt` 확인): `retreat 0xe00350` / `fc59a0 0xe168d0` / `condgate 0xc550b0` / **`movepri 0xc559e0`** / `itemnet guard 0x10587e0` / `disc18 0xd94d00` / `disc19 0xdece30` (+보조 3종) ⟹ **재핀 주소가 유효하다는 직접 증거**. 특히 **movepri는 `orig_len` 12가 맞았다는 뜻**(틀렸으면 즉사, §12.2)이고, **`itemnet guard`는 0.5.2에서 stale로 미설치였던 것이 복구된 실증**이다. `generic_build`·`gb region-D` 부재는 `INSTALL_DIAG_HOOKS=false`라 **설계대로**.

#### 12.9 ⏸**UI 주입(개인전술 "AI 버튼") 당분간 중단** — 유저 지시 2026-07-30
- `tfm2_ai_adjust.rs:25` **`const UI_INJECT_ON: bool = false;`** 게이트 신설, 호출 4곳 전부 감쌈(init `uinj::install` · `on_init` 백업설치 · `post_update` `uinj::tick`).
- 꺼진 상태에서는 **게임 UI 로더를 후킹하지 않고 `ui_inject.txt`도 읽지 않는다** ⟹ **`LOADER`/`PARSER`/`ALLOC` RVA 불확실성이 통째로 유예**된다(0.5.3 LOADER는 clone family라 프롤로그로 변별 불가 = 오후킹 위험이 있던 자리, §12.6). **재개는 이 한 줄만 `true`**.
- ⟹ 종전 "**`LOADER` 재검증 필요**"는 **ai_adjust 한정으로 유예**(모드가 안 씀). 단 **다른 모드(`banpick_illust`·`comptest_unlock`·`elemental_serpen`·`item_tactics`)의 `0x91ab0` 재검증 ⬜는 그대로 유효** — `python C:\tfm2mods\loader_053.py` 로 확인 가능.

#### 12.10 ★크래시 진범 규명(2026-07-30·재조사 금지) + cfg 편집 사고 교훈
- **2026-07-29 하루 APPCRASH 총 13건 중 10건이 `tfm2_banpick_illust.dll`에서 직접 폴트**(22:57~23:18). ★**`tfm2_ai_adjust`는 13건 중 단 한 번도 faulting module로 지목되지 않았다** ⟹ **ai_adjust 축 재조사 금지**. 나머지 3건 = **exe 내부 1건**(`0x12b95fb`, `mov rsi,[r12+rax*8+8]` 배열 인덱싱) + `unknown` **2건**(=§12.8이 해결하려는 그 상태).
  - ⚠유저는 `banpick_illust` 0.5.3 마이그를 이미 했다고 함(배포본 **23:52자**)인데 **그럼에도 크래시가 그 dll에서 났다** ⟹ ⬜**banpick_illust 쪽 잔여 문제**(별도 세션 대상·잔여트래커 등재). ⚠**단 크래시 시각(22:57~23:18)이 배포(23:52)보다 앞서므로 "0.5.2 구본 dll 잔존" 설명과도 정합** = 어느 쪽인지 **미확정·사실 승격 금지**(23:52 이후 크래시 유무부터 확인할 것).
- ★★**정정·후속(2026-07-30 오전, §12.11)**: 위 "나머지 3건" 중 **exe 내부 폴트 `exe+0x12b95fb`(07-30 00:03, 진단 ON 상태)** 는 원인 미상이었으나 **진범 규명 완료 = ai_adjust의 vt 슬롯 오후킹(`vt+0x1b8` → `+0x1c8`)** — 상세 = **§12.11**.
- ⚠★**cfg 편집 사고 교훈(버전무관·꼭 준수)**: PowerShell `-replace`에 **3요소를 넘겨 파싱 에러**가 나면서 `ForEach-Object`가 **해당 줄을 통째로 유실**시켰고, `Set-Content -Encoding UTF8`이 **BOM(`EF BB BF`)을 붙였다**. BOM은 게임 파서 실패 → **모드 강제 비활성**을 유발하는 문서화된 함정(CLAUDE.md §2). 백업에서 **바이트 복구** 후 **파이썬으로 원본 BOM 정책을 유지하며 재치환**해 해결(줄수 331→331·차이 정확히 2줄·첫 바이트 `0x23`). ⟹ ★**게임이 읽는 cfg/json은 PowerShell `Set-Content`로 쓰지 말 것. 파이썬 바이트 단위 쓰기 + 백업 대비 diff 검증이 표준 절차.**

#### 12.11 ★★진단 캡처(mpcap) 크래시 진범 = **vt 슬롯 오후킹** + world/provider `+0x40` 소스 반영 + mpcap 복구 (2026-07-30 오전 · 재조사 금지)
> ⚠**이 절은 2026-07-30 밤 기록 감사에서 "보고됐으나 파일에 없음"이 발견돼 복구 기재한 것**(근거 = 소스 주석 실물 `tfm2_ai_adjust.rs` L3523~3532 + 배포 산출물 실측). 최종 배포 dll = **3,338,240B · 2026-07-30 10:50:12**.

**(1) vt 슬롯 재시프트 `0x1b8` → `0x1c8`(=이번 크래시의 진범)** — `tfm2_ai_adjust.rs` **L3532**(`rd_u64(vtab+0x1c8)`), 주석 = L3523~3531.
- 0.5.3에서 이 vtable에 **`+0x1b8`·`+0x1c0` 두 메서드가 신설 삽입** ⟹ **`≥0x1b8` 슬롯이 전부 `+0x10` 시프트**(`<0x1b8`인 `0x28`/`0x118`/`0x138`/`0x140`/`0x150`/`0x168`/`0x1a0` 등은 **불변**). 객체 크기도 **`0xee88` → `0xeec8`**.
- 후계 확정 근거 = **0.5.2 `vt+0x1b8`(`0x2305520`, 핸들→엔티티 리졸버 61B)** 와 **0.5.3 `vt+0x1c8`(`0xee7d00`)** 가 **바이트 완전동일**.
- ⚠★**진범 메커니즘**: 0.5.3의 `vt+0x1b8`은 **sret 7인자 격자질의(`0x12b9480`)** 로 바뀌었는데 **그것도 유효 코드포인터라 `ptr_ok`를 통과** ⟹ 그대로 shadow-call되어 `rdx`(=핸들 정수)가 `&self`로 오용 → **`fn+0x17b`에서 AV**. 실측 폴트 **`exe+0x12b95fb`가 그 지점과 정확히 일치(재현 2/2)** ⟹ §12.10의 "exe 내부 1건(원인 미상)" 및 §12.7의 "진단 ON에서 크래시 이력" **둘 다 이 건으로 설명·해소**.
- ⚠**mpcap 전용 문제가 아니었다**: disc0/1/3 재현은 대체 여부와 무관하게 **매 판단마다 실행**되므로 라이브 경로도 같은 지뢰를 밟을 수 있었고, `mpcap=1`은 화이트리스트 밖 disc까지 돌려 **노출을 키웠을 뿐** ⟹ 프로덕션 안전상 필수 수정이었다.
- ★★**일반 교훈(버전무관·전 모드)**: **`ptr_ok`는 "유효 포인터"만 보증할 뿐 "그 슬롯이 내가 원하는 함수"임을 보증하지 않는다** — 슬롯 인덱스가 밀린 vtable에서는 **엉뚱한 함수가 조용히 shadow-call**된다(07-23 주석이 경고한 사고의 재발). ⟹ 패치 후엔 **슬롯 대상 함수의 바이트 동일성**(구버전 슬롯 함수 ↔ 신버전 후보)까지 대조할 것.

**(2) world/provider 구조체 `+0x40` 시프트 = ai_adjust 소스 전량 반영 완료** (종전 §12.7 ⬜"점검 미실시" 해소)
- 적용 매핑 8쌍 = `0xeab8→0xeaf8`(seed) / `0xeac0→0xeb00` / `0xeae9→0xeb29` / `0xeaf0→0xeb30` / `0xeb08→0xeb48` / `0xec98→0xecd8` / `0xecc8→0xed08` / `0xecd8→0xed18` — **치환 49곳**.
- ✅**실측 검증(2026-07-30 밤 재확인)**: 소스 **코드부의 구 오프셋 잔존 0곳**(주석의 취소선 표기만 남음) · 신 오프셋 코드부 **42곳**(`tfm2_ai_adjust.rs` 35 + `disc19_repro.rs` 7). 두 exe disp 분포 대조 = `0xeab8` 6→**0** & `0xeaf8` 5 / `0xeae9` 2→**0** & `0xeb29` 2 / `0xeaf0` 15 → `0xeb30` **15** / `0xeac0` 43 → `0xeb00` **47** ⟹ §13.2 serpen 실측(대역 `0xea00~0xf000`만 `+0x40`·저역 불변)과 정합.
- ⟹ 07-29자 우려(“A8_CACHE seed 키가 `0xeab8`이라 교차오염 수정이 조용히 무효화”)는 **해소**.

**(3) mpcap(진단 캡처) 복구 + disc7 연결**
- (1) 수정으로 **진단 ON(`mpcap=1`) 상태의 AV가 제거**돼 **movepri 캡처 대조가 다시 가능**해졌고, 그 캡처 로그가 **disc7 재현 대조(400건 단위)** 의 기반이 됐다.
- ★**(1)+(2)만으로 disc7 재현이 정상화**됐다(disc7 로직 무수정. 그 전엔 DIFF **7,365건 전부 `my=7 game=8`**) ⟹ **마이그 결함이 "무관해 보이는 재현 함수"를 조용히 망가뜨린다는 실증**. disc7 후속(scan2 원본 위임·396/400 DIFF=0·`d7_repl=1`)은 **§12.7 ✅부분 해소 줄** 참조.

#### 12.12 ★★게임 **vtable 슬롯 RVA 테이블 0.5.3 재핀 완료** + 신규 발견 "shadow-CALL 6종도 미재핀" (2026-07-30~31, 0.5.3 buildid 24451609) — **본 절 = 이 건의 정본** / 모드 작업 문서 = `REPORT\tfm2_ai_adjust\`(`RE\2026-07-30_vtable슬롯표_0.5.3재핀.md`)

**(1) ★핵심 정정 = 이 표의 베이스는 0.5.2가 아니라 "버전 혼재"였다**
- 52종 실측 분류 = **0.5.1 34종 / 0.5.0_3 14종 / 0.5.0_2 1종 / 미상 3종** (도구 `vtslot3_053.py`). 0.5.2 exe에서 **52종 중 48종이 함수 시작조차 아님**.
- ⟹ **0.5.1·0.5.2 두 번의 마이그에서 이 표가 통째로 누락**돼 있었다. 미매치가 **fallback(제네릭 바이트 디코더)으로 조용히 흡수**돼 크래시·로그가 0이라 **두 버전 동안 발견되지 않았다**.
- ⟹ 종전 §12.7의 "0.5.2 → 0.5.3 미재핀(정확도만 저하)" 표현은 **범위 과소평가**였음(정정).

**(2) 결과 = 확정 47종 / 폐기 4종 / `serpen.rs` 9종 추가**
- `disc19_repro.rs` = **47종 확정**. **폐기 4종** = 0.5.0_3 세대 구현이 0.5.3의 슬롯 `0x78`에 없는 것들로, **후계가 이미 표에 등재돼 있어 커버리지 손실 0**.
- ★**"이 파일의 표"가 아니라 "이 종류의 표"를 찾을 것** — 같은 슬롯표가 **`serpen.rs` `c8c_cast_get`에도 존재**했다 ⟹ **9종 추가 재핀**(신규 매핑 `0x19ed660`→**`0xee4130`** 포함).
- 매핑표 = `C:\tfm2mods\_vtslot_053_final.md` · 데이터 = `_vtslot_053_map.json`·`_vtslot_053_diff.json`.

**(3) shadow-CALL 대상 게임함수 6종도 미재핀이었다(신규 발견)** — ~~2종 확정 / 4종 = 지문 매칭 방식으로는 재핀 불가·재시도 금지~~ → ★**전수 확정(2026-07-31 후속 세션·ghidra-re)**
- ✅재핀 확정 2종(지문 매칭): **`0x1fce700`→`0xfdd430`**(명령 125↔125·니모닉 불일치 0·크기 448 동일·콜러 47↔48) · **`0x1fbe950`→`0xfcb940`**(147↔147·크기 531·콜러 47↔48).
- ~~⛔**미해결 4종 = `0x20a3fd0`·`0x1c974a0`·`0x237d910`·`0x236b6b0`**(마지막 것은 **0.5.1 값** = 두 버전 방치). 전체 지문·앞 12명령 지문 **둘 다 후보 0 또는 크기·콜러 불일치** ⟹ ★**ghidra-re 위임 대상**. **같은 도구(지문 매칭) 재실행 금지.**~~ → ✅**정정·해소(2026-07-31, 0.5.3): ghidra-re가 4종 전부 확정**

| 구 RVA(베이스) | **→ 0.5.3** | 확정 근거 |
|---|---|---|
| `0x20a3fd0`(0.5.0_3) d19_threat | **`0xcd3f00`** | disc19 핸들러 `0xdece30` 콜사이트의 인자식이 소스 `scan_w` 도출식과 **문자 그대로 일치** + 고유상수 전량 + 콜러 7↔7 |
| `0x1c974a0`(0.5.0_3) vt`+0x90` 계산게터 | **`0x16b70f0`** | **29바이트 전체 동일** + `imul r64,[rcx+0x108]`이 양 버전 `.text` 전역 **각 1건** + vtable `+0x90` 등재 실증 |
| `0x237d910`(0.5.0_3) VisionRoll | **`0xc8e4e0`** | 콜러 15↔15 + splitmix64 상수·bias식 전량 일치 |
| `0x236b6b0`(**0.5.1**) Fn2090 | **`0xcc9960`** | disc19 핸들러의 7인자 콜사이트 실측 + `out+0x28`/`out+0x29` 계약 일치 |

- ★★**지문 매칭이 실패한 원인(교훈·버전무관)** = 0.5.3의 **AI 크레이트 분리로 이 함수들이 `0x14xxxxxx` → `0x140cxxxxx` 대역으로 통째 이동**했고 크기·콜러 후보창에서 이탈했다. 게다가 **`0x1c974a0`은 leaf라 `.pdata` 엔트리가 없어** 함수시작 카탈로그 기반 매칭에 **원천적으로 안 잡힌다**(29B가 전 버전 동일한데도 못 찾은 이유) ⟹ ★**카탈로그에 없다 = 부재가 아니라 leaf일 수 있다**(`.pdata` 미수록 함수는 별도 스캔 필요).
- ⚠★★**VisionRoll(`0xc8e4e0`)은 인자가 3개 → 4개**(선두에 dead 인자 삽입 = `(dead, p5, p6, e)`) ⟹ **미반영 시 shadow-CALL 즉시 AV**. 소스 `F237d` **타입·호출 모두 수정 완료**.
- **구조체 필드 오프셋은 4종 모두 무변경.** 단 ⓐ**`0xcd3f00`은 선두 phase 게이트(`cmp rcx,0x1e`)가 삭제**돼 **rcx가 dead** ⟹ shadow 롤백(원본 호출로 되돌릴 때) 시 `phase < 0x1e → 0` 가드를 우리가 넣어야 한다 ⓑ**`0xc8e4e0`은 "이미 발각/최근 목격" 전치 게이트가 새로 생겼다** ⟹ **`d19_g1_pred_pure`가 그만큼 어긋난다**(과다 visible 방향·⬜보정 미실시).
- ✅**별건 확인 = ghidra-re가 "유력(2표본)"으로 보고한 geom vtable `+0x68` 시프트는 이 모드에 해당 없음** — `geom_vt68`·`geom_vtc0`는 **이름만 슬롯 번호를 딴 것**이고 실제로는 **필드 직독**(`gc+0x738`/`gc+0x740`·`gc+0x840`/`gc+0x848`)으로 이미 순수 재현돼 있다 ⟹ ⛔**이 축 재의심·재조사 금지.**
- 🟢(정정 전 기록) 4종은 그동안 **`code_ptr_ok()` 가드 + 기본 스위치 OFF**(`d19thr`·`d19_g1_shadow`·`D19_G1CF_SHADOW`=false)라 **리스크 0**이었다. 스위치를 켜려면 위 재핀 + VisionRoll 인자 4개화가 반영된 빌드여야 한다.
- 원본 RE 결과 = `REPORT\tfm2_ai_adjust\RE\2026-07-31_shadowcall-4종-0.5.3-재핀.md`.

**(4) ★★주소만 바꾸면 안 되는 사례 2건(전체 명령 disp 대조로 발견)**
- **`0x1d1f630`→`0xf01df0`** = walker의 ptr/len 오프셋이 **`0x68`/`0x70` → `0x50`/`0x58`**, **`0x80`/`0x88` → `0x68`/`0x70`**(**−0x18**).
- **`0x1dce1d0`→`0xf14c60`** = flat/ratio 필드가 **`0x10`/`0x18` → `0x00`/`0x08`**(**−0x10**).
- 두 건 모두 **재현 로직도 함께 수정**했다. ⟹ ★**재핀 = "주소 찾기 + 그 함수가 읽는 필드 오프셋 확인" 둘 다**(아래 §(6)②).
- 부수 확인 = **`call [r9+0x1b8]`→`[r9+0x1c8]` 시프트가 여러 함수에서 재관측 = §12.11 vt 슬롯 시프트의 독립 재확인**. 그 외 `[r9+0x40]→[r9+0x48]`·`[r9+0x48]→[r9+0x50]`·`[rdx+0xc8]→[rdx+0x110]`은 **게임 내부 vtable = 우리 로직 무영향**.

**(5) 빌드·배포·검증(실측)**
- `build_full.ps1 -MaxSize 4000000` **exit 0** → 배포 dll **3,336,192B · 2026-07-31 00:23:37**(직전 3,338,240B · 07-30 10:50:12).
- 배포본 바이트 검증 = **신 RVA 48/48 존재 · 구 RVA 53/53 부재**.
- ~~⬜**이번 변경분 인게임 재검증 미실시**. 확인 포인트 = disc19 진단 로그의 슬롯별 `미등재={}` 카운트 감소.~~ → ✅**종결(0.5.3, 2026-07-31 세션 후반)**: 검증 기준을 **"회귀 없음 + 크래시 0 + 배포본 구 RVA 잔존 0"** 으로 갈음(전부 확인 완료 = §12.13(3) + §12.12(5) 바이트 검증). ⛔**"비트동일 실증"은 대상 선정 오류였다** — disc19 재현은 관찰단계라 비트동일이 목표인 단계가 아님(§12.14(4)).

**(6) ★★일반화 교훈(버전 무관 — 다음 패치에 꼭 적용)**
1. ★**마이그 대상은 "상수 파일"이 아니라 "소스 전체의 하드코딩 RVA"다.** 흩어진 **match 아암 표**는 마이그 목록에서 빠지고, **조용한 fallback** 때문에 몇 버전이고 방치된다 ⟹ ★**다음 패치엔 `C:\tfm2mods\rvascan_053.py`(소스 전체 미재핀 RVA 감사)를 제일 먼저 돌릴 것.**
2. ★**재핀 = 주소 + 필드 오프셋, 둘 다.** 쌍이 확정되면 **전체 명령 disp 대조**(`vtslot_diff_053.py`)를 반드시 돌릴 것 — 실측: `strict` 지문으로 잡힌 **43쌍은 오프셋 불변**, `loose`/`mnem`으로 잡힌 **2쌍은 2/2가 오프셋 변경**.
3. **지문은 대상에 맞춰 2단계로**: 필드 변위 보존(**strict**) → 실패 시 전 숫자 마스킹(**loose**). **게터류는 숫자가 곧 신원**이라 전부 마스킹하면 서로 뒤섞인다.
4. **함수 경계는 `.pdata` 우선** — 휴리스틱만 쓰면 다음 함수까지 이어붙여 비교 결과가 전부 쓰레기가 된다(**불일치 7건이 경계 수정 후 1건**으로 줄었다).
5. **"이 파일의 표"가 아니라 "이 종류의 표"를 찾을 것**(위 §(2) `serpen.rs` 실례).

**(7) 신설 도구 (전부 `C:\tfm2mods\` · 다음 패치 재사용)**
- ★**`rvascan_053.py`** = 소스 전체 미재핀 RVA 감사(**다음 패치 1순위**) · `vtslot3_053.py`(베이스 버전 특정) · `vtslot4_053.py`(슬롯 오프셋 지문) · `vtslot5_053.py`(슬롯 분포 대조) · ★**`vtslot7_053.py`**(자동 매핑 본체) · `vtslot8_053.py`(전역 지문) · `vtslot9_053.py`(CGU 복제본 슬롯 배정) · ★**`vtslot_diff_053.py`**(쌍별 전체 명령·disp 대조) · `shadowcall_053.py`·`shadowcall2_053.py`.

#### 12.13 ★★movepri 점프테이블 전체(0.5.3) + **"disc11 DIFF 2,591건" 오보 정정** + 후속 인게임 검증 (2026-07-31 후속 세션) — **본 절 = 이 건의 정본** / 모드 문서 = `REPORT\tfm2_ai_adjust\RE\2026-07-31_disc11-movepri-점프테이블.md`·`03_시행착오.md`

**(1) ⛔오보 정정 = "subplan=11 DIFF 2,591건 = 신규 결함"은 틀렸다(메인 세션의 오독) — 재조사 금지**
- **진상 = 계측 오배선.** 게임 핸들러 **공통 에필로그(0.5.3 `0xc55d37`)** 가 `mov qword [out], 0xb` 로 **enum tag를 상수 11로 고정**하고, **실제 판단 결과는 payload `out+8`** 에 넣는다 ⟹ **재현(payload) ↔ 대조(tag)** 가 **서로 다른 량을 비교**한 것. **재현은 정상.**
- ★**이건 이미 `tfm2_ai_adjust.rs` L6502~6504(2026-07-23)에 적혀 있던 결론**이다("disc10·disc11의 mpcmp OK/DIFF는 무의미, **정본 지표는 pokecmp**"). 0.5.3에서 **구조가 동일함이 재확인**된 것뿐.
- ★**근본 조치(코드 수정·배포완)** — 주석만으로는 재발을 못 막는다는 것이 **2세션 연속 오독으로 실증**돼, `detour.rs` 판정부에서 **disc10·11을 OK/DIFF 집계에서 제외** + **`[N/A:tag고정]` 표기** + `mpcmp.txt` **헤더 경고 2줄** + 신설 카운터 **`MP_NA`**. 배포 dll **3,336,704B · 2026-07-31 01:58:36**.
- ★**역증명** = 재현값 집합 **{3,4,6,11,14,15,20}** 이 실제 핸들러 **`0xe030d0`의 반환 char 집합과 정확히 일치**.

**(2) ★0.5.3 movepri 점프테이블 전체(신규 구조 사실·재사용 가치 큼)**
- 디스패처 **`0xc559e0`** · 테이블 RVA **`0x31a7494`**(16엔트리) · 인덱싱 **`idx = (disc<2) ? 1 : disc-2`** 가 **0.5.3에도 유효**.
- ⚠**`idx0 = disc2`** 이고 **disc0·1은 idx1 = disc3 블록으로 합류**한다.

| disc | 0.5.3 핸들러 | disc | 0.5.3 핸들러 |
|---|---|---|---|
| 0 / 1 / 3 | `0xd803f0` | 12 | `0xc6e080` |
| 4 | `0xd71630` | 13 | `0xd81990` |
| 7 | `0xdff660` | 14 | `0xcb2340` |
| 9 | `0xe0f740` | 15 | `0xc7a550` |
| 10 | `0xd81be0` | 17 | `0xdec6b0` |
| **11** | **`0xe030d0`** | 2·5·6·8·16 | **인라인**(8=7 고정 / 6=`0xa` 고정 / 5=복사) |

- **`add rdx,8`(subp+8 전달) 있음** = disc4·7·9·12·13·14·15·17 / **없음(rdx 원본)** = **disc0·1·3**.
- ★★**disc18(`0xd94d00`)·disc19(`0xdece30`)는 이 테이블에 없다** — 별도 디스패처 **`FUN_140d945a0`** 소속 ⟹ **movepri disc0~17과 서로 다른 enum 번호공간**(두 축의 번호를 섞어 해석하지 말 것).
- **disc11 정체 = `game-ai\src\plan_legacy\old\line_gank\cover.rs`**(panic 문자열 실판독).
- ⬜**유력(미확정·사실 승격 금지)** = `my_serpen_poke`의 cand 해석이 0.5.3 게임과 다름 — 게임 = `*(l80 + side*0x28 + 0x1e0 + idx*8)` **직접배열**(`idx = *(SimState+0x8b0)`) / 모드 = `dd7_slot128` **핸들 3단 resolve**. 정적으로는 같은 대상으로 보이나 **런타임 미대조**.

#### 12.14 ★★리플레이 A/B로 **disc16/17 대체 "실효 있음" 확정** + ⛔"disc18/19 미발화" 오판 정정 + ⛔SubPlan 디스패처 `0xd98740` 후킹 금지 (2026-07-31 세션 후반, 0.5.3 buildid 24451609) — **본 절 = 이 건의 정본** / 모드 문서 = `REPORT\tfm2_ai_adjust\03_시행착오.md`·`02_구현정보.md`(§6 진단 플래그 지도·§7 A/B 절차)·`RE\2026-07-31_disc18-19-발화조건-SubPlan디스패처.md`·`RE\2026-07-31_SubPlan승격게이트-패치사이트.md`

**(1) ✅★★리플레이 A/B 실험 = disc16/17 대체가 게임 결과를 바꾼다(실효 확정) — 방법론 자산**
- **착상(유저 제안)**: 리플레이 = **시드 재시뮬레이션이라 결정론적** ⟹ 같은 다시보기를 **토글만 바꿔 두 번** 관전하면, 개입에 실효가 있을 때만 결과가 갈린다.
- ★**전용 토글 `d1617_repl` 신설**(cfg·기본 `1` = 종전 동작). ⚠전체 `mp_repl`을 끄면 **disc0/1/3/9/11 효과가 섞여 인과 특정 불가** ⟹ **16/17만 격리한 것이 이 실험의 핵심**.
- **결과**: OFF(게임 원본 판단) **16:1** — **같은 조건 2회 모두 동일 = 리플레이 결정론 실증** ↔ ON(우리 재현 대체) **14:2**. 계기 카운터도 뚜렷 = `d17_calls` **400(캡처만) → 54,942(대체)**.
- ⟹ ✅**disc16/17 대체는 실효 있다.** "대체가 사실 무효인 것 아니냐"는 의심은 **기각**.
- **부수 확정** = OFF 조건은 **게임 원본과의 직접 대조**를 제공한다 ⟹ disc17 재현 **`my=19 → game_code=19` OK 3,755 / DIFF 2** = **disc17 재현 정확도 실증**.
- ★★**방법론 채택(버전무관·전 모드 재사용)** = **리플레이 A/B = 개입 실효를 인과적으로 판정하는 표준 절차**. 조건 3가지 = ①**토글을 최소 단위로 격리**(전체 스위치 금지) ②**같은 리플레이** ③**먼저 같은 조건 2회로 결정론부터 확인**. → [[tfm2-replay-runbook]]

**(2) ⛔★★정정 = "disc18/19 훅이 설치되고도 미발화 = 국면 미도달"은 오판이었다**
- **진상** = 로그가 cfg **`dcap` 게이트 뒤**에 있었을 뿐. **`dcap=1`** 로 켜자 즉시 **`disc1819cap.txt` 43,401B · `disc19cmp.txt` 152,216B 생성** ⟹ **훅은 처음부터 계속 발화 중**이었다.
- **오판 원인** = `disc19_capture` 본문만 보고 "게이트 없이 무조건 쓴다"고 단정. 실제 게이트는 **한 겹 안쪽** `dump_disc_commands` 첫 줄 `if !DISC1819_CAP.load() { return; }`(**`disc19_repro.rs:79`**).
- **치른 비용** = 이 오판 위에 **ghidra-re 2건 + SubPlan 디스패처 후킹 크래시 1건 + 유저 조합테스트 4회**가 쌓였다.
- ★★**교훈(버전무관)** = **"로그가 없다"를 "코드가 안 돌았다"로 읽지 말 것.** 로그 경로의 게이트를 **끝까지 따라간 뒤** 판단하고, **검증 착수 시 진단 플래그 전수 점검을 선행**할 것(플래그 지도 = `REPORT\tfm2_ai_adjust\02_구현정보.md §6`).

**(3) ⛔★SubPlan 디스패처 `0xd98740` 후킹 = AV 크래시·방식 무관 재시도 금지**
- passthrough·**read-only 계측 wrap**(`install_wrap`·orig_len 12 경계 확인)이었는데도 **인게임 AV 2회 재현** = `code=0xc0000005` · **RIP `exe+0xc4225e`** · `faultAddr=0x0` · 콜러 `exe+0xca92ed`. 폴트 지점은 `0xc421b0`의 **`cmp qword [r15+rax],0`(r15=null)** = **후킹 지점과 무관한 함수**.
- **원인(유력)** = `0xd98740`은 프롤로그 직후 `lea rbp,[rsp+0x80]` → **`mov qword ptr [rbp], 0xfffffffffffffffe`** 로 **SEH 예외 프레임을 설정**한다 ⟹ 트램폴린이 **unwind 정보와 어긋난다**.
- 소스에 **`const SPDISP_PROBE: bool = false`** 로 영구 차단. ⛔**방식 무관 재시도 금지**.
- ★★**교훈(버전무관·전 모드)** = **"passthrough라서 안전"은 틀렸다** — 트램폴린을 새로 박는 행위 자체가 위험. ⟹ **SEH 프레임(`mov [rbp],-2`)을 세우는 함수는 후킹 대상에서 제외**(프롤로그 경계가 맞아도).
- 단 크래시 전 **5,269건 계측에는 성공** = `subplan_dispatch: total=5269 | 0:155 1:95 2:3692 4:93 6:7 7:247 8:912 11:68` (**18/19 = 0건**).

**(4) ⛔"vtable 재핀 비트동일 실증" 잔여 = 대상 선정 오류로 종결**
- **진상** = disc19 재현은 **관찰단계(미완성)** 다 — `disc19cmp.txt` 헤더가 **`(관찰단계: 전반A+tag3+후반B골격)`**, 실측 159건 중 **항목 일치 3 / 불일치 643**. 애초에 **비트동일이 목표인 단계가 아니었다**.
- ⟹ **§12.12 vtable 재핀의 검증은 "회귀 없음 + 크래시 0 + 배포본 구 RVA 잔존 0"으로 갈음**(전부 확인 완료) = **이 잔여 종결**.
- ⬜**남은 정식 지표 = `bdcmp.txt`(순수 basedmg vs shadow)** — 별개 경로라 계속 미발화 = **저우선 잔여로 유지**.

**(5) ⬜미해결 = `force_sp19` 강제가 게임에 전달되지 않음**
- `my_disc17` 반환을 **9,217회 전부 `0x13`(SubPlan19)으로 강제**했으나 게임 disc19 호출 양상 **무변화** ⟹ **out에 쓴 SubPlan 값이 `unit+0x6b0`까지 전달되지 않는 것으로 보임(추정·전달 경로 미규명)**.
- 단 (1)의 A/B에서 **대체 자체는 실효 확인**됐으므로 **"대체 전체가 무효"는 아니다.** 전달 경로 규명은 **별건 ⬜**.
- cfg 키 `force_sp19`는 검증 후 **제거**(소스 게이트는 남아 있고 기본 OFF).

**(6) 최종 상태(실측)**
- 배포 dll **3,338,240B · 2026-07-31 04:20:20**(A/B 토글 `d1617_repl` 포함분).
- cfg **프로덕션 원복 완료** = `log=0`·`mpcap=0`·`dcap=0`·`d1617_repl=1`, 검증 임시키 2종(`force_sp19`·`d19_bd_cmp`) **제거**. 첫 바이트 **`0x23`(BOM 없음)**·**361줄**.

**(3) 인게임 검증(2026-07-31 01:39~01:42 경기 · 재핀 반영 dll 3,336,192B)**
- ✅**크래시 0** · 훅 **10종 설치** · **byte-patch 62/62 전량**(obj 14 / gb 9 / sev 29 / d19 10 / vis 1) · **movepri 대체 275만 회 발화**(disc 3/8/10/12/14/16/17) · serpen(disc12) entry 400.
- ✅**회귀 없음** = 재핀 전후 대조 패턴 동일(subplan 0 = 400 OK · 9 = 3000 OK), **subplan 7은 396 → 400 OK 개선**(`path10` scan2 원본위임 **0건**).
- ~~⬜★**핵심 지표 미확보** = `bdcmp.txt`·`abil2_dbg.txt` **둘 다 미생성 = disc18/19 경로가 이 경기에 안 돌았음**(대조된 subplan = 0·7·9·10·11·12뿐) ⟹ §12.12 vtable 슬롯 재핀 및 §12.12(3) shadow-CALL 4종의 비트동일 실증은 여전히 ⬜. 재시도 시 disc18/19가 발화하는 국면(넥서스 압박)을 확보할 것.~~ → ⛔★★**정정(0.5.3, 2026-07-31 세션 후반 = §12.14(2)·(4))**: ①**"로그 미생성 = 경로 미발화"는 오판** — 로그가 cfg **`dcap` 게이트 뒤**에 있었을 뿐(`disc19_repro.rs:79` `dump_disc_commands` 첫 줄 `if !DISC1819_CAP.load() { return; }`), `dcap=1`로 켜자 즉시 **`disc1819cap.txt` 43,401B·`disc19cmp.txt` 152,216B 생성 = 훅은 처음부터 계속 발화 중**이었다. ②**"비트동일 실증 ⬜"도 대상 선정 오류** — disc19 재현은 **관찰단계(미완성)**라 애초에 비트동일이 목표가 아니었다 ⟹ **이 잔여는 종결**(§12.14(4)).

#### 12.15 ★신규 레버 — sev [C] 소극 경로 4임계 `sv_pa_*` 배선 (2026-08-03, 0.5.3) — 상세 정본 = `REPORT\tfm2_ai_adjust\02_구현정보.md §7b` + `RE\2026-08-03_sv소극경로-branchA-4임계-0.5.3재핀.md`
- §7.2-A14 §5·§7의 미배선 후보 **"[C] branch A 4임계"**(0.5.2 `0x22f0067~79`) = 0.5.3 **`0xcd4cd7`/`0xcd4cdd`/`0xcd4ce3`/`0xcd4ce9`**(orig **25/34/15/20**·전부 `48 83 f8/f9` imm8·★**tr_lo만 jb="이상" 의미**).
- `apply_sev_imm` **29→33사이트**·cfg 키 4종(`sv_pa_*`) 노출·`sv_enable` 게이트 공용. ~~⬜인게임 미검증~~ → ✅**적용 검증완(08-03 01:35: `sev_imm.txt` "applied=33/33 pa=[hh25 th45 hl15 tl20]"·크래시 0)**·⬜경기 체감 잔여.

#### 12.16 ★신규 레버 — subplan별 개별 단기 시야창 `vw_*` 25사이트 (2026-08-03, 0.5.3) — 사이트 전문 = `REPORT\tfm2_ai_adjust\RE\2026-08-03_개별시야창-120틱-전수스캔-0.5.3.md`(본 절 = 포인터만) + `02_구현정보.md §7b`
- 0.5.3 전수 스캔 = 시야창 **26곳**(신규 25 + 기배선 `oi_dn_lane_margin` 1). ★**구 "120틱 8곳 미개입"(§7.2-A14 §7) 기록 정정** — 구 "8" 카운트는 sev[B] 스킬타이머 게이트 8곳 오인 가능성.
- 신설 `apply_visshort_imm` · cfg 키 6종(`vw_lane/jungle/check/nexus/threat/score`, 기본 **-1=원본 120**) · 로그 `visshort_imm.txt` `applied=N/25`.
- disc12/14 = **공용 헬퍼 `0x12b6e20`** 경유 / disc7/9/10/11/13/15 = 개별 창 없음. ⚠**재현 짝 동기 규칙 = `vw_lane`↔`dd_lane_margin` · `vw_check`↔`ec_vision_ticks`**(한쪽만 바꾸면 재현↔원본 어긋남).
- ⚠부수 의혹(사실 승격 금지) = `0xdf9513`(disc17/19 공용 헬퍼)의 바이트 문맥이 개별 시야창 패턴(lastSeen+0x78 vs curtick)과 **100% 동일** ⟹ **`oi_dn_lane_margin` 라벨("레인 진척 마진")이 실은 시야 기억창일 의혹** — 런타임 대조로 시맨틱 재확정 전 라벨 유지 = 잔여트래커 #0i.
- 배포 상태(sv_pa+vw+poke 해금 합본·§12.15 공용) = ~~dll 3,393,024B 빌드완·⏳배포 게임락 대기~~ → ✅**배포완 = `mods\tfm2_ai_adjust\tfm2_ai_adjust.dll` **3,393,024B @2026-08-03 00:41:28** MD5[:8] `C589CBDB`(빌드 산출물 해시 일치 실측)** · ~~⬜인게임 검증~~ → ✅**적용 검증완(2026-08-03 01:35 게임 로드 실측)**: `visshort_imm.txt` **"applied=25/25 check=90 threat=90"** · `sev_imm.txt` **"applied=33/33 pa=[hh25 th45 hl15 tl20]"** · 기존 5종 회귀 0(obj 14/14·gb 9/9·d19 10/10·vis 1/1) · crash_log mtime 07-31 그대로(크래시 0) · hooks.txt 갱신 정상 · 활성 cfg = 13차 프리셋 값(sv_enable=1·tr_hi45·vw_check/threat90·vis 900·retreat 48) / ⬜잔여 = **경기 체감(정글 왕복 완화 여부)** · **poke 결정성 A/B 1회**.

#### 12.17 ★신규 레버 — `gk_*` 라인개입 갱 셋업 타이밍 3키 배선 (2026-08-03 심야, 0.5.3) — 사이트 전문 = `REPORT\tfm2_ai_adjust\RE\2026-08-03_갱셋업-타이밍상수-사이트표-0.5.3.md`(본 절 = 요지·포인터만) / 인과 경로 원문 = `RE\2026-08-03_Strategy-소비처-전수맵-라인개입분기-0.5.3.md`
- 경위 = 테스터 제보("정글 부쉬 왕복 = 라인개입 전략일 때만") → RE로 인과 경로 확정(jng 유일 소비처 passive_jungle `0xe00350`·갱 게이트 헬퍼 `0xe162a0`) 후 원본 상수 노출. 신설 **`apply_gank_imm` 14사이트** · cfg 키 3종(전부 기본 **-1=원본**) · 로그 `gank_imm.txt` `applied=N/14`:
  - **`gk_wait`** = LineGankerPlan wait_limit 생성 **5사이트×2바이트**(lea SIB 스케일비트 패치 · 원본 10/12/15/15/10초 · 2~72초 조합근사): A1 `0xe0237d`/`0xe02381` · A2 `0xe02cba`/`0xe02cbe` · A3 `0xd45967`/`0xd4596b` · A4 `0xd557ae`/`0xd557b2` · A5 `0xd55cda`/`0xd55cde`.
  - **`gk_hp_base_gank`** = `0xe01e53` `mov dl,0x46` — ★**jng=1 분기 카피만** = 라인개입 전용 · 실효 임계 = base − 스탯/5.
  - **`gk_window_margin`** = `0xe01ef7`/`0xe01f9e`/`0xe020d4` lea SIB ×5 · 허용 {2,3,5,9}.
- ⛔**patch 불가 한정판정(재조사 방지 — imm 바이트패치 방식 한정)**: setup_limit = **×1 add형 인코딩**이라 imm 패치 불가 / 헬퍼 `0xe162a0` 내부 imm **0건**(테이블·클로저 경유) = 바꾸려면 런타임 write 필요.
- 배포 상태 = dll **3,395,584B 빌드완 · ⏳배포 게임락 대기**(temp `tfm2_build\tfm2_ai_adjust_45100\`) / cfg·default.txt에 3키 노출(-1) / ★**14차 프리셋 신설** = `config\AI개선모드 14차.cfg` 15,958B(**gk_wait=18 · gk_window_margin=9 · gk_hp_base_gank=-1** 실험값) / 편집기 [매크로]탭 §신설·재빌드 중.
- ⚠**전 사이트 인게임 미검증(정적 확정만)** · ⬜왕복=LineGanker setup/wait 반복 루프 기전은 **추정(런타임 미확인 · 사실 승격 금지)** — 검증 = 배포 후 `gank_imm.txt applied=14/14` + 경기 체감(잔여트래커 #0i).

#### 12.18 ★★전술(Strategy)↔cfg키 게이팅 **교차 전수조사** (2026-08-03, 0.5.3) — **RE 원문·전문 = `REPORT\tfm2_ai_adjust\RE\2026-08-03_전술게이트-cfg키-교차전수조사-0.5.3.md`**(본 절 = 요지·조건표·포인터만)

> 목적 = "설정값이 왜 안 먹나"를 **팀전술(Strategy) 필드별 게이트** 축으로 전수 규명. 방법 = 즉시값/전술 read 스캔 + **엣지컷 지배관계**(`domedge.py`) + panic 문자열 함수 이름표 복원(`fnames.py`, 522개).

**(1) ~~★최대 발견 — `wav`(미니언웨이브) = 오브젝티브 시스템 마스터 게이트~~ → ⛔반박됨(2026-08-03 인게임 실측, 0.5.3 = §12.19(2))**
- 코드 경로 자체는 사실: `team_plan.rs` **`0xcf8b90`** = fast/slow 양경로 모두 **`wav != 0` → `LAB_cf9688`에서 `team_plan+0x3f5 = 0xFF`**, `wav == 0`일 때만 `LAB_cf95e6` → **오브젝티브 선택기 `0xe29980`**(분기 `0xcf95a8`/`0xcf95d6`).
- ~~⟹ `+0x3f5/0x3f6/0x3f7` 삼총사에 의존하는 disc12~17 계열 전체가 `wav≠0`에서 통째로 침묵 = `ec_*`·`ep_*`·`disc16_home_hp`·`disc17_*` 전부 무반영. "설정값이 안 먹는다"의 최대 원인 후보.~~ → ⛔**실측 반박**: `judge_dump=1` 전수 로그에서 **RED(wavJoinPriority=합류우선)가 disc12 2,653회·disc14 2,207회·disc16 433회 발화**(오히려 BLUE=웨이브우선보다 많음) ⟹ **오브젝티브 판단 전체를 끄는 마스터 게이트로 작동하지 않는다**(다른 경로가 `+0x3f5`를 채우거나 disc12~17이 그 필드에만 의존하지 않음). **편집기 경고 문구 철회** · ~~⬜`+0x3f5` 실제 write 경로 = 미지~~ → ✅**규명(08-03 밤·§12.20(4))**: `0xcf8b90`이 **`+0x3f5`만** 쓰고, 전이가 읽는 **`+0x3f6/+0x3f7`은 `0xcef570`~`0xcf837b` 클러스터(~40사이트)**가 별도로 쓴다 = 반박의 기계적 이유 확정. 상세 = **§12.19·§12.20**.
- ⚠**라벨 함정** = RE 원문 §8의 "wav=합류우선(0)" 표기는 **오기** — 값 기준 **0 = 웨이브우선**(정본 = [[tfm2-team-strategy-tactics]] Strategy 24B 표). 모드 `serpen.rs`가 쓰는 `sf+0x3f6`/`sf+0x3f7`가 바로 이 삼총사.

**(2) ~~★판정 무효화(재시도 금지 **해제**)~~ → ~~★부분 되돌림(08-03 실측)~~ → ⛔★★**최종 확정 = 생성 코드 자체가 없음(2026-08-03 밤, §12.20(2))** — disc13·disc15**
> ⛔★**현행 판정(최종)** = 「**`plan` 13/15를 쓰는 사이트가 바이너리 전역 0건**(즉시값+레지스터 경유+`[reg+0x598]` 전수) = **소비자만 남은 사장 변이** ⟹ `d13_engage_hp_pct`·`d15_engage_hp_pct` **값 무반영 확정·편입 실익 0·재시도 금지 복원**」. ~~⬜bld=모이기/스플릿 미검증~~ → **유보 해제**(조건 문제가 아니라 코드 부재). 아래 본문의 "생성 사이트 실재(`0xc6e4aa`/`0xcb2570`)"는 ⛔**철회 = 그 둘은 `sub_plan` 값이라 이 질문과 무관**(§12.20(2)).
> ~~★부분 되돌림(08-03 낮·§12.19(1)) = 「생성 코드는 실재하나 실사용 미발화」~~ → 위로 대체.
- ~~"게임이 생성하지 않는 죽은 틀·영구 미발화·모드 편입 실익 0·재시도 금지"(0.5.0_3~0.5.2 판정 = §7.2-A10·`ANA\_archive\DONE-0.5.2.md`·잔여트래커 #0e)~~ → **0.5.3 바이너리와 불일치 = 근거 무효**.
- SubPlan 슬롯 확정 = **champion AI state `+0x6b0`**(전이엔진 `0xc559e0` → 커밋 `0xd9b5f0` → 갱신 디스패처 `0xd98740`이 `*(X+0x6b0)`로 `sub_plan\*.rs` 선택 · 두 JT 모두 `idx = disc>=2 ? disc-2 : (1|7)`). ⟹ ★**정정(§12.20(1))**: `+0x6b0`=`sub_plan` / **실측 `disc`는 `plan` = `+0x598`·디스패처 `0xc559e0`** — **두 축이 별개 enum**.
- ~~**생성 사이트 실재** = `mov qword [r14],0xd` @ `0xc6e4aa` = SubPlan13 / `mov qword [r12],0xf` @ `0xcb2570` = SubPlan15 ⟹ "생성 사이트 부재"는 0.5.3에서 성립하지 않음~~ → ⛔**철회(08-03 밤·§12.20(2))**: 이 둘은 **`sub_plan` 값**(13 EpicPoke·15 SerpenHunt)이라 **disc(=plan) 13/15와 무관** — `plan` 13/15 생산자는 **전역 0건**.
- 도달 조건(디컴+asm 확정) = `+0x3f6 == 0` **&** `+0x3f7 == 3`(재배치 페이즈) **&** `NOT(bld == 내 lane AND 0xe2a540() == true)` ⟹ 엣지컷 결과 **bld 어느 값에서도 도달 가능**, 특히 **bld=5(모이기)/6(유연)에선 lane 일치가 구조적 불가라 오히려 EpicHunt가 기본 경로**.
- ~~★정정 판정(범위 명시) = 「`wav ≠ 0` 에서는 미발화 / `wav == 0` + 오브젝티브 phase(`+0x3f7`)==3 에서는 생성 경로 실재 · 라이브 발화 실측은 ⬜미확정」. `d13_engage_hp_pct`·`d15_engage_hp_pct` 死 판정 근거도 무효 = 재측정 대상.~~ → ★**실측 후 정정(08-03, §12.19(1))**: `wav==0`+bld유연에서도 **0회** ⟹ **값 무반영 확정**(⬜bld=모이기/스플릿만 미검증).
- ~~★재측정법 = `sp_seen=1`을 wav=웨이브우선 + bld=모이기/유연으로 전술 통제해 실행~~ → ⛔**`sp_seen.txt`는 발화 판정 도구로 부적격**(§12.19(4)) — **발화 판정 정본 도구 = `judge_dump=1`**. 실측 완료분 = §12.19. ⚠되살릴 경우 §7.2-A10/잔여트래커 #0e의 **필수 수정 2건**(단일→이중 역참조 · disc13 arm 배선 결함)은 그대로 유효.

**(3) ★라이브 결함 2건(우리 모드 소스 확인 완료)**
- (a)⛔**`eng_role4/3/2/_def`·`t_engage` = 전술 무관하게 死** — `MODS\tfm2_ai_adjust\src\tfm2_ai_adjust.rs:4230` `engage_reaches_roll`이 **07-28 desync 안전판으로 `return Some(false)` 무조건 반환**(소스 주석 "부작용: engage emit 경유 튜닝(eng_role*/t_engage 등) 잠정 무력화" = 자인). ⟹ 편집기·문서엔 **LIVE 표기**돼 유저가 죽은 노브를 튜닝해 왔음 → **편집기 ⛔DEAD 표기 완료(재빌드 중)**. 재활성 조건 = 소스 주석의 "극성 반전 + 5→8 + 오프셋 전면 재핀" 세트.
- (b)⬜**disc14 재현이 원본의 `bld == role` 선체크를 생략** — `MODS\tfm2_ai_adjust\src\serpen.rs:1137` 주석 "⬜슬롯 lane idx>4||sim+0x8b0!=idx 선체크 생략(best-effort)" ⟹ **모이기/유연 전술에서 모드가 게임에 없는 경로를 실행** = **재현 발산(desync) 원인 후보**(`d14_repl`이 desync 잔존 2개 중 하나인 것과 정합) · **미수정**.

**(4) 통합 조건표(요지 — 전문·근거는 RE 원문 §6)**
| 우리 키 / 사이트군 | 게이팅 전술필드 | 판정 |
|---|---|---|
| `ec_valid_hp`·`ec_commit_hp`·`ec_count_hp`·`ec_count_radius`·`ec_vision_ticks` | ~~**bld ∈ 0~4 & == 내 lane**~~ → ★**정정(0.5.3, 08-03 밤 후속·§12.22(4))**: 게이트 `0xc6e451`/`0xcb2735`는 실재하나 비교 대상이 **bld 태그가 아니라 전술 구조체 `@+0`의 u32 포지션 페이로드**이고 edi = `sim+0x8b0`(포지션 인덱스, 1=정글) ⟹ 조건 = **"내 포지션 == 전술이 지정한 스플릿 포지션"** | **전술 O**(발화가 전술에 종속) ⟹ 지정 포지션이 아닌 선수에겐 死. ⬜`@+0`이 mor(Split14) 소유인지 bld(Split) 소유인지 **미확정** |
| `ec_oz_hp`·`ec_iz_hp`·`ec_self_hp_low`·`ec_engage_dist2` · `ep_home_*` | — | 전술 무관 |
| `eng_role*`·`t_engage` | — | **死(전술 무관)** = 위 (3)(a) |
| `d19_*`·`oi_dn_*`(fight_model `0xcc3960`) | — | **전술 무관**(콜러 3곳 전부 ungated) |
| `sv_*`(sev severity 4군)·`vis_window`·`vw_*` | — | ~~전술 무관 ⚠컨테이너 내부 기준~~ → ★**전술 무관 확정(0.5.3, 08-03 밤 후속·§12.22(3)) = 상위 경로까지 포함**(리더 23함수 목록 밖) |
| `numbers_*`·`tower_*`·`stat_*` | ? | ⬜**미조사**(구 "전술 무관"은 **컨테이너 내부 기준**이라 상위 경로 미확인 = 사실 승격 금지·§12.22(3)) |
| `gk_*` `0xe0237d`·`0xe01e53`·`0xe020d4` | **jng == 1(Ganking=라인개입)** | 전용 — ★**확정(0.5.3, 08-03 밤 후속·§12.22(1), 직전 "미확정" 해소)** ⟹ **§12.17에서 패치한 3키는 라인개입 축 위주** |
| `gk_*` `0xe01ef7`·`0xe01d01` | **jng == 2(CounterJungle=카운터정글)** | 전용(확정·§12.22(1)) |
| `gk_*` `0xe01f9e`·`0xe01daa` | **jng == 0(GrowthAndCover=성장/커버)** | 전용(확정·§12.22(1)) |
| `gk_*` `0xe02cba`·`0xe00e1d`·`0xd45967`·`0xd557ae`·`0xd55cda` | — | 전술 무관(해당 함수 전술 read 0건·확정) |
| `gb` reach **`0xe08858`** | **fin == 0** | 전용(cutFin0 시 도달 불가) / 공용 헬퍼 `0xcdd067`은 전술 무관 |
| `gb_cnt_skip`·`gb_da_thr`·`gb_cnt_move`·`gb_db_engage`·`gb_score_mult` | — | **훅 미설치가 원인**(`RVA_GB_REGIOND_HOOK=0x22dafea` stale + `MIG_GB_CHANGED=true` SKIP) = 전술 무관·기존 판정 유지 |
| `d8_slot_thr` | — | **전술 무관하게 도달 불가**(disc2·disc8 movepri JT 엔트리 = `0xc55a34 mov qword [rsi],7` 상수) = 판정 유지 |
| `d4_*` 7키 | — | 생성 `0xd8065a`(`old\passive_line`)에 **전술 read 0건** ⟹ 미발화 원인 **별건·⬜미확정** |
| **오브젝티브 시스템 전체**(`+0x3f5/6/7` 의존 = disc12~17·`ec_*`·`ep_*`·`disc16/17_*`) | ~~wav == 0~~ | ~~wav ≠ 0 이면 전부 침묵~~ → ⛔**반박(실측)** = §12.19(2) — wav≠0 팀도 disc12/14/16 활발 발화 |

**(5) ★신규 "전술별 독립 카피 상수"(개별 튜닝 레버 후보 5군)**
1. **passive_jungle 0x46 HP게이트 3벌**(jng2 `0xe01d01` / jng1 `0xe01e53` / jng0 `0xe01daa` + 무관 카피 `0xe00e1d`) — 브리핑 가설을 **엣지컷으로 확증**.
2. **engage 0x46 HP게이트 2벌**(bat≠0 = `0xd5c1b0` / bat==0 = `0xd5d58a`) — **신규·전술별 개별 튜닝 가능**.
3. **engage reach 5사이트**(`0xd58847`·`0xd588c5`·`0xd58943`·`0xd589c1`·`0xd58a4b`) = **fin==0 전용** 블록 — 신규.
4. **`old\epic.rs` `0xcff440` bld 3-way mode**(`mode = bld>=5 ? bld-5 : 2` = 모이기0/유연1/스플릿2) — **에픽 플랜이 전술별 완전 별개 코드경로**·미탐색 상수 다수 예상(가장 큰 미개척지).
5. **defense_nexus def cmove `0xdf80fd`**(def==0 → 적 ≥2 / def≠0 → 적 ∈{1,2} = 같은 카운트 임계가 def로 스왑).
- 부수: SubPlan9 커밋 `0xd5b058`(twr==0 전용) / `0xd5d6c6`(def≠0 전용) · `old\serpen.rs` `0xe289ab`도 `bld>=5` 분기 + lane 일치 요구.

**(6) 부수 자산(재사용 가치 큼)** = panic 문자열로 **게임 AI 함수 이름표 522개 복원**(`fnames.py`/`fnames.txt`) + **엣지컷 지배관계 도구 `domedge.py`**(+`stratsites.py`·`sf2.py`·`sitemap.py`) — 현재 **scratchpad 소재 ⟹ ⬜`C:\tfm2mods\`로 이전 필요(미이전·소멸 위험)**.

**(7) ⬜미확정(사실 승격 금지)** = `disc16_home_hp`·`disc17_*` 개별 사이트 매핑(fin 분기 안팎) / disc5·disc6 생성 사이트(즉시값 스캔 미검출 = 레지스터 경유 가능성) / ~~**disc13·15 런타임 실발화**(정적으론 증명 불가)~~ → ✅**실측 완료 = §12.19(1)** / sev·vw 컨테이너의 상위 도미네이터 전수 / disc18(`0xd94d00`) 상위 선택 게이트.

#### 12.19 ★★`judge_dump=1` **인게임 전수 실측** — §12.18 판정 2건 정정 + 전술↔발화 비대칭 4건 (2026-08-03, 0.5.3 buildid 24451609) — **RE 원문 = `REPORT\tfm2_ai_adjust\RE\2026-08-03_judge_dump-실측-전술별-disc발화-대조.md`**(본 절 = 요지·포인터)

> 측정 = 리그 경기 1판(인게임 8/12 T1 vs Gen.G 2세트) 전수 로그 **54,769줄 / 최대 tick 44,309(완주) / 양팀 전부**(`match_log\match_00~02.txt` = 동일 경기 재sim 3회·분포 완전 일치 = 결정성 부수 증거) · dll 3,395,584B(gank 14사이트 포함)·cfg 14차 상당.
> 경기 전술 = **BLUE(★관리팀)** focTop/jngGrowthAndCover/srpMust/srtMust/**bldFlexible**/batPoking/morGather/twrDive/defGather/**finBattlePriority**/**wavWavePriority**/endFlexible ↔ **RED** focAll/jngGrowthAndCover/srpFlexible/srtMust/**bldFlexible**/batInitiating/morSplit14{Mid}/twrDive/defBattle/**finKillPriority**/**wavJoinPriority**/endFlexible.

> ⛔★**이름열 오라벨 경고(08-03 밤·§12.20(1))** — 아래 "이름"은 **`sub_plan` 표를 잘못 갖다 붙인 구 라벨**이다. 실측 disc는 **`plan` 축**이므로 정정: **12=EpicHuntAndPoke** · **14=SerpenHuntAndPoke** · **16=AttackNexus(넥서스 공격 — 세르펜 사냥 아님)** · **17=DefenseNexus(넥서스 방어 — 세르펜 견제 아님)** · 9=Battle · 10=LineGanker · 11=LineGankCover · 13=EpicHuntAndBattle · 15=SerpenHuntAndBattle. **카운트 수치는 그대로 유효.**

| disc | ~~이름~~(구 오라벨) | 전체 | BLUE(★관리팀) | RED | 비고 |
|---|---|---|---|---|---|
| 0/1 | 라인전 | 28,249 | 15,501 | 12,748 | |
| 7 | 귀환 | 4,725 | 3,198 | 1,527 | |
| 8 | 정글 | 1,719 | 841 | 878 | |
| 9 | 교전 | 12,914 | 5,993 | 6,921 | |
| **10** | 결사전 | 197 | **197** | **0** | ★BLUE 전용 |
| **11** | 은신 | 77 | **0** | **77** | ★RED 전용 |
| 12 | 모르가드 확인 | 2,992 | 339 | **2,653** | 양팀(RED 우세) |
| 14 | 모르가드 견제 | 2,992 | 785 | **2,207** | 양팀(RED 우세) |
| **16** | 세르펜 사냥 | 433 | **0** | **433** | ★RED 전용 |
| **17** | 세르펜 견제 | 471 | **471** | **0** | ★BLUE 전용 |
| **13** | 모르가드 사냥(EpicHunt) | **0** | **0** | **0** | ⛔ |
| **15** | 세르펜 교전판단(SerpenCheck) | **0** | **0** | **0** | ⛔ |

**(1) ★정정 — disc13/15 = "실사용 미발화" 실측 재확인(§12.18(2) 부분 되돌림)** → ⛔★**최종 정정(08-03 밤·§12.20(2)) = 원인은 "조건 미충족"이 아니라 "생산자 부재"**
- §12.18이 "wav==0+bld유연이면 EpicHunt가 **오히려 기본 경로**"라고 예측했으나, **정확히 그 조건(BLUE = wavWavePriority + bldFlexible)에서 44,309틱 내내 0회**. RED(bldFlexible·wavJoinPriority)도 0회.
- ~~⟹ 현행 판정 = 「생성 코드는 0.5.3 바이너리에 실재 / 실사용 미발화」~~ → ⛔**정정**: `plan` 13/15 **생산자가 바이너리 전역 0건**(그 "생성 코드"는 `sub_plan` 값 오인) ⟹ `d13_engage_hp_pct`·`d15_engage_hp_pct` **값 무반영 확정 · 편입 실익 0 · 재시도 금지**.
- ~~⬜남은 조건 1건 = bld=모이기(Gather)/스플릿~~ → **유보 해제(재측정 불요)** = §12.20(2).

**(2) ★정정 — `wav` 마스터 게이트 가설 = 반박**(§12.18(1) 참조)
- **RED(wavJoinPriority=합류우선)가 disc12 2,653회·disc14 2,207회·disc16 433회 발화**(BLUE보다 많음) ⟹ `0xcf8b90` 코드 경로 존재는 사실이나 **오브젝티브 판단 전체를 끄는 마스터 게이트가 아님**. 편집기 경고 문구 철회.
- ~~⬜신규 잔여 = `+0x3f5` 실제 write 경로 미지~~ → ✅**해소·기계적 이유 확정(08-03 밤·§12.20(4))**: `0xcf8b90`은 **`+0x3f5` 단독 write**(그게 곧 유일 write 경로)이고, **전이가 읽는 건 `+0x3f6/+0x3f7`** = `0xcef570`~`0xcf837b`(objective_handlers/discipline, ~40사이트)가 별도로 갱신 ⟹ 0xFF 고정과 무관. + **`+0x3f6`은 0/1 플래그가 아니라 오브젝티브 ID**(0xFF=없음·관측 2,4,5,7,8,9,10,11)이고 전이는 `==0`/`==1`을 보므로 **epic/serpen poke 경로는 ID가 0·1일 때만 활성**(⬜`+0x3f6←al` 도출 미추적).

**(3) ★신규 사실 — 전술↔발화 비대칭 실측 4건** → ⛔★**원인 정정(08-03 밤·§12.20(3)) = 전술이 아니라 `team_plan+0x4de`(팀 모드)**
- ~~**disc16 세르펜 사냥 = RED 전용(433 vs 0)** — RED fin=0/BLUE fin=1 + `sub_plan\serpen_hunt.rs`의 "fin≠0이면 즉시 return 0"과 예측 일치 = **코드 근거 교차 지지**~~ → ⛔**오귀속·철회**: disc16 = **AttackNexus**(SerpenHunt 아님) ⟹ 전제 자체가 오류. 실제 원인 = **`team_plan+0x4de` JT `0x31b63bc`**: **mode4→plan16(넥서스 공격)·mode2→plan17(넥서스 방어)**, **배타적**이라 완전 상보.
- **disc17 = BLUE 전용(471 vs 0)** → 위와 동일 원인(mode2) / **disc10 = BLUE 전용(197 vs 0)** → **LineGanker = 역할슬롯==1(정글러) ∧ mode∉{2,3,4,9} ∧ 채팅 경로** = **전술 아님** / **disc11 = RED 전용(0 vs 77)** → **LineGankCover 생산자 미발견 = ⬜미해결**.
- ⚠**단, "설정값이 특정 팀전술에서만 작동한다"는 명제 자체는 기각 아님**(sub_plan 계열 fin 게이트 등 다른 사례는 유효). **정정 범위 = 이 실측 비대칭 4건이 전술 근거가 아니었다**는 것.

**(4) ★방법론 교훈 2건(버전무관·재사용)**
- ①**`match_log` 로그는 가변 공백 정렬**(`cid24  ★ disc7(...)`) ⟹ `cid(\d+)(.) disc` 류 정규식은 54,769줄 중 **1,362줄만 매칭**돼 오판 직전이었음. **정본 패턴** = `^t\s*(-?\d+)\s+team(-?\d+)\s+cid(-?\d+)\s*(★?)\s*disc(\d+)\(([^)]*)\)`.
- ②⛔**`sp_seen.txt`로 발화 유무를 판정하면 안 된다** — (a)`MP_SAFE_DISC=[0,1,2,3,8,9,10,11,12,14,16,17]`에 없는 disc(13/15 등)는 **카운터 코드에 도달조차 안 함** (b)후퇴 발동만 세므로 disc7처럼 확실히 발화하는 것도 0으로 나옴. ⟹ **발화 판정 정본 도구 = `judge_dump=1`**.

**(5) ~~⬜후속~~ → ★**대부분 §12.20에서 해소·일부 철회**(2026-08-03 밤)** = ~~bld=모이기/스플릿에서 disc13/15 재측정~~ → ⛔**불요(생성 코드 자체가 없음 = §12.20(2))** · ~~fin↔disc16 A/B 확정~~ → ⛔**철회(원인=`team_plan+0x4de` 팀 모드 = §12.20(3))** · ~~disc10/11/17 비대칭 원인 필드~~ → ✅**10·17 규명·11만 ⬜미해결(§12.20(3))** · ~~`+0x3f5` write 경로~~ → ✅**규명(`0xcf8b90` 단독 · 전이가 읽는 건 `+0x3f6/0x3f7`로 별개 = §12.20(4))**. 남은 것 = 잔여트래커 #0i의 §12.20 신규 잔여 5건.

---

#### 12.20 ★★★**프로젝트 최대급 정정 — `plan`과 `sub_plan`은 별개 enum이고, 우리가 "disc"라 부른 값은 `sub_plan`이 아니라 상위 `plan`이었다** (2026-08-03 밤, 0.5.3 buildid 24451609) — **RE 원문·전문 = `REPORT\tfm2_ai_adjust\RE\2026-08-03_plan-vs-subplan-두enum-분리-발화조건-비트단위.md`**(본 절 = 요지·번호표·포인터만)

> ⚠**적용 범위** = **0.5.3 바이너리 실측**(디스어셈·serde 변이명 blob·JT arm callee 파일명 + `judge_dump=1` 44,309틱 실측과의 교차정합). 구버전(0.4.x~0.5.2)의 SubPlan 표 자체가 틀린 게 아니라, **그 표를 movepri `disc`(=plan 축)에 갖다 붙인 것이 틀렸다**.

**(1) ★두 enum 분리 — 확정**

| | **`plan`** (구 `old::Plan`) | **`sub_plan`** (SubPlan) |
|---|---|---|
| 저장 위치 | champ AI state **`+0x598`**(u64 disc + payload `+0x5a0`~, 값 전체 **0x118B**) | **`+0x6b0`** |
| 디스패처 | **`0xc559e0`**(= 우리 movepri 훅) · JT **`0x31a7494`** 16엔트리 · `idx = disc>=2 ? disc-2 : 1` | **`0xd98740`** · JT ~~`0x31ba314`~~ → ★**`0x31ba310`**(4B off 정정 = 0.5.3, 08-03 후속 **§12.23**) 19엔트리 · `idx = disc>=2 ? disc-2 : 7`(disc0/1→battle 폴백) |
| 변이 | 0..17 | 0..20 |
| 역할 | ★**`0xc559e0` = plan→sub_plan 매퍼**(plan 입력 → sub_plan 출력) | `sub_plan`으로 `sub_plan\*.rs` 선택·실행 |

- 근거 = ①serde 변이명 blob **2개**(plan `0x31afc75` / sub_plan `0x31b0ad4`) ②각 JT arm callee 파일명(fnames)과 blob 순서 완전 일치 ③`judge_dump` 실측 최대값 **17**(=plan 최대. sub_plan이면 18/19가 나와야 함) ④disc16/17이 팀 간 **완전 상보**.
- ★**번호표(0.5.3 확정)**
  - **`plan`**(= 실측 `disc`) : `0,1`=(이름 없음, PassiveLine과 같은 JT arm) · `2`=(이름 없음, sub_plan=Jungle 무조건) · **3 PassiveLine · 4 SinglePlanLine · 5 SinglePlanBattle · 6 DeathMatchBattle · 7 PassiveJungle · 8 ActiveRecall · 9 Battle · 10 LineGanker · 11 LineGankCover · 12 EpicHuntAndPoke · 13 EpicHuntAndBattle · 14 SerpenHuntAndPoke · 15 SerpenHuntAndBattle · 16 AttackNexus · 17 DefenseNexus**
  - **`sub_plan`** : ~~`0/1`=LineDefense+1(dedup) · **2 LineAttack · 3 LineSafe · 4 LineTotal · 5 LineWait · 6 Recall · 7 Jungle · 8 Battle · 9 DeathBattle · 10 Hide · 11 EpicCheck · 12 EpicHunt · 13 EpicPoke · 14 SerpenCheck · 15 SerpenHunt · 16 SerpenPoke · 17 AttackNexus · 18 DefenseNexus · 19 Steal · 20**(update arm 즉시 ret = 무동작·변이명 미확정)~~ → ⛔★★**전부 +1 시프트 오류·정정(0.5.3, 08-03 후속 = §12.23)**: JT 베이스를 `0x31ba314`로 4B 어긋나게 읽은 결과. **확정표 = `2 LineDefense · 3 LineAttack · 4 LineSafe · 5 LineTotal · 6 LineWait · 7 Recall · 8 Jungle · 9 Battle · 10 DeathBattle · 11 Hide · 12 EpicCheck · 13 EpicHunt · 14 EpicPoke · 15 SerpenCheck · 16 SerpenHunt · 17 SerpenPoke · 18 AttackNexus · 19 DefenseNexus · 20 Steal`**(= [[tfm2-subplan-transition]] **0.5.1 원표가 옳았다**·disc0/1→battle 폴백). 핸들러 RVA 포함 전문 = **§12.23(1)**.
- ⟹ ★**구 [[tfm2-subplan-transition]] 표("0.5.1 Debug name-getter 재확정·HIGH CONFIDENCE·재조사 금지")는 `sub_plan` 축 한정으로 범위 정정** — 그 표를 movepri disc에 적용한 것이 오류. ~~또 그 표의 "18 AttackNexus / 19 DefenseNexus"는 **sub_plan 기준 17/18로 1 어긋남**(정정. "disc18=`0xd94d00`·disc19=`0xdece30`"은 실제 sub_plan 17/18)~~ → ⛔★**이 "1 어긋남" 자체가 오류·철회(0.5.3, 08-03 후속 §12.23)**: JT 베이스 `0x31ba310`(구 `0x31ba314`=4B off)로 재해독 결과 **0.5.1 원표가 옳다** ⟹ **sub_plan 18=attack_nexus `0xd94d00` · 19=defense_nexus `0xdece30`**(구 표기 복원). ★**본 §12.20 안에서 언급한 sub_plan "이름"은 전부 +1 시프트된 구표 기준**이니 §12.23(1) 확정표로 다시 읽을 것(숫자·RVA는 유효).
- ⟹ ★★**우리 모드의 disc 라벨이 통째로 오라벨**(RVA 배선·재현 DIFF 판정은 유효, **이름·설명만** 틀림): **disc12 = EpicHuntAndPoke**(구 라벨 "모르가드 확인/EpicCheck" X) · **disc14 = SerpenHuntAndPoke**(구 "모르가드 견제/EpicPoke" X) · **disc16 = AttackNexus(넥서스 공격)** — 구 "세르펜 사냥" **X** · **disc17 = DefenseNexus(넥서스 방어)** — 구 "세르펜 견제" **X** · disc9 = Battle · disc10 = LineGanker · disc11 = LineGankCover. ⟹ cfg 키 `disc16_home_hp`·`disc17_*`·`ec_*`·`ep_*`의 **설명 라벨 전부 재검토 대상**(⚠**키 이름 자체는 호환상 유지 권장 — 설명만 정정**).

**(2) ★대정정 — disc13/15 최종 확정 = "생성 코드 자체가 없음"(사장 변이)**
- `plan` **13(EpicHuntAndBattle)** / **15(SerpenHuntAndBattle)** 를 쓰는 사이트가 **바이너리 전역 0건** — 8/32비트 즉시값 + **레지스터 경유(40~60명령 역추적)** + `[reg+0]`·`[reg+0x598]` 전수 스캔 결과.
- 오브젝티브 배정 함수는 **"…AndPoke"만** 배정: `old\epic.rs` **`cff2b0: mov qword [rcx],0xc`(=12)** / `old\serpen.rs` **`e29443: mov qword [rsi],0xe`(=14)**. 13/15 배정 없음. 소비자만 생존(movepri arm 13→`0xd81990`·15→`0xc7a550`, 파일 `old\epic\hunt_and_battle.rs`·`old\serpen\hunt_and_battle.rs`).
- ⟹ ⛔**08-03 낮 "생성 사이트 실재(`0xc6e4aa`/`0xcb2570`)" 근거는 철회** — 그 둘은 **`sub_plan` 값**(~~13 EpicPoke / 15 SerpenHunt~~ → 정정 **13 epic_hunt / 15 serpen_check**, §12.23(1) 확정표)이라 이 질문과 **무관**(혼동의 진원지). ⚠**"sub_plan 값이라 무관"이라는 결론 자체는 불변**.
- ⟹ **`d13_engage_hp_pct`·`d15_engage_hp_pct` = 값 무반영 확정 · 편입 실익 0 · 재시도 금지 복원**. ⬜"bld=모이기/스플릿 미검증" 유보도 **해제**(조건 문제가 아니라 **코드 부재**).
- 동일 확정 = **plan 3(PassiveLine)도 생산자 0건** — disc0/1과 핸들러(`0xd803f0`)를 공유하는데 3만 실측 0인 이유가 "핸들러 공유"가 아니라 **생산자 부재**. (반면 plan **4·5·6·2**는 생산자 실재 = "조건 미충족"이라 성격이 다름: 4=`d49175`/`d4c14b` · 5=`d4c052`/`d4ac6b` · 6=`d4a7a4` · 2=`chat.rs` `d54b39`/`d54d6a`/`d5510b`.)

**(3) ★대정정 — 팀 비대칭의 원인은 "전술"이 아니라 `team_plan+0x4de`(팀 모드)**
- plan 결정기 `0xd452e0` 최상위 분기: `d45451 lea ecx,[rax-2]` → `d45454 cmp ecx,7` → `d4545d` **JT `0x31b63bc`**(8엔트리, `idx = mode-2`).
- **mode 2 → plan 17 DefenseNexus**(`d4552d`, payload `+8 = [champ+0x820]` 자팀) / **mode 4 → plan 16 AttackNexus**(`d45553`, payload `+8 = 1-[champ+0x820]` 적팀, `+0x10 = [team_plan+0x4df]`) / mode 3·9 → plan 0 / 5~8·범위밖 → 일반 분기 `d4555f`. **두 값이 배타적**이라 실측 완전 상보가 설명됨(RED=mode4, BLUE=mode2). **생성 단계 게이트**(plan 자체가 안 만들어짐).
- ⟹ ⛔**§12.19(3)의 "fin↔disc16 인과(코드 근거 교차 지지)" = 오귀속·철회** — disc16이 SerpenHunt라는 **전제 자체가 틀렸음**(실제 AttackNexus).
- **disc10 LineGanker** 생산 조건(확정) = `0xd452e0` 내 `d45995` — **`[champ+0x8b0](역할슬롯)==1`(정글러)** ∧ `mode ∉ {2,3,4,9}` ∧ `d4592e cmp al,0xb`(==11) ∧ `d4593e r15d==1` + `chat.rs` `d557c5`/`d55cee`(갱크 요청/수락 채팅 경로) ⟹ **전술 아님**.
- **disc11 LineGankCover** = ⬜**생산자 미발견**(즉시값·레지스터·XMM 어느 경로도 미검출인데 실측 77회 ⟹ 분명히 존재. 채팅 수락 경로 구조체 복사 **추정**) — 소비자 `old\line_gank\cover.rs` `0xe030d0`/`0xe033a0`만 확인.
- ⚠**단, "전술이 판단 발화를 가른다"는 명제 자체는 기각 아님**(다른 사례 = sub_plan 계열 fin 게이트 등은 여전히 유효). **정정 범위 = 실측 비대칭 4건(disc16/17/10/11)이 전술 근거가 아니었다**는 것뿐.

**(4) ★wav 반박의 기계적 이유 확정(§12.18(1)·§12.19(2) 보강)**
- **`0xcf8b90`은 `+0x3f5`만 쓴다**(`cf9676`←`0xFF`, `cf9688`←`al`). 전이가 실제로 읽는 **`+0x3f6`/`+0x3f7`**은 **`0xcef570`~`0xcf837b` 클러스터**(`team_plan.rs`|`objective_discipline.rs`|`objective_handlers.rs`, **약 40사이트**)가 별도로 쓴다(항상 `+0x3f6` 바로 뒤에 쌍으로) ⟹ **`+0x3f5`=0xFF 고정과 무관하게 계속 갱신** = 실측 반박의 기계적 설명. 부수 write = `old\epic.rs` `0xce8660`/`0xce8f50`(`ce8eac`·`ce93d7`), `0xceee30`(`ceef0d`·`cef08b`).
- ★**`+0x3f6`은 epic/serpen 0/1 플래그가 아니라 "오브젝티브 ID"**(0xFF=없음, 관측 즉시값 **2,4,5,7,8,9,10,11**)인데 전이 핸들러는 `==0`(epic)·`==1`(serpen)을 본다 ⟹ **에픽/서펜 poke 경로는 그 ID가 0 또는 1일 때만 활성**. ⬜`+0x3f6←al` 사이트(`cf1323`·`ce8eac`)의 값 도출 **미추적 = 미확정**.

**(5) ★신규 구조 사실 — plan 결정기·생산자 분포**
- ★**plan 결정기 본체 = `0xd452e0`**(`plan_legacy\handler.rs`, `sret 0x118B`, 인자 = out_sret·team_plan·…·arg5=champ·arg6=&World). 커밋은 호출자 **`0xd48ec0`**: `d44150(&plan)`(drop) → **`memcpy(&state.plan, out, 0x118)`**(`d4cb6e`·`d4e01f`) ⟹ **12~17이 `[reg+0x598]`에 즉시값으로 안 보이던 이유**.
- `0xd452e0` 전 출력 사이트 = `d453e4`→14 · `d4543e`→12 · `d4552d`→17 · `d45553`→16 · `d45995`→10 · `d45f4d`→7 · `d462bc`→8 · **나머지 8곳→0** ⟹ 생성집합 {0,7,8,10,12,14,16,17}. 여기에 `d48ec0`{4,5,6,9,2} · `chat.rs`{2,9,10} · `engage.rs`{9} · **`0xd63d60`**{0,**1**}(`d654f8`) · `old\epic.rs` `cff2b0`{12} · `old\serpen.rs` `e29443`{14} ⟹ **실측 비영 집합 {0,1,7,8,9,10,11,12,14,16,17}과 정확히 일치**(11만 예외 = (3) 참조).
- ★**전이 5핸들러 조건 트리(A절)** — `0xc6e080`=`old\epic\hunt_and_poke`(plan12 처리·출력 sub_plan 20/12/7/2/13/14, 게이트 `[team_plan+0x3f6]==0` ∧ `+0x3f7==3`, 진행률 `pct=[e+0x658]*100/[e+0x610] >= 0x33`) / `0xcb2340`=`old\serpen\hunt_and_poke`(plan14·**A-1과 구조 동형**, `+0x3f6==1`·출력 15/16/17) / `0xd803f0`=`old\passive_line`(plan **0·1·3**·출력 사이트 7곳·⬜조건 트리 미추출) / `0xd71630`=`old\single_line`(plan4) / `0xc559e0` 인라인 arm(plan2·8→sub_plan 7 무조건 `c55a34` · plan6→10 `c55b60` · plan10→11 `c55d45` · plan11→11 `c55b96` · **plan16→위치밴드+HP 통과 시 2, 실패 7, `[World+(1-team)*32+0x148]==0`이면 18** `c55d59/5f`). ⚠**여기 sub_plan 숫자에 붙였던 이름은 +1 시프트 구표 기준이라 삭제** — §12.23(1) 확정표로 읽을 것(**7=recall · 10=death_battle · 11=hide · 18=attack_nexus · 2=line_defense**).
- ★**byte-patch 가능 imm 22종 표 확보**(전부 실바이트 확인 = RE 원문 D절) — 특히 **`d453f2` `3c f9` = epic 허용 페이즈 하한(↓ = EpicHuntAndPoke 조기 발화)** · **`d45360`/`d453b4` `b9 a1 01 00 00` = serpen 허용 페이즈 비트마스크 `0x1a1`(비트 추가 = 발화 확대)** · **`d45321` `49 83 ff 01` = plan12/14 생성의 역할슬롯==1(정글러) 게이트** · `d4555f`(일반분기 정글러 게이트) · `d45332`(`+0x4d8==1`) · `d45454`(팀모드 JT 범위) · `d45d2f/3c/49`(plan7) · `d462b4`(plan8) · `d4592e`(plan10) · `d4a894`(plan6 검사) · epic/serpen 전이측 `c6e2d4/c6e2ff/cb25e2/cb25ef`(obj_id·phase) · `c6e26c/c6e2c2/cb254f`(진행률 0x33). ⛔**패치 불가** = 팀모드 JT(`d4546b jmp rcx`)·`cb2761 jmp rcx`·`d455d6/d455f1` 페이즈 JT(테이블 경유) · `c6e236/40/60`·`cb25c0`(메모리-레지스터 비교).

**(6) ⬜신규 잔여(사실 승격 금지)** = ①**plan11(LineGankCover) 생산자 미발견**(실측 77회 = 존재 확실·채팅 수락 경로 구조체 복사 추정) ②plan **4·5·6** 완전 조건 트리 미추출 ③★**`0xd803f0`(plan 0/1/3 = 실측 최다) 조건 트리 미추출 = 다음 우선순위 1순위** ④`+0x3f6`이 0·1이 되는 레지스터 경로(`cf1323`·`ce8eac`) 미추적 ⑤`sub_plan` **20**의 의미(update arm 즉시 ret 확정·변이명 미확정) / 부수 = `cb305b`(sub_plan 2) 도달성(`jmp rcx` 테이블 미해석).

**(7) 산출물** = 신규 도구 `pathcond.py`(사이트별 필수 CFG 엣지 자동 전수 — `domedge.py` 수동 지정 대체)·`outstore.py`·`plan598.py`·`allprod.py`·`prod2.py` / 덤프 `f_d452e0.asm`·`f_d71630.asm`·`f_cff230.asm`·`f_e28750.asm` — ⚠**현재 scratchpad 소재 ⟹ ⬜`C:\tfm2mods\` 이전 필요**(§12.18(6)과 동일 리스크).

---

#### 12.21 ★★`0xd803f0`(`old\passive_line` · plan **0/1/3** = 실측 발화 **1위 28,249회**) **조건 트리 전수 확정 + `dd_*` 키↔원본 상수 전수 매핑 + ★라이브 결함 "완전 대체 중인 함수 내부의 byte-patch는 무효"** (2026-08-03, 0.5.3 buildid 24451609) — **RE 원문·전문 = `REPORT\tfm2_ai_adjust\RE\2026-08-03_passive_line-조건트리-dd키매핑-0.5.3.md`**(본 절 = 요지·매핑표·포인터만) / ⟹ **§12.20(6)③ "1순위 잔여" 해소**

**(1) 조건 트리 요지**
- **출력(sub_plan) 스토어 7곳** = `d8040f`→**7** / `d80444`→**6** / `d8065a`→**4** / `d80873`→**4 또는 7**(cmov) / `d8155d`→**2** / `d81728`→**6** / `d8177d`→**7**.
- **5단 흐름** = ①플래그 즉시분기 → ②봇 커버 블록 → ③메인 적탐색 → ④라인상태/카운트 → ⑤거리게이트 3종 → 종단.
- **헬퍼 4종** = **라인회랑 `0x1616d70`**(lane1 = `|x-(H-y)| < 64000` 대각 판정 / lane **0/1/2 = TOP/MID/BOT은 ⬜추정**) · **가시기억 `0x12b6e20`**(`+0x78` = **120틱**) · **생존자 카운트 `0xcef270`**(`out+0x18` = len) · **min-by-dist fold `0xdc83f0`**(⚠**최소 원소를 `rdx`로 반환** = 재구현 시 반환 규약 함정).

**(2) ★`dd_*` cfg 키 ↔ 게임 원본 상수 전수 매핑**

| cfg 키 | 원본값 | 사이트 | 비고 |
|---|---|---|---|
| `dd_cover_count` | 2 | `d807f2` | 정상 대응 |
| `dd_facet_thr` | 999 | `d80855` | 정상 대응 |
| `dd_lane_margin` | 120 | **6사이트** `d80b2e`/`d80bf5`/`d80cb7`/`d80d79`/`d80e42` + 콜리 `0x12b6e78` | 정상 대응 |
| `dd_near_dist` | 150000 | **10사이트** | 정상 대응 |
| `dd_main_near_dist` | 150000 | 5사이트 | 정상 대응 |
| `dd_gatee_dist` | 170000 | `d815a9` | 정상 대응 |
| `dd_ivar2_thr` | 2 | `d8171a` | 정상 대응 |
| `dd_survivor_thr` | 3 | `d8170b`·`d81747` | 정상 대응 |
| `dd_f22e80_margin` | 150000 | `d80ffb` | 정상 대응 |
| `dd_frontier_mult` | 30 | `shl 5` 조합 | 정상 대응 |
| ★`dd_ratio_thr` | **51**(`0x33`) | `d80861` | ⚠**정정** — 구 문서·§7.2-A7의 "원본 31"은 **유저 튜닝값 오기**. 의미 = **COVER 종단의 자기 체력% 임계**(구 "라인 비율" 설명 부정확) |
| ⚠`dd_n_thr` | **없음** | (그 자리 = Rust 배열 bounds 패닉 가드 `d81734`) | **원본에 대응 조건 자체가 없다** ⟹ 모드가 원본에 없는 게이트를 추가한 **半死 레버**: 기본 2면 무해, **1로 낮추면 게임에 없는 동작** 발생 |
| ⚠`aggr_lane` | **없음** | — | 모드 합성값(frontier 리스케일) |
| ⛔`dd_early_p3_thr`·`dd_cover_p3_thr` | **없음** | — | **死 재확증** — 함수 전체에 `param_3` 비교 명령 **0개**(§7.2-A7 §8 정정 3의 독립 재확인) |

**(3) ★★라이브 결함(신규·버전무관 규칙) — "완전 대체(replace) 중인 함수 내부의 byte-patch는 무효"**
- `vw_lane`의 5사이트(`d80b2e` 외)가 **전부 `0xd803f0` 내부**인데, 모드는 `dd7_repl=1`로 **이 함수를 통째 대체** ⟹ **게임 원본이 실행되지 않으므로 그 imm 패치는 라이브에서 아무 효과가 없다**(로그상 `applied`는 성공으로 찍힌다 = 조용한 실패).
- ⟹ **라인전 기억창의 실효 레버는 재현측 `dd_lane_margin`**뿐. 반대로 **`vw_jungle`(disc4 계열·`d4_repl=0`)은 유효** — 대체하지 않는 함수라서.
- ★**일반 교훈(버전무관·DONE 등재)**: 새 byte-patch 사이트를 배선하기 전 **"그 함수를 우리가 대체 중인가?"를 먼저 확인**할 것. 대체 중이면 imm 패치가 아니라 **재현측 코드에 노브를 배선**해야 한다.
- 부수 = **`vw_check`(콜리 `0x12b6e78`)와 `dd_lane_margin`이 같은 상수**(게임측/재현측) ⟹ **값 동기 필요**(#0i "재현 짝 동기 규칙"에 추가).

**(4) plan0 vs plan1 = 함수 내 구분 분기 0개**
- `rdx` 기반 read가 `+0x110`/`+0x112`/`+0x113`/`+0x115`/`+0x116` **5개뿐**이고 **디스크리미넌트를 읽지 않는다** ⟹ **plan0과 plan1의 동작은 비트동일**, 실측 빈도차는 **생산자 소관**.
- ⟹ **§7.2-A7 §7의 "write-set이 disc0/1/3 동일" 주장이 디스어셈으로 재확증**됨.

**(5) 미노출 상수 표 확보(RE 원문 C절 15종)**
- 대표 = `d80532` **COVER role 게이트(3)** · `d8175f` **LAST 타깃 kind(2)** · `d80ef8`(**2001**) · `d809c3`(**200000²**) · `d804eb`(**`0x1a1` 모드셋**) · `0x1616d70` **라인 회랑 폭(64000 = 라인전 체감 최대·전역 공유)**.
- ⚠**대부분이 `0xd803f0` 내부 = (3)에 의해 byte-patch로는 무효** ⟹ **재현측 배선이 정답**.

**(6) 조치(2026-08-03 회차에 수행)**
- ★**신규 노브 `dd_cover_role_min` 배선** = 재현측 **4사이트**의 `lane > 2` → `(lane as i64) >= tune("dd_cover_role_min", 3)`(소스 `tfm2_ai_adjust.rs` L**3496**·L**3789**·L**3961**·L**5587**) + cfg·`default.txt` 노출. **dll 3,395,584B 빌드완 · ⏳게임 락으로 배포 대기**(§12.17 gk 빌드와 동일 산출물) = ⬜배포·인게임 미검증.
- **설정편집기 대정정·배포** = 탭 제목 8개를 **plan 실명**으로 교체 · `vw_lane`에 **⛔무효 표기** · `dd_ratio_thr` 원본 **51** 반영 · `dd_n_thr` **半死** 주의 · 신규 노브 desc 추가.
- ★**`MODS\tfm2_ai_adjust\SUBPLAN_동작_전수조사.md` 전면 재작성 완료**(CLAUDE.md §3 문서 동기 의무) = plan 이름표 정정 · 실측 발화표 · 라인전 조건 트리 · 함정 3가지.

**(7) ⬜신규 잔여(사실 승격 금지)** = ①**`vw_lane` 처리 방침 결정**(키 삭제 / ⛔무효 표기 유지 / `dd7_repl=0` 시 한정 유효 명시 중 택1) ②**미노출 상수 15종의 재현측 배선 검토**(byte-patch 불가 = (5)) ③**plan11(LineGankCover) 생산자 미발견**(§12.20(6)① 계속) ④lane 0/1/2 = TOP/MID/BOT **추정**의 런타임 확증.

---

#### 12.22 ★★전술(Strategy) 게이팅 **전수 재조사(상위 경로 포함)** = `gk_*` 갈래 분리 확정 + 전술 리더 함수 **23개 한정** + 키별 조건표 + **정정 2건** + **Strategy 12필드 값 이름 전부 복원** (2026-08-03 밤 후속, 0.5.3 buildid 24451609) — **RE 원문·전문 = `REPORT\tfm2_ai_adjust\RE\2026-08-03_전술게이팅-전수재조사-상위경로포함-0.5.3.md`**(본 절 = 요지·표·포인터만) / ⟹ **§12.18(4) 조건표 2행 정정**(위 표 반영완)

**(1) ★★실무 최중요 — `gk_*` 사이트가 정글 전술 갈래별로 분리(확정, 직전 "미확정" 해소)**
- `0xe01f9e` = **jng0(GrowthAndCover) 성장/커버 전용** · **`0xe0237d`·`0xe01e53`·`0xe020d4` = jng1(Ganking) 라인개입 전용** · `0xe01ef7` = **jng2(CounterJungle) 카운터정글 전용** · `0xe02cba` = **전술 무관**.
- ⟹ ★**`judge_dump` 실측 경기(양팀 GrowthAndCover)에선 라인개입 전용 3사이트가 죽은 코드였다** ⟹ **14차 부쉬 왕복 실험은 팀전술 초반 정글 = "라인 개입"으로 설정해야 유효**(원 테스터 제보 조건과 일치). 현행 실험 설계에 반드시 반영.
- 부수 = `0xd48ec0`의 **plan7 기록(`0xd49897`)도 jng==2 전용**이라 그 경기에선 죽었고, 실측 **plan7 4,725틱은 전부 무게이트 경로(`0xd45f4d`)** — 실측과 정합.

**(2) ★전술 리더 함수 = 23개뿐(전수 확정 · 방법론 자산)**
`0xc688c0` `0xc6e080` `0xc86280` `0xc9ce50` `0xcb2340` `0xcde4c0` `0xce8660` `0xce8f50` `0xcf8b90` `0xcff230` `0xd06810` `0xd48ec0` `0xd53f40` `0xd577e0` `0xd73d90` `0xda0750` `0xdf7c60` `0xe00350` `0xe06c10` `0xe238f0` `0xe25380` `0xe28750` `0xebb9f0`
- ⟹ **이 목록 밖 컨테이너는 전술 무지** = 이후 게이팅 판정은 **상위 경로만 보면 된다**(전 사이트 도미네이터 전수 불요).

**(3) 키별 조건표(상위 경로까지 확정)**
| 구분 | 키 / 사이트 | 비고 |
|---|---|---|
| **전술 무관(확정)** | `sv_*` 4군 전부 · `vw_*` 전부 · `vis_window` · `d19_*` · `gb_*` 주경로 · `disc16_home_hp` · `disc17_*` · `rc_*` · `d7_*` · `oi_*` | `disc16/17_*` = **팀 모드 소관**(§12.20(3)) · `d7_*` = 팀 모드 7 · `oi_*` = 팀 모드 4 전용이나 **전술은 아님** |
| **전술 O** | `ec_*` · `ep_*` = **내 포지션 == 전술 지정 스플릿 포지션**(= (4)) · `poke_*` 2경로 = `0xd5b058`(**twr Dive 전용**) / `0xd5d6c6`(**def Battle 전용**) · `dd_*` = **발화 자체는 전술 무관, "어느 라인 배정"만 전술 영향**(chat.rs 모드5) | |
| ⬜**미조사** | `numbers_*` · `tower_*` · `stat_*`(모드 신규 후퇴층) | 구 "전술 무관"은 컨테이너 내부 기준 = **사실 승격 금지** |

**(4) ★정정 — `ec_*` "bld 스플릿 전용" 근거 필드 오식별(§12.18(4) 1행)**
- 게이트 `0xc6e451`/`0xcb2735`는 **실재**하나, 비교 대상이 ~~bld 태그~~가 아니라 **전술 구조체 `@+0`의 u32 포지션 페이로드**이고 `edi` = **`sim+0x8b0`**(포지션 인덱스, **1=정글**).
- ⟹ 조건 = **"내 포지션 == 전술이 지정한 스플릿 포지션"**(구 "모이기(5)/유연(6)에서 100% 死"는 근거 재기술 필요).
- ⬜**`@+0`이 mor(Split14) 소유인지 bld(Split) 소유인지 미확정**(사실 승격 금지).

**(5) plan10/11 비대칭 = 전술 아님·팀 모드**(§12.20(3) 보강)
- **plan10** = `0xd45995`에서 **`team_plan+0x4de == 11` ∧ 포지션 == 1(정글)** 일 때만 생산(`0xd4592e`·`0xd4593e`).
- 팀 모드 10/11은 **chat.rs가 메시지 종류 JT 안에서 기록**(`0xd54afa` ← 10 · `0xd54dd8` ← 11).
- ⬜**plan11 생산자 여전히 미발견**이나 **범위 축소** = `[reg+0x598]` 즉시값 **0건** ⟹ **0x118B memcpy 15곳**(`0xd46272` · `0xd48ec0` 6곳 · `0xd53f40` 7곳 · `0xe02658`) 중 하나. **확인법 = 런타임 BP로 src 첫 qword 관찰**.
- ⚠**재발 방지**: `0xc55d45`의 11은 **sub_plan 11**이지 plan 11이 아니다(혼동 지점).

**(6) ★자산 — Strategy 12필드 값 이름 전부 복원**(Rust 열거형 이름표 `.rdata 0x334e720~0x334ebf0` + World 필드명 배열 `0x31CBE08` idx26 = `strategy@+0xb248`)
- bat **0=Poking / 1=Initiating** · twr **0=Dive / 1=Poking** · def **0=Gather / 1=Battle** · fin **0=KillPriority / 1=BattlePriority** · wav **0=WavePriority / 1=JoinPriority** · foc **0=Top / 1=Bottom / 2=All** · jng **0=GrowthAndCover / 1=Ganking / 2=CounterJungle** · srp·srt **0=Must / 1=Flexible / 2=Giveup** · end **0=Stable / 1=Flexible / 2=Aggressive**.
- **실측과 전부 정합** ⟹ [[tfm2-team-strategy-tactics]] **24B 레이아웃 표에 값 이름 확정 반영**(구 표는 한글 disc 숫자만).
- ⚠**`@+0`/`@+4`/`@+8` 3개 u32의 필드 귀속은 ⬜미확정**.

**(7) ⚠주의 기록(도구 함정)** — chat.rs(`0xd53f40`)의 `cmp byte [rbp+0x100],2/3` **6곳은 전술이 아니라 plan 구조체 스택슬롯 재사용**인데 **자동 판정 도구가 전술로 오인**. ⟹ **chat.rs 신규 판정은 JT 해석(`pathcond2`) + 수동 확인 병행 필수**.

**(8) 모드 조치(기록만)** = 설정편집기 desc에 전술 조건 반영·배포 — `gk_wait`에 **"라인 개입 전술로 실험"** 안내 / `ec_*` 조건 정정 / `sv_*`·`vw_*`·`oi_*` 탭에 **"전술 무관"** 명시.

**(9) ⬜신규 잔여(사실 승격 금지)** = ①`numbers_*`·`tower_*`·`stat_*` 게이팅 미조사 ②plan11 생산자(범위 = 0x118B memcpy 15곳으로 축소) ③전술 구조체 `@+0` 필드 귀속(mor vs bld).

---

#### 12.23 ★★★**sub_plan JT 베이스 정정(`0x31ba310`) = 기존 sub_plan 번호표 전부 +1 시프트 오류** + **sub_plan 실행층(19핸들러) 조사 = 정글 3증상 원인 확정·출력 계약·튜닝 imm 확보** (2026-08-03 후속, 0.5.3 buildid 24451609) — **RE 원문 2건 = `REPORT\tfm2_ai_adjust\RE\2026-08-03_정글3증상-원인규명-subplan실행층-0.5.3.md` · `RE\2026-08-03_subplan실행층-JT해독-핵심3핸들러-0.5.3.md`**(본 절 = 요지·표·포인터만)

> ⚠**적용 범위** = **0.5.3 바이너리 실측**(JT 재해독 + 핸들러 디스어셈). ⟹ **§12.20(1)의 sub_plan 번호표·[[tfm2-subplan-transition]]·INDEX 역맵의 sub_plan 이름을 전부 정정**(plan 축 결론·RVA·DIFF 판정은 전부 유효).

**(1) ★★대정정 — sub_plan JT 베이스 = `0x31ba310`(구 `0x31ba314`는 4B off) ⟹ 번호표 전부 +1 시프트**
- 확증 3중 = ①**Ghidra 8080·8081 독립 2조사 일치** ②프로파일 버킷 `0x31b9fe0` 그룹과 정합 ③핸들러 파일명(fnames)·의미 정합(plan8 ActiveRecall → sub_plan 7 recall 등). 인덱싱 = `idx = disc>=2 ? disc-2 : 7`(disc0/1 → battle 폴백).
- ★**확정표(sub_plan → 핸들러 RVA, 0.5.3)**

| # | 이름 | RVA | # | 이름 | RVA |
|---|---|---|---|---|---|
| 2 | line_defense(26.8KB) | `0xc5e160` | 12 | epic_check | `0xc55ea0` |
| 3 | line_attack | `0xca2480` | 13 | epic_hunt | `0xc688c0` |
| 4 | line_safe | `0xdefa60` | 14 | epic_poke | `0xda9df0` |
| 5 | line_total | `0xc57580` | 15 | serpen_check | `0xc671e0` |
| 6 | line_wait | `0xd96a40` | 16 | serpen_hunt | `0xda0750` |
| **7** | **recall** | **`0xcb1a80`** | 17 | serpen_poke | `0xc599e0` |
| **8** | **jungle** | **`0xcb02a0`** | **18** | **attack_nexus** | **`0xd94d00`** |
| 9 | battle(20KB) | `0xca8a10` | **19** | **defense_nexus** | **`0xdece30`** |
| 10 | death_battle | `0xda59d0` | 20 | steal | `0xca7310` |
| 11 | hide | `0xca43c0` | | | |

- ⟹ ⛔**정정 대상(모순 제거 완료)** = §12.20(1)(2)의 sub_plan 표·"18/19 Nexus는 17/18로 1 어긋남"(**철회** — [[tfm2-subplan-transition]]의 **0.5.1 원표가 옳았다**) · 구 "7 Jungle"→**7 recall / 8 jungle** · 구 "2 LineAttack"→**2 line_defense** · 구 "20=무동작"→**20 steal**.
- ★**라인전 hot path 재판정** = `0xd803f0`(plan0/1/3) 출력 {2,4,6,7} = **line_defense·line_safe·line_wait·recall**(구 해석 "대기7/귀환6/전면4/라인공격2"의 **이름은 전부 오라벨** — §12.21(1) 숫자 자체는 유효).

**(2) ★정글 3증상 원인 확정(테스터 제보 대응)**
- ①**젠 대기 = 설계다(버그 아님)** — plan층 캠프 선택기 `0xe00350` 내 스코어링 루프(`e0281f`~`e02ae4`)가 **죽은 캠프의 "리스폰까지 남은 시간"을 비용으로 최소 선택**하고 `jungle.rs`는 그 좌표로 이동할 뿐 ⟹ 도착 후 정지. **jungle.rs엔 대기 분기 자체가 없다**.
- ②**우물행 = `recall.rs`(`0xcb1a80`)가 하드코딩 분수 좌표(side0 `32000`/`928000`)로 이동하는 게 전부**(앵커·폴백 구조 없음) + `jungle.rs`의 **camp4/5 경유 우회**가 "크게 한 바퀴"의 직접 원인. ⚠**우회 latch = SubPlan 페이로드 `+0x11`** ⟹ 플랜 재생성마다 `dfff1b`에서 **0으로 리셋** = 우회가 계속 부활.
- ③**부쉬 왕복 = sub_plan/hide 층에 주기 상수 없음** ⟹ **주기는 plan층 갱 타이머(`gk_*`)가 맞고**, 왕복 **진폭만** `hide.rs` 거리 상수(`0xca4ae3` = 250000²)로 조절 가능.

**(3) ★출력 계약 — 19핸들러 공통(확정)**
- 반환 = **32B `Vec<Order, &Arena>`**(`+0` ptr / `+8` alloc / `+0x10` cap / `+0x18` len). 원소 **`Order` = 0xA8B**, **태그 u8 `+0xA1`**.
- 태그 = **0** 직행 · **1** 경유 · **3** 우물이동 · **4** Recall · **5** 이동/교전 · **0xE**·**0xF** · **0x0F~0x11** 스킬 · **0xFF** None.
- 공통 헬퍼 = `0xc6efd0`(RawVec grow) · `0xc9c770`(extend_from_slice) · **`0xc365a0`(전 핸들러 공통 최종 후처리 · ⬜미조사)**.
- ⬜★**Order 태그 소비자(실행기) 미발견 = 레버리지 최대 미해결**(찾으면 이동·교전 명령을 실행 직전에 가로챌 수 있음).

**(4) ★전술 리더는 19개 중 2개뿐** = **`epic_hunt 0xc688c0` · `serpen_hunt 0xda0750`**. `line_attack`·`line_total`·`recall`·`jungle`·`hide`는 **전부 전술 무지**(깊이-1 콜리 교집합 0) ⟹ **sub_plan 실행층은 전술이 아니라 `team_plan` 레코드(`[ctx+0x10]`·0x2e8 스트라이드) 경유로만 간접 영향**을 받는다(§12.22(2) 리더 23함수 목록과 정합).

**(5) ★튜닝 imm 확보 = 2 RE 합산 ~25사이트(전부 ✅유효 = 이 층은 모드가 대체하지 않음)**
- **정글 계열** = `0xcb06be`(위험조회 인자 12) · `0xcb081c`(150000²+1 = 우물 선택지) · `0xcb12dc`/`0xcb13a0`(100000² 경유생략) · `0xcb144e`(80000 도착반경) · `0xcb08f2`계열·`0xcb1af6`계열(**우물 좌표 즉치**) · `0xe0276b`/`0xe0299a`(캠프 존점수 하한 −2) · `0xca4ae3`(hide 250000²).
- **plan7 HP 게이트** = `0xdffebf`(21) · `0xdfff03`(41) — ⚠**모드가 plan7을 대체 중인지 확인 필요**(`MP_SAFE_DISC`에 7은 없으나 `d7_repl` 존재 = §12.21(3) "대체 함수 내부 패치 무효" 규칙 적용 대상일 수 있음).
- **line 계열** = `ca2b76` · `ca2e0f` · `ca3766` · `c57ed3`계열(80000 범위) · `c57dcf`(9765625).

**(6) ⬜신규 잔여(사실 승격 금지)** = ①★**`line_defense`(26.8KB)·`line_safe`·`line_wait` = 진짜 hot인데 완전 미조사 = 1순위** ②`battle.rs`(20KB) 미조사 ③**Order 태그 소비자(실행기) 미발견** ④`0xc365a0`(공통 후처리) 미조사 ⑤K220·K270 = 런타임 값이라 **즉치 패치 불가** ⑥"캠프 전멸 시 무조건 탈락"으로 바꾸려면 즉치가 아니라 **분기 패치 필요**(`e028cc`/`e02aa1` `jae`→`jmp`).

---

### 13. ★`tfm2_elemental_serpen` 0.5.3 마이그 **완료·빌드·배포·✅인게임 검증완**(2026-07-29, 0.5.3 buildid 24451609, 최종 dll **424,960B** sha256[:16] **`73CC3FCB9357BB29`**(진단용 초당 flush 제거본 / ~~검증 前 배포본 `439C32826EBD3EC4`~~), `mod.mod_info` **v0.4.2**·deps ~~`>=0.5.3, <0.6.0`~~ → ★**`>=0.5.3, <0.5.4`**(유저 지시 07-29 = **0.5.4 나오면 자동 비활성**·정책 변경)·**BOM 없음(첫바이트 7b) 확인**) — **본 절 = 이 건의 정본** / ~~⬜인게임 미검증~~ → ✅**검증완(§13.6)** / ~~⬜릴리스 zip 미생성~~ → ✅**zip 생성완(23:58·§13.6)**

> 검증 방식 = **capstone+pefile exe↔exe 실측**(Ghidra는 교차확인용). ★**RVA 16종 전부 독립 2방법 이상 교차검증**. 소스 백업(0.5.2 베이스) = `C:\tfm2mods\tfm2_elemental_serpen\_bak\lib.rs.052bak`.

#### 13.1 함수시작 RVA (0.5.2 → 0.5.3) — 전부 실측 확정
| 상수 | 0.5.2 | **0.5.3** | 확정 근거(2방법 이상) |
|---|---|---|---|
| `SERPEN_RVA` | `0x21f8ca0` | **`0x1535810`** | 프롤로그 12B 동일 + 함수 +77 명령 1:1 대응 |
| `MOBATICK_RVA` | `0x230c290` | **`0xeeeac0`** | 문자열 `"game_core::simulation::game"` LEA 유일 + 전파투표 43표 + provider 오프셋 교차검증 |
| `SPAWN_HOOKS` | `0x53aae0`/`0x539f40` | **`0xabdf60`/`0xabd340`** | 프롤로그 동일·크기 1920→2073 동수·순서대응 |
| `LAUNCHER_RVA` | `0x1d96870` | **`0xeb8810`** | 씬빌더 2회∧리플레이 1회 다중도 유일해 + ghidra 독립확인 (**item_tactics §11.6과 동일 함수·값 일치**) |
| `LAUNCHER_RET_A` | `0x759c36` | **`0x9a3287`** | 콜사이트 `0x9a3282`+5 |
| `LAUNCHER_RET_B` | `0x75e5cf` | **`0x9a7b03`** | 콜사이트 `0x9a7afe`+5 |
| `LAUNCHER_RET_C` | `0x1555215` | **`0x229ad94`** | 콜사이트 `0x229ad8f`+5 (리플레이 게이트·v0.4.1 기능) |
| `RUNNER_CTOR_RVA` | `0x1d981e0` | **`0xeba490`** | 전파투표 1위 + ghidra(콜사이트 6곳 컨테이너 완전대응) |
| `UILOADER_RVA` | `0x5ac950` | **`0x2e1550`** | ★13.3 참조(경로문자열 LEA→직후 call·콜러사상 193/194) |
| `UIPARSER_RVA` | `0x24b5a00` | **`0x1a6530`** | 콜러 사상 3/3 (item_tactics `PARSER_RVA`와 일치) |
| `UIALLOC_RVA` | `0x25c4d30` | **`0x28f7df0`** | ★13.4 참조(2인자 shim 소멸 → 실할당자·**3인자**) |
| `RENDER_STEP_RVA` | `0x811500` | **`0x960df0`** | 프롤로그 동일 |
| `DMGA_RVA` | `0x22164a0` | **`0xfdbbb0`** | 프롤로그 12B 바이트동일·크기 811→811 |
| `DMGB_RVA` | `0x22d2b20` | **`0x12c3bb0`** | 프롤로그 동일 |
| `KEYRES_RVA` | `0xc2f990` | **`0x1b0aba0`** | 프롤로그 12B 바이트 완전동일 (스프라이트 seam — 미설치 시 스프라이트 전멸) |
| `ARG_STR_RVA` | `0xfef190` | **`0x1228a90`** | 콜러투표 61표 + `"Stats"` LEA 직후 콜 유일지점·크기 359→359·진입 15B 바이트동일 |

- ★**모든 훅의 `PROLOGUE` 상수(12B, `ARG_STR`은 15B)는 0.5.3에서 무수정 유효** — 전부 바이트 동일·명령경계 정확·스틸 구간에 rel/rip 참조 없음(실측).

#### 13.2 ★구조체 오프셋 변화 (이번 패치의 진짜 함정 — 타 모드 파급)
- **provider(World)의 `0xea00~0xf000` 대역이 통째로 `+0x40` 이동**. 그 **아래는 전부 불변**(엔티티 `0x40~0x400` n=4852 / World 슬롯맵 `0x400~0x1000` n=1024 / db `0x1000~0x2000` / Game) ⟹ **0x40 삽입 지점은 `0xe000~0xea00` 사이**, **저역까지 같이 밀면 안 된다**(§11.3의 "≥0xb278 전부 +0x40"보다 좁은 실측 경계).
  - `SEED_OFF` `0xeab8`→**`0xeaf8`** · `SIM_TICK_OFF` `0xeac0`→**`0xeb00`** · `CAMP_SPAWN_TICK` `0xecd0`→**`0xed10`** · `CAMP_WAVE_IDX` `0xecd8`→**`0xed18`** · kills Vec cap `0xed18`→**`0xed58`** · `KILLS_PTR` `0xed20`→**`0xed60`** · `KILLS_LEN` `0xed28`→**`0xed68`** · 버프잔여틱 `0xed30`/`0xed38`→**`0xed70`/`0xed78`** · `KILLS_BLUE` `0xed50`→**`0xed90`** · `KILLS_RED` `0xed58`→**`0xed98`** · World→MobaMode `0xeaf0`→**`0xeb30`**.
  - kills 엔트리 레이아웃 16B `{team:u64, tick:u64}`·인덱스 규약(0=blue,1=red) = **불변**.
- ★**`O_ENTITY_ACCESSOR` `0x1b8` → `0x1c8`(+0x10)** — SERPEN 함수 +77에서 `mov rax,[rbx+0x1b8]`→`[rbx+0x1c8]`로 **disp만 바뀐 것을 명령 단위 확인**. ⚠읽어서 **함수포인터로 호출**하는 값이라 틀리면 **즉시 크래시**. 엔티티 구조체의 나머지 필드는 전부 불변.
- **불변 확정 오프셋**: `0x68`·`0xb0`·`0x250`·`0x258`·`0x5a8`·`0x610`·`0x658`·`0x670`·`0x720`·`0x738`·`0x820`·`0x840`·`0x8b8`·`0x8c0`·스트라이드 `0x6a8`/`0x8d0`·`0x1dc0`·`0x1660`·db 계열(`0x1338`·`0x1598`·`0x1630`·`0x1670`·`0x1680`).
- ⬜**약한 판정 1건 = `EV_PTR_OFF`(`0x1678`)**: 사용처 5개뿐이라 +0x10:5 vs +0:4 근소 ⟹ **값 유지**. 읽기전용 + 불변식 자기검증이라 틀려도 조용히 미채택. **인게임에서 재생커서 동기 이상이 보이면 여기부터 의심**.

#### 13.3 ★★재시도 금지 — `0x91ab0`(UILOADER/asset-get)은 **오답**(타 모드 전파 필수)
- `_MIGRATE_053.md`의 UILOADER "확정 `0x91ab0`"은 **오답**. `0x91ab0`은 skel 유일(copy 1개)인 **무관 함수**다.
- 원인 = asset-get이 0.5.2에서 **26-copy**, 0.5.3에서 **30-copy 모노모픽 군집**이라 **통계매칭이 군집 대표를 잘못 집는다**.
- ✅**정답 도출법(재사용)** = 경로 문자열(`"asset/base/ui/layout/ingame"`·`".../main"`)의 **LEA → 직후 call 타깃**. **0.5.2에 돌리면 `0x5ac950`이 ingame 13회·main 17회로 재현 = 방법 자체가 검증됨** ⟹ 0.5.3 = **`0x2e1550`**(콜러 사상 투표로도 **193/194 독립 일치**).
- ⚠**같은 표에서 `LOADER_RVA=0x91ab0`으로 적힌 `tfm2_ai_adjust`·`tfm2_item_tactics`·`tfm2_comptest_unlock`, `RVA_ASSET_GET`/`RVA_ANIM_GET=0x91ab0`인 `tfm2_banpick_illust`도 전부 같은 오답**이다(0.5.2의 **서로 다른 3함수가 모두 `0x91ab0`으로 매칭된 것 자체가 증거**). ⟹ 그 모드들은 **`0x2e1550` 계열로 재검증 필요 — 안 고치면 UI 주입이 조용히 미발화**한다. (item_tactics는 §11.6에서 이미 `0x2e1550` 채택완 = 무해 / **ai_adjust §12.6의 "미조정 충돌 1건"은 이로써 해소 = `0x2e1550`이 정답**.)

#### 13.4 ★★2인자 `alloc(size,align)` shim = **LTO 인라인으로 소멸** → 실할당자 3인자 직접 호출
- 0.5.2 `0x25c4d30`의 **어떤 부분열도 0.5.3에 0회 등장**(실할당자 참조 함수가 5개→**10,644개**) = shim 소멸 실증.
- 대체 = shim이 align≤0x10에서 tail-jmp 하던 **실할당자 `0x28f7df0`**(0.5.2 `0x25d9640`과 **바이트동일**·GetProcessHeap→HeapAlloc thunk·HeapAlloc IAT 참조 유일 코드) — **ai_adjust §12.3과 완전 동일 결론(독립 도출·2:1)**.
- ⚠**인자 3개**(rcx=무시, rdx=flags, r8=size). **2인자 그대로 두면 rdx=align=8이 `HEAP_ZERO_MEMORY`로 해석되고 r8 미초기화 → 랜덤 크래시.** `ALLOC_RVA`를 쓰는 **ui_inject 계열 전 모드(ai_adjust·item_tactics·comptest_unlock·serpen) 공통 사항**.
- `dealloc`은 `0x1000`으로 이동, `realloc`만 **`0x28e3b10`**에 바이트동일 잔존(item_tactics `RVA_REALLOC` 확정값 재확인 OK).

#### 13.5 ★방법론 = 앵커맵 재사용 자산 (다음 모드 마이그는 재탐색 금지)
- 이번 패치는 **연속바이트 마스크시그가 전멸**했지만, **capstone 기반 "skel 유일쌍 시드 → caller/callee 다중도 투표 전파"** 로 **0.5.2↔0.5.3 함수쌍 25,862개**를 자동 확보했다.
- 자산: 앵커맵 캐시 **`C:\tfm2mods\_anchor_052_053.pkl`** + 조사 스크립트 `C:\tfm2mods\serpen_053{,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p}.py` ⟹ **다른 모드 마이그는 재탐색 말고 이걸 재사용할 것.**

#### 13.6 상태 / ✅인게임 검증 결과(2026-07-29) / ⬜잔여
- ✅빌드·배포 완료(모드 v0.4.1 → **v0.4.2**). ~~⬜인게임 검증 대기~~ → ✅✅**인게임 검증완 = §13.1~§13.4 전 RVA·오프셋 실증**.
- ✅**훅 12/12 설치 OK·실패 0건·`seh_faults=0`**: serpen `0x1535810` / launcher `0xeb8810` / 렌더스텝 `0x960df0` / 장로처형(MobaTick) `0xeeeac0` / 파이프라인B `0x12c3bb0` / 증폭A `0xfdbbb0` / 키리졸버 `0x1b0aba0` / ui loader `0x2e1550` / arg_str `0x1228a90` / **spawn `0xabdf60`·`0xabd340`** / **runner_ctor `0xeba490`**(뒤 3종은 경기 진입 시 설치 = 이번에 최초 확인).
- ✅**provider `+0x40` 시프트 실증(§13.2 확정)**: `[게임카운터] blue=2 red=1` 합 = `Vec.len=3` 정합(`0xed90`/`0xed98` ↔ `0xed68`) · `[웨이브] #0@7200 #1@15602 #2@25009 #3@32828` = spawn_tick 단조증가·웨이브 인덱스 순번 정확(`0xed10`/`0xed18`) · `LIVE_SEED=0x69f5bc33f99af87b` = `RENDER_SEED` 일치(`0xeaf8`) · `sim_tick=36265`·played/sim=0.73 정상(`0xeb00`). `elder_after=2` 설정대로 **3번째 웨이브부터 장로(-1) 배정**.
- ✅**`O_ENTITY_ACCESSOR 0x1c8` 실증**: 리졸버 호출→`kind==6` 게이트→템플릿 `len==21` 게이트를 **전부 통과해야** 나오는 `세르펜 웨이브 … → '장로 세르펜'` 로그 다수·**크래시 0**.
- ✅**런처 게이트 콜사이트 분류 실전 적중**: `[런처호출처] 0x220acb×8 0x20dac9c×45 0x195c5be×44 0x9a7b03×1★게이트` — 채택 게이트 = **`LAUNCHER_RET_B(0x9a7b03)`**, 비채택 3종은 사전에 "배경 리그"로 분류한 콜사이트(`0x220ac6`/`0x20dac97`/`0x195c5b9` +5)와 **완전 일치**(98회 발화 중 화면 경기 1회만 정확히 선별).
- ✅**DMGA 산술 정확**: 히트샘플 `dmg=92 tgt_hp=1000` → 다음 `tgt_hp=908`(=1000−92) ⟹ `O_CUR_HP(0x658)`·훅 정확. **처형 245회**·화면경기 발화 21회.
- ✅**UI 주입·툴팁 정상**: `ingame로드감지=1회 주입성공=1회`(UILOADER `0x2e1550` + UIALLOC **3인자** 실증 = §13.4 계약 확인) / `arg_str교체=53회` + 툴팁 본문(속성별 처치 스택 + 스탯 합산) 정상 생성. **세르펜 색(키리졸버 스프라이트 교체) = 유저 육안 확인 OK**. 재생커서 = 렌더스텝 정밀 경로(`played=26597 game_time="07:23"`).
- ⬜`EV_PTR_OFF`(§13.2 말미) 약판정 = **미해소·값 유지**(재생커서 동기 이상 시 1순위 — 이번 검증에선 이상 없음).
- ~~⬜릴리스 zip 미생성~~ → ✅**릴리스 zip 생성완(2026-07-29 23:58)**: `…\Teamfight Manager2\mods\release\0.5.3\tfm2_elemental_serpen.zip` **1,162,362B**·zip 루트 `tfm2_elemental_serpen\` 한 겹·**28엔트리**(0.5.2 릴리스 구성과 동일 = mod.mod_info·README_en/ko.md·serpen_probe.cfg·dll 424,960B·`config\*.cfg` 8종·`s\*` 14종·`text\serpen.i18n`). 검증 전항목 통과 = `mod.mod_info` 첫바이트 `7b`(BOM無)·`0.4.2`·deps `>=0.5.3, <0.5.4` / **`config\serpen.cfg` = `elder_after = 5` 정규화**(라이브는 유저 테스트값 `3` 그대로 보존 — **스테이징 사본만 치환**) / BOM 파일 0건 / 개인정보·경로 유출 0건(유일 매치는 의도된 `"author": "tfm2mods"`). 제외 = `serpen_probe.txt`·`serpen_divergence.txt`·`_art_src\`·`_bak_s\`·`mod.override_info.off`·`fanim_editor 바로가기.lnk`.
- ★★**신규 함정 = `serpen_probe.cfg`도 "라이브 복사 금지" 대상**(규칙14의 **두 번째 파일**): 라이브 = 개발용 **1,895B**(한글 주석 + 디버그 스위치 다수 = attr_system/spawn_hook/div_audit/producer_seam…) / 릴리스판 = **280B 영문 최소본**(`probe_log` + `buff_show_delay_tick`만) ⟹ **라이브를 그대로 담으면 개발 스위치가 유출**된다. 이번엔 0.5.2 릴리스판을 재사용(순수 ASCII 확인). 지금까지 규칙14는 `config\serpen.cfg`(elder_after)만 명시했으므로 **체크리스트 갱신 필요**.
- ⚠**기존 릴리스물 결함(미수정) = `mods\release\0.5.2\tfm2_elemental_serpen.zip`의 `config\serpen.cfg`에 깨진 문자 10개(`0x3f`=`?`)**: UTF-8 em-dash(`—` e2 80 94)·박스문자(`──`)가 주석에서 파괴 = **PS5.1 `Set-Content`/`Out-File -Encoding utf8` 계열의 전형 증상**(BOM 함정의 사촌). 주석만 깨져 **기능 영향 없음**. 0.5.3 zip은 그 파일을 재사용하지 않고 **라이브 원본에서 새로 만들어 `?` 0개**. ⟹ ★**앞으로 cfg 정규화는 반드시 `[System.IO.File]::WriteAllText(path, text, UTF8Encoding($false))` + 생성 후 `0x3f` 개수 검사**(배포 체크리스트 추가).

#### 13.7 ★검증에서 새로 확인된 사실 3건(다음 세션 필독)
- ★★**`probe_flush()` 호출부가 `on_init` 한 곳뿐이고 `on_init`은 프로세스당 1회만 발화**(실측: 연속 2 프로세스 관찰 — 시작 +5초에 1회 기록 후 무갱신) ⟹ **기본 빌드로는 경기 중 진단이 파일에 영영 안 찍힌다**. 인게임 관측이 필요하면 **`post_update` 말미에 1초 스로틀 `probe_flush()`를 임시 삽입 → 검증 후 제거**(이번 검증이 그 방식·최종 배포본은 제거본 `73CC3FCB9357BB29`). **되살리는 코드 블록을 `lib.rs` post_update 말미에 주석으로 보존**해 뒀다.
- ⚠**db(ClientDatabase) 진단 오프셋 2건이 0.5.3에서 어긋난 정황**(기능 영향 0 — 실판정은 `game_time` 노드·렌더스텝 경로가 담당): ①**`SCENE_TAG_OFF(0x1338)`** = 경기 중인데 `tag=9`(0.5.2 표에선 Prologue) — 정적 계열판정은 "불변"이었으나 **씬빌더 확정쌍에서 `+0x18`(`0x1350`) 신호 22회**와 부합 ⟹ **`+0x18` 이동 의심** ②**`VIEW2_TICK_OFF(0x1630)`** = 값 `53`으로 tick으로선 무의미 ③`EV_PTR_OFF(0x1678)` 약판정(+0x10 vs +0 = 5:4)도 미해소. ⟹ 셋 다 serpen에선 **진단 전용이라 당장 무해**하나, **db 계열 오프셋을 실기능에 쓰는 모드(crm·Spectator_Chat 등)엔 영향 가능** ⟹ 잔여트래커 "db 계열 0.5.3 재핀" 항목 등재.
- ⚠**진단 카운터 2건 무발화(기능 무영향)**: `enter_count=0`인데 serpen detour는 정상 동작(웨이브 배정 로그 다수) ⟹ **ENTER_COUNT 카운터가 죽어 있음**(표시 버그) / `rctor_n=0` — **runner_ctor 훅은 설치됐으나 발화 0회**(런처 게이트가 주 경로라 기능 무영향, 미발화 사실만 기록).

---

### 14. ★`tfm2_banpick_order` 0.5.3 마이그 **완료·빌드·배포**(2026-07-30, 0.5.3 buildid **24451609**, exe sha256[:16] `6afff2cdb6bfa98e`, dll **2,538,496B**, mod_info **v1.1.0**·deps `>=0.5.3`) — **본 절 = 이 건의 정본** / ~~⬜**인게임 미검증**~~ → ✅**인게임 검증완(2026-07-30 심야, 0.5.3)** = 커스텀 순서·팀 지정 정상(20/20 완주 / ⚠~~코치 위임 정상~~ → ★**정정(07-30 낮): 구성 착시** — 그 런은 `ui_highlight` 훅이 켜진 프로브 빌드. 릴리스 구성에선 **2번째 픽부터 위임 정지** = 진범·수정 = **§14.6**) / ~~⬜**잔여 1건 = 슬롯 "칸 채움색"만 바닐라 순서 = §14.5**~~ → ✅**규명·구현·배포(07-30 09:49)·인게임 검증완 = §14.5(6b)** / ⬜현재 대기 = **코치수정+Next정책 합본 빌드 배포·검증(§14.6)**

> 0.5.2 정본 = **§7.2-C**(구조·훅 역할·cfg 계약은 그대로 유효) · 모드 정본 = `MEM\tfm2-banpick-order-mod.md` **§12** · RE 정본 = `ANA\discovered-banpick-ai.md` §16·§17i·§17j.
> 근거 = **디스크 exe capstone+pefile 실측 대조**(Ghidra 아님) · 도구 `C:\tfm2mods\bo_sites_053.py`(인라인 phase 복제본 전수 추출기 — **다른 모드/버전 재사용 가능**), 산출물 `C:\tfm2mods\_bo_sites_053.json`.

#### 14.1 ★이번 마이그의 본질 = **훅 A(phase getter)가 함수째 소멸·인라인화** (타 모드 파급)
- 0.5.2 훅 **A `0x1cd9380`**(MSI phase getter, leaf) = **0.5.3에서 함수 소멸, 콜러들에 인라인 전개**. MSI 4벡터 합산(`+0x40/+0x58/+0x70/+0x88`)+`+0xf0`/`+0xf9` 패턴이 0.5.3 `.text`에 **0건**.
- phase 디스패처 **인라인 복제본 11개(0.5.2) → 30개(0.5.3)**, phase_from(B) **직접 콜러 26 → 3** ⟹ 0.5.2에서 "B 전체대체로 자동 커버"되던 소비처 다수가 0.5.3에선 인라인이라 **자동 커버되지 않는다**(§14.4 잔여).
- 픽 테이블 `.rdata` **`0x38397a8` → `0x3277c70`**(28B 내용 동일·하위 오프셋 `+0/+4/+0xa/+0x12` 동일).
- ★**0.5.3 신설 leaf `0x1bf3dd0` = `fn scene_step(&BanpickScene) -> u8`**(클라 콜러 **23곳**) = **A의 역할을 클라 쪽에서 승계**. ⚠반환값은 phase가 아니라 **단계 enum**: **0=밴 단계 / 1=픽 단계 / 2=픽 양팀 완료 / 0xff=그 외**. 내부식 = `total`(`+0x148/+0x160/+0x178/+0x190`), rule `+0xce`, ban `+0x3c0`, 꼬리 = `t1pick==rule+2 && t2pick==t1pick ? 2 : 0xff`.
- ✅**씬 오프셋 0.5.3 불변**(`0x148`/`0x160`/`0x178`/`0x190`/`0x3c0`/`0xce`/`0x3d0`/`0x43e`/`0x380`/`0x348`) · ✅**RMI 레이아웃 불변**(레코드 stride `0x100`, `+0xf0` ban_count·`+0xf9` rule·`+0xf8` side) — applier·commit 디스플레이스먼트 히스토그램 **완전 동일**로 실증.

#### 14.2 확정 RVA 표 (0.5.2 → 0.5.3, 전부 실측)
| 상수 | 0.5.2 | 0.5.3 | 근거 |
|---|---|---|---|
| **A** phase getter | `0x1cd9380` | ⛔**소멸(인라인화)** | 패턴 0건 |
| **A′** `scene_step`(신설 대체지점) | — | **`0x1bf3dd0`** | 씬 오프셋 지문 · 콜러 23 |
| **B** phase_from | `0x1d04120` | **`0x167c0e0`** | 진입 시그 `4d 01 c0 0f b6 c2 48 8d 15` = exe 유일 1히트 |
| **C** applier | `0x11e2140` | **`0x1bd8c20`** | 크기 547 동일 · disp 히스토그램 완전 동일 · 프롤로그 12B 동일 |
| appender pick_t1 | `0x11ce240` | **`0x1bc47f0`** | 씬 `+0x168/0x170/0x178` 지문 |
| appender pick_t2 | `0x11ce400` | **`0x1bc4980`** | `+0x180/0x188/0x190` |
| appender ban_t1 | `0x120c020` | **`0x1c028d0`** | `+0x138/0x140/0x148` |
| appender ban_t2 | `0x120c1d0` | **`0x1c02a50`** | `+0x150/0x158/0x160` |
| transition | `0x11d8ef0` | **`0x1bcf010`** | 프롤로그 동일 · 크기비 1.107 |
| banner | `0x11df9f0` | **`0x1bd63a0`** | `+0x43e`/`+0x380` 지문. ⚠**프롤로그 변경**(`56 57 53 48 83 ec 30`) — 모드는 **호출만** 해서 무관 |
| **E** lineup | `0x11cedb0` | **`0x1bc52b0`** | 프롤로그 12B 동일 |
| **F** commit | `0x1d075d0` | **`0x167fdd0`** | disp 히스토그램 완전 동일 |
| **D′** turn oracle | `0x1d07cf0` | **`0x1680500`** | 진입 13B 완전 동일 · 콜러 5↔5 대응 |
| AI 파리티 site1 / join1 | `0x1c04389` / `0x1c04475` | **`0x10a04e2` / `0x10a05f0`** | SIG1 바이트 동일·exe 유일. 컨테이너 `0x1c041c0`→**`0x10a0320`**. **al 미러 슬롯 `[rbp+0x6f]` 불변** |
| AI 파리티 site2 / join2 | `0x1c07938` / `0x1c07a09` | **`0x10a3cf8` / `0x10a3dc9`** | SIG2 유일. 컨테이너 `0x1c07880`→**`0x10a3c40`** |
| ★**신설 G**: AI턴 인라인 phase site / join | (0.5.2엔 훅 A가 담당) | **`0x1828213` / `0x18282fa`** | 컨테이너 = 서버 AI턴 `0xebe530`→**`0x1827e00`**. 스택사본 `total=[rbp+0x5eb0]`·`rule=[rbp+0x5d61]`·`ban=[rbp+0x5d58]`·합류 `mov [rbp+0x5ebf],al`. 창 **231B** |
| SFX site / end | `0x1251303` / `0x1251352` | **`0x1c56245` / `0x1c56294`** | 드레인 `0x1250370`→**`0x1c55300`**. 창 79B 동일. ⚠**씬 스택슬롯 `[rbp+0x12b0]`→`[rbp+0x12d0]`** |
| sfx 문자열 ban / pick | `0x373d596` / `0x373d5b2` | **`0x32adfb6` / `0x32adfd2`** | 문자열 검색 |
| panic hook | `0x25d4764` | **`0x28f2f34`** | 프롤로그·크기 일치 |
| `PANIC_SITES` 6종(디버그 전용) | `0x11da680` 등 | ⬜**미재핀 → 소스에서 0으로 비움** | 라벨 전용 = 기능 영향 없음 |

#### 14.3 모드 변경 · 빌드 · 배포
- 소스 = `C:\tfm2mods\tfm2_banpick_order\src\{hooks.rs, diag.rs}`. **훅 A → A′(`0x1bf3dd0` 전체대체, `hook_phase_scene`)** 로 교체 + ★**훅 G 신설**(AI턴 인라인 **38B 바이트패치** → `hook_phase_scalar` 호출).
- 빌드 = `powershell -File C:\tfm2mods\build_full.ps1 -Src ...\src\lib.rs -ModId tfm2_banpick_order -MaxSize 4000000` — **사이즈가드 초과 모드라 `build_inj.ps1` 불가**(§7.2-C와 동일 예외). dll **2,538,496B**(0.5.2 = 2,671,104B) · `mod.mod_info` **v1.1.0**·dependency **`>=0.5.3`**(BOM無 확인) · 게임 `mods\` 배포완.
- ✅**오프라인 안전검증**: 패치 4구간(G / AI1 / AI2 / SFX) 전부 **덮어쓰는 바이트로 착지하는 외부 분기 0건**(capstone 명령경계 기준), 죽은 arm 구간 인바운드도 실코드 블록만(패치 밖).
- ⬜**인게임 미검증**(게임 재시작 필요).

#### 14.4 ⬜잔여·한계 (사실 승격 금지)
- ⬜**phase 인라인 복제본 ~20개 미보정**: 드레인 `0x1c55300` ×7 · `0x15f97b0` · `0x15fad50` · `0x188dd30` ×2 · `0x188f360` · `0x1890450` ×2 · `0x1890fd0` · `0x193a940` · `0x1bd3960` ×2 · `0x24625c0` 등. 0.5.2에선 이들이 **B의 콜러**라 B 전체대체로 자동 커버됐으나 0.5.3은 인라인이라 **바닐라 phase로 남음** ⟹ **단계 배너/하이라이트/진행 게이트 불일치 가능**. 전수 메타데이터 = `C:\tfm2mods\_bo_sites_053.json`.
- ⬜AI 픽 스코어러 계열 인라인 = **미보정 유지(의도)** — 0.5.2와 동일 정책(품질만·크래시 없음).
- 재사용 도구 = `C:\tfm2mods\bo_sites_053.py`.
- ★**정정(2026-07-30 심야, 0.5.3)**: 위 "phase 인라인 복제본 미보정" 잔여 중 **UI 하이라이트 축은 phase 문제가 아니었다** — 인라인 6종을 전부 훅해도 화면 불변 ⟹ **§14.5로 판정 이관**(phase 미보정 자체는 배너/게이트 축에서만 잠재 잔여). ★**후속(07-30 낮)**: 그 잔여 중 드레인의 인라인 scene_step 재계산(`0x1c5a02a~`)이 실제로 **코치 위임 정지**를 유발 = **§14.6**(기존 drainA 스텁 상시 설치로 수정완).

#### 14.5 ★인게임 검증완 + ⛔UI 하이라이트 재시도 금지 목록 (2026-07-30 심야, 0.5.3 buildid **24451609**, 유저 확인)

**(1) ✅검증완(유저 확인)**
- **밴픽 커스텀 순서 · 팀 지정 · ~~코치 위임~~ = 정상 동작**. 20/20 완주 로그(`applier=20`·`lineup_skip=0`·커밋 커스텀 다수·`banner=2`로 밴↔픽 경계 FSM 재발동 정상·state 2→3→4 전이·종료 `phase=0xff state=5`). ⚠★**정정(07-30 낮, §14.6)**: 이 중 "코치 위임 정상"은 **구성 착시** — 이 심야 런은 `ui_highlight` 훅(drainA 포함)이 켜진 **프로브 빌드**였다. 릴리스 구성(`ui_highlight=0`)에선 drainA 미설치로 **2번째 픽부터 위임 정지**(진범·수정 = §14.6).
- **`in_turn`(선수 카드 우측 얇은 띠) + 자식 `turn_outline`(큰 카드 흰/붉은 테두리) = 커스텀 순서 추종 성공** — 모드가 SDK **`Node.visible` 직접 제어**. ★**강제 OFF 실험으로 "모드의 노드 쓰기가 렌더까지 도달함"을 실증**(하이라이트 축 조사 시 이 사실을 전제로 출발할 것).
- ~~⬜**미해결 = 슬롯 "칸 채움색" 1건**(현재 차례 칸이 팀색으로 칠해지는 것)만 여전히 바닐라 순서.~~ → ✅**규명·구현 완료(2026-07-30 오전, 0.5.3) = §14.5(6b)** / ~~⬜배포+인게임 검증 대기~~ → ✅**배포(07-30 09:49:07·2,613,248B)·인게임 검증완**(유저 "원래 안 되던 부분 잘 됨" = `+0x208` RGB write 실증 / blue `#263cbf` 위화감 보고 없음 = 간접 통과).

**(2) UI 노드 구조 = 실측 확정(재조사 불요)** — `bundle_unpacked/base/ui/layout/banpick/{red,blue}_pick_slot.ui` + 런타임 덤프
```
pick_slot : color_icon_button   (btn.back_color #1d1f2cff = 칸 기본 채움)
 ├ wait : empty      ← ★빈 카드의 **내용 컨테이너**(bar/position/proficiency_top/name)
 │                     ⛔끄면 선수 카드가 통째로 사라짐(2026-07-30 실사고·회귀)
 ├ in_turn : empty   ← 우측 얇은 띠(bar #e8e8e8ff)
 │   └ turn_outline : color #ef6471ff stroke3, 기본 visible:false  ← 큰 카드 테두리
 │                     ⚠부모(in_turn)를 꺼도 **자식이 켜진 채 남는다** → 반드시 같이 제어
 └ done : empty      ← 완료(챔프 표시)
ban_slot : #in_turn 이 **color 노드**(픽 슬롯과 달리 자체 채움)
게임 관용구: 활성 픽 슬롯 = `wait+ in_turn+ done-` (wait 는 켠 채 in_turn 을 덧켬)
```
- SDK: `Node.visible` 쓰기 가능·렌더 반영 실증 / `Node.runner.as_any()`로 러너 데이터 포인터 획득 가능(item_tactics 패턴) / 슬롯 러너 타입 = `game_view::ui::runner::color_icon_button::ColorIconButtonRunner`.

**(3) ⛔칸 채움색 — 아닌 것으로 판명된 후보 5종 (재시도 금지)**
1. **`0x1c252c0`**(슬롯 색 적용기 — `param_7`로 색세트 선택, Ghidra 확인) → **이 화면에서 호출 0회**(모드 카운터 `slotupd=0/0` 실측). 콜러 `0x1bf9560`@`0x1bfcc01`/`0x1bfd563`은 `[rbp+0x14f0]!=0` 분기 안이라 현 뷰 모드에선 미실행 = 다른 화면(관전/리플레이) 경로로 추정.
2. **`0x1bf9560`**(하단바/픽슬롯 섹션 빌더 31KB) = 씬을 안 읽는 **순수 렌더러**(phase_from 호출 0·픽테이블 0). 0.5.2 대응 `0x12028b0`도 동일.
3. **`0x1c1f270`** = `wait.name`/`in_turn.name`/`done.name` **라벨 세터**일 뿐.
4. **ColorIconButtonRunner `+0x13c`**(두 슬롯이 유일하게 갈린 필드 `0x49494949`/`0x21212121`) → 해당 상수가 `.text`에 **immediate로 존재하지 않음** ⟹ 색 결정 지점 아님(스타일 자산 유래). ★**오답 재확정(2026-07-30 오전, DR write BP 인게임 실측)**: 밴픽 내내 매 프레임 폴링에 게임 write **0건**(`fill_obs` 0) · 유일 write = 슬롯 생성 시 **외부 DLL memcpy 1회**(rip `0x7ff8f5c3cd07`/`0d0c` 외부 모듈·게임 콜사이트 RVA **`0x20733a3`** = 초기화 경로) ⟹ **렌더 미사용·재시도 금지**. 정답 = `+0x208` 계열(§14.5(6b)) — 구 메모리 §13의 "+0x208 계열" 표기가 정답이었고 모드의 구 `set_fill`(+0x13c write)은 폐기.
5. **`scene+0x3d0`** = "현재 행동 팀"이 **아니라 팀1 ID(경기 내내 고정)** — 로그 `sel=0x2b` 불변·`myT=0x2b/0x69`로 실증. 드레인 `0x1c67165`의 `cmp [rbp+0x1180],[rbp+0x1268]; cmove`(=`[scene+0x3d0]` vs `[app+0xe3b8]`)는 **좌우 진영 색조** 선택이지 슬롯 강조가 아님.

**(4) ⛔phase 계열 훅 6종 = 하이라이트에 무효 (재시도 금지)**
- 흰칸 목적으로 추가했다가 **전부 무효 판명**(probe로 전 지점이 커스텀 phase를 정상 수신함에도 화면 불변): 훅 **I** `0x193b434`(match_ui 인라인 phase) / **J** 드레인 cur `0x1c6605d`·next `0x1c66374` / **K** 드레인 3사이트 `0x1c5a0b2`·`0x1c5a5b9`·`0x1c5a9b1` / **M** 흰칸 개수 루프 `0x1c5aa99` / **N** 씬 원시 phase leaf `0x1bce8e0` / **O** `0x1c252c0`.
- ~~현재 소스에서 **cfg `ui_highlight` 기본 OFF**로 격리~~ → ★**정정(07-30 릴리스, 0.5.3): `ui_highlight` 기본 `true`로 변경** — 이 세트 안의 **drain cur/next(훅 J)·개수 루프가 코치 위임 펌프의 state-4 타이머 전진에 load-bearing**임이 인게임 확정(끄면 위임 턴 total 불변 정체). ⛔**끄지 말 것**(drainA(K)와 별개 축). 표시(하이라이트) 축은 여전히 무효이나 진행 축에서 필수. 노드 제어(`in_turn`/`turn_outline`)는 상시 동작.
- ★★**역할 정정(2026-07-30 낮, 0.5.3·ghidra-re — 재발 방지 핵심, 전문 = §14.6)**: "하이라이트에 무효" 판정 자체는 유효하나, **훅 K의 1사이트 `0x1c5a0b2`(DRAIN_HL2 "drainA_step", hooks.rs L1568) = 하이라이트용이 아니라 클라 자동행동 펌프의 load-bearing 게이트**(§14.6 체인의 인라인 바닐라 scene_step 재계산 디스패치). `ui_highlight` 격리에 같이 묶인 탓에 릴리스 구성에서 **코치 위임 정지** 유발 ⟹ 수정 = `install_drain_hl2`(3사이트) **게이트 밖 상시 설치**. "하이라이트 무효(표시 축)"와 "펌프 게이트(진행 축)"를 구분해 기록할 것.
- ⚠**크래시 실사고**: 최초 훅 **H**(AI 6사이트) 시도에서 **합류주소 2곳 오판 + arm 내부 부작용 명령(`mov rdx,[rbp+0x130]`) 누락**으로 크래시(`c0000005 @ 0x10a039b`). ★교훈(버전무관) = 인라인 복제본 패치 시 **①합류점은 arm의 `jae` 타깃으로 확정 ②arm 내부 메모리 로드(부작용)를 스텁이 재현 ③완료(0xff) 경로가 별도 주소로 갈리는지 확인**.
- ⚠**회귀 실사고**: `wait` 노드를 "다음 차례 흰색"으로 오인해 끄자 **선수 카드 UI 전체 소실**(wait = 카드 내용 컨테이너).

**(5) 신규 확정 사실 (0.5.3 구조)**
- ★**씬→phase leaf 2종 분리**: **`0x1bf3dd0`** = 단계 enum(0=밴 1=픽 2=완료 0xff, 클라 콜러 23) / ★**`0x1bce8e0` = 원시 phase(0..3, 0xff)**(콜러 `0x2262ca0`@`0x2262f25`, 소비 = `lea ecx,[rax-2]; cmp ecx,2`로 밴단계 판정). **둘 다 모드가 전체 대체 중**(A′·N).
- ★**흰칸 "동시 점등 개수" 공식 = 드레인 `0x1c55300`의 연속 same-phase 카운트 루프**(정본이 ⬜로 남겼던 것): `0x1c5aa95 lea rdi,[rsi+rdx]` → phase → `0x1c5ab31 cmp dil,r8b` → 같으면 `rdx++` → 결과 `[rbp+0x520]`.
- UI 빌더 **`0x1bd94f0`**(0.5.2 `0x11e2980` 대응) = 0.5.2와 동일하게 **phase_from을 call**하고 같은 시퀀스(`cmp al,0xff`→`cmp al,2`→팀 XOR) 유지 ⟹ **훅 B로 이미 커버됨**.

**(6) ⬜다음 세션 진입점 = 런타임 하드웨어 BP (정적 추적 금지)**
- 칸 채움색 1건만 남음. 권장 = 모드가 매 프레임 슬롯 노드 포인터를 이미 알고 있으므로, 그 노드의 **색 필드(`+0x208` 계열)에 DR 레지스터 write BP**를 걸어 쓰는 명령 주소를 실행 중 포착(모드가 이미 VEH 사용 중 = 구현 가능). ⛔**정적 추적은 후보 5종이 전부 빗나갔으므로 재시도 금지.**
- ✅**프로브 구현·배포완(2026-07-30 09:21, 0.5.3)**: `diag.rs` `arm_watch()`(현재 스레드 DR0~3·4바이트 write BP·CONTEXT `+0x48~+0x70` 직조작) + VEH `EXCEPTION_SINGLE_STEP`(0x80000004) 분기(Dr6&0xF 검사 → 유니크 rip를 crash_log.txt `DRBP dr6=... rip=g... stack:...`로 기록 → Dr6 클리어 → CONTINUE_EXECUTION) + `drbp_stats()` / `lib.rs` `drbp_probe()` = blue/red `pick_slot_0·1`의 ColorIconButtonRunner **`+0x13c`**에 BP arm + 전 픽슬롯 `+0x13c` 폴링(`fill_obs` 로그) + 120프레임마다 `drbp armed=... hits=...` → order_log.txt·drbp 중 `set_fill` 자기 write 중단 / `config.rs` cfg `drbp` 신설(기본 0). ~~⬜**인게임 실행 대기**(유저가 밴픽 1회 돌려야 로그 생성 — **칸 채움색 기록자 규명 = 진행중, 결론 아님**)~~ → ✅**실행 완료·규명 확정(2026-07-30 오전 = 아래 (6b))**. ⚠최초 arm 대상 `+0x13c`는 오답 재확정((3)-4 정정) → 프로브(drbp_probe)는 `+0x208` 감시로 재조준.

**(6b) ★★칸 채움색 = 규명 완료 (2026-07-30 오전, 0.5.3 buildid 24451609 — DR write BP + 러너블록 프레임 diff 프로브 2회 인게임 실측 / 근거 = `order_log.txt` `rdiff` 줄 + `crash_log.txt` `DRBP` 줄)**
- ★**라이브 렌더 소스 = `ColorIconButtonRunner` 데이터 `+0x208/+0x20c/+0x210`의 RGB f32 3워드**(알파 `+0x214`는 불변·미터치). 게임은 **턴 전환 순간에만** 이 3워드를 쓰고 draw가 매 프레임 읽음 ⟹ **post_update 매 프레임 덮어쓰기로 모드가 지배 가능**(in_turn visible 제어와 동일 성립 구조).
- **팔레트 확정**: 기본 `#1d1f2c`(rdiff 실측·.ui back_color 일치) / 다음 차례 `#4a4c56`(실측) / 현재 차례 = 팀색: red `#b02e3a`(실측) / blue `#263cbf`(champion_slot.ui L95↔L124 짝 = **추정** — ⬜인게임 검증 대상). f32 인코딩 = `n/255.0` 비트동일 실측(`29/255 = 0x3de8e8e9`).
- ★**프로브 방법론(재사용 가치)**: ①DR0~3 write BP(`diag.rs` `arm_watch` — 현재 스레드 CONTEXT 직조작 + VEH SINGLE_STEP 분기) ②러너블록 ≤0x400B 프레임 diff(`rdiff` — VirtualQuery region_cap 가드·(슬롯,오프셋)당 4회 로그 억제) ⟹ **정적 추적 전멸 상황에서 2라운드 만에 필드 특정**.
- **구현(완료)**: `src\lib.rs` **`set_fill_rgb()`** 신설(+0x208 RGB f32 3워드·비트동일 멱등 write)·구 `set_fill`/+0x13c 상수 삭제. 3상태 배선 = **Current→팀색**(그룹 좌우로 결정: `blue_picks`→`#263cbf` / red→`#b02e3a`) / **Next→`#4a4c56`** / **None→`#1d1f2c`**. ⬜배포+인게임 검증 대기 = (7).

**(7) 빌드·배포 상태**
- 소스 = `C:\tfm2mods\tfm2_banpick_order\src\{lib.rs,hooks.rs,config.rs,diag.rs}` · 빌드 = `build_full.ps1 -MaxSize 4000000`.
- ~~마지막 빌드 dll **2,585,088B**(직전 배포본 2,584,576B) — ⚠**게임 실행 중이라 배포 대기**, 임시 경로 `%TEMP%\tfm2_build\tfm2_banpick_order_24336\`~~ → ✅**해소(2026-07-30)**: 그 빌드는 09:11 배포됨 → **drbp 프로브 추가 빌드가 최종 = 게임 `mods\tfm2_banpick_order\tfm2_banpick_order.dll` 2,592,256B, 07-30 09:21:22**(`build_full.ps1` `OK: deployed` + Get-Item 실측). 배포 cfg = ~~**`drbp=1`·`debug=1`(진단 회차)**~~ → **`drbp=0`(칸 채움색 수정 활성 조건)·`debug=1`(07-30 오전 — 릴리스 시 둘 다 0)**.
- **(2026-07-30 오전 갱신)** 현 배포본 = ~~라운드2 프로브 빌드 2,612,736B(09:32)~~ → 칸 채움색 수정(§14.5(6b)) 빌드 **2,613,248B = ✅배포(09:49:07)·인게임 검증완**. ⚠cfg가 PS5.1 재저장 사고로 **BOM+한글 주석 파손** → 템플릿 기반 재작성 완료(BOM無 확인·값 무변).
- **(2026-07-30 낮 갱신)** 코치 위임 수정 + Next 표시 정책(§14.6) **합본 빌드 = 2,613,248B(채움색 빌드와 동일 크기 = 우연)**, 보관 `%TEMP%\tfm2_build\tfm2_banpick_order_31764\` — ~~⚠⬜**배포 대기(게임 실행 중 dll 락)** / ⬜인게임 검증(코치 위임 20/20·Next 표시) 대기~~ → ✅**배포(07-30 10:38:17)·인게임 재검증 = 색칠 전부 정상·코치 부분개선**(⚠일부 T2 턴 **랜덤 영구 정지 잔존** → 진범·수정 = §14.6 후속 2차). → ✅✅**v1.2.0 릴리스 완료(07-30)**: 워치독 킥 빌드 **배포 dll 2,606,592B @11:31:27**·릴리스 zip **840,712B**·deploy-verify 6항목 PASS·cfg 프로덕션 정규화(debug=0) — 코치 위임은 "완주하되 워치독 의존"(근본 미해결·차기 과제)으로 릴리스 = §14.6.
- cfg 신설 키: **`ui_highlight`**(기본 0 = 무효 훅 격리) · **`hl_force_off`**(진단용 강제 OFF, 기본 0) · **`ai_inline_phase`**(기본 0 = 크래시 냈던 훅 H 격리).
- ⬜**`debug=1` 상태로 둠**(다음 세션 진단 계속용 — **릴리스 시 0으로**).

#### 14.6 ★★코치 위임 정지("2번째 픽부터 안 해줌") 진범 = 클라 자동행동 펌프의 인라인 바닐라 scene_step 게이트 (2026-07-30 오전~낮, 0.5.3 buildid 24451609, ghidra-re 확정 — ~~수정 빌드완·⬜배포 대기~~ → ✅배포·부분개선 / ★후속 2차 = **랜덤 영구 정지 진범 = 트리거 dedup L1 지문 캐시 교착** + 워치독 수정 = 말미 후속 블록)

- ★**체인(전 구간 바이트레벨 확정)**: 클라 펌프 = 드레인 `0x1c55300`(매 프레임, 가드 `+0x446==2`·`+0x43e==0`·`+0x348==-1`·`+0x428<=0`·`+0x220==0`) → `0x1c55743 switch([scene+0x380])`(JT `0x32ba308`: state2→al=0/state4→al=1/state7→al=2) → **인라인 바닐라 scene_step 재계산(`0x1c5a02a~0x1c5a274`, 디스패치 `0x1c5a0b2`) → dl** → `0x1c5a28d cmp al,dl; jne 스킵` → 일치 시 `call 0x1bf77d0`(트리거: A′/B 대체분으로 재확인 + 내 팀 `ctx+0xe3b8` 확인 + 4벡터 지문 dedup → **ClientPacket disc 0x93(AI턴 요청) 큐잉**, push `0x1c10550`, `scene+0x1f8`) → 서버 CP 디스패처 `0x17e0240`(2단 JT `0x3284d48`/`0x3284d5c`, disc 0x93=idx145→arm `0x17e6536`) → **`0x17e659f call 0x1827e00` = 서버 AI턴의 유일 정적 콜사이트**.
- **기전**: 인라인 재계산이 모드 미보정이라 커스텀 픽 블록에서 바닐라 계산=밴 ≠ 기대 step=픽 → `jne` 스킵 → 펌프 정지. **수동 픽은 클릭 경로라 무관**. (~~가설 "팀비트 불일치"~~ → **step(밴/픽) 불일치**가 메커니즘.)
- ★★**역할 정정(§14.5(4)의 정정 bullet과 동일 건)**: 디스패치 `0x1c5a0b2` = 모드가 이미 검증된 2-way 스텁을 보유한 **훅 K DRAIN_HL2 "drainA_step"(hooks.rs L1568)과 동일 사이트** — `ui_highlight`(기본 0) 격리 탓에 릴리스 구성 미설치 → 코치 정지. §14.5(1) "코치 위임 정상" 통과 이유 = 그 심야 런이 훅 켜진 프로브 빌드(구성 착시).
- ✅**수정 = `install_drain_hl2`(3사이트)를 ui_highlight 게이트 밖 상시 설치로 이동**(hooks.rs). **신규 RE 패치 불요** — 기존 스텁 재사용(합류 `0x1c5a288` out=dl 불리언 픽=1/밴=0·완료 별도경로 `0x1c5a274`·크래시 0 인게임 실증 구성).
- **함께 반영 = "다음 차례" 표시 정책 변경(유저 결정)**: 인게임 확인 = "내 4픽 때 상대 4픽 칸이 같이 빛남"은 모드 Next 표시(`#4a4c56`)가 **정확히 동작한 것**(실제 다음 행동자가 상대 4픽)·버그 아님 — 단 유저 선택 = **같은 팀 연속일 때만 표시**. 수정 = `lib.rs` apply_turn_highlight: `same_team_next = next_phase != 0xff && (next_phase&1)==(phase&1)`일 때만 want_next.
- ⬜**미확정(사실 승격 금지)**: "1번째 픽은 됐다"의 발화 경로(서버 자기연쇄 `0x17f0790` D′ 콜사이트 후보 — 수정 방향 무영향 / ★후속 2차로 **서버측 패치 필요성 축은 종결** = 아래 가설2 기각) / `0x188dd30`·`0x188f360`·`0x1890450`·`0x1890fd0` = AI턴 내부 추천 서브루틴(품질층·기존 정책대로 미보정 무방).
- **빌드·배포**: 수정 2건 합본 빌드 = **2,613,248B**(§14.5(6b) 채움색 빌드와 동일 크기 = 우연), 보관 `%TEMP%\tfm2_build\tfm2_banpick_order_31764\` / ~~⚠⬜배포 대기(게임 실행 중 dll 락) / ⬜인게임 검증 대기~~ → ✅**배포(07-30 10:38:17)·인게임 재검증 = 부분개선**(훅 K 3/3 상시설치 유효 실증: T1(상대 AI) 턴 즉시·T2(위임) 턴 2~6초 진행) ⚠단 **일부 T2 턴이 랜덤 위치에서 영구 정지**(1차 런 P2#2·B2#5 / 2차 런 B2#1 — 위치가 런마다 다름 = 비결정 레이스) ⟹ 후속 2차로 규명.

**(후속 2차, 2026-07-30 낮 — ★랜덤 위치 영구 정지 진범 = 트리거 dedup 3층 중 L1 지문 캐시 교착 · ghidra-re 2차 조사(디스크 exe capstone 실측) · 워치독 수정 구현완·⬜배포 대기)**
- ★**트리거 `0x1bf77d0`의 재요청 차단 3층(전 오프셋 확정, 0.5.3)**:
  - **L0 "씽킹 in-flight" 레코드**: `scene+0xe0`(match dword, **sentinel -1 = 무효**)·`+0xe8`(set)·`+0x120`(내팀id)·`+0x128`(total)·`+0x130`(step u8)·`+0xf8/+0x100`(추천 Vec) — 전부 일치 시 bail. 서버 0x56 응답이 채움.
  - ★★**L1 지문 String 캐시 = 교착의 진원**: format!(`0x1400339e0`) String(구성 = scene+0xd0 16B·내팀id·step·total·4벡터 챔프명 전부) / 저장 `scene+0x288(cap)/+0x290(ptr)/+0x298(len)` / 비교 `0x1bf7cee`(len+memcmp). **"요청 발사" 시점(`0x1bf7d77`)에 갱신** ⟹ 요청이 서버에서 무산되면 같은 국면의 지문이 영원히 동일 = **재요청 영구 차단**(해제 조건이 커밋뿐 = 교착).
  - **L2 큐 중복 스캔**: `scene+0x200(ptr)/+0x208(len)`, stride **0x740**, 엔트리 `+0=disc/+8=match/+0x10=set` — 드레인 소비로 자연 해제(일시적).
- ★**요청 무산 경로(가설3 확정)**: 서버 AI턴 `0x1827e00`의 `0x1828303 call D′(0x1680500)` → `0x1828309 cmp rdx,rbx`(요청 참가자 팀) 불일치 → **`0x182813f mov qword[rsi],-1` 조기 리턴** → 메가함수 `0x17e65a5 cmp -1; je 0x17ed78a` = **응답·커밋 없이 요청만 소비**. 발생 기전 = 프레임 레이스(직전 커밋 브로드캐스트가 미드레인 상태에서 다음 요청 발사) — 정지 위치의 비결정성과 부합.
- ⛔**가설2(서버 자기연쇄 내 미보정 바닐라 phase) = 기각·서버측 추가 패치 대상 없음(재조사 금지)**: 메가함수 `0x17e0240`(168KB) 전수 스캔 = pick_table rip-rel **0건**·4벡터합 패턴 **0건** / D′ 콜사이트 2곳(`0x1828303`/`0x17f0790`) **전부 모드 대체 함수 경유**.
- 부기: `0x1bf77d0` 프롤로그 = `55 41 57 41 56 56 57 53 48 81 ec 38 08 00 00`(**ORIG_LEN=15** — 12B로 자르면 sub 명령 절단 = 주의)·rip-rel 없음 / 클라 패킷 push `0x1c10550` 직접 호출 = 중위험 shadow-call·**불요 판정**(워치독 방식이 대체).
- ✅**수정 = 모드 워치독 킥(바이트패치 아님·필드 write 2개·구현완)**: `hooks.rs tick()` — `IN_BANPICK && CUSTOM_ACTIVE` && total(Σ scene 4벡터 len) **480프레임(8s > 씽킹 최대 ~6s) 정체** && phase!=0xff → **`scene+0x298=0`**(L1 지문 len=0 — String 해제는 cap/ptr 기준이라 안전) + **`scene+0xe0=-1`**(L0 sentinel 무효화) → 펌프가 자동 재발사. 재무산 시 다음 주기 재킥(재시도 루프). 수동 턴에 킥돼도 무해(서버가 -1 무시). 상태 static = `WD_LAST_TOTAL`/`WD_FRAMES`/`WD_KICKS`·debug 시 "watchdog: kick#N" 로그.
- **빌드·배포(2차)**: 워치독 빌드 성공 **2,615,296B** / ~~⚠⬜배포 대기(게임 실행 중 dll 락)~~ → ✅**배포완·v1.2.0 릴리스 완료(2026-07-30)**: 게임 `mods\` dll **2,606,592B @11:31:27**·릴리스 zip **840,712B**(`<게임설치>\mods\release\0.5.3\`·PII0·로그0·mod_info v1.2.0·deps `>=0.5.3, <0.5.4`·cfg 프로덕션 정규화)·**deploy-verify 6항목 PASS**(참고: zip 내 README.txt만 UTF-8 BOM 있으나 게임 미read 문서라 무해).
- ★★**코치 위임 = "완주하되 워치독 의존"으로 릴리스 확정(현행 최종 상태·사실 승격 금지)** (2026-07-30, 0.5.3):
  - ★**ui_highlight 세트가 코치 위임 펌프에 load-bearing** — 인게임 확정: `ui_highlight` 훅 안의 **drain cur/next(훅 J)·개수 루프가 드레인 state-4 타이머 전진에 관여** ⟹ 끄면 위임 턴이 total 불변으로 정체. ⟹ **`config.rs` 기본 `ui_highlight=true`로 변경**(⛔끄지 말 것). drainA(훅 K)와는 **별개 축**.
  - ⚠**미완(승격 금지)**: ui_highlight ON + 워치독 상시 구성에서 **20/20 완주하나 워치독이 한 경기 ~30회 발화**(위임 턴마다 8초 정체 → L1 지문 무효화로 재개) ⟹ **정체 자체는 잔존·워치독이 뒷수습**. 근본원인(위 L1 지문 교착 + AI턴 `0x182813f` 조기리턴 레이스) **미해결**. **유저 결정(07-30) = "이대로 릴리스"**(체감 수용) ⟹ ⬜**매끄러운 해결(워치독 0~1회)=차기 과제**.
  - 워치독 = tick()에서 total 480프레임 정체 시 `scene+0x298=0`(L1 지문 len)+`scene+0xe0=-1`(L0 sentinel) 무효화. **상시 동작(debug 무관)**. Next 표시 정책 = `same_team_next`(같은 팀 연속만 밝은 회색)·색칠(칸 채움색 +0x208 RGB f32 + 다음차례 회색) = 완전 해결·인게임 확정.
- 직전 배포본 = 2,613,248B 10:38:17(훅 K 상시설치+Next 정책·색칠 전부 정상 검증됨).

**(후속 3차 = ★★v1.2.1 — 2026-07-31, 0.5.3 buildid 24451609 · 회귀 2건 수정·인게임 검증완·릴리스완 / 이 블록이 §14의 현행)**
- ★★**회귀 1 = 드레인 훅 호출 4줄 소실(v1.2.0 릴리스 빌드)** — `install()` 안에 "J·K·L·카운트 = **상시 설치**" 주석만 남고 **`install_drain_hl`/`install_drain_hl2`/`install_slotsel`/`install_hl_count` 호출이 통째로 삭제**된 채 릴리스. 증상 = 코치 위임이 **2번째 픽에서 영구 정지**(워치독 킥에도 재개 안 됨).
  - **진단 지문(재사용)**: `patched: … **drain=0+0 slot=false cnt=false**` / `trig` 고정(트리거 `0x1bf77d0` 호출 0회) / `state=4 timer` 동결 / `l0m`·`l1`은 킥으로 무효화됨(=**워치독 자체는 정상**, 부를 트리거가 없어 무효) / `latch=0 defer=2 anim=-1 cd=0 g220=0`(다른 가드 정상).
  - **물증** = dll 크기: 2,613,248B(07-30 10:38 검증본) → **2,606,592B**(11:31 릴리스본, −6,656B) → 복구 후 **2,615,808B**. 07-30 워치독 빌드 2,615,296B와 근접 ⟹ 워치독 빌드까지는 훅 생존, **릴리스 정리 빌드에서 소실**.
  - **수정** = 호출 4줄 복구(게이트 밖 상시) + 주석에 회귀 경고 명시. 소스 잘린 주석(`— K만 상시로는 정지 재현(타이머 동`)도 복원.
- ★★**회귀 2 = SFX 씬 스택슬롯 반쪽 마이그** — 0.5.3에서 `[rbp+0x12b0]`→`[rbp+0x12d0]`으로 바뀐 것을 **`SFX_SIG`(검증용)에만 반영하고 `install_sfx`의 emit 본문은 `0x12b0` 구값 유지**. 결과 = 엉뚱한 슬롯을 씬으로 읽어 `sfx_is_pick`이 `addr_ok` 실패 → 0(밴) 반환 → **픽 차례에도 밴 효과음**. ⚠시그 검증은 새 값으로 통과하므로 로그엔 `sfx patch: OK`로 정상 표기됨(= "설치 OK ≠ 올바른 동작"). **수정 = emit 바이트 `0xb0`→`0xd0` 1B**.
- ✅**인게임 검증완(2026-07-31, 유저 확인)**: 훅 `hook J 2/2 · K 3/3 · L OK · M OK` = `patched: drain=2+3 slot=true cnt=true` / **20/20 완주**(`vec=5.5.5.5 total=20 phase=0xff`·`lineup_skip=0`·`forced_pick=6`) / ★**`watchdog` 발화 0회**(2런 연속) / 효과음 = 유저 "픽소리 잘난다".
- ★★**판정 정정(§14.6 후속 2차 뒤집음)**: ~~"코치 위임 = 완주하되 워치독 의존(~30회 발화)·정체 잔존·근본 미해결·매끄러운 해결=차기 과제"~~ → **철회**. **J·K·L·M 4종이 전부 설치된 구성에서는 정체가 관측되지 않는다(워치독 0회)** ⟹ 그 "~30회 발화" 수치는 훅 일부가 빠진 구성의 것이었다. 워치독은 **잔존 안전망**으로 유지(L1 교착 레이스가 이론상 남고 비용 0). ⛔단 **서버측 추가 패치 대상 없음·재조사 금지**(가설2 기각)는 그대로 유효.
- **릴리스 v1.2.1**: 배포 dll **2,615,808B @2026-07-31 01:30:29**(`build_full.ps1 -MaxSize 4000000` → `OK: deployed (verified)`) · mod_info **v1.2.1**·`last_updated 2026-07-31`·deps `>=0.5.3, <0.5.4`·BOM無(7b) · cfg `debug=0`(BOM無 23) · zip **836,489B**(`<게임설치>\mods\release\0.5.3\`·엔트리 4·`rel_one.py` 생성) · `rel_verify.py` **OK**·`pii_check.py` **PII 0건**.
- ★**교훈(버전무관·배포 절차에 편입)**: ①**검증 통과 dll ≠ 릴리스 dll이면 릴리스본으로 재검증**(크기·해시 대조가 신호) ②**주석이 문장 중간에 끊겨 있으면 코드 삭제 사고 의심** ③load-bearing 훅은 **설치 카운터를 로그로 노출**(이번 진단이 5분에 끝난 이유) ④**같은 상수를 시그니처와 패치 본문에서 함께 쓰는 코드는 마이그 시 짝으로 갱신**(한쪽만 고치면 검증 통과 후 조용히 오작동).

### 15. ★`tfm2_comptest_unlock` 0.5.3 — **일일횟수 제한(`daily_remaining`) 미해결 건 해결**(2026-07-30, 0.5.3 buildid **24451609**) — **본 절 = 이 건의 정본** / ⬜**인게임 미검증**

> ★★**2차 스윕 정정(07-30 오후, 유저 실증 후)**: 1차(오전 09:22, 4사이트) 배포에도 유저 실증에서 **일일 5회 제한 잔존**(증상 = 버튼은 눌리나 안내문구 뜨며 거부·체력 무소모는 정상). 2차 ghidra-re 스윕으로 **진범 = 서버 comp_test 핸들러의 "수락 판정" daily 게이트(1차에 놓친 2번째 게이트)** 규명 → §15.5. ⟹ **실효 = server_pregate `0x17ef5f6`(04→ff) + inc_gate `0x17f239c`(→ff 상향) + 클라 `dr_inline_b`** 뿐. **`dr_inline_c` 폐기·원본 되돌림**(게이트 아니라 시드 성분), **a/d = 게이트 아님·표시용 포맷 인자**(무해). 재배포 = **203,776B @2026-07-30 09:56:25**.

> 배경: 07-30 00:04 배포본에서 유저 실증 = **`no_stamina_cost`는 작동 / 일일 실행 제한은 잔존**. 원인 = 마이그 시 `daily_remaining`을 **rva 0(스킵)** 으로 남겨둔 것. 0.5.2 값 `0x1f14090`으로 마스크시그를 돌려도 0.5.3에서 **0건**이었다.

#### 15.1 ★시그 0건의 진짜 이유 = 함수 재작성이 아니라 **완전 인라인화 + 의미 반전**
- ~~"leaf 함수가 재작성돼 시그가 깨졌다"~~ → **함수 자체가 사라졌다**(전 호출부에 인라인 전개). ⟹ 종전 수법인 **`mov eax,5; ret` 통짜 대체 = 원리적으로 불가**(대체 대상이 없음).
- ★**의미도 반전**: 0.5.2는 `remaining = max(0, 5 − count)`를 반환했으나, **0.5.3은 `used = (rec_id == outer_id) ? min(count, 5) : 0` 을 계산하고 `used >= 5` 로 차단**한다(잔여수 → 사용수). ⟹ "5를 돌려주기"가 아니라 **"used를 0으로 만들기"** 가 올바른 개입.
- **레코드 필드 = 컨테이너-상대 `+0x18` 시프트**(0.5.2 → 0.5.3): rec_id `base+0xdc04` → **`base+0xdc1c`** / count `base+0xdbf8` → **`base+0xdc10`** / outer_id `base+0xe41c` → **`base+0xe434`**.

#### 15.2 패치 = 클라 인라인 **4사이트 개별 패치**(구 `daily_remaining` 1항목을 대체)
| 이름 | RVA | orig | fixed | 위치·역할 |
|---|---|---|---|---|
| `dr_inline_a` | `0x18d9436` | `4c 0f 44 e2` (`cmove r12,rdx`) | ~~4B NOP~~ → **게이트 아님(무해)** | ~~클러스터 `0x18d9411`~~ → **표시용 포맷 인자**(07-30 2차 정정)·유지해도 무해하나 **사용횟수 표시 0 고정 부작용** |
| `dr_inline_b` | `0x18e3fd6` | `41 20 c5` (`and r13b,al`) | **`45 30 ed`** (`xor r13b,r13b`) | ★**실효 클라 게이트 = 이것 하나**(디컴 확정 `if(4<count&&rec_id==outer_id)ok=0` → run 버튼 + tactics 노드 `[+0x261]` 공유) |
| `dr_inline_c` | `0x18f18c7` | ~~`4c 0f 44 e1` (`cmove r12,rcx`) → 4B NOP~~ → ⛔**폐기·원본 되돌림** | ~~클라 RUN 핸들러 컨테이너~~ → **게이트 아니라 클라 요청 페이로드의 시드 성분**(`seed=(used|X<<32)^epoch_ms`)·서버는 자기 레코드로 판정하므로 무의미 + 시드 변화 부작용 회피(07-30 2차) |
| `dr_inline_d` | `0x1987a3d` | ~~`4c 0f 44 f8` (`cmove r15,rax`) → 4B NOP~~ → **게이트 아님(무해)** | ~~버튼 회색화의 실체~~ → **표시용 포맷 인자**(07-30 2차 정정)·유지해도 무해(표시 부작용) |

- ★**패치 원리(안전성 근거)**: A/C/D는 **선행 `xor`로 0을 만든 뒤 조건부로 count를 옮기는 형태**라 cmove를 NOP하면 `used = 0` ⟹ **게임 자신의 fresh-day(당일 첫 실행) 분기와 완전히 동일한 정상 상태**가 된다(상수 위조·경로 신설 없음). B는 exhausted 플래그를 0으로 고정.
- ★**레코드 write 경로 = 별개**(`0x9d67a3` / `0x9d6c9b` / `0x9d6cb4`) ⟹ **일일 카운터 기록 자체는 그대로 진행**되고, 표시·차단만 해제된다(세이브 정합 유지).

#### 15.3 서버 `daily_inc_gate` 0.5.3 = **`0x17f239c`** — imm8 `04`→**`ff` 상향**(07-30 2차 정정)
- `0x17f2386 cmp [rbx+0x1dc],esi` → `0x17f2392 mov rax,[rbx+0x1d0]` → `0x17f2399 cmp rax,4` → **`0x17f239d jbe`**. cmp의 **imm8 `04` → `ff`**(=-1 sign-extend)로 `jbe` 항상 taken = 무제한.
- ★**정정(07-30 2차)**: 1차의 fixed `7f`(127)는 헛점 — inc_gate가 카운터를 계속 증가(`0x17f6df7 inc rax; mov [rbx+0x1d0],rax`)시켜 **127 도달 시 재차단**된다 ⟹ `ff`로 상향 확정. 전역 시퀀스 **1히트(클론 아님)·라이브 검증 PASS**. 허용경로 = `0x17f2302`.
- ★**서버측은 레코드를 "포인터 + 내부오프셋 `0x1d0`/`0x1dc`"로 접근**한다 ⟹ 클라측처럼 **컨테이너-오프셋(`base+0xdcxx`)으로 스캔하면 절대 안 잡힌다**. **두 축을 같은 시그로 찾으려던 것이 그간 헛수고의 원인** — 다음 패치에도 **클라=컨테이너 상대 / 서버=포인터+내부오프셋**으로 분리해 탐색할 것.

#### 15.5 ★★진범 = 서버 "수락 판정" 사전거부 게이트 `server_pregate` **`0x17ef5f6`**(07-30 2차 스윕)
- 서버 comp_test 핸들러엔 daily 게이트가 **2개**인데 1차는 증가게이트(§15.3)만 패치, **수락 판정 게이트를 놓쳤다** = 유저가 본 "안내문구 뜨며 거부"의 실체.
- 사이트 = `0x17ef5ef cmp qword [rax+0x1d0],4`의 **imm8** → `orig 04` → **fixed `ff`**(=-1 sign-extend → `jbe` 항상 taken = 무제한). fall-through `0x17ef5fd mov byte [rsp+0x20],1`이 **거부코드 1(`no_attempts`) 생산** → `0x17ef616` 거부 디스패처(`FUN_141827af0`·호출 5곳·arg5=`[rsp+0x20]`가 코드값)로 전달.
- ★**code 1 생산지는 exe 전체 이 2곳뿐**(둘 다 daily 직후)·전역 시퀀스 `48 83 b8 d0 01 00 00 04` **1히트(클론 없음)**. **no_stamina_cost와 같은 라이브 pdata 함수(`0x17e0240`..`0x180924f`) 내 = 라이브 확정**. 허용경로 = `0x17f2302`.
- ⚠**불확실(추정 유지)**: arg5=1 ↔ `no_attempts` 대응은 디스패처 인자분포 + 문맥 기반 **고신뢰 추론**이지 문자열 테이블 직접 디코드 아님 ⟹ **인게임 실증이 최종 확인**.

#### 15.4 소스 반영 · 빌드 · 배포
- 소스 = `C:\tfm2mods\tfm2_comptest_unlock\src\tfm2_comptest_unlock.rs` **PATCHES 테이블**: ~~구 `daily_remaining`(rva 0) → `dr_inline_a`~`dr_inline_d` 4개~~ → **2차 정정**: `server_pregate`(= `daily_inc_gate` 바로 뒤 배치) 추가·`inc_gate` fixed `ff` 상향·**`dr_inline_c` 제거(원본 유지)**. 실효 항목 = server_pregate + inc_gate + dr_inline_b (a/d 무해 유지).
- ~~`OK: deployed tfm2_comptest_unlock.dll = 194048 bytes @ 2026-07-30 09:22:59`(1차)~~ → **2차 재배포**: `build_inj.ps1` **exit 0** · **`OK: deployed 203,776 bytes @ 2026-07-30 09:56:25`**.
- ⬜**인게임 미검증**(유저 테스트 예정) — 검증법 = 게임 완전 재시작 후 **comp_test를 일일 5회 초과 실행**(안내문구 부재·정상 실행 확인).
- ⚠**릴리스 전 잔여** = 검증 라운드용 **`LOG_ENABLED=true` 임시 ON 상태** → 확인 후 `false` 복귀 필요.
- 별건(⬜미해소) = comptest `LOADER`의 **`0x91ab0` 오답 재검증**(§7.3 §13.3 · `_MIGRATE_053.md` §1b) — 이 건과 무관.

---

### 16. ★`tfm2_level_cap` 0.5.3 — **훅 2사이트 재핀·빌드·배포완(v2.1.0)**(2026-07-31, 0.5.3 buildid **24451609**) — **본 절 = 이 건의 0.5.3 정본** / ⬜**인게임 미검증** (0.5.2 구현 정본 = §7.2-A12 §6)

#### 16.1 배포 실측 (근거)
- `build_inj.ps1` **exit 0 / `OK: deployed`** → `<게임설치>\mods\tfm2_level_cap\tfm2_level_cap.dll` = **198,144B · 2026-07-31 00:55:12**.
- 배포본 바이트 스캔 = **신 RVA 각 1건 / 구 RVA(0.5.2) 0건**.
- `mod.mod_info` 배포완 = version **2.1.0** · deps **`>=0.5.3, <0.5.4`**(0.5.4 자동 비활성 정책 준수) · **BOM 없음**(첫 3B `7b 0a 20`).
- ⬜**인게임 미검증**(훅 설치 로그·레벨 13+ 도달·경험치 바 정상·크래시 0 전부 미확인) · ~~⬜**0.5.3 릴리스 zip 미생성**~~ → ✅**릴리스 zip 완료(07-31, 검증 12/12 PASS)** = §16.7.

#### 16.2 훅 RVA 재핀 (0.5.2 → 0.5.3) — 둘 다 실측 확정
| 사이트 | 상수 | 0.5.2 | **0.5.3** | 원본 7B | 컨테이너 0.5.2 → 0.5.3 (body 오프셋) |
|---|---|---|---|---|---|
| 레벨업 | `RVA_LEN_LOAD` | `0x22d3fea` | **`0x12c5b44`** | `48 8b 90 10 0d 00 00`(`mov rdx,[rax+0xd10]`) | `0x22d3c60` → `0x12c56d0`(`-0x12c5e69`) |
| UI 경험치 바 | `RVA_UI_CMP` | `0x80ae73` | **`0x95a359`** | `48 3b 88 10 0d 00 00`(`cmp rcx,[rax+0xd10]`) — **0.5.2와 바이트 동일** | `0x803b30` → `0x952170`(`-0x95b682`) |

- **근거 = 독립 2방법 교차검증**: ①exe 바이트 전수 스캔(0.5.2·0.5.3 **양 버전 각각 유일 후보**) ②Ghidra(`ghidra_beta`) 함수 경계 포함 확인. `_MIGRATE_053.md`의 컨테이너 "유력" 판정은 **실측으로 맞았음이 확인**.
- ⚠**`_MIGRATE_053.md` 표의 "함수내 오프셋"(+906 / +29507)은 오답** — 실측은 **+1140 / +33257**. ⟹ 그 표는 **컨테이너만 유효하고 오프셋은 쓰지 말 것**(0.5.3 = 함수 크기 2~10% 증가로 오프셋 비보존).

#### 16.3 ★★교훈 — mid-func 훅 마이그는 **주소 이동만이 아니라 레지스터 할당까지 대조**해야 한다 (버전무관)
- **레벨업 경로의 GameSetting 베이스 레지스터가 `r14` → `rax`로 바뀌었다**(레벨 홀더도 `r13` → `r12`). 구 스텁은 **`rax`를 스크래치로 사용**했으므로 **RVA만 갈아끼웠으면 GameSetting 포인터가 깨져 크래시**했을 것. ⟹ 스텁을 **`rdx` + `r11`(push/pop) 스크래치로 재작성**해 해결.
- **같은 모드의 UI 경로는 원본 7B가 완전 동일**했다 ⟹ ★**사이트마다 따로 확인해야 한다**(한 사이트 무변경이 다른 사이트 무변경의 근거가 아님).
- ★**재핀 스캐너는 구버전(0.5.2)에 먼저 돌려 "기지의 정답이 유일하게 나오는지"로 방법을 검증한 뒤 신버전에 적용**했다 — 이 절차를 **다른 모드 재핀에도 권장**(0.5.3 LOADER 재핀·앵커맵 검증과 동일 원칙).

#### 16.4 구조 오프셋 = 0.5.3 불변 실측 확정 (재도출 금지)
- GameSetting **`+0xd00`=cap · `+0xd08`=ptr · `+0xd10`=len** — 0.5.2 값 그대로 유효(§7.2-A12 §6-A의 `cap=0` 안전장치·ptr 기준 판정 규칙도 그대로).

#### 16.5 구 dll(0.5.2본)의 0.5.3 거동 = **조용한 스킵 = 기능 사망(크래시 없음)**
- 0.5.2 dll을 0.5.3에 물리면 **훅 2개 모두 `byte mismatch`로 설치 스킵** ⟹ 레벨 상한이 바닐라로 되돌아갈 뿐 **크래시는 발생하지 않았다**. 이 모드의 "프롤로그/원본바이트 불일치 시 미설치=inert" 설계가 **패치 후 크래시를 막아준 실증 사례**(⚠반례 = 재빌드 안 한 `banpick_illust`는 실제 크래시 = §7.3 §12 ⑤).

#### 16.6 모드 작업 문서
- `C:\Users\dev\Desktop\claude\tfm2\mods_report\tfm2_level_cap\` = `01_구조.md` · `02_구현정보.md` · `03_시행착오.md` · `RE\2026-07-31_0.5.3-훅사이트-재핀.md`.

#### 16.7 ✅**0.5.3 릴리스 zip 완료 · 검증 12/12 PASS**(2026-07-31)
- 위치 = `<게임설치>\mods\release\0.5.3\tfm2_level_cap.zip` **97,198B · 5엔트리**(zip 루트 `tfm2_level_cap\` 한 겹).
- 구성 = `tfm2_level_cap.dll`(**198,144B** · sha256[:16] **`e4299b59f0f82dd3`** = 배포본과 바이트 동일) + `mod.mod_info`(v**2.1.0** · deps **`>=0.5.3, <0.5.4`** · BOM無) + `mod.override_info`(**`{}`**) + `tfm2_level_cap.cfg`(기본값 **18 / 2700..4200** 정규화) + `README.txt`(7,930B).
- 생성 = **정석 준수** `python C:\tfm2mods\rel_one.py tfm2_level_cap 0.5.2`(직전 zip 기준·**dll·mod_info만 라이브 반영** ⟹ 유저 튜닝 cfg 유출 방지) + README 추가.
- 검증 12항목 PASS(스크립트 = scratchpad `verify_lvzip.py`): 루트 한 겹 / dll 바이트 동일 / **신 RVA 2건·구 RVA 0건** / mod_info BOM無 / version·deps / author PII 0 / override_info `{}` / cfg 기본값 / 런타임 로그·부산물 0 / **PII 전수(`[A-Za-z]:\Users\dev` + `kikiki8710`) 0건** / 엔트리 5종.
- ★**부수 사실 ①**: **0.5.2 릴리스 zip은 `README.txt`가 빠진 4엔트리였다** → 0.5.3부터 5엔트리로 포함(PII·RVA 노출 0 확인). ⟹ **다른 모드 릴리스 점검 시 "README 누락" 패턴을 함께 볼 것**(⬜다른 모드는 이번 세션 **미확인**).
- ★**부수 사실 ②(규칙 후보)**: README의 "대상 게임 버전"을 `0.5.x` → **`0.5.3`** 으로 갱신하고 **"0.5.4 이상에서는 모드가 자동으로 꺼진다"**(deps 상한 `<0.5.4`) 안내를 추가했다. ⟹ ★**deps 정책은 유저 문서(README)에도 반영**이 규칙으로 일반화할 가치 있음 — ⬜**다른 모드 README 점검은 미확인**(사실 승격 금지).

---

## §7.4 · ★★게임 **0.5.4** 마이그레이션 — 패치 성격 · SDK · **전 모드 재빌드/재핀·배포 22종** (2026-08-05, 0.5.4) — **본 절 = 0.5.4의 정본**(모드별 RE 원문 = `REPORT\<MOD_ID>\RE\2026-08-05_0.5.4-RVA재핀.md`)

> ⚠ 본 절 위의 §7.3(0.5.3)·§7.2 등은 **이력**. 0.5.4 이후 작업은 이 절만 볼 것.
> ~~⚠ **`tfm2_ai_adjust` = 0.5.4 대응 제외(유저 지시)** — 여전히 0.5.3 기준~~ → ★**정정(2026-08-06, 0.5.4): ai_adjust도 0.5.4 마이그·배포·인게임 검증 완료** = RVA 정본 **`MODS\tfm2_ai_adjust\src\rva_054.rs`**(include! 를 `rva_053.rs`에서 전환)·deps `>=0.5.4, <0.5.5`·⚠빌드에 `--extern game_core` 필요 / 검증 `checked 756/756`·전 묶음 1034/1057·crash 0. §7.3 §12.x는 **이력**(RE 사실만 유효). 상세 = `REPORT\tfm2_ai_adjust\` + `MEM\DONE.md`.

### 1. 버전 사실 (실측)

| 항목 | 0.5.3 (직전) | **0.5.4 (현행)** |
|---|---|---|
| exe `TeamfightManager2.exe` | 74,970,624B | **75,936,256B** |
| sha256[:16] | `6afff2cdb6bfa98e` | **`78105410D74836F2`** |
| `.text` 크기 | 51,073,536 | **51,847,168 (+1.51%)** |
| 수령 | 2026-07-29 | **2026-08-05 11:52** |

- 백업 = `C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\`(exe + `TFM2ModUploader.exe` + `bundle.game_data` **1,128,856,877B** + `bundle_ui\` = bundle에서 추출한 `.ui` 288개).
- ★**패치 성격 = 온건**(`.text` **+1.51%**) — 0.5.2→0.5.3의 **+10.5% 전면 재컴파일과 성격이 다르다**. 함수 다수가 위치만 이동하고 코드·프롤로그가 보존되는 편이라 **지문 매칭 성공률이 높았다**(단 §7의 함정 참조 — "온건"이 "그대로"는 아니다).
- **Ghidra MCP 포트 매핑(실측 `list_segments`)**: `ghidra`(8080) = **0.5.3**(.text 0x30A5200) / `ghidra_beta`(8081) = **0.5.4**(.text 0x3172200). ⚠사용 전 `list_segments`로 신원 확인은 계속 규칙([[ghidra-mcp-setup]]).

### 2. SDK · toolchain · 빌드 스크립트

- **SDK = `C:\tfm2mods\sdk_054\mod-sdk`**(GitHub 릴리스 `0.5.4.zip` **544,301,174B** 추출·`base_version.txt` = 0.5.4).
- rlib **154개 = 0.5.3과 개수·파일명 동일**하나 **`mod_api`·`game_core`·`game_view`·`game_ai`·`engine_ui`·`engine_core` 6종 전부 내용 해시 DIFF** ⟹ ★**RVA 0 모드를 포함한 전 모드 재빌드 필요**(0.5.3과 같은 규칙).
- **toolchain = `nightly-2026-05-24` 유지**(`toolchain_version.txt` = `rustc 1.98.0-nightly (23a3312d9 2026-05-23)`) ⟹ **재설치 불요**.
- **링커 = `rust-lld` 필수**(0.5.3과 동일 — MSVC link.exe LNK1107 재조사 금지 = §7.3 §9).
- ✅**빌드 스크립트 4종 sdk_054 전환 완료**: `build_inj.ps1` · `build_full.ps1` · `build_full_remap.ps1` · `tfm2_dashboard_probe\build.ps1`.

### 3. deps 정책 (0.5.4 대역)

- **0.5.4 대역 = `>=0.5.4, <0.5.5`**. 일괄 도구 = 신설 **`C:\tfm2mods\bump_deps_054.ps1`**.
- ★**재빌드가 끝난 모드에만 적용**한다 — 미재빌드 모드는 `<0.5.4` 상한이 남아 **자동 비활성(안전망)** 으로 동작해야 하기 때문.
- **예외 = `legacy_save_patcher`**(stable ABI) = 상한 없는 `>=0.5.3` 유지.
- ⚠★**워크샵 `content\` 폴더의 mod_info 4종은 base 의존에 상한이 없다** ⟹ **자동 비활성이 안 걸린다**: `tfm2_ai_banpick_probe`·`tfm2_meta_champion_tiers`·`tfm2_meta_item_delegate` = `>=0.5.1` / `community_reaction_mod` = `>=0.1.0`. ⟹ 이번엔 워크샵 dll 4종을 **`*.bak_20260805_pre054`로 백업 후 직접 교체**했다. **다음 패치에도 워크샵 폴더는 별도 처리 대상**.
- ⚠**deps 대역 문자열의 띄어쓰기가 모드마다 다르다**(`>=0.5.3, <0.5.4` vs `>=0.5.3,<0.5.4`) — 리터럴 매칭이면 조용히 누락(`tfm2_item_tactics`에서 실제 발생) ⟹ **regex로 매칭할 것**.
- ⚠**`mod_info`의 `description`에도 버전 문구가 박혀 있다**("게임 0.5.3 전용 - 다른 버전에서는 자동으로 비활성화됩니다"). deps만 올리면 **유저에게 보이는 문구가 deps와 모순**된 채 남는다(comptest_unlock·item_tactics 2건 발견·수정) ⟹ 도구 = 신설 **`fix_modinfo_text_054.ps1`**.

### 4. 배포 완료 22종 (전부 배포본 크기·시각 확인 = CLAUDE.md §10 증거 규칙 충족) — ⬜**22종 전부 인게임 미검증**

| 모드 | 배포 dll |
|---|---|
| community_reaction_mod | 619,008B |
| Spectator_Chat | 332,800B |
| tfm2_mod_order | 203,776B |
| tfm2_ai_banpick_probe | 251,904B |
| tfm2_meta_champion_tiers | 229,888B |
| tfm2_meta_item_delegate | 254,464B |
| coaching_staff_view_plus | 284,160B |
| custom_tier_assignment | 2,684,928B |
| facility_view_plus | 287,744B |
| finance_view_plus | 147,456B |
| recruitment_view_plus | 333,312B |
| roster_view_plus | 433,664B |
| statistics_view_plus | 2,713,088B |
| training_view_plus | 2,608,640B |
| legacy_save_patcher | 363,008B |
| tfm2_comptest_unlock | 194,048B |
| tfm2_draft_overlay | 684,032B |
| tfm2_level_cap | 198,144B |
| tfm2_banpick_illust | 2,891,264B |
| tfm2_item_tactics | 569,856B |
| tfm2_banpick_order | 2,595,328B |
| TFM2_Meta_Dashboard | 335,872B (+ `tools\tfm2_save_probe.exe` 7,324,160B) |

- ~~⬜**미완 = `tfm2_elemental_serpen` 0.5.4 재핀(진행 중·별도 세션)** — 배포·릴리스 안 됨.~~ → ✅**정정(2026-08-08): 재핀 17/17·v0.4.3 배포(420,864B @08-05 13:10:55)·릴리스 zip·deps 0.5.4 완료 = §5.9**(08-08 현행 = v0.4.4). / ~~⛔`tfm2_ai_adjust` = 대응 제외(유저 지시)~~ → ✅**08-06 마이그·배포·인게임 검증 완료(`rva_054.rs`)**.

### 5. 모드별 RVA 재핀 결과 (0.5.3 → 0.5.4)

#### 5.1 공통 재확인 — 3개 모드가 **독립적으로 같은 답**
- `LOADER` **`0x2e1550` → `0x2e35d0`** · `PARSER` **`0x1a6530` → `0x1a3ce0`** · `ALLOC` **`0x28f7df0` → `0x29bb920`**.

#### 5.2 `tfm2_comptest_unlock` — 바이트패치 16 + 훅 4 재핀 (16/16 orig 실측 MATCH)
- ★★**orig 바이트가 6건 바뀌었다** — **주소만 갈았으면 byte mismatch로 조용히 skip = 기능 사망**이었을 것(0.5.4가 "온건"이어도 원본 바이트는 확인해야 한다는 실증).
- `server_roster_min`은 **명령 자체가 교체**(`lea rax,[rsi+rsi]` 4B → `add rbx,rbx`) ⟹ **nop3로 전략 재설계**.
- **실효 3사이트** = `server_pregate` **`0x20e5471`** · `daily_inc_gate` **`0x20e8246`** · `dr_inline_b` **`0x2310c86`**.
- ★**0.5.4 sim 랜드마크 재핀(2026-08-08 ghidra_beta+capstone·HIGH)**: run_tick 오라클 **`0x13b3150`**(구 0.5.1 `0x20566c0`) · 완주 폴러(feedback.rs) **`0x148a7c0`**(구 `0x206dc10`) · ~~sim 실행 본체~~ **`0x237c030`** = ★**부팅 시 배치 presim 8건 본체(정정 08-08 심야 인게임 프로브 실측·HIGH — 부팅+7초 8스레드 동시·comp_test 무관·내부 = 10회 몬테카를로 롤아웃+본경기 완주)** · 서버 메인루프(server.rs) **`0x20237c0`** · 패킷 디스패처 **`0x20d5bf0`**(구 `0xf1d2c0`) · 워커 결과처리기 **`0x2392ed0`**(MED). 부수 판정 = **comp_test N개 동시 실행 = 이미 지원**(~~매치당 detached 스레드+완주 폴러~~ → **실측 정정: comp_test는 0x237c030·폴러 0x148a7c0 미경유 — 요청당 전용 스레드 인라인**(워커 spawn `0x23b16d0`→body `0x2118ea0` 추정) = 요청 N개=스레드 N개 확증). 상세 = `REPORT\tfm2_comptest_unlock\RE\2026-08-08_동시실행_구조규명.md`(**실측 = 부록 A**).
- ⚠**소스 정정(2026-08-08)**: `tfm2_comptest_unlock.rs` L1265 `ORACLE_RVA = 0xeb6590`은 **0.5.4 오답**(`SIM_PROBE_ON=false`라 미설치 상태여서 무해했음) → **`0x13b3150`**으로 정정 완료. **비활성 프로브 상수라 재빌드·재배포 불요**(배포 dll은 그대로 유효).
- ★**(15) RUN 멀티발사 v1 구현·배포(2026-08-08 02:20·dll 212,992B)** — RUN 1클릭 = **5경기**(원클릭 1 + post_update 재발사 4·2프레임 간격 = 시드 성분 epoch_ms 자동 분리). ~~⬜인게임 미검증(유저 테스트 대기)~~ → ❌**실패 확정·⛔재시도 금지(0.5.4·2026-08-08 인게임: 기록 탭 경기 1건)** — post_update 시점엔 제출 큐가 무효(제출 계약 = 아래 (16)). 근거 = 위 RE 파일 **부록 A**(훅 안전 실증 = 0x13b3150 진입 트램폴린이 동시 멀티스레드 하 크래시 0 포함).
- ★★**(16) comp_test RUN 제출·시드 계약 확정(2026-08-08 ghidra-re·0.5.4 실측·HIGH)** — 정본 = `REPORT\tfm2_comptest_unlock\RE\2026-08-08_RUN재제출_차단원인.md`(여기는 요지+포인터). ①**제출 = 소켓이 아니라 RUN 핸들러 `0x231de30`의 param_2(RDX) = 호출자 소유 커맨드 Vec에 push**(Vec{cap@0x0,ptr@0x8,len@0x10}·stride **0x2120**·커맨드 `+0x0 tag=0x16`/`+0x10 패킷ptr`·패킷 `+0x0 disc=0x1c`·할당 0x740B·push 지점 `0x231eab0`·핸들러 콜러 `0x1cc7f40`·큐 = evt_ctx+0xc0) ⟹ **제출 성사 판정 = detour에서 `*(rdx+0x10)` 델타(새 훅 불요)**. ②**요청 시드** = `(used_today | (game[0xe3b8]<<32)) ^ game_time_ms`, **패킷 `+0x68`(u64)**(`+0x70` lane/모드 int·`+0x74` 5v5 byte) ⟹ **같은 프레임 N회 = 시드 전부 동일 → 동시 N경기엔 시드 변조 필수**. ③⛔**핸들러 `ret=1`로 성공 판별 불가**(출구 `0x231e265 mov al,1` 단 하나 = 전 경로 합류) — 첫 관문 = 클라 사전게이트 `0x2310a90`(1통과/2 downcast실패/0 = 일일한도·인원부족·중복·챔피언미지정). ④**서버 등록루프 함수시작 `0x2126b00`**(= 기존 패치 `server_roster_min 0x2126ed0`·`server_dedup_real 0x2126f73`의 소속 함수·프롤로그 push8 12B=훅 가능·AL 0xff성공/4챔피언/3요청내중복/2로스터부족/0조회실패·유일 콜러 = 디스패처 `0x20d5bf0` 내 `0x20de75d`) — ★**그 dedup은 요청 1건 내부 중복만 검사 = 세션 간 재제출 차단 아님**(memoize 가설 기각). ⑤성공 파이프라인 = 일일게이트 `0x20e5428` → 경기형성 호출 `0x20e81c0`(본체 **`0x2123590`**, 구 0.5.1 `0xf63f80`) → inc_gate `0x20e8247` → 응답 `0x56`. ⑥**RUN 멀티발사 v2 배포(2026-08-08 02:43·dll 219,136B)** = detour 본문 **동기 N회 루프** + 회차별 패킷 시드 XOR + len 델타 계측 + 보조 진단훅 2종(`0x2310a90`·`0x2126b00`) · ⬜**인게임 미검증** · ⚠**프로브 빌드(`LOG_ENABLED`·`CONC_PROBE_ON`·`CONC_ON`=true) 릴리스 전 복귀 필수**.
- ★★**(17) 결과 큐잉 v6 = 크래시로 롤백 · 현행 배포 = `QUEUE_ON=false`(2026-08-08 심야·0.5.4)** — 정본 = `REPORT\tfm2_comptest_unlock\03_시행착오.md` 08-08 "결과 큐잉 v6" + `…\RE\2026-08-08_결과기록_훅지점_확정.md`(여기는 요지+포인터). ①**결과 도착 함수 = `0x2327080`**(.pdata `[0x2327080,0x2327970)` · **Ghidra 미인식** — `FUN_142327010`은 cleanup funclet) · 기록·송신 = **`0x230c910`**(계약 `fn(a1, node, &cmdVec)`) · 러너 획득 = `*(node+0x230)` · 타입검증 = `*(node+0x238)==exe_base+0x33b91f8` · 기록 게이트 = **`+0x21a0`과 `+0x2280`의 쌍**(후자는 팝업 draw `0x23cd370`이 다음 프레임에 생성) · 상태머신 `+0x240c`(0~5). ②⛔**`0x2327080` 진입점 15B 훅 + Rust detour(로깅·Mutex·`format!`) = STATUS_STACK_OVERFLOW(`0xc00000fd`) 크래시 — 방식 한정 재시도 금지**(ai_adjust VEH crash_log 03:30: RIP `exe+0x303e337`=__chkstk 프로브 write, stack `exe+0x2327094`→`exe+0xa16162`). 원인 = 이 함수가 `mov eax,0x42c0; call __chkstk`로 **17KB 프레임**을 잡는데 진입점 훅은 프로브 **이전**에 detour 스택을 얹음. ⬜미시도 대안 = naked shim(원본 프롤로그 선행) / 호출자 `0xa15e20`로 훅 이동 / detour 경량화. ③**롤백·현행 배포 = `QUEUE_ON=false` 재빌드·배포 dll **225,280B @2026-08-08 03:32:43**(구 v6 = 231,936B @03:28)** ⟹ **동시 5경기 실행(v2 계열)은 정상 작동**, 결과 큐잉만 비활성. ⬜**잔여 = 경기 5완주인데 기록 1건**(`runner+0x21a0` 단일 Option 덮어쓰기 = 원인 확정·해법 미구현). ⚠프로브 빌드 플래그 릴리스 전 복귀 필요는 (16) 그대로 유효. → ★**(18)에서 잔여(기록 1건) 해소·기능 완성**.
- ★★**(18) 조합테스트 다회차 실행 = 완성·인게임 검증완(2026-08-08 새벽·0.5.4)** — 정본 = `REPORT\tfm2_comptest_unlock\04_동시실행_설계.md` + `…\RE\2026-08-08_*.md` **7건**(여기는 요지+RVA만). ①✅**순차 N경기 자동 실행**: RUN 1클릭 → N경기 순차 실행·**히스토리에 N건 전부 저장**·경기별 시드 상이(계측 `forge N / result N / csend N / hpush N`) · N = `comptest_items.cfg` **`runs`(1~10·기본 5)** ⟹ (17)의 ⬜"5완주인데 기록 1건"은 **해소**. ②✅**다회차 결과 화면 = 신규 UI 구현 0**: 기록탭 목록 빌더 **`0x2311c20`**의 **take 상수 imm32 `0x2311d43`(기본 `0x14`)를 N으로 poke** ⟹ 이번 회차 N건만 표시(카드·스크롤·정보/리플레이 버튼 전부 게임 것 그대로) · **자동 진입** = 기록완료 후 이동 페이지 imm8 **`0x230d0ec`** `5`(summary)→`1`(history) · 뒤로가기 = 준비탭 복귀 시 원복(30프레임 디바운스) · 목록 refresh = **`0x2306000`**. ③★**comp_test = 진짜 틱 시뮬레이션(정정 확정)**: 경기마다 전용 스레드에서 **`0x2325840`**이 `while(!is_over){ run_tick }` 완주 — 같은 날 오전 판정 "클라가 `0xa15e20`에서 동기 생성"은 **오독**(`0x235bf20` = 리플레이용 Game 셋업·run_tick/PRNG 호출 0회). ④★**실시간 관측 배선(검증완)**: run_tick **`0x13b3150`** 인자 rdx → **`game = *(rdx+0x1dc0)`** · 킬스코어 **`game+0xeb38`(blue)/`+0xeb40`(red)** · 진행 틱 **`+0xeb30`** · 시드 **`+0xeb28`**. 인게임 실측(5경기) = 28:14/20:3/29:12/37:0/9:12 · **경기당 29,638~42,713틱 · 5.1~7.7초** · 킬 간격 30~600ms ⟹ 실시간 표시 실효성 충분. ⑤★★**경기 식별키 = 시드**(게임이 배경에서 리그 sim을 상시 실행 ⟹ "RUN 직후 첫 sim = 내 경기"는 오판 = 남의 경기 수십 건 + 종료 game stale read) — 발사 시드(원본 1 + 변조 N−1) 등록 후 `game+0xeb28` 대조로 **오염 0·정확히 5경기만 포착**·병렬 N 동시추적에도 적용. ⚠완주 판정을 "스코어 변화 없음"으로 하면 **초반 킬 공백을 종료로 오판**(5건 중 4건 `0:0`) ⟹ 신호 = **틱 카운터 변화**. ⑥**현행 배포 dll = 238,592B @2026-08-08 05:07:56**. ⑦⬜**잔여 3** = ⓐ결과화면 진입 시 **마지막 1건 누락**(페이지 전환·목록 생성이 마지막 경기의 서버 저장보다 먼저 — 다른 탭 갔다 오면 5건) ⓑ**"테스트 결과/모든 결과" 토글 버튼** 미구현(ⓐⓑ **선결과제 동일 = 모드가 목록 refresh `0x2306000`을 유발하는 방법**·⬜RE 중) ⓒ**병렬 발사 + 전량 기록 = 경로 확정·미구현**(payload 0x268 소유권이 `0xA15E20`에 완전 이전 ⟹ 얕은 복사 = move / 지연 훅 **`0xA15E48`(14B)** + 드라이버 훅 **`0xA289C0`(22B·프레임당 1건 재주입)** = `RE\2026-08-08_병렬전량기록_경로A_확정.md`). ⚠프로브 빌드 플래그 릴리스 전 복귀 필요는 (16) 그대로 유효.

#### 5.3 `tfm2_draft_overlay` — 4종
- `LOADER` **`0x2e35d0`** · `PARSER` **`0x1a3ce0`** · `ALLOC` **`0x29bb920`** · `ANIM_GET` **`0x74c010`**. `BANPICK_LOADER = 0` 유지(0.5.3 판정 계승 = 자기 체인 무한재귀 방지).

#### 5.4 `tfm2_level_cap` — 2종
- `RVA_LEN_LOAD` **`0x14ece54`** · `RVA_UI_CMP` **`0xa99c29`**. ★**레지스터 할당 무변화 ⟹ 스텁 무수정**(0.5.3 때와 반대 사례 — 그래도 **사이트마다 확인**은 유지).
- GameSetting **`+0xd00`cap / `+0xd08`ptr / `+0xd10`len = 0.5.4 불변 확정**(재도출 금지).

#### 5.5 `tfm2_meta_item_delegate` — 재핀할 RVA 없음
- **하드코딩 RVA 0건 확정** ⟹ SDK 재빌드만.

#### 5.6 `tfm2_banpick_illust` — 29종 전건 재핀·미해결 0
- **ORIG_LEN 3건 전부 불변**(12 / 12 / 13). geom `.rdata` 블록 델타 **+0xe39b0**. `RVA_ASSET_GET` **`0x143d50` 불변**.
- ★교훈 = **skel(프롤로그) 불일치가 곧 "다른 함수"는 아니다** — `RVA_IMG_COLOR`는 레지스터 할당만 바뀌고 **size·콜러 다중집합은 완전 동일**이었다.

#### 5.7 `tfm2_item_tactics` — RVA 15 + 바이트패치 5 + 구조체
- ★★**athlete 구조체 −0x10 시프트**: stride **`0x8d0` → `0x8c0`** · athlete_id `0x810`→**`0x800`** · team `0x820`→**`0x810`** · position `0x8b0`→**`0x8a0`** · gold `0x888`→**`0x878`** · champ/owned/build Vec 전부 **−0x10**. **`≤0x90` 필드는 불변**.
- **`O_PROVIDER_SEED` `0xeaf8` → `0xeb28`**.
- ★**`CL_LAUNCHER_PROLOGUE[13]` `0x08` → `0x68`** — 미수정 시 **launcher 훅이 영영 미설치**(조용한 기능 사망).
- **0.5.4 불변 확정** = Game `+0x1dc0`/`+0x1dc8`/`+0x30` · roster `+0x840`/`+0x848` · PV stride `0x260` · 아이템 vtable `0x50`~`0x70` · `NT_SIZE 0x90`.
- ⚠**자동 전파 매칭 단독 신뢰 금지** — 조합테스트/`solo_rank_ui` 두 사이트를 **서로 뒤바꿔** 내놨고 **panic-location 지문**으로 교정했다.

#### 5.8 `tfm2_banpick_order` — 함수시작 16 + mid-func 18사이트
- 슬롯 이동 = SFX 씬슬롯 **`0x12d0`→`0x12f0`** · J/L 스텁 **`0x1108`→`0x1118`**·**`0x1308`→`0x1328`** · 드레인 프레임 **`0x1448`→`0x1468`**.
- 레지스터 변화 = I HL 출력 **`sil`→`bl`** · AI파리티 site1 out **`cl`→`dl`** · site2 out **`al`→`cl`**.
- ★신규 자체검증 도구 **`bo_verify54.py`**(소스 파싱 → exe 실바이트 대조·**FAIL 0**) — 다른 모드에도 재사용 가치.

#### 5.9 `tfm2_elemental_serpen` — 재핀 17/17 완료(2026-08-05 별도 세션) + v0.4.4 증분(2026-08-08)
- **재핀 정본 = `REPORT\tfm2_elemental_serpen\RE\2026-08-05_0.5.4-RVA재핀.md`(17/17)**. v0.4.3 = 빌드·배포(dll 420,864B @08-05 13:10:55)·릴리스 zip(1,160,409B·28엔트리)·deps `>=0.5.4, <0.5.5`. 08-08 06:39 기동 probe = **훅 9종 설치 OK·`seh_faults=0`**.
- ★**v0.4.4(2026-08-08) = comp_test(조합테스트) 다시보기 속성 미표시 수정**: 원인 = 렌더 게이트(런처 훅 retaddr 화이트리스트 A/B/C)가 comp_test 다시보기 경로 미포함 — comp_test는 정규 리플레이 핸들러 `0x1d13e60`이 아닌 **전용 재생 빌더 `0x2323aa0`**(training_ui.rs·CompTestHistoryEntry seed 재시뮬·**콜러 = comp_test 팝업 핸들러 `0x2326820`**)를 탄다(sim측 속성부여는 seed 무관 전 경기 적용 = 표시 계층만 사망). 수정 = ghidra-re 런처(`0x13b53d0`) 콜사이트 exe 바이트스캔 전수 9건 → comp_test 화면 재생 유일 콜사이트 **`0x2323ff9`** → **`LAUNCHER_RET_D = 0x2323ffe`**(0.5.4) 화이트리스트 추가. 콜사이트 전수표 = `REPORT\tfm2_elemental_serpen\RE\2026-08-08_comptest-다시보기-런처콜사이트.md`.
- ⛔**콜사이트 `0x235c382`(comp_test 백그라운드 sim 본체 추정)는 화이트리스트 추가 금지** — 헤드리스 sim이 화면 경기로 오인됨.
- 배포 = **v0.4.4 dll 420,864B @2026-08-08 06:49:24**·mod_info 0.4.4·커밋 `a8b93bb`·⚠릴리스 zip은 인게임 검증 후 보류(현 zip = v0.4.3 = 낡음). ⬜잔여 = comp_test 다시보기 1판 인게임 확인(`02_구현정보.md` 검증표 #11).

### 6. 부수 검증

- ✅**세이브 포맷 무변경(0.5.3 = 0.5.4)** — sdk_054로 재빌드한 `tfm2_save_probe.exe`로 실 세이브 **full-load** → `save_probe ok: teams=120 athletes=1144 match_replays=1228`·**salvage 아님** = **5연속 무변경**.
- ✅**daram2 `.ui` base 재대조 = 9종 이상무·조치 0건**: 0.5.4 bundle에서 `.ui` **288개** 추출(`tfm2_0.5.4\bundle_ui\`), 0.5.3↔0.5.4 **전수 대조 = 추가·삭제 0 / 변경 5개뿐**(`player_detail/layout.ui`·`team_data.ui`·`team_detail.ui` = `#summary`/`#toolbar` **91줄 순수 추가** / `ingame_component/player_info.ui`·`wide_player_info.ui` = 폰트 size 18). **이 5파일을 override 하는 모드 0건** ⟹ 0.5.3 `custom_tier_assignment` 식 사고 없음.
  - ⚠**"8종 재대조 금지"는 0.5.3 한정 판정이었다** — 이번에 9종 전수 재대조했고, **다음 패치에도 재대조는 필요**하다(범위 한정 판정의 실사례).
- ✅**릴리스 zip 13개** = `<게임설치>\mods\release\0.5.4\` — Spectator_Chat 162,191B(6) / tfm2_mod_order 103,166B(3) / tfm2_comptest_unlock 94,781B(3) / tfm2_draft_overlay 146,797B(3) / tfm2_level_cap 97,173B(5) / tfm2_item_tactics 258,229B(5) / tfm2_item_tactics_source 160,686B(11) / tfm2_banpick_illust 936,542B(6) / tfm2_banpick_order 832,303B(4) / community_reaction_mod 322,725B(3) / daram2_viewplus 9,414,162B(47) / 팀파매gg모드3종 617,409B(17·루트 `팀파매gg_0.5.4`) / TFM2_Meta_Dashboard 157,189,040B(944). **PII 실질 0건**.

### 7. ★교훈 (대부분 **버전무관** — 다음 패치에 그대로 적용)

1. ★**"재빌드 불요 모드"도 PII 검사 대상이다** — `legacy_save_patcher`(stable ABI·**일반 cargo 빌드**)의 dll에 `C:\Users\dev\.cargo\registry\...` PII가 박혀 있었고 **0.5.3 릴리스 zip에도 그대로 나갔다**. 원인 = cargo 빌드는 `build_full_remap.ps1`의 `--remap-path-prefix`를 **안 탄다**. 조치 = `$env:RUSTFLAGS="--remap-path-prefix=..."`로 재빌드(363,008B·PII 0).
2. ★**`pii_check.py`는 오탐·누락 양쪽이 있다** — 로컬 경로만 봐서 개인 핸들을 놓치고, 반대로 `@gmail.com`·`AppData\Roaming\TeamSamoyed`를 넓게 잡으면 **Chromium/Python 번들의 upstream 저자 이메일**과 **`$env:USERPROFILE` 변수 조립 코드**까지 **28건 오탐**한다. 신설 **`pii_check_054.py`** = 이 유저 것만 잡고 벤더 런타임 경로 제외.
3. ★**한글 리터럴을 가진 `.ps1`은 UTF-8 BOM 필수** — `rel_commit.ps1`이 BOM 없는 .ps1이라 **PowerShell 5.1이 ANSI로 읽어** 스크립트의 한글이 깨진 채 커밋 메시지에 들어가고 있었다(기존 커밋 `5326fe4` 등에서 확인). 조치 = BOM 부여. ⚠**게임이 읽는 json/mod_info는 여전히 BOM 금지**(정반대이니 혼동 말 것).
4. ★**PowerShell 변수명은 대소문자를 구분하지 않는다** — `$new`(치환 결과)가 `$NEW`(치환 대상 문자열)를 덮어써 **두 번째 모드의 `mod.mod_info`에 첫 모드의 json 전체가 끼어 들어갔다**(`tfm2_mod_order` 실제 손상 → 릴리스 zip에서 회수해 복구). 조치 = 스크립트에 **json 파싱·크기 델타 사후검증** 삽입.
5. ★**cfg는 릴리스 zip에서 라이브를 따라가면 안 된다**(기지 규칙·이번에 밟을 뻔) — `4items.cfg` 릴리스 기본값 = 영문 주석 + `slots = 3` / 라이브 = 한글 주석 + `slots = 4`(유저 테스트값).
6. **deps 대역 문자열 띄어쓰기 차이 → regex 매칭**(§3).
7. **`mod_info.description`의 버전 문구도 함께 갱신**(§3).
8. ★**소스 경로를 dll에 전혀 안 박는 모드가 있다** — `tfm2_meta_champion_tiers`는 **신규 빌드도 배포중 dll도 `.rs` 경로 0건**. rustc는 **panic location으로만** 경로를 박으므로 **도달 가능한 panic 사이트가 없으면 안 박힌다** ⟹ `build_inj.ps1`의 **신원 가드가 오탐**한다. 대체 마커 = **`build_extra.ps1 -IdentityString`**.
9. **skel 불일치 ≠ 다른 함수**(§5.6) — size·콜러 다중집합으로 판정.
10. **자동 전파 매칭 단독 신뢰 금지**(§5.7) — panic-location 지문 등 **독립 2방법 교차검증**.

### 8. 커밋 (`C:\tfm2mods`, master)
`9374552` level_cap · `7a7320a` comptest_unlock · `e1030f8` draft_overlay · `0cd3a6f` item_tactics · `d96d7a9` banpick_illust · `dffd1d8` banpick_order · `bee1f4a` 0.5.4 인프라(빌드 스크립트 4종 + 신규 도구 + RVA 재핀 도구 33종).

### 9. ⬜ 잔여 (사실 승격 금지)
- ~~⬜**`tfm2_elemental_serpen` 0.5.4 재핀 진행 중**(별도 에이전트) — 미배포·미릴리스.~~ → ✅**정정(2026-08-08): 재핀 17/17·v0.4.3 배포·릴리스 완료, 08-08 v0.4.4 갱신 배포(comp_test 다시보기 수정) = §5.9**. ⬜잔여 = comp_test 다시보기 1판 인게임 확인.
- ~~⛔**`tfm2_ai_adjust` = 0.5.4 대응 제외(유저 지시)** = 0.5.3 상태 유지.~~ → ✅**정정(2026-08-06): 0.5.4 마이그·배포·인게임 검증 완료**(`src\rva_054.rs`·deps `>=0.5.4, <0.5.5`·`--extern game_core` 필요·checked 756/756·1034/1057·crash 0·릴리스 zip 28엔트리). 0.5.4 신규 노브 3묶음(`auc_flee_*` 12·`an_*` 6·`path_*` 208사이트) 전부 적용 검증완.
- ⬜**22종 전부 인게임 미검증**.
- ⬜`tfm2_item_tactics`의 **SDK(Database) 상대 오프셋·Node 오프셋**은 exe RVA가 아니라 **rlib 레이아웃 의존**이라 정적 확인 불가(어긋나도 크래시는 아님).
- ⬜`tfm2_banpick_order`: **AI파리티 emit의 al/cl 복원 소스는 추론** · AI6 6사이트 cfg 기본 OFF 유지 · **phase 인라인 복제본 ~20개 미보정**.
- ⬜`tfm2_banpick_illust`: `raw.rs`의 SDK 구조체 오프셋은 rlib `offset_of!` 실측치라 **exe-diff로 검증 불가** — `ui_offset_probe\`를 **sdk_054로 재빌드해 확인 필요**.
- ⬜**워크샵 스팀 업로드(게시)** = 작성자 수동 = 미실시.

---

## §7.5 · ★★게임 **0.5.5** 마이그레이션 — 패치 성격 · exe↔exe 재핀 1차 (2026-08-12, 0.5.5) — **본 절 = 0.5.5의 정본**

> ⚠ 본 절 위 §7.4(0.5.4)·이하는 **이력**. 0.5.5 이후 작업은 이 절만 볼 것.
> ✅ **`tfm2_ai_adjust` = 0.5.5 완결(08-13)** — ~~별도 진행(본 절 범위 밖)~~ 정정: 마이그·배포·인게임 검증·릴리스·커밋 `4436581` 전부 완료(§5 행·§6 ai_adjust 확정분·nexus RVA 6종).

### 1. 버전 사실 (실측)

| 항목 | 0.5.4 (직전) | **0.5.5 (현행)** |
|---|---|---|
| exe `TeamfightManager2.exe` | 75,936,256B | **76,957,696B (+1,021,440B)** |
| sha256[:16] | `78105410D74836F2` | **`09E12009BB240EED`** |
| `.text` vsz | 51,847,263 | **52,745,711 (+1.73%)** |
| `.rdata` | 21,639,594 | **21,746,126 (+0.49%)** |
| `.pdata` 고유 함수 | 134,556 | **135,994 (+1,438)** |
| 인덱스 pkl | `_fnidx_054.pkl` | **`_fnidx_055.pkl`**(신규 빌드) |

- 백업 = `C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.5\TeamfightManager2.exe`(라이브와 동일 크기 확인).
- ★**패치 성격 = "온건한 크기, 넓은 로직 변경"**. `.text` +1.73%로 크기는 0.5.4급이나, **훅 대상 핵심 함수·컨테이너 다수가 본문 변경**(simulation/damage/AI/comptest/banpick 계열) ⟹ **순수 핫픽스(RVA-only)가 아니다**. 함수시작 재링크는 대부분 확정되나, **mid-func 바이트패치 컨테이너는 거의 전부 본문 변경 = 명령정렬 재핀 필요** ⟹ 0.5.4처럼 **모드별 정밀 재핀 세션**이 필요.
- ~~★**구조체는 대체로 불변 시사**: `item_tactics:BUY_ITEM`(athlete 오프셋 `[r8+0x490]` 등 사용)이 skel UNIQUE + `SEEDCTOR`(provider seed `[rsi+0xeb28]` 사용)가 마스크시그 UNIQUE로 잡힘 ⟹ **athlete −0x10 시프트 재발 없음·`O_PROVIDER_SEED 0xeb28` 불변 시사**(단 실측 재확인 권장 — 2패치 연속 이동 이력).~~ → ❌**오판(정정 2026-08-12 — serpen·item_tactics 세션 명령 실측)**: 0.5.5는 **구조체 4대역 동시 시프트**다(아래 §6 정본). skel UNIQUE는 "함수 신원"만 보증하지 구조체 오프셋 불변의 근거가 아니다(BUY_ITEM의 `[r8+0x4f0]`은 저대역 +0x60 이동분이었고 skel엔 안 잡혔음). ★교훈 = **구조체 판정은 반드시 대역별 명령 센서스로**.
- 재핀 도구(0.5.4 커밋 `bee1f4a` 재사용) + 신규 = `_mig055.py`(skel 매칭 엔진)·`_mig055b.py`(마스크시그+문자열xref+콜러수)·`_mig055c.py`(콜그래프 앵커전파 33,461 시드). `fnindex.py`로 `_fnidx_055.pkl` 빌드.

### 2. 함수시작 재핀 결과 (66 대상 중 UNIQUE 확정)

**공통 3심볼 + ANIM (전 모드 동일):**
- `LOADER` **`0x2e35d0` → `0x2e42d0`**(문자열-xref: layout/main×17·banpick×20, 0.5.4 재현 OK — clone family라 xref만 유효) · `PARSER` **`0x1a3ce0` → `0x1a3e70`**(skel 유일) · `ALLOC` **`0x29bb920` → `0x2a9bf30`**(skel/마스크 유일) · `ANIM_GET` **`0x74c010` → `0x844160`**(champions/priest#anim lea→call 1표 + CARD_DRAW 콜슬롯13 이중확증).

| 모드 | 확정 재링크(0.5.4→0.5.5) | 미확정(본문변경→재핀세션) |
|---|---|---|
| **draft_overlay** | LOADER `0x2e42d0`·PARSER `0x1a3e70`·ALLOC `0x2a9bf30`·ANIM_GET `0x844160`·BANPICK_LOADER=0 유지 = **전건 확정·소스 갱신완** | 없음 |
| **banpick_illust** | 함수 16 전건: FX_SET `0x1951330`·CARD_DRAW `0x1966dd0`·ILLUST_GET `0x1edf1f0`·SUBMIT `0x181590`·SUBMIT_TEXT `0x181810`·IMG_BUILD `0x182cb0`·IMG_UV `0x182b10`·IMG_FLAG `0x182fc0`·IMG_COLOR `0x1e6d3d0`(콜슬롯8/19 정렬)·IMG_SHADER `0x1845c0`·TEXT_BUILD `0x1821a0`·NAME_GET `0x19a11c0`·ASSET_GET `0x143cb0`(콜슬롯1)·ANIM_GET `0x844160`·SPRITE_CALC `0x19a60f0`·GAME_ALLOC `0x2a9bf30` | geom `.rdata` 6·mid-func 6·RVA_SLOTS(0-run) = **미처리**(재핀세션) |
| **banpick_order** | 함수시작 16 전건: APPLIER `0x1951100`·SLOTUPD `0x19ad1d0`·TRANSITION `0x19474f0`·BANNER `0x194e880`·LINEUP `0x193de10`·COMMIT `0x12156b0`·TRIGGER `0x196fdb0`·PANIC_HOOK `0x2a97074`·PHASE_SCENE `0x196c2c0`·PHASE_SCALAR `0x1210c40`(⚠프롤로그 lea disp 변경)·PHASE_RAW `0x1946d50`·TURN `0x1215de0`·APP_PICK_T1 `0x193d350`·APP_PICK_T2 `0x193d4e0`·APP_BAN_T1 `0x197ae90`·APP_BAN_T2 `0x197b010` | **mid-func 18 전부**(드레인·AI턴·match_ui·AI스코어러·AI6 컨테이너 본문변경·명령정렬 필요) |
| **item_tactics** | FN_DD_SETOPT `0x1c1a30`·GV_UPDATE `0x964350`·REALLOC `0x2a87a70`·BUY_ITEM `0xeb2c40`·ITEMNET_FORWARD `0x12624f0`·SEEDCTOR `0x14c2380`(마스크시그)·GAME_ALLOC `0x2a9bf30`·LOADER `0x2e42d0`·PARSER `0x1a3e70` | RVA_TIP_SHOW(유력 `0x2587990`)·CL_LAUNCHER(유력 `0x14ac3e0`·프롤로그변경)·SPAWN(gate off)·바이트패치 owned_cap/gate3(컨테이너 본문변경)·retaddr 필터·RVA_TIP_MEASURE_VT(.rdata) |
| **elemental_serpen** | SPAWN0 `0xb34440`·SPAWN1 `0xb33820`·RENDER_STEP `0x964350`·KEYRES `0x1be3ad0`·UILOADER `0x2e42d0`·UIPARSER `0x1a3e70`·UIALLOC `0x2a9bf30` | SERPEN(clone함정·kind게이트 확인 필수)·MOBATICK(유력 `0x14f7e40`·**본문 +23%**)·LAUNCHER(유력 `0x14ac3e0`)·RUNNER_CTOR(유력 `0x14ae060`)·DMGA·DMGB·ARG_STR(MULTI2)·retaddr 3종·구조체 재확인 |
| **comptest_unlock** | LOADER `0x2e42d0`·PARSER `0x1a3e70`·ALLOC `0x2a9bf30`·(reg_loop 컨테이너 `0x2126b00→0x21bee60`: server_dedup_real `0x2126f73→0x21bf2d3` orig OK·server_roster_min `0x2126ed0→0x21bf230` orig OK) | RUN_RVA(유력 `0x1aa2930`)·나머지 바이트패치 14(서버핸들러 `0x216e870`·팝업 `0x1a8aa10`·setup게이트 `0x1a95570`·버튼빌더 `0x1afcc20` 컨테이너로 좁혀지나 본문변경·정렬 필요)·CT_REGION/CLIENT 범위 |
| **level_cap** | 없음(둘 다 mid-func) | RVA_LEN_LOAD·RVA_UI_CMP 둘 다(컨테이너 `0x14ec9e0`·`0xa91a30`→유력 `0x955680`(ui_cmp) 본문변경·정렬 필요) |
| Spectator_Chat·community_reaction_mod | exe RVA 하드코딩 없음 = 재링크 대상 아님 | ClientDatabase 오프셋(LIVE_PLAYED_OFF 5528·LIVE_EVENTS_OFF 5744)은 db 레이아웃 의존 = serpen db 센서스로 재확인 |
| mod_order·meta_item_delegate·save_probe(TFM2_Meta_Dashboard)·daram2 9종 | 하드코딩 RVA 0 = **SDK 재빌드만** | — |

### 3. 소스 갱신 (이 세션)
- ✅**`tfm2_draft_overlay\src\lib.rs`** = LOADER `0x2e42d0`·PARSER `0x1a3e70`·ALLOC `0x2a9bf30`·ANIM_GET `0x844160` **4상수 갱신**(정정형 주석). BANPICK_LOADER=0 유지. **전건 확정·빌드 대기**.
- ⚠**나머지 모드는 소스 미갱신** — 함수시작 재링크는 다수 확정이나 **mid-func 바이트패치·본문변경 함수가 미확정**이라, 부분 갱신 시 stale 상수와 혼재해 위험(0.5.4는 모드당 전건 확정 후 일괄 갱신 원칙). ⟹ **모드별 재핀 세션에서 위 확정값을 시드로 받아 미확정분(정렬·문자열·panic-location 앵커)까지 완결 후 일괄 갱신**할 것.

### 4. ★재핀 세션이 이어받을 것 (모드별)
- **본문 변경으로 명령정렬 재핀 필요**: banpick_order mid-func 18(bo_054.py 재사용) · comptest 바이트패치 14(ct_054.py) · level_cap 2(pe054.py) · item_tactics owned_cap/gate3 · banpick_illust geom/mid-func.
- **clone/문자열 앵커 확정 필요**: serpen SERPEN(kind게이트 `cmp [rax+0x68],6`)·MOBATICK·LAUNCHER·DMGA/DMGB·ARG_STR · item_tactics TIP_SHOW/CL_LAUNCHER · comptest RUN_RVA.
- **구조체 재확인**(2패치 연속 이동 이력): provider `0xeb28`·athlete stride `0x8c0` — serpen/item_tactics 명령대조 센서스로 실측 · ClientDatabase 오프셋(Spectator_Chat·community_reaction_mod 의존).
- ~~**SDK**: 0.5.5 SDK 다운로드 중 = **빌드 전면 대기**.~~ → ✅**완료(2026-08-12)** — 아래 §5~§8이 완결 기록.

### 5. ★모드별 재핀·배포 완결 (2026-08-12 같은 날 — ai_adjust 제외 전건) — **재핀 표 정본 = 각 `REPORT\<MOD_ID>\RE\2026-08-12_0.5.5-RVA재핀.md`**(여기는 요지)

| 모드 | 재핀 결과 | 배포 dll (전부 08-12 실측) |
|---|---|---|
| tfm2_draft_overlay | 4상수 전건(1차에서 확정) | 684,032B @13:47:30 |
| tfm2_banpick_illust | 잔여 13 전건(geom 6+mid 6+SLOTS) ⟹ 총 29 갱신. ⚠geom `.rdata`는 0.5.4식 "블록 통째 델타"가 아니라 **내부 재배치**(개별 유일검색으로 판정) | 2,892,288B @14:00:22 |
| tfm2_banpick_order | 함수 16+mid 18 전건·`bo_verify_055.py` FAIL 0. AITURN 스택슬롯 +0x190·AI파리티 2사이트 emit 재작성·HL(I) 출력 bl→r14b 재작성. AI6 6사이트 재생성(cfg OFF 유지 — 켜지 말 것) | 2,596,352B @14:35:37 |
| tfm2_item_tactics | TIP_SHOW `0x2587990`·CL_LAUNCHER `0x14ac3e0`(PROLOGUE[13] `0x68→0x38`)·owned_cap `0x15206a9`·gate3 `0xeb2fa8`·retaddr 5·VT `0x333b970` + 구조체 §6 전수 반영. ★**정정(08-13, 인게임 결함 = TN 침묵 전멸)**: 08-12 재핀이 **worker 콜러 프레임 오프셋(TN_FR_*)을 누락**(RVA·구조체엔 오답 0 — 프레임 의존 레시피는 migrate_rva/스켈레톤이 원리적으로 못 잡는 별개 축) → registry 실측 발화120·스캔0·db관측 0x1388(3중 자기검증이 오주입은 전량 차단). **v2.9.1 재핀** = 대회 worker **`0x1c6a530`**(구 0.5.4 `0x2392ed0`)·프레임 imm `0x1ceb8→0x22cc8`·**caller_rbp=launcher진입rsp+0x88 공식 불변**·슬롯 db/cfg/set_end/시드 = rbp+`0x22bf8/0x22bd8/0x22a40/0x22ad8`(구 0x1cde8/0x1cdc0/0x1cce0/0x1cce8)·cfg맵(+0x2a0/+0x2d0·stride 0x160)/레코드(+0x138~+0x151)/세트블록(0x100/+0xf8) 전부 불변·TN 링 16→64. 부수 = launcher 스택 인자 4개 Box-clone→**Arc::clone**(payload +0x10 deref)·대회 매치노드 리스트 db+0x16c80/0x16c88→**`0x16c98/0x16ca0`**·db+0x738/0x9020 불변. ⬜인게임 T5~T7(스캔>0·NG=0·크래시0). 전문 = `REPORT\tfm2_item_tactics\RE\2026-08-13_대회레코드-프레임오프셋-0.5.5재핀.md`. ★**TNR 레지스터 캡처(worker 팀키를 레지스터에서 직접 캡처→TLS로 launcher 전달 = "인자 추가" 등가 ⟹ TN_FR_* 프레임 의존 제거)**: ~~v2.9.2 = 팀키 접근점 mid-func 훅 siteA `0x1c6b5f0`(r10)/siteB `0x1c6b66d`(rsi)~~ = **volatile r10 live 지점 선택으로 크래시(v2.9.2 crash_log 23:13)·폐기** → ✅**v2.9.4 방식B = post-call 지점으로 이설·인게임 검증 통과(08-13 23:54)**: **siteA `0x1c6bd22`**(call 0x1c6bd1c 직후·base=**rdi** nonvolatile·복귀 0x1c6bd30·taskType 8) / **siteB `0x1c6ca37`**(call 0x1c6ca31 직후·base=**rsi** nonvolatile·복귀 0x1c6ca45·taskType 5)·orig_len=14·양 사이트 원본14B `48 8B 85 50 73 01 00 48 89 85 20 5E 00 00`(비유일이나 RVA 직접지정이라 무관)·점프유입 0·전부 rbp-rel(position-independent). ★**다음 마이그 재탐색 앵커**(14B 비유일이라 확장 앵커 필수) = siteA `0x1c6bd15` `48 89 95 58 28 02 00`(mov[rbp+0x22858],rdx 유일)+call+nop → **hook=앵커+0x0D** / siteB `0x1c6c9fd` 6연속 byte-store all=1 run(유일)+lea+mov rdx,rdi+call+nop → **hook=앵커+0x3A**. 레코드 계약(base=P) = 팀A[P−0x18]·팀B[P−0x10]·n[P−0x148]·setArr[P−0x150]·side=byte[setArr+(n<<8)−8]. 디스패치 = JT @data `0x345727c`(index=taskType−4). ★**인게임 실측** = 설치 A=1/B=1·캡처 A=152/B=1·소비76·고아77·**교차검증 레지스터 vs 프레임 스캔 OK=76/NG=0**(방식B 정확성 입증)·**대회 배경 주경로 = taskType 8 확정**(캡처 A 152≫B 1)·크래시 무재발. +설치경합 compare_exchange(0→3) 차단. ⬜**잔여(강등 보류)** = TNR 커버리지 소비76/런처발화2469 ≈ 3%(post-call 조립점이 launcher 발화와 1:1 아님) ⟹ **프레임 스캔 강등 불가·"레지스터 우선+프레임 폴백" 유지가 현행 정답**·커버리지 상향은 별도 개선감(기능·안정성 무영향). 전문 = `…RE\2026-08-13_TNR-크래시-원인분석.md`(방식B 절) | ~~605,696B @14:27:04~~ → ~~606,720B @08-12 18:48(slot3)~~ → ~~606,720B @08-13 14:18:41(v2.9.1 TN 재핀·`073c6c0`)~~ → ~~610,816B @23:04:41(v2.9.2 TNR·`9183ded`·크래시)~~ → **610,816B @08-13 23:50:17(v2.9.4 post-call·커밋 `23ec557`·T5~T11 검증 통과)** |
| tfm2_elemental_serpen | 잔여 전건: SERPEN `0x16be600`·MOBATICK `0x14f7e40`·LAUNCHER `0x14ac3e0`·RUNNER_CTOR `0x14ae060`·DMGA `0x11596a0`·DMGB `0x14d6400`·ARG_STR `0x12e74f0`·retaddr 4 + 구조체 38상수. ★**정정(08-12 저녁, 인게임 결함)**: RVA 9종은 ghidra 전수 재검증 전부 정답이었으나 **`O_ENTITY_ACCESSOR` 0x1c8→`0x1e0`(+0x18) 미반영 + `KILLS_LEN_OFF` 0xf000 오기입(정답 0xef00)** 2건으로 전면 무동작 → 수정·재배포(전문 = `REPORT\tfm2_elemental_serpen\RE\2026-08-12_0.5.5-인게임결함-serpen무동작.md`) | ~~420,864B @14:27:30~~ → **420,864B @18:36:23(결함 수정본)** |
| tfm2_comptest_unlock | RUN `0x1aa2930`·바이트패치 17+imm 2 = 19/19 orig MATCH(**orig 변경 4건** 재계산)·LIVE 훅 ~35건·G_* 관측 오프셋 4종(§6). ★**중대 발견 = 소스 ITEMCONV/COLLECT가 0.5.3 잔존값 → 0.5.4 내내 두 훅 죽어 있었음**(0.5.5 `0x21d1cd0`/`0x1aa4290`로 복구 — 인게임 확인 권장) | 286,720B @14:34:50 |
| tfm2_level_cap | mid 2 전건: LEN_LOAD `0x14d819a`(orig 변경 — GameSetting 베이스 rax→**r14** 회귀·스텁 인코딩 교체)·UI_CMP `0x95d8b9`(스텁 무수정) | 198,144B @13:54:12 |
| Spectator_Chat·community_reaction_mod | ClientDatabase 오프셋 **전부 불변**(앵커 센서스 25쌍 — 0x1338·0x1598·0x1670·0x17F8·0x1818·0x1820 등 전건 일치) | 333,312B @14:17:12 / 620,544B @14:17:43(+워크샵 교체) |
| daram2 view_plus 8종 | RVA 0 = 재빌드만. ⚠**유일한 SDK API 변경 = `game_core::MerchandiseProduct` 필드 2 신설**(`anchor_sell_price`·`anchor_purchase_rate`) → facility_view_plus만 소스 수정(초기값 추정·인게임 확인 잔여) | ~~coaching 284,160B·custom_tier 2,685,952B·facility 288,256B·finance 147,456B·recruit 333,824B·roster 433,664B·stats 2,715,136B·training 2,609,664B (@13:56~14:04)~~ → ★**정정(08-12 릴리스 회차): facility 제외 7종을 `build_full_remap.ps1`로 PII-free 재빌드·재배포**(1차 재빌드가 remap 없는 `build_full.ps1`이라 dll에 소스경로 PII 잔존 — §9). 최종 = coaching 284,160B·custom_tier 2,685,952B·facility 288,256B(1차분 유지·PII 0)·finance 147,456B·recruit **333,312B**·roster 433,664B·stats **2,714,624B**·training 2,609,664B (@17:33~35) |
| gg 워크샵 3종 | RVA 0 재빌드 + **워크샵 content dll 직접 교체·백업 `*.bak_20260812_pre055`**(ai_banpick_probe `3738236728`·meta_champion_tiers `3738236964`·meta_item_delegate `3738241856`·community_reaction `3738958482`) | 251,904B·229,888B·242,688B @14:05 |
| tfm2_mod_order | RVA 0 재빌드 | 203,776B @14:06 |
| TFM2_Meta_Dashboard | dll + save_probe.exe 재빌드(백업 생성 — 0.5.4 때 백업 누락 해소). save_probe.exe는 mods\ + DashboardApp\ 두 경로 교체 | dll 335,360B @14:07 · exe 7,352,832B @14:10 |
| tfm2_html_overlay | ★**08-12 배포 22종 목록에서 누락됐던 신규 모드(08-11 생성) — 08-13 보완**. RVA 0 순수 SDK = sdk_055 재빌드만 + deps `>=0.5.5, <0.5.6` + mod_info **0.1.0→0.6.0**(내부 기능 버전 일치화)·BOM無·deploy-verify PASS·`WebView2Loader.dat` 동봉 유지. GG 번들 save_probe.exe는 위 행에서 이미 0.5.5 교체 = 추가 조치 불요. ⬜인게임 확인(모드 메뉴 재활성 필요) = CURRENT ⬜절 | 331,776B @**08-13** 00:32:29 |
| tfm2_ai_adjust | ★**08-13 보완**(08-12 배포 전건에서 유저 별도 진행이던 마지막 1종). `src\rva_055.rs` 신설(단일수정점)·훅10+데이터3+JT2·stale 8 inert 유지·`build_full.ps1`(sdk_055·`--extern game_core`)·자기검증 check055 ~984/989 OK. ★**AI 판단 구조 = 온건(대개편 아님)**: 판단 트리 골격 1:1·신규 서브시스템 0(함수 +1,438=이미지/코덱 라이브러리). 변경 3 = ①Plan enum 0.5.3 번호 복귀(plan_disc_053 identity·MOVEPRI 인덱스식 바이어스 부활) ②MOVEPRI orig_len 12→14(프롤로그 push r15) ③구조체 오프셋 시프트(§6 ai_adjust 확정분). ⚠부수 = **dd7_slot128 잠복버그 해소**(World 유닛-로스터 리졸버 0.5.4부터 +0x10 stale=오레코드 조용사 → 0.5.5 정정값 stride 0x9e0·flag 0x9c8·id 0x9d0로 동시 해소). ⚠inert 잔여 = ~~nexus_emg 미마이그(넥서스 비상수비 기능 비활성)~~ → ✅**해소(08-13 완전마이그·재활성 = §6 nexus RVA 6종)**·d19_emit verification 로그 부정확(무영향)·dead knob ~5. ✅**인게임 검증 통과(08-13)** = 훅10·imm_guard checked=581/625 blocked=0·크래시0·nexus_emg 발화·후퇴 rt_imm 11/11·★**TeamPlan.version=2 실측**(AUC_PROBE 재활성·08-06 우려된 후퇴체인 사망은 0.5.5 재현 안 됨→켠 채 릴리스). ✅**릴리스 zip** `release\0.5.5\tfm2_ai_adjust.zip` 5,035,942B·14엔트리·PII0·zip내 dll 대조 OK · ✅**커밋 `4436581`**(src/*.rs 12개·push 안 함)·mod_info v1.6.1 유지. 정본 = `REPORT\tfm2_ai_adjust\RE\2026-08-12_0.5.5-구조전수조사.md`·`…_2단계변경맵.md`·`2026-08-13_*` 3건 | ~~3,668,480B @03:50:29~~ → ~~04:19:54~~ → **3,670,016B @08-13 09:27:49**(AUC_PROBE 반영·검증본) |
| tfm2_flow_capture | ★**08-13 보완**(08-12 배포 22종에서 누락된 신규 모드 = 경기 흐름 캡처+다시보기 전술 패널). 함수 4: RUN_TICK `0x14aa160`·CTOR `0x14ac3e0`·SCENE_STEP `0x196c2c0`·RECO `0x12ae860` + 공통3 + 구조체 4대역 전수 반영. ✅인게임 검증 통과(훅5·크래시0·run_tick 700만·flow 497파일·캡처 정상). **RVA 정본 = §11** | 439,296B @08-13 10:31:42 (v0.7.1) |
| legacy_save_patcher | stable ABI = 재빌드 불요(예외 유지) | (변경 없음) |

### 6. ★★0.5.5 구조체 시프트 정본 (명령 실측 — serpen·item_tactics·comptest·**ai_adjust(08-13)** 4세션 교차 무모순)
- **provider(game=ctrl+0x1dc0) 대역 +0x168 균일**: SEED `0xeb28→0xec90` · SIM_TICK `0xeb30→0xec98` · KILL0 `0xeb38→0xeca0` · KILL1 `0xeb40→0xeca8`(킬 증가 사이트 `inc qword [rdx+0xeca0/8]` @0x14fe3aa 등 5쌍 일관) · CAMP_SPAWN→`0xeea8`·CAMP_WAVE→`0xeeb0`·KILLS_PTR→`0xeef8`·KILLS_LEN→~~`0xf000`~~**`0xef00`**(정정 08-12 인게임결함 — 0xf000은 오기입(+0x268), 정답 = +0x168 = Vec{cap 0xeef0/ptr 0xeef8/len 0xef00}). ⚠provider 상위대역 **4패치 연속 이동** = 다음 패치 최우선 점검.
- ★**리졸버/getter 슬롯 테이블(엔티티 접근자, rdx로 전달) = 전역 +0x18** (신설 08-12 인게임결함): serpen `O_ENTITY_ACCESSOR` `0x1c8→0x1e0`(SERPEN 0x16be600 @+0x4d `mov rax,[rbx+0x1e0]` ↔ 0.5.4 `[param_2+0x1c8]` 직접 대조·0x1d0→0x1e8 동반). ⚠**엔티티 필드 0x1c8과 다른 구조체** — 값이 같아 센서스 착시로 "불변" 오판했었다.
- **athlete(player) = 대역분할 시프트(⚠균일 아님)**: `[0x408,0x6b0)` 대역 **+0x60**(owned len `0x448→0x4a8`·build len `0x490→0x4f0` — BUY 진입부 `cmp qword[r8+0x4f0],0` 직접 확증) / `[0x800,…)` 대역 **+0x120**(athlete_id `0x800→0x920`·team `0x810→0x930`·position `0x8a0→0x9c0`·champ_tag→`0x9c8`·champ_key→`0x9d0`) / **stride `0x8c0→0x9e0`**. ⚠순진한 균일 +0x120 적용 시 힙 손상이었음(band-split 함정).
- **World +0x18**: W_CHAMP_DENSE `0x720→0x738`·W_CHAMP_SLOTS→`0x750`·W_PLAYER_DENSE `0x840→0x858`·CHAMP_STRIDE `0x6a8→0x6c0`.
- **entity ≥0x5a8 대역 +0x18**: ENTITY_ID `0x5a8→0x5c0`·EXEC_MAXHP→`0x628`·CUR_HP `0x658→0x670`·DMG_WINDOW `0x670→0x688`.
- ★**PlayerViewInfo stride `0x260→0x2C0`(+0x60)** (신설 08-12 인게임결함 — 0.5.5 재핀이 GameView 내부를 재검증 안 해 누락): ingame_ui `imul r10,r10,0x2c0` @0xadbefd 등 4곳·GV update 0x964350 디컴 그룹스킵 -0x2c0·상수 0x2600→0x2C00 12곳. **엔트리 내부(+0 team/+8 pos/items +0x50·0x58·0x60)·GameView 헤더(+0x1d0/0x1d8/0x1e8/0xa8~0xb8)·dyn Item vtable(+0x58 key/+0x60 icon)·둘째 맵 stride 0x1b8은 불변**. slot3 아이콘 미표시 결함의 근본원인(전문 = `REPORT\tfm2_item_tactics\RE\2026-08-12_0.5.5-인게임결함-slot3아이콘.md`).
- ★**ai_adjust 08-13 확정분(4세션째 교차·위 대역 규칙과 무모순)**: athlete mid대역 **+0x60**(판단력 `0x3f0→0x450`) · athlete 고대역 **+0x120**(team `0x810→0x930`·role `0x8a0→0x9c0`·handle `0x818→0x938`) · entity 전역 **+0x18**(plan `0x5e8→0x600`·subplan `0x708→0x720`) · provider **+0x168** · objective Vec go **+0x18** · 리졸버 슬롯 **`0x1c8→0x1e0`**(serpen과 동일 = 2모드 공통 사망패턴). ⚠**dd7_slot128(World 유닛-로스터 리졸버) = 0.5.4부터 +0x10 stale 잠복버그**였음(크래시 없이 오레코드 read) → 0.5.5 정정값 **stride 0x9e0·flag 0x9c8·id 0x9d0**로 동시 해소.
- ★**ai_adjust nexus_emg(넥서스 비상수비) 완전마이그 RVA 6종(08-13·2중 교차검증 r=1.00)**: NXE `0xce3c18→0xdd1d08`(컨테이너 defense_nexus `0xdd1cd0`)·NXE_FAIL `0xdd1d13`·NXE_PASS `0xdd1d26` / NXM `0xe097b5→0xd9c825`(컨테이너 free_dist `0xd9c…`)·NXM_PLAIN `0xd9c865`·NXM_AFTER `0xd9c84b`. **region 오프셋 = 전부 불변 확정**(region은 entity와 별개 구조체·ghidra 브래킷 확증): O_TWIN_LEN 0x148·O_T1 0x180/0x1a0/0x1c0·O_T2 0x190/0x1b0/0x1d0 / 스택슬롯 O_REG_SLOT [rbp+0x8d0]·O_SIDE_SLOT 0x8c0·NXM_BASE_MARGIN 0x880 불변. install window 정적검증 = 신 RVA MATCH(설치)/구 RVA MISMATCH(미설치) = 미설치→설치 전환 확인.
- **불변 확정**: Game `+0x1dc0/+0x1dc8` · 엔티티 저대역 ≤0x258(0x68·0xb0·0x108·~~0x1c8~~·0x250·0x258 — 0x1c8은 위 리졸버 슬롯 정정 참조, 엔티티 **필드** 0x1c8과 혼동 금지) · ~~ClientDatabase 전 오프셋~~ → **ClientDatabase 저대역(≤0x2970) 불변**(0x1338·0x1598·0x1630·0x1670·0x1680·0x1820 등 — dword 센서스 0.5.4↔0.5.5 카운트 안정, +0x840 이동 클러스터는 ~0x13338 이상 고대역에만) · ★**cps(champion_patch_statistics) 고대역 = `0x16ed8`**(0.5.5 런타임 실측 addr_of 대조, 구 하드코딩 0x16698 대비 +0x840 / 0.5.4 실측 0x16ec0 대비 실이동 +0x18. 센서스: new에서 0x16ed8 출현 10→96 급증) ⟹ **Spectator_Chat(0x1598/0x1670)·community_reaction(0x1338~0x1820) 영향 없음 판정(08-12)** · VIEW_TICK 0x258 · GameSetting `+0xd00/+0xd08/+0xd10` · DD(드롭다운) `+0x1788/+0x1528/stride 0xf8` · kind값(6/0xd) · 전 PROLOGUE 가드(BUY·SEEDCTOR·FN_DD·ITEMNET 4종 바이트 불변 실측).

### 7. 부수 검증·인프라 (2026-08-12)
- ✅**SDK = `C:\tfm2mods\sdk_055\mod-sdk`**(GitHub 릴리스 0.5.5.zip 553,903,875B). rlib 154개(동수)·**mod_api/game_core/game_ai/game_view 4종은 파일명(StableCrateId)까지 변경** + 핵심 전부 내용 DIFF ⟹ 전 모드 재빌드(이행 완료). toolchain `nightly-2026-05-24` 유지·rust-lld 유지. 빌드스크립트 6종 sdk_055 전환(build_inj·build_full·build_full_remap·build_extra·banpick_illust\build·dashboard_probe\build).
- ✅**deps 도구 = `bump_deps_055.ps1`**(대역 `>=0.5.5, <0.5.6`) — 재빌드 완료 모드에만 적용(원칙 유지).
- ✅**세이브 포맷 무변경(6연속)** — sdk_055 재빌드 save_probe.exe로 실 세이브(44,408,110B) full-load: `save_probe ok: teams=120 athletes=1451 match_replays=11012`.
- ✅**bundle `.ui` = 완전 무변경**: 0.5.5 추출 290파일(`tfm2_0.5.5\bundle_ui\`) ↔ 0.5.4 **전건 MD5 비트동일**(추가·삭제·변경 0) ⟹ daram2 override 충돌 0(0.5.4의 "변경 5개"보다도 무풍). bundle +74,180B는 `.ui` 외 자산 추정.
- ⚠신규 함정: bash에서 build 스크립트 `-Src`를 슬래시(`/`) 경로로 주면 신원검증 가드 전 모드 오탐(백슬래시로 재실행) = `mods_report\_공통_빌드릴리스_교훈.md` §14.

### 8. ⬜잔여 (0.5.5)
- ⬜**배포 전 모드 인게임 미검증**(정적 검증만 완료). 우선 관측 = banpick_order AI파리티/HL(레지스터 실변경·위험 최상) · comptest 복구된 ITEMCONV/COLLECT + 킬스코어 · item_tactics is_live(0xec90)·4템·BG4 · serpen probe 훅 9종 · facility_view_plus 앵커 필드 초기값.
- ★**08-12 저녁 1차 인게임 검증 결과 = 결함 2건 발견·수정·재배포**(본 세션): ①serpen 전면 무동작 = `O_ENTITY_ACCESSOR 0x1c8→0x1e0` 미반영 + `KILLS_LEN_OFF 0xf000` 오기입(→0xef00) ②item_tactics slot3 아이콘 미표시 = **PV stride 0x260→0x2C0** 미반영(§6 신설 줄). RVA 재핀 자체는 양 모드 전수 재검증에서 **오답 0**. ⚠serpen_probe "발화 전부 0"은 부팅+1초 스냅샷 아티팩트였음(1회성 on_init flush가 재시작에 덮임 — 전이 flush 추가로 해소). ⬜수정본 인게임 재검증(serpen 색/버프/장로 · slot3 아이콘·[slot3진단] 카운터) 잔여.
- ★**08-13 인게임 결함 3호 = item_tactics TN 침묵 전멸(프레임 오프셋 재핀 누락)** — 값·경위 = §5 item_tactics 행 정정 참조. v2.9.1 프레임 재핀 → v2.9.2 TNR 레지스터 캡처(프레임 의존 제거·팀키접근점 훅) → v2.9.2 크래시 → **✅v2.9.4 post-call 이설·T5~T11 인게임 검증 통과(23:54 실측·교차검증 OK=76/NG=0·taskType 8 대회 주경로 확정·크래시0)**. ★**교훈①(버전무관)** = 콜러 프레임 오프셋 레시피는 migrate_rva/스켈레톤 매칭이 원리적으로 못 잡음 ⟹ **버전업 체크리스트에 "프레임 의존 레시피 재핀" 항목 필수**(프레임 의존 = TN_FR_* 3슬롯). ★**교훈②(버전무관)** = mid-func 트램폴린 훅은 **"call 직후(post-call)"가 "명령 임의 지점"보다 압도적으로 안전**(call 경계는 volatile 이미 파괴 전제라 직후 live는 nonvolatile로 좁혀짐) — **훅 후보 1순위 기준 = 그 지점 live 레지스터가 volatile이냐 nonvolatile이냐**(시그·점프유입은 그다음). v2.9.2가 팀키접근점(volatile r10 live) 선택→크래시, v2.9.4가 post-call(nonvolatile rdi/rsi)로 해결. ⬜TNR 커버리지 3%(강등 보류·§5 잔여).
- ⬜**ui_offset_probe sdk_055 재실측**(banpick_illust raw.rs 등 SDK 구조체 오프셋 의존 모드의 rlib offset 검증 — exe RVA와 별개 축).
- ⬜**신규 핫픽스 모드 `tfm2_bancard_keep`(08-12 저녁 배포) 인게임 미검증** — A/B 판별 대기(§10)·릴리스 zip = 판별 후.
- ✅**`tfm2_ai_adjust` = 마이그·배포·인게임 검증·릴리스·커밋 전부 완결(08-13, §5 행)** (~~유저 별도 진행(본 절 범위 밖)~~ → ~~정적만 미검증~~ 정정 08-13) — 인게임 검증 통과(훅10·checked=581/625 blocked=0·크래시0·nexus_emg 발화·후퇴 rt_imm 11/11·TeamPlan.version=2 실측)·릴리스 zip 5,035,942B(14엔트리·PII0)·커밋 `4436581`·dll 최종 3,670,016B @09:27:49. ⚠**inert 잔여**(별도 처리·크래시/활성경로 오작동 아님) = d19_emit verification 로그 부정확(무영향)·dead knob ~5. **ai_adjust 0.5.5 잔여 없음.**
- ~~⬜릴리스 zip 0.5.5 + rel_commit(진행 예정).~~ → ✅**완료(08-12 오후) = §9**.

### 9. ★릴리스 zip 15종 + 커밋 (2026-08-12 오후 — ai_adjust 제외 전건)

- **경로 = `<게임설치>\mods\release\0.5.5\`**(신설). 방식 = 0.5.4 zip 기준 스테이징 + dll/exe/mod_info만 라이브 반영(도구 = 신설 `rel_055.py`·`rel_it_source_055.py`·`rel_pw_055.py`, 0.5.4판 답습). 검증 = `rel_verify.py 0.5.5`(인자화 개정) **15/15 OK**(zip↔라이브 dll 전건 일치·진단 플래그 0) + `pii_check_055.py` **실질 PII 0**(유일 히트 = Dashboard `C:\Users\YOURNAME\...` 예시 문구 오탐).
- 실물(파일명 / 크기 / 엔트리): Spectator_Chat 162,513B(6) · tfm2_mod_order 103,166B(3) · tfm2_comptest_unlock 131,084B(3) · tfm2_draft_overlay 146,799B(3) · tfm2_level_cap 97,171B(5) · tfm2_item_tactics 272,441B(5) · tfm2_item_tactics_source 146,014B(10) · tfm2_banpick_illust 933,771B(6) · tfm2_banpick_order 827,075B(4) · **tfm2_banpick_order_pw(tfm2) 827,123B(4)**(동일 dll 암호 zip·비번 "tfm2" — ⚠0.5.4판=AES였으나 도구 부재로 **ZipCrypto**로 생성, 호환 동등 이상) · community_reaction_mod 322,926B(3) · tfm2_elemental_serpen 1,160,489B(28) · daram2_viewplus 9,406,496B(47) · 팀파매gg모드3종 611,177B(17·루트 `팀파매gg_0.5.5`) · TFM2_Meta_Dashboard 157,196,557B(944).
- cfg 정규화 유지 실측: item_tactics `4items.cfg slots=3` · serpen `elder_after=5`·probe cfg 280B 최소본·`0x3f` 0. 문구 정정(스테이징만) = level_cap README 0.5.3→0.5.5(3건)·crm README 0.5.2→0.5.5(3건)·gg zip 내 delegate mod_info deps `>=0.5.5, <0.5.6`(라이브 대응 파일 없음 = fix_delegate_dep_054 방식 인라인).
- ★**PII 사고 방지 실측(교훈 = `REPORT\_공통_빌드릴리스_교훈.md` §15)**: 1차 daram2 재빌드가 `build_full.ps1`(remap 없음)이라 **7종 dll에 `C:\Users\dev\...tfm2-mods-main` 소스경로 잔존** → zip PII 검사에서 발각 → `build_full_remap.ps1`(sdk_055) 재빌드·재배포 후 zip 재생성 = PII 0.
- **커밋(rel_commit.ps1 -Base 0.5.5, `C:\tfm2mods` master)**: `4a41e77` draft_overlay · `b859625` banpick_illust · `fea01d9` banpick_order · `6bd3c65` item_tactics · `20ba625` elemental_serpen · `3ba495d` comptest_unlock · `459db95` level_cap · `aa65586` Spectator_Chat · `3845e29` community_reaction_mod (+ 0.5.5 인프라 커밋 별건). 소스 무변경 = mod_order·gg 3종(커밋 대상 없음). **daram2 소스 = 커밋 불가**(위치 = `…\tfm2\tfm2-mods-main\` — tfm2 repo `.gitignore` `/*` 화이트리스트로 미추적·tfm2mods 저장소 밖 = 0.5.4 회차와 동일).
- ★**08-12 저녁 인게임 결함 수정 회차(추가)**: item_tactics(slot3 stride)·serpen(리졸버 슬롯) 재빌드·재배포 후 릴리스 zip 3종 재생성(`rel_055.py tfm2_item_tactics`·`tfm2_elemental_serpen` + `rel_it_source_055.py`) — item_tactics 272,772B(5)·dll 606,720B / item_tactics_source 146,890B(10) / elemental_serpen 1,160,464B(28)·dll 420,864B. `rel_verify.py 0.5.5` 15/15 OK. 커밋: `c75cfe8` item_tactics · `cd2c54c` elemental_serpen · `0f04ed2` MIGRATION 정정.

### 10. ★신규 핫픽스 모드 `tfm2_bancard_keep` — 환경설정 "밴 카드 수" 리셋 방지 RVA 등재 (2026-08-12 저녁, 0.5.5 · ⬜인게임 미검증) — **본 절 = 이 모드 RVA의 정본**

- **목적**: 환경설정 "밴 카드 수"(1~5장)가 룸 설정 커밋 새니타이저의 **풀 부족 클램프**로 GPO+0x720(`room_practice_ban_count`)이 0으로 리셋되는 것을 방지(§8 유저 제보 = DONE.md 08-12 행 후속·바닐라 설계 동작으로 확정).
- **RVA(0.5.5)**: 새니타이저 = **`0x21d0ad0`**(`FUN_1421d0ad0`·클램프 블록 `0x21d116e`~`0x21d11dd` — 챔피언 풀(+0x165c8) < style×20+ban×2+15면 리셋) · **패치 사이트(리셋 store) = `0x21d11d2`** orig 11B `48 C7 86 20 07 00 00 00 00 00 00`(`mov qword [rsi+0x720],0`) → **NOP×11**. 0.5.4 대응 = store `0x2138f3a`(**바이트 완전 동일 = 회귀 아님**, RE 문서 실측).
- **구현**: init 1회 바이트패치 — orig 11B 실측 검증 후에만 적용·멱등·VirtualProtect RW→원복·exe base = GetModuleHandle 런타임 도출. + 진단 로그 `bancard_keep.txt`(패치 결과 + 서버 tick ~2초 폴링으로 GPO+0x720 변화 시점 기록 — RE 잔여 불확실성 **(A)클램프 원인 / (B)팝업 셀렉션 커밋 누락** 판별용).
- **배포 실측**: dll **143,872B @2026-08-12 19:42:12** → `<게임설치>\mods\tfm2_bancard_keep\tfm2_bancard_keep.dll` · mod.mod_info 569B **BOM無**(첫바이트 7b) · deps `>=0.5.5, <0.5.6`. 소스 = `C:\tfm2mods\tfm2_bancard_keep\src\lib.rs`.
- ⬜**잔여**: 인게임 미검증 — 유저가 밴 카드 5장 설정 후 로그에서 ban_count 변화 확인. **(A)면 해결 / (B)면 이 패치 무효 = 옵션 셋터 경로 추가 RE 필요**. 릴리스 zip = 판별 후.
- 정본 문서 = `REPORT\tfm2_bancard_keep\{01_구조,02_구현정보,03_시행착오}.md` + `RE\2026-08-12_밴카드설정-리셋-원인-RE.md`. 교훈(신원검증 file!() = 버전무관) = DONE.md 신원검증 행 + `03_시행착오.md` 08-12.

### 11. ★`tfm2_flow_capture` 0.5.5 마이그·인게임 검증 완결 (2026-08-13, 0.5.5) — **본 절 = 이 모드 RVA·오프셋의 0.5.5 정본**

> ★**08-12 배포 22종 목록에서 누락됐던 신규 모드**(2026-08-12 세션 신설 = 경기 흐름 캡처 + 다시보기 전술 편집 패널) — 08-13 version-migrator로 마이그·재빌드·인게임 검증 완료. RVA 단일수정점 = `C:\tfm2mods\tfm2_flow_capture\src\{lib.rs, ui_replay.rs}`(정정형 주석·근거 포함).

**함수 RVA(구 0.5.4 → 신 0.5.5, 근거):**
- **RUN_TICK** `0x13b3150 → 0x14aa160`(disp-masked 구조skel UNIQUE + comptest ORACLE 교차 + 프롤로그14/size 0x1529 동일)
- **CTOR**(World 생성자) `0x13b53d0 → 0x14ac3e0`(구조skel LCP 342 + item_tactics/serpen LAUNCHER 교차 + 프롤로그[13] `0x68→0x38` chkstk imm)
- **SCENE_STEP** `0x1dad900 → 0x196c2c0`(banpick_order PHASE_SCENE 동일 함수 + `movzx [rcx+0xce]` 시그)
- **RECO** `0x14a0240 → 0x12ae860`(skel UNIQUE·본문 무변경·delta −0x1f19e0)
- **공통 3심볼**(전 모드 동일·ui_replay.rs) = LOADER `0x2e42d0`·PARSER `0x1a3e70`·ALLOC `0x2a9bf30`.

**구조체 오프셋(0.5.5 4대역 시프트 — §6 규칙과 정합):**
- provider **+0x168**: G_SEED `0xeb28→0xec90`·G_TICK→`0xec98`·G_SCORE0→`0xeca0`·G_SCORE1→`0xeca8` / OBJ_LOGS tower→`0xeec0`·epic→`0xeed8`·serpen→`0xeef0` / STRAT_OFF `0xb248→0xb3b0`(stride 0x18 불변).
- World 저대역 **+0x18**: ENT_PTR `0x840→0x858`·ENT_LEN→`0x860` / CE_PTR `0x720→0x738`·CE_CNT→`0x730`·CE_STRIDE `0x6a8→0x6c0` / KL_PTR `0xb208→0xb220`·KL_LEN→`0xb228`(stride 0x30 불변).
- athlete 대역분할 stride **`0x8c0→0x9e0`**: A_ID→`0x920`·A_TEAM→`0x930`·A_GOLD `0x878→0x998`·A_POS→`0x9c0`·A_NAME_PTR `0x410→0x470`·A_NAME_LEN→`0x478`·A_DEAL `0x5b0→0x610`·A_TANK→`0x618`.
- entity ≥0x5a8 **+0x18**: CE_X `0x648→0x660`·CE_Y→`0x668`·CE_HP→`0x670`·CE_HPMAX `0x610→0x628`·CE_LEVEL `0x5b0→0x5c8`.
- **불변**: G_OFF 0x1dc0·CE_KIND 0x68·CE_TEAM 0x8·CE_LANE 0x11a·CE_NAME 0x250/0x258.
- ⚠**미확정(0.5.4값 유지·크래시 없음·세트번호 표시만 영향)**: RM_OFF 0x2a0·ENTRY_STRIDE 0x160·SGR_STRIDE 0xeea8 (ServerState running_matches — 별도 객체·정적 앵커 곤란·런타임 setfeed 관측 필요).
- ⚠**함정: 0x5b0이 두 구조체 공존** — A_DEAL(athlete +0x60→0x610) vs CE_LEVEL(entity +0x18→0x5c8), **별개 마이그**(값 우연근접·혼동 금지).

**인게임 검증(2026-08-13 실측):** 훅 5종 base+새RVA 설치(base `0x7ff72ee60000`·64KB정렬)·크래시 0·run_tick 700만 발화·flow 파일 497개·캡처 데이터 정상(ce=10 챔프 전원·골드 8만대 비0·좌표 맵범위·킬 유효인덱스·serpen/tower 오브젝트). 배포 dll **439,296B @2026-08-13 10:31:42**·mod_info **v0.7.1** deps `>=0.5.5, <0.5.6`·BOM無.
- 정본 문서 = `REPORT\tfm2_flow_capture\` + [[tfm2-match-feedback-pipeline]]. flow_capture 0.5.5 잔여 없음(미확정 3오프셋은 세트번호 표시 한정·크래시 무관).

### 12. ★신규 모드 `tfm2_champ_pos_lock` — 챔피언별 허용 포지션 제한 + 0.5.5 밴픽 포지션 함수 재핀 (2026-08-19, 0.5.5 · ⬜인게임 미검증) — **본 절 = 이 모드 RVA·0.5.5 포지션 함수의 정본**

- **목적**: 특정 챔피언을 특정 포지션에만 쓰게 제한(cfg). 소스 = `C:\tfm2mods\tfm2_champ_pos_lock\` · 문서 = `REPORT\tfm2_champ_pos_lock\` · 등재 = `MODS\MOD_REGISTRY.md` 비-T1.
- **축1(RVA 0)**: SDK `ModDraftScoreHook` score_pick — 허용 포지션 매칭(홀 조건)이 깨지는 후보를 Replace(-1e9).
- **축2 Hook A(하드코딩 RVA 1개)**: **`0x1294180`** = champ→eligible-positions 비트마스크 산출기에 트램폴린 detour(마스크 AND 교정). ★**유효범위 한정(3차 RE 08-19 밤 확정)**: 마스크 소비처 전수 = SGD 밴픽 스코어러 클러스터(캐시 게터 `0x11cda40` 경유)뿐 ⟹ **AI 평가/추천/자동배정에만 유효 — 유저 수동 스왑·최종 라인업 저장은 미지배**(하드 강제는 아래 apply_lineup 후속 개입점 후보).
- ★**0.5.5 밴픽 AI 포지션 함수 재핀(ghidra 8080=0.5.5 실측 신원확인 후 RE)**:
  - **fast_pos_fit_score = `0x126ec40`** (확정 — 코어식 0.6/0.15/0.08·divisor 3 바이트 대조. 0.4.13 `0x19e2310`의 후계)
  - **plan_selector_score = `0x127cee0`** (0.4.13 `0x19fe5c0`의 후계)
  - **champ→eligible-positions 비트마스크 산출기 = `0x1294180`** (0.4.13 assign_positions `0x19f72a0`의 실질 후계, **강한 추정** — 하위 5비트 = 그 챔프가 쓸 만한 포지션 집합·5포지션 순회 fast_pos_fit + 문턱)
  - 근거 = `REPORT\tfm2_champ_pos_lock\RE\2026-08-19_0.5.5-포지션배정-RE.md`.
- ★**3차 RE 신규 확정(서버권위라인업 개입점, 08-19 밤 — 근거 = `REPORT\tfm2_champ_pos_lock\RE\2026-08-19_서버권위라인업-개입점.md`)**:
  - **마스크 캐시 게터 = `0x11cda40`** — HashMap 캐시·반환 mask&0x1f·프롤로그 push 10B `55 41 57 41 56 41 54 56 57 53` + `48 81 EC B0 00 00 00` = **orig_len 17**.
  - **apply_lineup(`0x193de10`) 라인업 Rec Vec 레이아웃**: 콜러 `0x19e88e0`·r9=&Vec{ptr@0, base@+8, count@+0x10}·**Rec stride 0x28**: pos@+8(u32)·가드@+0x10(≠-1)·champ String 16B@+0x18 — **유저 스왑까지 하드 강제할 후속 개입점 최우선 후보**. ⚠단 sim이 이 Rec에서 pos를 읽는지 **런타임 미확정** + pos 오교정 시 unwrap 패닉 ⟹ **훅E식 가드 필수**.
  - ❌**정정(1차 RE 오인)**: ~~"`0x1f387f0`→`0x307ff0`(mgr+0x20,&order) 큐잉"~~ → **`0x307ff0` = siphash 해셔**(큐 push 아님).
  - ⛔**SwapDone(disc32) 소비 = 서버 모노리스 `0x216e870` 내부 async 상태처리에 흡수** ⟹ **소형 워커 detour 분리 불가(비권장)**.
- ★**축1+축2 종합 한계(3차 RE)**: 현행 v0.1.0 = AI 쪽 제한만 — 유저 수동 스왑·최종 라인업 하드 강제는 미구현(후보 = apply_lineup Rec 개입·위 ⚠조건부).
- ⛔**함정(재작업 방지 = DONE.md 08-19 행)**: 0.5.5 **`0x1a636c0`**(0.5.2 라인업조립 `0x1a26690`의 후계)은 **sim 참가자레코드(stride 0x100·pos u64) writer가 아님** — GAME_ALLOC 0xa0 힙에 id/champ/item/pos/strat을 **String 튜플**로 clone하는 직렬화 Rec 조립기(pos도 String)·내부 +0x9020은 해시맵(레코드 배열 아님). **"pos write 사이트 `0x1a67a65`/`0x1a67cd5` rcx 교정" 방식 개입 금지**. 구버전(0.5.2) 채록 참가자레코드 오프셋(+0x9020/stride 0x100/+0x68 u64/+0xf8 side)은 0.5.5에서 시프트 정황(side byte-store 스캔 0건) = **0.5.5 재특정 전까지 사용 금지**. 근거 = `REPORT\tfm2_champ_pos_lock\RE\2026-08-19_레코드조립기-정밀판독-정정.md`.
- **배포 실측**: v0.1.0 dll ~~166,400B @2026-08-19 23:31:09~~ → **166,400B @2026-08-19 23:43:00**(cfg 주석·mod_info 문구 정정 재빌드·크기 동일 — 정정 08-19 밤) → `<게임설치>\mods\tfm2_champ_pos_lock\tfm2_champ_pos_lock.dll`.
- ⬜**잔여**: 인게임 미검증(축1 Replace 반영·축2 detour 발화·마스크 교정 동작) · 릴리스 zip = 검증 후 · sim 참가자레코드 0.5.5 재특정(개입 확장 시에만) · **유저 수동 스왑 하드 강제 = 미구현**(apply_lineup Rec 개입 후보 — sim의 pos read 런타임 확인 선행).


## §7.6 · 게임 0.5.6 마이그레이션 — 패치 성격 · exe↔exe 재핀 (2026-08-20, 0.5.6) — 본 절 = 0.5.6의 정본

> ⚠ 본 절 위 §7.5(0.5.5)·이하는 이력. 0.5.6 이후 작업은 이 절만 볼 것. ~~⚠tfm2_ai_adjust는 별도 세션 진행(본 절 범위 밖·ai_adjust RVA 재핀 안 함)~~ → **정정(08-20 저녁): ai_adjust도 마이그·배포 완료 — 본 절 §3 tfm2_ai_adjust 절이 정본(~~⬜인게임 미검증만 잔여~~ → ✅**08-22 인게임 2판 검증·릴리스 zip까지 완결·잔여 없음** = §3 정정)**.

### 1. 버전 사실 (실측)
| 항목 | 0.5.5 (직전) | 0.5.6 (현행) |
|---|---|---|
| exe | 76,957,696B | 77,101,056B (+143,360B) |
| sha256[:16] | 09E12009BB240EED | A0D8E395581FDF5A |
| .text vsz | 0x324d5ef | 0x326866f (+110,720B, +0.21%) |
| 인덱스 pkl | _fnidx_055.pkl | _fnidx_056.pkl (fnindex.py 신규 빌드) |

- 백업: OLD=tfm2_0.5.5\TeamfightManager2.exe / NEW=tfm2_0.5.6\TeamfightManager2.exe (bundle +17,631B 변경 포함).
- 패치 성격 = "온건한 핫픽스급 + 국소 로직 변경". .text +0.21%(0.5.5의 +1.73%보다 훨씬 온건). 함수시작 재링크 대부분 UNIQUE(BYTE=SAME) = RVA-only 이동. NO MATCH/본문변경은 밴픽 AI 개선(패치노트 2) 영역에 집중(banpick_order AI6 컨테이너 2개). 순수 핫픽스에 가까움(0.5.5 같은 대공사 아님).
- 구조체 = 저대역 전면 불변(0.5.5와 큰 차이): provider 0xec90 / athlete 0x4a8/0x4f0/0x920/0x930/stride 0x9e0 / entity 0x5c0/0x628/0x670/0x688 / World / PV 0x2c0 / ClientDatabase 저대역 0x1338/0x1598/0x1670 전부 0.5.6에서 불변(BUY 진입 cmp[r8+0x4f0]·owned_cap cmp[rsi+0x4a8]·DMGA ent필드·SERPEN [rbx+0x1e0]·MOBATICK provider disp 실측 확증). 리졸버 슬롯 0x1e0·SEED 0xec90·KILLS 0xef00 불변.
- 유일한 구조체 이동 = ClientDatabase 고대역 +0x120: cps 0x16ed8->0x16ff8(직독 39/40) · TN 매치노드 ptr 0x16c98->0x16db8·len 0x16ca0->0x16dc0 · db+0x9020->0x9140. (item_tactics cps=폴백/진단용·매치노드 오프셋은 현 소스에 하드코딩 없음=v2.9.5 프레임 스캔 단독.) Spectator_Chat·crm 저대역엔 영향 0.
- 재핀 도구(신규) = _mig056.py·mig056_fns.py·mig056_mids.py·mig056_align.py·mig056_mono.py·mig056_tn.py·mig056_fix*.py.

### 2. 공통 4심볼 (전 모드)
- LOADER 0x2e42d0->0x2e6f60(string-xref layout/main x17·clone family) · PARSER 0x1a3e70->0x19ab40(skel UNIQUE·BYTE=SAME·2192B) · ALLOC 0x2a9bf30->0x2ab1670(skel/마스크 UNIQUE·BYTE=SAME·60B) · ANIM_GET 0x844160->0xbea4a0(CARD_DRAW 콜슬롯14 확정).

### 3. 모드별 재핀 표 (구 0.5.5 -> 신 0.5.6 · 판정) — 소스 갱신 완료(빌드는 메인 세션)

tfm2_flow_capture (전건 UNIQUE·갱신완): RUN_TICK 0x14aa160->0x14db7e0 · CTOR 0x14ac3e0->0x14dda60(launcher 9/9 전단사) · SCENE_STEP 0x196c2c0->0x24d1dc0(마스크시그·movzx[rcx+0xce]) · RECO 0x12ae860->0x2ce38f0 · 공통3. 구조체 0.5.5값 전부 유효. 미확정 3오프셋(RM_OFF 0x2a0·ENTRY_STRIDE 0x160·SGR_STRIDE 0xeea8) 유지(세트번호 표시만·크래시 무관).

tfm2_draft_overlay (공통4만·갱신완): LOADER·PARSER·ALLOC·ANIM_GET = 2절 값. 전건 확정.

tfm2_banpick_illust (전건 UNIQUE·갱신완 29상수): 함수시작 16(FX_SET 0x24b6e40·CARD_DRAW 0x24cc8d0·ILLUST_GET 0x2384420·SUBMIT 0x181650·SUBMIT_TEXT 0x1818d0·IMG_BUILD 0x182d70·IMG_UV 0x182bd0·IMG_FLAG 0x183080·IMG_COLOR 0x1ed700·IMG_SHADER 0x184680·TEXT_BUILD 0x182260·NAME_GET 0x25126f0·ASSET_GET 0x143d70·ANIM_GET 0xbea4a0·SPRITE_CALC 0x2517620·GAME_ALLOC 0x2ab1670) BYTE=SAME. geom .rdata 6(C_CARD_RECT 0x34cd6a0 등·16B 내용동일·이동만) + mid 6(I_SNAP_H 0x254d7c0·D_SNAP_W 0x254d7d6·D_CUT_LO 0x24d6b08·D_CUT_HI 0x24d6b16·D_ZIG_X1 0x254e622·D_ZIG_X2 0x254ed00·float 480/360/-70/70/-180 검증) + SLOTS 0x40c5000->0x40e6000(64B all-zero).

tfm2_item_tactics (갱신완): FN_DD 0x1c1af0·TIP_SHOW 0x1efca70(head-UNIQUE)·TIP_MEASURE_VT 0x333b970->0x334cd10(UImega lea @0xab5321)·GAME_ALLOC 0x2ab1670·GV_UPDATE 0xb52b80·REALLOC 0x2a9d1b0·CL_LAUNCHER 0x14dda60(launcher 9/9)·SEEDCTOR 0x10a3be0(seed[rsi+0xec90])·BUY_ITEM 0xebca20·ITEMNET_FWD 0xf53de0 전부 BYTE=SAME. 바이트패치: owned_cap 0x15206a9->0x154c679(orig 일치)·imm 0x154c680·gate3 sig 0xeb2fa8->0xebcd88·jbe 0xebcd8e(orig 일치). launcher retaddr 4(콜+5): 관전 0x8404e1·내경기 0x84544b·조테본경기 0x1af18a2·조테기록 0x1ac1b2e. TN: RA_TOURN 0x1c777d8->0x1f24068 + 프레임 슬롯 재핀(worker 0x1c6a530->0x1f16ea0·정렬 0.956·chkstk 0x22cc8->0x23ce8): TN_FR_DB 0x22bf8->0x23c18·TN_FR_CFG 0x22bd8->0x23c00·TN_FR_SETEND 0x22a40->0x23b20(프레임 오프셋=migrate_rva 못 잡는 별개 축·명령단위 대응). CPS_OFF 0x16ed8->0x16ff8(폴백·진단용). 구조체 저대역 전부 불변. ★**정정(0.5.6·08-20 저녁 — 인게임 전면 무동작 실사고)**: 위 재핀이 **exe 크기 버전 게이트 상수(GAME_EXE_SIZE, 구 76,957,696)를 누락** → 게이트가 설계대로 **모드 전체 자기비활성**(4번째 아이템 열 실종·크래시 없음·version_gate.txt로 특정) → `GAME_EXE_SIZE_056 = 77_101_056`(lib.rs) 수정·dll **606,720B @2026-08-20 17:42:24** 재배포·zip 2종 재생성·커밋 `6aa7da4`·~~⬜인게임 재확인 대기(4번째 열 표시)~~. exe-size 게이트 보유 모드 = item_tactics 유일(전 소스 grep 실측) — 3형제 축 = §4·DONE.md 3형제 행. ★★**최종 정정(0.5.6·08-20 저녁 — "지정 빌드 미적용" 완전 해결·인게임 검증완·유저 확인)**: 게이트는 **3중 원인 중 ①**일 뿐 — ②★**TN cfg 맵 오프셋 `+0x2a0/+0x2d0 → +0x320/+0x350`(+0x80) 미재핀**(0.5.5 채록 "cfg맵 +0x2a0/+0x2d0 불변" = §7.5 기준·**0.5.6은 +0x80 이동 — 이 신값이 정본**) ③★주범 = **워크샵 riot_items_tfm2 v0.9.2(제작자 0.5.6판·08-20 자동 업데이트)가 같은 BUY_ITEM(0xebca20)·SEEDCTOR·LAUNCHER를 선점 후킹**(riot dll 내 상수 5/3/2회 실측) → item_tactics buy 설치기만 체인 미지원이라 영구 포기 → **`install_replace_buy` 체인 후킹 구현**(외부 훅 12B 재배치·설치상태 3=체인OK 신설). 검증 = 설치3·buy 진입 369만·write 27·4칸 UI·크래시 0·유저 인게임 확인. 최종 클린판(BUY_REPORT=false) dll **607,232B @2026-08-20 19:03:57**·릴리스 zip 2종 갱신(273,201B/150,841B)·rel_verify 15/15 OK·커밋 `99148ce`(중간 `6aa7da4`). 전문 = `REPORT\tfm2_item_tactics\03_시행착오.md` 08-20 + `RE\2026-08-20_0.5.6-TN맵오프셋-BUY오독정정.md` + DONE.md 맨위 2행(워크샵 충돌 축·"발화 0"은 진입 카운터로만).

tfm2_elemental_serpen (갱신완): SERPEN 0x12ce9b0(리졸버 [rbx+0x1e0] 확증)·MOBATICK 0x1521770(provider disp 확증)·LAUNCHER 0x14dda60·RUNNER_CTOR 0x14ae060->0x14df6d0(콜슬롯·프롤로그 동일)·DMGA 0x17f8090(구조정렬 1.000·disp만 이동)·DMGB 0x1501db0·KEYRES 0x1f8bc80·ARG_STR 0x12e74f0->0x16b5a70(콜러 투표)·SPAWN0 0xaf97d0·SPAWN1 0xaf8bc0. retaddr A 0x8404e1·B 0x84544b·C 0x1db3884·D 0x1ac1b2e. 구조체(리졸버 0x1e0·SEED 0xec90·KILLS 0xef00·CAMP 0xeea8) 전부 불변. ★**정정(0.5.6·08-20 밤 — 화면경기 기능 전멸(장로UI·화면카운터·툴팁) 실사고 → 해결·인게임 검증완·유저 확인)**: **RVA 재핀은 전건 무결(RVA 무변경)** — 원인 = 워크샵 riot_items_tfm2 v0.9.2가 LAUNCHER(0x14dda60)를 **CALL식 후킹** + serpen이 init 선설치라 **체인 안쪽 → retaddr 오염**(런처 발화 105 중 화면경기적중 0·retaddr 쓰레기값 0x4c6d098ea·LIVE_SEED=0·배경 sim 속성 배정은 정상 = 훅 자체는 발화). 수정 = **런처 훅 지연 설치+체인 전환**: `launcher_install_tick`(post_update 매프레임) — 외부훅(`48 b8 <tgt> ff e0`) 출현 시 그 12B를 연속부로 담아 **우리=바깥쪽 체인 설치**(`install_stub_chained` 신규·스텁 골격 = install_stub_generic 동일) · 600프레임(≈10초) 미출현 시 직접 설치 폴백 · 설치 확정 후 재검증/재체인 금지. 검증(08-20 밤 인게임) = 체인 OK(외부훅 tgt 위 체인·지연 120프레임)·화면경기적중 1·retaddr rva 0x84544b(내경기 화이트리스트 정상)·재생경기 선택 포착·유저 "잘 나온다". 배포 = dll **423,936B @2026-08-20 19:18:56**·zip 1,161,552B(rel_verify 15/15 유지)·커밋 `6396c39`·push 완(16d9bf7..6396c39). 전문 = `REPORT\tfm2_elemental_serpen\03_시행착오.md` 08-20 + DONE.md 워크샵 충돌 행(serpen=관측 오염 축 통합).

tfm2_banpick_order (함수시작+mid 대부분 갱신완·AI6 잔여): 함수시작 16 전건(PHASE_SCENE 0x24d1dc0·PHASE_SCALAR 0x10ac1f0·APPLIER 0x24b6c10·SLOTUPD 0x251e3e0·PHASE_RAW 0x24ac690·APP_PICK_T1 0x24a2c90·T2 0x24a2e20·APP_BAN_T1 0x24e09a0·T2 0x24e0b20·TRANSITION 0x24acdc0·BANNER 0x24b4150·LINEUP 0x24a3750·COMMIT 0x10b0530·TURN 0x10b0c60·TRIGGER 0x24d58b0·PANIC_HOOK 0x2aac7b4) + PROLOGUE_SCALAR rip-rel disp 93 2B 1A->83 67 2F. mid 재작성: AI_SITE1/JOIN1 0xf97732/0xf97840·AI_SITE2/JOIN2 0xf9b348/0xf9b419(DELTA-OK·sig 일치) · AITURN_SITE/JOIN 0x2079b43/0x2079c2a(정렬 0.968·스텁슬롯 total 0x6040->0x6a20·rule 0x5ef1->0x68d1·ban 0x5ee8->0x68c8) · SFX_SITE/END 0x2550825/0x2550874(드레인 씬슬롯 0x12c0->0x1300)·STR_BAN 0x34d84f6·STR_PICK 0x34d8512 · DRAIN_HL 2(0x256066d/0x2560984)·HL(0x1f0086b)·DRAIN_HL2 3(0x25546a2/0x2554fa8/0x2554bb0)·SLOTSEL(0x2569fd5·트램폴린슬롯 0x12f8->0x1338) ~~전부 sig 일치~~ → ★**정정(0.5.6·08-20 오후 — "이어하기" 즉사 AV 실사고)**: 위 1차 재핀은 mid-func 스텁의 site/join **주소만** 갱신하고 **스텁이 하드코딩한 rbp-disp32(arm 부작용·입력 슬롯)를 재핀 안 함** — HL(0x1f0086b) 스텁 arm lea disp가 0.5.5값(0x9b10, 0.5.6 정답 0x9cf0)이라 r8 쓰레기가 해시맵 포인터로 역참조돼 AV(14:48·fault exe+0x1f5d126·fault addr 0x8001·comptest_crash.txt 실측 — SIG 9B가 arm 본문 미커버라 결함 스텁이 sig 체크 통과·설치됨). ghidra-re 디스크 바이트 교차검증 PASS 후 **동류 수정 4건** = HL rule ~~0xaec9~~→**0xb0a9** + HL arm lea ~~0x9b10~~→**0x9cf0** / DRAIN_HL cur total ~~0x10f8~~→**0x1128** / DRAIN_HL next rdx ~~0x12f8~~→**0x1338**(잠재 크래시 2호) / SLOTSEL SIG 끝2B ~~f7fe~~→**b5b6**(불일치 사일런트 스킵 = 슬롯 하이라이트 기능 죽어 있었음). + ★**훅 M(HL_COUNT) = 1차 재핀서 통째 누락 발견**(sig 스킵 = 흰칸 개수 기능 사망) → 신주소 **site 0x2555090·join 0x2555128** 재핀(SIG 8B `41 ff e2 4c 39 df 0f 83` 0.5.5 동일·단일매치·스텁 계약 불변). 재배포 = dll **2,587,136B @2026-08-20 15:07:59**(build_extra 신원 OK)·zip 823,795B + pw 823,843B 재생성·rel_verify OK·커밋 `40a8439`. ⬜인게임 재검증 대기(이어하기 무크래시·밴픽 하이라이트 3종·훅 M 흰칸 개수). 정본 = `REPORT\tfm2_banpick_order\RE\2026-08-20_0.5.6-이어하기크래시-HL스텁-disp미재핀.md` + DONE.md 버전무관 교훈행(mid-func 스텁 재핀 = disp 바이트 전수 대조 필수). AI6 6사이트 = 0.5.5값 유지(미갱신): 3 확정(ai_comp 0x214a9c5·ai_bb3 0x214c69a) + 3 본문변경(컨테이너 0x1cb8640 ai_reco1/2·0x1cbb1a0 ai_bb1/2 = 밴픽 AI 개선·sig/arm 재작성 필요) -> ghidra-re 이관. 안전: cfg.ai_inline_phase 기본 OFF라 미설치 + sig 검증 fail-safe = stale 무해.

tfm2_comptest_unlock (갱신완): 함수시작·컨테이너 27(RUN 0x1abbb90·CGATE 0x1aae7d0·SREG 0x2082390·RESULT 0x1af0790·CSEND 0x1aaa600·HPUSH 0x1792e60·RPLY2 0x1ac4350·RPLY3 0x1ac15d0·LIVEB 0x1af1440·CTX_CLONE 0x1f5f7a0·ARRIVE 0x1ac4bb0·WARN 0x1a94b00·REFRESH 0x1aa3c70·ITEMCONV 0x2094560·COLLECT 0x1abd4f0·RUST_ALLOC 0x29ed940·CLONE_CHAMP 0x1b9dd10·FN_DD 0x1bfc10·ORACLE=RUN_TICK 0x14db7e0·SIMBODY 0x235d550·DROP_CHAMP 0x1616920·INSERT 0xcb5890·ATH_GET_SC 0x157fa80·DRIVE 0x888d20/RESUME 0x888d36·DEDUP_INS 0xcb13c0 등). 바이트패치 19/19 orig MATCH(no_stamina 0x2048e37·dr_inline a/b/d 0x1aa3dd6/0x1aae9d6/0x1b0ac2c·panel_btn 0x1b0b052·daily_inc 0x2044205·server_pregate 0x20413ca·server_dedup_real 0x2082803·allow_dup 0x1aaee81·server_dedup 0x204022f·btn5v5 a/warn 0x1b0b054/0x1b0ac6c·server_roster_min 0x2082760·roster_count 0x1abbe88·collected 0x1abbe7c·collect_err 0x1abbe5f·run_push 0x1abc585·TAKE 0x1aafa9a·PAGE_IMM 0x1aaae2c). RUNNER_VT 0x348db28->0x34afda8(slot4 매핑 일치). CT_REGION/CLIENT 경계 갱신(inert). 미확정(전부 inert·CONC_PROBE/죽은 상수·fail-safe): MFORGE·A15E20·POLLER·SLOT·SIMBODY(CONC_PROBE OFF) / INSERT·ENQ·DEDUP_INS·ATH_GET·PUSH·SPAWN_CP·SRV(죽은 상수·원복) — 크래시/활성경로 무영향.

tfm2_level_cap (갱신완): LEN_LOAD 0x14d819a->0x1503b4a(델타OK·orig 498b96100d0000 일치·r14 베이스 불변) · UI_CMP 0x95d8b9->0xb4c0d9(컨테이너 본문변경이라 owner내 유일검색·orig 483b88100d0000 일치).

tfm2_champ_pos_lock (신규 모드·재핀완+유저픽차단 축 추가·⬜시각검증 잔여): 공용3(LOADER 0x2e6f60·PARSER 0x19ab40·ALLOC 0x2ab1670) + hookA POS_MASK 0x1294180->~~0x2e739e0~~ **0xf83830**(champ->eligible-positions 비트마스크 산출기·마스크시그 0x40 유일 — ★정정 08-21 ghidra-re: 구 0x2e739e0 = migrate_rva 오매칭(Skia GPU 렌더러·프롤로그 우연일치)로 hookA 0회 발화, 정답 0xf83830·호출경로 AI밴픽스코어러→캐시게터 0x10659d0→0xf83830→fast_pos_fit) + **hookD' scene_step 0x24d1dc0**(유저픽차단 씬 캡처·banpick_order A' 재사용·entry rcx=진짜 클라 밴픽 씬(23 client 콜러·매프레임·lookahead 아님·T1/T2 구분)·프롤로그 48 8B 81 60 01 00 00 48 8B 91 78 01·씬 4벡 T1BAN 0x140/T2BAN 0x158/T1PICK 0x170/T2PICK 0x188(Vec base+8=ptr·+0x10=len·stride 0x18)=banpick_order 동일·**체인 설치**(install_once_dp: chained면 즉시·아니면 ~300프레임 유예 후 원본)) + contains 0x943440->0xb31840(밴픽 그리드 회색판정 헬퍼) + slot_widget 0x1971b00->0x24d7640(banpick_champion_slot·프롤로그 2후보를 contains 콜사이트로 판별) + contains 콜사이트 0x1977c91/0x1977caa->0x24dd7ae/0x24dd7c7(새 slot_widget 내 E8->새 contains 스캔·정확히 2개). 프롤로그 전부 BYTE=SAME(RVA-only 이동). ⚠hookA/slot_widget 본문은 0x40 이후 변경(밴픽 AI 국소 변경)이나 함수시작·계약 불변으로 판단. deps >=0.5.6,<0.5.7·sdk_056 재빌드·dll ~~343,552B @08-20 14:40:50~~ → **479,232B @2026-08-21 02:17**(scene_step 유저픽차단 축·DRAFT_SNAP 폐기). 소스 단일수정점 = src\hooks.rs + src\inject.rs 상단. ★유저픽차단 최종 = **scene_step 훅 방식**(hookD entry·greying 콜사이트·DRAFT_SNAP(score_pick ctx) 3방식 폐기 = DONE.md 08-21). 검증 = blockdbg `userblock: pick_t1=[dual_blader] pick_t2=[clown] ban_t1=3 ban_t2=3 pinned=1 block=18 anyfeasible=true`(오염 없는 실상태)·`hookD'(scene_step 씬 캡처) 설치 OK (chained=true)`(체인 성공·banpick_order 무손상). ⬜픽단계 시각 회색화(ui_block>0)는 환경적 밴 프리즈로 미도달(아래).

tfm2_ai_adjust (0.5.6·08-20 저녁 별도 세션 편입 — 마이그·재빌드·배포·정적검증·커밋 완료·~~⬜인게임 미검증~~ → ✅**08-22 인게임 1판 4/5 PASS·③class_micro 4사이트 수정 재배포 → 2판 전 항목 PASS·릴리스 zip 완 = 완결·잔여 없음 = 절 끝 정정**): **훅10+데이터3 신값** = RETREAT 0xcead40 · GENERIC_BUILD(GB) 0xccae00 · FC59A0 0xd01950 · CONDGATE 0xe23190 · MOVEPRI 0xe23ad0(JT 0x338128c) · SUBPLAN 0xe55240(JT 0x3381d6c) · DISC18 0xd32b70 · DISC19 0xd426e0 · ITEMNET 0xf53de0 · AUCTION 0xe0e5e0 · TABLE_A 0x3370008 · C8C 0x336dcd0 · DISC7 0x3372198. **판단함수 본문변경 0건·전부 RVA-only**(1단계 전수조사 = `REPORT\tfm2_ai_adjust\RE\2026-08-20_0.5.6-판단함수-전수조사.md`). RVA 단일수정점 = **`src\rva_056.rs` 신설**(include 교체) · orig_table.rs 0.5.6 재생성 **908엔트리**(구 625) · **check056 = 991중 982 OK**(잔여 9 = 0.5.5 기준선 동형 inert/false-alarm) + 훅 10 프롤로그 전건 PASS. 빌드·배포 = build_full.ps1(sdk_056) · dll ~~3,664,896B @2026-08-20 20:41:38~~ → **3,664,896B @2026-08-22 05:47:10**(class_micro 4사이트 수정 재배포·identity-verified·커밋 `150cb05`) · deps `>=0.5.6, <0.5.7`(BOM無) · **deploy-verify 전 항목 PASS**(dll 내 신 RVA 5종 확증·구값 0회·stale 없음) · 커밋 `3ba3c22`(-NoZip·push 안 함) · 릴리스 zip = 인게임 검증 후. ★**0.5.5 회차 누락 5묶음 발견·복구**(전부 침묵 비활성이던 것): ①class_micro 18사이트 전체 0.5.4 stale → ~~16/18 복구~~ **정정(08-22): "16/18 복구"는 dry-run 계산 기준 — 적용 스크립트 병합 누락으로 mv2 3사이트+sf_margin 신값이 소스 미반영(실설치 12/18)·인게임 1판서 BLOCK으로 발각 → 4사이트 신값 재핀 = mv2_avoid_coef `0xe5b5a1`·mv2_margin `0xe5b5f5`·mv2_bias `0xe5ba4f`·sf_margin `0xd4a56b`(0.5.6 실바이트 win 전건 일치)·정적 18엔트리 16 OK+2 보류(ex_order_hold·ex_think_min = 기존 보류 2건)·check056 무회귀 982/991** ②bt_vision imm 11 ③nexus_emg imm 2(08-13 "완전마이그" 선언이 imm 축 누락) ④patch_toggle_bytes 2(파서 사각) ⑤OK_DESC 화이트리스트 0.5.4값 방치(0.5.5 내내 probe_basedmg 전 호출 차단이었음). 보류 2건(fail-safe inert·심층 RE 별건) = ex_order_hold(self 재료 소멸)·ex_think_min(명령 분해→재설계 필요). 잠재 위험 축 별건 = disc19_repro shadow-call 5종 게이팅 감사 권고. ~~⬜인게임 검증 항목 = 훅10 설치·imm_guard·class_verify self확보%·nexus_emg 발화·크래시 0.~~ → ★**인게임 1판 결과(08-22 05:33 세션·verify-analyst)**: ①훅10 설치 PASS(스텁 n=11) ②imm_guard PASS(checked=905/908·blocked=0) ④nexus_emg PASS(설치 2/2·판정 834만·비상 26%) ⑤크래시 PASS(이번 판 시간대 0건) — **③클래스 노브 부분 FAIL**(cs_lead 61% 기준선 재현·bt_vision 11 OK / mv2 3사이트+sf_margin BLOCK = 위 class_micro 정정의 발각 경위) → 수정 후 재배포 = 위 dll **@2026-08-22 05:47:10**·커밋 `150cb05` · ~~⬜2판째~~ → ★★**2판째 전 항목 PASS(08-22 05:49 세션·verify-analyst) = 0.5.6 인게임 검증 완결**: class_micro **설치 16/18**(수정 4사이트 `0xe5b5a1`·`0xe5b5f5`·`0xe5ba4f`·`0xd4a56b` 전부 설치·발화·클래스 전용값 실사용 — coef/margin 454만 중 145만·bias 20%·sf 16%·%는 판 구성 의존 지표라 기준선 차 ≠ 결함)·무회귀(훅 n=11·imm_guard 901/908 blocked=0·nexus_emg 2/2·cs_lead 73%)·크래시 0. ✅**릴리스 zip 완(08-22 05:54)** = `<게임설치>\mods\release\0.5.6\tfm2_ai_adjust.zip` **5,007,638B·14엔트리·v1.6.1**·deps `>=0.5.6, <0.5.7`·zip내 dll=라이브(3,664,896B @05:47:10)로 교체·rel_verify OK·생성=rel_one.py(0.5.5 기준 zip 자산 유지·dll/mod_info만 교체)·추가 커밋 없음(zip 생성 시점 소스 무변경·rel_commit "변경 없음" 확인·대응 소스 커밋=`150cb05`) ⟹ ★**tfm2_ai_adjust 0.5.6 = 마이그·배포·인게임 검증(2판)·릴리스·커밋 전부 완결·잔여 없음**(별건 제외: ①CrashDumps 10건 트리아지(칩 등록) ②ex_order_hold/ex_think_min 재설계 RE ③disc19 shadow-call 게이팅 감사 ④nx_cull_dist19 완전 개통 — 전부 0.5.6 마이그 무관 선택 개선). ★부수 판정(08-22) = **nx_imm applied=6/8 = 결함 아님·설계상 최대치**(등재 8 중 pskip 2는 0.5.4부터 의도적 미패치) · **nx_cull_dist19 = 0.5.4부터 3곳 중 1곳 부분 적용**(완전 개통 = 별건 재조사감). 상세 정본 = 위 1단계 RE + `RE\2026-08-20_0.5.6-2단계-소스재핀.md`(2단계) + `02_구현정보.md` "0.5.6 마이그" 절 + `03_시행착오.md` 08-20·08-22 절.

tfm2_champion_exclude (0.5.6·08-20 밤 재핀·재빌드·배포완·⬜인게임 미검증 = 훅 설치+패치데이 발화 — 0.5.5에서도 미검증): HOOK_RVA(후보 Vec 생성) 0x186e150->0x1894610(스켈레톤 UNIQUE·size 0x14b 동일·프롤로그 17B 완전 동일 ⟹ HOOK_ORIG/ORIG_LEN 불변). 앵커(코드 미사용·doc): day-proceed 0x1e34a00->0x2109760 · 패치데이 0x203acc0->0x2371820 · 신챔프추가 본체 0x202c440->0x2363ee0(유일 콜사이트 0x202c5f2->0x236406e — 0.5.6에도 콜러 1곳 재확인) · 렌더러 0x249f6f0->0x19fec50 / 0x24ca500->0x2220c30 · 초기풀 0xc2d980->NO MATCH(미추적·코드 미사용). 배포 dll 163,840B @2026-08-20 20:56:42 · deps >=0.5.6, <0.5.7 · 커밋 tfm2mods 0da8885. (0.5.5 값·메커니즘 = [[tfm2-champion-exclude-mod]] + REPORT\tfm2_champion_exclude\02_구현정보.md — 본 절이 0.5.6 RVA 정본.)

tfm2_bancard_keep (0.5.6·08-20 밤 재핀·재빌드·배포완·⬜인게임 미검증 = (A)클램프/(B)커밋누락 판별 대기 — 0.5.5에서도 미검증): PATCH_RVA 0x21d11d2->0x20939fa · 새니타이저 0x21d0ad0->0x2093310(스켈레톤 NO MATCH -> orig 11B 패턴 전수 스캔 + 클램프 블록 디스어셈 1:1 대조로 확정 · orig 11B 동일 = PATCH_ORIG 불변 · 챔피언풀 disp 0x165c8->0x166e8 확인). 배포 dll 143,872B @2026-08-20 20:56:50 · deps >=0.5.6, <0.5.7 · 커밋 ba6ec66. (구현·0.5.5 값 = §7.5 §10 = 이력 — 본 절이 0.5.6 RVA 정본.)

Spectator_Chat · community_reaction_mod (소스 수정 불요·SDK 재빌드만): exe RVA 하드코딩 없음. ClientDatabase 저대역 오프셋(LIVE_PLAYED_OFF 5528=0x1598·LIVE_EVENTS_OFF 5744=0x1670·scene 0x1338) 0.5.6 불변(동일오프셋 직독 0x1598 70/0x1670 60/0x1338 64 = 시프트 없음) -> 재빌드만으로 정상.

RVA 0(SDK 재빌드만): tfm2_mod_order·meta_item_delegate(워크샵 content 주의)·save_probe·daram2 view_plus 8종·tfm2_html_overlay(stable API v0.7.0 = 재빌드 불요 예상·deps 게이트만 점검)·legacy_save_patcher(stable·불요).

### 4. 잔여 (0.5.6)
- ~~전 모드~~ → **잔여 모드** 인게임 미검증(정적 재핀·프롤로그·orig 대조만 완료 — **08-20 저녁 검증완 = item_tactics 전면(4칸 UI·게이트·주입 write 27·크래시 0 = §3 최종 정정)·banpick_order 이어하기 무크래시(유저 확인 = HL 스텁 수정 유효)**). 우선 관측 = banpick_order AITURN/SFX 스텁 재핀(레지스터/슬롯 실변경 — ⚠**같은 축(스텁 disp)에서 HL이 실제로 이어하기 즉사 AV를 냈음(08-20 오후·§3 banpick_order 정정 절)**: HL 계열은 수정·재배포·이어하기 무크래시 확인 완이나 AITURN/SFX 스텁 disp도 동일 부류 위험 = 우선 관측 유지·밴픽 하이라이트 3종·훅 M 흰칸도 후속 관측) · item_tactics 잔여 관측 = BG4/TN 프레임 · comptest 바이트패치+병렬발사 · ~~serpen 색/버프/장로~~ → ✅**serpen = 08-20 밤 검증완**(화면경기 기능 복구·유저 확인 = §3 serpen 정정 — riot 런처 CALL식 후킹發 retaddr 오염이 원인이었음). ⚠deploy-verify FAIL 0은 이 축(스텁이 재현하는 rbp-disp32 바이트)을 못 잡음 — 정적 검증 한계 실증.
- ★**버전업 체크리스트 — migrate_rva/스켈레톤이 원리적으로 못 잡는 축 3형제**(각 실사고 1건·다음 마이그 때 명시 점검): ①**콜러 프레임 오프셋**(TN_FR_* — 0.5.5 item_tactics TN 침묵 전멸) ②**mid-func 스텁 하드코딩 rbp-disp32**(0.5.6 banpick_order 이어하기 즉사 AV) ③**버전 게이트 상수(exe 크기 등)**(0.5.6 item_tactics 전체 자기비활성 — §3 정정·exe-size 게이트 보유 = item_tactics 유일). 셋 다 정적 검증(deploy-verify·orig 대조) 미커버 = DONE.md 버전무관 행.
- ★**버전업 체크리스트 추가 축(08-20 저녁 실사고) — 워크샵 서드파티 모드 훅 충돌**: riot_items_tfm2(자동 업데이트)가 BUY_ITEM(0xebca20)·SEEDCTOR·LAUNCHER 선점 후킹 → replace형 설치기도 체인 분기 필수(item_tactics `install_replace_buy` 체인 구현완 = §3 최종 정정). + ★**두 번째 얼굴(08-20 밤·serpen)**: riot 런처 훅은 **CALL식**이라 안쪽에 체인되면 **retaddr 오염**(설치는 성공해도 retaddr 판정 전멸) ⟹ **retaddr 의존 훅 = 지연 설치+체인(바깥쪽)이 기본값**(serpen 해결·검증완 = §3 serpen 정정). **다음 마이그 때 riot 업데이트 시점·훅 충돌 항상 점검**(마이그 후 이상 시 "타 모드 신규 후킹" 용의선상) = DONE.md 맨위 행.
- ghidra-re 필요(NO MATCH·본문변경) = banpick_order AI6 컨테이너 2종(0x1cb8640 ai_reco1/2·0x1cbb1a0 ai_bb1/2 = 밴픽 AI 개선 로직 변경·sig/arm 재작성) — 단 cfg OFF·미설치라 빌드/기능 무영향. 그 외 활성 훅은 전부 재핀 완료.
- 빌드는 메인 세션(SDK 0.5.6 다운로드 중·본 세션은 상수 Edit만).

---

## §7.7 · 게임 0.5.7 마이그레이션 — 패치 성격 · exe↔exe 재핀 (2026-08-26, 0.5.7) — 본 절 = 0.5.7의 정본

> ⚠ 본 절 위 §7.6(0.5.6)·이하는 이력. 0.5.7 이후 작업은 이 절만 볼 것.
> ⚠ **본 절 = 정적 재핀 결과까지**. 빌드·배포·인게임 검증은 아래 §4 잔여 참조.

### 1. 버전 사실 (실측)
| 항목 | 0.5.6 (직전) | 0.5.7 (현행) |
|---|---|---|
| exe | 77,101,056B | **77,111,808B** (+10,752B) |
| sha256[:16] | A0D8E395581FDF5A | **5969A222B23EA0F1** |
| .text raw | 52,856,832B (0x3268800) | **52,852,736B (0x3267800)** (−4,096B) |
| 함수 수(.pdata) | 136,086 | **136,233** (+147) |
| 인덱스 pkl | _fnidx_056.pkl | **_fnidx_057.pkl** |

- 백업: OLD=`tfm2_0.5.6\TeamfightManager2.exe` / NEW=`tfm2_0.5.7\TeamfightManager2.exe`(라이브와 해시 동일 확인).
- 패치 성격 = **0.5.6보다도 온건**(.text 사실상 무변화). 단 **코드 배치는 전면 재링크**(스켈레톤 UNIQUE계 중 RVA 제자리 725 / 이동 37,001) ⟹ RVA는 전량 교체 필요.
- ★**단, AI 판단 계층은 0.5.6보다 큰 변경**: ai_adjust 훅 5종이 본문 변경(크기 변화) = 패치노트 "일부 챔피언 스킬 효과가 선수 AI의 스킬 사용 판단에 미반영" 수정의 실체. 상세 = §3 ai_adjust 절.
- 재핀 도구 = `_mig057.py`(스켈레톤해시, _mig056.py 답습) + `_t057g.py`(정규식 마스크시그) + `_t057i.py`(★신규 **콜러그래프 슬롯정렬 재핀** — 본문변경 함수용) + `_t057j.py`(LCS 콜슬롯 정렬) + `_t057k.py`(프롤로그·크기 대조). 결과 JSON = `_rva057.json`.

### 2. SDK · toolchain
- **SDK = `C:\tfm2mods\sdk_057\mod-sdk`** (GitHub 릴리스 `0.5.7.zip` **556,164,491B** · sha256 `e12351c55e5ad081c7080a9f92095a6b724bb1e051cd1d0dee749478020a3d29`).
- rlib 154개 **파일명 전원 동일(StableCrateId 무변경 — 0.5.6과 같은 패턴)**, 내용 DIFF = 핵심 6종 전부(mod_api·game_core·game_ai·game_view·engine_ui·engine_core) ⟹ **전 모드 재빌드 필요**.
- **toolchain = `nightly-2026-05-24` 유지**(`rustc 1.98.0-nightly (23a3312d9 2026-05-23)` — 0.5.6과 동일 문자열) ⟹ 재설치 불요.
- ⬜빌드스크립트 6종 `sdk_056`→`sdk_057` 전환 필요(build_inj·build_full·build_full_remap·build_extra·banpick_illust\build·dashboard_probe\build).

### 3. 재핀 표 (구 0.5.6 → 신 0.5.7) — **정적 재핀 완료 · 소스 반영은 미실시**

**공통 4심볼**: PARSER `0x19ab40`→**`0x1ab310`**(UNIQUE·BYTE=SAME) · ALLOC/GAME_ALLOC `0x2ab1670`→**`0x2ab4010`**(BYTE=SAME) · LOADER `0x2e6f60`→**`0x2ea930`**(string-xref `asset/base/ui/layout/main` x17 = 0.5.6과 동일 카운트) · ANIM_GET `0xbea4a0`→**`0x74bf10`**(CARD_DRAW 콜슬롯14).

tfm2_flow_capture: RUN_TICK `0x14db7e0`→**`0x106bae0`**(SAME) · CTOR/LAUNCHER `0x14dda60`→**`0x106dd60`**(HEAD_UNIQUE·⚠size 4398→4369·프롤로그 공통 13B) · SCENE_STEP `0x24d1dc0`→**`0x1e748a0`**(마스크시그 UNIQUE·프롤로그 20B 동일) · RECO `0x2ce38f0`→**`0x2ce6310`**(size 동일).

tfm2_banpick_illust (16 전건 확정): FX_SET `0x1e598b0` · CARD_DRAW `0x1e6f350` · ILLUST_GET `0x226f290` · SUBMIT `0x182430` · SUBMIT_TEXT `0x1826b0` · **IMG_BUILD `0x183b50`**(스켈레톤 NONE → CARD_DRAW 콜슬롯 27:27 완전대응으로 확정·⚠size 614→633·프롤로그 공통 9B) · IMG_UV `0x1839b0` · IMG_FLAG `0x183e90` · IMG_COLOR `0x1eeee0` · IMG_SHADER `0x185490` · TEXT_BUILD `0x183040` · NAME_GET `0x1e8d1e0` · **ASSET_GET `0x143de0`**(ILLUST_GET 콜슬롯1 @+0x74 동일오프셋) · ANIM_GET `0x74bf10` · SPRITE_CALC `0x1e92110` · GAME_ALLOC `0x2ab4010`. ⬜geom .rdata 6 · mid 6 · SLOTS 미재핀.

tfm2_item_tactics: FN_DD `0x1c1af0`→**`0x1c31f0`** · **TIP_SHOW `0x1efca70`→`0x1b1d500`**(head-UNIQUE + **콜러 3개 동수 교차검증**·UImega 콜사이트 `0x854399`·⚠size 10383→10010) · GV_UPDATE `0x90a090` · REALLOC `0x2a9fb50` · CL_LAUNCHER `0x106dd60` · SEEDCTOR `0x1635ae0` · BUY_ITEM `0xdf5490` · ITEMNET_FWD `0x17f09b0`. ⬜TIP_MEASURE_VT(.rdata)·바이트패치 4·launcher retaddr 4·TN 프레임슬롯 미재핀. ★★**GAME_EXE_SIZE 게이트 = `77_111_808`로 갱신 필수**(`src\lib.rs:5102` `GAME_EXE_SIZE_056`) — 미갱신 시 0.5.6 실사고 재현(모드 전체 침묵 자기비활성).

tfm2_elemental_serpen: SERPEN `0x12ce9b0`→**`0x14a25f0`**(⚠⚠size 5245→**4587**, −658B = 본문 실변경·프롤로그 공통 15B) · MOBATICK `0x10af580`(size 동일) · RUNNER_CTOR `0x106f9c0`(⚠size 3613→3646) · DMGA `0x15912c0` · DMGB `0x108f220` · KEYRES `0x238d6e0` · **ARG_STR `0x14cd960`**(콜러 316 ≈ 0.5.6의 313으로 2후보 중 확정) · SPAWN0 `0x89c440` · SPAWN1 `0x89b830` · LAUNCHER=`0x106dd60`. ⬜retaddr 4 미재핀.

tfm2_banpick_order (함수시작 15 확정): PHASE_SCENE=SCENE_STEP `0x1e748a0` · PHASE_SCALAR `0x163e2c0`(⚠프롤로그 공통 9B) · APPLIER `0x1e59680` · SLOTUPD `0x1e98ed0` · PHASE_RAW `0x1e4f2d0` · **APP_PICK_T1 `0x1e458d0` / T2 `0x1e45a60` / APP_BAN_T1 `0x1e83360` / T2 `0x1e834e0`**(쌍둥이 4종 = 스켈레톤 MULTI(2) → **콜러 수 3 vs 2**로 확정, 0.5.6과 동일 분포) · TRANSITION `0x1e4fa70` · BANNER `0x1e56e00` · LINEUP `0x1e46390` · COMMIT `0x1642600` · TURN `0x1642d30` · TRIGGER `0x1e782a0`. ⬜mid-func 스텁 전량(AI_SITE/JOIN·AITURN·SFX·DRAIN_HL·HL·SLOTSEL·훅M) 미재핀 — ⚠**0.5.6 즉사 AV 축(rbp-disp32)**이라 disp 바이트 전수 대조 필수.

tfm2_comptest_unlock (21 확정): RUN `0x1ad4410` · CGATE `0x1ac7050` · SREG `0x23339f0` · RESULT `0x1b182e0` · CSEND `0x1ac2e80` · HPUSH `0x17ae140` · RPLY2 `0x1adcbd0` · RPLY3 `0x1ad9e50` · LIVEB `0x1b18f90` · CTX_CLONE `0x1dec840` · ARRIVE `0x1add430` · WARN `0x1aad370` · REFRESH `0x1abc4f0` · ITEMCONV `0x2345c00` · COLLECT `0x1ad5d70` · RUST_ALLOC `0x29f00e0` · **CLONE_CHAMP `0x1a3e450`**(스켈레톤/마스크시그 NONE → **콜러그래프 17표**로 확정) · FN_DD `0x1c1300` · DROP_CHAMP `0x13557a0` · INSERT `0xca7c80` · ATH_GET_SC `0x12bdcf0` · ORACLE=RUN_TICK `0x106bae0`. **미해결 3(전부 0.5.6에서 inert 분류)** = SIMBODY(마스크시그 붕괴) · DRIVE `0x888d20`(E8 콜러 0 = 간접호출) · DEDUP_INS(3후보). ⬜바이트패치 19·RUNNER_VT 미재핀.

tfm2_champ_pos_lock: POS_MASK `0xf83830`→**`0x181fe60`**(size 동일) · contains `0x8e85f0` · slot_widget `0x1e79ff0`(⚠size 30954→30971) · scene_step `0x1e748a0` · 공용3. ⬜contains 콜사이트 2 미재핀.

tfm2_champion_exclude: HOOK_RVA `0x1894610`→**`0x188a660`**(UNIQUE·BYTE=SAME·size 331 동일 ⟹ HOOK_ORIG/ORIG_LEN 불변).

★**tfm2_ai_adjust — 0.5.7의 핵심 변경 지점**: RVA-only 4 = GENERIC_BUILD `0xccae00`→**`0xceb5f0`**(BYTE=SAME) · FC59A0 `0xd01950`→**`0xe61600`**(SAME) · MOVEPRI `0xe23ad0`→**`0xdb2760`**(SAME) · AUCTION `0xe0e5e0`→**`0xe8b800`**(size 동일) · ITEMNET=`0x17f09b0`.
**★본문 변경 5종(= 패치노트 AI 스킬판단 수정)**:
  · **RETREAT** `0xcead40`→**`0xe4a750`**(스켈레톤/마스크시그 NONE → **콜러그래프 확정**·호스트 `0xe23f00`→`0xdb2b90`·size 11030→**10995**)
  · **CONDGATE** `0xe23190`→**`0xdb1e20`**(NONE → **LCS 콜슬롯 정렬**로 확정 = 호스트 `0xdf4ce0`→`0xe723c0` 내 2사이트(슬롯 66·178) 모두 동일 결과 + **MOVEPRI와의 간격 `0x940`이 0.5.6과 동일**하게 보존 = 교차검증. size 513→**522**)
  · **SUBPLAN** `0xe55240`→**`0xcbf340`**(HEAD_UNIQUE·size 1310→**1278**)
  · **DISC18** `0xd32b70`→**`0xe9fd70`**(마스크시그 + 콜러그래프 **2방법 일치**·size 5891→**5934**)
  · **DISC19** `0xd426e0`→**`0xeae620`**(마스크시그 + 콜러그래프 **2방법 일치**·size 11221→**11292**)
  ★**CONDGATE 실변경 내용(디스어셈 1:1 대조 실측)** = ①레지스터 재할당(rsi↔rdi 스왑) ②인자 로드 `mov rsi,[rsp+0x98]`를 **프롤로그로 호이스팅** ③disc 분기 1곳에 `add rcx,8` + 인자 1개(`[rsp+0x30]`) 추가 후 다른 callee 호출. **구조체 오프셋(0x930/0x9c0/0x1e0/0x670/0x628/0x1d8/0x1d0)·JT 디스패치 골격은 전부 동일** ⟹ 로직 대개편 아님·**국소 변경**.
  ★**orig_len 영향**: CONDGATE 신 프롤로그 = `push rsi/rdi/rbx`+`sub rsp,0x40`(7B) + `mov rsi,[rsp+0x98]`(8B) = **정확히 15B 경계** ⟹ 기존 `orig_len 15` 그대로 유효(우연히 경계 일치 — ⚠반드시 재확인 후 사용).
  ⬜**미실시 = 바이트패치 908엔트리 재핀 · JT 베이스(Plan/SubPlan) 재핀 · 재현 코드(완전재구현) 정합성 검토**.

### 4. ★배포 완료 — RVA 0 모드 **14종** (2026-08-26 20:02~20:10 · 유저 지시로 RVA 0 우선)

전부 **sdk_057 재빌드 → 배포 → deps `>=0.5.7, <0.5.8` 갱신**. 배포 dll Length+mtime 확인 완료(CLAUDE.md §10 증거 규칙 충족) · mod_info 전건 BOM 없음(firstByte 0x7b)·jsonOk.

| 모드 | dll | 비고 |
|---|---|---|
| coaching_staff_view_plus | 284,160B | build_full_remap |
| custom_tier_assignment | 2,688,512B | build_full_remap |
| facility_view_plus | 289,280B | ⚠**`-SkipIdentity` 필요**(panic 경로 미기재 모드) |
| finance_view_plus | 147,456B | build_full_remap |
| recruitment_view_plus | 333,824B | build_full_remap |
| roster_view_plus | 433,664B | build_full_remap |
| statistics_view_plus | 2,717,184B | build_full_remap |
| training_view_plus | 2,612,736B | build_full_remap |
| Spectator_Chat | 333,312B | build_inj(소스 hex 6개 = 전부 주석 속 참고 RVA·코드 미사용 ⟹ RVA 0 확정) |
| tfm2_mod_order | 203,776B | build_inj |
| tfm2_meta_champion_tiers | 229,888B | ⚠build_inj 신원검증 실패(panic 경로 미기재) → **PII 2종 검사 후 수동 Copy-Item** |
| tfm2_ai_banpick_probe | 251,904B | build_inj(TFM2.gg-upstream
ative) |
| pts_ui_dump | 174,080B | build_inj |
| player_trade_system_test79_stable_pending_ui | 1,242,112B | build_inj(hex 2 = 해시상수·마스크, RVA 아님) |

- ⬜**인게임 미검증 전량**(정적 배포까지만).
- ★**미복구 14종 = 0.5.6 대역 유지 = 0.5.7에서 자동 비활성(안전 상태)**: tfm2_ai_adjust(★유저 지시로 이번 세션 보류) · tfm2_item_tactics · tfm2_banpick_illust · tfm2_draft_overlay · tfm2_elemental_serpen · tfm2_banpick_order · tfm2_comptest_unlock · tfm2_champ_pos_lock · tfm2_champion_exclude · tfm2_bancard_keep · tfm2_level_cap · tfm2_flow_capture · sylas · TFM2_Meta_Dashboard(save_probe 번들). **함수시작 훅 재핀은 위 §3에 완료돼 있음** — 소스 반영·빌드만 남음.
- ★**빌드 스크립트 6종 sdk_057 전환 완료**(build_inj L44·build_full L18·build_full_remap L25·build_extra L26·banpick_illustuild L10·dashboard_probeuild L4 — 각 `.bak057` 백업). **deps 헬퍼 신규 = `bump_deps_057.ps1`**(0.5.6판 기반·ASCII 유지·`$OLDRX`=0.5.6대역 정규식·`$NEWDEP`='>=0.5.7, <0.5.8').
- ★**`-SkipIdentity`는 PII 검사를 끄지 않는다**(build_full_remap L81~96 실측: ①PII 검사는 무조건 수행 / ②신원검증만 생략) ⟹ panic 경로 미기재 모드에 안전하게 사용 가능.

### 4b. ★2차 배포 — T1 활성 모드 **6/7** (2026-08-26 20:38~20:55 · 유저 지시 "T1도 이어서")

| 모드 | dll | 이번 회차에 새로 확정한 축 |
|---|---|---|
| tfm2_draft_overlay | 684,032B | 공통 4상수만(ANIM_GET·LOADER·PARSER·ALLOC) |
| community_reaction_mod | 620,544B | RVA 0(소스 hex 4개 = 전부 주석 속 참고값) · ★**실 로드처 = 워크샵 `contentÀ9300û8958482`** 에 배포(게임 mods\는 출력 전용) |
| tfm2_elemental_serpen | 423,936B | 함수 12 + SPAWN_HOOKS 2 + **launcher retaddr 4** |
| tfm2_banpick_illust | 2,894,336B | 함수 16 + **geom .rdata 6** + **mid 6** (SLOTS는 0.5.6값 유지) |
| tfm2_item_tactics | 607,232B | 함수 9 + TIP_MEASURE_VT + **바이트패치 4** + **TN 프레임 3** + RA_TOURN + ★게이트/프롤로그 |
| Spectator_Chat | 333,312B | (1차에서 완료) |

**tfm2_ai_adjust = 유저 지시로 보류**(0.5.6 대역 유지 = 자동 비활성). 사유·재핀값 = §3 ai_adjust 절.

#### 4b.1 새로 확정한 값 (구 0.5.6 → 신 0.5.7)
- **launcher retaddr**(serpen·item_tactics 공유): LAUNCHER 콜사이트 **9:9 전단사** 확인 후 owner별 오프셋 대응 —
  RET_A(관전) `0x8404e1`→**`0xb19ca9`** · RET_B(내경기) `0x84544b`→**`0xb1ebfb`** (둘 다 씬빌더 `0x8343a0`→**`0xb0d970`** 내 · ⚠본문 +826B로 콜 오프셋 이동 `+0xc13c`→`+0xc334` / `+0x110a6`→`+0x11286`) ·
  RET_C(리플레이) `0x1db3884`→**`0x1a03cd4`**(owner +0x97f 동일) · RET_D(조테기록) `0x1ac1b2e`→**`0x1ada3ae`**(+0x559 동일) · IT 조테본경기 `0x1af18a2`→**`0x1b193f2`**(+0x45d 동일) · **RA_TOURN** `0x1f24068`→**`0x1da7a65`**(worker `0x1f16ea0`→**`0x1d9a210`**).
- **banpick_illust geom(.rdata)** — 블록 통짜 매칭은 실패(0.5.7에서 앞쪽 float 추가로 뒤가 밀림), **값 지문으로 개별 확정**: C_CARD_RECT `0x34cd6a0`→**`0x348a740`**(`{-180,-240,360,480}` .rdata 유일) · C_SNAP_RECT →**`0x348a770`** · C_NORMAL →**`0x348a790`** · C_LINE_DIR →**`0x348a7b0`** · C_LINE_START →**`0x348a7c0`** · C_LINE_ANCHOR →**`0x348a7d0`**. (0.5.6 블록 내 오프셋 +0x20/+0x30/+0x50/+0x60/+0x70 → 0.5.7 +0x30/+0x50/+0x70/+0x80/+0x90.)
- **banpick_illust mid 6** — 컨테이너 델타 전건 성공(오프셋 동일): I_SNAP_H `0x254d7c0`→**`0x1ed5830`**(imm 480.0 구·신 일치) · D_SNAP_W →**`0x1ed5846`** · D_CUT_LO →**`0x1e794b8`** · D_CUT_HI →**`0x1e794c6`** · D_ZIG_X1 →**`0x1ed6692`** · D_ZIG_X2 →**`0x1ed6d70`**. **disp4가 가리키는 float를 전건 실측 검증**(360/-70/70/-180/-180 구·신 동일). 컨테이너 `0x254cf40`→`0x1ed4fb0`, `0x24d6a80`→`0x1e79430`(둘 다 UNIQUE).
- **SLOTS** `0x40e6000` = **0.5.7에서도 all-zero 64B 확인 ⟹ 값 유지**.
- **item_tactics TIP_MEASURE_VT** `0x334cd10`→**`0x3333ec8`** — UImega tipshow 콜사이트 `0xab5339`→**`0x854399`** 직전 `lea r8,[rip+..]` @ **call−0x18**(0.5.6과 동일 오프셋·창 내 유일). 0.5.6 값으로 방법 검산 통과.
- **item_tactics 바이트패치**: gate3 `0xebcd88`→**`0xdf57f8`** / jbe →**`0xdf57fe`**(resolver `0xebcb10`→`0xdf5580` UNIQUE·off +0x278 동일·orig `48837c24600276` 구·신 일치) · ★**owned_cap 은 컨테이너 델타 실패**(owner HEAD_UNIQUE지만 본문 변경으로 오프셋 밀림) → **`cmp qword[rsi+0x4a8],3` 바이트 유일검색**으로 확정: sig `0x154c679`→**`0x10da9a9`** / imm →**`0x10da9b0`**(구·신 .text 전체 **각각 1건**).
- ★**item_tactics TN 프레임 오프셋 3**(= 3형제 축 ①): TN_FR_DB `0x23c18`→**`0x23e48`** · TN_FR_CFG `0x23c00`→**`0x23e28`**(worker 내 사이트 오프셋 `+0xb9` 구·신 동일) · TN_FR_SETEND `0x23b20`→**`0x23d68`**. 각 명령 시그(pre+wild disp4+post)로 구·신 **각각 유일** 확인. ⚠**슬롯 간 상대 간격이 바뀜**(구 DB−CFG=0x18/DB−SETEND=0xf8 → 신 0x20/0xe0) ⟹ 델타 일괄 적용 금지·개별 재핀이 정답.
- ★**item_tactics 버전 게이트**(3형제 축 ③): `GAME_EXE_SIZE_056` **77_101_056 → 77_111_808**. 미갱신 시 0.5.6 실사고(모드 전체 침묵 자기비활성) 재현.
- ★**CL_LAUNCHER_PROLOGUE 변경**(17B 배열): chkstk imm **`0x25438` → `0x25418`**(index13 `0x38`→`0x18`). 프롤로그 6종 중 **이것만 변경**(FN_DD 12B·SEEDCTOR 12B·GV_UPDATE 12B·BUY_ITEM 19B·REALLOC 12B는 구·신 바이트 동일). serpen 프롤로그(SERPEN·MOBATICK·LAUNCHER·RENDER_STEP·RUNNER_CTOR·SPAWN0/1 8push 12B, KEYRES 14B)는 **전건 구==신 동일 ⟹ 무수정**.
- **serpen SPAWN_HOOKS** `[0xaf97d0, 0xaf8bc0]` → **`[0x89c440, 0x89b830]`**.

#### 4b.2 워크샵 서드파티 훅 충돌 점검 결과 (0.5.6 실사고 축)
- **`riot_items_tfm2` v0.9.2 deps = `=0.5.6`(정확히 고정)** ⟹ **0.5.7에서 자동 비활성 = 이번 회차엔 충돌 없음**. dll mtime 08-20 14:35(0.5.6판 그대로·0.5.7판 미출시). item_tactics `install_replace_buy`·serpen 지연체인은 "외부훅 없으면 직접 설치" 폴백이 있어 정상 동작 예상(⬜인게임 확인). ⚠**제작자가 0.5.7판을 올리면 충돌 축이 되살아난다** — 이상 발생 시 1순위 용의선상.
- ⚠**신규 발견 — deps 상한 없는 우리 워크샵 모드**: `tfm2_meta_champion_tiers`(3738236964, `>=0.5.1`) · `tfm2_meta_item_delegate`(3738241856, `>=0.5.1`) · `tfm2_ai_banpick_probe`(3738236728, `>=0.5.1`) · `community_reaction_mod`(3738958482, `>=0.1.0`) ⟹ **0.5.7에서도 구 dll이 그대로 로드된다**(자동 비활성 안 걸림 = `_공통_빌드릴리스_교훈.md §8` 축). 이번 회차엔 community_reaction_mod만 워크샵 경로에 재배포함. 나머지 3종은 **게임 `mods\` 에만 갱신**돼 있어 실 로드처와 어긋날 수 있음 ⟹ ⬜확인 필요. 또한 같은 mod_id가 워크샵에 **두 벌**씩 존재(3738xxxxxx = 08-20판 / 3999000xxx = 06-25판).

### 4c. ★3차 배포 — 잔여 9종 (2026-08-26 21:10~21:35 · 유저 지시 "나머지도 복구")

| 모드 | dll | 비고 |
|---|---|---|
| tfm2_flow_capture | 451,072B | 함수 4 + 공통 3 |
| tfm2_champion_exclude | 2,867,712B | ⚠1MB 초과 → `build_extra.ps1 -MaxSize` |
| tfm2_level_cap | 198,144B | mid 2(orig `498b96100d0000`·`483b88100d0000` 일치) |
| tfm2_bancard_keep | 143,872B | mid 1(PATCH_ORIG 11B 구·신 동일) |
| tfm2_champ_pos_lock | 3,183,104B | 함수 11 + contains 콜사이트 2 + 공용 3 · ⚠`-Externs` 필요 |
| tfm2_comptest_unlock | 286,720B | 함수 22 + **바이트패치 17/17** + 영역 경계 4 |
| sylas | 489,472B | 함수 19(EMIT 포함) |
| tfm2_ai_adjust | 3,676,160B | ★훅 10 신주소(`rva_057.rs` 신설) — 아래 4c.2 |
| TFM2_Meta_Dashboard | 305,152B | RVA 0 · dashboard_probeuild.ps1 |

**⛔미복구 = `tfm2_banpick_order` 1종**(안전 판단으로 의도적 보류 — 4c.3).

#### 4c.1 새로 확정한 값
- **champ_pos_lock**: POS_MASK `0x181fe60` · contains `0x8e85f0` · slot_widget `0x1e79ff0` · scene_step `0x1e748a0` · ICON_SETTER `0x1e89450` · ENTITY_ICON `0x1e92110` · PICK_DISPATCH `0x1e45a60` · COMMIT `0x1642600` · RECOMMEND `0x25549d0` · FINALIZE `0x1db6f30` · RECOMMEND_WBC `0x2556220` · **contains 콜사이트 `0x1e8016f`/`0x1e80188`**(slot_widget 내 off `+0x617f`/`+0x6198`, 구 `+0x616e`/`+0x6187` — widget size +17만큼 이동). ⚠**RVA_DISPATCH `0x2079730` = 재핀 불가**(skel/head 후보 0·마스크시그 0건 = 함수 대개편) ⟹ **0.5.6 값 stale 유지**. `build_tramp`가 `PROL_RECOMMEND` 프롤로그를 검증하므로 **불일치 시 미설치 = fail-safe**(코치 위임 관측·사후교정 기능만 죽음).
- **level_cap**: LEN_LOAD `0x1090fba` · UI_CMP `0x9035e9`(둘 다 마스크시그 UNIQUE + orig 7B 구·신 일치).
- **bancard_keep**: PATCH `0x234509a`(orig 11B 구·신 동일) · 새니타이저 `0x23449b0`(주석 참조만·코드 미사용).
- **champion_exclude**: HOOK `0x188a660`(BYTE=SAME·size 331 ⟹ HOOK_ORIG/ORIG_LEN 불변) · ICON_SETTER `0x1e89450` · 공통 3.
- **sylas**(19): GRAB `0x1504310` · COMBINE `0x159a430` · JT `0x161b390` · BASEULT `0x1556940` · CLONE `0x14f43c0` · ETICK `0x1583de0` · ADDBUFF `0x139a5d0` · BUFFPUSH `0x15925c0` · ASSEMBLE `0x1590010` · TAGSEL `0x23964e0` · KZONE `0x138f7d0` · CVIEW `0x104f060` · CVIEW_APPLY `0x1413830` · VIEWLOOP `0x8fb670` · VIEWFAIL `0x8fe4d8` · VIEWLOOKUP `0x8fe2d5` · AIEVAL `0xeba9d0` · ALLOC `0x2ab4010` · **EMIT `0x10556d0`**(스켈레톤 MULTI 4후보 → 콜러 owner의 콜슬롯 3/8 정렬로 확정). ⬜**EFF_VT_BASE `0x34200d8` 미재핀**(.rdata vtable 표 — 내용에 포인터가 있어 값 지문 매칭 0건). **안전 확인**: `rd_u64` 안전읽기 + `in_exe()` 검증 + **최빈값이 과반 미달이면 스스로 포기**하는 설계라 stale이어도 무해(`default_eff_stubs`).
- ★**comptest_unlock 바이트패치 17/17**: 클라 13건은 마스크시그(back 0x00~0x20·sig 0x50)로 orig 일치 확정. **서버 4건**(no_stamina_cost·daily_inc_gate·server_pregate·server_dedup)은 전부 같은 대형 서버 핸들러 `0x2031a00`(167,910B) 소속인데 그 함수가 skel/head/마스크시그 전부 NONE → ★**콜러그래프 투표(1표)와 "크기 근접 >100KB 유일 후보"가 둘 다 `0x22e2e70`(167,319B)를 가리켜 확정** → 명령 difflib 정렬(커버리지 85%)로 4건 전건 **명령 동형 + orig 1B 일치**: `0x22f9e6b` · `0x22f5619` · `0x22f2844` · `0x22f162e`. 영역 경계도 갱신(CT_REGION_LO `0x22e2e70` / HI `0x230bc07` / CT_CLIENT_LO `0x1a90000` / HI `0x1b40000` — 신 클라 사이트 최대 `0x1b36444`가 구 HI `0x1b20000`를 넘어섰다).

#### 4c.2 ★tfm2_ai_adjust — 훅만 복구, 바이트패치는 의도적으로 미복구 (실측 근거 포함)
- **훅 10 = 신주소**(`src
va_057.rs` 신설·`tfm2_ai_adjust.rs` include 전환). 값 = §3 ai_adjust 절.
- ⚠**바이트패치 908·class_micro 18·JT 베이스는 0.5.6 값 그대로**다. 이유는 아래.
- ★★**실측으로 밝혀낸 위험 1건**: 0.5.6 `EXPECT_ORIG` 표를 그대로 두고 0.5.7에서 같은 주소의 값을 읽어보면 **907건은 불일치(=`orig_guard_ok`가 blocked 처리 = fail-safe)이나 `0xe23be8` 1건만 우연히 expect와 일치**한다 ⟹ 가드를 통과해 **엉뚱한 코드를 패치**한다. 그 사이트는 MOVEPRI(`0xe23ad0`→`0xdb2760`) 내부 `+0x118`이라 재핀값이 `0xdb2878`. **`detour.rs` L2632의 그 1건만 신주소로 교체**해 위험을 제거했다.
- ★**표를 통째로 신주소로 바꾸면 오히려 위험해진다**(시도 후 되돌림): `orig_guard_ok`는 **표에 없는 사이트를 통과시킨다**(`detour.rs` L705). 패치 사이트 주소는 `detour.rs`의 `p!(base + 0x…)` 매크로에 하드코딩돼 있는데, 908 중 `base + 0x` 패턴으로 잡히는 건 **414건뿐**이라 표만 옮기면 나머지가 "표에 없음 → 무가드 패치"가 된다. ⟹ **표는 0.5.6 그대로 두는 것이 정답**(가드가 907건을 막아준다).
- 참고로 사이트 재핀 자체는 **665/908(73%)** 성공했다(`_t058e.py` = 컨테이너 델타 + 명령 difflib, 명령 동형 + 신 exe 실측값 == expect 까지 확인). 매핑은 `_ai057_sites.json`에 보존 — **다음 세션이 사이트·표를 동시에 갈아끼우면** 665건을 살릴 수 있다. 미재핀 243건은 0.5.7에서 **우연 일치 0건**이라 표에 남겨두면 전원 blocked.
- ⟹ **현재 배포본의 실제 상태** = 재현 디투어 훅 10은 신주소로 정상, **바이트패치 노브는 거의 전부 비활성(blocked)**, 크래시 위험 축은 제거됨. ⬜인게임 검증 필요.

#### 4c.3 ⛔tfm2_banpick_order — 배포하지 않음(안전 판단)
- 함수시작 15개는 재핀·소스 반영했으나 **빌드·배포·deps 갱신을 하지 않았다**. 배포본은 0.5.6 대역 유지 ⟹ 0.5.7에서 자동 비활성 = 안전.
- **근거(스텁 컨테이너 15개 전수 판정)**: **BYTE=SAME 0/15**. 그중 **9개는 컨테이너 자체가 재핀 불가(NONE)** — 밴픽 UI 대형 컨테이너 `0x254f8f0`(SFX·DRAIN_HL·DRAIN_HL2·SLOTSEL 전부 소속)과 AITURN 컨테이너 `0x2079730`(champ_pos_lock DISPATCH와 동일 함수). 재핀된 3개(AI1 `0x1837690`·AI2 `0x183b3b0`·HL `0x1d837a0`)도 전부 BYTE=DIFF이고 HL은 size 47102→45696으로 크게 변했다.
- ⚠**0.5.6 실사고가 정확히 이 축**: site/join 주소만 갱신하고 스텁이 박은 rbp-disp32를 안 고쳐 HL arm `lea r8,[rbp+0x9b10]`(정답 `0x9cf0`)가 **"이어하기" 즉사 AV**를 냈고, **SIG가 arm 본문을 미커버해 결함 스텁이 sig 체크를 통과**했다 ⟹ **sig 통과는 안전 보장이 아니다**.
- → 다음 세션: 컨테이너 9개를 ghidra-re로 규명 → 각 스텁의 disp 바이트를 신 exe arm과 **전수 대조**한 뒤에만 빌드. 소스 상단(`hooks.rs`)에 같은 경고 배너를 넣어뒀다.

### 4d. ★4차 — ai_adjust 바이트패치 복구 + banpick_order 컨테이너 규명 (2026-08-26 22:00~22:22)

#### 4d.1 ✅tfm2_ai_adjust 바이트패치 665건 복구 (사이트+가드표 **동시** 전환)
3차(§4c.2)에서 "한쪽만 못 바꾸니 표를 구버전으로 둔다"고 했던 것을 해소했다. **사이트 문맥을 전수 조사**해 나머지 형태를 찾은 것이 열쇠:
| 문맥 | 건수 |
|---|---|
| `base + 0xRVA` | 413 |
| `(0xRVA, &[opcode], N)` 튜플 배열 | 129 |
| `for rva in [0xRVA, …]` 배열 | 93 |
| `rva: 0xRVA,`(class_micro 구조체) | 5 |
⟹ **사이트 674건 치환**(detour.rs 667·class_micro.rs 5·nexus_emg.rs 2, `//` 이전 코드부만 — 주석의 이력 hex 보존) + **orig_table.rs 재생성**(신주소 665 + 구주소 유지 243 = 908·중복 rva 0).
- ★**주소 스왑 발견**: 구 `0xd4f847`→신 `0xebd8f7` 이면서 **동시에** 구 `0xe32617`→신 `0xd4f847`. 단일 pass 치환이라 이중 치환은 없었고, 검증 스크립트의 "구주소 잔존 1건"은 이 스왑 때문인 **오탐**(git diff로 확인).
- **정적 검증(배포 dll 바이너리 실측)**: 신주소 **665/665 임베드** · 구주소 잔존 0(스왑분 제외) · 표 908 유지 · 신주소 665개 전부 표에 존재 · 훅 **9/10 임베드**(SUBPLAN만 없는 것은 `SPDISP_PROBE=false` 로 dead code 제거된 진단용 wrap이라 **정상**).
- 배포 = dll **3,676,160B @2026-08-26 22:14:16** · 커밋 `c9244be`. ⬜인게임 검증(`imm_guard` checked/blocked 카운터로 확인).

#### 4d.2 ✅champ_pos_lock `RVA_DISPATCH` 재핀 — ★**콜리 지문** 기법
3차에서 "재핀 불가(skel/head/마스크시그 전부 NONE)"로 stale 유지했던 것을 **콜리 지문**으로 해결했다.
> **콜리 지문**: 구 함수가 호출하는 callee 들을 각각 재핀한 뒤, 신 exe 에서 **그 신-callee 들을 가장 많이 호출하는 함수**를 찾는다. 콜러가 1개뿐이라 콜러그래프 투표가 안 되는 함수에도 통한다.

`0x2079730` → **`0x232a950`**(지문 24/26 · 2위 16 · size 5345→5589 · 프롤로그 8push 12B 동일 ⟹ `PROL_RECOMMEND` 무수정). 배포 dll **3,183,104B @22:22:05**.

#### 4d.3 ⛔banpick_order — 컨테이너·사이트는 전부 규명, 그래도 **여전히 보류**(사유가 바뀜)
**진전**: 3차에 "재핀 불가"였던 컨테이너 2개를 콜리 지문으로 규명했고, 스텁 사이트도 **15/15 전건 확정**했다.
- 컨테이너: AITURN `0x2079730`→**`0x232a950`**(지문 24/26) · 밴픽 UI `0x254f8f0`→**`0x1ed7970`**(지문 **97/97**·2위 27·size 128349→127160) · HL `0x1effd90`→`0x1d837a0` · AI1 `0xf97570`→`0x1837690`(정렬 100%) · AI2 `0xf9b290`→`0x183b3b0`(정렬 100%)
- 사이트 15: AITURN_SITE **`0x232ad9a`** · AITURN_JOIN **`0x232b242`**(call 타겟이 재핀된 TURN `0x1642d30`과 일치 = 교차검증) · SFX_SITE **`0x1ed88b5`** · SFX_END **`0x1ed8904`** · DRAIN_HL_a **`0x1ee4de3`** · DRAIN_HL_b **`0x1ee50fa`** · DRAIN_HL2_a/b/c **`0x1ee2e47`/`0x1ee4132`/`0x1ee3359`** · SLOTSEL **`0x1eee9e2`** · HL **`0x1d84277`** · AI_SITE1/JOIN1 **`0x1837852`/`0x1837960`** · AI_SITE2/JOIN2 **`0x183b468`/`0x183b539`**

★★**그런데 disp 실측 결과 "disp만 갈면 되는" 문제가 아니었다** — 이게 이번 회차의 핵심 발견이다:
| 스텁 disp | 0.5.6 | 0.5.7 | |
|---|---|---|---|
| AITURN rule | `0x68d1` | **`0x6a11`** | 변경 |
| AITURN ban | `0x68c8` | **`0x6a08`** | 변경 |
| AITURN 스텁슬롯 | `0x6a20` | **`0x6b80`** | 변경 |
| AITURN_JOIN store | `0x6a2f` | **`0x6b8f`** | 변경 |
| SFX scene | `0x1300` | **`0x1290`** | 변경 |
| HL rule | `0xb0a9` | **`0xaf99`** | 변경 |
| DRAIN_HL cur total | `0x1128` | `0x1128` | 불변 |
| AI2 ban/rule | `0x110`/`0x108` | 동일 | 불변 |

⟹ **그냥 빌드했다면 0.5.6 즉사 AV가 그대로 재현됐을 것**이다(실증).

★**더 결정적인 것 — 스텁 재작성이 필요하다**:
- **HL**: 구 `mov r14b,0xff` + `lea r8,[rbp+0x9cf0]` → 신 **`mov bl,0xff` + `lea r13,[rbp-0x20]`**. **레지스터가 바뀌었고(r14b→bl, r8→r13) disp 성격도 다르다**(프레임 양수 → `rbp-0x20`). disp 치환으로 해결되지 않는다.
- **SLOTSEL**: 구에 있던 `mov rsi,[rbp+0x1338]`(사이트+0x1b)이 **신 exe에는 아예 없다**(바로 `jae`로 간다). 스텁이 재현할 원본 명령 자체가 사라졌다.
- **DRAIN_HL_a**: `cmp rsi,-1` → `cmp rdx,-1`(레지스터 변경).

⟹ **결론**: banpick_order 스텁은 "원본 명령을 재현 + 부작용 주입" 설계라, 원본 명령의 **레지스터 할당·구조가 바뀌면 스텁 코드 자체를 다시 써야 한다**. 주소·disp 치환만으로는 불가 ⟹ **배포 보류 유지**(0.5.6 대역 = 자동 비활성).
→ **다음 세션의 실제 작업** = 사이트별로 신 exe 명령 시퀀스를 읽어 **스텁 본문(재현 명령 + arm 부작용 + SIG)을 재작성**. 컨테이너·사이트 주소는 위 표로 이미 확보돼 있으니 그 단계는 건너뛴다. SIG는 **arm 본문까지 커버**하도록 확장할 것(0.5.6 교훈 — 미커버 시 결함 스텁이 통과한다).

### 4e. ★5차 — banpick_order 복구 + CPS_OFF 영구 해결 ⟹ **0.5.6 대역 잔존 0** (2026-08-26 22:30~22:45)

#### 4e.1 ✅item_tactics `CPS_OFF` — 하드코딩 폐기, **컴파일 타임 계산**으로 전환
`const CPS_OFF: usize = 0x16ff8;` → **`core::mem::offset_of!(game_core::Database, champion_patch_statistics)`**.
- 이 값은 exe RVA가 아니라 **SDK 구조체 필드 오프셋**이라 SDK가 정본이다. offset_of 로 바꾸면 **버전마다 자동 추종**하므로 이 축이 영구히 사라진다(빌드 성공으로 타입·필드 접근 가능 확인).
- 배포 dll에서 구값 `0x16ff8` 리터럴이 사라진 것도 확인. 실제 값은 인게임 진단 로그(`★실제 cps 오프셋 = …`)에 찍힌다.

#### 4e.2 ✅tfm2_banpick_order 복구 — "되는 것만 켜고 나머지는 **구조적으로** 막는다"
3차·4차에서 두 번 보류했던 모드를 **안전이 검증된 구성**으로 복구했다. 열쇠는 **SIG 대조 실측**이었다.

**① 자동 스킵되는 것(=기능 죽되 크래시 없음) — SIG가 신 exe와 불일치함을 실측**
| 스텁 | 구 SIG | 신 exe 바이트 | 원인 |
|---|---|---|---|
| HL | `ffe2488d510441b6ff` | `ffe2488d5104b3ff48` | `mov r14b,0xff` → **`mov bl,0xff`** |
| SLOTSEL | `4d01d2488d35b5b6` | `4d01d2488d3508cb` | `lea rsi,[rip+…]` disp 변경 |
| ~~DRAIN_HL 계열~~ | ~~(동일 컨테이너)~~ | ~~불일치~~ | ★**정정(§4f): SIG 일치해 실제로 설치된다** — 5차 대조 스크립트의 파싱 실패를 불일치로 오독한 것 |
⟹ 0.5.6 즉사 AV는 "SIG가 arm 을 미커버해 **결함 스텁이 통과**"한 경우였는데, 0.5.7은 **SIG 자체가 어긋나 통과 경로가 없다**. 주소는 미리 맞춰뒀다(나중에 스텁 재작성 시 바로 쓰도록).

**② ★AITURN — SIG는 통과하는데 위험해서 새 가드를 넣었다**
SIG(선두 14B)를 disp만 갈면 통과한다. 그런데 **site→join 구간을 실측**하니:
| | 0.5.6 | 0.5.7 |
|---|---|---|
| 구간 | 231B | **1192B** |
| 명령 | 53 | **246** |
| **call** | **0** | **21** |

이 스텁은 site→join을 **통째로 건너뛴다**. 구간이 순수 phase 계산(call 0)일 때만 안전한데, 0.5.7은 **신규 call 21개**가 생겨 건너뛰면 게임 로직이 소실된다. SIG는 선두 14B만 보므로 이 변화를 **못 잡는다** — 0.5.6 AV와 같은 부류의 사각이다.
⟹ **`AITURN_MAX_SPAN = 300` 구간 크기 가드 신설**(`install_aiturn` 진입부). 1192 > 300 이라 0.5.7에서 자동 스킵되고, 나중에 구조가 되돌아오면 자동으로 다시 통과한다. **편법(일부러 틀린 SIG)이 아니라 구조적 안전장치**다.

**③ 작동 예상**: SFX(SIG 일치 + 구간 **79B 구·신 동일** + scene disp `0x1300`→`0x1290` 갱신) · AI1/AI2(SIG 구·신 동일·disp `0x110`/`0x108` 불변 — 단 `cfg.ai_inline_phase` 기본 OFF) · 함수시작 훅 15.

**④ 프롤로그 검증 7종 대조 → 1건 갱신**: `PROLOGUE_SCALAR` 만 불일치였는데 앞 9B는 같고 **`lea rdx,[rip+disp]`의 rip-rel 3B**만 달랐다(`83 67 2F` → **`93 2C DF`**). 0.5.6에도 같은 축을 갱신했던 상수다. 나머지 6종(SCENE·APPLIER·SLOTUPD·PHASE_RAW·LINEUP·COMMIT)은 구·신 동일.

**⑤ 주소 갱신 전건**: 사이트/조인 10 + 문자열 2(`STR_BAN` `0x34915f0` · `STR_PICK` `0x349160c` — `asset/base/sound/sfx/{ban,pick}_sfx` 문자열 검색으로 유일 확정, 0.5.6 값 검산 통과) + 스킵 대상 8.

배포 = dll **2,597,888B @2026-08-26 22:44:37**(⚠1MB 초과 → `build_extra.ps1 -Externs @(…) -MaxSize`) · deps `>=0.5.7, <0.5.8`.

#### 4e.3 ★★게임 폴더 실측 = **0.5.7 대역 28종 / 0.5.6 대역 잔존 0**
0.5.7 마이그레이션의 모드 복구는 이로써 **전건 완료**. 남은 것은 인게임 검증과 릴리스 zip뿐이다.

### 4f. ★6차 — banpick_order 스텁 정밀 감사: **⚠5차의 결함 발견·수정** (2026-08-26 23:00~23:10)

> ★★**이 절은 5차(§4e.2)의 정정이다.** 5차에서 "HL·SLOTSEL·DRAIN_HL 계열은 SIG 불일치로 자동 스킵"이라 판정했는데, **DRAIN_HL 계열은 SIG가 일치해 실제로 설치된다**. 5차의 SIG 대조 스크립트가 DRAIN_HL 튜플을 파싱하지 못해 "불일치"로 잘못 집계한 것이다(파싱 실패를 불일치로 오독).

#### 4f.1 ⚠5차 배포본의 실제 결함 2건
1. **join 주소 미갱신** — 5차에 DRAIN_HL/DRAIN_HL2의 **site 만 신주소로 바꾸고 join/ff_join 은 0.5.6 값 그대로 두었다**. SIG가 일치해 스텁이 설치되고, 스텁 말미의 `jmp [rip]` 가 **구 join 주소로 점프**한다 ⟹ 0.5.7에서 엉뚱한 코드로 튀는 크래시 경로.
2. **DRAIN_HL[1] 스텁의 죽은 부작용** — 스텁이 재현하던 `mov rdx,[rbp+0x1338]`(0.5.6 회차에 "live 포인터=잠재 크래시"로 정정했던 그 항목)이 **0.5.7 게임 코드에는 아예 없다**(구 arm 3곳 전부에서 사라짐). 그대로 두면 rdx에 0.5.7 프레임의 무관한 값을 싣는다.

#### 4f.2 ✅수정 — join 9건 재핀 + 스텁 1건 재작성
**join 재핀(분기 정렬)**: 각 site 에서 정방향 분기 목록을 뽑아 구·신 **분기 인덱스로 대응**시켰다.
| 대상 | 0.5.6 | 0.5.7 | 근거 |
|---|---|---|---|
| DRAIN_HL[0] join | `0x2560933` | **`0x1ee50a9`** | 분기 #0 `jae` off `0x13` 동일 |
| DRAIN_HL[1] join | `0x2560a44` | **`0x1ee519b`** | 분기 #0 `jae` |
| DRAIN_HL2[0] join / ff | `0x2554878` / `0x2554864` | **`0x1ee3028`** / **`0x1ee3014`** | #25·#26 / #0 |
| DRAIN_HL2[1] join / ff | `0x255505e` / `0x2555145` | **`0x1ee41e8`** / **`0x1ee42cf`** | #14 `jmp` off `0x9e` 동일 / #0 |
| DRAIN_HL2[2] join | `0x2554f02` | **`0x1ee408c`** | 분기 #0 `jae` off `0xb` 동일 |
| SLOTSEL join1 / join2 | `0x256a110` / `0x256aa3c` | **`0x1eeeaf3`** / **`0x1eef15c`** | #12 `je` / #0 `jae` (⚠SIG 불일치라 미설치·맞춰만 둠) |

**DRAIN_HL[1] 스텁 재작성**: `+112` 의 7B `mov rdx,[rbp+0x1338]` **제거** ⟹ 142B → **135B**, `fn_off` 63 불변(제거 지점보다 앞), `join_off` **134 → 127**. 재작성 후 디스어셈으로 `fn_off`/`join_off` 위치와 **rbp-disp 0개**를 검증했다.

**변경 없이 그대로 둔 것(실측 근거)**: DRAIN_HL[0] 스텁 — disp `0x1128` 이 0.5.7에도 실재(사이트 주변 스캔) · 출력 `r14b` 가 신 arm `mov r14b,0xff` 와 일치. DRAIN_HL[1] 출력 `r15b` 도 신 arm `mov r15b,0xff` 와 일치. DRAIN_HL2 3종은 스텁에 rbp-disp 가 **0개**라 프레임 변화 무관.

**정적 검증(배포 dll 바이너리)**: 구 join 9건 **전부 0회** / 신 join 9건 **전부 1회 존재**. 배포 = dll **2,597,888B @2026-08-26 23:08:14**.

#### 4f.3 현재 banpick_order 훅 상태 (확정)
| 구분 | 대상 | 근거 |
|---|---|---|
| ✅설치·작동 | 함수시작 15 · SFX · **DRAIN_HL 2** · **DRAIN_HL2 3** | SIG 일치 + join 신주소 + 스텁 disp 검증 |
| ✅설치 가능 | AI1 / AI2 | SIG·disp 구·신 동일 (단 `cfg.ai_inline_phase` 기본 OFF) |
| ⛔자동 스킵 | **HL**(SIG `41b6ff`→`b3ff48`) · **SLOTSEL**(SIG `…b5b6`→`…08cb`) | SIG 불일치 실측 |
| ⛔가드 스킵 | **AITURN** | `AITURN_MAX_SPAN` (span 1192B > 300B) |

#### 4f.4 ★교훈 (버전무관)
- **"파싱 실패"를 "불일치"로 집계하면 정반대 결론이 난다.** 5차의 SIG 대조가 정확히 그랬다 — 스크립트가 튜플을 못 읽어 빈 결과를 냈는데 그것을 "SIG 불일치 = 안전"으로 해석했다. ⟹ 대조 스크립트는 **"몇 건을 실제로 읽었는가"를 반드시 출력**하고, 기대 건수와 대조할 것.
- **site 를 갈면 join 도 같은 트랜잭션에서 갈 것.** mid-func 스텁은 (site, join, ff_join) 이 한 세트다. 하나만 갱신하면 SIG는 통과하고 점프만 틀리는 **가장 나쁜 조합**이 된다.
- **스텁이 재현하는 원본 부작용이 신 버전에서 사라졌으면 재현도 지워야 한다.** 남겨두면 무관한 프레임 값을 레지스터에 싣는다.

### 4g. ★7차 — RE로 스텁 3종 복구 (AITURN·HL 재작성 / DRAIN 계열은 6차 완료) (2026-08-26 23:15~23:30)

기드라·capstone RE 로 "재작성 불가"로 미뤄뒀던 스텁들을 규명해 **3종을 복구**했다. 남은 것은 SLOTSEL 1종.

#### 4g.1 ✅AITURN — **span 가드는 과잉 차단이었다**
6차에 `AITURN_MAX_SPAN`(1192B > 300B)으로 막았는데, 점프테이블을 읽어보니 **차단할 이유가 없었다**:
| | 0.5.6 | 0.5.7 |
|---|---|---|
| arm 위치(JT[0..3]) | site+`0x21`/`0x5b`/`0x8e`/`0xc1` (전부 구간 내) | site+`0x21`/`0x430`/`0x45c`/`0x488` (**흩어짐**) |
| arm 로직 | `lea rcx,[rax+4/8/0xa/6]` + 픽테이블 lookup + `and al,1; or al,2` | **완전 동일** |
| total 소스 | `[rbp+0x6a20]` 스택 슬롯 | ★**`r13` 레지스터** (`cmp r13,rcx`) |

구간이 1192B가 된 것은 **arm 들이 흩어졌기 때문**이고, arm 들은 전부 join 으로 수렴한다 ⟹ 사이의 call 21개는 **이 경로에서 원래도 실행되지 않는다**. 통째 치환은 여전히 유효하다.

**스텁 재작성**: `mov rcx,[rbp+0x6a20]`(7B) → **`mov rcx,r13`**(3B) ⟹ 38B → **34B**. rule/ban disp 는 6차에 이미 갱신(`0x6a11`/`0x6a08`).
**가드 교체**: span → ★**JT 구조 가드**. `lea rdx,[rip+disp]` 의 disp 로 JT 를 구하고 **JT[0] == site+0x21** 인지 검증한다(디스패치 구조가 그대로인지가 진짜 안전 조건). 구·신 exe 양쪽에서 시뮬레이션해 **둘 다 통과**함을 확인했다.

#### 4g.2 ✅HL — 레지스터만 바뀌었다
join 을 읽어 소비 레지스터를 확정했다: 구 `mov eax, r14d` → 신 **`mov eax, ebx`** ⟹ phase 출력이 `r14b`→**`bl`**.
| 항목 | 0.5.6 | 0.5.7 |
|---|---|---|
| SIG | `ffe2488d510441b6ff`(9B) | **`ffe2488d5104b3ff4839d0`(11B)** — `mov r14b,0xff`(3B)→`mov bl,0xff`(2B), ★`cmp rax,rdx` 까지 커버 확장 |
| rule 슬롯 | `[rbp+0xb0a9]` | **`[rbp+0xaf99]`** |
| phase 출력 | `movzx r14d,[rsp+0x60]`(6B) | **`mov bl,[rsp+0x60]`**(4B) — 8비트만 써서 rbx 상위 보존(게임 arm 도 `mov bl`) |
| arm 부작용 | `lea r8,[rbp+0x9cf0]`(7B) | **`lea r13,[rbp-0x20]`**(4B) |
⟹ stub 147B → **142B**, `fn_off` 67 불변, `join_off` **139 → 134**. 재작성 후 디스어셈으로 오프셋·명령을 검증.

#### 4g.3 ✅SLOTSEL — **8차에 복구 완료**(아래 §4h)
~~SIG 불일치라 스킵 유지~~ → **정정**: SIG 불일치의 원인은 구조 변경이 아니라 **`lea rsi,[rip+disp]` 의 disp 가 SIG 안에 들어있었기 때문**이었다(명령 자체는 동일). §4h 참조.

#### 4g.4 최종 훅 상태 (SIG 9건 전수 실측)
| 상태 | 대상 |
|---|---|
| ✅설치 | 함수시작 15 · **AITURN** · **HL** · SFX · DRAIN_HL 2 · DRAIN_HL2 3 |
| ✅설치 가능 | AI1 / AI2 (`cfg.ai_inline_phase` 기본 OFF) |
| ⛔스킵 | **SLOTSEL** 1종 |
배포 = dll **2,598,912B @2026-08-26 23:24:38**.

#### 4g.5 ★검증 스크립트가 같은 실수를 두 번 냈다 (교훈 강화)
6차 교훈("파싱 실패를 불일치로 집계하지 말 것")을 적고도, 7차 최종 검증에서 **SLOTSEL SIG 를 0B 로 파싱해 "설치"로 오판**했다 — 빈 SIG는 빈 읽기와 같아 `nb == sig` 가 참이 되기 때문이다. 원인은 타입 선언 `(…, &[u8], &[u8], …)` 이 배열 정규식에 잡혀 인덱스가 밀린 것.
⟹ **대조 스크립트에는 `assert len(sig) > 0` 급의 가드를 반드시 넣는다.** "몇 건을 읽었는가"를 출력하는 것만으로는 부족하다 — 이번엔 건수(9/9)는 맞았고 **길이가 0**이었다.

### 4h. ★8차 — SLOTSEL 복구 ⟹ **banpick_order 스텁 4종 전건 완료** (2026-08-27 00:00)

#### 4h.1 SIG 불일치의 진짜 원인 = rip-rel disp
7차에 "SIG 불일치 = 구조 변경"으로 읽었는데 틀렸다. 바이트를 명령 단위로 뜯어보니:
```
구 0.5.6: 4d01d2 488d35 b5b6      = add r10,r10 ; lea rsi,[rip+0xf7b6b5]  (앞 2B만 SIG에 포함)
신 0.5.7: 4d01d2 488d35 08cb      = add r10,r10 ; lea rsi,[rip+0x15acb08]
```
⟹ **명령 자체는 완전 동일**하고 SIG 8B 안에 rip-rel disp 앞 2바이트가 들어가 있어서 어긋났을 뿐이다. SIG 를 신 disp 로 갱신하면 통과한다(⚠이 SIG 는 구조상 disp 를 포함하므로 **버전마다 갱신 필요** — 소스에 주석으로 명시).

#### 4h.2 join3 확정 — 명령 시퀀스가 완전 일치
7차에 "구 분기에서 대응을 못 찾음"이라 했던 `0x256a10a` 를, **join1 앵커(`0x256a110`→`0x1eeeaf3`) 기준 주변을 뜯어** 확정했다:
| 오프셋 | 구 0.5.6 | 신 0.5.7 |
|---|---|---|
| +0 | `test al, 1` | `test al, 1` |
| +2 | `je 0x256a110` | `je 0x1eeeaf3` |
| **+4** | **`cmp r9, r8`** ← join3 | **`cmp r9, r8`** ← **`0x1eeeaed`** |
| +7 | `setne dl` | `setne dl` |
| +a | `add ecx, -2` ← join1 | `add ecx, -2` ← `0x1eeeaf3` |
| +13 | `jne 0x256aa3c` | `jne 0x1eef15c` |
분기 수가 34→32로 준 것은 이 블록과 무관한 다른 곳의 변화였다.

#### 4h.3 스텁 재작성
`mov rsi,[rbp+0x1338]`(+99, 7B) **제거** — 이 부작용은 0.5.7 게임 코드에 없다(DRAIN_HL[1] 과 같은 축). ⟹ stub 174B → **167B**, `fn_off` 50 불변, join_off **130/148/166 → 123/141/159**.
★**내부 상대분기는 손댈 필요가 없었다** — `je`/`jne` 의 소스와 타겟이 **둘 다 제거 지점 이후**라 상대값이 그대로다. 재작성 후 디스어셈으로 확인(`je 0x83`=131 · `jne 0x95`=149 가 새 join 블록과 정확히 일치).

#### 4h.4 ★최종 — banpick_order 훅 전건 설치 (SIG 9건 전수 실측·`len>0` 가드 통과)
| 상태 | 대상 |
|---|---|
| ✅설치 | 함수시작 15 · AITURN · HL · SFX · DRAIN_HL 2 · DRAIN_HL2 3 · **SLOTSEL** |
| ✅설치 가능 | AI1 / AI2 (`cfg.ai_inline_phase` 기본 OFF) |
| ⛔스킵 | **없음** |
배포 = dll **2,598,912B @2026-08-27 00:00:50**. ⬜인게임 검증(특히 **"이어하기" 무크래시** — 0.5.6 AV 재현 여부).

#### 4h.5 ★교훈 — "SIG 불일치"를 구조 변경으로 단정하지 말 것
0.5.7 회차에서 SIG 불일치 4건 중 **3건(AITURN·HL·SLOTSEL)이 실제로는 국소 변화**였다:
- AITURN: 구간이 커진 것뿐(arm 흩어짐) — 로직 동일
- HL: 레지스터 재할당(`r14b`→`bl`)
- SLOTSEL: **SIG 안에 rip-rel disp 가 들어있었을 뿐**

⟹ SIG 가 안 맞으면 **바이트를 명령 단위로 뜯어 "무엇이 달라졌는지"를 먼저 본다.** 스킵은 안전하지만 기능이 죽으므로, 안전을 이유로 분석을 건너뛰면 복구 가능한 것을 포기하게 된다.
⟹ 역으로 **SIG 에 rip-rel disp·상대분기 바이트를 넣지 말 것**(넣으면 버전마다 무의미하게 깨진다). 넣어야 한다면 소스에 그 사실을 명시한다.

### 4i. ★★인게임 검증 (2026-08-27 00:09~00:30) — 크래시 0건·유저 "다 잘된다"

#### 4i.1 ⚠먼저: **첫 기동은 크래시했다**(원인·수정 = §4i.2)
0.5.7 배포 직후 게임 기동 시 **즉시 크래시**. `code=0xc0000005` · **RIP = `exe+0x2e6f03`** · faultAddr `0xffffffffffffffff`.
`0x2e6f03` 은 **0.5.6 LOADER(`0x2e6f60`) 근처** ⟹ 구 LOADER 주소로 후킹한 모드가 있다는 뜻이었다.

#### 4i.2 ★원인 = **서브 파일 누락**. 완료 게이트를 돌리지 않은 것이 화근
전 모드 `.rs` 를 코드부(주석 제외)만 스캔하니 **2개 파일 8건**이 0.5.6 값 그대로였다:

| 파일 | 누락 상수 |
|---|---|
| `tfm2_item_tactics\src\ui_inject.rs` | `LOADER_RVA`·`STRAT_LOADER_RVA`(둘 다 `0x2e6f60`) · `PARSER_RVA`(`0x19ab40`) · `ALLOC_RVA`(`0x2ab1670`) ← **크래시 직접 원인** |
| `tfm2_champ_pos_lock\src\hooks.rs` | `RVA_CPROD`(`0x1f16ea0`) · `RVA_DISP`(`0x2079730`) · `RVA_COMMITTER`(`0x24b6c10`) + 로그 문자열 1 |

- item_tactics 는 **`lib.rs` 만 보고 `ui_inject.rs` 를 놓쳤다**. champ_pos_lock 은 `RVA_DISPATCH` 를 고치고 **이름이 비슷한 `RVA_DISP` 를 놓쳤다**.
- ★**DONE.md 에 이미 "마이그 재핀 완료 게이트 = 구버전 RVA 리터럴 전 소스 grep(include·다중prefix·imm·toggle 사각)" 교훈이 있었는데 실행하지 않았다.** 스텁·SIG 같은 어려운 축에 집중하느라 기본 점검을 건너뛴 것이 원인이다.
- 수정 후 재빌드 → 배포 dll 정적 검증(구 주소 0회/신 주소 존재) + **전 배포 dll 에서 구 LOADER 잔존 0** 확인.
- ⟹ 완료 게이트 스크립트를 **`C:\tfm2mods\_t059q.py`** 로 상설화했다(전 모드 `.rs` 코드부만 스캔 · `_rva057.json` 의 구 RVA 전량 대조).

#### 4i.3 ✅검증 결과 — 로그로 확증
스냅샷(`_pretest_057.txt`) 대비 증분으로 판독.

| 항목 | 결과 |
|---|---|
| **크래시** | crash_log / panic_log / comptest_crash / banpick_order crash_log **전부 증가 0** · 신규 CrashDumps **0** |
| **ai_adjust 훅** | 스텁 인벤토리 **n=11 전부 신주소** — retreat `0xe4a750` · fc59a0 `0xe61600` · **condgate `0xdb1e20`** · movepri `0xdb2760` · itemnet `0x17f09b0` · **disc18 `0xe9fd70`** · **disc19 `0xeae620`** · auction `0xe8b800` ⟹ **본문변경 5종 포함 전건 설치·발화** |
| **ai_adjust 바이트패치** | **`imm_guard: checked=658/908, blocked=0`** ⟹ 잘못된 주소에 패치를 시도한 것이 **하나도 없다**. §4d.1 의 665건 복구가 실증됨 |
| **level_cap** | 훅 설치 + 실발화 `observed_len=17 calls=78778 patched=64` |
| **elemental_serpen** | 훅 5종 신주소 설치 — serpen `0x14a25f0` · 렌더스텝 `0x90a090` · 장로처형 `0x10af580` · 파이프라인B `0x108f220` · 증폭A `0x15912c0` |
| **community_reaction_mod** | `gpo_debug.txt` 갱신(워크샵 경로 dll 로드 확인) |
| 유저 확인 | level_cap · serpen · banpick_illust · comptest · community_reaction · Spectator_Chat · mod_order · 다람이 8종 = **"다 잘된다"** |

★**imm_guard 예측 정정**: §4d.1 작성 시 "blocked 가 243 근처로 나오는 것이 정상"이라 적었으나 **실제는 blocked=0**. 미재핀 243건은 `patch_imm_bytes` 의 **opcode 대조 단계에서 먼저 걸러져** 가드까지 도달하지 않았다(또는 그 배선이 이번 판에 안 돌았다). 예측보다 좋은 결과다.

#### 4i.4 ⬜로그 근거가 없는 항목(육안 확인만)
serpen **경기 중 기능**(런처 발화 0 = 이번엔 경기 미관전) · comptest **조합테스트 기능**(이번 실행 로그 없음 — 화면 미진입 추정) · banpick_illust(진단 로깅 OFF) · Spectator_Chat · mod_order · 다람이 8종.
**정상이라는 뜻이 아니라 "로그로는 확인 못 했다"는 뜻**이므로, 다음 플레이에서 해당 화면을 거치면 자동으로 근거가 쌓인다.

### 5. 잔여 (0.5.7)
- ~~⛔미복구 = `tfm2_banpick_order` 1종~~ → **✅5차에 복구·배포 완료(§4e.2)**. **게임 폴더 실측 0.5.7 대역 28종 / 0.5.6 대역 잔존 0 = 모드 복구 전건 완료.**
- ~~⬜ai_adjust 바이트패치 665건 복구 여지~~ → **✅복구 완료(§4d.1)**. 잔여 = 미재핀 243건(표에 구주소로 남겨 blocked = fail-safe) · JT 베이스 · class_micro 18 · 본문변경 5훅의 재현 코드 정합성.
- ~~⬜champ_pos_lock `RVA_DISPATCH` 재핀 불가~~ → **✅콜리 지문으로 확정 `0x232a950`·재배포 완(§4d.2)**. ⬜**sylas `EFF_VT_BASE`** 만 stale 유지(자체 fail-safe 확인·§4c.1).
- ~~⬜item_tactics `CPS_OFF` 미검증~~ → **✅`offset_of!` 컴파일 타임 계산으로 전환·이 축 영구 해소(§4e.1)**.
- ~~⬜banpick_order 스텁 재작성~~ → **✅4종 전건 복구 완료(§4f·§4g·§4h)**. 훅 스킵 **0건**. · ~~comptest 바이트패치 19~~ → **✅17/17 재핀 완료(§4c.1)** · **ai_adjust 바이트패치 908·JT 베이스**(665건 매핑은 확보·§4c.2).
- ~~⬜빌드·배포 전량~~ → **✅28종 배포 완료**(§4·§4b·§4c). ~~⬜빌드스크립트 sdk_057 전환~~ → **✅6종 전환 완료**(§4). ⬜**릴리스 zip 0.5.7 미생성** · ⬜**인게임 검증 전량 미실시**.
- ~~⬜deps 게이트 = `>=0.5.6, <0.5.7` 28종 자동 비활성·상한 갱신 필요~~ → **✅갱신 완료. 최종 실측(2026-08-26 21:40) = 0.5.7 대역 27종 + community_reaction_mod(워크샵 `>=0.1.0`) = 28종 / 0.5.6 대역 잔존 = `tfm2_banpick_order` 1종뿐**(의도된 비활성).
- ★**3형제 축 점검(0.5.6 실사고 재발 방지)** — ~~셋 다 미착수~~ → **정정(2026-08-26)**: ①콜러 프레임 오프셋 = **✅완료**(item_tactics TN_FR_DB/CFG/SETEND 개별 재핀·§4b.1 — 슬롯 간 상대 간격이 바뀌어 델타 일괄 적용은 오답이었다) ②mid-func 스텁 rbp-disp32 = **⛔미해결이라 banpick_order 배포 보류**(§4c.3) ③버전 게이트 = **✅완료**(`GAME_EXE_SIZE_056` 77_101_056 → **77_111_808**·§4b.1).
- ★**워크샵 서드파티 훅 충돌 점검** — **✅점검 완료(§4b.2)**: `riot_items_tfm2` v0.9.2 deps `=0.5.6` 고정 ⟹ **0.5.7에서 자동 비활성 = 이번 회차 충돌 없음**(0.5.7판 미출시). ⚠제작자가 0.5.7판을 올리면 축이 되살아나므로 이상 시 1순위 용의선상. + ⬜**deps 상한 없는 우리 워크샵 모드 3종**(gg 애드온) 실 로드처 확인(§4b.2).
- ⬜**패치노트發 신규 관측 항목**: ①**"선수 AI 모드 사용 시 다시보기가 실제 경기와 다르게 재생" 수정** — 리플레이 재계산에 **모드 AI가 이제 적용됨**(종전엔 기본 AI로 실행) ⟹ ai_adjust·flow_capture·리플레이 의존 모드의 동작 전제가 바뀜 = 별도 검토 필요 ②목록형 선택창 화면내 개폐+스크롤 전환(드롭다운 사용 모드) ③구버전 형식 파일 포함 모드 구독 시 밴픽 AI 오류로그 정리 ④비활성 모드 파일 미독 오류 기록 수정.
