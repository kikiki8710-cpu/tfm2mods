# -*- coding: utf-8 -*-
"""드레인(0x1c55300) 잔여 phase 인라인 복제본 5곳 → 스텁 생성 (전량 수동 확정).

앞선 훅 J 는 cur(0x1c6605d)/next(0x1c66374) 2곳만 잡았다. 유저 실측에서 "내 2번째 픽인데
상대 4번째 밴 슬롯 하이라이트가 켜져 있다"가 남은 건, 밴/픽 슬롯 하이라이트를 그리는
**다른 복제본들**이 여전히 바닐라이기 때문. 각 복제본을 디스어셈으로 하나씩 확정:

  A 0x1c5a0b2  jmp rsi   total=r9   2*ban=r10 rule=r8b  out=dl  join=0x1c5a288
      ⚠ arm/폴백이 `sub r9,r10`(=k) 로 r9 를 파괴하고, 완료 경로(0x1c5a274~) 는
        r8+=2 / rcx / rdx 를 쓰는 **별개 계산**이라 join 을 0x1c5a288(cmp dl,0xff)로 잡는다.
        out=dl 은 "1(픽/밴 유효) / 0(범위밖) / 2↑" 가 아니라 **테이블값<2 판정 결과**라
        원본 의미 = `phase != 0xff`. ⇒ 이 사이트는 dl 규약이 phase 가 아니라 **불리언**이다.
  B 0x1c5a5b9  jmp rax   total=rcx  2*ban=rdx rule=al   out=al  join=0x1c5a90b   (표준 phase)
  C 0x1c5a9b1  jmp r9    total=rsi  2*ban=rcx rule=r10b out=r8b join=0x1c5aa67   (표준 phase)
  D 0x1c5aa99  jmp r10   ⛔ **패치 제외** — 루프 본문 안이고 인덱스가 rsi+rdx 로 매 회전
        바뀐다(같은 phase 연속 개수 카운터). 진입 14B 확보도 불가(앞 명령이 lea rdi,[rsi+rdx]).
  E 0x1c6fb16  jmp r11   total=rax  2*ban=r10 rule=r11b out=?  join=0x1c70508
        ⚠ arm 이 `mov rsi,[rbp+0x1308]` 부작용 보유.

A 는 불리언 규약이라 별도 래퍼(phase!=0xff)를, B/C/E 는 phase 직결을 쓴다.
"""
import sys, json
sys.path.insert(0, r'C:\tfm2mods')
from _it_scan import N, BASE
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_OP_IMM

md = Cs(CS_ARCH_X86, CS_MODE_64)
md.detail = True
R = {'rax': 0, 'rcx': 1, 'rdx': 2, 'rbx': 3, 'rsp': 4, 'rbp': 5, 'rsi': 6, 'rdi': 7,
     'r8': 8, 'r9': 9, 'r10': 10, 'r11': 11, 'r12': 12, 'r13': 13, 'r14': 14, 'r15': 15}
R8 = {'al': 0, 'cl': 1, 'dl': 2, 'bl': 3, 'sil': 6, 'dil': 7, 'r8b': 8, 'r9b': 9,
      'r10b': 10, 'r11b': 11, 'r12b': 12, 'r13b': 13, 'r14b': 14, 'r15b': 15}
VOL = ['rcx', 'rdx', 'r8', 'r9', 'r10', 'r11']
SLOT = {r: 0x28 + 8 * i for i, r in enumerate(VOL)}
RS = 0x60
SUBSP, ADDSP, CALLRAX = b'\x48\x83\xec\x70', b'\x48\x83\xc4\x70', b'\xff\xd0'


def modrm(mod, reg, rm):
    return bytes([(mod << 6) | ((reg & 7) << 3) | (rm & 7)])


def mov_rr(d, s):
    a, b = R[d], R[s]
    return bytes([0x48 | ((b >= 8) << 2) | (a >= 8), 0x89]) + modrm(3, b, a)


def movzx_r11_from8(src8):
    s = R8[src8]
    return bytes([0x44 | ((s >= 8) << 0) | 0x00 if False else (0x44 | (1 if s >= 8 else 0)),
                  0x0f, 0xb6]) + modrm(3, 3, s)      # dst = r11d (reg=011 + REX.R)


