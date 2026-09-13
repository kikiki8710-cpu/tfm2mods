# -*- coding: utf-8 -*-
# 19차 배치C patch.json 생성 — mkpatch 참조구현 사용. 재실행 가능.
import sys; sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch
p = mkpatch.Patch(round=19, batch="C")

# ── G16: params.i 규약(sret=0, 소스 1..n). 세 명세 모두 0..n-1 로 적혀 있었다 ──
for spec, n in ((93, 6), (94, 8), (95, 6)):
    for k in range(n):
        p.fix("/specs[%d]/sig/params[%d]/i" % (spec, k), old=k, new=k + 1,
              evidence=u"G16 규약(07·15 정본: sret=0, 소스 인자 1..n). 이 함수는 sret 없음(define 첫 인자 i64 %%0=version) — 소스 인자 %d개 ⟹ i=1..%d. IR define: m05.ll:%s" % (n, n, {93: 49223, 94: 45035, 95: 47634}[spec]),
              behavior_change=False, found_by="reused", kind=u"실오류")

# ── G12: #97 consts[9]/[10] src_line 103 → 102 ──
ev12 = (u"m10.ll:10171 `%152 = select i1 %151, i64 64000, i64 960000, !dbg !22177` · 10189 `%161 = select i1 %151, i64 960000, i64 64000` — "
        u"!22177 사슬(dloc) = game.rs:319 heal_area ← hunt_and_poke.rs:102 check_recall ← :79 sub_plan. "
        u"두 리터럴은 :102 `heal_area(t)` 호출(인라인)에서 나오고, :103 에는 891999/896000 비교(10178·10194 `!dbg` L103<79)만 있다")
p.fix("/specs[97]/consts[9]/src_line", old=103, new=102, evidence=ev12, behavior_change=False, found_by="reused", kind=u"실오류")
p.fix("/specs[97]/consts[10]/src_line", old=103, new=102, evidence=ev12, behavior_change=False, found_by="reused", kind=u"실오류")

# ── G18: #95 logic 의 `strategy.early_serpen_top` 는 콜리(is_skip_serpen) 내부 — 주석(`//`)으로 옮겨 구제 ⑤ 적용. 게이트 자체는 오탐 ──
old95 = (u"     (is_skip_serpen 요지, serpen.rs:366~414: !serpen_exists→true / position==Top && is_line_phase && line_exists(Top) 일 때 "
         u"strategy.early_serpen_top: Must→false, Giveup→true, Flexible→적 탑타워·top_lead[team]>2·top minion_count>1 로 결정 / 그 외 false)")
new95 = (u"     // is_skip_serpen 요지(★콜리 내부 — 이 함수 본문은 Strategy+0x14 early_serpen_top 을 읽지 않는다. 본문이 읽는 것은 L176 Strategy+0x13 early_serpen 뿐), serpen.rs:366~414: "
         u"!serpen_exists→true / position==Top && is_line_phase && line_exists(Top) 일 때 strategy.early_serpen_top: Must→false, Giveup→true, "
         u"Flexible→적 탑타워·top_lead[team]>2·top minion_count>1 로 결정 / 그 외 false")
p.fix("/specs[95]/logic", old=old95, new=new95,
      evidence=(u"오탐: 이 함수의 IR(m05.ll 47634~49220)에서 Strategy 읽기는 49087 `%581 = getelementptr inbounds nuw i8, ptr %9, i64 19`(+0x13 early_serpen, L176) 하나뿐. "
                u"+0x14(early_serpen_top) 읽기는 별도 define is_skip_serpen(m05.ll:44823~45032) 안 44903 `%39 = getelementptr inbounds nuw i8, ptr %5, i64 20` 이고, "
                u"본문은 그것을 47702 `call … is_skip_serpen(i64 poison, ptr %1, ptr %2, ptr %3)` 로 호출만 한다. mem 행을 넣으면 없는 로드를 주장하게 되므로 서술을 주석으로 옮긴다"),
      behavior_change=False, found_by="new", kind=u"오탐")

# ── 보강: version 이 is_skip_serpen 에는 poison 으로 전달됨(#95·#96) ──
p.fix("/specs[95]/sig/params[0]/role",
      old=u"macro_judgement_penalty 에 그대로 전달만. 본문 분기 없음",
      new=u"macro_judgement_penalty 에 그대로 전달만(49039 `call i32 …macro_judgement_penalty(i64 %0, ptr %2)`). is_skip_serpen 에는 poison 으로 전달(47702 — 피호출 define 이 %0 을 안 읽는다). 본문 분기 없음",
      evidence=u"m05.ll:47702 `%27 = tail call … is_skip_serpen(i64 poison, ptr %1, ptr %2, ptr %3)` · is_skip_serpen define(44823) 본문에 %0 사용 0건 · 49039 macro_judgement_penalty(i64 %0, …)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[96]/sig/params[1]/role",
      old=u"v25_objective_splitter_should_join_contest 에만 전달",
      new=u"v25_objective_splitter_should_join_contest 에만 전달(52002 `i64 %1`). is_skip_serpen 에는 poison(51900 — 피호출이 version 을 안 읽는다)",
      evidence=u"m05.ll:51900 `%50 = tail call … is_skip_serpen(i64 poison, ptr %2, ptr %3, ptr %4)` · 52002 `…v25_objective_splitter_should_join_contest(i64 %1, ptr %3, ptr %4, i8 5)`",
      behavior_change=False, found_by="new", kind=u"보강")

