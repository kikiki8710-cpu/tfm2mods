# -*- coding: utf-8 -*-
"""mkpatch27E.py — 27차 배치E patch.json 생성(mkpatch 참조구현 사용 + delete/dict 항목은 직접 append)."""
import sys, io, json
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import mkpatch

p = mkpatch.Patch(round=27, batch="E")
EXE058 = u"0.5.8 백업 exe C:\\Users\\jungs\\Desktop\\claude\\tfm2\\tfm2_0.5.8\\TeamfightManager2.exe(77,666,816B · 09-02) — 라이브 exe 는 2026-09-16 14:40 패치본(86,082,048B · mig_verify 전 항목 STALE)이라 로스터 RVA 가 .pdata 함수 시작이 아니다"

# ═══════════════ 258 choose_goal ═══════════════
# G16 P3 / G5: &self 1 소스 인자 = 1 행(ArgumentPromotion 2 슬롯) — 행 병합
p.fix("/specs[258]/sig/params[1]/name",
      old=u"self.goal_score (ArgumentPromotion 조각 1/2)",
      new=u"self (&SmallActionLaneMinionPosition → LTO ArgumentPromotion 2 스칼라: goal_score i64 %1 · position_eval_purpose i8 %2)",
      evidence=u"m11.ll:43376 define `(ptr … %0, i64 %1, i8 %2, i64 noundef %3, ptr … %4, …)` · 호출자 m11.ll:46257~46260 이 self+0x20·self+0x78 을 load 해 넘김. P3 규약 = 소스 인자 1개는 1행(129 should_add_self_etc_buff_action 선례)",
      behavior_change=False, found_by="reused")
p.fix("/specs[258]/sig/params[1]/type", old=u"i64 %1", new=u"(i64, i8)",
      evidence=u"m11.ll:43376 define 인자 2·3 = `i64 %1, i8 %2` (ScalarPair 표기 — paramrole.ir_slots 가 2슬롯으로 센다)",
      behavior_change=False, found_by="reused")
p.fix("/specs[258]/sig/params[1]/role",
      old=u"IR 에 `&self` 는 없다(DI self = ptr poison). 호출자 get_input(m11.ll:46257~46260) 이 `self+0x20`(=SmallActionLaneMinionPosition.goal_score i64, tcxdict) 을 load 해 넘긴다. 본문에서는 push_candidate 의 13번째 인자 `target_score` 로만 3회 전달(43538·43869·43878) — 분기 없음. IR 속성 없음",
      new=u"IR 에 `&self` 는 없다(DI self = ptr poison) — 소스 인자 1개가 IR 2 슬롯 `i64 %1`(self.goal_score)·`i8 %2`(self.position_eval_purpose)로 쪼개졌다. 호출자 get_input(m11.ll:46257~46260) 이 `self+0x20`(=SmallActionLaneMinionPosition.goal_score i64, tcxdict) 과 `self+0x78`(position_eval_purpose: PositionEvalPurpose 1B) 을 load 해 넘긴다. 본문에서 %1 은 push_candidate 의 13번째 인자 `target_score` 로만 3회 전달(43538·43869·43878) · %2 는 push_candidate 10번째 인자(range(i8 0,13)) 로만 3회 전달 — 둘 다 분기 없음. IR 속성 없음. exe 0xe248f0 도 같은 분해(rdx=goal_score · r8b=purpose · argscan 0.5.8 백업 exe · sig.exe_abi)",
      evidence=u"m11.ll:43376 define · 43538/43869/43878 invoke push_candidate 의 13·10번째 인자 %1·%2 · exe e24924 `mov [rbp+0x178], rdx`→e24a9e→[rsp+0x70](push_candidate 15번째) · e2491d `mov byte [rbp+0x18f], r8b`→e24ac2→[rsp+0x58](12번째)",
      behavior_change=False, found_by="reused")
p.errors.append({"op": "delete", "path": "/specs[258]/sig/params[2]", "guard": u"self.position_eval_purpose (ArgumentPromotion 조각 2/2)",
                 "kind": u"실오류", "old": u"self.position_eval_purpose (ArgumentPromotion 조각 2/2)", "new": u"",
                 "evidence": u"params[1] 로 병합(P3 규약: 소스 인자 1개 = 1행 · i=1 중복 제거). 정보는 params[1] name/type/role 에 전부 보존",
                 "behavior_change": False, "found_by": "reused"})
