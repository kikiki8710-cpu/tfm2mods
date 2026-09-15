# -*- coding: utf-8 -*-
"""24차 배치E patch.json 생성 — mkpatch 참조구현 사용. 실행: python -X utf8 mkpatch24e.py"""
import sys; sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import mkpatch

p = mkpatch.Patch(round=24, batch="E")
ORA = u"오라클 o24e(_verify24/E/oracle/o24e_184.log · 케이스당 프로세스 1개 · link_name 직접 링크)"

# ───────────────────────── 184 SmallActionRecall::get_input ─────────────────────────
# ★L719 극성 반전 — Return 은 「도보 시간이 tps*3+60 이상」일 때
p.fix("/specs[184]/logic",
      old=u"if is_safe_recall(version,rnd,player,data,ps) && time < setting.tick_per_second*3 + 60 {    // L719 (is_safe_recall 먼저 평가)",
      new=u"if is_safe_recall(version,rnd,player,data,ps) && time >= setting.tick_per_second*3 + 60 {   // L719 (is_safe_recall 먼저 평가 · ★도보 시간이 3초+60틱 **이상**일 때만 귀환 시전, 그 안쪽은 걸어서 간다 — m08.ll:100017~100021 `%124 = icmp ult i64 %117, %123 ; br i1 %124, label %115(계속), label %125(store i64 1)` · 오라클 dist 480000/240000(time 480/240)→Some(Return), 239999(time 239)→Move)",
      evidence=u"m08.ll:100017 `%124 = icmp ult i64 %117, %123` · 100018 `br i1 %124, label %115, label %125` · 100021 `store i64 1, ptr %0`(=Return) 은 %125 즉 time<thr 가 **거짓**인 쪽 · " + ORA + u": dist 480000 speed 1000 (time 480≥240) → Some(Return) tag 1 · dist 240000 (time 240) → Some(Return) · dist 239999 (time 239) → Some(Move{240000,944000}) · dist 100000 → Move. 명세 문면대로면 480000 케이스가 Move 여야 하는데 Return 이다",
      behavior_change=True, found_by="new")
p.fix("/specs[184]/one_line",
      old=u"안전하고 가까우면 Input::Return",
      new=u"안전하고 우물까지 도보 시간(dist/move_speed)이 tps*3+60 틱 이상이면 Input::Return(그보다 가까우면 걸어서 간다)",
      evidence=u"m08.ll:100017~100021 극성(위 logic 정정) · " + ORA + u" 480000→Return / 100000→Move",
      behavior_change=True, found_by="new")
p.fix("/specs[184]/consts[4]/meaning",
      old=u"tps*3 (3초). time < tps*3+60 이면 Input::Return.",
      new=u"tps*3 (3초). time >= tps*3+60 이면 Input::Return(time < 이면 도보 경로 계속 — 오라클 240→Return·239→Move).",
      evidence=u"m08.ll:100015 `%122 = mul i64 %121, 3` · 100017 `icmp ult %117,%123` · 100018 br 방향 · " + ORA,
      behavior_change=True, found_by="new")
p.fix("/specs[184]/consts[5]/meaning",
      old=u"+60틱 여유 (tps=60 이면 1초). time < tps*3+60",
      new=u"+60틱 여유 (tps=60 이면 1초). time >= tps*3+60 이면 Return(경계 240 포함 — 오라클 time 240 → Return)",
      evidence=u"m08.ll:100016 `%123 = add i64 %122, 60` · 100017~100018 · " + ORA + u" dist 240000 speed 1000 → Some(Return)",
      behavior_change=True, found_by="new")
p.fix("/specs[184]/knobs[1]/what",
      old=u"즉시 Return 허용 시간", new=u"즉시 Return 최소 도보 시간",
      evidence=u"m08.ll:100017~100021 극성 · " + ORA, behavior_change=False, found_by="new")
