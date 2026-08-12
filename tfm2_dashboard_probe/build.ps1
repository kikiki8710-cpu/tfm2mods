# tfm2_item_tree_probe 전용 빌드 — build_inj.ps1 과 동일하되 game_core/serde_json extern 추가.
param([string]$Src = "C:\tfm2mods\tfm2_dashboard_probe\src\lib.rs")
$ModId = "TFM2_Meta_Dashboard"
$SDK  = "C:\tfm2mods\sdk_055\mod-sdk"   # ★0.5.3 SDK (게임 크레이트 rlib 236개 전원 DIFF → RVA 0 모드도 재빌드 필수)
$DEPS = "$SDK\deps"; $NAT = "$SDK\native"
$MODAPI = (Get-ChildItem "$DEPS\libmod_api-*.rlib")[0].FullName
$EUI    = (Get-ChildItem "$DEPS\libengine_ui-*.rlib")[0].FullName
$GC     = (Get-ChildItem "$DEPS\libgame_core-*.rlib")[0].FullName
# serde_json 은 rlib 2개(버전 중복) → mod_api 가 쓰는 쪽과 충돌 안 나게 최신 mtime 것 선택
$SJ     = (Get-ChildItem "$DEPS\libserde_json-*.rlib" | Sort-Object LastWriteTime -Desc)[0].FullName
$work = Join-Path $env:TEMP "tfm2_build\$ModId`_$PID"
New-Item -ItemType Directory -Force -Path $work | Out-Null
$out = "$work\$ModId.dll"; $errf = "$work\rustc_err.txt"
$started = Get-Date
# ★0.5.3: MSVC 2019 link.exe 는 sdk_053 rlib 재스캔 중 LNK1107 로 죽는다 ⟹ rust-lld 필수.
cmd /c "rustup run nightly-2026-05-24 rustc --crate-type cdylib --edition 2021 -C opt-level=1 -C overflow-checks=off -C linker-flavor=lld-link -C linker=rust-lld -L dependency=$DEPS -L native=$NAT --extern mod_api=$MODAPI --extern engine_ui=$EUI --extern game_core=$GC --extern serde_json=$SJ $Src -o `"$out`" 2> `"$errf`""
if ($LASTEXITCODE -ne 0) {
  Write-Output "=== BUILD FAILED (rustc exit $LASTEXITCODE) ==="
  Get-Content $errf | Select-Object -First 80 | ForEach-Object { Write-Output $_ }
  exit 1
}
$item = Get-Item $out
if ($item.LastWriteTime -lt $started) { Write-Output "FAIL: stale dll"; exit 1 }
Write-Output "BUILD OK: $out  $($item.Length) bytes  $($item.LastWriteTime)"
Write-Output "SJ=$SJ"
