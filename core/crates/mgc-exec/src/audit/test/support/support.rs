pub(crate) fn temp_dir() -> tempfile::TempDir {
    tempfile::tempdir().expect("create audit test directory")
}
