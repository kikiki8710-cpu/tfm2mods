# 8차 배치 B — 축 ``consts[].kind` — 상수의 종류` · 함수 `05`~`09` (40행)

> `mkdossier.py --axis` 생성물. **칸을 하나도 줄이지 않았다** — 7차까지 지시 오류는 전부 「줄여 적은 자리」에서 났다.

| # | 함수 | idx | value | src_line | kind | meaning | ev |
|---|---|---|---|---|---|---|---|
| `05` | v50_fold_dive_episode | 0 | -1 | 131 | 센티널 | Option<V50DiveEpLive> 의 None 니치값. 진입 게이트(icmp ne -1)와 take 의 store 양쪽에 등장. DISCR_EXACT=-1 이 None, 나머지 전부 Some · 오라클 실행 확증( 오라클 실행 확증: `LegacyPlanHandler+0x570` 초기값 −1 = None, fold 후 −1 로 되돌아감(take 무조건 실행) (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 |
| `05` | v50_fold_dive_episode | 1 | 7 | 139 | 임계 | end_reason 코드 7 = 다이브 포기시각(last_dive_abandon_tick) 집계에서 제외되는 종료사유. 이 값일 땐 no_contact 여도 갱신 안 함 | 4 |
| `05` | v50_fold_dive_episode | 2 | 1 | 146 | 임계 | end_plan 코드 1 = 플랜 이름이 "PassiveLine" 으로 시작. (같은 리터럴 1 이 start_in_range 저장 시 `and i8 %103, 1` 불리언 정규화 마스크로도 쓰임) · 오라클 실행 확증( 오라클 실행 확증: end_plan=1 (plan 이름 "PassiveLine") #A·#B·#G 26케이스 (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 |
| `05` | v50_fold_dive_episode | 3 | 2 | 147 | 임계 | end_plan 코드 2 = 플랜 이름이 "PassiveJungle" 로 시작 · 오라클 실행 확증( 오라클 실행 확증: end_plan=2 ("PassiveJungle") #G 4케이스 (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 |
| `05` | v50_fold_dive_episode | 4 | 3 | 148 | 임계 | end_plan 코드 3 = 플랜 이름이 "LineGank" 로 시작 | 4 |
| `05` | v50_fold_dive_episode | 5 | 4 | 149 | 임계 | end_plan 코드 4 = 플랜 이름에 "Epic" 포함 | 4 |
| `05` | v50_fold_dive_episode | 6 | 5 | 150 | 임계 | end_plan 코드 5 = 플랜 이름에 "Serpen" 포함 | 4 |
| `05` | v50_fold_dive_episode | 7 | 6 | 151 | 임계 | end_plan 코드 6 = 플랜 이름이 "ActiveRecall" 또는 "Recall" 로 시작(두 술어가 같은 phi 값으로 합류) | 4 |
| `05` | v50_fold_dive_episode | 8 | 7 | 152 | 임계 | end_plan 코드 7 = 플랜 이름이 "Battle" 로 시작. (end_reason 의 7 과 값만 같고 의미는 무관) · 오라클 실행 확증( 오라클 실행 확증: end_plan=7 ("Battle …") #B2 tag9 (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 |
| `05` | v50_fold_dive_episode | 9 | 8 | 153 | 임계 | end_plan 코드 8 = 플랜 이름에 "Nexus" 포함 · 오라클 실행 확증( 오라클 실행 확증: end_plan=8 ("AttackNexus"/"DefenseNexus") #B2 tag16·17 (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 |
| `05` | v50_fold_dive_episode | 10 | 9 | 153 | 임계 | end_plan 코드 9 = 위 어디에도 안 걸림(그 외 전부) · 오라클 실행 확증( 오라클 실행 확증: end_plan=9 ("DeathMatchBattle …"/"SinglePlanBattle …") #B tag0·5 (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 |
| `05` | v50_fold_dive_episode | 11 | 11 | 146 | 임계 | 패턴 "PassiveLine" 의 바이트 길이(starts_with 인자) · 오라클 실행 확증( 오라클 실행 확증: "PassiveLine"(11자) 접두 일치 경로가 실제로 1 을 낸다 (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 |
| `05` | v50_fold_dive_episode | 12 | 13 | 147 | 임계 | 패턴 "PassiveJungle" 의 길이 · 오라클 실행 확증( 오라클 실행 확증: "PassiveJungle"(13자) 접두 일치 경로가 실제로 2 를 낸다 #G (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 |
| `05` | v50_fold_dive_episode | 13 | 12 | 151 | 임계 | 패턴 "ActiveRecall" 의 길이 | 4 |
| `06` | v2_response_retreat_stance | 0 | 2 | 14 | 인덱스 | AI 버전 게이트 — version<2 면 본문 전체를 건너뛰고 RunAway. (같은 리터럴 2 가 team 배열 bounds-check 길이 `icmp ult team, 2` 로도 쓰인다 — **이 행 자체는 version 비교 임계**이지 인덱스가 아니다) | 4 |
| `06` | v2_response_retreat_stance | 1 | 1 | 20 | 인덱스 | 적 팀 인덱스 = 1 - player.info.team (2팀 고정 전제) | 4 |
| `06` | v2_response_retreat_stance | 2 | 3 | 36 | 태그 | BattleSubPlanGoal::KitingBack 의 태그값(DISCR_EXACT=3, variant 인덱스와 동일) · tcx 정본 대조( tcxdict --enum BattleSubPlanGoal **tcx 정본**: KitingBack 메모리태그 3, 페이로드 focus@+0x8) | 3 |
| `06` | v2_response_retreat_stance | 3 | 4 | 38 | 태그 | BattleSubPlanGoal::RunAway 의 태그값(DISCR_EXACT=4). 모든 조기탈출 경로의 기본 반환 · tcx 정본 대조( tcxdict --enum BattleSubPlanGoal **tcx 정본**: RunAway 메모리태그 4, fieldless) | 3 |
| `07` | sub_plan | 0 | 100 | 31 | 임계 | hp_ratio = hp*100/max_hp — 백분율 변환 계수 · 오라클 실행 확증( 오라클 실행 확증: hp_ratio 정수나눗셈 509/1000 → Recall · 510/1000 → EpicHunt ) | 2 |
| `07` | sub_plan | 1 | 51 | 36 | 태그 | ★귀환 HP 임계. hp_ratio < 51 (=HP 50% 이하)이면 귀환 후보. ★**오라클 8/8 **: 경계가 **정확히 50/51** 이다( 재확인 49/50·51/52 24/24). 세 번째 OR 항이 `hp < max && in_heal_area` 인 것도 확증. SubPlan 5=Recall · 11=EpicHunt 로 귀결되는 것을 런타임 확인(**이 행 자체는 HP 임계**이지 태그가 아니다) | 2 |
| `07` | sub_plan | 2 | 5 | 36 | 태그 | SubPlan 태그 5 = Recall (dienum 확인) | 3 |
| `07` | sub_plan | 3 | 9 | 43 | 태그 | SubPlan 태그 9 = Hide (dienum 확인) | 3 |
| `07` | sub_plan | 4 | 11 | 47 | 태그 | SubPlan 태그 11 = EpicHunt (dienum 확인) | 3 |
| `07` | sub_plan | 5 | 1 | 43 | 태그 | AroundBushOutlineType::Outline 의 **태그값 1** — Hide 서브플랜의 부시 접근 방식 · 오라클 실행 확증( 오라클 실행 확증: Hide 페이로드 out_line=1(Outline) bush 0·7·26 전부 ) | 2 |
| `07` | sub_plan | 6 | 0 | 32 | 임계 | live_list.get(0) — 살아있는 에픽 목록의 첫 원소만 본다. 동시에 need_recall/check_move/enemy_spotted_me 의 false 값이기도 함 · 오라클 실행 확증( 오라클 실행 확증: check_move=0 · enemy_spotted_me=0 · need_recall=0 · live_list.get(0) ) | 2 |
| `08` | is_end | 0 | 4 | 164 | 태그 | JungleType::Morgard 의 **태그값 4** (dienum JungleType 4). ⚠`src_line=164` 는 맞지만 **IR 에 리터럴이 없다** — L164 `take_active(Morgard)` 와 L172 `take_setup_like(Morgard)` 는 `icmp eq i8 (TeamPlan+0x41f), 0` 하나로 완전히 접혔고, `i8 4` 가 살아남는 곳은 **L169 `camp_pos`(m10.ll:7691) · L174 `v24_…`(m10.ll:7709) 둘뿐**이다. take_active/take_setup_like/camp_pos/v24_..._release_to_passive 4곳 전부에 이 캠프 상수가 박혀 있다 = 이 플랜은 에픽 전용 | 3 |
| `08` | is_end | 1 | 0 | 164 | 태그 | MainObjective::Morgard 태그값. TeamPlan+0x41f 를 이 값과 비교하는 것이 take_active(Morgard) 의 접힌 실체(MainObjective->Option<JungleType> 매핑은 인라인 소멸). 태그값 0 의 출처는 **dienum MainObjective**(3차 §7 ev 상향 ⑦ — 이 행만 근거 이름이 빠져 ev4 로 앉아 있었다. 같은 사실을 18 writes 는 'dienum MainObjective 7 = Repair' 로 인용하고 있다) | 3 |
| `08` | is_end | 2 | 1 | 172 | 태그 | ObjectPhase::Setup 태그값(dienum ObjectPhase 1). TeamPlan+0x420 == 1 이 setup_like | 3 |
| `08` | is_end | 3 | 32000 | 179 | 임계 | 셀 크기 — 월드좌표를 셀좌표로 나누는 **좌표변환 계수**다(임계가 아니다 — IR 관측도 `udiv` 2회뿐). is_visible_cell 인자용 · 오라클 실행 확증( 오라클 실행 확증: 캠프 288000 → 셀 (9,9), is_visible_cell(0,9,9)=true 로 L179 진입 ) | 2 |
| `08` | is_end | 4 | 22500000000 | 184 | 임계 | 150000^2 — 캠프↔가장가까운 적챔프 거리 제곱 임계. 이보다 멀면 '적이 캠프 근처에 없다'고 보고 종료 · 오라클 실행 확증( 오라클 실행 확증: 캠프 거리 150000 → 폴스루 / 150001 → (d) 종료 ) | 2 |
| `08` | is_end | 5 | 15 | 194 | 임계 | tick_per_second 에 곱해지는 초 단위 계수 = 15초. 에픽 리스폰까지 15초 넘게 남았으면 종료. ★**오라클 13/13 **: `15 × tick_per_second` 임계가 **정확히 900** 에서 갈린다(nrt 900 → false, 901 → true). 같은 실행에서 L164 objective 게이트 · L193 에픽 생존 · phase 판정 · version 무영향(9종)까지 동시 확증. ⚠setup 경로는 `v24_…=true` 가 먼저 발화해 (c)(d) 는 오라클 미도달(범위 명시) | 2 |
| `09` | check_favorable_engage_formation | 0 | 2 | 1209 | 임계 | tick_per_second × 2 = 2초 구간. IR 에선 shl i64 %30,1 로 접혀 리터럴 2 는 team bounds-check(icmp ult %7,2) 쪽에만 남아 있다 · 오라클 실행 확증(7차 배치B: B7_o3.tsv — 미니언 18마리를 세운 뒤 유리 대형(rear 아군 1명)을 만들어 `tick_per_second` 를 20점 스윕. 09 의 판정 반전점이 **tps 40→41** 이고 `danger` 창 임계가 81 이므로 2×41=82 ⟹ 계수 **K=2 만 20/20 일치, K=1·3·4 는 기각**. `shl i64 %30,1` 로 접혀 리터럴이 없던 값을 실행으로 고정했다) | 2 |
| `09` | check_favorable_engage_formation | 1 | 100000 | 1246 | 임계 | max_engage_dist = engage_range + 100000. 아군이 '교전에 참가할 수 있다'고 볼 여유 거리 · 오라클 실행 확증( 오라클 실행 확증: d = er+100000 → 포함 / +1 → 제외, er 3종(100000·200000·400000) 전부 ) | 2 |
| `09` | check_favorable_engage_formation | 2 | 40 | 1240 | 임계 | HP 비율 임계(%). hp*100/max_hp < 40 인 아군은 대형 계산에서 제외 · 오라클 실행 확증( 오라클 실행 확증: HP% 39 → false / 40 → true ) | 2 |
| `09` | check_favorable_engage_formation | 3 | 100 | 1240 | 임계 | 백분율 스케일. HP% 계산과 cos²/sin² 임계(×100 vs ×9)에 공통으로 쓰임 · 오라클 실행 확증( 오라클 실행 확증: 백분율 스케일 hp*100/max 정수나눗셈 399/1000·400/1000 ) | 2 |
| `09` | check_favorable_engage_formation | 4 | 9 | 1283 | 임계 | 9/100 = 0.09 임계. dot_sq*100 > len_product_sq*9 → cos² > 0.09 → \|cosθ\| > 0.3 (후퇴축과 ~72.5도 이내). 같은 값이 1286행 cross_sq(=sin²) 판정에도 쓰임 · 오라클 실행 확증( 오라클 실행 확증: dy 158989 → rear / 158990 → flank (= 91dx² > 9dy²) ) | 2 |
| `09` | check_favorable_engage_formation | 5 | 4 | 1280 | 임계 | is_front 임계. dot_sq*4 > len_product_sq → cos² > 1/4 → \|cosθ\| > 0.5 (후퇴축과 60도 이내). IR 에선 shl i128 %176,2 로 접혔고, 리터럴 4 는 아군 루프 상한(icmp ult %89,4)으로만 본문에 남아 있다 · 오라클 실행 확증( 오라클 실행 확증: dy 173205 → front / 173206 → flank (= 3dx² > dy²) ) | 2 |
| `09` | check_favorable_engage_formation | 6 | 5 | 1340 | 임계 | champ_to_base_sq × 5 (최종 비율식 좌변) · 오라클 실행 확증( 오라클 실행 확증: champ_to_base 547722 → true / 547723 → false ) | 2 |
| `09` | check_favorable_engage_formation | 7 | 6 | 1340 | 임계 | enemy_to_base_sq × 6 (최종 비율식 우변). 합쳐서 champ_to_base_sq <= 1.2 × enemy_to_base_sq · 오라클 실행 확증( 오라클 실행 확증: 같은 경계(×5 ≤ ×6) ) | 2 |
| `09` | check_favorable_engage_formation | 8 | 1 | 1223 | 임계 | 제로벡터 판정 임계. retreat_len_sq < 1 (=0) 이면 즉시 true, ally_len_sq < 1 (1256행) 이면 그 아군을 front 로 계산 · 오라클 실행 확증( 오라클 실행 확증: 적이 분수 중심 위 → 무조건 true / +1 → false · 겹친 아군 = front ) | 2 |
