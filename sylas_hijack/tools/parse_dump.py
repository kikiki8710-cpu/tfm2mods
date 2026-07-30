# effect_dump.txt 파서 + 시트 대조 → 챔프별 궁 구조/값 디코드
import re, json, sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
from effect_types import build_type_table
DUMP = r"C:\tfm2mods\sylas_hijack\effect_dump.txt"
SHEET = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\bundle_unpacked_full\setting\champion_info.champion_info_sheet"

def parse():
    txt = open(DUMP, encoding="utf-8", errors="replace").read()
    champs = {}
    cur = None; cur_slot = None
    for line in txt.splitlines():
        m = re.match(r"===== (\w+)( ULT#\d+)? ent=", line)
        if m:
            cur = m.group(1); champs.setdefault(cur, {})
            cur_slot = None
            continue
        m = re.match(r"\[(exec\.\w+|adef)\]", line)
        if m:
            cur_slot = m.group(1)
            champs.setdefault(cur, {}).setdefault(cur_slot, [])  # 누적(ULT 재덤프 병합)
            continue
        # 노드 라인: 들여쓰기 + vt=RVA:.. sz=.. d=.. [words]
        m = re.match(r"(\s*)vt=RVA:(0x[0-9a-f]+) sz=(0x[0-9a-f]+) d=(0x[0-9a-f]+) \[(.*)\]", line)
        if m and cur and cur_slot:
            depth = len(m.group(1)) // 2
            vt = int(m.group(2),16); sz=int(m.group(3),16); d=int(m.group(4),16)
            words = [int(x,16) for x in m.group(5).split()]
            champs[cur][cur_slot].append({"depth":depth,"vt":vt,"sz":sz,"d":d,"words":words,"strs":{}})
            continue
        # v2 문자열 라인: 직전 노드의 String 필드 (s@+0x18="asset/...")
        m = re.match(r'\s*s@\+(0x[0-9a-f]+)="(.*)"', line)
        if m and cur and cur_slot and champs.get(cur,{}).get(cur_slot):
            champs[cur][cur_slot][-1]["strs"][int(m.group(1),16)] = m.group(2)
    return champs

def sheet_ult(name):
    d = json.load(open(SHEET, encoding="utf-8"))
    return d.get(name,{}).get("ult",{})

def main():
    types = build_type_table()
    champs = parse()
    print(f"덤프된 챔프 {len(champs)}명: {sorted(champs)}\n")
    target = sys.argv[1] if len(sys.argv)>1 else None
    for name in ([target] if target else sorted(champs)):
        if name not in champs: print(f"[{name}] 덤프 없음"); continue
        print(f"===== {name} =====")
        su = sheet_ult(name)
        print(f"  시트 ult: {su}")
        for slot in ["exec.ult","adef"]:
            nodes = champs[name].get(slot, [])
            if not nodes: continue
            print(f"  [{slot}] 노드 {len(nodes)}개:")
            for n in nodes:
                disc = n["words"][0] & 0xff
                tname = types.get(disc, "?")
                # 값 후보 = 워드 중 시트값과 겹치는 것
                print(f"    {'  '*n['depth']}vt=0x{n['vt']:x} sz=0x{n['sz']:x} disc={disc}({tname}) words={[hex(w) for w in n['words'][:8]]}")
                for off, st in sorted(n.get("strs",{}).items()):
                    print(f"    {'  '*n['depth']}  └ str@+{off:#x} = \"{st}\"")
        print()

if __name__=="__main__":
    main()
