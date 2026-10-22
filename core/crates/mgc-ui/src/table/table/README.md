# Table model owner

Owns Table state, construction, row insertion, and the stable public type.
Sở hữu state Table, khởi tạo, thêm row và public type ổn định.

Rendering is delegated to the sibling render owner without exposing fields.
Render được giao cho owner render cùng cấp; field vẫn được ẩn.
