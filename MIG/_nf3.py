exec(open('_nf2.py',encoding='utf-8').read().split("A=load(E57")[0])
A=load(E57,'_s57c.pkl'); B=load(E58,'_s58c.pkl')
MOD={'game-ai\src\small_action.rs','game-ai\src\small_action\around.rs','game-ai\src\small_action\cast.rs','game-ai\src\small_action\move_actions.rs'}
for tag,pack in (('57',A),('58',B)):
    o=leascan(pack,MOD)
    print("==",tag,len(o))
    for (b,e),c in sorted(o.items(), key=lambda x:-(x[0][1]-x[0][0])):
        print("  %08x sz=%-8s %s"%(b,hex(e-b),c.most_common(3)))
