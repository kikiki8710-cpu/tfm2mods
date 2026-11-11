# -*- coding: utf-8 -*-
"""상세전술 0.7.2 릴리스 zip — target/package 산출물 검사 후 release\\0.7.2 로 복사(2026-11-11 · 0.7.1 판 _rel071_detail.py 사본).

검사: 엔트리 4개 · zip 루트 폴더 한 겹 · dll = 배포본 · mod_info BOM 없음·버전·deps · README 대상 버전 · cfg 기본값 · PII 없음.
"""
import hashlib
import json
import os
import shutil
import sys
import zipfile

MOD = 'tfm2_detail_tactics'
VER = '0.2.6'
GAME = '0.7.2'
ROOT = r'F:\claude\tfm2mods'
SRC = os.path.join(ROOT, MOD, 'target', 'package', MOD + '.zip')
DEPLOY = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\%s\%s.dll' % (MOD, MOD)
REVIEW_CFG = os.path.join(ROOT, MOD, 'target', 'review', 'detail_tactics_values.cfg')
OUT = os.path.join(ROOT, 'release', GAME)
PII = [b'C:\\Users', b'jungs', b'kikiki8710', 'C:\\Users'.encode('utf-16le'), 'jungs'.encode('utf-16le')]


def sha(b):
    return hashlib.sha256(b).hexdigest()


z = zipfile.ZipFile(SRC)
names = [i.filename.replace(chr(92), '/') for i in z.infolist()]
want = {MOD + '/' + n for n in (MOD + '.dll', 'mod.mod_info', 'README.md', 'detail_tactics_values.cfg')}
assert set(names) == want, names
print('엔트리', len(names))
data = {n.split('/')[-1]: z.read(i) for n, i in zip(names, z.infolist())}
dll = data[MOD + '.dll']
assert sha(dll) == sha(open(DEPLOY, 'rb').read()), 'zip dll ≠ 배포 dll'
print('dll = 배포본', len(dll), sha(dll)[:12].upper())
info = data['mod.mod_info']
assert info[:1] == b'{', 'BOM'
j = json.loads(info.decode('utf-8'))
assert j['version'] == VER and j['dependencies'][0]['version'] == '>=0.7.2, <0.7.3', j
print('mod_info', j['version'], j['dependencies'][0]['version'], '첫 바이트 %02x' % info[0])
readme = data['README.md'].decode('utf-8')
assert readme.startswith('# 상세전술 ' + VER) and '게임 ' + GAME + ' 전용' in readme
print('README 대상 버전 OK')
cfg = data['detail_tactics_values.cfg']
assert cfg == open(REVIEW_CFG, 'rb').read() and cfg[:3] != b'\xef\xbb\xbf'
print('cfg = 기본값 내보내기', len(cfg))
hits = [(n, p) for n, b in data.items() for p in PII if p in b]
assert not hits, hits
print('PII 0')
os.makedirs(OUT, exist_ok=True)
a = os.path.join(OUT, MOD + '.zip')
b = os.path.join(OUT, '%s-%s.zip' % (MOD, VER))
for p in (a, b):
    shutil.copyfile(SRC, p)
h = sha(open(a, 'rb').read())
assert h == sha(open(b, 'rb').read())
open(b + '.sha256', 'w', encoding='utf-8', newline='\n').write('%s  %s\n' % (h, os.path.basename(b)))
print('zip', os.path.getsize(a), h[:12].upper(), '→', OUT)
