import sys
raw=open(sys.argv[1],'rb').read()
start=int(sys.argv[2]); end=int(sys.argv[3])
seg=raw[start:end]
import struct
# hex dump with offsets
for i in range(0,len(seg),16):
    chunk=seg[i:i+16]
    off=start+i
    hexs=' '.join(f'{b:02x}' for b in chunk)
    asc=''.join(chr(b) if 32<=b<127 else '.' for b in chunk)
    print(f'{off}  {hexs:<48} {asc}')
