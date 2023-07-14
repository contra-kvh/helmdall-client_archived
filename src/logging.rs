use std::process;
use std::time::SystemTime;
use fern::Dispatch;
use log::{error, info, LevelFilter};

#[macro_export]
macro_rules! fatal {
    ($message:expr) => {
        error!("FATAL: {}", $message);
        process::exit(1);
    }
}

pub fn setup_logger(mut dispatch: Dispatch, level: &LevelFilter) {
    match dispatch
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{date}] {level: <6} {target} - {message}",
                date=humantime::format_rfc3339(SystemTime::now()),
                level=record.level(),
                target=record.target(),
                message=message
            ))
        })
        .level(*level)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        .apply() {
            Ok(_) => info!("logger initialized"),
            Err(e) => {
                println!("failed to initialize the logger:\n{e:?}");
                process::exit(1);
            }
    }
}