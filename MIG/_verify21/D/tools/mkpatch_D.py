# -*- coding: utf-8 -*-
import sys; sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch
S = "/specs[111]"
p = mkpatch.Patch(round=21, batch="D")

# ── G12 (src_line) ─────────────────────────────────────────────
p.fix(S + "/consts[93]/src_line", old=1349, new=1352,
      evidence=u"m13.ll:25643 `store i64 7812735363465179764, ptr %4312, !dbg !33698` — !33698(line 551 copy_nonoverlapping)→…→!25766(string.rs:2907 to_string)→!25764 = DILocation(line: 1352, scope: !23749) 루트. 1349 는 `prev.contains(\"Hunt\")||contains(\"Epic\")` 분기줄이고 \"try_kill\".to_string() 리터럴은 else 절 1352 에 있다(IR 사슬에 1352 만 존재·후보 유일)",
      behavior_change=False, found_by="reused")
p.fix(S + "/consts[96]/src_line", old=1453, new=1454,
      evidence=u"m13.ll:26894 `store i32 1684367716, ptr %4671, !dbg !34499` — !34499(line 551)→…→!25890(string.rs:2907)→!25888 = DILocation(line: 1454, scope: !23806) 루트. 1453 은 `if deaths>0` 줄, \"died\" 리터럴 alloc 은 1454(후보 유일)",
      behavior_change=False, found_by="reused")

# ── logic: 1450 사다리 극성(target 생존/소멸) ─────────────────────
p.fix(S + "/logic",
      old=u'else if my_hp_end% < 30 {("low_hp_retreat",0, target.is_none())} else if target.is_none() {("target_escaped",0,true)} else {("target_died",0,false)}',
      new=u'else if my_hp_end% < 30 {("low_hp_retreat",0, target.is_some())} else if target.is_some() /*get_entity_by_id Some = 표적 생존*/ {("target_escaped",0,true)} else /*None = 표적 소멸*/ {("target_died",0,false)}',
      evidence=u"m13.ll 블록 %4586/%4588 `br i1 %4486(=icmp eq ptr %4484(get_entity_by_id), null)` → None 경로 %4590→%4594→%4597→%4600: try_allocate_in(11B)+memcpy @anon.…65 = c\"target_died\"(11B) · Some 경로 %4592→%4614→%4617→%4619: 14B @anon.…66 = c\"target_escaped\" · %4653 phi target_escaped i8 = [1,%4628(Some·hp≥30)] [%4602,%4639(low_hp: phi 1@%4617 Some / 0@%4597 None)] [0,%4611(None)] — 명세는 None↔Some 이 뒤집혀 있었다",
      behavior_change=True, found_by="new")
p.fix(S + "/logic",
      old=u"1429 target None 이면 = 시작 hp 전부",
      new=u"1429 target None(=표적 소멸) 이면 = 시작 hp 전부(select %4486)",
      evidence=u"m13.ll 블록 %4496 `%4513 = select i1 %4486, i64 %4511, i64 %4512`(1429) — %4486 = get_entity_by_id 결과 null 판정. 1450 사다리 정정과 같은 술어",
      behavior_change=False, found_by="new", kind=u"보강")

# ── logic: 1254 인자 — self 가 아니라 &self.data(GoalData@0x0) · &mut self.plan ──
p.fix(S + "/logic",
      old=u"1254 let mut sub: SubPlan(72B) = self.plan.sub_plan(version, rnd, player, data, self, &self.team_plan(0xf8), &self.positioning_score(0x990), debug)   // 자식 명세, 계약만",
      new=u"1254 let mut sub: SubPlan(72B) = BigPlan::sub_plan(&mut self.plan(0x5e8, %1930 readonly 없음), version, rnd, player, data, &self.data(GoalData 0x0 = %0 readonly), &self.team_plan(0xf8), &self.positioning_score(0x990), debug)   // 자식 명세, 계약만 — self.plan 은 &mut 로 넘어가 내부 쓰기 가능(1270/1282 도 동일)",
      evidence=u"m13.ll:24363 `invoke @…BigPlan8sub_plan(sret(72) %145, ptr %1930, i64 %3906, ptr %2, ptr readonly %3, ptr readonly %4, ptr readonly %0, ptr %1652, ptr readonly %2007, ptr %5)` · tcx sig = fn(&mut BigPlan, usize, &mut StdRng, &PlayerState, &OperationData, &GoalData, &TeamPlan, &PositioningScoreData, &mut DebugFrameData) · tcxdict LegacyPlanHandler 0x0 data: GoalData(248B) ⟹ 6번째 인자 %0 은 &self.data 이지 self 가 아니다 · %1930 = self+1512(0x5e8) 에 readonly 없음 = &mut",
      behavior_change=False, found_by="new")

