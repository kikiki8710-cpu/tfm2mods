# -*- coding: utf-8 -*-
u"""배치 B 6차 — `patch.json` 생성기.

★손으로 행 목록을 옮겨 적지 않는다. 5차에 ev 상향 46행을 산문 표로만 보고했다가
  **한 행도 반영되지 않았다**(이번 라운드 실측 — §5 참조). 그래서 목록은 기계가 만든다.

ev 규칙(BRIEF §1):
  · `mem`(오프셋·필드명)  → 상한 **ev3**. 근거 = tcx 정본 + `offset_of!` 교차검증.
  · `consts`/`knobs`(임계·동작) → 실행 경계를 격리했으면 **ev2**.
"""
import io, json, os, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.dirname(os.path.dirname(HERE))
V3 = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))

# ── ⓐ offset_of! 로 직접 실행 대조한 (spec, row) ──────────────────────
OF = set()
for ln in io.open(os.path.join(HERE, "B6_o1.tsv"), encoding="utf-8"):
    p = ln.rstrip("\n").split("\t")
    if len(p) >= 8 and p[0] == "OFF" and p[2].startswith("mem[") and p[7] == "OK":
        OF.add((int(p[1]), int(p[2][4:-1])))

EV_TCX = (u"tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정")
EV_OF = (u"tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 "
         u"MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정")

ev_up = []
for i in range(5, 10):
    sp = V3["specs"][i]
    for j, m in enumerate(sp.get("mem") or []):
        if int(m.get("ev") or 4) < 4:
            continue
        if m.get("chk", "").startswith(u"확인불가"):
            continue          # vtable 슬롯 — divtable 소관이라 tcx 가 답하지 않는다
        ev_up.append({"path": "/specs[%d]/mem[%d]" % (i, j), "from": int(m.get("ev") or 4), "to": 3,
                      "evidence": EV_OF if (i, j) in OF else EV_TCX})

# ── consts / knobs — 실행으로 경계를 가른 것만. 못 가른 것은 **올리지 않는다** ──
O2 = u"(B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)"
R1 = u"(B6_r1.tsv/B6_r2.tsv — 5차 프로브 재실행, 출력 바이트 동일)"
R3 = u"(B6_r3.tsv — 5차 프로브 재실행, 출력 바이트 동일)"
R4 = u"(B6_r4.tsv — 5차 프로브 재실행, 출력 바이트 동일)"

