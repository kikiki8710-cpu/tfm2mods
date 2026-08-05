# -*- coding: utf-8 -*-
# illust_054g.py — 독립 2번째 방법: 구조지문(skel/head) 대조 + 유일성 검사
import pickle, sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
PO = pickle.load(open(r"C:\tfm2mods\_fnidx_053.pkl", "rb"))
PN = pickle.load(open(r"C:\tfm2mods\_fnidx_054.pkl", "rb"))
IO_, IN_ = PO["idx"], PN["idx"]
SO_, SN_ = PO["by_skel"], PN["by_skel"]
HO_, HN_ = PO["by_head"], PN["by_head"]

MAP = [("RVA_FX_SET",0x1bd8e50,0x1d92980),("RVA_CARD_DRAW",0x1bee8e0,0x1da8410),
       ("RVA_ILLUST_GET",0x1e91400,0x1ffd970),("RVA_SUBMIT",0x1859f0,0x181400),
       ("RVA_SUBMIT_TEXT",0x185c70,0x181680),("RVA_IMG_BUILD",0x187110,0x182b20),
       ("RVA_IMG_UV",0x186f70,0x182980),("RVA_IMG_FLAG",0x187420,0x182e30),
       ("RVA_IMG_COLOR",0x23b8150,0x1c2f8d0),("RVA_IMG_SHADER",0x188a20,0x184430),
       ("RVA_TEXT_BUILD",0x186600,0x182010),("RVA_NAME_GET",0x1c19520,0x1dd4240),
       ("RVA_ASSET_GET",0x143d50,0x143d50),("RVA_ANIM_GET",0x888fd0,0x74c010),
       ("RVA_SPRITE_CALC",0x1c1e4e0,0x1dd9170),("RVA_GAME_ALLOC",0x28f7df0,0x29bb920)]

print(f"{'상수':16s} {'0.5.3':>9s} {'0.5.4':>9s} {'size':>11s} {'skel':>6s} {'head':>6s}  판정")
print("="*104)
for nm, o, n in MAP:
    a, b = IO_.get(o), IN_.get(n)
    if not a or not b:
        print(f"  {nm:16s} 인덱스 없음 (old={bool(a)} new={bool(b)})"); continue
    skel_eq = a["skel"] == b["skel"]
    head_eq = a["head"] == b["head"]
    # 유일성: OLD 의 head 를 가진 NEW 함수 목록
    cand = HN_.get(a["head"], [])
    oc = HO_.get(a["head"], [])
    scand = SN_.get(a["skel"], [])
    verdict = []
    if skel_eq: verdict.append("skel완전일치")
    if head_eq: verdict.append("head일치")
    uniq = f"headNEW후보{len(cand)}/OLD{len(oc)}"
    if scand: uniq += f" skelNEW후보{len(scand)}"
    print(f"  {nm:16s} {o:#9x} {n:#9x} {a['size']:>5d}→{b['size']:<5d} "
          f"{'O' if skel_eq else 'X':>6s} {'O' if head_eq else 'X':>6s}  "
          f"{','.join(verdict) or '-'}  [{uniq}]"
          + ("" if not cand or n in cand else "  ⚠NEW후보에 미포함"))
