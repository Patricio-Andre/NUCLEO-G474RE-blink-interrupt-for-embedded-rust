# Makefile para o projeto blink_minimum (Nucleo G474RE)
# Alvos úteis para desenvolvimento embarcado com Rust.

CARGO := cargo
TARGET := thumbv7em-none-eabihf
CHIP := STM32G474RE
RELEASE_FLAGS := --release

.PHONY: all build release embed flash run clippy fmt doc check test clean

all: build

# Build de desenvolvimento para o target thumbv7em
build:
	$(CARGO) build --target $(TARGET)

# Build em release para o target
release:
	$(CARGO) build $(RELEASE_FLAGS) --target $(TARGET)

# Flash usando cargo-embed (requer cargo-embed instalado)
embed:
	cargo embed --chip $(CHIP) $(RELEASE_FLAGS)

# Flash usando cargo-flash (requer cargo-flash instalado)
flash:
	cargo-flash --chip $(CHIP) --target $(TARGET) $(RELEASE_FLAGS)

# Executar com o runner configurado em .cargo/config.toml (probe-run)
run:
	$(CARGO) run $(RELEASE_FLAGS)

# Rodar clippy e tratar warnings como erros
clippy:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

# Formatar código
fmt:
	$(CARGO) fmt

# Gerar documentação e abrir no navegador
doc:
	$(CARGO) doc --no-deps --open

# Checagem rápida para o target
check:
	$(CARGO) check --target $(TARGET)

# Rodar testes (não aplicável a no_std/embedded, mas incluído para conveniência)
test:
	$(CARGO) test

clean:
	$(CARGO) clean
