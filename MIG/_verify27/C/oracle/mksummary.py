# -*- coding: utf-8 -*-
"""오라클 로그를 세어 oracle_summary.json(ev_up 행 목록)을 만든다. 실행 후 mkpatch27C.py 를 다시 돌린다."""
import io, json, glob, os, re
OR = r"C:\tfm2mods\MIG\_verify27\C\oracle"
def count(prefix):
    m = d = 0; cases = {}
    for f in sorted(glob.glob(os.path.join(OR, prefix + "_case*.log"))):
        s = io.open(f, encoding="utf-8", errors="replace").read()
        for ln in s.splitlines():
            if ln.startswith("case="):
                c = int(re.match(r"case=(\d+)", ln).group(1)); cases[c] = ln
                if "\tMATCH" in ln: m += 1
                elif "\tDIFF" in ln: d += 1
    return m, d, cases
m4, d4, c4 = count("o254"); m2, d2, c2 = count("o252")
S4 = u"%d/%d MATCH · DIFF %d" % (m4, m4 + d4, d4); S2 = u"%d/%d MATCH · DIFF %d" % (m2, m2 + d2, d2)
print("254:", S4); print("252:", S2)
for c in sorted(c4): print("  254", c4[c][:110])
for c in sorted(c2): print("  252", c2[c][:110])
E4 = u"27차 배치C 오라클 o254.rs(hidden define #[link_name] 직접 호출 · 케이스당 프로세스 1개 · %s · _verify27\\C\\oracle\\o254_case*.log)" % S4
E2 = u"27차 배치C 오라클 o252.rs/o252b.rs(hidden define #[link_name] 직접 호출 · 제로버퍼 ScoreParameter · 케이스당 프로세스 1개 · %s · _verify27\\C\\oracle\\o252_case*.log)" % S2
rows = []
def r(path, why, e): rows.append([path, u"오라클 실행 확인: %s — %s" % (why, e)])
# 254
r("/specs[254]/consts[0]", u"case 15 attack_effect@tag=-1 → None [2,232]", E4)
r("/specs[254]/consts[3]", u"case 20 정글 창 → {1,0}(fights_back byte=1) · None 미니언 경로 byte1=1", E4)
r("/specs[254]/consts[4]", u"None 반환 byte0=2 전 케이스 관측", E4)
r("/specs[254]/consts[5]", u"case 10/11 bene 거리 range+1→None / range→{0,0}(ms=1 이라 ×30 이 1단위 감도로 확정)", E4)
r("/specs[254]/consts[6]", u"case 14 nexus.can_target=false→None / case 21 can_target=1·block_target_tick=0 강제→{0,0}", E4)
r("/specs[254]/consts[7]", u"case 16/17 아군 거리 60001→None / 60000→{0,0}(≤ 3600000000 경계)", E4)
r("/specs[254]/consts[10]", u"case 1 Attack{tower.id}→{0,0} · case 8 RunAway→None · case 9 Positioning→None", E4)
r("/specs[254]/knobs[1]", u"case 10/11 in_window 경계", E4)
r("/specs[254]/knobs[2]", u"case 16/17 60000² 경계", E4)
r("/specs[254]/knobs[3]", u"Attack(6)·Around(2) 포함 / RunAway(0)·Positioning(1) 제외 실측(Ult·Trace·Skill·Skill2 미실행)", E4)
r("/specs[254]/knobs[4]", u"정글 {1,0}(case 20) · 타워 {0,0}(case 1) · 넥서스 {0,0}(case 21) 각 단독 경로 실측(동시 우선순위·미니언 경로는 미실행)", E4)
# 252
r("/specs[252]/consts[0]", u"case 6 action=champ.empty(다른 Action) → 0", E2)
r("/specs[252]/consts[1]", u"case 4 target=적 챔프 → 0", E2)
r("/specs[252]/consts[2]", u"case 5 target=아군 타워(비챔피언) → 0", E2)
r("/specs[252]/consts[3]", u"case 15/16 적 거리 95000→44 / 95001→41(influence=range+35000 경계)", E2)
r("/specs[252]/consts[8]", u"case 13/14 적 거리 100000→41 / 100001→0(enemy_close 경계)", E2)
r("/specs[252]/consts[10]", u"case 11 applyed=1e6 → 56(=case 1 과 동일 · 45 캡)", E2)
r("/specs[252]/consts[11]", u"case 0/14 enemies_in_zone=0 ∧ threatened=0 → 0", E2)
r("/specs[252]/consts[12]", u"case 1 damage_reduce 30 → 기저 10", E2)
r("/specs[252]/consts[13]", u"case 10 damage_reduce 100 → 33→18 캡(64)", E2)
r("/specs[252]/consts[14]", u"case 1 합 56 = 10+5+12+14+15", E2)
r("/specs[252]/consts[16]", u"case 1 allies 2 → 12 · case 17 3 → 18(62)", E2)
r("/specs[252]/consts[18]", u"case 12 threatened 2 → 28(58)", E2)
r("/specs[252]/consts[20]", u"case 2/15 enemies_near 1 → +3", E2)
r("/specs[252]/consts[23]", u"case 1 pressure 45/3=15", E2)
r("/specs[252]/consts[25]", u"case 8 self_cover 4>1 → −36(14) · case 18 3>2 → −12(44)", E2)
r("/specs[252]/consts[26]", u"case 7 allies 1·near 0 → −15 / case 21 allies 2 → 감점 없음 / case 22 near 2 → 감점 없음", E2)
r("/specs[252]/consts[27]", u"case 7 → 30(=45−15)", E2)
r("/specs[252]/consts[29]", u"case 2/9 → 75 클램프", E2)
r("/specs[252]/knobs[0]", u"case 15/16 influence 경계", E2)
r("/specs[252]/knobs[1]", u"case 13/14 100000 경계", E2)
r("/specs[252]/knobs[5]", u"case 11 45 캡", E2)
r("/specs[252]/knobs[6]", u"같은 콜리 is_recent_visible 을 o254 case 18/19 가 last_visible=tick-120 가시 / tick-121 비가시로 실측", E4)
r("/specs[252]/knobs[7]", u"case 0/14 조기 0", E2)
r("/specs[252]/knobs[8]", u"case 10 18 캡", E2)
r("/specs[252]/knobs[9]", u"case 1 /3", E2)
r("/specs[252]/knobs[10]", u"case 1 +5", E2)
r("/specs[252]/knobs[11]", u"×6 실측(case 1·17) · 캡 4 는 case 25 참조", E2)
r("/specs[252]/knobs[12]", u"×14 실측(case 12) · 캡 4 미판별(case 9 는 75 클램프)", E2)
r("/specs[252]/knobs[16]", u"case 8/18 −12·min 3", E2)
r("/specs[252]/knobs[17]", u"case 7/21/22", E2)
r("/specs[252]/knobs[18]", u"case 2/9 75", E2)
# 조건부(추가 케이스 결과에 따라)
def ok(cases, c, want):
    return c in cases and ("got=%s\t" % want) in cases[c] and "\tMATCH" in cases[c]
