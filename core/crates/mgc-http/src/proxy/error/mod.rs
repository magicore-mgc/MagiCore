// Proxy error module export — keep construction errors local to proxy.
// Export module lỗi proxy — giữ lỗi khởi tạo trong owner proxy.

mod error;
pub(super) use error::ProxyError;
