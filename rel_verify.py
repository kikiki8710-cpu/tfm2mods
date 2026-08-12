# -*- coding: utf-8 -*-
# rel_verify.py [버전] — 릴리스 zip ↔ 현재 게임 설치본 정합성 검증. (버전 생략 시 0.5.5)
#   목적: 다른 세션이 zip 생성 이후 dll 을 재배포했는지(=zip 이 구본) 탐지 + 릴리스 부적합 cfg 플래그 탐지.
#   (구판은 0.5.3 하드코딩이었다 — 2026-08-12 인자화)
import zipfile, os, sys, io, hashlib, re, time

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
G = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods"
OUT = os.path.join(G, "release", sys.argv[1] if len(sys.argv) > 1 else "0.5.5")
# 릴리스에 들어가면 안 되는 진단 플래그
BAD = re.compile(r"^\s*(drbp|debug|log|detlog|dump|verbose|trace|mpcap|diag)\s*=\s*([1-9])", re.I | re.M)


def sha(b):
    return hashlib.sha256(b).hexdigest()[:16]


def zread(z, name):
    # 암호 zip 변형(tfm2_banpick_order_pw(tfm2) 등)은 비번 "tfm2" 로 읽는다
    try:
        return z.read(name)
    except RuntimeError:
        return z.read(name, pwd=b"tfm2")


print(f"{'zip':30s} {'엔트리':>6s}  dll/cfg 정합성")
print("-" * 100)
for f in sorted(os.listdir(OUT)):
    if not f.endswith(".zip"):
        continue
    z = zipfile.ZipFile(os.path.join(OUT, f))
    zt = os.path.getmtime(os.path.join(OUT, f))
    notes = []
    for i in z.infolist():
        if i.filename.endswith("/"):
            continue
        rel = i.filename.split("/", 1)[1] if "/" in i.filename else i.filename
        low = rel.lower()
        # zip 안 dll/exe ↔ 라이브 대조 (모드 폴더 = zip 루트 폴더명)
        root = i.filename.split("/", 1)[0]
        live = os.path.join(G, root, *rel.split("/"))
        if low.endswith((".dll", ".exe")) and os.path.exists(live):
            zd = zread(z, i.filename)
            ld = open(live, "rb").read()
            if sha(zd) != sha(ld):
                lt = time.strftime("%m-%d %H:%M:%S", time.localtime(os.path.getmtime(live)))
                notes.append(f"★{rel} zip {len(zd):,}B ≠ 라이브 {len(ld):,}B(갱신 {lt})")
        # 진단 플래그
        if low.endswith((".cfg", ".ini", ".txt")) and i.file_size < 200000:
            try:
                t = zread(z, i.filename).decode("utf-8", "replace")
            except Exception:
                continue
            m = BAD.search(t)
            if m:
                notes.append(f"★{rel} 진단플래그 `{m.group(0).strip()}`")
    n = len([i for i in z.infolist() if not i.filename.endswith("/")])
    zts = time.strftime("%m-%d %H:%M", time.localtime(zt))
    print(f"{f:30s} {n:>6d}  [{zts}] " + ("  |  ".join(notes) if notes else "OK"))
