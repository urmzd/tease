default: check

init:
    rustup component add clippy rustfmt
    sr init --merge 2>/dev/null || sr init

install:
    cargo build --release -p teasr-cli

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
    cargo publish -p teasr-term-render --dry-run
    cargo publish -p teasr-core --dry-run
    cargo publish -p teasr-cli --dry-run

record: install
    PATH="$(pwd)/target/release:$PATH" teasr showme

check: check-fmt lint test

ci: check-fmt lint build test
