# -*- coding: utf-8 -*-
u"""dossierfresh — 배치가 **자기가 읽은 지시문이 최신인지** 스스로 확인한다. (7차 배치C 적발로 신설)

## 왜 필요한가 — 신선도 계약이 자기 자신을 안 봤다
도시에 §0 은 `_spec/specs20_v3.json` 해시만 확인시켰다. 그런데 **지시문 본문도 라운드 중에 바뀐다** —
한 배치가 내 지시 오류를 보고하면 메인이 그 자리에서 고치기 때문이다.

7차 실측: 배치 C 가 받은 판은 17:41 생성, 디스크 현재는 17:58 이었다.
**`_spec` 해시는 일치**하는데 지시문이 네 곳 달랐고, 그중 하나가 `patch.json` **스키마**라
**옛 판대로 냈으면 0건 적용**이었다(이 라운드가 막으려던 「5차 317행 유실」과 같은 형태).

⟹ 「정본이 최신인가」와 「**내가 든 지시가 최신인가**」는 다른 물음이다. 둘 다 물어야 한다.

사용:
  python -X utf8 dossierfresh.py 7 C      → 배치 C 의 도시에·분책·정본 신선도
  python -X utf8 dossierfresh.py 7        → 네 배치 전부
종료코드: 0=FRESH · 1=STALE(작업을 멈추고 다시 읽어라) · 2=STAMP 없음
"""
import hashlib
import io
import json
import os
import sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))


def sha16(p):
    try:
        return hashlib.sha256(io.open(p, "rb").read()).hexdigest()[:16]
    except Exception:
        return None


def check(rnd, tag):
    d = os.path.join(HERE, "_verify%s" % rnd)
    stamp = os.path.join(d, "STAMP.json")
    if not os.path.exists(stamp):
        print(u"[%s] STAMP.json 이 없다 — `mkdossier.py %s` 를 먼저 돌려라." % (tag, rnd))
        return 2
    st = json.load(io.open(stamp, encoding="utf-8")).get(tag)
    if not st:
        print(u"[%s] 그 배치의 도장이 없다." % tag)
        return 2

    bad = []
    dos = os.path.join(d, "DOSSIER_%s.md" % tag)
    now = sha16(dos)
    if now != st["dossier"]:
        bad.append((u"지시문 `DOSSIER_%s.md`" % tag, st["dossier"], now))
    for fn, want in (st.get("spec_hashes") or {}).items():
        got = sha16(os.path.join(d, tag, fn))
        if got != want:
            bad.append((u"분책 `%s`" % fn, want, got))
    v3 = sha16(os.path.join(HERE, "_spec", "specs20_v3.json"))
    if v3 != st["v3"]:
        bad.append((u"정본 `specs20_v3.json`", st["v3"], v3))
    mk = sha16(os.path.join(HERE, "mkdossier.py"))
    if mk != st["mkdossier"]:
        bad.append((u"생성기 `mkdossier.py`(지시 형식이 바뀌었다)", st["mkdossier"], mk))

    if not bad:
        print(u"[%s] **FRESH** — 도장 %s 기준으로 지시문·분책·정본이 전부 그대로다."
              % (tag, st.get("built")))
        return 0
    print(u"[%s] ★**STALE — 작업을 멈추고 다시 읽어라.**" % tag)
    for what, was, now2 in bad:
        print(u"     %-44s 도장 %s → 현재 %s" % (what, was, now2))
    print(u"     ⟹ `DOSSIER_%s.md` 와 `_verify%s/%s/spec_*.md` 를 다시 Read 하라." % (tag, rnd, tag))
    print(u"     ⚠특히 **§5 보고 형식**이 바뀌었을 수 있다 — 그대로 내면 0건 적용이다.")
    return 1


def main():
    rnd = sys.argv[1] if len(sys.argv) > 1 else "8"
    tags = [sys.argv[2].upper()] if len(sys.argv) > 2 else ["A", "B", "C", "D"]
    rc = 0
    for t in tags:
        rc = max(rc, check(rnd, t))
    return rc


if __name__ == "__main__":
    sys.exit(main())
