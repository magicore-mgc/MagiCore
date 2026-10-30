//! Linux cgroup source module and nearest tests.
//! (Owner nguồn cgroup Linux và các kiểm thử đặt gần owner.)
mod linux;
#[cfg(target_os = "linux")]
pub(in crate::commands::optimizer::detect::memory) use linux::linux_cgroup_memory_limit_bytes;

#[cfg(test)]
#[path = "test/linux.rs"]
mod tests;
