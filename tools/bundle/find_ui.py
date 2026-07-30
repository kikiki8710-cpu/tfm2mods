import sys, os, re
path = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\bundle.game_data"
size = os.path.getsize(path)
needles = [b'asset/base/ui/layout/ingame', b'#game_time', b'draggable_popup', b'game_time:color']
hits = {n: [] for n in needles}
CH = 8*1024*1024
carry = b''
base = 0
with open(path,'rb') as f:
    while True:
        buf = f.read(CH)
        if not buf: break
        data = carry + buf
        for n in needles:
            st = 0
            while True:
                i = data.find(n, st)
                if i < 0: break
                off = base - len(carry) + i
                if len(hits[n]) < 8: hits[n].append(off)
                st = i + 1
        carry = data[-256:]
        base += len(buf)
for n, v in hits.items():
    print(f"{n.decode()!r:40} hits(first8)={v}")
