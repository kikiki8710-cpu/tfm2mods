# -*- coding: utf-8 -*-
import sys, io
sys.stdout.reconfigure(encoding="utf-8")
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch

p = mkpatch.Patch(round=23, batch="B")
O142 = "오라클 o142.exe(_verify23/B/oracle · define hidden 직접 링크 · 케이스당 프로세스 1개 · 20/20 IR 독해와 일치)"
O143 = "오라클 o143.exe(pub 직접 호출 · evaluate_action/interaction_score/economy 를 같은 시드 rng clone 으로 따로 재서 대조)"
O145 = "오라클 o145.exe(pub 직접 호출 · Entity::move_to 를 같은 인자로 직접 불러 예측)"
O146 = "오라클 o146.exe(pub 직접 호출 · rng clone 으로 gen_range::<u32>(0..len) 1회 예측 · 9/9 MATCH + 후보0 패닉 1)"
O147 = "오라클 o147.exe(pub 직접 호출 · 얕은 조기반환 8경로)"
O148 = "오라클 o148.exe(pub 직접 호출 · argv2 s/e 로 148·149 동시 · base 는 interaction_score 를 rng clone 으로 따로 잰 값 · 각 16케이스)"

# ───────── 142 path_needs_tower_escape ─────────
p.fix("/specs[142]/logic",
      old="L139: if is_unnecessary_enemy_tower_position(version, player, data, champ.x, champ.y) → false   (이미 그 위치면 회피 불필요)",
      new="L139: if is_unnecessary_enemy_tower_position(version, player, data, champ.x, champ.y) → true   (L140 `return true;` — 현 위치가 이미 불필요 적 타워 위치면 경로를 보지 않고 즉시 회피 필요)",
      evidence="m11.ll:52759~52760 `%40 = call … is_unnecessary_enemy_tower_position(i64 %0, ptr %1, ptr %2, i64 %37, i64 %39), !dbg !76061` / `br i1 %40, label %13, label %71, !dbg !76061`(=L139) · 52715 `%14 = phi i1 [ false, %4 ], [ false, %67 ], [ %113, %110 ], … [ true, %35 ]` — %35 가 그 call 블록이라 참이면 **true** 반환. "
               + O142 + " case3(hp40·champ 를 unnec=true 셀(272000,16000)에 두고 path[0]=unnec 아닌 곳) game=true(명세대로면 false) · case17/18 도 game=true. rmeta 줄길이: small_action.rs:140 = 17B = `    return true;`(`return false;` 면 18B · L1150 캘리브레이션과 같은 규약)",
      behavior_change=True, found_by="new")
p.fix("/specs[142]/one_line",
      old="현재 경로의 다음 웨이포인트가 '불필요한 적 타워 위치'면 true",
      new="현 위치가 이미 '불필요한 적 타워 위치'면 즉시 true, 아니면 현재 경로의 다음 웨이포인트가 그 위치면 true",
      evidence="위 logic 정정과 동일 근거(m11.ll:52715 phi `[ true, %35 ]` · o142 case3/17/18)",
      behavior_change=True, found_by="new", kind="보강")
p.fix("/specs[142]/consts[8]/meaning",
      old="aux m06 34852 — Chain 어댑터 a-side 소진 니치 태그(핵심 판정 아님)",
      new="aux m06 34852 — Chain 어댑터 a-side 소진 니치 태그(핵심 판정 아님). 본문에도 m11.ll:52840 `%74 = add i64 %7, -1, !dbg !76152`(L144 path_len−1 = index 상한)이 같은 리터럴이다",
      evidence="m11.ll:52840 `%74 = add i64 %7, -1, !dbg !76152` (!76152 루트 = small_action.rs:144) · o142 case16(index=7, path_len=2 → path[1] 사용 game=true)",
      found_by="new", kind="보강")
for i, ev in [(0, "case0 path_len=0 → false"), (4, "case9 hp=45 → 게이트 없이 true / case10 hp=46 → false(피격자 None)"),
              (5, "case11 hp=65+적타워 피격 → true / case12 hp=66 → false"), (6, "case13 path[0] 이 (move_speed·10)² 안 → path[1] 검사 true / case14 → false / case15 path_len 1 → next=0")]:
    p.ev("/specs[142]/consts[%d]" % i, evidence="오라클 실행 확인: " + O142 + " " + ev, to=2, found_by="new")
