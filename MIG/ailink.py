#!/usr/bin/env python3
"""ailink.py — SDK game_ai 비트코드 rlib 을 **패치해서 재조립**한다(모드에 직접 링크하기 위한 전처리).

무엇을 하나: `MIG/patches.json` 의 명세대로 IR 안의 상수/연산을 바꾼 rlib 을 만든다.
  명세는 RVA 가 아니라 **소스 좌표(파일:줄)** 로 지정한다 — 게임 패치가 와도 안 썩고,
  줄이 밀려 못 찾으면 조용히 넘어가는 대신 **0건으로 실패 보고**한다.

  patches.json 예:
    [{"file": "plan_legacy/old/battle.rs", "line": 1039,
      "from": "i64 120000", "to": "i64 400000", "note": "threat_range 하한"}]

⚠TypeId 치환은 여기서 하지 않는다 — 런타임(`agent_link::patch_typeids_in_self`)이 내 DLL 이미지에서
  처리하므로 여기서도 하면 이중 적용이 된다.

절차: llvm-ar x → cgu.NN.rcgu.o(비트코드) → llvm-dis → 텍스트 치환 → llvm-as **--module-hash** → llvm-ar rcs
  ⚠`--module-hash` 필수: 없으면 ThinLTO 모듈 해시가 0 이 되어 승격 심볼이 전부 `.llvm.0` 으로 중복돼 링크가 깨진다(2026-09-09 실측 20종).

출력: <out_dir>\\libgame_ai-<hash>.rlib  (파일명은 원본과 동일 — build_inj.ps1 의 $GAI 가 이 폴더를 우선한다)
사용: python MIG\\ailink.py [--sdk <mod-sdk>] [--out <dir>]
"""
import glob
import io
import json
import os
import re
import shutil
import subprocess
import sys
import time

SDK = r'C:\tfm2mods\sdk_058\mod-sdk'
OUT = r'C:\tfm2mods\sdk_058\deps_ailink'
PATCHES = r'C:\tfm2mods\MIG\patches.json'
TOOLCHAIN = 'nightly-2026-05-24'


def bin_dir():
    sysroot = subprocess.check_output(['rustc', '+' + TOOLCHAIN, '--print', 'sysroot']).decode().strip()
    return os.path.join(sysroot, 'lib', 'rustlib', 'x86_64-pc-windows-msvc', 'bin')


def apply_patches(text, patches, hits):
    """`!dbg !N` 이 가리키는 DILocation 의 줄 번호가 명세와 같은 **명령 줄에서만** 치환한다.

    ★`"raw": true` 인 명세는 줄 매칭 없이 **원문 그대로 치환**한다 — `define`/`declare` 처럼
      `!dbg` 가 없는 줄(함수 링키지·호출규약 변경 등)을 건드릴 때 쓴다. 적용 수는 그대로 센다.
    """
    changed = False
    for pi, pc in enumerate(patches):
        if not pc.get('raw'):
            continue
        n = text.count(pc['from'])
        if n:
            text = text.replace(pc['from'], pc['to'])
            hits[pi] += n
            changed = True
    loc = dict(re.findall(r'^!(\d+) = !DILocation\(line: (\d+)', text, re.M))
    lines = text.split('\n')
    for i, ln in enumerate(lines):
        md = re.search(r'!dbg !(\d+)', ln)
        if not md or md.group(1) not in loc:
            continue
        srcline = int(loc[md.group(1)])
        for pi, pc in enumerate(patches):
            if pc.get('raw') or srcline != pc['line'] or pc['from'] not in ln:
                continue
            lines[i] = lines[i].replace(pc['from'], pc['to'])
            hits[pi] += 1
            changed = True
    return ('\n'.join(lines), changed)


def main():
    a = sys.argv[1:]
    sdk = a[a.index('--sdk') + 1] if '--sdk' in a else SDK
    out = a[a.index('--out') + 1] if '--out' in a else OUT
    B = bin_dir()
    rlib = glob.glob(os.path.join(sdk, 'deps', 'libgame_ai-*.rlib'))[0]
    name = os.path.basename(rlib)
    patches = json.load(io.open(PATCHES, encoding='utf-8')) if os.path.exists(PATCHES) else []
    if not patches:
        print('patches.json 이 비었다 — 패치 없이 재조립만 한다(라운드트립 검증용)')
    hits = [0] * len(patches)

    work = os.path.join(out, '_work')
    shutil.rmtree(work, ignore_errors=True)
    os.makedirs(work)
    os.makedirs(out, exist_ok=True)
    t0 = time.time()
    subprocess.check_call([os.path.join(B, 'llvm-ar.exe'), 'x', rlib], cwd=work)
    members = subprocess.check_output([os.path.join(B, 'llvm-ar.exe'), 't', rlib]).decode().split()

    for mem in members:
        if not mem.endswith('.o'):
            continue
        o = os.path.join(work, mem)
        ll = o[:-2] + '.ll'
        subprocess.check_call([os.path.join(B, 'llvm-dis.exe'), o, '-o', ll])
        if patches:
            t = io.open(ll, encoding='utf-8').read()
            t2, changed = apply_patches(t, patches, hits)
            if changed:
                io.open(ll, 'w', encoding='utf-8').write(t2)
        subprocess.check_call([os.path.join(B, 'llvm-as.exe'), '--module-hash', ll, '-o', o])
        os.remove(ll)

    dst = os.path.join(out, name)
    if os.path.exists(dst):
        os.remove(dst)
    subprocess.check_call([os.path.join(B, 'llvm-ar.exe'), 'rcs', dst] + members, cwd=work)
    shutil.rmtree(work, ignore_errors=True)
    print('rlib 재조립 완료 %s (%.0fs)' % (dst, time.time() - t0))

    for pi, pc in enumerate(patches):
        print('   패치 #%d %s:%s  %s -> %s  적용 %d곳  (%s)'
              % (pi, pc.get('file', '?'), pc['line'], pc['from'], pc['to'], hits[pi], pc.get('note', '')))
    if patches and any(h == 0 for h in hits):
        print('⚠ 적용 0건인 패치가 있다 — 소스 줄이 밀렸거나 상수가 접혔다')
        sys.exit(2)


if __name__ == '__main__':
    main()
