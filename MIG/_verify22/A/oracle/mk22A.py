# -*- coding: utf-8 -*-
"""22차 배치A patch.json 생성 (mkpatch 참조구현 사용). 실행: python -X utf8 mk22A.py"""
import sys, io
sys.stdout.reconfigure(encoding='utf-8')
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch

p = mkpatch.Patch(round=22, batch="A")
ORA = u"오라클 22차A(_verify22/A/oracle/v22A_o1.rs · 명세 독립 재구현 ⓓ · 케이스당 프로세스 1개)"

# ═══════════════ 112 nontarget_windup_perceived ═══════════════
p.fix("/specs[112]/logic",
      old=u"caster.action_state.time /*+0x78*/",
      new=u"caster.action_state@{Attack|Skill|Skill2|Ult}.time /*+0x78 · variant 필드 usize(enum+0x8) — 메서드 아님*/",
      kind=u"오탐",
      evidence=u"tcx: `game_core::ChampionActionState::{Attack,Skill,Skill2,Ult}::time` 는 **Field**(champion.rs:27:32/28:31/29:32/30:29) · game_core 에 `::time` AssocFn 0건(tcxq grep `::time$` → Field 만) · MIR Entity::action_time entity.rs:1598:39~43 `((… as Attack).1: usize)` = 필드 복사. G9 CHAIN 규칙이 `time` 을 (다른 크레이트) 함수명과 겹친다는 이유로 메서드로 오판 — 표기만 `@variant` 형으로 바꿔 게이트를 닫는다",
      behavior_change=False, found_by="new")
p.fix("/specs[112]/logic",
      old=u"{ bb.last_visible[p.position.as_index()] + 120 >= game.tick() }   // 120틱 = 2초@60tps",
      new=u"{ bb.last_visible[pos_idx] + 120 >= game.tick() }   // pos_idx = p.info.position.as_index() (PlayerState+0x9c0 · g07.ll:157031~157034 · 콜리 내부 읽기라 이 함수 mem 표 밖) · last_visible = Blackboard+0x1e0 (g07.ll:157036~157038) · 120틱 = 2초@60tps · 오라클 22차A: tick500 에서 last_visible 379→false / 380→true (`icmp uge %25,%28` 경계 실행 확정)",
      kind=u"오탐",
      evidence=u"g07.ll:157031 `%19 = getelementptr inbounds nuw i8, ptr %16, i64 2496` · 157036 `%22 = getelementptr inbounds nuw i8, ptr %0, i64 480` · 157039 `%25 = add i64 %24, 120` · 157043 `%29 = icmp uge i64 %25, %28`. 이 읽기는 game_core 콜리 Blackboard::is_recent_visible 안에서 일어나고 이 함수(m04.ll 53108~53231)의 IR 에는 PlayerState+0x9c0 접근이 0건 — `mem` 은 이 함수가 짚는 자리의 표이므로 행이 없는 것이 옳다(G18 오탐). 문면은 `p.info.position` 으로 정밀화하고 주석으로 옮겨 게이트를 닫는다",
      behavior_change=False, found_by="new")
p.fix("/specs[112]/logic",
      old=u"let cast_start = game.tick().saturating_sub(elapsed);         // L53214~53219 vtable+0x28=tick, llvm.usub.sat",
      new=u"let cast_start = data.cache.game.tick().saturating_sub(elapsed);   // L53214~53219 vtable+0x28=tick, llvm.usub.sat · L565 66자 정합(`  let cast_start = data.cache.game.tick().saturating_sub(elapsed);`) · 오라클: elapsed 500/501/600 @tick500 → seed 동일(포화 0)",
      kind=u"보강",
      evidence=u"rmeta_srcmap utils.rs L565 = 66자(LF 제외) = `  let cast_start = data.cache.game.tick().saturating_sub(elapsed);` 정확 일치 · IR 53214 `%54 = getelementptr inbounds nuw i8, ptr %16, i64 40`(vtable+0x28) · 53219 `@llvm.usub.sat.i64(i64 %56, i64 %42)` · 오라클 v22A_o1 112 tick=500 el=500/501/600 → seed=0x1900002(cast_start 0) game==mine",
      behavior_change=False, found_by="new")
p.fix("/specs[112]/logic",
      old=u"let seed = player.info.id ^ (caster.id << 20) ^ (cast_start << 40);   // L53221~53226",
      new=u"let seed = (player.info.id as u64) ^ ((caster.id as u64) << 20) ^ ((cast_start as u64) << 40);   // L53221~53226 · L566 96자 정합 · 오라클 22차A: tick500 스윕(elapsed 0~600, cast_start 별 seed 변동 → react 61~113 변동) 34케이스 game==mine",
      kind=u"보강",
      evidence=u"rmeta_srcmap utils.rs L566 = 96자 = `  let seed = (player.info.id as u64) ^ ((caster.id as u64) << 20) ^ ((cast_start as u64) << 40);` 정확 일치 · IR 53223 `%60 = shl i64 %46, 20` · 53225 `%62 = shl i64 %57, 40` · 53224/53226 xor · 오라클 v22A_o1 112 (tick=500 lv=450) 34케이스 MISMATCH 0",
      behavior_change=False, found_by="new")
p.fix("/specs[112]/open[2]",
      old=u"action_time() 의 Champion 이외 EntityType 도 경과틱을 가질 수 있는지 — 본문은 tag 13 만 취급(entity.rs:1593). 다른 타입은 0 확정, 소스에 다른 arm 이 있었다면 사장 코드",
      new=u"★해소(22차A · MIR Entity::action_time entity.rs:1592~1613 · _tcx/mirdump_game_core.txt:7547): 소스 arm 은 `EntityType::Champion(champ)`(태그 13, L1593) 하나뿐이고 나머지는 `_ => 0`(L1611:12) — 다른 EntityType 은 0 확정이고 사장 arm 은 없다. Champion 내부 match(L1594): Idle(L1595)/Return(L1596)/Move(L1597) → 0 · Attack(L1598)/Skill(L1601)/Skill2(L1604)/Ult(L1607) → `.1`=time. 오라클 22차A: ty 태그 1(Minion 값) 캐스터 elapsed 0 → false(2/2)",
      kind=u"보강",
      evidence=u"mirdump_game_core.txt:7547~ `switchInt(move _2) -> [13: bb2, otherwise: bb1]` · bb1 `_0 = const 0_usize @entity.rs:1611:12-1611:13` · bb2 `switchInt(move _4) -> [0: bb10, 1: bb8, 2: bb9, 3: bb7, 4: bb6, 5: bb5, 6: bb4]` · bb7 `_0 = copy (… as Attack).1 @1598:39-1598:43`",
      behavior_change=False, found_by="new")
