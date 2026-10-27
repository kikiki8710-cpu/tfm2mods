# 핫픽스 노출 감사 — 0.6.5 기준 (2026-10-27, 과업 D · 읽기 전용 감사)

> 기계용 사본 = 같은 폴더 `hotfix_exposure_0.6.5.json`(`run_patchday.py` SUMMARY §8 이 앞 7열을 표로 붙인다).
> 기준 exe = 0.6.5 `DAF040DB8A3A3AE1…`(87,009,792B) · 대상 = 지금 설치된 `<게임>\mods\*` 77폴더 + 켜 둔 워크샵 6개.
> 이 감사는 **아무 파일도 바꾸지 않았다**(게임·모드·매니페스트 무수정). 측정에 쓴 1회용 스크립트는 `MIG\_patchday\_tmp\hotfix_exposure\`.

## 결론 (먼저 읽을 것)

1. **정식 패치(버전 번호가 0.6.6 으로 오름)면 안전하다.** 켜 둔 코드 모드 15개 중 deps 상한이 있는 10개(`>=0.6.5, <0.6.6`)는 게임이 **알아서 끈다**. 나머지 5개(`>=0.6.0`, 상한 없음)는 계속 켜지지만 전부 stable API 만 쓰고 게임 주소(RVA)를 하나도 박아 두지 않아서 무관하다.
2. **핫픽스(버전 문자열이 0.6.5 그대로이고 exe 만 바뀜)가 위험하다.** 이때는 deps 검사가 그대로 통과해 **15개가 전부 새 exe 위에서 로드된다.** 과거 핫픽스 7번은 모두 버전 문자열을 바꾸지 않았다(§1.2 실측) ⟹ 내일 핫픽스가 오면 이 상황이 된다.
3. 핫픽스 때 **크래시 위험이 높은 것은 2개**다 — 진입부 확인 없이 게임 코드를 덮어쓰는 지점이 있다.
   - `tfm2_comptest_unlock`: UI 로더 훅 `install_one` 이 확인 없이 12B 를 덮는다(`ui_inject.rs:334-361` · `post_update` 에서 호출돼 첫 프레임에 1회 설치 `tfm2_comptest_unlock.rs:4320`·`ui_inject.rs:363-371` · `UI_INJECT_ON=true` `:3409`).
   - `tfm2_elemental_serpen`: 툴팁 훅 `install_arg_str_hook` 이 확인 없이 15B 를 옮기고 12B 를 덮는다(`lib.rs:2736-2758`). 이 함수는 **모드 init 에서 바로** 불린다(`lib.rs:2564`).
   - 0.6.4·0.6.3 exe 에 0.6.5 주소를 대 보면 두 주소 모두 **다른 함수의 한가운데**를 가리킨다(§3) ⟹ 전면 재링크형 핫픽스면 엉뚱한 코드를 덮게 된다.
4. 중간 위험 1개: `banpick_view_plus` — 훅 3개는 진입부를 확인하지만, 그 훅 안에서 **확인 없이 부르는 게임 함수가 13개**다. 일부만 옮겨지는 핫픽스(과거 2/7회)면 훅은 걸리고 헬퍼는 엉뚱한 곳을 불러 밴픽 연출 중 크래시할 수 있다.
5. 나머지는 **안전하게 꺼지거나 기능만 죽는다**. 특히 `tfm2_detail_tactics` 는 EXE SHA256 이 1바이트라도 다르면 100전술 전부를 설치하지 않는다(`micro.rs:218`) — 실제로 이 길을 탄 로그도 있다(§2 행 10).
6. **권고**: 업데이트가 오면 **게임을 켜기 전에** `run_patchday.py` 를 먼저 돌려 핫픽스인지 본다. 핫픽스면 마이그·재배포가 끝날 때까지 `tfm2_comptest_unlock`·`tfm2_elemental_serpen` 을 **꺼 두고**(필수), `banpick_view_plus` 도 꺼 두기를 권한다. 끄는 방법은 §6.

---

## 0. 무엇을 보고 판단했나

| 재료 | 위치 | 쓴 곳 |
|---|---|---|
| 켜진 모드 목록 | `<게임>\config\game\mods.json` 의 `enabled_mods`(21개, 10-23 02:00) | 활성 여부 |
| 모드 deps | `<게임>\mods\*\mod.mod_info`(전부 BOM 없음·첫 바이트 `7b`) · 워크샵 `steamapps\workshop\content\3009300\<id>\mod.mod_info` | (a)(b) |
| 로더 규칙 | 0.6.5 exe 문자열(아래 §1.1) · `<게임>\mod-sdk-stable\README.txt` · 메모리 `tfm2-native-mod-loader-abi.md`·`CURRENT.md §1` | (a)(b) |
| 과거 핫픽스 | `F:\claude\tfm2\tfm2_<버전>\TeamfightManager2.exe` 백업 14개(0.4.13~0.6.5) | §1.2 |
| 모드 가드 | 각 모드 소스(커밋 `8548c99`·`1c3da9c`·`0901ce3`·`e5a28e6` 기준, 미커밋 변경 없음 — `tfm2_detail_tactics/tools` 2파일만 다른 세션 수정 중) | (c)(d) |
| 소스 = 배포 dll 확인 | 배포 dll 안에 소스의 가드 문자열이 있는지 대조(11개 모드 모두 확인. 예: 상세전술 dll 에 `게임 EXE SHA256 불일치`·기준 SHA 문자열, serpen dll 에 `tooltip arg_str hook installed`. comptest 는 `ath_get prologue mismatch` 1건만 없음 = 그 훅이 빌드에 안 들어간 것으로 보임, 추정) | (c) 근거 |
| 가드 강도 실측 | 매니페스트의 0.6.5 RVA·12B 를 0.6.4·0.6.3 exe 에 대어 봄 | §4 |

## 1. 로더는 무엇을 보고 모드를 끄나

### 1.1 비교 규칙 — 근거

- 로더는 **Rust `semver` 크레이트(1.0.26)** 로 게임 버전과 deps 를 비교한다. 0.6.5 exe 안 문자열:
  - `Failed to parse game version '…' as semver: … . Using 0.0.0` (파일 오프셋 0x3d3811f 부근)
  - `dependency '…' version … does not match requirement …` (0x3d29317 부근)
  - `Invalid version requirement for dependency '…'` (0x3d37b13) · `Mod '…' disabled: …`
  - `…\semver-1.0.26\src\parse.rs` (0x3dcd79c)
- 게임 버전 문자열은 exe 안에 **박혀 있다**: `0.6.5` 3곳(0x3a8daab 타이틀 · 0x3af1f31 · 0x3d388e8 = `Ignoring Workshop mod root … 'base' is reserved` 바로 옆 = 로더 영역). `mod-sdk-stable\base_version.txt` 는 SDK 쪽 표기다(지금 둘 다 0.6.5). 핫픽스 SDK 도 `base_version.txt` 를 안 바꿨다(0.5.0_hotfix SDK = `0.5.0`, 메모리 `tfm2-official-sdk-source.md:28`).
- exe 해시·빌드 번호는 로더가 보지 않는다. stable 모드는 **ABI 레벨이 게임보다 높을 때만** 거절된다(`mod-sdk-stable\README.txt` 끝 문단: "A module built once keeps loading on future game versions; the game only rejects mods that require a NEWER ABI level"). 같은 README 가 "game updates do not disable your mod" 하려면 deps 를 `>=` 로만 쓰라고 한다 = **상한을 두면 버전이 오를 때 꺼진다**는 공식 설명.
- deps 가 안 맞아 꺼진 모드는 `enabled_mods` 에서 **지워지지 않는다**(간접 근거: 0.6.0~0.6.2 동안의 `mods.json` 백업 `bak_20260917_pre_enable`·`bak_20260919_crm_off`·`bak_20260930_poslock_test` 에 deps 범위 밖인 `tfm2_item_tactics`(<0.5.9)·`sylas`(<0.6.1)·`tfm2_banpick_order` 등이 그대로 남아 있다). ⟹ 재마이그 후 deps 만 고쳐 다시 배포하면 메뉴에서 다시 켜지 않아도 돌아올 **가능성이 높다**(⬜미검증 · 단 모드 `version` 을 올리면 목록에서 빠질 수 있다 = 메모리 `tfm2-native-mod-loader-abi.md:17` ②).
- 로딩 **오류**(dll 심볼 없음 등)는 다르다 — `Mod force-disabled because a loading error` 로 강제로 꺼지고 저장된다(같은 메모리 :18). 크래시는 로딩 오류가 아니라서 강제 해제되지 않는다(추정 — 크래시하면 다음 기동에도 같은 모드가 다시 로드됨).

### 1.2 과거 핫픽스 실측 — 버전 문자열은 안 바뀌었고, 코드는 대부분 다시 배치됐다

| 핫픽스 (백업 폴더) | exe 안 버전 문자열 | 함수 경계가 그대로인 함수 | 처음 어긋나는 RVA | 분류 |
|---|---|---|---|---|
| 0.4.13 → 0.4.13_2 | `0.4.13` → `0.4.13` | 3,381 / 114,081 (3.0%) | 0x1000 | 전면 재배치 |
| 0.4.13_2 → _3 | 〃 | 30,974 (27.1%) | **0xb388a0** | **부분 이동** |
| 0.4.13_3 → _4 | 〃 | 5,999 (5.2%) | 0x3bd120 | 전면 재배치 |
| 0.4.13_4 → _5 | 〃 | 3,631 (3.2%) | 0x1330 | 전면 재배치 |
| 0.4.14 → 0.4.14_2 | `0.4.14` → `0.4.14` | 444 (0.4%) | 0x72f0 | 전면 재배치 |
| 0.5.0 → 0.5.0_2 | `0.5.0` → `0.5.0` | 31,029 (25.7%) | **0xb4b750** | **부분 이동** |
| 0.5.0_2 → 0.5.0_3 | `0.5.0` → `0.5.0` | 500 (0.4%) | 0x7910 | 전면 재배치 |
| (참고) 0.6.4 → 0.6.5 정식 | `0.6.4` → `0.6.5` | 1,596 / 155,151 (1.0%) | 0x63c30 | 전면 재배치 |

- 측정 = `.pdata` 함수 경계(begin,end) 일치 수 · 앞 64B 까지 같으면 별도 계수(`_tmp\hotfix_exposure\pdata_shift.py`). 버전 문자열 = `hfver.py`·`hfver2.py`.
- `tfm2_0.4.14_2` 와 `tfm2_0.4.14_hotfix` 는 **같은 파일**(sha256 동일)이라 표본에서 뺐다.
- 베타 exe 는 `0.5.0_beta1`·`0.5.0_beta3` 처럼 **밑줄 붙은 문자열**을 박았었다(`tfm2_0.5.1_b`·`tfm2_0.5.3_b`) — §1.3 의 "semver 파싱 실패" 경로가 실재한다는 뜻.
- ⟹ 이 문서의 핫픽스 두 갈래: **H2 = 전면 재배치**(7회 중 5회 · 거의 모든 주소가 바뀜) / **H1 = 부분 이동**(2회 · 앞 약 11~12MB 는 그대로, 그 뒤만 이동).

### 1.3 버전 문자열 변형별 판정

| 새 게임 버전 문자열 | `>=0.6.5, <0.6.6` (10종) | `>=0.6.0` (stable 5종) | `>=0.4.0` (콘텐츠 6종) | 근거 |
|---|---|---|---|---|
| `0.6.6` (정식) | **꺼짐** | 켜짐 | 켜짐 | semver 비교 + README |
| `0.6.5` (핫픽스 — 과거 7/7회) | **켜짐 → 새 exe 에서 로드** | 켜짐 | 켜짐 | §1.2 |
| `0.6.5+무엇` (빌드 메타데이터) | 켜짐 | 켜짐 | 켜짐 | **추정** — semver 는 `+` 뒤를 비교에서 무시 |
| `0.6.5-무엇` (프리릴리스) | 꺼짐 | **꺼짐** | **꺼짐** | **추정** — semver 1.x 는 같은 major.minor.patch 에 프리릴리스가 붙은 조건에만 프리릴리스 버전을 맞춰 준다 |
| `0.6.5_1`·`0.6.5.1` (semver 아님) | 꺼짐 | **꺼짐** | **꺼짐** | exe 문자열 `… as semver … Using 0.0.0` → 0.0.0 은 어느 `>=` 도 못 넘음. 실행 경로 자체는 **미실측** |

⚠맨 아래 두 줄이 오면 **모든 모드가 한꺼번에 꺼진다**(크래시는 없음). 놀라지 말고 경고창 문구(`does not match requirement`)로 확인할 것.

## 2. 모드별 표 — 지금 켜져 있는 21개

열 설명 — (a) 0.6.6 정식 패치 때 · (b) 버전 문자열이 0.6.5 그대로면 계속 로드되나 · (c) dll 이 패치 전에 스스로 확인하나(소스 file:line) · (d) 핫픽스 때 예상(H2 전면 재배치 / H1 부분 이동) · (e) 권고. 등급 = 핫픽스 때 크래시 위험.

| # | 모드 (등급) | deps | (a) 0.6.6 | (b) 0.6.5 핫픽스 | (c) 스스로 확인 — 근거 | (d) 핫픽스 예상 | (e) 권고 |
|---|---|---|---|---|---|---|---|
| 1 | `tfm2_comptest_unlock` 🔴높음 | `>=0.6.5, <0.6.6` | 자동 비활성 | **로드됨** | **혼합.** 바이트패치 = 원본 바이트 대조 `tfm2_comptest_unlock.rs:356-367` · 훅 = 진입 12B 대조 `:733-744`(범용 push8 2종)·`:3251-3259` · 진입 17B `:4115-4121`. ⚠**UI 로더 훅은 확인 없음** `ui_inject.rs:334-361`(이미 내 훅인지만 봄) — `post_update` 에서 `uinj::install()` `:4320`, `UI_INJECT_ON=true` `:3409`. ⚠확인 없이 부르는 게임 함수: `CTX_CLONE`/`CTX_DROP` `:1803·1809`(`CTXSNAP_ON=true` `:1775`, 훅 `liveb_detour` 안 `:1955`) · `REFRESH_RVA` `:2585` | **H2**: 로더 주소 0x337890 이 다른 함수 중간 → 12B 덮어씀 → 그 코드가 돌면 크래시. 나머지 훅은 대조 실패로 미설치. **H1**(경계가 0x337890 위면): 로더는 무사, 대조를 통과한 훅 안에서 무검증 호출이 옮겨진 주소로 가면 크래시 | **핫픽스면 게임 켜기 전 끄기(필수)** · 장기: `install_one` 에 진입부 대조 추가 |
| 2 | `tfm2_elemental_serpen` 🔴높음 | `>=0.6.5, <0.6.6` | 자동 비활성 | **로드됨** | **혼합.** 훅 대부분 진입 12B 대조 `lib.rs:2502-2515`(SERPEN)·`:2237-2246`(tramp12)·`:985-989`·`:1909-1918` — 18개 중 11개가 범용 push8. ⚠**툴팁 훅 확인 없음** `install_arg_str_hook` `:2736-2758`, init 에서 바로 호출 `:2564`. (UI 로더 훅 `:736` 도 확인 없지만 `USE_UILOADER_HOOK=false` `:655` 로 꺼져 있음) · 🔴구조체에서 읽은 함수포인터를 호출 `:2296-2298`(`O_ENTITY_ACCESSOR`) | **H2**: 기동 즉시 0x1657a70(다른 함수 중간)에 15B 이전·12B 덮어씀 → 크래시 위험. 나머지 훅 미설치. **H1**: 훅이 살아남고 엔티티 구조체가 바뀌었으면 함수포인터 호출에서 즉사(취약점 지도 🔴) | **핫픽스면 게임 켜기 전 끄기(필수)** · 장기: arg_str 에 진입부 대조 |
| 3 | `banpick_view_plus` 🟠중간 | `>=0.6.5, <0.6.6` | 자동 비활성 | **로드됨** | **진입부 대조 + 무검증 헬퍼.** 훅 3개 `showcase.rs:841-860`(외부훅이면 거절, 12·12·13B 대조 — 앞 둘은 범용 push8 `:63-68`) · 각자 독립 설치 `:897-921` · 기하 패치 13항 전부 맞아야 일괄 적용 `:790-817`. ⚠훅 안에서 확인 없이 부르는 함수 13개 `:29-52`(`gfn` = base+RVA `:222-224`) | **H2**: 훅 3개 대조 실패 → 연출 없음(안전). 기하 패치 SKIP. **H1**: 훅(0x220dda0~0x22cb030)이 그대로고 그 뒤 헬퍼(`NAME_GET` 0x22f9770·`SPRITE_CALC` 0x22feb50·`GAME_ALLOC` 0x2f2dc50)만 옮겨지면 밴픽 연출 중 크래시 | 핫픽스면 **끄기 권장**(밴픽 쇼케이스만 사라짐) |
| 4 | `tfm2_champ_pos_lock` 🟡낮음 | `>=0.6.5, <0.6.6` | 자동 비활성 | 로드됨 | **진입부 대조.** 훅 4개 12B — `pick_click_hook.rs:79`·`swap_confirm_hook.rs:41`·`legacy_assign.rs:75`·`ui_kit/draft_scene_stable.rs:78`(전부 범용 push8, 외부훅이면 체인) · 자동 스왑이 부르는 `select_swap` 은 **고유** 12B 대조 `auto_swap.rs:64-67` | **H2**: 훅 미설치 → 포지션 제한·회색 처리·AI 규칙이 조용히 안 걸림(크래시 없음). **H1**: 일부 훅만 설치될 수 있음 | 켜 둬도 됨 · 제한이 안 걸린다는 것만 알 것 |
| 5 | `tfm2_champion_exclude` 🟢 | `>=0.6.5, <0.6.6` | 자동 비활성 | 로드됨 | **진입부 정확 대조.** 17B(`HOOK_ORIG` `hook.rs:12`) 가 같거나 외부훅 형태일 때만 설치, 아니면 `install SKIPPED` `hook.rs:161-173` | 훅 미설치 → 제외 목록 무시(바닐라대로 신챔프 추가) | 켜 둬도 됨 · **시즌 패치(신챔프 추가) 진행은 마이그 후** |
| 6 | `tfm2_level_cap` 🟢 | `>=0.6.5, <0.6.6` | 자동 비활성 | 로드됨 | **원본 바이트 정확 대조.** 2곳 7B `lib.rs:112-125`, 대조 `:420-426`, 서로 독립 설치 `:466-475` | **H2**: 둘 다 미설치 → 레벨 상한 바닐라. **H1**: 한쪽만 설치 가능 → 레벨은 오르는데 경험치 바 깨짐(소스 주석 `:115-118`) | 켜 둬도 됨 · 레벨업 진행은 마이그 후 |
| 7 | `training_view_plus` 🟢 | `>=0.6.5, <0.6.6` | 자동 비활성 | 로드됨 | **구조 가드 + 진입부 정확 대조.** `ui_kit/training_plan_stable.rs:113`(①루프+0x90==state) · `:115`(②) · `:120-135`(함수 7종 진입 8~12B 정확, 1종 첫 바이트) — 하나라도 어긋나면 아무것도 안 씀 | UI 는 뜨고 "적용"이 `오류:` 로 끝남(`training_view_plus/src/lib.rs:294-296`) | 켜 둬도 됨 |
| 8 | `facility_view_plus` 🟢 | `>=0.6.5, <0.6.6` | 자동 비활성 | 로드됨 | **구조 가드 + 느슨한 진입 확인.** `ui_kit/team_sync_stable.rs:68`(①)·`:71`(②)·`:77-83`(③ 함수 3종 **첫 1바이트만** — 약함) | 생산은 stable API 로 서버 반영, 즉시 화면 동기만 실패 → "(화면 반영은 다음 진행 시)" `facility_view_plus/src/lib.rs:327-328` | 켜 둬도 됨 |
| 9 | `custom_tier_assignment` 🟢 | `>=0.6.5, <0.6.6` | 자동 비활성 | 로드됨 | 8과 같은 team_sync 가드 + `ui_kit/client_db_stable.rs:56`(raw 읽기 전 내 팀 ID 자가검증, 읽기 전용) | 티어 저장은 반영, 즉시 동기만 실패 `custom_tier_assignment/src/lib.rs:581` | 켜 둬도 됨 |
| 10 | `tfm2_detail_tactics` 🟢 | `>=0.6.5, <0.6.6` | 자동 비활성 | 로드됨 | **EXE SHA256 전체 대조.** `micro.rs:218`(기준 = `sites.rs:5` `DAF040DB…`) → 실패 시 전 전술 FAILED 후 종료 `:222` · 통과해도 전술별 창 바이트 대조 `:226`·설치 직전 재대조 `:193` · 패닉도 전부 원본 유지 `:204-208` | exe 가 1바이트라도 다르면 **100전술 전부 설치 안 함 = 바닐라 AI**(안전). 실측: `E:\claude\tfm2\logs\game_mods\tfm2_detail_tactics\detail_tactics.log` 1-44행 = `[120] 게임 EXE SHA256 불일치` 뒤 전 사이트 `calls=0 applied=0`(그 세션 exe 는 로그에 없어 원인 미확인) | 켜 둬도 안전 · 추적은 `tools\track_update.py`(run_patchday 8단계) 후 재생성·재배포 |
| 11 | `tfm2_mod_order` ⚪무관 | `>=0.6.0` | **계속 로드** | 로드됨 | 해당 없음 — RVA 0(소스에 `#[path]` 는 `common/log_dir.rs` 뿐 · 매니페스트 "SDK 전용") | 무관 | 조치 불요 |
| 12 | `Spectator_Chat` ⚪무관 | `>=0.6.0` | 계속 로드 | 로드됨 | 해당 없음 — stable 재작성(`StableMatchHook`+UI 라벨, `src/lib.rs:1-10`), `ui_kit_stable.rs` 도 stable API 만 | 무관 | 조치 불요 |
| 13 | `roster_view_plus` ⚪무관 | `>=0.6.0` | 계속 로드 | 로드됨 | 해당 없음 — RVA·raw 접근 없음(grep) | 무관 | 조치 불요 |
| 14 | `recruitment_view_plus` ⚪무관 | `>=0.6.0` | 계속 로드 | 로드됨 | 〃 | 무관 | 조치 불요 |
| 15 | `coaching_staff_view_plus` ⚪무관 | `>=0.6.0` | 계속 로드 | 로드됨 | 〃 | 무관 | 조치 불요 |
| 16-21 | 워크샵 `leefs_variety`·`2`·`3`·`4`·`5`·`touhou_project` ⚪무관 | `>=0.4.0` | 계속 로드 | 로드됨 | 해당 없음 — dll 없음(콘텐츠) | 무관(데이터 형식 변화는 이 감사 범위 밖) | 조치 불요 |

