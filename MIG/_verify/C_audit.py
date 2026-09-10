# -*- coding: utf-8 -*-
"""C_audit — 담당 5명세의 reads/writes 오프셋 주장을 tcxdict 사전으로 전수 대조."""
import io, json, os, re, subprocess, sys
MIG = r"C:\tfm2mods\MIG"
V   = os.path.join(MIG, "_verify")
FILES = ["10_should_end_object_finish_kill_priority_battl.json","11_v3_fall_back_to_passive.json",
         "12_handle_chat.json","13_target_bush_v30.json","14_update.json"]
# base 문자열 -> tcx 타입명 정규화
def norm(b):
    b = b.strip()
    b = re.sub(r"\(.*?\)", "", b)          # 괄호 주석 제거
    b = b.split("(")[0].strip()
    m = re.match(r"^[A-Za-z_][A-Za-z0-9_]*", b)
    return m.group(0) if m else None
SKIP = ("vtable",)
rows=[]
for f in FILES:
    d=json.load(io.open(os.path.join(V,f),encoding="utf-8"))
    for sec in ("reads","writes"):
        for e in d.get(sec,[]) or []:
            base=e.get("base",""); off=e.get("offset","")
            if any(s in base.lower() for s in SKIP): 
                rows.append((f,sec,base,off,e.get("name",""),"SKIP(vtable)")); continue
            t=norm(base)
            if not t or not re.match(r"^0x[0-9a-fA-F]+$", str(off).split("[")[0]):
                rows.append((f,sec,base,off,e.get("name",""),"SKIP(형식)")); continue
            o=str(off).split("[")[0]
            p=subprocess.run([sys.executable,"-X","utf8",os.path.join(MIG,"tcxdict.py"),t,o],
                             capture_output=True,text=True,encoding="utf-8",errors="replace")
            out=p.stdout or ""
            hits=[l.strip() for l in out.splitlines() if l.strip().startswith("★")]
            if "모호" in out: rows.append((f,sec,base,off,e.get("name",""),"모호"))
            elif hits: rows.append((f,sec,base,off,e.get("name",""),"; ".join(h[1:].strip() for h in hits)[:150]))
            else: rows.append((f,sec,base,off,e.get("name",""),"확인불가: "+out.strip().splitlines()[-1][:80] if out.strip() else "확인불가"))
w=io.open(os.path.join(V,"C_audit.tsv"),"w",encoding="utf-8")
for r in rows: w.write("\t".join(str(x) for x in r)+"\n")
w.close()
print("rows:",len(rows))
