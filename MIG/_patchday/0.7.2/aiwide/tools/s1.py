import sys, pickle, collections, json
sys.path.insert(0, r'F:\claude\tfm2mods\MIG')
import aidiff as A
OLD=r'F:\claude\tfm2\tfm2_0.7.1\TeamfightManager2.exe'
NEW=r'F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe'
io_, fo = A.load(OLD,'game-ai')
inn, fn = A.load(NEW,'game-ai')
pairs, gone, added = A.pair(fo, fn)
rows=[]
for ko,(kn,d,n) in pairs.items():
    v,extra=A.verdict(fo[ko],fn[kn])
    rows.append((v,ko,kn,extra))
ed=[r for r in rows if r[0]=='EDITED']
print(len(ed), len(gone), len(added))
cgo=pickle.load(open(r'F:\claude\tfm2mods\_cg_071.pkl','rb'))
cgn=pickle.load(open(r'F:\claude\tfm2mods\_cg_072.pkl','rb'))
print(type(cgo), list(cgo.keys())[:3] if isinstance(cgo,dict) else None)
k=list(cgo['caller'].keys())[:2]; print(k, cgo['caller'][k[0]][:5])
# module counts
mo=collections.Counter(v['top'] for v in fo.values()); mn=collections.Counter(v['top'] for v in fn.values())
for m in sorted(set(mo)|set(mn)):
    if mo[m]!=mn[m]: print('  mod', m, mo[m], '->', mn[m])
pickle.dump(dict(pairs=pairs,gone=gone,added=added,rows=rows), open('s1.pkl','wb'))
