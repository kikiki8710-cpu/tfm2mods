# -*- coding: utf-8 -*-
u"""10차 배치C patch.json 생성기 — `old` 문면을 정본에서 직접 뽑아 오타를 원천 차단한다."""
import io, json, os, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")

HERE = os.path.dirname(os.path.abspath(__file__))
P2 = os.path.join(HERE, "..", "..", "_spec", "specs20.json")
D = json.load(io.open(P2, encoding="utf-8"))
S = D["specs"]


def need(s, sub):
    if sub not in s:
        raise SystemExit(u"정본에 없는 문면: %r" % sub[:80])
    return sub


errors = []


def fix(path, old, new, kind, bc, ev, found_by="new"):
    errors.append({"path": path, "kind": kind, "old": old, "new": new,
                   "evidence": ev, "behavior_change": bc, "found_by": found_by})


# ── 10 : can_enemy_hit_objective 하드컷의 소스 줄 ───────────────────────────
fix("/specs[10]/consts[5]/src_line", 1188, 1190, u"실오류", False,
    u"`_gaibc/m10.ll:47496` `%23 = icmp ugt i64 %22, 19600000000, !dbg !53163` 이고 "
    u"`!53163` 의 inlinedAt 사슬 루트 = `fight_model.rs:1190`(dloc.py m10.ll 53163). "
    u"1188 은 `can_enemy_hit_objective` 의 **함수 머리줄**이다(tcx `_tcx/game_ai.json` "
    u"`game_ai::plan_legacy::old::can_enemy_hit_objective` sp.l=1188). "
    u"G12 는 명세의 `ir` 범위(m10.ll 49611~50100) 안만 훑어(srclinecheck.py:101·211) "
    u"아웃오브라인 콜리의 상수를 **구조적으로 못 본다** — 이 축의 사각지대.")

w10_5 = S[10]["knobs"][5]["where"]
fix("/specs[10]/knobs[5]/where", need(w10_5, u"fight_model.rs:1188"),
    u"fight_model.rs:1189~1190 (`steal_position_range = 140000` 선언 1189 / 제곱 비교 1190 "
    u"= `_gaibc/m10.ll:47496` `icmp ugt i64 %22, 19600000000`)",
    u"실오류", False,
    u"`!53082 = !DILocalVariable(name: \"steal_position_range\", ..., line: 1189)`(m10.ll:116960) "
    u"= 리터럴 140000 을 담는 지역변수 선언줄. 접힌 값 19600000000 의 소비 지점은 "
    u"m10.ll:47496(!53163 → fight_model.rs:1190). 1188 은 함수 머리줄이라 어느 쪽도 아니다. "
    u"⚠G13(whereline.py)은 `.ll` 줄만 대조하고 `*.rs:줄` 은 아예 안 본다(LINEREF 정규식).")

h10_1 = S[10]["resolved"][1]["now"]
fix("/specs[10]/history[1]/now",
    need(h10_1, u"steal_position_range (fight_model.rs:1189, m10.ll:47501)"),
    u"steal_position_range (선언 fight_model.rs:1189 / 비교 fight_model.rs:1190 = "
    u"`m10.ll:47496`; ~~m10.ll:47501~~ 은 그 다음 gep 줄이었다 — 10차 배치C 정정)",
    u"보강", False,
    u"m10.ll:47501 은 `%25 = getelementptr inbounds nuw i8, ptr %0, i64 1216` 이고 "
    u"19600000000 을 쓰는 명령은 47496 이다(awk 직접 확인).")

u10_3 = None
for t in S[10]["unknown"]:
    if t.startswith(u"path_finder::is_enemy_well_danger"):
        u10_3 = t
fix("/specs[10]/open",
    need(u10_3, u"version 이 여기서만 쓰이므로 버전 분기는 전적으로 이 함수 몫이다."),
    u"~~version 이 여기서만 쓰이므로 버전 분기는 전적으로 이 함수 몫이다~~ → **거짓**"
    u"(3차 배치C 확정 · 10차 배치C 재확인): `is_enemy_well_danger` 는 `%0`(version)을 "
    u"본문에서 한 번도 로드하지 않는다(`_gaibc/m03.ll:144500~144563`) — 이 계통에 버전 분기는 없다.",
    u"실오류", True,
    u"같은 명세의 `sig.params[0]` 주석과 `history[3]` 이 이미 「버전 분기표는 존재하지 않는다」로 "
    u"뒤집었는데 `closed[3]`(v2 `unknown`) 원문은 **반증된 주장을 그대로 달고 있다**. "
    u"mkdossier.py:286 이 `closed` 표를 키 `(q,a,ev)` 로 렌더하는데 mkspec3.py:479 는 "
    u"`{q, why}` 로 만들어 **닫은 근거 칸이 전 배치·전 라운드에서 공백**이라, 배치에는 "
    u"「반증된 문장 + 답 없음」만 보인다. 이 명세대로 `is_enemy_well_danger` 를 재구현하면 "
    u"없는 버전 분기를 넣게 된다.")