# G19 knobs[1] — 오탐
p.fix("/specs[258]/knobs[1]/effect",
      old=u"올리면 장거리 챔프가 목표에 더 가까이 서는 링 후보를 만든다(≈0.44셀 당 14000)",
      new=u"올리면 장거리 챔프가 목표에 더 가까이 서는 링 후보를 만든다(≈0.44셀 당 14000) (★G19 오탐: 소스 `attack_range.saturating_sub(14000)`(uint_macros 2472 인라인 · DI rhs=14000)이 m11.ll:43512 `%77 = add i64 %67, -14000` 로 접혀 IR 리터럴은 -14000 이다 — %67>69999 가 보장돼 포화 분기가 제거됐다. 노브 값 14000 이 맞고, knobval 에 「`add x, -N` 관측 = N 인정」 부호접힘 구제가 없어 생긴 오탐)",
      evidence=u"m11.ll:43512 `%77 = add i64 %67, -14000, !dbg` (사슬 L2472<496) · 43493 `%68 = icmp ugt i64 %67, 69999` 참 경로에서만 실행 · consts[3] 은 이미 -14000 으로 등재",
      behavior_change=False, found_by="new", kind=u"오탐")
# open[2] — exe promotion 판정 반전(스택 인자 0·xmm0 → 11 인자)
p.fix("/specs[258]/open[2]",
      old=u"exe 에서는 다르게 promotion 될 수 있어(0xe248f0 argscan 은 스택 인자 0·xmm0 읽음) sweep 편입 전 argscan 대조 필요",
      new=u"exe 0xe248f0(0.5.8 백업 exe · argscan) 은 **11 인자** = rcx sret(%0) · rdx goal_score(%1) · r8b purpose(%2) · r9 version(%3) · [rsp+0x20] player(%4) · +0x28 data(%5) · +0x30 champ(%6) · +0x38 target(%7) · +0x40 positioning_score.cx(i64 값) · +0x48 positioning_score.cy · +0x50 line(%9, byte) — &self 분해는 IR 과 같고 **추가로 &PositioningScoreData(%8) 가 (cx,cy) 두 값으로 승격**됐다(e24b54 `mov rax,[rbp+0x220]; add eax,-3` = cx 를 값으로 씀 · push_candidate 0xe25450 에도 [rsp+0x38]/[rsp+0x40] 두 값으로 전달 → exe push_candidate 17 인자의 +1 이 이것). ~~스택 인자 0·xmm0 읽음~~ 은 라이브 exe(14:40 패치본)를 스캔한 오독. 상세 = sig.exe_abi(27E 확정 · sweep 편입 시 EXE_ABI 재료)",
      evidence=u"argscan058 0xe248f0: 스택 7 + 레지스터 4 = 11 · e24916 `mov [rbp+0x170], r9`→push_candidate rdx(version) · e2492b `mov rbx,[rbp+0x210]`→`cmp dword [rbx+0x4c0],-1`(champ) · e24946 `mov r15,[rbp+0x218]`→`[r15+0x660]`(target) · e24b54/e24b65 `[rbp+0x220]`/`[rbp+0x228]` add -3 (cx,cy) · e24a93 `movzx eax, byte [rbp+0x230]`(line) · call 0xe25450 = e24b25·e24c81·e24f88 (3곳 = IR invoke 3회)",
      behavior_change=False, found_by="new", kind=u"실오류")
p.errors.append({"path": "/specs[258]/sig/exe_abi", "kind": u"보강", "old": None,
                 "new": {"exe": "0xe248f0", "exe_src": EXE058, "nargs_exe": 11, "nargs_ir": 10,
                         "map": ["rcx = sret Option<(u64,u64,i64)> (IR %0)", "rdx = self.goal_score i64 (IR %1)", "r8b = self.position_eval_purpose i8 (IR %2)",
                                 "r9 = version (IR %3)", "[rsp+0x20] = player (IR %4)", "[rsp+0x28] = data (IR %5)", "[rsp+0x30] = champ (IR %6)",
                                 "[rsp+0x38] = target &Entity (IR %7 · 본체가 +0x660/+0x668 을 load)", "[rsp+0x40] = positioning_score.cx i64 값 (IR %8 &PositioningScoreData 의 exe 추가 승격 1/2 · +0xab8)",
                                 "[rsp+0x48] = positioning_score.cy i64 값 (승격 2/2 · +0xac0)", "[rsp+0x50] = line i8 (IR %9 · movzx byte)"],
                         "promoted_beyond_ir": "&PositioningScoreData → (cx, cy) 두 i64 (exe 에서만) — push_candidate 0xe25450 도 pos_score 자리에 (cx,cy) 두 값을 받아 17 인자",
                         "evidence": "argscan058 0xe248f0(스택 7+레지스터 4) · e2491d/e24924/e24916 레지스터 spill · e2492b `cmp dword [rbx+0x4c0],-1` · e24946→`[r15+0x660]` · e24b54 `mov rax,[rbp+0x220]; add eax,-3` · e24a93 movzx byte [rbp+0x230] · call 0xe25450 @e24b25/e24c81/e24f88",
                         "sweep": "EXE_ABI: IR define 10 ≠ exe 11 — 재호출은 (cx,cy) 값을 직접 넘겨야 함(포인터 아님)"},
                 "evidence": u"위 map/evidence 필드 · 0.5.8 백업 exe 기준", "behavior_change": False, "found_by": "new"})

