# -*- coding: utf-8 -*-
u"""7차 배치 C 의 patch.json 생성 — `mkpatch.py`(참조구현)를 import 해서 쓴다(직접 JSON 을 쓰지 않는다)."""
import sys, io
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import mkpatch

p = mkpatch.Patch(round=7, batch="C")

# ═══════════════════════════════════════════════════════════════════════
# ⓪ G12 오탐 11건 — **명세가 옳다.** 도시에(17:58 판) 지시대로 `kind:"오탐"` 으로 낸다.
#    원인은 하나다: `srclinecheck.py` 가 **`switch` 케이스 값 줄에 `!dbg` 가 없어** 못 본다.
#    LLVM 텍스트 IR 에서 `switch` 는 여러 줄로 인쇄되고 `!dbg` 는 닫는 `]` 줄에만 붙는다.
#    열거형 태그 상수는 거의 전부 switch 케이스로 내려가므로 **태그 계열 consts 가 구조적으로 전부 오탐**이다.
#    (`invoke` 도 같다 — 인자 리터럴은 머리줄, `!dbg` 는 `to label ..` 줄)
#    ★독립 재현: 배치 A·D 가 상류 `srclinecheck.py` 를 이미 고쳤고(18:09 판, 47→10),
#      **현재 게이트로 배치 C 담당분은 0건**이다(`specgate.py --gate G12` 재측정). 결론이 일치한다.
FP12_EV = (u"명세가 옳다. 그 리터럴은 `switch` **케이스 값 줄**에 있고 그 줄엔 `!dbg` 가 없어 "
           u"초판 `srclinecheck.py` 가 통째로 버렸다(`!dbg` 는 닫는 `], !dbg !N` 줄에만 붙는다). "
           u"닫는 줄의 사슬을 펴면 주장한 줄이 그대로 나온다: %s. "
           u"독립 재현 %d — 상류 교정판(18:09)으로 재측정한 `specgate --gate G12` 에서도 "
           u"이 함수는 **0건**이다. 원문 = `_verify7/C/oracle/dbg_%d.out`")
for (i, j, line, chain, n) in (
        (11, 6, 429, u"`!21961`(m13.ll:12319 `switch i8 %40`) → [rule_scope.rs:91, rule_scope.rs:101, **handler.rs:429**]", 4),
        (11, 8, 429, u"`!21961` 케이스 `i8 2, label %50` → [.., **handler.rs:429**]", 4),
        (11, 9, 429, u"`!21961` 케이스 `i8 3, label %55` → [.., **handler.rs:429**]", 4),
        (11, 11, 429, u"`!21961` 케이스 `i8 5` · `!21982`/`!21983`/`!21990` 케이스 `i8 5` → [.., **handler.rs:429**]", 4),
        (12, 0, 12, u"`!36362`/`!36368`/`!36374` 케이스 `i8 0` → [runner.rs:283/287/291, rule_scope.rs:38/40/41, **chat.rs:12**]", 6),
        (12, 1, 12, u"`!36374` 케이스 `i8 1` → [runner.rs:291, rule_scope.rs:41, **chat.rs:12**]", 6),
        (12, 2, 12, u"`!36362` 케이스 `i8 2` → [runner.rs:283, rule_scope.rs:38, **chat.rs:12**]", 6),
        (12, 3, 12, u"`!36374` 케이스 `i8 3` → [runner.rs:291, rule_scope.rs:41, **chat.rs:12**]", 6),
        (12, 5, 12, u"`!36368`/`!36374` 케이스 `i8 5` → [.., **chat.rs:12**]", 6),
        (12, 7, 12, u"`!36362`/`!36368`/`!36374` 케이스 `i8 7` → [.., **chat.rs:12**]", 6),
        (14, 2, 261, u"`!55643`(m08.ll:94680 `switch i8 %37`) 케이스 `i8 2, label %64` → [**ganker.rs:261**, ganker.rs:54]", 1)):
    p.fix("/specs[%d]/consts[%d]/src_line" % (i, j), old=line, new=line,
          evidence=FP12_EV % (chain, n, i), kind=u"오탐",
          behavior_change=False, found_by="new")

