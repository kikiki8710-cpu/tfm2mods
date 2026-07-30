# TFM2.gg 대시보드 병합본 재적용 스크립트.
#   Steam 이 워크샵 아이템을 재동기화해 파일을 되돌렸을 때, 오늘(2026-07-14) 작업분을 한 번에 재적용.
#   payload\ (타겟 구조 미러)를 워크샵 대시보드에 복사 + BOM/크기 검증 + 챔프 이미지 재수집.
#
# 사용법:  powershell -ExecutionPolicy Bypass -File apply.ps1            (자동탐지)
#          powershell -ExecutionPolicy Bypass -File apply.ps1 -DashApp "<...\DashboardApp\resources\app>"
param(
    [string]$DashApp = "",
    [string]$SteamApps = "C:\Program Files (x86)\Steam\steamapps"
)
$ErrorActionPreference = "Stop"
$kit = Split-Path -Parent $MyInvocation.MyCommand.Path
$payload = Join-Path $kit "payload"
if (-not (Test-Path $payload)) { throw "payload 폴더가 없습니다: $payload" }

# 1) 타겟(resources\app) 자동탐지: workshop\content\3009300\*\DashboardApp\resources\app 중
#    main.cjs + tfm2_meta_dashboard\refresh_meta_dashboard.ps1 이 있는 곳.
if (-not $DashApp) {
    $wc = Join-Path $SteamApps "workshop\content\3009300"
    if (Test-Path $wc) {
        Get-ChildItem -LiteralPath $wc -Directory -ErrorAction SilentlyContinue | ForEach-Object {
            $cand = Join-Path $_.FullName "DashboardApp\resources\app"
            if ((Test-Path (Join-Path $cand "main.cjs")) -and
                (Test-Path (Join-Path $cand "tfm2_meta_dashboard\refresh_meta_dashboard.ps1"))) {
                $DashApp = $cand
            }
        }
    }
}
if (-not $DashApp -or -not (Test-Path $DashApp)) {
    throw "대시보드 대상(resources\app)을 못 찾음. -DashApp 로 직접 지정하세요."
}
Write-Host "Target: $DashApp" -ForegroundColor Cyan

# 2) 기존 파일 백업(.premerge.bak, 최초 1회만)
$backupList = @(
    "main.cjs",
    "tfm2_meta_dashboard\app.js",
    "tfm2_meta_dashboard\index.html",
    "tfm2_meta_dashboard\styles.css",
    "tfm2_meta_dashboard\tools\build_meta_data.py",
    "tfm2_meta_dashboard\tools\tfm2_save_probe.exe"
)
foreach ($rel in $backupList) {
    $dst = Join-Path $DashApp $rel
    $bak = "$dst.premerge.bak"
    if ((Test-Path $dst) -and -not (Test-Path $bak)) { Copy-Item -LiteralPath $dst -Destination $bak -Force }
}

# 3) payload 미러 복사(그 외 파일은 보존)
Get-ChildItem -LiteralPath $payload -Recurse -File | ForEach-Object {
    $rel = $_.FullName.Substring($payload.Length).TrimStart('\')
    $dst = Join-Path $DashApp $rel
    New-Item -ItemType Directory -Force -Path (Split-Path -Parent $dst) | Out-Null
    Copy-Item -LiteralPath $_.FullName -Destination $dst -Force
    Write-Host "  copied $rel"
}

# 4) 검증: BOM 없음(게임/파서가 UTF-8 no-BOM 요구) + 크기 일치
$fail = 0
$textFiles = @("tfm2_meta_dashboard\app.js","tfm2_meta_dashboard\index.html","tfm2_meta_dashboard\styles.css","tfm2_meta_dashboard\tools\build_meta_data.py","main.cjs")
foreach ($rel in $textFiles) {
    $dst = Join-Path $DashApp $rel
    $bytes = [System.IO.File]::ReadAllBytes($dst) | Select-Object -First 3
    if ($bytes.Count -ge 3 -and $bytes[0] -eq 0xEF -and $bytes[1] -eq 0xBB -and $bytes[2] -eq 0xBF) {
        Write-Host "  ! BOM 감지: $rel" -ForegroundColor Red; $fail++
    }
    $srcLen = (Get-Item (Join-Path $payload $rel)).Length
    $dstLen = (Get-Item $dst).Length
    if ($srcLen -ne $dstLen) { Write-Host "  ! 크기 불일치: $rel ($srcLen vs $dstLen)" -ForegroundColor Red; $fail++ }
}

# 5) 챔프 이미지 재수집(선택): gen_champ_uv.py 가 있으면 최신 렌더(없으면 payload 의 baked 이미지 사용).
$gen = "C:\tfm2mods\tfm2_draft_overlay\gen_champ_uv.py"
if (Test-Path $gen) {
    $dashRoot = Join-Path $DashApp "tfm2_meta_dashboard"
    $env:TFM2_DASH_ROOT = $dashRoot
    try {
        & python $gen 2>&1 | Select-Object -Last 2 | ForEach-Object { Write-Host "  gen: $_" }
    } catch { Write-Host "  gen 스킵(python/PIL 없음)" -ForegroundColor Yellow }
    Remove-Item Env:TFM2_DASH_ROOT -ErrorAction SilentlyContinue
}

if ($fail -eq 0) {
    Write-Host "APPLY OK — 대시보드를 재시작하면 병합본이 적용됩니다." -ForegroundColor Green
} else {
    Write-Host "APPLY 경고 $fail 건 — 위 항목 확인 필요." -ForegroundColor Red
    exit 1
}
