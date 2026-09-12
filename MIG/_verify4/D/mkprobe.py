# -*- coding: utf-8 -*-
"""TEMPLATE.rs 의 헤더(real_setting/setting_ok/mkgame)를 그대로 재사용해 프로브를 만든다."""
import io, sys
TPL = r'C:\tfm2mods\MIG\_verify3\TEMPLATE.rs'
tpl = io.open(TPL, encoding='utf-8').read()
head = tpl[:tpl.index('fn main() {')]
head = head.replace('use game_core::*;',
                    'use game_core::*;\nuse game_ai::plan_legacy::old as old;\n'
                    'use game_ai::plan_legacy::team_plan::TeamPlan;')
body = io.open(sys.argv[1], encoding='utf-8').read()
io.open(sys.argv[2], 'w', encoding='utf-8').write(head + body)
print('written', sys.argv[2], len(head + body))
