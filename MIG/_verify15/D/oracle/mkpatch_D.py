# -*- coding: utf-8 -*-
"""15차 배치 D patch.json 생성 — mkpatch 참조구현 사용. 실행: python -X utf8 _verify15/D/oracle/mkpatch_D.py"""
import sys, io, json
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch

p = mkpatch.Patch(round=15, batch="D")

# ★mkpatch.Patch.ev 는 `/specs[i]/mem[j]` 처럼 바깥 컨테이너 없는 배열 경로를 거부한다
#   (PATH group(3) 은 「중첩 필드」인데 그걸 「배열 원소」 판정에 쓴다 — mkpatch.py:134). 우회: 같은 계약으로 직접 넣는다.
def EV(path, to, evidence, found_by="reused", frm=4):
    m = mkpatch.PATH.match(path)
    assert m and m.group(4) is not None, path
    assert mkpatch.locate(path) is not None, path
    if (m.group(3) or m.group(2)) == "mem" and int(to) < 3:
        to = 3
    p.ev_up.append({"path": path, "from": int(frm), "to": int(to), "evidence": evidence, "found_by": found_by})
O1 = u"_verify15/D/oracle/D15_o1.rs 실행(케이스별 프로세스 1개 · 출력 D15_o1_case{0..7}.out)"

# ─────────────────────────────── 35 resolve_fight_stake ───────────────────────────────
# G13 knobs[0]
p.fix("/specs[35]/knobs[0]/where", old=u"IR m10.ll:42385 `icmp ult i64 %1, 2`", new=u"IR m10.ll:42389 `icmp ult i64 %1, 2`",
      evidence=u"m10.ll:42389 `%23 = icmp ult i64 %1, 2, !dbg !48118` (!48118 = fight_model.rs:574). 42385 는 `#dbg_value` 줄", found_by="reused")
# knobs[1] — G13 이 안 본 행(인용이 opcode 가 아니라 `i8 0`)
p.fix("/specs[35]/knobs[1]/where", old=u"IR m10.ll:42480 `i8 0`", new=u"IR m10.ll:42508 abandon 호출의 9번째 인자 `i8 0`",
      evidence=u"m10.ll:42508 `invoke fastcc void @…resolve_fight_full(ptr %19, i64 %52, ptr %3, ptr %5, ptr %51, i64 %44, ptr %8, i64 %9, i8 0, …)` — 42480 은 `to label %49 unwind label %47` 줄이고 ±2 안에 `i8 0` 이 없다", found_by="new")
# knobs[2] — 키가 (dist², id) 튜플
p.fix("/specs[35]/knobs[2]/where", old=u"IR m10.ll:42640~42660 min_by_key", new=u"IR m10.ll:42646~42660 첫 원소 키 (dist²@%99, id@%101) · 42666 min_by_key fold invoke · 42688 phi",
      evidence=u"m10.ll:42658 `%99 = add i64 %98, %97`(dist²) · 42660 `%101 = load i64, ptr %100`(+0x5c0 id) · 42666 `invoke void @…Map<Iter<&Entity>, min_by_key::key<&&Entity, (u64,usize), resolve_fight_stake::{closure#1}>>::fold` — 심볼 `3keyRB1n_TyjE` 의 `TyjE` = 튜플 (u64, usize)", found_by="new")
p.fix("/specs[35]/knobs[2]/value", old=u"distance_sq 최소", new=u"(distance_sq, id) 사전순 최소 — 동거리면 id 작은 쪽",
      evidence=u"m10.ll:42666 심볼 `…min_by_key3keyRB1n_TyjE…` = key<T=&&Entity, K=(u64,usize), F=closure#1> ⟹ 클로저#1 의 반환 = (u64,usize). 42661/42663/42665 `store i64 %99, ptr %15` / `store i64 %101, ptr %102(=%15+8)` / `store ptr %70, ptr %103(=%15+16)`(누산기 = ((dist², id), &elem)). 줄 길이 산술: fight_model.rs:597 = 99자, tcx closure#1 c=52 · closure#2 c=90 ⟹ 사이 35자 = ` (a.distance_sq(champ), a.id)).map(` ±0", behavior_change=True, found_by="new")
p.fix("/specs[35]/logic",
      old=u".min_by_key(|a| (a.x(+0x660).abs_diff(champ.x))^2 + (a.y(+0x668).abs_diff(champ.y))^2)\n             .map(|a| a.id(+0x5c0))              // 나와 가장 가까운 묶인 아군 = 구조 대상",
      new=u".min_by_key(|a| ( (a.x(+0x660).abs_diff(champ.x))^2 + (a.y(+0x668).abs_diff(champ.y))^2, a.id(+0x5c0) ))   // ★키 = (dist², id) 튜플 — 동거리면 id 작은 쪽\n             .map(|a| a.id(+0x5c0))              // 나와 가장 가까운 묶인 아군 = 구조 대상",
      evidence=u"m10.ll:42666 `min_by_key::key<&&Entity, (u64, usize), {closure#1}>` + m10.ll:42661/42663/42665 누산기 store(dist²@+0, id@+8, &elem@+16). 명세대로 dist² 단독 키로 재구현하면 동거리 시 `min_by_key` 규칙(첫 최소)이 적용돼 슬라이스 앞쪽 아군이 뽑히지만 실제는 id 최소", behavior_change=True, found_by="new")
