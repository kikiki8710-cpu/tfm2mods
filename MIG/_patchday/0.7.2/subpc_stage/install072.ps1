[Console]::OutputEncoding = [Text.Encoding]::UTF8
# 1호기에 0.7.2 게임 4파일 + 10모드 설치 (0.7.1 install071.ps1 과 같은 방식 · 메인 PC 배포본과 해시 일치 확인)
$ErrorActionPreference = 'Stop'
$g = 'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2'
$s = 'C:\tfm2_stage\072'
if (Get-Process TeamfightManager2 -ErrorAction SilentlyContinue) { throw 'game is running' }
if (Get-Process steam -ErrorAction SilentlyContinue) { throw 'steam is running' }
# mods.json 원본 보관(검증 뒤 원복용 · 이미 있으면 덮어쓰지 않음)
$mj = "$g\config\game\mods.json"
$bak = "$mj.bak_072_20261111"
if (-not (Test-Path $bak)) { Copy-Item -LiteralPath $mj -Destination $bak }
'mods.json.bak ' + (Get-FileHash $bak -Algorithm SHA256).Hash.Substring(0, 12)
Copy-Item "$s\TeamfightManager2.exe" "$g\TeamfightManager2.exe" -Force
Copy-Item "$s\bundle.game_data" "$g\bundle.game_data" -Force
Copy-Item "$s\TFM2ModUploader.exe" "$g\TFM2ModUploader.exe" -Force
Copy-Item "$s\base_version.txt" "$g\mod-sdk-stable\base_version.txt" -Force
& tar -xf "$s\mods072.tar" -C "$g"
if ($LASTEXITCODE -ne 0) { throw "tar rc=$LASTEXITCODE" }
Get-FileHash "$g\TeamfightManager2.exe" -Algorithm SHA256 | ForEach-Object { 'exe ' + $_.Hash }
Get-FileHash "$g\bundle.game_data" -Algorithm SHA256 | ForEach-Object { 'bundle ' + $_.Hash }
Get-FileHash "$g\TFM2ModUploader.exe" -Algorithm SHA256 | ForEach-Object { 'uploader ' + $_.Hash.Substring(0, 16) }
'base ' + (Get-Content "$g\mod-sdk-stable\base_version.txt")
foreach ($m in 'tfm2_level_cap','tfm2_champion_exclude','banpick_view_plus','tfm2_comptest_unlock','tfm2_elemental_serpen','tfm2_champ_pos_lock','training_view_plus','facility_view_plus','custom_tier_assignment','tfm2_transfer_fix') {
  $f = "$g\mods\$m\$m.dll"
  $i = "$g\mods\$m\mod.mod_info"
  '{0} {1} {2} info_first={3:x2}' -f $m, (Get-Item $f).Length, (Get-FileHash $f -Algorithm SHA256).Hash.Substring(0, 12), [IO.File]::ReadAllBytes($i)[0]
}
$j = Get-Content $mj -Raw | ConvertFrom-Json
'enabled ' + ($j.enabled_mods -join ',')
