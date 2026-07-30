# -*- coding: utf-8 -*-
# rel_053_aiadj.py — tfm2_ai_adjust 0.5.3 릴리스 zip.
#   기준 = 0.5.2 zip(20엔트리). 교체:
#     · dll / mod.mod_info / 설정편집기.exe / config/default.txt = 라이브(최신 빌드·기본값 정본)
#     · tfm2_ai_adjust.cfg = **config/default.txt 로 정규화** (라이브 cfg 는 유저 튜닝+`log = 1` 진단 상태)
#     · 나머지(프리셋 cfg·ui_inject·players) = 0.5.2 zip 유지
import os, sys, io, zipfile, json, shutil, re, hashlib

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
G = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods"
SRC_ZIP = os.path.join(G, "release", "0.5.2", "tfm2_ai_adjust.zip")
OUT_ZIP = os.path.join(G, "release", "0.5.3", "tfm2_ai_adjust.zip")
LIVE = os.path.join(G, "tfm2_ai_adjust")
STAGE = r"C:\Users\dev\AppData\Local\Temp\claude\C--Users-dev-Desktop-claude-tfm2\89fdc8ca-ad45-4a5c-a3de-4f289a646255\scratchpad\aiadj053.zip"

FROM_LIVE = {"tfm2_ai_adjust.dll", "mod.mod_info", "설정편집기.exe", "config/default.txt"}
CFG = "tfm2_ai_adjust.cfg"
DEBUG_RE = re.compile(r"^\s*(log|detlog|debug|dump|verbose|trace)\s*=\s*([1-9])", re.I | re.M)

z = zipfile.ZipFile(SRC_ZIP)
ents = [i for i in z.infolist() if not i.filename.endswith("/")]
print(f"기준 0.5.2 zip {len(ents)}엔트리 {os.path.getsize(SRC_ZIP):,}B")

# 0.5.2 릴리스본 cfg 가 클린했는지 참고 확인
old_cfg = z.read(f"tfm2_ai_adjust/{CFG}").decode("utf-8", "replace")
m = DEBUG_RE.search(old_cfg)
print(f"  (참고) 0.5.2 릴리스 cfg 디버그 노브: {'★' + m.group(0).strip() if m else '없음(클린)'}")

default_txt = open(os.path.join(LIVE, "config", "default.txt"), "rb").read()
live_cfg = open(os.path.join(LIVE, CFG), "rb").read()
m2 = DEBUG_RE.search(live_cfg.decode("utf-8", "replace"))
print(f"  라이브 cfg 디버그 노브: {'★' + m2.group(0).strip() if m2 else '없음'}  → 릴리스엔 default.txt 사용")
m3 = DEBUG_RE.search(default_txt.decode("utf-8", "replace"))
assert not m3, f"default.txt 에 디버그 노브 켜짐: {m3.group(0)}"

if os.path.exists(STAGE):
    os.remove(STAGE)
rep = []
with zipfile.ZipFile(STAGE, "w", zipfile.ZIP_DEFLATED) as out:
    for i in ents:
        rel = i.filename.split("/", 1)[1]
        if rel == CFG:
            data = default_txt
            rep.append((rel, i.file_size, len(data), "default.txt 로 정규화"))
        elif rel in FROM_LIVE:
            data = open(os.path.join(LIVE, *rel.split("/")), "rb").read()
            rep.append((rel, i.file_size, len(data), "라이브"))
        else:
            data = z.read(i.filename)
        out.writestr(i.filename, data)
for rel, a, b, why in rep:
    print(f"  교체 {rel:24s} {a:>10,} → {b:>10,}B  ({why})")

zz = zipfile.ZipFile(OUT_ZIP if False else STAGE)
n = len([i for i in zz.infolist() if not i.filename.endswith("/")])
raw = zz.read("tfm2_ai_adjust/mod.mod_info")
j = json.loads(raw.decode("utf-8"))
cfg_in = zz.read(f"tfm2_ai_adjust/{CFG}").decode("utf-8", "replace")
m4 = DEBUG_RE.search(cfg_in)
pii = [i.filename for i in zz.infolist()
       if not i.filename.endswith("/") and i.file_size < 40 * 1024 * 1024
       and re.search(rb"C:\\Users\\dev", zz.read(i.filename), re.I)]
print(f"\n검증: 엔트리 {n}(기준 {len(ents)}) BOM={'none' if raw[0]==0x7b else 'BAD'} "
      f"v{j.get('version')} author={j.get('author')} deps={[x.get('version') for x in j.get('dependencies', [])]}")
print(f"      zip 내 cfg 디버그 노브: {'★' + m4.group(0) if m4 else '없음(클린)'}   PII: {pii[:3] if pii else '0건'}")

os.makedirs(os.path.dirname(OUT_ZIP), exist_ok=True)
shutil.copy2(STAGE, OUT_ZIP)
print(f"배포: {OUT_ZIP}  {os.path.getsize(OUT_ZIP):,}B")
