# Windows root exit

Owns the Windows Job guard transfer and monitored-root completion after the root process exits.

The existing Job inspector remains the authority for surviving descendants. The guard is consumed exactly once by that path; an unmonitored command drops it before bounded output collection.

# Windows root-exit

> Owner này quản lý chuyển giao Windows Job guard và hoàn tất root process đã thoát.

> Job inspector hiện có tiếp tục quyết định descendant còn sống. Nhánh monitored consume guard đúng một lần; command không monitor sẽ thả guard trước khi thu output có deadline.