p.fix("/specs[35]/mem[12]/note", old=u"L597. 거리는 dx²+dy² (u64 abs_diff 제곱합)", new=u"L597. 키 = (dx²+dy² (u64 abs_diff 제곱합), id) 튜플 사전순 최소 — 동거리면 id 작은 아군",
      evidence=u"m10.ll:42666 `min_by_key3keyRB1n_TyjE`(K=(u64,usize)) · 42661/42663/42665 누산기 store", behavior_change=True, found_by="new")
# G16 P1 — role 에 IR 레지스터 대응을 적는다(슬라이스 2개는 SROA 로 ptr/len 두 레지스터)
for j, reg, pre in [
    (0, u"%1", u"IR %1(진입부에서 무조건 8B alloca %22 에 spill, m10.ll:42366 `store i64 %1, ptr %22`). "),
    (1, u"%2", u"IR %2. "), (2, u"%3", u"IR %3. "), (3, u"%4", u"IR %4. "), (4, u"%5", u"IR %5. "),
    (5, u"%6/%7", u"IR %6(ptr)/%7(len) — `&[&Entity]` 가 SROA 로 두 레지스터. "),
    (6, u"%8/%9", u"IR %8(ptr)/%9(len) — `&[&Entity]` 가 SROA 로 두 레지스터. "),
    (7, u"%10", u"IR %10(i8). "), (8, u"%11", u"IR %11(dereferenceable_or_null(1728) = Option<&Entity> 니치 null). "),
    (9, u"%12", u"IR %12. "), (10, u"%13", u"IR %13. ")]:
    cur = mkpatch.locate(u"/specs[35]/sig/params[%d]/role" % j)
    head = cur[:24]
    p.fix(u"/specs[35]/sig/params[%d]/role" % j, old=head, new=pre + head,
          evidence=u"m10.ll:42357 define 인자 순서 `(sret %0, i64 %1, ptr align16 deref(320) %2, ptr deref(24) %3, ptr deref(2528) %4, ptr deref(1728) %5, ptr %6, i64 %7, ptr %8, i64 %9, i8 %10, ptr deref_or_null(1728) %11, i64 %12, ptr deref(224) %13)` — G16 P1 규약(승격 대응 `%k` 명기)", found_by="reused")
# G16 P2 — sret 행 삽입(8차 확정 규약). ⚠role 정정 뒤에 넣는다(위 경로는 삽입 전 인덱스)
p.errors.append({"op": "insert", "path": "/specs[35]/sig/params", "at": 0, "guard": "(sret)", "guard_key": "name",
                 "new": {"i": 0, "name": "(sret)", "type": "&mut FightPrediction(64B)",
                         "note": u"IR %0 반환값 out-ptr. m10.ll:42357 `ptr dead_on_unwind noalias noundef writable sret([64 x i8]) align 8 captures(none) dereferenceable(64) %0`. 경로 A(42459)·B(42445) 는 resolve_fight_full 이 여기에 직접 쓰고(absolute), 경로 C 는 diff(%18) 를 42539 `llvm.memcpy(%0, %18, 64)` 로 복사한다. tcx 정본 대조: 소스 인자 11개 + sret = IR 인자 14개(슬라이스 2개 ×2)"},
                 "kind": u"실오류", "evidence": u"G16 P2 — 8차 확정 규약(sret 를 params[0], i=0). m10.ll:42357 define 첫 인자가 sret 인데 params[0] 이 version 이었다",
                 "behavior_change": False, "found_by": "reused"})
# G10 — open[1]·open[3] 의 사실 서술 꼬리를 물음으로 바꾼다(사실은 산문 보고 → notes)
p.errors.append({"path": "/specs[35]/open", "kind": u"실오류",
                 "old": u"resolve_fight_full(m10.ll:39915) 내부에서 baseline 이 net_value/line 에 어떻게 반영되는지는 안 읽었다 — 여기선 '기준선 인자로 abandon.net_value 를 넘긴다' 는 사실만 확정",
                 "new": u"resolve_fight_full(m10.ll:39915) 내부에서 baseline 인자(마지막 i64)가 net_value/line 산출에 어떻게 반영되는가 — 별도 함수라 미탐색(재료 = m10.ll:39915 define 독해)",
                 "evidence": u"G10 class 오분류. 사실('기준선 인자로 abandon.net_value(+0x30)를 넘긴다' — m10.ll:42516 `%55 = gep %19, 48` · 42517 `%56 = load i64, ptr %55` → 42519 diff 호출의 마지막 인자 `i64 %56`)은 notes 로 옮긴다(산문 보고)",
                 "behavior_change": False, "found_by": "reused"})
