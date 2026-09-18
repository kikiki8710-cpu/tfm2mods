# -*- coding: utf-8 -*-
"""assets/champ_excl_popup.ui 생성기(셀 120개 전개) + i18n 클래스 탭 키 보강. 0.6.0 stable 판(z 속성 없음 — 0.6.0 은 z 가 자식 렌더를 죽임)."""
import json, os
os.chdir(os.path.dirname(os.path.abspath(__file__)))
head = r'''champ_excl_popup:color {
  width: 1763px;
  height: 1003px;
  anchor_x: 0.5;
  pivot_x: 0.5;
  anchor_y: 0.5;
  pivot_y: 0.5;
  visible: false;
  color: #161721ff;
  rounding: Uniform { rounding: 12; }

  #header:label {
    @"asset/base/style/main#bold_label";
    x: 32px; y: 24px; width: 900px; height: 42px; size: 24; align_y: Center;
    text: "#asset/base/text/ui?champ_excl.title";
  }

  #close:button {
    width: 16px; height: 16px; anchor_x: 1; pivot_x: 1; x: -32px; y: 32px;
    source: "asset/base/ui/icons/cross"; color: #c2c6ceff;
    hover: { color: #e8e8e8ff; } active: { color: #e8e8e8ff; }
  }

  #filter_bar:empty {
    x: 32px; y: 81px; width: 900px; height: 40px;

    <<CLASS_DD>>

    #champ_search:text_edit {
      @"asset/base/style/main#text_edit";
      x: 158px; width: 200px; height: 40px;
      size: 16; align_y: Center;
      padding: { left: 44px; top: 5px; right: 15px; bottom: 5px; }
      placeholder: "#asset/base/text/ui?banpick.champion_search_placeholder";
      max_length: 40;

      #icon:image {
        ignore_event: true;
        x: -30px; y: 5px; width: 20px; height: 20px;
        source: "asset/base/ui/banpick/fi-rr-search";
        color: #858d9dff;
      }
    }

    #search_clear:color_icon_button {
      @"asset/base/style/main#tertiary_button";
      x: 366px; width: 40px; height: 40px;
      icon: { source: "asset/base/ui/icons/cross"; rect: { x: 12; y: 12; w: 16; h: 16; } }
    }

    #filter_count:label {
      @"asset/base/style/main#label";
      x: 420px; width: 240px; height: 40px; size: 15; align_y: Center;
      color: #858d9dff;
    }
  }

  #left:color {
    x: 32px; y: 136px; width: 1207px; height: 787px; color: #1d1f2cff;
    rounding: Uniform { rounding: 8; }

    #scroll:scroll_view {
      x: 16px; y: 16px; width: 1175px; height: 755px; speed: 100; bar_width: 4;
      bar_padding: { top: 8px; bottom: 8px; }
      bar: { source: "asset/base/sprite/white"; color: #37d5b3ff; hover: { color: #ecfbf8ff; } }
      back: { source: "asset/base/sprite/white"; color: #4a4c56ff; }

      #contents:empty {
        x: 8px; y: 8px; width: 1154px; height: 1900px;
        child_type: Table { spacing_x: 15px; spacing_y: 15px; }
'''
cell = '''        #cell{k}:color_icon_button {{ @"asset/base/style/main#tertiary_button"; width: 152px; height: 171px; visible: false;
          #icon:image {{ anchor_x: 0.5; pivot_x: 0.5; pivot_y: 1; x: 0px; y: 122px; width: 84px; height: 84px; ignore_event: true; }}
          #name:label {{ @"asset/base/style/main#label"; x: 2px; y: 132px; width: 148px; height: 22px; size: 13; align_x: Center; align_y: Center; ignore_event: true; }}
          #sel:color {{ visible: false; x: 0px; y: 0px; width: 152px; height: 171px; back_color: #00000000; color: #ff5c5cff; stroke: 3; rounding: Uniform {{ rounding: 8; }} ignore_event: true; }}
        }}
'''
tail = r'''      }
    }
  }

  #right:color {
    x: 1254px; y: 136px; width: 477px; height: 787px; color: #1d1f2cff;
    rounding: Uniform { rounding: 8; }

    #summary:label { @"asset/base/style/main#bold_label"; x: 24px; y: 24px; width: 429px; height: 28px; size: 18; align_y: Center; text: "#asset/base/text/ui?champ_excl.summary"; }
    #hint:label { @"asset/base/style/main#label"; x: 24px; y: 68px; width: 429px; height: 130px; size: 16; line_height: 26; align_y: Center; text: "#asset/base/text/ui?champ_excl.hint"; }
    #cnt_total_k:label { @"asset/base/style/main#label"; x: 24px; y: 218px; width: 290px; height: 24px; size: 16; align_y: Center; text: "#asset/base/text/ui?champ_excl.lbl_total"; }
    #cnt_total_v:label { @"asset/base/style/main#label"; x: 320px; y: 218px; width: 133px; height: 24px; size: 16; align_y: Center; }
    #cnt_sel_k:label { @"asset/base/style/main#bold_label"; x: 24px; y: 246px; width: 290px; height: 26px; size: 16; align_y: Center; text: "#asset/base/text/ui?champ_excl.lbl_sel"; }
    #cnt_sel_v:label { @"asset/base/style/main#bold_label"; x: 320px; y: 246px; width: 133px; height: 26px; size: 16; align_y: Center; }
    #src:label { @"asset/base/style/main#label"; x: 24px; y: 276px; width: 429px; height: 44px; size: 14; line_height: 20; color: #858d9dff; align_y: Center; }
    #note_all:label { @"asset/base/style/main#label"; x: 24px; y: 324px; width: 429px; height: 100px; size: 15; line_height: 24; color: #ffb84aff; align_y: Center; }

    #sel_none:color_icon_button { @"asset/base/style/main#tertiary_button"; x: 24px; y: 470px; width: 429px; height: 40px; text: { text: "#asset/base/text/ui?champ_excl.btn_none"; font: "asset/base/font/set/bold"; size: 17; align_x: Center; align_y: Center; } }
    #sel_all:color_icon_button { @"asset/base/style/main#tertiary_button"; x: 24px; y: 522px; width: 429px; height: 40px; text: { text: "#asset/base/text/ui?champ_excl.btn_all"; font: "asset/base/font/set/bold"; size: 17; align_x: Center; align_y: Center; } }
  }

  #cancel:color_icon_button { @"asset/base/style/main#tertiary_button"; x: 654px; y: 943px; width: 220px; height: 40px; text: { text: "#asset/base/text/ui?champ_excl.cancel"; font: "asset/base/font/set/bold"; size: 18; align_x: Center; align_y: Center; } }
  #ok:color_icon_button { @"asset/base/style/main#tertiary_button"; x: 890px; y: 943px; width: 220px; height: 40px; text: { text: "#asset/base/text/ui?champ_excl.ok"; font: "asset/base/font/set/bold"; size: 18; align_x: Center; align_y: Center; } }

  <<CLASS_LIST>>
}
'''
s = head + ''.join(cell.format(k=k) for k in range(120)) + tail
open('assets/champ_excl_popup.ui', 'w', encoding='utf-8', newline='\n').write(s)
p = 'assets/champion_exclude.i18n'
d = json.load(open(p, encoding='utf-8'))
d['en']['champ_excl'].update({"class_all": "All", "class_melee": "Fighter", "class_range": "Ranged", "class_magician": "Mage", "class_util": "Support", "class_assassin": "Assassin", "src_avail_unknown": "Released list could not be read - showing patch-day observations only"})
d['ko']['champ_excl'].update({"class_all": "전체", "class_melee": "전사", "class_range": "원거리", "class_magician": "마법사", "class_util": "전투보조", "class_assassin": "암살자", "src_avail_unknown": "출시 목록을 읽지 못함 — 패치데이 관측분만 표시"})
open(p, 'w', encoding='utf-8', newline='\n').write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")
print(len(s), open(p, 'rb').read()[:1])
