//! RAII owner for a Win32 handle plus read-only process identity queries.
//! Owner RAII cho Win32 handle và truy vấn danh tính process chỉ đọc.

#[cfg(windows)]
use super::super::super::error::WindowsJobError;
#[cfg(windows)]
use std::path::PathBuf;

/// Maximum UTF-16 path buffer accepted by the Windows process-image query API.
/// Dung lượng UTF-16 tối đa của API truy vấn đường dẫn image trên Windows.
#[cfg(windows)]
const WINDOWS_IMAGE_PATH_BUFFER_UNITS: usize = 32_768;

/// Owns one valid Win32 handle and closes it exactly once on drop.
/// Sở hữu một Win32 handle hợp lệ và đóng đúng một lần khi drop.
#[cfg(windows)]
pub(in crate::run::implementation::windows_job) struct WindowsOwnedHandle(
    windows_sys::Win32::Foundation::HANDLE,
);

#[cfg(windows)]
impl WindowsOwnedHandle {
    /// Take ownership of a non-null handle returned by a successful Win32 call.
    /// Nhận quyền sở hữu handle khác null do lời gọi Win32 thành công trả về.
    pub(in crate::run::implementation::windows_job::implementation) fn from_raw(
        handle: windows_sys::Win32::Foundation::HANDLE,
    ) -> Self {
        Self(handle)
    }

    /// Borrow the raw value for a synchronous Win32 call while this owner stays alive.
    /// Mượn giá trị thô trong lời gọi Win32 đồng bộ khi owner vẫn còn sống.
    pub(in crate::run::implementation::windows_job::implementation) fn as_raw(
        &self,
    ) -> windows_sys::Win32::Foundation::HANDLE {
        self.0
    }

    #[allow(unsafe_code)]
    pub(in crate::run::implementation::windows_job) fn full_image_path(
        &self,
    ) -> std::result::Result<PathBuf, WindowsJobError> {
        use std::os::windows::ffi::OsStringExt;
        use windows_sys::Win32::System::Threading::QueryFullProcessImageNameW;

        let mut buffer = vec![0u16; WINDOWS_IMAGE_PATH_BUFFER_UNITS];
        let mut length = buffer.len() as u32;
        // SAFETY: the handle is open for limited process queries and the buffer is writable
        // for the full capacity reported in `length`.
        // AN TOÀN: handle có quyền truy vấn giới hạn; buffer có thể ghi đủ dung lượng trong `length`.
        let queried = unsafe {
            QueryFullProcessImageNameW(self.as_raw(), 0, buffer.as_mut_ptr(), &mut length)
        };
        if queried == 0 {
            return Err(WindowsJobError::native_api(
                "QueryFullProcessImageNameW",
                super::super::windows_last_error(),
            ));
        }
        let original_length = length;
        let length = usize::try_from(length).map_err(WindowsJobError::ImagePathLengthConversion)?;
        if length == 0 || length > buffer.len() {
            return Err(WindowsJobError::InvalidImagePathLength {
                length: original_length,
                capacity: buffer.len(),
            });
        }
        buffer.truncate(length);
        Ok(PathBuf::from(std::ffi::OsString::from_wide(&buffer)))
    }

    #[allow(unsafe_code)]
    pub(in crate::run::implementation::windows_job) fn has_terminated(
        &self,
    ) -> std::result::Result<bool, WindowsJobError> {
        use windows_sys::Win32::Foundation::STILL_ACTIVE;
        use windows_sys::Win32::System::Threading::GetExitCodeProcess;

        let mut exit_code = 0;
        // SAFETY: this handle is open with PROCESS_QUERY_LIMITED_INFORMATION access.
        // AN TOÀN: handle đang mở với quyền PROCESS_QUERY_LIMITED_INFORMATION.
        let queried = unsafe { GetExitCodeProcess(self.as_raw(), &mut exit_code) };
        if queried == 0 {
            return Err(WindowsJobError::native_api(
                "GetExitCodeProcess",
                super::super::windows_last_error(),
            ));
        }
        Ok(exit_code != STILL_ACTIVE as u32)
    }
}

#[cfg(windows)]
impl Drop for WindowsOwnedHandle {
    #[allow(unsafe_code)]
    fn drop(&mut self) {
        let closed = unsafe { windows_sys::Win32::Foundation::CloseHandle(self.as_raw()) };
        if closed == 0 {
            // Read the thread-local Win32 code before any other native call can overwrite it.
            // Đọc mã Win32 ngay tại chỗ để native call khác không ghi đè nguyên nhân.
            let code = super::super::windows_last_error();
            super::super::deferred_handle_close::record_handle_close_failure(code);
        }
    }
}

#[cfg(all(test, windows))]
#[path = "test/mod.rs"]
mod test;
