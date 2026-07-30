# -*- coding: utf-8 -*-
# item_editor UI 를 tfm2_ui_inject 프레임워크 주입용 "조각 .ui" 로 생성.
#   button.ui → 사이드바(left), 위치 after:house_category (게이밍하우스 뒤)
#   modal.ui  → root (아이템 목록 모달 — 표 형태: 이름|가격|공격|주문|방어|체력|마저)
#   edit.ui   → root (가격/스탯 편집 팝업)
# ⚠ 조각 루트는 '#' 없이 (파서 규칙). 자식만 '#'. dll 은 ie_* 노드를 id 로 구동(변경 불필요).
import io, os
ROWS = 90
SRC_DIR = r"C:\tfm2mods\tfm2_item_editor\ui_inject"
DEP_DIR = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_item_editor\ui_inject"

# 표 컬럼: (헤더제목, 행내 x, width, align). 첫 칸=이름(_n), 나머지=값칸(_v0.._v5).
# 값칸 순서 = dll vals[]: [가격, 공격, 주문, 방어, 체력, 마저]  (STAT_OFFS 0=공격,1=주문,2=방어,3=체력,4=마저)
COLS = [
    ("이름", 14, 286, "Left"),
    ("가격", 304, 92, "Center"),
    ("공격", 396, 80, "Center"),
    ("주문", 476, 80, "Center"),
    ("방어", 556, 80, "Center"),
    ("체력", 636, 84, "Center"),
    ("마저", 720, 90, "Center"),
]

# 네이티브 사이드바 버튼 룩(scrim_btn 과 동일 구조 = color_icon_button + 아이콘 + 텍스트).
#   클릭 id = ie_open (dll 라우트 그대로). 아이콘=gold_get(돈/가격 테마). 폰트=regular(덜 두껍게).
BUTTON = '''ie_open_cat:empty {
  width: 100%; height: 48px;
  child_type: TopToBottom { spacing: 4px; }
  #ie_open:color_icon_button {
    width: 100%; height: 48px;
    hover_sound: "asset/base/sound/sfx/UI_mouse_hover";
    click_sound: "asset/base/sound/sfx/UI_mouse_click";
    color: #8a8d96ff;
    hover: { color: #e8e8e8ff; }
    active: { color: #e8e8e8ff; }
    btn: {
      color: #00000000;
      hover: { color: #ffffff14; }
      active: { color: #ffffff1f; }
    }
    icon: {
      source: "asset/base/ui/icons/gold_get";
      rect: { x: 24; y: 12; w: 24; h: 24; }
    }
    text: {
      text: "아이템 편집";
      rect: { x: 56; y: 14; w: 184; h: 20; }
      font: "asset/base/font/set/regular";
      size: 18;
      align_x: Left;
      align_y: Center;
    }
  }
}
'''

def row(i):
    rid = "ie_row%02d" % i
    cells = ""
    # 이름칸(_n): 활성=엔진 i18n 참조→현지화 / 비활성·미발견=코드명 폴백 (dll 이 채움)
    _, cx, cw, al = COLS[0]
    cells += '        #%s_n:label { @"asset/base/style/main#label"; x: %dpx; width: %dpx; height: 100%%; align_x: %s; align_y: Center; size: 15; ignore_event: true; text: ""; }\n' % (rid, cx, cw, al)
    # 값칸 v0..v5 (가격/공격/주문/방어/체력/마저) — 보이지 않는 칸(라벨만, 경계선 없음)
    for vi, (_, cx, cw, al) in enumerate(COLS[1:]):
        cells += '        #%s_v%d:label { @"asset/base/style/main#label"; x: %dpx; width: %dpx; height: 100%%; align_x: %s; align_y: Center; size: 14; ignore_event: true; text: ""; }\n' % (rid, vi, cx, cw, al)
    return '''      #%s:color {
        width: 824px; height: 42px;
        back_color: #1d1f2cff; color: #34384aff; stroke: 1; rounding: Uniform { rounding: 5; }
%s        #%s_b:button { width: 100%%; height: 100%%; hover_sound: "asset/base/sound/sfx/UI_mouse_hover"; }
      }
''' % (rid, cells, rid)

rows = "".join(row(i) for i in range(ROWS))

# 표 머리글 (스크롤 바깥, 패널 직속). 헤더 x = 30(scroll) + 행내 x → 데이터칸과 정렬.
def header():
    s = '    #ie_hdr_bg:color { x: 30px; y: 60px; width: 824px; height: 30px; back_color: #191b25ff; color: #2d3140ff; stroke: 1; rounding: Uniform { rounding: 5; } }\n'
    for ci, (t, cx, cw, al) in enumerate(COLS):
        s += '    #ie_hdr%d:label { @"asset/base/style/main#bold_label"; x: %dpx; y: 62px; width: %dpx; height: 26px; align_x: %s; align_y: Center; size: 13; ignore_event: true; text: "%s"; }\n' % (ci, 30 + cx, cw, al, t)
    return s

