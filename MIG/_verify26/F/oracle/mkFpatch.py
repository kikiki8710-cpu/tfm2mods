# -*- coding: utf-8 -*-
"""26차 F patch.json 생성 — mkpatch 참조구현 사용. python -X utf8 mkFpatch.py"""
import sys, io
sys.stdout.reconfigure(encoding='utf-8')
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch

p = mkpatch.Patch(round=26, batch="F")

# ───────────────────────── 212 AgentVerHamster::update_small_action ─────────────────────────
# G12 consts[11]
p.fix("/specs[212]/consts[11]/src_line", old=758, new=757,
      evidence=u"m14.ll:36871 `%335 = add nsw i8 %263, -15, !dbg !50337` · !50337 사슬 = small_action.rs:309(get_action) < lib.rs:757 (루트 757). 같은 명령 36864 `%331 = add nsw i8 %263, -15`(!dbg 없음 · CSE 복제) 도 소비자 36866~36867 `select … !dbg !49567`=L757. 본문 35955~37839 에 `-15` 리터럴은 이 둘뿐이고 758 루트 명령은 0건",
      behavior_change=False, found_by="reused")
p.fix("/specs[212]/consts[3]/meaning",
      old=u"failed_action 보존 창: 기록 tick + 60 ≥ now 이면 유지(1초 쿨다운). aux m06.ll:2364/2428 (retain 클로저 인라인)",
      new=u"failed_action 보존 창 임계(틱): 기록 tick + 60 < now 이면 제거(= t+60 ≥ now 유지 · 1초 쿨다운). aux m06.ll:2364 `%17 = add i64 %16, 60` · 2366 `%19 = icmp ult i64 %17, %18(game.tick())` 참→제거(memmove 압축 2373) / 2428~2429 동일(두 번째 루프)",
      evidence=u"m06.ll:2364~2366 · 2428~2429 (retain 클로저 인라인). `임계` 어휘 부재로 kind 미상이었다",
      behavior_change=False, found_by="reused", kind=u"보강")
p.fix("/specs[212]/mem[6]/name",
      old=u"small_action.<cast>.is_act (+0x10)",
      new=u"small_action@Attack.0.is_act (+0x10 · Skill/Skill2/Ult 도 같은 자리)",
      evidence=u"tcxdict --enum SmallActionPlay: Attack/Skill/Skill2/Ult 페이로드 24B(start@0·target@8·is_act@0x10) · m14.ll:36665 `gep %0, 10344`=0x2868=0x2858+0x10. `<cast>` 자리표시가 tcxaudit 부분일치를 냈다",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[212]/mem[33]/note",
      old=u"L695 retain(aux m06.ll · 60틱=1초 쿨다운) / L702~705(포기 Trace) / L720~723(무진전 RunAway·Recall) / L758~761(미발동 캐스트가 next 와 다를 때) / L767~770(Trace 가 비-Trace·비-캐스트 next 로 교체될 때). push 는 grow_one 경유 → ptr/cap/len 전부 변동 가능",
      new=u"L695 retain(aux m06.ll · 60틱=1초 쿨다운) / L702~705(포기 Trace) / L720~723(무진전 RunAway·Recall) / L758~761(미발동 캐스트가 next 와 다를 때) / L767~770(Trace 가 비-Trace·비-캐스트 next 로 교체될 때). push 는 grow_one 경유 → ptr/cap/len 전부 변동 가능. ★(26차F) 원소 32B `{ i64 tick, { i64 discr, [2 x i64] } }` 의 push 사이트별 기록 바이트: L705(36145~36152) tick@0·discr=4@8·target@16 3 store — **+0x18 미기록** / L723(36503~36512) 4 store 이나 discr %160=phi(태그3·4 → 0 RunAway) · +0x10/+0x18 = **phi undef 인입 store**(%158/%159 · 36302~36303) / L761(37261~37270) discr %469∈6..9 · +0x10 target · **+0x18 undef**(%468) / L770(37550~37559) %581=4 · +0x10 target · **+0x18 undef**(%580). 기존 원소 갱신은 +0 tick 만(36107 L703 · 36467 L721 · 37092 L759 · 37381 L768) ⟹ HEAP_SUBST 대조 시 원소 +0x18(RunAway 는 +0x10 도) 제외",
      evidence=u"m14.ll:36145~36152 · 36302~36303(`%158 = phi i64 [ … [ undef, %99 ] …`) · 36503~36512 · 37228~37230(%467/%468/%469 phi) · 37261~37270 · 37550~37559 · x.0 갱신 36107/36467/37092/37381",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[212]/mem[41]/note",
      old=u"L793 (m14.ll:37827) — 5비트 마스크",
      new=u"L793 (m14.ll:37827) — 5비트 마스크. count_nearby_enemies 는 internal fastcc ArgumentPromotion(m14.ll:37842 define `range(i16 0, 32) i16 (i64 %0, i64 %1, ptr %2, ptr %3, ptr %4)`) · 호출 37822 인자 = (champ.x +0x660, champ.y +0x668, player %2, data.cache %18, data.blackboard %687=data+16) — 소스 (&Entity,&PlayerState,&OperationData) 와 배치가 다르다(exe 인자 대조 시 주의)",
      evidence=u"m14.ll:37816~37822 · 37842 define",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[212]/sig/params[1]/note", old=u"",
      new=u"IR 속성 없음(`noalias noundef nonnull align 16 dereferenceable(320)` = &mut) · 본 함수 직접 소비(gen_range/choose) 0 · L730 get_small_action(m14.ll:36529) 에만 전달",
      evidence=u"m14.ll:35955 define · 본문 %1 등장 1회(36529)", behavior_change=False, found_by="new", kind=u"보강", force=True)
