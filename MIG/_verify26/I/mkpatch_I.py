# -*- coding: utf-8 -*-
"""26차 배치 I patch.json 생성 (mkpatch 참조구현 사용). specs 223~227."""
import sys, io, json
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import mkpatch

p = mkpatch.Patch(round=26, batch="I")
O223 = u"오라클 실행 확인(26차 I o223.exe · _verify26/I/oracle/o223_cases.log · pub 직접 호출 · mode=score/expect 프로세스 분리 25/27 값 일치, 나머지 2 는 Ult 가지에서 콜리 interaction_score 가 action_score.rs:932 unwrap 패닉)"
O224 = u"오라클 실행 확인(26차 I o224.exe · _verify26/I/oracle/o224_cases.log · 직접 호출자 v21_runaway_defensive_cc_bonus(define hidden · goal=RunAway 이면 그대로 tail-call) 를 link_name 으로 진입 · 독립 재구현(expect) 과 50/51 일치 · 나머지 1 은 래퍼 goal 게이트)"
O227 = u"오라클 실행 확인(26차 I o227.exe · _verify26/I/oracle/o227_cases.log · pub 직접 호출 · 20/20 예측 일치 · 경계 R/R+1 · 조준 예외 · 비활성 tick)"

# ───────────────────────── 227 v47_tower_focus_position_dangerous ─────────────────────────
p.fix("/specs[227]/logic",
      old=u"L634:   range = attack.range(t, 15000) + t.radius() + champ.radius()      // 인자 형태는 추정(unknown 참조)",
      new=u"L634:   range = attack.range(t) + 15000 + t.radius() + champ.radius()      // Effect::range 는 2인자(tcx effect.rs:25 `fn range(&self, caster:&Entity)->u64` · 줄길이 47=시그니처 정확히 일치) · 15000 은 L634 리터럴(오라클 W3~W7b: R 경계에서 true / R+1 false)",
      evidence=u"tcx game_core::Effect::range sig = fn(&Effect, &Entity) -> u64 (effect.rs:25:3~46 · rmeta 줄길이 47 = `  pub fn range(&self, caster: &Entity) -> u64 {`) — extra 인자 없음. m07.ll:51597 `%137 = add i64 %101, 15000 ;L26<634` 는 Reassociate 슬롯 재사용(24차 B #181 consts[7] 와 동일 현상). 224 의 같은 인라인(m05.ll:54326~54328 L26<568) 에는 15000 이 없다. 오라클 o227 W3/W4/W5/W5b/W6/W6b/W7/W7b: R = attack.range+15000+t.stat_buff_cached.range+growth*(level-1)+t.radius()+champ.radius() 에서 x=t.x+R → true, R+1 → false",
      behavior_change=False, found_by="new")
p.fix("/specs[227]/logic",
      old=u"// Effect::range(effect.rs:26) 인라인 = attack.range + 15000 + t.stat_buff_cached.range + attack.growth_range*(t.level-1)",
      new=u"// Effect::range(effect.rs:26) 인라인 = attack.range + t.stat_buff_cached.range + attack.growth_range*(t.level-1) — 15000 은 L634 의 별도 항(m07.ll:51597 add 의 !dbg 가 L26 인 것은 재결합 아티팩트)",
      evidence=u"위와 동일(tcx 2인자 시그니처 + 224 같은 인라인 대조 + 오라클 경계)", behavior_change=False, found_by="new")
p.fix("/specs[227]/logic",
      old=u"L641:     if let Some((id, _)) = info.nearest_enemy {",
      new=u"L641:     if let Some((_, id)) = info.nearest_enemy {                   // id = 튜플 .1(+0x98) · .0(+0x90) 은 안 읽음",
      evidence=u"m07.ll:51653~51655 `%175 = gep %86, 152`(=0x98) `%176 = load` ;; id = %176 · tcxdict Entity 0x98 = ty@Tower.info.nearest_enemy@Some.0.1 (0x90 = Some.0.0 은 미참조). 오라클 o227 W9: id 를 .0(0x90)에 쓰면 false, .1(0x98)에 쓰면 true(pos=far ne=me)",
      behavior_change=True, found_by="new")
