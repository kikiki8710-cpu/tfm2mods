# -*- coding: utf-8 -*-
"""
mkprobe060.py — 0.6.0 1단계 프로브 표 생성(stable 껍데기 하네스 `tfm2_judge_verify060` 용).
specs20_v060.json 의 v060.addr(0.6.0 RVA) 마다 진입부를 capstone 으로 읽어 「위치독립 명령 경계 ≥12B」 를 고르고
`tfm2_judge_verify060/src/probe_tbl.rs` 에 (idx, rva, name, orig bytes) 표를 쓴다. 조건 미달(rip-relative/분기/12B 미만) 은 목록으로 보고.
사용: python mkprobe060.py
"""
import io, json, os, sys, struct
sys.stdout.reconfigure(encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
HERE = os.path.dirname(os.path.abspath(__file__))
EXE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
OUT = os.path.join(HERE, "..", "tfm2_judge_verify060", "src", "probe_tbl.rs")
d = io.open(EXE, "rb").read()
pe = struct.unpack_from("<I", d, 0x3c)[0]; nsec = struct.unpack_from("<H", d, pe + 6)[0]; opt = struct.unpack_from("<H", d, pe + 20)[0]
secs = [struct.unpack_from("<IIII", d, pe + 24 + opt + i * 40 + 12) for i in range(nsec)]
def rva2off(r):
    for va, vsz, raw, rsz in secs:
        if va <= r < va + max(vsz, rsz): return raw + (r - va)
cs = Cs(CS_ARCH_X86, CS_MODE_64); cs.detail = True
def entry(rva, want=12, maxlen=32):
    off = rva2off(rva); code = d[off:off + 64]; acc = 0; bad = None
    for ins in cs.disasm(code, 0x140000000 + rva):
        if ins.mnemonic.startswith(("j", "call", "ret", "loop")) or "rip" in ins.op_str: bad = u"%s %s @+%d" % (ins.mnemonic, ins.op_str, acc); break
        acc += ins.size
        if acc >= want: return acc, code[:acc].hex(), None
        if acc > maxlen: break
    return acc, None, bad or u"경계 %d<%d" % (acc, want)
V = json.load(io.open(os.path.join(HERE, "_spec", "specs20_v060.json"), encoding="utf-8"))
rows = []; skipped = []; seen = {}
for sp in V["specs"]:
    b = sp["v060"]; a = b.get("addr")
    if not a: continue
    rva = int(a, 16)
    if rva in seen: skipped.append((a, sp["name"], u"중복(%s 와 같은 RVA)" % seen[rva])); continue
    n, hexb, why = entry(rva)
    if hexb is None: skipped.append((a, sp["name"], why)); continue
    seen[rva] = sp["name"]
    rows.append((len(rows), rva, sp["name"], sp.get("i"), b["verdict"].split("(")[0], n, hexb))
L = [u"// probe_tbl.rs — 자동 생성(MIG\\mkprobe060.py · 0.6.0 exe %s). 손으로 고치지 말 것." % os.path.basename(EXE),
     u"// 각 항목 = 명세 함수 1개: idx · 0.6.0 RVA · 이름 · 진입부 원본 바이트(위치독립 · 캡처 detour 가 스텁에 복사)",
     u"pub struct Probe { pub idx: u16, pub rva: usize, pub name: &'static str, pub spec_i: u32, pub verdict: &'static str, pub orig: &'static [u8] }",
     u"pub static PROBES: &[Probe] = &["]
for idx, rva, name, i, vd, n, hexb in rows:
    bytes_ = u", ".join(u"0x%s" % hexb[k:k + 2] for k in range(0, len(hexb), 2))
    L.append(u'    Probe { idx: %d, rva: 0x%x, name: "%s", spec_i: %s, verdict: "%s", orig: &[%s] },' % (idx, rva, name.replace('"', "'"), i if i is not None else 0, vd, bytes_))
L.append(u"];"); L.append(u"pub const N: usize = %d;" % len(rows))
os.makedirs(os.path.dirname(OUT), exist_ok=True)
io.open(OUT, "w", encoding="utf-8").write(u"\n".join(L) + u"\n")
md = [u"# probe060 표 요약 — %d 프로브 · 제외 %d" % (len(rows), len(skipped)), u"", u"| idx | rva | 함수 | i | 판정 | orig_len |", u"|---|---|---|---|---|---|"]
md += [u"| %d | `%x` | %s | %s | %s | %d |" % (idx, rva, name[:44], i, vd, n) for idx, rva, name, i, vd, n, _ in rows]
if skipped: md += [u"", u"## 제외", u""] + [u"- `%s` %s — %s" % s for s in skipped]
io.open(os.path.join(HERE, "_next", "probe060_tbl.md"), "w", encoding="utf-8").write(u"\n".join(md))
print(u"프로브 %d · 제외 %d" % (len(rows), len(skipped)))
for s in skipped: print(u"  ✗ %s %s — %s" % s)
