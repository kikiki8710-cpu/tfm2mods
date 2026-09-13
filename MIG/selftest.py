# -*- coding: utf-8 -*-
u"""selftest — 명세 파이프라인 **도구 자신**의 회귀 시험. (10차 신설, 2026-09-11)

## 왜 만드나 — 「고쳤다」가 네 번이나 거짓이었다

| 라운드 | 내가 「고쳤다」고 한 것 | 실제 |
|---|---|---|
| 7차 | `srclinecheck` 의 phi 구제 | **죽은 코드였다** — `litpat` 가 phi 인입을 애초에 매치 못 했다(9차 배치A 적발) |
| 7차 | 「gep 잡음 제거」 | 실제로 안 됐다 — gep 오프셋은 `i64 16` 처럼 **타입 접두를 달고** 통과한다 |
| 8차 | `kind` 파생의 증거 꼬리 절단 | 재생성해도 **분포가 불변**이었다(내가 분포를 세서 알았다) |
| 9차 | `srclinecheck` 승격 | 순환 import 를 만들어 **`G12=0` 이라는 거짓 초록**이 떴다 |

전부 **고친 뒤 다시 재지 않아서** 생겼다. 코드를 바꾸는 것과 동작이 바뀌는 것은 다른 사건이다.
⟹ 이 파일이 그걸 **기계로 판정**한다. 고치고 → 돌리고 → 또 고치고 → 또 돌린다.

## 무엇을 보나
게이트 19개가 **실제로 돌아가는지**(죽은 게이트 = 거짓 초록) · 렌더러가 **모든 키를 찍는지** ·
`applypatch` 의 판정 함수들이 **느슨하지 않은지**(조용한 no-op 다섯 번의 재발 방지) ·
회귀 감사가 통과하는지.

사용:  python -X utf8 selftest.py          전체
       python -X utf8 selftest.py -v       실패 상세
"""
import importlib
import io
import json
import os
import subprocess
import sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

V3 = os.path.join(HERE, "_spec", "specs20_v3.json")
D = json.load(io.open(V3, encoding="utf-8"))
S = D["specs"]

RESULT = []


def ok(name, cond, detail=u""):
    RESULT.append((bool(cond), name, detail))
    return bool(cond)


def run(args, timeout=1800):
    e = dict(os.environ, PYTHONIOENCODING="utf-8")
    p = subprocess.run([sys.executable, "-X", "utf8"] + args, cwd=HERE,
                       capture_output=True, text=True, encoding="utf-8",
                       errors="replace", env=e, timeout=timeout)
    return (p.stdout or u"") + (p.stderr or u""), p.returncode


# ── T1. 게이트가 **실제로 돈다** ───────────────────────────────────────
def t_gates():
    u"""★9차 실사고: 순환 import 로 검사기가 죽었는데 `G12=0` 이 떴다.
    `broken()` 이 그걸 `!` 로 바꿨으니, 여기서는 **`!` 가 하나도 없어야** 통과다."""
    out, _ = run(["specgate.py"])
    ok(u"specgate 실행", u"G1 자기모순" in out, out[:200])
    ok(u"★죽은 게이트 없음(`!` 0개)", u"=**!**" not in out,
       u"검사기가 죽으면 0 이 아니라 ! 로 찍힌다 — 지금 ! 가 있다")
    # 각 검사기를 직접 import 해서 호출까지 해본다(specgate 가 예외를 삼켜도 여기선 드러난다)
    mods = [("srclinecheck", "G12"), ("whereline", "G13"), ("memdir", "G14"),
            ("kindchk", "G15"), ("paramrole", "G16"), ("histprop", "G17"),
            ("xreflogic", "G18"), ("knobval", "G19")]
    for m, g in mods:
        try:
            mod = importlib.import_module(m)
            importlib.reload(mod)
            n = sum(len(mod.check_spec(sp) or []) for sp in S)
            ok(u"%s(%s) 직접 호출" % (g, m), True, u"적발 %d" % n)
        except Exception as ex:
            ok(u"%s(%s) 직접 호출" % (g, m), False, u"%s: %s" % (type(ex).__name__, ex))


