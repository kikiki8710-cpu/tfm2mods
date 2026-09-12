# -*- coding: utf-8 -*-
# TFM2 클릭 함수 — 인라인 dot-source 용 (`. C:\tfm2mods\MIG\_clickfn.ps1; Click-TFM 1082 328`).
# `_click.ps1` 과 같은 절차(TextInputHost kill → 강제 FG → 좌표 환산 1456x819 → hover 0.8s → mouse_event)인데
# `powershell -File` 이 하네스에서 백그라운드화되는 함정을 피하려고 함수로 만들었다(03_시행착오 §16 교훈 4 · 2026-09-13).
function Click-TFM {
  param([int]$X, [int]$Y, [double]$Hover = 0.8, [switch]$NoClick)
  Get-Process TextInputHost -EA SilentlyContinue | Stop-Process -Force -EA SilentlyContinue
  $cls = "WClk" + ([guid]::NewGuid().ToString("N").Substring(0, 8))
  Add-Type @"
using System; using System.Runtime.InteropServices;
public class $cls {
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x,int y);
  [DllImport("user32.dll")] public static extern void mouse_event(uint f,uint dx,uint dy,uint d,IntPtr e);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool BringWindowToTop(IntPtr h);
  [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h,int n);
  [DllImport("user32.dll")] public static extern bool SetWindowPos(IntPtr h,IntPtr a,int x,int y,int cx,int cy,uint f);
  [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] public static extern bool SystemParametersInfo(uint a,uint b,IntPtr c,uint d);
  [DllImport("user32.dll")] public static extern bool GetClientRect(IntPtr h,out RECT r);
  [DllImport("user32.dll")] public static extern void keybd_event(byte k,byte s,uint f,UIntPtr e);
  public struct RECT{public int l,t,r,b;} }
"@
  $p = Get-Process -Name TeamfightManager2 -EA SilentlyContinue
  if (-not $p) { Write-Output "ERR: game not running"; return }
  $h = $p.MainWindowHandle; $T = [type]$cls
  [void]$T::SystemParametersInfo(0x2001, 0, [IntPtr]::Zero, 0)
  [void]$T::SetWindowPos($h, [IntPtr](-1), 0, 0, 0, 0, 3); [void]$T::SetWindowPos($h, [IntPtr](-2), 0, 0, 0, 0, 3)
  [void]$T::ShowWindow($h, 3); [void]$T::BringWindowToTop($h); [void]$T::SetForegroundWindow($h)
  Start-Sleep -Milliseconds 400
  $fg = $T::GetForegroundWindow(); if ($fg -ne $h) { Write-Output "WARN: foreground not acquired fg=$fg game=$h" }
  $r = New-Object "$cls+RECT"; [void]$T::GetClientRect($h, [ref]$r)
  $cw = $r.r - $r.l; $ch = $r.b - $r.t; if ($cw -lt 100) { $cw = 1456; $ch = 819 }
  $sx = [int][math]::Round($X * $cw / 1456.0); $sy = [int][math]::Round($Y * $ch / 819.0)
  Write-Output "client=${cw}x${ch} img($X,$Y) -> screen($sx,$sy)"
  if ($NoClick) { return }
  [void]$T::SetCursorPos($sx, $sy); Start-Sleep -Milliseconds ([int]($Hover * 1000))
  $T::mouse_event(2, 0, 0, 0, [IntPtr]::Zero); Start-Sleep -Milliseconds 85; $T::mouse_event(4, 0, 0, 0, [IntPtr]::Zero)
  Write-Output "OK: clicked ($sx,$sy)"
}
# 키 연타 (예: Press-TFM 0x45 20 = 'e' 20회). 게임 포그라운드는 Click-TFM -NoClick 으로 먼저 확보.
function Press-TFM {
  param([int]$VK, [int]$Times = 1, [int]$GapMs = 120)
  $cls = "WKey" + ([guid]::NewGuid().ToString("N").Substring(0, 8))
  Add-Type @"
using System; using System.Runtime.InteropServices;
public class $cls { [DllImport("user32.dll")] public static extern void keybd_event(byte k,byte s,uint f,UIntPtr e); }
"@
  $T = [type]$cls
  for ($i = 0; $i -lt $Times; $i++) { $T::keybd_event([byte]$VK, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 40; $T::keybd_event([byte]$VK, 0, 2, [UIntPtr]::Zero); Start-Sleep -Milliseconds $GapMs }
  Write-Output "OK: key 0x$('{0:X}' -f $VK) x$Times"
}