## 3. 높음 2종 — 왜 "잘못된 주소에 패치"인가

두 지점 모두 "그 주소에 무엇이 있든" 덮어쓴다. 0.6.5 주소를 다른 버전 exe 에 대 보면(전면 재배치 핫픽스의 모의 실험):

| 지점 | 0.6.5 RVA | 0.6.4 exe 에서 그 주소 | 0.6.3 exe 에서 그 주소 |
|---|---|---|---|
| comptest UI 로더 (`LOADER_RVA`) | 0x337890 | 함수 0x3377f0 **+0xa0** 한가운데 | 함수 0x337870 **+0x20** 한가운데 |
| serpen 툴팁 (`ARG_STR_RVA`) | 0x1657a70 | 함수 0x16576a0 **+0x3d0** | 함수 0x1656230 **+0x1840** |
| (참고) banpick `NAME_GET` (무검증 호출) | 0x22f9770 | 함수 0x22f92a0 +0x4d0 | 함수 0x22f85e0 +0x1190 |
| (참고) comptest `CTX_CLONE` (무검증 호출) | 0x2a7c610 | 함수 0x2a7c600 +0x10 | 함수 0x2a7c5c0 +0x50 |

- 덮어쓴 자리는 명령 중간일 수 있고, 트램폴린에 옮긴 12~15B 가 명령을 자르거나 rip 상대 주소를 담고 있으면 그 경로를 타는 순간 오동작한다. 그 함수가 실제로 불리는지에 따라 **즉시 크래시 / 나중 크래시 / 우연히 무사**가 갈린다 — 어느 쪽인지는 미검증(게임 실행 금지 과업).
- 두 지점은 원래 같은 "체인 설치" 패턴이다(다른 모드와 같은 함수를 공유하려고 원본 대조를 일부러 뺐다 — `ui_inject.rs:333` 주석, serpen `lib.rs:733-735` 주석). 그래서 버전이 맞을 때는 문제가 없고 **핫픽스 때만** 터진다.
- 같은 패턴의 serpen UI 로더 훅(`lib.rs:736`)은 `USE_UILOADER_HOOK=false`(`:655`)라 이번엔 해당 없음.

