# usage: xref.py old|new TARGET [TARGET...]  -> E8/E9/Jcc rel32 and short jumps referencing target (within same function for short)
import sys, struct
sys.path.insert(0, r'F:\claude\tfm2mods\MIG')
import aidiff as A
P = {'old': r'F:\claude\tfm2\tfm2_0.7.1\TeamfightManager2.exe', 'new': r'F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe'}
img = A.Img(P[sys.argv[1]])
tg = [int(x, 16) for x in sys.argv[2:]]
d = img.data
for nm, va, sz, pr in img.secs:
    if nm != '.text': continue
    blob = d[pr:pr+sz]
    import numpy as np
    arr = np.frombuffer(blob, dtype=np.uint8)
    for t in tg:
        hits = []
        for op in (0xe8, 0xe9):
            idx = np.nonzero(arr[:-5] == op)[0]
            rel = np.frombuffer(blob[1:] + b'\0'*4, dtype=np.uint8)
            for i in idx:
                r = struct.unpack_from('<i', blob, i+1)[0]
                if va + i + 5 + r == t:
                    hits.append((va+i, 'call' if op == 0xe8 else 'jmp'))
        print(f'target {t:#x}:', ', '.join(f'{h[0]:#x}({h[1]})' for h in hits))
