# 대용량 데이터(버전 백업·빌드 산출물·로그)를 E:\claude\tfm2 로 옮긴다 (2026-10-21).
# 폴더: robocopy 복사 → 파일 수·바이트 대조 → 원본 삭제 → 원위치에 정션(mklink /J) — 기존 경로를 쓰는 도구·빌드가 그대로 동작.
# 파일: 복사 → 크기 대조 → 원본 삭제 → 원래 폴더의 _MOVED_TO_E.txt 에 기록.
# 사용: powershell -File move_to_e.ps1 -Plan <move_plan.json> [-Skip <정규식>] [-DryRun]
param([Parameter(Mandatory)][string]$Plan, [string]$Skip = '', [switch]$DryRun)
$ErrorActionPreference = 'Stop'
$manifest = 'E:\claude\tfm2\MOVE_MANIFEST.csv'
if (-not (Test-Path $manifest)) { 'utc,kind,type,source,dest,files,bytes,result' | Out-File -FilePath $manifest -Encoding utf8 }
$p = Get-Content -LiteralPath $Plan -Raw -Encoding UTF8 | ConvertFrom-Json

function Stat($path) {
    $f = Get-ChildItem -LiteralPath $path -Recurse -File -Force -ErrorAction SilentlyContinue
    [pscustomobject]@{ N = @($f).Count; B = [int64](($f | Measure-Object Length -Sum).Sum) }
}
function Log($kind, $type, $src, $dst, $n, $b, $res) {
    $line = '{0},{1},{2},"{3}","{4}",{5},{6},{7}' -f (Get-Date).ToUniversalTime().ToString('o'), $kind, $type, $src, $dst, $n, $b, $res
    Add-Content -LiteralPath $manifest -Value $line -Encoding utf8
    Write-Output "$res $type $src"
}

foreach ($d in $p.dirs) {
    $kind, $src, $dst = $d[0], $d[1], $d[2]
    if ($Skip -and $src -match $Skip) { continue }
    $item = Get-Item -LiteralPath $src -Force -ErrorAction SilentlyContinue
    if (-not $item) { continue }
    if ($item.Attributes -band [IO.FileAttributes]::ReparsePoint) { Log $kind dir $src $dst 0 0 'SKIP_already_link'; continue }
    if ($DryRun) { Write-Output "DRY dir $src -> $dst"; continue }
    $s = Stat $src
    New-Item -ItemType Directory -Force -Path $dst | Out-Null
    robocopy $src $dst /E /COPY:DAT /DCOPY:T /R:1 /W:1 /NFL /NDL /NP /NJH /NJS /MT:8 | Out-Null
    if ($LASTEXITCODE -ge 8) { Log $kind dir $src $dst $s.N $s.B "FAIL_robocopy_$LASTEXITCODE"; continue }
    $t = Stat $dst
    if ($t.N -lt $s.N -or $t.B -lt $s.B) { Log $kind dir $src $dst $s.N $s.B "FAIL_verify_$($t.N)_$($t.B)"; continue }
    Remove-Item -LiteralPath $src -Recurse -Force
    cmd /c mklink /J "$src" "$dst" | Out-Null
    if (-not (Test-Path -LiteralPath $src)) { Log $kind dir $src $dst $s.N $s.B 'FAIL_junction'; continue }
    Log $kind dir $src $dst $s.N $s.B 'OK'
}

foreach ($f in $p.files) {
    $kind, $src, $dst = $f[0], $f[1], $f[2]
    if ($Skip -and $src -match $Skip) { continue }
    if (-not (Test-Path -LiteralPath $src)) { continue }
    if ($DryRun) { Write-Output "DRY file $src -> $dst"; continue }
    $len = (Get-Item -LiteralPath $src -Force).Length
    New-Item -ItemType Directory -Force -Path (Split-Path $dst) | Out-Null
    try { Copy-Item -LiteralPath $src -Destination $dst -Force } catch { Log $kind file $src $dst 1 $len "FAIL_copy_locked"; continue }
    if ((Get-Item -LiteralPath $dst).Length -ne $len) { Log $kind file $src $dst 1 $len 'FAIL_verify'; continue }
    try { Remove-Item -LiteralPath $src -Force } catch { Remove-Item -LiteralPath $dst -Force; Log $kind file $src $dst 1 $len 'FAIL_delete_locked'; continue }
    Add-Content -LiteralPath (Join-Path (Split-Path $src) '_MOVED_TO_E.txt') -Value ("{0} -> {1}" -f (Split-Path $src -Leaf), $dst) -Encoding utf8
    Log $kind file $src $dst 1 $len 'OK'
}
