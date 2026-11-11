[Console]::OutputEncoding = [Text.Encoding]::UTF8
# 1호기 TFM2 상태 읽기 (읽기 전용) — 0.7.2 마이그 인게임 검증 준비
$g = 'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2'
'exe ' + (Get-Item "$g\TeamfightManager2.exe").Length + ' ' + (Get-FileHash "$g\TeamfightManager2.exe" -Algorithm SHA256).Hash.Substring(0, 16)
'base ' + (Get-Content "$g\mod-sdk-stable\base_version.txt")
'proc ' + ((Get-Process | Where-Object { $_.ProcessName -match 'TeamfightManager|steam|dnplayer' } | ForEach-Object { $_.ProcessName }) -join ',')
'free_GB ' + [math]::Round((Get-PSDrive C).Free / 1GB, 1)
$j = Get-Content "$g\config\game\mods.json" -Raw | ConvertFrom-Json
'enabled ' + ($j.enabled_mods -join ',')
'mods.json ' + (Get-FileHash "$g\config\game\mods.json" -Algorithm SHA256).Hash.Substring(0, 12)
foreach ($m in 'tfm2_level_cap','tfm2_champion_exclude','banpick_view_plus','tfm2_comptest_unlock','tfm2_elemental_serpen','tfm2_champ_pos_lock','training_view_plus','facility_view_plus','custom_tier_assignment','tfm2_transfer_fix','tfm2_detail_tactics') {
  $f = "$g\mods\$m\$m.dll"
  if (Test-Path $f) { '{0} {1} {2}' -f $m, (Get-Item $f).Length, (Get-FileHash $f -Algorithm SHA256).Hash.Substring(0, 12) } else { "$m (없음)" }
}
$d = Join-Path $env:APPDATA 'TeamSamoyed\TeamfightManager2\data'
Get-ChildItem $d -Filter 'save_*.data' -ErrorAction SilentlyContinue | Sort-Object LastWriteTime -Descending | Select-Object -First 6 | ForEach-Object { 'save ' + $_.Name + ' ' + $_.Length + ' ' + $_.LastWriteTime.ToString('MM-dd HH:mm') }
