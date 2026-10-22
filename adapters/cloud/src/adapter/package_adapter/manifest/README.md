# Cloud package manifest owner

Delegates manifest parsing to Web when CloudAdapter was constructed with the CDK/Pulumi package engine. Otherwise it returns the existing minimal Cloud manifest named from the project directory, falling back to `infra`.

Delegate parse manifest sang Web khi CloudAdapter được dựng với package engine CDK/Pulumi. Nếu không, trả manifest Cloud tối thiểu hiện có, lấy tên từ thư mục project và fallback về `infra`.
