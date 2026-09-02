# -*- coding: utf-8 -*-
"""수동 재핀 적용 — 주석·문자열을 제외한 **코드 위치의 리터럴만** 치환(길이보존 마스킹).
   repin.py cmd_apply 와 같은 규칙. 개별 판정으로 확정한 값에 쓴다."""
import sys, os
sys.path.insert(0, r'C:\tfm2mods\MIG')
from repin import mask_code, HEXP
ROOT = r'C:\tfm2mods'

def safe_sub(rel, mapping, note=None):
    path = os.path.join(ROOT, rel)
    raw = open(path, 'rb').read().decode('utf-8', 'replace')
    masked = mask_code(raw)
    edits = []
    for m in HEXP.finditer(masked):
        v = int(m.group(1), 16)
        if v in mapping:
            edits.append((m.start(), m.end(), '0x%x' % mapping[v]))
    buf = raw
    for s0, e0, new in reversed(edits):
        buf = buf[:s0] + new + buf[e0:]
    open(path, 'w', encoding='utf-8', newline='').write(buf)
    print('%-52s 코드위치 치환 %d곳 %s' % (rel, len(edits), note or ''))
    return len(edits)