# ═══════════════ 259 v48_projectile_profile ═══════════════
for k, val in ((10, 9), (11, 7)):
    p.fix("/specs[259]/consts[%d]/src_line" % k, old=208, new=218,
          evidence=u"m15.ll:30862 `%%134 = icmp ne i64 %%133, 9` / 30866 `select … i64 7` 의 !dbg 사슬 = L208<218<79<2671<165<271<218<199 — 208 은 projectile.rs(dodgeable) 콜리 줄이고 이 함수 파일(fight_check.rs) 줄은 218(필터 `p.dodgeable()`)·루트 199(closure#1). 같은 값이 30934/30938 (사슬 L223<199) 에도 있음",
          behavior_change=False, found_by="reused")
p.fix("/specs[259]/consts[10]/meaning",
      old=u"판정 아님",
      new=u"판정 아님. src_line 218 = L218 필터 `p.dodgeable()` 인라인 자리(30862 사슬 L208<218<…<199) · L223 match 의 30934 에도 같은 assume — projectile.rs:208 은 콜리 줄이라 이 함수 소스 줄 규약(fight_check.rs) 위반이었다",
      evidence=u"m15.ll:30862/30934 !dbg 사슬", behavior_change=False, found_by="reused")
p.fix("/specs[259]/consts[11]/meaning",
      old=u"tcxdict 니치 환산과 일치",
      new=u"tcxdict 니치 환산과 일치. src_line 218 = L218 필터 인라인 자리(30866 · 사슬 L208<218<…<199) · L223 match 의 30938 에도 같은 select",
      evidence=u"m15.ll:30866/30938 !dbg 사슬", behavior_change=False, found_by="reused")
# G14 mem[38] — 오탐
p.fix("/specs[259]/mem[38]/note",
      old=u"aux m00.ll:74830 — seed 불일치 시(L189). 저장 클로저는 seed 를 안 바꿈",
      new=u"aux m00.ll:74830 — seed 불일치 시(L189). 저장 클로저는 seed 를 안 바꿈 (★G14 오탐: m00.ll:74830 `store i64 %18, ptr %15` · %15 = `getelementptr inbounds nuw i8, ptr %6, i64 56`(74646) · %6 = TLS RefCell ptr(74614 call_once 반환) → RefCell+0x38 에 게임 seed(%18 = data.cache.game.seed())를 실제로 store 한다. 게이트가 base 로 잡은 「%124 shift +8」 은 이 함수 본문(m15.ll)의 ProjectileIter next 반환값이라 다른 객체)",
      evidence=u"m00.ll:74646 `%15 = getelementptr inbounds nuw i8, ptr %6, i64 56` · 74830 `store i64 %18, ptr %15, align 8` · 74650 `icmp eq i64 %16, %18`(불일치 분기 %72 → 74830)",
      behavior_change=False, found_by="new", kind=u"오탐")
# G1 — 오탐 (Option 겹수)
p.fix("/specs[259]/sig/tls/layout",
      old=u"tcxdict V48ProfileCache = map@0x0(48B)·seed@0x30(8B)",
      new=u"tcxdict V48ProfileCache = map@0x0(48B)·seed@0x30(8B) (★G1 「Option 겹수」 오탐: `Option<Option<(u64,u64,u64)>>` 는 캐시 값형(미기록/None확정/Some 3상태)이고 `Option<Effect>`·`Option<&Entity>` 는 슬롯 이펙트·인자형 — 서로 다른 값의 타입이라 같은 사실을 다르게 적은 것이 아니다. 게이트가 「같은 대상」 을 확인하지 않고 두 정규식의 공존만 본다)",
      evidence=u"m00.ll:74899 phi -1 / 75073 default -1 (Option<Option<…>> 외측 None) · m15.ll:30703/30710/30726 icmp -1 (Option<Effect> 니치) — 별개 값",
      behavior_change=False, found_by="new", kind=u"오탐")
