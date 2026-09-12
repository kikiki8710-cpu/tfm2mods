# UserPromptSubmit 훅 — RE/구현 "착수성" 프롬프트에만 §7·§9 리마인더 주입 + 프롬프트 내 심볼이 DONE.md에 있으면 콕 집어 경고.
# 구버전은 키워드(모드/찾아/이미/구조/메모리 등)가 너무 넓어 매 프롬프트 배너화(소음) → 착수성 동사로 축소(2026-07-11).
$ErrorActionPreference = 'SilentlyContinue'
# ★경로 해석 — 하드코딩 금지. (2026-09-12 /dream: `C:\Users\dev` 하드코딩으로 이 훅이
#   오랫동안 **무동작**이었다. 그 사이 메모리 29개가 15KB 상한을 총 250KB 초과했는데
#   아무 경고도 안 떴다. 계정·머신이 바뀌어도 살아 있도록 실행 시점에 찾는다.)
$mem = ''
$projRoot = Join-Path $env:USERPROFILE '.claude\projects'
$proj = Get-ChildItem $projRoot -Directory -ErrorAction SilentlyContinue |
        Where-Object { $_.Name -like '*-Desktop-claude-tfm2' } | Select-Object -First 1
if ($proj) { $mem = Join-Path $proj.FullName 'memory' }
$anaRoot = Join-Path $env:USERPROFILE 'Desktop\claude\tfm2\팀파매2모드 분석'
try { [Console]::InputEncoding  = [System.Text.Encoding]::UTF8 } catch {}
try { [Console]::OutputEncoding = [System.Text.Encoding]::UTF8 } catch {}
$raw = [Console]::In.ReadToEnd()
if ([string]::IsNullOrEmpty($raw)) { exit 0 }

# 1) 프롬프트에 등장한 심볼(FUN_xxx / 0x주소)이 DONE.md에 이미 판정돼 있으면 대상 지목 경고 (동적·고신호)
$doneFile = (Join-Path $mem 'DONE.md')
if (Test-Path $doneFile) {
  $syms = [regex]::Matches($raw, 'FUN_[0-9a-fA-F]{6,}|0x[0-9a-fA-F]{5,}') | ForEach-Object { $_.Value } | Select-Object -Unique -First 8
  if ($syms) {
    $doneTxt = Get-Content $doneFile -Raw -Encoding utf8
    $hits = @($syms | Where-Object { $doneTxt -match [regex]::Escape($_) })
    if ($hits.Count -gt 0) {
      Write-Output ("[!] 프롬프트의 심볼 " + ($hits -join ', ') + " 은(는) DONE.md에 기존 판정 있음 - 착수 전 prior-work로 확인(재작업 방지).")
    }
  }
}

# 2) 착수성 동사에만 §7/§9 리마인더 (조회성 일반어는 제외 - 배너 블라인드 방지)
$kw = '구현|재현|재구현|후킹|훅\s*걸|디컴|디스어셈|규명|리버스|마이그|패치\s*대응|detour|reimpl|decompile|disasm'
if ($raw -match $kw) {
  Write-Output "[§7/§9] 착수 전: 이미 됐나=prior-work / 구조·주소=game-atlas 에이전트 먼저. 지식베이스(INDEX/메모리/tracker/정본) 직접 grep 재유도·직접 Edit 금지(기록=record-keeper). DONE/DIFF=0/재시도금지면 재구현 말고 결론 사용. 현행 버전=MEM\CURRENT.md."
}
exit 0
