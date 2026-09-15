# -*- coding: utf-8 -*-
"""25차 배치E patch.json 생성 — 194 HideSubPlan::action_candidates. `python -X utf8 mkpatch_E.py`"""
import sys; sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch

p = mkpatch.Patch(round=25, batch="E")
S = "/specs[194]"

# ── G12 consts[19] src_line=81 : 오탐 (gep 오프셋 제외 규칙) ──
p.fix(S + "/consts[19]/meaning",
      old="AbstractGame vtable 슬롯 +0x28 = tick() (23833) — Trace.start_tick.",
      new="AbstractGame vtable 슬롯 +0x28 = tick() (23833) — Trace.start_tick. (★G12 오탐: 40 은 `%1373 = getelementptr inbounds nuw i8, ptr %1368, i64 40, !dbg !32518`(m02.ll:23833) 의 gep 오프셋이라 srclinecheck 규칙②가 후보에서 제외하고, 잔여 후보 [63] 은 `%366 = gep %17, i64 40 ;L69<836<3387<63`(21347, 클로저 env 필드 오프셋)이다. 주석본 cb7540.ll:3123 사슬 `;L45<81` = trace.rs:45 inlinedAt hide.rs:81 로 루트 줄 81 이 맞다)",
      evidence="_next/reach/cb7540.ll:3123 `23833|  %1373 = gep %1368, i64 40  ;L45<81` · m02.ll:23833 `%1373 = getelementptr inbounds nuw i8, ptr %1368, i64 40, !dbg !32518` · 23835 `%1375 = invoke noundef i64 %1374(ptr noundef nonnull %1367)`(tick 슬롯 호출, 같은 !dbg) → 23865 `store i64 %1375, ptr %1387`(+88 start_tick). srclinecheck.py L212 「gep 오프셋은 상수가 아니다」 규칙이 vtable 슬롯 오프셋을 걸러 claim 81 의 강후보가 0 이 된 도구 오탐",
      behavior_change=False, found_by="reused", kind=u"오탐")

# ── G15 consts[2] kind=임계 NEG : meaning 의 「임계 아님」 부정문이 관측(icmp ult)과 충돌 → 문면 정정 ──
p.fix(S + "/consts[2]/meaning",
      old="MapDef.bushes[30][30] 그리드 폭 — x,y 각각 <30 bounds check(20547/20551 · 20881/20885 · 21727/21731). 임계 아님",
      new="MapDef.bushes[30][30] 그리드 폭 = 배열 인덱스 상한 — x,y 각각 `icmp ult %idx, 30` bounds check(20547/20551 · 20881/20885 · 21727/21731), 실패 시 panic_bounds_check. 게임 판정값이 아닌 배열 경계 상한(IR 소비가 순서비교라 kind 는 임계로 둔다)",
      evidence="m02.ll:20547 `%106 = icmp ult i64 %108, 30` · 20551 `%107 = icmp ult i64 %110, 30` → 20555/20562 `panic_bounds_check(i64 %110, i64 30, …)`. kindchk 는 CMP_ORD 관측이면 낱말 `길이` 를 기각하고 `임계` 로 두는 설계(mkspec3._kind 규칙③)라 「임계 아님」 부정문이 G15 NEG 를 만든 것 — 값·줄·동작은 그대로",
      behavior_change=False, found_by="reused", kind=u"오탐")

# ── G16 sig.params[1] P4 : 「비-readonly」 부정 표기를 게이트가 못 읽음 → 문면 정정 ──
p.fix(S + "/sig/params[1]/role",
      old="(배치 M) IR 속성 noalias·비-readonly = &mut.",
      new="(배치 M) IR 속성 noalias 있음 · readonly 없음 = &mut.",
      evidence="m02.ll:20312 define 인자 `ptr noalias noundef align 8 captures(none) dereferenceable(16) %1` — readonly·readnone·initializes 없음. 명세 주장은 맞고 paramrole.NEGATED 가 `비-` 접두 부정을 부정으로 못 읽은 도구 오탐(같은 인자를 「readonly 없음」이라 적은 앞 절은 통과)",
      behavior_change=False, found_by="reused", kind=u"오탐")

