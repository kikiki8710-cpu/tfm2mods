#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""rlib2ll.py — SDK rlib 에서 **LLVM IR(.ll)** 를 뽑는다. 크레이트 아무거나.

왜 이게 중요한가:
  TFM2 SDK 의 rlib 들은 `-C embed-bitcode` 로 빌드돼 **아카이브 멤버 자체가 날 LLVM
  비트코드**다(오브젝트 파일이 아니다). `llvm-dis` 만 있으면 **디버그정보가 붙은 원본
  IR** 이 통째로 나온다 — 심볼·구조체 필드명·소스 파일·줄번호까지.
  ⟹ **Ghidra 디컴파일보다 훨씬 정확하다.** exe 에서 못 읽던 것이 여기선 읽힌다.

  실측(2026-09-10): 1~6차 명세에서 "game_core 는 별도 크레이트라 본문이 없다 ⟹
  확인 불가"로 포기한 게 최소 9건, "Arc<dyn> 이라 vtable 전역이 없다 ⟹ 원리적 불가"가
  8건이었는데 **전부 거짓**이었다. 해당 rlib 을 안 뽑았을 뿐이다.

사용:
  python rlib2ll.py --list                     # 비트코드 있는 rlib 목록
  python rlib2ll.py game_view                  # 뽑아서 C:\tfm2mods\_gvbc\v00.ll.. 로
  python rlib2ll.py game_view --out D:\tmp\gv --prefix x
"""
import argparse
import io
import os
import subprocess
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

SDK = r'C:\tfm2mods\sdk_058\mod-sdk\deps'
DIS = (r'C:\Users\jungs\.rustup\toolchains\nightly-2026-05-24-x86_64-pc-windows-msvc'
       r'\lib\rustlib\x86_64-pc-windows-msvc\bin\llvm-dis.exe')
BC_MAGIC = b'BC\xc0\xde'

# 이미 뽑아 둔 것 — 도구(distruct/dienum/fnparts/divtable)가 여기를 본다.
KNOWN = {
    'game_ai':   r'C:\tfm2mods\_gaibc',   # 24개 .ll · 303MB — AI 판단 계층
    'game_core': r'C:\tfm2mods\_gcbc',    # 16개 .ll · 992MB — 시뮬레이션 본체
}
# 관례: 크레이트별 기본 출력 폴더·파일 접두
DEFAULTS = {
    'game_view': (r'C:\tfm2mods\_gvbc', 'v'),
    'engine_core': (r'C:\tfm2mods\_ecbc', 'e'),
    'engine_asset': (r'C:\tfm2mods\_eabc', 'a'),
}


def members(path):
    """ar 아카이브의 멤버를 (이름, 오프셋, 크기)로 훑는다. `ar` 명령 없이 직접 판다."""
    f = io.open(path, 'rb')
    if f.read(8) != b'!<arch>\n':
        raise SystemExit('%s: ar 아카이브가 아니다' % path)
    while True:
        h = f.read(60)
        if len(h) < 60:
            break
        name = h[0:16].decode('ascii', 'replace').strip()
        try:
            size = int(h[48:58].decode('ascii').strip())
        except ValueError:
            break
        off = f.tell()
        yield name, off, size, f
        f.seek(off + size + (size & 1))


def bitcode_members(path):
    """비트코드 멤버만. rustc 는 `/0`, `/68` 같은 숫자 이름으로 넣는다."""
    out = []
    for name, off, size, f in members(path):
        if not (name.startswith('/') and name[1:].isdigit()):
            continue
        f.seek(off)
        if f.read(4) == BC_MAGIC:
            out.append((off, size))
    return out


def find_rlib(crate):
    hits = [x for x in os.listdir(SDK)
            if x.startswith('lib' + crate + '-') and x.endswith('.rlib')]
    if not hits:
        raise SystemExit('%s: SDK 에 없다 — `--list` 로 확인하라' % crate)
    return os.path.join(SDK, sorted(hits)[0])


def cmd_list():
    rows = []
    for fn in sorted(os.listdir(SDK)):
        if not fn.endswith('.rlib'):
            continue
        p = os.path.join(SDK, fn)
        try:
            n = len(bitcode_members(p))
        except SystemExit:
            n = 0
        if n:
            rows.append((os.path.getsize(p), fn, n))
    rows.sort(reverse=True)
    print('%-52s %12s %6s  %s' % ('rlib', '크기B', '멤버', '상태'))
    for sz, fn, n in rows:
        crate = fn[3:fn.rindex('-')]
        st = ('★추출됨 → ' + KNOWN[crate]) if crate in KNOWN else ''
        print('%-52s %12d %6d  %s' % (fn, sz, n, st))
    print()
    print('총 %d개 rlib 에 비트코드가 있다. `python rlib2ll.py <크레이트>` 로 뽑는다.' % len(rows))


def cmd_extract(crate, outdir, prefix, keep_bc):
    rlib = find_rlib(crate)
    if crate in KNOWN and not outdir:
        print('※ `%s` 는 이미 %s 에 뽑혀 있다. 다시 뽑을 이유가 없으면 그걸 써라.'
              % (crate, KNOWN[crate]))
        return
    if not outdir:
        outdir, prefix2 = DEFAULTS.get(crate, (r'C:\tfm2mods\_%sbc' % crate[:3], crate[0]))
        prefix = prefix or prefix2
    prefix = prefix or 'm'
    if not os.path.exists(DIS):
        raise SystemExit('llvm-dis 를 못 찾았다: %s' % DIS)
    if not os.path.isdir(outdir):
        os.makedirs(outdir)

    bcs = bitcode_members(rlib)
    tot = sum(s for _o, s in bcs)
    print('%s → 비트코드 멤버 %d개 · %.0f MB' % (os.path.basename(rlib), len(bcs), tot / 1048576.0))
    f = io.open(rlib, 'rb')
    made = []
    for i, (off, size) in enumerate(bcs):
        f.seek(off)
        bc = os.path.join(outdir, '%s%02d.bc' % (prefix, i))
        io.open(bc, 'wb').write(f.read(size))
        made.append(bc)
    print('  추출 완료. llvm-dis 중…')
    for bc in made:
        ll = bc[:-3] + '.ll'
        r = subprocess.run([DIS, bc, '-o', ll], capture_output=True)
        if r.returncode:
            print('  ⚠실패 %s: %s' % (os.path.basename(bc), r.stderr.decode('utf-8', 'replace')[:120]))
        elif not keep_bc:
            os.remove(bc)
    lls = sorted(x for x in os.listdir(outdir) if x.endswith('.ll'))
    sz = sum(os.path.getsize(os.path.join(outdir, x)) for x in lls)
    print('  → %s  (%d개 .ll · %.0f MB)' % (outdir, len(lls), sz / 1048576.0))
    print()
    print('다음: `grep -n "^define.*<함수명>" %s\\*.ll` 로 본체를 찾고 그 범위만 sed 로 읽어라.' % outdir)
    print('⚠**절대 통째로 읽지 마라.** 파일 하나가 50~75MB 다.')


def main():
    ap = argparse.ArgumentParser(add_help=True)
    ap.add_argument('crate', nargs='?')
    ap.add_argument('--list', action='store_true')
    ap.add_argument('--out')
    ap.add_argument('--prefix')
    ap.add_argument('--keep-bc', action='store_true')
    a = ap.parse_args()
    if a.list or not a.crate:
        cmd_list()
        return
    cmd_extract(a.crate, a.out, a.prefix, a.keep_bc)


if __name__ == '__main__':
    main()
