# -*- coding: utf-8 -*-
"""15차 배치C patch.json 생성 — mkpatch 참조구현 사용. 재실행 안전(매번 새로 만든다)."""
import sys, io
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch

p = mkpatch.Patch(round=15, batch="C")

# ★mkpatch.Patch.ev 는 `/specs[i]/mem[j]`(2단 배열) 경로에서 group(3)(=outer) 를 검사해 ValueError 를 낸다(참조구현 버그).
#   parse_path 로 idx 유무를 판정하는 판으로 대체한다(계약·출력 형식은 동일).
def _ev(self, path, evidence, to=2, frm=4, found_by="reused"):
    pp = mkpatch.parse_path(path)
    i, outer, field, idx, key = pp
    if idx is None:
        raise ValueError(u"ev 상향은 배열 원소만: %s" % path)
    if mkpatch.locate(path) is None:
        raise AssertionError(u"그 행이 없다 — %s" % path)
    if field == "mem" and int(to) < 3:
        to = 3
    self.ev_up.append({"path": path, "from": int(frm), "to": int(to),
                       "evidence": evidence, "found_by": found_by})
    return self
mkpatch.Patch.ev = _ev
O1 = u"오라클 C15_o1.tsv"
O2 = u"오라클 C15_o2.tsv"

# ═══════════════════════ #30 objective_is_damaged ═══════════════════════
# G13 실오류 — icmp ult 는 51750 에 있다
p.fix("/specs[30]/knobs[0]/where", old=u"m15.ll:51739", new=u"m15.ll:51750",
      evidence=u"m15.ll:51750 `%43 = icmp ult i64 %40, %42, !dbg !58244`(!58244 = line 0). 51739 는 #dbg_value 줄. "
               u"소스줄 45/48 은 IR dbg 가 아니라 tcx closure#1(45:20)/closure#3(48:20) span 으로 확정",
      behavior_change=False, found_by="reused")
# mem 근거의 IR 줄번호 8건 — 전부 어긋남(같은 define 이지만 #dbg_value 줄을 건너뛰고 센 듯)
for path, old, new, ev in [
    ("/specs[30]/mem[0]/note", u"m15.ll:51677", u"m15.ll:51681", u"m15.ll:51681 `%4 = load ptr, ptr %0, align 8, !dbg !58250`"),
    ("/specs[30]/mem[1]/note", u"m15.ll:51678", u"m15.ll:51682", u"m15.ll:51682 `%5 = load ptr, ptr %4, align 8` — vtable 호출 %9(ptr %5) 의 self"),
    ("/specs[30]/mem[5]/note", u"m15.ll:51706", u"m15.ll:51727", u"m15.ll:51727 `%30 = getelementptr inbounds nuw i8, ptr %24, i64 416` · 51728 load"),
    ("/specs[30]/mem[6]/note", u"m15.ll:51699~51703", u"m15.ll:51717~51721", u"m15.ll:51717 `%26 = gep … i64 424` · 51718 load · 51721 `%28 = icmp eq i64 %27, 0`"),
    ("/specs[30]/mem[7]/note", u"m15.ll:51775", u"m15.ll:51780", u"m15.ll:51780 `%54 = getelementptr inbounds nuw i8, ptr %48, i64 464`"),
    ("/specs[30]/mem[8]/note", u"m15.ll:51768~51772", u"m15.ll:51770~51774", u"m15.ll:51770 `%50 = gep … i64 472` · 51771 load · 51774 `%52 = icmp eq i64 %51, 0`"),
    ("/specs[30]/mem[9]/note", u"m15.ll:51735", u"m15.ll:51746", u"m15.ll:51746 `%39 = getelementptr inbounds nuw i8, ptr %38, i64 1648`"),
    ("/specs[30]/mem[10]/note", u"m15.ll:51737", u"m15.ll:51748", u"m15.ll:51748 `%41 = getelementptr inbounds nuw i8, ptr %38, i64 1576`"),
    ("/specs[30]/consts[0]/meaning", u"m15.ll:51674", u"m15.ll:51676", u"m15.ll:51675 `switch i8 %1, label %44 [` · 51676 `i8 4, label %3`"),
    ("/specs[30]/consts[1]/meaning", u"m15.ll:51675", u"m15.ll:51677", u"m15.ll:51677 `i8 5, label %13`"),
    ("/specs[30]/sig/params[1]/role", u"m15.ll:51673 switch", u"m15.ll:51675 switch", u"m15.ll:51675 `switch i8 %1, label %44 [`"),
]:
    p.fix(path, old=old, new=new, evidence=ev, behavior_change=False, found_by="reused")
