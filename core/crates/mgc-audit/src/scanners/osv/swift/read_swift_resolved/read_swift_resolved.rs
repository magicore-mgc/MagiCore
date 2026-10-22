//! v1/v2 Package.resolved parsing and pin/skipped classification.
//! Parse Package.resolved v1/v2 và phân loại pin cần bỏ qua.

use super::super::{SwiftResolved, push_swift_pin};
use crate::scanners::osv::OsvPin;
use mgc_types::{MgError, MgResult};

/// Read Package.resolved into OSV pins (`swift` ecosystem). Versionless
/// pins (branch/revision checkouts) are recorded as skipped — they
/// cannot be version-matched against advisories and must never be
/// silently counted clean.
/// Đọc Package.resolved thành ghim OSV (ecosystem `swift`). Ghim không
/// version (checkout branch/revision) được ghi là skipped — không đối
/// chiếu version với advisory được và không được phép đếm sạch âm thầm.
pub fn read_swift_resolved(raw: &str) -> MgResult<(Vec<OsvPin>, Vec<String>)> {
    let resolved: SwiftResolved = serde_json::from_str(raw)
        .map_err(|e| MgError::Other(format!("invalid Package.resolved: {e}")))?;

    let mut pins = Vec::new();
    let mut skipped = Vec::new();
    // v2 pins live at the root; v1 nests under object.pins.
    // Ghim v2 ở root; v1 nằm trong object.pins.
    for pin in resolved.pins {
        push_swift_pin(
            &pin.identity,
            pin.location.as_deref(),
            pin.state.as_ref(),
            &mut pins,
            &mut skipped,
        );
    }
    if let Some(object) = resolved.object {
        for pin in object.pins {
            push_swift_pin(
                &pin.package,
                pin.repository_url.as_deref(),
                pin.state.as_ref(),
                &mut pins,
                &mut skipped,
            );
        }
    }
    Ok((pins, skipped))
}
