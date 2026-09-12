# -*- coding: utf-8 -*-
"""15차 배치A patch.json 생성기 — mkpatch.Patch(참조구현) 경유. old 는 저장 시점에 v3 와 대조된다."""
import sys, io
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch

p = mkpatch.Patch(round=15, batch="A")

def _ev(self, path, evidence, to=2, frm=4, found_by="reused"):
    # mkpatch.Patch.ev 는 2단 경로(/specs[i]/mem[j])에서 group(3) 검사가 잘못돼 항상 거부한다(도구 결함) — 같은 계약으로 직접 만든다
    if mkpatch.locate(path) is None:
        raise AssertionError(u"그 행이 없다 — %s" % path)
    if "/mem[" in path and int(to) < 3:
        to = 3
    self.ev_up.append({"path": path, "from": int(frm), "to": int(to), "evidence": evidence, "found_by": found_by})
    return self
mkpatch.Patch.ev = _ev
O1 = u"오라클 o1.tsv(_verify15/A/oracle, 시드 game 1234·rnd 7)"
O2 = u"오라클 o2.tsv(_verify15/A/oracle, pub 래퍼 AgentVerHamster::buy_item/upgrade_item 경유 · rnd 클론으로 gen_range 까지 동일 재현)"

# ═══════════════════ #20 v27_active_objective_discipline ═══════════════════
p.fix("/specs[20]/consts[1]/meaning",
      old=u"GameMode 태그 0 = Moba. 아니면 unwrap_failed(v27_objective_entity 인라인, objective_discipline.rs:326/328)",
      new=u"GameMode 태그 0 = Moba(`get_game_mode().as_moba()` game.rs:231 인라인 → `unwrap` option.rs:1013). 아니면 unwrap_failed. v27_objective_entity 인라인 objective_discipline.rs:326/328 → 루트 490",
      evidence=u"m09.ll:19097 `%40 = icmp eq i64 %39, 0, !dbg !24953` · dloc !24953 = game.rs:231 as_moba ← objective_discipline.rs:326 v27_objective_entity ← 490 v27_active_objective_discipline. G1 은 src_line 490(루트)과 meaning 의 326 을 모순으로 봤으나 같은 사슬이다(→ 표기로 닫음)",
      behavior_change=False, found_by="reused", kind=u"보강")
p.fix("/specs[20]/logic",
      old=u"};   // moba = game.get_game_mode() 가 Moba(태그0)가 아니면 unwrap panic.",
      new=u"};   // moba = game.get_game_mode().as_moba().unwrap() — Moba(태그0)가 아니면 unwrap panic(inlinedAt: as_moba game.rs:231 + unwrap option.rs:1013).",
      evidence=u"m09.ll:19143 `unwrap_failed … !dbg !25006` · dloc !25006 = option.rs:1013 unwrap<&MobaMode> ← 326 ← 490. open[2] 「표기 불가(.moba().unwrap() vs match unreachable)」는 사슬이 `as_moba`+`unwrap` 을 직접 보여 주므로 반전",
      behavior_change=False, found_by="new", kind=u"보강")

# ═══════════════════ #21 upgrade_item ═══════════════════
p.fix("/specs[21]/consts[0]/meaning",
      old=u"두 루프(첫 매치 탐색·max fold) 모두 같은 4",
      new=u"두 사이트 모두 같은 4 — L1610 `items.iter().any(|it| it.tier() < 4)` 게이트(closure$0) 와 L1614 `filter(|it| it.tier() < 4).map(|it| it.tier()).max()`(closure$1/$2). 재현·조정 시 둘을 함께 바꿔야 한다(1차 루프는 '첫 매치 탐색'이 아니라 any() 다)",
      evidence=u"m14.ll:6652 `%28 = icmp ult i64 %27, 4, !dbg !20663` · dloc !20663 = lib.rs:1610 closure$0 ← slice/iter/macros.rs:332 any ← 1610 / m14.ll:6668 `%40 = icmp ult i64 %39, 4, !dbg !20772` · dloc !20772 = lib.rs:1614 closure$1 ← Filter::next ← Map::next ← reduce ← max_by ← max ← 1614",
      behavior_change=False, found_by="new", kind=u"실오류")