# ── sig.tls_note : 콜리 귀속이 틀렸다 — 전이 콜그래프에서 TLS 접점은 camp_pos→CAMP_POS_MEMO 뿐 ──
p.fix(S + "/sig/tls_note",
      old="TLS 는 콜리 내부(can1v1win m04.ll:54936 · battle_action m15.ll:23700 · expected_damage_target g06.ll:52355 등)의 몫이며 호출 순서는 logic 의 분기 순서 그대로(L76 can1v1win → L80/L87 battle_action; L173/189/205 expected_damage_target 은 정글 루프 안 순서)",
      new="전이 콜그래프(_gaibc+_gcbc 36,096 define · 직접호출 + LocalKey anon 상수 참조 + llvm.threadlocal.address, 25차 E cg2.py)에서 이 함수가 닿는 define 106개 중 TLS 접점은 **MapDef::camp_pos(g07.ll:152570) → CAMP_POS_MEMO(LocalKey<RefCell<(usize,[[Option<(u64,u64)>;2];8])>>::with) 단 1곳**이며, 호출 조건 = L34 `!self.check_move` 일 때 정확히 1회(L39 is_top_side 참 → Morgard(4) / 거짓 → Serpen(5), blue_side=team==0; m02.ll:20626/20630) — 다른 모든 콜리보다 먼저 발화. can1v1win(전이 10 define)·battle_action(60)·attack_summon_action(23)·expected_damage_target(6)·is_recent_visible·AroundPosition/AroundBush 생성자·Attack/Skill/Skill2::new 는 전이 TLS 0. vtable 간접 대상(AbstractGame::is_visible/tick/get_entity_by_id 의 Game·SingleLaneGame·DeathMatchGame·ExpectedGame impl, AbstractEntity-for-Entity impl 8개)도 TLS 0. SIEGE_STANCE_CACHE·CAST_BEAMS·HP_VALUE_MEMO·POS_EVAL_CACHE·INTER_CTX·DIE_TICK_CACHE·MAX_RANGE_CACHE·LAST_STAND_MEMO 는 이 루트에서 도달 불가",
      evidence="scratchpad25E/cg2.py+tlsreach2.py: reachable=106 · TLS hits=[d1 CAMP_POS_MEMO @ gcbc/g07.ll:152570 (camp_pos ← action_candidates)]. 대조군: BattleSubPlan::action_candidates 는 같은 도구로 505 define·TLS 16종(CAST_BEAMS·DIE_TICK_CACHE·POS_EVAL_CACHE…) 검출 — 도구가 TLS 를 보는 것은 확인됨. m02.ll:20626/20630 camp_pos 호출 2사이트는 %147(20621 `icmp ult i64 %146, %139`) 분기의 양쪽이라 L34 진입 시 정확히 1회",
      behavior_change=False, found_by="new", kind=u"실오류")

# ── sig.params[3] rnd : 실제 소비처 보강 ──
p.fix(S + "/sig/params[3]/role",
      old="이 함수 본체가 직접 읽지 않음 | (배치 M) can1v1win 1번 인자 · battle_action 2번 인자로 전달만",
      new="이 함수 본체가 직접 읽지 않음. ★실제 소비는 두 생성자뿐: can1v1win(m04.ll:54936 `%0` readnone)·battle_action(m15.ll:23700 `%2` readnone, DI 이름 `_rnd`)은 rnd 를 읽지 않는다. L43/L50 AroundPosition::new_with_out_line → wait_around(m04.ll:33007) 의 SliceRandom::choose(12칸 슬라이스) 1회, L96 AroundBush::new_with_out_line(m08.ll:105131) 의 SliceRandom::choose(그 bush 셀 목록 Vec) 1회 — 두 경로는 상호배타(L43/50 은 check_move 가 0 으로 남는 경로, L96 은 check_move==1 경로)라 호출당 최대 1회 소비. 오라클 36/36 RND_MATCH(같은 시드 복제본에 choose 를 같은 순서로 걸어 next_u64 일치) | (배치 M) can1v1win 1번 인자 · battle_action 2번 인자로 전달만(둘 다 readnone)",
      evidence="m04.ll:54936 `define … can1v1win(ptr noalias noundef readnone align 16 captures(none) dereferenceable(320) %0, …)` · m15.ll:23700 `battle_action(… , ptr noalias readnone align 16 captures(none) %2, …)` + !37144 `_rnd` · m04.ll:33067 wait_around 안 `SliceRandom::choose(… i64 noundef 12, …)` · m08.ll:105203 AroundBush::new_with_out_line 안 `SliceRandom::choose(… i64 noundef %30 …)`. 오라클 o194 C01~C34 RND_MATCH 36/36",
      behavior_change=False, found_by="new", kind=u"보강")

