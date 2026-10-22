# Shared crypto error boundary / Boundary lỗi crypto dùng chung

Holds the crate's public error compatibility surface. Feature owners keep their local typed errors and map them to `CryptoError`; this module does not retry, discard sources or introduce catch-all behavior.

Chứa bề mặt lỗi public tương thích của crate. Owner chức năng giữ typed error local và map về `CryptoError`; module này không retry, bỏ source hay thêm catch-all.
