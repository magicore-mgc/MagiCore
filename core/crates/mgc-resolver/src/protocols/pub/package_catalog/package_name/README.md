# Package identity / Định danh package

`package_name.rs` enforces the lowercase pub.dev path-segment grammar before a name reaches the registry URL. Invalid input keeps the established `MgError::InvalidPackageName` category. Unit tests are registered beside the validator.
