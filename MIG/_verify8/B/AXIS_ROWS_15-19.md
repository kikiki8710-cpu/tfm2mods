# 8차 배치 B — 축 ``consts[].kind` — 상수의 종류` · 함수 `15`~`19` (32행)

> `mkdossier.py --axis` 생성물. **칸을 하나도 줄이지 않았다** — 7차까지 지시 오류는 전부 「줄여 적은 자리」에서 났다.

| # | 함수 | idx | value | src_line | kind | meaning | ev |
|---|---|---|---|---|---|---|---|
| `15` | single_try_engage | 0 | 0 | 253 | 태그 | BattlePlanGoal 판별자 0 = TryKill (DISCR_EXACT=0 확인). 248행 다이브 분기에서도 같은 값 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| `15` | single_try_engage | 1 | 60 | 253 | 임계 | TryKill 의 두 번째 usize. 248행·253행 양쪽 동일. m13.ll 안의 다른 engage 계열 플랜 생성 지점(33846/33893/33985/34347/34396행 등)도 전부 60 — 공통 파라미터 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| `15` | single_try_engage | 2 | 1 | 249 | 임계 | 적팀 번호 계산 `1 - player.info.team` 의 1 (팀은 0/1) · 오라클 실행 확증(7차 배치D: 오라클 D7_o1 실행 확증 — `cache.iter_towers_without_nexus(1 - team)` 을 team 0/1 양쪽에서 돌려 **각각 8개, 8/8 전부 적팀(`Entity.team == Player(1-team)`), 좌표 유일 8**. 무결성 지표 towers=16 · twin 2/2 (TEMPLATE ② 오염 없음) — `1 - team` 의 극성이 실행으로 확정) | 2 |
| `15` | single_try_engage | 3 | 2 | 250 | 태그 | EntityType 태그 2 = Tower (dienum 확인). 최근접 엔티티가 타워일 때만 dive_tower 를 채운다 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| `15` | single_try_engage | 4 | -1 | 250 | 센티널 | 두 곳에서 쓰이는 None 니치값 — i8 -1(=255) = Option<TowerType>::None(dive_tower), i64 -1 = Option<SinglePlanBattle>::None(반환) · 오라클 실행 확증(7차 배치D: 오라클 D7_o1 실행 확증 — `Option<TowerType>::None` size=1 bytes=[255], `Option<SinglePlanBattle>` size=144 = inner size 144(니치라 오버헤드 0)이고 None 의 +0x0 8바이트를 i64 로 읽으면 **-1**. IR 의 `dereferenceable(144)`/`memcpy .. i64 144`/`store i64 -1, ptr %0` 과 완전 일치) | 2 |
| `15` | single_try_engage | 5 | 3 | 256 | 태그 | BattleSubPlanGoal 태그 3 = KitingBack → 플랜 미채택(None) · 오라클 실행 확증( BattleSubPlanGoal::KitingBack 실값의 태그워드 직독 = 3 (o1.txt)) | 2 |
| `15` | single_try_engage | 6 | 4 | 256 | 태그 | BattleSubPlanGoal 태그 4 = RunAway → 플랜 미채택(None) · 오라클 실행 확증( BattleSubPlanGoal::RunAway 태그워드 직독 = 4 (o1.txt) · o6.txt UPD 9행에서 update 후 실제 sub_goal 태그 4 관측) | 2 |
| `15` | single_try_engage | 7 | 7 | 256 | 태그 | BattleSubPlanGoal 태그 7 = End → 플랜 미채택(None) · 오라클 실행 확증( BattleSubPlanGoal::End 태그워드 직독 = 7 (o1.txt)) | 2 |
| `16` | max_range_nearly_can_use | 0 | -1 | 2399 | 센티널 | Option<Effect>::None 의 니치 값 — Effect.casting(CastingType,i32,범위 -1..3)이 -1 이면 이펙트 없음. Option::as_ref(core/option.rs:742)가 인라인된 형태. 동시에 (level-1) 계산의 add i64 %level, -1 로도 등장 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| `16` | max_range_nearly_can_use | 1 | 13 | 2409 | 태그 | EntityType::Champion 태그. 스킬/스킬2/궁 쿨다운 게이트는 '챔피언일 때만' 걸린다(비챔피언은 스킬 쿨 0 취급) · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| `16` | max_range_nearly_can_use | 2 | 2 | 2414 | 임계 | 스킬2 개방 레벨 게이트 — Entity::skill2_effect(entity.rs:1693)가 level>2 일 때만 Some 을 준다 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| `16` | max_range_nearly_can_use | 3 | 4 | 2421 | 임계 | 궁 개방 레벨 게이트 — Entity::ult_effect(entity.rs:1701)가 level>4 일 때만 Some 을 준다 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| `16` | max_range_nearly_can_use | 4 | 100 | 2403 | 임계 | Entity::radius(entity.rs:1515)의 퍼센트 기준 — radius*(radius_mult+100)/100. radius_mult==0 이면 곱셈 자체를 건너뛴다 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| `16` | max_range_nearly_can_use | 5 | 0 | 2398 | 임계 | range 누적 초기값(하나도 못 쓰면 그대로 반환) 겸 radius_mult==0 빠른 경로 비교값 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| `16` | max_range_nearly_can_use | 6 | 3 | 2402 | 태그 | EntityType::Nexus 태그 — 태그 0(None)과 함께 평타 쿨다운 게이트를 건너뛰고 곧장 사거리 계산으로 간다(Entity::attack_cooldown 에 해당 팔이 없음) · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| `17` | new | 0 | 0 | 749 | 태그 | BattlePlanGoal 태그 0 = TryKill — 채팅 방송의 유일한 조건. 동시에 모든 Option 필드의 판별자 0 = None · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| `17` | new | 1 | 3 | 750 | 태그 | Chat 태그 3 = Chat::Battle (DISCR_EXACT=3 확인). 아군에게 '전투 개시' 통보 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| `17` | new | 2 | 9223372036854775807 | 753 | 센티널 | i64::MAX — flee_die 초기값 센티널(도주사망 미기록) · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| `17` | new | 3 | -1 | 753 | 센티널 | 0xff 바이트 — dive_tower / main_objective 의 Option None 니치 센티널 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| `18` | v3_epicops_buff_window | 0 | 1 | 635 | 태그 | v3_epicops_repair_need 반환 태그 1 — '수리 필요 + 채팅까지'. objective=Repair 로 놓고 Chat::Repair 를 발화한다 | 4 |
| `18` | v3_epicops_buff_window | 1 | 2 | 635 | 태그 | v3_epicops_repair_need 반환 태그 2 — '수리 필요하지만 채팅은 없음'. objective=Repair 만 놓는다 | 4 |
| `18` | v3_epicops_buff_window | 2 | 7 | 637 | 태그 | MainObjective::Repair 의 태그값(dienum: 태그=variant 인덱스, 밀림 없음). 637줄·642줄 두 곳에서 저장 | 3 |
| `18` | v3_epicops_buff_window | 3 | 23 | 638 | 태그 | Chat::Repair 의 태그값(24B 열거형판 Chat) · 오라클 실행 확증( Chat::Repair(0) 실값의 태그바이트 직독 = 23 (o1.txt)) | 2 |
| `18` | v3_epicops_buff_window | 4 | 25 | 658 | 태그 | Chat::SerpenSetup 의 태그값 · 오라클 실행 확증( Chat::SerpenSetup(0) 태그바이트 직독 = 25 (o1.txt)) | 2 |
| `18` | v3_epicops_buff_window | 5 | 21 | 674 | 태그 | Chat::Press 의 태그값. v3_press_chat_line 이 None 일 때(최초 압박 선언) 선택. 줄번호 = is_none 판정 **epic.rs:673**, Chat 구성 **674/676**. ⚠인라인 체인에 섞여 보이는 682 는 **`core/src/option.rs`** 줄번호이고, epic.rs:682 자체는 함수 꼬리(lifetime.end + 공통 출구)다 · 오라클 실행 확증( Chat::Press(LineType::Top,0) 태그바이트 직독 = 21 (o1.txt)) | 2 |
| `18` | v3_epicops_buff_window | 6 | 22 | 676 | 태그 | Chat::PressChange 의 태그값. v3_press_chat_line 이 이미 Some 일 때(압박 라인 변경) 선택. 줄번호는 is_none 판정 **epic.rs:673** 이고 Chat 구성은 **674/676**. ⚠인라인 체인에 보이는 682 는 **`core/src/option.rs`** 줄번호이며, epic.rs:682 자체는 함수 꼬리(lifetime.end + 공통 출구)다 · 오라클 실행 확증( Chat::PressChange(LineType::Top,0) 태그바이트 직독 = 22 (o1.txt)) | 2 |
| `18` | v3_epicops_buff_window | 7 | -1 | 665 | 센티널 | Option<LineType> 의 None 니치값(0xFF). ① group_line==-1 이면 즉시 false 반환 ② v3_press_chat_line==-1 이면 Press vs PressChange 를 가른다. v3_epic_group_line 반환의 IR range 는 [-1,3) = {None, Top, Mid, Bottom} · 오라클 실행 확증( Option<LineType>::None 니치바이트 직독 = 0xff (o1.txt) + v3_epic_group_line 이 실제로 None 을 내는 케이스 관측(JungleOnly 전 mu, First/Bottom+Split14) (o2.txt)) | 2 |
| `19` | best_jungle_goal | 0 | 0 | 807 | 태그 | jungle_camps[0] = JungleType::Rhino — ★`JungleType` 의 **판별자**다(임계가 아니다). 오라클 D7_o1 실측 태그 = Rhino 0 · Mushroom 1 · Bee 3 · Stump 2 (dienum JungleType 0=Rhino). 같은 값 0 이 ① get_game_mode 반환 tag==0(=Some) 판정 ② player.info.team==0(=블루 진영) 판정에도 쓰인다 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| `19` | best_jungle_goal | 1 | 1 | 807 | 태그 | jungle_camps[1] = JungleType::Mushroom — ★`JungleType` 의 **판별자**다(임계가 아니다). 오라클 D7_o1 실측 태그 = Rhino 0 · Mushroom 1 · Bee 3 · Stump 2 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| `19` | best_jungle_goal | 2 | 3 | 807 | 태그 | jungle_camps[2] = JungleType::Bee — ★`JungleType` 의 **판별자**다(임계가 아니다). 오라클 D7_o1 실측 태그 = Rhino 0 · Mushroom 1 · Bee 3 · Stump 2 ★소스 배열 순서가 Rhino,Mushroom,Bee,Stump 임에 주의(2보다 3이 먼저 저장됨) · 오라클 실행 확증( 오라클 game==mine 236/236) · 오라클 실행 확증(7차 배치D: 오라클 D7_o1 직독 — `Option<JungleType>` Some(Bee) bytes=[3] (Rhino 0 · Mushroom 1 · Stump 2). 기존 근거였던 '236/236' 은 `best_jungle_goal` end-to-end 일치라 태그값 자체의 증거가 아니었다) | 2 |
| `19` | best_jungle_goal | 3 | 2 | 807 | 태그 | jungle_camps[3] = JungleType::Stump — ★`JungleType` 의 **판별자**다(임계가 아니다). 오라클 D7_o1 실측 태그 = Rhino 0 · Mushroom 1 · Bee 3 · Stump 2 · 오라클 실행 확증( 오라클 game==mine 236/236) · 오라클 실행 확증(7차 배치D: 오라클 D7_o1 직독 — Some(Stump) bytes=[2]. 배열 순서 Rhino,Mushroom,Bee,Stump 가 저장 태그 0,1,3,2 와 일치함을 실행으로 확인) | 2 |
| `19` | best_jungle_goal | 4 | -1 | 825 | 센티널 | Option<JungleType>::None 의 니치 센티널(=255). ① now_camp==None 판정(825줄) ② min_by_key 결과 unwrap 잔여검사 2곳(819/833줄, 시드가 있어 실제로는 발생 불가) · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
