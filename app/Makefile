TARGET := riscv64gc-unknown-none-elf
APP_DIR := target
APP_ELF := $(APP_DIR)/$(TARGET)/release/hello_world
APP_BIN := $(APP_ELF).bin
build: clean
	@cd ../libr && make build
	@RUSTFLAGS='-Clink-arg=-Tlinker.ld' cargo build --target $(TARGET) --release
	@riscv64-unknown-elf-objcopy $(APP_ELF) -S -O binary $(APP_BIN)   # 生成镜像
clean:
	@cd ../libr && make clean
	@rm -rf $(APP_DIR)
disasm:
	@riscv64-unknown-elf-objdump -sS $(APP_ELF)