# -*- coding: utf-8 -*-
"""★형태 무관 주소 스윕 — 남은 패치 사이트를 **호출 형태를 가정하지 않고** 찾아 옮긴다.

왜 필요했나: 패치 호출부가 최소 3가지 형태로 존재한다.
  ① `p!` / `pany!` 매크로            → sites.py
  ② `patch_imm_bytes(...)` 직접 호출  → sites2.py
  ③ **`(prefix, off)` 후보쌍 배열 순회** → 어느 파서도 못 잡음
     예) `const LEA32: [(&[u8],usize);6] = [...]` 를 `for (pre,off) in LEA32.iter()` 로 돌리는 형태
그래서 인게임 실측이 **applied=569/824** 로 나왔다(제가 본 것만 적용, 못 본 255개는 전부 실패).

이 스크립트는 형태를 안 본다. 대신:
  · 원본 백업(`detour.rs.053bak`)에서 **0.5.3 명령 시작인 hex 리터럴**을 전부 뽑고
  · 현재 파일에 **아직 그대로 남아 있는 것**만 골라
  · 짝짓기 엔진으로 옮긴다.
③ 형태는 prefix 가 이미 다중 후보라 **주소만 고치면 된다**(레지스터가 바뀌어도 후보 중 하나가 맞는다).

⚠주소만 고치는 게 안전한 이유: `patch_imm_bytes` 는 prefix 불일치 시 **아무것도 쓰지 않고 false**를
   돌려준다. 게다가 원본값 가드까지 있다. 즉 최악이라도 "안 붙음"이지 오패치가 아니다.
"""
import io, os, re, sys, collections

sys.path.insert(0, r'C:\tfm2mods\v54')
import reloc as R
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRCDIR = r'C:\tfm2mods\tfm2_ai_adjust\src'
B = 0x140000000
E3, E4 = R.E3, R.E4
HEX = re.compile(r'0x([0-9a-fA-F]{5,7})')


def text_ok(E, rva):
    """그 RVA 가 .pdata 함수 안의 **명령 시작**인가."""
    f = E.func_of(rva)
    if not f:
        return None
    for i in R.insns(E, f[0], f[1]):
        a = i.address - B
        if a == rva:
            return f
        if a > rva:
            return None
    return None


def candidates(path):
    """패치와 관련된 줄에서만 주소 후보를 뽑는다(주석 줄 제외)."""
    out = []
    lines = io.open(path, encoding='utf-8').read().split('\n')
    for i, ln in enumerate(lines):
        code = ln.split('//')[0]
        # ★문맥 정규식에 형태를 하나라도 빠뜨리면 그만큼 조용히 죽는다.
        #   실사고: `pm!`/`pmulti!`/튜플루프/`patch_toggle_bytes` 를 빠뜨려 ~117 사이트를 놓쳤다.
        #   패치에 쓰이는 **모든** 매크로·함수 이름을 여기 넣을 것
        #   (`grep -o 'macro_rules! [a-z_]*'` 로 주기적으로 대조).
        if not re.search(r'patch_imm_bytes|patch_toggle_bytes|patch6\(|patch14\('
                         r'|\bp!\(|\bpany!\(|\bpskip!\(|\bpm!\(|\bpmulti!\('
                         r'|for\s+\w+\s+in\s*\[|for\s*\([^)]*\)\s*in\s*\[', code):
            continue
        for m in HEX.finditer(code):
            out.append((i + 1, int(m.group(1), 16)))
    return out


def main():
    apply = '--apply' in sys.argv
    files = ['detour.rs', 'disc19_repro.rs']
    total = collections.Counter()
    plan = collections.defaultdict(list)

    for fn in files:
        cur = os.path.join(SRCDIR, fn)
        bak = cur + '.053bak'
        base_src = bak if os.path.exists(bak) else cur
        orig = set(a for _, a in candidates(base_src))
        for line, a in candidates(cur):
            if a not in orig:
                continue                      # 이미 옮긴 주소(0.5.4)
            f3 = text_ok(E3, a)
            if not f3:
                total['053명령아님'] += 1
                continue
            pr = R.pair_fn(f3[0], f3[1])
            if not pr:
                total['짝없음'] += 1
                plan['?'].append((fn, line, a, 0, '짝없음'))
                continue
            bs, be, ratio = pr
            i3 = {i.address - B: i for i in R.insns(E3, f3[0], f3[1])}
            ins = i3.get(a)
            i4 = R.insns(E4, bs, be)
            # 바이트 완전일치 우선 → 니모닉+길이 완화
            ex3 = [x for x, y in sorted(i3.items()) if y.bytes == ins.bytes]
            ex4 = [y.address - B for y in i4 if y.bytes == ins.bytes]
            if ex4 and len(ex3) == len(ex4) and a in ex3:
                tgt = ex4[ex3.index(a)]
                tier = '확정'
            else:
                k3 = [x for x, y in sorted(i3.items())
                      if y.mnemonic == ins.mnemonic and len(y.bytes) == len(ins.bytes)]
                k4 = [y.address - B for y in i4
                      if y.mnemonic == ins.mnemonic and len(y.bytes) == len(ins.bytes)]
                if k4 and len(k3) == len(k4) and a in k3:
                    tgt, tier = k4[k3.index(a)], '확정(완화)'
                else:
                    total['미확정'] += 1
                    plan['?'].append((fn, line, a, 0, '시그 %d→%d / 완화 %d→%d'
                                      % (len(ex3), len(ex4), len(k3), len(k4))))
                    continue
            total[tier] += 1
            plan[fn].append((fn, line, a, tgt, tier))

    print('남은 사이트 스윕 결과')
    for k, v in total.most_common():
        print('  %-12s %4d' % (k, v))

    if plan['?']:
        print('\n미확정(주소 유지 — 런타임에서 prefix 불일치로 skip = 안전):')
        for fn, line, a, _, why in plan['?'][:40]:
            print('  %s:%d  %06x  %s' % (fn, line, a, why))

    if not apply:
        print('\n(--apply 를 주면 실제로 고쳐 쓴다)')
        return

    for fn in files:
        if not plan[fn]:
            continue
        p = os.path.join(SRCDIR, fn)
        t = io.open(p, encoding='utf-8').read()
        n = 0
        for _, line, a, tgt, tier in plan[fn]:
            # 같은 주소가 여러 번 나올 수 있으므로 전역 치환(주석 안 값도 같이 바뀌지 않도록
            # `0x<addr>` 뒤에 usize 접미가 붙는 경우까지 포함해 정확히 매칭)
            pat = re.compile(r'0x0*%x\b' % a)
            new, k = pat.subn('0x%06x' % tgt, t)
            if k:
                t, n = new, n + k
        io.open(p, 'w', encoding='utf-8', newline='\n').write(t)
        print('  %s: %d개 치환' % (fn, n))


if __name__ == '__main__':
    main()
