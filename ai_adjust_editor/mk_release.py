# 릴리스 zip 생성 — <게임설치>\mods\release\<버전>\tfm2_ai_adjust.zip
#  · zip 루트에 tfm2_ai_adjust\ 폴더 한 겹
#  · 개인/런타임 파일 제외(로그·크래시·백업·판단덤프·개인 선수설정)
#  · 설명서.md 는 유저 지시로 제외
import io, os, sys, zipfile, hashlib
sys.stdout.reconfigure(encoding="utf-8")

GAME = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2"
SRC  = os.path.join(GAME, "mods", "tfm2_ai_adjust")
VER  = "0.5.3"
OUT  = os.path.join(GAME, "mods", "release", VER, "tfm2_ai_adjust.zip")
MOD  = "tfm2_ai_adjust"

# 포함할 것만 화이트리스트로 — 블랙리스트는 새 로그가 생기면 새는 구조라 쓰지 않는다.
ROOT_FILES = ["mod.mod_info", "mod.override_info", "tfm2_ai_adjust.dll",
              "tfm2_ai_adjust.cfg", "설정편집기.exe", "ui_inject.txt", "config_editor.hta"]
SUB_DIRS   = ["config", "ui_inject", "players"]
# 하위 폴더에서도 제외할 것
SKIP_EXT   = (".bak", ".tmp", ".log")
SKIP_NAME  = ("설명서.md",)                       # ★유저 지시: 설명서 제외
def skip(name):
    low = name.lower()
    if name in SKIP_NAME: return True
    if low.endswith(SKIP_EXT): return True
    if ".bak" in low: return True                  # default.txt.bak_xxx 류
    if low.endswith("_imm.txt"): return True       # 적용 결과 로그
    if low.startswith("crash") or low.startswith("_crash"): return True
    return False

items = []
for f in ROOT_FILES:
    p = os.path.join(SRC, f)
    if os.path.exists(p) and not skip(f): items.append((p, "%s/%s" % (MOD, f)))
for d in SUB_DIRS:
    dp = os.path.join(SRC, d)
    if not os.path.isdir(dp): continue
    for root, _, fs in os.walk(dp):
        for f in sorted(fs):
            if skip(f): continue
            p = os.path.join(root, f)
            rel = os.path.relpath(p, SRC).replace("\\", "/")
            items.append((p, "%s/%s" % (MOD, rel)))

os.makedirs(os.path.dirname(OUT), exist_ok=True)
if os.path.exists(OUT):
    os.replace(OUT, OUT + ".prev")
    print("기존 zip → tfm2_ai_adjust.zip.prev 로 보관")

with zipfile.ZipFile(OUT, "w", zipfile.ZIP_DEFLATED, compresslevel=9) as z:
    for p, arc in items: z.write(p, arc)

print("\n생성: %s" % OUT)
zf = zipfile.ZipFile(OUT)
print("엔트리 %d개 · %d B" % (len(zf.namelist()), os.path.getsize(OUT)))
for i in zf.infolist(): print("  %9d  %s" % (i.file_size, i.filename))
bad = zf.testzip()
print("\nCRC 검사:", "OK" if bad is None else "★손상 " + bad)
print("MD5[:8]:", hashlib.md5(open(OUT, "rb").read()).hexdigest()[:8].upper())
