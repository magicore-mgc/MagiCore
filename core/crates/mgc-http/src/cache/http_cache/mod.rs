// HttpCache module export — expose the existing shared cache API.
// Export module HttpCache — giữ API cache dùng chung hiện có.

mod http_cache;

pub use http_cache::HttpCache;
