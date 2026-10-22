# Fetcher archive extraction / Giải nén archive của fetcher

## Responsibility / Trách nhiệm

Owns the stable `mgc_fetcher::extract::*` API, bounded tar reading, destination path checks, CAS staging/publication and generation claims. Plain extraction and CAS extraction remain separate routes with their existing durability semantics.

## Tiếng Việt

Sở hữu API ổn định `mgc_fetcher::extract::*`, đọc tar có giới hạn, kiểm tra path đích, staging/publish CAS và generation claim. Giải nén thường và giải nén qua CAS vẫn là hai luồng riêng với chính sách durability hiện có.
