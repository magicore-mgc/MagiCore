# finding

EN: Owns the public `Finding` value and constructors for Critical, High, Medium and file context.

VI: Sở hữu giá trị `Finding` public và constructor cho Critical, High, Medium cùng đường dẫn file.

- API: public path remains `audit::Finding`; fields and constructor signatures are unchanged.
- Invariant: constructor severity and optional path values retain their prior representations.
- Errors: infallible value construction has no local error type.
- Extension: new finding metadata belongs here only when required by the existing audit contract.
- Tests: `test/builders.rs` retains the constructor assertions; runtime remains NOT_RUN.
