//! Offline and hybrid HTTP clients.
//! Client ngoại tuyến và client kết hợp; giữ các đường dẫn API công khai hiện có.

mod error;
mod hybrid_client;
mod offline_client;

pub use hybrid_client::HybridClient;
pub use offline_client::OfflineClient;
