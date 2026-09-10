# 커스텀 rustc 드라이버 빌드. 사용: powershell -File C:\tfm2mods\MIG\tcxbuild.ps1 -Src C:\tfm2mods\MIG\_tcxdrv\tcxdump.rs
param([Parameter(Mandatory=$true)][string]$Src,[string]$OutDir="C:\tfm2mods\MIG\_tcxdrv")
$TC="C:\Users\jungs\.rustup\toolchains\nightly-2026-05-24-x86_64-pc-windows-msvc"
$RUSTUP="C:\Users\jungs\.cargo\bin\rustup.exe"
$env:PATH="$TC\bin;$env:PATH"
$cmd="`"$RUSTUP`" run nightly-2026-05-24 rustc --edition 2021 -C opt-level=1 `"$Src`" --out-dir `"$OutDir`" 2>&1"
cmd /c $cmd
