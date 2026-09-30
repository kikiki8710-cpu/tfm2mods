$ErrorActionPreference = "Stop"

$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
$ModDir = Join-Path $Root "tfm2_champ_pos_lock_rework"
$SdkRoot = Join-Path $Root "sdk_060_stable"
$SdkTarget = Join-Path $SdkRoot "mod-api-stable"
$SdkCargo = Join-Path $SdkTarget "Cargo.toml"
$DistRoot = Join-Path $Root "dist"
$DistMod = Join-Path $DistRoot "tfm2_champ_pos_lock_rework"
$DistZip = Join-Path $DistRoot "tfm2_champ_pos_lock_rework_v1.0.0.zip"

Write-Host ""
Write-Host "=== TFM2 Champion Position Lock: Rework v1.0.0 ===" -ForegroundColor Cyan
Write-Host "Root: $Root"

function Find-Sdk {
    $candidates = @()
    if ($env:TFM2_STABLE_SDK_DIR) {
        $candidates += $env:TFM2_STABLE_SDK_DIR
        $candidates += (Join-Path $env:TFM2_STABLE_SDK_DIR "mod-api-stable")
    }
    $candidates += @(
        (Join-Path $Root "mod-sdk-stable\mod-api-stable"),
        "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mod-sdk-stable\mod-api-stable",
        "C:\Program Files\Steam\steamapps\common\Teamfight Manager2\mod-sdk-stable\mod-api-stable",
        "D:\SteamLibrary\steamapps\common\Teamfight Manager2\mod-sdk-stable\mod-api-stable",
        "E:\SteamLibrary\steamapps\common\Teamfight Manager2\mod-sdk-stable\mod-api-stable",
        "F:\SteamLibrary\steamapps\common\Teamfight Manager2\mod-sdk-stable\mod-api-stable"
    )
    foreach ($c in $candidates) {
        if ($c -and (Test-Path (Join-Path $c "Cargo.toml"))) { return $c }
    }
    return $null
}

if (-not (Test-Path $SdkCargo)) {
    Write-Host "[1/4] Official stable SDK not bundled; locating mod-api-stable..." -ForegroundColor Yellow
    $found = Find-Sdk

    if (-not $found) {
        $searchDirs = @($Root, (Join-Path $env:USERPROFILE "Downloads"))
        $zip = Get-ChildItem -Path $searchDirs -Filter "mod-sdk-stable*.zip" -File -ErrorAction SilentlyContinue |
            Sort-Object LastWriteTime -Descending | Select-Object -First 1
        if ($zip) {
            Write-Host "SDK zip found: $($zip.FullName)"
            $tmp = Join-Path $env:TEMP ("tfm2_sdk_" + [guid]::NewGuid().ToString("N"))
            New-Item -ItemType Directory -Force -Path $tmp | Out-Null
            Expand-Archive -LiteralPath $zip.FullName -DestinationPath $tmp -Force
            $cargoToml = Get-ChildItem -Path $tmp -Filter "Cargo.toml" -Recurse -File |
                Where-Object { $_.Directory.Name -eq "mod-api-stable" } |
                Select-Object -First 1
            if ($cargoToml) { $found = $cargoToml.Directory.FullName }
        }
    }

    if (-not $found) {
        Write-Host ""
        Write-Host "[ERROR] mod-api-stable SDK를 찾지 못했습니다." -ForegroundColor Red
        Write-Host "게임 설치 폴더의 mod-sdk-stable\mod-api-stable 또는 mod-sdk-stable*.zip을 준비한 뒤 다시 실행하세요."
        exit 1
    }

    New-Item -ItemType Directory -Force -Path $SdkRoot | Out-Null
    if (Test-Path $SdkTarget) { Remove-Item -Recurse -Force $SdkTarget }
    Copy-Item -Recurse -Force $found $SdkTarget
}
Write-Host "[1/4] SDK OK: $SdkCargo" -ForegroundColor Green

$cargoCmd = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $cargoCmd) {
    $fallback = Join-Path $env:USERPROFILE ".cargo\bin\cargo.exe"
    if (Test-Path $fallback) { $CargoExe = $fallback }
    else {
        Write-Host "[ERROR] cargo.exe를 찾지 못했습니다. Rust 설치 후 다시 실행하세요." -ForegroundColor Red
        exit 1
    }
} else { $CargoExe = $cargoCmd.Source }

# Avoid embedding the local source directory in panic/debug strings where possible.
$oldRustFlags = $env:RUSTFLAGS
$env:RUSTFLAGS = "--remap-path-prefix=$Root=src"

Write-Host "[2/4] Building Rework v1.0.0 release DLL..." -ForegroundColor Cyan
Push-Location $ModDir
try {
    & $CargoExe build --release
    if ($LASTEXITCODE -ne 0) { throw "cargo build failed ($LASTEXITCODE)" }
} finally {
    Pop-Location
    $env:RUSTFLAGS = $oldRustFlags
}

$dll = Join-Path $ModDir "target\release\tfm2_champ_pos_lock_rework.dll"
if (-not (Test-Path $dll)) {
    Write-Host "[ERROR] Build finished but DLL was not found: $dll" -ForegroundColor Red
    exit 1
}
Write-Host "[2/4] DLL OK: $dll" -ForegroundColor Green

Write-Host "[3/4] Creating install-ready mod folder..." -ForegroundColor Cyan
if (Test-Path $DistMod) { Remove-Item -Recurse -Force $DistMod }
New-Item -ItemType Directory -Force -Path (Join-Path $DistMod "text") | Out-Null
Copy-Item -Force $dll (Join-Path $DistMod "tfm2_champ_pos_lock_rework.dll")
Copy-Item -Force (Join-Path $ModDir "mod.mod_info") (Join-Path $DistMod "mod.mod_info")
Copy-Item -Force (Join-Path $ModDir "mod.override_info") (Join-Path $DistMod "mod.override_info")
Copy-Item -Force (Join-Path $ModDir "assets\text\poslock.i18n") (Join-Path $DistMod "text\poslock.i18n")

$cfg = @'
# tfm2_champ_pos_lock_rework - Champion Position Lock: Rework
# Position lists are edited in-game: Settings -> Gameplay -> Position Lock.
enabled=1
debug=0
ai_pick_gate=1
ai_assign_mask=1
user_pick_block=1
dump_ui=0
log_lineups=0
swap_force=1
swap_auto=1
perf_log=0
'@
Set-Content -LiteralPath (Join-Path $DistMod "tfm2_champ_pos_lock_rework.cfg") -Value $cfg -Encoding ASCII

if (Test-Path $DistZip) { Remove-Item -Force $DistZip }
Compress-Archive -Path $DistMod -DestinationPath $DistZip -CompressionLevel Optimal
Write-Host "[3/4] Package OK: $DistZip" -ForegroundColor Green

Write-Host "[4/4] SHA256" -ForegroundColor Cyan
$hash = Get-FileHash -Algorithm SHA256 $DistZip
Write-Host $hash.Hash
Write-Host ""
Write-Host "BUILD SUCCESS" -ForegroundColor Green
Write-Host "Install-ready ZIP:"
Write-Host "  $DistZip"
Write-Host ""
