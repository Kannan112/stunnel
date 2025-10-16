CARGO := cargo

.PHONY: all build release run test fmt clippy clean gen-proto

all: build

build:
	$(CARGO) build

release:
	$(CARGO) build --release

run:
	$(CARGO) run

test:
	$(CARGO) test

fmt:
	$(CARGO) fmt

clippy:
	$(CARGO) clippy -- -D warnings

clean:
	$(CARGO) clean

# Generate protobuf code using build.rs
gen-proto:
	$(CARGO) build

# Alias for gen-proto
protogen: gen-proto
