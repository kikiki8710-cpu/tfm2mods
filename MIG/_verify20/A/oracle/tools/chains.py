# -*- coding: utf-8 -*-
"""!dbg id 들의 inlinedAt 사슬을 한 줄로. 사용: python chains.py m10.ll 34230 34234 ..."""
import sys, os, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import irlib

def fmt(f, ids):
    lines, md = irlib.load(f)
    for i in ids:
        ch = irlib.chain(md, i)
        parts = []
        for (l, fn, fb) in ch:
            base = re.split(r'[\\/]', fb)[-1]
            parts.append('%s:%d[%s]' % (base, l, re.sub(r'<.*', '', fn)[:30]))
        print('!%d  %s' % (i, ' <- '.join(parts)))

if __name__ == '__main__':
    fmt(sys.argv[1], [int(x.lstrip('!')) for x in sys.argv[2:]])