# ── sig.params[2] version : 콜리 battle_action 도 안 쓴다 → 194 서브트리 전체에서 무효 ──
p.fix(S + "/sig/params[2]/role",
      old="battle_action 1번 인자로 그대로 전달(IR 25182·25240·25317)",
      new="battle_action 1번 인자로 그대로 전달(IR 25182·25240·25317) — 그 battle_action 도 DI 이름 `_version`(m15.ll:100685 !37143) · define 인자 `i64 %1`(noundef 없음)으로 읽지 않으므로 version 은 194 서브트리 전체에서 결과에 무관(오라클 C32 version=2 ↔ 기본 55 동일 · reach.py version=2 사장 블록 0)",
      evidence="m15.ll:23700 `define void @…battle_action(ptr … %0, i64 %1, ptr noalias readnone … %2, …)` + m15.ll:100685 `!37143 = !DILocalVariable(name: \"_version\", arg: 1, …)` · 100689 `_end_delay`. m02.ll 안 `%2` 출현 = define + 5 호출 인자뿐. _next/reach/cb7540.reach.txt `블록 351 · 살아있음 351 · 사장 0`",
      behavior_change=False, found_by="new", kind=u"보강")

# ── sig.ret : AroundPosition·AroundBush 의 live 바이트가 과대 기술 ──
p.fix(S + "/sig/ret",
      old="①AroundPosition(태그 없음, +0xb1=outline_type 0..2 가 곧 니치 비태그값) = new_with_out_line sret 184B 통째(L43/L50)",
      new="①AroundPosition(태그 없음, +0xb1=outline_type 0..2 가 곧 니치 비태그값) = new_with_out_line sret 184B 중 live 는 [0,0x68)(start_tick=game.tick()·goal_x/y=camp·goal_score_diff 0·target_x/y=camp·around_input 40B=wait_around 전량·around_radius 80000·end_delay 5) · +0xad(path_finder@tag 2=None) · +0xb0(position_eval_purpose 6) · +0xb1(outline_type) 뿐이고 [0x68,0xad)·[0xae,0xb0)·[0xb2,0xb8) 은 undef(alloca 잔재 — 생성자에 initializes 속성 없음, m08.ll:103209~103231) (L43/L50)",
      evidence="m08.ll:103172 define(initializes 없음) 본문 store: 103209 +0 %16(tick) · 103211 +8 %3 · 103213 +16 %4 · 103215 +24 0 · 103217 +32 · 103219 +40 · 103221 memcpy +104←%9(72B Option<PathFinder>, 그 안엔 103198 `store i8 2, ptr %17`(+69) 만) · 103223 memcpy +48←%8(wait_around sret 40B: m04.ll:33088~33096 +0..+32 5×i64 전량) · 103225 +88 80000 · 103227 +96 %5 · 103229 +177 %6 · 103231 +176 6. 오라클 덤프(o194 C01) 원소 +0x70=34 +0x78=2 +0x88=544 등 잔재 관측",
      behavior_change=False, found_by="new", kind=u"실오류")
