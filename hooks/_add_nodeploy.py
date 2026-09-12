# -*- coding: utf-8 -*-
u"""build_inj.ps1 에 `-NoDeploy` 스위치를 추가한다. (2026-09-12)

★왜 — 검증 모드(`tfm2_judge_verify`)처럼 **빌드만 하고 배포는 유저 확인 후** 해야 하는 경우가 있다.
  지금은 배포 생략 수단이 없어서, 에이전트가 **스크립트를 우회해 rustc 를 손으로 재현**했다.
  그게 바로 param 블록 주석이 경고하는 상황이다 —
  「2026-09-02: 이전엔 수동 rustc + Copy-Item 으로 우회했고, 그 과정에서
    stale dll 복사·opt 플래그 누락 같은 사고가 반복됐다 — 같은 경로로 통일.」
  ⟹ 우회를 막는 방법은 「우회하지 마라」가 아니라 **정규 경로에 그 기능을 두는 것**이다.

⚠`-NoDeploy` 는 ①~③(rustc exit·stale·사이즈가드·신원검증)을 **전부 그대로 통과시킨** 뒤
  ④배포만 건너뛴다. 검증을 건너뛰는 스위치가 아니다.
"""
import io
import sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
P = r"C:\tfm2mods\build_inj.ps1"
s = io.open(P, encoding="utf-8-sig").read()
bom = io.open(P, "rb").read(3) == b"\xef\xbb\xbf"

OLD_PARAM = u"  [int]$MaxSize = 1300000\n)"
NEW_PARAM = (u"  [int]$MaxSize = 1300000,\n"
             u"  # ★빌드·검증만 하고 **배포는 건너뛴다**(2026-09-12 신설).\n"
             u"  #   용도 = 배포 전에 유저 확인이 필요한 모드(예: 게임 `.text` 를 패치하는 검증 모드).\n"
             u"  #   ⚠①rustc exit ②stale ③사이즈가드 ④신원검증은 **그대로 다 돈다** — 건너뛰는 것은 복사뿐이다.\n"
             u"  #   이 스위치가 없어서 에이전트가 rustc 를 손으로 재현하는 일이 생겼고,\n"
             u"  #   그게 param 주석이 경고하는 「수동 우회 → stale·플래그 누락」 경로다.\n"
             u"  [switch]$NoDeploy\n)")

OLD_DEP = u"try {\n  New-Item -ItemType Directory -Force -Path $dep | Out-Null"
NEW_DEP = (u"if ($NoDeploy) {\n"
           u"  Write-Output \"OK: built (NoDeploy) $ModId.dll = $sz bytes @ $($item.LastWriteTime) -> $out\"\n"
           u"  Write-Output \"      배포 안 함. 배포하려면 -NoDeploy 없이 다시 돌려라.\"\n"
           u"  exit 0\n"
           u"}\n\n"
           u"try {\n  New-Item -ItemType Directory -Force -Path $dep | Out-Null")

n = 0
for a, b in ((OLD_PARAM, NEW_PARAM), (OLD_DEP, NEW_DEP)):
    if a not in s:
        print(u"⛔문면 불일치 — 이미 고쳐졌나? 찾던 것:\n%r" % a[:80])
        sys.exit(1)
    s = s.replace(a, b, 1)
    n += 1
io.open(P, "w", encoding="utf-8-sig" if bom else "utf-8", newline="").write(s)
print(u"%d곳 수정 (BOM %s)" % (n, u"유지" if bom else u"없음"))
