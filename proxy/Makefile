ARCH        := $(shell uname -m)
MUSL_TARGET  = $(ARCH)-unknown-linux-musl
BINARY      = target/$(MUSL_TARGET)/release/ductus

.PHONY: build build-static verify-static test

build:
	cargo build

build-static:
	cargo build --release --target=$(MUSL_TARGET)

verify-static: build-static
	file $(BINARY) | grep -q "statically linked"
	ldd $(BINARY) 2>&1 | grep -q "not a dynamic executable"
	@echo "OK: static binary verified"

test:
	cargo test
