# SessionStart 훅 — 지식베이스 크기 린트. 상한 초과 시 /dream 정리 권고를 세션 컨텍스트에 주입(재비대화 조기 감지).
# 상한 근거: 2026-07-11 재편 시 INDEX 311KB/MEMORY 21KB까지 비대해져 조회 실패·재작업 유발했음.
$ErrorActionPreference = 'SilentlyContinue'
try { [Console]::OutputEncoding = [System.Text.Encoding]::UTF8 } catch {}
$mem = "C:\Users\dev\.claude\projects\C--Users-dev-Desktop-claude-tfm2\memory"
$warn = @()
$f = Get-Item "$mem\INDEX.md" -ErrorAction SilentlyContinue
if ($f -and $f.Length -gt 40KB) { $warn += "INDEX.md $([math]::Round($f.Length/1KB))KB (상한 40KB)" }
$f = Get-Item "$mem\MEMORY.md" -ErrorAction SilentlyContinue
if ($f -and $f.Length -gt 5KB) { $warn += "MEMORY.md $([math]::Round($f.Length/1KB))KB (상한 5KB)" }
$f = Get-Item "$mem\DONE.md" -ErrorAction SilentlyContinue
if ($f -and $f.Length -gt 6KB) { $warn += "DONE.md $([math]::Round($f.Length/1KB))KB (상한 6KB)" }
$big = Get-ChildItem $mem -Filter "*.md" -ErrorAction SilentlyContinue | Where-Object { $_.Name -notin @('INDEX.md','MEMORY.md','DONE.md','CURRENT.md') -and $_.Length -gt 15KB }
if ($big) {
  $top = ($big | Sort-Object Length -Descending | Select-Object -First 5 | ForEach-Object { "$($_.Name)($([math]::Round($_.Length/1KB))KB)" }) -join ', '
  $warn += "메모리 파일 15KB 초과 $(@($big).Count)개: $top"
}
$tr = Get-Item "C:\Users\dev\Desktop\claude\tfm2\팀파매2모드 분석\reimpl-tracker.md" -ErrorAction SilentlyContinue
if ($tr -and $tr.Length -gt 100KB) { $warn += "reimpl-tracker.md $([math]::Round($tr.Length/1KB))KB (상한 100KB - 롤오버 필요)" }
if ($warn.Count -gt 0) {
  Write-Output "[KB-LINT] 지식베이스 크기 상한 초과 - 비대해지면 조회가 실패해 재작업이 늘어남. /dream 정리 권고:"
  $warn | ForEach-Object { Write-Output ("  - " + $_) }
}
exit 0
