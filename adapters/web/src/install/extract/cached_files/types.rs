//! File identity and content facts captured for a verified cache snapshot.
//! Fact về định danh và nội dung file trong snapshot cache đã xác minh.
#[derive(Clone, Copy)]
pub(crate) struct CachedFileMetadata {
    pub(super) len: u64,
    pub(super) executable: bool,
    pub(super) identity: CachedFileIdentity,
}

#[cfg(unix)]
pub(crate) type CachedFileIdentity = (u64, u64);
#[cfg(windows)]
pub(crate) type CachedFileIdentity = (u32, u64);
#[cfg(not(any(unix, windows)))]
pub(crate) type CachedFileIdentity = ();
