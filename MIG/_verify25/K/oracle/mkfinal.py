# -*- coding: utf-8 -*-
"""mkfinal.py — 이전 시도의 mkpatch25K.py 를 바탕으로 25차K 최종 patch.json 을 만든다.
   (1) 원본 스크립트 문면을 고쳐(brief_error ② 정정 · mem[35]/note 조기반환 보강 · 191 tls 검증법 보강)
   (2) 추가 정정: 190 sig.tls 신설(dict) · sret variant live 표(params[0].role ×2) · ev_up guard
   결과 = C:\\tfm2mods\\MIG\\_verify25\\K\\oracle\\mkpatch25K.py(덮어씀) → 실행 → patch.json"""
import io, os, sys, re
SRC = r"C:\tfm2mods\MIG\_verify25\K\oracle\mkpatch25K.py"
s = io.open(SRC, encoding="utf-8").read()
assert "p.save()" in s

# ── (1a) brief_error ②: 「190 tls 절은 patch 로 못 넣는다」 → 넣을 수 있다(applypatch 스칼라 분기의 dict 신설) — 판정 반전이므로 오류 1건으로 센다
old_be = [l for l in s.split("\n") if l.startswith("p.brief_error(") and "TLS 절 검증" in l]
assert len(old_be) == 1, old_be
s = s.replace(old_be[0], u'p.brief_error(u"지시 ④ TLS 절 검증 — 190 명세엔 `signature.tls` 절 자체가 없다(191 만 있음 · 두 함수는 위험 판정 블록이 문자 단위 동일한 형제인데 한쪽만 절이 있다 = mkspec/tls 절 생성기의 누락). ~~이전 시도는 「dict 하위키 신설은 그 필드가 dict 일 때만 받아 190 은 patch 로 못 넣는다」고 썼는데~~ applypatch ① 스칼라 분기가 `new` 가 dict 이고 현재값이 None/{} 이면 객체를 통째로 만든다(applypatch.py:459~465) → 이번 patch 에 `/specs[190]/sig/tls` 신설로 냈다(판정 반전 1건).")')

# ── (1b) 191 mem[35]/note 의 new 에 조기반환(rs:41·rs:55) 미기록 사실을 덧붙인다
anchor = u"오라클 실행 확인: 13케이스 before/after 전부 predict 일치.\","
assert anchor in s
s = s.replace(anchor, u"오라클 실행 확인: 13케이스 before/after 전부 predict 일치. ★조기반환 경로에서는 안 쓴다 — rs:41(위험) · rs:55(WaitGroup/SoftDisengage return) 는 rs:65~75 블록에 못 미친다: 오라클 e_v20_5(WaitGroup · 적 진영 is_enemy_side=true 인데도 after=0) · e_v20_15(SoftDisengage after=0).\",")

# ── (1c) 191 tls indirect 의 new 에 2차 검증법(깊이 무제한 fixpoint + fn-ptr 상수 간선 · vtable impl 4종) 을 덧붙인다
anchor2 = u"근거 = 25차K tlsreach.py(호출 그래프 BFS · TLS 이름토큰/@anon 상수 매칭).\","
assert anchor2 in s
s = s.replace(anchor2, u"근거 = 25차K tlsreach.py(호출 그래프 BFS · TLS 이름토큰/@anon 상수 매칭) + tlsfull.py(깊이 무제한 fixpoint · TLS 는 `thread_local` 전역 직접참조 + `@anon = constant ptr @F`(LocalKey `__getit` fn-ptr) 간선으로만 판정 — 이름 휴리스틱 0) 두 방법 결과 동일(11종). vtable 3슬롯(tick +0x28 · is_visible +0xf8 · get_entity_by_id +0x1f0)의 impl 4종(Game·SingleLaneGame·DeathMatchGame·ExpectedGame) 전이 TLS 0 → 간접호출도 TLS 접점 없음.\",")