p.fix("/specs[212]/sig/params[2]/note", old=u"+0x930 team · +0x9c0 position",
      new=u"IR `readonly` · +0x930 team(37289) · +0x9c0 position(37570) — L774 player_champion 인덱스 · L730 get_small_action(36529) · L793 count_nearby_enemies(37822) 전달",
      evidence=u"m14.ll:37289 · 37570 · 36529 · 37822", behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[212]/sig/params[3]/note", old=u"+0 cache · +8 context · +16 blackboard",
      new=u"IR `readonly` · +0 cache(36002 → game 팻포인터·player_champion) · +8 context(37600 → +0x3b debug) · +16 blackboard(37820 → count_nearby_enemies) · get_small_action(36529)·possible_risk(37705) 에 그대로 전달",
      evidence=u"m14.ll:36002 · 37600 · 37820 · 36529 · 37705", behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[212]/exe/evidence", old=u"fp",
      new=u"fp · argscan 0xe8e560(26차F): 스택 인자 0 + 레지스터 4 = exe 인자 4 = IR define 인자 4(self·rnd·player·data · ArgumentPromotion 없음) · 프롤로그 push 8 + sub rsp 0x3738",
      evidence=u"python -X utf8 argscan.py 0xe8e560 → 「스택 인자 0개 + 레지스터 4 = exe 인자 4」", behavior_change=False, found_by="reused", kind=u"보강")

# ───────────────────────── 213 PassiveLinePlan::v46_stage1 ─────────────────────────
# G12 consts[16] 오탐
p.fix("/specs[213]/consts[16]/meaning",
      old=u"(aux closure$1) vtable+0x28 = AbstractGame::tick 슬롯(divtable 실측) — game.tick() > tower_disable_tick 이면 타워 제외. 오프셋이지 판정값 아님",
      new=u"(aux closure$1) vtable+0x28 = AbstractGame::tick 슬롯(divtable 실측) — game.tick() > tower_disable_tick 이면 타워 제외(aux m04.ll:64208 `icmp ugt i64 %11, %13` 참→제외). 오프셋이지 판정값 아님(★G12 오탐: 이 40 은 aux call_mut 심 m04.ll:64202 `%9 = getelementptr inbounds nuw i8, ptr %8, i64 40, !dbg !73407` 에만 있고 !73407 = closure$1 line 685 ⊂ inlinedAt function.rs:298(call_mut 루트) 이라 본체 루트줄 규약으로는 안 잡힌다. 본체의 40 은 L708 hp% 임계(17683 `icmp ult i64 %596, 40`)·L691 gep 로 별개)",
      evidence=u"m04.ll:64202 + dloc !73407 → 「line 685 in closure$1 [passive_line.rs] / line 298 in call_mut [function.rs]」 · 66431 `%10 = gep %9, i64 40, !dbg !75728` 동일(두 번째 call_mut 심). G12 후보 691/708 은 본체의 다른 40",
      behavior_change=False, found_by="reused", kind=u"오탐")
