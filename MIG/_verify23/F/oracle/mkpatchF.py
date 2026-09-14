# -*- coding: utf-8 -*-
"""23차 배치F patch.json 생성 (mkpatch 참조구현 사용)."""
import sys, io
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch

p = mkpatch.Patch(round=23, batch="F")

# ───────── §4 G12 4건 = 전부 오탐 (case 라벨 / 호이스트) ─────────
p.fix("/specs[173]/consts[7]/meaning",
      old="적 팀 = 1 - my_team (110552 `sub nuw nsw i64 1, %34`)",
      new="적 팀 = 1 - my_team (110552 `sub nuw nsw i64 1, %34` — ★이 명령은 `!dbg` 없는 호이스트라 G12 후보에 안 잡힌다; 유일 소비자 111118 `invoke … %306(ptr %23, i64 %66, …)` 의 `!dbg !71173` 루트 = around.rs:347 이므로 명세가 옳다)",
      evidence="m08.ll:110552 `%66 = sub nuw nsw i64 1, %34`(줄 끝 !dbg 없음) · m08.ll:111118~111119 `%307 = invoke … i64 noundef %66 … to label %308 unwind label %103, !dbg !71173` · !71173 사슬 루트 around.rs:347. srclinecheck 의 소비자 구제(_users)가 invoke 2줄 형식(!dbg 가 다음 줄)을 못 봐서 오탐",
      kind="오탐", behavior_change=False, found_by="reused")
p.fix("/specs[174]/consts[3]/meaning",
      old="①ChampionActionState::Skill2 태그 ②EntityType::Epic 태그(L460 `epic.ty == Epic`)",
      new="①ChampionActionState::Skill2 태그(m02.ll:45266 `i64 5, label %84` — switch case 라벨이라 `!dbg` 없음; arm %84 의 `!51344` 사슬에 epic_poke.rs:437(closure$0) 이 있어 명세가 옳다 · G12 오탐) ②EntityType::Epic 태그(L460 `epic.ty == Epic`)",
      evidence="m02.ll:45264~45268 `switch i64 %76, label %72 [ i64 4, label %77 / i64 5, label %84 / i64 6, label %94 ], !dbg !51306` · arm %84 첫 명령 m02.ll:45295 `!dbg !51344` 사슬 = entity.rs:1693 → epic_poke.rs:437(closure$0) → … → epic_poke.rs:433. arm 종결자가 `switch i32 %91 … ]`(45303~45307) 이고 그 `!dbg` 가 `]` 줄에 있어 srclinebase._term_dbg(첫 `switch ` 줄만 봄)가 None 을 돌려줌 → 약한 후보 미산출",
      kind="오탐", behavior_change=False, found_by="reused")
p.fix("/specs[175]/consts[3]/meaning",
      old="①ChampionActionState::Skill2 태그 ②모든 후보 생성자 end_delay",
      new="①ChampionActionState::Skill2 태그(m14.ll:17092 `i64 5, label %82` case 라벨 — `!dbg` 없음; arm %82 의 `!31989` 사슬에 serpen_poke.rs:429(closure$0) 있음 · G12 오탐) ②모든 후보 생성자 end_delay",
      evidence="m14.ll:17090~17094 `switch i64 %74, label %70 [ i64 4, label %75 / i64 5, label %82 / i64 6, label %92 ], !dbg !31951` · m14.ll:17121 `!dbg !31989` 사슬 = entity.rs:1693 → serpen_poke.rs:429 → … → serpen_poke.rs:425",
      kind="오탐", behavior_change=False, found_by="reused")
p.fix("/specs[175]/consts[4]/meaning",
      old="①ChampionActionState::Ult 태그 ②EntityType::Serpen 태그(L451 `serpen.ty == Serpen`)",
      new="①ChampionActionState::Ult 태그(m14.ll:17093 `i64 6, label %92` case 라벨 — `!dbg` 없음; arm %92 의 `!31994` 사슬에 serpen_poke.rs:431(closure$0) 있음 · G12 오탐) ②EntityType::Serpen 태그(L451 `serpen.ty == Serpen`)",
      evidence="m14.ll:17093 case 라벨 · m14.ll:17144 `%93 = getelementptr … !dbg !31994` 사슬 = entity.rs:1701(ult_effect) → serpen_poke.rs:431(closure$0) → … → serpen_poke.rs:425",
      kind="오탐", behavior_change=False, found_by="reused")

