# rel_commit.ps1 — 릴리스와 git 커밋을 한 동작으로 묶는다.
#
# 왜 있나 (2026-08-02 신설)
#   릴리스 zip 은 "결과물"만 남기고 그걸 만든 소스 상태는 안 남는다. 실제로 2026-07-31 작업분이
#   커밋 없이 쌓여, 배포된 ai_adjust dll 에 대응하는 소스가 무엇인지 파일 mtime + dll 문자열 대조로
#   역추적해야 했다(미빌드 변경분 ct_hunt 를 그렇게 찾아냈다). 그 반복을 막는 게 목적이다.
#
# 사용법
#   powershell -File C:\tfm2mods\rel_commit.ps1 -ModId tfm2_elemental_serpen -Summary "성능 개편: 전역 락 제거"
#   powershell -File C:\tfm2mods\rel_commit.ps1 -ModId tfm2_item_tactics -DryRun      # 메시지만 확인
#
#   -Summary  : 사람이 쓰는 한 줄~여러 줄 변경점 요약(권장). 없으면 diff 통계만 들어간다.
#   -Base     : 게임 버전 폴더(기본 0.5.3) = release\<Base>\<ModId>.zip
#   -NoZip    : 릴리스 zip 이 아직 없을 때(배포만 하고 커밋)
#   -DryRun   : 커밋하지 않고 메시지와 검사 결과만 출력
param(
    [Parameter(Mandatory = $true)][string]$ModId,
    [string]$Summary = "",
    [string]$Base = "0.5.3",
    [switch]$NoZip,
    [switch]$DryRun
)

$ErrorActionPreference = "Stop"
$REPO = "C:\tfm2mods"
$GAME = "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods"

$srcDir = Join-Path $REPO $ModId
if (-not (Test-Path $srcDir)) { Write-Error "소스 폴더 없음: $srcDir"; exit 1 }

# ── 1) 배포 사실 수집 (증거 기반 — 추정 금지) ────────────────────────────────
$liveDll = Join-Path $GAME "$ModId\$ModId.dll"
$dllLine = "  dll        : (배포본 없음)"
$liveLen = -1
if (Test-Path $liveDll) {
    $d = Get-Item $liveDll
    $liveLen = $d.Length
    $dllLine = "  dll        : {0:N0}B @{1:yyyy-MM-dd HH:mm:ss}" -f $d.Length, $d.LastWriteTime
}

$mi = Join-Path $GAME "$ModId\mod.mod_info"
if (-not (Test-Path $mi)) { $mi = Join-Path $srcDir "mod.mod_info" }
$verLine = "  version    : (mod_info 없음)"
if (Test-Path $mi) {
    $raw = [System.IO.File]::ReadAllBytes($mi)
    $bom = ($raw.Length -ge 3 -and $raw[0] -eq 0xEF -and $raw[1] -eq 0xBB -and $raw[2] -eq 0xBF)
    try {
        $j = Get-Content $mi -Raw -Encoding UTF8 | ConvertFrom-Json
        $deps = ($j.dependencies | ForEach-Object { "$($_.mod_id) $($_.version)" }) -join ", "
        $verLine = "  version    : v{0}  deps=[{1}]  BOM={2}" -f $j.version, $deps, $(if ($bom) { "★있음(불량)" } else { "없음" })
    } catch {
        $verLine = "  version    : ⚠mod_info JSON 파싱 실패"
    }
}

$zipLine = "  release zip: (미생성)"
if (-not $NoZip) {
    $zip = Join-Path $GAME "release\$Base\$ModId.zip"
    if (Test-Path $zip) {
        Add-Type -AssemblyName System.IO.Compression.FileSystem
        $z = [System.IO.Compression.ZipFile]::OpenRead($zip)
        $n = $z.Entries.Count
        $ze = $z.Entries | Where-Object { $_.Name -eq "$ModId.dll" } | Select-Object -First 1
        $zlen = -1
        if ($ze) { $zlen = $ze.Length }
        $z.Dispose()
        $zipLine = "  release zip: {0:N0}B  엔트리 {1}  (release\{2}\)" -f (Get-Item $zip).Length, $n, $Base
        if ($zlen -ge 0 -and $liveLen -ge 0 -and $zlen -ne $liveLen) {
            $zipLine += "`n  ⚠경고      : zip 안 dll {0:N0}B ≠ 배포본 {1:N0}B — zip 이 낡았다(rel_one.py 로 재생성 권장)" -f $zlen, $liveLen
        }
    } else {
        $zipLine = "  release zip: 없음 (release\$Base\$ModId.zip)"
    }
}

# ── 2) 스테이징 ──────────────────────────────────────────────────────────────
Push-Location $REPO
git add -- "$ModId" | Out-Null
$staged = git diff --cached --name-only -- "$ModId"
if (-not $staged) {
    Write-Host "커밋할 변경 없음: $ModId (소스·mod_info 모두 이전 커밋과 동일)"
    Pop-Location
    exit 0
}
$stat = (git diff --cached --stat -w -- "$ModId" | Out-String).TrimEnd()

# ── 3) 메시지 조립 ───────────────────────────────────────────────────────────
$head = "release: $ModId"
if (Test-Path $mi) {
    try { $head = "release: $ModId v$($j.version)" } catch { }
}
$lines = @()
$lines += $head
$lines += ""
if ($Summary -ne "") {
    $lines += $Summary
    $lines += ""
}
$lines += "배포 사실 (rel_commit.ps1 자동 수집, 게임 $Base)"
$lines += $dllLine
$lines += $verLine
$lines += $zipLine
$lines += ""
$lines += "변경 파일"
$lines += $stat
$lines += ""
$lines += "Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>"
$msg = ($lines -join "`n")

if ($DryRun) {
    Write-Host "--- DryRun: 아래 메시지로 커밋됩니다 ---`n"
    Write-Host $msg
    git reset -q -- "$ModId"
    Pop-Location
    exit 0
}

# ⚠BOM 없는 UTF-8 로 써야 커밋 메시지 한글이 깨지지 않는다
# ⚠$env:TEMP 가 비어 있는 실행 환경이 있다(자동화·샌드박스 셸에서 실측 2026-08-03).
#   그대로 Join-Path 하면 빈 경로가 되어 "Empty path name is not legal" 로 죽는다.
$tmpDir = $env:TEMP
if ([string]::IsNullOrWhiteSpace($tmpDir)) { $tmpDir = [System.IO.Path]::GetTempPath() }
if ([string]::IsNullOrWhiteSpace($tmpDir)) { $tmpDir = $REPO }
$tmp = Join-Path $tmpDir "relcommit_$ModId.msg"
[System.IO.File]::WriteAllText($tmp, $msg, (New-Object System.Text.UTF8Encoding($false)))
git commit -F $tmp | Write-Host
Remove-Item $tmp -Force -ErrorAction SilentlyContinue
Pop-Location
