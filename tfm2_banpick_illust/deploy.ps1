# tfm2_banpick_illust 배포 — 게임 mods\<MOD_ID>\ 로 코드/레이아웃/메타 + 일러스트 에셋 동기화.
#
# 일러스트 PNG(65종 약 460MB)는 소스트리에 두지 않고 배포 폴더에만 둔다(중복 저장 방지).
# 원본 = 창작마당 "Pick Ban Plus"(3751386306) illust 폴더. 이미 배포돼 있으면 재복사 생략.
$ErrorActionPreference = "Stop"

$MOD  = "tfm2_banpick_illust"
$SRC  = "C:\tfm2mods\$MOD"
$GAME = "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2"
$DST  = "$GAME\mods\$MOD"
$ART  = "C:\Program Files (x86)\Steam\steamapps\workshop\content\3009300\3751386306\illust"

if (-not (Test-Path $GAME)) { throw "게임 설치 폴더 없음: $GAME" }

New-Item -ItemType Directory -Force -Path $DST, "$DST\ui\layout\banpick", "$DST\illust" | Out-Null

# 1) dll
$dll = "$SRC\$MOD.dll"
if (-not (Test-Path $dll)) { throw "dll 없음 — build.ps1 먼저 실행" }
Copy-Item $dll "$DST\$MOD.dll" -Force
$d = Get-Item "$DST\$MOD.dll"
Write-Output "dll      : $($d.Length) bytes  $($d.LastWriteTime)"

# 2) 메타 + 레이아웃
Copy-Item "$SRC\mod.mod_info"      "$DST\mod.mod_info"      -Force
Copy-Item "$SRC\mod.override_info" "$DST\mod.override_info" -Force
Copy-Item "$SRC\ui\layout\banpick\*.ui" "$DST\ui\layout\banpick\" -Force

# mod_info/override_info 는 BOM 없는 UTF-8 이어야 게임 파서가 읽는다 — 검증
foreach ($f in @("mod.mod_info", "mod.override_info")) {
    $b = [System.IO.File]::ReadAllBytes("$DST\$f")[0]
    if ($b -ne 0x7b) { throw "$f 선두 바이트가 0x7b('{')가 아님 (0x$('{0:x2}' -f $b)) — BOM 의심" }
    Write-Output "$f : BOM 없음 OK"
}

# 3) 일러스트 에셋 — **건드리지 않는다**.
#    2026-07-19 부터 아트팩은 유저가 직접 관리한다(진영별 blue/ red/ red_noflip/ 구조).
#    예전엔 워크샵 폴더에서 복사했는데, 그러면 유저가 교체한 팩을 덮어쓴다.
$ill = "$DST\illust"
if (Test-Path $ill) {
    $sub = Get-ChildItem $ill -Directory | Select-Object -ExpandProperty Name
    $cnt = (Get-ChildItem $ill -Recurse -File -Filter *.png).Count
    Write-Output "illust   : png $cnt개, 하위폴더 [$($sub -join ', ')] (그대로 둠)"
} else {
    Write-Warning "illust 폴더 없음: $ill — 일러스트가 안 뜹니다"
}

Write-Output "배포 완료: $DST"