p.fix("/specs[21]/knobs[0]/where", old=u"lib.rs:1610", new=u"lib.rs:1610 / 1614",
      evidence=u"같은 임계 4 가 두 줄에 있다: 1610(any 게이트, !dbg !20663) · 1614(filter 술어, !dbg !20772). 1610 만 바꾸면 any() 와 max() 의 집합이 어긋난다",
      behavior_change=False, found_by="new", kind=u"실오류")
p.fix("/specs[21]/logic",
      old=u"// L1610 (클로저 L1614): 내 아이템 중 tier<4 인 것의 최대 tier, 없으면 0\nlet my_max_tier = player.info.items.iter().filter(|it| it.tier() < 4).map(|it| it.tier()).max().unwrap_or(0);\n   // IR: 1차 루프로 tier<4 인 첫 원소를 찾고(없으면 0), 2차 루프로 first 를 잡은 뒤 fold(max_by Ord::cmp)",
      new=u"// L1610~1614: 내 아이템 중 tier<4 인 것의 최대 tier, 없으면 0\nlet my_max_tier = if player.info.items.iter().any(|it| it.tier() < 4) {            // L1610 closure$0 (slice::iter::any)\n  player.info.items.iter().filter(|it| it.tier() < 4).map(|it| it.tier()).max().unwrap_or(0)   // L1614 closure$1(filter)·closure$2(map)·max_by fold\n} else { 0 };\n   // IR: 1차 루프 = any()(m14.ll:17~20 블록, !dbg 1610), 2차 루프 = Filter::next 로 first 를 잡은 뒤 fold(max_by Ord::cmp, !dbg 1614). max 가 None 이면 phi 0(unwrap_failed 호출 없음 = unwrap() 아님)",
      evidence=u"dloc !20663/!20772/!20818 (위 consts[0] 근거) · m14.ll:6675 `%50 = phi i64 [ 0, %29 ], [ %48, %41 ], [ 0, %17 ]` · 본문에 unwrap_failed 없음. open[0] 「표기 불가」는 any+filter/map/max 구조까지 반전(unwrap_or(0) vs if-let 만 남음)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[21]/sig/params[4]/role",
      old=u"readnone — 본문에서 전혀 안 씀",
      new=u"readnone — 본문에서 전혀 안 씀. `&dyn AbstractGame` 팻포인터가 ABI 에서 (data, vtable) 2행으로 갈라진 것 — tcx 소스 인자 5 ↔ define 7행(sret+dyn 분할)이 정상이다. pub 래퍼 AgentVerHamster::upgrade_item(lib.rs:1193, m14.ll:35109)은 이 자리에 `ptr nonnull poison`, version 에 `i64 poison` 을 넘긴다(컴파일러가 미사용을 증명)",
      evidence=u"m14.ll:6620 define 인자 7개(`%4 = ptr nonnull readnone`, `%5 = ptr noalias readonly`) · m14.ll:35124 `tail call … @…12upgrade_item(… i64 poison, … ptr nonnull poison, …)`. G16(P3)·G5 는 `&dyn` 2분할을 안 세는 오탐. ⚠별개로 sig.tcx/vis/path 가 동명 트레이트 메서드(lib.rs:440, pub)를 가리키는 것은 mkspec3 conv() 의 leaf 매칭 오류(파생 필드라 여기서 못 고침 — REPORT 참조)",
      behavior_change=False, found_by="new", kind=u"오탐")

# ═══════════════════ #22 can_tower_focused ═══════════════════
p.fix("/specs[22]/consts[6]/value", old=1, new=-1,
      evidence=u"m07.ll:49255 `%106 = add i64 %105, -1, !dbg !61386`(L25 경로) · m07.ll:49375 `%193 = add i64 %192, -1, !dbg !61455`(L40 경로). 본문에 리터럴 1 은 없고(-1 로 접힘) G12 가 그래서 25 에서 1 을 못 찾았다",
      behavior_change=False, found_by="reused", kind=u"실오류")