p.errors.append({"path": "/specs[35]/open", "kind": u"실오류",
                 "old": u"L597 의 `.map(|a| a.id)` 는 closure#2(597:90), 키 closure#1(597:52) 로 tcx 에 있으나 IR 에는 인라인돼 별도 define 이 없다(fnparts: 서브프로그램 73 vs define 5). 극성·순서는 본문 phi(%113/%114)로 확정",
                 "new": u"L597 closure#1(597:52)·closure#2(597:90) 의 소스 표기 중 `a.distance_sq(champ)` 와 `champ.distance_sq(a)` 는 같은 길이(20자)라 줄 길이 산술로 못 가른다 — 표기 불가(동작은 확정: 키 = (dist², id))",
                 "evidence": u"G10 class 오분류(사실 서술 → notes 로). 새 물음은 줄 길이 산술 잔차: fight_model.rs:597 = 99자, `          diff.rescue_ally = remaining.iter().min_by_key(|a| (a.distance_sq(champ), a.id)).map(|a| a.id);` 가 ±0 이지만 distance_sq 의 수신자 표기는 동일 길이",
                 "behavior_change": False, "found_by": "new"})
# mem tcx 대조 → ev3
for j, ev in [(0, u"tcxdict OperationData 0x8 → context &GameContext"), (1, u"tcxdict GameContext 0x0 → pool &bumpalo::Bump"),
              (2, u"tcxdict Entity 0x5c0 → id usize"), (3, u"tcxdict Entity 0x660 → x u64"), (4, u"tcxdict Entity 0x668 → y u64"),
              (7, u"tcxdict FightPrediction 0x30 → net_value i64"), (8, u"tcxdict FightPrediction 0x38 → line@tag 1B"),
              (9, u"tcxdict FightPrediction 0x38 → line@tag 1B"), (10, u"tcxdict FightPrediction 0x39 → line_absolute@tag 1B"),
              (11, u"tcxdict FightPrediction 0x20 → rescue_ally@tag 8B"), (12, u"tcxdict FightPrediction 0x28 → rescue_ally@Some.0 usize")]:
    EV(u"/specs[35]/mem[%d]" % j, to=3, evidence=u"tcx 정본 대조: " + ev + u" (_verify15/D/oracle/tcx_mem_check.txt)", found_by="reused")

# ─────────────────────────────── 36 calculate_nexus_defense_count ───────────────────────────────
p.fix("/specs[36]/knobs[0]/where", old=u"IR m13.ll:56492 `icmp ult i64 %121, 22500000001`", new=u"IR m13.ll:56495 `icmp ult i64 %121, 22500000001`",
      evidence=u"m13.ll:56495 `%122 = icmp ult i64 %121, 22500000001, !dbg !58930`", found_by="reused")
p.fix("/specs[36]/knobs[1]/where", old=u"IR m13.ll:56592 `icmp ult i64 %156, 90000000001`", new=u"IR m13.ll:56625 `icmp ult i64 %156, 90000000001`",
      evidence=u"m13.ll:56625 `%157 = icmp ult i64 %156, 90000000001, !dbg !59156`", found_by="reused")
p.fix("/specs[36]/knobs[2]/where", old=u"IR m13.ll:56920 `select i1 %125, i64 2, ...`", new=u"IR m13.ll:56917 `select i1 %125, i64 2, i64 %273`",
      evidence=u"m13.ll:56917 `%274 = select i1 %125, i64 2, i64 %273, !dbg !59366` · 56919 `%276 = select i1 %53, i64 %274, i64 %275`", found_by="reused")
p.fix("/specs[36]/knobs[3]/where", old=u"IR m13.ll:56942 `icmp ugt i64 %282, 5`", new=u"IR m13.ll:56950 `icmp ugt i64 %282, 5`",
      evidence=u"m13.ll:56950 `%286 = icmp ugt i64 %282, 5, !dbg !59389` · 56953 `%289 = select i1 %286, i64 2, i64 %288` · 56959 `llvm.umax.i64(%289, %290)`", found_by="reused")
p.fix("/specs[36]/knobs[4]/where", old=u"aux m12.ll:42952 `add nsw i8 %30, -3`", new=u"aux m12.ll:42965 `add nsw i8 %30, -3`",
      evidence=u"m12.ll:42965 `%31 = add nsw i8 %30, -3, !dbg !72377` · 42966 `%32 = icmp ult i8 %31, 2` (!72377 = tower.rs:91 → inlinedAt 1313 → handler.rs:2364)", found_by="reused")
# mem[8] 오귀속 — D9-OFF: 한 오프셋 = 한 필드
p.fix("/specs[36]/mem[8]/name", old=u"top_lead/mid_lead/bottom_lead [line][team]", new=u"top_lead[team]",
      evidence=u"tcxdict AbstractGameWithCache 0x21c0 → top_lead[0] (0x21d0 mid_lead[0] · 0x21e0 bottom_lead[0]). IR m13.ll:56376 `%74 = getelementptr i64, ptr %57, i64 %52`(team*8) → 56410 `%83 = getelementptr i8, ptr %74, i64 %82`(line<<4) → 56411 `%84 = getelementptr i8, ptr %83, i64 8640` ⟹ cache+0x21c0+line*16+team*8. 오라클: cache+0x21c0+team*8 에 0 을 raw write 하자 `cache.line_lead(team, Top)` 가 0 을 돌려주고 36 의 결과가 바뀜(case4)", found_by="new")
p.fix("/specs[36]/mem[8]/note", old=u"[usize;2]×3 (line*16 + team*8).", new=u"[usize;2]×3 — line 스트라이드 16(Top 0x21c0 / Mid 0x21d0 mid_lead / Bottom 0x21e0 bottom_lead) + team*8.",
      evidence=u"tcxdict AbstractGameWithCache 0x21d0 → mid_lead[0] · 0x21e0 → bottom_lead[0]", found_by="new")