# ───────── §4 G19 [172] knobs[0].value ─────────
p.fix("/specs[172]/knobs[0]/value", old="5000 / 96000", new="-9190999999 (5000²=25000000 이하 / 96000²=9216000000 이상)",
      evidence="m08.ll:102621 `%54 = add i64 %52, -9216000000` · 102623 `%56 = icmp ult i64 %55, -9190999999`(접힌 범위검사: d² < 25000001 ∨ d² ≥ 9216000000). 소스 수준 5000²/96000² 은 리터럴로 안 남는다 — 178 knobs[0](-921574999999) 과 같은 표기로 통일",
      kind="보강", behavior_change=False, found_by="reused")
p.fix("/specs[172]/knobs[0]/what", old="목표 재선정 트리거(도착 반경 / 이탈 반경)",
      new="목표 재선정 트리거(도착 반경 5000 / 이탈 반경 96000 — IR 은 `d²-96000² ult 25000001-96000²` 로 접힘, 값 칸 = 그 비교 상수)",
      evidence="m08.ll:102621~102624", kind="보강", behavior_change=False, found_by="reused")

# ───────── §4 G20 R1 ─────────
p.fix("/specs[172]/mem[24]/name", old="candidates ptr/+0x8 bump/+0x10 cap/+0x18 len",
      new="candidates(ptr/+0x8 bump/+0x10 cap/+0x18 len)",
      evidence="173 mem[30] 과 같은 원점·같은 레이아웃(bumpalo Vec 32B: +0 ptr +8 bump +0x10 cap +0x18 len · m08.ll:103077~103088 push) — 문자열만 통일",
      kind="보강", behavior_change=False, found_by="reused")
for i in (27, 28, 29):
    p.fix("/specs[177]/mem[%d]/base" % i,
          old="후보 튜플 (i32 dx,i32 dy,i32 x,i32 y,i64 score) 24B" if i == 27 else "후보 튜플",
          new="후보 튜플(AroundRegion (i32 dx,i32 dy,i32 x,i32 y,i64 score) 24B)",
          evidence="m08.ll:99462 `getelementptr inbounds nuw { i32, i32, i32, i32, i64 }` push 사이트 · 179 의 「후보 튜플」은 다른 함수의 다른 튜플(.4 = score*10000+d/1000)이라 같은 (base,offset) 으로 묶이면 안 된다 — 함수 한정어로 분리(D9-OFF `<타입>(<한정어>)`)",
          kind="보강", behavior_change=False, found_by="new")
for i in (34, 35, 36):
    p.fix("/specs[179]/mem[%d]/base" % i,
          old="후보 튜플 (i32 dx,i32 dy,i32 x,i32 y,i64 key) 24B" if i == 34 else "후보 튜플",
          new="후보 튜플(Around (i32 dx,i32 dy,i32 x,i32 y,i64 key=score*10000+d/1000) 24B)",
          evidence="m08.ll:93595~93600 push 사이트(key = value*10000 + d/1000) · 177 과 튜플 의미가 다르다(.4 = score) — 함수 한정어로 분리",
          kind="보강", behavior_change=False, found_by="new")
for i in (29, 30, 31):
    p.fix("/specs[178]/mem[%d]/base" % i,
          old="후보 튜플 (i32 x,i32 y,i64 score) 16B" if i == 29 else "후보 튜플",
          new="후보 튜플(AroundPosition (i32 x,i32 y,i64 score) 16B)",
          evidence="m08.ll:104460~ push 사이트 `{ i32, i32, i64 }` 16B — 177/179 의 24B 튜플과 다른 타입이라 (base,offset 0x8) 이 .2=score ↔ .2=x 로 충돌한다(G20 R1 잠재 오탐) — 함수 한정어로 분리",
          kind="보강", behavior_change=False, found_by="new")

