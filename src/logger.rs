use std::io::stdout;
use std::sync::Mutex;
use std::fs::{ DirBuilder, File };

use tracing::Level;
use tracing_subscriber::{
    fmt::{ layer, time::UtcTime, writer::MakeWriterExt },
    util::SubscriberInitExt,
    layer::SubscriberExt,
    registry,
    EnvFilter,
    Layer,
};

const LOGS_DIRECTORY: &str = "logs";

pub fn init_logger() {
    init_logs_directory();
    let info_log_file = get_info_log_file();

    let info_layer = layer()
        .json()
        .with_writer(Mutex::new(info_log_file))
        .with_target(false)
        .with_thread_names(true)
        .with_timer(UtcTime::rfc_3339())
        .with_level(true)
        .with_filter(EnvFilter::from("INFO"));

    let error_log_file = get_error_log_file();

    let error_layer = layer()
        .json()
        .with_writer(Mutex::new(error_log_file))
        .with_target(true)
        .with_file(true)
        .with_thread_names(true)
        .with_filter(EnvFilter::from("ERROR"));

    let debug_layer = layer()
        .with_timer(UtcTime::rfc_3339())
        .with_writer(stdout.with_max_level(Level::DEBUG))
        .pretty();

    registry().with(info_layer).with(error_layer).with(debug_layer).init();
}

fn get_info_log_file() -> File {
    get_log_file("info")
}

fn get_error_log_file() -> File {
    get_log_file("error")
}

fn get_log_file(filename: &str) -> File {
    File::options()
        .write(true)
        .read(true)
        .append(true)
        .create(true)
        .open(format!("{LOGS_DIRECTORY}/{filename}.log"))
        .unwrap()
}

fn init_logs_directory() {
    DirBuilder::new().recursive(true).create(LOGS_DIRECTORY).unwrap();
}