# G12 3건 — 오탐(aux 클로저 m12.ll 의 !dbg 사슬이 그 줄을 가리킨다)
p.fix("/specs[36]/consts[10]/src_line", old=2362, new=2362, kind=u"오탐",
      evidence=u"명세가 옳다. 리터럴 1 은 aux m12.ll:42919 `%11 = icmp eq i64 %10, 1, !dbg !72356` 이고 `!72356 = !DILocation(line: 2362, scope: !70898, inlinedAt: !72353)`. G12 가 aux 범위(m12.ll 42894~42979)를 안 훑는다", found_by="reused")
p.fix("/specs[36]/consts[11]/src_line", old=2364, new=2364, kind=u"오탐",
      evidence=u"명세가 옳다. aux m12.ll:42956 `switch i64 %27, label %33 [ i64 3, label %34  i64 2, label %28 ], !dbg !72374` · `!72374 = !DILocation(line: 2364, …)`. switch 케이스 줄 + aux 범위 밖 = G12 이중 사각", found_by="reused")
p.fix("/specs[36]/consts[12]/src_line", old=2364, new=2364, kind=u"오탐",
      evidence=u"명세가 옳다. 같은 switch(m12.ll:42956, !72374 = handler.rs:2364) 의 `i64 2, label %28` 케이스", found_by="reused")
# open[0] — 런타임 관측 추가(의미 체계는 여전히 미탐색)
p.errors.append({"path": "/specs[36]/open", "kind": u"보강",
                 "old": u"`lead > 2` 를 게이트로 씀(0~2+ 정수 척도로 추정)",
                 "new": u"`lead > 2` 를 게이트로 씀(0~2+ 정수 척도로 추정). 15차 오라클 실측: start_game 직후(tick 0) 양팀·3라인 전부 2 — '2 = 대등 기준선' 가설, 계산식(_gcbc g15.ll:103509~ AbstractGameWithCache::new, simulation.rs:1697~1699 top_lead/mid_lead/bottom_lead, 27칸 격자 `icmp sgt i32 …, 2`)은 미탐색",
                 "evidence": u"D15_o1_case0.out `A36 team=0 lead=[2, 2, 2]` / `team=1 lead=[2, 2, 2]`(cache.line_lead(team, Top/Mid/Bottom))", "behavior_change": False, "found_by": "new"})
# ev_up — 오라클 실행
EV("/specs[36]/consts[0]", to=2, evidence=u"오라클 실행 확인: data.context.tutorial=First(태그1) 로만 바꾼 세계(case7)에서 epic+weak 인데 결과 0 — `(tutorial-1) <u 6` 게이트가 enemy_has_epic 을 죽인다. " + O1, found_by="new")
EV("/specs[36]/consts[3]", to=2, evidence=u"오라클 실행 확인: Game+0xed00+0x240 epic_minion_buff_time[1] 만 1000 으로 raw write → team0(적=1) 만 enemy_has_epic (case4 team0=1, team1=0; case5 team1 weak 이지만 0). " + O1, found_by="new")
EV("/specs[36]/consts[5]", to=2, evidence=u"오라클 실행 확인: cache+0x21c0+team*8(top_lead[team]) 을 0 으로 쓰면 weak_lead_state=true → epic 팀에서만 +1 (case4=1 / case6 epic 없음=0). " + O1, found_by="new")
EV("/specs[36]/consts[8]", to=2, evidence=u"오라클 실행 확인(부분): very_weak=false 경로에서 weak_lead_state as usize(=1) 가산 확인(case4). very_weak(적 미니언 150000 이내) 경로의 2 는 미니언 부재로 미실행. " + O1, found_by="new")
for j, ev in [(0, u"tcxdict OperationData 0x0 → cache"), (1, u"tcxdict OperationData 0x8 → context"), (2, u"tcxdict GameContext 0x0 → pool"),
              (3, u"tcxdict GameContext 0x38 → tutorial@tag 1B"), (4, u"tcxdict PlayerState 0x930 → info.team"), (5, u"tcxdict AbstractGameWithCache 0x0 → game(dyn)"),
              (7, u"tcxdict AbstractGameWithCache 0x170 → nexus[0]@Some.0"), (8, u"tcxdict AbstractGameWithCache 0x21c0 → top_lead[0] + 오라클 raw write 실측"),
              (9, u"tcxdict MobaMode 0x240 → epic_minion_buff_time[0] + 오라클 raw write(Game+0xed00 mode) 실측"),
              (10, u"tcxdict Entity 0x660 → x"), (11, u"tcxdict Entity 0x668 → y"), (12, u"tcxdict Entity 0x5c0 → id"), (13, u"tcxdict Entity 0x68 → ty@tag"),
              (14, u"tcxdict --enum EntityType 1 Minion 페이로드 +0x8 = Minion(176B), Minion.nearest_enemy +0x18 ⟹ Entity+0x88 tag/+0x90 Some.0"),
              (15, u"tcxdict --enum EntityType 1 + tcxdict Minion 0x18 nearest_enemy Option<usize>(16B)"),
              (16, u"tcxdict --enum EntityType 2 Tower 페이로드 +0x8 = Tower(192B), Tower.ty +0xb8 ⟹ Entity+0x128 TowerType(1B)")]:
    EV(u"/specs[36]/mem[%d]" % j, to=3, evidence=u"tcx 정본 대조: " + ev, found_by="reused")

