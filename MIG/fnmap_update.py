#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""fnmap_update.py — `AI함수지도` 기반 데이터(fndata 641함수)를 현행 사실로 갱신 + 관계 diff 보고 (2026-09-13)

왜: 지도(09-10 생성)는 명세 `exe.addr` 을 지문(fp)으로 붙였고, 그중 6건이 **다른 함수**였다(09-12 ghidra 확정).
    지도의 그 6 항목은 이름이 틀렸고, 진짜 주소는 지도에 없거나 다른 이름으로 있다. 또 ev1(런타임 DIFF=0) 결과가
    지도에 없다. 이 도구는 ①이름 정정 ②누락 함수 추가 ③호출관계를 exe 콜그래프(cg_exe.json)로 재계산해 diff
    ④ev1 상태를 노트로 박은 뒤, 새 기반 HTML 을 만든다 → `mkmap_html.py` 가 거기에 명세 패널을 붙인다.

입력: `_spec\\fnmap_base_2026-09-10.html`(원본 아티팩트 · 불변) · `cg_exe.json`(exe 콜그래프 · 10진 RVA) ·
      `aimap.json`(크기/줄) · `_spec\\specs20.json`(exe.addr) · `REPORT\\tfm2_judge_verify\\00_상태원장.md`(ev1)
출력: `_spec\\fnmap_base_current.html` + `REPORT\\tfm2_judge_verify\\RE\\<날짜>_함수지도_갱신_관계diff.md`
사용: python -X utf8 MIG\\fnmap_update.py   →  이어서 python -X utf8 MIG\\mkmap_html.py

fndata 필드(지도 JS 실측): a=RVA(hex) n=이름 m=모듈 b=bytes i=ins l=패닉줄 v=근거(fp|re|doc) f=플래그[] x=노트
                           ai=AI계층 **c=호출자** **o=콜리**(방향 주의 — JS 라벨 기준)
