check:
	cargo clippy -- \
		-D warnings \
		-D clippy::pedantic \
		-D clippy::nursery
