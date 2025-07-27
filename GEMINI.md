# Gemini Instructions


## After Code Changes:
After any code change (Thus before any commit), the following must be run:

- `cargo clippy --all`: Must not have any warnings or errors
- `cargo test --all`: All tests must pass. If significant changes are made, new tests must be created as necessary
- `cargo fmt`: All code must be formatted via cargo fmt
