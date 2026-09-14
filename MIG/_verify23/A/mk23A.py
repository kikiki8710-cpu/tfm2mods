# -*- coding: utf-8 -*-
"""mk23A.py — 23차 배치 A patch.json 생성(mkpatch 참조구현 사용)."""
import sys, io
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import mkpatch

p = mkpatch.Patch(round=23, batch="A")
ORA = u"오라클 실행 확인(23차A o23A.rs · 케이스당 프로세스 1개 · 로그 _verify23/A/oracle/o23A_case*.log): "

# ───────── 134 check_cell ─────────
p.fix("/specs[134]/logic",
      old=u"start_in_top = is_top_side(ctx, sx*32000+16000, sy)   (L1264 · 인라인 map_regions.rs:22~24)",
      new=u"start_in_top = is_top_side(ctx, sx*32000+16000, sy*32000+16000)   (L1264 · 인라인 map_regions.rs:22~24 · dbg_value `y = %7` 은 접힘 아티팩트 — 실제 y 인자는 셀 중심 sy*32000+16000)",
      evidence=u"m08.ll:112397 `%51 = mul nsw i64 %7, -32000, !dbg`(L1264) · 112406 `%56 = add i64 %55, -16000`(L1264) · 112407 `%57 = add i64 %56, %51`(map_regions.rs:22<1264) ⟹ ry = height − (sy*32000+16000) 이므로 is_top_side 의 y 인자 = sy*32000+16000. 오라클 case80~83 1600케이스에서 이 식으로 재구현한 판정이 game 과 0 mismatch",
      behavior_change=False, found_by="new", kind=u"보강")
for k in range(7):
    p.ev("/specs[134]/consts[%d]" % k, to=2, found_by="new",
         evidence=ORA + u"case80(None)·81(Outline)·82(Inline)·83(혼합) 각 400 무작위 셀조합 = 1600케이스 game==mine 0 mismatch(Allow 1134·Soft 429·Danger 37 전부 발화) — hidden 함수를 `#[link_name]` 로 직접 진입")
for k in range(3):
    p.ev("/specs[134]/knobs[%d]" % k, to=2, found_by="new",
         evidence=ORA + u"1600케이스 0 mismatch — Outline 대각선 횡단 시 lane_seq(Mid, team1)=[26,12,3,6,4,19,0] 면제·Soft(1)·Danger(2) 등급 전부 실행 관측")
for k in range(5, 12):
    p.ev("/specs[134]/sig/params[%d]" % k, to=2, frm=(3 if k == 5 else 4), found_by="new",
         evidence=ORA + u"out_line 0/1/2 및 sx,sy,nx,ny,tx,ty 무작위 1600케이스에서 재구현과 0 mismatch")

# ───────── 135 v3_lethal_tower_position ─────────
p.fix("/specs[135]/open[2]/q",
      old=u"L294 의 소스 표기가 `.map(|e| e.range()+20000).unwrap_or(20000)` 인지 `.map_or(0,..)+20000` 인지 — IR 은 phi [range+…+20000, 20000] 로 접혀 외연 동일(표기 불가)",
      new=u"L294 의 소스 표기 = `.map(|e| e.range(t)).unwrap_or(0) + 20000` 로 확정 — m06.ll:35268 `#dbg_value(i64 0, !51901, …, !51907)` 의 !51907 = `line 0 in unwrap_or<u64>` ← inlinedAt tower_discipline.rs:294(unwrap_or 의 파라미터 `default` = 0), map 프레임은 option.rs:1162<294. phi [range+…+20000, 20000] 은 0+20000 접힘. (`unwrap_or(20000)`·`map_or` 는 스코프명·default 값과 불일치)",
      evidence=u"m06.ll:35268 `#dbg_value(i64 0, !51901, !DIExpression(), !51907)` · `dloc m06.ll 51907` → line 0 in unwrap_or<u64> [option.rs] ← line 294 in closure$0 [tower_discipline.rs] · 35276~35280 의 !dbg 사슬 `L26<294<1162<294`(effect.rs:26 ← option.rs:1162 map) · 오라클 case20 타워 range=0 일 때 r=20000+radius(10000)=30000 경계 302000 true/302001 false",
      behavior_change=False, found_by="new", kind=u"보강")
