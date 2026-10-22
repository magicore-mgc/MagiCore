# Shared core mutation-journal tests

This nearest-owner test folder wires 28 existing async cases into six responsibility files: mutation admission/snapshots (5), journal lifecycle and filesystem links (6), crash recovery and isolation (5), project lineage/identity (5), legacy adoption/input handling (4), and cross-core rejection (3). The parent module context and helper imports remain centralized in `mod.rs`; production code continues to own runtime behavior and typed errors.

Folder test gần owner này nối 28 ca async hiện có vào sáu file theo trách nhiệm: admission/snapshot mutation (5), vòng đời journal và link hệ thống file (6), phục hồi crash/cô lập (5), lineage/identity project (5), tiếp nhận journal cũ/xử lý input (4), từ chối journal khác core (3). Context và helper import của module cha giữ tập trung tại `mod.rs`; code production vẫn sở hữu hành vi runtime và lỗi typed. Test được chuyển nhưng chưa chạy; source tĩnh không phải bằng chứng PASS runtime.
