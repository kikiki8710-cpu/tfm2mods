param([Parameter(Mandatory=$true)][string]$GameDir)
$ErrorActionPreference='Stop'
$gameRoot=(Resolve-Path -LiteralPath $GameDir).Path
if (-not (Test-Path -LiteralPath (Join-Path $gameRoot 'TeamfightManager2.exe'))) { throw 'GameDir is not the game installation.' }
if (Get-Process TeamfightManager2 -ErrorAction SilentlyContinue) { throw 'Close the game before installing.' }
$sourceRoot=Join-Path $PSScriptRoot 'target/package/tfm2_detail_tactics'
if (-not (Test-Path -LiteralPath (Join-Path $sourceRoot 'tfm2_detail_tactics.dll'))) { throw 'Run package_local.ps1 first.' }
$dest=Join-Path $gameRoot 'mods/tfm2_detail_tactics'
New-Item -ItemType Directory -Force -Path $dest | Out-Null
foreach($name in @('tfm2_detail_tactics.dll','mod.mod_info','README.md')) {Copy-Item -LiteralPath (Join-Path $sourceRoot $name) -Destination $dest}
$cfg=Join-Path $dest 'detail_tactics_values.cfg'
if(-not (Test-Path -LiteralPath $cfg)){Copy-Item -LiteralPath (Join-Path $sourceRoot 'detail_tactics_values.cfg') -Destination $cfg}
$actual=Get-Item -LiteralPath (Join-Path $dest 'tfm2_detail_tactics.dll')
if ((Get-FileHash -LiteralPath $actual.FullName).Hash -ne (Get-FileHash -LiteralPath (Join-Path $sourceRoot 'tfm2_detail_tactics.dll')).Hash){throw 'Deployed DLL hash mismatch'}
$actual | Select-Object FullName,Length,LastWriteTime