# ─────────────────────────────── 37 i_am_chosen_defender ───────────────────────────────
p.fix("/specs[37]/knobs[0]/where", old=u"IR m04.ll:59375 `icmp ult i64 %272, 90000000001`", new=u"IR m04.ll:59363 `icmp ult i64 %272, 90000000001`",
      evidence=u"m04.ll:59363 `%273 = icmp ult i64 %272, 90000000001, !dbg !68318` · 59364 `%274 = icmp ult i64 %272, %139` · 59365 `select i1 %273, i1 %274, i1 false` (!68318 = defense_nexus.rs:339)", found_by="reused")
p.fix("/specs[37]/knobs[1]/where", old=u"IR m04.ll:59431 `add nsw i8 %307, -3`", new=u"IR m04.ll:59449 `add nsw i8 %307, -3`",
      evidence=u"m04.ll:59449 `%308 = add nsw i8 %307, -3, !dbg !68405` · 59450 `%309 = icmp ult i8 %308, 2` · 59432 `switch i64 %301 [ i64 3 → %302 / i64 2 → %305 ]`", found_by="reused")
p.fix("/specs[37]/knobs[2]/where", old=u"IR m04.ll:58991 `llvm.umax.i64(%99, 1)`", new=u"IR m04.ll:58996 `llvm.umax.i64(%99, 1)`",
      evidence=u"m04.ll:58996 `%128 = call noundef i64 @llvm.umax.i64(i64 %99, i64 1), !dbg !68098` (!68098 → inlinedAt !68097 = defense_nexus.rs:329)", found_by="reused")
p.fix("/specs[37]/knobs[3]/where", old=u"aux m04.ll:15733/15771 `icmp ugt %50, 2` / `icmp ugt %50, 4`", new=u"aux m04.ll:15768/15801 `icmp ugt i64 %50, 2` / `icmp ugt i64 %50, 4`",
      evidence=u"m04.ll:15768 `%51 = icmp ugt i64 %50, 2, !dbg !28284`(entity.rs:1693 → 464 → 347) · 15801 `%68 = icmp ugt i64 %50, 4, !dbg !28295`(1701 → 465 → 347)", found_by="reused")
# G16 P1 — role 에 %k
for j, pre in [(0, u"IR %0. "), (1, u"IR %1. "), (2, u"IR %2(i64). "), (3, u"IR %3(ptr)/%4(len) — `&[usize]` 가 SROA 로 두 레지스터. ")]:
    cur = mkpatch.locate(u"/specs[37]/sig/params[%d]/role" % j)
    head = cur[:20]
    p.fix(u"/specs[37]/sig/params[%d]/role" % j, old=head, new=pre + head,
          evidence=u"m04.ll:58647 define 인자 `(ptr deref(2528) %0, ptr deref(24) %1, i64 %2, ptr %3, i64 %4)` — G16 P1 규약", found_by="reused")
# G12 5건 — 오탐
p.fix("/specs[37]/consts[4]/src_line", old=317, new=317, kind=u"오탐",
      evidence=u"명세가 옳다. Some 태그 1 은 리터럴이 아니라 m04.ll:59386 `%282 = trunc nuw i64 %281 to i1, !dbg !68337` 로 접혀 있고 `!68337 = line 1542 inlinedAt !68321 = !DILocation(line: 317)`. 검사기가 `trunc…to i1` 을 ==1 비교로 안 본다", found_by="new")
p.fix("/specs[37]/consts[5]/src_line", old=318, new=318, kind=u"오탐",
      evidence=u"명세가 옳다. Player 태그 0 검사 = m04.ll:59412 `%294 = trunc nuw i64 %293 to i1, !dbg !68358` (`br i1 %294 → skip`), `!68358 → !68357(line 318) → !67638(661) → !67650 = !DILocation(line: 318)`. 59425 `%298 = icmp eq i64 %297, %19, !dbg !68373` 도 루트 318", found_by="new")
p.fix("/specs[37]/consts[16]/src_line", old=464, new=464, kind=u"오탐",
      evidence=u"명세가 옳다. aux m04.ll:15768 `%51 = icmp ugt i64 %50, 2, !dbg !28284` · `!28284 = line 1693 inlinedAt !28229 = distinct !DILocation(line: 464, … inlinedAt: !28224(347))`. G12 가 aux 범위(m04.ll 15665~15883)를 안 훑는다", found_by="reused")
p.fix("/specs[37]/consts[17]/src_line", old=465, new=465, kind=u"오탐",
      evidence=u"명세가 옳다. aux m04.ll:15801 `%68 = icmp ugt i64 %50, 4, !dbg !28295` · `!28295 = line 1701 inlinedAt !28234 = line 465`", found_by="reused")
p.fix("/specs[37]/consts[18]/src_line", old=474, new=474, kind=u"오탐",
      evidence=u"명세가 옳다. aux m04.ll:15753 `%41 = icmp sgt i64 %40, 0, !dbg !28282` · `!28282 = !DILocation(line: 474, … inlinedAt: !28277(463) → !28224(347))`", found_by="reused")
