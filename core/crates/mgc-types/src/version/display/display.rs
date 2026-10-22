//! Formats a version value for its stable textual representation.
//! Định dạng Version thành biểu diễn text ổn định.

use super::super::Version;
use std::fmt;

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(pre) = &self.pre {
            write!(f, "-{pre}")?;
        }
        if let Some(dev) = self.dev {
            write!(f, ".dev{dev}")?;
        }
        if let Some(post) = self.post {
            write!(f, ".post{post}")?;
        }
        if let Some(build) = &self.build {
            write!(f, "+{build}")?;
        }
        Ok(())
    }
}
