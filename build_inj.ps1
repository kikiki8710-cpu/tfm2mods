# build_inj.ps1 — SDK + engine_ui 링크해서 mod 빌드·배포 (런타임 노드 구성용).
# 사용: powershell -File build_inj.ps1 -Src <lib.rs 절대경로> -ModId <mod_id>
# (경로에 공백 없음 전제 — SDK/소스는 C:\tfm2mods 아래. 배포대상만 Program Files=Copy-Item 처리)
#
# ⚠ $SDK 는 build_inj_050.ps1 과 동일하게 sdk_050_hotfix 를 가리킨다(=사실상 같은 스크립트).
#   버전을 나눠 쓸 거면 여기 $SDK 만 바꿀 것.
#
# ⚠⚠ 2026-07-08 사고 수정: 구버전은 산출물을 공유 경로 "$SDK\lib.dll" 에 떨어뜨렸다.
#   두 세션이 서로 다른 모드를 동시에 빌드하면 같은 파일을 두고 경쟁 → **남의 모드 dll을 복사 배포**.
#   실제로 Spectator_Chat 배포본이 tfm2_item_tactics dll(694,784B)로 덮였다(무증상, 크기 가드도 통과).
#   게다가 rustc 실패를 stderr 문자열 grep으로만 판정해 **exit code를 안 봤다** → 빌드 실패 시 이전 산출물을 복사.
#   → ①출력/에러 파일을 ModId+PID 로 격리 ②rustc exit code 확인 ③산출물이 "이번 실행"에서 생겼는지 mtime 확인
#     ④배포 전 dll 안에 ModId 문자열이 있는지 신원 검증(타모드/stale 차단. 활성 7개 모드 전부 ModId 문자열 보유 확인).
#     ⑤(2026-07-30 추가) 복사 후 mods 폴더의 배포본 Length 를 산출물과 대조 — "OK: deployed" 줄 자체가
#       배포 증거가 되도록 배포 경로·크기·mtime 을 출력한다(CLAUDE.md §10 증거 기반 완료 보고).
param(
  [Parameter(Mandatory=$true)][string]$Src,
  [Parameter(Mandatory=$true)][string]$ModId,
  # ★사이즈 가드 상한(기본 1.3MB). 정상적으로 큰 모드(ai_adjust·banpick_illust·champ_pos_lock·
  #   banpick_order·champion_exclude·scrim)는 이 값을 올려 통과시킨다. 가드의 목적은
  #   "생각보다 큰 산출물을 모르고 배포하는 것" 방지지 큰 모드 금지가 아니다.
  #   (2026-09-02: 이전엔 수동 rustc + Copy-Item 으로 우회했고, 그 과정에서
  #    stale dll 복사·opt 플래그 누락 같은 사고가 반복됐다 — 같은 경로로 통일.)
  [int]$MaxSize = 1300000
)
# ★ 정식 SDK 사용. mod_sdk\0.4.14 는 비공식 오빌드 = mod_api 내용이
# 게임 핫픽스와 다름(save_probe desync와 동일 함정, 메모리 tfm2-native-mod-loader-abi).
#
# ⚠⚠ 2026-07-08 수정: 구버전은 sdk_050(0.5.0 정식)을 가리켰다 — 게임은 0.5.0_hotfix인데
#   libgame_core/libgame_view rlib 이 달라 desync. "RVA 무영향 ≠ 재빌드 불요"(crm 에서
#   PE .text 섹션 해시 대조로 실증: 구SDK 752D7C76 ≠ hotfix SDK C99BFC83).
#   ⟹ 게임 핫픽스마다 SDK 를 갈고 **전 모드 재빌드**할 것. 정본 = sdk_051 (0.5.1 정식, game_core/view/mod_api rlib 0.5.1).
#   (구: sdk_050_hotfix2 = 0.5.0_3. 0.5.1 전환 2026-07-15 — 전 모드 sdk_051 재빌드 필요.)
#   (0.5.2 전환 2026-07-22: sdk_052. rlib 파일명=StableCrateId 4종 051과 동일(ABI 무변경)이나
#    내용 해시는 전부 다름 ⟹ "RVA 무영향 ≠ 재빌드 불요" 원칙대로 전 모드 sdk_052 재빌드 필요.
#    toolchain은 nightly-2026-05-24로 051과 동일.)
#   (0.5.3 전환 2026-07-29: sdk_053. 게임 rlib 236개 전원 내용 DIFF(전면 재컴파일) + libgame_ai 크레이트 신설
#    ⟹ RVA 0 모드 포함 **전 모드 재빌드 필수**. toolchain은 nightly-2026-05-24로 051/052와 동일.)
#   (0.5.4 전환 2026-08-05: sdk_054. rlib 파일 수 154개로 053과 동일하고 파일명(StableCrateId)도 동일하나
#    mod_api/game_core/game_view/game_ai/engine_ui/engine_core 6종 전부 내용 해시 DIFF
#    ⟹ "RVA 무영향 ≠ 재빌드 불요" 원칙대로 **RVA 0 모드 포함 전 모드 재빌드 필수**.
#    toolchain은 nightly-2026-05-24로 051~053과 동일(rust-toolchain.toml 실측) = 재설치 불요.)
#   (0.5.5 전환 2026-08-12: sdk_055. rlib 154개로 054와 동수. mod_api/game_core/game_ai/game_view 4종은
#    파일명(StableCrateId)까지 변경 + engine_core 등 핵심 전부 내용 해시 DIFF ⟹ **전 모드 재빌드 필수**.
#    toolchain은 nightly-2026-05-24 유지(toolchain_version.txt = rustc 1.98.0-nightly 23a3312d9 실측) = 재설치 불요.)
#   (0.5.6 전환 2026-08-20: sdk_056. rlib 154개 파일명 전원 동일(StableCrateId 무변경)이나
#    내용 해시 DIFF 8종 = 핵심 전부(mod_api/game_core+4MB/game_ai/game_view/engine_core/engine_ui/engine_asset/engine)
#    ⟹ "RVA 무영향 ≠ 재빌드 불요" 원칙대로 **전 모드 재빌드 필수**.
#    toolchain은 nightly-2026-05-24 유지(toolchain_version.txt = rustc 1.98.0-nightly 23a3312d9 실측) = 재설치 불요.)
#   (0.5.8 전환 2026-09-02: sdk_058. toolchain 동일 rustc 1.98.0-nightly(23a3312d9 2026-05-23), rlib 154개로 057과 동수.
#    exe 는 전면 재링크(.pdata 136,233->137,497 = +1,264) -> RVA 전량 재핑 + 전 모드 재빌드.)
$SDK  = "C:\tfm2mods\sdk_058\mod-sdk"
$DEPS = "$SDK\deps"; $NAT = "$SDK\native"
$MODAPI = (Get-ChildItem "$DEPS\libmod_api-*.rlib")[0].FullName
$EUI    = (Get-ChildItem "$DEPS\libengine_ui-*.rlib")[0].FullName
# engine_core: RenderState.commands 의 RenderCommand enum 이 mod_api 미노출이라 직접 링크
# (banpick_illust patchviz 가 사용. 안 쓰는 모드엔 --extern 만 추가돼 무해.)
$ECORE  = (Get-ChildItem "$DEPS\libengine_core-*.rlib")[0].FullName
# ★0.5.4: mod_api 가 game_core 를 더 이상 재수출하지 않는다(`use mod_api::*` 로는 안 잡힘).
#   ai_adjust 의 sp_strat_sig 가 `game_core::Strategy` 를 직접 쓰므로 extern 을 명시한다.
#   안 쓰는 모드엔 --extern 만 늘어나 무해.
$GCORE  = (Get-ChildItem "$DEPS\libgame_core-*.rlib")[0].FullName
# ★0.5.6: game_view 의 set_champion_icon/set_team_logo(ABI level 4·게임 자체 아이콘 크롭)를
#   classic 모드가 쓸 수 있게 링크(champ_pos_lock 등). 안 쓰는 모드엔 --extern 만 늘어나 무해.
$GVIEW  = (Get-ChildItem "$DEPS\libgame_view-*.rlib")[0].FullName
# ★2026-08-22: player_trade_system 이 `common::property_parsable::PropertyParsable` 를 직접 쓴다
#   (mod_api 재수출로는 안 잡힘). 안 쓰는 모드엔 --extern 만 늘어나 무해.
$COMMON = (Get-ChildItem "$DEPS\libcommon-*.rlib")[0].FullName

