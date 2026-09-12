# -*- coding: utf-8 -*-
u"""9차 배치B `patch.json` 생성 — `old` 문면을 **정본에서 직접 떠서** 오타를 원천 차단한다."""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))

MARK = u"\u26a0**\uc774 params \ud45c\ub294 sret out-ptr \uc744 \ube60\ub728\ub838\ub2e4**"   # ⚠**이 params 표는 sret out-ptr 을 빠뜨렸다**

errors = []
for i, nargs, nrows, cite in ((2, 8, 8, u"m12.ll:34867"), (17, 5, 5, u"m05.ll:17695")):
    sp = D["specs"][i]
    role = sp["sig"]["params"][1]["role"]
    k = role.index(MARK)
    old = role[k:]
    q = re.search(r"`([^`]*)`", old).group(1)          # define 헤더 인용(그대로 보존)
    new = (u"sret out-ptr \uc740 %s `%s` \uc758 `%%0` \uc774\ub2e4. "
           u"IR \uc778\uc790 %d\uac1c \u2194 params %d\ud589\uc73c\ub85c "
           u"**\uc790\ub9ac \ubc88\ud638\uac00 1:1 \ub85c \uc77c\uce58**\ud55c\ub2e4 \u2014 "
           u"9\ucc28 \uc2dc\uc810\uc5d0 `(sret)`(i=0) \ud589\uc774 params[0] \ub85c \uc2e4\ub824 \uc788\ub2e4"
           u"(8\ucc28 \ubc30\uce58C \ud655\uc815 \uaddc\uc57d \u2192 9\ucc28 `applypatch op:insert` \ub85c \ubc18\uc601). "
           u"~~\uc774 params \ud45c\ub294 sret \uc744 \ube60\ub728\ub838\ub2e4 \u00b7 \ud45c\uc758 \uc790\ub9ac \ubc88\ud638\uac00 "
           u"IR \ubcf4\ub2e4 \ud55c \uce78 \uc55e\uc120\ub2e4~~ \ub294 8\ucc28 \uae30\uc900 \uc11c\uc220\uc774\ub77c 9\ucc28\uc5d0 \uc815\uc815"
           % (cite, q, nargs, nrows))
    errors.append({
        "path": "/specs[%d]/sig/params[1]/role" % i,
        "kind": u"\uc2e4\uc624\ub958",
        "old": old,
        "new": new,
        "evidence": (u"%s `%s` \u2014 IR \uc778\uc790 %d\uac1c. "
                     u"\ud604 \uc815\ubcf8 `sig.params` \ub294 %d\ud589\uc774\uace0 params[0].name=`(sret)`\u00b7i=0 "
                     u"\uc774\ub77c **\ube60\uc9c4 \uac83\uc774 \uc5c6\ub2e4**(9\ucc28 \ub3c4\uc2dc\uc5d0 \u00a7\uc0c1\ud669: "
                     u"\uc0bd\uc785 \ud6c4 P2 2\uac74\uc774 \uc608\uace0\ub300\ub85c \uc0ac\ub77c\uc84c\ub2e4). "
                     u"`sig.tcx` \uc18c\uc2a4 \uc778\uc790 %d\uac1c + sret 1 = %d\ud589 \uac80\uc0b0\ub3c4 \ud1b5\uacfc"
                     % (cite, q[:60], nargs, nrows, nargs - 1, nrows)),
        "behavior_change": True,
        "found_by": "new",
    })

ev_up = [
    {"path": "/specs[2]/sig/params[0]", "from": 4, "to": 3,
     "evidence": (u"tcx \uc815\ubcf8 \ub300\uc870: `fn(&AttackNexusPlan, usize, &mut StdRng, &PlayerState, "
                  u"&OperationData, &TeamPlan, &mut DebugFrameData) -> SubPlan` \u2014 \uc18c\uc2a4 \uc778\uc790 7\uac1c. "
                  u"m12.ll:34867 `define void @\u2026AttackNexusPlan8sub_plan(ptr dead_on_unwind noalias noundef "
                  u"writable writeonly sret([72 x i8]) align 8 captures(none) dereferenceable(72) %0, \u2026)` "
                  u"\u2014 IR \uc778\uc790 8\uac1c = 7 + sret 1. \ubc18\ud658 SubPlan \uc774 72B \ub85c "
                  u"`sret([72 x i8])` \uc640 \uc77c\uce58"),
     "found_by": "new"},
    {"path": "/specs[17]/sig/params[0]", "from": 4, "to": 3,
     "evidence": (u"tcx \uc815\ubcf8 \ub300\uc870: `fn(usize, BattlePlanGoal, &OperationData, &PlayerState) "
                  u"-> DeathMatchBattle` \u2014 \uc18c\uc2a4 \uc778\uc790 4\uac1c. m05.ll:17695 "
                  u"`define void @\u2026DeathMatchBattle3new(ptr dead_on_unwind noalias noundef writable "
                  u"writeonly sret([384 x i8]) align 8 captures(none) dereferenceable(384) %0, i64 noundef %1, \u2026)` "
                  u"\u2014 IR \uc778\uc790 5\uac1c = 4 + sret 1. \ubc18\ud658 DeathMatchBattle \uc774 384B"),
     "found_by": "new"},
]

