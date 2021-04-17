build:
	@RUSTFLAGS="-C target-cpu=native -C link-arg=-s" cargo build --release
	@upx --best --lzma target/release/rcount