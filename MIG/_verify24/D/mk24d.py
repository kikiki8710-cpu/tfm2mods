# -*- coding: utf-8 -*-
"""24차 배치D patch.json 생성 — 183 interaction_score (mkpatch 참조구현 사용)."""
import sys, io, json
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch

p = mkpatch.Patch(round=24, batch="D")
S = "/specs[183]"
ORA = u"오라클 o24d.exe(_verify24/D/oracle · 케이스당 프로세스 1개 · run24d.py 41케이스 · 예측가능 40/40 MATCH)"

# ── G12 5건: get_action 인라인 상수의 루트 줄은 618 (사슬 L309<618) ──
for i, ln in ((1, 34943), (2, 34945), (3, 34947)):
    p.fix(S + "/consts[%d]/src_line" % i, old=309, new=618,
          evidence=u"_next/reach/d57540.ll:%d 사슬 `;L309<618` — 309 는 small_action.rs get_action 의 인라인 줄, 루트(action_score.rs)는 618. G12 루트 줄 규약" % ln,
          behavior_change=False, found_by="reused")
# consts[20]: IR 리터럴은 논리 idx 2 (= 니치 태그 5 − 3), 줄은 L915 switch
p.fix(S + "/consts[20]/value", old=5, new=2,
      evidence=u"d57540.ll:35645~35648 `switch i8 %41 [ i8 2, label %403 / i8 10, label %408 ] ;L915` — %41 = tag−3 논리 idx(34945~34947). 리터럴 5 는 이 switch 에 없다",
      behavior_change=False, found_by="new")
p.fix(S + "/consts[20]/src_line", old=907, new=915,
      evidence=u"d57540.ll:35648 switch 종결 `] ;L915`(case 라벨 줄 35646 은 !dbg 없음 — G12 도구 결함 ①이라 정정 뒤에도 후보에 안 잡힐 수 있다). 907 은 tower_score 줄이라 무관",
      behavior_change=False, found_by="reused")
p.fix(S + "/consts[20]/meaning", old=u"action 태그 5 = Around (35646) — Around 분기 bonus",
      new=u"SmallActionPlay 논리 idx 2 = Around(니치 태그 5−3 · 35646 `i8 2` · L915 switch) — Around 분기 bonus 선택",
      evidence=u"d57540.ll:35646 `i8 2, label %403` · 34945 `add nsw i8 %37, -3` · tcxdict --enum SmallActionPlay: Around 태그 5", behavior_change=False, found_by="new")
# consts[21]: 리터럴 13 은 EntityType Champion(L743·L953). LaneMinionPosition 은 switch 에서 논리 idx 10
p.fix(S + "/consts[21]/src_line", old=907, new=743,
      evidence=u"d57540.ll:35803 `%429 = icmp eq i64 %428, 13 ;L743` · 37545 `icmp eq i64 %1213, 13 ;L1775<953`. 907 에는 리터럴 13 없음(35647 은 `i8 10`)",
      behavior_change=False, found_by="reused")
p.fix(S + "/consts[21]/meaning", old=u"action 태그 13 = LaneMinionPosition (35647) · EntityType 태그 13 = Champion (35803·37545)",
      new=u"EntityType 태그 13 = Champion (35803 L743 · 37545 L953) — LaneMinionPosition 은 L915 switch 에 논리 idx 10(=니치 태그 13−3 · 35647 `i8 10`)으로 나타남",
      evidence=u"d57540.ll:35647 `i8 10, label %408` · 35803 · 37545", behavior_change=False, found_by="new")

# ── ★행동 오류: 위험 배수는 최대 HP 가 아니라 현재 hp(Entity+0x670) ──
p.fix(S + "/logic", old=u"max(maxhp,1)*max(s.", new=u"max(champ.hp,1)*max(s.",
      evidence=u"d57540.ll:35396~35402 `%229 = gep %33, i64 1648`(=Entity+0x670 hp) `%231 = umax(%230,1)` `%232 = mul %231,%228` `sdiv -100` ;L681 · 35409~35417 L685 동일 %231 · 36023~36034 `gep %33, 1648`…`sdiv 100` ;L781 · 36046~36057 L785. 최대 HP(+0x628=1576)는 L633·L664 에만 쓰인다. 오라클 T8(pts.risk=150,hp=1000,maxhp=2000): game=-4136 = mine(hp) ≠ mine(maxhp)=-8536 · R11d: game=960 = mine(hp) ≠ -2",
      behavior_change=True, found_by="new")
