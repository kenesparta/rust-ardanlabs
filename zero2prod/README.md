## 1.5 Continuous Integration

### Code coverage
```shell
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov
```

This command will compute the code coverage
```shell
cargo llvm-cov
```

### Linter
To use a Linter, we recommend clippy
```shell
rustup component add clippy

# This should fail the linter in the CI/CD pipeline
cargo clippy -- -D warnings
```

### Code Format
```shell
# It'll fail when a commit contains unformatted code, printing the difference to the console.
cargo fmt -- --check
```

### Security vulnerabilities
Rustsec site: https://github.com/RustSec
```shell
cargo install cargo-audit

# Execute:
cargo audit
```

> Give a man a fish, and you feed him for a day. Teach a man to fish, and you feed him for a lifetime.