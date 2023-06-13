run:
    cargo run

build:
    RUSTFLAGS="-C target-cpu=native" cargo build --release