# G5/G16 구조적 오탐 — ArgumentPromotion. params[0] role 에 근거 문면, i 값 정정
p.fix("/specs[213]/sig/params[0]/note",
      old=u"레이아웃(본문 관측 m04.ll:16576~16627·17180~17291): +0x0 ptr(비었을 때 dangling 8) · +0x8 &Bump(=ctx.pool) · +0x10 cap · +0x18 len. 4워드 전부 live. 원소 24B (usize,usize,usize) = (+0 e.id, +8 my_die, +16 kill_dps) 패딩 없음 · 전부 live(17284~17288)",
      new=u"레이아웃(본문 관측 m04.ll:16576~16627·17180~17291): +0x0 ptr(비었을 때 dangling 8 · 16576 `store ptr inttoptr (i64 8 to ptr)`) · +0x8 &Bump(=ctx.pool %27) · +0x10 cap · +0x18 len(16620 memset 0 16B). 4워드 전부 live · sret memcpy 32B 16661(L777) 단일 반환점. 원소 24B `{ i64, i64, i64 }`(17283 gep) = (+0 e.id %151, +8 my_die %268, +16 kill_dps %761) 패딩 없음 · 전부 live(17284~17288). ★(26차F · G5/G16 구조적 오탐) define = `internal fastcc` ArgumentPromotion: tcx 11인자 → IR 15슬롯 — self(&PassiveLinePlan) 는 미사용 삭제(DeadArgElim) · player → %2 info.team(+0x930) i64 + %3 info.position@tag(+0x9c0) i32 · data → %4 cache(+0) + %5 context(+8) · only Option<&[usize]> → %11 ptr(null=None)+%12 len(None 이면 undef) · near_enemies &bumpalo::Vec<&Entity> → %13 ptr+%14 len. 호출 3곳(m04.ll:21664·21883·22374 PassiveLinePlan::update) 전부 %64=player+2352 · %68=player+2496 · %70=data+0 · %79=data+8 을 넘긴다. exe 도 같은 배치(argscan 0xd26900: 레지스터 4 + 스택 11 = 15)",
      evidence=u"m04.ll:16364 define(15 인자 · %0 sret writeonly dereferenceable(32)) · 호출부 19164~19195(%63/%64 gep 2352 · %67/%68 gep 2496 · %70 load %4 · %78/%79 gep 8) · 21664/21883/22374 invoke 인자 · argscan.py 0xd26900",
      behavior_change=False, found_by="reused", kind=u"오탐")
p.fix("/specs[213]/sig/params[12]/i", old=11, new=12,
      evidence=u"m04.ll:16364 define: only.len = `i64 %12`(11 은 only.data_ptr). 팻포인터 두 슬롯이 같은 i 를 갖고 있었다 — IR %N 으로 정정", behavior_change=False, found_by="new")
p.fix("/specs[213]/sig/params[13]/i", old=12, new=13,
      evidence=u"m04.ll:16364 define: near_enemies.data_ptr = `ptr readonly captures(address) %13`", behavior_change=False, found_by="new")
p.fix("/specs[213]/sig/params[14]/i", old=12, new=14,
      evidence=u"m04.ll:16364 define: near_enemies.len = `i64 %14` (16523 `gep ptr, ptr %13, i64 %14` 끝 포인터)", behavior_change=False, found_by="new")