# G9 오탐 2건 — RUNBOOK §S5-c 의 「손확인」 등급. 눈으로 확인했고 전부 **필드**다.
for i, f in ((13, u"m10.ll 11483~11731"), (14, u"m08.ll 94569~94941")):
    p.fix("/specs[%d]/one_line" % i, old=u"", new=u"", force=True, kind=u"오탐",
          evidence=u"G9 손확인 4개(`line`/`nearest_enemy`/`position`/`team`)는 **전부 필드가 맞다** — "
                   u"%s 에서 직접 gep 로 읽는다: `self.line`(LineGankerPlan+0x28 / LineGankCoverPlan+0x20) · "
                   u"`Tower.nearest_enemy` 태그(Entity+0x88 = gep 136) · `info.position`(PlayerState+0x9c0 = gep 2496) · "
                   u"`info.team`(PlayerState+0x930 = gep 2352). 메서드 호출(`call`)은 0건이다. "
                   u"⟹ RUNBOOK §S5-c 가 말한 대로 **눈으로 보고 넘기는 것이 정상**인 등급" % f,
          behavior_change=False, found_by="reused")

# ═══════════════════════════════════════════════════════════════════════
# ① G12 나머지 1건도 **오탐**이다 — 내 중간 결론(252→265)을 스스로 반증한 것이다.
#    소스에는 `Top => if team == 0 {2} else {16}` 이 **252 줄에 실재**하고(logic 확정),
#    IR 이 그 비교를 CSE 해 `!dbg` 를 line 0 으로 떨궜을 뿐이다.
#    「IR 에 안 남았다」를 「소스에 없다」로 읽는 것이 바로 이 게이트의 오탐 기제다.
# ═══════════════════════════════════════════════════════════════════════
p.fix("/specs[14]/consts[4]/src_line", old=252, new=252, kind=u"오탐",
      evidence=u"명세가 옳다. `team == 0` 은 소스 252 줄에 실재한다(`Top => if team==0 {2} else {16}`). "
               u"IR 에서는 252/253/254 세 arm 의 비교가 `%57 = icmp eq i64 %10, 0`(m08.ll:94663) 하나로 CSE 됐고 "
               u"그 `!dbg !55636` 이 **line 0**(ganker.rs:54 프레임)이라 강한 후보에서 252 가 사라졌을 뿐이다. "
               u"같은 줄의 `select i1 %57, i64 2, i64 16`(!55741)은 **252 로 정상 귀속**된다 — "
               u"즉 그 줄 자체는 IR 에 살아 있고 조건 피연산자만 줄을 잃었다. "
               u"★내 1차 판단(252→265)을 **철회**한다 — 265 는 CSE 를 면한 다른 사본일 뿐 「이 상수의 출처」가 아니다. "
               u"상류 교정판 `specgate --gate G12` 재측정에서도 이 행은 **0건**이다",
      behavior_change=False, found_by="new")

p.fix("/specs[14]/consts[2]/meaning",
      old=u"별개로 CancelReason::TargetMissing 의 바이트값도 2 (ganker.rs:58)",
      new=u"같은 리터럴 2 는 이 함수에서 역할이 넷이다 — ①EntityType::Tower 판별자(m08.ll:94675 "
          u"`icmp eq i64 %60, 2`; `!dbg !55641` 이 **line 0** 이라 블록 종결자인 261 switch 로 귀속된다) "
          u"②`match line` 의 **LineType::Bottom 케이스 값**(m08.ll:94680 `switch i8 %37`, `!dbg !55643`→261) "
          u"③Top 라인·타워 전멸·team==0 의 **반환 부시 ID**(`select i1 %57, i64 2, i64 16`, !55741→252) "
          u"④CancelReason::TargetMissing 바이트값(`store i8 2`, !55882→58)",
      kind=u"보강",
      evidence=u"`!dbg` 사슬 실측(`_verify7/C/oracle/dbg_14.out`): !55643→[ganker.rs:261] · "
               u"!55641→[ganker.rs:0, ganker.rs:54] · !55741→[ganker.rs:252] · !55882→[.., ganker.rs:58]. "
               u"`src_line=261` 은 그대로 옳다(①②가 둘 다 261 로 귀속) — 다만 **같은 값이 부시 ID(252)로도 쓰인다**는 "
               u"사실이 표에 없어서, 재구현할 때 `consts` 만 보면 252 줄의 2 를 놓친다",
      behavior_change=False, found_by="new")

