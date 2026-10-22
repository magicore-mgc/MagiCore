# PackageJson schema / Schema PackageJson

Owns the serialized npm package shape, `PackageJson::new/load/save`, the 10 MiB bounded no-follow reader, and preservation of unknown flattened fields. Filesystem publication is delegated to the atomic-write owner.

Sở hữu dạng dữ liệu npm được serialize, `PackageJson::new/load/save`, reader không-follow có giới hạn 10 MiB và việc giữ các field lạ trong `extra`. Publish filesystem được giao cho owner atomic-write.

Local application manifests may omit `version`; the public string field is empty only for omission and serialization preserves absence. Explicit empty, null or non-string versions are rejected. The manifest projection validates nonempty versions as semver. Registry package metadata and publishing validation are separate contracts and remain strict.
Manifest ứng dụng local được phép thiếu `version`; field string công khai rỗng chỉ biểu diễn vắng mặt và serialize giữ nguyên việc thiếu field. Version tường minh rỗng/null/không phải chuỗi bị từ chối; projection kiểm semver khi có version. Metadata package registry và kiểm tra publish giữ contract riêng nghiêm ngặt.

The public type and method signatures are unchanged. Reads open a no-follow regular-file handle, check its metadata, and read at most the size cap plus one byte; saves use write-if-changed so identical bytes do not rewrite the file.

Type và chữ ký method công khai không đổi. Đọc qua handle file thường no-follow, kiểm metadata và chỉ đọc tối đa giới hạn cộng một byte; lưu qua write-if-changed để nội dung byte giống nhau không ghi lại file.