for i, ev in [(0, "case9(45)→true·case10(46)→false"), (1, "case11(65)→true·case12(66)→false"), (2, "case13/14/15 도달반경 10×move_speed(=10000) 안팎")]:
    p.ev("/specs[142]/knobs[%d]" % i, evidence="오라클 실행 확인: " + O142 + " " + ev, to=2, found_by="new")
for i, ev in [(0, "case0 path_len=0"), (1, "case16 index=7·path_len=2 → path[1]"), (2, "path[i] 를 new_target_no_check 로 만든 Box<[(u64,u64);70]> 에 써서 검사"),
              (9, "hp 40/45/46/60/65/66/80"), (10, "stat_cached.hp=100 으로 hp_ratio=hp"), (13, "move_speed=1000 → 반경 10000"),
              (14, "case5 last_attacked_from None → false"), (15, "case6 적타워 id → true / case8 적챔피언 id → false"), (18, "case7 아군타워 → false / case6 적타워 → true"),
              (20, "case17 적 타워 nearest_enemy 태그=1"), (21, "case17 nearest_enemy.1=champ.id → 게이트 통과")]:
    p.ev("/specs[142]/mem[%d]" % i, evidence="오라클 실행 확인: " + O142 + " " + ev, to=3, found_by="new")

# ───────── 143 LineDefenseSubPlan::score ─────────
p.fix("/specs[143]/knobs[1]/where", old="line_defense.rs:986", new="line_defense.rs:986 (m14.ll:30073 phi `[ 100, %129 ]` · %129 종결자 30155 `br label %89, !dbg !43941`=986)",
      evidence="G19 오탐 — 값 100 은 phi 인입 상수라 그 줄에 `!dbg` 가 없다(G12 교정 ②와 같은 형태). m14.ll:30073 `%90 = phi i64 [ … [ 100, %129 ], [ 5, %130 ] …` · 30153~30155 `129: … br label %89, !dbg !43941`(dloc → line 986 in score)",
      found_by="reused", kind="오탐")
p.fix("/specs[143]/knobs[2]/where", old="line_defense.rs:988", new="line_defense.rs:988 (m14.ll:30073 phi `[ 5, %130 ]` · %130 종결자 30158 `br label %89, !dbg !43942`=988)",
      evidence="G19 오탐 — 값 5 는 phi 인입 상수. m14.ll:30073 `[ 5, %130 ]` · 30157~30158 `130: … br label %89, !dbg !43942`(dloc → line 988 in score). 창 관측 [1168] 은 30137 gep 1168(attack_effect 오프셋) 잡음",
      found_by="reused", kind="오탐")
p.fix("/specs[143]/mem[26]/name", old="priority@tag = 2 (PriorityProfile::Lane, 니치 태그 2)", new="priority@tag",
      evidence="G20 R1 통일 — [165] evaluate_action 명세는 같은 (ActionContext, offset) 을 `anchor@tag`/`anchor@Lane.line` 로 부른다. 값·variant 는 `value`/`note` 칸에 있다(m14.ll:29923 `store i64 2, ptr %9`)",
      found_by="new", kind="보강")
p.fix("/specs[143]/mem[26]/note", old="m14.ll:29923 — 스택 로컬. 힙/인자 write 아님", new="m14.ll:29923 `store i64 2, ptr %9` = PriorityProfile::Lane(니치 태그 2, tcxdict --enum PriorityProfile) — 스택 로컬. 힙/인자 write 아님",
      evidence="m14.ll:29923 `store i64 2, ptr %9, align 8, !dbg !43797`", found_by="new", kind="보강")
p.fix("/specs[143]/mem[27]/name", old="anchor@tag = 1 (Anchor::Lane)", new="anchor@tag",
      evidence="G20 R1 [143]↔[165] 이름 통일(같은 (base,offset)=ActionContext 0x10). 값 1 은 `value` 칸", found_by="new", kind="보강")
