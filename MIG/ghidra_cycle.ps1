# ghidra_cycle.ps1 — Ghidra 저장·종료·플러그인 교체·재시작·프로그램 열기를 한 번에
#
#   powershell -File MIG\ghidra_cycle.ps1 [-Jar <새 jar 경로>] [-Program 0.5.8]
#
# ★왜 스크립트인가: MCP 플러그인을 고칠 때마다 이 사이클을 돈다. 손으로 하면
#   매번 같은 함정을 밟는다 —
#     · jar 는 Ghidra 가 실행 중이면 **잠겨서 교체되지 않는다**(조용히 구 버전이 계속 로드됨)
#     · 종료할 때 "Save Program?" 다이얼로그가 뜨는데, 놓치면 프로세스가 안 죽는다
#     · ★**프론트엔드만 띄우면 8081 이 안 열린다** — MCP 플러그인은 CodeBrowser 도구에
#       붙으므로 **프로그램을 실제로 열어야** 서버가 뜬다
#
# 종료 코드: 0 = 8081 개방까지 성공 / 1 = 실패

param(
    [string]$Jar = "",
    [string]$Program = "0.5.8",
    [string]$GhidraRun = "C:\Users\jungs\Desktop\claude\ghidra_12.1.2_PUBLIC_20260605\ghidra_12.1.2_PUBLIC\ghidraRun.bat",
    [string]$JarDest = "C:\Users\jungs\AppData\Roaming\ghidra\ghidra_12.1.2_PUBLIC\Extensions\GhidraMCP\lib\GhidraMCP.jar",
    [int]$Port = 8081
)

Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
Add-Type @'
using System; using System.Text; using System.Runtime.InteropServices; using System.Collections.Generic;
public class GC2 {
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L,T,R,B; }
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] static extern bool EnumWindows(EnumProc cb, IntPtr p);
  [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport("user32.dll", CharSet=CharSet.Unicode, EntryPoint="GetWindowTextW")] static extern int GT(IntPtr h, StringBuilder s, int n);
  [DllImport("user32.dll", CharSet=CharSet.Unicode, EntryPoint="GetClassNameW")] static extern int GCl(IntPtr h, StringBuilder s, int n);
  [DllImport("user32.dll")] static extern bool IsWindowVisible(IntPtr h);
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int c);
  [DllImport("user32.dll")] public static extern IntPtr SendMessageTimeout(IntPtr h, uint m, IntPtr w, IntPtr l, uint f, uint t, out IntPtr r);
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern void mouse_event(uint f, uint dx, uint dy, uint d, IntPtr e);
  delegate bool EnumProc(IntPtr h, IntPtr p);
  public static List<string> Wins(){ var r=new List<string>();
    EnumWindows((h,p)=>{ uint pid; GetWindowThreadProcessId(h,out pid);
      if(IsWindowVisible(h)){ var c=new StringBuilder(128); GCl(h,c,128); var t=new StringBuilder(512); GT(h,t,512);
        if(c.ToString().StartsWith("SunAwt")) r.Add(h.ToString("X")+"|"+pid+"|"+c.ToString()+"|"+t.ToString()); }
      return true; }, IntPtr.Zero);
    return r; }
  public static void Click(int x,int y,bool dbl){ SetCursorPos(x,y); System.Threading.Thread.Sleep(200);
    mouse_event(0x02,0,0,0,IntPtr.Zero); mouse_event(0x04,0,0,0,IntPtr.Zero);
    if(dbl){ System.Threading.Thread.Sleep(80); mouse_event(0x02,0,0,0,IntPtr.Zero); mouse_event(0x04,0,0,0,IntPtr.Zero); } }
}
'@

function Wait-Port([int]$p, [int]$sec) {
    for ($i = 0; $i -lt [int]($sec / 5); $i++) {
        Start-Sleep -Seconds 5
        if (Get-NetTCPConnection -State Listen -LocalPort $p -ErrorAction SilentlyContinue) { return $true }
    }
    return $false
}

# ── 1) 저장 ────────────────────────────────────────────────────────────
if (Get-NetTCPConnection -State Listen -LocalPort $Port -ErrorAction SilentlyContinue) {
    try {
        $r = Invoke-WebRequest -Uri "http://127.0.0.1:$Port/save_program" -TimeoutSec 900 -UseBasicParsing
        Write-Output "1) 저장: $($r.Content)"
    } catch { Write-Output "1) 저장 스킵(구 플러그인이면 엔드포인트 없음): $($_.Exception.Message)" }
}

