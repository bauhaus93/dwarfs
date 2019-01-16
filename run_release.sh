RUSTFLAGS="$RUSTFLAGS -A dead_code -A unused_imports" RUST_LOG="app,dwarfs=debug" cargo run --bin app --release