p.fix("/specs[22]/consts[6]/meaning",
      old=u"(level - 1) — `add i64 %level, -1` 로 나타남(IR 리터럴 -1)",
      new=u"(level − 1)·growth_range 의 오프셋가감 — 소스 `- 1` 이 IR 리터럴 `add i64 %level, -1` 로 접힘(effect.rs:26 Effect::range 인라인 ← 25/40). 별개로 consts[3] 의 -1 은 casting 니치 센티널",
      evidence=u"위 value 근거와 동일 · dloc !61386 = effect.rs:26 range ← tower_discipline.rs:25 ← 52",
      behavior_change=False, found_by="reused", kind=u"실오류")
p.fix("/specs[22]/consts[7]/src_line", old=21, new=17,
      evidence=u"m07.ll:49026 `%38 = sub nuw nsw i64 1, %25, !dbg !61248` · dloc !61248 = tower_discipline.rs:17 closure$0 ← 52. L21 의 Option Some 은 리터럴 없이 `%67 = trunc nuw i64 %66 to i1, !dbg !61331` 로 나타나므로 상수 행이 될 수 없다(G12 실제 후보 [17,52] 그대로)",
      behavior_change=False, found_by="reused", kind=u"실오류")
p.fix("/specs[22]/consts[7]/meaning",
      old=u"nearest_enemy Option 태그 1 = Some (trunc i64→i1)",
      new=u"적팀 인덱스 = 1 − player.info.team (`sub nuw nsw i64 1, %team`, towers(1-team) 인자) — 2팀 전제. (nearest_enemy Some 태그 1 은 IR 에 리터럴 없이 `trunc i64→i1` 로만 나타나 상수 행에서 뺐다 — mem[10] 참조)",
      evidence=u"위 src_line 근거와 동일. 상수 1 의 실제 소비처는 L17 뿐이다",
      behavior_change=False, found_by="reused", kind=u"실오류")
p.fix("/specs[22]/consts[2]/meaning",
      old=u"Effect::range(effect.rs:26) 인라인 — 사거리 = range + 15000 + stat_buff_cached.range + (level-1)*growth_range. L40 경로에도 동일",
      new=u"game_ai 쪽 리터럴(tower_discipline.rs:25/40 두 사이트) — 사거리 = Effect::range(t) + 15000, 여기서 Effect::range(effect.rs:26) = range + stat_buff_cached.range + (level-1)*growth_range 이고 **15000 을 포함하지 않는다**(_gcbc g06.ll:51634 is_in_range_ex 가 같은 effect.rs:26 을 인라인하는데 15000 이 없고 g06.ll 전체에 15000 이 0건). IR `!dbg` 가 effect.rs:26 에 귀속되는 것은 add 재결합 아티팩트. L40 경로에도 동일. 오라클 o1 CTF: 경계 R=range(t)+15000+t.radius+champ.radius 에서 R 참/R+1 거짓, 대립가설(15000 없음) 24행 기각",
      evidence=u"m07.ll:49302 `%166 = add i64 %101, 15000, !dbg !61386` · m07.ll:49394 `%222 = add i64 %188, 15000, !dbg !61455` · _gcbc/g06.ll:51634~51795 is_in_range_ex: `%76 = add i64 %19, %13`(sb_range+range) `%77 = add i64 %76, %75`((level-1)*growth) `%78 = add i64 %77, %31`(range_adjust vtable) — 15000 없음 · " + O1 + u" CTF 48행 MATCH(판별 24) · CTF2 24행 MATCH",
      behavior_change=True, found_by="new", kind=u"실오류")   # Effect::range 에 15000 을 넣어 재구현하면 L28 미니언 카운트(is_in_range)가 틀린다
p.fix("/specs[22]/knobs[1]/where", old=u"effect.rs:26 (인라인, tower_discipline.rs:25/40)", new=u"tower_discipline.rs:25 / 40",
      evidence=u"위 consts[2] 근거. 15000 은 game_core 가 아니라 game_ai 소스의 리터럴(두 줄 길이 68 동일 = 같은 문장)",
      behavior_change=False, found_by="new", kind=u"실오류")
