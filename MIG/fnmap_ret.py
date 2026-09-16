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
           "c94140": u"[추정] LineDefenseSubPlan::score 래퍼", "d84c60": u"[추정] abstract_input::attack 래퍼(position_eval 근방)",
           # ↓ 판 8(지도 전 노드 · 09-16) 추가 — rvaname Location 지문 / 콜리 집합
           "eb6100": u"fight_check::battle_action(fight_check.rs:622 · 본체 2/2)", "cd05f0": u"battle::base_battle_action(battle.rs:1306 · 본체 4/4)",
           "ccacf0": u"LineSafeSubPlan::action_candidates(line_safe.rs:26 · 본체 2/2)", "181cd60": u"game_core simulation.rs(1603/1741/1542 · 57 Location · 시뮬 본체)",
           "d95d00": u"[추정] game_ai simulation.rs:1848 캐시 래퍼(cached_damage_against 호출 5.2억회)", "d96190": u"[추정] simulation.rs:1848/game.rs:210 래퍼(estimate_damage_to 호출)",
           "d958b0": u"[추정] simulation.rs:1848 래퍼", "d96d00": u"[추정] tower_discipline.rs:533 LocalKey::with 캐시 래퍼", "c88300": u"[추정] action_score.rs:577/594 interaction 클로저·TLS 래퍼",
           "e03360": u"[추정] simulation.rs:1905 래퍼", "d6abe0": u"[추정] simulation.rs:1905 래퍼(v21_defensive_cc_score 호출)", "d663f0": u"[추정] simulation.rs:1905 래퍼",
           "ea0c00": u"[추정] line_defense.rs:125 클로저(max_range_nearly_can_use 호출)",
           "d70620": u"[추정] tower_discipline 공용 래퍼(aggro_damage·engage_requires_dive·survival_incoming 호출 · 9.9억회 · 지문 없음)",
           "d70530": u"[추정] position_eval 공용 래퍼(count_in_range_fold·engage_requires_dive 호출)", "e0cf10": u"[추정] max_range_cached TLS 래퍼(1.44억회)",
           "ca89a0": u"[추정] SerpenStanceData::update_plan 클로저 호출부", "ca8f60": u"[추정] EpicStanceData::update_plan 클로저 호출부",
           "eb8b00": u"[추정] available_cc_in_window 래퍼", "e0d720": u"[추정] support_min_action_range 래퍼"}