for k in (0, 1, 2):
    p.ev("/specs[135]/consts[%d]" % k, to=2, found_by="new",
         evidence=ORA + u"case20(v2,hp0) 418점 true 74 · case21(v1) 전부 false · case22(hp 999999 비치명) 전부 false · case23(v3) = case20 — 4×418 케이스 0 mismatch. 경계: 타워(272000,48000) r=30000 에서 (302000,48000) true / (302001,48000) false ⟹ dist² <= r²")
for k in (0, 1):
    p.ev("/specs[135]/knobs[%d]" % k, to=2, found_by="new",
         evidence=ORA + u"version 1→false / 2·3→판정 · 여유 20000 이 r=range(0)+20000+radius(10000)=30000 경계로 실측(302000 true·302001 false)")
p.ev("/specs[135]/sig/params[0]", to=2, found_by="new", evidence=ORA + u"version=1 이면 418점 전부 false, 2·3 이면 동일 판정")

# ───────── 136 v23_should_break_objective_hunt_anchor ─────────
# G16 P1 — (u64,u64) 튜플 ScalarPair 2슬롯: 오탐. role 에 IR 레지스터를 명기해 게이트를 닫는다.
p.fix("/specs[136]/sig/params[0]/role", old=u"본문에서 직접 읽지 않음 — 헬퍼 2종에 그대로 전달(m15.ll:56711·56713·56725·56727)",
      new=u"IR %0. 본문에서 직접 읽지 않음 — 헬퍼 2종에 그대로 전달(m15.ll:56711·56713·56725·56727)",
      evidence=u"m15.ll:56682 define 인자 6개 = (%0 player, %1 data, %2 champ, %3 objective, %4 camp_pos.0, %5 camp_pos.1) — 5번째 소스 인자 `(u64,u64)` 가 Rust ABI ScalarPair 로 2슬롯. DeadArgElim/ArgumentPromotion 아님(readonly·captures 속성 그대로, 인자 순서 소스와 동일). G16 `ir_slots` 가 튜플을 1슬롯으로 세어 P1 이 깨진 것 = 오탐",
      behavior_change=False, found_by="new", kind=u"오탐")
p.fix("/specs[136]/sig/params[1]/role", old=u"본문에서 직접 읽지 않음 — 헬퍼에 전달", new=u"IR %1. 본문에서 직접 읽지 않음 — 헬퍼에 전달",
      evidence=u"m15.ll:56682 define 두 번째 인자 `ptr … dereferenceable(24) %1`", behavior_change=False, found_by="new", kind=u"오탐")
p.fix("/specs[136]/sig/params[2]/role", old=u"내 챔피언. x/y 만 읽음(56707~56710)", new=u"IR %2. 내 챔피언. x/y 만 읽음(56707~56710)",
      evidence=u"m15.ll:56682 세 번째 인자 `dereferenceable(1728) %2` · 56707 `gep %2, 1632`", behavior_change=False, found_by="new", kind=u"오탐")
p.fix("/specs[136]/sig/params[3]/role", old=u"사냥 대상 오브젝트. stat_cached.hp(max)·hp 만 읽음(56689~56696)", new=u"IR %3. 사냥 대상 오브젝트. stat_cached.hp(max)·hp 만 읽음(56689~56696)",
      evidence=u"m15.ll:56682 네 번째 인자 `dereferenceable(1728) %3` · 56689 `gep %3, 1576`", behavior_change=False, found_by="new", kind=u"오탐")
for k in (0, 1, 2, 3, 5, 6):
    p.ev("/specs[136]/consts[%d]" % k, to=2, found_by="new",
         evidence=ORA + u"case40~48: hp%=20(20/100·2099/10000)→false, 21(21/100·2100/10000)→ 적 배치에 따라 true — 21 경계 양쪽 실측 · case43 personal_enemies=3>1 ∧ 3>allies(1) → true · case42 camp_enemies=3>2 ∧ 3>=allies(0)+2 → true · 헬퍼 반경 180000/160000/180000 으로 직접 호출한 계수로 재구현 = game 9/9 MATCH")
