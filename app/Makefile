TARGET := riscv64gc-unknown-none-elf
APP_DIR := target
APP_SRC_DIR := src/bin
TARGET_DIR := $(APP_DIR)/$(TARGET)/release

APP_NAMES := $(wildcard $(APP_SRC_DIR)/*.rs) # 匹配上的路径整体成为APP_NAMES数组
APP_ELFS := $(patsubst $(APP_SRC_DIR)/%.rs, $(TARGET_DIR)/%, $(APP_NAMES)) #遍历APP_NAMES中的每一个，如果匹配上参数1，则将参数1中的%部分替换掉参数2的%

build: clean
	@cd ../libr && make build
	@RUSTFLAGS='-Clink-arg=-Tlinker.ld' cargo build --target $(TARGET) --release
	$(foreach elf, $(APP_ELFS), riscv64-unknown-elf-objcopy $(elf) -S -O binary $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.bin, $(elf));) # 针对APP_ELFS中的每一个，命名为elf，然后去执行objcopy
clean:
	@cd ../libr && make clean
	@rm -rf $(APP_DIR)