# Adversarial CAS integrity tests / Test integrity CAS đối kháng

This folder owns the behavior modules for the `test_adversarial_integrity` Cargo integration target. The root `.rs` file remains the harness target and preserves all exact test names/cfg gates. `support/` owns only shared test setup; `test/` owns assertions grouped by integrity behavior. No production API or behavior is changed here.

Thư mục này sở hữu module hành vi của Cargo integration target `test_adversarial_integrity`. File `.rs` gốc vẫn là target của harness và giữ nguyên tên test/cfg. `support/` chỉ giữ setup dùng chung; `test/` giữ assertion chia theo hành vi integrity. Không đổi API hoặc hành vi production.
