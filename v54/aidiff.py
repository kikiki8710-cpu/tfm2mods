# -*- coding: utf-8 -*-
"""AI 판단 영역만 추려 0.5.3 ↔ 0.5.4 를 **소스 파일명 기준**으로 대조한다.

RVA 로 짝짓지 않는다 — RVA 는 통째로 밀렸을 수 있다.
소스 경로(+줄 번호 집합)로 짝지어야 "같은 함수인가"를 버전 무관하게 물을 수 있다.
"""
import io, os, sys, collections

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
D = r'C:\tfm2mods\v54'
AI_HINT = ('game-ai', 'plan_legacy', 'sub_plan', 'small_action', 'action_score',
           'objective', 'utils.rs', 'score_parameter', 'battle', 'line_', 'jungle',
           'hide', 'recall', 'nexus', 'serpen', 'epic', 'steal', 'death_battle',
           'handler', 'chat.rs', 'single_line', 'generic_build', 'passive_')


def load(ver):
    out = []
    for ln in io.open(os.path.join(D, '%s_srcmap.tsv' % ver), encoding='utf-8'):
        s, e, src, lines = ln.rstrip('\n').split('\t')
        out.append((int(s, 16), int(e, 16), src, lines))
    return out


def is_ai(src):
    return any(h in src for h in AI_HINT)


a = [r for r in load('053') if is_ai(r[2])]
b = [r for r in load('054') if is_ai(r[2])]
print('AI 관련 함수:  0.5.3 = %d   0.5.4 = %d' % (len(a), len(b)))

# 소스경로 → 함수들
ga = collections.defaultdict(list)
gb = collections.defaultdict(list)
for r in a:
    ga[r[2]].append(r)
for r in b:
    gb[r[2]].append(r)

only_a = sorted(set(ga) - set(gb))
only_b = sorted(set(gb) - set(ga))
both = sorted(set(ga) & set(gb))

print('\n소스 파일 기준')
print('  양쪽 다 있음 : %d' % len(both))
print('  0.5.3 에만   : %d' % len(only_a))
print('  0.5.4 에만   : %d' % len(only_b))

if only_a:
    print('\n★사라진 소스(0.5.3 전용) — 기능이 없어졌거나 파일이 이름을 바꿨다:')
    for s in only_a:
        print('   - %s   (함수 %d개)' % (s, len(ga[s])))
if only_b:
    print('\n★새로 생긴 소스(0.5.4 전용) — 신규 기능 후보:')
    for s in only_b:
        print('   + %s   (함수 %d개)' % (s, len(gb[s])))

# 같은 소스인데 함수 개수·크기가 달라진 것 = 로직 변경 후보
print('\n★같은 소스인데 모양이 바뀐 것 (로직 변경 후보):')
chg = 0
for s in both:
    fa, fb = ga[s], gb[s]
    sa = sum(x[1] - x[0] for x in fa)
    sb = sum(x[1] - x[0] for x in fb)
    if len(fa) != len(fb) or abs(sa - sb) > max(64, sa * 0.02):
        print('   %-58s 함수 %d→%d   총크기 %d→%d (%+d)' % (s[:58], len(fa), len(fb), sa, sb, sb - sa))
        chg += 1
print('   합계 %d개' % chg)

io.open(os.path.join(D, 'ai_srcsets.txt'), 'w', encoding='utf-8').write(
    '# 0.5.3 전용\n' + '\n'.join(only_a) +
    '\n\n# 0.5.4 전용\n' + '\n'.join(only_b) +
    '\n\n# 공통\n' + '\n'.join(both))
print('\n→ ai_srcsets.txt')
