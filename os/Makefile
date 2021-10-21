TARGET := riscv64gc-unknown-none-elf
TARGET_DIR := target
KERNEL_ELF := $(TARGET_DIR)/$(TARGET)/release/toyos
KERNEL_BIN := $(KERNEL_ELF).bin
BIOS_BIN := ../bios/rustsbi_qemu.bin # BIOS
KERNEL_MAIN_PHY_ADDR := 0x80200000 # BIOS默认跳转到这里
build: clean
	@cd ../app && make build
	@RUSTFLAGS='-Clink-arg=-Tlinker.ld' cargo build --target $(TARGET) --release
	@riscv64-unknown-elf-objcopy $(KERNEL_ELF) -S -O binary $(KERNEL_BIN)   # 生成镜像
run: build
	@qemu-system-riscv64  -machine virt -nographic -bios $(BIOS_BIN) \
	-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_MAIN_PHY_ADDR) \
	-device loader,file=../app/$(TARGET_DIR)/$(TARGET)/release/hello_world.bin,addr=0x80400000
clean:
	@rm -rf $(TARGET_DIR) $(KERNEL_ELF) $(KERNEL_ELF).o $(KERNEL_BIN)
disasm:
	@riscv64-unknown-elf-objdump -sS $(KERNEL_ELF)
debug1:
	@echo start running qemu
	@qemu-system-riscv64  -machine virt -nographic -bios $(BIOS_BIN) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_MAIN_PHY_ADDR) -s -S
debug2:
	@riscv64-unknown-elf-gdb -ex 'target remote localhost:1234' -ex 'break *$(KERNEL_MAIN_PHY_ADDR)' -ex 'x /16i $(KERNEL_MAIN_PHY_ADDR)' -ex 'c'
