# r21 공통 지시 (티어1 심층 21 · 0.5.8→0.6.0 · 「무엇이 어떻게 바뀌었나」 를 명세 패치 형식으로)

Ghidra MCP `ghidra`(8080) = 0.6.0 · `ghidra_beta`(8081) = 0.5.8 · base 0x140000000. **게임 실행 금지.** 웨이브 브리핑에 함수별 0.5.8 명세 logic **전문**이 있다 — 그것이 기준선이다.

## 산출 형식(함수마다)
1. **콜리·블록 지도**: 0.5.8 콜리 목록 ↔ 0.6.0 콜리 목록(RVA 짝 · 신규 · 소멸) · 신규 아웃라인 헬퍼는 정체 한 줄.
2. **명세 패치**: 0.5.8 logic 의 줄(`[Lnnn]` 또는 단계명) 기준으로 「그대로 / 조건 변경 / 상수 변경 / 분기 추가 / 분기 삭제 / 필드·콜리 교체」 를 표로. 새 분기는 **등가 Rust**(조건·계수·읽는 필드 오프셋·호출 헬퍼) 로. `version>=3` 분기는 「v3 경로」 로 별도 표기하고 v2 경로도 남긴다(값 미확정).
3. **새 구조체/enum/vt/ABI 사실** 별도 절(오프셋 표).
4. **확정/추정** 구분 · 디컴 인용은 변경 구간만 20~40줄 · Ghidra 실패 시 capstone(스크립트 예 `C:\tfm2mods\MIG\scr060\`).
5. 시간이 모자라면 「신규 구역 특정(주소·크기·호출 헬퍼)」 까지라도 내고 못 읽은 구역을 주소로 명시.

## 이미 확정된 0.6.0 사실(재조사 불요 · 그대로 적용) — 정본 `C:\tfm2mods\MIG\_next\spec_patch_060.md` §A(횡단 갱신 재료 표: 태그표 3종 · Effect vt 슬롯 · PlayerState/Blackboard/TeamPlan/LPH/BattlePlan/AgentVerHamster 오프셋 · ABI 7 · version≥3). **먼저 읽을 것.** 요지:
- SmallActionPlay 17→16(AroundHide 제거 · 태그 3+idx · AroundPosition 0/1/2) · BattleSubPlanGoal 8→5(Trace 0·Kiting 1·KitingBack 2·RunAway 3·End 4) · SubPlan 18(14 AttackNexus·17 ObjContest{slot@+8,poke@+0x10} · 태그 u64 0x8000…+idx) · BigPlan 16(Battle 니치 0/1 · 나머지 idx+2).
- Effect vt 신규 4(0x30·0x40·0x48·0x70 · 구 ≥0x58 +0x20) · estimate_damage_to 12857f0→1643790 산식 동일.
- PlayerState +0xd0 · items +0x510 · gold +0xa68 · +0x480(구 +0x450) · +0x49c(구 +0x464) · 특성 +0x49d Aggr/+0x49e Def/+0x49f FightJoiner/+0x4a0 LaneIntervention · +0x490 신규 스탯 · +0x9f0 챔프 id.
- Blackboard 0x5c8 · 소액션 레코드 +0x1d0 · big_goal +0x300 · last_seen +0x3e8 · 신규 Vec +0x250/+0x258(0x50B) · +0x4d8+i*8 · +0x260/+0x278.
- TeamPlan objective +0xcd5/+0xcd6/+0xcd7 · serpen {+0x3e0 phase,+0x3e4 mode} · epic {+0x400,+0x404} · mode 2 = 레거시 · 게이트 +0xcc0/cc1/cc2/cc5/cc7 · 마스크 +0xca3/ca4/ca5 · 계약 +0xc0/d0/d8/e0 · 카운터 +0xc30/c38/c70/c78 · camp_last_visible +0xa0/+0xa8 · chats +0x2e8 · ally_battle_stop +0x20+i*0x10.
- LPH v3_armed 0x24b5 · plan 0xf58 · dive_ep_live +0xe30 · last_dive_abandon +0x1f50 · team_plan +0x1408 · episodes +0x1288 · abort_src +0x24ec · last_lost_fight +0x2050/58 · 신규 +0x2060/68/70 · +0x24d5 · BattlePlan 0x228(+0x78 sub_goal · +0x98 · +0x1f8 with_dive · +0x204 screening · +0x205 gank_line · +0x207 dive_tower · +0x213 entry_src · +0x215 exit_src) · AgentVerHamster version +0x3608 · small_action 0x2a28.
- ABI: check_kill_die_tick eb82d0→eda920(version,data,judger,focus,&enemy,&ally,bool×3,&Option<(u64,u64,u64)>) · defensive_crisis e82c00(+extra_tick) · resolve_join_stake(+committed:i8,+horizon_sec) · resolve_fight_uncached/full(+bias:i8) · wave_priority_clearer_position(+exclude_jungler) · buff_value_v54(+version,+champ_incoming) · FightSituation 96B.
- 신규 헬퍼: ee3c80(포킹취약 아군) · ee0800(=구 fight_dps) · 16f2f40(활성 보유템) · fb12a0(정글 귀환 필요도) · de7750(DPS) · fafc10(free_dist 틱) · 1416fbbc0(특성 bound) · ec17f0(region 내 적) · ee8090(적 타워 사거리+8000) · efb5b0(ObjContest 진입점) · d52c00(TRAIT_AUD) · d72e70(다이브/킬 창) · fe1ad0(finish_race) · fb8da0(ambient) · ObjContest 헬퍼 ef5f20/ef57f0/ef6da0/ef7590/efb690/e89b70/10167f0/ef7680/e64cc0/ef4fc0/ef7350/ef7520(정체 일부 추정).
