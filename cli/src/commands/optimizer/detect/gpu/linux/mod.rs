//! Linux PCI GPU source and nearest fixture tests.
//! (Owner nguồn GPU PCI Linux và các kiểm thử fixture đặt gần owner.)
mod linux;
pub(super) use linux::detect_gpus_linux;

#[cfg(test)]
#[path = "test/linux.rs"]
mod tests;