p.fix("/specs[22]/knobs[1]/effect",
      old=u"game_core Effect::range 공용 상수 — 이 함수만 바꿀 수 없다(모든 이펙트 사거리에 공통). 여기서 바꾸려면 재현 시 별도 상수로 분리",
      new=u"이 함수 전용 리터럴(L25·L40 두 사이트를 함께 바꿔야 한다). Effect::range 자체엔 15000 이 없어 다른 이펙트 사거리·미니언 카운트 술어(is_in_range)에는 영향 없음. 올리면 타워를 더 멀리서부터 위험으로 봐서 사거리 밖에서도 접근을 꺼린다, 내리면 타워 경계 가까이까지 안전으로 본다",
      evidence=u"판정 반전(공용 상수 → 함수 전용 리터럴). 근거 = consts[2] 와 동일",
      behavior_change=False, found_by="new", kind=u"실오류")
p.fix("/specs[22]/logic",
      old=u"let range = t.attack_effect.unwrap().range(t);        // = range + 15000 + stat_buff_cached.range + (level-1)*growth_range",
      new=u"let range = t.attack_effect.unwrap().range(t) + 15000;   // Effect::range = range + stat_buff_cached.range + (level-1)*growth_range (15000 은 이 함수의 리터럴)",
      evidence=u"consts[2] 근거", behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[22]/logic",
      old=u"// L40\n    let range = t.attack_effect.unwrap().range(t);",
      new=u"// L40\n    let range = t.attack_effect.unwrap().range(t) + 15000;",
      evidence=u"m07.ll:49394 `%222 = add i64 %188, 15000, !dbg !61455`(L40)", behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[22]/logic",
      old=u"// 타워 사거리 안 내 미니언 수(aux m06/m11)",
      new=u"// 타워 사거리 안 내 미니언 수(aux m06/m11). ⚠is_in_range(effect.rs:63→is_in_range_ex:79~80, _gcbc g06.ll:51634)는 L34 와 다른 식: dist² <= (Effect::range(t) + ty.range_adjust(vtable+0xe8) + [t.radius() if casting==Targeting(0)] + m.radius())² — 15000 없음",
      evidence=u"_gcbc/g06.ll:51634~51795: `%29 = gep %22, 232`(EffectType vtable+0xe8 range_adjust) `%33 = load i32 %0+48`(casting@0x30) `%34 = icmp eq %33, 0` → caster radius phi[0 | radius] · `%85 = icmp ule i64 %83, %84`. open[3] 미탐색 → 사실 서술로",
      behavior_change=True, found_by="new", kind=u"보강")   # 명세대로 L34 식을 is_in_range 에 그대로 쓰면 미니언 카운트가 틀린다
for j in range(5):
    p.fix("/specs[22]/sig/params[%d]/i" % j, old=j, new=j + 1,
          evidence=u"G16(P3) 규약: sret 없음 → 소스 인자 1..5. define m07.ll:48968 는 bool 반환(레지스터)이라 sret 행이 없다",
          behavior_change=False, found_by="reused", kind=u"실오류")

# ═══════════════════ #23 buy_item ═══════════════════
p.fix("/specs[23]/sig/params[3]/role",
      old=u"readnone — 안 씀",
      new=u"readnone — 안 씀. `&dyn AbstractGame` 팻포인터가 ABI 에서 (data, vtable) 2행으로 갈라진 것 — tcx 소스 인자 5 ↔ define 6행(반환은 {i64,i64} 레지스터라 sret 없음)이 정상. pub 래퍼 AgentVerHamster::buy_item(lib.rs:1173, m14.ll:38174)은 `player.info.item_builds.len()==0`(PlayerState+0x4f0)일 때만 이 레거시 함수를 부르고(아니면 item_v26 경로) 이 자리에 `ptr nonnull poison`, version 에 `i64 poison` 을 넘긴다",
      evidence=u"m14.ll:8267 `define hidden { i64, i64 } @…8buy_item(i64 %0, ptr %1, ptr %2, ptr nonnull readnone %3, ptr %4, ptr %5)` · m14.ll:38176~38188 `%8 = gep %2, 1264` `%11 = icmp eq i64 %9, 0` → `tail call … @…8buy_item(i64 poison, …, ptr nonnull poison, …)` · tcxdict PlayerState 0x4f0 = info.item_builds.len. G16(P3)·G5 는 `&dyn` 2분할을 안 세는 오탐. ⚠sig.tcx/vis/path 가 트레이트 메서드(lib.rs:437)를 가리키는 것은 mkspec3 conv() leaf 매칭 오류(파생 필드)",
      behavior_change=False, found_by="new", kind=u"오탐")
