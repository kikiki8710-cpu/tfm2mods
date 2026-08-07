# -*- coding: utf-8 -*-
"""재작성된 detour.rs 를 **0.5.4 exe 실바이트로 검증**한다.

각 사이트마다 확인:
  ① 그 RVA 가 실제 명령 시작인가 (.pdata 함수 안에서 선형 디스어셈)
  ② 소스에 적힌 prefix 가 그 자리 실바이트와 맞는가  ← 틀리면 런타임에 조용히 skip
  ③ imm_off + width 가 그 명령 길이 안에 들어가는가
  ④ 그 자리의 현재 값(= 새 원본값)

②가 핵심이다. prefix 가 틀리면 크래시는 안 나지만 노브가 죽고, 로그엔
"적용 N/M" 숫자만 줄어 원인 추적이 어렵다.
"""
import io, os, sys, collections

sys.path.insert(0, r'C:\tfm2mods\v54')
import sites as S
import sites2 as S2   # ★직접 호출 사이트도 반드시 포함 — 빼면 검증·가드 사각지대가 된다
from pe2 import load
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

B = 0x140000000
E = load('054')
_c = {}


def insns(fs, fe):
    if fs not in _c:
        _c[fs] = {i.address - B: i for i in E.dis(fs, fe - fs)}
    return _c[fs]


def main():
    bad = collections.Counter()
    rows = []
    site = (S.parse() + S2.parse())
    for x in site:
        rva, off, w = x['rva'], x['off'], x['w']
        f = E.func_of(rva)
        if not f:
            bad['함수없음'] += 1
            rows.append((rva, '함수없음', ''))
            continue
        ins = insns(f[0], f[1]).get(rva)
        if ins is None:
            bad['명령경계아님'] += 1
            rows.append((rva, '명령경계아님', ''))
            continue
        if off + w > len(ins.bytes):
            bad['즉치범위초과'] += 1
            rows.append((rva, '즉치범위초과', '%s (%dB)' % (ins.bytes.hex(), len(ins.bytes))))
            continue
        okpre = any(bytes(ins.bytes[:len(p)]) == bytes(p) for p in x['pre'])
        if not okpre:
            bad['prefix불일치'] += 1
            rows.append((rva, 'prefix불일치',
                         '소스 %s / 실제 %s  (%s %s)'
                         % ('/'.join(bytes(p).hex() for p in x['pre']),
                            ins.bytes.hex(), ins.mnemonic, ins.op_str)))
            continue
        bad['OK'] += 1
        rows.append((rva, 'OK', '%d' % int.from_bytes(ins.bytes[off:off + w], 'little')))

    print('0.5.4 exe 대조 결과 (사이트 %d개)' % len(site))
    for k, v in bad.most_common():
        print('  %-12s %4d' % (k, v))
    ng = [r for r in rows if r[1] != 'OK']
    if ng:
        print('\n문제 사이트:')
        for rva, st, why in ng[:60]:
            print('  %06x  %-12s %s' % (rva, st, why[:96]))
    return rows


if __name__ == '__main__':
    main()