p.fix("/specs[184]/knobs[1]/effect",
      old=u"올리면 우물에서 더 멀어도(이동 시간 기준) 안전 판정만 통과하면 그 자리에서 귀환 시전",
      new=u"올리면 우물에서 더 멀리(도보 시간 기준) 있어야 그 자리에서 귀환을 시전하고 그 안쪽에선 걸어서 우물로 간다 · 내리면(0) 우물 밖이면 어디서든 안전 판정만 통과하면 귀환 시전. 안전 판정(is_safe_recall)은 별도 AND 조건",
      evidence=u"m08.ll:100017 `icmp ult i64 %117, %123` 참 → %115(도보 계속) · 거짓 → %125 Return · " + ORA + u" 480000/240000 → Return, 239999/100000 → Move",
      behavior_change=True, found_by="new")
# rnd 실소비처
p.fix("/specs[184]/sig/params[3]/role",
      old=u"이 함수 본문은 직접 안 읽고 is_safe_recall · SliceRandom::choose(version<2) · positioning_window_pick · PathFinder::new_target_with_policy/update_path 에 그대로 전달.",
      new=u"이 함수 본문은 직접 안 읽고 is_safe_recall · SliceRandom::choose(version<2) · positioning_window_pick · PathFinder::new_target_with_policy/update_path 에 그대로 전달. ★실소비는 choose(version<2)·positioning_window_pick(m03.ll define %1 readnone 없음)뿐 — is_safe_recall(m07.ll define `ptr noalias noundef readnone … dereferenceable(320) %1`)·new_target_with_policy<closure#10>(m03.ll:85370 `readnone` %1)·update_path<closure#11>(m03.ll:28139 `readnone` %1)은 rnd 를 읽지 않고 PathFinder::get_input(m03.ll:137276)은 rnd 인자가 없다. 오라클: version 55 전 케이스(Return·direct_heal·후보탐색·2회 호출) StdRng 320B 불변, version 1 후보탐색(choose)에서만 변화",
      evidence=u"m07.ll is_safe_recall define `readnone` %1 · m03.ll:85370 · m03.ll:28139 · m03.ll:137276 · " + ORA + u" rnd_changed 열",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[184]/sig/params[6]/role",
      old=u"readonly. 직접 읽는 필드: cx(0xab8) · cy(0xac0). 나머지는 positioning_score_at_* 에 전달",
      new=u"readonly. 직접 읽는 필드: cx(0xab8) · cy(0xac0). 나머지는 positioning_score_at_*·is_safe_recall 에 전달되지만 ★그 콜리들도 ps 를 읽지 않는다 — small_action::positioning_score_at_cell/at_position(m11.ll:53056/53069)은 position_eval::position_score_at_cell/at_position(m07.ll:24714/…)로 그대로 넘기고 두 함수 본문에 %4(ps) 사용 0건, position_eval_at(m07.ll:24507)은 ps 인자 자체가 없다 · is_safe_recall(m07.ll define) 본문도 %4 사용 0건. ⟹ 이 함수 전이 경로에서 PositioningScoreData 는 cx/cy 16B 만 살아있다(185 와 동일 · 단 이 define 은 hidden 이라 exe 인자는 포인터 그대로)",
      evidence=u"m11.ll:53056~53080 tail call 전달 · m07.ll:24714 position_score_at_cell 본문 %4 사용 0건(define 줄 제외) · m07.ll position_score_at_position 본문 %4 사용 0건 · m07.ll:24507 position_eval_at 인자 (sret,version,player,data,x,y,purpose) · is_safe_recall 본문 %4 사용 0건",
      behavior_change=False, found_by="new", kind=u"보강")
