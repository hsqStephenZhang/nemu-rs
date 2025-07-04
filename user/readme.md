## Compile

to make sure that codes in core and alloc is not compressed, we run `cargo build -Z build-std=core,alloc --release` to rebuild these two crates, and it works with the flag "-Ctarget-feature=-c" in `config.toml`
