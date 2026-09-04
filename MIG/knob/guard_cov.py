# -*- coding: utf-8 -*-
"""가드 커버리지 실측 — 살아있는 사이트가 실제로 `orig_guard_ok` 를 통과할 수 있는가.

가드는 fail-CLOSED 라 표에 (rva, off, width) 가 **정확히 일치**해야 통과한다.
"바이트가 맞다"와 "패치가 나간다"는 다른 얘기이고, 이걸 안 보면 무증상으로 전부 죽는다.
"""
import io, re, sys, struct
sys.path.insert(0, r'C:\tfm2mods\MIG')
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
import pefile, mig_verify as MV
pe=pefile.PE(MV.GAME_EXE, fast_load=True)
SEC=[(s.VirtualAddress,s.VirtualAddress+s.Misc_VirtualSize,s.get_data()) for s in pe.sections]
def rd(r,n):
    for a,b,d in SEC:
        if a<=r<b: return d[r-a:r-a+n]
exec(open('_audit_parse.py', encoding='utf-8').read())   # rows / sites

T=r'C:\tfm2mods\tfm2_ai_adjust\src\orig_table.rs'
ROW=re.compile(r'^\s*\((0x[0-9a-fA-F]+),\s*(\d+),\s*(\d+),\s*(\d+)\),', re.M)
tab={}
for m in ROW.finditer(io.open(T,encoding='utf-8').read()):
    tab.setdefault((int(m.group(1),16),int(m.group(2)),int(m.group(3))), int(m.group(4)))
print("표 키 %d개" % len(tab))

from collections import defaultdict, Counter
variants=defaultdict(list)
for r in rows: variants[(r['fn'],r['rva'])].append(r)
live=[]
for k,grp in variants.items():
    for r in grp:
        b=rd(r['rva'], r['off']+r['w']+4)
        if b and any(list(b[:len(p)])==p for p in r['pres']):
            live.append(r); break
print("살아있는 사이트 %d개" % len(live))

cov, nocov, mism = 0, [], []
for r in live:
    k=(r['rva'], r['off'], r['w'])
    if k in tab:
        b=rd(r['rva'], r['off']+r['w']+8)
        cur=(b[r['off']] if r['w']==1 else
             struct.unpack('<H',b[r['off']:r['off']+2])[0] if r['w']==2 else
             struct.unpack('<I',b[r['off']:r['off']+4])[0] if r['w']==4 else
             struct.unpack('<Q',b[r['off']:r['off']+8])[0])
        if cur==tab[k]: cov+=1
        else: mism.append((r,cur,tab[k]))
    else:
        nocov.append(r)
print("\n★가드 통과 가능 : %d / %d  (%.1f%%)" % (cov, len(live), 100.0*cov/len(live)))
print("표에 없어 **거부됨** : %d" % len(nocov))
print("표에 있으나 expect 불일치(=blocked) : %d" % len(mism))
c=Counter((r['fn'],r['line']) for r in nocov)
for (f,l),n in c.most_common(14):
    ex=next(r for r in nocov if r['fn']==f and r['line']==l)
    print("   미등록 %-14s:%-5d %2d사이트  val=%s" % (f,l,n,ex['val']))
print("   … 미등록 총 %d 소스행" % len(c))
for r,cur,exp in mism[:8]:
    print("   불일치 %-14s:%-5d 0x%-8x exe=%s expect=%s" % (r['fn'],r['line'],r['rva'],cur,exp))
