//! HMR version and broadcast subscription tests.
//! Test version HMR và broadcast subscription.

use super::HmrManager;
use crate::bundler::hmr::HmrEvent;

#[test]
fn starts_with_version_zero() {
    assert_eq!(HmrManager::new().version(), 0);
}

#[test]
fn subscription_receives_the_broadcast_event() {
    let manager = HmrManager::new();
    let mut receiver = manager.subscribe();
    manager.tx.send(HmrEvent::Reload).unwrap();

    assert!(matches!(receiver.try_recv(), Ok(HmrEvent::Reload)));
}
