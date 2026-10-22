//! Finding value and constructors for AI model audit results.
//! Giá trị finding và constructor cho kết quả audit model AI.

use super::super::Severity;

/// A single model-audit issue with optional file context.
/// Một vấn đề audit model, có thể kèm ngữ cảnh file.
#[derive(Debug, Clone)]
pub struct Finding {
    pub severity: Severity,
    pub category: String,
    pub message: String,
    pub file_path: Option<String>,
}

impl Finding {
    pub fn critical(category: &str, message: &str) -> Self {
        Finding {
            severity: Severity::Critical,
            category: category.to_string(),
            message: message.to_string(),
            file_path: None,
        }
    }

    pub fn high(category: &str, message: &str) -> Self {
        Finding {
            severity: Severity::High,
            category: category.to_string(),
            message: message.to_string(),
            file_path: None,
        }
    }

    pub fn medium(category: &str, message: &str) -> Self {
        Finding {
            severity: Severity::Medium,
            category: category.to_string(),
            message: message.to_string(),
            file_path: None,
        }
    }

    pub fn with_file(mut self, path: &str) -> Self {
        self.file_path = Some(path.to_string());
        self
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
