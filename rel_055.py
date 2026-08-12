# -*- coding: utf-8 -*-
"""rel_055.py — 0.5.4 릴리스 zip을 바탕으로 0.5.5 릴리스 zip을 만든다. (rel_054.py 답습)

방식(정석, [[tfm2-release-zip-location]] §zip 생성 함정):
  직전 버전 zip을 스테이징으로 삼아 **dll/exe/mod.mod_info만 라이브 배포본으로 교체**하고
  cfg·자산·README는 기준 zip 것을 그대로 유지한다.
  → ① 자산 누락 방지(엔트리 수 전수 대조로 검증 가능)
    ② 개발 머신의 라이브 cfg(테스트값)가 유출되지 않음

0.5.5 회차 추가분(rel_054 대비):
  - TEXT_FIX: 기준 zip에 낡은 버전 문구가 있는 텍스트 엔트리를 스테이징에서만 치환
    (level_cap README = 0.5.3 문구 잔존 / crm README = 0.5.2 문구 잔존 — 0.5.4 회차 미조치분,
     gg zip 내 tfm2_meta_item_delegate/mod.mod_info = 라이브 대응 파일이 게임 폴더에 없어
     rel 교체가 안 되므로 fix_delegate_dep_054.py 방식의 deps 치환을 여기서 인라인 수행)
  - JOBS에 TFM2_Meta_Dashboard 포함(0.5.4 때는 별도 처리)

⚠ Python zipfile 은 UTF-8 플래그를 알아서 세우므로 PowerShell CreateFromDirectory 의
   한글 파일명 깨짐 함정(entryNameEncoding 명시 금지)에 해당하지 않는다.
⚠ 게임 폴더(Program Files)는 샌드박스가 삭제를 막으므로 로컬에 만든 뒤 copy 로 덮어쓴다.

사용:
  python rel_055.py <zip이름(확장자 제외)>
  python rel_055.py --all
"""
import os, sys, io, json, shutil, zipfile, hashlib, re

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

GAME = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods"
SRC_VER, DST_VER = "0.5.4", "0.5.5"
SRC_DIR = os.path.join(GAME, "release", SRC_VER)
DST_DIR = os.path.join(GAME, "release", DST_VER)

# 개인·런타임 파일 제외 규칙(/deploy §4)
EXC_SUF = (".bak", ".pdb", ".exp", ".lib", ".log", ".old")
EXC_SUB = ("_log.txt", "_crash", "debug.txt", ".bak_")

LIVE_REPLACE_SUF = (".dll", ".exe")
LIVE_REPLACE_NAME = ("mod.mod_info",)

# (zip_stem, 엔트리 suffix) -> [(old_regex, new), ...] — 스테이징 사본에만 적용(라이브 무수정).
TEXT_FIX = {
    ("tfm2_level_cap", "tfm2_level_cap/README.txt"): [
        (r"Teamfight Manager 2  0\.5\.3", "Teamfight Manager 2  0.5.5"),
        (r"게임 0\.5\.3 전용", "게임 0.5.5 전용"),
        (r"0\.5\.4 이상으로 업데이트되면", "0.5.6 이상으로 업데이트되면"),
    ],
    ("community_reaction_mod", "community_reaction_mod/README_설치안내.txt"): [
        (r"게임 0\.5\.2 대응 패치", "게임 0.5.5 대응 패치"),
        (r"1\. 게임 0\.5\.2 대응", "1. 게임 0.5.5 대응"),
        (r"0\.5\.2에서도 정상 동작하도록 갱신했습니다", "0.5.5에서도 정상 동작하도록 갱신했습니다"),
    ],
    # 라이브 대응 파일이 게임 mods\ 에 없어 rel 교체가 안 되는 워크샵 모드(fix_delegate_dep_054 방식)
    ("팀파매gg모드3종", "tfm2_meta_item_delegate/mod.mod_info"): [
        (r">=\s*0\.5\.4\s*,\s*<\s*0\.5\.5", ">=0.5.5, <0.5.6"),
    ],
}


def live_path_for(entry: str):
    """zip 엔트리 경로 -> 게임 mods\\ 아래 대응 파일 경로. 없으면 None."""
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
    stage = os.path.join(os.environ["TEMP"], f"{zip_stem}_055.zip")
    if os.path.exists(stage):
        os.remove(stage)

    replaced, kept, skipped, missing, textfixed = [], 0, [], [], []
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
            # 스테이징 전용 텍스트 치환(낡은 버전 문구·워크샵 mod_info deps)
            for (stem, suf), fixes in TEXT_FIX.items():
                if stem == zip_stem and name.replace("\\", "/").endswith(suf):
                    txt = data.decode("utf-8")
                    n_fixed = 0
                    for rx, new in fixes:
                        txt, k = re.subn(rx, new, txt)
                        n_fixed += k
                    if n_fixed:
                        data = txt.encode("utf-8")  # BOM 없음
                        textfixed.append((name, n_fixed))
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
    for n, k in textfixed:
        print(f"   문구치환 {n} ({k}건)")
    if missing:
        print(f"   ⚠ 라이브 대응 파일 없음(기준 zip 것 유지): {', '.join(missing)}")
    if skipped:
        print(f"   제외: {', '.join(skipped)}")
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
    ("TFM2_Meta_Dashboard", None),
    ("팀파매gg모드3종", ("팀파매gg_0.5.4", "팀파매gg_0.5.5")),
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
