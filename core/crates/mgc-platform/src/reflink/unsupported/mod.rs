//! Unsupported-platform reflink fallback owner and test registration.
//! Owner fallback reflink cho nền tảng không hỗ trợ và đăng ký test.

mod unsupported;
pub(super) use self::unsupported::reflink_clone;
