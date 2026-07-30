# save_probe 재빌드 — 게임 패치로 세이브 포맷/SDK 가 바뀌면 이걸로 새 SDK에 맞춰 다시 빌드.
#   결과물은 payload\tfm2_meta_dashboard\tools\tfm2_save_probe.exe 로 떨어져 apply.ps1 이 배포.
#
# 사용법:  powershell -ExecutionPolicy Bypass -File build_probe.ps1
#          powershell -ExecutionPolicy Bypass -File build_probe.ps1 -Sdk "C:\tfm2mods\sdk_XXX\mod-sdk" -Toolchain nightly-YYYY-MM-DD
#
# ★함정: save_probe 는 game_core rlib 을 링크하므로 rlib 을 빌드한 것과 "동일한" rustc(nightly) 필요.
#        SDK 의 toolchain_version.txt (rustc 빌드일 +1일)에서 자동 도출. 모드 DLL 빌드용 툴체인과 다를 수 있음.
param(
    [string]$Sdk = "C:\tfm2mods\sdk_050_hotfix2\mod-sdk",
    [string]$Toolchain = ""
)
$ErrorActionPreference = "Stop"
$kit = Split-Path -Parent $MyInvocation.MyCommand.Path
$source = Join-Path $kit "tfm2_save_probe_047_050hf2.rs"
$output = Join-Path $kit "payload\tfm2_meta_dashboard\tools\tfm2_save_probe.exe"
if (-not (Test-Path $source)) { throw "probe 소스 없음: $source" }
if (-not (Test-Path $Sdk))    { throw "SDK 없음: $Sdk  (현행 SDK 는 MEM\CURRENT.md 참고)" }

$deps = Join-Path $Sdk "deps"; $native = Join-Path $Sdk "native"
$gc = Get-ChildItem -LiteralPath $deps -Filter "libgame_core-*.rlib"   | Select-Object -First 1
$ec = Get-ChildItem -LiteralPath $deps -Filter "libengine_core-*.rlib" | Select-Object -First 1
$fl = Get-ChildItem -LiteralPath $deps -Filter "libflate2-*.rlib"      | Select-Object -First 1
if (-not $gc) { throw "libgame_core-*.rlib 없음 ($deps)" }
if (-not $ec) { throw "libengine_core-*.rlib 없음 ($deps)" }
if (-not $fl) { throw "libflate2-*.rlib 없음 ($deps)" }

# 툴체인 자동 도출: toolchain_version.txt 의 rustc 빌드일 +1일 = nightly 날짜.
if (-not $Toolchain) {
    $tv = Join-Path $Sdk "toolchain_version.txt"
    if (Test-Path $tv) {
        $txt = Get-Content -LiteralPath $tv -Raw
        if ($txt -match "\((?<hash>[0-9a-f]+)\s+(?<date>\d{4}-\d{2}-\d{2})\)") {
            $d = [DateTime]::ParseExact($Matches.date, "yyyy-MM-dd", [Globalization.CultureInfo]::InvariantCulture)
            $Toolchain = "nightly-" + $d.AddDays(1).ToString("yyyy-MM-dd")
        }
    }
    if (-not $Toolchain) { $Toolchain = "nightly-2026-05-24" }  # sdk_050_hotfix2 기준 폴백
}
Write-Host "SDK: $Sdk" -ForegroundColor Cyan
Write-Host "Toolchain: $Toolchain" -ForegroundColor Cyan

$externs = @("--extern","game_core=$($gc.FullName)","--extern","engine_core=$($ec.FullName)","--extern","flate2=$($fl.FullName)")
# ★0.5.3(2026-07-30): MSVC 2019 link.exe(14.29.30133) 가 sdk_053 rlib 재스캔 중 LNK1107 로 죽는다
#   (rlib 자체는 정상 — 미해결 심볼이 생겨 아카이브를 재스캔할 때만 발생) ⟹ 툴체인 동봉 rust-lld 로 링크.
$linkerArgs = @("-C","linker-flavor=lld-link","-C","linker=rust-lld")
$rustup = Join-Path $env:USERPROFILE ".cargo\bin\rustup.exe"
New-Item -ItemType Directory -Force -Path (Split-Path -Parent $output) | Out-Null
if (Test-Path $rustup) {
    & $rustup run $Toolchain rustc -O --edition 2021 $source -o $output -L "dependency=$deps" -L "native=$native" @externs @linkerArgs
} else {
    & rustc -O --edition 2021 $source -o $output -L "dependency=$deps" -L "native=$native" @externs @linkerArgs
}
if ($LASTEXITCODE -ne 0) { Write-Host "빌드 실패" -ForegroundColor Red; exit $LASTEXITCODE }
Write-Host "Built $output" -ForegroundColor Green
Write-Host "다음: apply.ps1 로 대시보드에 배포 → 세이브 재선택 후 decode ok / base_network.debug.txt 확인." -ForegroundColor Green
