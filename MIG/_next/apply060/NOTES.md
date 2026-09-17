# apply060 — 배치 반환에서 나온 정정·특이사항(대장 §A/§D 에 마지막에 일괄 반영)

## batch_01(09-17 10:5x · 9 · A 7 · B 1 · A/C 1)
- best_jungle_goal: r22 d 정본으로 작성(skip_race · 좌표 팀1 방향 · 반환 (camp, owner_team)). 에픽/세르펜 리스폰 ≤30tps 임박이면 블록 B 전체 스킵(w11 「obj_soon 즉시 반환」 반대).
- ★정정 `d56570 v2_obj_restore_safe = (version, player, data)` — w11 §B9 · r22 c 의 「(version, rnd, player)」 오기(d57b00 은 rnd 를 받지 않음). 본체 = 내 챔프 None ‖ 근접 적 없음 → true · 아니면 `check_kill_die_tick(eda920) > 2·tps`.
- e13c10 가중치 표: 클램프 없음(`sub al,3; cmovb eax,6` · 16-qword 표 `[0,0,2,3,0,1,3,3,3,2,4,6,7,8,9,10]`) · f71610 veto 는 |base|<6 여부 무관 모든 후보에 호출.
- TeamPlan::update 웨이브우선 게이트 신규 분기: `vt+0xe8(game) ? player.strategy(+0x568).+0x10 : game.strategy(team)(vt+0x108).wav`.
- ee3c80(포킹취약) 계약: v≥2 · 아군 공격 reach < 최소 적 거리 · Σee0800(적 DPS)>0 · `hp·1000/Σ < max(3, atk_dur·100/max(1,+0x3fc+100))` · ee0800 세부 미독.
- update_on_dead pending tower_binary 복사 = `self+0x1ef8..+0x1f18 ← plan+0xb8..+0xd8`(0x20B · RE 「..+0x1f10」 보다 8B 넓음).
- 스팟체크 못 한 곳: faf2a0 본체 · ee2310 · 3a92f10 의미 · f71610 본체 · assign 헬퍼 5 · f1e3c0 · fe1ad0 · f18f90 감사 프롤로그 · dbd4e0(JT 파손).
- ⚠ 에이전트가 Write 도구 없이 heredoc 으로 md 를 썼다고 보고 → `chkapply060.py` 검사 통과(제어문자/탭/BOM 0).

## batch_02(09-17 11:1x · 9 · A 5 · A/B 4)
- ★정정 AgentVerHamster: `small_action = +0x3550`(태그 +0x3601 = 0x3550+0xb1) · `positioning_score = +0x2a88` — §A 「small_action 0x2a28」·RE §12 「+0x1d90→0x3550」 오기.
- f3da50 get_input 신규 prelude: `version>=3 && Moba` 면 pos_ring(LPH+0x458 = self+0x988) push_back((tick,x,y)) · `tick-front.tick > tps+1` pop_front — **r22 a pos_ring 세팅처 = 여기**.
- f3da50 신규 쓰기: DM 블록 직후 `input.tag >= 2`(Attack~Ult) 면 `LPH+0x5f0 = tick`(0.5.8 없음 · last_attack_input_tick 추정 · 독자 미상).
- v3 폴백 위치 정정: 재시도 체인(최대 4차 get_input) 끝 합류점 140f3e14c(RE §12 「1차 직후」 오기).
- 0.5.8 명세 오기: SmallActionTrace.last_escape 는 0.5.8 도 +0x95(명세 +0x8d 틀림).
- try_engage: target_id==0 이면 게이트 A 만 건너뛰고 B 는 실행.
- v46 A/B: `min(PlayerState+0x230,100)` 직접 사용(0.5.8 aggressive_ratio 0..1000 의 스케일 등가).
- 입력 지연: `gen_range((400+3*(100-min(a,100)))..=(1200-4*min(a,100)))/100` a=player+0x200 인라인(로직 변경 아님 추정).
- 미확인 오프셋(0.5.8 값 유지): SerpenHunt MobaMode live_list +0x1d0/+0x1d8 · cache.jungles +0xd0/others +0xf0 · PlayerState.parameter +0x180 · ObjectivePosture · debug.infos +0xa0 · evaluate_action lane_anchor_gain 본체.
- 도구: heredoc 8KB 초과 시 EOF 실패 → 청크 append(검사 통과).

