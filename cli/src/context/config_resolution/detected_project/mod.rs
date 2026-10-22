//! Own signature-based core selection for projects without `mgc.toml`.
//! Sở hữu việc chọn core theo chữ ký khi project chưa có `mgc.toml`.

mod detected_project;

pub(super) use detected_project::resolve;
