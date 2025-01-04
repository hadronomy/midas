_default:
    @just --list

check:
    cargo clippy --locked
    cargo fmt -- --check

fix:
    cargo clippy --fix --locked -- -D warnings

test:
    cargo nextest run

run:
    cargo run

dev:
    mise run

