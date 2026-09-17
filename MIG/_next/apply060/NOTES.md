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
