# -*- coding: utf-8 -*-
# 0.5.3 -> 0.5.4 상수 일괄 갱신 (tfm2_elemental_serpen/src/lib.rs)
import io, re, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
P = r"C:\tfm2mods\tfm2_elemental_serpen\src\lib.rs"
src = open(P, encoding="utf-8").read()

# (상수명, 구값, 신값)
M = [
 ("SERPEN_RVA",      "0x1535810", "0x1328950"),
 ("SEED_OFF",        "0xeaf8",    "0xeb28"),
 ("SIM_TICK_OFF",    "0xeb00",    "0xeb30"),
 ("CAMP_SPAWN_TICK", "0xed10",    "0xed40"),
 ("CAMP_WAVE_IDX",   "0xed18",    "0xed48"),
 ("MOBATICK_RVA",    "0xeeeac0",  "0x13ee0a0"),
 ("KILLS_BLUE_OFF",  "0xed90",    "0xedc0"),
 ("KILLS_RED_OFF",   "0xed98",    "0xedc8"),
 ("KILLS_PTR_OFF",   "0xed60",    "0xed90"),
 ("KILLS_LEN_OFF",   "0xed68",    "0xed98"),
 ("LAUNCHER_RVA",    "0xeb8810",  "0x13b53d0"),
 ("LAUNCHER_RET_A",  "0x9a3287",  "0x9e2079"),
 ("LAUNCHER_RET_B",  "0x9a7b03",  "0x9e6feb"),
 ("LAUNCHER_RET_C",  "0x229ad94", "0x1d147e4"),
 ("UILOADER_RVA",    "0x2e1550",  "0x2e35d0"),
 ("UIPARSER_RVA",    "0x1a6530",  "0x1a3ce0"),
 ("UIALLOC_RVA",     "0x28f7df0", "0x29bb920"),
 ("RENDER_STEP_RVA", "0x960df0",  "0xaa06c0"),
 ("RUNNER_CTOR_RVA", "0xeba490",  "0x13b7050"),
 ("DMGA_RVA",        "0xfdbbb0",  "0x10670a0"),
 ("DMGB_RVA",        "0x12c3bb0", "0x14eaef0"),
 ("KEYRES_RVA",      "0x1b0aba0", "0x218be90"),
 ("ARG_STR_RVA",     "0x1228a90", "0x16a31e0"),
 ("P_TEAM",          "0x820",     "0x810"),
 ("P_CHAMP_TAG",     "0x8b8",     "0x8a8"),
 ("P_CHAMP_KEY",     "0x8c0",     "0x8b0"),
 ("PLAYER_STRIDE",   "0x8d0",     "0x8c0"),
]
ok, fail = 0, []
for name, old, new in M:
    pat = re.compile(r"(^const\s+" + name + r"\s*:\s*usize\s*=\s*)" + re.escape(old) + r"(\s*;)", re.M)
    src2, n = pat.subn(lambda m: m.group(1) + new + m.group(2), src)
    if n != 1:
        fail.append((name, old, n)); continue
    src = src2; ok += 1
    # 주석 태그: 같은 줄 끝의 " // 0.5.3 (구0.5.2=..." 를 0.5.4 로 갱신
    line_pat = re.compile(r"(^const\s+" + name + r"\s*:\s*usize\s*=\s*" + re.escape(new) + r"\s*;\s*)(//.*)?$", re.M)
    def fix(m):
        tail = m.group(2) or ""
        note = "// 0.5.4 (\uad6c0.5.3=" + old + ")"
        if tail.startswith("//"):
            body = tail[2:].lstrip()
            body = re.sub(r"^0\.5\.3\s*\([^)]*\)\s*", "", body)
            return m.group(1) + note + (" " + body if body else "")
        return m.group(1) + note
    src = line_pat.sub(fix, src, count=1)

# SPAWN_HOOKS 배열
sp = re.compile(r"(^const\s+SPAWN_HOOKS\s*:\s*\[usize;\s*2\]\s*=\s*)\[0xabdf60,\s*0xabd340\](\s*;\s*)(//.*)?$", re.M)
src2, n = sp.subn(lambda m: m.group(1) + "[0xb31bb0, 0xb30f90]" + m.group(2)
                  + "// 0.5.4 (\uad6c0.5.3=0xabdf60/0xabd340). \ud504\ub86c\ub85c\uadf8 12B \ub3d9\uc77c\u00b7\ud06c\uae30 2073=2073\u00b7\ucf5c\ub7ec \ucee8\ud14c\uc774\ub108 \uc9c0\ubb38(367/+0xa1, 431/+0xc9) \uc644\uc804\uc77c\uce58.", src)
if n == 1: src = src2; ok += 1
else: fail.append(("SPAWN_HOOKS", "array", n))

open(P, "w", encoding="utf-8").write(src)
print("갱신 OK:", ok, "/", len(M) + 1)
if fail: print("실패:", fail)