# ── dir 자기모순 4건 ────────────────────────────────────────────────────────
for pth, why in (
    ("/specs[10]/mem[12]/dir", u"m10.ll 49611~50100 에 `getelementptr .. i64 408`·그 주소의 load 0건"),
    ("/specs[10]/mem[15]/dir", u"m10.ll 49611~50100 에 `getelementptr .. i64 456`·그 주소의 load 0건"),
    ("/specs[13]/mem[8]/dir", u"m10.ll 11483~11731 에 `getelementptr .. i64 112` 0건"),
    ("/specs[14]/mem[15]/dir", u"m08.ll 94569~94941 에 `getelementptr .. i64 112` 0건"),
):
    fix(pth, None, "-", u"실오류", False,
        u"그 행의 근거란이 **「`dir` 은 `\"-\"` 가 맞다」고 자기 입으로 적어 두었는데** v2 행에 "
        u"`dir` 키가 없어 `mkspec3.py:381`(`x.get(\"dir\") or dirk`)이 소속 배열에서 `r` 을 "
        u"파생시킨다 ⟹ 표는 `r`, 근거는 `-` 로 **같은 행 안에서 모순**. "
        u"같은 규약이 `14 mem[25]/[26]`(`setup_limit`/`wait_limit`)에서는 v2 에 `dir:\"-\"` 가 "
        u"박혀 제대로 `-` 로 나온다 — 규약은 있는데 4행만 누락. 실측: %s. "
        u"G1/G14 어느 쪽도 「근거문↔dir 칸」을 대조하지 않는다." % why)

# ── 12 : 두 번째 게이트의 정체 ──────────────────────────────────────────────
e12_7 = S[12]["new_knobs"][4]["effect"]
OLDMAP = u"(0=Top∧Mid∧Bottom / 1=Mid∧Bottom / 2=Top / 3=Mid / 4=Bottom / ≥5=무조건 — 9 tutorial × 16 값 전수 정합)"
fix("/specs[12]/knobs[7]/effect", need(e12_7, OLDMAP),
    u"— 그 게이트의 정체는 **`rule_scope::play_code_allowed`(rule_scope.rs:117~124)** 이고, "
    u"`chat_allowed` 의 rule_scope.rs:173·176 에서 불린다. 실제 팔은 집합이 아니라 **술어**다: "
    u"`0 → morgard_exists(ctx)`(:119) / `1 → serpen_exists(ctx)`(:120) / "
    u"`2 → line_exists(ctx,Top)`(:121) / `3 → line_exists(ctx,Mid)`(:122) / "
    u"`4 → line_exists(ctx,Bottom)`(:123) / `≥5 → 무조건 통과`(switch default). "
    u"~~0=Top∧Mid∧Bottom / 1=Mid∧Bottom~~ 은 **외연만 같은 오독**이다 — "
    u"TutorialType 9종 전부에서 `spawn_epic{0,7,8}` = Top∩Mid∩Bottom, "
    u"`spawn_serpen{0,5,7,8}` = Mid∩Bottom 이라 실행으로는 갈리지 않는다(METHOD_MAP ⑥ 한계 3)",
    u"실오류", False,
    u"IR 원문으로 독립 확인: `_gaibc/m13.ll:53679~53685` switch(Chat 태그 47·48) 와 "
    u"`53362~53368`(태그 49). 팔 `%115`/`%45` 의 `!dbg`(!56897/!56737) 사슬 = "
    u"`spawn_epic(runner.rs:263) ← morgard_exists(rule_scope.rs:46) ← play_code_allowed(:119)`; "
    u"팔 1(!56861/!56698) = `serpen_exists(:50) ← play_code_allowed(:120)`; "
    u"팔 2·3·4(!56872/56883/56894) = `line_exists(:25) ← play_code_allowed(:121·122·123)`. "
    u"`behavior_change=false` 인 이유 = `tcxdict --enum TutorialType` 이 **variant 9개뿐**이라 "
    u"두 표현의 외연이 전 입력에서 일치한다(재구현 결과는 안 바뀐다). 바뀌는 것은 **의미**다.")

