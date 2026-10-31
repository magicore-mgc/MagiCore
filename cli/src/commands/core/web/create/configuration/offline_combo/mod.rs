//! Offline split-fullstack combos: a framework half plus an owned backend half.
//! Combo fullstack tách rời offline: một nửa framework cộng nửa backend do MGC sở hữu.
//!
//! A split combo (`react-express`, `vue-nestjs`, …) is not a framework — it is
//! two halves in one project, and the combo leaf layer
//! (`web/fullstack/split/<combo>`) is published separately from the framework
//! it embeds. When that leaf is unavailable, this owner scaffolds the frontend
//! half through the framework's own initializer, then writes the backend half
//! into `server/` — the directory `detect_project_mode` already reads as
//! `FullstackSplit`. It never writes frontend framework source.
//! Combo tách rời (`react-express`, `vue-nestjs`, …) không phải một framework
//! mà là hai nửa trong một project, và leaf combo (`web/fullstack/split/<combo>`)
//! được publish riêng khỏi framework nó chứa. Khi thiếu leaf đó, owner này
//! scaffold nửa frontend bằng initializer chính thức của framework, rồi ghi nửa
//! backend vào `server/` — đúng thư mục `detect_project_mode` đã đọc là
//! `FullstackSplit`. Không bao giờ ghi source frontend của framework.
//!
//! Only stacks whose backend publishes a starter MagiCore can invoke through an
//! official initializer are composable offline (Node.js statements that ship a
//! CLI). Backends MagiCore can describe completely from data — FastAPI, Flask,
//! Django, Axum and Gin — are written by `native_backend` without running any
//! framework tool. Other halves fail closed with a message naming both halves.
//! Chỉ stack có backend phát hành starter mà MagiCore gọi được qua initializer
//! chính thức mới ghép offline được (backend Node có CLI). Backend mà MagiCore mô
//! tả trọn vẹn từ dữ liệu — FastAPI, Flask, Django, Axum và Gin — do
//! `native_backend` ghi mà không chạy công cụ framework nào. Nửa khác fail-closed
//! kèm thông điệp nêu rõ cả hai nửa.

#[path = "compose.rs"]
mod compose;
#[path = "native_backend.rs"]
#[cfg_attr(test, allow(unused_imports))]
mod native_backend;
#[path = "node_backend.rs"]
mod node_backend;

pub(in crate::commands::core::web::implementation) use compose::{compose_offline, is_combo};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
