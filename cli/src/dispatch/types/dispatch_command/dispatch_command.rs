//! DispatchCommand implementation owner — preserves the common/core route discriminator.
//! Owner giữ nguyên discriminator giữa tuyến dùng chung và tuyến theo core.

use crate::dispatch::types::{CommonCommand, CoreCommand};

#[allow(clippy::large_enum_variant)]
pub enum DispatchCommand {
    Common(CommonCommand),
    Core(CoreCommand),
}