p.fix("/specs[227]/logic",
      old=u"L657:     if let Some((id, _)) = info.nearest_enemy {",
      new=u"L657:     if let Some((_, id)) = info.nearest_enemy {                   // id = 튜플 .1(+0x98)",
      evidence=u"m07.ll:51701~51703 `%202 = gep %86, 152` `%203 = load` ;; id · 위와 동일 · 오라클 W9/W12/W13",
      behavior_change=True, found_by="new")
p.fix("/specs[227]/logic",
      old=u"L627: for t in cache.iter_towers(1 - team) {            // 적 타워(넥서스 포함 여부는 game_core 미독)",
      new=u"L627: for t in cache.iter_towers(1 - team) {            // 적 타워 8 + 넥서스 1(ty 3) — 오라클: iter_towers(1) 9개(ty 태그 [2×8, 3]) · 넥서스는 L628 에서 continue",
      evidence=u"o227.exe 출력 `towers_enemy 9 ty_tags=[2, 2, 2, 2, 2, 2, 2, 2, 3]`(TEMPLATE mkgame · start_game 만 · 타워 16/twin 2/2 정상 세계)",
      behavior_change=False, found_by="new")
p.fix("/specs[227]/consts[3]/meaning",
      old=u"사거리 여유 가산(셀 32000 의 약 0.47). add 명령의 DILocation 이 L26<634(effect.rs:26 Effect::range 인라인) — 같은 Effect::range 인라인이 line_defense.rs:59(unsafe_v19_non_champion_walkup)에선 15000 대신 caster_radius 를 같은 자리에 더하므로 `Effect::range(&self, caster, extra)` 의 extra 인자로 L634 에서 15000 을 넘긴 것으로 추정(add 는 nsw 없는 재결합 가능 연산)",
      new=u"사거리 여유 가산(셀 32000 의 약 0.47) — L634 의 game_ai 측 리터럴 `attack.range(t) + 15000 + …`. m07.ll:51597 `%137 = add i64 %101, 15000` 의 !dbg 가 L26<634 인 것은 LLVM Reassociate 가 Effect::range 인라인의 add 슬롯을 재사용한 아티팩트(24차 B #181 consts[7] 과 동일). Effect::range 는 tcx 정본 2인자 `fn(&Effect, &Entity)->u64`(effect.rs:25 · 줄길이 47 = 시그니처와 정확히 일치)라 extra 인자는 없다. 오라클 실행 확인: o227 W3/W4(R/R+1)·W5/W5b(+buff range)·W6/W6b(+growth·(level-1))·W7/W7b(+radius_mult) 8케이스 경계 갈림 = R 에 15000 포함",
      evidence=u"tcx game_core::Effect::range = fn(&'_ Effect, &'_ Entity) -> u64 (game_core.json i=2912 · sp effect.rs:25:3~46) · rmeta_srcmap effect.rs L25 bytes 48 → 47자 = `  pub fn range(&self, caster: &Entity) -> u64 {` · m05.ll:54326~54328(224 의 같은 인라인 L26<568)에 15000 없음 · 오라클 o227_cases.log W3~W7b",
      behavior_change=False, found_by="new")
p.fix("/specs[227]/knobs[0]/where",
      old=u"game-core effect.rs:26 Effect::range (인라인 · 본 함수 L634)",
      new=u"tower_discipline.rs:634 · m07.ll:51597 `%137 = add i64 %101, 15000`",
      evidence=u"15000 은 game_ai tower_discipline.rs:634 리터럴(Effect::range 는 2인자 · 위 consts[3] 근거)", behavior_change=False, found_by="new")
p.fix("/specs[227]/knobs[0]/effect",
      old=u"올리면 더 먼 위치까지 위험으로 본다(타워 접근 보수화). game_core 상수라 game_ai 재현 시 그대로 복제 필요",
      new=u"올리면 더 먼 위치까지 위험으로 본다(타워 접근 보수화). game_ai 측 리터럴(L634 · Effect::range 2인자 밖) — 재현 시 이 함수 안의 상수로 둔다",
      evidence=u"위와 동일", behavior_change=False, found_by="new")
