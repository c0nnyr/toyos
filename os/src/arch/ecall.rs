use super::riscv::ecall as raw_ecall; //使用的是自己外面的riscv模块，所以是super::，重命名符号为raw_ecall来表示下区别

enum EcallType {
    //枚举一下所有支持的对外暴露的ecall能力
    PutcharSerialIO,
    Shutdown,
}

//只支持枚举中的那些能力
fn _ecall(type_: EcallType, a0: usize, a1: usize, a2: usize) -> usize {
    let op_code: usize = match type_ {
        //这里的match保证我们所有的枚举都会被处理，不会漏掉
        EcallType::PutcharSerialIO => 0x01,
        EcallType::Shutdown => 0x08,
    };
    raw_ecall::ecall(op_code, a0, a1, a2)
}

//串口打印字符
pub fn putchar_serialio(ch: char) -> usize {
    _ecall(EcallType::PutcharSerialIO, ch as usize, 0, 0)
}

//关机
pub fn shutdown() -> ! {
    _ecall(EcallType::Shutdown, 0, 0, 0);
    panic!("never here")
}