# G9 — 오탐(radius 는 필드) + logic 표기 개선
p.fix("/specs[259]/logic",
      old=u"halfwidth = if p.shape == Circle { p.shape.radius } else { 20000 }",
      new=u"halfwidth = match p.shape { ProjectileShape::Circle{radius} => radius, _ => 20000 }   // radius = Circle variant 의 필드(tcx game_core::ProjectileShape::Circle::radius · projectile.rs:65 · +0x18) — ProjectileShape 에 radius() 메서드는 없다(★G9 「메서드」 판정 오탐 · 메서드로 적으면 존재하지 않는 호출) · 오라클 27E A(Circle 7000→7000)·B(Rect→20000)·D(Circle 1234→1234) 일치",
      evidence=u"_tcx/game_core.json: `game_core::ProjectileShape::Circle::radius` k=Field v=pub (projectile.rs:65) · ProjectileShape 의 AssocFn 은 is_in 하나 · m15.ll:30889~30892 tag==0 → 30924 load +24",
      behavior_change=False, found_by="new", kind=u"오탐")
p.errors.append({"path": "/specs[259]/sig/exe_abi", "kind": u"보강", "old": None,
                 "new": {"exe": "0xeba320", "exe_src": EXE058, "nargs_exe": 4, "nargs_ir": 4,
                         "map": ["rcx = sret Option<(u64,u64,u64)> 32B (IR %0)", "rdx = data (IR %1)", "r8 = caster (IR %2)", "r9 = slot u8 (IR %3)"],
                         "evidence": "argscan058 0xeba320: 스택 인자 0 + 레지스터 4 = 4 = IR 4 (외부 심볼 · 승격 없음)"},
                 "evidence": u"argscan058 0xeba320", "behavior_change": False, "found_by": "new"})
# ── 오라클 ev 상향(o259 · 케이스당 프로세스 1개 · 11/11 예측 일치) ──
O259 = u"오라클 실행 확인: 27E o259.exe(pub 직접 호출 · 케이스당 프로세스 1개) — A Linear(Circle 7000·speed 4321·Direction)→Some(4321,7000,0) · B Linear(Rect·speed 999·Position)→Some(999,20000,0) · C lv2 slot1→None 후 같은 프로세스 lv9→None(캐시 재생) · C2 새 프로세스 lv9→Some · D lv5 ult RangeProjectile(Circle 1234·apply 5)→Some(0,1234,5) · D2 lv4→None · E Targeting→None · F AttackEffect→None · G 동명 두 캐스터가 같은 항목 공유 · H 슬롯 None→None; %s"
for path, why in (
        ("/specs[259]/mem[33]", u"raw_tag 0/1 실측(None/Some)"),
        ("/specs[259]/mem[34]", u"speed 4321/999/0 실측"),
        ("/specs[259]/mem[35]", u"halfwidth 7000/20000/1234 실측"),
        ("/specs[259]/mem[36]", u"delay 0/5 실측(Delayed.applyed)"),
        ("/specs[259]/mem[4]", u"slot 0 → skill_effect 에 넣은 이펙트가 소비됨(A/B/E/F/H)"),
        ("/specs[259]/mem[6]", u"level 게이트 실측: slot1 lv2 None·lv9 Some / slot2 lv4 None·lv5 Some"),
        ("/specs[259]/mem[7]", u"slot 1 → skill2_effect 소비(C2)"),
        ("/specs[259]/mem[9]", u"slot 2 → ult_effect 소비(D)"),
        ("/specs[259]/mem[11]", u"casting Targeting→None · Position/Direction→Some(E/A/B)"),
        ("/specs[259]/mem[24]", u"shape Rect → halfwidth 20000(B) · Circle → radius(A/D)"),
        ("/specs[259]/mem[25]", u"Circle.radius 7000/1234 그대로 halfwidth"),
        ("/specs[259]/mem[26]", u"LinearDist.speed 4321/999 그대로 speed"),
        ("/specs[259]/mem[27]", u"Delayed.applyed 5 그대로 delay(D)"),
):
    p.ev(path, evidence=O259 % why, to=3, frm=4, found_by="new")
