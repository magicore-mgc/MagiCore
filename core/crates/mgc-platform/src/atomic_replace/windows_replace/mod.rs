//! Wires the Windows replacement implementation and its long-path tests.
//! Nối implementation thay file Windows và test path dài.

// The long-path tests reach the implementation through `super::super`, one
// level above this module, so the inner module may not be private.
// (Test path dài chạm implementation qua `super::super`, cao hơn module này một
// cấp, nên module bên trong không được private.)
pub(super) mod windows_replace;
pub(super) use self::windows_replace::windows_replace;