CK = [
    # ── 05 ── 이번 라운드 신규 실행
    ("/specs[5]/consts[0]", 2, u"오라클 실행 확증: `LegacyPlanHandler+0x570` 초기값 −1 = None, "
                               u"fold 후 −1 로 되돌아감(take 무조건 실행) " + O2),
    ("/specs[5]/consts[2]", 2, u"오라클 실행 확증: end_plan=1 (plan 이름 \"PassiveLine\") #A·#B·#G 26케이스 " + O2),
    ("/specs[5]/consts[3]", 2, u"오라클 실행 확증: end_plan=2 (\"PassiveJungle\") #G 4케이스 " + O2),
    ("/specs[5]/consts[8]", 2, u"오라클 실행 확증: end_plan=7 (\"Battle …\") #B2 tag9 " + O2),
    ("/specs[5]/consts[9]", 2, u"오라클 실행 확증: end_plan=8 (\"AttackNexus\"/\"DefenseNexus\") #B2 tag16·17 " + O2),
    ("/specs[5]/consts[10]", 2, u"오라클 실행 확증: end_plan=9 (\"DeathMatchBattle …\"/\"SinglePlanBattle …\") #B tag0·5 " + O2),
    ("/specs[5]/consts[11]", 2, u"오라클 실행 확증: \"PassiveLine\"(11자) 접두 일치 경로가 실제로 1 을 낸다 " + O2),
    ("/specs[5]/consts[12]", 2, u"오라클 실행 확증: \"PassiveJungle\"(13자) 접두 일치 경로가 실제로 2 를 낸다 #G " + O2),
    ("/specs[5]/knobs[1]", 2, u"오라클 실행 확증: #C 경계 — in_range_ticks 0 → last_dive_abandon_tick 갱신 / 1 → 갱신 없음 " + O2),
    ("/specs[5]/knobs[2]", 2, u"오라클 실행 확증: #C 경계 — team_holder_ticks 0 → 갱신 / 1 → 갱신 없음 " + O2),
    ("/specs[5]/knobs[3]", 2, u"오라클 실행 확증: end_plan 분류 사슬 #B 16/16 + #G 20/20 MATCH(독립 재구현 대조) " + O2),
    ("/specs[5]/knobs[4]", 2, u"오라클 실행 확증(범위 = aborted=false 가지만): 주입 abort_src 0/0x5A/0xFF 3종 전부 "
                              u"레코드 abort_src=0 으로 지워짐 #F. aborted=true 가지는 미도달 " + O2),
    ("/specs[5]/knobs[5]", 2, u"오라클 실행 확증: #C 4조합 — 두 게이트가 AND 로 걸린다 " + O2),
    ("/specs[5]/knobs[6]", 2, u"오라클 실행 확증: `update` 가 plan 을 바꾸자 end_plan 이 **바뀐 이름**을 따라갔다(#B 16/16) "
                              u"⟹ 분류가 문자열 의존임이 실행으로 확인 " + O2),
    # ── 06 ── tcx 등급(열거형 태그)
    ("/specs[6]/consts[2]", 3, u"tcxdict --enum BattleSubPlanGoal **tcx 정본**: KitingBack 메모리태그 3, 페이로드 focus@+0x8"),
    ("/specs[6]/consts[3]", 3, u"tcxdict --enum BattleSubPlanGoal **tcx 정본**: RunAway 메모리태그 4, fieldless"),
    # ── 07 ── 5차 상향분 복구(재실행으로 근거 재확립)
    ("/specs[7]/consts[0]", 2, u"오라클 실행 확증: hp_ratio 정수나눗셈 509/1000 → Recall · 510/1000 → EpicHunt " + R4),
    ("/specs[7]/consts[5]", 2, u"오라클 실행 확증: Hide 페이로드 out_line=1(Outline) bush 0·7·26 전부 " + R4),
    ("/specs[7]/consts[6]", 2, u"오라클 실행 확증: check_move=0 · enemy_spotted_me=0 · need_recall=0 · live_list.get(0) " + R4),
    ("/specs[7]/knobs[0]", 2, u"오라클 실행 확증: hp 50 → Recall / 51 → EpicHunt (챔프를 분수 밖으로 옮긴 뒤) " + R4),
    ("/specs[7]/knobs[1]", 2, u"오라클 실행 확증: Hide.out_line=1, bush 0·7·26 전부 " + R4),
    ("/specs[7]/knobs[2]", 2, u"오라클 실행 확증: EpicHunt 페이로드 need_recall=0 " + R4),
    ("/specs[7]/knobs[3]", 2, u"오라클 실행 확증: tps60 k=1059→Hide/1060→EpicHunt · tps30 29/30 · tps1 0/1 ⟹ 계수 1 · `>` 엄격 " + R4),
    ("/specs[7]/knobs[4]", 2, u"오라클 실행 확증: hp 50/51 경계 " + R4),
    ("/specs[7]/knobs[5]", 2, u"오라클 실행 확증: tps 3종에서 마진 = tick_per_second × 1 " + R4),
    ("/specs[7]/knobs[6]", 2, u"오라클 실행 확증: 에픽 hp=max → Recall / hp=max−1 → EpicHunt " + R4),
    ("/specs[7]/knobs[7]", 2, u"오라클 실행 확증: Hide.out_line 이 bush 3종 전부 1 " + R4),
    # ── 08 ── 5차 상향분 복구
    ("/specs[8]/consts[3]", 2, u"오라클 실행 확증: 캠프 288000 → 셀 (9,9), is_visible_cell(0,9,9)=true 로 L179 진입 " + R3),
    ("/specs[8]/consts[4]", 2, u"오라클 실행 확증: 캠프 거리 150000 → 폴스루 / 150001 → (d) 종료 " + R3),
    ("/specs[8]/knobs[2]", 2, u"오라클 실행 확증: phase None/Assemble/Hunt → v24=false·is_end=false, Setup → true·true (#P 진리표) " + R3),
    ("/specs[8]/knobs[6]", 2, u"오라클 실행 확증: v23 반경 180000 → v24=false((d)) / 180001 → v24=true((b)) " + R3),
    # ── 09 ── 5차 상향분 복구
    ("/specs[9]/consts[1]", 2, u"오라클 실행 확증: d = er+100000 → 포함 / +1 → 제외, er 3종(100000·200000·400000) 전부 " + R1),
    ("/specs[9]/consts[2]", 2, u"오라클 실행 확증: HP% 39 → false / 40 → true " + R1),
    ("/specs[9]/consts[3]", 2, u"오라클 실행 확증: 백분율 스케일 hp*100/max 정수나눗셈 399/1000·400/1000 " + R1),
    ("/specs[9]/consts[4]", 2, u"오라클 실행 확증: dy 158989 → rear / 158990 → flank (= 91dx² > 9dy²) " + R1),
    ("/specs[9]/consts[5]", 2, u"오라클 실행 확증: dy 173205 → front / 173206 → flank (= 3dx² > dy²) " + R1),
    ("/specs[9]/consts[6]", 2, u"오라클 실행 확증: champ_to_base 547722 → true / 547723 → false " + R1),
    ("/specs[9]/consts[7]", 2, u"오라클 실행 확증: 같은 경계(×5 ≤ ×6) " + R1),
    ("/specs[9]/consts[8]", 2, u"오라클 실행 확증: 적이 분수 중심 위 → 무조건 true / +1 → false · 겹친 아군 = front " + R1),
    ("/specs[9]/knobs[0]", 2, u"오라클 실행 확증: HP% 39/40 경계 + 민감도표 " + R1),
    ("/specs[9]/knobs[1]", 2, u"오라클 실행 확증: 여유 100000 경계 3종 " + R1),
    ("/specs[9]/knobs[2]", 2, u"오라클 실행 확증: front 각도 임계 4 의 경계 173205/173206 " + R1),
    ("/specs[9]/knobs[3]", 2, u"오라클 실행 확증: 1283 경계 158989/158990 + 1286 은 0~74 전 구간 무효(민감도 델타 0) " + R1),
    ("/specs[9]/knobs[4]", 2, u"오라클 실행 확증: 최종 비율 게이트 547722/547723 " + R1),
    ("/specs[9]/knobs[6]", 2, u"오라클 실행 확증: rear1→true / front1→false / front2+비율→true / flank1→false / "
                              u"flank2→true / flank1+front1→true 6종 " + R1),
]
for path, to, ev in CK:
    i = int(path.split("[")[1].split("]")[0])
    fld = path.split("/")[2].split("[")[0]
    j = int(path.split(fld + "[")[1].split("]")[0])
    cur = int((V3["specs"][i][fld][j].get("ev")) or 4)
    if cur <= to:
        print(u"skip(이미 ev%d) %s" % (cur, path)); continue
    ev_up.append({"path": path, "from": cur, "to": to, "evidence": ev})

ERRORS = json.load(io.open(os.path.join(HERE, "_errors.json"), encoding="utf-8"))
BRIEF = json.load(io.open(os.path.join(HERE, "_brief_errors.json"), encoding="utf-8"))

out = {"round": 6, "batch": "B", "errors": ERRORS, "ev_up": ev_up, "brief_errors": BRIEF}
io.open(os.path.join(HERE, "patch.json"), "w", encoding="utf-8").write(
    json.dumps(out, ensure_ascii=False, indent=1))
print(u"errors=%d · ev_up=%d (mem→3 %d · consts/knobs→2·3 %d)"
      % (len(ERRORS), len(ev_up),
         sum(1 for x in ev_up if "/mem[" in x["path"]),
         sum(1 for x in ev_up if "/mem[" not in x["path"])))