p.fix("/specs[227]/mem[24]/name",
      old=u"ty@Tower.info.nearest_enemy@Some.0",
      new=u"ty@Tower.info.nearest_enemy@Some.0.1 (id)",
      evidence=u"tcxdict Entity 0x98 = ty@Tower.info.nearest_enemy@Some.0.1 · Tower.nearest_enemy: Option<(usize,usize)>(tower.rs:61) · IR gep 152 · 다른 명세(#22/#116/#142/#150/#196 등)와 표기 통일(G20 R1)",
      behavior_change=False, found_by="new")
p.fix("/specs[227]/mem[24]/note",
      old=u"조준 대상 entity id (L642·L658 == champ.id / L607 get_entity_by_id)",
      new=u"조준 대상 entity id = 튜플 .1 (L642·L658 == champ.id / L607 get_entity_by_id) · .0(+0x90)은 이 함수가 안 읽음 · 오라클 o227 W9: id 를 +0x90 에 쓰면 false(L658 불발), +0x98 에 쓰면 true",
      evidence=u"m07.ll:51653/51701 gep 152 · 오라클 o227_cases.log W9~W13", behavior_change=False, found_by="new")

# ───────────────────────── 224 v21_defensive_cc_score ─────────────────────────
p.fix("/specs[224]/consts[21]/meaning", kind=u"오탐",
      old=u"direct_threat(dist2 <= catch_range²) 이면 +22",
      new=u"direct_threat(dist2 <= catch_range²) 이면 +22 (★G12 오탐: m05.ll:54482 `%228 = add nuw nsw i64 %225, 22 ;L602` 는 LLVM Reassociate 가 L602 `bonus += 16/35` 의 add 와 L609 `bonus += 22` 의 add 를 뒤바꿔 실은 것 — 바로 다음 54483 `%229 = add nuw nsw i64 %228, %227 ;L609` 가 L602 의 select(16/35 · 54480)를 더한다. 반대 경로 %231(not direct)에는 22 가 없고(54496~54498) rmeta 줄길이 L608=20자 `  if direct_threat {` · L609=16자 `    bonus += 22;` 이므로 소스 줄 609 가 맞다. 오라클 o224 B1/T1(direct → +22)·T2/T4(not direct → 22 없음))",
      evidence=u"m05.ll:54479~54483 원문: `%226 = icmp ugt i64 %148, %147, !dbg(602)` `%227 = select i1 %226, i64 16, i64 35, !dbg(602)` `%228 = add nuw nsw i64 %225, 22, !dbg(602)` `%229 = add nuw nsw i64 %228, %227, !dbg(609)` — 리터럴 22 가 실린 명령이 L602 dbg 를 받았지만 %227(16/35) 을 더하는 명령이 L609 dbg 를 받았다 = 두 add 의 dbg 가 교차. 소스 구조는 rmeta_srcmap battle_common.rs L601 53자(let mut bonus = (cc_time as i64 / 3).clamp(10, 45);) · L602 25자 · L603 16자(bonus += 16;) · L605 16자(bonus += 35;) · L608 20자(if direct_threat {) · L609 16자(bonus += 22;) 로 확정. 오라클 o224: direct 케이스(B1 99 = 30+35+22+12) vs not-direct(T2 107 = 30+35+24+6+12) 차이가 정확히 22",
      behavior_change=False, found_by="new")

# ───────────────────────── 225 AgentVerHamster::item_v26 ─────────────────────────
# G18: logic 의 +0x30 은 이 함수 IR 에 없는 오프셋(호출자에서 승격됨)
p.fix("/specs[225]/logic",
      old=u"  item_list = &context.item_list                                   // +0x30 (IR 는 승격된 %2)",
      new=u"  item_list = &context.item_list                                   // IR 는 승격된 %2(&Vec<Box<dyn ItemInfo>>) — GameContext 필드 오프셋은 호출자(buy_item/upgrade_item · exe e8fca4 `mov r15,[rax+0x30]`) 소관이라 이 함수 mem 표엔 없음",
      evidence=u"m14.ll:38283 define 인자 %2 = `ptr readonly captures(address_is_null) %2`(+ 38299~38300 llvm.assume nonnull) · %2 파생 gep 는 +8(ptr)·+16(len) 뿐(38332~38335) — GameContext(+0x30) 로드는 호출자 buy_item(m14.ll:38174)/upgrade_item(35109) 안에 있다. exe: buy_item 0xe8fca4 `mov r15, qword ptr [rax + 0x30]`(rax=ctx) 후 [r15+8]/[r15+0x10] 을 r8/r9 로 전달",
      behavior_change=False, found_by="new")
