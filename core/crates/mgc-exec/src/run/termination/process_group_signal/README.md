# process_group_signal / gửi signal process group

Owns the liveness recheck and Unix signal syscalls for already observed groups. It keeps root-direct fallback in the coordinator's plan and never signals an empty/unobserved group set.

Sở hữu liveness recheck và syscall signal Unix cho group đã quan sát. Fallback signal trực tiếp root vẫn theo planner/coordinator; owner không signal khi tập group rỗng/chưa quan sát.
