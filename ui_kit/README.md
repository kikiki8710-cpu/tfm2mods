# ui_kit — TFM2 UI 조작 헬퍼 (단일 모듈)

`ui_kit.rs` 하나로 TFM2 모드에서 라벨/색/토글/슬라이더/입력창/표시/클릭/드롭다운/목록을
**함수·타입 호출**로 다룬다. 모드가 아니라 **라이브러리 모듈**(declare_mod 없음).

## 불러오기
다른 모드 `lib.rs` 맨 위:
```rust
use mod_api::*;
#[path = "C:/tfm2mods/ui_kit/ui_kit.rs"]   // 복사 없이 공유 (추천)
mod ui_kit;
use ui_kit::*;
```
또는 `ui_kit.rs` 를 모드 `src/` 에 복사 후 `mod ui_kit; use ui_kit::*;`.
의존성은 `mod_api` 뿐 → 기본 `build_mod.bat` 그대로 빌드.

## 대원칙 (엔진 제약)
1. **코드로 노드 push 하면 렌더 안 됨.** UI는 `.ui` override 에 *미리 선언* → 코드는 찾아서 값/visible 만 바꾼다.
2. **클릭 이벤트는 `Click`/`RightClick` 뿐** (hover/press 없음). "클릭→색"은 *상태 변경→매 프레임 색 반영* 으로.
3. **filter 클로저는 노드를 못 만짐** → 클릭은 상태(Atomic)만 바꾸고, 시각은 post_update 가 반영. (Frame 이 캡슐화)
4. 오프셋은 SDK rlib=실게임 기준. 게임 업데이트 시 `C:\tfm2mods\ui_offset_probe\offsets2.rs` 재실행으로 갱신.
5. **런타임 레이아웃 override 는 비인터랙티브 노드에만.** `scroll_view`/`color_selectable`/`button` 같은 인터랙티브 노드의 width/height 를 런타임에 덮으면 **hover/상태전이 때 .ui 선언값으로 되돌아간다** → "호버하면 크기·스타일이 튐" 버그. 크기를 바꿔야 하면 `color`/`empty`/`label` 노드만 건드리고, 인터랙티브 노드는 `.ui` 고정값 또는 `width:100%`(부모 추종) 으로. (set_layout_* 헬퍼 주의표시 참고)
6. **모드 텍스트(i18n) 는 `mod.override_info` 등록 필수.** `text/*.i18n` 파일을 폴더에 두는 것만으론 무시됨. `"asset/base/text/<name>": { "remapping":"asset/<NS>/text/<name>", "type":"merge" }` 등록해야 로드. `merge` 면 바닐라 보존+추가, `override` 면 통째 교체. 형식: `{ "<lang>": { "<key>": { ... } } }`.

## API 요약

### 탐색
- `find(&Node, id) -> Option<&Node>`, `find_mut(&mut Node, id)`, `kind(&Node) -> &str`, `self_check(root, known_id)`

### 라벨 / 텍스트
- `label_get`, `label_set`, `label_set_color(Rgba)`, `label_set_size(f32)`
- `text_set_deep(n, s)` — 노드 또는 첫 자손 라벨에 텍스트

### 색 (`:color` 사각형)
- `rect_set_back(Rgba)`, `rect_set_color(Rgba)`, `rect_set_back_all(Rgba)`(4상태 강제)
- `Rgba::hex(0xRRGGBBAA)`, `Rgba::rgb8(r,g,b)`, `Rgba::new(r,g,b,a)`

### 표시
- `set_visible`, `show`, `hide`, `get_visible`, `set_visible_by_id(root,id,bool)`

### 토글버튼 (Checkbox/Selectable/ColorSelectable 자동판별)
- `toggle_get -> Option<bool>`, `toggle_set(bool)`, `toggle_flip`, `toggle_text_set(s)`

### 슬라이더 / 스크롤 / 입력창
- `slider_get/set(0..1)`, `scroll_get`, `scroll_set(0..1)`, `textedit_get/set`

