# Optimizer adapter contract

## Purpose

Owns the `OptimizerAdapter` trait only: a stable name, a runtime match predicate, and config-file generation. Implementations live in sibling modules; nothing here knows about compat gating or registry ordering.

## Phạm vi

Chỉ sở hữu trait `OptimizerAdapter`: tên ổn định, predicate khớp runtime và sinh file config. Implementation nằm ở các module anh em; module này không biết về compat gating hay thứ tự registry.
