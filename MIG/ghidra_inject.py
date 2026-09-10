#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""ghidra_inject.py - `dllmatch.json`(2안) 의 이름·시그니처를 **Ghidra 에 주입**한다.

왜:
  exe 가 stripped 라 Ghidra 는 전부 `FUN_140xxxxxx` 로 보여준다. 이름이 없으니
  ①어느 함수인지 매번 손으로 찾고 ②시그니처를 몰라 디컴이 틀린다.
  실사고(2026-09-10): Ghidra 가 가상호출의 **3번째 인자를 놓쳤고**(2개로 표시)
  그대로 구현했다가 r8 이 쓰레기라 게임이 AV 로 죽었다. 이름/인자만 있었으면 안 났다.

전 세대(`ghidra_syms.py` = 1안, 패닉줄 조인)와의 차이:
  1안은 **패닉 사이트가 없는 함수 964개**를 원리적으로 못 잡아 커버리지가 막혔다.
  2안(`dllmatch.py`)은 기계어 지문 + 호출그래프라 그 한계가 없다.

이름 규칙: `<소스모듈>__<함수이름>`. 충돌하면 `_2`, `_3` 을 붙인다.
안전장치:
  - 시작 시 **exe 프롤로그 바이트를 Ghidra 가 보는 것과 대조**한다.
    포트를 잘못 잡으면 **구버전 프로그램에 이름을 박고 성공했다고 보고**한다.
  - `--dry` 로 무엇을 바꿀지 먼저 본다.
  - 이미 같은 이름이면 건너뛴다(멱등).
