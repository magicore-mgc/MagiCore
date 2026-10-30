//! Wires the Windows replacement implementation and its long-path tests.
//! Nối implementation thay file Windows và test path dài.

// A module and a re-export of the same name cannot coexist, and the long-path
// tests reach the implementation through `super::super`, so the module is
// public at that scope and callers name the function through it.
// (Module và re-export cùng tên không thể cùng tồn tại; test path dài chạm
// implementation qua `super::super`, nên module public ở scope đó và caller gọi
// hàm qua module.)
pub(super) mod windows_replace;
