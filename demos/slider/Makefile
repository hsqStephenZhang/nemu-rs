TARGET := riscv64gc-unknown-none-elf
MODE := release
TARGET_DIR := ../../target/$(TARGET)/$(MODE)
APP := slider

OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64
CARGO := cargo

ELF := $(TARGET_DIR)/$(APP)
BIN := $(TARGET_DIR)/$(APP).bin
ELF_COPY := $(TARGET_DIR)/$(APP).elf

default: build

build: 
	@echo "[BUILD] cargo build --release --target=$(TARGET)"
	@cargo build -Z build-std=core,alloc --release --target=$(TARGET)
	@cp $(ELF) $(ELF_COPY)
	@$(OBJCOPY) --strip-all -O binary $(ELF) $(BIN)
	@echo "[OK] Created $(BIN) and $(ELF_COPY)"

.PHONY: build clean run default