# ── T2. 렌더러가 **모든 키를 찍는다** ──────────────────────────────────
def t_render():
    u"""★10차 실사고: 고정 컬럼 이름이 실제 키와 어긋나
    `sig.params`(타입) · `siblings`(전 행) · `closed`(닫은 근거)가 **전 라운드 공백**이었다.
    `history` 는 함수마다 키가 달라 고정 컬럼으로는 원리적으로 못 찍는다."""
    try:
        import mkdossier as MD
        importlib.reload(MD)
    except Exception as ex:
        ok(u"mkdossier import", False, str(ex))
        return
    miss = []
    for sp in S:
        groups = [
            (u"sig.params", (sp.get("sig") or {}).get("params") or []),
            (u"siblings", ((sp.get("siblings") or {}).get("entries") or [])),
            (u"closed", sp.get("closed") or []),
            (u"history", sp.get("history") or []),
            (u"open", sp.get("open") or []),
            (u"mem", sp.get("mem") or []), (u"consts", sp.get("consts") or []),
            (u"knobs", sp.get("knobs") or []), (u"callees", sp.get("callees") or []),
        ]
        for gname, rows in groups:
            for r in rows:
                if not isinstance(r, dict):
                    continue
                # 그 행의 모든 키가 렌더 결과에 **값으로** 나타나야 한다
                t = MD.table([r], (), ())
                body = u"\n".join(t)
                for k, v in r.items():
                    if v in (None, u"", [], {}):
                        continue
                    frag = MD._cell(v)[:24]
                    if frag and frag not in body:
                        miss.append(u"%02d %s.%s" % (sp["i"], gname, k))
    ok(u"★렌더러가 키를 빠뜨리지 않음", not miss,
       u"빠진 칸 %d개: %s" % (len(miss), u", ".join(sorted(set(miss))[:8])))

    # ★KB 표기는 **바이트 기준**(10차 배치D — 한글은 UTF-8 3바이트라 문자 수로 세면 ~30% 과소)
    probe = u"한글" * 1000                      # 6000바이트 · 2000문자
    ok(u"★KB 표기가 바이트 기준", abs(MD.kb(probe) - 6000 / 1024.0) < 0.01,
       u"kb()=%.2f · 문자기준이면 %.2f" % (MD.kb(probe), 2000 / 1024.0))

    # ★`exe` 가 없을 때 「None바이트」로 찍지 않는다(10차 배치A·C)
    noexe = [sp for sp in S if not (sp.get("exe") or {}).get("addr")]
    if noexe:
        body = u"\n".join(MD.render_fn(noexe[0]))
        ok(u"★exe 없음을 `None바이트` 로 찍지 않는다", u"None바이트" not in body,
           u"specs[%d]" % noexe[0]["i"])


