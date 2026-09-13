# -*- coding: utf-8 -*-
u"""18차 배치 B patch.json 생성 (specs[62]~[66]). mkpatch 참조구현 사용 + insert 는 dict 직접."""
import sys, io, json
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch

p = mkpatch.Patch(round=18, batch="B")
NEW = "new"

# ═══════════════════════════════ #65 G16 P1 — SROA/ArgumentPromotion %k 대응 ═══════════════════
p.fix("/specs[65]/sig/params[0]/role",
      old=u"dbg_value 가 poison(m09.ll:52668)이고 %0 은 expected_damage_target 의 context 인자(dereferenceable(64))로만 쓰임 = LLVM ArgumentPromotion.",
      new=u"IR %0 (ArgumentPromotion): dbg_value 가 poison(m09.ll:52668)이고 %0 은 null 가정(m09.ll:52700 `%11 = icmp ne ptr %0, null` → llvm.assume) 뒤 expected_damage_target 의 context 인자(dereferenceable(64), m09.ll:52722/52769/52805)로만 쓰임. 호출부는 data+8(context)을 미리 로드해 넘긴다(m09.ll:22353~22354 gep +8 → load, 22402 첫 인자 %96).",
      evidence=u"m09.ll:52666 define 인자 5개 `(ptr captures(address, read_provenance) %0, ptr readonly captures(address) %1, i64 %2, ptr … dereferenceable(1728) %3, i64 noundef %4)` vs 소스 인자 4개(DILocalVariable !47139~!47142 arg 1~4). %0: m09.ll:52700 `%11 = icmp ne ptr %0, null`, 52721 `llvm.assume(i1 %11)`, 52722 expected_damage_target 2번째 인자 `dereferenceable(64) %0`. 호출부 m09.ll:22353 `%95 = getelementptr inbounds nuw i8, ptr %4, i64 8` / 22354 `%96 = load ptr, ptr %95` → 22402 첫 인자 %96",
      behavior_change=False, found_by="reused", kind=u"보강")

p.fix("/specs[65]/sig/params[1]/role",
      old=u"순회 대상 챔피언 목록(DPS 기여자). len 0 이면 dps 합 = 0",
      new=u"IR %1 = 버퍼 ptr · IR %2 = len (ArgumentPromotion — 소스 `champions: &Vec<&Entity>` 한 인자가 두 레지스터로 승격, dbg_value poison m09.ll:52669, 슬라이스 fragment dbg_value 52679~52681). 호출부가 bumpalo Vec 의 +0(ptr)·+24(len)을 미리 로드해 넘긴다(m09.ll:22400~22402). 순회 대상 챔피언 목록(DPS 기여자). len 0 이면 루프 없이 dps 합 = 0 (m09.ll:52696 `%9 = icmp eq i64 %2, 0`)",
      evidence=u"m09.ll:52666 define 2·3번째 인자 `ptr readonly captures(address) %1, i64 %2`. 52669 `#dbg_value(ptr poison, !47140` (!47140 = champions arg 2). 52678~52681 `#dbg_value(ptr %1, !47189, !DIExpression(DW_OP_LLVM_fragment, 0, 64)` / `#dbg_value(i64 %2, !47189, …fragment, 64, 64)` = (ptr,len) 슬라이스. 52696 `%9 = icmp eq i64 %2, 0` → 74(루프 생략). 52685~52686 `%7 = shl nuw nsw i64 %2, 3` / `%8 = getelementptr inbounds nuw i8, ptr %1, i64 %7` = end ptr. 호출부 m09.ll:22400 `%110 = load ptr, ptr %35` / 22401 `%112 = load i64, ptr %111`(%111 = %35+24) → 22402 인자 2·3",
      behavior_change=False, found_by="reused", kind=u"보강")

p.fix("/specs[65]/sig/params[2]/role",
      old=u"피격 대상. expected_damage_target 의 target · CastingTarget::check 의 target",
      new=u"IR %3 (dbg_value m09.ll:52670, dereferenceable(1728)). 피격 대상. expected_damage_target 의 target(5번째 인자, m09.ll:52722/52769/52805) · CastingTarget::check 의 target(3번째 인자, m09.ll:52751/52788)",
      evidence=u"m09.ll:52670 `#dbg_value(ptr %3, !47141` (!47141 = epic arg 3). 52751 `… CastingTarget5check(ptr dereferenceable(4) %36, ptr dereferenceable(1728) %16, ptr dereferenceable(1728) %3)`. 52722 expected_damage_target 마지막 인자 `dereferenceable(1728) %3`",
      behavior_change=False, found_by="reused", kind=u"보강")