# tcx 대조 — mem 9행(2·3 은 이미 3)
for j, ev in [(0, u"tcxdict OperationData 0x0 = cache &AbstractGameWithCache"),
              (1, u"tcxdict AbstractGameWithCache 0x0 = game &dyn AbstractGame(16B, data ptr)"),
              (4, u"tcxdict --enum GameMode: Moba 페이로드 enum+0x8 &MobaMode"),
              (5, u"tcxdict MobaMode 0x1a0 = jungle_runner.epic.live_list.buf.inner.ptr"),
              (6, u"tcxdict MobaMode 0x1a8 = jungle_runner.epic.live_list.len"),
              (7, u"tcxdict MobaMode 0x1d0 = jungle_runner.serpen.live_list.buf.inner.ptr"),
              (8, u"tcxdict MobaMode 0x1d8 = jungle_runner.serpen.live_list.len"),
              (9, u"tcxdict Entity 0x670 = hp usize"),
              (10, u"tcxdict Entity 0x628 = stat_cached.hp usize")]:
    p.ev("/specs[30]/mem[%d]" % j, evidence=u"tcx 정본 대조: " + ev, to=3, found_by="reused")

# ═══════════════════════ #31 v3_epic_formation_role ═══════════════════════
p.fix("/specs[31]/knobs[0]/where", old=u"m09.ll:64821", new=u"m09.ll:64841",
      evidence=u"m09.ll:64841 `%37 = icmp ugt i64 %34, %36, !dbg !54604`(!54604 = epic.rs:857 inlinedAt 787). 64821 은 %23 gep 줄",
      behavior_change=False, found_by="reused")
for path, old, new, ev in [
    ("/specs[31]/consts[0]/meaning", u"m09.ll:64785", u"m09.ll:64788", u"m09.ll:64788 `%7 = icmp ugt i32 %5, 4, !dbg !54585`(782)"),
    ("/specs[31]/consts[6]/meaning", u"m09.ll:64897·64920", u"m09.ll:64890 icmp · 64913/64929 panic", u"m09.ll:64890 `%64 = icmp samesign ult i64 %63, 5` · 64913 `panic_bounds_check(i64 noundef %63, i64 noundef 5` · 64929 `…%6, i64 noundef 5`"),
    ("/specs[31]/consts[7]/meaning", u"m09.ll:64898", u"m09.ll:64909", u"m09.ll:64909 `%73 = icmp ult i64 %0, 21474836480, !dbg !54660`(797)"),
    ("/specs[31]/consts[8]/meaning", u"m09.ll:64881", u"m09.ll:64888", u"m09.ll:64888 `%63 = and i64 %0, 4294967295, !dbg !54645`(581 inlinedAt 796)"),
    ("/specs[31]/mem[0]/note", u"m09.ll:64878", u"m09.ll:64894", u"m09.ll:64894 `panic_bounds_check(i64 noundef %47, i64 noundef 2`"),
    ("/specs[31]/mem[3]/note", u"m09.ll:64809", u"m09.ll:64817", u"m09.ll:64817 `%22 = load ptr, ptr %21, align 8, !dbg !54595` → 64824 `%27 = tail call { i64, ptr } %26(ptr noundef nonnull %22)`"),
    ("/specs[31]/mem[8]/note", u"m09.ll:64887~64906", u"m09.ll:64899~64921", u"m09.ll:64899 `%68 = gep … ptr %67, i64 480` · 64905 `%72 = icmp eq ptr %71, null` · 64921 `%78 = icmp eq ptr %77, null`"),
]:
    p.fix(path, old=old, new=new, evidence=ev, behavior_change=False, found_by="reused")
