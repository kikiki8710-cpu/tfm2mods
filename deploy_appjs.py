# -*- coding: utf-8 -*-
# deploy_appjs.py — 대시보드 프론트 app.js 를 3곳에 배포.
#   ①게임 설치 DashboardApp ②kit payload(워크샵/apply.ps1 원본) ③0.5.3 릴리스 zip 내부 엔트리
import os, sys, io, zipfile, shutil, hashlib, time

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
SRC = r"C:\Users\dev\Downloads\app\app.js"
G = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods"
LIVE = os.path.join(G, "TFM2_Meta_Dashboard", "DashboardApp", "resources", "app",
                    "tfm2_meta_dashboard", "app.js")
PAYLOAD = r"C:\tfm2mods\tfm2gg_050hf2_merge\payload\tfm2_meta_dashboard\app.js"
ZIP = os.path.join(G, "release", "0.5.3", "TFM2_Meta_Dashboard.zip")
ZIP_ENTRY = "TFM2_Meta_Dashboard/DashboardApp/resources/app/tfm2_meta_dashboard/app.js"
STAGE = r"C:\Users\dev\AppData\Local\Temp\claude\C--Users-dev-Desktop-claude-tfm2\89fdc8ca-ad45-4a5c-a3de-4f289a646255\scratchpad\dash053_appjs.zip"

new = open(SRC, "rb").read()
h = hashlib.sha256(new).hexdigest()[:16]
print(f"소스: {SRC}  {len(new):,}B  sha256[:16]={h}")

# ① 게임 설치 / ② payload
for tag, dst in (("게임설치", LIVE), ("payload", PAYLOAD)):
    old = os.path.getsize(dst) if os.path.exists(dst) else 0
    if os.path.exists(dst):
        shutil.copy2(dst, dst + ".bak_20260730_pre_lane")
    open(dst, "wb").write(new)
    st = os.stat(dst)
    ok = hashlib.sha256(open(dst, "rb").read()).hexdigest()[:16] == h
    print(f"  [{tag}] {old:,} → {st.st_size:,}B  {time.strftime('%m-%d %H:%M:%S', time.localtime(st.st_mtime))}  "
          f"해시일치={'OK' if ok else '★FAIL'}")

# ③ 릴리스 zip 내부 엔트리 교체 (전체 재작성)
z = zipfile.ZipFile(ZIP)
ents = [i for i in z.infolist() if not i.filename.endswith("/")]
assert ZIP_ENTRY in {i.filename for i in ents}, "zip 에 app.js 엔트리 없음"
if os.path.exists(STAGE):
    os.remove(STAGE)
t0 = time.time()
with zipfile.ZipFile(STAGE, "w", zipfile.ZIP_DEFLATED) as out:
    for i in ents:
        out.writestr(i.filename, new if i.filename == ZIP_ENTRY else z.read(i.filename))
z.close()
shutil.copy2(STAGE, ZIP)
zz = zipfile.ZipFile(ZIP)
n = len([i for i in zz.infolist() if not i.filename.endswith("/")])
zh = hashlib.sha256(zz.read(ZIP_ENTRY)).hexdigest()[:16]
print(f"  [릴리스zip] {os.path.getsize(ZIP):,}B  엔트리 {n}(기준 {len(ents)})  "
      f"app.js 해시일치={'OK' if zh == h else '★FAIL'}  {time.time()-t0:.0f}s")
