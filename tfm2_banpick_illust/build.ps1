# tfm2_banpick_illust 빌드 — 순수 SDK(mod_api/engine_ui/engine_core 링크).
# 사이즈가드(1MB) 초과 모드라 build_inj.ps1 대신 rustc 직접 + Copy-Item 배포.
# 게임 패치 시: $SDK 를 새 SDK 경로로 바꾸고 이 스크립트만 재실행.
# (링커가 .lib/.exp 안내를 stderr 로 내므로 ErrorActionPreference=Stop 금지 — LASTEXITCODE 로 판정)
$ErrorActionPreference = "Continue"
$env:NoDefaultCurrentDirectoryInExePath = ""

# 0.5.4 전환(2026-08-05): 게임 rlib 전원 내용 DIFF ⟹ 재빌드 필수.
# (툴체인 = nightly-2026-05-24, sdk_054 의 rust-toolchain.toml 실측 = 0.5.3 과 동일)
$SDK       = "C:\tfm2mods\sdk_057\mod-sdk"
$DEPS      = "$SDK\deps"
$NATIVE    = "$SDK\native"
$TOOLCHAIN = "nightly-2026-05-24"
$MOD_API   = (Get-ChildItem "$DEPS\libmod_api-*.rlib")[0].FullName
$EUI       = (Get-ChildItem "$DEPS\libengine_ui-*.rlib")[0].FullName
$ECORE     = (Get-ChildItem "$DEPS\libengine_core-*.rlib")[0].FullName

Set-Location "C:\tfm2mods\tfm2_banpick_illust"

# ★0.5.3: MSVC link.exe 는 sdk_053 rlib 재스캔 중 LNK1107 로 죽는다 ⟹ rust-lld 필수.
& rustup run $TOOLCHAIN rustc --crate-type cdylib --edition 2021 `
    -C opt-level=1 -C overflow-checks=off `
    -C linker-flavor=lld-link -C linker=rust-lld `
    -L "dependency=$DEPS" `
    -L "native=$NATIVE" `
    --extern "mod_api=$MOD_API" `
    --extern "engine_ui=$EUI" `
    --extern "engine_core=$ECORE" `
    src\lib.rs -o tfm2_banpick_illust.dll

if ($LASTEXITCODE -eq 0) {
    $f = Get-Item tfm2_banpick_illust.dll
    Write-Output "Build successful: tfm2_banpick_illust.dll ($($f.Length) bytes, $($f.LastWriteTime))"
} else {
    Write-Output "Build failed"
    exit 1
}