# TLS 절(v2 signature.tls · v3 는 null) — 간접 접점 보강
p.fix("/specs[184]/sig/tls",
      old=u"이 함수 자체는 TLS 작성자도 소비자도 아니다.",
      new=u"이 함수 자체는 TLS 작성자도 소비자도 아니다. ★단 간접 접점이 하나 더 있다(24차 E): positioning_score_at_cell/at_position(L752·L838·L841·L906·L929) → position_eval::position_score_at_cell/at_position → position_eval_at(m07.ll:24507) 이 POS_EVAL_CACHE(thread_local RefCell<PosEvalCache> · 조회 클로저 m00.ll:75374 · 저장 클로저 m00.ll:75614)를 **조회+저장**한다. 키 PeKey 40B=(player.info.id@PlayerState+0x928, x, y, purpose@24, version@32) · idx=((purpose코드 | (y<<21)^(id<<42)) ^ x)*0x9E3779B97F4A7C15 >>55 (512 슬롯 직접사상 · 충돌=덮어씀) · PeSlot 104B={tick@0,key@8,val 56B@48} · None 니치=slot+97(val.on_periodic_trajectory)==2 · 히트=tag!=2 && slot.tick==tick && key 5필드 일치 · seed(vtable+0x20)≠cache.seed 면 512 슬롯 전부 무효화 후 seed 갱신 · 저장은 seed 가 같을 때만. ⟹ 미러 조건: 이 함수가 부르는 순서(goal(L752) → dx 0..7/dy 0..7 루프의 [cell(L838) → next(L841)] → 옛 goal(L906) → 새 goal(L929))대로 같은 (id,x,y,purpose=Recall,version) 키를 같은 tick·seed 에 넣어야 슬롯 상태가 같다. 같은 키·tick·seed 2회 조회 시 두 번째는 uncached 를 안 부르고 캐시값을 돌려준다(오라클 pe: 같은 tick 에 적 타워를 셀로 옮겨도 s2==s1(tower_risk 0 유지) · tick+1 → 재계산 tower_risk 150). 그 아래 position_eval_at_uncached 는 PE_PLAYER_CTX(Cell)·PE_CAND_MASKS·ATTACK_DMG_CACHE·TOWER_MINION_CNT_CACHE·EPC_CACHE·fight_check slot_cc_time/slot_ready(HashMap)·SIEGE_STANCE_CACHE·RESOLVE_FIGHT_CACHE 를 만진다(187 position_eval_at 명세 소관). update_path<closure#11> 경유 lethal_wave_position → ATTACK_DMG_CACHE 접점도 있다.",
      evidence=u"m07.ll:24507~24711 position_eval_at(키 구성 24533~24541 · 해시 24587~24593 · 조회 with 24597 · 히트 분기 24601 `icmp eq i8 %62, 2` · 저장 with 24645) · m00.ll:75374~75611 조회 클로저(seed 비교 75395~75399 · 512 클리어 75491~75497 · slot 비교 75415~75470) · m00.ll:75614~75727 저장 클로저 · scratchpad24E/tlstree.py BFS(depth 8) · " + ORA + u" pe[0]/pe[1]",
      behavior_change=False, found_by="new", kind=u"보강", force=True)

# ───────────────────────── 185 get_input_target ─────────────────────────
p.fix("/specs[185]/knobs[0]/effect",
      old=u"acc≥1000 이면 오차 0",
      new=u"acc=1000(상한 — skill_hit_accuracy = 150+min(stat,100)*85/10 ∈ [150,1000], g15.ll:125941 `range(i64 150, 1001)`)이면 오판 확률은 0 이지만 타이밍 오차는 te=max(1000-acc,1)=1 이라 gen_range(999..=1001) 이 여전히 1회 돈다(±1‰ · rnd 소비 1회) · 조준 오프셋(apply_aim_offset_*)만 acc>999 에서 0(rnd 소비 없음)",
      evidence=u"g15.ll:125941~125952 skill_hit_accuracy = umin(stat,100)*85/10+150 · m04.ll:33296 `%72 = umax(i64 %68, 1)` · 33305/33308 범위 1000±te · 33318 gen_range 호출은 %69(speed==0) 분기 외 무조건 · m04.ll:39228 `icmp ugt i64 %2, 999` / 39352",
      behavior_change=True, found_by="new")
