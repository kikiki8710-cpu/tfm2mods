# -*- coding: utf-8 -*-
# rel_053b.py — 0.5.3 릴리스 zip 전수 점검 + 내부 mod_info deps 를 '>=0.5.3, <0.5.4' 로 통일.
#   ⚠crm zip 은 mod_info 의도적 미동봉(이중 로드 방지) = 정상 · daram2 9종은 author=daram2 보존.
#   PII 검사 = 유저 로컬 절대경로 문자열(C:\Users\<name>) 잔존 여부.
import os, sys, io, zipfile, json, shutil, re

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
OUT = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\release\0.5.3"
STAGE = r"C:\Users\dev\AppData\Local\Temp\claude\C--Users-dev-Desktop-claude-tfm2\89fdc8ca-ad45-4a5c-a3de-4f289a646255\scratchpad\rel053b"
TARGET = ">=0.5.3, <0.5.4"
PII = [rb"C:\\Users\\dev", rb"C:/Users/dev"]


def scan(zp):
    z = zipfile.ZipFile(zp)
    infos, pii = [], []
    for i in z.infolist():
        if i.filename.endswith("/"):
            continue
        data = z.read(i.filename)
        if i.filename.endswith("mod.mod_info"):
            bom = data[0] != 0x7b
            try:
                j = json.loads(data.decode("utf-8"))
                infos.append((i.filename, j.get("version"), j.get("author"),
                              [x.get("version") for x in j.get("dependencies", []) if x.get("mod_id") == "base"], bom))
            except Exception as e:
                infos.append((i.filename, f"PARSE_ERR {e}", None, None, bom))
        for p in PII:
            if re.search(p, data, re.I):
                pii.append(i.filename)
                break
    return infos, pii


def fix(zp):
    """zip 내부 mod.mod_info 의 base deps 를 TARGET 으로 (필요할 때만 재작성)"""
    z = zipfile.ZipFile(zp)
    names = [i.filename for i in z.infolist() if i.filename.endswith("mod.mod_info")]
    need = False
    patched = {}
    for n in names:
        raw = z.read(n)
        if raw[0] != 0x7b:
            print(f"     ⚠{n} BOM — 건너뜀")
            continue
        j = json.loads(raw.decode("utf-8"))
        hit = False
        for x in j.get("dependencies", []):
            if x.get("mod_id") == "base" and x.get("version") != TARGET:
                x["version"] = TARGET
                hit = True
        if hit:
            need = True
            patched[n] = json.dumps(j, ensure_ascii=False, indent=2).encode("utf-8")
    if not need:
        return False
    os.makedirs(STAGE, exist_ok=True)
    tmp = os.path.join(STAGE, os.path.basename(zp))
    if os.path.exists(tmp):
        os.remove(tmp)
    with zipfile.ZipFile(tmp, "w", zipfile.ZIP_DEFLATED, compresslevel=9) as out:
        for i in z.infolist():
            if i.filename.endswith("/"):
                continue
            out.writestr(i.filename, patched.get(i.filename, z.read(i.filename)))
    z.close()
    shutil.copy2(tmp, zp)
    return True


zips = sorted(f for f in os.listdir(OUT) if f.endswith(".zip"))
print(f"■ 0.5.3 릴리스 zip {len(zips)}개\n")
for f in zips:
    zp = os.path.join(OUT, f)
    if len(sys.argv) > 1 and sys.argv[1] == "fix":
        if fix(zp):
            print(f"  {f}: deps 갱신함")
    infos, pii = scan(zp)
    n = len(zipfile.ZipFile(zp).namelist())
    print(f"  {f}  {os.path.getsize(zp):,}B  엔트리 {n}")
    if not infos:
        print("     mod_info 없음 (crm = 의도적 미동봉)")
    for nm, ver, au, dep, bom in infos:
        flag = "" if dep == [TARGET] else "  ← ★deps 불일치"
        print(f"     {nm:46s} v{ver} author={au} base={dep}{' BOM!' if bom else ''}{flag}")
    print(f"     PII: {'★' + str(pii[:3]) if pii else '0건'}")
