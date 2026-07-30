use mod_api::*;
fn probe(e: &UIEvent) {
    // 존재하는 변형만 컴파일됨. 없는 건 에러로 떨어져 정체 노출.
    let _ = matches!(e, UIEvent::Click{..});
    let _ = matches!(e, UIEvent::Hover{..});
    let _ = matches!(e, UIEvent::MouseDown{..});
    let _ = matches!(e, UIEvent::MouseUp{..});
    let _ = matches!(e, UIEvent::MouseMove{..});
    let _ = matches!(e, UIEvent::Press{..});
    let _ = matches!(e, UIEvent::Release{..});
    let _ = matches!(e, UIEvent::Down{..});
    let _ = matches!(e, UIEvent::Up{..});
    let _ = matches!(e, UIEvent::Scroll{..});
    let _ = matches!(e, UIEvent::Drag{..});
    let _ = matches!(e, UIEvent::Enter{..});
    let _ = matches!(e, UIEvent::Leave{..});
    let _ = matches!(e, UIEvent::Focus{..});
    let _ = matches!(e, UIEvent::Key{..});
    let _ = matches!(e, UIEvent::KeyDown{..});
    let _ = matches!(e, UIEvent::Input{..});
    let _ = matches!(e, UIEvent::RightClick{..});
    let _ = matches!(e, UIEvent::DoubleClick{..});
}
fn main(){}