for k in (0, 1, 2, 3, 4):
    p.ev("/specs[136]/knobs[%d]" % k, to=2, found_by="new", evidence=ORA + u"case40~48 9케이스 game==mine (21 경계·적 최소수·마진 전부 발화)")
p.ev("/specs[136]/mem[0]", to=3, found_by="new", evidence=ORA + u"case49 stat_cached.hp=0 → `attempt to divide by zero` at objective_helpers.rs:60:6 (함수 자신의 udiv · 가드 없음 확정)")
p.ev("/specs[136]/sig/params[4]", to=2, found_by="new", evidence=ORA + u"case42 camp_pos=(500000,500000) 에 적 3 배치 → 캠프 과부하 true (x=%4,y=%5 ScalarPair 전달 확인)")

# ───────── 137 v48_on_cast_line ─────────
p.fix("/specs[137]/notes[0]/q",
      old=u"exe 0xcd0480 의 fastcc 레지스터 배치(argscan 미실시) — IR 인자 5개는 소스 인자와 1:1 이라 승격 없음은 확정",
      new=u"exe 0xcd0480 의 fastcc 인자 배치 = IR 과 동일 5개(argscan 23차A 실시: pushes r15,r14,r12,rsi,rdi,rbx · sub rsp,0x298 · 스택 인자 1 = entry+0x20 ↔ IR %4(py) · 레지스터 4 = %0~%3) — 승격/소거 없음 확정",
      evidence=u"`python -X utf8 argscan.py 0xcd0480` → `entry_rsp+0x20 (arg5 · IR %4) x1: 140cd053c: mov rbx, qword ptr [rsp + 0x2f0] [size 8] ⟹ 스택 인자 1개 + 레지스터 4 = exe 인자 5` · IR m02.ll:66030 define 인자 5(ptr %0, ptr %1, i64 %2, i64 %3, i64 %4)",
      behavior_change=False, found_by="new", kind=u"보강")

# ───────── 138 EpicPokeSubPlan::score ─────────
NM21 = u"Around/AroundHide/LaneMinionPosition.target (get_action→SmallAction::Around{target_id})"
p.fix("/specs[138]/mem[21]/name", old=u"Around/AroundHide/LaneMinionPosition.target_id (get_action→SmallAction::Around{target_id})", new=NM21,
      evidence=u"tcxdict: SmallActionAround 0x8 `target usize` · SmallActionAroundHide 0x8 `target` · SmallActionLaneMinionPosition 0x8 `target` — 필드명은 `target`(get_action 이 SmallAction::Around{target_id} 로 옮김). m02.ll:46945 `gep %6, 8`",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[138]/mem[22]/name", old=u"AroundRegion x (idx4) / AroundPositionBush y (idx8)", new=u"AroundRegion.goal_x (idx4) / AroundPositionBush.target_y (idx8)",
      evidence=u"tcxdict: SmallActionAroundRegion 0x10 `goal_x` · SmallActionAroundPositionBush 0x10 `target_y`. m02.ll:46958 phi [16,%66]=idx4 x, [16,%75]=idx8 y",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[138]/mem[25]/name", old=u"AroundPosition x", new=u"AroundPosition.around_input.target_x (idx7 · WaitAroundInput+0x0)",
      evidence=u"tcxdict `SmallActionAroundPosition 0x30` → around_input.target_x (WaitAroundInput 40B: target_x@0 target_y@8 d@0x10 now_goal_x@0x18 now_goal_y@0x20). m02.ll:46958 phi [48,%74]=idx7 — goal_x(0x8)·target_x(0x20) 가 아니라 around_input 의 target 이다",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[138]/mem[26]/name", old=u"AroundPosition y", new=u"AroundPosition.around_input.target_y (idx7 · WaitAroundInput+0x8)",
      evidence=u"tcxdict `SmallActionAroundPosition 0x38` → around_input.target_y. m02.ll:46959 phi [56,%74]=idx7",
      behavior_change=False, found_by="new", kind=u"보강")