### 레이아웃 (위치·크기 직접 쓰기)
- `set_layout_xy(n,x,y)`, `set_layout_size(n,w,h)` (0이하 축은 유지)
- `set_layout_xy_by_id(root,id,x,y)`, `set_layout_size_by_id(root,id,w,h)` — id로 찾아 적용, 찾으면 true
- ⚠️ **px 선언 노드에만.** `width:100%`(Percent) 노드에 px 쓰면 깨짐. **인터랙티브 노드 크기는 런타임 변경 금지**(대원칙 5).

### 클릭 (filter_handler)
- `ensure_clicks(ui, &LAST, routes)` + `route(id, Rc<dyn Fn()>)` — 저수준(재등록 포함)

### 목록
- `fill_list(root, prefix, max_rows, &items) -> usize` — `{prefix}0..N` 행에 데이터 1:1 + 남는 행 hide

### Frame — 클릭+시각 일괄 (권장)
한 프레임의 클릭/색을 모아 `apply` 한 번에. ensure_clicks 를 내부에서 1회만 호출.
```rust
static ON: AtomicBool = AtomicBool::new(false);
static TAB: AtomicUsize = AtomicUsize::new(0);
static FH: AtomicUsize = AtomicUsize::new(usize::MAX);

let mut f = Frame::new();
f.button("ok_btn", Rc::new(|| { /* 실행 */ }));
f.toggle_color("box", &ON, Rgba::hex(0x37d5b3ff), Rgba::hex(0x1d1f2cff)); // 클릭=색토글
f.select(&["tab0","tab1","tab2"], &TAB, Rgba::hex(0x124f43ff), Rgba::hex(0x1d1f2cff)); // 탭/라디오
f.apply(ui, &FH);
```

### Dropdown — 커스텀 드롭다운 (완전제어)
네이티브 DropdownRunner(private)를 피하고 `color_selectable` 행으로 구성. `.ui` 선언 필요.
```rust
static OPEN: AtomicBool = AtomicBool::new(false);
static SEL: AtomicUsize = AtomicUsize::new(0);
static FH: AtomicUsize = AtomicUsize::new(usize::MAX);
const ITEMS: [&str;3] = ["1배속","2배속","3배속"];
const IDS: [&str;3] = ["dd_item0","dd_item1","dd_item2"];
static DD: Dropdown = Dropdown {
    header_id:"dd_header", header_label_id:"dd_header_label", panel_id:"dd_panel",
    item_ids:&IDS, items:&ITEMS, open:&OPEN, sel:&SEL,
};
// post_update:
DD.update(ui, &FH);
let chosen = DD.selected();          // 선택 인덱스
```
짝이 되는 `.ui` 스니펫은 `examples/dropdown_example.rs` 상단 주석 참고.

### 네이티브 룩 스크롤 풀다운 (패턴 — 게임 기본 드롭다운과 동일 외형 + 스크롤)
위 `Dropdown` 의 확장형. 게임 네이티브 드롭다운처럼 보이게 `color_selectable @"asset/base/style/main#strategy_option"`(자동 hover/선택 민트) 행을 `scroll_view` 안에 넣고, 트리거 버튼 위에 겹쳐 띄운다. 항목 많으면 스크롤, 적으면 높이 축소. **레퍼런스 구현 = `tfm2_scrim/src/lib.rs`** 의 `open_dropdown`/`fill_page`/`select_dropdown`/`set_pulldown_contents` (DD_* 상태 + 트리거별 위치계산). ui_kit 단독 컴포넌트로는 미추출(트리거 종류·위치가 모드마다 달라서) — 아래 규칙대로 복제.

