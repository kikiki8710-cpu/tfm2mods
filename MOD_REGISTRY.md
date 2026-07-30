# MOD_REGISTRY — 모드 현황/버전대응 (검색 신뢰도 판정용)

> **목적**: 어떤 기능/후킹이 이미 구현됐는지 모드 소스를 grep할 때, **그 모드가 현행 게임버전에 대응하는 유지중 모드인지, 아니면 안 쓰는 구/폐기 모드인지**를 먼저 알기 위한 단일 표. 폐기 모드에서 "기능 없다/RVA 어긋난다"를 보고 헛다리 짚거나 재구현하지 않기 위함.
>
> ⚠ **살아있는 정본(authoritative)은 아래 3곳** — 충돌 시 이들이 우선. 이 파일은 그 위에 올린 스냅샷 인덱스(2026-07-01 작성):
> - **현행 게임 버전·활성 모드(T1)** = `MEM\CURRENT.md` (단일 출처)
> - **현행 RVA 대응 모드** = `MIGRATION.md §7` 표(마이그 대상)
> - **최신 마이그 이력** = `MEM\tfm2-0.5.2-migration.md` 등 버전별 메모리
>
> 하드코딩 RVA에 의존하는 모드는 마지막 rebuild 이후 패치가 있었다면 훅이 어긋나 있을 수 있음(= 그 모드 소스의 RVA·후킹동작을 "현행 기준"으로 신뢰 금지 — 현행 버전은 CURRENT.md 확인).

## 판정 규칙 (에이전트용)
모드 소스에서 매치가 나오면 그 모드의 티어를 함께 보고할 것.
- **T1/T2 매치** → 현행 대응. 그 구현/후킹을 신뢰하고 인용.
- **T3(프로브/실험) 매치** → 로직/오프셋 재사용 참고용. "배포된 프로덕션 동작"으로 단정 말 것.
- **T4(구/폐기) 매치** → **신뢰 금지**. RVA/후킹이 어긋났을 수 있음. "기능이 없다"는 결론도 T4에서만 나오면 무효 — T1/T2/문서에서 재확인.
- 티어가 애매하면: `게임 mods\<id>\*.dll` 시각이 **2026-06-24(마지막 패치) 이후**면 현행일 확률 높음, 이전이면 구버전 의심.

---

## T1 — 현행 프로덕션 (현행(CURRENT.md 기준) RVA 마이그 대상, 신뢰)
CLAUDE.md §1 활성 + MIGRATION §7 표에 등재. 패치마다 migrate_rva.py 갱신.
★**2026-07-22 유저 지시: 패치 마이그 대상 = 아래 8종 한정.** (종전 T1 ~~`tfm2_item_editor`·`tfm2_scrim`~~=마이그 대상 제외→T4 동결. `tfm2_mod_scroll_fix`·`tfm2_comptest_unlock` 등 그 외도 제외.)
| MOD_ID | 역할 | 하드코딩 RVA 의존 / 비고 |
|---|---|---|
| `tfm2_ai_adjust` | AI 판단 config 인게임 편집 모드(설정편집기.exe 동봉). 활성훅=retreat 진입캡처/condgate/movepri/fc59a0 | 有(judge RVA 다수 + disc19/oi_* byte-patch — MIGRATION §7 표) |
| `tfm2_item_tactics` | 개인전술 모드템 주입(관전 팀 실주입·자연 빌드업) | 有(BUY_ITEM 0x1f01090 계열·로더 체인후킹). 정본=`MEM\tfm2-item-slot-count.md` §2c |
| `community_reaction_mod` | 커뮤 반응 + 경기 하이라이트 수집·플옵/국제전 인식 | SDK+ClientDatabase raw 오프셋(패치마다 재도출). ⚠실 로드경로=워크샵 content\3009300\3738958482 dll(게임 mods\폴더=출력전용) |
| `tfm2_banpick_illust` | 밴픽 픽 완료 슬롯 배경 일러스트 + **쇼케이스 연출 대체(v1.2.0 `src\showcase.rs`, 07-25)** + **버프/너프 이름색칠·8각 레이더(v1.3.0 `src\patchviz.rs`, 07-26 — daram2 §6b 재현)** | ~~**순수 SDK(RVA 0)**=SDK 교체+재빌드만~~ → **有(v1.2.0부터 훅 3=0x11e2370/0x11f9030/0xfdabe0 + FFI 14종 + 기하패치 12사이트 — `MIGRATION.md §7.2-B`)**=패치 시 마이그 필수. ⚠**v1.3.0부터 사이즈가드(1.3MB) 초과 모드(dll 2,798,080B)** ⇒ 빌드 = rustc 직접+Copy-Item(scrim·ai_adjust와 동일 예외). 정본=`MEM\tfm2-banpick-illust-mod.md` |
| `tfm2_draft_overlay` | 밴픽 메타 팝업 오버레이 | 有(asset-get copy 분화 0xeb17d0/0x40f3d0 등). 정본=`MEM\tfm2-draft-overlay-mod.md` |
| `tfm2_elemental_serpen` | 세르펜 오브젝티브(속성버프·색렌더·스프라이트) | 有(리졸버 seam 0x13c0e90 등). 정본=`MEM\tfm2-serpen-objective-system.md` |
| `tfm2_fog_damage_fix` | 시야 밖 노데미지 해제(v0.3.0=시야 완전 무시) | 훅 0·byte-patch 5곳=패치 시 RVA 재핀만. ⚠적용 시 sim≠바닐라. 정본=`MEM\tfm2-vision-fog-in-ai.md` |
| `Spectator_Chat` | 인게임 가짜채팅 + 창드래그위젯 | 라이브 raw 오프셋=패치마다 재도출. 정본=`MEM\tfm2-spectator-chat-mod.md` |

