# -*- coding: utf-8 -*-
"""화면 경기 헤더의 세르펜 처치 숫자(블루·레드)가 처음 바뀌면 「일시 정지」를 누른다.  python pause_on_kill.py [최대초=300]
좌표 = 원본 1920x1080. 블루 숫자 ≈ (637,24) · 레드 숫자 ≈ (1280,24) · 일시 정지 버튼 (1632,974)."""
import sys, time
sys.path.insert(0, r'F:\claude\sub_pc\tools')
import vnc_ctl
from PIL import ImageChops, ImageStat
LIMIT = float(sys.argv[1]) if len(sys.argv) > 1 else 300
BOXES = {'blue': (624, 10, 650, 38), 'red': (1267, 10, 1293, 38)}
def grab():
    v = vnc_ctl.RFB(); im = v.shot(); v.close(); return im
base = grab()
ref = {k: base.crop(b) for k, b in BOXES.items()}
t0 = time.time()
while time.time() - t0 < LIMIT:
    time.sleep(1.5)
    im = grab()
    for k, b in BOXES.items():
        d = sum(ImageStat.Stat(ImageChops.difference(ref[k], im.crop(b))).mean)
        if d > 12:
            v = vnc_ctl.RFB()
            for (x, y, m) in ((1632, 974, 0), (1632, 974, 1), (1632, 974, 0)):
                v.pointer(x, y, m); time.sleep(0.15)
            v.close()
            im.save(r'F:\claude\tfm2mods\MIG\_patchday\0.7.2\subpc\shots\s53_kill.png')
            print('CHANGED', k, round(d, 1), round(time.time() - t0)); sys.exit(0)
print('TIMEOUT'); sys.exit(1)