# G16/G5: params 를 소스 인자 6행(i=1..6)으로 — self(제거) · rnd · player · context(승격) · build_slot · inventory_index(ScalarPair 2슬롯)
p.errors.append({"op": "delete", "path": "/specs[225]/sig/params[6]", "guard": u"inventory_index.payload", "kind": u"실오류",
                 "old": u"inventory_index.payload", "new": u"(삭제: 행 5 inventory_index 에 병합)",
                 "evidence": u"G16 P3/G5: sig.tcx 소스 인자 6개 ⟹ params 6행. Option<usize> 는 Rust ABI ScalarPair 라 IR 슬롯 2개(%4 태그 range(0,2) · %5 페이로드)지만 소스 인자 1개 — 23차 A #136 튜플 2슬롯과 같은 규약으로 한 행에 적는다",
                 "behavior_change": False, "found_by": "reused"})
p.fix("/specs[225]/sig/params[5]/name", old=u"inventory_index.tag", new=u"inventory_index",
      evidence=u"소스 6번째 인자 inventory_index: Option<usize> (tcx lib.rs:1149) · IR %4(태그)·%5(페이로드) ScalarPair", behavior_change=False, found_by="reused", force=True)
p.fix("/specs[225]/sig/params[5]/type", old=u"i64 range(0,2) (%4)", new=u"Option<usize> — IR ScalarPair 2슬롯: %4 태그 i64 range(0,2) · %5 페이로드 i64",
      evidence=u"m14.ll:38283 `i64 noundef range(i64 0, 2) %4, i64 %5` · 38533 `trunc nuw i64 %4 to i1`(L1153) · 38549 `icmp ult i64 %5, %66`(L1154)", behavior_change=False, found_by="reused", force=True)
p.fix("/specs[225]/sig/params[5]/note", old=u"DI inventory_index[0..+8]",
      new=u"DI inventory_index[0..+8]=%4 · [8..+8]=%5 — 태그 0=None(buy_item 상수 0 · exe [rsp+0x28]=0)·1=Some(upgrade_item 상수 1). Some 이면 %5 = player.info.items 인덱스(L1154 bounds `icmp ult %5, items.len`), None 이면 %5 undef(미참조)",
      evidence=u"m14.ll:38533/38549 · exe buy_item e8fcf2 `mov qword ptr [rsp+0x28], 0`", behavior_change=False, found_by="reused", force=True)
p.fix("/specs[225]/sig/params[5]/role", old=u"Option<usize> 태그(0=None · 1=Some). buy_item 은 상수 0, upgrade_item 은 상수 1 을 넘김",
      new=u"Option<usize> 태그(0=None · 1=Some). buy_item 은 상수 0, upgrade_item 은 상수 1 을 넘김. 페이로드(%5)는 Some 일 때 player.info.items 의 인덱스(현재 들고 있는 아이템), None 이면 undef(미참조) — 옛 7행째(payload) 를 이 행에 병합",
      evidence=u"위와 동일", behavior_change=False, found_by="reused", force=True)
p.fix("/specs[225]/sig/params[3]/name", old=u"context.item_list", new=u"context",
      evidence=u"소스 4번째 인자 context:&GameContext (tcx) — IR 에는 승격된 item_list 포인터(%2)만 들어온다", behavior_change=False, found_by="reused", force=True)
p.fix("/specs[225]/sig/params[3]/type", old=u"&Vec<Box<dyn ItemInfo>> (%2)", new=u"&GameContext (IR: +0x30 item_list 만 ArgumentPromotion 된 &Vec<Box<dyn ItemInfo>> %2)",
      evidence=u"m14.ll:38283 `ptr readonly captures(address_is_null) %2` · %2 파생 = +8 ptr · +16 len(38332~38335) = Vec 헤더 · tcxdict GameContext 0x30 item_list &Vec<Box<dyn ItemInfo>>", behavior_change=False, found_by="reused", force=True)
