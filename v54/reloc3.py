# -*- coding: utf-8 -*-
"""★전 패치 사이트(388개) 재배치 + **새 prefix 산출**.

reloc.py 의 짝짓기 엔진을 detour.rs 실제 호출부에 적용한다. orig_table 에 없는
사이트까지 포함하고, 각 사이트의 **prefix 를 0.5.4 실바이트로 다시 계산**한다.

prefix 를 다시 계산해야 하는 이유: 0.5.4 는 스택 프레임이 통째로 밀렸다
(`[rbp+0x4f8]` → `[rbp+0x528]`). prefix 에 변위가 들어간 사이트는 RVA 만 고치면
`patch_imm_bytes` 의 opcode 검증에서 걸려 **조용히 skip** 된다. 크래시는 안 나지만
노브가 죽고, 로그에도 "적용 N/M" 숫자만 줄어 원인을 찾기 어렵다.

판정:
  확정      = 시그니처(즉치 가린 실바이트) 동일 + 함수 내 순번 일치
  확정(완화) = 니모닉·길이·값 동일 + 순번 일치 (레지스터/변위만 바뀜 → prefix 교체 필요)
  그 외     = 미확정. **추측으로 채우지 않는다.**

산출: map_054.tsv
"""
import io, os, sys, collections

sys.path.insert(0, r'C:\tfm2mods\v54')
import reloc as R
import sites as S

D = r'C:\tfm2mods\v54'
B = 0x140000000
E3, E4 = R.E3, R.E4


def main():
    site = S.parse()
    byfn = collections.defaultdict(list)
    orphan = []
    for x in site:
        f = E3.func_of(x['rva'])
        (byfn[f] if f else orphan).append(x)

    out = []
    for (fs, fe), lst in sorted(byfn.items()):
        pr = R.pair_fn(fs, fe)
        i3 = {i.address - B: i for i in R.insns(E3, fs, fe)}
        if not pr:
            for x in lst:
                out.append((x, 0, None, '짝없음', 'src=%s' % R.SRC3.get(fs, '?')))
            continue
        bs, be, ratio = pr
        i4 = R.insns(E4, bs, be)

        for x in lst:
            rva, off, w = x['rva'], x['off'], x['w']
            ins = i3.get(rva)
            if ins is None:
                out.append((x, 0, None, '명령경계불일치', '%06x 가 명령 시작이 아님' % rva))
                continue
            if off + w > len(ins.bytes):
                out.append((x, 0, None, '즉치범위초과', 'off%d+%d > 명령길이 %d' % (off, w, len(ins.bytes))))
                continue
            orig = R.val(ins, off, w)

            # 1차: 시그니처 동일
            key = R.sig(ins, off, w)
            a3 = [a for a, y in sorted(i3.items())
                  if off + w <= len(y.bytes) and R.sig(y, off, w) == key and R.val(y, off, w) == orig]
            a4 = [y.address - B for y in i4
                  if off + w <= len(y.bytes) and R.sig(y, off, w) == key and R.val(y, off, w) == orig]
            tier = None
            if a4 and len(a3) == len(a4) and rva in a3:
                tier, cand3, cand4 = '확정', a3, a4
            else:
                # 2차: 니모닉·길이·값 동일(레지스터/변위 변경 허용)
                def k(y):
                    return (y.mnemonic, len(y.bytes))
                b3 = [a for a, y in sorted(i3.items())
                      if k(y) == k(ins) and off + w <= len(y.bytes) and R.val(y, off, w) == orig]
                b4 = [y.address - B for y in i4
                      if k(y) == k(ins) and off + w <= len(y.bytes) and R.val(y, off, w) == orig]
                if b4 and len(b3) == len(b4) and rva in b3:
                    tier, cand3, cand4 = '확정(완화)', b3, b4
                else:
                    out.append((x, 0, None,
                                '개수불일치' if (a4 or b4) else '소멸',
                                '시그 %d→%d / 완화 %d→%d (골격 %.0f%%)'
                                % (len(a3), len(a4), len(b3), len(b4), ratio * 100)))
                    continue

            idx = cand3.index(rva)
            tgt = cand4[idx]
            t = next(y for y in i4 if y.address - B == tgt)
            # 새 prefix = 054 실바이트에서 **기존 prefix 와 같은 길이**만큼
            plen = max(len(p) for p in x['pre'])
            newpre = bytes(t.bytes[:plen])
            same = any(bytes(p) == newpre for p in x['pre'])
            out.append((x, tgt, newpre, tier,
                        '%d/%d번째, 골격 %.0f%%, prefix %s' % (idx + 1, len(cand3), ratio * 100,
                                                            '그대로' if same else '★교체필요')))

    for x in orphan:
        out.append((x, 0, None, '함수미상', '.pdata 에 없음'))

    with io.open(os.path.join(D, 'map_054.tsv'), 'w', encoding='utf-8', newline='') as fo:
        fo.write('file\tline\tkind\trva053\trva054\timm_off\twidth\told_pre\tnew_pre\t판정\t근거\n')
        for x, tgt, newpre, tier, why in sorted(out, key=lambda r: (r[0]['file'], r[0]['line'])):
            fo.write('%s\t%d\t%s\t%06x\t%s\t%d\t%d\t%s\t%s\t%s\t%s\n'
                     % (x['file'], x['line'], x['kind'], x['rva'],
                        ('%06x' % tgt) if tgt else '-', x['off'], x['w'],
                        '/'.join(bytes(p).hex() for p in x['pre']),
                        newpre.hex() if newpre else '-', tier, why))

    c = collections.Counter(r[3] for r in out)
    print('전 패치 사이트 %d개' % len(out))
    for k, v in c.most_common():
        print('  %-14s %4d' % (k, v))
    need = [r for r in out if r[2] and not any(bytes(p) == r[2] for p in r[0]['pre'])]
    print('\n★prefix 교체가 필요한 사이트 %d개' % len(need))
    print('→ map_054.tsv')


if __name__ == '__main__':
    main()