# (213 params[1..14] 의 v2 `role` 은 이미 IR 속성·용처를 담고 있다 — v3 role(=v2 note) 만 비어 보인 것. 추가 안 함)
# G20 R4 — 노브 where 에 icmp 인용(외연 150001 = stage2 `ugt 150000` 과 동일)
p.fix("/specs[213]/knobs[5]/where", old=u"passive_line.rs:686 (closure$1)",
      new=u"passive_line.rs:686 (closure$1 · aux m04.ll:64222 `icmp ult i64 %23, 150001` · 66460 동일)",
      evidence=u"m04.ll:64222 `%24 = icmp ult i64 %23, 150001, !dbg !73408` · 66460 `%25 = icmp ult i64 %24, 150001`. G20 R4: #220 knobs[2] `passive_line.rs:835` 150000 은 `icmp ugt i64 %186, 150000`(m04.ll:18493) 이라 외연(≤150000) 동일 — sharedchk._extent 가 icmp 인용을 읽으면 억제된다(220 쪽 where 에도 인용 필요)",
      behavior_change=False, found_by="reused", kind=u"보강")
p.fix("/specs[213]/knobs[5]/effect", old=u"내리면 억지 타워가 줄어 e_die 증가 → 커밋터 증가",
      new=u"내리면 억지 타워가 줄어 e_die 증가 → 커밋터 증가. IR 리터럴 150001(`< 150001` ⇔ 소스 `<= 150000`) — v46_stage2 L835 의 `> 150000` 과 같은 반경",
      evidence=u"m04.ll:64222", behavior_change=False, found_by="reused", kind=u"보강")
# G20 R2 — committers 원소 행 D9-OFF 규약(단일 오프셋)으로 분할: mem[51] → e.id, 삽입 my_die/kill_dps
p.fix("/specs[213]/mem[51]/offset", old=u"ptr[len]+0 / +8 / +16", new=u"0x0",
      evidence=u"D9-OFF: 원점 = 원소(24B `{ i64, i64, i64 }` 17283 gep). e.id store 17284 `store i64 %151, ptr %406`", behavior_change=False, found_by="reused", kind=u"보강")
p.fix("/specs[213]/mem[51]/name", old=u"e.id / my_die / kill_dps", new=u"e.id",
      evidence=u"17284 (%151 = e+0x5c0 load 16657)", behavior_change=False, found_by="reused", kind=u"보강")
p.fix("/specs[213]/mem[51]/base", old=u"committers 원소(bump 힙)", new=u"committers 원소(24B)",
      evidence=u"#220 mem[42] 와 같은 원점 이름으로 통일(G20 R1/R2)", behavior_change=False, found_by="reused", kind=u"보강")
p.fix("/specs[213]/mem[51]/value", old=u"m04.ll:17284~17288 (L775) · len+=1 (17291)", new=u"m04.ll:17284 (L775) · len+=1 (17291) · ptr[len] 원소(bump 힙 · cap==len 이면 reserve_internal_or_panic 17192)",
      evidence=u"17284~17291", behavior_change=False, found_by="reused", kind=u"보강")
p.errors.append({"op": "insert", "path": "/specs[213]/mem", "at": 52, "guard": u"my_die", "guard_key": "name",
                 "new": {"base": u"committers 원소(24B)", "offset": u"0x8", "name": u"my_die", "dir": "w",
                         "value": u"m04.ll:17286 `store i64 %268, ptr %407` (L775) — my_die = sat_sub(my_hp, kill_nuke)*60/max(kill_dps,1)(16958~16964)",
                         "note": u"ptr[len]+8 · 26차F 분할(D9-OFF)", "chk": u"확인불가(tcx 사전에 타입 없음)", "ev": 4},
                 "kind": u"보강", "evidence": u"m04.ll:17285~17286", "behavior_change": False, "found_by": "reused"})
