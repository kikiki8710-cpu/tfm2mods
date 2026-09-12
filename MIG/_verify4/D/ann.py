import io,re,sys
f=sys.argv[1]; a=int(sys.argv[2]); b=int(sys.argv[3])
p=f
import os
for base in (r'C:\tfm2mods\_gaibc',r'C:\tfm2mods\_gcbc',r'C:\tfm2mods\_gvbc'):
    if os.path.exists(os.path.join(base,f)): p=os.path.join(base,f); break
lines=io.open(p,encoding='utf-8',errors='replace').readlines()
md={}
for ln in lines:
    if ln.startswith('!'):
        m=re.match(r'^!(\d+) = (.*)$',ln.rstrip('\n'))
        if m: md[int(m.group(1))]=m.group(2)
files={}
for k,v in md.items():
    m=re.match(r'!DIFile\(filename: "([^"]*)"',v)
    if m: files[k]=os.path.basename(m.group(1))
def sp(i,seen=None):
    seen=seen or set()
    while i is not None and i not in seen:
        seen.add(i); s=md.get(i,'')
        m=re.search(r'DISubprogram\(name: "([^"]+)"',s)
        if m:
            fl=re.search(r'file: !(\d+)',s)
            return m.group(1), files.get(int(fl.group(1)),'?') if fl else '?'
        m=re.search(r'scope: !(\d+)',s)
        i=int(m.group(1)) if m else None
    return ('?','?')
def loc(i):
    out=[]; cur=i
    while True:
        s=md.get(cur,'')
        m=re.match(r'!DILocation\(line: (\d+),(?: column: (\d+),)? scope: !(\d+)(?:, inlinedAt: !(\d+))?\)',s)
        if not m: break
        n,fl=sp(int(m.group(3)))
        out.append(f"{n}@{fl}:{m.group(1)}")
        if not m.group(4): break
        cur=int(m.group(4))
    return (out[0]+" .. "+out[-1]) if len(out)>1 else (out[0] if out else "")
# local var names
vn={}
for k,v in md.items():
    m=re.match(r'!DILocalVariable\(name: "([^"]+)"',v)
    if m: vn[k]=m.group(1)
for k in range(a-1,b):
    t=lines[k].rstrip('\n')
    s=t.strip()
    if '#dbg_' in t:
        mm=re.search(r'#dbg_(\w+)\(([^,]+), !(\d+)',t)
        if mm and int(mm.group(3)) in vn:
            print(f"{k+1}:   ; dbg {vn[int(mm.group(3))]} = {mm.group(2)}")
        continue
    m=re.search(r'!dbg !(\d+)',t)
    tag=('   ['+loc(int(m.group(1)))+']') if m else ''
    print(f"{k+1}: {s[:190]}{tag}")
