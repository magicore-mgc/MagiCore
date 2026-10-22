pub mod git_source;
#[cfg(test)]
pub(super) use git_source::git_repo_name;
pub(super) use git_source::identity_to_dep_name;
pub use git_source::{git_transport_allowed, git_url_to_name, is_git_name};
#[cfg(test)]
mod test;