## T2 — 현행 활성/진행중 (데이터·경량RVA·네이티브SDK, 대체로 신뢰)
0.4.14 시기 작업/배포. RVA 의존이 적거나(UI데이터/오프셋스캔) 네이티브 SDK.
> ★2026-07-22: `tfm2_item_tactics`·`Spectator_Chat`·`community_reaction_mod`·`tfm2_draft_overlay`·`tfm2_elemental_serpen` → **T1 승격**(마이그 대상 8종, 위 T1 표 참조 — 구 T2 행 삭제).

| MOD_ID | 역할 | 비고 |
|---|---|---|
| `tfm2_aim_lead` | 스킬샷 조준 리드(magic_knight) | 진행중(Phase A 완료, 사거리게이트 남음). ⚠**0.5.0 미대응**(dll 06-27) |
| `Match_Info_Exporter` | 경기정보 export(네이티브 SDK) | 동작(0.4.14 기준). ⚠**0.5.0 미대응**(dll 06-28) |
| `packet_interceptor` / `Packet_Interceptor` | 클라↔서버 패킷 MITM | 네이티브. ⚠**0.5.0 미대응**(dll 06-27) |
| `tfm2_comptest_unlock` | 구성테스트 unlock | 0.5.1 빌드(07-18)·현재 비활성. **마이그 대상 제외(유저 지시 07-22)**. 정본=`MEM\tfm2-scrim-comptest-port.md` |
| `tfm2_fog` | 렌더 fog | ⚠**0.5.1 미마이그**: dll 07-08(0.5.0 시대)·CURRENT "배포본 오염·재빌드 필요"·fog DISP RVA 0.5.1=0x9f40a0 이동 — 재빌드+RVA 스왑 전 신뢰 금지 |

> ⚠**[2026-07-08 감사] 비활성 3모드 = 활성화 전 hotfix SDK 재빌드 필수.** `tfm2_aim_lead`(06-27) / `Match_Info_Exporter`(06-28) / `packet_interceptor`(06-27) 산출물은 **0.5.0 출시 이전** 빌드 ⟹ 현행 `sdk_050_hotfix` SDK일 수 없음(시각만으로 확정). 현재 `config\game\mods.json` `enabled_mods` 에 없어 **미로드 = 실사용 위험 0**이나, 켜려면 **재빌드 선행**(0.4.14 SDK ABI → 로더 ABI 게이트/크래시 위험) + 하드코딩 RVA 보유 모드(aim_lead)는 RVA 재마이그도 필요. 상세=`MEM\tfm2-0.5.0-migration.md §15.7-B`.
> ✅반면 **배포 중인 활성 10모드는 전부 hotfix SDK 정합 확인 완료**(PE `.text` 해시 전수 감사 10/10 PASS, 오염 0건) = `MEM\tfm2-0.5.0-migration.md §15.7-A`.

