# resolve_git_conflict_markers

Parses both Git conflict sides as canonical lockfiles, duplicates shared lines to both sides, and delegates the empty-base union to `merge3`. Any missing marker, parse failure or merge conflict remains `None`.

# resolve_git_conflict_markers — trách nhiệm

Parse hai phía Git conflict thành lockfile canonical, đưa dòng chung vào cả hai nhánh rồi giao phép union nền rỗng cho `merge3`. Thiếu marker, parse lỗi hoặc merge conflict tiếp tục trả `None`.
