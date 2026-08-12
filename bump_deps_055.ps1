# bump_deps_055.ps1 - raise base dependency of ALREADY-REBUILT mods to the 0.5.5 band.
#
# Policy (user directive 2026-07-30, carried into 0.5.5): base dep = ">=<current>, <<next>>"
#   so that when the next patch lands, stale dlls auto-disable at the loader and cannot crash the game.
# WARNING: apply ONLY to mods whose dll has already been rebuilt against sdk_055.
#          The upper bound (<0.5.5) on a not-yet-rebuilt mod IS the safety net.
# WARNING: the game parses these json files as UTF-8 WITHOUT BOM. A BOM makes the parser
#          fail and the mod is force-disabled. PowerShell 5.1's `Set-Content -Encoding utf8`
#          writes a BOM, so we write via UTF8Encoding($false) instead.
# NOTE: keep this file ASCII-only. PowerShell 5.1 reads a BOM-less .ps1 as ANSI, which
#       mangles non-ASCII text and can break parsing.
#
# usage: powershell -File bump_deps_055.ps1 -ModIds a,b,c
#        powershell -File bump_deps_055.ps1 -ModIds a -WhatIf
param(
  [Parameter(Mandatory=$true)][string[]]$ModIds,
  [string]$Root = "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods",
  [switch]$WhatIf
)
# NOTE: PowerShell variable names are CASE-INSENSITIVE. An earlier version of this
#       script used $NEW for the target dep string and $new for the rewritten file body;
#       $new clobbered $NEW, so the SECOND mod processed had the FIRST mod's entire json
#       spliced into its version field (corrupted tfm2_mod_order/mod.mod_info, 2026-08-05).
#       Keep these names distinct.
# The band is not written uniformly across mods: some use ">=0.5.4, <0.5.5" and some
# ">=0.5.4,<0.5.5" (no space). Match on a regex, not a literal.
$OLDRX = '>=\s*0\.5\.4\s*,\s*<\s*0\.5\.5'
$NEWDEP = '>=0.5.5, <0.5.6'
$enc = New-Object System.Text.UTF8Encoding($false)   # no BOM
foreach ($id in $ModIds) {
  $p = Join-Path $Root "$id\mod.mod_info"
  if (-not (Test-Path $p)) { Write-Output ("{0,-28} SKIP: no mod_info" -f $id); continue }
  $raw = [IO.File]::ReadAllText($p, [Text.Encoding]::UTF8)
  $b = [IO.File]::ReadAllBytes($p)
  $hadBom = ($b[0] -eq 0xEF -and $b[1] -eq 0xBB -and $b[2] -eq 0xBF)
  if ($raw -notmatch $OLDRX) {
    if ($raw -match [regex]::Escape($NEWDEP)) {
      Write-Output ("{0,-28} already 0.5.5 band" -f $id)
    } else {
      $m = [regex]::Match($raw, '"version"\s*:\s*"(>=[^"]*)"')
      $cur = if ($m.Success) { $m.Groups[1].Value } else { 'not-detected' }
      Write-Output ("{0,-28} WARN: expected string absent (current base dep: {1})" -f $id, $cur)
    }
    continue
  }
  $body = [regex]::Replace($raw, $OLDRX, $NEWDEP)
  if ($WhatIf) { Write-Output ("{0,-28} WOULD BUMP (hadBom={1})" -f $id, $hadBom); continue }
  [IO.File]::WriteAllText($p, $body, $enc)
  $after = [IO.File]::ReadAllBytes($p)
  $fb = $after[0]
  $ok = ($fb -eq 0x7b)
  # sanity: must still parse as json, and size must not have exploded (catches variable clobber)
  $delta = $after.Length - $b.Length
  $jsonOk = $true
  try { [void]([IO.File]::ReadAllText($p, [Text.Encoding]::UTF8) | ConvertFrom-Json) } catch { $jsonOk = $false }
  Write-Output ("{0,-28} BUMPED  firstByte=0x{1:x2} noBom={2} deltaBytes={3} jsonOk={4}" -f $id, $fb, $ok, $delta, $jsonOk)
  if (-not $ok)     { Write-Output ("   !! FAIL: first byte of $p is not '{' - parser will reject") }
  if (-not $jsonOk) { Write-Output ("   !! FAIL: $p no longer parses as json - restore it") }
}
