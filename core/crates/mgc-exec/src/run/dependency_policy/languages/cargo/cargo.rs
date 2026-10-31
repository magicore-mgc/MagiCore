//! Cargo dependency-management and locked/opt-in-offline command rules.
//! Rule command Cargo quản lý dependency và build locked/offline opt-in.

pub(in crate::run::implementation::dependency_policy::implementation::languages) fn denial_reason(
    _args: &[String],
    first: &str,
) -> Option<&'static str> {
    // Cargo has no "download-only-if-needed" switch: `--locked` pins the
    // lockfile, while `--offline` both forbids downloads AND requires every
    // crate to be present in the local registry cache. Requiring it here made
    // `mgc build` fail with Cargo's `no matching package named …` for any crate
    // the managed store did not hold yet, even with network available.
    // (Cargo không có cờ "chỉ tải khi cần": `--locked` ghim lockfile, còn
    // `--offline` vừa cấm tải vừa đòi crate đã có trong cache cục bộ. Bắt buộc
    // `--offline` khiến `mgc build` fail với crate chưa có dù có mạng.)
    match first {
        "add" | "remove" | "update" | "fetch" | "install" | "vendor" | "generate-lockfile" => {
            Some("Cargo dependency-management subcommands are not allowed through mgc-exec")
        }
        "package" | "publish" | "search" | "login" | "logout" | "owner" | "yank" | "tree"
        | "info" => Some("Cargo registry/package subcommands are not allowed through mgc-exec"),
        // `metadata` is a read-only query; it resolves the manifest/lock graph
        // and needs no offline switch (Cargo materializes nothing).
        // (`metadata` là truy vấn chỉ đọc; không cần cờ offline.)
        // A freshly scaffolded Rust project has no Cargo.lock yet; `--locked`
        // would make cargo refuse to create one, so the build could never run.
        // The CLI adds `--locked` itself whenever a lock exists, so the
        // lock-drift guarantee is preserved without blocking first builds.
        // (Project Rust vừa scaffold chưa có Cargo.lock; `--locked` sẽ khiến
        // cargo từ chối tạo lock nên build không chạy được. CLI tự thêm
        // `--locked` khi đã có lock, nên vẫn giữ bảo đảm chống drift.)
        "build" | "test" | "check" | "run" | "clippy" | "metadata" => None,
        // `new`/`init` create a project skeleton from the local template, they
        // never touch a registry or lockfile; the framework initializers need
        // them to scaffold a lib/rust project with no network.
        // (`new`/`init` tạo khung project từ template cục bộ, không chạm registry
        // hay lockfile; framework initializer cần chúng để scaffold lib/rust khi
        // không có mạng.)
        "new" | "init" => None,
        "fmt" | "clean" | "version" | "help" | "locate-project" | "read-manifest"
        | "verify-project" => None,
        "" => None,
        _ => Some("unclassified Cargo subcommands are blocked by mgc-exec policy"),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
