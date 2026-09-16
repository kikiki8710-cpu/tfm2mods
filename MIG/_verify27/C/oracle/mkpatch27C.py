# -*- coding: utf-8 -*-
"""27차 배치C patch.json 생성 — mkpatch 참조구현 사용. 오라클 결과 확정 후 ev_up 절을 채운다."""
import sys, io, json
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch

p = mkpatch.Patch(round=27, batch="C")
V3 = mkpatch.spec()["specs"]

# ───────────────────────────── 252 v16_knight_ult_zone_bonus ─────────────────────────────
# G12 consts[7] (50 @222) — 오탐: phi 상수 인입
p.fix("/specs[252]/consts[7]/meaning",
      old=u"hp_value 기본값(phi %583 [50,%559])",
      new=u"hp_value 기본값(phi %583 [50,%559] — ★G12 오탐: m05.ll:57343 `%583 = phi i64 [ 50, %559 ], [ %581, %573 ], !dbg !57151` 의 !57151 = line 0 · phi 상수 인입이라 L222 의 DILocation 자체가 없다 · 인입 블록 %559 의 종결자 57306 `br i1 %561, label %582, label %562, !dbg !57669` 는 macros.rs:180<348<L214(find 탈출) · rmeta 줄 길이 L221 13 / L222 14 / L223 7 자가 `} else {`/`(0, 50)`/`};` 3줄 형태와 정합 → 222 유지)",
      evidence=u"m05.ll:57343 `%583 = phi i64 [ 50, %559 ], [ %581, %573 ], !dbg !57151` · dloc !57151 = line 0 in v16_knight_ult_zone_bonus · 57306 `br i1 %561, label %582, label %562, !dbg !57669`(!57669 = macros.rs:180 < 348 < battle_common.rs:214) · rmeta_srcmap battle_common.rs L221~223 = 13/14/7 자 · 게이트가 낸 258 은 다른 상수(consts[22] smin 50 · 57238)",
      behavior_change=False, found_by="reused", kind=u"오탐")
# G12 consts[16] (6 @254) — 실오류: IR 리터럴 6 은 L258 dbg 의 인수분해 mul 에만 있다(루트 줄 규약)
p.fix("/specs[252]/consts[16]/src_line", old=254, new=258,
      evidence=u"m05.ll:57240~57241 `%524 = add nsw i64 %521, %516, !dbg !57637` · `%525 = mul i64 %524, 6, !dbg !57637` · dloc !57637 = line 258 in v16_knight_ult_zone_bonus(인라인 없음) — 리터럴 6 은 IR 사슬에 L258 로만 존재(L254 `add nsw i64 %523, 5, !dbg !57638`=254 에는 6 없음). 소스상 두 계수(L254 allies·L257 enemies_in_zone)가 6 이라는 사실은 meaning 에 유지",
      behavior_change=False, found_by="reused")
p.fix("/specs[252]/consts[16]/meaning",
      old=u"allies_in_zone 계수 — IR 은",
      new=u"allies_in_zone·enemies_in_zone 공통 계수(소스 L254·L257 · IR 위치는 L258 dbg 의 인수분해 mul 57241 — src_line 은 루트 줄 규약대로 258) — IR 은",
      evidence=u"위 src_line 정정과 짝(문면 동기)", behavior_change=False, found_by="reused", kind=u"보강")
# G13 knobs[6] where — g07.ll 줄번호 오기
p.fix("/specs[252]/knobs[6]/where",
      old=u"g07.ll:157025 `add 120`", new=u"g07.ll:157039 `%25 = add i64 %24, 120`",
      evidence=u"g07.ll:157039 `%25 = add i64 %24, 120, !dbg !187710` (157025 는 `br i1 %17, label %30, label %18` = get_player_by_champion_id None 분기) · 157036~157043: self+480(last_visible)[position] 로드 → +120 → `icmp uge %25, %28(tick)`",
      behavior_change=False, found_by="reused")
