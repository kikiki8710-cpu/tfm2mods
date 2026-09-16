# 0.6.0 명세 패치 대장(spec_patch_060) — 반영은 나중에 한 번에. 함수별 패치 요지 + RE 원문 포인터

> 원칙: 여기는 색인·요지. **패치 본문(등가 Rust·디컴 인용)은 RE 파일의 해당 절이 정본.** 반영 시 specs20_v3 의 `i` 로 찾는다. 판정 근거 = r19_result.md · r20_result.md.

## A. 횡단 갱신 재료(전 명세 공통)

| 재료 | 값 | 정본 |
|---|---|---|
| SmallActionPlay 태그(+0xb1) | AroundHide 제거 · 3+idx · AroundPosition 0/1/2 · 구멍 10→9 · 표 = RE enum정체 | `RE/2026-09-16_0.6.0_enum정체_SmallActionPlay_AroundHide제거_enumdiff_원문.md` |
| BattleSubPlanGoal | 8→5: Trace 0 · Kiting 1 · KitingBack 2 · RunAway 3 · End 4(구 Protect1/Assassin5/AssassinReady6 제거) | `RE/2026-09-16_r19_실변경17_배치A_score계열6_디컴대조_원문.md` §2 |
| SubPlan | 18: 14 AttackNexus 유지 · 17 ObjContest{slot@+8,poke@+0x10} 추가 · 태그 u8 idx+2 → u64 0x8000…+idx · Jungle(4) 니치 | `RE/2026-09-16_0.6.0_신규플랜RE_ObjContest_Dive_JoinTrait_특성_원문.md` · `RE/2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md` §15 |
| BigPlan | 16 불변 · 0.6.0 Battle(idx 7) 니치 dataful(태그 0/1) · 나머지 idx+2 · LPH 니치 기본 idx 4→7 | `RE/2026-09-17_r20_변경99_배치A_14_디컴대조_원문.md` §새 사실 · `RE/2026-09-16_r19_실변경17_배치C_판단함수5_디컴대조_원문.md` §5 |
| Effect vtable | 신규 4(0x30 damage_vs_target·0x40·0x48·0x70) · 구 0x38~0x50 +0x18 · 구 ≥0x58 +0x20 | `RE/2026-09-16_0.6.0_estimate_damage_to_변경_디컴대조_원문.md`(정정 부기) · `RE/2026-09-16_r19_실변경17_배치A_score계열6_디컴대조_원문.md` §부수 |
| PlayerState | +0xd0(0x928/0x930/0x9c0→0x9f8/0xa00/0xa90) · items +0x4a0→+0x510 · item_builds +0x4e8→+0x558 · gold +0x998→+0xa68 · +0x448→+0x478 · +0x450→+0x480 · +0x464→+0x49c · Box<dyn> +0x510→+0x580 · Strategy +0x4f8→+0x568 · 특성 +0x49d/+0x49e/+0x49f/+0x4a0 · +0x490 신규 | `RE/2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md` §15 · `RE/2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` §새 사실 |
| Blackboard(stride 0x2e8→0x5c8) | 소액션 레코드 +0x78→+0x1d0 · big_goal/in_battle +0xf8→+0x300 · last_seen +0x1e0→+0x3e8 · 신규 Vec +0x250/+0x258 · +0x4d8+i*8 tick · +0x260/+0x278 | `RE/2026-09-17_r20_변경99_배치A_14_디컴대조_원문.md` `RE/2026-09-17_r20_변경99_배치C_14_디컴대조_원문.md` `RE/2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` |
| TeamPlan | objective +0x41f/0x420/0x421→+0xcd5/0xcd6/0xcd7 · serpen {phase +0x3e0, mode +0x3e4} · epic {+0x400, +0x404}(mode 2 = 레거시) · 게이트 +0xcc0/cc1/cc2/cc5/cc7 · 마스크 +0xca3/ca4/ca5 · camp_last_visible +0x80/88→+0xa0/a8 · chats +0xc0→+0x2e8 · ally_battle_stop +0x0→+0x20+i*0x10 · steal 블록 +0x2b8 · vision 0x230→0x520 | `RE/2026-09-17_r20_변경99_배치C_14_디컴대조_원문.md` `RE/2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` `RE/2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` |
| LegacyPlanHandler | v3_armed 0x24b5 · plan 0xf58 · 필드표(PassiveLine.line 0x107b …) · dive_ep_live +0xe30 · last_dive_abandon +0x1f50 · team_plan +0x1408 · episodes +0x1288 · abort_src +0x24ec · last_lost_fight +0x2050/58 · 신규 +0x2060/68/70 · +0x24d5 | `RE/2026-09-16_r19_실변경17_배치C_판단함수5_디컴대조_원문.md` §5 · `RE/2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` `RE/2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` |
| BattlePlan(0x228) | +0x78 sub_goal · +0x98 · +0x1f8 with_dive · +0x204 screening · +0x205 gank_line · +0x207 dive_tower · +0x213 entry_src · +0x215 exit_src | `RE/2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` §새 구조체 |
| AgentVerHamster | version +0x3608(구 +0x2910) · small_action 0x2a28 · +0x3601 태그 · +0x3638 · +0x36c2/3 · +0x29dd | `RE/2026-09-17_r20_변경99_배치C_14_디컴대조_원문.md` `RE/2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` |
| 기타 | PlayerChampionCache 팀 530·포지션 106 qword · per-champ cache 0x320→0x350 · CombatParameter 신규 +0x14d8/+0x14f0 Vec<EnemyParam>(0xd8·+0x58 id) · ScoreParameter +0x1501 · map +0x38b8 region 격자 · Entity undying +0x488 · FightSituation 96B · estimate_damage_to 산식 동일+override 3 | `RE/2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` `RE/2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` `RE/2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` `RE/2026-09-16_0.6.0_estimate_damage_to_변경_디컴대조_원문.md` |
| ABI | check_kill_die_tick eb82d0→eda920 (version,data,judger,focus,&enemy,&ally,bool×3,&Option) · defensive_crisis(+extra_tick) · resolve_join_stake(+committed,+horizon_sec) · resolve_fight_uncached/full(+bias) · wave_priority_clearer_position(+exclude_jungler) · buff_value_v54(+version,+champ_incoming) | `RE/2026-09-17_r20_변경99_배치A_14_디컴대조_원문.md` §ABI · `RE/2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` · `RE/2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` |
| ★version | `version>=3` 분기 전반 신설 · 값 = AgentVerHamster+0x3608 · **런타임 확인 전까지 v2/v3 경로 둘 다 명세에 적는다** | `RE/2026-09-17_r20_변경99_배치C_14_디컴대조_원문.md` 공통 발견 |

