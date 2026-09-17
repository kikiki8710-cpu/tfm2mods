# -*- coding: utf-8 -*-
"""
calleeprop060.py — 「본체 동치」 판정 함수 중 콜리가 변경된 것(동작 동치 아님)을 뽑는다.
mig060_same.json 의 bad_callee(콜리 구→신 · mig060 판정) 를 최종 판정(specs20_v060 · mig060_judge 정정본)으로 다시 판정해
콜리가 정말 변경인 것만 남긴다. 출력 = _next/calleeprop060.md · specs20_v060.json 의 v060.callee_changed 갱신.
"""
import io, json, os, re, sys, collections
sys.stdout.reconfigure(encoding="utf-8")
HERE = os.path.dirname(os.path.abspath(__file__))
J = json.load(io.open(os.path.join(HERE, "_next", "mig060_same.json"), encoding="utf-8"))
_p = os.path.join(HERE, "_next", "mig060_same_eq46.json")   # r19/r20/r21 디컴 동치 46(mig060_same --only 로 콜리 등급만 채집 · 정렬은 실패해도 됨)
if os.path.exists(_p): J += json.load(io.open(_p, encoding="utf-8"))
JM = {r["old"]: r for r in json.load(io.open(os.path.join(HERE, "_next", "mig060_judge.json"), encoding="utf-8"))}
VP = os.path.join(HERE, "_spec", "specs20_v060.json"); V = json.load(io.open(VP, encoding="utf-8"))
VERD = {sp["v060"]["addr_058"]: sp["v060"]["verdict"] for sp in V["specs"] if sp["v060"]["addr_058"]}
NAME = {sp["v060"]["addr_058"]: sp.get("name") for sp in V["specs"] if sp["v060"]["addr_058"]}
OVERRIDE = {  # 비명세 콜리의 확정 판정(RE 근거)
    "eb82d0": (u"변경", u"check_kill_die_tick TLS 래퍼 → eda920/edb240: v3 신규 타임라인 알고리즘(r21 w1) · ABI 변경"),
    "12857f0": (u"변경", u"estimate_damage_to: 산식 동일 · 신챔프 override 3 · Effect vt 슬롯 시프트(RE 09-16 estimate_damage_to)"),
}
def final(old):
    """콜리 구 RVA → (최종 판정, 출처)"""
    if old in OVERRIDE: return OVERRIDE[old]
    v = VERD.get(old)
    if v: return (u"동치" if v.startswith((u"동치", u"✅동치")) else u"변경"), u"spec:" + v[:40]
    j = JM.get(old)
    if j:
        jv = j["verdict"].split("(")[0]
        if jv in (u"동일", u"이동만", u"오프셋 이동", u"오프셋만", u"오프셋만 변경"): return u"동치", u"judge:" + jv
        return u"변경?", u"judge:" + jv + u"(미대조)"
    return u"미지", u"판정 없음"
rows = []; c = collections.Counter()
for r in J:
    myv = VERD.get(r["old"], u"")
    if not myv.startswith((u"동치", u"✅동치")): continue
    bad = []
    for s in r.get("bad_callee") or []:
        m = re.match(r"([0-9a-f]+)→([0-9a-f]+)\s*(.*)", s)
        if not m: continue
        o, n, tag = m.groups()
        if tag.startswith(u"미지"): continue            # mig060 판정 없는 콜리(데이터/라이브러리/thunk) — 변경 근거 아님
        if tag.startswith(u"불일치"):                   # 정렬된 콜리 ≠ mig060 짝(vtable ICF/디스패치) — 별도 표기
            bad.append((o, n, NAME.get(o) or (JM.get(o) or {}).get("name") or "?", u"콜리 짝 불일치", tag)); continue
        fv, src = final(o)
        if fv != u"동치": bad.append((o, n, NAME.get(o) or (JM.get(o) or {}).get("name") or "?", fv, src))
    key = u"콜리 변경 있음" if any(b[3] == u"변경" for b in bad) else (u"콜리 변경?/짝 불일치" if bad else u"콜리 전부 동치")
    c[key] += 1
    rows.append((r["old"], r["new"], r["name"], myv, bad, key))
    for sp in V["specs"]:
        if sp["v060"]["addr_058"] == r["old"]:
            sp["v060"]["callee_changed"] = [dict(old=o, new=n, name=nm, verdict=fv) for o, n, nm, fv, _ in bad] or None
json.dump(V, io.open(VP, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
L = [u"# calleeprop060 — 본체 동치 함수의 콜리 변경 전파(09-17)", u"", u"판정: " + u" · ".join(u"%s %d" % kv for kv in c.most_common()), u"",
     u"> 본체가 동치라도 콜리가 변경이면 **명세 logic 은 재사용 가능 · 런타임 값은 콜리를 v3 로 바꿔야 game==mine**. 아래 콜리 목록이 재현 시 같이 갈아야 할 것.", u"",
     u"| 구 | 신 | 함수 | 본체 판정 | 변경 콜리(구→신 이름 · 최종 판정) |", u"|---|---|---|---|---|"]
for o, n, nm, myv, bad, key in sorted(rows, key=lambda x: (x[5] != u"콜리 변경 있음", x[5] != u"콜리 변경?(미대조)", x[0])):
    if not bad: continue
    L.append(u"| `%s` | `%s` | %s | %s | %s |" % (o, n, nm[:40], myv[:22], u"<br>".join(u"`%s→%s` %s · %s" % (b[0], b[1], b[2][:36], b[3]) for b in bad)))
io.open(os.path.join(HERE, "_next", "calleeprop060.md"), "w", encoding="utf-8").write(u"\n".join(L))
print(u" · ".join(u"%s %d" % kv for kv in c.most_common()))
