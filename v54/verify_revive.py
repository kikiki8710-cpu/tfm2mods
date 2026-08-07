# -*- coding: utf-8 -*-
"""★pskip 재조사로 되살린 사이트를 **0.5.4 실바이트로 최종 확인**한다.

각 행 = (053rva, 054rva, prefix, imm_off, width, 기대원본값).
확인 항목: ①054 rva 가 명령 시작인가 ②prefix 가 실바이트와 맞는가
          ③imm 이 명령 길이 안인가 ④그 자리 값이 기대 원본값인가
④까지 맞아야 `orig_table.rs` 가드를 통과해 실제로 패치된다.
"""
import io, sys
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load
if __name__ == '__main__':
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
B = 0x140000000

ROWS = [
  # 053,      054,      prefix,                              off, w, orig,      knob
  (0xcd0e38, 0xcaf845, (0x48,0x83,0xf8),                      3, 1, 100,        'pnex A(mode>=2 분기)'),
  (0xcd0e38, 0xcaf855, (0x48,0x83,0xf8),                      3, 1, 100,        'pnex B(mode<2 분기)'),
  (0xcd0e8e, 0xcaf8a0, (0xbe,),                               1, 4, 1000,       'pna2'),
  (0xcd0db9, 0xcaf6f7, (0x48,0x6b,0x8d,0xa0,0x06,0x00,0x00),  7, 1, 0x78,       'pks #1(field0)'),
  (0xcd0ddc, 0xcaf78f, (0x48,0x6b,0x8d,0x30,0x06,0x00,0x00),  7, 1, 0x78,       'pks #2(field0x10)'),
  (0xccce81, 0xcab773, (0x44,0x69,0xc0),                      3, 4, 656,        'pfar #1'),
  (0xcccef2, 0xcab817, (0x44,0x69,0xc0),                      3, 4, 656,        'pfar #2'),
  (0xccd76e, 0xcac08c, (0x49,0xbe),                           2, 8, 0x53d1ac100,'pflt(3→1 통합)'),
  (0xc8689a, 0xe58b3a, (0x49,0x69,0xc5),                      3, 4, 260000,     'wd #1'),
  (0xc868a1, 0xe58b41, (0x4d,0x69,0xcc),                      3, 4, 260000,     'wd #2'),
  (0,        0xe59569, (0x48,0x69,0xc3),                      3, 4, 260000,     'wd #3(0.5.4 신규경로)'),
  (0,        0xe59570, (0x4c,0x69,0xcf),                      3, 4, 260000,     'wd #4(0.5.4 신규경로)'),
  (0xdee222, 0xdadb55, (0x49,0x81,0xf8),                      3, 4, 390625,     'nx_cull_dist19(3→1)'),
  (0xd5f9fa, 0xead8fa, (0x48,0xb9),                           2, 8, 0x53d1ac101,'gusr'),
  (0xcb3efd, 0xdb869a, (0xb8,),                               1, 4, 30,         'lead #1(기본)'),
  (0xcb3efd, 0xdb8716, (0xb8,),                               1, 4, 30,         'lead #2(mode3 분기)'),
  (0xc7b4a5, 0xca8132, (0x48,0xb9),                           2, 8, 0x35a4e9001,'mvt'),
  (0xc7d4f0, 0xd8edbb, (0x48,0x81,0xbd,0xb8,0x00,0x00,0x00),  7, 4, 950,        'm0ng'),
  (0,        0xf3d658, (0x48,0x83,0xc3),                      3, 1, 0x78,       'mvm(공용 아웃라인·범위확대 주의)'),
]


def main():
    E = load('054')
    for r53, r54, pre, off, w, orig, name in ROWS:
        f = E.func_of(r54)
        st = []
        if not f:
            print('%-34s %06x  .pdata 함수 없음' % (name, r54)); continue
        ins = {i.address - B: i for i in E.dis(f[0], f[1] - f[0])}.get(r54)
        if ins is None:
            print('%-34s %06x  ⚠명령 경계 아님' % (name, r54)); continue
        b = bytes(ins.bytes)
        st.append('prefix ' + ('OK' if b.startswith(bytes(pre)) else '✗실제=%s' % b[:len(pre)].hex()))
        if off + w > len(b):
            st.append('✗imm 범위초과(명령 %dB)' % len(b))
            val = None
        else:
            val = int.from_bytes(b[off:off + w], 'little')
            st.append('값 %d %s' % (val, 'OK' if val == orig else '✗기대 %d' % orig))
        print('%-34s 053 %-7s→ 054 %06x  %-22s %-6s %s   [%s]'
              % (name, ('%06x' % r53) if r53 else '-', r54, b.hex(), ins.mnemonic, ins.op_str[:34], ' / '.join(st)))


if __name__ == '__main__':
    main()
