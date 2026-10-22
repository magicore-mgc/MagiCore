# Maven audit scanner owners

EN: This module selects the supported Java audit input and projects Maven pins to the shared OSV engine. Gradle verification metadata and direct fixed-version Maven POM dependencies are handled by separate owners. A POM scan does not resolve transitive Maven models, parents, BOMs, or profiles.

VI: Module này chọn đầu vào audit Java đang hỗ trợ và chuyển pin Maven sang engine OSV dùng chung. Metadata verification của Gradle và dependency POM Maven trực tiếp có version cố định thuộc hai owner riêng. Scan POM không resolve dependency bắc cầu, parent, BOM hoặc profile.

The module facade preserves the public scanner routes. The existing integration suite remains in the crate tests folder; focused owner tests are registered beside each implementation.
