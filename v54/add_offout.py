# -*- coding: utf-8 -*-
"""champ_verify.txt 출력에 athlete_id 오프셋 판별 결과를 넣는다."""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/tfm2_ai_adjust/src/tfm2_ai_adjust.rs'
t = io.open(P, encoding='utf-8').read()

OLD = '    s.push_str(&format!("\\n[우리팀 게이트 (self_team_only={})]\\n'
assert OLD in t, '앵커 불일치'

NEW = (
    '    // \u2605[08-06] athlete_id \uc624\ud504\uc14b \ud310\ubcc4 \uacb0\uacfc \u2014 \uc624\ubc84\ub77c\uc774\ub4dc \uc720\ubb34\uc640 \ubb34\uad00\ud558\uac8c \uc9d1\uacc4\ub41c\ub2e4.\n'
    '    {\n'
    '        let seen = OFF_SEEN.load(Ordering::Relaxed);\n'
    '        let h8 = OFF_HIT_800.load(Ordering::Relaxed);\n'
    '        let h1 = OFF_HIT_810.load(Ordering::Relaxed);\n'
    '        s.push_str(&format!("\\n[\u2605athlete_id \uc624\ud504\uc14b \ud310\ubcc4 (08-06)]\\n'
    '  \uad00\uce21 \ud45c\ubcf8 = {}\\n  +0x800 \uc774 \uc2e4\uc81c id = {}\\n  +0x810 \uc774 \uc2e4\uc81c id = {}\\n", seen, h8, h1));\n'
    '        if let Ok(g) = OFF_SAMPLE.lock() { if !g.is_empty() {\n'
    '            s.push_str("  \ud45c\ubcf8(0x800 / 0x810) = ");\n'
    '            for (a, b) in g.iter() { s.push_str(&format!("{}/{}  ", a, b)); }\n'
    "            s.push('\\n');\n"
    '        } }\n'
    '        s.push_str("  \u203b 0x800 \ucabd\uc774 \ud06c\uac8c \ub9ce\uc73c\uba74 08-06 \uc815\uc815(0x810\u21920x800)\uc774 \uc637\ub2e4. 0x810 \ucabd\uc774\uba74 \ub418\ub3cc\ub824\uc57c \ud55c\ub2e4.\\n");\n'
    '        s.push_str("  \u203b \ub458 \ub2e4 0\uc778\ub370 \ud45c\ubcf8>0 \uc774\uba74 \ub450 \uc790\ub9ac \ub2e4 athlete_id \uac00 \uc544\ub2c8\ub2e4. \ud45c\ubcf8\uc774 0 \uc774\uba74 \ub85c\uc2a4\ud130 \ubbf8\ud655\ubcf4(\uad00\ub9ac\ud654\uba74 \ubc29\ubb38 \ud544\uc694).\\n");\n'
    '    }\n'
) + OLD

io.open(P, 'w', encoding='utf-8', newline='\n').write(t.replace(OLD, NEW, 1))
print('출력부 삽입 완료')
