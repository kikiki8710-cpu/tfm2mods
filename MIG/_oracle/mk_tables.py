# -*- coding: utf-8 -*-
"""o_main.tsv + o_plan.tsv -> _oracle/<함수명>.{json,md}"""
import io, json, os, collections
os.chdir(os.path.dirname(os.path.abspath(__file__)))

TUT = ['None','First','TopSolo','Bottom','MidSolo','MidBottom','JungleOnly','Line','Total']
TI = {t: i for i, t in enumerate(TUT)}
NAME = {(0,1,2,3,4,5,6,7,8):'ALL', (0,7,8):'EPIC', (0,5,7,8):'SERPEN', (0,2,7,8):'L_Top',
        (0,4,5,7,8):'L_Mid', (0,1,3,5,7,8):'L_Bot', (0,6,8):'JUNGLE', ():'NEVER'}

rows = []
for fn in ['o_main.tsv', 'o_plan.tsv']:
    for l in io.open(fn, encoding='utf-8'):
        p = l.rstrip('\n').split('\t')
        if len(p) == 3: rows.append(p)

byfn = collections.defaultdict(lambda: collections.OrderedDict())
for f, inp, out in rows:
    q = inp.split(';'); tut = q[0][4:]; key = ';'.join(q[1:])
    byfn[f].setdefault(key, {})[tut] = out

DESC = {
 'valid_lines':'튜토리얼별 유효 라인 목록(&\'static [LineType]) — 입력=tutorial 뿐',
 'line_exists':'valid_lines(ctx).contains(line)',
 'fallback_line':'line_exists면 그대로, 아니면 대체 라인',
 'position_exists':'포지션이 그 튜토리얼에 존재하는가',
 'morgard_exists':'= spawn_epic(tutorial)',
 'serpen_exists':'= spawn_serpen(tutorial)',
 'steal_target_allowed':'Epic→morgard_exists / Serpen→serpen_exists',
 'steal_action_allowed':'None→true / Lurk(t),Commit(t)→steal_target_allowed(t)',
 'goal_allowed':'BigGoal 튜토리얼 게이트',
 'main_objective_allowed':'MainObjective 튜토리얼 게이트',
 'sub_objective_allowed':'SubObjective 튜토리얼 게이트',
 'plan_allowed':'= goal_allowed(ctx, BigPlan::goal(plan))  (IR m13.ll:54066 확증)',
 'chat_allowed':'Chat 태그별 튜토리얼 게이트',
}

for f, d in byfn.items():
    jr = []
    for k, v in d.items():
        row = {'input': k, 'by_tutorial': {t: v.get(t) for t in TUT}}
        vals = set(v.values())
        if vals <= {'true','false'}:
            s = tuple(sorted(TI[t] for t, o in v.items() if o == 'true'))
            row['allow_tutorials'] = list(s)
            row['class'] = NAME.get(s, 'other')
        jr.append(row)
    io.open('%s.json' % f, 'w', encoding='utf-8').write(json.dumps(
        {'function': 'game_ai::plan_legacy::rule_scope::' + f,
         'oracle': 'SDK rlib 직접 링크(LTO) 실행 결과 / SDK sdk_058 / 게임 0.5.8',
         'note': DESC.get(f, ''), 'tutorial_order': TUT, 'rows': jr},
        ensure_ascii=False, indent=1))

    L = ['# `%s` 전수 진리표 (오라클 실측)' % f, '',
         '> %s' % DESC.get(f, ''), '',
         '> 출처: SDK rlib 직접 링크 실행(`_oracle/o_main.rs`,`o_plan.rs`) / SDK `sdk_058` / 게임 0.5.8', '',
         '| 입력 | ' + ' | '.join('%d %s' % (i, t) for i, t in enumerate(TUT)) + ' | 허용 tut |',
         '|---|' + '---|' * (len(TUT) + 1)]
    for k, v in d.items():
        vals = set(v.values())
        cls = ''
        if vals <= {'true','false'}:
            s = tuple(sorted(TI[t] for t, o in v.items() if o == 'true'))
            cls = '`%s` %s' % (NAME.get(s, ''), list(s))
        cells = []
        for t in TUT:
            o = v.get(t, '?')
            cells.append({'true':'O','false':'.'}.get(o, o))
        L.append('| `%s` | ' % (k if k else '(인자 없음)') + ' | '.join(cells) + ' | %s |' % cls)
    io.open('%s.md' % f, 'w', encoding='utf-8').write('\n'.join(L) + '\n')
    print('%-24s rows=%d' % (f, len(d)))
