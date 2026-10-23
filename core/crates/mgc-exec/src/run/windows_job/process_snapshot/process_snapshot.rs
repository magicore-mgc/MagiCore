//! Query active Job Object members with bounded allocation and retry limits.
//! Truy vấn thành viên Job Object với allocation và số lần retry có giới hạn.

#[cfg(windows)]
use super::super::super::error::WindowsJobError;

#[cfg(windows)]
const WINDOWS_JOB_INITIAL_PROCESS_CAPACITY: usize = 128;
#[cfg(windows)]
const WINDOWS_JOB_MAX_PROCESS_CAPACITY: usize = 8192;

// SAFETY: every Win32 job-object query below passes the borrowed `job` handle
// and caller-owned buffers sized before each call; return codes are checked and
// buffers are re-read only after a success.
// AN TOÀN: mọi truy vấn job object bên dưới truyền handle `job` mượn và buffer
// do caller sở hữu đúng kích thước; mã trả về được kiểm trước khi đọc buffer.
#[cfg(windows)]
#[allow(unsafe_code)]
pub(in crate::run::implementation::windows_job::implementation) fn active_process_ids(
    job: windows_sys::Win32::Foundation::HANDLE,
) -> std::result::Result<Vec<u32>, WindowsJobError> {
    use std::time::Duration;
    use windows_sys::Win32::Foundation::ERROR_MORE_DATA;
    use windows_sys::Win32::System::JobObjects::{
        JobObjectBasicProcessIdList, QueryInformationJobObject,
    };

    #[repr(C)]
    struct ProcessIdListBuffer {
        assigned: u32,
        listed: u32,
        process_ids: [usize; 1],
    }

    let mut capacity = WINDOWS_JOB_INITIAL_PROCESS_CAPACITY;
    let mut last_counts = (0usize, 0usize);
    for _ in 0..super::super::super::super::WINDOWS_JOB_MEMBER_SNAPSHOT_RETRIES {
        let word_count = 2usize.saturating_add(capacity);
        let mut buffer = vec![0usize; word_count];
        let list = buffer.as_mut_ptr().cast::<ProcessIdListBuffer>();
        let byte_len = std::mem::size_of::<u32>()
            .saturating_mul(2)
            .saturating_add(capacity.saturating_mul(std::mem::size_of::<usize>()));
        let byte_len = u32::try_from(byte_len).map_err(WindowsJobError::ProcessIdBufferTooLarge)?;
        // SAFETY: `buffer` is aligned for usize, reserves the fixed two-u32 header plus
        // `capacity` process IDs, and stays alive for the synchronous Windows API call.
        // AN TOÀN: `buffer` căn chỉnh theo usize, đủ header hai u32 và `capacity` PID,
        // tồn tại suốt lời gọi Windows đồng bộ.
        let queried = unsafe {
            QueryInformationJobObject(
                job,
                JobObjectBasicProcessIdList,
                list.cast(),
                byte_len,
                std::ptr::null_mut(),
            )
        };
        if queried == 0 {
            let error = super::super::windows_last_error();
            if error == ERROR_MORE_DATA {
                if capacity >= WINDOWS_JOB_MAX_PROCESS_CAPACITY {
                    return Err(WindowsJobError::ProcessIdCapacityLimit {
                        maximum: WINDOWS_JOB_MAX_PROCESS_CAPACITY,
                    });
                }
                capacity = capacity
                    .saturating_mul(2)
                    .min(WINDOWS_JOB_MAX_PROCESS_CAPACITY);
                std::thread::sleep(Duration::from_millis(
                    super::super::super::super::WINDOWS_COMMAND_LINE_RETRY_DELAY_MS,
                ));
                continue;
            }
            return Err(WindowsJobError::native_api(
                "QueryInformationJobObject",
                error,
            ));
        }

        // SAFETY: the successful query wrote the documented header into this aligned buffer.
        // AN TOÀN: truy vấn thành công đã ghi header theo tài liệu vào buffer được căn chỉnh.
        let (assigned, listed) = unsafe { ((*list).assigned as usize, (*list).listed as usize) };
        last_counts = (assigned, listed);
        if listed > capacity {
            return Err(WindowsJobError::InvalidProcessIdCount { listed, capacity });
        }
        if assigned > listed {
            if assigned > WINDOWS_JOB_MAX_PROCESS_CAPACITY {
                return Err(WindowsJobError::ProcessIdCapacityExceeded {
                    assigned,
                    maximum: WINDOWS_JOB_MAX_PROCESS_CAPACITY,
                });
            }
            // Compiler fan-out can change the snapshot while the API enumerates it.
            // Compiler tạo process nhanh có thể đổi snapshot ngay trong lúc liệt kê.
            capacity = assigned
                .max(capacity.saturating_mul(2))
                .min(WINDOWS_JOB_MAX_PROCESS_CAPACITY);
            std::thread::sleep(Duration::from_millis(
                super::super::super::super::WINDOWS_COMMAND_LINE_RETRY_DELAY_MS,
            ));
            continue;
        }

        // SAFETY: `process_ids` begins immediately after the two-u32 header and `listed`
        // was checked against the allocated capacity above.
        // AN TOÀN: `process_ids` nằm ngay sau header hai u32; `listed` đã được kiểm tra.
        let ids = unsafe {
            std::slice::from_raw_parts(
                std::ptr::addr_of!((*list).process_ids).cast::<usize>(),
                listed,
            )
        };
        return ids
            .iter()
            .map(|pid| u32::try_from(*pid).map_err(WindowsJobError::ProcessIdConversion))
            .collect();
    }
    Err(WindowsJobError::IncompleteProcessSnapshot {
        assigned: last_counts.0,
        listed: last_counts.1,
        capacity,
    })
}

#[cfg(all(test, windows))]
#[path = "test/mod.rs"]
mod test;
