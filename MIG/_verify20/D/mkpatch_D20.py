# -*- coding: utf-8 -*-
import sys; sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch
p = mkpatch.Patch(round=20, batch="D")

# ── errors ──
p.fix("/specs[109]/sig/params[6]/role",
      old="%5 — noalias/readonly/dereferenceable 3속성 전부 없음(추정: UnsafeCell 포함 타입 or 호출측 속성 소실). 본문에서 %5 경유 store 0건",
      new="%5 — noalias·readonly·dereferenceable 전부 없음. 원인 = TeamPlan 이 !Freeze(내부 가변성): tcxdict TeamPlan +0x3f8/+0x400/+0x408 `V54Counter` = `std::sync::atomic::Atomic<usize>`(UnsafeCell) → rustc 가 `&T`(T: !Freeze) 인자에 noalias·readonly·dereferenceable 을 붙이지 않는다. 같은 타입의 다른 define 도 동일: `can_near_enemies_range` %1(m09.ll:25960 `ptr noundef nonnull align 8 %1`) · `tower_dive_is_viable` %4(m10.ll:42838 `ptr noundef nonnull align 8 %4`) — 속성 소실이 아니라 타입 성질. 본문에서 %5 경유 store 0건",
      evidence="m10.ll:13739 define `ptr noundef nonnull align 8 %5` · tcxdict TeamPlan(1064B) 0x3f8 v54_fs_pairings V54Counter(8B)=Atomic<usize> · m09.ll:25960 can_near_enemies_range &self 동일 속성 · m10.ll:42838 tower_dive_is_viable %4 동일 속성. 20차 D IR 독해(open[0] 해소)",
      behavior_change=False, found_by="new")

p.fix("/specs[109]/logic",
      old="t.attack_range(t)",
      new="t.attack_effect.unwrap().range(t)",
      evidence="m10.ll:15098~15112 (root 916~917 closure$10 · inlinedAt scope `range:25`=game_core::Effect::range effect.rs:25 · `%562=gep 1216`(attack_effect@tag) `icmp eq -1` unwrap · `%566=gep 1184`(range) `%568=gep 1192`(growth_range) `%570=gep 1480`(level) `%572=gep 1080`(stat_buff_cached.range)) · 1325~1331 도 m10.ll:18349~18364 동일 사슬(closure$38 ← range:25). `Entity::attack_range` 라는 메서드는 tcx 에 없고 callees[5] 가 `SmallActionLaneMinionPosition::attack_range fn(&Entity,&Entity)->Option<u64>` 를 후보로 잡은 것은 이 산문 표기가 만든 잡음 — 실체는 헤더에 정의된 Effect::range 인라인",
      behavior_change=True, found_by="new")

# ── ev_up (오라클 실행 확증: _verify20/D/oracle/v20D_o1.rs → v20D_o1.txt, 케이스당 프로세스 1개, pub 래퍼 BattlePlan::update 경유 745 tail-call) ──
O = "오라클 v20D_o1 실행 확인(pub 래퍼 BattlePlan::update 경유, 케이스당 프로세스 1개, setting_ok=true tps=60): "
def ev(path, to, ev_txt, guard=None):
    p.ev(path, to=to, evidence=O + ev_txt, found_by="new")
    if guard: p.ev_up[-1]["guard"] = guard
