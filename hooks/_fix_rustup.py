# -*- coding: utf-8 -*-
u"""build_inj.ps1 이 `rustup` 을 **스스로 찾게** 한다. (2026-09-12)

★증상 — Claude 세션 셸에서 이 스크립트를 돌리면
  `'rustup'은(는) 내부 또는 외부 명령... 이 아닙니다` 로 죽는다(rustc exit 1).
  그런데 스크립트는 stderr 를 `error\\[|^error:` 로만 grep 하므로
  **`=== BUILD FAILED (rustc exit 1) ===` 만 찍히고 사유가 안 보인다.**

★왜 중요한가 — 이 때문에 에이전트가 **스크립트를 우회해 rustc 명령줄을 손으로 재현**했다.
  그건 param 블록 주석이 스스로 경고하는 경로다:
  「2026-09-02: 이전엔 수동 rustc + Copy-Item 으로 우회했고, 그 과정에서
    stale dll 복사·opt 플래그 누락 같은 사고가 반복됐다 — 같은 경로로 통일.」
  ⟹ 우회를 막으려면 **정규 경로가 이 환경에서 그냥 돌아야** 한다.

고침 둘:
  ① `rustup` 을 PATH → `%USERPROFILE%\\.cargo\\bin\\rustup.exe` 순으로 해석해 절대경로로 호출
  ② rustc 가 실패했는데 `error:` 줄이 없으면 **stderr 앞부분을 그대로 출력**(무언의 실패 방지)
"""
import io
import sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
P = r"C:\tfm2mods\build_inj.ps1"
s = io.open(P, encoding="utf-8-sig").read()
bom = io.open(P, "rb").read(3) == b"\xef\xbb\xbf"

# ── ① rustup 해석 ────────────────────────────────────────────────────
OLD_CALL = u'cmd /c "rustup run nightly-2026-05-24 rustc'
NEW_RESOLVE = (
    u"# ★`rustup` 해석 — PATH 에 없을 수 있다(Claude 세션 셸이 사용자 PATH 를 온전히 안 물려받는다).\n"
    u"#   PATH 우선, 없으면 `%USERPROFILE%\\.cargo\\bin\\rustup.exe`. 둘 다 없으면 **거기서 멈춘다**\n"
    u"#   (조용히 실패하면 `rustc exit 1` 만 남아 사유를 못 찾는다 — 실제로 그 일이 있었다).\n"
    u"$rustup = (Get-Command rustup -ErrorAction SilentlyContinue).Source\n"
    u"if (-not $rustup) { $rustup = Join-Path $env:USERPROFILE '.cargo\\bin\\rustup.exe' }\n"
    u"if (-not (Test-Path $rustup)) {\n"
    u"  Write-Output \"FAIL: rustup 을 못 찾았다 (PATH 에도, $env:USERPROFILE\\.cargo\\bin 에도 없음)\"\n"
    u"  exit 1\n"
    u"}\n")
NEW_CALL = u'cmd /c "`"$rustup`" run nightly-2026-05-24 rustc'

if OLD_CALL not in s:
    print(u"⛔rustc 호출 문면 불일치 — 이미 고쳐졌나?")
    sys.exit(1)
s = s.replace(OLD_CALL, NEW_RESOLVE + NEW_CALL, 1)

# ── ② 무언의 실패 방지 ───────────────────────────────────────────────
OLD_FAIL = (u'  Write-Output "=== BUILD FAILED (rustc exit $rc) ==="\n'
            u'  if (Test-Path $errf) { Get-Content $errf | Select-String -Pattern "error\\[|^error:" '
            u'| Select-Object -First 40 | ForEach-Object { Write-Output $_.Line } }\n'
            u'  exit 1')
NEW_FAIL = (u'  Write-Output "=== BUILD FAILED (rustc exit $rc) ==="\n'
            u'  $lines = @()\n'
            u'  if (Test-Path $errf) { $lines = @(Get-Content $errf | Select-String -Pattern "error\\[|^error:") }\n'
            u'  if ($lines.Count -gt 0) { $lines | Select-Object -First 40 | ForEach-Object { Write-Output $_.Line } }\n'
            u'  else {\n'
            u'    # ★`error:` 가 없는 실패도 있다(예: rustup 자체가 없어서 cmd 가 낸 메시지).\n'
            u'    #   그때 아무것도 안 찍으면 **사유 없는 실패**가 되고 우회를 유발한다.\n'
            u'    Write-Output "      (error: 패턴 없음 — stderr 앞부분 그대로)"\n'
            u'    if (Test-Path $errf) { Get-Content $errf | Select-Object -First 12 | ForEach-Object { Write-Output ("      " + $_) } }\n'
            u'    Write-Output "      stderr 전문: $errf"\n'
            u'  }\n'
            u'  exit 1')
if OLD_FAIL not in s:
    print(u"⚠실패보고 문면 불일치 — ①만 적용했다")
else:
    s = s.replace(OLD_FAIL, NEW_FAIL, 1)
    print(u"② 무언의 실패 방지 적용")

io.open(P, "w", encoding="utf-8-sig" if bom else "utf-8", newline="").write(s)
print(u"① rustup 해석 적용 (BOM %s)" % (u"유지" if bom else u"없음"))