# ───────── ⑥ exe↔IR 인자 대응 (174/175 parameter 가 exe 에서 제거) ─────────
p.fix("/specs[174]/sig/abi",
      old="define void, 외부 심볼(비-fastcc). 인자 승격 없음.",
      new="define void, 외부 심볼(비-fastcc). ★exe 0xcc9740 은 **인자 6개**(IR 7): rcx=sret(%0) · rdx=self(%1, 호출자 0xcc6170 이 IR 대로 poison — 슬롯만 있고 미설정) · r8=version(%2) · r9=rnd(%3) · [entry+0x20]=player(%4) · [entry+0x28]=data(%5) · **parameter(%6) 는 exe 에 없음(LTO DeadArgElim — position_score_at_position 의 ps 인자가 exe 인라인 체인에서 죽어 이 함수도 안 읽는다; [entry+0x30] 접근 0 · 호출자도 미설정)**. 그 외 승격 없음.",
      evidence="argscan 0xcc9740: 스택 슬롯 +0x20(x4)·+0x28(x6) 만 접근, +0x30 접근 0 → 「스택 2 + 레지스터 4 = 6」 · argscan --caller 0xcc6170: 호출부 cc61fb~cc6212 `mov [rsp+0x28],rbx / mov [rsp+0x20],r14 / lea rcx,[rbp+0x1c0] / mov r8,rdi / mov r9,r15 / call 0x140cc9740`(rdx·[rsp+0x30] 미설정) · 직전 call 0xdd5270(v27_objective_discipline_action(sret,team_plan,version,rnd,player,data,i8 4)) 이 rdx=[rbp+0x458]=team_plan · r8=version · r9=rnd · +0x20=r14=player · +0x28=rbx=data 로 같은 레지스터를 씀 → rdi=version·r15=rnd·r14=player·rbx=data 확정 · callee 내부 cc98be~cc98d6 nontarget_windup_perceived(rcx=[rbp+0xd8]=r8orig=version, rdx=[rbp+0x1a0]=player, r8=[rbp+0x1a8]=data) · cc9b4d~cc9b66 position_score 계열 0xd84db0 호출이 (sret, version, player, data, cx*32000+16000, cy*32000+16000, i8 0xb) 로 ps 인자 없음",
      kind="실오류", behavior_change=False, found_by="new", force=True)
p.fix("/specs[175]/sig/abi",
      old="define void, 외부 심볼(비-fastcc). 인자 승격 없음.",
      new="define void, 외부 심볼(비-fastcc). ★exe 0xe80070 은 **인자 6개**(IR 7): rcx=sret · rdx=self(poison/미설정) · r8=version · r9=rnd · [entry+0x20]=player · [entry+0x28]=data · **parameter(%6) 는 exe 에 없음(DeadArgElim, 174 와 동일)**. 그 외 승격 없음.",
      evidence="argscan 0xe80070: +0x20(x4)·+0x28(x6) 만 접근 → 6 · argscan --caller 0xe7caf0: e7cb7b~e7cb92 `mov [rsp+0x28],rbx / mov [rsp+0x20],r14 / lea rcx,[rbp+0x1b8] / mov r8,rdi / mov r9,r15 / call 0x140e80070`(rdx·+0x30 미설정) · 직전 call 0xdd5270 이 (rdx=team_plan, r8, r9, +0x20=r14, +0x28=rbx, +0x30=i8 5) 로 같은 레지스터 배치",
      kind="실오류", behavior_change=False, found_by="new", force=True)
p.fix("/specs[174]/sig/params[6]/role",
      old="readonly. +0x9f0 positioning_score 의 주소만 position_score_at_position 에 넘김(본문 직접 load 없음).",
      new="readonly. +0x9f0 positioning_score 의 주소만 position_score_at_position 에 넘김(본문 직접 load 없음). ★exe 에는 이 인자가 없다(DeadArgElim — sig.abi 참조): 런타임 훅이 7번째 인자를 읽거나 재호출에 넘기면 안 된다.",
      evidence="argscan 0xcc9740 +0x30 접근 0 · 호출자 0xcc6170 [rsp+0x30] 미설정", kind="보강", behavior_change=False, found_by="new")