p.fix("/specs[225]/sig/params[3]/note", old=u"DI item_list = %2. exe 에선 (ptr,len) 두 인자",
      new=u"DI item_list = %2 (소스 context:&GameContext 의 +0x30 item_list 포인터 값만 승격 — GameContext 자체는 이 IR 에 없음) · define `ptr readonly captures(address_is_null) %2` m14.ll:38283 + llvm.assume(nonnull) 38300 · exe 에선 한 단계 더 승격돼 r8=ptr([r15+8]) · r9=len([r15+0x10]) 두 인자(argscan caller 0xe8fc80: r15=[ctx+0x30])",
      evidence=u"m14.ll:38283 · argscan.py 0xe8fd70 --caller 0xe8fc80 (e8fca4 mov r15,[rax+0x30] · e8fce0/e8fce4 mov r8/r9,[r15+8]/[r15+0x10])", behavior_change=False, found_by="reused", force=True)
p.fix("/specs[225]/sig/params[0]/note", old=u"tcx sig 에만 존재",
      new=u"tcx sig 에만 존재 — 본문 미사용이라 IR 인자에서 제거(인자로 안 넘어옴 · define 은 %0 rnd 부터 6개) · &mut self 쓰기 표면 없음",
      evidence=u"m14.ll:38283 define 인자 6개 (%0 rnd · %1 player · %2 item_list · %3 build_slot · %4 tag · %5 payload) — self 슬롯 없음 · exe 도 rcx=rnd(argscan)", behavior_change=False, found_by="reused", force=True)
p.fix("/specs[225]/sig/params[1]/note", old=u"DI rnd = %0",
      new=u"DI rnd = %0 · define `ptr noalias noundef nonnull align 16 dereferenceable(320) %0` m14.ll:38283 (readonly 없음 = 콜리 random_item_* 가 가변 소비)",
      evidence=u"m14.ll:38283", behavior_change=False, found_by="reused", force=True)
p.fix("/specs[225]/sig/params[2]/note", old=u"DI player = %1",
      new=u"DI player = %1 · define `ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(2528) %1` m14.ll:38283 — 읽기 +0x4e8/+0x4f0 item_builds · +0x4a0/+0x4a8 items · +0x998 gold",
      evidence=u"m14.ll:38283 · 38328~38331 · 38543/38561 · 38773", behavior_change=False, found_by="reused", force=True)
p.fix("/specs[225]/sig/params[4]/note", old=u"DI build_slot/slot/n = %3",
      new=u"DI build_slot/slot/n = %3 · define `i64 noundef %3` m14.ll:38283 · exe [rsp+0x20](caller) = 진입 [rsp+0xe0]",
      evidence=u"m14.ll:38283 · argscan", behavior_change=False, found_by="reused", force=True)

# ───────────────────────── 226 build_game_finish_check_state ─────────────────────────
p.fix("/specs[226]/mem[9]/name", old=u"{top,mid,bottom}_tower[1-t]", new=u"top_tower[1-t] (line=Top · Mid → mid_tower 0x1a0 · Bottom → bottom_tower 0x1c0 = 0x180+line*32)",
      evidence=u"tcxdict AbstractGameWithCache: 0x180 top_tower · 0x1a0 mid_tower · 0x1c0 bottom_tower ([Option<&Entity>;2] 16B) · m14.ll:7840~7847 `shl i8 %3, 5` + 384 + (1-t)*8 — chk 「오귀속」은 이름이 사전 필드명과 달라서 난 것", behavior_change=False, found_by="new")
p.fix("/specs[226]/mem[10]/name", old=u"{top,mid,bottom}_tower2[1-t]", new=u"top_tower2[1-t] (line=Top · Mid → mid_tower2 0x1b0 · Bottom → bottom_tower2 0x1d0 = 0x190+line*32)",
      evidence=u"tcxdict AbstractGameWithCache: 0x190 top_tower2 · 0x1b0 mid_tower2 · 0x1d0 bottom_tower2 · m14.ll:7848~7851", behavior_change=False, found_by="new")
p.fix("/specs[226]/mem[24]/note", old=u"aux 클로저: 328 + (1-t)*32 (Vec 32B, len +0x18) != 0",
      new=u"aux 클로저: twin_towers 는 0x130 [bumpalo Vec;2] · [1-t].len = 0x130 + (1-t)*32 + 0x18 = 328 + (1-t)*32 (m00.ll:73149~73153 gep {{ptr,ptr,i64},i64} + 328) != 0",
      evidence=u"tcxdict AbstractGameWithCache 0x130 twin_towers [Vec<&Entity>;2](64B) · m00.ll:73149 `getelementptr inbounds nuw { { ptr, ptr, i64 }, i64 }, ptr %87, i64 %94` · 73152 `+328`", behavior_change=False, found_by="new")
