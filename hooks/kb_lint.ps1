# SessionStart 훅 — 지식베이스 크기 린트. 상한 초과 시 /dream 정리 권고를 세션 컨텍스트에 주입(재비대화 조기 감지).
# 상한 근거: 2026-07-11 재편 시 INDEX 311KB/MEMORY 21KB까지 비대해져 조회 실패·재작업 유발했음.
$ErrorActionPreference = 'SilentlyContinue'
# ★경로 해석 — 하드코딩 금지. (2026-09-12 /dream: `C:\Users\dev` 하드코딩으로 이 훅이
#   오랫동안 **무동작**이었다. 그 사이 메모리 29개가 15KB 상한을 총 250KB 초과했는데
#   아무 경고도 안 떴다. 계정·머신이 바뀌어도 살아 있도록 실행 시점에 찾는다.)
$mem = ''
$projRoot = Join-Path $env:USERPROFILE '.claude\projects'
$proj = Get-ChildItem $projRoot -Directory -ErrorAction SilentlyContinue |
        Where-Object { $_.Name -like '*-Desktop-claude-tfm2' } | Select-Object -First 1
if ($proj) { $mem = Join-Path $proj.FullName 'memory' }
$anaRoot = Join-Path $env:USERPROFILE 'Desktop\claude\tfm2\팀파매2모드 분석'
try { [Console]::OutputEncoding = [System.Text.Encoding]::UTF8 } catch {}
$mem = $mem
$warn = @()
$f = Get-Item "$mem\INDEX.md" -ErrorAction SilentlyContinue
if ($f -and $f.Length -gt 50KB) { $warn += "INDEX.md $([math]::Round($f.Length/1KB))KB (상한 50KB - CLAUDE.md §8)" }
$f = Get-Item "$mem\MEMORY.md" -ErrorAction SilentlyContinue
if ($f -and $f.Length -gt 5KB) { $warn += "MEMORY.md $([math]::Round($f.Length/1KB))KB (상한 5KB)" }
$dn = Join-Path $mem 'DONE.md'
if (Test-Path $dn) {
  # ★총 바이트 상한은 2026-08-24 에 폐지됐다(판정이 많은 것은 레지스트리의 목적).
  #   현행 규약 = **행당 ≤150자**. ⚠문자 기준 — 바이트로 세면 한글이 과대계상된다.
  $long = @(Get-Content $dn -Encoding utf8 | Where-Object { $_.TrimEnd().Length -gt 150 })
  if ($long.Count -gt 0) { $warn += "DONE.md 행당 150자 초과 $($long.Count)행 (CLAUDE.md §8 - 판정을 산문에 묻지 말 것)" }
}
$f = Get-Item "$mem\CURRENT.md" -ErrorAction SilentlyContinue
if ($f -and $f.Length -gt 10KB) { $warn += "CURRENT.md $([math]::Round($f.Length/1KB))KB (역할=버전·활성모드뿐, 10KB 넘으면 이력이 침입한 것 - 경위는 MIGRATION/메모리 본문으로)" }
$big = Get-ChildItem $mem -Filter "*.md" -ErrorAction SilentlyContinue | Where-Object { $_.Name -notin @('INDEX.md','MEMORY.md','DONE.md','CURRENT.md') -and $_.Length -gt 15KB }
if ($big) {
  $top = ($big | Sort-Object Length -Descending | Select-Object -First 5 | ForEach-Object { "$($_.Name)($([math]::Round($_.Length/1KB))KB)" }) -join ', '
  $warn += "메모리 파일 15KB 초과 $(@($big).Count)개: $top"
}
$tr = Get-Item (Join-Path $anaRoot 'reimpl-tracker.md') -ErrorAction SilentlyContinue
if ($tr -and $tr.Length -gt 100KB) { $warn += "reimpl-tracker.md $([math]::Round($tr.Length/1KB))KB (상한 100KB - 롤오버 필요)" }
if ($warn.Count -gt 0) {
  Write-Output "[KB-LINT] 지식베이스 크기 상한 초과 - 비대해지면 조회가 실패해 재작업이 늘어남. /dream 정리 권고:"
  $warn | ForEach-Object { Write-Output ("  - " + $_) }
}
exit 0
