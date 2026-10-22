//! Typed failures at the Windows Job inspection boundary.
//! Lỗi có kiểu tại ranh giới kiểm tra Windows Job.

use std::fmt;

#[derive(Debug)]
pub(super) enum InspectionError {
    MemberInspection(anyhow::Error),
    CommandLineValidation {
        process_id: u32,
        image_name: String,
        source: anyhow::Error,
    },
    #[cfg(windows)]
    RetryScan(anyhow::Error),
}

impl fmt::Display for InspectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MemberInspection(_) => {
                formatter.write_str("failed to inspect Windows Job members")
            }
            Self::CommandLineValidation {
                process_id,
                image_name,
                ..
            } => write!(
                formatter,
                "failed to validate command line for Windows Job child PID {process_id} ({image_name})"
            ),
            #[cfg(windows)]
            Self::RetryScan(_) => formatter.write_str("Windows Job process scan retry failed"),
        }
    }
}

impl std::error::Error for InspectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::MemberInspection(source) => Some(source.as_ref()),
            #[cfg(windows)]
            Self::RetryScan(source) => Some(source.as_ref()),
            Self::CommandLineValidation { source, .. } => Some(source.as_ref()),
        }
    }
}

#[cfg(test)]
#[path = "test/error.rs"]
mod test;