p.fix("/specs[226]/sig/tls/role", old=u"작성자+소비자(읽고 없으면 계산해 쓰는 메모). 이 함수 밖의 다른 소비자는 이 배치 범위에서 미확인",
      new=u"작성자+소비자(읽고 없으면 계산해 쓰는 메모) — 유일 접점: 전 _gaibc 에서 `RefCell<FinishAggCache>…LocalKey::with` 인스턴스는 m00.ll:72405(이 함수의 closure#0) 1개, `@anon…71`(call_once fn-ptr) 참조는 m14.ll:7814 1곳뿐. m08.ll:722·m11.ll:40858/40864 의 FINISH_AGG_CACHE 참조는 std TLS 저장소(lazy Storage · __tls 접근자) 플럼빙",
      evidence=u"grep -c FinishAggCacheEE4with _gaibc/*.ll → m00 4(define 1 + 참조) · m14 3(call 1 + summary) · 그 외 0 / grep FINISH_AGG_CACHE m08·m11 = thread_local storage 심볼만", behavior_change=False, found_by="new")
p.fix("/specs[226]/sig/tls/layout",
      old=u"RefCell 안에서는 전부 +8 (슬롯[t][p] 태그 = 8+(t*5+p)*64+57, seed=+648, tick=+656)",
      new=u"RefCell 안에서는 전부 +8 (슬롯[t][p] 태그 = 8+(t*5+p)*64+57, seed=+648, tick=+656). 슬롯 원소 64B 중 살아있는 바이트 = 0x00~0x39(58B: usize×7 + has_enemy_twin_tower i8 @56 + has_epic_buff/태그 i8 @57) · +58~+63 6B 는 패딩(미스 경로 store 없음 m00.ll:73233~73248 · 히트 경로는 그 6B 를 memcpy 로 클로저 sret +58 에 그대로 복사 72585~72586/73347~73348 → 호출자 %11 은 0~57 만 읽음)",
      evidence=u"m00.ll:73233~73248 슬롯 store 9개(+0..+48 i64 ×7 · +56 i8 · +57 i8) · 72585 `gep %64, 58` + memcpy 6B · 73347~73348 sret+58 memcpy 6B · m14.ll:8156~8172 호출자 로드 = +0..+48, +56, +57", behavior_change=False, found_by="new")

# ───────────────────────── 223 DefenseNexusSubPlan::score ─────────────────────────
p.fix("/specs[223]/logic",
      old=u"주의: ①Ult(15) 는 L319 의 +100 가산 대상이지만 L329 매치에서는 0 가산(calculate_action_score 호출 없음).",
      new=u"주의: ①Ult(15) 는 L319 의 +100 가산 대상이지만 L329 매치에서는 0 가산(calculate_action_score 호출 없음) — 오라클 U3: Ult+대상 없음 → -99999 는 interaction_score 가 낸 값이고 본 함수 가산은 0 · Ult+대상 있음은 이 세계에서 interaction_score 가 action_score.rs:932 unwrap 패닉(콜리 입력 조건 미상 → 재료 부재).",
      evidence=u"o223_cases.log U1/U2(양쪽 PANIC · action_score.rs:932:125 `called Option::unwrap() on a None value`) · U3(score -99999 = expect base -99999 + extra 0)", behavior_change=False, found_by="new")

# ───────────────────────── ev_up ─────────────────────────
def evs(spec, field, idxs, ev, to=2):
    for j in idxs:
        p.ev("/specs[%d]/%s[%d]" % (spec, field, j), evidence=ev, to=to, found_by="new")