p.fix("/specs[112]/notes[0]",
      old=u"utils.rs:563 map 클로저의 소스 표기(`|p| p.info.parameter.skill_hit_base()`)는 인라인이라 추정 — 동작은 L53204~53205 로 확정",
      new=u"utils.rs:563 map 클로저의 소스 표기 = `    .map(|p| p.info.parameter.skill_hit_base())` **확정**(22차A: rmeta_srcmap L562=53자 `    let h = data.cache.player_by_champion_id(caster.id)` · L563=47자 · L564=19자 `    .unwrap_or(50);` 세 줄 전부 정확 일치 · dbg !62467 `p` arg2 line 563) — 동작은 L53204~53205 로 확정",
      kind=u"보강",
      evidence=u"python rmeta_srcmap.py game_ai utils.rs 553 568 → L562 54B/L563 48B/L564 20B(LF 포함) · m04.ll:62467 `!DILocalVariable(name: \"p\", arg: 2, … line: 563)` · 53204 `%50 = getelementptr inbounds nuw i8, ptr %47, i64 384` · 53205 call skill_hit_base",
      behavior_change=False, found_by="new")
for j in (2, 3, 4, 5, 8, 10):
    p.ev("/specs[112]/mem[%d]" % j, to=3, evidence=ORA + u" 112: raw 오프셋 독립 재현(0x180/0x10/0x0/0x0·0x8/0x70/0x5c0) 85케이스 MISMATCH 0", found_by="new")
for j in (0, 2, 3, 4, 5, 6, 7, 9, 10):
    p.ev("/specs[112]/consts[%d]" % j, to=2, frm=(3 if j == 2 else 4), evidence=ORA + u" 112: tick0/500 × 상태 0~6 × 비챔피언 × elapsed 스윕 85케이스 MISMATCH 0 · 경계 elapsed 107→false/108→true(react 108, uge)", found_by="new")
p.ev("/specs[112]/knobs[2]", to=2, evidence=ORA + u" 112: last_visible 379→false / 380→true @tick500 (+120 uge 경계)", found_by="new")

# ═══════════════ 113 v3_deadly_edge_cells ═══════════════
p.fix("/specs[113]/logic",
      old=u"fn v3_deadly_edge_cells(version, player, data) -> i32 {",
      new=u"fn v3_deadly_edge_cells(version, player, data) -> u32 {   // ★반환은 u32(tcx sig · L304 96자 정합) — IR 의 i32 는 LLVM 무부호 표기, min 은 llvm.umin.i32",
      kind=u"실오류",
      evidence=u"tcx sig `fn(usize, &PlayerState, &OperationData) -> u32` · rmeta_srcmap tower_discipline.rs L304 = 96자 = `pub fn v3_deadly_edge_cells(version: usize, player: &PlayerState, data: &OperationData) -> u32 {` 정확 일치 · m07.ll:49771 `%61 = call noundef range(i32 0, 61) i32 @llvm.umin.i32(i32 %60, i32 60)` · 오라클 22차A: hp=1·ms=1·타워 damage 12708741 → cells_raw=2147483650(=2^31+2) → game=60 — 부호있는 i32 min 으로 재구현하면 -2147483646 이 나온다",
      behavior_change=True, found_by="new")
p.fix("/specs[113]/logic",
      old=u"let cells = (edge_dmg.saturating_mul(30) / max(champ.hp, 1)) as i32 + 2;  // 30/현재HP 화폐",
      new=u"let cells = ((edge_dmg.saturating_mul(30) / max(champ.hp, 1)) as u32).wrapping_add(2);  // 30/현재HP 화폐 · trunc→u32 · `add i32` nsw/nuw 없음 = wrapping(오버플로 검사 없음)",
      kind=u"실오류",
      evidence=u"m07.ll:49766 `%59 = trunc i64 %58 to i32` · 49767 `%60 = add i32 %59, 2`(플래그 없음 · 패닉 분기 없음) · 오라클 22차A 26+9케이스 MISMATCH 0(wrap 케이스 포함)",
      behavior_change=True, found_by="new")
p.fix("/specs[113]/logic",
      old=u"  min(cells, 60)\n}",
      new=u"  min(cells, 60)   // llvm.umin.i32 = 부호없는 min. 오라클 22차A: cells_raw 2147483650 → 60 · hp 경계 9100→59 / 9063→60\n}",
      kind=u"보강",
      evidence=u"m07.ll:49771 llvm.umin.i32 · 오라클 v22A_113.tsv(hp=9100 game=59 mine=59 · hp=9063 game=60 · hp=1 ms=1 tatk=12708741 cells_raw=2147483650 game=60)",
      behavior_change=False, found_by="new")
p.fix("/specs[113]/sig/returns", force=True,   # v3 키는 `ret`, v2(applypatch 적용처) 키는 `returns` — mkpatch.locate 가 v3 를 읽어 force 필요(선례 20B/20E)
      old=u"i32 `range(i32 0, 61)` — 치명 엣지 셀 수.",
      new=u"u32(IR 표기 i32 · `range(i32 0, 61)` · min 은 llvm.umin.i32 무부호) — 치명 엣지 셀 수.",
      kind=u"실오류",
      evidence=u"tcx sig `-> u32` · m07.ll:49636 define `noundef range(i32 0, 61) i32` · 49771 `llvm.umin.i32` · 오라클 22차A wrap 케이스 game=60",
      behavior_change=False, found_by="new")
p.fix("/specs[113]/logic",
      old=u"find_map 순회 순서 = Chain: 고정 슬롯 [Option<&Entity>;6] (인덱스 0..6 순, null skip) → 추가 타워 슬라이스 순. '첫' 매치에서 멈춘다(try_fold Break) — 어느 타워를 집는지는 iter_towers_without_nexus 의 배열 순서에 종속(내부는 game_core g15.ll:108971 미독해).",
      new=u"find_map 순회 순서 = Chain: 고정 슬롯 6 = [top_tower(cache+0x180), mid_tower(+0x1a0), bottom_tower(+0x1c0), top_tower2(+0x190), mid_tower2(+0x1b0), bottom_tower2(+0x1d0)][team] 순(null skip · g15.ll:108978~108995 = simulation.rs:1832~1837) → twin_towers[team] 슬라이스(cache+0x130, stride 32 · L1838) 순. '첫' 매치에서 멈춘다(try_fold Break). ★오라클 22차A(killmask 9케이스, 슬롯별 damage 상이): 집는 타워 id 3→7→11→5→9→13→16→17→(전부 None → 2) 로 위 순서 실행 확정.",
      kind=u"보강",
      evidence=u"g15.ll:108978 `%6 = getelementptr inbounds nuw i8, ptr %1, i64 384`(0x180, L1832) · 108981 `i64 416`(0x1a0, L1833) · 108984 `i64 448`(0x1c0, L1834) · 108987 `i64 400`(0x190, L1835) · 108990 `i64 432`(0x1b0, L1836) · 108993 `i64 464`(0x1d0, L1837) · 109009 `i64 304`(0x130 twin_towers, L1838) · 109034~109044 슬롯 6개 store 순서 %8,%11,%14,%17,%20,%23 · tcxdict AbstractGameWithCache 0x180 top_tower/0x190 top_tower2/0x1a0 mid_tower/0x1b0 mid_tower2/0x1c0 bottom_tower/0x1d0 bottom_tower2/0x130 twin_towers · 오라클 v22A_113b.tsv",
      behavior_change=False, found_by="new")
