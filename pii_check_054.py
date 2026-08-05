# -*- coding: utf-8 -*-
"""pii_check_054.py — 0.5.4 릴리스 zip 전수 PII 스캔.

기존 `pii_check.py` 의 알려진 결함을 메운다:
  ⚠ 그 스크립트는 **로컬 경로(C:\\Users\\dev)만** 검사해서 `mod.mod_info` 의
    author 필드에 박힌 개인 핸들(kikiki8710)을 놓쳤다(2026-07-31 발각).
  ⟹ 여기서는 경로 + 개인 핸들 + 이메일 + 세이브 절대경로를 함께 본다.
     (gg 세트의 *_policy.tsv 주석에 유저 세이브 절대경로가 박혀 있던 전례가 있다.)

바이너리(dll/exe)도 검사한다 — dll 의 PII 는 rustc 가 panic location 으로 박는
소스 절대경로다(.pdb 문자열은 파일명뿐이라 PII 아님).

사용: python pii_check_054.py
"""
import os, io, re, sys, zipfile

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

REL = (r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2"
       r"\mods\release\0.5.4")

# (라벨, 바이트 패턴) — 대소문자 무시
#
# ⚠ 2026-08-05 정밀화: 처음엔 `@gmail\.com` 과 `AppData\Roaming\TeamSamoyed` 를 그대로 넣었더니
#   TFM2_Meta_Dashboard.zip 에서 28건이 잡혔는데 **전부 오탐**이었다:
#     - Chromium/Node/Python 라이선스·표준라이브러리에 박힌 **upstream 저자 이메일**
#     - `refresh_meta_dashboard.ps1` 의 `Join-Path $env:USERPROFILE "AppData\Roaming\TeamSamoyed\..."`
#       = 하드코딩 경로가 아니라 **환경변수 조립** (오히려 올바른 코드)
#   ⟹ 이메일은 이 유저 것만, 세이브경로는 드라이브문자로 시작하는 절대경로만 잡는다.
NEEDLES = [
    ("로컬경로",   rb"C:\\Users\\dev"),
    ("로컬경로",   rb"C:/Users/dev"),
    ("개인핸들",   rb"kikiki8710"),
    ("이메일",     rb"user@example\.com"),
    ("세이브절대경로", rb"[A-Za-z]:\\Users\\[^\\\r\n\"]+\\AppData\\Roaming\\TeamSamoyed"),
    ("유저명",     rb"\\dev\\"),
]

# 서드파티 번들 런타임 — 우리 산출물이 아니라 upstream 배포본이라 저자 정보가 들어 있는 게 정상.
SKIP_PATH_PARTS = ("/runtime/python/", "/licenses", "licenses.chromium")

total_hits = 0
for fn in sorted(os.listdir(REL)):
    if not fn.lower().endswith(".zip"):
        continue
    path = os.path.join(REL, fn)
    z = zipfile.ZipFile(path)
    hits = []
    skipped_vendor = 0
    for i in z.infolist():
        if i.filename.endswith("/"):
            continue
        low = i.filename.replace("\\", "/").lower()
        if any(part in low for part in SKIP_PATH_PARTS):
            skipped_vendor += 1
            continue
        data = z.read(i.filename)
        for label, pat in NEEDLES:
            for m in re.finditer(pat, data, re.IGNORECASE):
                s = max(0, m.start() - 40)
                ctx = data[s:m.end() + 40]
                ctx = ctx.decode("utf-8", "replace").replace("\n", " ").replace("\r", " ")
                hits.append((i.filename, label, ctx.strip()))
    total_hits += len(hits)
    mark = "PII 0" if not hits else f"!! PII {len(hits)}건"
    print(f"{fn:<34} 엔트리 {len(z.namelist()):>3}  {os.path.getsize(path):>12,}B  {mark}")
    seen = set()
    for f, label, ctx in hits:
        key = (f, label)
        if key in seen:
            continue
        seen.add(key)
        print(f"    [{label}] {f}")
        print(f"        ...{ctx}...")

print(f"\n총 PII 히트 {total_hits}건")
