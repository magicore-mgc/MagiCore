//! Metadata-fetch test registration and shared fixture imports.
//! Đăng ký test metadata-fetch và import fixture dùng chung.

#![allow(clippy::unwrap_used, clippy::await_holding_lock)]
#![allow(unsafe_code)]

use crate::cache::{SharedWebCache, current_unix_secs};
use crate::cache_metadata::{
    load_metadata_by_name_with_fallback, metadata_record_retry_deferred, metadata_ttl_secs,
};
use crate::native;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use self::support::{bind_test_listener, reg_key, restore_env_var, tempdir_real};

#[path = "deferred_retry_respects_stale_limit.rs"]
mod deferred_retry_respects_stale_limit;
#[path = "etag_persistence.rs"]
mod etag_persistence;
#[path = "stale_metadata_max_age.rs"]
mod stale_metadata_max_age;
#[path = "stale_retry_cooldown.rs"]
mod stale_retry_cooldown;
#[path = "support.rs"]
mod support;