p.fix(S + "/reads[51]/note", old=u"35158·35343·35397", new=u"35158·35343 (L633·L664 HP% 분모만 — L681·685·781·785 의 위험 배수는 +0x670 현재 hp)",
      evidence=u"d57540.ll:35397 은 `load i64, ptr %229` 이고 %229 = gep %33, 1648(+0x670 hp) — 1576(+0x628) 아님", behavior_change=False, found_by="new")
p.fix(S + "/reads[55]/note", old=u"35155·35492 외 — 나눗셈 분모(0 이면 panic)",
      new=u"35155·35492 외 — 나눗셈 분모(0 이면 panic) · 35397·36024·36047 max(hp,1) 위험 배수(L681·685·781·785)",
      evidence=u"d57540.ll:35396 `gep %33, i64 1648` ;L681 · 36023 ;L781 · 36046 ;L785", behavior_change=False, found_by="new")

# ── TLS 셀 크기: (seed,tick,pid,InterActionCtx) = 8+8+8+72 = 96B ──
p.fix(S + "/writes[1]/name", old=u"(seed, tick, pid, InterActionCtx) 98B",
      new=u"(seed, tick, pid, InterActionCtx) 96B(살아있는 바이트 90 — +0x62..0x68 패딩 6B 는 miss 저장에 없음)",
      evidence=u"m05.ll:158 TLS 전역 `<{ [104 x i8], [1 x i8], [7 x i8] }>`(RefCell 8+96) · m00.ll:92526~92550 store 는 +8..+24 키 3개 + +32..+88 i64 8개 + +96/+97 i8 2개뿐(패딩 store 없음) · 오라클 TLS 직독 seed@8 tick@16 pid@24 nearest@32..",
      behavior_change=False, found_by="new")

# ── returns: 대상 소멸(Around/Trace/Attack/Skill/Skill2)은 L965 재조회로 0 이 되지 -99999 가 아니다 ──
p.fix(S + "/sig/returns", old=u" / Trace·Around·Attack·Skill·Skill2 대상 소멸=기본값)", new=u" / L931 Ult 대상 소멸 — L965 재조회 없음)",
      evidence=u"d57540.ll:37615~37629 %1255 preds %162·%167·%172·%177·%182(Around/Trace/Attack/Skill/Skill2 대상 None → -99999 phi)가 L965 get_entity_by_id 재조회로 이어지고 null 이면 %1260 phi `[ 0, %1255 ]` → 0 반환. -99999 로 직행하는 pred 는 %285·%161·%201·%212 뿐",
      behavior_change=True, found_by="new")

# ── open[1] 문면: m11 31594·31649 는 다른 작성자가 아니라 같은 with 인스턴스의 try_fold/find 아웃오브라인 헬퍼 ──
p.fix(S + "/open", old=u"interaction_ctx 를 인라인한 다른 작성자(m11.ll 31594·31649 의 closure 본체 2벌)가 어느 함수인지 — 담당 밖. TLS 미러 설계 시 그 호출자도 같은 키로 덮어쓸 수 있음(미탐색)",
      new=u"[해소·사실 서술] m11.ll 31594·31649 는 다른 작성자가 아니라 `Iterator::try_fold<find::check<interaction_ctx::{closure#1}/{closure#4}>>` 아웃오브라인 헬퍼로, 같은 with 인스턴스(m00.ll 91764·92054)가 부른다. INTER_CTX.with 심볼의 call 은 전 모듈에서 m05.ll:35131(이 함수) 1곳뿐 — 다른 작성자 없음",
      evidence=u"grep: `InterActionCtxEEE4withNCNvB1B_15interaction_ctx0B1z_EB1D_(` call = m05.ll 35131 만(m00 은 define) · m00.ll:91764 `invoke ptr @…find5check…interaction_ctx0s1_0…` · 92054 `…0s4_0…` · m11.ll:31594/31649 define 이름 = `…8try_folduNCINvNvB2p_4find5checkB1s_QNCNCNvNtC…interaction_ctx…`",
      behavior_change=False, found_by="new", force=True)
