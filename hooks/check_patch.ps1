# TFM2 게임 exe 패치 감지 — Claude Code SessionStart 훅.
# 게임 exe 크기가 마지막 마이그레이션 기준(baseline)과 다르면 경고를 stdout으로 출력(→세션 컨텍스트에 주입).
$ErrorActionPreference = 'SilentlyContinue'
$exe      = "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
$baseline = "C:\tfm2mods\.exe_baseline.txt"
if (-not (Test-Path $exe)) { exit 0 }                       # 게임 미설치/이동 → 조용
$cur = (Get-Item $exe).Length
if (-not (Test-Path $baseline)) {                            # 최초 실행 → 기준만 기록
  Set-Content -Path $baseline -Value "$cur" -Encoding utf8
  exit 0
}
$old = ([string](Get-Content $baseline -Raw)).Trim()
if ("$cur" -ne $old) {
  Write-Output "[TFM2 PATCH] 게임 exe 크기 변경 감지: $old -> $cur bytes. 하드코딩 RVA가 어긋났을 수 있음 -> /migrate 로 재탐색 필요. (마이그 완료 후 baseline 자동 갱신됨)"
}
exit 0
