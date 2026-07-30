# draft_popup.ui 생성기 v6 — 동적 높이 스크롤(모드가 내용 행수에 맞춰 list 높이 세팅). 슬롯 100(여백부담 0).
NROWS = 100
LABEL = '@"asset/base/style/main#label"'
BOLD = '@"asset/base/style/main#bold_label"'

# ── 가로 스케일 ── 팝업을 가로 1.2배로 넓히고 내부 요소 x/width 를 함께 스케일(세로·폰트·행높이는 유지).
SX = 1.2
def sx(v):
    return int(round(v * SX))

# 전체 크기 (가로만 1.2배: 680 → 816)
W, H = sx(680), 824
LIST_W = sx(604)   # 스크롤 리스트(행 컨테이너) 폭
ROWH = 30
FS = 15          # 행 폰트
FS_SC = 15       # 점수/수치 폰트
FS_TT = 13       # 티어뱃지 폰트
FS_HEAD = 14     # 컬럼헤더 폰트

# 컬럼 위치(행 내부, 좌측 기준). name=c0 는 full-width. 전부 가로 스케일 적용.
COL_TIER_X, COL_TIER_W = sx(224), sx(46)
COL_SC_X, COL_SC_W = sx(278), sx(62)
COL_WR_X, COL_WR_W = sx(352), sx(70)
COL_PR_X, COL_PR_W = sx(434), sx(70)
COL_P5_X, COL_P5_W = sx(512), sx(60)    # 메타해석 6번째 컬럼(표본), 메타통계에선 빈칸
# No | 사진 | 이름 3컬럼(메타/해석 공통). c0=번호/불릿(좌측), face=얼굴, mname=이름.
COL_FACE_X, COL_FACE_W = sx(44), 28     # 얼굴 크기는 행높이(30px)에 맞춰 유지
COL_NAME_X, COL_NAME_W = sx(82), sx(134)
C0_X, C0_W = sx(10), sx(578)            # 라벨/전체폭 컬럼(가장 넓은 텍스트 필드)
VAL_X, VAL_W = sx(120), sx(468)         # 우측 넓은 값 필드

def btn(bid, x, w, h, ihw, icon="statistics"):
    return f'''    #{bid}:color_icon_button {{
      x: {x}px; y: 0px; width: {w}px; height: {h}px;
      hover_sound: "asset/base/sound/sfx/UI_mouse_hover";
      click_sound: "asset/base/sound/sfx/UI_mouse_click";
      color: #00000000;
      btn: {{ color: #ffffff0f; hover: {{ color: #ffffff22; }} active: {{ color: #ffffff33; }} rounding: Uniform {{ rounding: 6; }} }}
      icon: {{ source: "asset/base/ui/icons/{icon}"; rect: {{ x: 6; y: {(h-ihw)//2}; w: {ihw}; h: {ihw}; }} }}
    }}'''

def olabel(lid, x, w, h, size, color, text):
    return (f'    #{lid}:label {{ {BOLD}; x: {x}px; y: 0px; anchor_y: 0.5; pivot_y: 0.5; '
            f'width: {w}px; height: {h}px; size: {size}; ignore_event: true; align_x: Center; align_y: Center; '
            f'color: {color}; text: "{text}"; }}')

def col_label(cid, x, w, size, color, align, text):
    a = f"align_x: {align};" if align else ""
    return (f'#{cid}:label {{ {LABEL}; x: {x}px; anchor_y: 0.5; pivot_y: 0.5; '
            f'width: {w}px; height: 22px; size: {size}; {a} align_y: Center; color: {color}; text: "{text}"; }}')

