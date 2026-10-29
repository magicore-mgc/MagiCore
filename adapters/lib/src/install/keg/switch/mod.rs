//! Active-version switch owner wiring.
//! Wiring owner switch version active.

mod switch;

pub use switch::{SwitchOutcome, active_state, rollback_active, switch_active};