p.fix("/specs[23]/logic",
      old=u"let has_low_tier_item = player.info.items.iter().any(|x| x.tier() < 4);   // vtable+0x70",
      new=u"let has_low_tier_item = player.info.items.iter().any(|x| x.tier() < 4);   // vtable+0x70 — inlinedAt 사슬이 slice::iter::any(closure$0 L1480)를 직접 보인다(find().is_some() 아님)",
      evidence=u"m14.ll:8290 `%27 = icmp ult i64 %26, 4, !dbg !23224` · dloc !23224 = lib.rs:1480 closure$0 ← library/core/src/slice/iter/macros.rs:332 any<…buy_item::closure_env$0> ← 1480. open[0] 「표기 불가」 반전",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[23]/logic",
      old=u"// L1536 (stat() 재호출)",
      new=u"// L1536 (stat() 재호출 — 별개 sret alloca %9/%8 로 vtable+0x30 을 두 번 부른다. 지역변수 재사용이었다면 불투명 vtable 호출은 CSE 되지 않으므로 호출이 1회였을 것 ⟹ 소스가 stat() 을 두 번 쓴 것은 사실)",
      evidence=u"m14.ll:8482 `invoke void %131(ptr … sret([72 x i8]) … %9, …) !dbg !23406`(1523) · m14.ll:8512 `invoke void %131(… %8 …) !dbg !23427`(1536). open[1] 「표기 불가」 → 사실 서술",
      behavior_change=False, found_by="new", kind=u"보강")

# ═══════════════════ #24 v23_healthy_allies_near_point ═══════════════════
for j in range(6):
    p.fix("/specs[24]/sig/params[%d]/i" % j, old=j, new=j + 1,
          evidence=u"G16(P3) 규약: sret 없음(usize 반환, `define noundef range(i64 0, 6) i64`) → 소스 인자 1..6",
          behavior_change=False, found_by="reused", kind=u"실오류")

# ═══════════════════ ev_up ═══════════════════
# #20 — tcx + 오라클
TCX20 = u"tcxdict 대조: TeamPlan 0x130 objective_discipline@Some.0.wait_pos.0 / 0x149 objective_discipline@tag(Niche)=kind@tag · ObjectiveDisciplineState 32B {wait_pos.0 0x0, wait_pos.1 0x8, until_tick 0x10, target@tag 0x18, kind@tag 0x19} · MobaMode 0x1a0 epic.live_list.buf.ptr / 0x1d8 serpen.live_list.len · Entity 0x628 stat_cached.hp / 0x670 hp · GameMode 16B tag@0 Moba=0 payload@0x8 · " + O1 + u" V27 16/16 MATCH"
for j in (0, 1, 2, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17):  # 3·7 은 이미 ev3
    p.ev("/specs[20]/mem[%d]" % j, evidence=TCX20, to=3, found_by="reused")
p.ev("/specs[20]/consts[0]", evidence=O1 + u" V27 kind=2 → None 1/1", to=2, found_by="reused")
p.ev("/specs[20]/consts[2]", evidence=O1 + u" V27_HP Morgard 5/5(epic.live_list 에 엔티티 id 주입)", to=2, found_by="reused")
p.ev("/specs[20]/consts[3]", evidence=O1 + u" V27_HP_SERPEN 5/5(serpen 리스트 비어 있어 Some)", to=2, found_by="reused")
p.ev("/specs[20]/consts[4]", evidence=O1 + u" V27_HP hp*100/max: 359/1000→None · 360/1000→Some", to=2, found_by="reused")
p.ev("/specs[20]/consts[5]", evidence=O1 + u" V27_HP 임계 36 경계 판별(35.9%→None, 36.0%→Some)", to=2, found_by="reused")
p.ev("/specs[20]/knobs[0]", evidence=O1 + u" V27_HP 36 경계 판별", to=2, found_by="reused")
p.ev("/specs[20]/knobs[1]", evidence=O1 + u" V27 until==tick → None · until=tick+1 → Some (strict ugt 판별)", to=2, found_by="reused")

