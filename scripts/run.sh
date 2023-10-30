cargo clean
cargo build
cargo test
cargo clippy --verbose --all-targets --all-features -- -D warnings
cargo fmt --verbose --all -- --check
cargo-tarpaulin --manifest-path ../Cargo.toml --out Html --output-dir ../scripts