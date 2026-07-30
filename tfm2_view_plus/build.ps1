# tfm2_view_plus 빌드 — mod_api + game_core 둘 다 링크(하위 게임구조체 타입드 접근).
# 게임 패치 시: sdk_050_hotfix 를 새 SDK 로 교체 후 이 스크립트만 재실행.
# (링커가 .lib/.exp 생성 안내를 stderr 로 내므로 ErrorActionPreference=Stop 금지 — LASTEXITCODE 로 판정)
$ErrorActionPreference = "Continue"
$env:NoDefaultCurrentDirectoryInExePath = ""
$SDK = "C:\tfm2mods\sdk_050_hotfix\mod-sdk"
$DEPS = "$SDK\deps"
$NATIVE = "$SDK\native"
$MOD_API = (Get-ChildItem "$DEPS\libmod_api-*.rlib")[0].FullName
$GAME_CORE = (Get-ChildItem "$DEPS\libgame_core-*.rlib")[0].FullName
$TOOLCHAIN = "nightly-2026-05-24"

Set-Location "C:\tfm2mods\tfm2_view_plus"
& rustup run $TOOLCHAIN rustc --crate-type cdylib --edition 2021 `
    -L "dependency=$DEPS" `
    -L "native=$NATIVE" `
    --extern "mod_api=$MOD_API" `
    --extern "game_core=$GAME_CORE" `
    src\lib.rs -o tfm2_view_plus.dll
if ($LASTEXITCODE -eq 0) {
    $sz = (Get-Item tfm2_view_plus.dll).Length
    Write-Output "Build successful: tfm2_view_plus.dll ($sz bytes)"
} else {
    Write-Output "Build failed"
    exit 1
}
