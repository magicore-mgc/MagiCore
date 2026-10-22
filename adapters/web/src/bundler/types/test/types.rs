//! Public configuration/result surface test.
//! Test surface cấu hình/kết quả public.

use super::super::{BundleResult, BundlerConfig};

#[test]
fn public_configuration_and_result_fields_remain_constructible() {
    let config = BundlerConfig {
        entry: "src/main.tsx".into(),
        output_dir: "dist".into(),
        minify: true,
        sourcemap: true,
        target: "es2020".to_string(),
        public_path: "/".to_string(),
    };
    let result = BundleResult { size: 17 };

    assert_eq!(config.target, "es2020");
    assert_eq!(result.size, 17);
}
