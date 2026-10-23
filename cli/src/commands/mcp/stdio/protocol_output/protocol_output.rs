//! Preserve and restore the MCP protocol stdout handle.
//! Giữ và khôi phục handle stdout dành riêng cho giao thức MCP.

use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, Write};

/// Keep the JSON-RPC pipe separate while CLI command output goes to stderr.
/// Tách pipe JSON-RPC, chuyển output CLI sang stderr.
pub(in crate::commands::mcp) struct McpProtocolOutput {
    protocol: File,
    #[cfg(windows)]
    original_stdout: WinHandle,
}

#[cfg(windows)]
type WinHandle = *mut std::ffi::c_void;

#[cfg(windows)]
const WIN_STD_OUTPUT_HANDLE: u32 = (-11_i32) as u32;
#[cfg(windows)]
const WIN_STD_ERROR_HANDLE: u32 = (-12_i32) as u32;

#[cfg(windows)]
#[link(name = "kernel32")]
#[allow(unsafe_code)]
unsafe extern "system" {
    fn GetCurrentProcess() -> WinHandle;
    fn GetStdHandle(std_handle: u32) -> WinHandle;
    fn SetStdHandle(std_handle: u32, handle: WinHandle) -> i32;
    fn DuplicateHandle(
        source_process: WinHandle,
        source_handle: WinHandle,
        target_process: WinHandle,
        target_handle: *mut WinHandle,
        desired_access: u32,
        inherit_handle: i32,
        options: u32,
    ) -> i32;
    fn CloseHandle(handle: WinHandle) -> i32;
}

impl McpProtocolOutput {
    // SAFETY: only duplicates/redirects the process stdio handles; the saved handle is owned by `File`.
    // AN TOÀN: chỉ nhân đôi/chuyển hướng handle stdio; handle đã lưu được `File` sở hữu.
    #[allow(unsafe_code)]
    pub(in crate::commands::mcp::stdio) fn redirect_cli_output() -> Result<Self> {
        #[cfg(unix)]
        {
            use std::os::fd::FromRawFd;

            // Duplicate protocol stdout, then send ordinary fd 1 output to stderr.
            // Nhân đôi stdout giao thức, rồi chuyển output fd 1 thông thường sang stderr.
            // SAFETY: dup returns a new fd owned by this process; the result is
            // checked before use and later wrapped in `File`.
            // AN TOÀN: dup trả fd mới thuộc process này; kết quả được kiểm
            // trước khi dùng và sau đó bọc trong `File`.
            let protocol_fd = unsafe { libc::dup(libc::STDOUT_FILENO) };
            if protocol_fd < 0 {
                return Err(io::Error::last_os_error()).context("cannot preserve MCP stdout");
            }
            // SAFETY: both fds are the process's own stdio descriptors and the
            // return value is checked before continuing.
            // AN TOÀN: cả hai fd là stdio của process và giá trị trả về được kiểm.
            if unsafe { libc::dup2(libc::STDERR_FILENO, libc::STDOUT_FILENO) } < 0 {
                let error = io::Error::last_os_error();
                // SAFETY: `protocol_fd` came from `dup` above and is not owned
                // elsewhere on this error path.
                // AN TOÀN: `protocol_fd` đến từ `dup` ở trên và chưa owner khác
                // trên nhánh lỗi này.
                unsafe { libc::close(protocol_fd) };
                return Err(error).context("cannot redirect CLI output to stderr");
            }
            // SAFETY: ownership of `protocol_fd` transfers to `File` exactly once.
            // AN TOÀN: quyền sở hữu `protocol_fd` chuyển cho `File` đúng một lần.
            let protocol = unsafe { File::from_raw_fd(protocol_fd) };
            Ok(Self { protocol })
        }

        #[cfg(windows)]
        {
            use std::os::windows::io::FromRawHandle;

            let original_stdout = unsafe { GetStdHandle(WIN_STD_OUTPUT_HANDLE) };
            let stderr = unsafe { GetStdHandle(WIN_STD_ERROR_HANDLE) };
            if original_stdout.is_null() || stderr.is_null() {
                return Err(io::Error::last_os_error())
                    .context("MCP stdio handles are unavailable");
            }
            let current_process = unsafe { GetCurrentProcess() };
            let mut protocol_handle = std::ptr::null_mut();
            if unsafe {
                DuplicateHandle(
                    current_process,
                    original_stdout,
                    current_process,
                    &mut protocol_handle,
                    0,
                    0,
                    0x0000_0002,
                )
            } == 0
            {
                return Err(io::Error::last_os_error()).context("cannot preserve MCP stdout");
            }
            if unsafe { SetStdHandle(WIN_STD_OUTPUT_HANDLE, stderr) } == 0 {
                let error = io::Error::last_os_error();
                unsafe { CloseHandle(protocol_handle) };
                return Err(error).context("cannot redirect CLI output to stderr");
            }
            let protocol = unsafe { File::from_raw_handle(protocol_handle) };
            Ok(Self {
                protocol,
                original_stdout,
            })
        }

        #[cfg(not(any(unix, windows)))]
        {
            anyhow::bail!("MCP stdio isolation is unsupported on this platform")
        }
    }
}

impl Write for McpProtocolOutput {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.protocol.write(buffer)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.protocol.flush()
    }
}

impl Drop for McpProtocolOutput {
    // SAFETY: restores stdout from the live protocol handle before its `File` field is dropped.
    // AN TOÀN: khôi phục stdout từ handle protocol còn sống trước khi field `File` bị drop.
    #[allow(unsafe_code)]
    fn drop(&mut self) {
        #[cfg(unix)]
        {
            use std::os::fd::AsRawFd;
            // Restore process stdout after the MCP session ends.
            // Khôi phục stdout tiến trình khi phiên MCP kết thúc.
            // SAFETY: `protocol` owns a live fd and STDOUT_FILENO is the process
            // stdout being restored on Drop.
            // AN TOÀN: `protocol` sở hữu fd còn sống, STDOUT_FILENO là stdout
            // process được khôi phục trong Drop.
            unsafe { libc::dup2(self.protocol.as_raw_fd(), libc::STDOUT_FILENO) };
        }
        #[cfg(windows)]
        // SAFETY: restores the handle saved by `redirect_cli_output`; it stays
        // valid for the process lifetime.
        // AN TOÀN: khôi phục handle đã lưu bởi `redirect_cli_output`; handle
        // còn hiệu lực suốt vòng đời process.
        unsafe {
            SetStdHandle(WIN_STD_OUTPUT_HANDLE, self.original_stdout);
        }
    }
}
