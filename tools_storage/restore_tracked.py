"""E: 이관 중 git 추적 파일까지 옮겨진 것을 원위치로 되돌린다 (2026-10-21).
git 이 추적하는 파일은 저장소 작업 트리에 있어야 한다(옮기면 '삭제됨'으로 보인다). MOVE_MANIFEST 의 file·OK 행마다
소속 저장소(.git 파일/폴더를 위로 탐색)에서 `git ls-files --error-unmatch` 로 추적 여부를 보고, 추적이면 E: 사본을 원위치로 복사·대조 후
E: 사본 삭제 + 원래 폴더 _MOVED_TO_E.txt 에서 그 줄 삭제 + MANIFEST 에 RESTORED_tracked 행 추가.
사용: python -B -X utf8 restore_tracked.py [--dry]
"""
import csv, datetime, os, shutil, subprocess, sys

MAN = r"E:\claude\tfm2\MOVE_MANIFEST.csv"
dry = "--dry" in sys.argv


def repo_root(p):
    d = os.path.dirname(p)
    while True:
        if os.path.exists(os.path.join(d, ".git")):
            return d
        nd = os.path.dirname(d)
        if nd == d:
            return None
        d = nd


rows = list(csv.DictReader(open(MAN, encoding="utf-8-sig")))
done = {r["source"] for r in rows if r["result"] == "RESTORED_tracked"}
out = []
for r in rows:
    if r["type"] != "file" or r["result"] != "OK" or r["source"] in done:
        continue
    src, dst = r["source"], r["dest"]
    root = repo_root(src)
    if not root:
        continue
    rel = os.path.relpath(src, root)
    tracked = subprocess.run(["git", "-C", root, "ls-files", "--error-unmatch", rel], capture_output=True).returncode == 0
    if not tracked:
        continue
    print(("DRY " if dry else "") + "restore", src)
    if dry:
        continue
    shutil.copy2(dst, src)
    assert os.path.getsize(src) == os.path.getsize(dst)
    os.remove(dst)
    note = os.path.join(os.path.dirname(src), "_MOVED_TO_E.txt")
    if os.path.exists(note):
        keep = [l for l in open(note, encoding="utf-8-sig").read().splitlines() if not l.startswith(os.path.basename(src) + " -> ")]
        if keep:
            open(note, "w", encoding="utf-8").write("\n".join(keep) + "\n")
        else:
            os.remove(note)
    out.append(f'{datetime.datetime.utcnow().isoformat()}Z,{r["kind"]},file,"{src}","{dst}",1,{r["bytes"]},RESTORED_tracked')
if out:
    open(MAN, "a", encoding="utf-8").write("\n".join(out) + "\n")
print("restored", len(out))
