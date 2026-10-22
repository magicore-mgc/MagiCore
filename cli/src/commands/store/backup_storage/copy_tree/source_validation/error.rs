//! Typed failures while inspecting a source tree and resolving its links.
//! Lỗi có type khi kiểm tra source tree và resolve link.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::store::backup_storage::copy_tree) enum SourceValidationError {
    #[error("source tree root does not exist: {0}")]
    MissingRoot(PathBuf),
    #[error("inspect tree root {path}: {source}")]
    InspectRoot {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("refuse symlink tree root: {0}")]
    SymlinkRoot(PathBuf),
    #[error("tree root is not a directory: {0}")]
    NotDirectory(PathBuf),
    #[error("canonicalize tree root {path}: {source}")]
    CanonicalizeRoot {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("inspect source directory {path}: {source}")]
    InspectDirectory {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("canonicalize source directory {path}: {source}")]
    CanonicalizeDirectory {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("refuse symlink source directory: {0}")]
    SymlinkDirectory(PathBuf),
    #[error("source entry is not a real directory: {0}")]
    NotRealDirectory(PathBuf),
    #[error("source directory {path} escapes expected root {root}")]
    DirectoryEscapesRoot { path: PathBuf, root: PathBuf },
    #[error("read source directory {path}: {source}")]
    ReadDirectory {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("read source directory entry in {path}: {source}")]
    ReadEntry {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("inspect source entry {path}: {source}")]
    InspectEntry {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("unsupported special source entry: {0}")]
    SpecialEntry(PathBuf),
    #[error("open source file without following links {path}: {source}")]
    OpenSource {
        path: PathBuf,
        #[source]
        source: mgc_platform::OpenRegularFileError,
    },
    #[error("read source symlink {path}: {source}")]
    ReadLink {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("resolve source symlink {path}: {source}")]
    ResolveLink {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("symlink target escapes source tree: link={link}, target={target}, root={root}")]
    LinkEscapesRoot {
        link: PathBuf,
        target: PathBuf,
        root: PathBuf,
    },
    #[error("symlink target has unsupported file type: link={link}, target={target}")]
    UnsupportedLinkTarget { link: PathBuf, target: PathBuf },
    #[error("directory symlink would create a recursive tree cycle: link={link}, target={target}")]
    CyclicDirectoryLink { link: PathBuf, target: PathBuf },
}