p.fix("/specs[143]/mem[27]/note", old="m14.ll:29920", new="m14.ll:29920 `store i8 1, ptr %12` = Anchor::Lane 태그",
      evidence="m14.ll:29920 `store i8 1, ptr %12, align 8, !dbg !43797`", found_by="new", kind="보강")
p.fix("/specs[143]/mem[28]/name", old="anchor@Lane.line = self.line", new="anchor@Lane.line",
      evidence="G20 R1 [143]↔[165] 이름 통일(ActionContext 0x11). 값 self+0x1 은 `value` 칸", found_by="new", kind="보강")
p.fix("/specs[143]/mem[28]/note", old="m14.ll:29922", new="m14.ll:29922 `store i8 %11, ptr %13` (%11 = self+0x1 line)",
      evidence="m14.ll:29917 `%11 = load i8, ptr %10`(self+1) · 29922 `store i8 %11, ptr %13`(ctx+17)", found_by="new", kind="보강")
DEAD143 = ("★도달 불가: evaluate_action(Lane 앵커)은 Around 계열(Play idx 2·3·4·7·8·9·10) 이면 항상 Some 을 돌려주므로(specs[165] L77~89 · version 분기 없음) "
           "이 arm 은 L941 에서 이미 반환된 뒤라 실행되지 않는다. " + O143 + " case0(Around 아군 챔피언 300k → game=0=evaluate_action, live 면 5) · case1(넥서스 → 0, live 면 100) · case2(아군 타워 → 0, live 면 50)")
p.fix("/specs[143]/logic",
      old="    SmallAction::Around{target_id} /*Play idx 2·3·10*/ => {                     // L978",
      new="    SmallAction::Around{target_id} /*Play idx 2·3·10*/ => {                     // L978  ★도달 불가(evaluate_action 이 Around 계열엔 항상 Some → L941 반환 · o143 case0~2)",
      evidence=DEAD143, found_by="new", kind="보강")
p.fix("/specs[143]/one_line",
      old="행동별 보너스(Around 아군 원거리 50/100/5 · Attack/Skill/Skill2 는 calculate_action_score, 대상 없으면 -99999)",
      new="행동별 보너스(Attack/Skill/Skill2 는 calculate_action_score, 대상 없으면 -99999 · Around 아군 원거리 50/100/5 arm 은 evaluate_action 이 Around 계열을 먼저 Some 으로 가로채 사장)",
      evidence=DEAD143, found_by="new", kind="보강")
for i in range(4):
    p.fix("/specs[143]/knobs[%d]/effect" % i, old=mkpatch.locate("/specs[143]/knobs[%d]/effect" % i),
          new=mkpatch.locate("/specs[143]/knobs[%d]/effect" % i) + " ⚠사장 — 이 arm 은 도달 불가(evaluate_action 이 Around 계열을 먼저 Some 으로 반환 · o143 case0~2 실행 확인). 값을 바꿔도 동작 변화 없음",
          evidence=DEAD143, found_by="new", kind="보강")
p.ev("/specs[143]/consts[6]", evidence="오라클 실행 확인: " + O143 + " case4 Attack 대상 999999 → game = base+econ−99999 / case8 Skill2 대상 없음 → −99999(level 게이트 이전 반환)", to=2, found_by="new")
p.ev("/specs[143]/knobs[4]", evidence="오라클 실행 확인: " + O143 + " case4/case8 −99999", to=2, found_by="new")
p.ev("/specs[143]/consts[0]", evidence="오라클 실행 확인: " + O143 + " case0~3 evaluate_action(Lane ctx {priority: Lane, anchor: Lane{Top}}) 값이 score 반환값과 일치(=Some 경로)", to=2, frm=3, found_by="new")
p.ev("/specs[143]/consts[1]", evidence="오라클 실행 확인: " + O143 + " Anchor::Lane 컨텍스트로 evaluate_action Some 경로 실행 일치", to=2, found_by="new")
for i in [0, 1, 2, 3, 4, 5, 26, 27, 28]:
    p.ev("/specs[143]/mem[%d]" % i, evidence="오라클 실행 확인: " + O143 + " (LineDefenseSubPlan{Defensive,Top,Normal} 리터럴 · Lane 앵커 ctx 를 직접 만들어 같은 값 확인)", to=3, found_by="new")

