KERNEL_ELF := toyos
KERNEL_BIN := $(KERNEL_ELF).bin
BIOS_BIN := ../bios/rustsbi_qemu.bin # BIOS
KERNEL_MAIN_PHY_ADDR := 0x80200000 # BIOS默认跳转到这里
build: clean
	@riscv64-unknown-elf-as main.asm -o $(KERNEL_ELF).o   # 编译
	@riscv64-unknown-elf-ld $(KERNEL_ELF).o -Ttext $(KERNEL_MAIN_PHY_ADDR) -o $(KERNEL_ELF)   # 链接
	@riscv64-unknown-elf-objcopy $(KERNEL_ELF) -S -O binary $(KERNEL_BIN)   # 生成镜像
run: build
	@qemu-system-riscv64  -machine virt -nographic -bios $(BIOS_BIN) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_MAIN_PHY_ADDR)
clean:
	@rm -f $(KERNEL_ELF) $(KERNEL_ELF).o $(KERNEL_BIN)
disasm:
	@riscv64-unknown-elf-objdump -sS $(KERNEL_ELF)