for j, ev in [(0, u"tcxdict PlayerState 0x930 = info.team"), (1, u"tcxdict OperationData 0x0 = cache"),
              (2, u"tcxdict OperationData 0x8 = context &GameContext"), (3, u"tcxdict AbstractGameWithCache 0x0 = game(dyn data ptr)"),
              (5, u"tcxdict --enum GameMode 태그 0 Moba"), (6, u"tcxdict MobaMode 0x1b0 = jungle_runner.epic.next_respawn_tick"),
              (7, u"tcxdict MobaMode 0x1e0 = jungle_runner.serpen.next_respawn_tick"),
              (8, u"tcxdict AbstractGameWithCache 0x1e0 = player_champion[0][0]@tag (Niche, null=None)")]:
    p.ev("/specs[31]/mem[%d]" % j, evidence=u"tcx 정본 대조: " + ev, to=3, found_by="reused")
EV31 = (u"오라클 실행 확인(15차 배치C " + O1 + u" #31 5580행 MISMATCH 0): morgard_use Gather/Split14(5)/Split131(25) × position 5 × team 2 × "
        u"far_line 3벌(epic>serpen/epic<serpen/동률) × player_champion 부재 6벌 — Split14 직접반환 (1,far)·Split131 Top/Bottom/Mid 우선순위·"
        u"폴백=Gather 결과 동일 전건 일치")
for j in [0, 1, 2, 3, 4, 5, 7, 8]:
    p.ev("/specs[31]/consts[%d]" % j, evidence=EV31, to=2, found_by="reused")
for j in [0, 1]:
    p.ev("/specs[31]/knobs[%d]" % j, evidence=EV31, to=2, found_by="reused")
p.ev("/specs[31]/sig/params[0]", evidence=u"오라클 실행 확인(" + O1 + u"): MorgardUseStrategy 3 variant 전부 구성해 진입, 니치 디코드 대로 분기", to=2, frm=3, found_by="reused")
p.ev("/specs[31]/sig/params[1]", evidence=u"오라클 실행 확인(" + O1 + u"): Position 5종 전부 진입, split_position/position1/position2 와의 동치 비교 실측", to=2, frm=3, found_by="reused")

# ═══════════════════════ #32 serpen_giveup_chat_reason ═══════════════════════
p.fix("/specs[32]/knobs[0]/where", old=u"m05.ll:53081", new=u"m05.ll:53085",
      evidence=u"m05.ll:53085 `%51 = icmp ule i64 %39, %50, !dbg !54257`(serpen.rs:227). 53081 은 `%48 = gep … i64 608` 줄",
      behavior_change=False, found_by="reused")
p.fix("/specs[32]/mem[9]/note", old=u"m05.ll:53042", new=u"m05.ll:53049",
      evidence=u"m05.ll:53049 `%33 = getelementptr inbounds nuw i8, ptr %0, i64 2352, !dbg !54238`", behavior_change=False, found_by="reused")
p.fix("/specs[32]/mem[7]/note", old=u"m05.ll:53020~53024", new=u"m05.ll:53024~53028",
      evidence=u"m05.ll:53024 `%20 = gep … ptr %18, i64 472` · 53025 load · 53028 `%23 = icmp eq i64 %21, 0`", behavior_change=False, found_by="reused")
for j, ev in [(0, u"tcxdict OperationData 0x8 = context"), (1, u"tcxdict GameContext 0x38 = tutorial@tag (Direct 1B)"),
              (2, u"tcxdict OperationData 0x0 = cache"), (3, u"tcxdict AbstractGameWithCache 0x0 = game(dyn data ptr)"),
              (5, u"tcxdict --enum GameMode 태그 0 Moba"), (6, u"tcxdict --enum GameMode Moba 페이로드 enum+0x8"),
              (7, u"tcxdict MobaMode 0x1d8 = jungle_runner.serpen.live_list.len"),
              (8, u"tcxdict MobaMode 0x260 = serpen_count[0] ([usize;2])"), (9, u"tcxdict PlayerState 0x930 = info.team")]:
    p.ev("/specs[32]/mem[%d]" % j, evidence=u"tcx 정본 대조: " + ev, to=3, found_by="reused")