## 4. 진입부 가드가 실제로 얼마나 막아 주나 (모의 실험)

`_tmp\hotfix_exposure\guard_sim.py`: 각 모드 매니페스트의 0.6.5 RVA(무시·비RVA 제외)와 그 자리 12B 를 0.6.4·0.6.3 exe 의 **같은 RVA** 에 대 봤다 = "주소가 낡았을 때 12B 대조가 통과해 버리나".

| 모드 | 대상 RVA | 그중 범용 push8 프롤로그 | 0.6.4 exe 에서 12B 일치 | 0.6.3 exe 에서 12B 일치 |
|---|---|---|---|---|
| `tfm2_level_cap` | 2 | 0 | 0 | 0 |
| `tfm2_champion_exclude` | 1 | 0 | 0 | 0 |
| `banpick_view_plus` | 22 | 8 | 0 | 0 |
| `tfm2_comptest_unlock` | 69 | 17 | 0 | 0 |
| `tfm2_elemental_serpen` | 18 | 11 | 0 | 0 |
| `tfm2_champ_pos_lock` | 4 | 3 | 0 | 0 |
| `ui_kit`(training·facility·custom_tier·pos_lock 이 씀) | 11 | 4 | 0 | 0 |
| `tfm2_detail_tactics`(창 218 + 반환·분기 주소) | 321 | 0 | 0 | 0 |

