# -*- coding: utf-8 -*-
u"""C6 — specs[10]~[14] 의 `mem` 표 전량을 `tcxdict`(컴파일러 정본) 로 **행마다** 재조회한다.
`offset_of!` 프로브(o1/o2/o3)가 닿지 못한 행(열거형 페이로드·Vec 내부·private 필드·vtable 슬롯)을 메운다.
출력: TSV — i, idx, base, offset, name(명세), tcx가 답한 필드, 판정
"""
import io, json, os, re, subprocess, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = r"C:\tfm2mods\MIG"
V3 = json.load(io.open(os.path.join(HERE, "_spec", "specs20_v3.json"), encoding="utf-8"))

# 명세의 base 문자열 → tcxdict 가 아는 타입 이름
BASE = {
    "MainObjective(인자 %4, i24)": "game_ai::plan_legacy::team_plan::MainObjective",
    "Strategy(PlayerState::strategy 반환 24B)": "game_core::Strategy",
    "PlayerState": "game_core::PlayerState",
    "OperationData": "game_core::OperationData",
    "AbstractGameWithCache": "game_core::AbstractGameWithCache",
    "MobaMode": "game_core::MobaMode",
    "Entity": "game_core::Entity",
    "Entity(적 챔피언)": "game_core::Entity",
    "GameContext": "game_core::GameContext",
    "GameSetting": "game_core::GameSetting",
    "MapDef": "game_core::MapDef",
    "LegacyPlanHandler": "game_ai::plan_legacy::handler::LegacyPlanHandler",
    "LineGankerPlan": "game_ai::plan_legacy::old::LineGankerPlan",
    "PendingTraceEvent": "game_ai::plan_legacy::handler::PendingTraceEvent",
}
SKIP = ("vtable", "BigGoal", "BigPlan", "TraceEventType")

cache = {}


def ask(ty, off):
    key = (ty, off)
    if key in cache:
        return cache[key]
    p = subprocess.run([sys.executable, "-X", "utf8", os.path.join(HERE, "tcxdict.py"), ty, off],
                       capture_output=True, text=True, encoding="utf-8", errors="replace", cwd=HERE)
    out = p.stdout or ""
    hits = [l.strip() for l in out.splitlines() if l.strip().startswith("★")]
    cache[key] = hits
    return hits


rows = []
for i in range(10, 15):
    for j, m in enumerate(V3["specs"][i].get("mem") or []):
        base, off, name = m.get("base", ""), m.get("offset", ""), m.get("name", "")
        if any(s in base for s in SKIP):
            rows.append((i, j, base, off, name, "-", "SKIP(열거형/vtable — tcxdict 대상 아님)"))
            continue
        ty = BASE.get(base)
        if not ty:
            rows.append((i, j, base, off, name, "-", "SKIP(타입 매핑 없음)"))
            continue
        o = off.split("[")[0]
        hits = ask(ty, o)
        if not hits:
            rows.append((i, j, base, off, name, "", "NOHIT"))
            continue
        got = " | ".join(h.lstrip("★").strip() for h in hits)
        # 명세가 적은 이름의 핵심 토큰이 tcx 답에 들어 있는가
        core = re.split(r"[\s(:]", name.replace("[team]", "").replace("[position]", ""))[0]
        core = core.replace("[2]", "").replace("[len]", "")
        ok = "MATCH" if core and core.split(".")[-1].split("[")[0] in got else "CHECK"
        rows.append((i, j, base, off, name, got, ok))

print("i\tidx\tbase\toffset\tname(명세)\ttcx 응답\t판정")
for r in rows:
    print("\t".join(str(x) for x in r))
n = {}
for r in rows:
    n[r[6].split("(")[0]] = n.get(r[6].split("(")[0], 0) + 1
print("\n집계\t" + " · ".join("%s=%d" % (k, v) for k, v in sorted(n.items())))
