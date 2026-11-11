[Console]::OutputEncoding = [Text.Encoding]::UTF8
# 0.7.2 인게임 검증 뒤 원복·로그 회수 (0.7.1 after.ps1 과 같은 방식)
$g = 'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2'
if (Get-Process TeamfightManager2 -ErrorAction SilentlyContinue) { 'game still running'; exit 1 }
$c = "$g\config\game"
$bak = Join-Path $c 'mods.json.bak_072_20261111'
Copy-Item -LiteralPath $bak -Destination "$c\mods.json" -Force
'mods.json ' + (Get-FileHash "$c\mods.json" -Algorithm SHA256).Hash.Substring(0, 12) + ' bak ' + (Get-FileHash $bak -Algorithm SHA256).Hash.Substring(0, 12)
'first ' + ('{0:x2}' -f [IO.File]::ReadAllBytes("$c\mods.json")[0])
$j = Get-Content "$c\mods.json" -Raw | ConvertFrom-Json
'transfer_fix enabled=' + ($j.enabled_mods -contains 'tfm2_transfer_fix')
$out = 'C:\tfm2_stage\072\logs'
New-Item -ItemType Directory -Force $out | Out-Null
foreach ($p in 'tfm2_elemental_serpen\serpen_probe.txt', 'tfm2_transfer_fix\tfm2_transfer_fix.txt', 'tfm2_champ_pos_lock\champ_pos_lock_save.txt') {
  Copy-Item -LiteralPath "$g\mods\$p" -Destination $out -Force
}
Get-Content -LiteralPath "$g\mods\tfm2_champ_pos_lock\champ_pos_lock_log.txt" -Tail 600 -Encoding UTF8 | Set-Content -LiteralPath "$out\champ_pos_lock_log_tail.txt" -Encoding UTF8
Get-Content -LiteralPath "$g\mods\tfm2_level_cap\tfm2_level_cap.txt" -Tail 60 -Encoding UTF8 | Set-Content -LiteralPath "$out\tfm2_level_cap_tail.txt" -Encoding UTF8
Copy-Item -LiteralPath 'C:\Users\Owner\AppData\Roaming\TeamSamoyed\TeamfightManager2\data\log.log' -Destination $out -Force
Get-ChildItem $out | ForEach-Object { $_.Name + ' ' + $_.Length }
$since = [datetime]'2026-11-11 16:50'
Get-ChildItem $g -Filter 'crash*' -ErrorAction SilentlyContinue | Where-Object { $_.LastWriteTime -gt $since } | ForEach-Object { 'crash ' + $_.FullName }
Get-ChildItem "$g\mods" -Recurse -File -Include crash_log.txt, panic_log.txt, *_crash.txt -ErrorAction SilentlyContinue | Where-Object { $_.LastWriteTime -gt $since } | ForEach-Object { 'crash ' + $_.FullName }
Get-ChildItem 'C:\ProgramData\Microsoft\Windows\WER\ReportArchive' -Directory -ErrorAction SilentlyContinue | Where-Object { $_.Name -like '*Teamfight*' -and $_.LastWriteTime -gt $since } | ForEach-Object { 'WER ' + $_.Name }
'crash 검사 끝'