# ───────── 144 has_current_explicit_minion_action ─────────
p.fix("/specs[144]/sig/abi",
      old="internal fastcc (m11.ll:45591). 인자 승격 흔적:",
      new="internal fastcc (m11.ll:45591). ★exe 대응: 0xe266e0 는 인자 4 = (data, player, line, target) — IR %0 version 이 **DeadArgElim 으로 제거**(argscan 레지스터 4·스택 0 · 디컴 lane_minion.md:725 `bool FUN_140e266e0(longlong *param_1,longlong param_2,char param_3,longlong *param_4)` 본문에 version 사용 0 · 호출부 around.md:489 `FUN_140e266e0(param_5,param_4,*(…+0x11a),plVar16)`). 대응 = rcx←%1 data · rdx←%2 player · r8←%3 line · r9←%4 target. 인자 승격 흔적:",
      evidence="python -X utf8 argscan.py 0xe266e0 → `스택 인자 0개 + 레지스터 4 = exe 인자 4` · MIG/decomp/0.5.8/small_action/lane_minion.md:725~896 (4인자 · is_enemy_well_danger 인라인·FUN_140d97300(context,cache,player,x,y) 호출에 version 없음) · IR m11.ll:45591 5인자(i64 %0 version)",
      found_by="new", kind="보강", force=True)   # v3 sig 에는 abi 가 없고 v2 signature.abi 에만 있다(mkpatch 는 v3 를 본다)
p.fix("/specs[144]/sig/params[0]/role",
      old="본문 분기 없음. is_enemy_well_danger/is_unnecessary_enemy_tower_position 첫 인자로 전달.",
      new="본문 분기 없음. is_enemy_well_danger/is_unnecessary_enemy_tower_position 첫 인자로 전달(exe 0xe266e0 에서는 DeadArgElim 으로 이 인자가 없다 — sig.abi).",
      evidence="argscan 0xe266e0 = 4 · 디컴 4인자", found_by="new", kind="보강")

# ───────── 145 evaluation_position ─────────
for i, ev in [(10, "case0 RunAway goal(+8/+16) 을 (215000,763000) 으로 → move_to 1초 투영 (48600,887800) 일치"), (11, "case2 Around goal(+0x10/+0x18) 동일 목표 → 동일 결과"),
              (12, "case2 Around goal_y +0x18"), (18, "case1 Stop → (champ.x,champ.y) 그대로"), (19, "case1"), (21, "move_speed=700 → 이동량 42000 = 700×60"), (8, "tps=60 곱 확인(이동량 42000)"),
              (22, "case9 champ None → None / Some 경로 태그 1"), (23, "Some 경로 x"), (24, "Some 경로 y")]:
    p.ev("/specs[145]/mem[%d]" % i, evidence="오라클 실행 확인: " + O145 + " " + ev, to=3, found_by="new")
p.ev("/specs[145]/knobs[0]", evidence="오라클 실행 확인: " + O145 + " move_speed 700·tps 60 → 목표 방향 정확히 42000 이동(case0/2) · 가까운 목표(10)는 10 만 이동(case5)", to=2, found_by="new")
p.fix("/specs[145]/logic",
      old="AroundRegion(4)/AroundRunAway(5)                                   → None",
      new="AroundRegion(4)/AroundRunAway(5)                                   → None   (o145 case6 AroundRunAway → game=None 실행 확인)",
      evidence=O145 + " case6 tag=8(AroundRunAway) game=None · case9 champ None → None", found_by="new", kind="보강")

