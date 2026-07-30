# -*- coding: utf-8 -*-
# rel_053.py — 0.5.3 릴리스 zip 생성/점검.
#   방식(메모리 tfm2-release-zip-location §생성 방식 정석): 직전 버전 zip 엔트리를 "포함 화이트리스트"로 쓰고
#   각 파일은 게임 설치 폴더(실배포본)에서 가져온다 ⟹ 자산 누락 방지 + 최신 반영.
#   ⚠개인/런타임 파일 제외 · zip 루트에 <MOD_ID>\ 한 겹 · BOM 없는 mod_info · 로컬 생성 후 복사.
import os, sys, io, zipfile, json, shutil, hashlib
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

G = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods"
R = os.path.join(G, "release")
OUT = os.path.join(R, "0.5.3")
STAGE = r"C:\Users\dev\AppData\Local\Temp\claude\C--Users-dev-Desktop-claude-tfm2\89fdc8ca-ad45-4a5c-a3de-4f289a646255\scratchpad\rel053"

# 모드 → 기준(직전) 릴리스 버전
# ⚠tfm2_ai_adjust 는 제외 — 라이브 cfg/설정편집기 정규화 판단 + 다른 세션의 v1.5.0 갱신 검증이 선행돼야 함.
BASE = {"tfm2_banpick_illust": "0.5.2", "tfm2_draft_overlay": "0.5.1",
        "tfm2_comptest_unlock": "0.5.2", "tfm2_banpick_order": "0.5.2", "tfm2_mod_scroll_fix": "0.5.1"}
# 개인·런타임·빌드부산물 제외
EXC_SUF = (".bak", ".pdb", ".exp", ".lib", ".log", ".old")
EXC_SUB = ("_log.txt", "_crash", "debug.txt", "latest_match", "thumbnail.png", ".bak_")


def excluded(rel):
    l = rel.lower()
    return l.endswith(EXC_SUF) or any(x in l for x in EXC_SUB)


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()[:16]


def dry():
    for m, v in BASE.items():
        zp = os.path.join(R, v, m + ".zip")
        if not os.path.exists(zp):
            print(f"{m}: ⚠기준 zip 없음 {zp}")
            continue
        z = zipfile.ZipFile(zp)
        ents = [e for e in z.namelist() if not e.endswith("/")]
        live = os.path.join(G, m)
        have, missl = 0, []
        for e in ents:
            rel = e.split("/", 1)[1] if "/" in e else e
            if os.path.exists(os.path.join(live, *rel.split("/"))):
                have += 1
            else:
                missl.append(rel)
        extra = []
        relset = {e.split("/", 1)[1] if "/" in e else e for e in ents}
        for root, _, fs in os.walk(live):
            for f in fs:
                rel = os.path.relpath(os.path.join(root, f), live).replace("\\", "/")
                if excluded(rel) or rel in relset:
                    continue
                extra.append(rel)
        print(f"{m}: 기준 {v}({len(ents)}엔트리) → 설치폴더에 있음 {have} / 없음 {len(missl)} {missl[:5]}")
        if extra:
            print(f"     설치폴더 신규 파일 {len(extra)}: {extra[:10]}")


def build(m, v, extra_include=()):
    zp = os.path.join(R, v, m + ".zip")
    z = zipfile.ZipFile(zp)
    ents = [e for e in z.namelist() if not e.endswith("/")]
    live = os.path.join(G, m)
    st = os.path.join(STAGE, m)
    if os.path.exists(st):
        shutil.rmtree(st)
    os.makedirs(st)
    used_live = used_zip = 0
    files = []
    rels = [e.split("/", 1)[1] if "/" in e else e for e in ents]
    rels += list(extra_include)
    # ★정석: 기준 zip 을 그대로 풀고 **dll 과 mod.mod_info 만** 라이브(신규 배포본)로 교체한다.
    #   cfg 를 라이브에서 복사하면 유저 테스트값이 유출된다(실측: illust 1,953→968B, comptest 1,827→2,117B).
    #   자산(.ui 등)도 기준 zip 유지 = 릴리스 구성의 재현성 확보.
    LIVE_ONLY = (".dll", "mod.mod_info")
    for rel in rels:
        if excluded(rel):
            continue
        dst = os.path.join(st, *rel.split("/"))
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        src = os.path.join(live, *rel.split("/"))
        take_live = os.path.exists(src) and (rel.lower().endswith(".dll") or rel == "mod.mod_info")
        if take_live:
            shutil.copy2(src, dst)
            used_live += 1
        else:
            name = next((e for e in ents if (e.split("/", 1)[1] if "/" in e else e) == rel), None)
            if name is None:
                continue
            with open(dst, "wb") as f:
                f.write(z.read(name))
            used_zip += 1
        files.append(rel)
    # mod_info 검증(BOM·deps)
    mi = os.path.join(st, "mod.mod_info")
    info = ""
    if os.path.exists(mi):
        raw = open(mi, "rb").read()
        assert raw[0] == 0x7b, f"{m}: mod_info BOM!"
        j = json.loads(raw.decode("utf-8"))
        info = f"ver={j.get('version')} deps={[x.get('version') for x in j.get('dependencies', [])]} author={j.get('author')}"
    # zip 생성(로컬) → 복사
    local = os.path.join(STAGE, m + ".zip")
    if os.path.exists(local):
        os.remove(local)
    with zipfile.ZipFile(local, "w", zipfile.ZIP_DEFLATED, compresslevel=9) as out:
        for rel in sorted(files):
            out.write(os.path.join(st, *rel.split("/")), f"{m}/{rel}")
    os.makedirs(OUT, exist_ok=True)
    shutil.copy2(local, os.path.join(OUT, m + ".zip"))
    n = len(files)
    print(f"  {m}: 엔트리 {n}(기준 {len(ents)}) 설치폴더 {used_live}/zip폴백 {used_zip} "
          f"크기 {os.path.getsize(local):,}B  {info}")
    return n


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "build":
        print("■ 0.5.3 릴리스 zip 생성")
        for m, v in BASE.items():
            if os.path.exists(os.path.join(R, v, m + ".zip")):
                build(m, v)
            else:
                print(f"  {m}: 기준 zip 없음 — 스킵")
    else:
        dry()