for path, why in (
        ("/specs[259]/consts[3]", u"level>2 게이트: lv2 None(C) · lv9 Some(C2)"),
        ("/specs[259]/consts[4]", u"level>4 게이트: lv4 None(D2) · lv5 Some(D)"),
        ("/specs[259]/consts[9]", u"비-Circle(Rect) halfwidth 20000 실측(B)"),
        ("/specs[259]/consts[1]", u"sret tag 0(None) 실측 raw_tag=0 (C/D2/E/F/H)"),
        ("/specs[259]/consts[2]", u"sret tag 1(Some) 실측 raw_tag=1 (A/B/C2/D/G)"),
        ("/specs[259]/knobs[3]", u"C(lv2 None → lv9 여전히 None 캐시) · C2/D/D2 게이트 경계 실측"),
        ("/specs[259]/knobs[4]", u"G: 동명(swordman) 두 캐스터(id 18·19, 이펙트 speed 4321 vs 1) 중 두 번째가 첫 값(4321,7000,0) 재생 = 이름 키 공유 확증"),
):
    p.ev(path, evidence=O259 % why, to=2, frm=4, found_by="new")

# ═══════════════ 260 resolve_fight_stake_roster ═══════════════
p.fix("/specs[260]/sig/params[4]/name", old=u"roster.data_ptr", new=u"roster",
      evidence=u"P3 규약: 팻포인터 1 소스 인자 = 1행(35 resolve_fight_stake · 41 fight_participants 선례 `&[&Entity] (ptr %6, len %7)`)", behavior_change=False, found_by="reused")
p.fix("/specs[260]/sig/params[4]/type", old=u"&[(&Entity, i64, bool)] 팻포인터 data", new=u"&[(&Entity, i64, bool)] (팻포인터 2슬롯: ptr %4 · len %5)",
      evidence=u"m10.ll:47870 define 인자 5·6 = `ptr … %4, i64 noundef range(i64 0, 384307168202282326) %5`", behavior_change=False, found_by="reused")
p.fix("/specs[260]/sig/params[4]/role",
      old=u"`noalias nonnull readonly captures(address, read_provenance)` · 원소 24B: +0 &Entity(DI `a`) · +8 i64(도착 틱 → arrivals) · +16 bool(DI `bound`, 묶임 여부). tcx sig 확정 `&[(&Entity, i64, bool)]`",
      new=u"%4 = 원소 시작 `noalias nonnull readonly captures(address, read_provenance)` · %5 = len `range(i64 0, 384307168202282326)` = 2^63/24 → stride 24 확정. 원소 24B: +0 &Entity(DI `a`) · +8 i64(도착 틱 → arrivals) · +16 bool(DI `bound`, 묶임 여부). tcx sig 확정 `&[(&Entity, i64, bool)]`. exe 0xe0b030 도 [rsp+0x20]/[rsp+0x28] 두 슬롯(argscan 0.5.8 백업 exe)",
      evidence=u"m10.ll:47870 define · 47931 `gepS %4, i64 %5`(끝 포인터) · argscan058 0xe0b030 entry+0x20/+0x28", behavior_change=False, found_by="reused")
p.fix("/specs[260]/sig/params[6]/name", old=u"near_enemies.data_ptr", new=u"near_enemies",
      evidence=u"P3 규약(팻포인터 1행)", behavior_change=False, found_by="reused")
p.fix("/specs[260]/sig/params[6]/type", old=u"&[&Entity] data", new=u"&[&Entity] (팻포인터 2슬롯: ptr %6 · len %7)",
      evidence=u"m10.ll:47870 define 인자 7·8 = `ptr … %6, i64 noundef range(i64 0, 1152921504606846976) %7`", behavior_change=False, found_by="reused")
p.fix("/specs[260]/sig/params[6]/role",
      old=u"`noalias nonnull readonly` · 본문에서 안 읽음 — 세 콜리에 그대로 전달",
      new=u"%6 `noalias nonnull readonly` · %7 `range(i64 0, 1152921504606846976)` = 2^60 → stride 8. 본문에서 안 읽음 — 세 콜리(resolve_fight_full ×3 · m10.ll:48003/48127/48191)에 그대로 전달. exe 0xe0b030 [rsp+0x30]/[rsp+0x38]",
      evidence=u"m10.ll:48003·48127·48191 invoke resolve_fight_full(… ptr %6, i64 %7 …)", behavior_change=False, found_by="reused")
p.errors.append({"op": "delete", "path": "/specs[260]/sig/params[7]", "guard": u"near_enemies.len",
                 "kind": u"실오류", "old": u"near_enemies.len", "new": u"",
                 "evidence": u"params[6] 로 병합(P3: 팻포인터 = 1행 · i=5 중복 제거)", "behavior_change": False, "found_by": "reused"})
p.errors.append({"op": "delete", "path": "/specs[260]/sig/params[5]", "guard": u"roster.len",
                 "kind": u"실오류", "old": u"roster.len", "new": u"",
                 "evidence": u"params[4] 로 병합(P3: 팻포인터 = 1행 · i=4 중복 제거)", "behavior_change": False, "found_by": "reused"})