# ═══════════════════════════════════════════════════════════════════════
# ② §4-b `mem.dir` — 새 계측기(memdir.py)가 찾은 실오류/오분류 4건
#    ⚠`dir` 키 자체는 v2 정본에 없는 행이 많아 **패치 계약으로 추가할 수 없다**(REPORT §4).
#      여기서는 근거란(note)에 참인 사실을 박는다.
# ═══════════════════════════════════════════════════════════════════════
p.fix("/specs[10]/mem[12]/note", old=u"gep +408.",
      new=u"★이 함수는 `cap` 을 **읽지 않는다** — 참조용 행이므로 `dir` 은 `\"-\"` 가 맞다. "
          u"m10.ll 49611~50100 에 `getelementptr .. i64 408` 도 그 주소의 `load` 도 **0건**이고, "
          u"408 은 `#dbg_value(.., DW_OP_plus_uconst, 408, ..)` = Vec 변수의 **주소 표현**으로만 나온다. "
          u"실제 접근은 `len(+0x1a8, m10.ll:49696)` 과 `ptr(+0x1a0, m10.ll:49708)` 둘뿐이다.",
      evidence=u"`_verify7/C/oracle/memdir_C.out` + `awk 'NR>=49611&&NR<=50100' m10.ll | grep 'getelementptr.*i64 408'` = 0건",
      behavior_change=False, found_by="new")

p.fix("/specs[10]/mem[15]/note", old=u"gep +456.",
      new=u"★이 함수는 `cap` 을 **읽지 않는다** — 참조용 행이므로 `dir` 은 `\"-\"` 가 맞다. "
          u"m10.ll 49611~50100 에 `getelementptr .. i64 456` 도 그 주소의 `load` 도 **0건**이고, "
          u"456 은 `#dbg_value(.., DW_OP_plus_uconst, 456, ..)` 로만 나온다. "
          u"실제 접근은 `len(+0x1d8, m10.ll:49723)` 과 `ptr(+0x1d0, m10.ll:49733)` 둘뿐이다.",
      evidence=u"`_verify7/C/oracle/memdir_C.out` + `grep 'getelementptr.*i64 456'` = 0건",
      behavior_change=False, found_by="new")

p.fix("/specs[13]/mem[8]/note",
      old=u"EntityType::Tower 페이로드. Top(L149)/Bottom(L179) 분기에서만 꺼냄.",
      new=u"EntityType::Tower 페이로드의 **시작 표식**이다 — 참조용 행이므로 `dir` 은 `\"-\"` 가 맞다. "
          u"m10.ll 11483~11731 에 `getelementptr .. i64 112` 는 **0건**이고, 페이로드 필드는 "
          u"Entity **절대 오프셋**으로 직접 접근된다(`+0x88` nearest_enemy = gep 136, `+0x128` TowerType = gep 296). "
          u"Top(L149)/Bottom(L179) 분기에서 쓰이는 것은 그 절대 오프셋들이다.",
      evidence=u"`_verify7/C/oracle/memdir_C.out` — m10.ll 범위 gep 목록 = {104, 136, 296}",
      behavior_change=False, found_by="new")

p.fix("/specs[14]/mem[15]/note",
      old=u"dbg_value 로만 등장(DW_OP_plus_uconst 112). 아래 두 오프셋의 기준",
      new=u"`#dbg_value(.., DW_OP_plus_uconst, 112, ..)` 로만 등장한다 — **읽지도 쓰지도 않는 기준 표식**이라 "
          u"`dir` 은 `\"-\"` 가 맞다(같은 함수의 `setup_limit`/`wait_limit` 행과 같은 성격). "
          u"아래 두 오프셋(+0x88 / +0x128)은 Entity **절대 오프셋**으로 직접 gep 된다",
      evidence=u"`_verify7/C/oracle/memdir_C.out` — m08.ll 94569~94941 에 `gep .. i64 112` 0건",
      behavior_change=False, found_by="new")

