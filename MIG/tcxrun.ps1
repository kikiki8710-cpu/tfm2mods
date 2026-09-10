# 커스텀 rustc 드라이버 실행(SDK rlib 전부 링크). 사용 예:
#   powershell -File C:\tfm2mods\MIG\tcxrun.ps1 -Exe C:\tfm2mods\MIG\_tcxdrv\tcxdump.exe -Crate game_ai
#   powershell -File C:\tfm2mods\MIG\tcxrun.ps1 -Exe C:\tfm2mods\MIG\_tcxdrv\mirdump.exe -Crate game_core -Out C:/tfm2mods/MIG/_tcx/mirdump_game_core.txt
param(
  [Parameter(Mandatory=$true)][string]$Exe,
  [string]$Src="C:\tfm2mods\MIG\_tcxdrv\probe.rs",
  [string]$Crate="",
  [string]$Out=""
)
$TC="C:\Users\jungs\.rustup\toolchains\nightly-2026-05-24-x86_64-pc-windows-msvc"
$SDK="C:\tfm2mods\sdk_058\mod-sdk"; $DEPS="$SDK\deps"; $NAT="$SDK\native"
$MODAPI=(Get-ChildItem "$DEPS\libmod_api-*.rlib")[0].FullName
$GCORE=(Get-ChildItem "$DEPS\libgame_core-*.rlib")[0].FullName
$GVIEW=(Get-ChildItem "$DEPS\libgame_view-*.rlib")[0].FullName
$COMMON=(Get-ChildItem "$DEPS\libcommon-*.rlib")[0].FullName
$GAI=(Get-ChildItem "$DEPS\libgame_ai-*.rlib")[0].FullName
$ECORE=(Get-ChildItem "$DEPS\libengine_core-*.rlib")[0].FullName
$EUI=(Get-ChildItem "$DEPS\libengine_ui-*.rlib")[0].FullName
$work="$env:TEMP\tfm2_tcx"; New-Item -ItemType Directory -Force -Path $work | Out-Null
$env:PATH="$TC\bin;$env:PATH"     # rustc_driver-*.dll / std-*.dll 로드에 필수
if ($Crate -ne "") { $env:TCX_CRATE=$Crate }
if ($Out   -ne "") { $env:TCX_OUT=$Out }
$cmd="`"$Exe`" --edition 2021 --sysroot `"$TC`" -L dependency=$DEPS -L native=$NAT --extern mod_api=$MODAPI --extern engine_ui=$EUI --extern engine_core=$ECORE --extern game_core=$GCORE --extern game_view=$GVIEW --extern common=$COMMON --extern game_ai=$GAI `"$Src`" --out-dir `"$work`" --emit=metadata 2>&1"
cmd /c $cmd
