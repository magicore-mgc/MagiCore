# Cloud project gate

Detects only CDK/Pulumi root `package.json`, builds the concrete Cloud context and routes to the shared gate. It does not select or spawn a provider package manager.

Giữ đúng thứ tự parse compat rồi gate và truyền audit path như hiện tại.
