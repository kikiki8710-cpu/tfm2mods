# -*- coding: utf-8 -*-
"""AI 조합/추천 인라인 phase 6사이트 → 패치 스텁 바이트 생성기 (Rust 테이블 출력).

설계:
  · 사이트에는 `jmp [rip+0]` (FF 25 00000000 + qword) 14B만 쓴다 → **레지스터 무클로버**.
  · 스텁에서 volatile(rcx,rdx,r8,r9,r10,r11) 저장 → 인자 마샬 → 모드 phase_from 호출 →
    결과 al 을 out 레지스터로 → volatile 복원 → `jmp [rip+0]` 로 합류점 복귀.
  · rsp 는 0x70(16배수) 만큼만 내렸다 되돌리므로 정렬·shadow space 모두 충족.
"""
import sys, json
sys.path.insert(0, r'C:\tfm2mods')
from _it_scan import N
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

R = {'rax': 0, 'rcx': 1, 'rdx': 2, 'rbx': 3, 'rsp': 4, 'rbp': 5, 'rsi': 6, 'rdi': 7,
     'r8': 8, 'r9': 9, 'r10': 10, 'r11': 11, 'r12': 12, 'r13': 13, 'r14': 14, 'r15': 15}
R8 = {'al': 0, 'cl': 1, 'dl': 2, 'bl': 3, 'spl': 4, 'bpl': 5, 'sil': 6, 'dil': 7,
      'r8b': 8, 'r9b': 9, 'r10b': 10, 'r11b': 11, 'r12b': 12, 'r13b': 13, 'r14b': 14, 'r15b': 15}


def modrm(mod, reg, rm):
    return bytes([(mod << 6) | ((reg & 7) << 3) | (rm & 7)])


def mov_rr(dst, src):                      # mov dst64, src64
    d, s = R[dst], R[src]
    return bytes([0x48 | ((s >= 8) << 2) | (d >= 8), 0x89]) + modrm(3, s, d)


def movzx_r8(dst, src8):                   # movzx dst32, src8
    d, s = R[dst], R8[src8]
    rex = 0x40 | ((d >= 8) << 2) | (s >= 8)
    return bytes([rex, 0x0f, 0xb6]) + modrm(3, d, s)


def movzx_mem_rbp(dst, disp):              # movzx dst32, byte [rbp+disp32]
    d = R[dst]
    rex = 0x40 | ((d >= 8) << 2)
    return bytes([rex, 0x0f, 0xb6]) + modrm(2, d, 5) + disp.to_bytes(4, 'little', signed=True)


def shr1(reg):                             # shr reg64, 1
    r = R[reg]
    return bytes([0x48 | (r >= 8), 0xd1]) + modrm(3, 5, r)


def save(reg, off):                        # mov [rsp+off], reg64
    r = R[reg]
    return bytes([0x48 | ((r >= 8) << 2), 0x89]) + modrm(1, r, 4) + b'\x24' + bytes([off])


def load(reg, off):                        # mov reg64, [rsp+off]
    r = R[reg]
    return bytes([0x48 | ((r >= 8) << 2), 0x8b]) + modrm(1, r, 4) + b'\x24' + bytes([off])


def mov_out(dst8):                         # mov dst8, al
    d = R8[dst8]
    pre = bytes([0x40 | (d >= 8)]) if (d >= 8 or d in (4, 5, 6, 7)) else b''
    return pre + b'\x88' + modrm(3, 0, d)


SUBSP = b'\x48\x83\xec\x70'                # sub rsp, 0x70
ADDSP = b'\x48\x83\xc4\x70'                # add rsp, 0x70
CALLRAX = b'\xff\xd0'
VOL = ['rcx', 'rdx', 'r8', 'r9', 'r10', 'r11']
SLOT = {r: 0x28 + 8 * i for i, r in enumerate(VOL)}   # 0x28..0x50
RSLOT = 0x58                               # 결과 임시 슬롯

# ── 사이트 정의 (bo_ai6_053.py 추출 + 수동 확인) ─────────────────────────────
#  patch = 사이트에 쓰는 주소, total/ban/rule = 스텁 진입 시점 소스, ban_doubled = 2*ban 인가
SITES = [
    dict(name='ai_reco1', patch=0x188e206, join=0x188e6b6, total='rcx', ban='r10',
         ban_doubled=True, rule=('reg', 'r12b'), out='al', pre=[]),
    dict(name='ai_reco2', patch=0x188e403, join=0x188e6b6, total='rcx', ban='r10',
         ban_doubled=True, rule=('reg', 'r12b'), out='al', pre=[]),
    dict(name='ai_comp',  patch=0x188f79e, join=0x188f8df, total='rcx', ban='rdx',
         ban_doubled=True, rule=('reg', 'r13b'), out='al', pre=[]),
    dict(name='ai_bb1',   patch=0x18905fc, join=0x189083f, total='rax', ban='rcx',
         ban_doubled=True, rule=('reg', 'r10b'), out='r8b', pre=[]),
    dict(name='ai_bb2',   patch=0x18906e8, join=0x189083f, total='rcx', ban='rsi',
         ban_doubled=True, rule=('mem_rbp', 0x198), out='al', pre=[]),
    # ⚠ 이 사이트만 dl(rule)이 디스패처 직전 `lea rdx,[r13*2]` 로 파괴된다 →
    #    패치를 movzx 앞(0x1891090)으로 당기고, 밀려난 원본 4명령을 스텁이 재현한다.
    dict(name='ai_bb3',   patch=0x1891090, join=0x1891390, total='rcx', ban='r13',
         ban_doubled=False, rule=('reg', 'dl'), out='al',
         pre=[0x1891090 + 0x11, 0x18910a9, 0x18910ad, 0x18910b1]),
]