def save(r, off):
    x = R[r]
    return bytes([0x48 | ((x >= 8) << 2), 0x89]) + modrm(1, x, 4) + b'\x24' + bytes([off])


def load(r, off):
    x = R[r]
    return bytes([0x48 | ((x >= 8) << 2), 0x8b]) + modrm(1, x, 4) + b'\x24' + bytes([off])


def mov8_from_stack(out8):
    d = R8[out8]
    rex = 0x40 | ((d >= 8) << 2)
    return bytes([rex, 0x8a]) + modrm(1, d, 4) + b'\x24' + bytes([RS])


def shr1(r):
    x = R[r]
    return bytes([0x48 | (x >= 8), 0xd1]) + modrm(3, 5, x)


def build(total, dbl, rule8, out8, post=b''):
    b = bytearray()
    b += SUBSP
    b += save('rax', 0x20)
    for r in VOL:
        b += save(r, SLOT[r])
    b += movzx_r11_from8(rule8)
    b += mov_rr('r10', dbl)
    b += mov_rr('r9', total)
    b += mov_rr('rcx', 'r9')
    b += mov_rr('rdx', 'r11')
    b += mov_rr('r8', 'r10')
    b += shr1('r8')
    fo = len(b) + 2
    b += b'\x48\xb8' + b'\x00' * 8
    b += CALLRAX
    b += b'\x88\x44\x24' + bytes([RS])
    for r in VOL:
        b += load(r, SLOT[r])
    b += load('rax', 0x20)
    b += post
    b += mov8_from_stack(out8)
    b += ADDSP
    jo = len(b) + 6
    b += b'\xff\x25\x00\x00\x00\x00' + b'\x00' * 8
    return bytes(b), fo, jo


# kind: 0 = phase 직결, 1 = 불리언(phase != 0xff)
SITES = [
    dict(name='drainA_valid', patch=0x1c5a0b2, join=0x1c5a288, total='r9', dbl='r10',
         rule='r8b', out='dl', post=b'', kind=1),
    dict(name='drainB_phase', patch=0x1c5a5b9, join=0x1c5a90b, total='rcx', dbl='rdx',
         rule='al', out='al', post=b'', kind=0),
    dict(name='drainC_phase', patch=0x1c5a9b1, join=0x1c5aa67, total='rsi', dbl='rcx',
         rule='r10b', out='r8b', post=b'', kind=0),
    dict(name='drainE_phase', patch=0x1c6fb16, join=0x1c70508, total='rax', dbl='r10',
         rule='r11b', out='r11b',
         post=bytes([0x48, 0x8b, 0xb5, 0x08, 0x13, 0x00, 0x00]), kind=0),
]


def verify(patch, join):
    s, e = N.func_of(0x1c55300)
    bad = []
    boundary = False
    for ins in md.disasm(N.read(s, e - s), BASE + s):
        a = ins.address - BASE
        if a == join:
            boundary = True
        for op in ins.operands:
            if op.type == X86_OP_IMM and ins.mnemonic[0] == 'j':
                t = op.imm - BASE
                if patch < t < patch + 14 and not (patch <= a < join):
                    bad.append((hex(a), hex(t)))
    return bad, boundary


if __name__ == '__main__':
    rows = []
    for s in SITES:
        code, fo, jo = build(s['total'], s['dbl'], s['rule'], s['out'], s['post'])
        bad, boundary = verify(s['patch'], s['join'])
        ok = True
        for i in md.disasm(code, 0x1000):
            if i.mnemonic in ('jo', 'and', '(bad)'):
                ok = False
        print(f"== {s['name']} patch={s['patch']:#x} join={s['join']:#x} "
              f"span={s['join']-s['patch']} stub={len(code)}B kind={s['kind']} "
              f"inbound={bad} join_boundary={boundary} enc_ok={ok}")
        for i in md.disasm(code, 0x1000):
            print(f"     {i.bytes.hex():<20} {i.mnemonic} {i.op_str}")
        assert not bad and boundary and ok, s['name']
        rows.append(dict(name=s['name'], patch=s['patch'], join=s['join'], stub=code.hex(),
                         fn_off=fo, join_off=jo, sig=N.read(s['patch'], 8).hex(),
                         kind=s['kind']))
    json.dump(rows, open(r'C:\tfm2mods\_bo_drain5_stubs.json', 'w'), indent=1)
    print("\nsaved", len(rows))
