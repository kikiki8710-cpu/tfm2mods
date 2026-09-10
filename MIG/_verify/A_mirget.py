import io,sys,re
PATH=r'C:/tfm2mods/MIG/_tcx/mirdump_game_core.txt'
def get(pats, path=PATH, maxb=400):
    pats=[p.lower() for p in pats]
    out=[];cur=None;buf=[]
    with io.open(path,encoding='utf-8',errors='replace') as f:
        for line in f:
            if line.startswith('### '):
                if cur is not None:
                    out.append((cur,buf))
                h=line.rstrip('\n')
                hl=h.lower()
                cur = h if any(p in hl for p in pats) else None
                buf=[]
            elif cur is not None:
                if len(buf)<maxb: buf.append(line.rstrip('\n'))
        if cur is not None: out.append((cur,buf))
    return out
if __name__=='__main__':
    path = PATH
    args = sys.argv[1:]
    if args and args[0].startswith('@'):
        path = args[0][1:]; args = args[1:]
    for h,b in get(args, path):
        print(h); print('\n'.join(b)); print()