# ── logic: 1279 초기화 정밀화 (런타임 6168B sweep 재료) ─────────────
p.fix(S + "/logic",
      old=u"// tag 3, 280B 페이로드 0-초기화 + 5개 빈 Vec(ptr=8) + line@+0x11e",
      new=u"// tag 3(store i64 3 @+0) · 페이로드: v46_flee_entry tag@+8=0(None; 페이로드 +0x10~+0x20 미기록) · 5개 빈 Vec(cap 0·ptr=8) · +0x48~+0x120 memset 0 · line@+0x11e. ⚠+0x10~+0x20 과 +0x120~+0x180(enum 꼬리) 은 스택 잔재 그대로 self.plan 으로 memcpy 됨 → 0x5f8~0x608·0x708~0x768 은 비결정",
      evidence=u"m13.ll 블록 %3990(24592~24650): memset(%147+48,16)·(+72,16)·(+96,16)·(+120,16)·(+144,136)·(+280,6) · store i64 0 @+8·@+32 · store ptr 8 @+40/+64/+88/+112/+136 · store i8 line @+286 · store i64 3 @+0 — +16~+32 와 +288~+384 에 store/memset 없음. tcxdict PassiveLinePlan: 0x0 v46_flee_entry Option<(usize,u8)>(24B) → +0x8 tag(None=0)·페이로드 미초기화",
      behavior_change=False, found_by="new", kind=u"보강")

# ── logic: 1332/1339/1413/1420 checked 나눗셈(1303 만 panic) ────────
p.fix(S + "/logic",
      old=u"1332         (my_hp%, my_max) = my_champ.map(|c| (c.hp*100/c.max, c.max)).unwrap_or((0,0))",
      new=u"1332         (my_hp%, my_max) = my_champ.map(|c| (if c.max==0 {0} else {c.hp*100/c.max}, c.max)).unwrap_or((0,0))   // 1303 과 달리 max 0 이어도 panic 없음(icmp eq 0 → 0)",
      evidence=u"m13.ll 블록 %4251 `%4255 = icmp eq i64 %4254, 0; br i1 %4255, label %4261, label %4256` → %4261 phi %4262 = [0,%4251] · %4263(max) = [%4254,%4251] (주석본 ;L1332<1162<1330). 1303 은 %4133 panic_const_div_by_zero 로 다름",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/logic",
      old=u"1413     (my_hp_end%, my_hp_now) = my_champ.map(|c| (c.hp*100/c.max, c.hp)).unwrap_or((0,0))",
      new=u"1413     (my_hp_end%, my_hp_now) = my_champ.map(|c| (if c.max==0 {0} else {c.hp*100/c.max}, c.hp)).unwrap_or((0,0))   // max 0 → (0, c.hp), panic 없음. 1339/1420 의 target 쪽도 같은 checked 형",
      evidence=u"m13.ll 블록 %4467 `%4473 = icmp eq i64 %4470, 0; br i1 %4473, label %4477, label %4474` → %4477 phi %4478=[0,%4467] %4479=[%4472,%4467] · 1420: %4487 `%4492 = icmp eq i64 %4489, 0` → %4496 phi %4497=[0,%4487] %4498=[%4491,%4487] · 1339: %4269 `%4272 = icmp eq i64 %4271, 0`",
      behavior_change=False, found_by="new", kind=u"보강")

# ── logic: 1298 determine_transition_reason 는 &self 메서드 ────────
p.fix(S + "/logic",
      old=u"1298         let reason = determine_transition_reason(&prev, &name)   // 인라인 handler.rs:1675~1707, 순서대로:",
      new=u"1298         let reason = self.determine_transition_reason(&name)   // &self 메서드(prev = self.prev_plan_name 0x840 을 내부에서 읽음) · 인라인 handler.rs:1675~1707, 순서대로:",
      evidence=u"callees[13] tcx sig fn(&LegacyPlanHandler, &str) -> PlanTransitionReason · IR 인라인 본체가 prev 를 %4064/%4065(self+2120/2128 = prev_plan_name ptr/len)로 직접 로드(m13.ll 블록 %4077·%4094)",
      behavior_change=False, found_by="new", kind=u"보강")

# ── one_line 보강 ────────────────────────────────────────────────
p.fix(S + "/one_line",
      old=u"·서브플랜 병합·계측 기록",
      new=u"·v3 귀환→패시브 사다리(1263~1289: Recall+만피면 passive_plan/PassiveLine 으로 플랜 교체)·서브플랜 병합·계측 기록(트레이스/SIM_STATS/DIEWIN)",
      evidence=u"m13.ll 24383~24700(handler.rs:1263~1289) — 본 범위의 유일한 판정성 플랜 교체(self.plan@0x5e8 교체 #1·#2)가 one_line 에 없었다",
      behavior_change=False, found_by="new", kind=u"보강")

