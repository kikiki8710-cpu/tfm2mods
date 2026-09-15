# 이분(bisect) 1회: 마스크 파일 → sweep20_on.txt → 게임 기동 → 리플레이 진입 → N초 생존 확인 → (옵션) 종료. (2026-09-16 r17 판 2 크래시 이분용)
param([string]$Mask, [int]$Hold = 45, [switch]$Keep)
$G = "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_judge_verify"
Get-Process TeamfightManager2 -EA SilentlyContinue | Stop-Process -Force -EA SilentlyContinue; Start-Sleep 2
Copy-Item $Mask "$G\sweep20_on.txt" -Force
"mask=" + (Get-Content "$G\sweep20_on.txt")
Start-Process "steam://rungameid/3009300"; Start-Sleep 75
if (-not (Get-Process TeamfightManager2 -EA SilentlyContinue)) { "RESULT: DEAD-at-load"; exit 2 }
. C:\tfm2mods\MIG\_clickfn.ps1
Click-TFM 726 620 | Out-Null; Start-Sleep 2; Click-TFM 1082 328 | Out-Null; Start-Sleep 3; Click-TFM 864 564 | Out-Null; Start-Sleep 3
Click-TFM 98 196 | Out-Null; Start-Sleep 3; Click-TFM 763 134 | Out-Null; Start-Sleep 3; Click-TFM 1056 458 | Out-Null; Start-Sleep 5
Press-TFM 0x45 | Out-Null; Start-Sleep 1; Press-TFM 0x45 | Out-Null; Start-Sleep 1; Press-TFM 0x45 | Out-Null; Start-Sleep 1; Press-TFM 0x45 | Out-Null
Start-Sleep $Hold
$p = Get-Process TeamfightManager2 -EA SilentlyContinue
if ($p) { "RESULT: ALIVE after ${Hold}s"; if (-not $Keep) { Stop-Process -Name TeamfightManager2 -Force -EA SilentlyContinue }; exit 0 } else { "RESULT: DEAD"; exit 1 }
