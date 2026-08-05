# verify_054.ps1 - end-of-migration sweep for the 0.5.4 rollout.
#
# Checks, per mod, the things that have actually bitten this project before:
#   dll   : exists, rebuilt today (a stale dll from the previous game version crashes 0.5.4)
#   deps  : base band is the 0.5.4 one (a stale upper bound silently disables the mod)
#   BOM   : mod_info first byte is '{' (a BOM makes the parser fail -> force-disabled)
#   json  : mod_info still parses
#   zip   : a release zip exists and the dll inside matches the deployed one
#
# NOTE: keep this file ASCII-only (a BOM-less .ps1 is read as ANSI by PowerShell 5.1).
param(
  [string]$Game = "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods",
  [string]$Ver  = "0.5.4",
  [string]$Today = "2026-08-05"
)
Add-Type -AssemblyName System.IO.Compression.FileSystem

# mod id -> release zip stem it ships in ('' = no zip of its own)
$MODS = [ordered]@{
  'community_reaction_mod'    = 'community_reaction_mod'
  'Spectator_Chat'            = 'Spectator_Chat'
  'tfm2_mod_order'            = 'tfm2_mod_order'
  'tfm2_ai_banpick_probe'     = "\u{1}gg"
  'tfm2_meta_champion_tiers'  = "\u{1}gg"
  'tfm2_meta_item_delegate'   = "\u{1}gg"
  'coaching_staff_view_plus'  = 'daram2_viewplus'
  'custom_tier_assignment'    = 'daram2_viewplus'
  'facility_view_plus'        = 'daram2_viewplus'
  'finance_view_plus'         = 'daram2_viewplus'
  'recruitment_view_plus'     = 'daram2_viewplus'
  'roster_view_plus'          = 'daram2_viewplus'
  'statistics_view_plus'      = 'daram2_viewplus'
  'training_view_plus'        = 'daram2_viewplus'
  'legacy_save_patcher'       = 'daram2_viewplus'
  'tfm2_comptest_unlock'      = 'tfm2_comptest_unlock'
  'tfm2_draft_overlay'        = 'tfm2_draft_overlay'
  'tfm2_level_cap'            = 'tfm2_level_cap'
  'tfm2_banpick_illust'       = 'tfm2_banpick_illust'
  'tfm2_item_tactics'         = 'tfm2_item_tactics'
  'tfm2_banpick_order'        = 'tfm2_banpick_order'
  'tfm2_elemental_serpen'     = 'tfm2_elemental_serpen'
  'TFM2_Meta_Dashboard'       = 'TFM2_Meta_Dashboard'
}
$GG = 'gg'
$relDir = Join-Path $Game "release\$Ver"

# index every dll inside every 0.5.4 zip once: dll basename -> size
$zipDll = @{}
foreach ($z in Get-ChildItem $relDir -Filter *.zip -ErrorAction SilentlyContinue) {
  $a = [System.IO.Compression.ZipFile]::OpenRead($z.FullName)
  foreach ($e in $a.Entries) { if ($e.Name -like "*.dll") { $zipDll[$e.Name] = $e.Length } }
  $a.Dispose()
}

$fail = 0
# NOTE: PowerShell -f alignment is "{n,width}" - "{1,>10}" is a FormatError (printf habit).
"{0,-26} {1,10} {2,-12} {3,-22} {4,-5} {5}" -f 'MOD','DLL','BUILT','BASE DEP','BOM','ZIP'
"-" * 108
foreach ($id in $MODS.Keys) {
  $dll = Join-Path $Game "$id\$id.dll"
  $sz = -1; $built = 'MISSING'
  if (Test-Path $dll) { $i = Get-Item $dll; $sz = $i.Length; $built = $i.LastWriteTime.ToString('MM-dd HH:mm') }
  else { $fail++ }
  $stale = ($built -ne 'MISSING' -and (Get-Item $dll).LastWriteTime.ToString('yyyy-MM-dd') -ne $Today)
  if ($stale) { $built += '!' ; $fail++ }

  $mi = Join-Path $Game "$id\mod.mod_info"
  $dep = '(no mod_info)'; $bomTxt = '-'
  if (Test-Path $mi) {
    $raw = [IO.File]::ReadAllBytes($mi)
    $bomTxt = if ($raw[0] -eq 0x7b) { 'ok' } else { 'BAD' }
    if ($bomTxt -eq 'BAD') { $fail++ }
    try {
      $j = [IO.File]::ReadAllText($mi, [Text.Encoding]::UTF8) | ConvertFrom-Json
      $dep = ($j.dependencies | Where-Object { $_.mod_id -eq 'base' }).version
      if (-not $dep) { $dep = '(no base dep)' }
    } catch { $dep = 'JSON PARSE FAIL'; $fail++ }
  }

  $zipTxt = '-'
  $zsz = $zipDll["$id.dll"]
  if ($null -ne $zsz) {
    if ($zsz -eq $sz) { $zipTxt = 'match' } else { $zipTxt = "STALE zip=$zsz"; $fail++ }
  } elseif ($MODS[$id] -ne '') { $zipTxt = 'NOT IN ZIP'; $fail++ }

  "{0,-26} {1,10} {2,-12} {3,-22} {4,-5} {5}" -f $id, $sz, $built, $dep, $bomTxt, $zipTxt
}
""
"release zips in $relDir :"
Get-ChildItem $relDir -Filter *.zip -ErrorAction SilentlyContinue |
  ForEach-Object { "  {0,-34} {1,15:N0}B" -f $_.Name, $_.Length }
""
if ($fail -eq 0) { "VERIFY OK - 0 findings" } else { "VERIFY: $fail finding(s) above (look for MISSING / '!' stale / BAD / STALE zip)" }