p.fix("/specs[65]/sig/params[3]/role",
      old=u"잡아야 할 HP 량(현재 HP 가 아니라 호출자가 주는 값)",
      new=u"IR %4 (dbg_value m09.ll:52671, `i64 noundef %4`). 잡아야 할 HP 량(현재 HP 가 아니라 호출자가 주는 값). 559 에서만 소비(m09.ll:52829 `%76 = mul i64 %4, 1000`)",
      evidence=u"m09.ll:52671 `#dbg_value(i64 %4, !47142` (!47142 = hp arg 4). 본문에서 %4 의 유일한 use = 52829 `%76 = mul i64 %4, 1000` → 52830 `%77 = udiv i64 %76, %75` → 52832 `ret i64 %77`",
      behavior_change=False, found_by="reused", kind=u"보강")

# ═══════════════════════════════ #66 G16 P2 — sret out-ptr 행 삽입 ═══════════════════
p.errors.append({
    "op": "insert", "path": "/specs[66]/sig/params", "at": 0,
    "guard": "(sret)", "guard_key": "name",
    "new": {"i": 0, "name": "(sret)", "type": "*mut (bool,i32,i32)(12B, align 4)",
            "note": u"반환값 out-ptr. IR 첫 인자 `sret([12 x i8])`(m04.ll:62981) — (bool,i32,i32) 는 ScalarPair 가 아니라 메모리 반환. 레이아웃 = +0x0 .1 evaluated_score(i32) · +0x4 .0 ok(u8) · +0x8 .2 actual_score(i32) — m04.ll:63699 `store i32 %303, ptr %0` · m04.ll:63698 `store i8 %306, ptr %305`(%305=%0+4) · m04.ll:63701 `store i32 %263, ptr %307`(%307=%0+8). 조기 반환(false,0,0) 경로도 같은 세 자리에 store"},
    "kind": u"보강",
    "evidence": u"m04.ll:62981 `define void @…evaluate_gank_opportunity_with_score(ptr dead_on_unwind noalias noundef writable writeonly sret([12 x i8]) align 4 captures(none) dereferenceable(12) %0, ptr noalias noundef align 16 dereferenceable(320) %1, …, i32 noundef %5)` — 반환형 void + sret. 63696 `%305 = getelementptr inbounds nuw i8, ptr %0, i64 4` / 63698 `store i8 %306, ptr %305` / 63699 `store i32 %303, ptr %0` / 63700 `%307 = getelementptr inbounds nuw i8, ptr %0, i64 8` / 63701 `store i32 %263, ptr %307`. 규약 정본 = specs[7]·[15] params[0] `(sret)` i=0",
    "behavior_change": False, "found_by": "reused"})

# ═══════════════════════════════ #66 G12 consts[1] src_line 713 → 716 ═══════════════════
p.fix("/specs[66]/consts[1]/src_line", old=713, new=716,
      evidence=u"m04.ll:63091 `store i64 5, ptr %49, align 8, !dbg !72398` (%49 = %9+32, Filter 어댑터 구조체 안의 Range.end) — !72398 → Filter::new(filter.rs:28) ← Iterator::filter(iterator.rs:957) ← passive_jungle.rs:716. 63080 `#dbg_value(i64 5, !72342, …fragment 256…, !72397)` 도 루트 716. 713 은 IR !dbg 사슬 어디에도 없다(강후보 {716, 715(bounds check), 759(shl 5)} · 약후보 {716, 702}). 스펙이 인용한 63102 는 `store ptr %2` 줄이지 `store i64 5` 가 아니다",
      behavior_change=False, found_by="reused", kind=u"실오류")
p.fix("/specs[66]/consts[1]/meaning",
      old=u"포지션 수 — line_allies 후보 Range 0..5 (m04.ll:63102 store i64 5). 판정 아님",
      new=u"포지션 수 — line_allies 후보 Range 0..5. 리터럴 5 는 이터레이터 체인(713~716)이 Filter 구조체로 접힐 때 Range.end 로 저장되며 IR 위치는 체인 마지막 `.filter` 줄 716 (m04.ll:63091 `store i64 5, ptr %49` !dbg !72398 → filter.rs:28 ← iterator.rs:957 ← 716). 같은 5 가 715 closure$2 의 player_champion[team][pos] bounds check 에도 나온다(m01.ll:30837 `icmp ult i64 %27, 5`). 판정 아님",
      evidence=u"m04.ll:63091 `store i64 5, ptr %49, align 8, !dbg !72398`. m04.ll:63102 = `store ptr %2, ptr %51` (인용 오기). m01.ll:30837 `%54 = icmp ult i64 %27, 5, !dbg !41831` (!41831 → passive_jungle.rs:715 closure$2)",
      behavior_change=False, found_by="reused", kind=u"실오류")