EV32 = (u"오라클 실행 확인(15차 배치C " + O1 + u" #32 486행 MISMATCH 0): TutorialType 9종 × serpen.live_list len 0/1/2 × team 2 × "
        u"serpen_count (my,enemy)∈{0,1,2}² — tutorial∉{0,5,7,8} → None · len 0 → None · my<=enemy → Outnumbered · my>enemy → StackAhead 전건 일치")
for j in [0, 1, 2, 3, 4, 5]:
    p.ev("/specs[32]/consts[%d]" % j, evidence=EV32, to=2, found_by="reused")
p.ev("/specs[32]/knobs[0]", evidence=EV32 + u"(동률 my==enemy 는 Outnumbered 로 실측)", to=2, found_by="reused")
p.ev("/specs[32]/sig/params[0]", evidence=u"오라클 실행 확인(" + O1 + u"): team 0/1 선수로 진입해 serpen_count[team]/[1-team] 귀속이 실측대로", to=2, found_by="reused")

# ═══════════════════════ #33 v2_obj_restore_safe ═══════════════════════
# G1 자기모순 — consts[1] 은 506 의 `1 - team` 인데 meaning 이 515(tps*2) 를 같이 적음 → 515 언급 제거(consts[2] 가 담당)
p.fix("/specs[33]/consts[1]/meaning",
      old=u"enemy team = 1 - team (본체 %26, aux %35). 또 tps*2 가 `shl i64 %tps, 1` 로 접혀 리터럴 2 없음 (handler.rs:515)",
      new=u"enemy team = 1 - team (본체 m13.ll:11430 `%26 = sub nuw nsw i64 1, %12` !dbg 506 · aux 58259 `%35 = sub i64 1, %34` !dbg 508)",
      evidence=u"m13.ll:11430 `%26 = sub nuw nsw i64 1, %12, !dbg !21348`(!21348 = handler.rs:506) · 58259 `%35 = sub i64 1, %34, !dbg !60093`(508). "
               u"tps*2 접힘은 consts[2](515) 의 사실이라 여기서 뺀다",
      behavior_change=False, found_by="reused", kind=u"실오류")
# G16 P1 — self 소거를 검사기 어휘로. ⚠`arg\d` 가 들어가면 DROPPED 후보에서 제외되므로 'arg1' 표기를 피한다
p.fix("/specs[33]/sig/params[0]/role",
      old=u"DISubprogram arg1 은 self 이나 본문에서 전혀 안 읽혀 internal fastcc 승격 시 제거됨(IR 인자는 version/rnd/player/data/debug 5개)",
      new=u"self — 본문이 전혀 안 읽어 internal fastcc 승격 때 IR 인자에서 제거됨(DISubprogram 의 첫 인자 `self` !21306 로만 남음). IR 인자는 version/rnd/player/data/debug 5개",
      evidence=u"m13.ll:11386 define 인자 5개 (i64 %0, ptr align 16 deref(320) %1, ptr deref(2528) %2, ptr deref(24) %3, ptr deref(224) %4) · "
               u"m13.ll:89863 `!21306 = !DILocalVariable(name: \"self\", arg: 1, scope: !21301 …)` · 본문에 self 접근 0건",
      behavior_change=False, found_by="new", kind=u"보강")
# G16 P3 — i 는 소스 시그니처 1-based(sret 0). self=1 … debug=6
for j in range(6):
    p.fix("/specs[33]/sig/params[%d]/i" % j, old=j, new=j + 1,
          evidence=u"P3 규약(paramrole.py): i = sig.tcx 소스 인자 위치 1-based, sret=0. sig.tcx 인자 6개(self,version,rnd,player,data,debug) → 1..6. "
                   u"DISubprogram arg 번호와도 일치(self=1 … debug=6, m13.ll:89863~89868)",
          behavior_change=False, found_by="reused")
