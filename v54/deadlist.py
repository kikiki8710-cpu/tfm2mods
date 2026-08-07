# -*- coding: utf-8 -*-
"""남은 미확정 사이트 전수 목록 (sweep.py 로직 재사용, 절삭 없음 + 함수/소스 귀속).
용도: 이번 정밀 재조사의 작업 대상 목록 생성.  python deadlist.py [--tsv out.tsv]
"""
import io, os, re, sys, collections
sys.path.insert(0, r'C:\tfm2mods\v54')
import reloc as R
import sweep as SW
# ⚠sweep import 시 이미 stdout 을 감싼다. 재래핑 금지(closed file).
B = 0x140000000
E3, E4 = R.E3, R.E4
SRCDIR = SW.SRCDIR

def encl_fn_name(path, line):
    """사이트가 들어있는 rust fn 이름."""
    lines = io.open(path, encoding='utf-8').read().split('\n')
    for i in range(min(line, len(lines)) - 1, -1, -1):
        m = re.match(r'\s*(pub\s+)?(unsafe\s+)?fn\s+(\w+)', lines[i])
        if m: return m.group(3)
    return '?'

def ctxline(path, line):
    lines = io.open(path, encoding='utf-8').read().split('\n')
    return lines[line-1].strip()[:150] if line-1 < len(lines) else ''

rows=[]
for fn in ['detour.rs','disc19_repro.rs']:
    cur=os.path.join(SRCDIR,fn); bak=cur+'.053bak'
    base=bak if os.path.exists(bak) else cur
    orig=set(a for _,a in SW.candidates(base))
    seen=set()
    for line,a in SW.candidates(cur):
        if a not in orig: continue
        if (fn,line,a) in seen: continue
        seen.add((fn,line,a))
        f3=SW.text_ok(E3,a)
        if not f3:
            f=E3.func_of(a)
            rows.append((fn,line,a,'053명령아님','fn=%s'%('%06x'%f[0] if f else '-'),encl_fn_name(cur,line),ctxline(cur,line)))
            continue
        pr=R.pair_fn(f3[0],f3[1])
        if not pr:
            rows.append((fn,line,a,'짝없음','fn=%06x-%06x src=%s'%(f3[0],f3[1],R.SRC3.get(f3[0],'-')),encl_fn_name(cur,line),ctxline(cur,line)))
            continue
        bs,be,ratio=pr
        i3={i.address-B:i for i in R.insns(E3,f3[0],f3[1])}
        ins=i3.get(a); i4=R.insns(E4,bs,be)
        ex3=[x for x,y in sorted(i3.items()) if y.bytes==ins.bytes]
        ex4=[y.address-B for y in i4 if y.bytes==ins.bytes]
        if ex4 and len(ex3)==len(ex4) and a in ex3:
            continue  # 확정된 것
        k3=[x for x,y in sorted(i3.items()) if y.mnemonic==ins.mnemonic and len(y.bytes)==len(ins.bytes)]
        k4=[y.address-B for y in i4 if y.mnemonic==ins.mnemonic and len(y.bytes)==len(ins.bytes)]
        if k4 and len(k3)==len(k4) and a in k3:
            continue
        rows.append((fn,line,a,'미확정','fn3=%06x-%06x fn4=%06x-%06x %.0f%% sig %d->%d'%(f3[0],f3[1],bs,be,ratio*100,len(ex3),len(ex4)),encl_fn_name(cur,line),ctxline(cur,line)))

byfn=collections.Counter(r[5] for r in rows)
print('총 %d개'%len(rows))
for k,v in byfn.most_common(): print('  %-28s %d'%(k,v))
print()
for r in sorted(rows,key=lambda x:(x[5],x[2])):
    print('%-22s %-10s %06x %-10s %s'%(r[5],r[0]+':'+str(r[1]),r[2],r[3],r[4]))
    print('        | %s'%r[6])