# G7 open[2] — is_recent_visible 확정으로 해소(사실 서술 → 닫힘)
p.fix("/specs[252]/open[2]",
      old=u"IR·줄 길이 모두 1-team. 판정 동작은 확정, 의도만 미상. blackboard.last_visible 의 기록 주체(어느 팀 기준 '마지막 가시'인지)는 game_core 경계라 미독",
      new=u"★해소 — 오타가 아니라 정합이다. is_recent_visible(g07.ll:157005~157048) = game.is_visible(player.info.team, target.id)(vtable+0xf8) || { p = get_player_by_champion_id(target.id)(vtable+0x150); self.last_visible[+0x1e0][p.info.position(+0x9c0)] + 120 >= game.tick()(vtable+0x28) } 이므로 self 는 **target(적) 챔피언이 속한 팀의 blackboard** 여야 하고, target = enemy(1-team) 이라 [1-team] 이 맞다(_docs game_core.txt:21 「Blackboard[team]은 team 팀 자체 정보 추적, 관측은 1-team」 과 정합 · tcxdict Blackboard+0x1e0 last_visible [usize;5] = 포지션 인덱스). 27차 배치C 오라클 o254(같은 콜리·같은 인덱스 관례) last_visible=tick-120 → 가시 / tick-121 → 비가시 실측(case 18/19)",
      evidence=u"g07.ll:157011~157047 원문(157017 `tail call … %11(ptr %1, i64 %7, i64 %9)` = is_visible(team,id) · 157023 get_player_by_champion_id · 157031~157032 `gep %16, 2496`(+0x9c0 position) · 157036~157039 `gep %0, 480` + position*8 → `add 120` · 157043 `icmp uge`) · tcxdict Blackboard 0x1e0 last_visible [usize;5]",
      behavior_change=False, found_by="reused")

# ───────────────────────────── 253 LineSafeSubPlan::action_candidates ─────────────────────────────
# G5 self 가변성 — tcx `&mut`
p.fix("/specs[253]/sig/params[1]/ty",
      old=u"&LineSafeSubPlan (1B: +0x0 line: LineType u8 Top=0/Mid=1/Bottom=2)",
      new=u"&mut LineSafeSubPlan (1B: +0x0 line: LineType u8 Top=0/Mid=1/Bottom=2)",
      evidence=u"tcx sig `fn(&mut game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut StdRng, …)` · IR define m02.ll:47101 `%1 = ptr noalias noundef dereferenceable(1)`(readonly 없음 = &mut 과 정합) · 본문 `store … ptr %1` 0회(47272 `store ptr %1, ptr %77` 은 %1 **값**을 클로저 env 에 저장) ⟹ 선언은 &mut, 동작은 읽기만",
      behavior_change=False, found_by="reused")
p.fix("/specs[253]/logic",
      old=u"fn action_candidates(&self, version, rnd, player, data, _parameter)",
      new=u"fn action_candidates(&mut self, version, rnd, player, data, _parameter)",
      evidence=u"tcx 정본 sig(&mut self) · 위 params[1] 정정과 동기(G6 예방)", behavior_change=False, found_by="reused")