- ⟹ **전면 재배치형(H2)에서는 12B 대조가 전부 막는다.** 위험은 "대조 자체가 없는 곳"(§3)과 "대조를 통과한 훅 안의 무검증 호출"(H1)에만 남는다.
- 범용 push8(`55 41 57 41 56 41 55 41 54 56 57 53`)은 0.6.5 `.text` 에 **82,248번** 나온다(comptest 소스 주석 `:394` 은 0.5.3 에서 66,635곳). 16바이트 정렬 무작위 표본에서 우연히 맞을 확률 = **0.24%** — 낮지만 그 프롤로그만으로는 "그 함수"인지 알 수 없다.
- team_sync 의 "첫 1바이트만" 확인(facility·custom_tier)은 무작위 표본에서 build/send 5.0%·drop 15.4% 가 통과한다. 다만 그 앞의 구조 가드 ①(서버 루프 +0x90 == state)이 먼저 막고, 0.6.4·0.6.3 exe 에서는 세 함수가 동시에 통과하지 못했다.

## 5. 지금 꺼져 있는 설치 모드 — 켜면 노출되는 것

| 모드 | deps | 지금 | 켜면 (핫픽스 때) | 근거 |
|---|---|---|---|---|
| `tfm2_lane_tactics` | `>=0.6.5, <0.6.6` | OFF(상세전술 검수 때 끔 — `REPORT\tfm2_detail_tactics\01_인게임검수_정리.md:104`) | 훅 1개 범용 push8 12B 대조 → 대개 미설치 | `src/lib.rs:24-26·403` |
| `tfm2_tactics_verify` | `>=0.6.5, <0.6.6` | OFF | EXE SHA256 불일치 → "아무것도 설치하지 않음" | `src/lib.rs:42-47` |
| `tfm2_ai_analysis_verify` | `>=0.6.5, <0.6.6` | OFF | 기준 SHA 가 0.6.4(`a1d05045…`)라 0.6.5 에서도 무장 안 함 | `src/generated.rs:2` · `src/engine.rs:401` |
| `tfm2_flow_capture` | `>=0.6.0` | OFF | 무관(stable 재작성·RVA 0) | 매니페스트 노트 |
| `tfm2_meta_item_delegate` | `>=0.6.0` | OFF | 무관(RVA 0) | 매니페스트 노트 |
| `legacy_save_patcher` | `>=0.5.0` | OFF | 무관(소스 `legacy_save_patcher_stable\src` 에 RVA·코드쓰기 없음 — 배포 dll 09-16 과 소스 일치는 미검증) | grep |
| `tfm2_ibhook_probe` | `>=0.1.0` | OFF | 무관(stable 훅 프로브) · 진단용이니 켜지 말 것 | `src/lib.rs:1-26` |

