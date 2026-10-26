//! Process-tree guard that assigns children before resuming their suspended thread.
//! Guard cây process: gắn child vào Job trước khi resume thread đang suspend.

#[cfg(windows)]
use super::super::super::error::WindowsJobError;
#[cfg(windows)]
use super::super::owned_handle::WindowsOwnedHandle;

/// Closing this job handle kills its assigned process tree by OS policy.
/// Đóng handle job sẽ dừng cây tiến trình đã gắn theo chính sách của OS.
/// Source: https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects
/// Nguồn: tài liệu Microsoft xác nhận KILL_ON_JOB_CLOSE kết thúc toàn bộ job.
#[cfg(windows)]
pub(in crate::run::implementation) struct ProcessTreeGuard {
    job: WindowsOwnedHandle,
}

#[cfg(windows)]
impl ProcessTreeGuard {
    // SAFETY: the job object and its extended-limit struct are created and
    // initialised in this scope; the struct is passed by pointer while alive
    // and the returned handle is wrapped in an owned guard.
    // AN TOÀN: job object và struct giới hạn được tạo/khởi tạo trong scope
    // này; struct truyền bằng con trỏ khi còn sống, handle trả về được bọc
    // trong guard sở hữu.
    #[allow(unsafe_code)]
    pub(in crate::run::implementation) fn new() -> std::result::Result<Self, WindowsJobError> {
        use windows_sys::Win32::System::JobObjects::{
            CreateJobObjectW, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
            JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectExtendedLimitInformation,
            SetInformationJobObject,
        };

        // SAFETY: both arguments are documented as optional; passing null asks
        // the kernel for an unnamed job with default security. The returned
        // handle is checked for null before use and owned by `WindowsOwnedHandle`,
        // so it is closed exactly once.
        // AN TOÀN: cả hai tham số đều là tuỳ chọn theo tài liệu; truyền null để
        // kernel tạo job không tên với security mặc định. Handle trả về được kiểm
        // null trước khi dùng và thuộc `WindowsOwnedHandle`, nên đóng đúng một lần.
        let handle = unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
        if handle.is_null() {
            return Err(WindowsJobError::native_api(
                "CreateJobObjectW",
                super::super::windows_last_error(),
            ));
        }
        let guard = Self {
            job: WindowsOwnedHandle::from_raw(handle),
        };
        let mut limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
        limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        // SAFETY: `guard.job` is a live job handle owned by `guard`; `limits`
        // outlives the call and its declared size matches the information-class
        // constant passed alongside it.
        // AN TOÀN: `guard.job` là handle job còn sống do `guard` sở hữu; `limits`
        // sống lâu hơn lời gọi và kích thước khai báo khớp info-class truyền kèm.
        let configured = unsafe {
            SetInformationJobObject(
                guard.job.as_raw(),
                JobObjectExtendedLimitInformation,
                std::ptr::from_ref(&limits).cast(),
                std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            )
        };
        if configured == 0 {
            return Err(WindowsJobError::native_api(
                "SetInformationJobObject",
                super::super::windows_last_error(),
            ));
        }
        Ok(guard)
    }

    // SAFETY: `child` is borrowed for the call and its raw handle is valid; the
    // job handle belongs to `self`, so no handle is closed here.
    // AN TOÀN: `child` được mượn trong suốt lời gọi và raw handle còn hiệu lực;
    // job handle thuộc `self` nên không handle nào bị đóng ở đây.
    #[allow(unsafe_code)]
    pub(in crate::run::implementation) fn activate(
        &self,
        child: &std::process::Child,
    ) -> std::result::Result<(), WindowsJobError> {
        use std::os::windows::io::AsRawHandle;
        use windows_sys::Win32::System::JobObjects::AssignProcessToJobObject;

        let assigned =
            unsafe { AssignProcessToJobObject(self.job.as_raw(), child.as_raw_handle().cast()) };
        if assigned == 0 {
            return Err(WindowsJobError::native_api(
                "AssignProcessToJobObject",
                super::super::windows_last_error(),
            ));
        }
        super::super::primary_thread::resume_primary_thread(child.id())
    }

    // SAFETY: `self.job` is an owned job handle kept alive by the guard; the
    // exit code is a plain integer and the result is checked.
    // AN TOÀN: `self.job` là job handle do guard sở hữu và còn sống; exit code
    // là số nguyên và kết quả được kiểm.
    #[allow(unsafe_code)]
    pub(in crate::run::implementation) fn terminate(
        &self,
    ) -> std::result::Result<(), WindowsJobError> {
        use windows_sys::Win32::System::JobObjects::TerminateJobObject;

        let terminated = unsafe { TerminateJobObject(self.job.as_raw(), 1) };
        if terminated == 0 {
            return Err(WindowsJobError::native_api(
                "TerminateJobObject",
                super::super::windows_last_error(),
            ));
        }
        Ok(())
    }

    pub(in crate::run::implementation::windows_job) fn active_process_ids(
        &self,
    ) -> std::result::Result<Vec<u32>, WindowsJobError> {
        super::super::process_snapshot::active_process_ids(self.job.as_raw())
    }

    pub(in crate::run::implementation::windows_job) fn capture_job_member_process(
        &self,
        process_id: u32,
    ) -> std::result::Result<Option<WindowsOwnedHandle>, WindowsJobError> {
        super::super::member_capture::capture_job_member_process(self.job.as_raw(), process_id)
    }
}

#[cfg(all(test, windows))]
#[path = "test/mod.rs"]
mod test;
