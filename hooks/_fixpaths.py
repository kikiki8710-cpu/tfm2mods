# -*- coding: utf-8 -*-
u"""훅 3개의 **하드코딩 경로**를 사용자 독립 해석으로 바꾼다. (2026-09-12 /dream)

★왜 — `.claude\\settings.local.json` 이 등록한 훅 4개 중 셋이
  `C:\\Users\\dev\\.claude\\projects\\C--Users-dev-Desktop-claude-tfm2\\memory` 를
  하드코딩하고 있었다. 실제 홈은 `C:\\Users\\jungs` 이고 `C:\\Users\\dev` 는 **없다.**
  `-ErrorAction SilentlyContinue` 탓에 `Get-Item`/`Test-Path` 가 조용히 실패하고
  `exit 0` 으로 끝나서 **세 훅이 전부 무동작**이었다:
    kb_lint     → 크기 경고 0건 (그 사이 메모리 29개가 상한을 250KB 초과)
    inject_done → DONE.md 주입 0 (§7 재작업 방지의 자동층이 통째로 죽어 있었다)
    rework_guard→ 심볼 대조 0
  ⟹ 이 프로젝트가 12라운드 내내 잡아 온 **「조용한 no-op」** 이 인프라 자신에 있었다.

★고침 방침 — `jungs` 를 다시 하드코딩하지 않는다. 머신·계정이 바뀌어도 살아 있어야
  같은 사고가 재발하지 않는다. `$env:USERPROFILE` 에서 출발해 프로젝트 폴더를 **찾아서** 쓴다.
  ⚠worktree 세션은 `…-Desktop-claude-tfm2--claude-worktrees-…` 로 따로 생기므로
    `-like "*-Desktop-claude-tfm2"` 로 **base 만** 고른다(정본은 base).
"""
import io
import os
import sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
H = r"C:\tfm2mods\hooks"

OLD_MEM = r"C:\Users\dev\.claude\projects\C--Users-dev-Desktop-claude-tfm2\memory"
OLD_ANA = r"C:\Users\dev\Desktop\claude\tfm2\팀파매2모드 분석"

# 각 훅 머리에 끼울 해석기. `$mem`(메모리 폴더)·`$anaRoot`(ANA 폴더)를 만든다.
RESOLVER = u"""
# ★경로 해석 — 하드코딩 금지. (2026-09-12 /dream: `C:\\Users\\dev` 하드코딩으로 이 훅이
#   오랫동안 **무동작**이었다. 그 사이 메모리 29개가 15KB 상한을 총 250KB 초과했는데
#   아무 경고도 안 떴다. 계정·머신이 바뀌어도 살아 있도록 실행 시점에 찾는다.)
$mem = ''
$projRoot = Join-Path $env:USERPROFILE '.claude\\projects'
$proj = Get-ChildItem $projRoot -Directory -ErrorAction SilentlyContinue |
        Where-Object { $_.Name -like '*-Desktop-claude-tfm2' } | Select-Object -First 1
if ($proj) { $mem = Join-Path $proj.FullName 'memory' }
$anaRoot = Join-Path $env:USERPROFILE 'Desktop\\claude\\tfm2\\팀파매2모드 분석'
"""


def patch(name, subs, resolver_after_line=4):
    p = os.path.join(H, name)
    raw = io.open(p, encoding="utf-8-sig").read()
    bom = io.open(p, "rb").read(3) == b"\xef\xbb\xbf"
    before = raw
    for a, b in subs:
        raw = raw.replace(a, b)
    if raw == before:
        print(u"  %-20s 변경 없음(이미 고쳐졌나?)" % name)
        return False
    lines = raw.split("\n")
    # 해석기를 `$ErrorActionPreference` 줄 **뒤**에 넣는다(인코딩 설정보다 앞이어도 무방).
    at = next((i for i, l in enumerate(lines) if l.startswith("$ErrorActionPreference")), 2)
    lines.insert(at + 1, RESOLVER.strip("\n"))
    out = "\n".join(lines)
    io.open(p, "w", encoding="utf-8-sig" if bom else "utf-8", newline="").write(out)
    print(u"  %-20s 고침 (BOM %s)" % (name, u"유지" if bom else u"없음"))
    return True


n = 0
n += patch("kb_lint.ps1", [
    (u'"%s"' % OLD_MEM, u'$mem'),
    (u'"%s\\reimpl-tracker.md"' % OLD_ANA, u'(Join-Path $anaRoot \'reimpl-tracker.md\')'),
    # ★DONE.md 임계도 낡았다 — CLAUDE.md §8 이 2026-08-24 에 **행당 ≤150자**로 재설계했고
    #   총량 상한은 폐지됐다. 45KB 를 그대로 두면 172KB 인 현재 정상 파일이 거짓 경고를 낸다.
    #   ⟹ 바이트 경고를 **행 폭 경고**로 교체한다. ⚠행 폭은 **문자** 기준이다(한글 3바이트).
    (u'$f = Get-Item "$mem\\DONE.md" -ErrorAction SilentlyContinue\n'
     u'if ($f -and $f.Length -gt 45KB) { $warn += "DONE.md $([math]::Round($f.Length/1KB))KB '
     u'(상한 45KB - CLAUDE.md §8, 2026-08-07 정정)" }',
     u'$dn = Join-Path $mem \'DONE.md\'\n'
     u'if (Test-Path $dn) {\n'
     u'  # ★총 바이트 상한은 2026-08-24 에 폐지됐다(판정이 많은 것은 레지스트리의 목적).\n'
     u'  #   현행 규약 = **행당 ≤150자**. ⚠문자 기준 — 바이트로 세면 한글이 과대계상된다.\n'
     u'  $long = @(Get-Content $dn -Encoding utf8 | Where-Object { $_.TrimEnd().Length -gt 150 })\n'
     u'  if ($long.Count -gt 0) { $warn += "DONE.md 행당 150자 초과 $($long.Count)행 '
     u'(CLAUDE.md §8 - 판정을 산문에 묻지 말 것)" }\n'
     u'}'),
])
n += patch("inject_done.ps1", [
    (u'"%s\\DONE.md"' % OLD_MEM, u'(Join-Path $mem \'DONE.md\')'),
    (u'"%s\\INDEX.md"' % OLD_MEM, u'(Join-Path $mem \'INDEX.md\')'),
])
n += patch("rework_guard.ps1", [
    (u'"%s\\DONE.md"' % OLD_MEM, u'(Join-Path $mem \'DONE.md\')'),
])
print(u"\n%d개 파일 수정" % n)
