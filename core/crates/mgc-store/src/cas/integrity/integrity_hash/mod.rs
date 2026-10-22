//! Typed digest owner wiring. / Wiring owner digest typed.
//! Keeps the implementation and its nearest test registered here.
//! Đăng ký implementation cùng test gần owner tại đây.

#[path = "integrity_hash.rs"]
mod implementation;
pub use self::implementation::IntegrityHash;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
