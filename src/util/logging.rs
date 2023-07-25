use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Config, Handle,
};

use crate::models::logger::LoggerConfig;

pub struct Logger {
    handle: log4rs::Handle,
}

impl Logger {
    pub fn init() -> Logger {
        let stdout = ConsoleAppender::builder().build();
        let config = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .build(Root::builder().appender("stdout").build(LevelFilter::Info))
            .unwrap();
        Logger {
            handle: log4rs::init_config(config).unwrap(),
        }
    }

    pub fn update_verbosity(&self, config: &LoggerConfig) {
        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new("[{d}] {l} - {m}\n")))
            .build();
        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("[{d}] {l} - {m}\n")))
            .build(&config.log_path)
            .unwrap();

        let config = Config::builder()
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(
                        config.verbosity.unwrap_or(LevelFilter::Off),
                    )))
                    .build("stdout", Box::new(stdout)),
            )
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(
                Root::builder()
                    .appender("stdout")
                    .appender("logfile")
                    .build(config.verbosity.unwrap_or(LevelFilter::Off)),
            )
            .unwrap();

        self.handle.set_config(config);
    }
}
