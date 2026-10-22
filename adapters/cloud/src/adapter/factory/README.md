# Factory owner

Constructs the Cloud adapter, keeps `Ok(None)` for a non-Cloud project, and binds CDK/Pulumi registry credentials to the explicit project root.

Dựng Cloud adapter, giữ `Ok(None)` khi project không phải Cloud và bind credential CDK/Pulumi vào đúng project root.

This folder has one implementation owner; its module facade only wires exports.

Folder này có một owner implementation; facade module chỉ nối module và export.