p.errors.append({"op": "insert", "path": "/specs[213]/mem", "at": 53, "guard": u"kill_dps", "guard_key": "name",
                 "new": {"base": u"committers 원소(24B)", "offset": u"0x10", "name": u"kill_dps", "dir": "w",
                         "value": u"m04.ll:17288 `store i64 %761, ptr %408` (L775) — 적측 초당 DPS 합(L717 누적 phi %761)",
                         "note": u"ptr[len]+16 · 26차F 분할(D9-OFF) · stage2 는 이 칸을 읽지 않는다(#220 mem[42])", "chk": u"확인불가(tcx 사전에 타입 없음)", "ev": 4},
                 "kind": u"보강", "evidence": u"m04.ll:17287~17288", "behavior_change": False, "found_by": "reused"})
p.fix("/specs[213]/exe/evidence", old=u"fp",
      new=u"fp · argscan 0xd26900(26차F): 레지스터 4 + 스택 11 = exe 인자 15 = IR define 15(ArgumentPromotion 배치 그대로 · entry_rsp+0x20..+0x70 = %4..%14 순서 일치 · %10 range_gate 는 `cmp byte ptr` 1B)",
      evidence=u"python -X utf8 argscan.py 0xd26900 → 「스택 인자 11개 + 레지스터 4 = exe 인자 15」 슬롯별 크기 8/8/…/1/8/8/8/8", behavior_change=False, found_by="reused", kind=u"보강")

# ───────────────────────── 214 BattleSubPlan::score ─────────────────────────
# G19 knobs[0]/[1]/[13]: IR 리터럴(음수) 로 · 극성 문면 정정 · knobs[14] where 에 phi 인용
p.fix("/specs[214]/knobs[0]/value", old=6, new=-6,
      evidence=u"m02.ll:37317 `%91 = add i64 %26, -6, !dbg !44598`(L802) — IR 리터럴은 -6(소스 `base - 6`). 오라클 R2b(sup=적 e0 근접·goal Trace) score-base = -6 · R1/R2(sup none/아군) 0",
      behavior_change=False, found_by="reused")
p.fix("/specs[214]/knobs[0]/effect", old=u"올리면 서포트 압박(v15)이 가능한 상황에서 도주/귀환 후보가 더 억제된다",
      new=u"음수 리터럴(감점). 절댓값을 키우면(-6 → -12) 서포트 압박(v15_can_keep_support_pressure 참)이 가능한 상황에서 도주/귀환/AroundRunAway 후보가 더 억제된다 · 0 이면 무효. 오라클 실행 확인(26차F o214 R2b: delta -6 · R1/R2 0)",
      evidence=u"m02.ll:37317 · o214_cases.log R1/R2/R2b", behavior_change=False, found_by="reused", kind=u"보강")
p.fix("/specs[214]/knobs[0]/where", old=u"battle.rs:802", new=u"battle.rs:802 (m02.ll:37317 `add i64 %26, -6`)",
      evidence=u"m02.ll:37317", behavior_change=False, found_by="reused", kind=u"보강")
p.fix("/specs[214]/knobs[1]/value", old=30, new=-30,
      evidence=u"m02.ll:37408 `%150 = add i64 %26, -30, !dbg !44656`(L815). 오라클 T5(avoid 양쪽·far·!ctf_me·ctf_t·ctwt) delta -30 · T5b(sup=e0 겹침) -30(+10 아님 · L815 return 우선)",
      behavior_change=False, found_by="reused")
p.fix("/specs[214]/knobs[1]/effect", old=u"올리면 사거리 밖·타워 안 대상을 향한 Trace 가 더 강하게 배제된다(avoid_unnecessary_tower_trace 플래그일 때만)",
      new=u"음수 리터럴(감점). 절댓값을 키우면 사거리 밖·타워 안 대상을 향한 Trace 가 더 강하게 배제된다(self.avoid_unnecessary_tower_trace && action.avoid_unnecessary_tower 둘 다 참일 때만 · 감점 시 return 이라 L819 +10 과 겹치지 않음). 오라클 실행 확인(26차F o214 T5/T5b -30 · T6/T7 플래그 한쪽만 → 0)",
      evidence=u"m02.ll:37408 · o214_cases.log T5/T5b/T6/T7", behavior_change=False, found_by="reused", kind=u"보강")