lg12 = S[12]["logic"]
fix("/specs[12]/logic",
    need(lg12, u"if !rule_scope::chat_allowed(data.context, &chat) { return; } // 내부 미조사"),
    u"if !rule_scope::chat_allowed(data.context, &chat) { return; }\n"
    u"// 내부(rule_scope.rs:128~180, `_gaibc/m13.ll:53199~53900`) = Chat 태그별 switch.\n"
    u"//   태그 47·48·49 는 **2단 게이트**: ① play_code_allowed(chat+0x1, :117~124)\n"
    u"//      ② push_line_code_allowed(chat+0x2, :113~114) = line_from_code(:105) 가 None(코드>=3)이면 통과.\n"
    u"//   상세 = knobs[3]~[8].",
    u"보강", False,
    u"`// 내부 미조사` 는 3차 이후 `knobs[3]~[8]` 과 `closed[1]`(45칸 전수 일치)로 이미 해소됐는데 "
    u"`logic` 만 옛 문면으로 남아 있었다(G6 의 빈틈 — G6 는 *값이 어긋났나*만 본다).")

# ── 11 : one_line 이 함수 설명이 아니라 절차 메모다 ────────────────────────
ol11 = S[11]["one_line"]
BAD = ol11[ol11.index(u"※ **v2** 정정"):]
fix("/specs[11]/one_line", need(ol11, BAD),
    u"※ 게이트는 **version >= 2** 다 — 함수명의 `v3` 가 아니다. "
    u"오라클 실행 확증: version 0·1 은 `LegacyPlanHandler` 6168B 스냅샷 diff 전무, "
    u"2 부터 `v3_lapse_passive_fallbacks=1`.",
    u"보강", False,
    u"현행 `one_line` 은 ①`정정( , 오라클:` 처럼 **괄호 첫 항이 빈 문자열**로 깨져 있고 "
    u"②뒤 절반이 함수 설명이 아니라 「`specgate G1` 이 왜 못 잡았나」라는 **파이프라인 메모**다. "
    u"`one_line` 은 어떤 게이트도 안 보는 칸이라(도시에 §1 표) 5라운드째 그대로였다. "
    u"사실 관계(version>=2)는 유지하고 절차 메모만 걷어낸다.")

# ── 삽입: 빠져 있던 행 ─────────────────────────────────────────────────────
errors.append({
    "op": "insert", "path": "/specs[12]/mem", "at": 5,
    "guard": u"game.data_ptr(&dyn AbstractGame 데이터 절반)", "guard_key": "name",
    "kind": u"보강", "behavior_change": False, "found_by": "new",
    "new": {
        "base": "AbstractGameWithCache", "offset": "0x0",
        "name": u"game.data_ptr(&dyn AbstractGame 데이터 절반)",
        "note": u"`_gaibc/m13.ll:29570` `%66 = load ptr, ptr %65`(= cache+0x0)이고 그 값이 "
                u"vtable+0x28 tick 의 **self 인자**로 들어간다(m13.ll:29575 "
                u"`%71 = invoke noundef i64 %70(ptr noundef nonnull %66)`). "
                u"10·11 명세는 같은 팻포인터를 +0x0/+0x8 **두 행**으로 적었는데 12 만 +0x8 한 행뿐이라 "
                u"기계 대조(tcxaudit·G18) 대상에서 빠져 있었다 · tcx 정본 대조( `tcxdict`: "
                u"`AbstractGameWithCache.game` = +0x0 의 16B `&dyn AbstractGame` 팻포인터 ⟹ "
                u"데이터 절반이 +0x0 )",
    },
    "evidence": u"m13.ll:29569~29575 원문. mem[5] 의 note 가 「+0x0=데이터」라고 적으면서 행은 안 만들었다.",
})