"""
import argparse
import io
import json
import os
import re
import struct
import sys
import time
import urllib.parse
import urllib.request

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

HERE = os.path.dirname(os.path.abspath(__file__))
EXE = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
SRC = os.path.join(HERE, 'dllmatch.json')
IRDIRS = [r'C:\tfm2mods\_gaibc', r'C:\tfm2mods\_gcbc']

SUB_RE = re.compile(r'!DISubprogram\(name: "([^"]+)", linkageName: "([^"]+)"')
DEF_RE = re.compile(r'^define[^@\n]*@([A-Za-z0-9_.$]+)\(([^\n]*?)\)\s*(?:unnamed_addr|#)', re.M)


class Srv(object):
    def __init__(self, port, timeout=120):
        self.url = 'http://127.0.0.1:%d/' % port
        self.timeout = timeout

    def post(self, ep, data):
        body = urllib.parse.urlencode(data).encode()
        req = urllib.request.Request(self.url + ep, data=body)
        for attempt in (0, 1):
            try:
                with urllib.request.urlopen(req, timeout=self.timeout) as r:
                    return r.read().decode('utf-8', 'replace').strip()
            except Exception as ex:
                if attempt:
                    return 'ERR: %s' % ex
                time.sleep(1.0)

    def get(self, ep, **p):
        u = self.url + ep + '?' + urllib.parse.urlencode(p)
        try:
            with urllib.request.urlopen(u, timeout=self.timeout) as r:
                return r.read().decode('utf-8', 'replace')
        except Exception as ex:
            return 'ERR: %s' % ex


def exe_prologue(rva, n=16):
    d = open(EXE, 'rb').read(0x400)
    full = open(EXE, 'rb')
    off = struct.unpack_from('<I', d, 0x3c)[0]
    nsec = struct.unpack_from('<H', d, off + 6)[0]
    optsz = struct.unpack_from('<H', d, off + 20)[0]
    full.seek(0)
    data = full.read()
    for i in range(nsec):
        vsz, va, rsz, ra = struct.unpack_from('<IIII', data, off + 24 + optsz + i * 40 + 8)
        if va <= rva < va + max(vsz, rsz):
            o = ra + rva - va
            return data[o:o + n]
    return b''


def ir_args():
    """망글심볼 -> IR 인자 목록(문자열). 프로토타입 인자 개수의 근거."""
    out = {}
    for p in IRDIRS:
        import glob
        for f in sorted(glob.glob(os.path.join(p, '*.ll'))):
            t = io.open(f, encoding='utf-8', errors='ignore').read()
            known = {m.group(2) for m in SUB_RE.finditer(t)}
            for m in DEF_RE.finditer(t):
                if m.group(1) in known:
                    out[m.group(1)] = m.group(2)
            del t
    return out


def split_args(s):
    o, d, cur = [], 0, ''
    for ch in s:
        if ch in '([{':
            d += 1
        elif ch in ')]}':
            d -= 1
        if ch == ',' and d == 0:
            o.append(cur.strip())
            cur = ''
        else:
            cur += ch
    if cur.strip():
        o.append(cur.strip())
    return o


def c_type(a):
    a = a.strip()
    for pre, c in (('ptr', 'void *'), ('i64', 'longlong'), ('i32', 'int'),
                   ('i16', 'short'), ('i8', 'char'), ('i1', 'char'),
                   ('double', 'double'), ('float', 'float')):
        if a.startswith(pre):
            return c
    return 'longlong'


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--port', type=int, default=8081, help='Ghidra MCP 포트 (0.5.8=8081)')
    ap.add_argument('--minj', type=float, default=0.60)
    ap.add_argument('--dry', action='store_true')
    ap.add_argument('--limit', type=int, default=0)
    # ★중복 인스턴스(dupmatch.json)를 함께 주입한다. **합쳐서** 넣어야 하는 이유:
    #   이름 중복 번호(`base_2`, `base_3`)는 이 실행 안의 `seen` 으로만 매겨진다.
    #   dup 만 따로 넣으면 원본과 같은 이름을 1번으로 달아 Ghidra 에서 충돌한다.
    ap.add_argument('--extra', default='', help='추가 행 파일들(dllmatch 와 같은 포맷), 쉼표로 여러 개. 예: dupmatch.json,manual.json')
    a = ap.parse_args()

    s = Srv(a.port)
    rows = [r for r in json.load(io.open(SRC, encoding='utf-8')) if r['jaccard'] >= a.minj]
    for one in [x.strip() for x in a.extra.split(',') if x.strip()]:
        xp = one if os.path.isabs(one) else os.path.join(HERE, one)
        ex = [r for r in json.load(io.open(xp, encoding='utf-8')) if r.get('jaccard', 0) >= a.minj]
        have = {r['rva'] for r in rows}
        # ★이미 확정된 주소는 건드리지 않는다. 앞 파일이 우선 = **수기(manual)보다 자동(dllmatch)이 먼저**가
        #   아니라, 인자 순서대로다. 수기 확정을 우선하려면 --extra manual.json,dupmatch.json 순으로 준다.
        ex = [r for r in ex if r['rva'] not in have]
        print('추가 행 %d개 병합 (%s)' % (len(ex), os.path.basename(xp)))
        rows += ex

    # ── ★버전 대조: 엉뚱한 포트면 구버전에 이름을 박는다
    probe = rows[0]
    dis = s.get('disassemble_function', address=probe['addr'])
    want = exe_prologue(probe['rva'])
    if dis.startswith('ERR'):
        print('Ghidra 서버(%s)에 못 붙었다: %s' % (s.url, dis[:120]))
        return 2
    # ⚠`disassemble_function` 은 **원시 바이트를 주지 않는다**(`140d3fe50: PUSH R15` 형식).
    #   바이트를 정규식으로 찾으려던 첫 판은 아무것도 못 찾고 **조용히 통과**했다 -
    #   안전장치가 헛돌면 없느니만 못하다. 그래서 **니모닉 순서**로 대조한다.
    import capstone as _cs
    md = _cs.Cs(_cs.CS_ARCH_X86, _cs.CS_MODE_64)
    exp = [i.mnemonic.upper() for i in md.disasm(want, probe['rva'])][:6]
    got = re.findall(r'^\s*[0-9a-f]+:\s+([A-Z][A-Z0-9]*)', dis, re.M)[:6]
    if not exp or not got:
        print('⚠프롤로그를 읽지 못했다(ghidra=%r) - 중단.' % dis[:100])
        return 2
    n = min(len(exp), len(got))
    if exp[:n] != got[:n]:
        print('⚠프롤로그 불일치 - **다른 프로그램/버전에 붙어 있다**. 중단.')
        print('   ghidra=%s / exe=%s' % (got[:n], exp[:n]))
        return 2
    print('버전 대조 OK (%s @ %s · %s)' % (probe['addr'], s.url, ' '.join(got[:4])))

    args_of = ir_args()
    seen, work = {}, []
    for r in sorted(rows, key=lambda x: x['rva']):
        base = ('%s__%s' % (r.get('mod', '?'), r['name']))
        base = re.sub(r'[^A-Za-z0-9_]', '_', base)[:180]
        n = seen.get(base, 0) + 1
        seen[base] = n
        nm = base if n == 1 else '%s_%d' % (base, n)
        ar = args_of.get(r['mangled'])
        if ar is None:
            proto = None
        else:
            cs_ = [c_type(x) for x in split_args(ar)]
            ret = 'void' if any('sret' in x for x in split_args(ar)) else 'longlong'
            proto = '%s %s(%s)' % (ret, nm,
                                   ', '.join('%s a%d' % (c, i) for i, c in enumerate(cs_)) or 'void')
        work.append((r['addr'], nm, proto, r['jaccard'], r.get('via', '지문')))
    if a.limit:
        work = work[:a.limit]

    npro = sum(1 for w in work if w[2])
    print('주입 대상 %d개 (j>=%.2f) · 그중 프로토타입 있는 것 %d' % (len(work), a.minj, npro))
    if a.dry:
        for addr, nm, proto, j, via in work[:25]:
            print('  %-12s %-52s j=%.2f %-9s %s' % (addr, nm[:52], j, via, (proto or '(프로토 없음)')[:60]))
        print('  ... (--dry 라 실제로 바꾸지 않았다)')
        return 0

    ok = pon = fail = skip = 0
    t0 = time.time()
    for i, (addr, nm, proto, j, via) in enumerate(work):
        r1 = s.post('rename_function_by_address', {'function_address': addr, 'new_name': nm})
        if 'renamed successfully' in r1 or 'already' in r1.lower():
            ok += 1
        else:
            fail += 1
            if fail <= 10:
                print('  FAIL %s %s | %s' % (addr, nm[:40], r1[:120]))
            continue
        if proto:
            r2 = s.post('set_function_prototype', {'function_address': addr, 'prototype': proto})
            if r2.startswith('Function prototype set successfully'):
                pon += 1
        else:
            skip += 1
        if (i + 1) % 100 == 0:
            print('  ..%d/%d  %.0fs' % (i + 1, len(work), time.time() - t0), flush=True)
    print('이름 %d개 적용 · 프로토타입 %d개 적용 · 실패 %d · 프로토없음 %d · %.0fs'
          % (ok, pon, fail, skip, time.time() - t0))
    return 0


sys.exit(main())
