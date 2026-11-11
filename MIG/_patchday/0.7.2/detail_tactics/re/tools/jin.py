# usage: jin.py asmfile LO HI  -> list branch instructions whose target in [LO,HI)
import sys, re
lo = int(sys.argv[2], 16); hi = int(sys.argv[3], 16)
for line in open(sys.argv[1], encoding='utf-8'):
    m = re.search(r'\s(j\w+|call) 0x14([0-9a-f]+)', line)
    if m:
        t = int(m.group(2), 16)
        if lo <= t < hi: print(line.rstrip())
