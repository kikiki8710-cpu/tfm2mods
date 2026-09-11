# -*- coding: utf-8 -*-
u"""mkspec3 — `_spec\\specs20.json`(v2) → `_spec\\specs20_v3.json`(v3) 재구성. (2026-09-11)

## 왜 스키마를 바꾸나
v2 는 같은 사실을 `logic`(산문) · `reads/writes`(표) · `constants` · `knobs` · `resolved` 에
**2~4곳 중복** 보관한다. 그래서 정정이 한 곳만 반영되고 나머지가 살아남았다
(2026-09-11 1·2차 반증검증에서 나온 ~90건 중 대부분이 이 부류).

v3 의 원칙 세 개:
 1. **정본 필드를 명시한다.** 오프셋=`mem` / 상수=`consts` / 시그니처=`callees`·`sig`.
    `logic` 은 **재구현용 의사코드일 뿐이고 어긋나면 정본이 맞다**(`precedence` 에 명문화).
 2. **게이트 필드는 사람이 안 채운다.** `callees[].sig` · `callers` · `siblings` 는
    tcx·IR 코퍼스에서 **자동 생성**한다 — 1·2차 누락의 3대 원인이 정확히 이 셋이었다.
 3. **`open[]` 에는 진짜 열린 것만.** 해소된 항목은 `closed[]` 로 옮긴다(삭제하지 않는다).
    3차는 `open[]` 만 보면 된다.

## 증거 등급 `ev`
 1 = 런타임 실측(game==mine DIFF=0)   2 = SDK 오라클 실행   3 = tcx/MIR 정본
 4 = LLVM IR 독해                     5 = 추론(근거 약함)
`ev<=3` 이 뒤집히면 사고다. `ev>=4` 가 뒤집히는 것은 정상 수렴이다.
"""
import io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import spec3lib as L
import tcxdict as TD
sys.path.insert(0, os.path.join(HERE, '_spec'))
import closelist as CL

SRC = os.path.join(HERE, "_spec", "specs20.json")
DST = os.path.join(HERE, "_spec", "specs20_v3.json")
STRIKE = re.compile(r"~~(?!~).+?~~")

D = json.load(io.open(SRC, encoding="utf-8"))
S = D["specs"]

# ── 증거 등급 판정 ───────────────────────────────────────────────────
EV = [
    (1, (u"DIFF=0", u"비트동일", u"런타임 실측")),
    (2, (u"오라클", u"실행 확정", u"실행 확증", u"실행 확인", u"실행 검증", u"MATCH", u"실행으로")),
    (3, (u"tcx", u"MIR", u"tcxdict", u"정본", u"DWARF", u"srcmap", u"줄 길이", u"줄길이")),
    (4, (u"IR", u".ll:", u"icmp", u"dbg", u"m0", u"g0")),
]


def evtier(txt):
    t = txt or u""
    for tier, keys in EV:
        if any(k in t for k in keys):
            return tier
    if any(k in t for k in (u"추정", u"보인다", u"으로 보임", u"근거 없음")):
        return 5
    return 4


# ── 해소 판정: `open` 에서 내릴 것인가 ───────────────────────────────
DONE_MARK = (u"★해소", u"해소 —", u"확정(", u"→ **확정", u"-> **확정", u"삭제 —")
IDENT = re.compile(r"(?:\+0x[0-9a-f]{2,4}|[a-z][a-z0-9_]{4,40}\.rs:\d+|[a-z_][a-z0-9_]{5,40})")
UNSURE = re.compile(u"(확정 못|확정하지 못|미확인|확인 못|확인 안|확인 불가|추정|모름|알 수 없|"
                    u"미확정|안 봄|안 읽|못 했|못 함|미탐색|미독해|미검증|재료 부재|표기 불가)")


def is_closed(i, txt, resolved_blob):
    u"""★기본값은 **열림**이다. 닫는 것은 근거가 명시적일 때만.

    초판은 "resolved 와 식별자 2개 이상 겹치면 닫힘"으로 자동 판정했다가
    `get_input_target 내부는 안 봄`·`base_sub_goal 내부`·`single_tower_dive_is_viable` 같은
    **진짜 미탐색까지 닫았다.** 과하게 열어두면 3차가 재확인만 하고 넘어가지만,
    과하게 닫으면 3차가 그것을 '새 발견'으로 또 집는다 — 없애려는 게 정확히 그것이다."""
    if any(m in txt for m in DONE_MARK):
        return True, u"본문에 해소 표기가 있다"
    why = CL.closed_reason(i, txt)
    if why:
        return True, why
    return False, None


