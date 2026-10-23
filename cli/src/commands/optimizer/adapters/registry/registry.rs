//! Registry of every optimizer adapter instance.
//! Danh bạ mọi instance adapter optimizer.

use super::super::contract::OptimizerAdapter;
use super::super::{
    bun, candle, deno, flutter, go_ai, go_lib, node, python_lib, pytorch, react_native,
};
use super::super::{rust_lib, typescript_lib};

/// Get all available adapters — lấy tất cả adapters có sẵn
pub fn all_adapters() -> Vec<Box<dyn OptimizerAdapter>> {
    vec![
        // Web adapters — adapters web
        Box::new(node::NodeJsAdapter),
        Box::new(deno::DenoAdapter),
        Box::new(bun::BunAdapter),
        // AI adapters — adapters AI
        Box::new(pytorch::PyTorchAdapter),
        Box::new(candle::CandleAdapter),
        Box::new(go_ai::GoAiAdapter),
        // Lib adapters — adapters thư viện
        Box::new(rust_lib::RustLibAdapter),
        Box::new(go_lib::GoLibAdapter),
        Box::new(python_lib::PythonLibAdapter),
        Box::new(typescript_lib::TypeScriptLibAdapter),
        // App adapters — adapters ứng dụng
        Box::new(flutter::FlutterAdapter),
        Box::new(react_native::ReactNativeAdapter),
    ]
}
