-include Makefile.options
###############################################################################
test/unit:
	RUST_LOG=DEBUG cargo test --verbose
.PHONY: test/unit		
test/lint:
	cargo clippy --all-targets --all-features -- -D warnings
.PHONY: test/lint	
###############################################################################
