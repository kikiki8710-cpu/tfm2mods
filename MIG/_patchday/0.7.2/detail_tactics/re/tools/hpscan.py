# usage: hpscan.py old|new IMM(hex) [needle_hex] [back]  -> sites of cmp rax,IMM with needle (default HP imul) within back bytes before
import sys
sys.path.insert(0, r'F:\claude\tfm2mods\MIG')
import aidiff as A
P = {'old': r'F:\claude\tfm2\tfm2_0.7.1\TeamfightManager2.exe', 'new': r'F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe'}
img = A.Img(P[sys.argv[1]]); imm = int(sys.argv[2], 16)
needle = bytes.fromhex(sys.argv[3]) if len(sys.argv) > 3 and sys.argv[3] != '-' else b'\x68\x06\x00\x00\x64'
back = int(sys.argv[4]) if len(sys.argv) > 4 else 260
pat = (b'\x48\x83\xf8' + bytes([imm])) if imm < 0x80 else (b'\x48\x3d' + imm.to_bytes(4, 'little'))
d = img.data
for nm, va, sz, pr in img.secs:
    if nm != '.text': continue
    blob = d[pr:pr+sz]
    i = blob.find(pat)
    while i >= 0:
        r = va + i
        w = blob[max(0, i-back):i]
        if needle in w:
            f = img.owner(r)
            print(f'{r:#x} owner {f[0]:#x}' if f else f'{r:#x}')
        i = blob.find(pat, i+1)
