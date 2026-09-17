# -*- coding: utf-8 -*-
"""chkapply060.py — apply060/out/*.md 오염 검사(제어문자 · 탭 · BOM · 섹션 유무 · 백슬래시 시퀀스 흔적)"""
import io, glob, os, sys, re
sys.stdout.reconfigure(encoding="utf-8")
HERE = os.path.dirname(os.path.abspath(__file__))
bad = 0
for p in sorted(glob.glob(os.path.join(HERE, "_next", "apply060", "out", "*.md"))):
    s = io.open(p, encoding="utf-8").read()
    ctl = [(i, repr(c)) for i, c in enumerate(s) if ord(c) < 32 and c not in "\n\r\t"]
    tabs = s.count("\t"); bom = s.startswith("﻿")
    secs = [k for k in ("logic_060", "changes", "verified", "confidence") if re.search(r"^##\s*" + k, s, re.M)]
    # heredoc 오염 흔적: 탭이 코드 들여쓰기가 아닌 곳(경로 안 `C:<TAB>`) 에 있거나 제어문자
    susp = re.findall(r"[A-Za-z]:\t|\x0c|\x08|\x07", s)
    flag = ctl or bom or len(secs) < 4 or susp
    bad += bool(flag)
    print(u"%s %6d ctl=%d tabs=%d bom=%s secs=%d %s" % (os.path.basename(p), len(s), len(ctl), tabs, bom, len(secs), (u"⚠ " + str(ctl[:2]) + str(susp[:2])) if flag else u"ok"))
print(u"의심 %d" % bad)
