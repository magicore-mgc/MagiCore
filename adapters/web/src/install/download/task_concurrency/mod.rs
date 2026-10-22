//! Bounded Web download task concurrency owner wiring.
//! Wiring owner concurrency có giới hạn cho task download Web.

mod task_concurrency;

pub use task_concurrency::pipeline_task_concurrency_limit;