# G7·G10 open[0], G10 open[1]
p.errors.append({"path": "/specs[37]/open", "kind": u"실오류",
                 "old": u"expected_trade_net_hp(battle.rs:2493)·Effect::expected_damage_target·is_area·Blackboard::is_recent_visible 내부는 안 읽었다(별도 함수·game_core) — 여기선 호출 계약(인자·부호 판정)만 확정",
                 "new": u"expected_trade_net_hp(battle.rs:2493, m10.ll define)·Effect::expected_damage_target·Effect::is_area 의 내부(부호·계수·is_area 판정식)는 별도 함수라 미탐색 — 재료 = _gcbc `grep -n '^define.*expected_damage_target'` / m10.ll battle 본문",
                 "evidence": u"G7 과열림(is_recent_visible 은 shared 확정 항목이라 뺐다) + G10(사실 서술 꼬리 '호출 계약만 확정' 은 notes 로 — 산문 보고)", "behavior_change": False, "found_by": "reused"})
p.errors.append({"path": "/specs[37]/open", "kind": u"실오류",
                 "old": u"L363 키 비교의 튜플 성분 순서(bool@16 → i64@0 → u64@8 → u64@24)는 IR 비교 순서로 복원한 것 — 메모리 오프셋은 rustc 재배치 순이라 소스 튜플 선언 순과 다를 수 있으나 비교 순서 = 선언 순이라 논리는 확정",
                 "new": u"L348 튜플의 소스 선언 순서 자체는 mir=0 이라 미탐색(재료 = 줄 길이 산술: defense_nexus.rs:348) — IR 비교 순서 bool→i64→u64→u64(m04.ll:59269/59279/59289/59297)가 사전순 비교 순서이므로 논리에는 영향 없음",
                 "evidence": u"G10 class 오분류(사실 서술 → notes 로 산문 보고). 비교 순서 원문: m04.ll:59270 `icmp samesign ult i8 %229, %157` → 59280 `icmp slt i64 %233, %158` → 59290 `icmp ult i64 %237, %161` → 59297 `icmp ult i64 %241, %164`", "behavior_change": False, "found_by": "reused"})
# ev_up — 오라클
EV("/specs[37]/consts[0]", to=2, evidence=u"오라클 실행 확인: need_count=0 → 전 팀·전 포지션 false (B37 190/190 MATCH ×7세계). " + O1, found_by="new")
EV("/specs[37]/consts[12]", to=2, evidence=u"오라클 실행 확인: 아군 5슬롯 전부가 순위 비교에 들어감(독립 재구현 0..5 루프와 190/190 MATCH). " + O1, found_by="new")
EV("/specs[37]/consts[13]", to=2, evidence=u"오라클 실행 확인: better 카운트 = 키가 나보다 작은 아군 수, `better < need_count` 판정이 need 0..=5 × exclude 3종 × 10명 전수 MATCH. " + O1, found_by="new")
EV("/specs[37]/knobs[4]", to=2, evidence=u"오라클 실행 확인(부분): 미니언·가시 푸셔 없는 세계에서 키 = (false, 0, dist², id) 로 축소 → dist² 오름차순·동거리 id 순·strict `<` 가 190/190 MATCH. cant_handle_pusher/-clear 성분은 입력 판별력 부재(미실행). " + O1, found_by="new")
EV("/specs[37]/sig/params[2]", to=2, evidence=u"오라클 실행 확인: need_count=0 → 즉시 false(10명 × 3 exclude 전부). " + O1, found_by="new")
EV("/specs[37]/sig/params[3]", to=2, evidence=u"오라클 실행 확인: exclude=[내 id] → false / exclude=[최근접 1~2명] → 그 아군이 순위에서 빠져 better 가 준다(재구현과 190/190 MATCH). " + O1, found_by="new")
for j, ev in [(0, u"tcxdict PlayerState 0x930 → info.team"), (1, u"tcxdict PlayerState 0x9c0 → info.position@tag 4B"), (2, u"tcxdict OperationData 0x0 → cache"),
              (3, u"tcxdict OperationData 0x8 → context"), (4, u"tcxdict OperationData 0x10 → blackboard &[Blackboard;2]"), (5, u"tcxdict AbstractGameWithCache 0x0 → game"),
              (7, u"tcxdict AbstractGameWithCache 0x170 → nexus[0]@Some.0"), (8, u"tcxdict AbstractGameWithCache 0x1e0 → player_champion[0][0]@Some.0"),
              (9, u"tcxdict Entity 0x0 → team@tag 8B (TeamType: 0 Player/1 Neutral)"), (10, u"tcxdict Entity 0x8 → team@Player.0 usize"), (11, u"tcxdict Entity 0x68 → ty@tag"),
              (12, u"tcxdict --enum EntityType 1 + Minion 0x18 nearest_enemy ⟹ Entity+0x88"), (13, u"tcxdict Minion 0x18 Option<usize>(16B) ⟹ Some.0 = Entity+0x90"),
              (14, u"tcxdict --enum EntityType 2 + Tower 0xb8 ty TowerType ⟹ Entity+0x128"), (15, u"tcxdict Entity 0x5c0 → id"), (16, u"tcxdict Entity 0x660 → x"), (17, u"tcxdict Entity 0x668 → y"),
              (18, u"tcxdict Entity 0x4c0 → attack_effect@Some.0.casting@tag 4B"), (19, u"tcxdict Entity 0x490 → attack_effect@Some.0 (Effect 56B 선두)"),
              (20, u"tcxdict Entity 0x4f8 → skill_effect@Some.0.casting@tag"), (21, u"tcxdict Entity 0x4c8 → skill_effect@Some.0"), (22, u"tcxdict Entity 0x500 → skill2_effect@Some.0"),
              (23, u"tcxdict Entity 0x538 → ult_effect@Some.0"), (24, u"tcxdict Entity 0x5c8 → level"), (25, u"tcxdict Effect 0x30 → casting@tag 4B")]:
    EV(u"/specs[37]/mem[%d]" % j, to=3, evidence=u"tcx 정본 대조: " + ev, found_by="reused")