def row(i):
    bg = "#ffffff08" if i % 2 == 1 else "#00000000"
    return f'''        #row{i}:color_icon_button {{
          width: 100%; height: {ROWH}px;
          color: #00000000;
          btn: {{ color: {bg}; hover: {{ color: #ffffff14; }} active: {{ color: #ffffff1f; }} }}
          #r{i}c0:label {{ {LABEL}; x: {C0_X}px; y: 0px; anchor_y: 0.5; pivot_y: 0.5; width: {C0_W}px; height: 24px; size: {FS}; ignore_event: true; align_y: Center; color: #e8e8e8ff; text: ""; }}
          #r{i}face:image {{ x: {COL_FACE_X}px; anchor_y: 0.5; pivot_y: 0.5; width: {COL_FACE_W}px; height: {COL_FACE_W}px; visible: false; ignore_event: true; source: "asset/base/aseprite_resources/champions/demon#sheet"; }}
          #r{i}mname:label {{ {LABEL}; x: {COL_NAME_X}px; y: 0px; anchor_y: 0.5; pivot_y: 0.5; width: {COL_NAME_W}px; height: 24px; size: {FS}; ignore_event: true; align_y: Center; color: #e8e8e8ff; text: ""; }}
          #r{i}bg:color {{ x: {COL_TIER_X}px; y: 0px; anchor_y: 0.5; pivot_y: 0.5; width: {COL_TIER_W}px; height: 23px; ignore_event: true; color: #00000000; rounding: Uniform {{ rounding: 5; }}
            #r{i}tt:label {{ {BOLD}; width: {COL_TIER_W}px; height: 23px; anchor_y: 0.5; pivot_y: 0.5; size: {FS_TT}; ignore_event: true; align_x: Center; align_y: Center; color: #ffffffff; text: ""; }}
          }}
          #r{i}sc:label {{ {BOLD}; x: {COL_SC_X}px; y: 0px; anchor_y: 0.5; pivot_y: 0.5; width: {COL_SC_W}px; height: 24px; size: {FS_SC}; ignore_event: true; align_x: Right; align_y: Center; color: #7ee6d1ff; text: ""; }}
          #r{i}wr:label {{ {LABEL}; x: {COL_WR_X}px; y: 0px; anchor_y: 0.5; pivot_y: 0.5; width: {COL_WR_W}px; height: 24px; size: {FS_SC}; ignore_event: true; align_x: Right; align_y: Center; color: #e8e8e8ff; text: ""; }}
          #r{i}pr:label {{ {LABEL}; x: {COL_PR_X}px; y: 0px; anchor_y: 0.5; pivot_y: 0.5; width: {COL_PR_W}px; height: 24px; size: {FS_SC}; ignore_event: true; align_x: Right; align_y: Center; color: #a3a9b6ff; text: ""; }}
          #r{i}p5:label {{ {LABEL}; x: {COL_P5_X}px; y: 0px; anchor_y: 0.5; pivot_y: 0.5; width: {COL_P5_W}px; height: 24px; size: {FS_SC}; ignore_event: true; align_x: Right; align_y: Center; color: #a3a9b6ff; text: ""; }}
          #r{i}val:label {{ {BOLD}; x: {VAL_X}px; y: 0px; anchor_y: 0.5; pivot_y: 0.5; width: {VAL_W}px; height: 24px; size: {FS_SC}; ignore_event: true; align_x: Right; align_y: Center; color: #e8e8e8ff; text: ""; }}
        }}'''

# 탭바: 5버튼 + 5오버레이라벨 (아이콘: 통계/챔프/분석/코치/모의밴픽). 팝업폭 816-20=796 안에 5탭.
NUM_TABS = 5
TABW = 152   # 152*5 + 5*4 = 780 ≤ 796
tabx = [10 + (TABW + 5) * i for i in range(NUM_TABS)]
TAB_ICONS = ["statistics", "champion_info", "scouting", "coach", "coach"]
tabbtns = "\n".join(btn(f"tab{i}", tabx[i], TABW, 36, 16, TAB_ICONS[i]) for i in range(NUM_TABS))
tablbls = "\n".join(olabel(f"tab{i}l", tabx[i], TABW, 36, 15, c, t) for i, (t, c) in enumerate(
    [("메타통계", "#37d5b3ff"), ("챔피언정보", "#a3a9b6ff"), ("메타해석", "#a3a9b6ff"), ("밴픽코치", "#a3a9b6ff"), ("모의밴픽", "#a3a9b6ff")]))

