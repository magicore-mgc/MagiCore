//! Cache clean operation owner wiring.
//! File này chỉ nối module clean và export API nội bộ.

mod clean;

pub(super) use clean::clean;