# ── T3. `applypatch` 판정이 **느슨하지 않다** ──────────────────────────
def t_apply():
    u"""★「조용한 no-op」이 **다섯 번** 났다. 전부 「부분문자열이면 같다고 치자」에서.
    여기서는 그 다섯 가지를 하나씩 재현해 **막혔는지** 본다."""
    try:
        import applypatch as AP
        importlib.reload(AP)
    except Exception as ex:
        ok(u"applypatch import", False, str(ex))
        return
    V2 = json.load(io.open(os.path.join(HERE, "_spec", "specs20.json"), encoding="utf-8"))
    sp = V2["specs"][9]

    # ①정수 new 에서 죽지 않는다(7차 배치D)
    try:
        r = AP.already(sp, {"path": "/specs[9]/consts[0]/src_line", "old": 1, "new": 2})
        ok(u"already: 정수 new 에서 안 죽는다", r is False, u"돌려준 값 %r" % r)
    except Exception as ex:
        ok(u"already: 정수 new 에서 안 죽는다", False, u"%s: %s" % (type(ex).__name__, ex))

    # ②`old` 가 아직 있으면 미적용(8차 배치B)
    c = (sp.get("constants") or [{}])[0]
    mean = c.get("meaning") or u""
    if len(mean) > 60:
        r = AP.already(sp, {"path": "/specs[9]/consts[0]/meaning",
                            "old": mean[-40:], "new": u"Z" * 90})
        ok(u"already: `old` 가 남아 있으면 미적용", r is False, u"돌려준 값 %r" % r)

    # ③없는 칸을 채울 때 명세 전체로 폴백하지 않는다(9차 배치C)
    blob = json.dumps(sp, ensure_ascii=False)
    frag = [x for x in blob.split(u'"') if len(x) > 40]
    if frag:
        r = AP.already(sp, {"path": "/specs[9]/knobs[0]/__nosuchkey__",
                            "old": None, "new": frag[0][:70]})
        ok(u"already: 없는 칸을 명세 전체로 안 뒤진다", r is False, u"돌려준 값 %r" % r)

    # ④v3 키가 v2 키로 옮겨진다(8차 배치C)
    ok(u"V2KEY 매핑 존재", AP.V2KEY.get("role") == "note", str(AP.V2KEY))

    # ⑤경고는 실패 통에 들어가지 않는다(10차 배치B·C)
    ok(u"WARN 통이 따로 있다", hasattr(AP, "WARN"), u"insert 경고가 「적용 실패」로 찍히면 안 된다")

    # ⑥★`insert` 의 `at` 이 **v3 인덱스**로 대상 배열을 고른다(10차 배치D)
    #   v3 `mem` = `reads`+`writes` 인데 초판은 항상 `reads` 에만 꽂혔다.
    import copy
    for field, first, second, big in (("mem", "reads", "writes", 99),
                                      ("knobs", "knobs", "new_knobs", 99)):
        D2 = copy.deepcopy(V2)
        lg = []
        AP.apply_error(D2, {"op": "insert", "path": "/specs[18]/%s" % field, "at": big,
                            "guard": "__ST__", "guard_key": "name",
                            "new": {"name": "__ST__"}, "kind": "보강"}, lg)
        t18 = D2["specs"][18]
        inb = any((x or {}).get("name") == "__ST__" for x in (t18.get(second) or []))
        ina = any((x or {}).get("name") == "__ST__" for x in (t18.get(first) or []))
        ok(u"insert: `%s` 의 큰 `at` 은 뒤 배열(`%s`)로 간다" % (field, second), inb and not ina,
           u"%s=%s · %s=%s" % (first, ina, second, inb))
        # 작은 at 은 앞 배열
        D3 = copy.deepcopy(V2)
        AP.apply_error(D3, {"op": "insert", "path": "/specs[18]/%s" % field, "at": 0,
                            "guard": "__ST2__", "guard_key": "name",
                            "new": {"name": "__ST2__"}, "kind": "보강"}, [])
        t18b = D3["specs"][18]
        ok(u"insert: `%s` 의 `at=0` 은 앞 배열(`%s`)로 간다" % (field, first),
           any((x or {}).get("name") == "__ST2__" for x in (t18b.get(first) or [])), u"")

    # ⑥-b ★★**빈 배열에 삽입해도 정본에 들어간다**(11차 배치B — 조용한 no-op 여섯 번째)
    #   `spec.get("writes") or []` 가 빈 배열에서 **새 객체**를 돌려줘 10차 삽입 1건이 사라졌다.
    for i18, fld, tgt in ((8, "mem", "writes"), (8, "knobs", "new_knobs")):
        D7 = copy.deepcopy(V2)
        sp7 = D7["specs"][i18]
        base = {"mem": "reads", "knobs": "knobs"}[fld]
        n0 = len(sp7.get(base) or [])
        lg3 = []
        AP.apply_error(D7, {"op": "insert", "path": "/specs[%d]/%s" % (i18, fld),
                            "at": n0 + 99, "guard": "__EMPTY__", "guard_key": "name",
                            "new": {"name": "__EMPTY__"}, "kind": "보강"}, lg3)
        got = json.dumps(D7["specs"][i18], ensure_ascii=False)
        ok(u"★insert: 빈 `%s` 에도 **정본에** 들어간다" % tgt, "__EMPTY__" in got,
           u"log=%s" % (lg3[:1]))

    # ⑦★`ev_up` 의 `guard` 가 엉뚱한 행을 막는다(10차 배치C — 삽입 뒤 인덱스가 밀린다)
    D4 = copy.deepcopy(V2)
    lg2 = []
    r2 = AP.apply_evup(D4, {"path": "/specs[9]/consts[0]", "from": 4, "to": 2,
                            "evidence": "T", "guard": "__없는문자열__"}, 9, "T", lg2)
    ok(u"★ev_up: guard 불일치면 적용하지 않는다", r2 is False and bool(lg2), u"돌려준 값 %r" % r2)
    D5 = copy.deepcopy(V2)
    cval = ((D5["specs"][9].get("constants") or [{}])[0]).get("meaning") or u""
    if len(cval) > 12:
        r3 = AP.apply_evup(D5, {"path": "/specs[9]/consts[0]", "from": 4, "to": 2,
                                "evidence": "T", "guard": cval[:12]}, 9, "T", [])
        ok(u"ev_up: guard 가 맞으면 적용된다", r3 is True, u"돌려준 값 %r" % r3)

    # ⑧★`from` 이 현재값과 다르면 **경고가 남는다**(10차 배치B — 오기가 조용히 통과했다)
    AP.WARN[:] = []
    D6 = copy.deepcopy(V2)
    AP.apply_evup(D6, {"path": "/specs[9]/consts[0]", "from": 999, "to": 2,
                       "evidence": "T"}, 9, "T", [])
    ok(u"★ev_up: 거짓 `from` 이 조용히 통과하지 않는다", bool(AP.WARN),
       u"WARN %d건" % len(AP.WARN))
    AP.WARN[:] = []


