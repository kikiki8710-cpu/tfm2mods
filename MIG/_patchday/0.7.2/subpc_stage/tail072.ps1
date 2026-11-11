param([string]$Mod = 'tfm2_elemental_serpen', [string]$File = 'serpen_probe.txt', [int]$Tail = 60)
[Console]::OutputEncoding = [Text.Encoding]::UTF8
# 1호기 모드 로그 끝부분 읽기 (읽기 전용)
$p = Join-Path 'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods' (Join-Path $Mod $File)
'=== ' + $p + ' ' + (Get-Item -LiteralPath $p).Length + 'B ' + (Get-Item -LiteralPath $p).LastWriteTime.ToString('HH:mm:ss')
Get-Content -LiteralPath $p -Encoding UTF8 -Tail $Tail