# #21
TCX21 = u"tcxdict: GameContext 0x0 pool / 0x30 item_list · PlayerState 0x4a0 info.items.buf.ptr / 0x4a8 len / 0x998 info.gold · divtable ItemInfo 0x50 is_active / 0x68 price / 0x70 tier / 0x80 next_tier(g02.ll 81%) · " + O2 + u" UPG 50/50 MATCH"
for j in [k for k in range(22) if k != 7]:  # 7 은 이미 ev3
    p.ev("/specs[21]/mem[%d]" % j, evidence=TCX21, to=3, found_by="reused")
p.ev("/specs[21]/consts[0]", evidence=O2 + u" owned=[t3]→my_max=3(t4 후보) vs owned=[t4]→my_max=0(t5 후보): tier<4 필터 판별", to=2, found_by="reused")
p.ev("/specs[21]/consts[1]", evidence=O2 + u" owned=[t4] → my_max 0 → (0,13) Some", to=2, found_by="reused")
p.ev("/specs[21]/consts[2]", evidence=O2 + u" 후보 0 → None 22행", to=2, found_by="reused")
p.ev("/specs[21]/consts[3]", evidence=O2 + u" Some((i,idx)) 28행 튜플 동일", to=2, found_by="reused")
p.ev("/specs[21]/consts[4]", evidence=O2 + u" rnd 클론 gen_range(0..len) 으로 게임과 같은 원소 선택(3후보 케이스 포함)", to=2, found_by="reused")
p.ev("/specs[21]/knobs[0]", evidence=O2 + u" tier<4 필터 판별(위 consts[0])", to=2, found_by="reused")
p.ev("/specs[21]/knobs[1]", evidence=O2 + u" owned=[ad1,ad2] my_max=2: ad2(tier2) 후보 제외 → None (strict > 판별)", to=2, found_by="reused")
p.ev("/specs[21]/knobs[2]", evidence=O2 + u" gold 1199/1200 vs price 1200: 1199→None, 1200→Some (price<=gold 경계)", to=2, found_by="reused")
p.ev("/specs[21]/knobs[3]", evidence=O2 + u" rnd 클론 균등 gen_range 로 동일 선택", to=2, found_by="reused")

# #22
TCX22 = u"tcxdict: GameContext 0x0/0x8 · GameSetting 0x13f8 tower_attack_disable_tick · AbstractGameWithCache 0x0 game / 0x1e0 player_champion · PlayerState 0x930 info.team / 0x9c0 info.position@tag · Entity 0x68 ty@tag / 0x88 ty@Tower.info.nearest_enemy@tag / 0x98 nearest_enemy@Some.0.1 / 0x438 stat_buff_cached.range / 0x470 radius_mult(i32) / 0x490 attack_effect@Some.0 / 0x4a0 range / 0x4a8 growth_range / 0x4c0 attack_effect@tag(Niche)=casting@tag / 0x5c0 id / 0x5c8 level / 0x660 x / 0x668 y / 0x680 radius · " + O1 + u" CTF 72행+CTF2 24행+CTF_NE 6행 MATCH"
for j in range(23):
    p.ev("/specs[22]/mem[%d]" % j, evidence=TCX22, to=3, found_by="reused")
