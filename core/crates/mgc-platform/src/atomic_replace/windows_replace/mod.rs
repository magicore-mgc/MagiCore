//! Wires the Windows replacement implementation and its long-path tests.
//! Nối implementation thay file Windows và test path dài.

// The implementation file shares its name with this directory, so the inner
// module is named for its role instead: two modules cannot both be called
// `windows_replace` in the same tree.
// (File implementation trùng tên thư mục này, nên module bên trong được đặt tên
// theo vai trò: hai module không thể cùng tên `windows_replace` trong cùng cây.)
#[path = "windows_replace.rs"]
mod implementation;

#[cfg(windows)]
pub(super) use implementation::windows_replace;
