#!/usr/bin/env python3
"""aiprobe.py — "이 함수가 실제 경기에서 뜨긴 하는가" 를 재기 위한 **카운트 전용 프로브 표** 생성기.

왜: 남은 포팅 범위를 함수 수로 추정하면 틀린다. 죽은 코드가 섞여 있기 때문이다
    (실측 사례: tower_dive 의 is_unreasonable_tower_dive_enemy 서브트리 1,900줄 = version≤1 전용 사장).
    그래서 후보에 **호출 횟수만 세는 훅**을 달아 한 판 돌려 보고, 안 뜨는 것을 범위에서 뺀다.

프로브 스텁은 레지스터·스택·인자를 일절 건드리지 않는다:
    F0 48 FF 05 <disp32>   ; lock inc qword [rip+disp32]   (플래그만 바뀜 = x64 ABI 상 호출자가 보존 안 함)
    <원본 프롤로그 orig_len 바이트>
    FF 25 00 00 00 00 / dq fn+orig_len   ; 원본으로 복귀(레지스터 무손상)
⟹ 인자 개수·부동소수·페어 반환·sret 무엇이든 안전하다. 진입부만 `48 b8 <stub> ff e0` 로 12B 교체.

출력: `judge/probe_tbl.rs` — `(rva, orig_len, prolog[24], 모듈이름)` 표.

사용: python MIG\\aiprobe.py [--n 60] [--out <path>]
"""
import io, json, os, re, struct, sys

try:
    import capstone
except ImportError:
    print('capstone 필요'); sys.exit(2)

EXE = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
MAP = r'C:\tfm2mods\MIG\aimap.json'
GEN = r'C:\tfm2mods\tfm2_ai_adjust\src\judge\gen_fns.rs'
OUT = r'C:\tfm2mods\tfm2_ai_adjust\src\judge\probe_tbl.rs'


def sections(d):
    pe = struct.unpack_from('<I', d, 0x3c)[0]
    nsec = struct.unpack_from('<H', d, pe + 6)[0]
    opt = struct.unpack_from('<H', d, pe + 20)[0]
    out = []
    for i in range(nsec):
        o = pe + 24 + opt + i * 40
        vsz, va, rsz, ra = struct.unpack_from('<IIII', d, o + 8)
        out.append((va, vsz, ra, rsz))
    return out


def prolog_of(d, secs, rva, want=12, cap=24):
    """명령 경계 기준 최소 want 바이트. rip-상대·분기·호출이 섞이면 None."""
    off = None
    for va, vsz, ra, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            off = ra + (rva - va); break
    if off is None:
        return None
    md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
    md.detail = False
    n = 0
    for ins in md.disasm(d[off:off + 64], 0x140000000 + rva):
        m, o = ins.mnemonic, ins.op_str
        if m.startswith('j') or m in ('call', 'ret', 'loop') or 'rip' in o:
            return None
        n += ins.size
        if n >= want:
            return (n, d[off:off + n]) if n <= cap else None
    return None


def main(topn=60, out=OUT):
    d = open(EXE, 'rb').read()
    secs = sections(d)
    m = json.load(io.open(MAP, encoding='utf-8'))
    info, callees = m['info'], {k: set(v) for k, v in m['callees'].items()}
    hooked = set(m['hooked']); own = set(m['own'])
    from collections import defaultdict
    rdeg = defaultdict(int)
    for s, ds in callees.items():
        for x in ds:
            rdeg[x] += 1

    def clos(seeds):
        seen, st = set(seeds), list(seeds)
        while st:
            n = st.pop()
            for c in callees.get(n, ()):
                if c not in seen:
                    seen.add(c); st.append(c)
        return seen

    plan_entry = {r for r in info if rdeg[r] == 0 and info[r]['mod'].startswith('plan_legacy')}
    B = clos(hooked | plan_entry)
    rest = sorted(B - own, key=lambda r: -info[r]['ins'])[:topn]

    rows, skip = [], []
    for r in rest:
        rva = int(r, 16)
        p = prolog_of(d, secs, rva)
        if p is None:
            skip.append((r, info[r]['ins'], info[r]['mod'])); continue
        ln, by = p
        rows.append((r, rva, ln, by, info[r]['mod'], info[r]['ins']))

    L = ['//! probe_tbl.rs — **자동생성**(`MIG\\aiprobe.py`). 손으로 고치지 말 것.',
         '//!   목적 = "이 함수가 실제 경기에서 뜨는가" 만 센다(카운트 전용 · 인자 무관 · 기본 꺼짐 `judge_probe`).',
         '//!   남은 포팅 범위 추정에서 **죽은 코드를 빼기 위한** 계측이다.',
         '#![allow(dead_code)]',
         'pub struct Probe { pub rva: usize, pub len: u8, pub prolog: &\'static [u8], pub name: &\'static str, pub ins: u32 }',
         'pub static PROBES: &[Probe] = &[']
    for r, rva, ln, by, mod, ins in rows:
        L.append('    Probe { rva: %s, len: %d, prolog: &[%s], name: "%s", ins: %d },'
                 % (r, ln, ', '.join('0x%02x' % b for b in by), mod.replace('\\', '/'), ins))
    L.append('];')
    io.open(out, 'w', encoding='utf-8').write('\n'.join(L) + '\n')
    print('프로브 %d개 생성 (건너뜀 %d) -> %s' % (len(rows), len(skip), out))
    for s in skip[:10]:
        print('   skip %s %d ins %s (프롤로그가 분기/rip-상대)' % s)


if __name__ == '__main__':
    a = sys.argv[1:]
    n = int(a[a.index('--n') + 1]) if '--n' in a else 60
    o = a[a.index('--out') + 1] if '--out' in a else OUT
    main(n, o)