p.fix(S + "/sig/ret",
      old="②AroundBush(태그 12) = sret 120B 를 슬롯 [0,0x78) 에 memcpy, +0x70 out_line, [0x71,0x78)·[0x78,0xb1)·[0xb2,0xb8) undef(alloca 잔재), +0xb1=12 (L94/L96)",
      new="②AroundBush(태그 12) = sret 120B 를 슬롯 [0,0x78) 에 memcpy 하지만 생성자가 쓰는 바이트는 [0,0x28)(start_tick·change_tick·bush·target_x/y) · +0x6d(path_finder@tag 2=None) · +0x70(out_line) 뿐 — [0x28,0x6d)·[0x6e,0x70)·[0x71,0x78)·[0x78,0xb1)·[0xb2,0xb8) undef(alloca 잔재), +0xb1=12 (L94/L96)",
      evidence="m08.ll:104917 new_with_target 본문 store: 105099 +0 · 105101 +8 · 105104 +16 · 105106 +24 · 105108 +32 · 105110 `store i8 2` +109 · 105112 `store i8 %4` +112 (다른 %0 파생 쓰기 없음) / 105131 new_with_out_line: 105248~105261 동일 오프셋 집합. 두 define 모두 initializes 없음. 오라클 덤프(o194 C24) 원소 +0x28..+0x68 에 포인터 잔재 관측",
      behavior_change=False, found_by="new", kind=u"실오류")

# ── one_line : 배치 M 요약이 「enemy_spotted_me 분기」라 적었으나 함수는 enemy_spotted_me 를 읽지 않는다 ──
p.fix(S + "/one_line",
      old="배치 M(103~139) = check_move 거짓 분기·enemy_spotted_me 분기·반환",
      new="hide.rs:103~139 = `is_visible_to_enemy && !check_move` 분기(enemy_spotted_me 는 쓰기만 하고 읽지 않음)·반환",
      evidence="m02.ll:20802 `br i1 %79, label %244, label %243`(L107, %79 = 20442 is_visible 호출값) · 23986 `%1425 = load i8, ptr %215`(check_move 재로드) · self+0xa 는 store 2곳(20465·20589)뿐 load 0. 오라클 C33(esm=1 cm=0 vis=0) → L107 미진입 [AroundPosition]만, C34(esm=1 evis=1) 동일 — enemy_spotted_me 초기값은 결과에 무관",
      behavior_change=True, found_by="new", kind=u"실오류")

