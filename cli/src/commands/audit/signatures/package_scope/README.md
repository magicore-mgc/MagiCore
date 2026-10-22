# Package scope

Maps OCI package names through the registry's canonical core scope helper. Other protocols keep their package spelling. Invalid core-scoped OCI names map to the existing CLI signature-validation diagnostic.

Implementation: `package_scope.rs`. Tests: `test/package_scope.rs`. The module does not infer a core from package text.