errors.append({
    "op": "insert", "path": "/specs[14]/mem", "at": 1,
    "guard": u"chats.ptr (RawVec 원소 배열 시작)", "guard_key": "name",
    "kind": u"실오류", "behavior_change": False, "found_by": "new",
    "new": {
        "base": "LineGankerPlan", "offset": "0x8",
        "name": u"chats.ptr (RawVec 원소 배열 시작)",
        "note": u"★**표에 통째로 빠져 있던 행**(10차 배치C). 두 취소 경로가 모두 읽는다 — "
                u"`_gaibc/m08.ll:94879` `%163 = getelementptr inbounds nuw i8, ptr %0, i64 8` / "
                u"`94880` `%164 = load ptr, ptr %163` 과 `94921~94922`(%177/%178). "
                u"이 포인터 + `len*24` 가 `Chat` 원소를 쓰는 **실제 주소**라(94882 "
                u"`getelementptr inbounds nuw { i8, [23 x i8] }, ptr %164, i64 %158`) "
                u"없으면 push 재현이 불가능하다. 12 명세는 같은 Vec 을 cap/ptr/len **세 행**으로 적었다 · "
                u"tcx 정본 대조( `tcxdict LineGankerPlan 0x8` = `chats.buf.inner.ptr.pointer.pointer` )",
    },
    "evidence": u"m08.ll:94879~94889 · 94921~94931 원문 + `tcxdict LineGankerPlan 0x8`.",
})

errors.append({
    "op": "insert", "path": "/specs[12]/knobs", "at": 3,
    "guard": u"★play_code_allowed — Chat+0x1 「어느 목표의 콜인가」 1단 게이트", "guard_key": "what",
    "kind": u"보강", "behavior_change": False, "found_by": "new",
    "new": {
        "what": u"★play_code_allowed — Chat+0x1 「어느 목표의 콜인가」 1단 게이트",
        "where": u"rule_scope.rs:117~124 `play_code_allowed` (chat_allowed rule_scope.rs:173·176 에서 호출) / "
                 u"`_gaibc/m13.ll:53679~53685`(Chat 태그 47·48) · `53362~53368`(태그 49)",
        "value": u"code(Chat+0x1) 0→morgard_exists · 1→serpen_exists · 2→line_exists(Top) · "
                 u"3→line_exists(Mid) · 4→line_exists(Bottom) · ≥5→무조건 통과",
        "effect": u"태그 47·48·49(Play 계열) 는 **2단 게이트**를 통과해야 `handle_chat_inner` 로 간다 — "
                  u"1단이 이 표, 2단이 `push_line_code_allowed`(Chat+0x2, knobs[7]). "
                  u"어느 팔의 술어를 바꾸면 그 튜토리얼에서 해당 목표의 Play 콜이 통째로 열리거나 막힌다. "
                  u"일반 경기(TutorialType::None)는 6술어 전부 참이라 무영향 = 튜토리얼 전용. "
                  u"★8차 배치C 가 오라클로 「0=Top∧Mid∧Bottom / 1=Mid∧Bottom」 이라 적었던 것의 정체이고, "
                  u"그 표기는 외연만 같다(knobs[7] 참조)",
    },
    "evidence": u"m13.ll:53679~53685·53362~53368 switch + !dbg 사슬(dloc.py: !56897→play_code_allowed:119, "
                u"!56861→:120, !56872/56883/56894→:121/122/123). 명세에는 knobs[7] 의 근거문 꼬리에만 "
                u"묻혀 있어 `knobs` 표를 훑어서는 찾을 수 없었다.",
})

errors.append({
    "op": "insert", "path": "/specs[10]/knobs", "at": 6,
    "guard": u"can_enemy_hit_objective 의 스킬2·궁 레벨 게이트", "guard_key": "what",
    "kind": u"보강", "behavior_change": False, "found_by": "new",
    "new": {
        "what": u"can_enemy_hit_objective 의 스킬2·궁 레벨 게이트",
        "where": u"`_gaibc/m10.ll:47564` `%47 = icmp ugt i64 %46, 2` (skill2) · "
                 u"`m10.ll:47594` `%59 = icmp ugt i64 %46, 4` (ult) — %46 = Entity+0x5c8 `level`",
        "value": u"skill2 는 level>2, ult 는 level>4",
        "effect": u"적이 오브젝트를 「때릴 수 있다」고 볼 때 어떤 이펙트의 사거리를 세는지를 정한다. "
                  u"레벨이 임계 이하면 그 슬롯 대신 **빈 이펙트 상수**(`@anon...40`)를 넣어 "
                  u"`is_in_range_ex` 가 사실상 실패한다(m10.ll:47566·47596 `select`). "
                  u"임계를 내리면 저레벨 적의 스킬2/궁 사거리까지 위협으로 세어 오브젝트 전투를 더 오래 유지하고, "
                  u"올리면 평타·스킬1 사거리만 보게 된다. "
                  u"⚠`history[7]` 이 3차에 이 사실을 확정했는데 `knobs` 에는 안 실려 있었다(G17 의 빈틈)",
    },
    "evidence": u"m10.ll:47561~47601 원문 + `tcxdict Entity 0x5c8` = `level` / `0x500` = `skill2_effect` / "
                u"`0x538` = `ult_effect`. knobs[5](140000² 하드컷)가 이미 같은 콜리 내부 상수를 "
                u"노브로 싣고 있으므로 수록 기준은 동일하다.",
})

