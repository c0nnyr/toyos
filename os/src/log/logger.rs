use crate::io::serial_io::SerialIO; //引入SerialIO，SerialIO是我们目前的主要打印日志方式

pub enum Level {
    //日志分级枚举
    Debug = 1,
    Info,
    Warn,
    Error,
    Never,
}

pub struct Logger {
    min_level: Level,
    type_: LoggerType,
    dummy_writer: Option<DummyWriter>, //具体是什么Writer是未知的，所以都用Option来包裹
    serial_io_writer: Option<SerialIO>,
}

struct DummyWriter {} //空的Writer
impl core::fmt::Write for DummyWriter {
    fn write_str(&mut self, _s: &str) -> core::fmt::Result {
        Ok(())
    }
}

pub enum LoggerType {
    //定义所有支持的Logger类型
    DummyWriter,
    SerialIO,
}