p.brief_error(u"mkpatch.fix 가 `/open` 경로에서 v3 open(list of dict)을 받아 `tgt == old` 로 비교해 항상 거부한다(문면이 정본에 그대로 있어도) — force=True 로 우회했다. applypatch 는 unknown 문면 치환이라 적용된다")

# ── ev 상향 (오라클 실행 확인) ──
p.ev(S + "/sig/params[1]", to=2, evidence=u"오라클 실행 확인: 41케이스 전부 rnd.clone().next_u64() 호출 전후 동일(rnd_changed_game=false · Trace check_kill_die_tick 경로 포함)", found_by="new")
p.ev(S + "/consts[11]", to=2, evidence=u"오라클 실행 확인: R5(applyed_damage=1000≥hp) · R6(분수 안) → game=-99999 =mine", found_by="new")
p.ev(S + "/consts[26]", to=2, evidence=u"오라클 실행 확인: T1/T5/T6/T7(d>reach·가시 적·my_die≤walk_tick) → game=-9999999 =mine", found_by="new")
p.ev(S + "/consts[8]", to=2, evidence=u"오라클 실행 확인: R11d risk_damage=3000·hp=1000·s.risk=150 → base_damage=1500(=3000+1000*150/-100) → game=960 =mine", found_by="new")
p.ev(S + "/consts[12]", to=2, evidence=u"오라클 실행 확인: R2 bs=880 → 880/4-2+82 = 300 =game", found_by="new")
p.ev(S + "/consts[13]", to=2, evidence=u"오라클 실행 확인: R2 220-2+82 = 300 =game (−2 없으면 302)", found_by="new")
p.ev(S + "/consts[14]", to=2, evidence=u"오라클 실행 확인: R2 100*660/800=82 → 300 =game", found_by="new")
p.ev(S + "/consts[16]", to=2, evidence=u"오라클 실행 확인: R4(ne=2,na=1,diff=1) 200*660/800=165 → 383 =game", found_by="new")
p.ev(S + "/consts[17]", to=2, evidence=u"오라클 실행 확인: R4f(ne=0,na=1,diff=-1) 75*660/800=61 → 279 =game", found_by="new")
p.ev(S + "/consts[23]", to=2, evidence=u"오라클 실행 확인: 적 dx=150000 → vis150_mask=1(R4e, 300) · 150001 → 0(R4f, 279) — dist² < 150000²+1 경계 실행 확정", found_by="new")
p.ev(S + "/consts[28]", to=2, evidence=u"오라클 실행 확인: T2(ne=1,na=1,diff=0) 80*330/100=264 =game", found_by="new")
p.ev(S + "/consts[30]", to=2, evidence=u"오라클 실행 확인: T8 tower_possible=(1000*150/100)/3=500 → -4136 =game", found_by="new")
p.ev(S + "/knobs[0]", to=2, evidence=u"오라클 실행 확인: coef 100(R2)·200(R4)·75(R4f) 세 arm 이 game 값을 가른다(300 / 383 / 279)", found_by="new")
p.ev(S + "/knobs[1]", to=2, evidence=u"오라클 실행 확인: R2 880/4-2+100*660/800 = 300 =game", found_by="new")
p.ev(S + "/knobs[4]", to=2, evidence=u"오라클 실행 확인: 150000 in / 150001 out (R4e/R4f)", found_by="new")
for i in range(57, 63):
    p.ev(S + "/mem[%d]" % i, to=3, evidence=u"오라클 TLS 직독(extern #[thread_local] INTER_CTX0s_0 112B): +0x20 tag/+0x28 id · +0x30/+0x38 · +0x40/+0x48 · +0x50/+0x58 · +0x60 mask · +0x61 count — 41케이스 전부 재구현 ictx 와 일치", found_by="new")
# ⚠아래 reads 3행 삽입(G18) 뒤 writes 의 v3 인덱스가 3 밀린다 → mem[63]→mem[66] · mem[64]→mem[67] (삽입 후 인덱스)
p.ev_up.append({"path": S + "/mem[66]", "from": 4, "to": 3, "guard": "borrow", "found_by": "new",
                "evidence": u"오라클 TLS 직독: 호출 후 borrow@+0x0 == 0 (41/41) · 상태 바이트 @+0x68 == 1"})
