# -*- coding: utf-8 -*-
"""dupmatch.py — exe 안의 **중복 인스턴스**에 이름을 물려준다 (dllmatch 의 짝).

## 왜 필요한가 (2026-09-10 실측)
`dllmatch` 는 **일대일 배정**을 강제한다 — 한 심볼이 주소 수십 개를 먹는 사고를 막으려고 넣은 제약이다
(`is_enemy_danger_cell` 하나가 30개를 주장한 적이 있다). 그런데 이 제약은 **정당한 다대일**까지 막는다:
제네릭/모노모피즘 때문에 exe 에는 **같은 코드가 여러 벌** 있는데 rlib 심볼은 한 벌뿐인 경우다.
  실측: `free_dist` 는 exe 에 22개인데 IR 본체는 3개뿐 — `shared_free_dist`(1725B·432ins) 가 **11벌**이다.
  aimap 640개 중 **170개(27%)** 가 원소 2개 이상 군집에 속한다.
⟹ 지문·그래프·줄 어느 축으로도 못 푼다. **"이 둘은 같은 코드다" 를 먼저 확정**하고 이름을 복제해야 한다.

## 방법
함수 본문을 **정규화 디스어셈**해 해시한다. 위치 의존 피연산자를 지우는 것이 핵심이다:
  - 분기·호출의 즉치(타깃) → 지운다. 같은 코드라도 주소가 다르면 변위가 다르다.
  - RIP 상대 메모리 → `m[rip]` 로 뭉갠다(같은 이유).
  - 그 외 레지스터·즉치·메모리(base/index/scale/disp)는 그대로 쓴다 = 구조 오프셋이 보존된다.
같은 해시 군집 안에 **확정 이름이 정확히 1개** 있으면 나머지에 그 이름을 물려준다.
   ⚠확정 이름이 2개 이상이면 **물려주지 않는다** — 같은 코드에 다른 이름이 붙어 있다는 뜻이고,
     그건 기존 배정 중 하나가 틀렸다는 신호다(사람이 봐야 한다). 그 목록은 `conflict` 로 따로 낸다.

## 원리적 한계 (넘을 수 없다)
정규화 후 완전히 같은 코드는 **어떤 내용 기반 방법으로도 구분할 수 없다**.
실측 예: `0xcfac80` 을 두고 `get_input` 과 `get_input_cl` 이 갈렸다.
호출처·문맥 증거가 없으면 여기서 멈추는 게 맞다 — 찍으면 조용히 틀린 이름이 박힌다.

출력: `dupmatch.json` (dllmatch.json 과 **같은 행 포맷** + `via`) → `ghidra_inject.py --extra dupmatch.json`
"""
import io
import json
import os
import sys
import hashlib

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

import capstone

# dllmatch.py 는 맨 끝에서 main() 을 무조건 부른다 → 헬퍼만 쓰려고 사본을 import 한다.
_src = io.open(os.path.join(HERE, 'dllmatch.py'), encoding='utf-8').read().replace('\nmain()\n', '\n')
_shim = os.path.join(HERE, '_dllmatch_lib.py')
if not os.path.exists(_shim) or io.open(_shim, encoding='utf-8').read() != _src:
    io.open(_shim, 'w', encoding='utf-8').write(_src)
import _dllmatch_lib as D  # noqa: E402

MIN_INS = 8          # 명령 8개 미만은 군집이 무의미(어디에나 있는 스텁)
OUT = os.path.join(HERE, 'dupmatch.json')


