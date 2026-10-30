//! Wires the Windows replacement implementation and its long-path tests.
//! Nối implementation thay file Windows và test path dài.

// The implementation lives in a file of the same name, so this facade has to
// declare it as a module: a module and a function cannot share a name in one
// namespace, which is why the function is reached through the module.
// (Implementation nằm trong file cùng tên nên facade phải khai nó là module:
// module và hàm không thể cùng tên trong một namespace, vì vậy hàm được gọi
// qua module.)
#[path = "windows_replace.rs"]
pub(super) mod windows_replace;

#[cfg(windows)]
pub(super) use windows_replace::windows_replace;