# 데이터 범위 세그먼트(헤더 우측, 리프레시 왼쪽): 전체/대회/솔랭 (아이콘 all/crown/solorank).
SCOPEW = sx(74)
scopex = [sx(232) + (SCOPEW + 4) * i for i in range(3)]
SCOPE_ICONS = ["all", "crown", "solorank"]
def sbtn(bid, x, icon):
    return f'''    #{bid}:color_icon_button {{
      x: {x}px; anchor_y: 0.5; pivot_y: 0.5; width: {SCOPEW}px; height: 28px;
      hover_sound: "asset/base/sound/sfx/UI_mouse_hover";
      click_sound: "asset/base/sound/sfx/UI_mouse_click";
      color: #00000000;
      btn: {{ color: #ffffff0f; hover: {{ color: #ffffff22; }} active: {{ color: #37d5b344; }} rounding: Uniform {{ rounding: 5; }} }}
      icon: {{ source: "asset/base/ui/icons/{icon}"; rect: {{ x: 5; y: 6; w: 15; h: 15; }} }}
    }}'''
scopebtns = "\n".join(sbtn(f"scope{i}", scopex[i], SCOPE_ICONS[i]) for i in range(3))
scopelbls = "\n".join(
    f'    #scope{i}l:label {{ {BOLD}; x: {scopex[i]+10}px; anchor_y: 0.5; pivot_y: 0.5; width: {SCOPEW-12}px; height: 28px; size: 14; ignore_event: true; align_x: Center; align_y: Center; color: {c}; text: "{t}"; }}'
    for i, (t, c) in enumerate([("전체", "#37d5b3ff"), ("대회", "#a3a9b6ff"), ("솔랭", "#a3a9b6ff")]))

# 역할바: 6버튼 + 6라벨 (포지션 아이콘 all/top/jungle/mid/bottom/support)
ROLEW = sx(107)
rolex = [8 + (ROLEW + 3) * i for i in range(6)]
ROLE_ICONS = ["all", "top", "jungle", "mid", "bottom", "support"]
rolebtns = "\n".join(btn(f"role{i}", rolex[i], ROLEW, 30, 16, ROLE_ICONS[i]) for i in range(6))
rolelbls = "\n".join(olabel(f"role{i}l", rolex[i], ROLEW, 30, 15, c, t) for i, (t, c) in enumerate(
    [("전체", "#37d5b3ff"), ("탑", "#a3a9b6ff"), ("정글", "#a3a9b6ff"), ("미드", "#a3a9b6ff"), ("바텀", "#a3a9b6ff"), ("서포터", "#a3a9b6ff")]))

# 챔프 서브탭바: 4버튼 + 4라벨 (아이콘 chart/game_info/battle/time)
CSUBW = sx(160)
csubx = [8 + (CSUBW + 4) * i for i in range(4)]
CSUB_ICONS = ["chart", "game_info", "battle", "time"]
csubbtns = "\n".join(btn(f"csub{i}", csubx[i], CSUBW, 30, 16, CSUB_ICONS[i]) for i in range(4))
csublbls = "\n".join(olabel(f"csub{i}l", csubx[i], CSUBW, 30, 15, c, t) for i, (t, c) in enumerate(
    [("통계", "#37d5b3ff"), ("기본정보", "#a3a9b6ff"), ("상대·빌드", "#a3a9b6ff"), ("패치", "#a3a9b6ff")]))

# 밴픽코치 슬롯 바(밴픽코치 탭에서만; 역할바와 같은 위치): 상대밴/내밴/상대픽/내픽 → 탭하면 그 슬롯 픽 모드.
dslotx = [8 + (CSUBW + 4) * i for i in range(4)]
dslotbtns = "\n".join(btn(f"dslot{i}", dslotx[i], CSUBW, 30, 16) for i in range(4))
dslotlbls = "\n".join(olabel(f"dslot{i}l", dslotx[i], CSUBW, 30, 15, c, t) for i, (t, c) in enumerate(
    [("상대 밴", "#a3a9b6ff"), ("내 밴", "#a3a9b6ff"), ("상대 픽", "#a3a9b6ff"), ("내 픽", "#a3a9b6ff")]))

