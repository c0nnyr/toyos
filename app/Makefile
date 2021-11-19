TARGET := riscv64gc-unknown-none-elf
APP_DIR := target
APP_SRC_DIR := src/bin
TARGET_DIR := $(APP_DIR)/$(TARGET)/release

build: clean
	@cd ../libr && make build
	@RUSTFLAGS='-Clink-arg=-Tlinker.ld' cargo build --target $(TARGET) --release
clean:
	@cd ../libr && make clean
	@rm -rf $(APP_DIR)