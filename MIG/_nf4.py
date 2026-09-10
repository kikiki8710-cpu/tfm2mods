exec(open('_nf2.py',encoding='utf-8').read().split("A=load(E57")[0])
B=load(E58,'_s58c.pkl'); A=load(E57,'_s57c.pkl')
for r in (0xdfb840,0xdf36e0,0xe23fa0,0xe23750,0xe4c5c0,0xe65b10,0xcaf9f0,0xcafe10,0xe83390):
    fr,h=analyze(B[0],B[1],B[2],r)
    print("58 %08x sz=%-8s %s"%(r,hex(fr[1]-fr[0]) if fr else '?', h.most_common(3) if h else ''))
