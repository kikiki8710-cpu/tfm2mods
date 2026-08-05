# fix_modinfo_text_054.ps1 - refresh version-bearing TEXT inside mod.mod_info after a game-version bump.
#
# The dependency band is handled by bump_deps_054.ps1. This script fixes the *human-readable*
# fields that also carry the version and go stale silently:
#   - description strings like "game 0.5.3 only - auto-disabled on other versions"
#   - last_updated
# A stale "0.5.3 only" line is user-visible in the mod manager and contradicts the deps band
# we just set, so it must move with the release.
#
# NOTE: keep this file ASCII-only (a BOM-less .ps1 is read as ANSI by PowerShell 5.1).
# NOTE: the game parses mod_info as UTF-8 WITHOUT BOM - write via UTF8Encoding($false).
#
# usage: powershell -File fix_modinfo_text_054.ps1 -ModIds a,b   (or -WhatIf to preview)
param(
  [Parameter(Mandatory=$true)][string[]]$ModIds,
  [string]$Root = "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods",
  [string]$Today = "2026-08-05",
  [switch]$WhatIf
)
$enc = New-Object System.Text.UTF8Encoding($false)
foreach ($id in $ModIds) {
  $p = Join-Path $Root "$id\mod.mod_info"
  if (-not (Test-Path $p)) { Write-Output ("{0,-28} SKIP: no mod_info" -f $id); continue }
  $raw = [IO.File]::ReadAllText($p, [Text.Encoding]::UTF8)
  $body = $raw
  # version text inside description: 0.5.3 -> 0.5.4 (only in prose, deps already handled)
  $body = [regex]::Replace($body, '(?<!\d)0\.5\.3(?!\d)', '0.5.4')
  # but do NOT let that touch an already-correct dependency band
  $body = $body.Replace('">=0.5.4, <0.5.4"', '">=0.5.4, <0.5.5"')
  $body = [regex]::Replace($body, '"last_updated"\s*:\s*"[^"]*"', ('"last_updated": "' + $Today + '"'))
  if ($body -eq $raw) { Write-Output ("{0,-28} no change" -f $id); continue }
  if ($WhatIf) { Write-Output ("{0,-28} WOULD UPDATE" -f $id); continue }
  [IO.File]::WriteAllText($p, $body, $enc)
  $after = [IO.File]::ReadAllBytes($p)
  $jsonOk = $true; $dep = ''
  try {
    $o = [IO.File]::ReadAllText($p, [Text.Encoding]::UTF8) | ConvertFrom-Json
    $dep = ($o.dependencies | Where-Object { $_.mod_id -eq 'base' }).version
  } catch { $jsonOk = $false }
  Write-Output ("{0,-28} UPDATED  firstByte=0x{1:x2} jsonOk={2} base={3}" -f $id, $after[0], $jsonOk, $dep)
  if (-not $jsonOk) { Write-Output "   !! FAIL: $p no longer parses as json - restore it" }
}
