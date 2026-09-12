# -*- coding: utf-8 -*-
u"""show — 패치의 전/후를 한 줄씩 보여준다(정본 불변)."""
import io, json, os, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
pj = json.load(io.open(os.path.join(HERE, "patch.json"), encoding="utf-8"))
for n, e in enumerate(pj["errors"], 1):
    print(u"\n─── [%02d] %s" % (n, e["path"]))
    print(u"  전: %s" % e["old"].replace(u"\n", u"⏎"))
    print(u"  후: %s" % (e["new"].replace(u"\n", u"⏎") or u"(삭제)"))
