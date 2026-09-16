import sys, struct, pefile, collections, os
exec(open(os.environ['TEMP']+r"\vtscan.py", encoding='utf-8-sig').read().split("which=sys.argv[1]")[0])
which=sys.argv[1]; size=int(sys.argv[2],16); slot=int(sys.argv[3],16); tgt=int(sys.argv[4],16)+BASE
res=scan(OLD if which=='old' else NEW)
for r in res:
    if r[1]==size and len(r[3])>(slot-0x18)//8 and r[3][(slot-0x18)//8]==tgt:
        print(hex(r[0]), ' '.join(hex(x-BASE) for x in r[3][:18]))
