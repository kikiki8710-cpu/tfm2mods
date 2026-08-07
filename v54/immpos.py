# -*- coding: utf-8 -*-
"""★즉치 위치 검증 — 소스의 `imm_off`/`width` 가 **정말 그 명령의 즉치 자리인가**를
capstone 인코딩 정보로 대조한다.

왜 필요한가 (2026-08-05 크래시 실사고):
  0.5.3  `69 c0 90 02 00 00`    imul eax, eax, 0x290   (6B, imm @ off **2**)
  0.5.4  `44 69 c0 90 02 00 00` imul r8d, eax, 0x290   (7B, imm @ off **3**)  ← REX 접두 추가
재배치할 때 0.5.3의 off 2를 그대로 물려썼더니, off 2 = **ModRM 바이트**를 덮어
`imul r10d, dword ptr [rax+...]` 라는 **메모리 참조 명령으로 변질** → null+0x98 AV.

기존 검사가 왜 못 잡았나:
  · `check054.py` = prefix 일치 + `off+w ≤ 명령길이` 만 봤다 (2+4=6 ≤ 7 통과)
  · `orig_table` 가드 = **같은 (off,w) 에서 읽은 값**을 기대값으로 저장했다 → 자기 자신과 비교 = 무력
⟹ 두 검사 모두 "off 가 맞다"를 **전제**했다. 그 전제를 검사하는 게 이 파일이다.
"""
import io, os, sys, collections

sys.path.insert(0, r'C:\tfm2mods\v54')
import sites as S1
import sites2 as S2
import reloc as R
import capstone

B = 0x140000000
E4 = R.E4
_c = {}


def ins_at(rva):
    f = E4.func_of(rva)
    if not f:
        return None
    if f[0] not in _c:
        # detail 이 켜져 있어야 encoding 정보가 나온다
        _c[f[0]] = {i.address - B: i for i in R.insns(E4, f[0], f[1])}
    return _c[f[0]].get(rva)


def main():
    site = S1.parse() + S2.parse()
    bad, nodata, ok = [], 0, 0
    for x in site:
        i = ins_at(x['rva'])
        if i is None:
            nodata += 1
            continue
        enc = getattr(i, 'encoding', None)
        io_, is_ = (getattr(enc, 'imm_offset', 0), getattr(enc, 'imm_size', 0)) if enc else (0, 0)
        do_, ds_ = (getattr(enc, 'disp_offset', 0), getattr(enc, 'disp_size', 0)) if enc else (0, 0)
        if is_ == 0 and ds_ == 0:
            nodata += 1
            continue
        hit = (x['off'] == io_ and x['w'] == is_) or (x['off'] == do_ and x['w'] == ds_)
        # 폭이 다른 건 허용(1B 슬롯에 4B 폭을 안 쓰면 됨) — 위치만 정확하면 OK
        pos_ok = (is_ and x['off'] == io_) or (ds_ and x['off'] == do_)
        if pos_ok:
            ok += 1
        else:
            bad.append((x, i, io_, is_, do_, ds_))

    print('사이트 %d개 — 즉치/변위 위치 대조' % len(site))
    print('  OK %d / 어긋남 %d / 인코딩정보 없음 %d' % (ok, len(bad), nodata))
    if bad:
        print('\n★어긋난 사이트 (이대로 쓰면 명령이 깨진다):')
        for x, i, io_, is_, do_, ds_ in bad:
            print('  %06x  소스 off%d w%d   실제 imm@off%d(%dB) disp@off%d(%dB)   %s %s   %s:%d'
                  % (x['rva'], x['off'], x['w'], io_, is_, do_, ds_,
                     i.mnemonic, i.op_str, x['file'], x['line']))


if __name__ == '__main__':
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
    main()