그 밖 46개 폴더는 **꺼져 있고 deps 상한도 0.6.5 미만**이라 정식·핫픽스 어느 쪽이든 로드되지 않는다(목록 = JSON `not_loaded`). mod_info 가 없는 폴더 6개(`base_unpacked`·`release`·`community_reaction_mod` 로컬 미러·`tfm2_banpick_illust`·`tfm2_ui_probe`·`tfm2_view_plus`)는 로더가 모드로 안 본다.

## 6. 패치날 절차 (권고)

1. **업데이트가 왔으면 게임을 켜기 전에** `F:\claude\tfm2mods` 에서 `python -X utf8 MIG\patchday\run_patchday.py` — 0단계 detect 가 `base_version.txt` 와 exe sha 로 정식/핫픽스를 가른다(핫픽스면 라벨 `0.6.5_<sha앞8>`). SUMMARY §8 에 이 표가 붙는다.
   - 경계 성격(H1/H2)이 궁금하면: `python -X utf8 MIG\_patchday\_tmp\hotfix_exposure\pdata_shift.py F:\claude\tfm2\tfm2_0.6.5\TeamfightManager2.exe=<새 exe>` (몇 초. "같은 경계 %"·"첫 이동 RVA" 출력).
2. **정식 패치(0.6.6)면**: 그냥 켜도 된다 — 10종은 경고창과 함께 꺼지고 stable 5·콘텐츠 6만 돈다. 마이그는 `MIG\README.md` 절차대로.
3. **핫픽스(0.6.5 그대로)면**, 게임을 켜기 전에:
   1. `config\game\mods.json` 을 백업한다.
   2. `enabled_mods` 에서 `tfm2_comptest_unlock`·`tfm2_elemental_serpen`(필수)·`banpick_view_plus`(권장)를 뺀다. 게임이 꺼진 상태에서 고쳐야 한다 — serpen 은 init 에서 바로 덮어쓰므로 **모드 메뉴에 들어가기 전에** 터질 수 있다.
      - 근거: 상세전술 검수 절차가 같은 방식으로 모드를 켜고 끈다(`REPORT\tfm2_detail_tactics\01_인게임검수_정리.md:104` `activate_trial.py`). ⚠옛 기록(메모리 `tfm2-replay-runbook.md:90`, 06-18)은 "시작 시 덮어써져 무효"라고 했다 → 켠 뒤 모드 메뉴에서 실제로 꺼졌는지 확인할 것.
      - 대안: 모드 폴더 이름을 바꾼다(선례 `mods\_tfm2_ui_inject.disabled`). 폴더 이름이 mod id 라서 목록에서 다른 모드가 된다.
   3. 나머지는 켜 둬도 크래시 위험이 낮다. 다만 pos_lock·champion_exclude·level_cap 은 **조용히 안 걸린다** — 마이그 전에는 시즌 패치(신챔프 추가)·레벨업 진행을 피하는 게 좋다.
   4. 마이그·재빌드·재배포(deploy-verify PASS)까지 끝나면 뺀 모드를 다시 넣는다.
