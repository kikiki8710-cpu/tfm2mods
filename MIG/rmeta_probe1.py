# -*- coding: utf-8 -*-
import io,os,re,sys
sys.stdout.reconfigure(encoding='utf-8',errors='replace')
SDK=r'C:\tfm2mods\sdk_058\mod-sdk\deps'
def rmeta_path(c):
    for x in os.listdir(SDK):
        if x.startswith('lib'+c+'-') and x.endswith('.rmeta'): return os.path.join(SDK,x)
def rmeta_from_rlib(c):
    p=[x for x in os.listdir(SDK) if x.startswith('lib'+c+'-') and x.endswith('.rlib')][0]
    f=io.open(os.path.join(SDK,p),'rb'); assert f.read(8)==b'!<arch>\n'
    while True:
        h=f.read(60)
        if len(h)<60: return None
        name=h[0:16].decode('ascii','replace').strip().rstrip('/')
        size=int(h[48:58].decode('ascii').strip()); off=f.tell()
        if name=='lib.rmeta':
            f.seek(off); return f.read(size)
        f.seek(off+size+(size&1))
crate=sys.argv[1]
src=sys.argv[2] if len(sys.argv)>2 else 'rmeta'
d=open(rmeta_path(crate),'rb').read() if src=='rmeta' else rmeta_from_rlib(crate)
print('len=%d root=0x%x'%(len(d),int.from_bytes(d[8:16],'little')))
pats=[b'hunt_and_battle.rs',b'sub_plan.rs',b'defense_nexus.rs',b'rule_scope.rs',b'entity.rs',b'chat.rs',b'.rs']
for p in pats[:-1]:
    idxs=[m.start() for m in re.finditer(re.escape(p),d)]
    print('%-22s %d hits: %s'%(p.decode(),len(idxs),[hex(i) for i in idxs[:12]]))
n=len(re.findall(rb'\.rs',d)); print('total ".rs" occurrences:',n)
