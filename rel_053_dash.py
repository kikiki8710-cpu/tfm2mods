# -*- coding: utf-8 -*-
# rel_053_dash.py — TFM2_Meta_Dashboard 0.5.3 릴리스 zip.
#   기준 = 0.5.2 zip(944엔트리·157,988,106B)을 그대로 재작성하되 **버전임계 3개만 교체**:
#     TFM2_Meta_Dashboard.dll / DashboardApp/.../tools/tfm2_save_probe.exe / mod.mod_info
#   ⚠유저 세이브 파생물(meta-data.js·meta-chunks·save_probe_snapshot·*.log)은 0.5.2 zip 에도 미포함 = 그대로 유지.
import os, sys, io, zipfile, json, shutil, re, time

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
G = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods"
SRC_ZIP = os.path.join(G, "release", "0.5.2", "TFM2_Meta_Dashboard.zip")
OUT_ZIP = os.path.join(G, "release", "0.5.3", "TFM2_Meta_Dashboard.zip")
LIVE = os.path.join(G, "TFM2_Meta_Dashboard")
STAGE = r"C:\Users\dev\AppData\Local\Temp\claude\C--Users-dev-Desktop-claude-tfm2\89fdc8ca-ad45-4a5c-a3de-4f289a646255\scratchpad\dash053.zip"

# zip 내부 경로 → 라이브 상대경로 (이 3개만 신규 배포본으로 교체)
REPLACE = {
    "TFM2_Meta_Dashboard/TFM2_Meta_Dashboard.dll": "TFM2_Meta_Dashboard.dll",
    "TFM2_Meta_Dashboard/DashboardApp/resources/app/tfm2_meta_dashboard/tools/tfm2_save_probe.exe":
        r"DashboardApp\resources\app\tfm2_meta_dashboard\tools\tfm2_save_probe.exe",
    "TFM2_Meta_Dashboard/mod.mod_info": "mod.mod_info",
}
PII = [rb"C:\\Users\\dev", rb"C:/Users/dev"]

z = zipfile.ZipFile(SRC_ZIP)
ents = [i for i in z.infolist() if not i.filename.endswith("/")]
print(f"기준 0.5.2 zip: {len(ents)}엔트리 {os.path.getsize(SRC_ZIP):,}B")

# 교체 대상 존재 확인
for k, v in REPLACE.items():
    p = os.path.join(LIVE, v)
    if not os.path.exists(p):
        raise SystemExit(f"라이브 파일 없음: {p}")
    if k not in {i.filename for i in ents}:
        raise SystemExit(f"기준 zip 에 엔트리 없음: {k}")

if os.path.exists(STAGE):
    os.remove(STAGE)
t0 = time.time()
done = 0
with zipfile.ZipFile(STAGE, "w", zipfile.ZIP_DEFLATED) as out:
    for i in ents:
        if i.filename in REPLACE:
            src = os.path.join(LIVE, REPLACE[i.filename])
            data = open(src, "rb").read()
            done += 1
            print(f"  교체 {i.filename}  {i.file_size:,} → {len(data):,}B")
        else:
            data = z.read(i.filename)
        out.writestr(i.filename, data)
print(f"재작성 {len(ents)}엔트리 (교체 {done}) {time.time()-t0:.0f}s → {os.path.getsize(STAGE):,}B")

# 검증: 엔트리 수·mod_info·PII
zz = zipfile.ZipFile(STAGE)
n = len([i for i in zz.infolist() if not i.filename.endswith("/")])
raw = zz.read("TFM2_Meta_Dashboard/mod.mod_info")
j = json.loads(raw.decode("utf-8"))
pii = []
for i in zz.infolist():
    if i.filename.endswith("/") or i.file_size > 40 * 1024 * 1024:
        continue
    d = zz.read(i.filename)
    if any(re.search(p, d, re.I) for p in PII):
        pii.append(i.filename)
print(f"검증: 엔트리 {n}(기준 {len(ents)}) BOM={'none' if raw[0]==0x7b else 'BAD'} "
      f"ver={j.get('version')} author={j.get('author')} "
      f"deps={[x.get('version') for x in j.get('dependencies', [])]}")
print(f"      PII: {pii[:5] if pii else '0건'}")

os.makedirs(os.path.dirname(OUT_ZIP), exist_ok=True)
shutil.copy2(STAGE, OUT_ZIP)
print(f"배포: {OUT_ZIP}  {os.path.getsize(OUT_ZIP):,}B")