# ═══════════════════════════════ #66 G10/G7 open[0] → 사실 서술(shared.is_recent_visible 확정으로 해소) ═══════════════════
p.fix("/specs[66]/open[0]",
      old=u"적팀 판 last_visible 을 자기 시점 가시성으로 쓰는 것이 의도인지는 Blackboard 갱신 코드(미탐색)로만 확정 가능 — IR 로는 인덱스 사실만 확정",
      new=u"이 비대칭은 의도된 것이며 「사실 서술」이다: shared.is_recent_visible.blackboard_인덱스_의미(★확정) = Blackboard[T].last_visible[pos] 는 '팀 (1−T) 가 팀 T 의 pos 챔피언을 마지막으로 본 틱'(근거 Blackboard::update _gcbc/g15.ll:249663). 따라서 적 e 에 대해 blackboard[enemy_team].is_recent_visible(e) = '내 팀이 e 를 최근(120틱)에 봤는가' 가 맞고, big_goal 은 자기 팀 계획판이라 blackboard[team].in_big_line 이 맞다. 같은 패턴이 v23_recent_visible_enemies_near_point 에도 있다(m15.ll:55582 `%9 = sub i64 1, %8` → 55666 blackboard gep → 55742 is_recent_visible)",
      evidence=u"specs20_v3 shared.is_recent_visible.blackboard_인덱스_의미 ★확정 문면 + m04.ll:66367~66370 `%21 = load i64`(team) / `%22 = sub i64 1, %21` / `icmp ult i64 %22, 2` + m15.ll:55582 `%9 = sub i64 1, %8`, 55666 `getelementptr inbounds nuw { … } , ptr %19, i64 %9`, 55742 `Blackboard17is_recent_visible(ptr dereferenceable(744) %21, …)`",
      behavior_change=False, found_by="reused", kind=u"보강")

# ═══════════════════════════════ #63 G15 consts[5] 자기모순 문면 ═══════════════════
p.fix("/specs[63]/consts[5]/meaning",
      old=u"팀 수 — player.info.team 의 bounds check 상한(임계 아님)",
      new=u"팀 수 — player.info.team 의 bounds check 상한(m09.ll:37743 `%21 = icmp ult i64 %20, 2` → 실패 시 panic_bounds_check). 배열 [[_;5];2] 길이 = 비교 상한이라 임계로 분류하되 게임 판정 노브가 아닌 안전검사",
      evidence=u"m09.ll:37743 `%21 = icmp ult i64 %20, 2, !dbg !38030` / 37744 `br i1 %21, label %22, label %198` / 38188 panic_bounds_check. kindchk 관측 = CMP_ORD 만 → 파생 kind 는 임계로 고정되는데 옛 문면이 「임계 아님」이라 NEG 자기모순. 문면에서 부정을 걷고 「안전검사」로 명시",
      behavior_change=False, found_by="reused", kind=u"실오류")

# ═══════════════════════════════ G20 R4 — #63 39 vs #64 40 (오탐: 같은 의미, 다른 리터럴) ═══════════════════
p.fix("/specs[63]/knobs[0]/what", old=u"건강 판정 HP% 하한",
      new=u"건강 판정 HP% 하한(인라인 리터럴 39 — `ugt 39` ≡ HP%≥40)",
      evidence=u"m09.ll:37800~37802 `%36 = mul i64 %35, 100` / `%37 = udiv i64 %36, %30` / `%38 = icmp ugt i64 %37, 39` (598 도 37837~37839 동일). #64·#62 는 min_hp_ratio=40 을 콜리에 넘기고 콜리가 `ult %5`(미만 제외)로 비교해 같은 HP%≥40 — 값이 다른 것은 리터럴 표현(>39 vs 인자 40 ≥)이지 임계가 아니다. G20 R4 오탐",
      behavior_change=False, found_by="new", kind=u"오탐")
p.fix("/specs[63]/knobs[0]/effect",
      old=u"올리면(예 49) 저체력 인원이 양쪽 셈에서 빠짐. 아군·적 두 클로저에 각각 박혀 있어 둘 다 바꿔야 대칭 유지",
      new=u"올리면(예 49) 저체력 인원이 양쪽 셈에서 빠짐. 아군·적 두 클로저에 각각 박혀 있어 둘 다 바꿔야 대칭 유지. 의미상 #62/#64 의 min_hp_ratio 40(콜리 `ult` 로 ≥40) 과 같은 임계(HP%≥40)이며 여기만 인라인 리터럴 39(`ugt`) 로 나타난다",
      evidence=u"m09.ll:37802 `%38 = icmp ugt i64 %37, 39` ↔ m15.ll:53464 `%26 = icmp ult i64 %25, %5`(%5 = min_hp_ratio 인자 40, 미만이면 제외)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[64]/knobs[2]/what", old=u"건강 판정 HP% 하한",
      new=u"건강 판정 HP% 하한(min_hp_ratio)",
      evidence=u"m15.ll:54730 v23_healthy_allies_near_point 6번째 인자 `i64 noundef 40` / 54732 v23_recent_visible_enemies_near_point 동일. 콜리 objective_helpers.rs:12 v23_healthy: m15.ll:53464 `%26 = icmp ult i64 %25, %5` → HP% < 40 이면 제외 = HP%≥40 건강. #62 knobs[1] 과 같은 노브명·같은 값(40)으로 통일, #63 은 인라인 리터럴 39 라 이름을 갈랐다",
      behavior_change=False, found_by="new", kind=u"오탐")
