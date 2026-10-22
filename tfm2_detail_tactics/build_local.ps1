# 별도 산출물 디렉터리에 로컬 빌드만 수행한다. 게임 폴더 복사·실행은 하지 않는다.
param([switch]$RunTests)
$ErrorActionPreference = 'Stop'
$projectRoot = $PSScriptRoot
$cargoPath = Join-Path $env:USERPROFILE '.cargo/bin/cargo.exe'
if (-not (Test-Path -LiteralPath $cargoPath)) { throw "cargo.exe 없음: $cargoPath" }
Push-Location -LiteralPath $projectRoot
try {
    $env:PATH = (Split-Path -Parent $cargoPath) + ';' + $env:PATH
    if ($RunTests) {
        & $cargoPath '+nightly-2026-05-24' test --lib --offline -- --test-threads=1
        if ($LASTEXITCODE -ne 0) { throw '유닛테스트 실패' }
    }
    & $cargoPath '+nightly-2026-05-24' rustc --release --lib --offline -- -C linker-flavor=lld-link -C linker=rust-lld
    if ($LASTEXITCODE -ne 0) { throw 'DLL 빌드 실패' }
    Get-Item -LiteralPath (Join-Path $projectRoot 'target/release/tfm2_detail_tactics.dll') | Select-Object FullName,Length,LastWriteTime
} finally { Pop-Location }
