# -*- coding: utf-8 -*-
"""전술 104개 영어 이름·설명 표 생성 — tools/i18n/tactics_en.json → src/lang_en.rs (2026-11-11).

    python -X utf8 tools/gen_lang_en.py

검사: tactics.rs 의 전술 id 집합과 같다 · ASCII 만(영어 게임 폰트에 한글 글리프가 없다) · 이름 ≤40자 · 설명 ≤140자.
생성물 src/lang_en.rs 는 손으로 고치지 않는다(번역을 바꾸려면 JSON 을 고치고 다시 생성).
"""
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / 'tools' / 'i18n' / 'tactics_en.json'
OUT = ROOT / 'src' / 'lang_en.rs'

tab = (ROOT / 'src' / 'tactics.rs').read_text(encoding='utf-8')
body = tab[tab.index('pub static TACTICS'):tab.index('pub const N:')]
ids = re.findall(r'^\s*[vt]\("([a-z0-9_]+)"', body, re.M)
assert len(ids) == len(set(ids)), 'tactics.rs 에 중복 id'
d = json.loads(SRC.read_text(encoding='utf-8'))['tactics']
assert set(d) == set(ids), ('id 불일치', sorted(set(ids) - set(d)), sorted(set(d) - set(ids)))


def lit(s):
    assert s.isascii() and '\n' not in s, s
    return json.dumps(s)  # ASCII 만이라 Rust 문자열 리터럴과 같은 꼴(\" \\ 만 이스케이프)


for k in ids:
    assert 0 < len(d[k]['name']) <= 40, (k, d[k]['name'])
    assert 0 < len(d[k]['desc']) <= 140, (k, d[k]['desc'])
lines = ['//! 자동 생성 — tools/gen_lang_en.py (손으로 고치지 말 것). 원천 = tools/i18n/tactics_en.json',
         '//! 전술 id → 영어 이름·설명. 한국어 게임에서는 쓰지 않는다(lang::tactic_name/desc).',
         'pub fn name(id: &str) -> Option<&\'static str> {',
         '    Some(match id {']
lines += ['        %s => %s,' % (lit(k), lit(d[k]['name'])) for k in ids]
lines += ['        _ => return None,', '    })', '}',
          'pub fn desc(id: &str) -> Option<&\'static str> {', '    Some(match id {']
lines += ['        %s => %s,' % (lit(k), lit(d[k]['desc'])) for k in ids]
lines += ['        _ => return None,', '    })', '}', '']
OUT.write_text('\n'.join(lines), encoding='utf-8', newline='\n')
print('전술', len(ids), '→', OUT)
