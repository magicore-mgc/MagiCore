# Optimizer adapter registry

## Purpose

Owns the concrete adapter list (`all_adapters`) exactly as before: Web (Node, Deno, Bun), AI (PyTorch, Candle, Go AI), Lib (Rust, Go, Python, TypeScript) and App (Flutter, React Native). Ordering is preserved because selection returns fresh instances in this order.

## Phạm vi

Sở hữu danh sách adapter cụ thể (`all_adapters`) y như trước: Web (Node, Deno, Bun), AI (PyTorch, Candle, Go AI), Lib (Rust, Go, Python, TypeScript) và App (Flutter, React Native). Thứ tự được giữ vì selection trả instance mới theo đúng thứ tự này.