4. 켜 둔 모드가 꺼졌는지 확인할 로그(전부 `<게임>\mods\<모드>\` 또는 `E:\claude\tfm2\logs\game_mods\<모드>\`):
   `tfm2_level_cap.txt`(`byte mismatch`) · `champion_exclude.txt`(`HOOK FAIL: prologue mismatch -> install SKIPPED`) · `detail_tactics.log`(`게임 EXE SHA256 불일치`) · `champ_pos_lock_*.txt` · serpen `serpen_probe.txt`(`prologue mismatch — 훅 미설치`) · banpick `showcase_log.txt`(`hook A FAIL`·`geom patch SKIP`). ⚠대부분 디버그 로그가 꺼져 있어(예 `Spectator_Chat` `DBG=false`) 흔적이 안 남을 수 있다.

## 7. 미검증·사람 판단

- ⬜**크래시가 실제로 나는지**는 실행해 보지 않았다(게임 실행 금지). §3 은 "확인 없이 덮어쓴다"는 코드 근거 + "그 주소가 다른 함수 중간"이라는 모의 실험까지다.
- ⬜§1.3 의 `+`·`-`·밑줄 변형은 semver 크레이트 규칙과 exe 문자열에서 끌어낸 **추정**이다. 과거 핫픽스는 7번 모두 문자열을 안 바꿨다(실측).
- ⬜deps 불일치 모드가 `enabled_mods` 에 남는다는 것은 백업 비교에서 나온 **간접 근거**다. 재배포 후 자동 복귀 여부는 기동 후 모드 메뉴로 확인할 것.
- ⬜`mods.json` 을 게임 밖에서 고쳐도 먹는지: 최근 절차(activate_trial.py)는 먹었다고 하고, 06-18 기록은 아니라고 한다. 핫픽스 때 쓰기 전에 한 번 확인 필요.
- 사람 판단: comptest·serpen 의 무검증 설치를 고칠지(진입부 대조 추가 또는 상세전술처럼 EXE 신원 게이트) — 이번 과업은 소스를 바꾸지 않았다. 고친다면 `tfm2_detail_tactics` 의 `micro.rs:216-218`(exe 를 읽어 SHA256 비교)이 가장 강한 표본이다.
- 참고(이번 범위 밖): comptest `FN_DD_SETOPT_RVA` 의 기대 프롤로그(`:3442`, `55 41 57 …`)와 매니페스트 0.6.5 바이트(`41 57 41 56 …`)가 다르다 — 그 경로가 지금 쓰이지 않으면 무해, 쓰인다면 이미 조용히 꺼져 있다(미확인).

## 부록 — 재현

```
cd F:\claude\tfm2mods\MIG\_patchday\_tmp\hotfix_exposure
python -X utf8 collect.py collected.json          # mods\* + 워크샵 mod_info·deps·dll·활성
python -X utf8 strscan.py "<게임>\TeamfightManager2.exe"   # 로더 문자열
python -X utf8 hfver.py <exe…>                     # exe 안 버전 문자열(0.5.x~)
python -X utf8 hfver2.py <exe…>                    # 〃 (0.4.x)
python -X utf8 pdata_shift.py <구exe>=<신exe> …    # 함수 경계 유지율
python -X utf8 guard_sim.py guard_sim.json         # 낡은 RVA 에 12B 가드 통과율
python -X utf8 vp_sites.py                         # 코드 쓰기 지점 ↔ 소속 함수
python -X utf8 dllstr.py dllstr.json               # 배포 dll ↔ 소스 가드 문자열
```