# G16 params[3] rnd role — define 에 없는 속성어(readnone/readonly) 제거
p.fix("/specs[253]/sig/params[3]/role",
      old=u"IR 속성 없음(readnone/readonly 모두 없음) · 본문 직접 접근 0회 · gen_range 호출 사이트 0개 · 전달처 2곳: SmallActionAround::new(L34 또는 L37 중 하나만 실행 · 콜리 define 이 `_rnd` readnone) · fight_check::battle_action(L94 · 콜리 define 이 `_rnd` readnone) ⟹ 이 함수 서브트리에서 rnd 상태 변화 0 (콜리 속성 근거)",
      new=u"IR 속성: noalias align 16 dereferenceable(320) 만(메모리 접근 속성 없음 · m02.ll:47101 %3) · 본문 직접 접근 0회 · gen_range 호출 사이트 0개 · 전달처 2곳: SmallActionAround::new(L34 또는 L37 중 하나만 실행 · 콜리 define m08.ll:92385 의 `_rnd`(%2) 는 메모리 미접근 속성 보유) · fight_check::battle_action(L94 · 콜리 define m15.ll:23700 의 `_rnd`(%2) 도 같음) ⟹ 이 함수 서브트리에서 rnd 상태 변화 0 (콜리 속성 근거)",
      evidence=u"m02.ll:47101 define: `ptr noalias noundef align 16 dereferenceable(320) %3`(readnone/readonly 없음) · m08.ll:92385 `SmallActionAround::new(… ptr noalias noundef readnone align 16 captures(none) dereferenceable(320) %2 …)` · m15.ll:23700 `battle_action(… ptr noalias readnone align 16 captures(none) %2 …)` — G16 이 role 문면의 속성어를 이 define 의 주장으로 읽어 P4 를 냈다. 사실은 콜리 속성이라 문면에서 속성 토큰만 제거(사실 불변)",
      behavior_change=False, found_by="reused", kind=u"오탐")
# exe ABI(0.5.8 백업 exe) — exe.evidence 보강 + callers 채움
p.fix("/specs[253]/exe/evidence", old=u"fp",
      new=u"fp · argscan(0.5.8 백업 exe) 0xccacf0: .pdata 0xccacf0~0xccb943(3155B · 675 insn) · rcx=%0 sret · rdx=%1 self · r8=%2 version · r9=%3 rnd · [entry+0x20]=%4 player(x9) · [entry+0x28]=%5 data(x8) · %6 _parameter 는 본문 미사용이라 진입 스캔엔 안 보이고 호출자 0xe35bd0@e35e96 이 [rsp+0x20/0x28/0x30] 3슬롯을 채움 → exe 7 = IR 7 · ArgumentPromotion 없음. 패닉 Location line_safe.rs:26:8·28:52·simulation.rs:1824~1826 = 명세 L26/L28 unwrap·tower() bounds 와 정합",
      evidence=u"scratchpad27C\\fnprobe058.py 0xccacf0 · argscan058.py 0xccacf0 / --caller 0xe35bd0 (EXE = tfm2_0.5.8\\TeamfightManager2.exe — 설치본 exe 는 09-16 14:40 패치로 바뀌어 mig_verify STALE 1144)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[253]/exe/callers", old=u"[]", new=u'["e35bd0"]',
      evidence=u"fnprobe058 0xccacf0 callers(.text raw scan) 1: @e35e96 in fn 0xe35bd0(1230B) — IR 호출처 1곳(m12.ll:35190 SubPlan::action_candidates 디스패치)과 일치",
      behavior_change=False, found_by="new", kind=u"보강", force=True)

# ───────────────────────────── 254 noncombat_steroid_window ─────────────────────────────
# G12 consts[12] (8 @413) — 오탐: switch case 라벨
p.fix("/specs[254]/consts[12]/meaning",
      old=u"SmallAction 태그 8 = Skill2. aux m10.ll:6641.",
      new=u"SmallAction 태그 8 = Skill2. aux m10.ll:6641 switch case 라벨(★G12 오탐: case 라벨 줄엔 !dbg 가 없고, arm 블록 %53 첫 명령 6645 `gep %22, 128, !dbg !18593` = buff_value.rs:413 in closure$0 <406 closure$2 <427 closure$3 라 413 이 맞다 · 게이트의 436 은 본문의 다른 8).",
      evidence=u"m10.ll:6635~6642 `switch i64 %52, label %77 [ i64 9, label %73 / i64 2, label %69 / i64 4, label %65 / i64 6, label %61 / i64 7, label %57 / i64 8, label %53 ], !dbg !18591`(!18591=411) · 6644~6648 블록 53: `%54 = getelementptr inbounds nuw i8, ptr %22, i64 128, !dbg !18593`(dloc !18593 = line 413 in closure$0 → 406 closure$2 → 427 closure$3) · 6647 `icmp eq i64 %55, %6, !dbg !18594`(=415)",
      behavior_change=False, found_by="reused", kind=u"오탐")