# writes
ev("/specs[109]/mem[75]", 3, "case0/2/6/7/8/10/11/12 → sub_goal@tag 0x58 := 4(RunAway, 998) · case9(TeamPlan.objective=Nexus) → 7(End, 998) · case1/3 → Trace 유지", "sub_goal@tag")
ev("/specs[109]/mem[76]", 3, "case1/3 → 0x60 := 23(=focused enemy id, DIFF `0x60:0->23`)", "sub_goal.focus")
ev("/specs[109]/mem[80]", 3, "0xa8 := committed_dir 보정 후 die_tick — Trace/Kiting/Assassin/AssassinReady 프리셋(case0·12·10·11) 120=60+tps · KitingBack(case6) 0=saturating_sub · Protect/End(case7/8) 60 그대로", "prev_die_eval")
ev("/specs[109]/mem[89]", 3, "case3(near_enemies=5, 비소커) → 0xfd := 2(BacklineDPS) · case1(near_enemies=1) → 0 유지(1641 Standard)", "tactic")
ev("/specs[109]/mem[91]", 3, "초기 prev_die_eval=usize::MAX(0xff×8) → die_tick(120) < MAX → 0x102 := 1(saturating_add) 전 케이스", "die_fall_evals")
ev("/specs[109]/mem[96]", 3, "focused 없음 경로(case0/2/4~12) → 0x108 := 1 · focused 있음(case1/3) → 미기록(0 유지)", "exit_src")
ev("/specs[109]/mem[97]", 3, "case1/3 → 0x109 255→1(wave_obs) 1회 기록", "ff_wave_obs_open")
ev("/specs[109]/mem[98]", 3, "case1/3 → 0x10a 255→100 = hp(1)*100/max(maxhp(1),1)", "ff_wave_open_hp")
ev("/specs[109]/mem[99]", 3, "case1/3 → 0x10b 255→0(wave_pct)", "ff_wave_open_pct")
ev("/specs[109]/mem[102]", 3, "case1/3 → 0x10e 255→0(wave_danger)", "ff_wave_open_danger")
ev("/specs[109]/mem[104]", 3, "case0(250000 내 적 없음) → 0x110 255→0 · case2(적 20000 거리·비가시 tick=1000) → 1(invis) — 코드표 0/1 실측", "ff_exit1_cls")
# reads
ev("/specs[109]/mem[3]", 3, "sub_goal@tag 프리셋 7종(0/1/2/3/5/6/7)에 따라 755~766 committed_dir 이 갈렸다(위 mem[80] 값)", "sub_goal@tag")
ev("/specs[109]/mem[8]", 3, "초기 MAX 와 die_tick 비교(889) → die_fall_evals 1", "prev_die_eval")
ev("/specs[109]/mem[17]", 3, "main_objective None(255)→RunAway / Nexus(4)→End (996~998)", "main_objective@tag")
ev("/specs[109]/mem[21]", 3, "255 이면 1회 기록(case1/3 에서 1595 발화)", "ff_wave_obs_open")
ev("/specs[109]/mem[23]", 3, "255 이면 1회 기록(case0/2 에서 970 발화)", "ff_exit1_cls")
ev("/specs[109]/mem[37]", 3, "tps=60 이 die_tick 보정(±60)·die_tick=120 에 그대로 드러남", "tick_per_second")
# consts (동작 확정 → ev2)
p.ev("/specs[109]/consts[33]", to=2, evidence=O+"RunAway 태그 4 가 0x58 에 실제 기록됨(case0 등) · 프리셋 4 는 래퍼가 End(7)로 바꿔 750 조기 return 은 판정불가", found_by="new"); p.ev_up[-1]["guard"]="RunAway"
p.ev("/specs[109]/consts[34]", to=2, evidence=O+"case9 main_objective=Nexus → 0x58 := 7(End) 실기록", found_by="new"); p.ev_up[-1]["guard"]="End 태그"

for _t in [
 "§4 「이 배치 몫 미해소 = 0건」이라 적혀 있으나 `specgate.py --only 109` 는 G16 2건(p[6] team_plan `readonly`·`noalias`)을 낸다 — 도시에가 게이트 결과를 반영하지 않았거나 G16 을 집계에서 뺐다. (원인은 gate 쪽: `paramrole.NEGATED` 의 sep `[^가-힣A-Za-z]{0,8}` 가 「3속성 전부 없음」의 `3속성` 을 못 건너 부정 서술을 주장으로 읽음 → 이번 patch 로 문면을 `전부 없음` 형태로 바꿔 우회, 게이트 수정은 메인 몫)",
 "§4 지시 「G12 src_line 규약 = 루트 줄」은 `srclinecheck._ownlines` 구현(=inlinedAt 사슬 중 src 파일에 속한 **모든** 줄 허용)과 다르다. 같은 파일 클로저에 인라인된 상수는 루트(호출 줄 916)와 클로저 줄(917)이 둘 다 통과하며, 명세는 클로저 줄(917/1330/1291 등)을 쓰고 있다. 「루트만」으로 읽으면 이 함수 consts 6건(idx 5·6·7·15·31·36 계열)이 오탐된다 — 규약 문구를 「src 파일에 속한 사슬 줄(클로저면 클로저 줄 우선)」로 고칠 것",
 "§1 「pub 함수 = 오라클 권장」에 이 함수(in:battle)는 빠져 있지만, 형제 `BattlePlan::update`(pub, battle.rs:427)가 745 에서 update_v32 를 tail-call 하므로 래퍼 경유 오라클이 된다(이번에 13 케이스 실행). 도시에 §1 의 「상위 pub 래퍼를 노려라」가 siblings 표에서 어느 행인지 지목해 주면 다음 배치가 바로 쓴다",
 "§5 ev_up 은 `callees[]` 행에 못 쓴다(v2 에 배열이 없어 `인덱스 범위 밖`). 인라인 콜리(is_ignored_battle_enemy 등)는 IR 호출 심볼이 없어 영원히 ev4 인데, `!DISubprogram linkageName`(inlinedAt 사슬)로 망글링 심볼이 확정된다 — 산문 보고 §3 참조. mkspec3._rank_callees 앵커에 DISubprogram linkageName 을 추가하면 이 축이 닫힌다",
]: p.brief_error(_t)
p.save()
print("saved")