p.fix("/specs[113]/logic",
      old=u"        .find_map(|t| {\n            let eff = t.attack_effect.as_ref()?;                      // Entity+0x4c0 == -1 → None → 다음 타워\n            Some((eff.expected_damage_target(data.context, t as &dyn AbstractEntity, champ),   // Effect+0 = Entity+0x490\n                  t.attack_cooltime()))\n        })",
      new=u"        .find_map(|t| t.attack_effect.as_ref()                        // L312 · 바깥 클로저 인자 이름 t(dbg) · Entity+0x4c0 == -1 → None → 다음 타워\n            .map(|a| (a.expected_damage_target(data.context, t as &dyn AbstractEntity, champ),   // L313 · Option::map 클로저(option.rs:1162 프레임) 인자 이름 a(dbg) · Effect+0 = Entity+0x490 · 캡처 data__context·champ\n                      t.attack_cooltime())))",
      kind=u"보강",
      evidence=u"m11.ll:55721 `!DILocalVariable(name: \"t\", arg: 2, scope: !55716, … line: 312)` · 55736 `name: \"a\", arg: 2, scope: !55726, … line: 313` · 55737 `data__context`(line 304) · 55738 `champ`(line 308) · 34846 사슬 `tower_discipline.rs:313@closure$0 <- option.rs:1162@map<ref$<Effect>,tuple$<usize,usize>,…>` — `?` 가 아니라 `as_ref().map(|a| …)` 구조",
      behavior_change=False, found_by="new")
p.fix("/specs[113]/knobs[4]/where",
      old=u"tower_discipline.rs:305 (m07:49646)",
      new=u"tower_discipline.rs:305 (m07:49646 `icmp ult i64 %0, 2`)",
      kind=u"오탐",
      evidence=u"m07.ll:49646 `%6 = icmp ult i64 %0, 2, !dbg !61831` · 127 is_safe_recall 의 같은 노브는 m07.ll:58772 `%156 = icmp ugt i64 %0, 1` — 외연 동일(임계 2 = version≥2). G20 R4 는 sharedchk._extent 가 where/effect 문면의 `icmp …` 로만 임계를 환산하는데 양쪽 다 문면이 없어 값 1/2 로 충돌 판정(오탐). 이쪽 where 에 icmp 를 박아 닫는다 — 127(배치 C) 쪽도 `icmp ugt i64 %0, 1` 표기가 필요",
      behavior_change=False, found_by="new")
p.fix("/specs[113]/open[1]",
      old=u"find_map 이 '첫' 타워에서 멈추므로 적 타워가 여럿이면 어느 타워의 (shot,cool) 이 쓰이는지는 iter_towers_without_nexus 배열 순서에 달렸다 — 그 순서(라인/티어 순?)는 g15.ll 미독해",
      new=u"★해소(22차A · g15.ll:108971~109056 독해 + 오라클 실행): 순서 = top_tower → mid_tower → bottom_tower → top_tower2 → mid_tower2 → bottom_tower2 (cache+0x180/0x1a0/0x1c0/0x190/0x1b0/0x1d0 · simulation.rs:1832~1837) → twin_towers[team] 슬라이스(cache+0x130, L1838). 즉 1차 타워 3라인 → 2차 타워 3라인 → 쌍둥이 타워 순이고 각 슬롯이 None(파괴)이면 건너뛴다. 오라클 killmask 9케이스: 집힌 id 3→7→11→5→9→13→16→17→2(없음)",
      kind=u"보강",
      evidence=u"g15.ll:108978~108995 gep 384/416/448/400/432/464 · 109009 gep 304 · 109029~109055 Chain 조립(a: IntoIter alive 0..6 슬롯 %8,%11,%14,%17,%20,%23 · b: slice ptr %26/end %29) · 오라클 v22A_113b.tsv MISMATCH 0",
      behavior_change=False, found_by="new")
p.fix("/specs[113]/open[3]",
      old=u"info.position 태그 4 (5번째 variant) 가 무엇인지 안 봄 — [5 x ptr] 라 5슬롯 전부 유효 인덱스",
      new=u"★해소(22차A · tcxdict --enum Position): 태그 4 = Support (Top0 Jungle1 Mid2 Bottom3 Support4 · 인코딩 Direct · idx==태그) · Position::as_index = `discriminant as usize`(MIR entity.rs:581) — [5 x ptr] 5슬롯 전부 유효",
      kind=u"보강",
      evidence=u"tcxdict --enum Position: `4 4 4 Support` · mirdump_game_core.txt:6085 `_0 = copy _2 as usize (IntToInt) @entity.rs:581:5-581:15`",
      behavior_change=False, found_by="new")
p.fix("/specs[113]/open[4]",
      old=u"`add i32 %59, 2` 는 nsw 없음 → trunc 후 i32 wrap 가능(edge_dmg*30/hp 가 2^31 초과 시). 실전 값에선 도달 어려움(추정)",
      new=u"★실행 확인(22차A): `add i32 %59, 2` 는 nsw/nuw 없음 → trunc→u32 wrapping_add. 오라클 hp=1·ms=1·타워 damage 12708741(shot 12582912): edge_dmg=134217728000 → ×30 mod 2^32 = 2^31 → +2 = 2147483650 → umin → **60**(game==mine). wrap 은 도달 가능하나 결과는 상한 60 에 흡수된다 — 단 재구현이 i32 부호있는 min 을 쓰면 -2147483646(→ logic/ret 를 u32 로 정정). 실전 값(타워 한 방 ≤ 수천, hp ≥ 1)에선 2^31 미도달(추정 유지)",
      kind=u"보강",
      evidence=u"오라클 v22A_113.tsv: `ver=2 hp=1 ms=1 … tatk=12708741 game=60 mine=60 ok=true shot=12582912 cool=3 … cells_raw=2147483650` · search113.py 로 (shot·32000/3)·30 mod 2^32 = 2^31 되는 damage 를 탐색해 투입",
      behavior_change=False, found_by="new")
p.fix("/specs[113]/open[0]",
      old=u"`Effect::expected_damage_target`(g06.ll:52355) · `Entity::attack_cooltime`(g06.ll:66510) · `iter_towers_without_nexus`(g15.ll:108971) 내부는 안 읽음",
      new=u"`Effect::expected_damage_target`(g06.ll:52355) · `Entity::attack_cooltime`(g06.ll:66510) 내부는 안 읽음(22차A 오라클 관측만: 타워 TowerAttackEffect(damage,ratio 0) 에서 shot = damage 500→496 / 1000→991 / 4000→3961 / 12708741→12582912 ≈ damage − ⌊damage/101⌋ · attack_cooltime = 3) · `iter_towers_without_nexus`(g15.ll:108971) 는 22차A 에서 독해(open[1] 해소)",
      kind=u"보강",
      evidence=u"오라클 v22A_113.tsv shot 열 · g15.ll:108971~109056 독해(open[1] 참조)",
      behavior_change=False, found_by="new")
for j in (4, 5, 6, 7):
    p.ev("/specs[113]/mem[%d]" % j, to=3, evidence=ORA + u" 113: champ None(캐시 슬롯 null 쓰기)→2 · +0x640 ms · +0x670 hp · 타워 +0x4c0=-1 skip 전부 실행 일치(26+9케이스)", found_by="new")
for j in (0, 2, 3, 4, 5, 6, 7, 8, 9):
    p.ev("/specs[113]/consts[%d]" % j, to=2, evidence=ORA + u" 113: version 0/1/2/3/99 · ms 0/1/300/10667/16000/32000/32001/100000 · hp 0/1/100/9000/9063/9100/10000/20000/100000 · 타워 None/killmask · wrap 케이스 = 35케이스 MISMATCH 0", found_by="new")