# ── T4. 회귀 감사 ─────────────────────────────────────────────────────
def t_audit():
    out, _ = run(["auditrounds.py"])
    ok(u"auditrounds 전건 유지됨", u"전건 유지됨" in out,
       [l for l in out.split(u"\n") if u"커버리지 결손" in l][:1])


# ── T5. 신선도·도구 인벤토리 ──────────────────────────────────────────
def t_misc():
    out, _ = run(["mktools.py", "--check"])
    ok(u"미분류 도구 0", u"미분류 0" in out, out.strip().split(u"\n")[0] if out else u"")
    out2, _ = run(["mkspec3.py"])
    ok(u"mkspec3 재생성", (u"functions 20" in out2) or (u"functions 4" in out2) or (u"functions 5" in out2), out2[:160])   # 09-13 r7 편입으로 40 · r8 로 57


def t_callee_anchor():
    u"""12차 — `callees` 앵커. 네 배치가 전부 이 블록을 적발했으므로 못 박는다."""
    sys.path.insert(0, HERE)
    import mkspec3 as MS
    # ① 토큰 경계 — `range` 는 `12range_adjust` 에 **걸리면 안 된다**(거짓 ev3 3건의 원인)
    sym = u"_RNvMNtNtCs97f5S1uJLkH_9game_core10simulation6effectNtB2_6Effect12range_adjust"
    toks = MS.base_tokens(sym)
    ok(u"★callees: 토큰 경계 — `range` 가 `range_adjust` 에 안 걸린다",
       u"range" not in toks and u"range_adjust" in toks, u"토큰 %s" % sorted(toks))
    # ② 비식별자 성분 — `<'a, 'b>` 가 앵커를 막으면 안 된다(놓친 ev3 9건의 원인)
    ids = MS.path_idents(u"game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus")
    ok(u"★callees: 수명 성분이 앵커를 막지 않는다",
       ids == [u"game_core", u"AbstractGameWithCache", u"iter_towers_without_nexus"], str(ids))
    # ③ 문법어 — `<impl X as Y>::m` 의 `impl`/`as` 는 망글링에 없다
    ids2 = MS.path_idents(u"<game_core::CombineEffect as game_core::EffectType>::range_adjust")
    ok(u"★callees: 문법어(impl/as)를 빼고 판별 성분은 살린다",
       u"as" not in ids2 and u"CombineEffect" in ids2 and u"EffectType" in ids2, str(ids2))
    # ④ 제네릭 인자 속 클로저 — 자기호출 날조 금지
    clos = (u"_RINvNtCsX_4core4iter6traits7collect12from_iter_in"
            u"NCNvNtCshdEBA0ozCnw_7game_ai10buff_value16defensive_crisis0EE")
    ok(u"★callees: 제네릭 인자 속 클로저를 호출로 읽지 않는다",
       u"defensive_crisis" not in MS.base_tokens(clos),
       u"토큰 %s" % sorted(MS.base_tokens(clos)))
    # ⑤ 성분 1개짜리 경로는 앵커로 못 쓴다(`update` 119행 거짓 ev3 사고)
    rs = [{"path": u"<A as B>::update", "crate": "game_ai"}]
    MS._rank_callees(rs, u"update", [u"_RNvXs_NtCsX_7game_ai1x1y6update"])
    ok(u"★callees: 판별 성분 1개면 앵커로 쓰지 않는다", rs[0]["_anchor"] is False,
       u"_anchor=%r" % rs[0]["_anchor"])


