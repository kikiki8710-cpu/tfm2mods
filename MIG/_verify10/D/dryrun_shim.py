# -*- coding: utf-8 -*-
u"""`applypatch.py` 가 2026-09-11 21:46 수정으로 `NameError: WARN` 을 내게 됐다(insert 를 쓰면 즉사).
MIG 루트 도구는 수정 금지라, **모듈 네임스페이스에 `WARN` 만 주입**해 --dry 를 돌려 검증한다.
(이 파일은 배치 폴더 안에만 있고 루트 도구를 건드리지 않는다.)"""
import os, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))
import applypatch as AP  # noqa: E402

if not hasattr(AP, "WARN"):
    AP.WARN = []
sys.argv = ["applypatch.py", "10", "--only", "D", "--dry"]
rc = AP.main()
if AP.WARN:
    print(u"\n[shim] 삽입 경고 %d 건(실패 아님):" % len(AP.WARN))
    for w in AP.WARN:
        print(u"   ", u" | ".join(unicode(x) if str is bytes else str(x) for x in w))
sys.exit(rc or 0)