MODAL = '''ie_modal:empty {
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
      x: 30px; y: 18px; width: 520px; height: 32px;
      align_y: Center; size: 22; ignore_event: true; text: "아이템 편집";
    }
    #ie_close:button {
      anchor_x: 1; pivot_x: 1; x: -28px; y: 26px; width: 20px; height: 20px;
      source: "asset/base/ui/icons/cross"; color: #c2c6ceff;
      hover: { color: #e8e8e8ff; } active: { color: #e8e8e8ff; }
      hover_sound: "asset/base/sound/sfx/UI_mouse_hover";
    }
__HEADER__    #ie_scroll:scroll_view {
      x: 30px; y: 94px; width: 856px; height: 666px;
      speed: 100; bar_width: 4;
      bar: { source: "asset/base/sprite/white"; color: #37d5b3ff; hover: { color: #ecfbf8ff; } }
      back: { source: "asset/base/sprite/white"; color: #00000000; }
      #ie_list:empty {
        width: 828px; height: 4140px;
        child_type: TopToBottom { spacing: 4px; }
__ROWS__      }
    }
    #ie_sel_n:label {
      @"asset/base/style/main#label";
      x: 30px; y: 770px; width: 430px; height: 28px;
      align_x: Left; align_y: Center; size: 17; ignore_event: true; text: "";
    }
    #ie_sel:label {
      @"asset/base/style/main#label";
      x: 466px; y: 772px; width: 404px; height: 26px;
      align_x: Left; align_y: Center; size: 14; ignore_event: true; text: "행을 클릭해 아이템 선택";
    }
  }
}
'''
MODAL = MODAL.replace("__HEADER__", header()).replace("__ROWS__", rows)

EDIT_FIELDS = [("price","가격"),("atk","공격력"),("mag","주문력"),("def","방어력"),("hp","체력"),("mr","마저")]
def field(i, suf, label):
    y = 84 + i * 60
    return '''    #ie_lbl_%s:label { @"asset/base/style/main#label"; x:28px; y:%dpx; width:130px; height:46px; align_x:Left; align_y:Center; size:16; ignore_event:true; text:"%s"; }
    #ie_in_%s:text_edit { @"asset/base/style/main#text_edit"; x:170px; y:%dpx; width:330px; height:46px; size:17; align_y:Center; padding:{ left:15px; top:5px; right:15px; bottom:5px; } }
''' % (suf, y, label, suf, y)
fields = "".join(field(i, s, l) for i, (s, l) in enumerate(EDIT_FIELDS))
btn_y = 84 + len(EDIT_FIELDS) * 60 + 12
panel_h = btn_y + 52 + 24
EDIT = '''ie_edit:empty {
  width: 100%%; height: 100%%; visible: false;
  #ie_edit_dim:color_icon_button { width: 100%%; height: 100%%; btn: { color: #000000d0; } }
  #ie_edit_panel:color {
    anchor_x: 0.5; pivot_x: 0.5; anchor_y: 0.5; pivot_y: 0.5;
    width: 528px; height: %dpx; color: #1b1d28ff;
    rounding: Uniform { rounding: 12; }
    #ie_edit_title:label { @"asset/base/style/main#bold_label"; x: 28px; y: 22px; width: 472px; height: 30px; align_y: Center; size: 20; ignore_event: true; text: "편집"; }
    #ie_edit_name:label { @"asset/base/style/main#label"; x: 28px; y: 52px; width: 472px; height: 26px; align_y: Center; size: 16; ignore_event: true; text: ""; }
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

MANIFEST = '''# <ui상대경로> <타깃컨테이너id> <위치> [modal]
# modal = 그 조각 보일때 프레임워크가 배경(top) 입력차단 → 뒤쪽 호버팝업 누수 방지
ui_inject/button.ui left after:house_category
ui_inject/modal.ui root end modal
ui_inject/edit.ui root end modal
'''

files = {"button.ui": BUTTON, "modal.ui": MODAL, "edit.ui": EDIT}
for d in (SRC_DIR, DEP_DIR):
    os.makedirs(d, exist_ok=True)
    for name, content in files.items():
        io.open(os.path.join(d, name), "w", encoding="utf-8").write(content)
    print("wrote fragments to", d)
# 매니페스트는 모드 폴더 루트
for base in (r"C:\tfm2mods\tfm2_item_editor", r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_item_editor"):
    io.open(os.path.join(base, "ui_inject.txt"), "w", encoding="utf-8").write(MANIFEST)
    print("wrote manifest to", base)
