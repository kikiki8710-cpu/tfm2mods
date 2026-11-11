# 0.7.2 migration - build/deploy/SHA256-compare 10 mods (detail_tactics excluded) - main session 2026-11-11
# (ASCII only: PowerShell 5.1 reads BOM-less .ps1 as the ANSI code page)
# build = cargo build --release (stable shell) -> deploy_stable.py (keeps current deps) -> artifact vs deployed SHA256
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
$root = 'F:\claude\tfm2mods'
$logdir = Join-Path $root 'MIG\_patchday\0.7.2\build'
New-Item -ItemType Directory -Force $logdir | Out-Null
$gameMods = 'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods'
$mods = 'tfm2_level_cap','tfm2_champion_exclude','banpick_view_plus','tfm2_comptest_unlock','tfm2_elemental_serpen','tfm2_champ_pos_lock','training_view_plus','facility_view_plus','custom_tier_assignment','tfm2_transfer_fix'
if (Get-Process | Where-Object { $_.ProcessName -like 'TeamfightManager*' }) { 'GAME RUNNING - abort'; exit 2 }
$fail = 0
foreach ($m in $mods) {
    Push-Location (Join-Path $root $m)
    cmd /c "cargo build --release > `"$logdir\$m.log`" 2>&1"
    $code = $LASTEXITCODE
    Pop-Location
    if ($code -ne 0) { "BUILD FAIL  $m  (cargo $code) -> $logdir\$m.log"; $fail++; continue }
    $dll = Get-ChildItem (Join-Path $root "$m\target\release") -Filter *.dll | Sort-Object LastWriteTime -Descending | Select-Object -First 1
    $dep = python -X utf8 (Join-Path $root 'deploy_stable.py') $m --dll $dll.FullName 2>&1
    $dep | Out-File -Encoding utf8 "$logdir\$m.deploy.log"
    $ok = $dep | Select-String '^OK: deployed'
    $dst = Join-Path $gameMods "$m\$m.dll"
    $h1 = (Get-FileHash $dll.FullName -Algorithm SHA256).Hash
    $h2 = 'NONE'
    if (Test-Path $dst) { $h2 = (Get-FileHash $dst -Algorithm SHA256).Hash }
    $it = Get-Item $dst -ErrorAction SilentlyContinue
    $same = 'SHA256 MATCH'
    if ($h1 -ne $h2) { $same = 'SHA256 MISMATCH'; $fail++ }
    $dtag = 'deploy OK'
    if (-not $ok) { $dtag = 'NO deploy line'; $fail++ }
    $ts = $it.LastWriteTime.ToString("yyyy-MM-dd HH:mm:ss")
    "{0,-24} {1,9:N0}B  {2}  {3}  {4}  {5}  src={6}" -f $m, $it.Length, $ts, $h2.Substring(0,12), $same, $dtag, $dll.Name
}
"FAIL $fail"
exit $fail