p.fix("/specs[64]/knobs[2]/effect",
      old=u"올리면 저체력 아군이 셈에서 빠져 allies>=need 를 만족하기 어려워짐",
      new=u"올리면 저체력 아군이 셈에서 빠져 allies>=need 를 만족하기 어려워짐(적 셈 enemies 도 같은 값을 받으므로 저체력 적도 빠짐). 비교는 콜리 v23_healthy(objective_helpers.rs:12): HP% < min_hp_ratio 이면 제외 ⟹ HP%≥40 이 건강. #63 의 인라인 `> 39` 와 같은 임계",
      evidence=u"m15.ll:53455~53465 `%20 = icmp eq i64 %19, 0`(max_hp 0 → div0 패닉) / `%24 = mul i64 %23, 100` / `%25 = udiv i64 %24, %19` / `%26 = icmp ult i64 %25, %5` / `br i1 %26, label %46, label %28`(46 = 건너뜀). enemies 쪽 55703~55706 동일(`%33 = icmp ult i64 %32, %5`)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[62]/knobs[1]/effect",
      old=u"정확한 비교 방향은 피호출 함수 명세 참조",
      new=u"비교는 콜리 v23_healthy(objective_helpers.rs:12): HP% = hp*100/stat_cached.hp 가 min_hp_ratio 미만이면 제외(`icmp ult`) ⟹ HP%≥40 이 건강. #63 의 인라인 `> 39` 와 같은 임계",
      evidence=u"m15.ll:53464 `%26 = icmp ult i64 %25, %5, !dbg !60138` (!60138 → objective_helpers.rs:12 v23_healthy ← 27 closure$0) / 53465 `br i1 %26, label %46, label %28`. enemies 콜리 55705 `%33 = icmp ult i64 %32, %5`",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[62]/consts[7]/meaning",
      old=u"실제 비교식은 피호출 함수 내부(미독해)",
      new=u"콜리 v23_healthy(objective_helpers.rs:12, m15.ll:53464 `icmp ult i64 %25, %5`)가 HP%(hp*100/stat_cached.hp) < 40 이면 제외 ⟹ HP%≥40 이 건강(#63 의 `> 39` 와 동일 임계). stat_cached.hp==0 이면 콜리에서 div0 패닉(53455)",
      evidence=u"m15.ll:53455 `%20 = icmp eq i64 %19, 0` / 53460 `%24 = mul i64 %23, 100` / 53463 `%25 = udiv i64 %24, %19` / 53464 `%26 = icmp ult i64 %25, %5` / 53465 `br i1 %26, label %46, label %28`",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[64]/consts[5]/meaning",
      old=u"min_hp_ratio 인자(HP% 하한). 158·159 동일",
      new=u"min_hp_ratio 인자(HP% 하한). 158·159 동일. 콜리 v23_healthy(objective_helpers.rs:12, m15.ll:53464 `icmp ult i64 %25, %5`)가 HP% < 40 이면 제외 ⟹ HP%≥40 이 건강(#63 의 `> 39` 와 동일 임계)",
      evidence=u"m15.ll:54730/54732 인자 `i64 noundef 40` → 콜리 m15.ll:53464 / 55705 `icmp ult … %5`",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[62]/consts[6]/meaning",
      old=u"근접 반경(range 인자, 월드 단위 = 5.625셀). 아군·적 셈 둘 다 같은 값",
      new=u"근접 반경 임계(range 인자, 월드 단위 = 5.625셀). 아군·적 셈 둘 다 같은 값. 콜리에서 range² 와 제곱거리 비교: 아군 dist² ≤ range² 포함(m15.ll:53497 `icmp ule i64 %43, %14`, %14 = range*range 53425) / 적 dist² > range² 제외(55738 `icmp ugt i64 %50, %20`) — 외연 동일",
      evidence=u"m15.ll:53425 `%14 = mul i64 %4, %4` / 53497 `%44 = icmp ule i64 %43, %14` ; 55665 `%20 = mul i64 %4, %4` / 55738 `%51 = icmp ugt i64 %50, %20` / 55739 `br i1 %51, label %55, label %52`(52 = is_recent_visible 호출)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[64]/consts[4]/meaning",
      old=u"오브젝트 주변 인원 셈 반경(range). 158·159 동일",
      new=u"오브젝트 주변 인원 셈 반경 임계(range). 158·159 동일. 콜리에서 range² 와 제곱거리 비교(아군 m15.ll:53497 `icmp ule i64 %43, %14` 포함 / 적 55738 `icmp ugt i64 %50, %20` 제외)",
      evidence=u"m15.ll:53425 `%14 = mul i64 %4, %4` / 53497 `%44 = icmp ule i64 %43, %14` / 55665 `%20 = mul i64 %4, %4` / 55738 `%51 = icmp ugt i64 %50, %20`",
      behavior_change=False, found_by="new", kind=u"보강")

# ═══════════════════════════════ #62 G18 — logic 이 인용한 필드/vtable 슬롯 행 추가 ═══════════════════
def ins(path, at, name, row, evidence):
    p.errors.append({"op": "insert", "path": path, "at": at, "guard": name, "guard_key": "name",
                     "new": row, "kind": u"보강", "evidence": evidence,
                     "behavior_change": False, "found_by": "reused"})