p.fix("/specs[175]/sig/params[6]/role",
      old="readonly. +0x9f0 positioning_score 주소만 전달.",
      new="readonly. +0x9f0 positioning_score 주소만 전달. ★exe 에는 이 인자가 없다(DeadArgElim — sig.abi 참조).",
      evidence="argscan 0xe80070 +0x30 접근 0 · 호출자 0xe7caf0 [rsp+0x30] 미설정", kind="보강", behavior_change=False, found_by="new")

# ───────── ③ &mut rnd 부작용 — 오라클로 확정 ─────────
WP = ("positioning_window_pick(m11.ll:11351~11439)은 version>1 이면 rnd 를 안 쓴다: idx = mix((game.tick()/max(tps*6,1))<<32 ^ player.info.id) % len "
      "(mix: h^=h>>33; h^=20942; h*=0x9E3779B97F4A7C15; h^=h>>29) — 오라클 o_wpick 9/9 MATCH·rnd 바이트 불변; version≤1 만 rand::seq::SliceRandom::choose(rnd) 로 소비(2/2 rnd 변함). "
      "PathFinder new_target/update_path 인스턴스·is_safe_recall 은 rnd `readnone`(m03.ll:10867/29230 등 · m07.ll:58406) ⟹ ★version≥2 경로의 rnd 소비 = 0회")
p.fix("/specs[172]/sig/params[3]/role",
      old="★&mut — 본문 직접 store 0. positioning_window_pick · new_target · update_path · is_safe_recall 에 전달",
      new="★&mut — 본문 직접 store 0. positioning_window_pick · new_target · update_path · is_safe_recall 에 전달. " + WP,
      evidence="오라클 실행 확인: _verify23/F/oracle/o_wpick.rs 11케이스(케이스당 프로세스 1개) — version 2/3 9/9 MATCH(rnd_changed=false) · version 0/1 2/2 rnd_changed=true · m11.ll:11359 `icmp ugt i64 %0, 1` 분기 · 11363 SliceRandom::choose · 11383 vtable+40 tick · 11401 setting+0x12f8 tps · 11410 player+0x928 info.id · 11437 `urem`",
      kind="보강", behavior_change=False, found_by="new")
p.fix("/specs[173]/sig/params[3]/role",
      old="★&mut — 본문 직접 store 0. positioning_window_pick · new_target · update_path · is_safe_recall 전달",
      new="★&mut — 본문 직접 store 0. positioning_window_pick · new_target · update_path · is_safe_recall 전달. " + WP,
      evidence="오라클 실행 확인: o_wpick 9/9 MATCH (172 와 동일 인스턴스 `Tllllx`) · m03.ll:21357/36805 rnd readnone", kind="보강", behavior_change=False, found_by="new")
p.fix("/specs[177]/sig/params[3]/role",
      old="readonly 없음(&mut). 본문 직접 쓰기 0 — positioning_window_pick·PathFinder::new_target·update_path·is_safe_recall 에 그대로 전달(콜리가 소비)",
      new="readonly 없음(&mut). 본문 직접 쓰기 0 — positioning_window_pick·PathFinder::new_target·update_path·is_safe_recall 에 그대로 전달. " + WP,
      evidence="오라클 실행 확인: o_wpick 9/9 MATCH · m03.ll:3634/7405/27054 rnd readnone", kind="보강", behavior_change=False, found_by="new")
p.fix("/specs[178]/sig/params[3]/role",
      old="직접 쓰기 0 — positioning_window_pick·new_target_with_policy·update_path·is_safe_recall 에 전달",
      new="직접 쓰기 0 — positioning_window_pick·new_target_with_policy·update_path·is_safe_recall 에 전달. " + WP.replace("(m11.ll:11351~11439)", "(m11.ll:11443 `Tllx` 인스턴스 — 본체 동일)"),
      evidence="오라클 실행 확인: o_wpick 9/9 MATCH(Tllllx 인스턴스 · Tllx 는 같은 제네릭 본체) · m03.ll:88853/92336/30288 rnd readnone", kind="보강", behavior_change=False, found_by="new")
