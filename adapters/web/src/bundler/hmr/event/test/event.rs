//! Public HMR event shape test.
//! Test hình dạng event HMR public.

use super::HmrEvent;

#[test]
fn update_event_keeps_its_path_when_cloned() {
    let event = HmrEvent::Update("src/main.ts".to_string());
    let cloned = event.clone();

    assert!(matches!(cloned, HmrEvent::Update(path) if path == "src/main.ts"));
}