for j in range(5):
    p.ev("/specs[113]/knobs[%d]" % j, to=2, evidence=ORA + u" 113: 35케이스 MISMATCH 0(경계 hp 9100→59/9063→60 · ver 1→2/2→계산 · ms 32000→1틱/16000→2틱)", found_by="new")

# ═══════════════ 114 attack_summon_action ═══════════════
p.fix("/specs[114]/consts[0]/src_line", old=839, new=851,
      kind=u"실오류",
      evidence=u"m15.ll:39931 `!DILocalVariable(name: \"move_speed\", … line: 839)` 의 값은 28256 `%33 = load i64, ptr %32`(원값 · ×30 전) 이고 fight_check.rs 소속 DILocalVariable 전량(39925~39953)에 `leeway` 류 변수가 없다 · 28290 `%53 = mul i64 %33, 30` 은 !dbg 없이 호이스트된 명령(세 사용처 28508/28659/28805 의 공통 부분식) · L840 은 빈 줄(1B) · 리터럴 30 은 L851·861·871 의 `move_speed * 30` 세 곳(첫 줄 851 을 대표로) — G12 는 !dbg 없는 mul 을 못 봐서 통과시켰다",
      behavior_change=False, found_by="new")
p.fix("/specs[114]/consts[0]/meaning",
      old=u"이속 × 30틱(=0.5초@60tps) 를 사거리 여유로 가산 — 세 후보(평타·스킬·스킬2) 사거리 모두에 더함(28290 mul, 28508·28659·28805)",
      new=u"이속 × 30틱(=0.5초@60tps) 를 사거리 여유로 가산 — 세 후보(평타·스킬·스킬2) 사거리 모두에 더함. 소스: L839 `let move_speed = champ.stat_cached.move_speed`(dbg 변수 move_speed = 원값 %33 · leeway 변수 없음) → L851·861·871 각각 `move_speed * 30`(리터럴 3회) 을 LLVM 이 28290 `mul i64 %33, 30`(!dbg 없음) 하나로 호이스트해 28508·28659·28805 에서 가산",
      kind=u"보강",
      evidence=u"위 src_line 정정과 동일 근거(m15.ll:39931 · 28256 · 28290 · 39925~39953 변수 전량)",
      behavior_change=False, found_by="new")
p.fix("/specs[114]/knobs[1]/where",
      old=u"fight_check.rs:839 (m15.ll:28290)",
      new=u"fight_check.rs:851·861·871 (`move_speed * 30` 3곳 → m15.ll:28290 호이스트 mul · 가산 28508/28659/28805)",
      kind=u"실오류",
      evidence=u"consts[0] 와 동일 — L839 는 `move_speed` 원값 바인딩(dbg !39931 = %33)이고 `*30` 은 사용처 3줄에 있다",
      behavior_change=False, found_by="new")
p.fix("/specs[114]/logic",
      old=u"  let leeway = champ.stat_cached.move_speed * 30                                                     // L839 (28254~28256, 28290)",
      new=u"  let move_speed = champ.stat_cached.move_speed                                                      // L839 (28254~28256 · dbg 변수 move_speed = 원값) — 여유 `move_speed * 30` 은 L851/861/871 에서 각각(28290 = 호이스트된 공통 mul)\n  let leeway = move_speed * 30                                                                        // (의사코드 편의 — 소스엔 이 변수 없음)",
      kind=u"보강",
      evidence=u"consts[0] 정정 근거와 동일(m15.ll:39931 · 28256 · 28290)",
      behavior_change=False, found_by="new")
p.fix("/specs[114]/logic",
      old=u"  let mut ret = Vec::new_in(data.context.pool)                                                       // L838 (28249~28255)",
      new=u"  let mut ret /*소스 이름 candidates (dbg !39929)*/ = Vec::new_in(data.context.pool)                 // L838 (28249~28255)",
      kind=u"보강", evidence=u"m15.ll:39929 `!DILocalVariable(name: \"candidates\", … line: 838)`",
      behavior_change=False, found_by="new")
p.fix("/specs[114]/logic",
      old=u"  for target in cache.others[1 - team].iter() {                                                      // L841 적 팀 others(소환수) 순회 (28258~28275, 28323)",
      new=u"  for target /*소스 이름 e (dbg !39935 · `for &e in …` L841 60자 정합)*/ in cache.others[1 - team].iter() {   // L841 적 팀 others(소환수) 순회 (28258~28275, 28323) · others = cache+0xf0 [bumpalo Vec;2] stride 32(tcxdict)",
      kind=u"보강", evidence=u"m15.ll:39935 `!DILocalVariable(name: \"e\", … line: 841, type: !2740)`(=&Entity, champ 와 같은 타입) · rmeta_srcmap L841 = 60자 = `  for &e in data.cache.others[1 - player.info.team].iter() {` 정합",
      behavior_change=False, found_by="new")
p.fix("/specs[114]/logic",
      old=u"    if !target.is_visible_from(champ.player_team()) { continue }                                     // 28354~28371 (champ Neutral 이면 생략)",
      new=u"    if !target.is_visible_from(champ) { continue }                                                   // 28354~28371 · Entity::is_visible_from(&self=target, other=champ) -> bool: other.team.player_team() 이 Some(t) 면 target.visible_state[t]@tag == 0(Visible), Neutral 이면 true (MIR entity.rs:1481~1487 · player_team 인라인 entity.rs:1136~1137)",
      kind=u"실오류",
      evidence=u"tcx sig `game_core::Entity::is_visible_from fn(&Entity, &Entity) -> bool`(인자는 &Entity 이지 Option<usize> 가 아니다) · mirdump_game_core.txt:7364 bb0 `discriminant(((*_2).6: TeamType))` → bb4 `_5 = ((_3 as Some).0)` → bb1 `_0 = Eq(discriminant(((*_1).38: [VisibleState;2])[_5]), 0)` · bb3(Neutral) `_0 = const true` · m15.ll:28354~28371",
      behavior_change=False, found_by="new")
p.fix("/specs[114]/logic",
      old=u"        let r = atk.range + leeway + champ.stat_buff_cached.range + (level-1)*atk.growth_range\n              + range_adjust(atk, champ, target) + radius(champ) + radius(target)                     // L851~852 (28508~28513)",
      new=u"        let range = atk.range(champ) /*= atk.range + atk.growth_range*(champ.level-1) + champ.stat_buff_cached.range — Effect::range(&self,&Entity) MIR effect.rs:26*/\n                  + leeway + atk.range_adjust(champ, target) + champ.radius()                     // L851 (dbg 변수 range) · 28508~28512 — leeway 가산이 effect.rs:26 dbg 를 단 것은 LLVM 재결합\n        let r = range + target.radius()                                                             // L852 (dbg 변수 max_dist) · 28513",
      kind=u"보강",
      evidence=u"mirdump_game_core.txt:5058 Effect::range: `_4 = (*_1).0`(range, 26:5-15) · `_5 = Mul((*_1).1, (*_2).1 - 1)`(growth_range*(level-1), 26:18-63) · `_10 = (((*_2).18).23)`(stat_buff_cached.range, 26:66-98) · `_0 = Add(Add(_4,_5), _10)` — 인자 2개(self, caster) 뿐이라 leeway 는 range() 인자가 아님 · m15.ll:39939 `range`(line 851) · 39941 `max_dist`(line 852) · 28508 `%154 = add i64 %119, %53`(effect.rs:26 dbg) · 28513 `%159 = add i64 %158, %153`(fight_check.rs:852)",
      behavior_change=False, found_by="new")
