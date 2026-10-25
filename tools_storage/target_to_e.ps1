# 크레이트 빌드 산출물(target\)을 E:\claude\tfm2\tfm2mods\<상대경로>\target 에 두고 원위치에 정션을 건다 (2026-10-21).
# 공용 CARGO_TARGET_DIR 는 <crate>\target\release\*.dll 을 가정한 스크립트(build_local.ps1·deploy_stable.py·rel_commit.ps1)를 깨므로 쓰지 않는다.
# 사용: powershell -File target_to_e.ps1 -Crate F:\claude\tfm2mods\<crate>   (한 크레이트)
#       powershell -File target_to_e.ps1 -All                       (F:\claude\tfm2mods 아래 깊이 3 까지 Cargo.toml 이 있는 폴더 전부)
# 이미 정션이면 건너뛴다. target 이 실제 폴더로 있으면 robocopy 로 옮긴 뒤 정션. 없으면 E: 쪽 빈 폴더를 만들고 정션.
param([string]$Crate, [switch]$All)
$ErrorActionPreference = 'Stop'
$Root = 'F:\claude\tfm2mods'; $ERoot = 'E:\claude\tfm2\tfm2mods'
if (-not (Test-Path 'E:\claude\tfm2')) { Write-Output 'E:\claude\tfm2 없음 — 건너뜀'; exit 0 }
if (Get-Process cargo, rustc -ErrorAction SilentlyContinue) { Write-Output '빌드 중(cargo/rustc) — 끝난 뒤 다시 실행'; exit 2 }

# ★2026-10-25 안전장치: target 안에 정션·심볼릭 링크가 하나라도 있으면 거부(robocopy /E 는 정션을 따라가 복사하고,
#   PS 5.1 Remove-Item -Recurse 는 정션 너머 E: 실데이터까지 지울 수 있다). 그런 경우 = F:\claude\_tools\storage\offload_to_e.ps1.
function Find-Reparse([string]$root) {
    $stack = New-Object System.Collections.Generic.Stack[string]
    $stack.Push($root)
    while ($stack.Count -gt 0) {
        $d = $stack.Pop()
        foreach ($e in [IO.Directory]::EnumerateFileSystemEntries($d)) {
            $a = [IO.File]::GetAttributes($e)
            if ($a -band [IO.FileAttributes]::ReparsePoint) { return $e }
            if ($a -band [IO.FileAttributes]::Directory) { $stack.Push($e) }
        }
    }
    return $null
}

$crates = if ($All) {
    Get-ChildItem $Root -Recurse -Depth 2 -Filter Cargo.toml -File -ErrorAction SilentlyContinue |
        Where-Object { $_.FullName -notmatch '\\(target|_archive|\.git)\\' } | ForEach-Object { $_.DirectoryName }
} else { @((Resolve-Path $Crate).Path) }

foreach ($c in $crates) {
    if (-not $c.ToLower().StartsWith($Root.ToLower())) { Write-Output "SKIP(밖) $c"; continue }
    $t = Join-Path $c 'target'
    $dst = Join-Path $ERoot ($c.Substring($Root.Length).TrimStart('\') + '\target')
    $it = Get-Item -LiteralPath $t -Force -ErrorAction SilentlyContinue
    if ($it -and ($it.Attributes -band [IO.FileAttributes]::ReparsePoint)) { continue }
    if ($it) { $rp = Find-Reparse $t; if ($rp) { Write-Output "REFUSE(안에 정션/링크: $rp) $t — offload_to_e.ps1 을 쓸 것"; continue } }
    New-Item -ItemType Directory -Force -Path $dst | Out-Null
    if ($it) {
        robocopy $t $dst /E /COPY:DAT /DCOPY:T /R:1 /W:1 /NFL /NDL /NP /NJH /NJS /MT:8 | Out-Null
        if ($LASTEXITCODE -ge 8) { Write-Output "FAIL robocopy $t"; continue }
        Remove-Item -LiteralPath $t -Recurse -Force
    }
    cmd /c mklink /J "$t" "$dst" | Out-Null
    Write-Output ("{0} {1} -> {2}" -f ($(if ($it) { 'MOVED' } else { 'LINKED' })), $t, $dst)
}
