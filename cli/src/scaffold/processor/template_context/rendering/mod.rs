// Shared renderer owner wiring.
// Nối owner render context và token đường dẫn.

mod rendering;

pub(in crate::scaffold::processor) use rendering::{
    ContextValues, render_contract, render_core_target_path, render_target_path,
};
