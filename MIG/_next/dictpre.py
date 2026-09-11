# -*- coding: utf-8 -*-
u"""dictpre — 차기 대상(`LegacyPlanHandler::update`)이 건드리는 타입 중
**사전(tcxdict)이 못 풀거나 모호한 것**을 뽑아 `tcxaudit.ALIAS` 사전등록 후보를 만든다.

왜: 사전 자체는 tcx 에서 자동 생성돼 타입 16,355개가 이미 들어 있다(등록 불요).
손으로 등록해야 하는 것은 **짧은 이름·별칭 → 전체 def_path** 매핑(`tcxaudit.ALIAS`) 뿐이고,
그게 없으면 감사가 `확인불가`/`★모호` 로 떨어진다. 착수 첫날부터 감사가 돌게 미리 채운다.
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))
import tcxdict as TD
import spec3lib as L

TCX = json.load(io.open(os.path.join(os.path.dirname(HERE), "_tcx", "game_ai.json"), encoding="utf-8"))

# ① LegacyPlanHandler 필드 타입 전량
want = set()
for a in L.adt("LegacyPlanHandler", "game_ai"):
    pass
rows = TD.walk("game_ai::plan_legacy::handler::LegacyPlanHandler", depth=2)
for o, name, t, sz, note in rows:
    want.add(t)
print(u"① LegacyPlanHandler 필드(depth2) 타입 %d종" % len(want))

# ② 그 함수가 부르는 89종 심볼의 시그니처에 나오는 타입
sym = "_RNvMNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handlerNtB2_17LegacyPlanHandler6update"
irs = json.load(io.open(os.path.join(os.path.dirname(HERE), "_next", "irsize.json"), encoding="utf-8"))
u = [r for r in irs if r["sym"] == sym][0]
body = []
with io.open(os.path.join(r"C:\tfm2mods\_gaibc", u["file"]), "rb") as f:
    for i, raw in enumerate(f, 1):
        if u["frm"] <= i <= u["to"]:
            body.append(raw.decode("utf-8", "replace"))
txt = "".join(body)
callees = set(re.findall(r"@(_RN[A-Za-z0-9_$.]*7game_ai[A-Za-z0-9_$.]*)", txt))
TYPE = re.compile(r"([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)+)")


def names(s):
    out = []
    for i in range(len(s)):
        if not s[i].isdigit() or (i and s[i - 1].isdigit()):
            continue
        j = i
        while j < len(s) and s[j].isdigit():
            j += 1
        n = int(s[i:j])
        if n < 1 or j + n > len(s):
            continue
        nm = s[j:j + n]
        if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", nm):
            out.append(nm)
    return out


sigtypes = set()
for c in callees:
    ns = names(c)
    leaf = ns[-1] if ns else None
    if not leaf:
        continue
    for r in L.fnlookup(leaf):
        for m in TYPE.finditer(r.get("sig") or u""):
            sigtypes.add(m.group(1))
print(u"② 피호출 %d종의 시그니처에서 뽑은 타입 %d종" % (len(callees), len(sigtypes)))

# ③ 사전 해결 시험
cand = sorted({t for t in list(want) + list(sigtypes) if t})
fail, ambig, ok = [], [], 0
for t in cand:
    base = t.split("<")[0].strip().lstrip("&").replace("mut ", "").strip()
    if not base or base[0].islower() and "::" not in base:
        continue
    try:
        st, cs = TD.resolve_name(base, strict=True)
    except Exception as e:
        fail.append((base, u"예외 %s" % type(e).__name__))
        continue
    if st == "none":
        fail.append((base, u"사전에 없음"))
    elif len(cs) > 1 or (cs and len(cs[0][1]) > 1):
        ambig.append((base, u"후보 %d" % sum(len(x[1]) for x in cs)))
    else:
        ok += 1
print(u"\n③ 시험: 후보 %d종 → OK %d · 모호 %d · 실패 %d" % (len(cand), ok, len(ambig), len(fail)))
print(u"\n★모호(ALIAS 로 못박아야 하는 것):")
for b, w in sorted(set(ambig))[:40]:
    print(u"   %-52s %s" % (b, w))
print(u"\n★사전에 없음(std/제네릭 인스턴스이거나 이름이 다른 것):")
for b, w in sorted(set(fail))[:40]:
    print(u"   %-52s %s" % (b, w))
