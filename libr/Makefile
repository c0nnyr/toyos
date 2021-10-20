TARGET := riscv64gc-unknown-none-elf
LIBR_DIR := target
LIBR_LIB := $(LIBR_DIR)/$(TARGET)/release/libr
build: clean
	@cargo build --target $(TARGET) --release
clean:
	@rm -rf $(LIBR_DIR)