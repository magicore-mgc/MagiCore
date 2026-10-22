# Dependency resolver tests

These tests sit beside the resolver owner and cover trait registration, the Python foreign-lock refusal, Java/native lane routing, pending-lock replacement, and the lock-poison diagnostic. Network-backed Rust, Go, Maven, NuGet, and Python behavior remains covered by the existing adapter integration/native-engine suites.

## Phạm vi

Các test nằm sát resolver owner, kiểm tra trait registration, từ chối lock Python thuộc manager khác, route lane Java/native, thay pending-lock và diagnostic khi mutex bị poison. Hành vi Rust, Go, Maven, NuGet và Python cần mạng vẫn do integration/native-engine suite hiện có sở hữu.
