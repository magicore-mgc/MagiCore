//! Pick the native Web starter framework for a default-ref create request.
//! Chọn framework starter Web native cho yêu cầu create dùng ref mặc định.

pub(crate) fn native_web_starter_framework<'a>(
    requested_ref: &crate::scaffold::spec::ScaffoldRef,
    sub_type: &str,
    framework: Option<&'a str>,
) -> Option<&'a str> {
    if matches!(requested_ref, crate::scaffold::spec::ScaffoldRef::Default)
        && framework.is_some_and(|framework| {
            crate::scaffold::processors::web::WebProcessor::supports_native_project(
                sub_type, framework,
            )
        })
    {
        framework
    } else {
        None
    }
}
