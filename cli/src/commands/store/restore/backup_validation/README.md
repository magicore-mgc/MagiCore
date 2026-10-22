# Backup validation

Validates the backup root and each optional component before project mutation. It rejects symlinked/non-regular database entries, symlinked vstore roots, paths outside the canonical backup root, and unsafe entries inside the vstore. A backup may contain either component or both; an empty backup is rejected.

# Xác thực backup

Xác thực root backup và từng thành phần tùy chọn trước khi sửa project. Owner từ chối database dạng symlink/không phải file thường, vstore root dạng symlink, path thoát root backup canonical và entry không an toàn trong vstore. Backup được phép có một hoặc cả hai thành phần; backup rỗng bị từ chối.
