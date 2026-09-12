#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""whohooks.py — **배포된 모드 dll 중 어느 것이 20함수 RVA 를 들고 있나.** (2026-09-12 신설)

## 왜 필요한가
`judge_verify` 의 카운트 프로브는 진입부가 이미 `48 b8`(외부 훅)이면 **건너뛴다** —
07-18 에 두 모드가 서로를 재체인해 **게임이 먹통**이 된 실사고 때문이다(CLAUDE.md §3).
⟹ 우리 20함수를 다른 모드가 후킹하고 있으면 **그 함수는 발화수를 못 잰다.**

그걸 게임을 켜 보고 알면 한 판(≈2분 + 셋업)을 버린다. **정적으로 미리 좁힌다.**

## 판정 방식과 그 한계 (★이게 무엇을 증명하고 무엇을 증명하지 않는가)
모드가 RVA 를 하드코딩하면 그 값이 dll 안에 **리터럴로** 박힌다. 그걸 찾는다.
- `u32` 리틀엔디언(`c0 54 d3 00`) · `u64` 리틀엔디언 · ASCII 표기(`0xd354c0`·`d354c0`) 넷 다 본다.
- ⚠**히트 = 후킹 확정이 아니다.** 4바이트 우연 일치가 있고, RVA 를 「참조만」 하는 모드도 있다.
- ⚠**무히트 = 안전 확정도 아니다.** 심볼·지문으로 **런타임에 주소를 찾는** 모드는 리터럴이 없다
  (이 프로젝트의 `name2rva`·지문 매칭 방식이 그렇다).
⟹ 이건 **후보를 좁히는 도구**다. 최종 판정은 프로브의 「이미 훅됨」 보고다.
   판정 어휘로 쓰면 `재료 부재`(범위 = 리터럴 스캔) 이다.

사용: python -X utf8 MIG\\whohooks.py
"""
import io
import json
import os
import re
import struct
import sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")

MODS = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods"
TBL = r"C:\tfm2mods\tfm2_judge_verify\src\probe20_tbl.rs"


def targets():
    u"""프로브 표에서 (idx, rva, name) 을 읽는다. 표가 정본이다(여기 베끼지 않는다)."""
    t = io.open(TBL, encoding="utf-8").read()
    out = []
    for m in re.finditer(r"idx:\s*(\d+),\s*rva:\s*0x([0-9a-fA-F]+),.*?name:\s*\"([^\"]*)\"", t):
        out.append((int(m.group(1)), int(m.group(2), 16), m.group(3)))
    return out


def needles(rva):
    u"""그 RVA 가 dll 안에 박혀 있을 수 있는 모든 표기."""
    return [
        (u"u32le", struct.pack("<I", rva)),
        (u"u64le", struct.pack("<Q", rva)),
        (u"ascii0x", ("0x%x" % rva).encode("ascii")),
        (u"ascii", ("%x" % rva).encode("ascii")),
        (u"ABS64", struct.pack("<Q", 0x140000000 + rva)),   # 절대주소로 들고 있을 수도
    ]


def main():
    tg = targets()
    if not tg:
        print(u"⛔프로브 표에서 대상을 못 읽었다: %s" % TBL)
        return
    print(u"대상 %d함수 · mods 폴더 스캔\n" % len(tg))

    dlls = []
    for d in sorted(os.listdir(MODS)):
        p = os.path.join(MODS, d)
        if not os.path.isdir(p):
            continue
        cand = os.path.join(p, d + ".dll")
        if os.path.isfile(cand):
            dlls.append((d, cand))
        else:   # 폴더명≠dll명인 경우(비활성 선례 `_x.disabled` 등)도 본다
            for f in os.listdir(p):
                if f.lower().endswith(".dll"):
                    dlls.append((d, os.path.join(p, f)))
                    break
    print(u"검사 대상 dll %d개\n" % len(dlls))

    hits = {}
    for mod, path in dlls:
        try:
            b = io.open(path, "rb").read()
        except Exception as e:
            print(u"  (읽기 실패) %s — %s" % (mod, e))
            continue
        found = []
        for idx, rva, nm in tg:
            for tag, nd in needles(rva):
                if nd in b:
                    found.append((idx, nm, tag))
                    break
        if found:
            hits[mod] = (path, len(b), found)

    dis = lambda m: m.startswith("_") or ".disabled" in m
    print(u"=" * 96)
    if not hits:
        print(u"★리터럴 히트 0 — 리터럴 스캔 범위에서는 충돌 모드가 없다")
    for mod, (path, sz, found) in sorted(hits.items(), key=lambda x: -len(x[1][2])):
        mark = u"  (이미 비활성)" if dis(mod) else u"  ★활성"
        print(u"\n%s%s  %s B  · 히트 %d함수" % (mod, mark, u"{:,}".format(sz), len(found)))
        for idx, nm, tag in found:
            print(u"     #%02d %-46s [%s]" % (idx, nm[:46], tag))
    print(u"\n" + u"=" * 96)
    act = [m for m in hits if not dis(m)]
    print(u"★활성 상태로 히트한 모드 = %d개: %s" % (len(act), u", ".join(sorted(act)) or u"-"))
    print(u"⚠히트=후킹 확정 아님 · 무히트=안전 확정 아님(지문·심볼 탐색형은 리터럴이 없다).")
    print(u"   최종 판정은 프로브의 「이미 훅됨」 보고다.")


if __name__ == "__main__":
    main()
