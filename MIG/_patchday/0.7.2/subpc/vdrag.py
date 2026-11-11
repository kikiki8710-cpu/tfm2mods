# -*- coding: utf-8 -*-
"""1호기 화면에서 끌어다 놓기(VNC 포인터 · 좌표 = 원본 1920x1080).  python vdrag.py x1 y1 x2 y2"""
import sys, time
sys.path.insert(0, r'F:\claude\sub_pc\tools')
import vnc_ctl
x1, y1, x2, y2 = map(int, sys.argv[1:5])
v = vnc_ctl.RFB()
v.pointer(x1, y1, 0); time.sleep(0.3)
v.pointer(x1, y1, 1); time.sleep(0.25)
N = 20
for i in range(1, N + 1):
    v.pointer(x1 + (x2 - x1) * i // N, y1 + (y2 - y1) * i // N, 1); time.sleep(0.03)
time.sleep(0.3)
v.pointer(x2, y2, 0); time.sleep(0.3)
v.close()
print('drag', x1, y1, '->', x2, y2)
