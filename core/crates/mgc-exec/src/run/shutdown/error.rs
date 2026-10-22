//! Typed failures while installing process shutdown handlers.
//! Lỗi typed khi cài handler dừng process.

use std::fmt;

#[cfg(any(unix, windows))]
#[derive(Debug, Clone, Copy)]
pub(in crate::run::implementation) struct SystemCallFailure {
    operation: &'static str,
    code: Option<i64>,
}

#[cfg(any(unix, windows))]
impl SystemCallFailure {
    pub(super) fn new(operation: &'static str, code: Option<i64>) -> Self {
        Self { operation, code }
    }
}

#[cfg(any(unix, windows))]
impl fmt::Display for SystemCallFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code {
            Some(code) => write!(
                formatter,
                "{} failed with native error code {}",
                self.operation, code
            ),
            None => write!(
                formatter,
                "{} failed; native error code unavailable",
                self.operation
            ),
        }
    }
}

#[cfg(any(unix, windows))]
impl std::error::Error for SystemCallFailure {}

/// Signal-handler installation failures retained so the runner can fail closed.
/// Giữ lỗi cài signal handler để runner từ chối spawn an toàn.
#[derive(Debug, Clone, Copy)]
pub(in crate::run::implementation) enum ShutdownError {
    #[cfg(unix)]
    SignalHandlerRegistration {
        signal: &'static str,
        source: SystemCallFailure,
    },
    #[cfg(windows)]
    ConsoleHandlerRegistration { source: SystemCallFailure },
}

impl ShutdownError {
    #[cfg(unix)]
    pub(super) fn check_signal_handler_registration(
        signal: &'static str,
        registered: bool,
        code: Option<i32>,
    ) -> Result<(), Self> {
        if registered {
            Ok(())
        } else {
            Err(Self::SignalHandlerRegistration {
                signal,
                source: SystemCallFailure::new("libc::signal", code.map(i64::from)),
            })
        }
    }

    #[cfg(windows)]
    pub(super) fn check_console_handler_registration(
        registered: bool,
        code: Option<u32>,
    ) -> Result<(), Self> {
        if registered {
            Ok(())
        } else {
            Err(Self::ConsoleHandlerRegistration {
                source: SystemCallFailure::new("SetConsoleCtrlHandler", code.map(i64::from)),
            })
        }
    }
}

impl fmt::Display for ShutdownError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(unix)]
            Self::SignalHandlerRegistration { signal, source } => {
                write!(formatter, "failed to install {signal} handler: {source}")
            }
            #[cfg(windows)]
            Self::ConsoleHandlerRegistration { source } => {
                write!(
                    formatter,
                    "failed to install Windows console control handler: {source}"
                )
            }
        }
    }
}

impl std::error::Error for ShutdownError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            #[cfg(unix)]
            Self::SignalHandlerRegistration { source, .. } => Some(source),
            #[cfg(windows)]
            Self::ConsoleHandlerRegistration { source } => Some(source),
        }
    }
}
