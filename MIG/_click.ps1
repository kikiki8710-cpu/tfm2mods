# -*- coding: utf-8 -*-
# TFM2 클릭 헬퍼 — 런북(ANA\runtime-replay-runbook.md §0-3 · MEM tfm2-replay-runbook) 정본 절차.
#
# 왜 이 방식인가(런북 실측):
#   · computer-use 직접 클릭은 "desktop shell is frontmost" 로 자주 실패한다
#     (스크린샷이 비허용 앱을 숨겼다 복원하며 포커스를 뺏는다).
#   · TFM2 UI 는 **hover → press 상태전이를 요구**한다 — 좌표를 바로 누르면
#     "Clicked" 가 돌아와도 게임은 무반응이다. ⟹ SetCursorPos 후 0.8s hover.
#   · 한국어 IME 호스트 `TextInputHost` 가 포그라운드를 잡으면 클릭이 전부 막힌다
#     (SetForegroundWindow/AttachThreadInput/topmost 전부 실패) ⟹ 매번 먼저 죽인다.
#   · ⚠Alt 키 금지(IME 를 깨워 TextInputHost 를 부른다).
#   · Add-Type 클래스 이름은 매번 유일해야 한다(쉘 상태가 유지되지 않는다). `GC` 는 금지
#     (`[GC]` 가 .NET System.GC 로 해석돼 전 메서드가 MethodNotFound 가 된다).
#
# 사용: powershell -File _click.ps1 -X <좌표x> -Y <좌표y> [-Hover 0.8] [-Shot]
#   좌표는 **1456x819 스크린샷 기준**. 실제 화면이 2560x1440 이면 자동 환산한다.
param([int]$X, [int]$Y, [double]$Hover = 0.8, [switch]$NoClick)

$cls = "WClk" + ([guid]::NewGuid().ToString("N").Substring(0, 8))
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class $cls {
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern void mouse_event(uint f, uint dx, uint dy, uint d, IntPtr e);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool BringWindowToTop(IntPtr h);
  [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int n);
  [DllImport("user32.dll")] public static extern bool SetWindowPos(IntPtr h, IntPtr a, int x, int y, int cx, int cy, uint f);
  [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] public static extern bool SystemParametersInfo(uint a, uint b, IntPtr c, uint d);
  [DllImport("user32.dll")] public static extern bool GetClientRect(IntPtr h, out RECT r);
  public struct RECT { public int l, t, r, b; }
}
"@

# ① TextInputHost 제거(포그라운드 트랩)
Get-Process TextInputHost -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue

$p = Get-Process -Name TeamfightManager2 -ErrorAction SilentlyContinue
if (-not $p) { Write-Output "ERR: 게임 미실행"; exit 1 }
$h = $p.MainWindowHandle

# ② 포그라운드 강제 (락 타임아웃 0 → topmost 토글 → show/bring/setfg)
[void]([type]$cls)::SystemParametersInfo(0x2001, 0, [IntPtr]::Zero, 0)   # SPI_SETFOREGROUNDLOCKTIMEOUT
[void]([type]$cls)::SetWindowPos($h, [IntPtr](-1), 0, 0, 0, 0, 0x0003)   # HWND_TOPMOST | NOSIZE|NOMOVE
[void]([type]$cls)::SetWindowPos($h, [IntPtr](-2), 0, 0, 0, 0, 0x0003)   # HWND_NOTOPMOST
[void]([type]$cls)::ShowWindow($h, 3)                                     # SW_MAXIMIZE
[void]([type]$cls)::BringWindowToTop($h)
[void]([type]$cls)::SetForegroundWindow($h)
Start-Sleep -Milliseconds 400

$fg = ([type]$cls)::GetForegroundWindow()
if ($fg -ne $h) { Write-Output "WARN: 포그라운드 미확보 (fg=$fg, game=$h)" }

# ③ 좌표 환산 — 런북 기준 스크린샷은 1456x819, 실제 클라이언트가 다르면 비례 환산
$r = New-Object "$cls+RECT"
[void]([type]$cls)::GetClientRect($h, [ref]$r)
$cw = $r.r - $r.l; $ch = $r.b - $r.t
if ($cw -lt 100) { $cw = 1456; $ch = 819 }
$sx = [int][math]::Round($X * $cw / 1456.0)
$sy = [int][math]::Round($Y * $ch / 819.0)
Write-Output "client=${cw}x${ch}  img(${X},${Y}) -> screen(${sx},${sy})"

if ($NoClick) { exit 0 }

# ④ hover → press (★hover 없이는 게임이 무반응)
[void]([type]$cls)::SetCursorPos($sx, $sy)
Start-Sleep -Milliseconds ([int]($Hover * 1000))
([type]$cls)::mouse_event(0x0002, 0, 0, 0, [IntPtr]::Zero)   # LEFTDOWN
Start-Sleep -Milliseconds 85
([type]$cls)::mouse_event(0x0004, 0, 0, 0, [IntPtr]::Zero)   # LEFTUP
Write-Output "OK: clicked ($sx,$sy)"
