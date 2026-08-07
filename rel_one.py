# -*- coding: utf-8 -*-
# rel_one.py <MOD_ID> <기준버전> — 단일 모드 릴리스 zip 재생성(라이브 dll·mod_info 반영).
#   ⚠다른 세션이 dll 을 재배포하면 zip 이 구본이 된다 ⟹ rel_verify.py 로 탐지 후 이걸로 갱신.
import os, sys, io, zipfile, json, shutil, hashlib

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
G = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods"
EXC_SUF = (".bak", ".pdb", ".exp", ".lib", ".log", ".old")
EXC_SUB = ("_log.txt", "_crash", "debug.txt", ".bak_")

mod, base = sys.argv[1], sys.argv[2]
src = os.path.join(G, "release", base, mod + ".zip")
# 출력 버전 = argv[3](생략 시 기준버전과 동일). 구판은 0.5.3 하드코딩이라 패치 후 스테일 지뢰였다(08-08 수정).
out_ver = sys.argv[3] if len(sys.argv) > 3 else base
out = os.path.join(G, "release", out_ver, mod + ".zip")
os.makedirs(os.path.dirname(out), exist_ok=True)
live = os.path.join(G, mod)
z = zipfile.ZipFile(src)
ents = [i for i in z.infolist() if not i.filename.endswith("/")]
stage = os.path.join(os.environ["TEMP"], mod + "_rel.zip")
if os.path.exists(stage):
    os.remove(stage)
rep = []
with zipfile.ZipFile(stage, "w", zipfile.ZIP_DEFLATED, compresslevel=9) as o:
    for i in ents:
        rel = i.filename.split("/", 1)[1] if "/" in i.filename else i.filename
        low = rel.lower()
        if low.endswith(EXC_SUF) or any(x in low for x in EXC_SUB):
            continue
        p = os.path.join(live, *rel.split("/"))
        # dll/exe/mod_info 만 라이브 반영, cfg·자산은 기준 zip 유지(유저 테스트값 유출 방지)
        if os.path.exists(p) and (low.endswith((".dll", ".exe")) or rel == "mod.mod_info"):
            data = open(p, "rb").read()
            if hashlib.sha256(data).hexdigest() != hashlib.sha256(z.read(i.filename)).hexdigest():
                rep.append((rel, i.file_size, len(data)))
        else:
            data = z.read(i.filename)
        o.writestr(i.filename, data)
shutil.copy2(stage, out)
zz = zipfile.ZipFile(out)
raw = zz.read(f"{mod}/mod.mod_info") if f"{mod}/mod.mod_info" in zz.namelist() else b"{}"
j = json.loads(raw.decode("utf-8")) if raw != b"{}" else {}
for r, a, b2 in rep:
    print(f"  교체 {r:34s} {a:>10,} → {b2:>10,}B")
print(f"{mod}: {os.path.getsize(out):,}B 엔트리 {len(zz.namelist())}(기준 {len(ents)}) "
      f"v{j.get('version')} deps={[x.get('version') for x in j.get('dependencies', [])]}")
