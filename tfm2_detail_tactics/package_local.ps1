param([switch]$SkipBuild)
$ErrorActionPreference='Stop'
$projectRoot=$PSScriptRoot
$env:DT_EXPORT_DIR=Join-Path $projectRoot 'target/review'
if (-not $SkipBuild) { & (Join-Path $projectRoot 'build_local.ps1') -RunTests }
$packageDir=Join-Path $projectRoot 'target/package'
$stageDir=Join-Path $packageDir 'tfm2_detail_tactics'
New-Item -ItemType Directory -Force -Path $stageDir | Out-Null
foreach ($name in @('mod.mod_info','README.md')) { Copy-Item -LiteralPath (Join-Path $projectRoot $name) -Destination $stageDir }
Copy-Item -LiteralPath (Join-Path $projectRoot 'target/release/tfm2_detail_tactics.dll') -Destination $stageDir
Copy-Item -LiteralPath (Join-Path $projectRoot 'target/review/detail_tactics_values.cfg') -Destination $stageDir
$zipPath=Join-Path $packageDir 'tfm2_detail_tactics.zip'
# zip entries use '/' (PS5.1 Compress-Archive writes backslashes = ZIP spec violation; deploy-verify 2026-11-11)
& python -X utf8 (Join-Path $projectRoot 'tools/make_zip.py') $stageDir $zipPath
if ($LASTEXITCODE -ne 0) { throw 'zip build failed' }
Get-Item -LiteralPath $zipPath | Select-Object FullName,Length,LastWriteTime
