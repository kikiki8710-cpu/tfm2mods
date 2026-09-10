# -*- coding: utf-8 -*-
u"""losscheck — v2 -> v3 재구성에서 **빠진 정보가 없는지** 전수 대조. (2026-09-11)

방법: v2 각 함수의 모든 문자열 리프를 모아, 같은 함수의 v3 직렬화 텍스트에 부분문자열로
들어 있는지 본다. 없으면 손실 후보로 보고한다(정정으로 문구가 바뀐 것은 정상 — 그건 취소선/신문구로 남는다).
"""
import io, json, os, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
V2 = json.load(io.open(os.path.join(HERE, "specs20.json"), encoding="utf-8"))
V3 = json.load(io.open(os.path.join(HERE, "specs20_v3.json"), encoding="utf-8"))

def leaves(o, out, path=""):
    if isinstance(o, dict):
        for k, v in o.items(): leaves(v, out, path + "/" + str(k))
    elif isinstance(o, list):
        for i, v in enumerate(o): leaves(v, out, path + "[%d]" % i)
    elif isinstance(o, str):
        if len(o.strip()) >= 8: out.append((path, o))
    elif o is not None:
        out.append((path, str(o)))
    return out

miss_tot = 0
for i, (a, b) in enumerate(zip(V2["specs"], V3["specs"])):
    blob = json.dumps(b, ensure_ascii=False)
    # 이스케이프 차이로 인한 오탐 제거: 직렬화 잡음을 벗긴 사본으로도 대조한다
    norm = blob.replace(chr(92) + chr(92), chr(92)).replace(chr(92) + chr(34), chr(34)).replace(chr(92) + "n", chr(10))
    ls = leaves(a, [])
    miss = [(p, t) for p, t in ls if (t not in blob and t not in norm)]
    # 여러 줄 문자열은 줄 단위로 재확인(직렬화 시 개행 이스케이프 차이)
    real = []
    for p, t in miss:
        parts = [x.strip() for x in t.split("\n") if len(x.strip()) >= 12]
        if parts and all(x in norm or x in blob for x in parts):
            continue
        real.append((p, t))
    miss_tot += len(real)
    flag = "OK " if not real else "★손실?"
    print(u"%s [%02d] %-40s 리프 %3d / 미발견 %d" % (flag, i, (a.get("name") or "")[:40], len(ls), len(real)))
    for p, t in real[:6]:
        print(u"       %s" % p)
        print(u"         %s" % t[:160].replace("\n", " "))
# shared 도 대조
sa = json.dumps(V2.get("shared"), ensure_ascii=False, sort_keys=True)
sb = json.dumps(V3.get("shared"), ensure_ascii=False, sort_keys=True)
print(u"\nshared 동일: %s" % (sa == sb))
print(u"corrections v2=%d v3=%d" % (len(V2["meta"].get("corrections") or []),
                                    len(V3["meta"].get("corrections") or [])))
print(u"\n총 미발견 %d건" % miss_tot)
