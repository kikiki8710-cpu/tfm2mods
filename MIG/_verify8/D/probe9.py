# -*- coding: utf-8 -*-
u"""probe9 — 규칙 C 를 조인다: `was` 의 **수치 리터럴이 그 행의 `value` 필드와 같을 때만** 후보.

근거: 손검증에서 C(느슨판) 11건 중 10건이 오탐이었고, 오탐 전건의 공통점은
      「앵커가 일반 토큰 하나뿐이고 **폐기된 그 값 자체는 행에 없다**」였다.
      폐기 선언은 거의 항상 특정 값(60 · 태그 · 오프셋)에 대한 것이므로
      그 값이 행의 `value` 칸에 있는지를 필수 조건으로 걸면 앵커가 확정된다.
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.abspath(os.path.join(HERE, "..", ".."))
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
S = D["specs"]
STRIKE = re.compile(r"~~(?!~).+?~~")
RETRACT = re.compile(u"(노브가 아니다|노브가 아님|존재하지 않는다|소비처가 존재하지 않|"
                     u"소비처 0건|소비처가 없다|항목 삭제)")
TRACE = re.compile(u"(노브가 아니|아니다|아님|존재하지 않|0건|history 참조|소비처|삭제|"
                   u"무영향|변경하지 않|영향 없|진단용|계측|죽은)")
NUM = re.compile(r"(?<![\w.])(\d{1,12})(?![\w.])")
RARE = re.compile(r"\b([A-Za-z_][A-Za-z0-9_]{5,45})\b")


def live(x):
    return STRIKE.sub(u" ", json.dumps(x, ensure_ascii=False))


n = 0
for i, sp in enumerate(S):
    for hi, h in enumerate(sp.get("history") or []):
        now = live({k: v for k, v in h.items() if k != "was"})
        if not RETRACT.search(now):
            continue
        was = h.get("was") or u""
        nums = set(NUM.findall(was))
        anc = set(RARE.findall(was))
        if not nums or not anc:
            continue
        for f in ("knobs", "consts", "mem"):
            for j, x in enumerate(sp.get(f) or []):
                if not isinstance(x, dict):
                    continue
                v = x.get("value")
                if v is None:
                    continue
                vs = str(v)
                if not (nums & set(NUM.findall(vs))):
                    continue
                b = live(x)
                if not (anc & set(RARE.findall(b))):
                    continue
                if TRACE.search(b):
                    continue
                n += 1
                print(u"[%02d] h[%d] %s[%d] value=%s  | was=%s" % (i, hi, f, j, vs[:40], was[:80]))
print(u"엄격 C 후보 %d건" % n)
