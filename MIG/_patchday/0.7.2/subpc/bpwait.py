# -*- coding: utf-8 -*-
"""밴픽 화면에서 우리(레드) 차례가 올 때까지 기다린다 — 위쪽 막대 색으로 판별(빨강 우세 = 레드 차례).
python bpwait.py <저장 png> [최대초=120]   → 레드 차례면 종료코드 0, 시간 초과 1, 밴픽 화면이 아니면(막대 어두움) 2
"""
import subprocess, sys, time
from PIL import Image
OUT = sys.argv[1]
LIMIT = float(sys.argv[2]) if len(sys.argv) > 2 else 120
SHOT = [sys.executable, '-X', 'utf8', r'F:\claude\sub_pc\tools\vnc_ctl.py', 'shot', OUT, '--scale', '0.5']
t0 = time.time()
last = None
while time.time() - t0 < LIMIT:
    subprocess.run(SHOT, capture_output=True)
    im = Image.open(OUT).convert('RGB').crop((330, 2, 760, 20))
    px = list(im.getdata()) if not hasattr(im, 'get_flattened_data') else list(im.get_flattened_data())
    r = sum(p[0] for p in px) / len(px); g = sum(p[1] for p in px) / len(px); b = sum(p[2] for p in px) / len(px)
    last = (round(r), round(g), round(b))
    if r > 110 and r > b * 1.6:
        print('RED', last, round(time.time() - t0)); sys.exit(0)
    time.sleep(2)
print('TIMEOUT', last); sys.exit(1)
