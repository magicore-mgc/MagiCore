//! Nearest tests for scoped package mapping and payload preflight.
//! Test gần owner cho package scope và bước đo payload trước khi prune.

#[path = "candidate_plan.rs"]
mod candidate_plan;
#[cfg(any(unix, windows))]
#[path = "symlink_support.rs"]
mod symlink_support;