# 223: 실행으로 닿은 행
evs(223, "consts", [0,1,2,3,4,5,6,7,8,9,10,12,13,14,15,16,17,18,19], O223 + u" — 분기: +100(A3/A3b/A4/A4b ugt 1 경계) · -99999(A2/S3/T4) · +5(R1/R6/R7/R9 vs R2/R3/R4/R5/R8 0) · Skill2 level≤2 패닉(T2) · effect None 패닉(A5/S2/T3) · switch 가지 Attack/Skill/Skill2/Around/AroundHide")
evs(223, "consts", [11], O223 + u" — Push(2) 를 calculate_action_score 10번 인자로 넘긴 기대값(expect 모드 직접 호출)과 score 값 일치(A1/A3/S1/T1)")
evs(223, "knobs", [0,1,2,3,4], O223)
evs(223, "mem", list(range(0, 34)), O223 + u" — R1~R10(blackboard[1-team] front_minion 3라인·nexus[team]·Entity team/ty/id/x/y) · A/S/T(attack/skill/skill2 effect·tag·Box·level) · 전 케이스(player_champion·cache·context·SmallActionPlay 태그/+0x8)", to=3)
p.ev("/specs[223]/sig/params[3]", evidence=O223 + u" — 27케이스 전부 rnd 320B 전후 비트 동일(rnd_changed=false): 이 세계에서 score·interaction_score·calculate_action_score 모두 rnd 미소비", to=2, found_by="new")
p.ev("/specs[223]/sig/params[0]", evidence=O223 + u" — self 24B 전후 비트 동일(self_eq=true · 27케이스)", to=2, found_by="new")
p.ev("/specs[223]/sig/params[1]", evidence=O223 + u" — ver=1 → +100 없음(A4) · ver=2 → +100(A4b) · ver=55 기본", to=2, found_by="new")

# 224
evs(224, "consts", [0,1,2,3,4,5,6,7,8,9,10,12,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30], O224 + u" — G1/G2(cc None·0) · G3~G8 게이트 · B2~B5(clamp 10/45) · F1~F10(L570 경계·L602 경계·growth·buff·move_speed*8) · I1~I6(35%/15% 경계·sdiv/2) · N1~N4(*8·24 상한) · H1~H4(45/65) · T1~T4(catch 25000+ms*18) · R1/R2(itr 30000) · X1~X4(160 상한)")
evs(224, "consts", [11], O224 + u" — possible_risk(param.player, data, 90) 직접 호출값(0)이 incoming 합에 들어감(I1~I6 값 일치)")
evs(224, "consts", [13], O224 + u" — N1~N4 에서 near_enemies 판정(120000 이내 가시 적) 일치 · 40000 항이 갈리는 케이스(120000 밖·사거리+40000 안)는 미구성")
evs(224, "knobs", [0,1,2,3,4,5,6,7,8,10], O224)
evs(224, "mem", list(range(0, 32)), O224 + u" — 게이트/경계 케이스가 team·ty·cc_immune·cc(Vec push)·x/y·level·stat_buff.range·radius_mult·radius·move_speed·hp·stat_cached.hp·Effect range/growth/target·ScoreParameter applyed/risk/risk_possible_tower 를 각각 갈랐다", to=3)
for j in range(0, 8):
    p.ev("/specs[224]/sig/params[%d]" % j, evidence=O224, to=2, found_by="new")

# 227
evs(227, "consts", [0,1,2,3,4,5,6], O227 + u" — W1/W1b(tick ≥ disable) · W2~W7b(R 경계·buff·growth·radius_mult 100/0) · W8(attack_effect -1) · W9~W13(nearest_enemy) · W14(mid 타워 · 1-team)")
evs(227, "knobs", [0,3], O227)
p.ev("/specs[227]/knobs[1]", evidence=O227 + u" — W11/W12: 미니언 0 < 2 && in_range → true · W10/W13: in_range 거짓 → false (미니언이 없는 세계라 cnt≥2 가지는 미실행)", to=2, found_by="new")
evs(227, "mem", list(range(0, 25)), O227, to=3)
for j in range(0, 5):
    p.ev("/specs[227]/sig/params[%d]" % j, evidence=O227 + (u" — ver=1/55 동일(W16)" if j == 0 else u""), to=2, found_by="new")

