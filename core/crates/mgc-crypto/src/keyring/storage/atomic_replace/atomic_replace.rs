//! Atomic keyring file replacement — Thay file keyring atomic.
//! Dispatch atomic replacement by OS — Chọn atomic replace theo OS.

#[cfg(not(windows))]
use std::fs;
use std::path::Path;

pub(in crate::keyring::storage) fn atomic_replace_file(
    source: &Path,
    destination: &Path,
) -> std::io::Result<()> {
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        use windows_sys::Win32::Storage::FileSystem::{
            MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
        };

        let source_wide: Vec<u16> = source.as_os_str().encode_wide().chain(Some(0)).collect();
        let destination_wide: Vec<u16> = destination
            .as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect();
        // SAFETY: both NUL-terminated buffers live through the call; MoveFileExW retains neither.
        // AN TOÀN: hai buffer NUL còn sống hết lời gọi; MoveFileExW không giữ lại pointer nào.
        #[allow(unsafe_code)]
        let succeeded = unsafe {
            MoveFileExW(
                source_wide.as_ptr(),
                destination_wide.as_ptr(),
                MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
            )
        };
        if succeeded == 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }
    #[cfg(not(windows))]
    {
        fs::rename(source, destination)
    }
}
