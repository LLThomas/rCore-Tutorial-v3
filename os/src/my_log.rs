use log::{Log, Level, LevelFilter};
use crate::console;

pub fn init() {
    static LOGGER: Logger = Logger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("debug") => LevelFilter::Debug,
        Some("error") => LevelFilter::Error,
        Some("info") => LevelFilter::Info,
        Some("trace") => LevelFilter::Trace,
        Some("warn") => LevelFilter::Warn,
        _ => LevelFilter::Off
    });
}

struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        };
        console::print(format_args!(
            "\x1b[{}m{}\x1b[0m\n", level_to_color(record.level()), record.args()
        ));
    }

    fn flush(&self) {
        
    }
}

fn level_to_color(level: Level) -> u8 {
    match level {
        Level::Debug => 32,     // Green
        Level::Error => 31,     // Red
        Level::Info => 34,      // Blue
        Level::Trace => 90,     // Grey
        Level::Warn => 93       // Yellow
    }
}