p.ev("/specs[22]/consts[0]", evidence=O1 + u" cache.towers(1) 9원소 전부 tag 2 경로 실행", to=2, found_by="reused")
p.ev("/specs[22]/consts[1]", evidence=O1 + u" CTF_NE case=other(nearest_enemy=적 챔프): 미니언 0 < 2 → 사거리식 적용 R 참/R+1 거짓", to=2, found_by="reused")
p.ev("/specs[22]/consts[2]", evidence=O1 + u" CTF 대립가설(15000 없음) 24행 기각 · CTF2 range/growth/level/sb_range 주입 후 R 참/R+1 거짓 24행", to=2, found_by="new")
p.ev("/specs[22]/consts[4]", evidence=O1 + u" CTF2 champ radius_mult=50 / tower 20: radius*(mult+100)/100 로 계산한 R 이 경계와 일치", to=2, found_by="reused")
p.ev("/specs[22]/consts[5]", evidence=O1 + u" CTF radius_mult=0 경로(1차)와 ≠0 경로(2차) 둘 다 경계 일치", to=2, found_by="reused")
p.ev("/specs[22]/consts[6]", evidence=O1 + u" CTF2 level=3·growth=700 → (level-1)*growth=1400 반영된 R 이 경계와 일치", to=2, found_by="reused")
p.ev("/specs[22]/consts[7]", evidence=O1 + u" towers(1 - team) — team 0 플레이어로 적 타워 9개(팀1) 획득", to=2, found_by="reused")
p.ev("/specs[22]/knobs[0]", evidence=O1 + u" CTF_NE case=other 에서 cnt(0)<2 경로 실행 확인(미니언 없는 세계 — 임계값 자체의 경계는 미니언 스폰 필요·미측정)", to=3, found_by="reused")
p.ev("/specs[22]/knobs[1]", evidence=O1 + u" CTF 판별 24행", to=2, found_by="new")
p.ev("/specs[22]/knobs[2]", evidence=O1 + u" CTF_GATE tower_attack_disable_tick=0 → 타워 위치에서도 false", to=2, found_by="reused")
p.ev("/specs[22]/knobs[3]", evidence=O1 + u" CTF/CTF2 경계 = range+15000+t.radius()+champ.radius() 정확 일치(R 참·R+1 거짓 48행)", to=2, found_by="reused")

# #23
TCX23 = u"tcxdict: PlayerState 0x4a0/0x4a8 info.items · 0x510 info.champion.ptr / 0x518 vtable · 0x998 info.gold · GameContext 0x0/0x30 · divtable ItemInfo 0x50/0x68/0x70 · EntityStat.hp = 필드 3번째(+0x10) · " + O2 + u" BUY 540/540 MATCH"
for j in [k for k in range(20) if k != 3]:  # 3 은 이미 ev3
    p.ev("/specs[23]/mem[%d]" % j, evidence=TCX23, to=3, found_by="reused")
EV23 = {
    0: u"owned=[ad1](tier1)·[t3](tier3) → None, [t4]·[t4,t5] → 진행 (tier<4 판별)",
    1: u"owned=[t4,t5,t4](3개) → None, [t4,t5](2개) → Some (len>2 판별)",
    2: u"tier0 만 후보(ad1/ad2/t4/t5 후보 0회) — 카테고리 카운트 일치",
    3: u"Defense(2) 가 is_defensive 후보에 포함(cands [2,3,5])",
    4: u"MagicResistance(3) 포함(cands [2,3,5])",
    5: u"Hp(5) 포함(cands [2,3,5])",
    6: u"Magician/Util 저HP 에서 cands [4] 만(Magic)",
    7: u"공격형 cands [0,1](AD·AttackSpeed) — Support(6) 는 어디에도 없음",
    8: u"cat 0 Melee 분기 실행", 9: u"cat 1 Range 분기(cands [0,1])", 10: u"cat 2 Magician 분기(cands [4])",
    11: u"cat 3 Util 분기", 12: u"cat 4 Assassin 분기",
    13: u"Melee n=0: hp 1549→[0,1] / 1550→[2,3,5] (경계 판별)",
    14: u"Melee n=2: hp 1799→[0,1] / 1800→[2,3,5] (경계 판별)",
    15: u"Melee 중HP: n=2 만 공격형, n=1(owned=[t4]) 은 방어형",
    16: u"Melee 저HP·Assassin: n∈{0,2} 공격형 / n=1 방어형",
    17: u"None 반환 케이스 다수(gold=50 등)", 18: u"Some(idx) 반환 케이스 다수",
    19: u"rnd 클론 gen_range(0..len) 으로 같은 원소 선택(cands 3개 케이스 포함)",
}
for j, e in EV23.items():
    p.ev("/specs[23]/consts[%d]" % j, evidence=O2 + u" " + e, to=2, found_by="reused")
