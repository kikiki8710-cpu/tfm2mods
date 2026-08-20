# build_full_remap.ps1 — build_full.ps1 과 동일하되 **소스 절대경로 PII 제거**(--remap-path-prefix).
# 사용: powershell -File build_full_remap.ps1 -Src <lib.rs 절대경로> -ModId <mod_id> [-MaxSize <bytes>]
#
# 왜 별도인가:
#   rustc 는 panic location 용으로 컴파일 시 **소스 절대경로**를 dll 에 문자열로 박는다.
#   워크샵/릴리스 zip 에 그대로 나가면 유저명(C:\Users\dev\...)이 유출된다.
#   --remap-path-prefix 로 접두를 치환하면 dll 안엔 `mods-src\<mod>\src\lib.rs` 만 남는다.
#
#   ⚠ 그 결과 build_full.ps1 의 신원 검증(= dll 안에 소스 절대경로가 있어야 통과)은 **반드시 실패**한다.
#     그래서 여기선 신원 검증을 **remap 후 기대 문자열**(`mods-src\<mod>\src\lib.rs`)로 바꿔서 수행한다.
#   ⚠ panic 경로 문자열이 애초에 안 박히는 모드(예: facility_view_plus — DONE.md 함정 ①)는
#     검증이 오탐하므로 -SkipIdentity 로 넘기고, 대신 크기·mtime·PII 검사로만 판정한다.
#
# 그 외(게임 crate 전량 --extern, rust-lld 고정, opt-level=1, 사이즈가드, stale mtime, 락 처리)는
# build_full.ps1 과 동일하게 유지.
param(
  [Parameter(Mandatory=$true)][string]$Src,
  [Parameter(Mandatory=$true)][string]$ModId,
  [int]$MaxSize = 5000000,
  [switch]$SkipIdentity
)
# 0.5.4 전환 2026-08-05: sdk_054. rlib 파일 수·파일명은 053과 동일하나 mod_api/game_core/
# game_view/game_ai/engine_ui/engine_core 6종 전부 내용 해시 DIFF ⟹ RVA 0 모드도 재빌드 필수.
# toolchain 은 nightly-2026-05-24 유지(sdk_054\mod-sdk\toolchain_version.txt = rustc 1.98.0-nightly 23a3312d9 실측).
$SDK  = "C:\tfm2mods\sdk_056\mod-sdk"
$DEPS = "$SDK\deps"; $NAT = "$SDK\native"

# remap 원본 접두 = 소스 트리 루트. dll 에는 이 자리에 $REMAP_TO 가 들어간다.
# (0.5.6 2026-08-20 정정: 계정 경로가 dev → jungs 로 바뀌어 remap 접두 불일치 = 신원검증 전멸 + PII 잔존 위험이었음)
$REMAP_FROM = "C:\Users\jungs\Desktop\claude\tfm2\tfm2-mods-main"
$REMAP_TO   = "mods-src"

function Pick([string]$pat) { (Get-ChildItem "$DEPS\$pat")[0].FullName }
$X = @(
  ("mod_api="      + (Pick "libmod_api-*.rlib")),
  ("common="       + (Pick "libcommon-*.rlib")),
  ("engine_core="  + (Pick "libengine_core-*.rlib")),
  ("engine_ui="    + (Pick "libengine_ui-*.rlib")),
  ("engine_asset=" + (Pick "libengine_asset-*.rlib")),
  ("game_core="    + (Pick "libgame_core-*.rlib")),
  ("game_view="    + (Pick "libgame_view-*.rlib")),
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
cmd /c "rustup run nightly-2026-05-24 rustc --crate-type cdylib --edition 2021 -C opt-level=1 -C overflow-checks=off -C linker-flavor=lld-link -C linker=rust-lld --remap-path-prefix `"$REMAP_FROM=$REMAP_TO`" -L dependency=$DEPS -L native=$NAT $externs $Src -o `"$out`" 2> `"$errf`""
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

$bytes = [System.IO.File]::ReadAllBytes($out)
function Contains-Ascii([byte[]]$hay, [string]$s) {
  $n = [System.Text.Encoding]::ASCII.GetBytes($s)
  for ($i = 0; $i -le ($hay.Length - $n.Length); $i++) {
    if ($hay[$i] -eq $n[0]) {
      $ok = $true
      for ($j = 1; $j -lt $n.Length; $j++) { if ($hay[$i+$j] -ne $n[$j]) { $ok = $false; break } }
      if ($ok) { return $true }
    }
  }
  return $false
}

# ① PII 검사 (이 스크립트의 존재 이유) — remap 이 실제로 먹었는지
$srcFull = (Resolve-Path $Src).Path
if (Contains-Ascii $bytes $REMAP_FROM) { Write-Output "FAIL: PII 잔존 - dll 안에 '$REMAP_FROM' 있음(remap 미적용). 배포 중단."; exit 1 }
if (Contains-Ascii $bytes "C:\Users\dev") { Write-Output "FAIL: PII 잔존 - dll 안에 'C:\Users\dev' 있음. 배포 중단."; exit 1 }

# ② 신원 검증 — remap 후 기대 문자열
$expect = "$REMAP_TO\$ModId\src\lib.rs"
if (-not $SkipIdentity) {
  if (-not (Contains-Ascii $bytes $expect)) {
    Write-Output "FAIL: 신원 검증 실패 - dll 안에 '$expect' 없음 (타 모드/stale dll 의심). 배포 중단."
    Write-Output "      panic 경로가 애초에 안 박히는 모드면 -SkipIdentity 로 재실행할 것."
    Write-Output "      built: $out ($sz bytes)"
    exit 1
  }
} else {
  Write-Output "NOTE: -SkipIdentity - 신원 검증 생략(panic 경로 미기재 모드). PII 검사만으로 판정."
}

try {
  New-Item -ItemType Directory -Force -Path $dep | Out-Null
  Copy-Item $out "$dep\$ModId.dll" -Force -ErrorAction Stop
  Write-Output "OK: deployed $ModId.dll = $sz bytes (PII-free, verified)"
} catch {
  Write-Output "LOCKED: game running? build ok (size=$sz) but copy to mods\$ModId failed"
  Write-Output "        built dll kept at: $out"
  exit 2
}