p.fix("/specs[114]/notes[0]",
      old=u"Effect::range(effect.rs:26) 인라인의 정확한 인자(leeway 가 range() 인자인지 호출부 가산인지)는 소스 표기 미확정 — 합산 결과는 IR 로 확정",
      new=u"★확정(22차A · MIR): Effect::range(&self, caster: &Entity) -> u64 = self.range + self.growth_range * (caster.level − 1) + caster.stat_buff_cached.range (effect.rs:26:5~98, 인자 2개) — leeway(move_speed*30) 는 range() 인자가 아니라 호출부 L851/861/871 에서 가산된다. IR 28508 `add %119, %53`(leeway) 이 effect.rs:26 dbg 를 단 것은 LLVM 덧셈 재결합 아티팩트. 합산 결과는 오라클 22차A 17케이스(경계 20030/20031 · 25030/25031 · 30030/30031) 로 실행 확정",
      kind=u"보강",
      evidence=u"mirdump_game_core.txt:5058~5088 · tcx sig `game_core::Effect::range fn(&Effect, &Entity) -> u64` · 오라클 v22A_114.tsv",
      behavior_change=False, found_by="new")
p.fix("/specs[114]/mem[39]/note",
      old=u"184B memcpy(28572·28723·28869). 24B 이후 바이트는 지역 alloca 의 미초기화 잔여(런타임 대조 시 +0x18..+0xb1 은 비교 제외)",
      new=u"184B memcpy(28572·28723·28869). 페이로드 24B 중 **살아있는 바이트 = +0x0 start_tick(usize)·+0x8 target(usize)·+0x10 is_act(bool) = 17B**(tcxdict SmallActionAttack/Skill/Skill2 세 구조체 동일 레이아웃) · +0x11..+0x18 은 패딩(오라클 22차A: 같은 new 호출도 값이 달라 비결정 → 비교 제외) · +0x18..+0xb1 은 지역 alloca 미초기화 잔여(비교 제외) · 관측값 start_tick = game.tick(0/100) · target = e.id · is_act = 0",
      kind=u"보강",
      evidence=u"tcxdict SmallActionAttack: 0x0 start_tick usize / 0x8 target usize / 0x10 is_act bool (24B) · 오라클 v22A_114.tsv `el[i] tag=15 live17=[100,0,…,23,0,…] start_tick=100 target=23 is_act=0` · 첫 실행에서 별도 new() 호출의 pad7 이 [250,114,195,202,…] 로 달랐음",
      behavior_change=False, found_by="new")
p.fix("/specs[114]/sig/returns", force=True,
      old=u"bumpalo Vec<SmallActionPlay>(32B 전부 살아있음: ptr·bump·cap·len).",
      new=u"bumpalo Vec<SmallActionPlay>(32B 전부 쓰이나 **재현 가능한 값은 len 과 원소뿐** — ptr 은 bump 주소(len 0 이면 8), cap 은 할당기 성장값(오라클 관측 len 1→cap 1 · 2→2 · 3→4 · 6→8 · 8→8), bump = ctx.pool; 런타임 대조는 len · 원소별 태그(+0xb1) · 페이로드 17B 만).",
      kind=u"보강",
      evidence=u"오라클 v22A_114.tsv len/cap/ptr_dangling8 열(len=0 → ptr==8 · cap=0) · m15.ll:28249 `store ptr inttoptr (i64 8 to ptr)` · 28551/28702/28848 reserve_internal_or_panic",
      behavior_change=False, found_by="new")
p.fix("/specs[114]/open[1]",
      old=u"SmallActionAttack/Skill/Skill2::new(sret 24B, &OperationData, target_id) 의 24B 내용 미독(별도 함수). 184B 원소 중 +0x18..+0xb0 은 지역 alloca 미초기화 잔여 — 런타임 비교 시 제외 대상",
      new=u"★해소(22차A): SmallActionAttack/Skill/Skill2 = { start_tick: usize(+0x0), target: usize(+0x8), is_act: bool(+0x10) } 24B(tcxdict, 세 타입 동일) · 오라클 관측 new(data, id) 결과 = (start_tick = game.tick(), target = id, is_act = false) 17케이스 · +0x11..+0x18 패딩(비결정) · +0x18..+0xb1 미초기화 — 런타임 비교는 17B + 태그만",
      kind=u"보강",
      evidence=u"tcxdict SmallActionAttack/SmallActionSkill/SmallActionSkill2 · 오라클 v22A_114.tsv el[] 행(tick 0 → start_tick 0 · tick 100 → start_tick 100)",
      behavior_change=False, found_by="new")
for j in (0, 1, 2, 3, 4, 5, 8, 9):
    p.ev("/specs[114]/consts[%d]" % j, to=2, evidence=ORA + u" 114: 거리 경계(20030/20031 · 25030/25031 · 30030/30031 · 80000/80001) · 가시성 · level 2/3 · 태그 15/16/17 · 빈 Vec = 17케이스 MISMATCH 0(페이로드 17B 대조 포함)", found_by="new")
for j in (0, 1):
    p.ev("/specs[114]/knobs[%d]" % j, to=2, evidence=ORA + u" 114: dist 80000→후보 / 80001→없음 · 여유 = 이속(1)×30 이 사거리 20030 에 포함(경계 20030/20031)", found_by="new")
for j in (10, 11, 12, 13, 14, 15, 16, 17, 18, 20, 21, 23, 24, 25, 27, 28, 30, 31, 32, 33, 35, 36, 37, 38, 39, 40):
    p.ev("/specs[114]/mem[%d]" % j, to=3, evidence=ORA + u" 114: raw 오프셋으로 조건 조립(0x38 visible_state · 0x4a0/0x4a8 · 0x4d8/0x4e0 · 0x510/0x518 · 0x5c8 level · 0x660/0x668) 17케이스 MISMATCH 0 · 반환 Vec 32B·원소 184B·태그 +0xb1 직독", found_by="new")

