# -*- coding: utf-8 -*-
"""apgate.py — `tfm2_ai_adjust` 의 `apply_*` 바이트패치 체인을 cfg 로 on/off 해서
크래시 범인을 이분탐색한다.

배경(2026-09-02): `retreat_capture` 안의 apply 체인 43개 중 하나가 0.5.8 에서
잘못된 주소에 패치를 써 게임을 죽인다. 노브를 전부 기본값으로 해도 죽으므로
"cfg 와 무관하게 적용되는 패치"가 범인 — 함수 단위로 갈라야 한다.

사용
  python MIG\apgate.py list                # 전체 목록과 현재 상태
  python MIG\apgate.py off <name>...       # 지정한 것만 끔
  python MIG\apgate.py off --range 0 21    # 목록 [0,21) 을 끔
  python MIG\apgate.py on                  # 전부 켬(ap_* 줄 제거)
"""
import os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")

CFG = os.path.join("C:", os.sep, "Program Files (x86)", "Steam", "steamapps", "common",
                   "Teamfight Manager2", "mods", "tfm2_ai_adjust", "tfm2_ai_adjust.cfg")
NAMES = os.path.join("C:", os.sep, "tfm2mods", "MIG", "ap_names.txt")
MARK = "# [APGATE]"


def names():
    return [l.strip() for l in open(NAMES, encoding="utf-8") if l.strip()]


def read():
    return open(CFG, encoding="utf-8").read().split("\n")


def write(lines):
    open(CFG, "w", encoding="utf-8", newline="").write("\n".join(lines))


def clear(lines):
    """기존 ap_* 게이트 줄 제거."""
    out, drop = [], False
    for l in lines:
        if l.startswith(MARK):
            drop = True
            continue
        if drop and re.match(r"\s*ap_[a-z0-9_]+\s*=", l):
            continue
        drop = False
        out.append(l)
    return [l for l in out if not re.match(r"\s*ap_[a-z0-9_]+\s*=", l)]


def cmd_list():
    ns = names()
    cur = set(re.findall(r"^ap_([a-z0-9_]+)\s*=\s*0", "\n".join(read()), re.M))
    print("apply 체인 %d개  (OFF=%d)" % (len(ns), len(cur)))
    for i, n in enumerate(ns):
        print("  [%2d] %-16s %s" % (i, n, "OFF" if n in cur else "on"))


def cmd_off(targets):
    ns = names()
    bad = [t for t in targets if t not in ns]
    if bad:
        print("★ 목록에 없음: %s" % ", ".join(bad))
        return 1
    lines = clear(read())
    lines.append("")
    lines.append("%s 크래시 이분탐색 — 아래 apply 는 건너뛴다 (%d개)" % (MARK, len(targets)))
    for t in targets:
        lines.append("ap_%s = 0" % t)
    write(lines)
    print("OFF %d개: %s" % (len(targets), ", ".join(targets)))
    print("ON  %d개" % (len(ns) - len(targets)))
    return 0


def cmd_on():
    write(clear(read()))
    print("ap_* 게이트 전부 제거 (= 전부 ON)")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)
    c = sys.argv[1]
    if c == "list":
        cmd_list()
    elif c == "on":
        cmd_on()
    elif c == "off":
        a = sys.argv[2:]
        if a and a[0] == "--range":
            lo, hi = int(a[1]), int(a[2])
            sys.exit(cmd_off(names()[lo:hi]))
        sys.exit(cmd_off(a))
    else:
        print(__doc__)
