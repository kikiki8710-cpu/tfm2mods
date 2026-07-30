# -*- coding: utf-8 -*-
# release_ai_adjust_053.py — tfm2_ai_adjust 0.5.3 릴리스 zip 생성 (/deploy §4 준수)
#   ★배포처 = <게임설치>\mods\release\<게임버전>\<MOD_ID>.zip  (C:\tfm2mods\release 아님)
#   ★zip 루트에 <MOD_ID>\ 한 겹
#   ★cfg 는 "라이브 복사"가 아니라 스테이징 사본에서 **배포 기본값으로 정규화**
#     (진단 플래그만 0. 유저 튜닝값은 0.5.2 릴리스 관행대로 유지) — 라이브 파일은 절대 수정 안 함
#   ★BOM 없는 UTF-8 유지 / 개인·런타임 산출물 제외
import io, sys, os, re, shutil, zipfile
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

MODID = "tfm2_ai_adjust"
GAME = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2"
LIVE = os.path.join(GAME, "mods", MODID)
RELDIR = os.path.join(GAME, "mods", "release", "0.5.3")
STAGE = r"C:\Users\dev\AppData\Local\Temp\tfm2_stage_053"
OUT = os.path.join(RELDIR, MODID + ".zip")

# 0.5.2 릴리스 엔트리 구성을 그대로 따른다(검증된 구성)
FILES = ["mod.mod_info", "tfm2_ai_adjust.cfg", "tfm2_ai_adjust.dll", "ui_inject.txt", "설정편집기.exe"]
DIRS = {"config": lambda n: n.lower().endswith((".cfg", ".txt")) and not n.endswith(".bak"),
        "players": lambda n: n == "_사용법.txt",
        "ui_inject": lambda n: n.lower().endswith(".ui")}

# 진단 플래그만 배포 기본값으로. (튜닝값 vis_window/oi_*/d19i_* 등은 0.5.2 릴리스와 동일하게 유지)
NORM = {
    "log": ("0", "프로덕션(진단 OFF)"),
    "mpcap": ("0", "프로덕션(캡처 OFF)"),
    "hang_diag": ("0", "개발자 전용 워치독 — 배포본은 OFF"),
    "adv_prof": ("0", "개발자 전용 프로파일 — 배포본은 OFF"),
    "d7_repl": ("1", "★0.5.3 Recall 재가동(유저 지시). scan2 경로만 원본위임"),
}

if os.path.isdir(STAGE):
    shutil.rmtree(STAGE)
root = os.path.join(STAGE, MODID)
os.makedirs(root)

for f in FILES:
    src = os.path.join(LIVE, f)
    if not os.path.exists(src):
        print("  ⚠ 없음(건너뜀):", f); continue
    shutil.copy2(src, os.path.join(root, f))
for sub, keep in DIRS.items():
    s = os.path.join(LIVE, sub)
    if not os.path.isdir(s): continue
    os.makedirs(os.path.join(root, sub), exist_ok=True)
    for n in os.listdir(s):
        if os.path.isfile(os.path.join(s, n)) and keep(n):
            shutil.copy2(os.path.join(s, n), os.path.join(root, sub, n))

# ── cfg 정규화(스테이징 사본만) ──
cfgp = os.path.join(root, "tfm2_ai_adjust.cfg")
raw = open(cfgp, "rb").read()
assert raw[:3] != b"\xef\xbb\xbf", "라이브 cfg에 BOM이 있다 — 중단"
t = raw.decode("utf-8")
print("── cfg 정규화(스테이징 사본) ──")
for k, (v, why) in NORM.items():
    pat = re.compile(r'(?m)^(\s*%s\s*=\s*)(\S+)(.*)$' % k)
    m = pat.search(t)
    if not m:
        t += "\n%s = %s   # %s\n" % (k, v, why); print("  + %s = %s (신규)" % (k, v)); continue
    if m.group(2) != v:
        t = pat.sub(lambda mm: mm.group(1) + v + "   # " + why, t, count=1)
        print("  ~ %s : %s → %s" % (k, m.group(2), v))
    else:
        print("  = %s : %s (이미 기본값)" % (k, v))
open(cfgp, "wb").write(t.encode("utf-8"))          # BOM 없이 재기록
assert open(cfgp, "rb").read()[:3] != b"\xef\xbb\xbf"

# ── zip ──
os.makedirs(RELDIR, exist_ok=True)
if os.path.exists(OUT):
    print("  (기존 zip 덮어씀: %d B)" % os.path.getsize(OUT))
with zipfile.ZipFile(OUT, "w", zipfile.ZIP_DEFLATED) as z:
    for dp, _, fns in os.walk(root):
        for n in fns:
            full = os.path.join(dp, n)
            z.write(full, os.path.join(MODID, os.path.relpath(full, root)))
print("\n=== zip 생성: %s (%d B) ===" % (OUT, os.path.getsize(OUT)))
with zipfile.ZipFile(OUT) as z:
    for i in z.infolist():
        print("  %-52s %10d" % (i.filename, i.file_size))
    print("  총 %d 엔트리" % len(z.infolist()))
