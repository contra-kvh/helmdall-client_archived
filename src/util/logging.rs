use fern::Dispatch;
use log::LevelFilter::Warn;
use log::{info, LevelFilter};
use std::process;
use std::time::SystemTime;

#[macro_export]
macro_rules! fatal {
    ($message:expr) => {
        error!("FATAL: {}", $message);
        process::exit(1);
    };
}

pub fn setup_logger(level: &LevelFilter) {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{date}] {level: <6} {target} - {message}",
                date = humantime::format_rfc3339(SystemTime::now()),
                level = record.level(),
                target = record.target(),
                message = message
            ))
        })
        .level(*level)
        .level_for("ws", Warn)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        .apply();
}
