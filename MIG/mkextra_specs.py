#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""mkextra_specs.py — 명세 밖 4함수(#21~#24)의 「간이 명세」 JSON 생성 (2026-09-13)

출처 = `_spec\\explain_extra.md`(IR 1회 독해 · 반증검증 라운드 미경유 · ev4). 정본 `specs20.json`(20함수 · 12라운드)과
**섞지 않는다** — 별도 `_spec\\extra_specs.json` 으로 두고 `mkmap_html.py` 가 지도 패널에만 합쳐 붙인다.
정식 편입(라운드 검증·게이트)은 `addspec.py` 로 따로 한다(그때 이 파일은 폐기).
판정 흐름(logic)은 explain_extra.md 의 ``` 블록에서 그대로 가져온다(손 복제 금지).

사용: python -X utf8 MIG\\mkextra_specs.py  → _spec\\extra_specs.json
"""
import io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "_spec", "explain_extra.md")
OUT = os.path.join(HERE, "_spec", "extra_specs.json")
GRADE = u"⚠간이 명세(2026-09-13 IR 1회 독해 · 반증검증 미경유 · ev4) — "


def logic_of(md, idx):
    m = re.search(r"^### #%02d .*?\*\*판정 흐름\*\*:\s*```\n(.*?)```" % idx, md, re.S | re.M)
    return m.group(1).rstrip() if m else u""


def R(base, off, name, note):
    return dict(base=base, offset=off, name=name, note=note)


def main():
    md = io.open(SRC, encoding="utf-8").read()
    S = []
    S.append(dict(
        id="epic__v3_epic_group_line", name="v3_epic_group_line",
        sym="_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic18v3_epic_group_line",
        src=u"game-ai\\src\\plan_legacy\\old\\epic.rs", src_line=814, layer=u"레거시 플랜",
        ir=dict(file="m09.ll", frm=64337, to=64420), base_round="ir1", rounds=1,
        one_line=GRADE + u"모르가드 운용 전략(Gather/Split14/Split131)에 따라 4~5명 그룹이 압박할 라인을 고른다 — Gather=압박선 점수 최대, Split14=먼 라인 제외, Split131=Mid 고정(없으면 압박선)",
        signature=dict(tcx="fn(MorgardUseStrategy, &game_core::PlayerState, &game_core::OperationData) -> Option<LineType>", vis="pub",
                       params=[dict(i=0, name="strategy", type="MorgardUseStrategy(8B)", note=u"하위 u32 니치 태그: 5=Gather / 6=Split14(상위 u32=position) / 그 외=Split131"),
                               dict(i=1, name="player", type="&PlayerState", note=u"team 만"), dict(i=2, name="op", type="&OperationData", note=u"cache·context")]),
        logic=logic_of(md, 21),
        reads=[R("PlayerState", "0x930", "info.team", u"압박선 계산의 팀"), R("OperationData", "0x0", "cache", u"&AbstractGameWithCache"), R("OperationData", "0x8", "context", u"&GameContext"),
               R("MobaMode", "0x1b0", "jungle_runner.epic.next_respawn_tick", u"Split14 far_line: epic>serpen 이면 Top"), R("MobaMode", "0x1e0", "jungle_runner.serpen.next_respawn_tick", u"같은 비교의 반대편")],
        writes=[],
        constants=[dict(value=5, src_line=815, meaning=u"니치 태그 경계(tag>4 → tag-5 = 0 Gather / 1 Split14)"), dict(value=1, src_line=819, meaning=u"LineType::Mid — Split131 고정 선호"),
                   dict(value=0, src_line=855, meaning=u"far_line Top(에픽이 세르펜보다 늦게 리스폰)"), dict(value=2, src_line=857, meaning=u"far_line Bottom")],
        calls=["game_ai::plan_legacy::old::epic::v3_group_press_line", "game_ai::plan_legacy::old::epic::v3_split14_far_line", "game_ai::rule_scope::line_exists", "AbstractGame::get_game_mode(vtable 0x40)"],
        knobs=[dict(what=u"Split131 고정 선호 라인", where="epic.rs:819", value="Mid(1)", effect=u"바꾸면 3분할 전략의 기본 압박선이 바뀐다(추정)")],
        unknown=[u"far_line 「먼」의 지리적 근거(에픽=탑측/세르펜=바텀측)는 맵 데이터 미확인 → 추정", u"v3_group_press_line 내부 점수표는 본 범위 밖"],
        resolved=[], new_knobs=[], still_unknown=[],
        exe=dict(addr="dea4a0", module="epic", bytes=180, instrs=None, evidence=u"runtime(ev1 DIFF 0 236,569 · 09-12)", callers=["dce220"], callees=[])))
    S.append(dict(
        id="epic__v3_epicops_repair_need", name="v3_epicops_repair_need",
        sym="_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic22v3_epicops_repair_need",
        src=u"game-ai\\src\\plan_legacy\\old\\epic.rs", src_line=833, layer=u"레거시 플랜",
        ir=dict(file="m09.ll", frm=64975, to=65551), base_round="ir1", rounds=1,
        one_line=GRADE + u"아군 생존/체력 분포를 적과 비교해 지금 수리(Repair) 가야 하나를 0/1/2 로 — 1: 비전투·아군≤적·40%이하 2명↑(채팅 동반) / 2: 30%이상 아군이 적 수·아군 수보다 적음 / 0: 아님",
        signature=dict(tcx="fn(&game_core::PlayerState, &game_core::OperationData, &BigPlan) -> u8", vis="in:game_ai",
                       params=[dict(i=0, name="team", type="i64(argpromote)", note=u"player.info.team"), dict(i=1, name="cache", type="*AbstractGameWithCache", note=u"player_champion"), dict(i=2, name="plan", type="&BigPlan(384B)", note=u"goal() 태그만")]),
        logic=logic_of(md, 22),
        reads=[R("AbstractGameWithCache", "0x1e0", "player_champion", u"[team]·[1-team] 5칸 non-null 수"), R("Entity", "0x670", "hp", u"현재 HP"), R("Entity", "0x628", "stat_cached.hp", u"최대 HP(0 이면 div-by-zero 패닉)"), R("BigPlan", "goal()", "BigGoal 태그", u"5=Battle 이면 1등급 판정 건너뜀")],
        writes=[],
        constants=[dict(value=41, src_line=838, meaning=u"hurt 임계 HP<41% (=40% 이하)"), dict(value=1, src_line=839, meaning=u"hurt>1 = 2명 이상"), dict(value=29, src_line=844, meaning=u"healthy 임계 HP>29% (=30% 이상)"), dict(value=5, src_line=837, meaning=u"BigGoal::Battle 태그")],
        calls=["game_ai::plan_legacy::types::BigPlan::goal", "AbstractGameWithCache::iter_champions(인라인)"],
        knobs=[dict(what=u"빈사 아군 임계", where="epic.rs:838", value=41, effect=u"올리면 수리 1등급이 잦아진다(추정)"), dict(what=u"건강 아군 임계", where="epic.rs:844", value=29, effect=u"올리면 healthy 가 줄어 2등급 수리가 잦아진다(추정)"), dict(what=u"빈사 인원 기준", where="epic.rs:839", value=u"hurt>1", effect=u"2명 이상일 때만 1등급")],
        unknown=[u"1(채팅 동반)과 2(objective 만)의 설계 의도 — 소스 주석 부재"],
        resolved=[], new_knobs=[], still_unknown=[],
        exe=dict(addr="deaa70", module="epic", bytes=1230, instrs=None, evidence=u"runtime(ev1 DIFF 0 1,398,759 · 09-12) · internal fastcc → rlib 패치", callers=["dce220", "dda220"], callees=[])))
    S.append(dict(
        id="objective_helpers__is_object_being_taken_by_enemy", name="is_object_being_taken_by_enemy",
        sym="_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers30is_object_being_taken_by_enemy",
        src=u"game-ai\\src\\plan_legacy\\team_plan\\objective_helpers.rs", src_line=313, layer=u"점수화·술어",
        ir=dict(file="m15.ll", frm=53833, to=54066), base_round="ir1", rounds=1,
        one_line=GRADE + u"오브젝트(모르가드/세르펜)가 지금 적에게 먹히는 중인지 — 캠프를 최근 1초 내 봤고 ∧ 몹이 피해 입은 채 보이고 ∧ 적 예상 처치시간 ≤20초",
        signature=dict(tcx="fn(&PlayerState, &OperationData, &GoalData, &TeamPlan, WavePriorityObject) -> bool", vis="in:game_ai",
                       params=[dict(i=4, name="target", type="WavePriorityObject(1B)", note=u"0=Morgard / 1=Serpen (i1 전달)")]),
        logic=logic_of(md, 23),
        reads=[R("GameSetting", "0x12f8", "tick_per_second", u"tps"), R("GameContext", "0x38", "tutorial", u"에픽 {0,7,8} / 세르펜 {0,5,7,8} 외면 false"),
               R("TeamPlan", "0x80", "obj_spawn.epic_camp_last_visible_tick", u"seen = +tps >= tick"), R("TeamPlan", "0x88", "obj_spawn.serpen_camp_last_visible_tick", u"세르펜 판"),
               R("GoalData", "0x88", "epic.epic_enemy_tick", u"적팀 처치 예상 틱(duration)"), R("GoalData", "0xc0", "serpen.epic_enemy_tick", u"세르펜 판"),
               R("MobaMode", "0x1a0", "jungle_runner.epic.live_list", u"[0] = 몹 id"), R("MobaMode", "0x1d0", "jungle_runner.serpen.live_list", u"세르펜 판"),
               R("Entity", "0x5c0", "id", u"is_visible(team, id)"), R("Entity", "0x670", "hp", u"< max 면 피해 입음"), R("Entity", "0x628", "stat_cached.hp", u"최대 HP"), R("PlayerState", "0x930", "info.team", u"시야 판정 팀")],
        writes=[],
        constants=[dict(value=1, src_line=318, meaning=u"tps×1 — 캠프를 최근 1초 내 봤어야"), dict(value=20, src_line=323, meaning=u"tps×20 — 적 처치 예상 ≤20초"), dict(value=6, src_line=317, meaning=u"(tutorial-1)<u 6 = 튜토리얼 1..=6 이면 에픽 미스폰")],
        calls=["game_ai::rule_scope::morgard_exists", "game_ai::rule_scope::serpen_exists", "Game::as_moba", "AbstractGame::tick(vtable 0x28)", "AbstractGame::get_game_mode(0x40)", "AbstractGame::get_entity_by_id(0x1f0)", "AbstractGame::is_visible(0xf8)"],
        knobs=[dict(what=u"캠프 시야 신선도 창", where="objective_helpers.rs:318", value=u"tps×1", effect=u"늘리면 오래된 시야로도 「먹히는 중」 판정(추정)"), dict(what=u"적 처치 예상 창", where="objective_helpers.rs:323", value=u"tps×20", effect=u"올리면 더 이르게 빼앗김 경보(추정)")],
        unknown=[u"is_visible 내부 시야 규칙은 game_core 소관(2번째 인자 = entity id 확인)"],
        resolved=[], new_knobs=[], still_unknown=[],
        exe=dict(addr="ec9bf0", module="objective_helpers", bytes=364, instrs=None, evidence=u"runtime(ev1 DIFF 0 5,210,106 · 09-12)", callers=["dce220"], callees=[])))
    S.append(dict(
        id="serpen__v3_serpen_contest_clear_win", name="v3_serpen_contest_clear_win",
        sym="_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6serpen27v3_serpen_contest_clear_win",
        src=u"game-ai\\src\\plan_legacy\\old\\serpen.rs", src_line=52, layer=u"레거시 플랜",
        ir=dict(file="m05.ll", frm=53103, to=53408), base_round="ir1", rounds=1,
        one_line=GRADE + u"세르펜 근처 도달 가능한 적 전원 vs HP>39% 아군으로 resolve_fight 를 돌려 「확실히 이기는 싸움(Commit)」인지 — 적이 없으면 무조건 true",
        signature=dict(tcx="fn(usize, &PlayerState, &OperationData, &TeamPlan) -> bool", vis="pub",
                       params=[dict(i=0, name="version", type="usize", note=u"본문 미사용 · resolve_fight 1번 인자로 통과"), dict(i=3, name="team_plan", type="&TeamPlan", note=u"serpen_reachable_enemies 에 전달")]),
        logic=logic_of(md, 24),
        reads=[R("PlayerState", "0x930", "info.team", u"bounds<2"), R("PlayerState", "0x9c0", "info.position", u"as_index → 내 챔피언"), R("AbstractGameWithCache", "0x1e0", "player_champion", u"[team][pos]"),
               R("PlayerState", "0x180", "info.parameter", u"AthleteParameter → judge_accuracy"), R("GameContext", "0x0", "pool", u"bumpalo(allies collect_in)"), R("Entity", "0x670", "hp", u"아군 HP"), R("Entity", "0x628", "stat_cached.hp", u"최대 HP"), R("FightPrediction", "0x38", "line", u"== Commit(0)")],
        writes=[],
        constants=[dict(value=39, src_line=60, meaning=u"아군 전력 산입 임계 HP>39% (=40% 이상)"), dict(value=0, src_line=64, meaning=u"FightLine::Commit"), dict(value="0i8 / None", src_line=63, meaning=u"resolve_fight 6·7번 인자 고정값(의미 미확정)")],
        calls=["game_ai::plan_legacy::old::serpen::serpen_reachable_enemies", "game_ai::plan_legacy::old::fight_model::resolve_fight", "AthleteParameter::judge_accuracy", "Position::as_index(인라인)", "AbstractGameWithCache::iter_champions(인라인)"],
        knobs=[dict(what=u"아군 전력 산입 HP 임계", where="serpen.rs:60", value=39, effect=u"내리면 저체력 아군도 전력에 넣어 Commit 이 잦아진다(추정)")],
        unknown=[u"resolve_fight 의 i8=0 · Option<&Entity>=None 인자 의미(fight_model.rs:310 소관)", u"FightLine 결정 규칙(Commit/CommitAfterJoin/Disengage/Hold 중 0 만 통과)"],
        resolved=[], new_knobs=[], still_unknown=[],
        exe=dict(addr="d666c0", module="serpen", bytes=641, instrs=None, evidence=u"runtime(ev1 DIFF 0 35,419 · 09-12)", callers=["dce220", "ecab00"], callees=[])))
    for i, s in enumerate(S):
        s["i"] = 21 + i
        assert s["logic"], s["name"]
    io.open(OUT, "w", encoding="utf-8", newline="\n").write(json.dumps(dict(meta=dict(what=u"명세 밖 4함수 간이 명세(IR 1회 독해 · ev4)", source="explain_extra.md", date="2026-09-13"), specs=S), ensure_ascii=False, indent=1))
    print(u"OK %s (%d)" % (OUT, len(S)))


if __name__ == "__main__":
    main()
