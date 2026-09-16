#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""fnmap_dead.py — 「아예 도달 못하는」 함수를 지도에서 옆으로 격리한다(`dead` 표식 · 전체 지도 오른쪽 격리 상자). (2026-09-16 신설 · 유저 지시)

기준(단순 미발화는 격리하지 않는다 — 예: #10 should_end_object_finish_kill_priority_battle 은 특수 상황 미발화라 그대로 둔다):
  ① 모듈 death_battle / single_battle / single_line = 데스매치·SingleLane 전용(MOBA 5v5 도달 불가 · 판 8 배경 리그 sim 미발화 전부)
  ② 모듈 hunt_and_battle = #07 사장 variant(EpicHuntAndBattle 플랜 미생성 · 09-13 확정)
  ③ handle_press_epic 0xdcd930 = version≤1 전용 호출부(NA · 09-13)
  ④ single_try_engage 0xe5c1f0 · single_handle_solokill retain 0xca4a80 = SingleLane 전용(update 호출부 2곳 전부 get_game_mode()==SingleLane 가지 · 09-13)
  ⑤ check_cell 사본 0xc87af0 = 미사용 사본(실사용 = around::check_cell 0xdc8550 DIFF 0 · 정적 호출자 0 · 판 8 미발화) — **추정**
JS(마커 1회): layered() 가 dead 노드를 rank 계산에서 빼고 오른쪽 격리 상자에 격자로 놓는다 · 노드 점선·반투명 · 상세 패널 note · 레이블 「⛔도달 불가(격리) N」.
멱등: dead 표식을 전부 지우고 다시 단다.
사용: python -X utf8 MIG\\fnmap_dead.py [--map <html>]
"""
import io, json, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, __import__("os").path.dirname(__import__("os").path.abspath(__file__)))
import fnmap_ret
MARK = "/* DEADNODES-JS v1 */"
MOD_DEAD = {"death_battle": u"데스매치 전용(MOBA 5v5 도달 불가 · 판 8 배경 리그 sim 미발화)",
            "single_battle": u"SingleLane 전용(MOBA 5v5 도달 불가)", "single_line": u"SingleLane 전용(MOBA 5v5 도달 불가)",
            "hunt_and_battle": u"#07 사장 variant — EpicHuntAndBattle 플랜이 생성되지 않음(09-13 확정 · 디스패처의 죽은 arm)"}
ADDR_DEAD = {"dcd930": u"version≤1 전용 호출부(NA · 09-13) — 현행 version 2 에선 도달 불가",
             "e5c1f0": u"SingleLane 전용 — update 호출부 2곳 전부 get_game_mode()==SingleLane 가지(09-13 · #15)",
             "ca4a80": u"SingleLane 전용(single_handle_solokill 의 retain 모노모프)",
             "c87af0": u"[추정] 미사용 사본 — 실사용 check_cell = 0xdc8550(DIFF 0) · 정적 호출자 0 · 판 8 미발화"}

def main():
    av = sys.argv[1:]
    H = av[av.index("--map") + 1] if "--map" in av else fnmap_ret.DEF_MAP
    h = io.open(H, encoding="utf-8").read()
    m1 = re.search(r'<script id="fndata" type="application/json">(.*?)</script>', h, re.S)
    d = json.loads(m1.group(1))
    n_dead = 0
    for n in d:
        n.pop("dead", None)
        why = ADDR_DEAD.get(n["a"]) or (MOD_DEAD.get(n.get("m")) if n.get("L") != "ext" else None)
        if why: n["dead"] = why; n_dead += 1
    h = h[:m1.start(1)] + json.dumps(d, ensure_ascii=False, separators=(",", ":")) + h[m1.end(1):]
    if MARK not in h:
        def rep(a, b):
            nonlocal h
            assert h.count(a) == 1, (a[:60], h.count(a)); h = h.replace(a, b)
        # ① rank 계산: dead 는 고립 띠 대신 격리 상자 — layers 에 안 넣는다
        rep(u"      nodes.forEach(function(d){ var r = (adj[d.a].o.length+adj[d.a].c.length) ? rank[d.a] : maxR+2; rank[d.a]=r; (layers[r]=layers[r]||[]).push(d.a); });",
            u"      " + MARK + u" // 도달 불가(dead) 노드는 층에 안 넣고 오른쪽 격리 상자에 따로 놓는다\n"
            u"      var deadL = [];\n"
            u"      nodes.forEach(function(d){ if(d.dead){ deadL.push(d.a); rank[d.a] = -1; return; } var r = (adj[d.a].o.length+adj[d.a].c.length) ? rank[d.a] : maxR+2; rank[d.a]=r; (layers[r]=layers[r]||[]).push(d.a); });\n"
            u"      deadL.sort(function(a,b){ return BY[a].m.localeCompare(BY[b].m) || label(BY[a]).localeCompare(label(BY[b])); });")
        # ② 좌표: 본 배치 뒤 격리 상자(오른쪽)
        rep(u"      laidOut = true;\n    }\n    function drawLayer(){",
            u"      /* 격리 상자: 본 배치의 오른쪽 끝 + 여백, 위에서부터 6열 격자 */\n"
            u"      var maxX = -1e9; nodes.forEach(function(d){ if(!d.dead && pos[d.a].x > maxX) maxX = pos[d.a].x; });\n"
            u"      var DX = maxX + NW*2.2, DC = 6;\n"
            u"      deadL.forEach(function(a,i){ pos[a].x = DX + (i%DC)*(NW+GX); pos[a].y = GY + Math.floor(i/DC)*(NH+6); pos[a].rank = -1; });\n"
            u"      deadBox = deadL.length ? {x: DX - NW/2 - 14, y: GY - NH/2 - 30, w: DC*(NW+GX) + 28, h: Math.ceil(deadL.length/DC)*(NH+6) + 44, n: deadL.length} : null;\n"
            u"      laidOut = true;\n    }\n    var deadBox = null;\n    function drawLayer(){")
        # ③ 노드/간선 클래스 + 상자
        rep(u"        N.push('<g class=\"n'+(sel&&sel.a===d.a?' sel':'')+(d.f.indexOf('O')>=0?' ev':'')+(d.n?'':' un')+'\" data-a=\"'+d.a+'\" transform=",
            u"        N.push('<g class=\"n'+(sel&&sel.a===d.a?' sel':'')+(d.f.indexOf('O')>=0?' ev':'')+(d.n?'':' un')+(d.dead?' dead':'')+'\" data-a=\"'+d.a+'\" transform=")
        rep(u"        M.push('<text class=\"mod\" x=\"'+(minX-NW/2-16).toFixed(1)+'\" y=\"'+(yy+4).toFixed(1)+'\" text-anchor=\"end\">'+(isIso ? '미연결 '+ranks[r] : '깊이 '+r+' · '+ranks[r])+'</text>');\n      });",
            u"        if(r == -1) return;\n"
            u"        M.push('<text class=\"mod\" x=\"'+(minX-NW/2-16).toFixed(1)+'\" y=\"'+(yy+4).toFixed(1)+'\" text-anchor=\"end\">'+(isIso ? '미연결 '+ranks[r] : '깊이 '+r+' · '+ranks[r])+'</text>');\n      });\n"
            u"      if(deadBox){ M.push('<rect class=\"deadbox\" x=\"'+deadBox.x.toFixed(1)+'\" y=\"'+deadBox.y.toFixed(1)+'\" width=\"'+deadBox.w.toFixed(1)+'\" height=\"'+deadBox.h.toFixed(1)+'\" rx=\"8\"></rect>'+\n"
            u"        '<text class=\"mod deadlb\" x=\"'+(deadBox.x+12).toFixed(1)+'\" y=\"'+(deadBox.y+18).toFixed(1)+'\">⛔ 도달 불가(격리) '+deadBox.n+' — 데스매치·SingleLane 전용 · 사장 variant · version≤1 · 미사용 사본</text>'); }")
        rep(u"svg.big g.n.dim{opacity:.16}",
            u"svg.big g.n.dim{opacity:.16}\nsvg.big g.n.dead .bx{stroke-dasharray:3 2; stroke:#B3261E; fill-opacity:.35}\nsvg.big g.n.dead text.lb{fill:var(--muted); font-style:italic}\nsvg.big .deadbox{fill:none; stroke:#B3261E; stroke-width:1.2; stroke-dasharray:6 4; stroke-opacity:.7}\nsvg.big text.mod.deadlb{fill:#B3261E; fill-opacity:.9; font-size:12px}")
        # ④ 상세 패널 note
        rep(u"    var note = '';\n    if(d.x){",
            u"    var note = '';\n    if(d.dead){ note += '<div class=\"note doc\" style=\"border-left-color:#B3261E\"><b>⛔ 도달 불가(격리)</b> — '+esc(d.dead)+' · 전체 지도에서 오른쪽 격리 상자에 있다.</div>'; }\n    if(d.x){")
    io.open(H, "w", encoding="utf-8").write(h)
    print(u"dead 표식 %d → %s" % (n_dead, H))

if __name__ == "__main__":
    main()