p.fix("/specs[214]/knobs[1]/where", old=u"battle.rs:815", new=u"battle.rs:815 (m02.ll:37408 `add i64 %26, -30`)",
      evidence=u"m02.ll:37408", behavior_change=False, found_by="reused", kind=u"보강")
p.fix("/specs[214]/knobs[13]/value", old=30, new=-30,
      evidence=u"m02.ll:38289 `%585 = add i64 %568, -30`(L776 · !dbg 없음, 소비 38291 select/phi). 오라클 U3(ratio 71·300<710) -30 · U5b(301>300·ratio 75) -30 · U4(ratio 70) 0 · U5(300==300) 0",
      behavior_change=False, found_by="reused")
p.fix("/specs[214]/knobs[13]/effect", old=u"올리면 CC 도 자기버프도 없는 궁을 체력 넉넉한 챔피언에게 쓰는 걸 더 억제",
      new=u"음수 리터럴(감점). 절댓값을 키우면 CC 도 자기버프도 없는 궁을 체력 넉넉한 챔피언(hp*100/max_hp > 70 && dmg*3 < hp)에게 쓰는 걸 더 억제. 오라클 실행 확인(26차F o214 U3/U5b -30 · U4/U5 경계 0 · U6 cc 있음 0)",
      evidence=u"m02.ll:38285~38291 · o214_cases.log U3/U4/U5/U5b/U6", behavior_change=False, found_by="reused", kind=u"보강")
p.fix("/specs[214]/knobs[13]/where", old=u"battle.rs:776", new=u"battle.rs:776 (m02.ll:38289 `add i64 %568, -30`)",
      evidence=u"m02.ll:38289", behavior_change=False, found_by="reused", kind=u"보강")
p.fix("/specs[214]/knobs[14]/where", old=u"battle.rs:622/654/658/696/700/738/742",
      new=u"battle.rs:622/654/658/696/700/738/742 (m02.ll:38400 phi `[ -99999, %346 ] … [ -99999, %48 ] …` 7 인입)",
      evidence=u"G19 오탐(도구): -99999 는 38400 반환 phi 의 상수 인입뿐이라 소스 줄 앵커 창에 리터럴이 없다. 오라클 A3/A3b/K2/K4/K4c/U8/U9 score=-99999", behavior_change=False, found_by="reused", kind=u"오탐")
p.fix("/specs[214]/knobs[14]/effect", old=u"대상 소실·도주중 돌진스킬 — 사실상 선택 불가(다른 후보가 모두 이보다 낮을 일이 없음)",
      new=u"대상 소실·도주중 돌진스킬 — 사실상 선택 불가(다른 후보가 모두 이보다 낮을 일이 없음). 오라클 실행 확인(26차F o214: Attack/Skill/Ult 대상 소실 A3·K2·U9 / 돌진(RushEffect speed≠0 && applyed_effect 비어있지 않음)+goal RunAway K4·U8 · Trace 대상 소실은 -99999 아님 T4)",
      evidence=u"o214_cases.log", behavior_change=False, found_by="reused", kind=u"보강")
p.fix("/specs[214]/exe/evidence", old=u"fp",
      new=u"fp · pub 심볼(define 속성 없음 · 인자 8 = tcx 8, ArgumentPromotion 없음 · m02.ll:37119)",
      evidence=u"m02.ll:37119 define", behavior_change=False, found_by="reused", kind=u"보강")