# ═══════════════════════════════════════════════════════════════════════
# ③ §4-b `consts.kind` — 새 계측기(kindchk.py) + 각 행의 meaning 자체가 근거
# ═══════════════════════════════════════════════════════════════════════
KIND = []
# specs[10] : 768 은 `icmp eq` 기대값(phase 태그 3 을 <<8 한 것)
KIND.append((10, 1, u"임계", u"태그",
             u"m10.ll:49625 `icmp eq i32 %and, 768` — 순서비교가 아니라 **동등비교 기대값**이고 "
             u"내용은 ObjectPhase::Hunt(3) 태그다. `kindchk` 관측문맥 = EQ 뿐(ORD 0건)"))
# specs[11] : TutorialType / LineType variant 태그들
for j, nm in ((4, u"LineType::Mid"), (6, u"TutorialType::None"), (7, u"TutorialType::First"),
              (8, u"TutorialType::TopSolo"), (9, u"TutorialType::Bottom"),
              (10, u"TutorialType::MidSolo"), (11, u"TutorialType::MidBottom"),
              (12, u"TutorialType::JungleOnly"), (13, u"TutorialType::Line"),
              (14, u"TutorialType::Total")):
    KIND.append((11, j, u"임계", u"태그",
                 u"%s 의 **열거형 판별자**다(tcx `TutorialType` 태그 0..8 · `LineType` 0..2). "
                 u"IR 에서 `switch i8` 케이스 값 또는 `icmp eq` 로만 쓰인다 — 순서비교 없음. "
                 u"같은 명세의 consts[1]·[2]·[3] 은 이미 `태그` 라 **한 표 안에서 분류가 갈려 있었다**" % nm))
# specs[13] : 전부 '반환 부시 인덱스'
for j in (1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15):
    KIND.append((13, j, u"임계", u"인덱스",
                 u"이 값은 임계가 아니라 **반환 부시 인덱스**다 — 이 함수의 `ret` 이 "
                 u"「usize — 부시 인덱스. LLVM range(i64 2, 22)」이고 행의 `meaning` 자체가 "
                 u"「… 반환 부시」라고 적는다. IR 관측문맥도 `select`/`phi` 결과값이지 `icmp` 순서비교가 아니다"))
# specs[14] : 전부 '목표 부시 ID'
for j in (7, 9, 10, 11, 12, 13, 14, 15, 16, 18, 19, 20):
    KIND.append((14, j, u"임계", u"인덱스",
                 u"이 값은 임계가 아니라 **목표 부시 ID(=`map.bushes` 인덱스)** 다 — 행의 `meaning` 이 "
                 u"「목표 부시 ID」라고 적고, IR 에서는 `select i1 %57, i64 A, i64 B` 의 결과값으로만 나온다. "
                 u"순서비교(`icmp ult/ugt`)가 0건이다"))
# ★★이 37행은 **패치로 낼 수 없다.** `consts[].kind` 는 v2 정본에 **저장된 필드가 아니라**
#   `mkspec3.py:325~334` 가 `meaning` 문자열의 키워드로 **유도**하는 값이고, 기본값이 `임계` 다:
#       태그   ← meaning 에 "태그"/"판별자"/"variant"
#       센티널 ← "센티널"/"니치"/"0xff"/"MAX"
#       인덱스 ← "인덱스"
#       임계   ← **그 외 전부**(= 잔여 버킷)
#   ⟹ 「임계 111행」은 분류 결과가 아니라 **키워드가 안 걸린 나머지**다. 고칠 자리는 명세가 아니라 도구다.
#   목록은 `_verify7/C/kind_proposal.json` 으로 따로 낸다(REPORT §3).
import json, os
io.open(os.path.join(r"C:\tfm2mods\MIG\_verify7\C", "kind_proposal.json"), "w", encoding="utf-8").write(
    json.dumps({"note": u"consts[].kind 는 mkspec3.py 유도값이라 patch.json 으로 못 고친다. "
                        u"도구를 고친 뒤 재생성하면 아래대로 나와야 한다.",
                "rows": [{"path": "/specs[%d]/consts[%d]/kind" % (i, j), "now": o, "should": n,
                          "why": ev} for (i, j, o, n, ev) in KIND]},
               ensure_ascii=False, indent=1))
print(u"kind_proposal.json  %d행" % len(KIND))

