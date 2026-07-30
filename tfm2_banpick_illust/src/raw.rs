//! ImageRunner 의 이미지 소스를 런타임에 바꾸는 저수준 접근.
//!
//! ★왜 SDK 타입드 API 를 못 쓰는가 (2026-07-18 인게임 실측으로 확정)
//!   `node.runner.as_any().downcast_ref::<ImageRunner>()` 는 **항상 None** 이다.
//!   ImageRunner 노드 130개 전수 시도 → 성공 0 (Box 경유·명시적 역참조 둘 다 0).
//!   원인 = 게임 exe 에 링크된 `engine_ui` 와 SDK 가 배포하는 rlib 의 crate 해시가 달라
//!   **TypeId 가 불일치**한다. 반면 `type_name()` 은 exe 쪽 코드가 돌려주는 문자열이라
//!   정상이고, 메모리 배치도 동일하다. ⟹ **종류 판별 = type_name 문자열, 필드 접근 =
//!   데이터 포인터 + 구조체 오프셋**. (기 정본 = `ANA\discovered-ui-runner-offsets.md`,
//!   출하된 모드 전부가 이 패턴 — downcast 를 쓴 건 이 모드가 유일했고 그래서 실패했다.)
//!
//! ★ImageRunner 오프셋 (sdk_051 rlib `offset_of!` 실측)
//!   size = 848 / `style.normal` = 0 / **`source` = 0**(String 24B)
//!   상태 stride = **208** → normal 0 / hover 208 / active 416 / disabled 624
//!   `flip_x` = 200(+상태) / `image_scale` = 120 / `color` = 128 / `rect` = 164
//!
//! ⚠ String 은 필드 단위(ptr/cap/len)로 손으로 쓰지 말 것 — 이 빌드의 물리 배치가
//!   `{cap@0, ptr@8, len@16}` 로 비표준이다. `*(p as *mut String) = s.to_string()` 으로
//!   **통째 대입**하면 컴파일러가 맞춰주고 기존 String 도 정상 drop 된다(누수 없음).

use mod_api::Node;
use std::any::Any;

/// 상태 슬롯 4개(normal/hover/active/disabled).
const STATE_OFF: [usize; 4] = [0, 208, 416, 624];
const OFF_SOURCE: usize = 0;
const OFF_FLIP_X: usize = 200;

#[link(name = "kernel32")]
extern "system" {
    fn IsBadReadPtr(p: usize, n: usize) -> i32;
}

fn readable(p: usize, n: usize) -> bool {
    p >= 0x10000 && (p as u64) < 0x0000_8000_0000_0000 && unsafe { IsBadReadPtr(p, n) } == 0
}

/// ImageRunner 인스턴스의 데이터 포인터. 이미지 러너가 아니거나 수상하면 None.
fn img_base(n: &Node) -> Option<*mut u8> {
    if !n.runner.type_name().contains("ImageRunner") {
        return None;
    }
    let any: &dyn Any = n.runner.as_any();
    // 트레잇 객체(fat ptr) = [데이터 ptr, vtable ptr]
    let parts: [usize; 2] =
        unsafe { std::mem::transmute::<*const dyn Any, [usize; 2]>(any as *const dyn Any) };
    let base = parts[0];
    if !readable(base, 848) {
        return None;
    }
    Some(base as *mut u8)
}

/// normal 슬롯의 현재 source 를 읽는다.
pub fn get_source(n: &Node) -> Option<String> {
    let base = img_base(n)?;
    unsafe {
        let p = base.add(OFF_SOURCE) as *const String;
        if !readable(p as usize, 24) {
            return None;
        }
        Some((*p).clone())
    }
}

/// source 를 4상태 전부에 설정한다(hover 등으로 전이해도 유지되게).
pub fn set_source(n: &mut Node, src: &str) -> bool {
    let base = match img_base(n) {
        Some(b) => b,
        None => return false,
    };
    unsafe {
        // 레이아웃 검증 — normal 슬롯이 정상 String 으로 읽히지 않으면 건드리지 않는다.
        if get_source(n).is_none() {
            return false;
        }
        for st in STATE_OFF {
            *(base.add(st + OFF_SOURCE) as *mut String) = src.to_string();
        }
    }
    true
}

/// 좌우반전(레드 진영용)을 4상태 전부에 설정.
pub fn set_flip_x(n: &mut Node, v: bool) {
    if let Some(base) = img_base(n) {
        unsafe {
            for st in STATE_OFF {
                *(base.add(st + OFF_FLIP_X) as *mut u8) = v as u8;
            }
        }
    }
}

// ── LabelRunner 텍스트 읽기 (patchviz 표시명 수확용) ───────────────────────
//
// ★LabelRunner (size 496) — `discovered-ui-runner-offsets.md` 정본: text @ +352 (String).
//   ImageRunner 와 같은 이유(TypeId 불일치)로 다운캐스트 불가 → type_name + 오프셋.

const LABEL_SIZE: usize = 496;
const OFF_LABEL_TEXT: usize = 352;

/// style.normal.color (Color{r,g,b,a} 16B) — 오프셋 표 +40.
const OFF_LABEL_COLOR: usize = 40;

