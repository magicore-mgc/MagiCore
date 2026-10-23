# OCI upload range validation

## Purpose

Owns parsing and validating the `Content-Range` header for chunked blob uploads: digits-only bounds, non-invertible ranges, and a declared length that must equal the body length. Every failure maps to 416.

## Vietnamese

Sở hữu việc parse và validate header `Content-Range` cho upload blob theo chunk: cận chỉ gồm chữ số, range không đảo ngược, và độ dài khai báo phải bằng độ dài body. Mọi lỗi trả 416.