p.errors.append({"path": "/specs[260]/sig/exe_abi", "kind": u"보강", "old": None,
                 "new": {"exe": "0xe0b030", "exe_src": EXE058, "nargs_exe": 11, "nargs_ir": 11,
                         "map": ["rcx = sret FightPrediction 64B (IR %0)", "rdx = version (IR %1)", "r8 = data (IR %2)", "r9 = champ (IR %3)",
                                 "[rsp+0x20] = roster.ptr (IR %4)", "[rsp+0x28] = roster.len (IR %5 · e0b212 `cmp [rbp+0x168],0`)", "[rsp+0x30] = near_enemies.ptr (IR %6)",
                                 "[rsp+0x38] = near_enemies.len (IR %7)", "[rsp+0x40] = committed_dir i8 (IR %8 · movzx byte e0b1ad/e0b374)", "[rsp+0x48] = tower Option<&Entity> (IR %9)", "[rsp+0x50] = judge_accuracy (IR %10)"],
                         "evidence": "argscan058 0xe0b030: 스택 7 + 레지스터 4 = 11 = IR 11 · 바이트 슬롯 위치(9번째)가 IR i8 %8 과 일치 (hidden 심볼 · 승격 없음)"},
                 "evidence": u"argscan058 0xe0b030", "behavior_change": False, "found_by": "new"})
O260 = u"오라클 실행 확인: 27E o260.exe(define hidden 심볼 link_name 직접 진입 · 케이스당 프로세스 1개) — R1/R2·R5·R6 전 케이스 diff.line_absolute == absolute.line(5/5) · R3 자기자신 bound 와 unbound 결과 58B 동일(id 필터) · R6(attack_effect 주입) dir=1/0 에서 diff.line(0/3)≠absolute.line(2) → rescue_ally=Some(20)=거리² 최소 아군(a2 dist² 1.25e8 < a1 9e8) · dir=-1 라인 동일 → rescue tag 0(값 슬롯 콜리 잔값 -2); %s"
for path, why in (
        ("/specs[260]/mem[8]", u"line_absolute 가 absolute.line 과 5/5 일치"),
        ("/specs[260]/mem[13]", u"+0x39 line_absolute 기록 5/5"),
        ("/specs[260]/mem[14]", u"rescue tag 1(라인 갈림 R6 dir 1/0) · 0(같음)"),
        ("/specs[260]/mem[15]", u"rescue id = 20 (최근접 remaining)"),
        ("/specs[260]/mem[4]", u"bound=false 편성만이면 unbound 결과(absolute) · bound 면 diff(R6u net -123172 vs R6 net 0)"),
        ("/specs[260]/mem[5]", u"R3: 자기 자신 bound 라도 remaining 에서 제외(unbound 와 58B 동일)"),
        ("/specs[260]/mem[6]", u"거리² 최소 선택에 x 사용(a2 x+10000 vs a1 x+30000)"),
        ("/specs[260]/mem[7]", u"거리² 최소 선택에 y 사용(a2 y+5000)"),
):
    p.ev(path, evidence=O260 % why, to=3, frm=4, found_by="new")
for path, why in (
        ("/specs[260]/knobs[2]", u"remaining 필터 실측(R3·R6u/R6)"),
        ("/specs[260]/knobs[3]", u"최근접(dist²) 아군 id 20 선택 실측"),
        ("/specs[260]/consts[1]", u"rescue_ally Some 태그 1 실측(R6 dir 1/0)"),
):
    p.ev(path, evidence=O260 % why, to=2, frm=4, found_by="new")

# ═══════════════ 261 kite_reposition_point ═══════════════
p.fix("/specs[261]/sig/params[5]/name", old=u"nearest", new=u"nearest (&Entity → LTO ArgumentPromotion 2 스칼라: x i64 %5 · y i64 %6)",
      evidence=u"m02.ll:74099 define 인자 6·7 = `i64 %5, i64 %6` · 호출자 29948/29949 load Entity+1632/+1640", behavior_change=False, found_by="reused")
p.fix("/specs[261]/sig/params[5]/type",
      old=u"&Entity → 스칼라 승격 i64 %5(=nearest.x, Entity+0x660) · i64 %6(=nearest.y, Entity+0x668)",
      new=u"(i64, i64)",
      evidence=u"m02.ll:74099 define `… ptr … %4, i64 %5, i64 %6, i64 noundef %7)` — 소스 &Entity 1인자가 IR 2 스칼라(ScalarPair 표기 · paramrole.ir_slots 2). 원래 type 문면은 role 로 이동", behavior_change=False, found_by="reused")
