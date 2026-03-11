.PHONY: build test bootstrap-claude

build:
	cargo build --release

test:
	cargo test

bootstrap-claude:
	scripts/bootstrap-claude.sh