ins("/specs[62]/mem", 1, "game.data_ptr",
    {"base": "AbstractGameWithCache", "offset": "0x0", "name": "game.data_ptr",
     "note": u"&dyn AbstractGame 팻포인터의 데이터 절반 — get_game_mode/get_entity_by_id/is_visible 의 self (m15.ll:52587 `%18 = load ptr, ptr %17`, 52660, 52741)", "dir": "r"},
    u"m15.ll:52586 `%17 = load ptr, ptr %1`(cache) / 52587 `%18 = load ptr, ptr %17, align 8` → 52592 `%23 = tail call { i64, ptr } %22(ptr noundef nonnull %18)`. 다른 명세(03·04·08·…) 동명 규약 game.data_ptr")
ins("/specs[62]/mem", 2, "game.vtable_ptr",
    {"base": "AbstractGameWithCache", "offset": "0x8", "name": "game.vtable_ptr",
     "note": u"&dyn AbstractGame 팻포인터의 vtable 절반 (m15.ll:52588~52589 `%19 = getelementptr inbounds nuw i8, ptr %17, i64 8` / `%20 = load ptr, ptr %19`; 52661~52662, 52742~52743)", "dir": "r"},
    u"m15.ll:52588 `%19 = getelementptr inbounds nuw i8, ptr %17, i64 8` / 52589 `%20 = load ptr, ptr %19, align 8` → 52590 gep %20+64")
ins("/specs[62]/mem", 3, "get_game_mode",
    {"base": "AbstractGame::vtable", "offset": "0x40", "name": "get_game_mode",
     "note": u"divtable AbstractGame 0x40. 반환 {i64 tag, ptr payload} = GameMode; tag 0 = Moba → as_moba Some (m15.ll:52590~52596 Morgard / 52663~52669 Serpen)", "dir": "r"},
    u"m15.ll:52590 `%21 = getelementptr inbounds nuw i8, ptr %20, i64 64` / 52591 `%22 = load ptr, ptr %21, align 8, …, !invariant.load` / 52592 `%23 = tail call { i64, ptr } %22(ptr noundef nonnull %18)` / 52596 `%25 = icmp eq i64 %24, 0`. Serpen 경로 52663~52669 동일")
ins("/specs[62]/mem", 4, "get_entity_by_id",
    {"base": "AbstractGame::vtable", "offset": "0x1f0", "name": "get_entity_by_id",
     "note": u"divtable AbstractGame 0x1f0. (usize id) -> Option<&Entity>(널=None) — live_list[0] id 로 에픽 엔티티 조회, null 이면 false (m15.ll:52746~52749)", "dir": "r"},
    u"m15.ll:52746 `%70 = getelementptr inbounds nuw i8, ptr %69, i64 496` / 52747 `%71 = load ptr, ptr %70` / 52748 `%72 = tail call noundef align 8 ptr %71(ptr noundef nonnull %67, i64 noundef %58)` / 52749 `%73 = icmp eq ptr %72, null`")
ins("/specs[62]/mem", 5, "is_visible",
    {"base": "AbstractGame::vtable", "offset": "0xf8", "name": "is_visible",
     "note": u"divtable AbstractGame 0xf8. (team, entity_id) -> bool — 에픽이 내 팀에 보이는가 (m15.ll:52756~52758)", "dir": "r"},
    u"m15.ll:52756 `%77 = getelementptr inbounds nuw i8, ptr %69, i64 248` / 52757 `%78 = load ptr, ptr %77` / 52758 `%79 = tail call noundef zeroext i1 %78(ptr noundef nonnull %67, i64 noundef %62, i64 noundef %76)` (%62 = player.info.team, %76 = ent.id@+0x5c0)")

# ═══════════════════════════════ #62 logic / params 의 IR 줄 인용 정정(전부 어긋나 있었다) ═══════════════════
for old, new in [
    (u"(m15.ll:52560~52562, 극성:", u"(m15.ll:52567~52569, 극성:"),
    (u"(m15.ll:52571~52576 switch", u"(m15.ll:52578~52582 switch"),
    (u"호출됨(m15.ll:52766~52772 %92)", u"호출됨(m15.ll:52779~52793 %92 블록: len==0 경로도 camp_pos 호출 뒤 false)"),
    (u"먼저 평가(m15.ll:52716~52720)", u"먼저 평가(m15.ll:52756~52758)"),
    (u"풀피면 false (m15.ll:52723~52728)", u"풀피면 false (m15.ll:52762~52766)"),
    (u"// 206 (m15.ll:52741~52743:", u"// 206 (m15.ll:52774~52776:"),
    (u"전달됨(m15.ll:52694~52695, 52736)", u"전달됨(m15.ll:52739~52740, 52770·52772)"),
]:
    p.fix("/specs[62]/logic", old=old, new=new,
          evidence=u"m15.ll 현행 줄: 52567~52569 `%9 = add nsw i8 %8, -1` / `%10 = icmp ult i8 %9, 6` / `br i1 %10, label %106, label %16`; 52578~52582 `switch i8 %15, label %106 [ i8 0 / 5 / 7 / 8 → %35 ]`; 52779 `92:` 블록 · 52793 camp_pos; 52756~52758 gep 248 → is_visible 호출; 52762~52766 gep 1648/1576 → `%85 = icmp ult i64 %82, %84`; 52774~52776 `%89 = icmp ne i64 %88, 0` / `%90 = icmp samesign uge i64 %88, %87` / `%91 = select i1 %89, i1 %90, i1 false`; 52739~52740 extractvalue 1/0; 52770/52772 두 호출의 3·4번째 인자 `i64 noundef %66, i64 noundef %65`",
          behavior_change=False, found_by="new", kind=u"실오류")
