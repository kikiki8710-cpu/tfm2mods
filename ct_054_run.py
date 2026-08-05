# -*- coding: utf-8 -*-
import sys, io, re, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
import ct_054 as C

SRC = r"C:\tfm2mods\tfm2_comptest_unlock\src\tfm2_comptest_unlock.rs"
RE_P = re.compile(
    r'Patch\s*\{\s*name:\s*"([^"]+)"\s*,\s*rva:\s*(0x[0-9a-fA-F]+|0)\s*,'
    r'(?:[^}]*?)orig:\s*&\[([^\]]*)\]\s*,\s*fixed:\s*&\[([^\]]*)\]', re.S)

def b(s): return bytes(int(x.strip(), 16) for x in s.split(",") if x.strip())

src = open(SRC, encoding="utf-8", errors="replace").read()
sites = []
for m in RE_P.finditer(src):
    sites.append(dict(name=m.group(1), rva=int(m.group(2), 16),
                      orig=b(m.group(3)), fixed=b(m.group(4)),
                      line=src[:m.start()].count("\n") + 1))
print(f"PATCHES {len(sites)}건\n")
out = []
for s in sites:
    if s["rva"] == 0:
        print(f"  [SKIP0] {s['name']:22s} — 0.5.3 미해결(rva 0)")
        out.append(dict(name=s["name"], old=0, status="SKIP0")); continue
    o = C.roff(C.S3, s["rva"])
    act3 = C.D3[o:o+len(s["orig"])] if o is not None else b""
    if act3 != s["orig"]:
        print(f"  [STALE] {s['name']:22s} 0x{s['rva']:x} — 0.5.3 orig 불일치 실제={act3.hex(' ')} 선언={s['orig'].hex(' ')}")
        out.append(dict(name=s["name"], old=s["rva"], status="STALE53")); continue
    r = C.repin(s["rva"], s["orig"], s["name"])
    st = r["status"]
    if st.startswith("OK"):
        print(f"  [{st:12s}] {s['name']:22s} 0x{s['rva']:x} -> **0x{r['new']:x}**  k={r['k']} "
              f"cont=0x{r['cont']:x}({r['contsize']}B)  insn='{r['site_insn']}' orig054={r['orig_actual'].hex(' ')}")
        out.append(dict(name=s["name"], old=s["rva"], new=r["new"], status=st, k=r["k"],
                        insn=r["site_insn"], orig054=r["orig_actual"].hex(" ")))
    else:
        print(f"  [{st:12s}] {s['name']:22s} 0x{s['rva']:x} — cont=0x{r['cont']:x}({r['contsize']}B) "
              f"insn='{r['site_insn']}' hits={[hex(h) for h in r['hits']]}")
        print(f"          trace(k,hits54,hits53)={r['trace']}")
        out.append(dict(name=s["name"], old=s["rva"], status=st, insn=r["site_insn"],
                        hits=[hex(h) for h in r["hits"]], trace=r["trace"]))
json.dump(out, open(r"C:\tfm2mods\_ct_054.json","w",encoding="utf-8"), indent=1, ensure_ascii=False)
print("\n요약:", {k: sum(1 for x in out if x["status"]==k) for k in sorted(set(x["status"] for x in out))})
