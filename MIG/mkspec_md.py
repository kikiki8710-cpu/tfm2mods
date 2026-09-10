#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""mkspec_md.py — `specs20.json` 을 REPORT 용 마크다운 명세로 편다.

왜 생성하나: 20함수 × (요지·판정로직·노브·상수·오프셋·호출·해소·미확정)을 손으로 쓰면
JSON 과 어긋난다. **JSON 이 정본이고 문서는 그 투영**이어야 한다.
"""
import io
import json
import os
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, '_spec', 'specs20.json')
DST = os.path.join(
    r'C:\Users\jungs\Desktop\claude\tfm2\.claude\worktrees',
    r'silerus-mode-continue-aed092\mods_report\tfm2_ai_adjust',
    r'06_판단함수_상세명세_20.md')

LAYER_ORDER = ['입력 생성', '점수화·술어', '플랜 핸들러', '레거시 플랜', '기타']


def s(x):
    return '' if x is None else str(x)


def fence(txt):
    return '```\n' + s(txt).rstrip() + '\n```'


def main():
    d = json.load(io.open(SRC, encoding='utf-8'))
    m, sh = d['meta'], d['shared']
    P = []
    P.append('# 판단함수 상세 명세 20 (game_ai IR 독해)')
    P.append('')
    P.append('> **정본은 `C:\\tfm2mods\\MIG\\_spec\\specs20.json` 이다.** 이 문서는 그 투영이고 '
             '`mkspec_md.py` 로 다시 만든다. 손으로 고치지 마라 — JSON 을 고치고 재생성하라.')
    P.append('')
    P.append('- 게임 %s · 작성 %s' % (m['game'], m['date']))
    P.append('- %s' % m['what'])
    P.append('- %s' % m['rounds'])
    c = m['counts']
    P.append('- 확정 오프셋 **%d**(reads %d + writes %d) · 판정상수 **%d** · 피호출 **%d** · '
             '노브 **%d**(+09-10 신규 %d)' %
             (c['reads'] + c['writes'], c['reads'], c['writes'], c['constants'],
              c['calls'], c['knobs'], c['new_knobs']))
    P.append('- 09-10 해소 **%d건** · 남은 미확정 **%d건**(1~6차 원본 기록 %d건 중)' %
             (c['resolved'], c['still_unknown'], c['unknown']))
    P.append('- exe 함수 지도(641개)와 조인된 것 **%d/20** — 나머지 5개는 인라인이거나 지도 선별 밖(각 항목에 사유)' %
             c['joined_to_map'])
    P.append('')
    P.append('연결 문서 — 지도 아티팩트 `AI함수지도.html`(같은 폴더, 20개에 이 명세가 붙어 있다) · '
             '작성 지침 `C:\\tfm2mods\\MIG\\SPEC_GUIDE.md`')
    P.append('')
    P.append('---')
    P.append('')

    # ── 공통 사실 ──
    P.append('## 0. 함수 경계를 넘어 공유되는 사실')
    P.append('')
    P.append('개별 함수보다 **먼저** 읽어야 하는 것들이다. 20개 중 여러 함수가 같은 값·같은 술어를 쓴다.')
    P.append('')
    irv = sh.get('is_recent_visible', {})
    if irv:
        P.append('### 0-1. 시야 — `Blackboard::is_recent_visible`')
        P.append('')
        P.append('- **판정식**: %s' % irv.get('판정식'))
        P.append('- **%s**' % irv.get('핵심'))
        P.append('- %s' % irv.get('tps'))
        P.append('- **`blackboard[T]` 의 의미**: %s' % irv.get('blackboard_인덱스_의미'))
        P.append('- IR: `%s`' % irv.get('ir'))
        P.append('- 영향받는 함수: %s' % ', '.join('`%s`' % x for x in irv.get('영향', [])))
        P.append('- %s' % irv.get('이전_상태'))
        P.append('')
        fam = sh.get('is_recent_visible_4형제') or []
        if fam:
            P.append('| 함수 | 배열 | 시간창 | 특징 |')
            P.append('|---|---|---|---|')
            for f in fam:
                P.append('| `%s` | %s | **%d틱** | %s |' %
                         (f['name'], f['배열'], f['시간창'], f['특징']))
            P.append('')
    bb = sh.get('Blackboard', {})
    if bb:
        P.append('### 0-2. `Blackboard` (%dB) — AI 가 기억하는 것' % bb['size'])
        P.append('')
        P.append('| 오프셋 | 필드 | 타입 |')
        P.append('|---|---|---|')
        for f in bb.get('fields', []):
            P.append('| `%s` | `%s` | %s |' % (f['off'], f['name'], f['type']))
        P.append('')
        for w in bb.get('writers', []):
            P.append('- **쓰는 곳**: `%s` — %s (%s)' % (w['fn'], w['when'], w['ir']))
        if bb.get('역추론'):
            P.append('- ★%s' % bb['역추론'])
        P.append('')
    vt = sh.get('AbstractGame_vtable') or []
    if vt:
        P.append('### 0-3. `AbstractGame` vtable 슬롯 (확인된 것)')
        P.append('')
        P.append('| 슬롯 | 이름 | 시그니처 |')
        P.append('|---|---|---|')
        for x in vt:
            P.append('| `%s` | `%s` | `%s` |' % (x['slot'], x['name'], x['sig']))
        P.append('')
    ev = sh.get('EffectType_vtable', {})
    if ev:
        P.append('### 0-4. `EffectType` vtable 34슬롯 전체')
        P.append('')
        P.append('%s' % ev.get('확정', ''))
        P.append('')
        P.append('근거: %s' % ev.get('근거', ''))
        P.append('')
        sl = ev.get('slots', {})
        ks = sorted(sl, key=lambda k: int(k, 16))
        P.append('| 슬롯 | 이름 | 슬롯 | 이름 |')
        P.append('|---|---|---|---|')
        half = (len(ks) + 1) // 2
        for i in range(half):
            a = '`%s` | `%s`' % (ks[i], sl[ks[i]])
            b = ('`%s` | `%s`' % (ks[i + half], sl[ks[i + half]])) if i + half < len(ks) else ' | '
            P.append('| %s | %s |' % (a, b))
        P.append('')
        if ev.get('정정'):
            P.append('> %s' % ev['정정'])
            P.append('')
    eo = sh.get('Entity_공통_오프셋', {})
    if eo:
        P.append('### 0-5. `Entity` 공통 오프셋')
        P.append('')
        P.append('| 오프셋 | 의미 |')
        P.append('|---|---|')
        for k in sorted(eo, key=lambda x: int(x, 16)):
            P.append('| `%s` | %s |' % (k, eo[k]))
        P.append('')
    cd = sh.get('cooldown_확정', {})
    if cd:
        P.append('### 0-6. 쿨다운 4필드의 의미')
        P.append('')
        P.append('- **%s**' % cd.get('결론'))
        P.append('- 감소: %s' % cd.get('근거_감소'))
        P.append('- 증가: %s' % cd.get('근거_증가'))
        P.append('- %s' % cd.get('이전_상태'))
        P.append('')
    P.append('---')
    P.append('')

    # ── 함수별 ──
    specs = sorted(d['specs'], key=lambda x: (LAYER_ORDER.index(x['layer'])
                                              if x['layer'] in LAYER_ORDER else 9, x['name']))
    P.append('## 1. 계층별 목록')
    P.append('')
    P.append('| 계층 | 함수 | exe | 노브 | 미확정 |')
    P.append('|---|---|---|---|---|')
    for x in specs:
        addr = ('`0x140%s`' % x['exe']['addr']) if x['exe'] else '—'
        if x['exe'] and x['exe'].get('ambiguous'):
            addr += ' ⚠2곳'
        P.append('| %s | [`%s::%s`](#%s) | %s | %d | %d |' %
                 (x['layer'], os.path.basename(s(x['src'])).replace('.rs', ''), x['name'],
                  x['id'].lower(), addr,
                  len(x['knobs']) + len(x['new_knobs']), len(x['still_unknown'])))
    P.append('')
    P.append('---')
    P.append('')

    P.append('## 2. 함수별 상세')
    P.append('')
    cur = None
    for x in specs:
        if x['layer'] != cur:
            cur = x['layer']
            P.append('### 계층 — %s' % cur)
            P.append('')
        P.append('<a id="%s"></a>' % x['id'].lower())
        P.append('')
        P.append('#### `%s`' % x['name'])
        P.append('')
        P.append('> %s' % s(x['one_line']))
        P.append('')
        meta = ['소스 `%s:%s`' % (s(x['src']), s(x['src_line'])),
                'IR `%s:%s~%s`' % (x['ir']['file'], x['ir']['frm'], x['ir']['to']),
                '%d회 독립 재작성' % x['rounds']]
        if x['exe']:
            meta.append('exe `0x140%s` (%sB, 근거=%s)' %
                        (x['exe']['addr'], x['exe']['bytes'], x['exe'].get('evidence')))
        P.append('- ' + ' · '.join(meta))
        if x['exe'] and x['exe'].get('ambiguous'):
            P.append('- ⚠**이 이름은 exe 에 %d곳이다** (%s). 이 명세는 위 소스 기준이고 '
                     '**어느 주소가 그것인지는 확인하지 않았다.** 다른 쪽은 동명 다른 타입일 수 있다.' %
                     (len(x['exe']['ambiguous']),
                      ' / '.join('`0x140%s`' % a for a in x['exe']['ambiguous'])))
        if not x['exe']:
            P.append('- ⚠지도 미대응 — %s' % s(x.get('no_map_reason')))
        P.append('')
        if x.get('logic'):
            P.append('**판정 로직**')
            P.append('')
            P.append(fence(x['logic']))
            P.append('')
        kn = [k for k in x['knobs'] if isinstance(k, dict)]
        nk = [k for k in x['new_knobs'] if isinstance(k, dict)]
        if kn or nk:
            P.append('**바꿀 수 있는 값 (%d)**' % (len(kn) + len(nk)))
            P.append('')
            P.append('| 무엇 | 값 | 바꾸면 | 위치 |')
            P.append('|---|---|---|---|')
            for k in kn:
                P.append('| %s | `%s` | %s | `%s` |' %
                         (s(k.get('what')), s(k.get('value')), s(k.get('effect')), s(k.get('where'))))
            for k in nk:
                P.append('| ★신규 %s | `%s` | %s | `%s` |' %
                         (s(k.get('what')), s(k.get('value')) or '—',
                          s(k.get('effect')), s(k.get('where'))))
            P.append('')
        if x['constants']:
            P.append('**판정 상수 (%d)**' % len(x['constants']))
            P.append('')
            P.append('| 값 | 의미 | 소스줄 |')
            P.append('|---|---|---|')
            for k in x['constants']:
                if not isinstance(k, dict):
                    continue
                P.append('| `%s` | %s | %s |' % (s(k.get('value')), s(k.get('meaning')),
                                                 s(k.get('src_line'))))
            P.append('')
        if x['resolved']:
            P.append('**2026-09-10 에 확정된 것 (%d)**' % len(x['resolved']))
            P.append('')
            for r in x['resolved']:
                P.append('- ~~%s~~' % s(r.get('was')))
                P.append('  → %s' % s(r.get('now')))
                for k, v in r.items():
                    if k in ('was', 'now') or not isinstance(v, str):
                        continue
                    P.append('  - **%s**: %s' % (k, v))
                if r.get('aux'):
                    P.append('  - **aux**: `%s`' % json.dumps(r['aux'], ensure_ascii=False))
            P.append('')
        if x['still_unknown']:
            P.append('**아직 모르는 것 (%d)**' % len(x['still_unknown']))
            P.append('')
            for u in x['still_unknown']:
                P.append('- %s' % s(u))
            P.append('')
        if x['reads'] or x['writes']:
            P.append('<details><summary>읽는 필드 %d · 쓰는 필드 %d</summary>' %
                     (len(x['reads']), len(x['writes'])))
            P.append('')
            for tag, arr in (('읽기', x['reads']), ('쓰기', x['writes'])):
                if not arr:
                    continue
                P.append('| %s | 구조체 | 오프셋 | 필드 |' % tag)
                P.append('|---|---|---|---|')
                for r in arr:
                    if not isinstance(r, dict):
                        continue
                    P.append('| | %s | `%s` | `%s` %s |' %
                             (s(r.get('base')), s(r.get('offset')), s(r.get('name')),
                              s(r.get('note'))[:90]))
                P.append('')
            P.append('</details>')
            P.append('')
        P.append('---')
        P.append('')

    io.open(DST, 'w', encoding='utf-8', newline='').write('\n'.join(P))
    print('%s  (%.0f KB · %d줄)' % (DST, os.path.getsize(DST) / 1024.0, len(P)))


if __name__ == '__main__':
    main()