fn label_base(n: &Node) -> Option<usize> {
    if !n.runner.type_name().contains("LabelRunner") {
        return None;
    }
    let any: &dyn Any = n.runner.as_any();
    let parts: [usize; 2] =
        unsafe { std::mem::transmute::<*const dyn Any, [usize; 2]>(any as *const dyn Any) };
    if !readable(parts[0], LABEL_SIZE) {
        return None;
    }
    Some(parts[0])
}

/// 라벨 글자색(normal) 읽기.
pub fn label_color_get(n: &Node) -> Option<[f32; 3]> {
    let base = label_base(n)?;
    unsafe {
        let p = (base + OFF_LABEL_COLOR) as *const f32;
        Some([*p, *p.add(1), *p.add(2)])
    }
}

/// 라벨 글자색(normal) 쓰기 — 알파는 보존.
pub fn label_color_set(n: &mut Node, rgb: [f32; 3]) {
    if let Some(base) = label_base(n) {
        unsafe {
            let p = (base + OFF_LABEL_COLOR) as *mut f32;
            *p = rgb[0];
            *p.add(1) = rgb[1];
            *p.add(2) = rgb[2];
        }
    }
}

/// LabelRunner 의 현재 text. 라벨이 아니거나 수상하면 None.
pub fn label_text(n: &Node) -> Option<String> {
    if !n.runner.type_name().contains("LabelRunner") {
        return None;
    }
    let any: &dyn Any = n.runner.as_any();
    let parts: [usize; 2] =
        unsafe { std::mem::transmute::<*const dyn Any, [usize; 2]>(any as *const dyn Any) };
    let base = parts[0];
    if !readable(base, LABEL_SIZE) {
        return None;
    }
    unsafe {
        let p = (base + OFF_LABEL_TEXT) as *const String;
        if !readable(p as usize, 24) {
            return None;
        }
        Some((*p).clone())
    }
}

// ── 노드 레이아웃 강제 (게임이 매 프레임 되돌리는 좌표를 다시 씀) ──────────
//
// ★게임 코드가 밴픽 픽 슬롯의 `name`/`proficiency_*` 좌표를 런타임에 재설정한다.
//   그래서 `.ui` 오버라이드로 옮겨도 화면엔 원래 자리에 그려진다(자식 노드는
//   안 건드려서 그것만 반영되는 게 증상). ⟹ post_update 에서 매 프레임 재기입.
//   (선례 = item_tactics 컴팩트 스코어보드 blue_player 슬롯 간격, ui_kit 정본)
//
// ★Node 레이아웃 오프셋 (sdk_051 `offset_of!` 실측, ui_kit 기록과 일치)
//   width@112 / height@120 / x@128 / y@136, **상태 stride 128**
//   (normal 48 / hover 176 / active 304 / disabled 432)
//   각 값은 `Length` = { tag u32 @+0, f32 @+4 } ⟹ **f32 만 +4 에 쓴다**(tag 보존).
//   tag 를 안 건드리므로 `.ui` 가 px 선언인 노드에만 써야 한다(% 선언이면 깨짐).

const LAYOUT_STRIDE: usize = 128;
const LAYOUT_STATES: usize = 4;
const LF_WIDTH: usize = 112;
const LF_HEIGHT: usize = 120;
const LF_X: usize = 128;
const LF_Y: usize = 136;
/// Length 안에서 f32 값의 위치.
const LEN_VAL: usize = 4;

unsafe fn wr_len(base: *mut u8, field: usize, v: f32) {
    for i in 0..LAYOUT_STATES {
        *(base.add(field + i * LAYOUT_STRIDE + LEN_VAL) as *mut f32) = v;
    }
}

/// 노드 layout(normal 상태)의 (x, y, w, h) 읽기 — 게임이 런타임에 쓰는 실좌표.
/// (툴팁처럼 게임이 매 프레임 위치를 재기입하는 노드의 화면 좌표 취득용)
pub fn node_layout_rect(n: &Node) -> Option<(f32, f32, f32, f32)> {
    let base = n as *const Node as *const u8;
    if !readable(base as usize, std::mem::size_of::<Node>()) {
        return None;
    }
    unsafe {
        let rd = |field: usize| *(base.add(field + LEN_VAL) as *const f32);
        Some((rd(LF_X), rd(LF_Y), rd(LF_WIDTH), rd(LF_HEIGHT)))
    }
}

/// 노드의 x/y(+선택적 w/h)를 **4상태 전부**에 강제로 쓴다.
/// 4상태를 다 써야 hover 등으로 전이할 때 `.ui` 선언값으로 튕겨 돌아가지 않는다.
pub fn force_layout(n: &mut Node, x: f32, y: f32, wh: Option<(f32, f32)>) {
    let base = n as *mut Node as *mut u8;
    if !readable(base as usize, std::mem::size_of::<Node>()) {
        return;
    }
    unsafe {
        wr_len(base, LF_X, x);
        wr_len(base, LF_Y, y);
        if let Some((w, h)) = wh {
            wr_len(base, LF_WIDTH, w);
            wr_len(base, LF_HEIGHT, h);
        }
    }
}
