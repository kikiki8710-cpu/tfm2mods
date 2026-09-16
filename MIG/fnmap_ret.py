#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""fnmap_ret.py — 실측 호출 간선(`retedges.py` 출력)을 AI함수지도.html 에 얹는다. (2026-09-16 신설)

무엇을 바꾸나(멱등 · 마커로 재적용 방지):
  ① fndata: 노드마다 `im`=[[콜리, 횟수]](정적 지도에 없던 실측 간접 콜리) · `ic`=[[호출자, 횟수]](실측 호출자 · 지도 안) ·
     `ie`=[[호출자 rva, 횟수]](지도 밖 호출자 · 노드 없음) · `oc`={콜리: 횟수}(직접 간선 실측 횟수) · `ret`=총 실측 호출수
  ② JS build(): `im` 간선을 edges/adj 에 추가(kind 'im') → 탑다운 rank·ego 에 반영 → 「깊이 0」 vtable 진입점이 진짜 부모 밑으로 내려간다
  ③ drawLayer/drawForce: `class="e im"`(초록 실선 · 굵게) · 상세 패널에 「★실측 호출자/콜리(리턴 주소 · 횟수)」 열 · 헤더 stat 「실측 간접 간선」
사용: python -X utf8 MIG\\fnmap_ret.py _next\\retedges.json [--map <html>]
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
DEF_MAP = r"C:\Users\jungs\Desktop\claude\tfm2\.claude\worktrees\swap-order-button-style-fe9e04\mods_report\tfm2_ai_adjust\AI함수지도.html"
MARK = "/* RETEDGES-JS v1 */"
# 지도 밖 호출자 실명(rvaname 09-16 · 패닉 Location 지문) — 나머지는 지문 없는 소형 헬퍼(미명명)
EXTNAME = {"1879080": u"game_core mode.rs 시뮬레이션 틱 루프(AI 루트 · Agent::get_input 호출)", "c87850": u"last_stand_flags LocalKey::with(LAST_STAND_MEMO 작성자)",
           "eb82d0": u"check_kill_die_tick LocalKey::with(DieTickCache 래퍼)", "ec9640": u"find_wave_priority_clear_line", "e7b9f0": u"end_check(lib.rs:1211)",
           # ↓ 지문 없음 · 실측 콜리 집합으로 추정(판 7 · scratchpad\ext_callees.py) — 확정은 ghidra
           "e35bd0": u"[추정] SubPlan::action_candidates JT 디스패처(서브플랜 15종 전부를 여기서 호출)",
           "e360e0": u"[추정] SubPlan::calculate_score_parameter_value 디스패처(line_defense·battle)",
           "caf2e0": u"[추정] BigPlan::update JT 디스패처(passive_line·battle·line_ganker update)",
           "cafe10": u"[추정] BigPlan::next_plan JT 디스패처(passive_jungle·line_ganker)",
           "ca6700": u"[추정] SubPlan::score 디스패처(serpen_check·steal)",
           "e49a50": u"[추정] LegacyPlanHandler::update 아웃라인 조각(GoalData::update·passive_plan·handle_chat 호출)",
           "e9bf10": u"[추정] upgrade_item 래퍼", "e9c610": u"[추정] buy_item 래퍼", "e25450": u"[추정] can_tower_focused 캐시 래퍼(1.04억회)",
           "c986c0": u"[추정] is_cleared 호출 헬퍼", "c93f60": u"[추정] v30 tower_aggro_risk 래퍼", "c94500": u"[추정] v19 non_champion_walkup 래퍼",
           "c94140": u"[추정] LineDefenseSubPlan::score 래퍼", "d84c60": u"[추정] abstract_input::attack 래퍼(position_eval 근방)"}