# G7 open[0] — is_recent_visible 언급 제거(표기 불가 유지)
p.fix("/specs[254]/open[0]",
      old=u"IR 은 is_recent_visible true → 즉시 {2,undef} 반환으로 접혀 있고",
      new=u"IR 은 enemy_near 판정(390~394 any 클로저)이 true 인 칸에서 즉시 %440 {2,undef} 반환으로 접혀 있고(m10.ll:37655~37660 phi `[ 2, %214 ] … [ undef, … ]`)",
      evidence=u"m10.ll:37655~37657 `%441 = phi i8 [ undef, %214 ], [ undef, %159 ], … , [ %438, %437 ]` · `%442 = phi i8 [ 2, %214 ], [ 2, %159 ], … , [ %439, %437 ]` — G7 이 open 의 is_recent_visible 이름 인용을 과열림으로 잡음 · 물음의 본체(395~404 줄 형태)는 표기 불가 그대로",
      behavior_change=False, found_by="reused", kind=u"보강")
# G7 open[2] — Blackboard 인덱스 의미 해소
p.fix("/specs[254]/open[2]",
      old=u"'blackboard[t] = t 팀 챔피언들의 기록' 으로 읽으면 정합. is_recent_visible 내부(g07.ll:157005 · 계약만)는 안 읽음",
      new=u"★해소 — 'blackboard[t] = t 팀 챔피언들의 기록' 이 맞다. is_recent_visible(g07.ll:157005~157048) 은 self.last_visible[+0x1e0][target 의 player.info.position] + 120 >= game.tick() 을 보므로 self 는 target(적) 팀 blackboard = blackboard[enemy_team] 이어야 하고, team_action_hits 의 blackboard[my_team].small_actions[pi] 는 내 팀 챔피언 pi 의 소액션 기록 — 같은 관례. 27차 배치C 오라클 o254 case 18/19: bb[enemy].last_visible[enemy.pos] = tick-120 → 적 근접 판정 → None / tick-121 → 비가시 → Some 실측(24/24 MATCH)",
      evidence=u"g07.ll:157011~157047 원문 · tcxdict Blackboard 0x1e0 last_visible [usize;5] · 0x78 small_actions [Option<SmallAction>;5] · _docs game_core.txt:21 · _verify27\\C\\oracle\\o254_case18.log/o254_case19.log",
      behavior_change=False, found_by="reused")
# open[7] — exe 0xd75cb0 정체 해소
p.fix("/specs[254]/open[7]",
      old=u"exe 0xd75cb0(1166B · 콜리 0) = s1_0 클로저 추정(642 ins) — 이 클로저는 range_adjust 를 call 하므로 '콜리 0' 과는 안 맞을 수 있음(range_adjust 가 exe 에서 인라인됐거나 다른 함수일 가능성) · 미검증",
      new=u"★해소 — exe 0xd75cb0(0.5.8 백업 exe · .pdata 0xd75cb0~0xd7613e 1166B · 274 insn) = s1_0(in_window 클로저) 확정. fnprobe: 패닉 Location buff_value.rs:407:24(team_action_hits bounds check) · 점프테이블 5개(small_actions 5칸 switch 언롤) · 상수 0xd693a400 = 3600000000 · 인자 rcx,rdx 2개(env, target) · 콜리 = `call [rax+0xe8]` 1곳 + panic_bounds_check. 「콜리 0」은 오기 — `[rax+0xe8]` 이 Effect::range_adjust(g06.ll:51785~51803) 가 인라인된 잔여 = self.ty(Arc<dyn EffectType>) vtable+0xe8(232) 호출이다. 호출자 8곳 = 0xe03360(이 함수) 6곳 + 0xd717e0(286B · iter_minions any 체인) 2곳. 같은 크기 1166B 의 0xdef0a0·0xe229d0 도 e03360 에서 불리며 s1_0 의 LTO 복제본으로 추정",
      evidence=u"scratchpad27C\\fnprobe058.py 0xd75cb0 (EXE=tfm2_0.5.8 백업) · g06.ll:51800~51802 `%13 = getelementptr inbounds nuw i8, ptr %6, i64 232` → `%14 = load ptr` → `tail call noundef i64 %14(ptr %12, ptr %1, ptr %2)` = Effect::range_adjust 본체",
      behavior_change=False, found_by="new")
