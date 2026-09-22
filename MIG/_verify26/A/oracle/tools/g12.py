# -*- coding: utf-8 -*-
"""26A G12: consts[20] (값 20, src_line 1782) — 주석본 d8eff0.ll 에서 `, 20` 리터럴 사용 줄과 ;L 사슬을 전수."""
import io, re, sys
sys.stdout.reconfigure(encoding='utf-8')
P = r'C:\tfm2mods\MIG\_next\reach\d8eff0.ll'
lines = io.open(P, encoding='utf-8').read().split('\n')
pat = re.compile(r'\bmul\b.*\b20\b|\b20\b.*\bmul\b|(?:i64|i32) 20\b|, 20(?:$|[,\s\)])')
for l in lines:
    m = re.match(r'\s*(\d+)\|(.*)', l)
    if not m: continue
    ln, body = m.group(1), m.group(2)
    # 리터럴 20 만(레지스터 %20, 메타 !20, align 20, gep offset 아님)
    if re.search(r'(?<![%!\w.])20(?![\w.])', body) and ('mul' in body or 'add' in body or 'icmp' in body or 'select' in body or 'phi' in body):
        print(ln, body.strip()[:220])
