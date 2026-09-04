# -*- coding: utf-8 -*-
"""표 채우기 마지막 패스 — **살아있는데 표에 없는 사이트**를 추가한다.

배열을 통째로 갈아 끼운 재핀(pcol/coll·larn/lard·entity 2벌 등)은 git diff 로 구→신 쌍이
안 뽑혀 이관에서 빠졌다. 가드가 fail-CLOSED 라 **표에 없으면 패치가 아예 안 나가므로**
이 잔여를 채워야 이번 세션 작업이 실제로 동작한다.

안전장치
  · 대상은 **prefix 가 실제로 맞는(=살아있는) 사이트**로 한정한다.
  · 소스가 ORIG 를 명시했으면 **exe 실측과 일치할 때만** 추가(교차검증). 어긋나면 추가하지 않는다.
  · 같은 rva 에 (off,w) 가 다른 **낡은 배선 행**이 있으면 그 행은 제거한다(이분탐색이 그걸 먼저 잡아
    거부해 버리기 때문 — 09-02 사고와 같은 계열의 함정).
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
def imm_at(rva,off,w):
    b=rd(rva,off+w+8)
    if b is None or len(b)<off+w: return None
    return (b[off] if w==1 else struct.unpack('<H',b[off:off+2])[0] if w==2
            else struct.unpack('<I',b[off:off+4])[0] if w==4
            else struct.unpack('<Q',b[off:off+8])[0] if w==8 else None)

exec(open('_audit_parse.py', encoding='utf-8').read())
from collections import defaultdict
variants=defaultdict(list)
for r in rows: variants[(r['fn'],r['rva'])].append(r)
ORIGRE=re.compile(r"^[A-Za-z_]\w*\(\s*\w+\s*,\s*(0x[0-9a-fA-F_]+|\d[\d_]*)\s*[,)]")
live=[]
for k,grp in variants.items():
    for r in grp:
        b=rd(r['rva'], r['off']+r['w']+4)
        if b and any(list(b[:len(p)])==p for p in r['pres']):
            m=ORIGRE.match(r['val'].strip())
            r['sorig']=int(m.group(1).replace('_',''),0) if m else None
            live.append(r); break

P=r'C:\tfm2mods\tfm2_ai_adjust\src\orig_table.rs'
src=io.open(P,encoding='utf-8').read()
ROW=re.compile(r'^(\s*)\((0x[0-9a-fA-F]+),\s*(\d+),\s*(\d+),\s*(\d+)\),(.*)$', re.M)
ms=list(ROW.finditer(src))
tab=[dict(rva=int(m.group(2),16),off=int(m.group(3)),w=int(m.group(4)),
          orig=int(m.group(5)),tail=m.group(6).rstrip()) for m in ms]
head,tail_txt=src[:ms[0].start()],src[ms[-1].end():]
keys={(r['rva'],r['off'],r['w']) for r in tab}

add, skip, drop = [], [], set()
for r in live:
    k=(r['rva'],r['off'],r['w'])
    if k in keys: continue
    cur=imm_at(r['rva'],r['off'],r['w'])
    if cur is None:
        skip.append((r,'섹션 밖')); continue
    if r['sorig'] is not None and cur!=r['sorig']:
        skip.append((r,'exe %s != 소스ORIG %s' % (cur,r['sorig']))); continue
    tagsrc = '소스ORIG 교차확인' if r['sorig'] is not None else '[exe유래]'
    add.append(dict(rva=r['rva'],off=r['off'],w=r['w'],orig=cur,
                    tail='   // ★0.5.8 채움(%s·%s:%d)' % (tagsrc, r['fn'], r['line'])))
    keys.add(k)
    for t in tab:
        if t['rva']==r['rva'] and (t['off'],t['w'])!=(r['off'],r['w']):
            drop.add(id(t))

print("추가 %d행 / 추가불가 %d / 낡은배선 제거 %d행" % (len(add), len(skip), len(drop)))
for r,w in skip[:8]:
    print("   불가 %-14s:%-5d 0x%x %s" % (r['fn'],r['line'],r['rva'],w))
tab=[t for t in tab if id(t) not in drop]+add
tab.sort(key=lambda r:(r['rva'],r['off'],r['w']))
body='\n'.join("    (0x%x, %d, %d, %d),%s"%(r['rva'],r['off'],r['w'],r['orig'],r['tail']) for r in tab)
io.open(P+'.bak_fill','w',encoding='utf-8',newline='').write(src)
io.open(P,'w',encoding='utf-8',newline='').write(head+body+tail_txt)
ok=all(tab[i]['rva']>=tab[i-1]['rva'] for i in range(1,len(tab)))
print("표 %d행 / 오름차순: %s" % (len(tab), 'OK' if ok else '★깨짐'))
