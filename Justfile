default: check

init:
    rustup component add clippy rustfmt
    sr init

install:
    cargo install --path crates/teasr-cli

build:
    cargo build --workspace

run *ARGS:
    cargo run -p teasr-cli -- {{ARGS}}

test:
    cargo test --workspace

lint:
    cargo clippy --workspace -- -D warnings

fmt:
    cargo fmt --all

check-fmt:
    cargo fmt --all -- --check

publish:
    cargo publish -p teasr-core --dry-run
    cargo publish -p teasr-cli --dry-run

check: check-fmt lint test

ci: check-fmt lint build test