**.ui 구조 (핵심):**
```
#dd_root:empty { width:100%; height:100%; visible:false;
  #dd_close:color_icon_button { width:100%; height:100%; btn:{ color:#00000000; } }   // 바깥클릭=닫기(투명)
  #dd_panel:color { width:240px; height:284px; color:#4a4c56ff; stroke:1; back_color:#1d1f2cff; rounding:Uniform{rounding:8;}
    #dd_block:button { width:100%; height:100%; }                                       // 패널 클릭 흡수
    #dd_list:scroll_view { anchor_x:0.5; pivot_x:0.5; y:10px; width:228px; height:264px; speed:100;
        bar_width:4; bar:{ source:"asset/base/sprite/white"; color:#37d5b3ff; hover:{color:#ecfbf8ff;} }
        back:{ source:"asset/base/sprite/white"; color:#00000000; }                      // 트랙 투명(빈 띠 제거)
      #dd_contents:empty { anchor_x:0.5; pivot_x:0.5; width:216px; height:960px; child_type:TopToBottom{spacing:6px;}
        #dd_00:color_selectable { @"asset/base/style/main#strategy_option"; width:100%; height:40px; label:{size:18;} selected_label:{size:18;} visible:false; }
        // … 필요 최대 항목수만큼 행 노드(예: 40개). 부족하면 페이지네이션 필요.
      }
    }
  }
}
```
**런타임 규칙 (대원칙 5 적용 — 이게 핵심):**
- 행 채우기: `find_mut(root,"dd_NN")` → `toggle_text_set(n, name)`(텍스트) + `toggle_set(n, is_current)`(민트 하이라이트) + `n.visible=true`. 남는 행 `visible=false`.
- **인터랙티브 노드(list/행)는 폭·크기 런타임 override 금지.** 행은 `.ui` 에서 `width:100%` → 부모 추종. list 는 `.ui` 고정.
- 런타임으로 바꾸는 건 **비인터랙티브만**: `dd_contents`(empty) 높이 = 항목수×46(행40+간격6), `dd_panel`(color) 위치 `set_layout_xy_by_id` + (항목 적을 때) 높이.
- **항목 ≤5: 패널/리스트 높이 = 내용(`len*46-6`)+여백** 으로 축소(빈 공간·스크롤 영역 제거). **>5: 선언값(264/284) 유지** + 스크롤. (공용 노드면 >5 경우 반드시 선언값으로 되돌릴 것.)
- 위치: 트리거 버튼의 화면 좌상단에 `dd_panel` 을 겹침. 모달이 화면중앙이면 패널좌상단 오프셋(예 1120×640 → (400,220)) + 트리거 상대좌표. 폭은 240 고정 통일(트리거별 폭 매칭은 인터랙티브 override 필요 → 버그원천, 비권장).
- 외형 디테일: 트랙 `back` 투명, `dd_contents` `anchor_x:0.5`(좌우대칭), 행 간격 0 주면 "버튼 사이 hover" 영역 제거(붙여 표시).
- 닫기: `dd_close`(전체화면 투명버튼)가 바깥클릭 흡수 → 상태 false. 패널 위는 `dd_block` 이 흡수.

### DraggableWindow — 클릭드래그 창 이동
WinAPI 폴링(`GetCursorPos`+`GetAsyncKeyState`)으로 드래그를 감지, `Node.layout.x/y` 를 직접 써서 창을 움직인다.
마우스 좌표는 게임 창 클라이언트영역 + 16:9 레터박스 보정으로 게임 가상좌표(1920x1080)에 매핑 → 창모드/임의 해상도 대응.
```rust
//                                    node_id          handle  panel_w panel_h  bounds(x1,y1,x2,y2)
static DRAG: DraggableWindow = DraggableWindow::new("win_root", "", 400.0, 200.0, (0.0,0.0,1920.0,1080.0));
// post_update:
DRAG.update(&mut ui.root, &["C:\\mod\\dir"]);   // log_dirs: 디버그 로그 폴더(빈 슬라이스면 로그X)
```
- `node_id`: 위치를 바꿀 컨테이너 (`.ui` 에 `x:N y:N` px 고정값으로 선언 필수)
- `handle_id`: 드래그 잡는 영역 (보통 헤더바 노드 id). `""` 이면 창 전체가 핸들.
- `panel_w/panel_h`: 패널 크기(히트테스트 + 경계 clamp 용). `.ui` 의 width/height 와 일치시킬 것.
- `bounds=(x1,y1,x2,y2)`: 패널이 벗어날 수 없는 사각 영역(게임 가상좌표). 패널 우/하단이 (x2,y2)를 넘지 않게 clamp. **`(0,0,0,0)` 이면 제한 없음.**
- 드래그 on/off 토글: `DRAG.set_enabled(bool)` / `DRAG.is_enabled()` (런타임, 기본 on).
- 위치 강제: `DRAG.set_pos(&mut ui.root, x, y, log_dirs);` (bounds 로 clamp 됨).
- 좌표 진단: `DRAG.debug_label(root, "some_label_id", limit)` — 클라이언트크기/마우스/게임좌표/노드좌표를 라벨에 출력(파일 I/O 불가 환경용).