p.brief_error(u"§5 경로 문법: `/specs[i]/sig/params[j]/role` 은 v2 행에 `role` 과 `note` 가 **둘 다** 있는 명세(#225)에서 v2 `role` 에 적용되는데, v3 가 보여주는 `role` 열은 v2 `note`(mkspec3:565)다 — 배치가 본 문면(v3 role)을 old 로 쓰면 applypatch 가 「old 없음」으로 거부하고, 반대로 v2 role 문면을 쓰면 v3 에 안 보이는 칸을 고친다. 이 배치는 `/note` 경로로 직접 썼다(mkpatch.fix 는 v3 에 note 키가 없어 force 필요). 도구가 `role`→`note` 매핑을 「원래 키가 있으면 우선」이 아니라 「v3 가 표시하는 키」로 정해야 한다")
p.brief_error(u"§4 G16 P1/P3 지시(「소거 1개가 필요한데 role 이 소거를 주장한 것 0개」)는 #225 에서 두 현상이 겹친 것을 못 갈랐다: self 소거(-1) 와 Option<usize> ScalarPair(+1) 가 상쇄돼 IR 6 = 소스 6 인데, paramrole.ir_slots 가 Option 을 1슬롯으로 세고 DROPPED 어휘(`인자에서 제거`)만 본다. 6행으로 맞추면 P1 이 j→%j 로 **틀린 레지스터에** 매핑해 P4 를 검사하므로(rnd 행을 %1 player 와 대조) 이 배치는 속성 인용을 전부 `define …` 백틱(F1 IR 인용) 으로 써서 P4 주장을 비웠다 — ir_slots 에 `Option<정수>`(ScalarPair) 2슬롯 + DROPPED 보정 후 매핑을 넣어야 한다")
p.brief_error(u"프롬프트 초점 ④ 「DieTickCache ← check_kill_die_tick · POS_EVAL/INTER_CTX ← interaction_score」: #223 은 콜리 경유 TLS 만 있어 `sig.tls` 가 「없음」이 맞고(본문 @anon 6개 전부 panic Location), 그 대신 오라클에서 실측된 함정 = interaction_score/calculate_action_score 를 **같은 프로세스에서 두 번** 부르면 두 번째가 재생돼 값 대조가 무효(첫 판 rnd_eq 비교가 그래서 깨졌다) — 프롬프트 ⑥ 「케이스당 프로세스 1개」는 대상 함수만이 아니라 **기대값 계산(콜리 직접 호출)도 별도 프로세스**여야 한다는 뜻으로 명시해 달라")
p.brief_error(u"프롬프트 초점 ⑥ 「internal fastcc 는 exe 인자 배치를 argscan 으로 재확인만」— #224(internal fastcc)는 **직접 호출자 v21_runaway_defensive_cc_bonus 가 define hidden 이고 goal==RunAway 이면 인자를 그대로 tail-call** 하므로 link_name 으로 실행 진입이 된다(51케이스). 「internal 이면 오라클 불가」로 읽히는 문면을 「호출자가 얇은 hidden 래퍼면 그리로 진입」으로 고쳐 달라(#225 item_v26·#226 finish 도 호출자 buy_item/end_check 가 pub/hidden 이나 입력 조립(ItemInfo 객체·end_check 출력 해석)이 커서 이번엔 미탐색)")
p.brief_error(u"#227 notes[0](ev5) 와 consts[3](ev5) 가 `Effect::range(&self, caster, extra)` 3인자를 「추정」했는데 tcx sig(effect.rs:25 · 2인자)가 이미 정본에 있었다(callees[10] 행) — 명세 안의 callees 시그니처와 consts 의 추정이 모순인데 G20 도 G4 도 안 잡는다(같은 명세 안 callees↔consts/notes 교차는 무검사 축). 검사기 제안: consts/notes 문면의 `<이름>(…)` 호출 표기 인자 수를 callees 의 sig 인자 수와 대조")
# from 을 v3 현재 ev 로 맞춘다
for u in p.ev_up:
    row = mkpatch.locate(u["path"])
    if isinstance(row, dict) and isinstance(row.get("ev"), int):
        u["from"] = row["ev"]
# kind 보정: 문면 보강은 보강으로
BOGANG = [(226, None), (223, '/logic'), (225, '/note'), (225, 'params[5]/role'), (227, 'mem[24]/note'), (227, 'iter_towers')]
for e in p.errors:
    if e['kind'] != u'실오류': continue
    for si, key in BOGANG:
        if e['path'].startswith('/specs[%d]' % si) and (key is None or key in e['path'] or key in e['old']):
            e['kind'] = u'보강'
out = p.save()
