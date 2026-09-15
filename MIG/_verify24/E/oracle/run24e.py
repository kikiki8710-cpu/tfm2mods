# -*- coding: utf-8 -*-
"""24차 E 오라클 드라이버 — 케이스당 프로세스 1개(TEMPLATE 함정 ③). 로그 = oracle/o24e_184.log"""
import subprocess, os, sys, io
EXE = os.path.join(os.environ.get("LOCALAPPDATA", r"C:\Users\jungs\AppData\Local"), "Temp", "tfm2_spanprobe", "o24e.exe")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.stdout.reconfigure(encoding="utf-8", errors="replace")

# 184: dist speed version well call2 tick enemy
CASES = [
    "480000 1000 55 0 0 1000",   # 멀리·time 480 ≥ 240 → Return (IR)  / 명세문면 → Move
    "100000 1000 55 0 0 1000",   # 가까이·time 100 → Move(direct_heal)
    "240000 1000 55 0 0 1000",   # 경계 time == 240 → Return
    "239999 1000 55 0 0 1000",   # 경계 time == 239 → Move
    "239000 1000 55 0 0 1000",   # time 239 · d ≥ 200001 → direct_heal 아님 → 절2/3 후보 탐색 경로
    "200000 1000 55 0 0 1000",   # d=200000 < 200001 → direct_heal
    "200001 1000 55 0 0 1000",   # d=200001 → direct_heal 아님
    "0 1000 55 1 0 1000",        # 우물 안 → None
    "480000 0 55 0 0 1000",      # speed 0 → L716 div0 패닉(exit 101)
    "480000 1000 1 0 0 1000",    # version 1(legacy): direct_heal/enemy_knows 안 봄 · Return 판정은 동일
    "100000 1000 1 0 0 1000",    # version 1 · 가까이
    "239000 1000 55 0 1 1000",   # 후보 탐색 + 2회 호출(path_finder 재사용)
    "100000 1000 55 0 1 1000",   # direct_heal + 2회 호출
    "480000 100000 55 0 0 1000", # speed 100000 → time 4 → Move
    "24000 100 55 0 0 1000",     # d 24000 · speed 100 → time 240 이지만 x=56000 ≤ rx 64000 → 우물 안 → None (L691 이 L719 보다 먼저)
    "480000 1000 55 0 0 1000 1", # 적 Top 을 내 옆 50000 Visible — is_safe_recall 은 여전히 true(default 챔프 공격력 0) → Return
    "239000 1000 55 0 1 1000 1", # 적 근접 + 후보 탐색 + 2회 호출
    "100000 1000 55 0 0 1000 1", # 적 근접 + direct_heal
]
PE_CASES = ["0", "1"]   # POS_EVAL_CACHE 히트/스테일 실험 (purpose Recall / AttackStance)

def run(args):
    r = subprocess.run([EXE] + args.split(), capture_output=True, text=True, encoding="utf-8", errors="replace")
    return r.returncode, r.stdout, r.stderr

if __name__ == "__main__":
    log = io.open(os.path.join(HERE, "o24e_184.log"), "w", encoding="utf-8")
    for c in CASES:
        rc, out, err = run("184 " + c)
        res = [l for l in out.splitlines() if l.startswith("RESULT")]
        line = "case[%s] rc=%d\n  %s" % (c, rc, "\n  ".join(res) if res else (err.strip().splitlines() or ["(no output)"])[-1])
        print(line); log.write(line + "\n")
    for c in PE_CASES:
        rc, out, err = run("pe " + c)
        res = out[out.find("RESULTPE"):] if "RESULTPE" in out else (err.strip() or "(no output)")
        line = "pe[%s] rc=%d\n  %s" % (c, rc, res.strip().replace("\n", "\n  "))
        print(line); log.write(line + "\n")
    log.close()
