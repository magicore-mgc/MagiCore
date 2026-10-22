// Shared ecosystem lookup wiring and local test registration.
// Nối helper nhận diện core dùng chung và test cục bộ.
mod ecosystem;
#[cfg(test)]
mod test;

pub(super) use ecosystem::require_ecosystem;
