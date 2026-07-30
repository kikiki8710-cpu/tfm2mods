# -*- coding: utf-8 -*-
# pii_editor.py — remap 재빌드한 설정편집기의 PII 제거 확인.
import re, sys, io, os

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
PAT = re.compile(rb"[A-Za-z]:\\Users\\dev", re.I)
NEW = r"C:\tfm2mods\ai_adjust_editor\target\release\ai_adjust_editor.exe"
OLD = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_ai_adjust\설정편집기.exe"
for tag, p in (("remap 재빌드", NEW), ("기존 배포본", OLD)):
    d = open(p, "rb").read()
    hits = list(PAT.finditer(d))
    print(f"{tag}: {os.path.getsize(p):,}B  PII {len(hits)}건")
    for m in hits[:3]:
        print("     …" + d[m.start() - 20:m.end() + 90].decode("utf-8", "replace").replace("\x00", "")[:130])
    # remap 결과 확인
    for probe in (b"/cargo/registry", b"/src/"):
        c = len(re.findall(re.escape(probe), d))
        if c:
            print(f"     remap 흔적 {probe.decode()} ×{c}")
