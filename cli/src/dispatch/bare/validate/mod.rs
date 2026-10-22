// Shared bare-request preflight wiring and local test registration.
// Nối preflight bare dùng chung và test cục bộ.
#[cfg(test)]
mod test;
mod validate;

pub(super) use validate::validate_bare_request;