"""
import io, json, os, re, sys, time

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
BASE = os.path.join(HERE, "_spec", "fnmap_base_2026-09-10.html")
OUT = os.path.join(HERE, "_spec", "fnmap_base_current.html")
CG = os.path.join(HERE, "cg_exe.json")
AIMAP = os.path.join(HERE, "aimap.json")
V2 = os.path.join(HERE, "_spec", "specs20.json")
REP = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_judge_verify"
LEDGER = os.path.join(REP, u"00_상태원장.md")
DIFF_MD = os.path.join(REP, "RE", time.strftime("%Y-%m-%d") + u"_함수지도_갱신_관계diff.md")

# ★정정표 — 근거 = RE\2026-09-12_04번16번_RVA오염_확정_ghidra.md · RE\2026-09-12_의심5건_진짜RVA_확정_ghidra.md ·
#   RE\2026-09-12_02번_BigPlan_sub_plan_확정_ghidra.md · 원장 §1.  (idx, module, name, old_addr, old_true_name, new_addr)
FIX = [
    (4,  "defense_nexus", "handle_line_defense",                         "d3e4b0", u"has_line_defense_threat", "d3cfa0"),
    (16, "battle",        "max_range_nearly_can_use",                    "c809d0", u"LocalKey::with (max_range_cached TLS 메모 본체)", "e0daa0"),
    (10, "fight_model",   "should_end_object_finish_kill_priority_battle","e12050", u"resolve_join_stake::{{closure}}#0", "e0c560"),
    (11, "handler",       "v3_fall_back_to_passive",                     "e6fe60", u"v2_obj_restore_safe::{{closure}}#0", "e4b5d0"),
    (12, "chat",          "handle_chat",                                 "e70330", u"handle_chat_inner::filter().count() 인스턴스", "e595b0"),
    (15, "modes",         "single_try_engage",                           "ca4a80", u"Vec<&Entity>::retain 모노모프(single_handle_solokill)", "e5c1f0"),
]
# 지도에 없던 함수 추가 (idx, module, name, addr) — RVA 근거 = 원장 §1
ADD = [
    (3,  "buff_value",      "defensive_crisis",        "e01c40"),
    (5,  "handler",         "v50_fold_dive_episode",   "e59190"),
    (7,  "hunt_and_battle", "sub_plan",                "ccc010"),
    (13, "line_gank_cover", "target_bush_v30",         "df1c80"),
    (21, "epic",            "v3_epic_group_line",      "dea4a0"),
]
# 이미 맞게 있던 것 (idx, addr) — ev1 노트만 붙인다
OK = {0: "d354c0", 1: "d5ba80", 6: "e657a0", 8: "defa20", 9: "ebd570", 14: "db90f0", 18: "dce220", 19: "d40b20",
      22: "deaa70", 23: "ec9bf0", 24: "d666c0"}
HOST02 = "caf9f0"   # #02 = BigPlan::sub_plan 호스트 · AttackNexus arm 0xcafa57 인라인
NAMES = {0: "ult", 1: "calculate_jungle_action_score", 2: "AttackNexusPlan::sub_plan", 3: "defensive_crisis", 4: "handle_line_defense",
         5: "v50_fold_dive_episode", 6: "v2_response_retreat_stance", 7: "EpicHuntAndBattlePlan::sub_plan", 8: "is_end",
         9: "check_favorable_engage_formation", 10: "should_end_object_finish_kill_priority_battle", 11: "v3_fall_back_to_passive",
         12: "handle_chat", 13: "target_bush_v30", 14: "LineGankerPlan::update", 15: "single_try_engage", 16: "max_range_nearly_can_use",
         17: "DeathMatchBattle::new", 18: "v3_epicops_buff_window", 19: "best_jungle_goal", 21: "v3_epic_group_line",
         22: "v3_epicops_repair_need", 23: "is_object_being_taken_by_enemy", 24: "v3_serpen_contest_clear_win"}


def ledger_ev1():
    ev = {}
    for l in io.open(LEDGER, encoding="utf-8"):
        m = re.match(r"^\|\s*(\d{2})\s*\|", l)
        if m:
            c = [x.strip() for x in l.strip().strip("|").split("|")]
            ev[int(m.group(1))] = re.sub(r"[*`]", "", c[5]) if len(c) > 5 else ""
    for i in (21, 22, 23, 24):
        ev.setdefault(i, u"✅ DIFF 0(명세 밖·회계 보류)")
    return ev


def main():
    h = io.open(BASE, encoding="utf-8").read()
    m = re.search(r'(<script id="fndata" type="application/json">)(.*?)(</script>)', h, re.S)
    data = json.loads(m.group(2))
    by = {e["a"]: e for e in data}
    cg = json.load(io.open(CG, encoding="utf-8"))
    cg = {int(k): set(v) for k, v in cg.items()}
    rev = {}
    for a, cs in cg.items():
        for c in cs:
            rev.setdefault(c, set()).add(a)
    aim = json.load(io.open(AIMAP, encoding="utf-8"))["info"]
    ev1 = ledger_ev1()

    def name_of(a):
        e = by.get(a)
        return (u"%s::%s" % (e["m"], e["n"])) if e else u"0x%s" % a

    # ── 갱신 전 관계 스냅샷(우리 함수 · 이름 기준 옛 항목)
    before = {}
    for idx, mod, nm, old, _t, new in FIX:
        e = by.get(old)
        before[idx] = (old, [name_of(x) for x in e["c"]], [name_of(x) for x in e["o"]]) if e else (old, None, None)
    for idx, a in OK.items():
        e = by[a]; before[idx] = (a, [name_of(x) for x in e["c"]], [name_of(x) for x in e["o"]])
    for idx, mod, nm, a in ADD:
        e = by.get(a); before[idx] = (a, [name_of(x) for x in e["c"]] if e else None, [name_of(x) for x in e["o"]] if e else None)
    e = by[HOST02]; before[2] = (HOST02, [name_of(x) for x in e["c"]], [name_of(x) for x in e["o"]])

    # ── ① 옛 주소 항목 = 진짜 이름으로
    for idx, mod, nm, old, true, new in FIX:
        e = by[old]
        e["n"] = true; e["v"] = "re"; e["x"] = (u"09-12 ghidra 확정: 지문(fp)이 %s 로 붙였던 자리. 진짜 %s 는 0x%s" % (nm, nm, new))
        e["ai"] = e.get("ai", 1)
    # ── ② 진짜 주소 항목 = 우리 이름 (없으면 추가)
    mapset = set(by); added = set()
    def ensure(idx, mod, nm, a, note):
        e = by.get(a)
        if e is None:
            added.add(a)
            info = aim.get("0x" + a) or {}
            e = dict(a=a, n=nm, m=mod, b=info.get("bytes", 0), i=info.get("ins", 0), l=info.get("lines", []), v="re", f=[], x=note, ai=1, o=[], c=[])
            data.append(e); by[a] = e; mapset.add(a)
        else:
            if e["n"] != nm:
                note = (u"(지도 옛 이름 %s::%s → 정정) " % (e["m"], e["n"])) + note
            e["n"] = nm; e["m"] = mod; e["v"] = "re"; e["x"] = note
        return e
    for idx, mod, nm, old, true, new in FIX:
        ensure(idx, mod, nm, new, u"#%02d · 09-12 ghidra 확정(옛 fp 주소 0x%s 는 %s) · ev1 %s" % (idx, old, true, ev1.get(idx, "")))
    for idx, mod, nm, a in ADD:
        ensure(idx, mod, nm, a, u"#%02d · 원장 §1 · ev1 %s" % (idx, ev1.get(idx, "")))
    for idx, a in OK.items():
        e = by[a]; e["v"] = "re"; e["x"] = u"#%02d · ev1 %s · 런타임 DIFF 0 = 주소 증명(09-12)" % (idx, ev1.get(idx, ""))
    e = by[HOST02]
    e["n"] = u"BigPlan::sub_plan (host)"; e["m"] = "types"; e["v"] = "re"
    e["x"] = u"#02 AttackNexusPlan::sub_plan 은 독립 함수 없음 — 이 호스트의 점프테이블 idx14 arm 0xcafa57 에 인라인(09-12 ghidra) · midpin ev1 %s" % ev1.get(2, "")
    # ── ③ 관계 재계산(exe 콜그래프 · 지도 집합 내부 엣지만 · 방향: c=호출자 o=콜리)
    touched = [x[5] for x in FIX] + [x[3] for x in ADD] + list(OK.values()) + [HOST02] + [x[3] for x in FIX]
    changed = {}
    for a in set(touched):
        ai = int(a, 16); e = by[a]
        new_o = sorted("%x" % x for x in cg.get(ai, set()) if "%x" % x in mapset and "%x" % x != a)
        new_c = sorted("%x" % x for x in rev.get(ai, set()) if "%x" % x in mapset and "%x" % x != a)
        if new_o != sorted(e["o"]) or new_c != sorted(e["c"]):
            changed[a] = (list(e["c"]), list(e["o"]), new_c, new_o)
        e["o"], e["c"] = new_o, new_c
    # 다른 항목들의 c/o 에 남은 옛 참조는 그대로(주소 기준이라 유효)
    # ── 쓰기
    h2 = h[:m.start(2)] + json.dumps(data, ensure_ascii=False, separators=(",", ":")) + h[m.end(2):]
    h2 = h2.replace(u"TFM2 AI 함수 지도", u"TFM2 AI 함수 지도 (갱신 %s · ev1 반영)" % time.strftime("%Y-%m-%d"), 1)
    io.open(OUT, "w", encoding="utf-8", newline="").write(h2)

    # ── diff 보고
    L = [u"지시: AI함수지도(09-10) 를 현행 사실(RVA 정정 6건·누락 5건·#02 호스트·ev1 16/19)로 갱신하고 함수 상관관계가 바뀌었는지 검사",
         u"요약: 지도 항목 %d→%d. 이름 정정 6(옛 fp 주소=다른 함수)·추가 %d·호스트 표기 1·ev1 노트 24. 호출관계는 exe 콜그래프(cg_exe.json)로 재계산 — 아래 표가 관계 변화 전량. 게임 exe 는 09-10 과 동일(0.5.8)이라 **콜그래프 자체는 불변**이고, 바뀐 것은 「어느 주소를 그 이름으로 불렀나」다." % (len(data) - len([1 for _ in ADD if 1]) + 0, len(data), len(ADD)),
         u"", u"# 함수지도 갱신 · 관계 diff (%s · 게임 0.5.8)" % time.strftime("%Y-%m-%d"), u"",
         u"생성 = `MIG\\fnmap_update.py` → `_spec\\fnmap_base_current.html` → `mkmap_html.py` → `REPORT\\tfm2_ai_adjust\\AI함수지도.html`", u"",
         u"## 1. 이름 정정 6건 (지도가 다른 함수를 우리 이름으로 부르고 있었다)", u"",
         u"| # | 함수 | 옛 지도 주소 → 진짜 정체 | 진짜 주소 | 진짜 주소가 지도에 있었나 |", u"|---|---|---|---|---|"]
    for idx, mod, nm, old, true, new in FIX:
        was = before.get(idx)
        L.append(u"| %02d | `%s` | `0x%s` = %s | `0x%s` | %s |" % (idx, nm, old, true, new, u"없음(추가)" if new in added else u"있음(이름 정정)"))
    L += [u"", u"## 2. 함수별 호출관계 — 갱신 전(옛 지도) vs 후(exe 콜그래프 · 지도 641+ 집합 내부 엣지)", u"",
          u"| # | 함수 | 주소 | 호출자(전 → 후) | 콜리(전 → 후) | 변화 |", u"|---|---|---|---|---|---|"]
    order = sorted(set(list(OK) + [x[0] for x in FIX] + [x[0] for x in ADD] + [2]))
    for idx in order:
        a = {**OK, **{x[0]: x[5] for x in FIX}, **{x[0]: x[3] for x in ADD}, 2: HOST02}[idx]
        e = by[a]; ba, bc, bo = before.get(idx, (a, None, None))
        nc = [name_of(x) for x in e["c"]]; no = [name_of(x) for x in e["o"]]
        def fmt(xs): return u"—" if xs is None else (u"(0)" if not xs else u", ".join(u"`%s`" % x for x in xs))
        chg = u"주소 변경(옛 관계는 딴 함수 것)" if ba != a else (u"변화 없음" if (sorted(bc or []) == sorted(nc) and sorted(bo or []) == sorted(no)) else u"★관계 변화")
        L.append(u"| %02d | `%s` | `0x%s`%s | %s → %s | %s → %s | %s |" % (idx, NAMES.get(idx, e["n"]), a, (u"(옛 `0x%s`)" % ba if ba != a else u""), fmt(bc), fmt(nc), fmt(bo), fmt(no), chg))
    L += [u"", u"## 3. 판정", u"",
          u"- **콜그래프 불변**: exe 가 같아서(0.5.8 · sha `4ED3AED0`) 호출 엣지 자체는 09-10 과 동일. 「상관관계가 바뀌었나」의 답 = **엣지는 안 바뀌었고, 6 함수의 엣지가 딴 함수 것이었다**(위 §1). 그 6 함수의 진짜 관계는 §2 「주소 변경」 행.",
          u"- 「변화 없음」 행 = 옛 지도 관계가 그대로 유효 · 「★관계 변화」 행 = 지도 집합에 새 함수 5개가 들어오면서 엣지가 드러난 것(예: `#04` → `has_line_defense_threat`).",
          u"- 명세 쪽 관계(IR 호출자 수 `callers.count`·tcx `callees`)는 `specs20_v3.json` 이 정본이고 이 지도와 별개 축(IR 은 인라인 전, exe 는 인라인 후) — 두 수가 다르면 인라인/LTO 가 원인이지 오류가 아니다.",
          u"", u"## 4. 원문 — 정정표(스크립트 `FIX`/`ADD`/`OK`)와 관계 변화 raw", u"", u"```"]
    for a, (oc, oo, nc, no) in sorted(changed.items()):
        L.append(u"%s %s: callers %s -> %s | callees %s -> %s" % (a, by[a]["n"], oc, nc, oo, no))
    L.append(u"```")
    io.open(DIFF_MD, "w", encoding="utf-8", newline="\n").write(u"\n".join(L))
    print(u"OK: %s (%d entries) · diff → %s · 관계 변경 %d항목" % (OUT, len(data), DIFF_MD, len(changed)))


if __name__ == "__main__":
    main()