# ev 상향: 오라클 실행 확증 행
evs214 = [
 ("/specs[214]/knobs[3]",  u"오라클 실행 확인(26차F o214 T1 far → +10 · T2 근접 0 · T1b dist==mr 0 · T1c dist=mr+1 +10)"),
 ("/specs[214]/knobs[4]",  u"오라클 실행 확인(26차F o214 A1/K6/K7c/U7 Nexus → +200 · 4 arm 전부)"),
 ("/specs[214]/knobs[5]",  u"오라클 실행 확인(26차F o214 A8b/A9b/K9b Epic·Serpen dmg≥hp → +100 · 경계 dmg==hp 포함)"),
 ("/specs[214]/knobs[6]",  u"오라클 실행 확인(26차F o214 A8/A9/K9 dmg<hp → max(score/3,1)=1)"),
 ("/specs[214]/knobs[8]",  u"오라클 실행 확인(26차F o214 A6b/A6c/K10b/U10b 적 정글 attack_effect 기준 dmg≥hp → +80 · A6/K10/U10/U10c → max(score,1) · A7 내 정글 → max(score,1) · Skill/Ult arm 도 판정은 attack_effect 피해)"),
 ("/specs[214]/knobs[9]",  u"오라클 실행 확인(26차F o214 K5 dist 30000 → ×5 · K5d 70000 → ×1 · K5e 80000 → 0 · K5f 79999 → 0)"),
 ("/specs[214]/knobs[10]", u"오라클 실행 확인(26차F o214 K5 cc=120 → (120*3+3)*5=1815 · K5g cc=1 → 30 · U13 궁 Stun 도 1815 · K5c goal Trace → 0)"),
 ("/specs[214]/knobs[11]", u"오라클 실행 확인(26차F o214 U2 dmg 100≥hp 50 → +40 · U2b dmg==hp → +40)"),
 ("/specs[214]/knobs[12]", u"오라클 실행 확인(26차F o214 U3 ratio 71 → -30 · U4 ratio 70 → 0)"),
 ("/specs[214]/consts[9]",  u"오라클 실행 확인(26차F o214 A3/K2/U9/K4/U8 -99999)"),
 ("/specs[214]/consts[20]", u"오라클 실행 확인(26차F o214 A1/K6/K7c/U7 +200)"),
 ("/specs[214]/consts[21]", u"오라클 실행 확인(26차F o214 A8b/A9b/K9b +100)"),
 ("/specs[214]/consts[22]", u"오라클 실행 확인(26차F o214 A8/A9/K9 max(score/3,1))"),
 ("/specs[214]/consts[26]", u"오라클 실행 확인(26차F o214 A6b/A6c/K10b/U10b +80)"),
 ("/specs[214]/consts[31]", u"오라클 실행 확인(26차F o214 K5/K5d/K5e/K5f)"),
 ("/specs[214]/consts[32]", u"오라클 실행 확인(26차F o214 K5d 70000→1 · K5f 79999→0)"),
 ("/specs[214]/consts[33]", u"오라클 실행 확인(26차F o214 K5 1815 · K5g 30)"),
 ("/specs[214]/consts[34]", u"오라클 실행 확인(26차F o214 U2/U2b +40)"),
 ("/specs[214]/consts[36]", u"오라클 실행 확인(26차F o214 U3 71→-30 · U4 70→0)"),
 ("/specs[214]/consts[37]", u"오라클 실행 확인(26차F o214 U5 300==300→0 · U5b 301→-30)"),
 ("/specs[214]/consts[29]", u"오라클 실행 확인(26차F o214 K7 level 1 Skill2 → 패닉(interaction_score 안 action_score.rs 에서 먼저) · K7b level 3 정상)"),
 ("/specs[214]/consts[30]", u"오라클 실행 확인(26차F o214 U1 level 4 Ult → 패닉 · U2 level 5 정상)"),
]
for path, ev in evs214:
    p.ev(path, evidence=ev, to=2, frm=4, found_by="new")
# 214 sig 인자 역할 오라클 확증(self 읽기 필드)
p.ev("/specs[214]/sig/params[0]", evidence=u"오라클 실행 확인(26차F o214 self_diff 전 케이스 빈 문자열 = 48B 무변경 · readonly 정합 · support_target/goal/avoid 분기 T1/T5/T6/K4/U8 실측)", to=2, frm=4, found_by="new")
p.ev("/specs[214]/sig/params[6]", evidence=u"오라클 실행 확인(26차F o214 태그 19 Stop/3 RunAway/4 Recall/14 Trace/15~18 캐스트 전 arm 진입 · +0x60 Trace.target · +0x90 avoid(new_avoid_tower=1 T5 vs new=0 T7))", to=2, frm=4, found_by="new")