p.ev("/specs[138]/consts[2]", to=2, found_by="new", evidence=ORA + u"case70~72(epic live_list 0 = 미가시): Stop → s(0) · RunAway → s/2 (-99999→-49999) · Recall → -49999 = sdiv 0방향 절사 실측")
p.ev("/specs[138]/knobs[1]", to=2, found_by="new", evidence=ORA + u"case71/72 interaction_score=-99999 → score=-49999 (RunAway·Recall) · case70 Stop → 0")

# ───────── 139 SerpenPokeSubPlan::score ─────────
p.fix("/specs[139]/mem[21]/name", old=u"Around/AroundHide/LaneMinionPosition.target_id", new=NM21,
      evidence=u"tcxdict: SmallActionAround/AroundHide/LaneMinionPosition 0x8 필드명 `target`. m14.ll:18804 `gep %6, 8`",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[139]/mem[22]/name", old=u"AroundRegion x / AroundPositionBush y", new=u"AroundRegion.goal_x (idx4) / AroundPositionBush.target_y (idx8)",
      evidence=u"tcxdict: SmallActionAroundRegion 0x10 goal_x · SmallActionAroundPositionBush 0x10 target_y. m14.ll:18817 phi [16,%69]=idx4, [16,%78]=idx8",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[139]/mem[25]/name", old=u"AroundPosition x", new=u"AroundPosition.around_input.target_x (idx7 · WaitAroundInput+0x0)",
      evidence=u"tcxdict `SmallActionAroundPosition 0x30` → around_input.target_x. m14.ll:18817 phi [48,%77]=idx7",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[139]/mem[26]/name", old=u"AroundPosition y", new=u"AroundPosition.around_input.target_y (idx7 · WaitAroundInput+0x8)",
      evidence=u"tcxdict `SmallActionAroundPosition 0x38` → around_input.target_y. m14.ll:18818 phi [56,%77]=idx7",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[139]/knobs[1]/what", old=u"보너스 적용 거리 게이트", new=u"세르펜 지향 보너스 적용 거리 게이트(champ↔serpen distance_sq · serpen_poke.rs:518)",
      evidence=u"G20 R4: 143 의 동명 노브(39999999999)와 이름이 같아 값 충돌로 잡힘 — 다른 함수·다른 대상 거리라 이름을 특정해 분리. m14.ll:18753 `icmp ult i64 %67, 22500000001`",
      behavior_change=False, found_by="new", kind=u"오탐")
p.ev("/specs[139]/consts[6]", to=2, found_by="new", evidence=ORA + u"case73~75(serpen live_list 0): Stop/RunAway/Recall 전부 0 (interaction_score=-99999 여도 0) — L513 early-return 0 실측")

# ───────── 140 SmallActionAroundBush::new_with_target ─────────
p.fix("/specs[140]/consts[4]/meaning",
      old=u"aux from_iter: std Vec 초기 cap 4(원소 16B) — RawVecInner::try_allocate_in(4,false,8,16). 판정값 아님.",
      new=u"aux from_iter: std Vec 초기 cap 4(원소 16B) — RawVecInner::try_allocate_in(4,false,8,16) = m06.ll:44229 `i64 noundef 4`(별도 define `spec_from_iter_nested::from_iter<(usize,usize),Filter<…new_with_target::closure_env$0>>` m06.ll:44095~44454 · !dbg 루트 = spec_from_iter_nested.rs:30, game_ai 소스 줄 없음). 본체 IR 범위엔 이 4 가 없고 호출부 `collect()` 가 L1180 이라 src_line=1180. ★G12 후보 [1181] 은 본체 104968 `shl nuw nsw i64 %21, 4`(원소 stride 16 접힘)로 다른 상수 — 게이트가 aux define 을 안 보는 오탐. 판정값 아님.",
      evidence=u"m06.ll:44229 `call void @…RawVecInner15try_allocate_in…(ptr … %5, i64 noundef 4, i1 noundef zeroext false, i64 noundef 8, i64 noundef 16), !dbg !62836` · `dloc m06.ll 62836` → raw_vec/mod.rs:434 ← 177 ← vec/mod.rs:977 ← 524 ← spec_from_iter_nested.rs:30(루트, game_ai 줄 없음) · 본체 m08.ll:104968 `%22 = shl nuw nsw i64 %21, 4, !dbg`(L961<100<1042<1181 = slice 길이×16)",
      behavior_change=False, found_by="new", kind=u"오탐")
