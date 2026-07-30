# -*- coding: utf-8 -*-
# 바닐라 main.ui 에 아이템편집 사이드바 버튼 + 가운데 모달(scroll_view 목록) 주입.
import io, os

# 합친 override 베이스 = 스크림 main.ui(바닐라+스크림UI). 여기에 item_editor UI 가산
# → 결과 = 바닐라+스크림+item_editor 한 파일. item_editor 가 load order 마지막이라 이 합친 게 이김.
# (스크림 main.ui 가 바뀌면 이 gen 재실행 필요. 스크림 무수정.)
REF = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_scrim\ui\layout\main.ui"
OUT_SRC = r"C:\tfm2mods\tfm2_item_editor\ui\layout\main.ui"
OUT_DEP = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_item_editor\ui\layout\main.ui"
ROWS = 90  # 선언 행 수(스크롤). 실제 표시는 dll 이 realN 만 보이고 나머지 hide.

src = io.open(REF, encoding="utf-8").read()

CATEGORY = '''      #ie_bar:color {
        anchor_x: 0.5; pivot_x: 0.5;
        width: 215px; height: 1px; color: #a6a6a6ff;
      }
      #ie_category:empty {
        width: 100%; height: 48px;
        child_type: TopToBottom { spacing: 4px; }
        #ie_open_box:color {
          width: 100%; height: 40px;
          back_color: #1d2740ff; color: #4a6c9aff; stroke: 1;
          rounding: Uniform { rounding: 6; }
          #ie_open_t:label {
            @"asset/base/style/main#label";
            width: 100%; height: 100%; align_x: Center; align_y: Center;
            size: 16; ignore_event: true; text: "아이템 편집";
          }
          #ie_open:button {
            width: 100%; height: 100%;
            hover_sound: "asset/base/sound/sfx/UI_mouse_hover";
          }
        }
      }

'''

def row(i):
    rid = "ie_row%02d" % i
    return '''          #%s:color {
            width: 824px; height: 42px;
            back_color: #1d1f2cff; color: #34384aff; stroke: 1; rounding: Uniform { rounding: 5; }
            #%s_t:label {
              @"asset/base/style/main#label";
              x: 14px; width: 796px; height: 100%%;
              align_x: Left; align_y: Center; size: 15; ignore_event: true; text: "";
            }
            #%s_b:button { width: 100%%; height: 100%%; hover_sound: "asset/base/sound/sfx/UI_mouse_hover"; }
          }
''' % (rid, rid, rid)

rows = "".join(row(i) for i in range(ROWS))

MODAL = '''  #ie_modal:empty {
    width: 100%;
    height: 100%;
    visible: false;

    #ie_dim:color_icon_button {
      width: 100%; height: 100%;
      btn: { color: #000000c0; }
    }

    #ie_panel:color {
      anchor_x: 0.5; pivot_x: 0.5;
      anchor_y: 0.5; pivot_y: 0.5;
      width: 900px; height: 880px;
      color: #161721ff;
      rounding: Uniform { rounding: 12; }

      #ie_title:label {
        @"asset/base/style/main#bold_label";
        x: 30px; y: 20px; width: 600px; height: 32px;
        align_y: Center; size: 22; ignore_event: true;
        text: "아이템 가격 편집";
      }

      #ie_close:button {
        anchor_x: 1; pivot_x: 1; x: -28px; y: 28px; width: 20px; height: 20px;
        source: "asset/base/ui/icons/cross";
        color: #c2c6ceff;
        hover: { color: #e8e8e8ff; }
        active: { color: #e8e8e8ff; }
        hover_sound: "asset/base/sound/sfx/UI_mouse_hover";
      }

      #ie_scroll:scroll_view {
        x: 30px; y: 64px; width: 856px; height: 700px;
        speed: 100; bar_width: 4;
        bar: { source: "asset/base/sprite/white"; color: #37d5b3ff; hover: { color: #ecfbf8ff; } }
        back: { source: "asset/base/sprite/white"; color: #00000000; }
        #ie_list:empty {
          width: 828px; height: 4140px;
          child_type: TopToBottom { spacing: 4px; }
__ROWS__        }
      }

      #ie_sel:label {
        @"asset/base/style/main#label";
        x: 30px; y: 778px; width: 840px; height: 28px;
        align_x: Left; align_y: Center; size: 16; ignore_event: true;
        text: "행을 클릭해 아이템 선택";
      }
    }
  }

'''
MODAL = MODAL.replace("__ROWS__", rows)

