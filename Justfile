default:
    @just --list

release:
    cargo build --workspace --release

lint:
    cargo clippy --workspace --all-targets -- -D warnings

bin package:
    cargo run -p "{{ package }}"

# run a named binary from a lesson crate: just run-bin l-03-ownership guard
run-bin package name:
    cargo run -p "{{ package }}" --bin "{{ name }}"

workspace:
    cargo build --workspace

test:
    cargo test --workspace

test-lesson package:
    cargo test -p "{{ package }}"

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

check: fmt-check lint

crates_dir := "crates"
new-bin name:
    cargo new "{{ crates_dir }}/{{ name }}" --bin --vcs none

new-lib name:
    cargo new "{{ crates_dir }}/{{ name }}" --lib --vcs none