p.fix("/specs[62]/sig/params[2]/role", old=u"즉시 false (m15.ll:52549~52552)", new=u"즉시 false (m15.ll:52555 `switch i8 %2, label %106 [` · 52556 `i8 4, label %4` · 52557 `i8 5, label %11`)",
      evidence=u"m15.ll:52555 `switch i8 %2, label %106 [` / 52556 `i8 4, label %4` / 52557 `i8 5, label %11`. 52549~52552 는 #dbg_value 줄",
      behavior_change=False, found_by="new", kind=u"실오류")

# ═══════════════════════════════ #65 logic / consts / open 의 IR 줄 인용 정정 ═══════════════════
for old, new in [
    (u"unwrap_failed 패닉 (m09.ll:52690~52701)", u"unwrap_failed 패닉 (m09.ll:52713~52715 tag 검사, 52729 unwrap_failed)"),
    (u"div0 패닉 (m09.ll:52694~52713)", u"div0 패닉 (m09.ll:52724 `icmp eq i64 %23, 0` → 52745; 단 attack_cooltime 은 umax(…,3) 라 실제로는 0 불가 — _gcbc g06.ll:66510 `range(i64 3, 0)`)"),
    (u"&NONE(@anon.…288) }  (m09.ll:52745~52749)", u"&NONE(@anon.…288) }  (m09.ll:52757~52761)"),
]:
    p.fix("/specs[65]/logic", old=old, new=new,
          evidence=u"m09.ll:52713 `%17 = getelementptr inbounds nuw i8, ptr %16, i64 1216` / 52715 `%19 = icmp eq i32 %18, -1` / 52729 `tail call void @…unwrap_failed`; 52724 `%24 = icmp eq i64 %23, 0` / 52745 panic_const_div_by_zero; 52757 gep 1480 / 52759 `%42 = icmp ugt i64 %41, 2` / 52760 gep 1280 / 52761 `%44 = select i1 %42, ptr %43, ptr @anon.…288`. _gcbc g06.ll:66510 `define noundef range(i64 3, 0) i64 @…Entity15attack_cooltime` · 본문 끝 `llvm.umax.i64(i64 %15, i64 3)`",
          behavior_change=False, found_by="new", kind=u"실오류")
p.fix("/specs[65]/consts[2]/meaning", old=u"(m09.ll:52746)", new=u"(m09.ll:52759)",
      evidence=u"m09.ll:52759 `%42 = icmp ugt i64 %41, 2, !dbg !47247`. 52746 은 `%31 = getelementptr … i64 1272` 줄",
      behavior_change=False, found_by="new", kind=u"실오류")
p.fix("/specs[65]/consts[3]/meaning", old=u"상수 1 로 접힘 m09.ll:52829)", new=u"상수 1 로 접힘 — umax m09.ll:52822, phi 52826 `[ 1, %5 ]`)",
      evidence=u"m09.ll:52822 `%73 = tail call i64 @llvm.umax.i64(i64 %61, i64 1)` / 52826 `%75 = phi i64 [ 1, %5 ], [ %73, %72 ]`. 52829 는 `%76 = mul i64 %4, 1000`",
      behavior_change=False, found_by="new", kind=u"실오류")
p.fix("/specs[65]/consts[4]/meaning",
      old=u"쿨타임 0 검사(0 이면 panic_const_div_by_zero)",
      new=u"쿨타임 0 검사(0 이면 panic_const_div_by_zero — 단 콜리가 umax 로 하한을 보장해 도달 불가: attack_cooltime·skill_cooltime ≥3, skill2_cooltime ≥1)",
      evidence=u"_gcbc g06.ll:66510 `define noundef range(i64 3, 0) i64 @…Entity15attack_cooltime` 본문 `%16 = tail call noundef i64 @llvm.umax.i64(i64 %15, i64 3)`; 66405 `range(i64 3, 0)` skill_cooltime umax 3; 66747 `range(i64 1, 0)` skill2_cooltime umax 1",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[65]/open[4]", old=u"상대 gep(m09.ll:52750~52751, 52774)이라 접힘", new=u"상대 gep(m09.ll:52763 `%45 = getelementptr inbounds nuw i8, ptr %44, i64 48`, 52787 `%58 = … ptr %44, i64 40`)이라 접힘",
      evidence=u"m09.ll:52761 `%44 = select i1 %42, ptr %43, ptr @anon.…288` / 52763 `%45 = getelementptr inbounds nuw i8, ptr %44, i64 48` / 52787 `%58 = getelementptr inbounds nuw i8, ptr %44, i64 40`",
      behavior_change=False, found_by="new", kind=u"실오류")