p.fix("/specs[185]/sig/params[5]/role",
      old=u"그 콜리도 `_positioning_score` 미사용(m07.ll:24714 본문 gep 0건)",
      new=u"그 콜리도 `_positioning_score` 미사용(m07.ll:24714 본문 gep 0건 · position_eval_at 은 ps 인자 자체가 없음). ★24차 재검증: 본문 %5 사용 전수 = gep 2744/2752 4건(m04.ll:33435/33438/33678/33681) + position_score_at_cell 인자 전달 24건 — 다른 필드 읽기 0건. exe 호출자 쪽(argscan --caller): ult 0xd35666 `movups xmm0, xmmword ptr [rax+0xab8]` → 0xd35677 `movups [rsp+0x28], xmm0` · skill 0xd359f1/0xd35a02 동일 — cx·cy 16B 가 스택 인자 6·7(entry+0x28/+0x30)로 들어간다(ArgumentPromotion 확정 · FAKE 재구성 가능 판정 유지)",
      evidence=u"argscan.py 0xd31f20 --caller 0xd354c0 / --caller 0xd35930 · argscan.py 0xd31f20(스택 6 + 레지스터 4 = 10) · m04.ll 33111~35159 `%5` grep 전수",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[185]/sig/tls",
      old=u"직접 접점 없음(thread_local!/LocalKey::with 0건).",
      new=u"직접 접점 없음(thread_local!/LocalKey::with 0건). ★간접 접점(24차 E): auto_target 경로(L537~549 · L382~394)의 position_score_at_cell 24회(6×6 · dx 바깥/dy 안쪽 순서 · purpose AttackStance 태그 12) → position_eval_at(m07.ll:24507) 이 POS_EVAL_CACHE(thread_local RefCell<PosEvalCache> · m00.ll:75374 조회/75614 저장)를 조회+저장한다 — 키 (player.info.id, x=(cx-3+dx)*32000+16000, y=…, purpose 12, version) · 512 슬롯 직접사상 · 히트 조건 tick·seed·키 일치. 미러는 같은 순서로 36셀을 넣어야 슬롯 상태가 같고, 셀 범위 밖(x<0||y<0||x>29||y>29)은 position_score_at_cell 이 risk 9999 센티널을 돌려주며 TLS 를 건드리지 않는다(m07.ll:24714~24725). 같은 키·tick·seed 2회 조회 시 두 번째는 uncached 없이 캐시값(오라클 pe[1] purpose AttackStance: 같은 tick 세계 변화 무시 s2==s1 · tick+1 재계산). uncached 아래 TLS(PE_PLAYER_CTX·PE_CAND_MASKS·ATTACK_DMG_CACHE·TOWER_MINION_CNT_CACHE·EPC_CACHE·slot_cc_time/slot_ready·SIEGE_STANCE_CACHE·RESOLVE_FIGHT_CACHE)는 187 소관.",
      evidence=u"scratchpad24E/tlstree.py BFS(루트 get_input_target · depth 8) · m07.ll:24507~24711 · m00.ll:75374~75727 · " + ORA + u" pe[1]",
      behavior_change=False, found_by="new", kind=u"보강", force=True)
# (exe_args.note 보강은 applypatch 가 dict 하위 키 경로를 못 받아 산문으로 보고 — 도구 결함)

# ───────────────────────── ev 상향(오라클 실행 확증) ─────────────────────────
EV = ORA + u": "
for path, ev in [
    ("/specs[184]/consts[3]", u"dist 200000 → direct_heal(prog_best_tick 0 유지 = 절2/3 생략) / 200001 → 후보탐색(prog_best_tick 1000 으로 리셋) — 경계 <200001 확정"),
    ("/specs[184]/consts[4]", u"time 240 → Return / 239 → Move (tps=60 · 3*60+60=240)"),
    ("/specs[184]/consts[5]", u"time 240 → Return / 239 → Move"),
    ("/specs[184]/consts[6]", u"Return 케이스 sret +0 = i64 1 (tag=1)"),
    ("/specs[184]/consts[7]", u"우물 안(56000,928000 · x≤rx 64000) → None sret +0 = -1 (tag=-1)"),
    ("/specs[184]/consts[2]", u"version 1: 후보탐색에서 rnd 변화(choose) · self+0x40 policy.direct_cross=1 · Return 판정은 version 무관(480000 → Return)"),
    ("/specs[184]/knobs[0]", u"200000 → direct_heal / 200001 → 후보탐색"),
    ("/specs[184]/knobs[1]", u"time 480/240 → Return · 239/100/4 → Move (극성 정정 후 문면과 일치)"),
    ("/specs[184]/mem[14]", u"fountain[0]=(0,896000,64000,960000) 읽어 우물 안 판정 → None (dist 24000 speed 100 케이스도 x=56000≤64000 이라 L691 이 L719 보다 먼저 None)"),
    ("/specs[184]/mem[19]", u"move_speed 0 → exit 101(div_by_zero 패닉) · speed 1000/100000 으로 time 이 갈림"),
    ("/specs[184]/mem[27]", u"pf_tag 2(None) → 0(Some) 전이 관측 · 2회 호출 시 0 유지"),
    ("/specs[184]/mem[41]", u"direct_heal/후보탐색 케이스 goal_x=32000 기록(healp.x)"),
    ("/specs[184]/mem[42]", u"goal_y=928000 기록"),
    ("/specs[184]/mem[43]", u"goal_committed 0→1 (+0x80) · Return/None 경로는 미기록(self 136B 불변)"),
    ("/specs[184]/mem[44]", u"2회 호출에서 prog_best_dist_sq u64::MAX → 226885(+0x70 · L734) 기록 · 후보탐색 1회차엔 L937 리셋(-1 유지)"),
    ("/specs[184]/mem[45]", u"후보탐색 케이스 prog_best_tick 0→1000(+0x78 · L938) · direct_heal 케이스는 미기록"),
    ("/specs[184]/mem[47]", u"self+0x0..+0x45 72B 가 새 PathFinder 로 채워짐(pf_tag 2→0 · path/planned_verdict Box 포인터 기록) · 2회 호출에서 두 Box 포인터 동일(재할당·drop 없음)"),
]:
    p.ev(path, evidence=EV + ev, to=2, found_by="new")