for path, old, new, ev in [
    ("/specs[33]/consts[0]/meaning", u"본체 m13.ll:11400, aux 58270", u"본체 m13.ll:11404 icmp·11408 panic, aux 58272",
     u"m13.ll:11404 `%13 = icmp ult i64 %12, 2` · 11408 `panic_bounds_check(i64 noundef %12, i64 noundef 2` · 58272 aux panic(len 2)"),
    ("/specs[33]/consts[2]/meaning", u"m13.ll:11514", u"m13.ll:11515", u"m13.ll:11515 `%60 = shl i64 %59, 1, !dbg !21439`(515) · 11514 는 tps load"),
    ("/specs[33]/consts[3]/meaning", u"aux 클로저: 150000^2 + 1. distance_sq(e, champ) < 이 값", u"aux 클로저: 거리 임계 150000^2 + 1. distance_sq(e, champ) < 이 값",
     u"kind=미상 → 임계. m13.ll:58251 `%29 = icmp ult i64 %28, 22500000001, !dbg !60066`(507) — 비교 피연산자 = 임계"),
    ("/specs[33]/consts[3]/meaning", u"(m13.ll:58254)", u"(m13.ll:58251)", u"m13.ll:58251 `%29 = icmp ult i64 %28, 22500000001`"),
    ("/specs[33]/knobs[0]/where", u"IR m13.ll:11514 `shl i64 %59, 1` → 11515 `icmp ugt %55, %60`", u"IR m13.ll:11515 `shl i64 %59, 1` → 11516 `icmp ugt %55, %60`",
     u"m13.ll:11515 `%60 = shl i64 %59, 1` · 11516 `%61 = icmp ugt i64 %55, %60` (둘 다 !dbg !21439 = 515)"),
    ("/specs[33]/knobs[1]/where", u"aux m13.ll:58254", u"aux m13.ll:58251", u"m13.ll:58251 `%29 = icmp ult i64 %28, 22500000001`"),
    ("/specs[33]/mem[0]/note", u"m13.ll:58236", u"m13.ll:58257", u"m13.ll:58257 `%33 = getelementptr inbounds nuw i8, ptr %32, i64 2352, !dbg !60093`"),
    ("/specs[33]/mem[4]/note", u"aux m13.ll:58255~58262, Blackboard stride = 그 gep 구조체 크기", u"aux m13.ll:58264~58268, stride 744B = tcxdict Blackboard 744B · is_recent_visible 1인자 dereferenceable(744)",
     u"m13.ll:58264 `%38 = load ptr, ptr %7`(closure+24 = blackboard) · 58265 gep [1-team] · 58268 `is_recent_visible(ptr … dereferenceable(744) %39` · tcxdict Blackboard 744B"),
    ("/specs[33]/mem[5]/note", u"m13.ll:11407~11419", u"m13.ll:11418~11423", u"m13.ll:11418 `%20 = gep … ptr %19, i64 480` · 11421 `%23 = load ptr, ptr %22` · 11422 `%24 = icmp eq ptr %23, null`"),
    ("/specs[33]/mem[8]/note", u"m13.ll:11449~11450", u"m13.ll:11469~11470", u"m13.ll:11469 `%42 = load ptr, ptr %41`(context) · 11470 `%43 = load ptr, ptr %42`(pool)"),
    ("/specs[33]/mem[10]/note", u"m13.ll:11512~11514", u"m13.ll:11513~11516", u"m13.ll:11513 `%58 = gep … ptr %57, i64 4856` · 11514 load · 11515 shl · 11516 icmp ugt"),
    ("/specs[33]/mem[11]/note", u"m13.ll:11454", u"m13.ll:11476~11478", u"m13.ll:11476 `%44 = gep … ptr %9, i64 24` · 11477 load · 11478 `%46 = icmp eq i64 %45, 0`"),
]:
    p.fix(path, old=old, new=new, evidence=ev, behavior_change=False, found_by="reused")
for j, ev in [(0, u"tcxdict PlayerState 0x930 = info.team"), (1, u"tcxdict PlayerState 0x9c0 = info.position@tag (Direct 4B)"),
              (2, u"tcxdict OperationData 0x0 = cache"), (3, u"tcxdict OperationData 0x8 = context"),
              (4, u"tcxdict OperationData 0x10 = blackboard &[Blackboard;2]"), (5, u"tcxdict AbstractGameWithCache 0x1e0 = player_champion[0][0]@tag"),
              (6, u"tcxdict AbstractGameWithCache 0x0 = game(dyn data ptr)"), (7, u"tcxdict AbstractGameWithCache 0x0 game 16B 팻포인터 후반 8B = vtable"),
              (8, u"tcxdict GameContext 0x0 = pool &Bump"), (9, u"tcxdict GameContext 0x8 = setting &GameSetting"),
              (10, u"tcxdict GameSetting 0x12f8 = tick_per_second usize"), (12, u"tcxdict Entity 0x660 = x u64"), (13, u"tcxdict Entity 0x668 = y u64")]:
    p.ev("/specs[33]/mem[%d]" % j, evidence=u"tcx 정본 대조: " + ev, to=3, found_by="reused")

