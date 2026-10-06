default:
    @just --list

release:
    cargo build --workspace --release

lint:
    cargo clippy --workspace --all-targets -- -D warnings

bin package="rust":
    cargo run -p "{{ package }}"

workspace:
    cargo build --workspace

test:
    cargo test --workspace

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

run:
    cargo run

check: fmt-check lint

crates_dir := "crates"
new-bin name:
    cargo new "{{ crates_dir }}/{{ name }}" --bin --vcs none

new-lib name:
    cargo new "{{ crates_dir }}/{{ name }}" --lib --vcs none
