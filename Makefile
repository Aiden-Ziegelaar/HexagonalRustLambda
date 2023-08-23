test:
	cargo test

build-debug:
	cargo lambda build --output-format zip --arm64

build-production:
	cargo lambda build --output-format zip --arm64 --release