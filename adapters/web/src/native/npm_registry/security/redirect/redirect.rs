//! security redirect.
// Chứa đúng trách nhiệm của module; giữ nguyên hành vi, API, chính sách lỗi và bảo mật hiện có.

// Keep bounded redirects without crossing protocol credential boundaries.
// Giữ redirect mặc định có giới hạn, nhưng không chuyển token qua scheme khác.
pub(in crate::native::npm_registry) fn registry_redirect_policy() -> reqwest::redirect::Policy {
    reqwest::redirect::Policy::custom(|attempt| {
        if let Some(previous) = attempt.previous().last()
            && previous.scheme() != attempt.url().scheme()
        {
            return attempt.error(std::io::Error::other(
                "registry redirect changes URL scheme; configure the final secure URL explicitly",
            ));
        }
        reqwest::redirect::Policy::default().redirect(attempt)
    })
}
