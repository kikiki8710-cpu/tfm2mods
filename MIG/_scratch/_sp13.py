exec(open('srcmap.py',encoding='utf-8').read())
import hashlib,pickle,os,time
md2=Cs(CS_ARCH_X86,CS_MODE_64); md2.detail=False
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
NUM=re.compile(r'0x[0-9a-f]+')
def build(path,cache):
    if os.path.exists(cache): return pickle.load(open(cache,'rb'))
    img=Img(path); img.prep()
    out={}
    t=time.time()
    for i,(b,e) in enumerate(img._fs):
        if e<=b or e-b>0x20000: continue
        code=img.code(b,e-b)
        if not code: continue
        h=hashlib.md5(); n=0
        for ins in md2.disasm(code,BASE+b):
            h.update((ins.mnemonic+" "+NUM.sub('I',ins.op_str)).encode()); n+=1
        out[b]=(h.hexdigest(),n,e-b)
    print(path,"fns",len(out),"%.1fs"%(time.time()-t))
    pickle.dump(out,open(cache,'wb'))
    return out
A=build(E57,'_nh57.pkl'); B=build(E58,'_nh58.pkl')
import collections
ha=collections.defaultdict(list); hb=collections.defaultdict(list)
for r,(h,n,s) in A.items(): ha[h].append(r)
for r,(h,n,s) in B.items(): hb[h].append(r)
un57=[r for r,(h,n,s) in A.items() if h not in hb]
un58=[r for r,(h,n,s) in B.items() if h not in ha]
print("unmatched 0.5.7:",len(un57)," 0.5.8:",len(un58))
pickle.dump((un57,un58),open('_unm.pkl','wb'))