ev_up = [
    {"path": "/specs[11]/knobs[14]", "from": 4, "to": 3,
     "evidence": u"`tcxdict LegacyPlanHandler 0x1802` = `v2_assign@Some.0.0: u8` — "
                 u"오프셋·타입이 tcx 정본과 일치(10차 배치C).", "found_by": "new"},
    # ⚠경로가 `knobs[8]` 이 아니라 `knobs[9]` 인 이유 = 같은 patch 의 `/specs[12]/knobs` insert 가
    #   **먼저** 적용돼(applypatch.py:460 → 472) v2 `knobs` 가 3→4 행이 되기 때문이다.
    #   삽입 후 `v3 knobs[9]` = `new_knobs[5]` = `★v3_epicops_armed 로 Press 콜 봉인`.
    {"path": "/specs[12]/knobs[9]", "from": 4, "to": 3,
     "evidence": u"`tcxdict LegacyPlanHandler 0x180a` = `v3_epicops_armed: bool` — "
                 u"오프셋·타입이 tcx 정본과 일치(10차 배치C). "
                 u"(경로는 같은 patch 의 knobs insert 적용 **후** 인덱스다)", "found_by": "new"},
]

brief = [
    u"★`mkdossier.py:286` 이 `closed` 표를 `('q','a','ev')` 로 렌더하는데 `mkspec3.py:479` 는 "
    u"`{'q':…, 'why':…}` 로 만든다 — 키 이름이 달라 **「닫은 근거」 칸이 전 함수·전 배치·전 라운드에서 공백**이다. "
    u"이번 라운드가 나에게 시킨 검사가 정확히 `closed[]` 의 근거 유효성인데, 도시에가 그 근거를 안 보여준다. "
    u"(`mkspec3_md.py:212` 는 `x['why']` 를 제대로 찍는다 — 두 렌더러의 계약이 다르다.)",
    u"★도시에 §5 규칙란의 「⚠**새 항목 추가(`append`)는 구현이 없다**」는 **거짓**이다. "
    u"`applypatch.py:242~277` 에 `op:insert`/`op:delete` 가 구현돼 있고 도시에 본문(임무 지시부)도 "
    u"insert 문법을 안내한다 — **같은 지시문 안에서 §5 와 본문이 정면으로 어긋난다**. "
    u"§5 만 읽은 배치는 「행 추가 불가」로 판단해 `빠진 행`을 산문에만 적고 끝낸다(8차 배치C 가 실제로 그랬다).",
    u"`is_closed` 의 근거 귀속이 어긋난 실례: `11 closed[5]`(rnd/player/debug 내부 사용처)와 "
    u"`11 closed[6]`(sub_plan/merge/passive_plan 내부)에 붙은 `why` 가 둘 다 "
    u"「tcx sig 가 (BigPlan, u8) — 뒤 8B 는 별도 반환값」인데 이건 `closed[4]` 의 답이다. "
    u"진짜 답은 `history[5]`·`history[4]` 에 따로 있다. 내용은 닫혀 있으므로 **표시 결함**이지만, "
    u"`why` 를 렌더하기 시작하면 이런 오귀속이 바로 드러난다 ⟹ 위 두 항목은 같이 고쳐야 한다.",
    u"G12(`srclinecheck.py`)는 명세의 `ir.frm~ir.to` 안만 훑는다(:101·:211). 그래서 "
    u"**아웃오브라인 콜리에서 온 상수**(10 consts[5] = `can_enemy_hit_objective` 의 19600000000)는 "
    u"후보 자체가 0이라 조용히 통과한다 — 이번 라운드의 실오류가 정확히 거기 있었다. "
    u"제안: `consts[].src_line` 의 파일이 명세의 `src` 와 다르거나, 값이 `ir` 범위 안에서 "
    u"**한 번도 안 나오면** `fnparts` 로 그 값이 등장하는 함수를 찾아 그쪽 범위까지 훑도록 확장.",
    u"G13(`whereline.py`)은 `[mg]\\d{2}\\.ll` 줄참조만 대조하고 `*.rs:줄` 형태의 `where` 는 "
    u"**아예 검사 대상이 아니다**(LINEREF 정규식). 내 담당 5함수의 `where` 51칸 중 `.ll` 참조가 없는 것이 "
    u"과반이고, 실제로 `10 knobs[5]`(`fight_model.rs:1188`)가 **함수 머리줄**을 가리키고 있었다. "
    u"제안: `*.rs:줄` 도 `_tcx` 의 `sp`(파일·줄범위)와 대조해 「그 함수 범위 안인가 / 머리줄 아닌가」만 봐도 "
    u"이 부류는 잡힌다.",
    u"`mem[].dir` 의 `\"-\"` 규약이 문서화돼 있지 않다. v2 행에 `dir:\"-\"` 를 넣으면 "
    u"`mkspec3.py:381` 이 존중하는데(`14 mem[25]/[26]` 이 그렇다), 같은 뜻을 **근거문에만** 적은 4행"
    u"(`10 mem[12]`·`10 mem[15]`·`13 mem[8]`·`14 mem[15]`)은 표가 `r` 로 나온다. "
    u"제안 게이트(G20): 근거문에 「`dir` 은 `\"-\"`」·「읽지도 쓰지도 않는다」가 있는데 행의 `dir` 이 `-` 가 "
    u"아니면 불일치. 반증식이라 오탐이 거의 없다.",
    u"★`applypatch.py` 의 출력이 **성공한 삽입을 「적용 실패」로 찍는다**. `--dry` 실행에서 "
    u"「★적용 실패 4건」 아래에 내 insert 4건의 경고(`⚠삽입 — 뒤 인덱스가 밀렸다`)가 나오는데 "
    u"바로 다음 줄 집계는 「정정 15 성공 / 0 실패」다. `log` 를 실패 목록으로만 렌더하는 탓이고, "
    u"「빨간불이 상수가 되면 아무도 안 본다」(`SPEC_RUNBOOK §S5-f`)가 **이 도구 자신에게** 생긴 사례다.",
    u"★`insert` 경고의 「이후 N행 이동」은 **v2 배열 안**만 센다. `/specs[12]/knobs at=3` 은 "
    u"「이후 0행 이동」이라 찍히지만, v3 `knobs` = v2 `knobs`+`new_knobs` 라 실제로는 "
    u"`new_knobs` 18행의 **v3 인덱스가 전부 +1** 된다. 더 위험한 것은 `ev_up` 이 `errors` **뒤에** "
    u"적용된다는 점이다(applypatch.py:460 → 472) — 같은 배열에 insert 를 낸 배치가 "
    u"`ev_up` 경로를 삽입 **전** 인덱스로 쓰면 **조용히 엉뚱한 행의 근거란에 도장이 찍힌다**. "
    u"(내 첫 판이 정확히 그랬고 `--dry` 는 `2/2 성공` 으로 통과시켰다 — 이번 patch 는 삽입 후 "
    u"인덱스 `/specs[12]/knobs[9]` 로 고쳐 냈다.) 제안: `apply_evup` 도 `guard`/`guard_key` 를 받아 "
    u"대상 행의 식별 문자열이 맞는지 확인하게 할 것.",
    u"`13` 의 `exe` 가 `None` 이라 도시에가 「`None`(None) · None바이트 · None명령」으로 찍는다. "
    u"`target_bush_v30` 은 `define internal fastcc` 라 exe 에 독립 함수가 없는 것이 정상인데, "
    u"표기가 「조인 실패」와 구별되지 않는다(`callers: []` 를 두고 이미 같은 주의가 적혀 있다).",
]

out = {"round": 10, "batch": "C", "errors": errors, "ev_up": ev_up, "brief_errors": brief}
io.open(os.path.join(HERE, "patch.json"), "w", encoding="utf-8").write(
    json.dumps(out, ensure_ascii=False, indent=1))
print(u"errors=%d ev_up=%d brief=%d" % (len(errors), len(ev_up), len(brief)))
