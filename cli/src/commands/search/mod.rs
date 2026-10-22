//! Wire the shared package-search command owners.
//! Khai báo và nối các owner của lệnh tìm kiếm package dùng chung.

mod classification;
mod error;
mod output;
mod query;
mod registry_request;
mod results;
mod search;
mod types;

pub use search::run;
