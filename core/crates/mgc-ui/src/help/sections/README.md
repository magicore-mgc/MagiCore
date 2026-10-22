# Help sections

Each same-name section owner formats one part of the stable help layout. The owner module exports one private renderer, implementation lives in `<owner>/<owner>.rs`, and each owner has its own nearest test and README. The shared command-name owner prevents spelling drift. Keep section order/profile selection in `../help.rs`; do not dispatch commands here.

# Các section help

Mỗi owner section cùng tên dựng một phần bố cục help ổn định. Module owner export một renderer private, implementation tại `<owner>/<owner>.rs`, mỗi owner có test gần nhất và README. Owner tên command dùng chung tránh lệch cách viết. Thứ tự/chọn profile ở `../help.rs`; không dispatch command ở đây.