p.fix("/specs[254]/exe/evidence", old=u"fp",
      new=u"fp · argscan(0.5.8 백업 exe) 0xe03360: .pdata 0xe03360~0xe03eca(2922B · 669 insn) · rcx=%0 player · rdx=%1 data · r8=%2 caster · r9=%3 beneficiary · 스택 인자 0 → exe 4 = IR 4 · 반환 {i8,i8} 는 ax. is_recent_visible 은 exe 에서 인라인(콜리 [r13+0xf8] is_visible·[r13+0x150] get_player_by_champion_id·[r13+0x28] tick 가 이 함수 본문에 직접 보임) · 콜리 0xd75cb0/0xdef0a0/0xe229d0(각 1166B = s1_0 복제)·0xd717e0(286B) · 호출자 0xd5bbf0(19586B) 2곳 = IR 호출처 2곳(m05.ll:43307·43888)과 일치",
      evidence=u"scratchpad27C\\fnprobe058.py 0xe03360 · argscan058.py 0xe03360 (EXE = tfm2_0.5.8\\TeamfightManager2.exe)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[254]/exe/callers", old=u"[]", new=u'["d5bbf0"]',
      evidence=u"fnprobe058 0xe03360 callers 2: @d5f92f·@d5ff09 in fn 0xd5bbf0(19586B)", behavior_change=False, found_by="new", kind=u"보강", force=True)
p.fix("/specs[252]/exe/evidence", old=u"fp",
      new=u"fp · argscan(0.5.8 백업 exe) 0xd68090: .pdata 0xd68090~0xd69114(4228B · 1015 insn) · rcx=%0 player · rdx=%1 data · r8=%2 parameter · r9=%3 action · [entry+0x20]=%4 champ(x2) · [entry+0x28]=%5 target(x2) → exe 6 = IR 6 · ArgumentPromotion 없음. 패닉 Location simulation.rs:1905:5(iter_champions bounds)·battle_common.rs:232:25(sdiv overflow) = 명세 L201/L232 와 정합 · is_recent_visible 은 exe 에서 인라인([r12+0xf8]·[r12+0x150]·[r12+0x28]) · 콜리 [rax+0x68] as_any·[rax+0x18] type_id·0xd83230(459B)·0xd37680(129B)·0x1323a00(118B) · 호출자 0xcc3080(4491B)·0xeade00(3913B) 2곳 = IR 호출처 2곳(Battle/DeathBattle score)과 일치",
      evidence=u"scratchpad27C\\fnprobe058.py 0xd68090 · argscan058.py 0xd68090 (EXE = tfm2_0.5.8\\TeamfightManager2.exe)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[252]/exe/callers", old=u"[]", new=u'["cc3080", "eade00"]',
      evidence=u"fnprobe058 0xd68090 callers 2: @cc3f61 in fn 0xcc3080(4491B) · @eaeb9d in fn 0xeade00(3913B)", behavior_change=False, found_by="new", kind=u"보강", force=True)

# ───────────────────────────── ev_up: tcx chk OK 인 mem 행(상한 3) ─────────────────────────────
def mem_ev(i, rows, extra):
    for r in rows:
        p.ev("/specs[%d]/mem[%d]" % (i, r), to=3, frm=4,
             evidence=u"tcx 정본 대조(tcxaudit chk=OK · 오프셋·필드명 일치)%s" % extra, found_by="reused")

