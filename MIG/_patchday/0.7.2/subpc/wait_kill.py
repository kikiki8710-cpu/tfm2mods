# -*- coding: utf-8 -*-
"""serpen_probe 의 [화면카운터] 가 B+R>0 이 될 때까지 기다린다(화면 경기 세르펜 첫 처치).  python wait_kill.py [최대초=240]"""
import re, subprocess, sys, time
SSH = ['ssh', '-i', r'C:\Users\jungs\.ssh\subpc_ed25519', '-o', 'BatchMode=yes', 'owner@192.168.11.59',
       r'powershell -NoProfile -ExecutionPolicy Bypass -File C:\tfm2_stage\072\tail072.ps1 -Tail 120']
LIMIT = float(sys.argv[1]) if len(sys.argv) > 1 else 240
t0 = time.time()
while time.time() - t0 < LIMIT:
    out = subprocess.run(SSH, capture_output=True).stdout.decode('utf-8', 'replace')
    m = re.search(r'\[화면카운터\] 세르펜 화면처치수 B=(\d+) R=(\d+)', out)
    if m and int(m.group(1)) + int(m.group(2)) > 0:
        print('KILL', m.group(0), round(time.time() - t0)); sys.exit(0)
    time.sleep(3)
print('TIMEOUT'); sys.exit(1)