p.ev_up.append({"path": S + "/mem[67]", "from": 4, "to": 3, "guard": "(seed, tick, pid, InterActionCtx)", "found_by": "new",
                "evidence": u"오라클 TLS 직독: seed@+0x8=1234(game.seed()) tick@+0x10=game.tick() pid@+0x18=player.info.id — 41/41 · TWOCALL: 같은 키 재호출 = 셀 불변(적 이동 무시) · tick+1 = 재계산"})
# ── G18: PositioningScore(sret) 소비 바이트 행 추가 (reads 끝 = at 63) ──
for k, row in enumerate([
    {"base": "PositioningScore(sret %13 RunAway / %12 Trace)", "offset": "0x0", "name": "risk",
     "note": "35373(L679 dest)·35984(L779 pts) · L681 max(hp,1)*max(risk,0)/-100 · L781 /100"},
    {"base": "PositioningScore(sret)", "offset": "0x8", "name": "tower_risk",
     "note": "35376(L679)·35988(L779) · L685 /-100 · L785 /100 → /3"},
    {"base": "PositioningScore(sret)", "offset": "0x31", "name": "on_periodic_trajectory(Option<PositioningScore> 니치 · ==2 → None)",
     "note": "35378~35387 L680 · 35991~36008 L781 — 2 면 dest/pts=None 경로(risk_damage/risk_possible_tower 기본값). 이 함수가 읽는 sret 바이트는 [0,16)+{49} 뿐(+0x10~+0x30·+0x32~ 미독) — position_eval_at 이 +0x31 을 항상 store 하는지는 187 검증 항목"}]):
    p.errors.append({"op": "insert", "path": S + "/reads", "at": 63 + k, "guard": row["name"], "guard_key": "name", "new": row,
                     "kind": u"보강", "found_by": "new", "behavior_change": False,
                     "evidence": u"d57540.ll:35373 `%219 = load i64, ptr %13` · 35375~35376 `gep %13, i64 8` load · 35378~35382 `gep %13, i64 49` `icmp eq i8 %223, 2 ;L680` · 35984/35987/35991 동일(%12). G18 이 지적한 `s.tower_risk` 행"})
p.ev(S + "/mem[23]", to=3, evidence=u"오라클: RunAway/Around/Trace/Attack 생성물의 +0xb1 = 3/5/14/15 직독 · 각 분기 진입 확인", found_by="new")
p.ev(S + "/mem[24]", to=3, evidence=u"오라클: Trace 는 +0x60 = target id · Around/Attack 은 +0x8 = target id (직독) — chk 「오귀속」은 사전 조회 결함", found_by="new")

p.brief_error(u"지시문(v24_prompt_D)이 배치D 몫이 아닌 항목을 대량 요구한다 — PositioningScore sret 작성자(187)·Option<Input>/Option<InputTarget>·&mut self PathFinder drop(182/184)·POS_EVAL_CACHE/EPC_CACHE/PE_*·internal 3(180/181/185) exe 대응표는 DOSSIER_D 담당(183 만)에 없다. 183 에서 닿는 것(INTER_CTX·PositioningScore 소비 바이트)만 했다")
p.brief_error(u"§5 스키마로 `sig.tls` 하위 키(name/layout/key/value_semantics…)를 고칠 경로가 없다 — PATH 는 `/sig/tls/key` 를 (outer=sig, field=tls, key=key) 로 받지만 apply_error 가 dict 필드를 처리하지 않는다(문자열 아님·문면 못 찾음). TLS 절 정정 6건은 산문 보고로만 낸다")
p.brief_error(u"§4 G12 후보 「consts[20] 실제 후보=[593]」은 aux with 인스턴스(m00.ll 92169 `icmp ult %286, 5 ;L1916<…<593`)의 슬롯 상한 5 다 — 본체 상수와 무관한 줄을 후보로 준다(도구가 aux 도 훑는다는 설명이 지시문에 없다)")
p.brief_error(u"v2 `calls` 는 문자열 배열이라 `op:insert`(객체만 허용)로 빠진 콜리(`AbstractGameWithCache::iter_towers_without_nexus` · `is_recent_visible` 의 closure#0 경유)를 추가할 수 없다")
p.brief_error(u"G3 「siblings.plan == null · 화이트리스트 없음」은 오탐 — `_RNvNtCshdEBA0ozCnw_7game_ai12action_score17interaction_score` 는 정상 파싱되는 모듈 자유 함수(Nv in Nt action_score)다. 화이트리스트에 interaction_score 를 넣어야 한다")
p.save()
