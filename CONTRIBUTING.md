# Contributing

Contributions should preserve the crate's deterministic, date-only design. Runtime code must not use the local clock, timezone, locale, environment, filesystem, or network unless a narrowly scoped API explicitly asks for that behavior.

Before opening a pull request, run:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo test --doc --all-features
cargo doc --no-deps --all-features
cargo package --list
cargo publish --dry-run
```

Add regression tests for invalid input, region containment, and deterministic rendering when changing layout calculations. Keep optional dependencies behind features and update README examples when public behavior changes.