brief = [
    u"\u00a7\uc0c1\ud669\uc774 \u300cP4 6\uac74\u00b7P6 14\uac74\uc744 \ubc18\uc99d\ud558\ub77c\u300d\uace0\ub9cc \uc801\uc5c8\uace0 "
    u"\u300c\uadf8 20\uac74\uc774 \uc804\ubd80 \uac19\uc740 3\ud589(`02` p[1] \u00b7 `03` p[4] \u00b7 `17` p[1])\uc5d0\uc11c "
    u"\ub098\uc628\ub2e4\u300d\ub294 \uc0ac\uc2e4\uc740 \ube60\uc84c\ub2e4. 126\ud589 \uc911 3\ud589\uc774\ub77c "
    u"\u300c20\uac74\u300d\uc774\ub77c\ub294 \uc218\uac00 \uac81\uc900 \uac83\ubcf4\ub2e4 \ud6e8\uc52c \uc9c1\uc744 \uc218 \uc788\uc5c8\ub2e4.",
    u"\ub3c4\uc2dc\uc5d0 \u00a71 \uc740 \u300c`_gates/params_role/` \uc758 \ud6c4\ubcf4\ub97c \uc804\ubd80 \ub3cc\ub824 "
    u"\ud1b5\ud569\ud558\ub77c\u300d\uace0 \ud558\ub294\ub370, 8\ucc28 \ubc30\uce58C \uac00 \uc774\ubbf8 \ud1b5\ud569\ud574 "
    u"`MIG\\paramrole.py` \ub85c \ub0c8\ub2e4. \ub3c4\uc2dc\uc5d0\uac00 8\ucc28\uc6a9 \ubcf8\ubb38\uc744 \uadf8\ub300\ub85c "
    u"\uc4f0\uace0 \uc788\uc5b4 9\ucc28 \uc9c0\uc2dc(\ubd80\ubaa8 \uba54\uc2dc\uc9c0)\uc640 \u00a71 \uc774 \uc5b4\uae7d\ub09c\ub2e4.",
    u"\u00a75 \ud45c\uac00 `errors[].kind` \uc5d0 `\uc624\ud0d0`\uc744 \ud5c8\uc6a9\ud558\uc9c0\ub9cc "
    u"`apply_error` \ub294 `path`/`old`/`new` \uac00 \uc788\uc5b4\uc57c \ub3cc\uc544\uac04\ub2e4. "
    u"\u300c\uac8c\uc774\ud2b8\uac00 \ud2c0\ub838\ub2e4\u300d\ub294 \uba85\uc138\uc5d0 \uace0\uce60 \ubb38\uba74\uc774 \uc5c6\uc5b4 "
    u"`errors[]` \ub85c \ubabb \ub0b8\ub2e4 \u2014 \uc774\ubc88 \uc624\ud0d0 20\uac74\uc740 REPORT \u00a72 \uc640 "
    u"`brief_errors` \ub85c\ub9cc \ub0a8\ub294\ub2e4.",
    u"\u00a75 \uac00 `ev_up[].from`\uc744 \uc694\uad6c\ud558\uc9c0\ub9cc `apply_evup` \ub294 `to` \ub9cc \uc77d\ub294\ub2e4 "
    u"(`from` \uc740 \uac80\uc99d\uc5d0\ub3c4 \uc548 \uc4f0\uc778\ub2e4) \u2014 \uc624\uae30\uac00 \uc870\uc6a9\ud788 \ud1b5\uacfc\ud55c\ub2e4.",
]

out = {"round": 9, "batch": "B", "errors": errors, "ev_up": ev_up, "brief_errors": brief}
p = os.path.join(MIG, "_verify9", "B", "patch.json")
io.open(p, "w", encoding="utf-8").write(json.dumps(out, ensure_ascii=False, indent=1))
print(u"wrote %s  errors=%d ev_up=%d brief=%d" % (p, len(errors), len(ev_up), len(brief)))
for e in errors:
    print(u"\n--- %s\nOLD: %s\nNEW: %s" % (e["path"], e["old"][:120], e["new"][:160]))
