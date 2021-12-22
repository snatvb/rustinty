
use log::{LevelFilter, Log, Metadata, Record};
use simplelog::{Config, SharedLogger, WriteLogger, CombinedLogger};
use std::{
    ffi::{CString},
    os::raw::c_char,
};


#[repr(C)]
pub struct LogMessage {
    level: LevelFilter,
    message: *const c_char,
}

impl LogMessage {
    pub fn new(record: &Record) -> Self {
        let c_message = CString::new(record.args().to_string()).unwrap();
        let p_message = c_message.as_ptr();
        std::mem::forget(c_message);
        Self {
            level: record.level().to_level_filter(),
            message: p_message,
        }
    }
}


#[no_mangle]
pub extern "C" fn rustiny_logger_bind(callback: extern "C" fn(LogMessage)) {
    log::info!("rustiny_logger_bind");
    let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = vec![Box::new(UnityLogger {
        level: LevelFilter::Trace,
        logger_callback: Box::new(move |log_msg| callback(log_msg)),
    })];
    loggers.push(WriteLogger::new(
        LevelFilter::Trace,
        Config::default(),
        std::fs::File::create("rustiny.log").unwrap(),
    ));
    CombinedLogger::init(loggers).unwrap();
    log::info!("Logger initialized");
}


pub struct UnityLogger {
    pub level: LevelFilter,
    logger_callback: Box<dyn Fn(LogMessage) + Send + Sync + 'static>,
}


impl log::Log for UnityLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        (self.logger_callback)(LogMessage::new(record));

    }

    fn flush(&self) {}
}

impl SharedLogger for UnityLogger {
    fn level(&self) -> LevelFilter {
        self.level
    }
    fn config(&self) -> Option<&Config> {
        None
    }
    fn as_log(self: Box<Self>) -> Box<dyn Log> {
        Box::new(*self)
    }
}