# ★모드별·프로세스별 격리 작업 폴더 (공유 $SDK\lib.dll 사용 금지)
$work = Join-Path $env:TEMP "tfm2_build\$ModId`_$PID"
New-Item -ItemType Directory -Force -Path $work | Out-Null
$out  = "$work\$ModId.dll"
$errf = "$work\rustc_err.txt"
$dep  = "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\$ModId"

$env:RUSTFLAGS = "-C overflow-checks=off"   # ⚠ cmd /c 자식에 전달 안 됨(실효 없음!) — 실제 플래그는 L41 rustc 명령줄에 직접. 2026-07-18까지 이게 안 먹혀서 전 모드가 opt-level=0 디버그빌드로 컴파일됨(~5배 느림).
$started = Get-Date
# cmd 로 실행해서 stderr 리다이렉트 (PowerShell 의 native-stderr 래핑 회피)
# ⚠⚠ 2026-07-29 (0.5.3): MSVC 2019 link.exe(14.29.30133) 가 sdk_053 rlib 을 읽다 죽는다
#   — `LNK1107: 파일이 잘못되었거나 손상되었습니다. 0x55E40에서 읽을 수 없습니다`(= libmod_api rlib 파일 끝 오프셋).
#   rlib 자체는 정상(ar 구조·심볼테이블 오프셋 전부 파일 내 유효, zip↔추출본 329/329 크기 일치).
#   미해결 심볼이 생겨 링커가 아카이브를 재스캔할 때만 터진다(심볼 0인 SDK 템플릿은 MSVC 로도 링크 성공).
#   ⟹ 툴체인 동봉 `rust-lld` 로 전환하면 동일 소스가 그대로 링크된다(0.5.3 전 모드 확인).
cmd /c "rustup run nightly-2026-05-24 rustc --crate-type cdylib --edition 2021 -C opt-level=1 -C overflow-checks=off -C linker-flavor=lld-link -C linker=rust-lld -L dependency=$DEPS -L native=$NAT --extern mod_api=$MODAPI --extern engine_ui=$EUI --extern engine_core=$ECORE --extern game_core=$GCORE --extern game_view=$GVIEW --extern common=$COMMON $Src -o `"$out`" 2> `"$errf`""
$rc = $LASTEXITCODE

