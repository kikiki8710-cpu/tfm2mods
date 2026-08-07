# 키 이름 정리 + 설명문 ★ 제거
#   db_*      → dm_*   (데스매치 모드 전용)
#   db_safe_* → sf_*   (⚠데스매치 전용이 아니다 — 일반 경기의 '때려도 되나' 판정도 같은 함수를 쓴다)
#   설명문·안내문·섹션 제목의 ★ 전부 제거
import io, os, re, sys, glob
sys.stdout.reconfigure(encoding="utf-8")

MODSRC = r"C:\tfm2mods\tfm2_ai_adjust\src"
EDSRC  = r"C:\tfm2mods\ai_adjust_editor\src\main.rs"
CFGDIR = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_ai_adjust"

# 긴 이름 먼저 치환해야 db_safe_* 가 db_* 규칙에 먼저 안 먹힌다
REN = [("db_safe_margin", "sf_margin"), ("db_safe_radius", "sf_radius"), ("db_safe_mem", "sf_mem"),
       ("db_safe_", "sf_")]
for k in ["execute_hp", "lasthit", "lookahead", "near_ally", "near_enemy", "skill2_level",
          "skill_hp", "ult_level", "ult_lookahead", "ult_mask_focus", "ult_mask_rally",
          "ult_mask_safe", "ult_rally2", "ult_rally", "ult_range"]:
    REN.append(("db_" + k, "dm_" + k))
REN += [("db_near_", "dm_near_"), ("db_ult_", "dm_ult_")]   # FLOW 접두

def rename_in(path, count_only=False):
    s = io.open(path, encoding="utf-8", errors="replace").read()
    orig, n = s, 0
    for a, b in REN:
        c = s.count(a)
        if c: s = s.replace(a, b); n += c
    if not count_only and s != orig:
        io.open(path, "w", encoding="utf-8", newline="\n" if path.endswith(".cfg") or path.endswith(".txt") else None).write(s)
    return n

tot = 0
for f in sorted(glob.glob(os.path.join(MODSRC, "*.rs"))):
    c = rename_in(f)
    if c: print("  %-28s %d곳" % (os.path.basename(f), c)); tot += c
c = rename_in(EDSRC); print("  %-28s %d곳" % ("편집기 main.rs", c)); tot += c
for f in [os.path.join(CFGDIR, "tfm2_ai_adjust.cfg"),
          os.path.join(CFGDIR, "config", "default.txt"),
          os.path.join(CFGDIR, "config", "테스트A.cfg"),
          os.path.join(CFGDIR, "config", "테스트B.cfg")]:
    if os.path.exists(f):
        c = rename_in(f); print("  %-28s %d곳" % (os.path.basename(f), c)); tot += c
print("키 이름 치환 합계 %d곳\n" % tot)

# ── 설명문 ★ 제거 (편집기의 사용자 노출 문자열만) ──
s = io.open(EDSRC, encoding="utf-8").read()
def strip_star(seg):
    seg = re.sub(r'★+\s*', '', seg)
    seg = re.sub(r'[ \t]{2,}', ' ', seg)
    return seg

# desc_static 본문
i = s.index("fn desc_static("); j = s.index("\n}", i)
before = s[i:j].count("★"); s = s[:i] + strip_star(s[i:j]) + s[j:]
# TABS(섹션 제목 + note) · FLOW(label/note/title/sub/body)
k = s.index("static TABS")
m = s.index("static FLOW")
end = s.index("\n];", m) + 3
b2 = s[k:end].count("★"); s = s[:k] + strip_star(s[k:end]) + s[end:]
io.open(EDSRC, "w", encoding="utf-8").write(s)
print("★ 제거: 설명문 %d개 · 탭/순서도 %d개" % (before, b2))