import runpy
ORACLE = {}
try:
    ORACLE = json.load(io.open(r"C:\Users\jungs\AppData\Local\Temp\claude\C--Users-jungs-Desktop-claude-tfm2--claude-worktrees-swap-order-button-style-fe9e04\8c613d5e-d69c-428f-aff5-fe06054da7f0\scratchpad\scratchpad27C\oracle_summary.json", encoding="utf-8"))
except Exception:
    pass
o254 = ORACLE.get("254", u"")
o252 = ORACLE.get("252", u"")
mem_ev(253, [r for r in range(0, 35) if r != 8], u"")
mem_ev(254, [r for r in range(0, 22)], (u" + 오라클 실행 확인(27차 배치C o254.rs %s)" % o254) if o254 else u"")
mem_ev(252, [r for r in range(3, 27) if r not in (14, 26)], (u" + 오라클 실행 확인(27차 배치C o252.rs %s)" % o252) if o252 else u"")

# consts/knobs ev_up — 오라클이 실제로 닿은 행만 (결과 확정 후 채움)
FRM = {"/specs[252]/consts[2]": 3, "/specs[254]/consts[2]": 5}   # 현재 ev 가 4 가 아닌 행(dry-run 경고)
for path, ev in ORACLE.get("rows", []):
    p.ev(path, to=2, frm=FRM.get(path, 4), evidence=ev, found_by="new")

p.ev("/specs[253]/consts[8]", to=4, frm=5, evidence=u"IR 독해 확정: m02.ll:48222 `%416 = mul i64 %349, 30, !dbg !53850`(dloc = line 59 in attack_tower_action < 97 action_candidates) · %349 = champ.stat_cached.move_speed(+0x640) · 48223~48228 7항 덧셈 → 48230 제곱 → 48231 `icmp ugt %373(dist²), %423` — 리터럴·줄·소비 전부 IR 에 있다(추론 5 가 아니라 IR 4)", found_by="new")
for b in ORACLE.get("brief", []):
    p.brief_error(b)
p.brief_error(u"설치본 exe 가 2026-09-16 14:40 에 바뀌었다(패치 · `mig_verify.py check` = PASS 0 / STALE 1144). fnprobe.py·argscan.py 의 EXE 상수가 설치본을 가리켜 r18 exe 주소(d68090·ccacf0·e03360·d75cb0) 전부가 「not a pdata start」로 나온다 — 도시에·METHOD_MAP 어디에도 경고가 없다. 0.5.8 재료는 `C:\\Users\\jungs\\Desktop\\claude\\tfm2\\tfm2_0.5.8\\TeamfightManager2.exe`(09-02 백업) 로 재현됐다(fnprobe058/argscan058 = EXE 상수만 바꾼 복제). 도구에 `--exe` 인자 또는 STAMP 의 exe 해시 검증이 필요하다")
p.brief_error(u"v27_prompt_C 초점 ③(kite_reposition_point·choose_goal·push_candidate·base_battle_action)·④(base_defense_focus 인라인) 은 배치 C 담당 3함수(252~254)에 없는 함수다 — 공용 문안이 배치별로 안 갈렸다. C 몫으로는 담당 3함수의 exe↔IR 인자 대응(6=6 · 7=7 · 4=4)만 확정했다")
p.brief_error(u"§1 표의 exe 열 `(None) · None바이트 · None명령` 은 세 함수 모두 0.5.8 백업 exe 에서 채울 수 있다(d68090 4228B/1015 · ccacf0 3155B/675 · e03360 2922B/669) — 그런데 `exe.bytes`/`exe.insn` 이 정본에 키 자체가 없어 `/specs[i]/exe/bytes` 경로로는 applypatch 가 받지 못한다(cur None → 문자열 분기 실패). exe.evidence 문면에만 실었다")
p.save()
