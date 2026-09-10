# -*- coding: utf-8 -*-
u"""B_audit — 담당 5건(_verify/05~09)의 reads/writes 오프셋 주장을 tcxdict 정본으로 전수 재확인."""
import json, io, os, sys, re

HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.dirname(HERE)
sys.path.insert(0, MIG)
import tcxdict as TD

try:
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
except Exception:
    pass

ALIAS = {
    "PlayerState": "game_core::PlayerState",
    "PlayerState(GamePlayer)": "game_core::PlayerState",
    "OperationData": "game_ai::OperationData",
    "SubPlan(sret)": "game_ai::plan_legacy::sub_plan::SubPlan",
    "LegacyPlanHandler": "game_ai::plan_legacy::handler::LegacyPlanHandler",
    "AbstractGameWithCache": "game_core::AbstractGameWithCache",
    "GameContext": "game_ai::GameContext",
    "GameSetting": "game_core::GameSetting",
    "MapDef": "game_core::MapDef",
    "MobaMode": "game_core::MobaMode",
    "Entity": "game_core::Entity",
    "GoalData": "game_ai::plan_legacy::goal_data::GoalData",
    "TeamPlan": "game_ai::plan_legacy::team_plan::TeamPlan",
    "EpicHuntAndBattlePlan": "game_ai::plan_legacy::old::epic::hunt_and_battle::EpicHuntAndBattlePlan",
    "Blackboard": "game_core::Blackboard",
}
SKIP = ("vtable",)

FILES = ["05_v50_fold_dive_episode.json", "06_v2_response_retreat_stance.json",
         "07_sub_plan.json", "08_is_end.json", "09_check_favorable_engage_formation.json"]


def resolve_base(base):
    if any(s in base for s in SKIP):
        return None, u"SKIP(vtable 슬롯 — 구조체 아님)"
    if base in ALIAS:
        t = ALIAS[base]
        if TD.T(t):
            return t, ""
        # def_path 로 by_path 조회
        d = TD.D()
        if t in d["by_path"]:
            ts = d["by_path"][t]
            if len(ts) == 1:
                return ts[0], ""
            return None, u"모호(%d): %s" % (len(ts), ts[:4])
    t = TD.pick(base, quiet=True)
    if t:
        return t, ""
    return None, u"사전에 없음"


def q(t, off):
    rows = TD.walk(t, want=off)
    exact = [r for r in rows if r[0] == off]
    if exact:
        return "OK", exact
    if rows:
        return "COVER", rows
    return "NONE", []


def main():
    for fn in FILES:
        p = os.path.join(HERE, fn)
        d = json.load(io.open(p, encoding="utf-8"))
        print(u"\n########## %s  (%s)" % (fn, d["name"]))
        for kind in ("reads", "writes"):
            for r in d.get(kind, []):
                base = r.get("base", "")
                offs = r.get("offset", "")
                name = r.get("name", "")
                try:
                    off = int(str(offs), 16) if str(offs).startswith("0x") else int(offs)
                except Exception:
                    print(u"  [?] %s %s %s — 오프셋 파싱 불가" % (kind, base, offs))
                    continue
                t, err = resolve_base(base)
                if t is None:
                    print(u"  [--] %-28s +%-8s %-40s %s" % (base, offs, name[:40], err))
                    continue
                st, rows = q(t, off)
                if st == "OK":
                    got = "; ".join("%s : %s" % (rr[1], rr[2].split("::")[-1]) for rr in rows[:2])
                    ok = u"OK" if _match(name, rows) else u"★불일치?"
                    print(u"  [%s] %-24s +%-8s 주장=%-34s tcx=%s" % (ok, base, offs, name[:34], got))
                elif st == "COVER":
                    rr = sorted(rows, key=lambda x: -x[0])[0]
                    print(u"  [~덮음] %-22s +%-8s 주장=%-30s tcx=%s@0x%x (%sB)"
                          % (base, offs, name[:30], rr[1], rr[0], rr[3]))
                else:
                    print(u"  [★없음] %-22s +%-8s 주장=%-30s (패딩/관통불가)" % (base, offs, name[:30]))


def _match(claim, rows):
    c = re.sub(r"[^a-z0-9_]", "", claim.lower())
    for rr in rows:
        f = re.sub(r"[^a-z0-9_]", "", rr[1].lower())
        if not f:
            continue
        # leaf 단위 비교
        leaf = rr[1].split(".")[-1].split("@")[0].lower()
        leaf = re.sub(r"\[.*?\]", "", leaf)
        if leaf and leaf in c:
            return True
        if f in c or c in f:
            return True
    return False


main()
