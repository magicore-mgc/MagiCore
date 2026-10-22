# Extraction locks / Lock extraction

Owns the process-local lock registries keyed by canonical package root and package id. Keep key semantics and lock lifetime unchanged in this move-only batch.

Quản lý registry lock trong process theo canonical package root và package id. Giữ nguyên semantics khóa và vòng đời lock trong batch chỉ di chuyển này.
