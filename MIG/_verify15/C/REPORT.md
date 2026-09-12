# 15차 배치 C 보고 — `30`~`34` (게임 0.5.8) · 2026-09-13

기계 적용분 = `patch.json` (정정 52 · ev상향 82 · brief_errors 5) — `applypatch.py 15 --only C --dry` **52/52 · 82/82 · 실패 0 · 동작 변경 0**.
오라클 = `oracle/C15_o1.rs`(#31·#32·#34, 6,882행) + `oracle/C15_o2.rs`(#34 양성 주입 90행) — **MISMATCH 0**.

## 0. 신선도
- 착수 시 `dossierfresh.py 15 C` = **FRESH**. patch 제출 직전 재실행 = **STALE**(`specs20_v3.json` 03d55ff8 → 14948de7).
  차이를 07:57 백업(`_spec/specs20.bak-2026-09-13-0757-addspec.json`)과 대조 — **20~38 의 `exe` 칸 구조 변경(값 동일) + 39번 spec 추가뿐**, 담당 5함수의 다른 칸은 변화 없음.
  `DOSSIER_C.md`·`spec_*.md` 는 08:00:49 그대로. dry-run 이 현행 v2 에 52/52 통과했으므로 patch 는 현행 정본 기준으로 유효하다.

## 1. 함수별 결과

| # | 정정 | ev↑ | behavior_change | 오라클 | 요지 |
|---|---|---|---|---|---|
| 30 objective_is_damaged | 12 | 9 | 0 | 재료 부재(pub 경로 0) | G13 실오류 1(51739→51750) + mem/consts 근거의 IR 줄번호 11건 어긋남. 오프셋·태그·vtable 슬롯 전부 tcx 일치 |
| 31 v3_epic_formation_role | 8 | 20 | 0 | 5,580행 MISMATCH 0 | G13 실오류 1(64821→64841) + 줄번호 7건. 로직(Split14/Split131 우선순위·폴백) 실행 확증 |
| 32 serpen_giveup_chat_reason | 3 | 18 | 0 | 486행 MISMATCH 0 | G13 실오류 1(53081→53085) + 줄번호 2건. tutorial 9종·live_list·count 경계 실행 확증 |
| 33 v2_obj_restore_safe | 20 | 13 | 0 | 재료 부재(pub 경로 0) | G1(consts[1]↔515 혼입 제거)·G16 P1(self 소거 어휘)·P3(i 1..6) 해소 + 줄번호 11건. Blackboard stride 744 확정 |
| 34 has_line_defense_threat | 9 | 23 | 0 | 816+90행 MISMATCH 0 | G13 실오류 2(60443→60449 · 60448→60455) + 줄번호 7건. 양성(any 분기 true) 주입으로 실측 |

**합계: 정정 52(실오류 50 · 보강 2) / behavior_change 0 / 판정 반전 0.**
⚠줄번호 오류 밀도가 높은 이유: r7 명세의 IR 인용은 대부분 `#dbg_value` 줄을 건너뛰고 센 듯 실제보다 2~21줄 앞선다(예외적으로 #32 mem[0] 52989, #31 params[0] 64778~64779 는 정확). 오프셋·태그·극성·분기순서는 **5함수 전부 오류 0**.

## 2. 게이트 미해소(§4) 처리

| 게이트 | 항목 | 판정 | 근거 |
|---|---|---|---|
| G1 | [33] consts[1] 506↔515 | **실오류** | meaning 에 `1-team`(506)과 `tps*2 접힘`(515) 두 사실이 섞임 → 515 언급을 consts[2] 로 넘기고 `m13.ll:11430 sub nuw nsw i64 1, %12 !dbg 506` / aux 58259 로 재기술 |
| G10 | [33] open[0] self 소거 | **사실 서술** → notes 이관 요청(§4 산문) | `m13.ll:11386` define 인자 5개 · `!21306 DILocalVariable(name:"self", arg:1)` · 본문 self 접근 0건 |
| G13 | [30]/[31]/[32]/[34]×2 knobs.where | **실오류 5/5** (오탐 0) | 각각 `icmp` 실제 줄 51750 / 64841 / 53085 / 60449 / 60455 (IR 원문 인용 = patch evidence) |
| G16 P1 | [33] params[0] | **실오류(어휘)** | 명세는 "제거됨" 이라 적었으나 `paramrole.DROPPED` = `poison|인자로 안 넘어옴|인자에서 제거|인자 없음` 이고 `arg\d` 포함 시 제외 → "IR 인자에서 제거됨" 으로, `arg1` 표기 제거 |
| G16 P3 | [33] params i | **실오류** | sig.tcx 6인자 → i=1..6 (DISubprogram arg 번호와 동일) |
| G7 | [33] open[2] 과열림 | **부분 폐기** | `is_recent_visible` 은 shared 확정 → open 을 `is_ignored_well_enemy(fight_model.rs:754) 내부 미탐색` 으로 좁혀 달라(§4) |

## 3. 게이트가 없는 축(§4-b) 표본 대조

- **`mem` 오프셋 전수 61행**(30:11 · 31:9 · 32:10 · 33:14 · 34:17) — `tcxdict` 로 전부 대조, **불일치 0**. 실행: `python -X utf8 tcxdict.py <Type> <off>` / `--enum JungleType|GameMode|MorgardUseStrategy|LineType|Position|TutorialType|SerpenGiveUpReason|EntityType|ObjectPhase`, `divtable.py AbstractGame 0x40|0x1f0`. ev_up(→3) 45행 제출(이미 3인 행 제외).
- **`one_line`** 5건 — 전부 IR 과 일치(30: hp<stat_cached.hp 엄격 / 31: Gather·Split14·Split131 배정 / 32: my<=enemy→Outnumbered / 33: 150000 내 최근가시 적·tps*2 / 34: from_mid<-3000 ∨ count<-2 ∧ 근처 미니언 nearest==tower). 오류 0.
- **`callees`** — ①vtable 간접호출(get_game_mode 0x40 / get_entity_by_id 0x1f0)을 `divtable` 로 풀면 **ExpectedGame impl** 인데 명세 후보엔 Game/SingleLaneGame 만 실려 있고 ExpectedGame 이 빠짐(#30·#31·#32 공통, 자동 열거 누락). ②`team` → `game_view::ClientData::team` 등 3건은 `logic` 산문 잡음. ③#33 `distance_sq` 는 `Entity::distance_sq`(entity.rs:2157, aux !60088 = 3147 inlinedAt 507) 가 맞고 `utils::distance_sq` 는 아님.
  ⟹ **검사기 제안**: callees 후보 열거 시 `divtable` 결과 impl 을 자동 포함하고, 후보가 `game_view::` 인데 함수가 `game_ai::` 이면 잡음 플래그.
- **`closed`** [31] "799/802 결합 순서 column 부재" — 유효(IR and/or 외연은 오라클 5,580행으로 재확증).
- **빠진 행**: #34 `open[4]` 의 전제 — `EntityType::Minion` 은 **struct variant `Minion { info }`**(tcx `EntityType::Minion::info` Field, 오라클 컴파일 시 rustc E0164 실측) → `if let EntityType::Minion(info)` 표기는 배제. 남는 표기 불가 = `Minion { info }` 패턴 vs 헬퍼.

## 4. `open`/`notes` 산문 반영 요청 (patch 경로 없음 — 메인이 넣는다)

- **[30] open[0]** → **닫힘**: 호출처 2곳(m09.ll:29752·29881) 모두 `TeamPlan::repair_misunderstood_objective`(team_plan.rs:603~) 가 `update_objective_after_steal`(:910) 에 인라인된 본체. ①:665 `target_started = objective_is_damaged(..)` ②:686 `new_phase = select(%1508 || damaged, i16 259, 257)` — 259/257 = (phase Hunt(3)/Setup(1), with_battle=1) **추정**(ObjectPhase 태그 3=Hunt/1=Setup, MainObjective 페이로드 +1 phase/+2 with_battle). 검증법: team_plan 명세 담당이 %1508 정체·store 대상 확인.
- **[30] open[1]·open[2]** → **사실 서술 → notes**(get(0) 확정 / live_list 원소 = 엔티티 id).
- **[32] open[1]** 표기 불가 유지 + 줄 길이 산술 결과 첨부: L229=10자 `  } else {` ±0, L230=41자 = `    Some(SerpenGiveUpReason::StackAhead)` ±0 ⟹ **else-arm = StackAhead**(= `if my<=enemy {Outnumbered} else {StackAhead}` 형). 단 L228=40자는 `    Some(SerpenGiveUpReason::Outnumbered)`(42) 와 잔차 2 → 확정 보류. L227(43자)는 변수명 미상이라 산술 불가(재료 부재: 변수명).
- **[33] open[0]·open[4]** → **사실 서술 → notes**. **open[3]** → **닫힘**: Blackboard stride = **744B**(`tcxdict Blackboard` 744B · aux m13.ll:58268 `is_recent_visible(... dereferenceable(744) %39`). **open[2]** → `is_ignored_well_enemy` 만 미탐색으로 축소.
- **[34] open[3]** → **닫힘**: `AbstractGameWithCache::minions(team,pool)` = `top_minions[team] ++ mid_minions[team] ++ bottom_minions[team]`(_gcbc g15.ll:109587/109604/109621 gep 16/80/144) — 라인 무관 그 팀 캐시 전부. 오라클: `cache.mid_minions[enemy]` 에 꽂은 복제 미니언이 line=Top/Mid/Bottom 전부에서 검출. (live 필터는 캐시 생성 `AbstractGameWithCache::new` 소관 — 미탐색)
- **[34] open[4]** → 보강(위 §3 빠진 행). **[34] open[0]** from_mid/minion_count 의미: `_docs\*.txt` grep 0건(**재료 부재: rmeta 주석**) · 미탐색 = `_gcbc` Blackboard 갱신처(`from_mid` store 지점, g07.ll/g09.ll DWARF 만 확인).
- **[31] open[0..2]** 미탐색 유지(범위 밖). 참고 실측: `TutorialType::None` 에서 `line_exists` Top/Mid/Bottom 전부 true.

## 5. 실제로 실행한 것 (명령줄)

    cd /c/tfm2mods/MIG && python -X utf8 dossierfresh.py 15 C                       # FRESH(착수) / STALE(제출 직전, exe 칸 변경뿐)
    sed -n '51666,51800p' /c/tfm2mods/_gaibc/m15.ll ; sed -n '64777,64972p' m09.ll ; sed -n '52986,53100p' m05.ll
    sed -n '11386,11527p' m13.ll ; sed -n '58204,58285p' m13.ll ; sed -n '60400,60577p' m04.ll   # 본체 5 + aux 1
    grep -n -E '^!(58244|58250|...) = ' m15.ll   (각 함수 !dbg → inlinedAt 루트까지)
    python -X utf8 tcxdict.py ... (61행) · tcxq.py items/lines/grep · divtable.py AbstractGame 0x40/0x1f0
    sh _verify3/build.sh C:/tfm2mods/MIG/_verify15/C/oracle/C15_o1.rs && %TEMP%/tfm2_spanprobe/C15_o1.exe > oracle/C15_o1.tsv   # #31 5580 · #32 486 · #34 816 → bad 0
    sh _verify3/build.sh .../C15_o2.rs && C15_o2.exe > oracle/C15_o2.tsv                                                     # #34b 90 → bad 0
    python -X utf8 <scratch>/v15C/mkpatch_C.py && PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 15 --only C --dry

오라클 세팅 무결성: `setting_ok true (width/height 960000, tps 60, radius 10000)`, towers 16, minions 18(600틱). TLS 메모 함수 없음(세 함수 본문에 thread_local 호출 0).

## 6. 지시(도시에)·도구 오류 → `brief_errors` 5건
1. §4 G16 [33] 의 「소거 주장 0개」는 검사기 어휘(`DROPPED` 정규식 + `arg\d` 제외 규칙)를 도시에가 안 알려줘서 난 것 — 어휘 표를 §4 에 실어야 한다.
2. §1 `ev≥4(미실행)` 수치가 mem 행(상한 3, 실행으로 안 내려감)을 포함 — 실제 오라클 표적 수가 아니다.
3. §1 의 「pub 아니면 상위 pub 래퍼/형제 복제본」 — #30·#33 은 직접 호출자까지 전부 `in:` 가시성이라 pub 경로 0. 도시에가 미리 표시해 주면 탐색 시간을 아낀다.
4. §4-b callees: vtable 슬롯을 풀어도(ExpectedGame) 명세 후보에 그 impl 이 없어 patch 로 못 넣는다(자동 열거 누락).
5. (도구) `mkpatch.Patch.ev` 가 `/specs[i]/mem[j]` 2단 배열 경로에서 `group(3) is None` 검사로 ValueError — group(4) 로 고쳐야 한다. 배치C 는 스크래치에서 parse_path 판으로 대체.

## 7. 막힌 지점·소요
- 막힘: ①`mkpatch.ev` 버그(위 5) — 우회 5분. ②#32 L227~L230 줄 길이 산술 — 변수명 미상으로 L227 산술 불가, L228 잔차 2(같은 가설 2회 시도 후 중단, 남은 재료 = exe 디스어셈 — 단 외연 동일이라 그것도 못 가른다 = 표기 불가 유지).
- 오라클 #34 1차(o1)는 자연 발생 미니언의 nearest_enemy 가 전부 None/미니언 id 라 any 분기 양성이 0건 → o2 로 복제 주입(TEMPLATE ⑦)해 양성 14건 확보.
- 소요 ≈ 22분(정본 3문서 읽기 6 · IR/tcx 대조 8 · 오라클 5 · patch/보고 3).
