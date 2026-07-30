param(
    [string]$SdkDir = "",
    [string]$RustupToolchain = ""
)

$ErrorActionPreference = "Stop"

$script = Join-Path $PSScriptRoot "build_mod_047.ps1"
$params = @{}
if ($SdkDir) {
    $params["SdkDir"] = $SdkDir
}
if ($RustupToolchain) {
    $params["RustupToolchain"] = $RustupToolchain
}

& $script @params
exit $LASTEXITCODE
