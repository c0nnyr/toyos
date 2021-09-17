KERNEL_BIN=./os.bin # 操作系统镜像
BIOS_BIN=./rustsbi_qemu.bin # BIOS
KERNEL_MAIN_PHY_ADDR=0x80200000 # BIOS默认跳转到这
qemu-system-riscv64  -machine virt -nographic -bios $BIOS_BIN -device loader,file=$KERNEL_BIN,addr=$KERNEL_MAIN_PHY_ADDR