# ───────── 146 new_with_out_line ─────────
p.fix("/specs[146]/sig/params[3]/role",
      old="★본문에서 전혀 읽지 않음(dead 인자). tcx 시그니처 3번째 = &PlayerState",
      new="★본문에서 전혀 읽지 않음(dead 인자). tcx 시그니처 3번째 = &PlayerState. 소스 이름은 `_player` — rmeta 줄길이 산술: around.rs:1154 = 146B 인데 `  pub fn new_with_out_line(rnd: &mut StdRng, data: &OperationData, player: &PlayerState, bush: usize, out_line: AroundBushOutlineType) -> Self {` 는 145B 라 1자가 남고, 같은 규약으로 L1150 `new` 시그니처 98B·L1151 `    Self::new_with_out_line(rnd, data, player, bush, AroundBushOutlineType::None)` 82B 가 정확히 맞는다",
      evidence="python -X utf8 rmeta_srcmap.py game_ai around.rs 1148 1180 → L1150=98 L1151=82 L1154=146 L1157=31 L1158=52 L1161=65 (개행 1B 포함 규약 · L1150/1151/1157/1158/1161 전부 재구성 문면과 일치)",
      found_by="new", kind="보강")
p.fix("/specs[146]/consts[1]/meaning", old="candidates.len() == 0 → 경고 출력 분기 (105174)",
      new="candidates.is_empty() → 경고 출력 분기 (105174 `icmp eq i64 %20, 0`; L1157 = 31B = `    if candidates.is_empty() {`)",
      evidence="m08.ll:105174 `%22 = icmp eq i64 %20, 0` · rmeta L1157=31B(`len() == 0` 판은 32B) · " + O146 + " case100(bush 999999) stdout `no candidates for bush: 999999` 후 exit 101 패닉 around.rs:1161:56",
      found_by="new", kind="보강")
p.fix("/specs[146]/logic", old="print!(\"no candidates for bush: {}\\n\", bush)      # stdout, @anon…99      (L1158)",
      new="println!(\"no candidates for bush: {}\", bush)      # stdout, @anon…99 (조각 \"no candidates for bush: \"+\"\\n\") · L1158=52B 일치      (L1158)",
      evidence="rmeta L1158=52B = `      println!(\"no candidates for bush: {}\", bush);` · o146 case100 출력 확인", found_by="new", kind="보강")
p.fix("/specs[146]/logic", old="chosen = candidates.choose(rnd).unwrap()      # 비면 None → unwrap_failed 패닉     (L1161)",
      new="(target_x, target_y) = *candidates.choose(rnd).unwrap()      # 비면 None → unwrap_failed 패닉(Location 1161:56 = `unwrap` 토큰 · L1161=65B 일치)     (L1161)",
      evidence="@anon…100 = {file(34B=game-ai\\src\\small_action\\around.rs), line 0x489=1161, col 0x38=56} · rmeta L1161=65B = `    let (target_x, target_y) = *candidates.choose(rnd).unwrap();`(`.unwrap` 이 col 55~56) · o146 case100 패닉 메시지 `around.rs:1161:56`",
      found_by="new", kind="보강")
for i, ev in [(0, "IR 상수 @anon…94 를 디코드한 71셀 표로 후보를 필터한 예측 = 실행값(9/9)"), (1, "case100 후보 0 → 메시지+패닉"), (2, "target = cell*32000+16000 9/9"), (3, "동일"), (4, "sret+0x6d = 2 (9/9)")]:
    p.ev("/specs[146]/consts[%d]" % i, evidence="오라클 실행 확인: " + O146 + " " + ev, to=2, found_by="new")
p.ev("/specs[146]/knobs[0]", evidence="오라클 실행 확인: " + O146 + " 표 71셀 × map.bushes 필터가 게임 산출과 9/9 일치(bush 1~6 · 시드 3종)", to=2, found_by="new")
for i, ev in [(4, "map.bushes[y][x]==bush 필터 예측 일치"), (5, "len 0 분기(case100)"), (6, "start_tick=777=game.tick()"), (7, "change_tick=777"), (8, "bush"), (9, "target_x"), (10, "target_y"), (11, "+0x6d=2"), (12, "out_line 0/1/2 그대로"),
              (13, "rng clone 에 gen_range::<u32>(0..len) 1회 적용 후 next_u64 가 실제 rnd 와 동일(9/9) — u32 경로·1회 소비 확정"), (14, "case100 stdout 확인")]:
    p.ev("/specs[146]/mem[%d]" % i, evidence="오라클 실행 확인: " + O146 + " " + ev, to=3, found_by="new")

