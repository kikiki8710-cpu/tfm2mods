# SessionStart 훅 — DONE.md(재시도 금지/완료 단일 레지스트리)를 세션 시작 시 통째로 주입.
# 구버전(INDEX grep -First 20)은 최신 DONE이 뒤에 append되면 주입에서 탈락하는 결함이 있어 교체됨(2026-07-11).
$ErrorActionPreference = 'SilentlyContinue'
try { [Console]::OutputEncoding = [System.Text.Encoding]::UTF8 } catch {}
$done = "C:\Users\dev\.claude\projects\C--Users-dev-Desktop-claude-tfm2\memory\DONE.md"
if (Test-Path $done) {
  $txt = Get-Content $done -Raw -Encoding utf8
  # frontmatter(---...---) 블록은 주입에서 제외(노이즈)
  $txt = [regex]::Replace($txt, '^(\xEF\xBB\xBF)?---\r?\n[\s\S]*?\r?\n---\r?\n', '')
  if ($txt.Length -gt 12000) { $txt = $txt.Substring(0, 12000) + "`n...(잘림 - DONE.md 비대. /dream 정리 필요)" }
  Write-Output "[TFM2 DONE 레지스트리 - 아래 항목은 재조사/재구현 금지. 상세=근거파일. 새 판정은 record-keeper가 표 맨 위에 추가]"
  Write-Output $txt
} else {
  # 폴백(전환기 안전망): DONE.md 미생성이면 INDEX에서 추출 — 최신 항목이 뒤에 있으므로 -Last 사용
  $index = "C:\Users\dev\.claude\projects\C--Users-dev-Desktop-claude-tfm2\memory\INDEX.md"
  if (Test-Path $index) {
    $lines = Get-Content $index -Encoding utf8 | Where-Object { $_ -match 'DONE|재시도\s*금지' } | Select-Object -Last 20
    if ($lines) {
      Write-Output "[TFM2 완료 목록(INDEX 폴백 - DONE.md 미생성 상태)]"
      $lines | ForEach-Object { Write-Output ("  " + $_.Trim()) }
    }
  }
}
exit 0
