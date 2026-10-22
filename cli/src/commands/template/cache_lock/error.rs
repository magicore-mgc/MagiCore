use anyhow::anyhow;

pub(super) fn lock_directory_is_not_real() -> anyhow::Error {
    anyhow!("template cache lock directory is not a real directory")
}

pub(super) fn lock_file_is_not_regular() -> anyhow::Error {
    anyhow!("template cache lock is not a regular file")
}