# 밴픽코치 챔프 그리드(밴픽코치 탭 전용, #list 상단): 8열×12행=96셀. 셀=클릭버튼 + 하이라이트bg + 얼굴.
#   ↑ 72셀은 챔프 수(base+모드) 부족 → 96으로(8열 유지, 행만 12로). 높이는 런타임 동적(빈 줄 축소).
GCOLS, GROWS = 10, 12   # 가로 1.2배(리스트 725px) → 열 수 8→10 (셀 71px 유지, 10*71=710 ≤ 725)
GC_W, GC_H = 71, 40
NGRID = GCOLS * GROWS
def gcell(k):
    cx = 2 + (k % GCOLS) * GC_W
    cy = 2 + (k // GCOLS) * GC_H
    return (f'          #gcell{k}:color_icon_button {{ x: {cx}px; y: {cy}px; width: {GC_W-3}px; height: {GC_H-3}px; '
            f'hover_sound: "asset/base/sound/sfx/UI_mouse_hover"; click_sound: "asset/base/sound/sfx/UI_mouse_click"; color: #00000000; '
            f'btn: {{ color: #00000000; hover: {{ color: #ffffff1f; }} active: {{ color: #ffffff33; }} rounding: Uniform {{ rounding: 5; }} }} '
            f'icon: {{ source: "asset/base/ui/icons/statistics"; rect: {{ x: 0; y: 0; w: 0; h: 0; }} }}\n'
            f'            #gbg{k}:color {{ width: 100%; height: 100%; ignore_event: true; color: #00000000; rounding: Uniform {{ rounding: 5; }} }}\n'
            f'            #gimg{k}:image {{ anchor_x: 0.5; pivot_x: 0.5; anchor_y: 0.5; pivot_y: 0.5; width: 30px; height: 30px; visible: false; ignore_event: true; source: "asset/base/aseprite_resources/champions/demon#sheet"; }}\n'
            f'          }}')
gcells = "\n".join(gcell(k) for k in range(NGRID))
pgrid = (f'          #pgrid:color {{ width: 100%; height: {GROWS*GC_H + 6}px; color: #00000000; visible: false;\n'
         f'{gcells}\n          }}')

# ── 모의밴픽(탭4) 컨트롤 패널: 토글버튼(세트/룰/밴수/진영) + 가중치 6×[-][+] + 패치보정. #list 최상단, 탭4만 표시. ──
#   활성 하이라이트·값 라벨은 모드가 #BPSTATE 를 파싱해 매프레임 갱신.
def pbtn(bid, x, y, w, h, text, size=14):
    return (f'          #{bid}:color_icon_button {{ x: {x}px; y: {y}px; width: {w}px; height: {h}px; '
            f'hover_sound: "asset/base/sound/sfx/UI_mouse_hover"; click_sound: "asset/base/sound/sfx/UI_mouse_click"; color: #00000000; '
            f'btn: {{ color: #ffffff12; hover: {{ color: #ffffff28; }} active: {{ color: #ffffff3c; }} rounding: Uniform {{ rounding: 5; }} }} }}\n'
            f'          #{bid}l:label {{ {BOLD}; x: {x}px; y: {y}px; width: {w}px; height: {h}px; size: {size}; ignore_event: true; align_x: Center; align_y: Center; color: #a3a9b6ff; text: "{text}"; }}')
def plabel(lid, x, y, w, text, color="#8891a0ff", size=13, align="Left"):
    return (f'          #{lid}:label {{ {BOLD}; x: {x}px; y: {y}px; width: {w}px; height: 26px; size: {size}; ignore_event: true; align_x: {align}; align_y: Center; color: {color}; text: "{text}"; }}')
_bpp = []
# Row1 y=4: 세트(1-5) + 룰(3)   ※ x/width 전부 가로 스케일(sx)
_bpp.append(plabel("bp_setlbl", 0, 4, sx(34), "세트"))
for i in range(5): _bpp.append(pbtn(f"bpset{i}", sx(38) + sx(38) * i, 4, sx(34), 26, str(i + 1)))
_bpp.append(plabel("bp_rulelbl", sx(238), 4, sx(30), "룰"))
for i, t in enumerate(["일반", "피어리스", "하드"]): _bpp.append(pbtn(f"bprule{i}", sx(272) + sx(70) * i, 4, sx(66), 26, t))
# Row2 y=34: 밴수(4/6) + 내진영
_bpp.append(plabel("bp_banlbl", 0, 34, sx(34), "밴수"))
for i, t in enumerate(["4밴", "6밴"]): _bpp.append(pbtn(f"bpban{i}", sx(38) + sx(52) * i, 34, sx(48), 26, t))
_bpp.append(plabel("bp_sidelbl", sx(168), 34, sx(54), "내진영"))
_bpp.append(pbtn("bpside", sx(224), 34, sx(96), 26, "블루"))
# 가중치 rows y=68,100,132 (2열): 각 [라벨][-][값][+]
_WL = [("stat", "단순통계"), ("solo", "솔랭"), ("meta", "메타"), ("syn", "시너지"), ("game", "게임학습"), ("ctr", "카운터")]
def _wcell(key, label, x, y):
    return "\n".join([
        plabel(f"bpw_{key}lbl", x, y, sx(70), label, "#a3a9b6ff"),
        pbtn(f"bpwdec_{key}", x + sx(74), y, sx(26), 26, "−", 18),
        plabel(f"bpw_{key}val", x + sx(102), y, sx(46), "0", "#e8e8e8ff", 14, "Center"),
        pbtn(f"bpwinc_{key}", x + sx(150), y, sx(26), 26, "+", 16),
    ])
for r in range(3):
    _bpp.append(_wcell(_WL[r * 2][0], _WL[r * 2][1], 0, 68 + 32 * r))
    _bpp.append(_wcell(_WL[r * 2 + 1][0], _WL[r * 2 + 1][1], sx(300), 68 + 32 * r))
# 패치보정 (챔피언 그리드 바로 위) y=166
_bpp.append(pbtn("bppatch", 0, 166, sx(220), 28, "패치보정 ON", 14))
BPP_H = 200
bppanel = (f'          #bppanel:color {{ width: 100%; height: {BPP_H}px; color: #00000000; visible: false;\n'
           + "\n".join(_bpp) + "\n          }")

# 그리드를 헤더(1줄)+슬롯(4줄) 밑, 상대밴성향 위에 두기 위해 rows를 상단5(row0~4)/나머지로 분할.
rows_top = "\n".join(row(i) for i in range(5))
rows_rest = "\n".join(row(i) for i in range(5, NROWS))
colhead = "\n          ".join([
    col_label("ch_no", 10, 30, FS_HEAD, "#6f7686ff", "", "No"),
    col_label("ch_name", COL_NAME_X, COL_NAME_W, FS_HEAD, "#6f7686ff", "", "챔피언"),
    col_label("ch_tier", COL_TIER_X, COL_TIER_W, FS_HEAD, "#6f7686ff", "Center", "티어"),
    col_label("ch_sc", COL_SC_X, COL_SC_W, FS_HEAD, "#6f7686ff", "Right", "점수"),
    col_label("ch_wr", COL_WR_X, COL_WR_W, FS_HEAD, "#6f7686ff", "Right", "승률"),
    col_label("ch_pr", COL_PR_X, COL_PR_W, FS_HEAD, "#6f7686ff", "Right", "픽률"),
    col_label("ch_p5", COL_P5_X, COL_P5_W, FS_HEAD, "#6f7686ff", "Right", "표본"),
])

# ── 클릭 차단막 (ghidra 확정) ──
#   히트테스트는 z 무관·순수 DFS 순서. 승자 = 점을 포함하는 '가장 나중' 노드 1개(소비, 전파 없음).
#   color_icon_button 은 block_event()==true 로 **무조건 히트 후보**(ignore_event 조차 무시).
#   rect w/h==0 이면 후보에서 탈락 → width:100% 대신 **명시 px** 로 준다.
#   팝업의 첫 자식 → 우리 콘텐츠(뒤에 선언)가 DFS 뒤라 차단막을 이김 → 우리 클릭은 정상.
#   ⚠ z 는 쓰지 않는다(히트에 무의미). 렌더 z(300)는 다른 노드에만.
#   헤더(상단 44px)는 덮지 않는다 → draggable_popup 드래그 핸들 보존(차단막이 mouse-down 을 먹으면 드래그가 죽는다).
HDR_H = 44
ov_blocker = (f'  #ov_blocker:color_icon_button {{ x: 0px; y: {HDR_H}px; width: {W}px; height: {H - HDR_H}px; '
              f'btn: {{ color: #00000000; hover: {{ color: #00000000; }} active: {{ color: #00000000; }} }} }}')

ui = f'''draft_root:empty {{
  width: 100%;
  height: 100%;

  #draft_overlay:draggable_popup {{
  anchor_x: 1;
  pivot_x: 1;
  anchor_y: 0;
  pivot_y: 0;
  x: -36px;
  y: 64px;
  width: {W}px;
  height: {H}px;
  visible: true;
  back_color: #00000000;

{ov_blocker}
  #ov_bg:color {{ x: 0px; y: 0px; width: 100%; height: 100%; ignore_event: true; color: #161721ff; rounding: Uniform {{ rounding: 8; }} }}

  #header:color {{
    width: 100%;
    height: 44px;
    color: #0f1016ff;
    ignore_event: true;
    rounding: Individual {{ top_left: 8; top_right: 8; }}
    #title:label {{
      {BOLD};
      x: 18px; anchor_y: 0.5; pivot_y: 0.5;
      width: {sx(210)}px; height: 28px; size: 18; align_y: Center;
      color: #37d5b3ff;
      text: "TFM2.gg 메타 분석";
    }}
{scopebtns}
{scopelbls}
    #refresh_btn:color_icon_button {{
      anchor_x: 1; pivot_x: 1; x: -48px; anchor_y: 0.5; pivot_y: 0.5; width: 106px; height: 28px;
      hover_sound: "asset/base/sound/sfx/UI_mouse_hover";
      click_sound: "asset/base/sound/sfx/UI_mouse_click";
      color: #00000000;
      btn: {{ color: #ffffff0f; hover: {{ color: #37d5b344; }} active: {{ color: #37d5b366; }} rounding: Uniform {{ rounding: 6; }} }}
      icon: {{ source: "asset/base/ui/icons/refresh"; rect: {{ x: 8; y: 7; w: 14; h: 14; }} }}
    }}
    #refresh_lbl:label {{
      {BOLD}; anchor_x: 1; pivot_x: 1; x: -48px; anchor_y: 0.5; pivot_y: 0.5;
      width: 106px; height: 24px; size: 14; ignore_event: true; align_x: Center; align_y: Center;
      color: #a3a9b6ff; text: "새로고침";
    }}
    #close_btn:color_icon_button {{
      anchor_x: 1; pivot_x: 1; x: -10px; anchor_y: 0.5; pivot_y: 0.5; width: 30px; height: 28px;
      hover_sound: "asset/base/sound/sfx/UI_mouse_hover";
      click_sound: "asset/base/sound/sfx/UI_mouse_click";
      color: #00000000;
      btn: {{ color: #ffffff14; hover: {{ color: #ef4b5f88; }} active: {{ color: #ef4b5fbb; }} rounding: Uniform {{ rounding: 6; }} }}
      icon: {{ source: "asset/base/ui/icons/cross"; rect: {{ x: 8; y: 7; w: 14; h: 14; }} }}
      #close_icon:image {{ ignore_event: true; x: 8px; y: 7px; width: 14px; height: 14px; source: "asset/base/ui/icons/cross"; color: #ef6b6bff; }}
    }}
  }}

  #tabbar:empty {{
    margin: {{ top: 50px; left: 10px; right: 10px; }}
    width: 100%; height: 36px;
{tabbtns}
{tablbls}
  }}

  #rolebar:empty {{
    margin: {{ top: 92px; left: 10px; right: 10px; }}
    width: 100%; height: 30px;
{rolebtns}
{rolelbls}
  }}

  #csub:empty {{
    margin: {{ top: 92px; left: 10px; right: 10px; }}
    width: 100%; height: 30px;
    visible: false;
{csubbtns}
{csublbls}
  }}


  #body:color {{
    margin: {{ top: 130px; left: 12px; right: 12px; bottom: 12px; }}
    width: 100%; height: 100%;
    color: #0f1016ff;
    rounding: Uniform {{ rounding: 8; }}

    #colhead:color {{
      margin: {{ top: 8px; left: 16px; right: 16px; }}
      width: 100%; height: 24px;
      color: #00000000;
      {colhead}
    }}

    #scrollzone:color {{
      margin: {{ top: 36px; left: 4px; right: 4px; }}
      width: 100%; height: 626px;
      color: #00000000;

      #sbar_track:color {{ anchor_x: 1; pivot_x: 1; x: -2px; y: 0px; width: 6px; height: 626px; ignore_event: true; visible: false; color: #ffffff10; rounding: Uniform {{ rounding: 3; }}
        #sbar_thumb:color {{ x: 0px; y: 0px; width: 6px; height: 60px; ignore_event: true; color: #37d5b3cc; rounding: Uniform {{ rounding: 3; }} }}
      }}

      #scroll:scroll_view {{
        width: 100%; height: 626px;
        speed: 60; bar_width: 7;
        bar: {{ source: "asset/base/sprite/white"; color: #37d5b3ff; hover: {{ color: #ecfbf8ff; }} }}
        back: {{ source: "asset/base/sprite/white"; color: #4a4c56ff; }}

        #list:empty {{
          margin: {{ left: 12px; right: 20px; }}
          width: {LIST_W}px; height: {NROWS*31 + 34}px; y: 4px;
          child_type: TopToBottom {{ spacing: 1px; }}

          #status:label {{ {LABEL}; width: 100%; height: 22px; size: 13; align_y: Center; color: #6f7686ff; text: "데이터 대기 중… (TFM2.gg 를 켜세요)"; }}
{bppanel}
{rows_top}
{pgrid}
{rows_rest}
        }}
      }}
    }}
  }}

  #champ_img:image {{
    x: {sx(195)}px; y: 130px;
    width: 88px; height: 88px;
    visible: false;
    ignore_event: true;
    source: "asset/base/aseprite_resources/champions/demon#sheet";
  }}
  }}

  #draft_toggle:color_icon_button {{
    anchor_x: 1; pivot_x: 1; anchor_y: 0; pivot_y: 0;
    x: -136px; y: 7px; width: 96px; height: 36px;
    hover_sound: "asset/base/sound/sfx/UI_mouse_hover";
    click_sound: "asset/base/sound/sfx/UI_mouse_click";
    color: #00000000;
    btn: {{ color: #37d5b3dd; hover: {{ color: #37d5b3ff; }} active: {{ color: #2aa88fff; }} rounding: Uniform {{ rounding: 6; }} }}
    icon: {{ source: "asset/base/ui/icons/coach"; rect: {{ x: 0; y: 0; w: 0; h: 0; }} }}
  }}
  #draft_toggle_lbl:label {{
    {BOLD}; anchor_x: 1; pivot_x: 1; anchor_y: 0; pivot_y: 0;
    x: -136px; y: 7px; width: 96px; height: 36px; size: 15; ignore_event: true;
    align_x: Center; align_y: Center; color: #0f1016ff; text: "대시보드";
  }}
}}
'''

import io
# ── z-order ── ★z 는 전역 드로우 정렬키(기본 100)이며 자식에게 상속되지 않는다(ghidra 규명).
#   루트에만 z 를 주면 투명 사각형 하나만 올라가고, 실제로 그려지는 자식들은 z=100 → 게임 밴픽
#   호버 툴팁(z=232/233)에 덮인다. 게임도 서브트리를 올릴 때 자식마다 z 를 박는다.
#   ⟹ 조각의 모든 노드에 z 부여. 단 scroll_view 는 게임 .ui 에 z 사용례가 없어 파서 ERROR 위험 → 제외.
Z_TOP = 300   # 툴팁(233)·champion_tooltip(231)·stats_popup(200) 위 / 로딩 오버레이(20000) 아래
import re as _re
def _apply_z(text, z=Z_TOP):
    def _r(m):
        if m.group(1) == "#ov_blocker":
            return m.group(0)
        # scroll_view 도 z 를 준다: 안 주면 z=100 이라 우리 배경(z=300)이 스크롤바를 덮는다.
        return "%s:%s { z: %d;" % (m.group(1), m.group(2), z)
    return _re.sub(r"(#?[A-Za-z0-9_]+):([a-z_]+)\s*\{", _r, text)
ui = _apply_z(ui)

with io.open(r"C:\tfm2mods\tfm2_draft_overlay\ui_inject\draft_popup.ui", "w", encoding="utf-8") as f:
    f.write(ui)
print("generated v3 (big): W=%d H=%d rows=%d bytes=%d" % (W, H, NROWS, len(ui)))
