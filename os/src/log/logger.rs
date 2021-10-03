use crate::io::serial_io::SerialIO; //引入SerialIO，SerialIO是我们目前的主要打印日志方式
use spin::Mutex;
#[derive(core::cmp::PartialOrd, core::cmp::PartialEq)]
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

impl Logger {
    pub fn init(&mut self, min_level: Level, type_: LoggerType) {
        self.min_level = min_level;
        self.type_ = type_;
        match self.type_ {
            LoggerType::DummyWriter => self.dummy_writer = Some(DummyWriter {}),
            LoggerType::SerialIO => self.serial_io_writer = Some(SerialIO {}),
        };
    }
    pub fn should_log(&self, level: &Level) -> bool {
        level >= &self.min_level
    }
    pub fn get_fmt_prefix(&self, level: &Level) -> &'static str {
        match self.type_ {
            LoggerType::DummyWriter => match level {
                &Level::Debug => "KDebug:",
                &Level::Info => "KInfo:",
                &Level::Warn => "KWarn:",
                &Level::Error => "KError:",
                &Level::Never => "KNever:",
            },
            LoggerType::SerialIO => match level {
                &Level::Debug => "\x1b[90mKDebug:", //灰色
                &Level::Info => "\x1b[34mKInfo:",   //蓝色
                &Level::Warn => "\x1b[32mKWarn:",   //黄色
                &Level::Error => "\x1b[31mKError:", //红色
                &Level::Never => "KNever:",         //不应该到这里
            },
        }
    }

    pub fn get_fmt_suffix(&self, _level: &Level) -> &'static str {
        match self.type_ {
            LoggerType::DummyWriter => "",
            LoggerType::SerialIO => "\x1b[0m",
        }
    }
}

impl core::fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self.type_ {
            //枚举具体的logger类型，使用具体的writer来打印
            LoggerType::DummyWriter => match &mut self.dummy_writer {
                //枚举所有writer的可能，非空情况下才真正打印
                Some(writer) => writer.write_str(s),
                None => Ok(()),
            },
            LoggerType::SerialIO => match &mut self.serial_io_writer {
                //枚举所有writer的可能，非空情况下才真正打印
                Some(writer) => writer.write_str(s),
                None => Ok(()),
            },
        }
    }
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

pub static LOGGER: Mutex<Logger> = Mutex::new(Logger {
    min_level: Level::Never,
    type_: LoggerType::DummyWriter,
    dummy_writer: Some(DummyWriter {}),
    serial_io_writer: None,
});