# ── ev 상향: 오라클 o194(46 케이스 · 케이스당 프로세스 1개 · 46/46 MATCH) ──
O = u"오라클 실행 확인(25차 E o194 · C%s): %s"
EV = [
 ("mem[45]", "07/14/05", u"enemy_spotted_me +0xa: C07(cm=1·!vis) 1→0 · C14(vis) 0→1 · C05(cm=0 진입·!vis·L45 에서 cm=1) 1 유지 — 세 조건 모두 예측 일치"),
 ("mem[46]", "05/35/36", u"check_move +0x9: C05/C35(dist²=1e10, > 아님) 0→1 · C36(dist²>1e10) 0 유지"),
 ("mem[2]", "05/23", u"check_move 재로드: C05 같은 호출 안에서 L45 의 1 이 L57 분기에 보임(AroundBush 진입) · C23 L107 skip"),
 ("mem[0]", "01~09", u"bush id 1~9 각각 map.bushes 검색·AroundBush +0x10=bush 일치"),
 ("mem[1]", "08/09", u"out_line 0/2 가 AroundBush +0x70 에 그대로"),
 ("consts[7]", "35/36", u"경계 100000²: dist²=1e10 → check_move=1 · 1e10+2e5+1 → AroundPosition (`>` 확정)"),
 ("consts[9]", "37/38", u"경계 250000: 적@bush+250000 → nearest Some(L94) · +250001 → None(L96)"),
 ("consts[10]", "39/40", u"경계 150000: 적@champ+150000 → enemies=1(dominated) · +150001 → 0"),
 ("consts[27]", "41/42", u"L117 경계 150000: +150000 → enemies=1 · +150001 → 0"),
 ("consts[5]", "01/02/27", u"is_top_side 참 → 원소 +8/+16 = camp_pos(Morgard, team==0) 좌표(288000,288000)"),
 ("consts[6]", "03/04/06", u"is_top_side 거짓 → camp_pos(Serpen) 좌표 · end_delay 5 (+0x60)"),
 ("consts[8]", "01~04", u"AroundPosition +0xb1 = 1(Outline)"),
 ("consts[13]", "05~13", u"AroundBush 태그 12 @+0xb1"),
 ("consts[14]", "16b/16c", u"Trace 태그 14 @+0xb1 (can_fight 경로)"),
 ("consts[15]", "16b/16c", u"Trace +0x78 = 15000"),
 ("consts[16]", "16b", u"Trace +0x55=2 · +0x95=2 (None 니치)"),
 ("consts[17]", "14/15/16/17", u"RunAway 태그 3 @+0xb1"),
 ("consts[20]", "24", u"Attack 태그 15 (정글 hp=0 ≤ dmg=0 · 사거리 안)"),
 ("consts[21]", "24", u"Skill 태그 16"),
 ("consts[28]", "14/19", u"RunAway +0x18 end_delay=5 (L84·L126·L132)"),
 ("consts[29]", "19/20", u"배치 M RunAway 태그 3"),
 ("knobs[0]", "35/36", u"캠프 접근 반경 경계 1e10 `>` 확정"),
 ("knobs[1]", "37/38", u"부시 기준 최근접 적 반경 경계 250000 확정"),
 ("knobs[2]", "39/40", u"아군/적 집계 반경 경계 150000 확정"),
 ("knobs[5]", "24/25", u"hp=0·dmg=0 → Attack/Skill push · hp=1·dmg=0 → 없음 (≤ 등호 확정)"),
 ("knobs[6]", "01/14/16b", u"end_delay 5: AroundPosition +0x60 · RunAway +0x18 · Trace +0x80"),
 ("knobs[7]", "16b", u"Trace attack_range_margin 15000 @+0x78"),
 ("knobs[8]", "01~04/27", u"top→Morgard / bottom→Serpen 캠프 좌표 일치"),
 ("knobs[9]", "01~04", u"outline_type 1 @+0xb1"),
 ("knobs[10]", "14/16", u"RunAway with_skill +0x80 = 0"),
 ("knobs[11]", "41/42", u"L114/117 반경 경계 150000"),
 ("knobs[12]", "19/20/22", u"배치 M RunAway end_delay +0x18 = 5"),
 ("knobs[13]", "19/20/22", u"배치 M RunAway with_skill +0x80 = 0"),
]
for path, cases, txt in EV:
    p.ev(S + "/" + path, evidence=O % (cases, txt), to=2, frm=(3 if path == "consts[5]" else 4), found_by="new")

p.brief_error(u"프롬프트 ②의 `next_plan`(Option<BigPlan> 384B)·`attack`(Option<Input> 32B) 출력은 194 에 없다 — 이 함수의 출력은 sret bumpalo Vec<SmallActionPlay> 32B + &mut self 2바이트뿐(r16 루트 공용 문안이 함수별로 걸러지지 않음)")
p.brief_error(u"§4 게이트 3건(G12 consts[19] · G15 consts[2] · G16 params[1])은 전수 오탐이었다 — srclinecheck 가 vtable 슬롯 gep 오프셋을 상수 후보에서 제외(규칙②) · kindchk 가 「임계 아님」 부정문과 CMP_ORD 관측을 동시에 안고 NEG 를 냄 · paramrole.NEGATED 가 `비-readonly` 접두 부정을 못 읽음. 셋 다 게이트 쪽 수정 대상")
p.brief_error(u"프롬프트 ④ TLS 작성자 9종 중 194 루트에서 도달하는 것은 CAMP_POS_MEMO 하나뿐 — 나머지 8종(SIEGE_STANCE·CAST_BEAMS·HP_VALUE·POS_EVAL·INTER_CTX·DieTick·MAX_RANGE·LAST_STAND)은 전이 콜그래프에서 도달 불가라 「이 루트 sweep 이 r14/r15 TLS 보류 11 을 닫는다」는 기대는 194 로는 성립하지 않는다(battle 루트 cbbdb0 쪽 몫)")
p.save()
print("saved")