# ── consts.kind 어휘(meaning) ──────────────────────────────────
p.fix(S + "/consts[98]/meaning",
      old=u"game.tick() % 6 == 0 — SIM_STATS ring 기록 주기 / 1630 DIEWIN 로그 주기",
      new=u"game.tick() % 6 == 0 — 계수(urem 나눗수, 6틱 주기): SIM_STATS ring 기록 / 1630 DIEWIN 로그. 태그 아님(1325 의 assume ne 6 은 consts[90])",
      evidence=u"m13.ll 블록 %4931 `%4932 = urem i64 %4924, 6; %4933 = icmp ne i64 %4932, 0`(;L1575) · 1630 도 동일 urem — kind 가 `태그` 로 파생돼 있었다(같은 값 6 의 assume ne 관측이 CMP_EQ 로 잡힌 것)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/consts[103]/meaning",
      old=u"150000^2+1 — aux 클로저(1366/1371/1580/1652)의 distance_sq < 150000² (150k 반경). 본문엔 없고 aux 에 있음",
      new=u"150000^2+1 — aux 클로저(1366/1371/1580/1652)의 거리 임계: distance_sq < 150000² (150k 반경). 본문(m13.ll 14467~28943)엔 리터럴 없음·aux define 에만 있음",
      evidence=u"본문 grep 22500000001 = 0건(콜리 안) → kind `미상` 잔여 버킷. 어휘 `임계` 를 넣으면 mkspec3 ①(관측 없음→낱말 존중) 경로로 `임계`",
      behavior_change=False, found_by="new", kind=u"보강")

# ── mem: G20 R2 — 0x5f0 행 이름 가르기 ───────────────────────────
p.fix(S + "/mem[11]/name",
      old=u"plan 페이로드(+8) — chats() 오프셋 1520/1544/1624/1760",
      new=u"plan+8(페이로드 — chats() 오프셋 1520/1544/1624/1760)",
      evidence=u"sharedchk.norm_name 이 이 행을 `plan` 으로 정규화해 0x5e8 과 충돌(G20 R2). 새 이름은 `plan+8` 로 정규화됨(sharedchk.names_of 로 확인). tcxdict BigPlan 페이로드 = enum+0x8",
      behavior_change=False, found_by="new", kind=u"보강")

# ── mem: 229 plan w 노트 보강(cleanuppad 사본) ─────────────────────
p.fix(S + "/mem[229]/note",
      old=u"(배치 D) L1281 (24658 drop_glue<BigPlan>(&self.plan) → 24684 memcpy 384). plan_allowed 통과 시. ★HEAP: 옛 plan drop",
      new=u"(배치 D) L1281 (24658 drop_glue<BigPlan>(&self.plan) → 24684 memcpy 384). plan_allowed 통과 시. ★HEAP: 옛 plan drop · 두 교체 모두 drop_glue 의 unwind cleanuppad(%3977: 24576 / %4022: 24680)에서도 같은 memcpy 를 해 언와인딩 시에도 새 플랜이 self.plan 에 들어간다 · 1254/1270/1282 sub_plan 은 &mut self.plan 전달(별도 행)",
      evidence=u"m13.ll:24576 `call @llvm.memcpy(ptr %1930, ptr %148, i64 384)` in %3977 cleanuppad(24571 invoke 의 unwind) · 24680 in %4022 cleanuppad · 24580/24684 정상 경로",
      behavior_change=False, found_by="new", kind=u"보강")

# ── mem: 235 battle_start_state 노트 보강 ────────────────────────
p.fix(S + "/mem[235]/note",
      old=u"L1375 (25991 drop_glue<Option<BattleStartState>> 옛값 → 26016~26032 write).",
      new=u"L1375 (25991 drop_glue<Option<BattleStartState>> 옛값 → 26016~26032 write; 25996~26012 은 그 drop_glue 의 unwind cleanuppad %4403 에서 같은 write).",
      evidence=u"m13.ll 블록 %4400 invoke drop_glue … to %4413 unwind %4403 — %4403(25996~26012)·%4413(26016~26032) 둘 다 memcpy 48 + store ×8 @self+2376~2432",
      behavior_change=False, found_by="new", kind=u"보강")

# ── mem: infos 는 읽기가 아니라 쓰기 — reads[142] 삭제 후 writes 로 ──
p.errors.append({"op": "delete", "path": S + "/mem[142]",
                 "guard": u"1509~1512 HashMap<usize,Vec<String>>.entry(champ.id).or_insert(vec![]).push(..)",
                 "kind": u"실오류", "old": u"dir=r", "new": u"(writes 로 이동)",
                 "evidence": u"m13.ll 27266~27600(handler.rs:1509~1512): hashbrown rustc_entry → or_insert → Vec<String>::push_mut(grow_one) — debug.infos(%5+0xa0) 에 삽입(쓰기). 배치 C 의 같은 필드 행(mem[227] L1125)은 w. 읽기 행으로 앉아 있어 G14 사각(콜리 안 store)",
                 "behavior_change": False, "found_by": "new"})
