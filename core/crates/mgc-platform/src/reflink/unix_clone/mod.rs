//! Unix FICLONE implementation owner and test registration.
//! Owner implementation FICLONE trên Unix và đăng ký test.

mod unix_clone;
pub(super) use self::unix_clone::reflink_clone;
