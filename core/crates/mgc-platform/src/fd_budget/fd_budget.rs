//! Bound how many files the process holds open at once.
//! Chặn số file process mở cùng lúc.
//!
//! Concurrency limits bound *tasks*, not *file descriptors*. A resolver that
//! allows 24 downloads can still hold 72 descriptors (socket, part file, and the
//! archive being extracted) for a moment, and a graph with many transitive
//! packages multiplies that by however many operations overlap. The kernel limit
//! is per process and, once reached, every open fails with EMFILE — including
//! the ones the process needs to report the failure.
//! Giới hạn concurrency chặn *task*, không chặn *file descriptor*. Resolver cho
//! phép 24 tải vẫn có thể giữ 72 descriptor (socket, file part, archive đang giải
//! nén) trong chốc lát, và graph nhiều package bắc cầu nhân lên theo số thao tác
//! chồng nhau. Giới hạn kernel là theo process và khi chạm trần, mọi lần mở đều
//! fail EMFILE — kể cả lần cần để báo lỗi.
//!
//! This is a budget, not a fix for a leak: the process states how many
//! descriptors it intends to hold, and refuses to start work it cannot fund.
//! Đây là ngân sách, không phải bản vá rò rỉ: process tuyên bố số descriptor nó
//! định giữ và từ chối bắt đầu việc nó không đủ ngân sách.

use std::sync::Arc;

use tokio::sync::Semaphore;

/// Default share of the process descriptor limit that MagiCore will use.
///
/// A quarter leaves room for the runtime, the store database, terminal handles
/// and the child processes' own descriptors, which are counted against the same
/// process table on most systems.
/// Một phần tư để dành cho runtime, database store, handle terminal và
/// descriptor của process con — trên đa số hệ thống chúng tính vào cùng bảng
/// process.
const DESCRIPTOR_SHARE_DENOMINATOR: usize = 4;

/// Floor so a very low limit does not reduce the budget to nothing.
/// Sàn để giới hạn quá thấp không làm ngân sách về 0.
const MIN_BUDGET: usize = 32;

/// Ceiling: past this, more parallelism stops helping and starts hurting.
/// Trần: quá mức này, thêm song song không còn giúp mà bắt đầu hại.
const MAX_BUDGET: usize = 512;

/// Descriptor budget the process will spend on concurrent opens.
/// Ngân sách descriptor process dùng cho mở file đồng thời.
pub fn descriptor_budget() -> usize {
    if let Some(configured) = std::env::var("MAGICORE_MAX_OPEN_FILES")
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
    {
        return configured;
    }

    let limit = process_descriptor_limit().unwrap_or(1024);
    (limit / DESCRIPTOR_SHARE_DENOMINATOR).clamp(MIN_BUDGET, MAX_BUDGET)
}

/// The kernel's per-process descriptor limit, when the platform exposes it.
/// Giới hạn descriptor mỗi process của kernel, khi nền tảng cho biết.
#[cfg(unix)]
#[allow(unsafe_code)]
fn process_descriptor_limit() -> Option<usize> {
    // SAFETY: `getrlimit` writes into a caller-provided struct and takes no
    // other pointer; the struct is initialised and outlives the call.
    // AN TOÀN: `getrlimit` ghi vào struct do caller cấp và không nhận con trỏ
    // khác; struct đã khởi tạo và sống lâu hơn lời gọi.
    unsafe {
        let mut limit = libc::rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };
        if libc::getrlimit(libc::RLIMIT_NOFILE, &mut limit) == 0 {
            Some(limit.rlim_cur as usize)
        } else {
            None
        }
    }
}

#[cfg(not(unix))]
fn process_descriptor_limit() -> Option<usize> {
    // Windows has no equivalent per-process descriptor rlimit; the handle limit
    // is large and bounded by available memory, so the default stands.
    // Windows không có rlimit descriptor tương đương; giới hạn handle lớn và bị
    // chặn bởi bộ nhớ khả dụng, nên dùng mặc định.
    None
}

/// Returns the shared descriptor semaphore for this process.
/// Trả semaphore descriptor dùng chung của process.
pub fn global_descriptor_semaphore() -> Arc<Semaphore> {
    static SEMAPHORE: std::sync::OnceLock<Arc<Semaphore>> = std::sync::OnceLock::new();
    SEMAPHORE
        .get_or_init(|| Arc::new(Semaphore::new(descriptor_budget())))
        .clone()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
