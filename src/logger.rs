use std::fs::{DirBuilder, File};
use std::io::stdout;
use std::sync::Mutex;

use tracing::Level;
use tracing_subscriber::{
    fmt::{layer, time::UtcTime, writer::MakeWriterExt},
    layer::SubscriberExt,
    registry,
    util::SubscriberInitExt,
    EnvFilter, Layer,
};

const LOGS_DIRECTORY: &str = "logs";

pub fn init_logger() {
    init_logs_directory(LOGS_DIRECTORY);
    let info_log_file = get_info_log_file(LOGS_DIRECTORY);

    let info_layer = layer()
        .json()
        .with_writer(Mutex::new(info_log_file))
        .with_target(false)
        .with_thread_names(true)
        .with_timer(UtcTime::rfc_3339())
        .with_level(true)
        .with_filter(EnvFilter::from("INFO"));

    let error_log_file = get_error_log_file(LOGS_DIRECTORY);

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

    registry()
        .with(info_layer)
        .with(error_layer)
        .with(debug_layer)
        .init();
}

fn get_info_log_file(dir: &str) -> File {
    get_log_file(dir, "info")
}

fn get_error_log_file(dir: &str) -> File {
    get_log_file(dir, "error")
}

fn get_log_file(dir: &str, filename: &str) -> File {
    let path = format!("{dir}/{filename}.log");

    File::options()
        .write(true)
        .read(true)
        .append(true)
        .create(true)
        .open(path)
        .unwrap()
}

fn init_logs_directory(dir_path: &str) {
    DirBuilder::new().recursive(true).create(dir_path).unwrap();
}

#[cfg(test)]
mod tests {
    use tempfile::{TempDir, TempPath};

    use super::*;

    #[test]
    fn test_init_logs_directory() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_path_str = String::from(tmp_dir.path().to_str().unwrap());

        let path = TempPath::from_path(format!("{tmp_path_str}/logs"));

        init_logs_directory(path.to_str().unwrap());

        assert!(path.is_dir());
    }

    #[test]
    fn test_get_and_init_log_files() {
        let tmp_log_dir = TempDir::new().unwrap();
        init_logs_directory(tmp_log_dir.path().to_str().unwrap());

        let tmp_dir_path = tmp_log_dir.path().to_str().unwrap();

        let info_file = get_log_file(tmp_dir_path, "info");
        let error_file = get_log_file(tmp_dir_path, "error");
        assert!(info_file.metadata().unwrap().is_file());
        assert!(error_file.metadata().unwrap().is_file());

        let info_file = get_info_log_file(tmp_dir_path);
        let error_file = get_error_log_file(tmp_dir_path);

        assert!(info_file.metadata().unwrap().is_file());
        assert!(error_file.metadata().unwrap().is_file());
    }
}