# ── 보강: #94 mem[8] 이름 규약(다른 명세와 동일 문자열 `player_champion`) — chk 오귀속 해소 ──
p.fix("/specs[94]/mem[8]/name", old=u"player_champion[team]/[1-team]", new=u"player_champion",
      evidence=u"tcxdict AbstractGameWithCache 0x1e0 = player_champion[0][0]@tag. IR m05.ll:45130 `gep %15, i64 480` 뒤 [5 x ptr] 인덱스 %18(1-team)·%17(team). 첨자 표기를 note 로 옮겨 다른 19개 명세와 같은 뼈대 이름으로 맞춘다(G20 표기 억제 규칙 적용 가능)",
      behavior_change=False, found_by="reused", kind=u"보강")
p.fix("/specs[94]/mem[8]/note", old=u"", new=u"[1-team] 5슬롯 Some 개수 = live_enemy(45145~45185) · [team] = live_ally 필터·near 카운트(45197~)",
      evidence=u"m05.ll:45131 `getelementptr [5 x ptr], ptr %21, i64 %18` · 45197 `… i64 %17`",
      behavior_change=False, found_by="reused", kind=u"보강")

# ── ev_up: 오라클 실행(v19C_o1.tsv, 케이스당 프로세스 1개) ──
p.ev("/specs[93]/mem[10]", to=3, frm=4, found_by="new",
     evidence=u"오라클 실행 확인: 게임 시작 직후 MobaMode+0x1d8 serpen.live_list.len=0 에서 check_serpen_giveup 10/10 true(L241 경로, v19C_o1.tsv case0)")
p.ev("/specs[96]/consts[2]", to=2, frm=4, found_by="new",
     evidence=u"오라클 실행 확인: phase=Hunt(3) → Option<BigPlan> tag 14(SerpenHuntAndPoke) 4/4 (v19C_o1.tsv case3)")
p.ev("/specs[96]/consts[3]", to=2, frm=4, found_by="new",
     evidence=u"오라클 실행 확인: phase None(0)·Assemble(2) → tag -1(None) 8/8, Setup(1) → 본문 진입(tag 14 = L496 경로) 4/4 (v19C_o1.tsv case3)")
p.ev("/specs[96]/consts[4]", to=2, frm=4, found_by="new",
     evidence=u"오라클 실행 확인: 반환 sret +0 i64 == 14 (Hunt·Setup 경로 8/8, v19C_o1.tsv case3)")
p.ev("/specs[96]/consts[23]", to=2, frm=4, found_by="new",
     evidence=u"오라클 실행 확인: None 반환 시 sret +0 i64 == -1 (8/8, v19C_o1.tsv case3)")
p.ev("/specs[97]/consts[2]", to=2, frm=4, found_by="new",
     evidence=u"오라클 실행 확인: 세르펜 live_list 비어 있음 → SubPlan tag 13 = SerpenCheck{move_check:false} 10/10 (v19C_o1.tsv case4, L48~50 경로)")
p.ev("/specs[96]/mem[26]", to=3, frm=3, found_by="new",
     evidence=u"오라클 실행 확인: sret +0 tag = -1(None) / 14(SerpenHuntAndPoke) 관측(v19C_o1.tsv case3) — 3(PassiveLine) 은 미관측")

# ── 내 지시(도시에)의 오류 ──
p.brief_error(u"§1 표의 #97 시그니처·경로가 `EpicHuntAndPokePlan::sub_plan`(sig.tcx `&mut EpicHuntAndPokePlan`)으로 적혀 있다 — mkspec3 `_symtoks` 가 v0 망글 백참조 `B2_`(NtB2_21SerpenHuntAndPokePlan)를 식별자로 잘못 잘라(`_2`,`S`) 타입 토큰이 사라지고, basename `hunt_and_poke.rs` 가 epic/serpen 둘 다 맞아 첫 후보(epic)가 뽑혔다. 형제표 5행이 정답(`SerpenHuntAndPokePlan::sub_plan`). 파생 필드라 patch 로 못 고친다 — mkspec3 수정 필요")
p.brief_error(u"§4 G19 `[91] knobs[0] 관측(창)=[100, 266]` 의 266 은 리터럴이 아니라 블록 라벨 `266:`(m04.ll:29132)이다 — knobval `_DROP` 이 `%266` 은 지우지만 줄머리 `NNN:` 라벨은 못 지운다. 그리고 21 은 29351 `icmp ult i64 %265, 21` 에 있는데 `!dbg` 가 없어(점프스레딩 복제 블록) 앵커가 안 잡힌다")
p.brief_error(u"§4 G18 `[95] early_serpen_top` 는 콜리(is_skip_serpen) 내부 필드다 — xreflogic P2 구제 ⑤(주석) 는 `//` 만 보고 괄호 안 요지 서술은 못 걸러 오탐")
p.brief_error(u"§1 표의 vis 열 — #92 `in:game_ai` 는 맞으나, 담당 pub 5개도 「오라클 권장」만 있고 어떤 조기반환 경로가 오라클로 닿는지(live_list 비어 있음 등) 안내가 없어 각 배치가 같은 셋업을 다시 발명한다. 이번 v19C_o1.rs 가 serpen 5함수 공용 셋업이다")
p.save()
