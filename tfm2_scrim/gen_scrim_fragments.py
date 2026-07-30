# -*- coding: utf-8 -*-
# scrim UI 를 override(main.ui 통째교체) → ui_inject 프레임워크 주입 으로 전환.
#   기존 override main.ui 에서 scrim 추가블록 2개만 추출 → 조각 .ui:
#     scrim_category(사이드바 버튼) → left after:house_category
#     scrim_modal(모달)            → root end modal
#   ⚠ 조각 루트는 '#' 없이(파서 규칙). dll 은 노드를 id/path 로 구동(변경 불필요).
import io, os

SRC_MAIN = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_scrim\ui\layout\main.ui"
OUT_SRC = r"C:\tfm2mods\tfm2_scrim\ui_inject"
OUT_DEP = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_scrim\ui_inject"
DEP_ROOT = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_scrim"

lines = io.open(SRC_MAIN, encoding="utf-8").read().split("\n")

def extract_block(root_id):
    start = None
    for i, ln in enumerate(lines):
        if ("#" + root_id + ":") in ln and "{" in ln:
            start = i; break
    if start is None:
        raise SystemExit("root not found: " + root_id)
    depth = 0; end = None
    for j in range(start, len(lines)):
        depth += lines[j].count("{") - lines[j].count("}")
        if depth == 0:
            end = j; break
    if end is None:
        raise SystemExit("brace unbalanced: " + root_id)
    block = lines[start:end + 1]
    indent = len(block[0]) - len(block[0].lstrip())
    out = []
    for b in block:
        out.append(b[indent:] if b[:indent].strip() == "" else b.lstrip())
    out[0] = out[0].lstrip()
    if out[0].startswith("#"):
        out[0] = out[0][1:]   # 루트만 # 제거
    return "\n".join(out).rstrip() + "\n", (end - start + 1)

button, nb = extract_block("scrim_category")
modal, nm = extract_block("scrim_modal")
strat, ns = extract_block("scrim_strat_modal")  # 팀전술 모달 (별도 루트블록 — 누락됐던 것)
items, ni = extract_block("scrim_items_modal")  # 개인전술(아이템) 모달 (별도 루트블록 — 누락됐던 것)
ddp, nddp = extract_block("scrim_dd_p")         # 드롭다운: 선수/아이템 (폭240, dll이 set_visible 로 찾음)
dds, ndds = extract_block("scrim_dd_s")         # 드롭다운: 전술/스플릿 (폭304)
ddc, nddc = extract_block("scrim_dd_c")         # 드롭다운: 챔프 그리드
# 사이드바 버튼 글자가 bold 라 네이티브보다 두꺼움 → regular 로 (버튼만; 모달 헤더 bold 는 유지)
button = button.replace("asset/base/font/set/bold", "asset/base/font/set/regular")
print("scrim_category block: %d lines" % nb)
print("scrim_modal       block: %d lines" % nm)
print("scrim_strat_modal block: %d lines" % ns)
print("scrim_items_modal block: %d lines" % ni)
print("scrim_dd_p/s/c blocks: %d/%d/%d lines" % (nddp, ndds, nddc))
# sanity: 루트 # 없는지 + 자식 # 있는지
assert button.lstrip().startswith("scrim_category:"), "button root bad"
assert modal.lstrip().startswith("scrim_modal:"), "modal root bad"
assert strat.lstrip().startswith("scrim_strat_modal:"), "strat root bad"
assert items.lstrip().startswith("scrim_items_modal:"), "items root bad"
assert ddp.lstrip().startswith("scrim_dd_p:"), "dd_p root bad"
assert dds.lstrip().startswith("scrim_dd_s:"), "dd_s root bad"
assert ddc.lstrip().startswith("scrim_dd_c:"), "dd_c root bad"
assert "#scrim_btn" in button, "scrim_btn missing"
assert "#scrim_close" in modal, "scrim_close missing"
assert "#scrim_open_strat" in modal, "scrim_open_strat missing"  # 모달에 전술 열기 버튼 있어야
assert "#scrim_strat_ok" in strat or "#scrim_st_tab_b" in strat, "strat content missing"
assert "#scrim_ddp_00" in ddp, "scrim_ddp rows missing"
assert "#scrim_dds_00" in dds, "scrim_dds rows missing"

MANIFEST = """# scrim UI 조각 (override main.ui → 프레임워크 주입 전환)
ui_inject/scrim_button.ui left after:house_category
ui_inject/scrim_modal.ui root end modal
ui_inject/scrim_strat_modal.ui root end modal
ui_inject/scrim_items_modal.ui root end modal
ui_inject/scrim_dd_p.ui root end modal
ui_inject/scrim_dd_s.ui root end modal
ui_inject/scrim_dd_c.ui root end modal
"""

for d in (OUT_SRC, OUT_DEP):
    os.makedirs(d, exist_ok=True)
    io.open(os.path.join(d, "scrim_button.ui"), "w", encoding="utf-8").write(button)
    io.open(os.path.join(d, "scrim_modal.ui"), "w", encoding="utf-8").write(modal)
    io.open(os.path.join(d, "scrim_strat_modal.ui"), "w", encoding="utf-8").write(strat)
    io.open(os.path.join(d, "scrim_items_modal.ui"), "w", encoding="utf-8").write(items)
    io.open(os.path.join(d, "scrim_dd_p.ui"), "w", encoding="utf-8").write(ddp)
    io.open(os.path.join(d, "scrim_dd_s.ui"), "w", encoding="utf-8").write(dds)
    io.open(os.path.join(d, "scrim_dd_c.ui"), "w", encoding="utf-8").write(ddc)
    print("wrote fragments to", d)
for base in (r"C:\tfm2mods\tfm2_scrim", DEP_ROOT):
    os.makedirs(base, exist_ok=True)
    io.open(os.path.join(base, "ui_inject.txt"), "w", encoding="utf-8").write(MANIFEST)
    print("wrote manifest to", base)

# override_info: main 레이아웃 override 제거, text/ui merge 만 유지 (백업 후)
ovr_path = os.path.join(DEP_ROOT, "mod.override_info")
if os.path.exists(ovr_path):
    bak = os.path.join(DEP_ROOT, "mod.override_info.preinject.bak")
    if not os.path.exists(bak):
        io.open(bak, "w", encoding="utf-8").write(io.open(ovr_path, encoding="utf-8").read())
        print("backed up override_info →", bak)
NEW_OVERRIDE = '''{
  "asset/base/text/ui": {
    "remapping": "asset/Scrim_Probe/text/ui",
    "type": "merge"
  }
}
'''
io.open(ovr_path, "w", encoding="utf-8").write(NEW_OVERRIDE)
print("rewrote override_info (main override 제거, text/ui merge 유지)")