EVK23 = {0: EV23[0], 1: EV23[1], 2: EV23[13] + u" · " + EV23[14], 3: u"Util: hp 1549→[4] / 1550→[2,3,5] (경계 판별)", 4: EV23[7], 5: EV23[19]}
for j, e in EVK23.items():
    p.ev("/specs[23]/knobs[%d]" % j, evidence=O2 + u" " + e, to=2, found_by="reused")

# #24
TCX24 = u"tcxdict: PlayerState 0x930 info.team · AbstractGameWithCache 0x1e0 player_champion[2][5] · Entity 0x628 stat_cached.hp / 0x670 hp / 0x660 x / 0x668 y · " + O1 + u" V23 150/150 + V23HP 6/6 MATCH"
for j in range(7):
    p.ev("/specs[24]/mem[%d]" % j, evidence=TCX24, to=3, found_by="reused")
p.ev("/specs[24]/consts[0]", evidence=O1 + u" V23HP: 아군 1명 hp 360/1000 → min_hp 36 포함·37 제외 (hp*100/max >= min 판별)", to=2, found_by="reused")
p.ev("/specs[24]/consts[2]", evidence=O1 + u" range=0·min_hp=101 → 0 (count init)", to=2, found_by="reused")
p.ev("/specs[24]/knobs[1]", evidence=O1 + u" V23 각 아군 거리 d 에 대해 range=d 포함·d-1 제외 — 'MATCH(판별 ule)' 행으로 <= 확정", to=2, found_by="reused")

# ═══════════════════ brief_errors ═══════════════════
p.brief_error(u"§1 표의 vis 가 21 upgrade_item·23 buy_item 을 `pub` 으로 적었으나 IR 심볼(`_RNv…12upgrade_item` m14.ll:6620 / `…8buy_item` m14.ll:8267)은 `define hidden` 자유함수 `game_ai::upgrade_item`/`game_ai::buy_item`(tcx v=in:game_ai, lib.rs:1603/1477)이다. pub 인 것은 동명 트레이트 메서드(lib.rs:440/437)와 inherent 래퍼(lib.rs:1193/1173)뿐 — 오라클은 그 래퍼로 진입했다. 원인 = mkspec3 conv() L512~516 이 leaf 이름+파일명으로 첫 tcx 항목을 고르고 src_line 을 안 본다(sig.tcx/vis/path 가 전부 트레이트 메서드 것으로 파생됨).")
p.brief_error(u"§4 G16(P3) 「params 행 수 ≠ tcx 소스 인자 수」(21·23) 는 `&dyn Trait` 팻포인터가 define 에서 (data, vtable) 2행으로 갈라지는 것을 안 센 오탐이다. 검사기는 sig.tcx 의 `&dyn`/`&[T]`/`&str` 마다 +1 을 더해야 한다.")
p.brief_error(u"§4 G12 22 consts[6] 「src_line=25 인데 사슬에 없다」는 명세 value 가 1 인데 IR 리터럴이 -1 이라 생긴 것이고(값 오류), consts[7] 는 실제로 L17 의 `1 - team` 이었다 — 「실제 후보 [17,52]」가 정답이었다. 게이트 자체는 옳았다.")
p.brief_error(u"§0 신선도: 작업 중(08:31) 배치 B/C/D/M 패치 적용으로 v3 해시가 바뀌어 dossierfresh 가 STALE 을 냈다. 담당 20~24 는 이름 기준 대조에서 `exe` 블록만 변경(정정 대상 필드 불변)이라 계속 진행했다 — 도구가 「내 담당 spec 이 바뀌었나」를 함수 단위로 알려 주면 4배치 동시 실행에서 불필요한 재독을 막을 수 있다.")
p.brief_error(u"§2 TEMPLATE.rs 는 `default 챔프 stat_cached.hp == 1`(move_speed==1 과 같은 함정) 을 안 적었다 — hp 백분율 임계(36·min_hp)는 1000 등 실값을 주입해야 경계가 갈린다(1차 실행에서 36% 가 0/1 로 붕괴해 재측정).")

p.save()