# ═══════════════ 115 is_cleared ═══════════════
p.fix("/specs[115]/open[1]",
      old=u"camp_pos 가 2번 호출되는 것이 소스에서 튜플 1회 호출인지 2회 호출인지 — IR 로는 구분 불가(표기 불가). 동작(x,y 동일 호출)은 확정",
      new=u"★판정 반전(22차A): **소스도 2회 호출**이다 — LLVM 은 호출을 하나에서 둘로 분열시키지 않으며, 두 call(L62484 `.0` 추출 · L62486 `.1` 추출)이 합쳐지지 않은 것은 camp_pos 가 readonly/readnone 속성이 없기(m04.ll:68802 declare · TLS RefCell 메모 쓰기) 때문. 즉 `map.camp_pos(camp, team == 0).0` 과 `….1` 두 식. 줄 길이도 정합: rmeta_srcmap L853 = 134자 ↔ 2회형 `  let dist = distance(champ.x, champ.y, data.context.map.camp_pos(camp, team == 0).0, data.context.map.camp_pos(camp, team == 0).1);` 132자(±2) / 1회형(`let (x, y) = …` 분리)은 한 줄 ≤ ~105자. 동작(x,y 동일 인자 호출)은 오라클 33케이스로 확정",
      kind=u"실오류",
      evidence=u"m04.ll:62484 `%53 = tail call { i64, i64 } @…MapDef8camp_pos(… %51, i8 noundef %0, i1 noundef zeroext %52)` · 62485 `extractvalue %53, 0` · 62486 두 번째 call(같은 인자) · 62487 `extractvalue %55, 1` · 68802 `declare { i64, i64 } @…camp_pos(…) unnamed_addr #1`(readonly/readnone 없음) · rmeta_srcmap passive_jungle.rs L853 = 135B(LF 포함) · 오라클 v22A_115.tsv 33/33",
      behavior_change=False, found_by="new")
p.fix("/specs[115]/logic",
      old=u"let dist = distance(champ.x, champ.y, map.camp_pos(camp, team == 0));                     // L62475~62488 (camp_pos 가 x,y 각각 1회씩 2번 call — 순수함수 CSE 미적용, 소스는 1회 호출로 추정)",
      new=u"let dist = distance(champ.x, champ.y, data.context.map.camp_pos(camp, team == 0).0, data.context.map.camp_pos(camp, team == 0).1);   // L62475~62488 — ★소스도 2회 호출(LLVM 은 호출을 분열시키지 않고, camp_pos 는 readonly 속성이 없어 CSE 도 안 됨 · L853 134자 정합) · 오라클 22차A 33케이스 일치",
      kind=u"실오류",
      evidence=u"open[1] 정정과 동일(m04.ll:62484~62487 · 68802 · rmeta_srcmap L853)",
      behavior_change=False, found_by="new")
p.fix("/specs[115]/logic",
      old=u"let move_tick = dist / champ.stat_cached.move_speed;                                     // L62490~62496 udiv (speed 0 → panic)",
      new=u"let move_tick = dist / champ.stat_cached.move_speed;                                     // L62490~62496 udiv (speed 0 → panic_const_div_by_zero · 오라클 22차A 실측 Location passive_jungle.rs:854:21 'attempt to divide by zero')",
      kind=u"보강",
      evidence=u"오라클 v22A_o1 115 ms=0: `thread 'main' panicked at game-ai\\src\\plan_legacy\\old\\passive_jungle.rs:854:21: attempt to divide by zero` · m04.ll:62510 panic_const_div_by_zero",
      behavior_change=False, found_by="new")
p.fix("/specs[115]/open[2]",
      old=u"_version/_rnd/_debug 는 IR 상 완전 미사용(readnone/poison) — 시그니처 호환용으로 추정",
      new=u"_version/_rnd/_debug 는 IR 상 완전 미사용(readnone/poison) — 시그니처 호환용으로 추정. 오라클 22차A: 더미 StdRng(seed 1)·DebugFrameData::default() 를 넘긴 33케이스 + 패닉 3케이스 전부 정상(readnone 실행 확증 · `_` 접두 이름은 dbg !71412/!71413/!71418) — 사실 서술로 notes 이관 권고",
      kind=u"보강",
      evidence=u"m04.ll:62404 `ptr noalias readnone align 16 captures(none) %3` · `ptr noalias readnone align 8 captures(none) %8` · 62409 `#dbg_value(i64 poison, !71412` · 오라클 v22A_115.tsv",
      behavior_change=False, found_by="new")
for j in (1, 3, 7, 8, 9, 10, 11, 12, 13):
    p.ev("/specs[115]/mem[%d]" % j, to=3, frm=(3 if j in (1, 7, 10) else 4), evidence=ORA + u" 115: next_respawn_tick[team][idx](TeamPlan+0x378) · GameContext+0x20/+0x8 · GameSetting+0x12f8 · Entity+0x660/0x668/0x640 을 독립 재현해 33케이스 MISMATCH 0", found_by="new")
p.ev("/specs[115]/consts[2]", to=2, evidence=ORA + u" 115: team 0/1 로 camp_pos 좌표 대칭(496000,752000)↔(752000,496000) 확인 · 33케이스 MISMATCH 0", found_by="new")
for j in (0, 1):
    p.ev("/specs[115]/knobs[%d]" % j, to=2, evidence=ORA + u" 115: next = rhs → false / rhs+1 → true / rhs−1 → false (rhs = move_tick+offset+tick+60) · offset 0/77/500 · tps 60 포함 경계 정확", found_by="new")

# ═══════════════ 116 can_tower_focused_when_attack ═══════════════
p.fix("/specs[116]/notes[0]",
      old=u"Entity+0x68 == 2 비교(m07:51020~51023)에 !dbg 가 없어 소스 줄(89 또는 90)과 표기(`let EntityType::Tower{info}` vs `matches!`) 는 미확정 — 동작(태그≠2 → false) 은 확정",
      new=u"★확정(22차A): Entity+0x68 == 2 비교(m07:51020~51023)는 **L91 `    if let EntityType::Tower { info } = &tower.ty {`**(rmeta_srcmap 51자 정확 일치 · dbg !63101 `info` line 91) — let-else 도 `matches!` 도 아니다. !dbg 가 없는 것은 L89 `?`(option.rs:2775 branch, `let champ = cache.player_champion[player.info.team][player.info.position.as_index()]?;` 90자 정합)의 null 검사와 `select`(51023) 로 병합됐기 때문. 클로저는 L88 `let f = move || {`(19자) … L119 `f().unwrap_or(false)`(22자 정합) 이며 Option<bool> 을 돌려준다(L94 `return Some(true);` 28자 · L103 `Some(cnt < 2)` 21자 정합)",
      kind=u"보강",
      evidence=u"python rmeta_srcmap.py game_ai tower_discipline.rs 83 120 → L88 20B/L89 91B/L91 52B/L92 50B/L93 28B/L94 29B/L103 22B/L119 23B(LF 포함) · m07.ll:63101 `!DILocalVariable(name: \"info\", … line: 91)` · 63021 `name: \"f\", … line: 88` · 51018 `%33 = icmp ne ptr %32, null, !dbg !63141`(option.rs:2775@branch) · 51023 `%37 = select i1 %33, i1 %36, i1 false`",
      behavior_change=False, found_by="new")
p.fix("/specs[116]/mem[7]/note",
      old=u"이 load 는 !dbg 없음(L89~90 사이 추정, `champ.is_some() && matches!(tower.ty, Tower{..})` 형태)",
      new=u"이 load 는 !dbg 없음 — L91 `if let EntityType::Tower { info } = &tower.ty {`(51자 정합, dbg 변수 info L91) 의 태그 비교가 L89 `?` 의 null 검사와 select(m07:51023) 로 병합된 것(`matches!` 아님)",
      kind=u"실오류",
      evidence=u"notes[0] 정정과 동일(rmeta_srcmap L91 = 52B · m07.ll:63101 · 51018/51023)",
      behavior_change=False, found_by="new")
