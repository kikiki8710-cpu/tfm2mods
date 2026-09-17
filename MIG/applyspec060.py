# -*- coding: utf-8 -*-
"""
applyspec060.py — `_next/apply060/out/<old>.md`(에이전트 산출 · 형식 = mkapply060.py 도크스트링) 를 읽어
specs20_v060.json 의 해당 spec `v060` 블록에 logic_060 / changes_060 / verified_060 / confidence_060 를 채운다.
원본 `logic`(0.5.8) 은 손대지 않는다. 형식 위반 파일은 목록으로 보고하고 건너뛴다.
사용: python applyspec060.py            → 병합 + _next/apply060/STATUS.md
"""
import io, json, os, re, sys, glob
sys.stdout.reconfigure(encoding="utf-8")
HERE = os.path.dirname(os.path.abspath(__file__)); OUT = os.path.join(HERE, "_next", "apply060", "out")
VP = os.path.join(HERE, "_spec", "specs20_v060.json"); V = json.load(io.open(VP, encoding="utf-8"))
BY = {sp["v060"]["addr_058"]: sp for sp in V["specs"] if sp["v060"].get("addr_058")}
SEC = re.compile(r"^##\s*(logic_060|changes|verified|confidence)\b.*$", re.M)
def parse(path):
    s = io.open(path, encoding="utf-8").read()
    m = re.match(r"#\s*`?([0-9a-f]{5,7})`?\s*→\s*`?([0-9a-f]{5,8})`?\s*(.*)", s.strip().split("\n")[0])
    if not m: return None, u"헤더 형식(# <old>→<new> <name>) 아님"
    old = m.group(1)
    parts = {}; idx = [(mm.start(), mm.end(), mm.group(1)) for mm in SEC.finditer(s)]
    for k, (st, en, name) in enumerate(idx):
        body = s[en:(idx[k + 1][0] if k + 1 < len(idx) else len(s))].strip()
        if name == "logic_060":  # ``` 펜스 제거
            body = re.sub(r"^```[a-zA-Z]*\n", "", body); body = re.sub(r"\n```\s*$", "", body)
        parts[name] = body
    if "logic_060" not in parts or len(parts["logic_060"]) < 40: return old, u"logic_060 없음/너무 짧음"
    return old, parts
ok, bad, rows = 0, [], []
for path in sorted(glob.glob(os.path.join(OUT, "*.md"))):
    old, parts = parse(path)
    if not isinstance(parts, dict): bad.append((os.path.basename(path), parts)); continue
    sp = BY.get(old)
    if not sp: bad.append((os.path.basename(path), u"specs20_v060 에 없는 구 RVA %s" % old)); continue
    b = sp["v060"]
    b["logic_060"] = parts["logic_060"]; b["changes_060"] = parts.get("changes"); b["verified_060"] = parts.get("verified")
    conf = (parts.get("confidence") or u"").strip(); b["confidence_060"] = conf[:300]
    b["applied_from"] = os.path.basename(path); ok += 1
    rows.append((old, b.get("addr"), sp.get("name"), (conf.split("\n")[0] if conf else u"?")[:60], len(parts["logic_060"])))
json.dump(V, io.open(VP, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
todo = [sp for sp in V["specs"] if sp["v060"].get("addr_058") and not sp["v060"]["verdict"].startswith((u"동치", u"✅동치", u"소멸")) and not sp["v060"].get("logic_060")]
L = [u"# apply060 STATUS — logic_060 병합 현황", u"", u"병합 %d · 형식 오류 %d · 미작성(변경 중) %d" % (ok, len(bad), len(todo)), u"",
     u"| 구 | 신 | 함수 | confidence | logic_060 길이 |", u"|---|---|---|---|---|"]
L += [u"| `%s` | `%s` | %s | %s | %d |" % r for r in rows]
if bad: L += [u"", u"## 형식 오류", u""] + [u"- %s: %s" % b for b in bad]
if todo: L += [u"", u"## 미작성(변경 판정인데 logic_060 없음)", u""] + [u"- `%s` %s (%s)" % (sp["v060"]["addr_058"], sp.get("name"), sp["v060"]["verdict"][:30]) for sp in todo]
io.open(os.path.join(HERE, "_next", "apply060", "STATUS.md"), "w", encoding="utf-8").write(u"\n".join(L))
print(u"병합 %d · 형식 오류 %d · 미작성 %d" % (ok, len(bad), len(todo)))
for b in bad: print(u"  ✗ %s: %s" % b)
