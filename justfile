default: help

help:
    @echo "Usage: just [recipe]"
    @echo "Available recipes:"
    @just --list | tail -n +2 | awk '{printf "  \033[36m%-20s\033[0m %s\n", $1, substr($0, index($0, $2))}'

setup:
    cargo fetch --locked

run *args:
    @cargo run -- {{args}}

fix:
    cargo fmt
    cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged -- -D warnings

check:
    cargo fmt --check
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all-targets --all-features