CLASS = ((u"재료 부재", u"재료 부재"), (u"표기 불가", u"표기 불가"),
         (u"미탐색", u"미탐색"), (u"원리적", u"재료 부재"))


def classify(txt):
    for k, v in CLASS:
        if k in txt:
            return v
    return u"미탐색"


# ── 술어·피호출자 이름 수집 ──────────────────────────────────────────
CALLNAME = re.compile(r"\b([a-z_][a-z0-9_]{3,45})\s*\(")
SKIP = set(u"""if while for match let return fn move some none ok err self
print format vec box new_with min max abs sub add mul div shl shr sat unwrap
expect clone copy into from as_ref as_mut iter map filter fold any all count len
is_some is_none is_ok is_err unwrap_or saturating_sub saturating_add wrapping_add
checked_add store load gep icmp select switch phi call invoke usub umin umax sext
zext trunc bitcast inttoptr ptrtoint memcpy memset
abs_diff and_then filter_map min_by_key max_by_key collect_in from_iter_in new_in
copied cloned flatten into_iter iter_mut is_some_and unwrap_failed drop_glue
panic_bounds_check panic_const_div_by_zero panic_const_div_overflow alloca dangling
call_mut call_once pipe choose true false pred game context blackboard nexus
growth_range radius_mult sdiv udiv srem urem""".split())
# ★위 목록은 std/bumpalo/LLVM intrinsic/패닉 헬퍼/필드명이다. game_ai·game_core 의
#   **판정 술어가 아니므로** 시그니처를 물을 대상이 아니다(specgate G4 잡음의 원인이었다).


def harvest_callees(sp):
    names = set()
    for c in sp.get("calls") or []:
        if isinstance(c, str):
            names.add(c.split("::")[-1].split("(")[0].strip())
        elif isinstance(c, dict):
            for k in ("name", "fn", "callee"):
                if c.get(k):
                    names.add(str(c[k]).split("::")[-1])
    for m in CALLNAME.finditer(STRIKE.sub(u" ", sp.get("logic") or u"")):
        n = m.group(1)
        if n not in SKIP and not n.startswith("_") and not n.startswith("llvm."):
            names.add(n)
    return sorted(n for n in names if n and n not in SKIP and len(n) > 3)


# ★망글링 심볼에서 타입명을 뽑는다. rustc v0 망글링은 `<길이><이름>` 이라
#   길이접두를 **정확히 세어** 잘라야 한다.
#   ⚠초판은 `\d+([A-Z][A-Za-z0-9]*Plan)\b` 를 썼는데 `17LineGankCoverPlan15target_bush_v30` 처럼
#     `Plan` 뒤에 바로 숫자가 붙으면 워드경계가 성립하지 않아 **20개 전부 매치 0** 이었다.
#     그리고 specgate G3 가 `plan==null` 을 무조건 통과시켜 이 구멍을 못 잡았다
#     (3차 배치 C 가 13·14 에서 적발 — G3 가 막으려던 실패와 같은 형태).
LENPFX = re.compile(r"(\d+)([A-Za-z_][A-Za-z0-9_]*)")


def mangled_names(sym):
    u"""망글링 심볼 안의 `<길이><이름>` 후보를 **겹침 허용**으로 전부 돌려준다.

    ⚠순차 워커로 만들면 안 된다 — `NtB2_17LineGankCoverPlan` 에서 `2` 를 길이로 읽어
    `_1` 을 소비해 버리고 그 뒤 `17LineGankCoverPlan` 을 놓친다(실측 1/20).
    모든 시작 위치를 독립적으로 시도하고 길이가 맞는 것만 채택한다."""
    u"""⚠`re.finditer` 로도 안 된다 — 겹치지 않으므로 `…0ozCnw_7game_ai…` 의 `0` 이
    길이 0 으로 매치되며 문자열 전체를 삼킨다(실측 0/20). 위치를 직접 훑는다."""
    out = []
    for i in range(len(sym)):
        if not sym[i].isdigit() or (i and sym[i - 1].isdigit()):
            continue
        j = i
        while j < len(sym) and sym[j].isdigit():
            j += 1
        n = int(sym[i:j])
        if n < 1 or j + n > len(sym):
            continue
        name = sym[j:j + n]
        if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", name):
            out.append(name)
    return out


