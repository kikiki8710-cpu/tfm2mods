# build_full.ps1 — 게임 crate 전량(--extern) 주입 빌드·배포.
# 사용: powershell -File build_full.ps1 -Src <lib.rs 절대경로> -ModId <mod_id> [-MaxSize <bytes>]
#
# build_inj.ps1 과의 차이 — 왜 별도인가:
#   ① daram2 뷰플러스 계열은 `extern crate common/game_core/game_view/...` 를 직접 쓴다.
#      `-L dependency` 만으로는 해결되지 않아(E0463) 게임 crate 를 전부 --extern 으로 명시해야 한다.
#   ② serde_json/flate2 는 deps 에 2벌씩 있다. **게임이 실제로 링크하는 쪽**을 골라야 하며
#      (rustc 링커 인자 로그로 확인: serde_json-9ce4f02.../flate2-76adaee9...), 잘못 고르면 타입 불일치.
#   ③ 사이즈 가드가 파라미터(-MaxSize) — statistics/custom_tier/training 등 2~3MB 모드용.
# 안전장치(exit code·stale mtime·소스경로 신원검증·게임 락 처리)는 build_inj.ps1 과 동일하게 유지.
#
# ⚠ 링커: 0.5.3 부터 MSVC 2019 link.exe 는 rlib 재스캔 중 LNK1107 로 죽는다 → rust-lld 고정(build_inj.ps1 주석 참조).
param(
  [Parameter(Mandatory=$true)][string]$Src,
  [Parameter(Mandatory=$true)][string]$ModId,
  [int]$MaxSize = 5000000
)
$SDK  = "C:\tfm2mods\sdk_054\mod-sdk"
$DEPS = "$SDK\deps"; $NAT = "$SDK\native"

function Pick([string]$pat) { (Get-ChildItem "$DEPS\$pat")[0].FullName }
# 게임 crate (deps 에 1벌씩)
# ⚠ 각 항목을 괄호로 묶을 것 — `"a=" + (Pick x), "b=" + (Pick y)` 는 PowerShell 이 `+` 를 배열결합으로
#   파싱해 전 항목이 공백으로 이어붙은 **문자열 1개**가 된다(= --extern 접두가 첫 항목에만 붙음).
$X = @(
  ("mod_api="      + (Pick "libmod_api-*.rlib")),
  ("common="       + (Pick "libcommon-*.rlib")),
  ("engine_core="  + (Pick "libengine_core-*.rlib")),
  ("engine_ui="    + (Pick "libengine_ui-*.rlib")),
  ("engine_asset=" + (Pick "libengine_asset-*.rlib")),
  ("game_core="    + (Pick "libgame_core-*.rlib")),
  ("game_view="    + (Pick "libgame_view-*.rlib")),
  # ⚠ 2벌 존재 → 게임이 링크하는 해시로 고정
  "serde_json=$DEPS\libserde_json-9ce4f0220edb6ae7.rlib",
  "flate2=$DEPS\libflate2-76adaee9a71bfe42.rlib"
)
$externs = ($X | ForEach-Object { "--extern $_" }) -join " "

$work = Join-Path $env:TEMP "tfm2_build\$ModId`_$PID"
New-Item -ItemType Directory -Force -Path $work | Out-Null
$out  = "$work\$ModId.dll"
$errf = "$work\rustc_err.txt"
$dep  = "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\$ModId"

$started = Get-Date
cmd /c "rustup run nightly-2026-05-24 rustc --crate-type cdylib --edition 2021 -C opt-level=1 -C overflow-checks=off -C linker-flavor=lld-link -C linker=rust-lld -L dependency=$DEPS -L native=$NAT $externs $Src -o `"$out`" 2> `"$errf`""
$rc = $LASTEXITCODE

if ($rc -ne 0) {
  Write-Output "=== BUILD FAILED (rustc exit $rc) ==="
  if (Test-Path $errf) { Get-Content $errf | Select-String -Pattern "error\[|^error:" | Select-Object -First 40 | ForEach-Object { Write-Output $_.Line } }
  exit 1
}
if (-not (Test-Path $out)) { Write-Output "FAIL: $ModId.dll not produced"; exit 1 }
$item = Get-Item $out
if ($item.LastWriteTime -lt $started) { Write-Output "FAIL: stale dll (mtime $($item.LastWriteTime) < build start $started)"; exit 1 }
$sz = $item.Length
if ($sz -ge $MaxSize) { Write-Output "FAIL: oversized dll ($sz >= $MaxSize) - 사이즈 가드"; exit 1 }

# 신원 검증: dll 안에 이번에 컴파일한 소스의 절대경로가 박혀 있어야 한다(타모드/stale 차단).
$srcFull = (Resolve-Path $Src).Path
$bytes = [System.IO.File]::ReadAllBytes($out)
$needle = [System.Text.Encoding]::ASCII.GetBytes($srcFull)
$found = $false
for ($i = 0; $i -le ($bytes.Length - $needle.Length); $i++) {
  if ($bytes[$i] -eq $needle[0]) {
    $ok = $true
    for ($j = 1; $j -lt $needle.Length; $j++) { if ($bytes[$i+$j] -ne $needle[$j]) { $ok = $false; break } }
    if ($ok) { $found = $true; break }
  }
}
if (-not $found) {
  Write-Output "FAIL: 신원 검증 실패 - dll 안에 소스 경로 '$srcFull' 없음 (타 모드/stale dll 의심). 배포 중단."
  Write-Output "      built: $out ($sz bytes)"
  exit 1
}

try {
  New-Item -ItemType Directory -Force -Path $dep | Out-Null
  Copy-Item $out "$dep\$ModId.dll" -Force -ErrorAction Stop
  Write-Output "OK: deployed $ModId.dll = $sz bytes (verified)"
} catch {
  Write-Output "LOCKED: game running? build ok (size=$sz) but copy to mods\$ModId failed"
  Write-Output "        built dll kept at: $out"
  exit 2
}
