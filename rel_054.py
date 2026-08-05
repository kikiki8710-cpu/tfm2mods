# -*- coding: utf-8 -*-
"""rel_054.py — 0.5.3 릴리스 zip을 바탕으로 0.5.4 릴리스 zip을 만든다.

방식(정석, [[tfm2-release-zip-location]] §zip 생성 함정):
  직전 버전 zip을 스테이징으로 삼아 **dll/exe/mod.mod_info만 라이브 배포본으로 교체**하고
  cfg·자산·README는 기준 zip 것을 그대로 유지한다.
  → ① 자산 누락 방지(엔트리 수 전수 대조로 검증 가능)
    ② 개발 머신의 라이브 cfg(테스트값)가 유출되지 않음

zip 종류 2가지를 모두 다룬다:
  - 단일 모드 zip : 엔트리 = "<mod_id>/<rel>"            (예: Spectator_Chat.zip)
  - 다중 모드 zip : 엔트리 = "<루트>/<mod_id>/<rel>" 또는 "<mod_id>/<rel>"
                    (daram2_viewplus.zip = 9종, 팀파매gg모드3종.zip = 3종, 루트명에 버전이 박혀 있음)

⚠ Python zipfile 은 UTF-8 플래그를 알아서 세우므로 PowerShell CreateFromDirectory 의
   한글 파일명 깨짐 함정(entryNameEncoding 명시 금지)에 해당하지 않는다.
⚠ 게임 폴더(Program Files)는 샌드박스가 삭제를 막으므로 로컬에 만든 뒤 copy 로 덮어쓴다.

사용:
  python rel_054.py <zip이름(확장자 제외)> [--root-rename 구루트=새루트]
  python rel_054.py --all
"""
import os, sys, io, json, shutil, zipfile, hashlib

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

GAME = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods"
SRC_VER, DST_VER = "0.5.3", "0.5.4"
SRC_DIR = os.path.join(GAME, "release", SRC_VER)
DST_DIR = os.path.join(GAME, "release", DST_VER)

# 개인·런타임 파일 제외 규칙(/deploy §4)
EXC_SUF = (".bak", ".pdb", ".exp", ".lib", ".log", ".old")
EXC_SUB = ("_log.txt", "_crash", "debug.txt", ".bak_")

LIVE_REPLACE_SUF = (".dll", ".exe")
LIVE_REPLACE_NAME = ("mod.mod_info",)


def live_path_for(entry: str):
    """zip 엔트리 경로 -> 게임 mods\\ 아래 대응 파일 경로. 없으면 None.

    엔트리 선두 세그먼트를 하나씩 벗겨가며 mods\\<mod_id>\\... 로 실재하는 조합을 찾는다.
    (다중 모드 zip 은 앞에 버전이 박힌 루트 폴더가 한 겹 더 있다.)
    """
    parts = entry.replace("\\", "/").split("/")
    for i in range(len(parts) - 1):
        cand = os.path.join(GAME, *parts[i:])
        if os.path.exists(cand):
            return cand
    return None


def rebuild(zip_stem: str, root_rename=None):
    src = os.path.join(SRC_DIR, zip_stem + ".zip")
    dst = os.path.join(DST_DIR, zip_stem + ".zip")
    if not os.path.exists(src):
        print(f"{zip_stem}: 기준 zip 없음 -> SKIP")
        return
    os.makedirs(DST_DIR, exist_ok=True)
    zin = zipfile.ZipFile(src)
    ents = [i for i in zin.infolist() if not i.filename.endswith("/")]
    stage = os.path.join(os.environ["TEMP"], f"{zip_stem}_054.zip")
    if os.path.exists(stage):
        os.remove(stage)

    replaced, kept, skipped, missing = [], 0, [], []
    with zipfile.ZipFile(stage, "w", zipfile.ZIP_DEFLATED, compresslevel=9) as zout:
        for i in ents:
            name = i.filename
            low = name.replace("\\", "/").rsplit("/", 1)[-1].lower()
            if low.endswith(EXC_SUF) or any(x in low for x in EXC_SUB):
                skipped.append(name)
                continue
            want_live = low.endswith(LIVE_REPLACE_SUF) or low in LIVE_REPLACE_NAME
            data = None
            if want_live:
                p = live_path_for(name)
                if p:
                    data = open(p, "rb").read()
                    old = zin.read(name)
                    if hashlib.sha256(data).hexdigest() != hashlib.sha256(old).hexdigest():
                        replaced.append((name, len(old), len(data)))
                else:
                    missing.append(name)
            if data is None:
                data = zin.read(name)
                kept += 1
            out_name = name
            if root_rename:
                a, b = root_rename
                if out_name.startswith(a + "/") or out_name.startswith(a + "\\"):
                    out_name = b + out_name[len(a):]
            zout.writestr(out_name, data)

    shutil.copy2(stage, dst)
    zz = zipfile.ZipFile(dst)
    names = zz.namelist()
    print(f"\n=== {zip_stem}  {os.path.getsize(dst):,}B  엔트리 {len(names)} (기준 {len(ents)})")
    for n, a, b in replaced:
        print(f"   교체 {n:<52} {a:>10,} -> {b:>10,}B")
    if missing:
        print(f"   ⚠ 라이브 대응 파일 없음(기준 zip 것 유지): {', '.join(missing)}")
    if skipped:
        print(f"   제외: {', '.join(skipped)}")
    # mod_info 요약(단일/다중 모두)
    for n in names:
        if n.replace("\\", "/").endswith("mod.mod_info"):
            try:
                j = json.loads(zz.read(n).decode("utf-8"))
                deps = [d.get("version") for d in j.get("dependencies", []) if d.get("mod_id") == "base"]
                print(f"   mod_info {j.get('mod_id'):<28} v{j.get('version'):<8} author={j.get('author'):<10} base={deps}")
            except Exception as e:
                print(f"   ⚠ mod_info 파싱 실패 {n}: {e}")


JOBS = [
    ("Spectator_Chat", None),
    ("tfm2_mod_order", None),
    ("tfm2_comptest_unlock", None),
    ("tfm2_draft_overlay", None),
    ("tfm2_level_cap", None),
    ("tfm2_item_tactics", None),
    ("tfm2_banpick_illust", None),
    ("tfm2_banpick_order", None),
    ("tfm2_elemental_serpen", None),
    ("community_reaction_mod", None),
    ("daram2_viewplus", None),
    ("팀파매gg모드3종", ("팀파매gg_0.5.3", "팀파매gg_0.5.4")),
]

if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "--all":
        for stem, rr in JOBS:
            rebuild(stem, rr)
    else:
        stem = sys.argv[1]
        rr = None
        for j, r in JOBS:
            if j == stem:
                rr = r
        for a in sys.argv[2:]:
            if a.startswith("--root-rename="):
                x, y = a.split("=", 1)[1].split("|")
                rr = (x, y)
        rebuild(stem, rr)