def t_guards():
    u"""12차 — 새로 넣은 억제 규칙이 **진짜 적발까지 삼키지 않는지**.

    ★이 절이 존재하는 이유: 부정문 가드를 넣자마자 슬롯 7개 중 5개를 억제해
      게이트가 「0」을 찍었는데 그건 깨끗해서가 아니라 **눈을 가려서**였다."""
    sys.path.insert(0, HERE)
    import histprop as HP, sharedchk as SH, kindchk as KC
    ok(u"★G17 부정문 가드: 지시(`ev 4→2`)는 억제하지 않는다",
       not HP.neg_slot(u"consts[3]·knobs[1] **ev 4→2**(⚠뒤에 해설이 "
                       u"붙고 대상이 아니고 적혀 있다)"),
       u"지시 뒤에 해설이 붙었다고 억제하면 조용한 no-op 이 된다")
    ok(u"★G17 부정문 가드: 「대상이 아니다」는 억제한다",
       HP.neg_slot(u"consts[2]`(값 13)는 이 지시의 대상이 아니고"),
       u"명세가 「아니다」라고 말할 수 있어야 한다")
    # ⚠초판은 `… or True` 로 써서 **언제나 통과**했다. 자기 자신이 가짜 검사였다.
    #   `dir` 이 갈리는 실제 데이터를 넣어 **발화하지 않는지**로 판정한다.
    probe = [{"name": u"a", "mem": [{"base": u"P", "offset": u"0x8", "name": u"f", "dir": u"-"}]},
             {"name": u"b", "mem": [{"base": u"P", "offset": u"0x8", "name": u"f", "dir": u"r"}]}]
    ok(u"★G20: `dir` 미기재를 cross-spec 결함으로 세지 않는다",
       not SH.check(probe)[0],
       u"12차 배치C 가 IR 로 반증 — `13` 은 인자승격이라 본문 로드가 0건이고 "
       u"읽기는 호출부에 있다. 채웠으면 없는 로드를 주장하게 된다")
    ok(u"★G20: 튜플 첨자 `.0` 은 표기 차이로 본다",
       SH.skel(u"ty.Champion.0.skill_cooldown") == SH.skel(u"ty.Champion.skill_cooldown"),
       SH.skel(u"ty.Champion.0.skill_cooldown"))
    ok(u"★G15: 구제는 WEAK 관측도 본다(`길이`)",
       u"DBGSTR" in KC.SUPPORT.get(u"길이", ()) and u"DBGSTR" in KC.WEAK,
       u"지지가 전부 WEAK 인 kind 는 규칙②가 `obs` 를 봐야 구제된다")


def main():
    for f in (t_gates, t_render, t_apply, t_audit, t_misc,
              t_callee_anchor, t_guards):
        try:
            f()
        except Exception as ex:
            ok(f.__name__, False, u"%s: %s" % (type(ex).__name__, ex))
    bad = [r for r in RESULT if not r[0]]
    print(u"=" * 92)
    for good, name, det in RESULT:
        mark = u"  OK  " if good else u"★FAIL "
        line = u"%s %s" % (mark, name)
        if (not good or "-v" in sys.argv) and det:
            line += u"\n        %s" % (det if isinstance(det, str) else repr(det))[:400]
        print(line)
    print(u"=" * 92)
    print(u"%d/%d 통과%s" % (len(RESULT) - len(bad), len(RESULT),
                            u"" if not bad else u"  → ★%d건 실패" % len(bad)))
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