# ═══════════════════════════════ open 정리 — 확정된 것은 「사실 서술」로 선언 ═══════════════════
p.fix("/specs[65]/open[0]",
      old=u"`dpt` 가 누적자라는 것은 이름·줄번호로 추정(동작은 확정)",
      new=u"`dpt` 가 누적자라는 것은 dbg_value 로 확정된 「사실 서술」이다: 누적 phi/add 결과가 !47145(dpt, line 538)에 매번 붙고(m09.ll:52706·52736·52778·52814), 최종 udiv 결과 %77 만 !47143(tick, line 536)에 붙는다(52831)",
      evidence=u"m09.ll:52706 `#dbg_value(i64 %13, !47145` / 52736 `#dbg_value(i64 %29, !47145` / 52831 `#dbg_value(i64 %77, !47143`. !47143 = DILocalVariable(name: \"tick\", line 536) / !47145 = (name: \"dpt\", line 538) (m09.ll:117741·117743)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[65]/open[1]",
      old=u"_gcbc 에 define 있음, 미독해. 특히 cooltime 이 0 을 돌려줄 수 있는지(div0 패닉 경로 도달 가능성)",
      new=u"expected_damage_target·CastingTarget::check 는 미독해(미탐색). cooltime 3종은 18차 B 가 읽었다: 셋 다 `llvm.umax` 로 하한을 보장해 0 을 돌려주지 못하므로 본문의 div0 패닉 경로는 도달 불가 — attack_cooltime ≥3(_gcbc g06.ll:66510) · skill_cooltime ≥3(66405) · skill2_cooltime ≥1(66747)",
      evidence=u"_gcbc g06.ll:66510 `define noundef range(i64 3, 0) i64 @…Entity15attack_cooltime` + 본문 `%16 = tail call noundef i64 @llvm.umax.i64(i64 %15, i64 3)` / 66405 skill_cooltime `range(i64 3, 0)` umax 3 / 66747 skill2_cooltime `range(i64 1, 0)` umax 1",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[65]/open[4]", old=u"skill2_effect@tag / @Some.0.target 로 일치 확인", new=u"skill2_effect@tag / @Some.0.target 로 일치 확인 — 「사실 서술」이다",
      evidence=u"문면 자체가 확정 관측(tcxdict 일치)이라 open 이 아니라 notes 로 분류돼야 한다(G10 어휘)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[63]/open[1]",
      old=u"dloc 루트가 simulation.rs:1905 iter_champions 를 가리켜 iter_champions(team) 가 유력. 두 클로저(closure$0/1)가 같은 식인지는 IR 동일(hp*100/max>39)로만 확인",
      new=u"iter_champions(team).filter(closure$0).count() 인라인으로 확정된 「사실 서술」이다 — !38030 사슬 = `line 1905 in iter_champions ← line 597`(m09.ll:37743~37745 gep +480·[5 x ptr]) 이고 !38199 사슬 = closure$0(597) ← Filter ← count(iterator.rs:142) ← 597. 두 클로저는 IR 동일(hp*100/max >u 39; 37800~37802 / 37837~37839)",
      evidence=u"dloc m09.ll !38030: `line 1905 in iter_champions [simulation.rs]` ← `line 597 in should_keep_object_for_contested_wave_priority`. !38199: `line 597 in closure$0` ← filter.rs:138 ← … ← `line 142 in count<FilterMap<Iter<Option<&Entity>>…>>` ← 597. m09.ll:37745 `%25 = getelementptr inbounds nuw [5 x ptr], ptr %24, i64 %20`",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[62]/open[2]",
      old=u"담당 범위 밖(m15.ll:53341 / 55548 에 define 있음, 미독해)",
      new=u"18차 B 가 읽었다 — 「사실 서술」이다. 둘 다 player_champion[team 또는 1-team] 5칸을 언롤해 (a) max_hp==0 → div0 패닉 (b) HP% = hp*100/stat_cached.hp 가 min_hp_ratio 미만이면 제외(m15.ll:53464 / 55705 `icmp ult … %5`) (c) 제곱거리 ≤ range²(53497 `ule`, 적은 55738 `ugt` 로 제외) 를 센다. 적 쪽은 추가로 blackboard[1-team].is_recent_visible(game, player, e) 가 참이어야 한다(55582 `sub i64 1, %8` → 55666 gep → 55742 호출). 가시성 정의는 shared.is_recent_visible(★확정)",
      evidence=u"m15.ll:53455 `%20 = icmp eq i64 %19, 0` / 53464 `%26 = icmp ult i64 %25, %5` / 53497 `%44 = icmp ule i64 %43, %14`(%14 = 53425 `mul i64 %4, %4`) ; 55705 `%33 = icmp ult i64 %32, %5` / 55738 `%51 = icmp ugt i64 %50, %20` / 55742 `tail call noundef zeroext i1 @…Blackboard17is_recent_visible(ptr dereferenceable(744) %21, ptr noundef nonnull %15, ptr dereferenceable(816) %17, ptr dereferenceable(2528) %0, …)` / 55582 `%9 = sub i64 1, %8`",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[64]/open[3]",
      old=u"line_exists(m13.ll:53138)·is_near_line(_gcbc g09.ll)·v25_objective_far_split_pressure·v23_*_near_point 내부 — 담당 범위 밖, 미독해",
      new=u"line_exists(m13.ll:53138)·is_near_line(_gcbc g09.ll)·v25_objective_far_split_pressure 내부 — 담당 범위 밖, 미독해(「미탐색」이다). v23_*_near_point 는 18차 B 가 읽음 → specs[62] notes 참조(HP% < min 제외 · dist² ≤ range² · 적은 blackboard[1-team] 최근가시 판정)",
      evidence=u"m15.ll:53464 / 55705 / 53497 / 55738 / 55742 (specs[62] open[2] 정정과 동일 근거)",
      behavior_change=False, found_by="new", kind=u"보강")