**⚠️ 게임 mod DLL 은 `std::fs` 파일쓰기가 막혀있다(검증됨).** 디버그는 파일 대신 `debug_label`(UI 라벨)로.
**⚠️ DLL 배포 경로** — `build_mod.bat` 은 `rustc -o <stem>.dll` 상대경로라 산출물이 **현재 작업 디렉토리**에 생긴다. SDK 폴더의 동명 옛 dll 을 복사하지 말 것. (Spectator_Chat 은 게임이 `Spectator_Chat.dll` 이름으로 읽음.)

**⚠️ LENGTH_F32_OFF** — Length enum 내 f32 위치는 런타임 자동감지(`LEN_OFF`, 0 또는 4). 좌표가 이상하면 감지로직 확인.

## 컴포넌트 옵션 레퍼런스
함수화된 고수준 컴포넌트(`Frame` / `Dropdown` / `DraggableWindow` / `fill_list`)의 파라미터·옵션 한눈에.

### Frame — 클릭+시각 일괄 (체이닝)
매 프레임 `Frame::new()` → 메서드 체이닝 → `.apply()`. `static FH: AtomicUsize = AtomicUsize::new(usize::MAX)` 하나를 재등록 추적용으로 넘긴다.

| 메서드 | 시그니처 | 동작 / 옵션 |
|---|---|---|
| `new()` | `() -> Frame` | 빈 프레임 생성 |
| `button` | `(id, cb: ClickFn)` | 클릭 시 `cb` 실행. `cb = Rc::new(\|\| {...})` |
| `toggle_color` | `(id, &'static AtomicBool, on: Rgba, off: Rgba)` | 클릭마다 bool 뒤집고 그 상태로 `:color` 배경색(on/off) |
| `toggle_flag` | `(id, &'static AtomicBool)` | 클릭마다 bool 토글만(색 X). 시각은 직접 반영 |
| `select` | `(&[ids], &'static AtomicUsize, hi: Rgba, base: Rgba)` | 라디오/탭: 선택된 것만 `hi`, 나머지 `base` |
| `on_click` | `(id, cb)` | `button` 별칭(저수준 라우트) |
| `apply` | `(ui: &mut GameUI, &FH)` | 클릭 1회 재등록 + 모든 시각 반영 |
| `selected` | `() -> usize` | (Frame 자체엔 없음; 상태는 직접 든 `AtomicUsize`로 읽음) |

- 색 옵션은 모두 `Rgba` → `Rgba::hex(0xRRGGBBAA)` / `rgb8(r,g,b)` / `new(r,g,b,a)`.
- **`:color` 노드에만** 색 적용(버튼스타일 노드는 hover색이 덮음).

### Dropdown — 커스텀 드롭다운
구조체 리터럴로 구성(필드 7개). `.ui` 에 header/header_label/panel/item0..N(color_selectable) 미리 선언 필수.

