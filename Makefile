
# Makefile for the blink_minimum project (Nucleo G474RE)
# Useful targets for embedded Rust development.

CARGO := cargo
TARGET := thumbv7em-none-eabihf
CHIP := STM32G474RE
PACKAGE := NUCLEO-G474RE-blink-for-embedded-rust
RELEASE_FLAGS := --release
TARGET_DIR := target/$(TARGET)

.PHONY: all build release embed flash run clippy fmt doc check test clean

all: build

# Debug build for the target
build:
	$(CARGO) build --target $(TARGET)

# Release build for the target
release:
	$(CARGO) build $(RELEASE_FLAGS) --target $(TARGET)

# Flash using cargo-embed (requires cargo-embed installed)
embed:
	cargo embed --chip $(CHIP) $(RELEASE_FLAGS)

# Flash using cargo-flash (requires cargo-flash installed)
flash:
	cargo-flash --chip $(CHIP) --target $(TARGET) $(RELEASE_FLAGS)

# Run clippy and treat warnings as errors
clippy:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

# Format code
fmt:
	$(CARGO) fmt

# Generate documentation and open in browser (host toolchain)
doc:
	$(CARGO) doc --no-deps --open

# Quick check for the target
check:
	$(CARGO) check --target $(TARGET)

clean:
	$(CARGO) clean
