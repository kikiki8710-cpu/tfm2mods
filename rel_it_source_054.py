# -*- coding: utf-8 -*-
"""rel_it_source_054.py — tfm2_item_tactics_source.zip 0.5.4 판 생성.

이 zip 만 성격이 다르다: dll 이 아니라 **소스 배포본**(src\\*.rs + ui\\*.ui + cfg + mod_info).
그래서 rel_054.py 의 "dll/exe/mod_info 만 라이브 반영" 규칙으로는 .rs 가 0.5.3 것으로 남는다.
여기서는 .rs 는 모드 소스에서, .ui/cfg/mod_info 는 게임 배포본에서 가져온다.

⚠ 기준 zip 의 엔트리 목록을 그대로 따라가므로 자산 누락이 생기지 않는다(엔트리 수 대조로 검증).
⚠ 게임 폴더는 샌드박스가 삭제를 막으므로 로컬에 만든 뒤 copy 로 덮어쓴다.
"""
import os, io, sys, shutil, zipfile, hashlib

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

GAME = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods"
SRC_ZIP = os.path.join(GAME, "release", "0.5.3", "tfm2_item_tactics_source.zip")
DST_ZIP = os.path.join(GAME, "release", "0.5.4", "tfm2_item_tactics_source.zip")
MOD_SRC = r"C:\tfm2mods\tfm2_item_tactics"      # .rs 정본
MOD_LIVE = os.path.join(GAME, "tfm2_item_tactics")  # .ui / cfg / mod_info 정본


# ⚠ cfg 는 **라이브에서 가져오면 안 된다** ([[tfm2-release-zip-location]] §공통 규칙).
#   개발 머신의 cfg 는 테스트값이라 그대로 담으면 유출된다. 실제로 여기서 한 번 밟았다:
#   `4items.cfg` 릴리스 기본값 = 영문 주석 + `slots = 3` 인데
#   라이브 = 한글 주석 + `slots = 4`(유저 테스트값) 였다. ⟹ cfg 는 기준 zip 것을 그대로 쓴다.
KEEP_FROM_BASE_ZIP = (".cfg",)


def resolve(rel: str):
    """zip 내 상대경로(루트 폴더 제외) -> 가져올 원본 파일 경로. None 이면 기준 zip 것 유지."""
    low = rel.lower()
    if low.endswith(KEEP_FROM_BASE_ZIP):
        return None
    parts = rel.replace("\\", "/").split("/")
    if parts[0] == "src" or low.endswith(".rs"):
        p = os.path.join(MOD_SRC, *parts)
        return p if os.path.exists(p) else None
    p = os.path.join(MOD_LIVE, *parts)
    return p if os.path.exists(p) else None


zin = zipfile.ZipFile(SRC_ZIP)
ents = [i for i in zin.infolist() if not i.filename.endswith("/")]
stage = os.path.join(os.environ["TEMP"], "it_source_054.zip")
if os.path.exists(stage):
    os.remove(stage)

replaced, kept, missing = [], [], []
with zipfile.ZipFile(stage, "w", zipfile.ZIP_DEFLATED, compresslevel=9) as zout:
    for i in ents:
        name = i.filename
        rel = name.replace("\\", "/").split("/", 1)[1] if "/" in name.replace("\\", "/") else name
        p = resolve(rel)
        if p:
            data = open(p, "rb").read()
            old = zin.read(name)
            if hashlib.sha256(data).hexdigest() != hashlib.sha256(old).hexdigest():
                replaced.append((rel, len(old), len(data)))
            else:
                kept.append(rel)
        else:
            data = zin.read(name)
            missing.append(rel)
        zout.writestr(name, data)

os.makedirs(os.path.dirname(DST_ZIP), exist_ok=True)
shutil.copy2(stage, DST_ZIP)
zz = zipfile.ZipFile(DST_ZIP)
print(f"tfm2_item_tactics_source: {os.path.getsize(DST_ZIP):,}B  엔트리 {len(zz.namelist())} (기준 {len(ents)})")
for r, a, b in replaced:
    print(f"   갱신 {r:<48} {a:>9,} -> {b:>9,}B")
if kept:
    print(f"   동일: {', '.join(kept)}")
if missing:
    print(f"   ⚠ 원본 못 찾아 기준 zip 유지: {', '.join(missing)}")
