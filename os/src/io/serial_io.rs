use crate::arch; //绝对路径引用
pub struct SerialIO {}

impl core::fmt::Write for SerialIO {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.chars() {
            arch::ecall::putchar_serialio(ch);
        }
        Ok(()) //这里是返回值，等价于return Ok(())。OK是Result的一个枚举类型
    }
}
