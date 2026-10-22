# Flutter root manifest projection

Owns reading the root `pubspec.yaml` and projecting its root identity, version and sorted `dependencies`/`dev_dependencies` names. It does not resolve package versions or inspect Flutter SDK manifests.

## Trách nhiệm

Sở hữu đọc `pubspec.yaml` root và chuyển identity, version cùng tên dependency/dev-dependency theo thứ tự đã sắp xếp. Module không resolve version và không đọc manifest Flutter SDK.
