# graph_from_v4_lockfile

## Responsibility

Converts a v4 lock document to an install graph without collapsing package identity; validates package references, artifact URLs and installer-enforced SRI.

## Trách nhiệm

Chuyển lock v4 thành install graph mà không gộp identity; chặn package reference, URL artifact và SRI không hợp lệ.

## Contract

This folder owns only this named operation and its module boundary. Existing crate API and test locations stay stable.

Folder này chỉ sở hữu operation có tên và ranh giới module tương ứng. API crate và vị trí test hiện có được giữ nguyên.