p.fix("/specs[116]/logic",
      old=u"  (|| {   // L88~119 즉시호출 클로저(인라인)",
      new=u"  let f = move || -> Option<bool> {   // L88 `let f = move || {`(dbg 변수 f · 19자) … L119 `f().unwrap_or(false)`(22자 정합) — 인라인 · `?`/Some 으로 Option<bool>",
      kind=u"실오류",
      evidence=u"m07.ll:63021 `!DILocalVariable(name: \"f\", scope: !63022, … line: 88)` · rmeta_srcmap L88 = 20B · L119 = 23B(`  f().unwrap_or(false)` 22자) · L120 = 2B(`}`)",
      behavior_change=False, found_by="new")
p.fix("/specs[116]/logic",
      old=u"    let champ = cache.player_champion[team][player.info.position]?;   // cache+0x1e0 · None → false",
      new=u"    let champ = cache.player_champion[player.info.team][player.info.position.as_index()]?;   // L89 90자 정합 · cache+0x1e0 · None → `?` → None → unwrap_or(false)",
      kind=u"보강", evidence=u"rmeta_srcmap tower_discipline.rs L89 = 91B(LF 포함) = `    let champ = cache.player_champion[player.info.team][player.info.position.as_index()]?;` 90자 정확 일치 · m07.ll:51018 option.rs:2775@branch",
      behavior_change=False, found_by="new")
p.fix("/specs[116]/logic",
      old=u"    let EntityType::Tower { info } = &tower.ty else { return false; };   // Entity+0x68 == 2 (dbg 줄 없음)",
      new=u"    if let EntityType::Tower { info } = &tower.ty {   // L91 (51자 정합 · dbg 변수 info) · Entity+0x68 == 2 · 아니면 if-let 끝 → 클로저가 None → false (태그 비교 load 에 !dbg 없음 = L89 `?` 와 select 병합)",
      kind=u"실오류",
      evidence=u"rmeta_srcmap L91 = 52B = `    if let EntityType::Tower { info } = &tower.ty {` 51자 정확 일치(let-else 형 53자 · matches! 형 부정합) · m07.ll:63101 info line 91 · 51020~51023",
      behavior_change=False, found_by="new")
p.fix("/specs[116]/logic",
      old=u"        if id == champ.id { return true; }                        // Entity+0x5c0",
      new=u"        if id == champ.id { return Some(true); }                  // L93~94 (27자·28자 정합) · Entity+0x5c0",
      kind=u"보강", evidence=u"rmeta_srcmap L93 = 28B(`        if id == champ.id {` 27자) · L94 = 29B(`          return Some(true);` 28자)",
      behavior_change=False, found_by="new")
p.fix("/specs[116]/logic",
      old=u"        cnt < 2                                                    // ★L103: 미니언 2기 미만이면 내가 물린다",
      new=u"        Some(cnt < 2)                                              // ★L103(21자 정합): 미니언 2기 미만이면 내가 물린다 · 오라클 22차A cnt 1→true / 2→false",
      kind=u"보강", evidence=u"rmeta_srcmap L103 = 22B(`        Some(cnt < 2)` 21자) · m07.ll:51090 `%62 = icmp ult i64 %58, 2` · 오라클 v22A_116.tsv fin=1 game=true / fin=2 game=false",
      behavior_change=False, found_by="new")
p.fix("/specs[116]/logic",
      old=u"        !in_range_minions\n    }\n  })()\n}",
      new=u"        Some(!in_range_minions)   // L111\n    }   // (if-let Tower 가 아니면 여기서 None)\n  };\n  f().unwrap_or(false)   // L119 (22자 정합)\n}",
      kind=u"보강", evidence=u"rmeta_srcmap L119 = 23B · m07.ll:51050 `%50 = xor i1 %49, true`(L111) · 51095 phi [false,%4],[%62,%51],[%50,%48],[false,%25],[true,%42]",
      behavior_change=False, found_by="new")
p.fix("/specs[116]/logic",
      old=u"순회 = iter_minions(team) 의 3 슬라이스 Chain(Chain(top,mid),bottom 추정 — g15.ll:102624 미독해) 순.",
      new=u"순회 = iter_minions(team) = top_minions[team](cache+0x10 · simulation.rs:1848).chain(mid_minions[team](+0x50 · L1849)).chain(bottom_minions[team](+0x90 · L1850)) 순 — g15.ll:102633~102700 독해(22차A) · 오라클 18케이스 MISMATCH 0(top/bottom 슬라이스에 가짜 미니언 배치).",
      kind=u"보강",
      evidence=u"g15.ll:102633 `%6 = getelementptr inbounds nuw i8, ptr %1, i64 16`(L1848) · 102650 `i64 80`(L1849) · 102675 `i64 144`(L1850) · 102695~ Chain 조립 · tcxdict AbstractGameWithCache 0x10 top_minions/0x50 mid_minions/0x90 bottom_minions",
      behavior_change=False, found_by="new")
p.fix("/specs[116]/open[0]",
      old=u"`iter_minions(cache, team)`(g15.ll:102624) 이 돌려주는 3 슬라이스의 정체(top/mid/bottom_minions[team] 추정 — cache 필드 배치 0x10/0x50/0x90 근거) 는 내부 미독해",
      new=u"★해소(22차A · g15.ll:102624~102700 독해): iter_minions(cache, team) = top_minions[team](+0x10, L1848) → mid_minions[team](+0x50, L1849) → bottom_minions[team](+0x90, L1850) 의 Chain<Chain<Copied<Iter>,Copied<Iter>>,Copied<Iter>> · 오라클 22차A 18케이스로 순회·count·any 실행 확정",
      kind=u"보강",
      evidence=u"g15.ll:102633/102650/102675 gep 16/80/144 · 102695 `store i64 1, ptr %0`(Chain.a Some) · 오라클 v22A_116.tsv",
      behavior_change=False, found_by="new")
for j in (3, 6, 8, 9, 10):
    p.ev("/specs[116]/mem[%d]" % j, to=3, evidence=ORA + u" 116: GameSetting+0x13f8(tick 9999998/9999999) · 캐시 슬롯 null · +0x88/+0x98 nearest_enemy 직접 쓰기 · +0x5c0 id 비교 18케이스 MISMATCH 0", found_by="new")
for j in (1, 2):
    p.ev("/specs[116]/consts[%d]" % j, to=2, frm=(3 if j == 1 else 4), evidence=ORA + u" 116: 비타워 엔티티(챔피언) 전달 → false · cnt 0/1→true, 2/3→false(경계 2) · 18케이스 MISMATCH 0", found_by="new")
for j in (0, 1):
    p.ev("/specs[116]/knobs[%d]" % j, to=2, evidence=ORA + u" 116: cnt 1→true / 2→false · tick 9999998→계산 / 9999999(=tower_attack_disable_tick)→false", found_by="new")

# ═══════════════ 117 camp_idx ═══════════════
p.fix("/specs[117]/consts[0]/src_line", old=35, new=36,
      kind=u"보강",
      evidence=u"rmeta_srcmap team_plan.rs L35 = 17B(`  match jungle {` 16자) · L36 = 28B(`    JungleType::Rhino => 0,` 27자 정합) · L37 31B/L38 26B/L39 28B(`Mushroom => 1,`/`Bee => 2,`/`Stump => 3,` 각 30/25/27자 정합) — Rhino arm 은 L36 이고 IR 은 switch(L35, m09:54336)에서 곧장 병합블록 phi `[ 0, %1 ]`(m09:54357, line 0)로 가 L36 DILocation 이 없다. 다른 arm(consts[1]~[3])이 arm 줄(37/39/38)을 쓰므로 일관되게 36",
      behavior_change=False, found_by="new")
