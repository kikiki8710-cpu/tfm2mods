# -*- coding: utf-8 -*-
"""09-18 유저 요청: ①포지션 탭 선택색 미적용(0.6.0 stable 은 color_selectable 의 selected 상태를 못 씀) → color_icon_button 탭으로 교체하고
   모드가 색을 직접 칠한다 ②클래스 필터 버튼 6개 → 풀다운(버튼 + 목록 패널). 적용 대상 = pos_lock_popup.ui / champ_excl_popup.ui(생성기)."""
import os, re
BTN = '@"asset/base/style/main#tertiary_button";'

def tab(id_, w, h, text_tag, size=16, extra=''):
    return f'#{id_}:color_icon_button {{ {BTN} width: {w}px; height: {h}px; {extra} text: {{ text: "{text_tag}"; font: "asset/base/font/set/bold"; size: {size}; align_x: Center; align_y: Center; }} }}'

def class_dd(prefix_i18n, x, y):
    # 풀다운 버튼(현재 클래스명 라벨 + ▼) — 목록 패널은 팝업 루트 마지막 자식(class_list)으로 별도 스폰(트리 순서로 위에 그려짐)
    return f'''    #class_dd:color_icon_button {{ {BTN} x: {x}px; y: {y}px; width: 150px; height: 40px;
      text: {{ text: "▼"; size: 14; align_x: Right; align_y: Center; }}
      #label:label {{ @"asset/base/style/main#label"; ignore_event: true; x: 14px; width: 110px; height: 40px; size: 16; align_y: Center; text: "{prefix_i18n}class_all"; }}
    }}
'''

def class_list(prefix_i18n, x, y):
    keys = ["class_all", "class_melee", "class_range", "class_magician", "class_util", "class_assassin"]
    rows = "\n".join(f'    #{k}:color_icon_button {{ {BTN} width: 150px; height: 40px; text: {{ text: "{prefix_i18n}{k}"; size: 15; align_x: Center; align_y: Center; }} }}' for k in keys)
    return f'''
  #class_list:color {{
    visible: false; x: {x}px; y: {y}px; width: 150px; height: 244px;
    color: #1d1f2cff; stroke: 1; back_color: #4a4c56ff;
    rounding: Uniform {{ rounding: 8; }}
    padding: {{ left: 0px; right: 0px; top: 2px; bottom: 2px; }}
    child_type: TopToBottom {{ spacing: 0px; }}
{rows}
  }}
'''

def patch_poslock():
    p = r'C:\tfm2mods\tfm2_champ_pos_lock\assets\pos_lock_popup.ui'
    s = open(p, encoding='utf-8').read()
    # ① 포지션 탭
    for k in ["top", "jungle", "mid", "bottom", "support"]:
        old = re.search(rf'    #tab_{k}:color_selectable \{{[^\n]*\n', s).group(0)
        s = s.replace(old, '    ' + tab(f'tab_{k}', 150, 32, f'#asset/base/text/ui?pos_lock.tab_{k}') + '\n')
    # ② 클래스 탭 블록 → 풀다운 버튼
    i = s.index('    #class_tabs:color {'); j = s.index('    }\n', i) + 6
    s = s[:i] + class_dd('#asset/base/text/ui?pos_lock.', 0, 0) + s[j:]
    # ③ 목록 패널 = 루트 마지막 자식 (filter_bar x 802 + 0, y 81 + 44)
    k = s.rindex('}')
    s = s[:k] + class_list('#asset/base/text/ui?pos_lock.', 802, 125) + '}\n'
    open(p, 'w', encoding='utf-8', newline='\n').write(s)
    print('poslock ui ok')

def patch_excl_gen():
    p = r'C:\tfm2mods\tfm2_champion_exclude\_gen_popup_ui.py'
    s = open(p, encoding='utf-8').read()
    i = s.index('    #class_tabs:color {'); j = s.index('    }\n', i) + 6
    s = s[:i] + class_dd('#asset/base/text/ui?champ_excl.', 0, 0).replace('{{', '{').replace('}}', '}') + s[j:]
    # 목록 패널: tail 의 마지막 '}' 앞에 삽입
    lst = class_list('#asset/base/text/ui?champ_excl.', 32, 125).replace('{{', '{').replace('}}', '}')
    k = s.index("  #ok:color_icon_button")
    k2 = s.index('\n', k) + 1
    s = s[:k2] + lst + s[k2:]
    open(p, 'w', encoding='utf-8', newline='\n').write(s)
    print('excl gen ok')

patch_poslock(); patch_excl_gen()