p.fix("/specs[179]/sig/params[3]/role",
      old="직접 쓰기 0 — positioning_window_pick·new_target_with_policy·update_path·is_safe_recall·SmallActionRunAway::get_input 에 전달",
      new="직접 쓰기 0 — positioning_window_pick·new_target_with_policy·update_path·is_safe_recall·SmallActionRunAway::get_input 에 전달. " + WP + " · SmallActionRunAway::get_input(m08.ll:106076) 도 `version>1`(106773) 이면 window_pick(108847), 아니면 SliceRandom::choose(108842) — 같은 규칙",
      evidence="오라클 실행 확인: o_wpick 9/9 MATCH · m03.ll:78404/81887/25963 rnd readnone · m08.ll:106773 `%204 = icmp ugt i64 %2, 1` → 108838 분기", kind="보강", behavior_change=False, found_by="new")
p.fix("/specs[176]/sig/params[3]/role",
      old="본문 직접 write 없음 — new_target_with_policy/update_path/RunAway::get_input 에 &mut 전달",
      new="본문 직접 write 없음 — new_target_with_policy/update_path/RunAway::get_input 에 &mut 전달. new_target_with_policy/update_path 인스턴스는 rnd `readnone`(m03.ll:103055/106538/34623/35714); RunAway::get_input 은 version>1 이면 positioning_window_pick(rnd 미소비 · 오라클 9/9), version≤1 만 SliceRandom::choose ⟹ version≥2 경로 rnd 소비 0회",
      evidence="m03.ll:103055 define … `ptr noalias noundef readnone align 16 … dereferenceable(320) %1` · m08.ll:106773/108838~108847 · 오라클 o_wpick", kind="보강", behavior_change=False, found_by="new")
p.fix("/specs[176]/sig/params[7]/role",
      old="본문 직접 write 없음 — RunAway::get_input 에만 전달(64364)",
      new="본문 직접 write 없음 — RunAway::get_input 에만 전달(64364). 그 콜리의 %7 도 `readnone captures(none)`(m08.ll:106076) ⟹ 이 경로에서 debug 는 어디서도 안 읽힌다(179 와 동일)",
      evidence="m08.ll:106076 define …SmallActionRunAway9get_input(… ptr noalias readnone align 8 captures(none) %7)", kind="보강", behavior_change=False, found_by="reused")

# ───────── ev 상향 ─────────
for i in (172, 173, 177, 178, 179, 176):
    p.ev("/specs[%d]/sig/params[3]" % i, to=2, frm=4,
         evidence="오라클 실행 확인: positioning_window_pick 진리표 9/9 MATCH(version>1 rnd 불변) + 2/2 rnd 변함(version≤1) — _verify23/F/oracle/o_wpick.log",
         found_by="new")
for i in (174, 175):
    p.ev("/specs[%d]/sig/params[6]" % i, to=3, frm=4,
         evidence="exe 실측(argscan · 호출자 디스어셈): 7번째 인자는 exe 에 없다(+0x30 접근 0 · 호출자 미설정)", found_by="new")

# ───────── 174/175 오라클(pub 직접 호출) — 에픽/세르펜 미스폰 경로 ─────────
OR = "오라클 실행 확인(_verify23/F/oracle/o_poke.rs · start_game 직후 tick=100 · version=2 · ScoreParameter 제로버퍼): "
p.fix("/specs[174]/sig/returns",
      old="sret Vec<SmallActionPlay>(32B 전부 live).",
      new="sret Vec<SmallActionPlay>(32B 전부 live — " + OR + "vec32 ptr/bump/cap/len = pool 주소·cap 1·len 1 실측).",
      evidence=OR + "len=1 · [0] variant=AroundRegion tag@0xb1=7 · +0x8 target_region=7 · +0x20 goal_risk=9223372036854775807 · +0x28 end_delay=5 · +0x75 path_finder@tag=2(None) · rnd_changed=false · bump==pool — o_poke.log case 0",
      kind="보강", behavior_change=False, found_by="new", force=True)
