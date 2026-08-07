# 경기 한 판 뒤 실행 — 모든 *_imm.txt 의 applied=N/M 을 모아 PASS/FAIL 로 보여준다.
#   사용: python C:\tfm2mods\tfm2_ai_adjust\check_applied.py
import os, re, sys, time

MODDIR = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_ai_adjust"
DLL    = os.path.join(MODDIR, "tfm2_ai_adjust.dll")

# 08-03 정정 후 기대치. 여기 숫자와 다르면 그 그룹에 새 결함이 생긴 것이다.
EXPECT = {
    "auction_imm.txt": 55,   # ★08-03 결함 5종 수정분 — 50이 나오면 수정본이 안 실린 것
    "score2_imm.txt":  38,   # ★동상 — 33이면 구버전 dll
    "move_imm.txt":    23,
    "db_imm.txt":      39,
    "cast_imm.txt":    38,
    "score_imm.txt":   22,
    # ── 2026-08-04 배선분 — gen_expect.py 가 검증기에서 자동 산출(손으로 고치지 말 것) ──
    "ae_imm.txt": 13,
    "bv_imm.txt": 35,
    "c3_imm.txt": 63,
    "d4_imm.txt": 24,
    "eh_imm.txt": 56,
    "hd_imm.txt": 40,
    "lt_imm.txt": 5,
    "lv_imm.txt": 9,
    "move2_imm.txt": 12,
    "nx_imm.txt": 8,
    "rt_imm.txt": 11,
    "th_imm.txt": 54,
}

pat = re.compile(r"applied=(\d+)/(\d+)")
dll_mtime = os.path.getmtime(DLL) if os.path.exists(DLL) else 0

rows, bad, stale = [], 0, 0
for name in sorted(os.listdir(MODDIR)):
    if not name.endswith("_imm.txt"):
        continue
    p = os.path.join(MODDIR, name)
    try:
        m = pat.search(open(p, encoding="utf-8", errors="replace").read())
    except OSError:
        continue
    if not m:
        continue
    n, tot = int(m.group(1)), int(m.group(2))
    mt = os.path.getmtime(p)
    old = mt < dll_mtime                      # dll 보다 오래됐으면 구버전이 남긴 로그
    exp = EXPECT.get(name)
    if old:
        st, stale = "STALE", stale + 1
    elif n < tot or (exp is not None and tot != exp):
        st, bad = "FAIL", bad + 1
    else:
        st = "PASS"
    rows.append((st, name, n, tot, exp, time.strftime("%H:%M:%S", time.localtime(mt))))

w = max((len(r[1]) for r in rows), default=10)
print(f"dll  : {time.strftime('%Y-%m-%d %H:%M:%S', time.localtime(dll_mtime))}\n")
for st, name, n, tot, exp, ts in rows:
    extra = ""
    if exp is not None and tot != exp:
        extra = f"  <-- 총 사이트 수가 기대({exp})와 다름"
    elif n < tot:
        extra = f"  <-- {tot - n}건 미적용"
    print(f"  [{st:5s}] {name:<{w}}  {n}/{tot}   {ts}{extra}")

print()
if stale:
    print(f"⚠ STALE {stale}건 — dll 보다 오래된 로그입니다. 게임을 한 판 더 돌려야 갱신됩니다.")
if bad:
    print(f"✗ FAIL {bad}건 — 위 표의 미적용 사이트를 verify_fail2.py 로 특정하세요.")
if not bad and not stale:
    print("✓ 전 그룹 PASS — 모든 byte-patch 사이트가 적용됐습니다.")
sys.exit(1 if (bad or stale) else 0)
