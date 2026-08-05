# -*- coding: utf-8 -*-
"""gg 3종 zip 안의 tfm2_meta_item_delegate/mod.mod_info base 의존을 0.5.4 대역으로 올린다.

왜 따로 필요한가: rel_054.py 는 mod_info 를 "게임 mods\\<id>\\mod.mod_info" 에서 가져오는데,
tfm2_meta_item_delegate 는 그 파일이 게임 폴더에 없다(실 로드처가 워크샵 폴더).
그래서 기준 zip(0.5.3) 것이 그대로 실려 base 의존이 ">=0.5.3, <0.5.4" 로 남는다
= 이 zip 을 푼 유저의 모드가 0.5.4 에서 자동 비활성된다.

zip 은 엔트리 교체가 안 되므로 전체를 다시 쓴다(다른 엔트리는 바이트 그대로 복사).
⚠ 게임 폴더(Program Files)는 샌드박스가 삭제를 막으므로 로컬에 만든 뒤 copy 로 덮어쓴다.
⚠ mod_info 는 BOM 없는 UTF-8 이어야 한다.
"""
import os, io, re, sys, shutil, zipfile, json

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

ZIP = (r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2"
       r"\mods\release\0.5.4\팀파매gg모드3종.zip")
OLDRX = r">=\s*0\.5\.3\s*,\s*<\s*0\.5\.4"
NEW = ">=0.5.4, <0.5.5"
TARGET_SUFFIX = "tfm2_meta_item_delegate/mod.mod_info"

zin = zipfile.ZipFile(ZIP)
ents = [i for i in zin.infolist() if not i.filename.endswith("/")]
stage = os.path.join(os.environ["TEMP"], "gg3_054_fix.zip")
if os.path.exists(stage):
    os.remove(stage)

changed = []
with zipfile.ZipFile(stage, "w", zipfile.ZIP_DEFLATED, compresslevel=9) as zout:
    for i in ents:
        data = zin.read(i.filename)
        if i.filename.replace("\\", "/").endswith(TARGET_SUFFIX):
            txt = data.decode("utf-8")
            if re.search(OLDRX, txt):
                txt = re.sub(OLDRX, NEW, txt)
                data = txt.encode("utf-8")          # BOM 없음
                changed.append(i.filename)
        zout.writestr(i.filename, data)

shutil.copy2(stage, ZIP)

zz = zipfile.ZipFile(ZIP)
print(f"{os.path.basename(ZIP)}: {os.path.getsize(ZIP):,}B  엔트리 {len(zz.namelist())} (기준 {len(ents)})")
print(f"수정: {changed if changed else '(없음 - 이미 0.5.4 대역이거나 대상 미발견)'}")
for n in zz.namelist():
    if n.replace("\\", "/").endswith("mod.mod_info"):
        raw = zz.read(n)
        j = json.loads(raw.decode("utf-8"))
        deps = [d.get("version") for d in j.get("dependencies", []) if d.get("mod_id") == "base"]
        bom = raw[:3] == b"\xef\xbb\xbf"
        print(f"  {j.get('mod_id'):<28} v{j.get('version'):<8} base={deps}  BOM={bom}")