## batch_03(09-17 11:3x · 9 · A 6 · A/B 2 · B 1)
- v3_epicops_buff_window exe ABI = 6인자(self,version,player,data,goal_data,plan) — 0.5.8 도 rnd 소거(명세 sig 7 은 IR 기준).
- f1e3c0 의미 확정: (dead_allies, camp 240000² 내 최근가시 적 수, obj hp%(없으면 100)) = v4_serpen_lock{tick,dead_allies,near_enemies,obj_hp_pct} 와 일치 · 계약 유지 판정이면 채팅/objective 변경 없이 즉시 true.
- buff_value_v54 §9: `window = min(dur_sec,6)` · `window*epic_incoming < max(recv.hp,1) → return 0`(즉시 반환 · 클램프 안 거침).
- update_state d52c00(TRAIT_AUD) 은 LPH +0xea8..+0xf42 · +0x11c0/+0x11d8 · +0x20a0..+0x20b0 · +0x23d0 · +0x24a8/9 · +0x24ea/b 를 씀 — 감사 전용인지 재소비되는지 미확정(11KB · 별도 명세 후보).
- position_score_at_position ff5ec0 = 7인자(sret,version,player,data,x,y,purpose) · positioning_score 포인터 없음(TLS 캐시) · 콜러 3곳 좌표를 `min(v/32000,29)*32000+16000` 셀 중심으로 스냅.
- efb5b0: slot1 = camp_pos(2)·camp_pos(4) 중점 · slot0 = camp_pos(0)·camp_pos(5) 중점 · side=!is_blue. camp_pos bool 은 일관되게 `team != 0`.
- check_serpen_giveup 판단페널티 인라인 `(424 − min(9J,300))/125` = 0.5.8 식 등가.
- 미확인: parameter.positioning_score 0.6.0 위치 · PlayerState+0x180 parameter · +0x490 스탯 이름 · bb+0x4d8/agent+0x3698 틱 기록처.

## batch_04(09-17 11:5x · 9 · A 7 · A/B 1 · B 1)
- ★정정 PassiveLinePlan::sub_plan 갱크 게이트(L863): `rec.map_or(false, |r| r.line==line) || (cd5==8 && cd6==line)`(OR) — 배치 E §10 의 unwrap_or_else 표기 오기(레코드 line 불일치면 레거시 검사로).
- ecb2b0 Recall 순서: `minion_diff>2 && !f1f8d0 → Recall else LineWait` · 타워 경로 `ty==2 && +0x88!=0 && !f1f8d0 && (!aggr || !ec6dc0) → Recall`.
- f0cfe0 healthy_side 술어 실체 = `hp%≥40 && (!(x>192000 && h−y<x && h−y<w−192000) || |x−(h−y)|<64000)`(양 버전 동일).
- fd7b40: 배치 B §10 「+0x3e4=Epic 모드」 추정 폐기(씬 니치).
- ed3dd0: aggr=1 이어도 f71fc0 호출(카운터) · 판정은 `!aggr && def` 한정.
- e82c00 eda920 인자 `(version,data,tp,target,clone,&opts{8,game,0,0},0,0,0,&None)` · 슬롯 명명 skill/skill2/ult(배치 B 「attack/skill/skill2」 오기).
- f28320 strategy 조회 인라인 = `is_solorank(vt 0xe8) ? player.+0x568 : game.strategy(team)(vt 0x108)` · game_finish +0x57d.
- 미확인: f10c60·19903f0·ee0800·fe1ad0·f27910·f0e390·ef36b0·f892d0·f88690 내부 · ee8450 틱 루프 명령 대조 · bias 실호출값.