p.ev("/specs[140]/consts[0]", to=2, found_by="new", evidence=ORA + u"case15: 71셀 표의 (x,y)→map.bushes 값 히스토그램 = MapDef::moba 의 bushes≠0 셀 전량 히스토그램과 완전 일치(id 1~24, 셀 71개) · case12 24수풀×200무작위 표적 4800케이스 재구현(표 필터+min_by_key 첫 동률)과 0 mismatch")
for k in (1, 2):
    p.ev("/specs[140]/consts[%d]" % k, to=2, found_by="new", evidence=ORA + u"target_x/y = x*32000+16000 / y*32000+16000 이 4800+168 케이스 전부 일치")
for k in (0, 1):
    p.ev("/specs[140]/knobs[%d]" % k, to=2, found_by="new", evidence=ORA + u"case14 표에 없는 bush id(987654) → `Option::unwrap()` on None panic at around.rs:1182:94 · 제곱거리 최소 셀 선택 4968케이스 0 mismatch")
for k in range(8, 15):
    p.ev("/specs[140]/mem[%d]" % k, to=3, found_by="new", evidence=ORA + u"반환 구조체 Debug 실측 `SmallActionAroundBush { start_tick: 0, change_tick: 0, bush: <인자>, target_x, target_y, path_finder: None, out_line: <인자> }`(tick 0 · 24×… 케이스)")

# ───────── 141 SmallActionPlay::get_input ─────────
p.fix("/specs[141]/mem[11]/name", old=u"위 goal_y · Around/AroundHide/LaneMinionPosition.goal_x",
      new=u"RunAway/Positioning/AroundPosition.goal_y · AroundPositionBush.target_y · Around/AroundHide/LaneMinionPosition.goal_x",
      evidence=u"tcxdict: SmallActionRunAway 0x10 goal_y · SmallActionPositioning 0x10 goal_y · SmallActionAroundPosition 0x10 goal_y · SmallActionAroundPositionBush 0x10 target_y · SmallActionAround/AroundHide/LaneMinionPosition 0x10 goal_x. m11.ll:43287 phi [16,%179]×4(idx0·6·7·8 y), [16,%186](idx2·3·10 x)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[141]/mem[18]/name", old=u"@tag", new=u"tag",
      evidence=u"G20 R1: 179 가 같은 (Input,0x0) 을 `tag` 로 부른다 — 같은 명세의 sret 행(mem[22])도 `tag`. 표기 통일(Direct 인코딩 8B 판별자, tcxdict --enum game_core::Input)",
      behavior_change=False, found_by="new", kind=u"오탐")
p.fix("/specs[141]/mem[22]/value",
      old=u"-1 None (42866 champ 없음 · 43132 Move 보정 시 champ 없음)",
      new=u"-1 None (42866 champ 없음 · 43132 Move 보정 시 champ 없음 — None 은 +0 8B 만 정의, 통과 경로 memcpy 32B 는 지역 %9 의 미정의 페이로드도 복사: 오라클 case65 Attack→None 실측 bytes [-1, 0, <스택 포인터>, 6])",
      evidence=ORA + u"case65 `game=None bytes=[ffffffffffffffff, 0, 7ff64f2c72f0, 6]` · m11.ll:42866 `store i64 -1, ptr %0` · 43109 memcpy 32B",
      behavior_change=False, found_by="new", kind=u"보강")
