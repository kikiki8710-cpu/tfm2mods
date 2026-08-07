# 모드 폴더 정리 — 재생성되는 로그·백업을 _old\ 로 옮긴다(삭제 아님, 되돌릴 수 있게).
#  KEEP 는 화이트리스트. 새 로그가 생겨도 자동으로 KEEP 에 안 들어가므로 안전.
import os, shutil, sys, collections
sys.stdout.reconfigure(encoding="utf-8")

MD  = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_ai_adjust"
OLD = os.path.join(MD, "_old")

KEEP_FILES = {
    "mod.mod_info", "mod.override_info",
    "tfm2_ai_adjust.dll", "tfm2_ai_adjust.cfg", "설정편집기.exe",
    "ui_inject.txt", "config_editor.hta", "설명서.md",
    "plan_reimpl.cfg",                 # ★모드가 읽는 설정 파일 — 로그 아님
    "crash_log.txt",                   # 크래시 증거는 남긴다
}
KEEP_DIRS = {"config", "players", "ui_inject", "_crash", "match_log", "_old"}

moved = collections.Counter()
os.makedirs(os.path.join(OLD, "logs"), exist_ok=True)
os.makedirs(os.path.join(OLD, "cfg_bak"), exist_ok=True)

for name in sorted(os.listdir(MD)):
    p = os.path.join(MD, name)
    if os.path.isdir(p):
        if name not in KEEP_DIRS: print("  ⚠예상 못한 폴더(그대로 둠):", name)
        continue
    if name in KEEP_FILES: continue
    low = name.lower()
    if ".bak" in low or "bak_" in low or ".devbak" in low or ".diagbak" in low:
        dst = os.path.join(OLD, "cfg_bak", name); moved["백업"] += 1
    else:
        dst = os.path.join(OLD, "logs", name); moved["로그"] += 1
    if os.path.exists(dst): os.remove(dst)
    shutil.move(p, dst)

print("옮김: " + " · ".join("%s %d개" % (k, v) for k, v in moved.items()))
rest = [f for f in sorted(os.listdir(MD)) if not os.path.isdir(os.path.join(MD, f))]
print("\n남은 파일 %d개:" % len(rest))
for f in rest: print("   %9d  %s" % (os.path.getsize(os.path.join(MD, f)), f))
print("\n남은 폴더:", " ".join(sorted(d for d in os.listdir(MD) if os.path.isdir(os.path.join(MD, d)))))
