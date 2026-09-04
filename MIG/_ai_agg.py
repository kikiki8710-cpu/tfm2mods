# -*- coding: utf-8 -*-
import json,io,sys,collections,re
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
V=json.load(open(r"C:\tfm2mods\MIG\_ai_verdict2.json",encoding="utf-8"))
C=[r for r in V if r["verdict"]=="CONST_OR_OFFSET_CHANGED"]
pat=re.compile(r"^(\w+),(.*)$")
shift=collections.Counter(); kind=collections.Counter(); imm=collections.Counter()
mem=re.compile(r"m\[(\d+)\+(\d+)\*(\d+)\+(0x[0-9a-f]+|0)\]")
for r in C:
    for k,x,y in r.get("diff_sample",[]):
        mx=mem.findall(x); my=mem.findall(y)
        if len(mx)==1 and len(my)==1 and mx[0][:3]==my[0][:3]:
            a=int(mx[0][3],16); b=int(my[0][3],16)
            shift[(a,b)]+=1; kind["mem_disp"]+=1
        else:
            kind["기타"]+=1
            if "movabs" in x: kind["movabs 상수(해시시드류)"]+=1
print("=== CONST_OR_OFFSET_CHANGED 71건의 diff 표본 유형 ===", dict(kind))
print("\n=== 관측된 오프셋 이동 (구→신, 표본 건수) ===")
for (a,b),n in shift.most_common(40):
    print("   %#-7x -> %#-7x  (Δ%+#x)  %d건"%(a,b,b-a,n))
d=collections.Counter(b-a for (a,b),n in shift.items() for _ in range(n))
print("\n   Δ 분포:",dict(d))