# ═══════════════════════ #34 has_line_defense_threat ═══════════════════════
p.fix("/specs[34]/knobs[0]/where", old=u"m04.ll:60443", new=u"m04.ll:60449",
      evidence=u"m04.ll:60449 `%23 = icmp slt i64 %22, -3000, !dbg !69447`(defense_nexus.rs:589). 60443 은 %20 phi 근처 #dbg_value 줄",
      behavior_change=False, found_by="reused")
p.fix("/specs[34]/knobs[1]/where", old=u"m04.ll:60448", new=u"m04.ll:60455",
      evidence=u"m04.ll:60455 `%27 = icmp slt i32 %26, -2, !dbg !69447`(589)", behavior_change=False, found_by="reused")
for path, old, new, ev in [
    ("/specs[34]/mem[0]/note", u"m04.ll:60408~60420", u"m04.ll:60411~60428", u"m04.ll:60411 `%6 = gep … ptr %0, i64 2352` · 60413 `%8 = icmp ult i64 %7, 2` · 60428 panic_bounds_check(len 2)"),
    ("/specs[34]/mem[1]/note", u"m04.ll:60414", u"m04.ll:60417~60419", u"m04.ll:60417 `%10 = gep … ptr %1, i64 16` · 60418 load · 60419 `%12 = getelementptr inbounds nuw { … }, ptr %11, i64 %7`(744B stride)"),
    ("/specs[34]/mem[5]/note", u"m04.ll:60441~60443", u"m04.ll:60447~60449", u"m04.ll:60447 `%21 = gep … ptr %20, i64 16` · 60448 load i64 · 60449 icmp slt -3000"),
    ("/specs[34]/mem[6]/note", u"m04.ll:60446~60448", u"m04.ll:60453~60455", u"m04.ll:60453 `%25 = gep … ptr %20, i64 32` · 60454 load i32 · 60455 icmp slt -2"),
    ("/specs[34]/mem[9]/note", u"m04.ll:60457", u"m04.ll:60464~60465", u"m04.ll:60464 `%32 = load ptr, ptr %31`(context) · 60465 `%33 = load ptr, ptr %32`(pool)"),
    ("/specs[34]/mem[10]/note", u"m04.ll:60461", u"m04.ll:60469", u"m04.ll:60469 `%34 = load ptr, ptr %5, align 8`(sret Vec+0x0 ptr)"),
    ("/specs[34]/mem[16]/note", u"m04.ll:60515~60517", u"m04.ll:60533~60539", u"m04.ll:60533 `%59 = gep … ptr %43, i64 144` · 60538 load · 60539 `%61 = icmp eq i64 %60, %3`"),
]:
    p.fix(path, old=old, new=new, evidence=ev, behavior_change=False, found_by="reused")
for j, ev in [(0, u"tcxdict PlayerState 0x930 = info.team"), (1, u"tcxdict OperationData 0x10 = blackboard"),
              (2, u"tcxdict Blackboard 0x0 = top_minion_state BrainMinionParameter(40B)"), (3, u"tcxdict Blackboard 0x28 = mid_minion_state"),
              (4, u"tcxdict Blackboard 0x50 = bottom_minion_state"), (5, u"tcxdict BrainMinionParameter 0x10 = from_mid i64"),
              (6, u"tcxdict BrainMinionParameter 0x20 = minion_count i32"), (7, u"tcxdict OperationData 0x0 = cache"),
              (8, u"tcxdict OperationData 0x8 = context"), (9, u"tcxdict GameContext 0x0 = pool"),
              (12, u"tcxdict Entity 0x660 = x"), (13, u"tcxdict Entity 0x668 = y"),
              (15, u"tcxdict Entity 0x88 = ty@Minion.info.nearest_enemy@tag (Direct 8B)"), (16, u"tcxdict Entity 0x90 = ty@Minion.info.nearest_enemy@Some.0 usize")]:
    p.ev("/specs[34]/mem[%d]" % j, evidence=u"tcx 정본 대조: " + ev, to=3, found_by="reused")
