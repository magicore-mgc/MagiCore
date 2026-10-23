# Cloud adapter-path firewall

## Purpose

Owns the T0.3-clo-gap rule: only CDK and Pulumi projects that have a `package.json` may enter the native Web engine; Terraform, Cloudflare, missing manifests and unknown types fail closed. Without the `clo` feature the gate always fails closed.

## Vietnamese

Sở hữu quy tắc T0.3-clo-gap: chỉ project CDK/Pulumi có `package.json` mới được vào Web engine native; Terraform, Cloudflare, thiếu manifest và type lạ đều fail-closed. Khi không bật feature `clo`, gate luôn fail-closed.
