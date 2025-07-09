TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR := ./src/bin
TARGET_DIR := ../target/$(TARGET)/$(MODE)
BUILD_DIR := build
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64
PY := python3

TEST ?= 0

ifeq ($(TEST), 0)
	APPS := $(wildcard $(APP_DIR)/*.rs)
else
	APPS := $(wildcard $(APP_DIR)/test*.rs) $(APP_DIR)/initproc.rs $(APP_DIR)/user_shell.rs
endif

ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))
ELFS += $(TARGET_DIR)/io_test_keyboard

# Debug output
$(info ==============================)
$(info APP_DIR = $(APP_DIR))
$(info APPS = $(APPS))
$(info ELFS = $(ELFS))
$(info ==============================)

elf: $(APPS)
	@echo "[INFO] Would run: cargo build -Z build-std=core,alloc --release"
	# force rebuild the core and alloc with 
	@cargo build -Z build-std=core,alloc --release



binary: elf
	@$(foreach elf, $(ELFS), \
		cp $(elf) $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.elf, $(elf)); \
		$(OBJCOPY) --strip-all -O binary $(elf) $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.bin, $(elf));)

pre:
	@mkdir -p $(BUILD_DIR)/bin/
	@mkdir -p $(BUILD_DIR)/elf/
	@mkdir -p $(BUILD_DIR)/app/
	@$(foreach t, $(APPS), cp $(t) $(BUILD_DIR)/app/;)

build: clean pre binary 
	@$(foreach t, $(ELFS), cp $(t).bin $(BUILD_DIR)/bin/;)
	@$(foreach t, $(ELFS), cp $(t).elf $(BUILD_DIR)/elf/;)

clean:
	@cargo clean
	@rm -rf $(BUILD_DIR)

.PHONY: elf binary build clean
