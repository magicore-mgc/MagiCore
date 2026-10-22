//! Shared secret-censor behavior owner.
//! Owner hành vi che chuỗi nhạy cảm dùng chung.

mod secret_censor;

pub use self::secret_censor::censor_secret;

#[cfg(test)]
#[path = "test/secret_censor.rs"]
mod tests;