def gen(site):
    b = bytearray()
    # 0) rule 을 먼저 스크래치(r11)로 — pre 재현이 소스를 파괴할 수 있다.
    kind = site['rule'][0]
    if kind == 'reg':
        rule_first = movzx_r8('r11', site['rule'][1])
    else:
        rule_first = movzx_mem_rbp('r11', site['rule'][1])
    # 1) 프레임 + volatile 저장 (r11 은 rule 을 담은 뒤 저장하면 원본 값이 사라지므로
    #    저장 → rule 로드 순서로 한다.)
    b += SUBSP
    for r in VOL:
        b += save(r, SLOT[r])
    b += rule_first                       # r11 = rule (원본 r11 은 이미 스택에 저장됨)
    b += mov_rr('r10', site['ban'])       # r10 = ban(또는 2*ban)
    b += mov_rr('r9', site['total'])      # r9  = total
    # 2) 밀려난 원본 명령 재현 (있으면) — 스택/비휘발 레지스터 대상이라 순서만 지키면 안전
    for a in site['pre']:
        n = N.read(a, 16)
        md = Cs(CS_ARCH_X86, CS_MODE_64)
        ins = next(md.disasm(bytes(n), 0))
        b += ins.bytes
    # 3) 인자 마샬 → phase_from(rcx=total, rdx=rule, r8=ban)
    b += mov_rr('rcx', 'r9')
    b += mov_rr('rdx', 'r11')
    b += mov_rr('r8', 'r10')
    if site['ban_doubled']:
        b += shr1('r8')
    fn_off = len(b) + 2
    b += b'\x48\xb8' + b'\x00' * 8        # movabs rax, <phase_from>  (설치 시 채움)
    b += CALLRAX
    b += b'\x88\x44\x24' + bytes([RSLOT]) # mov [rsp+0x58], al
    # 4) volatile 복원 → 결과를 out 레지스터로
    for r in VOL:
        b += load(r, SLOT[r])
    b += b'\x0f\xb6\x44\x24' + bytes([RSLOT])  # movzx eax, byte [rsp+0x58]
    if site['out'] != 'al':
        b += mov_out(site['out'])
    b += ADDSP
    join_off = len(b) + 6
    b += b'\xff\x25\x00\x00\x00\x00' + b'\x00' * 8   # jmp [rip+0] → join (설치 시 채움)
    return bytes(b), fn_off, join_off


if __name__ == '__main__':
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    rows = []
    for s in SITES:
        code, fo, jo = gen(s)
        span = s['join'] - s['patch']
        print(f"== {s['name']}: patch={s['patch']:#x} join={s['join']:#x} span={span} "
              f"stub={len(code)}B fn@{fo} join@{jo}  sig={N.hexat(s['patch'], 8)}")
        assert span >= 14, 'span<14'
        for ins in md.disasm(code, 0x1000):
            print(f"     {ins.address:#06x} {ins.bytes.hex():<20} {ins.mnemonic} {ins.op_str}")
        rows.append(dict(name=s['name'], patch=s['patch'], join=s['join'],
                         sig=N.read(s['patch'], 8).hex(), stub=code.hex(),
                         fn_off=fo, join_off=jo))
    json.dump(rows, open(r'C:\tfm2mods\_bo_ai6_stubs.json', 'w'), indent=1)
    # Rust 테이블 출력
    print("\n\n// ── 생성 코드 (hooks.rs 에 붙여넣기) ──")
    print("const AI6: [(usize, usize, &[u8], &[u8], usize, usize); 6] = [")
    for r in rows:
        sig = ', '.join(f'0x{r["sig"][i:i+2]}' for i in range(0, 16, 2))
        stub = ', '.join(f'0x{r["stub"][i:i+2]}' for i in range(0, len(r['stub']), 2))
        print(f"    // {r['name']}")
        print(f"    (0x{r['patch']:x}, 0x{r['join']:x}, &[{sig}], &[{stub}], {r['fn_off']}, {r['join_off']}),")
    print("];")