## T3 — 프로브/실험 (로직·오프셋 재사용 참고용, 프로덕션 아님)
런타임 탐색·오프셋 도출용. "배포된 확정 동작"으로 인용 금지.
`Plan_Probe`, `Scrim_Probe`, `UI_Probe`, `sim_probe`, `crm_probe`, `ui_offset_probe`, `tfm2_inj_probe`, `tfm2_ai_banpick_probe`, `save_probe_test`, `plan_perf`, `sylas`, `sylas_art`, `sylas_hijack`, `draft_helper`(egui 폐기 — 밴픽도우미는 TFM2.gg 대시보드 탭으로 이관), `tfm2_move_guard`, `tfm2_move_capture`, `tfm2_panic_probe`, `tfm2_active_probe`(이상 4종=07-05~08 프로브류, 게임 mods\에 실배포돼 있으나 프로덕션 아님 — 2026-07-18 등재)

> ★2026-07-22: `tfm2_banpick_illust`·`tfm2_fog_damage_fix` → **T1 승격**(마이그 대상 8종, 위 T1 표 참조 — 구 T3 행 삭제. 상세 이력=CURRENT.md·각 정본 메모리).

| MOD_ID | 역할 | 비고 |
|---|---|---|
| `tfm2_view_plus` | daram2 뷰플러스 **9종 통합 재구현**(2026-07-09~10 작업) | **순수 SDK**. 밴픽 일러스트는 ⬜미구현으로 중단 → 그 기능만 `tfm2_banpick_illust`로 분리 신설(07-18). `API_NOTES.md`에 mod_api 심문 결과 정리=참조가치 있음 |
| `tfm2_level_cap` | 인게임 챔피언 **레벨 상한 상향**(cfg의 최대 레벨·need_exp CSV). ~~**데이터 전용·DLL 없음**(`mod.override_info` merge로 game_setting 3벌 교체)~~ → ★**정정(v2.0.0부터·2026-07-23)**: **네이티브 DLL + 런타임 트램폴린 2사이트**로 need_exp Vec 강제 교체(merge 단독은 불가·`mod.override_info={}`) | ~~**RVA 0 = 마이그 대상 아님**~~ → ★**정정(2026-07-31)**: **비-T1인 것은 맞으나 하드코딩 RVA 2개 보유 ⟹ 패치마다 재핀 필수**. 현행 = **0.5.3 마이그·배포완 v2.1.0**(`RVA_LEN_LOAD 0x12c5b44`·`RVA_UI_CMP 0x95a359`·dll 198,144B @07-31 00:55:12) / ⬜인게임 미검증·⬜zip. 정본=**`MODS\MIGRATION.md §7.3 §16`**(0.5.3)·§7.2-A12 §6(구현) + `REPORT\tfm2_level_cap\` + `MEM\tfm2-asset-override-merge.md` §6 |
| `tfm2_ai_adjust_2` | AI 판단 조정 **린 재설계 Phase 1**(2026-07-19·0.5.1): [A]원본 상수 byte-patch 29사이트 [B]movepri 원본실행 wrap 후퇴판단 [C]recall(fc59a0)만 재현. dll 194,560B(원본의 5.6%)·진단인프라/UI주입/선수별 제거 | 배포완·⬜인게임 검증 대기·⬜dd_*/ec_* 미커버(Phase 2). ⚠**원본 `tfm2_ai_adjust`(T1)와 동시 활성 금지**(이중후킹+byte-patch 중복—전환 시 원본 비활성 필수). 정본=`MEM\tfm2-ai-adjust-2-redesign.md` |
| `tfm2_transfer_tweak` | 이적 수락 문턱 하향(1.2~2.25→기본 1.1~1.8) + 사유9 "이적 생각 없음"을 초고액(문턱+unwilling_surcharge 0.8)배 제안으로 관철. 셀러팀 판정 불변·랜덤 주입 없음(결정론 유지)·cfg=dll 옆 `transfer_tweak.cfg`(자동생성, 10키) | **비-T1(마이그 대상 아님)** ⚠단 **하드코딩 RVA 다수**(rdata 0x3835560 패치·disp 재지향 5사이트·detour 0x1d15e90) = 패치 시 전부 재핀 필요. 배포완·⬜인게임 미검증(2026-07-24·0.5.2). 정본=`MEM\tfm2-transfer-negotiation.md` §모드 |
| `tfm2_banpick_order` | 밴픽 진행 순서(밴/픽 턴 시퀀스) **cfg 재정의** — 밴↔픽 인터리브 자유 배치 **+ ★팀 순서 지정 + 단계 배너 + 자체 밴픽 AI(`ModDraftScoreHook`)**. ~~훅3~~→~~훅5~~ → **훅7** = phase 2종 전체대체(A `0x1cd9380`/B `0x1d04120`, ~~MY_MSI 게이트~~**제거=전 경기 적용**)+적용기 shim(C `0x11e2140`)+**턴 오라클 전체대체(D′ `0x1d07cf0`)**+라인업 검증 스킵(E `0x11cedb0`)+**권위 커밋기(F `0x1d075d0`)**+AI파리티 바이트패치 2사이트(`0x1c04389`/`0x1c07938`)·cfg 검증 실패=바닐라 폴백 | **비-T1(마이그 대상 아님)** ⚠단 **하드코딩 RVA ~~3종~~→~~5종~~→7종=패치 시 재핀 필요(정본=`MIGRATION.md §7.2-C`)**. ✅**v1.0.0 인게임 검증완·릴리스완(2026-07-29·0.5.2, dll 2,671,104B·zip 843,291B·deploy-verify 7 PASS)** ⚠**사이즈가드 초과=rustc 직접 빌드 예외**. RE 정본=`ANA\discovered-banpick-ai.md §16`+**§17i·§17j** / 모드 정본=`MEM\tfm2-banpick-order-mod.md §11` |
| `tfm2_mod_order` | 타이틀 "모드 관리" 팝업(`title.ui #mods_popup`)의 모드 목록 **표시순서 변경** — 행클릭/↑↓=선택(선택 이름 노랑)·Ctrl+↑↓=이동(자동저장)·홀드 자동반복·화면밖 자동스크롤. 영속=`mod_order.txt`(전체 mod_id 순서). ★게임 기본표시순=**mod_id ASCII 오름차순**(순서 저장필드 없음) | **비-T1·순수 SDK(하드코딩 RVA 0)** = 패치 대응 = **SDK 재빌드만**(scroll_fix·crm과 동일 최저티어, RVA 재핀 불필요). v0.1.0 구현·인게임 검증완(07-27)·릴리스완(0.5.2). 정본=`MEM\tfm2-mod-order-mod.md` |
| `tfm2_banpick_showcase` | 밴픽 쇼케이스 연출 **전체 대체**(밴 취소선/2분할+픽 중앙 비행 → 가로형 커스텀 카드 폭520·높이 아트비율 ~408 + `tfm2_banpick_illust` 아트팩 일러) | ⛔**07-25 `tfm2_banpick_illust` v1.2.0에 통합·별도 모드 철수(유저 지시)** — 게임 `mods\` 폴더·0.5.2 릴리스 zip **삭제**(이중 훅 방지). **소스 폴더는 `FFI_CONTRACT.md`(RE 계약 정본) 보존용 유지·이동 금지.** RVA 등재=§7.2-B(illust 소속 이관=T1 자동 마이그). ~~✅배포·인게임 검증완(07-25, dll 182,272B)·릴리스완~~=통합 전 이력. 정본=`MEM\tfm2-banpick-showcase.md`(RE 유효) + `MEM\tfm2-banpick-illust-mod.md` §쇼케이스 통합 |

## T4 — 구/폐기 (신뢰 금지 — RVA·후킹 어긋남 가능)
오래 미유지. 검색에서 나와도 "현행 동작"으로 보지 말 것.
| MOD_ID | 최종 | 비고 |
|---|---|---|
| `tfm2_item_editor` | 0.5.0_3 (0.5.1 미마이그) | **구 T1 → 마이그 대상 제외·동결(유저 지시 2026-07-22)**. 아이템 price/stat 런타임 편집·RVA 有(STAT_FN/PER_ITEM/SUM). 0.5.1 마이그 잔여 ⬜=폐기(DONE.md). RVA=구버전 기준=신뢰 금지 |
| `tfm2_scrim` | 0.5.0_3 (0.5.1 미마이그) | **구 T1 → 마이그 대상 제외·동결(유저 지시 2026-07-22)**. 스크림/구성테스트+구 `tfm2_ui_inject` 합병(`mod uinj`). 0.5.1 마이그 잔여 ⬜=폐기(DONE.md). RVA=구버전 기준=신뢰 금지 |
| `plan_reimpl` | 07-02 | plan_v2 AI judge 완전대체(condgate/movepri/recall/dd7700/retreat). **2026-07-02 소스·배포 영구삭제**(plan-reimpl-legacy 메모리에 RE사실·RVA 보존; RVA는 tfm2-0.4.14-migration.md·MIGRATION §7에도 보존). AI조정 기능은 `tfm2_ai_adjust`(T1)로 계승 |
| `tfm2_fight_decision_guard` | 05-29 | 초기 실험, plan_reimpl로 흡수 |
| `tfm2_support_roam_guard` | 05-30 | 초기 실험 |
| `tfm2_4items` | 06-17 | 아이템칸4 실험(정본=tfm2-item-slot-count 메모리) |
| `tfm2_meta_champion_tiers` | 06-25 | 메타(TFM2.gg 계열) |
| `tfm2_meta_item_delegate` | 06-25 | 메타(TFM2.gg 계열) |

## 서드파티/SDK/공용 (모드 아님)
- `TeamfightManager2Mod`, `sdk_0414_new`(정식 0.4.14 SDK — SDK모드 빌드시 이걸로), `mod_sdk`(⚠`mod_sdk\0.4.14`는 비공식 오빌드 함정=tfm2-native-mod-loader-abi 참조)
- `TFM2.gg-upstream` (서드파티 메타툴 소스, upstream 0.4.11 동결)
- `ui_kit` (공용 UI 모듈 — 복사말고 `#[path=...]` import)
- `ult_replicas` (에셋 전용 폴더 — 코드 없음, 게임 mods\에 존재)

