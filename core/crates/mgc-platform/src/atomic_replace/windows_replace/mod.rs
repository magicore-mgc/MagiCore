//! Wires the Windows replacement implementation and its long-path tests.
//! Nối implementation thay file Windows và test path dài.

// The implementation file shares its name with this directory, so the inner
// module is named for its role instead: two modules cannot both be called
// `windows_replace` in the same tree.
// (File implementation trùng tên thư mục này, nên module bên trong được đặt tên
// theo vai trò: hai module không thể cùng tên `windows_replace` trong cùng cây.)
#[path = "windows_replace.rs"]
mod implementation;

// The function is scoped to `atomic_replace`, which is two levels above this
// facade, so the re-export must grant the same scope.
// (Hàm scoped tới `atomic_replace`, cao hơn facade này hai cấp, nên re-export
// phải cấp cùng scope.)
#[cfg(windows)]
pub(in crate::atomic_replace) use implementation::windows_replace;
