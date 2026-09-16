# -*- coding: utf-8 -*-
"""
mkspec060.py — specs20_v3.json(0.5.8 정본 · 불변) 에서 specs20_v060.json 을 만든다.
각 spec 에 `v060` 블록을 붙인다:
  addr        0.6.0 RVA(mig060_judge · 정정 반영) / None(미발견)
  verdict     동치 / 동치(+필드) / 변경·한 줄 / 변경·다건 / 변경·전담 / 심층 / 미발견   (r19_result · r20_result · spec_patch_060 §B/§D 기준)
  patch_ref   패치 요지 + RE 파일·절(spec_patch_060 §B/§D 행)
  logic_060   변경 함수는 에이전트가 채운 패치된 logic(없으면 None) — 채우는 단계는 별도(applyspec060.py)
  common      횡단 갱신 재료 적용 메모(§A: 태그표·vt 슬롯·오프셋 표) — 텍스트 자동 치환은 하지 않는다(오염 위험) · 재현 시 §A 표를 같이 읽는다
  version_gate  이 함수에 version>=3 분기가 있는지(RE 근거) · 값은 런타임 미확정
사용: python mkspec060.py   → _spec/specs20_v060.json · _next/spec060_summary.md
"""
import io, json, os, re, sys, collections
sys.stdout.reconfigure(encoding="utf-8")
HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(io.open(os.path.join(HERE, "_spec", "specs20_v3.json"), encoding="utf-8"))
JM = {r["old"]: r for r in json.load(io.open(os.path.join(HERE, "_next", "mig060_judge.json"), encoding="utf-8"))}
def parse_result(path, col):
    """r19_result / r20_result 표에서 구 RVA → 판정 문자열"""
    out = {}
    for line in io.open(path, encoding="utf-8"):
        m = re.match(r"\|\s*(?:[A-F]\s*\|\s*)?`([0-9a-f]+)`\s*\|\s*`([0-9a-f]+)`\s*\|[^|]*\|\s*([^|]+)\|", line)
        if m: out[m.group(1)] = m.group(3).strip()
    return out
R19 = parse_result(os.path.join(HERE, "_next", "r19_result.md"), 3)
R20 = parse_result(os.path.join(HERE, "_next", "r20_result.md"), 4)
PATCH = {}
for line in io.open(os.path.join(HERE, "_next", "spec_patch_060.md"), encoding="utf-8"):
    m = re.match(r"\|\s*`([0-9a-f]+)`\s*\|\s*`([0-9a-f?]+)`\s*\|\s*([^|]*)\|\s*([^|]*)\|\s*([^|]*)\|\s*([^|]*)\|\s*([^|]*)\|", line)
    if m and m.group(1) not in PATCH: PATCH[m.group(1)] = dict(kind=m.group(5).strip(), note=m.group(6).strip(), ref=m.group(7).strip())
    m2 = re.match(r"\|\s*`([0-9a-f]+)`\s*\|\s*`([0-9a-f]+)`\s*\|\s*([^|]*)\|\s*([^|]*)\|\s*([^|]*)\|$", line.rstrip())
    if m2 and m2.group(1) not in PATCH: PATCH[m2.group(1)] = dict(kind=u"심층", note=m2.group(4).strip(), ref=m2.group(5).strip())
TIER1 = set()
for line in io.open(os.path.join(HERE, "_next", "r20_tier1.md"), encoding="utf-8"):
    m = re.search(r"`([0-9a-f]+)`→", line)
    if m: TIER1.add(m.group(1))
def verdict_for(o):
    if o in R19: return (u"동치" if R19[o].startswith(u"동치") else u"변경") + u"(r19: %s)" % R19[o]
    if o in R20: return R20[o] + u"(r20)"
    j = JM.get(o)
    if not j or "new" not in j: return u"미발견"
    v = j["verdict"].split("(")[0]
    if v in (u"동일", u"이동만", u"오프셋 이동", u"오프셋만"): return u"동치(mig060_same 확정: %s)" % v
    if o in TIER1: return u"심층(티어1)"
    return u"변경(mig060: %s)" % v
out = []; C = collections.Counter()
for sp in D["specs"]:
    o = ((sp.get("exe") or {}).get("addr") or "").lower()
    j = JM.get(o, {}); p = PATCH.get(o)
    vd = verdict_for(o) if o else u"exe 없음"
    key = vd.split("(")[0]; C[key] += 1
    blk = dict(addr=j.get("new"), addr_058=o or None, verdict=vd, patch_kind=(p or {}).get("kind"), patch_note=(p or {}).get("note"), patch_ref=(p or {}).get("ref"),
               logic_060=None, common=u"spec_patch_060.md §A 표 적용(PlayerState +0xd0 · Blackboard 0x5c8 · 태그 SmallActionPlay/BattleSubPlanGoal/SubPlan · Effect vt · TeamPlan/LPH/BattlePlan 오프셋)",
               version_gate=None, note=(j.get("how") or "")[:120])
    sp2 = dict(sp); sp2["v060"] = blk; out.append(sp2)
D2 = dict(D); D2["specs"] = out; D2["meta_v060"] = dict(base="specs20_v3 (0.5.8)", made="2026-09-17", sources=["mig060_judge", "r19_result", "r20_result", "spec_patch_060"], note=u"logic_060 는 applyspec060 단계에서 채움 · 원본 logic 은 손대지 않음")
json.dump(D2, io.open(os.path.join(HERE, "_spec", "specs20_v060.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
L = [u"# specs20_v060 요약(0.6.0 v060 블록) — %d specs" % len(out), u"", u"판정 분포: " + u" · ".join(u"%s %d" % kv for kv in C.most_common()), u"", u"| i | 구 | 신 | 함수 | 판정 | 패치 |", u"|---|---|---|---|---|---|"]
for sp in out:
    b = sp["v060"]; L.append(u"| %s | `%s` | `%s` | %s | %s | %s |" % (sp.get("i"), b["addr_058"], b["addr"], (sp.get("name") or "")[:40], b["verdict"][:50], (b["patch_kind"] or u"") + ((u" · " + b["patch_note"][:60]) if b["patch_note"] else u"")))
io.open(os.path.join(HERE, "_next", "spec060_summary.md"), "w", encoding="utf-8").write(u"\n".join(L))
print(u"specs20_v060.json %d · " % len(out) + u" · ".join(u"%s %d" % kv for kv in C.most_common()))
