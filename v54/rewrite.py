# -*- coding: utf-8 -*-
"""detour.rs 를 0.5.4 로 고쳐 쓴다 (0.5.3 원본은 detour_053.rs.bak 로 보존).

고치는 것 3가지:
  ① `base + 0xOLD` → `base + 0xNEW`   (루프 배열 안의 주소도 포함)
  ② prefix 리터럴 → 0.5.4 실바이트    (스택 변위·레지스터가 밀린 자리)
  ③ 미확정 사이트 → `pskip!`          (세되 패치는 안 함 = 오패치 원천 차단)

③이 필요한 이유: `orig_guard_ok` 는 **표에 없는 RVA 를 무조건 통과**시킨다.
그래서 미확정 사이트에 0.5.3 주소를 그냥 남겨두면 prefix 우연일치 하나로
엉뚱한 자리를 덮어쓸 수 있다. 명시적으로 끈다.

⚠루프(`for a in [..]`)는 주소 여러 개가 **prefix 하나를 공유**한다. 0.5.4 에서
   주소마다 prefix 가 갈리면 루프를 유지할 수 없으므로 개별 `p!` 로 펼친다.
"""
import io, os, re, sys, collections, shutil

sys.path.insert(0, r'C:\tfm2mods\v54')
import sites as S
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

D = r'C:\tfm2mods\v54'
SRCDIR = r'C:\tfm2mods\tfm2_ai_adjust\src'
DET = os.path.join(SRCDIR, 'detour.rs')


def load_map():
    m = {}
    first = True
    for ln in io.open(os.path.join(D, 'map_054.tsv'), encoding='utf-8'):
        if first:
            first = False
            continue
        p = ln.rstrip('\n').split('\t')
        m[(int(p[3], 16), int(p[5]), int(p[6]))] = dict(
            new=int(p[4], 16) if p[4] != '-' else 0, newpre=p[8], tier=p[9], why=p[10])
    return m


def pre_lit(hexs):
    return '&[' + ','.join('0x%02x' % b for b in bytes.fromhex(hexs)) + ']'


def main():
    mp = load_map()
    site = S.parse()
    txt = io.open(DET, encoding='utf-8').read()
    lines = txt.split('\n')

    # 루프 단위로 prefix 일관성 검사
    loops = collections.defaultdict(list)
    for x in site:
        if x['loop']:
            loops[(x['line'], x['loop'][0], x['loop'][1], x['off'], x['w'])].append(x)
    split_needed = []
    for k, lst in loops.items():
        pres = set()
        for x in lst:
            r = mp.get((x['rva'], x['off'], x['w']))
            pres.add(r['newpre'] if r and r['tier'].startswith('확정') else None)
        if len(pres) > 1:
            split_needed.append((k, lst, pres))

    print('루프 %d개 중 prefix 가 갈리는 것 %d개' % (len(loops), len(split_needed)))
    for (line, var, addrs, off, w), lst, pres in split_needed:
        print('  L%d  for %s in [%s]  →  prefix %d종' %
              (line, var, ', '.join('%06x' % a for a in addrs), len(pres)))
        for x in lst:
            r = mp.get((x['rva'], x['off'], x['w']))
            print('      %06x → %s  %s  pre=%s' %
                  (x['rva'], ('%06x' % r['new']) if r and r['new'] else '?',
                   r['tier'] if r else '?', (r['newpre'] if r else '-')))
    return mp, site, lines, split_needed


if __name__ == '__main__':
    main()
