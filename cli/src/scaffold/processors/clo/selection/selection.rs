//! Natively supported cloud framework selection.
//! Chọn framework cloud được sinh native.

/// Frameworks generated directly (the `_` arm is the Pulumi template).
/// Unknown ids keep the layer-required error.
/// (Framework tự sinh trực tiếp — id lạ vẫn fail rõ.)
pub(crate) fn supports(framework: &str) -> bool {
    matches!(
        framework,
        "terraform"
            | "terraform-gcp"
            | "cdk"
            | "cdk-typescript"
            | "cloudflare"
            | "lambda"
            | "pulumi"
    )
}
