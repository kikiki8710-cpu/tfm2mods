# -*- coding: utf-8 -*-
"""구조체 오프셋 0.5.3 → 0.5.4 치환. **코드 부분만** 바꾸고 주석은 건드리지 않는다
(주석은 그 시점의 근거 기록이라 덮어쓰면 이력이 사라진다).

근거 = 두 에이전트가 **같은 함수의 명령을 직접 대조**해 독립적으로 낸 결론:
  team/side   +0x820 → +0x810     (abstract_input 명령대조 + minion_wave_risk)
  role/champIdx +0x8b0 → +0x8a0
  판단력       +0x3f8 → +0x3f0     (판단력 계산 계수 하나가 제거되며 같이 이동)
  plan        +0x598 → +0x5e8     (plan 디스패처 호출자에서 imm write 실측, 2건 교차확증)
  sub_plan    +0x6b0 → +0x708     (경매 폴백 `cmp dword[unit_ai+0x708],0xa` 실측)

⚠`0x598`은 **다른 구조체에서도 쓰인다**(disc19_repro/genbuild_repro 의 divisor 페어).
   그쪽은 문맥이 달라 자동 치환 대상에서 제외한다.
"""
import io, os, re, sys, collections

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
SRC = r'C:\tfm2mods\tfm2_ai_adjust\src'

MAP = [('0x820', '0x810'), ('0x8b0', '0x8a0'), ('0x3f8', '0x3f0')]
# plan/sub_plan 은 사용처가 적어 개별 처리
EXACT = [
    ('tfm2_ai_adjust.rs',
     'let slot = if readable(ent + 0x6b0, 4) { rd_u32(ent + 0x6b0) as i64 } else { -1 };',
     'let slot = if readable(ent + 0x708, 4) { rd_u32(ent + 0x708) as i64 } else { -1 };   // ★0.5.4(was 0x6b0)'),
    ('tfm2_ai_adjust.rs',
     'let plan = if readable(ent + 0x598, 4) { rd_u32(ent + 0x598) as i64 } else { -1 };',
     'let plan = if readable(ent + 0x5e8, 4) { rd_u32(ent + 0x5e8) as i64 } else { -1 };   // ★0.5.4(was 0x598)\n'
     '    //   ⚠★Plan **번호도 −2 시프트**됐다(Battle 9→7, DefenseNexus 17→15). 값을 해석해 쓰는 쪽은 같이 고칠 것.'),
]

# 이 파일들은 다른 구조체 문맥이라 자동치환 제외
SKIP_AUTO = {'genbuild_repro.rs'}


def strip_comment(s):
    """줄에서 코드 부분만 반환(문자열 리터럴 안의 // 는 이 코드베이스에 없음)."""
    i = s.find('//')
    return (s, '') if i < 0 else (s[:i], s[i:])


def main():
    total = collections.Counter()
    for fn in os.listdir(SRC):
        if not fn.endswith('.rs') or fn in SKIP_AUTO or fn.startswith('rva_'):
            continue
        p = os.path.join(SRC, fn)
        lines = io.open(p, encoding='utf-8').read().split('\n')
        out, ch = [], 0
        for ln in lines:
            code, com = strip_comment(ln)
            new = code
            for a, b in MAP:
                if a in new:
                    new = new.replace(a, b)
            if new != code:
                ch += new != code
                for a, b in MAP:
                    total[a] += code.count(a)
                com = (com or '//') + '  ★0.5.4 오프셋 이동 반영'
            out.append(new + com)
        if ch:
            io.open(p, 'w', encoding='utf-8', newline='\n').write('\n'.join(out))
            print('  %-24s %d줄' % (fn, ch))

    for fn, a, b in EXACT:
        p = os.path.join(SRC, fn)
        t = io.open(p, encoding='utf-8').read()
        if a in t:
            io.open(p, 'w', encoding='utf-8', newline='\n').write(t.replace(a, b, 1))
            print('  %-24s 개별: %s' % (fn, a[:44]))
            total['exact'] += 1
        else:
            print('  ⚠%s 개별 치환 대상 없음: %s' % (fn, a[:44]))

    print('\n치환 합계: %s' % dict(total))


if __name__ == '__main__':
    main()
