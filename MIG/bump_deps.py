#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""bump_deps.py — mod.mod_info 의 base 의존 대역을 새 게임 버전으로 올린다.

  python MIG\\bump_deps.py --to 0.5.8 [--dry] [--mods a,b,c]

왜 필요한가: base 요구가 `>=0.5.7, <0.5.8` 인 채로 0.5.8 게임을 켜면 **모드가 조용히
자동 비활성**된다. RVA 재핀·재빌드가 다 끝나도 이 한 줄 때문에 "아무 것도 안 되는" 것처럼 보인다.

★★왜 PowerShell 로 하지 말아야 하나 (2026-09-02 실사고 — 이 파일의 존재 이유):
  같은 일을 하는 `bump_deps_058.ps1` 을 **BOM 없는 UTF-8 + 한글 주석**으로 쓴 결과,
  PowerShell 5.1 이 그 파일을 **ANSI 로 읽어** 스크립트가 깨졌고,
  `WriteAllText` 가 빈 문자열을 써서 **mod.mod_info 30개가 0바이트가 됐다**(모드 전량 사망).
  게임측 사본은 git 밖이라 zip·소스·수동 재구성으로 복구해야 했다.
  ⟹ 파일을 건드리는 배치 작업은 **파이썬으로** 한다. .ps1 을 쓸 거면 **ASCII-only** 로.

안전장치: ①원본을 `.bak_pre<버전>` 으로 남긴다 ②쓰기 후 재파싱 + BOM·빈파일 검사
         (BOM 붙은 json 은 게임 파서가 못 읽어 모드가 강제 비활성 — CLAUDE.md §2 함정 ②)
"""
import os, sys, json, shutil, argparse

GAME = os.path.join('C:', os.sep, 'Program Files (x86)', 'Steam', 'steamapps',
                    'common', 'Teamfight Manager2', 'mods')
SRC = os.path.join('C:', os.sep, 'tfm2mods')
ALT = os.path.join('C:', os.sep, 'Users', 'jungs', 'Desktop', 'claude', 'tfm2', 'tfm2-mods-main')
ROOTS = (SRC, ALT, GAME)


def next_minor(v):
    a, b, c = (int(x) for x in v.split('.'))
    return '%d.%d.%d' % (a, b, c + 1)


def bump_one(path, new, dry, tag):
    if not os.path.isfile(path) or os.path.getsize(path) == 0:
        return None
    raw = open(path, 'rb').read()
    if raw[:3] == b'\xef\xbb\xbf':
        return ('BOM', path)
    doc = json.loads(raw.decode('utf-8'))
    hit = False
    for d in doc.get('dependencies') or []:
        if d.get('mod_id') == 'base' and d.get('version') != new:
            d['version'] = new
            hit = True
    if not hit:
        return ('skip', path)
    if dry:
        return ('DRY', path)
    shutil.copyfile(path, path + '.bak_pre' + tag)
    open(path, 'w', encoding='utf-8', newline='').write(json.dumps(doc, ensure_ascii=False))
    b = open(path, 'rb').read()
    assert b and b[:3] != b'\xef\xbb\xbf', 'BOM/빈파일 — 즉시 .bak 에서 복구할 것'
    json.loads(b.decode('utf-8'))
    return ('OK', path)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--to', required=True, help='새 게임 버전 (예: 0.5.8)')
    ap.add_argument('--mods', help='쉼표 목록. 생략하면 게임 mods\\ 전체에서 "구 대역 상한"을 가진 것')
    ap.add_argument('--dry', action='store_true')
    a = ap.parse_args()
    new = '>=%s, <%s' % (a.to, next_minor(a.to))
    tag = a.to.replace('.', '')

    if a.mods:
        mods = [m.strip() for m in a.mods.split(',') if m.strip()]
    else:
        mods = sorted(d for d in os.listdir(GAME)
                      if os.path.isfile(os.path.join(GAME, d, 'mod.mod_info')))
    n = {}
    for m in mods:
        for root in ROOTS:
            r = bump_one(os.path.join(root, m, 'mod.mod_info'), new, a.dry, tag)
            if r is None:
                continue
            n[r[0]] = n.get(r[0], 0) + 1
            if r[0] in ('OK', 'DRY', 'BOM'):
                print('%-5s %s' % (r[0], r[1]))
    print('%s -> %s' % (new, n))


if __name__ == '__main__':
    main()