# ① rustc exit code 우선 (문자열 grep 은 보조)
if ($rc -ne 0) {
  Write-Output "=== BUILD FAILED (rustc exit $rc) ==="
  if (Test-Path $errf) { Get-Content $errf | Select-String -Pattern "error\[|^error:" | Select-Object -First 40 | ForEach-Object { Write-Output $_.Line } }
  exit 1
}
$errs = @()
if (Test-Path $errf) { $errs = Get-Content $errf | Select-String -Pattern "error\[|^error:" }
if ($errs.Count -gt 0) { Write-Output "=== BUILD ERRORS ==="; $errs | Select-Object -First 40 | ForEach-Object { Write-Output $_.Line }; exit 1 }
if (-not (Test-Path $out)) { Write-Output "FAIL: $ModId.dll not produced"; exit 1 }

# ② 산출물이 이번 실행에서 생겼는지 (stale 차단)
$item = Get-Item $out
if ($item.LastWriteTime -lt $started) { Write-Output "FAIL: stale dll (mtime $($item.LastWriteTime) < build start $started)"; exit 1 }
$sz = $item.Length
if ($sz -ge $MaxSize) { Write-Output "FAIL: oversized dll ($sz >= $MaxSize) - 사이즈 가드 (-MaxSize 로 상한 조정)"; exit 1 }

# ③ 신원 검증: dll 안에 "이번에 컴파일한 소스의 절대경로" 문자열이 박혀 있어야 한다.
#    rustc 가 panic location 등으로 소스 경로를 PE 에 박으므로, 이 dll 이 정말 $Src 에서
#    나왔는지 증명한다. 활성 7개 모드 전부 자기 소스경로 보유 + 타모드 경로 0 확인(2026-07-08).
#    ⚠ ModId 문자열로 검사하면 안 된다 — `-o <ModId>.dll` 이라 PE 헤더에 그 파일명이 박혀
#      **어떤 dll이든 무조건 통과**한다(가드 무력화. 실제로 이 실수로 tfm2_fog 배포본을 덮었다).
$srcFull = (Resolve-Path $Src).Path
$bytes = [System.IO.File]::ReadAllBytes($out)
$needle = [System.Text.Encoding]::ASCII.GetBytes($srcFull)
$found = $false
for ($i = 0; $i -le ($bytes.Length - $needle.Length); $i++) {
  if ($bytes[$i] -eq $needle[0]) {
    $ok = $true
    for ($j = 1; $j -lt $needle.Length; $j++) { if ($bytes[$i+$j] -ne $needle[$j]) { $ok = $false; break } }
    if ($ok) { $found = $true; break }
  }
}
if (-not $found) {
  Write-Output "FAIL: 신원 검증 실패 - dll 안에 소스 경로 '$srcFull' 없음 (타 모드/stale dll 의심). 배포 중단."
  Write-Output "      built: $out ($sz bytes)"
  exit 1
}

try {
  New-Item -ItemType Directory -Force -Path $dep | Out-Null
  Copy-Item $out "$dep\$ModId.dll" -Force -ErrorAction Stop
  # ⑤ 배포본 검증: mods 폴더에 실제로 복사됐고 방금 빌드한 산출물과 크기가 같은지
  $deployed = Get-Item "$dep\$ModId.dll" -ErrorAction Stop
  if ($deployed.Length -ne $sz) {
    Write-Output "FAIL: 배포본 크기 불일치 - $dep\$ModId.dll = $($deployed.Length) bytes, 산출물 = $sz bytes. 배포 실패로 간주."
    exit 1
  }
  Write-Output "OK: deployed $ModId.dll = $sz bytes @ $($deployed.LastWriteTime) -> $dep\$ModId.dll"
} catch {
  Write-Output "LOCKED: game running? build ok (size=$sz) but copy to mods\$ModId failed"
  Write-Output "        built dll kept at: $out"
  exit 2
}
