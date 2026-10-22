//! Shared secure reads and atomic project-manifest writes.
//! Đầu mối export các thao tác đọc an toàn và ghi atomic cho manifest project.

mod atomic_file;
mod error;
mod project_file;
mod project_path;
mod regular_file;

#[cfg(unix)]
mod project_read;
#[cfg(unix)]
mod project_write;

pub use self::error::{ProjectFileError, ProjectFileErrorKind, ProjectFileRecovery};
pub use self::project_file::{
    atomic_write_project_regular, atomic_write_regular, read_project_regular_text,
    read_regular_text,
};