def plan_type(sp):
    u"""이 함수가 달려 있는 **impl 타입**을 돌려준다(자유 함수면 None).

    ⚠"Plan 으로 끝나는 것"만 찾으면 `LegacyPlanHandler`(05·06·11·12·15)·`DeathMatchBattle`(17)
    처럼 형제가 실제로 중요한 타입을 놓친다 — 3차 배치 C 가 `LegacyPlanHandler` 형제로
    `handle_chat_inner` 계열을 찾아냈다. **UpperCamel 이름 중 마지막 것**을 쓴다."""
    ups = [n for n in mangled_names(sp.get("sym") or u"")
           if n[:1].isupper() and "_" not in n]
    return ups[-1] if ups else None


# ── 변환 ─────────────────────────────────────────────────────────────
def conv(i, sp):
    resolved_blob = json.dumps(sp.get("resolved") or [], ensure_ascii=False)
    o = {
        "i": i, "id": sp.get("id"), "name": sp.get("name"), "sym": sp.get("sym"),
        "src": sp.get("src"), "src_line": sp.get("src_line"),
        "layer": sp.get("layer"), "ir": sp.get("ir"), "one_line": sp.get("one_line"),
    }

    # sig — tcx 정본으로 덮고, 사람이 쓴 note 는 role 로 보존
    leaf = (sp.get("name") or "").split("::")[-1]
    cands = L.fnlookup(leaf)
    mine = [c for c in cands if c.get("file") and
            os.path.basename(str(sp.get("src") or "")).lower() in str(c["file"]).lower()]
    tcxfn = (mine or cands or [None])[0]
    o["sig"] = {
        "tcx": (tcxfn or {}).get("sig"),
        "vis": (tcxfn or {}).get("vis"),
        "path": (tcxfn or {}).get("path"),
        "mir": (tcxfn or {}).get("mir"),
        "ev": 3 if tcxfn and tcxfn.get("sig") else 5,
        "params": [{"i": p.get("i"), "name": p.get("name"), "type": p.get("type"),
                    "role": p.get("note"), "ev": evtier(p.get("note"))}
                   for p in (sp.get("signature", {}) or {}).get("params", [])],
        "ret": (sp.get("signature", {}) or {}).get("returns"),
    }

    o["logic"] = sp.get("logic")
    o["logic_note"] = (u"★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/"
                       u"`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** "
                       u"(v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)")

    # mem — reads + writes 통합 + 사전 대조
    mem = []
    for dirk, field in (("r", "reads"), ("w", "writes")):
        for x in sp.get(field) or []:
            base, off = x.get("base"), x.get("offset")
            chk = None
            try:
                nb = TD.norm_base(base or "")
                if nb.startswith("@SKIP"):
                    chk = u"SKIP: " + nb[6:]
                else:
                    st, cs = TD.resolve_name(nb, strict=True)
                    chk = u"타입미해결" if st == "none" else u"타입해결"
            except Exception:
                chk = u"조회실패"
            row = dict(x)                      # ★원본 승계 — 골라 담으면 새 키가 사라진다
            row.update({"dir": dirk, "chk": chk, "ev": evtier(x.get("note"))})
            mem.append(row)
    o["mem"] = mem

    # consts — kind 를 붙여 '노브'와 '구조적 태그'를 가른다
    consts = []
    for x in sp.get("constants") or []:
        m = (x.get("meaning") or u"")
        kind = (u"태그" if any(k in m for k in (u"태그", u"판별자", u"variant")) else
                u"센티널" if any(k in m for k in (u"센티널", u"니치", u"0xff", u"MAX")) else
                u"인덱스" if u"인덱스" in m else u"임계")
        row = dict(x)
        row.update({"kind": kind, "ev": evtier(m)})
        consts.append(row)
    o["consts"] = consts

    # knobs — knobs + new_knobs 통합
    knobs = []
    for src, tag in ((sp.get("knobs") or [], u"기존"), (sp.get("new_knobs") or [], u"신규")):
        for x in src:
            row = dict(x)
            row.update({"src": tag,
                        "ev": evtier(u"%s %s %s" % (x.get("where"), x.get("effect"), x.get("note")))})
            knobs.append(row)
    o["knobs"] = knobs

    # ★callees — sig 를 자동으로 채운다 (1차 01 「유령 노브」의 원인 봉쇄)
    callees, unmatched = [], []
    for n in harvest_callees(sp):
        rs = L.fnlookup(n)
        if not rs:
            unmatched.append(n)
            continue
        for r in rs[:3]:
            callees.append({"name": n, "path": r["path"], "vis": r["vis"],
                            "sig": r["sig"], "at": u"%s:%s" % (r["file"], r["line"]),
                            "mir": r["mir"], "xinl": r["xinl"], "ev": 3})
    o["callees"] = callees
    o["callees_unmatched"] = {
        "names": unmatched,
        "note": u"tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 "
                u"std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. "
                u"⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, "
                u"3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것."}
    o["callees_note"] = (u"★tcx 에서 **자동 생성**한다(mkspec3.py). 사람이 채우는 필드로 두면 빈다 — "
                         u"01 의 「정글 캠프 티어 컷」 유령 노브가 `is_jungle(&self, usize)` 시그니처를 "
                         u"안 본 탓에 생겼다. **인자를 받는 술어면 IR 의 임계 상수는 접힘일 수 있다.**")

    # ★callers — 전수 자동 열거 (09 「유일 호출처」의 원인 봉쇄)
    sym = sp.get("sym") or ""        # ★IR 은 `@_RNv...` 라 leading underscore 를 벗기면 0건이 된다
    sites = L.callsites(sym, dirs=("gai",)) if sym else []
    o["callers"] = {
        "count": len(sites),
        "sites": [u"%s:%d" % (s["file"], s["line"]) for s in sites],
        "scope": u"_gaibc(game_ai) 전량 call/invoke 스캔",
        "note": u"★자동 열거(spec3lib.callsites, 0.7초). "
                u"1차가 09 를 「유일 호출처」로 적었다가 실제 8곳이었다 — 세지 않고 쓰는 일을 구조적으로 막는다. "
                u"exe 쪽 `callers: []` 는 '호출자 없음'이 아니라 **조인 실패**다.",
        "ev": 4,
    }

    # ★siblings — 같은 Plan 의 다른 진입점 (14 target_bush_v41 누락의 원인 봉쇄)
    pt = plan_type(sp)
    if pt:
        sib = L.siblings(pt)
        o["siblings"] = {
            "plan": pt, "count": len(sib),
            "entries": [{"path": s["path"], "vis": s["vis"], "at": u"%s:%s" % (s["file"], s["line"]),
                         "mir": s["mir"], "sig": s["sig"]} for s in sib],
            "note": u"★자동 열거. 14 가 `update` 만 보고 `sub_plan`/`next_plan` 이 쓰는 "
                    u"`target_bush_v41`·`target_bush` 를 통째로 놓쳤다 — 형제를 안 보면 "
                    u"'이 플랜의 목표'를 틀리게 재구현한다.",
            "ev": 3,
        }
    else:
        o["siblings"] = {"plan": None, "count": 0, "entries": [],
                         "note": u"Plan 타입 메서드가 아니다(자유 함수)."}

    # open / closed
    op, cl = [], []
    for txt in (sp.get("unknown") or []) + (sp.get("still_unknown") or []):
        t = str(txt)
        done, why = is_closed(i, t, resolved_blob)
        if done:
            cl.append({"q": t, "why": why})
        else:
            op.append({"q": t, "class": classify(t), "ev": evtier(t)})
    o["open"] = op
    o["closed"] = cl
    o["open_note"] = (u"★3차는 **`open[]` 만** 보면 된다. `closed[]` 는 이미 닫힌 것(삭제하지 않고 남긴다). "
                      u"판정 어휘 = 미탐색 / 재료 부재(범위 열거 필수) / 표기 불가. "
                      u"「불가」를 쓰기 전에 `METHOD_MAP §0` 을 볼 것.")

    # ★절단 금지 — 초판이 400자로 잘라 정보를 흘렸다. resolved 원문을 그대로 보존한다.
    o["history"] = [{"was": r.get("was"), "now": r.get("now"),
                     **{k: v for k, v in r.items() if k not in ("was", "now")}}
                    for r in (sp.get("resolved") or [])]
    o["history_note"] = (u"v2 `resolved[]` 원문 그대로(정정 이력). **본문이 아니라 참조용**이다 — "
                         u"여기 있는 사실이 `mem`/`consts`/`knobs`/`callees` 에 반영됐는지는 "
                         u"`specgate.py` 가 검사한다.")
    o["exe"] = sp.get("exe")
    o["calls_raw"] = sp.get("calls")          # v2 원문 보존(손실 방지)
    o["base_round"] = sp.get("base_round")
    o["rounds"] = sp.get("rounds")
    # ★남은 v2 키는 이름을 몰라도 통째로 승계한다 — 하드코딩하면 새 키가 조용히 사라진다
    KNOWN = {"id", "name", "sym", "src", "src_line", "layer", "ir", "one_line", "signature",
             "logic", "reads", "writes", "constants", "calls", "knobs", "unknown", "exe",
             "resolved", "new_knobs", "still_unknown", "base_round", "rounds"}
    extra = {k: v for k, v in sp.items() if k not in KNOWN}
    if extra:
        o["v2_extra"] = extra
    return o


