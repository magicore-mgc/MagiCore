//! Wire multi-lock pin aggregation and its hermetic test.
//! Nối gom pin từ nhiều lock và test hermetic.

mod pin_collection;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub use self::pin_collection::collect_dotnet_pins;
