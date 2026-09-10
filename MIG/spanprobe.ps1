param([Parameter(Mandatory=$true)][string]$Src,[string]$Extra="")
$SDK="C:\tfm2mods\sdk_058\mod-sdk"; $DEPS="$SDK\deps"; $NAT="$SDK\native"
$MODAPI=(Get-ChildItem "$DEPS\libmod_api-*.rlib")[0].FullName
$GCORE=(Get-ChildItem "$DEPS\libgame_core-*.rlib")[0].FullName
$GVIEW=(Get-ChildItem "$DEPS\libgame_view-*.rlib")[0].FullName
$COMMON=(Get-ChildItem "$DEPS\libcommon-*.rlib")[0].FullName
$GAI=(Get-ChildItem "$DEPS\libgame_ai-*.rlib")[0].FullName
$ECORE=(Get-ChildItem "$DEPS\libengine_core-*.rlib")[0].FullName
$EUI=(Get-ChildItem "$DEPS\libengine_ui-*.rlib")[0].FullName
$RUSTUP=Join-Path $env:USERPROFILE ".cargo\bin\rustup.exe"
$work="$env:TEMP\tfm2_spanprobe"; New-Item -ItemType Directory -Force -Path $work | Out-Null
$cmd = "`"$RUSTUP`" run nightly-2026-05-24 rustc --edition 2021 -L dependency=$DEPS -L native=$NAT --extern mod_api=$MODAPI --extern engine_ui=$EUI --extern engine_core=$ECORE --extern game_core=$GCORE --extern game_view=$GVIEW --extern common=$COMMON --extern game_ai=$GAI $Extra `"$Src`" --out-dir `"$work`" 2>&1"
cmd /c $cmd