# ── (2) 추가 정정 — p.save() 앞에 삽입
OR = u"오라클 25차K(_verify25/K/oracle/o25k.rs · run25k.py · 케이스당 프로세스 1)"
add = u'''
# ───────────────── 추가(최종판) ─────────────────
# ⑨ 190 sig.tls 절 신설 — 191 과 같은 형식(direct / indirect_callees_in_order). 스칼라 분기의 dict 신설(현재값 None).
p.fix("/specs[190]/sig/tls", old=None,
      new={"direct": u"없음 — 본 범위(m14.ll 18966~20981)에 thread_local 전역 직접참조·LocalKey::with 호출 0건(tlsfull.py: direct TLS refs [] · 간접(vtable/fnptr) 호출부 0).",
           "indirect_callees_in_order": u"[TLS 접점이 있는 콜리만, 호출 순서] ①position_score_at_position(rs:138, purpose=General 2 · m14.ll:19241) → position_eval_at: POS_EVAL_CACHE 직접 · position_eval_at_uncached 경유 EPC_CACHE·PE_CAND_MASKS·PE_PLAYER_CTX·ATTACK_DMG_CACHE·TOWER_MINION_CNT_CACHE·SLOT_READY_MEMO(slot_ready_cached)·CC_TIME_MEMO(slot_cc_time_cached)·SIEGE_STANCE_CACHE(v47_siege_stance)·RESOLVE_FIGHT_CACHE(v47_siege_stance→resolve_fight→resolve_fight_full) — 10종 · 이 함수 경로의 TLS 접점은 이 1호출뿐(위험 판정 전에 1회 · 조기반환 여부와 무관하게 항상 호출). ⚠나머지 직접 콜리 30종(nontarget_windup_perceived·SmallActionRunAway::new/new_with_skill·SmallActionAround::new·battle_action·iter_minions·can_attack/can_skill/can_skill2·Effect::range_adjust·CastingTarget::check·SmallActionAttack/Skill/Skill2::new·attack_summon_action·iter_towers·can_tower_focused_when_attack·attack_structure_skill_action 등)은 전이 TLS 0(깊이 무제한). CAMP_POS_MEMO(camp_pos)·CAST_BEAMS·HP_VALUE_MEMO·INTER_CTX·DIE_TICK_CACHE·MAX_RANGE_CACHE·LAST_STAND_MEMO 는 이 루트에서 도달 불가(정적 콜그래프 · fn-ptr 상수 간선 포함). 미도달 사각 = 콜리 내부 vtable 간접호출(battle_action 6곳 · attack_structure_skill_action 6곳 등 79 define) — AbstractGame impl 4종의 tick/is_visible/get_entity_by_id 는 TLS 0 확인. 근거 = 25차K tlsreach.py(tls190_d25.txt) + tlsfull.py(scratchpad25K · 두 방법 결과 동일). 미러 재현 시 position_score_at_position 1회의 캐시 채움만 재현하면 된다."},
      evidence=u"tlsfull.py 출력(scratchpad25K/tlsfull_out.txt): ROOT …AttackNexusSubPlan17action_candidates m14.ll 18966 reachable defines 348 · direct TLS refs [] · indirect call sites 0 · transitive TLS = ATTACK_DMG_CACHE,CC_TIME_MEMO,EPC_CACHE,PE_CAND_MASKS,PE_PLAYER_CTX,POS_EVAL_CACHE,RESOLVE_FIGHT_CACHE,SIEGE_STANCE_CACHE,SLOT_READY_MEMO,TOWER_MINION_CNT_CACHE(전부 6. L19241 position_score_at_position 경유) · 나머지 30 콜리 TLS 없음 · m07.ll:24637 `call @…LocalKey…PosEvalCache…with…position_eval_at0…(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.97…)` → m07.ll:118 `@anon…97 = constant ptr @…POS_EVAL_CACHE00…call_once` → m07.ll:63551 `@llvm.threadlocal.address.p0(ptr @…POS_EVAL_CACHE0023___RUST_STD_INTERNAL_VAL)`",
      behavior_change=False, found_by="new", kind=u"보강", force=True)   # 필드가 없어 locate=None → force

# ⑩ sret variant 별 live 바이트 표(런타임 _SAP_EL 재료) — 190
p.fix("/specs[190]/sig/params[0]/role",
      old=u"원소 184B, 태그 @+0xb1(=177).",
      new=u"원소 184B, 태그 @+0xb1(=177). ★이 함수가 sret 에 넣을 수 있는 variant = {RunAway 3, Around 5, Attack 15, Skill 16, Skill2 17} 뿐(자체 push = 3·5·15·16·17 · extend 콜리: battle_action → 15/16/17(m15.ll:24160/24476/24709/24788/25119/25352/25490) · attack_summon_action → 15/16/17(m15.ll:28529/28680/28826) · attack_structure_skill_action → 16/17(m15.ll:34407/34571)). push 규약 = 생성자 sret(136B 또는 24B) alloca → 184B alloca 로 memcpy(생성자 크기만큼) → +177 태그 store → Vec 원소로 memcpy 184B(예: m14.ll:19311~19313 Around · 19370~19372 RunAway) ⟹ 원소 live 바이트 = 생성자 `initializes` ∪ {177}, 나머지는 두 alloca 의 스택 잔재. variant 별: RunAway(3) = [0,56)+[125,126)+[128,132)+{177} (new_with_skill m08.ll:92086 · new 92133 initializes((0,56),(125,126),(128,132)) · +128 with_skill: new=1 · new_with_skill=인자) · Around(5) = [0,56)+[125,126)+[128,130)+{177} (m08.ll:92385 initializes((0,56),(125,126),(128,130)) · +40 range=80000 · +128 end_delay) · Attack(15)/Skill(16)/Skill2(17) = [0,17)+{177} (m07.ll:7129/7645/12020 initializes((0,17)) · +0 tick · +8 target id · +16 i8 0). 오라클 n_* 덤프(elem[i] tag/[0,56)/+125/[128,132))와 일치.",
      evidence=u"m14.ll:19311~19313 `memcpy(%26,%25,136) / gep %26,177 / store i8 5` · 19370~19372 `memcpy(%33,%32,136) / store i8 3` · 생성자 define 줄 initializes 속성(m08.ll:92086/92133/92385 · m07.ll:7129/7645/12020) · 콜리 본문 +177 store 전수(elemstores.py) · " + OR + u" n_base elem[0] tag=5 [0,56)=… +125=02 [128,130)=0500 · elem[1] tag=3 +125=02 [128,132)=01000000",
      behavior_change=False, found_by="new", kind=u"보강")
# ⑪ 191
p.fix("/specs[191]/sig/params[0]/role",
      old=u"원소 184B, 태그 @+0xb1(177).",
      new=u"원소 184B, 태그 @+0xb1(177). ★이 함수가 sret 에 넣을 수 있는 variant = {RunAway 3, AroundPosition(untagged · +177=0), Trace 14, Attack 15, Skill 16, Skill2 17}(자체 push = 3·AroundPosition·14 · extend 콜리: battle_action → 15/16/17 · attack_summon_action → 15/16/17). variant 별 live 바이트(그 외 = alloca 스택 잔재): RunAway(3) = [0,56)+[125,126)+[128,132)+{177} (m02.ll:10100~10102 memcpy 136 + store i8 3 · 생성자 initializes((0,56),(125,126),(128,132)) · +128 with_skill: rs:40 =1 · rs:49 =0 · rs:95 new=1) · AroundPosition(untagged) = [0,48)+[48,88)+[88,104)+{173,176,177} (생성자 m08.ll:103238 sret 184B 직접: +0 tick(vtable+0x28) · +8/+16 target x,y · +24 0 · +32/+40 goal x,y · +48..+88 wait_around 40B 전부 live(m04.ll:33088~33096: target_x,target_y,d=60000,now_goal x,y) · +88 80000 · +96 end_delay · +104..+176 path_finder 72B 중 +173(=%8+69) i8 2 만 · +176 i8 6 · +177 i8 0(outline_type — SmallActionPlay 니치 판별자 자리)) · Trace(14) = [0,8)+{85}+[88,150)+{177} (본문 인라인 m02.ll:10012~10042: +0 i64 0 · +85 i8 2 · +88 tick · +96 target id · +104/+112 goal xy(get_entity_by_id 있으면 +0x660/+0x668, 없으면 0/0) · +120 15000 · +128 5 · +136 0 · +144 i8 0 · +145 i8 1(attack_range_only) · +146~148 0 · +149 i8 2 · +177 14) · Attack/Skill/Skill2 = [0,17)+{177}. 오라클 e_* 덤프(RunAway [0,56)+125+[128,132) · AroundPosition [0,104)+173+176 · Trace +0/+85/[88,150))와 일치.",
      evidence=u"elemstores.py(scratchpad25K) m02.ll 9581~10950 alloca 기준 store 전수: %22(Trace) +0/+85/+88/+96/+104/+112/+120/+128/+136/+144~149/+177 · %29/%15/%33 memcpy 136 + +177 i8 3 · %17/%19/%21/%27 memcpy 184(AroundPosition 생성자 sret) · m08.ll:103252~103306 AroundPosition::new store/memcpy 전수 · m04.ll:33088~33096 wait_around sret 5 store · " + OR + u" e_v20_31 elem[0] tag=14 +0=0 +85=02 [88,150)=… · e_base_t0 elem[0] AroundPosition [0,48)/[48,104) +173=02 +176=06",
      behavior_change=False, found_by="new", kind=u"보강")

# ⑫ ev_up guard(삽입 뒤 인덱스 밀림 방지 — mem 행은 이름으로 확인)
for u in p.ev_up:
    if u["path"] == "/specs[191]/mem[36]": u["guard"] = u"move_check"
    if u["path"] == "/specs[191]/mem[37]": u["guard"] = u"infos"
    if u["path"] == "/specs[190]/consts[18]": u["guard"] = u"56"
    if u["path"] == "/specs[190]/consts[16]": u["guard"] = u"30"
    if u["path"] == "/specs[191]/consts[12]": u["guard"] = u"4900000001"
    if u["path"] == "/specs[191]/consts[13]": u["guard"] = u"22500000000"
p.brief_error(u"지시 ②는 sret 를 「bumpalo Vec<SmallActionPlay> 32B」와 「next_plan Option<BigPlan> 384B · attack Option<Input> 32B」를 함께 열거했는데 이 배치 2함수는 action_candidates 라 sret 는 Vec 32B 하나뿐이다(next_plan/attack 은 다른 루트(next_plan/attack 계열)의 몫) — 배치별로 sret 종류를 갈라 적어야 한다. 또 variant 목록의 AroundPosition 「+0xb1 = outline_type」은 맞지만 그 값이 0 으로 고정 저장되는 것(m08.ll:103302)까지 적어야 런타임 _SAP_EL 이 「태그 0 = AroundPosition」으로 읽을 수 있다.")
p.brief_error(u"지시 ④의 TLS 작성자 목록(v48_cast_beams·champion_hp_value·interaction_score·check_kill_die_tick·max_range_cached·LAST_STAND_MEMO·CAMP_POS_MEMO)은 r16 루트 20개 전체의 합집합이라 이 배치엔 190 → 10종(position_score_at_position 1호출) · 191 → +CAMP_POS_MEMO 만 닿는다. 「이 함수가 직접·간접으로 부르는 TLS 작성자」를 배치별로 미리 좁혀 주지 않으면 배치마다 전 corpus 콜그래프를 다시 짠다(이번엔 24초짜리 캐시 도구를 만들어 해결 — tlsfull.py · callgraph.json 을 다음 배치가 재사용 가능).")
'''
s = s.replace("\np.save()", add + "\np.save()")
io.open(SRC, "w", encoding="utf-8").write(s)
print("written", SRC)