def norm_hash(cs, data, off, rva, size):
    """위치 의존 피연산자를 지운 정규화 해시 + 명령 수."""
    o = off(rva)
    if o is None or size <= 0:
        return None, 0
    h = hashlib.blake2b(digest_size=16)
    n = 0
    for ins in cs.disasm(data[o:o + size], rva):
        n += 1
        parts = [ins.mnemonic]
        jump = (capstone.x86.X86_GRP_JUMP in ins.groups or capstone.x86.X86_GRP_CALL in ins.groups)
        for op in ins.operands:
            if op.type == capstone.x86.X86_OP_REG:
                parts.append('r%d' % op.reg)
            elif op.type == capstone.x86.X86_OP_IMM:
                parts.append('i' if jump else 'i%d' % op.imm)
            else:
                m = op.mem
                parts.append('m[rip]' if m.base == capstone.x86.X86_REG_RIP
                             else 'm%d,%d,%d,%d' % (m.base, m.index, m.scale, m.disp))
        h.update(('|'.join(parts) + ';').encode())
    return h.hexdigest(), n


def main():
    info = json.load(open(os.path.join(HERE, 'aimap.json'))).get('info', {})
    exe_d, exe_base, exe_secs, exe_exc = D.pe(D.EXE)
    exe_off = D.make_off(exe_secs)
    exe_fr = D.func_ranges(exe_d, exe_secs, exe_exc)
    rows = json.load(io.open(os.path.join(HERE, 'dllmatch.json'), encoding='utf-8'))
    named = {r['rva']: r for r in rows}
    src_of, _lines = D.ir_index()

    cs = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
    cs.detail = True

    clusters, meta = {}, {}
    for rva_s, e in info.items():
        if not isinstance(e, dict):
            continue
        rva = int(rva_s, 16)
        sz = exe_fr.get(rva) or int(e.get('bytes', 0))
        if sz <= 0:
            continue
        hh, n = norm_hash(cs, exe_d, exe_off, rva, sz)
        if not hh or n < MIN_INS:
            continue
        meta[rva] = sz
        clusters.setdefault(hh, []).append(rva)

    multi = {h: v for h, v in clusters.items() if len(v) > 1}
    out, conflict, orphan = [], [], 0
    for h, v in multi.items():
        hits = [r for r in v if r in named]
        miss = [r for r in v if r not in named]
        if not miss:
            continue
        if not hits:
            orphan += 1
            continue
        names = {named[r]['mangled'] for r in hits}
        if len(names) > 1:
            for r in miss:
                conflict.append({'addr': '0x%x' % (0x140000000 + r),
                                 'names': sorted(D.leaf(x) for x in names)})
            continue
        mang = sorted(names)[0]
        for r in miss:
            out.append({
                'addr': '0x%x' % (0x140000000 + r), 'rva': r, 'mangled': mang,
                'name': D.leaf(mang),
                # ★모듈은 **심볼 자신의 것**을 쓴다. aimap 의 모듈 라벨은 인라인 탓에
                #   같은 함수가 여러 모듈로 흩어져 있어(v3_survival_incoming 이 3개 모듈로) 이름이 오도된다.
                'mod': src_of.get(mang, named[hits[0]].get('mod', '?')),
                'jaccard': 1.0, 'contain': 1.0, 'bytes': meta[r],
                'via': '중복군집', 'cluster': len(v),
                'from': '0x%x' % (0x140000000 + hits[0]),
            })

    out.sort(key=lambda r: r['rva'])
    json.dump(out, io.open(OUT, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
    json.dump(conflict, io.open(os.path.join(HERE, 'dupconflict.json'), 'w', encoding='utf-8'),
              ensure_ascii=False, indent=1)

    print('aimap %d개 해시 · 군집 %d개 · 원소 2개 이상 %d개(함수 %d개)'
          % (len(meta), len(clusters), len(multi), sum(len(v) for v in multi.values())))
    print('★물려줄 수 있음 %d개 -> %s' % (len(out), os.path.basename(OUT)))
    print('⚠충돌(같은 코드에 다른 이름 %d개) -> dupconflict.json  ※기존 배정 중 하나가 틀렸다는 신호'
          % len(conflict))
    print('  군집 전체가 미확정 %d군집(물려줄 이름 없음)' % orphan)
    for r in out[:25]:
        print('  %-12s %-22s %7dB 군집%-3d <- %s' % (r['addr'], r['mod'], r['bytes'], r['cluster'], r['name']))
    return 0


sys.exit(main())
