# -*- coding: utf-8 -*-
u"""명세에서 쓰고 있는 핵심 오프셋을 tcx layout_of 로 표적 검증(중첩 구조체까지 재귀 해석).
사용: python -X utf8 tcxverify.py
"""
import json, io, os, sys, functools, re
TCX = r"C:\tfm2mods\MIG\_tcx"
CRATES = ("game_ai", "game_core", "game_view")

@functools.lru_cache(maxsize=8)
def load(cr):
    with io.open(os.path.join(TCX, cr + ".json"), encoding="utf-8") as f:
        return json.load(f)

@functools.lru_cache(maxsize=8)
def adt_index(cr):
    idx = {}
    for it in load(cr)["items"]:
        if "adt" in it:
            idx.setdefault(it["p"], it)
    return idx

def find_adt(path):
    for cr in CRATES:
        it = adt_index(cr).get(path)
        if it: return it
    return None

def base_ty(t):
    """'game_core::EntityStat' 같은 경로만 추출(참조/제네릭 제거)."""
    t = t.strip()
    t = re.sub(r"^&('[^ ]+ )?(mut )?", "", t)
    m = re.match(r"^([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)+)$", t)
    return m.group(1) if m else None

def fields_at(it):
    """단일 variant ADT -> [(name, ty, offset)]"""
    lay = it.get("layout")
    vs = it["adt"]["variants"]
    if not lay or len(vs) != 1: return None
    off = lay.get("offsets")
    if off is None: return None
    return [(f["n"], f["t"], off[i]) for i, f in enumerate(vs[0]["fields"]) if i < len(off)]

def resolve(path, target, depth=0, prefix="", base=0):
    """target 바이트 오프셋에 해당하는 필드 경로를 찾는다."""
    if depth > 4: return []
    it = find_adt(path)
    if it is None: return []
    fl = fields_at(it)
    if fl is None: return []
    out = []
    for n, t, o in fl:
        abs_o = base + o
        if abs_o == target:
            out.append((prefix + n, t))
        bt = base_ty(t)
        if bt:
            sub = find_adt(bt)
            if sub is not None:
                lay = sub.get("layout")
                sz = (lay or {}).get("size")
                if sz and abs_o <= target < abs_o + sz:
                    out += resolve(bt, target, depth + 1, prefix + n + ".", abs_o)
    return out

TARGETS = [
 ("game_core::Entity", {
    0x68:"ty", 0x5c0:"id", 0x5c8:"level", 0x628:"hp(최대)", 0x630:"defence",
    0x638:"magic_resistance", 0x640:"move_speed", 0x660:"x", 0x668:"y", 0x670:"hp",
    0x688:"recently_attacked_by", 0x6a0:"block_target_tick", 0x6b4:"stat_version", 0x6b9:"can_target"}),
 ("game_core::AbstractGameWithCache", {
    0x280:"player_champion_cache", 0x1e0:"player_champion", 0x21c0:"*_lead", 0x21d0:"*_lead", 0x21e0:"*_lead"}),
 ("game_core::MapDef", {
    0x78:"walls", 0x1c98:"bushes", 0x38b8:"regions", 0x54d8:"region_dist",
    0x6ba0:"region_centers", 0x6d70:"fountains", 0x6db0:"is_line_region", 0x12b8:"width", 0x12c0:"height"}),
 ("game_ai::plan_legacy::handler::LegacyPlanHandler", {
    0xf8:"team_plan", 0x15b8:"ff_call_*", 0x15c0:"ff_call_*", 0x15f0:"ff_call_*",
    0x15f8:"ff_battle_exit", 0x1608:"latch", 0x1610:"mf_swap", 0x530:"pending_global_ult_target"}),
 ("game_ai::plan_legacy::old::BattlePlan", {
    0x58:"sub_goal", 0x103:"dive_abort_src", 0x107:"entry_src", 0x108:"exit_src",
    0x110:"ff_exit1_cls", 0x111:"exit_sub"}),
 ("game_ai::plan_legacy::old::FightPrediction", {
    0x0:"focus_target", 0x10:"soaker", 0x20:"rescue_ally", 0x30:"net_value", 0x38:"line", 0x39:"line_absolute"}),
 ("game_core::Effect", {
    0x10:"range", 0x18:"growth_range", 0x20:"start_timing", 0x28:"target", 0x30:"casting"}),
 ("game_core::GameSetting", {0x12f8:"tick_per_second"}),
 ("game_core::MobaMode", {0x18:"jungle_runner", 0x240:"epic_minion_buff_time"}),
]

def main():
    for path, exp in TARGETS:
        it = find_adt(path)
        print("\n=== %s" % path)
        if it is None:
            leaf = path.rsplit("::", 1)[-1]
            cands = []
            for cr in CRATES:
                cands += [p for p in adt_index(cr) if p.rsplit("::", 1)[-1] == leaf]
            print("  [타입 없음] 동명 후보:", cands[:10])
            continue
        lay = it.get("layout") or {}
        print("  size=%s align=%s  (%s:%s)  variants=%d" % (
            lay.get("size"), lay.get("align"), it["sp"]["f"], it["sp"]["l"], len(it["adt"]["variants"])))
        if not lay:
            for v in it["adt"]["variants"]:
                print("   variant %s: %s" % (v["n"], [f["n"] for f in v["fields"]][:24]))
            continue
        for o in sorted(exp):
            r = resolve(path, o)
            names = [x[0] for x in r]
            ok = "OK  " if any(exp[o].split("(")[0] == nm.split(".")[-1] or exp[o] == nm for nm in names) else ("HIT " if names else "MISS")
            print("   +0x%-5x 기대=%-26s tcx=%s   %s" % (o, exp[o], names, ok))

main()