# ↓ ghidra-re 09-16 낮 · 23개 정체 판정(RE 2026-09-16_지도밖호출자23_ghidra정체판정_원문.md) — 위 [추정] 항목을 덮어쓴다(정정형)
EXTNAME.update({
    "e360e0": u"SubPlan::calculate_score_parameter_value 디스패처(JT 0x33e8954 · 17 arm · 인라인 14 · 단순 필드 산출 · ghidra 09-16)",
    "df1f50": u"LineGankCoverPlan::next_plan(BigPlan::next_plan JT arm 9 · ghidra 09-16)",
    "c88300": u"LocalKey::with<interaction_ctx 클로저>(InterActionCtx 메모 캐시 · action_score.rs:577 · ghidra 09-16)",
    "e03360": u"buff_value::noncombat_steroid_window(ghidra 09-16 · ⚠지도 d75cb0 라벨과 충돌 · 확인 필요)",
    "e0b030": u"fight_model::resolve_fight_stake_roster(09-13 ghidra 확정 · ⚠09-16 재판정은 resolve_join_stake 로 봤으나 그건 0xe05e70=#54 DIFF 0 이라 기존 확정 유지)",
    "eb5dd0": u"fight_check::expected_dps(오프셋 6개 일치 · ghidra 09-16)",
    "dd9f30": u"[추정] TeamPlan::can_near_enemies(can_near_enemies_range 위임 · ghidra 09-16)",
    "e25030": u"[추정 강] SmallActionLaneMinionPosition::target_score(ghidra 09-16)",
    "e2f4f0": u"buff_value_v54 내부 Filter/Map fold<min_by_key>(champion_hp_value 최소 선택 · ghidra 09-16)",
    "e02bc0": u"buff_value::v54_aoe_ally_heal_value(ghidra 09-16)",
    "eba320": u"[추정 강] fight_check::v48_projectile_profile(ExpectedGame 캐스트 시뮬 · JT 9 arm · ghidra 09-16 · ⚠지도 c875a0 라벨과 충돌)",
    "cd5ee0": u"battle::kite_reposition_point(1,193B · ghidra 09-16 · ⚠지도 cadcd0 라벨과 충돌)",
    "d70620": u"can_tower_focused 의 Chain<Chain<…>> fold 인스턴스(count · 판단은 클로저 e3b840 등 · ghidra 09-16)",
    "e25450": u"[추정] SmallActionLaneMinionPosition::choose_goal 후보 클로저(push_candidate 경로 · can_tower_focused 캐시 래퍼 아님 · ghidra 09-16)",
    "e0cf10": u"battle::max_range_cached 외곽(로스터 idx→TLS 키 · 폴백 e0e890 max_range · ghidra 09-16)",
    "ca6700": u"LegacyPlanHandler::get_small_action 점수 합성 클로저(SubPlan::score 디스패처 e388c0 호출 · W[CATTBL]/1000 · ghidra 09-16)",
    "e35bd0": u"SubPlan::action_candidates 디스패처(JT 0x33e8910 · 17 arm 전부 아웃라인 · ghidra 09-16)",
    "caf2e0": u"BigPlan::update 디스패처(JT 0x33d75b8 · 실호출 9 · ghidra 09-16)",
    "cafe10": u"BigPlan::next_plan 디스패처(JT 0x33d76ac · 실호출 3 · ghidra 09-16)",
    "e49a50": u"LegacyPlanHandler::update_on_dead(update 아웃라인 조각 아님 · ghidra 09-16)",
    "e9bf10": u"<AgentVerHamster as AiAgent>::upgrade_item vtable thunk(vtable 0x33ebba0 slot 16 · ghidra 09-16)",
    "e9c610": u"<AgentVerHamster as AiAgent>::buy_item vtable thunk(slot 15 · ghidra 09-16)",
    "e9bf40": u"<AgentVerHamster as AiAgent>::update_on_dead vtable thunk(slot 17 · ghidra 09-16)",
    "e388c0": u"SubPlan::score JT 디스패처(ghidra 09-16 · r17 실증과 일치)",
    "e0e890": u"battle::max_range(비캐시 본체 · ghidra 09-16)",
    "e248f0": u"[추정] SmallActionLaneMinionPosition::choose_goal(ghidra 09-16)",
})

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
    # 판 태그·footer 수치는 매 실행 갱신(JS 는 1회 패치라 문자열만 치환)
    tag = (j.get("meta") or {}).get("tag") or u"판 ?"
    h = re.sub(u"리턴 주소 · 판 ?[\d+]+", u"리턴 주소 · " + tag, h)
    h = re.sub(u"판 ?[\d+]+ 에서 진입부 프로브 \d+곳이", u"%s 에서 진입부 프로브 %d곳이" % (tag, (j.get("meta") or {}).get("probes", 0)), h)
    h = re.sub(u"슬롯당 16칸 히스토그램 · \d+행", u"슬롯당 16칸 히스토그램 · %d행" % (j.get("meta") or {}).get("rows", 0), h)
    h = re.sub(u"없던 간접 간선 \d+개와 지도 밖 호출자 \d+곳", u"없던 간접 간선 %d개와 지도 밖 호출자 %d곳" % (n_im, len(j["ext"])), h)
    h = re.sub(u"실측 간접 호출\(리턴 주소 히스토그램 · 09-16 판 ?[\d+]+\)", u"실측 간접 호출(리턴 주소 히스토그램 · 09-16 %s)" % tag, h)
    io.open(H, "w", encoding="utf-8").write(h)
    print(u"im 간선 %d · 실측 호출자 있는 노드 %d → %s" % (n_im, n_ic, H))

if __name__ == "__main__":
    main()
