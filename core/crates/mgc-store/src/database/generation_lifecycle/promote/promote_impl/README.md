# CAS generation promote implementation

## Responsibility

Owns the private promotion coordinator called only inside the immediate transaction coordinated by the parent `promote` owner. Token verification, old-generation retirement, and final state flip each have a same-name child owner.

## Trách nhiệm

Quản lý coordinator promote private, chỉ được gọi bên trong transaction immediate do owner `promote` điều phối. Kiểm token, nghỉ hưu generation cũ và đổi state cuối có owner con riêng cùng tên.

## Boundaries

The parent owns transaction begin/commit/rollback. The child owners preserve token checks, SQL order, affected-row guard, typed errors, and diagnostic text. These modules are visible only within the parent promote owner.

## Ranh giới

Owner cha chịu trách nhiệm begin/commit/rollback. Các owner con giữ nguyên kiểm tra token, thứ tự SQL, kiểm tra số row, lỗi có type và thông báo. Các module này chỉ hiển thị trong owner promote cha.