# ───────── 147 line_action_economy_adjustment ─────────
for i, ev in [(0, "case7 level=2 Skill2 → 0(effect None 경로)"), (1, "case8 level=4 Ult → 0"), (2, "case7/8 None → 0")]:
    p.ev("/specs[147]/consts[%d]" % i, evidence="오라클 실행 확인: " + O147 + " " + ev, to=2, found_by="new")
p.ev("/specs[147]/knobs[3]", evidence="오라클 실행 확인: " + O147 + " case7(level 2 Skill2)·case8(level 4 Ult) → 0", to=2, found_by="new")
for i, ev in [(7, "case0 RunAway(태그3) → 0 · Attack 15/Skill 16/Skill2 17/Ult 18 진입"), (16, "case3 아군 대상 → 0"), (18, "case4 적 타워(ty 2) → 0 · case3/5 챔피언 13 진행"), (20, "case5 CastingTarget::Ally 효과 → check 거부 → 0")]:
    p.ev("/specs[147]/mem[%d]" % i, evidence="오라클 실행 확인: " + O147 + " " + ev, to=3, found_by="new")

# ───────── 148 / 149 ─────────
for idx, nm, L in [(148, "serpen", "L851"), (149, "epic", "L856")]:
    DEAD = ("★champions(team) 은 player_champion[team][0..5] 의 Some 을 전부 모을 뿐(_gcbc g15.ll:109887~ · 자기 자신 제외 없음) ⟹ 자기 자신이 dist 0 ≤ 200000²·hp ≥ hp 로 any 를 항상 true 로 만들어 -10 은 **도달 불가**. "
            + O148 + " champions(0) 첫 원소 = champ(id 18) 실측 · case1(아군 전원 900k·hp100) / case3(아군 200001) / case4(근접 hp 499) / case6 Recall / case15 AroundRunAway 전부 delta=0")
    p.fix("/specs[%d]/logic" % idx,
          old="let champions = data.cache.champions(player.info.team, ctx.pool);                       // %s (bumpalo Vec<&Entity>, 아군 팀 챔피언)" % L,
          new="let champions = data.cache.champions(player.info.team, ctx.pool);                       // %s (bumpalo Vec<&Entity>, 아군 팀 챔피언 — ★자기 자신 포함(g15.ll:109887) ⟹ 아래 any 는 항상 true, -10 도달 불가 · o148 실행 확인)" % L,
          evidence=DEAD, found_by="new", kind="보강")
    p.fix("/specs[%d]/one_line" % idx, old="RunAway: 위험옵션 시 근접 아군 없으면 -10", new="RunAway: 위험옵션 시 근접 아군 없으면 -10 — champions() 가 자기 자신을 포함해 사장",
          evidence=DEAD, found_by="new", kind="보강")
    for k in (0, 1):
        cur = mkpatch.locate("/specs[%d]/knobs[%d]/effect" % (idx, k))
        p.fix("/specs[%d]/knobs[%d]/effect" % (idx, k), old=cur, new=cur + " ⚠사장 — champions() 가 자기 자신을 포함해 근접 아군 술어가 항상 참(o148 실행 확인). 값을 바꿔도 동작 변화 없음",
              evidence=DEAD, found_by="new", kind="보강")
    p.fix("/specs[%d]/consts[10]/value" % idx, old=12, new=-12,
          evidence=("m02.ll:64985 `%170 = add nsw i8 %30, -12, !dbg !70830` · 64986 `%171 = icmp ult i8 %170, 3`" if idx == 148 else "m15.ll:50405 `%138 = add nsw i8 %30, -12, !dbg !57486` · 50406 `%139 = icmp ult i8 %138, 3`") + " — 본문 리터럴은 −12 다(SPEC_GUIDE §3 `value` 는 본문 숫자 그대로). 뜻은 그대로 idx−12 <u 3",
          behavior_change=False, found_by="new", kind="실오류")
    p.fix("/specs[%d]/consts[10]/meaning" % idx, old="Play idx-12 <u 3 ⇔ idx 12·13·14 (Attack·Skill·Skill2)", new="Play idx+(−12) <u 3 ⇔ idx 12·13·14 (Attack·Skill·Skill2) — 리터럴은 `add nsw i8 %30, -12`",
          evidence="위와 동일", found_by="new", kind="보강")
    which = "s" if idx == 148 else "e"
    p.ev("/specs[%d]/consts[2]" % idx, frm=3, evidence="오라클 실행 확인: " + O148 + " %s case7(champ 를 camp_pos(%s,true) 위에) → 0 / case10(다른 오브젝트 캠프 위) → -99999 — 캠프 선택이 %s 인 것 확정" % (which, "Serpen" if idx == 148 else "Morgard", "Serpen(672000,672000)" if idx == 148 else "Morgard(288000,288000)"), to=2, found_by="new")
    p.ev("/specs[%d]/consts[3]" % idx, evidence="오라클 실행 확인: " + O148 + " %s case8 dist=attack_dist−1(=140999, range100000+30000+stat_range0+(3−1)×500+radius10000) → 0 / case9 dist=attack_dist → -99999 (엄격 <)" % which, to=2, found_by="new")
    p.ev("/specs[%d]/consts[5]" % idx, evidence="오라클 실행 확인: " + O148 + " %s case9 Trace 사거리 밖 −99999 · case11 Attack 대상 없음 −99999" % which, to=2, found_by="new")
    p.ev("/specs[%d]/consts[9]" % idx, evidence="오라클 실행 확인: " + O148 + " %s case0 risk_epic_damage=0 → game=base(delta 0) · case12 Stop/case13 Around → 0" % which, to=2, found_by="new")
    p.ev("/specs[%d]/knobs[2]" % idx, evidence="오라클 실행 확인: " + O148 + " %s case8/9 attack_dist 경계에서 0 ↔ −99999 전환" % which, to=2, found_by="new")
    for m, ev in [(5, "camp_pos 호출 결과가 case7/10 을 갈랐다"), (10, "case0(0) vs case1(1) — 0 이면 champions() 호출 없이 base"), (16, "attack_effect 주입 후 Trace 진입(None 이면 unwrap 패닉 경로)"),
                  (18, "range=100000"), (19, "growth_range=500 · level 3 → +1000"), (20, "level=3"), (21, "stat_buff_cached.range=0"), (22, "radius_mult=0 경로"), (23, "radius=10000 → attack_dist=141000 경계 일치"),
                  (13, "champ.x 캠프 기준 이동"), (14, "champ.y")]:
        p.ev("/specs[%d]/mem[%d]" % (idx, m), evidence="오라클 실행 확인: " + O148 + " " + which + " " + ev, to=3, frm=(3 if m == 10 else 4), found_by="new")