# ─────────────────────────────── 38 can_tower_focused_when_battle ───────────────────────────────
p.fix("/specs[38]/knobs[0]/where", old=u"IR m07.ll:51201 `add i64 %5, 15000`", new=u"IR m07.ll:51185 `add i64 %5, 15000`(루프 밖으로 호이스트, !dbg 없음) · 51298 `icmp ugt i64 %75, %112`",
      evidence=u"m07.ll:51185 `%47 = add i64 %5, 15000` (블록 44, 루프 진입 전) · 51293~51297 `%108 = add i64 %47, %62` … `%112 = add i64 %111, %107` · 51298 `%113 = icmp ugt i64 %75, %112, !dbg !63340`(line 70)", found_by="reused")
p.errors.append({"path": "/specs[38]/open", "kind": u"보강",
                 "old": u"AbstractGameWithCache::towers(team, pool) 가 살아있는 타워만 주는지(파괴된 타워 제외 여부)는 game_core 미독",
                 "new": u"AbstractGameWithCache::towers(team, pool) 가 살아있는 타워만 주는지(파괴된 타워 제외 여부)는 game_core 미독. 15차 오라클 실측: 시작 상태에서 towers(enemy).len()=9 = iter_towers_without_nexus 8 + **넥서스 1**(ty=Nexus) — 넥서스도 순회 대상이나 기본 넥서스는 attack_effect=None 이라 건너뜀",
                 "evidence": u"D15_o1_case0.out `C38 team=0 towers(1)=9 iter_towers_without_nexus=8 nexus_in_towers=1` · `tower#8 attack_effect=None`", "behavior_change": False, "found_by": "new"})
EV("/specs[38]/consts[4]", to=2, evidence=u"오라클 실행 확인: 적 타워 8기 × d∈{0,5000} 에서 dist = thr-1/thr/thr+1 (thr = range+(lv-1)*growth+stat_range+t.radius+champ.radius+d+15000) → true/true/false, 96/96 MATCH ×7세계 — 15000 가산과 `<=` 극성 확정. " + O1, found_by="new")
EV("/specs[38]/consts[5]", to=2, evidence=u"오라클 실행 확인(부분): radius_mult==0 경로(radius 그대로 10000+10000)만 실행 — mult≠0 의 (100+mult)/100 경로는 입력 판별력 부재. " + O1, found_by="new")
EV("/specs[38]/knobs[0]", to=2, evidence=u"오라클 실행 확인: 임계 = …+d+15000 경계 3점 96/96 MATCH. " + O1, found_by="new")
EV("/specs[38]/knobs[1]", to=2, evidence=u"오라클 실행 확인: setting.tower_attack_disable_tick=0 세계(case3, tick 0)에서 전 96 케이스 false. " + O1, found_by="new")
EV("/specs[38]/knobs[2]", to=2, evidence=u"오라클 실행 확인: d=0 → thr 35000 / d=5000 → thr 40000 로 경계가 정확히 d 만큼 이동. " + O1, found_by="new")
EV("/specs[38]/sig/params[5]", to=2, evidence=u"오라클 실행 확인: d=0/5000 경계 이동 96/96 MATCH. " + O1, found_by="new")
for j, ev in [(0, u"tcxdict AbstractGameWithCache 0x0 → game"), (2, u"tcxdict GameContext 0x8 → setting &GameSetting"), (3, u"tcxdict GameSetting 0x13f8 → tower_attack_disable_tick usize"),
              (4, u"tcxdict GameContext 0x0 → pool"), (5, u"tcxdict PlayerState 0x930 → info.team"), (6, u"tcxdict PlayerState 0x9c0 → info.position@tag"),
              (7, u"tcxdict AbstractGameWithCache 0x1e0 → player_champion[0][0]@Some.0"), (10, u"tcxdict Entity 0x4c0 → attack_effect@Some.0.casting@tag"),
              (11, u"tcxdict Entity 0x4a0 → attack_effect@Some.0.range u64"), (12, u"tcxdict Entity 0x4a8 → attack_effect@Some.0.growth_range u64"),
              (13, u"tcxdict Entity 0x5c8 → level"), (14, u"tcxdict Entity 0x438 → stat_buff_cached.range usize"), (15, u"tcxdict Entity 0x660 → x"), (16, u"tcxdict Entity 0x668 → y"),
              (17, u"tcxdict Entity 0x470 → stat_buff_cached.radius_mult i32"), (18, u"tcxdict Entity 0x680 → radius usize")]:
    EV(u"/specs[38]/mem[%d]" % j, to=3, evidence=u"tcx 정본 대조: " + ev, found_by="reused")