## batch_05(09-17 11:0x · 9 · A 7 · B+ 1 · B 1)
- ★정정 evaluate_gank_opportunity_with_score fb1510: 「노이즈 폭 /10→/20」 은 오판(0.5.8 도 /20) — 변경은 노이즈 헬퍼 de2fa0 아웃라인뿐 · RNG 소비 동일 ⟹ **판정 「동치(오프셋·헬퍼만)」 로 정정**(r20 A · §B 행).
- ★정정 position_eval_at_uncached 신규 항(ffd219~ffd46d): 중심점 나누는 수 = n(n+1 아님) · 순회 = near_allies · 비교 `ally.stat_cached.hp > my.stat_cached.hp` · ×2 조건 = `my_role(+0x20)==4` · 가산 대상 = score.risk(+0x6f0) · 위치 1127~1134 사이 · 「부호 재작성 1항」 미해석.
- v46_stage1: my_tower 는 양버전 (x,y) 좌표 · 0.6.0 `env::var("V46_TRACE")` 트레이스(판정 무관).
- SerpenCheck: L48 좌표 양버전 셀 중심 · L64~65 posture 디버그 로그 삭제 · Trace 태그 0xe→0xd.
- GoalData::update: nexus_final_stand/base_defense_focus 는 양버전 캐시 헬퍼 db0160 비트(0x100 / 0x1|0x10000) — 재현 시 반환 레이아웃 확정 필요.
- fight_participants: `+0xcc1==1`(상시) 분기에서 구 ally_battle_stop_tick 검사 생략(사장) · ally_is_bound ee3000 debug 인자 제거.
- buy_item: `len>2 continue` 삭제 → 활성보유 `>3 → None` 선검사 · should_recall_to_shop `>2→>3` · 카운트 헬퍼 16f2f40.

## batch_06(09-17 11:2x · 9 · A 5 · A- 3 · B+ 1)
- PassiveLine::update 신규 래치 3(배치 C 누락): `+0x116 gank_lowhp_said` · `+0x11a cancel_low_hp_sent`(v3) · `+0x11c hide_line_too_sent`(v3 · 0xff None) — L269 Cancel 채팅 skip/L299 HideLineToo skip/갱크 없음 exit 리셋.
- ★정정 bb 팀 콜 Vec 항목 조건: `kind(+0x46)==1 && members(+0x40)[my_pos]!=0`(RE 「+0x40!=0」 오기 · L232·L254).
- ★정정 1416fbbc0 호출부 비교: `heal_die_gain(:624) < out[0](best_diff_bound)`(RE 「elapsed*60/tps」 라벨 오기) · +0x31d=Aggr · +0x31e=Def · +0xb0=aggr(+0x230).
- f904b0 정체 확정: 적 챔프 중 (visible ‖ last_seen+120≥tick) && dist²(e,camp)<150000²+1.
- passive_plan Setup travel = /max(ms,1) · Split 사장 경로 PassiveLine +0x120=1 = v4_split.
- hunt_and_poke: v3 는 hp_ratio<51 안 읽음 · strategy 인라인 = `is_solorank ? player+0x568 : game.strategy(team)`.
- PassiveJungle Hide 페이로드: +8 ambush_cell None · +0x20 self+0x48 bush region · +0x28=0x01000000(stealth 1) · +0x2c=1 out_line.

## batch_09(09-17 11:3x · 1 · A)
- AroundBush::get_input: 로직 변경 1건 그대로(name≠"around_bush"(len 11) 면 HeapFree +0x50/+0x58 · 태그 +0x6d=2 → new_target · rnd 소비 달라짐) · **self 레이아웃 불변**(dispcheck 의 0x18→0x50/0x20→0x18/0x50→0x58 은 삽입 블록에 의한 오배열 → dispcheck 규칙 라벨 정정) · 오프셋 변경은 PlayerState 만 · version 분기 없음 · 콜리 e609e0/e1f610/e2d650(모노모프)/e5bb90.

