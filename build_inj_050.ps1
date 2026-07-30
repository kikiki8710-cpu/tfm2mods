# build_inj_050.ps1 — 0.5.0 SDK로 mod 빌드·배포. build_inj.ps1의 0.5.0 변종($SDK만 다름).
# 사용: powershell -ExecutionPolicy Bypass -File build_inj_050.ps1 -Src <lib.rs 절대경로> -ModId <mod_id>
# toolchain은 0.4.14와 동일(nightly-2026-05-24). SDK만 sdk_050(mod_api b6ac0336, 게임 0.5.0 buildid 24102827 매칭).
#
# ⚠⚠ 2026-07-08 사고 수정: 구버전은 산출물을 공유 경로 "$SDK\lib.dll" 에 떨어뜨렸다.
#   두 세션이 서로 다른 모드를 동시에 빌드하면 같은 파일을 두고 경쟁 → **남의 모드 dll을 복사 배포**.
#   실제로 Spectator_Chat 배포본이 tfm2_item_tactics dll(694,784B)로 덮였다(무증상, 크기 가드도 통과).
#   게다가 rustc 실패를 stderr 문자열 grep으로만 판정해 **exit code를 안 봤다** → 빌드 실패 시 이전 산출물을 복사.
#   → ①출력/에러 파일을 ModId+PID 로 격리 ②rustc exit code 확인 ③산출물이 "이번 실행"에서 생겼는지 mtime 확인
#     ④배포 전 dll 안에 ModId 문자열이 있는지 신원 검증(타모드/stale 차단).
param(
  [Parameter(Mandatory=$true)][string]$Src,
  [Parameter(Mandatory=$true)][string]$ModId
)
# ★ 0.5.0_hotfix 공식 SDK (2026-07-08 21:55 게임 핫픽스와 매칭. GitHub Releases 0.5.0_hotfix.zip).
#   구 sdk_050(0.5.0 정식)로 빌드하면 libgame_core/libgame_view 가 게임과 desync.
#   0.4.14용 sdk_0414_new 쓰면 ABI desync 크래시(tfm2-native-mod-loader-abi 교훈).
#   toolchain 은 nightly-2026-05-24 로 0.5.0/0.4.14 와 동일, libmod_api 는 crate-id 불변(ABI 호환).
$SDK  = "C:\tfm2mods\sdk_050_hotfix2\mod-sdk"
$DEPS = "$SDK\deps"; $NAT = "$SDK\native"
$MODAPI = (Get-ChildItem "$DEPS\libmod_api-*.rlib")[0].FullName
$EUI    = (Get-ChildItem "$DEPS\libengine_ui-*.rlib")[0].FullName

# ★모드별·프로세스별 격리 작업 폴더 (공유 $SDK\lib.dll 사용 금지)
$work = Join-Path $env:TEMP "tfm2_build\$ModId`_$PID"
New-Item -ItemType Directory -Force -Path $work | Out-Null
$out  = "$work\$ModId.dll"
$errf = "$work\rustc_err.txt"
$dep  = "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\$ModId"

$env:RUSTFLAGS = "-C overflow-checks=off"
$started = Get-Date
cmd /c "rustup run nightly-2026-05-24 rustc --crate-type cdylib --edition 2021 -L dependency=$DEPS -L native=$NAT --extern mod_api=$MODAPI --extern engine_ui=$EUI $Src -o `"$out`" 2> `"$errf`""
$rc = $LASTEXITCODE

# ① rustc exit code 우선 (문자열 grep 은 보조)
if ($rc -ne 0) {
  Write-Output "=== BUILD FAILED (rustc exit $rc) ==="
  if (Test-Path $errf) { Get-Content $errf | Select-String -Pattern "error\[|^error:" | Select-Object -First 40 | ForEach-Object { Write-Output $_.Line } }
  exit 1
}
$errs = @()
if (Test-Path $errf) { $errs = Get-Content $errf | Select-String -Pattern "error\[|^error:" }
if ($errs.Count -gt 0) { Write-Output "=== BUILD ERRORS ==="; $errs | Select-Object -First 40 | ForEach-Object { Write-Output $_.Line }; exit 1 }
if (-not (Test-Path $out)) { Write-Output "FAIL: $ModId.dll not produced"; exit 1 }

# ② 산출물이 이번 실행에서 생겼는지 (stale 차단)
$item = Get-Item $out
if ($item.LastWriteTime -lt $started) { Write-Output "FAIL: stale dll (mtime $($item.LastWriteTime) < build start $started)"; exit 1 }
$sz = $item.Length
if ($sz -ge 1300000) { Write-Output "FAIL: oversized dll ($sz) - 사이즈 가드"; exit 1 }

# ③ 신원 검증: dll 안에 "이번에 컴파일한 소스의 절대경로" 문자열이 박혀 있어야 한다.
#    rustc 가 panic location 등으로 소스 경로를 PE 에 박으므로, 이 dll 이 정말 $Src 에서
#    나왔는지 증명한다. 활성 7개 모드 전부 자기 소스경로 보유 + 타모드 경로 0 확인(2026-07-08).
#    ⚠ ModId 문자열로 검사하면 안 된다 — `-o <ModId>.dll` 이라 PE 헤더에 그 파일명이 박혀
#      **어떤 dll이든 무조건 통과**한다(가드 무력화. 실제로 이 실수로 tfm2_fog 배포본을 덮었다).
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
