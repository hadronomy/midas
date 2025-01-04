_default:
    @just --list

check:
    cargo clippy --locked
    cargo fmt -- --check

fix:
    cargo clippy --fix --locked -- -D warnings

test:
    cargo nextest run

run *release:
    @if [ "{{release}}" = "release" ]; then \
        cargo run --release; \
    else \
        cargo run; \
    fi

dev:
    mise run

