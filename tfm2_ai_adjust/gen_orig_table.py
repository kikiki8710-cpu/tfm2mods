# patch_imm_bytes 의 '원본값 일치' 가드용 테이블을 검증기에서 생성한다.
#  형식: (instruction_rva: u32, imm_off: u8, width: u8, expect_orig: u64)
#  출처는 verify_deep.py(1차 125) + wave2_sites.json(2차 205). 손으로 적지 않는다.
import io, os, sys, json
sys.stdout.reconfigure(encoding="utf-8")
HERE = os.path.dirname(os.path.abspath(__file__))

rows = {}   # (rva, off, width) -> orig

# ── 1차 ──
src = io.open(os.path.join(HERE, "verify_deep.py"), encoding="utf-8").read()
src = src.split("okc, fails")[0] + "\nRESULT = C\n"
ns = {}
exec(compile(src, "verify_deep", "exec"), ns)
for lab, rva, cands, w, want in ns["RESULT"]:
    # 후보 prefix 여러 개면 실제로 맞는 것 하나만 남는다 — resolve 는 verify 가 이미 했으므로
    # 여기서는 imm 오프셋이 후보마다 갈릴 수 있어 (rva,width) 기준으로만 기록하고 off 는 확정분을 쓴다.
    for pre, off in cands:
        g = ns["rd"](rva, len(pre))
        if g is not None and list(g) == pre and ns["imm"](rva, off, w) == want:
            rows[(rva, off, w)] = want
            break

# ── 2차 ──
d = json.load(io.open(os.path.join(HERE, "wave2_sites.json"), encoding="utf-8"))
for k, v in d.items():
    w, orig = v["w"], v["orig"]
    for a, pre, off in v["sites"]:
        rows[(a, off, w)] = orig
    for a, pre, off in v.get("sites_p1", []):
        rows[(a, off, w)] = orig + 1

items = sorted(rows.items())
out = []
out.append("// ⚠자동 생성 파일 — `gen_orig_table.py` 가 검증기에서 뽑는다. **손으로 고치지 말 것.**")
out.append("//   용도 = `patch_imm_bytes` 의 '원본값 일치' 가드(#26).")
out.append("//   주소가 어긋났는데 prefix 가 우연히 같으면 패치가 '성공'으로 계상되던 구조적 결함을 막는다")
out.append("//   (2026-08-03 `ld_chase_stop` 실사고: 원본이 5인 슬롯에 15000을 써 넣고 있었다).")
out.append("//   형식 = (instruction_rva, imm_off, width, expect_orig). rva 오름차순 — 이분탐색.")
out.append("pub static EXPECT_ORIG: &[(u32, u8, u8, u64)] = &[")
for (rva, off, w), orig in items:
    out.append("    (0x%06x, %d, %d, %d)," % (rva, off, w, orig))
out.append("];")
io.open(os.path.join(HERE, "src", "orig_table.rs"), "w", encoding="utf-8").write("\n".join(out) + "\n")
print("orig_table.rs 생성: %d 사이트" % len(items))