| 필드 | 타입 | 의미 |
|---|---|---|
| `header_id` | `&'static str` | 클릭=펼침 토글하는 헤더 노드 |
| `header_label_id` | `&'static str` | 현재 선택값 텍스트를 쓸 라벨 |
| `panel_id` | `&'static str` | 펼침 목록 컨테이너(visible 토글됨) |
| `item_ids` | `&'static [&str]` | 각 행 노드 id (color_selectable) |
| `items` | `&'static [&str]` | 각 행 표시 텍스트 (item_ids 와 1:1) |
| `open` | `&'static AtomicBool` | 펼침 상태 |
| `sel` | `&'static AtomicUsize` | 선택 인덱스 |

| 메서드 | 시그니처 | 동작 |
|---|---|---|
| `update` | `(ui: &mut GameUI, &FH)` | 클릭 재등록 + 펼침/선택/하이라이트/헤더값 반영 |
| `selected` | `() -> usize` | 선택 인덱스 |
| `selected_text` | `() -> &'static str` | 선택 항목 텍스트 |

- 이 `Dropdown` 은 단순형(스크롤·네이티브 외형 없음). **네이티브 룩 + 스크롤**이 필요하면 위 "네이티브 룩 스크롤 풀다운" 패턴 사용(`color_selectable @strategy_option` + `scroll_view`, 자동 hover/민트선택). 레퍼런스: `tfm2_scrim/src/lib.rs`.

### DraggableWindow — 클릭드래그 창 이동
| 항목 | 시그니처 | 의미 / 옵션 |
|---|---|---|
| `new` | `(node_id, handle_id, panel_w, panel_h, bounds:(x1,y1,x2,y2))` | 생성. 아래 파라미터 표 참고 |
| `update` | `(&mut Node /*root*/, log_dirs: &[&str])` | 매 프레임 호출(드래그 폴링+이동) |
| `set_pos` | `(&mut Node, x, y, log_dirs)` | 위치 강제(bounds 로 clamp) |
| `set_enabled` / `is_enabled` | `(bool)` / `() -> bool` | 드래그 on/off 런타임 토글(기본 on) |
| `is_dragging` | `() -> bool` | 현재 드래그 중인지 |
| `debug_label` | `(&mut Node, label_id, limit)` | 좌표 진단을 라벨에 출력(limit=0 항상) |

| `new` 파라미터 | 의미 |
|---|---|
| `node_id` | 위치 바꿀 컨테이너(`.ui` 에 `x:N y:N` px 고정 선언 필수) |
| `handle_id` | 드래그 잡는 영역. `""` → 창 전체가 핸들 |
| `panel_w/panel_h` | 패널 크기(히트테스트+clamp). `.ui` width/height 와 일치 |
| `bounds=(x1,y1,x2,y2)` | 못 벗어나는 사각영역(게임 가상좌표). **`(0,0,0,0)`=무제한** |

- 마우스→게임좌표: 클라이언트영역+16:9 레터박스 보정(창모드/임의 해상도 OK).
- 디버그는 파일 불가 → `debug_label` 사용.

### fill_list — 목록 1:1 채우기
| 시그니처 | 동작 |
|---|---|
| `fill_list(root, prefix, max_rows, &items) -> usize` | `{prefix}0..max_rows` 행에 `items` 텍스트 1:1, 남는 행 hide. 채운 개수 반환 |

## 주의
- 색 토글/선택은 **`:color` 노드**에. 버튼스타일 노드는 자체 hover/active 색이 덮음(필요시 `rect_set_back_all`, hover 피드백 손실).
- 토글류 `selected` 만 바꾸면 시각만 갱신, 게임 내부 onclick 연동은 안 탐 → **모드 자기 UI** 용.
- 모든 set 은 type_name 게이트 + (텍스트는) 레이아웃 검증 후 기록 → 어긋나면 자동 스킵(안전).

## 파일
- `ui_kit.rs` — 모듈 본체(이것만 import).
- `examples/dropdown_example.rs` — 사용 예(컴파일 대상 아님).
- 오프셋 근거: `팀파매2모드 분석/discovered-ui-runner-offsets.md`, 구조: `discovered-ui-architecture.md`.
