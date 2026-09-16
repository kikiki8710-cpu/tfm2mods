# -*- coding: utf-8 -*-
"""
mig060_fix.py — mig060_same 이 호출부 실측으로 잡은 「mig060 짝 오류」를 mig060_judge.{json,md} 에 정정형으로 반영.
채택 기준: 구 함수 skel == 신 후보 skel 이고 크기 동일(강한 근거)인 것만. 그 외(ICF 로 접힌 동형 함수 · 디스패처 블록 오정렬)는 기록만.
사용: python mig060_fix.py [mig060_same_fix.json 경로]
"""
import io, json, os, sys, pickle
sys.stdout.reconfigure(encoding="utf-8")
HERE = os.path.dirname(os.path.abspath(__file__))
FIX = sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "_next", "mig060_same_fix.json")
FO = pickle.load(open(r"C:\tfm2mods\_fnidx_058.pkl", "rb"))["idx"]; FN = pickle.load(open(r"C:\tfm2mods\_fnidx_060.pkl", "rb"))["idx"]
J = json.load(io.open(os.path.join(HERE, "_next", "mig060_judge.json"), encoding="utf-8"))
JM = {r["old"]: r for r in J}
md = io.open(os.path.join(HERE, "_next", "mig060_judge.md"), encoding="utf-8").read()
applied = []; skipped = []
for f in json.load(io.open(FIX, encoding="utf-8")):
    o, w, n = int(f["old"], 16), int(f["was"], 16), int(f["now"], 16)
    fo, fw, fn = FO.get(o, {}), FN.get(w, {}), FN.get(n, {})
    strong = fo.get("skel") and fo["skel"] == fn.get("skel") and fo.get("size") == fn.get("size") and fo["skel"] != fw.get("skel")
    if not strong: skipped.append(u"%s %s→%s(%s)" % (f["old"], f["was"], f["now"], u"둘 다 skel 동일·ICF" if fo.get("skel") == fw.get("skel") == fn.get("skel") else u"근거 약")); continue
    r = JM[f["old"]]
    old_line = None
    for line in md.split(u"\n"):
        if line.startswith(u"| `%s` | `%s` |" % (f["old"], f["was"])): old_line = line; break
    r["new_was"] = f["was"]; r["new"] = f["now"]; r["verdict_was"] = r["verdict"]; r["verdict"] = u"동일(skel)"; r["conf"] = "B(호출부)"
    r["how"] = u"정정(mig060_same 호출부 실측 · skel 동일·크기 동일) ← 구 %s %s" % (f["was"], r["how"])
    r["size"] = [fo["size"], fn["size"]]; r["ninsn"] = [fo["ninsn"], fn["ninsn"]]
    if old_line:
        new_line = u"| `%s` | ~~`%s`~~→`%s` | %s | %s | %s | ~~%s~~→**동일(skel)** | %d→%d | %d→%d | | | | | | %s |" % (
            f["old"], f["was"], f["now"], r["name"][:44], r["src"], r["conf"], r["verdict_was"], fo["size"], fn["size"], fo["ninsn"], fn["ninsn"], r["how"])
        md = md.replace(old_line, new_line)
    applied.append(u"%s %s→%s %s" % (f["old"], f["was"], f["now"], r["name"][:30]))
json.dump(J, io.open(os.path.join(HERE, "_next", "mig060_judge.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
io.open(os.path.join(HERE, "_next", "mig060_judge.md"), "w", encoding="utf-8").write(md)
print(u"정정 적용 %d: " % len(applied) + u" · ".join(applied))
print(u"보류 %d: " % len(skipped) + u" · ".join(skipped))