## B. 함수별 패치(변경 판정 42 · 한 줄 21 · 다건 16 · 전담 4 · 콜리 1)

| 구 | 신 | i | 함수 | 종류 | 패치 요지 | 정본(RE 파일 · 절) |
|---|---|---|---|---|---|---|
| `dc2960` | `ed0400` | 168 | SmallActionAroundBush::get_input | 한 줄 | `if self.path_finder.as_ref().is_some_and(|pf| pf.name != "around_bush") { self.path_finder = None; }` 를 `if is_none { new_target }` 앞에 삽입 | `2026-09-16_r19_실변경17_배치B_SmallAction_get_input계열6_디컴대조_원문.md` §AroundBush::get_input 변경 구간 |
| `d40f10` | `fb1510` | 66 | evaluate_gank_opportunity_with_score | 한 줄 | 노이즈 폭 `k=(1000-judge)/10` → `/20` (헬퍼 de2fa0 · RNG 마스크 달라짐) | `2026-09-17_r20_변경99_배치A_14_디컴대조_원문.md` §7 |
| `e7b640` | `f27ea0` | 23 | buy_item | 한 줄 | `items.len()>2 → continue` 삭제 → 선검사 `active_owned = items.filter(|it| list.any(is_active && name==)).count(); if >3 → None` | `2026-09-17_r20_변경99_배치A_14_디컴대조_원문.md` §8 |
| `d26900` | `ec4c20` | 213 | PassiveLinePlan::v46_stage1 | 한 줄 | (c,b,a′) = aggr(+0x49d)?(0,67,32) : def(+0x49e)?(240,67,32) : 구 공식 · 시그니처 (team,my_pos)→player | `2026-09-17_r20_변경99_배치A_14_디컴대조_원문.md` §12 |
| `df0a90` | `dcb890` | 72 | SerpenHuntAndPokePlan::is_end | 다건 | 진입: `tp.3e4==2 → (tp.404!=2 || tp.cd5!=1) → true` · setup_like = (3e4==2)?(404==2&&cd5==1&&cd6==1):(3e0==1) | `2026-09-17_r20_변경99_배치A_14_디컴대조_원문.md` §10 |
| `e01c40` | `e82c00` | 3 | defensive_crisis | 한 줄 | 6번째 인자 extra_tick · `die_imminent = death < (v>=3 ? extra_tick : 0) + tps*2` · 짝 e82c00 | `2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md` §2 |
| `e7acd0` | `f27590` | 71 | should_recall_to_shop | 한 줄 | `if active_cnt > 3 && item_list[next].tier()==0 { return false }`(구 >2) · 카운트 = 16f2f40 | `2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md` §5 |
| `defa20` | `fd7b40` | 8 | is_end | 한 줄 | L164 `tp.3e4!=2 → true` + `tp.404==2 && tp.cd5!=0 → true` · L172 setup_like = (404==2 ? cd6 : 400)==1 | `2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md` §10 |
| `e04f50` | `ee3e40` | 41 | fight_participants | 다건 | 근접 아군 push `(a, poke_vulnerable? tps*6+1 : 0, false)` · +0xcc1 이면 bb[+0x4d8+i*8] 6초 게이트 · 원거리 `walk<=tps*6 && !poke_vulnerable` · 술어 ee3c80 | `2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md` §8 |
| `dccc60` | `ff11f0` | 74 | GoalData::update | 한 줄 | goal_data.rs:27 앞 `if v>=3 && champ.undying { remain=max(undying buffs duration); if remain>tps { heal_commit=false } }` | `2026-09-17_r20_변경99_배치C_14_디컴대조_원문.md` §1 |
| `e8b5e0` | `e9de70` | 192 | SerpenCheckSubPlan::action_candidates | 다건 | L25 안 `if tp.cc0==0 {move_check=true} else { near>=REQ[tutorial] || anchor.pos==my_pos }` · 표 [3,2,1,2,1,3,1,3,3] · 바이어스 [60000,0,40000,60000,20000] | `2026-09-17_r20_변경99_배치C_14_디컴대조_원문.md` §3 |
| `d28800` | `ec7180` | 206 | PassiveLinePlan::update | 다건 | ①check_recall v3 undying 게이트 ②v46 bound = 1416fbbc0(특성) ③L232 Gank 게이트 v≥2 bb Vec(+0x250) 조회 → 없으면 cd5==8 && cd6==line | `2026-09-17_r20_변경99_배치C_14_디컴대조_원문.md` §4 |
| `d851d0` | `ff62e0` | 180 | position_eval_at_uncached | 전담 | 종반 신규 항: 고체력 적 중심점 이격 보너스(cap 25 · purpose 4 ×2 · cap 60) → score+0x6f0 · 나머지 1항 부호 재작성 미해석 | `2026-09-17_r20_변경99_배치C_14_디컴대조_원문.md` §5 |
| `e5d300` | `d5ff20` | 69 | try_engage_dive | 한 줄 | `plan.screening=true; update; screening=false; if sub_goal>=2 {None}` | `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` §1 |
| `e8fd70` | `f3d660` | 225 | AgentVerHamster::item_v26 | 한 줄 | 활성템 `count>=4`(구 len>=3) && !tier → None | `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` §4 |
| `d639f0` | `1010060` | 93 | check_serpen_giveup | 한 줄 | take_hunt_commit = (3e4==2) ? (404==2&&cd5==1&&cd6==3) : (3e0==3) | `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` §2 |
| `dce220` | `ef4690` | 18 | v3_epicops_buff_window | 다건 | [2] 세르펜 징벌: cc5==0 이면 구 · else 계약 레코드(+0xc0/d0/d8/e0 · f1e3c0) 검사 후 리셋(+0x3c8..+0x3e4) | `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` §3 |
| `e8d6f0` | `f3adb0` | 218 | AgentVerHamster::update_state | 다건 | awareness_lapse 8번째 인자 m=1000+min(+0x490,100)²/20 · plan update 뒤 d52c00(TRAIT_AUD) · agent+0x29dd 플래그 | `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` §5 |
| `dffa10` | `e80960` | 133 | buff_value_v54 | 다건 | 시그니처 +version,+champ_incoming · v3 crisis: undying 이면 !(crisis&&champ_incoming>0&&crisis.0) → epic_incoming<1 → 0 · dur*epic_incoming<hp → 0 | `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` §8 |
| `e07430` | `ee6d40` | 70 | tower_dive_is_viable | 다건 | v3 near_enemies 클로저 e0fc40(대상 기준 150000·최근가시) · pen=(900-9J)*(min(my_die,9999)+min(t_die,9999))/2000 · viable = t_die+escape+pen+tps/2 < my_die | `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` §10 |
| `eaeda0` | `f6bda0` | 198 | EpicHuntSubPlan::action_candidates | 한 줄 | get_move 전 `if tp.cc7 { s=efb5b0(tp,1,team0); if dist²(epic,me) > dist²(me,s) { push AroundPosition::new(s,5); skip } }` | `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` §13 |
| `e8aeb0` | `f34d10` | 240 | LineDefenseSubPlan::calculate_score_para | 한 줄 | my = aggr?value : def?90 : value · param.+0x1501=true · ev = aggr?150 : def?35 : line_style?50:100 | `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` §3 |
| `e388c0` | `ed3dd0` | 246 | SerpenCheckSubPlan::score | 다건 | 디스패처 후처리: `if param.1501 && def && !aggr { if let Some((a,b))=f71fc0 { if a>0 && score>=-9998 && b<2a { score=-9999 } } }` · idx 17 ObjContest arm = f72620 | `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` §4 |
| `e05e70` | `ee4e70` | 49 | resolve_join_stake | 다건 | 인자 (committed:i8, horizon_sec) · horizon=tps*horizon_sec · cc1 게이트 bb[+0x4d8] 6초 · resolve_fight_full 에 committed 전달·bias 0 | `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` §7 |
| `e7b9f0` | `f28320` | 255 | end_check | 한 줄 | 프렐류드 `if v>=3 { if let Some(r)=finish_race { margin=Aggr?tps:Flex?2tps:3tps; if r.total+margin < r.min_defender_arrival {return true} } }` | `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` §8 |
| `dd5db0` | `f0cfe0` | 89 | TeamPlan::v24_objective_setup_should_che | 한 줄 | L91 게이트: 3e4!=2 → camp 5 && 3e0==1 → [2,1] · 404!=2 → camp 4 && 400==1 → [0,1] · 둘 다 2 → cd5/cd6 | `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` §9 |
| `d2c5d0` | `ecb2b0` | 56 | PassiveLinePlan::sub_plan | 다건 | 6건: +0x117→LineWait · 갱크 게이트 bb Vec · +0x119 action_type 0 · 라인 매칭 이중모드 · Recall 앞 f1f8d0/ec6dc0 · style = !aggr && (pos==1 || def) | `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` §10 |
| `dfe2a0` | `ed5720` | 86 | FightSituation::build | 다건 | FightSituation 96B: endangered_carry·my_skills_ready·my_ult_ready·has_frontline_ally·net_dps 삭제 | `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` §11 |
| `e083c0` | `ee8450` | 207 | resolve_fight_uncached | 한 줄 | bias:i8 인자(judge_acc 뒤) · committed==-1: thr=unit*(1-bias) · 그 외 thr=unit*(1-min(bias,0)) · bias 0 이면 동치 | `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` §12 |
| `ec9de0` | `f88880` | 73 | wave_priority_clearer_position | 한 줄 | 4번째 인자 exclude_jungler: `pick(excl).or_else(|| pick(false))` · 콜러 passive_plan 4곳(0/LPH+0x24d5/*rsi/1) | `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §1 |
| `dc2070` | `ecfa40` | 157 | SmallActionAroundPositionBush::get_input | 다건 | PathVerdict: `ok && ((region==2&&flag2)||(region==7&&flag7)) && cheb(next,target)>=2 → 2` · flag=ec17f0(region 내 적) | `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §2 |
| `e5ca10` | `d5f3c0` | 80 | try_engage | 다건 | v3 게이트 A(LPH+0x2060/68/70 최근 실패 교전 reason∈{7,13,15,16}·3tps·ee8090) · B(bb+0x278/+0x260 타워 250000) · v3 update 2회 · +0x98 min · End→None | `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §3 |
| `dff080` | `ddd7e0` | 68 | base_sub_goal | 한 줄 | `Some(_) => if v>=3 && Response && ally_within_120000 { Kiting } else { KitingBack }` | `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §4 |
| `e29b40` | `100c2f0` | 165 | action_eval::evaluate_action | 다건 | score += aggressive_gain(aggr: 최근접 적 피해×hp_value/hp · reach+ms*90 감쇠) − defensive_penalty(def: 대칭) | `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §5 |
| `d3b2a0` | `eed7f0` | 92 | v46_flee_gate_check | 다건 | A = aggr?0 : def?240 : 구 · B = 67(특성) : 구 · v3 `*60→*tps` | `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §7 |
| `dbd260` | `fdb210` | 184 | SmallActionRecall::get_input | 한 줄 | v3: 최근가시 적 250000 내 `r1=ckdt(true,false,false)==0 || r2=ckdt(true,true,false)<tps → goal=healp, committed` | `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §9 |
| `e900b0` | `f3da50` | 208 | get_input | 한 줄 | v3 폴백: input None && 도주계(RunAway/Recall/AroundRunAway) && !우물 && 적 200000 내 && !캐스팅 → Move(healp) | `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §12 |
| `cb1ce0` | `e986c0` | 197 | SerpenHuntSubPlan::action_candidates | 한 줄 | get_move: `if tp.cc7 { s=efb5b0(tp,0,team0); if dist²(me,serpen) > dist²(me,s) { push AroundPosition::new(s,5) } }` | `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §13 |
| `d5bbf0` | `f77820` | 181 | calculate_interaction_action_score | 전담 | v3 분기 5지점(도달가능 산식 · bb 슬롯 스캔 · +0x49c 게이트 8번째 인자 · 꼬리 집계) — 20KB | `2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md` §11 |
| `cce210` | `fd98c0` | 176 | SmallActionTrace::get_input | 전담 | v2 우물 회피/lethal PathFinder 정책(trace_avoid_well·_lethal) · self 레이아웃 변경 | `2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md` §13 |
| `defcd0` | `fd7f70` | 83 | EpicHuntAndPokePlan::sub_plan | 전담 | ObjContest 반환 · hp 51→50(v3) · tp 게이트 다수 | `2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md` §14 |
| `cbbdb0` | `e9fe00` | 188 | BattleSubPlan::action_candidates | 콜리 | 본체 동치 · 콜리 resolve_fight_stake ee5e80(ee3c80 호출)·fight_participants·stake_roster 변경 | `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §14 |

## C. 동치 판정(오프셋·태그·슬롯·RVA 갱신만) = r19 16 + r20 38 → 목록 `r19_result.md` · `r20_result.md`(E/E+ 행) · 갱신 값은 A 절 표로 일괄.

## D. 심층(티어1 15 + 이관 6 = 21) — 별도 라운드(r21~) 결과가 오면 여기 B 표에 행을 추가한다. 목록 = `r20_result.md` 말미.