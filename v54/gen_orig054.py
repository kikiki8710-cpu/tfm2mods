# -*- coding: utf-8 -*-
"""orig_table.rs 를 0.5.4 기준으로 재생성한다.

원본값은 **0.5.4 exe 에서 직접 읽는다**. 0.5.3 표를 옮겨 적지 않는다 —
값이 바뀐 자리(`pe_minion_add` 4096000000→4096000001 등)를 놓치면
가드가 정상 사이트를 막아버려 노브가 조용히 죽는다.
"""
import io, os, sys, collections

sys.path.insert(0, r'C:\tfm2mods\v54')
import sites as S
import sites2 as S2   # ★직접 호출 사이트도 반드시 포함 — 빼면 검증·가드 사각지대가 된다
from pe2 import load
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

B = 0x140000000
E = load('054')
OUT = r'C:\tfm2mods\tfm2_ai_adjust\src\orig_table.rs'
_c = {}


def ins_at(rva):
    f = E.func_of(rva)
    if not f:
        return None
    if f[0] not in _c:
        _c[f[0]] = {i.address - B: i for i in E.dis(f[0], f[1] - f[0])}
    return _c[f[0]].get(rva)


ent = {}
dup = collections.Counter()
for x in (S.parse() + S2.parse()):
    rva, off, w = x['rva'], x['off'], x['w']
    i = ins_at(rva)
    if i is None or off + w > len(i.bytes):
        continue
    v = int.from_bytes(i.bytes[off:off + w], 'little')
    if rva in ent and ent[rva] != (off, w, v):
        dup[rva] += 1
        continue           # 같은 주소를 다른 폭으로 = 표에 넣지 않는다(가드가 보류 처리)
    ent[rva] = (off, w, v)

rows = sorted((r, ) + v for r, v in ent.items())
body = '\n'.join('    (0x%x, %d, %d, %d),' % r for r in rows)
io.open(OUT, 'w', encoding='utf-8', newline='\n').write(
    '// ⚠자동 생성 파일 — `C:\\tfm2mods\\v54\\gen_orig054.py` 가 **0.5.4 exe 에서 직접** 뽑는다.\n'
    '//   손으로 고치지 말 것.\n'
    '//   용도 = `patch_imm_bytes` 의 \'원본값 일치\' 가드(#26).\n'
    '//   주소가 어긋났는데 prefix 가 우연히 같으면 패치가 \'성공\'으로 계상되던 결함을 막는다.\n'
    '//   형식 = (instruction_rva, imm_off, width, expect_orig). rva 오름차순 — 이분탐색.\n'
    '//   ★0.5.4 기준. 0.5.3 값을 옮겨 적은 것이 아니라 새 exe 에서 읽었다\n'
    '//     (`pe_minion_add` 처럼 값 자체가 바뀐 자리가 있다).\n'
    'pub static EXPECT_ORIG: &[(u32, u8, u8, u64)] = &[\n' + body + '\n];\n')

print('사이트 %d개 기록 (중복주소 제외 %d)' % (len(rows), sum(dup.values())))
print('→ %s' % OUT)
