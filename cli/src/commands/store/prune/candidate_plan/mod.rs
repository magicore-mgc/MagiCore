//! Preflights all unreferenced package paths and logical payload sizes.
//! Kiểm tra trước toàn bộ path package chưa tham chiếu và dung lượng payload logic.

mod candidate_plan;
mod error;
pub(super) use candidate_plan::plan_candidates;
pub(super) use error::CandidatePlanError;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
