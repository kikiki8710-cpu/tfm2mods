param(
    [string]$SdkDir = ""
)

$ErrorActionPreference = "Stop"

$script = Join-Path $PSScriptRoot "build_mod_047.ps1"
if ($SdkDir) {
    & $script -SdkDir $SdkDir
} else {
    & $script
}
exit $LASTEXITCODE