def main():
    out = {
        "meta": {
            "game": D["meta"].get("game"), "date": "2026-09-11", "schema": "v3",
            "what": u"game_ai 판단함수 20개 명세 — 1·2차 반증검증 반영본(v3 재구성).",
            "precedence": u"★정본 우선순위: mem(오프셋) · consts(상수) · callees/sig.tcx(시그니처) "
                          u"> logic(의사코드). 어긋나면 앞의 것이 맞다.",
            "ev_tiers": {"1": u"런타임 실측(game==mine DIFF=0)", "2": u"SDK 오라클 실행",
                         "3": u"tcx/MIR/DWARF 정본", "4": u"LLVM IR 독해", "5": u"추론(근거 약함)"},
            "ev_rule": u"ev<=3 이 뒤집히면 사고다. ev>=4 가 뒤집히는 것은 정상 수렴이다.",
            "autofill": u"callees[].sig · callers · siblings 는 tcx/IR 코퍼스에서 자동 생성"
                        u"(spec3lib.py). 사람이 채우지 않는다.",
            "corrections": D["meta"].get("corrections", []),
        },
        "shared": D.get("shared"),
        "specs": [],
    }
    tot = {"open": 0, "closed": 0, "knobs": 0, "callees": 0, "unres": 0, "callers": 0}
    for i, sp in enumerate(S):
        o = conv(i, sp)
        out["specs"].append(o)
        tot["open"] += len(o["open"]); tot["closed"] += len(o["closed"])
        tot["knobs"] += len(o["knobs"]); tot["callees"] += len(o["callees"])
        tot["unres"] += len(o["callees_unmatched"]["names"])
        tot["callers"] += o["callers"]["count"]
        print(u"[%02d] %-44s open %2d / closed %2d · knobs %2d · callees %2d(미매칭 %d) · callers %d"
              % (i, (o["name"] or "")[:44], len(o["open"]), len(o["closed"]), len(o["knobs"]),
                 len(o["callees"]), len(o["callees_unmatched"]["names"]),
                 o["callers"]["count"]))
    out["meta"]["counts"] = {
        "functions": len(out["specs"]), "open": tot["open"], "closed": tot["closed"],
        "knobs": tot["knobs"], "callees": tot["callees"],
        "callees_unresolved": tot["unres"], "caller_sites": tot["callers"],
        "mem": sum(len(x["mem"]) for x in out["specs"]),
        "consts": sum(len(x["consts"]) for x in out["specs"]),
    }
    with io.open(DST, "w", encoding="utf-8", newline="\r\n") as f:
        f.write(json.dumps(out, ensure_ascii=False, indent=1))
    print(u"\n" + "=" * 92)
    print(u"%s  (%.0f KB)" % (DST, os.path.getsize(DST) / 1024.0))
    print(u"  " + u" · ".join(u"%s %s" % (k, v) for k, v in out["meta"]["counts"].items()))


if __name__ == "__main__":
    main()
