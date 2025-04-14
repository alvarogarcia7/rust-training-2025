pre-commit: test clippy
.PHONY: pre-commit

test:
	cargo test --all --all-features --tests
.PHONY: test

clippy:
	cargo clippy --all --all-features --tests -- -D warnings
.PHONY: clippy