p.fix("/specs[261]/sig/params[5]/role",
      old=u"DI arg 6 이름 nearest(!77530, battle.rs:1218). LTO ArgumentPromotion 으로 IR define 엔 &Entity 대신 x·y 두 i64 가 들어온다",
      new=u"IR 슬롯 = i64 %5(=nearest.x, Entity+0x660) · i64 %6(=nearest.y, Entity+0x668). DI arg 6 이름 nearest(!77530, battle.rs:1218). LTO ArgumentPromotion 으로 IR define 엔 &Entity 대신 x·y 두 i64 가 들어온다",
      evidence=u"m02.ll:74099 define · 74121 `sub i64 %13, %5` · 74126 `sub i64 %16, %6` · 74219/74348 `add … %5` · 74352 `add … %6`", behavior_change=False, found_by="reused")
p.fix("/specs[261]/open[0]",
      old=u"version/player/data/parameter/champ 중 어느 하나가 exe 에서 소거됐는지(DeadArgElim/IPSCCP) 미확정. 호출부 0xcbe020 은 rcx=[rbp+0x538]·rdx=[rbp+0x620]·r8=[rbp+0x628]·r9=[rbp+0x588] 을 넘김. 확정하려면 ghidra-re 로 호출자 0xcbbdb0 의 4 레지스터 출처 대응(sweep 편입 전 필수, EXE_ABI 판정)",
      new=u"**소거된 것은 parameter(%3)** (27E 확정 · 0.5.8 백업 exe 디스어셈): exe 7 = rcx version(%0) · rdx player(%1) · r8 data(%2) · r9 champ(%4) · [rsp+0x20] nearest.x(%5) · [rsp+0x28] nearest.y(%6) · [rsp+0x30] attack_range(%7). 근거 ①콜리 본체 cd5f26 `mov rcx,[r9+0x660]`·`[r9+0x668]`·cd5f86 `[r14(=r9)+0x470]`·`[r14+0x680]` = champ ②r8→rsi: cd5f82 `mov r9,[rsi+8]` → `[r9+8]`(setting)·`[r9+0x20]`(map) 이 adjust_position(cd5fcc) 인자 = data ③cd604a~cd605a position_eval_at(0xd84db0) 호출 rdx=rbx(exe rcx)·r8=[rsp+0x48](exe rdx)·r9=rsi(data) ↔ IR position_eval_at(version, player, data, x, y, purpose) 순서 → rcx=version·rdx=player ④cd60a3~cd60b8 v48_on_cast_line(0xcd0480) rcx=[rsp+0x48](player)·rdx=rsi(data) 도 일치 ⑤콜리 본체에 `+0x9f0`(positioning_score) 접근 0 — exe 에서 position_score_at_position 이 인라인되며(cd5fd9~cd6035: /32000·min 29·×32000+16000 셀 스냅 후 position_eval_at 호출) &PositioningScoreData 가 죽어 DAE 됨. 호출부 0xcbe020 의 rcx=[rbp+0x538](지역 스칼라 · version)·rdx=[rbp+0x620]·r8=[rbp+0x628]·r9=[rbp+0x588](champ) — [rbp+0x620]/[rbp+0x628] 은 호출자 base_positioning 의 exe 6·7번째 인자(호출자 자체의 exe ABI 는 미확정). 상세 = sig.exe_abi · EXE_ABI: 재호출 시 parameter 를 빼고 7 인자로",
      evidence=u"argscan058 0xcd5ee0(스택 3+레지스터 4=7) · xdis cd5ee0~cd60bd · IR m02.ll:74230 position_score_at_position(sret, %0, %1, %2, %29=parameter+2544, …) ↔ exe 인라인",
      behavior_change=False, found_by="new", kind=u"보강")