# ─────────────────────────────── 39 is_wave_priority_start_line ───────────────────────────────
p.fix("/specs[39]/knobs[0]/where", old=u"IR m15.ll:52887 `icmp ugt i64 %21, 8`", new=u"IR m15.ll:52901 `icmp ugt i64 %21, 8`",
      evidence=u"m15.ll:52901 `%22 = icmp ugt i64 %21, 8, !dbg !59419` (!59419 = objective_helpers.rs:223) · 52900 `%21 = load i64, ptr %20`(line_minions[enemy]+0x18 len)", found_by="reused")
p.fix("/specs[39]/consts[3]/meaning", old=u"first_tower_position: team == 0 이면 (x,y) 를 뒤집는다(TowerType::get_position 인라인, tower.rs:105/110/115)",
      new=u"first_tower_position: `icmp eq i64 team, 0` 으로 select — team0 → (48000,272000)/(368000,592000)/(688000,912000), team1 → 성분 교환 (272000,48000)/(592000,368000)/(912000,688000) (TowerType::get_position 인라인, tower.rs:105/110/115)",
      evidence=u"m15.ll:52957 `%40 = icmp eq i64 %6, 0, !dbg !59526`(루트 230) · 52972 `%46 = select i1 %40, i64 272000, i64 48000` · 52973 `%47 = select i1 %40, i64 48000, i64 272000` · 52976 `distance(i64 %47, i64 %46, nexus.x, nexus.y)` ⟹ team0 = (x 48000, y 272000). '뒤집는다' 는 기준이 안 적혀 양의로 읽힌다", kind=u"보강", found_by="new")
for j, ev in [(0, u"tcxdict OperationData 0x0 → cache"), (1, u"tcxdict PlayerState 0x930 → info.team"), (2, u"tcxdict AbstractGameWithCache 0x10 → top_minions[0] (bumpalo Vec 32B: buf.ptr@0 · len@0x18)"),
              (3, u"tcxdict AbstractGameWithCache 0x50 → mid_minions[0]"), (4, u"tcxdict AbstractGameWithCache 0x90 → bottom_minions[0]"),
              (7, u"tcxdict AbstractGameWithCache 0x170 → nexus[0]@Some.0"), (8, u"tcxdict AbstractGameWithCache 0x180 → top_tower[0]@Some.0 (IR m15.ll:52935 `shl nuw nsw i8 %2, 5` = line*32, 52938 `+384`)"),
              (9, u"tcxdict Entity 0x660 → x"), (10, u"tcxdict Entity 0x668 → y")]:
    EV(u"/specs[39]/mem[%d]" % j, to=3, evidence=u"tcx 정본 대조: " + ev, found_by="reused")

# ─────────────────────────────── 지시(도시에) 오류 ───────────────────────────────
p.brief_error(u"§4 G16 [35] 두 번째 항목 문면이 잘렸다: 「8차 확정 규약 = sret 를」 에서 끝나 규약 본문(params[0] name `(sret)`, i=0)이 안 보인다 — paramrole.py:346~348 원문을 읽어야 알 수 있었다. mkdossier 가 G16 상세줄을 `[:N]` 로 자른다.")
p.brief_error(u"§4 G12 [37] 항목 5건이 전부 오탐인데 「실제 후보」가 aux 범위(m04.ll 15665~15883)와 `trunc … to i1` 접힘을 못 봐서 생긴 것이다 — srclinecheck.py 가 spec.aux 범위와 trunc-to-i1 을 훑지 않는다(검사기 결함, 명세 결함 아님). [36] 3건도 aux(m12.ll) 미스캔 오탐.")
p.brief_error(u"§4 G13 [35] knobs[1] `i8 0`(42480 → 실제 42508)은 G13 이 적발하지 못했다 — 인용이 opcode 가 아닌 인자 리터럴이면 검사 대상에서 빠진다. [36] knobs[2] 도 3줄 어긋났는데(56920 → 56917) ±2 경계라 미적발.")
p.brief_error(u"§1 표의 [37] 형제 0개·[35] 형제 0개 — 35 는 `resolve_fight_stake_roster`(fight_model.rs:645, m10.ll:47870, 인자 `&[(&Entity,i64,bool)]` roster 판, 호출처 m10.ll:16386/16527 이 35 의 호출처 16359/16464 와 같은 함수) 가 있는데 siblings 자동 열거가 안 잡았다(G3 사각).")
p.brief_error(u"§5 생성 보조 `mkpatch.py` 의 `Patch.ev()` 가 `/specs[i]/mem[j]`·`/specs[i]/consts[j]` 같은 최상위 배열 경로를 「ev 상향은 배열 원소만」으로 거부한다 — mkpatch.py:134 가 PATH group(3)(중첩 필드)을 원소 판정에 쓴다(group(4) 가 인덱스). 이 배치는 같은 계약의 dict 를 직접 넣어 우회했다.")
p.brief_error(u"§4 G10 [35] open[1] 의 인용 「0.ll:39915) 내부에서…」 가 앞이 잘려 있어(원문 `resolve_fight_full(m10.ll:39915)`) 도시에만 보면 어느 파일인지 모른다.")

out = p.save()
print(out)