# 편집 팝업 (행 클릭 시 뜸): 제목 + 6필드(가격+5스탯) text_edit + 확인/취소
EDIT_FIELDS = [("price","가격"),("atk","공격력"),("mag","주문력"),("def","방어력"),("hp","체력"),("mr","마저")]
def field(i, suf, label):
    y = 72 + i * 60
    return '''      #ie_lbl_%s:label { @"asset/base/style/main#label"; x:28px; y:%dpx; width:130px; height:46px; align_x:Left; align_y:Center; size:16; ignore_event:true; text:"%s"; }
      #ie_in_%s:text_edit { @"asset/base/style/main#text_edit"; x:170px; y:%dpx; width:330px; height:46px; size:17; align_y:Center; padding:{ left:15px; top:5px; right:15px; bottom:5px; } }
''' % (suf, y, label, suf, y)
fields = "".join(field(i, s, l) for i, (s, l) in enumerate(EDIT_FIELDS))
btn_y = 72 + len(EDIT_FIELDS) * 60 + 12
panel_h = btn_y + 52 + 24
EDIT = '''  #ie_edit:empty {
    width: 100%%; height: 100%%; visible: false;

    #ie_edit_dim:color_icon_button { width: 100%%; height: 100%%; btn: { color: #000000d0; } }

    #ie_edit_panel:color {
      anchor_x: 0.5; pivot_x: 0.5; anchor_y: 0.5; pivot_y: 0.5;
      width: 528px; height: %dpx; color: #1b1d28ff; rounding: Uniform { rounding: 12; }

      #ie_edit_title:label { @"asset/base/style/main#bold_label"; x: 28px; y: 22px; width: 472px; height: 30px; align_y: Center; size: 20; ignore_event: true; text: "편집"; }

%s
      #ie_confirm_box:color {
        x: 28px; y: %dpx; width: 230px; height: 52px;
        back_color: #1f6f5cff; color: #2aa784ff; stroke: 1; rounding: Uniform { rounding: 8; }
        #ie_confirm_t:label { @"asset/base/style/main#label"; width:100%%; height:100%%; align_x:Center; align_y:Center; size:17; ignore_event:true; text:"확인 (적용)"; }
        #ie_confirm:button { width:100%%; height:100%%; hover_sound:"asset/base/sound/sfx/UI_mouse_hover"; }
      }
      #ie_cancel_box:color {
        x: 270px; y: %dpx; width: 230px; height: 52px;
        back_color: #2a2c3aff; color: #4a4c56ff; stroke: 1; rounding: Uniform { rounding: 8; }
        #ie_cancel_t:label { @"asset/base/style/main#label"; width:100%%; height:100%%; align_x:Center; align_y:Center; size:17; ignore_event:true; text:"취소"; }
        #ie_cancel:button { width:100%%; height:100%%; hover_sound:"asset/base/sound/sfx/UI_mouse_hover"; }
      }
    }
  }

''' % (panel_h, fields, btn_y, btn_y)

anchor_home = "      #home:empty {"
anchor_tip = "  #tooltip:color {"
assert anchor_home in src and anchor_tip in src, "anchor 없음"
src = src.replace(anchor_home, CATEGORY + anchor_home, 1)
src = src.replace(anchor_tip, MODAL + EDIT + anchor_tip, 1)

for p in (OUT_SRC, OUT_DEP):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    io.open(p, "w", encoding="utf-8").write(src)
    print("wrote", p, len(src), "bytes")