def main():
    av = sys.argv[1:]
    src = av[0]
    H = av[av.index("--map") + 1] if "--map" in av else DEF_MAP
    j = json.load(io.open(src, encoding="utf-8"))
    h = io.open(H, encoding="utf-8").read()
    m1 = re.search(r'<script id="fndata" type="application/json">(.*?)</script>', h, re.S)
    d = json.loads(m1.group(1)); BY = {n["a"]: n for n in d}
    # ① 데이터
    for n in d:
        for k in ("im", "ic", "ie"): n[k] = []
        n["oc"] = {}; n["ret"] = 0
    for e in j["edges"]:
        s, t, c = e["s"], e["t"], e["n"]
        if s not in BY or t not in BY: continue
        BY[t]["ret"] += c
        BY[t]["ic"].append([s, c])
        if e["kind"] == "indirect":
            BY[s]["im"].append([t, c])
        else:
            BY[s]["oc"][t] = c
    for e in j["ext"]:
        t = e["t"]
        if t in BY:
            BY[t]["ie"].append([e["s_rva"], e["n"], EXTNAME.get(e["s_rva"], "")]); BY[t]["ret"] += e["n"]
    for n in d:
        n["ic"].sort(key=lambda x: -x[1]); n["im"].sort(key=lambda x: -x[1]); n["ie"].sort(key=lambda x: -x[1])
    n_im = sum(len(n["im"]) for n in d); n_ic = sum(1 for n in d if n["ic"] or n["ie"])
    h = h[:m1.start(1)] + json.dumps(d, ensure_ascii=False, separators=(",", ":")) + h[m1.end(1):]
    # ② JS (한 번만)
    if MARK not in h:
        def rep(a, b):
            nonlocal h
            assert h.count(a) == 1, (a[:60], h.count(a))
            h = h.replace(a, b)
        rep(u"      nodes.forEach(function(d){ d.o.forEach(function(t){ if(idx[t]!=null && t!==d.a){ edges.push([d.a,t]); adj[d.a].o.push(t); adj[t].c.push(d.a); } }); });",
            u"      nodes.forEach(function(d){ d.o.forEach(function(t){ if(idx[t]!=null && t!==d.a){ edges.push([d.a,t]); adj[d.a].o.push(t); adj[t].c.push(d.a); } }); });\n"
            u"      " + MARK + u" // 실측 간접 간선(리턴 주소 히스토그램 · 정적 지도에 없던 것)을 같은 그래프에 얹는다 — rank/ego 가 진짜 부모를 본다\n"
            u"      nodes.forEach(function(d){ (d.im||[]).forEach(function(p){ var t = p[0]; if(idx[t]!=null && t!==d.a && adj[d.a].o.indexOf(t)<0){ edges.push([d.a,t,'im',p[1]]); adj[d.a].o.push(t); adj[t].c.push(d.a); } }); });")
        rep(u"        E.push('<path class=\"e'+(isB?' back':'')+'\" data-s=\"'+e[0]+'\" data-t=\"'+e[1]+'\" d=\"'+d+'\"></path>');",
            u"        E.push('<path class=\"e'+(isB?' back':'')+(e[2]==='im'?' im':'')+'\" data-s=\"'+e[0]+'\" data-t=\"'+e[1]+'\" d=\"'+d+'\"><title>'+(e[2]==='im'?'실측 간접 호출 '+fmt(e[3])+'회':'직접 호출')+'</title></path>');")
        rep(u"        E.push('<path class=\"e\" data-s=\"'+e[0]+'\" data-t=\"'+e[1]+'\" d=\"M'+a.x.toFixed(1)+','+a.y.toFixed(1)+' L'+b.x.toFixed(1)+','+b.y.toFixed(1)+'\"></path>');",
            u"        E.push('<path class=\"e'+(e[2]==='im'?' im':'')+'\" data-s=\"'+e[0]+'\" data-t=\"'+e[1]+'\" d=\"M'+a.x.toFixed(1)+','+a.y.toFixed(1)+' L'+b.x.toFixed(1)+','+b.y.toFixed(1)+'\"></path>');")
        rep(u"    document.getElementById('rel').innerHTML =\n      col('이 함수를 부르는 함수', d.c) + col('이 함수가 부르는 함수', d.o);",
            u"    function colN(title, arr, ext){\n"
            u"      if(!arr.length && !(ext&&ext.length)) return '<div><h3>'+title+'</h3><p class=\"none\">없음(이번 판 미발화 또는 프로브 밖)</p></div>';\n"
            u"      return '<div><h3>'+title+' <span class=\"num\">('+(arr.length+((ext||[]).length))+')</span></h3><ul>'+\n"
            u"        arr.map(function(p){ var t = BY[p[0]]; return '<li><button data-a=\"'+p[0]+'\">'+esc(t?fullName(t):('FUN_1'+p[0]))+' <span class=\"num\">'+fmt(p[1])+'회</span></button></li>'; }).join('')+\n"
            u"        (ext||[]).map(function(p){ return '<li><span class=\"none\">exe 0x'+p[0]+(p[2]?' '+esc(p[2]):' (지도 밖 · 미명명)')+' '+fmt(p[1])+'회</span></li>'; }).join('')+'</ul></div>';\n"
            u"    }\n"
            u"    document.getElementById('rel').innerHTML =\n      col('이 함수를 부르는 함수(정적)', d.c) + col('이 함수가 부르는 함수(정적)', d.o) +\n"
            u"      colN('★실측 호출자(리턴 주소 · 판 7 · vtable/JT 포함)', d.ic||[], d.ie||[]) + colN('★실측 간접 콜리(정적 지도에 없던 것)', d.im||[]);")
        rep(u"    [fmt(edges),'직접 호출'],", u"    [fmt(edges),'직접 호출'],\n    [fmt(DATA.reduce(function(s,d){return s+((d.im||[]).length);},0)),'실측 간접 간선'],")
        rep(u"svg.big .e.dim{stroke-opacity:.07}", u"svg.big .e.dim{stroke-opacity:.07}\nsvg.big .e.im{stroke:var(--new); stroke-opacity:.9; stroke-width:1.7}\nsvg.big .e.im.out,svg.big .e.im.in{stroke-width:2.4}")
        rep(u"<i style=\"background:var(--ly-etc)\"></i>기타 · 옅은 띠 = 모듈로 추정</span>",
            u"<i style=\"background:var(--ly-etc)\"></i>기타 · 옅은 띠 = 모듈로 추정 · <i style=\"background:var(--new)\"></i>초록 선 = 실측 간접 호출(리턴 주소 히스토그램 · 09-16 판 7)</span>")
        rep(u".rel{display:grid; grid-template-columns:1fr 1fr; gap:0}", u".rel{display:grid; grid-template-columns:1fr 1fr 1fr 1fr; gap:0}\n@media (max-width:1100px){ .rel{grid-template-columns:1fr 1fr} }")
    io.open(H, "w", encoding="utf-8").write(h)
    print(u"im 간선 %d · 실측 호출자 있는 노드 %d → %s" % (n_im, n_ic, H))

if __name__ == "__main__":
    main()
