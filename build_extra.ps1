# build_extra.ps1 - build+deploy a mod that needs EXTRA --extern crates beyond what
# build_inj.ps1 passes (mod_api / engine_ui / engine_core), or that exceeds its 1MB size guard.
#
# Keeps every guard build_inj.ps1 has, because those guards exist for real incidents:
#   (1) rustc exit code is authoritative (a stderr grep alone once deployed a stale dll)
#   (2) artifact mtime must be >= build start   -> blocks stale dll
#   (3) the dll must contain the absolute source path rustc embeds -> proves THIS source
#       produced it. Do NOT check for the ModId string instead: `-o <ModId>.dll` puts that
#       name in the PE header, so every dll would pass and the guard would be worthless.
#   (4) after Copy-Item, re-stat the deployed file and compare Length -> "deployed" is only
#       claimed with evidence (CLAUDE.md section 10).
# Work dir is isolated per mod+PID; a shared output path once let one mod's dll overwrite another's.
#
# NOTE: keep this file ASCII-only (a BOM-less .ps1 is read as ANSI by PowerShell 5.1).
#
# usage:
#   powershell -File build_extra.ps1 -Src <lib.rs abs> -ModId <id> -Externs game_core,game_view
#   ... -MaxSize 4000000       (raise the size guard for statically-linked-asset mods)
#   ... -NoDeploy              (build only)
param(
  [Parameter(Mandatory=$true)][string]$Src,
  [Parameter(Mandatory=$true)][string]$ModId,
  [string[]]$Externs = @(),
  [int]$MaxSize = 1300000,
  [switch]$NoDeploy,
  [string]$Sdk = "C:\tfm2mods\sdk_056\mod-sdk",
  # Fallback identity marker for mods that embed NO absolute source path.
  # rustc only bakes the source path in via panic locations; a mod with no reachable panic
  # site has none (verified 2026-08-05: tfm2_meta_champion_tiers, both the fresh build and
  # the shipped 0.5.3 dll, contain zero ".rs" paths). Pass a string that comes from THIS
  # mod's source - e.g. a filename it opens - not the mod id: `-o <ModId>.dll` puts the mod
  # id in the PE header, so an id check would pass for any dll and prove nothing.
  [string]$IdentityString = ""
)
$DEPS = "$Sdk\deps"; $NAT = "$Sdk\native"
$ext = @()
foreach ($c in @('mod_api','engine_ui','engine_core') + $Externs) {
  $r = Get-ChildItem "$DEPS\lib$c-*.rlib" -ErrorAction SilentlyContinue | Select-Object -First 1
  if (-not $r) { Write-Output "FAIL: rlib for '$c' not found in $DEPS"; exit 1 }
  $ext += "--extern $c=$($r.FullName)"
}
$extStr = $ext -join ' '
$work = Join-Path $env:TEMP "tfm2_build\$ModId`_$PID"
New-Item -ItemType Directory -Force -Path $work | Out-Null
$out = "$work\$ModId.dll"; $errf = "$work\rustc_err.txt"
$started = Get-Date

# opt-level=1 is deliberate: opt-level=2/3 inflates reproduced-detour frames and crashes the
# game with STATUS_STACK_OVERFLOW (rayon worker small stacks + recursive sim). 2026-07-18.
# Flags go on the rustc command line, NOT $env:RUSTFLAGS - the latter is not inherited by the
# cmd /c -> rustup -> rustc child chain, which silently gave every mod a debug build until 07-18.
cmd /c "rustup run nightly-2026-05-24 rustc --crate-type cdylib --edition 2021 -C opt-level=1 -C overflow-checks=off -C linker-flavor=lld-link -C linker=rust-lld -L dependency=$DEPS -L native=$NAT $extStr $Src -o `"$out`" 2> `"$errf`""
$rc = $LASTEXITCODE
if ($rc -ne 0) {
  Write-Output "=== BUILD FAILED (rustc exit $rc) ==="
  if (Test-Path $errf) { Get-Content $errf | Select-String -Pattern 'error\[|^error:' | Select-Object -First 40 | ForEach-Object { Write-Output $_.Line } }
  exit 1
}
if (-not (Test-Path $out)) { Write-Output "FAIL: $ModId.dll not produced"; exit 1 }
$item = Get-Item $out
if ($item.LastWriteTime -lt $started) { Write-Output "FAIL: stale dll (mtime $($item.LastWriteTime) < start $started)"; exit 1 }
$sz = $item.Length
if ($sz -ge $MaxSize) { Write-Output "FAIL: oversized dll ($sz >= $MaxSize) - raise -MaxSize only if this mod is known to link static assets"; exit 1 }

$srcFull = (Resolve-Path $Src).Path
$blob = [Text.Encoding]::ASCII.GetString([IO.File]::ReadAllBytes($out))
$found = $blob.Contains($srcFull)
$how = "source path"
if (-not $found -and $IdentityString -ne "") {
  $found = $blob.Contains($IdentityString)
  $how = "marker '$IdentityString'"
}
if (-not $found) {
  Write-Output "FAIL: identity check - neither '$srcFull' nor the marker is embedded in dll (other mod / stale). built: $out ($sz bytes)"
  Write-Output "      If this mod genuinely embeds no source path, pass -IdentityString <a string unique to this mod's source>."
  exit 1
}
Write-Output "identity OK via $how"
if ($NoDeploy) { Write-Output "OK: built (not deployed) $ModId.dll = $sz bytes -> $out"; exit 0 }

$dep = "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\$ModId"
try {
  New-Item -ItemType Directory -Force -Path $dep | Out-Null
  Copy-Item $out "$dep\$ModId.dll" -Force -ErrorAction Stop
  $deployed = Get-Item "$dep\$ModId.dll" -ErrorAction Stop
  if ($deployed.Length -ne $sz) { Write-Output "FAIL: deployed size $($deployed.Length) != artifact $sz"; exit 1 }
  Write-Output "OK: deployed $ModId.dll = $sz bytes @ $($deployed.LastWriteTime) -> $dep\$ModId.dll"
} catch {
  Write-Output "FAIL: deploy error (game running = dll locked?): $($_.Exception.Message)"
  exit 2
}