# ═══════════════════════════════════════════════════════════════════════
# ④ `knobs.where` — G13 1건 + G13 이 못 보는 형태 1건
# ═══════════════════════════════════════════════════════════════════════
p.fix("/specs[14]/knobs[0]/where", old=u"IR m08.ll 94643 `icmp ult i64 %32, 41`",
      new=u"IR m08.ll 94618 `icmp ult i64 %32, 41`",
      evidence=u"m08.ll:94618 = `%33 = icmp ult i64 %32, 41, !dbg !55574`. "
               u"94643 은 `%45 = getelementptr inbounds nuw i8, ptr %44, i64 384` 로 무관하다. "
               u"범위(94569~94941) 안에서 리터럴 41 이 있는 줄은 94618 **하나뿐**",
      behavior_change=False, found_by="inherited")

p.fix("/specs[13]/knobs[9]/where", old=u"_gaibc/m02.ll:9260·9265",
      new=u"★`target_bush_v30` 본문(`_gaibc/m10.ll:11483~11731`)에는 이 클램프가 **없다**(umin·32000·29 전부 0건) — "
          u"클램프는 부시 ID 를 셀 좌표로 바꾸는 **호출부** 쪽이다. 실측 확인된 site = "
          u"`_gaibc/m08.ll:94830 udiv 32000` → `94836/94843 llvm.umin.i64(.., 29)`(`!dbg`→`ganker.rs:55`, "
          u"= specs[14] `LineGankerPlan::update`). cover 쪽 대응 호출부 줄은 **미탐색**",
      evidence=u"①`awk 'NR>=11483&&NR<=11731' m10.ll | grep -cE '32000|umin|, 29'` = **0** "
               u"②구 인용 `m02.ll:9260·9265` 두 줄에는 umin/32000/29 가 없다(실측) "
               u"③근처의 진짜 클램프 m02.ll:9277~9289 는 `!dbg` 가 **trace.rs:147~148** 로 풀린다(cover.rs 아님) "
               u"— `_verify7/C/oracle/dbg_13.out`",
      behavior_change=False, found_by="new")

# ═══════════════════════════════════════════════════════════════════════
# ⑤ ev 상향 — 오라클 실행 확인 (숫자를 직접 쓰지 않고 근거를 준다)
# ═══════════════════════════════════════════════════════════════════════
p.ev("/specs[12]/knobs[5]", to=2, found_by="new",
     evidence=u"오라클 `_verify7/C/oracle/o7c.out §O7C-A` — `TutorialType` 9종 전수 호출로 "
              u"`spawn_serpen()` = **{0 None, 5 MidBottom, 7 Line, 8 Total}** 확정(9/9). "
              u"덤으로 `spawn_epic`={0,7,8} · `spawn_top_minion`={0,2,7,8} · `spawn_mid_minion`={0,4,5,7,8} · "
              u"`spawn_bottom_minion`={0,1,3,5,7,8} · `player_count`=[5,2,1,2,1,3,1,4,5] 전수 확정")

p.ev("/specs[12]/knobs[7]", to=2, found_by="new",
     evidence=u"오라클 `_verify7/C/oracle/o7c2.out` — `chat_allowed` 를 Chat 태그 47/48/49/50/55 × "
              u"라인코드 바이트 0..7 × tutorial 9종으로 전수 평가. **코드 ≥3 이면 전 튜토리얼 허용**이고 "
              u"≤2 면 그 LineType 의 `position_exists` 집합과 **완전히 일치**(Top{0,2,7,8}/Mid{0,4,5,7,8}/"
              u"Bottom{0,1,3,5,7,8}) ⟹ `code>=3 → 라인 존재검사 우회` 실행 확정. "
              u"★부수 발견: PlayCall/PlayPhaseChange/PlayPropose 는 **+0x1 의 첫 u8 에 두 번째 게이트**가 있다 "
              u"(0=Top∧Mid∧Bottom / 1=Mid∧Bottom / 2=Top / 3=Mid / 4=Bottom / ≥5=무조건 — 9 tutorial × 16 값 전수 정합)")