# ── 2) 종료 (다이얼로그는 Enter=기본버튼 Save 로 처리) ──────────────────
$procs = @(Get-Process javaw -ErrorAction SilentlyContinue)
foreach ($p in $procs) {
    $null = $p.CloseMainWindow()
    Start-Sleep -Seconds 5
    foreach ($w in [GC2]::Wins()) {
        $f = $w -split '\|'
        if ($f[1] -eq $p.Id -and $f[2] -eq 'SunAwtDialog') {
            $h = [IntPtr][Convert]::ToInt64($f[0], 16)
            [void][GC2]::SetForegroundWindow($h); Start-Sleep -Milliseconds 700
            [System.Windows.Forms.SendKeys]::SendWait("{ENTER}")
            Write-Output "   다이얼로그 처리: $($f[3])"
        }
    }
    $p.WaitForExit(180000) | Out-Null
}
# 남은 최상위 창(프론트엔드)에 WM_CLOSE
foreach ($w in [GC2]::Wins()) {
    $f = $w -split '\|'
    if ($f[2] -eq 'SunAwtFrame') {
        $rr = [IntPtr]::Zero
        [void][GC2]::SendMessageTimeout([IntPtr][Convert]::ToInt64($f[0], 16), 0x0010, [IntPtr]::Zero, [IntPtr]::Zero, 0, 20000, [ref]$rr)
    }
}
Start-Sleep -Seconds 5
$left = @(Get-Process javaw -ErrorAction SilentlyContinue)
if ($left.Count -gt 0) { $left | ForEach-Object { $_.WaitForExit(60000) | Out-Null } }
if (@(Get-Process javaw -ErrorAction SilentlyContinue).Count -gt 0) {
    Write-Output "2) ✗ Ghidra 가 안 죽음 — 화면 확인 필요"; exit 1
}
Write-Output "2) Ghidra 종료 완료"

# ── 3) jar 교체 ────────────────────────────────────────────────────────
if ($Jar -ne "") {
    if (-not (Test-Path $Jar)) { Write-Output "3) ✗ jar 없음: $Jar"; exit 1 }
    Copy-Item $Jar $JarDest -Force
    Write-Output "3) jar 교체: $((Get-Item $JarDest).Length) bytes"
} else { Write-Output "3) jar 교체 생략" }

# ── 4) 기동 ────────────────────────────────────────────────────────────
Start-Process -FilePath $GhidraRun -WindowStyle Minimized
Write-Output "4) ghidraRun 실행"
$fe = $null
for ($i = 0; $i -lt 40; $i++) {
    Start-Sleep -Seconds 3
    $fe = [GC2]::Wins() | Where-Object { $_ -match 'SunAwtFrame\|Ghidra: ' } | Select-Object -First 1
    if ($fe) { break }
}
if (-not $fe) { Write-Output "4) ✗ 프론트엔드가 안 뜸"; exit 1 }

# ── 5) 프로그램 열기 (★이걸 해야 8081 이 열린다) ───────────────────────
$h = [IntPtr][Convert]::ToInt64((($fe -split '\|')[0]), 16)
[void][GC2]::ShowWindow($h, 9)
[void][GC2]::SetForegroundWindow($h)
Start-Sleep -Seconds 2
$r = New-Object GC2+RECT
[void][GC2]::GetWindowRect($h, [ref]$r)
# Filter 입력란 = 창 좌상단 기준 (400, 412) · 첫 결과 항목 = (170, 183)
[GC2]::Click($r.L + 400, $r.T + 412, $false)
Start-Sleep -Milliseconds 400
[System.Windows.Forms.SendKeys]::SendWait($Program)
Start-Sleep -Seconds 1
[GC2]::Click($r.L + 170, $r.T + 183, $true)
Write-Output "5) '$Program' 더블클릭 — CodeBrowser 대기"

if (Wait-Port $Port 360) {
    Write-Output "★완료 — $Port 개방"
    exit 0
} else {
    # 실패하면 화면을 남겨 사람이 볼 수 있게 한다
    $shot = "$env:TEMP\ghidra_cycle_fail.png"
    $bmp = New-Object System.Drawing.Bitmap(($r.R - $r.L), ($r.B - $r.T))
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.CopyFromScreen($r.L, $r.T, 0, 0, (New-Object System.Drawing.Size(($r.R - $r.L), ($r.B - $r.T))))
    $bmp.Save($shot, [System.Drawing.Imaging.ImageFormat]::Png)
    $g.Dispose(); $bmp.Dispose()
    Write-Output "✗ $Port 미개방 — 화면: $shot"
    exit 1
}