## batch_07(09-17 11:1x · 9 · A 3 · A/B 2 · B 4)
- 0.5.8 명세 정정: check_kill L2462~2463 「타워 attack_cooltime 0 → div0 패닉」 오기(양버전 `max(cool*100/max(as+100,1),3)`) · L2509 lapse 게이트 `+0x464!=0` 는 0.5.8 에도 있었음(명세 누락) → 0.6.0 `+0x49c==1`.
- handle_chat_inner HideLine v3 추가 단계: d35630→d37d10 사전검증 뒤 발신자 Bottom/Support(from>2) 면 파트너 슬롯(4/3) 존재 시 d37c50 재검증 + LineGanker line 을 d37c50 출력(+0x6df)으로 재지정 · 파트너 None 이면 무시 · 콜리 4종 정체 미확정.
- ComebackPick Jungle 가지 `f1dd50(3, player, data, &team_plan, line)` 게이트 신설(w7 미기재 · 첫 인자 상수 3).
- calculate_action_score v≥2 롤 = 결정적 splitmix(seed = judger.id<<8 ^ (t.id<<0x14|0x5a3e) ^ game seed ^ (tick/(6·tps))<<0x28) — 0.5.8 동일 구조.
- EpicCheck 는 version 분기 없음(+0xcc0 게이트 · 상시 1).
- 「★사장(armed 상시 1)」 표기 도입: f10f70 v27 discipline · f024d0 !armed 가지 · eff930 GIVEUP 경로.
- 미독 잔여: d5d700 · f024d0 f02ef6..f04f8f · ee0fe0/ee2310 · fa3350 · 블록 A/C′ · phase 핸들러 5 · ef7680/efa880/efd2f0 · window_intent cache 카운터(+0x478/+0x23a0/+0x23b0) · counter_jungle_route(+0x18) 생성처.
- 도구: 본문에 `ri` 등 PowerShell alias 토큰이 있으면 샌드박스 차단 → 에이전트가 `rinfo` 로 치환(logic 텍스트 검토 시 참고).

## batch_08(09-17 11:4x · 8(+1) · A 4 · B 4 · C 1)
- 신규 확정 오프셋: get_small_action `judge_noise_plan = LPH+0xe20(Some)/+0xe28(disc)` · `judge_noise_ratio = LPH+0x2428..+0x2480 [i64;11]` · `disc = plan.tag≥2 ? tag−2 : 7`(Battle 니치→7) · score 클로저 env = {&sub_plan(+0x13a0), &pos_ring(+0x458), &version, …, debug}.
- 에고웨이브 스케일: passive_plan ③ `(500−10r)(10e)/1000` vs LPH::update egowave_check `(500−r10)·e10/500`(w4 표기 · asm 재대조 안 됨) → 재현 전 d40486~ 상수 스팟체크 권장.
- LPH::update L1124 backfight 콜리 짝: w4 「e0bf70→eecea0 is_ignored」 vs w2 「eecea0 = is_unreasonable_tower_dive_enemy · eeb400 = is_ignored_battle_enemy」 → ★확인 필요.
- update_v32 ★미독 7(e11a60·decd30·dcb320·ee3770·tp 위치이력·P15/P16 결합식) · passive_plan ★미독 9(ff0c10·f1dd50·f18bf0·fb8da0·d56ac0·efc410·d754d0·fb78b0·코드30/31) · LPH::update ★미독 11(6 구역).
- 런타임 실측 반영 표기: extra=None · cc2=0 · cc7=1 · +0x24d5/+0x24d6=1 → 사장/상시.
- 계측 필드 0.6.0 오프셋 미확인(재현 무영향): v48/v3_cand_src/… · PlayerState statistics · order_ratio.