# 삭제 뒤 인덱스: reads 142 · writes 98 → mem 240. at=229 = writes[87] = 옛 229(plan) 바로 뒤
p.errors.append({"op": "insert", "path": S + "/mem", "at": 229,
                 "guard": u"plan (&mut, BigPlan::sub_plan)", "guard_key": "name",
                 "new": {"base": "LegacyPlanHandler", "offset": "0x5e8", "name": u"plan (&mut, BigPlan::sub_plan)",
                         "value": u"콜리 내부 쓰기(계약만)",
                         "note": u"(배치 D) L1254/1270/1282 (24363·24583·24687) BigPlan::sub_plan(&mut self.plan, …) — tcx sig 첫 인자 &mut BigPlan · IR %1930(self+1512) 에 readonly 없음. 내부 쓰기는 자식 명세 소관(1010 의 BigPlan::update 행과 같은 형)"},
                 "kind": u"보강", "old": None,
                 "evidence": u"m13.ll:24363 `invoke …BigPlan8sub_plan(sret %145, ptr %1930, …)` (readonly 속성 없음) · callees[98] sig fn(&mut BigPlan, …)",
                 "behavior_change": False, "found_by": "new"})
# 삽입 뒤 mem 241 → at=241 = writes 끝
p.errors.append({"op": "insert", "path": S + "/mem", "at": 241,
                 "guard": u"infos (HashMap<usize,Vec<String>>) ★HEAP(외부)", "guard_key": "name",
                 "new": {"base": "DebugFrameData", "offset": "0xa0", "name": u"infos (HashMap<usize,Vec<String>>) ★HEAP(외부)",
                         "value": u"entry(champ.id 0x5c0).or_insert(vec![]).push(format!(…)) ×4",
                         "note": u"(배치 D) L1509~1512 (27266~27600) context.debug 일 때만 · self 아님. 1509 `speed : {move_speed 0x640}` · 1510 `Main Objective: {:?}`(team_plan.objective 0x517) · 1511 `Plan: {:?}`(get_name) · 1512 `SubPlan: {:?}`(sub_plan 0x768). rustc_entry(hashbrown)→or_insert→push_mut/grow_one"},
                 "kind": u"실오류", "old": None,
                 "evidence": u"m13.ll 27266~27600: `invoke …hashbrown…rustc_entry`·`…or_insert`·`…push_mut`/`grow_one` (주석본 ;L825<2521<1509 등) — 읽기 행 mem[142] 를 쓰기 행으로 옮김",
                 "behavior_change": False, "found_by": "new"})

# ── 지시 오류 ─────────────────────────────────────────────────────
p.brief_error(u"프롬프트의 사장 판정 파일 경로 `_nexteach\\e4c5c0.reach.txt` 는 없다 — 실제 = `MIG\\_next\\reach\\e4c5c0.reach.txt`(version=2·Moba 접기, 사장 254) / `update_e4c5c0.reach.txt`(version 접기만, 사장 30)")
p.brief_error(u"「G16/G19/G7/G20 R2 초점」 — specgate --gate G7/G16/G19 는 [111] 에 미해소 0건(실행 확인). 111 에 남은 건 G12 5건과 G20 R1/R2 뿐")
p.brief_error(u"§4 「이 배치 몫 = 5건」은 함수 단위 합계다 — 줄 범위로 가르면 D 몫은 consts[93](1349)·[96](1453) 2건뿐이고 [30](698)=B·[46](1013)/[64](1133)=C 몫")
p.brief_error(u"★도구 결함: reach.py 의 INVOKE 정규식 `unwind label %(\\S+)` 가 주석본의 `unwind label %4214, !!33386` 꼴에서 `%4214,` 를 라벨로 잡아 정리패드 10쌍(4214/4216·4708/4710·4748/4750·4763/4765·4788/4790·4803/4805·4828/4830·4850/4852·4875/4877·4889/4891)을 「사장」으로 오판 — 본 범위(1252~1672)의 dead_blocks 30개 중 20개가 이것이고 실제 사장 논리 블록은 0")
p.brief_error(u"오라클 지시(6168B 스냅샷 diff)는 본 배치에선 불요했다 — 갈린 항목(1450 사다리 극성)이 문자열 리터럴 신원(@anon.65 \"target_died\"/@anon.66 \"target_escaped\")과 CFG 로 확정돼 실행 없이 닫혔다")
p.save()