p.fix("/specs[175]/sig/returns",
      old="sret Vec<SmallActionPlay>(32B 전부 live).",
      new="sret Vec<SmallActionPlay>(32B 전부 live — " + OR + "vec32 ptr/bump/cap/len 실측·cap 1·len 1).",
      evidence=OR + "len=1 · [0] variant=AroundRegion tag@0xb1=7 · +0x8 target_region=2 · +0x20 goal_risk=i64::MAX · +0x28 end_delay=5 · +0x75=2 · rnd_changed=false — o_poke.log case 1",
      kind="보강", behavior_change=False, found_by="new", force=True)
for i, reg in ((174, 7), (175, 2)):
    p.ev("/specs[%d]/knobs[0]" % i, to=2, frm=4, evidence="오라클 실행 확인: 에픽/세르펜 부재 경로에서 AroundRegion.target_region=%d 실측(o_poke.log)" % reg, found_by="new")
    p.ev("/specs[%d]/sig/params[0]" % i, to=2, frm=(3 if i == 174 else 4), evidence="오라클 실행 확인: sret Vec 32B ptr/bump/cap/len 실측(bump==ctx.pool · cap=len=1) — o_poke.log", found_by="new")
    p.ev("/specs[%d]/mem[%d]" % (i, 55 if i == 174 else 53), to=3, frm=4, evidence="오라클 실행 확인: 원소 +0xb1 태그 = 7(AroundRegion) 실측 · tcxdict --enum SmallActionPlay 판별자 enum+0xb1 니치", found_by="new")

# ───────── 지시 오류 ─────────
p.brief_error("§4 G12 4건(173 consts[7] · 174 consts[3] · 175 consts[3]·[4])은 전부 오탐이었다 — ①switch case 라벨 3건은 arm 종결자가 또 다른 `switch` 라 `!dbg` 가 `]` 줄에 붙는데 srclinebase._term_dbg 가 `switch ` 헤더 줄만 보고 None 을 돌려줘 7차 귀속 구제가 발화하지 않았다 ②호이스트 1건은 소비자가 `invoke`(2줄 형식) 라 srclinecheck._users 구제가 다음 줄의 `!dbg` 를 못 봤다. 게이트 수정 제안: _term_dbg 에서 종결자가 switch 면 이어지는 `]` 줄까지 DBG 를 찾고, _users 에서 invoke 다음 줄(`to label`)의 !dbg 를 채택")
p.brief_error("§4 G20 R1 [177]↔[179] 「후보 튜플 0x10」은 구조적 오탐 — 두 함수의 지역 튜플이 서로 다른 타입/의미(.4=score vs .4=score*10000+d/1000)인데 base 문자열이 우연히 같았다. 이번에 함수 한정어로 분리해 닫았지만, sharedchk 는 지역(스택/bumpalo) 임시 객체 base 를 cross-spec 비교에서 제외하거나 함수별로 키를 나눠야 한다(178 의 16B 튜플 0x8=.2=score 도 같은 충돌 잠재)")
p.brief_error("지시 ⑥「internal fastcc 함수(137·144·152·155·167·169·170·171)의 exe↔IR 대응표」는 배치F 담당(172~179) 밖 항목이다 — F 의 8함수는 전부 `define hidden`/외부 심볼이고 fastcc 가 아니다. 대신 8함수 전부 argscan 을 돌려 IR 인자 수와 대조했고 174/175 에서 IR 7 vs exe 6(parameter DeadArgElim) 을 찾았다")
p.brief_error("도시에 §1 「vis 가 pub 이 아니면 오라클 직접 진입이 막힌다」는 22차 C·D 이후 stale — `define hidden` 은 link_name 으로 직접 링크된다(이번 o_wpick 도 in:game_ai::small_action 의 hidden 제네릭 인스턴스를 직접 호출). §1 문구를 METHOD_MAP ⑥ 정정판으로 맞춰야 배치가 pub 래퍼를 찾느라 헤매지 않는다")

p.save()