EV34 = (u"오라클 실행 확인(15차 배치C " + O1 + u" #34 816행 + " + O2 + u" #34b 90행 MISMATCH 0): from_mid ∈{-3001,-3000,-2999} × minion_count ∈{-3,-2,-1} 경계 · "
        u"team/line 오귀속 감지용 반대 세팅 · 복제 미니언 nearest_enemy=Some(tower) 주입 → 양성 true 실측(inj 0/1), None·Tower타입 → false")
for j in [0, 1, 2, 3, 4]:
    p.ev("/specs[34]/consts[%d]" % j, evidence=EV34, to=2, found_by="reused")
for j in [0, 1]:
    p.ev("/specs[34]/knobs[%d]" % j, evidence=EV34, to=2, found_by="reused")
p.ev("/specs[34]/sig/params[2]", evidence=u"오라클 실행 확인(" + O1 + u"): line Top/Mid/Bottom 3종 진입, 다른 라인 blackboard 를 반대로 세팅해도 결과 불변(라인 귀속 실측)", to=2, frm=3, found_by="reused")
p.ev("/specs[34]/sig/params[3]", evidence=u"오라클 실행 확인(" + O2 + u"): tower_id 를 tw0/tw1 로 바꾸면 nearest_enemy 와 일치하는 쪽만 true", to=2, found_by="reused")

# ═══════════════════════ 지시(도시에) 오류 ═══════════════════════
p.brief_error(u"§4 G16 [33] 문구 「role 이 소거를 주장한 것은 0개」는 검사기 어휘(DROPPED=`poison|인자로 안 넘어옴|인자에서 제거|인자 없음`, 단 `arg\\d` 포함 시 제외) 를 안 알려줘서 "
              u"명세가 '제거됨' 이라 적고도 안 잡힌 것 — 도시에에 그 어휘 표를 실어야 한다(아니면 매 라운드 같은 자리에서 걸린다)")
p.brief_error(u"§1 표의 `ev≥4(미실행)` 수치(30:13 / 31:19 / 32:17 / 33:20 / 34:23)는 mem 행(상한 ev3, 실행으로 안 내려감)을 포함해 세고 있다 — "
              u"'오라클로 내려라' 대상은 consts/knobs/params 뿐이라 실제 표적 수와 다르다")
p.brief_error(u"§1 은 `30`(in:game_ai)·`33`(in:handler) 에 「상위 pub 래퍼나 형제 복제본을 노려라」고 하는데, 둘 다 tcx 로 보면 직접 호출자(repair_misunderstood_objective / update 계열)도 전부 in: 가시성이라 "
              u"pub 진입 경로가 없다 — 이런 함수는 도시에가 미리 '오라클 재료 부재(pub 경로 0)' 로 표시해 주면 탐색 시간을 아낀다")
p.brief_error(u"§4-b 「`callees` 의 ev4 행 중 판정에 쓰이는 술어는 IR 호출 심볼로 확정해 달라」— 담당 5함수의 vtable 간접호출(get_game_mode/get_entity_by_id) 은 divtable 로 슬롯은 풀리지만(0x40/0x1f0 = ExpectedGame impl) "
              u"명세 callees 에는 Game/SingleLaneGame impl 만 후보로 실려 있고 ExpectedGame impl 이 빠져 있다(자동 열거 누락) — patch 경로가 없어 산문으로만 보고")

p.brief_error(u"(도구) mkpatch.Patch.ev 가 `/specs[i]/mem[j]` 같은 2단 배열 경로에서 `m.group(3) is None` 검사로 ValueError 를 낸다 — "
              u"group(3) 은 outer(sig) 자리라 mem/consts/knobs 전부 막힌다. 배치C 는 parse_path 기반으로 대체해 썼다(스크래치 mkpatch_C.py). group(4) 검사로 고쳐야 한다")
p.save()
print("saved", len(p.errors), "errors", len(p.ev_up), "ev_up", len(p.brief_errors), "brief")
