//! Linux scan test facade.
//! Facade test scan Linux.

// include! keeps the tests in this module, so their `super::` paths stay one
// level below the linux implementation.
// (include! giữ test trong module này, nên đường `super::` vẫn ở một cấp dưới
// implementation của linux.)
include!("linux.rs");
