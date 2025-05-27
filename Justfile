fix-and-check:
   cargo +nightly fmt
   cargo clippy
   cargo deny check

check:
    cargo +nightly fmt -- --check
    cargo clippy
    cargo deny check