if ok(c2, 23, 47): r("/specs[252]/consts[6]", u"case 23 applyed=200 → 47(hp_value 100 캡 · 캡 없으면 56)", E2); r("/specs[252]/knobs[3]", u"case 23", E2)
if ok(c2, 24, 56): r("/specs[252]/consts[9]", u"case 24 target.hp=0·applyed=5 → 56(max(1) 분모 → 500→45 캡)", E2)
if ok(c2, 25, 19): r("/specs[252]/consts[19]", u"case 25 enemies_near 5→4(19)", E2); r("/specs[252]/consts[21]", u"case 25 enemies_in_zone 4→3(19)", E2); r("/specs[252]/knobs[13]", u"case 25", E2); r("/specs[252]/knobs[14]", u"case 25", E2); r("/specs[252]/consts[24]", u"case 25 self_cover-allies=3 → min 3 → −36", E2)
if 24 in c4 and 25 in c4 and "got=None" in c4[24] and "got=Some" in c4[25] and "\tMATCH" in c4[24] and "\tMATCH" in c4[25]:
    r("/specs[254]/consts[2]", u"case 24/25 적 거리 engage(bene)+c.radius → None / +1 → {0,0}(ms=1 이라 ×120 이 1단위 감도로 확정)", E4); r("/specs[254]/knobs[0]", u"case 24/25 engage 경계", E4)
json.dump({"254": S4, "252": S2, "rows": rows, "brief": []},
          io.open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "oracle_summary.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("rows", len(rows))
