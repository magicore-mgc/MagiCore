//! Coordinate the verified release-to-install sequence.
//! Điều phối chuỗi từ release đã xác minh tới cài binary.

mod self_update;
pub use self_update::run;
