//! Capture command stdout to bounded disk-backed storage.
//! Thu output command vào file tạm có giới hạn và khôi phục stdout an toàn.

#[cfg(unix)]
use anyhow::Context;
use anyhow::Result;
#[cfg(unix)]
use std::fs::File;
#[cfg(unix)]
use std::io::{self, Read, Seek, SeekFrom, Write};

/// Bound command output returned through MCP while keeping disk-backed capture.
/// Giới hạn output command trả qua MCP, dùng file tạm để không phình bộ nhớ.
#[cfg(unix)]
const MAX_MCP_CAPTURE_BYTES: u64 = 1_048_576;

#[cfg(unix)]
pub(in crate::commands::mcp) struct McpStdoutCapture {
    output: File,
    previous_stdout: File,
    restored: bool,
}

#[cfg(not(unix))]
pub(in crate::commands::mcp) struct McpStdoutCapture;

impl McpStdoutCapture {
    // SAFETY: dup/dup2 operate on fds owned by this process; saved fds are
    // closed in Drop and every call's return value is checked before use.
    // AN TOÀN: dup/dup2 thao tác trên fd của process này; fd đã lưu được đóng
    // trong Drop và mọi giá trị trả về được kiểm trước khi dùng.
    #[allow(unsafe_code)]
    pub(in crate::commands::mcp::stdio) fn begin() -> Result<Self> {
        #[cfg(unix)]
        {
            use std::os::fd::{AsRawFd, FromRawFd};

            let output = tempfile::tempfile().context("cannot create MCP output capture")?;
            // Save session stderr fd 1, then redirect command output to a private temp file.
            // Lưu fd 1 của stderr phiên, rồi chuyển output command vào file tạm riêng.
            // SAFETY: dup returns a new fd owned by this process; the result is
            // checked and later wrapped in `File`.
            // AN TOÀN: dup trả fd mới thuộc process; kết quả được kiểm và sau
            // đó bọc trong `File`.
            let previous_fd = unsafe { libc::dup(libc::STDOUT_FILENO) };
            if previous_fd < 0 {
                return Err(io::Error::last_os_error()).context("cannot preserve MCP stderr route");
            }
            // SAFETY: `output` owns a live fd and STDOUT_FILENO is this process's
            // stdout; the return value is checked.
            // AN TOÀN: `output` sở hữu fd còn sống và STDOUT_FILENO là stdout
            // của process; giá trị trả về được kiểm.
            if unsafe { libc::dup2(output.as_raw_fd(), libc::STDOUT_FILENO) } < 0 {
                let error = io::Error::last_os_error();
                // SAFETY: `previous_fd` came from `dup` above and has no other
                // owner on this error path.
                // AN TOÀN: `previous_fd` đến từ `dup` ở trên và không có owner
                // khác trên nhánh lỗi này.
                unsafe { libc::close(previous_fd) };
                return Err(error).context("cannot capture command stdout");
            }
            Ok(Self {
                output,
                // SAFETY: ownership of `previous_fd` transfers to `File` once.
                // AN TOÀN: quyền sở hữu `previous_fd` chuyển cho `File` một lần.
                previous_stdout: unsafe { File::from_raw_fd(previous_fd) },
                restored: false,
            })
        }

        #[cfg(not(unix))]
        {
            Ok(Self)
        }
    }

    #[cfg(unix)]
    pub(in crate::commands::mcp::stdio) fn finish(mut self) -> Result<String> {
        let flush_result = io::stdout().flush();
        self.restore_stdout();
        flush_result?;
        self.output.seek(SeekFrom::Start(0))?;
        let mut bytes = Vec::new();
        Read::by_ref(&mut self.output)
            .take(MAX_MCP_CAPTURE_BYTES + 1)
            .read_to_end(&mut bytes)?;
        let truncated = bytes.len() as u64 > MAX_MCP_CAPTURE_BYTES;
        if truncated {
            bytes.truncate(MAX_MCP_CAPTURE_BYTES as usize);
        }
        let mut text = String::from_utf8_lossy(&bytes).into_owned();
        if truncated {
            text.push_str("\n[command output truncated]");
        }
        Ok(text)
    }

    #[cfg(not(unix))]
    pub(in crate::commands::mcp::stdio) fn finish(self) -> Result<String> {
        Ok(String::new())
    }

    #[cfg(unix)]
    #[allow(unsafe_code)]
    fn restore_stdout(&mut self) {
        use std::os::fd::AsRawFd;

        if !self.restored {
            // Restore the session stderr route before reading or replying.
            // Khôi phục đường stderr phiên trước khi đọc capture hoặc trả response.
            // SAFETY: `previous_stdout` owns a live fd; STDOUT_FILENO is
            // restored exactly once guarded by `restored`.
            // AN TOÀN: `previous_stdout` sở hữu fd còn sống; STDOUT_FILENO
            // được khôi phục đúng một lần nhờ cờ `restored`.
            unsafe { libc::dup2(self.previous_stdout.as_raw_fd(), libc::STDOUT_FILENO) };
            self.restored = true;
        }
    }
}

#[cfg(unix)]
impl Drop for McpStdoutCapture {
    fn drop(&mut self) {
        self.restore_stdout();
    }
}