# ═══════════════════════════════ #66 mem[16..18] sret store 줄 인용 정정 ═══════════════════
p.fix("/specs[66]/mem[16]/note", old=u"m04.ll:63698", new=u"m04.ll:63699 `store i32 %303, ptr %0`",
      evidence=u"m04.ll:63699 `store i32 %303, ptr %0, align 4, !dbg !73027` (63698 은 `store i8 %306, ptr %305` = ok)",
      behavior_change=False, found_by="new", kind=u"실오류")
p.fix("/specs[66]/mem[17]/note", old=u"m04.ll:63695~63697", new=u"m04.ll:63695~63698 (`%304 = icmp sge i32 %303, %5` → `%305 = getelementptr inbounds nuw i8, ptr %0, i64 4` → `store i8 %306, ptr %305`)",
      evidence=u"m04.ll:63695 `%304 = icmp sge i32 %303, %5` / 63696 `%305 = getelementptr inbounds nuw i8, ptr %0, i64 4` / 63697 `%306 = zext i1 %304 to i8` / 63698 `store i8 %306, ptr %305, align 4`",
      behavior_change=False, found_by="new", kind=u"실오류")
p.fix("/specs[66]/mem[18]/note", old=u"m04.ll:63700", new=u"m04.ll:63701 `store i32 %263, ptr %307`",
      evidence=u"m04.ll:63700 `%307 = getelementptr inbounds nuw i8, ptr %0, i64 8` / 63701 `store i32 %263, ptr %307, align 4`",
      behavior_change=False, found_by="new", kind=u"실오류")

# ═══════════════════════════════ brief_errors ═══════════════════
p.brief_error(u"§1 표의 #65 `open` 5 / #66 `open` 7 은 「이번 라운드에 볼 것」이라 적혔지만 #66 open[0] 은 shared.is_recent_visible 에 이미 ★확정 답이 있었다(G7 도 같은 것을 가리킴). 도시에가 §4 의 G7·G10 두 건이 **같은 항목**임을 말하지 않아 두 건으로 셀 뻔했다.")
p.brief_error(u"§4 G20 R4 는 「같은 노브가 값 39/40」이라 적었지만 #64 의 40 은 본문 리터럴이 아니라 콜리에 넘기는 인자이고 비교(`ult`)는 콜리 안에 있다 — 게이트가 `knobs.what` 문자열 동일성만 보므로 「값 통일」이 아니라 「이름에 리터럴/인자 구분을 담는 것」이 맞는 조치였다. 지시문의 「문면을 통일」은 이 방향을 명시하지 않았다.")
p.brief_error(u"§5 는 `open`/`notes` 에 insert/delete 가 안 된다고만 적었는데, 문면 치환(`/specs[i]/open[k]` + old 부분문자열)은 applypatch ② 경로로 된다(v2 unknown/still_unknown 문면 탐색). 이 사실이 없어 open 정정을 전부 산문으로 미룰 뻔했다.")
p.brief_error(u"#62·#65 의 `logic`/`consts.meaning`/`params.role` 안 `mNN.ll:LINE` 인용이 대부분 현행 .ll 과 어긋나 있었는데(#62 8곳 전부 · #65 5곳) 어떤 게이트도 이를 보지 않는다(G13 은 knobs.where 만). 검사기 제안: paramrole 의 P6(CITE+QIR 조각 대조)을 `logic`·`consts.meaning`·`open` 문면에도 돌리는 「G21 인용 대조」— `m\\d+\\.ll:(\\d+)(~\\d+)?` 뒤 백틱 IR 조각이 그 줄(범위)에 있는지.")

out = p.save()
print(out)