# ───────────────────────── 지시문(도시에) 오류 ─────────────────────────
p.brief_error(u"§3 「명세 전문 무손실」이 v3 기준이라 v2 `signature.tls`(184·185 모두 문면 있음)·`signature.exe_args`(185 · ArgumentPromotion 대응표+FAKE 판정)·`signature.calls_contract`(184) 가 도시에에 안 실렸다(v3 sig.tls=null). 지시문 ③ 「signature.tls 검증」은 도시에만 봐선 검증할 대상 자체가 안 보인다 — mkdossier 가 v2 signature 하위 키를 함께 실어야 한다.")
p.brief_error(u"§4 G7 「open 이 shared.is_recent_visible 를 부르는데 확정」— 184 open[1] 은 콜리 존재가 아니라 blackboard[1-team] 을 self 로 넘긴 의미를 묻는 항목이라 콜리 이름 매칭만으로 과열림 판정하면 오탐이다. (다만 실제 의미는 22차 A 가 112 에서 오라클로 닫은 사실이라 notes 로 옮기는 것이 맞다 — 산문 참조)")
p.brief_error(u"지시문 ③ 의 INTER_CTX·EPC_CACHE·PE_CAND_MASKS·PE_PLAYER_CTX·ATTACK_DMG_CACHE·TOWER_MINION_CNT_CACHE 는 184/185 가 직접 만지지 않고 position_eval_at_uncached(187) 안에서만 닿는다 — 배치 E 몫은 POS_EVAL_CACHE(+PathFinder scratch)뿐인데 배치 공통 문안이라 범위가 안 갈려 있다.")
p.brief_error(u"applypatch/mkpatch 가 `/specs[i]/sig/exe_args/note` 같은 **dict 하위 키** 경로를 못 받는다(parse_path 가 idx 없는 3단 경로를 배열 원소 분기로 넘겨 `int(None)` TypeError — applypatch.py:520 · 실측 --dry 크래시). 185 exe_args.note 보강분은 산문으로만 냈다.")
p.brief_error(u"§1 표의 exe 열 「dbd260 (None) · None바이트 · None명령」— exe 크기·명령수가 None 으로 찍힌다(fnprobe 미연결). 185 도 동일.")
p.brief_error(u"§4-b 「callees ev4 행 = 판정 보류」 안내와 달리 184 callees[10~12] is_empty(PatchSetting/ModAiRegistry/PublicChampionEvidence)·[30~32] push(Staff/Athlete/Contract serialize)·[34~36] tick 후보 3개 중 2개는 이 함수와 무관한 동명 함수다 — 「후보 3개 중 상위 3개」로 실린 행은 판정 보류가 아니라 잡음이라 별도 표기가 필요하다.")
p.save()
