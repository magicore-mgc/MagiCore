# Shared archive extraction / Giải nén archive dùng chung

## Responsibility / Trách nhiệm

Owns the stable `protocols::archive` API, safe tar extraction, Cargo archive verification and atomic file publication. It delegates to focused same-name owners and maps local typed errors back to the existing `MgError` surface. The similar `mgc-fetcher` implementation remains a separate crate boundary in this batch.

## Tiếng Việt

Sở hữu API ổn định `protocols::archive`, giải nén tar an toàn, xác minh archive Cargo và publish file nguyên tử. Module điều phối các owner cùng tên và ánh xạ lỗi typed cục bộ về bề mặt `MgError` hiện có. Bản giải nén tương tự trong `mgc-fetcher` vẫn là ranh giới crate riêng trong batch này.
