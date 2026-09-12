# 9차 배치 B — 축 ``sig.params[].role` — **G16 승격**` · 함수 `15`~`19` (30행)

> `mkdossier.py --axis` 생성물. **칸을 하나도 줄이지 않았다** — 7차까지 지시 오류는 전부 「줄여 적은 자리」에서 났다.

| # | 함수 | idx | i | name | ty | role | ev |
|---|---|---|---|---|---|---|---|
| `15` | single_try_engage | 0 | 0 | (sret ret) |  | 반환값 out-param. tag 오프셋 0x0(support_target 의 Option 니치)에 -1 저장 = None | 4 |
| `15` | single_try_engage | 1 | 1 | self |  | team_plan(+0xf8)·positioning_score(+0x990) 만 만짐. WARN ~~team_plan 은 &mut 로 하위에 전달~~ 은 **거짓**(2026-09-11 검증배치 D): tcx 정본상 `self` 부터가 **공유 `&LegacyPlanHandler`** 라 `&mut self.team_plan` 이 성립 불가이고, `single_tower_dive_is_viable`·`SinglePlanBattle::update` 둘 다 **`&TeamPlan`(공유)** 를 받는다. 이 계열에 team_plan 부작용은 **없다**(&mut 인 것은 rnd·debug·battle 뿐) | 3 |
| `15` | single_try_engage | 2 | 2 | version |  | AI 버전. 이 함수 본문에는 version 분기가 없고 new/new_dive/update/single_tower_dive_is_viable 로 그대로 전달만 함 | 4 |
| `15` | single_try_engage | 3 | 3 | rnd |  | 본문에서 직접 안 씀. single_tower_dive_is_viable 과 update 로 전달 | 4 |
| `15` | single_try_engage | 4 | 4 | player |  | info.team(+0x930) 만 직접 읽음 | 4 |
| `15` | single_try_engage | 5 | 5 | data |  | data.cache(+0x0) = &AbstractGameWithCache(8840B). 그 +0x0/+0x8 이 &dyn AbstractGame 팻포인터 | 4 |
| `15` | single_try_engage | 6 | 6 | target_id |  | 노릴 대상 엔티티 핸들. get_entity_by_id 인자이자 TryKill 목표로 그대로 들어감 | 4 |
| `15` | single_try_engage | 7 | 7 | debug |  | 본문에서 직접 안 씀. 하위 두 함수로 전달 | 4 |
| `16` | max_range_nearly_can_use | 0 | 1 | champ |  | 시전자. ty·쿨다운·레벨·radius·stat_buff_cached 를 전부 여기서 읽는다 | 4 |
| `16` | max_range_nearly_can_use | 1 | 2 | target |  | 대상. CastingTarget::check 의 두번째 인자 + target.radius() 가 사거리에 더해진다 | 4 |
| `16` | max_range_nearly_can_use | 2 | 3 | tick |  | 쿨다운 허용 여유(틱). 남은 쿨 > tick 이면 그 이펙트는 제외. 호출부는 40/50/60 리터럴을 넘긴다(m02.ll:13159/13199/15125, m14.ll:14280/14359, m15.ll:14745/14785/16295) — '거의(nearly) 쓸 수 있다'의 '거의'가 이 값 | 4 |
| `17` | new | 0 | 0 | (sret) |  | 반환값 out-ptr. m05.ll:17695 ptr dead_on_unwind noalias noundef writable writeonly sret([384 x i8]) align 8 captures(none) dereferenceable(384) %0 | 4 |
| `17` | new | 1 | 1 | version |  | ★IR %1 (=%0 은 반환 out-ptr). AI 버전 게이트. 이 함수 본문에서는 분기 없음 — base_sub_goal 로 그대로 전달만 한다(!dbg 753). ⚠**이 params 표는 sret out-ptr 을 빠뜨렸다** — m05.ll:17695 `define void @…DeathMatchBattle3new(ptr dead_on_unwind noalias noundef writable writeonly sret([384 x i8]) align 8 captures(none) dereferenceable(384) %0, i64 noundef %1, …)` 로 IR 인자가 5개인데 표는 4행뿐이라 **표의 자리 번호가 IR 보다 한 칸 앞선다.** 8차 확정 규약은 07·15 처럼 `(sret)`(i=0) 행을 params[0] 로 싣는 것인데 `applypatch` 에 행 추가가 없어 기계로 못 넣었다(8차 배치C) | 4 |
| `17` | new | 2 | 2 | goal |  | enum2$<...BattlePlanGoal>. 태그 i64@+0x0(!range 0..4), TryKill 은 (usize @+0x8, usize @+0x10). 그대로 self.main_goal 로 이동 | 4 |
| `17` | new | 3 | 3 | data |  | cache/context/blackboard. 여기선 cache 만 씀 | 4 |
| `17` | new | 4 | 4 | player |  | 본문에서 직접 로드하는 필드 없음 — base_sub_goal 인자로만 전달 | 4 |
| `18` | v3_epicops_buff_window | 0 | 1 | self |  | objective/chats/eo_serpen_punish_issues/v3_press_chat_line 을 갱신하는 쓰기 대상 | 4 |
| `18` | v3_epicops_buff_window | 1 | 2 | version |  | AI 버전 게이트. 이 함수 본문엔 version 분기가 없고 v3_serpen_contest_clear_win 으로 그대로 전달만 한다 | 4 |
| `18` | v3_epicops_buff_window | 2 | 3 | rnd |  | PlayerState::strategy 호출에만 넘긴다(전략 샘플링용). 본문에서 직접 안 쓴다 | 4 |
| `18` | v3_epicops_buff_window | 3 | 4 | player |  | info.team(+0x930)·info.position(+0x9c0) 두 필드만 직접 읽는다 | 4 |
| `18` | v3_epicops_buff_window | 4 | 5 | data |  | cache(+0x0) 만 직접 역참조. cache.game(&dyn AbstractGame) 과 cache.player_champion(+0x1e0) 을 쓴다 | 4 |
| `18` | v3_epicops_buff_window | 5 | 6 | goal_data |  | is_object_being_taken_by_enemy 로 그대로 전달만. 이 함수는 필드를 안 읽는다 | 4 |
| `18` | v3_epicops_buff_window | 6 | 7 | plan |  | v3_epicops_repair_need 로 그대로 전달만. 이 함수는 필드를 안 읽는다 | 4 |
| `19` | best_jungle_goal | 0 | 1 | version |  | AI 버전 게이트. ★이 함수 본문에는 version 비교 분기가 하나도 없다. 주소(%13)만 잡아 filter 클로저 환경 +0x8 에 담아 넘기는데, is_cleared 호출 인자 자리에는 poison 이 들어가 실제로도 안 쓰인다 | 4 |
| `19` | best_jungle_goal | 1 | 2 | rnd |  | champ 가 None 이고 now_camp 도 None 일 때만 사용(SliceRandom::choose) | 4 |
| `19` | best_jungle_goal | 2 | 3 | player |  | info.team(0x930) · info.position(0x9c0) 만 읽는다 | 4 |
| `19` | best_jungle_goal | 3 | 4 | data |  | cache(+0x0)=&AbstractGameWithCache, context(+0x8)=&GameContext | 4 |
| `19` | best_jungle_goal | 4 | 5 | team_plan |  | 본문에서 직접 안 읽음. filter 클로저 환경 +0x20 에 담겨 is_cleared 의 arg7 로만 전달 | 4 |
| `19` | best_jungle_goal | 5 | 6 | now_camp |  | -1(=255)=None. 현재 잡고 있던 캠프. champ 가 None 인 경로에서만 쓰인다 | 4 |
| `19` | best_jungle_goal | 6 | 7 | debug |  | 본문에서 안 씀. filter 클로저 환경 +0x28 에 담기지만 is_cleared 호출 시 poison | 4 |