p.brief_error(u"★★**도시에의 신선도 계약이 자기 자신을 안 본다.** §0 은 `_spec/specs20*.json` 해시만 "
        u"확인시키는데, 도시에 본문은 `mkdossier.py` 의 산출물이고 **그 도구가 라운드 중에 바뀐다.** "
        u"실측: 내가 받아 읽은 판 = 생성 `17:41:21`, 디스크의 현재 판 = 생성 `17:58:27`, "
        u"`mkdossier.py` mtime = `18:09:28` — **`_spec` 해시는 둘 다 일치**하는데 지시문은 4곳이 달랐다"
        u"(§4 「이 배치 몫 = 15건」 명시 · §4 G12 오탐 경고 · §5 `entries`→`errors` 스키마 교정 · "
        u"§5 `consts.kind` 파생 필드 주석). 17:41 판대로 보고했으면 **patch.json 이 0건 적용**이었다. "
        u"⟹ §0 표에 **도시에 자신의 sha256 과 `mkdossier.py` 의 sha256**을 넣고, 배치가 시작할 때 그것까지 "
        u"확인하게 하라. 이번엔 내가 독립적으로 같은 4건을 재발견해 손해가 없었지만 그건 우연이다.")
p.brief_error(u"★도시에 §1 `ev≥4` 의 **분모가 안 적혀 있다** — `mkdossier.py:49 EV_FIELDS=(\"mem\",\"consts\",\"knobs\")` "
        u"라 `notes`/`open` 은 세지 않는다. 그래서 표는 specs[10] 을 `0` 으로 적지만 실제로는 "
        u"`notes[0]`·`notes[1]` 이 ev4 다. 5차의 「ev 집계 스코프 미기재 → 배치B 오적발」과 같은 형태의 재발이다.")
p.brief_error(u"★도시에 §5 규칙에 아직 **`\"op\": \"append\"`** 가 남아 있는데 `applypatch.py` 에 `op` 처리가 "
        u"**전혀 없다**(`main()` 은 `errors`/`ev_up` 만 읽고 `apply_error` 에 append 분기가 없다). "
        u"그 지시대로 새 `notes` 항목을 내면 **조용히 무시**된다 — 5차 유실과 같은 형태다.")
p.brief_error(u"★도시에 §4-b 와 §5 가 **한 파일 안에서 서로 반대**다. §4-b 는 `consts.kind` 186행을 "
        u"「아직 게이트가 없는 축」으로 표적 지정하는데, §5 는 같은 파일에서 「파생 필드라 `errors[]` 로 못 쓴다」"
        u"고 적는다. §4-b 를 **「무측정 축」**으로 고치고 `mem.dir`·`sig.params.role` 과 분리해야 한다"
        u"(그 둘은 진짜로 저장된 값이라 검사 가능하고, 실제로 이번에 결함이 나왔다).")
p.brief_error(u"★★`consts[].kind` 는 **명세에 저장된 값이 아니다** — `mkspec3.py:325~334` 가 `meaning` 의 "
              u"키워드로 유도하고 **잔여를 전부 `임계`로 떨군다**(태그←\"태그/판별자/variant\", 센티널←\"센티널/니치/0xff/MAX\", "
              u"인덱스←\"인덱스\", 그 외 = 임계). 따라서 도시에 §4-b 의 「`consts.kind` 186행 무검사」는 "
              u"**무검사가 아니라 무측정**이다 — 독립 정보가 0 이라 검사기를 붙여도 `meaning` 의 표기 습관만 잰다. "
              u"실측: 배치 C 74행 중 32행이 IR 문맥과 어긋났고(`kindchk_C.out`), 전 20함수로는 **72/186**. "
              u"고칠 자리는 명세가 아니라 도구다(제안 = `_verify7/C/kind_proposal.json` + REPORT §3).")
p.brief_error(u"★`applypatch` 의 경로 문법으로는 **행에 없는 키를 새로 만들 수 없다**. "
        u"`mem[].dir` 은 v2 `reads`/`writes` 행에 대개 키 자체가 없고 `mkspec3` 이 배열 소속으로 유도하므로, "
        u"「읽지도 쓰지도 않는 참조용 행(`dir:\"-\"`)」으로 고치려면 **v2 행에 `dir` 키를 새로 넣어야 하는데 "
        u"계약이 그 자리를 가리키지 못한다.** 6차가 적발한 「계약이 못 가리키는 자리는 영원히 안 고쳐진다」와 같은 형태다. "
        u"대상 4행 = specs[10] mem[12]·mem[15] · specs[13] mem[8] · specs[14] mem[15].")

p.save()