p.errors.append({"path": "/specs[261]/sig/exe_abi", "kind": u"보강", "old": None,
                 "new": {"exe": "0xcd5ee0", "exe_src": EXE058, "nargs_exe": 7, "nargs_ir": 8,
                         "map": ["rcx = version (IR %0)", "rdx = player (IR %1)", "r8 = data (IR %2)", "r9 = champ (IR %4)",
                                 "[rsp+0x20] = nearest.x (IR %5)", "[rsp+0x28] = nearest.y (IR %6)", "[rsp+0x30] = attack_range (IR %7)"],
                         "eliminated": "parameter &ScoreParameter (IR %3) — exe 에서 position_score_at_position 인라인 후 &positioning_score 미사용 → DAE",
                         "ret": "{i64,i64} = rax(x)·rdx(y) (cd6372 `mov rax, rbx` … 두 워드 반환)",
                         "evidence": "argscan058 0xcd5ee0 · cd5f26 [r9+0x660] · cd5f82 mov r9,[rsi+8] · cd604a~cd605a position_eval_at(rdx=rbx, r8=[rsp+0x48], r9=rsi) · cd60a3~cd60b8 v48_on_cast_line(rcx=[rsp+0x48], rdx=rsi) · 본체에 +0x9f0 접근 0",
                         "sweep": "EXE_ABI: IR 8 ≠ exe 7 — 재호출은 parameter 를 빼고 (version, player, data, champ, nx, ny, range)"},
                 "evidence": u"위 map/evidence 필드 · 0.5.8 백업 exe 기준", "behavior_change": False, "found_by": "new"})

# ═══════════════ brief_errors ═══════════════
p.brief_error(u"★게임 exe 가 2026-09-16 14:40 에 패치됐다(86,082,048B · mig_verify check 전 0.5.8 항목 STALE). 도시에·로스터·spec 의 exe RVA(e248f0/eba320/e0b030/cd5ee0)는 0.5.8 exe 기준인데 argscan/fnprobe/xdis 등 9개 도구가 라이브 exe 경로를 하드코딩해 라이브 exe 로 돌리면 「.pdata 못 찾음」·「스택 인자 0」 같은 거짓 결과가 나온다(258 open[2] 의 「스택 인자 0·xmm0」 이 그 사고). 0.5.8 백업 = C:\\Users\\jungs\\Desktop\\claude\\tfm2\\tfm2_0.5.8\\TeamfightManager2.exe(77,666,816B) 로 재측정했다. 제안: 도시에 §0 에 exe 경로+크기/sha 스탬프를 넣고 도구는 --exe 를 받게.")
p.brief_error(u"§4 G9 지시 「logic 표기를 `radius()` 로 고칠 것」 은 틀렸다 — tcx 에 ProjectileShape::radius 메서드는 없고(AssocFn 은 is_in 하나) radius 는 Circle/DirDot variant 의 **필드**다. 지시대로 고쳤으면 존재하지 않는 호출을 명세에 적는 판정 반전 오류. match 패턴 표기로 고쳤다.")
p.brief_error(u"「patch 에 `sig.exe_abi` 로 확정」 — sig 에는 이미 `abi`(문자열) 칸이 있고 4함수 전부 null 인데 applypatch 는 null 문자열 칸을 채울 수 없다(dict 만 가능). 그래서 `sig.exe_abi` dict 로 냈다 — `abi`(문자열)·`exe_abi`(dict) 두 칸의 관계를 도구/도시에가 정해야 한다(mkdossier 는 exe_abi 를 렌더하지 않는다).")
p.brief_error(u"G1 「Option 겹수」 규칙이 같은 대상인지 보지 않고 `Option<Option<` 와 bare `Option<X>` 의 공존만 잡는다 — TLS 캐시 값형이 Option<Option<T>> 인 함수(259)는 인자형 Option<&Entity>·Option<Effect> 와 늘 공존하므로 구조적 오탐. 규칙을 「같은 심볼/필드에 두 겹수」 로 좁혀야 한다.")
p.brief_error(u"G14 base 동정이 aux 파일(m00.ll)의 클로저를 본 함수 파일(m15.ll)의 레지스터 번호로 해석했다(mem[38] `%124 shift +8`) — mem 행 근거가 `aux mNN.ll:LINE` 이면 그 파일에서 base 를 찾아야 한다.")
p.brief_error(u"knobval(G19) 에 부호접힘 구제가 없다 — `x.saturating_sub(N)`/`x - N` 이 `add x, -N` 으로 접히면 값 N 인 노브가 V1 로 잡힌다(258 knobs[1]). 구제 규칙 「창에 -N 관측 = N 인정」 제안.")
p.brief_error(u"TEMPLATE.mkgame 은 10명 전원 이름이 \"swordman\" 이라 v48 처럼 **이름을 TLS 키로 쓰는 함수**는 다른 엔티티끼리도 캐시를 공유한다(o259 G 케이스 실측) — 오라클 함정 ③ 의 변형으로 TEMPLATE 에 적어 둘 것.")
p.brief_error(u"지시문 규칙 「파이썬은 Write 도구로」 를 o260.rs 수정 1회에서 heredoc 으로 어겼다(자기 신고 · 결과는 Edit 로 마무리).")
p.save()
