# 네이티브 스크롤 풀다운 (max_items_height) — 재사용 스니펫

게임의 "풀다운인데 항목이 많으면 스크롤되는" 드롭다운(상품 생산 모달의 상품 선택 등)은
**커스텀이 아니라 네이티브 `dropdown` 노드 + `max_items_height` 속성 한 줄**이다.
우리가 scrim에서 hover 재계산 버그와 싸우며 만든 커스텀 풀다운(color_selectable 기반,
`dropdown_example.rs`)은 사실 불필요했다. 이 스니펫이 정석.

- 발견: 2026-06-28. 게임 0.4.14 핫픽스.
- 관련 메모리: `tfm2-native-dropdown`, `tfm2-ui-runtime-layout`.

---

## 1. 비밀은 `max_items_height` 한 줄

| 요소 | 출처 | 역할 |
|---|---|---|
| 펼침 배경·스크롤바·hover 룩 | `@"asset/base/style/main#dropdown"` (공유 스타일) | 룩 전부 자동 |
| **스크롤 발동** | **`max_items_height: 280`** | 옵션 총높이가 이 px를 넘으면 펼침 목록 자동 스크롤 |
| 행 높이/폭 | `item_layout: { height: 36; width: 176; }` | 항목 N개 × 36px = 총높이 |
| 옵션 내용 | 코드로 주입 (`NativeDropdown::set_options`) | 동적 목록 OK |

`max_items_height`가 **없으면** 항목 수만큼 무한히 펼쳐진다(스크롤 안 남). 옵션이 적은
드롭다운(`champion_tier_dropdown` 등)엔 의도적으로 없음.

게임 내 실제 사용값:
- produce 모달(상품 선택) `pause.ui` athlete/product_dropdown = **280**
- `option.ui` (해상도/언어 등) = 280 / 320 / 400
- `records.ui`, `statistics.ui` = 400

---

## 2. 게임 원본 (pause.ui:2789 / :2814 — 상품 생산 모달)

```ui
#product_dropdown:dropdown {
  @"asset/base/style/main#dropdown";        // ← 펼침 배경/스크롤바/hover 룩 전부 상속
  x: 242px;
  width: 180px;
  height: 40px;                             // 닫혀있을 때(헤더) 높이
  max_items_height: 280;                    // ★ 펼침 목록 최대 높이 → 초과분 스크롤
  text: { size: 15; }                       // 헤더(선택값) 글자
  item_text: { size: 15; }                  // 펼침 항목 글자
  text_layout:      { x: 12px; y: 6px; width: 100%; height: 28px; }
  item_layout:      { height: 36px; width: 176px; x: 2px; }   // 행 1개 = 36px 높이
  item_text_layout: { x: 12px; y: 4px; width: 100%; height: 28px; }
}
```

---

## 3. 재사용 템플릿 (.ui)

우리 모드의 `mods/<MOD>/ui/layout/<화면>.ui` 에 아래 노드를 주입하고
`mod.override_info` 로 override. 노드는 .ui 에 **미리 선언**해야 렌더된다.

```ui
#my_dd:dropdown {
  @"asset/base/style/main#dropdown";
  width: 240px;
  height: 40px;
  max_items_height: 300;                    // 원하는 펼침 높이. (행높이×보이고픈개수)
  text: { size: 16; }
  item_text: { size: 16; }
  text_layout:      { x: 12px; y: 6px; width: 100%; height: 28px; }
  item_layout:      { height: 36px; width: 236px; x: 2px; }
  item_text_layout: { x: 12px; y: 4px; width: 100%; height: 28px; }
}
```

설계 메모:
- `max_items_height` ≈ (보여주고 싶은 항목 수) × `item_layout.height`. 예: 8개 보이려면 8×36 = 288.
- `item_layout.width` 는 보통 헤더 width − 4 (좌우 2px 마진, `x: 2px`).
- 폭은 고정 px 로. `width:100%` 등 Percent 는 런타임 레이아웃 override 와 충돌(메모리 `tfm2-ui-runtime-layout`).

---

## 4. 짝이 되는 코드 (ui_kit::NativeDropdown)

```rust
static DD: NativeDropdown = NativeDropdown::new("my_dd");

// 모달이 visible 된 뒤 1회만 (ABI 호출 — 로드시점 호출 금지):
DD.set_options(root, &["손목 보호대","안경","마우스","마우스 패드",
                       "키보드","응원봉","포스터","달력"], /*sel=*/0);

// 매 프레임 선택 폴링:
if let Some(idx) = DD.selected(root) { /* idx 사용 */ }
```

`max_items_height` 는 .ui 선언 속성이므로 코드에서 따로 호출할 필요 없음.
(ui_kit 의 `set_popup_max_height` = runner+0x1d8 직접 라이트는 .ui 를 못 고칠 때의 보조 폴백.
 .ui 를 통제할 수 있으면 `max_items_height` 가 정석·안전.)

---

## 5. 왜 커스텀(color_selectable) 대신 이걸?

`dropdown_example.rs` 의 커스텀 Dropdown 은 color_selectable 행을 직접 쌓는 방식 →
인터랙티브 노드라 hover/스크롤 시 레이아웃이 .ui 선언값으로 재계산되며 런타임 override 가
되돌려지는 함정(메모리 `tfm2-ui-runtime-layout`)이 있었다. 네이티브 `dropdown` 노드는
펼침/스크롤/클립/스크롤바를 엔진이 전부 처리하므로 그 버그 자체가 없다.
