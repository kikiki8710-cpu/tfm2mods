import sys, struct, pefile, collections, os
exec(open(os.environ['TEMP']+r"\vtscan.py", encoding='utf-8-sig').read().split("which=sys.argv[1]")[0])
res=scan(NEW)
NONE=0x1125520+BASE
eff=[r for r in res if len(r[3])>10 and (r[3][(0x38-0x18)//8]==NONE or r[3][(0x68-0x18)//8]==0x1125510+BASE)]
print("effect vtables:",len(eff))
c=collections.Counter(r[3][(0x30-0x18)//8] for r in eff)
print("slot30:",[(hex(k-BASE),v) for k,v in c.most_common()])
for r in eff:
    if r[3][(0x30-0x18)//8]!=NONE:
        print(hex(r[0]), hex(r[1]), ' '.join(hex(x-BASE) for x in r[3][:12]))
c=collections.Counter(r[3][(0x48-0x18)//8] for r in eff)
print("slot48:",[(hex(k-BASE),v) for k,v in c.most_common()])
c=collections.Counter((r[3][2]==r[3][5]) for r in eff)
print("slot28==slot40:",c)
for r in eff:
    if r[3][2]!=r[3][5]:
        print(hex(r[0]), hex(r[1]), ' '.join(hex(x-BASE) for x in r[3][:12]))
