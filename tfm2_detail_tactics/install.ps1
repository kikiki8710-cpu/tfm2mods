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
else {
    # 기존 사용자 수치는 그대로 두고 새 버전에 추가된 전술 섹션만 보충한다.
    $utf8=New-Object System.Text.UTF8Encoding($false)
    $existing=[System.IO.File]::ReadAllText($cfg)
    $defaults=[System.IO.File]::ReadAllText((Join-Path $sourceRoot 'detail_tactics_values.cfg'))
    $missing=@([regex]::Matches($defaults,'(?ms)^\[([^\]\r\n]+)\]\r?\n.*?(?=^\[|\z)') | Where-Object {
        -not [regex]::IsMatch($existing,'(?m)^\['+[regex]::Escape($_.Groups[1].Value)+'\]\s*$')
    })
    if($missing.Count -gt 0){
        $backup=$cfg+'.before_'+(Get-Date -Format 'yyyyMMdd_HHmmssfff')+'.bak'
        Copy-Item -LiteralPath $cfg -Destination $backup
        $merged=$existing.TrimEnd()+"`n`n"+(($missing | ForEach-Object {$_.Value.TrimEnd()}) -join "`n`n")+"`n"
        [System.IO.File]::WriteAllText($cfg,$merged,$utf8)
    }
}
$actual=Get-Item -LiteralPath (Join-Path $dest 'tfm2_detail_tactics.dll')
if ((Get-FileHash -LiteralPath $actual.FullName).Hash -ne (Get-FileHash -LiteralPath (Join-Path $sourceRoot 'tfm2_detail_tactics.dll')).Hash){throw 'Deployed DLL hash mismatch'}
$actual | Select-Object FullName,Length,LastWriteTime