---
_스냅샷: 2026-07-02(plan_reimpl 강등·ui_inject 합병 반영) → 2026-07-18(deploy-verify 전수점검: 고아 9종 등재 — T2 4종 draft_overlay/elemental_serpen/comptest_unlock/fog·T3 4종 프로브류·기타 ult_replicas + T1 헤더 "0.4.14" stale 정정) → 2026-07-19(`tfm2_ai_adjust_2` T3 등재) → 2026-07-21(`tfm2_fog_damage_fix` T3 등재) → **2026-07-22(유저 지시: 마이그 대상 8종 한정 — T1 재편·item_editor/scrim T4 동결·T2 5종/T3 2종 T1 승격)** → 2026-07-24(`tfm2_transfer_tweak` T3 등재 — 비-T1·RVA 재핀 필요) → 2026-07-25(`tfm2_banpick_showcase` T3 등재 — 비-T1·RVA 재핀 필요·인게임 검증완) → **2026-07-25(showcase → `tfm2_banpick_illust` v1.2.0 통합·철수, illust=RVA 보유로 비고 정정)** → 2026-07-26(`tfm2_banpick_order` T3 등재 — 비-T1·RVA 3종 재핀 필요·⬜인게임 미검증) → 2026-07-27(`tfm2_mod_order` T3 등재 — 비-T1·순수SDK·RVA0·SDK재빌드만·인게임 검증완·릴리스완) → **2026-07-29(`tfm2_banpick_order` 훅3→훅5·RVA 5종 재핀 대상·인터리브 인게임 검증완 반영 → 같은 날 **v1.0.0 릴리스**: 훅7·RVA 7종·팀순서 지정 실현·자체 밴픽 AI·rustc 직접 빌드 예외)**. 패치/신규모드 시 T1(MIGRATION §7)·T2 갱신. 티어 재판정은 게임 mods\ dll 시각 + CLAUDE.md §1로._
