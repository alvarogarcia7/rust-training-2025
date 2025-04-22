pre-commit: test clippy format
.PHONY: pre-commit

pre-push: test clippy format build
.PHONY: pre-push

format:
	cargo fmt --all -- --check
.PHONY: format

build:
	cargo build --all --all-features
.PHONY: build


test:
	cargo test --all --all-features --tests
.PHONY: test

test-doc:
	cargo test --doc
.PHONY: test-doc

clippy:
	cargo clippy --all --all-features --tests -- -D warnings
.PHONY: clippy