for k in (1, 4, 5):
    p.ev("/specs[141]/consts[%d]" % k, to=2, found_by="new", evidence=ORA + u"case60~67: Stop→Some(Move{champ}) (v1·v2) · 적 우물(892000,0)·위험 경계(800000,0) → Some(Move{700000,0}) (v1·v2, dist²=1e10≥4000001) · Attack→None(tag -1) 통과 — 독립 재구현(pub 잎 safe_move_avoiding_enemy_well·enemy_well_escape_position·adjust_position·target_position 호출)과 7/7 MATCH")
for k in (22, 23, 24):
    p.ev("/specs[141]/mem[%d]" % k, to=3, found_by="new", evidence=ORA + u"sret 32B 실측 bytes: Move [0, x, y, 0] · None [-1, …미정의] (case60~67)")
p.ev("/specs[141]/knobs[2]", to=2, found_by="new", evidence=ORA + u"case62(v2)/63(v1) 우물 위험 시 둘 다 탈출 Move · v2 도달 판정은 이 맵에서 escape(700000,0)↔위험경계(800000,0) 거리 1e5 라 ≤2000 분기 미발화(관측 한정)")

# ───────── brief errors ─────────
p.brief_error(u"§4 G16 [136] 은 오탐이다 — `(u64,u64)` 튜플 인자는 Rust ABI ScalarPair 로 IR 2슬롯인데 `paramrole.ir_slots` 가 `&dyn/&[T]/&str/Box<dyn>` 만 2슬롯으로 센다. 튜플 `(T,U)`(스칼라 2개 ≤16B)도 2슬롯으로 세면 P1 이 저절로 맞는다(role 에 `%k` 를 강제하지 않아도 됨).")
p.brief_error(u"§4 G12 [140] consts[4] 는 오탐이다 — 상수 4 가 본체가 아니라 aux define(m06.ll:44095 from_iter 모노모프)에 있고 그 !dbg 루트가 std 줄(spec_from_iter_nested.rs:30)이라 game_ai 소스 줄이 없다. srclinecheck 가 aux define 을 안 보고 본체의 `shl …, 4`(stride 접힘)를 후보로 잡았다. aux 범위·「루트 줄 없음」 상수는 판정 보류로 처리해야 한다.")
p.brief_error(u"§4 G20 R1 [138][139][141] SmallActionPlay 0x10 은 구조적이다 — 열거형 변종 union 이라 같은 오프셋에 변종별 다른 필드가 있고, 각 함수가 읽는 변종 집합이 달라 이름이 달라진다. R1 은 base 가 tcx 열거형이면 `/`·`·` 로 나눈 변종별 성분으로 비교하거나 억제해야 한다(이번 패치로 tcx 필드명으로 통일했지만 집합 차이는 남는다).")
p.brief_error(u"§1 표의 [134] vis `in:game_ai::small_action::around` 는 「오라클 직접 진입이 막힌다」가 아니다 — `define hidden`(비-internal)이라 `extern \"Rust\" #[link_name]` 로 직접 링크·실행됐다(1600케이스). §1 문구 「pub 이 아니면 래퍼/복제본을 노려라」는 internal fastcc 에만 맞다.")
p.brief_error(u"지시문 ② 「Option<SmallActionPlay> 184B(태그 +0xb1 · variant 별 live)」 는 이 배치 8함수 어디에도 없는 타입이다(141 은 &mut SmallActionPlay, 140 은 sret SmallActionAroundBush 120B). Vec<SmallActionPlay> 원소 live 표는 tcx 레이아웃으로 만들어 산문에 실었다.")
p.brief_error(u"heapsurf.py 는 인라인된 `__rust_dealloc(ptr, 1120/70, …)` 을 못 본다(정규식이 grow_one|drop_glue|drop_in_place 만) — 141 콜리 11개에서 Option<PathFinder> Box 해제 24곳이 전부 그 형태라 `(없음)` 을 냈다. 도구 결함으로 보고.")
p.save()