p.brief_error(u"§4 G16 [213] 「15행 → 12행이어야 한다」와 G5 「11 vs 14」는 둘 다 internal fastcc ArgumentPromotion(self 삭제 · player→team+pos · data→cache+ctx · 팻포인터 2슬롯×2)이라 구조적이다 — 게이트가 `define internal fastcc` 면 tcx 인자 수와 비교하지 말고 exe(argscan) 와 비교해야 한다. 이번 도시에에서 G16/G5 를 「문면·표 정정」으로 분류한 것은 이 함수에 맞지 않았다.")
p.brief_error(u"§4 G12 [213] consts[16] 「실제 후보 691/708」은 본체의 다른 40(L708 hp% 임계 · L691 gep)이고, 명세의 40 은 aux call_mut 심 안에 있다 — srclinecheck 가 `ir.aux` 범위를 안 읽거나, 읽더라도 call_mut 루트(function.rs:298)에서 멈춰 closure 줄(685)을 못 본다. aux 의 루트는 「closure$N 의 line」으로 잡아야 한다.")
p.brief_error(u"§4 G19 [214] knobs[14] -99999 는 반환 phi(38400) 상수 인입뿐이라 소스 줄 앵커 창에 절대 안 보인다(G12 교정 ②와 같은 형태). 오탐이며, where 에 phi 줄 인용을 넣어 억제했다.")
p.brief_error(u"§4 G20 R4 [213]/[220] 150001 vs 150000 은 `ult 150001` ⇔ `ugt 150000` 외연 동일이다. sharedchk._extent 는 양쪽 where/effect 에 icmp 인용이 있어야 억제한다 — 213 쪽은 넣었고 220 knobs[2].where 에 `m04.ll:18493 \\`icmp ugt i64 %186, 150000\\`` 인용이 있어야 닫힌다(배치 H 몫). 도구 쪽 대안: 인용이 한쪽만 있어도 consts 의 같은 값 행(213 consts[15] `< 150001 ⇔ ≤150000`)에서 외연을 읽게.")
p.brief_error(u"§4 G20 R2 [213]/[220] committers 원소 — 213 mem[51] 을 D9-OFF 단일 오프셋 3행으로 갈랐지만 220 mem[42] 가 여전히 `0x0 / 0x8` 복합 표기라 R2 는 220 쪽 분할(배치 H)까지 남는다.")
p.brief_error(u"★도구: 도시에 `sig.params` 「역할」열 = v2 `note`(mkspec3 role=p.get('note'))이고 v2 `role`(작성자의 IR 속성·용처 문면 — 213 은 14행 전부 채워져 있다)은 도시에에 안 실린다 → 배치가 빈 칸으로 오인해 다시 쓴다. 게다가 applypatch 는 `/…/role` 경로에서 v2 행에 `role` 키가 있으면 그쪽을 겨냥한다(V2KEY 우선 규칙) → v3 문면(=note)으로 쓴 old 가 불일치(내 dry-run 실패 3건) · old=\"\" 로 쓰면 `str.replace(\"\", new)` 가 **문자 사이마다 삽입**해 v2 role 을 오염시키는데 dry-run 은 성공으로 찍는다(14건 그렇게 찍혔다 · 제출 전 자체 적발해 `/note` 경로로 바꿨다). 규약을 「v3 role ↔ v2 note 고정」으로 하거나 v3 에 v2 role 을 별도 열로 실어야 한다.")
p.brief_error(u"지시문 ⑥ 「SubPlan self 와 ScoreParameter 는 조립 가능」은 맞았고, 추가로 SmallActionPlay 도 pub 생성자(SmallActionAttack/Skill/Skill2/Ult/Trace/RunAway/Recall::new · Stop 은 단위 variant)로 직접 조립된다 — action_candidates 를 거치지 않아도 arm 별 진입이 된다. RushEffect 는 `speed!=0 && applyed_effect.len()!=0` 이어야 expected_rush_effect 가 참(g08.ll:108954) — 첫 시도(빈 Vec)는 그래서 거짓 음성이었다(2회째 정정).")
p.save()
