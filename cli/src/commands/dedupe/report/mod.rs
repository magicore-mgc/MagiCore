//! Dedupe report owner wiring — exports the stable JSON/text renderer.
//! Wiring owner report dedupe — export renderer JSON/text ổn định.
mod error;
mod report;
pub(super) use report::print_report;
