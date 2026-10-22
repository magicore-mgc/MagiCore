//! Wire scanner status slug projection and local tests.
//! Nối ánh xạ slug trạng thái scanner và test cục bộ.

#[path = "status.rs"]
mod status;

pub(super) use self::status::status_slug;

#[cfg(test)]
#[path = "test/status.rs"]
mod tests;