p.fix("/specs[117]/notes[0]",
      old=u"소스 L36 이 무엇인지(IR 에 줄 36 DILocation 없음 — `match jungle {` 다음 빈 줄이거나 Rhino arm 이 35 에 접힘 추정). 동작은 4 arm 전부 확정",
      new=u"★확정(22차A · rmeta_srcmap): L36 = `    JungleType::Rhino => 0,`(27자 정합) — Rhino arm 이다. IR 에 L36 DILocation 이 없는 것은 switch 의 `i8 0, label %6`(m09:54337) 이 곧장 병합 블록으로 가고 값 0 이 phi 상수(line 0)로만 남기 때문. L35 `  match jungle {`(16자) · L40 `    _ => unreachable!(),`(24자 · Location 40:10 = `unreachable!` 시작 칸) · L41 `  }` · L42 `}`. 동작은 오라클 22차A 6/6(0→0 1→1 2→3 3→2 · 4/5 → panic team_plan.rs:40:10)",
      kind=u"보강",
      evidence=u"python rmeta_srcmap.py game_ai plan_legacy\\\\team_plan.rs 30 44 → L34 47B/L35 17B/L36 28B/L37 31B/L38 26B/L39 28B/L40 25B/L41 4B/L42 2B(LF 포함) · 오라클 v22A_o1 117 case 0~5",
      behavior_change=False, found_by="new")
p.fix("/specs[117]/logic",
      old=u"    JungleType::Morgard | JungleType::Serpen => unreachable!(),   // 태그 4·5 → core::panicking::panic (L40, team_plan.rs:40:10)",
      new=u"    _ => unreachable!(),   // L40 (24자 정합 `    _ => unreachable!(),` · Location 40:10 = `unreachable!` 시작 칸) · 태그 4 Morgard·5 Serpen → core::panicking::panic — 오라클 22차A 실측 'internal error: entered unreachable code' @team_plan.rs:40:10",
      kind=u"보강",
      evidence=u"rmeta_srcmap L40 = 25B(24자 · `JungleType::Morgard | JungleType::Serpen => unreachable!(),` 형은 62자로 부정합) · m09.ll:54344 panic(@anon…304, 40, @anon…305) · 오라클 case 4/5 stderr",
      behavior_change=False, found_by="new")
p.fix("/specs[117]/logic",
      old=u"호출자 1곳(exe 0xde5300 · 54B).",
      new=u"IR 호출처 3곳(m04.ll:32448·32575 = PassiveJunglePlan::next_plan · m04.ll:62447 = is_cleared) · 자기 exe 주소 0xde5300(54B).",
      kind=u"실오류",
      evidence=u"m04.ll:32448 `%1193 = call noundef i64 @…camp_idx(i8 noundef %1106)` · 32575 `%1259 = call …camp_idx(i8 noundef %1107)`(둘 다 define PassiveJunglePlan::next_plan 안) · 62447 `%27 = tail call noundef i64 @…camp_idx(i8 noundef %0)`(is_cleared) — v3 헤더 「호출처 3곳」과 일치. 0xde5300 은 camp_idx 자신의 exe 주소(exe.addr)이지 호출자가 아니다",
      behavior_change=False, found_by="new")
for j in range(5):
    p.ev("/specs[117]/consts[%d]" % j, to=2, evidence=ORA + u" 117: JungleType 0~5 각 프로세스 1개 — 0→0 1→1 2→3 3→2 · 4/5 → panic 'internal error: entered unreachable code' team_plan.rs:40:10 (6/6)", found_by="new")

# ═══════════════ 지시(도시에) 오류 ═══════════════
p.brief_error(u"v22_prompt_A ⑥「should_add_self_etc_buff_action(129) exe 인자 대응표」와 ④의 internal fastcc 3함수(check_nontarget 119·line_recall_pressure_penalty 120·129)는 배치 A(112~117) 담당이 아니다(129 = DOSSIER_D §1). 4배치 공통 초점 문단이 A 지시문에 그대로 실려 「보고에: 129 exe 인자 대응표」가 A 에게 요구됐다 — 해당 없음으로 보고")
p.brief_error(u"DOSSIER_A §4 G9: 「`time` 는 **메서드**이고 … logic 표기를 `time()` 로 고칠 것」은 틀렸다 — tcx 에 ChampionActionState::{Attack,Skill,Skill2,Ult}::time 은 Field 이고 game_core 에 `time` AssocFn 이 없다. 게이트(specgate G9 CHAIN)가 다른 크레이트의 동명 함수로 오판한 오탐이며 `time()` 로 고쳤으면 명세가 틀려졌다")
p.brief_error(u"DOSSIER_A §4 G20 R4 「[113]`2` · [127]`1` 같은 노브가 값 1/2」— 두 함수의 버전 게이트는 `icmp ult %0,2`(113) 와 `icmp ugt %0,1`(127) 로 외연이 같다(임계 2). sharedchk._extent 가 where/effect 문면의 icmp 로만 환산하는데 양쪽 다 icmp 문면이 없어 생긴 오탐 — 도구 쪽은 value 정규화(`>1`→2) 또는 IR 직독으로 고쳐야 한다")
p.brief_error(u"DOSSIER_A §4 G18 「[112] logic 이 필드 p.position 을 읽는데 mem 표에 없다」— 그 읽기는 game_core 콜리 Blackboard::is_recent_visible(g07.ll:157031) 내부이고 logic 의 `[game_core 콜리 …]` 부속 블록에 적힌 것이다. xreflogic P2 가 콜리 독해 블록을 본문으로 읽는다(구제 통로: `[… 콜리 …]` 헤더 아래 블록 제외 또는 주석 처리)")
p.brief_error(u"spec_11x.md 헤더 「exe | `d399a0` (None) · None바이트 · None명령」— mkdossier 가 exe.size/insn 이 없을 때 None 을 그대로 찍는다(6함수 전부). 표기 결함이지 사실 오류는 아님")
p.brief_error(u"도구 결함: 반환값 칸의 키가 v3 는 `sig.ret`, v2(applypatch 적용처)는 `signature.returns` 라 `/specs[i]/sig/ret` 경로는 mkpatch 검증은 통과하고 applypatch 는 「필드가 문자열이 아니고 문면도 못 찾음」으로 거부한다(dry-run 실측 2건). 선례(20B/20E)대로 `/sig/returns` + force=True 로 우회 — V2KEY 에 `ret→returns` 를 넣어야 한다")
p.brief_error(u"§1 표의 「ev≥4(미실행)」 수치가 실행 대상 수와 무관함은 §1 자체가 경고하지만, 112 의 consts[8](50 · unwrap_or 기본값)은 소유 선수가 없으면서 game.is_visible 이 true 인 엔티티가 필요해 이 오라클 세팅(start_game 직후, 미니언 미스폰)으로는 닿지 않는다 — 범위 명시: 미실행(재료 = 월드 시야 상태 조작 미탐색)")

p.save()