p.brief_error("§4 [143] G19 knobs[1]/[2] 2건은 phi 인입 상수(!dbg 없음) — G12 교정 ②와 같은 형태인데 G19(knobs.value) 검사기가 그 귀속을 안 한다. 2건 전부 오탐이었고, knobs[0] 은 where 에 IR 줄이 있어 통과한 것을 보면 검사 창이 where 문면에 의존한다(도구 결함).")
p.brief_error("§4 [143] G20 R1 2건은 「구조적」이 아니라 이름 문면(`anchor@tag = 1 (…)` 대 `anchor@tag`)의 분열이라 mem.name 치환으로 닫힌다 — 도시에 지시(구조적이면 보고만)와 달리 정정 대상이었다.")
p.brief_error("초점 ②의 Option<Input>/Option<SmallActionPlay>/Vec<SmallActionPlay>/&mut self PathFinder 항목은 이 배치 8함수 어디에도 해당 타입의 sret·&mut self 가 없다(142 는 &PathFinder 읽기, 146 은 path_finder=None 쓰기뿐). 배치별 초점을 함수 목록으로 걸러 넣어야 한다.")
p.brief_error("초점 ⑥의 144 「exe ≤4 vs IR 5」는 DeadArgElim(version) 이고 ArgumentPromotion 이 아니다. 169/170 「exe 5 vs IR 4」는 반대로 &PlayerState → (team i64, position u32) ArgumentPromotion — 둘을 한 문장에 묶으면 처리 규칙(병합 vs 삭제)이 반대다.")
p.save()
