# -*- coding: utf-8 -*-
"""패키지 zip — 스테이지 폴더를 zip 루트에 폴더 한 겹으로 담는다(엔트리 구분자 '/' · ZIP 규격).

    python -X utf8 tools/make_zip.py <스테이지 폴더> <zip 경로>

PowerShell 5.1 Compress-Archive 는 엔트리 이름에 '\\' 를 써서(ZIP 규격 위반) 윈도우 밖 압축 도구에서 폴더가 안 풀린다
(deploy-verify 2026-11-11 경고 — 0.2.3~0.2.6 릴리스 zip 전부). 그래서 package_local.ps1 이 이 스크립트를 부른다.
"""
import os
import sys
import zipfile

stage, out = sys.argv[1], sys.argv[2]
root = os.path.basename(os.path.normpath(stage))
names = sorted(os.listdir(stage))
assert names, stage
tmp = out + '.tmp'
with zipfile.ZipFile(tmp, 'w', zipfile.ZIP_DEFLATED) as z:
    for n in names:
        p = os.path.join(stage, n)
        assert os.path.isfile(p), ('하위 폴더는 넣지 않는다', p)
        z.write(p, root + '/' + n)
os.replace(tmp, out)
with zipfile.ZipFile(out) as z:
    got = [i.filename for i in z.infolist()]
assert all('\\' not in g and g.startswith(root + '/') for g in got), got
print(out, os.path.getsize(out), got)
