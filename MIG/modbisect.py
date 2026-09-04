# -*- coding: utf-8 -*-
"""modbisect.py — 크래시 범인 모드 이분탐색 도구.

⚠파일명 주의: 처음 `bisect.py` 로 만들었더니 **표준 라이브러리 `bisect` 를 가려**
   같은 폴더의 `repin.py`(`import bisect`)가 `bisect.bisect_right` 없음으로 죽었다.
   MIG\ 안에 표준 모듈과 같은 이름을 두지 말 것.

왜: 모드 스택 프레임(`MOD+0x289a`)이 여러 dll에 공통인 SDK 콜백 스텁이라
    크래시 로그만으로는 범인을 특정할 수 없다. deps 대역을 내려
    게임 로더가 스스로 비활성화하게 만드는 방식으로 on/off 를 제어한다.

사용
  python MIG\bisect.py list                 # 현재 0.5.8 에서 로드되는 모드 목록
  python MIG\bisect.py off <mod> [<mod>..]  # 지정 모드만 끔 (deps 를 <0.5.8 로)
  python MIG\bisect.py off --all            # 전부 끔
  python MIG\bisect.py off --half A         # 목록 앞 절반만 끔 (B = 뒷 절반)
  python MIG\bisect.py on --all             # 전부 원복 (.bak_bisect 에서)
백업: 각 mod.mod_info 옆에 .bak_bisect (원복 시 삭제)
"""
import os, sys, json, re, shutil

MODS = os.path.join("C:" + os.sep, "Program Files (x86)", "Steam", "steamapps",
                    "common", "Teamfight Manager2", "mods")
CUR = (0, 5, 8)
OFF_BAND = ">=0.5.7, <0.5.8"
BAK = ".bak_bisect"

sys.stdout.reconfigure(encoding="utf-8", errors="replace")


def band_ok(s):
    ok = True
    for part in s.split(","):
        m = re.match(r"\s*(>=|<=|>|<|==|=)?\s*(\d+)\.(\d+)\.(\d+)", part)
        if not m:
            continue
        op = m.group(1) or "=="
        v = tuple(int(x) for x in m.group(2, 3, 4))
        ok &= {">=": CUR >= v, ">": CUR > v, "<=": CUR <= v,
               "<": CUR < v, "==": CUR == v, "=": CUR == v}[op]
    return ok


def read(mi):
    raw = open(mi, "rb").read()
    if raw[:1] != b"{":
        raise ValueError("BOM/비정상 시작 바이트: %s" % mi)
    return json.loads(raw.decode("utf-8"))


def write(mi, j):
    out = json.dumps(j, ensure_ascii=False).encode("utf-8")
    open(mi, "wb").write(out)
    chk = open(mi, "rb").read()
    assert chk[:1] == b"{", "BOM 생김"
    json.loads(chk.decode("utf-8"))          # 재파싱 검증


def base_dep(j):
    for d in j.get("dependencies", []):
        if isinstance(d, dict) and d.get("mod_id") == "base":
            return d
    return None


def scan():
    """(mod_id, dll있음, base대역, 0.5.8로드여부) 목록."""
    out = []
    for d in sorted(os.listdir(MODS)):
        p = os.path.join(MODS, d)
        mi = os.path.join(p, "mod.mod_info")
        if not os.path.isdir(p) or not os.path.isfile(mi):
            continue
        if not [f for f in os.listdir(p) if f.endswith(".dll")]:
            continue
        try:
            j = read(mi)
        except Exception as e:
            out.append((d, True, "ERR:%s" % e, None))
            continue
        dep = base_dep(j)
        band = dep["version"] if dep else "(base 없음)"
        out.append((d, True, band, band_ok(band) if dep else True))
    return out


def cmd_list():
    rows = scan()
    live = [r for r in rows if r[3]]
    print("### 0.5.8 에서 로드되는 모드 = %d개 ###" % len(live))
    for i, (m, _, band, _) in enumerate(live):
        print("  [%2d] %-34s %s" % (i, m, band))
    print()
    print("### 이미 꺼진 모드 = %d개 ###" % len([r for r in rows if not r[3]]))
    return [r[0] for r in live]


def cmd_off(targets):
    n = 0
    for m in targets:
        mi = os.path.join(MODS, m, "mod.mod_info")
        if not os.path.isfile(mi):
            print("  건너뜀(없음): %s" % m)
            continue
        if not os.path.isfile(mi + BAK):
            shutil.copy2(mi, mi + BAK)
        j = read(mi)
        dep = base_dep(j)
        if dep is None:
            j.setdefault("dependencies", []).append(
                {"mod_id": "base", "version": OFF_BAND})
        else:
            dep["version"] = OFF_BAND
        write(mi, j)
        n += 1
        print("  OFF  %s" % m)
    print("총 %d개 비활성화" % n)


def cmd_on():
    n = 0
    for d in sorted(os.listdir(MODS)):
        mi = os.path.join(MODS, d, "mod.mod_info")
        if os.path.isfile(mi + BAK):
            shutil.copy2(mi + BAK, mi)
            os.remove(mi + BAK)
            n += 1
            print("  ON   %s" % d)
    print("총 %d개 원복" % n)


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)
    cmd = sys.argv[1]
    args = sys.argv[2:]
    if cmd == "list":
        cmd_list()
    elif cmd == "on":
        cmd_on()
    elif cmd == "off":
        if args and args[0] == "--all":
            cmd_off([r[0] for r in scan() if r[3]])
        elif args and args[0] == "--half":
            live = [r[0] for r in scan() if r[3]]
            half = len(live) // 2
            cmd_off(live[:half] if args[1].upper() == "A" else live[half:])
        else:
            cmd_off(args)
    else:
        print(__doc__)
